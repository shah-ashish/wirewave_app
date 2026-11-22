use gpui::{
    div, img, prelude::*, px, rgba, Div, FontWeight, WindowControlArea,
};
use wirewave_core_config::{APP_CONFIG, THEME};

/// Custom dark Windows 11 titlebar matching the Desktop Settings screenshot.
pub fn render_titlebar() -> Div {
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

    let controls = div()
        .flex()
        .flex_row()
        .items_center()
        .h_full()
        .child(window_control_button(
            "win-min",
            "—",
            WindowControlArea::Min,
            false,
            |window| window.minimize_window(),
        ))
        .child(window_control_button(
            "win-max",
            "□",
            WindowControlArea::Max,
            false,
            |window| window.zoom_window(),
        ))
        .child(window_control_button(
            "win-close",
            "×",
            WindowControlArea::Close,
            true,
            |window| window.remove_window(),
        ));

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .w_full()
        .h(px(m.titlebar_height))
        .bg(rgba(c.bg_canvas))
        .border_b_1()
        .border_color(rgba(c.border_subtle))
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
    glyph: &'static str,
    area: WindowControlArea,
    is_close: bool,
    on_press: F,
) -> impl IntoElement
where
    F: Fn(&mut gpui::Window) + 'static,
{
    let c = THEME.colors;
    let hover_bg = match is_close {
        true => c.status_err_fg,
        false => c.bg_card_hover,
    };

    div()
        .id(id)
        .w(px(44.0))
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .window_control_area(area)
        .text_size(px(14.0))
        .text_color(rgba(c.text_secondary))
        .hover(move |s| s.bg(rgba(hover_bg)).text_color(rgba(c.text_primary)))
        .on_click(move |_event, window, _cx| on_press(window))
        .child(glyph)
}
