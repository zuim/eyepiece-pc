//! `adb` (platform-tools) helpers: forward, deploy, stop.

use std::process::Command;
use std::time::Duration;

/// The bridge of tinkerglass's PC Display mod: the only one eyepiece-pc starts.
pub const BRIDGE: &str = "/vendor/tinkerglass/current/mods/pcdisplay/bin/pcdisplay_bridge";
/// Its process name, for `pidof`.
const BRIDGE_PROC: &str = "pcdisplay_bridge";
/// core.menu's launch request (tinkerglass contract `files.launch_req`): the bridge starts as from
/// the goggles' menu, with the menu's environment and in the UI's SELinux domain, so it may use
/// the hub (head tracking) also under enforcing, unlike a bridge started from the ADB shell.
const LAUNCH_REQ: &str = "/data/vendor/tinkerglass/run/menu/launch.req";
/// Its log (the default of a bridge with `TG_RUN_DIR`; the menu's bridge writes it too).
pub const DEVICE_LOG: &str = "/data/vendor/tinkerglass/run/pcdisplay/bridge.log";

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

    /// Start tinkerglass's bridge on the goggles through core.menu, with `-1` (exit with the PC
    /// session: a bridge left running keeps the goggles black) plus `extra_args`. A bridge
    /// already running (from the menu) hands over to it by itself. Needs the mods running.
    pub fn deploy(&self, extra_args: &str, log: &dyn Fn(String)) -> Result<(), String> {
        if let Err(e) = self.run(&["get-state"], true) {
            return Err(format!(
                "ADB could not select the goggles: {e}\n\
                 Connect them by USB, switch them on and enable ADB (tinkerglass: Connect in the web \
                 app, or `tg connect`). If `adb devices` \
                 lists more than one device, set the goggles' serial in Advanced → adb serial; \
                 on Windows, make sure Advanced → adb executable points to the same adb.exe."
            ));
        }
        let exe = BRIDGE;
        let help = self.shell(&format!("[ -x {exe} ] && {exe} -h 2>&1; true"), false).unwrap_or_default();
        if help.trim().is_empty() {
            return Err("PC Display is not installed on the goggles: install tinkerglass with its PC Display \
                        mod, then connect again"
                .into());
        }
        // a fresh log (the older one kept as .1): the "listening on" wait below must not see an
        // older run's line
        self.shell(&format!("mv -f {DEVICE_LOG} {DEVICE_LOG}.1 2>/dev/null; true"), false)?;
        let line = launch_line(extra_args);
        let taken = self.shell(
            &format!(
                "printf '%s\\n' {} > {LAUNCH_REQ}.tmp && mv -f {LAUNCH_REQ}.tmp {LAUNCH_REQ} && \
                 i=0; while [ -e {LAUNCH_REQ} ] && [ $i -lt 20 ]; do sleep 0.1; i=$((i+1)); done; \
                 [ -e {LAUNCH_REQ} ] && rm -f {LAUNCH_REQ} && echo none; true",
                shell_quote(&line)
            ),
            true,
        )?;
        if taken.contains("none") {
            return Err("the tinkerglass mods are not running on the goggles: start them (web app, or `tg start`), \
                        then connect again"
                .into());
        }
        // adb forward accepts a TCP connect even when nothing listens on the goggles yet (and
        // closes it right away), so a connect is no proof: wait for the bridge's own log line.
        // adb may drop for a moment while the goggles re-enumerate USB (sometimes until
        // `adb reconnect`).
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
                        log("goggles' USB is re-enumerating …".into());
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
        log(format!("bridge running on goggles ({exe}, pid {pid})"));
        if started.contains("core.display did not hide the GUI") {
            log("WARNING: the goggles' mods are not running, so the bridge can't show a picture \
                 (head tracking only): start them (web app, or `tg start`), then connect again"
                .into());
        }
        Ok(())
    }
}

/// The launch request core.menu reads: the mod id, then the bridge's arguments. `-1` makes the
/// bridge exit with the PC session; `-l` puts its log where [`Adb::deploy`] waits for it.
pub fn launch_line(extra_args: &str) -> String {
    format!("pcdisplay -1 -l {DEVICE_LOG} {}", extra_args.trim()).trim_end().to_string()
}

/// `s` as one single-quoted word for the device's shell (the request is written with `printf`).
fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_line_has_exit_with_session_and_log() {
        assert_eq!(
            launch_line(""),
            "pcdisplay -1 -l /data/vendor/tinkerglass/run/pcdisplay/bridge.log"
        );
        assert_eq!(
            launch_line(" -M -F 4 "),
            "pcdisplay -1 -l /data/vendor/tinkerglass/run/pcdisplay/bridge.log -M -F 4"
        );
    }

    #[test]
    fn launch_line_is_shell_quoted() {
        assert_eq!(shell_quote("a b"), "'a b'");
        assert_eq!(shell_quote("-O 'plane:1'"), "'-O '\''plane:1'\'''");
    }
}
