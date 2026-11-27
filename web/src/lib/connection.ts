import { RpcClient, type RpcCommand, type RpcResponse } from "./rpc";

export type RpcTransport = Pick<RpcClient, "connect" | "send" | "close">;

export type ConnectionOptions = {
  onEvent: (response: RpcResponse) => void;
  onStatus: (status: string) => void;
  onError: (message: string) => void;
  /** Called before each (re)connection attempt, when state tied to the previous client becomes stale. */
  onConnecting?: () => void;
  createClient?: (onEvent: (response: RpcResponse) => void, onStatus: (status: string) => void) => RpcTransport;
  reconnectDelayMs?: number;
  refreshIntervalMs?: number;
};

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/** Owns the RPC client: connects, reconnects after failures and periodically refreshes the state. */
export class Connection {
  private current?: RpcTransport;
  private status = "starting";
  private disposed = false;
  private reconnectTimer?: ReturnType<typeof setTimeout>;
  private refreshTimer?: ReturnType<typeof setInterval>;
  private refreshing = false;

  constructor(private readonly options: ConnectionOptions) {}

  /** The active client; changes on every reconnect, so it can be used to detect stale async work. */
  get client(): RpcTransport | undefined {
    return this.current;
  }

  start(): void {
    void this.connect();
    this.refreshTimer = setInterval(() => this.refresh(), this.options.refreshIntervalMs ?? 5000);
  }

  dispose(): void {
    this.disposed = true;
    clearTimeout(this.reconnectTimer);
    clearInterval(this.refreshTimer);
    const client = this.current;
    this.current = undefined;
    client?.close();
  }

  send(command: RpcCommand): Promise<RpcResponse> {
    if (!this.current) return Promise.reject(new Error("RPC connection is not ready"));
    return this.current.send(command);
  }

  private isCurrent(client: RpcTransport): boolean {
    return !this.disposed && this.current === client;
  }

  private async connect(): Promise<void> {
    if (this.disposed) return;
    this.options.onConnecting?.();
    const create = this.options.createClient ?? ((onEvent, onStatus) => new RpcClient(onEvent, onStatus));
    const client: RpcTransport = create(
      (response) => {
        if (!this.disposed) this.options.onEvent(response);
      },
      (status) => {
        if (!this.isCurrent(client)) return;
        this.setStatus(status);
        if (status === "disconnected") this.scheduleReconnect();
      },
    );
    this.current = client;
    try {
      await client.connect();
      if (this.isCurrent(client)) await client.send({ getState: {} });
    } catch (error) {
      if (this.isCurrent(client)) {
        this.setStatus(errorMessage(error));
        this.scheduleReconnect();
      }
    }
    if (!this.isCurrent(client) || this.status !== "connected") client.close();
  }

  private setStatus(status: string): void {
    this.status = status;
    this.options.onStatus(status);
  }

  private scheduleReconnect(): void {
    clearTimeout(this.reconnectTimer);
    this.reconnectTimer = setTimeout(() => void this.connect(), this.options.reconnectDelayMs ?? 1200);
  }

  private refresh(): void {
    if (this.disposed || !this.current || this.status !== "connected" || this.refreshing) return;
    this.refreshing = true;
    this.current
      .send({ getState: {} })
      .catch((error) => {
        if (!this.disposed) this.options.onError(errorMessage(error));
      })
      .finally(() => {
        this.refreshing = false;
      });
  }
}
