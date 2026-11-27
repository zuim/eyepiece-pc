<script lang="ts">
  import ChoiceSelect from "./ChoiceSelect.svelte";
  import Icon from "./Icon.svelte";
  import PaneEditor from "./PaneEditor.svelte";
  import TransparencyEditor from "./TransparencyEditor.svelte";
  import { settingsHelp } from "../settings-help";
  import {
    bitrates,
    choiceLabel,
    currentSetting,
    encoderLabel,
    groupSettings,
    selectedBitrate,
    selectedChoice,
    settingSliderStep,
    titleCase,
    type Schema,
    type Setting,
  } from "../panel-model";
  let {
    schema,
    values,
    commandEdit = $bindable(),
    commandLine,
    applyCommand,
    copyText,
    resetSettings,
    changeSetting,
    changeAlphaSettings,
    changePaneDuringDrag,
    cancelPaneDragUpdate,
  }: {
    schema: Schema | undefined;
    values: Record<string, unknown>;
    commandEdit: string;
    commandLine: string;
    applyCommand: () => Promise<void>;
    copyText: (text: string) => Promise<void>;
    resetSettings: () => Promise<void>;
    changeSetting: (setting: Setting, value: unknown) => Promise<void>;
    changeAlphaSettings: (changes: Record<string, unknown>) => Promise<void>;
    changePaneDuringDrag: (value: string, mode: "move" | "resize") => void;
    cancelPaneDragUpdate: () => void;
  } = $props();

  // Key color settings are edited through the transparency editor.
  const hiddenSettings = ["color_key", "key_similarity", "key_blend"];
  const groups = $derived(groupSettings(schema, hiddenSettings));
</script>

