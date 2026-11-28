//! Per-stream UDP round-trip measurements. No wall-clock synchronization is required.
use std::{
    collections::VecDeque,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

pub const PROBE_PREFIX: &[u8] = b"WW2U_PING";
pub const REPLY_PREFIX: &[u8] = b"WW2U_PONG";
pub const DOWN_AFTER: Duration = Duration::from_secs(3);
pub const SLOW_AFTER: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveLatency {
    Pending,
    Millis(u64),
    Slow,
    Down,
}

impl LiveLatency {
    pub fn label(self) -> String {
        match self {
            Self::Pending => "—".into(),
            Self::Millis(ms) => format!("{ms} ms"),
            Self::Slow => "Slow".into(),
            Self::Down => "Down".into(),
        }
    }
}

pub struct StreamHealth {
    enabled: AtomicBool,
    state: Mutex<ProbeState>,
}

struct ProbeState {
    started: Instant,
    pending: VecDeque<(u64, Instant)>,
    last_reply: Option<Instant>,
    rtt: Option<Duration>,
}

impl StreamHealth {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            state: Mutex::new(ProbeState {
                started: Instant::now(),
                pending: VecDeque::new(),
                last_reply: None,
                rtt: None,
            }),
        }
    }

    pub fn stop(&self) {
        self.enabled.store(false, Ordering::Release);
    }
    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    pub fn disconnected(&self) {
        let mut state = self.state.lock().unwrap();
        state.started = Instant::now() - DOWN_AFTER;
        state.last_reply = None;
        state.pending.clear();
    }

    pub fn probe(&self, sequence: u64, now: Instant) -> Vec<u8> {
        let mut state = self.state.lock().unwrap();
        state
            .pending
            .retain(|(_, sent)| now.saturating_duration_since(*sent) < DOWN_AFTER);
        state.pending.push_back((sequence, now));
        while state.pending.len() > 4 {
            state.pending.pop_front();
        }
        [PROBE_PREFIX, &sequence.to_be_bytes()].concat()
    }

    pub fn reply(&self, payload: &[u8], now: Instant) -> bool {
        let Some(raw) = payload.strip_prefix(REPLY_PREFIX) else {
            return false;
        };
        let Ok(bytes) = <[u8; 8]>::try_from(raw) else {
            return true;
        };
        let seq = u64::from_be_bytes(bytes);
        let mut state = self.state.lock().unwrap();
        if let Some(index) = state
            .pending
            .iter()
            .position(|(pending, _)| *pending == seq)
        {
            if let Some((_, sent)) = state.pending.remove(index) {
                if now.saturating_duration_since(sent) >= DOWN_AFTER {
                    return true;
                }
                state.rtt = Some(now.saturating_duration_since(sent));
                state.last_reply = Some(now);
            }
        }
        true
    }

    pub fn snapshot(&self) -> (LiveLatency, Option<u64>) {
        self.at(Instant::now())
    }

    fn at(&self, now: Instant) -> (LiveLatency, Option<u64>) {
        let state = self.state.lock().unwrap();
        let ms = state.rtt.map(|rtt| (rtt.as_millis() as u64).max(1));
        let age = now.saturating_duration_since(state.last_reply.unwrap_or(state.started));
        let label = if age >= DOWN_AFTER {
            LiveLatency::Down
        } else if age >= Duration::from_secs(2) {
            LiveLatency::Slow
        } else if state.rtt.is_some_and(|rtt| rtt >= SLOW_AFTER) {
            LiveLatency::Slow
        } else {
            ms.map(LiveLatency::Millis).unwrap_or(LiveLatency::Pending)
        };
        (label, ms)
    }
}

impl Default for StreamHealth {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn measures_matching_reply_and_detects_drop_and_recovery() {
        let health = StreamHealth::new();
        let now = Instant::now();
        assert_eq!(health.at(now).0, LiveLatency::Pending);
        health.probe(1, now);
        let reply = [REPLY_PREFIX, &1u64.to_be_bytes()].concat();
        health.reply(&reply, now + Duration::from_millis(3));
        assert_eq!(
            health.at(now + Duration::from_millis(4)).0,
            LiveLatency::Millis(3)
        );
        assert_eq!(health.at(now + Duration::from_secs(4)).0, LiveLatency::Down);
        health.probe(2, now + Duration::from_secs(5));
        health.reply(
            &[REPLY_PREFIX, &2u64.to_be_bytes()].concat(),
            now + Duration::from_millis(5004),
        );
        assert_eq!(
            health.at(now + Duration::from_millis(5005)).0,
            LiveLatency::Millis(4)
        );
    }
    #[test]
    fn ignores_wrong_duplicate_and_malformed_replies() {
        let health = StreamHealth::new();
        let now = Instant::now();
        health.probe(7, now);
        health.reply(&[REPLY_PREFIX, &6u64.to_be_bytes()].concat(), now);
        health.reply(REPLY_PREFIX, now);
        assert_eq!(health.at(now).0, LiveLatency::Pending);
        let reply = [REPLY_PREFIX, &7u64.to_be_bytes()].concat();
        health.reply(&reply, now + SLOW_AFTER);
        health.reply(&reply, now + Duration::from_secs(1));
        assert_eq!(health.at(now + SLOW_AFTER), (LiveLatency::Slow, Some(100)));
        assert!(!health.reply(b"audio", now));
    }

    #[test]
    fn delayed_reply_survives_next_probe_and_disconnect_is_immediate() {
        let health = StreamHealth::new();
        let now = Instant::now();
        health.probe(1, now);
        health.probe(2, now + Duration::from_secs(1));
        health.reply(
            &[REPLY_PREFIX, &1u64.to_be_bytes()].concat(),
            now + Duration::from_millis(1200),
        );
        assert_eq!(
            health.at(now + Duration::from_millis(1201)),
            (LiveLatency::Slow, Some(1200))
        );
        health.disconnected();
        assert_eq!(health.snapshot().0, LiveLatency::Down);
    }
}
