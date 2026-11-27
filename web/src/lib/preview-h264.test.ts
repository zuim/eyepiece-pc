import { describe, expect, it } from "vitest";
import { avcDescription, lengthPrefixed, nalUnits } from "./preview-h264";

describe("H.264 preview framing", () => {
  it("splits three- and four-byte Annex B start codes", () => {
    const frame = new Uint8Array([0, 0, 0, 1, 0x67, 0x64, 0, 0x33, 0, 0, 1, 0x68, 0x01, 0, 0, 0, 1, 0x65, 0x88]);

    expect(nalUnits(frame).map((unit) => [...unit])).toEqual([
      [0x67, 0x64, 0, 0x33],
      [0x68, 0x01],
      [0x65, 0x88],
    ]);
  });

  it("builds AVC decoder configuration from SPS and PPS units", () => {
    const sps = new Uint8Array([0x67, 0x64, 0, 0x33]);
    const pps = new Uint8Array([0x68, 0x01]);
    const result = avcDescription([sps, pps]);

    expect(result?.codec).toBe("avc1.640033");
    expect([...(result?.description ?? [])]).toEqual([1, 0x64, 0, 0x33, 0xff, 0xe1, 0, 4, ...sps, 1, 0, 2, ...pps]);
    expect(avcDescription([new Uint8Array([0x67, 0x64, 0])])).toBeUndefined();
  });

  it("length-prefixes each NAL unit in network byte order", () => {
    expect([...lengthPrefixed([new Uint8Array([0x65, 0x88]), new Uint8Array([0x09])])]).toEqual([
      0, 0, 0, 2, 0x65, 0x88, 0, 0, 0, 1, 0x09,
    ]);
  });
});
