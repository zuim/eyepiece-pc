export function installMockWorker(schema) {
  const values = Object.fromEntries(schema.settings.map((setting) => [setting.name, setting.default]));
  const commands = [];
  const paneUpdates = [];
  const propertyUpdates = [];
  const previewUpdates = [];
  let runningValues = {};
  let running = false;
  let failNextSetting = false;
  let protocol;
  window.mockWorker = {
    commands,
    paneUpdates,
    propertyUpdates,
    previewUpdates,
    failNextSetting() {
      failNextSetting = true;
    },
  };

  function wireValue(value) {
    if (value === null || value === undefined) return { unset: true };
    if (typeof value === "boolean") return { boolean: value };
    if (typeof value === "number") return Number.isInteger(value) ? { integer: value } : { number: value };
    if (typeof value === "string") return { text: value };
    return { strings: { values: value } };
  }

  function properties(source) {
    return Object.entries(source).map(([name, value]) => ({ name, value: wireValue(value) }));
  }

  function state() {
    return {
      settingsSchemaJson: JSON.stringify(schema),
      properties: properties(values),
      runningProperties: properties(runningValues),
      phase: running ? "running" : "stopped",
      desiredRunning: running,
      connected: running,
      commandLine: `eyepiece-pc --source ${values.source} --bitrate ${values.bitrate}`,
      runningCommandLine: running
        ? `eyepiece-pc --source ${runningValues.source} --bitrate ${runningValues.bitrate}`
        : "",
    };
  }

  const fetchOriginal = window.fetch;
  window.fetch = (input, init) => {
    if (typeof input === "string" && input === "/api/session") {
      return Promise.resolve(
        new Response(JSON.stringify({ token: "mock", workerStatus: "connected" }), {
          headers: { "Content-Type": "application/json" },
        }),
      );
    }
    return fetchOriginal(input, init);
  };

  class MockSocket extends EventTarget {
    static CONNECTING = 0;
    static OPEN = 1;
    static CLOSING = 2;
    static CLOSED = 3;
    readyState = MockSocket.CONNECTING;
    binaryType = "arraybuffer";

    constructor(url) {
      super();
      if (!String(url).includes("/api/rpc?")) throw new Error("Unexpected WebSocket URL");
      import("/src/lib/generated/eyepiece-rpc.js")
        .then(({ eyepiece }) => {
          protocol = eyepiece.rpc.v1;
          this.readyState = MockSocket.OPEN;
          this.dispatchEvent(new Event("open"));
        })
        .catch((error) => {
          console.error("Mock RPC initialization failed", error);
          this.dispatchEvent(new Event("error"));
        });
    }

    send(frame) {
      const request = protocol.Envelope.decode(new Uint8Array(frame).subarray(4));
      commands.push(request.message);
      if (request.setProperties) {
        for (const property of request.setProperties.properties) {
          propertyUpdates.push({ name: property.name, value: property.value });
          if (property.name === "pane") paneUpdates.push({ value: property.value.text ?? null, at: performance.now() });
        }
        if (failNextSetting) {
          failNextSetting = false;
          this.reply({ id: request.id, error: { message: "Mock setting rejected" } });
          return;
        }
        for (const property of request.setProperties.properties) {
          const value = property.value;
          values[property.name] =
            value.kind === "unset"
              ? null
              : value.kind === "strings"
                ? value.strings.values
                : value.kind === "integer"
                  ? Number(value.integer)
                  : value[value.kind];
        }
        this.reply({ id: request.id, state: state() });
        const opacityUpdates = request.setProperties.properties
          .filter((property) => ["gui_opacity", "pane_opacity"].includes(property.name))
          .map((property) => [property.name, values[property.name]]);
        if (running && opacityUpdates.length) {
          setTimeout(() => {
            for (const [name, opacity] of opacityUpdates) runningValues[name] = opacity;
            this.reply({ state: state() });
          }, 20);
        }
      } else if (request.setPreview) {
        previewUpdates.push({ enabled: request.setPreview.enabled, keyframesOnly: request.setPreview.keyframesOnly });
        this.reply({ id: request.id, acknowledged: true });
      } else if (request.start || request.restart) {
        running = true;
        runningValues = { ...values };
        this.reply({ id: request.id, state: state() });
        setTimeout(() => {
          this.reply({ metrics: { intervalS: 1, reportIntervalS: 1, sentFps: 60, shownFps: 58, dropped: 3 } });
          this.reply({ metrics: { intervalS: 1, reportIntervalS: 1, sentFps: 80, shownFps: 78, dropped: 5 } });
        }, 20);
      } else if (request.stop) {
        running = false;
        this.reply({ id: request.id, state: state() });
      } else if (request.getState) {
        this.reply({ id: request.id, state: state() });
      } else if (request.listKmsOutputs) {
        this.reply({ id: request.id, kmsOutputs: { outputs: ["HDMI-A-1", "DP-1"] } });
      } else {
        this.reply({ id: request.id, acknowledged: true });
      }
    }

    reply(message) {
      const payload = protocol.Envelope.encode(message).finish();
      const frame = new Uint8Array(payload.length + 4);
      new DataView(frame.buffer).setUint32(0, payload.length, false);
      frame.set(payload, 4);
      queueMicrotask(() => this.dispatchEvent(new MessageEvent("message", { data: frame.buffer })));
    }

    close() {
      this.readyState = MockSocket.CLOSED;
      this.dispatchEvent(new Event("close"));
    }
  }
  window.WebSocket = MockSocket;
}
