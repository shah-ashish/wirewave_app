use crate::{
    components::{section_label, status_pill, surface_card},
    drivers::drivers_logic::DriversState,
};
use gpui::{div, prelude::*, px, rgba, Context, Div, FontWeight, Render};
use wirewave_core_config::THEME;
use wirewave_feature_camera::CameraBackendPreference;

pub trait HasDriverActions: 'static {
    fn refresh_drivers(&mut self);
    fn download_recommended_driver(&mut self);
    fn download_recommended_camera_driver(&mut self);
    fn select_camera_backend(&mut self, backend: CameraBackendPreference);
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

    let camera_header = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .child(section_label("VIRTUAL CAMERA DRIVER"));

    let backend_pill = |id: &'static str,
                        label: &'static str,
                        target: CameraBackendPreference,
                        cx: &mut Context<V>| {
        let active = state.camera_backend == target;
        div()
            .id(id)
            .cursor_pointer()
            .px(px(12.0))
            .py(px(6.0))
            .rounded(px(8.0))
            .bg(rgba(if active { colors.accent_blue } else { colors.bg_card_hover }))
            .text_size(px(THEME.typography.font_badge))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(colors.text_primary))
            .on_click(cx.listener(move |view, _, _, cx| {
                view.select_camera_backend(target);
                cx.notify();
            }))
            .child(label)
    };

    let backend_selector = div()
        .px(px(metrics.spacing_md))
        .py(px(12.0))
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
                        .text_color(rgba(colors.text_primary))
                        .child("Active camera backend"),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_badge - 1.0))
                        .text_color(rgba(colors.text_secondary))
                        .child("Choose Windows 11 native virtual camera or OBS fallback"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(6.0))
                .child(backend_pill("cam-backend-auto", "Auto", CameraBackendPreference::Auto, cx))
                .child(backend_pill("cam-backend-win11", "Windows 11", CameraBackendPreference::Windows11, cx))
                .child(backend_pill("cam-backend-obs", "OBS", CameraBackendPreference::Obs, cx)),
        );

    let mf_row = div()
        .px(px(metrics.spacing_md))
        .py(px(12.0))
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
                        .text_color(rgba(colors.text_primary))
                        .child("WireWave Camera (Windows 11)"),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_badge - 1.0))
                        .text_color(rgba(colors.text_secondary))
                        .child("Built-in Media Foundation virtual camera (wirewave_vcam.dll)"),
                ),
        )
        .child(if state.mf_installed.is_some() {
            status_pill(true, "Available")
        } else {
            status_pill(false, "DLL missing")
        });

    let camera_row = if let Some(driver) = &state.obs_installed {
        div()
            .px(px(metrics.spacing_md))
            .py(px(12.0))
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
                            .text_color(rgba(colors.text_primary))
                            .child(driver.name.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_badge - 1.0))
                            .text_color(rgba(colors.text_secondary))
                            .child("OBS DirectShow virtual camera fallback"),
                    ),
            )
            .child(status_pill(true, "Installed"))
    } else {
        div()
            .px(px(metrics.spacing_md))
            .py(px(12.0))
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
                            .text_color(rgba(colors.text_primary))
                            .child("OBS Virtual Camera"),
                    )
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_badge - 1.0))
                            .text_color(rgba(colors.text_secondary))
                            .child("Not installed · Optional fallback"),
                    ),
            )
            .child(
                div()
                    .id("download-recommended-camera-driver")
                    .cursor_pointer()
                    .px(px(14.0))
                    .py(px(7.0))
                    .rounded(px(8.0))
                    .bg(rgba(colors.accent_blue))
                    .text_size(px(THEME.typography.font_badge))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(colors.text_primary))
                    .on_click(cx.listener(|view, _, _, cx| {
                        view.download_recommended_camera_driver();
                        cx.notify();
                    }))
                    .child("Download"),
            )
    };

    div()
        .flex()
        .flex_col()
        .gap(px(metrics.spacing_lg))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(metrics.spacing_sm))
                .child(header)
                .child(surface_card().child(row)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(metrics.spacing_sm))
                .child(camera_header)
                .child(
                    surface_card()
                        .child(backend_selector)
                        .child(mf_row)
                        .child(camera_row),
                ),
        )
}
