<script lang="ts">
  // „Sequence By Color“ (Menü Anordnen): Farbfolge wählen, Überdeckung beachten (eigene Erweiterung, Analyse_Teil3/16),
  // Vorschau der Farbwechsel vorher → nachher aus dem Kern (`reihenfolge_aendern`, ohne zu übernehmen).
  import { kern, type DesignJson } from "$kern/kern";
  import { arbeit } from "../arbeit.svelte";
  import Dialog from "../Dialog.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { garnText } from "../listen.svelte";

  let { schliessen }: { schliessen: () => void } = $props();

  // Farben der Objekte in der ersten Reihenfolge ihres Auftretens
  const start = arbeit.objekte.map((o) => (o.farbe ?? "#000000").toLowerCase()).filter((f, k, a) => a.indexOf(f) === k);
  let farben = $state<string[]>(start);
  let ueberdeckung = $state(true);
  let nurAuswahl = $state(arbeit.auswahl.length > 1);
  let vorschau = $state<{ vorher: number; nachher: number; warnungen: number } | null>(null);

  function garnZu(farbe: string) {
    const e = arbeit.ergebnis;
    const k = e?.farben.findIndex((f) => f.toLowerCase() === farbe) ?? -1;
    return k >= 0 ? e?.garne[k] : undefined;
  }
  const befehl = $derived({
    art: "nach_farbe" as const,
    farben,
    ueberdeckung,
    auswahl: nurAuswahl ? [...arbeit.auswahl] : [],
  });

  $effect(() => {
    const b = befehl;
    const design = $state.snapshot(arbeit.design) as DesignJson;
    kern("reihenfolge_aendern", { design, befehl: b })
      .then((r) => (vorschau = { vorher: r.farbwechsel_vorher, nachher: r.farbwechsel_nachher, warnungen: r.vertauscht.length }))
      .catch(() => (vorschau = null));
  });

  function schieben(k: number, d: number) {
    const f = [...farben];
    const [x] = f.splice(k, 1);
    f.splice(k + d, 0, x);
    farben = f;
  }
</script>

<Dialog titel="Nach Farbe sortieren" breite={480} {schliessen}>
  <p class="mut small" style="margin:0 0 10px">Objekte gleicher Farbe nacheinander sticken – weniger Garnwechsel. Reihenfolge der Farben:</p>
  <ol class="ol" style="border:1px solid var(--line); margin-bottom:10px">
    {#each farben as f, k (f)}
      {@const g = garnZu(f)}
      <li style="display:grid; grid-template-columns:22px 20px minmax(0,1fr) auto; gap:8px; align-items:center; padding:6px 10px; border-bottom:1px solid color-mix(in srgb,var(--color-text) 7%,transparent)">
        <span class="mut tab">{k + 1}</span><span class="sw" style="background:{g?.rgb ?? f}"></span><span class="small">{g ? garnText(g) : f}</span>
        <span>
          <button class="tb sm" aria-label="nach oben" disabled={k === 0} onclick={() => schieben(k, -1)}>↑</button>
          <button class="tb sm" aria-label="nach unten" disabled={k === farben.length - 1} onclick={() => schieben(k, 1)}>↓</button>
        </span>
      </li>
    {/each}
  </ol>
  <label class="chk" style="margin-bottom:6px"><input type="checkbox" bind:checked={ueberdeckung} /><span><b style="font-weight:500">Überdeckung beachten</b> <span class="mut small">– Objekte, die sich überdecken, behalten ihre Lage (eigene Erweiterung; das Original sortiert ohne Prüfung)</span></span></label>
  {#if arbeit.auswahl.length > 1}
    <label class="chk" style="margin-bottom:6px"><input type="checkbox" bind:checked={nurAuswahl} /><span>nur die {arbeit.auswahl.length} gewählten Objekte</span></label>
  {/if}
  {#if vorschau}
    <div class={vorschau.warnungen ? "warnbox" : "okbox"}>
      <Icon d={vorschau.warnungen ? IC.warnung : IC.haken} />
      <span>Farbwechsel {vorschau.vorher} → {vorschau.nachher}{vorschau.warnungen ? ` · ${vorschau.warnungen} ${vorschau.warnungen === 1 ? "Objekt käme" : "Objekte kämen"} unter eine Fläche` : ""}</span>
    </div>
  {/if}
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    <button class="btn btn-primary" onclick={() => (schliessen(), arbeit.umordnen(befehl))}>Sortieren</button>
  {/snippet}
</Dialog>
