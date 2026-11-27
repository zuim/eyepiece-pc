//! Simple main view (default): Start/Stop, what to show, a quality preset, head tracking,
//! preview, a one-line status and the notice banner. Everything else is in the full view,
//! which this only switches to; both edit the same settings.

use super::{fmt_ms, App};
use crate::engine::Phase;
use crate::settings::{self, PRESETS};
use eframe::egui::{self, Color32, RichText};
use std::sync::atomic::Ordering;

const WARN: Color32 = Color32::from_rgb(220, 180, 60);
const GOOD: Color32 = Color32::from_rgb(80, 200, 120);
const BAD: Color32 = Color32::from_rgb(230, 90, 80);

/// A choice in the "Show" picker: label, (source, output), why it is unavailable.
struct Screen {
    label: String,
    source: &'static str,
    output: Option<String>,
    missing: Option<String>,
}

impl App {
    fn screens(&self) -> Vec<Screen> {
        let mut v = vec![];
        if cfg!(target_os = "linux") {
            let missing = (!self.kms_allowed)
                .then(|| "needs the capture permission: install + setcap (README, Permissions)".to_string());
            v.push(Screen { label: "First screen (KMS)".into(), source: "kms", output: None, missing: missing.clone() });
            for o in &self.outputs {
                v.push(Screen { label: format!("Screen {o} (KMS)"), source: "kms", output: Some(o.clone()), missing: missing.clone() });
            }
            v.push(Screen {
                label: "Screen or window… (PipeWire)".into(),
                source: "pipewire",
                output: None,
                missing: (!cfg!(feature = "pipewire")).then(|| "not in this build".into()),
            });
            if cfg!(feature = "x11") && std::env::var_os("DISPLAY").is_some() {
                v.push(Screen { label: "X11 screen".into(), source: "x11", output: None, missing: None });
            }
        }
        if cfg!(windows) {
            v.push(Screen { label: "Main screen".into(), source: "dxgi", output: None, missing: None });
        }
        if cfg!(target_os = "macos") {
            v.push(Screen { label: "Main screen (ScreenCaptureKit)".into(), source: "screencapturekit", output: None, missing: None });
        }
        v.push(Screen { label: "Test pattern".into(), source: "test", output: None, missing: None });
        v
    }

    fn screen_label(&self) -> String {
        let a = &self.saved.args;
        match a.source.as_str() {
            "kms" => match &a.output {
                Some(o) => format!("Screen {o} (KMS)"),
                None => "First screen (KMS)".into(),
            },
            "pipewire" => "Screen or window… (PipeWire)".into(),
            "x11" => "X11 screen".into(),
            "dxgi" => "Main screen".into(),
            "screencapturekit" => "Main screen (ScreenCaptureKit)".into(),
            "test" => "Test pattern".into(),
            "file" => "Video file (full view)".into(),
            s => s.to_string(),
        }
    }

    fn status_line(&self) -> (String, Color32) {
        let Some(e) = &self.engine else { return ("Not running".into(), Color32::GRAY) };
        let mut parts = vec![];
        let col = match e.phase() {
            Phase::Preparing => {
                parts.push("Starting…".to_string());
                WARN
            }
            Phase::Connected => {
                parts.push("Connected".into());
                GOOD
            }
            Phase::Disconnected => {
                parts.push("Disconnected".into());
                WARN
            }
            Phase::Stopping => {
                parts.push("Stopping…".into());
                WARN
            }
            Phase::Ended(Ok(())) => {
                parts.push("Stopped".into());
                Color32::GRAY
            }
            Phase::Ended(Err(err)) => return (format!("Error: {err}"), BAD),
        };
        if self.running() {
            if let Some(l) = e.shared.data_link.lock().unwrap().clone() {
                parts.push(l);
            }
            if let Some(s) = self.last_snapshot() {
                if e.args.no_video {
                    parts.push(format!("head tracking {:.0} Hz", s.pose_hz));
                } else {
                    parts.push(format!("{:.0} fps", s.sent_fps));
                    if s.lat_p50_ms.is_finite() {
                        parts.push(format!("latency {} ms", fmt_ms(s.lat_p50_ms).trim()));
                    }
                }
            }
        }
        (parts.join("  ·  "), col)
    }

