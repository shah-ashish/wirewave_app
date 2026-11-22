use crate::{
    about::AboutSnapshot,
    components::{section_label, setting_row, status_pill, surface_card, value_text},
};
use gpui::{div, prelude::*, px, Div};
use wirewave_core_config::THEME;

/// Renders the ABOUT section and grouped metadata card matching the desktop design.
pub fn render_about_section(snapshot: &AboutSnapshot) -> Div {
    let m = THEME.metrics;

    let rows: [(&str, Div); 4] = [
        ("App name", value_text(snapshot.app_name, false)),
        ("Device name", value_text(snapshot.device_name.clone(), false)),
        (
            "Connected phone",
            value_text(snapshot.connected_phone.clone(), false),
        ),
        ("App ID", value_text(snapshot.app_id, true)),
    ];

    let mut card = surface_card();
    for (idx, (label, value)) in rows.into_iter().enumerate() {
        card = card.child(setting_row(label, value, idx > 0));
    }

    card = card.child(setting_row(
        "Compatibility",
        status_pill(
            snapshot.compatibility.is_supported(),
            snapshot.compatibility.badge_label(),
        ),
        true,
    ));

    div()
        .flex()
        .flex_col()
        .gap(px(m.spacing_sm))
        .child(section_label("ABOUT"))
        .child(card)
}
