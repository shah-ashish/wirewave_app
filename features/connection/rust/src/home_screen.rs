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
use std::time::Duration;
use wirewave_core_session::{SessionState, SharedSession};

/// Desktop Home view switching cleanly between Idle, Pairing modal, and Connected states.
pub struct HomeScreen {
    session: SharedSession,
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

        Self { session }
    }
}

impl CanCancelPairing for HomeScreen {
    fn on_cancel_pairing(&mut self) {
        self.session.cancel_pairing();
    }
}

impl CanDisconnect for HomeScreen {
    fn on_disconnect(&mut self) {
        self.session.disconnect_active();
    }
}

impl Render for HomeScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.session.snapshot();

        match &snapshot.state {
            SessionState::Idle => div()
                .relative()
                .size_full()
                .child(render_idle_view(&IdleViewState::from_snapshot(&snapshot))),
            SessionState::Pairing(pending) => div()
                .relative()
                .size_full()
                .child(render_idle_view(&IdleViewState::from_snapshot(&snapshot)))
                .child(render_pairing_modal(&PairingModalState::from_pending(pending), cx)),
            SessionState::Connected(active) => div()
                .relative()
                .size_full()
                .child(render_connected_view(
                    &ConnectedViewState::from_active(active),
                    cx,
                )),
        }
    }
}
