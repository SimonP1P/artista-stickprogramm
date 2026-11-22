// Zustand des Programms: **ein Design als JSON** (Format des Kerns, `kern/schnittstelle/kern.ts`), Rückgängig/Wiederholen,
// Auswahl, Modus – und die Ergebnisse des Kerns. Die Oberfläche rechnet keine Geometrie und keine Stiche: jede
// Änderung schickt das Design (entprellt) an `design_berechnen` und `design_formen` (UEBERGABE §3, CLAUDE.md §4 Regel 7).

import {
  kern,
  istFehler,
  type DesignJson,
  type Ergebnis,
  type Fehler,
  type GarnInfo,
  type MonogrammJson,
  type ObjektForm,
  type ObjektJson,
  type Pruefung,
  type RahmenInfo,
  type ReihenfolgeBefehl,
  type Transformation,
  type UmrandungJson,
} from "$kern/kern";
import { listen } from "./listen.svelte";
import type { RahmenWahl } from "./dialoge/RahmenDialog.svelte";

export type Modus = "einfach" | "experte";
/** Wahl im Dialog „Schrift wählen“: Original-Alphabet oder Computer-Schrift */
export type SchriftWahl = { alphabet?: string; schrift?: string };
/** offener Dialog (gerendert in +page.svelte) */
export type OffenerDialog =
  | { art: "schrift"; objekt: number; reiter?: "alphabete" | "computer" | "google" | "favoriten" }
  | {
      art: "garn";
      objekt: number;
      titel?: string;
      /** alle diese Objekte bekommen das Garn (z. B. eine Farbe eines Bilds) */
      alle?: number[];
      /** Garn für die Umrandung (`umrandung.farbe`) statt für das Objekt */
      umrandung?: boolean;
      /** Garn für eine Farbe des Monogramm-Rahmens (Nummer in `rahmen_farben`) statt für die Initialen */
      rahmen?: number;
    }
  | { art: "rahmen"; objekt: number }
  | { art: "bild"; pfad?: string }
  | { art: "svg" }
  | { art: "nach_farbe" }
  | { art: "muster"; objekt: number }
  | { art: "step"; objekt: number }
  | { art: "motiv"; objekt: number }
  | { art: "warnung"; vertauscht: number[][]; namen: string[]; vorher: string };
export type Ansicht = "stiche" | "form" | "abspielen";

/** Neues, leeres Design mit den Standardwerten des Formats (`design.schema.json`). */
export function leeresDesign(): DesignJson {
  return {
    name: "Design",
    rahmen: "artista 130 x 100",
    stoff: "0 Software Defaults",
    garnkarte: "Default",
    objekte: [],
  };
}

/** Fehler aus einem abgelehnten `kern(…)`-Aufruf. */
export function alsFehler(x: unknown): Fehler {
  return istFehler(x) ? x : { meldung: String(x), objekt: null };
}

/** Startoptionen aus der Befehlszeile: `artista-stickprogramm [DATEI.json] [--experte] [--dunkel|--hell] [--auswahl N]` */
type StartOptionen = {
  datei?: string;
  modus?: Modus;
  dunkel?: boolean;
  auswahl?: number;
  /** gewählter Buchstabe (Nr.) des Objekts `auswahl` – nur für Bildschirmfotos */
  buchstabe?: number;
  ansicht?: Ansicht;
  /** nur für Bildschirmfotos (app/werkzeuge/bildschirmfoto.py) */
  schritt?: number;
  dialog?: OffenerDialog;
  /** Maschinen-Zustand für Bildschirmfotos: Stufe des Sendens bzw. Liste (ausgeführt in +page.svelte) */
  maschine?: "anschluss" | "plan" | "fertig" | "sticken" | "liste";
};

/** Element des einfachen Modus: eine Gruppe oder ein einzelnes Objekt */
export type Element = {
  /** Schlüssel der Gruppe (sonst ein einzelnes Objekt) */
  gruppe?: string;
  /** Objektart bzw. Herkunft der Gruppe (bild, svg, gruppiert) */
  art: string;
  /** Objektnummern */
  objekte: number[];
  name: string;
};

/** Anzeigename eines Objekts: Name, sonst Text/Initialen, sonst Nr. */
export function objektName(o: ObjektJson | undefined, i: number): string {
  if (!o) return `Nr. ${i + 1}`;
  const x = o as { name?: string; text?: string; initialen?: string };
  return x.name || x.text?.split("\n")[0] || x.initialen || `Nr. ${i + 1}`;
}

