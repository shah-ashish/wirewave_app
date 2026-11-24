use wirewave_feature_camera::{detect_compatible_camera_driver, CompatibleCameraDriver};
use wirewave_feature_mic::{detect_compatible_mic_driver, CompatibleMicDriver};

#[derive(Debug, Clone)]
pub struct DriversState {
    pub installed: Option<CompatibleMicDriver>,
    pub camera_installed: Option<CompatibleCameraDriver>,
}

impl DriversState {
    pub fn detect() -> Self {
        Self {
            installed: detect_compatible_mic_driver(),
            camera_installed: detect_compatible_camera_driver(),
        }
    }
}
