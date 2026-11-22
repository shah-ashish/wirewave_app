use crate::{
    components::surface_card,
    update::update_logic::{HasUpdateController, UpdateAction, UpdateController, UpdatePhase},
};
use gpui::{div, prelude::*, px, relative, rgba, Context, Div, FontWeight, Render};
use wirewave_core_config::{APP_CONFIG, THEME};

/// Renders the Version + Check/Download/Progress/Install card matching the desktop design.
pub fn render_update_section<V: Render + HasUpdateController>(
    controller: &UpdateController,
    cx: &mut Context<V>,
) -> Div {
    let c = THEME.colors;
    let t = THEME.typography;
    let m = THEME.metrics;

    let version_label = format!("Version {}", APP_CONFIG.version_name);

    let trailing = match controller.primary_action() {
        Some((label, action)) => div()
            .id("update-action-btn")
            .cursor_pointer()
            .text_size(px(t.font_body))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(c.accent_blue))
            .hover(|style| style.text_color(rgba(c.accent_blue_hover)))
            .on_click(cx.listener(move |view, _event, _window, cx| {
                dispatch_action(view, action, cx);
            }))
            .child(label)
            .into_any_element(),
        None => {
            let status_text = match &controller.phase {
                UpdatePhase::Checking => "Checking...".to_string(),
                UpdatePhase::Downloading { percent, .. } => format!("Downloading {percent}%"),
                _ => String::new(),
            };
            div()
                .text_size(px(t.font_body))
                .text_color(rgba(c.text_secondary))
                .child(status_text)
                .into_any_element()
        }
    };

    let header_row = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .min_h(px(m.row_height + 6.0))
        .px(px(m.spacing_md))
        .child(
            div()
                .text_size(px(t.font_card_title))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(c.text_primary))
                .child(version_label),
        )
        .child(trailing);

    let mut card = surface_card().child(header_row);

    match &controller.phase {
        UpdatePhase::Available(release) => {
            card = card.child(detail_banner(
                format!("Version {} is ready to download.", release.version_name),
                c.text_secondary,
            ));
        }
        UpdatePhase::Downloading {
            version_name,
            percent,
        } => {
            card = card.child(progress_footer(version_name, *percent));
        }
        UpdatePhase::ReadyToInstall { version_name, .. } => {
            card = card.child(detail_banner(
                format!("Version {version_name} downloaded and verified. Click Install to apply."),
                c.status_ok_fg,
            ));
        }
        UpdatePhase::Failed { message, .. } => {
            card = card.child(detail_banner(message.clone(), c.status_err_fg));
        }
        UpdatePhase::Idle | UpdatePhase::Checking | UpdatePhase::UpToDate => {}
    }

    card
}

fn dispatch_action<V: Render + HasUpdateController>(
    view: &mut V,
    action: UpdateAction,
    cx: &mut Context<V>,
) {
    match action {
        UpdateAction::Check => view.updater_mut().trigger_check(cx),
        UpdateAction::Download => view.updater_mut().trigger_download(cx),
        UpdateAction::Install => {
            view.updater_mut().trigger_install();
            cx.notify();
        }
        UpdateAction::OpenStore => {
            view.updater_mut().open_store();
            cx.notify();
        }
        UpdateAction::Retry => view.updater_mut().retry(cx),
    }
}

fn detail_banner(text: String, color: u32) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    div()
        .border_t_1()
        .border_color(rgba(c.divider))
        .px(px(m.spacing_md))
        .py(px(m.spacing_sm))
        .text_size(px(THEME.typography.font_badge))
        .text_color(rgba(color))
        .child(text)
}

fn progress_footer(version_name: &str, percent: u8) -> Div {
    let c = THEME.colors;
    let m = THEME.metrics;
    let fraction = (percent as f32 / 100.0).clamp(0.0, 1.0);

    div()
        .border_t_1()
        .border_color(rgba(c.divider))
        .px(px(m.spacing_md))
        .py(px(m.spacing_sm))
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(
            div()
                .flex()
                .flex_row()
                .justify_between()
                .text_size(px(THEME.typography.font_badge))
                .text_color(rgba(c.text_secondary))
                .child(format!("Downloading v{version_name}"))
                .child(format!("{percent}%")),
        )
        .child(
            div()
                .w_full()
                .h(px(6.0))
                .rounded(px(m.radius_pill))
                .bg(rgba(c.border_subtle))
                .overflow_hidden()
                .child(
                    div()
                        .h_full()
                        .w(relative(fraction))
                        .rounded(px(m.radius_pill))
                        .bg(rgba(c.accent_blue)),
                ),
        )
}
