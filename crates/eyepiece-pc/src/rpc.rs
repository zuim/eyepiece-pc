use crate::cli::{self, Args};
use crate::session::{CaptureUpdate, Control, Events, GuiUpdate};
use clap::Parser;
use eyepiece_av::encoder;
use eyepiece_link::stats::Snapshot;
#[cfg(test)]
use prost::Message;
use serde::{Deserialize, Serialize};
use serde_json::{Value as Json, json};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
#[cfg(test)]
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use eyepiece_link::proto::wire;
mod preview;
mod web;

use wire::{Property, Value, value};

struct Request {
    id: String,
    command: Option<request::Command>,
}

mod request {
    use super::wire;

    pub enum Command {
        GetState(wire::GetState),
        SetProperties(wire::SetProperties),
        Start(wire::Start),
        Stop(wire::Stop),
        Restart(wire::Restart),
        Recenter(wire::Recenter),
        ForceIdr(wire::ForceIdr),
        SetPreview(wire::SetPreview),
        SetCommandLine(wire::SetCommandLine),
        ListKmsOutputs(wire::ListKmsOutputs),
    }
}

#[derive(Clone)]
struct Response {
    id: String,
    event: Option<response::Event>,
}

mod response {
    use super::wire;

    #[derive(Clone, Debug)]
    pub enum Event {
        State(wire::State),
        Error(wire::Error),
        Log(wire::Log),
        Pose(wire::Pose),
        Metrics(wire::Metrics),
        Preview(wire::Preview),
        KmsOutputs(wire::KmsOutputs),
    }
}

const SCHEMA: &str = include_str!("../../../contract/settings.json");
const MAX_FRAME: usize = 8 * 1024 * 1024;

fn decode_request(data: &[u8]) -> Result<Request, String> {
    use wire::envelope::Message;
    let envelope = eyepiece_link::proto::decode_envelope(data, MAX_FRAME)
        .map_err(|e| format!("invalid RPC envelope: {e}"))?;
    let command = match envelope.message.ok_or("RPC envelope has no message")? {
        Message::GetState(value) => request::Command::GetState(value),
        Message::SetProperties(value) => request::Command::SetProperties(value),
        Message::Start(value) => request::Command::Start(value),
        Message::Stop(value) => request::Command::Stop(value),
        Message::Restart(value) => request::Command::Restart(value),
        Message::Recenter(value) => request::Command::Recenter(value),
        Message::ForceIdr(value) => request::Command::ForceIdr(value),
        Message::SetPreview(value) => request::Command::SetPreview(value),
        Message::SetCommandLine(value) => request::Command::SetCommandLine(value),
        Message::ListKmsOutputs(value) => request::Command::ListKmsOutputs(value),
        _ => return Err("unexpected RPC request message".into()),
    };
    Ok(Request { id: envelope.id.unwrap_or_default(), command: Some(command) })
}

fn response_envelope(response: Response) -> Result<wire::Envelope, String> {
    use wire::envelope::Message;
    let message = match response.event.ok_or("RPC response has no event")? {
        response::Event::State(value) => Message::State(value),
        response::Event::Error(value) => Message::Error(value),
        response::Event::Log(value) => Message::Log(value),
        response::Event::Pose(value) => Message::Pose(value),
        response::Event::Metrics(value) => Message::Metrics(value),
        response::Event::Preview(value) => Message::Preview(value),
        response::Event::KmsOutputs(value) => Message::KmsOutputs(value),
    };
    Ok(wire::Envelope {
        id: (!response.id.is_empty()).then_some(response.id),
        sender: None,
        recipient: None,
        message: Some(message),
    })
}

#[cfg(test)]
fn request_envelope(request: Request) -> wire::Envelope {
    use wire::envelope::Message;
    let message = request.command.map(|command| match command {
        request::Command::GetState(value) => Message::GetState(value),
        request::Command::SetProperties(value) => Message::SetProperties(value),
        request::Command::Start(value) => Message::Start(value),
        request::Command::Stop(value) => Message::Stop(value),
        request::Command::Restart(value) => Message::Restart(value),
        request::Command::Recenter(value) => Message::Recenter(value),
        request::Command::ForceIdr(value) => Message::ForceIdr(value),
        request::Command::SetPreview(value) => Message::SetPreview(value),
        request::Command::SetCommandLine(value) => Message::SetCommandLine(value),
        request::Command::ListKmsOutputs(value) => Message::ListKmsOutputs(value),
    });
    wire::Envelope { id: Some(request.id), sender: None, recipient: None, message }
}

#[derive(Clone, Serialize, Deserialize)]
struct Schema {
    version: u32,
    settings: Vec<Setting>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Setting {
    name: String,
    group: String,
    description: String,
    #[serde(rename = "type")]
    kind: String,
    default: Json,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing)]
    platform_defaults: Option<std::collections::HashMap<String, Json>>,
}

#[derive(Default)]
struct CapabilityCache {
    encoders: HashMap<(String, String), Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Saved {
    args: Args,
    desired_running: bool,
}

enum Input {
    Request(Request),
    Started(u64),
    Link(u64, bool),
    CaptureApplied(u64, Box<Args>, bool, bool, bool, Result<(), String>),
    GuiApplied(u64, u8, bool, Result<(), String>),
    Finished(u64, Result<(), String>),
    Closed,
    Failed(String),
}

struct Session {
    id: u64,
    args: Args,
    live_target: Args,
    control: Arc<Control>,
    thread: Option<JoinHandle<()>>,
}

impl Drop for Session {
    fn drop(&mut self) {
        self.control.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            if thread.join().is_err() {
                eprintln!("[eyepiece-pc] RPC session thread panicked");
            }
        }
    }
}

fn schema() -> Result<Schema, String> {
    let mut schema: Schema =
        serde_json::from_str(SCHEMA).map_err(|e| format!("invalid settings schema: {e}"))?;
    let platform = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(windows) {
        "windows"
    } else {
        "linux"
    };
    for setting in &mut schema.settings {
        if let Some(default) = setting
            .platform_defaults
            .as_ref()
            .and_then(|defaults| defaults.get(platform))
        {
            setting.default = default.clone();
        }
    }
    Ok(schema)
}

fn source_supported(source: &str) -> bool {
    match source {
        "kms" => cfg!(target_os = "linux"),
        "pipewire" => cfg!(all(target_os = "linux", feature = "pipewire")),
        "x11" => cfg!(all(target_os = "linux", feature = "x11")),
        "dxgi" => cfg!(windows),
        "screencapturekit" => cfg!(target_os = "macos"),
        "test" | "file" => true,
        _ => false,
    }
}

#[cfg(any(target_os = "linux", test))]
fn parse_kms_outputs(listing: &str) -> Vec<String> {
    listing
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("output "))
        .filter_map(|line| line.split_whitespace().next().map(str::to_owned))
        .collect()
}

fn list_kms_outputs(device: &str) -> Result<Vec<String>, String> {
    #[cfg(target_os = "linux")]
    {
        let listing = crate::capture::kms::list(device)?;
        Ok(parse_kms_outputs(&listing))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = device;
        Err("KMS output discovery is only available on Linux".into())
    }
}

fn encoder_choices(
    codec_name: &str,
    vaapi_device: &str,
    aliases: &[String],
    cache: &mut CapabilityCache,
) -> Result<Vec<String>, String> {
    let key = (codec_name.to_string(), vaapi_device.to_string());
    if let Some(choices) = cache.encoders.get(&key) {
        return Ok(choices.clone());
    }
    let codec: encoder::Codec = codec_name.parse()?;
    let choices = aliases
        .iter()
        .filter(|alias| {
            if alias.as_str() == "auto" {
                encoder::resolve(alias, codec, vaapi_device).is_ok()
            } else {
                encoder::probe(&encoder::encoder_name(alias, codec), vaapi_device)
            }
        })
        .cloned()
        .collect::<Vec<_>>();
    cache.encoders.insert(key, choices.clone());
    Ok(choices)
}

