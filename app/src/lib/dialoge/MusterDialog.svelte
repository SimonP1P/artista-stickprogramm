<script lang="ts">
  // Füllmuster (242, `fuellmuster_liste`) bzw. Step-Muster 1–20 (`step_muster_liste`) wählen – mit Vorschau-Kacheln aus
  // den Daten des Kerns: Füllmuster = Linien einer Kachel, wiederholt um i·A + j·B (UEBERGABE §3); Step-Muster =
  // Einstiche einer Tatami-Probefläche, berechnet vom Kern.
  import { kern, type StepMusterInfo } from "$kern/kern";
  import Dialog from "../Dialog.svelte";
  import { listen } from "../listen.svelte";

  let {
    art,
    aktuell,
    waehlen,
    schliessen,
  }: { art: "muster" | "step"; aktuell?: string | number | null; waehlen: (w: string | number | null) => void; schliessen: () => void } =
    $props();

  let suche = $state("");
  let step = $state.raw<StepMusterInfo[]>([]);
  $effect(() => {
    if (art === "muster") listen.musterLaden();
    else kern("step_muster_liste", {}).then((s) => (step = s)).catch(() => {});
  });
  const muster = $derived(
    listen.muster.filter((m) => m.name.toLowerCase().includes(suche.trim().toLowerCase())),
  );
  const VERSATZ = [-2, -1, 0, 1, 2];
  function box(a: number[], b: number[]) {
    const s = Math.max(Math.hypot(a[0], a[1]), Math.hypot(b[0], b[1])) * 2;
    return `${-s / 2} ${-s / 2} ${s} ${s}`;
  }
</script>

<Dialog titel={art === "muster" ? "Füllmuster wählen" : "Step-Muster wählen"} breite={820} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">{art === "muster" ? "Füllmuster wählen" : "Step-Muster wählen"}</h2>
    {#if art === "muster"}
      <span class="badge">{muster.length} von {listen.muster.length}</span>
      <span style="flex:1"></span>
      <input class="input sm" placeholder="Suchen …" aria-label="Füllmuster suchen" bind:value={suche} style="width:200px" />
    {/if}
  {/snippet}
  {#if art === "muster"}
    <div style="display:grid; grid-template-columns:repeat(6,minmax(0,1fr)); gap:8px">
      {#each muster as m (m.name)}
        <button class="mtile" class:on={aktuell === m.name || aktuell === m.nummer} onclick={() => waehlen(m.name)} title={m.name}>
          <svg viewBox={box(m.a, m.b)} aria-hidden="true">
            {#each VERSATZ as ii (ii)}
              {#each VERSATZ as jj (jj)}
                <g transform="translate({ii * m.a[0] + jj * m.b[0] - (m.a[0] + m.b[0]) / 2} {-(ii * m.a[1] + jj * m.b[1]) + (m.a[1] + m.b[1]) / 2})">
                  {#each m.zuege as z, k (k)}
                    <polyline points={z.map(([x, y]) => `${x},${-y}`).join(" ")} fill="none" stroke="currentColor" stroke-width={Math.hypot(m.a[0], m.a[1]) / 40} />
                  {/each}
                </g>
              {/each}
            {/each}
          </svg>
          <span style="white-space:nowrap; overflow:hidden; text-overflow:ellipsis">{m.name}</span>
        </button>
      {/each}
    </div>
  {:else}
    <p class="mut small" style="margin:0 0 10px">Struktur der Tatami-Fläche: Versatz der Reihen (Tabelle aus dem Original, DIALOG.DLL). Kacheln vom Kern berechnet (12 × 8 mm, Software Defaults).</p>
    <div style="display:grid; grid-template-columns:repeat(5,minmax(0,1fr)); gap:8px">
      <button class="mtile" class:on={aktuell == null} onclick={() => waehlen(null)}>
        <svg viewBox="0 0 12 8" aria-hidden="true"><text x="6" y="4.6" font-size="1.6" text-anchor="middle" fill="currentColor">wie Stoff/Stil</text></svg>
        <span>ohne Angabe</span>
      </button>
      {#each step as s (s.nr)}
        <button class="mtile" class:on={aktuell === s.nr} onclick={() => waehlen(s.nr)}>
          <svg viewBox="-0.3 -8.3 12.6 8.6" aria-hidden="true">
            <polyline points={s.einstiche.map(([x, y]) => `${x},${-y}`).join(" ")} fill="none" stroke="currentColor" stroke-width=".06" opacity=".5" />
            {#each s.einstiche as [x, y], k (k)}<circle cx={x} cy={-y} r=".11" fill="currentColor" />{/each}
          </svg>
          <span>Nr. {s.nr}{s.nr === 1 ? " · gleichmäßig" : ""}</span>
        </button>
      {/each}
    </div>
  {/if}
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
  {/snippet}
</Dialog>
