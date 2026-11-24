//! eyepiece-pc command line: the options, and how they become a [`Config`] (adb deploy/forward
//! included). The `eyepiece-pc` binary is [`main`]; other frontends (the GUI) build an [`Args`]
//! and use [`prepare`] + [`run_prepared`], so every frontend takes the same code path.
//! Replaces legacy/eyepiece_stream.py (same protocol, same bridge). See README.md.

use crate::session::{self, Config, Control, Events, SourceKind};
use clap::Parser;
use eyepiece_av::{Codec, EncoderConfig};
use eyepiece_capture::SyncMode;
use eyepiece_link::adb::Adb;
use eyepiece_link::{LinkOptions, LogFn};
use std::io::BufRead;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

const BRIDGE_NAME: &str = "eyepiece_bridge";

/// `--deploy` without `--bridge`: `eyepiece_bridge` next to this executable, else
/// `$XDG_DATA_HOME/eyepiece/eyepiece_bridge` (default `~/.local/share/...`); `None` = no local
/// bridge → the one already on the goggles (natartos' Eyepiece mod) is started.
fn find_bridge() -> Option<PathBuf> {
    let mut tried = vec![];
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
    {
        tried.push(dir.join(BRIDGE_NAME));
    }
    let data = std::env::var_os("XDG_DATA_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")));
    if let Some(d) = data {
        tried.push(d.join("eyepiece").join(BRIDGE_NAME));
    }
    tried.into_iter().find(|p| p.is_file())
}

#[derive(Parser, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
#[command(
    name = "eyepiece-pc",
    version,
    about = "Stream the PC screen to DJI Goggles 3 over USB (Eyepiece PC side)",
    after_help = "Keys (stdin): r + Enter = recenter, i + Enter = force IDR, q + Enter = quit.\n\
Examples:\n  eyepiece-pc                                 # start the bridge on the goggles, KMS capture of the first screen\n  \
eyepiece-pc --output eDP-1 --encoder nvenc --bitrate 30M\n  \
eyepiece-pc --no-deploy                     # use the bridge started from the goggles' menu\n  \
eyepiece-pc --source test --host 127.0.0.1 --no-forward   # against eyepiece_bridge_host -N\n  \
eyepiece-pc --list                          # outputs / planes"
)]
pub struct Args {
    // ---- transport ----
    /// adb executable
    #[arg(long, default_value = "adb")]
    pub adb: String,
    /// adb device serial
    #[arg(long)]
    pub serial: Option<String>,
    /// TCP port (bridge listens on 127.0.0.1:PORT on the goggles)
    #[arg(long, default_value_t = 42420)]
    pub port: u16,
    /// host to connect to (adb forward ends on 127.0.0.1; RNDIS/Wi-Fi: goggles IP + bridge -A)
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    /// data link: auto (the goggles' USB network when it comes up, else adb forward) |
    /// usb-net (USB network only) | adb (adb forward only, no USB switch). The bridge
    /// switches the goggles' USB to network (RNDIS) + adb once per boot and hands out an
    /// address by DHCP; connect that network on the PC once (no special settings) for
    /// ~2–4 ms lower latency. Linux finds the interface; other systems check that the
    /// goggles answer on it
    #[arg(long, default_value = "auto")]
    pub transport: String,
    /// don't run `adb forward` (host bridge, RNDIS, forward set up elsewhere)
    #[arg(long)]
    pub no_forward: bool,
    /// don't start a bridge for this session: use the one already running on the goggles
    /// (started from their menu). Default: start one (push --bridge / a local
    /// eyepiece_bridge if there is one, else the bridge installed on the goggles); it
    /// replaces a bridge started from the menu and exits with this session
    #[arg(long)]
    pub no_deploy: bool,
    /// (the default; kept for scripts)
    #[arg(long, hide = true)]
    pub deploy: bool,
    /// eyepiece_bridge binary for --deploy [default: next to this executable, else
    /// $XDG_DATA_HOME/eyepiece/eyepiece_bridge]
    #[arg(long)]
    pub bridge: Option<PathBuf>,
    /// extra eyepiece_bridge args for --deploy (e.g. --bridge-args=-M)
    #[arg(long, default_value = "", allow_hyphen_values = true)]
    pub bridge_args: String,
    /// ask the bridge to exit when we quit
    #[arg(long)]
    pub stop_bridge: bool,
    /// keep running when the bridge disconnects and reconnect (IDR on every connect)
    #[arg(long)]
    pub reconnect: bool,
    /// don't connect to a bridge at all (encode only: benchmarks, --dump)
    #[arg(long)]
    pub no_connect: bool,

