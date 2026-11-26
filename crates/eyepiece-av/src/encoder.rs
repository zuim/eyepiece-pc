//! libavcodec encoder with low-latency defaults per encoder family.
//!
//! Rate control gets the real frame rate (time_base 1/fps, framerate fps): ffmpeg's
//! kmsgrab declared 1,000,000 fps and starved every encoder to ~0.3 Mbit/s.

use crate::annexb;
use crate::convert::Target;
use crate::ffi::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    H264,
    Hevc,
}
impl std::str::FromStr for Codec {
    type Err = String;
    fn from_str(s: &str) -> Result<Codec, String> {
        match s {
            "h264" | "avc" => Ok(Codec::H264),
            "hevc" | "h265" => Ok(Codec::Hevc),
            _ => Err(format!("unknown codec {s} (h264|hevc)")),
        }
    }
}
impl Codec {
    pub fn wire(&self) -> u8 {
        match self {
            Codec::H264 => eyepiece_link::proto::EP_CODEC_H264,
            Codec::Hevc => eyepiece_link::proto::EP_CODEC_HEVC,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Nvenc,
    Vaapi,
    Amf,
    Qsv,
    VideoToolbox,
    X264,
    X265,
    Other,
}

#[derive(Debug, Clone)]
pub struct EncoderConfig {
    pub codec: Codec,
    /// auto | nvenc | vaapi | amf | qsv | videotoolbox | x264 | any FFmpeg encoder name
    pub encoder: String,
    pub fps: u32,
    pub bitrate: u64,
    /// keyframe interval in frames
    pub gop: u32,
    /// VBV size in frames of bitrate
    pub vbv_frames: f64,
    pub slices: u32,
    pub preset: Option<String>,
    pub tune: Option<String>,
    pub profile: Option<String>,
    pub intra_refresh: bool,
    /// extra AVOptions (override the defaults)
    pub opts: Vec<(String, String)>,
    pub vaapi_device: String,
}

/// Map aliases to an FFmpeg encoder name.
pub fn encoder_name(alias: &str, codec: Codec) -> String {
    let h = codec == Codec::H264;
    match alias {
        "nvenc" => if h { "h264_nvenc" } else { "hevc_nvenc" }.into(),
        "vaapi" => if h { "h264_vaapi" } else { "hevc_vaapi" }.into(),
        "amf" => if h { "h264_amf" } else { "hevc_amf" }.into(),
        "qsv" => if h { "h264_qsv" } else { "hevc_qsv" }.into(),
        "videotoolbox" | "vt" => if h {
            "h264_videotoolbox"
        } else {
            "hevc_videotoolbox"
        }
        .into(),
        "x264" => "libx264".into(),
        "x265" => "libx265".into(),
        "sw" | "software" => if h { "libx264" } else { "libx265" }.into(),
        other => other.into(),
    }
}

pub fn family(name: &str) -> Family {
    if name.ends_with("_nvenc") {
        Family::Nvenc
    } else if name.ends_with("_vaapi") {
        Family::Vaapi
    } else if name.ends_with("_amf") {
        Family::Amf
    } else if name.ends_with("_qsv") {
        Family::Qsv
    } else if name.ends_with("_videotoolbox") {
        Family::VideoToolbox
    } else if name == "libx264" {
        Family::X264
    } else if name == "libx265" {
        Family::X265
    } else {
        Family::Other
    }
}

pub fn target_for(f: Family) -> Target {
    match f {
        Family::Vaapi => Target::Vaapi,
        Family::VideoToolbox => Target::VideoToolbox,
        Family::X265 => Target::Yuv420p,
        _ => Target::Nv12,
    }
}

/// True if an encoder with this name exists in the linked libavcodec.
pub fn exists(name: &str) -> bool {
    !unsafe { avcodec_find_encoder_by_name(cstr(name).as_ptr()) }.is_null()
}

/// Can this encoder actually open here (driver present)? Opens a tiny session.
pub fn probe(name: &str, vaapi_device: &str) -> bool {
    if !exists(name) {
        return false;
    }
    match family(name) {
        Family::Vaapi => {
            crate::hw::hwdevice(AVHWDeviceType::AV_HWDEVICE_TYPE_VAAPI, Some(vaapi_device)).is_ok()
        }
        _ => unsafe {
            let c = avcodec_find_encoder_by_name(cstr(name).as_ptr());
            let ctx = avcodec_alloc_context3(c);
            if ctx.is_null() {
                return false;
            }
            (*ctx).width = 640;
            (*ctx).height = 480;
            (*ctx).pix_fmt = if family(name) == Family::X265 {
                AVPixelFormat::AV_PIX_FMT_YUV420P
            } else {
                AVPixelFormat::AV_PIX_FMT_NV12
            };
            (*ctx).time_base = AVRational { num: 1, den: 60 };
            (*ctx).framerate = AVRational { num: 60, den: 1 };
            let prev = av_log_get_level();
            av_log_set_level(AV_LOG_QUIET as i32);
            let mut opts = Dict::new();
            if family(name) == Family::VideoToolbox {
                opts.set("allow_sw", "0");
                opts.set("realtime", "1");
            }
            let ok = avcodec_open2(ctx, c, &mut opts.0) >= 0;
            av_log_set_level(prev);
            let mut ctx = ctx;
            avcodec_free_context(&mut ctx);
            ok
        },
    }
}

#[cfg(target_os = "macos")]
const AUTO_ENCODERS: &[&str] = &["videotoolbox", "software"];
#[cfg(not(target_os = "macos"))]
const AUTO_ENCODERS: &[&str] = &["nvenc", "vaapi", "amf", "qsv", "software"];

/// "auto": VideoToolbox on macOS; NVENC, VAAPI, AMF, QSV elsewhere; then software.
pub fn resolve(alias: &str, codec: Codec, vaapi_device: &str) -> Result<String, String> {
    if alias != "auto" {
        let n = encoder_name(alias, codec);
        let encoder = unsafe { avcodec_find_encoder_by_name(cstr(&n).as_ptr()) };
        if encoder.is_null() {
            return Err(format!("encoder {n} not in this FFmpeg build"));
        }
        let expected = match codec {
            Codec::H264 => AVCodecID::AV_CODEC_ID_H264,
            Codec::Hevc => AVCodecID::AV_CODEC_ID_HEVC,
        };
        if unsafe { (*encoder).id } != expected {
            return Err(format!(
                "encoder {n} does not encode {codec:?}; choose a matching encoder or codec"
            ));
        }
        return Ok(n);
    }
    for a in AUTO_ENCODERS {
        let n = encoder_name(a, codec);
        if probe(&n, vaapi_device) {
            return Ok(n);
        }
    }
    Err("no working encoder found".into())
}

pub struct Packet {
    /// Annex-B access unit (parameter sets included on keyframes)
    pub data: Vec<u8>,
    pub key: bool,
    pub pts: i64,
}

pub struct Encoder {
    ctx: *mut AVCodecContext,
    pkt: *mut AVPacket,
    pub name: String,
    pub family: Family,
    cfg: EncoderConfig,
    param_sets: Vec<u8>,
    pub description: String,
}
unsafe impl Send for Encoder {}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            av_packet_free(&mut self.pkt);
            avcodec_free_context(&mut self.ctx);
        }
    }
}

