use crate::{
    about::{load_about_snapshot, render_about_section},
    compat_popup_ui::render_incompatibility_popup,
    connection::{
        connection_logic::ConnectionSettingsState,
        connection_ui::{HasConnectionSettingsActions, render_connection_section},
    },
    devices::devices_ui::{HasDeviceListActions, render_devices_section},
    drivers::{
        drivers_logic::DriversState,
        drivers_ui::{HasDriverActions, render_drivers_section},
    },
    services::{
        services_logic::ServicesSettingsState,
        services_ui::{HasServicesSettingsActions, render_services_section},
    },
};
use gpui::{Context, FontWeight, Render, Window, div, prelude::*, px, rgba};
use wirewave_core_compatibility::CompatibilityReport;
use wirewave_core_config::THEME;
use wirewave_core_identity::DeviceIdentity;
use wirewave_core_session::{SessionState, SharedSession};
use wirewave_core_transport::TransportPriority;

/// Composes unified single-page Settings (Connection, Services, Drivers, Devices, About).
pub struct SettingsScreen {
    identity: DeviceIdentity,
    compatibility: CompatibilityReport,
    session: SharedSession,
    priority_dropdown_open: bool,
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
            priority_dropdown_open: false,
        }
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

    fn toggle_mouse_permission(&mut self) {
        let current = self.session.snapshot().preferences.mouse_service_allowed;
        self.session.set_mouse_service_allowed(!current);
    }
}

impl HasDeviceListActions for SettingsScreen {
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
        let c = THEME.colors;
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

        let page_header = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_size(px(26.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(c.text_primary))
                    .child("Settings"),
            )
            .child(
                div()
                    .text_size(px(THEME.typography.font_body))
                    .text_color(rgba(c.text_secondary))
                    .child("Connection, permissions, drivers, and paired devices"),
            );

        let content = div()
            .w_full()
            .max_w(px(m.content_max_width + 260.0))
            .flex()
            .flex_col()
            .gap(px(m.spacing_lg))
            .child(page_header)
            .child(render_connection_section(&conn_state, cx))
            .child(render_services_section(&services_state, cx))
            .child(render_drivers_section(&DriversState::detect(), cx))
            .child(render_devices_section(&snapshot, cx))
            .child(render_about_section(&about));

        let mut root = div()
            .id("settings-scroll-root")
            .relative()
            .size_full()
            .overflow_y_scroll()
            .p(px(m.spacing_lg))
            .flex()
            .flex_col()
            .items_center()
            .child(content);

        if let Some(popup) = render_incompatibility_popup(&about.compatibility) {
            root = root.child(popup);
        }

        root
    }
}
