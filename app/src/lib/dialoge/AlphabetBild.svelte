<script lang="ts">
  // Vorschau eines Alphabets: Umrisse aus dem Kern (`alphabet_vorschau`), eingepasst über die viewBox.
  import type { AlphabetVorschau } from "$kern/kern";
  import { listen } from "../listen.svelte";

  let { name }: { name: string } = $props();
  let v = $state.raw<AlphabetVorschau | null>(null);
  let fehler = $state(false);

  $effect(() => {
    v = null;
    fehler = false;
    const n = name;
    listen
      .alphabetVorschau(n)
      .then((x) => n === name && (v = x))
      .catch(() => (fehler = true));
  });

  const box = $derived.by(() => {
    const b = v?.begrenzung;
    if (!b) return "0 0 1 1";
    const r = 0.8;
    return `${b[0] - r} ${-b[3] - r} ${b[2] - b[0] + 2 * r} ${b[3] - b[1] + 2 * r}`;
  });
</script>

{#if v}
  <svg viewBox={box} preserveAspectRatio="xMidYMid meet" style="width:100%; height:100%" aria-hidden="true">
    {#each v.linien.filter((l) => l.art !== "winkellinie") as l, k (k)}
      <path
        d={l.punkte.map(([x, y], j) => `${j ? "L" : "M"}${x.toFixed(2)} ${(-y).toFixed(2)}`).join("") + (l.geschlossen ? "Z" : "")}
        fill="none"
        stroke="currentColor"
        stroke-width="0.25"
      />
    {/each}
  </svg>
{:else}
  <span class="mut small">{fehler ? "–" : "…"}</span>
{/if}
