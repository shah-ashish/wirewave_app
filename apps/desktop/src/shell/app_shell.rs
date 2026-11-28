use crate::shell::{
    sidebar::{HasNavigation, NavTab, render_sidebar},
    titlebar::render_titlebar,
};
use gpui::{Context, Entity, FontWeight, Render, Window, div, prelude::*, px, rgba, svg};
use wirewave_core_config::{APP_CONFIG, BuildChannel, THEME};
use wirewave_core_session::{SessionState, SharedSession};
use wirewave_feature_connection::HomeScreen;
use wirewave_feature_settings::{
    SettingsScreen,
    update::{UpdateController, UpdatePhase, render_update_section, update_logic::HasUpdateController},
};

/// Root desktop shell composing titlebar, sidebar navigation, Home, Settings, and Update overlay.
pub struct AppShell {
    active_tab: NavTab,
    session: SharedSession,
    home: Entity<HomeScreen>,
    settings: Entity<SettingsScreen>,
    updater: UpdateController,
    update_open: bool,
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
            updater: UpdateController::default(),
            update_open: false,
        }
    }
}

impl HasUpdateController for AppShell {
    fn updater_mut(&mut self) -> &mut UpdateController {
        &mut self.updater
    }
}

impl HasNavigation for AppShell {
    fn select_tab(&mut self, tab: NavTab) {
        self.active_tab = tab;
    }

    fn toggle_update_panel(&mut self, cx: &mut Context<Self>) {
        self.update_open = !self.update_open;
        let should_auto_check = self.update_open
            && matches!(self.updater.phase, UpdatePhase::Idle)
            && matches!(APP_CONFIG.build_channel, BuildChannel::Dev);
        if should_auto_check {
            self.updater.trigger_check(cx);
        }
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

        let mut content_area = div()
            .relative()
            .flex_1()
            .min_h(px(0.0))
            .h_full()
            .bg(rgba(c.bg_canvas))
            .child(active_view);

        if self.update_open {
            content_area = content_area.child(render_update_overlay(&self.updater, cx));
        }

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
                    .child(render_sidebar(
                        self.active_tab,
                        is_connected,
                        &self.updater.phase,
                        self.update_open,
                        cx,
                    ))
                    .child(content_area),
            )
    }
}

fn render_update_overlay(
    updater: &UpdateController,
    cx: &mut Context<AppShell>,
) -> impl IntoElement {
    let c = THEME.colors;
    let m = THEME.metrics;

    let header = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_size(px(THEME.typography.font_section))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.text_section))
                .child("SOFTWARE UPDATE"),
        )
        .child(
            div()
                .id("close-update-overlay")
                .cursor_pointer()
                .size(px(28.0))
                .rounded(px(8.0))
                .bg(rgba(c.bg_card_hover))
                .hover(|s| s.bg(rgba(c.border_subtle)))
                .flex()
                .items_center()
                .justify_center()
                .on_click(cx.listener(|view, _ev, _win, cx| {
                    view.update_open = false;
                    cx.notify();
                }))
                .child(
                    svg()
                        .path("assets/brand/icons/window-close.svg")
                        .size(px(12.0))
                        .text_color(rgba(c.text_secondary)),
                ),
        );

    let panel = div()
        .w_full()
        .bg(rgba(c.bg_sidebar))
        .border_1()
        .border_color(rgba(c.border_subtle))
        .rounded(px(m.radius_card + 2.0))
        .p(px(16.0))
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(header)
        .child(render_update_section(updater, cx));

    let backdrop = div()
        .id("update-overlay-backdrop")
        .absolute()
        .inset_0()
        .bg(rgba(c.scrim))
        .on_click(cx.listener(|view, _ev, _win, cx| {
            view.update_open = false;
            cx.notify();
        }));

    div()
        .absolute()
        .inset_0()
        .child(backdrop)
        .child(
            div()
                .absolute()
                .bottom(px(m.spacing_md))
                .left(px(m.spacing_md))
                .right(px(m.spacing_md))
                .max_w(px(420.0))
                .child(panel),
        )
}
