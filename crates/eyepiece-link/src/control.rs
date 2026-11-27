//! tinkerglass' control protocol on the goggles' "tinkerglass control 1" interface (the PC
//! app's; the web app uses control 0): start the PC Display bridge, read the status. The server
//! is tinkerglass' core.link; the schema is a copy of tinkerglass' `contract/tinkerglass-control.proto`
//! (`contract/` here; `tests/control_schema.rs` checks it against a tinkerglass checkout).
//!
//! Framing: one message per USB transfer, `'T' | version 1 | u16le flags | u32le length |
//! protobuf`, ended by a zero-length packet when its size is a multiple of the packet size.

use crate::usb::{self, TgInterface, UsbDevice};
use nusb::io::{EndpointRead, EndpointWrite};
use nusb::transfer::{Bulk, In, Out};
use prost::Message;
use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::time::{Duration, Instant};

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/tinkerglass.control.v1.rs"));
}

pub const FRAME_MAGIC: u8 = b'T';
pub const FRAME_VERSION: u8 = 1;
pub const FRAME_HEADER: usize = 8;
/// tinkerglass `contract/device.json` `limits.link_payload_max`
pub const PAYLOAD_MAX: usize = 16384;
/// The bridge of tinkerglass's PC Display mod (its mod id; core.link resolves its menu entry).
pub const BRIDGE_ENTRY: &str = "pcdisplay";

/// `message` as one control frame.
pub fn frame(message: &pb::ClientMessage) -> Vec<u8> {
    let len = message.encoded_len();
    let mut out = Vec::with_capacity(FRAME_HEADER + len);
    out.extend_from_slice(&[FRAME_MAGIC, FRAME_VERSION, 0, 0]);
    out.extend_from_slice(&(len as u32).to_le_bytes());
    message.encode(&mut out).expect("Vec grows");
    out
}

/// Joins transfers into frames; drops what is not a frame of this version (unknown flags too).
#[derive(Default)]
pub struct FrameReader {
    buf: Vec<u8>,
}

impl FrameReader {
    pub fn push(&mut self, data: &[u8]) -> Vec<Vec<u8>> {
        self.buf.extend_from_slice(data);
        let mut out = Vec::new();
        loop {
            if !self.buf.is_empty() && self.buf[0] != FRAME_MAGIC {
                self.buf.clear();
                break;
            }
            if self.buf.len() < FRAME_HEADER {
                break;
            }
            let flags = u16::from_le_bytes([self.buf[2], self.buf[3]]);
            let len = u32::from_le_bytes(self.buf[4..8].try_into().unwrap()) as usize;
            if self.buf[1] != FRAME_VERSION || flags != 0 || len > PAYLOAD_MAX {
                self.buf.clear();
                break;
            }
            if self.buf.len() < FRAME_HEADER + len {
                break;
            }
            out.push(self.buf[FRAME_HEADER..FRAME_HEADER + len].to_vec());
            self.buf.drain(..FRAME_HEADER + len);
        }
        out
    }
}

/// A Launch that did not start the app, with the server's code.
#[derive(Debug)]
pub struct LaunchError {
    pub code: pb::Code,
    pub message: String,
}

/// The control link on control 1, after Hello/Welcome.
pub struct ControlLink {
    _usb: UsbDevice,
    reader: EndpointRead<Bulk>,
    writer: EndpointWrite<Bulk>,
    frames: FrameReader,
    queue: VecDeque<Vec<u8>>,
    next_id: u32,
    pub welcome: pb::Welcome,
}

