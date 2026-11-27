export class GenerationGuard {
  private generation = 0;
  private disposed = false;

  current(): number {
    return this.generation;
  }

  invalidate(): void {
    this.generation++;
  }

  dispose(): void {
    this.disposed = true;
    this.invalidate();
  }

  isCurrent(generation: number): boolean {
    return !this.disposed && generation === this.generation;
  }
}
