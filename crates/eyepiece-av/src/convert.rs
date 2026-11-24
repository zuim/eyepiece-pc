//! Scale / letterbox / colour-convert with libavfilter, on the GPU where possible:
//!
//!   DRM-PRIME (KMS, PipeWire dmabuf) ─hwmap→ VAAPI ─scale_vaapi(hq, aspect)─pad_vaapi─scale_vaapi(nv12, bt709 tv)
//!        └→ VAAPI encoder directly (zero-copy), or hwdownload NV12 (3 MB) for NVENC/x264/AMF/QSV
//!   CPU RGB/YUV ─(hwupload → same VAAPI chain)  or  swscale (bicubic) + pad
//!
//! Bars are added while still RGB: pad_vaapi on NV12 writes Y=U=V=0 (green).
//! The graph is rebuilt when the input size/format changes.

use crate::ffi::*;
use std::ptr;

/// swscale slice threads for the CPU path: 2560×720 BGRx→NV12 (bicubic) takes ≈9.5 ms on one
/// thread and ≈3.2 ms on four (FFmpeg 9, 16-core laptop); more threads gain little and would
/// compete with the encoder.
const SWS_THREADS: u32 = 4;

/// What the encoder wants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// VAAPI surfaces (NV12) for *_vaapi encoders
    Vaapi,
    VideoToolbox,
    /// system-memory NV12 (nvenc, libx264, amf, qsv)
    Nv12,
    /// system-memory yuv420p (libx265)
    Yuv420p,
}

#[derive(Debug, Clone)]
pub struct ConvertConfig {
    pub width: u32,
    pub height: u32,
    /// keep the aspect ratio with black bars (else stretch)
    pub letterbox: bool,
    pub target: Target,
    /// VAAPI render node for CPU input (hwupload) / VAAPI encode
    pub vaapi_device: String,
    /// use the GPU (VAAPI) for scaling CPU frames too
    pub gpu_scale_cpu_input: bool,
    /// extra filters appended before the output format (advanced, `--vf`)
    pub extra_vf: Option<String>,
    /// scale_vaapi mode: hq (default) | fast | nl_anamorphic | default
    pub scale_mode: String,
}

struct Graph {
    graph: *mut AVFilterGraph,
    src: *mut AVFilterContext,
    sink: *mut AVFilterContext,
    key: (i32, i32, i32, usize),
}
impl Drop for Graph {
    fn drop(&mut self) {
        unsafe { avfilter_graph_free(&mut self.graph) };
    }
}

pub struct Converter {
    cfg: ConvertConfig,
    fps: i32,
    vaapi: Option<BufRef>,
    graph: Option<Graph>,
    pub description: String,
    /// use FFmpeg's hwdownload filter instead of map + uncached-memory copy
    filter_download: bool,
    dl: Option<Download>,
}
unsafe impl Send for Converter {}

fn hw_key(f: &Frame) -> usize {
    let r = unsafe { (*f.0).hw_frames_ctx };
    if r.is_null() {
        0
    } else {
        unsafe { (*r).data as usize }
    }
}

impl Converter {
    pub fn new(cfg: ConvertConfig, fps: u32) -> Converter {
        let filter_download = std::env::var_os("EYEPIECE_FILTER_DOWNLOAD").is_some();
        Converter {
            cfg,
            fps: fps as i32,
            vaapi: None,
            graph: None,
            description: String::new(),
            filter_download,
            dl: None,
        }
    }

    fn vaapi(&mut self) -> Result<BufRef, String> {
        if self.vaapi.is_none() {
            self.vaapi = Some(crate::hw::hwdevice(
                AVHWDeviceType::AV_HWDEVICE_TYPE_VAAPI,
                Some(&self.cfg.vaapi_device),
            )?);
        }
        Ok(self.vaapi.clone().unwrap())
    }

