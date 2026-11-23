//! `adb` (platform-tools) helpers: forward, deploy, stop — same behaviour as eyepiece_stream.py.

use std::process::Command;
use std::time::Duration;

/// The bridge of eyepiece's PC Display mod: the only one eyepiece-pc starts.
pub const BRIDGE: &str = "/vendor/eyepiece/current/mods/pcdisplay/bin/pcdisplay_bridge";
/// Its process name, for `pidof`.
const BRIDGE_PROC: &str = "pcdisplay_bridge";
/// The same environment core.menu gives it: its run dir (lock, log, DHCP files; the ADB shell
/// may write it under enforcing too) and eyepiece's run root (core.display's GUI-hide lease).
const BRIDGE_ENV: &str = "EP_RUN_DIR=/data/vendor/eyepiece/run/pcdisplay EP_RUN_ROOT=/data/vendor/eyepiece/run";
/// Its log (the default of a bridge with `EP_RUN_DIR`; the menu's bridge writes it too).
pub const DEVICE_LOG: &str = "/data/vendor/eyepiece/run/pcdisplay/bridge.log";

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
        // SIGTERM: the bridge leaves the hub (which stops the MCU stream) and hands the display
        // back; wait until it is gone (usually well under a second), at most 3 s
        let _ = self.shell(
            &format!(
                "p=$(pidof {BRIDGE_PROC}) && kill $p && \
                 i=0; while pidof {BRIDGE_PROC} >/dev/null && [ $i -lt 30 ]; do sleep 0.1; i=$((i+1)); done; true"
            ),
            false,
        );
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
                &format!("[ -x {BRIDGE} ] && {BRIDGE} -Z 2>&1 | grep 'MCU attitude'; true"),
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
        !self.shell(&format!("pidof {BRIDGE_PROC}"), false).unwrap_or_default().trim().is_empty()
    }

    /// Start eyepiece's bridge on the goggles with `-1` (exit with the PC session: a bridge left
    /// running keeps the goggles black) plus `extra_args`. A bridge already running (from the
    /// menu) hands over to it by itself.
    /// `usb_net`: ask the bridge to switch the goggles' USB to network + adb (`-U rndis`; only
    /// bridges that know the option get it).
    pub fn deploy(&self, extra_args: &str, usb_net: bool, log: &dyn Fn(String)) -> Result<(), String> {
        if let Err(e) = self.run(&["get-state"], true) {
            return Err(format!(
                "ADB could not select the goggles: {e}\n\
                 Connect them by USB, switch them on and enable ADB (eyepiece: Connect in the web \
                 app, or `ep connect`). If `adb devices` \
                 lists more than one device, set the goggles' serial in Advanced → adb serial; \
                 on Windows, make sure Advanced → adb executable points to the same adb.exe."
            ));
        }
        let exe = BRIDGE;
        let mut args = extra_args.to_string();
        let help = self.shell(&format!("[ -x {exe} ] && {exe} -h 2>&1; true"), false).unwrap_or_default();
        if help.trim().is_empty() {
            return Err("PC Display is not installed on the goggles: install eyepiece with the PC Display mod \
                        (web app, or `ep install`)"
                .into());
        }
        if usb_net && !args.contains("-U") {
            if help.contains("-U adb|rndis") {
                args = format!("-U rndis {args}");
            } else {
                log("this bridge has no USB network support (an older build): adb only".into());
            }
        }
        // a fresh log (the older one kept as .1): the "listening on" wait below must not see an
        // older run's line
        self.shell(&format!("mv -f {DEVICE_LOG} {DEVICE_LOG}.1 2>/dev/null; true"), false)?;
        self.shell(&format!("{BRIDGE_ENV} nohup {exe} -1 -b -l {DEVICE_LOG} {args} >/dev/null 2>&1 < /dev/null &"), true)?;
        // adb forward accepts a TCP connect even when nothing listens on the goggles yet (and
        // closes it right away), so a connect is no proof: wait for the bridge's own log line.
        // A bridge that switches USB to RNDIS + adb drops adb for a few seconds (sometimes
        // until `adb reconnect`).
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
        let pid = self.shell(&format!("pidof {BRIDGE_PROC}"), false).unwrap_or_default();
        let started = self.shell(&format!("cat {DEVICE_LOG} 2>/dev/null; true"), false).unwrap_or_default();
        for l in started.lines().filter(|l| l.contains("rndis:")) {
            log(format!("goggles: {}", l.split_once(' ').map(|x| x.1).unwrap_or(l)));
        }
        log(format!("bridge running on goggles ({exe}, pid {pid})"));
        if started.contains("core.display did not hide the GUI") {
            log("WARNING: the goggles' mods are not running, so the bridge can't show a picture \
                 (head tracking only): start them (web app, or `ep start`), then connect again"
                .into());
        }
        Ok(())
    }
}
