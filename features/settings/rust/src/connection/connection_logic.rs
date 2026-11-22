use wirewave_core_session::ConnectionPreferences;
use wirewave_core_transport::TransportPriority;

#[derive(Debug, Clone, Copy)]
pub struct ConnectionSettingsState {
    pub priority: TransportPriority,
    pub keep_alive_in_background: bool,
    pub dropdown_open: bool,
}

impl ConnectionSettingsState {
    pub fn from_prefs(prefs: ConnectionPreferences, dropdown_open: bool) -> Self {
        Self {
            priority: prefs.priority,
            keep_alive_in_background: prefs.keep_alive_in_background,
            dropdown_open,
        }
    }
}

pub const ALL_PRIORITIES: [TransportPriority; 2] = [
    TransportPriority::WifiFirst,
    TransportPriority::BluetoothFirst,
];
