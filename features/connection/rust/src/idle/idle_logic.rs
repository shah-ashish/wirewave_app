use wirewave_core_session::SessionSnapshot;

#[derive(Debug, Clone)]
pub struct IdleViewState {
    pub pc_name: String,
    pub wifi_ready: bool,
    pub bluetooth_ready: bool,
    pub trusted_count: usize,
    pub speaker_allowed: bool,
    pub mic_allowed: bool,
    pub mouse_allowed: bool,
}

impl IdleViewState {
    pub fn from_snapshot(snapshot: &SessionSnapshot) -> Self {
        Self {
            pc_name: snapshot.pc_name.clone(),
            wifi_ready: snapshot.wifi_listening,
            bluetooth_ready: snapshot.bluetooth_listening,
            trusted_count: snapshot.trusted_devices.len(),
            speaker_allowed: snapshot.preferences.speaker_service_allowed,
            mic_allowed: snapshot.preferences.mic_service_allowed,
            mouse_allowed: snapshot.preferences.mouse_service_allowed,
        }
    }
}
