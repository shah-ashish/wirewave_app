use crate::{
    components::{section_label, surface_card},
    services::services_logic::{ServicePermissionItem, ServicesSettingsState},
};
use gpui::{Context, Div, Render, div, prelude::*, px, rgba};
use wirewave_core_config::THEME;
use wirewave_core_session::ChannelId;

pub trait HasServicesSettingsActions: 'static {
    fn toggle_speaker_permission(&mut self);
    fn toggle_mic_permission(&mut self);
    fn toggle_mouse_permission(&mut self);
    fn toggle_permission_for_channel(&mut self, channel: ChannelId) {
        match channel {
            ChannelId::Speaker => self.toggle_speaker_permission(),
            ChannelId::Mic => self.toggle_mic_permission(),
            ChannelId::Mouse => self.toggle_mouse_permission(),
            _ => {}
        }
    }
}

pub fn render_services_section<V: Render + HasServicesSettingsActions>(
    state: &ServicesSettingsState,
    cx: &mut Context<V>,
) -> Div {
    let m = THEME.metrics;
    let mut card = surface_card();

    for (idx, item) in state.items.iter().enumerate() {
        let mut row = render_permission_row(item, cx);
        if idx > 0 {
            row = row.border_t_1().border_color(rgba(THEME.colors.divider));
        }
        card = card.child(row);
    }

    div()
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(section_label("SERVICES"))
        .child(card)
}

fn render_permission_row<V: Render + HasServicesSettingsActions>(
    item: &ServicePermissionItem,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let track = if item.enabled {
        c.status_ok_fg
    } else {
        c.border_subtle
    };
    let align = if item.enabled {
        div().justify_end()
    } else {
        div().justify_start()
    };
    let channel = item.channel;

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
                        .child(item.title),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_badge - 1.0))
                        .text_color(rgba(c.text_secondary))
                        .child(item.subtitle),
                ),
        )
        .child(
            align
                .id(item.id)
                .cursor_pointer()
                .w(px(46.0))
                .h(px(26.0))
                .p(px(3.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(track))
                .flex()
                .items_center()
                .on_click(cx.listener(move |view, _ev, _win, cx| {
                    view.toggle_permission_for_channel(channel);
                    cx.notify();
                }))
                .child(
                    div()
                        .size(px(20.0))
                        .rounded(px(m.radius_pill))
                        .bg(rgba(c.bg_canvas)),
                ),
        )
}
