//! TCP connection to the goggles' bridge (pcdisplay_bridge): send side (video/stream/ping/ctrl), receive thread
//! (pose, pong, frame reports, log, info), clock sync, IDR requests, opentrack output.

use crate::clock::mono_us;
use crate::proto::*;
use crate::stats::{Interval, Snapshot};
use std::collections::VecDeque;
use std::io::{IoSlice, Read, Write};
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub type LogFn = Arc<dyn Fn(String) + Send + Sync>;

#[derive(Clone)]
pub struct LinkOptions {
    /// host:port of the bridge (adb forward: 127.0.0.1:42420)
    pub addr: String,
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
    sock: Mutex<TcpStream>,
    raw: TcpStream,
    pub shared: Arc<Shared>,
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

impl Link {
    /// Connect (retrying until `connect_timeout`) and start the receive + ping threads.
    pub fn connect(o: &LinkOptions, log: LogFn) -> Result<Arc<Link>, String> {
        let t_end = Instant::now() + o.connect_timeout;
        let s = loop {
            match connect_once(&o.addr) {
                Ok(s) => break s,
                Err(e) => {
                    if Instant::now() >= t_end {
                        return Err(format!(
                            "cannot connect to the bridge on {} ({e}); is it running and is \
                             `adb forward tcp:{p} tcp:{p}` set?",
                            o.addr,
                            p = EP_PORT
                        ));
                    }
                    std::thread::sleep(Duration::from_millis(300));
                }
            }
        };
        s.set_nodelay(true).ok();
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
            unacked: AtomicU32::new(0),
            t_first_unacked: AtomicU64::new(0),
            last_forced_idr: AtomicU64::new(0),
            opentrack: ot,
            print_pose: o.print_pose,
            log,
        });
        let link = Arc::new(Link {
            raw: s.try_clone().map_err(|e| e.to_string())?,
            sock: Mutex::new(s.try_clone().map_err(|e| e.to_string())?),
            shared,
        });
        {
            let l = link.clone();
            let rx = s;
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

    pub fn close(&self) {
        self.shared.alive.store(false, Ordering::Relaxed);
        let _ = self.raw.shutdown(std::net::Shutdown::Both);
    }

    fn fail(&self, e: std::io::Error) -> std::io::Error {
        if self.shared.alive.swap(false, Ordering::Relaxed) {
            (self.shared.log)(format!("connection to goggles closed ({e})"));
        }
        e
    }

    pub fn send(&self, mtype: u8, flags: u8, payload: &[u8]) -> std::io::Result<()> {
        let h = header(mtype, flags, payload.len() as u32);
        let mut s = self.sock.lock().unwrap();
        write_all_vectored(&mut s, &[&h, payload]).map_err(|e| self.fail(e))
    }

    /// EP_VIDEO without copying the AU (header + ep_video_t + AU in one writev).
    pub fn send_video(&self, pts_us: u64, codec: u8, key: bool, au: &[u8]) -> std::io::Result<()> {
        let pre = video_prefix(pts_us, codec);
        let h = header(EP_VIDEO, if key { EP_FLAG_KEY } else { 0 }, (pre.len() + au.len()) as u32);
        {
            let mut s = self.sock.lock().unwrap();
            write_all_vectored(&mut s, &[&h, &pre, au]).map_err(|e| self.fail(e))?;
        }
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

    pub fn send_stream(&self, w: u16, h: u16, codec: u8, fps: u32, layout: u8) -> std::io::Result<()> {
        self.send(EP_STREAM, 0, &stream_payload(w, h, codec, fps, layout))
    }
    pub fn recenter(&self) -> std::io::Result<()> {
        self.send(EP_CTRL, 0, &[EP_OP_RECENTER])
    }
    /// Start/stop the goggles' head-attitude stream (the IMU stream runs only while on).
    pub fn set_head_tracking(&self, on: bool) -> std::io::Result<()> {
        self.send(EP_CTRL, 0, &[EP_OP_HEADTRACK, on as u8])
    }
    pub fn quit_bridge(&self) -> std::io::Result<()> {
        self.send(EP_CTRL, 0, &[EP_OP_QUIT])
    }

    /// Bytes written but not yet taken by the peer (adb server / bridge): Linux SIOCOUTQ.
    pub fn unsent_bytes(&self) -> Option<usize> {
        #[cfg(target_os = "linux")]
        {
            use std::os::fd::AsRawFd;
            let mut n: libc::c_int = 0;
            let r = unsafe { libc::ioctl(self.raw.as_raw_fd(), libc::TIOCOUTQ, &mut n) };
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
            if self.send(EP_PING, 0, &mono_us().to_le_bytes()).is_err() {
                break;
            }
            n += 1;
            // quick initial sync, then 1 Hz (windowed min-RTT over the last 16 pongs)
            std::thread::sleep(Duration::from_millis(if n < 8 { 150 } else { 1000 }));
        }
    }

    fn rx_loop(&self, mut s: TcpStream) {
        let mut buf = Vec::new();
        let r: std::io::Result<()> = (|| {
            while self.alive() {
                let mut h = [0u8; HDR_LEN];
                s.read_exact(&mut h)?;
                let (t, _f, len) = parse_header(&h).ok_or_else(|| std::io::Error::other("bad magic from goggles"))?;
                if len > EP_MAX_PAYLOAD {
                    return Err(std::io::Error::other("oversized message from goggles"));
                }
                buf.resize(len as usize, 0);
                s.read_exact(&mut buf)?;
                self.handle(t, &buf);
            }
            Ok(())
        })();
        if let Err(e) = r {
            if self.alive() {
                self.fail(e);
            }
        }
        self.shared.alive.store(false, Ordering::Relaxed);
    }

    fn handle(&self, t: u8, p: &[u8]) {
        let sh = &self.shared;
        match t {
            EP_POSE => {
                let Some(pose) = Pose::parse(p) else { return };
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
            EP_PONG if p.len() >= 16 => {
                let t_pc = u64::from_le_bytes(p[0..8].try_into().unwrap());
                let t_dev = u64::from_le_bytes(p[8..16].try_into().unwrap());
                let now = mono_us();
                let rtt = now.saturating_sub(t_pc);
                sh.clock.lock().unwrap().add(rtt, t_dev as i64 - (t_pc + rtt / 2) as i64);
            }
            EP_FRAME => {
                let Some(f) = FrameReport::parse(p) else { return };
                sh.unacked.store(0, Ordering::Relaxed);
                let off = sh.offset_us();
                let mut s = sh.stats.lock().unwrap();
                s.shown += 1;
                s.dec.push(f.decode_us);
                s.cvt.push(f.convert_us);
                s.q_max = s.q_max.max(f.queue_depth);
                s.dropped = f.dropped;
                if let Some(o) = off {
                    s.lat.push(f.t_shown_us as i64 - o - f.pts_us as i64);
                    s.recv.push(f.t_recv_us as i64 - o - f.pts_us as i64);
                }
            }
            EP_LOG => {
                let m = String::from_utf8_lossy(p).to_string();
                // "decode queue overflow → waiting for next keyframe": resync now, not at the next GOP
                if m.contains("keyframe") || m.contains("overflow") {
                    sh.key_request.store(true, Ordering::Relaxed);
                }
                (sh.log)(format!("goggles: {m}"));
            }
            EP_KEYREQ => sh.key_request.store(true, Ordering::Relaxed),
            EP_INFO => {
                let Some(i) = Info::parse(p) else { return };
                (sh.log)(format!(
                    "bridge v{}: fb {}x{}, decoder={}, headtrack flags={:#x}",
                    i.version,
                    i.fb_w,
                    i.fb_h,
                    i.decoder_name(),
                    i.headtrack
                ));
                if i.version != EP_VERSION {
                    (sh.log)(format!("WARNING: protocol version mismatch (bridge {}, pc {EP_VERSION})", i.version));
                }
                *sh.info.lock().unwrap() = Some(i);
            }
            _ => {}
        }
    }
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
