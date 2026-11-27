import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { Connection, type RpcTransport } from "./connection";
import type { RpcResponse } from "./rpc";

class FakeClient implements RpcTransport {
  connect = vi.fn(async () => {
    this.onStatus("connected");
  });
  send = vi.fn(async () => ({}) as RpcResponse);
  close = vi.fn();
  constructor(readonly onStatus: (status: string) => void) {}
}

function setup(failFirstConnect = false) {
  const clients: FakeClient[] = [];
  const statuses: string[] = [];
  const errors: string[] = [];
  const onConnecting = vi.fn();
  const connection = new Connection({
    onEvent: () => {},
    onStatus: (status) => statuses.push(status),
    onError: (message) => errors.push(message),
    onConnecting,
    createClient: (_onEvent, onStatus) => {
      const client = new FakeClient(onStatus);
      if (failFirstConnect && clients.length === 0) client.connect.mockRejectedValue(new Error("refused"));
      clients.push(client);
      return client;
    },
  });
  return { clients, statuses, errors, onConnecting, connection };
}

describe("Connection", () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  it("connects and requests the initial state", async () => {
    const { clients, statuses, connection } = setup();
    connection.start();
    await vi.advanceTimersByTimeAsync(0);
    expect(statuses).toEqual(["connected"]);
    expect(clients[0].send).toHaveBeenCalledWith({ getState: {} });
    connection.dispose();
  });

  it("reconnects with a new client after a failed attempt and after a disconnect", async () => {
    const { clients, statuses, onConnecting, connection } = setup(true);
    connection.start();
    await vi.advanceTimersByTimeAsync(0);
    expect(statuses).toEqual(["refused"]);
    expect(clients[0].close).toHaveBeenCalled();
    await vi.advanceTimersByTimeAsync(1200);
    expect(clients).toHaveLength(2);
    expect(connection.client).toBe(clients[1]);
    clients[1].onStatus("disconnected");
    await vi.advanceTimersByTimeAsync(1200);
    expect(clients).toHaveLength(3);
    expect(onConnecting).toHaveBeenCalledTimes(3);
    connection.dispose();
  });

  it("refreshes the state periodically and reports failures", async () => {
    const { clients, errors, connection } = setup();
    connection.start();
    await vi.advanceTimersByTimeAsync(0);
    clients[0].send.mockRejectedValueOnce(new Error("timed out"));
    await vi.advanceTimersByTimeAsync(5000);
    expect(errors).toEqual(["timed out"]);
    await vi.advanceTimersByTimeAsync(5000);
    expect(clients[0].send).toHaveBeenCalledTimes(3);
    connection.dispose();
  });

  it("closes the client and stops reconnecting after dispose", async () => {
    const { clients, connection } = setup();
    connection.start();
    await vi.advanceTimersByTimeAsync(0);
    connection.dispose();
    expect(clients[0].close).toHaveBeenCalled();
    clients[0].onStatus("disconnected");
    await vi.advanceTimersByTimeAsync(10_000);
    expect(clients).toHaveLength(1);
    await expect(connection.send({ getState: {} })).rejects.toThrow("not ready");
  });
});
