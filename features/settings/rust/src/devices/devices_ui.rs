use crate::{
    components::{section_label, surface_card},
    devices::devices_logic::{rows_for_tab, DeviceRowModel, DeviceTab},
};
use gpui::{div, prelude::*, px, rgba, Context, Div, FontWeight, Render};
use wirewave_core_config::THEME;
use wirewave_core_session::SessionSnapshot;

pub trait HasDeviceListActions: 'static {
    fn select_device_tab(&mut self, tab: DeviceTab);
    fn disconnect_active_device(&mut self);
    fn block_device_by_id(&mut self, device_id: String);
    fn unblock_device_by_id(&mut self, device_id: String);
    fn forget_device_by_id(&mut self, device_id: String);
}

/// Renders Desktop Settings DEVICES card matching Desktop_4_-_Settings_-_App.png.
pub fn render_devices_section<V: Render + HasDeviceListActions>(
    snapshot: &SessionSnapshot,
    active_tab: DeviceTab,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    let mut tab_bar = div()
        .flex()
        .flex_row()
        .gap(px(4.0))
        .p(px(4.0))
        .rounded(px(10.0))
        .bg(rgba(c.bg_sidebar));

    for (idx, tab) in DeviceTab::ALL.into_iter().enumerate() {
        let is_active = tab == active_tab;
        let (bg, fg) = match is_active {
            true => (c.border_subtle, c.text_primary),
            false => (c.bg_sidebar, c.text_secondary),
        };
        tab_bar = tab_bar.child(
            div()
                .id(("dev-tab", idx))
                .flex_1()
                .cursor_pointer()
                .py(px(7.0))
                .rounded(px(7.0))
                .bg(rgba(bg))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(THEME.typography.font_badge))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(fg))
                .on_click(cx.listener(move |view, _ev, _win, cx| {
                    view.select_device_tab(tab);
                    cx.notify();
                }))
                .child(tab.label()),
        );
    }

    let rows = rows_for_tab(snapshot, active_tab);
    let mut card = surface_card().child(div().p(px(12.0)).child(tab_bar));

    if rows.is_empty() {
        let empty_text = match active_tab {
            DeviceTab::Connected => "No phone connected",
            DeviceTab::Trusted => "No trusted devices yet",
            DeviceTab::Blocked => "No blocked devices",
        };
        card = card.child(
            div()
                .border_t_1()
                .border_color(rgba(c.divider))
                .px(px(m.spacing_md))
                .py(px(18.0))
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(c.text_secondary))
                .child(empty_text),
        );
    } else {
        for (idx, row) in rows.into_iter().enumerate() {
            card = card.child(render_device_row(idx, row, active_tab, cx));
        }
    }

    div()
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(section_label("DEVICES"))
        .child(card)
}

fn render_device_row<V: Render + HasDeviceListActions>(
    idx: usize,
    row: DeviceRowModel,
    tab: DeviceTab,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let dev_id_a = row.device_id.clone();
    let dev_id_b = row.device_id.clone();

    let actions = match tab {
        DeviceTab::Connected => div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .child(action_chip(
                ("disc-dev", idx),
                "Disconnect",
                c.text_primary,
                cx.listener(|view, _ev, _win, cx| {
                    view.disconnect_active_device();
                    cx.notify();
                }),
            ))
            .child(action_chip(
                ("blk-conn", idx),
                "Block",
                c.status_err_fg,
                cx.listener(move |view, _ev, _win, cx| {
                    view.block_device_by_id(dev_id_a.clone());
                    cx.notify();
                }),
            )),
        DeviceTab::Trusted => div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .child(action_chip(
                ("fgt-dev", idx),
                "Forget",
                c.text_primary,
                cx.listener(move |view, _ev, _win, cx| {
                    view.forget_device_by_id(dev_id_a.clone());
                    cx.notify();
                }),
            ))
            .child(action_chip(
                ("blk-tr", idx),
                "Block",
                c.status_err_fg,
                cx.listener(move |view, _ev, _win, cx| {
                    view.block_device_by_id(dev_id_b.clone());
                    cx.notify();
                }),
            )),
        DeviceTab::Blocked => div().flex().flex_row().gap(px(8.0)).child(action_chip(
            ("unblk-dev", idx),
            "Unblock",
            c.accent_blue,
            cx.listener(move |view, _ev, _win, cx| {
                view.unblock_device_by_id(dev_id_a.clone());
                cx.notify();
            }),
        )),
    };

    div()
        .border_t_1()
        .border_color(rgba(c.divider))
        .px(px(m.spacing_md))
        .py(px(14.0))
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_size(px(THEME.typography.font_body))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(c.text_primary))
                        .child(row.name),
                )
                .child(
                    div()
                        .font_family("Consolas")
                        .text_size(px(THEME.typography.font_badge - 1.0))
                        .text_color(rgba(c.text_secondary))
                        .child(row.short_id),
                ),
        )
        .child(actions)
}

fn action_chip(
    id: (&'static str, usize),
    label: &'static str,
    text_color: u32,
    handler: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    let c = THEME.colors;
    div()
        .id(id)
        .cursor_pointer()
        .px(px(14.0))
        .py(px(7.0))
        .rounded(px(8.0))
        .bg(rgba(c.bg_card_hover))
        .hover(|s| s.bg(rgba(c.border_subtle)))
        .text_size(px(THEME.typography.font_badge))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(text_color))
        .on_click(handler)
        .child(label)
}