fn selected_encoder_family(
    args: &Args,
    aliases: &[String],
    available: &[String],
) -> Option<encoder::Family> {
    let codec: encoder::Codec = args.codec.parse().ok()?;
    let name = encoder::resolve(&args.encoder, codec, &args.vaapi_device).ok()?;
    if aliases.contains(&args.encoder) {
        if !available.contains(&args.encoder) {
            return None;
        }
    } else if !encoder::probe(&name, &args.vaapi_device) {
        return None;
    }
    Some(encoder::family(&name))
}

fn setting_visible(
    name: &str,
    args: &Args,
    encoder_choices: &[String],
    family: Option<encoder::Family>,
) -> bool {
    let source = args.source.as_str();
    let video = !args.no_video;

    match name {
        "no_connect" => true,
        "bridge_args" => !args.no_connect && !args.no_launch && args.host.is_none(),
        "serial" | "no_launch" => !args.no_connect && args.host.is_none(),
        "host" | "reconnect" | "stop_bridge" => !args.no_connect,
        "source" | "fps" | "cursor" | "sync" | "output" | "kms_device" | "kms_plane"
        | "kms_simulate" | "display" | "region" | "input" | "lavfi" | "restore_token"
        | "display_id" | "dxgi" => {
            if !video {
                return false;
            }
            if !matches!(name, "source" | "fps") && !source_supported(source) {
                return false;
            }
            match name {
                "source" | "fps" => true,
                "cursor" => matches!(source, "pipewire" | "screencapturekit"),
                "sync" => matches!(source, "kms" | "x11"),
                "output" | "kms_plane" | "kms_simulate" => source == "kms",
                "kms_device" => matches!(source, "kms" | "x11"),
                "display" | "region" => source == "x11",
                "input" => source == "file",
                "lavfi" => source == "test",
                "restore_token" => source == "pipewire",
                "display_id" => source == "screencapturekit",
                "dxgi" => source == "dxgi",
                _ => false,
            }
        }
        "encoder" | "codec" | "bitrate" | "stereo" | "pane" | "alpha" | "color_key"
        | "key_similarity" | "key_blend" | "vaapi_device" | "gop" | "slices" | "vbv_frames"
        | "preset" | "tune" | "profile" | "intra_refresh" | "enc_opt" | "no_letterbox"
        | "cpu_scale" | "scale_mode" | "vf" => {
            if !video {
                return false;
            }
            use encoder::Family;
            match name {
                "vaapi_device" => {
                    family == Some(Family::Vaapi)
                        || (args.encoder == "auto"
                            && encoder_choices.iter().any(|choice| choice == "vaapi"))
                        || (source == "kms" && args.kms_simulate)
                }
                "preset" => matches!(
                    family,
                    Some(Family::Nvenc | Family::Qsv | Family::X264 | Family::X265)
                ),
                "tune" => matches!(family, Some(Family::Nvenc | Family::X264 | Family::X265)),
                "profile" => match family {
                    Some(Family::Nvenc | Family::Vaapi) => true,
                    Some(Family::Amf | Family::Qsv | Family::VideoToolbox | Family::X264) => {
                        args.codec == "h264"
                    }
                    _ => false,
                },
                "intra_refresh" => {
                    matches!(family, Some(Family::Nvenc | Family::X264 | Family::X265))
                }
                "slices" => family == Some(Family::X265),
                "key_similarity" | "key_blend" => args.color_key.is_some(),
                _ => true,
            }
        }
        _ => true,
    }
}

fn state_schema(base: &Schema, args: &Args, cache: &mut CapabilityCache) -> Result<Schema, String> {
    let encoder_aliases = base
        .settings
        .iter()
        .find(|setting| setting.name == "encoder")
        .and_then(|setting| setting.choices.as_deref())
        .ok_or("encoder choices are missing from settings schema")?;
    let encoder_options = encoder_choices(&args.codec, &args.vaapi_device, encoder_aliases, cache)?;
    let family = selected_encoder_family(args, encoder_aliases, &encoder_options);
    let mut codecs = Vec::new();
    for codec in ["h264", "hevc"] {
        if !encoder_choices(codec, &args.vaapi_device, encoder_aliases, cache)?.is_empty() {
            codecs.push(codec.to_string());
        }
    }
    let mut schema = base.clone();
    for setting in &mut schema.settings {
        match setting.name.as_str() {
            "source" => {
                if let Some(choices) = &mut setting.choices {
                    choices.retain(|source| source_supported(source));
                }
            }
            "encoder" => setting.choices = Some(encoder_options.clone()),
            "codec" => setting.choices = Some(codecs.clone()),
            _ => {}
        }
        if !setting_visible(&setting.name, args, &encoder_options, family) {
            setting.visible = Some(false);
        }
    }
    Ok(schema)
}

fn default_args(schema: &Schema) -> Result<Args, String> {
    let mut args = serde_json::to_value(Args::default()).map_err(|e| e.to_string())?;
    for setting in &schema.settings {
        args[&setting.name] = setting.default.clone();
    }
    serde_json::from_value(args).map_err(|e| format!("invalid schema defaults: {e}"))
}

fn settings_path() -> Result<PathBuf, String> {
    directories::ProjectDirs::from("", "", "eyepiece")
        .map(|p| p.config_dir().join("eyepiece-rpc.json"))
        .ok_or_else(|| "cannot determine settings directory".into())
}

fn load(path: &Path, schema: &Schema) -> Result<Saved, String> {
    match std::fs::read(path) {
        Ok(bytes) => {
            let mut saved: Saved = serde_json::from_slice(&bytes)
                .map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
            // settings of the ADB era: "host" was the adb-forward address ("127.0.0.1", with
            // the port apart); USB needs none
            if saved.args.host.as_deref().is_some_and(|h| !h.contains(':')) {
                saved.args.host = None;
            }
            validate_args(&saved.args, schema)?;
            Ok(saved)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Saved {
            args: default_args(schema)?,
            desired_running: false,
        }),
        Err(e) => Err(format!("cannot read {}: {e}", path.display())),
    }
}

