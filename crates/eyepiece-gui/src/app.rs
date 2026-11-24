//! The window: main controls on the left (source, start/stop, head tracking, preview), tabs
//! on the right (overview with preview / pose / latency, compare pins, advanced options, log).

use crate::engine::{Engine, Phase};
use crate::preview::Knobs;
use crate::settings::{self, Pin, Saved, METRICS};
use crate::{plot, pose};

#[path = "simple.rs"]
mod simple;
use eframe::egui::{self, Color32, RichText, TextureHandle, TextureOptions};
use eyepiece_pc::cli::Args;
use eyepiece_pc::link::stats::Snapshot;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Overview,
    Compare,
    Advanced,
    Log,
}

pub struct Options {
    pub config: Option<PathBuf>,
    pub start: bool,
    pub exit_after: Option<f64>,
    pub screenshot: Option<PathBuf>,
    pub tab: Option<String>,
}

pub struct App {
    saved: Saved,
    config: Option<PathBuf>,
    /// settings as last written (autosave compares against it)
    saved_toml: String,
    /// first unsaved change (autosave 1 s after the last change)
    changed_at: Option<Instant>,
    /// state before "Reset to defaults", for Undo
    undo: Option<(Args, settings::UiPrefs, Instant)>,
    kms_allowed: bool,
    engine: Option<Engine>,
    /// args the running engine was started with differ from the edited ones
    knobs: Arc<Knobs>,
    tab: Tab,
    outputs: Vec<String>,
    outputs_err: Option<String>,
    tex: Option<TextureHandle>,
    tex_seq: u64,
    cmdline_edit: String,
    cmdline_err: Option<String>,
    status: Option<String>,
    opts: Options,
    t_open: Instant,
    screenshot_sent: bool,
    list_outputs_pending: bool,
}

/// Capture sources: (name, why it is not available in this build, if it isn't).
const SOURCES: &[(&str, Option<&str>)] = &[
    ("kms", if cfg!(target_os = "linux") { None } else { Some("Linux only") }),
    (
        "pipewire",
        if !cfg!(target_os = "linux") {
            Some("Linux only")
        } else if cfg!(feature = "pipewire") {
            None
        } else {
            Some("not in this build (feature pipewire)")
        },
    ),
    (
        "x11",
        if !cfg!(target_os = "linux") {
            Some("Linux only")
        } else if cfg!(feature = "x11") {
            None
        } else {
            Some("not in this build (feature x11)")
        },
    ),
    ("dxgi", if cfg!(windows) { None } else { Some("Windows only") }),
    ("screencapturekit", if cfg!(target_os = "macos") { None } else { Some("macOS only") }),
    ("test", None),
    ("file", None),
];
#[cfg(target_os = "macos")]
const ENCODERS: &[&str] = &["auto", "videotoolbox", "x264"];
#[cfg(not(target_os = "macos"))]
const ENCODERS: &[&str] = &["auto", "nvenc", "vaapi", "amf", "qsv", "x264"];
const BITRATES: &[&str] = &["10M", "15M", "20M", "30M", "40M", "50M"];

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, saved: Saved, opts: Options, status: Option<String>) -> App {
        let ctx = cc.egui_ctx.clone();
        ctx.global_style_mut(|s| {
            s.spacing.item_spacing = egui::vec2(8.0, 6.0);
            s.spacing.button_padding = egui::vec2(8.0, 4.0);
        });
        let knobs = Arc::new(Knobs {
            enabled: saved.ui.preview.into(),
            keyframes_only: saved.ui.preview_keyframes_only.into(),
            max_fps: saved.ui.preview_fps.into(),
            width: saved.ui.preview_width.into(),
        });
        let cmdline_edit = settings::command_line(&saved.args);
        let mut app = App {
            saved,
            config: opts.config.clone(),
            saved_toml: String::new(),
            changed_at: None,
            undo: None,
            kms_allowed: crate::caps::kms_allowed(),
            engine: None,
            knobs,
            tab: Tab::Overview,
            outputs: vec![],
            outputs_err: None,
            tex: None,
            tex_seq: 0,
            cmdline_edit,
            cmdline_err: None,
            status,
            opts,
            t_open: Instant::now(),
            screenshot_sent: false,
            list_outputs_pending: false,
        };
        app.saved_toml = toml::to_string(&app.saved).unwrap_or_default();
        app.tab = match app.opts.tab.as_deref() {
            Some("compare") => Tab::Compare,
            Some("advanced") => Tab::Advanced,
            Some("log") => Tab::Log,
            _ => Tab::Overview,
        };
        #[cfg(target_os = "linux")]
        app.refresh_outputs(); // for the output pickers (simple and full view)
        if app.opts.start {
            app.start(&ctx);
        }
        app
    }

    fn running(&self) -> bool {
        self.engine.as_ref().is_some_and(|e| !e.finished())
    }

    fn start(&mut self, ctx: &egui::Context) {
        if let Some(mut e) = self.engine.take() {
            e.stop();
            e.join();
        }
        let c = ctx.clone();
        let repaint: Arc<dyn Fn() + Send + Sync> = Arc::new(move || c.request_repaint());
        self.save_now(); // the settings of every run are kept
        let mut args = self.saved.args.clone();
        // an ffplay window only if asked for in Advanced; the GUI has its own preview
        args.list = false;
        self.engine = Some(Engine::start(args, self.knobs.clone(), repaint));
        self.tex_seq = 0;
    }

    fn stop(&mut self) {
        if let Some(e) = &self.engine {
            e.stop();
        }
    }

    #[cfg(target_os = "linux")]
    fn refresh_outputs(&mut self) {
        match eyepiece_pc::capture::kms::list(&self.saved.args.kms_device) {
            Ok(s) => {
                self.outputs = s
                    .lines()
                    .filter_map(|l| l.trim_start().strip_prefix("output "))
                    .filter_map(|l| l.split_whitespace().next().map(str::to_string))
                    .collect();
                self.outputs_err = None;
            }
            Err(e) => {
                self.outputs.clear();
                self.outputs_err = Some(e);
            }
        }
    }

    fn save_now(&mut self) {
        if let Some(p) = &self.config {
            if let Err(e) = self.saved.save(p) {
                self.status = Some(format!("saving settings failed: {e}"));
            }
        }
        self.saved_toml = toml::to_string(&self.saved).unwrap_or_default();
        self.changed_at = None;
    }

    fn last_snapshot(&self) -> Option<Snapshot> {
        let e = self.engine.as_ref()?;
        e.shared.history.lock().unwrap().back().map(|x| x.1.clone())
    }

    fn pin(&mut self) {
        let Some(e) = &self.engine else { return };
        let h = e.shared.history.lock().unwrap();
        let Some(&(t_last, _)) = h.back() else { return };
        let win = self.saved.ui.pin_window_s;
        let snaps: Vec<&Snapshot> = h.iter().filter(|(t, _)| t_last - t < win - 1e-3).map(|x| &x.1).collect();
        let metrics = settings::aggregate(&snaps);
        drop(h);
        let notes = e.shared.facts.lock().unwrap().join("\n");
        let n = self.saved.pins.len() + 1;
        self.saved.pins.push(Pin {
            name: format!("#{n} {}", e.args.encoder),
            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
            settings: settings::settings_summary(&e.args),
            command: settings::command_line(&e.args),
            notes,
            metrics,
            selected: true,
        });
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let before = settings::command_line(&self.saved.args);

        if self.saved.ui.simple {
            egui::CentralPanel::default().show(ui, |ui| {
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| self.simple_view(ui));
            });
        } else {
            self.full_view(ui);
        }
        self.after_frame(&ctx, &before);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_now();
        if let Some(mut e) = self.engine.take() {
            e.stop();
            e.join();
        }
    }
}