/** Objektarten, deren Stichart sich wechseln lässt (Kern `stichart_aendern`) */
/** Objektarten mit Fläche: dort wirkt `umrandung` (Kern: `aussparen::bedeckt`); Linien haben keine */
export const MIT_FLAECHE = ["tatami", "satinflaeche", "muster", "satin", "satinlinie", "blanket", "text", "ttf", "monogramm", "applikation"];

export const WECHSELBAR = ["lauf", "dreifach", "satinlinie", "blanket", "motivlinie", "tatami", "satinflaeche", "muster"];
/** Standardname je Stichart (neue Objekte); ein Objekt mit Standardnamen bekommt beim Wechsel den neuen */
export const STANDARD_NAME: Record<string, string> = {
  lauf: "Laufstich",
  dreifach: "Dreifachstich",
  satinlinie: "Satinlinie",
  blanket: "Blanket",
  motivlinie: "Motivlinie",
  tatami: "Fläche",
  satinflaeche: "Satinfläche",
  muster: "Füllmuster",
};
const STANDARD_NAMEN = Object.values(STANDARD_NAME);

/** freier Gruppenschlüssel g1, g2 … */
function freierSchluessel(gruppen: Record<string, unknown>): string {
  let n = 1;
  while (`g${n}` in gruppen) n++;
  return `g${n}`;
}

/** Gruppen ohne Objekte entfernen */
/**
 * Gruppe `g` auflösen: die Objekte verlieren `gruppe`, eine reine Verschiebung der Gruppe geht an jedes Objekt über
 * (Drehen/Skalieren wirkt dort um die Mitte des Objekts statt um die der Gruppe und lässt sich nicht verlustfrei
 * übertragen). Ergebnis: `true`, wenn dabei Größe/Drehung der Gruppe verloren geht.
 */
function gruppeInObjekte(d: DesignJson, g: string): boolean {
  const t = d.gruppen?.[g]?.transform;
  const leer = !t || Object.keys(t).length === 0;
  let verloren = false;
  for (const o of d.objekte ?? []) {
    if (o.gruppe !== g) continue;
    delete o.gruppe;
    if (leer || !t) continue;
    const nurSchieben = Object.keys(t).every((k) => k === "verschieben");
    if (nurSchieben && t.verschieben) {
      const ot = (o.transform ??= {});
      const [vx, vy] = ot.verschieben ?? [0, 0];
      ot.verschieben = [vx + t.verschieben[0], vy + t.verschieben[1]];
    } else verloren = true;
  }
  delete d.gruppen?.[g];
  return verloren;
}

function gruppenAufraeumen(d: DesignJson) {
  if (!d.gruppen) return;
  const zahl = new Map<string, number>();
  for (const o of d.objekte ?? []) if (o.gruppe) zahl.set(o.gruppe, (zahl.get(o.gruppe) ?? 0) + 1);
  for (const k of Object.keys(d.gruppen)) if (!zahl.get(k)) delete d.gruppen[k];
  if (!Object.keys(d.gruppen).length) delete d.gruppen;
}

const ENTPRELLEN_MS = 140;
const ZUSAMMENFASSEN_MS = 900;
const VERLAUF_MAX = 200;

class Arbeit {
  /** das Design, das die Oberfläche bearbeitet */
  design = $state<DesignJson>(leeresDesign());
  modus = $state<Modus>("einfach");
  ansicht = $state<Ansicht>("stiche");
  /** Schritt des Assistenten (einfacher Modus): 1 Rahmen & Stoff · 2 Gestalten · 3 Prüfen & Garn · 4 Senden */
  schritt = $state(1);
  dialog = $state<OffenerDialog | null>(null);
  dunkel = $state(false);
  /** gewählte Objekte (Nummern ab 0, Klickfolge) */
  auswahl = $state<number[]>([]);
  /** gewählter Buchstabe eines Texts (W9: Klick auf den Buchstaben des gewählten Texts, `Ergebnis.buchstabe`) */
  buchstabe = $state<{ objekt: number; nr: number } | null>(null);
  /** Datei des Designs (null = noch nicht gespeichert) */
  pfad = $state<string | null>(null);
  geaendert = $state(false);

  // Ergebnisse des Kerns (nicht tief reaktiv – groß)
  ergebnis = $state.raw<Ergebnis | null>(null);
  /** zählt jede fertige Berechnung (Ergebnis oder Fehler) – die Vorschau hält die Live-Griffe bis zur nächsten */
  stand = $state(0);
  formen = $state.raw<ObjektForm[]>([]);
  pruefung = $state.raw<Pruefung | null>(null);
  /** Fehler der letzten Berechnung; die Vorschau zeigt dann den letzten gültigen Stand */
  fehler = $state.raw<Fehler | null>(null);
  rechnet = $state(false);
  /** Paare [früher, später] überdeckender Objekte (`ueberdeckungen`, nur Expertenmodus) */
  ueberdeckungen = $state.raw<number[][]>([]);
  /** Dauer der letzten Berechnung in ms */
  dauer = $state<number | null>(null);

