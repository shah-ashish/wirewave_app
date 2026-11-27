pub mod active_channel;
#[cfg(windows)]
mod bluetooth_listener;
pub mod feature_host;
pub mod feature_registry;
pub mod handshake;
pub mod key_injector;
pub mod session_manager;
pub mod session_state;
pub mod settings_store;
mod stream_hosts;
pub mod transport_listener;
pub mod trust_store;

pub use active_channel::ActiveControlChannel;
pub use feature_host::FeatureHost;
pub use feature_registry::{FeatureRegistry, SharedFeatureRegistry};
pub use handshake::HandshakeCoordinator;
pub use session_manager::{
    ActiveSession, PendingPairing, SessionManager, SessionSnapshot, SessionState, SharedSession,
    MicStreamHost, SpeakerStreamHost,
};
pub use settings_store::{ConnectionPreferences, PreferencesStore};
pub use transport_listener::{StreamAcceptor, TransportListener};
pub use trust_store::{TrustStore, TrustedDevice};
pub use wirewave_core_stream::ChannelId;
pub use wirewave_core_transport::ControlMessage;
