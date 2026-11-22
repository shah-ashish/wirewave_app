pub mod cipher;
pub mod keys;
pub mod otp;

pub use cipher::{derive_session_key, CipherRole, SessionCipher};
pub use keys::{fetch_or_create_keypair, verify_peer_signature, IdentityKeypair};
pub use otp::{compute_otp_proof, generate_otp, verify_otp_proof};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("unable to resolve local data directory")]
    NoDataDir,
    #[error("key storage I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("key serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid public key encoding")]
    InvalidPublicKey,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("encryption failed")]
    EncryptFailed,
    #[error("decryption or authentication failed")]
    DecryptFailed,
    #[error("nonce counter exhausted")]
    NonceExhausted,
}