    pub(super) fn simple_view(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let w = ui.available_width().min(480.0);
        let pad = ((ui.available_width() - w) / 2.0).max(0.0);
        ui.horizontal_top(|ui| {
            ui.add_space(pad);
            ui.vertical(|ui| {
                ui.set_width(w);
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("Eyepiece").size(26.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Full view").on_hover_text("all options, latency, compare, log").clicked() {
                            self.saved.ui.simple = false;
                        }
                    });
                });
                ui.add_space(10.0);

                let running = self.running();
                let label = if running { "Stop" } else { "Start" };
                if ui.add_sized([w, 60.0], egui::Button::new(RichText::new(label).size(24.0))).clicked() {
                    if running {
                        self.stop();
                    } else {
                        self.start(&ctx);
                    }
                }
                ui.add_space(6.0);
                let (st, col) = self.status_line();
                ui.vertical_centered(|ui| ui.label(RichText::new(st).color(col)));
                if let Some(n) = self.engine.as_ref().and_then(|e| e.shared.notice.lock().unwrap().clone()) {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(w - 16.0);
                        ui.label(RichText::new(n).color(WARN));
                    });
                }
                if running && self.engine.as_ref().is_some_and(|e| settings::command_line(&e.args) != settings::command_line(&self.saved.args)) {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Changed settings apply on restart.").color(WARN));
                        if ui.button("Restart").clicked() {
                            self.start(&ctx);
                        }
                    });
                }
                self.undo_banner(ui);
                ui.add_space(14.0);

                egui::Grid::new("simple_grid").num_columns(2).spacing([14.0, 12.0]).show(ui, |ui| {
                    // ---- what to show ----
                    ui.label(RichText::new("Show").strong());
                    let screens = self.screens();
                    let mut pick: Option<(&'static str, Option<String>)> = None;
                    egui::ComboBox::from_id_salt("simple_screen").selected_text(self.screen_label()).width(w - 140.0).show_ui(ui, |ui| {
                        for sc in &screens {
                            let sel = self.saved.args.source == sc.source && (sc.source != "kms" || self.saved.args.output == sc.output);
                            let r = ui.add_enabled(sc.missing.is_none(), egui::Button::selectable(sel, sc.label.as_str()));
                            if r.clicked() {
                                pick = Some((sc.source, sc.output.clone()));
                            }
                            if let Some(m) = &sc.missing {
                                r.on_disabled_hover_text(m.as_str());
                            }
                        }
                    });
                    if let Some((src, out)) = pick {
                        self.saved.args.source = src.into();
                        if src == "kms" {
                            self.saved.args.output = out;
                        }
                    }
                    ui.end_row();
                    if self.saved.args.source == "kms" && !self.kms_allowed && cfg!(target_os = "linux") {
                        ui.label("");
                        ui.label(RichText::new("KMS capture needs the capture permission (README, Permissions). PipeWire works without it.").small().color(WARN));
                        ui.end_row();
                    }

                    // ---- quality ----
                    ui.label(RichText::new("Quality").strong());
                    ui.horizontal_wrapped(|ui| {
                        let cur = settings::preset_of(&self.saved.args);
                        for (i, (name, hint, codec, rate)) in PRESETS.iter().enumerate() {
                            if ui.selectable_label(cur == Some(i), *name).on_hover_text(*hint).clicked() {
                                self.saved.args.codec = codec.to_string();
                                self.saved.args.bitrate = rate.to_string();
                            }
                        }
                        if cur.is_none() {
                            ui.label(RichText::new(format!("Custom ({} {})", self.saved.args.codec, self.saved.args.bitrate)).weak())
                                .on_hover_text("set in the full view");
                        }
                    });
                    ui.end_row();

                    // ---- head tracking ----
                    ui.label(RichText::new("Head tracking").strong());
                    ui.horizontal(|ui| {
                        let implied = self.saved.args.no_video || self.saved.args.print_pose;
                        let mut ht = self.saved.args.head_tracking || implied;
                        let r = ui.add_enabled(!implied, egui::Checkbox::new(&mut ht, "to opentrack"));
                        if r.on_disabled_hover_text("on: implied by head-tracking-only mode").changed() {
                            self.saved.args.head_tracking = ht;
                            if let Some(e) = &self.engine {
                                e.ctl.set_head_tracking(ht);
                            }
                        }
                        if ui.add_enabled(running && (ht || implied), egui::Button::new("Recenter")).clicked() {
                            if let Some(e) = &self.engine {
                                e.ctl.recenter.store(true, Ordering::Relaxed);
                            }
                        }
                    });
                    ui.end_row();
                    if self.head_tracking_on() {
                        // small version of the full view's 3D pose (dimmed without data)
                        let pose = self
                            .engine
                            .as_ref()
                            .filter(|_| running)
                            .and_then(|e| e.shared.link.lock().unwrap().clone())
                            .and_then(|l| *l.shared.last_pose.lock().unwrap());
                        ui.label("");
                        ui.horizontal_top(|ui| {
                            let (r, _) = ui.allocate_exact_size(egui::vec2(180.0, 126.0), egui::Sense::hover());
                            crate::pose::draw(ui, r, pose.map(|p| (p.yaw, p.pitch, p.roll)));
                            ui.vertical(|ui| {
                                ui.add_space(30.0);
                                for (n, v) in [("yaw  ", pose.map(|p| p.yaw)), ("pitch", pose.map(|p| p.pitch)), ("roll ", pose.map(|p| p.roll))] {
                                    let v = v.map(|v| format!("{v:+6.1}°")).unwrap_or_else(|| "     –".into());
                                    ui.label(RichText::new(format!("{n} {v}")).monospace().weak());
                                }
                            });
                        });
                        ui.end_row();
                    }

                    // ---- data link ----
                    ui.label(RichText::new("Connection").strong());
                    let mut on = settings::usb_link(&self.saved.args);
                    if ui.checkbox(&mut on, "USB (auto)").on_hover_text(settings::USB_LINK_HINT).changed() {
                        settings::set_usb_link(&mut self.saved.args, on);
                    }
                    ui.end_row();

                    // ---- preview ----
                    ui.label(RichText::new("Preview").strong());
                    ui.checkbox(&mut self.saved.ui.preview, "show what the goggles get");
                    ui.end_row();
                });
                ui.add_space(10.0);
                if self.saved.ui.preview && running {
                    self.preview_view(ui, true);
                }
                ui.add_space(16.0);
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Settings are saved automatically.").small().weak());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("Reset to defaults").on_hover_text("all options and preview settings; pins are kept; Undo for 20 s").clicked() {
                            self.reset_defaults();
                        }
                    });
                });
            });
        });
    }
}
