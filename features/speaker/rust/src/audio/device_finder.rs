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

const VIRTUAL_SPEAKER_KEYWORDS: &[&str] = &[
    "wirewave virtual",
    "virtual speaker",
    "vb-audio",
    "voicemeeter",
    "virtual audio",
];

/// Pure endpoint selector:
/// 1. If the user's currently selected Windows default playback device is a virtual speaker, use it.
/// 2. Otherwise, if any active virtual playback device is present, prefer it.
/// 3. Otherwise, fall back to the currently selected Windows default playback device.
pub fn select_playback_endpoint(
    default_endpoint: Option<&AudioEndpointInfo>,
    active_endpoints: &[AudioEndpointInfo],
) -> Option<AudioEndpointInfo> {
    if let Some(def) = default_endpoint {
        if is_virtual_speaker_name(&def.name) {
            return Some(AudioEndpointInfo {
                kind: EndpointKind::VirtualSpeaker,
                ..def.clone()
            });
        }
    }

    if let Some(virt) = active_endpoints
        .iter()
        .find(|ep| is_virtual_speaker_name(&ep.name))
    {
        return Some(AudioEndpointInfo {
            kind: EndpointKind::VirtualSpeaker,
            ..virt.clone()
        });
    }

    default_endpoint.map(|def| AudioEndpointInfo {
        kind: EndpointKind::SystemDefault,
        ..def.clone()
    })
}

pub fn is_virtual_speaker_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    VIRTUAL_SPEAKER_KEYWORDS.iter().any(|kw| lower.contains(kw))
}

/// Enumerates active Windows render devices and resolves the optimal endpoint.
#[cfg(windows)]
pub fn detect_active_playback_endpoint() -> Option<AudioEndpointInfo> {
    use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
    use windows::Win32::Media::Audio::{
        eConsole, eRender, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
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
            .GetDefaultAudioEndpoint(eRender, eConsole)
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
    fn prefers_virtual_speaker_when_present_else_default() {
        let def = ep("1", "Speakers (Realtek High Definition Audio)");
        let virt = ep("2", "VB-Audio Virtual Cable");

        let chosen = select_playback_endpoint(Some(&def), &[def.clone(), virt.clone()]).expect("some");
        assert_eq!(chosen.id, "2");
        assert_eq!(chosen.kind, EndpointKind::VirtualSpeaker);

        let fallback = select_playback_endpoint(Some(&def), &[def.clone()]).expect("some");
        assert_eq!(fallback.id, "1");
        assert_eq!(fallback.kind, EndpointKind::SystemDefault);
    }
}
