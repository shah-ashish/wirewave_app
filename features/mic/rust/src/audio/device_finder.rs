pub const RECOMMENDED_DRIVER_URL: &str = "https://vb-audio.com/Cable/";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibleMicDriver {
    pub endpoint_id: String,
    pub name: String,
}

/// Selects an installed virtual render endpoint that exposes its audio through
/// a paired recording endpoint. Known compatible drivers are preferred in a
/// deterministic order; VB-CABLE is only recommended when none is installed.
pub fn select_compatible_mic_driver(
    endpoints: &[(String, String)],
) -> Option<CompatibleMicDriver> {
    const COMPATIBLE_NAMES: &[&str] = &[
        "cable input",
        "voicemeeter input",
        "voicemeeter aux input",
        "voicemeeter vaio3 input",
        "virtual audio cable",
    ];

    COMPATIBLE_NAMES.iter().find_map(|needle| {
        endpoints
            .iter()
            .find(|(_, name)| name.to_ascii_lowercase().contains(needle))
            .map(|(endpoint_id, name)| CompatibleMicDriver {
                endpoint_id: endpoint_id.clone(),
                name: name.clone(),
            })
    })
}

pub fn open_recommended_driver_page() -> Result<(), String> {
    open::that(RECOMMENDED_DRIVER_URL).map_err(|error| error.to_string())
}

#[cfg(windows)]
pub fn detect_compatible_mic_driver() -> Option<CompatibleMicDriver> {
    use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
    use windows::Win32::Media::Audio::{
        eRender, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
    };

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()?;
        let collection = enumerator
            .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
            .ok()?;
        let mut endpoints = Vec::new();
        for index in 0..collection.GetCount().unwrap_or(0) {
            let Ok(device) = collection.Item(index) else {
                continue;
            };
            let Some(endpoint_id) = device.GetId().ok().and_then(|id| id.to_string().ok()) else {
                continue;
            };
            let Some(name) = device
                .OpenPropertyStore(STGM_READ)
                .ok()
                .and_then(|store| store.GetValue(&PKEY_Device_FriendlyName).ok())
                .map(|value| value.to_string())
            else {
                continue;
            };
            endpoints.push((endpoint_id, name));
        }
        select_compatible_mic_driver(&endpoints)
    }
}

#[cfg(not(windows))]
pub fn detect_compatible_mic_driver() -> Option<CompatibleMicDriver> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_an_installed_compatible_driver_before_recommending_vb_cable() {
        let endpoints = vec![
            ("physical".into(), "Speakers (Realtek)".into()),
            ("virtual".into(), "VoiceMeeter Input (VB-Audio VoiceMeeter VAIO)".into()),
        ];

        let selected = select_compatible_mic_driver(&endpoints).expect("compatible driver");
        assert_eq!(selected.endpoint_id, "virtual");
        assert!(selected.name.contains("VoiceMeeter"));
    }

    #[test]
    fn does_not_treat_a_physical_speaker_as_a_mic_driver() {
        let endpoints = vec![("physical".into(), "Speakers (Realtek)".into())];
        assert!(select_compatible_mic_driver(&endpoints).is_none());
    }
}