impl App {
    fn full_view(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top").show(ui, |ui| self.top_bar(ui));
        egui::Panel::left("main").default_size(310.0).resizable(true).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| self.main_panel(ui));
        });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                for (t, name) in [(Tab::Overview, "Overview"), (Tab::Compare, "Compare"), (Tab::Advanced, "Advanced"), (Tab::Log, "Log")] {
                    ui.selectable_value(&mut self.tab, t, name);
                }
            });
            ui.separator();
            match self.tab {
                Tab::Overview => {
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| self.overview(ui));
                }
                Tab::Compare => self.compare(ui),
                Tab::Advanced => {
                    egui::ScrollArea::vertical().show(ui, |ui| self.advanced(ui));
                }
                Tab::Log => self.log(ui),
            }
        });
    }

    fn after_frame(&mut self, ctx: &egui::Context, before: &str) {
        // live knobs->preview thread
        let u = &self.saved.ui;
        self.knobs.enabled.store(u.preview, Ordering::Relaxed);
        self.knobs.max_fps.store(u.preview_fps, Ordering::Relaxed);
        self.knobs.width.store(u.preview_width, Ordering::Relaxed);
        self.knobs.keyframes_only.store(u.preview_keyframes_only, Ordering::Relaxed);
        if before != settings::command_line(&self.saved.args) {
            self.cmdline_edit = settings::command_line(&self.saved.args);
        }
        // autosave: everything in the settings file (options, view, preview, pins), 1 s after
        // the last change; also on Start and on exit
        let now_toml = toml::to_string(&self.saved).unwrap_or_default();
        if now_toml != self.saved_toml {
            let t = *self.changed_at.get_or_insert_with(Instant::now);
            if t.elapsed() > Duration::from_secs(1) {
                self.save_now();
            } else {
                ctx.request_repaint_after(Duration::from_millis(300));
            }
        }
        if self.undo.as_ref().is_some_and(|u| u.2.elapsed() > Duration::from_secs(20)) {
            self.undo = None;
        }

        // repaint pacing: pose view ~60 Hz while tracking, otherwise on events + 4 Hz
        if self.running() {
            let tracking = self.engine.as_ref().is_some_and(|e| e.shared.link.lock().unwrap().is_some()) && self.head_tracking_on();
            ctx.request_repaint_after(Duration::from_millis(if tracking { 16 } else { 250 }));
        }

        if let Some(t) = self.opts.exit_after {
            if self.t_open.elapsed().as_secs_f64() >= t {
                if let (Some(p), false) = (self.opts.screenshot.clone(), self.screenshot_sent) {
                    self.screenshot_sent = true;
                    ctx.request_screenshot(move |img| {
                        if let Err(e) = write_ppm(&p, &img) {
                            eprintln!("[eyepiece-gui] screenshot: {e}");
                        }
                    });
                    ctx.request_repaint();
                } else {
                    self.stop();
                    if !self.running() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    } else {
                        ctx.request_repaint_after(Duration::from_millis(50));
                    }
                }
            } else {
                ctx.request_repaint_after(Duration::from_millis(100));
            }
        }
    }

    /// Options + preview settings back to the defaults; pins and the view mode are kept.
    /// Undo is offered for 20 s.
    fn reset_defaults(&mut self) {
        let d = Saved::default();
        let old_args = std::mem::replace(&mut self.saved.args, d.args);
        let simple = self.saved.ui.simple;
        let old_ui = std::mem::replace(&mut self.saved.ui, settings::UiPrefs { simple, ..d.ui });
        self.undo = Some((old_args, old_ui, Instant::now()));
    }

    fn undo_banner(&mut self, ui: &mut egui::Ui) {
        if self.undo.is_none() {
            return;
        }
        ui.horizontal(|ui| {
            ui.label(RichText::new("Settings reset to defaults (pins kept).").color(Color32::from_rgb(220, 180, 60)));
            if ui.button("Undo").clicked() {
                if let Some((a, u, _)) = self.undo.take() {
                    self.saved.args = a;
                    self.saved.ui = u;
                }
            }
        });
    }

    fn head_tracking_on(&self) -> bool {
        let a = &self.saved.args;
        a.head_tracking || a.no_video || a.print_pose
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Eyepiece");
            if ui.button("Simple view").on_hover_text("back to the simple main view").clicked() {
                self.saved.ui.simple = true;
            }
            ui.separator();
            let (txt, col) = match self.engine.as_ref().map(|e| e.phase()) {
                None => ("idle".to_string(), ui.visuals().weak_text_color()),
                Some(Phase::Preparing) => ("starting…".into(), Color32::from_rgb(220, 180, 60)),
                Some(Phase::Connected) => ("streaming".into(), Color32::from_rgb(80, 200, 120)),
                Some(Phase::Disconnected) => ("disconnected".into(), Color32::from_rgb(220, 180, 60)),
                Some(Phase::Stopping) => ("stopping…".into(), Color32::from_rgb(220, 180, 60)),
                Some(Phase::Ended(Ok(()))) => ("stopped".into(), ui.visuals().weak_text_color()),
                Some(Phase::Ended(Err(e))) => (format!("error: {e}"), Color32::from_rgb(230, 90, 80)),
            };
            if self.running() && self.saved.args.no_connect && matches!(self.engine.as_ref().map(|e| e.phase()), Some(Phase::Preparing)) {
                ui.label(RichText::new("encoding (no bridge)").color(Color32::from_rgb(80, 200, 120)));
            } else {
                ui.label(RichText::new(txt).color(col));
            }
            if let Some(s) = self.last_snapshot().filter(|_| self.running()) {
                ui.separator();
                if self.engine.as_ref().is_some_and(|e| e.args.no_video) {
                    ui.label(format!("head tracking only, pose {:.0} Hz", s.pose_hz));
                } else {
                    ui.label(format!("{:.0} fps  {:.1} Mbit/s  shown {} ms p50", s.sent_fps, s.mbps, fmt_ms(s.lat_p50_ms).trim()));
                }
            }
            if let Some(s) = &self.status {
                ui.separator();
                ui.label(RichText::new(s).color(Color32::from_rgb(220, 180, 60)));
            }
        });
    }

    fn main_panel(&mut self, ui: &mut egui::Ui) {
        let running = self.running();
        let ctx = ui.ctx().clone();
        ui.add_space(4.0);
        let label = if running { "Stop" } else { "Start" };
        let b = egui::Button::new(RichText::new(label).size(18.0)).min_size(egui::vec2(ui.available_width(), 36.0));
        if ui.add(b).clicked() {
            if running {
                self.stop();
            } else {
                self.start(&ctx);
            }
        }
        if running && self.engine.as_ref().is_some_and(|e| settings::command_line(&e.args) != settings::command_line(&self.saved.args)) {
            ui.horizontal(|ui| {
                ui.label(RichText::new("settings changed").color(Color32::from_rgb(220, 180, 60)));
                if ui.button("Restart").clicked() {
                    self.start(&ctx);
                }
            });
        }
        if let Some(n) = self.engine.as_ref().and_then(|e| e.shared.notice.lock().unwrap().clone()) {
            ui.add_space(4.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new(format!("⚠ {n}")).color(Color32::from_rgb(220, 180, 60)));
            });
        }
        self.undo_banner(ui);
        ui.add_space(6.0);

        let a = &mut self.saved.args;
        egui::Grid::new("main_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("Source");
            egui::ComboBox::from_id_salt("source").selected_text(a.source.as_str()).show_ui(ui, |ui| {
                for (s, missing) in SOURCES {
                    let r = ui.add_enabled(missing.is_none(), egui::Button::selectable(a.source == *s, *s));
                    if r.clicked() {
                        a.source = s.to_string();
                    }
                    if let Some(why) = missing {
                        r.on_disabled_hover_text(*why);
                    }
                }
            });
            ui.end_row();
            match a.source.as_str() {
                "kms" => {
                    ui.label("Output");
                    ui.horizontal(|ui| {
                        let sel = a.output.clone().unwrap_or_else(|| "first active".into());
                        egui::ComboBox::from_id_salt("output").selected_text(sel).show_ui(ui, |ui| {
                            ui.selectable_value(&mut a.output, None, "first active");
                            for o in &self.outputs {
                                ui.selectable_value(&mut a.output, Some(o.clone()), o);
                            }
                        });
                        #[cfg(target_os = "linux")]
                        if ui.small_button("List").on_hover_text("list the connected outputs (KMS)").clicked() {
                            self.list_outputs_pending = true;
                        }
                    });
                    ui.end_row();
                }
                "x11" => {
                    ui.label("Region");
                    opt_text(ui, &mut a.region, "whole screen (WxH+X+Y)");
                    ui.end_row();
                }
                "file" => {
                    ui.label("File");
                    opt_text(ui, &mut a.input, "media file (looped)");
                    ui.end_row();
                }
                "dxgi" => {
                    ui.label("Adapter,output");
                    ui.text_edit_singleline(&mut a.dxgi);
                    ui.end_row();
                }
                "screencapturekit" => {
                    ui.label("Display ID");
                    opt_num(ui, &mut a.display_id).on_hover_text("leave empty to use the first available display");
                    ui.end_row();
                }
                _ => {}
            }
            ui.label("Bridge");
            ui.horizontal(|ui| {
                ui.radio_value(&mut a.no_deploy, false, "start (default)");
                ui.radio_value(&mut a.no_deploy, true, "use running (goggles menu)");
            });
            ui.end_row();
            ui.label("Data link").on_hover_text(settings::USB_NET_HINT);
            ui.horizontal(|ui| {
                let on = settings::usb_net(a);
                if ui.radio(on, "USB network (faster)").on_hover_text(settings::USB_NET_HINT).clicked() {
                    settings::set_usb_net(a, true);
                }
                if ui.radio(!on, "adb").on_hover_text("adb forward only; the goggles' USB is left as it is").clicked() {
                    settings::set_usb_net(a, false);
                }
            });
            ui.end_row();
            ui.label("Encoder");
            egui::ComboBox::from_id_salt("encoder").selected_text(a.encoder.as_str()).show_ui(ui, |ui| {
                for s in ENCODERS {
                    ui.selectable_value(&mut a.encoder, s.to_string(), *s);
                }
            });
            ui.end_row();
            ui.label("Bitrate");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut a.bitrate).desired_width(60.0));
                egui::ComboBox::from_id_salt("bitrate").selected_text("").width(24.0).show_ui(ui, |ui| {
                    for s in BITRATES {
                        ui.selectable_value(&mut a.bitrate, s.to_string(), *s);
                    }
                });
            });
            ui.end_row();
            ui.label("FPS");
            ui.add(egui::DragValue::new(&mut a.fps).range(1..=240));
            ui.end_row();
        });
        if self.list_outputs_pending {
            self.list_outputs_pending = false;
            #[cfg(target_os = "linux")]
            self.refresh_outputs();
        }
        if let Some(e) = &self.outputs_err {
            ui.label(RichText::new(e).small().weak());
        }

        ui.add_space(8.0);
        ui.separator();
        ui.label(RichText::new("Head tracking").strong());
        let implied = self.saved.args.no_video || self.saved.args.print_pose;
        let mut ht = self.saved.args.head_tracking || implied;
        let r = ui.add_enabled(!implied, egui::Checkbox::new(&mut ht, "Head tracking (to opentrack)"));
        if r.on_disabled_hover_text("on: implied by --no-video / --print-pose").changed() {
            self.saved.args.head_tracking = ht;
            if let Some(e) = &self.engine {
                e.ctl.set_head_tracking(ht);
            }
        }
        ui.add_enabled_ui(running, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Recenter").clicked() {
                    if let Some(e) = &self.engine {
                        e.ctl.recenter.store(true, Ordering::Relaxed);
                    }
                }
                if ui.button("Force IDR").on_hover_text("send a keyframe now").clicked() {
                    if let Some(e) = &self.engine {
                        e.ctl.idr.store(true, Ordering::Relaxed);
                    }
                }
            });
        });

        ui.add_space(8.0);
        ui.separator();
        ui.label(RichText::new("Preview").strong());
        ui.checkbox(&mut self.saved.ui.preview, "Show what the goggles get");
        ui.add_enabled_ui(self.saved.ui.preview, |ui| {
            egui::Grid::new("pv_grid").num_columns(2).show(ui, |ui| {
                ui.label("Max fps");
                ui.add(egui::DragValue::new(&mut self.saved.ui.preview_fps).range(1..=100));
                ui.end_row();
                ui.label("Width");
                egui::ComboBox::from_id_salt("pvw").selected_text(self.saved.ui.preview_width.to_string()).show_ui(ui, |ui| {
                    for w in [320u32, 480, 640, 960, 1280, 1920] {
                        ui.selectable_value(&mut self.saved.ui.preview_width, w, w.to_string());
                    }
                });
                ui.end_row();
            });
            ui.checkbox(&mut self.saved.ui.preview_keyframes_only, "Keyframes only (lightest)");
        });
        if let Some(p) = self.engine.as_ref().and_then(|e| e.preview.as_ref()) {
            let s = &p.stats;
            ui.label(
                RichText::new(format!(
                    "decoded {}  shown {}  dropped {}  decode {:.1} ms",
                    s.decoded.load(Ordering::Relaxed),
                    s.shown.load(Ordering::Relaxed),
                    s.dropped.load(Ordering::Relaxed),
                    s.dec_us.load(Ordering::Relaxed) as f64 / 1000.0
                ))
                .small()
                .weak(),
            );
        }
    }

    fn overview(&mut self, ui: &mut egui::Ui) {
        let snap = self.last_snapshot();
        // preview + pose side by side
        let show_pv = self.saved.ui.preview;
        let show_pose = self.head_tracking_on();
        if show_pv || show_pose {
            ui.horizontal_top(|ui| {
                if show_pv {
                    self.preview_view(ui, false);
                }
                if show_pose {
                    self.pose_view(ui);
                }
            });
            ui.separator();
        }
        self.latency_view(ui, snap.as_ref());
    }

    fn preview_view(&mut self, ui: &mut egui::Ui, full_width: bool) {
        if let Some(p) = self.engine.as_ref().and_then(|e| e.preview.as_ref()) {
            if let Some(img) = p.image.lock().unwrap().take() {
                if img.seq != self.tex_seq {
                    self.tex_seq = img.seq;
                    // alpha is always 255: premultiplied = unmultiplied, without the per-pixel pass
                    let ci = egui::ColorImage::from_rgba_premultiplied([img.w, img.h], &img.rgba);
                    match &mut self.tex {
                        Some(t) => t.set(ci, TextureOptions::LINEAR),
                        None => self.tex = Some(ui.ctx().load_texture("preview", ci, TextureOptions::LINEAR)),
                    }
                }
            }
        }
        let w = if full_width { ui.available_width() } else { (ui.available_width() * 0.55).clamp(240.0, 640.0) };
        let size = egui::vec2(w, w * 9.0 / 16.0);
        let running = self.running();
        ui.vertical(|ui| {
            ui.set_width(w);
            match (&self.tex, running) {
                (Some(t), true) => {
                    ui.add(egui::Image::new(t).fit_to_exact_size(size));
                }
                _ => {
                    let (r, p) = ui.allocate_painter(size, egui::Sense::hover());
                    p.rect_filled(r.rect, 4.0, Color32::from_gray(16));
                    p.text(
                        r.rect.center(),
                        egui::Align2::CENTER_CENTER,
                        if running { "waiting for a keyframe…" } else { "preview (when streaming)" },
                        egui::FontId::proportional(14.0),
                        Color32::from_gray(140),
                    );
                }
            }
            ui.label(
                RichText::new(format!(
                    "Preview: about {} fps, downscaled, for checking the picture (not latency). The goggles get every frame.",
                    self.saved.ui.preview_fps
                ))
                .small()
                .weak(),
            );
        });
    }

    fn pose_view(&mut self, ui: &mut egui::Ui) {
        let pose = self
            .engine
            .as_ref()
            .and_then(|e| e.shared.link.lock().unwrap().clone())
            .and_then(|l| *l.shared.last_pose.lock().unwrap());
        ui.vertical(|ui| {
            let w = ui.available_width().clamp(180.0, 320.0);
            let (r, _) = ui.allocate_exact_size(egui::vec2(w, w * 0.7), egui::Sense::hover());
            pose::draw(ui, r, pose.map(|p| (p.yaw, p.pitch, p.roll)));
            egui::Grid::new("pose_nums").num_columns(2).show(ui, |ui| {
                for (n, v) in [("yaw", pose.map(|p| p.yaw)), ("pitch", pose.map(|p| p.pitch)), ("roll", pose.map(|p| p.roll))] {
                    ui.label(n);
                    ui.monospace(v.map(|v| format!("{v:+7.1}°")).unwrap_or_else(|| "     –".into()));
                    ui.end_row();
                }
            });
            if let Some(s) = self.last_snapshot() {
                ui.label(RichText::new(format!("{:.0} Hz", s.pose_hz)).small().weak());
            }
            if ui.add_enabled(self.running(), egui::Button::new("Recenter")).clicked() {
                if let Some(e) = &self.engine {
                    e.ctl.recenter.store(true, Ordering::Relaxed);
                }
            }
        });
    }

    fn latency_view(&mut self, ui: &mut egui::Ui, snap: Option<&Snapshot>) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Latency").strong());
            ui.label(RichText::new("per stats interval (Advanced->stats interval)").small().weak());
        });
        type Pair = fn(&Snapshot) -> (f64, f64);
        let rows: [(&str, Option<Pair>); 7] = [
            ("capture->encoder in", Some(|s| (s.cap_enc_p50_ms, s.cap_enc_p95_ms))),
            ("encode", Some(|s| (s.enc_p50_ms, s.enc_p95_ms))),
            ("capture->sent", Some(|s| (s.cap_send_p50_ms, s.cap_send_p95_ms))),
            ("capture->received (goggles)", Some(|s| (s.recv_p50_ms, s.recv_p95_ms))),
            ("goggles decode", Some(|s| (s.dec_p50_ms, s.dec_p95_ms))),
            ("goggles convert", Some(|s| (s.cvt_p50_ms, s.cvt_p95_ms))),
            ("capture->shown", Some(|s| (s.lat_p50_ms, s.lat_p95_ms))),
        ];
        ui.horizontal_top(|ui| {
            egui::Grid::new("lat").num_columns(3).striped(true).min_col_width(60.0).show(ui, |ui| {
                ui.label("");
                ui.label(RichText::new("p50 ms").weak());
                ui.label(RichText::new("p95 ms").weak());
                ui.end_row();
                for (name, get) in rows {
                    let strong = name == "capture->shown";
                    ui.label(if strong { RichText::new(name).strong() } else { RichText::new(name) });
                    let (a, b) = snap.zip(get).map(|(s, g)| g(s)).unwrap_or((f64::NAN, f64::NAN));
                    ui.monospace(fmt_ms(a));
                    ui.monospace(fmt_ms(b));
                    ui.end_row();
                }
            });
            ui.add_space(24.0);
            egui::Grid::new("thr").num_columns(2).striped(true).show(ui, |ui| {
                let s = snap.cloned().unwrap_or_default();
                let kv: [(&str, String); 9] = [
                    ("sent", format!("{:.1} fps", s.sent_fps)),
                    ("shown", format!("{:.1} fps", s.shown_fps)),
                    ("bitrate", format!("{:.1} Mbit/s", s.mbps)),
                    ("captured", format!("{:.1} fps", s.captured_fps)),
                    ("skipped", s.skipped.to_string()),
                    ("IDR", s.idr.to_string()),
                    ("dropped (goggles)", s.dropped.to_string()),
                    ("queue max", s.q_max.to_string()),
                    ("rtt", fmt_ms(s.rtt_ms) + " ms"),
                ];
                for (k, v) in kv {
                    ui.label(k);
                    ui.monospace(if snap.is_some() { v } else { "–".into() });
                    ui.end_row();
                }
            });
        });
        ui.add_space(6.0);
        if let Some(e) = &self.engine {
            let h = e.shared.history.lock().unwrap();
            plot::latency_plot(ui, &h, 120.0);
        } else {
            plot::latency_plot(ui, &Default::default(), 120.0);
        }
        ui.horizontal(|ui| {
            let can = self.engine.as_ref().is_some_and(|e| !e.shared.history.lock().unwrap().is_empty());
            if ui.add_enabled(can, egui::Button::new("Pin")).on_hover_text("store current settings + the mean of the last stats intervals for Compare").clicked() {
                self.pin();
            }
            ui.label("over the last");
            ui.add(egui::DragValue::new(&mut self.saved.ui.pin_window_s).range(1.0..=600.0).suffix(" s"));
            ui.label(RichText::new(format!("{} pins", self.saved.pins.len())).weak());
        });
    }

    fn compare(&mut self, ui: &mut egui::Ui) {
        if self.saved.pins.is_empty() {
            ui.label("No pins yet. Start a session, let it settle, then press Pin on the Overview tab. Change a setting, restart, pin again, compare here.");
            return;
        }
        ui.horizontal(|ui| {
            if ui.button("Select all").clicked() {
                self.saved.pins.iter_mut().for_each(|p| p.selected = true);
            }
            if ui.button("Select none").clicked() {
                self.saved.pins.iter_mut().for_each(|p| p.selected = false);
            }
            if ui.button("Delete selected").clicked() {
                self.saved.pins.retain(|p| !p.selected);
            }
            if ui.button("Copy table (Markdown)").clicked() {
                ui.ctx().copy_text(pins_markdown(self.saved.pins.iter().filter(|p| p.selected)));
            }
        });
        ui.separator();
        // checkboxes
        ui.horizontal_wrapped(|ui| {
            for p in self.saved.pins.iter_mut() {
                ui.checkbox(&mut p.selected, p.name.as_str());
            }
        });
        let sel: Vec<usize> = (0..self.saved.pins.len()).filter(|&i| self.saved.pins[i].selected).collect();
        if sel.is_empty() {
            return;
        }
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("cmp").striped(true).num_columns(sel.len() + 1).min_col_width(90.0).show(ui, |ui| {
                ui.label(RichText::new("name").weak());
                for &i in &sel {
                    if ui.add(egui::TextEdit::singleline(&mut self.saved.pins[i].name).desired_width(120.0)).changed() {
                            }
                }
                ui.end_row();
                ui.label(RichText::new("time").weak());
                for &i in &sel {
                    ui.label(fmt_time(self.saved.pins[i].time));
                }
                ui.end_row();
                ui.label(RichText::new("settings").weak());
                for &i in &sel {
                    let p = &self.saved.pins[i];
                    ui.label(RichText::new(&p.settings)).on_hover_text(format!("{}\n\n{}", p.command, p.notes));
                }
                ui.end_row();
                ui.label(RichText::new("window").weak());
                for &i in &sel {
                    let m = &self.saved.pins[i].metrics;
                    ui.label(format!("{:.0} s ({} iv)", m.seconds, m.intervals));
                }
                ui.end_row();
                for (row, (name, _)) in METRICS.iter().enumerate() {
                    ui.label(*name);
                    let vals: Vec<f64> = sel
                        .iter()
                        .map(|&i| self.saved.pins[i].metrics.values.iter().find(|v| v.0 == *name).map(|v| v.1).unwrap_or(f64::NAN))
                        .collect();
                    let higher_better = row < 3;
                    let best = vals.iter().copied().filter(|v| v.is_finite()).fold(None, |b: Option<f64>, v| {
                        Some(match b {
                            None => v,
                            Some(b) => if higher_better { b.max(v) } else { b.min(v) },
                        })
                    });
                    for v in vals {
                        let t = RichText::new(fmt_ms(v)).monospace();
                        let is_best = sel.len() > 1 && best == Some(v) && v.is_finite();
                        ui.label(if is_best { t.strong().color(Color32::from_rgb(80, 200, 120)) } else { t });
                    }
                    ui.end_row();
                }
            });
        });
    }

    fn advanced(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("All eyepiece-pc options. They apply on the next Start (head tracking, preview, recenter and IDR apply live).").weak());
        ui.add_space(4.0);
        // command line view / apply
        egui::CollapsingHeader::new("Command line").default_open(true).show(ui, |ui| {
            ui.add(egui::TextEdit::multiline(&mut self.cmdline_edit).desired_rows(2).desired_width(f32::INFINITY).code_editor());
            ui.horizontal(|ui| {
                if ui.button("Apply").on_hover_text("parse as eyepiece-pc options (unset options get defaults)").clicked() {
                    match parse_cmdline(&self.cmdline_edit) {
                        Ok(a) => {
                            let si = self.saved.args.stats_interval;
                            self.saved.args = a;
                            if !self.cmdline_edit.contains("--stats-interval") {
                                self.saved.args.stats_interval = si;
                            }
                            self.cmdline_err = None;
                        }
                        Err(e) => self.cmdline_err = Some(e),
                    }
                }
                if ui.button("Copy").clicked() {
                    ui.ctx().copy_text(settings::command_line(&self.saved.args));
                }
                if ui.button("Reset to defaults").on_hover_text("all options and preview settings; pins are kept; Undo for 20 s").clicked() {
                    self.reset_defaults();
                }
            });
            if let Some(e) = &self.cmdline_err {
                ui.label(RichText::new(e).color(Color32::from_rgb(230, 90, 80)));
            }
        });
        let a = &mut self.saved.args;
        section(ui, "Transport", |ui| {
            row(ui, "adb executable", |ui| ui.text_edit_singleline(&mut a.adb));
            row(ui, "adb serial", |ui| opt_text(ui, &mut a.serial, "first device"));
            row(ui, "port", |ui| ui.add(egui::DragValue::new(&mut a.port)));
            row(ui, "host", |ui| ui.text_edit_singleline(&mut a.host));
            row(ui, "data link", |ui| {
                egui::ComboBox::from_id_salt("transport").selected_text(a.transport.as_str()).show_ui(ui, |ui| {
                    for (v, h) in [
                        ("auto", "goggles' USB network when connected, else adb"),
                        ("usb-net", "USB network only"),
                        ("adb", "adb forward only"),
                    ] {
                        ui.selectable_value(&mut a.transport, v.to_string(), v).on_hover_text(h);
                    }
                })
            });
            row(ui, "no adb forward", |ui| ui.checkbox(&mut a.no_forward, "--no-forward"));
            row(ui, "use running bridge", |ui| ui.checkbox(&mut a.no_deploy, "--no-deploy"));
            row(ui, "bridge binary", |ui| opt_path(ui, &mut a.bridge, "next to exe / $XDG_DATA_HOME/eyepiece"));
            row(ui, "bridge args", |ui| ui.text_edit_singleline(&mut a.bridge_args));
            row(ui, "stop bridge on quit", |ui| ui.checkbox(&mut a.stop_bridge, "--stop-bridge"));
            row(ui, "reconnect", |ui| ui.checkbox(&mut a.reconnect, "--reconnect"));
            row(ui, "no bridge (encode only)", |ui| ui.checkbox(&mut a.no_connect, "--no-connect"));
        });
        section(ui, "Source", |ui| {
            row(ui, "source", |ui| ui.text_edit_singleline(&mut a.source));
            row(ui, "kms device", |ui| ui.text_edit_singleline(&mut a.kms_device));
            row(ui, "kms plane", |ui| ui.add(egui::DragValue::new(&mut a.kms_plane)));
            row(ui, "output", |ui| opt_text(ui, &mut a.output, "first active"));
            row(ui, "sync", |ui| combo(ui, "sync", &mut a.sync, &["vblank", "poll", "timer"]));
            row(ui, "x11 display", |ui| opt_text(ui, &mut a.display, "$DISPLAY"));
            row(ui, "x11 region", |ui| opt_text(ui, &mut a.region, "WxH+X+Y"));
            row(ui, "file input", |ui| opt_text(ui, &mut a.input, "media file"));
            row(ui, "test lavfi graph", |ui| opt_text(ui, &mut a.lavfi, "testsrc2"));
            row(ui, "pipewire restore token", |ui| opt_path(ui, &mut a.restore_token, "file"));
            row(ui, "cursor", |ui| ui.checkbox(&mut a.cursor, "--cursor").on_hover_text("PipeWire or ScreenCaptureKit"));
            row(ui, "macOS display ID", |ui| opt_num(ui, &mut a.display_id));
            row(ui, "dxgi adapter,output", |ui| ui.text_edit_singleline(&mut a.dxgi));
            row(ui, "kms simulate (test aid)", |ui| ui.checkbox(&mut a.kms_simulate, "--kms-simulate"));
        });
        section(ui, "Video", |ui| {
            row(ui, "fps", |ui| ui.add(egui::DragValue::new(&mut a.fps).range(1..=240)));
            row(ui, "codec", |ui| combo(ui, "codec", &mut a.codec, &["h264", "hevc"]));
            row(ui, "stereo 3D", |ui| {
                ui.checkbox(&mut a.stereo, "--stereo").on_hover_text(
                    "the source is both eyes side by side (e.g. a VR compositor window); sent as \
                     2560x720, the goggles switch to their 3D mode (~60 Hz) for the session",
                )
            });
            row(ui, "encoder", |ui| ui.text_edit_singleline(&mut a.encoder));
            row(ui, "vaapi device", |ui| ui.text_edit_singleline(&mut a.vaapi_device));
            row(ui, "bitrate", |ui| ui.text_edit_singleline(&mut a.bitrate));
            row(ui, "gop (0 = 2 s)", |ui| ui.add(egui::DragValue::new(&mut a.gop)));
            row(ui, "slices", |ui| ui.add(egui::DragValue::new(&mut a.slices).range(1..=32)));
            row(ui, "vbv frames", |ui| ui.add(egui::DragValue::new(&mut a.vbv_frames).range(0.1..=60.0).speed(0.1)));
            row(ui, "preset", |ui| opt_text(ui, &mut a.preset, "per encoder"));
            row(ui, "tune", |ui| opt_text(ui, &mut a.tune, "per encoder"));
            row(ui, "profile", |ui| opt_text(ui, &mut a.profile, "high"));
            row(ui, "intra refresh", |ui| ui.checkbox(&mut a.intra_refresh, "--intra-refresh"));
            row(ui, "encoder options", |ui| vec_lines(ui, &mut a.enc_opt, "KEY=VALUE per line"));
            row(ui, "stretch (no letterbox)", |ui| ui.checkbox(&mut a.no_letterbox, "--no-letterbox"));
            row(ui, "CPU scale", |ui| ui.checkbox(&mut a.cpu_scale, "--cpu-scale"));
            row(ui, "scale mode", |ui| combo(ui, "scale_mode", &mut a.scale_mode, &["hq", "fast", "default"]));
            row(ui, "extra filters", |ui| opt_text(ui, &mut a.vf, "--vf"));
            row(ui, "no video (tracking only)", |ui| ui.checkbox(&mut a.no_video, "--no-video"));
            row(ui, "head tracking", |ui| ui.checkbox(&mut a.head_tracking, "--head-tracking"));
        });
        section(ui, "Outputs / diagnostics", |ui| {
            row(ui, "opentrack UDP", |ui| ui.text_edit_singleline(&mut a.opentrack).on_hover_text("empty = off"));
            row(ui, "print pose (stdout)", |ui| ui.checkbox(&mut a.print_pose, "--print-pose"));
            row(ui, "ffplay preview window", |ui| opt_text(ui, &mut a.preview, "off (X,Y)"));
            row(ui, "dump Annex-B to", |ui| opt_path(ui, &mut a.dump, "off"));
            row(ui, "stop after frames", |ui| opt_num(ui, &mut a.frames));
            row(ui, "stop after seconds", |ui| opt_num(ui, &mut a.duration));
            row(ui, "stats interval s", |ui| ui.add(egui::DragValue::new(&mut a.stats_interval).range(0.2..=60.0).speed(0.1)));
            row(ui, "FFmpeg log level", |ui| combo(ui, "fflog", &mut a.ffmpeg_log, &["quiet", "error", "warning", "info", "verbose", "debug"]));
        });
        if let Some(p) = &self.config {
            ui.label(RichText::new(format!("settings file: {}", p.display())).small().weak());
        }
    }

    fn log(&mut self, ui: &mut egui::Ui) {
        let Some(e) = &self.engine else {
            ui.label("No session yet.");
            return;
        };
        let lines: Vec<String> = e.shared.log.lock().unwrap().iter().cloned().collect();
        ui.horizontal(|ui| {
            if ui.button("Copy log").clicked() {
                ui.ctx().copy_text(lines.join("\n"));
            }
        });
        egui::ScrollArea::both().stick_to_bottom(true).auto_shrink([false, false]).show(ui, |ui| {
            for l in &lines {
                ui.monospace(l);
            }
        });
    }
}