  // Listen des Kerns (einmal geladen)
  rahmenListe = $state.raw<RahmenInfo[]>([]);

  /** Vorwahl der Stichart für neue Formen (wie „Outline/Fill“ + Stichart-Liste des Originals) */
  vorwahl = $state<{ modus: "fuellung" | "umriss"; fuellung: string; umriss: string }>({
    modus: "fuellung",
    fuellung: "tatami",
    umriss: "lauf",
  });

  /** Meldung oben (z. B. „gespeichert“, Fehler beim Öffnen) */
  meldung = $state<{ text: string; art: "ok" | "err" | "info" } | null>(null);

  #zurueck = $state<string[]>([]);
  #vor = $state<string[]>([]);
  #letzteGruppe: { name: string; zeit: number } | null = null;
  #zeitgeber: ReturnType<typeof setTimeout> | null = null;
  #lauf = 0;

  get kannZurueck() {
    return this.#zurueck.length > 0;
  }
  get kannVor() {
    return this.#vor.length > 0;
  }
  get objekte(): ObjektJson[] {
    return this.design.objekte ?? [];
  }
  /** Garn eines Objekts: Farbblock mit der Farbe des Objekts (nicht der eines einzeln gefärbten Buchstabens), sonst
   *  der erste Block, in dem es vorkommt */
  garnVon(i: number): GarnInfo | undefined {
    const e = this.ergebnis;
    if (!e || i < 0) return undefined;
    const farbe = String((this.objekte[i] as { farbe?: string } | undefined)?.farbe ?? "").toLowerCase();
    const drin = (b: { objekt: number; objekte: number[] }) => b.objekt === i || b.objekte.includes(i);
    let k = e.bloecke.findIndex((b, n) => drin(b) && (e.farben[n] ?? "").toLowerCase() === farbe);
    if (k < 0) k = e.bloecke.findIndex(drin);
    return k >= 0 ? e.garne[k] : undefined;
  }
  /** Garn einer Farbe eines Objekts (z. B. einer Farbe des Monogramm-Rahmens): Farbblock des Objekts in dieser Farbe */
  farbGarn(i: number, farbe: string): GarnInfo | undefined {
    const e = this.ergebnis;
    if (!e) return undefined;
    const f = farbe.toLowerCase();
    const k = e.bloecke.findIndex((b, n) => (b.objekt === i || b.objekte.includes(i)) && (e.farben[n] ?? "").toLowerCase() === f);
    return k >= 0 ? e.garne[k] : undefined;
  }
  /** Garn der Umrandung eines Objekts: Farbblock des Objekts in der Farbe der Umrandung */
  umrandungGarn(i: number): GarnInfo | undefined {
    const e = this.ergebnis;
    const u = this.objekte[i]?.umrandung;
    if (!e || !u) return undefined;
    const farbe = (u.farbe ?? "#000000").toLowerCase();
    const k = e.bloecke.findIndex((b, n) => (b.objekt === i || b.objekte.includes(i)) && (e.farben[n] ?? "").toLowerCase() === farbe);
    return k >= 0 ? e.garne[k] : undefined;
  }
  /** Objekte aus `nummern`, die eine Umrandung haben können (Fläche) */
  mitFlaeche(nummern: number[]): number[] {
    return nummern.filter((i) => MIT_FLAECHE.includes(this.objekte[i]?.art ?? ""));
  }
  /**
   * Umrandung (eigene Erweiterung, Kern `umrandung`) an den Objekten mit Fläche ein- oder ausschalten bzw. ändern:
   * `f` bekommt die bisherige Umrandung (oder `{}` beim Einschalten) und ändert sie; `null` schaltet aus. Eine Gruppe
   * (z. B. ein umgewandeltes Bild) bekommt sie an jeder Fläche – Linien zwischen den Farben wie bei einer Zeichnung.
   */
  umrandungSetzen(nummern: number[], f: ((u: UmrandungJson) => void) | null) {
    const ziel = this.mitFlaeche(nummern);
    if (!ziel.length) return;
    // gemeinsame Werte: die der ersten Fläche, die schon eine Umrandung hat
    const vorlage = ziel.map((i) => this.objekte[i]?.umrandung).find((u) => !!u);
    this.aendern((d) => {
      for (const i of ziel) {
        const o = d.objekte?.[i];
        if (!o) continue;
        if (!f) {
          delete o.umrandung;
          continue;
        }
        const u: UmrandungJson = { ...(o.umrandung ?? (vorlage ? JSON.parse(JSON.stringify(vorlage)) : {})) };
        f(u);
        o.umrandung = u;
      }
    });
  }

