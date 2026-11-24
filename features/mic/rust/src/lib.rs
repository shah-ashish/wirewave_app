pub mod audio;
pub mod ui;

pub use audio::{
    device_finder::{
        detect_compatible_mic_driver, open_recommended_driver_page,
        select_compatible_mic_driver, CompatibleMicDriver, RECOMMENDED_DRIVER_URL,
    },
    render_loop::{MicService, MicStatus},
};
pub use ui::{mic_logic::MicTileState, mic_ui::render_mic_tile};

impl wirewave_core_session::MicStreamHost for MicService {
    fn start_stream(
        &self,
        channel: std::sync::Arc<wirewave_core_stream::UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<String, String> {
        self.start(channel, runtime)
    }

    fn stop_stream(&self) {
        self.stop();
    }

    fn available_driver(&self) -> Option<String> {
        self.detected_driver().map(|driver| driver.name)
    }
}

impl wirewave_core_session::FeatureHost for MicService {
    fn channel_id(&self) -> wirewave_core_stream::ChannelId {
        wirewave_core_stream::ChannelId::Mic
    }

    fn display_name(&self) -> &'static str {
        "Microphone"
    }

    fn is_allowed(&self, prefs: &wirewave_core_session::ConnectionPreferences) -> bool {
        prefs.mic_service_allowed && self.detected_driver().is_some()
    }

    fn status_detail(&self) -> Option<String> {
        self.detected_driver().map(|driver| driver.name)
    }

    fn start_stream(
        &self,
        channel: std::sync::Arc<wirewave_core_stream::UdpDataChannel>,
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
    ) -> wirewave_core_session::ControlMessage {
        wirewave_core_session::ControlMessage::MicState {
            allowed,
            active,
            detail,
            udp_port,
            stream_id,
            udp_key_hex,
        }
    }
}
