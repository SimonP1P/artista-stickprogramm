<script lang="ts">
  // Senden in Stufen (UEBERGABE §3 „Dialoge“, §7 Sicherheit): 1 Anschluss → 2 Plan prüfen (nur gelesen) → 3 Übertragen
  // (Fortschritt je Teil, Wiederholungen sichtbar, Abbrechen) → 4 Ergebnis (Farbfolge mit Garnen). Geschrieben wird nur
  // über „Bestätigen und schreiben“; Sticken nur über den eigenen Gefahrendialog.
  import { arbeit } from "../arbeit.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { garnText } from "../listen.svelte";
  import { ANLEITUNG_RAHMEN, maschine, naechsteSchritte, SIMULATION } from "../maschine.svelte";
  import { ganz } from "../zahl";
  import { zufallsSpruch } from "../sprueche";
  import { untrack } from "svelte";

  const STUFEN = ["1 Anschluss", "2 Plan prüfen", "3 Übertragen", "4 Ergebnis"];

  // Zufallsspruch im Ergebnis (Wunsch Nutzer; die Meldung am Ende des Stickens zeigt die Maschine selbst)
  let spruch = $state("");
  $effect(() => {
    if (maschine.phase === "fertig") untrack(() => (spruch = zufallsSpruch(spruch)));
  });
  const stufe = $derived(
    { anschluss: 0, plan: 1, laeuft: 2, fertig: 3, fehler: maschine.plan ? 2 : 0 }[maschine.phase],
  );

  const teile = $derived.by(() => {
    const p = maschine.plan;
    if (!p) return [];
    const l = [
      { was: "DATEN", label: "Design-Daten (Stiche)", bytes: p.bytes_daten },
      { was: "ICON", label: "Vorschaubild (Icon 72 × 62)", bytes: p.bytes_icon },
      { was: "INFO", label: "Name und Infos", bytes: 0 },
      { was: "Garnfarbtabelle", label: "Garnfarbtabelle", bytes: p.bytes_farben },
    ];
    return l.filter((t) => t.was === "INFO" || t.bytes > 0);
  });

  const gesamt = $derived(teile.reduce((s, t) => s + t.bytes, 0));
  const erledigt = $derived(
    teile.reduce((s, t) => s + Math.min(t.bytes, maschine.fortschritt[t.was]?.erledigt ?? 0), 0),
  );
  const prozent = $derived(gesamt ? Math.round((erledigt / gesamt) * 100) : 0);
  const wiederholungen = $derived(Object.values(maschine.fortschritt).reduce((s, f) => s + f.wiederholungen, 0));
  const garne = $derived(arbeit.ergebnis?.garne ?? []);

  function teilStand(was: string, bytes: number) {
    const f = maschine.fortschritt[was];
    if (!f) return { pct: 0, txt: maschine.phase === "laeuft" ? "wartet" : "" };
    const pct = Math.round((f.erledigt / Math.max(1, f.gesamt)) * 100);
    return {
      pct,
      txt: `${ganz(f.erledigt)} / ${ganz(f.gesamt || bytes)} Byte${f.wiederholungen ? ` · ${f.wiederholungen} × wiederholt` : ""}`,
    };
  }
</script>

