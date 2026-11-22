use crate::{
    components::{section_label, surface_card},
    connection::connection_logic::{ConnectionSettingsState, ALL_PRIORITIES},
};
use gpui::{div, prelude::*, px, rgba, Context, Div, FontWeight, Render};
use wirewave_core_config::THEME;
use wirewave_core_transport::TransportPriority;

pub trait HasConnectionSettingsActions: 'static {
    fn toggle_priority_dropdown(&mut self);
    fn select_priority(&mut self, priority: TransportPriority);
    fn toggle_keep_alive(&mut self);
}

/// Renders Desktop Settings CONNECTION card matching Desktop_4_-_Settings_-_App.png.
pub fn render_connection_section<V: Render + HasConnectionSettingsActions>(
    state: &ConnectionSettingsState,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let t = THEME.typography;
    let m = THEME.metrics;

    let priority_row = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .min_h(px(m.row_height))
        .px(px(m.spacing_md))
        .child(
            div()
                .text_size(px(t.font_body))
                .text_color(rgba(c.text_primary))
                .child("Connection priority"),
        )
        .child(
            div()
                .id("priority-dropdown-trigger")
                .cursor_pointer()
                .px(px(14.0))
                .py(px(7.0))
                .rounded(px(8.0))
                .bg(rgba(c.bg_card_hover))
                .border_1()
                .border_color(rgba(c.border_subtle))
                .text_size(px(t.font_badge))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.text_primary))
                .on_click(cx.listener(|view, _ev, _win, cx| {
                    view.toggle_priority_dropdown();
                    cx.notify();
                }))
                .child(format!("{}  ▾", state.priority.label())),
        );

    let toggle_track_bg = match state.keep_alive_in_background {
        true => c.status_ok_fg,
        false => c.border_subtle,
    };
    let toggle_align = match state.keep_alive_in_background {
        true => div().justify_end(),
        false => div().justify_start(),
    };

    let keep_alive_row = div()
        .border_t_1()
        .border_color(rgba(c.divider))
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
                        .text_size(px(t.font_body))
                        .text_color(rgba(c.text_primary))
                        .child("Keep alive in background"),
                )
                .child(
                    div()
                        .text_size(px(t.font_badge - 1.0))
                        .text_color(rgba(c.text_secondary))
                        .child("Closing the window minimizes to the tray and keeps the connection running"),
                ),
        )
        .child(
            toggle_align
                .id("keep-alive-toggle")
                .cursor_pointer()
                .w(px(46.0))
                .h(px(26.0))
                .p(px(3.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(toggle_track_bg))
                .flex()
                .flex_row()
                .items_center()
                .on_click(cx.listener(|view, _ev, _win, cx| {
                    view.toggle_keep_alive();
                    cx.notify();
                }))
                .child(
                    div()
                        .size(px(20.0))
                        .rounded(px(m.radius_pill))
                        .bg(rgba(c.text_primary)),
                ),
        );

    let mut card = surface_card().child(priority_row);

    if state.dropdown_open {
        let mut menu = div()
            .border_t_1()
            .border_color(rgba(c.divider))
            .bg(rgba(c.bg_sidebar))
            .px(px(m.spacing_md))
            .py(px(6.0))
            .flex()
            .flex_col()
            .gap(px(4.0));

        for (idx, option) in ALL_PRIORITIES.into_iter().enumerate() {
            let is_selected = option == state.priority;
            let text_color = match is_selected {
                true => c.accent_blue,
                false => c.text_primary,
            };
            menu = menu.child(
                div()
                    .id(("priority-opt", idx))
                    .cursor_pointer()
                    .px(px(10.0))
                    .py(px(8.0))
                    .rounded(px(8.0))
                    .hover(|s| s.bg(rgba(c.bg_card_hover)))
                    .text_size(px(t.font_badge))
                    .text_color(rgba(text_color))
                    .on_click(cx.listener(move |view, _ev, _win, cx| {
                        view.select_priority(option);
                        cx.notify();
                    }))
                    .child(option.label()),
            );
        }
        card = card.child(menu);
    }

    card = card.child(keep_alive_row);

    div()
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(section_label("CONNECTION"))
        .child(card)
}
