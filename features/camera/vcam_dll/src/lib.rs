//! Windows 11 Media Foundation virtual camera COM source DLL (`wirewave_vcam.dll`).
//!
//! Reads NV12 video frames written by WireWave Desktop from `Local\WireWaveVirtualCamFrame`
//! shared memory and serves them to Windows Camera Frame Server via `IMFMediaSource`.

#[cfg(windows)]
mod win_vcam {
    use std::ffi::c_void;
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::sync::Mutex;
    use windows::core::{
        implement, w, Error, IUnknown, Interface, GUID, HRESULT, PCWSTR, PROPVARIANT,
    };
    use windows::Win32::Foundation::{
        CloseHandle, BOOL, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_INVALIDARG, E_NOINTERFACE,
        E_POINTER, HANDLE, HMODULE, MAX_PATH, S_FALSE, S_OK,
    };
    use windows::Win32::Media::MediaFoundation::{
        IMFAsyncCallback, IMFAsyncResult, IMFMediaBuffer, IMFMediaEvent,
        IMFMediaEventGenerator, IMFMediaEventGenerator_Impl, IMFMediaEventQueue, IMFMediaSource,
        IMFMediaSource_Impl, IMFMediaStream, IMFMediaStream_Impl, IMFPresentationDescriptor,
        IMFSample, IMFStreamDescriptor, MFCreateEventQueue,
        MFCreateMediaType, MFCreateMemoryBuffer, MFCreatePresentationDescriptor, MFCreateSample,
        MFCreateStreamDescriptor, MFMediaType_Video, MFVideoFormat_NV12,
        MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS, MFMEDIASOURCE_IS_LIVE,
        MF_E_SHUTDOWN, MF_MT_DEFAULT_STRIDE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE,
        MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE,
        MEMediaSample, MENewStream, MEStreamStarted, MEStreamStopped,
        MEUpdatedStream,
    };
    use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
    use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
    use windows::Win32::System::Memory::{
        MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ,
        MEMORY_MAPPED_VIEW_ADDRESS,
    };
    use windows::Win32::System::Registry::{
        RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW, HKEY_CURRENT_USER,
        KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ,
    };

    /// `{B7F6E9A2-4C1D-4E8B-9A3F-1D2E3F4A5B6C}`
    pub const CLSID_WIREWAVE_VCAM: GUID = GUID::from_u128(0xB7F6E9A2_4C1D_4E8B_9A3F_1D2E3F4A5B6C);
    const CLSID_STR: &str = "{B7F6E9A2-4C1D-4E8B-9A3F-1D2E3F4A5B6C}";
    const SHM_NAME: PCWSTR = w!("Local\\WireWaveVirtualCamFrame");
    const SHM_MAGIC: u32 = 0x5757_5643;
    const DEFAULT_WIDTH: u32 = 1280;
    const DEFAULT_HEIGHT: u32 = 720;
    const DEFAULT_FPS: u32 = 30;

    static DLL_REF_COUNT: AtomicU32 = AtomicU32::new(0);
    static mut MODULE_HANDLE: HMODULE = HMODULE(std::ptr::null_mut());

    #[repr(C)]
    struct ShmHeader {
        magic: u32,
        seq: u64,
        width: u32,
        height: u32,
        data_len: u32,
        _reserved: u32,
    }

    struct ShmReader {
        mapping: HANDLE,
        view: MEMORY_MAPPED_VIEW_ADDRESS,
    }

    unsafe impl Send for ShmReader {}
    unsafe impl Sync for ShmReader {}

    impl ShmReader {
        fn open() -> Option<Self> {
            let mapping = unsafe { OpenFileMappingW(FILE_MAP_READ.0, false, SHM_NAME) }.ok()?;
            let view = unsafe { MapViewOfFile(mapping, FILE_MAP_READ, 0, 0, 0) };
            if view.Value.is_null() {
                unsafe { let _ = CloseHandle(mapping); }
                return None;
            }
            Some(Self { mapping, view })
        }

        fn read_latest_nv12(&self) -> Option<(u32, u32, Vec<u8>)> {
            let hdr_ptr = self.view.Value as *const ShmHeader;
            let hdr = unsafe { std::ptr::read_volatile(hdr_ptr) };
            if hdr.magic != SHM_MAGIC || hdr.width == 0 || hdr.height == 0 || hdr.data_len == 0 {
                return None;
            }
            let expected = (hdr.width as usize) * (hdr.height as usize) * 3 / 2;
            if (hdr.data_len as usize) < expected || expected > 1920 * 1080 * 3 / 2 {
                return None;
            }
            let payload_ptr = unsafe { (self.view.Value as *const u8).add(std::mem::size_of::<ShmHeader>()) };
            let mut out = vec![0u8; expected];
            unsafe {
                std::ptr::copy_nonoverlapping(payload_ptr, out.as_mut_ptr(), expected);
            }
            Some((hdr.width, hdr.height, out))
        }
    }

