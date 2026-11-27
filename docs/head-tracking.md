# Head tracking into games

Head tracking is **off by default**: the goggles' IMU stream only runs while a PC asks for it.
`--head-tracking` turns it on (`--no-video` and `--print-pose` imply it); the bridge stops it
again when the PC disconnects. `eyepiece-pc` then sends the goggles' yaw/pitch/roll (degrees;
+yaw right, +pitch up, +roll right ear down) to opentrack's **"UDP over network"** input on
`127.0.0.1:4242` (`--opentrack HOST:PORT`, `--opentrack ''` = off). The pose is sent as six little-endian
doubles (x, y, z, yaw, pitch, roll) with x, y and z zero.

In opentrack: *Input* = "UDP over network", port 4242. Set a centre key in opentrack, and use
`r` in eyepiece-pc to recenter the goggles' yaw. Then pick the output for your game:

- **Windows**: *Output* = **"freetrack 2.0 enhanced"**. It serves both the FreeTrack and the
  TrackIR (NPClient) interface, which most games with head-tracking support use.
- **Linux, games under Wine/Proton**: *Output* = **"Wine -- Windows layer for Unix"**, pointed
  at the game's prefix (Steam/Proton: choose the Proton version and the game's AppID, i.e.
  `steamapps/compatdata/<appid>/pfx`). opentrack then provides the FreeTrack/TrackIR
  interface inside that prefix. Start opentrack before the game.
- Native Linux games with opentrack/UDP support can read the UDP stream directly.

## OpenXR through Monado (Linux, experimental)

`eyepiece-xr` (`crates/eyepiece-xr`) forwards the pose that eyepiece-pc already sends to opentrack into Monado's
`remote` driver, so OpenXR applications can use the goggles' head tracking. It listens for the opentrack UDP
packets on `127.0.0.1:4243` and sends Monado's remote-driver packets to `monado-service` on `127.0.0.1:4242`
(TCP), together with the goggles' optics (default 44° diagonal field of view per eye and 63 mm IPD):

```sh
eyepiece-pc --head-tracking --opentrack 127.0.0.1:4243
P_OVERRIDE_ACTIVE_CONFIG=remote monado-service        # started only when VR is used
eyepiece-xr                                           # see --help for --fov, --ipd, --height, --predict, --test
```

`--test` sends a slow synthetic yaw/pitch sweep for checking the setup without the goggles.

The head-tracking messages are described in [the bridge protocol](protocol/bridge.md#telemetry-and-clock-sync).
