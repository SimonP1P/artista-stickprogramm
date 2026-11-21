<script lang="ts">
  // Hauptfenster nach Entwurf v3 (UI_Mockups/v3_2026-11-21/UEBERGABE.md §2): Kopf · links Assistent (einfach) bzw.
  // Objektliste (Experte) · Mitte Stichvorschau · rechts Inspektor (Experte) · Statusleiste.
  import { onMount } from "svelte";
  import { arbeit } from "$lib/arbeit.svelte";
  import { beispiel, dateiname, expExportieren, neu, oeffnen, speichern } from "$lib/dateien.svelte";
  import Icon from "$lib/Icon.svelte";
  import { ART, IC } from "$lib/icons";
  import Statusleiste from "$lib/Statusleiste.svelte";
  import Vorschau from "$lib/Vorschau.svelte";
  import Assistent from "$lib/Assistent.svelte";
  import AlphabetDialog from "$lib/dialoge/AlphabetDialog.svelte";
  import GarnDialog from "$lib/dialoge/GarnDialog.svelte";
  import EinlesenDialog from "$lib/dialoge/EinlesenDialog.svelte";
  import type { GarnInfo } from "$kern/kern";

  let menue = $state<"datei" | null>(null);
  let spruenge = $state(true);
  let einstiche = $state(false);
  let raster = $state(true);
  let zoom = $state(1);
  let vorschau: Vorschau | undefined = $state();

  const istMac = typeof navigator !== "undefined" && /Mac/i.test(navigator.platform || navigator.userAgent);
  const leer = $derived(arbeit.objekte.length === 0);
  const kopfInfo = $derived(
    [arbeit.design.rahmen?.replace(/^artista /, ""), arbeit.design.stoff].filter(Boolean).join(" · ") +
      (arbeit.pfad ? ` · ${dateiname(arbeit.pfad)}${arbeit.geaendert ? " *" : ""}` : arbeit.geaendert ? " · nicht gespeichert" : ""),
  );

  type Eintrag = { l: string; go?: () => void; kbd?: string; spaeter?: string } | "-";
  const dateiMenue: Eintrag[] = [
    { l: "Neu", go: neu, kbd: "Strg+N" },
    { l: "Öffnen …", go: oeffnen, kbd: "Strg+O" },
    { l: "Speichern", go: () => speichern(), kbd: "Strg+S" },
    { l: "Speichern unter …", go: () => speichern(true) },
    "-",
    { l: "Beispiel laden", go: beispiel },
    { l: "Bild umwandeln …", go: () => (arbeit.dialog = { art: "bild" }) },
    { l: "SVG einlesen …", go: () => (arbeit.dialog = { art: "svg" }) },
    "-",
    { l: "Stickdatei exportieren (EXP) …", go: expExportieren, kbd: "Strg+E" },
    { l: "Designs aus der Maschine …", spaeter: "E3" },
  ];

  /** Garn des Farbblocks, mit dem ein Objekt beginnt (Anzeige „jetzt: …“) */
  function garnVon(o: number): GarnInfo | undefined {
    const e = arbeit.ergebnis;
    const k = e?.bloecke.findIndex((b) => b.objekt === o || b.objekte.includes(o)) ?? -1;
    return k >= 0 ? e?.garne[k] : undefined;
  }
  function garnSetzen(o: number, g: GarnInfo) {
    arbeit.aendern((d) => {
      const x = d.objekte?.[o];
      if (!x) return;
      x.garn = g.nummer;
      x.farbe = g.rgb;
    });
    arbeit.dialog = null;
  }

  function waehle(f?: () => void) {
    menue = null;
    f?.();
  }

  function tasten(ev: KeyboardEvent) {
    const strg = ev.ctrlKey || ev.metaKey;
    const ziel = ev.target as HTMLElement;
    const eingabe = ziel?.closest?.("input, textarea, select, [contenteditable]");
    if (arbeit.dialog) return; // Dialog behandelt Esc selbst
    if (ev.key === "Escape") {
      if (menue) menue = null;
      else arbeit.waehlen(null);
      return;
    }
    if (!strg) return;
    const k = ev.key.toLowerCase();
    if (k === "z" && !ev.shiftKey && !eingabe) (ev.preventDefault(), arbeit.zurueck());
    else if ((k === "y" || (k === "z" && ev.shiftKey)) && !eingabe) (ev.preventDefault(), arbeit.vor());
    else if (k === "s") (ev.preventDefault(), speichern(ev.shiftKey));
    else if (k === "o") (ev.preventDefault(), oeffnen());
    else if (k === "n") (ev.preventDefault(), neu());
    else if (k === "e") (ev.preventDefault(), expExportieren());
  }

  onMount(() => {
    arbeit.starten();
  });
