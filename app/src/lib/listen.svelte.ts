// Auswahllisten aus dem Kern, einmal geladen und zwischengespeichert (Alphabete, Stoffe, Garnkarten, Garne,
// Alphabet-Vorschauen). Beschriftungen auf Deutsch sind reine Anzeige.

import {
  kern,
  type AlphabetInfo,
  type AlphabetVorschau,
  type GarnInfo,
  type StoffInfo,
} from "$kern/kern";

class Listen {
  alphabete = $state.raw<AlphabetInfo[]>([]);
  stoffe = $state.raw<StoffInfo[]>([]);
  garnkarten = $state.raw<string[]>([]);
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

  garne(karte: string): Promise<GarnInfo[]> {
    let p = this.#garne.get(karte);
    if (!p) {
      p = kern("garnkarte_garne", { name: karte });
      this.#garne.set(karte, p);
      p.catch(() => this.#garne.delete(karte));
    }
    return p;
  }

  alphabetVorschau(alphabet: string): Promise<AlphabetVorschau> {
    let p = this.#vorschau.get(alphabet);
    if (!p) {
      p = kern("alphabet_vorschau", { alphabet, text: "Aa Bb" });
      this.#vorschau.set(alphabet, p);
    }
    return p;
  }

  alphabet(name: string | undefined): AlphabetInfo | undefined {
    return this.alphabete.find((a) => a.name === name);
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
