use crate::{
    active_channel::ActiveControlChannel,
    feature_host::FeatureHost,
    feature_registry::{FeatureRegistry, SharedFeatureRegistry},
    handshake::{HandshakeCoordinator, decode_x25519, send_control},
    settings_store::{ConnectionPreferences, PreferencesStore},
    trust_store::{TrustStore, TrustedDevice},
};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Instant,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::watch,
};
use wirewave_core_config::APP_CONFIG;
use wirewave_core_crypto::IdentityKeypair;
use wirewave_core_discovery::run_udp_discovery_responder;
use wirewave_core_identity::DeviceIdentity;
use wirewave_core_stream::{ChannelId, UdpDataChannel};
use wirewave_core_transport::{
    ControlMessage, RejectCode, TransportKind, TransportPriority, read_frame,
};

#[derive(Debug, Clone)]
pub struct PendingPairing {
    pub device_id: String,
    pub device_name: String,
    pub transport: TransportKind,
    pub otp_code: String,
    pub expires_at: Instant,
    pub ttl_secs: u64,
}

impl PendingPairing {
    pub fn remaining_secs(&self) -> u64 {
        self.expires_at
            .saturating_duration_since(Instant::now())
            .as_secs()
    }

    pub fn formatted_code(&self) -> String {
        match self.otp_code.len() == 6 {
            true => format!("{}  {}", &self.otp_code[..3], &self.otp_code[3..]),
            false => self.otp_code.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub device_id: String,
    pub device_name: String,
    pub transport: TransportKind,
}

impl ActiveSession {
    pub fn short_id(&self) -> String {
        let raw = self.device_id.trim_start_matches("wwd_");
        match raw.len() >= 8 {
            true => format!("{}-{}", &raw[..4], &raw[4..8]),
            false => raw.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SessionState {
    Idle,
    Pairing(PendingPairing),
    Connected(ActiveSession),
}

#[derive(Debug, Clone)]
pub struct SessionSnapshot {
    pub pc_name: String,
    pub state: SessionState,
    pub preferences: ConnectionPreferences,
    pub trusted_devices: Vec<TrustedDevice>,
    pub blocked_devices: Vec<TrustedDevice>,
    pub wifi_listening: bool,
    pub bluetooth_listening: bool,
}

struct SessionInner {
    state: SessionState,
    handshake_reservation: Option<HandshakeReservation>,
    trust: TrustStore,
    prefs: PreferencesStore,
    active_cancel_tx: Option<watch::Sender<bool>>,
    pairing_cancel_tx: Option<watch::Sender<bool>>,
    wifi_listening: bool,
    bluetooth_listening: bool,
    speaker_policy_tx: watch::Sender<bool>,
    mic_policy_tx: watch::Sender<bool>,
    mouse_policy_tx: watch::Sender<bool>,
}

#[derive(Debug, Clone)]
struct HandshakeReservation {
    device_id: String,
    transport: TransportKind,
}

pub type SharedSession = Arc<SessionManager>;

/// Legacy trait for Speaker feature compatibility.
pub trait SpeakerStreamHost: Send + Sync {
    fn start_stream(&self, channel: Arc<UdpDataChannel>, runtime: tokio::runtime::Handle);
    fn stop_stream(&self);
    fn status_detail(&self) -> Option<String>;
}

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

/// Adapter converting legacy SpeakerStreamHost into polymorphic FeatureHost.
struct SpeakerHostAdapter(Arc<dyn SpeakerStreamHost>);
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

/// Adapter converting legacy MicStreamHost into polymorphic FeatureHost.
struct MicHostAdapter(Arc<dyn MicStreamHost>);
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

/// Coordinates the 1-to-1 desktop session lock, trust store, priority arbitration,
/// and delegates handshakes and active control loop handling to focused collaborator objects.
pub struct SessionManager {
    desktop_id: String,
    pc_name: String,
    keypair: Arc<IdentityKeypair>,
    inner: RwLock<SessionInner>,
    feature_registry: SharedFeatureRegistry,
    handshake: HandshakeCoordinator,
    active_channel: ActiveControlChannel,
}

impl SessionManager {
    /// Preferred constructor using polymorphic `FeatureRegistry`.
    pub fn new(
        identity: DeviceIdentity,
        keypair: IdentityKeypair,
        feature_registry: SharedFeatureRegistry,
    ) -> SharedSession {
        let pc_name = hostname::get()
            .ok()
            .and_then(|s| s.into_string().ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "Windows PC".to_string());

        let prefs = PreferencesStore::load_default();
        let (speaker_policy_tx, _) = watch::channel(prefs.get().speaker_service_allowed);
        let (mic_policy_tx, _) = watch::channel(prefs.get().mic_service_allowed);
        let (mouse_policy_tx, _) = watch::channel(prefs.get().mouse_service_allowed);

        let keypair = Arc::new(keypair);
        let handshake = HandshakeCoordinator::new(
            identity.device_id.clone(),
            pc_name.clone(),
            Arc::clone(&keypair),
        );
        let active_channel = ActiveControlChannel::new(Arc::clone(&feature_registry));

        Arc::new(Self {
            desktop_id: identity.device_id,
            pc_name,
            keypair,
            inner: RwLock::new(SessionInner {
                state: SessionState::Idle,
                handshake_reservation: None,
                trust: TrustStore::load_default(),
                prefs,
                active_cancel_tx: None,
                pairing_cancel_tx: None,
                wifi_listening: false,
                bluetooth_listening: false,
                speaker_policy_tx,
                mic_policy_tx,
                mouse_policy_tx,
            }),
            feature_registry,
            handshake,
            active_channel,
        })
    }

    /// Backward-compatible constructor accepting legacy Speaker and Mic stream hosts.
    pub fn new_with_hosts(
        identity: DeviceIdentity,
        keypair: IdentityKeypair,
        speaker_host: Arc<dyn SpeakerStreamHost>,
        mic_host: Arc<dyn MicStreamHost>,
    ) -> SharedSession {
        let registry = Arc::new(FeatureRegistry::new());
        registry.register(Arc::new(SpeakerHostAdapter(speaker_host)));
        registry.register(Arc::new(MicHostAdapter(mic_host)));
        Self::new(identity, keypair, registry)
    }

    pub fn snapshot(&self) -> SessionSnapshot {
        let guard = self.inner.read().expect("session lock poisoned");
        SessionSnapshot {
            pc_name: self.pc_name.clone(),
            state: guard.state.clone(),
            preferences: guard.prefs.get(),
            trusted_devices: guard.trust.list_trusted(),
            blocked_devices: guard.trust.list_blocked(),
            wifi_listening: guard.wifi_listening,
            bluetooth_listening: guard.bluetooth_listening,
        }
    }

    pub fn is_busy(&self) -> bool {
        let guard = self.inner.read().expect("session lock poisoned");
        guard.handshake_reservation.is_some() || !matches!(guard.state, SessionState::Idle)
    }

    pub fn set_priority(&self, priority: TransportPriority) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.prefs.set_priority(priority);
    }

    pub fn set_keep_alive(&self, enabled: bool) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.prefs.set_keep_alive(enabled);
    }

    pub fn set_speaker_service_allowed(&self, allowed: bool) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.prefs.set_speaker_service_allowed(allowed);
        let _ = guard.speaker_policy_tx.send(allowed);
        if !allowed {
            if let Some(speaker) = self.feature_registry.get(ChannelId::Speaker) {
                speaker.stop_stream();
            }
        }
    }

    pub fn set_mic_service_allowed(&self, allowed: bool) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.prefs.set_mic_service_allowed(allowed);
        let _ = guard.mic_policy_tx.send(allowed);
        if !allowed {
            if let Some(mic) = self.feature_registry.get(ChannelId::Mic) {
                mic.stop_stream();
            }
        }
    }

    pub fn set_mouse_service_allowed(&self, allowed: bool) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.prefs.set_mouse_service_allowed(allowed);
        let _ = guard.mouse_policy_tx.send(allowed);
        if !allowed {
            if let Some(mouse) = self.feature_registry.get(ChannelId::Mouse) {
                mouse.stop_stream();
            }
        }
    }

