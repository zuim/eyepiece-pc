import { describe, expect, it } from "vitest";
import { errorHelp } from "./error-help";

describe("error help", () => {
  it("says what to do about a video or control interface another program holds", () => {
    for (const message of [
      "cannot connect to the bridge over USB: the goggles' video interface is in use by another program",
      "the goggles' control interface is in use by another program",
    ]) {
      const steps = errorHelp(message)?.steps.join(" ");
      expect(steps).toMatch(/second eyepiece-pc/);
      expect(steps).toMatch(/replug the goggles/);
    }
  });

  it("adds nothing to other errors", () => {
    expect(errorHelp("goggles not found in tinkerglass USB mode")).toBeNull();
    expect(errorHelp("")).toBeNull();
  });
});
