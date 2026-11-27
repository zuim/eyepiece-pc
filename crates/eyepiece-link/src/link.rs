//! Connection to the goggles bridge (pcdisplay_bridge, over USB or TCP): send side (video/stream/ping/ctrl), receive thread
//! (pose, pong, frame reports, log, info), clock sync, IDR requests, opentrack output.

use crate::clock::mono_us;
use crate::proto::*;
use crate::stats::{Interval, Snapshot};
use nusb::io::{EndpointRead, EndpointWrite};
use nusb::transfer::{Bulk, In, Out};
use nusb::{Device, Interface};
use prost::Message;
use std::collections::VecDeque;
use std::io::{IoSlice, Read, Write};
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

pub type LogFn = Arc<dyn Fn(String) + Send + Sync>;

/// How to reach the bridge. Normal use is USB: the bridge is started through the control link
/// (control 1) and then reached on the video interface. TCP only reaches a bridge started with
/// `-p` (the host-built bridge of the interop tests).
#[derive(Clone)]
pub enum LinkTransport {
    Tcp,
    UsbBulk {
        serial: Option<String>,
        /// start the bridge first through the control link with these arguments (`-1`, ...);
        /// None: use a bridge that already runs (started from the goggles' menu)
        launch: Option<Vec<String>>,
    },
}

#[derive(Clone)]
pub struct LinkOptions {
    /// host:port of a bridge started with `-p` (TCP transport only)
    pub addr: String,
    pub transport: LinkTransport,
    pub connect_timeout: Duration,
    /// opentrack "UDP over network" target, e.g. 127.0.0.1:4242
    pub opentrack: Option<String>,
    pub print_pose: bool,
    /// ask the bridge for head tracking (EP_POSE) on every (re)connect
    pub head_tracking: bool,
}

/// Windowed minimum-RTT clock sync: offset = dev_clock - pc_clock from the ping with the
/// lowest RTT among the last N (tracks drift, unlike "best ever").
#[derive(Default)]
struct ClockSync {
    samples: VecDeque<(u64, i64)>, // (rtt, offset)
}
impl ClockSync {
    fn add(&mut self, rtt: u64, offset: i64) {
        self.samples.push_back((rtt, offset));
        while self.samples.len() > 16 {
            self.samples.pop_front();
        }
    }
    fn best(&self) -> Option<(u64, i64)> {
        self.samples.iter().min_by_key(|s| s.0).copied()
    }
}

pub struct Shared {
    pub alive: AtomicBool,
    clock: Mutex<ClockSync>,
    pub stats: Mutex<Interval>,
    pub info: Mutex<Option<Info>>,
    pub last_pose: Mutex<Option<Pose>>,
    key_request: AtomicBool,
    configure_ack: Mutex<Option<(u32, Option<u32>)>>,
    configure_cv: Condvar,
    gui_alpha_ack: Mutex<Option<(u32, bool, Option<u32>)>>,
    gui_alpha_cv: Condvar,
    /// AUs sent since the last EP_FRAME, and when the first of them was sent
    unacked: AtomicU32,
    t_first_unacked: AtomicU64,
    last_forced_idr: AtomicU64,
    opentrack: Option<(UdpSocket, std::net::SocketAddr)>,
    print_pose: bool,
    pub log: LogFn,
}

impl Shared {
    pub fn offset_us(&self) -> Option<i64> {
        self.clock.lock().unwrap().best().map(|b| b.1)
    }
    pub fn rtt_us(&self) -> Option<u64> {
        self.clock.lock().unwrap().best().map(|b| b.0)
    }
}

pub struct Link {
    sock: Mutex<Option<Writer>>,
    raw: Option<TcpStream>,
    _usb: Option<(Device, Interface)>,
    transport_name: String,
    selected_transport: LinkTransport,
    pub shared: Arc<Shared>,
    configure_lock: Mutex<()>,
    next_configure_id: AtomicU32,
    /// True while the bridge has acknowledged a stream configuration that video may use.
    configured: AtomicBool,
    gui_alpha_lock: Mutex<()>,
    next_gui_alpha_id: AtomicU32,
}

enum Writer {
    Tcp(TcpStream),
    Usb(EndpointWrite<Bulk>),
}

enum Reader {
    Tcp(TcpStream),
    Usb(EndpointRead<Bulk>),
}

fn video_frame_prefix(pts_us: u64, codec: u8, key: bool, au_len: usize) -> std::io::Result<([u8; 4], Vec<u8>, Vec<u8>)> {
    if au_len == 0 || au_len > EP_MAX_PAYLOAD as usize {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid video access unit size"));
    }
    let video = wire::Video { pts_us, codec: codec.into(), key, access_unit: Vec::new() };
    let mut video_prefix = video.encode_to_vec();
    video_prefix.push(0x22); // Video.access_unit
    prost::encoding::encode_varint(au_len as u64, &mut video_prefix);
    let video_len = video_prefix.len() + au_len;
    let mut packet_prefix = Vec::new();
    prost::encoding::encode_key(50, prost::encoding::WireType::LengthDelimited, &mut packet_prefix);
    prost::encoding::encode_varint(video_len as u64, &mut packet_prefix);
    let packet_len = packet_prefix.len() + video_len;
    if packet_len > EP_MAX_PAYLOAD as usize {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "bridge message exceeds size limit"));
    }
    Ok(((packet_len as u32).to_be_bytes(), packet_prefix, video_prefix))
}

impl Writer {
    fn write_parts(&mut self, parts: &[&[u8]]) -> std::io::Result<()> {
        match self {
            Writer::Tcp(stream) => write_all_vectored(stream, parts),
            Writer::Usb(stream) => {
                for part in parts {
                    stream.write_all(part).map_err(usb_io_error)?;
                }
                stream.flush().map_err(usb_io_error)
            }
        }
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Reader::Tcp(stream) => stream.read(buf),
            Reader::Usb(stream) => stream.read(buf).map_err(usb_io_error),
        }
    }
}

fn usb_io_error(error: std::io::Error) -> std::io::Error {
    #[cfg(target_os = "macos")]
    if error.get_ref().and_then(|e| e.downcast_ref::<nusb::transfer::TransferError>())
        == Some(&nusb::transfer::TransferError::Unknown(0xe00002ed))
    {
        return std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            "USB device not responding (IOKit 0xe00002ed); check the USB cable and hub",
        );
    }
    error
}

fn connect_once(addr: &str) -> std::io::Result<TcpStream> {
    let mut last = std::io::Error::other("no address");
    for a in addr.to_socket_addrs()? {
        match TcpStream::connect_timeout(&a, Duration::from_secs(2)) {
            Ok(s) => return Ok(s),
            Err(e) => last = e,
        }
    }
    Err(last)
}