    pub fn refresh_mic_service_state(&self) {
        let guard = self.inner.read().expect("session lock poisoned");
        let allowed = guard.prefs.get().mic_service_allowed;
        guard.mic_policy_tx.send_replace(allowed);
    }

    pub fn disconnect_active(&self) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        if let Some(tx) = guard.active_cancel_tx.take() {
            let _ = tx.send(true);
        }
        guard.state = SessionState::Idle;
    }

    pub fn cancel_pairing(&self) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        if let Some(tx) = guard.pairing_cancel_tx.take() {
            let _ = tx.send(true);
        }
        if matches!(guard.state, SessionState::Pairing(_)) {
            guard.state = SessionState::Idle;
        }
    }

    pub fn block_device(&self, device_id: &str) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.trust.set_blocked(device_id, true);
        let should_drop = matches!(
            &guard.state,
            SessionState::Connected(active) if active.device_id == device_id
        );
        if should_drop {
            if let Some(tx) = guard.active_cancel_tx.take() {
                let _ = tx.send(true);
            }
            guard.state = SessionState::Idle;
        }
    }

    pub fn unblock_device(&self, device_id: &str) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.trust.set_blocked(device_id, false);
    }

    pub fn forget_device(&self, device_id: &str) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        guard.trust.forget(device_id);
        let should_drop = matches!(
            &guard.state,
            SessionState::Connected(active) if active.device_id == device_id
        );
        if should_drop {
            if let Some(tx) = guard.active_cancel_tx.take() {
                let _ = tx.send(true);
            }
            guard.state = SessionState::Idle;
        }
    }

    /// Spawns the dedicated background Tokio runtime hosting discovery and listeners.
    pub fn start_background_services(self: &SharedSession) {
        let session = Arc::clone(self);
        std::thread::Builder::new()
            .name("wirewave-session-rt".to_string())
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                {
                    Ok(rt) => rt,
                    Err(err) => {
                        tracing::error!(%err, "Failed to start session Tokio runtime");
                        return;
                    }
                };
                runtime.block_on(async move {
                    let disc_session = Arc::clone(&session);
                    let desktop_id = disc_session.desktop_id.clone();
                    let pc_name = disc_session.pc_name.clone();
                    let desktop_public_key_hex = disc_session.keypair.public_key_hex();
                    tokio::spawn(async move {
                        let busy_flag = Arc::clone(&disc_session);
                        let signing_session = Arc::clone(&disc_session);
                        if let Err(err) = run_udp_discovery_responder(
                            desktop_id,
                            pc_name,
                            desktop_public_key_hex,
                            move |payload| signing_session.keypair.sign(payload),
                            move || busy_flag.is_busy(),
                        )
                        .await
                        {
                            tracing::error!(%err, "UDP discovery responder stopped");
                        }
                    });

                    #[cfg(windows)]
                    {
                        let bt_session = Arc::clone(&session);
                        let runtime_handle = tokio::runtime::Handle::current();
                        tokio::task::spawn_blocking(move || {
                            let ready_session = Arc::clone(&bt_session);
                            let stream_session = Arc::clone(&bt_session);
                            let result = crate::bluetooth_listener::run_rfcomm_listener(
                                APP_CONFIG.connection.bt_service_uuid,
                                move || {
                                    if let Ok(mut guard) = ready_session.inner.write() {
                                        guard.bluetooth_listening = true;
                                    }
                                    tracing::info!("WireWave Bluetooth RFCOMM listener active");
                                },
                                move |stream| {
                                    let Ok(stream) = TcpStream::from_std(stream) else {
                                        return;
                                    };
                                    let mgr = Arc::clone(&stream_session);
                                    runtime_handle.spawn(async move {
                                        mgr.handle_incoming_stream(
                                            stream,
                                            TransportKind::Bluetooth,
                                        )
                                        .await;
                                    });
                                },
                            );
                            if let Err(err) = result {
                                tracing::error!(%err, "Bluetooth RFCOMM listener stopped");
                            }
                        });
                    }

                    session.run_wifi_tcp_listener().await;
                });
            })
            .expect("spawn session runtime thread");
    }

    async fn run_wifi_tcp_listener(self: Arc<Self>) {
        let port = APP_CONFIG.connection.tcp_port;
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = match TcpListener::bind(addr).await {
            Ok(l) => {
                if let Ok(mut guard) = self.inner.write() {
                    guard.wifi_listening = true;
                }
                tracing::info!(port, "WireWave encrypted TCP listener active");
                l
            }
            Err(err) => {
                tracing::error!(%err, port, "Failed to bind WireWave TCP listener");
                return;
            }
        };

        loop {
            let Ok((stream, _peer)) = listener.accept().await else {
                continue;
            };
            let mgr = Arc::clone(&self);
            tokio::spawn(async move {
                mgr.handle_incoming_stream(stream, TransportKind::WiFi)
                    .await;
            });
        }
    }

    async fn handle_incoming_stream(
        self: Arc<Self>,
        mut stream: TcpStream,
        actual_transport: TransportKind,
    ) {
        let Ok(raw_hello) = read_frame(&mut stream).await else {
            return;
        };
        let Ok(ControlMessage::Hello {
            protocol_domain,
            device_id,
            device_name,
            public_key_hex,
            ephemeral_x25519_hex,
            transport,
            require_pairing,
        }) = serde_json::from_slice::<ControlMessage>(&raw_hello)
        else {
            return;
        };

        if protocol_domain != APP_CONFIG.connection.protocol_domain {
            return;
        }

        if transport != actual_transport {
            let _ = send_control(
                &mut stream,
                &ControlMessage::Rejected {
                    code: RejectCode::AuthFailed,
                },
            )
            .await;
            return;
        }

        let decision = self.arbitrate_incoming(&device_id, actual_transport, require_pairing);
        let peer_x25519 = match decode_x25519(&ephemeral_x25519_hex) {
            Some(pk) => pk,
            None => return,
        };

        match decision {
            IncomingDecision::Reject(code) => {
                let _ = send_control(&mut stream, &ControlMessage::Rejected { code }).await;
            }
            IncomingDecision::TrustedChallenge { stored_pubkey } => {
                let handshake_res = self
                    .handshake
                    .execute_trusted_challenge(
                        stream,
                        &device_id,
                        &stored_pubkey,
                        peer_x25519,
                        &ephemeral_x25519_hex,
                        transport,
                    )
                    .await;
                if let Ok((stream, session_key)) = handshake_res {
                    Arc::clone(&self)
                        .enter_connected_loop(
                            stream,
                            device_id.clone(),
                            device_name,
                            transport,
                            session_key,
                        )
                        .await;
                }
            }
            IncomingDecision::RequireOtp => {
                let session_for_set = Arc::clone(&self);
                let session_for_trust = Arc::clone(&self);
                let session_for_clear = Arc::clone(&self);

                let handshake_res = self
                    .handshake
                    .execute_otp_pairing(
                        stream,
                        device_id.clone(),
                        device_name.clone(),
                        public_key_hex,
                        peer_x25519,
                        &ephemeral_x25519_hex,
                        transport,
                        move |pending, cancel_tx| {
                            let mut guard = session_for_set
                                .inner
                                .write()
                                .expect("session lock poisoned");
                            guard.handshake_reservation = None;
                            guard.pairing_cancel_tx = Some(cancel_tx);
                            guard.state = SessionState::Pairing(pending);
                        },
                        move |dev_id, dev_name, pubkey| {
                            let mut guard = session_for_trust
                                .inner
                                .write()
                                .expect("session lock poisoned");
                            guard.trust.upsert_trusted(dev_id, dev_name, pubkey);
                            guard.pairing_cancel_tx = None;
                        },
                        move |dev_id| {
                            session_for_clear.clear_pairing_if_matching(dev_id);
                        },
                    )
                    .await;

                if let Ok((stream, session_key)) = handshake_res {
                    Arc::clone(&self)
                        .enter_connected_loop(
                            stream,
                            device_id.clone(),
                            device_name,
                            transport,
                            session_key,
                        )
                        .await;
                }
            }
        }
        self.clear_handshake_reservation(&device_id, transport);
    }

    fn arbitrate_incoming(
        &self,
        device_id: &str,
        incoming_transport: TransportKind,
        require_pairing: bool,
    ) -> IncomingDecision {
        let mut guard = self.inner.write().expect("session lock poisoned");
        if guard.trust.is_blocked(device_id) {
            return IncomingDecision::Reject(RejectCode::Blocked);
        }
        if guard.handshake_reservation.is_some() {
            return IncomingDecision::Reject(RejectCode::Busy);
        }

        let trusted_pubkey = guard
            .trust
            .get(device_id)
            .filter(|d| !d.blocked)
            .map(|d| d.public_key_hex.clone());

        match guard.state.clone() {
            SessionState::Idle => {
                guard.handshake_reservation = Some(HandshakeReservation {
                    device_id: device_id.to_string(),
                    transport: incoming_transport,
                });
                if require_pairing {
                    IncomingDecision::RequireOtp
                } else {
                    match trusted_pubkey {
                        Some(pk) => IncomingDecision::TrustedChallenge { stored_pubkey: pk },
                        None => IncomingDecision::RequireOtp,
                    }
                }
            }
            SessionState::Pairing(_) => IncomingDecision::Reject(RejectCode::Busy),
            SessionState::Connected(active) => {
                let priority = guard.prefs.get().priority;
                let can_preempt = priority.outranks(incoming_transport, active.transport)
                    && trusted_pubkey.is_some()
                    && !require_pairing;
                match (can_preempt, trusted_pubkey) {
                    (true, Some(pk)) => {
                        if let Some(tx) = guard.active_cancel_tx.take() {
                            let _ = tx.send(true);
                        }
                        guard.state = SessionState::Idle;
                        guard.handshake_reservation = Some(HandshakeReservation {
                            device_id: device_id.to_string(),
                            transport: incoming_transport,
                        });
                        IncomingDecision::TrustedChallenge { stored_pubkey: pk }
                    }
                    _ => IncomingDecision::Reject(RejectCode::Busy),
                }
            }
        }
    }

    async fn enter_connected_loop(
        self: Arc<Self>,
        stream: TcpStream,
        device_id: String,
        device_name: String,
        transport: TransportKind,
        session_key: [u8; 32],
    ) {
        let (cancel_tx, cancel_rx) = watch::channel(false);
        let (speaker_policy_rx, mic_policy_rx, mouse_policy_rx) = {
            let mut guard = self.inner.write().expect("session lock poisoned");
            guard.handshake_reservation = None;
            guard.active_cancel_tx = Some(cancel_tx);
            guard.state = SessionState::Connected(ActiveSession {
                device_id: device_id.clone(),
                device_name,
                transport,
            });
            (
                guard.speaker_policy_tx.subscribe(),
                guard.mic_policy_tx.subscribe(),
                guard.mouse_policy_tx.subscribe(),
            )
        };

        self.active_channel
            .run_loop(
                stream,
                session_key,
                cancel_rx,
                speaker_policy_rx,
                mic_policy_rx,
                mouse_policy_rx,
            )
            .await;

        let mut guard = self.inner.write().expect("session lock poisoned");
        let still_same = matches!(
            &guard.state,
            SessionState::Connected(active) if active.device_id == device_id && active.transport == transport
        );
        if still_same {
            guard.active_cancel_tx = None;
            guard.state = SessionState::Idle;
        }
    }

    fn clear_pairing_if_matching(&self, device_id: &str) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        let matches_peer = matches!(
            &guard.state,
            SessionState::Pairing(p) if p.device_id == device_id
        );
        if matches_peer {
            guard.pairing_cancel_tx = None;
            guard.state = SessionState::Idle;
        }
    }

    fn clear_handshake_reservation(&self, device_id: &str, transport: TransportKind) {
        let mut guard = self.inner.write().expect("session lock poisoned");
        let matches_connection = guard
            .handshake_reservation
            .as_ref()
            .is_some_and(|reservation| {
                reservation.device_id == device_id && reservation.transport == transport
            });
        if matches_connection {
            guard.handshake_reservation = None;
        }
    }
}

enum IncomingDecision {
    Reject(RejectCode),
    TrustedChallenge { stored_pubkey: String },
    RequireOtp,
}
