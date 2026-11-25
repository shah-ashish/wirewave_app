pub mod camera_finder;
pub mod mf_vcam;
pub mod obs_vcam;

pub use camera_finder::{
    detect_compatible_camera_driver, detect_mf_virtual_camera, detect_obs_virtual_camera,
    open_recommended_camera_driver_page, preferred_camera_backend, set_preferred_camera_backend,
    CameraBackendPreference, CameraDriverKind, CompatibleCameraDriver,
    RECOMMENDED_CAMERA_DRIVER_URL,
};

/// Converts an RGBA8888 frame to NV12 (Y plane followed by interleaved UV plane).
pub fn rgba_to_nv12(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let w = (width & !1) as usize;
    let h = (height & !1) as usize;
    if w == 0 || h == 0 || rgba.len() < (width as usize) * (height as usize) * 4 {
        return Vec::new();
    }

    let y_len = w * h;
    let mut nv12 = vec![0u8; y_len + y_len / 2];
    let stride = (width as usize) * 4;

    for row in 0..h {
        let src_row = &rgba[row * stride..row * stride + w * 4];
        let y_row = &mut nv12[row * w..(row + 1) * w];
        for col in 0..w {
            let px = &src_row[col * 4..col * 4 + 4];
            let r = px[0] as i32;
            let g = px[1] as i32;
            let b = px[2] as i32;
            let y = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
            y_row[col] = y.clamp(0, 255) as u8;
        }
    }

    let uv_base = y_len;
    for row in (0..h).step_by(2) {
        let src_row = &rgba[row * stride..row * stride + w * 4];
        let uv_row = &mut nv12[uv_base + (row / 2) * w..uv_base + (row / 2 + 1) * w];
        for col in (0..w).step_by(2) {
            let px = &src_row[col * 4..col * 4 + 4];
            let r = px[0] as i32;
            let g = px[1] as i32;
            let b = px[2] as i32;
            let u = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
            let v = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;
            uv_row[col] = u.clamp(0, 255) as u8;
            uv_row[col + 1] = v.clamp(0, 255) as u8;
        }
    }

    nv12
}

#[cfg(windows)]
pub enum VirtualCameraSink {
    MediaFoundation(mf_vcam::MfVirtualCameraSink),
    Obs(obs_vcam::ObsVirtualCameraSink),
}

#[cfg(not(windows))]
pub enum VirtualCameraSink {
    None,
}

impl VirtualCameraSink {
    pub fn open_best(width: u32, height: u32) -> Result<(Self, String), String> {
        #[cfg(windows)]
        {
            let pref = preferred_camera_backend();
            if pref != CameraBackendPreference::Obs {
                match mf_vcam::MfVirtualCameraSink::start() {
                    Ok(mf) => {
                        return Ok((
                            Self::MediaFoundation(mf),
                            "WireWave Camera (Windows 11 Virtual Camera)".into(),
                        ));
                    }
                    Err(err) if pref == CameraBackendPreference::Windows11 => return Err(err),
                    Err(_) => {}
                }
            }
            if pref != CameraBackendPreference::Windows11
                && camera_finder::detect_obs_virtual_camera().is_some()
            {
                let obs = obs_vcam::ObsVirtualCameraSink::open_or_create(width, height)?;
                return Ok((Self::Obs(obs), "OBS Virtual Camera".into()));
            }
            Err("No compatible virtual camera driver available for selected backend".into())
        }
        #[cfg(not(windows))]
        {
            let _ = (width, height);
            Err("Virtual camera output is only supported on Windows".into())
        }
    }

    pub fn write_rgba(&self, width: u32, height: u32, rgba: &[u8]) {
        #[cfg(windows)]
        {
            let nv12 = rgba_to_nv12(width, height, rgba);
            if nv12.is_empty() {
                return;
            }
            match self {
                Self::MediaFoundation(mf) => mf.write_nv12(width & !1, height & !1, &nv12),
                Self::Obs(obs) => obs.write_nv12(width & !1, height & !1, &nv12),
            }
        }
        #[cfg(not(windows))]
        {
            let _ = (width, height, rgba);
        }
    }
}
