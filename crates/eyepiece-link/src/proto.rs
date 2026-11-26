//! Shared protobuf envelope and validated bridge application types.

pub mod wire {
    include!(concat!(env!("OUT_DIR"), "/eyepiece.rpc.v1.rs"));
}
use prost::Message;
use std::io;

pub const EP_PORT: u16 = 42420;
pub const EP_VERSION: u32 = 4;
pub const EP_MAX_PAYLOAD: u32 = 4 << 20;

pub fn encode_frame(envelope: &wire::Envelope, max_len: usize) -> io::Result<Vec<u8>> {
    if envelope.message.is_none() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "envelope has no message"));
    }
    let len = envelope.encoded_len();
    if len == 0 || len > max_len || len > u32::MAX as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "envelope exceeds size limit"));
    }
    let mut result = Vec::with_capacity(4 + len);
    result.extend_from_slice(&(len as u32).to_be_bytes());
    envelope.encode(&mut result).map_err(io::Error::other)?;
    Ok(result)
}

pub fn decode_envelope(data: &[u8], max_len: usize) -> io::Result<wire::Envelope> {
    if data.is_empty() || data.len() > max_len {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid envelope size"));
    }
    let envelope = wire::Envelope::decode(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    if envelope.message.is_none() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "envelope has no message"));
    }
    Ok(envelope)
}

pub fn bridge_frame(message: wire::envelope::Message) -> io::Result<Vec<u8>> {
    encode_frame(&wire::Envelope { id: None, sender: None, recipient: None, message: Some(message) }, EP_MAX_PAYLOAD as usize)
}

pub const EP_CODEC_H264: u8 = 0;
pub const EP_CODEC_HEVC: u8 = 1;
pub const EP_OP_RECENTER: u8 = 1;
pub const EP_OP_QUIT: u8 = 2;
/// Start/stop the goggles' MCU attitude stream (EP_POSE).
pub const EP_OP_HEADTRACK: u8 = 3;

/// ep_stream_t.layout: mono, or both eyes side by side (2560x720, the goggles' 3D mode).
pub const EP_LAYOUT_MONO: u8 = 0;
pub const EP_LAYOUT_SBS: u8 = 1;
/// One eye of a side-by-side stream (the 3D display timing; no scaler on the goggles).
pub const SBS_EYE_W: u32 = 1280;
pub const SBS_EYE_H: u32 = 720;
/// ep_info_t.flags: the bridge understands EP_LAYOUT_SBS.
pub const EP_INFO_STEREO: u32 = 0x01;
pub const EP_INFO_PANE_NV12: u32 = 0x02;
pub const EP_INFO_PANE_BGRA: u32 = 0x04;
pub const EP_INFO_PANE_OPAQUE_BGRA: u32 = 0x08;
pub const EP_INFO_PANE_RECONNECT: u32 = 0x10;
pub const EP_INFO_RECONFIG: u32 = 0x20;
pub const EP_INFO_GUI_ALPHA: u32 = 0x40;
pub const EP_INFO_GUI_ALPHA_ACTIVE: u32 = 0x80;
pub const EP_INFO_PANE_ALPHA: u32 = 0x100;

/// Output pane format. Bgra packs color and grayscale alpha side by side;
/// OpaqueBgra uses normal-width color video. Both use limited-range BT.709.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaneFormat {
    Nv12,
    Bgra,
    OpaqueBgra,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pane {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub format: PaneFormat,
}

impl Pane {
    /// Validate against the advertised panel and capabilities before sending.
    pub fn payload(self, info: Info) -> Result<wire::Pane, &'static str> {
        let capability = match self.format {
            PaneFormat::Nv12 => EP_INFO_PANE_NV12,
            PaneFormat::Bgra => EP_INFO_PANE_BGRA,
            PaneFormat::OpaqueBgra => EP_INFO_PANE_OPAQUE_BGRA,
        };
        if info.flags & capability == 0 {
            return Err("bridge does not support the requested pane format");
        }
        if self.width == 0 || self.height == 0 || self.width % 2 != 0 || self.height % 2 != 0
            || u32::from(self.x) + u32::from(self.width) > u32::from(info.fb_w)
            || u32::from(self.y) + u32::from(self.height) > u32::from(info.fb_h)
        {
            return Err("pane must have nonzero even dimensions and fit within the panel");
        }
        Ok(wire::Pane {
            x: self.x.into(), y: self.y.into(), width: self.width.into(), height: self.height.into(),
            format: match self.format { PaneFormat::Nv12 => 0, PaneFormat::Bgra => 1, PaneFormat::OpaqueBgra => 2 },
        })
    }

    pub fn encoded_size(self) -> Result<(u16, u16), &'static str> {
        let width = match self.format {
            PaneFormat::Nv12 => self.width,
            PaneFormat::Bgra => self.width.checked_mul(2).ok_or("packed pane width overflows")?,
            PaneFormat::OpaqueBgra => self.width,
        };
        Ok((width, self.height))
    }
}

