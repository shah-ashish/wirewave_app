use crate::ui::speaker_logic::SpeakerTileState;
use gpui::prelude::*;
use wirewave_core_config::APP_CONFIG;
use wirewave_core_ui::FeatureStatusTile;

/// Read-only desktop speaker status. Activation is controlled by the phone.
pub fn render_speaker_tile(state: &SpeakerTileState, latency: Option<&str>) -> impl IntoElement {
    FeatureStatusTile {
        id: "speaker-service-tile",
        title: "Speaker",
        latency,
        icon_path: APP_CONFIG.assets.icon_speaker_svg,
        active: state.is_on,
        subtitle: &state.subtitle,
        detail: state.endpoint_hint.as_deref(),
    }
    .render()
}
