use crate::{
    components::{section_label, surface_card},
    devices::devices_logic::{
        DeviceStatus, UnifiedDeviceRow, device_summary_label, unified_device_rows,
    },
};
use gpui::{Context, Div, FontWeight, Render, div, prelude::*, px, rgba, svg};
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_core_session::SessionSnapshot;

pub trait HasDeviceListActions: 'static {
    fn disconnect_active_device(&mut self);
    fn block_device_by_id(&mut self, device_id: String);
    fn unblock_device_by_id(&mut self, device_id: String);
    fn forget_device_by_id(&mut self, device_id: String);
}

/// Renders unified Desktop Settings DEVICES card (no inner tabs, no raw device IDs).
pub fn render_devices_section<V: Render + HasDeviceListActions>(
    snapshot: &SessionSnapshot,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let rows = unified_device_rows(snapshot);
    let summary = device_summary_label(snapshot);

    let mut header = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .child(section_label("DEVICES"));

    if !summary.is_empty() {
        header = header.child(
            div()
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(c.text_secondary))
                .child(summary),
        );
    }

    let mut card = surface_card();
    if rows.is_empty() {
        card = card.child(empty_devices_row());
    } else {
        for (idx, row) in rows.into_iter().enumerate() {
            card = card.child(render_device_row(idx, row, cx));
        }
    }

    div()
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(header)
        .child(card)
}

fn empty_devices_row() -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    div()
        .px(px(m.spacing_md))
        .py(px(18.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(14.0))
        .child(phone_badge(c.bg_card_hover, c.text_secondary))
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .flex()
                .flex_col()
                .gap(px(3.0))
                .child(div().text_size(px(THEME.typography.font_body)).font_weight(FontWeight::MEDIUM).text_color(rgba(c.text_primary)).child("No paired devices yet"))
                .child(div().text_size(px(THEME.typography.font_badge)).text_color(rgba(c.text_secondary)).child("Pair a phone from the Home screen to manage it here.")),
        )
}

fn render_device_row<V: Render + HasDeviceListActions>(
    idx: usize,
    row: UnifiedDeviceRow,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let dev_id_a = row.device_id.clone();
    let dev_id_b = row.device_id.clone();

    let (badge_bg, badge_fg, dot_color, status_text, status_color) = match row.status {
        DeviceStatus::Connected(transport) => (
            c.bg_nav_active,
            c.accent_blue,
            Some(c.status_ok_fg),
            format!("Connected · {}", transport.label()),
            c.status_ok_fg,
        ),
        DeviceStatus::Trusted => (
            c.bg_card_hover,
            c.text_secondary,
            None,
            "Trusted · Offline".to_string(),
            c.text_secondary,
        ),
        DeviceStatus::Blocked => (
            c.status_err_bg,
            c.status_err_fg,
            Some(c.status_err_fg),
            "Blocked".to_string(),
            c.status_err_fg,
        ),
    };

    let mut status_line = div().flex().flex_row().items_center().gap(px(6.0));
    if let Some(dot) = dot_color {
        status_line = status_line.child(div().size(px(6.0)).rounded(px(m.radius_pill)).bg(rgba(dot)));
    }
    status_line = status_line.child(div().text_size(px(THEME.typography.font_badge)).text_color(rgba(status_color)).child(status_text));

    let actions = match row.status {
        DeviceStatus::Connected(_) => div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(8.0))
            .child(action_chip(("disc-dev", idx), "Disconnect", c.text_primary, c.bg_canvas, c.text_secondary, cx.listener(|view, _ev, _win, cx| {
                view.disconnect_active_device();
                cx.notify();
            })))
            .child(action_chip(("blk-conn", idx), "Block", c.bg_card_hover, c.status_err_fg, c.status_err_bg, cx.listener(move |view, _ev, _win, cx| {
                view.block_device_by_id(dev_id_a.clone());
                cx.notify();
            }))),
        DeviceStatus::Trusted => div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(8.0))
            .child(action_chip(("fgt-dev", idx), "Forget", c.bg_card_hover, c.text_primary, c.border_subtle, cx.listener(move |view, _ev, _win, cx| {
                view.forget_device_by_id(dev_id_a.clone());
                cx.notify();
            })))
            .child(action_chip(("blk-tr", idx), "Block", c.bg_card_hover, c.status_err_fg, c.status_err_bg, cx.listener(move |view, _ev, _win, cx| {
                view.block_device_by_id(dev_id_b.clone());
                cx.notify();
            }))),
        DeviceStatus::Blocked => div().flex().flex_row().gap(px(8.0)).child(action_chip(
            ("unblk-dev", idx),
            "Unblock",
            c.bg_nav_active,
            c.accent_blue,
            c.border_subtle,
            cx.listener(move |view, _ev, _win, cx| {
                view.unblock_device_by_id(dev_id_a.clone());
                cx.notify();
            }),
        )),
    };

    let info = div()
        .flex_1()
        .min_w(px(200.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(14.0))
        .child(phone_badge(badge_bg, badge_fg))
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(div().text_size(px(THEME.typography.font_body)).font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.text_primary)).child(row.name))
                .child(status_line),
        );

    let base = div()
        .px(px(m.spacing_md))
        .py(px(14.0))
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .justify_between()
        .gap(px(12.0))
        .child(info)
        .child(actions);

    match idx > 0 {
        true => base.border_t_1().border_color(rgba(c.divider)),
        false => base,
    }
}

fn phone_badge(bg: u32, icon_color: u32) -> Div {
    div()
        .size(px(40.0))
        .rounded(px(10.0))
        .bg(rgba(bg))
        .border_1()
        .border_color(rgba(THEME.colors.border_subtle))
        .flex()
        .items_center()
        .justify_center()
        .child(svg().path(APP_CONFIG.assets.icon_phone_svg).size(px(18.0)).text_color(rgba(icon_color)))
}

fn action_chip(
    id: (&'static str, usize),
    label: &'static str,
    bg: u32,
    text_color: u32,
    hover_bg: u32,
    handler: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .cursor_pointer()
        .px(px(14.0))
        .py(px(7.0))
        .rounded(px(8.0))
        .bg(rgba(bg))
        .border_1()
        .border_color(rgba(THEME.colors.border_subtle))
        .hover(move |s| s.bg(rgba(hover_bg)))
        .text_size(px(THEME.typography.font_badge))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(text_color))
        .on_click(handler)
        .child(label)
}