fn open_tcp(addr: &str, timeout: Duration) -> Result<(Writer, Reader, TcpStream), String> {
    let t_end = Instant::now() + timeout;
    let stream = loop {
        match connect_once(addr) {
            Ok(stream) => break stream,
            Err(e) => {
                if Instant::now() >= t_end {
                    return Err(format!(
                        "cannot connect to the bridge on {addr} ({e}); is a bridge with -p listening there?"
                    ));
                }
                std::thread::sleep(Duration::from_millis(300));
            }
        }
    };
    stream.set_nodelay(true).ok();
    let writer = stream.try_clone().map_err(|e| e.to_string())?;
    let raw = stream.try_clone().map_err(|e| e.to_string())?;
    Ok((Writer::Tcp(writer), Reader::Tcp(stream), raw))
}

fn open_usb(serial: Option<&str>) -> Result<(Writer, Reader, (Device, Interface), String), String> {
    let usb = crate::usb::discover(serial)?;
    let endpoints = usb.endpoints;
    let in_endpoint = usb
        .interface
        .endpoint::<Bulk, In>(endpoints.in_address)
        .map_err(|e| format!("cannot open video interface IN endpoint ({e})"))?;
    let out_endpoint = usb
        .interface
        .endpoint::<Bulk, Out>(endpoints.out_address)
        .map_err(|e| format!("cannot open video interface OUT endpoint ({e})"))?;
    // A full-size USB packet does not terminate a larger bulk transfer.
    // Read one packet at a time so sparse frame reports are not held until
    // a short packet or 64 KiB of subsequent traffic arrives.
    let reader = EndpointRead::new(in_endpoint, endpoints.in_max_packet_size)
        .with_num_transfers(2)
        .with_read_timeout(Duration::from_millis(100));
    let writer = EndpointWrite::new(out_endpoint, 64 * 1024)
        .with_write_timeout(Duration::from_secs(2));
    let name = format!("USB bulk interface {}", endpoints.interface_number);
    Ok((
        Writer::Usb(writer),
        Reader::Usb(reader),
        (usb.device, usb.interface),
        name,
    ))
}

/// USB: claim the video interface, then start the bridge through the control link (when asked
/// to). Claiming first means a second eyepiece-pc stops at "in use" instead of replacing the
/// bridge of the first one's session. No fallback to anything else.
fn select_initial<T>(
    requested: &LinkTransport,
    open_tcp: impl FnOnce() -> Result<T, String>,
    launch: impl FnOnce(Option<&str>, &[String]) -> Result<(), String>,
    open_usb: impl FnOnce(Option<&str>) -> Result<T, String>,
) -> Result<(T, LinkTransport), String> {
    match requested {
        LinkTransport::Tcp => open_tcp().map(|opened| (opened, LinkTransport::Tcp)),
        LinkTransport::UsbBulk { serial, launch: args } => {
            let opened = open_usb(serial.as_deref())
                .map_err(|error| format!("cannot connect to the bridge over USB: {error}"))?;
            if let Some(args) = args {
                launch(serial.as_deref(), args)?;
            }
            Ok((opened, requested.clone()))
        }
    }
}

impl Link {
    /// Connect (retrying until `connect_timeout`) and start the receive + ping threads.
    pub fn connect(o: &LinkOptions, log: LogFn) -> Result<Arc<Link>, String> {
        let ((writer, reader, raw, usb, transport_name), selected_transport) = select_initial(
            &o.transport,
            || {
                let (writer, reader, raw) = open_tcp(&o.addr, o.connect_timeout)?;
                Ok((
                    writer,
                    reader,
                    Some(raw),
                    None,
                    format!("TCP {}", o.addr),
                ))
            },
            |serial, args| crate::control::launch_bridge(serial, args, &*log).map(|_| ()),
            |serial| {
                let (writer, reader, usb, name) = open_usb(serial)?;
                Ok((writer, reader, None, Some(usb), name))
            },
        )?;
        let ot = match &o.opentrack {
            Some(a) if !a.is_empty() => {
                let addr = a
                    .to_socket_addrs()
                    .map_err(|e| format!("--opentrack {a}: {e}"))?
                    .next()
                    .ok_or_else(|| format!("--opentrack {a}: no address"))?;
                let u = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
                Some((u, addr))
            }
            _ => None,
        };
        let shared = Arc::new(Shared {
            alive: AtomicBool::new(true),
            clock: Mutex::new(ClockSync::default()),
            stats: Mutex::new(Interval::default()),
            info: Mutex::new(None),
            last_pose: Mutex::new(None),
            key_request: AtomicBool::new(false),
            configure_ack: Mutex::new(None),
            configure_cv: Condvar::new(),
            gui_alpha_ack: Mutex::new(None),
            gui_alpha_cv: Condvar::new(),
            unacked: AtomicU32::new(0),
            t_first_unacked: AtomicU64::new(0),
            last_forced_idr: AtomicU64::new(0),
            opentrack: ot,
            print_pose: o.print_pose,
            log,
        });
        let link = Arc::new(Link {
            raw,
            sock: Mutex::new(Some(writer)),
            _usb: usb,
            transport_name,
            selected_transport,
            shared,
            configure_lock: Mutex::new(()),
            next_configure_id: AtomicU32::new(1),
            configured: AtomicBool::new(false),
            gui_alpha_lock: Mutex::new(()),
            next_gui_alpha_id: AtomicU32::new(1),
        });
        {
            let l = link.clone();
            let rx = reader;
            std::thread::Builder::new().name("ep-rx".into()).spawn(move || l.rx_loop(rx)).unwrap();
        }
        {
            let l = link.clone();
            std::thread::Builder::new().name("ep-ping".into()).spawn(move || l.ping_loop()).unwrap();
        }
        if o.head_tracking {
            link.set_head_tracking(true).map_err(|e| e.to_string())?;
        }
        Ok(link)
    }

    pub fn alive(&self) -> bool {
        self.shared.alive.load(Ordering::Relaxed)
    }

    pub fn transport_name(&self) -> &str {
        &self.transport_name
    }

    pub fn selected_transport(&self) -> LinkTransport {
        self.selected_transport.clone()
    }

    pub fn close(&self) {
        let _ack = self.shared.configure_ack.lock().unwrap();
        let _gui_ack = self.shared.gui_alpha_ack.lock().unwrap();
        self.shared.alive.store(false, Ordering::Relaxed);
        self.shared.configure_cv.notify_all();
        self.shared.gui_alpha_cv.notify_all();
        drop(_gui_ack);
        drop(_ack);
        self.shutdown_io();
    }

