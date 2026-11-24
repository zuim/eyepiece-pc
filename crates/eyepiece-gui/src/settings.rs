//! Persistent GUI state: the `eyepiece-pc` options ([`Args`], the same struct the CLI
//! parses), GUI preferences and pinned measurements, in one TOML file in the platform's
//! config directory (Linux `~/.config/eyepiece/eyepiece-gui.toml`, Windows
//! `%APPDATA%\eyepiece\config\eyepiece-gui.toml`).

use eyepiece_pc::cli::Args;
use eyepiece_pc::link::stats::Snapshot;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct UiPrefs {
    /// simple main view (default) instead of the full view
    pub simple: bool,
    pub preview: bool,
    pub preview_width: u32,
    pub preview_fps: u32,
    pub preview_keyframes_only: bool,
    /// seconds of stats a pin averages over
    pub pin_window_s: f64,
}

impl Default for UiPrefs {
    fn default() -> Self {
        UiPrefs { simple: true, preview: false, preview_width: 640, preview_fps: 50, preview_keyframes_only: false, pin_window_s: 10.0 }
    }
}

/// Quality presets of the simple view: (name, explanation, codec, bitrate). They set only
/// codec + bitrate; everything else keeps its value. Basis: device A/B runs (H.264 lowest
/// latency; H.265 about +1 ms PC encode, clearly better picture at the same bitrate).
pub const PRESETS: &[(&str, &str, &str, &str)] = &[
    #[cfg(not(target_os = "macos"))]
    ("Low latency", "H.264, 20 Mbit/s (the default): lowest latency", "h264", "20M"),
    #[cfg(target_os = "macos")]
    ("Balanced", "H.265, 20 Mbit/s (the default on macOS): VideoToolbox, sharp picture at low latency", "hevc", "20M"),
    ("High quality", "H.265, 30 Mbit/s: sharper picture, about 1 ms more encode time", "hevc", "30M"),
    ("Low bandwidth", "H.265, 8 Mbit/s: for slow links; H.265 keeps more detail at low bitrates", "hevc", "8M"),
];

/// Index of the preset the options match, if any.
pub fn preset_of(a: &Args) -> Option<usize> {
    let codec = match a.codec.as_str() {
        "h265" => "hevc",
        "avc" => "h264",
        c => c,
    };
    PRESETS.iter().position(|p| p.2 == codec && p.3.eq_ignore_ascii_case(&a.bitrate))
}

/// The GUI's *USB network* switch over `--transport`: on = `auto` (the goggles' USB network,
/// adb when it does not come up; `usb-net` from Advanced also counts as on), off = `adb`.
pub fn usb_net(a: &Args) -> bool {
    a.transport != "adb"
}

pub fn set_usb_net(a: &mut Args, on: bool) {
    if on != usb_net(a) {
        a.transport = if on { "auto" } else { "adb" }.into();
    }
}

pub const USB_NET_HINT: &str = "Stream over the goggles' USB network (RNDIS): about 2–4 ms lower latency \
     than adb. Falls back to adb with a notice when the network does not come up on this PC. \
     Off: adb only, the goggles' USB is left as it is.";

/// Measured numbers of a pin: means over the pinned window of the per-interval values
/// (percentiles are per stats interval, then averaged).
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Metrics {
    pub intervals: u32,
    pub seconds: f64,
    pub values: Vec<(String, f64)>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Pin {
    pub name: String,
    /// unix seconds
    pub time: u64,
    /// options that differ from the defaults, `key=value`
    pub settings: String,
    /// equivalent `eyepiece-pc` command line
    pub command: String,
    /// encoder actually used etc. (from the session log)
    pub notes: String,
    pub metrics: Metrics,
    /// shown in the Compare table
    #[serde(default = "yes")]
    pub selected: bool,
}

fn yes() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Saved {
    pub args: Args,
    pub ui: UiPrefs,
    pub pins: Vec<Pin>,
}

impl Default for Saved {
    fn default() -> Self {
        // GUI default: 1 s stats (the CLI prints every 3 s) for a livelier plot
        let args = Args { stats_interval: 1.0, ..Args::default() };
        Saved { args, ui: UiPrefs::default(), pins: vec![] }
    }
}

pub fn default_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "eyepiece").map(|d| d.config_dir().join("eyepiece-gui.toml"))
}

