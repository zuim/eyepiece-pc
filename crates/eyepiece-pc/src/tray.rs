//! The Windows notification-area icon of a double-click start: open the web UI, start or stop
//! the stream, quit. Click opens the web UI; the menu is on the right button.

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

/// Run the icon on this thread (it owns the window messages) until `exit` is set or Quit is
/// chosen. `phase` returns the session phase (idle, starting, running, stopping) when it changed.
pub fn run(act: &dyn Fn(Action), phase: &mut dyn FnMut() -> Option<String>, exit: &AtomicBool) -> Result<(), String> {
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
        .with_menu_on_left_click(false)
        .with_tooltip("eyepiece-pc")
        .with_icon(icon()?)
        .build()
        .map_err(|e| format!("tray icon: {e}"))?;
    while !exit.load(Ordering::Relaxed) {
        pump_messages();
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

fn pump_messages() {
    use windows_sys::Win32::UI::WindowsAndMessaging::{DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE};
    let mut message: MSG = unsafe { std::mem::zeroed() };
    while unsafe { PeekMessageW(&mut message, std::ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {
        unsafe {
            TranslateMessage(&message);
            DispatchMessageW(&message);
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
