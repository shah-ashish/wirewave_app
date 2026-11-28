use crate::{connected::connected_logic::ConnectedViewState, service_slots::ActiveServiceSlots};
use gpui::{Context, Div, FontWeight, Render, div, prelude::*, px, rgba, svg};
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_core_transport::TransportKind;

pub trait CanDisconnect: 'static {
    fn on_disconnect(&mut self);
}

/// Renders responsive Desktop Connected Home with a device card and wrapping service grid.
pub fn render_connected_view<V: Render + CanDisconnect>(
    state: &ConnectedViewState,
    slots: &dyn ActiveServiceSlots,
    cx: &mut Context<V>,
) -> impl IntoElement {
    let c = THEME.colors;
    let m = THEME.metrics;

    let services_row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap(px(14.0))
        .w_full()
        .child(slots.render_speaker_tile())
        .child(slots.render_mic_tile())
        .child(slots.render_mouse_tile());

    let services_section = div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .w_full()
        .child(
            div()
                .text_size(px(THEME.typography.font_section))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.text_section))
                .child("ACTIVE SERVICES"),
        )
        .child(services_row);

    let footer_note = div()
        .text_size(px(THEME.typography.font_badge))
        .text_color(rgba(c.text_secondary))
        .child("Audio and pointer streams are controlled from your phone.");

    let content = div()
        .w_full()
        .max_w(px(m.content_max_width + 260.0))
        .flex()
        .flex_col()
        .gap(px(m.spacing_lg))
        .child(device_header_card(state, cx))
        .child(services_section)
        .child(footer_note);

    div()
        .id("connected-scroll-root")
        .size_full()
        .overflow_y_scroll()
        .p(px(m.spacing_lg))
        .flex()
        .flex_col()
        .items_center()
        .child(content)
}

fn device_header_card<V: Render + CanDisconnect>(
    state: &ConnectedViewState,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let transport_icon = match state.transport {
        TransportKind::WiFi => APP_CONFIG.assets.icon_wifi_svg,
        TransportKind::Bluetooth => APP_CONFIG.assets.icon_bluetooth_svg,
    };

    let phone_badge = div()
        .size(px(48.0))
        .rounded(px(14.0))
        .bg(rgba(c.bg_card_hover))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .flex()
        .items_center()
        .justify_center()
        .child(svg().path(APP_CONFIG.assets.icon_phone_svg).size(px(22.0)).text_color(rgba(c.text_primary)));

    let connected_pill = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.0))
        .px(px(10.0))
        .py(px(3.0))
        .rounded(px(m.radius_pill))
        .bg(rgba(c.status_ok_bg))
        .child(div().size(px(6.0)).rounded(px(m.radius_pill)).bg(rgba(c.status_ok_fg)))
        .child(div().text_size(px(THEME.typography.font_badge)).font_weight(FontWeight::MEDIUM).text_color(rgba(c.status_ok_fg)).child("Connected"));

    let transport_pill = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.0))
        .px(px(10.0))
        .py(px(3.0))
        .rounded(px(m.radius_pill))
        .bg(rgba(c.bg_sidebar))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .child(svg().path(transport_icon).size(px(12.0)).text_color(rgba(c.text_secondary)))
        .child(div().text_size(px(THEME.typography.font_badge)).text_color(rgba(c.text_primary)).child(state.transport.label()));

    let device_info = div()
        .flex_1()
        .min_w(px(220.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(14.0))
        .child(phone_badge)
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).text_color(rgba(c.text_primary)).child(state.device_name.clone()))
                .child(div().flex().flex_row().flex_wrap().items_center().gap(px(8.0)).child(connected_pill).child(transport_pill)),
        );

    let disconnect_btn = div()
        .id("home-disconnect-btn")
        .cursor_pointer()
        .px(px(18.0))
        .py(px(10.0))
        .rounded(px(10.0))
        .bg(rgba(c.text_primary))
        .border_1()
        .border_color(rgba(c.text_primary))
        .hover(|s| s.bg(rgba(c.text_secondary)).border_color(rgba(c.text_secondary)))
        .text_size(px(THEME.typography.font_body))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgba(c.bg_canvas))
        .on_click(cx.listener(|view, _ev, _win, cx| {
            view.on_disconnect();
            cx.notify();
        }))
        .child("Disconnect");

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
        .child(device_info)
        .child(disconnect_btn)
}
