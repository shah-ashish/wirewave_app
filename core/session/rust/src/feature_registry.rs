use crate::{feature_host::FeatureHost, settings_store::ConnectionPreferences};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use wirewave_core_stream::ChannelId;
use wirewave_core_transport::ControlMessage;

/// Thread-safe registry holding all active feature hosts.
///
/// Encapsulates feature lifecycle and state query dispatching across all registered features.
pub struct FeatureRegistry {
    features: RwLock<HashMap<ChannelId, Arc<dyn FeatureHost>>>,
    health: RwLock<HashMap<ChannelId, Arc<wirewave_core_stream::health::StreamHealth>>>,
}

pub type SharedFeatureRegistry = Arc<FeatureRegistry>;

impl FeatureRegistry {
    pub fn new() -> Self {
        Self {
            features: RwLock::new(HashMap::new()),
            health: RwLock::new(HashMap::new()),
        }
    }

    /// Registers a feature host. Overwrites any previous registration for the same channel.
    pub fn register(&self, host: Arc<dyn FeatureHost>) {
        let mut guard = self
            .features
            .write()
            .expect("feature registry lock poisoned");
        guard.insert(host.channel_id(), host);
    }

    pub fn monitor(&self, kind: ChannelId, channel: &Arc<wirewave_core_stream::UdpDataChannel>) {
        self.clear_health(kind);
        self.health
            .write()
            .unwrap()
            .insert(kind, channel.monitor(kind));
    }

    pub fn latency(&self, kind: ChannelId) -> Option<wirewave_core_stream::health::LiveLatency> {
        self.health
            .read()
            .unwrap()
            .get(&kind)
            .map(|health| health.snapshot().0)
    }

    pub fn clear_health(&self, kind: ChannelId) {
        if let Some(health) = self.health.write().unwrap().remove(&kind) {
            health.stop();
        }
    }

    pub fn connection_lost(&self) {
        for health in self.health.read().unwrap().values() {
            health.disconnected();
        }
    }

    /// Retrieves a registered feature host by its channel ID.
    pub fn get(&self, channel: ChannelId) -> Option<Arc<dyn FeatureHost>> {
        let guard = self
            .features
            .read()
            .expect("feature registry lock poisoned");
        guard.get(&channel).cloned()
    }

    /// Lists all registered feature hosts.
    pub fn list_all(&self) -> Vec<Arc<dyn FeatureHost>> {
        let guard = self
            .features
            .read()
            .expect("feature registry lock poisoned");
        guard.values().cloned().collect()
    }

    /// Stops all running feature streams cleanly.
    pub fn stop_all(&self) {
        let guard = self
            .features
            .read()
            .expect("feature registry lock poisoned");
        for host in guard.values() {
            self.clear_health(host.channel_id());
            host.stop_stream();
        }
    }

    /// Generates initial control messages for all registered features to announce their initial state.
    pub fn initial_states(&self, prefs: &ConnectionPreferences) -> Vec<ControlMessage> {
        let guard = self
            .features
            .read()
            .expect("feature registry lock poisoned");
        guard
            .values()
            .map(|host| host.initial_state(prefs))
            .collect()
    }
}

impl Default for FeatureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tokio::runtime::Handle;
    use wirewave_core_stream::UdpDataChannel;

    struct MockFeature {
        channel: ChannelId,
        stopped: Arc<AtomicBool>,
    }

    impl FeatureHost for MockFeature {
        fn channel_id(&self) -> ChannelId {
            self.channel
        }
        fn display_name(&self) -> &'static str {
            "Mock"
        }
        fn is_allowed(&self, _prefs: &ConnectionPreferences) -> bool {
            true
        }
        fn status_detail(&self) -> Option<String> {
            Some("Mock status".into())
        }
        fn start_stream(
            &self,
            _channel: Arc<UdpDataChannel>,
            _runtime: Handle,
        ) -> Result<Option<String>, String> {
            Ok(Some("Started".into()))
        }
        fn stop_stream(&self) {
            self.stopped.store(true, Ordering::SeqCst);
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

    #[test]
    fn registers_and_retrieves_features_polymorphically() {
        let registry = FeatureRegistry::new();
        let stopped = Arc::new(AtomicBool::new(false));
        let mock = Arc::new(MockFeature {
            channel: ChannelId::Speaker,
            stopped: stopped.clone(),
        });

        registry.register(mock);
        assert!(registry.get(ChannelId::Speaker).is_some());
        assert!(registry.get(ChannelId::Mic).is_none());

        registry.stop_all();
        assert!(stopped.load(Ordering::SeqCst));
    }
}
