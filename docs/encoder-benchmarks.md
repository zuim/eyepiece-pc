# macOS Encoder Benchmarks

## Latest Findings

**macOS default policy: prefer VideoToolbox hardware encoding.** `auto` tries
hardware-only VideoToolbox first, preserving CPU headroom; explicit `--encoder x264`
selects the software pipeline. Both production paths match their winning live
references; see [production reference alignment](#production-reference-alignment).

**Lowest measured host latency: x264 `superfast` + `zerolatency`, using the
existing BGRA capture/CPU conversion path.** The
[matched live comparison](#x264-versus-videotoolbox-live-comparison) measures
**4.50 / 5.16 / 5.68 ms p50/p95/p99** versus VideoToolbox NV12 readback
**8.74 / 9.65 / 10.40 ms**, both about **111 fps** delivered at 120 fps requested.
x264 costs about **1.56 CPU cores** versus **0.12** for VideoToolbox in those
longer runs. This supersedes the earlier preference for copy-free VideoToolbox
when low latency and high FPS take precedence over avoiding copies. It is an
explicit alternative, not a request to change the hardware-first default.
Receiver latency and equal-quality comparisons remain unverified.

The [completed experiment matrix](#completed-experiment-matrix) below supersedes
the earlier single-run recommendations. It includes 46 native variants and 10
software/FFmpeg variants, each repeated three times, plus an injected timeout
check. Historical measurements remain below for comparison.

- [Virtual 1080p display](#virtual-1080p-display): native capture **7.34 ms**, with
  matching 1920x1080 input/output and scaling disabled. The display was restored
  to its original 1600x900 mode. Observed capture was about 60 fps, not 100 fps.
- Source-correct window capture: **6.65 ms** median callback-to-packet, versus
  **9.42 ms** full display. Crop: **7.40 ms**. No application framebuffer readback.
- 720p: **5.55 ms**, at reduced resolution. Native 1080p HEVC and BGRA did not win.
- Corrected FFmpeg completion interposer: **9.54 ms** at 100 fps, versus
  **33.26 ms** ordinary native FFmpeg delivery. Copied NV12: **8.55 ms**.
- One reference buffer roughly halves delivered FPS; AQ/speed/peak-rate/lookahead
  hints were rejected by the selected low-latency encoder. Do not infer effects
  from rejected settings. All-intra property acceptance did not guarantee all-I output.
- Receiver-dependent experiments and actual power/visual-quality measurements
  remain blocked or unmeasured, with reasons in the coverage table.

Measured 2026-11-23 on a MacBook Pro M4 Pro (14 CPU cores, 48 GB RAM), macOS
26.6.2, FFmpeg 9.0.2 / libavcodec 63.1.102. Release builds, moving 1920x1080
testsrc2, 20 Mbit/s, nominal 100 fps, GOP 200, repository encoder defaults.
Different codecs/presets do not imply equivalent visual quality.

## Unpaced Encoding

Each run discards 60 warmup frames and measures 600 frames. Pattern generation
and conversion are excluded from encode-call timings but included in pipeline
throughput. Packet latency includes delayed output; the encoder is flushed and
every measured packet must be returned. These are single-run measurements, not
statistical guarantees. Unpaced throughput differs from paced capture latency.

| Encoder | Open ms | Encode p50 / p95 ms | Packet p50 / p95 ms | Pipeline fps | Stream Mbit/s |
| --- | ---: | ---: | ---: | ---: | ---: |
| x264, superfast/zerolatency | 1.7 | 1.08 / 1.23 | 1.08 / 1.23 | 450.3 | 18.9 |
| VideoToolbox H.264 | 183.1 | 4.74 / 4.92 | 4.74 / 4.92 | 155.0 | 20.0 |
| x265, ultrafast/zerolatency | 8.5 | 2.93 / 3.84 | 7.23 / 8.77 | 226.0 | 19.9 |
| VideoToolbox HEVC | 195.7 | 5.19 / 5.39 | 5.19 / 5.39 | 144.4 | 20.0 |

All four returned 600/600 measured packets. x265 warned about VBV underflow on
keyframes. Apple rejected `prio_speed=1`; this hint has no effect on this device.
VideoToolbox remains hardware-only (`allow_sw=0`); `AV_CODEC_FLAG_LOW_DELAY`
selects Apple's low-latency rate-control encoder, and `realtime=1` remains set.

```sh
cargo run --release -p eyepiece-av --example encoder_bench -- x264 h264 100 600
cargo run --release -p eyepiece-av --example encoder_bench -- videotoolbox h264 100 600
cargo run --release -p eyepiece-av --example encoder_bench -- x265 hevc 100 600
cargo run --release -p eyepiece-av --example encoder_bench -- videotoolbox hevc 100 600
```

Arguments: encoder alias, codec, nominal fps, measured frame count, optional input
mode (`test`, `native`, or `readback`). Screen modes require macOS and fresh
capture frames; they time out if the screen supplies too few frames. Dimensions
and bitrate stay fixed for comparisons. Encode-call budget is not asynchronous
encoder capacity; use actual pipeline throughput and packet timing together.

## Live ScreenCaptureKit

H.264, 1920x1080 output, 20 Mbit/s target. Screen content changed during terminal
output; it was not a controlled full-motion workload. Times use callback arrival,
not compositor presentation, and exclude goggles/network/decode/display latency.
Values below are steady-state windows after encoder startup.

| Path | Requested fps | Delivered fps | Encoder p50 ms | Capture-to-send p50 / p95 ms |
| --- | ---: | ---: | ---: | ---: |
| Original native NV12 handoff | 60 | 57.1 | 69.6 | 69.7 / 72.8 |
| Original native NV12 handoff | 100 | 90.2 | 44.4 | 44.4 / 46.3 |
| NV12 readback before hardware encoding | 60 | 57.5 | 5.1 | 8.3 / 9.4 |
| NV12 readback before hardware encoding | 100 | 90.8 | 5.6 | 8.8 / 9.5 |
| BGRA CPU capture/scale, VideoToolbox | 100 | 90.8 | 4.9 | 8.7 / 10.4 |

The native handoff showed roughly four captured frames of delay even with Apple's
`com.apple.videotoolbox.videoencoder.h264.rtvc` encoder selected. NV12 download
through `av_hwframe_transfer_data` removes that measured delay while retaining
ScreenCaptureKit scaling, letterboxing and Rec.709 conversion. Download costs
roughly 3.2 ms on this machine and sacrifices zero-copy handoff. This is a
workaround for observed native-input buffering. Follow-up experiments below
identify FFmpeg packet delivery as the main source of the multi-frame delay,
not equivalent processing time inside Apple's encoder. Other hardware/OS
versions may behave differently.

Forced IDR replay previously spun at roughly 195,000 frames/s during startup,
reusing one surface and causing hundreds of thousands of mailbox replacements.
Replay now waits for the requested frame interval. Normal capture remains driven
by ScreenCaptureKit; requesting 100 fps does not guarantee 100 delivered fps.

```sh
cargo build --release -p eyepiece-pc
target/release/eyepiece-pc --source screencapturekit --no-connect \
  --encoder videotoolbox --fps 60 --duration 10 --stats-interval 2
target/release/eyepiece-pc --source screencapturekit --no-connect \
  --encoder videotoolbox --fps 100 --duration 10 --stats-interval 2
target/release/eyepiece-pc --source screencapturekit --no-connect \
  --encoder videotoolbox --cpu-scale --fps 100 --duration 10 --stats-interval 2
```

Do not enable `max_ref_frames=1` together with `spatial_aq=0` based on general
low-latency advice: that experiment reduced synthetic output from 100 to about
50 fps and raised bitrate to about 36 Mbit/s on this machine. Those experimental
defaults were reverted.

## Copy-Free Follow-Up Experiments

The primary objective here is capture-to-transport-ready packet latency, not
unpaced throughput. Production CLI/GUI routing remains unchanged during these
experiments. The following probes are standalone and do not send to goggles.

### Native APIs Without FFmpeg

`crates/eyepiece-av/examples/native_vt_bench.swift` connects ScreenCaptureKit
directly to `VTCompressionSession`. It passes the captured `CVPixelBuffer`
without application raw-pixel locks, downloads or copies. It retains each
sample until the native output callback, disables reordering, requires hardware
encoding and enables low-latency rate control. Capture handles scaling, black
letterboxing and Rec.709. Compressed packets are copied/converted to Annex-B;
this is not a claim that Apple performs no internal GPU transfers.

The probe measures native completion and transport-ready Annex-B packet latency
separately, discards the first 30 frames, and validates completed/submitted
counts. Capture presentation timestamps and callback arrival use the CoreMedia
host clock. Presentation-to-packet includes measured capture age; neither metric
includes network, goggles decode or display. Optional file writes add disk work.

Final runs with letterboxing and Annex-B packaging:

| Native API path | Requested fps | Callback-to-packet p50 / p95 ms | Presentation-to-packet p50 / p95 ms |
| --- | ---: | ---: | ---: |
| H.264, NV12, 1080p | 60 | 9.54 / 10.23 | 10.25 / 10.91 |
| H.264, NV12, 1080p | 100 | 9.70 / 10.25 | 10.31 / 12.40 |
| HEVC, NV12, 1080p | 100 | 9.59 / 10.53 | 10.13 / 13.04 |

H.264 and HEVC dumps decoded without FFmpeg errors, at 1920x1080. H.264
returned 241/241 frames; HEVC returned 319/319. These are short, changing desktop
workloads, not quality-equivalent controlled footage. Delivered rates varied
with screen activity; requesting 100 fps did not reliably deliver 100 fps.

Earlier isolated experiments, before always-on Annex-B packaging:

| Experiment | Observation |
| --- | --- |
| Native H.264 NV12, 60 fps | Callback output p50 9.19 ms, p95 9.82 ms |
| Complete each submitted frame | No latency improvement: about 9.6 ms; peak one in flight |
| Capture BGRA instead of NV12, 100 fps | No meaningful improvement: p50 9.38 ms, p95 10.06 ms |
| Capture queue depth 1 | Only one frame delivered; unsuitable on this platform |
| Capture queue depth 2 | About 9.65 ms; lower delivery cadence than depth 3 |
| Limit encoder to one in-flight frame | Dropped 14 frames; about 9.55 ms, lower cadence |
| H.264 1280x720 NV12 | Encode callback p50 5.73 ms, p95 8.24 ms; lower capture cadence |
| Disable low-latency rate control, complete each frame | p50 8.42 ms but p95 14.05 ms; worse tail |
| Set MaxFrameDelayCount=1 | Rejected with OSStatus -12900; no tuning effect |

```sh
swiftc -O -parse-as-library -sdk "$(xcrun --show-sdk-path)" \
  crates/eyepiece-av/examples/native_vt_bench.swift -o target/native-vt-bench
target/native-vt-bench 60 6 -1 3 1 0 0 h264 nv12
target/native-vt-bench 100 6 -1 3 1 0 0 h264 nv12
target/native-vt-bench 100 6 -1 3 1 0 0 hevc nv12
target/native-vt-bench 100 6 -1 3 1 0 0 h264 bgra
target/native-vt-bench 100 6 -1 3 1 0 0 h264 nv12 1280 720
```

Positional arguments: fps, duration seconds, maximum frame delay (`-1` leaves
unset), capture queue depth, low-latency rate control (0/1), maximum in-flight
frames (0 disables limit), complete each frame (0/1), codec, pixel format,
optional width and height, optional Annex-B dump path. Screen Recording
permission must be granted to the launching terminal/app.

```sh
target/native-vt-bench 60 4 -1 3 1 0 0 h264 nv12 1920 1080 target/native-probe.h264
ffmpeg -hide_banner -v error -i target/native-probe.h264 -f null -
ffprobe -v error -count_frames -show_entries stream=codec_name,width,height,nb_read_frames \
  -of json target/native-probe.h264
```

### FFmpeg Delivery Backlog

FFmpeg 9.0.2's `vtenc_frame` submits a frame and calls
`vtenc_q_pop(vtctx, !frame, ...)`: while submitting, it does not wait for the
asynchronous native callback. It pops at most one completed sample per submitted
frame. An initial backlog therefore persists even after steady state begins,
and completed output is gated on later frame submissions. A fast 0.07 ms
encode call is submission time, not encoding completion.

`vt_complete_interpose.c` is an experimental macOS dynamic interposer. It calls
`VTCompressionSessionCompleteFrames` after each public EncodeFrame call. This
forces completion before FFmpeg tries to pop output, using no private FFmpeg
layout or application framebuffer readback.

Matched fresh-capture runs, 60 warmup + 120 measured frames, 1080p H.264:

| FFmpeg input/delivery | Requested fps | Callback-to-packet p50 / p95 ms |
| --- | ---: | ---: |
| Native surfaces, normal FFmpeg | 60 | 52.25 / 104.60 |
| Native surfaces, completion interposer | 60 | 9.12 / 10.18 |
| Native surfaces, completion interposer | 100 | 9.62 / 10.83 |
| Current NV12 readback workaround | 60 | 8.86 / 9.56 |

Every run returned 120/120 measured packets. Earlier forced-replay runs showed
69.03 ms normal native latency versus 9.64 ms with completion. Fresh capture
only is now used to avoid replay-induced cadence distortion.

```sh
clang -dynamiclib -framework VideoToolbox -framework CoreMedia \
  crates/eyepiece-av/examples/vt_complete_interpose.c -o target/vt-complete-interpose.dylib
cargo build --release -p eyepiece-av --example encoder_bench
target/release/examples/encoder_bench videotoolbox h264 60 120 native
DYLD_INSERT_LIBRARIES="$PWD/target/vt-complete-interpose.dylib" \
  target/release/examples/encoder_bench videotoolbox h264 60 120 native
target/release/examples/encoder_bench videotoolbox h264 60 120 readback
```

The interposer is a diagnostic tool, not a production preload recommendation.
It applies to all VideoToolbox encode calls in the injected process and changes
submission into a blocking operation. Hardened/signed applications can also
reject library injection. Do not inject it into unrelated applications.

### Recommendation

For eliminating framebuffer copies, a native callback-driven encoder is now
demonstrably viable: approximately 10 ms from capture presentation to Annex-B
packet readiness on this machine. A narrower alternative is fixing FFmpeg's
low-delay native delivery to complete submitted frames before popping output.
Neither approach has yet been integrated into the streaming session.

Do not switch to BGRA, shrink capture queues, or use HEVC solely for lower
latency based on these results. Keep ScreenCaptureKit NV12 with queue depth 3
and low-latency H.264; receiver support and tail latency matter more than a small
median difference. The copied path remains about 0.7 ms faster in the matched
60 fps run, but copy-free paths avoid its framebuffer bandwidth and CPU work.
Goggles presentation latency must be measured before claiming an end-to-end win.

## Online Research and Next Experiments

Researched 2026-11-23. These are source-backed approaches and proposed checks,
not additional benchmark results. No production settings changed in this pass.

### Highest-Priority Work

1. **Native callback delivery with bounded lifetime.** Apple's
   [low-latency VideoToolbox session](https://developer.apple.com/videos/play/wwdc2021/10158/)
   describes hardware one-in/one-out encoding and output handlers. Current
   [Sunshine native encoder](https://github.com/SunshineStream/Sunshine/blob/master/src/platform/macos/vt_encoder.mm)
   implements direct CVPixelBuffer encoding and waits for its callback with a
   timeout, rather than waiting for subsequent capture frames to deliver output.
   This supports our measured native-path result. Compare callback wait versus
   CompleteFrames using identical animated input; include IDR, dropped-frame,
   timeout and shutdown behavior. Avoid blocking a capture callback or holding
   a capture-state lock while waiting for encoding or network sends.

2. **Peak real-time frame-rate hint.** macOS 15+ adds
   [MaximumRealTimeFrameRate](https://developer.apple.com/documentation/videotoolbox/kvtcompressionpropertykey_maximumrealtimeframerate).
   Unlike ExpectedFrameRate, this describes peak possible submission rate and
   lets the encoder configure for it; RealTime must be enabled. Our native probe
   sets ExpectedFrameRate but does not set this newer hint. Query supported
   properties, then compare unset versus 60/100/120 at fixed output size and
   actual capture cadence. Measure p95/p99, dropped frames, power and quality;
   this hint is not a guarantee of lower latency.

3. **Capture pool ownership, not just queue size.** Apple's
   [advanced ScreenCaptureKit talk](https://developer.apple.com/videos/play/wwdc2022/10155/)
   explains that held IOSurfaces become unavailable to capture, and large pools
   can let stale frames accumulate. Its 2022 guidance specifies depth 3-8,
   default 3; do not assume our depth-1/2 experiments are supported or equivalent
   across OS versions. Keep latest-frame replacement, promptly release superseded
   samples, and release encoding input after the native callback. Instrument
   outstanding surface count and capture presentation age. Test whether retaining
   the last sample for static-screen IDRs reduces pool headroom under load.
   Do not infer that queue depth equals a fixed number of buffered video frames.

4. **Separate bitrate target from hard byte windows.** Apple's
   [DataRateLimits documentation](https://developer.apple.com/documentation/videotoolbox/kvtcompressionpropertykey_dataratelimits)
   describes hard byte/time windows, unset by default. FFmpeg sets such a window
   when rc_max_rate is nonzero; our encoder sets it equal to the bitrate target.
   Sunshine deliberately omits DataRateLimits in low-latency mode because its
   authors observed frame drops on large screen changes. Test target-only versus
   a hard window on scrolling, scene cuts and repeated IDRs. Removing a ceiling
   can also worsen transport queueing: record actual frame sizes, write duration,
   receiver queue depth and capture-to-display p95, not just encoder p50.

### Useful Secondary Approaches

- **Long-term reference compatibility and recovery.** Sunshine explicitly sets
  EnableLTR=false for H.264, citing freezes in some hardware decoders. Verify
  goggles compatibility before using LTR; do not assume an encoder's default
  is safe. Apple's WWDC21 session shows LTR recovery using acknowledged frames
  instead of a large IDR, but requires application-level acknowledgement tokens
  and receiver support. A standalone EnableLTR=false check differs from the
  rejected max_ref_frames=1/spatial_aq=0 experiment. Also benchmark less frequent
  periodic IDRs with reliable explicit resync requests; decoder recovery is the
  required gate, not bitrate savings alone.

- **GPU-only custom conversion or source integration.** If custom crop, stereo
  composition or scaling cannot be expressed through ScreenCaptureKit, use
  [CVMetalTextureCache](https://developer.apple.com/documentation/corevideo/cvmetaltexturecache)
  to expose CVPixelBuffer-backed textures and process into a pooled,
  VideoToolbox-compatible IOSurface buffer. This can avoid CPU readback but may
  add a GPU pass, synchronization and an internal transfer. When we control the
  source renderer, producing encoder input directly is an architectural option
  that can bypass desktop capture/composition. It is not a general replacement
  for capturing arbitrary applications. Benchmark against capture-side scaling,
  which Apple already documents as hardware accelerated.

- **Capture a single window or smaller source region.** ScreenCaptureKit supports
  desktop-independent window capture and hardware scaling. Compare a game window
  or source crop against full-display capture under identical motion and output
  dimensions. Keep output dimensions stable; Apple's talk warns frequent output
  resizing causes additional allocations. This could reduce composition/input
  work, but no latency gain has been established for this machine.

- **Dirty-region transport.** Apple's advanced capture talk exposes dirty rects
  and describes sending only changed regions. This is relevant to desktop/UI
  workloads, but requires a receiver that reconstructs prior frames. It is not
  an AVOption that automatically makes our full-frame H.264 goggles protocol
  region-based. Avoid this redesign unless controlled sparse-UI tests justify it.

- **Receiver-side low-latency decoding.** Android API 30+ exposes
  [MediaFormat.KEY_LOW_LATENCY](https://developer.android.com/reference/android/media/MediaFormat#KEY_LOW_LATENCY),
  which asks decoders not to hold data beyond codec requirements. Actual goggles
  API/decoder support and access to the receiver implementation must be checked.
  Correlate encoder completion, send completion, decode completion and display
  presentation. An encoder-only improvement need not improve glass-to-glass time.

### Knobs and Alternatives to Deprioritize

- [SuggestedLookAheadFrameCount](https://developer.apple.com/documentation/videotoolbox/kvtcompressionpropertykey_suggestedlookaheadframecount)
  directly affects latency in standard encoding, but Apple explicitly says it is
  ignored when EnableLowLatencyRateControl=true. It is not a missing low-latency
  tuning knob in our selected rtvc path.
- MaxFrameDelayCount and speed-priority hints already failed our capability tests.
  Power-efficiency and reference-count properties should be queried and tested
  separately, not bundled into speculative presets.
- All-intra encoding eliminates inter-frame prediction but increases bitrate and
  potential transmission delay. Apple documents this through
  [AllowTemporalCompression=false](https://developer.apple.com/documentation/videotoolbox/kvtcompressionpropertykey_allowtemporalcompression).
  It requires a fixed-quality/full-pipeline comparison, not an assumption that
  removing prediction improves total latency.
- [CGDisplayStream](https://developer.apple.com/documentation/coregraphics/cgdisplaystream)
  is a legacy display-stream alternative with no demonstrated advantage here.
  ScreenCaptureKit already supplies GPU-backed capture, scaling and conversion;
  replacing it or adding an AVFoundation recording pipeline is not the first
  optimization to pursue.
- Current [FFmpeg master](https://github.com/FFmpeg/FFmpeg/blob/master/libavcodec/videotoolboxenc.c)
  still uses the nonblocking vtenc_q_pop(vtctx, !frame, ...) submission path.
  An FFmpeg upgrade alone is not evidence that our native-delivery backlog is fixed.

Use repeatable animated input, a fixed display refresh and output resolution,
one setting change per run, and at least three runs per variant. Include CPU,
surface count, frame-size distribution, p95/p99 and receiver presentation timing.
Check supported property dictionaries and actual output before attributing changes
to an option. Source-code observations from Sunshine are useful engineering
evidence, not Apple API guarantees or verified goggles behavior.

## Completed Experiment Matrix

### Method and Limitations

Same M4 Pro, macOS and FFmpeg versions as above. Display 1 reports 1800x1169
logical dimensions and 120 Hz. A 960x540 animated window updates at 120 timer
ticks/s. Native trials run for four seconds and exclude the first 30 encoded
frames from latency distributions. Default: 1920x1080, H.264 High, NV12,
100 fps requested, 20 Mbit/s target, GOP 200, low-latency rate control, queue 3,
no hard rate window and no application framebuffer readback. Three repetitions
per variant, one setting changed except explicitly labeled stress combinations.

Tables report the **median of three per-run quantiles**, not pooled quantiles or
confidence intervals. p99 has only a few tail samples in these short trials.
FPS includes startup; encoder and capture rates can differ. Reported native
drops are successful callbacks with no encoded sample, separate from application
backpressure skips. Failed/invalid preliminary runs are retained in raw files
but excluded from the canonical result set.

The animation source was made floating and App Nap was suppressed after a
preliminary cadence collapse. Capture scope and native baselines were rerun;
crop coordinates now use actual SCWindow bounds, and letterboxing uses the
window/crop aspect ratio. Scope changes alter included pixels and scaling, so
their gains are workflow-specific, not an equal-content compression comparison.
Direct GPU input uses a moving checkerboard, not the desktop workload.

Some ScreenCaptureKit sample presentation timestamps are slightly ahead of
callback arrival. The revised probe records these negative capture ages rather
than silently excluding them. Consequently, presentation-to-packet figures can
be smaller than callback-to-packet. The latter is the primary comparison here;
neither is physical glass-to-glass latency. Earlier presentation measurements
used a nonnegative-age filter and are not directly comparable.

### Software Baseline Reruns

Unpaced 1080p testsrc2, 20 Mbit/s, nominal 100 fps, 60 warmup + 600 measured
frames, same presets. Every run returned 600/600 packets. Pattern generation and
conversion are excluded from encode timing but included in pipeline throughput.

| Encoder | Original encode p50 / p95 ms | Rerun encode p50 / p95 ms | Original pipeline fps | Rerun pipeline fps |
| --- | ---: | ---: | ---: | ---: |
| x264 | 1.08 / 1.23 | 1.10 / 1.23 | 450.3 | 447.9 |
| x265 | 2.93 / 3.84 | 2.97 / 3.66 | 226.0 | 235.7 |
| VideoToolbox H.264 | 4.74 / 4.92 | 4.75 / 4.97 | 155.0 | 152.2 |
| VideoToolbox HEVC | 5.19 / 5.39 | 5.21 / 5.43 | 144.4 | 141.2 |

The original encoder-speed result reproduces: x264 remains much faster than
VideoToolbox for this preset/workload. This does not imply equivalent quality.

### FFmpeg Live Delivery Reruns

Fresh frames, 60 warmup + 120 measured frames, three repetitions. Every run
returned 120/120 measured packets. All timings below include conversion and
packet packaging after capture callback arrival.

| Path | Requested fps | Original callback-to-packet p50 / p95 ms | Rerun p50 / p95 ms | Rerun pipeline fps |
| --- | ---: | ---: | ---: | ---: |
| FFmpeg native | 60 | 52.25 / 104.60 | 51.96 / 55.67 | 57.5 |
| FFmpeg native | 100 | 44.4 / 46.3 (earlier session) | 33.26 / 34.84 | 90.2 |
| FFmpeg completion interposer | 60 | 9.12 / 10.18 | 9.61 / 10.63 | 57.6 |
| FFmpeg completion interposer | 100 | 9.62 / 10.83 | 9.54 / 10.29 | 90.1 |
| FFmpeg NV12 readback | 60 | 8.86 / 9.56 | 8.79 / 9.65 | 57.3 |
| FFmpeg NV12 readback | 100 | 8.8 / 9.5 (earlier session) | 8.55 / 9.79 | 90.2 |

Backlog size is timing-dependent, not an invariant four frames. Ordinary native
FFmpeg submit calls were about 0.04-0.05 ms while packet delivery lagged by
33-52 ms. Forced completion removes that delivery backlog without framebuffer
readback. An initial runner launched injection through Apple's protected `time`
utility, which stripped DYLD_INSERT_LIBRARIES; those uninjected runs were excluded.
The corrected runner sets injection via `/usr/bin/env` after `time` launches.

### Native Capture and Delivery

p50/p95/p99 below are capture-callback-to-Annex-B-packet milliseconds.
The final matched 100 fps full-display baseline is 9.42 / 10.30 / 10.64 ms.
Small sub-millisecond differences across separate batches are not evidence of a
meaningful gain. Previous single-run native H.264 was 9.54-9.70 ms median.

| Variant | Delivered fps | p50 / p95 / p99 ms | Result |
| --- | ---: | ---: | --- |
| Full display, 100 fps | 90.02 | 9.42 / 10.30 / 10.64 | Matched final baseline |
| Full display, 60 fps | 57.43 | 9.18 / 10.21 / 10.43 | No meaningful latency reduction |
| Callback wait on latest-frame worker | 88.88 | 9.09 / 9.98 / 10.19 | One in flight; 2/2/3 mailbox skips |
| Complete every frame | 85.54 | 9.46 / 10.33 / 10.59 | No latency win |
| Retain last captured sample | 87.87 | 9.16 / 10.19 / 10.46 | No clear effect under this load; not a pool-saturation proof |
| Capture queue 1 | 0.25 | n/a | Exactly one frame per run; no measured samples |
| Capture queue 2 | 88.94 | 9.36 / 10.23 / 10.66 | No latency win |
| Capture queue 4 | 89.63 | 9.12 / 10.17 / 10.53 | No meaningful win |
| Capture queue 8 | 89.12 | 9.45 / 10.28 / 10.69 | No latency win |
| Maximum one in-flight frame | 87.86 | 9.38 / 10.25 / 10.42 | Application skips 8/6/24 frames |
| Maximum two in-flight frames | 89.20 | 9.42 / 10.19 / 10.51 | One application skip each run |
| Single-window NV12 capture | 93.77 | 6.65 / 7.11 / 7.27 | About 29% lower median; different capture scope |
| Source crop to motion window | 89.89 | 7.40 / 8.08 / 8.28 | About 21% lower median; different capture scope |
| Native BGRA input | 89.88 | 9.36 / 10.29 / 10.46 | No improvement over NV12 |
| BGRA -> GPU CoreImage/Metal -> pooled NV12 | 89.72 | 9.78 / 10.72 / 11.08 | Extra GPU pass adds latency |
| Direct GPU-generated NV12 | 99.96 | 7.56 / 8.83 / 9.59 | No capture/readback; different source workload |
| Native HEVC | 90.27 | 9.76 / 10.71 / 11.02 | No H.264 latency win |
| 1280x720 H.264 NV12 | 89.69 | 5.55 / 7.77 / 8.12 | About 41% lower median at reduced resolution |

The capture-scope and 720p findings improve on the earlier weakly controlled
experiments. Keep queue depth 3 unless another workload demonstrates a benefit.
The latest-frame worker avoids blocking the capture callback while waiting for
the encoder, but its small number of skips must be considered in integration.

### Encoder Properties and Rate Control

Rejected properties remain in the table to show that apparent timing changes
cannot be attributed to them. All accepted values were set before PrepareToEncode.

| Variant | p50 / p95 / p99 ms | Capability and behavior |
| --- | ---: | --- |
| MaximumRealTimeFrameRate=60 | 9.29 / 10.16 / 10.42 | Rejected -12900 |
| MaximumRealTimeFrameRate=100 | 9.19 / 10.15 / 10.49 | Rejected -12900 |
| MaximumRealTimeFrameRate=120 | 9.31 / 10.14 / 10.50 | Rejected -12900 |
| PrioritizeEncodingSpeedOverQuality=1 | 9.35 / 10.17 / 10.57 | Rejected -12900 |
| MaxFrameDelayCount=1 | 9.42 / 10.27 / 10.56 | Rejected -12900 |
| SpatialAdaptiveQPLevel=0 | 9.49 / 10.31 / 10.51 | Rejected -12900 |
| SpatialAdaptiveQPLevel=1 | 9.36 / 10.23 / 10.41 | Rejected -12900 |
| SuggestedLookAheadFrameCount=0, low-latency | 9.36 / 10.11 / 10.27 | Rejected -12900; no active tuning effect |
| EnableLTR=0 | 9.42 / 10.22 / 10.74 | Accepted; no median win; software decoding passed |
| EnableLTR=1 | 9.44 / 10.27 / 10.54 | Accepted; software decoding passed; goggles recovery untested |
| ReferenceBufferCount=2 | 9.35 / 10.19 / 10.38 | Accepted; no improvement |
| ReferenceBufferCount=1 | 9.12 / 9.97 / 10.13 | Accepted; 162/180/180 encoder drops; about 45 fps |
| References=1 plus AQ=0 | 9.40 / 10.15 / 10.45 | AQ rejected; 181/183/181 encoder drops; about 45 fps |
| MaxAllowedFrameQP=20 | 8.99 / 10.25 / 10.51 | Accepted; two encoder drops per run |
| MaximizePowerEfficiency=0 | 9.36 / 10.20 / 10.54 | Accepted; no meaningful latency improvement |
| MaximizePowerEfficiency=1 | 9.37 / 10.17 / 10.36 | Accepted; no meaningful latency improvement |
| Standard rate control | 8.50 / 12.28 / 13.04 | Lower median, worse p95/p99; about 89.8 fps |
| Standard rate control + lookahead=0 | 18.66 / 21.02 / 21.83 | Accepted; substantial regression |
| DataRateLimits, one-second window | 9.33 / 10.09 / 10.38 | Accepted; no encoder drops at 20 Mbit/s |
| DataRateLimits, 40 ms window | 9.39 / 10.31 / 10.61 | Accepted; no encoder drops at 20 Mbit/s |

Reference count 1 alone reproduces the earlier combined tuning regression.
The selected rtvc encoder rejected AQ, so that earlier regression must not be
attributed to AQ being disabled. Reference-count-1 output was roughly 24 Mbit/s
and almost every surviving frame was a keyframe; it is not a useful speed preset.

### Intra, GOP and Bitrate Stress

| Variant | Delivered fps | p50 / p95 / p99 ms | Output behavior |
| --- | ---: | ---: | --- |
| AllowTemporalCompression=false, low-latency | 90.19 | 9.38 / 10.33 / 10.69 | Accepted but only 2 keyframes per run; not all-intra |
| Force every submitted frame to IDR | 89.56 | 9.50 / 10.30 / 10.66 | Every output keyframe; about 44 Mbit/s at a 20 Mbit/s target |
| AllowTemporalCompression=false, standard encoder | 89.48 | 8.28 / 12.15 / 13.12 | Every output keyframe; about 19.5 Mbit/s; worse tails |
| Forced IDR every 30 submitted frames | 83.67 | 9.33 / 10.19 / 10.50 | 11-13 keyframes/run; about 10.6 Mbit/s versus baseline 1-2 Mbit/s |
| GOP 600 | 81.54 | 9.38 / 10.17 / 10.42 | One keyframe/run; about 0.71 Mbit/s; no latency win |
| 2 Mbit/s target, forced IDR every 15 frames | 43.24 | 9.51 / 10.24 / 10.43 | 185-191 native encoder drops/run |
| Same stress + one-second hard window | 42.73 | 9.40 / 10.17 / 10.48 | 189-190 native encoder drops/run |
| Same stress + 40 ms hard window | 42.70 | 9.44 / 10.26 / 10.59 | 189 native encoder drops/run |

Under this stress, target-only rate control already drops heavily; removing hard
windows does not solve it. Surviving-frame latency alone hides that failure.
Observed output bitrates are approximate, based on measured packet bytes and
estimated post-warmup duration, not a guaranteed transport ceiling. More complex
full-screen footage is needed before generalizing the bitrate-window result.

### CPU, Memory and Output Validation

CPU seconds are user+system time for the entire four-second process, including
startup. Maximum RSS is process memory, not total IOSurface/GPU/WindowServer
memory. The separate animation process is not included. No energy measurement
or claim of lower power is made from CPU time.

| Representative path | CPU seconds | Maximum RSS MiB |
| --- | ---: | ---: |
| Full-display native NV12 | about 0.40 | about 39 |
| Retain last sample | 0.33 | 39.27 |
| Native BGRA | 0.40 | 40.08 |
| GPU conversion pass | 0.63 | 43.17 |
| Direct GPU input | 0.66 | 34.83 |
| 720p | 0.25 | 38.70 |
| Standard rate control | 0.21 | 36.77 |

Twelve representative streams decoded without FFmpeg errors, with complete
counts and expected dimensions: baseline 179 frames, window 190, crop 185, GPU
conversion 180, direct GPU 200, HEVC 180, 720p 183, LTR-off 186, LTR-on 182,
all-intra-property 185, forced-all-intra 176 and IDR-every-30 181. Forced-all-intra
decoded 176/176 keyframes; the all-intra-property stream decoded only one
keyframe. IDR-every-30 decoded seven keyframes, matching native metadata. Final
source-aspect-correct window/crop dumps were decoded again after the geometry fix.
This validates decodability and IDR behavior, not objective visual quality.

An intentionally lost completion notification exercised the worker's one-second
timeout. It detected four waits, dropped 185 unclaimed captures, completed all
four submitted encoder callbacks and terminated rather than hanging. Its
nonzero exit is an expected injected failure, not a latency benchmark. Normal
shutdown checks complete/flush submitted callbacks with no live in-flight frames.

### Coverage and Blockers

| Proposed experiment | Status |
| --- | --- |
| Native callbacks versus CompleteFrames | Executed, three runs each; FFmpeg interposer also rerun |
| Peak FPS, AQ, speed, frame delay, power, references and lookahead | Executed with support/status checks |
| Retained capture surface, queue depth, bounded in-flight input | Executed; no source-pool saturation conclusion claimed |
| Target-only/hard-window bitrate, scene motion, repeated IDRs, GOP | Executed; includes 2 Mbit/s stress |
| All-intra | Executed with standard-encoder and forced-IDR behavior controls |
| Single-window, cropped display and lower resolution | Executed; corrected geometry and repeated comparisons |
| GPU-only conversion and direct source input | Executed using Metal-backed CoreImage and pooled NV12; direct input is a synthetic source surrogate |
| Dropped-frame, timeout and shutdown paths | Exercised; encoder drops distinguished from errors; injected lost-notification check |
| LTR acknowledgement-based recovery | Blocked: no attached goggles, no receiver acknowledgement/token implementation |
| Dirty-region transport | Blocked: existing receiver expects full-frame video; requires a new reconstruction protocol and receiver |
| Receiver low-latency decoding, network queueing, reconnect recovery | Blocked: `adb devices -l` returned no devices; receiver source/control unavailable in this workspace |
| Physical end-to-end display latency | Blocked: no connected goggles/display instrumentation; host timestamps are not sufficient |
| Actual encoder power/energy | Unmeasured: privileged power sampler required; no privilege escalation attempted |
| Objective matched-quality comparison | Unmeasured: no frame-aligned raw reference for changed capture scope/direct GPU input |
| CGDisplayStream capture | Attempted compilation; SDK marks capture APIs unavailable from macOS 15.0, directing callers to ScreenCaptureKit |
| Arbitrary application renderer integration | Not performed: direct GPU surrogate tested, but third-party renderer changes are outside this workspace |

No result above establishes a goggles compatibility or glass-to-glass win.
The experiments support offering window/crop capture and 720p when suitable,
and replacing FFmpeg's delayed native packet delivery with a bounded native
callback path. Do not adopt reference count 1, assume peak-rate hints work, or
prefer standard rate control solely because of its slightly lower median.

### Reproduce and Inspect

```sh
swiftc -O -parse-as-library -sdk "$(xcrun --show-sdk-path)" \
  crates/eyepiece-av/examples/native_vt_bench.swift -o target/native-vt-bench
target/native-vt-bench animate
```

Run the animation in a separate terminal; stop it after benchmarking. Then:

```sh
node crates/eyepiece-av/examples/run_encoder_experiments.mjs
BENCH_FILTER='^(native100|window|crop)$' BENCH_OUTPUT=target/scope.json \
  node crates/eyepiece-av/examples/run_encoder_experiments.mjs
cargo build --release -p eyepiece-av --example encoder_bench
clang -dynamiclib -framework VideoToolbox -framework CoreMedia \
  crates/eyepiece-av/examples/vt_complete_interpose.c -o target/vt-complete-interpose.dylib
BENCH_FFMPEG=1 BENCH_OUTPUT=target/baselines.json \
  node crates/eyepiece-av/examples/run_encoder_experiments.mjs
```

Runner defaults: three repetitions, four seconds per native run. BENCH_SECONDS,
BENCH_REPEATS, BENCH_FILTER and BENCH_OUTPUT select subsets and output. The
timeout-check variant intentionally exits unsuccessfully and records its result.
VT_* environment settings map directly to variants in the runner; unsupported
properties log their exact status. RESULT JSON includes p50/p95/p99, FPS,
frame-size distributions, encoder drops, skipped input and timestamp validity.

Raw canonical data: `target/encoder-experiments-final.json` (138 native trials,
30 software/FFmpeg trials, one injected timeout). Per-batch stdout/stderr and
CPU/RSS are preserved in `target/encoder-experiments-*.json`; bitstream validation
results are in `target/encoder-experiments-validation.json`. These generated
artifacts are ignored build outputs. Earlier cadence, geometry, injected-launch
and drop-accounting failures are not part of the canonical comparison.

## Virtual 1080p Display

### Display and Pixel Dimensions

The virtual display named "DJI Goggles" is display ID 10 in this session. It
was initially **1600x900 at 100 Hz**, not 1080p. Available modes include native
1920x1080 at 100 and 60 Hz. Benchmarks temporarily switched it to 1920x1080@100
using CGDisplaySetDisplayMode, then restored 1600x900 successfully after every
session. No persistent display settings were changed.

The built-in display illustrates why logical dimensions are insufficient:
ScreenCaptureKit reports 1800x1169 logical points, while the current mode has
3600x2338 backing pixels. The virtual 1080p mode has **1920x1080 logical and
1920x1080 backing pixels**, allowing a genuine one-to-one capture.

The scaling-disabled validation logged:

```text
DISPLAY id=10 size=1920x1080 backing=1920x1080 refresh=100.0
CAPTURE scalesToFit=false output=1920x1080
INPUT pixels=1920x1080 format=875704438
```

The format is video-range NV12 (420v). The encoded stream decoded without errors
at 1920x1080, returning all 54 validation frames. This confirms capture/output
dimensions and no requested geometric rescaling, not the absence of internal
GPU composition, format conversion or transfers.

### Matched Native Results

Three four-second repetitions per variant; first 30 frames excluded from latency
quantiles. H.264 High, 20 Mbit/s target, queue 3, native CVPixelBuffer handoff,
no application framebuffer readback. The animation window was placed on display
10. Entries are medians of three per-run distributions. Display-specific content,
pixel density and actual cadence differ from the earlier built-in display runs.

| Path | Requested fps | Delivered fps | Callback-to-packet p50 / p95 / p99 ms |
| --- | ---: | ---: | ---: |
| Built-in full display, earlier baseline | 100 | 90.03 | 9.42 / 10.30 / 10.64 |
| Built-in full display, earlier 60 fps baseline | 60 | 57.43 | 9.18 / 10.21 / 10.43 |
| Virtual 1080p, scalesToFit=true | 100 | 59.69 | 7.44 / 7.87 / 8.40 |
| Virtual 1080p, scalesToFit=false | 100 | 60.03 | 7.34 / 7.90 / 8.12 |
| Virtual 1080p, no scale, callback-wait worker | 100 | 59.79 | 7.42 / 7.91 / 8.24 |
| Virtual 1080p, no scale, 60 fps requested | 60 | 59.40 | 6.77 / 7.33 / 7.58 |
| Virtual 1080p, no scale, native BGRA | 100 | 60.09 | 7.42 / 7.89 / 8.26 |

The 0.10 ms median difference between scaling on/off is too small to establish
a performance benefit. When the backing pixels already match the output,
scalesToFit=true is an identity-size request; there is no geometric resizing
needed. Explicitly disabling it is valid here but is not a material speedup.
The virtual-display path itself is faster than the earlier built-in display
path, including the roughly cadence-matched 60 fps comparison. One scaling-on
trial had a 35.58 ms p99 outlier; the table's median-of-quantiles must not hide it.
No encoder errors or native dropped frames occurred in the main 15 runs. The
callback-wait variant skipped one unclaimed frame in its third run.

Native no-scale process cost: about 0.28 CPU seconds per four-second trial,
39.78 MiB maximum RSS. These do not include WindowServer/GPU memory or energy.

### Readback Comparison and Cadence

Three fresh-capture repetitions on the same temporary virtual 1080p mode:

| FFmpeg path | Requested fps | Median delivered fps | Callback-to-packet p50 / p95 ms |
| --- | ---: | ---: | ---: |
| Completion interposer, native input | 100 | 31.2 | 7.26 / 8.29 |
| Current NV12 readback path | 100 | 31.3 | 6.53 / 7.28 |

All six runs returned 120/120 measured packets after 60 warmup frames. Actual
capture cadence varied: about 30 fps in the first two repetitions and 60 fps
in the third. These values must not be compared as equal-throughput 100 fps
measurements. Readback remained slightly faster, with about 5.88 ms encode time
and roughly 0.65 ms additional capture/conversion cost in this workload.

The mode reports 100 Hz and the motion source logged roughly 120 draw calls/s,
but ScreenCaptureKit delivered only about 30-60 complete frames/s during these
checks. Disabling scaling did not remove that limitation. The cause within the
virtual display/compositor/capture scheduling path is not established; these
tests do **not** demonstrate a 100 fps virtual-display streaming path. A static
validation run delivered fewer frames and was used only for decoding checks.

Virtual-display sample presentation timestamps were sometimes ahead of callback
arrival, even yielding negative presentation-to-packet values. Use the callback
latency and actual frame cadence here, not those timestamps as glass-to-glass
latency or proof of capture occurring before presentation.

### Conclusion and Reproduction

**Yes, rescaling can be skipped when the virtual display is set to native
1920x1080 backing pixels and capture/encoding output is also 1920x1080.** At its
restored 1600x900 mode, a 1920x1080 stream needs upscaling; alternatively, a
1600x900 output could be one-to-one if the receiver supports that resolution.
NV12 conversion and compressed-packet handling still remain. This pass changes
benchmark tools only; production capture settings were not changed.

```sh
target/native-vt-bench list
VT_DISPLAY_ID=10 BENCH_VIRTUAL=1 BENCH_ANIMATE=1 BENCH_FILTER='^virtual' \
  BENCH_OUTPUT=target/encoder-experiments-virtual1080.json \
  target/native-vt-bench with-mode node crates/eyepiece-av/examples/run_encoder_experiments.mjs
VT_DISPLAY_ID=10 BENCH_ANIMATE=1 BENCH_FFMPEG=1 \
  BENCH_FILTER='^ff-(readback|complete)100$' \
  BENCH_OUTPUT=target/encoder-experiments-virtual-ffmpeg.json \
  target/native-vt-bench with-mode node crates/eyepiece-av/examples/run_encoder_experiments.mjs
```

The wrapper restores the prior mode on normal completion and thrown errors;
do not forcibly kill it mid-session. Display IDs can change, so enumerate before
rerunning. VT_NO_SCALE=1 rejects capture if the selected mode's backing pixels
do not match requested output dimensions. The runner starts/stops the animation
with BENCH_ANIMATE=1 and records its draw log. Raw virtual results, cadence logs
and per-run output are saved separately in `target/encoder-experiments-virtual*.json`.

## x264 Versus VideoToolbox Live Comparison

### Question and Method

Avoiding framebuffer copies is no longer a requirement. The objective is lowest
capture-to-packet latency while sustaining high frame rates. Earlier synthetic
x264 timings did not establish that its advantage survived actual capture and
conversion. The live benchmark previously rejected software encoders; it now
supports both encoders on matching capture pipelines:

- **Readback:** ScreenCaptureKit scales/letterboxes to 1920x1080 NV12, then the
  same converter downloads NV12 for either encoder. This isolates the encoder
  difference without confounding CPU versus capture-side color conversion.
- **BGRA:** ScreenCaptureKit captures source-size BGRA, copies its pixels to CPU
  memory, then the same FFmpeg converter scales/letterboxes to 1920x1080 NV12
  for either encoder. This is x264's existing production capture route.

Same M4 Pro, 20 Mbit/s configured target, H.264 High, four slices, GOP two
nominal seconds and no B-frames. x264 uses superfast/zerolatency unless a preset
is named; VideoToolbox uses its existing hardware-only low-delay defaults.
Capture is full-display, not cropped/window-only. Built-in display 1 remained
at 120 Hz, with 3600x2338 backing pixels. The floating 960x540 animation provides
motion on an otherwise ordinary desktop. This is **not a full-motion game** or
an equal-quality comparison. Runs share the same source/pipeline configuration,
not an identical recorded sequence of captured frames.

Main matrix: 18 configurations (60/100/120 fps requested, two capture paths,
both encoders, plus x264 ultrafast/veryfast readback alternatives), three runs
each. Each run excludes 60 warmup frames and measures 240 frames. Run order is
reversed for the middle repetition to reduce order bias. Follow-up runs measure
1200 frames per configuration (about 11 seconds, not a long thermal soak),
600 frames under eight independent CPU-load processes, and virtual 1080p.
Six separate eight-second release CLI sessions verify the real two-stage
production capture/encode code path. No network receiver is involved.

Metrics start at capture callback arrival, include copies/conversion/waiting,
and end after an Annex-B packet is available. Conversion-only and encode-call
timings are also recorded. BGRA readback occurs inside Source::next and is
included in capture-to-packet/input-age, not conversion-only time. Reported
quantiles are medians of three per-run quantiles, not pooled distributions.

### Matched Live Results

| Requested fps | Encoder and input | Delivered fps | Capture-to-packet p50 / p95 / p99 ms | Process CPU % |
| --- | --- | ---: | ---: | ---: |
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

The software encoder advantage survives matching the NV12 capture/readback
pipeline. BGRA is faster still for x264 in these trials, despite its copy,
because it avoids this machine's relatively expensive NV12 hwframe download.
For VideoToolbox the BGRA route uses much more CPU without a dependable latency
benefit. Delivered FPS is similar within each request rate: both are primarily
capture-paced here, not evidence that VideoToolbox matches x264's unpaced capacity.

### Longer Runs and CPU Contention

Three repetitions each at 120 fps requested. CPU percentage is user+system time
divided by whole process runtime, including warmup/startup; **100% means one CPU
core, not the whole 14-core machine**. It excludes WindowServer, the animation,
the media-engine service and the independent load workers. It is not power usage.

| Condition | Encoder and input | Delivered fps | Capture-to-packet p50 / p95 / p99 ms | Process CPU % |
| --- | --- | ---: | ---: | ---: |
| 1200 measured frames | x264 superfast, BGRA | 111.02 | 4.50 / 5.16 / 5.68 | 156.4 |
| 1200 measured frames | x264 superfast, NV12 readback | 111.11 | 5.63 / 6.53 / 7.41 | 127.7 |
| 1200 measured frames | VideoToolbox, NV12 readback | 111.21 | 8.74 / 9.65 / 10.40 | 12.1 |
| Eight CPU-load workers, 600 frames | x264 superfast, BGRA | 111.85 | 5.50 / 6.62 / 7.78 | 149.1 |
| Eight CPU-load workers, 600 frames | x264 superfast, NV12 readback | 111.97 | 5.43 / 6.08 / 6.51 | 101.1 |
| Eight CPU-load workers, 600 frames | VideoToolbox, NV12 readback | 112.20 | 7.95 / 8.45 / 8.75 | 6.0 |

CPU-load workers run PBKDF2 continuously in separate processes. Eight workers
do not saturate every core, emulate a particular game, or create GPU contention.
x264's BGRA tails worsen under this test but remain below VideoToolbox's. A
heavily CPU-bound game could change the choice. Lower VideoToolbox numbers in
the contention batch are not proof that CPU load improves encoding: scheduling,
frequency and source content were not held constant enough for that inference.

Conversion/encoding breakdown for the longer runs (medians, not additive tail
quantiles):

| Path | Conversion p50 / p95 ms | Encode p50 / p95 ms | Extra source work |
| --- | ---: | ---: | --- |
| x264, BGRA | 1.14 / 1.30 | 1.52 / 2.03 | BGRA copy included before conversion timing |
| x264, NV12 readback | 3.25 / 3.62 | 2.35 / 3.06 | Minimal source wrapping |
| VideoToolbox, NV12 readback | 3.15 / 3.70 | 5.44 / 6.01 | Minimal source wrapping |

### x264 Preset Checks

Same NV12/readback capture route; three 240-frame runs per preset/rate. These do
not establish matched quality. Ultrafast also changes coding tools and should
not be chosen solely because its name sounds faster.

| Requested fps | x264 preset | Delivered fps | Capture-to-packet p50 / p95 / p99 ms | Process CPU % |
| --- | --- | ---: | ---: | ---: |
| 60 | ultrafast | 57.76 | 5.27 / 8.05 / 9.83 | 62.9 |
| 60 | superfast | 57.92 | 5.36 / 8.47 / 10.32 | 68.4 |
| 60 | veryfast | 57.85 | 5.71 / 7.84 / 8.82 | 92.2 |
| 100 | ultrafast | 91.53 | 5.15 / 7.41 / 8.12 | 80.2 |
| 100 | superfast | 91.77 | 5.05 / 7.28 / 7.86 | 92.9 |
| 100 | veryfast | 91.42 | 5.63 / 8.63 / 9.50 | 122.7 |
| 120 | ultrafast | 107.70 | 4.86 / 6.96 / 7.48 | 92.4 |
| 120 | superfast | 109.95 | 4.62 / 7.55 / 8.20 | 103.0 |
| 120 | veryfast | 109.98 | 4.91 / 6.85 / 7.48 | 135.8 |

No preset change beats the established BGRA/superfast path convincingly enough
to justify changing defaults. The selected superfast/zerolatency preset remains
the conservative latency-first starting point.

### Real Session and Virtual Display Checks

The actual release CLI session, using its capture thread, conversion, latest-frame
mailbox and encode thread, corroborates the example's result. At 120 fps requested:

- x264: roughly 4.4-4.5 ms capture-to-send median, 4.9-5.3 ms p95, and
  approximately 107-112 fps across printed windows; no mailbox skips observed.
- VideoToolbox readback: roughly 8.5-9.0 ms median, 9.5-10.0 ms steady-state p95.
  Most steady windows reached 109-111 fps, but the third run fell to 84-89 fps.
  It skipped 24-27 captures during encoder startup, then zero in printed steady
  windows. These are capture-paced observations, not proof of encoder overload.

With no connection, "capture-to-send" excludes real socket blocking and any
receiver. Production x264 uses BGRA by default, while VideoToolbox uses the
capture-side NV12/readback path. Both produce the configured 1080p output.

Virtual display 10 was temporarily set to native 1920x1080@100, then restored
to its original 1600x900 mode. Three 240-frame runs at 100 fps requested:

| Path | Median delivered fps | Capture-to-packet p50 / p95 / p99 ms |
| --- | ---: | ---: |
| x264 superfast, NV12 readback | 60.19 | 3.39 / 6.80 / 7.19 |
| x264 superfast, BGRA | 59.45 | 3.70 / 5.22 / 6.45 |
| VideoToolbox, NV12 readback | 59.66 | 6.48 / 7.06 / 8.35 |

The third repetition fell to roughly 39-42 fps across all three variants; the
first two were near 60 fps. Therefore, x264 reduces virtual-display latency but
does not solve its missing 100 fps capture cadence. On this display, NV12
readback costs only about 0.65-0.70 ms rather than the built-in display's ~3 ms,
so BGRA is not unconditionally the best input on every display.

### Validation and Recommendation

All 81 measured example runs returned every requested packet: 54 main-matrix,
nine longer-run, nine CPU-contention and nine virtual-display runs. All six
production sessions exited successfully. Separate x264/BGRA and VideoToolbox/
readback captured dumps decoded at 1920x1080 with all 180 frames (60 warmup plus
120 measured). Raw H.264 contains no container timestamps; decoding with an
explicit `-r 120` input avoids null-muxer timestamp warnings and completed cleanly.

Identical 20 Mbit/s configuration is **not equal output bitrate or quality**.
The longer desktop runs produced nominal stream rates around 17 Mbit/s for
x264 versus 3.4 Mbit/s for VideoToolbox. Nominal rates use configured FPS and
packet sizes; they are not measured socket throughput. Encoder rate control,
screen-content treatment and capture conversion differ. No PSNR/VMAF, identical
captured-frame replay, or goggles presentation measurement has established
equal quality or equal downstream transport/decode cost. Full-motion, GPU-heavy
game footage and long thermal/battery behavior remain untested.

**For the stated priority, choose x264 superfast/zerolatency now.** On the tested
built-in full-display workload it cuts host latency roughly in half without
giving up the observed high FPS. It is already available through `--encoder x264`;
no new native backend is required. Retain VideoToolbox as the lower-CPU option
for CPU-constrained applications, battery/power-sensitive scenarios (energy not
measured here), or workloads where receiver/quality testing favors it. Do not
promote copy-free delivery over measured latency as an end in itself.

120 fps requested produced about 110-112 fps on this 120 Hz display, not a
guaranteed 120 fps stream. Choose the request rate according to capture cadence
and receiver capability; high-rate host-only tests do not certify goggles mode
support. No production encoder-selection defaults were changed in this pass.

### Reproduce the Comparison

```sh
cargo build --release -p eyepiece-av --example encoder_bench
cargo build --release -p eyepiece-pc
VT_DISPLAY_ID=1 BENCH_COMPARE=1 BENCH_ANIMATE=1 \
  BENCH_OUTPUT=target/encoder-compare-builtin.json \
  node crates/eyepiece-av/examples/run_encoder_experiments.mjs
VT_DISPLAY_ID=1 BENCH_COMPARE=1 BENCH_ANIMATE=1 BENCH_FRAMES=1200 \
  BENCH_FILTER='^(x264-bgra-superfast|x264-readback-superfast|videotoolbox-readback-default)-120$' \
  BENCH_OUTPUT=target/encoder-compare-sustained.json \
  node crates/eyepiece-av/examples/run_encoder_experiments.mjs
VT_DISPLAY_ID=1 BENCH_COMPARE=1 BENCH_ANIMATE=1 BENCH_CPU_WORKERS=8 BENCH_FRAMES=600 \
  BENCH_FILTER='^(x264-bgra-superfast|x264-readback-superfast|videotoolbox-readback-default)-120$' \
  BENCH_OUTPUT=target/encoder-compare-contention.json \
  node crates/eyepiece-av/examples/run_encoder_experiments.mjs
VT_DISPLAY_ID=1 BENCH_SESSION=1 BENCH_ANIMATE=1 BENCH_SECONDS=8 \
  BENCH_OUTPUT=target/encoder-compare-production.json \
  node crates/eyepiece-av/examples/run_encoder_experiments.mjs
```

The native probe must first be compiled as above for the animation/mode wrapper.
BENCH_COMPARE defaults to 240 measured frames and three repetitions. BENCH_PRESET
selects an x264 preset when invoking encoder_bench directly. Input mode `bgra`
uses ordinary CPU capture; `readback` uses capture-side NV12 for either encoder.
BENCH_DUMP optionally writes all warmup/measured Annex-B packets for validation;
do not enable dumps in latency trials. Structured RESULT output includes capture,
conversion and encode p50/p95/p99 and packet sizes. Raw comparison output is in
`target/encoder-compare-*.json`, excluded build artifacts, not committed fixtures.

## Production Reference Alignment

The requested macOS default remains hardware-first. The supported primary H.264
paths are VideoToolbox and explicit x264. The production pipeline was audited
against `videotoolbox-readback-default-120` and `x264-bgra-superfast-120` in the
matched live runs, not against the standalone native Swift probe or an injected
FFmpeg process. Both paths already used their benchmark-winning implementation;
no speculative encoder tuning or capture rewrite was needed.

| Component | Software: x264 reference and production | Hardware: VideoToolbox reference and production |
| --- | --- | --- |
| Selection | Explicit `x264` / `libx264` | macOS `auto` first candidate; explicit `videotoolbox` / `vt` also supported |
| Capture API | ScreenCaptureKit, source-size BGRA | ScreenCaptureKit, output-size video-range NV12 |
| Capture buffering | Queue depth 3; latest-sample replacement; paced forced replay | Same |
| Pixel preparation | CPU BGRA copy, bicubic CPU scaling, letterboxing, Rec.709 limited-range NV12 | Capture-side scaling/letterboxing/Rec.709, then `av_hwframe_transfer_data` to CPU NV12 |
| FFmpeg encoder | `libx264` | `h264_videotoolbox`, no native input buffering or runtime interposer |
| Private options | `preset=superfast tune=zerolatency profile=high forced-idr=1 x264-params=scenecut=0:repeat-headers=1:aud=0` | `allow_sw=0 realtime=1 prio_speed=1 profile=high` |
| Common settings | H.264, no B-frames, LOW_DELAY, 20 Mbit/s configured target, four-frame VBV, four slices, GOP two nominal seconds | Same configured values; VBV/slices are driver-managed |
| Frame handoff | Production capture/convert thread -> one-frame latest mailbox -> encode/send thread | Same |
| Output | Annex-B with parameter sets on keyframes and existing receiver protocol | Same |

The hardware reference is the tested FFmpeg readback configuration, including its
configured max rate; it is not the native probe's target-only rate-control
experiment. `prio_speed=1` remains in the reference options even though the M4
rejects the hint. Do not attribute any measured improvement to that hint.

The benchmark example sets gpu_scale_cpu_input=false while normal CLI configuration
uses true, but the CPU GPU-upload branch is Linux-only. On macOS x264 both
settings route to the same CPU filter chain; this is not a conversion difference.
Production adds the bounded capture/encode overlap absent from the sequential
example, and the separate production-session runs validated this architecture.

The CLI default remains 100 fps for the goggles; the reference runs request 120
fps to exercise host capacity. This is intentional receiver policy, not a missed
encoder optimization. Do not raise the default to 120 based on host-only results.
User settings (`--cpu-scale`, `--vf`, encoder options, presets, codec, dimensions,
stereo and requested frame rate) intentionally override parts of the reference.
Ordinary matching-dimension capture needs no geometric resize; no new native
backend or unconditional scaling optimization was introduced.

### Selection Contract and Regression Checks

- macOS `auto` retains VideoToolbox-first order, then codec-appropriate software
  fallback if hardware cannot open. It does not choose x264 because x264 is faster.
- `x264` always names libx264, and `x265` always names libx265. Selecting a codec
  incompatible with either alias now fails instead of changing encoder silently.
- Explicit FFmpeg names are checked against the requested codec too; for example,
  `libx264` with HEVC and `hevc_videotoolbox` with H.264 are rejected.
- Generic `software` / `sw` remain codec-aware for compatibility. Existing explicit
  x265 and non-macOS hardware support are not removed by this macOS H.264 alignment.

Regression coverage protects hardware-first order, literal aliases, mismatch
rejection, per-family reference options, x264 BGRA CPU conversion, VideoToolbox
NV12 readback, and shared CLI default selection/tuning. The GUI uses the same
Args/prepare/session path; its macOS encoder list already offers auto,
VideoToolbox and x264. A saved explicit x264 selection remains explicit rather
than being overwritten with auto.

Post-change release CLI smoke checks at the default 100 fps request verified
the exact capture/conversion and option descriptions against these references.
Default auto opened h264_videotoolbox, and all 440 captured output frames decoded
at 1920x1080; explicit x264 opened libx264 and all 457 output frames decoded.
Steady printed host latencies were about 9.1 ms for hardware and 4.8 ms for x264,
consistent with earlier results but not a replacement benchmark campaign.
Four incompatible alias/name combinations failed before source creation; valid
auto/HEVC and explicit x265/HEVC still opened their intended encoders. Nineteen
affected unit tests passed, and the release CLI and debug CLI/GUI were rebuilt.
