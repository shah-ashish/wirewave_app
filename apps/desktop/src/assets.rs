use gpui::{AssetSource, Result, SharedString};
use std::borrow::Cow;
use wirewave_core_config::APP_CONFIG;

/// Embeds brand icons declared in `config/wirewave.config.json` directly into the binary.
pub struct EmbeddedAssets;

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let bytes: Option<&'static [u8]> = match path {
            p if p == APP_CONFIG.assets.icon_desktop_mark => {
                Some(include_bytes!("../../../assets/brand/wirewave_mark.png"))
            }
            p if p == APP_CONFIG.assets.icon_settings_svg => {
                Some(include_bytes!("../../../assets/brand/icons/settings.svg"))
            }
            p if p == APP_CONFIG.assets.icon_monitor_svg => {
                Some(include_bytes!("../../../assets/brand/icons/monitor.svg"))
            }
            p if p == APP_CONFIG.assets.icon_check_svg => {
                Some(include_bytes!("../../../assets/brand/icons/check.svg"))
            }
            _ => None,
        };
        Ok(bytes.map(Cow::Borrowed))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![
            APP_CONFIG.assets.icon_desktop_mark.into(),
            APP_CONFIG.assets.icon_settings_svg.into(),
            APP_CONFIG.assets.icon_monitor_svg.into(),
            APP_CONFIG.assets.icon_check_svg.into(),
        ])
    }
}
