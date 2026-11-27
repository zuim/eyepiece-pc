import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { PaneDragThrottle } from "./pane-drag-throttle";

describe("PaneDragThrottle", () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  function setup() {
    const sent: string[] = [];
    const throttle = new PaneDragThrottle(
      async (value) => {
        sent.push(value);
        return true;
      },
      () => "100x100+0+0",
      () => Date.now(),
    );
    return { sent, throttle };
  }

  it("sends the first move immediately and coalesces later moves to the latest value", () => {
    const { sent, throttle } = setup();
    throttle.update("100x100+2+0", "move");
    throttle.update("100x100+4+0", "move");
    throttle.update("100x100+6+0", "move");
    expect(sent).toEqual(["100x100+2+0"]);
    vi.advanceTimersByTime(40);
    expect(sent).toEqual(["100x100+2+0", "100x100+6+0"]);
  });

  it("drops queued updates on cancel", () => {
    const { sent, throttle } = setup();
    throttle.update("100x100+2+0", "move");
    throttle.update("100x100+4+0", "move");
    throttle.cancel();
    vi.advanceTimersByTime(200);
    expect(sent).toEqual(["100x100+2+0"]);
  });

  it("limits resizes to a lower rate than moves", () => {
    const { sent, throttle } = setup();
    throttle.update("100x100+2+0", "resize");
    throttle.update("102x100+2+0", "resize");
    vi.advanceTimersByTime(50);
    expect(sent).toHaveLength(1);
    vi.advanceTimersByTime(20);
    expect(sent).toHaveLength(2);
  });
});
