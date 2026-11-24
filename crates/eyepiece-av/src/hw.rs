//! Hardware device contexts and wrapping captured buffers as AVFrames (no copies).

use crate::ffi::*;
use std::ptr;
use eyepiece_capture as cap;

pub fn hwdevice(kind: AVHWDeviceType, device: Option<&str>) -> Result<BufRef, String> {
    let mut r: *mut AVBufferRef = ptr::null_mut();
    let d = device.map(cstr);
    let ret = unsafe {
        av_hwdevice_ctx_create(&mut r, kind, d.as_ref().map_or(ptr::null(), |c| c.as_ptr()), ptr::null_mut(), 0)
    };
    check(ret, &format!("hw device {:?} {}", kind, device.unwrap_or("")))?;
    Ok(BufRef(r))
}

/// DRM fourcc → software pixel format of that layout.
pub fn fourcc_to_pix(f: u32) -> Option<AVPixelFormat> {
    use cap::*;
    Some(match f {
        DRM_FORMAT_XRGB8888 => AVPixelFormat::AV_PIX_FMT_BGR0,
        DRM_FORMAT_ARGB8888 => AVPixelFormat::AV_PIX_FMT_BGRA,
        DRM_FORMAT_XBGR8888 => AVPixelFormat::AV_PIX_FMT_RGB0,
        DRM_FORMAT_ABGR8888 => AVPixelFormat::AV_PIX_FMT_RGBA,
        DRM_FORMAT_XRGB2101010 => AVPixelFormat::AV_PIX_FMT_X2RGB10LE,
        DRM_FORMAT_XBGR2101010 => AVPixelFormat::AV_PIX_FMT_X2BGR10LE,
        DRM_FORMAT_NV12 => AVPixelFormat::AV_PIX_FMT_NV12,
        _ => return None,
    })
}

pub fn cpu_to_pix(f: cap::CpuFormat) -> AVPixelFormat {
    match f {
        cap::CpuFormat::Bgrx => AVPixelFormat::AV_PIX_FMT_BGR0,
        cap::CpuFormat::Rgbx => AVPixelFormat::AV_PIX_FMT_RGB0,
        cap::CpuFormat::X2Rgb10 => AVPixelFormat::AV_PIX_FMT_X2RGB10LE,
        cap::CpuFormat::X2Bgr10 => AVPixelFormat::AV_PIX_FMT_X2BGR10LE,
    }
}

/// Keeps a DRM device + a DRM_PRIME frames context matching the current framebuffer
/// (recreated when size/format change).
#[cfg(target_os = "linux")]
pub struct DrmFrames {
    pub device: BufRef,
    frames: Option<(BufRef, u32, u32, u32)>,
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct DrmHolder {
    desc: AVDRMFrameDescriptor,
    _buf: cap::DmaBuf, // owns the dma-buf fds until FFmpeg releases the frame
}

#[cfg(target_os = "linux")]
unsafe extern "C" fn free_drm(_opaque: *mut std::ffi::c_void, data: *mut u8) {
    drop(Box::from_raw(data as *mut DrmHolder));
}
unsafe extern "C" fn free_vec(opaque: *mut std::ffi::c_void, _data: *mut u8) {
    drop(Box::from_raw(opaque as *mut Vec<u8>));
}

#[cfg(target_os = "linux")]
impl DrmFrames {
    pub fn new(device: &str) -> Result<DrmFrames, String> {
        Ok(DrmFrames { device: hwdevice(AVHWDeviceType::AV_HWDEVICE_TYPE_DRM, Some(device))?, frames: None })
    }

    fn frames_ctx(&mut self, w: u32, h: u32, fourcc: u32) -> Result<BufRef, String> {
        if let Some((f, fw, fh, ff)) = &self.frames {
            if (*fw, *fh, *ff) == (w, h, fourcc) {
                return Ok(f.clone());
            }
        }
        let sw = fourcc_to_pix(fourcc).ok_or_else(|| {
            format!(
                "framebuffer format {} not supported (8/10-bit RGB or NV12 only; fp16/HDR scan-out is not)",
                cap::fourcc_str(fourcc)
            )
        })?;
        unsafe {
            let r = av_hwframe_ctx_alloc(self.device.0);
            if r.is_null() {
                return Err("av_hwframe_ctx_alloc".into());
            }
            let fc = (*r).data as *mut AVHWFramesContext;
            (*fc).format = AVPixelFormat::AV_PIX_FMT_DRM_PRIME;
            (*fc).sw_format = sw;
            (*fc).width = w as i32;
            (*fc).height = h as i32;
            let mut rr = r;
            if let Err(e) = check(av_hwframe_ctx_init(r), "DRM frames ctx") {
                av_buffer_unref(&mut rr);
                return Err(e);
            }
            let b = BufRef(r);
            self.frames = Some((b.clone(), w, h, fourcc));
            Ok(b)
        }
    }

