<script lang="ts">
  import { settingsHelp } from "../settings-help";
  import { colorPickerValue, currentSetting } from "../panel-model";

  let {
    help,
    values,
    onChange,
    showTuning = true,
  }: {
    help: string;
    values: Record<string, unknown>;
    onChange: (changes: Record<string, unknown>) => void;
    showTuning?: boolean;
  } = $props();

  const alpha = $derived(currentSetting("alpha", values));
  const colorKey = $derived(typeof values.color_key === "string" ? values.color_key : null);
  const keySimilarity = $derived(typeof values.key_similarity === "number" ? values.key_similarity : 0);
  const keyBlend = $derived(typeof values.key_blend === "number" ? values.key_blend : 0);
  const enabled = $derived(alpha === "source" || colorKey !== null);
  const source = $derived(alpha === "source" ? "source" : "color_key");

  function toggleTransparency(event: Event) {
    const checked = (event.currentTarget as HTMLInputElement).checked;
    onChange({ alpha: "opaque", color_key: checked ? "00ff00" : null });
  }

  function changeSource(event: Event) {
    const selected = (event.currentTarget as HTMLSelectElement).value;
    onChange(
      selected === "source"
        ? { alpha: "source", color_key: null }
        : { alpha: "opaque", color_key: colorKey ?? "00ff00" },
    );
  }
</script>

<div class="transparency-editor">
  <label class="toggle"
    ><span class="setting-label">Enable Transparency<span class="setting-help">{help}</span></span><input
      aria-label="Enable Transparency"
      type="checkbox"
      checked={enabled}
      onchange={toggleTransparency}
    /></label
  >
  {#if enabled}
    <label class="mode-setting"
      ><span class="setting-label">Alpha Source<span class="setting-help">{settingsHelp.alpha_source}</span></span>
      <select aria-label="Alpha Source" value={source} onchange={changeSource}>
        <option value="source">Source alpha</option>
        <option value="color_key">Key Color</option>
      </select>
    </label>
    {#if source === "color_key" && showTuning}
      <div class="color-row">
        <span class="setting-label">Key Color<span class="setting-help">{settingsHelp.color_key}</span></span>
        <div class="color-setting">
          <input
            aria-label="Key Color picker"
            type="color"
            value={colorPickerValue(colorKey ?? "00ff00")}
            oninput={(event) => onChange({ color_key: event.currentTarget.value.slice(1) })}
          />
          <input
            aria-label="Key Color RRGGBB"
            type="text"
            value={colorKey ?? "00ff00"}
            placeholder="RRGGBB"
            oninput={(event) => {
              if (/^#?[\da-f]{6}$/i.test(event.currentTarget.value)) {
                onChange({ color_key: event.currentTarget.value.replace(/^#/, "") });
              }
            }}
            onchange={(event) => onChange({ color_key: event.currentTarget.value.replace(/^#/, "") })}
          />
        </div>
      </div>
      <label class="number-setting">
        <span class="setting-label">Tolerance<span class="setting-help">{settingsHelp.key_similarity}</span></span>
        <span class="range-setting"
          ><input
            aria-label="Key Color tolerance"
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={keySimilarity}
            oninput={(event) => onChange({ key_similarity: Number(event.currentTarget.value) })}
          /><output>{keySimilarity.toFixed(2)}</output></span
        >
      </label>
      <label class="number-setting">
        <span class="setting-label">Softness<span class="setting-help">{settingsHelp.key_blend}</span></span>
        <span class="range-setting"
          ><input
            aria-label="Key Color softness"
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={keyBlend}
            oninput={(event) => onChange({ key_blend: Number(event.currentTarget.value) })}
          /><output>{keyBlend.toFixed(2)}</output></span
        >
      </label>
    {/if}
  {/if}
</div>

<style>
  .transparency-editor {
    display: grid;
    gap: 16px;
    min-width: 0;
    width: 100%;
  }
  .toggle,
  .mode-setting,
  .color-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    color: var(--label-color);
  }
  .number-setting {
    display: grid;
    gap: 8px;
    min-width: 0;
    color: var(--label-color);
  }
  .mode-setting select {
    width: min(52%, 260px);
  }
  .color-setting {
    display: flex;
    gap: 6px;
    width: min(52%, 260px);
    min-width: 0;
    margin-left: auto;
  }
  .color-setting input[type="color"] {
    width: 42px;
    min-width: 42px;
    height: 36px;
    padding: 3px;
  }
  .color-setting input[type="text"] {
    width: 100%;
    padding: 8px;
  }
  @media (max-width: 700px) {
    .color-setting,
    .mode-setting select {
      width: 100%;
    }
    .mode-setting,
    .color-row {
      align-items: flex-start;
      flex-direction: column;
    }
  }
</style>
