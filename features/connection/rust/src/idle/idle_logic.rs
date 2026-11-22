use wirewave_core_session::SessionSnapshot;

#[derive(Debug, Clone)]
pub struct IdleViewState {
    pub pc_name: String,
    pub wifi_ready: bool,
    pub bluetooth_ready: bool,
}

impl IdleViewState {
    pub fn from_snapshot(snapshot: &SessionSnapshot) -> Self {
        Self {
            pc_name: snapshot.pc_name.clone(),
            wifi_ready: snapshot.wifi_listening,
            bluetooth_ready: snapshot.bluetooth_listening,
        }
    }
}
