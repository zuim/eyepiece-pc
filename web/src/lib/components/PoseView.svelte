<script lang="ts">
  import { onMount } from "svelte";
  import { poseScene, type Pose } from "../pose-model";

  let { pose = null }: { pose?: Pose | null } = $props();
  let canvas: HTMLCanvasElement;
  let error = $state("");
  let redraw: () => void = () => {};

  onMount(() => {
    const context = canvas.getContext("2d");
    if (!context) {
      error = "Canvas is unavailable";
      return;
    }
    redraw = () => {
      const { width, height } = canvas.getBoundingClientRect();
      if (!width || !height) return;
      const ratio = window.devicePixelRatio || 1;
      const pixelWidth = Math.round(width * ratio);
      const pixelHeight = Math.round(height * ratio);
      // Assigning canvas dimensions reallocates the bitmap, so only do it on actual resizes.
      if (canvas.width !== pixelWidth || canvas.height !== pixelHeight) {
        canvas.width = pixelWidth;
        canvas.height = pixelHeight;
      }
      context.setTransform(ratio, 0, 0, ratio, 0, 0);
      context.clearRect(0, 0, width, height);
      const scene = poseScene(width, height, pose ?? { yaw: 0, pitch: 0, roll: 0 });
      const line = (points: number[][], color: string, thickness = 1, dash: number[] = []) => {
        if (points.length < 2) return;
        context.beginPath();
        context.moveTo(points[0][0], points[0][1]);
        for (const point of points.slice(1)) context.lineTo(point[0], point[1]);
        context.strokeStyle = color;
        context.lineWidth = thickness;
        context.setLineDash(dash);
        context.stroke();
        context.setLineDash([]);
      };
      line(scene.ring, "#46504d");
      line(scene.tick, "#46504d");
      context.globalAlpha = pose ? 1 : 0.45;
      for (const face of scene.faces) {
        context.beginPath();
        context.moveTo(face.points[0][0], face.points[0][1]);
        for (const point of face.points.slice(1)) context.lineTo(point[0], point[1]);
        context.closePath();
        context.fillStyle = face.front ? "#8dc7b6" : "#293332";
        context.fill();
        context.strokeStyle = "#d5dfda";
        context.lineWidth = 1;
        context.stroke();
      }
      context.globalAlpha = 1;
      line(scene.shadow, "#81908a", 1.5, [3, 3]);
      context.globalAlpha = pose ? 1 : 0.45;
      line(scene.arrow.slice(0, 2), "#9cddc9", 2);
      line([scene.arrow[1], scene.arrow[2]], "#9cddc9", 2);
      line([scene.arrow[1], scene.arrow[3]], "#9cddc9", 2);
      context.globalAlpha = 1;
    };
    const resize = new ResizeObserver(redraw);
    resize.observe(canvas);
    redraw();
    return () => {
      resize.disconnect();
      redraw = () => {};
    };
  });

  // `redraw` reads `pose`, so this re-runs whenever the pose changes.
  $effect(() => redraw());
</script>

<div class="pose-view">
  <canvas
    bind:this={canvas}
    aria-label={pose
      ? `Live Head Pose, Yaw ${pose.yaw.toFixed(1)} degrees, Pitch ${pose.pitch.toFixed(1)} degrees, Roll ${pose.roll.toFixed(1)} degrees`
      : "Head Pose Unavailable"}
  ></canvas>
  {#if error}<span class="unavailable">{error}</span>{/if}
</div>

<style>
  .pose-view {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
    border: 1px solid #293334;
    border-radius: 10px;
    background: #07090b;
  }
  canvas {
    display: block;
    width: 100%;
    height: 100%;
  }
  .unavailable {
    position: absolute;
    inset: auto 1rem 1rem;
    padding: 0.65rem 0.8rem;
    border-radius: 8px;
    background: #111a1ddd;
    color: #ffb6a8;
    text-align: center;
    font-size: 0.82rem;
  }
</style>
