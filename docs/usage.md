# Command-line usage

`eyepiece-pc` is a command-line program with the [web UI](web-ui.md) running alongside on the same engine.
Run `eyepiece-pc --help` for every option.

## Web UI, command-line mode and tray

- `eyepiece-pc` streams at once and serves the web UI at http://127.0.0.1:4174/; `--open-web` (`--open`)
  also opens it in the browser. The terminal shows log lines and stream state; statistics are in the web UI.
  Options given on the command line are saved as web UI settings, so the next start uses them.
- `--no-web` is the plain command-line mode: only the given options, nothing saved, the statistics line on
  stderr, exit when the stream ends. One-run options (`--list`, `--no-connect`, `--dump`, `--frames`,
  `--duration`, `--preview`, `--print-pose`) select it by themselves, so a benchmark never changes the saved
  settings; `--web` keeps the web UI with them.
- On Windows, a start without a terminal of its own (double-click, shortcut) hides the console window, opens
  the web UI and shows a notification-area icon: click opens the web UI, the menu starts or stops the stream
  and quits. It does not start a stream by itself, but resumes one that was running when it was last closed.
  A second double-click opens the running instance. `--tray` shows the icon from a terminal too.
- Settings live in `%APPDATA%\eyepiece\config\eyepiece-rpc.json` on Windows, `~/.config/eyepiece/` on Linux
  (`$XDG_CONFIG_HOME`) and `~/Library/Application Support/eyepiece/` on macOS; `EYEPIECE_CONFIG_DIR`
  points elsewhere (portable installs, tests). The path is printed at start.

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
eyepiece-pc --transport usb-bulk                    # only the tinkerglass USB video interface
eyepiece-pc --no-deploy                             # use a bridge that is already running
```

By default eyepiece-pc starts the bridge itself, so there is no need to open PC Display in the goggles'
menu first (a bridge started there hands over by itself). It asks tinkerglass's menu service to launch the
installed PC Display bridge (`/vendor/tinkerglass/current/mods/pcdisplay/bin/pcdisplay_bridge`) with `-1 -l <log>`
plus any `--bridge-args`; nothing is pushed to the goggles. The bridge exits with the PC session and logs to
`/data/vendor/tinkerglass/run/pcdisplay/bridge.log`. Without the mod, or with the tinkerglass mods not
running, eyepiece-pc stops with a message saying so.

`--no-deploy` uses a bridge that is already running (for example started from More > PC Display).

## Resolution, input and statistics

By default the stream is 1920x1080, the goggles' panel size: they have no display scaler.
Every source is scaled to fit on the PC and letterboxed with black bars (`--no-letterbox`
stretches instead). A [picture-in-picture pane](pip-and-transparency.md) is encoded at its own size.

Mouse pointer (Windows, DXGI): the captured frames carry no hardware cursor, so eyepiece-pc draws it from
the pointer position and shape Desktop Duplication reports (monochrome, colour and masked-colour shapes).
`--dxgi-cursor auto` (default) draws it only when the image does not already show it (a software cursor),
`on` always, `off` never. `cargo run -p eyepiece-capture --example dxgi-cursor -- auto 5 [output]` shows
which case a machine is in.

Stdin (both modes): `r` recenter, `i` force an IDR, `q` quit (each + Enter). Ctrl-C quits cleanly
(twice: hard). `--dump out.h264` also writes the exact Annex-B stream; `--no-connect
--duration 10` encodes without a bridge (command-line mode). See `eyepiece-pc --help` for all
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
