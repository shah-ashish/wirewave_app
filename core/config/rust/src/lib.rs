pub mod theme;

pub use theme::{ColorTokens, MetricTokens, ThemeTokens, TypographyTokens};

/// Controls whether updates run in-app (`Dev`) or redirect to the OS store (`Store`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildChannel {
    Dev,
    Store,
}

#[derive(Debug, Clone, Copy)]
pub struct ConnectionConfig {
    pub tcp_port: u16,
    pub udp_discovery_port: u16,
    pub bt_service_uuid: &'static str,
    pub protocol_domain: &'static str,
    pub otp_ttl_secs: u64,
    pub otp_max_attempts: u8,
    pub heartbeat_interval_secs: u64,
    pub heartbeat_timeout_secs: u64,
    pub default_priority: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct AssetPaths {
    pub icon_desktop_ico: &'static str,
    pub icon_desktop_mark: &'static str,
    pub icon_settings_svg: &'static str,
    pub icon_monitor_svg: &'static str,
    pub icon_check_svg: &'static str,
    pub icon_home_svg: &'static str,
    pub icon_wifi_svg: &'static str,
    pub icon_bluetooth_svg: &'static str,
    pub icon_phone_svg: &'static str,
    pub icon_speaker_svg: &'static str,
    pub icon_mic_svg: &'static str,
    pub icon_keyboard_svg: &'static str,
    pub icon_mouse_svg: &'static str,
    pub icon_camera_svg: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct AppConfig {
    pub app_name: &'static str,
    pub app_id: &'static str,
    pub update_app_id: &'static str,
    pub version_name: &'static str,
    pub version_code: u32,
    pub update_endpoint: &'static str,
    pub store_url: &'static str,
    pub min_windows_build: u32,
    pub min_os_label: &'static str,
    pub build_channel: BuildChannel,
    pub connection: ConnectionConfig,
    pub assets: AssetPaths,
}

include!(concat!(env!("OUT_DIR"), "/generated_config.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_id_and_update_app_id_are_distinct() {
        assert_ne!(APP_CONFIG.app_id, APP_CONFIG.update_app_id);
        assert!(!APP_CONFIG.app_name.is_empty());
        assert_eq!(APP_CONFIG.version_name, "1.1.4");
    }
}
