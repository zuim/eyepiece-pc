import { describe, expect, it } from "vitest";
import schema from "../../../contract/settings.json";
import { settingsHelp } from "./settings-help";

describe("settings help", () => {
  it("documents every schema setting", () => {
    for (const setting of schema.settings) {
      expect(settingsHelp[setting.name], setting.name).toBeTruthy();
    }
  });
});
