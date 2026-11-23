use gpui::{div, img, prelude::*, px, rgba, svg, Div, FontWeight, WindowControlArea};
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_core_session::SharedSession;

/// Custom dark Windows 11 titlebar.
pub fn render_titlebar(session: SharedSession) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;

    let brand = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(10.0))
        .pl(px(16.0))
        .child(
            img(APP_CONFIG.assets.icon_desktop_mark)
                .size(px(22.0))
                .rounded(px(4.0)),
        )
        .child(
            div()
                .text_size(px(THEME.typography.font_title))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.text_primary))
                .child(APP_CONFIG.app_name),
        );

    let min_session = session.clone();
    let close_session = session.clone();

    let controls = div()
        .flex()
        .flex_row()
        .items_center()
        .h_full()
        .child(window_control_button(
            "win-min",
            "assets/brand/icons/window-minimize.svg",
            14.0,
            WindowControlArea::Min,
            false,
            move |window| {
                if !min_session.snapshot().preferences.keep_alive_in_background {
                    min_session.disconnect_active();
                }
                window.minimize_window();
            },
        ))
        .child(window_control_button(
            "win-max",
            "assets/brand/icons/window-maximize.svg",
            13.0,
            WindowControlArea::Max,
            false,
            |window| window.zoom_window(),
        ))
        .child(window_control_button(
            "win-close",
            "assets/brand/icons/window-close.svg",
            14.0,
            WindowControlArea::Close,
            true,
            move |window| {
                close_session.disconnect_active();
                window.remove_window();
            },
        ));

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .w_full()
        .h(px(m.titlebar_height))
        .bg(rgba(c.bg_canvas))
        .child(
            div()
                .flex_1()
                .h_full()
                .flex()
                .items_center()
                .window_control_area(WindowControlArea::Drag)
                .child(brand),
        )
        .child(controls)
}

fn window_control_button<F>(
    id: &'static str,
    icon_path: &'static str,
    icon_size: f32,
    area: WindowControlArea,
    is_close: bool,
    on_press: F,
) -> impl IntoElement
where
    F: Fn(&mut gpui::Window) + 'static,
{
    let c = THEME.colors;
    let hover_bg = if is_close {
        0xC42B1CFF
    } else {
        c.bg_card_hover
    };

    div()
        .id(id)
        .w(px(46.0))
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .window_control_area(area)
        .text_color(rgba(c.text_secondary))
        .hover(move |style| {
            style
                .bg(rgba(hover_bg))
                .text_color(rgba(c.text_primary))
        })
        .on_click(move |_event, window, _cx| on_press(window))
        .child(
            svg()
                .path(icon_path)
                .size(px(icon_size))
                .text_color(rgba(c.text_primary)),
        )
}
