<script module lang="ts">
  import { Chart, Legend, LineController, LineElement, LinearScale, PointElement, Tooltip } from "chart.js";

  Chart.register(LineController, LineElement, LinearScale, PointElement, Tooltip, Legend);
</script>

<script lang="ts">
  import { onMount } from "svelte";
  import { chartSeries, type MetricSample } from "../telemetry";

  let {
    samples = [],
    label,
    unit,
    secondaryUnit = "",
    series,
  }: {
    samples?: MetricSample[];
    label: string;
    unit: string;
    secondaryUnit?: string;
    series: { key: string; label: string; color: string; secondary?: boolean }[];
  } = $props();

  let canvas: HTMLCanvasElement;
  let chart: Chart<"line"> | undefined;

  onMount(() => {
    chart = new Chart(canvas, {
      type: "line",
      data: { datasets: [] },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        animation: false,
        normalized: true,
        interaction: { mode: "index", intersect: false },
        plugins: {
          legend: { labels: { color: "#b7c4be", boxWidth: 12, usePointStyle: true } },
        },
        scales: {
          x: {
            type: "linear",
            min: -120,
            max: 0,
            ticks: { color: "#81908a", callback: (value) => (Number(value) === 0 ? "Now" : `${value} s`) },
            grid: { color: "#293131" },
          },
          y: {
            beginAtZero: true,
            suggestedMax: 5,
            title: { display: true, text: unit, color: "#81908a" },
            ticks: { color: "#81908a" },
            grid: { color: "#293131" },
          },
          ...(secondaryUnit
            ? {
                y1: {
                  position: "right" as const,
                  beginAtZero: true,
                  title: { display: true, text: secondaryUnit, color: "#81908a" },
                  ticks: { color: "#81908a" },
                  grid: { drawOnChartArea: false },
                },
              }
            : {}),
        },
      },
    });
    update();
    return () => {
      chart?.destroy();
      chart = undefined;
    };
  });

  function update() {
    if (!chart) return;
    chart.data.datasets = series.map((item) => {
      const data = chartSeries(samples, item.key);
      return {
        label: item.label,
        data,
        borderColor: item.color,
        backgroundColor: item.color,
        yAxisID: item.secondary ? "y1" : "y",
        borderWidth: 1.5,
        pointRadius: (context) => {
          const index = context.dataIndex;
          return data[index]?.y != null && data[index - 1]?.y == null && data[index + 1]?.y == null ? 2 : 0;
        },
        pointHitRadius: 8,
        spanGaps: false,
      };
    });
    chart.update("none");
  }

  // `update` reads `samples` and `series`, so this re-runs when either changes.
  $effect(() => update());
</script>

<div class="chart">
  <canvas bind:this={canvas} aria-label={label}></canvas>
  {#if !samples.length}<span>No Statistics Yet</span>{/if}
</div>

<style>
  .chart {
    position: relative;
    height: 260px;
  }
  span {
    position: absolute;
    inset: 50% 0 auto;
    color: #81908a;
    text-align: center;
    pointer-events: none;
  }
</style>
