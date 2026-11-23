use crate::audio::capture_loop::SpeakerStatus;

#[derive(Debug, Clone)]
pub struct SpeakerTileState {
    pub is_on: bool,
    pub subtitle: String,
    pub endpoint_hint: Option<String>,
}

impl SpeakerTileState {
    pub fn from_status(status: &SpeakerStatus) -> Self {
        match status {
            SpeakerStatus::Off => Self {
                is_on: false,
                subtitle: "Off".to_string(),
                endpoint_hint: None,
            },
            SpeakerStatus::Streaming { endpoint } => Self {
                is_on: true,
                subtitle: "On".to_string(),
                endpoint_hint: Some(format!("{} · {}", endpoint.kind.label(), endpoint.name)),
            },
        }
    }
}
