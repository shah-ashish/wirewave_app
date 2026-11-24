use crate::ui::camera_tile_logic::CameraTileState;
use gpui::{div, prelude::*, px, rgba, svg, FontWeight};
use wirewave_core_config::{APP_CONFIG, THEME};

/// Read-only Desktop Camera status tile. Activation is controlled by the phone.
pub fn render_camera_tile(state: &CameraTileState) -> impl IntoElement {
    let colors = THEME.colors;
    let metrics = THEME.metrics;
    let (background, border, accent) = if state.active {
        (colors.bg_nav_active, colors.accent_blue, colors.accent_blue)
    } else {
        (colors.bg_card, colors.border_subtle, colors.text_secondary)
    };

    let mut labels = div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(
            div()
                .text_size(px(THEME.typography.font_body))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(colors.text_primary))
                .child("Camera"),
        )
        .child(
            div()
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(accent))
                .child(state.subtitle.clone()),
        );
    if let Some(driver_name) = &state.driver_name {
        labels = labels.child(
            div()
                .text_size(px(10.0))
                .text_color(rgba(colors.text_secondary))
                .child(driver_name.clone()),
        );
    }

    div()
        .id("camera-service-tile")
        .flex_1()
        .bg(rgba(background))
        .border_1()
        .border_color(rgba(border))
        .rounded(px(metrics.radius_card - 2.0))
        .px(px(14.0))
        .py(px(14.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.0))
        .child(
            svg()
                .path(APP_CONFIG.assets.icon_camera_svg)
                .size(px(18.0))
                .text_color(rgba(accent)),
        )
        .child(labels)
}
