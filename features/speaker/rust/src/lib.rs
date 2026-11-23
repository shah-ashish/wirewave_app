pub mod audio;
pub mod ui;

pub use audio::{
    capture_loop::{SpeakerService, SpeakerStatus},
    device_finder::{detect_active_playback_endpoint, select_playback_endpoint, AudioEndpointInfo, EndpointKind},
    laud::{LaudHeader, FRAMES_PER_PACKET, LAUD_HEADER_BYTES, TARGET_SAMPLE_RATE},
};
pub use ui::{
    speaker_logic::SpeakerTileState,
    speaker_ui::{render_speaker_tile, CanToggleSpeaker},
};
