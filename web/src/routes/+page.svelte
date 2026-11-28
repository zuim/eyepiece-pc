<script lang="ts">
  import { onMount, untrack } from "svelte";
  import type { eyepiece } from "$lib/generated/eyepiece-rpc.js";
  import ComparePanel from "$lib/components/ComparePanel.svelte";
  import ConfirmDialog from "$lib/components/ConfirmDialog.svelte";
  import { errorHelp } from "$lib/error-help";
  import LogPanel from "$lib/components/LogPanel.svelte";
  import OverviewPanel from "$lib/components/OverviewPanel.svelte";
  import GogglesPanel from "$lib/components/GogglesPanel.svelte";
  import SettingsPanel from "$lib/components/SettingsPanel.svelte";
  import StatisticsPanel from "$lib/components/StatisticsPanel.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import StreamControls from "$lib/components/StreamControls.svelte";
  import {
    coerceSettingValue,
    findSetting,
    trackingEnabled,
    type Metrics,
    type Pin,
    type Schema,
    type Setting,
  } from "$lib/panel-model";
  import { Connection } from "$lib/connection";
  import { OptimisticSettings } from "$lib/optimistic-settings";
  import { PaneDragThrottle } from "$lib/pane-drag-throttle";
  import type { Pose } from "$lib/pose-model";
  import { metricValues, pickVideoFile, quitApp, previewCommand, propertyValue, wireValue, type RpcCommand, type RpcResponse } from "$lib/rpc";
  import { appendSample, summarizeWindow, type MetricSample } from "$lib/telemetry";
  import {
    defaultPreferences,
    loadPreferences,
    nextPinId,
    savePreferences,
    type PreviewSettings,
  } from "$lib/ui-preferences";

  const tabs = ["Overview", "Goggles", "Statistics", "Compare", "Settings", "Log"];
  const MAX_LOG_LINES = 500;
  const UNDO_TIMEOUT_MS = 20_000;

  let schema = $state.raw<Schema>();
  let values = $state.raw<Record<string, unknown>>({});
  let runningValues = $state.raw<Record<string, unknown>>({});
  let commandLine = $state("");
  let runningCommandLine = $state("");
  let commandEdit = $state("");
  let phase = $state("connecting");
  let workerStatus = $state("starting");
  let desiredRunning = $state(false);
  let connected = $state(false);
  let preview = $state<PreviewSettings>({ ...defaultPreferences.preview });
  let pinWindow = $state(defaultPreferences.pinWindow);
  let pins = $state.raw<Pin[]>(defaultPreferences.pins);
  let previewSample = $state.raw<eyepiece.rpc.v1.IPreview>();
  let previewSubscription = $state("");
  let previewSubscribing = $state(false);
  let kmsOutputs = $state.raw<string[]>([]);
  let listingKmsOutputs = $state(false);
  let pickingFile = $state(false);
  let pose = $state.raw<Pose | null>(null);
  let latest = $state.raw<Metrics>({});
  let samples = $state.raw<MetricSample[]>([]);
  let logs = $state.raw<string[]>([]);
  let activeTab = $state("Overview");
  /** the goggles over tinkerglass' control link (control 1), as the PC side reports them */
  let goggles = $state<RpcResponse["goggles"]>(null);
  let statusMessage = $state("");
  const statusHelp = $derived(errorHelp(statusMessage));
  let resetUndo = $state.raw<Record<string, unknown>>();
  let confirmingReset = $state(false);
  let confirmingQuit = $state(false);
  let quit = $state(false);
  let preferencesReady = $state(false);
  // Not rendered, so plain variables.
  let subscribedPreviewOn = false;
  let subscribedKeyframesOnly = false;
  let nextPin = 1;
  let undoTimer: ReturnType<typeof setTimeout> | undefined;
  let disposed = false;
  const settingEdits = new OptimisticSettings();
  const connection = new Connection({
    onEvent: consume,
    onStatus: (status) => {
      workerStatus = status;
      if (status === "disconnected") pose = null;
    },
    onError: (message) => (statusMessage = message),
    onConnecting: () => {
      previewSubscription = "";
      previewSample = undefined;
      subscribedPreviewOn = false;
      subscribedKeyframesOnly = false;
    },
  });
  const paneDrag = new PaneDragThrottle(
    (value) => submitSettings({ pane: value }),
    () => values.pane,
  );

  function errorMessage(error: unknown): string {
    return error instanceof Error ? error.message : String(error);
  }

  function consume(response: RpcResponse) {
    if (response.state) {
      const nextPhase = response.state.phase ?? "unknown";
      if (nextPhase === "starting" && phase !== "starting") {
        samples = [];
        latest = {};
      }
      phase = nextPhase;
      desiredRunning = response.state.desiredRunning ?? false;
      connected = response.state.connected ?? false;
      try {
        schema = JSON.parse(response.state.settingsSchemaJson ?? "{}") as Schema;
      } catch (error) {
        statusMessage = `Invalid settings schema: ${errorMessage(error)}`;
        return;
      }
      values = settingEdits.reconcile(propertyMap(response.state.properties));
      runningValues = propertyMap(response.state.runningProperties);
      const nextCommand = response.state.commandLine ?? "";
      if (nextCommand !== commandLine) commandEdit = nextCommand;
      commandLine = nextCommand;
      runningCommandLine = response.state.runningCommandLine ?? "";
      if (!connected || !trackingEnabled(values)) pose = null;
    } else if (response.metrics) {
      latest = metricValues(response.metrics);
      samples = appendSample(samples, latest);
    } else if (response.pose) {
      if (trackingEnabled(values)) {
        pose = { yaw: response.pose.yaw ?? 0, pitch: response.pose.pitch ?? 0, roll: response.pose.roll ?? 0 };
      }
    } else if (response.log) {
      logs = [...logs.slice(-(MAX_LOG_LINES - 1)), response.log.line ?? ""];
    } else if (response.preview) {
      previewSample = response.preview;
    } else if (response.kmsOutputs) {
      kmsOutputs = response.kmsOutputs.outputs ?? [];
    } else if (response.goggles) {
      goggles = response.goggles;
    } else if (response.error) {
      statusMessage = response.error.message ?? "RPC error";
    }
  }

  function propertyMap(properties: eyepiece.rpc.v1.IProperty[] | null | undefined): Record<string, unknown> {
    return Object.fromEntries((properties ?? []).map((property) => [property.name, propertyValue(property.value)]));
  }

  async function syncPreviewSubscription(enabled: boolean, keyframesOnly: boolean) {
    const client = connection.client;
    previewSubscription = `${enabled}:${keyframesOnly}`;
    previewSubscribing = true;
    try {
      if (!client) throw new Error("RPC connection is not ready");
      await client.send(await previewCommand(enabled, keyframesOnly));
      if (connection.client !== client || disposed) return;
      subscribedPreviewOn = enabled;
      subscribedKeyframesOnly = keyframesOnly;
      if (!enabled) previewSample = undefined;
    } catch (error) {
      if (connection.client !== client || disposed) return;
      preview = {
        ...preview,
        on: preview.on === enabled ? subscribedPreviewOn : preview.on,
        keyframesOnly: preview.keyframesOnly === keyframesOnly ? subscribedKeyframesOnly : preview.keyframesOnly,
      };
      statusMessage = errorMessage(error);
    } finally {
      previewSubscribing = false;
    }
  }

  async function discoverKmsOutputs() {
    if (!connection.client || listingKmsOutputs) return;
    listingKmsOutputs = true;
    try {
      await send({ listKmsOutputs: {} });
    } finally {
      listingKmsOutputs = false;
    }
  }

  async function quitEyepiece() {
    try {
      await quitApp();
      quit = true;
      connection.dispose();
    } catch (error) {
      statusMessage = `Cannot quit: ${errorMessage(error)}`;
    }
  }

  async function browseVideoFile() {
    const setting = findSetting(schema, "input");
    if (!setting || pickingFile) return;
    pickingFile = true;
    try {
      const path = await pickVideoFile();
      if (path) await changeSetting(setting, path);
    } catch (error) {
      statusMessage = `Cannot choose a file: ${errorMessage(error)}`;
    } finally {
      pickingFile = false;
    }
  }

  onMount(() => {
    try {
      const prefs = loadPreferences(localStorage);
      ({ preview, pinWindow, pins } = prefs);
      nextPin = nextPinId(pins);
    } catch (error) {
      statusMessage = `Cannot load UI preferences: ${errorMessage(error)}`;
    }
    preferencesReady = true;
    connection.start();
    return () => {
      disposed = true;
      if (undoTimer) clearTimeout(undoTimer);
      paneDrag.cancel();
      connection.dispose();
    };
  });

  // Responses, including state, are applied by the RpcClient event handler.
  async function send(command: RpcCommand): Promise<boolean> {
    if (disposed) return false;
    if (command.setCommandLine) paneDrag.cancel();
    statusMessage = "";
    try {
      await connection.send(command);
      return true;
    } catch (error) {
      if (!disposed) statusMessage = errorMessage(error);
      return false;
    }
  }

  async function submitSettings(next: Record<string, unknown>): Promise<boolean> {
    const edits = settingEdits.begin(next);
    values = settingEdits.snapshot();
    if (!trackingEnabled(values)) pose = null;
    const accepted = await send({
      setProperties: {
        properties: Object.entries(next).map(([name, value]) => ({ name, value: wireValue(value) })),
      },
    });
    values = settingEdits.settle(edits);
    return accepted;
  }

  // Explicit pane edits supersede any queued drag update; the drag throttle itself uses submitSettings.
  function setSettings(next: Record<string, unknown>): Promise<boolean> {
    if ("pane" in next) paneDrag.cancel();
    return submitSettings(next);
  }

  async function applySetting(setting: Setting, raw: unknown): Promise<boolean> {
    const result = coerceSettingValue(setting, raw);
    if ("error" in result) {
      statusMessage = result.error;
      return false;
    }
    return setSettings({ [setting.name]: result.value });
  }

  async function changeSetting(setting: Setting, raw: unknown) {
    await applySetting(setting, raw);
  }

  async function changeAlphaSettings(changes: Record<string, unknown>) {
    const entries = Object.entries(changes);
    if (entries.length === 1) {
      const [name, value] = entries[0];
      const setting = findSetting(schema, name);
      if (!setting) {
        statusMessage = `Missing setting definition: ${name}`;
        return;
      }
      await changeSetting(setting, value);
      return;
    }
    await setSettings(changes);
  }

  async function resetSettings() {
    if (!schema) return;
    const previous = { ...values };
    if (await setSettings(Object.fromEntries(schema.settings.map((setting) => [setting.name, setting.default])))) {
      resetUndo = previous;
      if (undoTimer) clearTimeout(undoTimer);
      undoTimer = setTimeout(() => {
        resetUndo = undefined;
      }, UNDO_TIMEOUT_MS);
    }
  }

  async function applyCommand() {
    if (await send({ setCommandLine: { commandLine: commandEdit } })) commandEdit = commandLine;
  }

  async function undoReset() {
    if (resetUndo && (await setSettings(resetUndo))) resetUndo = undefined;
  }

  async function copyText(text: string) {
    try {
      await navigator.clipboard.writeText(text);
    } catch (error) {
      statusMessage = `Copy failed: ${errorMessage(error)}`;
    }
  }

  function pinResults() {
    if (!schema) return;
    if (!Number.isFinite(pinWindow) || pinWindow < 1 || pinWindow > 600) {
      statusMessage = "Pin window must be between 1 and 600 seconds";
      return;
    }
    const summary = summarizeWindow(samples, pinWindow);
    if (!summary) return;
    const id = nextPin++;
    pins = [
      ...pins,
      {
        id,
        label: `Run ${id}`,
        settings: { ...runningValues },
        command: runningCommandLine,
        metrics: summary.metrics,
        selected: true,
        time: Date.now(),
        seconds: summary.seconds,
        intervals: summary.intervals,
      },
    ];
  }

  const running = $derived(["starting", "running", "stopping"].includes(phase));
  const workerConnected = $derived(workerStatus === "connected");
  const trackingImplied = $derived(Boolean(values.no_video || values.print_pose));
  const trackingOn = $derived(trackingEnabled(values));
  const previewHasAlpha = $derived(
    previewSample?.packedAlpha === true || runningValues.alpha === "source" || Boolean(runningValues.color_key),
  );
  const settingsChanged = $derived(
    running &&
      runningCommandLine !== "" &&
      Object.keys(runningValues).some(
        (key) => key !== "head_tracking" && JSON.stringify(runningValues[key]) !== JSON.stringify(values[key]),
      ),
  );

  $effect(() => {
    const { on, keyframesOnly } = preview;
    if (
      preferencesReady &&
      workerConnected &&
      !previewSubscribing &&
      previewSubscription !== `${on}:${keyframesOnly}`
    ) {
      untrack(() => void syncPreviewSubscription(on, keyframesOnly));
    }
  });

  $effect(() => {
    if (!preferencesReady) return;
    try {
      savePreferences(localStorage, { preview, pinWindow, pins });
    } catch (error) {
      statusMessage = `Cannot save UI preferences: ${errorMessage(error)}`;
    }
  });
