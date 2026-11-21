// Auswahllisten aus dem Kern, einmal geladen und zwischengespeichert (Alphabete, Stoffe, Garnkarten, Garne,
// Alphabet-Vorschauen). Beschriftungen auf Deutsch sind reine Anzeige.

import {
  kern,
  type AlphabetInfo,
  type AlphabetVorschau,
  type GarnInfo,
  type GoogleKatalog,
  type SchriftVorschau,
  type MusterInfo,
  type SchriftInfo,
  type StoffInfo,
} from "$kern/kern";

class Listen {
  alphabete = $state.raw<AlphabetInfo[]>([]);
  stoffe = $state.raw<StoffInfo[]>([]);
  garnkarten = $state.raw<string[]>([]);
  muster = $state.raw<MusterInfo[]>([]);
  schriften = $state.raw<SchriftInfo[]>([]);
  google = $state.raw<GoogleKatalog | null>(null);
  /** Favoriten der Schriftauswahl: „a:Alphabet“, „s:Computer-Schrift“, „g:Google-Familie“ (nur auf diesem Rechner) */
  favoriten = $state<string[]>(favoritenLesen());
  #google: Promise<void> | null = null;
  #schriftVorschau = new Map<string, Promise<SchriftVorschau>>();
  #muster: Promise<void> | null = null;
  #schriften: Promise<void> | null = null;
  #garne = new Map<string, Promise<GarnInfo[]>>();
  #vorschau = new Map<string, Promise<AlphabetVorschau>>();
  #geladen: Promise<void> | null = null;

  laden(): Promise<void> {
    this.#geladen ??= Promise.all([
      kern("alphabete_liste", {}).then((a) => (this.alphabete = a)),
      kern("stoffe_liste", {}).then((s) => (this.stoffe = s)),
      kern("garnkarten_liste", {}).then((g) => (this.garnkarten = g)),
    ]).then(() => {});
    return this.#geladen;
  }

  /** Füllmuster (242) – bei Bedarf */
  musterLaden(): Promise<void> {
    this.#muster ??= kern("fuellmuster_liste", {}).then((m) => {
      this.muster = m;
    });
    return this.#muster;
  }

  /** Computer-Schriften des Systems und heruntergeladene Google Fonts – bei Bedarf (das erste Mal kann es dauern) */
  schriftenLaden(neu = false): Promise<void> {
    if (neu) this.#schriften = null;
    this.#schriften ??= kern("schriften_liste", { neu_einlesen: neu }).then((m) => {
      this.schriften = m;
    });
    this.#schriften.catch(() => (this.#schriften = null));
    return this.#schriften;
  }

  /** Katalog der Google Fonts (im Programm mitgeliefert, ohne Internet) */
  googleLaden(): Promise<void> {
    this.#google ??= kern("google_schriften_liste", {}).then((g) => {
      this.google = g;
    });
    return this.#google;
  }

  /** Google-Schrift herunterladen (Internet); danach Listen neu, Ergebnis = Name für das Design */
  async googleHerunterladen(familie: string): Promise<string> {
    const s = await kern("google_schrift_laden", { familie });
    const g = this.google;
    if (g) this.google = { ...g, schriften: g.schriften.map((x) => (x.familie === familie ? { ...x, geladen: true } : x)) };
    await this.schriftenLaden(true);
    return s.name;
  }

  /** Vorschau einer Computer-Schrift mit Stickbarkeit (zwischengespeichert je Schrift und Text) */
  schriftVorschau(schrift: string, text = "Abc"): Promise<SchriftVorschau> {
    const k = `${schrift}\u0000${text}`;
    let p = this.#schriftVorschau.get(k);
    if (!p) {
      p = kern("schrift_vorschau", { schrift, text });
      this.#schriftVorschau.set(k, p);
      p.catch(() => this.#schriftVorschau.delete(k));
    }
    return p;
  }

  istFavorit(id: string): boolean {
    return this.favoriten.includes(id);
  }

  favoritUmschalten(id: string) {
    this.favoriten = this.istFavorit(id) ? this.favoriten.filter((x) => x !== id) : [...this.favoriten, id];
    try {
      localStorage.setItem(FAVORITEN, JSON.stringify(this.favoriten));
    } catch {
      /* ohne Speicher */
    }
  }

  garne(karte: string): Promise<GarnInfo[]> {
    let p = this.#garne.get(karte);
    if (!p) {
      p = kern("garnkarte_garne", { name: karte });
      this.#garne.set(karte, p);
      p.catch(() => this.#garne.delete(karte));
    }
    return p;
  }

  alphabetVorschau(alphabet: string, text = "Aa Bb"): Promise<AlphabetVorschau> {
    const k = `${alphabet}\u0000${text}`;
    let p = this.#vorschau.get(k);
    if (!p) {
      p = kern("alphabet_vorschau", { alphabet, text });
      this.#vorschau.set(k, p);
      p.catch(() => this.#vorschau.delete(k));
    }
    return p;
  }

  alphabet(name: string | undefined): AlphabetInfo | undefined {
    return this.alphabete.find((a) => a.name === name);
  }
}

const FAVORITEN = "schrift_favoriten";

function favoritenLesen(): string[] {
  try {
    const x = JSON.parse(localStorage.getItem(FAVORITEN) ?? "[]");
    return Array.isArray(x) ? x.filter((y) => typeof y === "string") : [];
  } catch {
    return [];
  }
}

export const listen = new Listen();

/** Deutsche Beschreibung der 8 Stoffe des Originals (Anzeige; Werte und Namen aus dem Kern, `stoffe_liste`). */
export const STOFF_TEXT: Record<string, { de: string; bsp: string }> = {
  "0 Software Defaults": { de: "Standardwerte der Software", bsp: "wenn unsicher" },
  "Knit Light Weight": { de: "Strick / Jersey, leicht", bsp: "T-Shirt" },
  "Knit Medium Weight": { de: "Strick, mittel", bsp: "Sweatshirt, Polo" },
  "Knit Heavy Weight": { de: "Strick, schwer", bsp: "Fleece, Strickjacke" },
  "Terry Medium Weight": { de: "Frottee, mittel", bsp: "Handtuch" },
  "Terry Heavy Weight": { de: "Frottee, schwer", bsp: "Bademantel" },
  "Woven Light Weight": { de: "Gewebe, leicht", bsp: "Bluse, Hemd" },
  "Woven Medium Weight": { de: "Gewebe, mittel", bsp: "Baumwolle, Leinen" },
  "Woven Heavy Weight": { de: "Gewebe, schwer", bsp: "Jeans, Canvas" },
};

/** Garn als Text „Marke · Nummer · Name“ (Farbe nie das einzige Merkmal, UEBERGABE §5) */
export const garnText = (g: GarnInfo | undefined) => (g ? `${g.marke} · ${g.nummer} · ${g.name}` : "–");