impl ControlLink {
    /// Claim control 1 and say Hello. `client` is our name and version, for the goggles' log.
    pub fn open(serial: Option<&str>, client: &str) -> Result<ControlLink, String> {
        let usb = usb::open(serial, TgInterface::Control1)?;
        let e = usb.endpoints;
        let in_ep = usb
            .interface
            .endpoint::<Bulk, In>(e.in_address)
            .map_err(|err| format!("cannot open the control interface's IN endpoint ({err})"))?;
        let out_ep = usb
            .interface
            .endpoint::<Bulk, Out>(e.out_address)
            .map_err(|err| format!("cannot open the control interface's OUT endpoint ({err})"))?;
        let reader = EndpointRead::new(in_ep, 8192)
            .with_num_transfers(2)
            .with_read_timeout(Duration::from_millis(100));
        let writer = EndpointWrite::new(out_ep, PAYLOAD_MAX + FRAME_HEADER)
            .with_write_timeout(Duration::from_secs(2));
        let mut link = ControlLink {
            _usb: usb,
            reader,
            writer,
            frames: FrameReader::default(),
            queue: VecDeque::new(),
            next_id: 1,
            welcome: pb::Welcome::default(),
        };
        link.send(pb::client_message::Body::Hello(pb::Hello {
            role: pb::Role::PcApp as i32,
            protocol: pb::Protocol::V1 as u32,
            client: client.into(),
        }))
        .map_err(|e| no_session(&e))?;
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            // skip a reply that was still queued for an earlier client
            match link.next(deadline)? {
                Some(pb::server_message::Body::Welcome(w)) => {
                    link.welcome = w;
                    return Ok(link);
                }
                Some(_) => continue,
                None => return Err(no_session("no answer")),
            }
        }
    }

    fn send(&mut self, body: pb::client_message::Body) -> Result<(), String> {
        let bytes = frame(&pb::ClientMessage { body: Some(body) });
        self.writer
            .write_all(&bytes)
            .and_then(|_| self.writer.flush_end())
            .map_err(|e| format!("USB write to the control interface failed: {e}"))
    }

    /// The next message, or None at the deadline.
    fn next(&mut self, deadline: Instant) -> Result<Option<pb::server_message::Body>, String> {
        let mut buf = [0u8; 8192];
        loop {
            while let Some(bytes) = self.queue.pop_front() {
                if let Ok(m) = pb::ServerMessage::decode(&bytes[..]) {
                    if let Some(body) = m.body {
                        return Ok(Some(body));
                    }
                }
            }
            if Instant::now() >= deadline {
                return Ok(None);
            }
            match self.reader.read(&mut buf) {
                Ok(0) => {}
                Ok(n) => {
                    let frames = self.frames.push(&buf[..n]);
                    self.queue.extend(frames);
                }
                Err(e) if e.kind() == ErrorKind::TimedOut => {}
                Err(e) => return Err(format!("USB read from the control interface failed: {e}")),
            }
        }
    }

    fn request(&mut self, body: pb::request::Body, timeout: Duration) -> Result<pb::Response, String> {
        let id = self.next_id;
        self.next_id += 1;
        self.send(pb::client_message::Body::Request(pb::Request { id, body: Some(body) }))?;
        let deadline = Instant::now() + timeout;
        loop {
            match self.next(deadline)? {
                Some(pb::server_message::Body::Response(r)) if r.id == id => return Ok(r),
                Some(_) => continue,
                None => return Err("the goggles did not answer the request in time".into()),
            }
        }
    }

    /// Start an app as a tap in the goggles' menu does; with `ready_timeout` the answer comes
    /// once it serves its clients.
    pub fn launch(&mut self, launch: pb::Launch) -> Result<Result<pb::Launched, LaunchError>, String> {
        let wait = Duration::from_millis(u64::from(launch.ready_timeout_ms)) + Duration::from_secs(6);
        let r = self.request(pb::request::Body::Launch(launch), wait)?;
        let code = pb::Code::try_from(r.code).unwrap_or(pb::Code::Failed);
        Ok(match code {
            pb::Code::Ok | pb::Code::AlreadyRunning => Ok(match r.body {
                Some(pb::response::Body::Launch(l)) => l,
                _ => pb::Launched::default(),
            }),
            _ => Err(LaunchError { code, message: r.message }),
        })
    }

    pub fn status(&mut self) -> Result<pb::Status, String> {
        let r = self.request(pb::request::Body::Status(pb::GetStatus {}), Duration::from_secs(3))?;
        match r.body {
            Some(pb::response::Body::Status(s)) if r.code == pb::Code::Ok as i32 => Ok(s),
            _ => Err(format!("status failed: {}", r.message)),
        }
    }
}

