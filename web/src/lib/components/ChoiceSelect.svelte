<script lang="ts">
  /** Select over a fixed set of choices; shows a disabled placeholder when `value` is not one of them. */
  let {
    value,
    choices,
    onChange,
    label = (choice) => choice,
    placeholder = "Select a supported option",
    ariaLabel,
  }: {
    value: string;
    choices: readonly string[];
    onChange: (value: string) => void;
    label?: (choice: string) => string;
    placeholder?: string;
    ariaLabel?: string;
  } = $props();
</script>

<select aria-label={ariaLabel} {value} onchange={(event) => onChange(event.currentTarget.value)}>
  {#if !choices.includes(value)}<option value="" disabled>{placeholder}</option>{/if}
  {#each choices as choice}<option value={choice}>{label(choice)}</option>{/each}
</select>
