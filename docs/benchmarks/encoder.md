# macOS encoder measurements

Host-side capture and encode latency on macOS, used to choose the encoder defaults described in
[macOS](../macos.md). Receiver, goggles and end-to-end display latency are **not** included, and nothing here
establishes equal image quality between encoders.

Setup: MacBook Pro M4 Pro (14 CPU cores, 48 GB), macOS 26.6.2, FFmpeg 9.0.2, release builds, 1920x1080,
20 Mbit/s, nominal 100 fps, GOP 200, the repository's encoder defaults. Single-run or three-run medians; treat the
numbers as indicative.

## Results in short

- **Default: VideoToolbox hardware encoding.** It uses about 0.12 CPU cores in the longer runs, against about 1.56
  for x264.
- **Lowest host latency: x264 `superfast` + `zerolatency`** on the BGRA capture and CPU conversion path:
  4.50 / 5.16 / 5.68 ms p50 / p95 / p99 capture-to-packet, against 8.74 / 9.65 / 10.40 ms for VideoToolbox with
  NV12 readback. Both delivered about 111 fps for 120 requested. Select it with `--encoder x264`.
- A VideoToolbox stream at the same nominal 20 Mbit/s produced a much lower actual bitrate than x264 (about 3.4
  against 17 Mbit/s on desktop content), so bitrate and quality are not comparable.
- Requesting 100 or 120 fps delivers about 91 or 110 fps on this 120 Hz display: capture cadence, not the
  encoder, limits the rate.
- Native VideoToolbox handoff buffered about four frames (44 to 70 ms capture-to-send). Reading the
  ScreenCaptureKit-scaled NV12 surface back to the CPU before encoding costs about 3.2 ms and brings the total to
  about 8 to 9 ms, so the default path does that.
- With the virtual display set to native 1920x1080 backing pixels, no rescaling is needed; at other display modes
  ScreenCaptureKit scales.

## Unpaced encoding

600 frames after 60 warm-up frames; pattern generation and conversion are excluded from the encode timings.

| Encoder | Open ms | Encode p50 / p95 ms | Packet p50 / p95 ms | Pipeline fps | Mbit/s |
|---|---:|---:|---:|---:|---:|
| x264, superfast/zerolatency | 1.7 | 1.08 / 1.23 | 1.08 / 1.23 | 450.3 | 18.9 |
| VideoToolbox H.264 | 183.1 | 4.74 / 4.92 | 4.74 / 4.92 | 155.0 | 20.0 |
| x265, ultrafast/zerolatency | 8.5 | 2.93 / 3.84 | 7.23 / 8.77 | 226.0 | 19.9 |
| VideoToolbox HEVC | 195.7 | 5.19 / 5.39 | 5.19 / 5.39 | 144.4 | 20.0 |

x265 warns about VBV underflow on keyframes. VideoToolbox rejects `prio_speed=1` on this machine, so the hint has
no effect. `AV_CODEC_FLAG_LOW_DELAY` selects Apple's low-latency rate-control encoder.

## Live ScreenCaptureKit

Capture-to-packet latency (callback arrival to encoded packet), full-display desktop content with an animation
running, 120 Hz display. Excludes goggles, network, decode and display.

| Requested fps | Encoder and input | Delivered fps | p50 / p95 / p99 ms | Process CPU % |
|---:|---|---:|---|---:|
| 60 | x264 superfast, NV12 readback | 57.92 | 5.36 / 8.47 / 10.32 | 68.4 |
| 60 | VideoToolbox, NV12 readback | 57.61 | 8.97 / 9.84 / 10.23 | 7.5 |
| 60 | x264 superfast, BGRA | 57.68 | 5.25 / 5.94 / 7.42 | 93.1 |
| 60 | VideoToolbox, BGRA | 57.96 | 9.00 / 10.74 / 11.76 | 37.6 |
| 100 | x264 superfast, NV12 readback | 91.77 | 5.05 / 7.28 / 7.86 | 92.9 |
| 100 | VideoToolbox, NV12 readback | 91.14 | 8.90 / 10.12 / 10.23 | 10.6 |
| 100 | x264 superfast, BGRA | 91.79 | 4.64 / 6.38 / 6.78 | 111.7 |
| 100 | VideoToolbox, BGRA | 91.50 | 8.41 / 10.77 / 11.21 | 44.6 |
| 120 | x264 superfast, NV12 readback | 109.95 | 4.62 / 7.55 / 8.20 | 103.0 |
| 120 | VideoToolbox, NV12 readback | 110.69 | 8.28 / 10.44 / 11.03 | 10.7 |
| 120 | x264 superfast, BGRA | 109.64 | 4.33 / 6.25 / 7.62 | 121.0 |
| 120 | VideoToolbox, BGRA | 109.73 | 8.14 / 10.27 / 11.34 | 49.2 |

x264 stays ahead when the NV12 readback pipeline is matched, and BGRA is faster still for x264 because it avoids
the NV12 hardware-frame download. For VideoToolbox, BGRA uses much more CPU without a dependable latency gain.
Delivered fps is about equal within each rate, because capture paces both.

## Reproducing

```sh
# Encoder microbenchmark: alias, codec, nominal fps, measured frames [, input mode: test|native|readback]
cargo run --release -p eyepiece-av --example encoder_bench -- x264 h264 100 600
cargo run --release -p eyepiece-av --example encoder_bench -- videotoolbox h264 100 600

# Live pipeline without goggles
cargo build --release -p eyepiece-pc
target/release/eyepiece-pc --source screencapturekit --no-connect \
  --encoder videotoolbox --fps 100 --duration 10 --stats-interval 2
target/release/eyepiece-pc --source screencapturekit --no-connect \
  --encoder videotoolbox --cpu-scale --fps 100 --duration 10 --stats-interval 2
target/release/eyepiece-pc --source screencapturekit --no-connect \
  --encoder x264 --fps 100 --duration 10 --stats-interval 2
```

`crates/eyepiece-av/examples/run_encoder_experiments.mjs` drives the full comparison (with the helpers beside it)
and writes JSON under `target/`; run it only when you need the matrix.
