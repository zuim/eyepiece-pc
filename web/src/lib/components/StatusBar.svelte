<script lang="ts">
  import { titleCase } from "../panel-model";

  let {
    workerStatus,
    phase,
    connected,
    desiredRunning,
    noConnect,
  }: { workerStatus: string; phase: string; connected: boolean; desiredRunning: boolean; noConnect: boolean } =
    $props();

  const workerConnected = $derived(workerStatus === "connected");
  const workerPending = $derived(["starting", "running", "connecting"].includes(workerStatus));
  const workerLabel = $derived(
    workerConnected
      ? "Connected"
      : workerPending
        ? "Connecting"
        : workerStatus === "disconnected"
          ? "Disconnected"
          : "Error",
  );
  const linkExpected = $derived(workerConnected && !connected && desiredRunning && !noConnect);
</script>

<div class="top-actions" aria-label="Session Status">
  <span
    class="status"
    class:live={workerConnected}
    class:pending={workerPending}
    class:problem={!workerConnected && !workerPending}
    title={workerStatus}><i></i>Worker: {workerLabel}</span
  >
  <span
    class="status"
    class:live={workerConnected && connected}
    class:pending={linkExpected && phase === "starting"}
    class:problem={linkExpected && phase !== "starting"}
    ><i></i>Link: {!workerConnected ? "Unknown" : connected ? "Connected" : "Not Connected"}</span
  >
  <span
    class="status"
    class:live={workerConnected && phase === "running"}
    class:pending={workerConnected && (phase === "starting" || phase === "stopping")}
    ><i></i>{workerConnected ? titleCase(phase) : "Unknown"}</span
  >
</div>

<style>
  .top-actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: 6px;
    min-width: 0;
  }
  .status {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    padding: 7px 10px;
    border: 1px solid #303a37;
    border-radius: 999px;
    background: #151c1b;
    color: #a8b8b0;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.09em;
  }
  .status i {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #727d78;
  }
  .status.live i {
    background: #7de1b5;
    box-shadow: 0 0 12px #7de1b588;
  }
  .status.pending i {
    background: #e6c58e;
  }
  .status.problem i {
    background: #f48e7b;
  }
</style>
