<script lang="ts">
  import Icon from "./Icon.svelte";
  import {
    bestPinId,
    comparisonMarkdown,
    describeSettings,
    metricRows,
    pinMetricText,
    type Pin,
    type Schema,
  } from "../panel-model";
  import type { MetricSample } from "../telemetry";

  let {
    pins = $bindable(),
    schema,
    running,
    samples,
    pinResults,
    copyText,
  }: {
    pins: Pin[];
    schema: Schema | undefined;
    running: boolean;
    samples: MetricSample[];
    pinResults: () => void;
    copyText: (text: string) => Promise<void>;
  } = $props();

  const selectedPins = $derived(pins.filter((pin) => pin.selected));
  const bestByMetric = $derived(new Map(metricRows.map(([key]) => [key, bestPinId(key, selectedPins)])));

  function updatePin(id: number, changes: Partial<Pin>) {
    pins = pins.map((pin) => (pin.id === id ? { ...pin, ...changes } : pin));
  }
  function selectAll(selected: boolean) {
    pins = pins.map((pin) => ({ ...pin, selected }));
  }
</script>

<section class="panel page-panel">
  <div class="panel-heading">
    <div>
      <span class="eyebrow">BENCHMARKS</span>
      <h1>Compare Runs</h1>
      <p>Pin running settings and mean statistics over the selected window.</p>
    </div>
    <button class="primary" disabled={!running || !samples.length} onclick={pinResults}
      ><Icon name="pin" /> Pin Current Run</button
    >
  </div>
  {#if pins.length === 0}<div class="empty-state">
      No runs pinned. Start streaming, wait for statistics, then pin a run.
    </div>
  {:else}
    <div class="compare-actions">
      <button class="secondary" onclick={() => selectAll(true)}>Select All</button>
      <button class="secondary" onclick={() => selectAll(false)}>Select None</button>
      <button
        class="secondary"
        disabled={!selectedPins.length}
        onclick={() => (pins = pins.filter((pin) => !pin.selected))}><Icon name="trash" /> Delete Selected</button
      >
      <button
        class="secondary"
        disabled={!selectedPins.length}
        onclick={() => copyText(comparisonMarkdown(selectedPins))}><Icon name="copy" /> Copy Table (Markdown)</button
      >
    </div>
    <div class="pin-selection">
      {#each pins as pin}<label
          ><input
            type="checkbox"
            checked={pin.selected}
            onchange={(event) => updatePin(pin.id, { selected: event.currentTarget.checked })}
          />
          {pin.label}</label
        >{/each}
    </div>
    <div class="table-wrap">
      <table>
        <thead
          ><tr
            ><th>Metric</th>{#each selectedPins as pin}<th
                ><input
                  aria-label="Run Name"
                  value={pin.label}
                  onchange={(event) => updatePin(pin.id, { label: event.currentTarget.value.trim() || pin.label })}
                /><button
                  class="remove-pin"
                  aria-label="Remove {pin.label}"
                  onclick={() => (pins = pins.filter((item) => item.id !== pin.id))}>×</button
                ></th
              >{/each}</tr
          ></thead
        >
        <tbody
          ><tr
            ><th>Time</th>{#each selectedPins as pin}<td>{new Date(pin.time).toLocaleString()}</td>{/each}</tr
          ><tr
            ><th>Window</th>{#each selectedPins as pin}<td>{pin.seconds.toFixed(1)} s ({pin.intervals} intervals)</td
              >{/each}</tr
          >{#each metricRows as [key, label]}<tr
              ><th>{label}</th>{#each selectedPins as pin}<td class:best={bestByMetric.get(key) === pin.id}
                  >{pinMetricText(key, pin)}</td
                >{/each}</tr
            >{/each}</tbody
        >
      </table>
    </div>
    <div class="compare-settings">
      <h2>Configuration Snapshots</h2>
      {#each selectedPins as pin}<details>
          <summary>{pin.label}</summary>
          <pre>{pin.command}</pre>
          <pre>{describeSettings(pin.settings, schema)}</pre>
        </details>{/each}
    </div>
  {/if}
</section>

<style>
  .remove-pin {
    margin-left: 8px;
    border: 0;
    background: none;
    color: inherit;
    font-size: 15px;
  }
  .pin-selection {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 10px;
    margin-top: 14px;
  }
  .empty-state {
    display: grid;
    min-height: 200px;
    place-items: center;
    padding: 24px;
    border: 1px dashed #34423d;
    border-radius: 10px;
    background: #101716;
    color: #91a39a;
    text-align: center;
  }
  td.best {
    color: #9cddc9;
    font-weight: 750;
  }
  .compare-settings {
    margin-top: 24px;
  }
  .compare-settings h2 {
    color: #dce5e1;
    font-size: 14px;
  }
</style>
