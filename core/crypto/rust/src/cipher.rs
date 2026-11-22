use crate::CryptoError;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use sha2::{Digest, Sha256};
use wirewave_core_config::APP_CONFIG;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Derives a 32-byte ChaCha20-Poly1305 session key bound to the WireWave protocol domain.
pub fn derive_session_key(shared_secret: &[u8; 32], transcript: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(APP_CONFIG.connection.protocol_domain.as_bytes());
    hasher.update(b":session-key:");
    hasher.update(shared_secret);
    hasher.update(transcript);
    let digest = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&digest);
    key
}

/// Authenticated ChaCha20-Poly1305 frame cipher with monotonic nonce counters.
/// Automatically zeroizes key material when the session drops.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SessionCipher {
    send_key: [u8; 32],
    recv_key: [u8; 32],
    send_counter: u64,
    recv_counter: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherRole {
    Desktop,
    Android,
}

impl SessionCipher {
    pub fn new(key: [u8; 32], role: CipherRole) -> Self {
        let desktop_to_android = derive_directional_key(&key, b"desktop-to-android");
        let android_to_desktop = derive_directional_key(&key, b"android-to-desktop");
        let (send_key, recv_key) = match role {
            CipherRole::Desktop => (desktop_to_android, android_to_desktop),
            CipherRole::Android => (android_to_desktop, desktop_to_android),
        };
        Self {
            send_key,
            recv_key,
            send_counter: 0,
            recv_counter: 0,
        }
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let counter = self.send_counter;
        self.send_counter = self
            .send_counter
            .checked_add(1)
            .ok_or(CryptoError::NonceExhausted)?;
        let nonce_bytes = build_nonce(counter);
        let cipher = ChaCha20Poly1305::new((&self.send_key).into());
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), plaintext)
            .map_err(|_| CryptoError::EncryptFailed)?;

        let mut frame = Vec::with_capacity(8 + ciphertext.len());
        frame.extend_from_slice(&counter.to_be_bytes());
        frame.extend_from_slice(&ciphertext);
        Ok(frame)
    }

    pub fn decrypt(&mut self, frame: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if frame.len() < 8 {
            return Err(CryptoError::DecryptFailed);
        }
        let mut ctr_bytes = [0u8; 8];
        ctr_bytes.copy_from_slice(&frame[..8]);
        let counter = u64::from_be_bytes(ctr_bytes);
        if counter < self.recv_counter {
            return Err(CryptoError::DecryptFailed);
        }
        let nonce_bytes = build_nonce(counter);
        let cipher = ChaCha20Poly1305::new((&self.recv_key).into());
        let plaintext = cipher
            .decrypt(Nonce::from_slice(&nonce_bytes), &frame[8..])
            .map_err(|_| CryptoError::DecryptFailed)?;
        self.recv_counter = counter.saturating_add(1);
        Ok(plaintext)
    }
}

fn derive_directional_key(session_key: &[u8; 32], direction: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(APP_CONFIG.connection.protocol_domain.as_bytes());
    hasher.update(b":directional-key:");
    hasher.update(session_key);
    hasher.update(b":");
    hasher.update(direction);
    hasher.finalize().into()
}

fn build_nonce(counter: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[4..12].copy_from_slice(&counter.to_be_bytes());
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_cipher_roundtrips_and_blocks_replay() {
        let key = derive_session_key(&[9u8; 32], b"t1");
        let mut tx = SessionCipher::new(key, CipherRole::Desktop);
        let mut rx = SessionCipher::new(key, CipherRole::Android);

        let encrypted = tx.encrypt(b"hello wirewave").expect("encrypt");
        assert_eq!(rx.decrypt(&encrypted).expect("decrypt"), b"hello wirewave");
        assert!(rx.decrypt(&encrypted).is_err(), "replay must be rejected");
    }

}
