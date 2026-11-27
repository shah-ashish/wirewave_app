use gpui::{FontWeight, div, prelude::*, px, rgba, svg};
use wirewave_core_config::THEME;

/// Read-only status component. Feature lifecycle and actions belong to the caller.
pub struct FeatureStatusTile<'a> {
    pub id: &'static str,
    pub title: &'static str,
    pub icon_path: &'static str,
    pub active: bool,
    pub subtitle: &'a str,
    pub detail: Option<&'a str>,
}

impl FeatureStatusTile<'_> {
    pub fn render(self) -> impl IntoElement {
        let colors = THEME.colors;
        let metrics = THEME.metrics;
        let (background, border, accent) = if self.active {
            (colors.bg_nav_active, colors.accent_blue, colors.accent_blue)
        } else {
            (colors.bg_card, colors.border_subtle, colors.text_secondary)
        };

        let mut labels = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(
                div()
                    .text_size(px(THEME.typography.font_body))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(colors.text_primary))
                    .child(self.title),
            )
            .child(
                div()
                    .text_size(px(THEME.typography.font_badge))
                    .text_color(rgba(accent))
                    .child(self.subtitle.to_owned()),
            );

        if let Some(detail) = self.detail {
            labels = labels.child(
                div()
                    .text_size(px(10.0))
                    .text_color(rgba(colors.text_secondary))
                    .child(detail.to_owned()),
            );
        }

        div()
            .id(self.id)
            .flex_1()
            .bg(rgba(background))
            .border_1()
            .border_color(rgba(border))
            .rounded(px(metrics.radius_card - 2.0))
            .px(px(14.0))
            .py(px(14.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(
                svg()
                    .path(self.icon_path)
                    .size(px(18.0))
                    .text_color(rgba(accent)),
            )
            .child(labels)
    }
}
