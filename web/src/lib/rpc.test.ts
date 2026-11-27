import { afterEach, describe, expect, it, vi } from "vitest";
import { eyepiece } from "./generated/eyepiece-rpc.js";
import { metricValues, pickVideoFile, previewCommand, propertyValue, RpcClient, wireValue } from "./rpc";

describe("preview codec negotiation", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("advertises supported H.264 in the protobuf request", async () => {
    const probe = vi.fn().mockResolvedValue({ supported: true });
    vi.stubGlobal("VideoDecoder", { isConfigSupported: probe });
    vi.stubGlobal("EncodedVideoChunk", class {});
    const command = await previewCommand(true);
    const bytes = eyepiece.rpc.v1.Envelope.encode({ id: "preview", ...command }).finish();
    expect(eyepiece.rpc.v1.Envelope.decode(bytes).setPreview).toMatchObject({
      enabled: true,
      supportedCodecs: ["h264"],
    });
    expect(probe).toHaveBeenCalledWith(expect.objectContaining({ codec: "avc1.640033" }));
  });

  it("rejects unsupported or unavailable decoders instead of enabling preview", async () => {
    vi.stubGlobal("VideoDecoder", undefined);
    await expect(previewCommand(true)).rejects.toThrow("unavailable");
    vi.stubGlobal("EncodedVideoChunk", class {});
    vi.stubGlobal("VideoDecoder", { isConfigSupported: vi.fn().mockResolvedValue({ supported: false }) });
    await expect(previewCommand(true)).rejects.toThrow("does not support H.264");
  });

  it("can disable preview without WebCodecs", async () => {
    vi.stubGlobal("VideoDecoder", undefined);
    expect(await previewCommand(false)).toEqual({ setPreview: { enabled: false } });
  });
});

describe("RPC connection disposal", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("does not create a WebSocket when the session fetch resolves after close", async () => {
    let resolveSession!: (response: Response) => void;
    let sessionSignal: AbortSignal | undefined;
    const fetchSession = vi.fn((_url: string, init?: RequestInit) => {
      sessionSignal = init?.signal ?? undefined;
      return new Promise<Response>((resolve) => {
        resolveSession = resolve;
      });
    });
    const webSocket = vi.fn();
    vi.stubGlobal("fetch", fetchSession);
    vi.stubGlobal("WebSocket", webSocket);

    const client = new RpcClient(
      () => {},
      () => {},
    );
    const connecting = client.connect();
    client.close();
    resolveSession({
      ok: true,
      json: async () => ({ token: "session", workerStatus: "connected" }),
    } as Response);

    await expect(connecting).rejects.toThrow("RPC client is closed");
    expect(sessionSignal?.aborted).toBe(true);
    expect(fetchSession).toHaveBeenCalledWith(
      "/api/session",
      expect.objectContaining({
        signal: expect.any(AbortSignal),
      }),
    );
    expect(webSocket).not.toHaveBeenCalled();
  });
});

