export const PANEL_WIDTH = 1920;
export const PANEL_HEIGHT = 1080;
export const DEFAULT_PANE = "960x540+480+270";

export type PaneGeometry = { width: number; height: number; x: number; y: number };
export type PaneDragMode = "move" | "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw";

export function parsePane(value: string): PaneGeometry | null {
  const match = /^(\d+)x(\d+)\+(\d+)\+(\d+)$/.exec(value);
  if (!match) return null;
  const [, width, height, x, y] = match.map(Number);
  const pane = { width, height, x, y };
  return validPane(pane) ? pane : null;
}

export function validPane({ width, height, x, y }: PaneGeometry): boolean {
  return (
    [width, height, x, y].every(Number.isSafeInteger) &&
    width > 0 &&
    height > 0 &&
    width % 2 === 0 &&
    height % 2 === 0 &&
    x >= 0 &&
    y >= 0 &&
    x + width <= PANEL_WIDTH &&
    y + height <= PANEL_HEIGHT
  );
}

export function formatPane({ width, height, x, y }: PaneGeometry): string {
  return `${width}x${height}+${x}+${y}`;
}

export function dragPane(pane: PaneGeometry, mode: PaneDragMode, dx: number, dy: number): PaneGeometry {
  const clamp = (value: number, min: number, max: number) => Math.min(max, Math.max(min, value));
  if (mode === "move") {
    return {
      ...pane,
      x: clamp(Math.round(pane.x + dx), 0, PANEL_WIDTH - pane.width),
      y: clamp(Math.round(pane.y + dy), 0, PANEL_HEIGHT - pane.height),
    };
  }
  const left = mode.includes("w");
  const top = mode.includes("n");
  const horizontal = left || mode.includes("e");
  const vertical = top || mode.includes("s");
  const maxWidth = left ? pane.x + pane.width : PANEL_WIDTH - pane.x;
  const maxHeight = top ? pane.y + pane.height : PANEL_HEIGHT - pane.y;
  const width = horizontal
    ? clamp(Math.round((pane.width + (left ? -dx : dx)) / 2) * 2, 2, maxWidth - (maxWidth % 2))
    : pane.width;
  const height = vertical
    ? clamp(Math.round((pane.height + (top ? -dy : dy)) / 2) * 2, 2, maxHeight - (maxHeight % 2))
    : pane.height;
  return {
    width,
    height,
    x: left ? pane.x + pane.width - width : pane.x,
    y: top ? pane.y + pane.height - height : pane.y,
  };
}