fn save(path: &Path, saved: &Saved) -> Result<(), String> {
    let parent = path.parent().ok_or("settings path has no parent")?;
    std::fs::create_dir_all(parent).map_err(|e| format!("create settings directory: {e}"))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system clock error: {e}"))?
        .as_nanos();
    let temp = parent.join(format!(".eyepiece-rpc-{}-{nonce}.tmp", std::process::id()));
    let bytes = serde_json::to_vec_pretty(saved).map_err(|e| e.to_string())?;
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let result = (|| -> io::Result<()> {
        let mut file = opts.open(&temp)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        std::fs::rename(&temp, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result.map_err(|e| format!("save {}: {e}", path.display()))
}

fn validate_value(setting: &Setting, v: &Json) -> Result<(), String> {
    let optional = setting.kind.starts_with("optional_");
    if optional && v.is_null() {
        return Ok(());
    }
    let valid = match setting.kind.as_str() {
        "boolean" => v.is_boolean(),
        "string" | "optional_string" => v.is_string(),
        "choice" => v.as_str().is_some_and(|s| {
            setting
                .choices
                .as_ref()
                .is_some_and(|c| c.iter().any(|x| x == s))
        }),
        "integer" | "optional_integer" => v.as_u64().is_some(),
        "number" | "optional_number" => v.as_f64().is_some_and(f64::is_finite),
        "strings" => v.as_array().is_some_and(|a| a.iter().all(Json::is_string)),
        _ => false,
    };
    if !valid {
        return Err(format!("invalid value for {}", setting.name));
    }
    if let Some(n) = v.as_f64() {
        if setting.min.is_some_and(|min| n < min) || setting.max.is_some_and(|max| n > max) {
            return Err(format!("{} is outside its allowed range", setting.name));
        }
    }
    Ok(())
}

fn validate_args(args: &Args, schema: &Schema) -> Result<(), String> {
    let values = serde_json::to_value(args).map_err(|e| e.to_string())?;
    for setting in &schema.settings {
        validate_value(setting, &values[&setting.name])?;
    }
    cli::pane_config(args)?;
    command_line(args, schema)?;
    Ok(())
}

fn apply_cli_overrides(
    saved: &mut Saved,
    args: Args,
    overrides: std::collections::HashSet<String>,
    schema: &Schema,
) -> Result<bool, String> {
    if args.list || args.deploy {
        return Err("--list and --deploy cannot be used with --web".into());
    }

    let mut values = serde_json::to_value(&saved.args).map_err(|e| e.to_string())?;
    let cli_values = serde_json::to_value(args).map_err(|e| e.to_string())?;
    let mut changed = false;
    for name in overrides {
        if name == "web" {
            continue;
        }
        let setting = schema
            .settings
            .iter()
            .find(|setting| setting.name == name)
            .ok_or_else(|| format!("--{} cannot be used with --web", name.replace('_', "-")))?;
        values[&setting.name] = cli_values[&setting.name].clone();
        changed = true;
    }
    if !changed {
        return Ok(false);
    }

    let args: Args =
        serde_json::from_value(values).map_err(|e| format!("invalid CLI settings: {e}"))?;
    validate_args(&args, schema)?;
    saved.args = args;
    Ok(true)
}

fn from_wire(v: Value) -> Result<Json, String> {
    match v.kind.ok_or("property value is missing")? {
        value::Kind::Boolean(v) => Ok(json!(v)),
        value::Kind::Integer(v) => Ok(json!(v)),
        value::Kind::Number(v) if v.is_finite() => Ok(json!(v)),
        value::Kind::Number(_) => Err("non-finite property value".into()),
        value::Kind::Text(v) => Ok(json!(v)),
        value::Kind::Strings(v) => Ok(json!(v.values)),
        value::Kind::Unset(true) => Ok(Json::Null),
        value::Kind::Unset(false) => Err("invalid unset value".into()),
    }
}

fn to_wire(v: &Json) -> Result<Value, String> {
    let kind = match v {
        Json::Null => value::Kind::Unset(true),
        Json::Bool(v) => value::Kind::Boolean(*v),
        Json::Number(v) if v.is_i64() || v.is_u64() => {
            value::Kind::Integer(v.as_i64().ok_or("integer exceeds RPC range")?)
        }
        Json::Number(v) => value::Kind::Number(v.as_f64().ok_or("invalid number")?),
        Json::String(v) => value::Kind::Text(v.clone()),
        Json::Array(v) => value::Kind::Strings(wire::StringList {
            values: v
                .iter()
                .map(|x| x.as_str().map(str::to_owned).ok_or("invalid string list"))
                .collect::<Result<_, _>>()?,
        }),
        Json::Object(_) => return Err("invalid property object".into()),
    };
    Ok(Value { kind: Some(kind) })
}

fn properties(args: &Args, schema: &Schema) -> Result<Vec<Property>, String> {
    let json = serde_json::to_value(args).map_err(|e| e.to_string())?;
    schema
        .settings
        .iter()
        .map(|s| {
            Ok(Property {
                name: s.name.clone(),
                value: Some(to_wire(&json[&s.name])?),
            })
        })
        .collect()
}

fn apply_properties(
    args: &Args,
    schema: &Schema,
    properties: Vec<Property>,
) -> Result<Args, String> {
    let mut json = serde_json::to_value(args).map_err(|e| e.to_string())?;
    let mut seen = std::collections::HashSet::new();
    for property in properties {
        if !seen.insert(property.name.clone()) {
            return Err(format!("duplicate property {}", property.name));
        }
        let setting = schema
            .settings
            .iter()
            .find(|s| s.name == property.name)
            .ok_or_else(|| format!("unknown property {}", property.name))?;
        let v = from_wire(property.value.ok_or("property value is missing")?)?;
        validate_value(setting, &v)?;
        json[&property.name] = v;
    }
    let args: Args = serde_json::from_value(json).map_err(|e| format!("invalid settings: {e}"))?;
    validate_args(&args, schema)?;
    Ok(args)
}

fn command_line(args: &Args, schema: &Schema) -> Result<String, String> {
    let current = serde_json::to_value(args).map_err(|e| e.to_string())?;
    let defaults = serde_json::to_value(Args::default()).map_err(|e| e.to_string())?;
    let mut words = vec!["eyepiece-pc".to_string()];
    for setting in &schema.settings {
        let value = &current[&setting.name];
        if value == &defaults[&setting.name] || value.is_null() {
            continue;
        }
        let flag = format!("--{}", setting.name.replace('_', "-"));
        match value {
            Json::Bool(true) => words.push(flag),
            Json::Bool(false) => {
                return Err(format!("cannot represent {} on the CLI", setting.name));
            }
            Json::Array(values) => {
                for value in values {
                    words.push(format!(
                        "{flag}={}",
                        value.as_str().ok_or("invalid string list")?
                    ));
                }
            }
            Json::String(value) => words.push(format!("{flag}={value}")),
            Json::Number(value) => words.push(format!("{flag}={value}")),
            _ => return Err(format!("cannot represent {} on the CLI", setting.name)),
        }
    }
    shlex::try_join(words.iter().map(String::as_str))
        .map_err(|e| format!("cannot quote command: {e}"))
}

fn parse_command_line(command: &str, schema: &Schema) -> Result<Args, String> {
    let mut words = shlex::split(command).ok_or("invalid command line quoting")?;
    if words
        .first()
        .is_some_and(|word| word.ends_with("eyepiece-pc") || word.ends_with("eyepiece-pc.exe"))
    {
        words.remove(0);
    }
    let args = Args::try_parse_from(std::iter::once("eyepiece-pc".to_string()).chain(words))
        .map_err(|e| e.to_string())?;
    if args.list || args.deploy {
        return Err("--list and --deploy are not supported in web settings".into());
    }
    validate_args(&args, schema)?;
    Ok(args)
}

fn emit(tx: &SyncSender<Response>, id: &str, event: response::Event) -> Result<(), String> {
    tx.send(Response {
        id: id.into(),
        event: Some(event),
    })
    .map_err(|e| format!("RPC output closed: {e}"))
}

fn state(
    saved: &Saved,
    schema: &Schema,
    capabilities: &mut CapabilityCache,
    phase: &str,
    connected: bool,
    session: Option<&Session>,
) -> Result<wire::State, String> {
    let display_schema = state_schema(schema, &saved.args, capabilities)?;
    Ok(wire::State {
        version: env!("CARGO_PKG_VERSION").into(),
        settings_schema_json: serde_json::to_string(&display_schema)
            .map_err(|e| format!("serialize settings schema: {e}"))?,
        properties: properties(&saved.args, schema)?,
        phase: phase.into(),
        desired_running: saved.desired_running,
        connected,
        command_line: command_line(&saved.args, schema)?,
        running_command_line: session
            .map(|s| command_line(&s.args, schema))
            .transpose()?
            .unwrap_or_default(),
        running_properties: session
            .map(|s| properties(&s.args, schema))
            .transpose()?
            .unwrap_or_default(),
    })
}

fn metrics(s: &Snapshot) -> wire::Metrics {
    wire::Metrics {
        interval_s: s.interval_s,
        sent_fps: s.sent_fps,
        mbps: s.mbps,
        shown_fps: s.shown_fps,
        lat_p50_ms: s.lat_p50_ms,
        lat_p95_ms: s.lat_p95_ms,
        cap_send_p50_ms: s.cap_send_p50_ms,
        cap_send_p95_ms: s.cap_send_p95_ms,
        cap_enc_p50_ms: s.cap_enc_p50_ms,
        cap_enc_p95_ms: s.cap_enc_p95_ms,
        enc_p50_ms: s.enc_p50_ms,
        enc_p95_ms: s.enc_p95_ms,
        dec_p50_ms: s.dec_p50_ms,
        dec_p95_ms: s.dec_p95_ms,
        cvt_p50_ms: s.cvt_p50_ms,
        cvt_p95_ms: s.cvt_p95_ms,
        recv_p50_ms: s.recv_p50_ms,
        recv_p95_ms: s.recv_p95_ms,
        q_max: s.q_max,
        dropped: s.dropped,
        skipped: s.skipped,
        idr: s.idr,
        captured_fps: s.captured_fps,
        pose_hz: s.pose_hz,
        rtt_ms: s.rtt_ms,
        report_interval_s: s.report_interval_s,
        sampled_at_ms: match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_secs_f64() * 1000.0,
            Err(error) => {
                eprintln!("[eyepiece-pc] cannot timestamp statistics: {error}");
                0.0
            }
        },
    }
}

fn spawn_session(
    id: u64,
    args: Args,
    input: mpsc::Sender<Input>,
    output: SyncSender<Response>,
    preview: Arc<Mutex<preview::Subscription>>,
) -> Session {
    let session_args = args.clone();
    let control = Arc::new(Control::default());
    let ctl = control.clone();
    let thread = thread::spawn(move || {
        let log = {
            let output = output.clone();
            Arc::new(move |line: String| {
                let _ = output.try_send(Response {
                    id: String::new(),
                    event: Some(response::Event::Log(wire::Log { line })),
                });
            }) as eyepiece_link::LogFn
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut prepared =
                cli::prepare(&args, &log)?.ok_or("cannot list outputs in RPC mode")?;
            prepared.cfg.stats_window = Some(1.0);
            if let Some(link) = prepared.cfg.link.as_mut() {
                link.print_pose = false;
            }
            let _ = input.send(Input::Started(id));
            let out_stats = output.clone();
            let mut events = Events::new(
                log,
                Arc::new(move |s| {
                    if out_stats
                        .send(Response {
                            id: String::new(),
                            event: Some(response::Event::Metrics(metrics(s))),
                        })
                        .is_err()
                    {
                        eprintln!("[eyepiece-pc] RPC statistics output closed");
                    }
                }),
            );
            let out_link = output.clone();
            let in_link = input.clone();
            let ctl_link = ctl.clone();
            events.on_link = Some(Arc::new(move |link| {
                let _ = in_link.send(Input::Link(id, link.is_some()));
                if let Some(link) = link {
                    let output = out_link.clone();
                    let ctl = ctl_link.clone();
                    thread::spawn(move || {
                        let mut last = None;
                        while link.alive() && !ctl.stop.load(Ordering::Relaxed) {
                            let pose = *link.shared.last_pose.lock().unwrap();
                            if let Some(p) = pose {
                                if last != Some(p.t_dev_us) {
                                    last = Some(p.t_dev_us);
                                    let _ = output.try_send(Response {
                                        id: String::new(),
                                        event: Some(response::Event::Pose(wire::Pose {
                                            yaw: p.yaw,
                                            pitch: p.pitch,
                                            roll: p.roll,
                                            device_us: p.t_dev_us,
                                            active: p.active != 0,
                                            source: p.source.into(),
                                        })),
                                    });
                                }
                            }
                            thread::sleep(Duration::from_millis(16));
                        }
                    });
                }
            }));
            let (tap, preview_worker) = preview::spawn(
                prepared.cfg.enc.codec,
                prepared.cfg.enc.clone(),
                preview,
                output,
            )?;
            events.au = Some(tap);
            let result = cli::run_prepared(prepared, ctl, events);
            preview_worker
                .join()
                .map_err(|_| "preview worker panicked")?;
            result
        }))
        .unwrap_or_else(|_| Err("session thread panicked".into()));
        let _ = input.send(Input::Finished(id, result));
    });
    Session {
        id,
        live_target: session_args.clone(),
        args: session_args,
        control,
        thread: Some(thread),
    }
}

