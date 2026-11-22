use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use wirewave_core_config::APP_CONFIG;

const BEACON_MAGIC: [u8; 4] = *b"WW2D";
const PUBLIC_KEY_LEN: usize = 32;
const SIGNATURE_LEN: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostAnnouncement {
    pub desktop_id: String,
    pub pc_name: String,
    pub tcp_port: u16,
    pub bt_service_uuid: String,
    pub busy: bool,
}

/// Bytes signed by the desktop identity key for discovery announcements.
pub fn discovery_signing_payload(payload: &[u8]) -> Vec<u8> {
    let mut signed = Vec::with_capacity(
        APP_CONFIG.connection.protocol_domain.len() + 11 + payload.len(),
    );
    signed.extend_from_slice(APP_CONFIG.connection.protocol_domain.as_bytes());
    signed.extend_from_slice(b":discovery:");
    signed.extend_from_slice(payload);
    signed
}

/// Encodes `WW2D || Ed25519 public key || signature || payload`.
pub fn encode_signed_announcement(
    payload: &[u8],
    public_key: &[u8; PUBLIC_KEY_LEN],
    signature: &[u8; SIGNATURE_LEN],
) -> Vec<u8> {
    let mut packet = Vec::with_capacity(
        BEACON_MAGIC.len() + PUBLIC_KEY_LEN + SIGNATURE_LEN + payload.len(),
    );
    packet.extend_from_slice(&BEACON_MAGIC);
    packet.extend_from_slice(public_key);
    packet.extend_from_slice(signature);
    packet.extend_from_slice(payload);
    packet
}

pub fn encode_probe() -> Vec<u8> {
    [&BEACON_MAGIC[..], b"PROBE"].concat()
}

pub fn is_probe(raw: &[u8]) -> bool {
    raw == encode_probe()
}

/// Runs the WireWave-only authenticated UDP discovery responder on the configured port.
pub async fn run_udp_discovery_responder(
    desktop_id: String,
    pc_name: String,
    desktop_public_key_hex: String,
    sign_announcement: impl Fn(&[u8]) -> Vec<u8> + Send + Sync + 'static,
    is_busy: impl Fn() -> bool + Send + Sync + 'static,
) -> std::io::Result<()> {
    let port = APP_CONFIG.connection.udp_discovery_port;
    let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));
    let socket = UdpSocket::bind(bind_addr).await?;
    tracing::info!(port, "Authenticated WireWave UDP discovery responder listening");

    let mut buf = [0u8; 1024];
    loop {
        let Ok((len, peer)) = socket.recv_from(&mut buf).await else {
            continue;
        };
        if !is_probe(&buf[..len]) {
            continue;
        }

        let announcement = HostAnnouncement {
            desktop_id: desktop_id.clone(),
            pc_name: pc_name.clone(),
            tcp_port: APP_CONFIG.connection.tcp_port,
            bt_service_uuid: APP_CONFIG.connection.bt_service_uuid.to_string(),
            busy: is_busy(),
        };
        if let Ok(json) = serde_json::to_vec(&announcement) {
            let signing_payload = discovery_signing_payload(&json);
            let signature = sign_announcement(&signing_payload);
            let public_key = hex::decode(&desktop_public_key_hex)
                .ok()
                .and_then(|bytes| bytes.try_into().ok());
            let signature: Option<[u8; SIGNATURE_LEN]> = signature.try_into().ok();
            if let (Some(public_key), Some(signature)) = (public_key, signature) {
                let reply = encode_signed_announcement(&json, &public_key, &signature);
                let _ = socket.send_to(&reply, peer).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_has_wirewave_magic_and_exact_payload() {
        let probe = encode_probe();
        assert!(is_probe(&probe));
        assert!(!is_probe(b"WW2DPROBE-extra"));
    }
}
