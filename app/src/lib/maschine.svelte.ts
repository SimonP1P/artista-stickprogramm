// Maschine in der Oberfläche (Etappe E3): Anschluss, Prüfen, Senden in Stufen (Anschluss → Plan nur lesen →
// Bestätigen → Übertragen → Ergebnis), Sticken nur nach dem Gefahrendialog, Designs aus der Maschine.
// Alle Befehle über `kern(…)` (`kern/stick-api/src/maschine.rs`). Sicherheit: ANFORDERUNGEN §4, CLAUDE.md §4 Regel 1/1a –
// geschrieben wird nur in `schreiben()`, und das nur aus dem Plan-Schritt bzw. nach dem Gefahrendialog.

import {
  kern,
  type DesignJson,
  type Fehler,
  type Fortschritt,
  type ListenInfo,
  type GelesenInfo,
  type MaschinenInfo,
  type SendeInfo,
  type SendePlanInfo,
} from "$kern/kern";
import { arbeit, alsFehler } from "./arbeit.svelte";

export const SIMULATION = "Simulation";

export type Phase = "anschluss" | "plan" | "laeuft" | "fertig" | "fehler";

/** Anleitung nach einem Rahmenwechsel (Gerätetest 19.11.2026, ANFORDERUNGEN §4.3) */
export const ANLEITUNG_RAHMEN = [
  "Nadel und Nähfuß hoch",
  "Am Display „Layout 1“ wählen",
  "«check» drücken – der Rahmen fährt und erkennt den Rahmen",
  "«Mustermitte anzeigen» → «ok»",
  "Nähfuß senken – dann Start an der Maschine",
];

function merken(k: string, v: string) {
  try {
    localStorage.setItem(k, v);
  } catch {
    /* ohne Speicher */
  }
}
function holen(k: string): string | null {
  try {
    return localStorage.getItem(k);
  } catch {
    return null;
  }
}

class MaschinenSteuerung {
  ports = $state<string[]>([]);
  /** gewählter Anschluss: Portname oder SIMULATION */
  port = $state<string>(holen("artista.port") ?? SIMULATION);
  /** Ergebnis der letzten Prüfung */
  info = $state.raw<MaschinenInfo | null>(null);
  /** eine Maschinen-Aktion läuft */
  laeuft = $state(false);
  pruefFehler = $state.raw<Fehler | null>(null);

  // Senden
  dialogOffen = $state(false);
  phase = $state<Phase>("anschluss");
  wahl = $state({ icon: true, farben: true });
  plan = $state.raw<SendePlanInfo | null>(null);
  /** Fortschritt je Teil (was → letzter Stand) */
  fortschritt = $state<Record<string, Fortschritt>>({});
  bericht = $state.raw<SendeInfo | null>(null);
  fehler = $state.raw<Fehler | null>(null);
  /** Senden mit Sticken (nach dem Gefahrendialog) */
  mitSticken = $state(false);
  stickenDialog = $state(false);
  rahmenErledigt = $state(false);

  // Designs aus der Maschine
  listeOffen = $state(false);
  liste = $state.raw<ListenInfo | null>(null);
  gelesen = $state.raw<GelesenInfo | null>(null);
  listeFehler = $state.raw<Fehler | null>(null);

  #design: DesignJson | null = null;
  #ereignis = false;

  get simulation() {
    return this.port === SIMULATION;
  }
  get anschluss() {
    return { port: this.simulation ? "" : this.port, simulation: this.simulation };
  }
  /** Rahmen seit dem letzten Senden gewechselt (dann Anleitung zeigen) */
  get rahmenGewechselt() {
    const r = holen("artista.letzter_rahmen");
    return r !== null && r !== (arbeit.design.rahmen ?? "");
  }

  /** Grund, warum nicht gesendet werden kann (null = kann senden) */
  get sperrgrund(): string | null {
    const e = arbeit.ergebnis;
    if (this.laeuft) return "Es läuft gerade eine Aktion mit der Maschine.";
    if (arbeit.fehler) return "Das Design hat einen Fehler – erst beheben.";
    if (!e || arbeit.rechnet) return "Das Design wird noch berechnet.";
    if (!arbeit.objekte.length) return "Das Design ist leer.";
    if (e.ausserhalb > 0) return `${e.ausserhalb} Einstiche liegen außerhalb des Stickfelds – erst einpassen.`;
    if (e.daten_bytes >= e.daten_grenze) return "Das Design ist zu groß für die Maschine.";
    return null;
  }

  async portsLaden() {
    try {
      this.ports = await kern("maschine_ports", {});
    } catch {
      this.ports = [];
    }
    if (this.port !== SIMULATION && !this.ports.includes(this.port) && this.ports.length === 0) this.port = SIMULATION;
    await this.#ereignisseHoeren();
  }

  portSetzen(p: string) {
    this.port = p;
    this.info = null;
    this.pruefFehler = null;
    merken("artista.port", p);
  }

  async #ereignisseHoeren() {
    if (this.#ereignis) return;
    this.#ereignis = true;
    try {
      const { listen } = await import("@tauri-apps/api/event");
      await listen<Fortschritt>("maschine_fortschritt", (e) => {
        this.fortschritt = { ...this.fortschritt, [e.payload.was]: e.payload };
      });
    } catch {
      /* ohne Tauri (Bildschirmfoto) */
    }
  }

