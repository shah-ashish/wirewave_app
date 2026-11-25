pub mod driver;
pub mod service;
pub mod ui;

pub use driver::{
    detect_compatible_camera_driver, detect_mf_virtual_camera, detect_obs_virtual_camera,
    open_recommended_camera_driver_page, preferred_camera_backend, set_preferred_camera_backend,
    CameraBackendPreference, CameraDriverKind, CompatibleCameraDriver, VirtualCameraSink,
    RECOMMENDED_CAMERA_DRIVER_URL,
};
pub use service::{CameraService, CameraStatus, DecodedFrame};
pub use ui::{render_camera_tile, CameraPreviewScreen, CameraTileState};
