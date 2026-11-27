export type SettingEditToken = { name: string; revision: number };

export class OptimisticSettings {
  private confirmed: Record<string, unknown>;
  private readonly revisions = new Map<string, number>();
  private readonly pending = new Map<string, { revision: number; value: unknown }>();

  constructor(initial: Record<string, unknown> = {}) {
    this.confirmed = { ...initial };
  }

  begin(changes: Record<string, unknown>): SettingEditToken[] {
    return Object.entries(changes).map(([name, value]) => {
      const revision = (this.revisions.get(name) ?? 0) + 1;
      this.revisions.set(name, revision);
      this.pending.set(name, { revision, value });
      return { name, revision };
    });
  }

  reconcile(confirmed: Record<string, unknown>): Record<string, unknown> {
    this.confirmed = { ...confirmed };
    return this.snapshot();
  }

  settle(tokens: SettingEditToken[]): Record<string, unknown> {
    for (const { name, revision } of tokens) {
      if (this.pending.get(name)?.revision === revision) this.pending.delete(name);
    }
    return this.snapshot();
  }

  snapshot(): Record<string, unknown> {
    const values = { ...this.confirmed };
    for (const [name, edit] of this.pending) values[name] = edit.value;
    return values;
  }
}