    /// DMA-BUF → AV_PIX_FMT_DRM_PRIME frame (descriptor only; the pixels stay on the GPU).
    pub fn wrap(&mut self, d: cap::DmaBuf) -> Result<Frame, String> {
        let fc = self.frames_ctx(d.width, d.height, d.fourcc)?;
        let mut desc: AVDRMFrameDescriptor = Default::default();
        use std::os::fd::AsRawFd;
        desc.nb_objects = d.objects.len() as i32;
        for (i, o) in d.objects.iter().enumerate() {
            desc.objects[i].fd = o.fd.as_raw_fd();
            desc.objects[i].size = o.size;
            desc.objects[i].format_modifier = d.modifier;
        }
        desc.nb_layers = 1;
        desc.layers[0].format = d.fourcc;
        desc.layers[0].nb_planes = d.planes.len() as i32;
        for (i, p) in d.planes.iter().enumerate() {
            desc.layers[0].planes[i].object_index = p.object as i32;
            desc.layers[0].planes[i].offset = p.offset as isize;
            desc.layers[0].planes[i].pitch = p.pitch as isize;
        }
        let (w, h) = (d.width, d.height);
        let holder = Box::into_raw(Box::new(DrmHolder { desc, _buf: d }));
        let f = Frame::alloc();
        unsafe {
            let buf = av_buffer_create(
                holder as *mut u8,
                std::mem::size_of::<AVDRMFrameDescriptor>(),
                Some(free_drm),
                ptr::null_mut(),
                0,
            );
            if buf.is_null() {
                drop(Box::from_raw(holder));
                return Err("av_buffer_create".into());
            }
            let fr = f.0;
            (*fr).buf[0] = buf;
            (*fr).data[0] = holder as *mut u8;
            (*fr).format = AVPixelFormat::AV_PIX_FMT_DRM_PRIME as i32;
            (*fr).width = w as i32;
            (*fr).height = h as i32;
            (*fr).hw_frames_ctx = fc.new_ref();
        }
        Ok(f)
    }
}

/// CPU image → AVFrame referencing the image's buffer (no copy).
pub fn wrap_cpu(img: cap::CpuImage) -> Result<Frame, String> {
    let f = Frame::alloc();
    let fmt = cpu_to_pix(img.format);
    let (w, h, stride) = (img.width, img.height, img.stride);
    let v = Box::into_raw(Box::new(img.data));
    unsafe {
        let data = (*v).as_mut_ptr();
        let buf = av_buffer_create(data, (*v).len(), Some(free_vec), v as *mut _, 0);
        if buf.is_null() {
            drop(Box::from_raw(v));
            return Err("av_buffer_create".into());
        }
        let fr = f.0;
        (*fr).buf[0] = buf;
        (*fr).data[0] = data;
        (*fr).linesize[0] = stride as i32;
        (*fr).format = fmt as i32;
        (*fr).width = w as i32;
        (*fr).height = h as i32;
    }
    Ok(f)
}

#[cfg(target_os = "macos")]
pub struct VideoToolboxFrames {
    device: BufRef,
    frames: Option<(BufRef, i32, i32)>,
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn free_pixel_buffer(opaque: *mut std::ffi::c_void, _data: *mut u8) {
    drop(Box::from_raw(opaque as *mut cap::screencapturekit::CVPixelBuffer));
}

#[cfg(target_os = "macos")]
impl VideoToolboxFrames {
    pub fn new() -> Result<Self, String> {
        Ok(Self { device: hwdevice(AVHWDeviceType::AV_HWDEVICE_TYPE_VIDEOTOOLBOX, None)?, frames: None })
    }

