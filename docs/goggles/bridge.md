# Goggles side: the PC Display mod and bridge

The goggles half lives in [tinkerglass](https://github.com/tinkerglass-org/tinkerglass), the Goggles 3 mod
platform. Its **PC Display** mod adds *PC Display* to the goggles' More menu and ships
**`pcdisplay_bridge`**, a native binary that receives the video stream, decodes it in hardware, drives the
display and returns head tracking. This page documents it from the PC project's point of view. The wire format is
in the [bridge protocol](../protocol/bridge.md); how the display composes layers is in
[display composition](display-composition.md).

```
PC ──USB video interface (or adb forward tcp:42420)──► pcdisplay_bridge ──► VC8000D decode ──► VIDEO0 or GFX1 plane
PC ◄──── head pose / frame stats ──── pcdisplay_bridge ◄── MCU head attitude (100 Hz)
```

Everything here is experimental; **never use it while flying.**

The sections after [tinkerglass differences](#tinkerglass-differences) (display modes, options, internals, USB
bulk) were written for the natartos-era bridge (`eyepiece_bridge`, which put the stream on the display through a
hook in the DJI GUI). The protocol, the display planes and the stream behaviour are the same; the differences are
listed under tinkerglass differences and win where they disagree.

- [Requirements](#requirements)
- [Starting the bridge](#starting-the-bridge)
- [tinkerglass differences](#tinkerglass-differences)
- [Display modes](#display-modes)
- [Bridge options](#bridge-options)
- [Safety](#safety)
- [Internals](#internals)
- [USB bulk transport](#usb-bulk-transport)
- [Build and deploy](#build-and-deploy)
- [Known limits](#known-limits)

## Requirements

- tinkerglass installed on the goggles with the **PC Display** mod, and its mods running (install and start steps
  are in the tinkerglass documentation).
- `adb` on the PC. In tinkerglass's USB mode the video stream travels over the goggles' own USB interface and
  `adb` only starts the bridge; without that mode `adb forward` carries the stream.
- The PC side, **eyepiece-pc** (this repository).

## Starting the bridge

1. **From the PC (default).** `eyepiece-pc` writes a launch request for tinkerglass's menu service
   (`/data/vendor/tinkerglass/run/menu/launch.req`, content `pcdisplay -1 -l <log> <args>`), so the bridge starts as
   from the goggles' menu. It exits with the PC session (`-1`). A bridge already running from the menu hands over by
   itself. Nothing is pushed to the goggles; if the mod is not installed, or the tinkerglass mods are not running,
   `eyepiece-pc` stops with a message saying so. See the [CLI guide](../usage.md).
2. **From the goggles' menu.** **More → PC Display**, then `eyepiece-pc --no-deploy` (and
   `adb forward tcp:42420 tcp:42420` when not using the USB interface).
3. **By hand** from `adb shell`: `/vendor/tinkerglass/current/mods/pcdisplay/bin/pcdisplay_bridge -h` lists the
   options. A bridge started from the ADB shell runs in another SELinux domain than the menu's; prefer 1 or 2.

The log is `/data/vendor/tinkerglass/run/pcdisplay/bridge.log`.

## tinkerglass differences

- **Capabilities.** The bridge advertises `STEREO`, `PANE_BGRA`, `PANE_OPAQUE_BGRA`, `PANE_ALPHA` and `RECONFIG`.
  It does not advertise `GUI_ALPHA`, `GUI_ALPHA_ACTIVE` or `PANE_RECONNECT`: while a `Configure` is active it hides
  the DJI GUI completely (through tinkerglass's display service), and a disconnect releases everything. The PC
  treats the missing capabilities as unavailable: `--gui-opacity` is ignored with a notice, the stream still starts.
- **Panes and the full-panel stream** both work in the same bridge process, without `-R`.
- **Transport.** The USB video interface needs no bridge option (`-B` is gone); see
  [transports](../transports.md). There is no USB network (RNDIS) mode.
- **No overlay mod, no backdrop hook.** The GUI backdrop handshake and the `eyepiece.*` files below belong to the
  natartos-era bridge.

## Display modes

The display has three layers the bridge can use. See
[display composition](display-composition.md) for the stack and evidence.

| Mode | Plane | When | Formats |
|---|---|---|---|
| Panel video | VIDEO0 (plane 4), below the GUI | `Configure` without a pane | NV12 from the decoder, scanned out directly; optional side-by-side stereo with 3D timing |
| Opaque pane | GFX1 (plane 6), between radio video and the GUI | `Configure` with pane format 2 | decoder NV12 scanned out on GFX1 with hardware global alpha; no colour conversion |
| Packed-alpha pane | GFX1 | pane format 1 | colour in the left half, grayscale alpha in the right half of one frame; reconstructed to ARGB on the goggles |

The goggles have **no display scaler**; the PC scales to the requested size.

### GFX1 panes

- Requires hardware decode on VOP0 and a bridge started with `-1` or `-R`. The bridge
  advertises the pane capabilities only in that mode and acquires GFX1 without stopping its
  current owner; before using GFX1 it stops `dji_arhome` if that service is enabled and
  running, and restores it when the bridge exits. If the service was already disabled or stopped,
  its state is left unchanged. If the bridge is killed first, restore the service with
  `adb shell setprop dji.arhome_service 1` when it was enabled before the session.
  `dji_arhome` may provide AR markers and motion-controller functions.
- **Opaque panes** push decoder NV12 frames straight to GFX1 (`yuv420sp-8`, global alpha,
  blending offset). `EYEPIECE_PANE_RGB=1` selects the older RGB888-I32 conversion path.
- **Packed-alpha panes** use the encoded frame twice as wide as the pane: colour on the left
  (limited-range BT.709), straight grayscale alpha on the right (luma 16 transparent, 235
  opaque). The bridge reconstructs ARGB with NEON-optimised, multi-threaded staging and
  retains the last staged image so opacity changes need no new frame. Fullscreen transparency
  therefore means a 3840×1080 stream.
- GFX1 acquisition can leave the GUI and GFX1 both at blending order 1. The overlay moves a colliding GUI to
  order 2, above GFX1, and the bridge refreshes blending even at GUI alpha 255.
- Opaque H.264 panes up to 1280×720 default to 20 decoder buffers for small VideoToolbox
  streams with large reference counts; `-F` overrides this.
- When the decode queue backs up, the bridge skips some decoded GFX1 frames before conversion
  while still decoding reference frames. This reduces conversion load without forcing an IDR;
  sustained overload can still overflow the input queue, in which case lower the capture frame
  rate or the pane size. Display error `-1001` is a generic HAL rejection, not proof of a full
  queue. The bridge drops transient rejected frames and ends the session after 30 consecutive
  rejections; other display errors are fatal.
- Stereo GFX1 output is unavailable: VOP1 has no GFX1 plane.

### Live reconfiguration

A running GFX1 stream can be replaced by sending another `Configure` (see
[the protocol](../protocol/bridge.md#configure)).

- Moving a pane (only `x` or `y` differ) moves GFX1 without reopening the decoder.
- Any other change (size, format, codec, dropping the pane) drains in-flight frames, releases
  the decoder and opens a new one on the next keyframe.
- A VIDEO0 decoder, once open, is kept until the bridge exits; replacing it is rejected.
- Opacity is preserved across reconfiguration.

### Retained sessions (`-R`)

With `-R` the bridge stays running after the client disconnects and keeps the last GFX1 frame
on screen, and the DJI GUI stays suppressed behind it. A reconnecting client sends `Configure`
again: an identical request resumes, a different one replaces the retained stream and the image
may disappear while the new decoder starts. Quit the persistent bridge when finished.

### GUI and pane opacity

- **GUI opacity** is applied by the overlay mod on the DJI GUI plane and acknowledged only
  after the display API succeeds. It is held by a lock the bridge releases on disconnect; the
  overlay then restores stock blending, unless a `-R` pane is retained. Fullscreen GUI hiding
  takes precedence and rejects opacity changes.
- **Pane opacity** multiplies source and colour-key alpha. Alpha panes use pixel blending;
  opaque panes use hardware global alpha.

### GUI backdrop suppression

With no air-unit video the DJI GUI draws an opaque background layer that would hide GFX1 and
VIDEO0. While a GUI-visible VIDEO0 or GFX1 stream is displayed, the Eyepiece mod suppresses that
layer through a hook, and restores DJI's requested visibility after the bridge exits or crashes.
A retained `-R` frame keeps suppression active. The hook only works with the `libew_gls.so` build
ID `54323da7526004b9e5c3d75d172539e5`, and the bridge refuses these streams if the hook does not
acknowledge the request. The mechanism is described in
[display composition](display-composition.md#gui-backdrop-suppression). Air-unit video behaviour
with the hook installed has not been tested.

## Bridge options

*tinkerglass:* the PC Display bridge takes `-p -A -V -H -b -1 -N -F -S -L -O -M -Q -l -Z -h`; `-B`, `-R`, `-s` and `-c`
below are natartos-era options it does not have (`-U` is accepted and ignored).

```text
-p PORT      TCP port on 127.0.0.1 (default 42420)
-A           listen on all interfaces instead of loopback
-B           add the dedicated FunctionFS bulk interface (experimental)
-s WxH       output dimensions (default 1920x1080)
-c N         colour-convert threads (default 4)
-V           no video (head tracking only)
-H           no head tracking
-Z           only send the MCU head-attitude stop, then exit (recovery)
-b           exit on Back key (menu launches)
-1           exit after the first client disconnects
-R           allow matching GFX1 sessions to reconnect; stay running until quit
-F N         hw: number of frame buffers (default 12)
-S DIR       hw: duss_hal_mem_sync(frame, DIR) before reading frames
-O plane[:M] zero-copy scan-out on VOP mask M (1 = VOP0, 3 = both; default plane:1)
-L R,F,V     hw: decoder start-config knobs (no_reorder default 1, +0x24, +0x2c)
-N           null decoder (protocol and latency plumbing test; nothing is displayed)
-l FILE      log file (tinkerglass: eyepiece-pc passes /data/vendor/tinkerglass/run/pcdisplay/bridge.log)
```

Direct VIDEO0 output is the default. GUI opacity and pane alpha are configured through the
protocol, not through options. GFX1 panes use BGRA output and manage `dji_arhome` while active.

## Safety

- **Head tracking.** To get head tracking, the bridge starts the goggles' MCU attitude stream
  (bus command `0x15/0x31` to the MCU). It sends only two fixed payloads, start and stop, and
  stops the stream on every exit path: normal exit, the bridge dying (the helper process gets a
  parent-death signal) and the helper dying (the bridge runs a stopper). If the stream is ever
  left running, run `/vendor/tinkerglass/current/mods/pcdisplay/bin/pcdisplay_bridge -Z`; it sends only the stop
  and exits.
- **Files.** At runtime the bridge writes only under `/data/local/tmp`: `eyepiece.log`, `eyepiece.backdrop`
  (the GUI backdrop handshake), `eyepiece.gui-alpha` (the GUI opacity request) and `eyepiece.bulk-state` (the
  USB bulk recovery snapshot). It never writes `/system`, `/vendor` or any partition. The overlay restores
  display state when the bridge exits or crashes.
- **`dji_arhome`.** If the bridge is forcibly killed before cleanup, restore the service as
  described under [GFX1 panes](#gfx1-panes).
- **Installing is the only write to `/vendor`**, and it needs a deliberate remount; see
  [Build and deploy](#build-and-deploy).
- Never use it while flying; test with stationary goggles.

## Internals

- `eyepiece_bridge.c` handles connections, requests and the decode queue. A single decoder
  thread owns the decoder: it applies `Configure`, pane opacity changes and disconnect drains,
  and decodes queued access units.
- `wire.c` validates and (de)serializes the generated protobuf `Envelope` messages, which the bridge
  uses directly with the generated enums. Video and ID bytes borrow the receive buffer.
- `hwdec.c` submits access units to the VC8000D through DJI's HAL (`/dev/vdec0`, `/dev/ion0`).
  VIDEO0 and opaque GFX1 panes receive the decoded NV12 frames directly; alpha panes use
  `yuv2bgra.c` to stage and convert packed colour and alpha to ARGB. Decoder frames are returned
  on a separate thread.
- `dispout.c` drives the display, tracks scan-out buffers across VOPs, and reclaims dropped frames. A
  buffer rejected by every VOP stays with its caller.
- `usb_bulk.c` manages FunctionFS. `headtrack.c` isolates the MCU bus client in a forked helper process.

Ownership rules to keep when changing decoder, display or reconnect code:

- Pane reconfiguration drains decoder callbacks and returned frames **before** releasing display
  resources.
- On shutdown, HAL buffers are retained until process exit when their ownership cannot be released
  safely. A decoder that cannot be moved or released ends the bridge.
- If the shutdown watcher, decoder or requested Back-key thread cannot start, the bridge logs the
  failure and exits. If the head-tracking reader cannot start, its helper is terminated and reaped;
  video stays available.

## USB bulk transport

*tinkerglass:* the video interface is part of tinkerglass's USB mode (interface `ff/54/03`, owned by its USB service),
not something the bridge creates; there is no `-B`, no recovery snapshot and no gadget switch by the bridge. The
description below is the natartos-era `-B` mode.

`-B` adds a separately named `ffs.eyepiece` FunctionFS vendor interface without reusing DJI's bulk services.
TCP listeners stay available. This is **experimental**: do not treat it as ready for normal use.

- The gadget switch temporarily disconnects USB. Normal shutdown restores the previous binding, and
  startup checks a recovery snapshot in `/data/local/tmp/eyepiece.bulk-state`; an unreadable or
  invalid snapshot prevents startup without changing the gadget. A power cycle returns the runtime
  gadget configuration to stock if recovery fails.
- Endpoint pumps use bounded 64 KiB staging buffers and blocking FunctionFS reads and writes (endpoint
  files do not implement `.poll`). The socketpair between the pumps and the bridge uses non-blocking polling
  with a 500 ms send deadline. A stop flag, `SIGUSR1` wake-ups and thread joins bound teardown.
- Measured on a Mac at High-Speed: ping/pong about 0.4 ms RTT, and sustained 1080p 20 Mbit/s streams at about
  20 fps without drops.
- Intermittent USB resets can occur independently and interrupt ADB or bulk sessions.

The host side is described in [transports](../transports.md).

## Build and deploy

tinkerglass builds and installs the bridge with its PC Display mod (`mods/pcdisplay/bridge/` in the tinkerglass
repository; see its documentation for the toolchain and the install). The bridge needs the schema of this
repository, `contract/eyepiece-rpc.proto`, at build time. `build.sh host` there builds an x86-64
`pcdisplay_bridge_host` for protocol tests on the PC (see
[building](../development/building.md#bridge-interoperability-tests)). eyepiece-pc never pushes a bridge to the
goggles and never writes to `/vendor`.

## Known limits

- **GFX1 transparency at full panel** (a 3840×1080 packed stream) has not been tried on the goggles; the decoder
  and conversion capacity at that size is unknown. Packed-alpha conversion takes about 7 ms per 640×480 frame
  with four threads (see [display composition](display-composition.md#conversion-performance)).
- **Live reconfiguration** has been exercised with frame reports only (pane moves, resizes, opaque and packed
  changes, H.264 and HEVC, reconnect). Visual output and long-duration operation are unverified.
- **Air-unit video** with the backdrop hook installed has not been tested.
- **Stereo GFX1** is unavailable (VOP1 has no GFX1 plane).
- **Large size or format changes under load**, for example an 88 fps stream, can make the decoder stop time out.
  Position-only moves avoid the decoder restart; other changes drain frames first.
- Display error `-1001` is a generic HAL rejection and does not prove a full queue.
- SELinux enforcing, USB bulk soak tests, forced IN backpressure on USB bulk, and real desktop capture over bulk
  are untested.

Typical numbers for a 552×300 opaque HEVC pane at 100 fps over `adb`: no drops, capture-to-submit about 6.5 ms p50
and 9.5 ms p95; per frame, about 2.2 ms conversion, 4 µs cache sync and 45 µs display push. The first frames of a
stream take tens of milliseconds (decoder start and backdrop acknowledgement); that backlog is skipped rather than
displayed.
