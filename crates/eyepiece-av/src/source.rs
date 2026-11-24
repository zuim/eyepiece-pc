//! Frame sources as AVFrames: screen capture backends (wrapped without copies), the
//! lavfi test pattern and looping media files.

use crate::ffi::*;
use crate::hw::wrap_cpu;
#[cfg(target_os = "linux")]
use crate::hw::DrmFrames;
use std::ptr;
use eyepiece_capture::pacer::Pacer;
use eyepiece_capture::{Capture, FrameData};
use eyepiece_link::clock::{mono_us, sleep_until_us};

pub struct SourceFrame {
    pub frame: Frame,
    /// when the content was current on the PC (vblank / present / generated)
    pub t_us: u64,
    /// when we had it in hand
    pub t_ready_us: u64,
}

pub trait Source: Send {
    /// Next frame when due (see `Capture::next_frame`); None = nothing new for ~100 ms.
    fn next(&mut self, force: bool) -> Result<Option<SourceFrame>, String>;
    fn describe(&self) -> String;
    fn refresh_hz(&self) -> Option<f64>;
}

/// A screen capture backend → AVFrames (DMA-BUF → DRM_PRIME, CPU → wrapped buffer).
pub struct CaptureSource {
    cap: Box<dyn Capture>,
    #[cfg(target_os = "linux")]
    drm: Option<DrmFrames>,
    #[cfg(target_os = "macos")]
    videotoolbox: Option<crate::hw::VideoToolboxFrames>,
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    drm_device: String,
}
impl CaptureSource {
    /// `drm_device`: DRM node used to wrap DMA-BUFs (the GPU that owns them).
    pub fn new(cap: Box<dyn Capture>, drm_device: &str) -> CaptureSource {
        CaptureSource {
            cap,
            #[cfg(target_os = "linux")]
            drm: None,
            #[cfg(target_os = "macos")]
            videotoolbox: None,
            drm_device: drm_device.into(),
        }
    }
}
impl Source for CaptureSource {
    fn next(&mut self, force: bool) -> Result<Option<SourceFrame>, String> {
        let Some(f) = self.cap.next_frame(force)? else { return Ok(None) };
        let frame = match f.data {
            #[cfg(target_os = "linux")]
            FrameData::DmaBuf(d) => {
                if self.drm.is_none() {
                    self.drm = Some(DrmFrames::new(&self.drm_device)?);
                }
                self.drm.as_mut().unwrap().wrap(d)?
            }
            #[cfg(target_os = "macos")]
            FrameData::PixelBuffer(pixels) => {
                if self.videotoolbox.is_none() {
                    self.videotoolbox = Some(crate::hw::VideoToolboxFrames::new()?);
                }
                self.videotoolbox.as_mut().unwrap().wrap(pixels)?
            }
            FrameData::Cpu(c) => wrap_cpu(c)?,
        };
        Ok(Some(SourceFrame { frame, t_us: f.t_us, t_ready_us: f.t_ready_us }))
    }
    fn describe(&self) -> String {
        self.cap.describe()
    }
    fn refresh_hz(&self) -> Option<f64> {
        self.cap.refresh_hz()
    }
}

/// A source-only filter graph pulled frame by frame (lavfi test pattern).
struct PullGraph {
    graph: *mut AVFilterGraph,
    sink: *mut AVFilterContext,
}
impl Drop for PullGraph {
    fn drop(&mut self) {
        unsafe { avfilter_graph_free(&mut self.graph) };
    }
}
impl PullGraph {
    fn new(desc: &str) -> Result<PullGraph, String> {
        unsafe {
            let graph = avfilter_graph_alloc();
            let mut g = PullGraph { graph, sink: ptr::null_mut() };
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
            let mut inputs = avfilter_inout_alloc();
            (*inputs).name = av_strdup(cstr("out").as_ptr());
            (*inputs).filter_ctx = g.sink;
            let mut outputs: *mut AVFilterInOut = ptr::null_mut();
            let r = avfilter_graph_parse_ptr(graph, cstr(desc).as_ptr(), &mut inputs, &mut outputs, ptr::null_mut());
            avfilter_inout_free(&mut inputs);
            avfilter_inout_free(&mut outputs);
            check(r, &format!("lavfi '{desc}'"))?;
            check(avfilter_graph_config(graph, ptr::null_mut()), "lavfi config")?;
            Ok(g)
        }
    }
    fn pull(&mut self) -> Result<Frame, String> {
        let f = Frame::alloc();
        check(unsafe { av_buffersink_get_frame(self.sink, f.0) }, "lavfi pull")?;
        Ok(f)
    }
}
unsafe impl Send for PullGraph {}

