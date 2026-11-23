use crate::{
    connected::{
        connected_logic::ConnectedViewState,
        connected_ui::{render_connected_view, CanDisconnect},
    },
    idle::{idle_logic::IdleViewState, idle_ui::render_idle_view},
    pairing::{
        pairing_logic::PairingModalState,
        pairing_ui::{render_pairing_modal, CanCancelPairing},
    },
};
use gpui::{div, prelude::*, Context, Render, Window};
use std::{sync::Arc, time::Duration};
use wirewave_core_session::{SessionState, SharedSession};
use wirewave_feature_speaker::{
    detect_active_playback_endpoint, CanToggleSpeaker, SpeakerService, SpeakerTileState,
};

/// Desktop Home view switching cleanly between Idle, Pairing modal, and Connected states.
pub struct HomeScreen {
    session: SharedSession,
    speaker_service: Arc<SpeakerService>,
    speaker_state: SpeakerTileState,
}

impl HomeScreen {
    pub fn new(session: SharedSession, cx: &mut Context<Self>) -> Self {
        // Periodic refresh keeps OTP countdown and incoming connection changes live.
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(350))
                    .await;
                let keep_running = this.update(cx, |_, cx| cx.notify()).is_ok();
                if !keep_running {
                    break;
                }
            }
        })
        .detach();

        let speaker_service = Arc::new(SpeakerService::new());
        let speaker_state = SpeakerTileState::from_status(&speaker_service.status());

        Self {
            session,
            speaker_service,
            speaker_state,
        }
    }
}

impl CanCancelPairing for HomeScreen {
    fn on_cancel_pairing(&mut self) {
        self.session.cancel_pairing();
    }
}

impl CanDisconnect for HomeScreen {
    fn on_disconnect(&mut self) {
        self.speaker_service.stop();
        self.speaker_state = SpeakerTileState::from_status(&self.speaker_service.status());
        self.session.disconnect_active();
    }
}

impl CanToggleSpeaker for HomeScreen {
    fn on_toggle_speaker(&mut self) {
        if self.speaker_service.is_running() || self.speaker_state.is_on {
            self.speaker_service.stop();
            self.speaker_state = SpeakerTileState::from_status(&self.speaker_service.status());
        } else {
            let ep_hint = detect_active_playback_endpoint()
                .map(|ep| format!("{} · {}", ep.kind.label(), ep.name))
                .or_else(|| Some("Default Speaker".to_string()));

            self.speaker_state = SpeakerTileState {
                is_on: true,
                subtitle: "On".to_string(),
                endpoint_hint: ep_hint,
            };
        }
    }
}

impl Render for HomeScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.session.snapshot();

        match &snapshot.state {
            SessionState::Idle => {
                if self.speaker_service.is_running() || self.speaker_state.is_on {
                    self.speaker_service.stop();
                    self.speaker_state = SpeakerTileState::from_status(&self.speaker_service.status());
                }
                div()
                    .relative()
                    .size_full()
                    .child(render_idle_view(&IdleViewState::from_snapshot(&snapshot)))
            }
            SessionState::Pairing(pending) => {
                if self.speaker_service.is_running() || self.speaker_state.is_on {
                    self.speaker_service.stop();
                    self.speaker_state = SpeakerTileState::from_status(&self.speaker_service.status());
                }
                div()
                    .relative()
                    .size_full()
                    .child(render_idle_view(&IdleViewState::from_snapshot(&snapshot)))
                    .child(render_pairing_modal(&PairingModalState::from_pending(pending), cx))
            }
            SessionState::Connected(active) => div()
                .relative()
                .size_full()
                .child(render_connected_view(
                    &ConnectedViewState::from_active(active, self.speaker_state.clone()),
                    cx,
                )),
        }
    }
}