fn queue_capture_update(session: &mut Session, desired: &Args, input: &mpsc::Sender<Input>) {
    let update_source = matches!(session.args.source.as_str(), "file" | "test")
        && matches!(desired.source.as_str(), "file" | "test")
        && (desired.source != "file" || desired.input.is_some())
        && (session.live_target.source != desired.source
            || (desired.source == "file" && session.live_target.input != desired.input)
            || (desired.source == "test" && session.live_target.lavfi != desired.lavfi));
    let running_pane = cli::pane_config(&session.live_target).ok().flatten();
    let desired_pane = cli::pane_config(desired).ok().flatten();
    let update_pane = matches!((running_pane, desired_pane), (Some(a), Some(b)) if a.pane != b.pane);
    let update_alpha = matches!((running_pane, desired_pane), (Some(a), Some(b))
        if (a.pane == b.pane && a.pane.format == eyepiece_link::proto::PaneFormat::Bgra) || update_pane)
        && (session.live_target.alpha != desired.alpha
            || session.live_target.color_key != desired.color_key
            || session.live_target.key_similarity != desired.key_similarity
            || session.live_target.key_blend != desired.key_blend);
    if !update_source && !update_alpha && !update_pane {
        return;
    }
    let source = if update_source {
        match cli::build_source(desired) {
            Ok(source) => Some(source),
            Err(error) => {
                let _ = input.send(Input::CaptureApplied(
                    session.id,
                    Box::new(desired.clone()),
                    true,
                    false,
                    false,
                    Err(error),
                ));
                return;
            }
        }
    } else {
        None
    };
    let alpha = if update_alpha { desired_pane.map(|p| p.alpha) } else { None };
    let pane = if update_pane { desired_pane } else { None };
    let args = desired.clone();
    let id = session.id;
    let input = input.clone();
    copy_capture_fields(&mut session.live_target, desired, update_source, update_alpha, update_pane);
    session.control.queue_capture_update(CaptureUpdate {
        source,
        alpha,
        pane,
        complete: Box::new(move |result| {
            let _ = input.send(Input::CaptureApplied(
                id,
                Box::new(args),
                update_source,
                update_alpha,
                update_pane,
                result,
            ));
        }),
    });
}

fn queue_gui_update(session: &mut Session, desired: &Args, input: &mpsc::Sender<Input>) {
    for pane in [false, true] {
        let target = if pane { &mut session.live_target.pane_opacity } else { &mut session.live_target.gui_opacity };
        let opacity = if pane { desired.pane_opacity } else { desired.gui_opacity };
        if *target == opacity { continue; }
        *target = opacity;
        let id = session.id;
        let input = input.clone();
        session.control.gui_updates.lock().unwrap().push_back(GuiUpdate {
            opacity, pane,
            complete: Box::new(move |result| {
                let _ = input.send(Input::GuiApplied(id, opacity, pane, result));
            }),
        });
    }
}

fn copy_capture_fields(target: &mut Args, source: &Args, update_source: bool, update_alpha: bool, update_pane: bool) {
    if update_source {
        target.source = source.source.clone();
        target.input = source.input.clone();
        target.lavfi = source.lavfi.clone();
    }
    if update_alpha {
        target.alpha = source.alpha.clone();
        target.color_key = source.color_key.clone();
        target.key_similarity = source.key_similarity;
        target.key_blend = source.key_blend;
    }
    if update_pane {
        target.pane = source.pane.clone();
    }
}

