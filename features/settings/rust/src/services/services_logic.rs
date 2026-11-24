use wirewave_core_session::{ChannelId, ConnectionPreferences};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicePermissionItem {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub enabled: bool,
    pub channel: ChannelId,
}

#[derive(Debug, Clone)]
pub struct ServicesSettingsState {
    pub speaker_allowed: bool,
    pub mic_allowed: bool,
    pub mouse_allowed: bool,
    pub items: Vec<ServicePermissionItem>,
}

impl ServicesSettingsState {
    pub fn from_prefs(prefs: ConnectionPreferences) -> Self {
        let items = vec![
            ServicePermissionItem {
                id: "speaker-permission-toggle",
                title: "Speaker",
                subtitle: "Allow the connected phone to start and stop desktop audio streaming",
                enabled: prefs.speaker_service_allowed,
                channel: ChannelId::Speaker,
            },
            ServicePermissionItem {
                id: "mic-permission-toggle",
                title: "Mic",
                subtitle: "Allow the connected phone to start and stop microphone streaming",
                enabled: prefs.mic_service_allowed,
                channel: ChannelId::Mic,
            },
            ServicePermissionItem {
                id: "mouse-permission-toggle",
                title: "Mouse",
                subtitle: "Allow the connected phone to control mouse cursor and gestures",
                enabled: prefs.mouse_service_allowed,
                channel: ChannelId::Mouse,
            },
        ];

        Self {
            speaker_allowed: prefs.speaker_service_allowed,
            mic_allowed: prefs.mic_service_allowed,
            mouse_allowed: prefs.mouse_service_allowed,
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflects_default_enabled_speaker_permission() {
        let state = ServicesSettingsState::from_prefs(ConnectionPreferences::default());

        assert!(state.speaker_allowed);
        assert!(state.mic_allowed);
        assert!(state.mouse_allowed);
        assert_eq!(state.items.len(), 3);
        assert!(state.items[0].enabled);
        assert!(state.items[1].enabled);
        assert!(state.items[2].enabled);
    }
}
