use crate::service::CameraStatus;

#[derive(Debug, Clone)]
pub struct CameraTileState {
    pub active: bool,
    pub subtitle: String,
    pub driver_name: Option<String>,
}

impl CameraTileState {
    pub fn from_status(status: &CameraStatus) -> Self {
        match status {
            CameraStatus::Off => Self {
                active: false,
                subtitle: "Inactive".into(),
                driver_name: None,
            },
            CameraStatus::Active { driver_name } => Self {
                active: true,
                subtitle: "Active".into(),
                driver_name: Some(driver_name.clone()),
            },
        }
    }
}
