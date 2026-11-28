use crate::ui::mic_logic::MicTileState;
use gpui::prelude::*;
use wirewave_core_config::APP_CONFIG;
use wirewave_core_ui::FeatureStatusTile;

/// Read-only desktop mic status. Activation is controlled by the phone.
pub fn render_mic_tile(state: &MicTileState, latency: Option<&str>) -> impl IntoElement {
    FeatureStatusTile {
        id: "mic-service-tile",
        title: "Mic",
        latency,
        icon_path: APP_CONFIG.assets.icon_mic_svg,
        active: state.active,
        subtitle: &state.subtitle,
        detail: state.driver_name.as_deref(),
    }
    .render()
}
