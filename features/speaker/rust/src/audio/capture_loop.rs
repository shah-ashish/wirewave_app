use crate::audio::{
    device_finder::{detect_active_playback_endpoint, AudioEndpointInfo},
    dsp::{downmix_to_stereo, f32_to_s16le_bytes, StereoResampler},
    laud::{
        LaudHeader, FLAG_SILENCE, FRAMES_PER_PACKET, PCM_BYTES_PER_PACKET,
        SAMPLE_FORMAT_S16LE_48K, TARGET_CHANNELS, TARGET_SAMPLE_RATE,
    },
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use wirewave_core_stream::{ChannelId, UdpDataChannel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeakerStatus {
    Off,
    Streaming { endpoint: AudioEndpointInfo },
}

/// Manages WASAPI loopback capture streaming encrypted 256-frame LAUD packets over UDP.
pub struct SpeakerService {
    running: Arc<AtomicBool>,
    status: Arc<RwLock<SpeakerStatus>>,
}

impl Default for SpeakerService {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeakerService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            status: Arc::new(RwLock::new(SpeakerStatus::Off)),
        }
    }

    pub fn status(&self) -> SpeakerStatus {
        self.status
            .read()
            .map(|g| g.clone())
            .unwrap_or(SpeakerStatus::Off)
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    /// Starts capturing the optimal playback endpoint (virtual speaker if present, else default)
    /// and streaming encrypted LAUD datagrams over `udp_channel`.
    pub fn start(&self, udp_channel: Arc<UdpDataChannel>, rt_handle: tokio::runtime::Handle) {
        if self.running.swap(true, Ordering::AcqRel) {
            return;
        }
        let running = Arc::clone(&self.running);
        let status = Arc::clone(&self.status);

        let _ = std::thread::Builder::new()
            .name("wirewave-speaker-loopback".to_string())
            .spawn(move || {
                run_capture_supervisor(running, status, udp_channel, rt_handle);
            });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Release);
        if let Ok(mut guard) = self.status.write() {
            *guard = SpeakerStatus::Off;
        }
    }
}

fn run_capture_supervisor(
    running: Arc<AtomicBool>,
    status: Arc<RwLock<SpeakerStatus>>,
    udp_channel: Arc<UdpDataChannel>,
    rt_handle: tokio::runtime::Handle,
) {
    let mut seq: u32 = 0;
    let mut timestamp_frames: u64 = 0;

    while running.load(Ordering::Acquire) {
        let Some(endpoint) = detect_active_playback_endpoint() else {
            tracing::warn!("No active Windows playback endpoint found; retrying");
            std::thread::sleep(std::time::Duration::from_millis(500));
            continue;
        };

        if let Ok(mut guard) = status.write() {
            *guard = SpeakerStatus::Streaming {
                endpoint: endpoint.clone(),
            };
        }
        tracing::info!(
            endpoint = %endpoint.name,
            kind = endpoint.kind.label(),
            "Speaker WASAPI loopback attached"
        );

        run_endpoint_loop(
            &endpoint,
            &running,
            &udp_channel,
            &rt_handle,
            &mut seq,
            &mut timestamp_frames,
        );
    }

    if let Ok(mut guard) = status.write() {
        *guard = SpeakerStatus::Off;
    }
}

