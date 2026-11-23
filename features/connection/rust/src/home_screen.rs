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
use wirewave_feature_speaker::{SpeakerService, SpeakerTileState};
use wirewave_feature_mic::{MicService, MicTileState};

/// Desktop Home view switching cleanly between Idle, Pairing modal, and Connected states.
pub struct HomeScreen {
    session: SharedSession,
    speaker_service: Arc<SpeakerService>,
    speaker_state: SpeakerTileState,
    mic_service: Arc<MicService>,
    mic_state: MicTileState,
}

impl HomeScreen {
    pub fn new(
        session: SharedSession,
        speaker_service: Arc<SpeakerService>,
        mic_service: Arc<MicService>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(Duration::from_millis(350))
                .await;
            let keep_running = this.update(cx, |_, cx| cx.notify()).is_ok();
            if !keep_running {
                break;
            }
        })
        .detach();

        let speaker_state = SpeakerTileState::from_status(&speaker_service.status());
        let mic_state = MicTileState::from_status(&mic_service.status());

        Self {
            session,
            speaker_service,
            speaker_state,
            mic_service,
            mic_state,
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
        self.mic_service.stop();
        self.session.disconnect_active();
    }
}

impl Render for HomeScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.session.snapshot();
        self.speaker_state = SpeakerTileState::from_status(&self.speaker_service.status());
        self.mic_state = MicTileState::from_status(&self.mic_service.status());

        match &snapshot.state {
            SessionState::Idle => {
                if self.speaker_service.is_running() {
                    self.speaker_service.stop();
                }
                if self.mic_service.is_running() {
                    self.mic_service.stop();
                }
                div()
                    .relative()
                    .size_full()
                    .child(render_idle_view(&IdleViewState::from_snapshot(&snapshot)))
            }
            SessionState::Pairing(pending) => {
                if self.speaker_service.is_running() {
                    self.speaker_service.stop();
                }
                if self.mic_service.is_running() {
                    self.mic_service.stop();
                }
                div()
                    .relative()
                    .size_full()
                    .child(render_idle_view(&IdleViewState::from_snapshot(&snapshot)))
                    .child(render_pairing_modal(
                        &PairingModalState::from_pending(pending),
                        cx,
                    ))
            }
            SessionState::Connected(active) => {
                div().relative().size_full().child(render_connected_view(
                    &ConnectedViewState::from_active(
                        active,
                        self.speaker_state.clone(),
                        self.mic_state.clone(),
                    ),
                    cx,
                ))
            }
        }
    }
}
