use wirewave_core_config::APP_CONFIG;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityStatus {
    Supported,
    Unsupported { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub status: CompatibilityStatus,
    pub detected_os: String,
    pub minimum_required: &'static str,
}

impl CompatibilityReport {
    pub fn is_supported(&self) -> bool {
        matches!(self.status, CompatibilityStatus::Supported)
    }

    pub fn badge_label(&self) -> &'static str {
        match self.status {
            CompatibilityStatus::Supported => "Supported",
            CompatibilityStatus::Unsupported { .. } => "Unsupported",
        }
    }
}

/// Evaluates whether the host OS meets the minimum Windows 11 requirement.
pub fn check_compatibility() -> CompatibilityReport {
    let (major, build) = query_windows_version();
    evaluate_windows_build(major, build, APP_CONFIG.min_windows_build)
}

/// Pure evaluator separated for deterministic unit testing.
pub fn evaluate_windows_build(major: u32, build: u32, min_build: u32) -> CompatibilityReport {
    let detected_os = format!("Windows {major} (Build {build})");
    let status = match (major >= 10, build >= min_build) {
        (true, true) => CompatibilityStatus::Supported,
        _ => CompatibilityStatus::Unsupported {
            reason: format!(
                "{} requires {} or newer. Detected: {detected_os}.",
                APP_CONFIG.app_name, APP_CONFIG.min_os_label
            ),
        },
    };

    CompatibilityReport {
        status,
        detected_os,
        minimum_required: APP_CONFIG.min_os_label,
    }
}

#[cfg(windows)]
fn query_windows_version() -> (u32, u32) {
    // RtlGetVersion bypasses GetVersionEx manifest shimming and returns true Win11 build numbers.
    #[repr(C)]
    struct RtlOsVersionInfoW {
        dw_os_version_info_size: u32,
        dw_major_version: u32,
        dw_minor_version: u32,
        dw_build_number: u32,
        dw_platform_id: u32,
        sz_csd_version: [u16; 128],
    }

    #[link(name = "ntdll")]
    unsafe extern "system" {
        fn RtlGetVersion(version_info: *mut RtlOsVersionInfoW) -> i32;
    }

    let mut info = RtlOsVersionInfoW {
        dw_os_version_info_size: std::mem::size_of::<RtlOsVersionInfoW>() as u32,
        dw_major_version: 0,
        dw_minor_version: 0,
        dw_build_number: 0,
        dw_platform_id: 0,
        sz_csd_version: [0; 128],
    };

    let status = unsafe { RtlGetVersion(&mut info) };
    match status {
        0 => (info.dw_major_version, info.dw_build_number),
        _ => (0, 0),
    }
}

#[cfg(not(windows))]
fn query_windows_version() -> (u32, u32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_11_build_is_supported() {
        let report = evaluate_windows_build(10, 22631, 22000);
        assert!(report.is_supported());
        assert_eq!(report.badge_label(), "Supported");
    }

    #[test]
    fn windows_10_build_is_unsupported() {
        let report = evaluate_windows_build(10, 19045, 22000);
        assert!(!report.is_supported());
        assert_eq!(report.badge_label(), "Unsupported");
    }
}
