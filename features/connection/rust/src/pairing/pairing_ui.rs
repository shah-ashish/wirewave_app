use crate::pairing::pairing_logic::PairingModalState;
use gpui::{div, prelude::*, px, relative, rgba, svg, Context, Div, FontWeight, Render};
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_core_transport::TransportKind;

pub trait CanCancelPairing: 'static {
    fn on_cancel_pairing(&mut self);
}

/// Renders the Desktop OTP pairing modal over a dimmed scrim.
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

    let otp_box = div()
        .w_full()
        .py(px(16.0))
        .px(px(20.0))
        .rounded(px(m.radius_nav + 2.0))
        .bg(rgba(c.bg_sidebar))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .flex()
        .flex_col()
        .items_center()
        .gap(px(6.0))
        .child(
            div()
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
        );

    let dialog = div()
        .w_full()
        .max_w(px(400.0))
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
                .text_size(px(THEME.typography.font_section))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.text_section))
                .child("PAIRING REQUEST"),
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
                .bg(rgba(c.bg_sidebar))
                .border_1()
                .border_color(rgba(c.border_subtle))
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
        .child(otp_box)
        .child(
            div()
                .w_full()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .pt(px(2.0))
                .child(
                    div()
                        .flex_1()
                        .h(px(5.0))
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
                        .font_family("Consolas")
                        .text_size(px(THEME.typography.font_badge))
                        .text_color(rgba(c.text_secondary))
                        .child(format!("{}s", state.remaining_secs)),
                ),
        )
        .child(
            div()
                .id("cancel-pairing-btn")
                .w_full()
                .mt(px(4.0))
                .h(px(42.0))
                .rounded(px(10.0))
                .bg(rgba(c.bg_card_hover))
                .border_1()
                .border_color(rgba(c.border_subtle))
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
        .p(px(m.spacing_md))
        .bg(rgba(c.scrim))
        .flex()
        .items_center()
        .justify_center()
        .child(dialog)
}
