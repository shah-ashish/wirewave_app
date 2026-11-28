use crate::idle::idle_logic::IdleViewState;
use gpui::{div, prelude::*, px, rgba, svg, Div, FontWeight, SharedString};
use wirewave_core_config::{APP_CONFIG, THEME};

/// Renders the responsive Desktop Idle Home view with host readiness, guide, and trust summary.
pub fn render_idle_view(state: &IdleViewState) -> impl IntoElement {
    let m = THEME.metrics;
    let content = div()
        .w_full()
        .max_w(px(m.content_max_width + 260.0))
        .flex()
        .flex_col()
        .gap(px(m.spacing_md))
        .child(hero_card(state))
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(m.spacing_md))
                .w_full()
                .child(guide_card(state))
                .child(summary_card(state)),
        );

    div()
        .id("idle-scroll-root")
        .size_full()
        .overflow_y_scroll()
        .p(px(m.spacing_lg))
        .flex()
        .flex_col()
        .items_center()
        .child(content)
}

fn hero_card(state: &IdleViewState) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    let monitor_badge = div()
        .size(px(48.0))
        .rounded(px(14.0))
        .bg(rgba(c.bg_card_hover))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .flex()
        .items_center()
        .justify_center()
        .child(svg().path(APP_CONFIG.assets.icon_monitor_svg).size(px(22.0)).text_color(rgba(c.text_primary)));

    let title_block = div()
        .flex_1()
        .min_w(px(0.0))
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(div().text_size(px(26.0)).font_weight(FontWeight::BOLD).text_color(rgba(c.text_primary)).child("Ready to connect"))
        .child(div().text_size(px(THEME.typography.font_body)).text_color(rgba(c.text_secondary)).child(format!("{} · Local network", state.pc_name)));

    let listeners = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(8.0))
        .child(listener_pill(APP_CONFIG.assets.icon_wifi_svg, "Wi-Fi", state.wifi_ready))
        .child(listener_pill(APP_CONFIG.assets.icon_bluetooth_svg, "Bluetooth", state.bluetooth_ready));

    div()
        .w_full()
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card + 2.0))
        .p(px(20.0))
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .justify_between()
        .gap(px(m.spacing_md))
        .child(
            div()
                .flex_1()
                .min_w(px(220.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(14.0))
                .child(monitor_badge)
                .child(title_block),
        )
        .child(listeners)
}

fn guide_card(state: &IdleViewState) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    div()
        .flex_1()
        .min_w(px(280.0))
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card))
        .p(px(20.0))
        .flex()
        .flex_col()
        .gap(px(14.0))
        .child(section_heading("HOW TO CONNECT"))
        .child(step_row("1", format!("Open {} on your phone on the same Wi-Fi or Bluetooth.", APP_CONFIG.app_name)))
        .child(step_row("2", format!("Select {} from Nearby computers.", state.pc_name)))
        .child(step_row("3", "Confirm the 6-digit code. Trusted phones reconnect automatically."))
}

fn summary_card(state: &IdleViewState) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let trusted_label = match state.trusted_count {
        0 => "None yet".to_string(),
        1 => "1 paired".to_string(),
        count => format!("{count} paired"),
    };

    let trusted_row = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .pb(px(12.0))
        .child(div().text_size(px(THEME.typography.font_body)).text_color(rgba(c.text_secondary)).child("Trusted phones"))
        .child(div().text_size(px(THEME.typography.font_body)).font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.text_primary)).child(trusted_label));

    let services_block = div()
        .border_t_1()
        .border_color(rgba(c.divider))
        .pt(px(12.0))
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(div().text_size(px(THEME.typography.font_badge)).text_color(rgba(c.text_secondary)).child("Allowed services"))
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(8.0))
                .child(permission_chip("Speaker", state.speaker_allowed))
                .child(permission_chip("Mic", state.mic_allowed))
                .child(permission_chip("Mouse", state.mouse_allowed)),
        );

    div()
        .flex_1()
        .min_w(px(240.0))
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card))
        .p(px(20.0))
        .flex()
        .flex_col()
        .child(section_heading("THIS PC"))
        .child(div().pt(px(14.0)).child(trusted_row))
        .child(services_block)
}

fn section_heading(title: &'static str) -> Div {
    div()
        .text_size(px(THEME.typography.font_section))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgba(THEME.colors.text_section))
        .child(title)
}

fn step_row(num: &'static str, text: impl Into<SharedString>) -> Div {
    let c = THEME.colors;
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(12.0))
        .child(
            div()
                .size(px(22.0))
                .rounded(px(THEME.metrics.radius_pill))
                .bg(rgba(c.bg_sidebar))
                .border_1()
                .border_color(rgba(c.border_subtle))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(THEME.typography.font_badge - 1.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.text_primary))
                .child(num),
        )
        .child(div().flex_1().min_w(px(0.0)).text_size(px(THEME.typography.font_body - 1.0)).text_color(rgba(c.text_secondary)).child(text.into()))
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
        .px(px(12.0))
        .py(px(7.0))
        .rounded(px(THEME.metrics.radius_pill))
        .bg(rgba(c.bg_sidebar))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .child(div().size(px(7.0)).rounded(px(THEME.metrics.radius_pill)).bg(rgba(dot_color)))
        .child(svg().path(icon).size(px(14.0)).text_color(rgba(c.text_secondary)))
        .child(div().text_size(px(THEME.typography.font_badge)).font_weight(FontWeight::MEDIUM).text_color(rgba(c.text_primary)).child(label))
}

fn permission_chip(label: &'static str, allowed: bool) -> Div {
    let c = THEME.colors;
    let (bg, fg, border) = match allowed {
        true => (c.status_ok_bg, c.status_ok_fg, c.status_ok_bg),
        false => (c.bg_sidebar, c.text_secondary, c.border_subtle),
    };

    div()
        .px(px(10.0))
        .py(px(5.0))
        .rounded(px(THEME.metrics.radius_pill))
        .bg(rgba(bg))
        .border_1()
        .border_color(rgba(border))
        .text_size(px(THEME.typography.font_badge))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(fg))
        .child(label)
}
