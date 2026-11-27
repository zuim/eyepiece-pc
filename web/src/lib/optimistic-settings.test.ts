import { describe, expect, it } from "vitest";
import { OptimisticSettings } from "./optimistic-settings";

describe("optimistic settings", () => {
  it("rolls back a failed latest edit to confirmed state, not a failed prior edit", () => {
    const settings = new OptimisticSettings({ bitrate: "20M" });
    const first = settings.begin({ bitrate: "30M" });
    const second = settings.begin({ bitrate: "40M" });

    expect(settings.settle(first)).toEqual({ bitrate: "40M" });
    expect(settings.settle(second)).toEqual({ bitrate: "20M" });
  });

  it("retains newer edits when older requests settle", () => {
    const settings = new OptimisticSettings({ encoder: "auto" });
    const first = settings.begin({ encoder: "nvenc" });
    const second = settings.begin({ encoder: "software" });

    settings.reconcile({ encoder: "nvenc" });
    expect(settings.settle(first)).toEqual({ encoder: "software" });
    expect(settings.settle(second)).toEqual({ encoder: "nvenc" });
  });

  it("overlays pending edits on refreshed server state", () => {
    const settings = new OptimisticSettings({ fps: 60 });
    const edit = settings.begin({ fps: 90 });

    expect(settings.reconcile({ fps: 60, codec: "h264" })).toEqual({ fps: 90, codec: "h264" });
    expect(settings.settle(edit)).toEqual({ fps: 60, codec: "h264" });
  });
});
