use crate::{
    about::{load_about_snapshot, render_about_section, AboutSnapshot},
    compat_popup_ui::render_incompatibility_popup,
    update::{update_logic::HasUpdateController, render_update_section, UpdateController},
};
use gpui::{div, prelude::*, px, Context, Render, Window};
use wirewave_core_compatibility::CompatibilityReport;
use wirewave_core_config::THEME;
use wirewave_core_identity::DeviceIdentity;

/// Composes the Update card, About card, and blocking compatibility popup.
pub struct SettingsScreen {
    about: AboutSnapshot,
    updater: UpdateController,
}

impl SettingsScreen {
    pub fn new(
        identity: &DeviceIdentity,
        compatibility: CompatibilityReport,
        connected_phone: Option<String>,
    ) -> Self {
        Self {
            about: load_about_snapshot(identity, compatibility, connected_phone),
            updater: UpdateController::default(),
        }
    }
}

impl HasUpdateController for SettingsScreen {
    fn updater_mut(&mut self) -> &mut UpdateController {
        &mut self.updater
    }
}

impl Render for SettingsScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let m = THEME.metrics;

        let content = div()
            .w_full()
            .max_w(px(m.content_max_width))
            .flex()
            .flex_col()
            .gap(px(m.spacing_lg))
            .child(render_update_section(&self.updater, cx))
            .child(render_about_section(&self.about));

        let mut root = div()
            .relative()
            .size_full()
            .p(px(m.spacing_lg + 8.0))
            .child(content);

        if let Some(popup) = render_incompatibility_popup(&self.about.compatibility) {
            root = root.child(popup);
        }

        root
    }
}
