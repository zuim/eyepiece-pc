import type { AlphaPreviewMode } from "./preview-alpha";
import type { Pin } from "./panel-model";

export const PREFERENCES_KEY = "eyepiece-web-ui";
export const PREVIEW_WIDTHS = [320, 480, 640, 960, 1280, 1920] as const;
export const PREVIEW_FPS = [10, 15, 20, 30, 50, 100] as const;

export type PreviewSettings = {
  on: boolean;
  maxFps: number;
  width: number;
  keyframesOnly: boolean;
  alphaMode: AlphaPreviewMode;
};

export type UiPreferences = {
  preview: PreviewSettings;
  pinWindow: number;
  pins: Pin[];
};

export const defaultPreferences: UiPreferences = {
  preview: { on: false, maxFps: 50, width: 640, keyframesOnly: false, alphaMode: "composite" },
  pinWindow: 10,
  pins: [],
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isPin(value: unknown): value is Pin {
  return (
    isRecord(value) &&
    typeof value.id === "number" &&
    typeof value.label === "string" &&
    typeof value.command === "string" &&
    isRecord(value.settings) &&
    isRecord(value.metrics) &&
    typeof value.selected === "boolean" &&
    typeof value.time === "number" &&
    typeof value.seconds === "number" &&
    typeof value.intervals === "number"
  );
}

/** Parses stored preferences, keeping defaults for missing or invalid fields. Throws on malformed JSON or pins. */
export function parsePreferences(stored: string): UiPreferences {
  const data: unknown = JSON.parse(stored);
  if (!isRecord(data)) throw new Error("invalid preferences");
  const next: UiPreferences = { ...defaultPreferences, preview: { ...defaultPreferences.preview } };
  const preview = isRecord(data.preview) ? data.preview : {};
  if (typeof preview.on === "boolean") next.preview.on = preview.on;
  if (typeof preview.maxFps === "number" && (PREVIEW_FPS as readonly number[]).includes(preview.maxFps)) {
    next.preview.maxFps = preview.maxFps;
  }
  if (typeof preview.width === "number" && (PREVIEW_WIDTHS as readonly number[]).includes(preview.width)) {
    next.preview.width = preview.width;
  }
  if (typeof preview.keyframesOnly === "boolean") next.preview.keyframesOnly = preview.keyframesOnly;
  if (preview.alphaMode === "composite" || preview.alphaMode === "mask") next.preview.alphaMode = preview.alphaMode;
  if (typeof data.pinWindow === "number" && data.pinWindow >= 1 && data.pinWindow <= 600) {
    next.pinWindow = data.pinWindow;
  }
  if (data.pins !== undefined) {
    if (!Array.isArray(data.pins) || !data.pins.every(isPin)) throw new Error("invalid saved pins");
    next.pins = data.pins;
  }
  return next;
}

export function loadPreferences(storage: Pick<Storage, "getItem">): UiPreferences {
  const stored = storage.getItem(PREFERENCES_KEY);
  return stored ? parsePreferences(stored) : parsePreferences("{}");
}

export function savePreferences(storage: Pick<Storage, "setItem">, preferences: UiPreferences): void {
  storage.setItem(PREFERENCES_KEY, JSON.stringify(preferences));
}

export function nextPinId(pins: readonly Pin[]): number {
  return Math.max(0, ...pins.map((pin) => pin.id)) + 1;
}
