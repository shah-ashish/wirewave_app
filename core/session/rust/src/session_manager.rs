use crate::{
    settings_store::{ConnectionPreferences, PreferencesStore},
    trust_store::{TrustStore, TrustedDevice},
};
use rand_core::{OsRng, RngCore};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::watch,
    time,
};
use wirewave_core_config::APP_CONFIG;
use wirewave_core_crypto::{
    derive_session_key, generate_otp, verify_otp_proof, verify_peer_signature, IdentityKeypair,
    CipherRole, SessionCipher,
};
use wirewave_core_discovery::run_udp_discovery_responder;
use wirewave_core_identity::DeviceIdentity;
use wirewave_core_transport::{
    read_frame, write_frame, ControlMessage, RejectCode, TransportKind, TransportPriority,
};
use x25519_dalek::{EphemeralSecret, PublicKey};

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
}

#[derive(Debug, Clone)]
struct HandshakeReservation {
    device_id: String,
    transport: TransportKind,
}

pub type SharedSession = Arc<SessionManager>;

/// Owns the 1-to-1 desktop session lock, trust agreement store, priority arbitration,
/// and background encrypted listeners (Wi-Fi TCP + UDP authenticated discovery).
pub struct SessionManager {
    desktop_id: String,
    pc_name: String,
    keypair: IdentityKeypair,
    inner: RwLock<SessionInner>,
}

impl SessionManager {
    pub fn new(identity: DeviceIdentity, keypair: IdentityKeypair) -> SharedSession {
        let pc_name = hostname::get()
            .ok()
            .and_then(|s| s.into_string().ok())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "Windows PC".to_string());

        Arc::new(Self {
            desktop_id: identity.device_id,
            pc_name,
            keypair,
            inner: RwLock::new(SessionInner {
                state: SessionState::Idle,
                handshake_reservation: None,
                trust: TrustStore::load_default(),
                prefs: PreferencesStore::load_default(),
                active_cancel_tx: None,
                pairing_cancel_tx: None,
                wifi_listening: false,
                bluetooth_listening: false,
            }),
        })
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

