# Installation

How to install `eyepiece-pc` on the PC, and what the goggles need. To build from source see
[building](development/building.md). To run it see [usage](usage.md).

## What you need

- The DJI Goggles 3 running [tinkerglass](https://github.com/tinkerglass-org/tinkerglass) with its **PC Display**
  mod (see [the goggles side](goggles/bridge.md)).
- A USB cable to the goggles and `adb` (Android platform-tools) on the PC.
- This tool, `eyepiece-pc`, from a release or built from source.

## Prebuilt releases

Download from the GitHub releases page:

- `eyepiece-pc-<version>-linux-x86_64.tar.gz` (`eyepiece-pc`) — built on Debian testing against **FFmpeg 9**
  (libavcodec 63). It needs the FFmpeg 9 runtime libraries (`libavcodec63`, `libavfilter12`,
  `libavformat63`, `libavutil61`), libdrm and libpipewire-0.3; on distributions with another
  FFmpeg major, build from source.
- `eyepiece-pc-<version>-linux-x86_64.AppImage` — the CLI with FFmpeg 9 bundled, for distributions with glibc 2.39 or
  newer (Ubuntu 24.04, Debian 13, Fedora 40 and later). No KMS screen capture from the
  AppImage (it cannot hold the capability): use PipeWire/X11 capture or the tar.gz.
  NVENC needs NVIDIA driver 530 or newer; AMF and QSV are not included.
- `eyepiece-pc-<version>-windows-x86_64.zip` (`eyepiece-pc.exe`) — includes the FFmpeg DLLs (BtbN shared build).
  **The Windows (DXGI capture) path is untested.**
- `eyepiece-pc-<version>-macos-arm64.dmg` — Apple Silicon CLI app bundle,
  with FFmpeg libraries included; requires macOS 15 or newer. The bundle is
  ad-hoc signed, **not Developer ID signed or notarized**, so Gatekeeper can
  block it until the user explicitly approves it in System Settings.

You also need `adb` (Android platform-tools) and the goggles running tinkerglass with the PC Display
mod. eyepiece-pc starts the bridge installed with the mod by itself; it does not push a bridge to the goggles.
On Windows the video interface binds WinUSB by itself, so there is no driver to install.
On macOS, ensure Homebrew's `bin` directory is on PATH so `adb` can be found
(typically `/opt/homebrew/bin`).
The optional external `--preview` window also requires `ffplay` on PATH.

Recommended on Linux: install to `~/.local/bin` (see [Permissions](#linux-kms-capture-permissions)).

```sh
tar xf eyepiece-pc-*-linux-x86_64.tar.gz
install -Dm755 eyepiece-pc-*/eyepiece-pc ~/.local/bin/eyepiece-pc
```

## Linux KMS capture permissions

KMS capture (the default on Linux) reads the framebuffer the display controller is scanning
out. The kernel only hands out framebuffer handles (`drmModeGetFB2`) to callers with
**CAP_SYS_ADMIN**; opening `/dev/dri/card0` needs group `video`. Give the capability to an
installed copy — a rebuild drops file capabilities, and a capability binary should not live
in a user-writable build directory:

```sh
install -Dm755 target/release/eyepiece-pc ~/.local/bin/eyepiece-pc
sudo setcap cap_sys_admin+ep ~/.local/bin/eyepiece-pc
getcap ~/.local/bin/eyepiece-pc          # (libcap2-bin) → cap_sys_admin=ep
```

10-bit framebuffers (`AR30`/`AB30`/`XR30`/`XB30`, the KWin 6 default) are imported
directly. fp16 (`AB48`, HDR on) is not supported. `eyepiece-pc` drops the capability
immediately for every non-KMS run and refuses `--vf` while it holds it (filters can load
plugin code). `--source pipewire` and `--source x11` need no privileges.
