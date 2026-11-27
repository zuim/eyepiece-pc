//! One streaming session: source → convert → encode → bridge, plus stats, preview, dump.
//!
//! Single pipeline thread, no queues: a frame is captured when due (display refresh),
//! converted, encoded and its AU written to the socket before the next one is taken.
//! If anything is slow, the next refresh is simply missed (drop, never queue).

use crate::preview::Preview;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use eyepiece_av::encoder;
use eyepiece_av::{ConvertConfig, Converter, Encoder, EncoderConfig, Packet, PackedAlpha, Source};
use eyepiece_capture::SyncMode;
use eyepiece_link::clock::mono_us;
use eyepiece_link::proto;
use eyepiece_link::stats::{Interval, Snapshot};
use eyepiece_link::{Link, LinkOptions, LogFn};

/// Goggles panel dimensions. Sources are scaled on the PC; packed overlays use
/// their negotiated dimensions instead.
pub const PANEL_W: u32 = 1920;
pub const PANEL_H: u32 = 1080;

#[derive(Debug, Clone)]
pub enum SourceKind {
    #[cfg(target_os = "linux")]
    Kms { device: String, plane: u32, output: Option<String>, simulate: bool },
    #[cfg(all(target_os = "linux", feature = "x11"))]
    X11 { display: Option<String>, region: Option<(u32, u32, i32, i32)>, vblank_device: String },
    #[cfg(all(target_os = "linux", feature = "pipewire"))]
    PipeWire { restore_token_file: Option<PathBuf>, cursor: bool },
    #[cfg(windows)]
    Dxgi { adapter: u32, output: u32, cursor: eyepiece_capture::cursor::CursorMode },
    #[cfg(target_os = "macos")]
    ScreenCaptureKit { display_id: Option<u32>, cursor: bool },
    Test { lavfi: Option<String> },
    File { path: String },
}

#[derive(Clone, Copy, Debug)]
pub struct PaneConfig {
    pub pane: proto::Pane,
    pub alpha: PackedAlpha,
}

#[derive(Clone)]
pub struct Config {
    pub source: SourceKind,
    pub sync: SyncMode,
    /// DRM node that owns captured DMA-BUFs (KMS device / PipeWire's GPU)
    pub drm_device: String,
    /// encoder settings; `fps` and `gop` may be lowered to the source refresh
    pub enc: EncoderConfig,
    pub letterbox: bool,
    pub gpu_scale_cpu_input: bool,
    pub vf: Option<String>,
    pub scale_mode: String,
    /// false: head tracking only
    pub video: bool,
    /// None: no bridge (encode only, e.g. with `dump`)
    pub link: Option<LinkOptions>,
    pub reconnect: bool,
    pub stop_bridge: bool,
    pub dump: Option<PathBuf>,
    pub preview: Option<(i32, i32)>,
    pub stats_interval: f64,
    pub stats_window: Option<f64>,
    pub max_frames: Option<u64>,
    pub duration_s: Option<f64>,
    /// side-by-side stereo (2560x720, the goggles' 3D mode) instead of 1920x1080 mono
    pub stereo: bool,
    pub pane: Option<PaneConfig>,
    pub gui_opacity: u8,
    pub pane_opacity: u8,
}

impl Config {
    /// Encoded frame size: packed overlay, panel (mono), or side-by-side stereo.
    pub fn frame_size(&self) -> (u32, u32) {
        if let Some(pane) = self.pane {
            let multiplier = if pane.pane.format == proto::PaneFormat::Bgra { 2 } else { 1 };
            (u32::from(pane.pane.width) * multiplier, u32::from(pane.pane.height))
        } else if self.stereo {
            (2 * proto::SBS_EYE_W, proto::SBS_EYE_H)
        } else {
            (PANEL_W, PANEL_H)
        }
    }
}

/// Remote control for a running session (CLI stdin, a UI, signals).
#[derive(Default)]
pub struct Control {
    pub stop: AtomicBool,
    pub recenter: AtomicBool,
    pub idr: AtomicBool,
    /// pending head-tracking change: 0 = none, 1 = off, 2 = on (see [`Control::set_head_tracking`])
    pub head_tracking: AtomicU8,
    pub(crate) capture_updates: Mutex<VecDeque<CaptureUpdate>>,
    pub(crate) gui_updates: Mutex<VecDeque<GuiUpdate>>,
}

pub(crate) struct GuiUpdate {
    pub opacity: u8,
    pub pane: bool,
    pub complete: Box<dyn FnOnce(Result<(), String>) + Send>,
}

pub(crate) struct CaptureUpdate {
    pub source: Option<SourceKind>,
    pub alpha: Option<PackedAlpha>,
    pub pane: Option<PaneConfig>,
    pub complete: Box<dyn FnOnce(Result<(), String>) + Send>,
}

impl Control {
    /// Turn the goggles' head tracking on/off while running; kept across reconnects.
    pub fn set_head_tracking(&self, on: bool) {
        self.head_tracking.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    }

    pub(crate) fn queue_capture_update(&self, mut update: CaptureUpdate) {
        let mut updates = self.capture_updates.lock().unwrap();
        if let Some(previous) = updates.pop_back() {
            let CaptureUpdate { source, alpha, pane, complete } = previous;
            update.source = update.source.or(source);
            if update.pane.is_none() {
                update.pane = pane;
                if let (Some(pane), Some(alpha)) = (&mut update.pane, update.alpha) {
                    pane.alpha = alpha;
                }
            }
            update.alpha = update.alpha.or(alpha);
            let next_complete = update.complete;
            update.complete = Box::new(move |result| {
                complete(result.clone());
                next_complete(result);
            });
        }
        updates.push_back(update);
    }
}

struct StatsWindow {
    duration: f64,
    elapsed: f64,
    intervals: VecDeque<(f64, Interval)>,
}

impl StatsWindow {
    fn new(duration: f64) -> Self {
        Self { duration, elapsed: 0.0, intervals: VecDeque::new() }
    }

    fn clear(&mut self) {
        self.intervals.clear();
        self.elapsed = 0.0;
    }

    fn push(&mut self, dt: f64, interval: Interval) -> (Interval, f64) {
        self.intervals.push_back((dt, interval));
        self.elapsed += dt;
        while self.intervals.len() > 1 && self.elapsed - self.intervals.front().unwrap().0 >= self.duration {
            self.elapsed -= self.intervals.pop_front().unwrap().0;
        }
        let mut combined = Interval::default();
        for (_, interval) in &self.intervals {
            combined.extend(interval);
        }
        (combined, self.elapsed)
    }
}

/// Encoded-stream tap: every AU right after it was written to the socket (`key` = IDR).
/// Runs on the encode/send thread: it must only copy and hand off (bounded, non-blocking).
pub type AuTap = Arc<dyn Fn(&[u8], bool, bool, bool) + Send + Sync>;
/// Called with the link on every (re)connect, and with `None` when it is gone.
pub type LinkFn = Arc<dyn Fn(Option<Arc<Link>>) + Send + Sync>;