    /// Spawns the dedicated background Tokio runtime hosting the authenticated UDP
    /// discovery responder and the WireWave-exclusive encrypted TCP listener.
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
                        if let Err(err) =
                            run_udp_discovery_responder(
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
                                        mgr.handle_incoming_stream(stream).await;
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
                mgr.handle_incoming_stream(stream).await;
            });
        }
    }

    async fn handle_incoming_stream(self: Arc<Self>, mut stream: TcpStream) {
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
        }) = serde_json::from_slice::<ControlMessage>(&raw_hello)
        else {
            return;
        };

        // Strict app-to-app isolation: reject any client with wrong protocol domain.
        if protocol_domain != APP_CONFIG.connection.protocol_domain {
            return;
        }

        // Arbitrate lock, blocklist, trust, and transport priority.
        let decision = self.arbitrate_incoming(&device_id, transport);
        let peer_x25519 = match decode_x25519(&ephemeral_x25519_hex) {
            Some(pk) => pk,
            None => return,
        };

        match decision {
            IncomingDecision::Reject(code) => {
                let _ = send_control(&mut stream, &ControlMessage::Rejected { code }).await;
            }
            IncomingDecision::TrustedChallenge { stored_pubkey } => {
                Arc::clone(&self).run_trusted_handshake(
                    stream,
                    device_id.clone(),
                    device_name,
                    stored_pubkey,
                    peer_x25519,
                    transport,
                )
                .await;
            }
            IncomingDecision::RequireOtp => {
                Arc::clone(&self).run_otp_pairing(
                    stream,
                    device_id.clone(),
                    device_name,
                    public_key_hex,
                    peer_x25519,
                    transport,
                )
                .await;
            }
        }
        self.clear_handshake_reservation(&device_id, transport);
    }

    fn arbitrate_incoming(&self, device_id: &str, incoming_transport: TransportKind) -> IncomingDecision {
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
                match trusted_pubkey {
                    Some(pk) => IncomingDecision::TrustedChallenge { stored_pubkey: pk },
                    None => IncomingDecision::RequireOtp,
                }
            }
            SessionState::Pairing(_) => IncomingDecision::Reject(RejectCode::Busy),
            SessionState::Connected(active) => {
                // Preempt only if the incoming link strictly outranks the current link AND the peer is trusted.
                let priority = guard.prefs.get().priority;
                let can_preempt =
                    priority.outranks(incoming_transport, active.transport) && trusted_pubkey.is_some();
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

    async fn run_trusted_handshake(
        self: Arc<Self>,
        mut stream: TcpStream,
        device_id: String,
        device_name: String,
        stored_pubkey: String,
        peer_x25519: PublicKey,
        transport: TransportKind,
    ) {
        let server_ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let server_ephemeral_pub = PublicKey::from(&server_ephemeral);
        let shared = server_ephemeral.diffie_hellman(&peer_x25519);

        let mut nonce = [0u8; 32];
        OsRng.fill_bytes(&mut nonce);
        let nonce_hex = hex::encode(nonce);

        let challenge = ControlMessage::Challenge {
            desktop_id: self.desktop_id.clone(),
            desktop_name: self.pc_name.clone(),
            desktop_public_key_hex: self.keypair.public_key_hex(),
            ephemeral_x25519_hex: hex::encode(server_ephemeral_pub.as_bytes()),
            nonce_hex: nonce_hex.clone(),
        };
        if send_control(&mut stream, &challenge).await.is_err() {
            return;
        }

        let Ok(raw_reply) = read_frame(&mut stream).await else {
            return;
        };
        let Ok(ControlMessage::ChallengeReply { signature_hex }) =
            serde_json::from_slice::<ControlMessage>(&raw_reply)
        else {
            return;
        };

        let transcript = format!("{}:{}:{}", self.desktop_id, device_id, nonce_hex);
        let sig_valid = hex::decode(&signature_hex)
            .ok()
            .and_then(|sig| verify_peer_signature(&stored_pubkey, transcript.as_bytes(), &sig).ok())
            .is_some();

        if !sig_valid {
            let _ = send_control(
                &mut stream,
                &ControlMessage::Rejected {
                    code: RejectCode::AuthFailed,
                },
            )
            .await;
            return;
        }

        let desktop_sig = hex::encode(self.keypair.sign(transcript.as_bytes()));
        if send_control(
            &mut stream,
            &ControlMessage::SessionReady {
                desktop_signature_hex: desktop_sig,
                transport,
            },
        )
        .await
        .is_err()
        {
            return;
        }

        let session_key = derive_session_key(shared.as_bytes(), transcript.as_bytes());
        self.enter_connected_loop(
            stream,
            device_id,
            device_name,
            transport,
            SessionCipher::new(session_key, CipherRole::Desktop),
        )
            .await;
    }

    async fn run_otp_pairing(
        self: Arc<Self>,
        mut stream: TcpStream,
        device_id: String,
        device_name: String,
        peer_pubkey_hex: String,
        peer_x25519: PublicKey,
        transport: TransportKind,
    ) {
        let server_ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let server_ephemeral_pub = PublicKey::from(&server_ephemeral);
        let shared = server_ephemeral.diffie_hellman(&peer_x25519);

        let otp_code = generate_otp();
        let ttl_secs = APP_CONFIG.connection.otp_ttl_secs;
        let (cancel_tx, mut cancel_rx) = watch::channel(false);

        {
            let mut guard = self.inner.write().expect("session lock poisoned");
            guard.handshake_reservation = None;
            guard.pairing_cancel_tx = Some(cancel_tx);
            guard.state = SessionState::Pairing(PendingPairing {
                device_id: device_id.clone(),
                device_name: device_name.clone(),
                transport,
                otp_code: otp_code.clone(),
                expires_at: Instant::now() + Duration::from_secs(ttl_secs),
                ttl_secs,
            });
        }

        let prompt = ControlMessage::OtpRequired {
            desktop_id: self.desktop_id.clone(),
            desktop_name: self.pc_name.clone(),
            desktop_public_key_hex: self.keypair.public_key_hex(),
            ephemeral_x25519_hex: hex::encode(server_ephemeral_pub.as_bytes()),
            ttl_secs,
        };
        if send_control(&mut stream, &prompt).await.is_err() {
            self.clear_pairing_if_matching(&device_id);
            return;
        }

        let transcript = format!("{}:{}", self.desktop_id, device_id);
        let deadline = time::Instant::now() + Duration::from_secs(ttl_secs);
        let max_attempts = APP_CONFIG.connection.otp_max_attempts;
        let mut verified = false;

        for _ in 0..max_attempts {
            let incoming = tokio::select! {
                _ = time::sleep_until(deadline) => {
                    let _ = send_control(&mut stream, &ControlMessage::Rejected { code: RejectCode::OtpExpired }).await;
                    break;
                }
                _ = cancel_rx.changed() => {
                    let _ = send_control(&mut stream, &ControlMessage::Rejected { code: RejectCode::Cancelled }).await;
                    break;
                }
                frame = read_frame(&mut stream) => frame,
            };

            let Ok(raw) = incoming else {
                break;
            };
            let Ok(ControlMessage::OtpProof {
                proof_hex,
                signature_hex,
            }) = serde_json::from_slice::<ControlMessage>(&raw)
            else {
                break;
            };

            let proof_ok = hex::decode(&proof_hex)
                .map(|p| verify_otp_proof(shared.as_bytes(), &otp_code, transcript.as_bytes(), &p))
                .unwrap_or(false);
            let sig_ok = hex::decode(&signature_hex)
                .ok()
                .and_then(|s| verify_peer_signature(&peer_pubkey_hex, transcript.as_bytes(), &s).ok())
                .is_some();

            if proof_ok && sig_ok {
                verified = true;
                break;
            }

            let _ = send_control(
                &mut stream,
                &ControlMessage::Rejected {
                    code: RejectCode::InvalidOtp,
                },
            )
            .await;
        }

        if !verified {
            self.clear_pairing_if_matching(&device_id);
            return;
        }

        {
            let mut guard = self.inner.write().expect("session lock poisoned");
            guard.trust.upsert_trusted(
                device_id.clone(),
                device_name.clone(),
                peer_pubkey_hex,
            );
            guard.pairing_cancel_tx = None;
        }

        let desktop_sig = hex::encode(self.keypair.sign(transcript.as_bytes()));
        if send_control(
            &mut stream,
            &ControlMessage::SessionReady {
                desktop_signature_hex: desktop_sig,
                transport,
            },
        )
        .await
        .is_err()
        {
            self.clear_pairing_if_matching(&device_id);
            return;
        }

        let session_key = derive_session_key(shared.as_bytes(), transcript.as_bytes());
        self.enter_connected_loop(
            stream,
            device_id,
            device_name,
            transport,
            SessionCipher::new(session_key, CipherRole::Desktop),
        )
            .await;
    }

    async fn enter_connected_loop(
        self: Arc<Self>,
        mut stream: TcpStream,
        device_id: String,
        device_name: String,
        transport: TransportKind,
        mut cipher: SessionCipher,
    ) {
        let (cancel_tx, mut cancel_rx) = watch::channel(false);
        {
            let mut guard = self.inner.write().expect("session lock poisoned");
            guard.handshake_reservation = None;
            guard.active_cancel_tx = Some(cancel_tx);
            guard.state = SessionState::Connected(ActiveSession {
                device_id: device_id.clone(),
                device_name,
                transport,
            });
        }

        let hb_interval = Duration::from_secs(APP_CONFIG.connection.heartbeat_interval_secs);
        let hb_timeout = Duration::from_secs(APP_CONFIG.connection.heartbeat_timeout_secs);
        let mut ticker = time::interval(hb_interval);
        let mut seq: u64 = 0;
        let mut last_seen = Instant::now();

        loop {
            tokio::select! {
                _ = cancel_rx.changed() => {
                    let msg = ControlMessage::Disconnect { reason: "server_disconnect".into() };
                    let _ = send_encrypted(&mut stream, &mut cipher, &msg).await;
                    break;
                }
                _ = ticker.tick() => {
                    if last_seen.elapsed() > hb_timeout {
                        break;
                    }
                    seq = seq.wrapping_add(1);
                    let ping = ControlMessage::HeartbeatPing { seq };
                    if send_encrypted(&mut stream, &mut cipher, &ping).await.is_err() {
                        break;
                    }
                }
                frame = read_frame(&mut stream) => {
                    let Ok(encrypted) = frame else { break; };
                    let Ok(plain) = cipher.decrypt(&encrypted) else { break; };
                    let Ok(msg) = serde_json::from_slice::<ControlMessage>(&plain) else { break; };
                    last_seen = Instant::now();
                    match msg {
                        ControlMessage::HeartbeatPing { seq } => {
                            let _ = send_encrypted(&mut stream, &mut cipher, &ControlMessage::HeartbeatPong { seq }).await;
                        }
                        ControlMessage::HeartbeatPong { .. } => {}
                        ControlMessage::Disconnect { .. } => break,
                        _ => {}
                    }
                }
            }
        }

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

fn decode_x25519(hex_str: &str) -> Option<PublicKey> {
    let bytes: [u8; 32] = hex::decode(hex_str).ok()?.try_into().ok()?;
    Some(PublicKey::from(bytes))
}

async fn send_control(stream: &mut TcpStream, msg: &ControlMessage) -> Result<(), ()> {
    let bytes = serde_json::to_vec(msg).map_err(|_| ())?;
    write_frame(stream, &bytes).await.map_err(|_| ())
}

async fn send_encrypted(
    stream: &mut TcpStream,
    cipher: &mut SessionCipher,
    msg: &ControlMessage,
) -> Result<(), ()> {
    let plain = serde_json::to_vec(msg).map_err(|_| ())?;
    let encrypted = cipher.encrypt(&plain).map_err(|_| ())?;
    write_frame(stream, &encrypted).await.map_err(|_| ())
}
