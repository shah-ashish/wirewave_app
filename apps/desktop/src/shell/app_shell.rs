use crate::shell::{
    sidebar::{render_sidebar, HasNavigation, NavTab},
    titlebar::render_titlebar,
};
use gpui::{div, prelude::*, px, rgba, Context, Entity, Render, Window};
use wirewave_core_config::THEME;
use wirewave_core_session::{SessionState, SharedSession};
use wirewave_feature_connection::HomeScreen;
use wirewave_feature_settings::SettingsScreen;

/// Root desktop shell composing the titlebar, sidebar navigation, Home, and Settings.
pub struct AppShell {
    active_tab: NavTab,
    session: SharedSession,
    home: Entity<HomeScreen>,
    settings: Entity<SettingsScreen>,
}

impl AppShell {
    pub fn new(
        session: SharedSession,
        home: Entity<HomeScreen>,
        settings: Entity<SettingsScreen>,
    ) -> Self {
        Self {
            active_tab: NavTab::Home,
            session,
            home,
            settings,
        }
    }
}

impl HasNavigation for AppShell {
    fn select_tab(&mut self, tab: NavTab) {
        self.active_tab = tab;
    }
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = THEME.colors;
        let is_connected = matches!(self.session.snapshot().state, SessionState::Connected(_));

        let active_view = match self.active_tab {
            NavTab::Home => self.home.clone().into_any_element(),
            NavTab::Settings => self.settings.clone().into_any_element(),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgba(c.bg_canvas))
            .text_color(rgba(c.text_primary))
            .child(render_titlebar(self.session.clone()))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .child(render_sidebar(self.active_tab, is_connected, cx))
                    .child(
                        div()
                            .flex_1()
                            .min_h(px(0.0))
                            .h_full()
                            .bg(rgba(c.bg_canvas))
                            .child(active_view),
                    ),
            )
    }
}