fn default_opts(f: Family, c: &EncoderConfig) -> Vec<(String, String)> {
    let h264 = c.codec == Codec::H264;
    let mut o: Vec<(&str, String)> = vec![];
    match f {
        Family::Nvenc => {
            o.push(("preset", "p1".into()));
            o.push(("tune", "ull".into()));
            o.push(("zerolatency", "1".into()));
            o.push(("delay", "0".into()));
            o.push(("rc", "cbr".into()));
            o.push(("forced-idr", "1".into()));
            if h264 {
                // the goggles decode in hardware: high/CABAC (+1.3 dB vs baseline/CAVLC)
                o.push(("profile", "high".into()));
                o.push(("coder", "cabac".into()));
            }
            if c.intra_refresh {
                o.push(("intra-refresh", "1".into()));
            }
        }
        Family::Vaapi => {
            o.push(("rc_mode", "CBR".into()));
            o.push(("async_depth", "1".into())); // default 2 queues a frame: +15 ms measured
            if h264 {
                o.push(("profile", "high".into()));
                o.push(("coder", "cabac".into()));
            } else {
                o.push(("profile", "main".into()));
            }
        }
        Family::Amf => {
            o.push(("usage", "ultralowlatency".into()));
            o.push(("rc", "cbr".into()));
            o.push(("quality", "speed".into()));
            o.push(("forced_idr", "1".into()));
            if h264 {
                o.push(("profile", "high".into()));
                o.push(("coder", "cabac".into()));
            }
        }
        Family::Qsv => {
            o.push(("preset", "veryfast".into()));
            o.push(("async_depth", "1".into()));
            o.push(("look_ahead", "0".into()));
            o.push(("forced_idr", "1".into()));
            if h264 {
                o.push(("profile", "high".into()));
            }
        }
        Family::VideoToolbox => {
            o.push(("allow_sw", "0".into()));
            o.push(("realtime", "1".into()));
            o.push(("prio_speed", "1".into()));
            if h264 {
                o.push(("profile", "high".into()));
            } else {
                // VT HEVC otherwise derives the DPB from the level (up to 6 refs), which exhausts
                // the goggles' VC8000D frame buffers and stalls the decoder. Not for H.264:
                // there VT then emits near-intra frames.
                o.push(("max_ref_frames", "1".into()));
            }
        }
        Family::X264 => {
            // superfast = fastest preset that keeps CABAC; zerolatency = sliced threads,
            // no lookahead, no B-frames
            o.push(("preset", "superfast".into()));
            o.push(("tune", "zerolatency".into()));
            o.push(("profile", "high".into()));
            o.push(("forced-idr", "1".into()));
            let mut p = "scenecut=0:repeat-headers=1:aud=0".to_string();
            if c.intra_refresh {
                p += ":intra-refresh=1";
            }
            o.push(("x264-params", p));
        }
        Family::X265 => {
            o.push(("preset", "ultrafast".into()));
            o.push(("tune", "zerolatency".into()));
            o.push(("forced-idr", "1".into()));
            let mut p = format!("bframes=0:scenecut=0:repeat-headers=1:slices={}", c.slices);
            if c.intra_refresh {
                p += ":intra-refresh=1";
            }
            o.push(("x265-params", p));
        }
        Family::Other => {}
    }
    let mut v: Vec<(String, String)> = o.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    let mut set = |k: &str, val: &Option<String>| {
        if let Some(x) = val {
            v.retain(|(kk, _)| kk != k);
            v.push((k.into(), x.clone()));
        }
    };
    set("preset", &c.preset);
    set("tune", &c.tune);
    set("profile", &c.profile);
    for (k, val) in &c.opts {
        v.retain(|(kk, _)| kk != k);
        v.push((k.clone(), val.clone()));
    }
    v
}

