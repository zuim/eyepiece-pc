export type MetricSample = { at: number; values: Record<string, number> };

function sampleWeight(values: Record<string, number>): number {
  const reportInterval = values.report_interval_s;
  if (Number.isFinite(reportInterval) && reportInterval > 0) return reportInterval;
  const interval = values.interval_s;
  return Number.isFinite(interval) && interval > 0 ? interval : 1;
}

function weightedMean(samples: MetricSample[], key: string): number | undefined {
  let total = 0;
  let weight = 0;
  for (const sample of samples) {
    const value = sample.values[key];
    if (!Number.isFinite(value)) continue;
    const weightForSample = sampleWeight(sample.values);
    total += value * weightForSample;
    weight += weightForSample;
  }
  return weight > 0 ? total / weight : undefined;
}

function counterDelta(samples: MetricSample[], key: string): number | undefined {
  if (samples.length < 2) return undefined;
  let total = 0;
  let observed = false;
  for (let index = 1; index < samples.length; index++) {
    const previous = samples[index - 1].values[key];
    const current = samples[index].values[key];
    if (!Number.isFinite(previous) || !Number.isFinite(current)) continue;
    observed = true;
    total += current >= previous ? current - previous : current;
  }
  return observed ? total : undefined;
}

export function aggregateTelemetry(samples: MetricSample[], counterBaseline?: MetricSample): Record<string, number> {
  const keys = new Set(samples.flatMap((sample) => Object.keys(sample.values)));
  const means: Record<string, number> = {};
  for (const key of keys) {
    if (["interval_s", "report_interval_s", "sampled_at_ms"].includes(key)) continue;
    if (key === "dropped") {
      const delta = counterDelta(counterBaseline ? [counterBaseline, ...samples] : samples, key);
      if (delta !== undefined) means[key] = delta;
    } else if (key === "q_max") {
      const values = samples.map((sample) => sample.values[key]).filter(Number.isFinite);
      if (values.length) means[key] = values.reduce((maximum, value) => Math.max(maximum, value));
    } else {
      const mean = weightedMean(samples, key);
      if (mean !== undefined) means[key] = mean;
    }
  }
  for (const key of ["skipped", "idr"]) {
    const rates = samples.flatMap(({ values }) => {
      const rate = values.interval_s > 0 ? values[key] / values.interval_s : NaN;
      return Number.isFinite(rate) ? [{ rate, weight: sampleWeight(values) }] : [];
    });
    const totalWeight = rates.reduce((sum, sample) => sum + sample.weight, 0);
    if (totalWeight > 0) {
      means[`${key}_per_s`] = rates.reduce((sum, sample) => sum + sample.rate * sample.weight, 0) / totalWeight;
    }
  }
  return means;
}

export function chartSeries(samples: MetricSample[], key: string, spanMs = 120_000) {
  const end = samples.at(-1)?.at;
  if (end === undefined) return [];
  const visible = samples.filter((sample) => sample.at >= end - spanMs);
  return visible.flatMap((sample, index) => {
    const previous = visible[index - 1];
    const interval = previous?.values.report_interval_s || previous?.values.interval_s;
    const gap = previous && interval > 0 && sample.at - previous.at > interval * 2500;
    return [
      ...(gap ? [{ x: (previous.at + interval * 1000 - end) / 1000, y: null }] : []),
      { x: (sample.at - end) / 1000, y: Number.isFinite(sample.values[key]) ? sample.values[key] : null },
    ];
  });
}

const SAMPLE_RETENTION_MS = 600_000;
const MAX_SAMPLES = 6000;

/** Appends a metrics report, dropping samples older than the retention window or beyond the size cap. */
export function appendSample(
  samples: MetricSample[],
  values: Record<string, number>,
  fallbackNow = Date.now(),
): MetricSample[] {
  const at = values.sampled_at_ms > 0 ? values.sampled_at_ms : fallbackNow;
  return [
    ...samples.filter((sample) => sample.at >= at - SAMPLE_RETENTION_MS).slice(-(MAX_SAMPLES - 1)),
    { at, values: { ...values } },
  ];
}

export type WindowSummary = { metrics: Record<string, number>; seconds: number; intervals: number };

/** Aggregates the samples of the last `windowSeconds`; undefined when the window holds none. */
export function summarizeWindow(
  samples: MetricSample[],
  windowSeconds: number,
  now = Date.now(),
): WindowSummary | undefined {
  const recent = samples.filter((sample) => sample.at >= now - windowSeconds * 1000);
  if (!recent.length) return undefined;
  const firstIndex = samples.indexOf(recent[0]);
  return {
    metrics: aggregateTelemetry(recent, firstIndex > 0 ? samples[firstIndex - 1] : undefined),
    seconds: recent.reduce(
      (sum, sample) => sum + (sample.values.report_interval_s || sample.values.interval_s || 0),
      0,
    ),
    intervals: recent.length,
  };
}