    impl Drop for ShmReader {
        fn drop(&mut self) {
            unsafe {
                let _ = UnmapViewOfFile(self.view);
                let _ = CloseHandle(self.mapping);
            }
        }
    }

    fn pack_u64(hi: u32, lo: u32) -> u64 {
        ((hi as u64) << 32) | (lo as u64)
    }

    fn create_nv12_media_type(width: u32, height: u32) -> windows::core::Result<windows::Win32::Media::MediaFoundation::IMFMediaType> {
        unsafe {
            let mt = MFCreateMediaType()?;
            mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
            mt.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_NV12)?;
            mt.SetUINT64(&MF_MT_FRAME_SIZE, pack_u64(width, height))?;
            mt.SetUINT64(&MF_MT_FRAME_RATE, pack_u64(DEFAULT_FPS, 1))?;
            mt.SetUINT32(&MF_MT_INTERLACE_MODE, 2)?; // MFVideoInterlace_Progressive
            mt.SetUINT32(&MF_MT_DEFAULT_STRIDE, width)?;
            Ok(mt)
        }
    }

    #[implement(IMFMediaStream, IMFMediaEventGenerator)]
    struct WireWaveMediaStream {
        source: IMFMediaSource,
        descriptor: IMFStreamDescriptor,
        events: IMFMediaEventQueue,
        shutdown: AtomicBool,
        shm: Mutex<Option<ShmReader>>,
        frame_index: AtomicU32,
    }

    impl WireWaveMediaStream {
        fn new(source: IMFMediaSource, descriptor: IMFStreamDescriptor) -> windows::core::Result<Self> {
            let events = unsafe { MFCreateEventQueue()? };
            Ok(Self {
                source,
                descriptor,
                events,
                shutdown: AtomicBool::new(false),
                shm: Mutex::new(ShmReader::open()),
                frame_index: AtomicU32::new(0),
            })
        }

        fn ensure_shm_frame(&self) -> (u32, u32, Vec<u8>) {
            if let Ok(mut guard) = self.shm.lock() {
                if guard.is_none() {
                    *guard = ShmReader::open();
                }
                if let Some(reader) = guard.as_ref() {
                    if let Some(frame) = reader.read_latest_nv12() {
                        return frame;
                    }
                }
            }
            // Black NV12 fallback frame so Frame Server never stalls before first packet arrives.
            let y_len = (DEFAULT_WIDTH * DEFAULT_HEIGHT) as usize;
            let uv_len = y_len / 2;
            let mut black = vec![16u8; y_len + uv_len];
            black[y_len..].fill(128);
            (DEFAULT_WIDTH, DEFAULT_HEIGHT, black)
        }
    }

    impl IMFMediaEventGenerator_Impl for WireWaveMediaStream_Impl {
        fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> windows::core::Result<IMFMediaEvent> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.GetEvent(dwflags.0) }
        }

        fn BeginGetEvent(
            &self,
            pcallback: Option<&IMFAsyncCallback>,
            punkstate: Option<&IUnknown>,
        ) -> windows::core::Result<()> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.BeginGetEvent(pcallback, punkstate) }
        }

        fn EndGetEvent(&self, presult: Option<&IMFAsyncResult>) -> windows::core::Result<IMFMediaEvent> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.EndGetEvent(presult) }
        }

        fn QueueEvent(
            &self,
            met: u32,
            guidextendedtype: *const GUID,
            hrstatus: HRESULT,
            pvvalue: *const PROPVARIANT,
        ) -> windows::core::Result<()> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.QueueEventParamVar(met, guidextendedtype, hrstatus, pvvalue) }
        }
    }

    impl IMFMediaStream_Impl for WireWaveMediaStream_Impl {
        fn GetMediaSource(&self) -> windows::core::Result<IMFMediaSource> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            Ok(self.source.clone())
        }

        fn GetStreamDescriptor(&self) -> windows::core::Result<IMFStreamDescriptor> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            Ok(self.descriptor.clone())
        }

        fn RequestSample(&self, ptoken: Option<&IUnknown>) -> windows::core::Result<()> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }

            let (_w, _h, nv12) = self.ensure_shm_frame();
            let idx = self.frame_index.fetch_add(1, Ordering::AcqRel) as i64;
            let frame_duration = 10_000_000i64 / (DEFAULT_FPS as i64);

            unsafe {
                let buffer: IMFMediaBuffer = MFCreateMemoryBuffer(nv12.len() as u32)?;
                let mut ptr: *mut u8 = std::ptr::null_mut();
                buffer.Lock(&mut ptr, None, None)?;
                std::ptr::copy_nonoverlapping(nv12.as_ptr(), ptr, nv12.len());
                buffer.Unlock()?;
                buffer.SetCurrentLength(nv12.len() as u32)?;

                let sample: IMFSample = MFCreateSample()?;
                sample.AddBuffer(&buffer)?;
                sample.SetSampleTime(idx * frame_duration)?;
                sample.SetSampleDuration(frame_duration)?;

                if let Some(token) = ptoken {
                    // MFSampleExtension_Token
                    let token_guid = windows::Win32::Media::MediaFoundation::MFSampleExtension_Token;
                    sample.SetUnknown(&token_guid, token)?;
                }

                let unk: IUnknown = sample.cast()?;
                self.events.QueueEventParamUnk(
                    MEMediaSample.0 as u32,
                    &GUID::zeroed(),
                    S_OK,
                    &unk,
                )?;
            }

            Ok(())
        }
    }

    #[implement(IMFMediaSource, IMFMediaEventGenerator)]
    struct WireWaveMediaSource {
        events: IMFMediaEventQueue,
        presentation: IMFPresentationDescriptor,
        stream: Mutex<Option<IMFMediaStream>>,
        shutdown: AtomicBool,
    }

    impl WireWaveMediaSource {
        fn new() -> windows::core::Result<Self> {
            unsafe {
                let events = MFCreateEventQueue()?;
                let mt = create_nv12_media_type(DEFAULT_WIDTH, DEFAULT_HEIGHT)?;
                let stream_desc = MFCreateStreamDescriptor(0, &[Some(mt.clone())])?;
                let handler = stream_desc.GetMediaTypeHandler()?;
                handler.SetCurrentMediaType(&mt)?;

                let presentation = MFCreatePresentationDescriptor(Some(&[Some(stream_desc)]))?;
                presentation.SelectStream(0)?;

                Ok(Self {
                    events,
                    presentation,
                    stream: Mutex::new(None),
                    shutdown: AtomicBool::new(false),
                })
            }
        }
    }

    impl IMFMediaEventGenerator_Impl for WireWaveMediaSource_Impl {
        fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> windows::core::Result<IMFMediaEvent> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.GetEvent(dwflags.0) }
        }

        fn BeginGetEvent(
            &self,
            pcallback: Option<&IMFAsyncCallback>,
            punkstate: Option<&IUnknown>,
        ) -> windows::core::Result<()> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.BeginGetEvent(pcallback, punkstate) }
        }

        fn EndGetEvent(&self, presult: Option<&IMFAsyncResult>) -> windows::core::Result<IMFMediaEvent> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.EndGetEvent(presult) }
        }

        fn QueueEvent(
            &self,
            met: u32,
            guidextendedtype: *const GUID,
            hrstatus: HRESULT,
            pvvalue: *const PROPVARIANT,
        ) -> windows::core::Result<()> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.events.QueueEventParamVar(met, guidextendedtype, hrstatus, pvvalue) }
        }
    }

    impl IMFMediaSource_Impl for WireWaveMediaSource_Impl {
        fn GetCharacteristics(&self) -> windows::core::Result<u32> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            Ok(MFMEDIASOURCE_IS_LIVE.0 as u32)
        }

        fn CreatePresentationDescriptor(&self) -> windows::core::Result<IMFPresentationDescriptor> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            unsafe { self.presentation.Clone() }
        }

        fn Start(
            &self,
            ppresentationdescriptor: Option<&IMFPresentationDescriptor>,
            _pguidtimeformat: *const GUID,
            pvarstartposition: *const PROPVARIANT,
        ) -> windows::core::Result<()> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            let pd = ppresentationdescriptor.ok_or_else(|| Error::from(E_INVALIDARG))?;
            let mut selected = BOOL(0);
            let mut sd_opt: Option<IMFStreamDescriptor> = None;
            unsafe { pd.GetStreamDescriptorByIndex(0, &mut selected, &mut sd_opt)? };
            if !selected.as_bool() {
                return Ok(());
            }
            let sd = sd_opt.ok_or_else(|| Error::from(E_FAIL))?;

            let mut guard = self.stream.lock().map_err(|_| Error::from(E_FAIL))?;
            let (stream_iface, is_new) = if let Some(existing) = guard.as_ref() {
                (existing.clone(), false)
            } else {
                let source_iface: IMFMediaSource = unsafe { self.cast()? };
                let created: IMFMediaStream = WireWaveMediaStream::new(source_iface, sd)?.into();
                *guard = Some(created.clone());
                (created, true)
            };

            unsafe {
                let unk: IUnknown = stream_iface.cast()?;
                let event_type = if is_new { MENewStream.0 } else { MEUpdatedStream.0 };
                self.events
                    .QueueEventParamUnk(event_type as u32, &GUID::zeroed(), S_OK, &unk)?;
                stream_iface.QueueEvent(
                    MEStreamStarted.0 as u32,
                    &GUID::zeroed(),
                    S_OK,
                    pvarstartposition,
                )?;
            }
            Ok(())
        }

        fn Stop(&self) -> windows::core::Result<()> {
            if self.shutdown.load(Ordering::Acquire) {
                return Err(Error::from(MF_E_SHUTDOWN));
            }
            if let Ok(guard) = self.stream.lock() {
                if let Some(stream_iface) = guard.as_ref() {
                    unsafe {
                        let _ = stream_iface.QueueEvent(
                            MEStreamStopped.0 as u32,
                            &GUID::zeroed(),
                            S_OK,
                            std::ptr::null(),
                        );
                    }
                }
            }
            Ok(())
        }

        fn Pause(&self) -> windows::core::Result<()> {
            Ok(())
        }

        fn Shutdown(&self) -> windows::core::Result<()> {
            self.shutdown.store(true, Ordering::Release);
            unsafe {
                let _ = self.events.Shutdown();
            }
            Ok(())
        }
    }

    #[implement(IClassFactory)]
    struct WireWaveClassFactory;

    impl IClassFactory_Impl for WireWaveClassFactory_Impl {
        fn CreateInstance(
            &self,
            punkouter: Option<&IUnknown>,
            riid: *const GUID,
            ppvobject: *mut *mut c_void,
        ) -> windows::core::Result<()> {
            if ppvobject.is_null() {
                return Err(Error::from(E_POINTER));
            }
            unsafe { *ppvobject = std::ptr::null_mut() };
            if punkouter.is_some() {
                return Err(Error::from(E_NOINTERFACE));
            }
            let source: IMFMediaSource = WireWaveMediaSource::new()?.into();
            unsafe { source.query(&*riid, ppvobject).ok() }
        }

        fn LockServer(&self, flock: BOOL) -> windows::core::Result<()> {
            if flock.as_bool() {
                DLL_REF_COUNT.fetch_add(1, Ordering::AcqRel);
            } else {
                DLL_REF_COUNT.fetch_sub(1, Ordering::AcqRel);
            }
            Ok(())
        }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn DllMain(hmodule: HMODULE, reason: u32, _reserved: *mut c_void) -> BOOL {
        if reason == 1 {
            unsafe { MODULE_HANDLE = hmodule };
        }
        BOOL(1)
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn DllGetClassObject(
        rclsid: *const GUID,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        if rclsid.is_null() || riid.is_null() || ppv.is_null() {
            return E_POINTER;
        }
        unsafe {
            *ppv = std::ptr::null_mut();
            if *rclsid != CLSID_WIREWAVE_VCAM {
                return CLASS_E_CLASSNOTAVAILABLE;
            }
            let factory: IClassFactory = WireWaveClassFactory.into();
            factory.query(&*riid, ppv)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn DllCanUnloadNow() -> HRESULT {
        if DLL_REF_COUNT.load(Ordering::Acquire) == 0 {
            S_OK
        } else {
            S_FALSE
        }
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn DllRegisterServer() -> HRESULT {
        let mut path_buf = [0u16; MAX_PATH as usize];
        let len = unsafe { GetModuleFileNameW(MODULE_HANDLE, &mut path_buf) } as usize;
        if len == 0 {
            return E_FAIL;
        }
        let subkey: Vec<u16> = format!("Software\\Classes\\CLSID\\{CLSID_STR}\\InprocServer32\0")
            .encode_utf16()
            .collect();
        let threading: Vec<u16> = "Both\0".encode_utf16().collect();

        unsafe {
            let mut hkey = Default::default();
            let res = RegCreateKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey.as_ptr()),
                0,
                None,
                REG_OPTION_NON_VOLATILE,
                KEY_WRITE,
                None,
                &mut hkey,
                None,
            );
            if res.is_err() {
                return E_FAIL;
            }
            let dll_bytes = std::slice::from_raw_parts(
                path_buf.as_ptr() as *const u8,
                (len + 1) * 2,
            );
            let _ = RegSetValueExW(hkey, None, 0, REG_SZ, Some(dll_bytes));
            let thr_bytes = std::slice::from_raw_parts(
                threading.as_ptr() as *const u8,
                threading.len() * 2,
            );
            let _ = RegSetValueExW(hkey, w!("ThreadingModel"), 0, REG_SZ, Some(thr_bytes));
            let _ = RegCloseKey(hkey);
        }
        S_OK
    }

    #[unsafe(no_mangle)]
    pub extern "system" fn DllUnregisterServer() -> HRESULT {
        let subkey: Vec<u16> = format!("Software\\Classes\\CLSID\\{CLSID_STR}\0")
            .encode_utf16()
            .collect();
        unsafe {
            let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()));
        }
        S_OK
    }
}
