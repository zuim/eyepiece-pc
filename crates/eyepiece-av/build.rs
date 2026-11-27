// Generates FFmpeg bindings with bindgen against the installed headers.
//
// Linux/macOS: FFmpeg dev packages installed (Debian: libavcodec-dev libavfilter-dev
// libavformat-dev libavutil-dev libswscale-dev; Homebrew: ffmpeg) → found with pkg-config.
// Windows (MSVC): a pinned BtbN FFmpeg build is downloaded once into the target directory and its
// DLLs are copied next to the binaries, so `cargo build` needs no setup.
// Anywhere: FFMPEG_INCLUDE_DIR=<dir with libavcodec/…> and FFMPEG_LIB_DIR=<dir with the libraries>
// use that FFmpeg instead (e.g. headers unpacked from the .debs, see env-local.sh).
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// FFmpeg 9 for Windows: BtbN's shared GPL build. A month-end autobuild, because BtbN keeps those
/// (the daily ones are pruned). CI uses the same one.
const WIN_FFMPEG_TAG: &str = "autobuild-2026-10-31-12-59";
const WIN_FFMPEG_NAME: &str = "ffmpeg-n9.0.2-39-g7adfbe40dc-win64-gpl-shared-9.0";
const WIN_FFMPEG_SHA256: &str = "1d9823c35c7a7e2f54d1438cf61030d6efabe3278de01c191309d1e828bb74df";

const LIBS: &[(&str, &str)] = &[
    ("libavcodec", "avcodec"),
    ("libavfilter", "avfilter"),
    ("libavformat", "avformat"),
    ("libavutil", "avutil"),
];

fn main() {
    println!("cargo:rerun-if-env-changed=FFMPEG_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=FFMPEG_LIB_DIR");
    println!("cargo:rerun-if-changed=wrapper.h");
    let mut includes: Vec<PathBuf> = vec![];
    let windows_msvc = env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    if let Ok(dir) = env::var("FFMPEG_INCLUDE_DIR") {
        includes.push(dir.into());
        if let Ok(l) = env::var("FFMPEG_LIB_DIR") {
            println!("cargo:rustc-link-search=native={l}");
        }
        for (_, l) in LIBS {
            println!("cargo:rustc-link-lib=dylib={l}");
        }
    } else if windows_msvc {
        let ffmpeg = windows_ffmpeg();
        includes.push(ffmpeg.join("include"));
        println!("cargo:rustc-link-search=native={}", ffmpeg.join("lib").display());
        for (_, l) in LIBS {
            println!("cargo:rustc-link-lib=dylib={l}");
        }
        copy_dlls(&ffmpeg.join("bin"));
    } else {
        for (pc, _) in LIBS {
            let lib = pkg_config::Config::new().probe(pc).unwrap_or_else(|e| {
                panic!(
                    "{pc} not found ({e}).\nInstall the FFmpeg development packages \
                     (Debian/Ubuntu: sudo apt install libavcodec-dev libavfilter-dev libavformat-dev libavutil-dev) \
                     or set FFMPEG_INCLUDE_DIR / FFMPEG_LIB_DIR."
                )
            });
            includes.extend(lib.include_paths);
        }
    }
    let mut b = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("(av|avcodec|avfilter|avformat|avio)_.*")
        .allowlist_type("(AV|av).*")
        .allowlist_var("(AV|FF)_.*")
        .allowlist_var("(AVERROR|AVSEEK)_.*")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .prepend_enum_name(false)
        .derive_default(true)
        .layout_tests(false)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for i in &includes {
        b = b.clang_arg(format!("-I{}", i.display()));
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ffi.rs");
    b.generate().expect("bindgen failed (is libclang installed? set LIBCLANG_PATH)").write_to_file(out).unwrap();
}

/// target/<profile> (or target/<triple>/<profile>): OUT_DIR lies below <that>/build/.
fn profile_dir() -> PathBuf {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    out.ancestors()
        .find(|dir| dir.file_name().is_some_and(|name| name == "build"))
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("OUT_DIR {} is not below a build directory", out.display()))
        .to_path_buf()
}

/// The pinned FFmpeg build under the target directory, downloaded and checked on first use.
fn windows_ffmpeg() -> PathBuf {
    let root = profile_dir().parent().unwrap().join("ffmpeg");
    let dir = root.join(WIN_FFMPEG_NAME);
    if dir.join("include/libavcodec/avcodec.h").is_file() && dir.join("lib/avcodec.lib").is_file() {
        return dir;
    }
    std::fs::create_dir_all(&root).unwrap();
    // Windows' own curl.exe and tar.exe (bsdtar reads zip), not a Git-for-Windows one on PATH
    let system = PathBuf::from(env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into())).join("System32");
    let zip = root.join(format!("{WIN_FFMPEG_NAME}.zip"));
    let url = format!("https://github.com/BtbN/FFmpeg-Builds/releases/download/{WIN_FFMPEG_TAG}/{WIN_FFMPEG_NAME}.zip");
    // not a cargo:warning: cargo replays those on every later build, as if it downloaded again
    eprintln!("downloading FFmpeg for Windows (once, ~85 MB): {url}");
    run(Command::new(system.join("curl.exe")).args(["-fsSL", "--retry", "3", "-o"]).arg(&zip).arg(&url), "download FFmpeg");
    // certutil's text is localized (and not UTF-8); its second line is the hash in hex
    let out = run(Command::new(system.join("certutil.exe")).arg("-hashfile").arg(&zip).arg("SHA256"), "hash the FFmpeg zip");
    let hash = String::from_utf8_lossy(&out.stdout).into_owned();
    let hash: String = hash.lines().nth(1).unwrap_or("").chars().filter(|c| c.is_ascii_hexdigit()).collect();
    if !hash.eq_ignore_ascii_case(WIN_FFMPEG_SHA256) {
        let _ = std::fs::remove_file(&zip);
        panic!("FFmpeg download {url} has sha256 {hash}, expected {WIN_FFMPEG_SHA256}");
    }
    let _ = std::fs::remove_dir_all(&dir);
    run(Command::new(system.join("tar.exe")).arg("-xf").arg(&zip).arg("-C").arg(&root), "unpack FFmpeg");
    let _ = std::fs::remove_file(&zip);
    assert!(dir.join("include/libavcodec/avcodec.h").is_file(), "unpacked FFmpeg lacks {}", dir.display());
    dir
}

/// Next to the binaries (target/<profile>) and the test executables (deps/), where Windows looks.
fn copy_dlls(bin: &Path) {
    let profile = profile_dir();
    for dest in [profile.clone(), profile.join("deps")] {
        std::fs::create_dir_all(&dest).unwrap();
        for entry in std::fs::read_dir(bin).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("dll")) {
                let to = dest.join(path.file_name().unwrap());
                let same = std::fs::metadata(&to).ok().map(|m| m.len()) == std::fs::metadata(&path).ok().map(|m| m.len());
                if !same {
                    std::fs::copy(&path, &to).unwrap_or_else(|e| panic!("copy {} → {}: {e}", path.display(), to.display()));
                }
            }
        }
    }
}

fn run(cmd: &mut Command, what: &str) -> std::process::Output {
    let out = cmd.output().unwrap_or_else(|e| panic!("{what}: {e}"));
    if !out.status.success() {
        panic!("{what} failed ({}): {}", out.status, String::from_utf8_lossy(&out.stderr));
    }
    out
}
