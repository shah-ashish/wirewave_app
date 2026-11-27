use crate::{feature_host::FeatureHost, settings_store::ConnectionPreferences};
use std::sync::Arc;
use wirewave_core_stream::{ChannelId, UdpDataChannel};
use wirewave_core_transport::ControlMessage;

/// Legacy trait for Speaker feature compatibility.
pub trait SpeakerStreamHost: Send + Sync {
    fn start_stream(&self, channel: Arc<UdpDataChannel>, runtime: tokio::runtime::Handle);
    fn stop_stream(&self);
    fn status_detail(&self) -> Option<String>;
}

/// Adapter converting legacy SpeakerStreamHost into polymorphic FeatureHost.
pub(crate) struct SpeakerHostAdapter(pub(crate) Arc<dyn SpeakerStreamHost>);
impl FeatureHost for SpeakerHostAdapter {
    fn channel_id(&self) -> ChannelId {
        ChannelId::Speaker
    }
    fn display_name(&self) -> &'static str {
        "Speaker"
    }
    fn is_allowed(&self, prefs: &ConnectionPreferences) -> bool {
        prefs.speaker_service_allowed
    }
    fn status_detail(&self) -> Option<String> {
        self.0.status_detail()
    }
    fn requires_peer_hello(&self) -> bool {
        true
    }
    fn start_stream(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: tokio::runtime::Handle,
    ) -> Result<Option<String>, String> {
        self.0.start_stream(channel, runtime);
        Ok(self.status_detail())
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
        ControlMessage::SpeakerState {
            allowed,
            active,
            detail,
            udp_port,
            stream_id,
            udp_key_hex,
        }
    }
}
