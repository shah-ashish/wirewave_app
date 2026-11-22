use wirewave_core_session::{SessionSnapshot, SessionState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceTab {
    Connected,
    Trusted,
    Blocked,
}

impl DeviceTab {
    pub const ALL: [Self; 3] = [Self::Connected, Self::Trusted, Self::Blocked];

    pub fn label(self) -> &'static str {
        match self {
            Self::Connected => "Connected",
            Self::Trusted => "Trusted",
            Self::Blocked => "Blocked",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceRowModel {
    pub device_id: String,
    pub short_id: String,
    pub name: String,
}

pub fn rows_for_tab(snapshot: &SessionSnapshot, tab: DeviceTab) -> Vec<DeviceRowModel> {
    match tab {
        DeviceTab::Connected => match &snapshot.state {
            SessionState::Connected(active) => vec![DeviceRowModel {
                device_id: active.device_id.clone(),
                short_id: active.short_id(),
                name: active.device_name.clone(),
            }],
            SessionState::Idle | SessionState::Pairing(_) => Vec::new(),
        },
        DeviceTab::Trusted => snapshot
            .trusted_devices
            .iter()
            .map(|d| DeviceRowModel {
                device_id: d.device_id.clone(),
                short_id: d.short_id(),
                name: d.device_name.clone(),
            })
            .collect(),
        DeviceTab::Blocked => snapshot
            .blocked_devices
            .iter()
            .map(|d| DeviceRowModel {
                device_id: d.device_id.clone(),
                short_id: d.short_id(),
                name: d.device_name.clone(),
            })
            .collect(),
    }
}