    fn vaapi_chain(&self, in_w: i32, in_h: i32) -> String {
        let (w, h) = (self.cfg.width, self.cfg.height);
        let m = &self.cfg.scale_mode;
        let same_aspect = (in_w as i64 * h as i64 - in_h as i64 * w as i64).abs() < (w as i64) * 2;
        let download_pads = self.cfg.target == Target::Nv12 && !self.filter_download;
        let mut s = if self.cfg.letterbox && !same_aspect && download_pads {
            // one VPP pass to the picture size; the bars come for free from the
            // pre-blackened download buffers (see `Download`)
            format!(
                "scale_vaapi=w={w}:h={h}:mode={m}:force_original_aspect_ratio=decrease:force_divisible_by=2:\
                 format=nv12:out_color_matrix=bt709:out_range=tv"
            )
        } else if self.cfg.letterbox && !same_aspect {
            format!(
                "scale_vaapi=w={w}:h={h}:mode={m}:force_original_aspect_ratio=decrease:force_divisible_by=2,\
                 pad_vaapi=w={w}:h={h}:x=(ow-iw)/2:y=(oh-ih)/2:color=black,\
                 scale_vaapi=format=nv12:out_color_matrix=bt709:out_range=tv"
            )
        } else {
            format!(
                "scale_vaapi=w={w}:h={h}:mode={m}:format=nv12:out_color_matrix=bt709:out_range=tv"
            )
        };
        if let Some(x) = &self.cfg.extra_vf {
            s = format!("{s},{x}");
        }
        match self.cfg.target {
            Target::Vaapi => s,
            // downloaded by `download()` (mapped + streaming-load copy), see below
            Target::Nv12 | Target::VideoToolbox if !self.filter_download => s,
            Target::Nv12 | Target::VideoToolbox => s + ",hwdownload,format=nv12",
            Target::Yuv420p => s + ",hwdownload,format=nv12,format=yuv420p",
        }
    }

    fn cpu_chain(&self) -> String {
        let (w, h) = (self.cfg.width, self.cfg.height);
        let fmt = if self.cfg.target == Target::Yuv420p {
            "yuv420p"
        } else {
            "nv12"
        };
        let mut s = if self.cfg.letterbox {
            format!(
                "scale=w={w}:h={h}:flags=bicubic:threads={SWS_THREADS}:force_original_aspect_ratio=decrease:force_divisible_by=2:\
                 out_color_matrix=bt709:out_range=tv,pad=w={w}:h={h}:x=(ow-iw)/2:y=(oh-ih)/2:color=black"
            )
        } else {
            format!("scale=w={w}:h={h}:flags=bicubic:threads={SWS_THREADS}:out_color_matrix=bt709:out_range=tv")
        };
        if let Some(x) = &self.cfg.extra_vf {
            s = format!("{s},{x}");
        }
        s += &format!(",format={fmt}");
        if self.cfg.target == Target::Vaapi {
            s += ",hwupload";
        }
        s
    }

    fn build(&mut self, f: &Frame) -> Result<Graph, String> {
        let (w, h, fmt) = (f.width(), f.height(), f.format());
        let hw_in = unsafe { (*f.0).hw_frames_ctx };
        let is_drm = fmt == AVPixelFormat::AV_PIX_FMT_DRM_PRIME as i32;
        let chain = if is_drm {
            format!(
                "hwmap=derive_device=vaapi:mode=read,{}",
                self.vaapi_chain(w, h)
            )
        } else if self.cfg.target == Target::Vaapi
            || (cfg!(target_os = "linux")
                && self.cfg.gpu_scale_cpu_input
                && (w as u32, h as u32) != (self.cfg.width, self.cfg.height))
        {
            // GPU scaling pays off only when there is something to scale
            format!("hwupload,{}", self.vaapi_chain(w, h))
        } else {
            self.cpu_chain()
        };
        let needs_device = chain.contains("hwupload");
        unsafe {
            let graph = avfilter_graph_alloc();
            if graph.is_null() {
                return Err("avfilter_graph_alloc".into());
            }
            let mut g = Graph {
                graph,
                src: ptr::null_mut(),
                sink: ptr::null_mut(),
                key: (w, h, fmt, hw_key(f)),
            };
            (*graph).nb_threads = 4;
            // buffersrc: parameters (incl. the hw frames context) before init
            g.src = avfilter_graph_alloc_filter(
                graph,
                avfilter_get_by_name(cstr("buffer").as_ptr()),
                cstr("in").as_ptr(),
            );
            if g.src.is_null() {
                return Err("buffersrc alloc".into());
            }
            let par = av_buffersrc_parameters_alloc();
            (*par).format = fmt;
            (*par).width = w;
            (*par).height = h;
            (*par).time_base = AVRational {
                num: 1,
                den: self.fps,
            };
            (*par).sample_aspect_ratio = AVRational { num: 1, den: 1 };
            (*par).hw_frames_ctx = hw_in; // borrowed: parameters_set takes its own ref
            let r = av_buffersrc_parameters_set(g.src, par);
            av_free(par as *mut _);
            check(r, "buffersrc parameters")?;
            check(avfilter_init_dict(g.src, ptr::null_mut()), "buffersrc init")?;
            check(
                avfilter_graph_create_filter(
                    &mut g.sink,
                    avfilter_get_by_name(cstr("buffersink").as_ptr()),
                    cstr("out").as_ptr(),
                    ptr::null(),
                    ptr::null_mut(),
                    graph,
                ),
                "buffersink",
            )?;
            let mut inputs: *mut AVFilterInOut = ptr::null_mut();
            let mut outputs: *mut AVFilterInOut = ptr::null_mut();
            // segment API: filters are created first, so hwupload/hwmap can get the device
            // before their init runs
            let mut seg: *mut AVFilterGraphSegment = ptr::null_mut();
            let dev = if needs_device {
                Some(self.vaapi()?)
            } else {
                None
            };
            let mut r = avfilter_graph_segment_parse(graph, cstr(&chain).as_ptr(), 0, &mut seg);
            if r >= 0 {
                r = avfilter_graph_segment_create_filters(seg, 0);
            }
            if r >= 0 {
                if let Some(dev) = &dev {
                    for i in 0..(*graph).nb_filters as usize {
                        let fc = *(*graph).filters.add(i);
                        if (*fc).hw_device_ctx.is_null() {
                            (*fc).hw_device_ctx = dev.new_ref();
                        }
                    }
                }
                // returns the chain's open input/output pads: link them to src/sink
                r = avfilter_graph_segment_apply(seg, 0, &mut inputs, &mut outputs);
                if r >= 0 && (inputs.is_null() || outputs.is_null()) {
                    r = averror(libc::EINVAL);
                }
                if r >= 0 {
                    r = avfilter_link(g.src, 0, (*inputs).filter_ctx, (*inputs).pad_idx as u32);
                }
                if r >= 0 {
                    r = avfilter_link((*outputs).filter_ctx, (*outputs).pad_idx as u32, g.sink, 0);
                }
            }
            avfilter_graph_segment_free(&mut seg);
            avfilter_inout_free(&mut inputs);
            avfilter_inout_free(&mut outputs);
            check(r, &format!("filter graph '{chain}'"))?;
            check(
                avfilter_graph_config(graph, ptr::null_mut()),
                &format!("configure '{chain}'"),
            )?;
            self.description = format!("{w}x{h} {} → {chain}", pix_fmt_name(fmt));
            Ok(g)
        }
    }

