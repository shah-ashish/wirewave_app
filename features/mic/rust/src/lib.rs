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