  /** erstes gewähltes Objekt oder -1 */
  get gewaehlt(): number {
    return this.auswahl.length ? this.auswahl[0] : -1;
  }

  // ------------------------------------------------------------------ Gruppen und Elemente
  /** Gruppe eines Objekts („Group“ des Originals, Feld `gruppe`) */
  gruppeVon(i: number): string | undefined {
    return this.objekte[i]?.gruppe ?? undefined;
  }
  /** Objekte einer Gruppe (Nummern) */
  mitglieder(g: string): number[] {
    return this.objekte.flatMap((o, i) => (o.gruppe === g ? [i] : []));
  }
  /** Gruppe, wenn genau alle Objekte einer Gruppe gewählt sind – dann wirken Griffe, Pfeiltasten, Größe und Drehen
   *  auf die Transformation der Gruppe */
  get auswahlGruppe(): string | null {
    const g = this.auswahl.length ? this.gruppeVon(this.auswahl[0]) : undefined;
    if (!g) return null;
    const m = this.mitglieder(g);
    return m.length === this.auswahl.length && m.every((i) => this.auswahl.includes(i)) ? g : null;
  }
  /**
   * Elemente für den einfachen Modus: jede Gruppe ist **ein** Element (z. B. ein umgewandeltes Bild), jedes Objekt
   * ohne Gruppe ein eigenes – in Stickreihenfolge des ersten Objekts.
   */
  get elemente(): Element[] {
    const out: Element[] = [];
    this.objekte.forEach((o, i) => {
      const g = o.gruppe ?? undefined;
      const da = g ? out.find((e) => e.gruppe === g) : undefined;
      if (da) return void da.objekte.push(i);
      const info = g ? this.design.gruppen?.[g] : undefined;
      out.push({
        gruppe: g,
        art: g ? (info?.herkunft ?? "gruppiert") : o.art,
        objekte: [i],
        name: g ? info?.name || "Gruppe" : objektName(o, i),
      });
    });
    return out;
  }
  /** Element, zu dem die Auswahl gehört (ganz oder teilweise) */
  get elementGewaehlt(): Element | undefined {
    const i = this.gewaehlt;
    return i < 0 ? undefined : this.elemente.find((e) => e.objekte.includes(i));
  }
  /** Element wählen (alle seine Objekte) */
  elementWaehlen(e: Element | undefined) {
    this.buchstabe = null;
    this.auswahl = e ? [...e.objekte] : [];
  }
  /**
   * Transformation der Auswahl ändern: bei einer ganzen Gruppe die der Gruppe (um ihre Mitte), sonst die jedes
   * gewählten Objekts (`nurErstes`: nur das erste – Skalieren/Drehen mit dem Griff).
   */
  auswahlTransformieren(f: (t: Transformation) => void, gruppe?: string, nurErstes = false) {
    const g = this.auswahlGruppe;
    const aus = [...this.auswahl];
    this.aendern((d) => {
      if (g && d.gruppen?.[g]) return void f((d.gruppen[g].transform ??= {}));
      for (const i of nurErstes ? aus.slice(0, 1) : aus) f((d.objekte![i].transform ??= {}));
    }, gruppe);
  }

  /** Gewählte Objekte zu einer neuen Gruppe zusammenfassen („Group“) */
  gruppieren(name = "Gruppe") {
    const aus = [...this.auswahl];
    if (aus.length < 2 || this.auswahlGruppe) return;
    // Gruppen in Gruppen gibt es nicht (ein Feld `gruppe` je Objekt): alte Gruppen der Auswahl erst auflösen
    const alt = new Set(aus.map((i) => this.gruppeVon(i)).filter((g): g is string => !!g));
    let verloren = false;
    this.aendern((d) => {
      for (const g of alt) verloren = gruppeInObjekte(d, g) || verloren;
      d.gruppen ??= {};
      const k = freierSchluessel(d.gruppen);
      d.gruppen[k] = { name, herkunft: "gruppiert" };
      for (const i of aus) d.objekte![i].gruppe = k;
      gruppenAufraeumen(d);
    });
    this.auswahl = aus;
    if (verloren) this.zeige("Gruppiert – Größe/Drehung der alten Gruppe gilt jetzt nicht mehr (Rückgängig möglich)", "info");
  }
  /** Gruppe der gewählten Objekte auflösen („Ungroup“): die Transformation der Gruppe geht an jedes Objekt über,
   *  wenn es selbst keine hat (sonst bleibt sie dort und die der Gruppe entfällt – Hinweis) */
  gruppeAufloesen() {
    const gruppen = new Set(this.auswahl.map((i) => this.gruppeVon(i)).filter((g): g is string => !!g));
    if (!gruppen.size) return;
    let verloren = false;
    this.aendern((d) => {
      for (const g of gruppen) verloren = gruppeInObjekte(d, g) || verloren;
      gruppenAufraeumen(d);
    });
    if (verloren) this.zeige("Gruppe aufgelöst – Größe/Drehung der Gruppe gilt jetzt nicht mehr (Rückgängig möglich)", "info");
  }

