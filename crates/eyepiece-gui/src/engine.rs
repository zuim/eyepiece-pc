//! The streaming session behind the GUI: the same `cli::prepare` + `cli::run_prepared` path
//! as the `eyepiece-pc` CLI, on its own thread. The UI never touches the pipeline threads:
//! it flips atomics in [`Control`] and reads [`EngineShared`] (mutex snapshots updated by the
//! session's stats thread once per stats interval, by log lines, and the link's last pose).

use crate::preview::{Knobs, Preview};
use eyepiece_pc::cli::{self, Args};
use eyepiece_pc::link::stats::Snapshot;
use eyepiece_pc::link::{Link, LogFn};
use eyepiece_pc::session::{Control, Events};
use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;

const LOG_LINES: usize = 1000;
const HISTORY: usize = 900;

#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    Preparing,
    Connected,
    Disconnected,
    Stopping,
    Ended(Result<(), String>),
}

pub struct EngineShared {
    pub log: Mutex<VecDeque<String>>,
    /// (seconds since start, snapshot), one per stats interval
    pub history: Mutex<VecDeque<(f64, Snapshot)>>,
    pub link: Mutex<Option<Arc<Link>>>,
    pub phase: Mutex<Phase>,
    /// "encoder: …" / "source: …" / "convert: …" lines, for pins
    pub facts: Mutex<Vec<String>>,
    /// last "NOTICE: …" log line (something the user should do, e.g. connect the USB network)
    pub notice: Mutex<Option<String>>,
    /// actual USB bulk, USB network, TCP, or adb transport
    pub data_link: Mutex<Option<String>>,
}

pub struct Engine {
    pub ctl: Arc<Control>,
    pub shared: Arc<EngineShared>,
    pub preview: Option<Arc<Preview>>,
    pub args: Args,
    thread: Option<JoinHandle<()>>,
}

impl Engine {
    pub fn start(args: Args, knobs: Arc<Knobs>, repaint: Arc<dyn Fn() + Send + Sync>) -> Engine {
        let t0 = Instant::now();
        let shared = Arc::new(EngineShared {
            log: Mutex::new(VecDeque::new()),
            history: Mutex::new(VecDeque::new()),
            link: Mutex::new(None),
            phase: Mutex::new(Phase::Preparing),
            facts: Mutex::new(vec![]),
            notice: Mutex::new(None),
            data_link: Mutex::new(None),
        });
        let log: LogFn = {
            let (sh, rp) = (shared.clone(), repaint.clone());
            Arc::new(move |s: String| {
                eprintln!("[eyepiece-gui] {s}");
                if ["source:", "encoder:", "convert:", "bridge v"].iter().any(|p| s.starts_with(p)) {
                    sh.facts.lock().unwrap().push(s.clone());
                }
                if let Some(n) = s.strip_prefix("NOTICE: ") {
                    *sh.notice.lock().unwrap() = Some(n.to_string());
                    if n.contains("Using adb") {
                        *sh.data_link.lock().unwrap() = Some("adb".into());
                    }
                }
                if let Some(d) = s.strip_prefix("data link: ") {
                    let l = if d.contains("USB bulk") {
                        "USB bulk"
                    } else if d.contains("USB network") {
                        "USB network"
                    } else if d.contains("TCP") {
                        "TCP"
                    } else {
                        "adb"
                    };
                    *sh.data_link.lock().unwrap() = Some(l.into());
                }
                let mut l = sh.log.lock().unwrap();
                l.push_back(s);
                while l.len() > LOG_LINES {
                    l.pop_front();
                }
                drop(l);
                rp();
            })
        };
        let preview = if args.no_video {
            None
        } else {
            match args.codec.parse().and_then(|c| Preview::new(c, knobs, repaint.clone())) {
                Ok(p) => Some(p),
                Err(e) => {
                    log(format!("preview unavailable: {e}"));
                    None
                }
            }
        };
        let ctl = Arc::new(Control::default());
        let mut ev = {
            let (sh, rp, lg) = (shared.clone(), repaint.clone(), log.clone());
            Events::new(
                log.clone(),
                Arc::new(move |s: &Snapshot| {
                    lg(s.line());
                    let mut h = sh.history.lock().unwrap();
                    h.push_back((t0.elapsed().as_secs_f64(), s.clone()));
                    while h.len() > HISTORY {
                        h.pop_front();
                    }
                    drop(h);
                    rp();
                }),
            )
        };
        if let Some(p) = &preview {
            let p = p.clone();
            ev.au = Some(Arc::new(move |au: &[u8], key: bool| p.tap(au, key)));
        }
        {
            let (sh, rp) = (shared.clone(), repaint.clone());
            ev.on_link = Some(Arc::new(move |l: Option<Arc<Link>>| {
                let mut ph = sh.phase.lock().unwrap();
                if !matches!(*ph, Phase::Stopping) {
                    *ph = if l.is_some() { Phase::Connected } else { Phase::Disconnected };
                }
                drop(ph);
                *sh.link.lock().unwrap() = l;
                rp();
            }));
        }
        let mut a = args.clone();
        a.list = false; // the GUI lists outputs itself
        let thread = {
            let (sh, ctl, rp) = (shared.clone(), ctl.clone(), repaint.clone());
            std::thread::Builder::new()
                .name("ep-gui-session".into())
                .spawn(move || {
                    if a.source == "kms" && !a.kms_simulate && !a.no_video {
                        crate::caps::raise_effective();
                    }
                    let r = match cli::prepare(&a, &log) {
                        Ok(Some(p)) => cli::run_prepared(p, ctl, ev),
                        Ok(None) => Ok(()),
                        Err(e) => Err(e),
                    };
                    if let Err(e) = &r {
                        log(format!("error: {e}"));
                    }
                    *sh.link.lock().unwrap() = None;
                    *sh.phase.lock().unwrap() = Phase::Ended(r);
                    rp();
                })
                .expect("spawn session thread")
        };
        Engine { ctl, shared, preview, args, thread: Some(thread) }
    }

    pub fn stop(&self) {
        let mut ph = self.shared.phase.lock().unwrap();
        if !matches!(*ph, Phase::Ended(_)) {
            *ph = Phase::Stopping;
        }
        self.ctl.stop.store(true, Ordering::Relaxed);
    }

    pub fn phase(&self) -> Phase {
        self.shared.phase.lock().unwrap().clone()
    }

    pub fn finished(&self) -> bool {
        self.thread.as_ref().is_none_or(|t| t.is_finished())
    }

    /// Wait for the session thread (it exits shortly after `stop`).
    pub fn join(&mut self) {
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.stop();
        self.join();
    }
}