#[cfg(windows)]
fn run_endpoint_loop(
    endpoint: &AudioEndpointInfo,
    running: &AtomicBool,
    udp_channel: &Arc<UdpDataChannel>,
    rt_handle: &tokio::runtime::Handle,
    seq: &mut u32,
    timestamp_frames: &mut u64,
) {
    use std::time::{Duration, Instant};
    use windows::core::HSTRING;
    use windows::Win32::Media::Audio::{
        IAudioCaptureClient, IAudioClient, IMMDeviceEnumerator, MMDeviceEnumerator,
        AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoTaskMemFree, CLSCTX_ALL, COINIT_MULTITHREADED,
    };

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let Ok(enumerator): Result<IMMDeviceEnumerator, _> =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
        else {
            std::thread::sleep(Duration::from_millis(400));
            return;
        };

        let Ok(device) = enumerator.GetDevice(&HSTRING::from(&endpoint.id)) else {
            return;
        };
        let Ok(audio_client): Result<IAudioClient, _> = device.Activate(CLSCTX_ALL, None) else {
            return;
        };
        let Ok(mix_format_ptr) = audio_client.GetMixFormat() else {
            return;
        };
        let mix_format = *mix_format_ptr;
        let src_channels = mix_format.nChannels as usize;
        let src_rate = mix_format.nSamplesPerSec;

        let init_res = audio_client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            AUDCLNT_STREAMFLAGS_LOOPBACK,
            200_000, // 20 ms buffer in 100-ns units
            0,
            mix_format_ptr,
            None,
        );
        CoTaskMemFree(Some(mix_format_ptr.cast()));
        if init_res.is_err() {
            return;
        }

        let Ok(capture): Result<IAudioCaptureClient, _> = audio_client.GetService() else {
            return;
        };
        if audio_client.Start().is_err() {
            return;
        }

        let mut stereo_buf = Vec::new();
        let mut resampled_buf = Vec::new();
        let mut pcm_accum: Vec<u8> = Vec::with_capacity(PCM_BYTES_PER_PACKET * 4);
        let mut resampler = StereoResampler::new(src_rate, TARGET_SAMPLE_RATE);
        let mut last_endpoint_check = Instant::now();

        while running.load(Ordering::Acquire) {
            // Hot-swap if user switches Windows playback device or plugs in a virtual speaker.
            if last_endpoint_check.elapsed() >= Duration::from_secs(2) {
                last_endpoint_check = Instant::now();
                if let Some(current) = detect_active_playback_endpoint() {
                    if current.id != endpoint.id {
                        tracing::info!(next = %current.name, "Playback endpoint changed; re-attaching");
                        break;
                    }
                }
            }

            let Ok(mut packet_len) = capture.GetNextPacketSize() else {
                break;
            };
            if packet_len == 0 {
                std::thread::sleep(Duration::from_millis(2));
                continue;
            }

            while packet_len > 0 && running.load(Ordering::Acquire) {
                let mut data_ptr = std::ptr::null_mut();
                let mut num_frames = 0u32;
                let mut flags = 0u32;
                if capture
                    .GetBuffer(&mut data_ptr, &mut num_frames, &mut flags, None, None)
                    .is_err()
                {
                    break;
                }

                let is_silent = (flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32)) != 0;
                if is_silent || data_ptr.is_null() {
                    let silent_frames =
                        ((num_frames as u64 * TARGET_SAMPLE_RATE as u64) / src_rate.max(1) as u64) as usize;
                    pcm_accum.resize(pcm_accum.len() + silent_frames * 4, 0);
                } else {
                    let total_samples = (num_frames as usize) * src_channels;
                    let slice = std::slice::from_raw_parts(data_ptr.cast::<f32>(), total_samples);
                    downmix_to_stereo(slice, src_channels, &mut stereo_buf);
                    resampler.process(&stereo_buf, &mut resampled_buf);
                    f32_to_s16le_bytes(&resampled_buf, &mut pcm_accum);
                }

                let _ = capture.ReleaseBuffer(num_frames);

                while pcm_accum.len() >= PCM_BYTES_PER_PACKET {
                    let chunk: Vec<u8> = pcm_accum.drain(..PCM_BYTES_PER_PACKET).collect();
                    let header = LaudHeader {
                        flags: match is_silent {
                            true => FLAG_SILENCE,
                            false => 0,
                        },
                        channels: TARGET_CHANNELS,
                        sample_format: SAMPLE_FORMAT_S16LE_48K,
                        sequence: *seq,
                        timestamp_frames: *timestamp_frames,
                    };
                    *seq = seq.wrapping_add(1);
                    *timestamp_frames = timestamp_frames.wrapping_add(FRAMES_PER_PACKET as u64);

                    let packet = header.encode_into(&chunk);
                    let ch = Arc::clone(udp_channel);
                    rt_handle.spawn(async move {
                        let _ = ch.send_datagram(ChannelId::Speaker, &packet).await;
                    });
                }

                packet_len = capture.GetNextPacketSize().unwrap_or(0);
            }
        }

        let _ = audio_client.Stop();
    }
}

#[cfg(not(windows))]
fn run_endpoint_loop(
    _endpoint: &AudioEndpointInfo,
    running: &AtomicBool,
    _udp_channel: &Arc<UdpDataChannel>,
    _rt_handle: &tokio::runtime::Handle,
    _seq: &mut u32,
    _timestamp_frames: &mut u64,
) {
    while running.load(Ordering::Acquire) {
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
