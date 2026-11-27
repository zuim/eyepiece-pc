# eyepiece-pc

PC side of **Eyepiece**: use the DJI Goggles 3 as a USB display for your PC, with head tracking back.
`eyepiece-pc` captures the PC screen, encodes it with low latency (NVENC, VAAPI, AMF, QSV, macOS
VideoToolbox or x264 through the FFmpeg libraries) and sends it over USB to `eyepiece_bridge` on the
goggles. The goggles' head attitude comes back at 100 Hz and is forwarded to
[opentrack](https://github.com/opentrack/opentrack), so games with head-tracking support can use it.

```
 screen capture ─► convert (GPU) ─► encode ─► eyepiece-pc ══ USB ══► eyepiece_bridge ─► hardware decode ─► goggles display
                                                  ▲                          │
                                       opentrack ◄┴── head pose, frame stats ◄┘
```

The goggles side lives in [natartos](https://github.com/zuim/natartos) (branch `eyepiece`, mod
`mods/eyepiece`; a fork of grom-labs' natartos): it adds an **Eyepiece** entry to the goggles' menu and puts
the stream on the display. The wire format is defined in
[`contract/eyepiece-rpc.proto`](contract/eyepiece-rpc.proto), shared by the bridge protocol and the
web UI's RPC.

## Features

- Low-latency capture from KMS, PipeWire, X11, DXGI (Windows) or ScreenCaptureKit (macOS), paced to the display
  refresh; about 13–14 ms capture-to-shown with KMS and NVENC.
- Full-panel video or a movable picture-in-picture pane, with colour-key or source-alpha transparency and live
  GUI and pane opacity.
- Head tracking into games through opentrack (Windows, and Linux through Wine/Proton).
- USB through `adb forward`, or a dedicated USB bulk interface.
- A browser UI (`eyepiece-pc --web`) with live settings, statistics, head-pose view and stream preview.
- Many settings change live without restarting the stream, and a stream that was running resumes when the
  web UI server restarts; `--reconnect` rides out a link drop.

## Quick start

1. Get a release or [build from source](docs/development/building.md); install `adb`.
2. Connect the goggles (running natartos with the Eyepiece mod) over USB.
3. Run it:

```sh
eyepiece-pc                       # capture the first screen, best encoder
eyepiece-pc --head-tracking       # stream and head tracking to opentrack
eyepiece-pc --web                 # the browser UI at http://127.0.0.1:4174/
```

`eyepiece-pc` starts the bridge on the goggles by itself. Everything is experimental: **never use it while flying.**

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

AGPL-3.0-or-later, same as natartos. See [LICENSE](LICENSE).