// ---- small widgets ----

fn section(ui: &mut egui::Ui, title: &str, add: impl FnOnce(&mut egui::Ui)) {
    egui::CollapsingHeader::new(title).default_open(false).show(ui, |ui| {
        egui::Grid::new(title).num_columns(2).spacing([12.0, 6.0]).striped(true).show(ui, add);
    });
}

fn row<R>(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui) -> R) {
    ui.label(label);
    add(ui);
    ui.end_row();
}

fn combo(ui: &mut egui::Ui, id: &str, v: &mut String, items: &[&str]) -> egui::Response {
    egui::ComboBox::from_id_salt(id)
        .selected_text(v.as_str())
        .show_ui(ui, |ui| {
            for s in items {
                ui.selectable_value(v, s.to_string(), *s);
            }
        })
        .response
}

fn opt_text(ui: &mut egui::Ui, v: &mut Option<String>, hint: &str) -> egui::Response {
    let mut s = v.clone().unwrap_or_default();
    let r = ui.add(egui::TextEdit::singleline(&mut s).hint_text(hint));
    if r.changed() {
        *v = (!s.is_empty()).then_some(s);
    }
    r
}

fn opt_path(ui: &mut egui::Ui, v: &mut Option<PathBuf>, hint: &str) -> egui::Response {
    let mut s = v.as_ref().map(|p| p.display().to_string()).unwrap_or_default();
    let r = ui.add(egui::TextEdit::singleline(&mut s).hint_text(hint));
    if r.changed() {
        *v = (!s.is_empty()).then(|| PathBuf::from(s));
    }
    r
}

