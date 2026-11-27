# macOS

On macOS `eyepiece-pc` captures with [ScreenCaptureKit](https://github.com/doom-fish/screencapturekit-rs) and
encodes with FFmpeg's VideoToolbox encoders. ScreenCaptureKit is the default source (`sck` is an alias).

## Building

Needs macOS 13 or newer, Xcode 26 (the macOS 26 SDK and Swift compiler, for the Swift bridges), current stable
Rust (the lockfile has Edition 2024 dependencies, so old Cargo versions fail) and the FFmpeg headers. The Intel
macOS Nix shell is not supported: its Swift 5.10 and macOS 14 SDK are too old for the `apple-metal` dependency.

```sh
brew install rust ffmpeg pkg-config llvm android-platform-tools
export LIBCLANG_PATH="$(brew --prefix llvm)/lib"
export MACOSX_DEPLOYMENT_TARGET=13.0
cargo build --release
```

Make sure Homebrew's `bin` directory (typically `/opt/homebrew/bin`) is on `PATH` so `adb` is found.

## Running

```sh
target/release/eyepiece-pc --list                                        # displays
target/release/eyepiece-pc --encoder videotoolbox --fps 60 --cursor
target/release/eyepiece-pc --display-id 1 --encoder videotoolbox         # an ID from --list
```

Grant your terminal (or the packaged app) permission in **System Settings > Privacy & Security > Screen &
System Audio Recording**, then restart it. macOS can blank protected content. The first available display is
used unless `--display-id` is given.

## Capture

- Capture uses a latest-frame slot: a frame the encoder has not claimed is replaced by a newer one, and captured
  surfaces stay reference-counted until conversion releases them. Frames are stamped with the shared monotonic
  clock when the callback arrives.
- `--fps` is a maximum. The display refresh and ScreenCaptureKit delivery set the actual cadence, and `--sync`
  has no effect on ScreenCaptureKit scheduling.
- A static screen delivers no frames, so forced keyframes replay the last frame, paced at the requested frame
  interval, also while waiting for the first encoded keyframe.
- With a VideoToolbox encoder and no `--vf` or `--cpu-scale`, capture requests encoder-sized video-range NV12
  (`420v`) surfaces, and ScreenCaptureKit does the scaling, chroma-aligned letterboxing and Rec. 709
  conversion. The buffer is wrapped in an `AV_PIX_FMT_VIDEOTOOLBOX` frame and then read back to CPU NV12 before
  encoding. Handing the native buffer straight to the encoder is faster to wire but made the encoder queue about
  four frames, so the readback is deliberate.
- Software encoders, `--vf`, `--cpu-scale`, and test and file sources use BGRA capture and CPU conversion.
- Transparency uses CPU capture frames (see [PiP and transparency](pip-and-transparency.md)).

## Encoding

- `--encoder auto` tries hardware-only VideoToolbox first, then falls back to x264 (x265 for HEVC). Explicit
  `--encoder videotoolbox` sets `allow_sw=0`, `realtime=1` and `prio_speed=1` and uses no B-frames; if the device
  or the linked FFmpeg cannot open it, `eyepiece-pc` reports an error instead of falling back to software.
  `--enc-opt` values override these defaults.
- `--codec` defaults to HEVC on macOS (H.264 elsewhere). VideoToolbox HEVC is limited to one reference frame
  (`max_ref_frames=1`) so the goggles' decoder does not run out of frame buffers.
- VideoToolbox is driven by a target bitrate; the VBV and slice options are managed by the driver and are not
  strict CBR/VBV as with NVENC. `--enc-opt constant_bit_rate=1` requests strict CBR where supported and is
  rejected otherwise.
- `x264` means `libx264` (H.264 only) and `x265` means `libx265` (HEVC only). A mismatch such as
  `--encoder x264 --codec hevc` is an error. `auto` and `software` choose a codec-appropriate software encoder,
  and explicit FFmpeg encoder names are checked against the codec too.
- Hardware stays the default to preserve CPU headroom. `--encoder x264` is the option that gives the lowest
  host latency; see [encoder benchmarks](benchmarks/encoder.md).

To check the FFmpeg build and run a hardware-only encode without capture or goggles:

```sh
ffmpeg -hide_banner -encoders | grep videotoolbox
target/release/eyepiece-pc --source test --encoder videotoolbox --fps 60 \
  --no-connect --duration 10 --dump mac-hardware.h264 --ffmpeg-log debug
```

The session log names `h264_videotoolbox` and its `allow_sw=0` option. An encoder listed by `ffmpeg -encoders` is
not proof that the hardware can open it.

## App bundle

CI builds `eyepiece-pc-<version>-macos-arm64.dmg` with `VERSION=<tag-or-commit> bash packaging/macos/build-dmg.sh`
on macOS 15. It holds the CLI (`Eyepiece.app/Contents/MacOS/eyepiece-pc`) and its non-system libraries, plus
Homebrew formula metadata for FFmpeg and its dependencies. The bundle needs macOS 15 or newer because its Homebrew
libraries are built there. It is ad-hoc signed, not Developer ID signed or notarized (that needs separate
credentials), so Gatekeeper can block it until it is approved in System Settings. Its `Info.plist` carries
`NSScreenCaptureUsageDescription`; no entitlement bypasses the Screen Recording consent.

## Testing

CI builds the Apple-only code and runs the tests that do not capture; it does not exercise permissions, live
capture or hardware encoding. Linux tests cover layout and converter routing; the macOS unit tests cover native
buffer retention and frame cloning. To compare capture-side scaling with BGRA CPU conversion without goggles:

```sh
target/release/eyepiece-pc --encoder videotoolbox --fps 60 --no-connect --duration 10 --dump nv12.h264
target/release/eyepiece-pc --encoder videotoolbox --fps 60 --cpu-scale --no-connect --duration 10 --dump cpu.h264
```

On a Mac, check decoded output dimensions, black bars on a 16:10 or portrait display, colours, forced keyframes
(`i` then Enter) on a static screen, reconnect, and memory use under encoder load.
