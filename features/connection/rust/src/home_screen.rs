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
    service_slots::ActiveServiceSlots,
};
use gpui::{div, prelude::*, Context, Render, Window};
use std::{sync::Arc, time::Duration};
use wirewave_core_session::{SessionState, SharedSession};

/// Desktop Home view switching cleanly between Idle, Pairing modal, and Connected states.
pub struct HomeScreen {
    session: SharedSession,
    service_slots: Arc<dyn ActiveServiceSlots>,
}

impl HomeScreen {
    pub fn new(
        session: SharedSession,
        service_slots: Arc<dyn ActiveServiceSlots>,
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

        Self {
            session,
            service_slots,
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
        self.service_slots.stop_all();
        self.session.disconnect_active();
    }
}

impl Render for HomeScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.session.snapshot();

        match &snapshot.state {
            SessionState::Idle => {
                if self.service_slots.is_any_running() {
                    self.service_slots.stop_all();
                }
                div()
                    .relative()
                    .size_full()
                    .child(render_idle_view(&IdleViewState::from_snapshot(&snapshot)))
            }
            SessionState::Pairing(pending) => {
                if self.service_slots.is_any_running() {
                    self.service_slots.stop_all();
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
                    &ConnectedViewState::from_active(active),
                    self.service_slots.as_ref(),
                    cx,
                ))
            }
        }
    }
}