    fn frames_ctx(&mut self, width: i32, height: i32) -> Result<BufRef, String> {
        if let Some((frames, previous_width, previous_height)) = &self.frames {
            if (width, height) == (*previous_width, *previous_height) {
                return Ok(frames.clone());
            }
        }
        let frames = BufRef(unsafe { av_hwframe_ctx_alloc(self.device.0) });
        if frames.0.is_null() {
            return Err("Could not allocate VideoToolbox frames context".into());
        }
        unsafe {
            let context = &mut *((*frames.0).data as *mut AVHWFramesContext);
            context.format = AVPixelFormat::AV_PIX_FMT_VIDEOTOOLBOX;
            context.sw_format = AVPixelFormat::AV_PIX_FMT_NV12;
            context.width = width;
            context.height = height;
            check(av_hwframe_ctx_init(frames.0), "VideoToolbox frames context")?;
        }
        self.frames = Some((frames.clone(), width, height));
        Ok(frames)
    }

    pub fn wrap(&mut self, pixels: cap::screencapturekit::CVPixelBuffer) -> Result<Frame, String> {
        if pixels.pixel_format() != u32::from_be_bytes(*b"420v") {
            return Err("VideoToolbox capture requires video-range NV12".into());
        }
        let width = i32::try_from(pixels.width()).map_err(|_| "VideoToolbox width overflow")?;
        let height = i32::try_from(pixels.height()).map_err(|_| "VideoToolbox height overflow")?;
        if width < 2 || height < 2 || width % 2 != 0 || height % 2 != 0 {
            return Err("VideoToolbox capture requires positive even dimensions".into());
        }
        let context = self.frames_ctx(width, height)?;
        let frame = Frame::alloc();
        let pointer = pixels.as_ptr();
        let owner = Box::into_raw(Box::new(pixels));
        unsafe {
            let buffer = av_buffer_create(pointer as *mut u8, 1, Some(free_pixel_buffer), owner.cast(), AV_BUFFER_FLAG_READONLY as i32);
            if buffer.is_null() {
                drop(Box::from_raw(owner));
                return Err("Could not retain pixel buffer in an AVFrame".into());
            }
            (*frame.0).buf[0] = buffer;
            (*frame.0).data[3] = pointer.cast();
            (*frame.0).format = AVPixelFormat::AV_PIX_FMT_VIDEOTOOLBOX as i32;
            (*frame.0).width = width;
            (*frame.0).height = height;
            (*frame.0).hw_frames_ctx = context.new_ref();
            (*frame.0).color_range = AVColorRange::AVCOL_RANGE_MPEG;
            (*frame.0).colorspace = AVColorSpace::AVCOL_SPC_BT709;
            (*frame.0).color_primaries = AVColorPrimaries::AVCOL_PRI_BT709;
            (*frame.0).color_trc = AVColorTransferCharacteristic::AVCOL_TRC_BT709;
        }
        Ok(frame)
    }
}

#[cfg(all(test, target_os = "macos"))]
mod videotoolbox_tests {
    use super::*;
    use cap::screencapturekit::CVPixelBuffer;

    #[test]
    fn cloned_avframe_keeps_its_native_pixel_buffer_alive() {
        let pixels = CVPixelBuffer::create(64, 32, u32::from_be_bytes(*b"420v")).unwrap();
        let pointer = pixels.as_ptr();
        let mut frames = VideoToolboxFrames::new().unwrap();
        let frame = frames.wrap(pixels).unwrap();
        assert_eq!(unsafe { (*frame.0).data[3] }, pointer.cast());
        let cloned_pointer = unsafe { av_frame_clone(frame.0) };
        assert!(!cloned_pointer.is_null());
        let cloned = Frame(cloned_pointer);
        drop(frame);
        drop(frames);
        let retained = unsafe { CVPixelBuffer::from_raw_borrowed((*cloned.0).data[3].cast()) }.unwrap();
        assert_eq!((retained.width(), retained.height()), (64, 32));
        assert_eq!(retained.as_ptr(), pointer);
        assert_eq!(cloned.format(), AVPixelFormat::AV_PIX_FMT_VIDEOTOOLBOX as i32);
    }

    #[test]
    fn bgra_surfaces_are_not_mislabeled_as_nv12() {
        let pixels = CVPixelBuffer::create(64, 32, u32::from_be_bytes(*b"BGRA")).unwrap();
        assert!(VideoToolboxFrames::new().unwrap().wrap(pixels).is_err());
    }
}
