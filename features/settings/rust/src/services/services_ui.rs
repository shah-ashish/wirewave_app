use crate::{
    components::{section_label, surface_card},
    services::services_logic::ServicesSettingsState,
};
use gpui::{div, prelude::*, px, rgba, Context, Div, Render};
use wirewave_core_config::THEME;

pub trait HasServicesSettingsActions: 'static {
    fn toggle_speaker_permission(&mut self);
}

pub fn render_services_section<V: Render + HasServicesSettingsActions>(
    state: &ServicesSettingsState,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let track = if state.speaker_allowed {
        c.status_ok_fg
    } else {
        c.border_subtle
    };
    let align = if state.speaker_allowed {
        div().justify_end()
    } else {
        div().justify_start()
    };

    let row = div()
        .px(px(m.spacing_md))
        .py(px(14.0))
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(px(16.0))
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(3.0))
                .child(
                    div()
                        .text_size(px(THEME.typography.font_body))
                        .text_color(rgba(c.text_primary))
                        .child("Speaker"),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_badge - 1.0))
                        .text_color(rgba(c.text_secondary))
                        .child(
                            "Allow the connected phone to start and stop desktop audio streaming",
                        ),
                ),
        )
        .child(
            align
                .id("speaker-permission-toggle")
                .cursor_pointer()
                .w(px(46.0))
                .h(px(26.0))
                .p(px(3.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(track))
                .flex()
                .flex_row()
                .items_center()
                .on_click(cx.listener(|view, _ev, _win, cx| {
                    view.toggle_speaker_permission();
                    cx.notify();
                }))
                .child(
                    div()
                        .size(px(20.0))
                        .rounded(px(m.radius_pill))
                        .bg(rgba(c.text_primary)),
                ),
        );

    div()
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(section_label("SERVICES"))
        .child(surface_card().child(row))
}