describe("RPC request sending", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("correlates framed envelope responses and rejects missing messages", async () => {
    class Socket extends EventTarget {
      static OPEN = 1;
      static instance: Socket;
      readyState = 0;
      binaryType = "arraybuffer";
      sent?: Uint8Array;
      closeCode?: number;

      constructor() {
        super();
        Socket.instance = this;
        queueMicrotask(() => {
          this.readyState = Socket.OPEN;
          this.dispatchEvent(new Event("open"));
        });
      }

      send(data: Uint8Array) {
        this.sent = data;
      }
      close(code?: number) {
        this.closeCode = code;
        this.readyState = 3;
      }
      receive(payload: Uint8Array) {
        const frame = new Uint8Array(payload.length + 4);
        new DataView(frame.buffer).setUint32(0, payload.length, false);
        frame.set(payload, 4);
        this.dispatchEvent(new MessageEvent("message", { data: frame.buffer }));
      }
    }

    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ token: "session", workerStatus: "connected" }),
      }),
    );
    vi.stubGlobal("location", { protocol: "http:", host: "localhost" });
    vi.stubGlobal("WebSocket", Socket);
    const client = new RpcClient(
      () => {},
      () => {},
    );
    await client.connect();
    await expect(client.send({})).rejects.toThrow("no command");
    await expect(client.send({ getState: {}, stop: {} })).rejects.toThrow("multiple values");
    expect(Socket.instance.sent).toBeUndefined();
    const pending = client.send({ getState: {} });
    const frame = Socket.instance.sent!;
    expect(new DataView(frame.buffer, frame.byteOffset).getUint32(0, false)).toBe(frame.length - 4);
    const request = eyepiece.rpc.v1.Envelope.decode(frame.subarray(4));
    expect(request.message).toBe("getState");
    Socket.instance.receive(eyepiece.rpc.v1.Envelope.encode({ id: request.id, acknowledged: true }).finish());
    expect((await pending).acknowledged).toBe(true);
    Socket.instance.receive(
      eyepiece.rpc.v1.Envelope.encode({
        state: {},
        error: { message: "invalid" },
      }).finish(),
    );
    expect(Socket.instance.closeCode).toBe(1003);
    Socket.instance.closeCode = undefined;
    Socket.instance.dispatchEvent(new MessageEvent("message", { data: "not binary" }));
    expect(Socket.instance.closeCode).toBe(1003);
    Socket.instance.closeCode = undefined;
    Socket.instance.receive(eyepiece.rpc.v1.Envelope.encode({ id: "missing" }).finish());
    expect(Socket.instance.closeCode).toBe(1003);
    client.close();
  });

  it("returns a rejected promise when WebSocket.send throws", async () => {
    class FailingSocket extends EventTarget {
      static OPEN = 1;
      readyState = 0;
      binaryType = "arraybuffer";

      constructor() {
        super();
        queueMicrotask(() => {
          this.readyState = FailingSocket.OPEN;
          this.dispatchEvent(new Event("open"));
        });
      }

      send() {
        throw new Error("Socket send failed");
      }

      close() {
        this.readyState = 3;
        this.dispatchEvent(new Event("close"));
      }
    }

    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ token: "session", workerStatus: "connected" }),
      }),
    );
    vi.stubGlobal("location", { protocol: "http:", host: "localhost" });
    vi.stubGlobal("WebSocket", FailingSocket);

    const client = new RpcClient(
      () => {},
      () => {},
    );
    await client.connect();
    let request: ReturnType<RpcClient["send"]> | undefined;

    expect(() => {
      request = client.send({ getState: {} });
    }).not.toThrow();
    await expect(request).rejects.toThrow("Socket send failed");
    client.close();
  });
});

describe("metricValues", () => {
  it("maps decoded protobuf names and includes zero-valued metrics", () => {
    const bytes = eyepiece.rpc.v1.Metrics.encode({
      intervalS: 0.2,
      sentFps: 30,
      capEncP95Ms: 1.5,
      reportIntervalS: 0.1,
      sampledAtMs: 1700000000123,
    }).finish();
    expect(metricValues(eyepiece.rpc.v1.Metrics.decode(bytes))).toMatchObject({
      interval_s: 0.2,
      sent_fps: 30,
      cap_enc_p95_ms: 1.5,
      report_interval_s: 0.1,
      sampled_at_ms: 1700000000123,
      dropped: 0,
    });
  });
});

describe("wire values", () => {
  it("round-trips supported setting values", () => {
    expect(wireValue(null)).toEqual({ unset: true });
    expect(wireValue(3)).toEqual({ integer: 3 });
    expect(wireValue(0.5)).toEqual({ number: 0.5 });
    expect(wireValue(["a"])).toEqual({ strings: { values: ["a"] } });
    expect(() => wireValue({})).toThrow("Unsupported");
    const decoded = eyepiece.rpc.v1.Value.decode(eyepiece.rpc.v1.Value.encode(wireValue("x")).finish());
    expect(propertyValue(decoded as never)).toBe("x");
  });
});

describe("video file dialog", () => {
  const json = (body: unknown, status = 200) =>
    new Response(JSON.stringify(body), { status, headers: { "Content-Type": "application/json" } });

  it("sends the session token and returns the chosen path or null", async () => {
    const fetcher = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
      if (input === "/api/session") return json({ token: "secret" });
      expect(new Headers(init?.headers).get("x-eyepiece-token")).toBe("secret");
      return json({ path: "C:\Videos\clip.mkv" });
    });
    await expect(pickVideoFile(fetcher as typeof fetch)).resolves.toBe("C:\Videos\clip.mkv");
    fetcher.mockImplementation(async (input: RequestInfo | URL) =>
      input === "/api/session" ? json({ token: "secret" }) : json({ path: null }),
    );
    await expect(pickVideoFile(fetcher as typeof fetch)).resolves.toBeNull();
  });

  it("reports dialog failures", async () => {
    const fetcher = vi.fn(async (input: RequestInfo | URL) =>
      input === "/api/session" ? json({ token: "t" }) : json({ error: "no file dialog available" }),
    );
    await expect(pickVideoFile(fetcher as typeof fetch)).rejects.toThrow("no file dialog available");
    fetcher.mockImplementation(async (input: RequestInfo | URL) =>
      input === "/api/session" ? json({ token: "t" }) : new Response("Forbidden", { status: 403 }),
    );
    await expect(pickVideoFile(fetcher as typeof fetch)).rejects.toThrow("403");
  });
});
