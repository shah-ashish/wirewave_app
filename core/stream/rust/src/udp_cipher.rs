use crate::{ChannelId, StreamError};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Nonce,
};
use sha2::{Digest, Sha256};
use wirewave_core_config::APP_CONFIG;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const UDP_MAGIC: [u8; 4] = *b"WW2U";
pub const UDP_HEADER_BYTES: usize = 4 + 1 + 4 + 8; // magic + channel + stream_id + seq
pub const MAX_UDP_DATAGRAM_BYTES: usize = 1400; // Safe Wi-Fi MTU (prevents IP fragmentation)

/// Derives an isolated 32-byte UDP stream key from the Stage 2 session key.
pub fn derive_udp_key(session_key: &[u8; 32], stream_id: u32) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(APP_CONFIG.connection.protocol_domain.as_bytes());
    hasher.update(b":udp-stream:");
    hasher.update(session_key);
    hasher.update(stream_id.to_be_bytes());
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

/// 64-packet sliding anti-replay window tolerating minor out-of-order UDP delivery.
#[derive(Debug, Default)]
pub struct ReplayWindow {
    highest_seq: u64,
    bitmap: u64,
    initialized: bool,
}

impl ReplayWindow {
    pub fn check_and_record(&mut self, seq: u64) -> bool {
        if !self.initialized {
            self.initialized = true;
            self.highest_seq = seq;
            self.bitmap = 1;
            return true;
        }

        match seq.cmp(&self.highest_seq) {
            std::cmp::Ordering::Greater => {
                let shift = seq - self.highest_seq;
                self.bitmap = match shift >= 64 {
                    true => 1,
                    false => (self.bitmap << shift) | 1,
                };
                self.highest_seq = seq;
                true
            }
            std::cmp::Ordering::Equal => false,
            std::cmp::Ordering::Less => {
                let delta = self.highest_seq - seq;
                if delta >= 64 {
                    return false;
                }
                let mask = 1u64 << delta;
                if (self.bitmap & mask) != 0 {
                    return false;
                }
                self.bitmap |= mask;
                true
            }
        }
    }
}

/// Authenticated per-datagram ChaCha20-Poly1305 cipher for encrypted UDP streams.
/// Automatically zeroizes key material on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct UdpCipher {
    key: [u8; 32],
    stream_id: u32,
    send_seq: u64,
    #[zeroize(skip)]
    replay: ReplayWindow,
}

impl UdpCipher {
    pub fn new(key: [u8; 32], stream_id: u32) -> Self {
        Self {
            key,
            stream_id,
            send_seq: 1,
            replay: ReplayWindow::default(),
        }
    }

    pub fn encrypt_datagram(
        &mut self,
        channel: ChannelId,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, StreamError> {
        let seq = self.send_seq;
        self.send_seq = self.send_seq.wrapping_add(1);

        let mut header = [0u8; UDP_HEADER_BYTES];
        header[0..4].copy_from_slice(&UDP_MAGIC);
        header[4] = channel as u8;
        header[5..9].copy_from_slice(&self.stream_id.to_be_bytes());
        header[9..17].copy_from_slice(&seq.to_be_bytes());

        let nonce = build_udp_nonce(channel as u8, seq);
        let cipher = ChaCha20Poly1305::new((&self.key).into());
        let ciphertext = cipher
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: plaintext,
                    aad: &header,
                },
            )
            .map_err(|_| StreamError::EncryptFailed)?;

        let total = UDP_HEADER_BYTES + ciphertext.len();
        if total > MAX_UDP_DATAGRAM_BYTES {
            return Err(StreamError::DatagramTooLarge);
        }

        let mut datagram = Vec::with_capacity(total);
        datagram.extend_from_slice(&header);
        datagram.extend_from_slice(&ciphertext);
        Ok(datagram)
    }

    pub fn decrypt_datagram(&mut self, datagram: &[u8]) -> Result<(ChannelId, u64, Vec<u8>), StreamError> {
        if datagram.len() < UDP_HEADER_BYTES + 16 || datagram[0..4] != UDP_MAGIC {
            return Err(StreamError::InvalidHeader);
        }

        let channel = ChannelId::from_byte(datagram[4]).ok_or(StreamError::InvalidHeader)?;
        let mut sid_bytes = [0u8; 4];
        sid_bytes.copy_from_slice(&datagram[5..9]);
        let stream_id = u32::from_be_bytes(sid_bytes);
        if stream_id != self.stream_id {
            return Err(StreamError::StreamMismatch);
        }

        let mut seq_bytes = [0u8; 8];
        seq_bytes.copy_from_slice(&datagram[9..17]);
        let seq = u64::from_be_bytes(seq_bytes);

        let header = &datagram[0..UDP_HEADER_BYTES];
        let nonce = build_udp_nonce(channel as u8, seq);
        let cipher = ChaCha20Poly1305::new((&self.key).into());
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: &datagram[UDP_HEADER_BYTES..],
                    aad: header,
                },
            )
            .map_err(|_| StreamError::DecryptFailed)?;

        if !self.replay.check_and_record(seq) {
            return Err(StreamError::ReplayedSequence);
        }

        Ok((channel, seq, plaintext))
    }
}

fn build_udp_nonce(channel: u8, seq: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[3] = channel;
    nonce[4..12].copy_from_slice(&seq.to_be_bytes());
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn udp_cipher_handles_reordering_and_rejects_replay() {
        let key = derive_udp_key(&[5u8; 32], 42);
        let mut tx = UdpCipher::new(key, 42);
        let mut rx = UdpCipher::new(key, 42);

        let d1 = tx.encrypt_datagram(ChannelId::Speaker, b"pkt1").expect("enc 1");
        let d2 = tx.encrypt_datagram(ChannelId::Speaker, b"pkt2").expect("enc 2");

        // Out-of-order arrival (d2 before d1) must succeed
        let (_, seq2, p2) = rx.decrypt_datagram(&d2).expect("dec 2");
        let (_, seq1, p1) = rx.decrypt_datagram(&d1).expect("dec 1");
        assert_eq!((seq1, p1), (1, b"pkt1".to_vec()));
        assert_eq!((seq2, p2), (2, b"pkt2".to_vec()));

        // Replayed d1 must fail
        assert!(matches!(
            rx.decrypt_datagram(&d1),
            Err(StreamError::ReplayedSequence)
        ));
    }
}
