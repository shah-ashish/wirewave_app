use gpui::{AssetSource, Result, SharedString};
use std::borrow::Cow;
use wirewave_core_config::APP_CONFIG;

/// Embeds all brand & UI icons declared in `config/wirewave.config.json` into the binary.
pub struct EmbeddedAssets;

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let a = APP_CONFIG.assets;
        let bytes: Option<&'static [u8]> = match path {
            p if p == a.icon_desktop_mark => {
                Some(include_bytes!("../../../assets/brand/wirewave_mark.png"))
            }
            p if p == a.icon_settings_svg => {
                Some(include_bytes!("../../../assets/brand/icons/settings.svg"))
            }
            p if p == a.icon_monitor_svg => {
                Some(include_bytes!("../../../assets/brand/icons/monitor.svg"))
            }
            p if p == a.icon_check_svg => {
                Some(include_bytes!("../../../assets/brand/icons/check.svg"))
            }
            p if p == a.icon_home_svg => {
                Some(include_bytes!("../../../assets/brand/icons/home.svg"))
            }
            p if p == a.icon_wifi_svg => {
                Some(include_bytes!("../../../assets/brand/icons/wifi.svg"))
            }
            p if p == a.icon_bluetooth_svg => {
                Some(include_bytes!("../../../assets/brand/icons/bluetooth.svg"))
            }
            p if p == a.icon_phone_svg => {
                Some(include_bytes!("../../../assets/brand/icons/phone.svg"))
            }
            p if p == a.icon_speaker_svg => {
                Some(include_bytes!("../../../assets/brand/icons/speaker.svg"))
            }
            p if p == a.icon_mic_svg => Some(include_bytes!("../../../assets/brand/icons/mic.svg")),
            p if p == a.icon_keyboard_svg => {
                Some(include_bytes!("../../../assets/brand/icons/keyboard.svg"))
            }
            p if p == a.icon_mouse_svg => {
                Some(include_bytes!("../../../assets/brand/icons/mouse.svg"))
            }
            "assets/brand/icons/window-minimize.svg" => Some(include_bytes!(
                "../../../assets/brand/icons/window-minimize.svg"
            )),
            "assets/brand/icons/window-maximize.svg" => Some(include_bytes!(
                "../../../assets/brand/icons/window-maximize.svg"
            )),
            "assets/brand/icons/window-close.svg" => Some(include_bytes!(
                "../../../assets/brand/icons/window-close.svg"
            )),
            _ => None,
        };
        Ok(bytes.map(Cow::Borrowed))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        let a = APP_CONFIG.assets;
        Ok(vec![
            a.icon_desktop_mark.into(),
            a.icon_settings_svg.into(),
            a.icon_monitor_svg.into(),
            a.icon_check_svg.into(),
            a.icon_home_svg.into(),
            a.icon_wifi_svg.into(),
            a.icon_bluetooth_svg.into(),
            a.icon_phone_svg.into(),
            a.icon_speaker_svg.into(),
            a.icon_mic_svg.into(),
            a.icon_keyboard_svg.into(),
            a.icon_mouse_svg.into(),
            "assets/brand/icons/window-minimize.svg".into(),
            "assets/brand/icons/window-maximize.svg".into(),
            "assets/brand/icons/window-close.svg".into(),
        ])
    }
}
