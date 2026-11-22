<script lang="ts">
  // Hauptfenster nach Entwurf v3 (UI_Mockups/v3_2026-11-21/UEBERGABE.md §2): Kopf · links Assistent (einfach) bzw.
  // Objektliste (Experte) · Mitte Stichvorschau · rechts Inspektor (Experte) · Statusleiste.
  import { onMount } from "svelte";
  import { arbeit } from "$lib/arbeit.svelte";
  import { beispiel, dateiname, designEinfuegen, expExportieren, neu, oeffnen, speichern } from "$lib/dateien.svelte";
  import Icon from "$lib/Icon.svelte";
  import { ART, IC } from "$lib/icons";
  import Statusleiste from "$lib/Statusleiste.svelte";
  import Vorschau from "$lib/Vorschau.svelte";
  import Assistent from "$lib/Assistent.svelte";
  import SchriftDialog from "$lib/dialoge/SchriftDialog.svelte";
  import GarnDialog from "$lib/dialoge/GarnDialog.svelte";
  import EinlesenDialog from "$lib/dialoge/EinlesenDialog.svelte";
  import type { CraftVorlage, GarnInfo, ObjektJson } from "$kern/kern";
  import { maschine, SIMULATION } from "$lib/maschine.svelte";
  import SendenDialog from "$lib/dialoge/SendenDialog.svelte";
  import StickenDialog from "$lib/dialoge/StickenDialog.svelte";
  import AusMaschineDialog from "$lib/dialoge/AusMaschineDialog.svelte";
  import NachFarbeDialog from "$lib/dialoge/NachFarbeDialog.svelte";
  import WarnungDialog from "$lib/dialoge/WarnungDialog.svelte";
  import MusterDialog from "$lib/dialoge/MusterDialog.svelte";
  import MotivDialog from "$lib/dialoge/MotivDialog.svelte";
  import MotivSpeichernDialog from "$lib/dialoge/MotivSpeichernDialog.svelte";
  import FarbenMischenDialog from "$lib/dialoge/FarbenMischenDialog.svelte";
  import KopierenDialog from "$lib/dialoge/KopierenDialog.svelte";
  import NummerDialog from "$lib/dialoge/NummerDialog.svelte";
  import RahmenDialog from "$lib/dialoge/RahmenDialog.svelte";
  import EigeneRahmenDialog from "$lib/dialoge/EigeneRahmenDialog.svelte";
  import Objektliste from "$lib/Objektliste.svelte";
  import Inspektor from "$lib/inspektor/Inspektor.svelte";
  import {
    BESONDERE,
    FORMEN,
    FUELLUNG,
    KOMMT_NOCH,
    neuesObjekt,
    SCHRIFT,
    STANDARD_FARBE,
    STICHART_HILFE,
    STICHART_NAME,
    UMRISS,
    type FormWerkzeug,
  } from "$lib/einfuegen";
  import { WECHSELBAR } from "$lib/arbeit.svelte";
  import { listen } from "$lib/listen.svelte";
  import { knotenBild } from "$lib/knoten";
  import { formAusPunkten, ZEICHEN_HILFE, type ZeichenPunkt } from "$lib/zeichnen";
  import type { AusrichtArt, ReihenfolgeBefehl } from "$kern/kern";

  let menue = $state<"datei" | "maschine" | "einfuegen" | "anordnen" | "fuellung" | "umriss" | null>(null);
  /** Lage des Stichart-Menüs neben der Werkzeugleiste */
  let stichartPos = $state({ x: 0, y: 0 });
  let spruenge = $state(true);
  let einstiche = $state(false);
  let raster = $state(true);
  let zoom = $state(1);
  let vorschau: Vorschau | undefined = $state();
  type Werkzeug = "auswahl" | "einstich" | "ausstich" | "knoten" | "zeichnen";
  let werkzeug = $state<Werkzeug>((globalThis as { __START__?: { werkzeug?: Werkzeug } }).__START__?.werkzeug ?? "auswahl");
  /** Form beim Zeichnen mit der Maus (Nachtauftrag N0) */
  let zeichenForm = $state<FormWerkzeug>(
    (globalThis as { __START__?: { zeichenForm?: FormWerkzeug } }).__START__?.zeichenForm ?? "linie",
  );
  const knotenZustand = $derived(werkzeug === "knoten" && arbeit.auswahl.length === 1 ? knotenBild(arbeit.objekte[arbeit.auswahl[0]]) : { knoten: [], linien: [], umwandeln: false });

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
    arbeit.auswahlTransformieren((t) => {
      const [vx, vy] = t.verschieben ?? [0, 0];
      t.verschieben = [Math.round((vx + dx) * 100) / 100, Math.round((vy + dy) * 100) / 100];
    }, "pfeil");
  }

  const leer = $derived(arbeit.objekte.length === 0);
  const kopfInfo = $derived(
    [arbeit.design.rahmen?.replace(/^artista /, ""), arbeit.design.stoff].filter(Boolean).join(" · ") +
      (arbeit.pfad ? ` · ${dateiname(arbeit.pfad)}${arbeit.geaendert ? " *" : ""}` : arbeit.geaendert ? " · nicht gespeichert" : ""),
  );

  type Eintrag = { l: string; go?: () => void; kbd?: string; spaeter?: string } | "-";
  const dateiMenue: Eintrag[] = [
    { l: "Neu", go: neu, kbd: "Strg+N" },
    { l: "Öffnen … (auch DST, PES, JEF, VP3, EXP)", go: oeffnen, kbd: "Strg+O" },
    { l: "Speichern", go: () => speichern(), kbd: "Strg+S" },
    { l: "Speichern unter …", go: () => speichern(true) },
    "-",
    { l: "Beispiel laden", go: beispiel },
    { l: "Bild umwandeln …", go: () => (arbeit.dialog = { art: "bild" }) },
    { l: "Zauberstab (Flächen im Bild anklicken) …", go: () => (arbeit.dialog = { art: "bild", modus: "zauber" }) },
    { l: "PhotoSnap (Foto) …", go: () => (arbeit.dialog = { art: "bild", modus: "photo" }) },
    { l: "SVG einlesen …", go: () => (arbeit.dialog = { art: "svg" }) },
    "-",
    { l: "Stickdatei exportieren (EXP) …", go: expExportieren, kbd: "Strg+E" },
    { l: "Designs aus der Maschine …", go: () => maschine.listeOeffnen() },
    "-",
    { l: "Darstellung hell/dunkel umschalten", go: () => (arbeit.dunkel = !arbeit.dunkel) },
  ];

  /** Garn des Farbblocks, mit dem ein Objekt beginnt (Anzeige „jetzt: …“) */
  /** Farben des Monogramm-Rahmens aus dem Katalog: eigene (`rahmen_farben`), sonst die der Vorlage */
  function monogrammRahmenFarben(o: number): string[] {
    const x = arbeit.objekte[o];
    if (x?.art !== "monogramm") return [];
    const v = listen.monogrammRahmen(x.vorlage);
    return (v?.farben ?? []).map((f, k) => x.rahmen_farben?.[k] || f);
  }
  function garnVon(o: number): GarnInfo | undefined {
    return arbeit.garnVon(o);
  }
  function garnSetzen(o: number, g: GarnInfo, alle?: number[]) {
    arbeit.aendern((d) => {
      for (const i of alle?.length ? alle : [o]) {
        const x = d.objekte?.[i];
        if (!x) continue;
        x.garn = g.nummer;
        x.farbe = g.rgb;
      }
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

  /** Craft-Stitch-Vorlage (Kern `craft_vorlagen`) einfügen: Linie bzw. Fläche in der Rahmenmitte mit den Werten des Originals */
  function craftEinfuegen(v: CraftVorlage) {
    const form =
      v.form === "linie"
        ? { punkte: [[-25, -6], [-8, 6], [8, -6], [25, 6]] }
        : { umriss: { kreis: { mitte: [0, 0], radius: 20 } } };
    arbeit.einfuegen({ ...(v.felder as object), ...form, name: v.name, farbe: STANDARD_FARBE } as ObjektJson);
  }
  // Vorlagen gleich beim Start laden, damit das Menü sofort vollständig ist
  $effect(() => void listen.craftLaden().catch(() => {}));

  /** Form-Werkzeug: mit der Maus zeichnen (Punkt für Punkt wie im Original, Handbuch S. 58) */
  function formZeichnen(form: FormWerkzeug) {
    zeichenForm = form;
    werkzeug = "zeichnen";
    arbeit.ansicht = arbeit.ansicht === "abspielen" ? "stiche" : arbeit.ansicht;
  }
  /** Gezeichnete Form als Objekt mit der vorgewählten Stichart einfügen (Umriss über den Kern `stichart_aendern`) */
  async function gezeichnet(form: FormWerkzeug, punkte: ZeichenPunkt[], quadrat: boolean) {
    const r = formAusPunkten(form, punkte, arbeit.vorwahl, quadrat);
    if (!r) return arbeit.zeige("Form zu klein – bitte noch einmal zeichnen.", "info");
    arbeit.einfuegen(r.objekt);
    if (r.umriss) await arbeit.stichartAendern([arbeit.objekte.length - 1], r.umriss);
  }
  /** Stichart-Menü (Füllung/Umriss) neben der Werkzeugleiste öffnen */
  function stichartMenue(modus: "fuellung" | "umriss", ev: MouseEvent) {
    arbeit.vorwahl.modus = modus;
    const r = (ev.currentTarget as HTMLElement).getBoundingClientRect();
    stichartPos = { x: r.right + 6, y: r.top };
    menue = menue === modus ? null : modus;
  }
  /** Stichart wählen: Vorwahl für neue Formen und – wie im Original – für die gewählten Linien/Flächen */
  function stichartWaehlen(modus: "fuellung" | "umriss", art: string) {
    arbeit.vorwahl.modus = modus;
    arbeit.vorwahl[modus] = art;
    const passend = arbeit.auswahl.filter((i) => WECHSELBAR.includes(arbeit.objekte[i]?.art ?? ""));
    if (passend.length) arbeit.stichartAendern(passend, art);
  }
  const auswahlWechselbar = $derived(arbeit.auswahl.some((i) => WECHSELBAR.includes(arbeit.objekte[i]?.art ?? "")));
  const formTitel = (f: (typeof FORMEN)[number]) =>
    f.form === "linie"
      ? `${f.name} – wird ${STICHART_NAME[arbeit.vorwahl.umriss]} (Umriss)`
      : f.form === "saeule"
        ? `${f.name} – ${f.hilfe}`
        : `${f.name} – wird ${arbeit.vorwahl.modus === "fuellung" ? `Fläche: ${STICHART_NAME[arbeit.vorwahl.fuellung]}` : `Umriss: ${STICHART_NAME[arbeit.vorwahl.umriss]}`}`;

  /** Menü Anordnen: Befehle auf die Auswahl (`reihenfolge_aendern`) */
  const auswahlSortiert = $derived([...arbeit.auswahl].sort((a, b) => a - b));
  const anordnenMenue = $derived.by((): { l: string; kbd?: string; go?: () => void }[] => {
    const a = arbeit.auswahl;
    const n = arbeit.objekte.length;
    const erst = auswahlSortiert[0] ?? -1;
    const letzt = auswahlSortiert.at(-1) ?? -1;
    const um = (b: ReihenfolgeBefehl) => () => arbeit.umordnen(b);
    const nachNummer = (vor: boolean) => () => (arbeit.dialog = { art: "nummer", vor });
    return [
      { l: "An den Anfang", go: a.length ? um({ art: "an_anfang", auswahl: [...a] }) : undefined },
      { l: "Ans Ende", go: a.length ? um({ art: "ans_ende", auswahl: [...a] }) : undefined },
      { l: "Eins nach vorn", kbd: "Alt+↑", go: a.length && erst > 0 ? um({ art: "vor", auswahl: [...a], ziel: erst - 1 }) : undefined },
      { l: "Eins nach hinten", kbd: "Alt+↓", go: a.length && letzt < n - 1 ? um({ art: "nach", auswahl: [...a], ziel: letzt + 1 }) : undefined },
      { l: "Wie ausgewählt (Klickfolge)", go: a.length > 1 ? um({ art: "wie_ausgewaehlt", auswahl: [...a] }) : undefined },
      { l: "Vor Objekt Nr. …", go: a.length ? nachNummer(true) : undefined },
      { l: "Nach Objekt Nr. …", go: a.length ? nachNummer(false) : undefined },
      { l: "Nach Farbe sortieren …", go: n > 1 ? () => (arbeit.dialog = { art: "nach_farbe" }) : undefined },
      { l: "Gruppieren", kbd: "Strg+G", go: a.length > 1 && !arbeit.auswahlGruppe ? () => arbeit.gruppieren() : undefined },
      { l: "Gruppe auflösen", kbd: "Strg+Umschalt+G", go: a.some((i) => arbeit.gruppeVon(i)) ? () => arbeit.gruppeAufloesen() : undefined },
      { l: "Als Motiv speichern …", go: a.length ? () => (arbeit.dialog = { art: "motiv_speichern" }) : undefined },
      { l: "Blackwork Run (Linien in einem Zug)", go: arbeit.blackworkFaehig.length > 1 ? () => arbeit.blackworkLauf() : undefined },
      { l: "Teil-Abdeckung (Partial Appliqué)", go: arbeit.auswahlApplikationen.length > 1 ? () => arbeit.teilAbdeckung() : undefined },
    ];
  });

  /** Ausrichten (Arrange-Leiste des Originals, STR 33233–33239): nach dem zuletzt gewählten Objekt */
  const AUSRICHTEN: [AusrichtArt, string, string][] = [
    ["links", IC.ausrLinks, "Linke Kanten"],
    ["mitte_senkrecht", IC.ausrMitteSenkrecht, "Mitten senkrecht übereinander"],
    ["rechts", IC.ausrRechts, "Rechte Kanten"],
    ["oben", IC.ausrOben, "Obere Kanten"],
    ["mitte_waagrecht", IC.ausrMitteWaagrecht, "Mitten waagrecht nebeneinander"],
    ["unten", IC.ausrUnten, "Untere Kanten"],
    ["mitte", IC.ausrMitte, "Mitten aufeinander"],
  ];

  function waehle(f?: () => void) {
    menue = null;
    f?.();
  }

  function tasten(ev: KeyboardEvent) {
    const strg = ev.ctrlKey || ev.metaKey;
    const ziel = ev.target as HTMLElement;
    const eingabe = ziel?.closest?.("input, textarea, select, [contenteditable]");
    if (arbeit.dialog || maschine.dialogOffen || maschine.listeOffen || maschine.stickenDialog) return; // Dialog behandelt Esc selbst
    if (!eingabe && werkzeug === "zeichnen" && vorschau?.zeichenTaste(ev.key)) return ev.preventDefault();
    if (ev.key === "Escape") {
      if (werkzeug !== "auswahl") werkzeug = "auswahl";
      else if (menue) menue = null;
      else arbeit.waehlen(null);
      return;
    }
    if (arbeit.modus === "experte" && !eingabe && werkzeug === "knoten" && vorschau?.knotenTaste(ev.key)) return ev.preventDefault();
    // Entf (Windows) bzw. Löschtaste (Mac, ⌫) löscht die gewählten Objekte – in beiden Modi
    if (!eingabe && (ev.key === "Delete" || ev.key === "Backspace") && arbeit.auswahl.length && arbeit.ansicht !== "abspielen")
      return (ev.preventDefault(), arbeit.loeschen());
    if (arbeit.modus === "experte" && !eingabe) {
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
    // Strg+A wählt alle Objekte (Original „Select All“), nicht den Text der Seite; Strg+G gruppieren – beide Modi
    if (k === "a" && !eingabe && arbeit.ansicht !== "abspielen") return (ev.preventDefault(), arbeit.alleWaehlen());
    if (k === "g" && !eingabe) return (ev.preventDefault(), ev.shiftKey ? arbeit.gruppeAufloesen() : arbeit.gruppieren());
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
    <button class="btn btn-primary blueprint" style="margin-left:6px; gap:7px" disabled={!!maschine.sperrgrund} title={maschine.sperrgrund ?? "An die Maschine senden (erst Plan, dann bestätigen)"} onclick={() => maschine.sendenOeffnen()}>
      <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>
      <Icon d={IC.senden} />Senden …
    </button>

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
          <div class="mh">Form zeichnen – {arbeit.vorwahl.modus === "fuellung" ? `Fläche: ${STICHART_NAME[arbeit.vorwahl.fuellung]}` : `Umriss: ${STICHART_NAME[arbeit.vorwahl.umriss]}`}</div>
          {#each FORMEN as f (f.form)}
            <button class="mi" role="menuitem" onclick={() => waehle(() => formZeichnen(f.form))}><Icon d={f.icon} />{f.name}</button>
          {/each}
          <div class="msep"></div>
          <div class="mh">Schrift</div>
          {#each SCHRIFT as a (a)}
            <button class="mi" role="menuitem" onclick={() => waehle(() => einfuegen(a))}><Icon d={ART[a][1]} />{ART[a][0]}</button>
          {/each}
          <div class="msep"></div>
          <div class="mh">Besondere</div>
          {#each BESONDERE as a (a)}
            <button class="mi" role="menuitem" onclick={() => waehle(() => einfuegen(a))}><Icon d={ART[a][1]} />{ART[a][0]}</button>
          {/each}
          <div class="msep"></div>
          <button class="mi" role="menuitem" onclick={() => waehle(() => (arbeit.dialog = { art: "bild" }))}><Icon d={IC.bild} />Bild umwandeln …</button>
          <button class="mi" role="menuitem" onclick={() => waehle(() => (arbeit.dialog = { art: "bild", modus: "zauber" }))}><Icon d={IC.zauberstab} />Zauberstab …</button>
          <button class="mi" role="menuitem" onclick={() => waehle(() => (arbeit.dialog = { art: "bild", modus: "photo" }))}><Icon d={ART.photosnap[1]} />PhotoSnap (Foto) …</button>
          <button class="mi" role="menuitem" onclick={() => waehle(() => (arbeit.dialog = { art: "svg" }))}><Icon d={IC.bezier} />SVG einlesen …</button>
          <button class="mi" role="menuitem" title="Gespeichertes Design (JSON) oder fremde Stickdatei (DST, PES, JEF, VP3, EXP) als ein Element" onclick={() => waehle(designEinfuegen)}><Icon d={ART.stiche[1]} />Design oder Stickdatei einfügen …</button>
        </div>
        <div style="padding:4px; background:var(--panel)">
          <div class="mh">Craft Stitches – Handstich-Optik</div>
          {#each listen.craft as v (v.id)}
            <button class="mi" role="menuitem" title={v.beschreibung} onclick={() => waehle(() => craftEinfuegen(v))}>
              <Icon d={ART[(v.felder as { art: string }).art]?.[1] ?? ""} /><span style="flex:1">{v.name}</span><span class="mut small">{v.form === "linie" ? "Linie" : "Fläche"}</span>
            </button>
          {/each}
          <div class="small mut" style="padding:2px 10px 6px; white-space:normal">Linien in einem Zug: mehrere Linien wählen → Anordnen → „Blackwork Run“.</div>
          <div class="msep"></div>
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
        <div class="small mut" style="padding:2px 10px 6px; max-width:280px; white-space:normal">Liegt ein Objekt danach unter einer Fläche, über der es lag, warnt das Programm.</div>
        <div class="msep"></div>
        <div class="mh">Ausrichten – am zuletzt gewählten</div>
        <div class="ausr" role="group" aria-label="Ausrichten">
          {#each AUSRICHTEN as [art, d, l] (art)}
            <button class="tb" role="menuitem" title={l} aria-label={l} disabled={arbeit.auswahlEinheiten < 2} onclick={() => waehle(() => arbeit.ausrichten(art))}><Icon {d} /></button>
          {/each}
        </div>
        <div class="mh">Kopieren</div>
        <button class="mi" role="menuitem" disabled={!arbeit.auswahl.length} onclick={() => waehle(() => (arbeit.dialog = { art: "kopieren", weg: "spiegeln" }))}><Icon d={IC.spiegelnKopie} />Spiegeln und kopieren (Mirror-Merge) …</button>
        <button class="mi" role="menuitem" disabled={!arbeit.auswahl.length} onclick={() => waehle(() => (arbeit.dialog = { art: "kopieren", weg: "kranz" }))}><Icon d={IC.kranz} />Kranz / Kaleidoskop …</button>
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
  {#if menue === "fuellung" || menue === "umriss"}
    {@const modus = menue}
    <div class="menu" role="menu" aria-label={modus === "fuellung" ? "Stichart Füllung" : "Stichart Umriss"} style="position:fixed; left:{stichartPos.x}px; top:{stichartPos.y}px; width:340px">
      <div class="mh">{modus === "fuellung" ? "Füllung (Fläche)" : "Umriss (Linie)"} – Stichart</div>
      {#each modus === "fuellung" ? FUELLUNG : UMRISS as a (a)}
        {@const an = arbeit.vorwahl[modus] === a}
        <button class="mi" role="menuitemradio" aria-checked={an} onclick={() => waehle(() => stichartWaehlen(modus, a))} style="white-space:normal">
          <Icon d={ART[a]?.[1] ?? ""} />
          <span style="flex:1"><b style="font-weight:500">{STICHART_NAME[a]}</b> <span class="mut small">– {STICHART_HILFE[a]}</span></span>
          {#if an}<Icon d={IC.haken} />{/if}
        </button>
      {/each}
      <div class="msep"></div>
      <div class="small mut" style="padding:4px 10px 8px; white-space:normal">
        Gilt für neue Formen{auswahlWechselbar ? " und für die gewählten Linien/Flächen (Form bleibt)" : ""}. Wie im Original jederzeit änderbar.
      </div>
    </div>
  {/if}

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
      <Assistent />
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
          <div class="rail" role="toolbar" aria-label="Werkzeuge" aria-orientation="vertical" data-tip-rechts>
            <button class="tb" aria-pressed={werkzeug === "auswahl"} aria-label="Auswählen und Griffe" title="Auswählen, ziehen, Griffe (skalieren, drehen) · Esc" onclick={() => (werkzeug = "auswahl")}><Icon d={IC.zeiger} groesse="l" /></button>
            <button class="tb" aria-pressed={werkzeug === "knoten"} disabled={arbeit.auswahl.length !== 1} aria-label="Knoten bearbeiten" title="Knoten bearbeiten – Punkte ziehen; Entf löscht, Einfg fügt ein" onclick={() => ((werkzeug = "knoten"), (arbeit.ansicht = "form"))}><Icon d={IC.knoten} groesse="l" /></button>
            <button class="tb" aria-pressed={werkzeug === "einstich"} disabled={arbeit.auswahl.length !== 1} aria-label="Einstich setzen" title="Einstich (E) des gewählten Objekts setzen – hier beginnt es" onclick={() => (werkzeug = "einstich")}><Icon d={IC.einstich} groesse="l" /></button>
            <button class="tb" aria-pressed={werkzeug === "ausstich"} disabled={arbeit.auswahl.length !== 1} aria-label="Ausstich setzen" title="Ausstich (A) des gewählten Objekts setzen – hier endet es" onclick={() => (werkzeug = "ausstich")}><Icon d={IC.ausstich} groesse="l" /></button>
            <span class="sep"></span>
            <button class="tb stw" class:an={arbeit.vorwahl.modus === "fuellung"} aria-haspopup="menu" aria-expanded={menue === "fuellung"} aria-label="Füllung: {STICHART_NAME[arbeit.vorwahl.fuellung]}" title="Füllung: {STICHART_NAME[arbeit.vorwahl.fuellung]} – neue Formen werden Flächen · Klick: Stichart wählen (auch für die Auswahl)" onclick={(ev) => stichartMenue("fuellung", ev)}><Icon d={ART[arbeit.vorwahl.fuellung]?.[1] ?? ""} groesse="l" /><span class="stw-l">Fläche</span></button>
            <button class="tb stw" class:an={arbeit.vorwahl.modus === "umriss"} aria-haspopup="menu" aria-expanded={menue === "umriss"} aria-label="Umriss: {STICHART_NAME[arbeit.vorwahl.umriss]}" title="Umriss: {STICHART_NAME[arbeit.vorwahl.umriss]} – Linien und, wenn gewählt, Formen als Umriss · Klick: Stichart wählen (auch für die Auswahl)" onclick={(ev) => stichartMenue("umriss", ev)}><Icon d={ART[arbeit.vorwahl.umriss]?.[1] ?? ""} groesse="l" /><span class="stw-l">Umriss</span></button>
            <span class="sep"></span>
            {#each FORMEN as f (f.form)}
              <button class="tb" aria-pressed={werkzeug === "zeichnen" && zeichenForm === f.form} aria-label="{f.name} zeichnen" title={formTitel(f)} onclick={() => formZeichnen(f.form)}><Icon d={f.icon} groesse="l" /></button>
            {/each}
            <span class="sep"></span>
            {#each ["text", "monogramm"] as a (a)}
              <button class="tb" aria-label="{ART[a][0]} einfügen" title="{ART[a][0]} einfügen" onclick={() => einfuegen(a)}><Icon d={ART[a][1]} groesse="l" /></button>
            {/each}
            <span class="sep"></span>
            {#each BESONDERE as a (a)}
              <button class="tb" aria-label="{ART[a][0]} einfügen" title="{ART[a][0]} einfügen" onclick={() => einfuegen(a)}><Icon d={ART[a][1]} groesse="l" /></button>
            {/each}
            <span class="sep"></span>
            <button class="tb" aria-label="Weitere Werkzeuge – kommen noch" title="Kommt noch: mit der Maus zeichnen, Motivfüllung, Effekte, Kreuzstich …" onclick={() => (menue = "einfuegen")} style="border:1px dashed var(--muted)"><Icon d={IC.kommtNoch} groesse="l" /></button>
          </div>
        {/if}
        <div class="stage">
          <Vorschau bind:this={vorschau} bind:zoom bind:werkzeug {zeichenForm} {gezeichnet} {spruenge} {einstiche} {raster} bis={arbeit.ansicht === "abspielen" ? bis : null} />
          {#if werkzeug === "knoten"}
            <div class="floater small" style="left:12px; bottom:12px; padding:7px 10px; max-width:520px">
              {#if knotenZustand.umwandeln}
                <div style="display:flex; gap:10px; align-items:center; margin-bottom:6px">
                  <span style="flex:1">Dieses Objekt hat {knotenZustand.knoten.length ? "auch " : ""}Formen (Kreis, Oval, Bogen, Rechteck) oder Größe/Drehung – zum Bearbeiten erst in Knoten umwandeln.</span>
                  <button class="btn btn-primary" style="min-height:28px; padding:2px 10px" onclick={() => arbeit.inKnoten(arbeit.auswahl[0])}>In Knoten umwandeln</button>
                </div>
              {/if}
              <b style="font-weight:500">Knoten</b> – Knoten (□ Ecke, ◯ Kurvenpunkt) und Anfasser (○) ziehen · Rechtsklick auf einen Knoten: Ecke ↔ Kurve · gewählten Punkt mit Entf löschen, mit Einfg (oder +) dahinter einen einfügen · Esc beendet.
            </div>
          {:else if werkzeug === "zeichnen"}
            <div class="floater small" style="left:12px; bottom:12px; padding:7px 10px; max-width:560px; display:flex; gap:10px; align-items:center">
              <span style="flex:1"><b style="font-weight:500">{FORMEN.find((f) => f.form === zeichenForm)?.name} zeichnen</b> – {ZEICHEN_HILFE[zeichenForm]} · ⌫ letzter Punkt zurück · Esc beendet</span>
              <button class="btn btn-primary" style="min-height:28px; padding:2px 10px" onclick={() => vorschau?.zeichnenAbschliessen()}>Fertig</button>
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
          {#if leer && !arbeit.rechnet && werkzeug !== "zeichnen"}
            {#if arbeit.modus === "experte"}
              <div class="floater" style="left:50%; top:50%; transform:translate(-50%,-50%); width:380px; padding:18px">
                <h3 style="margin:0 0 4px; font-size:21px">Leeres Design</h3>
                <p class="mut" style="margin:0 0 12px">Links eine Form wählen und Punkt für Punkt zeichnen, Schrift oder Monogramm einfügen – oder:</p>
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
  {#if arbeit.dialog?.art === "motiv"}
    {@const d = arbeit.dialog}
    {@const x = arbeit.objekte[d.objekt] as Record<string, unknown> | undefined}
    <MotivDialog
      satz={String(x?.satz ?? (x?.art === "motivfuellung" ? "Children" : "Heirloom"))}
      motiv={String(x?.motiv ?? (x?.art === "motivfuellung" ? "901" : "705"))}
      waehlen={(satz, motiv) => {
        arbeit.aendern((dd) => {
          const y = dd.objekte?.[d.objekt] as Record<string, unknown> | undefined;
          if (!y) return;
          y.satz = satz;
          y.motiv = motiv;
        });
        arbeit.dialog = null;
      }}
      schliessen={() => (arbeit.dialog = null)}
    />
  {/if}
  {#if arbeit.dialog?.art === "nach_farbe"}<NachFarbeDialog schliessen={() => (arbeit.dialog = null)} />{/if}
  {#if arbeit.dialog?.art === "motiv_speichern"}<MotivSpeichernDialog schliessen={() => (arbeit.dialog = null)} />{/if}
  {#if arbeit.dialog?.art === "kopieren"}<KopierenDialog weg={arbeit.dialog.weg} schliessen={() => (arbeit.dialog = null)} />{/if}
  {#if arbeit.dialog?.art === "eigene_rahmen"}<EigeneRahmenDialog schliessen={() => (arbeit.dialog = null)} />{/if}
  {#if arbeit.dialog?.art === "nummer"}<NummerDialog vor={arbeit.dialog.vor} schliessen={() => (arbeit.dialog = null)} />{/if}
  {#if arbeit.dialog?.art === "farben_mischen"}<FarbenMischenDialog objekt={arbeit.dialog.objekt} schliessen={() => (arbeit.dialog = null)} />{/if}
  {#if arbeit.dialog?.art === "warnung"}
    <WarnungDialog vertauscht={arbeit.dialog.vertauscht} namen={arbeit.dialog.namen} vorher={arbeit.dialog.vorher} schliessen={() => (arbeit.dialog = null)} />
  {/if}
  {#if maschine.dialogOffen}<SendenDialog />{/if}
  {#if maschine.stickenDialog}<StickenDialog />{/if}
  {#if maschine.listeOffen}<AusMaschineDialog />{/if}
  {#if arbeit.dialog?.art === "rahmen"}
    {@const o = arbeit.dialog.objekt}
    {@const x = arbeit.objekte[o]}
    <RahmenDialog
      aktuell={x?.art === "monogramm" ? { vorlage: x.vorlage ?? undefined, rahmen: x.rahmen ?? undefined } : {}}
      waehlen={(w) => {
        arbeit.monogrammRahmen(o, w);
        arbeit.dialog = null;
      }}
      schliessen={() => (arbeit.dialog = null)}
    />
  {:else if arbeit.dialog?.art === "schrift"}
    {@const o = arbeit.dialog.objekt}
    {@const x = arbeit.objekte[o]}
    {@const mono = x?.art === "monogramm" ? x : undefined}
    {@const vorlage = listen.monogrammRahmen(mono?.vorlage)}
    <SchriftDialog
      start={arbeit.dialog.reiter}
      aktuell={mono
        ? mono.alphabet
          ? { alphabet: mono.alphabet }
          : { schrift: mono.schrift ?? vorlage?.schrift ?? undefined }
        : x && "alphabet" in x
          ? { alphabet: x.alphabet ?? undefined }
          : x && "schrift" in x
            ? { schrift: x.schrift ?? undefined }
            : {}}
      text={mono ? mono.initialen : x && "text" in x ? x.text : undefined}
      hoehe={mono && vorlage ? (Math.min(...vorlage.hoehen) * (mono.rahmen_groesse ?? vorlage.groesse)) / vorlage.groesse : x && "hoehe" in x ? x.hoehe : undefined}
      waehlen={(w) => {
        // erst die Nummer lesen: `o` hängt am offenen Dialog und ist danach nicht mehr lesbar
        if (mono) arbeit.monogrammSchrift(o, w);
        else arbeit.schriftWechseln(o, w);
        arbeit.dialog = null;
      }}
      schliessen={() => (arbeit.dialog = null)}
    />
  {:else if arbeit.dialog?.art === "garn"}
    {@const o = arbeit.dialog.objekt}
    {@const furUmrandung = !!arbeit.dialog.umrandung}
    {@const rahmenNr = arbeit.dialog.rahmen}
    {@const rahmenFarbe = rahmenNr != null ? monogrammRahmenFarben(o)[rahmenNr] : undefined}
    <GarnDialog
      titel={furUmrandung ? "Garn der Umrandung" : rahmenNr != null ? `Garn des Rahmens (Farbe ${rahmenNr + 1})` : undefined}
      untertitel={arbeit.dialog.titel ?? arbeit.objekte[o]?.name ?? ""}
      aktuell={furUmrandung ? arbeit.umrandungGarn(o) : rahmenFarbe ? arbeit.farbGarn(o, rahmenFarbe) : garnVon(o)}
      waehlen={(g) => {
        const alle = arbeit.dialog?.art === "garn" ? arbeit.dialog.alle : undefined;
        if (rahmenNr != null) {
          const farben = monogrammRahmenFarben(o);
          farben[rahmenNr] = g.rgb;
          arbeit.aendern((d) => {
            const x = d.objekte?.[o];
            if (x?.art === "monogramm") x.rahmen_farben = farben;
          });
          arbeit.dialog = null;
          return;
        }
        if (!furUmrandung) return garnSetzen(o, g, alle);
        arbeit.umrandungSetzen(alle?.length ? alle : [o], (u) => (u.farbe = g.rgb));
        arbeit.dialog = null;
      }}
      schliessen={() => (arbeit.dialog = null)}
    />
  {:else if arbeit.dialog?.art === "bild" || arbeit.dialog?.art === "svg"}
    <EinlesenDialog art={arbeit.dialog.art} startPfad={arbeit.dialog.art === "bild" ? arbeit.dialog.pfad : undefined} startModus={arbeit.dialog.art === "bild" ? arbeit.dialog.modus : undefined} schliessen={() => (arbeit.dialog = null)} />
  {/if}
</div>
