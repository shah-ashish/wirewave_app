use crate::idle::idle_logic::IdleViewState;
use gpui::{div, prelude::*, px, rgba, svg, Div, FontWeight};
use wirewave_core_config::{APP_CONFIG, THEME};

pub fn render_idle_view(state: &IdleViewState) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    let monitor_badge = div()
        .size(px(56.0))
        .rounded(px(16.0))
        .bg(rgba(c.bg_card_hover))
        .flex()
        .items_center()
        .justify_center()
        .child(
            svg()
                .path(APP_CONFIG.assets.icon_monitor_svg)
                .size(px(26.0))
                .text_color(rgba(c.text_secondary)),
        );

    let card = div()
        .w(px(460.0))
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card + 4.0))
        .py(px(36.0))
        .px(px(28.0))
        .flex()
        .flex_col()
        .items_center()
        .gap(px(14.0))
        .child(monitor_badge)
        .child(
            div()
                .text_size(px(24.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgba(c.text_primary))
                .child("Ready to connect"),
        )
        .child(
            div()
                .text_size(px(THEME.typography.font_body))
                .text_color(rgba(c.text_secondary))
                .child(state.pc_name.clone()),
        )
        .child(
            div()
                .pt(px(8.0))
                .text_size(px(11.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.text_section))
                .child("LISTENING ON"),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(10.0))
                .child(listener_pill(
                    APP_CONFIG.assets.icon_wifi_svg,
                    "Wi-Fi",
                    state.wifi_ready,
                ))
                .child(listener_pill(
                    APP_CONFIG.assets.icon_bluetooth_svg,
                    "Bluetooth",
                    state.bluetooth_ready,
                )),
        );

    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(20.0))
        .child(card)
        .child(
            div()
                .text_size(px(THEME.typography.font_body))
                .text_color(rgba(c.text_secondary))
                .child(format!(
                    "Open {} on your phone to connect.",
                    APP_CONFIG.app_name
                )),
        )
}

fn listener_pill(icon: &'static str, label: &'static str, active: bool) -> Div {
    let c = THEME.colors;
    let dot_color = match active {
        true => c.status_ok_fg,
        false => c.text_secondary,
    };

    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .px(px(14.0))
        .py(px(7.0))
        .rounded(px(THEME.metrics.radius_pill))
        .bg(rgba(c.bg_card_hover))
        .child(
            div()
                .size(px(7.0))
                .rounded(px(THEME.metrics.radius_pill))
                .bg(rgba(dot_color)),
        )
        .child(svg().path(icon).size(px(14.0)).text_color(rgba(c.text_secondary)))
        .child(
            div()
                .text_size(px(THEME.typography.font_badge))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.text_primary))
                .child(label),
        )
}
