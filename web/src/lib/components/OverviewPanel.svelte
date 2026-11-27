<script lang="ts">
  import Icon from "./Icon.svelte";
  import PoseView from "./PoseView.svelte";
  import Preview from "./Preview.svelte";
  import PaneEditor from "./PaneEditor.svelte";
  import TransparencyEditor from "./TransparencyEditor.svelte";
  import { settingsHelp } from "../settings-help";
  import type { eyepiece } from "../generated/eyepiece-rpc.js";
  import type { Pose } from "../pose-model";
  import { currentSetting, findSetting, formatMetric, type Metrics, type Schema, type Setting } from "../panel-model";
  import type { RpcCommand } from "../rpc";
  import { PREVIEW_FPS, PREVIEW_WIDTHS, type PreviewSettings } from "../ui-preferences";
  let {
    preview = $bindable(),
    previewSample,
    previewHasAlpha,
    pose,
    latest,
    schema,
    values,
    trackingImplied,
    trackingOn,
    running,
    changeSetting,
    changeAlphaSettings,
    changePaneDuringDrag,
    cancelPaneDragUpdate,
    send,
  }: {
    preview: PreviewSettings;
    previewSample: eyepiece.rpc.v1.IPreview | undefined;
    previewHasAlpha: boolean;
    pose: Pose | null;
    latest: Metrics;
    schema: Schema | undefined;
    values: Record<string, unknown>;
    trackingImplied: boolean;
    trackingOn: boolean;
    running: boolean;
    changeSetting: (setting: Setting, value: unknown) => Promise<void>;
    changeAlphaSettings: (changes: Record<string, unknown>) => Promise<void>;
    changePaneDuringDrag: (value: string, mode: "move" | "resize") => void;
    cancelPaneDragUpdate: () => void;
    send: (command: RpcCommand) => Promise<boolean>;
  } = $props();

  const headTrackingSetting = $derived(findSetting(schema, "head_tracking"));
  const paneSetting = $derived(findSetting(schema, "pane"));
  const alphaSetting = $derived(findSetting(schema, "alpha"));
  const opacitySettings = $derived(
    ["pane_opacity", "gui_opacity"]
      .map((name) => findSetting(schema, name))
      .filter((setting): setting is Setting => !!setting && setting.visible !== false),
  );
</script>

