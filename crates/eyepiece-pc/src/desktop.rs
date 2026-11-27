//! Small desktop integration: opening the web UI in the browser, telling a double-click start
//! from a terminal, error boxes, and the native file dialog behind the web UI's "Browse".

/// Open `url` in the default browser.
pub fn open_url(url: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_SHOWNORMAL};
        let verb = wide("open");
        let file = wide(url);
        // values above 32 mean success
        let result = unsafe {
            ShellExecuteW(std::ptr::null_mut(), verb.as_ptr(), file.as_ptr(), std::ptr::null(), std::ptr::null(), SW_SHOWNORMAL)
        };
        if result as isize > 32 { Ok(()) } else { Err(format!("cannot open {url} (error {})", result as isize)) }
    }
    #[cfg(not(windows))]
    {
        let program = if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
        std::process::Command::new(program)
            .arg(url)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .spawn()
            .map(drop)
            .map_err(|e| format!("cannot open {url} with {program}: {e}"))
    }
}

/// Whether the process was started without a terminal of its own: on Windows, a console that
/// holds only this process was created for it (a double-click in Explorer or a shortcut).
/// Elsewhere programs started from a desktop get no console window, so this is always false
/// and the tray is not offered.
pub fn started_from_desktop() -> bool {
    #[cfg(windows)]
    {
        let mut processes = [0u32; 2];
        let count = unsafe {
            windows_sys::Win32::System::Console::GetConsoleProcessList(processes.as_mut_ptr(), processes.len() as u32)
        };
        count == 1
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Close the console window that was created for a double-click start (the tray takes over).
pub fn hide_console() {
    #[cfg(windows)]
    unsafe {
        windows_sys::Win32::System::Console::FreeConsole();
    }
}

/// Tell the user about a fatal error when there is no terminal to print it to.
pub fn error_box(message: &str) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};
        let text = wide(message);
        let title = wide("eyepiece-pc");
        unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), MB_OK | MB_ICONERROR) };
    }
    #[cfg(not(windows))]
    eprintln!("[eyepiece-pc] {message}");
}

/// Ask for a video file with the system's file dialog. The web UI runs on the same computer as
/// eyepiece-pc (loopback only), so the dialog appears where the user is; `Ok(None)` is a cancel.
/// Blocks until the dialog closes.
pub fn pick_video_file() -> Result<Option<String>, String> {
    #[cfg(windows)]
    {
        windows_file_dialog()
    }
    #[cfg(target_os = "macos")]
    {
        dialog_command("osascript", &["-e", "POSIX path of (choose file with prompt \"Video file for eyepiece-pc\")"])
            .map(|result| result.ok_or_else(|| "osascript is not available".to_string()))?
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        let dialogs: [(&str, &[&str]); 2] = [
            ("zenity", &["--file-selection", "--title=Video file for eyepiece-pc"]),
            ("kdialog", &["--getopenfilename", ".", "video/*", "--title", "Video file for eyepiece-pc"]),
        ];
        for (program, args) in dialogs {
            if let Some(result) = dialog_command(program, args)? {
                return Ok(result);
            }
        }
        Err("no file dialog available (install zenity or kdialog), type the path instead".into())
    }
}

/// Run a dialog program: `Ok(None)` when it is not installed, `Ok(Some(None))` on cancel.
#[cfg(not(windows))]
fn dialog_command(program: &str, args: &[&str]) -> Result<Option<Option<String>>, String> {
    let output = match std::process::Command::new(program).args(args).stdin(std::process::Stdio::null()).output() {
        Ok(output) => output,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("{program}: {e}")),
    };
    let path = String::from_utf8_lossy(&output.stdout).trim_end_matches(['\r', '\n']).to_string();
    Ok(Some((output.status.success() && !path.is_empty()).then_some(path)))
}

#[cfg(windows)]
fn windows_file_dialog() -> Result<Option<String>, String> {
    use windows_sys::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
    use windows_sys::Win32::UI::Controls::Dialogs::{
        CommDlgExtendedError, GetOpenFileNameW, OFN_EXPLORER, OFN_FILEMUSTEXIST, OFN_NOCHANGEDIR, OFN_PATHMUSTEXIST,
        OPENFILENAMEW,
    };
    // a thread of its own: the dialog needs a single-threaded COM apartment
    std::thread::spawn(|| {
        let filter: Vec<u16> = "Video files\0*.mkv;*.mp4;*.mov;*.webm;*.avi;*.ts;*.m2ts;*.flv;*.wmv\0All files\0*.*\0\0"
            .encode_utf16()
            .collect();
        let title = wide("Video file for eyepiece-pc");
        let mut file = vec![0u16; 32 * 1024];
        let mut dialog: OPENFILENAMEW = unsafe { std::mem::zeroed() };
        dialog.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
        dialog.lpstrFilter = filter.as_ptr();
        dialog.lpstrFile = file.as_mut_ptr();
        dialog.nMaxFile = file.len() as u32;
        dialog.lpstrTitle = title.as_ptr();
        dialog.Flags = OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_NOCHANGEDIR;
        let initialized = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) } >= 0;
        let chosen = unsafe { GetOpenFileNameW(&mut dialog) } != 0;
        let error = if chosen { 0 } else { unsafe { CommDlgExtendedError() } };
        if initialized {
            unsafe { CoUninitialize() };
        }
        if chosen {
            let len = file.iter().position(|&c| c == 0).unwrap_or(file.len());
            Ok(Some(String::from_utf16_lossy(&file[..len])))
        } else if error == 0 {
            Ok(None)
        } else {
            Err(format!("file dialog failed (error {error:#x})"))
        }
    })
    .join()
    .map_err(|_| "file dialog thread panicked".to_string())?
}

#[cfg(windows)]
pub(crate) fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
