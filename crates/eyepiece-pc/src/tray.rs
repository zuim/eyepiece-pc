//! The tray icon of a desktop start (notification area on Windows and Linux, menu bar on macOS):
//! open the web UI, start or stop the stream, quit. Click opens the web UI where the platform
//! reports clicks; the menu is on the right button (Windows, Linux) or the icon (macOS).
//!
//! Linux uses StatusNotifierItem over D-Bus (KDE, most other desktops; GNOME with the
//! AppIndicator extension). Without a tray host the icon simply does not appear and the web UI's
//! Quit remains.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

pub enum Action {
    Open,
    Start,
    Stop,
    Quit,
}

/// Run the icon on this thread until `exit` is set or Quit is chosen. On macOS this must be the
/// main thread: it also runs the application's event loop. `phase` returns the session phase
/// (idle, starting, running, stopping) when it changed.
pub fn run(act: &dyn Fn(Action), phase: &mut dyn FnMut() -> Option<String>, exit: &AtomicBool) -> Result<(), String> {
    let events = EventLoop::new()?;
    let open = MenuItem::new("Open eyepiece-pc", true, None);
    let start = MenuItem::new("Start stream", false, None);
    let stop = MenuItem::new("Stop stream", false, None);
    let quit = MenuItem::new("Quit", true, None);
    let menu = Menu::new();
    menu.append_items(&[
        &open,
        &PredefinedMenuItem::separator(),
        &start,
        &stop,
        &PredefinedMenuItem::separator(),
        &quit,
    ])
    .map_err(|e| format!("tray menu: {e}"))?;
    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(cfg!(target_os = "macos"))
        .with_tooltip("eyepiece-pc")
        .with_icon(icon()?)
        .build()
        .map_err(|e| format!("tray icon: {e}"))?;
    while !exit.load(Ordering::Relaxed) {
        events.pump();
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            let action = if event.id == open.id() {
                Action::Open
            } else if event.id == start.id() {
                Action::Start
            } else if event.id == stop.id() {
                Action::Stop
            } else if event.id == quit.id() {
                Action::Quit
            } else {
                continue;
            };
            let quitting = matches!(action, Action::Quit);
            act(action);
            if quitting {
                return Ok(());
            }
        }
        while let Ok(event) = TrayIconEvent::receiver().try_recv() {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                act(Action::Open);
            }
        }
        if let Some(phase) = phase() {
            start.set_enabled(phase == "idle");
            stop.set_enabled(phase == "running" || phase == "starting");
            let _ = tray.set_tooltip(Some(format!("eyepiece-pc: {}", match phase.as_str() {
                "running" => "streaming",
                "idle" => "not streaming",
                other => other,
            })));
        }
        std::thread::sleep(Duration::from_millis(30));
    }
    Ok(())
}

/// The platform's event delivery for the icon on this thread.
struct EventLoop {
    #[cfg(target_os = "macos")]
    app: objc2::rc::Retained<objc2_app_kit::NSApplication>,
}

impl EventLoop {
    fn new() -> Result<EventLoop, String> {
        #[cfg(target_os = "macos")]
        {
            use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
            let mtm = objc2::MainThreadMarker::new().ok_or("the menu-bar icon needs the main thread")?;
            let app = NSApplication::sharedApplication(mtm);
            // a menu-bar item only: no Dock icon, no menu bar of its own
            app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
            app.finishLaunching();
            Ok(EventLoop { app })
        }
        #[cfg(not(target_os = "macos"))]
        Ok(EventLoop {})
    }

    /// Deliver the pending window-system events (Linux: none, D-Bus runs on its own threads).
    fn pump(&self) {
        #[cfg(windows)]
        {
            use windows_sys::Win32::UI::WindowsAndMessaging::{DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE};
            let mut message: MSG = unsafe { std::mem::zeroed() };
            while unsafe { PeekMessageW(&mut message, std::ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {
                unsafe {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            use objc2_app_kit::NSEventMask;
            let mode = unsafe { objc2_foundation::NSDefaultRunLoopMode };
            while let Some(event) = self.app.nextEventMatchingMask_untilDate_inMode_dequeue(NSEventMask::Any, None, mode, true) {
                self.app.sendEvent(&event);
            }
        }
    }
}

/// A drawn eyepiece: a teal ring with a dark lens, no image file needed.
fn icon() -> Result<Icon, String> {
    const SIZE: u32 = 32;
    let mut rgba = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    let center = (SIZE as f32 - 1.0) / 2.0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let d = ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();
            let pixel = if d <= 9.0 {
                [0x14, 0x22, 0x20, 0xff]
            } else if d <= 15.5 {
                [0x3f, 0xd0, 0xa8, 0xff]
            } else {
                [0, 0, 0, 0]
            };
            rgba.extend_from_slice(&pixel);
        }
    }
    Icon::from_rgba(rgba, SIZE, SIZE).map_err(|e| format!("tray icon image: {e}"))
}
