use crate::driver::{detect_compatible_camera_driver, CompatibleCameraDriver, VirtualCameraSink};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex, RwLock,
};
use tokio::sync::watch;
use wirewave_core_session::{ConnectionPreferences, FeatureHost};
use wirewave_core_stream::{ChannelId, UdpDataChannel};
use wirewave_core_transport::ControlMessage;

const CHUNK_HEADER_BYTES: usize = 8;
const MAX_CHUNKS_PER_FRAME: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CameraStatus {
    Off,
    Active { driver_name: String },
}

#[derive(Debug, Clone)]
pub struct DecodedFrame {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

struct FrameAssembler {
    frame_id: u32,
    total_chunks: usize,
    received_count: usize,
    chunks: Vec<Option<Vec<u8>>>,
}

impl FrameAssembler {
    fn new() -> Self {
        Self {
            frame_id: 0,
            total_chunks: 0,
            received_count: 0,
            chunks: Vec::new(),
        }
    }

    fn push_packet(&mut self, payload: &[u8]) -> Option<Vec<u8>> {
        // Fallback if an unchunked raw JPEG arrives.
        if payload.len() >= 2 && payload[0] == 0xFF && payload[1] == 0xD8 {
            return Some(payload.to_vec());
        }
        if payload.len() <= CHUNK_HEADER_BYTES {
            return None;
        }

        let frame_id = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
        let chunk_idx = u16::from_be_bytes([payload[4], payload[5]]) as usize;
        let total_chunks = u16::from_be_bytes([payload[6], payload[7]]) as usize;

        if total_chunks == 0 || total_chunks > MAX_CHUNKS_PER_FRAME || chunk_idx >= total_chunks {
            return None;
        }

        if frame_id != self.frame_id || total_chunks != self.total_chunks {
            // Only advance forward (allow wrap-around).
            if self.total_chunks != 0 && frame_id < self.frame_id && (self.frame_id - frame_id) < 1000 {
                return None;
            }
            self.frame_id = frame_id;
            self.total_chunks = total_chunks;
            self.received_count = 0;
            self.chunks = vec![None; total_chunks];
        }

        if self.chunks[chunk_idx].is_none() {
            self.chunks[chunk_idx] = Some(payload[CHUNK_HEADER_BYTES..].to_vec());
            self.received_count += 1;
        }

        if self.received_count == self.total_chunks {
            let mut full = Vec::new();
            for part in self.chunks.drain(..) {
                if let Some(bytes) = part {
                    full.extend_from_slice(&bytes);
                }
            }
            self.total_chunks = 0;
            self.received_count = 0;
            return Some(full);
        }

        None
    }
}

pub struct CameraService {
    running: Arc<AtomicBool>,
    generation: Arc<AtomicU64>,
    status: Arc<RwLock<CameraStatus>>,
    latest_frame: Arc<RwLock<Option<Vec<u8>>>>,
    latest_decoded: Arc<RwLock<Option<DecodedFrame>>>,
    sink: Arc<Mutex<Option<VirtualCameraSink>>>,
    stop_tx: Mutex<Option<watch::Sender<bool>>>,
}

impl Default for CameraService {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            generation: Arc::new(AtomicU64::new(0)),
            status: Arc::new(RwLock::new(CameraStatus::Off)),
            latest_frame: Arc::new(RwLock::new(None)),
            latest_decoded: Arc::new(RwLock::new(None)),
            sink: Arc::new(Mutex::new(None)),
            stop_tx: Mutex::new(None),
        }
    }

    pub fn status(&self) -> CameraStatus {
        self.status
            .read()
            .map(|value| value.clone())
            .unwrap_or(CameraStatus::Off)
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    pub fn detected_driver(&self) -> Option<CompatibleCameraDriver> {
        detect_compatible_camera_driver()
    }

    pub fn latest_frame(&self) -> Option<Vec<u8>> {
        self.latest_frame.read().ok().and_then(|f| f.clone())
    }

    pub fn latest_decoded_frame(&self) -> Option<DecodedFrame> {
        self.latest_decoded.read().ok().and_then(|f| f.clone())
    }

    pub fn start(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<String, String> {
        let driver = self
            .detected_driver()
            .ok_or_else(|| "Compatible virtual camera driver not found".to_string())?;

        self.stop();
        let _generation = self.generation.fetch_add(1, Ordering::AcqRel) + 1;
        self.running.store(true, Ordering::Release);
        if let Ok(mut status) = self.status.write() {
            *status = CameraStatus::Active {
                driver_name: driver.name.clone(),
            };
        }

        let (stop_tx, mut stop_rx) = watch::channel(false);
        if let Ok(mut holder) = self.stop_tx.lock() {
            *holder = Some(stop_tx);
        }

        let receive_channel = Arc::clone(&channel);
        let receive_running = Arc::clone(&self.running);
        let frame_store = Arc::clone(&self.latest_frame);
        let decoded_store = Arc::clone(&self.latest_decoded);
        let sink_holder = Arc::clone(&self.sink);
        let status_holder = Arc::clone(&self.status);

        runtime.spawn(async move {
            tokio::select! {
                _ = stop_rx.changed() => return,
                result = receive_channel.accept_peer_hello() => {
                    if result.is_err() {
                        return;
                    }
                }
            }

            let mut assembler = FrameAssembler::new();

            while receive_running.load(Ordering::Acquire) {
                tokio::select! {
                    _ = stop_rx.changed() => break,
                    packet = receive_channel.recv_datagram() => {
                        match packet {
                            Ok((ChannelId::Camera, _seq, payload)) => {
                                if let Some(jpeg) = assembler.push_packet(&payload) {
                                    if let Ok(mut lock) = frame_store.write() {
                                        *lock = Some(jpeg.clone());
                                    }

                                    if let Ok(img) = image::load_from_memory_with_format(&jpeg, image::ImageFormat::Jpeg) {
                                        let rgba_img = img.to_rgba8();
                                        let (width, height) = rgba_img.dimensions();
                                        let rgba = rgba_img.into_raw();

                                        if let Ok(mut sink_guard) = sink_holder.lock() {
                                            if sink_guard.is_none() {
                                                match VirtualCameraSink::open_best(width, height) {
                                                    Ok((opened, actual_name)) => {
                                                        *sink_guard = Some(opened);
                                                        if let Ok(mut st) = status_holder.write() {
                                                            *st = CameraStatus::Active {
                                                                driver_name: actual_name,
                                                            };
                                                        }
                                                    }
                                                    Err(err) => {
                                                        tracing::warn!("Virtual camera sink unavailable: {err}");
                                                    }
                                                }
                                            }
                                            if let Some(sink) = sink_guard.as_ref() {
                                                sink.write_rgba(width, height, &rgba);
                                            }
                                        }

                                        if let Ok(mut lock) = decoded_store.write() {
                                            *lock = Some(DecodedFrame { width, height, rgba });
                                        }
                                    }
                                }
                            }
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    }
                }
            }
        });

        Ok(driver.name)
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Release);
        if let Ok(mut holder) = self.stop_tx.lock() {
            if let Some(tx) = holder.take() {
                let _ = tx.send(true);
            }
        }
        if let Ok(mut sink_guard) = self.sink.lock() {
            *sink_guard = None;
        }
        if let Ok(mut status) = self.status.write() {
            *status = CameraStatus::Off;
        }
        if let Ok(mut lock) = self.latest_frame.write() {
            *lock = None;
        }
        if let Ok(mut lock) = self.latest_decoded.write() {
            *lock = None;
        }
    }
}

impl FeatureHost for CameraService {
    fn channel_id(&self) -> ChannelId {
        ChannelId::Camera
    }

    fn display_name(&self) -> &'static str {
        "Camera"
    }

    fn is_allowed(&self, prefs: &ConnectionPreferences) -> bool {
        prefs.camera_service_allowed && self.detected_driver().is_some()
    }

    fn status_detail(&self) -> Option<String> {
        self.detected_driver().map(|driver| driver.name)
    }

    fn start_stream(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<Option<String>, String> {
        self.start(channel, runtime).map(Some)
    }

    fn stop_stream(&self) {
        self.stop();
    }

    fn make_state_message(
        &self,
        allowed: bool,
        active: bool,
        detail: Option<String>,
        udp_port: Option<u16>,
        stream_id: Option<u32>,
        udp_key_hex: Option<String>,
    ) -> ControlMessage {
        ControlMessage::CameraState {
            allowed,
            active,
            detail,
            udp_port,
            stream_id,
            udp_key_hex,
        }
    }
}
