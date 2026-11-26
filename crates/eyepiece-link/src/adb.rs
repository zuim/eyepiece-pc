//! `adb` (platform-tools) helpers: forward, deploy, stop — same behaviour as eyepiece_stream.py.

use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// Where `--deploy` pushes the bridge.
pub const DEVICE_DIR: &str = "/data/local/tmp/eyepiece";
/// The bridge installed by the natartos Eyepiece mod (started from the goggles' menu).
pub const MENU_BRIDGE: &str = "/vendor/natartos/apps/eyepiece/eyepiece_bridge";
/// The bridge's log on the goggles.
pub const DEVICE_LOG: &str = "/data/local/tmp/eyepiece.log";

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
        if let Ok(pid) = self.shell("pidof eyepiece_bridge", false) {
            if !pid.is_empty() {
                let _ = self.shell(&format!("kill {pid}"), false);
                // the bridge's helper sends the MCU head-attitude stop on SIGTERM
                std::thread::sleep(Duration::from_millis(2500));
            }
        }
        self.mcu_stop_safety();
    }

    /// Belt and braces for the MCU head-attitude stream: with no bridge running, send the
    /// stop (`eyepiece_bridge -Z`). Covers the case the bridge can't handle itself (bridge and
    /// helper both SIGKILLed): the stream outlives them and logs a router error per packet
    /// on the goggles. Harmless when the stream is already off. Returns the bridge's line.
    pub fn mcu_stop_safety(&self) -> Option<String> {
        if self.bridge_running() {
            return None;
        }
        let out = self
            .shell(
                &format!(
                    "for b in {DEVICE_DIR}/eyepiece_bridge {MENU_BRIDGE}; do \
                     [ -x $b ] && {{ $b -Z 2>&1 | grep 'MCU attitude'; break; }}; done"
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

    /// Is the natartos overlay hook loaded into dji_glasses? Checks the process maps first
    /// (reliable), then natartos.log (may be truncated). True only on positive evidence.
    pub fn overlay_loaded(&self) -> bool {
        let out = self
            .shell(
                "for p in $(pidof dji_glasses); do grep -q hook_overlay /proc/$p/maps 2>/dev/null && \
                 { echo maps; exit 0; }; done; \
                 grep -q 'hook_overlay loaded' /data/local/tmp/natartos.log 2>/dev/null && echo log",
                false,
            )
            .unwrap_or_default();
        !out.trim().is_empty()
    }

    /// True if an eyepiece_bridge process runs on the goggles.
    pub fn bridge_running(&self) -> bool {
        !self.shell("pidof eyepiece_bridge", false).unwrap_or_default().trim().is_empty()
    }

    /// Start the bridge on the goggles with `-1` (exit with the PC session: a bridge left
    /// running keeps the goggles black) plus `extra_args`. `bridge` = a local binary to push
    /// first; `None` = use one already on the goggles (pushed earlier, or the one natartos
    /// installed with the Eyepiece mod).
    /// Ask supported bridges to enable USB bulk. Older bridges keep their existing behavior;
    /// the caller decides whether that is acceptable for the selected mode.
    pub fn deploy(
        &self,
        bridge: Option<&Path>,
        extra_args: &str,
        usb_bulk: bool,
        log: &dyn Fn(String),
    ) -> Result<(), String> {
        if let Err(e) = self.run(&["get-state"], true) {
            return Err(format!(
                "ADB could not select the goggles: {e}\n\
                 Connect them by USB, switch them on and enable ADB (toolya adb). If `adb devices` \
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
                        &format!(
                            "for p in {DEVICE_DIR}/eyepiece_bridge {MENU_BRIDGE}; do [ -x $p ] && echo $p && break; done"
                        ),
                        true,
                    )?
                    .trim()
                    .to_string();
                if p.is_empty() {
                    return Err("no eyepiece_bridge on the PC (next to this program, or --bridge PATH) \
                                and none on the goggles: install natartos with the Eyepiece mod, or \
                                download eyepiece_bridge from the natartos release"
                        .into());
                }
                log(format!("no local eyepiece_bridge: starting the one on the goggles ({p})"));
                p
            }
        };
        let mut args = extra_args.to_string();
        if usb_bulk && !args.split_whitespace().any(|arg| arg == "-B") {
            let help = self.shell(&format!("{exe} -h 2>&1; true"), false).unwrap_or_default();
            if help.contains("-B          add a dedicated FunctionFS bulk interface") {
                args = format!("-B {args}");
            } else {
                log("this eyepiece_bridge has no USB bulk support; auto mode will use adb forwarding".into());
            }
        }
        self.stop_bridge();
        // a fresh log: the "listening on" wait below must not see an older run's line
        self.shell(&format!("rm -f {DEVICE_LOG}"), false)?;
        self.shell(&format!("nohup {exe} -1 -b {args} >/dev/null 2>&1 < /dev/null &"), true)?;
        // adb forward accepts a TCP connect even when nothing listens on the goggles yet (and
        // closes it right away), so a connect is no proof: wait for the bridge's own log line.
        // Startup can take >1 s while the bridge waits for overlay acknowledgements, and
        // FunctionFS USB configuration changes can briefly disconnect adb.
        let t0 = std::time::Instant::now();
        let mut last_reconnect = t0;
        let mut offline_logged = false;
        loop {
            match self.shell(&format!("cat {DEVICE_LOG} 2>/dev/null; true"), true) {
                Ok(l) if l.contains("listening on") => break,
                Ok(l) if l.contains("shutting down") || l.contains("giving up") => {
                    return Err(format!("eyepiece_bridge exited during startup:\n{l}"));
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
                return Err(format!("eyepiece_bridge is not listening:\n{tail}"));
            }
            std::thread::sleep(Duration::from_millis(300));
        }
        let pid = self.shell("pidof eyepiece_bridge", false).unwrap_or_default();
        log(format!("eyepiece_bridge running on goggles (pid {pid})"));
        if !self.overlay_loaded() {
            log("WARNING: natartos overlay not detected (hook_overlay.so is not mapped in dji_glasses) — \
                 if the goggles stay black, start natartos (`toolya natartos start`); its overlay mod \
                 puts the stream on the display"
                .into());
        }
        Ok(())
    }
}