    // ---- source ----
    /// kms | pipewire | x11 | test | file | dxgi | screencapturekit (macOS)
    #[arg(long, default_value = if cfg!(target_os = "macos") { "screencapturekit" } else if cfg!(windows) { "dxgi" } else { "kms" })]
    pub source: String,
    /// kms: DRM device (the GPU driving the displays)
    #[arg(long, default_value = "/dev/dri/card0")]
    pub kms_device: String,
    /// kms: plane id (0 = primary plane of --output / first active screen); see --list
    #[arg(long, default_value_t = 0)]
    pub kms_plane: u32,
    /// kms: output (connector) name, e.g. eDP-1, DP-2
    #[arg(long)]
    pub output: Option<String>,
    /// capture timing: vblank (PC display refresh, default) | poll (1 ms, takes a new
    /// framebuffer as soon as the compositor commits it) | timer (free-running)
    #[arg(long, default_value = "vblank")]
    pub sync: String,
    /// x11: display (default $DISPLAY)
    #[arg(long)]
    pub display: Option<String>,
    /// x11: region WxH+X+Y (default: whole screen)
    #[arg(long)]
    pub region: Option<String>,
    /// file: media file (looped)
    #[arg(long)]
    pub input: Option<String>,
    /// test: lavfi source graph instead of testsrc2 (e.g. 'smptehdbars=size=1920x1080:rate=100')
    #[arg(long)]
    pub lavfi: Option<String>,
    /// pipewire: file to keep the portal's restore token in (no dialog next time)
    #[arg(long)]
    pub restore_token: Option<PathBuf>,
    /// pipewire / screencapturekit: draw the cursor into the stream
    #[arg(long)]
    pub cursor: bool,
    /// macOS: display ID from --list (default: first available display)
    #[arg(long)]
    pub display_id: Option<u32>,
    /// dxgi: adapter,output index
    #[arg(long, default_value = "0,0")]
    pub dxgi: String,
    /// kms test aid without CAP_SYS_ADMIN: real vblank/flip pacing of the plane, content
    /// from VAAPI test surfaces through the same DMA-BUF path
    #[arg(long, hide = true)]
    pub kms_simulate: bool,
    /// list displays (KMS outputs/planes on Linux, ScreenCaptureKit displays on macOS)
    #[arg(long)]
    pub list: bool,