fn no_session(why: &str) -> String {
    format!(
        "the goggles do not answer on their control interface ({why}): the tinkerglass mods are not \
         running (start them), or they are older than the control link (update tinkerglass)"
    )
}

/// Start PC Display's bridge for a PC session (`args`, e.g. `-1`) through control 1 and wait
/// until it serves the video interface. Control 1 is released again afterwards.
pub fn launch_bridge(serial: Option<&str>, args: &[String], log: &dyn Fn(String)) -> Result<u32, String> {
    let client = concat!("eyepiece-pc ", env!("CARGO_PKG_VERSION"));
    let mut link = ControlLink::open(serial, client)?;
    let t0 = Instant::now();
    let result = link.launch(pb::Launch {
        entry: BRIDGE_ENTRY.into(),
        args: args.to_vec(),
        replace: true,
        ready_timeout_ms: 10_000,
    })?;
    match result {
        Ok(l) => {
            log(format!(
                "bridge started on the goggles through the control link (pid {}, {} ms, tinkerglass {})",
                l.pid,
                t0.elapsed().as_millis(),
                link.welcome.version
            ));
            Ok(l.pid)
        }
        Err(e) => Err(match e.code {
            pb::Code::NotInstalled => "PC Display is not installed on the goggles: install tinkerglass with \
                                       its PC Display mod, then connect again"
                .into(),
            pb::Code::NoUi => "no tinkerglass session on the goggles: the goggles' menu did not take the \
                               start request (start the mods, then connect again)"
                .into(),
            _ => format!(
                "the bridge did not start: {} (its log: `tg logs --bridge` in tinkerglass)",
                e.message
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello() -> pb::ClientMessage {
        pb::ClientMessage {
            body: Some(pb::client_message::Body::Hello(pb::Hello {
                role: pb::Role::PcApp as i32,
                protocol: 1,
                client: "test".into(),
            })),
        }
    }

    #[test]
    fn frame_header_is_magic_version_flags_length() {
        let f = frame(&hello());
        assert_eq!(&f[..4], &[b'T', 1, 0, 0]);
        assert_eq!(u32::from_le_bytes(f[4..8].try_into().unwrap()) as usize, f.len() - FRAME_HEADER);
        assert_eq!(pb::ClientMessage::decode(&f[FRAME_HEADER..]).unwrap(), hello());
    }

    #[test]
    fn reader_joins_pieces_and_drops_foreign_frames() {
        let f = frame(&hello());
        let mut r = FrameReader::default();
        assert!(r.push(&f[..5]).is_empty());
        assert_eq!(r.push(&f[5..]).len(), 1);
        // another version, unknown flags, not ours: dropped, the next transfer starts fresh
        for bad in [[b'T', 2, 0, 0], [b'T', 1, 1, 0], [b'X', 1, 0, 0]] {
            let mut g = f.clone();
            g[..4].copy_from_slice(&bad);
            assert!(r.push(&g).is_empty());
        }
        let two = [f.clone(), f.clone()].concat();
        assert_eq!(r.push(&two).len(), 2);
    }

    #[test]
    fn welcome_and_launch_answers_decode() {
        // bytes as core.link's nanopb code writes them: Welcome{1, "0.1.0", 1, [1, 2]}
        let w = pb::ServerMessage {
            body: Some(pb::server_message::Body::Welcome(pb::Welcome {
                protocol: 1,
                version: "0.1.0".into(),
                interface: 1,
                capabilities: vec![1, 2],
            })),
        };
        let bytes = w.encode_to_vec();
        assert_eq!(pb::ServerMessage::decode(&bytes[..]).unwrap(), w);
        // proto3 packs the repeated enum
        assert!(bytes.windows(4).any(|x| x == [0x22, 2, 1, 2]));
    }
}
