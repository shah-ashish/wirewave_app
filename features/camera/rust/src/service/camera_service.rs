use crate::driver::{detect_compatible_camera_driver, CompatibleCameraDriver};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex, RwLock,
};
use tokio::sync::watch;
use wirewave_core_session::{ConnectionPreferences, FeatureHost};
use wirewave_core_stream::{ChannelId, UdpDataChannel};
use wirewave_core_transport::ControlMessage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CameraStatus {
    Off,
    Active { driver_name: String },
}

pub struct CameraService {
    running: Arc<AtomicBool>,
    generation: Arc<AtomicU64>,
    status: Arc<RwLock<CameraStatus>>,
    latest_frame: Arc<RwLock<Option<Vec<u8>>>>,
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
            stop_tx: Mutex::new(None),
        }
    }

    pub fn status(&self) -> CameraStatus {
        self.status.read().map(|value| value.clone()).unwrap_or(CameraStatus::Off)
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

        runtime.spawn(async move {
            tokio::select! {
                _ = stop_rx.changed() => return,
                result = receive_channel.accept_peer_hello() => {
                    if result.is_err() {
                        return;
                    }
                }
            }

            while receive_running.load(Ordering::Acquire) {
                tokio::select! {
                    _ = stop_rx.changed() => break,
                    packet = receive_channel.recv_datagram() => {
                        match packet {
                            Ok((ChannelId::Camera, _seq, payload)) => {
                                if let Ok(mut lock) = frame_store.write() {
                                    *lock = Some(payload);
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
        if let Ok(mut status) = self.status.write() {
            *status = CameraStatus::Off;
        }
        if let Ok(mut lock) = self.latest_frame.write() {
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
