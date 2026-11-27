export type AlphaPreviewMode = "composite" | "mask";

export function unpackPreviewAlpha(
  pixels: Uint8ClampedArray,
  width: number,
  height: number,
  mode: AlphaPreviewMode = "composite",
): Uint8ClampedArray<ArrayBuffer> {
  if (
    !Number.isSafeInteger(width) ||
    !Number.isSafeInteger(height) ||
    width <= 0 ||
    width % 2 !== 0 ||
    height <= 0 ||
    pixels.length !== width * height * 4
  ) {
    throw new Error("Invalid packed-alpha preview dimensions");
  }
  const colorWidth = width / 2;
  const output = new Uint8ClampedArray(colorWidth * height * 4);
  for (let y = 0; y < height; y++) {
    const row = y * width * 4;
    for (let x = 0; x < colorWidth; x++) {
      const color = row + x * 4;
      const mask = color + colorWidth * 4;
      const target = (y * colorWidth + x) * 4;
      const alpha = Math.round(0.2126 * pixels[mask] + 0.7152 * pixels[mask + 1] + 0.0722 * pixels[mask + 2]);
      if (mode === "mask") {
        output[target] = alpha;
        output[target + 1] = alpha;
        output[target + 2] = alpha;
        output[target + 3] = 255;
      } else {
        output[target] = pixels[color];
        output[target + 1] = pixels[color + 1];
        output[target + 2] = pixels[color + 2];
        // Canvas already expands limited-range video to full-range RGB.
        output[target + 3] = alpha;
      }
    }
  }
  return output;
}
