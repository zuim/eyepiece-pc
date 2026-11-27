import { eyepiece } from "./generated/eyepiece-rpc.js";

const MAX_FRAME = 8 * 1024 * 1024;
const v1 = eyepiece.rpc.v1;
type WireValue = eyepiece.rpc.v1.IValue & {
  kind?: "boolean" | "integer" | "number" | "text" | "strings" | "unset";
};

export type RpcResponse = InstanceType<typeof v1.Envelope>;
export type RpcCommand = Pick<
  eyepiece.rpc.v1.IEnvelope,
  | "getState"
  | "setProperties"
  | "start"
  | "stop"
  | "restart"
  | "recenter"
  | "forceIdr"
  | "setPreview"
  | "setCommandLine"
  | "listKmsOutputs"
>;

export async function previewCommand(enabled: boolean, keyframesOnly = false): Promise<RpcCommand> {
  if (!enabled) return { setPreview: { enabled: false } };
  if (typeof VideoDecoder === "undefined" || typeof EncodedVideoChunk === "undefined") {
    throw new Error("WebCodecs video decoding is unavailable in this browser");
  }
  const support = await VideoDecoder.isConfigSupported({
    codec: "avc1.640033",
    optimizeForLatency: true,
    hardwareAcceleration: "no-preference",
  });
  if (!support.supported) throw new Error("Browser does not support H.264 preview decoding");
  return {
    setPreview: {
      enabled: true,
      supportedCodecs: ["h264"],
      keyframesOnly,
    },
  };
}