/// Everything the bridge needs to receive video: the codec, layout and an optional GFX1 pane.
/// It is the single stream description; the encoded frame size follows from it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamConfig {
    pub codec: u8,
    pub layout: u8,
    pub pane: Option<Pane>,
}

impl StreamConfig {
    /// Encoded frame size: the pane (twice as wide when packed with alpha), else the panel
    /// for mono video or both eyes side by side for stereo.
    pub fn frame_size(self, panel: (u16, u16)) -> Result<(u16, u16), &'static str> {
        match (self.pane, self.layout) {
            (Some(_), EP_LAYOUT_SBS) => Err("stereo cannot be combined with a pane"),
            (Some(pane), _) => pane.encoded_size(),
            (None, EP_LAYOUT_SBS) => Ok(((2 * SBS_EYE_W) as u16, SBS_EYE_H as u16)),
            (None, EP_LAYOUT_MONO) => Ok(panel),
            _ => Err("unknown stream layout"),
        }
    }

    /// Validate against the advertised panel and capabilities before sending.
    pub fn payload(self, info: Info) -> Result<wire::Configure, &'static str> {
        if self.codec > EP_CODEC_HEVC {
            return Err("unknown codec");
        }
        if self.layout == EP_LAYOUT_SBS && info.flags & EP_INFO_STEREO == 0 {
            return Err("bridge does not support stereo");
        }
        self.frame_size((info.fb_w, info.fb_h))?;
        let pane = self.pane.map(|pane| pane.payload(info)).transpose()?;
        Ok(wire::Configure { codec: self.codec.into(), layout: self.layout.into(), pane })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pose {
    pub t_dev_us: u64,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub active: u8,
    pub source: u8,
}
impl TryFrom<wire::Pose> for Pose {
    type Error = &'static str;
    fn try_from(p: wire::Pose) -> Result<Self, Self::Error> {
        Ok(Self {
            t_dev_us: p.device_us, yaw: p.yaw, pitch: p.pitch, roll: p.roll,
            active: p.active.into(), source: p.source.try_into().map_err(|_| "invalid pose source")?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FrameReport {
    pub pts_us: u64,
    pub t_recv_us: u64,
    pub t_shown_us: u64,
    pub decode_us: u32,
    pub convert_us: u32,
    pub queue_depth: u32,
    pub dropped: u32,
}
impl From<wire::Frame> for FrameReport {
    fn from(p: wire::Frame) -> Self {
        Self { pts_us: p.pts_us, t_recv_us: p.received_us, t_shown_us: p.shown_us,
               decode_us: p.decode_us, convert_us: p.convert_us, queue_depth: p.queue_depth, dropped: p.dropped }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Info {
    pub version: u32,
    pub fb_w: u16,
    pub fb_h: u16,
    pub decoder: u8,
    pub headtrack: u8,
    /// EP_INFO_* capabilities
    pub flags: u32,
}
impl Info {
    pub fn decoder_name(&self) -> &'static str {
        match self.decoder {
            1 => "software",
            2 => "hardware",
            _ => "none",
        }
    }
}
impl TryFrom<wire::Info> for Info {
    type Error = &'static str;
    fn try_from(p: wire::Info) -> Result<Self, Self::Error> {
        Ok(Self {
            version: p.version,
            fb_w: p.framebuffer_width.try_into().map_err(|_| "invalid framebuffer width")?,
            fb_h: p.framebuffer_height.try_into().map_err(|_| "invalid framebuffer height")?,
            decoder: p.decoder.try_into().map_err(|_| "invalid decoder")?,
            headtrack: p.headtrack.try_into().map_err(|_| "invalid headtrack flags")?,
            flags: p.flags,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn envelope_framing_validates_message_and_size() {
        let empty = wire::Envelope { id: Some("1".into()), ..Default::default() };
        assert!(encode_frame(&empty, 1024).is_err());
        assert!(decode_envelope(&empty.encode_to_vec(), 1024).is_err());
        let envelope = wire::Envelope {
            id: Some("42".into()), sender: Some("pc".into()), recipient: Some("bridge".into()),
            message: Some(wire::envelope::Message::Ping(wire::Ping { pc_us: 123 })),
        };
        let length = envelope.encoded_len();
        let frame = encode_frame(&envelope, length).unwrap();
        assert_eq!(u32::from_be_bytes(frame[..4].try_into().unwrap()) as usize, length);
        assert_eq!(decode_envelope(&frame[4..], length).unwrap(), envelope);
        assert!(encode_frame(&envelope, length - 1).is_err());
        assert!(decode_envelope(&frame[4..], length - 1).is_err());
    }

    #[test]
    fn pane_payload_and_capabilities() {
        let info = Info { fb_w: 1920, fb_h: 1080, flags: EP_INFO_PANE_BGRA, ..Info::default() };
        let pane = Pane { x: 100, y: 50, width: 640, height: 480, format: PaneFormat::Bgra };
        assert_eq!(pane.payload(info).unwrap(), wire::Pane { x: 100, y: 50, width: 640, height: 480, format: 1 });
        assert_eq!(pane.encoded_size(), Ok((1280, 480)));
        assert!(pane.payload(Info { flags: 0, ..info }).is_err());
        assert!(Pane { x: 1300, ..pane }.payload(info).is_err());
        assert!(Pane { width: 639, ..pane }.payload(info).is_err());
        assert!(Pane { width: 0, ..pane }.payload(info).is_err());
        assert!(Pane { width: 40000, ..pane }.encoded_size().is_err());
        let fullscreen = Pane { x: 0, y: 0, width: 1920, height: 1080, format: PaneFormat::Bgra };
        assert!(fullscreen.payload(info).is_ok());
        assert_eq!(fullscreen.encoded_size(), Ok((3840, 1080)));
        let opaque = Pane { format: PaneFormat::OpaqueBgra, ..fullscreen };
        assert!(opaque.payload(info).is_err());
        assert_eq!(opaque.payload(Info { flags: EP_INFO_PANE_OPAQUE_BGRA, ..info }).unwrap().format, 2);
        assert_eq!(opaque.encoded_size(), Ok((1920, 1080)));
    }

    #[test]
    fn stream_config_derives_frame_size_and_validates() {
        let info = Info {
            fb_w: 1920, fb_h: 1080,
            flags: EP_INFO_PANE_BGRA | EP_INFO_STEREO,
            ..Info::default()
        };
        let pane = Pane { x: 100, y: 50, width: 640, height: 480, format: PaneFormat::Bgra };
        let config = StreamConfig { codec: EP_CODEC_HEVC, layout: EP_LAYOUT_MONO, pane: Some(pane) };
        let payload = config.payload(info).unwrap();
        assert_eq!(payload.pane, Some(pane.payload(info).unwrap()));
        assert_eq!((payload.codec, payload.layout), (1, 0));
        assert_eq!(config.frame_size((1920, 1080)), Ok((1280, 480)));

        let panel = StreamConfig { codec: EP_CODEC_H264, layout: EP_LAYOUT_MONO, pane: None };
        assert_eq!(panel.payload(info).unwrap().pane, None);
        assert_eq!(panel.frame_size((1920, 1080)), Ok((1920, 1080)));
        let stereo = StreamConfig { layout: EP_LAYOUT_SBS, ..panel };
        assert_eq!(stereo.frame_size((1920, 1080)), Ok((2560, 720)));
        assert!(stereo.payload(info).is_ok());
        assert!(stereo.payload(Info { flags: EP_INFO_PANE_BGRA, ..info }).is_err());
        assert!(StreamConfig { pane: Some(pane), ..stereo }.payload(info).is_err());
        assert!(StreamConfig { codec: 2, ..panel }.payload(info).is_err());
        assert!(StreamConfig { layout: 2, ..panel }.payload(info).is_err());
        assert!(StreamConfig { pane: Some(Pane { x: 1300, ..pane }), ..config }.payload(info).is_err());
        assert!(config.payload(Info { flags: EP_INFO_STEREO, ..info }).is_err());
    }
}