fn opt_num<T: egui::emath::Numeric + Default>(ui: &mut egui::Ui, v: &mut Option<T>) -> egui::Response {
    ui.horizontal(|ui| {
        let mut on = v.is_some();
        if ui.checkbox(&mut on, "").changed() {
            *v = on.then(T::default);
        }
        if let Some(x) = v.as_mut() {
            ui.add(egui::DragValue::new(x));
        }
    })
    .response
}

fn vec_lines(ui: &mut egui::Ui, v: &mut Vec<String>, hint: &str) -> egui::Response {
    let mut s = v.join("\n");
    let r = ui.add(egui::TextEdit::multiline(&mut s).desired_rows(2).hint_text(hint));
    if r.changed() {
        *v = s.lines().map(str::trim).filter(|l| !l.is_empty()).map(str::to_string).collect();
    }
    r
}

fn fmt_ms(v: f64) -> String {
    if v.is_finite() {
        format!("{v:6.1}")
    } else {
        "     –".into()
    }
}

/// UTC "YYYY-MM-DD HH:MM" from unix seconds (no date crate needed).
fn fmt_time(t: u64) -> String {
    let days = (t / 86_400) as i64;
    let s = t % 86_400;
    // civil from days (Howard Hinnant)
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + (m <= 2) as i64;
    format!("{y:04}-{m:02}-{d:02} {:02}:{:02} UTC", s / 3600, s % 3600 / 60)
}