export function metricValues(metrics: eyepiece.rpc.v1.IMetrics): Record<string, number> {
  const values: Record<string, number> = {};
  for (const [name, value] of Object.entries(
    v1.Metrics.toObject(new v1.Metrics(metrics), { defaults: true, longs: Number }),
  )) {
    if (typeof value === "number") {
      values[name.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`)] = value;
    }
  }
  return values;
}

export class RpcClient {
  private socket?: WebSocket;
  private sequence = 0;
  private closed = false;
  private connectController?: AbortController;
  private readonly pending = new Map<
    string,
    {
      resolve: (response: RpcResponse) => void;
      reject: (error: Error) => void;
      timeout: ReturnType<typeof setTimeout>;
    }
  >();

  constructor(
    private readonly onEvent: (response: RpcResponse) => void,
    private readonly onStatus: (status: string) => void,
  ) {}

  async connect(): Promise<void> {
    if (this.closed) throw new Error("RPC client is closed");
    const controller = new AbortController();
    this.connectController = controller;
    try {
      const response = await fetch("/api/session", { cache: "no-store", signal: controller.signal });
      if (this.closed) throw new Error("RPC client is closed");
      if (!response.ok) throw new Error(`Backend session request failed (${response.status})`);
      const session = (await response.json()) as { token: string; workerStatus: string };
      if (this.closed) throw new Error("RPC client is closed");
      const protocol = location.protocol === "https:" ? "wss:" : "ws:";
      const socket = new WebSocket(`${protocol}//${location.host}/api/rpc?token=${encodeURIComponent(session.token)}`);
      socket.binaryType = "arraybuffer";
      this.socket = socket;
      this.onStatus(session.workerStatus);
      await new Promise<void>((resolve, reject) => {
        const cleanup = () => {
          socket.removeEventListener("open", onOpen);
          socket.removeEventListener("error", onError);
          socket.removeEventListener("close", onClose);
        };
        const onOpen = () => {
          cleanup();
          this.onStatus("connected");
          resolve();
        };
        const onError = () => {
          cleanup();
          reject(new Error("RPC WebSocket connection failed"));
        };
        const onClose = () => {
          cleanup();
          reject(new Error("RPC WebSocket closed before connecting"));
        };
        socket.addEventListener("open", onOpen, { once: true });
        socket.addEventListener("error", onError, { once: true });
        socket.addEventListener("close", onClose, { once: true });
        if (this.closed) {
          cleanup();
          socket.close();
          reject(new Error("RPC client is closed"));
        }
      });
      if (this.closed) {
        socket.close();
        throw new Error("RPC client is closed");
      }
      socket.addEventListener("message", (event: MessageEvent<unknown>) => this.receive(event.data));
      socket.addEventListener("close", () => {
        this.onStatus("disconnected");
        this.rejectPending(new Error("RPC connection closed"));
      });
    } finally {
      if (this.connectController === controller) this.connectController = undefined;
    }
  }

  send(command: RpcCommand): Promise<RpcResponse> {
    const socket = this.socket;
    if (!socket || socket.readyState !== WebSocket.OPEN) {
      return Promise.reject(new Error("RPC connection is not open"));
    }
    const id = `${Date.now()}-${++this.sequence}`;
    const request = { id, ...command };
    const validation = v1.Envelope.verify(request);
    if (validation) return Promise.reject(new Error(`Invalid RPC request: ${validation}`));
    if (!v1.Envelope.create(request).message) {
      return Promise.reject(new Error("RPC request has no command"));
    }
    let payload: Uint8Array;
    try {
      payload = v1.Envelope.encode(request).finish();
    } catch (error) {
      return Promise.reject(error instanceof Error ? error : new Error(String(error)));
    }
    if (payload.byteLength === 0 || payload.byteLength > MAX_FRAME) {
      return Promise.reject(new Error("RPC request exceeds frame limit"));
    }
    const frame = new Uint8Array(payload.byteLength + 4);
    new DataView(frame.buffer).setUint32(0, payload.byteLength, false);
    frame.set(payload, 4);
    try {
      socket.send(frame);
    } catch (error) {
      return Promise.reject(error instanceof Error ? error : new Error(String(error)));
    }
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pending.delete(id);
        reject(new Error("RPC request timed out"));
      }, 15_000);
      this.pending.set(id, { resolve, reject, timeout });
    });
  }

  close(): void {
    if (this.closed) return;
    this.closed = true;
    this.connectController?.abort();
    this.socket?.close();
    this.rejectPending(new Error("RPC client closed"));
  }

  private receive(buffer: unknown): void {
    if (!(buffer instanceof ArrayBuffer) || buffer.byteLength < 5 || buffer.byteLength > MAX_FRAME + 4) {
      this.socket?.close(1003, "Invalid protobuf frame");
      return;
    }
    const frame = new Uint8Array(buffer);
    const length = new DataView(buffer).getUint32(0, false);
    if (length !== frame.byteLength - 4) {
      this.socket?.close(1003, "Invalid protobuf frame");
      return;
    }
    let response: RpcResponse;
    try {
      const envelope = v1.Envelope.decode(frame.subarray(4));
      const validation = v1.Envelope.verify(envelope);
      if (validation) throw new Error(`Invalid RPC response: ${validation}`);
      if (
        !envelope.message ||
        !["state", "error", "log", "pose", "metrics", "preview", "acknowledged", "kmsOutputs"].includes(
          envelope.message,
        )
      ) {
        throw new Error("Unexpected RPC response message");
      }
      response = envelope;
    } catch (error) {
      this.socket?.close(1003, "Invalid protobuf response");
      this.onStatus(error instanceof Error ? error.message : "Invalid protobuf response");
      return;
    }
    if (response.id) {
      const pending = this.pending.get(response.id);
      if (pending) {
        clearTimeout(pending.timeout);
        this.pending.delete(response.id);
        if (response.error) pending.reject(new Error(response.error.message ?? "RPC request failed"));
        else pending.resolve(response);
      }
    }
    this.onEvent(response);
  }

  private rejectPending(error: Error): void {
    for (const pending of this.pending.values()) {
      clearTimeout(pending.timeout);
      pending.reject(error);
    }
    this.pending.clear();
  }
}

export function propertyValue(value: WireValue | null | undefined): unknown {
  if (!value || !value.kind) return null;
  switch (value.kind) {
    case "boolean":
      return value.boolean ?? false;
    case "integer":
      return Number(value.integer?.toString() ?? 0);
    case "number":
      return value.number ?? 0;
    case "text":
      return value.text ?? "";
    case "strings":
      return value.strings?.values ?? [];
    case "unset":
      return null;
  }
}

export function wireValue(value: unknown): eyepiece.rpc.v1.IValue {
  if (value === null || value === undefined) return { unset: true };
  if (typeof value === "boolean") return { boolean: value };
  if (typeof value === "number") {
    return Number.isInteger(value) ? { integer: value } : { number: value };
  }
  if (typeof value === "string") return { text: value };
  if (Array.isArray(value) && value.every((item) => typeof item === "string")) {
    return { strings: { values: value } };
  }
  throw new Error("Unsupported settings value");
}
