use rand_core::{OsRng, RngCore};
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::watch, time};
use wirewave_core_config::APP_CONFIG;
use wirewave_core_crypto::{
    compute_exchange_hash, derive_session_key, generate_otp, verify_otp_proof,
    verify_peer_signature, IdentityKeypair,
};
use wirewave_core_transport::{
    read_frame, write_frame, ControlMessage, RejectCode, TransportKind,
};
use x25519_dalek::{EphemeralSecret, PublicKey};
use crate::session_manager::PendingPairing;

pub struct HandshakeCoordinator {
    desktop_id: String,
    pc_name: String,
    keypair: Arc<IdentityKeypair>,
}

impl HandshakeCoordinator {
    pub fn new(desktop_id: String, pc_name: String, keypair: Arc<IdentityKeypair>) -> Self {
        Self {
            desktop_id,
            pc_name,
            keypair,
        }
    }

    /// Authenticates a previously-trusted peer via cryptographic challenge-response signature.
    pub async fn execute_trusted_challenge(
        &self,
        mut stream: TcpStream,
        device_id: &str,
        stored_pubkey: &str,
        peer_x25519: PublicKey,
        client_x25519_hex: &str,
        transport: TransportKind,
    ) -> Result<(TcpStream, [u8; 32]), ()> {
        let server_ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let server_ephemeral_pub = PublicKey::from(&server_ephemeral);
        let server_x25519_hex = hex::encode(server_ephemeral_pub.as_bytes());
        let shared = server_ephemeral.diffie_hellman(&peer_x25519);

        let mut nonce = [0u8; 32];
        OsRng.fill_bytes(&mut nonce);
        let nonce_hex = hex::encode(nonce);

        let challenge = ControlMessage::Challenge {
            desktop_id: self.desktop_id.clone(),
            desktop_name: self.pc_name.clone(),
            desktop_public_key_hex: self.keypair.public_key_hex(),
            ephemeral_x25519_hex: server_x25519_hex.clone(),
            nonce_hex: nonce_hex.clone(),
        };
        if send_control(&mut stream, &challenge).await.is_err() {
            return Err(());
        }

        let Ok(raw_reply) = read_frame(&mut stream).await else {
            return Err(());
        };
        let Ok(ControlMessage::ChallengeReply { signature_hex }) =
            serde_json::from_slice::<ControlMessage>(&raw_reply)
        else {
            return Err(());
        };

        let exchange_hash = compute_exchange_hash(
            &self.desktop_id,
            device_id,
            &self.keypair.public_key_hex(),
            stored_pubkey,
            &server_x25519_hex,
            client_x25519_hex,
            &nonce_hex,
        );
        let sig_valid = hex::decode(&signature_hex)
            .ok()
            .and_then(|sig| verify_peer_signature(stored_pubkey, &exchange_hash, &sig).ok())
            .is_some();

        if !sig_valid {
            let _ = send_control(
                &mut stream,
                &ControlMessage::Rejected {
                    code: RejectCode::AuthFailed,
                },
            )
            .await;
            return Err(());
        }

        let desktop_sig = hex::encode(self.keypair.sign(&exchange_hash));
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
            return Err(());
        }

        let session_key = derive_session_key(shared.as_bytes(), &exchange_hash);
        Ok((stream, session_key))
    }

    /// Executes the first-time 6-digit OTP pairing exchange.
    pub async fn execute_otp_pairing<FSetPairing, FSaveTrust, FClearPairing>(
        &self,
        mut stream: TcpStream,
        device_id: String,
        device_name: String,
        peer_pubkey_hex: String,
        peer_x25519: PublicKey,
        client_x25519_hex: &str,
        transport: TransportKind,
        on_start_pairing: FSetPairing,
        on_pair_success: FSaveTrust,
        on_pairing_end: FClearPairing,
    ) -> Result<(TcpStream, [u8; 32]), ()>
    where
        FSetPairing: FnOnce(PendingPairing, watch::Sender<bool>),
        FSaveTrust: FnOnce(String, String, String),
        FClearPairing: FnOnce(&str),
    {
        let server_ephemeral = EphemeralSecret::random_from_rng(OsRng);
        let server_ephemeral_pub = PublicKey::from(&server_ephemeral);
        let server_x25519_hex = hex::encode(server_ephemeral_pub.as_bytes());
        let shared = server_ephemeral.diffie_hellman(&peer_x25519);

        let otp_code = generate_otp();
        let ttl_secs = APP_CONFIG.connection.otp_ttl_secs;
        let (cancel_tx, mut cancel_rx) = watch::channel(false);

        let pending = PendingPairing {
            device_id: device_id.clone(),
            device_name: device_name.clone(),
            transport,
            otp_code: otp_code.clone(),
            expires_at: std::time::Instant::now() + Duration::from_secs(ttl_secs),
            ttl_secs,
        };
        on_start_pairing(pending, cancel_tx);

        let prompt = ControlMessage::OtpRequired {
            desktop_id: self.desktop_id.clone(),
            desktop_name: self.pc_name.clone(),
            desktop_public_key_hex: self.keypair.public_key_hex(),
            ephemeral_x25519_hex: server_x25519_hex.clone(),
            ttl_secs,
        };
        if send_control(&mut stream, &prompt).await.is_err() {
            on_pairing_end(&device_id);
            return Err(());
        }

        let exchange_hash = compute_exchange_hash(
            &self.desktop_id,
            &device_id,
            &self.keypair.public_key_hex(),
            &peer_pubkey_hex,
            &server_x25519_hex,
            client_x25519_hex,
            "",
        );
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
                .map(|p| verify_otp_proof(shared.as_bytes(), &otp_code, &exchange_hash, &p))
                .unwrap_or(false);
            let sig_ok = hex::decode(&signature_hex)
                .ok()
                .and_then(|s| verify_peer_signature(&peer_pubkey_hex, &exchange_hash, &s).ok())
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
            on_pairing_end(&device_id);
            return Err(());
        }

        on_pair_success(device_id.clone(), device_name, peer_pubkey_hex);

        let desktop_sig = hex::encode(self.keypair.sign(&exchange_hash));
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
            on_pairing_end(&device_id);
            return Err(());
        }

        let session_key = derive_session_key(shared.as_bytes(), &exchange_hash);
        Ok((stream, session_key))
    }
}

pub async fn send_control(stream: &mut TcpStream, msg: &ControlMessage) -> std::io::Result<()> {
    let payload = serde_json::to_vec(msg).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    write_frame(stream, &payload).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn decode_x25519(hex_str: &str) -> Option<PublicKey> {
    let bytes = hex::decode(hex_str).ok()?;
    if bytes.len() != 32 {
        return None;
    }
    let mut fixed = [0u8; 32];
    fixed.copy_from_slice(&bytes);
    Some(PublicKey::from(fixed))
}
