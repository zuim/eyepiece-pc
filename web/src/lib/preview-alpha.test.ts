import { describe, expect, it } from "vitest";
import { unpackPreviewAlpha } from "./preview-alpha";

describe("packed preview alpha", () => {
  it("combines corresponding rows without premultiplying color or expanding alpha twice", () => {
    const pixels = new Uint8ClampedArray([
      255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 128, 128, 128, 255, 0, 0, 255, 255, 80, 100, 120, 255, 255, 255,
      255, 255, 64, 64, 64, 255,
    ]);
    expect(unpackPreviewAlpha(pixels, 4, 2)).toEqual(
      new Uint8ClampedArray([255, 0, 0, 0, 0, 255, 0, 128, 0, 0, 255, 255, 80, 100, 120, 64]),
    );
  });

  it("can render packed alpha as an opaque grayscale mask", () => {
    const pixels = new Uint8ClampedArray([255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 128, 128, 128, 255]);
    expect(unpackPreviewAlpha(pixels, 4, 1, "mask")).toEqual(new Uint8ClampedArray([0, 0, 0, 255, 128, 128, 128, 255]));
  });

  it("rejects malformed packed frames", () => {
    for (const [width, height] of [
      [3, 2],
      [0, 2],
      [4, 0],
      [4, 1.5],
      [4, 2],
    ]) {
      expect(() => unpackPreviewAlpha(new Uint8ClampedArray(4), width, height)).toThrow(
        "Invalid packed-alpha preview dimensions",
      );
    }
  });
});
