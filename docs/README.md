# Eyepiece documentation

Eyepiece uses the DJI Goggles 3 as a USB display for a PC, with head tracking sent back. These pages
cover the PC tool (`eyepiece-pc`) and the goggles side (tinkerglass's PC Display mod and `pcdisplay_bridge`).
Start with the [project README](../README.md) for an overview.

## Using Eyepiece

| Page | Contents |
|---|---|
| [Installation](install.md) | Releases, requirements, Linux KMS capture permissions |
| [Command-line usage](usage.md) | Starting a stream, resolution, statistics, options |
| [Picture in picture and transparency](pip-and-transparency.md) | GFX1 pane, colour key, GUI and pane opacity |
| [Head tracking into games](head-tracking.md) | opentrack output, Wine/Proton, Linux games |
| [Transports and USB bulk](transports.md) | tinkerglass's video interface and `adb forward` |
| [Web UI](web-ui.md) | The browser interface, live settings, preview |
| [macOS](macos.md) | ScreenCaptureKit, VideoToolbox, the app bundle |

## How it works

| Page | Contents |
|---|---|
| [Architecture](architecture.md) | Pipeline, crates, latency design, status |
| [Bridge protocol](protocol/bridge.md) | PC ↔ goggles wire format and state machine |
| [RPC protocol](protocol/rpc.md) | Web UI ↔ `eyepiece-pc` |

## Goggles side

| Page | Contents |
|---|---|
| [Mod and bridge](goggles/bridge.md) | Starting, display modes, options, safety, internals, build and deploy |
| [Display composition](goggles/display-composition.md) | Planes, formats, GUI backdrop, research log |
| [Device dump](goggles/device-dump.md) | Raw inventory of a goggles unit |

## Developing

| Page | Contents |
|---|---|
| [Building and testing](development/building.md) | Toolchains, tests, packaging, CI |
| [Open work](development/todo.md) | Unsorted notes |
| [Encoder measurements](benchmarks/encoder.md) | macOS encoder and capture results |
| [Serialization benchmark](benchmarks/serialization.md) | Video message encode/decode cost |
