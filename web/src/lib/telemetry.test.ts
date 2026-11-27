import { describe, expect, it } from "vitest";
import { aggregateTelemetry, appendSample, chartSeries, summarizeWindow } from "./telemetry";

describe("chartSeries", () => {
  it("uses elapsed time, preserves missing intervals, and limits the window", () => {
    expect(
      chartSeries(
        [
          { at: 0, values: { latency: 1 } },
          { at: 100_000, values: { latency: 2 } },
          { at: 150_000, values: { latency: NaN } },
          { at: 200_000, values: { latency: 4 } },
        ],
        "latency",
      ),
    ).toEqual([
      { x: -100, y: 2 },
      { x: -50, y: null },
      { x: 0, y: 4 },
    ]);
  });

  it("breaks lines when reports are missing rather than connecting across gaps", () => {
    expect(
      chartSeries(
        [
          { at: 1000, values: { latency: 1, interval_s: 1, report_interval_s: 0.1 } },
          { at: 1100, values: { latency: 2, interval_s: 1, report_interval_s: 0.1 } },
          { at: 1600, values: { latency: 3, interval_s: 1, report_interval_s: 0.5 } },
        ],
        "latency",
      ),
    ).toEqual([
      { x: -0.6, y: 1 },
      { x: -0.5, y: 2 },
      { x: -0.4, y: null },
      { x: 0, y: 3 },
    ]);
  });

  it("weights rates by elapsed report time rather than raw counters", () => {
    expect(
      aggregateTelemetry([
        { at: 1, values: { skipped: 2, idr: 1, interval_s: 2 } },
        { at: 2, values: { skipped: 4, idr: 2, interval_s: 1 } },
        { at: 3, values: { skipped: 10, idr: 4, interval_s: 0 } },
      ]),
    ).toMatchObject({ skipped_per_s: 2, idr_per_s: 1 });
  });

  it("uses elapsed-time means, counter deltas, and queue high-water marks for pins", () => {
    expect(
      aggregateTelemetry(
        [
          { at: 1000, values: { sent_fps: 20, dropped: 100, q_max: 4, report_interval_s: 0.1 } },
          { at: 1300, values: { sent_fps: 40, dropped: 103, q_max: 8, report_interval_s: 0.3 } },
          { at: 1400, values: { sent_fps: 50, dropped: 2, q_max: 6, report_interval_s: 0.1 } },
        ],
        { at: 900, values: { dropped: 100 } },
      ),
    ).toMatchObject({
      sent_fps: 38,
      dropped: 5,
      q_max: 8,
    });
  });

  it("uses the sample before a pin window to count its first report", () => {
    expect(
      aggregateTelemetry([{ at: 2000, values: { dropped: 9 } }], { at: 1900, values: { dropped: 6 } }),
    ).toMatchObject({ dropped: 3 });
  });

  it("does not report zero drops when a single sample has no counter baseline", () => {
    expect(aggregateTelemetry([{ at: 2000, values: { dropped: 9 } }])).not.toHaveProperty("dropped");
  });
});

describe("appendSample", () => {
  it("uses the reported timestamp, falls back to now, and drops stale samples", () => {
    const first = appendSample([], { sampled_at_ms: 1000, sent_fps: 1 });
    expect(first[0].at).toBe(1000);
    const next = appendSample(first, { sampled_at_ms: 1000 + 700_000, sent_fps: 2 });
    expect(next.map((sample) => sample.at)).toEqual([701_000]);
    expect(appendSample([], { sent_fps: 1 }, 42)[0].at).toBe(42);
  });
});

describe("summarizeWindow", () => {
  const sample = (at: number, dropped: number) => ({
    at,
    values: { interval_s: 1, report_interval_s: 1, sent_fps: 60, dropped },
  });

  it("aggregates the window and counts dropped frames against the preceding sample", () => {
    const samples = [sample(1000, 5), sample(2000, 7), sample(3000, 10)];
    expect(summarizeWindow(samples, 1.5, 3500)).toEqual({
      metrics: { sent_fps: 60, dropped: 5 },
      seconds: 2,
      intervals: 2,
    });
  });

  it("returns undefined when no sample is inside the window", () => {
    expect(summarizeWindow([sample(1000, 0)], 1, 10_000)).toBeUndefined();
  });
});