    fn fail(&self, e: std::io::Error) -> std::io::Error {
        let ack = self.shared.configure_ack.lock().unwrap();
        let gui_ack = self.shared.gui_alpha_ack.lock().unwrap();
        let was_alive = self.shared.alive.swap(false, Ordering::Relaxed);
        self.shared.configure_cv.notify_all();
        self.shared.gui_alpha_cv.notify_all();
        drop(gui_ack);
        drop(ack);
        if was_alive {
            (self.shared.log)(format!("connection to goggles closed on {} ({e})", self.transport_name));
        }
        self.shutdown_io();
        e
    }

    fn shutdown_io(&self) {
        if let Some(raw) = &self.raw {
            let _ = raw.shutdown(std::net::Shutdown::Both);
        }
        self.sock.lock().unwrap().take();
    }

    fn write_parts(&self, parts: &[&[u8]]) -> std::io::Result<()> {
        let result = {
            let mut writer = self.sock.lock().unwrap();
            match writer.as_mut() {
                Some(writer) => writer.write_parts(parts),
                None => Err(std::io::ErrorKind::BrokenPipe.into()),
            }
        };
        result.map_err(|e| self.fail(e))
    }

    fn send(&self, message: wire::envelope::Message) -> std::io::Result<()> {
        let frame = bridge_frame(message)?;
        self.write_parts(&[&frame])
    }

    fn send_request(&self, id: u32, message: wire::envelope::Message) -> std::io::Result<()> {
        let envelope = wire::Envelope {
            id: Some(id.to_string()), sender: None, recipient: None, message: Some(message),
        };
        let frame = encode_frame(&envelope, EP_MAX_PAYLOAD as usize)?;
        self.write_parts(&[&frame])
    }

    /// Encode the protobuf prefixes separately so the access unit is not copied.
    pub fn send_video(&self, pts_us: u64, codec: u8, key: bool, au: &[u8]) -> std::io::Result<()> {
        let _guard = self.configure_lock.lock().unwrap();
        if !self.configured.load(Ordering::Relaxed) {
            return Err(std::io::Error::other("stream is not configured"));
        }
        let (length, packet_prefix, video_prefix) = video_frame_prefix(pts_us, codec, key, au.len())?;
        self.write_parts(&[&length, &packet_prefix, &video_prefix, au])?;
        let now = mono_us();
        if self.shared.unacked.fetch_add(1, Ordering::Relaxed) == 0 {
            self.shared.t_first_unacked.store(now, Ordering::Relaxed);
        }
        if key {
            self.shared.last_forced_idr.store(now, Ordering::Relaxed);
        }
        Ok(())
    }

    /// The bridge's EP_INFO (arrives right after connecting; None until then).
    pub fn info(&self) -> Option<Info> {
        *self.shared.info.lock().unwrap()
    }

    /// Define or replace the stream. Video is refused until the bridge acknowledges it, and
    /// again after any failure. Stop sending old video before calling; resume with an IDR.
    pub fn configure(&self, config: StreamConfig, timeout: Duration) -> std::io::Result<()> {
        let _guard = self.configure_lock.lock().unwrap();
        let info = self.info().ok_or_else(|| std::io::Error::other("bridge capabilities not received"))?;
        let replacing = self.configured.swap(false, Ordering::Relaxed);
        if replacing && info.flags & EP_INFO_RECONFIG == 0 {
            return Err(std::io::Error::other("bridge does not support live stream reconfiguration"));
        }
        let payload = config.payload(info).map_err(std::io::Error::other)?;
        let id = self.next_configure_id.fetch_add(1, Ordering::Relaxed);
        *self.shared.configure_ack.lock().unwrap() = Some((id, None));
        if let Err(e) = self.send_request(id, wire::envelope::Message::Configure(payload)) {
            *self.shared.configure_ack.lock().unwrap() = None;
            return Err(e);
        }
        let ack = self.shared.configure_ack.lock().unwrap();
        let (mut ack, _) = self.shared.configure_cv.wait_timeout_while(ack, timeout, |ack| {
            ack.as_ref().is_some_and(|(_, status)| status.is_none()) && self.alive()
        }).unwrap();
        let result = ack.take().and_then(|(_, status)| status);
        drop(ack);
        match result {
            Some(0) => {
                self.configured.store(true, Ordering::Relaxed);
                Ok(())
            }
            // A rejected request leaves the previous stream active.
            Some(status @ (1 | 3)) => {
                self.configured.store(replacing, Ordering::Relaxed);
                Err(std::io::Error::other(if status == 1 {
                    "bridge rejected the stream configuration"
                } else {
                    "bridge does not support this stream configuration"
                }))
            }
            Some(status) => Err(self.fail(std::io::Error::other(format!("bridge stream configuration failed ({status})")))),
            None => Err(self.fail(std::io::Error::new(std::io::ErrorKind::TimedOut, "bridge configuration not acknowledged"))),
        }
    }
    pub fn set_gui_alpha(&self, alpha: u8, timeout: Duration) -> std::io::Result<()> {
        self.set_alpha(alpha, timeout, false)
    }

    pub fn set_pane_alpha(&self, alpha: u8, timeout: Duration) -> std::io::Result<()> {
        self.set_alpha(alpha, timeout, true)
    }

    fn set_alpha(&self, alpha: u8, timeout: Duration, pane: bool) -> std::io::Result<()> {
        let _guard = self.gui_alpha_lock.lock().unwrap();
        let info = self.info().ok_or_else(|| std::io::Error::other("bridge capabilities not received"))?;
        let name = if pane { "pane opacity" } else { "GUI opacity" };
        let capability = if pane { EP_INFO_PANE_ALPHA } else { EP_INFO_GUI_ALPHA };
        if info.flags & capability == 0 {
            return Err(std::io::Error::other(format!("bridge does not support {name}")));
        }
        let id = self.next_gui_alpha_id.fetch_add(1, Ordering::Relaxed);
        let message = if pane {
            wire::envelope::Message::PaneAlpha(wire::PaneAlpha { alpha: alpha.into() })
        } else {
            wire::envelope::Message::GuiAlpha(wire::GuiAlpha { alpha: alpha.into() })
        };
        *self.shared.gui_alpha_ack.lock().unwrap() = Some((id, pane, None));
        if let Err(e) = self.send_request(id, message) {
            *self.shared.gui_alpha_ack.lock().unwrap() = None;
            return Err(e);
        }
        let ack = self.shared.gui_alpha_ack.lock().unwrap();
        let (mut ack, _) = self.shared.gui_alpha_cv.wait_timeout_while(ack, timeout, |ack| {
            ack.as_ref().is_some_and(|(_, _, status)| status.is_none()) && self.alive()
        }).unwrap();
        let result = ack.take().and_then(|(_, _, status)| status);
        drop(ack);
        match result {
            Some(0) => Ok(()),
            Some(status) => Err(std::io::Error::other(format!("{name} unavailable ({status}); check bridge and overlay"))),
            None => Err(self.fail(std::io::Error::new(std::io::ErrorKind::TimedOut, format!("{name} not acknowledged")))),
        }
    }
    pub fn recenter(&self) -> std::io::Result<()> {
        self.send(wire::envelope::Message::Control(wire::Control { operation: EP_OP_RECENTER.into(), enabled: false }))
    }
    /// Start/stop the goggles' head-attitude stream (the IMU stream runs only while on).
    pub fn set_head_tracking(&self, on: bool) -> std::io::Result<()> {
        self.send(wire::envelope::Message::Control(wire::Control { operation: EP_OP_HEADTRACK.into(), enabled: on }))
    }
    pub fn quit_bridge(&self) -> std::io::Result<()> {
        self.send(wire::envelope::Message::Control(wire::Control { operation: EP_OP_QUIT.into(), enabled: false }))
    }