impl Saved {
    pub fn load(path: &Path) -> Result<Saved, String> {
        match std::fs::read_to_string(path) {
            Ok(s) => toml::from_str(&s).map_err(|e| format!("{}: {e}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Saved::default()),
            Err(e) => Err(format!("{}: {e}", path.display())),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let s = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
        }
        let tmp = path.with_extension("toml.tmp");
        std::fs::write(&tmp, s).map_err(|e| format!("{}: {e}", tmp.display()))?;
        std::fs::rename(&tmp, path).map_err(|e| format!("{}: {e}", path.display()))
    }
}

/// Options that differ from the CLI defaults as (cli name, value) — generic over every
/// field of [`Args`] via serde, so new CLI options show up without GUI changes.
pub fn changed_options(a: &Args) -> Vec<(String, toml::Value)> {
    let (Ok(cur), Ok(def)) = (toml::Table::try_from(a), toml::Table::try_from(Args::default())) else {
        return vec![];
    };
    let mut out = vec![];
    for (k, v) in &cur {
        if def.get(k) != Some(v) {
            out.push((k.replace('_', "-"), v.clone()));
        }
    }
    // an option set to None is absent from `cur` but present in `def` (none today: every
    // Option defaults to None) — nothing to emit for it
    out
}

fn shell_quote(s: &str) -> String {
    if !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || "-_./:,=+@%".contains(c)) {
        s.to_string()
    } else if cfg!(windows) {
        format!("\"{}\"", s.replace('"', "\\\""))
    } else {
        format!("'{}'", s.replace('\'', "'\\''"))
    }
}

fn value_str(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => s.clone(),
        toml::Value::Float(f) => {
            if f.fract() == 0.0 {
                format!("{f:.0}")
            } else {
                f.to_string()
            }
        }
        other => other.to_string(),
    }
}

/// The options as `eyepiece-pc` arguments (without the program name), only non-defaults.
pub fn to_argv(a: &Args) -> Vec<String> {
    let mut out = vec![];
    for (k, v) in changed_options(a) {
        match &v {
            toml::Value::Boolean(true) => out.push(format!("--{k}")),
            toml::Value::Boolean(false) => {} // no bool option defaults to true
            toml::Value::Array(xs) => {
                for x in xs {
                    out.push(format!("--{k}={}", value_str(x)));
                }
            }
            v => out.push(format!("--{k}={}", value_str(v))),
        }
    }
    out
}

pub fn command_line(a: &Args) -> String {
    let mut s = String::from("eyepiece-pc");
    for x in to_argv(a) {
        s.push(' ');
        match x.split_once('=') {
            Some((k, v)) => {
                s += k;
                s.push('=');
                s += &shell_quote(v);
            }
            None => s += &x,
        }
    }
    s
}

pub fn settings_summary(a: &Args) -> String {
    let v: Vec<String> = changed_options(a)
        .into_iter()
        .filter(|(k, _)| k != "stats-interval")
        .map(|(k, v)| match v {
            toml::Value::Boolean(true) => k,
            v => format!("{k}={}", value_str(&v)),
        })
        .collect();
    if v.is_empty() {
        "defaults".into()
    } else {
        v.join(" ")
    }
}

