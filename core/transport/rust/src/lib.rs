use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const FRAME_MAGIC: [u8; 4] = *b"WW2F";
const MAX_FRAME_BYTES: usize = 64 * 1024;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("invalid protocol magic — non-WireWave client rejected")]
    InvalidProtocolMagic,
    #[error("frame exceeds maximum allowed size")]
    FrameTooLarge,
    #[error("transport I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("message codec error: {0}")]
    Codec(#[from] serde_json::Error),
}

/// Active link transport between Desktop and Android.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    WiFi,
    Bluetooth,
}

impl TransportKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::WiFi => "Wi-Fi",
            Self::Bluetooth => "Bluetooth",
        }
    }
}

/// Server-side connection priority preference selectable in Desktop Settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportPriority {
    WifiFirst,
    BluetoothFirst,
}

impl TransportPriority {
    pub fn from_config(raw: &str) -> Self {
        match raw.to_ascii_lowercase().as_str() {
            "bluetooth" | "bluetooth_first" => Self::BluetoothFirst,
            _ => Self::WifiFirst,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::WifiFirst => "Wi-Fi (Default)",
            Self::BluetoothFirst => "Bluetooth",
        }
    }

    pub fn rank(self, kind: TransportKind) -> u8 {
        match (self, kind) {
            (Self::WifiFirst, TransportKind::WiFi)
            | (Self::BluetoothFirst, TransportKind::Bluetooth) => 1,
            _ => 2,
        }
    }

    /// Returns `true` when `incoming` strictly outranks `current`.
    pub fn outranks(self, incoming: TransportKind, current: TransportKind) -> bool {
        self.rank(incoming) < self.rank(current)
    }
}

/// WireWave-exclusive control frames (serialized inside encrypted or handshake envelopes).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    Hello {
        protocol_domain: String,
        device_id: String,
        device_name: String,
        public_key_hex: String,
        ephemeral_x25519_hex: String,
        transport: TransportKind,
        #[serde(default)]
        require_pairing: bool,
    },
    Challenge {
        desktop_id: String,
        desktop_name: String,
        desktop_public_key_hex: String,
        ephemeral_x25519_hex: String,
        nonce_hex: String,
    },
    ChallengeReply {
        signature_hex: String,
    },
    OtpRequired {
        desktop_id: String,
        desktop_name: String,
        desktop_public_key_hex: String,
        ephemeral_x25519_hex: String,
        ttl_secs: u64,
    },
    OtpProof {
        proof_hex: String,
        signature_hex: String,
    },
    SessionReady {
        desktop_signature_hex: String,
        transport: TransportKind,
    },
    HeartbeatPing {
        seq: u64,
    },
    HeartbeatPong {
        seq: u64,
    },
    Disconnect {
        reason: String,
    },
    Rejected {
        code: RejectCode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectCode {
    Busy,
    Blocked,
    InvalidOtp,
    OtpExpired,
    Cancelled,
    AuthFailed,
}

/// Writes a length-prefixed WireWave binary frame (`WW2F` + u32 BE len + payload).
pub async fn write_frame<W: AsyncWrite + Unpin>(
    writer: &mut W,
    payload: &[u8],
) -> Result<(), TransportError> {
    if payload.len() > MAX_FRAME_BYTES {
        return Err(TransportError::FrameTooLarge);
    }
    writer.write_all(&FRAME_MAGIC).await?;
    writer.write_all(&(payload.len() as u32).to_be_bytes()).await?;
    writer.write_all(payload).await?;
    writer.flush().await?;
    Ok(())
}

/// Reads and validates a WireWave binary frame, rejecting non-WireWave streams on byte 0.
pub async fn read_frame<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Vec<u8>, TransportError> {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic).await?;
    if magic != FRAME_MAGIC {
        return Err(TransportError::InvalidProtocolMagic);
    }
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len > MAX_FRAME_BYTES {
        return Err(TransportError::FrameTooLarge);
    }
    let mut body = vec![0u8; len];
    reader.read_exact(&mut body).await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transport_priority_ranks_correctly() {
        assert!(TransportPriority::WifiFirst.outranks(TransportKind::WiFi, TransportKind::Bluetooth));
        assert!(!TransportPriority::WifiFirst.outranks(TransportKind::Bluetooth, TransportKind::WiFi));
        assert!(TransportPriority::BluetoothFirst.outranks(TransportKind::Bluetooth, TransportKind::WiFi));
    }
}