    // ---- video ----
    /// frame rate (panel runs at 100 Hz; lowered to the source refresh if that is lower)
    #[arg(long, default_value_t = 100)]
    pub fps: u32,
    /// h264 | hevc (default: hevc on macOS, VideoToolbox; else h264)
    #[arg(long, default_value = DEFAULT_CODEC)]
    pub codec: String,
    /// stereo 3D: the source is both eyes side by side (left | right) and is sent as
    /// 2560x720 (1280x720 per eye); the goggles switch to their 3D mode (~60 Hz) for the
    /// session. Use with a side-by-side source (e.g. a VR compositor window); --source test
    /// shows a stereo test pattern
    #[arg(long)]
    pub stereo: bool,
    /// auto (VideoToolbox on macOS, else nvenc→vaapi→amf→qsv; then software) | videotoolbox | nvenc | vaapi | amf | qsv | x264 (h264 only) | x265 (hevc only) | FFmpeg name
    #[arg(long, default_value = "auto")]
    pub encoder: String,
    /// VAAPI render node (GPU conversion, VAAPI encode)
    #[arg(long, default_value = "/dev/dri/renderD128")]
    pub vaapi_device: String,
    /// bitrate, e.g. 20M, 800k
    #[arg(long, default_value = "20M")]
    pub bitrate: String,
    /// keyframe interval in frames (0 = 2 s)
    #[arg(long, default_value_t = 0)]
    pub gop: u32,
    /// slices per frame
    #[arg(long, default_value_t = 4)]
    pub slices: u32,
    /// VBV size in frames of bitrate
    #[arg(long, default_value_t = 4.0)]
    pub vbv_frames: f64,
    /// encoder preset (default per encoder: nvenc p1, x264 superfast, …)
    #[arg(long)]
    pub preset: Option<String>,
    /// encoder tune (default: nvenc ull, x264 zerolatency)
    #[arg(long)]
    pub tune: Option<String>,
    /// encoder profile (default: high for H.264)
    #[arg(long)]
    pub profile: Option<String>,
    /// gradual intra refresh instead of periodic IDR (nvenc/x264/x265)
    #[arg(long)]
    pub intra_refresh: bool,
    /// extra encoder AVOption KEY=VALUE (repeatable; overrides defaults)
    #[arg(long = "enc-opt", value_name = "KEY=VALUE")]
    pub enc_opt: Vec<String>,
    /// stretch instead of letterboxing (black bars, aspect kept)
    #[arg(long)]
    pub no_letterbox: bool,
    /// Use CPU scaling instead of VAAPI; on macOS also force BGRA capture/readback
    #[arg(long)]
    pub cpu_scale: bool,
    /// GPU scaler quality: hq | fast | default
    #[arg(long, default_value = "hq")]
    pub scale_mode: String,
    /// extra filters appended to the conversion chain (advanced)
    #[arg(long)]
    pub vf: Option<String>,
    /// head tracking only (implies --head-tracking)
    #[arg(long)]
    pub no_video: bool,
    /// turn on the goggles' head tracking (100 Hz pose → opentrack); off by default, the
    /// IMU stream only runs while a PC asks for it. Implied by --no-video and --print-pose
    #[arg(long)]
    pub head_tracking: bool,

    // ---- outputs / diagnostics ----
    /// send pose to opentrack 'UDP over network' input ('' = off)
    #[arg(long, default_value = "127.0.0.1:4242")]
    pub opentrack: String,
    #[arg(long)]
    pub print_pose: bool,
    /// show the exact encoded stream in an ffplay window at X,Y
    #[arg(long, num_args = 0..=1, default_missing_value = "0,160", value_name = "X,Y")]
    pub preview: Option<String>,
    /// also write the Annex-B stream to FILE
    #[arg(long)]
    pub dump: Option<PathBuf>,
    /// stop after N frames
    #[arg(long)]
    pub frames: Option<u64>,
    /// stop after S seconds
    #[arg(long)]
    pub duration: Option<f64>,
    /// stats interval in seconds
    #[arg(long, default_value_t = 3.0)]
    pub stats_interval: f64,
    /// FFmpeg log level (quiet, error, warning, info, verbose, debug)
    #[arg(long, default_value = "warning")]
    pub ffmpeg_log: String,
}

fn parse_rate(s: &str) -> Result<u64, String> {
    let (n, mul) = match s.chars().last().map(|c| c.to_ascii_lowercase()) {
        Some('k') => (&s[..s.len() - 1], 1e3),
        Some('m') => (&s[..s.len() - 1], 1e6),
        _ => (s, 1.0),
    };
    n.parse::<f64>()
        .map(|v| (v * mul) as u64)
        .map_err(|_| format!("bad rate {s}"))
}

