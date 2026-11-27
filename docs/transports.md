# Transports and USB bulk transfers

`eyepiece-pc` reaches the bridge either through `adb forward` (TCP on the PC's loopback) or over a dedicated USB
bulk interface. The byte stream is the same; see the [bridge protocol](protocol/bridge.md#transports-and-framing).
`--transport auto` (the default) tries USB bulk first and falls back to adb; `--transport adb` and
`--transport usb-bulk` force one. On the goggles the bulk interface needs the bridge started with `-B`
([goggles side](goggles/bridge.md#usb-bulk-transport)).

## USB bulk

`--transport usb-bulk` selects the dedicated `Eyepiece bulk` interface without TCP
fallback. `auto` attempts bulk before falling back to adb forwarding, and reconnects
retain the initially selected transport. `--serial` selects the USB device.

The host uses the pure-Rust `nusb` library. Linux needs device-scoped USB permissions;
macOS needs the vendor interface free of other drivers; Windows needs WinUSB bound
only to the Eyepiece interface, never the ADB interface.

### Troubleshooting

An intermittent USB disconnect (on macOS reported as `IOKit 0xe00002ed`) means the USB device stopped
responding, not that the decoder crashed. Check the cable and hub, and compare with `--transport adb`.
Automatic selection does not switch transports after a connection is made.

### Hardware regression test

The ignored hardware regression exercises pane movement, resizing, packed alpha,
fullscreen/PiP transitions, GUI/pane opacity acknowledgements, resumed frame reports
and reconnect. Start a matching bridge with `-R -H` (add `-B` for bulk); run
only with stationary goggles, never while flying:

```sh
adb forward tcp:42420 tcp:42420
EYEPIECE_TEST_BRIDGE=127.0.0.1:42420 EYEPIECE_TEST_CODEC=hevc \
  cargo test -p eyepiece-pc --lib session::tests::device_pane_reconfiguration -- --ignored --nocapture
# Add EYEPIECE_TEST_TRANSPORT=usb-bulk to exercise the dedicated USB interface.
```
