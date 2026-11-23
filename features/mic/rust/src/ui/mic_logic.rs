use crate::audio::render_loop::MicStatus;

#[derive(Debug, Clone)]
pub struct MicTileState {
    pub active: bool,
    pub subtitle: String,
    pub driver_name: Option<String>,
}

impl MicTileState {
    pub fn from_status(status: &MicStatus) -> Self {
        match status {
            MicStatus::Off => Self {
                active: false,
                subtitle: "Inactive".into(),
                driver_name: None,
            },
            MicStatus::Active { driver_name } => Self {
                active: true,
                subtitle: "Active".into(),
                driver_name: Some(driver_name.clone()),
            },
        }
    }
}
