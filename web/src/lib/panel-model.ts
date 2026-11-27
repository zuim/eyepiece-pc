export type SettingType =
  | "boolean"
  | "choice"
  | "string"
  | "strings"
  | "optional_string"
  | "integer"
  | "optional_integer"
  | "number"
  | "optional_number";
export type Setting = {
  name: string;
  group: string;
  description: string;
  type: SettingType;
  default: unknown;
  min?: number;
  max?: number;
  choices?: string[];
  visible?: boolean;
};
export type Schema = { version: number; settings: Setting[] };
export type Metrics = Record<string, number>;
export type Pin = {
  id: number;
  label: string;
  settings: Record<string, unknown>;
  command: string;
  metrics: Metrics;
  selected: boolean;
  time: number;
  seconds: number;
  intervals: number;
};

export const bitrates = ["10M", "15M", "20M", "30M", "40M", "50M"];
export const encoderLabels: Record<string, string> = {
  auto: "Automatic",
  videotoolbox: "Apple VideoToolbox",
  nvenc: "NVIDIA NVENC",
  vaapi: "VA-API",
  amf: "AMD AMF",
  qsv: "Intel Quick Sync",
  software: "Software",
};
export const choiceLabels: Record<string, Record<string, string>> = {
  source: {
    kms: "Linux KMS",
    pipewire: "PipeWire",
    x11: "X11",
    dxgi: "Windows DXGI",
    screencapturekit: "macOS ScreenCaptureKit",
    test: "Test Pattern",
    file: "Video File",
  },
  dxgi_cursor: {
    auto: "Automatic (when missing)",
    on: "Always",
    off: "Never",
  },
  sync: {
    vblank: "Vertical Blank",
    poll: "Polling",
    timer: "Timer",
  },
  codec: {
    h264: "H.264 (AVC)",
    hevc: "H.265 (HEVC)",
  },
  alpha: {
    opaque: "Opaque",
    source: "Source Alpha",
  },
  scale_mode: {
    hq: "High Quality",
    fast: "Fast",
    default: "Default",
  },
  ffmpeg_log: {
    quiet: "Quiet",
    error: "Error",
    warning: "Warning",
    info: "Info",
    verbose: "Verbose",
    debug: "Debug",
  },
};
export const settingGroups = [
  ["Connection", ["serial", "host", "no_connect", "reconnect"]],
  ["Bridge", ["no_launch", "bridge_args", "stop_bridge"]],
  [
    "Capture",
    [
      "source",
      "fps",
      "cursor",
      "sync",
      "output",
      "kms_device",
      "kms_plane",
      "kms_simulate",
      "display",
      "region",
      "input",
      "lavfi",
      "restore_token",
      "display_id",
      "dxgi",
      "dxgi_cursor",
    ],
  ],
  [
    "Video",
    [
      "codec",
      "bitrate",
      "stereo",
      "pane",
      "pane_opacity",
      "gui_opacity",
      "alpha",
      "color_key",
      "key_similarity",
      "key_blend",
      "no_letterbox",
      "cpu_scale",
      "scale_mode",
      "vf",
    ],
  ],
  [
    "Encoder",
    ["encoder", "vaapi_device", "gop", "slices", "vbv_frames", "preset", "tune", "profile", "intra_refresh", "enc_opt"],
  ],
  ["Head Tracking", ["head_tracking", "no_video", "opentrack", "print_pose"]],
  ["Diagnostics", ["preview", "dump", "frames", "duration", "stats_interval", "ffmpeg_log"]],
] as const;
export const metricRows = [
  ["sent_fps", "Sent FPS", "fps"],
  ["shown_fps", "Shown FPS", "fps"],
  ["captured_fps", "Captured FPS", "fps"],
  ["mbps", "Bandwidth", "Mbit/s"],
  ["lat_p50_ms", "Capture → Shown P50", "ms"],
  ["lat_p95_ms", "Capture → Shown P95", "ms"],
  ["cap_send_p50_ms", "Capture → Sent P50", "ms"],
  ["cap_send_p95_ms", "Capture → Sent P95", "ms"],
  ["cap_enc_p50_ms", "Capture → Encoder Input P50", "ms"],
  ["cap_enc_p95_ms", "Capture → Encoder Input P95", "ms"],
  ["enc_p50_ms", "Encode P50", "ms"],
  ["enc_p95_ms", "Encode P95", "ms"],
  ["dec_p50_ms", "Goggles Decode P50", "ms"],
  ["dec_p95_ms", "Goggles Decode P95", "ms"],
  ["recv_p50_ms", "Capture → Received P50", "ms"],
  ["recv_p95_ms", "Capture → Received P95", "ms"],
  ["cvt_p50_ms", "Goggles Convert P50", "ms"],
  ["cvt_p95_ms", "Goggles Convert P95", "ms"],
  ["rtt_ms", "Round-Trip", "ms"],
  ["dropped", "Dropped Frames", ""],
  ["skipped_per_s", "Skipped /s", ""],
  ["idr_per_s", "IDR /s", ""],
  ["q_max", "Queue Maximum", ""],
  ["pose_hz", "Pose Rate", "Hz"],
] as const;
export const latencyRows = [
  ["Capture → Encoder Input", "cap_enc_p50_ms", "cap_enc_p95_ms"],
  ["Encode", "enc_p50_ms", "enc_p95_ms"],
  ["Capture → Sent", "cap_send_p50_ms", "cap_send_p95_ms"],
  ["Capture → Received (Goggles)", "recv_p50_ms", "recv_p95_ms"],
  ["Goggles Decode", "dec_p50_ms", "dec_p95_ms"],
  ["Goggles Convert", "cvt_p50_ms", "cvt_p95_ms"],
  ["Capture → Shown", "lat_p50_ms", "lat_p95_ms"],
] as const;
export const throughputRows = [
  ["Sent", "sent_fps", "fps"],
  ["Shown", "shown_fps", "fps"],
  ["Bitrate", "mbps", "Mbit/s"],
  ["Captured", "captured_fps", "fps"],
  ["Skipped", "skipped", ""],
  ["IDR", "idr", ""],
  ["Dropped (Goggles)", "dropped", ""],
  ["Queue Maximum", "q_max", ""],
  ["Round-Trip", "rtt_ms", "ms"],
  ["Pose Rate", "pose_hz", "Hz"],
] as const;
export const latencySeries = [
  { key: "enc_p50_ms", label: "Encode P50", color: "#9696a0" },
  { key: "cap_send_p50_ms", label: "Capture → Sent P50", color: "#5aa0f0" },
  { key: "recv_p50_ms", label: "Capture → Received P50", color: "#c88cf0" },
  { key: "lat_p50_ms", label: "Capture → Shown P50", color: "#50c878" },
  { key: "lat_p95_ms", label: "Capture → Shown P95", color: "#e6aa46" },
];
export const rateSeries = [
  { key: "sent_fps", label: "Sent FPS", color: "#50c878" },
  { key: "shown_fps", label: "Shown FPS", color: "#5aa0f0" },
  { key: "captured_fps", label: "Captured FPS", color: "#c88cf0" },
  { key: "mbps", label: "Bitrate", color: "#e6aa46", secondary: true },
];