    /// Convert one frame (consumed). Returns the encoder-ready frame.
    pub fn convert(&mut self, f: Frame) -> Result<Option<Frame>, String> {
        if f.format() == AVPixelFormat::AV_PIX_FMT_VIDEOTOOLBOX as i32 {
            if self.cfg.target != Target::VideoToolbox
                || self.cfg.extra_vf.is_some()
                || (f.width() as u32, f.height() as u32) != (self.cfg.width, self.cfg.height)
            {
                return Err("Native VideoToolbox surfaces require matching output size, a VideoToolbox encoder, and no --vf; use CPU capture for custom conversion".into());
            }
            self.graph = None;
            let out = Frame::alloc();
            unsafe {
                check(
                    av_hwframe_transfer_data(out.0, f.0, 0),
                    "VideoToolbox NV12 download",
                )?;
                check(
                    av_frame_copy_props(out.0, f.0),
                    "VideoToolbox frame properties",
                )?;
            }
            self.description =
                "ScreenCaptureKit-scaled NV12 readback (avoids native encoder buffering)".into();
            return Ok(Some(out));
        }
        let key = (f.width(), f.height(), f.format(), hw_key(&f));
        // a new hw frames context (capture recreated it) also needs a new graph; the old
        // one stays referenced by the old buffersrc, so its address can't be reused
        let rebuild = self.graph.as_ref().map_or(true, |g| g.key != key);
        if rebuild {
            self.graph = None;
            self.graph = Some(self.build(&f)?);
        }
        let g = self.graph.as_mut().unwrap();
        unsafe {
            // moves the frame's references into the graph (f is left empty)
            check(av_buffersrc_add_frame_flags(g.src, f.0, 0), "buffersrc add")?;
            drop(f);
            let out = Frame::alloc();
            let r = av_buffersink_get_frame(g.sink, out.0);
            if r == EAGAIN_ {
                return Ok(None);
            }
            check(r, "buffersink get")?;
            if out.format() == AVPixelFormat::AV_PIX_FMT_VAAPI as i32
                && self.cfg.target != Target::Vaapi
            {
                let (w, h) = (self.cfg.width as i32, self.cfg.height as i32);
                if self.dl.as_ref().map_or(true, |d| (d.w, d.h) != (w, h)) {
                    self.dl = Some(Download::new(w, h)?);
                }
                return self.dl.as_mut().unwrap().download(out).map(Some);
            }
            Ok(Some(out))
        }
    }

