<script lang="ts">
  import {
    DEFAULT_PANE,
    PANEL_HEIGHT,
    PANEL_WIDTH,
    dragPane,
    formatPane,
    parsePane,
    validPane,
    type PaneDragMode,
    type PaneGeometry,
  } from "../pane-geometry";

  type DragState = {
    pointerId: number;
    startX: number;
    startY: number;
    width: number;
    height: number;
    pane: PaneGeometry;
    mode: PaneDragMode;
    original: string;
    requested: Set<string>;
  };

  let {
    value,
    help,
    onChange,
    onDragChange,
    onDragConflict,
    showPanel = true,
  }: {
    value: string | null;
    help: string;
    onChange: (value: string | null) => void;
    onDragChange: (value: string, mode: "move" | "resize") => void;
    onDragConflict: () => void;
    showPanel?: boolean;
  } = $props();

  let panel = $state.raw<HTMLDivElement>();
  let draft = $state.raw<PaneGeometry | null>(null);
  let error = $state("");
  let drag = $state.raw<DragState | null>(null);

  const handles: { direction: Exclude<PaneDragMode, "move">; label: string; x: number; y: number }[] = [
    { direction: "nw", label: "top left", x: 0, y: 0 },
    { direction: "n", label: "top", x: 0.5, y: 0 },
    { direction: "ne", label: "top right", x: 1, y: 0 },
    { direction: "w", label: "left", x: 0, y: 0.5 },
    { direction: "e", label: "right", x: 1, y: 0.5 },
    { direction: "sw", label: "bottom left", x: 0, y: 1 },
    { direction: "s", label: "bottom", x: 0.5, y: 1 },
    { direction: "se", label: "bottom right", x: 1, y: 1 },
  ];

  const pane = $derived(draft ?? (value === null ? null : parsePane(value)));

  // The pane was changed by someone else mid-drag.
  $effect(() => {
    if (drag && value !== drag.original && (value === null || !drag.requested.has(value))) {
      onDragConflict();
      drag = null;
      draft = null;
      error = "Pane changed elsewhere during drag; try again.";
    }
  });

  function startDrag(event: PointerEvent, mode: PaneDragMode) {
    if (!pane || value === null || event.button !== 0) return;
    const bounds = panel?.getBoundingClientRect();
    if (!bounds) return;
    if (!bounds.width || !bounds.height) return;
    event.preventDefault();
    event.stopPropagation();
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
    drag = {
      pointerId: event.pointerId,
      startX: event.clientX,
      startY: event.clientY,
      width: bounds.width,
      height: bounds.height,
      pane,
      mode,
      original: value,
      requested: new Set(),
    };
    draft = pane;
    error = "";
  }

  function moveDrag(event: PointerEvent) {
    if (!drag || drag.pointerId !== event.pointerId) return;
    draft = dragPane(
      drag.pane,
      drag.mode,
      ((event.clientX - drag.startX) * PANEL_WIDTH) / drag.width,
      ((event.clientY - drag.startY) * PANEL_HEIGHT) / drag.height,
    );
    const next = formatPane(draft);
    if (next !== drag.original || drag.requested.size) {
      drag.requested.add(next);
      onDragChange(next, drag.mode === "move" ? "move" : "resize");
    }
  }

  function endDrag(event: PointerEvent, cancelled = false) {
    if (!drag || drag.pointerId !== event.pointerId) return;
    const next = draft;
    const original = drag.original;
    const requested = drag.requested;
    const mode = drag.mode === "move" ? "move" : "resize";
    drag = null;
    draft = null;
    if (value !== original && (value === null || !requested.has(value))) {
      onDragConflict();
      error = "Pane changed elsewhere during drag; try again.";
    } else if (cancelled) {
      if (requested.size) onDragChange(original, mode);
    } else if (next && (formatPane(next) !== original || requested.size)) {
      onDragChange(formatPane(next), mode);
    }
  }

  function editField(field: keyof PaneGeometry, event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    if (!pane) return;
    const next = { ...pane, [field]: input.valueAsNumber };
    if (!validPane(next)) {
      error = "Enter nonzero even dimensions that fit within the panel.";
      input.value = String(pane[field]);
      return;
    }
    error = "";
    if (formatPane(next) !== value) onChange(formatPane(next));
  }
</script>

