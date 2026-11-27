use gpui::IntoElement;
use std::sync::Arc;
use wirewave_core_session::{FeatureRegistry, SharedFeatureRegistry};
use wirewave_feature_connection::ActiveServiceSlots;
use wirewave_feature_mic::{MicService, MicTileState, render_mic_tile};
use wirewave_feature_mouse::{MouseService, MouseTileState, render_mouse_tile};
use wirewave_feature_speaker::{SpeakerService, SpeakerTileState, render_speaker_tile};

/// Injected implementation of active service slots for Desktop Home, composed at the application root.
pub(crate) struct DesktopFeatures {
    speaker: Arc<SpeakerService>,
    mic: Arc<MicService>,
    mouse: Arc<MouseService>,
}

impl DesktopFeatures {
    pub(crate) fn new() -> Self {
        Self {
            speaker: Arc::new(SpeakerService::new()),
            mic: Arc::new(MicService::new()),
            mouse: Arc::new(MouseService::new()),
        }
    }

    /// Registers the same service instances that back the desktop tiles.
    pub(crate) fn registry(&self) -> SharedFeatureRegistry {
        let registry = Arc::new(FeatureRegistry::new());
        registry.register(self.speaker.clone());
        registry.register(self.mic.clone());
        registry.register(self.mouse.clone());
        registry
    }
}

impl ActiveServiceSlots for DesktopFeatures {
    fn render_speaker_tile(&self) -> gpui::AnyElement {
        let state = SpeakerTileState::from_status(&self.speaker.status());
        render_speaker_tile(&state).into_any_element()
    }

    fn render_mic_tile(&self) -> gpui::AnyElement {
        let state = MicTileState::from_status(&self.mic.status());
        render_mic_tile(&state).into_any_element()
    }

    fn render_mouse_tile(&self) -> gpui::AnyElement {
        let is_active = self.mouse.is_running();
        let state = MouseTileState::new(is_active);
        render_mouse_tile(&state).into_any_element()
    }

    fn stop_all(&self) {
        self.speaker.stop();
        self.mic.stop();
        self.mouse.stop();
    }

    fn is_any_running(&self) -> bool {
        self.speaker.is_running() || self.mic.is_running() || self.mouse.is_running()
    }
}
