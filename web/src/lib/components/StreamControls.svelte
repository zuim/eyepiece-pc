<script lang="ts">
  import ChoiceSelect from "./ChoiceSelect.svelte";
  import Icon from "./Icon.svelte";
  import {
    bitrates,
    choiceLabel,
    currentSetting,
    encoderLabel,
    findSetting,
    selectedBitrate,
    selectedChoice,
    titleCase,
    type Schema,
    type Setting,
  } from "../panel-model";
  import type { RpcCommand } from "../rpc";

  let {
    schema,
    values,
    running,
    settingsChanged,
    workerConnected,
    kmsOutputs,
    listingKmsOutputs,
    pickingFile,
    send,
    changeSetting,
    discoverKmsOutputs,
    browseVideoFile,
  }: {
    schema: Schema | undefined;
    values: Record<string, unknown>;
    running: boolean;
    settingsChanged: boolean;
    workerConnected: boolean;
    kmsOutputs: string[];
    listingKmsOutputs: boolean;
    pickingFile: boolean;
    send: (command: RpcCommand) => Promise<boolean>;
    changeSetting: (setting: Setting, value: unknown) => Promise<void>;
    discoverKmsOutputs: () => Promise<void>;
    browseVideoFile: () => Promise<void>;
  } = $props();

  const videoSettings = ["encoder", "codec", "bitrate", "fps"];
  const sourceFields: Record<string, string> = {
    kms: "output",
    x11: "region",
    file: "input",
    dxgi: "dxgi",
    screencapturekit: "display_id",
  };

  const captureSetting = $derived(findSetting(schema, "source"));
  const bridgeSetting = $derived(findSetting(schema, "no_launch"));
  const source = $derived(currentSetting("source", values));
  const sourceField = $derived(sourceFields[source] ?? "");
  const sourceParameter = $derived(findSetting(schema, sourceField));
  const output = $derived(currentSetting("output", values));
  const showVideo = $derived(
    videoSettings.some((name) => {
      const setting = findSetting(schema, name);
      return !!setting && setting.visible !== false;
    }),
  );
</script>

