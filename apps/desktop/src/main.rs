#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod shell;

use assets::EmbeddedAssets;
use gpui::{
    px, size, App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowDecorations,
    WindowOptions,
};
use shell::AppShell;
use wirewave_core_compatibility::check_compatibility;
use wirewave_core_config::APP_CONFIG;
use wirewave_core_identity::{fetch_or_create_device_id, DeviceIdentity};
use wirewave_feature_settings::SettingsScreen;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Stage 1 background bootstrap:
    // 1. Verify OS compatibility (Windows 11 build >= 22000).
    let compatibility = check_compatibility();
    tracing::info!(
        os = %compatibility.detected_os,
        supported = compatibility.is_supported(),
        "Bootstrap compatibility check completed"
    );

    // 2. Fetch existing persistent device ID or generate and store a new one.
    let identity = fetch_or_create_device_id().unwrap_or_else(|err| {
        tracing::warn!(%err, "Falling back to ephemeral device identity");
        DeviceIdentity {
            schema_version: 1,
            device_id: wirewave_core_identity::generate_device_id(),
        }
    });
    tracing::info!(
        app_id = APP_CONFIG.app_id,
        update_app_id = APP_CONFIG.update_app_id,
        version = APP_CONFIG.version_name,
        "Bootstrap identity ready"
    );

    Application::new()
        .with_assets(EmbeddedAssets)
        .run(move |cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(960.0), px(620.0)), cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(680.0), px(480.0))),
                titlebar: Some(TitlebarOptions {
                    title: Some(APP_CONFIG.app_name.into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            };

            let compat = compatibility.clone();
            let id = identity.clone();

            cx.open_window(options, move |_window, cx| {
                let settings = cx.new(|_| SettingsScreen::new(&id, compat.clone(), None));
                cx.new(|_| AppShell::new(settings, compat.is_supported()))
            })
            .expect("open main WireWave window");

            cx.activate(true);
        });
}
