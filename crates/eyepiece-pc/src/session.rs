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
use eyepiece_av::{ConvertConfig, Converter, Encoder, EncoderConfig, Packet, Source};
use eyepiece_capture::SyncMode;
use eyepiece_link::clock::mono_us;
use eyepiece_link::proto;
use eyepiece_link::stats::{Interval, Snapshot};
use eyepiece_link::{Link, LinkOptions, LogFn};

/// Encoded size, fixed: the goggles panel. They have no display scaler and the bridge refuses
/// any other stream size, so every source is scaled (and letterboxed) to this on the PC.
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
    Dxgi { adapter: u32, output: u32 },
    #[cfg(target_os = "macos")]
    ScreenCaptureKit { display_id: Option<u32>, cursor: bool },
    Test { lavfi: Option<String> },
    File { path: String },
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
    pub max_frames: Option<u64>,
    pub duration_s: Option<f64>,
    /// side-by-side stereo (2560x720, the goggles' 3D mode) instead of 1920x1080 mono
    pub stereo: bool,
}

impl Config {
    /// Encoded frame size: the panel (mono), or both eyes side by side (stereo).
    pub fn frame_size(&self) -> (u32, u32) {
        if self.stereo {
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
}

impl Control {
    /// Turn the goggles' head tracking on/off while running; kept across reconnects.
    pub fn set_head_tracking(&self, on: bool) {
        self.head_tracking.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    }
}

/// Encoded-stream tap: every AU right after it was written to the socket (`key` = IDR).
/// Runs on the encode/send thread: it must only copy and hand off (bounded, non-blocking).
pub type AuTap = Arc<dyn Fn(&[u8], bool) + Send + Sync>;
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
        SourceKind::Dxgi { adapter, output } => {
            let d = eyepiece_capture::dxgi::DxgiCapture::open(*adapter, *output, fps)?;
            Box::new(eyepiece_av::CaptureSource::new(Box::new(d), ""))
        }
        #[cfg(target_os = "macos")]
        SourceKind::ScreenCaptureKit { display_id, cursor } => {
            let capture = if _encoder_family == encoder::Family::VideoToolbox && cfg.vf.is_none() && cfg.gpu_scale_cpu_input {
                eyepiece_capture::screencapturekit::ScreenCapture::open_native(
                    *display_id, cfg.enc.fps, *cursor, cfg.frame_size(), cfg.letterbox,
                )?
            } else {
                eyepiece_capture::screencapturekit::ScreenCapture::open(*display_id, cfg.enc.fps, *cursor)?
            };
            Box::new(eyepiece_av::CaptureSource::new(Box::new(capture), ""))
        }
        SourceKind::Test { lavfi } => {
            let (w, h) = cfg.frame_size();
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

/// Connect and announce the stream (EP_STREAM must precede video).
fn connect(cfg: &mut Config, enc: &EncoderConfig, ev: &Events) -> Result<Option<Arc<Link>>, String> {
    let Some(lo) = cfg.link.clone() else { return Ok(None) };
    let l = Link::connect(&lo, ev.log.clone())?;
    if let Some(options) = cfg.link.as_mut() {
        options.transport = l.selected_transport();
    }
    (ev.log)(format!("data link: {}", l.transport_name()));
    if cfg.video {
        if cfg.stereo {
            // EP_INFO comes right after the connect: only stereo-capable bridges get SBS
            let t0 = std::time::Instant::now();
            while l.info().is_none() && t0.elapsed() < Duration::from_secs(2) {
                std::thread::sleep(Duration::from_millis(20));
            }
            if l.info().is_none_or(|i| i.flags & proto::EP_INFO_STEREO == 0) {
                return Err("this eyepiece_bridge has no stereo support (needs natartos \
                            v0.1.0-eyepiece.5 or newer, or --bridge with a current build)"
                    .into());
            }
        }
        let (w, h) = cfg.frame_size();
        let layout = if cfg.stereo { proto::EP_LAYOUT_SBS } else { proto::EP_LAYOUT_MONO };
        l.send_stream(w as u16, h as u16, enc.codec.wire(), enc.fps, layout).map_err(|e| e.to_string())?;
    }
    ev.link_changed(Some(l.clone()));
    Ok(Some(l))
}

pub fn run(mut cfg: Config, ctl: Arc<Control>, ev: Events) -> Result<(), String> {
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
        let on_stats = ev.stats.clone();
        std::thread::Builder::new()
            .name("ep-stats".into())
            .spawn(move || {
                let mut t_last = mono_us();
                while !ctl.stop.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_millis(100));
                    let now = mono_us();
                    if (now - t_last) as f64 / 1e6 < interval {
                        continue;
                    }
                    let dt = (now - t_last) as f64 / 1e6;
                    t_last = now;
                    let mut iv = link
                        .lock()
                        .unwrap()
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
                    let l = link.lock().unwrap().clone();
                    let rtt = l.as_ref().and_then(|l| l.shared.rtt_us());
                    let pose = l.as_ref().and_then(|l| l.shared.last_pose.lock().unwrap().map(|p| (p.yaw, p.pitch)));
                    on_stats(&Snapshot::from_interval(&iv, dt, rtt, pose));
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
        std::thread::Builder::new()
            .name("ep-capture".into())
            .spawn(move || {
                let mut conv = Converter::new(convert_cfg, conv_fps);
                let mut described = false;
                let r: Result<(), String> = (|| {
                    while !ctl.stop.load(Ordering::Relaxed) {
                        // force a frame of a static screen when an IDR is pending
                        let Some(sf) = src.next(need_idr.load(Ordering::Relaxed))? else { continue };
                        pc_stats.lock().unwrap().captured += 1;
                        let t_conv = mono_us();
                        let Some(frame) = conv.convert(sf.frame)? else { continue };
                        if !described {
                            log(format!("convert: {}", conv.description));
                            described = true;
                        }
                        let item = Item { frame, t_cap: sf.t_us, t_conv, t_conv_done: mono_us() };
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
                if let Ok(Some(l)) = connect(&mut c2, &cfg.enc, &ev) {
                    *link.lock().unwrap() = Some(l);
                    need_idr.store(true, Ordering::Relaxed); // the bridge waits for an IDR
                }
                continue;
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
            let it = match slot.take(Duration::from_millis(100)) {
                Taken::Item(it) => it,
                Taken::Empty => continue,
                Taken::Closed(r) => return r,
            };
            let t_enc_in = mono_us();
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
                    tap(&p.data, p.key);
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
}

enum Taken {
    Item(Item),
    Empty,
    Closed(Result<(), String>),
}

/// One-frame mailbox between the stages: `put` replaces an unclaimed frame.
#[derive(Default)]
struct Slot {
    m: Mutex<(Option<Item>, Option<Result<(), String>>)>,
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
