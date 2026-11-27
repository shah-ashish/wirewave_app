use crate::{feature_host::FeatureHost, settings_store::ConnectionPreferences};
use std::sync::Arc;
use wirewave_core_stream::{ChannelId, UdpDataChannel};
use wirewave_core_transport::ControlMessage;

/// Legacy trait for Microphone feature compatibility.
pub trait MicStreamHost: Send + Sync {
    fn start_stream(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<String, String>;
    fn stop_stream(&self);
    fn available_driver(&self) -> Option<String>;
}

/// Adapter converting legacy MicStreamHost into polymorphic FeatureHost.
pub(crate) struct MicHostAdapter(pub(crate) Arc<dyn MicStreamHost>);
impl FeatureHost for MicHostAdapter {
    fn channel_id(&self) -> ChannelId {
        ChannelId::Mic
    }
    fn display_name(&self) -> &'static str {
        "Microphone"
    }
    fn is_allowed(&self, prefs: &ConnectionPreferences) -> bool {
        prefs.mic_service_allowed && self.0.available_driver().is_some()
    }
    fn status_detail(&self) -> Option<String> {
        self.0.available_driver()
    }
    fn start_stream(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<Option<String>, String> {
        self.0.start_stream(channel, runtime).map(Some)
    }
    fn stop_stream(&self) {
        self.0.stop_stream();
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
        ControlMessage::MicState {
            allowed,
            active,
            detail,
            udp_port,
            stream_id,
            udp_key_hex,
        }
    }
}
