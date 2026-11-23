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
    update::{render_update_section, update_logic::HasUpdateController, UpdateController},
};
use gpui::{div, prelude::*, px, Context, Render, Window};
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

impl Render for SettingsScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let m = THEME.metrics;
        let snapshot = self.session.snapshot();

        let connected_phone = match &snapshot.state {
            SessionState::Connected(active) => Some(active.device_name.clone()),
            SessionState::Idle | SessionState::Pairing(_) => None,
        };
        let about = load_about_snapshot(&self.identity, self.compatibility.clone(), connected_phone);
        let conn_state =
            ConnectionSettingsState::from_prefs(snapshot.preferences, self.priority_dropdown_open);

        let content = div()
            .w_full()
            .max_w(px(m.content_max_width + 120.0))
            .flex()
            .flex_col()
            .gap(px(m.spacing_lg))
            .child(render_update_section(&self.updater, cx))
            .child(render_connection_section(&conn_state, cx))
            .child(render_devices_section(&snapshot, self.device_tab, cx))
            .child(render_about_section(&about));

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
