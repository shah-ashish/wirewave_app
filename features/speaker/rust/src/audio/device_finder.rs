/// Classification of the Windows playback endpoint chosen for WASAPI loopback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointKind {
    VirtualSpeaker,
    SystemDefault,
}

impl EndpointKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::VirtualSpeaker => "Virtual Speaker",
            Self::SystemDefault => "Default Speaker",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioEndpointInfo {
    pub id: String,
    pub name: String,
    pub kind: EndpointKind,
}

/// Pure endpoint selector:
/// 1. Use the playback device currently selected by Windows.
/// 2. If Windows temporarily has no default, use the first active render endpoint.
pub fn select_playback_endpoint(
    default_endpoint: Option<&AudioEndpointInfo>,
    active_endpoints: &[AudioEndpointInfo],
) -> Option<AudioEndpointInfo> {
    default_endpoint
        .or_else(|| active_endpoints.first())
        .map(|endpoint| AudioEndpointInfo {
            kind: EndpointKind::SystemDefault,
            ..endpoint.clone()
        })
}

/// Enumerates active Windows render devices and resolves the optimal endpoint.
#[cfg(windows)]
pub fn detect_active_playback_endpoint() -> Option<AudioEndpointInfo> {
    use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
    use windows::Win32::Media::Audio::{
        eMultimedia, eRender, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
        DEVICE_STATE_ACTIVE,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
    };

    unsafe fn read_endpoint(dev: &IMMDevice) -> Option<AudioEndpointInfo> {
        let id = unsafe { dev.GetId().ok()?.to_string().ok()? };
        let store = unsafe { dev.OpenPropertyStore(STGM_READ).ok()? };
        let prop = unsafe { store.GetValue(&PKEY_Device_FriendlyName).ok()? };
        let name = prop.to_string();
        Some(AudioEndpointInfo {
            id,
            name,
            kind: EndpointKind::SystemDefault,
        })
    }

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()?;

        let default_ep = enumerator
            .GetDefaultAudioEndpoint(eRender, eMultimedia)
            .ok()
            .and_then(|d| read_endpoint(&d));

        let mut active = Vec::new();
        if let Ok(collection) = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE) {
            let count = collection.GetCount().unwrap_or(0);
            for i in 0..count {
                if let Ok(dev) = collection.Item(i) {
                    if let Some(info) = read_endpoint(&dev) {
                        active.push(info);
                    }
                }
            }
        }

        select_playback_endpoint(default_ep.as_ref(), &active)
    }
}

#[cfg(not(windows))]
pub fn detect_active_playback_endpoint() -> Option<AudioEndpointInfo> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ep(id: &str, name: &str) -> AudioEndpointInfo {
        AudioEndpointInfo {
            id: id.into(),
            name: name.into(),
            kind: EndpointKind::SystemDefault,
        }
    }

    #[test]
    fn selects_windows_default_even_when_virtual_speaker_is_present() {
        let def = ep("1", "Speakers (Realtek High Definition Audio)");
        let virt = ep("2", "VB-Audio Virtual Cable");

        let chosen =
            select_playback_endpoint(Some(&def), &[def.clone(), virt.clone()]).expect("some");
        assert_eq!(chosen.id, "1");
        assert_eq!(chosen.kind, EndpointKind::SystemDefault);

        let fallback = select_playback_endpoint(Some(&def), &[def.clone()]).expect("some");
        assert_eq!(fallback.id, "1");
        assert_eq!(fallback.kind, EndpointKind::SystemDefault);
    }

    #[test]
    fn falls_back_to_first_active_endpoint_when_windows_has_no_default() {
        let first = ep("1", "Speakers (USB Audio)");
        let second = ep("2", "HDMI Output");

        let chosen = select_playback_endpoint(None, &[first.clone(), second]).expect("some");

        assert_eq!(chosen.id, first.id);
        assert_eq!(chosen.kind, EndpointKind::SystemDefault);
    }
}
