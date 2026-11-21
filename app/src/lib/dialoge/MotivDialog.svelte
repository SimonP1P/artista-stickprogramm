<script lang="ts">
  // Motiv für eine Motivlinie wählen (Handbuch Kap. 25 „Select Pattern“): Motivsatz (`motive_liste`), dann Motiv mit
  // Vorschaubild aus den Linien des Kerns (`motivsatz_motive`, mm, Grundlinie von [0, 0] nach [laenge, 0]).
  import { kern, type MotivInfo, type MotivsatzInfo } from "$kern/kern";
  import Dialog from "../Dialog.svelte";

  let {
    satz,
    motiv,
    waehlen,
    schliessen,
  }: { satz: string; motiv: string; waehlen: (satz: string, motiv: string) => void; schliessen: () => void } = $props();

  let saetze = $state.raw<MotivsatzInfo[]>([]);
  // svelte-ignore state_referenced_locally
  let gewaehlt = $state(satz);
  let motive = $state.raw<MotivInfo[]>([]);
  let fehler = $state("");
  $effect(() => {
    kern("motive_liste", {}).then((s) => (saetze = s)).catch((e) => (fehler = String(e?.meldung ?? e)));
  });
  $effect(() => {
    const s = gewaehlt;
    motive = [];
    kern("motivsatz_motive", { satz: s })
      .then((m) => {
        if (gewaehlt === s) motive = m;
      })
      .catch((e) => (fehler = String(e?.meldung ?? e)));
  });
  function box(m: MotivInfo) {
    const pts = m.linien.flat();
    if (!pts.length) return "0 0 1 1";
    const xs = pts.map((q) => q[0]);
    const ys = pts.map((q) => -q[1]);
    const [x0, x1, y0, y1] = [Math.min(...xs, 0), Math.max(...xs, m.laenge), Math.min(...ys), Math.max(...ys)];
    const s = Math.max(x1 - x0, y1 - y0) * 1.15 || 1;
    return `${(x0 + x1) / 2 - s / 2} ${(y0 + y1) / 2 - s / 2} ${s} ${s}`;
  }
</script>

<Dialog titel="Motiv wählen" breite={820} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">Motiv wählen</h2>
    <span class="badge">{motive.length} Motive</span>
    <span style="flex:1"></span>
    <select class="input sm" aria-label="Motivsatz" bind:value={gewaehlt} style="width:200px">
      {#if !saetze.some((s) => s.name === gewaehlt)}<option value={gewaehlt}>{gewaehlt}</option>{/if}
      {#each saetze as s (s.datei)}<option value={s.name}>{s.name.replace(/^_/, "")}</option>{/each}
    </select>
  {/snippet}
  {#if fehler}<p class="small" style="color:var(--danger, #b00)">{fehler}</p>{/if}
  <p class="mut small" style="margin:0 0 10px">Motivsätze der CD. Die Linie (gestrichelt) ist der Wiederholabstand; der Kern verteilt die Motive gleichmäßig wie das Original.</p>
  <div style="display:grid; grid-template-columns:repeat(6,minmax(0,1fr)); gap:8px">
    {#each motive as m (m.name)}
      <button class="mtile" class:on={gewaehlt === satz && motiv === m.name} onclick={() => waehlen(gewaehlt, m.name)} title="{m.name} · {m.laenge.toFixed(1)} × {m.hoehe.toFixed(1)} mm">
        <svg viewBox={box(m)} aria-hidden="true">
          <line x1="0" y1="0" x2={m.laenge} y2="0" stroke="currentColor" stroke-width={m.laenge / 120} stroke-dasharray="{m.laenge / 30} {m.laenge / 30}" opacity=".35" />
          {#each m.linien as l, k (k)}
            <polyline points={l.map(([x, y]) => `${x},${-y}`).join(" ")} fill="none" stroke="currentColor" stroke-width={Math.max(m.laenge, m.hoehe) / 70} />
          {/each}
        </svg>
        <span style="white-space:nowrap; overflow:hidden; text-overflow:ellipsis">{m.name}</span>
      </button>
    {/each}
  </div>
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
  {/snippet}
</Dialog>