</script>

<svelte:window onkeydown={tasten} />

<div class="sp" class:dunkel={arbeit.dunkel} style="height:100vh; min-width:1280px">
  <header style="flex:none; height:48px; display:flex; align-items:center; gap:4px; padding:0 10px; border-bottom:1px solid var(--line); background:var(--color-bg); position:relative; z-index:31">
    <div style="display:flex; flex-direction:column; line-height:1.05; padding-right:10px; margin-right:2px; border-right:1px solid var(--line)">
      <span class="hd" style="font-size:17px">Stickprogramm</span>
      <span class="mut" style="font-size:11px">für BERNINA artista 185</span>
    </div>
    <button class="mbtn" aria-haspopup="menu" aria-expanded={menue === "datei"} onclick={() => (menue = menue === "datei" ? null : "datei")}>
      Datei<Icon d={IC.chevronUnten} groesse="s" />
    </button>
    <span style="width:1px; height:22px; background:var(--line); margin:0 4px"></span>
    <button class="tb" aria-label="Rückgängig (Strg+Z)" title="Rückgängig (Strg+Z)" disabled={!arbeit.kannZurueck} onclick={() => arbeit.zurueck()}>
      <Icon d={IC.rueckgaengig} />
    </button>
    <button class="tb" aria-label="Wiederholen (Strg+Y)" title="Wiederholen (Strg+Y)" disabled={!arbeit.kannVor} onclick={() => arbeit.vor()}>
      <Icon d={IC.wiederholen} />
    </button>
    <div style="flex:1; min-width:0; display:flex; align-items:center; justify-content:center; gap:10px">
      <input
        class="input sm"
        aria-label="Name des Designs"
        style="width:190px; font-weight:500"
        value={arbeit.design.name ?? ""}
        onchange={(ev) => arbeit.aendern((d) => (d.name = (ev.currentTarget as HTMLInputElement).value), "name")}
      />
      <span class="mut small" style="white-space:nowrap; overflow:hidden; text-overflow:ellipsis; min-width:0">{kopfInfo}</span>
    </div>
    <div class="seg seg-sm" role="radiogroup" aria-label="Modus">
      <label class="seg-opt"><input type="radio" name="modus" checked={arbeit.modus === "einfach"} onchange={() => (arbeit.modus = "einfach")} />Einfach</label>
      <label class="seg-opt"><input type="radio" name="modus" checked={arbeit.modus === "experte"} onchange={() => (arbeit.modus = "experte")} />Experte</label>
    </div>
    <button class="mbtn" title="Maschine – folgt in Etappe E3" disabled style="border:1px solid var(--line); margin-left:6px; gap:7px">
      <span aria-hidden="true" style="width:9px; height:9px; border-radius:50%; background:var(--muted)"></span>Maschine
    </button>
    {#if istMac}
      <button class="btn btn-primary blueprint" style="margin-left:6px; gap:7px" onclick={expExportieren}>
        <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>
        <Icon d={IC.herunterladen} />EXP exportieren
      </button>
    {:else}
      <button class="btn btn-primary blueprint" style="margin-left:6px; gap:7px" disabled title="Senden folgt in Etappe E3">
        <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>
        <Icon d={IC.senden} />Senden …
      </button>
    {/if}

    {#if menue === "datei"}
      <div class="menu" role="menu" aria-label="Datei" style="left:142px; top:44px">
        {#each dateiMenue as m, k (k)}
          {#if m === "-"}
            <div class="msep"></div>
          {:else}
            <button class="mi" role="menuitem" disabled={!m.go} onclick={() => waehle(m.go)}>
              {m.l}
              {#if m.spaeter}<span class="soon" style="margin-left:auto">folgt ({m.spaeter})</span>{/if}
              {#if m.kbd}<span class="kbd">{m.kbd}</span>{/if}
            </button>
          {/if}
        {/each}
      </div>
    {/if}
  </header>
  {#if menue}<div class="shade" role="presentation" onclick={() => (menue = null)}></div>{/if}

  {#if arbeit.meldung}
    <div
      class={arbeit.meldung.art === "err" ? "errbox" : arbeit.meldung.art === "ok" ? "okbox" : "note"}
      role={arbeit.meldung.art === "err" ? "alert" : "status"}
      style="position:absolute; top:56px; left:50%; transform:translateX(-50%); z-index:45; box-shadow:var(--shadow-md); align-items:center; max-width:640px"
    >
      <Icon d={arbeit.meldung.art === "err" ? IC.fehler : IC.haken} />
      <span style="flex:1">{arbeit.meldung.text}</span>
      <button class="tb sm" aria-label="Schließen" onclick={() => (arbeit.meldung = null)}><Icon d={IC.kreuz} groesse="s" /></button>
    </div>
  {/if}

  <div style="flex:1; min-height:0; display:flex">
    {#if arbeit.modus === "einfach"}
      <Assistent {istMac} />
    {:else}
      <aside class="pane" aria-label="Objekte" style="width:268px; border-right:1px solid var(--line)">
        <div class="pane-h"><span class="hd" style="font-size:17px">Objekte</span><span class="badge">{arbeit.objekte.length}</span></div>
        <div class="scroll" role="listbox" aria-label="Objekte in Stickreihenfolge">
          {#each arbeit.objekte as o, i (i)}
            {@const art = ART[o.art] ?? [o.art, ""]}
            <button
              class="orow"
              class:on={arbeit.auswahl.includes(i)}
              class:err={arbeit.fehler?.objekt === i}
              role="option"
              aria-selected={arbeit.auswahl.includes(i)}
              onclick={(ev) => arbeit.waehlen(i, ev.shiftKey || ev.ctrlKey || ev.metaKey)}
            >
              <span class="grip"><Icon d={IC.griff} groesse="s" /></span>
              <span class="tab" style="text-align:right">{i + 1}</span>
              <span class="sw" style="background:{o.farbe ?? '#000000'}"></span>
              <span title={art[0]}><Icon d={art[1]} groesse="s" /></span>
              <span class="ostack"><b>{o.name || ("text" in o ? o.text : "") || art[0]}</b><span>{art[0]}</span></span>
              <span></span>
            </button>
          {/each}
        </div>
      </aside>
    {/if}

    <section aria-label="Stichvorschau" style="flex:1; min-width:0; display:flex; flex-direction:column; position:relative">
      <div class="vbar">
        <div class="seg seg-sm" role="radiogroup" aria-label="Ansicht" style="flex:none">
          <label class="seg-opt"><input type="radio" name="ansicht" checked={arbeit.ansicht === "stiche"} onchange={() => (arbeit.ansicht = "stiche")} />Stiche</label>
          {#if arbeit.modus === "experte"}
            <label class="seg-opt"><input type="radio" name="ansicht" checked={arbeit.ansicht === "form"} onchange={() => (arbeit.ansicht = "form")} />Form</label>
          {/if}
        </div>
        <span style="width:6px"></span>
        <button class="mbtn tgl" aria-pressed={spruenge} onclick={() => (spruenge = !spruenge)} style="height:28px; font-size:12.5px; padding:0 8px; flex:none">Sprünge</button>
        <button class="mbtn tgl" aria-pressed={einstiche} onclick={() => (einstiche = !einstiche)} style="height:28px; font-size:12.5px; padding:0 8px; flex:none">Einstiche</button>
        <button class="mbtn tgl" aria-pressed={raster} onclick={() => (raster = !raster)} style="height:28px; font-size:12.5px; padding:0 8px; flex:none">Raster</button>
        <span style="flex:1"></span>
        <span class="mut small tab" aria-live="polite" title="Stiche neu berechnet (design_berechnen)" style="white-space:nowrap">
          ↻ {arbeit.rechnet ? "…" : arbeit.dauer !== null ? `${arbeit.dauer} ms` : "–"}
        </span>
        <span style="width:1px; height:20px; background:var(--line); margin:0 4px"></span>
        <button class="tb" aria-label="Verkleinern" onclick={() => (zoom = Math.max(0.25, zoom / 1.25))}><Icon d={IC.verkleinern} /></button>
        <span class="tab small" style="width:46px; text-align:center; white-space:nowrap">{Math.round(zoom * 100)} %</span>
        <button class="tb" aria-label="Vergrößern" onclick={() => (zoom = Math.min(40, zoom * 1.25))}><Icon d={IC.vergroessern} /></button>
        <button class="tb" aria-label="Rahmen einpassen" title="Rahmen einpassen" onclick={() => vorschau?.einpassen()}><Icon d={IC.einpassen} /></button>
      </div>
      <div style="flex:1; min-height:0; display:flex">
        <div class="stage">
          <Vorschau bind:this={vorschau} bind:zoom {spruenge} {einstiche} {raster} />
          {#if arbeit.fehler}
            <div class="fbanner errbox" role="alert" style="align-items:center; box-shadow:var(--shadow-sm)">
              <Icon d={IC.fehler} />
              <span style="flex:1"><b style="font-weight:600">Kann nicht berechnet werden.</b> {arbeit.fehler.meldung} Die Vorschau zeigt den letzten gültigen Stand.</span>
              {#if arbeit.fehler.objekt !== null && arbeit.fehler.objekt !== undefined}
                <button class="btn btn-secondary" style="min-height:28px; padding:2px 10px; color:inherit" onclick={() => arbeit.waehlen(arbeit.fehler?.objekt ?? null)}>Zum Objekt</button>
              {/if}
            </div>
          {/if}
          {#if leer && !arbeit.rechnet}
            {#if arbeit.modus === "experte"}
              <div class="floater" style="left:50%; top:50%; transform:translate(-50%,-50%); width:380px; padding:18px">
                <h3 style="margin:0 0 4px; font-size:21px">Leeres Design</h3>
                <p class="mut" style="margin:0 0 12px">Werkzeuge zum Zeichnen folgen in Etappe E4 – bis dahin:</p>
                <div style="display:grid; grid-template-columns:1fr 1fr; gap:6px">
                  <button class="btn btn-secondary" onclick={oeffnen}>Öffnen …</button>
                  <button class="btn btn-secondary" onclick={beispiel}>Beispiel laden</button>
                </div>
              </div>
            {:else}
              <div class="floater" style="left:50%; top:calc(50% + 40px); transform:translate(-50%,0); padding:10px 14px; font-size:13px">
                Noch nichts im Rahmen – im Schritt „Gestalten“ Text oder Monogramm einfügen.
              </div>
            {/if}
          {/if}
        </div>
      </div>
      <Statusleiste />
    </section>
  </div>

  {#if arbeit.dialog?.art === "alphabet"}
    {@const o = arbeit.dialog.objekt}
    {@const x = arbeit.objekte[o]}
    <AlphabetDialog
      aktuell={x && "alphabet" in x ? x.alphabet : undefined}
      waehlen={(name) => {
        arbeit.aendern((d) => {
          const y = d.objekte?.[o];
          if (y && "alphabet" in y) y.alphabet = name;
        });
        arbeit.dialog = null;
      }}
      schliessen={() => (arbeit.dialog = null)}
    />
  {:else if arbeit.dialog?.art === "garn"}
    {@const o = arbeit.dialog.objekt}
    <GarnDialog
      untertitel={arbeit.dialog.titel ?? arbeit.objekte[o]?.name ?? ""}
      aktuell={garnVon(o)}
      waehlen={(g) => garnSetzen(o, g)}
      schliessen={() => (arbeit.dialog = null)}
    />
  {:else if arbeit.dialog?.art === "bild" || arbeit.dialog?.art === "svg"}
    <EinlesenDialog art={arbeit.dialog.art} schliessen={() => (arbeit.dialog = null)} />
  {/if}
</div>
