<script lang="ts">
  // Reiter „JSON“ (Expertenmodus, UEBERGABE §6.5): gewähltes Objekt bzw. ganzes Design als Text; beim Tippen prüft der
  // Kern (`design_pruefen`), „Übernehmen“ ersetzt Objekt bzw. Design. So ist jede neue Kern-Funktion sofort nutzbar.
  import { untrack } from "svelte";
  import { kern, type DesignJson, type ObjektJson, type Pruefung } from "$kern/kern";
  import { arbeit, alsFehler } from "../arbeit.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";

  const i = $derived(arbeit.auswahl.length === 1 ? arbeit.auswahl[0] : -1);
  const quelle = $derived(JSON.stringify(i >= 0 ? arbeit.objekte[i] : arbeit.design, null, 2));

  let text = $state("");
  let pruefung = $state.raw<Pruefung | null>(null);
  let syntax = $state<string | null>(null);
  let geaendert = $state(false);
  let zeit: ReturnType<typeof setTimeout> | null = null;

  // neuer Stand aus dem Design, solange nicht getippt wurde
  $effect(() => {
    const q = quelle;
    if (!untrack(() => geaendert)) text = q;
  });
  // anderes Objekt gewählt: von vorn
  $effect(() => {
    void i;
    untrack(() => {
      geaendert = false;
      text = quelle;
      pruefung = null;
      syntax = null;
    });
  });

  function kandidat(): DesignJson | null {
    try {
      const v = JSON.parse(text);
      syntax = null;
      if (i < 0) return v as DesignJson;
      const d = JSON.parse(JSON.stringify($state.snapshot(arbeit.design))) as DesignJson;
      d.objekte![i] = v as ObjektJson;
      return d;
    } catch (x) {
      syntax = (x as Error).message;
      return null;
    }
  }

  function tippen(v: string) {
    text = v;
    geaendert = true;
    if (zeit) clearTimeout(zeit);
    zeit = setTimeout(async () => {
      const d = kandidat();
      if (!d) return (pruefung = null);
      try {
        pruefung = await kern("design_pruefen", { design: d });
      } catch (x) {
        pruefung = { fehler: alsFehler(x), hinweise: [] };
      }
    }, 300);
  }

  function uebernehmen() {
    const d = kandidat();
    if (!d) return;
    arbeit.ersetzen(d, arbeit.pfad, true);
    geaendert = false;
  }
</script>

<div style="padding:10px 14px; display:flex; flex-direction:column; gap:8px; height:100%; box-sizing:border-box">
  <span class="mut small">{i >= 0 ? `Objekt Nr. ${i + 1}` : "Ganzes Design (kein Objekt gewählt)"} – Felder: SCHNITTSTELLE.md</span>
  <textarea
    class="input"
    spellcheck="false"
    style="flex:1; min-height:360px; font-family:ui-monospace, Menlo, Consolas, monospace; font-size:12px; white-space:pre"
    value={text}
    oninput={(ev) => tippen((ev.currentTarget as HTMLTextAreaElement).value)}
  ></textarea>
  {#if syntax}
    <div class="errbox"><Icon d={IC.fehler} /><span>JSON: {syntax}</span></div>
  {:else if pruefung?.fehler}
    <div class="errbox"><Icon d={IC.fehler} /><span>{pruefung.fehler.meldung}</span></div>
  {:else if pruefung}
    <div class="okbox"><Icon d={IC.haken} /><span>Geprüft – in Ordnung{pruefung.hinweise.length ? ` · ${pruefung.hinweise.length} Hinweis(e)` : ""}</span></div>
  {/if}
  {#each pruefung?.hinweise ?? [] as h (h.meldung)}<div class="warnbox small"><Icon d={IC.warnung} /><span>{h.meldung}</span></div>{/each}
  <div style="display:flex; gap:8px; justify-content:flex-end">
    <button class="btn btn-secondary" disabled={!geaendert} onclick={() => ((geaendert = false), (text = quelle), (pruefung = null))}>Verwerfen</button>
    <button class="btn btn-primary" disabled={!geaendert || !!syntax || !!pruefung?.fehler} onclick={uebernehmen}>Übernehmen</button>
  </div>
</div>
