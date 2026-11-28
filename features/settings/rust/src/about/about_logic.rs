use wirewave_core_compatibility::CompatibilityReport;
use wirewave_core_config::APP_CONFIG;
use wirewave_core_identity::DeviceIdentity;

/// Immutable data snapshot rendered by `about_ui`.
#[derive(Debug, Clone)]
pub struct AboutSnapshot {
    pub app_name: &'static str,
    pub version_name: &'static str,
    pub device_name: String,
    pub connected_phone: String,
    pub app_id: &'static str,
    pub device_id: String,
    pub compatibility: CompatibilityReport,
}

/// Collects local host metadata, persistent identity, and OS compatibility status.
pub fn load_about_snapshot(
    identity: &DeviceIdentity,
    compatibility: CompatibilityReport,
    connected_phone: Option<String>,
) -> AboutSnapshot {
    let device_name = hostname::get()
        .ok()
        .and_then(|os| os.into_string().ok())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "Windows PC".to_string());

    AboutSnapshot {
        app_name: APP_CONFIG.app_name,
        version_name: APP_CONFIG.version_name,
        device_name,
        connected_phone: connected_phone.unwrap_or_else(|| "Not connected".to_string()),
        app_id: APP_CONFIG.app_id,
        device_id: identity.device_id.clone(),
        compatibility,
    }
}
