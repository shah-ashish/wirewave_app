pub mod input_injector;
mod packet;
pub mod ui;

pub use ui::{render_mouse_tile, MouseTileState};

use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use tokio::sync::watch;
use wirewave_core_session::{ConnectionPreferences, ControlMessage, FeatureHost};
use wirewave_core_stream::{ChannelId, UdpDataChannel};

pub use packet::{EVENT_BUTTON_DOWN, EVENT_BUTTON_UP, EVENT_CLICK, EVENT_MOVE_REL, EVENT_SCROLL};
use packet::MouseEvent;

pub struct MouseService {
    running: Arc<AtomicBool>,
    generation: Arc<AtomicU64>,
    stop_tx: Mutex<Option<watch::Sender<bool>>>,
}

impl Default for MouseService {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            generation: Arc::new(AtomicU64::new(0)),
            stop_tx: Mutex::new(None),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    pub fn start(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<String, String> {
        self.stop();
        let generation = self.generation.fetch_add(1, Ordering::AcqRel) + 1;
        self.running.store(true, Ordering::Release);

        let (stop_tx, mut stop_rx) = watch::channel(false);
        if let Ok(mut holder) = self.stop_tx.lock() {
            *holder = Some(stop_tx);
        }

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

                        let Ok((ChannelId::Mouse, _, payload)) = packet else {
                            continue;
                        };

                        if let Some(event) = MouseEvent::decode(&payload) {
                            input_injector::inject_event(event);
                        }
                    }
                }
            }
        });

        Ok("Windows SendInput".into())
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Release);
        if let Ok(mut holder) = self.stop_tx.lock() {
            if let Some(tx) = holder.take() {
                let _ = tx.send(true);
            }
        }
    }
}

impl FeatureHost for MouseService {
    fn channel_id(&self) -> ChannelId {
        ChannelId::Mouse
    }

    fn display_name(&self) -> &'static str {
        "Mouse"
    }

    fn is_allowed(&self, prefs: &ConnectionPreferences) -> bool {
        prefs.mouse_service_allowed
    }

    fn status_detail(&self) -> Option<String> {
        Some("Windows SendInput".into())
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
        ControlMessage::MouseState {
            allowed,
            active,
            detail,
            udp_port,
            stream_id,
            udp_key_hex,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_service_traits() {
        let service = MouseService::new();
        assert_eq!(service.channel_id(), ChannelId::Mouse);
        assert_eq!(service.display_name(), "Mouse");
        assert!(!service.is_running());
    }

    #[test]
    fn test_binary_packet_structure() {
        let mut packet = [0u8; 6];
        packet[0] = EVENT_MOVE_REL;
        packet[1..3].copy_from_slice(&(-15i16).to_be_bytes());
        packet[3..5].copy_from_slice(&25i16.to_be_bytes());
        packet[5] = 0;

        assert_eq!(packet[0], EVENT_MOVE_REL);
        assert_eq!(i16::from_be_bytes([packet[1], packet[2]]), -15);
        assert_eq!(i16::from_be_bytes([packet[3], packet[4]]), 25);
    }
}
