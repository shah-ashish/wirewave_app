use crate::{
    components::{section_label, surface_card},
    services::services_logic::ServicesSettingsState,
};
use gpui::{div, prelude::*, px, rgba, Context, Div, Render};
use wirewave_core_config::THEME;

pub trait HasServicesSettingsActions: 'static {
    fn toggle_speaker_permission(&mut self);
    fn toggle_mic_permission(&mut self);
}

#[derive(Clone, Copy)]
enum ServicePermission {
    Speaker,
    Mic,
}

pub fn render_services_section<V: Render + HasServicesSettingsActions>(
    state: &ServicesSettingsState,
    cx: &mut Context<V>,
) -> Div {
    let m = THEME.metrics;
    let speaker = render_permission_row(
        "speaker-permission-toggle",
        "Speaker",
        "Allow the connected phone to start and stop desktop audio streaming",
        state.speaker_allowed,
        ServicePermission::Speaker,
        cx,
    );
    let mic = render_permission_row(
        "mic-permission-toggle",
        "Mic",
        "Allow the connected phone to start and stop microphone streaming",
        state.mic_allowed,
        ServicePermission::Mic,
        cx,
    )
    .border_t_1()
    .border_color(rgba(THEME.colors.divider));

    div()
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(section_label("SERVICES"))
        .child(surface_card().child(speaker).child(mic))
}

fn render_permission_row<V: Render + HasServicesSettingsActions>(
    id: &'static str,
    title: &'static str,
    subtitle: &'static str,
    enabled: bool,
    permission: ServicePermission,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let track = if enabled { c.status_ok_fg } else { c.border_subtle };
    let align = if enabled { div().justify_end() } else { div().justify_start() };

    div()
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
                            .child(title),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_badge - 1.0))
                        .text_color(rgba(c.text_secondary))
                        .child(subtitle),
                ),
        )
        .child(
            align
                .id(id)
                .cursor_pointer()
                .w(px(46.0))
                .h(px(26.0))
                .p(px(3.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(track))
                .flex()
                .flex_row()
                .items_center()
                .on_click(cx.listener(move |view, _ev, _win, cx| {
                    match permission {
                        ServicePermission::Speaker => view.toggle_speaker_permission(),
                        ServicePermission::Mic => view.toggle_mic_permission(),
                    }
                    cx.notify();
                }))
                .child(
                    div()
                        .size(px(20.0))
                        .rounded(px(m.radius_pill))
                        .bg(rgba(c.text_primary)),
                ),
        )
}
