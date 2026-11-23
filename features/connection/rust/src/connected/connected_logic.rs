use wirewave_core_config::APP_CONFIG;
use wirewave_core_session::ActiveSession;
use wirewave_core_transport::TransportKind;
use wirewave_feature_speaker::SpeakerTileState;
use wirewave_feature_mic::MicTileState;

#[derive(Debug, Clone, Copy)]
pub struct ServiceTileSpec {
    pub name: &'static str,
    pub icon_path: &'static str,
    pub status_badge: &'static str,
}

#[derive(Debug, Clone)]
pub struct ConnectedViewState {
    pub device_name: String,
    pub transport: TransportKind,
    pub speaker: SpeakerTileState,
    pub mic: MicTileState,
    pub services: [ServiceTileSpec; 5],
}

impl ConnectedViewState {
    pub fn from_active(
        active: &ActiveSession,
        speaker: SpeakerTileState,
        mic: MicTileState,
    ) -> Self {
        let a = APP_CONFIG.assets;
        Self {
            device_name: active.device_name.clone(),
            transport: active.transport,
            speaker,
            mic,
            services: [
                ServiceTileSpec {
                    name: "Speaker",
                    icon_path: a.icon_speaker_svg,
                    status_badge: "Coming soon",
                },
                ServiceTileSpec {
                    name: "Mic",
                    icon_path: a.icon_mic_svg,
                    status_badge: "Coming soon",
                },
                ServiceTileSpec {
                    name: "Keyboard",
                    icon_path: a.icon_keyboard_svg,
                    status_badge: "Coming soon",
                },
                ServiceTileSpec {
                    name: "Mouse",
                    icon_path: a.icon_mouse_svg,
                    status_badge: "Coming soon",
                },
                ServiceTileSpec {
                    name: "Camera",
                    icon_path: a.icon_camera_svg,
                    status_badge: "Coming soon",
                },
            ],
        }
    }
}
