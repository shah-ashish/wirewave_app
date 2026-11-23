use crate::{
    components::{section_label, status_pill, surface_card},
    drivers::drivers_logic::DriversState,
};
use gpui::{div, prelude::*, px, rgba, Context, Div, FontWeight, Render};
use wirewave_core_config::THEME;

pub trait HasDriverActions: 'static {
    fn refresh_drivers(&mut self);
    fn download_recommended_driver(&mut self);
}

pub fn render_drivers_section<V: Render + HasDriverActions>(
    state: &DriversState,
    cx: &mut Context<V>,
) -> Div {
    let colors = THEME.colors;
    let metrics = THEME.metrics;
    let header = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .child(section_label("VIRTUAL MICROPHONE DRIVER"))
        .child(
            div()
                .id("refresh-mic-drivers")
                .cursor_pointer()
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(colors.accent_blue))
                .on_click(cx.listener(|view, _, _, cx| {
                    view.refresh_drivers();
                    cx.notify();
                }))
                .child("Refresh"),
        );

    let row = if let Some(driver) = &state.installed {
        div()
            .px(px(metrics.spacing_md))
            .py(px(14.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_body))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(colors.text_primary))
                            .child(driver.name.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_badge - 1.0))
                            .text_color(rgba(colors.text_secondary))
                            .child("Compatible virtual microphone driver"),
                    ),
            )
            .child(status_pill(true, "Installed"))
    } else {
        div()
            .px(px(metrics.spacing_md))
            .py(px(14.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_body))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(colors.text_primary))
                            .child("VB-CABLE Virtual Audio Device"),
                    )
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_badge - 1.0))
                            .text_color(rgba(colors.text_secondary))
                            .child("No compatible driver found · Recommended"),
                    ),
            )
            .child(
                div()
                    .id("download-recommended-mic-driver")
                    .cursor_pointer()
                    .px(px(14.0))
                    .py(px(7.0))
                    .rounded(px(8.0))
                    .bg(rgba(colors.accent_blue))
                    .text_size(px(THEME.typography.font_badge))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(colors.text_primary))
                    .on_click(cx.listener(|view, _, _, cx| {
                        view.download_recommended_driver();
                        cx.notify();
                    }))
                    .child("Download"),
            )
    };

    div()
        .flex()
        .flex_col()
        .gap(px(metrics.spacing_sm))
        .child(header)
        .child(surface_card().child(row))
}
