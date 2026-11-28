use wirewave_core_session::{SessionSnapshot, SessionState};
use wirewave_core_transport::TransportKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    Connected(TransportKind),
    Trusted,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct UnifiedDeviceRow {
    pub device_id: String,
    pub name: String,
    pub status: DeviceStatus,
}

/// Builds a single unified device list ordered Connected -> Trusted -> Blocked, without duplicates.
pub fn unified_device_rows(snapshot: &SessionSnapshot) -> Vec<UnifiedDeviceRow> {
    let mut rows = Vec::new();
    let active_id = match &snapshot.state {
        SessionState::Connected(active) => {
            rows.push(UnifiedDeviceRow {
                device_id: active.device_id.clone(),
                name: active.device_name.clone(),
                status: DeviceStatus::Connected(active.transport),
            });
            Some(active.device_id.as_str())
        }
        SessionState::Idle | SessionState::Pairing(_) => None,
    };

    for device in &snapshot.trusted_devices {
        if active_id == Some(device.device_id.as_str()) {
            continue;
        }
        rows.push(UnifiedDeviceRow {
            device_id: device.device_id.clone(),
            name: device.device_name.clone(),
            status: DeviceStatus::Trusted,
        });
    }

    for device in &snapshot.blocked_devices {
        if active_id == Some(device.device_id.as_str()) {
            continue;
        }
        rows.push(UnifiedDeviceRow {
            device_id: device.device_id.clone(),
            name: device.device_name.clone(),
            status: DeviceStatus::Blocked,
        });
    }

    rows
}

/// Formats a concise summary label for the DEVICES section header.
pub fn device_summary_label(snapshot: &SessionSnapshot) -> String {
    let connected_count = match &snapshot.state {
        SessionState::Connected(_) => 1usize,
        SessionState::Idle | SessionState::Pairing(_) => 0usize,
    };
    let trusted_count = snapshot.trusted_devices.len();
    let blocked_count = snapshot.blocked_devices.len();

    match (connected_count, trusted_count, blocked_count) {
        (0, 0, 0) => String::new(),
        (1, 0 | 1, 0) => "1 connected".to_string(),
        (1, t, 0) => format!("1 connected · {t} trusted"),
        (0, t, 0) => format!("{t} trusted"),
        (0, 0, b) => format!("{b} blocked"),
        (c, t, b) => format!("{c} connected · {t} trusted · {b} blocked"),
    }
}
