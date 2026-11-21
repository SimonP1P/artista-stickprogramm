<script lang="ts">
  // Objektliste wie die „Color Film“ des Originals (UEBERGABE §3): Nr, Farbe, Art, Name · Garn; Ziehen ändert die
  // Stickreihenfolge (`reihenfolge_aendern` vor/nach), Marke „über N“ aus `ueberdeckungen`, „Fehler“ am Objekt.
  import { arbeit } from "./arbeit.svelte";
  import Icon from "./Icon.svelte";
  import { ART, IC } from "./icons";

  let { anordnen }: { anordnen: () => void } = $props();

  let ziehe = $state<number | null>(null);
  let ziel = $state<number | null>(null);

  const e = $derived(arbeit.ergebnis);
  function garnVon(i: number) {
    const g = e ? arbeit.garnVon(i) : undefined;
    return g ? `${g.nummer} ${g.name}` : "";
  }
  /** Objekte, über denen i liegt (später gestickt, überdeckt) */
  function ueber(i: number) {
    return arbeit.ueberdeckungen.filter(([, spaeter]) => spaeter === i).map(([frueher]) => frueher);
  }
  const titel = (o: { name?: string; art: string }, i: number) =>
    o.name || ("text" in o ? String((o as { text?: string }).text ?? "") : "") || ART[o.art]?.[0] || `Nr. ${i + 1}`;

  function fallen(i: number) {
    const von = ziehe;
    ziehe = null;
    ziel = null;
    if (von === null || von === i) return;
    const auswahl = arbeit.auswahl.includes(von) ? [...arbeit.auswahl] : [von];
    // nach unten gezogen: hinter das Ziel, nach oben: davor
    arbeit.umordnen(von < i ? { art: "nach", auswahl, ziel: i } : { art: "vor", auswahl, ziel: i });
  }
</script>

<aside class="pane" aria-label="Objekte" style="width:268px; border-right:1px solid var(--line)">
  <div class="pane-h">
    <h2 class="hd" style="font-size:17px; margin:0">Objekte</h2>
    <span class="badge">{arbeit.objekte.length}</span>
    <span style="flex:1"></span>
    <button class="tb sm" aria-label="Duplizieren" title="Duplizieren (Strg+D)" disabled={!arbeit.auswahl.length} onclick={() => arbeit.duplizieren()}><Icon d={IC.kopieren} groesse="s" /></button>
    <button class="tb sm" aria-label="Löschen" title="Löschen (Entf)" disabled={!arbeit.auswahl.length} onclick={() => arbeit.loeschen()}><Icon d={IC.papierkorb} groesse="s" /></button>
  </div>
  <div class="small mut" style="display:grid; grid-template-columns:12px 22px 16px 18px minmax(0,1fr); gap:6px; padding:5px 8px 5px 7px; border-bottom:1px solid var(--line)">
    <span></span><span>Nr</span><span></span><span>Art</span><span>Name · Garn</span>
  </div>
  <ol class="ol scroll" role="listbox" aria-label="Stickreihenfolge" aria-multiselectable="true">
    {#each arbeit.objekte as o, i (i)}
      {@const art = ART[o.art] ?? [o.art, ""]}
      {@const ue = ueber(i)}
      <li role="option" aria-selected={arbeit.auswahl.includes(i)}>
        <button
          class="orow"
          class:on={arbeit.auswahl.includes(i)}
          class:err={arbeit.fehler?.objekt === i}
          class:drop={ziel === i && ziehe !== i}
          draggable="true"
          ondragstart={(ev) => {
            ziehe = i;
            ev.dataTransfer?.setData("text/plain", String(i));
          }}
          ondragover={(ev) => {
            ev.preventDefault();
            ziel = i;
          }}
          ondrop={(ev) => {
            ev.preventDefault();
            fallen(i);
          }}
          ondragend={() => ((ziehe = null), (ziel = null))}
          onclick={(ev) => arbeit.waehlen(i, ev.shiftKey || ev.ctrlKey || ev.metaKey)}
          aria-label="Objekt {i + 1}: {art[0]}, {titel(o, i)}"
        >
          <span class="grip"><Icon d={IC.griff} groesse="s" /></span>
          <span class="tab mut" style="text-align:right">{i + 1}</span>
          <span class="sw" style="background:{o.farbe ?? '#000000'}"></span>
          <span title={art[0]}><Icon d={art[1]} /></span>
          <span class="ostack"><b>{titel(o, i)}</b><span>{art[0]}{garnVon(i) ? ` · ${garnVon(i)}` : ""}</span></span>
          <span style="display:flex; flex-direction:column; gap:2px; align-items:flex-end">
            {#if arbeit.fehler?.objekt === i}<span class="badge err">Fehler</span>{/if}
            {#if ue.length}<span class="badge" title="liegt über {ue.map((x) => `Nr. ${x + 1}`).join(', ')}">über {ue.map((x) => x + 1).join(", ")}</span>{/if}
          </span>
        </button>
      </li>
    {:else}
      <li class="mut small" style="padding:12px">Noch keine Objekte – Einfügen oder Werkzeug links der Vorschau.</li>
    {/each}
  </ol>
  <div style="flex:none; padding:8px 12px; border-top:1px solid var(--line); font-size:12px" class="mut">
    <div style="display:flex; justify-content:space-between">
      <span>{e?.farben.length ?? 0} Farben · {e?.statistik.farbwechsel ?? 0} Farbwechsel</span>
      <button class="btn btn-ghost" style="font-size:12px; padding:0 4px" onclick={anordnen}>Anordnen …</button>
    </div>
    <div>Ziehen = Stickreihenfolge ändern · Alt+↑/↓</div>
  </div>
</aside>
