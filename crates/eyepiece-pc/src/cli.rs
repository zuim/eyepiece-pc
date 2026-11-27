//! eyepiece-pc command line: the options, and how they become a [`Config`] (the USB connection
//! included). The `eyepiece-pc` binary is [`main`]; other frontends (the GUI) build an [`Args`]
//! and use [`prepare`] + [`run_prepared`], so every frontend takes the same code path.

use crate::session::{self, Config, Control, Events, PaneConfig, SourceKind};
use clap::{parser::ValueSource, ArgMatches, CommandFactory, FromArgMatches, Parser};
use eyepiece_av::{Codec, EncoderConfig, PackedAlpha};
use eyepiece_capture::SyncMode;
use eyepiece_link::{LinkOptions, LinkTransport, LogFn};
use eyepiece_link::proto::{Info, Pane, PaneFormat, EP_INFO_PANE_BGRA};
use std::io::BufRead;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[command(
    name = "eyepiece-pc",
    version,
    about = "Stream the PC screen to DJI Goggles 3 over USB (Eyepiece PC side)",
    after_help = "Keys (stdin): r + Enter = recenter, i + Enter = force IDR, q + Enter = quit.\n\
Examples:\n  eyepiece-pc                                 # start the bridge on the goggles, KMS capture of the first screen\n  \
eyepiece-pc --output eDP-1 --encoder nvenc --bitrate 30M\n  \
eyepiece-pc --no-launch                     # use the bridge started from the goggles' menu\n  \
eyepiece-pc --source test --host 127.0.0.1:42420   # against pcdisplay_bridge_host -N -p 42420\n  \
eyepiece-pc --list                          # outputs / planes"
)]
pub struct Args {
    // ---- connection (USB: the goggles in tinkerglass USB mode) ----
    /// USB serial number of the goggles (only needed with more than one connected)
    #[arg(long)]
    pub serial: Option<String>,
    /// connect over TCP to HOST:PORT instead of USB, to a bridge started with -p (the host-built
    /// bridge of the interop tests; never the goggles in normal use). Starts no bridge
    #[arg(long, value_name = "HOST:PORT")]
    pub host: Option<String>,
    /// don't start a bridge for this session: use the one already running on the goggles
    /// (started from their menu). Default: start tinkerglass's bridge (PC Display) through the
    /// goggles' control interface; it replaces a bridge started from the menu and exits with
    /// this session
    #[arg(long, alias = "no-deploy")]
    #[serde(alias = "no_deploy")]
    pub no_launch: bool,
    /// (the default; kept for scripts)
    #[arg(long, hide = true)]
    pub deploy: bool,
    /// extra bridge args (e.g. --bridge-args=-M)
    #[arg(long, default_value = "", allow_hyphen_values = true)]
    pub bridge_args: String,
    /// ask the bridge to exit when we quit (always done for a bridge this session launched)
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
    /// stereo 3D side-by-side encoding (GFX1 stereo streaming is not available)
    #[arg(long)]
    pub stereo: bool,
    /// GFX1 picture-in-picture pane: WIDTHxHEIGHT+X+Y (live changes require a compatible bridge)
    #[arg(long)]
    pub pane: Option<String>,
    /// Opacity of the goggles' UI over the picture, 0 to 100 percent; 0 (the default) hides it and
    /// keeps the buttons from it
    #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(u8).range(0..=100))]
    pub gui_opacity: u8,
    /// GFX1 pane opacity, 0 to 100 percent; multiplies source/color-key alpha
    #[arg(long, default_value_t = 100, value_parser = clap::value_parser!(u8).range(0..=100))]
    pub pane_opacity: u8,
    /// source alpha: opaque | source (works with or without --pane)
    #[arg(long, visible_alias = "pane-alpha", default_value = "opaque")]
    #[serde(alias = "pane_alpha")]
    pub alpha: String,
    /// optional color key as RRGGBB, e.g. 00ff00
    #[arg(long)]
    pub color_key: Option<String>,
    /// color-key similarity, 0..1 (0 = exact RGB)
    #[arg(long, default_value_t = 0.0)]
    pub key_similarity: f32,
    /// color-key softness within the similarity radius, 0..1
    #[arg(long, default_value_t = 0.0)]
    pub key_blend: f32,
    /// auto (VideoToolbox on macOS, else nvenc→vaapi (Linux)→amf→qsv; then software) | videotoolbox | nvenc | vaapi (Linux) | amf | qsv | x264 (h264 only) | x265 (hevc only) | FFmpeg name
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
    /// start the local web interface
    #[arg(long)]
    #[serde(skip)]
    pub web: bool,
}

