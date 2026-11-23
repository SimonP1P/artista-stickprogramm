<script lang="ts">
  // Übertragen mit einem Klick (Entscheidung des Nutzers 22.11.2026, `UI_Mockups/UX_ANALYSE_2026-11-22.md` S1–S8; wie
  // das Original „Write to Machine“ → „Writing file(s) to device“, Handbuch S. 265–266): ein Balken, danach „Fertig“
  // mit der Farbfolge. Technisches nur unter „Details“; Fehler mit nächsten Schritten und „Nochmal“.
  import { arbeit } from "../arbeit.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { garnText } from "../listen.svelte";
  import { maschine, naechsteSchritte, SIMULATION } from "../maschine.svelte";
  import { zufallsSpruch } from "../sprueche";
  import { untrack } from "svelte";

  // Zufallsspruch am Ende (Wunsch Nutzer; die Meldung am Ende des Stickens zeigt die Maschine selbst)
  let spruch = $state("");
  $effect(() => {
    if (maschine.phase === "fertig") untrack(() => (spruch = zufallsSpruch(spruch)));
  });

  const stand = $derived(maschine.fortschritt["Übertragung"]);
  const prozent = $derived(stand ? Math.min(100, Math.round((stand.erledigt / Math.max(1, stand.gesamt)) * 100)) : 0);
  const garne = $derived(arbeit.ergebnis?.garne ?? []);
  const b = $derived(maschine.bericht);
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && !maschine.laeuft && (e.stopPropagation(), maschine.schliessen())} />

<div class="dlg-bg" role="presentation">
  <div class="dlg" role="dialog" aria-modal="true" aria-label="An die Maschine übertragen" style="width:520px; max-height:calc(100vh - 40px)">
    <div class="dlg-h">
      <Icon d={IC.senden} groesse="l" />
      <h2 class="hd">An die Maschine übertragen</h2>
      <span class="mut small" style="overflow:hidden; text-overflow:ellipsis; white-space:nowrap">„{arbeit.design.name ?? "Design"}“</span>
      <span style="flex:1"></span>
      {#if !maschine.laeuft}<button class="tb" aria-label="Schließen" onclick={() => maschine.schliessen()}><Icon d={IC.kreuz} /></button>{/if}
    </div>

    <div class="dlg-b">
      {#if maschine.simulation}
        <div class="simbox" style="margin-bottom:12px"><Icon d={IC.monitor} /><span><b>Simulation</b> – es wird nichts gesendet. Anschluss oben im Menü „Simulation“ wählen.</span></div>
      {/if}

      {#if maschine.phase === "suchen" || maschine.phase === "laeuft"}
        <div style="display:flex; align-items:baseline; gap:10px; margin-bottom:6px">
          <span class="hd tab" style="font-size:30px">{prozent} %</span>
          <span class="mut">{maschine.phase === "suchen" ? "Suche die Maschine …" : stand ? "Übertrage …" : "Verbinde …"}</span>
        </div>
        <div style="height:10px; background:color-mix(in srgb,var(--color-text) 10%,transparent)"><div style="height:10px; width:{prozent}%; background:var(--color-accent); transition:width .3s"></div></div>
        {#if stand?.wiederholungen}
          <p class="mut small" style="margin:8px 0 0">Kleine Übertragungsfehler werden automatisch wiederholt ({stand.wiederholungen} ×).</p>
        {/if}
      {:else if maschine.phase === "fertig" && b}
        {#if b.gestickt === false}
          <div class="warnbox" style="font-size:14px"><Icon d={IC.warnung} /><span><b style="font-weight:600">Übertragen, aber die Maschine hat das Design nicht übernommen.</b> Bitte „Nochmal“.</span></div>
        {:else}
          <div class="okbox" style="font-size:14px"><Icon d={IC.haken} /><span><b style="font-weight:600">Fertig – die Maschine ist bereit.</b> Gestickt wird mit Start an der Maschine.</span></div>
        {/if}
        {#if spruch}
          <button
            class="btn btn-ghost"
            style="display:block; width:100%; text-align:left; margin:8px 0 0; padding:8px 11px; font-size:14px; font-style:italic; color:var(--color-accent); white-space:normal"
            title="Noch ein Spruch"
            onclick={() => (spruch = zufallsSpruch(spruch))}>„{spruch}“</button>
        {/if}
        {#if maschine.rahmenNeu}
          <div class="note" style="margin-top:10px">Rahmen gewechselt? An der Maschine einmal «check» drücken.</div>
        {/if}
        {#each b.hinweise as h (h)}<div class="warnbox" style="margin-top:8px"><Icon d={IC.warnung} /><span>{h}</span></div>{/each}
        {#if garne.length > 1}
          <div class="note" style="margin-top:10px">
            <b style="font-weight:500">Garne in dieser Reihenfolge:</b>
            <ol style="margin:6px 0 0; padding-left:20px">
              {#each garne as g, k (k)}<li><span class="sw" style="display:inline-block; vertical-align:-2px; margin-right:6px; background:{g.rgb}"></span>{garnText(g)}</li>{/each}
            </ol>
          </div>
        {/if}
        <details class="small mut" style="margin-top:10px">
          <summary>Details</summary>
          <ul style="margin:4px 0 0; padding-left:18px">
            {#each b.schritte as s (s)}<li>{s}</li>{/each}
            <li>Anschluss {b.sitzung.port} · Firmware {b.sitzung.naehmaschine} · Stickmodul {b.sitzung.stickmodul}</li>
            <li>Mitschnitt: {b.sitzung.mitschnitt}</li>
          </ul>
        </details>
      {:else if maschine.phase === "fehler" && maschine.fehler}
        <div class="errbox" style="margin-bottom:12px; font-size:14px">
          <Icon d={IC.fehler} />
          <span><b style="font-weight:600">{maschine.fehler.code === "abgebrochen" ? "Abgebrochen." : "Übertragen hat nicht geklappt."}</b> {maschine.fehler.meldung.replace(/ \(Mitschnitt: [^)]*\)$/, "")}</span>
        </div>
        <ol style="margin:0 0 0; padding-left:20px">{#each naechsteSchritte(maschine.fehler) as s (s)}<li style="margin-bottom:3px">{s}</li>{/each}</ol>
        {#if maschine.fehler.code === "keine_verbindung" && maschine.ports.length > 1}
          <div class="field" style="margin-top:12px">
            <label for="sport">Anschluss</label>
            <select id="sport" class="input" value={maschine.port} onchange={(e) => maschine.portSetzen((e.currentTarget as HTMLSelectElement).value)}>
              {#each maschine.ports as p (p)}<option value={p}>{p}</option>{/each}
              <option value={SIMULATION}>Simulation (ohne Maschine)</option>
            </select>
          </div>
        {/if}
      {/if}
    </div>

    <div class="dlg-f">
      {#if maschine.laeuft}
        <button class="btn btn-secondary" onclick={() => maschine.abbrechen()}>Abbrechen</button>
      {:else if maschine.phase === "fertig" && b?.gestickt !== false}
        <button class="btn btn-primary" onclick={() => maschine.schliessen()}>Schließen</button>
      {:else}
        <button class="btn btn-secondary" style="margin-right:auto" onclick={() => maschine.schliessen()}>Schließen</button>
        {#if maschine.fehler?.code === "farbtabelle"}
          <button class="btn btn-secondary" onclick={() => maschine.erneut(true)}>Ohne Garnfarben</button>
        {/if}
        <button class="btn btn-primary" onclick={() => maschine.erneut()}>Nochmal</button>
      {/if}
    </div>
  </div>
</div>
