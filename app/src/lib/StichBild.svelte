<script lang="ts">
  // Kleines Stichbild eines Ergebnisses (z. B. Design aus der Maschine): Stiche je Farbblock, Sprünge unterbrochen.
  // Reine Darstellung der Stiche aus dem Kern.
  import type { Ergebnis } from "$kern/kern";

  let { ergebnis }: { ergebnis: Ergebnis } = $props();

  const pfade = $derived.by(() => {
    const out: { farbe: string; d: string }[] = [];
    let block = 0;
    let d = "";
    let neu = true;
    for (const s of ergebnis.stiche) {
      if (s.art === "farbwechsel") {
        if (d) out.push({ farbe: ergebnis.farben[block] ?? "#000", d });
        d = "";
        block++;
        neu = true;
      } else if (s.art === "sprung") neu = true;
      else {
        d += `${neu ? "M" : "L"}${s.x.toFixed(1)} ${(-s.y).toFixed(1)}`;
        neu = false;
      }
    }
    if (d) out.push({ farbe: ergebnis.farben[block] ?? "#000", d });
    return out;
  });
  const box = $derived.by(() => {
    const b = ergebnis.begrenzung ?? [-50, -50, 50, 50];
    const r = 3;
    return `${b[0] - r} ${-b[3] - r} ${b[2] - b[0] + 2 * r} ${b[3] - b[1] + 2 * r}`;
  });
</script>

<svg viewBox={box} preserveAspectRatio="xMidYMid meet" style="width:100%; height:100%" role="img" aria-label="Stichbild {ergebnis.name}">
  {#each pfade as p, k (k)}
    {@const n = parseInt(p.farbe.slice(1), 16)}
    {@const hell = (0.299 * (n >> 16) + 0.587 * ((n >> 8) & 255) + 0.114 * (n & 255)) / 255 > 0.85}
    {#if hell}<path d={p.d} fill="none" stroke="#8a8f95" stroke-width=".45" stroke-linejoin="round" />{/if}
    <path d={p.d} fill="none" stroke={p.farbe} stroke-width=".25" stroke-linejoin="round" />
  {/each}
</svg>
