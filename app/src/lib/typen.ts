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

// ---- Schritt 4: an die Maschine (maschine_pruefen, maschine_planen, maschine_senden)

export interface GarnInfo {
  rgb: string;
  nummer: number;
  marke: string;
  name: string;
}

/** Garnfarbbereich der Maschine (Schritt 12 des Originals) */
export interface FarbInfo {
  max_farben: number;
  groesse: number;
  text: string;
  farben: GarnInfo[];
}

export interface Sitzung {
  port: string;
  naehmaschine: string;
  stickmodul: string;
  baud: number;
  /** PC-Modus mit 0x100 + T r M E verlassen */
  verlassen: boolean;
  mitschnitt: string;
}

export interface MaschinenInfo {
  sitzung: Sitzung;
  farbbereich: FarbInfo;
}

export interface PlanInfo {
  sitzung: Sitzung;
  schritte: string[];
  hinweise: string[];
  farbbereich: FarbInfo | null;
}

export interface SendeInfo {
  sitzung: Sitzung;
  schritte: string[];
  /** Befehl 0x08 bestätigt (null = nicht gestickt) */
  gestickt: boolean | null;
}

export interface Fortschritt {
  was: string;
  erledigt: number;
  gesamt: number;
}
