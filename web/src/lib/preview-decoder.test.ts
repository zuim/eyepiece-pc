import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { PreviewDecoder, type PreviewSample } from "./preview-decoder";

class FakeVideoDecoder {
  static instances: FakeVideoDecoder[] = [];
  static isConfigSupported = vi.fn(async (config: unknown) => ({ supported: true, config }));
  state = "unconfigured";
  decodeQueueSize = 0;
  chunks: unknown[] = [];
  constructor(readonly init: { output: (frame: unknown) => void; error: (error: Error) => void }) {
    FakeVideoDecoder.instances.push(this);
  }
  configure() {
    this.state = "configured";
  }
  decode(chunk: unknown) {
    this.chunks.push(chunk);
  }
  reset() {
    this.state = "unconfigured";
  }
  close() {
    this.state = "closed";
  }
}

const startCode = [0, 0, 0, 1];
const accessUnit = (...nals: number[][]) => new Uint8Array(nals.flatMap((nal) => [...startCode, ...nal]));
const sps = [0x67, 0x64, 0x00, 0x33];
const pps = [0x68, 0xee, 0x3c];
const idr = [0x65, 1, 2, 3];
const delta = [0x41, 1, 2, 3];

const keyframe = (): PreviewSample => ({ codec: "h264", key: true, accessUnit: accessUnit(sps, pps, idr) });
const deltaFrame = (): PreviewSample => ({ codec: "h264", key: false, accessUnit: accessUnit(delta) });

describe("PreviewDecoder", () => {
  beforeEach(() => {
    FakeVideoDecoder.instances = [];
    vi.stubGlobal("VideoDecoder", FakeVideoDecoder);
    vi.stubGlobal(
      "EncodedVideoChunk",
      class {
        constructor(readonly init: { type: string }) {}
      },
    );
  });
  afterEach(() => vi.unstubAllGlobals());

  function setup() {
    const messages: string[] = [];
    const decoder = new PreviewDecoder(vi.fn(), (message) => messages.push(message));
    return { decoder, messages };
  }

  it("waits for a keyframe before decoding delta frames", async () => {
    const { decoder, messages } = setup();
    await decoder.decode(deltaFrame());
    expect(messages).toEqual(["Waiting for keyframe"]);
    expect(FakeVideoDecoder.instances).toHaveLength(0);

    await decoder.decode(keyframe());
    await decoder.decode(deltaFrame());
    const [instance] = FakeVideoDecoder.instances;
    expect(instance.chunks).toHaveLength(2);
    expect(messages.at(-1)).toBe("");
  });

  it("requires a new keyframe after a discontinuity", async () => {
    const { decoder, messages } = setup();
    await decoder.decode(keyframe());
    await decoder.decode({ ...deltaFrame(), discontinuity: true });
    expect(messages.at(-1)).toBe("Waiting for keyframe");
    expect(FakeVideoDecoder.instances[0].chunks).toHaveLength(1);
  });

  it("resets and resynchronises when the decode queue is overloaded", async () => {
    const { decoder, messages } = setup();
    await decoder.decode(keyframe());
    FakeVideoDecoder.instances[0].decodeQueueSize = 20;
    await decoder.decode(deltaFrame());
    expect(messages.at(-1)).toBe("Preview decoder overloaded; waiting for keyframe");
    FakeVideoDecoder.instances[0].decodeQueueSize = 0;
    await decoder.decode(deltaFrame());
    expect(messages.at(-1)).toBe("Waiting for keyframe");
  });

  it("rejects other codecs and unsupported configurations", async () => {
    const { decoder, messages } = setup();
    await decoder.decode({ codec: "hevc", accessUnit: new Uint8Array([1]) });
    expect(messages.at(-1)).toContain("HEVC preview is not supported");
    FakeVideoDecoder.isConfigSupported.mockResolvedValueOnce({ supported: false, config: undefined } as never);
    await decoder.decode(keyframe());
    expect(messages.at(-1)).toBe("Browser does not support avc1.640033");
  });

  it("does not configure a decoder after being closed during support probing", async () => {
    const { decoder } = setup();
    const decoding = decoder.decode(keyframe());
    decoder.close();
    await decoding;
    expect(FakeVideoDecoder.instances).toHaveLength(0);
  });
});
