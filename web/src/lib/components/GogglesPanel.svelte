<script lang="ts">
  // The goggles over tinkerglass' control link (control 1): the tinkerglass status, the apps
  // and every mod's settings. A change goes to the goggles at once; they check it against their
  // installed schema, and the new state comes back as a report (the goggles' own menu and the
  // tinkerglass web app change the same settings, and those show up here too).
  import type { eyepiece } from "$lib/generated/eyepiece-rpc.js";

  type Goggles = eyepiece.rpc.v1.IGoggles;
  type Setting = eyepiece.rpc.v1.IGogglesSetting;

  let { goggles, set }: { goggles: Goggles | null | undefined; set: (mod: string, key: string, value: string) => void } = $props();

  // a slider shows its value while it moves; it is sent when let go
  let sliding = $state<Record<string, string>>({});
  const id = (mod: string, s: Setting) => `${mod}.${s.key}`;
</script>

<section class="panel page-panel">
  <div class="panel-heading">
    <div>
      <span class="eyebrow">TINKERGLASS</span>
      <h1>Goggles</h1>
    </div>
  </div>

  {#if !goggles?.connected}
    <p class="muted">{goggles?.message || "Looking for the goggles…"}</p>
    <p class="muted small">This page talks to the goggles over tinkerglass' control interface ("control 1"), which
      exists while the tinkerglass mods run in its USB mode (start at power-on).</p>
  {:else}
    <div class="facts">
      <span>tinkerglass {goggles.version}</span>
      <span class:bad={!goggles.healthy}>{goggles.healthy ? "running, healthy" : "starting or unhealthy"}{goggles.session ? ` · ${goggles.session}` : ""}</span>
      {#each goggles.apps ?? [] as a (a.entry)}
        {#if a.running}<span>{a.label} runs</span>{/if}
      {/each}
    </div>
    {#if goggles.message}<p class="muted small">{goggles.message}</p>{/if}

    <div class="mods">
      {#each goggles.mods ?? [] as m (m.mod)}
        <div class="mod">
          <h2>{m.name || m.mod}</h2>
          {#each m.settings ?? [] as s (s.key)}
            {@const key = id(m.mod ?? "", s)}
            {#if s.type === "bool"}
              <label class="check"><input type="checkbox" checked={s.value === "1"}
                onchange={(e) => set(m.mod ?? "", s.key ?? "", e.currentTarget.checked ? "1" : "0")} /> {s.label}</label>
            {:else if s.type === "enum"}
              <label class="setting-label">{s.label}
                <select value={s.value} onchange={(e) => set(m.mod ?? "", s.key ?? "", e.currentTarget.value)}>
                  {#each s.options ?? [] as o (o)}<option>{o}</option>{/each}
                </select></label>
            {:else if s.type === "int"}
              <label class="setting-label">{s.label} <span class="muted">{sliding[key] ?? s.value}{s.unit}</span>
                <input type="range" min={s.min} max={s.max} step={s.step || 1} value={Number(s.value)}
                  oninput={(e) => (sliding[key] = e.currentTarget.value)}
                  onchange={(e) => { set(m.mod ?? "", s.key ?? "", e.currentTarget.value); delete sliding[key]; }} /></label>
            {:else}
              <label class="setting-label">{s.label}
                <input type="text" value={s.value} onchange={(e) => set(m.mod ?? "", s.key ?? "", e.currentTarget.value)} /></label>
            {/if}
            {#if !s.live}<span class="muted small">Applies after a short reload of the goggles' menu.</span>{/if}
          {/each}
        </div>
      {:else}
        <p class="muted">No mod on the goggles has settings.</p>
      {/each}
    </div>
  {/if}
</section>

<style>
  .facts { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 14px; }
  .facts span { padding: 4px 10px; border-radius: 999px; background: #17201e; font-size: 13px; }
  .facts .bad { background: #3a2a14; }
  .mods { display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 14px; }
  .mod { display: grid; gap: 10px; align-content: start; padding: 14px; border: 1px solid #303b37; border-radius: 10px; background: #0b100f; }
  h2 { margin: 0; font-size: 16px; }
  .check { display: flex; gap: 8px; align-items: center; }
  .muted { color: #8a9893; }
  .small { font-size: 12px; }
</style>
