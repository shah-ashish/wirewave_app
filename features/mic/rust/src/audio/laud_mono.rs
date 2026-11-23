pub const LAUD_MAGIC: [u8; 4] = *b"LAUD";
pub const LAUD_VERSION: u8 = 1;
pub const LAUD_HEADER_BYTES: usize = 20;
pub const TARGET_SAMPLE_RATE: u32 = 48_000;
pub const MONO_CHANNELS: u8 = 1;
pub const SAMPLE_FORMAT_S16LE_48K: u8 = 0x01;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaudMonoHeader {
    pub sequence: u32,
    pub timestamp_frames: u64,
}

impl LaudMonoHeader {
    pub fn decode(raw: &[u8]) -> Option<(Self, &[u8])> {
        if raw.len() < LAUD_HEADER_BYTES
            || raw[0..4] != LAUD_MAGIC
            || raw[4] != LAUD_VERSION
            || raw[6] != MONO_CHANNELS
            || raw[7] != SAMPLE_FORMAT_S16LE_48K
        {
            return None;
        }
        let pcm = &raw[LAUD_HEADER_BYTES..];
        if pcm.len() % 2 != 0 {
            return None;
        }
        Some((
            Self {
                sequence: u32::from_be_bytes(raw[8..12].try_into().ok()?),
                timestamp_frames: u64::from_be_bytes(raw[12..20].try_into().ok()?),
            },
            pcm,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_stereo_and_odd_sized_pcm() {
        let mut packet = Vec::from(LAUD_MAGIC);
        packet.extend_from_slice(&[LAUD_VERSION, 0, MONO_CHANNELS, SAMPLE_FORMAT_S16LE_48K]);
        packet.extend_from_slice(&1u32.to_be_bytes());
        packet.extend_from_slice(&0u64.to_be_bytes());
        packet.extend_from_slice(&[0, 0]);
        assert!(LaudMonoHeader::decode(&packet).is_some());
        packet[6] = 2;
        assert!(LaudMonoHeader::decode(&packet).is_none());
        packet[6] = 1;
        packet.push(0);
        assert!(LaudMonoHeader::decode(&packet).is_none());
    }
}