/// testsrc2 (moving pattern + frame counter), generated when due.
pub struct TestSource {
    g: PullGraph,
    pacer: Pacer,
    desc: String,
}
impl TestSource {
    pub fn new(w: u32, h: u32, fps: u32, lavfi: Option<&str>) -> Result<TestSource, String> {
        let desc = lavfi.map(|s| s.to_string()).unwrap_or_else(|| format!("testsrc2=size={w}x{h}:rate={fps}"));
        Ok(TestSource { g: PullGraph::new(&desc)?, pacer: Pacer::new(fps as f64, None), desc })
    }
}
impl Source for TestSource {
    fn next(&mut self, _force: bool) -> Result<Option<SourceFrame>, String> {
        sleep_until_us(self.pacer.next_due());
        self.pacer.taken(mono_us());
        let frame = self.g.pull()?;
        // 'captured' = generated (pattern rendering time is not pipeline latency)
        let t = mono_us();
        Ok(Some(SourceFrame { frame, t_us: t, t_ready_us: t }))
    }
    fn describe(&self) -> String {
        format!("lavfi {}", self.desc)
    }
    fn refresh_hz(&self) -> Option<f64> {
        None
    }
}

/// A media file, decoded and looped forever, paced at the stream fps.
pub struct FileSource {
    fmt: *mut AVFormatContext,
    dec: *mut AVCodecContext,
    stream: i32,
    pkt: *mut AVPacket,
    pacer: Pacer,
    path: String,
}
unsafe impl Send for FileSource {}
impl Drop for FileSource {
    fn drop(&mut self) {
        unsafe {
            av_packet_free(&mut self.pkt);
            avcodec_free_context(&mut self.dec);
            avformat_close_input(&mut self.fmt);
        }
    }
}
impl FileSource {
    pub fn new(path: &str, fps: u32) -> Result<FileSource, String> {
        unsafe {
            let mut fmt: *mut AVFormatContext = ptr::null_mut();
            check(avformat_open_input(&mut fmt, cstr(path).as_ptr(), ptr::null(), ptr::null_mut()), &format!("open {path}"))?;
            check(avformat_find_stream_info(fmt, ptr::null_mut()), "find_stream_info")?;
            let mut codec: *const AVCodec = ptr::null();
            let stream = check(
                av_find_best_stream(fmt, AVMediaType::AVMEDIA_TYPE_VIDEO, -1, -1, &mut codec, 0),
                "no video stream",
            )?;
            let dec = avcodec_alloc_context3(codec);
            let st = *(*fmt).streams.add(stream as usize);
            avcodec_parameters_to_context(dec, (*st).codecpar);
            (*dec).thread_count = 0;
            check(avcodec_open2(dec, codec, ptr::null_mut()), "open decoder")?;
            Ok(FileSource { fmt, dec, stream, pkt: av_packet_alloc(), pacer: Pacer::new(fps as f64, None), path: path.into() })
        }
    }
    fn decode_one(&mut self) -> Result<Frame, String> {
        unsafe {
            let f = Frame::alloc();
            loop {
                let r = avcodec_receive_frame(self.dec, f.0);
                if r >= 0 {
                    return Ok(f);
                }
                if r != EAGAIN_ && r != AVERROR_EOF_ {
                    check(r, "decode")?;
                }
                if r == AVERROR_EOF_ {
                    avcodec_flush_buffers(self.dec);
                }
                let r = av_read_frame(self.fmt, self.pkt);
                if r == AVERROR_EOF_ {
                    // loop
                    av_seek_frame(self.fmt, self.stream, 0, AVSEEK_FLAG_BACKWARD as i32);
                    avcodec_flush_buffers(self.dec);
                    continue;
                }
                check(r, "read")?;
                if (*self.pkt).stream_index == self.stream {
                    let r = avcodec_send_packet(self.dec, self.pkt);
                    av_packet_unref(self.pkt);
                    if r < 0 && r != EAGAIN_ {
                        check(r, "send_packet")?;
                    }
                } else {
                    av_packet_unref(self.pkt);
                }
            }
        }
    }
}
impl Source for FileSource {
    fn next(&mut self, _force: bool) -> Result<Option<SourceFrame>, String> {
        let frame = self.decode_one()?;
        sleep_until_us(self.pacer.next_due());
        let t = mono_us();
        self.pacer.taken(t);
        Ok(Some(SourceFrame { frame, t_us: t, t_ready_us: t }))
    }
    fn describe(&self) -> String {
        format!("file {}", self.path)
    }
    fn refresh_hz(&self) -> Option<f64> {
        None
    }
}
