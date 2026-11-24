use gpui::{div, prelude::*, px, rgba, svg, FontWeight, IntoElement};
use wirewave_core_config::{APP_CONFIG, THEME};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseTileState {
    pub active: bool,
    pub subtitle: &'static str,
}

impl MouseTileState {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            subtitle: if active { "Active" } else { "Inactive" },
        }
    }
}

pub fn render_mouse_tile(state: &MouseTileState) -> impl IntoElement {
    let colors = THEME.colors;
    let metrics = THEME.metrics;
    let (background, border, accent) = if state.active {
        (colors.bg_nav_active, colors.accent_blue, colors.accent_blue)
    } else {
        (colors.bg_card, colors.border_subtle, colors.text_secondary)
    };

    let labels = div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(
            div()
                .text_size(px(THEME.typography.font_body))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(colors.text_primary))
                .child("Mouse"),
        )
        .child(
            div()
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(accent))
                .child(state.subtitle),
        );

    div()
        .id("mouse-service-tile")
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
                .path(APP_CONFIG.assets.icon_mouse_svg)
                .size(px(18.0))
                .text_color(rgba(accent)),
        )
        .child(labels)
}
