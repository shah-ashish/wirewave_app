pub const RECOMMENDED_CAMERA_DRIVER_URL: &str = "https://obsproject.com/";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibleCameraDriver {
    pub name: String,
}

pub fn open_recommended_camera_driver_page() -> Result<(), String> {
    open::that(RECOMMENDED_CAMERA_DRIVER_URL).map_err(|error| error.to_string())
}

#[cfg(windows)]
pub fn detect_compatible_camera_driver() -> Option<CompatibleCameraDriver> {
    use std::path::Path;
    use windows::core::w;
    use windows::Win32::System::Registry::{HKEY_LOCAL_MACHINE, KEY_READ, RegOpenKeyExW};

    // 1. Registry checks for OBS Virtual Camera CLSID
    let clsid_64 = w!("SOFTWARE\\Classes\\CLSID\\{A3FCE0F5-3493-419F-958A-ABA1250EC20B}");
    let clsid_32 = w!("SOFTWARE\\Classes\\WOW6432Node\\CLSID\\{A3FCE0F5-3493-419F-958A-ABA1250EC20B}");

    let is_registered = unsafe {
        let mut key = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, clsid_64, 0, KEY_READ, &mut key).is_ok() {
            true
        } else if RegOpenKeyExW(HKEY_LOCAL_MACHINE, clsid_32, 0, KEY_READ, &mut key).is_ok() {
            true
        } else {
            false
        }
    };

    if is_registered {
        return Some(CompatibleCameraDriver {
            name: "OBS Virtual Camera".into(),
        });
    }

    // 2. Direct binary installation fallback check
    let standard_obs_paths = [
        r"C:\Program Files\obs-studio\data\obs-plugins\win-dshow\obs-virtualcam-module64.dll",
        r"C:\Program Files (x86)\obs-studio\data\obs-plugins\win-dshow\obs-virtualcam-module32.dll",
    ];

    for path in &standard_obs_paths {
        if Path::new(path).exists() {
            return Some(CompatibleCameraDriver {
                name: "OBS Virtual Camera".into(),
            });
        }
    }

    None
}

#[cfg(not(windows))]
pub fn detect_compatible_camera_driver() -> Option<CompatibleCameraDriver> {
    None
}
