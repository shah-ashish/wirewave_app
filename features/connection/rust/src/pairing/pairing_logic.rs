use wirewave_core_session::PendingPairing;
use wirewave_core_transport::TransportKind;

#[derive(Debug, Clone)]
pub struct PairingModalState {
    pub device_name: String,
    pub transport: TransportKind,
    pub formatted_otp: String,
    pub remaining_secs: u64,
    pub progress_fraction: f32,
}

impl PairingModalState {
    pub fn from_pending(pending: &PendingPairing) -> Self {
        let remaining = pending.remaining_secs();
        let fraction = match pending.ttl_secs > 0 {
            true => (remaining as f32 / pending.ttl_secs as f32).clamp(0.0, 1.0),
            false => 0.0,
        };
        Self {
            device_name: pending.device_name.clone(),
            transport: pending.transport,
            formatted_otp: pending.formatted_code(),
            remaining_secs: remaining,
            progress_fraction: fraction,
        }
    }
}