export function formatMetric(value: number | undefined, unit: string): string {
  if (value === undefined || !Number.isFinite(value)) return "—";
  return `${value.toFixed(unit === "fps" || unit === "Hz" ? 1 : 2)}${unit ? ` ${unit}` : ""}`;
}

export function currentSetting(name: string, current: Record<string, unknown>): string {
  const value = current[name];
  return value === null || value === undefined ? "" : String(value);
}

export function colorPickerValue(value: string): string {
  const hex = value.replace(/^#/, "");
  return /^[\da-f]{6}$/i.test(hex) ? `#${hex}` : "#00ff00";
}

export function settingSliderStep(setting: Setting): number | undefined {
  if (
    setting.name === "fps" ||
    setting.min === undefined ||
    setting.max === undefined ||
    setting.max <= setting.min ||
    setting.max - setting.min > 1000 ||
    (!setting.type.includes("integer") && !setting.type.includes("number"))
  )
    return undefined;
  if (setting.name === "key_similarity" || setting.name === "key_blend") return 0.01;
  return setting.type.includes("integer") ? 1 : 0.1;
}

export function choiceLabel(setting: string, choice: string): string {
  return choiceLabels[setting]?.[choice] ?? titleCase(choice.replace(/[-_]/g, " "));
}

export function selectedChoice(setting: Setting, current: Record<string, unknown>): string {
  const value = currentSetting(setting.name, current);
  return setting.choices?.includes(value) ? value : "";
}

export function selectedBitrate(current: Record<string, unknown>): string {
  const value = currentSetting("bitrate", current);
  return bitrates.includes(value) ? value : "";
}

export function encoderLabel(encoder: string): string {
  return encoderLabels[encoder] ?? titleCase(encoder.replace(/[-_]/g, " "));
}

export function titleCase(label: string): string {
  return label.replace(/\b[a-zA-Z][a-zA-Z0-9]*\b/g, (word, index: number) => {
    if (word === "opentrack" || word === "macOS" || word === "ffplay") return word;
    if (index > 0 && ["and", "in", "of", "on", "per", "to"].includes(word)) return word;
    return word[0].toUpperCase() + word.slice(1);
  });
}

export function trackingEnabled(current: Record<string, unknown>): boolean {
  return Boolean(current.head_tracking || current.no_video || current.print_pose);
}

export function describeSettings(current: Record<string, unknown>, schema?: Schema): string {
  if (!schema) return "";
  return schema.settings
    .map((setting) => {
      const value = current[setting.name];
      const text =
        value === null || value === undefined ? "Not Set" : Array.isArray(value) ? value.join(", ") : String(value);
      return `${titleCase(setting.description)}: ${text}`;
    })
    .join("\n");
}

export type SettingGroup = { name: string; settings: Setting[] };

/** Groups visible settings; settings missing from `settingGroups` are collected in "Other". */
export function groupSettings(schema: Schema | undefined, hidden: readonly string[] = []): SettingGroup[] {
  if (!schema) return [];
  const byName = new Map(schema.settings.map((setting) => [setting.name, setting]));
  const known = new Set<string>();
  const groups: SettingGroup[] = settingGroups.map(([name, names]) => {
    names.forEach((setting) => known.add(setting));
    return {
      name,
      settings: names.map((setting) => byName.get(setting)).filter((setting): setting is Setting => !!setting),
    };
  });
  const other = schema.settings.filter((setting) => !known.has(setting.name));
  if (other.length) groups.push({ name: "Other", settings: other });
  return groups
    .map((group) => ({
      name: group.name,
      settings: group.settings.filter((setting) => setting.visible !== false && !hidden.includes(setting.name)),
    }))
    .filter((group) => group.settings.length > 0);
}

export type SettingValue = { value: unknown } | { error: string };

/** Converts raw form input to the value type of `setting`, validating numeric constraints. */
export function coerceSettingValue(setting: Setting, raw: unknown): SettingValue {
  let value = raw;
  const numeric =
    setting.type === "integer" ||
    setting.type === "optional_integer" ||
    setting.type === "number" ||
    setting.type === "optional_number";
  if (numeric) {
    const optional = setting.type.startsWith("optional");
    value = raw === "" && optional ? null : typeof raw === "string" && raw.trim() === "" ? NaN : Number(raw);
  } else if (setting.type === "optional_string") {
    value = raw === "" ? null : raw;
  } else if (setting.type === "strings" && typeof raw === "string") {
    value = raw
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
  }
  if (typeof value === "number") {
    const integer = setting.type.includes("integer");
    if (
      !Number.isFinite(value) ||
      (integer && !Number.isInteger(value)) ||
      (setting.min !== undefined && value < setting.min) ||
      (setting.max !== undefined && value > setting.max)
    ) {
      return { error: `${setting.name} is outside its allowed range` };
    }
  }
  return { value };
}

export function findSetting(schema: Schema | undefined, name: string): Setting | undefined {
  return schema?.settings.find((setting) => setting.name === name);
}

const HIGHER_IS_BETTER = new Set(["sent_fps", "shown_fps", "captured_fps", "pose_hz", "mbps"]);

/** Id of the pin with the best value for `metric`, or undefined when fewer than two pins report it. */
export function bestPinId(metric: string, pins: readonly Pin[]): number | undefined {
  const reported = pins.filter((pin) => Number.isFinite(pin.metrics[metric]));
  if (reported.length < 2) return undefined;
  const values = reported.map((pin) => pin.metrics[metric]);
  const best = HIGHER_IS_BETTER.has(metric) ? Math.max(...values) : Math.min(...values);
  return reported.find((pin) => pin.metrics[metric] === best)?.id;
}

export function pinMetricText(metric: string, pin: Pin): string {
  return formatMetric(pin.metrics[metric], metricRows.find((row) => row[0] === metric)?.[2] ?? "");
}

export function comparisonMarkdown(pins: readonly Pin[]): string {
  const cell = (text: string) => text.replaceAll("|", "\\|");
  return [
    `| Metric | ${pins.map((pin) => cell(pin.label)).join(" | ")} |`,
    `| --- | ${pins.map(() => "---").join(" | ")} |`,
    ...metricRows.map(([key, label]) => `| ${label} | ${pins.map((pin) => pinMetricText(key, pin)).join(" | ")} |`),
  ].join("\n");
}
