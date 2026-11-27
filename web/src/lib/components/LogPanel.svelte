<script lang="ts">
  import Icon from "./Icon.svelte";

  let { logs, copyText, clear }: { logs: string[]; copyText: (text: string) => Promise<void>; clear: () => void } =
    $props();
</script>

<section class="panel page-panel">
  <div class="panel-heading">
    <div>
      <span class="eyebrow">DIAGNOSTICS</span>
      <h1>Session Log</h1>
    </div>
    <div class="compare-actions">
      <button class="secondary" onclick={() => copyText(logs.join("\n"))}><Icon name="copy" /> Copy Log</button>
      <button class="secondary" onclick={clear}><Icon name="trash" /> Clear</button>
    </div>
  </div>
  <pre>{logs.length ? logs.join("\n") : "No log messages yet."}</pre>
</section>

<style>
  pre {
    min-height: 55vh;
    padding: 16px;
    border: 1px solid #303b37;
    border-radius: 10px;
    background: #0b100f;
  }
</style>