  /** Start: Listen laden, Startoptionen der Befehlszeile (`window.__START__`, gesetzt von src-tauri) anwenden. */
  async starten() {
    const start = (globalThis as { __START__?: StartOptionen }).__START__ ?? {};
    if (start.modus) this.modus = start.modus;
    // Darstellung: Startoption, sonst wie das System
    this.dunkel = start.dunkel ?? globalThis.matchMedia?.("(prefers-color-scheme: dark)").matches ?? false;
    if (start.ansicht) this.ansicht = start.ansicht;
    if (start.schritt) this.schritt = start.schritt;
    try {
      this.rahmenListe = await kern("rahmen_liste", {});
    } catch (x) {
      this.zeige(alsFehler(x).meldung, "err");
    }
    if (start.datei) {
      try {
        this.design = JSON.parse(await kern("datei_lesen", { pfad: start.datei }));
        this.pfad = start.datei;
      } catch (x) {
        this.zeige(`Öffnen fehlgeschlagen: ${x instanceof SyntaxError ? x.message : alsFehler(x).meldung}`, "err");
      }
    }
    if (typeof start.auswahl === "number") this.waehlen(start.auswahl);
    if (typeof start.auswahl === "number" && typeof start.buchstabe === "number")
      this.buchstabe = { objekt: start.auswahl, nr: start.buchstabe };
    await this.berechnen();
    if (start.dialog) this.dialog = start.dialog;
  }

  zeige(text: string, art: "ok" | "err" | "info" = "info") {
    this.meldung = { text, art };
    if (art !== "err") setTimeout(() => this.meldung?.text === text && (this.meldung = null), 4000);
  }

