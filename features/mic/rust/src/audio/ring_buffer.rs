use std::collections::VecDeque;

pub struct MonoRingBuffer {
    samples: VecDeque<f32>,
    capacity: usize,
    phase: f64,
    previous: f32,
}

impl MonoRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(capacity),
            capacity: capacity.max(256),
            phase: 0.0,
            previous: 0.0,
        }
    }

    pub fn push_s16le(&mut self, bytes: &[u8]) {
        for pair in bytes.chunks_exact(2) {
            if self.samples.len() == self.capacity {
                self.samples.pop_front();
            }
            self.samples
                .push_back(i16::from_le_bytes([pair[0], pair[1]]) as f32 / 32768.0);
        }
    }

    pub fn pop_mono(&mut self, src_rate: u32, dst_rate: u32) -> f32 {
        if src_rate == dst_rate {
            return self.samples.pop_front().unwrap_or(0.0);
        }
        while self.phase >= 1.0 {
            self.previous = self.samples.pop_front().unwrap_or(0.0);
            self.phase -= 1.0;
        }
        let next = self.samples.front().copied().unwrap_or(0.0);
        let result = self.previous + (next - self.previous) * self.phase as f32;
        self.phase += src_rate.max(1) as f64 / dst_rate.max(1) as f64;
        result
    }

    pub fn clear(&mut self) {
        self.samples.clear();
        self.phase = 0.0;
        self.previous = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_old_audio_when_latency_capacity_is_reached() {
        let mut buffer = MonoRingBuffer::new(256);
        for sample in 0..300i16 {
            buffer.push_s16le(&sample.to_le_bytes());
        }
        assert_eq!(buffer.pop_mono(48_000, 48_000), 44.0 / 32768.0);
    }
}