fn parse_rate(s: &str) -> Result<u64, String> {
    let (n, mul) = match s.chars().last().map(|c| c.to_ascii_lowercase()) {
        Some('k') => (&s[..s.len() - 1], 1e3),
        Some('m') => (&s[..s.len() - 1], 1e6),
        _ => (s, 1.0),
    };
    let rate = n.parse::<f64>().map_err(|_| format!("bad rate {s}"))? * mul;
    if !rate.is_finite() || rate <= 0.0 || rate >= i64::MAX as f64 {
        return Err(format!("bad rate {s}"));
    }
    let rate = rate as u64;
    if rate == 0 {
        return Err(format!("bad rate {s}"));
    }
    Ok(rate)
}

fn parse_preview_position(s: &str) -> Result<(i32, i32), String> {
    let (x, y) = match s.split_once(',') {
        Some((x, y)) if !y.contains(',') => (x, y),
        Some(_) => return Err("--preview expects X,Y".into()),
        None => (s, "0"),
    };
    Ok((
        x.parse()
            .map_err(|_| "--preview has invalid X coordinate")?,
        y.parse()
            .map_err(|_| "--preview has invalid Y coordinate")?,
    ))
}

#[cfg(any(windows, test))]
fn parse_dxgi(s: &str) -> Result<(u32, u32), String> {
    let (adapter, output) = s.split_once(',').ok_or("--dxgi expects ADAPTER,OUTPUT")?;
    if output.contains(',') {
        return Err("--dxgi expects ADAPTER,OUTPUT".into());
    }
    Ok((
        adapter
            .parse()
            .map_err(|_| "--dxgi has invalid adapter index")?,
        output
            .parse()
            .map_err(|_| "--dxgi has invalid output index")?,
    ))
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

/// Parse pane geometry without validating it against the panel.
pub fn parse_pane_geometry(spec: &str) -> Result<Pane, String> {
    let mut parts = spec.split('+');
    let size = parts.next().unwrap_or_default();
    let (w, h) = parse_size(size)?;
    let parse_coord = |s: Option<&str>| -> Result<u16, String> {
        s.ok_or_else(|| format!("--pane {spec}: expected WIDTHxHEIGHT+X+Y"))?
            .parse().map_err(|_| format!("--pane {spec}: invalid coordinate"))
    };
    let x = parse_coord(parts.next())?;
    let y = parse_coord(parts.next())?;
    if parts.next().is_some() || w > u16::MAX as u32 || h > u16::MAX as u32 {
        return Err(format!("--pane {spec}: expected WIDTHxHEIGHT+X+Y"));
    }
    Ok(Pane { x, y, width: w as u16, height: h as u16, format: PaneFormat::Bgra })
}

/// Validate overlay options without starting a capture or deploying a bridge.
pub fn pane_config(a: &Args) -> Result<Option<PaneConfig>, String> {
    if !a.key_similarity.is_finite() || !(0.0..=1.0).contains(&a.key_similarity)
        || !a.key_blend.is_finite() || !(0.0..=1.0).contains(&a.key_blend)
    {
        return Err("--key-similarity and --key-blend must be 0..1".into());
    }
    let source_alpha = match a.alpha.as_str() {
        "opaque" => false,
        "source" => true,
        _ => return Err("--alpha must be opaque or source".into()),
    };
    let alpha = if let Some(color) = &a.color_key {
        let hex = color.strip_prefix('#').unwrap_or(color);
        if hex.len() != 6 || !hex.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err("--color-key must be RRGGBB".into());
        }
        PackedAlpha::ColorKey {
            rgb: u32::from_str_radix(hex, 16).map_err(|_| "invalid --color-key")?,
            similarity: a.key_similarity, blend: a.key_blend, source_alpha,
        }
    } else {
        if source_alpha { PackedAlpha::Source } else { PackedAlpha::Opaque }
    };
    let transparent = source_alpha || a.color_key.is_some();
    if a.pane.is_none() && !transparent && (a.no_video || a.no_connect) {
        return Ok(None);
    }
    if a.stereo || a.no_video || (transparent && a.vf.is_some()) {
        return Err("GFX1 output requires mono video; transparency does not support --vf".into());
    }
    let format = if transparent { PaneFormat::Bgra } else { PaneFormat::OpaqueBgra };
    let pane = match &a.pane {
        Some(spec) => Pane { format, ..parse_pane_geometry(spec)? },
        None => Pane { x: 0, y: 0, width: session::PANEL_W as u16, height: session::PANEL_H as u16, format },
    };
    pane.payload(Info { fb_w: session::PANEL_W as u16, fb_h: session::PANEL_H as u16,
                        flags: EP_INFO_PANE_BGRA | eyepiece_link::proto::EP_INFO_PANE_OPAQUE_BGRA, ..Info::default() }).map_err(str::to_string)?;
    pane.encoded_size().map_err(str::to_string)?;
    Ok(Some(PaneConfig { pane, alpha }))
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

fn ffmpeg_level(s: &str) -> Result<i32, String> {
    Ok(match s {
        "quiet" => -8,
        "panic" => 0,
        "fatal" => 8,
        "error" => 16,
        "warning" => 24,
        "info" => 32,
        "verbose" => 40,
        "debug" => 48,
        _ => return Err(format!("invalid --ffmpeg-log {s}")),
    })
}

fn validate_numeric_options(a: &Args) -> Result<(), String> {
    if !(1..=240).contains(&a.fps) {
        return Err("--fps must be 1..240".into());
    }
    if !(1..=32).contains(&a.slices) {
        return Err("--slices must be 1..32".into());
    }
    if !a.vbv_frames.is_finite() || !(0.1..=60.0).contains(&a.vbv_frames) {
        return Err("--vbv-frames must be 0.1..60".into());
    }
    if !a.stats_interval.is_finite() || !(0.1..=60.0).contains(&a.stats_interval) {
        return Err("--stats-interval must be 0.1..60".into());
    }
    if a.duration.is_some_and(|duration| !duration.is_finite() || duration < 0.0) {
        return Err("--duration must be a finite non-negative number".into());
    }
    Ok(())
}

pub(crate) fn build_source(a: &Args) -> Result<SourceKind, String> {
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
            let (adapter, output) = parse_dxgi(&a.dxgi)?;
            SourceKind::Dxgi { adapter, output }
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
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--rpc")) {
        if let Err(e) = crate::rpc::run() {
            eprintln!("[eyepiece-pc] RPC error: {e}");
            std::process::exit(1);
        }
        return;
    }
    let matches = Args::command().get_matches();
    let overrides = explicit_options(&matches);
    let a = Args::from_arg_matches(&matches).unwrap_or_else(|error| error.exit());
    if a.web {
        if let Err(e) = crate::rpc::run_web(a, overrides) {
            eprintln!("[eyepiece-pc] RPC error: {e}");
            std::process::exit(1);
        }
        return;
    }
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

/// The bridge's arguments for a PC session: `-1` (exit with this session: a bridge left
/// running keeps the goggles black), then `--bridge-args`.
pub fn launch_args(bridge_args: &str) -> Vec<String> {
    std::iter::once("-1".to_string()).chain(bridge_args.split_whitespace().map(String::from)).collect()
}

/// USB unless `--host` names a TCP bridge (tests). USB starts the bridge through the control
/// interface first, unless `--no-launch`.
fn pick_connection(a: &Args) -> Result<(String, LinkTransport), String> {
    if let Some(host) = &a.host {
        if host.rsplit_once(':').is_none_or(|(h, p)| h.is_empty() || p.parse::<u16>().is_err()) {
            return Err(format!("--host {host}: expected HOST:PORT"));
        }
        return Ok((host.clone(), LinkTransport::Tcp));
    }
    Ok((
        String::new(),
        LinkTransport::UsbBulk {
            serial: a.serial.clone(),
            launch: (!a.no_launch).then(|| launch_args(&a.bridge_args)),
        },
    ))
}

fn explicit_options(matches: &ArgMatches) -> std::collections::HashSet<String> {
    Args::command()
        .get_arguments()
        .filter(|arg| matches.value_source(arg.get_id().as_str()) == Some(ValueSource::CommandLine))
        .map(|arg| arg.get_id().as_str().to_owned())
        .collect()
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

/// A session ready to run: options turned into a [`Config`]. The bridge is started when the
/// session connects (and again on every reconnect).
pub struct Prepared {
    pub cfg: Config,
}

/// Everything before streaming: privileges, --list, option parsing.
/// `Ok(None)`: nothing to stream (`--list` printed its output).
pub fn prepare(a: &Args, log: &LogFn) -> Result<Option<Prepared>, String> {
    #[cfg(target_os = "linux")]
    limit_privileges(a)?;
    validate_numeric_options(a)?;
    eyepiece_av::ffi::set_log_level(ffmpeg_level(&a.ffmpeg_log)?);
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
    let pane = pane_config(a)?;
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
    if bitrate as f64 * a.vbv_frames / f64::from(fps) > f64::from(i32::MAX) {
        return Err("--bitrate and --vbv-frames exceed the encoder buffer limit".into());
    }
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

    let link = if a.no_connect {
        None
    } else {
        let (addr, transport) = pick_connection(a)?;
        if a.no_launch && a.host.is_none() {
            log("using the bridge that runs on the goggles (More → PC Display there)".into());
        }
        Some((addr, transport))
    };
    let link = link.map(|(addr, transport)| LinkOptions {
        addr,
        transport,
        connect_timeout: Duration::from_secs(15),
        opentrack: (!a.opentrack.is_empty()).then(|| a.opentrack.clone()),
        print_pose: a.print_pose,
        head_tracking: a.head_tracking || a.no_video || a.print_pose,
    });
    let preview = match &a.preview {
        Some(p) => Some(parse_preview_position(p)?),
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
        // a bridge this session launched (-1) exits with it anyway; Quit makes that immediate (over USB a
        // session's end is otherwise only seen at the bridge's read timeout, seconds of a frozen picture).
        // A bridge started from the goggles' menu (--no-launch) stays for the next client.
        stop_bridge: a.stop_bridge || (!a.no_launch && !a.no_connect),
        dump: a.dump.clone(),
        preview,
        stats_interval: a.stats_interval,
        stats_window: None,
        max_frames: a.frames,
        duration_s: a.duration,
        stereo: a.stereo,
        pane,
        gui_opacity: a.gui_opacity,
        pane_opacity: a.pane_opacity,
    };
    Ok(Some(Prepared { cfg }))
}

/// Run the session (blocks until it ends). The bridge started for it exits with it, and
/// tinkerglass' hub stops the head-tracking stream when the bridge leaves.
pub fn run_prepared(p: Prepared, ctl: Arc<Control>, ev: Events) -> Result<(), String> {
    session::run(p.cfg, ctl, ev)
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

    #[test]
    fn usb_is_the_only_normal_connection_and_launches_the_bridge() {
        let (_, t) = pick_connection(&Args::default()).unwrap();
        assert!(matches!(t, LinkTransport::UsbBulk { serial: None, launch: Some(ref a) } if a == &["-1"]));
        let args = Args::try_parse_from(["eyepiece-pc", "--serial", "S", "--bridge-args=-H -F 4"]).unwrap();
        let (_, t) = pick_connection(&args).unwrap();
        assert!(matches!(t, LinkTransport::UsbBulk { serial: Some(ref s), launch: Some(ref a) }
            if s == "S" && a == &["-1", "-H", "-F", "4"]));
        // --no-launch (and the old name) uses the running bridge
        for flag in ["--no-launch", "--no-deploy"] {
            let (_, t) = pick_connection(&Args::try_parse_from(["eyepiece-pc", flag]).unwrap()).unwrap();
            assert!(matches!(t, LinkTransport::UsbBulk { launch: None, .. }));
        }
        // TCP only for a test bridge, by HOST:PORT
        let (addr, t) = pick_connection(&Args::try_parse_from(["eyepiece-pc", "--host", "127.0.0.1:4242"]).unwrap()).unwrap();
        assert!(matches!(t, LinkTransport::Tcp));
        assert_eq!(addr, "127.0.0.1:4242");
        for bad in ["127.0.0.1", ":1", "h:x"] {
            assert!(pick_connection(&Args { host: Some(bad.into()), ..Args::default() }).is_err(), "{bad}");
        }
        // the ADB options are gone
        for removed in ["--adb=adb", "--transport=adb", "--no-forward", "--port=42420"] {
            assert!(Args::try_parse_from(["eyepiece-pc", removed]).is_err(), "{removed}");
        }
    }

    #[test]
    fn bitrate_parser_rejects_non_finite_and_non_positive_values() {
        for rate in ["0", "-1", "NaN", "inf", "9223372036854775808"] {
            assert!(parse_rate(rate).is_err(), "{rate}");
        }
        assert_eq!(parse_rate("20M").unwrap(), 20_000_000);
    }

    #[test]
    fn numeric_options_reject_values_that_can_panic_or_spin() {
        let valid = Args::default();
        validate_numeric_options(&valid).unwrap();
        for args in [
            Args { fps: 0, ..valid.clone() },
            Args { slices: 0, ..valid.clone() },
            Args { vbv_frames: f64::NAN, ..valid.clone() },
            Args { stats_interval: 0.0, ..valid.clone() },
            Args { stats_interval: f64::INFINITY, ..valid.clone() },
            Args { duration: Some(f64::NAN), ..valid.clone() },
            Args { duration: Some(-1.0), ..valid },
        ] {
            assert!(validate_numeric_options(&args).is_err());
        }
        assert!(ffmpeg_level("unknown").is_err());
        assert_eq!(ffmpeg_level("warning").unwrap(), 24);
    }

    #[test]
    fn preview_position_rejects_malformed_coordinates() {
        assert_eq!(parse_preview_position("10,-20").unwrap(), (10, -20));
        assert_eq!(parse_preview_position("10").unwrap(), (10, 0));
        for value in ["x,20", "10,y", "10,20,30"] {
            assert!(parse_preview_position(value).is_err(), "{value}");
        }
    }

    #[test]
    fn dxgi_requires_two_valid_indices() {
        assert_eq!(parse_dxgi("2,3").unwrap(), (2, 3));
        for value in ["2", "x,3", "2,y", "2,3,4"] {
            assert!(parse_dxgi(value).is_err(), "{value}");
        }
    }

    #[test]
    fn web_flag_parses_with_regular_options() {
        let matches = Args::command()
            .try_get_matches_from(["eyepiece-pc", "--source", "test", "--no-connect", "--web"])
            .unwrap();
        let overrides = explicit_options(&matches);
        let args = Args::from_arg_matches(&matches).unwrap();
        assert!(args.web);
        assert_eq!(args.source, "test");
        assert!(args.no_connect);
        assert!(overrides.contains("web"));
        assert!(overrides.contains("source"));
        assert!(overrides.contains("no_connect"));
        assert!(!overrides.contains("fps"));
    }

    #[test]
    fn pane_options_validate_geometry_and_key() {
        let args = Args::try_parse_from([
            "eyepiece-pc", "--pane", "640x480+100+50", "--color-key", "00ff00",
            "--pane-alpha", "source",
        ]).unwrap();
        let pane = pane_config(&args).unwrap().unwrap();
        assert_eq!((pane.pane.width, pane.pane.height, pane.pane.x, pane.pane.y), (640, 480, 100, 50));
        assert!(matches!(pane.alpha, PackedAlpha::ColorKey { rgb: 0x00ff00, source_alpha: true, .. }));
        for bad in ["640x480+1500+0", "641x480+0+0", "640x480+0+0+1", "640x480+-1+0"] {
            let a = Args { pane: Some(bad.into()), ..Args::default() };
            assert!(pane_config(&a).is_err(), "{bad}");
        }
        let a = Args { pane: Some("640x480+0+0".into()), stereo: true, ..Args::default() };
        assert!(pane_config(&a).is_err());
    }

    #[test]
    fn fullscreen_alpha_and_key_use_packed_full_panel() {
        let opaque = pane_config(&Args::default()).unwrap().unwrap();
        assert_eq!(opaque.pane.format, PaneFormat::OpaqueBgra);
        assert_eq!(opaque.pane.encoded_size(), Ok((1920, 1080)));
        assert!(pane_config(&Args { no_connect: true, ..Args::default() }).unwrap().is_none());
        for options in [
            vec!["--alpha", "source"],
            vec!["--pane-alpha", "source"],
            vec!["--color-key", "00ff00"],
            vec!["--alpha", "source", "--color-key", "00ff00"],
        ] {
            let args = Args::try_parse_from(std::iter::once("eyepiece-pc").chain(options)).unwrap();
            let config = pane_config(&args).unwrap().unwrap();
            assert_eq!((config.pane.x, config.pane.y, config.pane.width, config.pane.height), (0, 0, 1920, 1080));
            assert_eq!(config.pane.encoded_size(), Ok((3840, 1080)));
            assert!(pane_config(&Args { stereo: true, ..args }).is_err());
        }
        assert!(pane_config(&Args { key_blend: 1.1, ..Args::default() }).is_err());
        assert_eq!(pane_config(&Args { key_blend: 0.1, key_similarity: 0.1, ..Args::default() }).unwrap().unwrap().pane.format, PaneFormat::OpaqueBgra);
        assert!(pane_config(&Args { reconnect: true, ..Args::default() }).is_ok());
        assert!(pane_config(&Args { stereo: true, ..Args::default() }).is_err());
    }

    #[test]
    fn disabling_transparency_uses_unpacked_output_with_or_without_pip() {
        for pane in [None, Some("640x480+100+50".to_string())] {
            for key in [false, true] {
                let mut args = Args { pane: pane.clone(), ..Args::default() };
                if key {
                    args.color_key = Some("00ff00".into());
                } else {
                    args.alpha = "source".into();
                }
                let transparent = pane_config(&args).unwrap().unwrap();
                assert_eq!(transparent.pane.format, PaneFormat::Bgra);

                args.alpha = "opaque".into();
                args.color_key = None;
                let opaque = pane_config(&args).unwrap().unwrap();
                assert_eq!(opaque.pane.format, PaneFormat::OpaqueBgra);
                assert!(matches!(opaque.alpha, PackedAlpha::Opaque));
                assert_eq!(
                    opaque.pane.encoded_size(),
                    Ok((opaque.pane.width, opaque.pane.height))
                );
            }
        }
    }
}