fn pins_markdown<'a>(pins: impl Iterator<Item = &'a Pin>) -> String {
    let pins: Vec<&Pin> = pins.collect();
    let mut s = String::from("| |");
    for p in &pins {
        s += &format!(" {} |", p.name);
    }
    s += "\n|---|";
    s += &"---|".repeat(pins.len());
    s += "\n| settings |";
    for p in &pins {
        s += &format!(" `{}` |", p.settings);
    }
    for (name, _) in METRICS {
        s += &format!("\n| {name} |");
        for p in &pins {
            let v = p.metrics.values.iter().find(|v| v.0 == *name).map(|v| v.1).unwrap_or(f64::NAN);
            s += &format!(" {} |", fmt_ms(v).trim());
        }
    }
    s + "\n"
}

/// Split a command line (simple shell quoting: '…', "…", backslash) and parse it as
/// `eyepiece-pc` options; a leading `eyepiece-pc` is optional.
fn parse_cmdline(s: &str) -> Result<Args, String> {
    use clap::Parser;
    let mut words = vec![];
    let mut cur = String::new();
    let mut have = false;
    let mut q: Option<char> = None;
    let mut it = s.chars();
    while let Some(c) = it.next() {
        match (q, c) {
            (None, '\'' | '"') => {
                q = Some(c);
                have = true;
            }
            (Some(qc), c) if c == qc => q = None,
            (None | Some('"'), '\\') => {
                if let Some(n) = it.next() {
                    cur.push(n);
                }
            }
            (None, c) if c.is_whitespace() => {
                if have || !cur.is_empty() {
                    words.push(std::mem::take(&mut cur));
                }
                have = false;
            }
            (_, c) => cur.push(c),
        }
    }
    if q.is_some() {
        return Err("unterminated quote".into());
    }
    if have || !cur.is_empty() {
        words.push(cur);
    }
    if words.first().is_some_and(|w| w.ends_with("eyepiece-pc") || w.ends_with("eyepiece-pc.exe")) {
        words.remove(0);
    }
    let argv = std::iter::once("eyepiece-pc".to_string()).chain(words);
    Args::try_parse_from(argv).map_err(|e| e.to_string())
}

