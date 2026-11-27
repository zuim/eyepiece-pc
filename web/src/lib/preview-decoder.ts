import type { eyepiece } from "./generated/eyepiece-rpc.js";
import { GenerationGuard } from "./generation-guard";
import { avcDescription, lengthPrefixed, nalUnits } from "./preview-h264";

export type PreviewSample = eyepiece.rpc.v1.IPreview;

const MAX_DECODE_QUEUE = 8;

function decoderError(error: unknown): string {
  return error instanceof Error ? `Decoder error: ${error.message}` : "Video decoder failed";
}

/** Decodes H.264 preview access units with WebCodecs, resynchronising on keyframes after gaps or overload. */
export class PreviewDecoder {
  /** Upper bound for emitted frames per second; surplus frames are dropped. */
  maxFps = 50;
  packedAlpha = false;

  private decoder?: VideoDecoder;
  private configured = false;
  private waitingForKey = true;
  private config?: VideoDecoderConfig;
  private configuring?: Promise<void>;
  private codec = "";
  private codecString = "";
  private lastShownTimestamp = -1;
  private readonly generation = new GenerationGuard();

  /** `onFrame` takes ownership of the frame and must close it. */
  constructor(
    private readonly onFrame: (frame: VideoFrame, packedAlpha: boolean) => void,
    private readonly onMessage: (message: string) => void,
  ) {}

  close(): void {
    this.generation.invalidate();
    if (this.decoder && this.decoder.state !== "closed") this.decoder.close();
    this.decoder = undefined;
    this.configured = false;
    this.waitingForKey = true;
    this.config = undefined;
    this.codec = "";
    this.codecString = "";
    this.lastShownTimestamp = -1;
    this.configuring = undefined;
  }

  dispose(): void {
    this.generation.dispose();
    this.close();
  }

  async decode(sample: PreviewSample): Promise<void> {
    if (typeof VideoDecoder === "undefined" || typeof EncodedVideoChunk === "undefined") {
      this.onMessage("WebCodecs video decoding is unavailable in this browser");
      return;
    }
    const codecName = sample.codec ?? "";
    if (codecName.toLowerCase() !== "h264") {
      this.onMessage(`${codecName.toUpperCase() || "Unknown codec"} preview is not supported by this browser decoder`);
      return;
    }
    const data = sample.accessUnit;
    if (!(data instanceof Uint8Array) || data.length === 0) return;
    const packedAlpha = sample.packedAlpha === true;
    if (this.packedAlpha !== packedAlpha) {
      this.close();
      this.packedAlpha = packedAlpha;
    }
    const units = nalUnits(data);
    const key = sample.key === true;
    const discontinuity = sample.discontinuity === true;
    if ((discontinuity || !this.configured || this.waitingForKey) && !key) {
      this.waitingForKey = true;
      this.onMessage("Waiting for keyframe");
      return;
    }
    if (discontinuity && this.decoder && this.decoder.state !== "closed") {
      this.decoder.reset();
      this.configured = false;
      this.lastShownTimestamp = -1;
    }
    const config =
      avcDescription(units) ??
      (this.config ? { codec: this.codecString, description: this.config.description as Uint8Array } : undefined);
    if (!config) {
      this.onMessage("Waiting for H.264 parameter sets and keyframe");
      return;
    }
    if (this.decoder && (this.codec !== codecName || this.codecString !== config.codec)) this.close();
    if (!this.decoder || !this.configured) await this.configure(config, codecName);
    const decoder = this.decoder;
    if (!this.configured || !decoder || decoder.state !== "configured") return;
    try {
      if (decoder.decodeQueueSize > MAX_DECODE_QUEUE) {
        decoder.reset();
        this.configured = false;
        this.waitingForKey = true;
        this.onMessage("Preview decoder overloaded; waiting for keyframe");
        return;
      }
      const timestamp = Number(sample.timestampUs?.toString() ?? 0);
      decoder.decode(
        new EncodedVideoChunk({
          type: key ? "key" : "delta",
          timestamp: Number.isFinite(timestamp) ? timestamp : 0,
          data: lengthPrefixed(units),
        }),
      );
      if (key) this.waitingForKey = false;
      this.onMessage("");
    } catch (error) {
      this.onMessage(decoderError(error));
    }
  }

  private async configure(config: { codec: string; description: Uint8Array }, codecName: string): Promise<void> {
    if (this.configuring) await this.configuring;
    if (this.decoder && this.configured) return;
    const selected: VideoDecoderConfig = {
      codec: config.codec,
      description: config.description,
      optimizeForLatency: true,
      hardwareAcceleration: "no-preference",
    };
    const generation = this.generation.current();
    const configuration = (async () => {
      const support = await VideoDecoder.isConfigSupported(selected);
      if (!this.generation.isCurrent(generation)) return;
      if (!support.supported || !support.config) {
        this.onMessage(`Browser does not support ${config.codec}`);
        return;
      }
      if (!this.decoder || this.decoder.state === "closed") {
        this.decoder = new VideoDecoder({
          output: (frame) => this.output(frame),
          error: (error) => {
            this.close();
            this.onMessage(`Decoder error: ${error.message}`);
          },
        });
      }
      this.decoder.configure(support.config);
      this.config = support.config;
      this.configured = true;
      this.codec = codecName;
      this.codecString = config.codec;
    })();
    this.configuring = configuration;
    try {
      await configuration;
    } catch (error) {
      if (this.generation.isCurrent(generation)) this.onMessage(decoderError(error));
    } finally {
      if (this.configuring === configuration) this.configuring = undefined;
    }
  }

  private output(frame: VideoFrame): void {
    const intervalUs = 1_000_000 / Math.max(1, this.maxFps);
    const tooSoon =
      this.lastShownTimestamp >= 0 &&
      frame.timestamp > this.lastShownTimestamp &&
      frame.timestamp - this.lastShownTimestamp < intervalUs;
    if (tooSoon) {
      frame.close();
      return;
    }
    this.lastShownTimestamp = frame.timestamp;
    this.onFrame(frame, this.packedAlpha);
  }
}
