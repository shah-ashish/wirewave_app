//! OBS Virtual Camera shared-memory ring-buffer writer (Windows 10 & 11 fallback).

#[cfg(windows)]
mod inner {
    use std::sync::atomic::{AtomicU64, Ordering};
    use windows::core::{w, PCWSTR};
    use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    use windows::Win32::System::Memory::{
        CreateFileMappingW, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_WRITE,
        MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE,
    };

    const OBS_SHM_NAME: PCWSTR = w!("OBSVirtualCamVideo");
    const QUEUE_SLOTS: usize = 3;
    const ALIGN_SIZE: usize = 32;

    #[repr(C)]
    struct QueueHeader {
        write_idx: u32,
        read_idx: u32,
        state: u32,
        offsets: [u32; QUEUE_SLOTS],
        type_val: u32,
        cx: u32,
        cy: u32,
        interval: u64,
        reserved: [u32; 8],
    }

    #[repr(C)]
    struct FrameHeader {
        timestamp: u64,
        linesize: [u32; 4],
        reserved: [u32; 2],
    }

    fn align_up(v: usize) -> usize {
        (v + ALIGN_SIZE - 1) & !(ALIGN_SIZE - 1)
    }

    pub struct ObsVirtualCameraSink {
        mapping: HANDLE,
        view: MEMORY_MAPPED_VIEW_ADDRESS,
        width: u32,
        height: u32,
        slot_offsets: [usize; QUEUE_SLOTS],
        frame_counter: AtomicU64,
    }

    unsafe impl Send for ObsVirtualCameraSink {}
    unsafe impl Sync for ObsVirtualCameraSink {}

    impl ObsVirtualCameraSink {
        pub fn open_or_create(width: u32, height: u32) -> Result<Self, String> {
            let hdr_size = align_up(std::mem::size_of::<QueueHeader>());
            let frame_hdr_size = align_up(std::mem::size_of::<FrameHeader>());
            let nv12_size = (width as usize) * (height as usize) * 3 / 2;
            let slot_size = align_up(frame_hdr_size + nv12_size);
            let total_size = hdr_size + slot_size * QUEUE_SLOTS;

            let mapping = unsafe {
                OpenFileMappingW(FILE_MAP_WRITE.0, false, OBS_SHM_NAME).or_else(|_| {
                    CreateFileMappingW(
                        INVALID_HANDLE_VALUE,
                        None,
                        PAGE_READWRITE,
                        0,
                        total_size as u32,
                        OBS_SHM_NAME,
                    )
                })
            }
            .map_err(|e| format!("Failed to open OBS Virtual Camera shared memory: {e}"))?;

            let view = unsafe { MapViewOfFile(mapping, FILE_MAP_WRITE, 0, 0, 0) };
            if view.Value.is_null() {
                unsafe { let _ = CloseHandle(mapping); }
                return Err("MapViewOfFile failed for OBSVirtualCamVideo".into());
            }

            let slot_offsets = [
                hdr_size,
                hdr_size + slot_size,
                hdr_size + slot_size * 2,
            ];

            let q_hdr = QueueHeader {
                write_idx: 0,
                read_idx: 0,
                state: 2, // SHARED_QUEUE_STATE_READY
                offsets: [
                    slot_offsets[0] as u32,
                    slot_offsets[1] as u32,
                    slot_offsets[2] as u32,
                ],
                type_val: 0, // Video
                cx: width,
                cy: height,
                interval: 333_333, // ~30 fps in 100ns units
                reserved: [0; 8],
            };
            unsafe {
                std::ptr::write_volatile(view.Value as *mut QueueHeader, q_hdr);
            }

            Ok(Self {
                mapping,
                view,
                width,
                height,
                slot_offsets,
                frame_counter: AtomicU64::new(0),
            })
        }

        pub fn write_nv12(&self, width: u32, height: u32, nv12: &[u8]) {
            if width != self.width || height != self.height {
                return;
            }
            let expected = (width as usize) * (height as usize) * 3 / 2;
            if nv12.len() < expected {
                return;
            }

            let idx = self.frame_counter.fetch_add(1, Ordering::AcqRel);
            let slot = (idx as usize) % QUEUE_SLOTS;
            let offset = self.slot_offsets[slot];
            let frame_hdr_size = align_up(std::mem::size_of::<FrameHeader>());

            let f_hdr = FrameHeader {
                timestamp: idx * 33_333_333,
                linesize: [width, width, 0, 0],
                reserved: [0; 2],
            };

            unsafe {
                let slot_ptr = (self.view.Value as *mut u8).add(offset);
                std::ptr::write_volatile(slot_ptr as *mut FrameHeader, f_hdr);
                std::ptr::copy_nonoverlapping(
                    nv12.as_ptr(),
                    slot_ptr.add(frame_hdr_size),
                    expected,
                );
                let q_ptr = self.view.Value as *mut QueueHeader;
                std::ptr::write_volatile(&raw mut (*q_ptr).write_idx, idx as u32);
            }
        }
    }

    impl Drop for ObsVirtualCameraSink {
        fn drop(&mut self) {
            unsafe {
                let q_ptr = self.view.Value as *mut QueueHeader;
                std::ptr::write_volatile(&raw mut (*q_ptr).state, 3); // SHARED_QUEUE_STATE_STOPPING
                let _ = UnmapViewOfFile(self.view);
                let _ = CloseHandle(self.mapping);
            }
        }
    }
}

#[cfg(windows)]
pub use inner::ObsVirtualCameraSink;
