#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod shell;

use assets::EmbeddedAssets;
use gpui::{
    px, size, App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds,
    WindowDecorations, WindowOptions,
};
use shell::AppShell;
use wirewave_core_compatibility::CompatibilityChecker;
use wirewave_core_config::APP_CONFIG;
use wirewave_core_crypto::{fetch_or_create_keypair, IdentityKeypair};
use wirewave_core_identity::{DeviceIdentity, DeviceIdentityStore};
use wirewave_core_session::{FeatureRegistry, SessionManager};
use wirewave_feature_connection::{ActiveServiceSlots, HomeScreen};
use wirewave_feature_settings::SettingsScreen;
use wirewave_feature_speaker::{render_speaker_tile, SpeakerService, SpeakerTileState};
use wirewave_feature_mic::{render_mic_tile, MicService, MicTileState};
use wirewave_feature_camera::{render_camera_tile, CameraPreviewScreen, CameraService, CameraTileState};

/// Injected implementation of active service slots for Desktop Home, composed at the application root.
struct DesktopServiceSlots {
    speaker: std::sync::Arc<SpeakerService>,
    mic: std::sync::Arc<MicService>,
    mouse: std::sync::Arc<wirewave_feature_mouse::MouseService>,
    camera: std::sync::Arc<CameraService>,
}

impl ActiveServiceSlots for DesktopServiceSlots {
    fn render_speaker_tile(&self) -> gpui::AnyElement {
        use gpui::IntoElement;
        let state = SpeakerTileState::from_status(&self.speaker.status());
        render_speaker_tile(&state).into_any_element()
    }

    fn render_mic_tile(&self) -> gpui::AnyElement {
        use gpui::IntoElement;
        let state = MicTileState::from_status(&self.mic.status());
        render_mic_tile(&state).into_any_element()
    }

    fn render_mouse_tile(&self) -> gpui::AnyElement {
        use gpui::IntoElement;
        let is_active = self.mouse.is_running();
        let state = wirewave_feature_mouse::MouseTileState::new(is_active);
        wirewave_feature_mouse::render_mouse_tile(&state).into_any_element()
    }

    fn render_camera_tile(&self) -> gpui::AnyElement {
        use gpui::IntoElement;
        let state = CameraTileState::from_status(&self.camera.status());
        render_camera_tile(&state).into_any_element()
    }

    fn stop_all(&self) {
        self.speaker.stop();
        self.mic.stop();
        self.mouse.stop();
        self.camera.stop();
    }

    fn is_any_running(&self) -> bool {
        self.speaker.is_running() || self.mic.is_running() || self.mouse.is_running() || self.camera.is_running()
    }
}

/// Result of single-instance mutual exclusion check.
#[cfg(windows)]
enum SingleInstance {
    Acquired(windows::Win32::Foundation::HANDLE),
    AlreadyRunning,
    Unavailable,
}

/// Acquires a Windows named mutex to enforce a single running instance.
///
/// Returns the mutex `HANDLE` which must be kept alive for the process lifetime
/// (dropping it releases the mutex and allows a new instance to start).
/// Returns `SingleInstance::AlreadyRunning` if another instance is already running — the caller must exit.
#[cfg(windows)]
fn acquire_single_instance() -> SingleInstance {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::ERROR_ALREADY_EXISTS;
    use windows::Win32::System::Threading::CreateMutexW;

    let name: Vec<u16> = "Local\\WireWaveDesktopSingleInstance\0"
        .encode_utf16()
        .collect();

    // SAFETY: name is a valid null-terminated UTF-16 string; no security attributes needed.
    let handle = unsafe { CreateMutexW(None, true, PCWSTR(name.as_ptr())) };
    match handle {
        Ok(h) => {
            // If the mutex already existed the OS returns ERROR_ALREADY_EXISTS via GetLastError.
            let last = unsafe { windows::Win32::Foundation::GetLastError() };
            if last == ERROR_ALREADY_EXISTS {
                tracing::warn!(
                    "WireWave is already running — refusing to start a second instance."
                );
                SingleInstance::AlreadyRunning
            } else {
                SingleInstance::Acquired(h)
            }
        }
        Err(err) => {
            // Cannot create mutex (unusual). Log and continue — better than a silent blank window.
            tracing::warn!(%err, "Could not create single-instance mutex; proceeding anyway.");
            SingleInstance::Unavailable
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Enforce a single running instance using a Windows named mutex.
    // If another instance already holds the mutex (ports 3030/3032 are in use),
    // log and exit cleanly rather than launching a broken second window.
    #[cfg(windows)]
    let _single_instance_guard = match acquire_single_instance() {
        SingleInstance::Acquired(guard) => Some(guard),
        SingleInstance::Unavailable => None,
        SingleInstance::AlreadyRunning => {
            tracing::error!(
                "Another WireWave instance is already running. \
                 Close it before starting a new one (or the installed version \
                 before running a debug build)."
            );
            std::process::exit(1);
        }
    };

    // Stage 1 background bootstrap:
    let compatibility_checker = CompatibilityChecker::new();
    let compatibility = compatibility_checker.check();
    tracing::info!(
        os = %compatibility.detected_os,
        supported = compatibility.is_supported(),
        "Bootstrap compatibility check completed"
    );

    let identity_store = DeviceIdentityStore::default_store().ok();
    let identity = identity_store
        .as_ref()
        .and_then(|store| store.fetch_or_create().ok())
        .unwrap_or_else(|| {
            tracing::warn!("Falling back to ephemeral device identity");
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

    // Stage 2 secure session bootstrap (Ed25519 keypair + background Wi-Fi/discovery runtime):
    let keypair = fetch_or_create_keypair().unwrap_or_else(|err| {
        tracing::warn!(%err, "Falling back to ephemeral identity keypair");
        IdentityKeypair::generate()
    });

    let speaker_service = std::sync::Arc::new(SpeakerService::new());
    let mic_service = std::sync::Arc::new(MicService::new());
    let mouse_service = std::sync::Arc::new(wirewave_feature_mouse::MouseService::new());
    let camera_service = std::sync::Arc::new(CameraService::new());

    // Polymorphic feature registry:
    let feature_registry = std::sync::Arc::new(FeatureRegistry::new());
    feature_registry.register(speaker_service.clone());
    feature_registry.register(mic_service.clone());
    feature_registry.register(mouse_service.clone());
    feature_registry.register(camera_service.clone());

    let session = SessionManager::new(
        identity.clone(),
        keypair,
        feature_registry,
    );
    if compatibility.is_supported() {
        session.start_background_services();
    }

    Application::new()
        .with_assets(EmbeddedAssets)
        .run(move |cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(720.0), px(500.0))),
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
            let shared_session = session.clone();
            let cam_service = camera_service.clone();
            let slots = std::sync::Arc::new(DesktopServiceSlots {
                speaker: speaker_service.clone(),
                mic: mic_service.clone(),
                mouse: mouse_service.clone(),
                camera: camera_service.clone(),
            });

            cx.open_window(options, move |_window, cx| {
                let home = cx.new(|cx| HomeScreen::new(
                    shared_session.clone(),
                    slots,
                    cx,
                ));
                let camera = cx.new(|_| CameraPreviewScreen::new(
                    shared_session.clone(),
                    cam_service.clone(),
                ));
                let settings =
                    cx.new(|_| SettingsScreen::new(&id, compat.clone(), shared_session.clone()));
                cx.new(|_| AppShell::new(shared_session.clone(), home, camera, settings))
            })
            .expect("open main WireWave window");

            cx.activate(true);
        });
}