<div class="pane-editor">
  <label class="enable"
    ><span class="setting-label">Enable Picture-in-Picture<span class="setting-help">{help}</span></span><input
      aria-label="Enable Picture-in-Picture"
      type="checkbox"
      checked={value !== null}
      onchange={(event) => {
        error = "";
        onChange(event.currentTarget.checked ? DEFAULT_PANE : null);
      }}
    /></label
  >
  {#if value !== null}
    {#if pane}
      {#if showPanel}
        <div class="stage" bind:this={panel} aria-label="1920 by 1080 panel">
          <button
            type="button"
            class="pane"
            style:left={`${(pane.x / PANEL_WIDTH) * 100}%`}
            style:top={`${(pane.y / PANEL_HEIGHT) * 100}%`}
            style:width={`${(pane.width / PANEL_WIDTH) * 100}%`}
            style:height={`${(pane.height / PANEL_HEIGHT) * 100}%`}
            aria-label="Move picture-in-picture pane (use position fields for keyboard input)"
            onpointerdown={(event) => startDrag(event, "move")}
            onpointermove={moveDrag}
            onpointerup={(event) => endDrag(event)}
            onpointercancel={(event) => endDrag(event, true)}
          >
            <span>Pane</span>
          </button>
          {#each handles as handle}
            <button
              type="button"
              class="resize {handle.direction}"
              style:left={`clamp(7px, ${((pane.x + pane.width * handle.x) / PANEL_WIDTH) * 100}%, calc(100% - 7px))`}
              style:top={`clamp(7px, ${((pane.y + pane.height * handle.y) / PANEL_HEIGHT) * 100}%, calc(100% - 7px))`}
              aria-label={`Resize picture-in-picture pane from ${handle.label} (use size and position fields for keyboard input)`}
              onpointerdown={(event) => startDrag(event, handle.direction)}
              onpointermove={moveDrag}
              onpointerup={(event) => endDrag(event)}
              onpointercancel={(event) => endDrag(event, true)}
            ></button>
          {/each}
        </div>
      {/if}
      <div class="fields">
        {#each [{ key: "width", label: "Width", max: PANEL_WIDTH, step: 2 }, { key: "height", label: "Height", max: PANEL_HEIGHT, step: 2 }, { key: "x", label: "X", max: PANEL_WIDTH, step: 1 }, { key: "y", label: "Y", max: PANEL_HEIGHT, step: 1 }] as field}
          <label
            >{field.label} (px)<input
              type="number"
              min={field.step === 2 ? 2 : 0}
              max={field.max}
              step={field.step}
              value={pane[field.key as keyof PaneGeometry]}
              onchange={(event) => editField(field.key as keyof PaneGeometry, event)}
            /></label
          >
        {/each}
      </div>
      <p>Dimensions must be even and fit the 1920 × 1080 panel. Live changes require a compatible bridge.</p>
    {:else}
      <p class="error">
        Invalid pane geometry: {value}. Disable and re-enable picture in picture to reset it, or edit the CLI command.
      </p>
    {/if}
    {#if error}<p class="error" role="alert">{error}</p>{/if}
  {/if}
</div>

<style>
  .pane-editor {
    display: grid;
    gap: 10px;
    min-width: 0;
  }
  .enable {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    width: 100%;
    color: var(--label-color);
  }
  .setting-label {
    display: grid;
    gap: 4px;
    font-size: 12px;
  }
  .setting-help {
    color: #91a39a;
    font-size: 11px;
    line-height: 1.4;
  }
  p {
    margin: 0;
    color: #91a39a;
    font-size: 11px;
  }
  .stage {
    position: relative;
    width: 100%;
    aspect-ratio: 16 / 9;
    overflow: hidden;
    border: 1px solid #465752;
    border-radius: 8px;
    background: #0b1110;
  }
  .pane {
    position: absolute;
    display: grid;
    place-items: center;
    padding: 0;
    border: 2px solid #9cddc9;
    background: #47796580;
    color: #e6ece9;
    cursor: move;
    touch-action: none;
  }
  .pane span {
    pointer-events: none;
    font-size: 11px;
  }
  .resize {
    position: absolute;
    width: 14px;
    height: 14px;
    padding: 0;
    transform: translate(-50%, -50%);
    border: 2px solid #0b1110;
    border-radius: 4px;
    background: #9cddc9;
    touch-action: none;
  }
  .n,
  .s {
    cursor: ns-resize;
  }
  .e,
  .w {
    cursor: ew-resize;
  }
  .nw,
  .se {
    cursor: nwse-resize;
  }
  .ne,
  .sw {
    cursor: nesw-resize;
  }
  .fields {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 8px;
  }
  .fields label {
    display: grid;
    gap: 5px;
    color: #aab8b1;
    font-size: 11px;
  }
  .fields input {
    width: 100%;
    padding: 8px;
  }
  .error {
    color: #ffc2ad;
  }
  @media (max-width: 480px) {
    .fields {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
</style>
