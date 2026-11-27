import { describe, expect, it } from "vitest";
import { defaultPreferences, nextPinId, parsePreferences } from "./ui-preferences";

const pin = {
  id: 3,
  label: "Run 3",
  settings: {},
  command: "",
  metrics: {},
  selected: true,
  time: 1,
  seconds: 1,
  intervals: 1,
};

describe("UI preferences", () => {
  it("keeps defaults for missing or out-of-range fields", () => {
    expect(parsePreferences(JSON.stringify({ preview: { width: 123, maxFps: 37 }, pinWindow: 0 }))).toEqual(
      defaultPreferences,
    );
  });

  it("restores valid fields and pins", () => {
    const prefs = parsePreferences(
      JSON.stringify({ preview: { on: true, width: 960, alphaMode: "mask" }, pins: [pin] }),
    );
    expect(prefs).toMatchObject({ preview: { on: true, width: 960, alphaMode: "mask" }, pins: [pin] });
  });

  it("rejects malformed documents and pins", () => {
    expect(() => parsePreferences("[]")).toThrow("invalid preferences");
    expect(() => parsePreferences(JSON.stringify({ pins: [{ ...pin, metrics: null }] }))).toThrow("invalid saved pins");
    expect(() => parsePreferences(JSON.stringify({ pins: "x" }))).toThrow("invalid saved pins");
  });

  it("allocates ids above existing pins", () => {
    expect(nextPinId([])).toBe(1);
    expect(nextPinId([{ ...pin, id: 7 }, pin])).toBe(8);
  });
});
