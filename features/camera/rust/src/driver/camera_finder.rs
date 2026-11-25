use std::sync::atomic::{AtomicU8, Ordering};

pub const RECOMMENDED_CAMERA_DRIVER_URL: &str = "https://obsproject.com/";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CameraBackendPreference {
    Auto = 0,
    Windows11 = 1,
    Obs = 2,
}

impl CameraBackendPreference {
    fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Windows11,
            2 => Self::Obs,
            _ => Self::Auto,
        }
    }
}

static PREFERRED_BACKEND: AtomicU8 = AtomicU8::new(0);

pub fn preferred_camera_backend() -> CameraBackendPreference {
    CameraBackendPreference::from_u8(PREFERRED_BACKEND.load(Ordering::Acquire))
}

pub fn set_preferred_camera_backend(pref: CameraBackendPreference) {
    PREFERRED_BACKEND.store(pref as u8, Ordering::Release);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CameraDriverKind {
    WindowsMediaFoundation,
    ObsVirtualCamera,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibleCameraDriver {
    pub name: String,
    pub kind: CameraDriverKind,
}

pub fn open_recommended_camera_driver_page() -> Result<(), String> {
    open::that(RECOMMENDED_CAMERA_DRIVER_URL).map_err(|error| error.to_string())
}

#[cfg(windows)]
pub fn detect_mf_virtual_camera() -> Option<CompatibleCameraDriver> {
    if crate::driver::mf_vcam::find_vcam_dll().is_some() {
        Some(CompatibleCameraDriver {
            name: "WireWave Camera (Windows 11 Virtual Camera)".into(),
            kind: CameraDriverKind::WindowsMediaFoundation,
        })
    } else {
        None
    }
}

#[cfg(not(windows))]
pub fn detect_mf_virtual_camera() -> Option<CompatibleCameraDriver> {
    None
}

#[cfg(windows)]
pub fn detect_obs_virtual_camera() -> Option<CompatibleCameraDriver> {
    use std::path::Path;
    use windows::core::w;
    use windows::Win32::System::Registry::{RegCloseKey, RegOpenKeyExW, HKEY_LOCAL_MACHINE, KEY_READ};

    let clsid_64 = w!("SOFTWARE\\Classes\\CLSID\\{A3FCE0F5-3493-419F-958A-ABA1250EC20B}");
    let clsid_32 =
        w!("SOFTWARE\\Classes\\WOW6432Node\\CLSID\\{A3FCE0F5-3493-419F-958A-ABA1250EC20B}");

    let is_registered = unsafe {
        let mut key = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, clsid_64, 0, KEY_READ, &mut key).is_ok() {
            let _ = RegCloseKey(key);
            true
        } else if RegOpenKeyExW(HKEY_LOCAL_MACHINE, clsid_32, 0, KEY_READ, &mut key).is_ok() {
            let _ = RegCloseKey(key);
            true
        } else {
            false
        }
    };

    if is_registered {
        return Some(CompatibleCameraDriver {
            name: "OBS Virtual Camera".into(),
            kind: CameraDriverKind::ObsVirtualCamera,
        });
    }

    let standard_obs_paths = [
        r"C:\Program Files\obs-studio\data\obs-plugins\win-dshow\obs-virtualcam-module64.dll",
        r"C:\Program Files (x86)\obs-studio\data\obs-plugins\win-dshow\obs-virtualcam-module32.dll",
    ];

    for path in &standard_obs_paths {
        if Path::new(path).exists() {
            return Some(CompatibleCameraDriver {
                name: "OBS Virtual Camera".into(),
                kind: CameraDriverKind::ObsVirtualCamera,
            });
        }
    }

    None
}

#[cfg(not(windows))]
pub fn detect_obs_virtual_camera() -> Option<CompatibleCameraDriver> {
    None
}

#[cfg(windows)]
pub fn detect_compatible_camera_driver() -> Option<CompatibleCameraDriver> {
    match preferred_camera_backend() {
        CameraBackendPreference::Auto => {
            detect_mf_virtual_camera().or_else(detect_obs_virtual_camera)
        }
        CameraBackendPreference::Windows11 => detect_mf_virtual_camera(),
        CameraBackendPreference::Obs => detect_obs_virtual_camera(),
    }
}

#[cfg(not(windows))]
pub fn detect_compatible_camera_driver() -> Option<CompatibleCameraDriver> {
    None
}
