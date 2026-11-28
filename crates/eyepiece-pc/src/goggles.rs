//! The goggles over tinkerglass' control link (control 1), kept open while eyepiece-pc runs: the
//! tinkerglass status, the running apps and every mod's settings, for the browser UI, which can
//! change a setting. The link is the process's shared one
//! ([`eyepiece_link::control::SHARED`]), so starting the bridge uses it too.
//!
//! One thread: open the link (again every two seconds while the mods do not answer), subscribe
//! to status and settings events, report a [`wire::Goggles`] whenever something changed, and
//! carry out the UI's setting changes.

use eyepiece_link::control::{self, pb, ControlLink, SHARED};
use eyepiece_link::proto::wire;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

enum Cmd {
    Set { module: String, key: String, value: String },
}

/// The monitor: its last report and a way to change a setting.
#[derive(Clone)]
pub struct Monitor {
    tx: Sender<Cmd>,
    last: Arc<Mutex<wire::Goggles>>,
}

impl Monitor {
    /// Start the thread. `report` gets every new state (and the error of a failed change).
    pub fn spawn(serial: Option<String>, report: impl Fn(Result<wire::Goggles, String>) + Send + 'static) -> Monitor {
        let (tx, rx) = mpsc::channel();
        let last = Arc::new(Mutex::new(wire::Goggles { message: "looking for the goggles…".into(), ..Default::default() }));
        let shared = last.clone();
        thread::Builder::new()
            .name("goggles".into())
            .spawn(move || run(serial, rx, shared, report))
            .expect("goggles thread");
        Monitor { tx, last }
    }

    /// The last report.
    pub fn last(&self) -> wire::Goggles {
        self.last.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// Change one setting; the new state follows as a report.
    pub fn set(&self, module: String, key: String, value: String) {
        let _ = self.tx.send(Cmd::Set { module, key, value });
    }
}

/// The state as the link reports it now; schemas are kept per connection (they change only
/// with an install, which restarts the session and with it the link).
fn snapshot(link: &mut ControlLink, schemas: &mut HashMap<String, pb::ModSchema>) -> Result<wire::Goggles, String> {
    let s = link.status()?;
    let mut g = wire::Goggles {
        connected: true,
        version: s.version,
        session: s.session,
        healthy: s.healthy,
        apps: s.apps.into_iter().map(|a| wire::GogglesApp { entry: a.entry, label: a.label, running: a.pid != 0 }).collect(),
        ..Default::default()
    };
    if !link.can(pb::Capability::Config) || !link.can(pb::Capability::Schema) {
        g.message = "the tinkerglass on the goggles is too old to show its settings here (update it)".into();
        return Ok(g);
    }
    for m in link.get_config("")? {
        if !schemas.contains_key(&m.r#mod) {
            let schema = link.get_schema(&m.r#mod)?;
            schemas.insert(m.r#mod.clone(), schema);
        }
        let schema = &schemas[&m.r#mod];
        let values: HashMap<&str, &str> = m.values.iter().map(|v| (v.key.as_str(), v.value.as_str())).collect();
        g.mods.push(wire::GogglesMod {
            r#mod: m.r#mod.clone(),
            name: schema.name.clone(),
            settings: schema
                .fields
                .iter()
                .map(|f| wire::GogglesSetting {
                    key: f.key.clone(),
                    r#type: f.r#type.clone(),
                    label: f.label.clone(),
                    value: values.get(f.key.as_str()).map_or_else(|| f.default_value.clone(), |v| v.to_string()),
                    options: f.options.clone(),
                    min: f.min,
                    max: f.max,
                    step: f.step,
                    unit: f.unit.clone(),
                    live: f.live,
                })
                .collect(),
        });
    }
    Ok(g)
}

fn run(
    serial: Option<String>,
    rx: Receiver<Cmd>,
    last: Arc<Mutex<wire::Goggles>>,
    report: impl Fn(Result<wire::Goggles, String>),
) {
    let publish = |g: wire::Goggles| {
        let mut l = last.lock().unwrap_or_else(|e| e.into_inner());
        if *l != g {
            *l = g.clone();
            drop(l);
            report(Ok(g));
        }
    };
    let lock = || SHARED.lock().unwrap_or_else(|e| e.into_inner());
    let mut schemas = HashMap::new();
    // a session that ends says nothing: the link just goes quiet. A status request after a quiet
    // while is the heartbeat (and keeps the report fresh).
    let mut heard = Instant::now();
    loop {
        // connect
        if lock().is_none() {
            match ControlLink::open(serial.as_deref(), control::CLIENT) {
                Ok(mut link) => {
                    schemas.clear();
                    let ok = (|| {
                        if link.can(pb::Capability::Events) {
                            link.subscribe(&[pb::Topic::Status, pb::Topic::Config])?;
                        }
                        snapshot(&mut link, &mut schemas)
                    })();
                    match ok {
                        Ok(g) => {
                            *lock() = Some(link);
                            heard = Instant::now();
                            publish(g);
                        }
                        Err(e) => publish(wire::Goggles { message: e, ..Default::default() }),
                    }
                }
                Err(e) => publish(wire::Goggles { message: e, ..Default::default() }),
            }
            if lock().is_none() {
                match rx.recv_timeout(Duration::from_secs(2)) {
                    Ok(Cmd::Set { .. }) => report(Err("the goggles are not connected over the control link".into())),
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return,
                }
                continue;
            }
        }
        // the UI's changes, then the goggles' events (briefly, so a bridge launch gets the link)
        let mut changed = false;
        let mut failed: Option<String> = None;
        loop {
            match rx.try_recv() {
                Ok(Cmd::Set { module, key, value }) => {
                    let r = lock().as_mut().map(|l| l.set_config(&module, &[(key.clone(), value)], true));
                    match r {
                        Some(Ok(_)) => changed = true,
                        Some(Err(e)) => report(Err(format!("the goggles did not take {module} {key}: {e}"))),
                        None => {}
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return,
            }
        }
        {
            let mut guard = lock();
            if let Some(link) = guard.as_mut() {
                match link.poll_event(Duration::from_millis(200)) {
                    Ok(Some(e)) => changed |= !matches!(e.body, Some(pb::event::Body::Closing(_))),
                    Ok(None) => {}
                    Err(e) => failed = Some(e),
                }
                changed |= heard.elapsed() > Duration::from_secs(5);
                if failed.is_none() && changed {
                    heard = Instant::now();
                    match snapshot(link, &mut schemas) {
                        Ok(g) => {
                            drop(guard);
                            publish(g);
                        }
                        Err(e) => failed = Some(e),
                    }
                }
            }
        }
        if let Some(e) = failed {
            // the session ended (or the goggles went away): look again
            *lock() = None;
            publish(wire::Goggles {
                message: format!("the control link ended ({e}); the tinkerglass mods are not running"),
                ..Default::default()
            });
        } else {
            thread::sleep(Duration::from_millis(20));
        }
    }
}