pub struct Events {
    pub log: LogFn,
    pub stats: Arc<dyn Fn(&Snapshot) + Send + Sync>,
    pub au: Option<AuTap>,
    pub on_link: Option<LinkFn>,
}

impl Events {
    pub fn new(log: LogFn, stats: Arc<dyn Fn(&Snapshot) + Send + Sync>) -> Events {
        Events { log, stats, au: None, on_link: None }
    }
    fn link_changed(&self, l: Option<Arc<Link>>) {
        if let Some(f) = &self.on_link {
            f(l);
        }
    }
}

fn make_source(cfg: &Config, _encoder_family: encoder::Family) -> Result<Box<dyn Source>, String> {
    #[cfg(any(target_os = "linux", windows))]
    let fps = cfg.enc.fps as f64;
    Ok(match &cfg.source {
        #[cfg(target_os = "linux")]
        SourceKind::Kms { device, plane, output, simulate } => {
            let k = eyepiece_capture::kms::KmsCapture::open(&eyepiece_capture::kms::KmsConfig {
                device: device.clone(),
                plane: *plane,
                output: output.clone(),
                fps,
                sync: cfg.sync,
                simulate: *simulate,
            })?;
            if *simulate {
                Box::new(eyepiece_av::simkms::SimKmsSource::new(k, device, &cfg.enc.vaapi_device)?)
            } else {
                Box::new(eyepiece_av::CaptureSource::new(Box::new(k), device))
            }
        }
        #[cfg(all(target_os = "linux", feature = "x11"))]
        SourceKind::X11 { display, region, vblank_device } => {
            let x = eyepiece_capture::x11::X11Capture::open(&eyepiece_capture::x11::X11Config {
                display: display.clone(),
                region: *region,
                fps,
                sync: cfg.sync,
                vblank_device: vblank_device.clone(),
            })?;
            Box::new(eyepiece_av::CaptureSource::new(Box::new(x), &cfg.drm_device))
        }
        #[cfg(all(target_os = "linux", feature = "pipewire"))]
        SourceKind::PipeWire { restore_token_file, cursor } => {
            let p = eyepiece_capture::portal::PipeWireCapture::open(&eyepiece_capture::portal::PipeWireConfig {
                fps,
                restore_token_file: restore_token_file.clone(),
                cursor: *cursor,
            })?;
            Box::new(eyepiece_av::CaptureSource::new(Box::new(p), &cfg.drm_device))
        }
        #[cfg(windows)]
        SourceKind::Dxgi { adapter, output, cursor } => {
            let d = eyepiece_capture::dxgi::DxgiCapture::open(*adapter, *output, fps, *cursor)?;
            Box::new(eyepiece_av::CaptureSource::new(Box::new(d), ""))
        }
        #[cfg(target_os = "macos")]
        SourceKind::ScreenCaptureKit { display_id, cursor } => {
            let capture = if cfg.pane.is_none_or(|p| p.pane.format == proto::PaneFormat::OpaqueBgra) && _encoder_family == encoder::Family::VideoToolbox && cfg.vf.is_none() && cfg.gpu_scale_cpu_input {
                eyepiece_capture::screencapturekit::ScreenCapture::open_native(
                    *display_id, cfg.enc.fps, *cursor, cfg.frame_size(), cfg.letterbox,
                )?
            } else {
                eyepiece_capture::screencapturekit::ScreenCapture::open(*display_id, cfg.enc.fps, *cursor)?
            };
            Box::new(eyepiece_av::CaptureSource::new(Box::new(capture), ""))
        }
        SourceKind::Test { lavfi } => {
            let (w, h) = cfg.pane.map_or_else(|| cfg.frame_size(), |p| (u32::from(p.pane.width), u32::from(p.pane.height)));
            // stereo test pattern: the same scene per eye, the right eye mirrored, so a
            // swapped or duplicated eye is obvious
            let sbs = format!(
                "testsrc2=size={}x{h}:rate={},split[l][r];[r]hflip[rf];[l][rf]hstack=inputs=2",
                w / 2,
                cfg.enc.fps
            );
            let lavfi = lavfi.clone().or(cfg.stereo.then_some(sbs));
            Box::new(eyepiece_av::TestSource::new(w, h, cfg.enc.fps, lavfi.as_deref())?)
        }
        SourceKind::File { path } => Box::new(eyepiece_av::FileSource::new(path, cfg.enc.fps)?),
    })
}

struct Pending {
    pts: i64,
    t_cap: u64,
    t_conv: u64,
    t_conv_done: u64,
    t_enc_in: u64,
}

/// The complete stream description the bridge needs; the encoded size follows from it.
fn stream_config(cfg: &Config, codec: u8) -> proto::StreamConfig {
    proto::StreamConfig {
        codec,
        layout: if cfg.stereo { proto::EP_LAYOUT_SBS } else { proto::EP_LAYOUT_MONO },
        pane: cfg.pane.map(|p| p.pane),
    }
}

/// Connect and configure the stream (the bridge refuses video until it is acknowledged).
fn connect(cfg: &mut Config, enc: &EncoderConfig, ev: &Events) -> Result<Option<Arc<Link>>, String> {
    let Some(lo) = cfg.link.clone() else { return Ok(None) };
    let l = Link::connect(&lo, ev.log.clone())?;
    if let Some(options) = cfg.link.as_mut() {
        options.transport = l.selected_transport();
    }
    (ev.log)(format!("data link: {}", l.transport_name()));
    let t0 = std::time::Instant::now();
    while l.info().is_none() && l.alive() && t0.elapsed() < Duration::from_secs(2) {
        std::thread::sleep(Duration::from_millis(20));
    }
    if l.info().is_none() {
        l.close();
        return Err("bridge did not provide compatible protobuf protocol information; update the bridge".into());
    }
    let flags = l.info().map_or(0, |i| i.flags);
    let configured = configure_link(&l, cfg, enc, flags);
    if let Err(error) = configured {
        l.close();
        return Err(error);
    }
    ev.link_changed(Some(l.clone()));
    Ok(Some(l))
}

/// A connection starts with the goggles' UI hidden (GUI opacity 0) and a fully opaque pane; only
/// other values are sent, and they need the bridge's capability.
fn configure_link(l: &Link, cfg: &Config, enc: &EncoderConfig, flags: u32) -> Result<(), String> {
    if cfg.gui_opacity != 0 {
        if flags & proto::EP_INFO_GUI_ALPHA == 0 {
            return Err("this bridge cannot show the goggles' UI (no GUI_ALPHA); use --gui-opacity 0".into());
        }
        l.set_gui_alpha(gui_alpha(cfg.gui_opacity), Duration::from_secs(3)).map_err(|e| format!("GUI opacity: {e}"))?;
    }
    if !cfg.video {
        return Ok(());
    }
    if cfg.stereo && flags & proto::EP_INFO_STEREO == 0 {
        return Err("this bridge offers no stereo now (a bridge started from the goggles' menu shows mono only)".into());
    }
    l.configure(stream_config(cfg, enc.codec.wire()), Duration::from_secs(5))
        .map_err(|e| format!("stream configuration: {e}"))?;
    if cfg.pane.is_some() && cfg.pane_opacity != 100 {
        if flags & proto::EP_INFO_PANE_ALPHA == 0 {
            return Err("this bridge cannot change the pane opacity (no PANE_ALPHA)".into());
        }
        l.set_pane_alpha(gui_alpha(cfg.pane_opacity), Duration::from_secs(3)).map_err(|e| format!("pane opacity: {e}"))?;
    }
    Ok(())
}