    /// Frames context of the converter's output (for VAAPI encoders).
    pub fn output_hw_frames(&self) -> *mut AVBufferRef {
        match &self.graph {
            Some(g) => unsafe { av_buffersink_get_hw_frames_ctx(g.sink) },
            None => ptr::null_mut(),
        }
    }
}

/// VAAPI surface → system-memory NV12 (NVENC/x264/AMF/QSV input), letterboxed.
///
/// Maps the surface (waits for the GPU) and copies with SSE4.1 streaming loads
/// (`av_image_copy_uc_from`): the mapping is uncached memory. The picture is copied
/// centred into full-size buffers from a pool whose buffers were filled black once on
/// allocation, so letterboxing costs no extra GPU pass and no per-frame fill.
struct Download {
    pool: *mut AVBufferPool,
    w: i32,
    h: i32,
    stride: i32,
}
unsafe impl Send for Download {}

#[repr(C)]
struct PoolInfo {
    y_size: usize,
}

unsafe extern "C" fn pool_alloc(opaque: *mut std::ffi::c_void, size: usize) -> *mut AVBufferRef {
    let info = &*(opaque as *const PoolInfo);
    let b = av_buffer_alloc(size);
    if !b.is_null() {
        let d = std::slice::from_raw_parts_mut((*b).data, size);
        d[..info.y_size].fill(16); // Y black (tv range)
        d[info.y_size..].fill(128); // UV neutral
    }
    b
}
unsafe extern "C" fn pool_free(opaque: *mut std::ffi::c_void) {
    drop(Box::from_raw(opaque as *mut PoolInfo));
}

impl Download {
    fn new(w: i32, h: i32) -> Result<Download, String> {
        let stride = (w + 63) & !63;
        let y_size = stride as usize * h as usize;
        let size = y_size + stride as usize * (h as usize / 2) + 64;
        let info = Box::into_raw(Box::new(PoolInfo { y_size }));
        let pool = unsafe {
            av_buffer_pool_init2(size, info as *mut _, Some(pool_alloc), Some(pool_free))
        };
        if pool.is_null() {
            return Err("av_buffer_pool_init2".into());
        }
        Ok(Download { pool, w, h, stride })
    }

    fn download(&mut self, hw: Frame) -> Result<Frame, String> {
        unsafe {
            let m = Frame::alloc();
            (*m.0).format = AVPixelFormat::AV_PIX_FMT_NV12 as i32;
            check(
                av_hwframe_map(m.0, hw.0, AV_HWFRAME_MAP_READ as i32),
                "map VAAPI surface",
            )?;
            let (pw, ph) = (hw.width().min(self.w), hw.height().min(self.h));
            let (x, y) = (((self.w - pw) / 2) & !1, ((self.h - ph) / 2) & !1);
            let buf = av_buffer_pool_get(self.pool);
            if buf.is_null() {
                return Err("buffer pool".into());
            }
            let out = Frame::alloc();
            let o = &mut *out.0;
            o.buf[0] = buf;
            o.format = AVPixelFormat::AV_PIX_FMT_NV12 as i32;
            o.width = self.w;
            o.height = self.h;
            o.linesize[0] = self.stride;
            o.linesize[1] = self.stride;
            o.data[0] = (*buf).data;
            o.data[1] = (*buf).data.add(self.stride as usize * self.h as usize);
            // destination = the centred picture rectangle
            let dst: [*mut u8; 4] = [
                o.data[0].add(y as usize * self.stride as usize + x as usize),
                o.data[1].add((y / 2) as usize * self.stride as usize + x as usize),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ];
            let dl: [isize; 4] = [self.stride as isize, self.stride as isize, 0, 0];
            let sl: [isize; 4] = std::array::from_fn(|i| (*m.0).linesize[i] as isize);
            let sd: [*const u8; 4] = std::array::from_fn(|i| (*m.0).data[i] as *const u8);
            av_image_copy_uc_from(
                dst.as_ptr(),
                dl.as_ptr(),
                sd.as_ptr(),
                sl.as_ptr(),
                AVPixelFormat::AV_PIX_FMT_NV12,
                pw,
                ph,
            );
            av_frame_copy_props(out.0, hw.0);
            o.color_range = AVColorRange::AVCOL_RANGE_MPEG;
            o.colorspace = AVColorSpace::AVCOL_SPC_BT709;
            Ok(out)
        }
    }
}

