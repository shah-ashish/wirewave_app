use super::mouse_logic::MouseTileState;
use gpui::prelude::*;
use wirewave_core_config::APP_CONFIG;
use wirewave_core_ui::FeatureStatusTile;

/// Read-only desktop mouse status. Activation is controlled by the phone.
pub fn render_mouse_tile(state: &MouseTileState, latency: Option<&str>) -> impl IntoElement {
    FeatureStatusTile {
        id: "mouse-service-tile",
        title: "Mouse",
        latency,
        icon_path: APP_CONFIG.assets.icon_mouse_svg,
        active: state.active,
        subtitle: state.subtitle,
        detail: None,
    }
    .render()
}
