<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { PreviewDecoder, type PreviewSample } from "../preview-decoder";
  import { unpackPreviewAlpha, type AlphaPreviewMode } from "../preview-alpha";

  let {
    sample,
    enabled = false,
    maxFps = 50,
    maxWidth = 640,
    alphaMode = "composite",
  }: {
    sample: PreviewSample | undefined;
    enabled?: boolean;
    maxFps?: number;
    maxWidth?: number;
    alphaMode?: AlphaPreviewMode;
  } = $props();
  let canvas: HTMLCanvasElement | undefined;
  let message = $state("Preview Off");
  let packedAlpha = $state(false);
  let packedCanvas: HTMLCanvasElement | undefined;
  const decoder = new PreviewDecoder(showFrame, (text) => (message = text));

  function showFrame(frame: VideoFrame, hasPackedAlpha: boolean) {
    try {
      if (!canvas) return;
      packedAlpha = hasPackedAlpha;
      const sourceWidth = packedAlpha ? frame.displayWidth / 2 : frame.displayWidth;
      const width = Math.max(1, Math.min(sourceWidth, Math.floor(maxWidth)));
      const height = Math.max(1, Math.round((frame.displayHeight * width) / sourceWidth));
      if (canvas.width !== width || canvas.height !== height) {
        canvas.width = width;
        canvas.height = height;
      }
      const context = canvas.getContext("2d");
      if (!context) throw new Error("Preview canvas is unavailable");
      if (!packedAlpha) {
        context.drawImage(frame, 0, 0, width, height);
        return;
      }
      packedCanvas ??= document.createElement("canvas");
      if (packedCanvas.width !== width * 2 || packedCanvas.height !== height) {
        packedCanvas.width = width * 2;
        packedCanvas.height = height;
      }
      const packedContext = packedCanvas.getContext("2d", { willReadFrequently: true });
      if (!packedContext) throw new Error("Packed-alpha preview canvas is unavailable");
      packedContext.drawImage(frame, 0, 0, packedCanvas.width, packedCanvas.height);
      const pixels = packedContext.getImageData(0, 0, packedCanvas.width, packedCanvas.height);
      const combined = unpackPreviewAlpha(pixels.data, pixels.width, pixels.height, alphaMode);
      context.putImageData(new ImageData(combined, width, height), 0, 0);
    } catch (error) {
      message = error instanceof Error ? `Preview render error: ${error.message}` : "Preview rendering failed";
    } finally {
      frame.close();
    }
  }

  function onEnabledChange(on: boolean) {
    if (!on) {
      decoder.close();
      message = "Preview Off";
      canvas?.getContext("2d")?.clearRect(0, 0, canvas.width, canvas.height);
    } else if (message === "Preview Off") {
      message = "Waiting for preview";
    }
  }

  $effect(() => {
    decoder.maxFps = maxFps;
  });
  $effect(() => {
    const current = sample;
    if (current && enabled) untrack(() => void decoder.decode(current));
  });
  $effect(() => {
    const on = enabled;
    untrack(() => onEnabledChange(on));
  });
  onDestroy(() => decoder.dispose());
</script>

<div
  class="preview-frame"
  class:packed-alpha={packedAlpha && alphaMode === "composite"}
  class:alpha-mask={packedAlpha && alphaMode === "mask"}
>
  <canvas bind:this={canvas} aria-label="Decoded Stream Preview"></canvas>
  {#if message}<div class="notice">{message}</div>{/if}
</div>

<style>
  .preview-frame {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
    border: 1px solid #293334;
    border-radius: 10px;
    background: #07090b;
  }
  .preview-frame.packed-alpha {
    background: repeating-conic-gradient(#182022 0% 25%, #293334 0% 50%) 0 0 / 24px 24px;
  }
  .preview-frame.alpha-mask {
    background: #07090b;
  }
  canvas {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: contain;
  }
  .notice {
    position: absolute;
    inset: auto 1rem 1rem;
    padding: 0.65rem 0.8rem;
    border-radius: 8px;
    background: #111a1ddd;
    color: #d7e3df;
    text-align: center;
    font-size: 0.82rem;
  }
</style>
