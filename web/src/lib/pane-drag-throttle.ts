import { parsePane } from "./pane-geometry";

const MOVE_INTERVAL_MS = 1000 / 25;
const RESIZE_INTERVAL_MS = 1000 / 15;

/** Rate-limits pane updates while dragging, always delivering the latest value. */
export class PaneDragThrottle {
  private timer: ReturnType<typeof setTimeout> | undefined;
  private pending: { value: string; interval: number } | undefined;
  private lastSent = -Infinity;
  private lastValue: string | undefined;

  constructor(
    private readonly send: (value: string) => Promise<boolean>,
    private readonly confirmed: () => unknown,
    private readonly now: () => number = () => performance.now(),
  ) {}

  update(value: string, mode: "move" | "resize"): void {
    if (this.timer === undefined && value === this.lastValue && this.confirmed() === value) return;
    const next = parsePane(value);
    const current = typeof this.confirmed() === "string" ? parsePane(this.confirmed() as string) : null;
    const sizeChanged = next && current && (next.width !== current.width || next.height !== current.height);
    this.pending = { value, interval: mode === "resize" || sizeChanged ? RESIZE_INTERVAL_MS : MOVE_INTERVAL_MS };
    if (this.timer !== undefined) clearTimeout(this.timer);
    this.flush();
  }

  cancel(): void {
    if (this.timer !== undefined) clearTimeout(this.timer);
    this.timer = undefined;
    this.pending = undefined;
    this.lastValue = undefined;
  }

  private flush = (): void => {
    this.timer = undefined;
    const pending = this.pending;
    if (!pending) return;
    const delay = pending.interval - (this.now() - this.lastSent);
    if (delay > 0) {
      this.timer = setTimeout(this.flush, Math.ceil(delay));
      return;
    }
    this.pending = undefined;
    if (pending.value === this.lastValue && this.confirmed() === pending.value) return;
    this.lastSent = this.now();
    this.lastValue = pending.value;
    void this.send(pending.value).then((accepted) => {
      if (!accepted && this.lastValue === pending.value) this.lastValue = undefined;
    });
  };
}
