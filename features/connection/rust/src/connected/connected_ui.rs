use crate::{
    connected::connected_logic::{ConnectedViewState, ServiceTileSpec},
    service_slots::ActiveServiceSlots,
};
use gpui::{div, prelude::*, px, rgba, svg, Context, Div, FontWeight, Render};
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_core_transport::TransportKind;

pub trait CanDisconnect: 'static {
    fn on_disconnect(&mut self);
}

/// Renders Desktop Connected Home screen matching Desktop_Home_-_Connected.png,
/// with active services rendered via the injected `ActiveServiceSlots`.
pub fn render_connected_view<V: Render + CanDisconnect>(
    state: &ConnectedViewState,
    slots: &dyn ActiveServiceSlots,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    let transport_icon = match state.transport {
        TransportKind::WiFi => APP_CONFIG.assets.icon_wifi_svg,
        TransportKind::Bluetooth => APP_CONFIG.assets.icon_bluetooth_svg,
    };

    let device_card = div()
        .w_full()
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card))
        .p(px(20.0))
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .child(
                    div()
                        .size(px(48.0))
                        .rounded(px(14.0))
                        .bg(rgba(c.bg_card_hover))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            svg()
                                .path(APP_CONFIG.assets.icon_phone_svg)
                                .size(px(22.0))
                                .text_color(rgba(c.text_primary)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .text_size(px(20.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgba(c.text_primary))
                                .child(state.device_name.clone()),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(6.0))
                                        .px(px(10.0))
                                        .py(px(3.0))
                                        .rounded(px(m.radius_pill))
                                        .bg(rgba(c.status_ok_bg))
                                        .child(
                                            div()
                                                .size(px(6.0))
                                                .rounded(px(m.radius_pill))
                                                .bg(rgba(c.status_ok_fg)),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(THEME.typography.font_badge))
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgba(c.status_ok_fg))
                                                .child("Connected"),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(6.0))
                                        .px(px(10.0))
                                        .py(px(3.0))
                                        .rounded(px(m.radius_pill))
                                        .bg(rgba(c.bg_card_hover))
                                        .child(
                                            svg()
                                                .path(transport_icon)
                                                .size(px(12.0))
                                                .text_color(rgba(c.text_secondary)),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(THEME.typography.font_badge))
                                                .text_color(rgba(c.text_primary))
                                                .child(state.transport.label()),
                                        ),
                                ),
                        ),
                ),
        )
        .child(
            div()
                .id("home-disconnect-btn")
                .cursor_pointer()
                .px(px(18.0))
                .py(px(10.0))
                .rounded(px(10.0))
                .bg(rgba(c.bg_card_hover))
                .hover(|s| s.bg(rgba(c.status_err_bg)))
                .text_size(px(THEME.typography.font_body))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.status_err_fg))
                .on_click(cx.listener(|view, _ev, _win, cx| {
                    view.on_disconnect();
                    cx.notify();
                }))
                .child("Disconnect"),
        );

    let row_top = div()
        .flex()
        .flex_row()
        .gap(px(12.0))
        .w_full()
        .child(slots.render_speaker_tile())
        .child(slots.render_mic_tile());

    let row_bottom = div()
        .flex()
        .flex_row()
        .gap(px(12.0))
        .w_full()
        .child(slots.render_mouse_tile())
        .child(slots.render_camera_tile());

    let services_grid = div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .w_full()
        .child(row_top)
        .child(row_bottom);

    div()
        .size_full()
        .p(px(m.spacing_lg + 8.0))
        .flex()
        .flex_col()
        .gap(px(28.0))
        .child(device_card)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    div()
                        .text_size(px(THEME.typography.font_section))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(c.text_section))
                        .child("Active services"),
                )
                .child(services_grid),
        )
}

#[allow(dead_code)]
fn service_placeholder_tile(spec: ServiceTileSpec) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    div()
        .flex_1()
        .min_w(px(0.0))
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card))
        .p(px(18.0))
        .flex()
        .flex_col()
        .justify_between()
        .h(px(120.0))
        .child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .justify_between()
                .child(
                    div()
                        .size(px(36.0))
                        .rounded(px(10.0))
                        .bg(rgba(c.bg_card_hover))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            svg()
                                .path(spec.icon_path)
                                .size(px(18.0))
                                .text_color(rgba(c.text_secondary)),
                        ),
                )
                .child(
                    div()
                        .px(px(8.0))
                        .py(px(3.0))
                        .rounded(px(m.radius_pill))
                        .bg(rgba(c.bg_card_hover))
                        .child(
                            div()
                                .text_size(px(THEME.typography.font_badge))
                                .font_weight(FontWeight::NORMAL)
                                .text_color(rgba(c.text_secondary))
                                .child(spec.status_badge),
                        ),
                ),
        )
        .child(
            div()
                .text_size(px(15.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.text_primary))
                .child(spec.name),
        )
}
