//! Session state shared by connection orchestration and UI consumers.

use crate::{settings_store::ConnectionPreferences, trust_store::TrustedDevice};
use std::time::Instant;
use wirewave_core_transport::TransportKind;

#[derive(Debug, Clone)]
pub struct PendingPairing {
    pub device_id: String,
    pub device_name: String,
    pub transport: TransportKind,
    pub otp_code: String,
    pub expires_at: Instant,
    pub ttl_secs: u64,
}

impl PendingPairing {
    pub fn remaining_secs(&self) -> u64 {
        self.expires_at
            .saturating_duration_since(Instant::now())
            .as_secs()
    }

    pub fn formatted_code(&self) -> String {
        match self.otp_code.len() == 6 {
            true => format!("{}  {}", &self.otp_code[..3], &self.otp_code[3..]),
            false => self.otp_code.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub device_id: String,
    pub device_name: String,
    pub transport: TransportKind,
}

impl ActiveSession {
    pub fn short_id(&self) -> String {
        let raw = self.device_id.trim_start_matches("wwd_");
        match raw.len() >= 8 {
            true => format!("{}-{}", &raw[..4], &raw[4..8]),
            false => raw.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SessionState {
    Idle,
    Pairing(PendingPairing),
    Connected(ActiveSession),
}

#[derive(Debug, Clone)]
pub struct SessionSnapshot {
    pub pc_name: String,
    pub state: SessionState,
    pub preferences: ConnectionPreferences,
    pub trusted_devices: Vec<TrustedDevice>,
    pub blocked_devices: Vec<TrustedDevice>,
    pub wifi_listening: bool,
    pub bluetooth_listening: bool,
}
