<script lang="ts">
  // Designs aus der Maschine (nur lesen): Liste (`maschine_liste`), eines lesen (`maschine_lesen`, Befehl 0x400) und
  // mit den Garnfarben der Maschine ansehen, als EXP speichern (`gelesen_exp_speichern`).
  import { save } from "@tauri-apps/plugin-dialog";
  import { kern } from "$kern/kern";
  import { arbeit, alsFehler } from "../arbeit.svelte";
  import Dialog from "../Dialog.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { maschine, SIMULATION } from "../maschine.svelte";
  import StichBild from "../StichBild.svelte";
  import { ganz, zahlFest } from "../zahl";

  let gewaehlt = $state<number | null>(null);
  const lesend = $derived(maschine.laeuft && maschine.fortschritt["DATEN"]);

  async function expSpeichern() {
    const g = maschine.gelesen;
    if (!g) return;
    const p = await save({ filters: [{ name: "Stickdatei (EXP)", extensions: ["exp"] }], defaultPath: `Maschine_Nr${String(g.nr).padStart(2, "0")}.exp` });
    if (!p) return;
    try {
      const n = await kern("gelesen_exp_speichern", { pfad: p });
      arbeit.zeige(`EXP gespeichert (${ganz(n)} Byte)`, "ok");
    } catch (x) {
      arbeit.zeige(alsFehler(x).meldung, "err");
    }
  }
</script>

<Dialog titel="Designs aus der Maschine" breite={780} schliessen={() => (maschine.listeOffen = false)}>
  {#snippet kopf()}
    <h2 class="hd">Designs aus der Maschine</h2>
    {#if maschine.liste}<span class="badge">{maschine.liste.designs.length} gespeichert</span>{/if}
  {/snippet}
  {#if maschine.simulation}
    <div class="simbox" style="margin-bottom:10px"><Icon d={IC.monitor} /><span><b>Simulation:</b> die Designs kommen aus der nachgebauten Maschine.</span></div>
  {/if}
  <div style="display:flex; gap:8px; align-items:end; margin-bottom:10px">
    <div class="field" style="flex:1">
      <label for="aport">Anschluss</label>
      <select id="aport" class="input" value={maschine.port} onchange={(e) => maschine.portSetzen((e.currentTarget as HTMLSelectElement).value)}>
        {#each maschine.ports as p (p)}<option value={p}>{p}</option>{/each}
        <option value={SIMULATION}>Simulation (ohne Maschine)</option>
      </select>
    </div>
    <button class="btn btn-primary" disabled={maschine.laeuft} onclick={() => maschine.listeLesen()}>{maschine.laeuft && !lesend ? "Lese …" : "Liste lesen (nur lesen)"}</button>
  </div>
  {#if maschine.listeFehler}<div class="errbox" style="margin-bottom:10px"><Icon d={IC.fehler} /><span>{maschine.listeFehler.meldung}</span></div>{/if}
  <div style="display:grid; grid-template-columns:300px minmax(0,1fr); gap:12px; height:380px">
    <div class="scroll" style="border:1px solid var(--line)">
      {#each maschine.liste?.designs ?? [] as d (d.nr)}
        <button class="mrow" class:on={gewaehlt === d.nr} aria-pressed={gewaehlt === d.nr} disabled={maschine.laeuft} onclick={() => ((gewaehlt = d.nr), maschine.designLesen(d.nr))}>
          <span class="tab mut">{d.nr}</span><span style="white-space:nowrap; overflow:hidden; text-overflow:ellipsis">{d.name}</span>
        </button>
      {:else}
        <p class="mut small" style="padding:10px">Noch keine Liste – „Liste lesen“.</p>
      {/each}
    </div>
    <div style="display:flex; flex-direction:column; gap:8px; min-height:0">
      {#if lesend}
        {@const f = maschine.fortschritt["DATEN"]}
        <p class="mut">Lese Nr. {gewaehlt} … {ganz(f.erledigt)} / {ganz(f.gesamt)} Byte{f.wiederholungen ? ` · ${f.wiederholungen} × wiederholt` : ""}</p>
        <button class="btn btn-secondary" onclick={() => maschine.abbrechen()}>Abbrechen</button>
      {:else if maschine.gelesen}
        {@const g = maschine.gelesen}
        <div>
          <b style="font-weight:500">{g.ergebnis.name}</b>
          <div class="mut small tab">{ganz(g.ergebnis.statistik.stiche)} Stiche · {g.ergebnis.farben.length} {g.ergebnis.farben.length === 1 ? "Farbe" : "Farben"} · {zahlFest(g.ergebnis.statistik.breite)} × {zahlFest(g.ergebnis.statistik.hoehe)} mm · {ganz(g.bytes)} Byte</div>
        </div>
        <div style="flex:1; min-height:0; border:1px solid var(--line); background:var(--panel); padding:6px"><StichBild ergebnis={g.ergebnis} /></div>
        <div class="note small">Nur Ansicht mit den Garnfarben der Maschine – kein Bearbeiten. Als EXP sichern, um es aufzuheben.</div>
      {:else}
        <p class="mut" style="margin:auto; text-align:center">Design links wählen – es wird nur gelesen (≈ 1,6 KB/s).</p>
      {/if}
    </div>
  </div>
  {#snippet fuss()}
    <button class="btn btn-secondary" style="margin-right:auto" onclick={() => (maschine.listeOffen = false)}>Schließen</button>
    <button class="btn btn-primary" disabled={!maschine.gelesen || maschine.laeuft} onclick={expSpeichern}><Icon d={IC.herunterladen} />Als EXP speichern …</button>
  {/snippet}
</Dialog>
