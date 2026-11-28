pub mod udp_channel;
pub mod udp_cipher;
pub mod health;
mod health_log;

pub use udp_channel::{UDP_HELLO_PAYLOAD, UdpDataChannel, UdpEndpointHandle};
pub use udp_cipher::{ReplayWindow, UDP_MAGIC, UdpCipher, derive_udp_key};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Identifies each feature stream multiplexed over the encrypted UDP data plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChannelId {
    Speaker = 1,
    Mic = 2,
    Mouse = 3,
    Keyboard = 4,
}

impl ChannelId {
    pub fn from_byte(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(Self::Speaker),
            2 => Some(Self::Mic),
            3 => Some(Self::Mouse),
            4 => Some(Self::Keyboard),
            _ => None,
        }
    }
}

/// Control-plane messages exchanged over Stage 2's encrypted TCP channel
/// to negotiate and synchronize real-time UDP feature streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FeatureControlMessage {
    FeatureStart {
        channel: ChannelId,
        udp_port: u16,
        stream_id: u32,
    },
    FeatureStop {
        channel: ChannelId,
    },
    FeatureStateSync {
        channel: ChannelId,
        active: bool,
        detail: Option<String>,
    },
}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("UDP socket I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("datagram exceeds safe Wi-Fi MTU")]
    DatagramTooLarge,
    #[error("invalid WireWave UDP datagram header")]
    InvalidHeader,
    #[error("stream ID mismatch")]
    StreamMismatch,
    #[error("replayed or stale UDP sequence")]
    ReplayedSequence,
    #[error("UDP encryption failed")]
    EncryptFailed,
    #[error("UDP authentication or decryption failed")]
    DecryptFailed,
    #[error("authenticated UDP peer has not been established")]
    PeerNotEstablished,
}
