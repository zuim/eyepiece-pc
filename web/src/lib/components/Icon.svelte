<script lang="ts">
  let {
    name,
  }: {
    name: "play" | "pause" | "stop" | "recenter" | "keyframe" | "pin" | "copy" | "trash" | "restart";
  } = $props();
</script>

<svg
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {#if name === "play"}<path d="M8 5v14l11-7z" fill="currentColor" stroke="none" />
  {:else if name === "pause"}<path d="M7 5h4v14H7zM14 5h4v14h-4z" fill="currentColor" stroke="none" />
  {:else if name === "stop"}<rect x="6" y="6" width="12" height="12" rx="1" fill="currentColor" stroke="none" />
  {:else if name === "recenter"}<circle cx="12" cy="12" r="7" /><circle cx="12" cy="12" r="2" /><path
      d="M12 2v3M12 19v3M2 12h3M19 12h3"
    />
  {:else if name === "keyframe"}<circle cx="8" cy="12" r="4" /><path d="M12 12h9m-3 0v3m-3-3v2" />
  {:else if name === "pin"}<path d="M9 3h6l-1 6 3 3v2H7v-2l3-3zM12 14v7" />
  {:else if name === "copy"}<rect x="8" y="8" width="11" height="11" rx="2" /><path
      d="M16 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h3"
    />
  {:else if name === "trash"}<path d="M4 7h16M9 7V4h6v3m-9 0 1 13h10l1-13M10 11v5m4-5v5" />
  {:else}<path d="M20 7v5h-5M4 17v-5h5" /><path d="M5.5 9A7 7 0 0 1 18 6l2 1M4 17l2 1a7 7 0 0 0 12.5-3" />{/if}
</svg>

<style>
  svg {
    display: inline-block;
    flex: none;
    width: 16px;
    height: 16px;
    vertical-align: -3px;
  }
</style>