<section class="control-strip">
  <div class="control-header" class:restart-pending={settingsChanged}>
    <div>
      <span class="eyebrow">SESSION</span>
      <h2>Stream Controls</h2>
    </div>
    <div class="control-actions">
      <div class="stream-state-control" class:restart-pending={settingsChanged} role="group" aria-label="Stream state">
        <button
          class:danger={running}
          class="primary start-button"
          disabled={!workerConnected}
          onclick={() => send(running ? { stop: {} } : { start: {} })}
          ><Icon name={running ? "stop" : "play"} /> {running ? "Stop Stream" : "Start Stream"}</button
        >
        {#if settingsChanged}
          <div class="restart-control">
            <span class="restart-tooltip" id="restart-tooltip" role="tooltip">Settings changed; restart to apply.</span>
            <button class="secondary" aria-describedby="restart-tooltip" onclick={() => send({ restart: {} })}
              ><Icon name="restart" /> Restart</button
            >
          </div>
        {/if}
      </div>
    </div>
  </div>
  {#if schema}
    <div class="control-groups">
      {#if captureSetting?.visible !== false}
        <fieldset class="control-group capture-group">
          <legend>Capture</legend>
          {#if captureSetting}
            <label
              ><span>Capture</span><ChoiceSelect
                value={selectedChoice(captureSetting, values)}
                choices={captureSetting.choices ?? []}
                label={(choice) => choiceLabel("source", choice)}
                placeholder="Select a supported source"
                onChange={(value) => changeSetting(captureSetting, value)}
              /></label
            >
          {/if}
          {#if sourceParameter && sourceParameter.visible !== false}
            {#if sourceField === "output"}
              <label
                ><span>{titleCase(sourceParameter.description)}</span>
                <div class="kms-output-picker">
                  <select
                    aria-label="KMS output"
                    value={output}
                    onchange={(event) => changeSetting(sourceParameter, event.currentTarget.value)}
                  >
                    <option value="">First active output</option>
                    {#if output && !kmsOutputs.includes(output)}
                      <option value={output}>{output} (saved)</option>
                    {/if}
                    {#each kmsOutputs as kmsOutput}<option value={kmsOutput}>{kmsOutput}</option>{/each}
                  </select>
                  <button
                    class="secondary"
                    disabled={!workerConnected || listingKmsOutputs}
                    onclick={discoverKmsOutputs}
                  >
                    {listingKmsOutputs ? "Listing…" : "Discover"}
                  </button>
                </div>
              </label>
            {:else if sourceField === "input"}
              <label
                ><span>{titleCase(sourceParameter.description)}</span>
                <div class="file-picker">
                  <input
                    aria-label="Video file"
                    type="text"
                    placeholder="Path to a video file"
                    value={currentSetting(sourceField, values)}
                    onchange={(event) => changeSetting(sourceParameter, event.currentTarget.value)}
                  />
                  <button class="secondary" disabled={!workerConnected || pickingFile} onclick={browseVideoFile}>
                    {pickingFile ? "Choosing…" : "Browse…"}
                  </button>
                </div>
              </label>
            {:else}
              <label
                ><span>{titleCase(sourceParameter.description)}</span><input
                  type={sourceField === "display_id" ? "number" : "text"}
                  min={sourceParameter.min}
                  max={sourceParameter.max}
                  placeholder={sourceField === "region" ? "WxH+X+Y" : "Default"}
                  value={currentSetting(sourceField, values)}
                  onchange={(event) => changeSetting(sourceParameter, event.currentTarget.value)}
                /></label
              >
            {/if}
          {/if}
        </fieldset>
      {/if}
      {#if bridgeSetting && bridgeSetting.visible !== false}
        <fieldset class="control-group connection-group">
          <legend>Connection</legend>
          <label
            ><span>Bridge</span><select
              value={values.no_launch ? "running" : "start"}
              onchange={(event) => changeSetting(bridgeSetting, event.currentTarget.value === "running")}
              ><option value="start">Start Bridge</option><option value="running">Use Running Bridge</option></select
            ></label
          >
        </fieldset>
      {/if}
      {#if showVideo}
        <fieldset class="control-group video-group">
          <legend>Video</legend>
          {#each videoSettings as name}
            {@const setting = findSetting(schema, name)}
            {#if setting && setting.visible !== false}
              <label
                ><span>{titleCase(setting.description)}</span>
                {#if name === "encoder"}
                  <ChoiceSelect
                    value={selectedChoice(setting, values)}
                    choices={setting.choices ?? []}
                    label={encoderLabel}
                    placeholder="Select a supported encoder"
                    onChange={(value) => changeSetting(setting, value)}
                  />
                {:else if setting.type === "choice"}
                  <ChoiceSelect
                    value={selectedChoice(setting, values)}
                    choices={setting.choices ?? []}
                    label={(choice) => choiceLabel(setting.name, choice)}
                    onChange={(value) => changeSetting(setting, value)}
                  />
                {:else if name === "bitrate"}
                  <ChoiceSelect
                    ariaLabel="Bitrate"
                    value={selectedBitrate(values)}
                    choices={bitrates}
                    placeholder="Select bitrate"
                    onChange={(value) => changeSetting(setting, value)}
                  />
                {:else}
                  <input
                    aria-label="Frames per second"
                    type="number"
                    min={setting.min}
                    max={setting.max}
                    step="1"
                    value={currentSetting(name, values)}
                    onchange={(event) => changeSetting(setting, event.currentTarget.value)}
                  />
                {/if}
              </label>
            {/if}
          {/each}
        </fieldset>
      {/if}
    </div>
  {/if}
</section>

<style>
  .control-strip {
    display: grid;
    gap: 16px;
    margin: 18px 0;
    padding: 20px;
    border: 1px solid #303c38;
    border-radius: 14px;
    background: linear-gradient(125deg, #18211f, #131a1a);
  }
  .control-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 18px;
  }
  .control-header.restart-pending {
    padding-bottom: 44px;
  }
  .control-header h2 {
    margin: 4px 0 8px;
    font-size: 17px;
    letter-spacing: -0.025em;
  }
  .control-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .stream-state-control {
    display: flex;
  }
  .stream-state-control.restart-pending .start-button {
    border-radius: 8px 0 0 8px;
  }
  .restart-control {
    position: relative;
  }
  .restart-control button {
    height: 100%;
    border-radius: 0 8px 8px 0;
  }
  .restart-tooltip {
    position: absolute;
    z-index: 2;
    top: calc(100% + 9px);
    right: 0;
    width: max-content;
    max-width: min(280px, calc(100vw - 64px));
    padding: 7px 10px;
    border: 1px solid #806c48;
    border-radius: 8px;
    background: #282219;
    color: #e6c58e;
    font-size: 12px;
  }
  .restart-tooltip::after {
    content: "";
    position: absolute;
    bottom: 100%;
    right: 24px;
    width: 8px;
    height: 8px;
    border-left: 1px solid #806c48;
    border-top: 1px solid #806c48;
    background: #282219;
    transform: translateY(50%) rotate(45deg);
  }
  .control-groups {
    display: grid;
    grid-template-columns: minmax(190px, 1fr) minmax(230px, 1.15fr) minmax(400px, 2fr);
    gap: 12px;
  }
  .control-group {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    align-content: start;
    gap: 12px;
    min-width: 0;
    margin: 0;
    padding: 12px;
    border: 1px solid #303d39;
    border-radius: 10px;
    background: #111918;
  }
  .control-group legend {
    padding: 0 6px;
    color: #a9beb3;
    font-size: 11px;
    font-weight: 700;
  }
  .control-group label {
    display: grid;
    gap: 6px;
    min-width: 0;
    color: #91a39a;
    font-size: 11px;
    font-weight: 600;
  }
  .control-group input,
  .control-group :global(select) {
    width: 100%;
    height: 37px;
    font-size: 13px;
    font-weight: 400;
  }
  .kms-output-picker,
  .file-picker {
    display: flex;
    min-width: 0;
    gap: 6px;
  }
  .kms-output-picker select,
  .file-picker input {
    flex: 1;
  }
  .kms-output-picker button,
  .file-picker button {
    padding: 8px;
    white-space: nowrap;
  }
  .capture-group {
    grid-template-columns: 1fr;
  }
  .video-group {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }
  .start-button.danger {
    background: #4a2d2b;
    color: #ffc2ad;
  }
  @media (max-width: 1100px) {
    .control-groups {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
    .capture-group {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
    .video-group {
      grid-column: 1 / -1;
    }
  }
  @media (max-width: 550px) {
    .control-strip {
      padding: 14px;
    }
    .control-header {
      align-items: flex-start;
      flex-direction: column;
    }
    .control-groups {
      grid-template-columns: 1fr;
    }
    .video-group {
      grid-column: auto;
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
    .control-actions {
      width: 100%;
    }
    .stream-state-control {
      flex: 1 1 150px;
    }
    .stream-state-control .start-button {
      flex: 1;
    }
  }
</style>
