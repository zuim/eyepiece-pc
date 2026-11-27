<script lang="ts">
  import Icon from "./Icon.svelte";
  import TelemetryChart from "./TelemetryChart.svelte";
  import {
    formatMetric,
    latencyRows,
    throughputRows,
    latencySeries,
    rateSeries,
    type Metrics,
    type Pin,
  } from "../panel-model";
  import type { MetricSample } from "../telemetry";

  let {
    latest,
    samples,
    running,
    pins,
    pinWindow = $bindable(),
    pinResults,
  }: {
    latest: Metrics;
    samples: MetricSample[];
    running: boolean;
    pins: Pin[];
    pinWindow: number;
    pinResults: () => void;
  } = $props();
</script>

<section class="panel page-panel metrics-panel">
  <div class="panel-heading">
    <div>
      <span class="eyebrow">PERFORMANCE</span>
      <h1>Stream Statistics</h1>
      <p>Rolling sample window: {formatMetric(latest.interval_s, "s")}</p>
    </div>
  </div>
  <div class="telemetry-tables">
    <div>
      <h2>Latency</h2>
      <div class="table-wrap">
        <table>
          <thead><tr><th>Stage</th><th>P50 (ms)</th><th>P95 (ms)</th></tr></thead><tbody>
            {#each latencyRows as [label, p50, p95]}<tr
                ><th>{label}</th><td>{formatMetric(latest[p50], "")}</td><td>{formatMetric(latest[p95], "")}</td></tr
              >{/each}
          </tbody>
        </table>
      </div>
    </div>
    <div>
      <h2>Throughput</h2>
      <div class="table-wrap">
        <table>
          <tbody>
            {#each throughputRows as [label, key, unit]}<tr
                ><th>{label}</th><td>{formatMetric(latest[key], unit)}</td></tr
              >{/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
  <div class="chart-wrap">
    <h2>Latency History</h2>
    <TelemetryChart {samples} label="Latency History" unit="ms" series={latencySeries} />
  </div>
  <div class="chart-wrap">
    <h2>Stream Rate and Bitrate</h2>
    <TelemetryChart
      {samples}
      label="Stream Rate and Bitrate History"
      unit="fps"
      secondaryUnit="Mbit/s"
      series={rateSeries}
    />
  </div>
  <div class="pin-controls">
    <button class="secondary" disabled={!running || !samples.length} onclick={pinResults}
      ><Icon name="pin" /> Pin Current Run</button
    >
    <label>Last <input type="number" min="1" max="600" bind:value={pinWindow} /> Seconds</label>
    <span class="subtle">{pins.length} Pins</span>
  </div>
</section>

<style>
  .subtle {
    color: #81908a;
    font-size: 11px;
  }
  .telemetry-tables {
    display: grid;
    grid-template-columns: minmax(0, 1.4fr) minmax(0, 1fr);
    gap: 24px;
  }
  .telemetry-tables h2,
  .chart-wrap h2 {
    font-size: 14px;
  }
  .chart-wrap {
    margin-top: 28px;
  }
  .pin-controls {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 10px;
    margin-top: 14px;
  }
  .pin-controls input {
    width: 75px;
  }
  @media (max-width: 850px) {
    .telemetry-tables {
      grid-template-columns: 1fr;
    }
  }
</style>
