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
  import { maschine, SIMULATION } from "$lib/maschine.svelte";
  import SendenDialog from "$lib/dialoge/SendenDialog.svelte";
  import StickenDialog from "$lib/dialoge/StickenDialog.svelte";
  import AusMaschineDialog from "$lib/dialoge/AusMaschineDialog.svelte";
  import NachFarbeDialog from "$lib/dialoge/NachFarbeDialog.svelte";
  import WarnungDialog from "$lib/dialoge/WarnungDialog.svelte";
  import MusterDialog from "$lib/dialoge/MusterDialog.svelte";
  import Objektliste from "$lib/Objektliste.svelte";
  import Inspektor from "$lib/inspektor/Inspektor.svelte";
  import { EINFUEGEN, KOMMT_NOCH, neuesObjekt } from "$lib/einfuegen";
  import { listen } from "$lib/listen.svelte";
  import type { ReihenfolgeBefehl } from "$kern/kern";

  let menue = $state<"datei" | "maschine" | "einfuegen" | "anordnen" | null>(null);
  let spruenge = $state(true);
  let einstiche = $state(false);
  let raster = $state(true);
  let zoom = $state(1);
  let vorschau: Vorschau | undefined = $state();
  type Werkzeug = "auswahl" | "einstich" | "ausstich" | "knoten";
  let werkzeug = $state<Werkzeug>((globalThis as { __START__?: { werkzeug?: Werkzeug } }).__START__?.werkzeug ?? "auswahl");

  // Abspielen: Stickfolge Stich für Stich (nur Darstellung, keine Maschinen-Simulation)
  let bis = $state(0);
  let spielt = $state(false);
  const gesamt = $derived(arbeit.ergebnis?.stiche.length ?? 0);
  const stopps = $derived(
    (arbeit.ergebnis?.stiche ?? []).flatMap((s, i) => (s.art === "farbwechsel" ? [i] : [])),
  );
  $effect(() => {
    if (!spielt) return;
    let id = 0;
    const schritt = () => {
      bis = Math.min(gesamt, bis + Math.max(3, Math.round(gesamt / 600)));
      if (bis >= gesamt) spielt = false;
      else id = requestAnimationFrame(schritt);
    };
    id = requestAnimationFrame(schritt);
    return () => cancelAnimationFrame(id);
  });
  $effect(() => {
    if (arbeit.ansicht !== "abspielen") spielt = false;
  });

  /** Pfeiltasten: gewählte Objekte um 0,5 mm (Umschalt: 5 mm) verschieben */
  function pfeil(dx: number, dy: number) {
    const aus = [...arbeit.auswahl];
    arbeit.aendern((d) => {
      for (const i of aus) {
        const t = (d.objekte![i].transform ??= {});
        const [vx, vy] = t.verschieben ?? [0, 0];
        t.verschieben = [Math.round((vx + dx) * 100) / 100, Math.round((vy + dy) * 100) / 100];
      }
    }, "pfeil");
  }

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
    { l: "Designs aus der Maschine …", go: () => maschine.listeOeffnen() },
    "-",
    { l: "Darstellung hell/dunkel umschalten", go: () => (arbeit.dunkel = !arbeit.dunkel) },
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

  /** Startoption `maschine` (nur Bildschirmfotos, app/werkzeuge/bildschirmfoto.py): Ablauf bis zur Stufe vorführen */
  async function maschineVorfuehren() {
    const z = (globalThis as { __START__?: { maschine?: string } }).__START__?.maschine;
    if (!z) return;
    if (z === "liste") {
      maschine.listeOeffnen();
      await maschine.listeLesen();
      const nr = maschine.liste?.designs[0]?.nr;
      if (nr) await maschine.designLesen(nr);
      return;
    }
    maschine.sendenOeffnen();
    if (z === "anschluss") return;
    await maschine.planen();
    if (z === "plan") return;
    await maschine.schreiben(false);
    if (z === "sticken") maschine.stickenDialog = true;
  }

  async function einfuegen(art: string) {
    let schrift: string | undefined;
    if (art === "ttf") {
      await listen.schriftenLaden().catch(() => {});
      schrift = listen.schriften[0]?.name;
      if (!schrift) return arbeit.zeige("Keine Computer-Schrift gefunden.", "err");
    }
    const o = neuesObjekt(art, schrift);
    if (o) arbeit.einfuegen(o);
  }

  /** Menü Anordnen: Befehle auf die Auswahl (`reihenfolge_aendern`) */
  const auswahlSortiert = $derived([...arbeit.auswahl].sort((a, b) => a - b));
  const anordnenMenue = $derived.by((): { l: string; kbd?: string; go?: () => void }[] => {
    const a = arbeit.auswahl;
    const n = arbeit.objekte.length;
    const erst = auswahlSortiert[0] ?? -1;
    const letzt = auswahlSortiert.at(-1) ?? -1;
    const um = (b: ReihenfolgeBefehl) => () => arbeit.umordnen(b);
    const nachNummer = (vor: boolean) => () => {
      const t = prompt(`${vor ? "Vor" : "Nach"} Objekt Nr. (1 … ${n}):`);
      const z = t ? parseInt(t, 10) - 1 : NaN;
      if (z >= 0 && z < n) arbeit.umordnen(vor ? { art: "vor", auswahl: [...a], ziel: z } : { art: "nach", auswahl: [...a], ziel: z });
    };
    return [
      { l: "An den Anfang", go: a.length ? um({ art: "an_anfang", auswahl: [...a] }) : undefined },
      { l: "Ans Ende", go: a.length ? um({ art: "ans_ende", auswahl: [...a] }) : undefined },
      { l: "Eins nach vorn", kbd: "Alt+↑", go: a.length && erst > 0 ? um({ art: "vor", auswahl: [...a], ziel: erst - 1 }) : undefined },
      { l: "Eins nach hinten", kbd: "Alt+↓", go: a.length && letzt < n - 1 ? um({ art: "nach", auswahl: [...a], ziel: letzt + 1 }) : undefined },
      { l: "Wie ausgewählt (Klickfolge)", go: a.length > 1 ? um({ art: "wie_ausgewaehlt", auswahl: [...a] }) : undefined },
      { l: "Vor Objekt Nr. …", go: a.length ? nachNummer(true) : undefined },
      { l: "Nach Objekt Nr. …", go: a.length ? nachNummer(false) : undefined },
      { l: "Nach Farbe sortieren …", go: n > 1 ? () => (arbeit.dialog = { art: "nach_farbe" }) : undefined },
    ];
  });

  function waehle(f?: () => void) {
    menue = null;
    f?.();
  }

  function tasten(ev: KeyboardEvent) {
    const strg = ev.ctrlKey || ev.metaKey;
    const ziel = ev.target as HTMLElement;
    const eingabe = ziel?.closest?.("input, textarea, select, [contenteditable]");
    if (arbeit.dialog || maschine.dialogOffen || maschine.listeOffen || maschine.stickenDialog) return; // Dialog behandelt Esc selbst
    if (ev.key === "Escape") {
      if (werkzeug !== "auswahl") werkzeug = "auswahl";
      else if (menue) menue = null;
      else arbeit.waehlen(null);
      return;
    }
    if (arbeit.modus === "experte" && !eingabe) {
      if (werkzeug === "knoten" && vorschau?.knotenTaste(ev.key)) return ev.preventDefault();
      if ((ev.key === "Delete" || ev.key === "Backspace") && arbeit.auswahl.length) return (ev.preventDefault(), arbeit.loeschen());
      if (ev.altKey && (ev.key === "ArrowUp" || ev.key === "ArrowDown")) {
        const m = anordnenMenue[ev.key === "ArrowUp" ? 2 : 3];
        ev.preventDefault();
        return m.go?.();
      }
      if (strg && ev.key.toLowerCase() === "d") return (ev.preventDefault(), arbeit.duplizieren());
      const pf: Record<string, [number, number]> = { ArrowLeft: [-1, 0], ArrowRight: [1, 0], ArrowUp: [0, 1], ArrowDown: [0, -1] };
      if (pf[ev.key] && !ev.altKey && arbeit.auswahl.length) {
        const k = ev.shiftKey ? 5 : 0.5;
        ev.preventDefault();
        return pfeil(pf[ev.key][0] * k, pf[ev.key][1] * k);
      }
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

  // Überdeckungen (Objektliste) gibt es nur im Expertenmodus – beim Umschalten nachholen
  $effect(() => {
    if (arbeit.modus === "experte") arbeit.planen(0);
  });

  onMount(() => {
    maschine.portsLaden();
    arbeit.starten().then(() => maschineVorfuehren());
  });
</script>

<svelte:window onkeydown={tasten} />

<div class="sp" class:dunkel={arbeit.dunkel} style="height:100vh; min-width:1280px">
  {#if maschine.simulation}
    <div class="simband" role="status" style="flex:none; display:flex; align-items:center; gap:10px; padding:4px 14px; font-size:12.5px; font-weight:500">
      <Icon d={IC.monitor} />
      <span><b style="letter-spacing:.08em">SIMULATION</b> – keine echte Maschine. Es wird nichts gesendet und nichts gestickt.</span>
      <span style="margin-left:auto">Anschluss: Simulation · Mitschnitt wird trotzdem gespeichert</span>
    </div>
  {/if}
  <header style="flex:none; height:48px; display:flex; align-items:center; gap:4px; padding:0 10px; border-bottom:1px solid var(--line); background:var(--color-bg); position:relative; z-index:31">
    <div style="display:flex; flex-direction:column; line-height:1.05; padding-right:10px; margin-right:2px; border-right:1px solid var(--line)">
      <span class="hd" style="font-size:17px">Stickprogramm</span>
      <span class="mut" style="font-size:11px">für BERNINA artista 185</span>
    </div>
    <button class="mbtn" aria-haspopup="menu" aria-expanded={menue === "datei"} onclick={() => (menue = menue === "datei" ? null : "datei")}>
      Datei<Icon d={IC.chevronUnten} groesse="s" />
    </button>
    {#if arbeit.modus === "experte"}
      <button class="mbtn" aria-haspopup="menu" aria-expanded={menue === "einfuegen"} onclick={() => (menue = menue === "einfuegen" ? null : "einfuegen")}>Einfügen<Icon d={IC.chevronUnten} groesse="s" /></button>
      <button class="mbtn" aria-haspopup="menu" aria-expanded={menue === "anordnen"} onclick={() => (menue = menue === "anordnen" ? null : "anordnen")}>Anordnen<Icon d={IC.chevronUnten} groesse="s" /></button>
    {/if}
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
    <button
      class="mbtn"
      title={maschine.laeuft ? "Maschine arbeitet …" : maschine.info ? `Firmware ${maschine.info.sitzung.naehmaschine} · ${maschine.info.farbbereich.text}` : "Anschluss und Maschine"}
      aria-haspopup="menu"
      aria-expanded={menue === "maschine"}
      onclick={() => (menue = menue === "maschine" ? null : "maschine")}
      style="border:1px solid var(--line); margin-left:6px; gap:7px"
    >
      <span aria-hidden="true" style="width:9px; height:9px; border-radius:50%; background:{maschine.laeuft ? 'var(--warn)' : maschine.simulation ? 'var(--sim)' : maschine.info ? 'var(--ok)' : maschine.pruefFehler ? 'var(--err)' : 'var(--muted)'}"></span>
      {maschine.laeuft && maschine.phase === "laeuft" ? "Sende …" : maschine.simulation ? "Simulation" : `${maschine.port} · artista 185`}
    </button>
    {#if istMac}
      <button class="btn btn-primary blueprint" style="margin-left:6px; gap:7px" onclick={expExportieren}>
        <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>
        <Icon d={IC.herunterladen} />EXP exportieren
      </button>
    {:else}
      <button class="btn btn-primary blueprint" style="margin-left:6px; gap:7px" disabled={!!maschine.sperrgrund} title={maschine.sperrgrund ?? "An die Maschine senden (erst Plan, dann bestätigen)"} onclick={() => maschine.sendenOeffnen()}>
        <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>
        <Icon d={IC.senden} />Senden …
      </button>
    {/if}

    {#if menue === "maschine"}
      <div class="menu" role="menu" aria-label="Maschine" style="right:120px; top:44px; width:300px">
        <div class="mh">Anschluss</div>
        <div style="padding:2px 10px 8px">
          <select class="input sm" style="width:100%" aria-label="Anschluss" value={maschine.port} onchange={(e) => maschine.portSetzen((e.currentTarget as HTMLSelectElement).value)}>
            {#each maschine.ports as p (p)}<option value={p}>{p}</option>{/each}
            <option value={SIMULATION}>Simulation (ohne Maschine)</option>
          </select>
        </div>
        <button class="mi" role="menuitem" disabled={maschine.laeuft} onclick={() => waehle(() => maschine.pruefen())}>Maschine prüfen (nur lesen)</button>
        <button class="mi" role="menuitem" disabled={maschine.laeuft} onclick={() => waehle(() => maschine.listeOeffnen())}>Designs aus der Maschine …</button>
        {#if istMac}
          <button class="mi" role="menuitem" disabled={!!maschine.sperrgrund} onclick={() => waehle(() => maschine.sendenOeffnen())}>Senden …</button>
        {/if}
        {#if maschine.laeuft && !maschine.dialogOffen && maschine.phase === "laeuft"}
          <button class="mi" role="menuitem" onclick={() => waehle(() => (maschine.dialogOffen = true))}>Fortschritt anzeigen</button>
        {/if}
        <div class="msep"></div>
        <div class="small mut" style="padding:4px 10px 8px; white-space:normal">
          {#if maschine.info}Firmware {maschine.info.sitzung.naehmaschine} · Stickmodul {maschine.info.sitzung.stickmodul} · {maschine.info.farbbereich.text}
          {:else if maschine.pruefFehler}<span style="color:var(--err)">{maschine.pruefFehler.meldung}</span>
          {:else}Noch nicht geprüft.{/if}
        </div>
      </div>
    {/if}
    {#if menue === "einfuegen"}
      <div class="menu" role="menu" aria-label="Einfügen" style="left:205px; top:44px; display:grid; grid-template-columns:240px 300px; padding:0">
        <div style="padding:4px; border-right:1px solid var(--line)">
          <div class="mh">Objekt einfügen</div>
          {#each EINFUEGEN as x (x.art)}
            <button class="mi" role="menuitem" onclick={() => waehle(() => einfuegen(x.art))}><Icon d={ART[x.art]?.[1] ?? ""} />{ART[x.art]?.[0] ?? x.art}</button>
          {/each}
          <div class="msep"></div>
          <button class="mi" role="menuitem" onclick={() => waehle(() => (arbeit.dialog = { art: "bild" }))}><Icon d={IC.bild} />Bild umwandeln …</button>
          <button class="mi" role="menuitem" onclick={() => waehle(() => (arbeit.dialog = { art: "svg" }))}><Icon d={IC.bezier} />SVG einlesen …</button>
        </div>
        <div style="padding:4px; background:var(--panel)">
          <div class="mh">Kommt noch – Platz ist vorgesehen</div>
          {#each KOMMT_NOCH as t (t)}
            <button class="mi" role="menuitem" disabled style="white-space:normal; align-items:flex-start"><Icon d={IC.kommtNoch} /><span style="flex:1">{t}</span><span class="soon">kommt noch</span></button>
          {/each}
        </div>
      </div>
    {/if}
    {#if menue === "anordnen"}
      <div class="menu" role="menu" aria-label="Anordnen" style="left:282px; top:44px">
        <div class="mh">Stickreihenfolge – auf die Auswahl</div>
        {#each anordnenMenue as m (m.l)}
          <button class="mi" role="menuitem" disabled={!m.go} onclick={() => waehle(m.go)}>{m.l}{#if m.kbd}<span class="kbd">{m.kbd}</span>{/if}</button>
        {/each}
        <div class="msep"></div>
        <div class="small mut" style="padding:4px 10px 8px; max-width:280px; white-space:normal">Liegt ein Objekt danach unter einer Fläche, über der es lag, warnt das Programm.</div>
      </div>
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
      <Objektliste anordnen={() => (menue = "anordnen")} />
    {/if}

    <section aria-label="Stichvorschau" style="flex:1; min-width:0; display:flex; flex-direction:column; position:relative">
      <div class="vbar">
        <div class="seg seg-sm" role="radiogroup" aria-label="Ansicht" style="flex:none">
          <label class="seg-opt"><input type="radio" name="ansicht" checked={arbeit.ansicht === "stiche"} onchange={() => (arbeit.ansicht = "stiche")} />Stiche</label>
          {#if arbeit.modus === "experte"}
            <label class="seg-opt"><input type="radio" name="ansicht" checked={arbeit.ansicht === "form"} onchange={() => (arbeit.ansicht = "form")} />Form</label>
          {/if}
          <label class="seg-opt"><input type="radio" name="ansicht" checked={arbeit.ansicht === "abspielen"} onchange={() => ((arbeit.ansicht = "abspielen"), (bis = 0), (spielt = true))} />Abspielen</label>
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
        {#if arbeit.modus === "experte"}
          <div class="rail" role="toolbar" aria-label="Werkzeuge" aria-orientation="vertical">
            <button class="tb" aria-pressed={werkzeug === "auswahl"} aria-label="Auswählen und Griffe" title="Auswählen, ziehen, Griffe (skalieren, drehen)" onclick={() => (werkzeug = "auswahl")}><Icon d={IC.zeiger} groesse="l" /></button>
            <button class="tb" aria-pressed={werkzeug === "knoten"} disabled={arbeit.auswahl.length !== 1} aria-label="Knoten bearbeiten" title="Knoten bearbeiten (ziehen; Entf löscht, Einfg fügt ein)" onclick={() => ((werkzeug = "knoten"), (arbeit.ansicht = "form"))}><Icon d={IC.knoten} groesse="l" /></button>
            <button class="tb" aria-pressed={werkzeug === "einstich"} disabled={arbeit.auswahl.length !== 1} aria-label="Einstich setzen" title="Einstich des gewählten Objekts setzen" onclick={() => (werkzeug = "einstich")}><Icon d={IC.einaus} groesse="l" /></button>
            <button class="tb" aria-pressed={werkzeug === "ausstich"} disabled={arbeit.auswahl.length !== 1} aria-label="Ausstich setzen" title="Ausstich des gewählten Objekts setzen" onclick={() => (werkzeug = "ausstich")} style="transform:scaleX(-1)"><Icon d={IC.einaus} groesse="l" /></button>
            <span class="sep"></span>
            {#each EINFUEGEN as x, k (x.art)}
              {#if k > 0 && EINFUEGEN[k - 1].gruppe !== x.gruppe}<span class="sep"></span>{/if}
              <button class="tb" aria-label="{ART[x.art]?.[0]} einfügen" title="{ART[x.art]?.[0]} einfügen" onclick={() => einfuegen(x.art)}><Icon d={ART[x.art]?.[1] ?? ""} groesse="l" /></button>
            {/each}
            <span class="sep"></span>
            <button class="tb" aria-label="Weitere Werkzeuge – kommen noch" title="Kommt noch: Motive, Effekte, Kreuzstich …" onclick={() => (menue = "einfuegen")} style="border:1px dashed var(--muted)"><Icon d={IC.kommtNoch} groesse="l" /></button>
          </div>
        {/if}
        <div class="stage">
          <Vorschau bind:this={vorschau} bind:zoom bind:werkzeug {spruenge} {einstiche} {raster} bis={arbeit.ansicht === "abspielen" ? bis : null} />
          {#if werkzeug === "knoten"}
            <div class="floater small" style="left:12px; bottom:12px; padding:7px 10px; max-width:520px">
              <b style="font-weight:500">Knoten</b> – Punkte ziehen · gewählten Punkt mit Entf löschen, mit Einfg (oder +) dahinter einen einfügen · Esc beendet.
              Formen wie Kreis/Rechteck und Objekte mit Größe/Drehung: Werte im Inspektor bzw. JSON (Umwandeln in Punkte kommt noch).
            </div>
          {:else if werkzeug !== "auswahl"}
            <div class="floater small" style="left:12px; bottom:12px; padding:7px 10px">
              <b style="font-weight:500">{werkzeug === "einstich" ? "Einstich" : "Ausstich"} setzen</b> – in die Vorschau klicken (Esc bricht ab)
            </div>
          {/if}
          {#if arbeit.ansicht === "abspielen"}
            <div class="floater" style="left:12px; right:12px; bottom:12px; padding:8px 12px">
              <div style="display:flex; align-items:center; gap:10px">
                <button class="btn btn-primary" style="min-width:110px" onclick={() => (bis >= gesamt && (bis = 0), (spielt = !spielt))}>
                  <Icon d={spielt ? IC.pause : IC.abspielen} />{spielt ? "Anhalten" : "Abspielen"}
                </button>
                <div style="flex:1; position:relative; padding-top:10px">
                  {#each stopps as st, k (k)}
                    <span title="Farbwechsel {k + 1}" style="position:absolute; top:0; left:{(st / Math.max(1, gesamt)) * 100}%; width:10px; height:8px; margin-left:-1px; background:{arbeit.ergebnis?.farben[k + 1] ?? '#000'}; border:1px solid rgba(0,0,0,.4)"></span>
                  {/each}
                  <input type="range" min="0" max={gesamt} value={bis} oninput={(ev) => ((spielt = false), (bis = Number((ev.currentTarget as HTMLInputElement).value)))} aria-label="Stickfolge" style="width:100%; accent-color:var(--color-accent)" />
                </div>
              </div>
              <div class="small mut tab" style="margin-top:2px">Stich {bis.toLocaleString("de-DE")} von {gesamt.toLocaleString("de-DE")} · Abspielen zeigt nur die Reihenfolge – es ist keine Maschinen-Simulation.</div>
            </div>
          {/if}
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
                <p class="mut" style="margin:0 0 12px">Links ein Werkzeug wählen (fügt ein Objekt in der Rahmenmitte ein) – oder:</p>
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
    {#if arbeit.modus === "experte"}<Inspektor />{/if}
  </div>

  {#if arbeit.dialog?.art === "muster" || arbeit.dialog?.art === "step"}
    {@const d = arbeit.dialog}
    {@const x = arbeit.objekte[d.objekt] as Record<string, unknown> | undefined}
    <MusterDialog
      art={d.art}
      aktuell={(d.art === "muster" ? x?.muster : x?.step_muster) as string | number | null | undefined}
      waehlen={(w) => {
        const feld = d.art === "muster" ? "muster" : "step_muster";
        arbeit.aendern((dd) => {
          const y = dd.objekte?.[d.objekt] as Record<string, unknown> | undefined;
          if (!y) return;
          if (w === null) delete y[feld];
          else y[feld] = w;
        });
        arbeit.dialog = null;
      }}
      schliessen={() => (arbeit.dialog = null)}
    />
  {/if}
  {#if arbeit.dialog?.art === "nach_farbe"}<NachFarbeDialog schliessen={() => (arbeit.dialog = null)} />{/if}
  {#if arbeit.dialog?.art === "warnung"}
    <WarnungDialog vertauscht={arbeit.dialog.vertauscht} namen={arbeit.dialog.namen} vorher={arbeit.dialog.vorher} schliessen={() => (arbeit.dialog = null)} />
  {/if}
  {#if maschine.dialogOffen}<SendenDialog />{/if}
  {#if maschine.stickenDialog}<StickenDialog />{/if}
  {#if maschine.listeOffen}<AusMaschineDialog />{/if}
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