fn parse_size(s: &str) -> Result<(u32, u32), String> {
    let (w, h) = s
        .split_once('x')
        .ok_or_else(|| format!("bad size {s} (WxH)"))?;
    Ok((
        w.parse().map_err(|_| "bad width")?,
        h.parse().map_err(|_| "bad height")?,
    ))
}

#[allow(dead_code)]
fn parse_region(s: &str) -> Result<(u32, u32, i32, i32), String> {
    let mut it = s.split('+');
    let (w, h) = parse_size(it.next().unwrap_or(""))?;
    let x = it
        .next()
        .unwrap_or("0")
        .parse()
        .map_err(|_| "bad region x")?;
    let y = it
        .next()
        .unwrap_or("0")
        .parse()
        .map_err(|_| "bad region y")?;
    Ok((w, h, x, y))
}

fn log_fn() -> LogFn {
    Arc::new(|s: String| eprintln!("[eyepiece-pc] {s}"))
}

fn ffmpeg_level(s: &str) -> i32 {
    match s {
        "quiet" => -8,
        "panic" => 0,
        "fatal" => 8,
        "error" => 16,
        "warning" => 24,
        "info" => 32,
        "verbose" => 40,
        "debug" => 48,
        _ => 24,
    }
}

fn build_source(a: &Args) -> Result<SourceKind, String> {
    Ok(match a.source.as_str() {
        #[cfg(target_os = "linux")]
        "kms" => SourceKind::Kms {
            device: a.kms_device.clone(),
            plane: a.kms_plane,
            output: a.output.clone(),
            simulate: a.kms_simulate,
        },
        #[cfg(all(target_os = "linux", feature = "x11"))]
        "x11" => SourceKind::X11 {
            display: a.display.clone(),
            region: a.region.as_deref().map(parse_region).transpose()?,
            vblank_device: a.kms_device.clone(),
        },
        #[cfg(all(target_os = "linux", feature = "pipewire"))]
        "pipewire" => SourceKind::PipeWire { restore_token_file: a.restore_token.clone(), cursor: a.cursor },
        #[cfg(windows)]
        "dxgi" => {
            let (ad, out) = a.dxgi.split_once(',').unwrap_or(("0", "0"));
            SourceKind::Dxgi { adapter: ad.parse().unwrap_or(0), output: out.parse().unwrap_or(0) }
        }
        #[cfg(target_os = "macos")]
        "screencapturekit" | "sck" => SourceKind::ScreenCaptureKit { display_id: a.display_id, cursor: a.cursor },
        "test" => SourceKind::Test { lavfi: a.lavfi.clone() },
        "file" => SourceKind::File { path: a.input.clone().ok_or("--source file needs --input FILE")? },
        s => return Err(format!("source {s} not available in this build (kms|pipewire|x11|test|file|dxgi|screencapturekit; pipewire needs --features pipewire)")),
    })
}

/// The `eyepiece-pc` binary.
pub fn main() {
    let a = Args::parse();
    let log = log_fn();
    if let Err(e) = real_main(a, log.clone()) {
        log(format!("error: {e}"));
        std::process::exit(1);
    }
}

