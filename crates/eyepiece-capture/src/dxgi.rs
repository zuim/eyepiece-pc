//! Windows: DXGI Desktop Duplication. Frames arrive when DWM presents (synchronised to
//! the display by construction) and are stamped with DXGI's LastPresentTime (QPC, same
//! clock as `eyepiece_link::clock::mono_us` on Windows).
//!
//! The frames carry no hardware cursor: the pointer comes separately (position and shape) and is
//! drawn into the frame ([`crate::cursor`]) as `--dxgi-cursor` says.
//!
//! This version copies through a staging texture
//! into a CPU BGRA frame (~1–2 ms at 1080p); the zero-copy path (shared D3D11 texture →
//! AV_PIX_FMT_D3D11 → h264_nvenc/h264_amf/h264_qsv) is the obvious next step.

use crate::cursor::{self, CursorMode, PointerShape, ShapeKind};
use crate::pacer::Pacer;
use crate::*;
use eyepiece_link::clock::{mono_us, qpc_to_us};
use windows::core::Interface;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_UNKNOWN;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

pub struct DxgiCapture {
    device: ID3D11Device,
    ctx: ID3D11DeviceContext,
    output: IDXGIOutput1,
    dup: IDXGIOutputDuplication,
    staging: Option<(ID3D11Texture2D, u32, u32)>,
    pacer: Pacer,
    qpc_freq: i64,
    desc: String,
    cursor: CursorMode,
    shape: Option<PointerShape>,
    /// top-left of the pointer shape in output coordinates, and whether it is on this output
    pointer: (i32, i32, bool),
    drawn: Option<bool>,
    /// the output's top-left on the virtual desktop (for the GetCursorInfo fallback)
    origin: (i32, i32),
}

// COM pointers are only used from the capture thread
unsafe impl Send for DxgiCapture {}

fn qpc_freq() -> i64 {
    extern "system" {
        fn QueryPerformanceFrequency(f: *mut i64) -> i32;
    }
    let mut f = 1i64;
    unsafe { QueryPerformanceFrequency(&mut f) };
    f
}

impl DxgiCapture {
    /// `cursor`: Desktop Duplication frames carry no pointer unless the system drew a software
    /// cursor into the desktop image; [`CursorMode::Auto`] draws one only when the image lacks it.
    pub fn open(adapter: u32, output_idx: u32, fps: f64, cursor: CursorMode) -> Result<DxgiCapture, String> {
        unsafe {
            let e = |x: windows::core::Error| format!("DXGI: {x}");
            let factory: IDXGIFactory1 = CreateDXGIFactory1().map_err(e)?;
            let ad: IDXGIAdapter1 = factory.EnumAdapters1(adapter).map_err(e)?;
            let out: IDXGIOutput = ad.EnumOutputs(output_idx).map_err(e)?;
            let output: IDXGIOutput1 = out.cast().map_err(e)?;
            let mut device = None;
            let mut ctx = None;
            D3D11CreateDevice(
                &ad,
                D3D_DRIVER_TYPE_UNKNOWN,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut ctx),
            )
            .map_err(e)?;
            let device = device.ok_or("no D3D11 device")?;
            let ctx = ctx.ok_or("no D3D11 context")?;
            let dup = output.DuplicateOutput(&device).map_err(e)?;
            let od = out.GetDesc().map_err(e)?;
            let r = od.DesktopCoordinates;
            let desc = format!("dxgi adapter {adapter} output {output_idx} {}x{}", r.right - r.left, r.bottom - r.top);
            Ok(DxgiCapture { device, ctx, output, dup, staging: None, pacer: Pacer::new(fps, None), qpc_freq: qpc_freq(), desc,
                cursor, shape: None, pointer: (0, 0, false), drawn: None, origin: (r.left, r.top) })
        }
    }

    fn staging(&mut self, w: u32, h: u32) -> Result<ID3D11Texture2D, String> {
        if let Some((t, sw, sh)) = &self.staging {
            if (*sw, *sh) == (w, h) {
                return Ok(t.clone());
            }
        }
        let d = D3D11_TEXTURE2D_DESC {
            Width: w,
            Height: h,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: 0,
            CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
            MiscFlags: 0,
        };
        let mut t = None;
        unsafe { self.device.CreateTexture2D(&d, None, Some(&mut t)) }.map_err(|e| format!("staging texture: {e}"))?;
        let t = t.ok_or("staging texture")?;
        self.staging = Some((t.clone(), w, h));
        Ok(t)
    }
}

