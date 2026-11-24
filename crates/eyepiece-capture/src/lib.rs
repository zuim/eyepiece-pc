//! Capture backends. Each produces [`Frame`]s stamped with the PC monotonic clock
//! ([`eyepiece_link::clock::mono_us`]) at the moment the content became current on the PC
//! display (DRM vblank, PipeWire buffer arrival, DXGI present time), not when we got
//! around to reading it.
//!
//! Frames are GPU buffers (DMA-BUF into VAAPI, native macOS pixel buffers into
//! VideoToolbox) or CPU images.
//! No backend queues frames: `next_frame` always returns the newest content.

#[cfg(target_os = "linux")]
pub mod drm;
#[cfg(target_os = "linux")]
pub mod kms;
#[cfg(all(target_os = "linux", feature = "pipewire"))]
pub mod portal;
#[cfg(all(target_os = "linux", feature = "x11"))]
pub mod x11;
#[cfg(windows)]
pub mod dxgi;
#[cfg(target_os = "macos")]
pub mod screencapturekit;

pub mod pacer;
pub mod layout;

#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;

/// fourcc helper (DRM_FORMAT_*).
pub const fn fourcc(s: &[u8; 4]) -> u32 {
    (s[0] as u32) | (s[1] as u32) << 8 | (s[2] as u32) << 16 | (s[3] as u32) << 24
}
pub const DRM_FORMAT_XRGB8888: u32 = fourcc(b"XR24");
pub const DRM_FORMAT_ARGB8888: u32 = fourcc(b"AR24");
pub const DRM_FORMAT_XBGR8888: u32 = fourcc(b"XB24");
pub const DRM_FORMAT_ABGR8888: u32 = fourcc(b"AB24");
pub const DRM_FORMAT_XRGB2101010: u32 = fourcc(b"XR30");
pub const DRM_FORMAT_ARGB2101010: u32 = fourcc(b"AR30");
pub const DRM_FORMAT_XBGR2101010: u32 = fourcc(b"XB30");
pub const DRM_FORMAT_ABGR2101010: u32 = fourcc(b"AB30");
pub const DRM_FORMAT_NV12: u32 = fourcc(b"NV12");
pub const DRM_FORMAT_MOD_INVALID: u64 = 0x00ff_ffff_ffff_ffff;
pub const DRM_FORMAT_MOD_LINEAR: u64 = 0;

pub fn fourcc_str(f: u32) -> String {
    f.to_le_bytes().iter().map(|&c| if c.is_ascii_graphic() { c as char } else { '?' }).collect()
}

/// The alpha channel of a scanned-out framebuffer is never used: map the alpha variants
/// to their X (padding) twins, which have the identical memory layout. This is what
/// kmsfix.c did for ffmpeg; VAAPI imports XR30/XB30 but not AR30/AB30.
pub fn opaque_fourcc(f: u32) -> u32 {
    match f {
        DRM_FORMAT_ARGB2101010 => DRM_FORMAT_XRGB2101010,
        DRM_FORMAT_ABGR2101010 => DRM_FORMAT_XBGR2101010,
        DRM_FORMAT_ARGB8888 => DRM_FORMAT_XRGB8888,
        DRM_FORMAT_ABGR8888 => DRM_FORMAT_XBGR8888,
        x => x,
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct DmaObject {
    pub fd: OwnedFd,
    pub size: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct DmaPlane {
    pub object: usize,
    pub offset: u32,
    pub pitch: u32,
}

/// A GPU buffer shared as DMA-BUF (KMS framebuffer, PipeWire dmabuf).
#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct DmaBuf {
    pub width: u32,
    pub height: u32,
    /// DRM fourcc, already made opaque (see [`opaque_fourcc`])
    pub fourcc: u32,
    pub modifier: u64,
    pub objects: Vec<DmaObject>,
    pub planes: Vec<DmaPlane>,
}

/// Packed RGB layouts a CPU capture can deliver (byte order in memory).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuFormat {
    /// B,G,R,X bytes (DRM XR24, X11 depth 24, DXGI B8G8R8A8)
    Bgrx,
    /// R,G,B,X bytes (DRM XB24)
    Rgbx,
    /// 32-bit little-endian x2r10g10b10 (DRM XR30)
    X2Rgb10,
    /// 32-bit little-endian x2b10g10r10 (DRM XB30)
    X2Bgr10,
}

#[derive(Debug)]
pub struct CpuImage {
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub format: CpuFormat,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum FrameData {
    #[cfg(target_os = "linux")]
    DmaBuf(DmaBuf),
    #[cfg(target_os = "macos")]
    PixelBuffer(screencapturekit::CVPixelBuffer),
    Cpu(CpuImage),
}

#[derive(Debug)]
pub struct Frame {
    /// PC monotonic µs at which this content was on the PC screen (vblank / present /
    /// buffer arrival)
    pub t_us: u64,
    /// µs at which the backend had the frame in hand (t_ready - t_us = capture wait)
    pub t_ready_us: u64,
    pub data: FrameData,
}

impl Frame {
    pub fn size(&self) -> (u32, u32) {
        match &self.data {
            #[cfg(target_os = "linux")]
            FrameData::DmaBuf(d) => (d.width, d.height),
            #[cfg(target_os = "macos")]
            FrameData::PixelBuffer(pixels) => (pixels.width() as u32, pixels.height() as u32),
            FrameData::Cpu(c) => (c.width, c.height),
        }
    }
}

/// When to take a frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncMode {
    /// at the PC display's vblank, only when the content changed (KMS: page flip seen)
    Vblank,
    /// poll the plane every ~1 ms and take a new framebuffer as soon as the compositor
    /// commits it (before it is even scanned out)
    Poll,
    /// free-running timer at the target fps (like ffmpeg's grabbers)
    Timer,
}

impl std::str::FromStr for SyncMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "vblank" => Ok(SyncMode::Vblank),
            "poll" => Ok(SyncMode::Poll),
            "timer" => Ok(SyncMode::Timer),
            _ => Err(format!("unknown sync mode {s} (vblank|poll|timer)")),
        }
    }
}

pub trait Capture: Send {
    /// Block until the next frame is due and return it. `force`: return the current
    /// content even if unchanged (an IDR is needed). Returns Ok(None) after ~100 ms
    /// without new content (static screen) so the caller can do housekeeping.
    fn next_frame(&mut self, force: bool) -> Result<Option<Frame>, String>;
    /// human-readable description (format, size, refresh)
    fn describe(&self) -> String;
    /// refresh rate of the captured display, if known
    fn refresh_hz(&self) -> Option<f64>;
}
