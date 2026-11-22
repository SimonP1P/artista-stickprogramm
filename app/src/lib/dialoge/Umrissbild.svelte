<script lang="ts">
  // Gefüllte Buchstaben aus den Linien des Kerns (`alphabet_vorschau`, `schrift_vorschau`): Satinsäulen der
  // Original-Alphabete als Paare Seite A/B, geschlossene Umrisse der Computer-Schriften nach Nonzero-Regel, offene
  // Linien (Läufe) als Strich. Nur Anzeige – die Geometrie kommt aus dem Kern.
  import type { FormLinie } from "$kern/kern";
  import { formPfade } from "../formpfade";

  let { linien, begrenzung }: { linien: FormLinie[]; begrenzung?: number[] | null } = $props();

  const teile = $derived(formPfade(linien));

  const b = $derived(begrenzung && begrenzung.length === 4 ? begrenzung : null);
  const rand = $derived(b ? Math.max(b[2] - b[0], b[3] - b[1]) * 0.04 : 0);
  const box = $derived(b ? `${b[0] - rand} ${-b[3] - rand} ${b[2] - b[0] + 2 * rand} ${b[3] - b[1] + 2 * rand}` : "0 0 1 1");
  const strich = $derived(b ? (b[3] - b[1]) * 0.025 : 0.2);
</script>

<svg viewBox={box} preserveAspectRatio="xMidYMid meet" style="width:100%; height:100%" aria-hidden="true">
  {#each teile.saeulen as d, k (k)}<path {d} fill="currentColor" stroke="currentColor" stroke-width={strich * 0.3} stroke-linejoin="round" />{/each}
  {#if teile.flaechen}<path d={teile.flaechen} fill="currentColor" fill-rule="nonzero" />{/if}
  {#each teile.offen as d, k (k)}<path {d} fill="none" stroke="currentColor" stroke-width={strich} stroke-linecap="round" />{/each}
</svg>
