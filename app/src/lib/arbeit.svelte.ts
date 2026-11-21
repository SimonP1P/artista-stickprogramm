// Zustand des Programms: **ein Design als JSON** (Format des Kerns, `kern/schnittstelle/kern.ts`), Rückgängig/Wiederholen,
// Auswahl, Modus – und die Ergebnisse des Kerns. Die Oberfläche rechnet keine Geometrie und keine Stiche: jede
// Änderung schickt das Design (entprellt) an `design_berechnen` und `design_formen` (UEBERGABE §3, CLAUDE.md §4 Regel 7).

import {
  kern,
  istFehler,
  type DesignJson,
  type Ergebnis,
  type Fehler,
  type ObjektForm,
  type ObjektJson,
  type Pruefung,
  type RahmenInfo,
  type ReihenfolgeBefehl,
} from "$kern/kern";

export type Modus = "einfach" | "experte";
/** offener Dialog (gerendert in +page.svelte) */
export type OffenerDialog =
  | { art: "alphabet"; objekt: number }
  | { art: "garn"; objekt: number; titel?: string }
  | { art: "bild" }
  | { art: "svg" }
  | { art: "nach_farbe" }
  | { art: "muster"; objekt: number }
  | { art: "step"; objekt: number }
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
  ansicht?: Ansicht;
  /** nur für Bildschirmfotos (app/werkzeuge/bildschirmfoto.py) */
  schritt?: number;
  dialog?: OffenerDialog;
  /** Maschinen-Zustand für Bildschirmfotos: Stufe des Sendens bzw. Liste (ausgeführt in +page.svelte) */
  maschine?: "anschluss" | "plan" | "fertig" | "sticken" | "liste";
};

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
  /** Datei des Designs (null = noch nicht gespeichert) */
  pfad = $state<string | null>(null);
  geaendert = $state(false);

  // Ergebnisse des Kerns (nicht tief reaktiv – groß)
  ergebnis = $state.raw<Ergebnis | null>(null);
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
  /** erstes gewähltes Objekt oder -1 */
  get gewaehlt(): number {
    return this.auswahl.length ? this.auswahl[0] : -1;
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
    if (typeof start.auswahl === "number") this.auswahl = [start.auswahl];
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
      for (const i of aus) {
        const kopie = JSON.parse(JSON.stringify(d.objekte![i])) as ObjektJson;
        if (kopie.name) kopie.name = `${kopie.name} (Kopie)`;
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

  waehlen(i: number | null, hinzufuegen = false) {
    if (i === null || i < 0) this.auswahl = [];
    else if (hinzufuegen)
      this.auswahl = this.auswahl.includes(i) ? this.auswahl.filter((x) => x !== i) : [...this.auswahl, i];
    else this.auswahl = [i];
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
      if (this.modus === "experte")
        kern("ueberdeckungen", { design })
          .then((u) => nr === this.#lauf && (this.ueberdeckungen = u))
          .catch(() => (this.ueberdeckungen = []));
    } catch (x) {
      if (nr !== this.#lauf) return;
      this.fehler = alsFehler(x);
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
