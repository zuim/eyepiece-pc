import { describe, expect, it } from "vitest";
import {
  bestPinId,
  choiceLabel,
  comparisonMarkdown,
  coerceSettingValue,
  describeSettings,
  groupSettings,
  colorPickerValue,
  selectedChoice,
  settingSliderStep,
  type Setting,
} from "./panel-model";

function setting(overrides: Partial<Setting> = {}): Setting {
  return {
    name: "test",
    group: "Video",
    description: "Test setting",
    type: "number",
    default: 0,
    min: 0,
    max: 10,
    ...overrides,
  };
}

describe("panel model", () => {
  it("normalizes valid color keys and falls back for invalid values", () => {
    expect(colorPickerValue("#a1B2c3")).toBe("#a1B2c3");
    expect(colorPickerValue("abcdef")).toBe("#abcdef");
    expect(colorPickerValue("#abc")).toBe("#00ff00");
    expect(colorPickerValue("gg0000")).toBe("#00ff00");
  });

  it("keeps choice labels stable and hides unsupported current choices", () => {
    const transport = setting({ name: "transport", type: "choice", choices: ["auto", "adb"] });

    expect(choiceLabel("transport", "adb")).toBe("Android Debug Bridge (ADB)");
    expect(selectedChoice(transport, { transport: "adb" })).toBe("adb");
    expect(selectedChoice(transport, { transport: "usb-bulk" })).toBe("");
  });

  it("offers sliders only for bounded numeric settings with useful step sizes", () => {
    expect(settingSliderStep(setting())).toBe(0.1);
    expect(settingSliderStep(setting({ type: "integer" }))).toBe(1);
    expect(settingSliderStep(setting({ name: "key_similarity", max: 1 }))).toBe(0.01);
    expect(settingSliderStep(setting({ name: "fps" }))).toBeUndefined();
    expect(settingSliderStep(setting({ type: "string" }))).toBeUndefined();
    expect(settingSliderStep(setting({ max: 0 }))).toBeUndefined();
    expect(settingSliderStep(setting({ max: 2000 }))).toBeUndefined();
  });
});

describe("coerceSettingValue", () => {
  it("validates numeric ranges, integrality and empty input", () => {
    const integer = setting({ type: "integer" });
    expect(coerceSettingValue(integer, "5")).toEqual({ value: 5 });
    expect(coerceSettingValue(integer, "1.5")).toHaveProperty("error");
    expect(coerceSettingValue(integer, "")).toHaveProperty("error");
    expect(coerceSettingValue(integer, "11")).toHaveProperty("error");
    expect(coerceSettingValue(setting({ type: "optional_integer" }), "")).toEqual({ value: null });
    expect(coerceSettingValue(setting({ type: "number" }), "1.5")).toEqual({ value: 1.5 });
  });

  it("splits string lists and clears optional strings", () => {
    expect(coerceSettingValue(setting({ type: "strings" }), " a \n\nb")).toEqual({ value: ["a", "b"] });
    expect(coerceSettingValue(setting({ type: "optional_string" }), "")).toEqual({ value: null });
  });
});

describe("groupSettings", () => {
  it("drops hidden settings and collects unknown ones under Other", () => {
    const schema = {
      version: 1,
      settings: [
        setting({ name: "fps" }),
        setting({ name: "codec", visible: false }),
        setting({ name: "mystery" }),
        setting({ name: "color_key" }),
      ],
    };
    const groups = groupSettings(schema, ["color_key"]);
    expect(groups.map((group) => [group.name, group.settings.map((item) => item.name)])).toEqual([
      ["Capture", ["fps"]],
      ["Other", ["mystery"]],
    ]);
  });
});

describe("describeSettings", () => {
  it("renders unset and list values", () => {
    const schema = {
      version: 1,
      settings: [setting({ name: "a", description: "first" }), setting({ name: "b", description: "second" })],
    };
    expect(describeSettings({ b: ["x", "y"] }, schema)).toBe("First: Not Set\nSecond: x, y");
  });
});

describe("run comparison", () => {
  const pin = (id: number, metrics: Record<string, number>) => ({
    id,
    label: `Run ${id}`,
    settings: {},
    command: "",
    metrics,
    selected: true,
    time: 0,
    seconds: 1,
    intervals: 1,
  });

  it("picks the highest throughput and the lowest latency", () => {
    const pins = [pin(1, { sent_fps: 60, enc_p50_ms: 5 }), pin(2, { sent_fps: 90, enc_p50_ms: 8 })];
    expect(bestPinId("sent_fps", pins)).toBe(2);
    expect(bestPinId("enc_p50_ms", pins)).toBe(1);
  });

  it("ignores metrics reported by fewer than two runs", () => {
    expect(bestPinId("sent_fps", [pin(1, { sent_fps: 60 }), pin(2, {})])).toBeUndefined();
  });

  it("escapes pipes in the markdown table header", () => {
    const [header] = comparisonMarkdown([{ ...pin(1, {}), label: "a|b" }]).split("\n");
    expect(header).toBe("| Metric | a\\|b |");
  });
});
