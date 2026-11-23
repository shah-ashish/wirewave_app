use crate::pairing::pairing_logic::PairingModalState;
use gpui::{div, prelude::*, px, relative, rgba, svg, Context, Div, FontWeight, Render};
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_core_transport::TransportKind;

pub trait CanCancelPairing: 'static {
    fn on_cancel_pairing(&mut self);
}

/// Renders the Desktop OTP pairing modal matching Desktop_Home_-_Pairing_request.png.
pub fn render_pairing_modal<V: Render + CanCancelPairing>(
    state: &PairingModalState,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    let transport_icon = match state.transport {
        TransportKind::WiFi => APP_CONFIG.assets.icon_wifi_svg,
        TransportKind::Bluetooth => APP_CONFIG.assets.icon_bluetooth_svg,
    };

    let dialog = div()
        .w(px(380.0))
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card + 4.0))
        .p(px(28.0))
        .flex()
        .flex_col()
        .items_center()
        .gap(px(14.0))
        .child(
            div()
                .text_size(px(11.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.text_section))
                .child("Pairing request"),
        )
        .child(
            div()
                .text_size(px(22.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgba(c.text_primary))
                .child(state.device_name.clone()),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .px(px(12.0))
                .py(px(5.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(c.bg_card_hover))
                .child(
                    svg()
                        .path(transport_icon)
                        .size(px(13.0))
                        .text_color(rgba(c.text_secondary)),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_badge))
                        .text_color(rgba(c.text_primary))
                        .child(format!("via {}", state.transport.label())),
                ),
        )
        .child(
            div()
                .py(px(10.0))
                .font_family("Consolas")
                .text_size(px(38.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgba(c.text_primary))
                .child(state.formatted_otp.clone()),
        )
        .child(
            div()
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(c.text_secondary))
                .child("Enter this code on your phone."),
        )
        .child(
            div()
                .w_full()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .pt(px(4.0))
                .child(
                    div()
                        .flex_1()
                        .h(px(4.0))
                        .rounded(px(m.radius_pill))
                        .bg(rgba(c.border_subtle))
                        .overflow_hidden()
                        .child(
                            div()
                                .h_full()
                                .w(relative(state.progress_fraction))
                                .rounded(px(m.radius_pill))
                                .bg(rgba(c.accent_blue)),
                        ),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_badge))
                        .text_color(rgba(c.text_secondary))
                        .child(format!("{}s", state.remaining_secs)),
                ),
        )
        .child(
            div()
                .id("cancel-pairing-btn")
                .w_full()
                .mt(px(6.0))
                .h(px(42.0))
                .rounded(px(10.0))
                .bg(rgba(c.bg_card_hover))
                .hover(|s| s.bg(rgba(c.border_subtle)))
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(THEME.typography.font_body))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.text_primary))
                .on_click(cx.listener(|view, _ev, _win, cx| {
                    view.on_cancel_pairing();
                    cx.notify();
                }))
                .child("Cancel pairing"),
        );

    div()
        .absolute()
        .inset_0()
        .bg(rgba(c.scrim))
        .flex()
        .items_center()
        .justify_center()
        .child(dialog)
}
