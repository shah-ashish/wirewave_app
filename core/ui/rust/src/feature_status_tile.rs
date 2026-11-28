use gpui::{FontWeight, div, prelude::*, px, rgba, svg};
use wirewave_core_config::THEME;

/// Read-only status component. Feature lifecycle and actions belong to the caller.
pub struct FeatureStatusTile<'a> {
    pub id: &'static str,
    pub title: &'static str,
    pub latency: Option<&'a str>,
    pub icon_path: &'static str,
    pub active: bool,
    pub subtitle: &'a str,
    pub detail: Option<&'a str>,
}

impl FeatureStatusTile<'_> {
    pub fn render(self) -> gpui::Stateful<gpui::Div> {
        let colors = THEME.colors;
        let metrics = THEME.metrics;

        let (card_border, icon_bg, icon_fg, badge_bg, badge_fg) = match self.active {
            true => (
                colors.accent_blue,
                colors.bg_nav_active,
                colors.accent_blue,
                colors.status_ok_bg,
                colors.status_ok_fg,
            ),
            false => (
                colors.border_subtle,
                colors.bg_card_hover,
                colors.text_secondary,
                colors.bg_sidebar,
                colors.text_secondary,
            ),
        };

        let detail_text = self.detail.unwrap_or("Controlled by phone").to_owned();

        let top_row = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .size(px(36.0))
                    .rounded(px(10.0))
                    .bg(rgba(icon_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        svg()
                            .path(self.icon_path)
                            .size(px(18.0))
                            .text_color(rgba(icon_fg)),
                    ),
            )
            .child(
                div()
                    .px(px(10.0))
                    .py(px(3.0))
                    .rounded(px(metrics.radius_pill))
                    .bg(rgba(badge_bg))
                    .text_size(px(THEME.typography.font_badge - 1.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(badge_fg))
                    .child(self.subtitle.to_owned()),
            );

        let body = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_card_title))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(colors.text_primary))
                            .child(self.title),
                    )
                    .child(
                        div()
                            .text_size(px(THEME.typography.font_badge))
                            .text_color(rgba(colors.text_secondary))
                            .child(self.latency.unwrap_or("—").to_owned()),
                    ),
            )
            .child(
                div()
                    .text_size(px(THEME.typography.font_badge))
                    .text_color(rgba(colors.text_secondary))
                    .child(detail_text),
            );

        div()
            .id(self.id)
            .flex_1()
            .min_w(px(190.0))
            .min_h(px(124.0))
            .bg(rgba(colors.bg_card))
            .border_1()
            .border_color(rgba(card_border))
            .rounded(px(metrics.radius_card))
            .p(px(18.0))
            .overflow_hidden()
            .flex()
            .flex_col()
            .justify_between()
            .gap(px(16.0))
            .child(top_row)
            .child(body)
    }
}
