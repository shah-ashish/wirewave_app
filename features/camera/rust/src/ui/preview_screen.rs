use crate::service::CameraService;
use gpui::{div, prelude::*, px, rgba, svg, Context, FontWeight, Render, Window};
use std::sync::Arc;
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_core_session::{SessionState, SharedSession};

pub struct CameraPreviewScreen {
    session: SharedSession,
    camera_service: Arc<CameraService>,
}

impl CameraPreviewScreen {
    pub fn new(session: SharedSession, camera_service: Arc<CameraService>) -> Self {
        Self {
            session,
            camera_service,
        }
    }
}

impl Render for CameraPreviewScreen {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let colors = THEME.colors;
        let metrics = THEME.metrics;
        let is_connected = matches!(self.session.snapshot().state, SessionState::Connected(_));
        let is_camera_active = self.camera_service.is_running();
        let driver_name = self
            .camera_service
            .detected_driver()
            .map(|d| d.name)
            .unwrap_or_else(|| "No virtual camera driver detected".into());

        let status_badge = if !is_connected {
            ("Disconnected", colors.status_err_fg, colors.status_err_bg)
        } else if is_camera_active {
            ("Live Stream Active", colors.status_ok_fg, colors.status_ok_bg)
        } else {
            ("Camera Inactive", colors.text_secondary, colors.bg_card)
        };

        let content = if !is_connected {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(16.0))
                .p(px(32.0))
                .child(
                    svg()
                        .path(APP_CONFIG.assets.icon_camera_svg)
                        .size(px(48.0))
                        .text_color(rgba(colors.text_secondary)),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_section))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(colors.text_primary))
                        .child("No Phone Connected"),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_body))
                        .text_color(rgba(colors.text_secondary))
                        .child("Connect your phone on the Home screen to view live camera."),
                )
        } else if !is_camera_active {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(16.0))
                .p(px(32.0))
                .child(
                    svg()
                        .path(APP_CONFIG.assets.icon_camera_svg)
                        .size(px(48.0))
                        .text_color(rgba(colors.accent_blue)),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_section))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(colors.text_primary))
                        .child("Camera Inactive on Phone"),
                )
                .child(
                    div()
                        .text_size(px(THEME.typography.font_body))
                        .text_color(rgba(colors.text_secondary))
                        .child("Navigate to the Camera screen on your phone to start streaming live video."),
                )
        } else {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(16.0))
                .p(px(24.0))
                .child(
                    div()
                        .w_full()
                        .h(px(360.0))
                        .rounded(px(metrics.radius_card))
                        .bg(rgba(colors.bg_canvas))
                        .border_1()
                        .border_color(rgba(colors.border_subtle))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(12.0))
                        .child(
                            svg()
                                .path(APP_CONFIG.assets.icon_camera_svg)
                                .size(px(36.0))
                                .text_color(rgba(colors.accent_blue)),
                        )
                        .child(
                            div()
                                .text_size(px(THEME.typography.font_body))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(colors.text_primary))
                                .child("Streaming Live Video via WireWave UDP"),
                        )
                        .child(
                            div()
                                .text_size(px(THEME.typography.font_badge))
                                .text_color(rgba(colors.text_secondary))
                                .child(format!("Virtual Output Device: {}", driver_name)),
                        ),
                )
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(metrics.spacing_lg))
            .gap(px(metrics.spacing_md))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_title))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgba(colors.text_primary))
                            .child("Camera Preview"),
                    )
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(metrics.radius_pill))
                            .bg(rgba(status_badge.2))
                            .text_size(px(THEME.typography.font_badge))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(status_badge.1))
                            .child(status_badge.0),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .bg(rgba(colors.bg_card))
                    .border_1()
                    .border_color(rgba(colors.border_subtle))
                    .rounded(px(metrics.radius_card))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(content),
            )
    }
}
