# Transports and USB bulk transfers

`eyepiece-pc` reaches the bridge either over the goggles' tinkerglass video interface (USB bulk) or through
`adb forward` (TCP on the PC's loopback). The byte stream is the same; see the
[bridge protocol](protocol/bridge.md#transports-and-framing). `--transport auto` (the default) uses the USB
interface when it is present and falls back to adb otherwise (for example in a stock USB configuration);
`--transport adb` and `--transport usb-bulk` force one. The interface exists in tinkerglass's USB mode, which
the goggles enter by themselves after the mods start; the bridge needs no option for it.

## USB bulk

`--transport usb-bulk` selects the *tinkerglass video* interface without TCP fallback. `auto` attempts it
before falling back to adb forwarding, and reconnects retain the initially selected transport. `--serial`
selects the USB device.

The interface is found by USB id 2CA3:0020 and interface class/subclass/protocol `ff/54/03`, never by its number;
its one bulk OUT and one bulk IN endpoint are found by direction. The goggles are high-speed only (512-byte
packets). The other tinkerglass interfaces (control, protocol 1 and 2) belong to the web app and are never
opened. Only one program can hold the video interface: a second `eyepiece-pc` fails to claim it and says so.

The host uses the pure-Rust `nusb` library. Windows binds WinUSB by itself (the interface carries a Microsoft OS
descriptor), so no driver step is needed. Linux needs device-scoped USB permissions (a udev rule for 2CA3:0020).

### Troubleshooting

An intermittent USB disconnect (on macOS reported as `IOKit 0xe00002ed`) means the USB device stopped
responding, not that the decoder crashed. Check the cable and hub, and compare with `--transport adb`.
Automatic selection does not switch transports after a connection is made.

### Hardware regression test

The ignored hardware regression exercises pane movement, resizing, packed alpha,
fullscreen/PiP transitions, GUI/pane opacity acknowledgements, resumed frame reports
and reconnect. Start a matching bridge (`pcdisplay_bridge -H`); run
only with stationary goggles, never while flying:

```sh
adb forward tcp:42420 tcp:42420
EYEPIECE_TEST_BRIDGE=127.0.0.1:42420 EYEPIECE_TEST_CODEC=hevc \
  cargo test -p eyepiece-pc --lib session::tests::device_pane_reconfiguration -- --ignored --nocapture
# Add EYEPIECE_TEST_TRANSPORT=usb-bulk to exercise the dedicated USB interface.
```