<section class="panel page-panel">
  <div class="panel-heading">
    <div>
      <span class="eyebrow">CONFIGURATION</span>
      <h1>Settings</h1>
      <p>Changes persist in the local RPC worker and take effect on the next start unless live control is supported.</p>
    </div>
  </div>
  {#if schema}
    <section class="command-section">
      <label for="command-line">Equivalent CLI Command</label>
      <textarea id="command-line" rows="2" bind:value={commandEdit} spellcheck="false"></textarea>
      <div class="compare-actions">
        <button class="primary" onclick={applyCommand}>Apply Command</button>
        <button class="secondary" onclick={() => copyText(commandLine)}><Icon name="copy" /> Copy Command</button>
        <button class="secondary" onclick={resetSettings}><Icon name="restart" /> Reset to Defaults</button>
      </div>
    </section>
    <div class="settings-groups">
      {#each groups as group (group.name)}
        <details class="settings-group" open>
          <summary><h2>{group.name}</h2></summary>
          <div class="settings-grid">
            {#each group.settings as setting (setting.name)}
              {@const sliderStep = settingSliderStep(setting)}
              {#if setting.name === "pane"}
                <div class="pane-setting">
                  <PaneEditor
                    help={settingsHelp[setting.name] ?? setting.description}
                    value={typeof values.pane === "string" ? values.pane : null}
                    showPanel={false}
                    onChange={(value) => changeSetting(setting, value)}
                    onDragChange={changePaneDuringDrag}
                    onDragConflict={cancelPaneDragUpdate}
                  />
                </div>
              {:else if setting.name === "alpha"}
                <div class="setting-row">
                  <TransparencyEditor
                    help={settingsHelp[setting.name] ?? setting.description}
                    {values}
                    showTuning={false}
                    onChange={changeAlphaSettings}
                  />
                </div>
              {:else if setting.name === "encoder"}
                <div class="setting-row">
                  <span class="setting-label"
                    >{titleCase(setting.description)}<span class="setting-help"
                      >{settingsHelp[setting.name] ?? setting.description}</span
                    ></span
                  >
                  <span class="encoder-setting">
                    <ChoiceSelect
                      ariaLabel="Encoder Selection"
                      value={selectedChoice(setting, values)}
                      choices={setting.choices ?? []}
                      label={encoderLabel}
                      placeholder="Select a supported encoder"
                      onChange={(value) => changeSetting(setting, value)}
                    />
                    <input
                      aria-label="Custom Encoder"
                      type="text"
                      value={currentSetting(setting.name, values)}
                      onchange={(event) => changeSetting(setting, event.currentTarget.value)}
                    />
                  </span>
                </div>
              {:else if setting.name === "bitrate"}
                <label class="setting-row"
                  ><span class="setting-label"
                    >{titleCase(setting.description)}<span class="setting-help"
                      >{settingsHelp[setting.name] ?? setting.description}</span
                    ></span
                  >
                  <ChoiceSelect
                    ariaLabel="Bitrate"
                    value={selectedBitrate(values)}
                    choices={bitrates}
                    placeholder="Select bitrate"
                    onChange={(value) => changeSetting(setting, value)}
                  />
                </label>
              {:else}
                <label class="setting-row"
                  ><span class="setting-label"
                    >{titleCase(setting.description)}<span class="setting-help"
                      >{settingsHelp[setting.name] ?? setting.description}</span
                    ></span
                  >
                  {#if setting.type === "boolean"}
                    <input
                      type="checkbox"
                      checked={Boolean(values[setting.name])}
                      onchange={(event) => changeSetting(setting, event.currentTarget.checked)}
                    />
                  {:else if setting.type === "choice"}
                    <ChoiceSelect
                      value={selectedChoice(setting, values)}
                      choices={setting.choices ?? []}
                      label={(choice) => choiceLabel(setting.name, choice)}
                      onChange={(value) => changeSetting(setting, value)}
                    />
                  {:else if setting.type === "strings"}
                    <textarea
                      rows="2"
                      value={((values[setting.name] as string[]) ?? []).join("\n")}
                      onchange={(event) => changeSetting(setting, event.currentTarget.value)}
                    ></textarea>
                  {:else if sliderStep !== undefined}
                    <span class="range-setting">
                      <input
                        aria-label={setting.description}
                        type="range"
                        min={setting.min}
                        max={setting.max}
                        step={sliderStep}
                        value={currentSetting(setting.name, values)}
                        onchange={(event) => changeSetting(setting, event.currentTarget.value)}
                      />
                      <output>{currentSetting(setting.name, values)}</output>
                    </span>
                  {:else}
                    <input
                      type={setting.type.includes("integer") || setting.type.includes("number") ? "number" : "text"}
                      min={setting.min}
                      max={setting.max}
                      step={setting.type.includes("integer") ? "1" : "any"}
                      value={currentSetting(setting.name, values)}
                      placeholder={setting.type.startsWith("optional") ? "Not Set" : ""}
                      onchange={(event) => changeSetting(setting, event.currentTarget.value)}
                    />
                  {/if}
                </label>
              {/if}
            {/each}
          </div>
        </details>
      {/each}
    </div>
  {/if}
</section>

<style>
  .settings-group h2 {
    color: #dce5e1;
    font-size: 14px;
  }
  .settings-groups {
    display: grid;
    gap: 12px;
  }
  .command-section {
    display: grid;
    gap: 10px;
    margin-bottom: 24px;
    padding: 16px;
    border: 1px solid #34413b;
    border-radius: 10px;
    background: #111918;
  }
  .command-section > label {
    font-weight: 650;
  }
  .command-section textarea {
    width: 100%;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  }
  .command-section .compare-actions {
    margin-top: 0;
  }
  .settings-group {
    margin: 0;
    padding: 16px;
  }
  .settings-group summary {
    display: list-item;
  }
  .settings-group h2 {
    display: inline;
  }
  .settings-group .settings-grid {
    padding-top: 16px;
  }
  .settings-grid {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 16px;
  }
  .setting-row {
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
  }
  .pane-setting {
    display: grid;
    align-content: start;
    gap: 10px;
    min-width: 0;
  }
  .setting-row > input:not([type="checkbox"]),
  .setting-row > :global(select),
  .setting-row > textarea,
  .encoder-setting {
    width: min(52%, 260px);
  }
  .range-setting {
    width: min(52%, 260px);
  }
  .encoder-setting {
    display: grid;
    gap: 6px;
    min-width: 0;
  }
  .encoder-setting :global(select),
  .encoder-setting input {
    width: 100%;
  }
  @media (max-width: 550px) {
    .setting-row {
      align-items: flex-start;
      flex-direction: column;
    }
    .setting-row > input:not([type="checkbox"]),
    .setting-row > :global(select),
    .setting-row > textarea,
    .encoder-setting,
    .range-setting {
      width: 100%;
    }
  }
</style>