<div class="dlg-bg" role="presentation">
  <div class="dlg" role="dialog" aria-modal="true" aria-label="An die Maschine senden" style="width:720px; min-height:440px; max-height:calc(100vh - 40px)">
    {#if maschine.simulation}
      <div class="simband" style="flex:none; padding:5px 16px; font-size:12.5px; font-weight:500">SIMULATION – keine Maschine. Alle Schritte werden nur nachgespielt.</div>
    {/if}
    <div class="dlg-h">
      <Icon d={IC.senden} groesse="l" />
      <h2 class="hd">An die Maschine senden</h2>
      <span class="mut small">„{arbeit.design.name ?? "Design"}“</span>
      <span style="flex:1"></span>
      <button class="tb" aria-label="Schließen" disabled={maschine.phase === "laeuft"} onclick={() => maschine.schliessen()}><Icon d={IC.kreuz} /></button>
    </div>
    <div style="flex:none; display:flex; padding:0 16px">
      {#each STUFEN as s, k (k)}
        <span class="sst" class:cur={k === stufe} class:done={k < stufe} class:bad={k === stufe && maschine.phase === "fehler"}>{s}</span>
      {/each}
    </div>

    <div class="dlg-b">
      {#if maschine.phase === "anschluss"}
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:14px">
          <div>
            <div class="field" style="margin-bottom:10px">
              <label for="sport">Anschluss</label>
              <select id="sport" class="input" value={maschine.port} onchange={(e) => maschine.portSetzen((e.currentTarget as HTMLSelectElement).value)}>
                {#each maschine.ports as p (p)}<option value={p}>{p}</option>{/each}
                <option value={SIMULATION}>Simulation (ohne Maschine)</option>
              </select>
            </div>
            <button class="btn btn-secondary" style="width:100%" disabled={maschine.laeuft} onclick={() => maschine.pruefen()}>
              {maschine.laeuft ? "Prüfe …" : "Maschine prüfen (nur lesen)"}
            </button>
            {#if maschine.info}
              <div class="okbox" style="margin-top:8px"><Icon d={IC.haken} /><span>artista 185 antwortet · Firmware {maschine.info.sitzung.naehmaschine} · Stickmodul {maschine.info.sitzung.stickmodul} · {maschine.info.farbbereich.text}</span></div>
            {:else if maschine.pruefFehler}
              <div class="errbox" style="margin-top:8px"><Icon d={IC.fehler} /><span>{maschine.pruefFehler.meldung}</span></div>
            {/if}
          </div>
          <div class="note" style={maschine.rahmenGewechselt ? "border-color:var(--warn); background:var(--warn-bg); color:var(--warn)" : ""}>
            <b style="font-weight:600">{maschine.rahmenGewechselt ? "Rahmen gewechselt?" : "Nach einem Rahmenwechsel"}</b> vorher an der Maschine:
            <ol style="margin:6px 0 8px; padding-left:20px">
              {#each ANLEITUNG_RAHMEN as a (a)}<li style="margin-bottom:2px">{a}</li>{/each}
            </ol>
            {#if maschine.rahmenGewechselt}
              <label style="display:flex; gap:8px; align-items:center; color:var(--color-text)"><input type="checkbox" bind:checked={maschine.rahmenErledigt} style="width:16px; height:16px" />erledigt – Rahmen ist erkannt</label>
            {/if}
          </div>
        </div>
        {#if maschine.sperrgrund}<p class="small" style="color:var(--err); margin:10px 0 0">{maschine.sperrgrund}</p>{/if}
      {:else if maschine.phase === "plan" && maschine.plan}
        <p style="margin:0 0 10px">Gelesen und geprüft – <b style="font-weight:600">noch nichts geschrieben.</b> Diese Schreibzugriffe sind geplant:</p>
        <div role="table" aria-label="Geplante Schreibzugriffe" style="margin-bottom:12px; font-size:13.5px">
          <div role="row" class="small mut" style="display:grid; grid-template-columns:minmax(0,1fr) 110px 260px; gap:10px; padding:6px 0; border-bottom:1px solid var(--line); text-transform:uppercase; letter-spacing:.06em; font-size:11px">
            <span role="columnheader">Was</span><span role="columnheader" style="text-align:right">Größe</span><span role="columnheader">Ziel</span>
          </div>
          {#each teile as t (t.was)}
            <div role="row" style="display:grid; grid-template-columns:minmax(0,1fr) 110px 260px; gap:10px; padding:7px 0; border-bottom:1px solid color-mix(in srgb,var(--color-text) 8%,transparent)">
              <span role="cell">{t.label}</span><span role="cell" class="tab" style="text-align:right">{t.bytes ? `${ganz(t.bytes)} Byte` : "–"}</span>
              <span role="cell" class="mut small">Speicher der Maschine, danach zurücklesen</span>
            </div>
          {/each}
        </div>
        <div style="display:flex; gap:16px; margin-bottom:12px">
          <label style="display:flex; gap:7px; align-items:center"><input type="checkbox" bind:checked={maschine.wahl.icon} onchange={() => maschine.planen()} style="width:16px; height:16px" />Vorschaubild senden</label>
          <label style="display:flex; gap:7px; align-items:center"><input type="checkbox" bind:checked={maschine.wahl.farben} onchange={() => maschine.planen()} style="width:16px; height:16px" />Garnfarben senden ({maschine.plan.garne.length} Farben)</label>
        </div>
        {#each maschine.plan.hinweise as h (h)}<div class="warnbox" style="margin-bottom:6px"><Icon d={IC.warnung} /><span>{h}</span></div>{/each}
        <div class="okbox" style="margin-bottom:6px">
          <Icon d={IC.haken} /><span>Design passt in den Rahmen · {ganz((arbeit.ergebnis?.daten_bytes ?? 0) / 1024)} KB von ≈ {ganz((arbeit.ergebnis?.daten_grenze ?? 0) / 1024)} KB{maschine.plan.farbbereich ? ` · Garnfarbtabelle der Maschine: ${maschine.plan.farbbereich.text}` : ""}</span>
        </div>
        <details class="small mut" style="margin-bottom:6px"><summary>Schritte im Einzelnen (aus dem Kern)</summary><ul style="margin:4px 0 0; padding-left:18px">{#each maschine.plan.schritte as s (s)}<li>{s}</li>{/each}</ul></details>
        <div class="note">Gestickt wird dabei nicht. Das Design bleibt nur bis zum Ausschalten in der Maschine (nicht dauerhaft gespeichert).</div>
      {:else if maschine.phase === "laeuft"}
        <div style="display:flex; align-items:baseline; gap:10px; margin-bottom:4px">
          <span class="hd tab" style="font-size:30px">{prozent} %</span>
          <span class="mut">{maschine.mitSticken ? "Übertragung läuft, danach fährt der Rahmen." : "Übertragung läuft – andere Maschinen-Aktionen sind gesperrt."}</span>
        </div>
        <div style="height:8px; background:color-mix(in srgb,var(--color-text) 10%,transparent); margin-bottom:12px"><div style="height:8px; width:{prozent}%; background:var(--color-accent)"></div></div>
        {#each teile as t (t.was)}
          {@const st = teilStand(t.was, t.bytes)}
          <div class="tl" class:done={st.pct >= 100}>
            <span class="dot">{#if st.pct >= 100}<Icon d={IC.haken} />{/if}</span><span>{t.label}</span><span class="mut small tab">{st.txt}</span>
            <div class="bar"><i style="width:{st.pct}%"></i></div>
          </div>
        {/each}
        {#if wiederholungen}
          <div class="warnbox" style="margin-top:10px"><Icon d={IC.zuruecksetzen} /><span>Übertragungsfehler – automatisch wiederholt ({wiederholungen} ×).</span></div>
        {/if}
      {:else if maschine.phase === "fertig" && maschine.bericht}
        <div class="okbox" style="margin-bottom:10px; font-size:14px">
          <Icon d={IC.haken} />
          <span><b style="font-weight:600">{maschine.bericht.gestickt ? "Gesendet – die Maschine stickt." : maschine.bericht.gestickt === false ? "Gesendet – Sticken nicht bestätigt." : "Gesendet."}</b>
            Alles geschrieben und zurückgelesen – stimmt. PC-Modus verlassen: {maschine.bericht.sitzung.verlassen ? "bestätigt" : "nicht bestätigt"}.</span>
        </div>
        {#if spruch}
          <button
            class="btn btn-ghost"
            style="display:block; width:100%; text-align:left; margin:0 0 10px; padding:8px 11px; font-size:14px; font-style:italic; color:var(--color-accent); white-space:normal"
            title="Noch ein Spruch"
            onclick={() => (spruch = zufallsSpruch(spruch))}>„{spruch}“</button>
        {/if}
        {#each maschine.bericht.schritte as s (s)}
          <div class="tl done"><span class="dot"><Icon d={IC.haken} /></span><span>{s}</span><span></span></div>
        {/each}
        <div class="note" style="margin-top:12px">
          <b style="font-weight:500">An der Maschine:</b> Farbstopps in dieser Reihenfolge –
          <ol style="margin:6px 0 0; padding-left:20px">
            {#each garne as g, k (k)}<li><span class="sw" style="display:inline-block; vertical-align:-2px; margin-right:6px; background:{g.rgb}"></span>{garnText(g)}</li>{/each}
          </ol>
          <p style="margin:6px 0 0">Verbindungsfäden nach dem Sticken von Hand schneiden.</p>
        </div>
        <p class="mut small" style="margin:10px 0 0">Mitschnitt: {maschine.bericht.sitzung.mitschnitt}</p>
      {:else if maschine.phase === "fehler" && maschine.fehler}
        <div class="errbox" style="margin-bottom:12px; font-size:14px">
          <Icon d={IC.fehler} />
          <span><b style="font-weight:600">{maschine.fehler.code === "abgebrochen" ? "Abgebrochen." : "Senden nicht möglich."}</b> {maschine.fehler.meldung}{maschine.plan ? " Es wurde nichts gestickt." : ""}</span>
        </div>
        <div class="note">
          <b style="font-weight:500">So geht es weiter:</b>
          <ol style="margin:6px 0 0; padding-left:20px">{#each naechsteSchritte(maschine.fehler) as s (s)}<li>{s}</li>{/each}</ol>
        </div>
      {/if}
    </div>

    <div class="dlg-f">
      {#if maschine.phase === "anschluss"}
        <button class="btn btn-secondary" onclick={() => maschine.schliessen()}>Abbrechen</button>
        <button class="btn btn-primary" disabled={!!maschine.sperrgrund || maschine.laeuft || (maschine.rahmenGewechselt && !maschine.rahmenErledigt)} onclick={() => maschine.planen()}>
          {maschine.laeuft ? "Plane …" : "Plan erstellen (nur lesen)"}
        </button>
      {:else if maschine.phase === "plan"}
        <button class="btn btn-secondary" style="margin-right:auto" onclick={() => (maschine.phase = "anschluss")}>Zurück</button>
        <button class="btn btn-secondary" onclick={() => maschine.schliessen()}>Abbrechen</button>
        <button class="btn btn-primary blueprint" disabled={maschine.laeuft} onclick={() => maschine.schreiben(false)}>
          <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>Bestätigen und schreiben
        </button>
      {:else if maschine.phase === "laeuft"}
        <span class="mut small" style="margin-right:auto">Abbrechen beendet die Übertragung; der PC-Modus wird trotzdem verlassen, gestickt wird nicht.</span>
        <button class="btn btn-secondary" onclick={() => maschine.abbrechen()}>Abbrechen</button>
        <button class="btn btn-secondary" onclick={() => (maschine.dialogOffen = false)}>Im Hintergrund weiter</button>
      {:else if maschine.phase === "fertig"}
        <button class="btn btn-secondary" style="margin-right:auto" onclick={() => maschine.schliessen()}>{maschine.bericht?.gestickt ? "Schließen" : "Fertig – nicht sticken"}</button>
        {#if !maschine.bericht?.gestickt}
          <button class="btn btn-danger" onclick={() => (maschine.stickenDialog = true)}><Icon d={IC.warnung} />Jetzt sticken …</button>
        {/if}
      {:else}
        <button class="btn btn-secondary" onclick={() => maschine.schliessen()}>Schließen</button>
        <button class="btn btn-primary" onclick={() => maschine.erneut()}>Erneut versuchen</button>
      {/if}
    </div>
  </div>
</div>