</script>

<svelte:head>
  <meta name="description" content="Local control room for Eyepiece PC streaming." />
</svelte:head>

<main class="shell">
  <header class="topbar">
    <a class="brand" href="/"
      ><span class="brand-mark">E</span><span>eyepiece<span class="brand-sub"> / PC CONTROL</span></span></a
    >
    <div class="header-actions">
      <StatusBar {workerStatus} {phase} {connected} {desiredRunning} noConnect={Boolean(values.no_connect)} />
      <button class="secondary quit-button" disabled={!workerConnected} onclick={() => (confirmingQuit = true)}>Quit</button>
    </div>
  </header>

  {#if statusMessage}<div class="alert" role="alert">
      <div class="alert-body">
        <p>{statusMessage}</p>
        {#if statusHelp}<p class="alert-lead">{statusHelp.lead}</p>
          <ul>{#each statusHelp.steps as step}<li>{step}</li>{/each}</ul>{/if}
      </div>
      <button aria-label="Dismiss" onclick={() => (statusMessage = "")}>×</button>
    </div>{/if}
  <ConfirmDialog
    bind:open={confirmingReset}
    title="Reset all settings?"
    message="Every setting returns to its default. Settings that cannot change live apply when the stream restarts. You can undo this for 20 seconds."
    confirmLabel="Reset Settings"
    onConfirm={() => void resetSettings()}
  />
  <ConfirmDialog
    bind:open={confirmingQuit}
    title="Quit eyepiece-pc?"
    message="The stream stops and eyepiece-pc exits; this page stops working until you start it again."
    confirmLabel="Quit"
    onConfirm={() => void quitEyepiece()}
  />
  {#if quit}<div class="notice" role="status">eyepiece-pc has quit. You can close this tab.</div>{/if}
  {#if resetUndo}<div class="notice">
      Settings reset. <button class="secondary" onclick={undoReset}>Undo</button>
    </div>{/if}

  <StreamControls
    {schema}
    {values}
    {running}
    {settingsChanged}
    {workerConnected}
    {kmsOutputs}
    {listingKmsOutputs}
    {pickingFile}
    {send}
    {changeSetting}
    {discoverKmsOutputs}
    {browseVideoFile}
    requestReset={() => (confirmingReset = true)}
  />

  <nav class="tabs" aria-label="Workspace Sections">
    {#each tabs as tab}
      <button
        class:active={activeTab === tab}
        aria-current={activeTab === tab ? "page" : undefined}
        onclick={() => (activeTab = tab)}>{tab}</button
      >
    {/each}
  </nav>

  {#if activeTab === "Overview"}
    <OverviewPanel
      bind:preview
      {previewSample}
      {previewHasAlpha}
      {pose}
      {latest}
      {schema}
      {values}
      {trackingImplied}
      {trackingOn}
      {running}
      {changeSetting}
      {changeAlphaSettings}
      {send}
      changePaneDuringDrag={(value, mode) => paneDrag.update(value, mode)}
      cancelPaneDragUpdate={() => paneDrag.cancel()}
    />
  {:else if activeTab === "Goggles"}
    <GogglesPanel {goggles} set={(mod, key, value) => void send({ setGogglesSetting: { mod, key, value } })} />
  {:else if activeTab === "Statistics"}
    <StatisticsPanel {latest} {samples} {running} {pins} bind:pinWindow {pinResults} />
  {:else if activeTab === "Compare"}
    <ComparePanel bind:pins {schema} {running} {samples} {pinResults} {copyText} />
  {:else if activeTab === "Settings"}
    <SettingsPanel
      {schema}
      {values}
      bind:commandEdit
      {commandLine}
      {applyCommand}
      {copyText}
      resetSettings={async () => {
        confirmingReset = true;
      }}
      {changeSetting}
      {changeAlphaSettings}
      changePaneDuringDrag={(value, mode) => paneDrag.update(value, mode)}
      cancelPaneDragUpdate={() => paneDrag.cancel()}
    />
  {:else}
    <LogPanel {logs} {copyText} clear={() => (logs = [])} />
  {/if}

  <footer><span>Eyepiece Local Control UI</span><span>Loopback Only · RPC Worker {workerStatus}</span></footer>
</main>

<style>
  .header-actions {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 10px;
    min-width: 0;
  }
  .quit-button {
    padding: 6px 12px;
  }
  .shell {
    max-width: 1500px;
    margin: auto;
    padding: 0 34px 32px;
  }
  .topbar {
    min-height: 74px;
    padding: 12px 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    border-bottom: 1px solid #252b2e;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 11px;
    color: inherit;
    text-decoration: none;
    font-size: 17px;
    font-weight: 720;
    letter-spacing: -0.03em;
  }
  .brand-mark {
    display: grid;
    width: 31px;
    height: 31px;
    place-items: center;
    border-radius: 9px;
    background: #9cddc9;
    color: #10211c;
    font-weight: 900;
  }
  .brand-sub {
    color: #71807a;
    font-size: 10px;
    font-weight: 650;
    letter-spacing: 0.12em;
  }
  .alert {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin: 16px 0 0;
    padding: 11px 14px;
    border: 1px solid #714a3f;
    border-radius: 10px;
    background: #281a18;
    color: #ffc2ad;
  }
  .alert-body p {
    margin: 0;
  }
  .alert-body .alert-lead {
    margin-top: 6px;
    color: #f3d6cb;
  }
  .alert-body ul {
    margin: 2px 0 0;
    padding-left: 20px;
    color: #f3d6cb;
  }
  .alert button {
    align-self: flex-start;
    border: 0;
    background: none;
    color: inherit;
    font-size: 19px;
  }
  .notice {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 12px;
    color: #e6c58e;
  }
  .tabs {
    display: flex;
    align-items: center;
    gap: 4px;
    min-height: 48px;
    border-bottom: 1px solid #2c3733;
  }
  .tabs button {
    align-self: stretch;
    padding: 0 16px;
    border: 0;
    border-bottom: 2px solid transparent;
    background: none;
    color: #95a59c;
    font-weight: 600;
  }
  .tabs button:hover {
    color: #e6ece9;
  }
  .tabs button.active {
    border-bottom-color: #9cddc9;
    color: #d8f3e7;
  }
  footer {
    display: flex;
    justify-content: space-between;
    padding: 24px 2px 0;
    color: #5f6b66;
    font-size: 10px;
  }
  @media (max-width: 850px) {
    .shell {
      padding: 0 18px 22px;
    }
  }
  @media (max-width: 550px) {
    .shell {
      padding: 0 12px 18px;
    }
    .topbar {
      min-height: 62px;
      align-items: flex-start;
    }
    .brand-sub {
      display: none;
    }
    .tabs {
      gap: 0;
      overflow-x: auto;
    }
    .tabs button {
      padding: 0 10px;
      font-size: 12px;
      white-space: nowrap;
    }
    footer {
      flex-direction: column;
      gap: 6px;
    }
  }
</style>