    /// Bytes written but not yet taken by the peer (the bridge): Linux SIOCOUTQ.
    pub fn unsent_bytes(&self) -> Option<usize> {
        #[cfg(target_os = "linux")]
        {
            use std::os::fd::AsRawFd;
            let mut n: libc::c_int = 0;
            let raw = self.raw.as_ref()?;
            let r = unsafe { libc::ioctl(raw.as_raw_fd(), libc::TIOCOUTQ, &mut n) };
            if r == 0 {
                return Some(n as usize);
            }
        }
        None
    }

    /// True once if the bridge needs an IDR: it said so (log / EP_KEYREQ), or video is sent
    /// but no frame has been shown for a while (bridge waiting for a keyframe). Rate-limited.
    pub fn take_key_request(&self) -> bool {
        let sh = &self.shared;
        let now = mono_us();
        let last = sh.last_forced_idr.load(Ordering::Relaxed);
        if now.saturating_sub(last) < 250_000 {
            return false;
        }
        if sh.key_request.swap(false, Ordering::Relaxed) {
            return true;
        }
        let video_decoder = sh.info.lock().unwrap().map(|i| i.decoder != 0).unwrap_or(false);
        let n = sh.unacked.load(Ordering::Relaxed);
        let t0 = sh.t_first_unacked.load(Ordering::Relaxed);
        video_decoder && n >= 4 && now.saturating_sub(t0) > 200_000
    }

    pub fn note_forced_idr(&self) {
        self.shared.stats.lock().unwrap().idr += 1;
    }

    /// Statistics since the last call.
    pub fn take_snapshot(&self, interval_s: f64) -> Snapshot {
        let iv = self.shared.stats.lock().unwrap().reset_keep_totals();
        let pose = self.shared.last_pose.lock().unwrap().map(|p| (p.yaw, p.pitch));
        Snapshot::from_interval(&iv, interval_s, self.shared.rtt_us(), pose)
    }

    fn ping_loop(&self) {
        let mut n = 0u32;
        while self.alive() {
            if self.send(wire::envelope::Message::Ping(wire::Ping { pc_us: mono_us() })).is_err() {
                break;
            }
            n += 1;
            // quick initial sync, then 1 Hz (windowed min-RTT over the last 16 pongs)
            std::thread::sleep(Duration::from_millis(if n < 8 { 150 } else { 1000 }));
        }
    }

    fn rx_loop(&self, mut s: Reader) {
        let mut buf = Vec::new();
        let r: std::io::Result<()> = (|| {
            while self.alive() {
                let mut h = [0u8; 4];
                read_exact_active(&mut s, &mut h, &self.shared.alive)?;
                let len = u32::from_be_bytes(h);
                if len == 0 || len > EP_MAX_PAYLOAD {
                    return Err(std::io::Error::other("invalid bridge protobuf message size (incompatible bridge?)"));
                }
                buf.resize(len as usize, 0);
                read_exact_active(&mut s, &mut buf, &self.shared.alive)?;
                let envelope = decode_envelope(&buf, EP_MAX_PAYLOAD as usize)?;
                self.handle(envelope)?;
            }
            Ok(())
        })();
        if let Err(e) = r {
            if self.alive() {
                self.fail(e);
            }
        }
        let _ack = self.shared.configure_ack.lock().unwrap();
        let _gui_ack = self.shared.gui_alpha_ack.lock().unwrap();
        self.shared.alive.store(false, Ordering::Relaxed);
        self.shared.configure_cv.notify_all();
        self.shared.gui_alpha_cv.notify_all();
    }

