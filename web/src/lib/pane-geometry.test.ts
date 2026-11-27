import { describe, expect, it } from "vitest";
import { dragPane, formatPane, parsePane, validPane } from "./pane-geometry";

describe("pane geometry", () => {
  it("parses and formats panel geometry and rejects invalid dimensions or bounds", () => {
    const pane = { width: 640, height: 480, x: 101, y: 51 };
    expect(parsePane(formatPane(pane))).toEqual(pane);
    for (const value of ["640x480+1281+0", "641x480+0+0", "0x480+0+0", "640x480-1+0", "640x480+0+0+1"]) {
      expect(parsePane(value)).toBeNull();
    }
    expect(validPane({ ...pane, x: NaN })).toBe(false);
  });

  it("clamps movement and resizing to the panel while keeping sizes even", () => {
    const pane = { width: 640, height: 480, x: 101, y: 51 };
    expect(dragPane(pane, "move", -500, 2000)).toEqual({ ...pane, x: 0, y: 600 });
    expect(dragPane(pane, "se", 2000, 2000)).toEqual({ ...pane, width: 1818, height: 1028 });
    expect(dragPane(pane, "se", -2000, -2000)).toEqual({ ...pane, width: 2, height: 2 });
    expect(dragPane(pane, "se", 3, 5)).toEqual({ ...pane, width: 644, height: 486 });
  });

  it("resizes each edge and corner while anchoring the opposite edges", () => {
    const pane = { width: 640, height: 480, x: 101, y: 51 };
    const cases = [
      ["n", { width: 640, height: 460, x: 101, y: 71 }],
      ["s", { width: 640, height: 500, x: 101, y: 51 }],
      ["e", { width: 650, height: 480, x: 101, y: 51 }],
      ["w", { width: 630, height: 480, x: 111, y: 51 }],
      ["ne", { width: 650, height: 460, x: 101, y: 71 }],
      ["nw", { width: 630, height: 460, x: 111, y: 71 }],
      ["se", { width: 650, height: 500, x: 101, y: 51 }],
      ["sw", { width: 630, height: 500, x: 111, y: 51 }],
    ] as const;
    for (const [direction, expected] of cases) {
      expect(dragPane(pane, direction, 10, 20)).toEqual(expected);
    }
    expect(dragPane(pane, "nw", -2000, -2000)).toEqual({ width: 740, height: 530, x: 1, y: 1 });
    expect(dragPane(pane, "nw", 2000, 2000)).toEqual({ width: 2, height: 2, x: 739, y: 529 });
  });
});
