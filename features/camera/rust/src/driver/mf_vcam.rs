//! Windows 11 `MFCreateVirtualCamera` sink + shared-memory NV12 writer.

#[cfg(windows)]
mod inner {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use windows::core::{w, PCWSTR};
    use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    use windows::Win32::Media::MediaFoundation::{
        IMFVirtualCamera, MFCreateVirtualCamera, MFShutdown, MFStartup,
        MFVirtualCameraAccess_CurrentUser, MFVirtualCameraLifetime_Session,
        MFVirtualCameraType_SoftwareCameraSource, MFSTARTUP_NOSOCKET, MF_VERSION,
    };
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};
    use windows::Win32::System::Memory::{
        CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_WRITE,
        MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE,
    };
    use windows::Win32::System::Registry::{
        RegCloseKey, RegCreateKeyExW, RegSetValueExW, HKEY_CURRENT_USER, KEY_WRITE,
        REG_OPTION_NON_VOLATILE, REG_SZ,
    };

    const CLSID_STR: &str = "{B7F6E9A2-4C1D-4E8B-9A3F-1D2E3F4A5B6C}";
    const SHM_NAME: PCWSTR = w!("Local\\WireWaveVirtualCamFrame");
    const SHM_MAGIC: u32 = 0x5757_5643;
    const MAX_W: usize = 1920;
    const MAX_H: usize = 1080;
    const MAX_NV12_BYTES: usize = MAX_W * MAX_H * 3 / 2;

    #[repr(C)]
    struct ShmHeader {
        magic: u32,
        seq: u64,
        width: u32,
        height: u32,
        data_len: u32,
        _reserved: u32,
    }

    pub fn find_vcam_dll() -> Option<PathBuf> {
        let exe = std::env::current_exe().ok()?;
        let dir = exe.parent()?;
        let candidate = dir.join("wirewave_vcam.dll");
        if candidate.exists() {
            return Some(candidate);
        }
        None
    }

    fn ensure_com_dll_registered(dll_path: &PathBuf) -> Result<(), String> {
        let subkey: Vec<u16> = format!("Software\\Classes\\CLSID\\{CLSID_STR}\\InprocServer32\0")
            .encode_utf16()
            .collect();
        let dll_wide: Vec<u16> = format!("{}\0", dll_path.display()).encode_utf16().collect();
        let threading: Vec<u16> = "Both\0".encode_utf16().collect();

        unsafe {
            let mut hkey = Default::default();
            RegCreateKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey.as_ptr()),
                0,
                None,
                REG_OPTION_NON_VOLATILE,
                KEY_WRITE,
                None,
                &mut hkey,
                None,
            )
            .ok()
            .map_err(|e| format!("RegCreateKeyExW failed: {e}"))?;

            let dll_bytes =
                std::slice::from_raw_parts(dll_wide.as_ptr() as *const u8, dll_wide.len() * 2);
            let _ = RegSetValueExW(hkey, None, 0, REG_SZ, Some(dll_bytes));
            let thr_bytes =
                std::slice::from_raw_parts(threading.as_ptr() as *const u8, threading.len() * 2);
            let _ = RegSetValueExW(hkey, w!("ThreadingModel"), 0, REG_SZ, Some(thr_bytes));
            let _ = RegCloseKey(hkey);
        }
        Ok(())
    }

    pub struct MfVirtualCameraSink {
        vcam: IMFVirtualCamera,
        mapping: HANDLE,
        view: MEMORY_MAPPED_VIEW_ADDRESS,
        seq: AtomicU64,
        com_initialized: bool,
    }

    unsafe impl Send for MfVirtualCameraSink {}
    unsafe impl Sync for MfVirtualCameraSink {}

    impl MfVirtualCameraSink {
        pub fn start() -> Result<Self, String> {
            let dll_path = find_vcam_dll()
                .ok_or_else(|| "wirewave_vcam.dll not found next to executable".to_string())?;
            ensure_com_dll_registered(&dll_path)?;

            let total_size = (std::mem::size_of::<ShmHeader>() + MAX_NV12_BYTES) as u32;
            let mapping = unsafe {
                CreateFileMappingW(
                    INVALID_HANDLE_VALUE,
                    None,
                    PAGE_READWRITE,
                    0,
                    total_size,
                    SHM_NAME,
                )
            }
            .map_err(|e| format!("CreateFileMappingW failed: {e}"))?;

            let view = unsafe { MapViewOfFile(mapping, FILE_MAP_WRITE, 0, 0, 0) };
            if view.Value.is_null() {
                unsafe { let _ = CloseHandle(mapping); }
                return Err("MapViewOfFile failed".into());
            }

            let com_hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
            let com_initialized = com_hr.is_ok();

            unsafe {
                MFStartup(MF_VERSION, MFSTARTUP_NOSOCKET)
                    .map_err(|e| format!("MFStartup failed: {e}"))?;
            }

            let clsid_wide: Vec<u16> = format!("{CLSID_STR}\0").encode_utf16().collect();
            let vcam = unsafe {
                MFCreateVirtualCamera(
                    MFVirtualCameraType_SoftwareCameraSource,
                    MFVirtualCameraLifetime_Session,
                    MFVirtualCameraAccess_CurrentUser,
                    w!("WireWave Camera"),
                    PCWSTR(clsid_wide.as_ptr()),
                    None,
                )
            }
            .map_err(|e| {
                unsafe {
                    let _ = MFShutdown();
                    if com_initialized {
                        CoUninitialize();
                    }
                    let _ = UnmapViewOfFile(view);
                    let _ = CloseHandle(mapping);
                }
                format!("MFCreateVirtualCamera failed: {e}")
            })?;

            if let Err(e) = unsafe { vcam.Start(None) } {
                unsafe {
                    let _ = vcam.Remove();
                    let _ = MFShutdown();
                    if com_initialized {
                        CoUninitialize();
                    }
                    let _ = UnmapViewOfFile(view);
                    let _ = CloseHandle(mapping);
                }
                return Err(format!("IMFVirtualCamera::Start failed: {e}"));
            }

            Ok(Self {
                vcam,
                mapping,
                view,
                seq: AtomicU64::new(1),
                com_initialized,
            })
        }

        pub fn write_nv12(&self, width: u32, height: u32, nv12: &[u8]) {
            if width == 0 || height == 0 || nv12.len() > MAX_NV12_BYTES {
                return;
            }
            let seq = self.seq.fetch_add(1, Ordering::AcqRel);
            let hdr = ShmHeader {
                magic: SHM_MAGIC,
                seq,
                width,
                height,
                data_len: nv12.len() as u32,
                _reserved: 0,
            };
            unsafe {
                let payload_ptr = (self.view.Value as *mut u8).add(std::mem::size_of::<ShmHeader>());
                std::ptr::copy_nonoverlapping(nv12.as_ptr(), payload_ptr, nv12.len());
                std::ptr::write_volatile(self.view.Value as *mut ShmHeader, hdr);
            }
        }
    }

    impl Drop for MfVirtualCameraSink {
        fn drop(&mut self) {
            unsafe {
                let _ = self.vcam.Stop();
                let _ = self.vcam.Remove();
                let _ = MFShutdown();
                if self.com_initialized {
                    CoUninitialize();
                }
                let _ = UnmapViewOfFile(self.view);
                let _ = CloseHandle(self.mapping);
            }
        }
    }
}

#[cfg(windows)]
pub use inner::{find_vcam_dll, MfVirtualCameraSink};
