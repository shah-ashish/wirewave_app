use wirewave_core_session::ConnectionPreferences;

#[derive(Debug, Clone, Copy)]
pub struct ServicesSettingsState {
    pub speaker_allowed: bool,
}

impl ServicesSettingsState {
    pub fn from_prefs(prefs: ConnectionPreferences) -> Self {
        Self {
            speaker_allowed: prefs.speaker_service_allowed,
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
    }
}