<section class="overview-grid">
  <article class="panel stream-panel">
    <div class="panel-heading">
      <div>
        <span class="eyebrow">LIVE OUTPUT</span>
        <h1>Stream Preview</h1>
      </div>
      <button
        class="icon-button"
        aria-label={preview.on ? "Pause Preview" : "Start Preview"}
        title={preview.on ? "Pause Preview" : "Start Preview"}
        aria-pressed={preview.on}
        onclick={() => (preview.on = !preview.on)}><Icon name={preview.on ? "pause" : "play"} /></button
      >
    </div>
    <div class="visual-slot">
      <Preview
        sample={previewSample}
        enabled={preview.on}
        maxFps={preview.maxFps}
        maxWidth={preview.width}
        alphaMode={preview.alphaMode}
      />
    </div>
    <details class="preview-tuning">
      <summary>Preview tuning</summary>
      <div class="preview-controls">
        <label
          >Max FPS
          <select bind:value={preview.maxFps}>
            {#each PREVIEW_FPS as fps}<option value={fps}>{fps}</option>{/each}
          </select>
        </label>
        <label
          >Max width
          <select bind:value={preview.width}>
            {#each PREVIEW_WIDTHS as width}<option value={width}>{width}px</option>{/each}
          </select>
        </label>
        <label class="preview-check">
          <input type="checkbox" bind:checked={preview.keyframesOnly} />
          Keyframes only
        </label>
        {#if previewHasAlpha}
          <label
            >Alpha view
            <select bind:value={preview.alphaMode}>
              <option value="composite">Composite</option>
              <option value="mask">Alpha mask</option>
            </select>
          </label>
        {/if}
      </div>
    </details>
  </article>
  <article class="panel pose-panel">
    <div class="panel-heading">
      <div>
        <span class="eyebrow">IMU</span>
        <h2>Head Pose</h2>
      </div>
      <div class="panel-actions">
        <button
          class="icon-button"
          disabled={!headTrackingSetting || trackingImplied}
          aria-label={trackingOn ? "Stop Head Tracking" : "Start Head Tracking"}
          title={trackingImplied
            ? "Required by Head Tracking Only or Print Pose"
            : trackingOn
              ? "Stop Head Tracking"
              : "Start Head Tracking"}
          aria-pressed={trackingOn}
          onclick={() => headTrackingSetting && changeSetting(headTrackingSetting, !values.head_tracking)}
          ><Icon name={trackingOn ? "stop" : "play"} /></button
        >
        <button
          class="icon-button"
          disabled={!running || !trackingOn}
          aria-label="Recenter Head Pose"
          title="Recenter Head Pose"
          onclick={() => send({ recenter: {} })}><Icon name="recenter" /></button
        >
      </div>
    </div>
    <div class="visual-slot"><PoseView {pose} /></div>
    <div class="pose-readout">
      <div><span>YAW</span><strong>{pose ? `${pose.yaw.toFixed(1)}°` : "—"}</strong></div>
      <div><span>PITCH</span><strong>{pose ? `${pose.pitch.toFixed(1)}°` : "—"}</strong></div>
      <div><span>ROLL</span><strong>{pose ? `${pose.roll.toFixed(1)}°` : "—"}</strong></div>
      <div><span>RATE</span><strong>{formatMetric(latest.pose_hz, "Hz")}</strong></div>
    </div>
  </article>
  {#if paneSetting && paneSetting.visible !== false}
    <article class="panel editor-panel">
      <div class="panel-heading">
        <div>
          <span class="eyebrow">VIDEO</span>
          <h2>Picture-in-Picture</h2>
        </div>
      </div>
      <PaneEditor
        help={settingsHelp.pane ?? paneSetting.description}
        value={typeof values.pane === "string" ? values.pane : null}
        onChange={(value) => {
          if (paneSetting) void changeSetting(paneSetting, value);
        }}
        onDragChange={changePaneDuringDrag}
        onDragConflict={cancelPaneDragUpdate}
      />
    </article>
  {/if}
  {#if (alphaSetting && alphaSetting.visible !== false) || opacitySettings.length}
    <article class="panel editor-panel">
      <div class="panel-heading">
        <div>
          <span class="eyebrow">VIDEO</span>
          <h2>Transparency</h2>
        </div>
      </div>
      <div class="transparency-controls">
        {#if alphaSetting && alphaSetting.visible !== false}
          <TransparencyEditor
            help={settingsHelp.alpha ?? alphaSetting.description}
            {values}
            onChange={changeAlphaSettings}
          />
        {/if}
        {#each opacitySettings as setting}
          <label class="opacity"
            ><span class="setting-label"
              >{setting.description}<span class="setting-help">{settingsHelp[setting.name] ?? setting.description}</span
              ></span
            >
            <span class="range-setting"
              ><input
                aria-label={setting.description}
                type="range"
                min={setting.min}
                max={setting.max}
                step="1"
                value={currentSetting(setting.name, values)}
                onchange={(event) => changeSetting(setting, event.currentTarget.value)}
              /><output>{currentSetting(setting.name, values)}</output></span
            >
          </label>
        {/each}
      </div>
    </article>
  {/if}
</section>

<style>
  .overview-grid {
    display: grid;
    grid-template-columns: minmax(0, 1.65fr) minmax(280px, 0.75fr);
    gap: 16px;
    padding-top: 18px;
  }
  .stream-panel,
  .pose-panel {
    display: flex;
    flex-direction: column;
    height: clamp(380px, 35vw, 560px);
    min-width: 0;
    padding: 20px;
  }
  .editor-panel {
    min-width: 0;
    padding: 20px;
    align-self: start;
  }
  .transparency-controls {
    display: grid;
    gap: 16px;
  }
  .opacity {
    display: grid;
    gap: 8px;
    min-width: 0;
  }
  .visual-slot {
    flex: 1;
    min-height: 0;
  }
  .pose-readout span {
    color: #81908a;
    font-size: 11px;
  }
  .panel-actions {
    display: flex;
    gap: 8px;
  }
  .preview-tuning {
    margin-top: 12px;
    padding: 9px 12px;
    font-size: 12px;
  }
  .preview-controls {
    display: flex;
    flex-wrap: wrap;
    align-items: end;
    gap: 12px;
    padding-top: 10px;
  }
  .preview-controls label {
    display: grid;
    gap: 5px;
    color: #91a39a;
    font-size: 11px;
  }
  .preview-controls select {
    min-width: 105px;
    padding: 7px 8px;
  }
  .preview-controls .preview-check {
    display: flex;
    align-items: center;
    padding-bottom: 8px;
  }
  .icon-button {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    padding: 0;
  }
  .icon-button[aria-pressed="true"] {
    border-color: #4e826a;
    color: #9cddc9;
  }
  .pose-readout {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 8px;
    padding: 14px 2px 0;
  }
  .pose-readout div {
    display: grid;
    gap: 5px;
  }
  .pose-readout strong {
    font-size: 15px;
    font-variant-numeric: tabular-nums;
  }
  @media (max-width: 850px) {
    .overview-grid {
      grid-template-columns: 1fr;
    }
  }
  @media (max-width: 550px) {
    .stream-panel,
    .pose-panel,
    .editor-panel {
      padding: 14px;
    }
  }
</style>
