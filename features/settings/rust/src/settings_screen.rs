use crate::{
    about::{load_about_snapshot, render_about_section},
    compat_popup_ui::render_incompatibility_popup,
    connection::{
        connection_logic::ConnectionSettingsState,
        connection_ui::{render_connection_section, HasConnectionSettingsActions},
    },
    devices::{
        devices_logic::DeviceTab,
        devices_ui::{render_devices_section, HasDeviceListActions},
    },
    drivers::{
        drivers_logic::DriversState,
        drivers_ui::{render_drivers_section, HasDriverActions},
    },
    services::{
        services_logic::ServicesSettingsState,
        services_ui::{render_services_section, HasServicesSettingsActions},
    },
    update::{render_update_section, update_logic::HasUpdateController, UpdateController},
};
use gpui::{div, prelude::*, px, rgba, Context, FontWeight, Render, Window};
use wirewave_core_compatibility::CompatibilityReport;
use wirewave_core_config::THEME;
use wirewave_core_identity::DeviceIdentity;
use wirewave_core_session::{SessionState, SharedSession};
use wirewave_core_transport::TransportPriority;

/// Composes Update, Connection, Devices, and About sections plus the compatibility blocker.
pub struct SettingsScreen {
    identity: DeviceIdentity,
    compatibility: CompatibilityReport,
    session: SharedSession,
    updater: UpdateController,
    priority_dropdown_open: bool,
    device_tab: DeviceTab,
    settings_tab: SettingsTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingsTab {
    General,
    Drivers,
}

impl SettingsScreen {
    pub fn new(
        identity: &DeviceIdentity,
        compatibility: CompatibilityReport,
        session: SharedSession,
    ) -> Self {
        Self {
            identity: identity.clone(),
            compatibility,
            session,
            updater: UpdateController::default(),
            priority_dropdown_open: false,
            device_tab: DeviceTab::Connected,
            settings_tab: SettingsTab::General,
        }
    }
}

impl HasUpdateController for SettingsScreen {
    fn updater_mut(&mut self) -> &mut UpdateController {
        &mut self.updater
    }
}

impl HasConnectionSettingsActions for SettingsScreen {
    fn toggle_priority_dropdown(&mut self) {
        self.priority_dropdown_open = !self.priority_dropdown_open;
    }

    fn select_priority(&mut self, priority: TransportPriority) {
        self.session.set_priority(priority);
        self.priority_dropdown_open = false;
    }

    fn toggle_keep_alive(&mut self) {
        let current = self.session.snapshot().preferences.keep_alive_in_background;
        self.session.set_keep_alive(!current);
    }
}

impl HasServicesSettingsActions for SettingsScreen {
    fn toggle_speaker_permission(&mut self) {
        let current = self.session.snapshot().preferences.speaker_service_allowed;
        self.session.set_speaker_service_allowed(!current);
    }

    fn toggle_mic_permission(&mut self) {
        let current = self.session.snapshot().preferences.mic_service_allowed;
        self.session.set_mic_service_allowed(!current);
    }
}

impl HasDeviceListActions for SettingsScreen {
    fn select_device_tab(&mut self, tab: DeviceTab) {
        self.device_tab = tab;
    }

    fn disconnect_active_device(&mut self) {
        self.session.disconnect_active();
    }

    fn block_device_by_id(&mut self, device_id: String) {
        self.session.block_device(&device_id);
    }

    fn unblock_device_by_id(&mut self, device_id: String) {
        self.session.unblock_device(&device_id);
    }

    fn forget_device_by_id(&mut self, device_id: String) {
        self.session.forget_device(&device_id);
    }
}

impl HasDriverActions for SettingsScreen {
    fn refresh_drivers(&mut self) {
        self.session.refresh_mic_service_state();
    }

    fn download_recommended_driver(&mut self) {
        let _ = wirewave_feature_mic::open_recommended_driver_page();
    }
}

impl Render for SettingsScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let m = THEME.metrics;
        let snapshot = self.session.snapshot();

        let connected_phone = match &snapshot.state {
            SessionState::Connected(active) => Some(active.device_name.clone()),
            SessionState::Idle | SessionState::Pairing(_) => None,
        };
        let about =
            load_about_snapshot(&self.identity, self.compatibility.clone(), connected_phone);
        let conn_state =
            ConnectionSettingsState::from_prefs(snapshot.preferences, self.priority_dropdown_open);
        let services_state = ServicesSettingsState::from_prefs(snapshot.preferences);

        let general_active = self.settings_tab == SettingsTab::General;
        let drivers_active = self.settings_tab == SettingsTab::Drivers;
        let colors = THEME.colors;
        let tabs = div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .child(
                div()
                    .id("settings-general-tab")
                    .cursor_pointer()
                    .px(px(14.0))
                    .py(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgba(if general_active { colors.bg_nav_active } else { colors.bg_card }))
                    .text_color(rgba(if general_active { colors.accent_blue } else { colors.text_secondary }))
                    .font_weight(FontWeight::MEDIUM)
                    .on_click(cx.listener(|view, _, _, cx| {
                        view.settings_tab = SettingsTab::General;
                        cx.notify();
                    }))
                    .child("General"),
            )
            .child(
                div()
                    .id("settings-drivers-tab")
                    .cursor_pointer()
                    .px(px(14.0))
                    .py(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgba(if drivers_active { colors.bg_nav_active } else { colors.bg_card }))
                    .text_color(rgba(if drivers_active { colors.accent_blue } else { colors.text_secondary }))
                    .font_weight(FontWeight::MEDIUM)
                    .on_click(cx.listener(|view, _, _, cx| {
                        view.settings_tab = SettingsTab::Drivers;
                        cx.notify();
                    }))
                    .child("Drivers"),
            );

        let tab_content = match self.settings_tab {
            SettingsTab::General => div()
                .flex()
                .flex_col()
                .gap(px(m.spacing_lg))
                .child(render_update_section(&self.updater, cx))
                .child(render_connection_section(&conn_state, cx))
                .child(render_services_section(&services_state, cx))
                .child(render_devices_section(&snapshot, self.device_tab, cx))
                .child(render_about_section(&about)),
            SettingsTab::Drivers => div()
                .flex()
                .flex_col()
                .gap(px(m.spacing_lg))
                .child(render_drivers_section(&DriversState::detect(), cx)),
        };

        let content = div()
            .w_full()
            .max_w(px(m.content_max_width + 120.0))
            .flex()
            .flex_col()
            .gap(px(m.spacing_lg))
            .child(tabs)
            .child(tab_content);

        let mut root = div()
            .id("settings-scroll-root")
            .relative()
            .size_full()
            .overflow_y_scroll()
            .p(px(m.spacing_lg + 8.0))
            .child(content);

        if let Some(popup) = render_incompatibility_popup(&about.compatibility) {
            root = root.child(popup);
        }

        root
    }
}
