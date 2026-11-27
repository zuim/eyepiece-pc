import { describe, expect, it } from "vitest";
import { GenerationGuard } from "./generation-guard";

describe("async generation guard", () => {
  it("blocks resource creation when async setup completes after disposal", async () => {
    const guard = new GenerationGuard();
    const generation = guard.current();
    let finishSetup!: () => void;
    let created = false;
    const setup = new Promise<void>((resolve) => {
      finishSetup = resolve;
    }).then(() => {
      if (guard.isCurrent(generation)) created = true;
    });

    guard.dispose();
    finishSetup();
    await setup;

    expect(created).toBe(false);
  });

  it("invalidates stale setup after configuration changes", () => {
    const guard = new GenerationGuard();
    const staleGeneration = guard.current();

    guard.invalidate();

    expect(guard.isCurrent(staleGeneration)).toBe(false);
    expect(guard.isCurrent(guard.current())).toBe(true);
  });
});