fn gui_alpha(opacity: u8) -> u8 {
    ((u16::from(opacity) * 255 + 50) / 100) as u8
}

pub fn run(mut cfg: Config, ctl: Arc<Control>, ev: Events) -> Result<(), String> {
    if cfg.gui_opacity > 100 || cfg.pane_opacity > 100 {
        return Err("GUI and pane opacity must be between 0 and 100".into());
    }
    if cfg.pane_opacity != 100 && (!cfg.video || cfg.pane.is_none()) {
        return Err("pane opacity requires GFX1 video output".into());
    }
    if let Some(pane) = cfg.pane {
        if cfg.stereo || !cfg.video || (pane.pane.format == proto::PaneFormat::Bgra && cfg.vf.is_some())
            || !matches!(pane.pane.format, proto::PaneFormat::Bgra | proto::PaneFormat::OpaqueBgra)
        {
            return Err("GFX1 output requires mono video; transparency does not support custom filters".into());
        }
        pane.pane.payload(proto::Info {
            fb_w: PANEL_W as u16, fb_h: PANEL_H as u16,
            flags: proto::EP_INFO_PANE_BGRA | proto::EP_INFO_PANE_OPAQUE_BGRA, ..proto::Info::default()
        }).map_err(str::to_string)?;
        pane.pane.encoded_size().map_err(str::to_string)?;
    }
    let log = ev.log.clone();
    let pc_stats = Arc::new(Mutex::new(Interval::default()));

    let enc_name = if cfg.video { encoder::resolve(&cfg.enc.encoder, cfg.enc.codec, &cfg.enc.vaapi_device)? } else { String::new() };
    let family = encoder::family(&enc_name);

    // ---- source first: its refresh rate decides the real frame rate ----
    let mut src = if cfg.video { Some(make_source(&cfg, family)?) } else { None };
    if let Some(s) = &src {
        log(format!("source: {}", s.describe()));
        if let Some(r) = s.refresh_hz() {
            let r_fps = r.round() as u32;
            if cfg.sync != SyncMode::Timer && r_fps < cfg.enc.fps {
                log(format!(
                    "source refreshes at {r:.2} Hz < --fps {}: encoding at {r_fps} fps (rate control needs the real rate)",
                    cfg.enc.fps
                ));
                let gop_s = cfg.enc.gop as f64 / cfg.enc.fps as f64;
                cfg.enc.fps = r_fps;
                cfg.enc.gop = (gop_s * r_fps as f64).round().max(1.0) as u32;
            }
        }
    }
    let enc_settings = cfg.enc.clone();
    let link: Arc<Mutex<Option<Arc<Link>>>> =
        Arc::new(Mutex::new(connect(&mut cfg, &enc_settings, &ev)?));

    // ---- stats thread ----
    {
        let link = link.clone();
        let pc_stats = pc_stats.clone();
        let ctl = ctl.clone();
        let interval = cfg.stats_interval;
        let window = cfg.stats_window;
        let on_stats = ev.stats.clone();
        std::thread::Builder::new()
            .name("ep-stats".into())
            .spawn(move || {
                let mut t_last = mono_us();
                let mut history = window.map(StatsWindow::new);
                let mut previous_link: Option<Arc<Link>> = None;
                while !ctl.stop.load(Ordering::Relaxed) {
                    let elapsed = (mono_us() - t_last) as f64 / 1e6;
                    if elapsed < interval {
                        std::thread::sleep(Duration::from_secs_f64((interval - elapsed).min(0.1)));
                    }
                    if ctl.stop.load(Ordering::Relaxed) {
                        break;
                    }
                    let now = mono_us();
                    if (now - t_last) as f64 / 1e6 < interval {
                        continue;
                    }
                    let dt = (now - t_last) as f64 / 1e6;
                    t_last = now;
                    let l = link.lock().unwrap().clone();
                    if !match (&previous_link, &l) {
                        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                        (None, None) => true,
                        _ => false,
                    } {
                        if let Some(history) = &mut history {
                            history.clear();
                        }
                    }
                    previous_link = l.clone();
                    let mut iv = l
                        .as_ref()
                        .map(|l| l.shared.stats.lock().unwrap().reset_keep_totals())
                        .unwrap_or_default();
                    let pc = pc_stats.lock().unwrap().reset_keep_totals();
                    iv.sent = pc.sent;
                    iv.bytes = pc.bytes;
                    iv.captured = pc.captured;
                    iv.skipped = pc.skipped;
                    iv.idr = pc.idr;
                    iv.cap_send = pc.cap_send;
                    iv.cap_enc = pc.cap_enc;
                    iv.enc = pc.enc;
                    let rtt = l.as_ref().and_then(|l| l.shared.rtt_us());
                    let pose = l.as_ref().and_then(|l| l.shared.last_pose.lock().unwrap().map(|p| (p.yaw, p.pitch)));
                    if let Some(history) = &mut history {
                        let (combined, history_s) = history.push(dt, iv);
                        let mut snapshot = Snapshot::from_interval(&combined, history_s, rtt, pose);
                        snapshot.report_interval_s = dt;
                        on_stats(&snapshot);
                    } else {
                        on_stats(&Snapshot::from_interval(&iv, dt, rtt, pose));
                    }
                }
            })
            .unwrap();
    }

    let mut dump = match &cfg.dump {
        Some(p) => Some(File::create(p).map_err(|e| format!("--dump {}: {e}", p.display()))?),
        None => None,
    };
    let mut preview = match cfg.preview {
        Some(pos) if cfg.video => Some(Preview::spawn(pos, cfg.enc.codec, cfg.enc.fps, &log)?),
        _ => None,
    };

    let (frame_w, frame_h) = cfg.frame_size();
    let convert_cfg = ConvertConfig {
        width: frame_w,
        height: frame_h,
        letterbox: cfg.letterbox,
        target: encoder::target_for(family),
        vaapi_device: cfg.enc.vaapi_device.clone(),
        gpu_scale_cpu_input: cfg.gpu_scale_cpu_input,
        extra_vf: cfg.vf.clone(),
        scale_mode: cfg.scale_mode.clone(),
        packed_alpha: cfg.pane.filter(|p| p.pane.format == proto::PaneFormat::Bgra).map(|p| p.alpha),
    };

    // ---- stage A (own thread): capture when due + convert on the GPU ----
    // Hands frames to stage B through a single slot: a newer frame replaces one that B
    // hasn't picked up yet (counted as skipped). Two stages let capture/convert of frame
    // N+1 overlap encode/send of frame N without ever holding more than one frame.
    let slot = Arc::new(Slot::default());
    let need_idr = Arc::new(AtomicBool::new(true));
    let conv_fps = cfg.enc.fps;
    let stage_a = src.take().map(|mut src| {
        let slot = slot.clone();
        let ctl = ctl.clone();
        let need_idr = need_idr.clone();
        let pc_stats = pc_stats.clone();
        let log = log.clone();
        let mut capture_cfg = cfg.clone();
        std::thread::Builder::new()
            .name("ep-capture".into())
            .spawn(move || {
                let mut convert_cfg = convert_cfg;
                let mut conv = Converter::new(convert_cfg.clone(), conv_fps);
                let mut described = false;
                let r: Result<(), String> = (|| {
                    while !ctl.stop.load(Ordering::Relaxed) {
                        let update = ctl.capture_updates.lock().unwrap().pop_front();
                        if let Some(update) = update {
                            let CaptureUpdate { source, alpha, pane, complete } = update;
                            let result = (|| {
                                if convert_cfg.target == eyepiece_av::convert::Target::Vaapi {
                                    return Err("live capture updates are not supported with VAAPI encoding".into());
                                }
                                if pane.is_some_and(|p| p.pane.format == proto::PaneFormat::Bgra) && capture_cfg.vf.is_some() {
                                    return Err("live transparency requires removing the custom filter and restarting".into());
                                }
                                let mut candidate_cfg = capture_cfg.clone();
                                let source_changed = source.is_some();
                                if let Some(source) = source {
                                    candidate_cfg.source = source;
                                }
                                if let Some(pane) = pane {
                                    candidate_cfg.pane = Some(pane);
                                }
                                let size_changed = candidate_cfg.frame_size() != capture_cfg.frame_size();
                                let reopen_source = source_changed || (size_changed && matches!(candidate_cfg.source, SourceKind::Test { .. }));
                                #[cfg(target_os = "macos")]
                                let reopen_source = reopen_source
                                    || (pane.is_some() && matches!(candidate_cfg.source, SourceKind::ScreenCaptureKit { .. })
                                        && (size_changed || candidate_cfg.pane.map(|p| p.pane.format) != capture_cfg.pane.map(|p| p.pane.format)));
                                let mut new_source = if reopen_source { Some(make_source(&candidate_cfg, family)?) } else { None };
                                let mut new_convert_cfg = convert_cfg.clone();
                                if let Some(pane) = pane {
                                    let (width, height) = candidate_cfg.frame_size();
                                    new_convert_cfg.width = width;
                                    new_convert_cfg.height = height;
                                    new_convert_cfg.packed_alpha = (pane.pane.format == proto::PaneFormat::Bgra).then_some(pane.alpha);
                                } else if let Some(alpha) = alpha {
                                    new_convert_cfg.packed_alpha = Some(alpha);
                                }
                                let mut new_conv = Converter::new(new_convert_cfg.clone(), conv_fps);
                                let source = new_source.as_mut().unwrap_or(&mut src);
                                let sf = source.next(true)?.ok_or("updated source produced no frame")?;
                                let t_conv = mono_us();
                                let frame = new_conv.convert(sf.frame)?.ok_or("updated converter produced no frame")?;
                                let reconfigure = if let Some(pane) = pane {
                                    let (tx, rx) = std::sync::mpsc::channel();
                                    slot.put(Item {
                                        frame, t_cap: sf.t_us, t_conv, t_conv_done: mono_us(),
                                        reconfigure: Some((pane, tx)),
                                    });
                                    loop {
                                        match rx.recv_timeout(Duration::from_millis(100)) {
                                            Ok(result) => { result?; break; }
                                            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return Err("pane reconfiguration interrupted".into()),
                                            Err(std::sync::mpsc::RecvTimeoutError::Timeout) if ctl.stop.load(Ordering::Relaxed) =>
                                                return Err("session stopped during pane reconfiguration".into()),
                                            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                                        }
                                    }
                                    true
                                } else {
                                    need_idr.store(true, Ordering::Relaxed);
                                    slot.put(Item { frame, t_cap: sf.t_us, t_conv, t_conv_done: mono_us(), reconfigure: None });
                                    false
                                };
                                if let Some(new_source) = new_source {
                                    src = new_source;
                                }
                                capture_cfg = candidate_cfg;
                                conv = new_conv;
                                convert_cfg = new_convert_cfg;
                                described = false;
                                if reconfigure { need_idr.store(true, Ordering::Relaxed); }
                                Ok(())
                            })();
                            complete(result);
                            continue;
                        }
                        // force a frame of a static screen when an IDR is pending
                        let Some(sf) = src.next(need_idr.load(Ordering::Relaxed))? else { continue };
                        pc_stats.lock().unwrap().captured += 1;
                        let t_conv = mono_us();
                        let Some(frame) = conv.convert(sf.frame)? else { continue };
                        if !described {
                            log(format!("convert: {}", conv.description));
                            described = true;
                        }
                        let item = Item { frame, t_cap: sf.t_us, t_conv, t_conv_done: mono_us(), reconfigure: None };
                        if slot.put(item) {
                            pc_stats.lock().unwrap().skipped += 1;
                        }
                    }
                    Ok(())
                })();
                slot.close(r);
            })
            .unwrap()
    });

    // ---- stage B (this thread): encode + send, connection management ----
    let mut enc: Option<Encoder> = None;
    let codec_wire = cfg.enc.codec.wire();
    let fps = cfg.enc.fps as f64;
    let mut t0: Option<u64> = None;
    let mut last_pts = -1i64;
    let mut pending: VecDeque<Pending> = VecDeque::new();
    let mut pkts: Vec<Packet> = vec![];
    let mut frames_sent = 0u64;
    let t_start = mono_us();
    let mut last_reconnect = 0u64;
    let mut head_tracking = cfg.link.as_ref().is_some_and(|l| l.head_tracking);
    let mut preview_reset = false;

    let result: Result<(), String> = (|| {
        loop {
            if ctl.stop.load(Ordering::Relaxed) {
                return Ok(());
            }
            if let Some(d) = cfg.duration_s {
                if (mono_us() - t_start) as f64 / 1e6 >= d {
                    return Ok(());
                }
            }
            if cfg.max_frames.is_some_and(|m| frames_sent >= m) {
                return Ok(());
            }
            // ---- connection state ----
            let cur = link.lock().unwrap().clone();
            if let Some(l) = &cur {
                if !l.alive() {
                    *link.lock().unwrap() = None;
                    ev.link_changed(None);
                    if !cfg.reconnect {
                        if cfg.pane.is_some() {
                            return Err("pane connection lost; check data-link and bridge logs".into());
                        }
                        return Ok(());
                    }
                    log("bridge connection lost, reconnecting…".into());
                    continue;
                }
                if ctl.recenter.swap(false, Ordering::Relaxed) {
                    let _ = l.recenter();
                    log("recenter sent".into());
                }
                let ht = ctl.head_tracking.swap(0, Ordering::Relaxed);
                if ht != 0 && (ht == 2) != head_tracking {
                    head_tracking = ht == 2;
                    let _ = l.set_head_tracking(head_tracking);
                    log(format!("head tracking {}", if head_tracking { "on" } else { "off" }));
                }
                let update = ctl.gui_updates.lock().unwrap().pop_front();
                if let Some(update) = update {
                    let name = if update.pane { "pane opacity" } else { "GUI opacity" };
                    let result = if update.opacity > 100 {
                        Err(format!("{name} must be between 0 and 100"))
                    } else if update.pane {
                        l.set_pane_alpha(gui_alpha(update.opacity), Duration::from_secs(3))
                            .map_err(|e| format!("{name}: {e}"))
                    } else {
                        l.set_gui_alpha(gui_alpha(update.opacity), Duration::from_secs(3))
                            .map_err(|e| format!("{name}: {e}"))
                    };
                    if result.is_ok() {
                        if update.pane { cfg.pane_opacity = update.opacity } else { cfg.gui_opacity = update.opacity }
                    }
                    (update.complete)(result);
                }
            } else if cfg.link.is_some() {
                // reconnect loop (only reached with --reconnect); stage A keeps running
                // and its frames are dropped meanwhile
                let _ = slot.take(Duration::from_millis(0));
                if mono_us() - last_reconnect < 1_000_000 {
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                }
                last_reconnect = mono_us();
                let mut c2 = cfg.clone();
                if let Some(lo) = c2.link.as_mut() {
                    lo.connect_timeout = Duration::from_millis(500);
                    lo.head_tracking = head_tracking;
                }
                match connect(&mut c2, &cfg.enc, &ev) {
                    Ok(Some(l)) => {
                        *link.lock().unwrap() = Some(l);
                        need_idr.store(true, Ordering::Relaxed); // the bridge waits for an IDR
                    }
                    Ok(None) => {}
                    Err(error) => log(format!("reconnect failed: {error}")),
                }
                continue;
            }
            if cur.is_none() && cfg.link.is_none() {
                let update = ctl.gui_updates.lock().unwrap().pop_front();
                if let Some(update) = update {
                    (update.complete)(Err("opacity control requires a bridge connection".into()));
                }
            }
            if stage_a.is_none() {
                // head tracking only
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            let key_req = cur.as_ref().is_some_and(|l| l.take_key_request()) || ctl.idr.swap(false, Ordering::Relaxed);
            if key_req && !need_idr.swap(true, Ordering::Relaxed) {
                log("IDR requested (bridge resync)".into());
            }

            // ---- next converted frame ----
            let mut it = match slot.take(Duration::from_millis(100)) {
                Taken::Item(it) => it,
                Taken::Empty => continue,
                Taken::Closed(r) => return r,
            };
            let t_enc_in = mono_us();
            if let Some((pane, done)) = it.reconfigure.take() {
                let result = (|| -> Result<Encoder, String> {
                    let mut next = cfg.clone();
                    next.pane = Some(pane);
                    let stream = stream_config(&next, codec_wire);
                    if let Some(l) = &cur {
                        let info = l.info().ok_or("bridge capabilities not received")?;
                        stream.payload(info).map_err(str::to_string)?;
                    } else if cfg.link.is_some() {
                        return Err("bridge is disconnected".into());
                    }
                    // SAFETY: the converted frame and its optional hardware context are live.
                    let encoder = unsafe { Encoder::open(&cfg.enc, &enc_name, &it.frame, (*it.frame.0).hw_frames_ctx) }?;
                    if let Some(l) = &cur {
                        l.configure(stream, Duration::from_secs(5))
                            .map_err(|e| format!("bridge reconfiguration: {e}"))?;
                    }
                    Ok(encoder)
                })();
                match result {
                    Ok(encoder) => {
                        log(format!("encoder: {}", encoder.description));
                        enc = Some(encoder);
                        cfg.pane = Some(pane);
                        pending.clear();
                        pkts.clear();
                        need_idr.store(true, Ordering::Relaxed);
                        preview_reset = true;
                        let _ = done.send(Ok(()));
                    }
                    Err(error) => {
                        let _ = done.send(Err(error));
                        continue;
                    }
                }
            }
            let frame = it.frame;
            if enc.is_none() {
                // SAFETY: frame.0 is a live AVFrame; its hw_frames_ctx is null or valid.
                let e = unsafe { Encoder::open(&cfg.enc, &enc_name, &frame, (*frame.0).hw_frames_ctx) }?;
                log(format!("encoder: {}", e.description));
                enc = Some(e);
            }
            let base = *t0.get_or_insert(it.t_cap);
            let mut pts = ((it.t_cap.saturating_sub(base)) as f64 * fps / 1e6).round() as i64;
            if pts <= last_pts {
                pts = last_pts + 1;
            }
            last_pts = pts;
            pending.push_back(Pending { pts, t_cap: it.t_cap, t_conv: it.t_conv, t_conv_done: it.t_conv_done, t_enc_in });
            while pending.len() > 32 {
                pending.pop_front();
            }
            let idr_now = need_idr.load(Ordering::Relaxed);
            enc.as_mut().unwrap().encode(frame, pts, idr_now, &mut pkts)?;
            let t_enc_out = mono_us();

            for p in pkts.drain(..) {
                let Some(i) = pending.iter().position(|x| x.pts == p.pts) else { continue };
                let meta = pending.remove(i).unwrap();
                if idr_now && p.key {
                    need_idr.store(false, Ordering::Relaxed);
                    pc_stats.lock().unwrap().idr += 1;
                }
                if let Some(l) = &cur {
                    if let Err(e) = l.send_video(meta.t_cap, codec_wire, p.key, &p.data) {
                        if !cfg.reconnect {
                            return Err(format!("send: {e}"));
                        }
                        continue;
                    }
                }
                let t_sent = mono_us();
                {
                    let mut s = pc_stats.lock().unwrap();
                    s.sent += 1;
                    s.bytes += p.data.len() as u64;
                    s.cap_send.push((t_sent - meta.t_cap) as u32);
                    // capture→encoder input: capture wait + GPU convert (+ slot wait)
                    s.cap_enc.push((meta.t_enc_in - meta.t_cap) as u32);
                    s.enc.push((t_enc_out - meta.t_enc_in) as u32);
                    let _ = (meta.t_conv, meta.t_conv_done);
                }
                frames_sent += 1;
                if let Some(d) = dump.as_mut() {
                    d.write_all(&p.data).map_err(|e| format!("dump: {e}"))?;
                }
                if let Some(pv) = preview.as_mut() {
                    pv.feed(&p.data);
                }
                if let Some(tap) = &ev.au {
                    let packed = cfg.pane.is_some_and(|pane| pane.pane.format == proto::PaneFormat::Bgra);
                    tap(&p.data, p.key, packed, preview_reset);
                    if p.key { preview_reset = false; }
                }
            }
        }
    })();

    ctl.stop.store(true, Ordering::Relaxed);
    if let Some(a) = stage_a {
        let _ = a.join();
    }
    if let Some(l) = link.lock().unwrap().take() {
        if cfg.stop_bridge {
            let _ = l.quit_bridge();
        }
        l.close();
        ev.link_changed(None);
    }
    drop(preview);
    drop(enc);
    result
}

struct Item {
    frame: eyepiece_av::ffi::Frame,
    t_cap: u64,
    t_conv: u64,
    t_conv_done: u64,
    reconfigure: Option<(PaneConfig, std::sync::mpsc::Sender<Result<(), String>>)>,
}

enum Taken {
    Item(Item),
    Empty,
    Closed(Result<(), String>),
}

/// One-frame mailbox between the stages: `put` replaces an unclaimed frame.
type SlotState = (Option<Item>, Option<Result<(), String>>);

#[derive(Default)]
struct Slot {
    m: Mutex<SlotState>,
    cv: std::sync::Condvar,
}
impl Slot {
    /// true if an older frame was replaced (dropped)
    fn put(&self, it: Item) -> bool {
        let mut g = self.m.lock().unwrap();
        let dropped = g.0.replace(it).is_some();
        self.cv.notify_one();
        dropped
    }
    fn close(&self, r: Result<(), String>) {
        self.m.lock().unwrap().1 = Some(r);
        self.cv.notify_one();
    }
    fn take(&self, timeout: Duration) -> Taken {
        let mut g = self.m.lock().unwrap();
        if g.0.is_none() && g.1.is_none() && !timeout.is_zero() {
            g = self.cv.wait_timeout_while(g, timeout, |g| g.0.is_none() && g.1.is_none()).unwrap().0;
        }
        if let Some(it) = g.0.take() {
            return Taken::Item(it);
        }
        match g.1.take() {
            Some(r) => Taken::Closed(r),
            None => Taken::Empty,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gui_opacity_scales_to_full_alpha_range() {
        assert_eq!(gui_alpha(0), 0);
        assert_eq!(gui_alpha(50), 128);
        assert_eq!(gui_alpha(100), 255);
    }

    #[test]
    #[ignore = "requires stationary goggles and a separately started hardware bridge"]
    fn device_pane_reconfiguration() {
        use std::sync::mpsc;
        use std::time::Instant;

        let addr = std::env::var("EYEPIECE_TEST_BRIDGE").expect("EYEPIECE_TEST_BRIDGE is required");
        let transport = match std::env::var("EYEPIECE_TEST_TRANSPORT").as_deref().unwrap_or("tcp") {
            "tcp" => eyepiece_link::LinkTransport::Tcp,
            "usb-bulk" => eyepiece_link::LinkTransport::UsbBulk { serial: None, launch: None },
            value => panic!("unsupported EYEPIECE_TEST_TRANSPORT: {value}"),
        };
        let codec = std::env::var("EYEPIECE_TEST_CODEC").unwrap_or_else(|_| "h264".into());
        let stress = std::env::var_os("EYEPIECE_TEST_EDGE_STRESS").is_some();
        let args = crate::cli::Args {
            source: "test".into(), no_connect: true,
            pane: Some(if stress { "1178x736+100+100" } else { "640x480+100+100" }.into()),
            encoder: if stress { "videotoolbox" } else { "software" }.into(),
            codec, fps: if stress { 88 } else { 10 },
            lavfi: stress.then(|| "testsrc2=size=1920x1080:rate=88".into()),
            color_key: stress.then(|| "00ff00".into()),
            ..crate::cli::Args::default()
        };
        let log: LogFn = Arc::new(|message| eprintln!("{message}"));
        let mut cfg = crate::cli::prepare(&args, &log).unwrap().unwrap().cfg;
        cfg.stats_interval = 0.25;
        cfg.reconnect = true;
        cfg.stop_bridge = true;
        cfg.link = Some(LinkOptions {
            addr, transport,
            connect_timeout: Duration::from_secs(3), opentrack: None, print_pose: false, head_tracking: false,
        });
        let ctl = Arc::new(Control::default());
        let worker_ctl = ctl.clone();
        let (stats_tx, stats_rx) = mpsc::channel();
        let (link_tx, link_rx) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let mut events = Events::new(log, Arc::new(move |stats| {
                let _ = stats_tx.send(stats.shown_fps);
            }));
            events.on_link = Some(Arc::new(move |link| {
                if let Some(link) = link { let _ = link_tx.send(link); }
            }));
            run(cfg, worker_ctl, events)
        });
        let link = link_rx.recv_timeout(Duration::from_secs(10)).unwrap();
        assert!(link.info().is_some_and(|info| info.flags & proto::EP_INFO_RECONFIG != 0));
        assert!(link.info().is_some_and(|info| info.flags & proto::EP_INFO_PANE_ALPHA != 0));
        let wait_for_frames = |link: &Arc<Link>| {
            let deadline = Instant::now() + Duration::from_secs(12);
            let mut intervals = 0;
            while Instant::now() < deadline {
                if stats_rx.recv_timeout(Duration::from_secs(1)).is_ok_and(|fps| fps > 0.0) {
                    intervals += 1;
                    if intervals >= 3 { return; }
                }
                assert!(link.alive(), "bridge disconnected while waiting for frames");
            }
            panic!("bridge stopped reporting frames");
        };
        wait_for_frames(&link);
        let (done_tx, done_rx) = mpsc::channel();
        let changes = if stress {
            vec![
                (0, 0, 1178, 736, proto::PaneFormat::Bgra),
                (742, 0, 1178, 736, proto::PaneFormat::Bgra),
                (742, 344, 1178, 736, proto::PaneFormat::Bgra),
                (0, 344, 1178, 736, proto::PaneFormat::Bgra),
                (802, 704, 1118, 376, proto::PaneFormat::Bgra),
                (1218, 722, 702, 358, proto::PaneFormat::Bgra),
                (742, 344, 1178, 736, proto::PaneFormat::Bgra),
                (100, 100, 1178, 736, proto::PaneFormat::Bgra),
            ].repeat(3)
        } else {
            vec![
                (200, 150, 640, 480, proto::PaneFormat::OpaqueBgra),
                (300, 200, 320, 240, proto::PaneFormat::OpaqueBgra),
                (300, 200, 320, 240, proto::PaneFormat::Bgra),
                (100, 100, 640, 480, proto::PaneFormat::Bgra),
                (100, 100, 640, 480, proto::PaneFormat::OpaqueBgra),
                (0, 0, 1920, 1080, proto::PaneFormat::OpaqueBgra),
                (480, 270, 960, 540, proto::PaneFormat::OpaqueBgra),
            ].repeat(3)
        };
        for (index, (x, y, width, height, format)) in changes.into_iter().enumerate() {
            let pane = PaneConfig {
                pane: proto::Pane { x, y, width, height, format },
                alpha: PackedAlpha::ColorKey { rgb: 0x00ff00, similarity: 0.1, blend: 0.0, source_alpha: false },
            };
            let done = done_tx.clone();
            let start = Instant::now();
            ctl.queue_capture_update(CaptureUpdate {
                source: None, alpha: None, pane: Some(pane),
                complete: Box::new(move |result| { let _ = done.send(result); }),
            });
            done_rx.recv_timeout(Duration::from_secs(15)).unwrap().unwrap();
            eprintln!("reconfigured {width}x{height}+{x}+{y} {format:?} in {:?}", start.elapsed());
            while stats_rx.try_recv().is_ok() {}
            wait_for_frames(&link);
            for pane in [false, true] {
                let opacity = if pane { [0, 50, 100][index % 3] } else { [50, 0, 100][index % 3] };
                let (done, applied) = mpsc::channel();
                ctl.gui_updates.lock().unwrap().push_back(GuiUpdate {
                    opacity, pane,
                    complete: Box::new(move |result| { let _ = done.send(result); }),
                });
                applied.recv_timeout(Duration::from_secs(5)).unwrap().unwrap();
            }
            while stats_rx.try_recv().is_ok() {}
            wait_for_frames(&link);
        }
        link.close();
        let reconnected = link_rx.recv_timeout(Duration::from_secs(15)).unwrap();
        while stats_rx.try_recv().is_ok() {}
        wait_for_frames(&reconnected);
        ctl.stop.store(true, Ordering::Relaxed);
        assert!(worker.join().unwrap().is_ok());
    }

    #[test]
    fn pane_updates_reconfigure_bridge_and_preserve_stream_after_rejection() {
        use prost::Message;
        use std::io::{Read, Write};
        use std::net::TcpListener;
        use std::sync::mpsc;

        enum BridgeEvent {
            Request(proto::wire::Configure),
            Opacity(u32),
            Video(bool, Vec<u8>),
        }
        fn decoded_size(data: &[u8]) -> (i32, i32) {
            use eyepiece_av::ffi::*;
            unsafe {
                let codec = avcodec_find_decoder(AVCodecID::AV_CODEC_ID_H264);
                assert!(!codec.is_null());
                let mut decoder = avcodec_alloc_context3(codec);
                assert!(!decoder.is_null());
                check(avcodec_open2(decoder, codec, std::ptr::null_mut()), "open decoder").unwrap();
                let mut packet = av_packet_alloc();
                assert!(!packet.is_null());
                check(av_new_packet(packet, data.len() as i32), "allocate packet").unwrap();
                std::ptr::copy_nonoverlapping(data.as_ptr(), (*packet).data, data.len());
                check(avcodec_send_packet(decoder, packet), "send packet").unwrap();
                let frame = Frame::alloc();
                check(avcodec_receive_frame(decoder, frame.0), "receive frame").unwrap();
                let size = (frame.width(), frame.height());
                av_packet_free(&mut packet);
                avcodec_free_context(&mut decoder);
                size
            }
        }
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (bridge_tx, bridge_rx) = mpsc::channel();
        let bridge = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let info = proto::wire::Info {
                version: proto::EP_VERSION, framebuffer_width: 1920, framebuffer_height: 1080,
                flags: proto::EP_INFO_PANE_BGRA | proto::EP_INFO_PANE_OPAQUE_BGRA | proto::EP_INFO_RECONFIG | proto::EP_INFO_PANE_ALPHA,
                ..Default::default()
            };
            socket.write_all(&proto::bridge_frame(proto::wire::envelope::Message::Info(info)).unwrap()).unwrap();
            let mut pending_video = true;
            let mut requests = 0;
            loop {
                let mut header = [0u8; 4];
                if socket.read_exact(&mut header).is_err() { break; }
                let len = u32::from_be_bytes(header) as usize;
                let mut payload = vec![0; len];
                if socket.read_exact(&mut payload).is_err() { break; }
                let packet = proto::wire::Envelope::decode(&payload[..]).unwrap();
                match packet.message {
                    Some(proto::wire::envelope::Message::Video(video)) if pending_video => {
                        bridge_tx.send(BridgeEvent::Video(video.key, video.access_unit)).unwrap();
                        pending_video = false;
                    }
                    Some(proto::wire::envelope::Message::Configure(request)) => {
                        requests += 1;
                        let ack = proto::wire::Acknowledgement { status: if requests == 3 { 1 } else { 0 } };
                        bridge_tx.send(BridgeEvent::Request(request)).unwrap();
                        let response = proto::wire::Envelope {
                            id: packet.id, sender: None, recipient: None,
                            message: Some(proto::wire::envelope::Message::ConfigureAck(ack)),
                        };
                        socket.write_all(&proto::encode_frame(&response, proto::EP_MAX_PAYLOAD as usize).unwrap()).unwrap();
                        pending_video = true;
                    }
                    Some(proto::wire::envelope::Message::PaneAlpha(request)) => {
                        bridge_tx.send(BridgeEvent::Opacity(request.alpha)).unwrap();
                        let response = proto::wire::Envelope {
                            id: packet.id, sender: None, recipient: None,
                            message: Some(proto::wire::envelope::Message::PaneAlphaAck(
                                proto::wire::Acknowledgement { status: if request.alpha == 0 { 2 } else { 0 } },
                            )),
                        };
                        socket.write_all(&proto::encode_frame(&response, proto::EP_MAX_PAYLOAD as usize).unwrap()).unwrap();
                    }
                    _ => {}
                }
            }
        });
        let args = crate::cli::Args {
            source: "test".into(), no_connect: true, pane: Some("64x64+0+0".into()),
            encoder: "software".into(), codec: "h264".into(), fps: 30,
            ..crate::cli::Args::default()
        };
        let log: LogFn = Arc::new(|_| {});
        let mut cfg = crate::cli::prepare(&args, &log).unwrap().unwrap().cfg;
        cfg.link = Some(LinkOptions {
            addr: addr.to_string(), transport: eyepiece_link::LinkTransport::Tcp,
            connect_timeout: Duration::from_secs(2), opentrack: None, print_pose: false, head_tracking: false,
        });
        let ctl = Arc::new(Control::default());
        let worker_ctl = ctl.clone();
        let (preview_tx, preview_rx) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let mut events = Events::new(log, Arc::new(|_| {}));
            events.au = Some(Arc::new(move |_, key, packed, reset| {
                if reset && key { let _ = preview_tx.send(packed); }
            }));
            run(cfg, worker_ctl, events)
        });
        let BridgeEvent::Request(initial) = bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap() else { panic!("missing configure") };
        assert_eq!((initial.codec, initial.layout), (0, 0));
        assert_eq!(initial.pane.map(|p| (p.width, p.height)), Some((64, 64)));
        // a fully opaque pane is the connection's start state: no PaneAlpha is sent for it
        assert!(matches!(bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap(), BridgeEvent::Video(true, _)));
        for (opacity, alpha, accepted) in [(50, 128, true), (0, 0, false), (100, 255, true)] {
            let (done_tx, done_rx) = mpsc::channel();
            ctl.gui_updates.lock().unwrap().push_back(GuiUpdate {
                opacity, pane: true,
                complete: Box::new(move |result| { done_tx.send(result).unwrap(); }),
            });
            let event = bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            assert!(matches!(event, BridgeEvent::Opacity(value) if value == alpha));
            assert_eq!(done_rx.recv_timeout(Duration::from_secs(10)).unwrap().is_ok(), accepted);
        }
        let (done_tx, done_rx) = mpsc::channel();
        let queue = |pane: PaneConfig| {
            let done = done_tx.clone();
            ctl.capture_updates.lock().unwrap().push_back(CaptureUpdate {
                source: None, alpha: None, pane: Some(pane),
                complete: Box::new(move |result| { done.send(result).unwrap(); }),
            });
        };
        let pane = |width, format| PaneConfig {
            pane: proto::Pane { x: 10, y: 20, width, height: 32, format },
            alpha: PackedAlpha::ColorKey { rgb: 0x00ff00, similarity: 0.1, blend: 0.0, source_alpha: false },
        };
        queue(pane(32, proto::PaneFormat::Bgra));
        let BridgeEvent::Request(first) = bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap() else { panic!("missing request") };
        let first_pane = first.pane.unwrap();
        assert_eq!((first_pane.x, first_pane.y, first_pane.width, first_pane.height), (10, 20, 32, 32));
        done_rx.recv_timeout(Duration::from_secs(10)).unwrap().unwrap();
        let BridgeEvent::Video(true, data) = bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap() else { panic!("missing keyframe") };
        assert_eq!(decoded_size(&data), (64, 32));
        assert!(preview_rx.recv_timeout(Duration::from_secs(10)).unwrap());

        queue(pane(48, proto::PaneFormat::OpaqueBgra));
        assert!(matches!(bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap(), BridgeEvent::Request(_)));
        assert!(done_rx.recv_timeout(Duration::from_secs(10)).unwrap().is_err());
        assert!(matches!(bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap(), BridgeEvent::Video(_, _)));

        queue(pane(48, proto::PaneFormat::OpaqueBgra));
        let BridgeEvent::Request(last) = bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap() else { panic!("missing request") };
        let last_pane = last.pane.unwrap();
        assert_eq!((last_pane.width, last_pane.height, last_pane.format), (48, 32, 2));
        done_rx.recv_timeout(Duration::from_secs(10)).unwrap().unwrap();
        let BridgeEvent::Video(true, data) = bridge_rx.recv_timeout(Duration::from_secs(10)).unwrap() else { panic!("missing keyframe") };
        assert_eq!(decoded_size(&data), (48, 32));
        assert!(!preview_rx.recv_timeout(Duration::from_secs(10)).unwrap());
        ctl.stop.store(true, Ordering::Relaxed);
        assert!(worker.join().unwrap().is_ok());
        bridge.join().unwrap();
    }

    #[test]
    fn capture_updates_preserve_pipeline_after_invalid_source() {
        let args = crate::cli::Args {
            source: "test".into(),
            no_connect: true,
            pane: Some("64x64+0+0".into()),
            color_key: Some("00ff00".into()),
            encoder: "software".into(),
            codec: "h264".into(),
            fps: 30,
            ..crate::cli::Args::default()
        };
        let log: LogFn = Arc::new(|_| {});
        let cfg = crate::cli::prepare(&args, &log).unwrap().unwrap().cfg;
        let ctl = Arc::new(Control::default());
        let worker_ctl = ctl.clone();
        let frames = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let worker_frames = frames.clone();
        let worker = std::thread::spawn(move || {
            let mut events = Events::new(log, Arc::new(|_| {}));
            events.au = Some(Arc::new(move |_, _, _, _| { worker_frames.fetch_add(1, Ordering::Relaxed); }));
            run(cfg, worker_ctl, events)
        });
        let wait_for_frame = |previous| {
            let deadline = std::time::Instant::now() + Duration::from_secs(10);
            while frames.load(Ordering::Relaxed) <= previous && std::time::Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(10));
            }
            assert!(frames.load(Ordering::Relaxed) > previous);
        };
        wait_for_frame(0);
        let (tx, rx) = std::sync::mpsc::channel();
        let done = tx.clone();
        ctl.capture_updates.lock().unwrap().push_back(CaptureUpdate {
            source: Some(SourceKind::File { path: "/nonexistent/eyepiece-test-video".into() }),
            alpha: None,
            pane: None,
            complete: Box::new(move |r| { done.send(r).unwrap(); }),
        });
        assert!(rx.recv_timeout(Duration::from_secs(10)).unwrap().is_err());
        wait_for_frame(frames.load(Ordering::Relaxed));
        ctl.capture_updates.lock().unwrap().push_back(CaptureUpdate {
            source: Some(SourceKind::Test { lavfi: Some("color=red:size=64x64:rate=30".into()) }),
            alpha: Some(PackedAlpha::ColorKey {
                rgb: 0xff0000, similarity: 0.1, blend: 0.0, source_alpha: false,
            }),
            pane: None,
            complete: Box::new(move |r| { tx.send(r).unwrap(); }),
        });
        let result = rx.recv_timeout(Duration::from_secs(10));
        if result.as_ref().is_ok_and(Result::is_ok) {
            wait_for_frame(frames.load(Ordering::Relaxed));
        }
        ctl.stop.store(true, Ordering::Relaxed);
        let worker_result = worker.join().unwrap();
        assert!(worker_result.is_ok(), "{worker_result:?}");
        result.unwrap().unwrap();
    }

    #[test]
    fn rolling_statistics_combine_bursts_and_expire_missing_data() {
        let mut window = StatsWindow::new(1.0);
        let burst = Interval {
            shown: 20,
            dec: vec![1_000, 3_000],
            q_max: 4,
            dropped: 7,
            ..Default::default()
        };
        window.push(0.25, burst);
        for _ in 0..3 {
            let (combined, elapsed) = window.push(0.25, Interval { dropped: 7, ..Default::default() });
            let snapshot = Snapshot::from_interval(&combined, elapsed, None, None);
            assert_eq!(snapshot.shown_fps, 20.0 / elapsed);
            assert_eq!(snapshot.dec_p50_ms, 3.0);
            assert_eq!(snapshot.q_max, 4);
            assert_eq!(snapshot.dropped, 7);
        }
        let (combined, elapsed) = window.push(0.25, Interval { dropped: 7, ..Default::default() });
        let snapshot = Snapshot::from_interval(&combined, elapsed, None, None);
        assert_eq!(snapshot.shown_fps, 0.0);
        assert!(snapshot.dec_p50_ms.is_nan());
        assert_eq!(snapshot.dropped, 7);
        window.clear();
        let (combined, elapsed) = window.push(0.25, Interval::default());
        assert_eq!(elapsed, 0.25);
        assert_eq!(combined.dropped, 0);
    }
}
