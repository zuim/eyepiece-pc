# Web UI

`eyepiece-pc` serves a local browser interface (TypeScript and Svelte) for controlling the stream, with
live settings, statistics, the head pose and an encoded preview, whenever it runs (unless `--no-web`). It
talks to the same engine as the command line through the [RPC protocol](protocol/rpc.md).

```sh
target/release/eyepiece-pc --open-web    # opens http://127.0.0.1:4174/ in the browser
```

`EYEPIECE_PORT` changes the loopback port. CLI options override and persist the corresponding saved
settings ([usage](usage.md#web-ui-command-line-mode-and-tray) has the modes, the Windows tray icon and where
the settings are kept). The static app is read from `web/` next to the executable, else from `web/build` of
the checkout the binary was built from, so it can be started from any directory; `EYEPIECE_WEB_DIR` points it
elsewhere. A development build without a built app falls back to the command-line mode with a note.

## Using it

The tabs are **Overview**, **Statistics**, **Compare**, **Settings** and **Log**. Basic stream controls
(start or stop, restart) and the capture, connection and video selectors stay above the tabs. With the
*File* capture, **Browse…** opens the system's file dialog on the PC (eyepiece-pc shows it; the page only
gets the path back) and the path field accepts a typed or pasted path. There is no keyframe button: the
stream sends a keyframe every two seconds (`--gop`), on every connect and whenever the goggles or the
preview need one, and `i` + Enter in the terminal still forces one.

## Discovery from tinkerglass

`GET /api/hello` answers `{"app": "eyepiece-pc", "version": …, "url": …}` with a CORS header for
tinkerglass's web app (`https://tinkerglass-org.github.io`, the `*.tinkerglass-prs.pages.dev` previews and any
`http://localhost` / `http://127.0.0.1` origin of its development server); `EYEPIECE_DISCOVERY_ORIGINS` adds
comma-separated origins. It carries no token and no settings, so tinkerglass's PC Display panel can show
whether eyepiece-pc runs instead of a plain link. Browsers with Local Network Access (Chrome) ask the user
before a public site may reach `127.0.0.1`, so the panel should probe only after a click, or when that
permission is already granted.

- **Overview** shows the stream preview and the head pose, plus the graphical picture-in-picture editor and the
  transparency tuning. Drag the pane or its corner to move or resize it; pixel fields allow precise keyboard
  control. Enabling transparency defaults to *Key Color* with a green key. GUI and pane opacity are always
  available here and in Settings.
- **Statistics** shows per-stage p50 and p95 latency, throughput and 120-second latency and rate charts.
  Statistics use at least a one-second rolling window; the reporting interval (default 0.1 s, 10 Hz, for new
  configurations; adjustable in Settings) only controls how often they update. Missing reports are gaps in the
  charts.
- **Compare** pins capture the running configuration and the mean telemetry over a chosen window.
- **Settings** are grouped by function. The equivalent command line updates as settings change and can be
  copied, or edited and applied; omitted options take their CLI defaults. *Reset to Defaults* (also
  *Reset Settings* beside the start button) asks for confirmation, restores the web defaults and
  can be undone for 20 seconds.
- **Log** shows session and bridge log lines.

Settings are owned and persisted by `eyepiece-pc`, not by the browser, and a stream that was running resumes
when the server restarts. The browser stores only view preferences and comparison pins. New web
configurations enable cursor capture and head tracking by default; saved values and CLI defaults are unchanged.
The settings schema in each state carries the choices this machine supports and hides settings that do not
apply to the chosen source, encoder or connection.

## Live changes

While a stream runs:

- file and test sources can be switched between each other, or updated, without restarting;
- colour key and source alpha change live when the stream is already transparent with the same geometry;
- with a bridge that supports reconfiguration, pane geometry and the opaque or transparent format change
  without restarting the bridge;
- GUI and pane opacity and head tracking apply immediately.

A change that cannot be applied live, including any live capture change with VAAPI encoding, leaves the
previous capture active and the setting marked as pending; a **Restart** button appears beside the stream
controls. See the [RPC protocol](protocol/rpc.md#session-phases-and-live-updates) for the full list.

## Preview

The preview needs a browser with WebCodecs and H.264 support. On subscribing the browser reports which codecs
it can decode. The server forwards the original codec when supported; otherwise a separate bounded worker
decodes and re-encodes the preview with a compatible software encoder, and the goggles' stream is unchanged.
Subscribers share one codec all of them support; a subscription that would leave none is rejected without
changing the others. Packed-alpha streams are previewed by rebuilding transparency from the right-hand
grayscale mask and drawing the result over a checkerboard at the pane's aspect ratio.

Preview backpressure is bounded: the stream is never blocked by transcoding or browser decoding. The UI reports
unavailable WebCodecs, unsupported codecs and decoder errors. The colour-key editor has its own small live
preview, independent of the Overview toggle; it uses continuous frames even when Overview is set to keyframes
only, and needs a running stream.

## Building and developing

Needs Node.js 24 or newer and pnpm. Build the CLI and the app:

```sh
cd web
pnpm install --allow-build=esbuild --allow-build=protobufjs
pnpm rpc:generate        # TypeScript bindings from ../contract/eyepiece-rpc.proto
pnpm check
pnpm test
pnpm build
cd ..
cargo build --release -p eyepiece-pc
```

For Vite development, start the server from the repository root with
`EYEP_DEV_ORIGIN=http://127.0.0.1:5173 target/release/eyepiece-pc --web`, then run `pnpm dev` in `web`; the
dev server proxies `/api` and WebSocket requests to port 4174.

Browser workflow, layout and accessibility tests use Chromium and a mock RPC worker, and do not touch a
device or change settings:

```sh
cd web
pnpm exec playwright install chromium
pnpm test:browser
```

The Rust bindings are generated at build time from the same contract, and
[`contract/settings.json`](../contract/settings.json) is the single settings schema. The web server is local
tooling: see [Security](protocol/rpc.md#security) before exposing it.