    fn handle(&self, envelope: wire::Envelope) -> std::io::Result<()> {
        let sh = &self.shared;
        let pane_alpha_ack = matches!(&envelope.message, Some(wire::envelope::Message::PaneAlphaAck(_)));
        let id = envelope.id;
        match envelope.message.ok_or_else(|| std::io::Error::other("envelope has no message"))? {
            wire::envelope::Message::Pose(value) => {
                let pose = Pose::try_from(value)
                    .map_err(std::io::Error::other)?;
                *sh.last_pose.lock().unwrap() = Some(pose);
                sh.stats.lock().unwrap().poses += 1;
                if let Some((u, a)) = &sh.opentrack {
                    // opentrack UDP input: 6 little-endian doubles x,y,z,yaw,pitch,roll
                    let mut b = [0u8; 48];
                    for (i, v) in [0.0, 0.0, 0.0, pose.yaw as f64, pose.pitch as f64, pose.roll as f64]
                        .iter()
                        .enumerate()
                    {
                        b[i * 8..i * 8 + 8].copy_from_slice(&v.to_le_bytes());
                    }
                    let _ = u.send_to(&b, a);
                }
                if sh.print_pose {
                    println!(
                        "pose yaw={:7.2} pitch={:7.2} roll={:7.2} src={}",
                        pose.yaw, pose.pitch, pose.roll, pose.source
                    );
                }
            }
            wire::envelope::Message::Pong(pong) => {
                let t_pc = pong.pc_us;
                let t_dev = pong.device_us;
                let now = mono_us();
                let invalid = || std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid bridge pong timestamp");
                let rtt = now.checked_sub(t_pc).ok_or_else(invalid)?;
                let offset = i64::try_from(i128::from(t_dev) - i128::from(t_pc + rtt / 2))
                    .map_err(|_| invalid())?;
                sh.clock.lock().unwrap().add(rtt, offset);
            }
            wire::envelope::Message::Frame(value) => {
                let f = FrameReport::from(value);
                let off = sh.offset_us();
                let times = off.map(|o| {
                    let elapsed = |timestamp| {
                        i64::try_from(i128::from(timestamp) - i128::from(o) - i128::from(f.pts_us))
                            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid bridge frame timestamp"))
                    };
                    Ok::<_, std::io::Error>((elapsed(f.t_shown_us)?, elapsed(f.t_recv_us)?))
                }).transpose()?;
                sh.unacked.store(0, Ordering::Relaxed);
                let mut s = sh.stats.lock().unwrap();
                s.shown += 1;
                s.dec.push(f.decode_us);
                s.cvt.push(f.convert_us);
                s.q_max = s.q_max.max(f.queue_depth);
                s.dropped = f.dropped;
                if let Some((latency, receive)) = times {
                    s.lat.push(latency);
                    s.recv.push(receive);
                }
            }
            wire::envelope::Message::Log(value) => {
                let m = value.line;
                // "decode queue overflow → waiting for next keyframe": resync now, not at the next GOP
                if m.contains("keyframe") || m.contains("overflow") {
                    sh.key_request.store(true, Ordering::Relaxed);
                }
                (sh.log)(format!("goggles: {m}"));
            }
            wire::envelope::Message::KeyRequest(_) => {
                sh.key_request.store(true, Ordering::Relaxed);
            }
            wire::envelope::Message::ConfigureAck(wire::Acknowledgement { status }) => {
                let id = bridge_ack_id(id.as_deref())?;
                let mut ack = sh.configure_ack.lock().unwrap();
                if let Some((pending, result)) = ack.as_mut() {
                    if *pending == id && result.is_none() {
                        *result = Some(status);
                        sh.configure_cv.notify_all();
                    }
                }
            }
            wire::envelope::Message::GuiAlphaAck(wire::Acknowledgement { status }) |
            wire::envelope::Message::PaneAlphaAck(wire::Acknowledgement { status }) => {
                let id = bridge_ack_id(id.as_deref())?;
                let mut ack = sh.gui_alpha_ack.lock().unwrap();
                if let Some((pending, pane, result)) = ack.as_mut() {
                    if *pending == id && *pane == pane_alpha_ack && result.is_none() {
                        *result = Some(status);
                        sh.gui_alpha_cv.notify_all();
                    }
                }
            }
            wire::envelope::Message::Info(value) => {
                let i = Info::try_from(value)
                    .map_err(std::io::Error::other)?;
                (sh.log)(format!(
                    "bridge v{}: fb {}x{}, decoder={}, headtrack flags={:#x}",
                    i.version,
                    i.fb_w,
                    i.fb_h,
                    i.decoder_name(),
                    i.headtrack
                ));
                if i.version != EP_VERSION {
                    return Err(std::io::Error::other(format!("protocol version mismatch (bridge {}, pc {EP_VERSION})", i.version)));
                }
                *sh.info.lock().unwrap() = Some(i);
            }
            _ => return Err(std::io::Error::other("unexpected bridge message")),
        }
        Ok(())
    }
}

