//! Wire protocol of eyepiece's PC Display mod (all little-endian). Constants, message types
//! and struct layouts come from eyepiece `mods/pcdisplay/protocol.json` through
//! `proto_gen.rs` (generated there, copied here; eyepiece's CI compares the two).
//!
//! Every message: an `ep_hdr_t` (`'E' 'P' type flags u32 len`) + payload.

include!("proto_gen.rs");

pub fn header(mtype: u8, flags: u8, len: u32) -> [u8; ep_hdr_t::SIZE] {
    let mut h = [0u8; ep_hdr_t::SIZE];
    h[ep_hdr_t::MAGIC0] = EP_MAGIC[0];
    h[ep_hdr_t::MAGIC1] = EP_MAGIC[1];
    h[ep_hdr_t::TYPE] = mtype;
    h[ep_hdr_t::FLAGS] = flags;
    h[ep_hdr_t::LEN..ep_hdr_t::LEN + 4].copy_from_slice(&len.to_le_bytes());
    h
}

/// Parsed header: (type, flags, len). None = bad magic.
pub fn parse_header(h: &[u8; ep_hdr_t::SIZE]) -> Option<(u8, u8, u32)> {
    if h[ep_hdr_t::MAGIC0] != EP_MAGIC[0] || h[ep_hdr_t::MAGIC1] != EP_MAGIC[1] {
        return None;
    }
    Some((h[ep_hdr_t::TYPE], h[ep_hdr_t::FLAGS], u32le(h, ep_hdr_t::LEN)))
}

fn u16le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64le(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}
fn f32le(b: &[u8], o: usize) -> f32 {
    f32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// ep_video_t: PC capture time (PC monotonic clock) and codec (the bridge picks its decoder
/// from it); the access unit follows.
pub fn video_prefix(pts_us: u64, codec: u8) -> [u8; ep_video_t::SIZE] {
    let mut b = [0u8; ep_video_t::SIZE];
    b[ep_video_t::PTS_US..ep_video_t::PTS_US + 8].copy_from_slice(&pts_us.to_le_bytes());
    b[ep_video_t::CODEC] = codec;
    b
}

/// ep_stream_t: must precede the video (the hardware decoder sizes its buffers from it).
/// Sent once per connection.
pub fn stream_payload(width: u16, height: u16, codec: u8, fps: u32, layout: u8) -> [u8; ep_stream_t::SIZE] {
    let mut b = [0u8; ep_stream_t::SIZE];
    b[ep_stream_t::WIDTH..ep_stream_t::WIDTH + 2].copy_from_slice(&width.to_le_bytes());
    b[ep_stream_t::HEIGHT..ep_stream_t::HEIGHT + 2].copy_from_slice(&height.to_le_bytes());
    b[ep_stream_t::CODEC] = codec;
    b[ep_stream_t::FPS] = fps.min(255) as u8;
    b[ep_stream_t::LAYOUT] = layout;
    b
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
impl Pose {
    pub fn parse(p: &[u8]) -> Option<Pose> {
        use ep_pose_t as o;
        (p.len() >= o::SIZE).then(|| Pose {
            t_dev_us: u64le(p, o::T_DEV_US),
            yaw: f32le(p, o::YAW_DEG),
            pitch: f32le(p, o::PITCH_DEG),
            roll: f32le(p, o::ROLL_DEG),
            active: p[o::ACTIVE],
            source: p[o::SOURCE],
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
impl FrameReport {
    pub fn parse(p: &[u8]) -> Option<FrameReport> {
        use ep_frame_t as o;
        (p.len() >= o::SIZE).then(|| FrameReport {
            pts_us: u64le(p, o::PTS_US),
            t_recv_us: u64le(p, o::T_RECV_US),
            t_shown_us: u64le(p, o::T_SHOWN_US),
            decode_us: u32le(p, o::DECODE_US),
            convert_us: u32le(p, o::CONVERT_US),
            queue_depth: u32le(p, o::QUEUE_DEPTH),
            dropped: u32le(p, o::DROPPED),
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Info {
    pub version: u32,
    pub fb_w: u16,
    pub fb_h: u16,
    pub decoder: u8,
    pub headtrack: u8,
    /// EP_INFO_* capabilities (0 from older bridges)
    pub flags: u8,
}
impl Info {
    pub fn parse(p: &[u8]) -> Option<Info> {
        use ep_info_t as o;
        (p.len() >= o::SIZE).then(|| Info {
            version: u32le(p, o::VERSION),
            fb_w: u16le(p, o::FB_W),
            fb_h: u16le(p, o::FB_H),
            decoder: p[o::DECODER],
            headtrack: p[o::HEADTRACK],
            flags: p[o::FLAGS],
        })
    }
    pub fn decoder_name(&self) -> &'static str {
        match self.decoder {
            EP_DECODER_SOFTWARE => "software",
            EP_DECODER_HARDWARE => "hardware",
            _ => "none",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn header_roundtrip() {
        let h = header(EP_VIDEO, EP_FLAG_KEY, 1234);
        assert_eq!(parse_header(&h), Some((EP_VIDEO, EP_FLAG_KEY, 1234)));
        assert_eq!(&h[..2], b"EP");
    }
    /// Payloads as eyepiece's bridge (host build, null decoder) sent them.
    #[test]
    fn parse_bridge_payloads() {
        let hex = |s: &str| (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect::<Vec<_>>();
        let i = Info::parse(&hex("010000008007380401000000")).unwrap();
        assert_eq!((i.version, i.fb_w, i.fb_h, i.decoder_name(), i.flags), (EP_VERSION, 1920, 1080, "software", 0));
        let f = FrameReport::parse(&hex(
            "09030000000000006d06d30c2a0000008106d30c2a00000000000000000000000000000000000000",
        ))
        .unwrap();
        assert_eq!((f.pts_us, f.t_shown_us - f.t_recv_us, f.dropped), (777, 20, 0));
        assert!(Info::parse(&[0u8; ep_info_t::SIZE - 1]).is_none());
    }
    #[test]
    fn stream_layout() {
        let s = stream_payload(1920, 1080, EP_CODEC_H264, 100, EP_LAYOUT_MONO);
        assert_eq!(s, [0x80, 0x07, 0x38, 0x04, 0, 100, 0, 0]);
    }
}
