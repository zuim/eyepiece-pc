# Picture in picture and transparency

The stream can fill the panel or sit in a movable pane over the goggles' own video and GUI. The pane is a
GFX1 plane between the radio video and the DJI GUI. How the pane reaches the goggles is described in
[the bridge protocol](protocol/bridge.md#configure); how the display composes layers in
[display composition](goggles/display-composition.md).

```sh
eyepiece-pc --pane 960x540+480+270 --color-key 00ff00     # keyed PiP
eyepiece-pc --alpha source                                  # full panel, source alpha
eyepiece-pc --color-key 00ff00                              # full panel, keyed
eyepiece-pc --gui-opacity 50 --pane-opacity 80
```

## Pane geometry

- Without `--pane`, the pane is the entire 1920x1080 panel.
- With `--pane WIDTHxHEIGHT+X+Y`, the values are panel pixels; width and height must be even and the pane must
  fit the panel.
- The stream is encoded at the pane size. The goggles have no scaler, so the PC scales every source to fit.
- Stereo output is unavailable (see [usage](usage.md#stereo-3d)).

## Opaque and transparent panes

| Setting | Stream | On the goggles |
|---|---|---|
| default (`--alpha opaque`, no colour key) | normal-width video | decoder NV12 scanned out directly, hardware global alpha |
| `--alpha source` and/or `--color-key` | colour and a grayscale alpha mask side by side (twice the pane width) | the bridge reconstructs ARGB and blends per pixel |

- `--alpha opaque` ignores input alpha. `source` needs a source that has alpha.
- `--color-key RRGGBB` keys either mode. The key is matched in RGB before scaling.
  `--key-similarity` (0..1, default 0 = exact RGB) sets the tolerance and `--key-blend` (0..1, default 0 = hard
  key) the softness inside it. Colours outside the tolerance stay opaque.
- Scaling and lossy compression can soften the mask. Letterbox bars are transparent in a transparent pane and
  black in an opaque one.
- Transparency needs CPU capture frames: DRM/VAAPI and native VideoToolbox surfaces are rejected for it, and
  ScreenCaptureKit switches to CPU capture. `--vf` is not supported with transparency.
- `--pane-alpha` is an alias for `--alpha`.
- Preview and `--dump` show the packed frame when transparency is on.
- A fullscreen transparent pane is a 3840x1080 stream; check that the goggles' decoder and conversion keep up
  before relying on it.

## Opacity

- `--gui-opacity 0..100` (default 100) sets the DJI GUI plane's opacity, separately from video transparency.
  It needs a bridge advertising `GUI_ALPHA`. tinkerglass's PC Display does not: it hides the DJI GUI completely
  while a stream is configured, so the option is ignored with a notice there (the stream still starts). The
  setting stays while a reconnect bridge retains a frame; otherwise the goggles return to stock when the
  bridge releases the GUI or exits.
- `--pane-opacity 0..100` (default 100) fades the pane. It multiplies source and colour-key alpha, so
  transparent pixels stay transparent. The fade happens on the goggles, so preview and dumps are unchanged. It
  needs a bridge advertising `PANE_ALPHA`.

Both apply live from the [web UI](web-ui.md), without restarting the decoder. See
[the protocol](protocol/bridge.md#opacity) for the requests.

## Bridge requirements and reconnecting

- Deployment starts the bridge with `-1` (it exits with the session). tinkerglass's PC Display needs no `-R` for a
  pane; with `--no-deploy`, start the bridge from More > PC Display.
- A bridge without `PANE_RECONNECT` (tinkerglass's) retains nothing: a disconnect releases the display, and
  `--reconnect` sets the pane up again on reconnect.
- With `--reconnect` the bridge keeps the last image on screen while disconnected. Reconnecting with the same
  configuration resumes the stream; a different geometry, pixel format or codec replaces the retained stream, and
  the image can disappear while the new decoder starts. Stop the persistent bridge when finished.

## Live changes

With a bridge that advertises reconfiguration support, pane position, size and the opaque/transparent format
change without restarting the session, the bridge or the connection; each change sends one `Configure` and
resumes with a keyframe. Colour-key and source-alpha changes apply live while the stream is already
transparent with the same geometry. A bridge without the capability leaves the setting pending for a restart.
Changing the codec or encoder always needs a restart.