fn bridge_ack_id(id: Option<&str>) -> std::io::Result<u32> {
    id.filter(|id| !id.is_empty() && id.len() <= 10
        && (id.len() == 1 || !id.starts_with('0'))
        && id.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid bridge acknowledgement ID"))
}

fn read_exact_active<R: Read>(reader: &mut R, buf: &mut [u8], alive: &AtomicBool) -> std::io::Result<()> {
    let mut offset = 0;
    while offset < buf.len() {
        if !alive.load(Ordering::Relaxed) {
            return Err(std::io::ErrorKind::Interrupted.into());
        }
        match reader.read(&mut buf[offset..]) {
            Ok(0) if !alive.load(Ordering::Relaxed) => return Err(std::io::ErrorKind::Interrupted.into()),
            Ok(0) => return Err(std::io::ErrorKind::UnexpectedEof.into()),
            Ok(n) => offset += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut && alive.load(Ordering::Relaxed) => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}


/// writev until everything is written (std's write_all_vectored is unstable).
fn write_all_vectored(s: &mut TcpStream, parts: &[&[u8]]) -> std::io::Result<()> {
    let mut idx = 0usize;
    let mut off = 0usize;
    while idx < parts.len() {
        let slices: Vec<IoSlice> = std::iter::once(&parts[idx][off..])
            .chain(parts[idx + 1..].iter().copied())
            .map(IoSlice::new)
            .collect();
        let mut n = match s.write_vectored(&slices) {
            Ok(0) => return Err(std::io::ErrorKind::WriteZero.into()),
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        while idx < parts.len() && n >= parts[idx].len() - off {
            n -= parts[idx].len() - off;
            idx += 1;
            off = 0;
        }
        off += n;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn read_packet(stream: &mut TcpStream) -> wire::Envelope {
        let mut length = [0; 4];
        stream.read_exact(&mut length).unwrap();
        let mut payload = vec![0; u32::from_be_bytes(length) as usize];
        stream.read_exact(&mut payload).unwrap();
        wire::Envelope::decode(&payload[..]).unwrap()
    }

    fn ack_frame(id: u32, message: wire::envelope::Message) -> Vec<u8> {
        encode_frame(&wire::Envelope {
            id: Some(id.to_string()), sender: None, recipient: None, message: Some(message),
        }, EP_MAX_PAYLOAD as usize).unwrap()
    }

    #[test]
    #[ignore = "manual serialization microbenchmark; run in release mode"]
    fn benchmark_video_serialization() {
        use std::hint::black_box;

        fn measure(mut operation: impl FnMut(), iterations: usize) -> (u128, u128) {
            const BATCHES: usize = 100;
            let mut samples = Vec::with_capacity(BATCHES);
            for _ in 0..BATCHES {
                let start = Instant::now();
                for _ in 0..iterations / BATCHES {
                    operation();
                }
                samples.push(start.elapsed().as_nanos() / (iterations / BATCHES) as u128);
            }
            samples.sort_unstable();
            (samples[BATCHES / 2], samples[BATCHES * 95 / 100])
        }

        println!("access_unit_bytes,iterations,encoded_bytes,prefix_median_ns,prefix_p95_ns,prost_encode_median_ns,prost_encode_p95_ns,prost_decode_median_ns,prost_decode_p95_ns");
        for (size, iterations) in [(8192, 20000), (25000, 20000), (262144, 5000), ((4 << 20) - 64, 500)] {
            let envelope = wire::Envelope {
                message: Some(wire::envelope::Message::Video(wire::Video {
                    pts_us: 123456789, codec: 1, key: true, access_unit: vec![0xa5; size],
                })),
                ..Default::default()
            };
            let frame = encode_frame(&envelope, EP_MAX_PAYLOAD as usize).unwrap();
            let (prefix, prefix_p95) = measure(|| {
                black_box(video_frame_prefix(123456789, 1, true, black_box(size)).unwrap());
            }, iterations);
            let (encode, encode_p95) = measure(|| {
                black_box(encode_frame(black_box(&envelope), EP_MAX_PAYLOAD as usize).unwrap());
            }, iterations);
            let (decode, decode_p95) = measure(|| {
                black_box(decode_envelope(black_box(&frame[4..]), EP_MAX_PAYLOAD as usize).unwrap());
            }, iterations);
            println!("{size},{iterations},{},{prefix},{prefix_p95},{encode},{encode_p95},{decode},{decode_p95}", frame.len() - 4);
        }
    }

    #[test]
    fn video_protobuf_preserves_access_units_and_size_limit() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            let mut videos = Vec::new();
            while videos.len() < 2 {
                let packet = read_packet(&mut stream);
                if let Some(wire::envelope::Message::Video(video)) = packet.message {
                    videos.push(video);
                }
            }
            videos
        });
        let options = LinkOptions {
            addr: addr.to_string(), transport: LinkTransport::Tcp,
            connect_timeout: Duration::from_secs(2), opentrack: None,
            print_pose: false, head_tracking: false,
        };
        let link = Link::connect(&options, Arc::new(|_| {})).unwrap();
        assert!(link.send_video(1, EP_CODEC_HEVC, true, &[1]).is_err());
        link.configured.store(true, Ordering::Relaxed);
        link.send_video(123, EP_CODEC_HEVC, true, &[1, 2, 3]).unwrap();
        let mut video = wire::Video {
            pts_us: 123, codec: EP_CODEC_HEVC.into(), key: true,
            access_unit: vec![0xa5; EP_MAX_PAYLOAD as usize],
        };
        loop {
            let video_len = video.encoded_len();
            let packet_len = 2 + prost::encoding::encoded_len_varint(video_len as u64) + video_len;
            if packet_len <= EP_MAX_PAYLOAD as usize {
                assert_eq!(packet_len, EP_MAX_PAYLOAD as usize);
                break;
            }
            video.access_unit.pop();
        }
        link.send_video(video.pts_us, EP_CODEC_HEVC, true, &video.access_unit).unwrap();
        video.access_unit.push(0xa5);
        assert_eq!(link.send_video(123, EP_CODEC_HEVC, true, &video.access_unit).unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
        assert!(link.send_video(123, EP_CODEC_HEVC, true, &[]).is_err());
        let received = server.join().unwrap();
        assert_eq!(received[0].access_unit, [1, 2, 3]);
        assert_eq!((received[0].pts_us, received[0].codec, received[0].key), (123, 1, true));
        video.access_unit.pop();
        assert_eq!(received[1], video);
        link.close();
    }

    #[test]
    fn incompatible_and_malformed_bridge_messages_close_the_link() {
        let version = bridge_frame(wire::envelope::Message::Info(wire::Info { version: EP_VERSION - 1, ..Default::default() })).unwrap();
        let mut bad_frame = bridge_frame(wire::envelope::Message::Pong(wire::Pong {
            pc_us: 0, device_us: 0,
        })).unwrap();
        bad_frame.extend(bridge_frame(wire::envelope::Message::Frame(wire::Frame {
            shown_us: u64::MAX, ..Default::default()
        })).unwrap());
        let cases = [
            (version, "protocol version mismatch"),
            (bridge_frame(wire::envelope::Message::Pong(wire::Pong {
                pc_us: u64::MAX, device_us: 0,
            })).unwrap(), "invalid bridge pong timestamp"),
            (bridge_frame(wire::envelope::Message::Pong(wire::Pong {
                pc_us: 0, device_us: u64::MAX,
            })).unwrap(), "invalid bridge pong timestamp"),
            (bad_frame, "invalid bridge frame timestamp"),
            (b"EP\x85\x00\x0c\x00\x00\x00".to_vec(), "incompatible bridge"),
            (vec![0, 0, 0, 1, 0], "failed to decode"),
            (vec![0, 0, 0, 2, 0x12, 0], "has no message"),
        ];
        for (bytes, expected) in cases {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let (done_tx, done_rx) = std::sync::mpsc::channel();
            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream.write_all(&bytes).unwrap();
                done_rx.recv_timeout(Duration::from_secs(3)).unwrap();
            });
            let messages = Arc::new(Mutex::new(Vec::new()));
            let captured = messages.clone();
            let options = LinkOptions {
                addr: addr.to_string(), transport: LinkTransport::Tcp,
                connect_timeout: Duration::from_secs(2), opentrack: None,
                print_pose: false, head_tracking: false,
            };
            let link = Link::connect(&options, Arc::new(move |message| captured.lock().unwrap().push(message))).unwrap();
            let deadline = Instant::now() + Duration::from_secs(2);
            while (link.alive() || !messages.lock().unwrap().iter().any(|message| message.contains(expected)))
                && Instant::now() < deadline
            {
                std::thread::sleep(Duration::from_millis(5));
            }
            assert!(!link.alive());
            let messages = messages.lock().unwrap();
            assert!(messages.iter().any(|message| message.contains(expected)),
                    "missing {expected} in {:?}", *messages);
            drop(messages);
            done_tx.send(()).unwrap();
            server.join().unwrap();
        }
    }

    #[test]
    fn opacity_requires_matching_ack_type_and_id_and_reports_failure() {
        for pane in [false, true] {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            let info = wire::Info { version: EP_VERSION, flags: EP_INFO_GUI_ALPHA | EP_INFO_PANE_ALPHA, ..Default::default() };
            stream.write_all(&bridge_frame(wire::envelope::Message::Info(info)).unwrap()).unwrap();
            let mut requests = 0;
            while requests < 3 {
                let packet = read_packet(&mut stream);
                let alpha = match packet.message {
                    Some(wire::envelope::Message::GuiAlpha(request)) if !pane => request.alpha,
                    Some(wire::envelope::Message::PaneAlpha(request)) if pane => request.alpha,
                    _ => continue,
                };
                let ack = |status| {
                    let ack = wire::Acknowledgement { status };
                    if pane { wire::envelope::Message::PaneAlphaAck(ack) }
                    else { wire::envelope::Message::GuiAlphaAck(ack) }
                };
                requests += 1;
                let id: u32 = packet.id.unwrap().parse().unwrap();
                if requests == 3 { std::thread::sleep(Duration::from_millis(300)); break; }
                assert_eq!(alpha, if requests == 1 { 128 } else { 0 });
                if requests == 1 {
                    let wrong = wire::Acknowledgement { status: 2 };
                    let wrong = if pane { wire::envelope::Message::GuiAlphaAck(wrong) }
                                else { wire::envelope::Message::PaneAlphaAck(wrong) };
                    stream.write_all(&ack_frame(id, wrong)).unwrap();
                    stream.write_all(&ack_frame(id + 1, ack(0))).unwrap();
                }
                stream.write_all(&ack_frame(id, ack(if requests == 2 { 2 } else { 0 }))).unwrap();
            }
        });
        let options = LinkOptions {
            addr: addr.to_string(), transport: LinkTransport::Tcp,
            connect_timeout: Duration::from_secs(2), opentrack: None,
            print_pose: false, head_tracking: false,
        };
        let link = Link::connect(&options, Arc::new(|_| {})).unwrap();
        let until = Instant::now() + Duration::from_secs(2);
        while link.info().is_none() && Instant::now() < until {
            std::thread::sleep(Duration::from_millis(5));
        }
        let info = link.info().unwrap();
        let set = |alpha, timeout| {
            if pane { link.set_pane_alpha(alpha, timeout) } else { link.set_gui_alpha(alpha, timeout) }
        };
        *link.shared.info.lock().unwrap() = Some(Info { flags: 0, ..info });
        assert!(set(128, Duration::from_secs(2)).is_err());
        *link.shared.info.lock().unwrap() = Some(info);
        set(128, Duration::from_secs(2)).unwrap();
        assert!(set(0, Duration::from_secs(2)).is_err());
        assert!(link.alive());
        let error = set(255, Duration::from_millis(100)).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert!(!link.alive());
        server.join().unwrap();
        }
    }

    #[test]
    fn configure_waits_for_matching_ack_and_gates_video() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            let info = wire::Info { version: EP_VERSION, framebuffer_width: 1920, framebuffer_height: 1080,
                flags: (EP_INFO_RECONFIG | EP_INFO_PANE_BGRA).into(), ..Default::default() };
            stream.write_all(&bridge_frame(wire::envelope::Message::Info(info)).unwrap()).unwrap();
            let mut requests = 0;
            while requests < 4 {
                let packet = read_packet(&mut stream);
                let Some(wire::envelope::Message::Configure(_)) = packet.message else { continue; };
                requests += 1;
                let id: u32 = packet.id.unwrap().parse().unwrap();
                if requests == 1 {
                    stream.write_all(&ack_frame(id + 1, wire::envelope::Message::ConfigureAck(wire::Acknowledgement { status: 0 }))).unwrap();
                }
                let status = match requests { 2 => 1, 3 => 3, 4 => 2, _ => 0 };
                stream.write_all(&ack_frame(id, wire::envelope::Message::ConfigureAck(wire::Acknowledgement { status }))).unwrap();
            }
        });
        let options = LinkOptions {
            addr: addr.to_string(), transport: LinkTransport::Tcp,
            connect_timeout: Duration::from_secs(2), opentrack: None,
            print_pose: false, head_tracking: false,
        };
        let link = Link::connect(&options, Arc::new(|_| {})).unwrap();
        let until = Instant::now() + Duration::from_secs(2);
        while link.info().is_none() && Instant::now() < until {
            std::thread::sleep(Duration::from_millis(5));
        }
        let pane = Pane { x: 20, y: 30, width: 640, height: 480, format: PaneFormat::Bgra };
        let config = StreamConfig { codec: EP_CODEC_H264, layout: EP_LAYOUT_MONO, pane: Some(pane) };
        // Video is refused until a configuration is acknowledged.
        assert!(link.send_video(1, EP_CODEC_H264, true, &[1]).is_err());
        link.configure(config, Duration::from_secs(2)).unwrap();
        link.send_video(2, EP_CODEC_H264, true, &[1]).unwrap();
        // Invalid and unsupported requests leave the active stream usable.
        assert!(link.configure(config, Duration::from_secs(2)).is_err());
        assert!(link.alive());
        link.send_video(3, EP_CODEC_H264, true, &[1]).unwrap();
        assert!(link.configure(config, Duration::from_secs(2)).is_err());
        assert!(link.alive());
        // A failed restart closes the link.
        assert!(link.configure(config, Duration::from_secs(2)).is_err());
        assert!(!link.alive());
        assert!(link.send_video(4, EP_CODEC_H264, true, &[1]).is_err());
        server.join().unwrap();
    }

    #[test]
    fn configure_times_out_and_closes_the_link() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            let info = wire::Info { version: EP_VERSION, framebuffer_width: 1920, framebuffer_height: 1080, ..Default::default() };
            stream.write_all(&bridge_frame(wire::envelope::Message::Info(info)).unwrap()).unwrap();
            loop {
                if matches!(read_packet(&mut stream).message, Some(wire::envelope::Message::Configure(_))) { break; }
            }
            std::thread::sleep(Duration::from_millis(300));
        });
        let options = LinkOptions {
            addr: addr.to_string(), transport: LinkTransport::Tcp,
            connect_timeout: Duration::from_secs(2), opentrack: None,
            print_pose: false, head_tracking: false,
        };
        let link = Link::connect(&options, Arc::new(|_| {})).unwrap();
        let until = Instant::now() + Duration::from_secs(2);
        while link.info().is_none() && Instant::now() < until {
            std::thread::sleep(Duration::from_millis(5));
        }
        let config = StreamConfig { codec: EP_CODEC_H264, layout: EP_LAYOUT_MONO, pane: None };
        let error = link.configure(config, Duration::from_millis(100)).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert!(!link.alive());
        server.join().unwrap();
    }

    enum ReadStep {
        Data(Vec<u8>),
        Timeout,
    }

    struct FragmentedReader {
        steps: VecDeque<ReadStep>,
        pending: VecDeque<u8>,
    }

    impl Read for FragmentedReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.pending.is_empty() {
                match self.steps.pop_front() {
                    Some(ReadStep::Data(bytes)) => self.pending.extend(bytes),
                    Some(ReadStep::Timeout) => return Err(std::io::ErrorKind::TimedOut.into()),
                    None => return Ok(0),
                }
            }
            let len = buf.len().min(self.pending.len());
            for byte in &mut buf[..len] {
                *byte = self.pending.pop_front().unwrap();
            }
            Ok(len)
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn usb_not_responding_is_fatal_but_read_poll_timeouts_are_not() {
        use nusb::transfer::TransferError;

        let error = usb_io_error(TransferError::Unknown(0xe00002ed).into());
        assert_eq!(error.kind(), std::io::ErrorKind::ConnectionAborted);
        assert!(error.to_string().contains("USB device not responding"));
        assert!(error.to_string().contains("USB cable"));

        for error in [
            std::io::Error::from(std::io::ErrorKind::TimedOut),
            TransferError::Cancelled.into(),
            TransferError::Unknown(0x1234).into(),
        ] {
            let kind = error.kind();
            let message = error.to_string();
            let mapped = usb_io_error(error);
            assert_eq!(mapped.kind(), kind);
            assert_eq!(mapped.to_string(), message);
        }
    }

    #[test]
    fn exact_read_preserves_fragments_across_timeouts() {
        let mut reader = FragmentedReader {
            steps: [
                ReadStep::Data(vec![1, 2]),
                ReadStep::Timeout,
                ReadStep::Data(vec![3]),
                ReadStep::Timeout,
                ReadStep::Data(vec![4, 5]),
            ]
            .into(),
            pending: VecDeque::new(),
        };
        let alive = AtomicBool::new(true);
        let mut bytes = [0; 5];

        read_exact_active(&mut reader, &mut bytes, &alive).unwrap();

        assert_eq!(bytes, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn exact_read_reassembles_max_payload_across_unaligned_fragments() {
        let payload_len = EP_MAX_PAYLOAD as usize;
        let fragment_sizes = [1, 511, 512, 513, 65_535, 65_536];
        let mut steps = VecDeque::new();
        let mut expected = Vec::new();
        let mut offset = 0;
        let mut fragment_index = 0;
        while offset < payload_len {
            let fragment_len = fragment_sizes[fragment_index % fragment_sizes.len()]
                .min(payload_len - offset);
            let value = fragment_index as u8;
            steps.push_back(ReadStep::Data(vec![value; fragment_len]));
            steps.push_back(ReadStep::Timeout);
            expected.push((offset, fragment_len, value));
            offset += fragment_len;
            fragment_index += 1;
        }

        let mut reader = FragmentedReader { steps, pending: VecDeque::new() };
        let alive = AtomicBool::new(true);
        let mut payload = vec![0; payload_len];
        read_exact_active(&mut reader, &mut payload, &alive).unwrap();

        for (offset, fragment_len, value) in expected {
            assert!(payload[offset..offset + fragment_len]
                .iter()
                .all(|byte| *byte == value));
        }
    }

    #[test]
    fn exact_read_stops_after_cancellation() {
        let mut reader = FragmentedReader { steps: VecDeque::new(), pending: VecDeque::new() };
        let alive = AtomicBool::new(false);

        let error = read_exact_active(&mut reader, &mut [0; 1], &alive).unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::Interrupted);
    }

    #[test]
    fn concurrent_sends_keep_message_frames_serialized() {
        use std::net::TcpListener;
        use std::sync::{Arc, Mutex};
        use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64};
        use std::thread;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let client = TcpStream::connect(address).unwrap();
        let (mut peer, _) = listener.accept().unwrap();
        let shared = Arc::new(Shared {
            alive: AtomicBool::new(true),
            clock: Mutex::new(ClockSync::default()),
            stats: Mutex::new(Interval::default()),
            info: Mutex::new(None),
            last_pose: Mutex::new(None),
            key_request: AtomicBool::new(false),
            configure_ack: Mutex::new(None),
            configure_cv: Condvar::new(),
            gui_alpha_ack: Mutex::new(None),
            gui_alpha_cv: Condvar::new(),
            unacked: AtomicU32::new(0),
            t_first_unacked: AtomicU64::new(0),
            last_forced_idr: AtomicU64::new(0),
            opentrack: None,
            print_pose: false,
            log: Arc::new(|_| {}),
        });
        let raw = client.try_clone().unwrap();
        let link = Arc::new(Link {
            sock: Mutex::new(Some(Writer::Tcp(client))),
            raw: Some(raw),
            _usb: None,
            transport_name: "TCP test".into(),
            selected_transport: LinkTransport::Tcp,
            shared,
            configure_lock: Mutex::new(()),
            next_configure_id: AtomicU32::new(1),
            configured: AtomicBool::new(false),
            gui_alpha_lock: Mutex::new(()),
            next_gui_alpha_id: AtomicU32::new(1),
        });

        const THREADS: usize = 6;
        const MESSAGES_PER_THREAD: usize = 12;
        const PAYLOAD_LEN: usize = 2048;
        let receive = thread::spawn(move || {
            for _ in 0..THREADS * MESSAGES_PER_THREAD {
                let packet = read_packet(&mut peer);
                let Some(wire::envelope::Message::Video(message)) = packet.message else { panic!("expected video"); };
                assert_eq!(message.access_unit.len(), PAYLOAD_LEN);
                assert!(message.access_unit.iter().all(|byte| *byte == message.access_unit[0]));
            }
        });
        let senders: Vec<_> = (0..THREADS)
            .map(|thread_id| {
                let link = link.clone();
                thread::spawn(move || {
                    let payload = wire::Video { access_unit: vec![thread_id as u8; PAYLOAD_LEN], ..Default::default() };
                    for _ in 0..MESSAGES_PER_THREAD {
                        link.send(wire::envelope::Message::Video(payload.clone())).unwrap();
                    }
                })
            })
            .collect();
        for sender in senders {
            sender.join().unwrap();
        }
        receive.join().unwrap();
        link.close();
    }

    #[test]
    fn usb_claims_the_video_interface_then_launches_the_bridge() {
        let order = Mutex::new(Vec::new());
        let requested = LinkTransport::UsbBulk { serial: Some("serial".into()), launch: Some(vec!["-1".into()]) };
        let (selected, actual) = select_initial(
            &requested,
            || -> Result<&str, String> { panic!("no TCP for USB") },
            |serial, args| {
                assert_eq!((serial, args), (Some("serial"), &["-1".to_string()][..]));
                order.lock().unwrap().push("launch");
                Ok(())
            },
            |serial| {
                assert_eq!(serial, Some("serial"));
                order.lock().unwrap().push("video");
                Ok("bulk")
            },
        )
        .unwrap();
        assert_eq!(selected, "bulk");
        assert!(matches!(actual, LinkTransport::UsbBulk { launch: Some(_), .. }), "reconnects launch again");
        assert_eq!(*order.lock().unwrap(), ["video", "launch"]);
    }

    #[test]
    fn a_busy_video_interface_launches_nothing_and_nothing_falls_back() {
        let requested = LinkTransport::UsbBulk { serial: None, launch: Some(vec![]) };
        let result = select_initial(
            &requested,
            || -> Result<&str, String> { panic!("no TCP fallback") },
            |_, _| panic!("another session's bridge must not be replaced"),
            |_| Err("the goggles' video interface is in use by another program".into()),
        );
        assert!(matches!(result, Err(error) if error.contains("in use")));
        let failed = select_initial(
            &requested,
            || -> Result<&str, String> { panic!("no TCP fallback") },
            |_, _| Err("PC Display is not installed on the goggles".into()),
            |_| Ok("bulk"),
        );
        assert!(matches!(failed, Err(error) if error.contains("not installed")));
    }

    #[test]
    fn without_launch_the_running_bridge_is_used() {
        let requested = LinkTransport::UsbBulk { serial: None, launch: None };
        let result = select_initial(
            &requested,
            || -> Result<&str, String> { panic!("no TCP") },
            |_, _| panic!("no launch"),
            |_| Err("goggles not found in tinkerglass USB mode".into()),
        );
        assert!(matches!(result, Err(error) if error.contains("over USB") && error.contains("not found")));
    }
}
