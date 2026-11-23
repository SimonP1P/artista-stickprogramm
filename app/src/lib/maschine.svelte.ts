// Maschine in der Oberfläche: Anschluss, Übertragen mit **einem** Klick (Fenster mit einem Fortschrittsbalken:
// verbinden → schreiben und zurücklesen → „Bereit“ → Knopf **„Start“** = Befehl 0x08, erst dann bewegt sich die
// Maschine → PC-Modus verlassen; Entscheidungen des Nutzers 22.11.2026, `UI_Mockups/UX_ANALYSE_2026-11-22.md`),
// Designs aus der Maschine.
// Alle Befehle über `kern(…)` (`kern/stick-api/src/maschine.rs`). Für Claude-Sitzungen gilt CLAUDE.md §4 Regel 1 weiter.

import {
  kern,
  type DesignJson,
  type Fehler,
  type Fortschritt,
  type ListenInfo,
  type GelesenInfo,
  type MaschinenInfo,
  type SendeInfo,
} from "$kern/kern";
import { arbeit, alsFehler } from "./arbeit.svelte";

export const SIMULATION = "Simulation";

export type Phase = "suchen" | "laeuft" | "fertig" | "fehler";

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

  // Übertragen
  /** Fenster „An die Maschine übertragen“ (beide Modi; im Expertenmodus mit „Details“) */
  dialogOffen = $state(false);
  /** Übertragen läuft gerade (Suchen, Verbinden, Senden) */
  uebertraegt = $state(false);
  phase = $state<Phase>("laeuft");
  /** Fortschritt je Teil (was → letzter Stand); beim Senden nur „Übertragung“ */
  fortschritt = $state<Record<string, Fortschritt>>({});
  bericht = $state.raw<SendeInfo | null>(null);
  fehler = $state.raw<Fehler | null>(null);
  /** Rahmen seit dem letzten Übertragen gewechselt (Stand beim Start) */
  rahmenNeu = $state(false);
  /** „Start“ gedrückt, Maschine übernimmt das Design */
  startet = $state(false);

  // Designs aus der Maschine
  listeOffen = $state(false);
  liste = $state.raw<ListenInfo | null>(null);
  gelesen = $state.raw<GelesenInfo | null>(null);
  listeFehler = $state.raw<Fehler | null>(null);

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
  /** übertragen, wartet auf „Start“ (Kern meldet „Bereit“) */
  get bereit() {
    return this.laeuft && !!this.fortschritt["Bereit"] && !this.startet;
  }

  /** kurzer Hinweis statt „Übertragen“ auf dem Knopf, wenn das Design so nicht gesendet werden kann */
  get sperrKurz(): string | null {
    const e = arbeit.ergebnis;
    if (!e || arbeit.rechnet) return null;
    if (e.ausserhalb > 0) return "Passt nicht";
    if (e.daten_bytes >= e.daten_grenze) return "Zu groß";
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

  // ---------------------------------------------------------------- Übertragen

  /**
   * Anschluss finden (UX-Analyse S3): gemerkter Port, wenn vorhanden; sonst der einzige Port; sonst der erste, an dem
   * eine artista antwortet (nur lesen). Ergebnis: ob ein Anschluss feststeht.
   */
  async #anschlussFinden(): Promise<boolean> {
    await this.portsLaden();
    const gemerkt = holen("artista.port");
    if (gemerkt === SIMULATION) return true;
    if (gemerkt && this.ports.includes(gemerkt)) {
      this.port = gemerkt;
      return true;
    }
    if (this.ports.length === 1) {
      this.portSetzen(this.ports[0]);
      return true;
    }
    if (this.ports.length === 0) {
      // gemerkter Port fehlt: Fehler „keine Verbindung“ mit Hinweisen; Rechner ohne Anschluss: Simulation
      this.port = gemerkt ?? SIMULATION;
      return true;
    }
    this.phase = "suchen";
    for (const p of this.ports) {
      try {
        this.info = await kern("maschine_pruefen", { port: p, simulation: false });
        this.portSetzen(p);
        return true;
      } catch {
        /* nächster Anschluss */
      }
    }
    this.fehler = {
      meldung: `An keinem Anschluss (${this.ports.join(", ")}) antwortet eine artista.`,
      objekt: null,
      code: "keine_verbindung",
    } as Fehler;
    return false;
  }

  /** **Schreibt in die Maschine**, wartet auf „Start“ (dann Stickmodus) – ein Klick „Übertragen“ */
  async uebertragen() {
    if (this.laeuft || this.sperrgrund) return;
    const design = $state.snapshot(arbeit.design) as DesignJson;
    this.laeuft = true;
    this.uebertraegt = true;
    this.dialogOffen = true;
    this.startet = false;
    this.phase = "laeuft";
    this.fortschritt = {};
    this.bericht = null;
    this.fehler = null;
    this.rahmenNeu = this.rahmenGewechselt;
    try {
      if (!(await this.#anschlussFinden())) {
        this.phase = "fehler";
        return;
      }
      this.phase = "laeuft";
      this.bericht = await kern("maschine_senden", {
        design,
        ...this.anschluss,
        wahl: { icon: true, farben: true, sticken: true, auf_start_warten: true },
      });
      this.phase = "fertig";
      merken("artista.letzter_rahmen", design.rahmen ?? "");
    } catch (x) {
      this.fehler = alsFehler(x);
      this.phase = "fehler";
    } finally {
      this.laeuft = false;
      this.uebertraegt = false;
      this.startet = false;
    }
  }

  /** „Start“: die Maschine übernimmt das Design (Befehl 0x08), der Rahmen fährt in die Startlage */
  async starten() {
    if (!this.bereit) return;
    this.startet = true;
    try {
      await kern("maschine_starten", {});
    } catch {
      this.startet = false;
    }
  }

  schliessen() {
    if (this.laeuft) return;
    this.dialogOffen = false;
  }

  async abbrechen() {
    try {
      await kern("maschine_abbrechen", {});
    } catch {
      /* nichts lief */
    }
  }

  /** nach einem Fehler gleich noch einmal */
  erneut() {
    this.uebertragen();
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
  const mac = typeof navigator !== "undefined" && /Mac/.test(navigator.platform);
  switch (f?.code) {
    case "keine_verbindung":
      return [
        "Maschine einschalten – Stickmodul angesteckt?",
        `Kabel prüfen${maschine.ports.length > 1 ? ` oder unten einen anderen Anschluss wählen (jetzt ${port})` : ` (Anschluss ${port})`}.`,
        ...(mac ? ["Am Mac hängt die Maschine an einem USB-Seriell-Adapter (/dev/cu.usbserial…)."] : []),
        "„Nochmal“ drücken.",
      ];
    case "stickmodul":
      return ["Stickmodul angesteckt?", "Maschine aus- und wieder einschalten.", "„Nochmal“ drücken."];
    case "zurueckgelesen_weicht_ab":
      return ["Es wurde nichts gestickt.", "Maschine aus- und wieder einschalten.", "„Nochmal“ drücken."];
    case "farbtabelle":
      return ["Maschine aus- und wieder einschalten, dann „Nochmal“."];
    case "zu_gross":
      return ["Design verkleinern oder in Teile aufteilen."];
    case "ausserhalb":
      return ["Design einpassen (Schritt „Gestalten“ → Einpassen)."];
    case "abgebrochen":
      return ["Es wurde nichts gestickt."];
    case "belegt":
      return ["Warten, bis die laufende Aktion fertig ist."];
    default:
      return ["Maschine aus- und wieder einschalten.", `Kabel und Anschluss (${port}) prüfen.`, "„Nochmal“ drücken."];
  }
}
