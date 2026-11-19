// Datentypen der Befehle aus src-tauri/src/lib.rs

export interface RahmenInfo {
  name: string;
  breite: number;
  hoehe: number;
  form: [number, number][];
  hinweis: string;
}

export type StichArt = "stich" | "sprung" | "farbwechsel";

export interface Stich {
  x: number;
  y: number;
  art: StichArt;
}

export interface Statistik {
  stiche: number;
  spruenge: number;
  farbwechsel: number;
  breite: number;
  hoehe: number;
  min_stich: number;
  max_stich: number;
}

export interface Ergebnis {
  name: string;
  rahmen: RahmenInfo;
  stoff: string;
  stiche: Stich[];
  farben: string[];
  statistik: Statistik;
  ausserhalb: number;
  /** Zeichen eines Texts, die das Alphabet nicht hat */
  fehlende_zeichen: string;
}

export interface AlphabetInfo {
  name: string;
  datei: string;
}