fn read_request(input: &mut impl Read) -> Result<Option<Request>, String> {
    let mut len = [0; 4];
    loop {
        match input.read(&mut len[..1]) {
            Ok(0) => return Ok(None),
            Ok(_) => break,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.to_string()),
        }
    }
    input
        .read_exact(&mut len[1..])
        .map_err(|e| format!("truncated frame header: {e}"))?;
    let len = u32::from_be_bytes(len) as usize;
    if len == 0 || len > MAX_FRAME {
        return Err(format!("invalid RPC frame length {len}"));
    }
    let mut data = vec![0; len];
    input
        .read_exact(&mut data)
        .map_err(|e| format!("truncated frame: {e}"))?;
    decode_request(data.as_slice())
        .map(Some)
        .map_err(|e| format!("invalid RPC request: {e}"))
}

fn write_response(output: &mut impl Write, response: Response) -> Result<(), String> {
    let frame = eyepiece_link::proto::encode_frame(&response_envelope(response)?, MAX_FRAME)
        .map_err(|e| format!("encode RPC response: {e}"))?;
    output
        .write_all(&frame)
        .and_then(|_| output.flush())
        .map_err(|e| format!("write RPC response: {e}"))
}

pub fn run() -> Result<(), String> {
    if std::env::args_os().len() != 2 {
        return Err("--rpc cannot be combined with other command-line options".into());
    }
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::sync_channel::<Response>(32);
    let reader = input_tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        loop {
            match read_request(&mut stdin) {
                Ok(Some(req)) => {
                    if reader.send(Input::Request(req)).is_err() {
                        break;
                    }
                }
                Ok(None) => {
                    let _ = reader.send(Input::Closed);
                    break;
                }
                Err(e) => {
                    let _ = reader.send(Input::Failed(e));
                    break;
                }
            }
        }
    });
    let writer_input = input_tx.clone();
    thread::spawn(move || {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        for event in output_rx {
            if let Err(e) = write_response(&mut stdout, event) {
                let _ = writer_input.send(Input::Failed(e));
                break;
            }
        }
    });
    run_worker(input_tx, input_rx, output_tx, None)
}

pub fn run_web(args: Args, overrides: std::collections::HashSet<String>) -> Result<(), String> {
    web::run(args, overrides)
}

