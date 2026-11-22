//! `adb` (platform-tools) helpers: forward, deploy, stop — same behaviour as eyepiece_stream.py.

use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// Where `--deploy` pushes a local bridge.
pub const DEVICE_DIR: &str = "/data/local/tmp/eyepiece";
/// Bridges installed on the goggles (started from their menu), tried in this order: eyepiece's
/// PC Display mod, then natartos' Eyepiece mod (legacy; remove at eyepiece-pc 1.0).
pub const INSTALLED_BRIDGES: [&str; 2] = [
    "/vendor/eyepiece/current/mods/pcdisplay/bin/pcdisplay_bridge",
    "/vendor/natartos/apps/eyepiece/eyepiece_bridge",
];
/// Process names of a running bridge (eyepiece, natartos/pushed), for `pidof`.
const BRIDGE_PROCS: &str = "pcdisplay_bridge eyepiece_bridge";
/// Present when eyepiece is installed.
const EYEPIECE_CURRENT: &str = "/vendor/eyepiece/current";
/// eyepiece's run root: with it a bridge started here finds core.display's GUI-hide lease
/// (`run/display/`). Its own temp files stay in /data/local/tmp (no `EP_RUN_DIR`), which the
/// ADB shell can write in every SELinux mode; the bridge from the menu is stopped first anyway.
const EYEPIECE_ENV: &str = "EP_RUN_ROOT=/data/vendor/eyepiece/run";
/// The log of a bridge started here (`-l`; the ADB shell can write it in every SELinux mode).
pub const DEVICE_LOG: &str = "/data/local/tmp/eyepiece.log";

/// `p1 p2 …` for a shell `for` loop: the pushed bridge, then the installed ones.
fn bridge_candidates() -> String {
    std::iter::once(format!("{DEVICE_DIR}/eyepiece_bridge"))
        .chain(INSTALLED_BRIDGES.iter().map(|s| s.to_string()))
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Debug)]
pub struct Adb {
    pub exe: String,
    pub serial: Option<String>,
}

impl Adb {
    pub fn new(exe: impl Into<String>, serial: Option<String>) -> Adb {
        Adb { exe: exe.into(), serial }
    }

    fn cmd(&self) -> Command {
        let mut c = Command::new(&self.exe);
        if let Some(s) = &self.serial {
            c.arg("-s").arg(s);
        }
        c
    }

    /// Run `adb <args>`; Err with stderr on failure (when `check`).
    pub fn run(&self, args: &[&str], check: bool) -> Result<String, String> {
        let out = self
            .cmd()
            .args(args)
            .output()
            .map_err(|e| format!("cannot run {}: {e} (is adb installed? --adb PATH)", self.exe))?;
        let so = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if check && !out.status.success() {
            let se = String::from_utf8_lossy(&out.stderr).trim().to_string();
            return Err(format!("adb {} failed: {}", args.join(" "), if se.is_empty() { so } else { se }));
        }
        Ok(so)
    }

    pub fn shell(&self, cmd: &str, check: bool) -> Result<String, String> {
        self.run(&["shell", cmd], check)
    }

    pub fn forward(&self, local: u16, remote: u16) -> Result<(), String> {
        self.run(&["forward", &format!("tcp:{local}"), &format!("tcp:{remote}")], true).map(|_| ())
    }

    pub fn stop_bridge(&self) {
        if let Ok(pid) = self.shell(&format!("pidof {BRIDGE_PROCS}"), false) {
            if !pid.is_empty() {
                let _ = self.shell(&format!("kill {pid}"), false);
                // the bridge's helper sends the MCU head-attitude stop on SIGTERM
                std::thread::sleep(Duration::from_millis(2500));
            }
        }
        self.mcu_stop_safety();
    }

    /// Belt and braces for the MCU head-attitude stream: with no bridge running, send the
    /// stop (`<bridge> -Z`). Covers the case the bridge can't handle itself (bridge and
    /// helper both SIGKILLed): the stream outlives them and logs a router error per packet
    /// on the goggles. Harmless when the stream is already off. Returns the bridge's line.
    pub fn mcu_stop_safety(&self) -> Option<String> {
        if self.bridge_running() {
            return None;
        }
        let out = self
            .shell(
                &format!(
                    "for b in {}; do [ -x $b ] && {{ $b -Z 2>&1 | grep 'MCU attitude'; break; }}; done",
                    bridge_candidates()
                ),
                false,
            )
            .unwrap_or_default();
        let out = out.trim();
        // drop the bridge's own "[…] " log prefix
        let out = match out.strip_prefix('[').and_then(|r| r.split_once("] ")) {
            Some((_, rest)) => rest,
            None => out,
        };
        (!out.is_empty()).then(|| format!("goggles: {out}"))
    }

    /// True if a bridge process runs on the goggles.
    pub fn bridge_running(&self) -> bool {
        !self.shell(&format!("pidof {BRIDGE_PROCS}"), false).unwrap_or_default().trim().is_empty()
    }

