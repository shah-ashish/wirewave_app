use std::sync::Arc;
use tokio::runtime::Handle;
use wirewave_core_stream::{ChannelId, UdpDataChannel};
use wirewave_core_transport::ControlMessage;
use crate::settings_store::ConnectionPreferences;

/// Common interface for any real-time feature streaming over the encrypted UDP data plane.
///
/// Follows the Open/Closed Principle: new features (Mouse, Keyboard, Camera) can be added
/// without modifying core session code by implementing this trait and registering with `FeatureRegistry`.
pub trait FeatureHost: Send + Sync {
    /// Associated stream channel identifier (Speaker, Mic, Mouse, Keyboard, Camera).
    fn channel_id(&self) -> ChannelId;

    /// Human-readable display label.
    fn display_name(&self) -> &'static str;

    /// Checks if this feature is permitted by user settings and prerequisites (e.g. installed drivers).
    fn is_allowed(&self, prefs: &ConnectionPreferences) -> bool;

    /// Returns human-readable status detail (e.g. output device name or driver name).
    fn status_detail(&self) -> Option<String>;

    /// Whether this feature sends from Desktop to Phone and requires a `WW2U_HELLO` firewall punch.
    fn requires_peer_hello(&self) -> bool {
        false
    }

    /// Starts streaming over the provided authenticated UDP data channel.
    fn start_stream(
        &self,
        channel: Arc<UdpDataChannel>,
        runtime: Handle,
    ) -> Result<Option<String>, String>;

    /// Stops any active stream and releases hardware/network resources.
    fn stop_stream(&self);

    /// Constructs the protocol control message representing this feature's state.
    fn make_state_message(
        &self,
        allowed: bool,
        active: bool,
        detail: Option<String>,
        udp_port: Option<u16>,
        stream_id: Option<u32>,
        udp_key_hex: Option<String>,
    ) -> ControlMessage;

    /// Builds the initial state message sent upon entering a connected session.
    fn initial_state(&self, prefs: &ConnectionPreferences) -> ControlMessage {
        let allowed = self.is_allowed(prefs);
        let detail = if allowed {
            self.status_detail()
        } else {
            Some("Blocked by desktop settings".to_string())
        };
        self.make_state_message(allowed, false, detail, None, None, None)
    }
}