fn run_worker(
    input_tx: mpsc::Sender<Input>,
    input_rx: mpsc::Receiver<Input>,
    output_tx: SyncSender<Response>,
    cli_overrides: Option<(Args, std::collections::HashSet<String>)>,
) -> Result<(), String> {
    let schema = schema()?;
    let path = settings_path()?;
    let mut saved = load(&path, &schema)?;
    if let Some((args, overrides)) = cli_overrides {
        if apply_cli_overrides(&mut saved, args, overrides, &schema)? {
            save(&path, &saved)?;
        }
    }
    let preview = Arc::new(Mutex::new(preview::Subscription::default()));
    let mut phase = "idle";
    let mut connected = false;
    let mut session: Option<Session> = None;
    let mut next_session_id = 1u64;
    let mut restart = false;
    let mut capabilities = CapabilityCache::default();
    if saved.desired_running {
        phase = "starting";
        session = Some(spawn_session(
            next_session_id,
            saved.args.clone(),
            input_tx.clone(),
            output_tx.clone(),
            preview.clone(),
        ));
        next_session_id += 1;
    }
    emit(
        &output_tx,
        "",
        response::Event::State(state(
            &saved,
            &schema,
            &mut capabilities,
            phase,
            connected,
            session.as_ref(),
        )?),
    )?;
    let mut failure = None;
    while let Ok(message) = input_rx.recv() {
        let mut id = String::new();
        let mut error = None;
        let mut event = None;
        match message {
            Input::Closed => break,
            Input::Failed(e) => {
                failure = Some(e);
                break;
            }
            Input::Started(id) => {
                if session.as_ref().is_some_and(|s| s.id == id) && phase != "stopping" {
                    phase = "running";
                    if let Some(s) = &mut session {
                        queue_capture_update(s, &saved.args, &input_tx);
                        queue_gui_update(s, &saved.args, &input_tx);
                    }
                }
            }
            Input::Link(id, value) => {
                if session.as_ref().is_some_and(|s| s.id == id) {
                    connected = value;
                }
            }
            Input::CaptureApplied(id, args, source, alpha, pane, result) => {
                if let Some(s) = session.as_mut().filter(|s| s.id == id) {
                    match result {
                        Ok(()) => copy_capture_fields(&mut s.args, &args, source, alpha, pane),
                        Err(e) => {
                            if source && s.live_target.source == args.source
                                && s.live_target.input == args.input
                                && s.live_target.lavfi == args.lavfi
                            {
                                copy_capture_fields(&mut s.live_target, &s.args, true, false, false);
                            }
                            if alpha && s.live_target.alpha == args.alpha
                                && s.live_target.color_key == args.color_key
                                && s.live_target.key_similarity == args.key_similarity
                                && s.live_target.key_blend == args.key_blend
                            {
                                copy_capture_fields(&mut s.live_target, &s.args, false, true, false);
                            }
                            if pane && s.live_target.pane == args.pane {
                                copy_capture_fields(&mut s.live_target, &s.args, false, false, true);
                            }
                            emit(&output_tx, "", response::Event::Error(wire::Error {
                                message: format!("live capture update failed: {e}"),
                            }))?;
                        }
                    }
                }
            }
            Input::GuiApplied(id, opacity, pane, result) => {
                if let Some(s) = session.as_mut().filter(|s| s.id == id) {
                    let applied = if pane { &mut s.args.pane_opacity } else { &mut s.args.gui_opacity };
                    let target = if pane { &mut s.live_target.pane_opacity } else { &mut s.live_target.gui_opacity };
                    match result {
                        Ok(()) => *applied = opacity,
                        Err(e) => {
                            if !pane { *applied = 100; }
                            if *target == opacity {
                                *target = *applied;
                            }
                            emit(&output_tx, "", response::Event::Error(wire::Error {
                                message: format!("live {} opacity update failed: {e}", if pane { "pane" } else { "GUI" }),
                            }))?;
                        }
                    }
                }
            }
            Input::Finished(id, result) => {
                if !session.as_ref().is_some_and(|s| s.id == id) {
                    continue;
                }
                if let Some(mut old) = session.take() {
                    if let Some(thread) = old.thread.take() {
                        thread.join().map_err(|_| "session thread panicked")?;
                    }
                }
                connected = false;
                phase = "idle";
                if let Err(e) = result {
                    emit(
                        &output_tx,
                        "",
                        response::Event::Error(wire::Error { message: e }),
                    )?;
                }
                if restart && saved.desired_running {
                    restart = false;
                    phase = "starting";
                    session = Some(spawn_session(
                        next_session_id,
                        saved.args.clone(),
                        input_tx.clone(),
                        output_tx.clone(),
                        preview.clone(),
                    ));
                    next_session_id += 1;
                } else if saved.desired_running {
                    saved.desired_running = false;
                    save(&path, &saved)?;
                }
            }
            Input::Request(req) => {
                id = req.id;
                let result = (|| -> Result<(), String> {
                    match req.command.ok_or("missing RPC command")? {
                        request::Command::GetState(_) => {}
                        command @ (request::Command::SetProperties(_)
                        | request::Command::SetCommandLine(_)) => {
                            let args = match command {
                                request::Command::SetProperties(update) => {
                                    apply_properties(&saved.args, &schema, update.properties)?
                                }
                                request::Command::SetCommandLine(update) => {
                                    parse_command_line(&update.command_line, &schema)?
                                }
                                _ => unreachable!(),
                            };
                            let old = std::mem::replace(&mut saved.args, args);
                            if let Err(e) = save(&path, &saved) {
                                saved.args = old;
                                return Err(e);
                            }
                            if let Some(s) = &mut session {
                                let old_tracking =
                                    old.head_tracking || old.no_video || old.print_pose;
                                let new_tracking = saved.args.head_tracking
                                    || saved.args.no_video
                                    || saved.args.print_pose;
                                if old_tracking != new_tracking {
                                    s.control.set_head_tracking(new_tracking);
                                    s.args.head_tracking = new_tracking;
                                }
                                if phase == "running" {
                                    queue_capture_update(s, &saved.args, &input_tx);
                                    queue_gui_update(s, &saved.args, &input_tx);
                                }
                            }
                        }
                        request::Command::Start(_) => {
                            if session.is_some() {
                                return Err("session is already running".into());
                            }
                            saved.desired_running = true;
                            if let Err(e) = save(&path, &saved) {
                                saved.desired_running = false;
                                return Err(e);
                            }
                            phase = "starting";
                            session = Some(spawn_session(
                                next_session_id,
                                saved.args.clone(),
                                input_tx.clone(),
                                output_tx.clone(),
                                preview.clone(),
                            ));
                            next_session_id += 1;
                        }
                        request::Command::Stop(_) => {
                            let previous = saved.desired_running;
                            saved.desired_running = false;
                            if let Err(e) = save(&path, &saved) {
                                saved.desired_running = previous;
                                return Err(e);
                            }
                            restart = false;
                            if let Some(s) = &session {
                                phase = "stopping";
                                s.control.stop.store(true, Ordering::Relaxed);
                            }
                        }
                        request::Command::Restart(_) => {
                            if session.is_none() {
                                return Err("session is not running".into());
                            }
                            let previous = saved.desired_running;
                            saved.desired_running = true;
                            if let Err(e) = save(&path, &saved) {
                                saved.desired_running = previous;
                                return Err(e);
                            }
                            restart = true;
                            phase = "stopping";
                            session
                                .as_ref()
                                .unwrap()
                                .control
                                .stop
                                .store(true, Ordering::Relaxed);
                        }
                        request::Command::Recenter(_) => {
                            session
                                .as_ref()
                                .ok_or("session is not running")?
                                .control
                                .recenter
                                .store(true, Ordering::Relaxed);
                        }
                        request::Command::ForceIdr(_) => {
                            session
                                .as_ref()
                                .ok_or("session is not running")?
                                .control
                                .idr
                                .store(true, Ordering::Relaxed);
                        }
                        request::Command::SetPreview(p) => {
                            preview.lock().unwrap().set(&p)?;
                            if p.enabled {
                                if let Some(s) = &session {
                                    s.control.idr.store(true, Ordering::Relaxed);
                                }
                            }
                        }
                        request::Command::ListKmsOutputs(_) => {
                            event = Some(response::Event::KmsOutputs(wire::KmsOutputs {
                                outputs: list_kms_outputs(&saved.args.kms_device)?,
                            }));
                        }
                    }
                    Ok(())
                })();
                if let Err(e) = result {
                    error = Some(e);
                }
            }
        }
        if let Some(e) = error {
            emit(
                &output_tx,
                &id,
                response::Event::Error(wire::Error { message: e }),
            )?;
        } else if let Some(event) = event {
            emit(&output_tx, &id, event)?;
        } else {
            emit(
                &output_tx,
                &id,
                response::Event::State(state(
                    &saved,
                    &schema,
                    &mut capabilities,
                    phase,
                    connected,
                    session.as_ref(),
                )?),
            )?;
        }
    }
    if let Some(mut s) = session {
        s.control.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = s.thread.take() {
            thread.join().map_err(|_| "session thread panicked")?;
        }
    }
    match failure {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gui_opacity_updates_wait_for_completion() {
        let args = Args::default();
        let control = Arc::new(Control::default());
        let mut session = Session {
            id: 1, args: args.clone(), live_target: args.clone(),
            control: control.clone(), thread: None,
        };
        let (tx, rx) = mpsc::channel();
        let desired = Args { gui_opacity: 25, pane_opacity: 50, ..args };
        queue_gui_update(&mut session, &desired, &tx);
        queue_gui_update(&mut session, &desired, &tx);
        assert_eq!(session.args.gui_opacity, 0);
        assert_eq!(session.args.pane_opacity, 100);
        let update = control.gui_updates.lock().unwrap().pop_front().unwrap();
        assert_eq!((update.opacity, update.pane), (25, false));
        (update.complete)(Err("unsupported".into()));
        assert!(matches!(rx.recv().unwrap(), Input::GuiApplied(1, 25, false, Err(_))));
        let update = control.gui_updates.lock().unwrap().pop_front().unwrap();
        assert_eq!((update.opacity, update.pane), (50, true));
        assert!(control.gui_updates.lock().unwrap().is_empty());
        (update.complete)(Ok(()));
        assert!(matches!(rx.recv().unwrap(), Input::GuiApplied(1, 50, true, Ok(()))));
        let schema = schema().unwrap();
        for name in ["gui_opacity", "pane_opacity"] {
            let setting = schema.settings.iter().find(|setting| setting.name == name).unwrap();
            assert!(validate_value(setting, &serde_json::json!(0)).is_ok());
            assert!(validate_value(setting, &serde_json::json!(100)).is_ok());
            assert!(validate_value(setting, &serde_json::json!(101)).is_err());
            assert!(validate_value(setting, &serde_json::json!(-1)).is_err());
        }
    }

    #[test]
    fn live_updates_only_queue_compatible_capture_changes() {
        let args = Args {
            source: "file".into(),
            input: Some("first.mp4".into()),
            pane: Some("640x480+0+0".into()),
            alpha: "source".into(),
            ..Args::default()
        };
        let control = Arc::new(Control::default());
        let mut session = Session {
            id: 1, args: args.clone(), live_target: args.clone(),
            control: control.clone(), thread: None,
        };
        let (tx, rx) = mpsc::channel();
        let mut desired = args.clone();
        desired.input = Some("second.mp4".into());
        desired.color_key = Some("00ff00".into());
        queue_capture_update(&mut session, &desired, &tx);
        queue_capture_update(&mut session, &desired, &tx);
        let update = control.capture_updates.lock().unwrap().pop_front().unwrap();
        assert!(update.source.is_some() && update.alpha.is_some());
        assert!(control.capture_updates.lock().unwrap().is_empty());
        (update.complete)(Ok(()));
        assert!(matches!(rx.recv().unwrap(), Input::CaptureApplied(1, _, true, true, false, Ok(()))));

        queue_capture_update(&mut session, &args, &tx);
        assert!(control.capture_updates.lock().unwrap().pop_front().is_some());
        let mut resized = args.clone();
        resized.pane = Some("320x240+0+0".into());
        resized.key_blend = 0.5;
        queue_capture_update(&mut session, &resized, &tx);
        let update = control.capture_updates.lock().unwrap().pop_front().unwrap();
        assert_eq!(update.pane.unwrap().pane.width, 320);
        assert!(update.alpha.is_some());
    }

    #[test]
    fn disabling_transparency_queues_reconfiguration() {
        let running = Args { alpha: "source".into(), ..Args::default() };
        let mut session = Session {
            id: 1,
            args: running.clone(),
            live_target: running,
            control: Arc::new(Control::default()),
            thread: None,
        };
        let desired = Args::default();
        let (input, _receiver) = mpsc::channel();
        queue_capture_update(&mut session, &desired, &input);

        let update = session.control.capture_updates.lock().unwrap().pop_front().unwrap();
        assert_eq!(update.pane.unwrap().pane.format, eyepiece_link::proto::PaneFormat::OpaqueBgra);
        assert_eq!(session.args.alpha, "source");
        assert_eq!(session.live_target.alpha, "opaque");
    }

    #[test]
    fn pending_pane_updates_coalesce_to_latest_settings() {
        let args = Args { source: "test".into(), pane: Some("640x480+0+0".into()), ..Args::default() };
        let control = Arc::new(Control::default());
        let mut session = Session { id: 1, args: args.clone(), live_target: args.clone(), control: control.clone(), thread: None };
        let (tx, rx) = mpsc::channel();
        let mut desired = args;
        desired.pane = Some("320x240+10+20".into());
        desired.color_key = Some("00ff00".into());
        queue_capture_update(&mut session, &desired, &tx);
        desired.pane = Some("320x240+30+40".into());
        desired.key_blend = 0.5;
        queue_capture_update(&mut session, &desired, &tx);
        desired.key_blend = 0.75;
        queue_capture_update(&mut session, &desired, &tx);
        let update = control.capture_updates.lock().unwrap().pop_front().unwrap();
        let pane = update.pane.unwrap();
        assert_eq!((pane.pane.x, pane.pane.y), (30, 40));
        assert!(matches!(pane.alpha, eyepiece_av::PackedAlpha::ColorKey { blend, .. } if blend == 0.75));
        assert!(control.capture_updates.lock().unwrap().is_empty());
        (update.complete)(Ok(()));
        assert!(matches!(rx.recv().unwrap(), Input::CaptureApplied(1, _, false, true, true, Ok(()))));
        assert!(matches!(rx.recv().unwrap(), Input::CaptureApplied(1, _, false, true, true, Ok(()))));
        assert!(matches!(rx.recv().unwrap(), Input::CaptureApplied(1, _, false, true, false, Ok(()))));
    }

    #[test]
    fn related_settings_follow_source_and_encoder() {
        let mut args = Args::default();
        args.source = "file".into();
        assert!(setting_visible("input", &args, &[], None));
        assert!(!setting_visible("sync", &args, &[], None));
        assert!(!setting_visible("kms_plane", &args, &[], None));

        args.encoder = "software".into();
        args.codec = "hevc".into();
        let family = Some(encoder::Family::X265);
        assert!(setting_visible("preset", &args, &[], family));
        assert!(setting_visible("slices", &args, &[], family));
        assert!(setting_visible("intra_refresh", &args, &[], family));
        assert!(!setting_visible("profile", &args, &[], family));
        assert!(!setting_visible("vaapi_device", &args, &[], family));

        args.no_video = true;
        assert!(!setting_visible("source", &args, &[], family));
        assert!(!setting_visible("encoder", &args, &[], family));
        assert!(setting_visible("no_connect", &args, &[], family));

        args.no_video = false;
        args.no_launch = true;
        assert!(!setting_visible("bridge_args", &args, &[], family));
        assert!(setting_visible("stop_bridge", &args, &[], family));
        args.no_connect = true;
        assert!(!setting_visible("no_launch", &args, &[], family));
        assert!(!setting_visible("stop_bridge", &args, &[], family));
        assert!(setting_visible("no_connect", &args, &[], family));
    }

    #[test]
    fn a_tcp_test_bridge_hides_the_usb_settings() {
        let mut args = Args::default();
        for name in ["host", "serial", "no_launch", "bridge_args"] {
            assert!(setting_visible(name, &args, &[], None), "{name}");
        }
        args.host = Some("127.0.0.1:42420".into());
        assert!(setting_visible("host", &args, &[], None));
        for name in ["serial", "no_launch", "bridge_args"] {
            assert!(!setting_visible(name, &args, &[], None), "{name}");
        }
        args.no_connect = true;
        assert!(!setting_visible("host", &args, &[], None));
    }

    #[test]
    fn source_choices_match_build_features() {
        assert_eq!(source_supported("kms"), cfg!(target_os = "linux"));
        assert_eq!(
            source_supported("pipewire"),
            cfg!(all(target_os = "linux", feature = "pipewire"))
        );
        assert_eq!(
            source_supported("x11"),
            cfg!(all(target_os = "linux", feature = "x11"))
        );
        assert_eq!(source_supported("dxgi"), cfg!(windows));
        assert_eq!(
            source_supported("screencapturekit"),
            cfg!(target_os = "macos")
        );
        assert!(source_supported("test"));
        assert!(source_supported("file"));
    }

    #[test]
    fn parses_kms_output_names_only() {
        assert_eq!(
            parse_kms_outputs(
                "/dev/dri/card0:\n  output eDP-1     crtc  42 1920x1080 @ 60 Hz\n  plane 1 output eDP-1\n  output DP-2      crtc   0 off\n"
            ),
            ["eDP-1", "DP-2"]
        );
    }

    #[test]
    fn settings_defaults_and_updates_are_validated() {
        let schema = schema().unwrap();
        let args = default_args(&schema).unwrap();
        validate_args(&args, &schema).unwrap();
        assert!(args.cursor);
        assert!(args.head_tracking);
        assert_eq!(args.stats_interval, 0.1);
        let exposed_schema = serde_json::to_value(&schema).unwrap();
        assert_eq!(exposed_schema["version"], 1);
        assert_eq!(exposed_schema["settings"][0]["group"], "Transport");
        assert!(
            exposed_schema["settings"][12]
                .get("platform_defaults")
                .is_none()
        );
        let expected = serde_json::to_value(Args::default()).unwrap();
        for setting in &schema.settings {
            assert!(
                expected.get(&setting.name).is_some(),
                "unknown schema setting {}",
                setting.name
            );
            if !["cursor", "head_tracking", "stats_interval"].contains(&setting.name.as_str()) {
                assert_eq!(
                    expected[&setting.name], setting.default,
                    "wrong default for {}",
                    setting.name
                );
            }

            validate_value(setting, &setting.default)
                .unwrap_or_else(|e| panic!("invalid default: {e}"));
        }
        let names: std::collections::HashSet<_> =
            schema.settings.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(
            names.len(),
            schema.settings.len(),
            "duplicate schema setting"
        );
        for name in expected.as_object().unwrap().keys() {
            assert!(
                names.contains(name.as_str()) || name == "deploy" || name == "list",
                "missing schema setting {name}"
            );
        }
        let invalid = Property {
            name: "fps".into(),
            value: Some(Value {
                kind: Some(value::Kind::Integer(0)),
            }),
        };
        assert!(apply_properties(&args, &schema, vec![invalid]).is_err());
        let invalid_choice = Property {
            name: "scale_mode".into(),
            value: Some(Value {
                kind: Some(value::Kind::Text("invalid".into())),
            }),
        };
        assert!(apply_properties(&args, &schema, vec![invalid_choice]).is_err());
        let invalid_range = Property {
            name: "stats_interval".into(),
            value: Some(Value {
                kind: Some(value::Kind::Number(61.0)),
            }),
        };
        assert!(apply_properties(&args, &schema, vec![invalid_range]).is_err());
        let below_minimum = Property {
            name: "stats_interval".into(),
            value: Some(Value {
                kind: Some(value::Kind::Number(0.09)),
            }),
        };
        assert!(apply_properties(&args, &schema, vec![below_minimum]).is_err());
        let unknown = Property {
            name: "missing".into(),
            value: Some(Value {
                kind: Some(value::Kind::Boolean(true)),
            }),
        };
        assert!(apply_properties(&args, &schema, vec![unknown]).is_err());
    }

    #[test]
    fn web_cli_overrides_only_explicit_settings() {
        let schema = schema().unwrap();
        let mut saved = Saved {
            args: default_args(&schema).unwrap(),
            desired_running: false,
        };
        saved.args.codec = "hevc".into();
        let args = Args::try_parse_from(["eyepiece-pc", "--source", "test", "--web"]).unwrap();

        assert!(
            apply_cli_overrides(
                &mut saved,
                args,
                std::collections::HashSet::from(["source".into(), "web".into()]),
                &schema,
            )
            .unwrap()
        );
        assert_eq!(saved.args.source, "test");
        assert_eq!(saved.args.codec, "hevc");
        assert!(!saved.args.web);
    }

    #[test]
    fn web_cli_rejects_non_setting_actions() {
        let schema = schema().unwrap();
        let mut saved = Saved {
            args: default_args(&schema).unwrap(),
            desired_running: false,
        };
        let args = Args {
            list: true,
            ..Args::default()
        };
        assert!(
            apply_cli_overrides(
                &mut saved,
                args,
                std::collections::HashSet::from(["list".into()]),
                &schema,
            )
            .is_err()
        );
    }

    #[test]
    fn web_cli_applies_diagnostic_setting() {
        let schema = schema().unwrap();
        let mut saved = Saved {
            args: default_args(&schema).unwrap(),
            desired_running: false,
        };
        let args = Args {
            ffmpeg_log: "debug".into(),
            ..Args::default()
        };
        assert!(
            apply_cli_overrides(
                &mut saved,
                args,
                std::collections::HashSet::from(["ffmpeg_log".into()]),
                &schema,
            )
            .unwrap()
        );
        assert_eq!(saved.args.ffmpeg_log, "debug");
    }

    #[test]
    fn property_updates_are_typed_and_atomic() {
        let schema = schema().unwrap();
        let args = default_args(&schema).unwrap();
        let fps = || Property {
            name: "fps".into(),
            value: Some(Value {
                kind: Some(value::Kind::Integer(60)),
            }),
        };
        let changed = apply_properties(&args, &schema, vec![fps()]).unwrap();
        assert_eq!(changed.fps, 60);
        assert_eq!(args.fps, 100);
        let invalid = Property {
            name: "head_tracking".into(),
            value: Some(Value {
                kind: Some(value::Kind::Text("true".into())),
            }),
        };
        assert!(apply_properties(&args, &schema, vec![fps(), invalid]).is_err());
        assert_eq!(args.fps, 100);
        assert!(apply_properties(&args, &schema, vec![fps(), fps()]).is_err());
    }

    #[test]
    fn command_line_roundtrips_cli_defaults_and_quoted_values() {
        let schema = schema().unwrap();
        let mut args = default_args(&schema).unwrap();
        args.bridge_args = "-O 'plane:1'".into();
        args.enc_opt = vec!["a=b c".into(), "x=y".into()];
        args.opentrack.clear();
        let command = command_line(&args, &schema).unwrap();
        let parsed = parse_command_line(&command, &schema).unwrap();
        assert_eq!(
            serde_json::to_value(args).unwrap(),
            serde_json::to_value(parsed).unwrap()
        );
        assert!(parse_command_line("eyepiece-pc --fps 0", &schema).is_err());
        assert!(parse_command_line("eyepiece-pc --list", &schema).is_err());
        assert!(parse_command_line("eyepiece-pc --bitrate 'oops", &schema).is_err());
        let invalid = Property {
            name: "bridge_args".into(),
            value: Some(Value {
                kind: Some(value::Kind::Text("bad\0value".into())),
            }),
        };
        assert!(apply_properties(&default_args(&schema).unwrap(), &schema, vec![invalid]).is_err());
    }

    #[test]
    fn preview_envelope_moves_access_unit_without_copying() {
        let access_unit = vec![1, 2, 3];
        let data = access_unit.as_ptr();
        let response = Response {
            id: String::new(),
            event: Some(response::Event::Preview(wire::Preview {
                access_unit,
                ..Default::default()
            })),
        };
        let envelope = response_envelope(response).unwrap();
        let Some(wire::envelope::Message::Preview(preview)) = envelope.message else {
            panic!("expected preview");
        };
        assert_eq!(preview.access_unit.as_ptr(), data);
        assert_eq!(preview.access_unit, [1, 2, 3]);
    }

    #[test]
    fn frame_roundtrip_and_limits() {
        let response = Response {
            id: "x".into(),
            event: Some(response::Event::Error(wire::Error { message: "test".into() })),
        };
        let mut bytes = vec![];
        write_response(&mut bytes, response.clone()).unwrap();
        assert_eq!(
            u32::from_be_bytes(bytes[..4].try_into().unwrap()) as usize,
            bytes.len() - 4
        );
        assert_eq!(wire::Envelope::decode(&bytes[4..]).unwrap().id.as_deref(), Some("x"));
        assert!(read_request(&mut &b"\xff\xff\xff\xff"[..]).is_err());
        assert!(read_request(&mut &b"\0\0\0\x02\x08"[..]).is_err());
        assert!(decode_request(&wire::Envelope { id: Some("x".into()), ..Default::default() }.encode_to_vec()).is_err());
        assert!(decode_request(&response_envelope(response).unwrap().encode_to_vec()).is_err());
        let request = Request {
            id: "abc".into(),
            command: Some(request::Command::GetState(wire::GetState {})),
        };
        let encoded = request_envelope(request).encode_to_vec();
        let mut frame = (encoded.len() as u32).to_be_bytes().to_vec();
        frame.extend(encoded);
        assert_eq!(
            read_request(&mut frame.as_slice()).unwrap().unwrap().id,
            "abc"
        );
        struct InterruptedOnce<'a>(bool, &'a [u8]);
        impl Read for InterruptedOnce<'_> {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                if !self.0 {
                    self.0 = true;
                    return Err(io::ErrorKind::Interrupted.into());
                }
                self.1.read(buf)
            }
        }
        assert_eq!(read_request(&mut InterruptedOnce(false, &frame)).unwrap().unwrap().id, "abc");
        assert!(read_request(&mut &[][..]).unwrap().is_none());
    }

    #[test]
    fn settings_persist_and_reload() {
        let schema = schema().unwrap();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("eyepiece-rpc-test-{}-{unique}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("settings.json");
        let mut saved = load(&path, &schema).unwrap();
        saved.args.fps = 48;
        saved.args.cursor = false;
        saved.args.head_tracking = false;
        saved.desired_running = true;
        save(&path, &saved).unwrap();
        let restored = load(&path, &schema).unwrap();
        assert_eq!(restored.args.fps, 48);
        assert!(!restored.args.cursor);
        assert!(!restored.args.head_tracking);
        assert!(restored.desired_running);
        // an ADB-era file: its adb fields are ignored and its forward address dropped
        let mut old = serde_json::to_value(&saved).unwrap();
        old["args"]["host"] = "127.0.0.1".into();
        old["args"]["adb"] = "adb".into();
        old["args"]["transport"] = "auto".into();
        old["args"].as_object_mut().unwrap().remove("no_launch");
        old["args"]["no_deploy"] = true.into();
        std::fs::write(&path, serde_json::to_vec(&old).unwrap()).unwrap();
        let migrated = load(&path, &schema).unwrap();
        assert_eq!(migrated.args.host, None);
        assert!(migrated.args.no_launch, "no_deploy is no_launch now");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
        std::fs::write(&path, b"not json").unwrap();
        assert!(load(&path, &schema).is_err());
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn dropping_session_requests_stop_and_joins() {
        let control = Arc::new(Control::default());
        let worker_control = control.clone();
        let finished = Arc::new(AtomicBool::new(false));
        let worker_finished = finished.clone();
        let thread = thread::spawn(move || {
            while !worker_control.stop.load(Ordering::Relaxed) {
                thread::yield_now();
            }
            worker_finished.store(true, Ordering::Relaxed);
        });
        drop(Session {
            id: 1,
            args: Args::default(),
            live_target: Args::default(),
            control,
            thread: Some(thread),
        });
        assert!(finished.load(Ordering::Relaxed));
    }
}