/// Rows of the latency/throughput table: (label, value of a snapshot, is a latency in ms).
pub type Getter = fn(&Snapshot) -> f64;
pub const METRICS: &[(&str, Getter)] = &[
    ("sent fps", |s| s.sent_fps),
    ("shown fps", |s| s.shown_fps),
    ("Mbit/s", |s| s.mbps),
    ("capture->enc in p50", |s| s.cap_enc_p50_ms),
    ("capture->enc in p95", |s| s.cap_enc_p95_ms),
    ("encode p50", |s| s.enc_p50_ms),
    ("encode p95", |s| s.enc_p95_ms),
    ("capture->sent p50", |s| s.cap_send_p50_ms),
    ("capture->sent p95", |s| s.cap_send_p95_ms),
    ("capture->received p50", |s| s.recv_p50_ms),
    ("capture->received p95", |s| s.recv_p95_ms),
    ("goggles decode p50", |s| s.dec_p50_ms),
    ("goggles decode p95", |s| s.dec_p95_ms),
    ("goggles convert p50", |s| s.cvt_p50_ms),
    ("capture->shown p50", |s| s.lat_p50_ms),
    ("capture->shown p95", |s| s.lat_p95_ms),
    ("skipped /s", |s| s.skipped as f64 / s.interval_s),
    ("IDR /s", |s| s.idr as f64 / s.interval_s),
    ("rtt ms", |s| s.rtt_ms),
];

/// Mean of each metric over the snapshots (NaN values are left out).
pub fn aggregate(snaps: &[&Snapshot]) -> Metrics {
    let values = METRICS
        .iter()
        .map(|(name, get)| {
            let v: Vec<f64> = snaps.iter().map(|s| get(s)).filter(|x| x.is_finite()).collect();
            let m = if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 };
            (name.to_string(), m)
        })
        .collect();
    Metrics { intervals: snaps.len() as u32, seconds: snaps.iter().map(|s| s.interval_s).sum(), values }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn table(a: &Args) -> toml::Table {
        toml::Table::try_from(a).unwrap()
    }

    #[test]
    fn presets_match_defaults_and_aliases() {
        assert_eq!(preset_of(&Args::default()), Some(0), "the CLI default is the first preset");
        let a = Args { codec: "h265".into(), bitrate: "8m".into(), ..Args::default() };
        assert_eq!(preset_of(&a), Some(2));
        let a = Args { bitrate: "25M".into(), ..Args::default() };
        assert_eq!(preset_of(&a), None);
    }

    #[test]
    fn usb_net_switch() {
        let d = Saved::default();
        assert!(usb_net(&d.args), "USB network is on by default");
        assert!(!d.args.head_tracking, "head tracking is off by default");
        let mut a = Args::default();
        set_usb_net(&mut a, false);
        assert_eq!(a.transport, "adb");
        assert_eq!(command_line(&a), "eyepiece-pc --transport=adb");
        set_usb_net(&mut a, true);
        assert_eq!(a.transport, "auto");
        a.transport = "usb-net".into();
        set_usb_net(&mut a, true);
        assert_eq!(a.transport, "usb-net", "strict mode from Advanced is kept");
    }

    #[test]
    fn defaults_give_empty_command() {
        assert_eq!(command_line(&Args::default()), "eyepiece-pc");
    }

    #[test]
    fn argv_roundtrips_through_clap() {
        let a = Args::try_parse_from([
            "eyepiece-pc",
            "--source",
            "screencapturekit",
            "--display-id",
            "2",
            "--encoder",
            "videotoolbox",
            "--stereo",
            "--bitrate",
            "30M",
            "--head-tracking",
            "--enc-opt",
            "rc=cbr",
            "--enc-opt",
            "b_ref_mode=0",
            "--vbv-frames",
            "2.5",
            "--opentrack",
            "",
            "--bridge-args=-M",
            "--preview",
        ])
        .unwrap();
        let mut argv = vec!["eyepiece-pc".to_string()];
        argv.extend(to_argv(&a));
        let b = Args::try_parse_from(&argv).unwrap();
        assert_eq!(table(&a), table(&b), "argv {argv:?}");
    }

    #[test]
    fn saved_roundtrips_through_toml() {
        let mut s = Saved::default();
        s.args.encoder = "nvenc".into();
        s.args.serial = Some("x".into());
        s.pins.push(Pin { name: "a".into(), metrics: aggregate(&[&Snapshot::default()]), ..Default::default() });
        let t = toml::to_string_pretty(&s).unwrap();
        let b: Saved = toml::from_str(&t).unwrap();
        assert_eq!(table(&s.args), table(&b.args));
        assert_eq!(b.pins.len(), 1);
    }
}