fn write_ppm(p: &std::path::Path, img: &egui::ColorImage) -> std::io::Result<()> {
    let [w, h] = img.size;
    let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
    for c in &img.pixels {
        out.extend_from_slice(&[c.r(), c.g(), c.b()]);
    }
    std::fs::write(p, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmdline_parses_quotes() {
        let a = parse_cmdline("eyepiece-pc --source test --opentrack '' --lavfi \"testsrc2=size=1920x1080\"").unwrap();
        assert_eq!(a.source, "test");
        assert_eq!(a.opentrack, "");
        assert_eq!(a.lavfi.as_deref(), Some("testsrc2=size=1920x1080"));
        assert!(parse_cmdline("--bogus").is_err());
    }

    #[test]
    fn command_line_reparses() {
        let a = Args { bitrate: "30M".into(), opentrack: String::new(), enc_opt: vec!["a=b c".into()], ..Default::default() };
        let b = parse_cmdline(&settings::command_line(&a)).unwrap();
        assert_eq!(settings::command_line(&a), settings::command_line(&b));
    }

    #[test]
    fn time_format() {
        assert_eq!(fmt_time(0), "1970-01-01 00:00 UTC");
        assert_eq!(fmt_time(1_700_000_000), "2023-11-14 22:13 UTC");
    }
}
