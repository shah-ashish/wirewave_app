use wirewave_feature_mic::{detect_compatible_mic_driver, CompatibleMicDriver};

#[derive(Debug, Clone)]
pub struct DriversState {
    pub installed: Option<CompatibleMicDriver>,
}

impl DriversState {
    pub fn detect() -> Self {
        Self {
            installed: detect_compatible_mic_driver(),
        }
    }
}
