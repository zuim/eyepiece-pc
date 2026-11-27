# eyepiece-pc

PC side of **Eyepiece**: use the DJI Goggles 3 as a USB display for your PC, with head tracking back.
`eyepiece-pc` captures the PC screen, encodes it with low latency (NVENC, VAAPI, AMF, QSV, macOS
VideoToolbox or x264 through the FFmpeg libraries) and sends it over USB to the PC Display bridge (`pcdisplay_bridge`) on
the goggles. The goggles' head attitude comes back at 100 Hz and is forwarded to
[opentrack](https://github.com/opentrack/opentrack), so games with head-tracking support can use it.

```
 screen capture ─► convert (GPU) ─► encode ─► eyepiece-pc ══ USB ══► pcdisplay_bridge ─► hardware decode ─► goggles display
                                                  ▲                          │
                                       opentrack ◄┴── head pose, frame stats ◄┘
```

The goggles side is the **PC Display** mod of [tinkerglass](https://github.com/tinkerglass-org/tinkerglass), the
Goggles 3 mod platform: it adds *PC Display* to the goggles' More menu, and tinkerglass's display service puts
the stream on the display. The wire format is defined in
[`contract/eyepiece-rpc.proto`](contract/eyepiece-rpc.proto), shared by the bridge protocol and the
web UI's RPC.

## Features

- Low-latency capture from KMS, PipeWire, X11, DXGI (Windows) or ScreenCaptureKit (macOS), paced to the display
  refresh; about 13–14 ms capture-to-shown with KMS and NVENC.
- Full-panel video or a movable picture-in-picture pane, with colour-key or source-alpha transparency and live
  GUI and pane opacity.
- Head tracking into games through opentrack (Windows, and Linux through Wine/Proton).
- USB through tinkerglass's own video interface (WinUSB on Windows, no driver step), or `adb forward` as the
  fallback.
- A local browser UI (always running alongside, `--open-web` opens it) with live settings, statistics, head-pose
  view and stream preview; on Windows a double-click start puts eyepiece-pc in the notification area.
- Many settings change live without restarting the stream, and a stream that was running resumes when the
  web UI server restarts; `--reconnect` rides out a link drop.

## Quick start

1. Get a release or [build from source](docs/development/building.md); install `adb`.
2. Connect the goggles (running tinkerglass with the PC Display mod) over USB.
3. Run it:

```sh
eyepiece-pc                       # capture the first screen, best encoder
eyepiece-pc --head-tracking       # stream and head tracking to opentrack
eyepiece-pc --open-web            # and open the web UI (http://127.0.0.1:4174/)
eyepiece-pc --no-web              # plain command line: no web UI, nothing saved
```

`eyepiece-pc` starts the bridge on the goggles by itself. Everything is experimental: **never use it while flying.**

On Windows, double-clicking `eyepiece-pc.exe` opens the web UI in the browser and leaves an icon in the
notification area (click: open the web UI; right-click: start or stop the stream, quit).

## Known issues

- **Mouse pointer on Windows (DXGI).** Desktop Duplication frames do not contain a hardware cursor, and on
  Windows 11 26H2 the pointer went missing from the stream. eyepiece-pc now draws it itself
  (`--dxgi-cursor auto`, the default: only when the captured image does not already show it; `on`, `off`).
  Pointers on a rotated display are not handled yet.

## Documentation

See the [documentation index](docs/README.md). Main entry points:

| | |
|---|---|
| [Installation](docs/install.md) | [Command-line usage](docs/usage.md) |
| [Picture in picture and transparency](docs/pip-and-transparency.md) | [Head tracking into games](docs/head-tracking.md) |
| [Web UI](docs/web-ui.md) | [Architecture](docs/architecture.md) |
| [Bridge protocol](docs/protocol/bridge.md) | [Goggles mod and bridge](docs/goggles/bridge.md) |

## Status

KMS capture with NVENC, VAAPI and x264 is tested with the goggles. Other capture sources and encoders are
less tested; the Windows path has never been run. See [architecture](docs/architecture.md#status) for the
full table.

## License

AGPL-3.0-or-later. See [LICENSE](LICENSE).