impl DxgiCapture {
    /// The pointer shape of this frame (only reported when it changed); keeps the old one on error.
    unsafe fn pointer_shape(&mut self, size: u32) -> Option<PointerShape> {
        let mut data = vec![0u8; size as usize];
        let mut required = 0u32;
        let mut shape = DXGI_OUTDUPL_POINTER_SHAPE_INFO::default();
        let result =
            unsafe { self.dup.GetFramePointerShape(size, data.as_mut_ptr().cast(), &mut required, &mut shape) };
        if result.is_err() {
            return self.shape.take();
        }
        let kind = match DXGI_OUTDUPL_POINTER_SHAPE_TYPE(shape.Type as i32) {
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME => ShapeKind::Monochrome,
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR => ShapeKind::Color,
            DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR => ShapeKind::MaskedColor,
            _ => return None,
        };
        data.truncate(required.min(size) as usize);
        Some(PointerShape { kind, width: shape.Width, height: shape.Height, pitch: shape.Pitch, data })
    }

    /// Draw the pointer into a captured frame as the mode says.
    fn draw_pointer(&mut self, data: &mut [u8], width: u32, height: u32, stride: usize) {
        let (left, top, visible) = self.pointer;
        let Some(shape) = self.shape.as_ref().filter(|_| visible && self.cursor != CursorMode::Off) else {
            self.drawn = None;
            return;
        };
        let mut image = cursor::Image { data, width, height, stride };
        let draw = self.cursor == CursorMode::On || !cursor::already_shown(&mut image, shape, left, top);
        if draw {
            cursor::draw(&mut image, shape, left, top);
        }
        self.drawn = Some(draw);
    }

    /// Whether the pointer went into the last frame: `None` when it is not on this output (or
    /// drawing is off), `Some(false)` when the image already showed it (auto).
    pub fn pointer_drawn(&self) -> Option<bool> {
        self.drawn
    }

    /// Top-left of the pointer shape in output coordinates (last reported).
    pub fn pointer_position(&self) -> (i32, i32) {
        (self.pointer.0, self.pointer.1)
    }
}

impl Capture for DxgiCapture {
    fn next_frame(&mut self, force: bool) -> Result<Option<Frame>, String> {
        unsafe {
            let mut info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut res: Option<IDXGIResource> = None;
            match self.dup.AcquireNextFrame(100, &mut info, &mut res) {
                Ok(()) => {}
                Err(e) if e.code() == DXGI_ERROR_WAIT_TIMEOUT => return Ok(None),
                Err(e) if e.code() == DXGI_ERROR_ACCESS_LOST => {
                    // mode change / secure desktop: recreate the duplication
                    self.dup = self.output.DuplicateOutput(&self.device).map_err(|e| format!("DXGI: {e}"))?;
                    return Ok(None);
                }
                Err(e) => return Err(format!("AcquireNextFrame: {e}")),
            }
            let mouse = info.LastMouseUpdateTime != 0;
            if mouse {
                let p = info.PointerPosition;
                self.pointer = (p.Position.x, p.Position.y, p.Visible.as_bool());
            }
            if info.PointerShapeBufferSize > 0 {
                self.shape = self.pointer_shape(info.PointerShapeBufferSize);
            } else if self.shape.is_none() && self.cursor != CursorMode::Off {
                // Desktop Duplication reports the pointer only once it moves: until then, ask the
                // window manager (also the shape, at its own size, until Duplication sends one)
                if let Some((shape, left, top, visible)) = system_pointer(self.origin) {
                    self.shape = Some(shape);
                    self.pointer = (left, top, visible);
                }
            }
            let t = if info.LastPresentTime != 0 {
                qpc_to_us(info.LastPresentTime, self.qpc_freq)
            } else if mouse {
                qpc_to_us(info.LastMouseUpdateTime, self.qpc_freq)
            } else {
                mono_us()
            };
            // LastPresentTime == 0: only the mouse moved, which is a new frame only when we draw it
            let take = (info.LastPresentTime != 0 || (mouse && self.cursor != CursorMode::Off) || force) && self.pacer.due(t);
            let result = if take {
                self.pacer.taken(t);
                let tex: ID3D11Texture2D = res.ok_or("no frame resource")?.cast().map_err(|e| e.to_string())?;
                let mut d = D3D11_TEXTURE2D_DESC::default();
                tex.GetDesc(&mut d);
                let st = self.staging(d.Width, d.Height)?;
                self.ctx.CopyResource(&st, &tex);
                let mut m = D3D11_MAPPED_SUBRESOURCE::default();
                self.ctx.Map(&st, 0, D3D11_MAP_READ, 0, Some(&mut m)).map_err(|e| format!("Map: {e}"))?;
                let stride = d.Width as usize * 4;
                let mut data = vec![0u8; stride * d.Height as usize];
                for y in 0..d.Height as usize {
                    let src = std::slice::from_raw_parts((m.pData as *const u8).add(y * m.RowPitch as usize), stride);
                    data[y * stride..(y + 1) * stride].copy_from_slice(src);
                }
                self.ctx.Unmap(&st, 0);
                self.draw_pointer(&mut data, d.Width, d.Height, stride);
                Some(Frame {
                    t_us: t,
                    t_ready_us: mono_us(),
                    data: FrameData::Cpu(CpuImage { width: d.Width, height: d.Height, stride, format: CpuFormat::Bgrx, data }),
                })
            } else {
                None
            };
            let _ = self.dup.ReleaseFrame();
            Ok(result)
        }
    }
    fn describe(&self) -> String {
        self.desc.clone()
    }
    fn refresh_hz(&self) -> Option<f64> {
        None
    }
}

