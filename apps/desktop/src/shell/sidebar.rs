use gpui::{div, prelude::*, px, rgba, svg, Div};
use wirewave_core_config::{APP_CONFIG, THEME};

/// Left navigation rail with active Settings tile and bottom host status indicator.
pub fn render_sidebar(is_online: bool) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    let settings_tile = div()
        .size(px(42.0))
        .rounded(px(m.radius_nav))
        .bg(rgba(c.bg_nav_active))
        .flex()
        .items_center()
        .justify_center()
        .child(
            svg()
                .path(APP_CONFIG.assets.icon_settings_svg)
                .size(px(20.0))
                .text_color(rgba(c.accent_blue)),
        );

    let dot_color = match is_online {
        true => c.status_ok_fg,
        false => c.text_secondary,
    };

    let status_indicator = div()
        .relative()
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .child(
            svg()
                .path(APP_CONFIG.assets.icon_monitor_svg)
                .size(px(20.0))
                .text_color(rgba(c.text_secondary)),
        )
        .child(
            div()
                .absolute()
                .bottom(px(3.0))
                .right(px(2.0))
                .size(px(8.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(dot_color))
                .border_1()
                .border_color(rgba(c.bg_sidebar)),
        );

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_between()
        .w(px(m.sidebar_width))
        .h_full()
        .py(px(16.0))
        .bg(rgba(c.bg_sidebar))
        .border_r_1()
        .border_color(rgba(c.border_subtle))
        .child(settings_tile)
        .child(status_indicator)
}
