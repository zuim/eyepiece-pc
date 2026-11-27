# Goggles display composition

How the DJI Goggles 3 compose their display and how Eyepiece puts PC video on it. Everything here describes
**firmware 01.00.1600** and is not assumed to hold on other firmware. For the bridge that implements it see
[the goggles bridge](bridge.md); the raw device inventory is in [the device dump](device-dump.md).

The GUI backdrop section below describes the natartos-era hook. tinkerglass's PC Display hides the DJI GUI
completely while a stream is configured instead (through tinkerglass's display service), so none of it applies there.

- [Goals](#goals)
- [Display stack](#display-stack)
- [Pixel formats](#pixel-formats)
- [How the stock apps render](#how-the-stock-apps-render)
- [GUI backdrop suppression](#gui-backdrop-suppression)
- [Transparency transport](#transparency-transport)
- [Decoder lifecycle](#decoder-lifecycle)
- [Conversion performance](#conversion-performance)

## Goals

- Composite PC video above the goggles' radio video.
- Support full-screen video and smaller picture-in-picture (PiP) regions, scaled by the PC.
- Keep the normal DJI GUI and its controls.
- Support transparency, and keep the overlay visible when there is no radio video.

## Display stack

VOP0 outputs 1920×1080 at about 100 Hz.

| Plane | Layer | Order | Stock owner | Format |
|---|---|---:|---|---|
| 4 | VIDEO0 | 0, bottom | `dji_media_server` | NV12 radio video |
| 6 | GFX1 | 1 | `dji_arhome` | ARGB8888 |
| 5 | GFX0 | 2, top | `dji_glasses` | ARGB8888, pixel alpha |

- The order is what this firmware reports, not something to assume elsewhere. WRB is a writeback path, not another
  overlay input.
- There is no display-plane scaler, so the PC scales every stream to the size it asks the bridge for. A stream
  smaller than the panel is shown 1:1, centred.
- Eyepiece uses VIDEO0 for panel video and GFX1 for panes, which sit between the radio video and the GUI.
- `dji_arhome` owns GFX1 by default and may provide AR markers and motion-controller functions. The bridge stops
  it only while it uses GFX1 and restores it afterwards (see [GFX1 panes](bridge.md#gfx1-panes)).
- GFX1 accepts global alpha and per-pixel alpha with blending order 1. GFX0 is the GUI with pixel alpha.

## Pixel formats

`libduml_fb.so` maps the pixel formats to display format identifiers:

| Format | Identifier | Alpha-capable on GFX1 |
|---|---:|---|
| `DUSS_PIXFMT_ARGB8_I32` | **20015** | yes: the stock format |
| `DUSS_PIXFMT_RGB888_I32` | 20002 | no |
| `DUSS_PIXFMT_RGB565` | 20012 | no |

Format identifiers from other goggles generations (for example the public msp-osd producer's) do not apply:
20012 and 20002 are not ARGB here. That producer also uses a different framebuffer ABI and blending algorithm.

For 20015, `fb_generate_frame_info` computes a 4-byte-per-pixel row aligned to the requested horizontal
alignment. It writes stride at `0x40`, zero at `0x44`, aligned height at `0x4c`, plane byte size at `0x50`, plane
count 1 at `0xb0`, dimensions at `0x38` and `0x3c`, and the format at `0x28`; the buffer handle goes at `0x20` via
`fb_mem_alloc_with_info`. For 1920×1080 with the stock alignments the stride is 7680 and the size 8,294,400
bytes. A successful `push_frame` return does not by itself prove the plane shows alpha.

Decoder output (NV12) carries no per-pixel alpha, so alpha panes are converted to ARGB. The exact ARGB channel
order and alpha math have not been verified quantitatively. `mods/pcdisplay/bridge/gfx1_alpha_test.c` in tinkerglass is
a standalone probe that submits an ARGB pattern to GFX1 (`bridge/build.sh alpha-test`).

## How the stock apps render

From static inspection of binaries copied from the device:

- `dji_arhome` imports Embedded Wizard drawing functions and `EwGuiBspInitFromFile`, and reads
  `/system/etc/gui/zv90x_ar_disp.json`, which configures 1920×1080 `DUSS_PIXFMT_ARGB8_I32` for render and
  display, GFX1, LCDC composition, blending enabled, global alpha disabled and order 1.
- `dji_glasses` loads `libew_gls.so`, `libewrte_gfx_soft.11.00.so` and `libewplatform_unity.so`. Its display
  configurations specify the same ARGB format, GFX0, blending enabled, global alpha disabled and order 2. The
  exact config file in use was not established; the active GFX0 reports `argb8888`, pixel alpha, order 2.
- The shared Embedded Wizard renderer updates double buffers. `EW_PLATFORM::DispAgent` allocates two framebuffer
  descriptors via `dfb_alloc_buffer_alignment` (256-byte horizontal and 1-pixel vertical alignment), maps their ION
  buffers and gives the addresses to the GUI platform. Frames pass through `OutputRedirector` and the compositor;
  the LCDC compositor calls the display engine, which sets plane blending and calls
  `duss_hal_display_push_frame`. Alternate redirection and conversion paths exist.

Evidence: `libduml_fb.so` SHA-256 `ed7e912d70dde5450b7e9ac27416f7b3dfb41830285a0ac361fdfc03220470b9`,
`libewplatform_unity.so` `073851dd8c3be08436344df54b8edd36ecd22c3cd00fcfe5b5a4167795072cb5`, AR config
`c117105fc1ddfab1e7807d352f5efaa4852653c6c48d0737a8d5cec9ca058400`.

## GUI backdrop suppression

With no air-unit video the goggles' UI draws a background that hides GFX1 and VIDEO0. While the UI is hidden
(GUI opacity 0, the default) this does not matter. With a GUI opacity above 0 the background shows over the
picture at that opacity; tinkerglass does not suppress it yet. natartos' Eyepiece mod did, with a hook that only
worked for the `libew_gls.so` build with ID `54323da7526004b9e5c3d75d172539e5` (SHA-256
`3021cebd64993711a922647afed2b0694c74db0004bef6a7e3e8804cfb9cff61`). This section records how that layer works.
Offsets below are ELF virtual addresses for that binary and are not portable.

**The layer.** `ZV902Application__Init` calls `DuFPVApplication__Init`, which initialises a
`DuBackgroundBackgroundLayer` **embedded** at root + `0x1020` and adds it to the root group. Its image child is
assigned the glasses' background external bitmap (`DuGlassesResources__GetBackgroundExternalBitmap`).

**Its Active outlet** (built at `0x856c38..0x856e6c`):

```text
VSTOffState.Active AND
    (((FPVState.Active OR MenuState.Active OR Drawer.ContentActive)
      AND NOT CommonStatus.IsUavConnectedProp)
     OR (Upgrade.UpgradeStatus == 3))
```

The aircraft-connected property is not necessarily the same as "a radio frame is displayed", and there is an
upgrade-status branch, so the property must not be faked globally.

**Two visibility paths.** `MxLayerLayer_onUpdate` reads the outlet and calls `CoreGroup__OnSetVisible` directly,
and `DuBackgroundBackgroundLayer_onActive` calls it with the byte at layer + `0x311`. Both dispatch through VMT slot
`0xd8`, which resolves to `CoreGroup_OnSetVisible`, so hooking `onActive` alone cannot suppress the layer. The mod
replaced only this class's visibility slot with a wrapper that remembered the latest stock request, forced the layer
invisible only while PC video was displayed, and restored the request afterwards. `layer + 0x311` and the global
aircraft property were left alone.

**GUI-thread work.** `EwGuiRun` creates a GUI task whose loop calls `EwProcessSignals` through the PLT at
`0x968a58` (relocation `0x6cdff50`; the only call site) before `CoreRoot__DoesNeedUpdate` and rendering. The hook
applied pending hide and restore requests there, on the GUI thread, with the original setter. A worker thread must
never call the setter itself: cross-thread Embedded Wizard calls hang. (tinkerglass's core.display runs its own
UI-thread work at the same place.)

## Transparency transport

Colour and a grayscale alpha mask are packed side by side in one ordinary H.264 or HEVC frame, and the goggles
reconstruct BGRA from it. Layered HEVC alpha is not available on the goggles' decoder, and ordinary decoders may
discard the auxiliary layer.

- The PC converter generates packed frames from CPU input, preserving source alpha or combining it with colour keying.
- The bridge reconstructs straight-alpha BGRA at half the stream width from the packed NV12 or I420 frame, using the
  right half's luma (16 transparent, 235 opaque) as alpha and limited-range BT.709 for colour (`yuv2bgra.c`).
- CPU reads of uncached decoder buffers are slow (about 430 ms per full-HD frame), so the bridge stages the needed rows
  into cached memory first and converts with NEON.

## Decoder lifecycle

Constraints the bridge's decoder handling is built around:

- Reopening the decoder needs ION released first: stop the decoder, close it, free its buffers, then stop and close
  ION. Reopening without this fails (`ion0 open/start: -1004`).
- The HAL stop call needs its configuration argument; omitting it crashes.
- Stopping takes 13 to 26 ms and opening 9 to 15 ms. Size and format changes wait for submitted frames and returned
  frames to drain before stopping, because stopping under load can time out waiting for end of stream.
- Position-only pane updates move GFX1 without touching the decoder.
- Opaque panes skip conversion and push decoder NV12 frames to GFX1. Alpha panes need the ARGB conversion, because
  only ARGB planes carry per-pixel alpha.

## Conversion performance

On the ARMv8 device, four threads, cached NV12 to ION BGRA, display excluded:

| Size | Scalar | NEON |
|---|---:|---:|
| 640×480 | 8.5 ms | 1.9 ms |
| 1920×1080 | 51 to 60 ms | 14.4 to 14.9 ms |

With decoder input and BGRA output both in ION, staging only the needed luma and chroma rows in parallel before the
NEON conversion takes 7.4 ms per 640×480 frame and 46.6 ms at 1920×1080. These times exclude decoder synchronisation,
encoding and display.

Opaque panes avoid the conversion: a 1920×1080 HEVC full-panel stream at 100 fps shows at 100 fps with 11 ms p50
capture-to-shown, and a 552×300 pane at 60% opacity takes about 4.5 ms.