impl Encoder {
    /// Open for frames like `first` (format/size); `hw_frames`: the converter's output
    /// frames context for VAAPI or VideoToolbox.
    ///
    /// # Safety
    /// `hw_frames` must be null or a valid `AVBufferRef` for the duration of the call
    /// (the encoder takes its own reference).
    pub unsafe fn open(
        cfg: &EncoderConfig,
        name: &str,
        first: &Frame,
        hw_frames: *mut AVBufferRef,
    ) -> Result<Encoder, String> {
        let fam = family(name);
        unsafe {
            let codec = avcodec_find_encoder_by_name(cstr(name).as_ptr());
            if codec.is_null() {
                return Err(format!("encoder {name} not found"));
            }
            let ctx = avcodec_alloc_context3(codec);
            let c = &mut *ctx;
            c.width = first.width();
            c.height = first.height();
            c.pix_fmt = std::mem::transmute::<i32, AVPixelFormat>(first.format());
            c.sample_aspect_ratio = AVRational { num: 1, den: 1 };
            c.time_base = AVRational {
                num: 1,
                den: cfg.fps as i32,
            };
            c.framerate = AVRational {
                num: cfg.fps as i32,
                den: 1,
            };
            c.bit_rate = cfg.bitrate as i64;
            c.rc_max_rate = cfg.bitrate as i64;
            c.rc_min_rate = if matches!(
                fam,
                Family::Nvenc | Family::Vaapi | Family::Qsv | Family::Amf
            ) {
                cfg.bitrate as i64
            } else {
                0
            };
            c.rc_buffer_size =
                ((cfg.bitrate as f64 * cfg.vbv_frames / cfg.fps as f64) as i64).max(1000) as i32;
            c.gop_size = cfg.gop as i32;
            c.max_b_frames = 0;
            c.slices = cfg.slices as i32;
            c.flags |= AV_CODEC_FLAG_LOW_DELAY as i32;
            c.color_range = AVColorRange::AVCOL_RANGE_MPEG;
            c.colorspace = AVColorSpace::AVCOL_SPC_BT709;
            c.color_primaries = AVColorPrimaries::AVCOL_PRI_BT709;
            c.color_trc = AVColorTransferCharacteristic::AVCOL_TRC_BT709;
            if fam == Family::X264 || fam == Family::X265 || fam == Family::Other {
                c.thread_count = 0; // auto (x264 uses sliced threads with zerolatency)
            }
            if !hw_frames.is_null() {
                c.hw_frames_ctx = av_buffer_ref(hw_frames);
            }
            let opts = default_opts(fam, cfg);
            let mut d = Dict::new();
            for (k, v) in &opts {
                d.set(k, v);
            }
            let r = avcodec_open2(ctx, codec, &mut d.0);
            if r < 0 {
                let mut ctx = ctx;
                avcodec_free_context(&mut ctx);
                return Err(format!("open {name}: {}", err_str(r)));
            }
            let unused = d.keys();
            if !unused.is_empty() {
                eprintln!(
                    "[eyepiece-pc] {name}: options not recognised: {}",
                    unused.join(", ")
                );
            }
            let mut param_sets = vec![];
            if c.extradata_size > 0 {
                let ed = std::slice::from_raw_parts(c.extradata, c.extradata_size as usize);
                param_sets = annexb::param_sets_from_extradata(ed, cfg.codec == Codec::Hevc);
            }
            let rate_control = if fam == Family::VideoToolbox {
                "target bitrate (driver-managed VBV/slices)"
            } else {
                "CBR"
            };
            let description = format!(
                "{name} {}x{} {} {} fps, {:.1} Mbit/s {rate_control}, VBV {:.1} frames, GOP {}, {} slices, opts [{}]",
                c.width,
                c.height,
                pix_fmt_name(first.format()),
                cfg.fps,
                cfg.bitrate as f64 / 1e6,
                cfg.vbv_frames,
                cfg.gop,
                cfg.slices,
                opts.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" ")
            );
            Ok(Encoder {
                ctx,
                pkt: av_packet_alloc(),
                name: name.into(),
                family: fam,
                cfg: cfg.clone(),
                param_sets,
                description,
            })
        }
    }

    /// Encode one frame (its pts in 1/fps units). `idr`: force an IDR now.
    pub fn encode(
        &mut self,
        frame: Frame,
        pts: i64,
        idr: bool,
        out: &mut Vec<Packet>,
    ) -> Result<(), String> {
        unsafe {
            let f = &mut *frame.0;
            f.pts = pts;
            if idr {
                f.pict_type = AVPictureType::AV_PICTURE_TYPE_I;
                f.flags |= AV_FRAME_FLAG_KEY as i32;
            } else {
                f.pict_type = AVPictureType::AV_PICTURE_TYPE_NONE;
            }
            check(avcodec_send_frame(self.ctx, frame.0), "avcodec_send_frame")?;
            drop(frame);
            self.drain(out)
        }
    }

    /// Finish encoding and collect delayed packets. No more frames may be submitted.
    pub fn finish(&mut self, out: &mut Vec<Packet>) -> Result<(), String> {
        check(
            unsafe { avcodec_send_frame(self.ctx, std::ptr::null()) },
            "avcodec flush",
        )?;
        self.drain(out)
    }

    fn drain(&mut self, out: &mut Vec<Packet>) -> Result<(), String> {
        unsafe {
            loop {
                let r = avcodec_receive_packet(self.ctx, self.pkt);
                if r == EAGAIN_ || r == AVERROR_EOF_ {
                    return Ok(());
                }
                check(r, "avcodec_receive_packet")?;
                let p = &*self.pkt;
                let data = std::slice::from_raw_parts(p.data, p.size as usize);
                let key = p.flags & AV_PKT_FLAG_KEY as i32 != 0;
                let hevc = self.cfg.codec == Codec::Hevc;
                let mut v = Vec::with_capacity(data.len() + self.param_sets.len());
                if annexb::is_annexb(data) {
                    if key && !annexb::has_param_sets(data, hevc) {
                        v.extend_from_slice(&self.param_sets);
                    }
                    v.extend_from_slice(data);
                } else {
                    // length-prefixed (global-header style) output → Annex-B
                    if key {
                        v.extend_from_slice(&self.param_sets);
                    }
                    annexb::lp_to_annexb(data, 4, &mut v);
                }
                out.push(Packet {
                    data: v,
                    key,
                    pts: p.pts,
                });
                av_packet_unref(self.pkt);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_software_encoders_do_not_change_with_codec() {
        for codec in [Codec::H264, Codec::Hevc] {
            assert_eq!(encoder_name("x264", codec), "libx264");
            assert_eq!(encoder_name("x265", codec), "libx265");
        }
        assert_eq!(encoder_name("software", Codec::H264), "libx264");
        assert_eq!(encoder_name("software", Codec::Hevc), "libx265");
        for (alias, codec) in [
            ("x264", Codec::Hevc),
            ("libx264", Codec::Hevc),
            ("x265", Codec::H264),
            ("hevc_videotoolbox", Codec::H264),
        ] {
            if exists(&encoder_name(alias, codec)) {
                assert!(resolve(alias, codec, "")
                    .unwrap_err()
                    .contains("does not encode"));
            }
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_auto_prefers_hardware_with_codec_appropriate_fallback() {
        assert_eq!(AUTO_ENCODERS, &["videotoolbox", "software"]);
        assert_eq!(
            encoder_name(AUTO_ENCODERS[0], Codec::H264),
            "h264_videotoolbox"
        );
        assert_eq!(
            encoder_name(AUTO_ENCODERS[0], Codec::Hevc),
            "hevc_videotoolbox"
        );
        assert_eq!(encoder_name(AUTO_ENCODERS[1], Codec::H264), "libx264");
        assert_eq!(encoder_name(AUTO_ENCODERS[1], Codec::Hevc), "libx265");
    }

    #[test]
    fn videotoolbox_aliases_and_target() {
        assert_eq!(
            encoder_name("videotoolbox", Codec::H264),
            "h264_videotoolbox"
        );
        assert_eq!(encoder_name("vt", Codec::Hevc), "hevc_videotoolbox");
        assert_eq!(family("h264_videotoolbox"), Family::VideoToolbox);
        assert_eq!(family("hevc_videotoolbox"), Family::VideoToolbox);
        assert_eq!(target_for(Family::VideoToolbox), Target::VideoToolbox);
        assert_eq!(family("h264_nvenc"), Family::Nvenc);
    }

    fn config() -> EncoderConfig {
        EncoderConfig {
            codec: Codec::H264,
            encoder: "videotoolbox".into(),
            fps: 60,
            bitrate: 20_000_000,
            gop: 120,
            vbv_frames: 4.0,
            slices: 4,
            preset: None,
            tune: None,
            profile: None,
            intra_refresh: false,
            opts: vec![],
            vaapi_device: String::new(),
        }
    }

    #[test]
    fn x264_defaults_match_latency_benchmark_reference() {
        let defaults = default_opts(Family::X264, &config());
        assert_eq!(
            defaults,
            [
                ("preset".into(), "superfast".into()),
                ("tune".into(), "zerolatency".into()),
                ("profile".into(), "high".into()),
                ("forced-idr".into(), "1".into()),
                (
                    "x264-params".into(),
                    "scenecut=0:repeat-headers=1:aud=0".into()
                ),
            ]
        );
        assert_eq!(target_for(Family::X264), Target::Nv12);
    }

    #[test]
    fn videotoolbox_defaults_require_hardware_and_allow_overrides() {
        let mut config = config();
        let defaults = default_opts(Family::VideoToolbox, &config);
        for (key, value) in [
            ("allow_sw", "0"),
            ("realtime", "1"),
            ("prio_speed", "1"),
            ("profile", "high"),
        ] {
            assert!(defaults.contains(&(key.into(), value.into())));
        }
        assert!(!defaults
            .iter()
            .any(|(key, _)| key == "preset" || key == "tune" || key == "constant_bit_rate"));
        assert_eq!(defaults.len(), 4);
        config.codec = Codec::Hevc;
        let hevc = default_opts(Family::VideoToolbox, &config);
        assert!(hevc.contains(&("max_ref_frames".into(), "1".into())));
        assert!(!hevc.iter().any(|(key, _)| key == "profile"));
        config.codec = Codec::H264;
        config.opts.push(("allow_sw".into(), "1".into()));
        let options = default_opts(Family::VideoToolbox, &config);
        assert!(options.contains(&("allow_sw".into(), "1".into())));
        assert_eq!(
            options.iter().filter(|(key, _)| key == "allow_sw").count(),
            1
        );
    }
}