  /** nur lesen: verbinden, Garnfarbtabelle lesen, PC-Modus verlassen */
  async pruefen() {
    if (this.laeuft) return;
    this.laeuft = true;
    this.pruefFehler = null;
    try {
      this.info = await kern("maschine_pruefen", this.anschluss);
    } catch (x) {
      this.info = null;
      this.pruefFehler = alsFehler(x);
    } finally {
      this.laeuft = false;
    }
  }

  // ---------------------------------------------------------------- Senden

  sendenOeffnen() {
    this.dialogOffen = true;
    this.phase = "anschluss";
    this.plan = null;
    this.bericht = null;
    this.fehler = null;
    this.mitSticken = false;
    this.rahmenErledigt = !this.rahmenGewechselt;
    this.portsLaden();
  }

  schliessen() {
    if (this.phase === "laeuft") return; // läuft weiter, Fenster bleibt („Im Hintergrund weiter“ nutzen)
    this.dialogOffen = false;
    this.stickenDialog = false;
  }

  /** Stufe 1: Plan – nur lesen */
  async planen() {
    if (this.laeuft || this.sperrgrund) return;
    this.#design = $state.snapshot(arbeit.design) as DesignJson;
    this.laeuft = true;
    this.fehler = null;
    try {
      this.plan = await kern("maschine_planen", {
        design: this.#design,
        ...this.anschluss,
        wahl: { ...this.wahl, sticken: false },
      });
      this.phase = "plan";
    } catch (x) {
      this.fehler = alsFehler(x);
      this.phase = "fehler";
    } finally {
      this.laeuft = false;
    }
  }

  /**
   * Stufe 2: **schreibt in die Maschine** – nur aus dem bestätigten Plan bzw. nach dem Gefahrendialog (mit Sticken).
   * Sticken heißt: das bestätigte Design noch einmal übertragen, zurücklesen und dann Befehl 0x08 (wie am Gerät
   * erprobt, 19.11.2026).
   */
  async schreiben(sticken: boolean) {
    if (this.laeuft || !this.plan || !this.#design) return;
    this.laeuft = true;
    this.mitSticken = sticken;
    this.phase = "laeuft";
    this.fortschritt = {};
    this.fehler = null;
    try {
      this.bericht = await kern("maschine_senden", {
        design: this.#design,
        ...this.anschluss,
        wahl: { ...this.wahl, sticken },
      });
      this.phase = "fertig";
      merken("artista.letzter_rahmen", this.#design.rahmen ?? "");
      this.dialogOffen = true;
    } catch (x) {
      this.fehler = alsFehler(x);
      this.phase = "fehler";
      this.dialogOffen = true;
    } finally {
      this.laeuft = false;
    }
  }

  async abbrechen() {
    try {
      await kern("maschine_abbrechen", {});
    } catch {
      /* nichts lief */
    }
  }

  /** Fehler: neu planen, bevor wieder geschrieben wird */
  erneut() {
    this.phase = "anschluss";
    this.fehler = null;
    this.plan = null;
  }

  // ---------------------------------------------------------------- Designs aus der Maschine

  listeOeffnen() {
    this.listeOffen = true;
    this.gelesen = null;
    this.listeFehler = null;
    this.portsLaden();
  }

  async listeLesen() {
    if (this.laeuft) return;
    this.laeuft = true;
    this.listeFehler = null;
    try {
      this.liste = await kern("maschine_liste", this.anschluss);
    } catch (x) {
      this.listeFehler = alsFehler(x);
    } finally {
      this.laeuft = false;
    }
  }

  async designLesen(nr: number) {
    if (this.laeuft) return;
    this.laeuft = true;
    this.listeFehler = null;
    this.fortschritt = {};
    try {
      this.gelesen = await kern("maschine_lesen", { ...this.anschluss, nr });
    } catch (x) {
      this.listeFehler = alsFehler(x);
    } finally {
      this.laeuft = false;
    }
  }
}

export const maschine = new MaschinenSteuerung();

/** Nächste Schritte je Fehlercode (ANFORDERUNGEN §4.2) */
export function naechsteSchritte(f: Fehler | null): string[] {
  const port = maschine.simulation ? "Simulation" : maschine.port;
  switch (f?.code) {
    case "keine_verbindung":
      return [`Kabel und Anschluss (${port}) prüfen.`, "Maschine einschalten, Stickmodul angesteckt?", "„Erneut versuchen“."];
    case "stickmodul":
      return ["Stickmodul angesteckt?", "Maschine aus- und wieder einschalten.", "„Erneut versuchen“."];
    case "zurueckgelesen_weicht_ab":
      return ["Es wurde nichts gestickt.", "Maschine aus- und wieder einschalten.", "„Erneut versuchen“ – das Programm plant neu."];
    case "farbtabelle":
      return ["„Garnfarben senden“ abwählen und ohne Farben senden."];
    case "zu_gross":
      return ["Design verkleinern oder in Teile aufteilen."];
    case "ausserhalb":
      return ["Design einpassen (Schritt „Gestalten“ → Einpassen)."];
    case "abgebrochen":
      return ["Es wurde nichts gestickt. „Erneut versuchen“, wenn gewünscht."];
    case "belegt":
      return ["Warten, bis die laufende Aktion fertig ist."];
    default:
      return ["Maschine aus- und wieder einschalten.", `Kabel und Anschluss (${port}) prüfen.`, "„Erneut versuchen“ – das Programm plant neu, bevor es schreibt."];
  }
}
