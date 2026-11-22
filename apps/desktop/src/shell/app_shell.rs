use crate::shell::{sidebar::render_sidebar, titlebar::render_titlebar};
use gpui::{div, prelude::*, rgba, Context, Entity, Render, Window};
use wirewave_core_config::THEME;
use wirewave_feature_settings::SettingsScreen;

/// Root desktop shell composing the custom titlebar, left rail, and active feature view.
pub struct AppShell {
    settings: Entity<SettingsScreen>,
    is_online: bool,
}

impl AppShell {
    pub fn new(settings: Entity<SettingsScreen>, is_online: bool) -> Self {
        Self {
            settings,
            is_online,
        }
    }
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let c = THEME.colors;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgba(c.bg_canvas))
            .text_color(rgba(c.text_primary))
            .child(render_titlebar())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .size_full()
                    .child(render_sidebar(self.is_online))
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .bg(rgba(c.bg_canvas))
                            .child(self.settings.clone()),
                    ),
            )
    }
}
