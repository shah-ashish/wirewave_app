use crate::audio::{
    device_finder::{detect_compatible_mic_driver, CompatibleMicDriver},
    laud_mono::{LaudMonoHeader, TARGET_SAMPLE_RATE},
    ring_buffer::MonoRingBuffer,
};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex, RwLock,
};
use tokio::sync::watch;
use wirewave_core_stream::{ChannelId, UdpDataChannel};

const RING_CAPACITY_SAMPLES: usize = 2_880;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MicStatus {
    Off,
    Active { driver_name: String },
}

pub struct MicService {
    running: Arc<AtomicBool>,
    generation: Arc<AtomicU64>,
    status: Arc<RwLock<MicStatus>>,
    ring: Arc<Mutex<MonoRingBuffer>>,
    stop_tx: Mutex<Option<watch::Sender<bool>>>,
}

impl Default for MicService {
    fn default() -> Self {
        Self::new()
    }
}

impl MicService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            generation: Arc::new(AtomicU64::new(0)),
            status: Arc::new(RwLock::new(MicStatus::Off)),
            ring: Arc::new(Mutex::new(MonoRingBuffer::new(RING_CAPACITY_SAMPLES))),
            stop_tx: Mutex::new(None),
        }
    }

    pub fn status(&self) -> MicStatus {
        self.status.read().map(|value| value.clone()).unwrap_or(MicStatus::Off)
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    pub fn detected_driver(&self) -> Option<CompatibleMicDriver> {
        detect_compatible_mic_driver()
    }

    pub fn start(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<String, String> {
        let driver = self
            .detected_driver()
            .ok_or_else(|| "Compatible virtual microphone driver not found".to_string())?;

        self.stop();
        let generation = self.generation.fetch_add(1, Ordering::AcqRel) + 1;
        self.running.store(true, Ordering::Release);
        if let Ok(mut ring) = self.ring.lock() {
            ring.clear();
        }
        if let Ok(mut status) = self.status.write() {
            *status = MicStatus::Active {
                driver_name: driver.name.clone(),
            };
        }

        let (stop_tx, mut stop_rx) = watch::channel(false);
        if let Ok(mut holder) = self.stop_tx.lock() {
            *holder = Some(stop_tx);
        }

        let receive_ring = Arc::clone(&self.ring);
        let receive_running = Arc::clone(&self.running);
        let receive_generation = Arc::clone(&self.generation);
        let receive_channel = Arc::clone(&channel);
        runtime.spawn(async move {
            tokio::select! {
                _ = stop_rx.changed() => return,
                result = receive_channel.accept_peer_hello() => {
                    if result.is_err() {
                        return;
                    }
                }
            }
            loop {
                tokio::select! {
                    _ = stop_rx.changed() => break,
                    packet = receive_channel.recv_datagram() => {
                        if !receive_running.load(Ordering::Acquire)
                            || receive_generation.load(Ordering::Acquire) != generation
                        {
                            break;
                        }
                        let Ok((ChannelId::Mic, _, payload)) = packet else { continue; };
                        let Some((_, pcm)) = LaudMonoHeader::decode(&payload) else { continue; };
                        if let Ok(mut ring) = receive_ring.lock() {
                            ring.push_s16le(pcm);
                        }
                    }
                }
            }
        });

        let render_running = Arc::clone(&self.running);
        let render_generation = Arc::clone(&self.generation);
        let render_status = Arc::clone(&self.status);
        let render_ring = Arc::clone(&self.ring);
        let endpoint_id = driver.endpoint_id.clone();
        std::thread::Builder::new()
            .name("wirewave-mic-render".into())
            .spawn(move || {
                render_to_driver(
                    &endpoint_id,
                    generation,
                    &render_running,
                    &render_generation,
                    &render_ring,
                );
                if render_generation.load(Ordering::Acquire) == generation {
                    render_running.store(false, Ordering::Release);
                    if let Ok(mut status) = render_status.write() {
                        *status = MicStatus::Off;
                    }
                }
            })
            .map_err(|error| {
                self.stop();
                format!("Unable to start microphone renderer: {error}")
            })?;

        Ok(driver.name)
    }

    pub fn stop(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        self.running.store(false, Ordering::Release);
        if let Ok(mut holder) = self.stop_tx.lock() {
            if let Some(sender) = holder.take() {
                let _ = sender.send(true);
            }
        }
        if let Ok(mut ring) = self.ring.lock() {
            ring.clear();
        }
        if let Ok(mut status) = self.status.write() {
            *status = MicStatus::Off;
        }
    }
}

#[cfg(windows)]
fn render_to_driver(
    endpoint_id: &str,
    generation: u64,
    running: &AtomicBool,
    current_generation: &AtomicU64,
    ring: &Arc<Mutex<MonoRingBuffer>>,
) {
    use std::time::Duration;
    use windows::core::HSTRING;
    use windows::Win32::Media::Audio::{
        IAudioClient, IAudioRenderClient, IMMDeviceEnumerator, MMDeviceEnumerator,
        WAVEFORMATEXTENSIBLE, AUDCLNT_SHAREMODE_SHARED,
    };
    use windows::Win32::Media::KernelStreaming::KSDATAFORMAT_SUBTYPE_PCM;
    use windows::Win32::Media::Multimedia::KSDATAFORMAT_SUBTYPE_IEEE_FLOAT;
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoTaskMemFree, CLSCTX_ALL, COINIT_MULTITHREADED,
    };

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let Ok(enumerator): Result<IMMDeviceEnumerator, _> =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
        else {
            return;
        };
        let Ok(device) = enumerator.GetDevice(&HSTRING::from(endpoint_id)) else {
            return;
        };
        let Ok(client): Result<IAudioClient, _> = device.Activate(CLSCTX_ALL, None) else {
            return;
        };
        let Ok(format_ptr) = client.GetMixFormat() else {
            return;
        };
        let format = *format_ptr;
        let channels = format.nChannels as usize;
        let sample_rate = format.nSamplesPerSec;
        let bits = format.wBitsPerSample;
        let format_tag = format.wFormatTag;
        let block_align = format.nBlockAlign;

        let sample_format = match (format_tag, bits) {
            (3, 32) => Some(RenderSampleFormat::Float32),
            (1, 16) => Some(RenderSampleFormat::Pcm16),
            (0xfffe, _) => {
                let extensible = format_ptr.cast::<WAVEFORMATEXTENSIBLE>();
                let sub_format = std::ptr::addr_of!((*extensible).SubFormat).read_unaligned();
                match (sub_format, bits) {
                    (KSDATAFORMAT_SUBTYPE_IEEE_FLOAT, 32) => Some(RenderSampleFormat::Float32),
                    (KSDATAFORMAT_SUBTYPE_PCM, 16) => Some(RenderSampleFormat::Pcm16),
                    _ => None,
                }
            }
            _ => None,
        };
        let Some(sample_format) = sample_format else {
            CoTaskMemFree(Some(format_ptr.cast()));
            tracing::warn!(format_tag, bits, "Virtual microphone format is not yet supported");
            return;
        };
        let bytes_per_sample = match sample_format {
            RenderSampleFormat::Float32 => 4usize,
            RenderSampleFormat::Pcm16 => 2usize,
        };
        let valid_layout = channels > 0
            && sample_rate > 0
            && channels
                .checked_mul(bytes_per_sample)
                .is_some_and(|expected| expected == block_align as usize);
        if !valid_layout {
            CoTaskMemFree(Some(format_ptr.cast()));
            tracing::warn!(
                channels,
                sample_rate,
                block_align,
                "Virtual microphone reported an invalid audio buffer layout"
            );
            return;
        }

        let initialized = client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            0,
            200_000,
            0,
            format_ptr,
            None,
        );
        CoTaskMemFree(Some(format_ptr.cast()));
        if initialized.is_err() {
            return;
        }
        let Ok(buffer_frames) = client.GetBufferSize() else {
            return;
        };
        let Ok(render): Result<IAudioRenderClient, _> = client.GetService() else {
            return;
        };
        if client.Start().is_err() {
            return;
        }

        while running.load(Ordering::Acquire)
            && current_generation.load(Ordering::Acquire) == generation
        {
            let Ok(padding) = client.GetCurrentPadding() else {
                break;
            };
            let available = buffer_frames.saturating_sub(padding);
            if available == 0 {
                std::thread::sleep(Duration::from_millis(2));
                continue;
            }
            let Ok(pointer) = render.GetBuffer(available) else {
                break;
            };
            if let Ok(mut source) = ring.lock() {
                if sample_format == RenderSampleFormat::Float32 {
                    let output = std::slice::from_raw_parts_mut(
                        pointer.cast::<f32>(),
                        available as usize * channels,
                    );
                    for frame in output.chunks_exact_mut(channels) {
                        let sample = source.pop_mono(TARGET_SAMPLE_RATE, sample_rate);
                        frame.fill(sample);
                    }
                } else {
                    let output = std::slice::from_raw_parts_mut(
                        pointer.cast::<i16>(),
                        available as usize * channels,
                    );
                    for frame in output.chunks_exact_mut(channels) {
                        let sample = (source.pop_mono(TARGET_SAMPLE_RATE, sample_rate)
                            .clamp(-1.0, 1.0)
                            * 32767.0) as i16;
                        frame.fill(sample);
                    }
                }
            }
            if render.ReleaseBuffer(available, 0).is_err() {
                break;
            }
        }
        let _ = client.Stop();
    }
}

#[cfg(windows)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum RenderSampleFormat {
    Float32,
    Pcm16,
}

#[cfg(not(windows))]
fn render_to_driver(
    _endpoint_id: &str,
    generation: u64,
    running: &AtomicBool,
    current_generation: &AtomicU64,
    _ring: &Arc<Mutex<MonoRingBuffer>>,
) {
    while running.load(Ordering::Acquire)
        && current_generation.load(Ordering::Acquire) == generation
    {
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