impl Drop for Download {
    fn drop(&mut self) {
        unsafe { av_buffer_pool_uninit(&mut self.pool) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> ConvertConfig {
        ConvertConfig {
            width: 64,
            height: 32,
            letterbox: true,
            target: Target::VideoToolbox,
            vaapi_device: String::new(),
            gpu_scale_cpu_input: false,
            extra_vf: None,
            scale_mode: "hq".into(),
        }
    }

    fn native_frame() -> Frame {
        let frame = Frame::alloc();
        unsafe {
            (*frame.0).width = 64;
            (*frame.0).height = 32;
            (*frame.0).format = AVPixelFormat::AV_PIX_FMT_VIDEOTOOLBOX as i32;
        }
        frame
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn native_surface_downloads_nv12_without_a_filter_graph() {
        use eyepiece_capture::screencapturekit::CVPixelBuffer;
        let mut converter = Converter::new(config(), 60);
        let pixels = CVPixelBuffer::create(64, 32, u32::from_be_bytes(*b"420v")).unwrap();
        let mut frames = crate::hw::VideoToolboxFrames::new().unwrap();
        let frame = frames.wrap(pixels).unwrap();
        let input = cpu_frame();
        unsafe {
            check(
                av_hwframe_transfer_data(frame.0, input.0, 0),
                "upload test pixels",
            )
            .unwrap();
        }
        let result = converter.convert(frame).unwrap().unwrap();
        assert_eq!(result.format(), AVPixelFormat::AV_PIX_FMT_NV12 as i32);
        assert_eq!((result.width(), result.height()), (64, 32));
        unsafe {
            assert!((*result.0).hw_frames_ctx.is_null());
            assert_eq!(*(*result.0).data[0], 16);
            assert_eq!(*(*result.0).data[1], 128);
            assert_eq!((*result.0).color_range, AVColorRange::AVCOL_RANGE_MPEG);
        }
        assert!(converter.graph.is_none());
        assert!(converter.description.contains("readback"));
    }

    #[test]
    fn incompatible_native_conversion_is_rejected() {
        let mut mismatched = config();
        mismatched.width = 128;
        let mut filtered = config();
        filtered.extra_vf = Some("hflip".into());
        let mut software = config();
        software.target = Target::Nv12;
        for settings in [mismatched, filtered, software] {
            assert!(Converter::new(settings, 60)
                .convert(native_frame())
                .is_err());
        }
    }

    fn cpu_frame() -> Frame {
        let frame = Frame::alloc();
        unsafe {
            (*frame.0).width = 64;
            (*frame.0).height = 32;
            (*frame.0).format = AVPixelFormat::AV_PIX_FMT_NV12 as i32;
            check(av_frame_get_buffer(frame.0, 32), "allocate test frame").unwrap();
            std::ptr::write_bytes((*frame.0).data[0], 16, (*frame.0).linesize[0] as usize * 32);
            std::ptr::write_bytes(
                (*frame.0).data[1],
                128,
                (*frame.0).linesize[1] as usize * 16,
            );
        }
        frame
    }

    #[test]
    fn x264_bgra_input_uses_benchmark_cpu_conversion() {
        let frame = Frame::alloc();
        unsafe {
            (*frame.0).width = 96;
            (*frame.0).height = 64;
            (*frame.0).format = AVPixelFormat::AV_PIX_FMT_BGR0 as i32;
            check(av_frame_get_buffer(frame.0, 32), "allocate BGRA test frame").unwrap();
            std::ptr::write_bytes((*frame.0).data[0], 0, (*frame.0).linesize[0] as usize * 64);
        }
        let mut settings = config();
        settings.target = Target::Nv12;
        let mut converter = Converter::new(settings, 60);
        let output = converter.convert(frame).unwrap().unwrap();
        assert_eq!(output.format(), AVPixelFormat::AV_PIX_FMT_NV12 as i32);
        assert_eq!((output.width(), output.height()), (64, 32));
        assert!(unsafe { (*output.0).hw_frames_ctx.is_null() });
        assert!(converter.description.contains("bgr0"));
        assert!(converter
            .description
            .contains(&format!("flags=bicubic:threads={SWS_THREADS}")));
        assert!(converter
            .description
            .contains("out_color_matrix=bt709:out_range=tv"));
        assert!(converter.description.contains("pad=w=64:h=32"));
        assert!(converter.description.ends_with("format=nv12"));
    }

    #[test]
    fn cpu_input_remains_supported_for_videotoolbox_encoding() {
        let mut converter = Converter::new(config(), 60);
        let result = converter.convert(cpu_frame()).unwrap().unwrap();
        assert_eq!(result.format(), AVPixelFormat::AV_PIX_FMT_NV12 as i32);
        assert_eq!((result.width(), result.height()), (64, 32));
        assert!(unsafe { (*result.0).hw_frames_ctx.is_null() });
    }
}
