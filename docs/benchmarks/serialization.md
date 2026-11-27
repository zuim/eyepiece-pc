# Video protocol serialization microbenchmark

These benchmarks measure hot-buffer serialization cost of the `video` message, not network, USB, decoder or
goggles latency. Results are median batch averages (100 batches) on an Apple M4 Pro, macOS 26.6.2, release Rust and
`cc -O3`; the reported p95 is a percentile of batch averages, not frame latency.

| AU bytes | Protobuf overhead | PC vectored prefix | Prost full encode | Prost full decode | Bridge decode |
|---:|---:|---:|---:|---:|---:|
| 8,192 | 16 B | 92 ns | 140 ns | 318 ns | 365 ns |
| 25,000 | 18 B | 89 ns | 316 ns | 623 ns | 365 ns |
| 262,144 | 18 B | 81 ns | 3,541 ns | 7,172 ns | 360 ns |
| 4,194,240 | 20 B | 83 ns | 49,941 ns | 109,333 ns | 400 ns |

The PC sends a small protobuf prefix separately from the borrowed access unit (`access_unit` is the last field
of `Video`), using vectored writes, so the access unit is never copied. The bridge decodes with the access unit
borrowed from its receive allocation, which it keeps until decode finishes; its cost is therefore independent of
size. The hardware decoder still copies the unit into its own stream buffer, and socket and USB buffering may
copy. Web UI preview framing copies into a contiguous WebSocket frame, and preview subscribers receive cloned
events.

Run the Rust benchmark with `cargo test --release -p eyepiece-link --lib
benchmark_video_serialization -- --ignored --nocapture`. To run the bridge
benchmark, first generate nanopb sources using the interoperability instructions
in [building](../development/building.md#bridge-interoperability-tests), then:

```sh
cc -O3 -std=gnu11 -I "$out" -I "$NANOPB_DIR" "$bridge/tests/wire_bench.c" "$bridge/wire.c" \
  "$out/eyepiece-rpc.pb.c" "$NANOPB_DIR/pb_common.c" "$NANOPB_DIR/pb_encode.c" \
  "$NANOPB_DIR/pb_decode.c" -o "$out/wire_bench"
"$out/wire_bench"
```
