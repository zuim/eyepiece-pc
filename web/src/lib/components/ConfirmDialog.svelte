<script lang="ts">
  /** A modal question with a confirm and a cancel button; `open` shows it. */
  let {
    open = $bindable(false),
    title,
    message,
    confirmLabel,
    onConfirm,
  }: {
    open: boolean;
    title: string;
    message: string;
    confirmLabel: string;
    onConfirm: () => void;
  } = $props();

  let dialog: HTMLDialogElement | undefined = $state();

  $effect(() => {
    if (!dialog) return;
    if (open && !dialog.open) dialog.showModal();
    if (!open && dialog.open) dialog.close();
  });

  function confirm() {
    open = false;
    onConfirm();
  }
</script>

<dialog bind:this={dialog} aria-labelledby="confirm-title" onclose={() => (open = false)}>
  <h2 id="confirm-title">{title}</h2>
  <p>{message}</p>
  <div class="actions">
    <button class="secondary" onclick={() => (open = false)}>Cancel</button>
    <button class="primary danger" onclick={confirm}>{confirmLabel}</button>
  </div>
</dialog>

<style>
  dialog {
    max-width: min(440px, calc(100vw - 32px));
    padding: 22px;
    border: 1px solid #3a4a45;
    border-radius: 14px;
    background: #151c1b;
    color: #e9efec;
  }
  dialog::backdrop {
    background: #000000a0;
  }
  h2 {
    margin: 0 0 8px;
    font-size: 17px;
  }
  p {
    margin: 0 0 18px;
    color: #a9b7b1;
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
  .danger {
    background: #e7a58f;
  }
  .danger:hover {
    background: #f2bba8;
  }
</style>
