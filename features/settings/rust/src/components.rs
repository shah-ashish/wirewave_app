use gpui::{div, prelude::*, px, rgba, svg, Div, FontWeight, SharedString};
use wirewave_core_config::{APP_CONFIG, THEME};

/// Shared rounded dark surface card matching the OLED Settings layout.
pub fn surface_card() -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    div()
        .w_full()
        .bg(rgba(c.bg_card))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card))
        .overflow_hidden()
}

/// Uppercase muted section header (e.g. "ABOUT").
pub fn section_label(label: impl Into<SharedString>) -> Div {
    let c = THEME.colors;
    let t = THEME.typography;
    div()
        .text_size(px(t.font_section))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(c.text_section))
        .pl(px(4.0))
        .child(label.into())
}

/// Single key-value row inside a grouped settings card.
pub fn setting_row(
    label: impl Into<SharedString>,
    trailing: impl IntoElement,
    show_top_divider: bool,
) -> Div {
    let c = THEME.colors;
    let t = THEME.typography;
    let m = THEME.metrics;

    let base = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .h(px(m.row_height))
        .px(px(m.spacing_md))
        .child(
            div()
                .text_size(px(t.font_body))
                .text_color(rgba(c.text_primary))
                .child(label.into()),
        )
        .child(trailing);

    match show_top_divider {
        true => base.border_t_1().border_color(rgba(c.divider)),
        false => base,
    }
}

/// Muted right-aligned value text for standard or monospace metadata rows.
pub fn value_text(value: impl Into<SharedString>, monospace: bool) -> Div {
    let c = THEME.colors;
    let t = THEME.typography;
    let size = match monospace {
        true => t.font_mono,
        false => t.font_body,
    };
    let el = div()
        .text_size(px(size))
        .text_color(rgba(c.text_secondary))
        .child(value.into());

    match monospace {
        true => el.font_family("Consolas"),
        false => el,
    }
}

/// Pill badge for Supported / Unsupported compatibility status.
pub fn status_pill(supported: bool, label: impl Into<SharedString>) -> Div {
    let c = THEME.colors;
    let t = THEME.typography;
    let m = THEME.metrics;
    let (bg, fg, prefix_icon) = match supported {
        true => (c.status_ok_bg, c.status_ok_fg, Some(APP_CONFIG.assets.icon_check_svg)),
        false => (c.status_err_bg, c.status_err_fg, None),
    };

    let mut pill = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(m.spacing_xs))
        .px(px(12.0))
        .py(px(5.0))
        .rounded(px(m.radius_pill))
        .bg(rgba(bg))
        .text_size(px(t.font_badge))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(fg));

    if let Some(icon_path) = prefix_icon {
        pill = pill.child(
            svg()
                .path(icon_path)
                .size(px(13.0))
                .text_color(rgba(fg)),
        );
    }

    pill.child(label.into())
}
