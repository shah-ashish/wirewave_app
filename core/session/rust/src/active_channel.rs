use crate::feature_registry::SharedFeatureRegistry;
use rand_core::{OsRng, RngCore};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{net::TcpStream, sync::watch, time};
use wirewave_core_config::APP_CONFIG;
use wirewave_core_crypto::{CipherRole, SessionCipher};
use wirewave_core_stream::{ChannelId, UdpDataChannel, derive_udp_key};
use wirewave_core_transport::{ControlMessage, read_frame, write_frame};

/// Encapsulates the active, encrypted TCP control loop for an established session.
///
/// Handles heartbeats, incoming feature start/stop commands, policy updates,
/// and delegates real-time UDP stream setup to registered `FeatureHost` implementations.
pub struct ActiveControlChannel {
    feature_registry: SharedFeatureRegistry,
}

impl ActiveControlChannel {
    pub fn new(feature_registry: SharedFeatureRegistry) -> Self {
        Self { feature_registry }
    }

    pub async fn run_loop(
        &self,
        mut stream: TcpStream,
        session_key: [u8; 32],
        mut cancel_rx: watch::Receiver<bool>,
        mut speaker_policy_rx: watch::Receiver<bool>,
        mut mic_policy_rx: watch::Receiver<bool>,
        mut mouse_policy_rx: watch::Receiver<bool>,
    ) {
        let mut cipher = SessionCipher::new(session_key, CipherRole::Desktop);

        // Send initial feature states:
        let initial_speaker_allowed = *speaker_policy_rx.borrow();
        if let Some(host) = self.feature_registry.get(ChannelId::Speaker) {
            let initial_state = host.make_state_message(
                initial_speaker_allowed,
                false,
                (!initial_speaker_allowed).then(|| "Blocked by desktop settings".into()),
                None,
                None,
                None,
            );
            if send_encrypted(&mut stream, &mut cipher, &initial_state)
                .await
                .is_err()
            {
                return;
            }
        }

        let mic_permission = *mic_policy_rx.borrow();
        if let Some(host) = self.feature_registry.get(ChannelId::Mic) {
            let mic_driver = host.status_detail();
            let mic_allowed = mic_permission && mic_driver.is_some();
            let mic_detail = match (mic_permission, mic_driver) {
                (false, _) => Some("Blocked by desktop settings".into()),
                (true, None) => Some("Compatible virtual microphone driver required".into()),
                (true, Some(driver)) => Some(driver),
            };
            let initial_mic_state =
                host.make_state_message(mic_allowed, false, mic_detail, None, None, None);
            if send_encrypted(&mut stream, &mut cipher, &initial_mic_state)
                .await
                .is_err()
            {
                return;
            }
        }

        let initial_mouse_allowed = *mouse_policy_rx.borrow();
        if let Some(host) = self.feature_registry.get(ChannelId::Mouse) {
            let mouse_detail = match initial_mouse_allowed {
                false => Some("Blocked by desktop settings".into()),
                true => host.status_detail(),
            };
            let initial_mouse_state = host.make_state_message(
                initial_mouse_allowed,
                false,
                mouse_detail,
                None,
                None,
                None,
            );
            if send_encrypted(&mut stream, &mut cipher, &initial_mouse_state)
                .await
                .is_err()
            {
                return;
            }
        }

        let hb_interval = Duration::from_secs(APP_CONFIG.connection.heartbeat_interval_secs);
        let hb_timeout = Duration::from_secs(APP_CONFIG.connection.heartbeat_timeout_secs);
        let mut ticker = time::interval(hb_interval);
        let mut seq: u64 = 0;
        let mut last_seen = Instant::now();
        let mut speaker_hello_task: Option<tokio::task::JoinHandle<()>> = None;

        loop {
            tokio::select! {
                _ = cancel_rx.changed() => {
                    let msg = ControlMessage::Disconnect { reason: "server_disconnect".into() };
                    let _ = send_encrypted(&mut stream, &mut cipher, &msg).await;
                    break;
                }
                changed = speaker_policy_rx.changed() => {
                    if changed.is_err() {
                        break;
                    }
                    let allowed = *speaker_policy_rx.borrow();
                    if let Some(host) = self.feature_registry.get(ChannelId::Speaker) {
                        if !allowed {
                            if let Some(task) = speaker_hello_task.take() {
                                task.abort();
                            }
                            host.stop_stream();
                        }
                        let state = host.make_state_message(
                            allowed,
                            false,
                            (!allowed).then(|| "Blocked by desktop settings".into()),
                            None,
                            None,
                            None,
                        );
                        if send_encrypted(&mut stream, &mut cipher, &state).await.is_err() {
                            break;
                        }
                    }
                }
                changed = mic_policy_rx.changed() => {
                    if changed.is_err() {
                        break;
                    }
                    let permission = *mic_policy_rx.borrow();
                    if let Some(host) = self.feature_registry.get(ChannelId::Mic) {
                        host.stop_stream();
                        let driver = host.status_detail();
                        let allowed = permission && driver.is_some();
                        let detail = match (permission, driver) {
                            (false, _) => Some("Blocked by desktop settings".into()),
                            (true, None) => Some("Compatible virtual microphone driver required".into()),
                            (true, Some(name)) => Some(name),
                        };
                        let state = host.make_state_message(
                            allowed,
                            false,
                            detail,
                            None,
                            None,
                            None,
                        );
                        if send_encrypted(&mut stream, &mut cipher, &state).await.is_err() {
                            break;
                        }
                    }
                }
                changed = mouse_policy_rx.changed() => {
                    if changed.is_err() {
                        break;
                    }
                    let allowed = *mouse_policy_rx.borrow();
                    if let Some(host) = self.feature_registry.get(ChannelId::Mouse) {
                        if !allowed {
                            host.stop_stream();
                        }
                        let detail = match allowed {
                            false => Some("Blocked by desktop settings".into()),
                            true => host.status_detail(),
                        };
                        let state = host.make_state_message(
                            allowed,
                            false,
                            detail,
                            None,
                            None,
                            None,
                        );
                        if send_encrypted(&mut stream, &mut cipher, &state).await.is_err() {
                            break;
                        }
                    }
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
                        ControlMessage::SpeakerCommand { active: false } => {
                            if let Some(task) = speaker_hello_task.take() {
                                task.abort();
                            }
                            if let Some(host) = self.feature_registry.get(ChannelId::Speaker) {
                                host.stop_stream();
                                let state = host.make_state_message(
                                    *speaker_policy_rx.borrow(),
                                    false,
                                    None,
                                    None,
                                    None,
                                    None,
                                );
                                let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                            }
                        }
                        ControlMessage::SpeakerCommand { active: true } => {
                            let allowed = *speaker_policy_rx.borrow();
                            if let Some(host) = self.feature_registry.get(ChannelId::Speaker) {
                                if !allowed {
                                    let state = host.make_state_message(
                                        false,
                                        false,
                                        Some("Blocked by desktop settings".into()),
                                        None,
                                        None,
                                        None,
                                    );
                                    let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    continue;
                                }

                                let stream_id = OsRng.next_u32();
                                let udp_key = derive_udp_key(&session_key, stream_id);
                                match UdpDataChannel::bind(udp_key, stream_id).await {
                                    Ok((channel, endpoint)) => {
                                        if let Some(task) = speaker_hello_task.take() {
                                            task.abort();
                                        }
                                        let accept_channel = Arc::clone(&channel);
                                        speaker_hello_task = Some(tokio::spawn(async move {
                                            let _ = accept_channel.accept_peer_hello().await;
                                        }));
                                        let detail = host.start_stream(channel, tokio::runtime::Handle::current()).ok().flatten();
                                        let state = host.make_state_message(
                                            true,
                                            true,
                                            detail,
                                            Some(endpoint.local_port),
                                            Some(endpoint.stream_id),
                                            Some(hex::encode(udp_key)),
                                        );
                                        let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    }
                                    Err(err) => {
                                        tracing::warn!(%err, "Unable to bind speaker UDP stream");
                                        let state = host.make_state_message(
                                            true,
                                            false,
                                            Some("Unable to open audio stream".into()),
                                            None,
                                            None,
                                            None,
                                        );
                                        let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    }
                                }
                            }
                        }
                        ControlMessage::MicCommand { active: false } => {
                            if let Some(host) = self.feature_registry.get(ChannelId::Mic) {
                                host.stop_stream();
                                let permission = *mic_policy_rx.borrow();
                                let driver = host.status_detail();
                                let allowed = permission && driver.is_some();
                                let state = host.make_state_message(
                                    allowed,
                                    false,
                                    match (permission, driver) {
                                        (false, _) => Some("Blocked by desktop settings".into()),
                                        (true, None) => Some("Compatible virtual microphone driver required".into()),
                                        (true, Some(name)) => Some(name),
                                    },
                                    None,
                                    None,
                                    None,
                                );
                                let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                            }
                        }
                        ControlMessage::MicCommand { active: true } => {
                            let permission = *mic_policy_rx.borrow();
                            if let Some(host) = self.feature_registry.get(ChannelId::Mic) {
                                let driver = host.status_detail();
                                if !permission || driver.is_none() {
                                    let state = host.make_state_message(
                                        false,
                                        false,
                                        if permission {
                                            Some("Compatible virtual microphone driver required".into())
                                        } else {
                                            Some("Blocked by desktop settings".into())
                                        },
                                        None,
                                        None,
                                        None,
                                    );
                                    let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    continue;
                                }

                                host.stop_stream();
                                let stream_id = OsRng.next_u32();
                                let udp_key = derive_udp_key(&session_key, stream_id);
                                match UdpDataChannel::bind(udp_key, stream_id).await {
                                    Ok((channel, endpoint)) => {
                                        match host.start_stream(channel, tokio::runtime::Handle::current()) {
                                            Ok(driver_name) => {
                                                let state = host.make_state_message(
                                                    true,
                                                    true,
                                                    driver_name,
                                                    Some(endpoint.local_port),
                                                    Some(endpoint.stream_id),
                                                    Some(hex::encode(udp_key)),
                                                );
                                                let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                            }
                                            Err(error) => {
                                                let state = host.make_state_message(
                                                    true,
                                                    false,
                                                    Some(error),
                                                    None,
                                                    None,
                                                    None,
                                                );
                                                let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        tracing::warn!(%error, "Unable to bind microphone UDP stream");
                                        let state = host.make_state_message(
                                            true,
                                            false,
                                            Some("Unable to open microphone stream".into()),
                                            None,
                                            None,
                                            None,
                                        );
                                        let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    }
                                }
                            }
                        }
                        ControlMessage::MouseCommand { active: false } => {
                            if let Some(host) = self.feature_registry.get(ChannelId::Mouse) {
                                host.stop_stream();
                                let allowed = *mouse_policy_rx.borrow();
                                let detail = match allowed {
                                    false => Some("Blocked by desktop settings".into()),
                                    true => host.status_detail(),
                                };
                                let state = host.make_state_message(
                                    allowed,
                                    false,
                                    detail,
                                    None,
                                    None,
                                    None,
                                );
                                let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                            }
                        }
                        ControlMessage::MouseCommand { active: true } => {
                            let allowed = *mouse_policy_rx.borrow();
                            if let Some(host) = self.feature_registry.get(ChannelId::Mouse) {
                                if !allowed {
                                    let state = host.make_state_message(
                                        false,
                                        false,
                                        Some("Blocked by desktop settings".into()),
                                        None,
                                        None,
                                        None,
                                    );
                                    let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    continue;
                                }

                                host.stop_stream();
                                let stream_id = OsRng.next_u32();
                                let udp_key = derive_udp_key(&session_key, stream_id);
                                match UdpDataChannel::bind(udp_key, stream_id).await {
                                    Ok((channel, endpoint)) => {
                                        let detail = host.start_stream(channel, tokio::runtime::Handle::current()).ok().flatten();
                                        let state = host.make_state_message(
                                            true,
                                            true,
                                            detail,
                                            Some(endpoint.local_port),
                                            Some(endpoint.stream_id),
                                            Some(hex::encode(udp_key)),
                                        );
                                        let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    }
                                    Err(err) => {
                                        tracing::warn!(%err, "Unable to bind mouse UDP stream");
                                        let state = host.make_state_message(
                                            true,
                                            false,
                                            Some("Unable to open mouse stream".into()),
                                            None,
                                            None,
                                            None,
                                        );
                                        let _ = send_encrypted(&mut stream, &mut cipher, &state).await;
                                    }
                                }
                            }
                        }
                        ControlMessage::KeyInput { text, key_code } => {
                            crate::key_injector::inject_key_input(text.as_deref(), key_code.as_deref());
                        }
                        ControlMessage::Disconnect { .. } => break,
                        _ => {}
                    }
                }
            }
        }

        if let Some(task) = speaker_hello_task.take() {
            task.abort();
        }
        self.feature_registry.stop_all();
    }
}

async fn send_encrypted(
    stream: &mut TcpStream,
    cipher: &mut SessionCipher,
    msg: &ControlMessage,
) -> Result<(), ()> {
    let payload = serde_json::to_vec(msg).map_err(|_| ())?;
    let encrypted = cipher.encrypt(&payload).map_err(|_| ())?;
    write_frame(stream, &encrypted).await.map_err(|_| ())
}