  #schnappschuss(): string {
    return JSON.stringify($state.snapshot(this.design));
  }

  /**
   * Design ändern. `gruppe`: aufeinanderfolgende Änderungen derselben Gruppe (z. B. Tippen im Textfeld) ergeben nur
   * einen Schritt für Rückgängig.
   */
  aendern(f: (d: DesignJson) => void, gruppe?: string) {
    const jetzt = Date.now();
    const zusammen =
      gruppe && this.#letzteGruppe?.name === gruppe && jetzt - this.#letzteGruppe.zeit < ZUSAMMENFASSEN_MS;
    if (!zusammen) {
      this.#zurueck = [...this.#zurueck.slice(-VERLAUF_MAX + 1), this.#schnappschuss()];
    }
    this.#letzteGruppe = gruppe ? { name: gruppe, zeit: jetzt } : null;
    this.#vor = [];
    f(this.design);
    this.geaendert = true;
    this.planen();
  }

  /** ganzes Design ersetzen (Öffnen, Neu, Bild/SVG einlesen, Umordnen) – mit Rückgängig */
  ersetzen(d: DesignJson, pfad: string | null = this.pfad, geaendert = true) {
    this.#zurueck = [...this.#zurueck.slice(-VERLAUF_MAX + 1), this.#schnappschuss()];
    this.#vor = [];
    this.#letzteGruppe = null;
    this.design = d;
    this.pfad = pfad;
    this.geaendert = geaendert;
    this.auswahl = this.auswahl.filter((i) => i < (d.objekte?.length ?? 0));
    this.planen(0);
  }

  zurueck() {
    const z = this.#zurueck.at(-1);
    if (z === undefined) return;
    this.#vor = [...this.#vor, this.#schnappschuss()];
    this.#zurueck = this.#zurueck.slice(0, -1);
    this.#uebernehmen(z);
  }

  vor() {
    const v = this.#vor.at(-1);
    if (v === undefined) return;
    this.#zurueck = [...this.#zurueck, this.#schnappschuss()];
    this.#vor = this.#vor.slice(0, -1);
    this.#uebernehmen(v);
  }

  #uebernehmen(text: string) {
    this.#letzteGruppe = null;
    this.design = JSON.parse(text);
    this.geaendert = true;
    this.auswahl = this.auswahl.filter((i) => i < this.objekte.length);
    this.planen(0);
  }

  /** Objekt anhängen und wählen */
  /**
   * Stichart der Objekte `nummern` wechseln (Kern `stichart_aendern`; Form, Farbe, Gruppe … bleiben). Nicht
   * wechselbare Objekte (Text, Säule …) werden übergangen; Fehler und Hinweise erscheinen als Meldung.
   */
  /**
   * Schrift eines Texts wechseln (Kern `schrift_wechseln`): Original-Alphabet ↔ Computer-Schrift, Text und Satz
   * bleiben. Wird daraus ein Text mit Original-Alphabet, bekommt er Stichart „automatisch“ wie jeder neue Text.
   */
  async schriftWechseln(i: number, wahl: SchriftWahl) {
    const design = $state.snapshot(this.design) as DesignJson;
    try {
      // Stichart und Unterlage übernimmt der Kern (auto bleibt auto, Tatami ↔ Step)
      const x = (await kern("schrift_wechseln", { design, objekt: i, ...wahl })) as ObjektJson;
      this.aendern((d) => (d.objekte![i] = x));
    } catch (e) {
      this.zeige(alsFehler(e).meldung, "err");
    }
  }

  /**
   * Rahmen eines Monogramms wählen (Dialog „Rahmen wählen“). Rahmen aus dem Katalog (eigene Erweiterung): die Schrift
   * der Vorlage wird bei Bedarf von Google Fonts geladen; eine selbst gewählte Schrift bleibt, ebenso von Hand gewählte
   * Garne – sonst Initialen und Rahmen in den Farben der Vorlage. Rahmen des Originals: Diamond2/3 (Raute, Zierwinkel,
   * ohne).
   */
  async monogrammRahmen(i: number, wahl: RahmenWahl) {
    const alt = this.objekte[i] as (MonogrammJson & { art: string }) | undefined;
    if (alt?.art !== "monogramm") return;
    if ("rahmen" in wahl) {
      this.aendern((d) => {
        const x = d.objekte![i] as MonogrammJson;
        for (const k of ["vorlage", "schrift", "alphabet", "rahmen_groesse", "rahmen_farben"] as const) delete x[k];
        x.rahmen = wahl.rahmen;
        x.initialen = x.initialen.toUpperCase();
      });
      return;
    }
    try {
      await listen.monogrammeLaden();
      const r = listen.monogrammRahmen(wahl.vorlage);
      if (!r) return;
      const vorher = listen.monogrammRahmen(alt.vorlage);
      // eigene Schrift: Alphabet oder eine andere als die der bisherigen Vorlage
      const eigen = !!alt.alphabet || (!!alt.schrift && alt.schrift !== vorher?.schrift);
      if (!eigen) {
        await listen.schriftenLaden();
        if (!listen.schriftVorhanden(r.schrift)) {
          this.zeige(`Schrift „${r.schrift}“ wird von Google Fonts geladen …`, "info");
          await listen.googleHerunterladen(r.schrift);
        }
      }
      this.aendern((d) => {
        const x = d.objekte![i] as MonogrammJson & { garn?: string };
        x.vorlage = r.name;
        if (!eigen) {
          x.schrift = r.schrift;
          delete x.alphabet;
        }
        delete x.rahmen;
        delete x.rahmen_farbe;
        if (!x.garn && r.farben[0]) x.farbe = r.farben[0];
      });
    } catch (e) {
      this.zeige(alsFehler(e).meldung, "err");
    }
  }

  /** Schrift der Initialen eines Monogramms mit Rahmen aus dem Katalog (Computer-Schrift oder Original-Alphabet) */
  monogrammSchrift(i: number, wahl: SchriftWahl) {
    this.aendern((d) => {
      const x = d.objekte![i] as MonogrammJson;
      if (wahl.alphabet) {
        x.alphabet = wahl.alphabet;
        delete x.schrift;
      } else if (wahl.schrift) {
        x.schrift = wahl.schrift;
        delete x.alphabet;
      }
    });
  }

  async stichartAendern(nummern: number[], art: string) {
    let design = $state.snapshot(this.design) as DesignJson;
    const neu = new Map<number, ObjektJson>();
    const meldungen: string[] = [];
    for (const i of nummern) {
      const o = design.objekte?.[i];
      if (!o || !WECHSELBAR.includes(o.art) || o.art === art) continue;
      try {
        const r = await kern("stichart_aendern", { design, objekt: i, art });
        const x = r.objekt as ObjektJson;
        if (!o.name || STANDARD_NAMEN.includes(o.name)) x.name = STANDARD_NAME[art] ?? o.name;
        neu.set(i, x);
        design = { ...design, objekte: design.objekte!.map((y, k) => (k === i ? x : y)) };
        meldungen.push(...r.hinweise);
      } catch (x) {
        meldungen.push(alsFehler(x).meldung);
      }
    }
    if (neu.size) this.aendern((d) => neu.forEach((x, i) => (d.objekte![i] = x)));
    if (meldungen.length) this.zeige([...new Set(meldungen)].join(" · "), neu.size ? "info" : "err");
  }

  /**
   * „Umrandung dazu“ (eigene Erweiterung – im Original zwei Objekte von Hand, Handbuch S. 114): hinter der Fläche `i`
   * eine Linie mit derselben Form und der Umriss-Stichart der Vorwahl; beide werden gruppiert.
   */
  async umrandungDazu(i: number, art = this.vorwahl.umriss) {
    const design = $state.snapshot(this.design) as DesignJson;
    try {
      const r = await kern("stichart_aendern", { design, objekt: i, art });
      const linie = r.objekt as ObjektJson;
      linie.name = `${STANDARD_NAME[art] ?? "Umrandung"} (Umrandung)`;
      delete linie.einstich;
      delete linie.ausstich;
      this.aendern((d) => {
        const o = d.objekte![i];
        if (!o.gruppe) {
          d.gruppen ??= {};
          const k = freierSchluessel(d.gruppen);
          d.gruppen[k] = { name: o.name || "Fläche mit Umrandung", herkunft: "gruppiert" };
          o.gruppe = k;
        }
        linie.gruppe = o.gruppe;
        d.objekte!.splice(i + 1, 0, linie);
      });
      this.waehlen(i);
    } catch (x) {
      this.zeige(alsFehler(x).meldung, "err");
    }
  }

  einfuegen(o: ObjektJson) {
    this.aendern((d) => {
      d.objekte ??= [];
      d.objekte.push(o);
    });
    this.auswahl = [this.objekte.length - 1];
  }

  duplizieren() {
    const aus = [...this.auswahl].sort((a, b) => a - b);
    if (!aus.length) return;
    this.aendern((d) => {
      // eine kopierte Gruppe wird eine neue Gruppe
      const neueGruppe = new Map<string, string>();
      for (const i of aus) {
        const kopie = JSON.parse(JSON.stringify(d.objekte![i])) as ObjektJson;
        if (kopie.name) kopie.name = `${kopie.name} (Kopie)`;
        const g = kopie.gruppe;
        if (g && d.gruppen?.[g]) {
          if (!neueGruppe.has(g)) {
            const k = freierSchluessel(d.gruppen);
            d.gruppen[k] = { ...JSON.parse(JSON.stringify(d.gruppen[g])), name: `${d.gruppen[g].name ?? "Gruppe"} (Kopie)` };
            neueGruppe.set(g, k);
          }
          kopie.gruppe = neueGruppe.get(g);
        }
        d.objekte!.push(kopie);
      }
    });
    const n = this.objekte.length;
    this.auswahl = aus.map((_, k) => n - aus.length + k);
  }

  loeschen() {
    const aus = new Set(this.auswahl);
    if (!aus.size) return;
    this.aendern((d) => {
      d.objekte = (d.objekte ?? []).filter((_, i) => !aus.has(i));
      gruppenAufraeumen(d);
    });
    this.auswahl = [];
  }

  /**
   * Umordnen über den Kern (`reihenfolge_aendern`, wie Menü „Arrange“). Liegt danach ein Objekt unter einer Fläche,
   * über der es lag, öffnet sich die Warnung (Rückgängig möglich).
   */
  async umordnen(befehl: ReihenfolgeBefehl) {
    const vorher = this.#schnappschuss();
    const namen = this.objekte.map((o, i) => o.name || `Nr. ${i + 1}`);
    try {
      const r = await kern("reihenfolge_aendern", { design: JSON.parse(vorher), befehl });
      const alteAuswahl = this.auswahl;
      this.ersetzen(r.design, this.pfad, true);
      // Auswahl folgt den Objekten
      this.auswahl = alteAuswahl.map((i) => r.reihenfolge.indexOf(i)).filter((i) => i >= 0);
      if (r.vertauscht.length) this.dialog = { art: "warnung", vertauscht: r.vertauscht, namen, vorher };
      else if (befehl.art === "nach_farbe")
        this.zeige(`Nach Farbe sortiert: ${r.farbwechsel_vorher} → ${r.farbwechsel_nachher} Farbwechsel`, "ok");
      return r;
    } catch (x) {
      this.zeige(alsFehler(x).meldung, "err");
      return null;
    }
  }

  /** Knoten-Werkzeug: Formen und Transformation des Objekts `nr` vom Kern in Knoten umwandeln lassen
   *  (`in_knoten_umwandeln`); Rückgängig möglich. */
  async inKnoten(nr: number) {
    try {
      const o = await kern("in_knoten_umwandeln", { design: JSON.parse(this.#schnappschuss()), objekt: nr });
      this.aendern((d) => {
        d.objekte![nr] = o;
      });
      this.zeige("In Knoten umgewandelt – Stiche neu berechnet", "ok");
    } catch (x) {
      this.zeige(alsFehler(x).meldung, "err");
    }
  }

  /** Warnung „Rückgängig“: alte Reihenfolge wiederherstellen */
  wiederherstellen(vorher: string) {
    this.ersetzen(JSON.parse(vorher), this.pfad, true);
  }

  /** Objekt wählen; gehört es zu einer Gruppe, wird die ganze Gruppe gewählt (wie im Original), außer `einzeln`
   *  (Objektliste im Expertenmodus) */
  waehlen(i: number | null, hinzufuegen = false, einzeln = false) {
    this.buchstabe = null;
    if (i === null || i < 0) return void (this.auswahl = []);
    const g = einzeln ? undefined : this.gruppeVon(i);
    const neu = g ? this.mitglieder(g) : [i];
    if (hinzufuegen)
      this.auswahl = this.auswahl.includes(i)
        ? this.auswahl.filter((x) => !neu.includes(x))
        : [...this.auswahl, ...neu.filter((x) => !this.auswahl.includes(x))];
    else this.auswahl = neu;
  }

  /** Alle Objekte wählen („Select All“, Strg+A – Handbuch S. 41) */
  alleWaehlen() {
    this.buchstabe = null;
    this.auswahl = this.objekte.map((_, i) => i);
  }

  /**
   * Auswahlrechteck („Selecting objects with a bounding box“, Handbuch S. 42): gewählt werden die Objekte, die ganz im
   * Rechteck liegen (Begrenzung aus `design_formen`). Wie im Original gilt das nicht für Gruppen: liegt ein Teil einer
   * Gruppe ganz im Rechteck, wird die ganze Gruppe gewählt (Auslegung von „Unless they have already been grouped“).
   * `hinzufuegen` (Umschalt/Strg): zur bisherigen Auswahl dazu.
   */
  imRechteckWaehlen([x0, y0, x1, y1]: [number, number, number, number], hinzufuegen = false) {
    this.buchstabe = null;
    const drin = new Set<number>();
    for (const f of this.formen) {
      const b = f.begrenzung;
      if (!b || b[0] < x0 || b[1] < y0 || b[2] > x1 || b[3] > y1) continue;
      const g = this.gruppeVon(f.objekt);
      for (const i of g ? this.mitglieder(g) : [f.objekt]) drin.add(i);
    }
    const neu = hinzufuegen ? new Set([...this.auswahl, ...drin]) : drin;
    this.auswahl = [...neu].sort((a, b) => a - b);
  }

  /** Neuberechnung planen (entprellt, beim Tippen) */
  planen(ms = ENTPRELLEN_MS) {
    if (this.#zeitgeber) clearTimeout(this.#zeitgeber);
    this.#zeitgeber = setTimeout(() => this.berechnen(), ms);
  }

  /** Stiche und Formen vom Kern holen; ältere Antworten werden verworfen. */
  async berechnen() {
    const nr = ++this.#lauf;
    const design = $state.snapshot(this.design) as DesignJson;
    this.rechnet = true;
    const t0 = performance.now();
    try {
      const [ergebnis, formen, pruefung] = await Promise.all([
        kern("design_berechnen", { design }),
        kern("design_formen", { design }),
        kern("design_pruefen", { design }),
      ]);
      if (nr !== this.#lauf) return;
      this.ergebnis = ergebnis;
      this.formen = formen;
      this.pruefung = pruefung;
      this.fehler = null;
      this.dauer = Math.round(performance.now() - t0);
      this.stand++;
      if (this.modus === "experte")
        kern("ueberdeckungen", { design })
          .then((u) => nr === this.#lauf && (this.ueberdeckungen = u))
          .catch(() => (this.ueberdeckungen = []));
    } catch (x) {
      if (nr !== this.#lauf) return;
      this.fehler = alsFehler(x);
      this.stand++;
      // Prüfung trotzdem holen (Hinweise), Fehler bleibt sichtbar
      kern("design_pruefen", { design })
        .then((p) => nr === this.#lauf && (this.pruefung = p))
        .catch(() => {});
    } finally {
      if (nr === this.#lauf) this.rechnet = false;
    }
  }
}

export const arbeit = new Arbeit();
