# Command-line usage

`eyepiece-pc` is a command-line program; `eyepiece-pc --web` adds the [web UI](web-ui.md) on top of the
same engine. Run `eyepiece-pc --help` for every option.

## Basic use

Connect the goggles over USB, then run:

```sh
eyepiece-pc --list                                  # outputs, planes, framebuffer formats
eyepiece-pc                                         # capture the first screen, best encoder
eyepiece-pc --output eDP-1 --encoder nvenc --bitrate 30M
eyepiece-pc --head-tracking                         # stream + head tracking to opentrack
eyepiece-pc --no-video --print-pose                 # head tracking only
eyepiece-pc --source pipewire --restore-token ~/.config/eyepiece.token   # Wayland portal, no root
eyepiece-pc --source test --preview                 # test pattern + local ffplay preview
eyepiece-pc --source file --input clip.mkv
eyepiece-pc --transport usb-bulk                    # use the dedicated USB bulk link
eyepiece-pc --no-deploy                             # use a bridge that is already running
```

By default eyepiece-pc starts the bridge itself, so there is no need to open Eyepiece in the goggles'
menu first (a bridge started there is replaced). A local `eyepiece_bridge` next to the executable, in
`$XDG_DATA_HOME/eyepiece/` or given with `--bridge` is pushed to `/data/local/tmp/eyepiece/`;
otherwise the one installed on the goggles is used. It runs with `-1 -b` (plus `-R` with `--reconnect`
and a pane, and any `--bridge-args`), exits with the PC session, and logs to
`/data/local/tmp/eyepiece.log` on the goggles.

`--no-deploy` uses a bridge that is already running. eyepiece-pc always asks for a GFX1 pane (the whole
panel by default), which needs a bridge started with `-1` or `-R`
(see [the bridge options](goggles/bridge.md#bridge-options)); a bridge started from the goggles' menu
rejects it.

## Resolution, input and statistics

By default the stream is 1920x1080, the goggles' panel size: they have no display scaler.
Every source is scaled to fit on the PC and letterboxed with black bars (`--no-letterbox`
stretches instead). A [picture-in-picture pane](pip-and-transparency.md) is encoded at its own size.

Stdin: `r` recenter, `i` force an IDR, `q` quit (each + Enter). Ctrl-C quits cleanly
(twice: hard). `--dump out.h264` also writes the exact Annex-B stream; `--no-connect
--duration 10` encodes without a bridge. See `eyepiece-pc --help` for all
options (`--fps` default 100 = goggles panel, `--bitrate` default 20M, `--sync
vblank|poll|timer`, `--reconnect`, `--host` for a directly reachable bridge, …).

Stats every 3 s:
```
sent 100.3 fps 30.1 Mbit/s | shown 100.3 fps | capture→shown p50 7.5 ms p95 8.1 ms (capture→send 7.5) |
decode … convert … | queue≤0 dropped 0 | pose … Hz | rtt 0.1 ms | pc: cap 99.9 fps, conv 3.6 enc 3.8 ms, cap→send p95 8.1, skip 0 idr 0
```
`capture` is the PC display's vblank at which the frame was on screen (KMS), buffer arrival
(PipeWire), DXGI present time, or generation time (test/file). `conv` = capture → encoder
input, `enc` = encoder in → AU out, `skip` = frames dropped because the encoder was still
busy, `idr` = forced IDRs.

Environment: `EYEPIECE_FILTER_DOWNLOAD=1` forces the libavfilter `hwdownload` path in the
converter (debugging).

## Stereo 3D

Stereo streaming to the goggles is not available: it would need a GFX1 plane on VOP1, which does not exist.
`--stereo --no-connect` still encodes side-by-side video (2560x720, at most 60 fps) without a goggles
connection.

## See also

- [Picture in picture and transparency](pip-and-transparency.md)
- [Head tracking into games](head-tracking.md)
- [Transports and USB bulk](transports.md)
- [Web UI](web-ui.md)
- [macOS](macos.md)
