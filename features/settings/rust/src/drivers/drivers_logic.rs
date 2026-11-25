use wirewave_feature_camera::{
    detect_compatible_camera_driver, detect_mf_virtual_camera, detect_obs_virtual_camera,
    preferred_camera_backend, CameraBackendPreference, CompatibleCameraDriver,
};
use wirewave_feature_mic::{detect_compatible_mic_driver, CompatibleMicDriver};

#[derive(Debug, Clone)]
pub struct DriversState {
    pub installed: Option<CompatibleMicDriver>,
    pub camera_installed: Option<CompatibleCameraDriver>,
    pub mf_installed: Option<CompatibleCameraDriver>,
    pub obs_installed: Option<CompatibleCameraDriver>,
    pub camera_backend: CameraBackendPreference,
}

impl DriversState {
    pub fn detect() -> Self {
        Self {
            installed: detect_compatible_mic_driver(),
            camera_installed: detect_compatible_camera_driver(),
            mf_installed: detect_mf_virtual_camera(),
            obs_installed: detect_obs_virtual_camera(),
            camera_backend: preferred_camera_backend(),
        }
    }
}
