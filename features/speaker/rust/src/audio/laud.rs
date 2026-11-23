pub const LAUD_MAGIC: [u8; 4] = *b"LAUD";
pub const LAUD_VERSION: u8 = 1;
pub const LAUD_HEADER_BYTES: usize = 20;
pub const TARGET_SAMPLE_RATE: u32 = 48_000;
pub const TARGET_CHANNELS: u8 = 2;
pub const SAMPLE_FORMAT_S16LE_48K: u8 = 0x01;

/// 256 stereo S16LE frames = 1,024 bytes PCM (~5.33 ms).
/// Keeps total encrypted UDP datagram (~1,077 B) safely below 1,400 B Wi-Fi MTU.
pub const FRAMES_PER_PACKET: usize = 256;
pub const PCM_BYTES_PER_PACKET: usize = FRAMES_PER_PACKET * (TARGET_CHANNELS as usize) * 2;

pub const FLAG_SILENCE: u8 = 0x01;
pub const FLAG_DISCONTINUITY: u8 = 0x02;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaudHeader {
    pub flags: u8,
    pub channels: u8,
    pub sample_format: u8,
    pub sequence: u32,
    pub timestamp_frames: u64,
}

impl LaudHeader {
    pub fn encode_into(&self, pcm_bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(LAUD_HEADER_BYTES + pcm_bytes.len());
        out.extend_from_slice(&LAUD_MAGIC);
        out.push(LAUD_VERSION);
        out.push(self.flags);
        out.push(self.channels);
        out.push(self.sample_format);
        out.extend_from_slice(&self.sequence.to_be_bytes());
        out.extend_from_slice(&self.timestamp_frames.to_be_bytes());
        out.extend_from_slice(pcm_bytes);
        out
    }

    pub fn decode(raw: &[u8]) -> Option<(Self, &[u8])> {
        if raw.len() < LAUD_HEADER_BYTES || raw[0..4] != LAUD_MAGIC || raw[4] != LAUD_VERSION {
            return None;
        }
        let mut seq_b = [0u8; 4];
        seq_b.copy_from_slice(&raw[8..12]);
        let mut ts_b = [0u8; 8];
        ts_b.copy_from_slice(&raw[12..20]);

        Some((
            Self {
                flags: raw[5],
                channels: raw[6],
                sample_format: raw[7],
                sequence: u32::from_be_bytes(seq_b),
                timestamp_frames: u64::from_be_bytes(ts_b),
            },
            &raw[LAUD_HEADER_BYTES..],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laud_packet_roundtrips_and_fits_wifi_mtu() {
        let hdr = LaudHeader {
            flags: 0,
            channels: TARGET_CHANNELS,
            sample_format: SAMPLE_FORMAT_S16LE_48K,
            sequence: 19,
            timestamp_frames: 4864,
        };
        let pcm = vec![0u8; PCM_BYTES_PER_PACKET];
        let encoded = hdr.encode_into(&pcm);
        assert!(encoded.len() + 33 < 1400, "must fit under 1400 B MTU");

        let (decoded, payload) = LaudHeader::decode(&encoded).expect("decode");
        assert_eq!(decoded, hdr);
        assert_eq!(payload.len(), PCM_BYTES_PER_PACKET);
    }
}
