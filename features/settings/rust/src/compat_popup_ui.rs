use crate::components::surface_card;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight};
use wirewave_core_compatibility::{CompatibilityReport, CompatibilityStatus};
use wirewave_core_config::THEME;

/// Blocking modal overlay displayed on launch if the OS fails compatibility checks.
pub fn render_incompatibility_popup(report: &CompatibilityReport) -> Option<Div> {
    let CompatibilityStatus::Unsupported { ref reason } = report.status else {
        return None;
    };

    let c = THEME.colors;
    let t = THEME.typography;
    let m = THEME.metrics;

    let dialog = surface_card()
        .max_w(px(420.0))
        .p(px(m.spacing_lg))
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(
            div()
                .text_size(px(t.font_title))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.status_err_fg))
                .child("Device Not Compatible"),
        )
        .child(
            div()
                .text_size(px(t.font_body))
                .text_color(rgba(c.text_primary))
                .child(reason.clone()),
        )
        .child(
            div()
                .text_size(px(t.font_badge))
                .text_color(rgba(c.text_secondary))
                .child(format!("Minimum required: {}", report.minimum_required)),
        );

    Some(
        div()
            .absolute()
            .inset_0()
            .bg(rgba(c.scrim))
            .flex()
            .items_center()
            .justify_center()
            .child(dialog),
    )
}
