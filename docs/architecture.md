# Architecture

```
 KMS plane / PipeWire / X11 / DXGI ──► convert (GPU: scale, letterbox, NV12) ──► encode (NVENC/VAAPI/…)
        stamped at PC vblank               stage A thread                          stage B thread
                                                                                       │ Annex-B AU, pts = capture time
 opentrack UDP ◄── pose ◄──────── Eyepiece protocol (adb forward tcp:42420) ◄──────────┘──► eyepiece_bridge
```

## Design and latency

The workspace crates (`crates/`) keep the engine usable without a UI:

| crate | contents |
|---|---|
| `eyepiece-link` | The [bridge protocol](protocol/bridge.md) over a TCP or USB bulk link (vectored writes, no access-unit copy), the receive thread, windowed min-RTT clock sync, keyframe requests, opentrack output, statistics, adb deploy and forward. No FFmpeg. |
| `eyepiece-capture` | The `Capture` trait and its backends: `kms` (libdrm, vblank pacing), `x11` (XShm), `portal` (PipeWire, default feature `pipewire`), `dxgi` (Windows) and `screencapturekit` (macOS), plus `pacer` (refresh decimation). No FFmpeg. |
| `eyepiece-av` | FFmpeg bindings (bindgen), zero-copy DMA-BUF and CPU frame wrapping, the GPU converter (libavfilter), encoder presets, Annex-B handling, test and file sources. |
| `eyepiece-pc` | `session::run(Config, Control, Events)` (the library API), `cli` (options to `Config`, adb), and `rpc` (settings, session control and the web server). |
| `eyepiece-xr` | A separate small tool that forwards the opentrack pose into Monado's remote driver for OpenXR on Linux; see [head tracking](head-tracking.md#openxr-through-monado-linux-experimental). |

Latency measures:
- **Refresh-synchronised capture.** KMS: `drmWaitVBlank` on the plane's CRTC, then the
  plane's current framebuffer is exported; a frame is only taken when the framebuffer changed
  (page flip) — or when an IDR is needed — and decimated evenly to `--fps` (120 Hz panel →
  100 fps takes 5 of 6 vblanks). `--sync poll` checks the plane every 1 ms; `--sync timer`
  free-runs. PipeWire: buffer arrival. DXGI: AcquireNextFrame (DWM present).
- **Zero-copy GPU path.** DMA-BUF → `hwmap` into VAAPI → `scale_vaapi` (aspect kept) → NV12
  BT.709 limited → `h264_vaapi` directly. For NVENC/x264/AMF/QSV the scaled NV12 picture is
  copied with streaming loads into pooled buffers whose black bars were filled once.
- **Correct frame rate to rate control**: time_base 1/fps, pts from capture time, CBR with
  VBV = `--vbv-frames` frames.
- **No queues.** Capture + convert hands frames to encode + send through a one-frame slot; a
  newer frame replaces an unclaimed one (`skip`). The AU is written to the socket as soon as
  the encoder returns it.
- **Immediate IDR** on (re)connect, when the bridge reports a queue overflow or needs a
  keyframe, when frames are sent but none is shown for 200 ms, and on `i`.

Defaults: H.264 High with CABAC (HEVC on macOS), GOP 2 s, CBR, VBV 4 frames, 4 slices, no B-frames; NVENC
`p1`/`ull`/zerolatency, VAAPI `async_depth=1`, x264 `superfast`/`zerolatency`; letterboxed 1920x1080.
Measured with NVENC at 100 fps and 30 Mbit/s: capture to send about 7.5 ms p50 / 8 ms p95, and with KMS
capture about 13 to 14 ms p50 from capture to shown on the goggles.

## Session control flow

A session is one capture/encode/send pipeline. It is configured once and can be changed while it runs:

1. Connect to the bridge, wait for its `Info`, then send one `Configure` (codec, layout, optional pane). The bridge
   refuses video until it has acknowledged it. See the [bridge protocol](protocol/bridge.md).
2. Stage A (capture and GPU convert) hands frames to stage B (encode and send) through a one-frame slot.
3. A live pane or transparency change builds the new converter and encoder first, then sends a `Configure`; only after
   the acknowledgement does it start sending the new stream, beginning with a keyframe. A rejected change keeps the old
   stream running.
4. Frame reports, pose, and clock-sync pongs come back on the same connection and feed the statistics and opentrack.

The [RPC protocol](protocol/rpc.md) and the [web UI](web-ui.md) control the same engine from a browser.

## Status

| Component | State |
|---|---|
| KMS capture (with `setcap`), vblank pacing, 10-bit, VAAPI to NVENC | tested with the goggles |
| NVENC, VAAPI, x264 | tested |
| HEVC | works with the bridge's hardware decoder (the macOS default); less tested than H.264 |
| AMF, QSV | wired up, untested |
| X11 (XShm) | tested; slow for very large roots, use `--region` |
| PipeWire portal (default feature `pipewire`, Linux) | builds; not yet run against the goggles |
| Windows DXGI | builds in CI, never run; CPU copy through a staging texture |
| macOS ScreenCaptureKit and VideoToolbox | host-side capture and encoding measured on an M4 Pro (see [encoder measurements](benchmarks/encoder.md)); goggles untested |
| Stereo to the goggles | unavailable |
| USB bulk transport | experimental |

## Documentation map

| Topic | Where |
|---|---|
| Install and run | [install](install.md), [usage](usage.md) |
| Features | [PiP and transparency](pip-and-transparency.md), [head tracking](head-tracking.md), [transports](transports.md), [web UI](web-ui.md), [macOS](macos.md) |
| Wire formats | [bridge protocol](protocol/bridge.md), [RPC protocol](protocol/rpc.md) |
| Goggles side | [bridge and mod](goggles/bridge.md), [display composition](goggles/display-composition.md), [device dump](goggles/device-dump.md) |
| Build, test, release | [building](development/building.md) |
| Measurements | [encoder measurements](benchmarks/encoder.md), [serialization benchmark](benchmarks/serialization.md) |
