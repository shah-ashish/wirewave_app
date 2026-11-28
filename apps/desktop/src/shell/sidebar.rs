use gpui::{Context, Div, Render, div, prelude::*, px, rgba, svg};
use wirewave_core_config::{APP_CONFIG, THEME};
use wirewave_feature_settings::update::UpdatePhase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavTab {
    Home,
    Settings,
}

pub trait HasNavigation: 'static + Sized {
    fn select_tab(&mut self, tab: NavTab);
    fn toggle_update_panel(&mut self, cx: &mut Context<Self>);
}

/// Left navigation rail with Home + Settings buttons, bottom Update icon, and connection dot.
pub fn render_sidebar<V: Render + HasNavigation>(
    active_tab: NavTab,
    is_connected: bool,
    update_phase: &UpdatePhase,
    update_open: bool,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let a = APP_CONFIG.assets;

    let nav_items = div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(10.0))
        .child(nav_button(
            "nav-home",
            a.icon_home_svg,
            NavTab::Home,
            active_tab == NavTab::Home,
            cx,
        ))
        .child(nav_button(
            "nav-settings",
            a.icon_settings_svg,
            NavTab::Settings,
            active_tab == NavTab::Settings,
            cx,
        ));

    let dot_color = match is_connected {
        true => c.status_ok_fg,
        false => c.text_secondary,
    };

    let status_indicator = div()
        .relative()
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .child(svg().path(a.icon_monitor_svg).size(px(20.0)).text_color(rgba(c.text_secondary)))
        .child(
            div()
                .absolute()
                .bottom(px(3.0))
                .right(px(2.0))
                .size(px(8.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(dot_color))
                .border_1()
                .border_color(rgba(c.bg_canvas)),
        );

    let bottom_rail = div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(12.0))
        .child(update_button(update_phase, update_open, cx))
        .child(status_indicator);

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_between()
        .w(px(m.sidebar_width))
        .h_full()
        .py(px(16.0))
        .bg(rgba(c.bg_canvas))
        .child(nav_items)
        .child(bottom_rail)
}

fn update_button<V: Render + HasNavigation>(
    phase: &UpdatePhase,
    is_open: bool,
    cx: &mut Context<V>,
) -> impl IntoElement {
    let c = THEME.colors;
    let m = THEME.metrics;

    let (bg, icon_color, badge_color) = match (is_open, phase) {
        (true, _) => (c.bg_nav_active, c.accent_blue, None),
        (false, UpdatePhase::Checking | UpdatePhase::Downloading { .. }) => {
            (c.bg_canvas, c.accent_blue, Some(c.accent_blue))
        }
        (false, UpdatePhase::Available(_)) => (c.bg_canvas, c.text_primary, Some(c.accent_blue)),
        (false, UpdatePhase::ReadyToInstall { .. }) => {
            (c.bg_canvas, c.text_primary, Some(c.status_ok_fg))
        }
        (false, UpdatePhase::Failed { .. }) => {
            (c.bg_canvas, c.text_secondary, Some(c.status_err_fg))
        }
        (false, UpdatePhase::Idle | UpdatePhase::UpToDate) => (c.bg_canvas, c.text_secondary, None),
    };

    let mut btn = div()
        .id("nav-update")
        .relative()
        .cursor_pointer()
        .size(px(42.0))
        .rounded(px(m.radius_nav))
        .bg(rgba(bg))
        .hover(|s| s.bg(rgba(c.bg_card_hover)))
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(|view, _ev, _win, cx| {
            view.toggle_update_panel(cx);
            cx.notify();
        }))
        .child(
            svg()
                .path("assets/brand/icons/update.svg")
                .size(px(20.0))
                .text_color(rgba(icon_color)),
        );

    if let Some(dot_color) = badge_color {
        btn = btn.child(
            div()
                .absolute()
                .top(px(7.0))
                .right(px(7.0))
                .size(px(7.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(dot_color))
                .border_1()
                .border_color(rgba(c.bg_canvas)),
        );
    }

    btn
}

fn nav_button<V: Render + HasNavigation>(
    id: &'static str,
    icon_path: &'static str,
    target: NavTab,
    is_active: bool,
    cx: &mut Context<V>,
) -> impl IntoElement {
    let c = THEME.colors;
    let m = THEME.metrics;
    let (bg, icon_color) = match is_active {
        true => (c.bg_nav_active, c.accent_blue),
        false => (c.bg_canvas, c.text_secondary),
    };

    div()
        .id(id)
        .cursor_pointer()
        .size(px(42.0))
        .rounded(px(m.radius_nav))
        .bg(rgba(bg))
        .hover(|s| s.bg(rgba(c.bg_card_hover)))
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(move |view, _ev, _win, cx| {
            view.select_tab(target);
            cx.notify();
        }))
        .child(svg().path(icon_path).size(px(20.0)).text_color(rgba(icon_color)))
}