    /// Start the bridge on the goggles with `-1` (exit with the PC session: a bridge left
    /// running keeps the goggles black) plus `extra_args`. `bridge` = a local binary to push
    /// first; `None` = use one already on the goggles (pushed earlier, or the one installed
    /// with eyepiece's PC Display mod or natartos' Eyepiece mod).
    /// `usb_net`: ask the bridge to switch the goggles' USB to network + adb (`-U rndis`; only
    /// bridges that know the option get it).
    pub fn deploy(&self, bridge: Option<&Path>, extra_args: &str, usb_net: bool, log: &dyn Fn(String)) -> Result<(), String> {
        if let Err(e) = self.run(&["get-state"], true) {
            return Err(format!(
                "ADB could not select the goggles: {e}\n\
                 Connect them by USB, switch them on and enable ADB (eyepiece: Connect in the web \
                 app, or `ep connect`). If `adb devices` \
                 lists more than one device, set the goggles' serial in Advanced → adb serial; \
                 on Windows, make sure Advanced → adb executable points to the same adb.exe."
            ));
        }
        let exe = match bridge {
            Some(bridge) => {
                if !bridge.is_file() {
                    return Err(format!("{} not found (--bridge)", bridge.display()));
                }
                self.shell(&format!("mkdir -p {DEVICE_DIR}"), true)?;
                self.run(&["push", &bridge.to_string_lossy(), &format!("{DEVICE_DIR}/eyepiece_bridge")], true)?;
                self.shell(&format!("chmod 755 {DEVICE_DIR}/eyepiece_bridge"), true)?;
                format!("{DEVICE_DIR}/eyepiece_bridge")
            }
            None => {
                let p = self
                    .shell(
                        &format!("for p in {}; do [ -x $p ] && echo $p && break; done", bridge_candidates()),
                        true,
                    )?
                    .trim()
                    .to_string();
                if p.is_empty() {
                    return Err("no bridge on the PC (eyepiece_bridge next to this program, or --bridge PATH) \
                                and none on the goggles: install eyepiece with the PC Display mod"
                        .into());
                }
                log(format!("no local bridge: starting the one on the goggles ({p})"));
                p
            }
        };
        let mut args = extra_args.to_string();
        if usb_net && !args.contains("-U") {
            let help = self.shell(&format!("{exe} -h 2>&1; true"), false).unwrap_or_default();
            if help.contains("-U adb|rndis") {
                args = format!("-U rndis {args}");
            } else {
                log("this bridge has no USB network support (an older build): adb only".into());
            }
        }
        let eyepiece = self.shell(&format!("[ -d {EYEPIECE_CURRENT} ] && echo y; true"), false).unwrap_or_default();
        let env = if eyepiece.trim() == "y" { EYEPIECE_ENV } else { "" };
        self.stop_bridge();
        // a fresh log: the "listening on" wait below must not see an older run's line
        self.shell(&format!("rm -f {DEVICE_LOG}"), false)?;
        self.shell(&format!("{env} nohup {exe} -1 -b -l {DEVICE_LOG} {args} >/dev/null 2>&1 < /dev/null &"), true)?;
        // adb forward accepts a TCP connect even when nothing listens on the goggles yet (and
        // closes it right away), so a connect is no proof: wait for the bridge's own log line.
        // Startup takes >1 s without fb_gui_hide (overlay ack wait), and a bridge that switches
        // USB to RNDIS + adb drops adb for a few seconds (sometimes until `adb reconnect`).
        let t0 = std::time::Instant::now();
        let mut last_reconnect = t0;
        let mut offline_logged = false;
        loop {
            match self.shell(&format!("cat {DEVICE_LOG} 2>/dev/null; true"), true) {
                Ok(l) if l.contains("listening on") => break,
                Ok(l) if l.contains("shutting down") || l.contains("giving up") => {
                    return Err(format!("the bridge exited during startup:\n{l}"));
                }
                Ok(_) => {}
                Err(_) => {
                    if !offline_logged {
                        log("goggles' USB is re-enumerating (bridge switches to USB network) …".into());
                        offline_logged = true;
                    }
                    if last_reconnect.elapsed() >= Duration::from_secs(3) {
                        let _ = self.run(&["reconnect", "offline"], false);
                        last_reconnect = std::time::Instant::now();
                    }
                }
            }
            if t0.elapsed() >= Duration::from_secs(25) {
                let tail = self.shell(&format!("tail -n 20 {DEVICE_LOG}"), false).unwrap_or_default();
                return Err(format!("the bridge is not listening:\n{tail}"));
            }
            std::thread::sleep(Duration::from_millis(300));
        }
        let pid = self.shell(&format!("pidof {BRIDGE_PROCS}"), false).unwrap_or_default();
        let started = self.shell(&format!("cat {DEVICE_LOG} 2>/dev/null; true"), false).unwrap_or_default();
        for l in started.lines().filter(|l| l.contains("rndis:")) {
            log(format!("goggles: {}", l.split_once(' ').map(|x| x.1).unwrap_or(l)));
        }
        log(format!("bridge running on goggles ({exe}, pid {pid})"));
        // the bridge's own word on the GUI plane (it hides it through the display service in
        // dji_glasses: eyepiece's core.display, natartos' overlay)
        if started.contains("overlay has no fb_gui_hide") || started.contains("framebuffer unavailable") {
            log("WARNING: the goggles' menu could not be hidden for the stream (no answer from the display \
                 service in the goggles' UI), so the picture may stay behind it. eyepiece: start the mods \
                 (web app, or `ep start`) after `ep connect` (it makes the session permissive, which a bridge \
                 started over ADB needs); natartos: `toolya natartos start`"
                .into());
        }
        Ok(())
    }
}
