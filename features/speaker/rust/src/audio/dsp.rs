/// Converts interleaved f32 samples of arbitrary channel count into interleaved stereo f32.
pub fn downmix_to_stereo(samples: &[f32], src_channels: usize, out: &mut Vec<f32>) {
    out.clear();
    if src_channels == 0 {
        return;
    }
    let frames = samples.len() / src_channels;
    out.reserve(frames * 2);

    match src_channels {
        1 => {
            for &s in samples {
                out.push(s);
                out.push(s);
            }
        }
        2 => out.extend_from_slice(samples),
        ch => {
            // Mix front L/R + center/surround energy gently into stereo without clipping.
            for frame in samples.chunks_exact(ch) {
                let mut left = frame[0];
                let mut right = frame[1];
                if ch >= 3 {
                    let center = frame[2] * 0.707;
                    left += center;
                    right += center;
                }
                if ch >= 6 {
                    left += frame[4] * 0.5;
                    right += frame[5] * 0.5;
                }
                out.push(left.clamp(-1.0, 1.0));
                out.push(right.clamp(-1.0, 1.0));
            }
        }
    }
}

/// Stateful linear stereo resampler targeting 48 kHz.
pub struct StereoResampler {
    src_rate: u32,
    dst_rate: u32,
    phase: f64,
    prev_l: f32,
    prev_r: f32,
}

impl StereoResampler {
    pub fn new(src_rate: u32, dst_rate: u32) -> Self {
        Self {
            src_rate: src_rate.max(1),
            dst_rate: dst_rate.max(1),
            phase: 0.0,
            prev_l: 0.0,
            prev_r: 0.0,
        }
    }

    pub fn process(&mut self, stereo_in: &[f32], out: &mut Vec<f32>) {
        out.clear();
        if stereo_in.len() < 2 {
            return;
        }
        if self.src_rate == self.dst_rate {
            out.extend_from_slice(stereo_in);
            return;
        }

        let in_frames = stereo_in.len() / 2;
        let step = self.src_rate as f64 / self.dst_rate as f64;
        let est = ((in_frames as f64) / step).ceil() as usize + 2;
        out.reserve(est * 2);

        while self.phase < in_frames as f64 {
            let idx = self.phase.floor() as usize;
            let frac = (self.phase - idx as f64) as f32;

            let (l0, r0) = match idx {
                0 => (self.prev_l, self.prev_r),
                i => (stereo_in[(i - 1) * 2], stereo_in[(i - 1) * 2 + 1]),
            };
            let (l1, r1) = (stereo_in[idx * 2], stereo_in[idx * 2 + 1]);

            out.push(l0 + (l1 - l0) * frac);
            out.push(r0 + (r1 - r0) * frac);
            self.phase += step;
        }

        self.phase -= in_frames as f64;
        self.prev_l = stereo_in[(in_frames - 1) * 2];
        self.prev_r = stereo_in[(in_frames - 1) * 2 + 1];
    }
}

/// Converts stereo f32 samples `[-1.0, 1.0]` to interleaved S16LE bytes.
pub fn f32_to_s16le_bytes(stereo_f32: &[f32], out: &mut Vec<u8>) {
    out.reserve(stereo_f32.len() * 2);
    for &sample in stereo_f32 {
        let scaled = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
        out.extend_from_slice(&scaled.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mono_downmix_duplicates_to_stereo() {
        let mut out = Vec::new();
        downmix_to_stereo(&[0.5, -0.25], 1, &mut out);
        assert_eq!(out, vec![0.5, 0.5, -0.25, -0.25]);
    }

    #[test]
    fn resampler_passthrough_at_48k() {
        let mut r = StereoResampler::new(48_000, 48_000);
        let mut out = Vec::new();
        r.process(&[0.1, -0.1, 0.2, -0.2], &mut out);
        assert_eq!(out, vec![0.1, -0.1, 0.2, -0.2]);
    }
}