/// The current pointer from GetCursorInfo and its icon bitmaps: shape, top-left in output
/// coordinates (`origin`: the output's top-left on the virtual desktop) and whether it shows.
/// Physical pixels: the thread is per-monitor DPI aware for the call.
fn system_pointer(origin: (i32, i32)) -> Option<(PointerShape, i32, i32, bool)> {
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::UI::HiDpi::{SetThreadDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};
    use windows::Win32::UI::WindowsAndMessaging::*;
    unsafe {
        let previous = SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        let mut ci = CURSORINFO { cbSize: std::mem::size_of::<CURSORINFO>() as u32, ..Default::default() };
        let got = GetCursorInfo(&mut ci).is_ok();
        if !previous.is_invalid() {
            SetThreadDpiAwarenessContext(previous);
        }
        if !got || ci.hCursor.is_invalid() {
            return None;
        }
        let mut ii = ICONINFO::default();
        GetIconInfo(HICON(ci.hCursor.0), &mut ii).ok()?;
        let bits = |bitmap: HBITMAP, bpp: u16| -> Option<(u32, u32, Vec<u8>)> {
            let mut bm = BITMAP::default();
            if bitmap.is_invalid()
                || GetObjectW(HGDIOBJ(bitmap.0), std::mem::size_of::<BITMAP>() as i32, Some((&mut bm as *mut BITMAP).cast())) == 0
            {
                return None;
            }
            let (w, h) = (bm.bmWidth as u32, bm.bmHeight as u32);
            let pitch = (w * bpp as u32).div_ceil(32) * 4;
            let mut data = vec![0u8; (pitch * h) as usize];
            let header = BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w as i32,
                biHeight: -(h as i32), // top-down
                biPlanes: 1,
                biBitCount: bpp,
                ..Default::default()
            };
            // a 1 bpp DIB needs its two-colour table: BITMAPINFO has room for one entry only
            let mut with_palette = [0u8; std::mem::size_of::<BITMAPINFOHEADER>() + 8];
            std::ptr::copy_nonoverlapping((&header as *const BITMAPINFOHEADER).cast::<u8>(), with_palette.as_mut_ptr(), std::mem::size_of::<BITMAPINFOHEADER>());
            let dc = GetDC(None);
            let lines = GetDIBits(dc, bitmap, 0, h, Some(data.as_mut_ptr().cast()), with_palette.as_mut_ptr().cast(), DIB_RGB_COLORS);
            ReleaseDC(None, dc);
            (lines == h as i32).then_some((w, h, data))
        };
        let mask = bits(ii.hbmMask, 1);
        let color = if ii.hbmColor.is_invalid() { None } else { bits(ii.hbmColor, 32) };
        let _ = DeleteObject(HGDIOBJ(ii.hbmMask.0));
        if !ii.hbmColor.is_invalid() {
            let _ = DeleteObject(HGDIOBJ(ii.hbmColor.0));
        }
        let shape = match (color, mask) {
            // monochrome: the mask bitmap holds the AND and the XOR mask
            (None, Some((w, h, data))) => {
                PointerShape { kind: ShapeKind::Monochrome, width: w, height: h, pitch: w.div_ceil(32) * 4, data }
            }
            (Some((w, h, mut data)), mask) => {
                if data.chunks(4).any(|p| p[3] != 0) {
                    PointerShape { kind: ShapeKind::Color, width: w, height: h, pitch: w * 4, data }
                } else {
                    // no alpha: the AND mask says where the colour XORs instead of replacing
                    let (_, _, mask) = mask?;
                    let mask_pitch = w.div_ceil(32) as usize * 4;
                    for y in 0..h as usize {
                        for x in 0..w as usize {
                            let and = mask.get(y * mask_pitch + x / 8).is_some_and(|b| b & (0x80 >> (x % 8)) != 0);
                            data[(y * w as usize + x) * 4 + 3] = if and { 0xff } else { 0 };
                        }
                    }
                    PointerShape { kind: ShapeKind::MaskedColor, width: w, height: h, pitch: w * 4, data }
                }
            }
            (None, None) => return None,
        };
        let left = ci.ptScreenPos.x - ii.xHotspot as i32 - origin.0;
        let top = ci.ptScreenPos.y - ii.yHotspot as i32 - origin.1;
        Some((shape, left, top, ci.flags.0 & CURSOR_SHOWING.0 != 0))
    }
}