/// The binary may carry CAP_SYS_ADMIN (file capability, for KMS capture). Keep it only
/// while it is needed: every non-KMS run drops it at once, and KMS runs refuse inputs that
/// could load foreign code (--vf can name plugin filters such as frei0r).
#[cfg(target_os = "linux")]
fn limit_privileges(a: &Args) -> Result<(), String> {
    #[repr(C)]
    struct Hdr {
        version: u32,
        pid: i32,
    }
    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    struct Data {
        effective: u32,
        permitted: u32,
        inheritable: u32,
    }
    const V3: u32 = 0x2008_0522;
    let mut h = Hdr {
        version: V3,
        pid: 0,
    };
    let mut d = [Data::default(); 2];
    let r = unsafe { libc::syscall(libc::SYS_capget, &mut h as *mut Hdr, d.as_mut_ptr()) };
    let has_caps = r == 0 && (d[0].permitted != 0 || d[1].permitted != 0);
    if !has_caps {
        return Ok(());
    }
    let kms = a.source == "kms" && !a.kms_simulate && !a.no_video;
    if kms || a.list {
        if a.vf.is_some() {
            return Err(
                "--vf is refused while running with CAP_SYS_ADMIN (it can load plugin code)".into(),
            );
        }
        return Ok(());
    }
    let mut h = Hdr {
        version: V3,
        pid: 0,
    };
    let z = [Data::default(); 2];
    let r = unsafe { libc::syscall(libc::SYS_capset, &mut h as *mut Hdr, z.as_ptr()) };
    if r != 0 {
        return Err(format!(
            "dropping capabilities failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

/// Where to connect: the goggles' USB network if usable (see `--transport`), else the
/// `adb forward` end (`--host`, 127.0.0.1).
fn pick_host(a: &Args, log: &dyn Fn(String)) -> Result<String, String> {
    let mode = a.transport.as_str();
    match mode {
        "auto" | "usb-net" | "adb" => {}
        m => return Err(format!("--transport {m}: expected auto | usb-net | adb")),
    }
    if mode == "adb" || a.host != "127.0.0.1" {
        return Ok(a.host.clone()); // explicit --host (host bridge, Wi-Fi, …) wins
    }
    pick_usb_net(a, mode == "usb-net", log)
}

/// Linux: find the goggles' RNDIS interface in sysfs and check that it has its address, so
/// a present but unconfigured interface gets a "connect the network" notice.
#[cfg(target_os = "linux")]
fn pick_usb_net(a: &Args, strict: bool, log: &dyn Fn(String)) -> Result<String, String> {
    use eyepiece_link::net;
    // right after the bridge's USB switch the interface appears and DHCP needs a moment
    let wait = Duration::from_secs(if strict { 15 } else { 4 });
    match net::wait_usb_net(wait) {
        Some(n) if n.configured => {
            log(format!(
                "data link: goggles' USB network ({} → {})",
                n.ifname,
                net::GOGGLES_ADDR
            ));
            Ok(net::GOGGLES_ADDR.into())
        }
        Some(n) => {
            let msg = format!(
                "the goggles' USB network ({}) is not connected on this PC. Add it once in your \
                 network settings as a wired connection with \"connect automatically\" on \
                 (automatic/DHCP: the goggles hand out the address) for ~2–4 ms lower latency",
                n.ifname
            );
            if strict {
                return Err(msg);
            }
            log(format!("NOTICE: {msg}. Using adb meanwhile."));
            Ok(a.host.clone())
        }
        None if strict => Err("the goggles' USB network was not found on this PC \
            (bridge started without it, or an older bridge)"
            .into()),
        None => {
            log("data link: adb forward (no USB network from the goggles)".into());
            Ok(a.host.clone())
        }
    }
}

/// Windows, macOS, …: no interface lookup; the goggles must answer on their address
/// (driver bound, DHCP done). Untested on these systems.
#[cfg(not(target_os = "linux"))]
fn pick_usb_net(a: &Args, strict: bool, log: &dyn Fn(String)) -> Result<String, String> {
    use eyepiece_link::net;
    let wait = Duration::from_secs(if strict { 20 } else { 6 });
    if net::wait_goggles_reachable(wait) {
        log(format!(
            "data link: goggles' USB network ({})",
            net::GOGGLES_ADDR
        ));
        return Ok(net::GOGGLES_ADDR.into());
    }
    let why = if cfg!(target_os = "macos") {
        "macOS has no built-in driver for it (RNDIS)"
    } else if cfg!(windows) {
        "Windows needs its RNDIS driver bound to the goggles' network interface (Device Manager)"
    } else {
        "the PC needs an RNDIS driver and DHCP on it"
    };
    let msg = format!("the goggles' USB network did not come up on this PC ({why})");
    if strict {
        return Err(msg);
    }
    log(format!(
        "NOTICE: {msg}. Using adb. Turn off \"USB network\" (--transport adb) to skip this check."
    ));
    Ok(a.host.clone())
}

/// HEVC on macOS: VideoToolbox HEVC costs about 0.3 ms more encode than H.264 there but
/// gives a clearly better picture at the same bitrate.
pub const DEFAULT_CODEC: &str = if cfg!(target_os = "macos") { "hevc" } else { "h264" };

impl Default for Args {
    /// The CLI defaults (`eyepiece-pc` without options).
    fn default() -> Args {
        Args::parse_from(["eyepiece-pc"])
    }
}

/// A session ready to run: adb deploy/forward done, options turned into a [`Config`].
pub struct Prepared {
    pub cfg: Config,
    /// the bridge was pushed + started by us (it exits with the session)
    pub deployed: bool,
    adb: Adb,
}

/// Everything before streaming: privileges, --list, option parsing, adb deploy/forward.
/// `Ok(None)`: nothing to stream (`--list` printed its output).
pub fn prepare(a: &Args, log: &LogFn) -> Result<Option<Prepared>, String> {
    #[cfg(target_os = "linux")]
    limit_privileges(a)?;
    eyepiece_av::ffi::set_log_level(ffmpeg_level(&a.ffmpeg_log));
    #[cfg(target_os = "linux")]
    if a.list {
        print!("{}", eyepiece_capture::kms::list(&a.kms_device)?);
        return Ok(None);
    }
    #[cfg(target_os = "macos")]
    if a.list {
        print!("{}", eyepiece_capture::screencapturekit::list()?);
        return Ok(None);
    }
    let codec: Codec = a.codec.parse()?;
    let bitrate = parse_rate(&a.bitrate)?;
    let sync: SyncMode = a.sync.parse()?;
    let mut opts = vec![];
    for kv in &a.enc_opt {
        let (k, v) = kv
            .split_once('=')
            .ok_or_else(|| format!("--enc-opt {kv}: want KEY=VALUE"))?;
        opts.push((k.to_string(), v.to_string()));
    }
    // the 3D display timing refreshes at ~60 Hz
    let fps = if a.stereo { a.fps.min(60) } else { a.fps };
    let enc = EncoderConfig {
        codec,
        encoder: a.encoder.clone(),
        fps,
        bitrate,
        gop: if a.gop > 0 { a.gop } else { 2 * fps },
        vbv_frames: a.vbv_frames,
        slices: a.slices,
        preset: a.preset.clone(),
        tune: a.tune.clone(),
        profile: a.profile.clone(),
        intra_refresh: a.intra_refresh,
        opts,
        vaapi_device: a.vaapi_device.clone(),
    };

    let adb = Adb::new(a.adb.clone(), a.serial.clone());
    if !a.no_connect {
        if !a.no_deploy {
            let bridge = a.bridge.clone().or_else(find_bridge);
            adb.deploy(
                bridge.as_deref(),
                &a.bridge_args,
                a.transport != "adb",
                &**log,
            )?;
        } else if !a.no_forward && !adb.bridge_running() {
            log(
                "no eyepiece_bridge running on the goggles: start Eyepiece from the goggles' menu \
                 (natartos), or use --deploy"
                    .into(),
            );
        }
        if !a.no_forward {
            adb.forward(a.port, a.port)?;
        }
    }
    let host = if a.no_connect {
        a.host.clone()
    } else {
        pick_host(a, &**log)?
    };
    let link = (!a.no_connect).then(|| LinkOptions {
        addr: format!("{}:{}", host, a.port),
        connect_timeout: Duration::from_secs(15),
        opentrack: (!a.opentrack.is_empty()).then(|| a.opentrack.clone()),
        print_pose: a.print_pose,
        head_tracking: a.head_tracking || a.no_video || a.print_pose,
    });
    let preview = match &a.preview {
        Some(p) => {
            let (x, y) = p.split_once(',').unwrap_or((p, "0"));
            Some((x.parse().unwrap_or(0), y.parse().unwrap_or(0)))
        }
        None => None,
    };
    let cfg = Config {
        source: build_source(a)?,
        sync,
        drm_device: a.kms_device.clone(),
        enc,
        letterbox: !a.no_letterbox,
        gpu_scale_cpu_input: !a.cpu_scale,
        vf: a.vf.clone(),
        scale_mode: a.scale_mode.clone(),
        video: !a.no_video,
        link,
        reconnect: a.reconnect,
        stop_bridge: a.stop_bridge,
        dump: a.dump.clone(),
        preview,
        stats_interval: a.stats_interval,
        max_frames: a.frames,
        duration_s: a.duration,
        stereo: a.stereo,
    };
    Ok(Some(Prepared {
        cfg,
        deployed: !a.no_deploy && !a.no_connect,
        adb,
    }))
}

/// Run the session (blocks until it ends), then the post-session safety step of `--deploy`.
pub fn run_prepared(p: Prepared, ctl: Arc<Control>, ev: Events) -> Result<(), String> {
    let log = ev.log.clone();
    let res = session::run(p.cfg, ctl, ev);
    if p.deployed {
        // the -1 bridge exits with the session; make sure the MCU attitude stream is off
        std::thread::sleep(std::time::Duration::from_secs(3));
        if let Some(l) = p.adb.mcu_stop_safety() {
            log(l);
        }
    }
    res
}

fn real_main(a: Args, log: LogFn) -> Result<(), String> {
    let Some(prepared) = prepare(&a, &log)? else {
        return Ok(());
    };

    let ctl = Arc::new(Control::default());
    {
        let ctl = ctl.clone();
        let log = log.clone();
        // stdin: r = recenter, i = IDR, q = quit (only when interactive)
        if std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            std::thread::spawn(move || {
                for line in std::io::stdin().lock().lines() {
                    let Ok(l) = line else { break };
                    match l.trim() {
                        "r" => ctl.recenter.store(true, Ordering::Relaxed),
                        "i" => ctl.idr.store(true, Ordering::Relaxed),
                        "q" => {
                            ctl.stop.store(true, Ordering::Relaxed);
                            break;
                        }
                        "" => {}
                        x => log(format!("unknown key {x:?} (r/i/q)")),
                    }
                }
            });
        }
    }
    #[cfg(unix)]
    {
        // Ctrl-C: stop cleanly (sends quit to the bridge with --stop-bridge)
        static mut CTL: Option<Arc<Control>> = None;
        extern "C" fn on_sig(_: libc::c_int) {
            #[allow(static_mut_refs)]
            if let Some(c) = unsafe { CTL.as_ref() } {
                if c.stop.swap(true, Ordering::Relaxed) {
                    std::process::exit(130);
                }
            }
        }
        unsafe {
            CTL = Some(ctl.clone());
            libc::signal(libc::SIGINT, on_sig as *const () as libc::sighandler_t);
            libc::signal(libc::SIGTERM, on_sig as *const () as libc::sighandler_t);
        }
    }
    let lg = log.clone();
    run_prepared(
        prepared,
        ctl,
        Events::new(log, Arc::new(move |s| lg(s.line()))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encoder_selection_and_tuning_match_reference() {
        let args = Args::default();
        assert_eq!(args.encoder, "auto");
        assert_eq!(args.codec, DEFAULT_CODEC);
        assert_eq!(args.bitrate, "20M");
        assert_eq!(args.slices, 4);
        assert_eq!(args.vbv_frames, 4.0);
        assert_eq!(args.gop, 0);
        assert!(!args.cpu_scale);
        assert!(!args.no_letterbox);
        assert!(args.preset.is_none());
        assert!(args.tune.is_none());
        assert!(args.profile.is_none());
        assert!(args.enc_opt.is_empty());
    }
}
