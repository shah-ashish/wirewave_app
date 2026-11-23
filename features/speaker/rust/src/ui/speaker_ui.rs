use crate::ui::speaker_logic::SpeakerTileState;
use gpui::{div, prelude::*, px, rgba, svg, Context, FontWeight, Render};
use wirewave_core_config::{APP_CONFIG, THEME};

pub trait CanToggleSpeaker: 'static {
    fn on_toggle_speaker(&mut self);
}

/// Interactive Desktop Speaker tile matching Desktop_Home_-_Connected.png.
/// Replaces the Stage 2 "Coming soon" placeholder for Speaker.
pub fn render_speaker_tile<V: Render + CanToggleSpeaker>(
    state: &SpeakerTileState,
    cx: &mut Context<V>,
) -> impl IntoElement {
    let c = THEME.colors;
    let m = THEME.metrics;

    let (bg, border, icon_fg, status_fg) = match state.is_on {
        true => (c.bg_nav_active, c.accent_blue, c.accent_blue, c.accent_blue),
        false => (c.bg_card, c.border_subtle, c.text_secondary, c.text_secondary),
    };

    let mut labels = div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(
            div()
                .text_size(px(THEME.typography.font_body))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.text_primary))
                .child("Speaker"),
        )
        .child(
            div()
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(status_fg))
                .child(state.subtitle.clone()),
        );

    if let Some(ref hint) = state.endpoint_hint {
        labels = labels.child(
            div()
                .text_size(px(10.0))
                .text_color(rgba(c.text_secondary))
                .child(hint.clone()),
        );
    }

    div()
        .id("speaker-service-tile")
        .flex_1()
        .cursor_pointer()
        .bg(rgba(bg))
        .border_1()
        .border_color(rgba(border))
        .rounded(px(m.radius_card - 2.0))
        .px(px(14.0))
        .py(px(14.0))
        .hover(|s| s.bg(rgba(c.bg_card_hover)))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.0))
        .on_click(cx.listener(|view, _ev, _win, cx| {
            view.on_toggle_speaker();
            cx.notify();
        }))
        .child(
            svg()
                .path(APP_CONFIG.assets.icon_speaker_svg)
                .size(px(18.0))
                .text_color(rgba(icon_fg)),
        )
        .child(labels)
}
