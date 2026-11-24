pub mod audio;
pub mod ui;

pub use audio::{
    capture_loop::{SpeakerService, SpeakerStatus},
    device_finder::{
        detect_active_playback_endpoint, select_playback_endpoint, AudioEndpointInfo, EndpointKind,
    },
    laud::{LaudHeader, FRAMES_PER_PACKET, LAUD_HEADER_BYTES, TARGET_SAMPLE_RATE},
};
pub use ui::{speaker_logic::SpeakerTileState, speaker_ui::render_speaker_tile};

impl wirewave_core_session::SpeakerStreamHost for SpeakerService {
    fn start_stream(
        &self,
        channel: std::sync::Arc<wirewave_core_stream::UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) {
        self.start(channel, runtime);
    }

    fn stop_stream(&self) {
        self.stop();
    }

    fn status_detail(&self) -> Option<String> {
        detect_active_playback_endpoint().map(|endpoint| endpoint.name)
    }
}

impl wirewave_core_session::FeatureHost for SpeakerService {
    fn channel_id(&self) -> wirewave_core_stream::ChannelId {
        wirewave_core_stream::ChannelId::Speaker
    }

    fn display_name(&self) -> &'static str {
        "Speaker"
    }

    fn is_allowed(&self, prefs: &wirewave_core_session::ConnectionPreferences) -> bool {
        prefs.speaker_service_allowed
    }

    fn status_detail(&self) -> Option<String> {
        detect_active_playback_endpoint().map(|endpoint| endpoint.name)
    }

    fn requires_peer_hello(&self) -> bool {
        true
    }

    fn start_stream(
        &self,
        channel: std::sync::Arc<wirewave_core_stream::UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<Option<String>, String> {
        self.start(channel, runtime);
        Ok(self.status_detail())
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
    ) -> wirewave_core_session::ControlMessage {
        wirewave_core_session::ControlMessage::SpeakerState {
            allowed,
            active,
            detail,
            udp_port,
            stream_id,
            udp_key_hex,
        }
    }
}
