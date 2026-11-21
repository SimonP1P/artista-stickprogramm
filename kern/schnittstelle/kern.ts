// ERZEUGT aus dem Rust-Kern (stick-kern/src/format.rs, stick-api) – nicht von Hand ändern; neu erzeugen: stick api exportieren
// Design-Format Fassung 1. Objektnummern ab 0. Koordinaten in mm, Ursprung Rahmenmitte, +y oben.

import { invoke } from "@tauri-apps/api/core";

/** Abdeckstich der Applikation */
export type Abdeckung = ("satin" | "blanket");

/** Original-Alphabet (ESB-Datei). */
export type AlphabetInfo = {
  datei: string;
  /** größte empfohlene Höhe in mm (Handbuch) */
  max_hoehe?: number | null;
  /** kleinste empfohlene Höhe in mm (Handbuch „Alphabet Samples“; fehlt, wenn das Handbuch keine nennt) */
  min_hoehe?: number | null;
  /** Name wie im Design-Feld `alphabet` */
  name: string;
};

/** Umrisse eines Probetexts und der Zeichenvorrat des Alphabets. */
export type AlphabetVorschau = {
  /** umschließendes Rechteck [x0, y0, x1, y1] in mm */
  begrenzung?: number[] | null;
  /** Linien des Probetexts (Umrisse, Satinseiten …) in mm, wie `design_formen` */
  linien: FormLinie[];
  /** alle Zeichen, die das Alphabet hat */
  zeichen: string;
};

/** Alphabet und Probetext für die Vorschau. */
export type AlphabetVorschauEingabe = {
  /** Name wie im Design-Feld `alphabet` */
  alphabet: string;
  /** Höhe in mm (Standard 10) */
  hoehe?: number | null;
  /** Probetext (Standard „Abc“) */
  text?: string | null;
};

/** Anschluss: COM-Port oder Simulation (ohne Maschine, es wird nichts gesendet). */
export type Anschluss = {
  /**
   * Name des Ports (z. B. „COM3“); bei `simulation` ohne Bedeutung
   * @default ""
   */
  port?: string;
  /**
   * Simulation statt echter Maschine
   * @default false
   */
  simulation?: boolean;
};

/** Applikation: Platzierung → Stopp → Heften → Stopp → Abdecken (`applikation.rs`). */
export type ApplikationJson = {
  abdeckung?: (Abdeckung | null);
  /** fester Satinabstand des Abdeckstichs in mm */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /** Breite des Abdeck-Satins in mm (Standard 2,5) */
  breite?: number | null;
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** Abstand des Heftstichs in mm */
  heft_abstand?: number | null;
  /** Breite des Heftstichs in mm */
  heft_breite?: number | null;
  loecher?: PunktAngabe[];
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Stichlänge der Platzierungslinie in mm */
  platz_laenge?: number | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stil?: (ApplikationStil | null);
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  umriss: PunktAngabe;
};

/** Arbeitsweise der Applikation */
export type ApplikationStil = ("vorgeschnitten" | "zuschneiden");

export type Art = ("stich" | "sprung" | "farbwechsel");

/** Ausrichtung eines Texts */
export type Ausrichtung = (("links" | "rechts") | "mitte" | "block");

/** Auto Jump eines Objekts: `true`/`false` oder Werte in mm. */
export type AutoJumpJson = (boolean | AutoJumpWerte);

export type AutoJumpWerte = {
  /** Höchststich in mm, 0,3 … 12,7 (Standard 12,7); länger = Sprung */
  max?: number | null;
  /** Länge der Sprünge in mm, 0,3 … 12,7 (Standard 7,0) */
  sprung?: number | null;
};

/** Bild in Stickflächen umwandeln (eigene Festlegung, Auto Digitizer des Originals nicht nachgebaut). */
export type BildEingabe = {
  /** Breite des Ergebnisses in mm (Standard 80) */
  breite?: number | null;
  /** Zahl der Farben (Standard 6) */
  farben?: number | null;
  /** Glättung in mm (Standard 0,2) */
  glaettung?: number | null;
  /** kleinste Fläche in mm² (Standard 4) */
  min_flaeche?: number | null;
  /**
   * Hintergrund mitsticken
   * @default false
   */
  mit_hintergrund?: boolean;
  /** Bilddatei (PNG, JPG, BMP …) */
  pfad: string;
  /** Rahmen des neuen Designs */
  rahmen?: string | null;
};

/** Ergebnis der Bildumwandlung. */
export type BildErgebnis = {
  /** Bildgröße in Punkten */
  breite_px: number;
  /** neues Design (Tatami-Flächen nach Farben) */
  design: DesignWert;
  farben: number;
  flaechen: number;
  hoehe_px: number;
};

/** Blanket (Langettenstich, 0x2727) entlang einer Linie. */
export type BlanketJson = {
  /** Abstand der Zähne in mm (Vorlage 2,0) */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /** Breite in mm (Vorlage 3,0) */
  breite?: number | null;
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** @default false */
  geschlossen?: boolean;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  punkte: PunktAngabe;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  versatz?: (Versatz | null);
};

/** Einzelner Buchstabe eines Texts (`"buchstaben"`). */
export type BuchstabeJson = {
  /** eigene Farbe */
  farbe?: string | null;
  /** Nummer des Buchstabens (1 = erster, ohne Leerzeichen und Zeilenumbrüche) */
  nr: number;
  /** verschieben, drehen, skalieren … um die Mitte des Buchstabens */
  transform?: (Transformation | null);
  /**
   * neue Stichwinkel: Strecken [[x1, y1], [x2, y2]] quer über die Säulen
   * @default []
   */
  winkellinien?: Punkt[][];
};

/** Ein Design und optional Rahmen/Stoff aus der Oberfläche (haben Vorrang vor den Angaben im Design). */
export type DesignEingabe = {
  design: DesignWert;
  /** Rahmen (Befehl `rahmen_liste`) statt `design.rahmen` */
  rahmen?: string | null;
  /** Stoff (Befehl `stoffe_liste`) statt `design.stoff` */
  stoff?: string | null;
};

/** **Ein Design**: Rahmen, Stoff, Garnkarte, Objekte. Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben. */
export type DesignJson = {
  /** Einzelwerte des Stoffs/Stils überschreiben, z. B. {"satin": {"zugausgleich": 0.3}} (Parameterkatalog) */
  einstellungen?: (EinstellungenJson | null);
  /** Endpunkt (Standard: Sprung zurück zur Rahmenmitte) */
  ende?: (Ende | null);
  /**
   * Garnkarte (Befehl `garnkarten_liste`): jede Farbe → nächstes Garn; „keine“ = ohne (Standard „Default“ wie die
   * Maschine)
   * @default "Default"
   */
  garnkarte?: string;
  /**
   * Closest Join: jedes Objekt ohne eigenen Einstich beginnt nahe dem vorigen Ende (nicht bei Schrift)
   * @default false
   */
  kuerzeste_verbindung?: boolean;
  /**
   * Name des Designs (erscheint in der Designliste der Maschine)
   * @default "Design"
   */
  name?: string;
  /** Stickobjekte in Stickreihenfolge */
  objekte?: ObjektJson[];
  /**
   * Stickrahmen (Befehl `rahmen_liste`)
   * @default "artista 130 x 100"
   */
  rahmen?: "artista 255 x 145 Oval" | "artista 255 x 145 Oval Rotated" | "artista 200 x 155" | "artista 200 x 155 Rotated" | "artista 130 x 100" | "artista 130 x 100 Rotated" | "artista 50 x 72 Free Arm" | "artista 50 x 72 Free Arm Rotated" | "artista 50 x 72 Normal" | "artista 50 x 72 Normal Rotated" | "artista 40 x 60" | "artista 40 x 60 Rotated" | "artista 400 x 150 MEGA" | "artista 400 x 150 MEGA Rotated";
  /** Startpunkt (Standard: Rahmenmitte) */
  start?: (Start | null);
  /**
   * Stoff: ändert Dichte, Unterlage, Zugausgleich (8 Stoffe + Software Defaults)
   * @default "0 Software Defaults"
   */
  stoff?: "0 Software Defaults" | "Knit Light Weight" | "Knit Medium Weight" | "Knit Heavy Weight" | "Terry Medium Weight" | "Terry Heavy Weight" | "Woven Light Weight" | "Woven Medium Weight" | "Woven Heavy Weight";
  /** Transformation des ganzen Designs (um seine Mitte) */
  transform?: (Transformation | null);
  /** Fassung des Formats (ohne Angabe: aktuelle Fassung) */
  version?: number | null;
};

/** Ein Design im Format `design.schema.json` (als JSON-Objekt, nicht als Text). */
export type DesignWert = DesignJson;

/** Einzelwerte des Stils überschreiben (Parameterkatalog `PARAMETER.md`); jedes Feld darf fehlen; `stoff` nur am Design */
export type EinstellungenJson = {
  lauf?: {
    /**
     * ab diesem Knick wird an der Ecke eingestochen – nur für berechnete Linien (Unterlagen; eigene Festlegung).
     * Formen aus dem Design: Einstich an jedem Eckpunkt wie im Original (`Analyse_Teil3/07` §1).
     * @default 30.0
     */
    ecke_grad?: number;
    /**
     * Kontur-Lauf 2,5 mm (AMT 0x27a8, hoch)
     * @default 2.5
     */
    laenge?: number;
    /**
     * min. 0,8 mm (AMT 0x27a8, hoch)
     * @default 0.8
     */
    min_laenge?: number;
    /**
     * Chord Gap bei variabler Länge (Lauf in Flächen, hoch)
     * @default 0.09
     */
    sehnenabstand?: number;
    /**
     * „feste Länge“ ist Standard (hoch)
     * @default false
     */
    variabel?: boolean;
  };
  satin?: {
    /**
     * AMT 0x2723 (hoch)
     * @default 0.4
     */
    abstand_fest?: number;
    /** @default true */
    auto_spacing?: boolean;
    /**
     * Stoff-/Stilfaktor AMT 0x2736 (hoch)
     * @default 1.0
     */
    faktor?: number;
    /** Kurzstiche an Innenkurven (Effekt 0x273a, `Analyse_Teil3/10` §2) */
    kurzstiche?: {
      /** @default false */
      an?: boolean;
      /**
       * 0x281c: Kürzungsfaktoren als Dreieckstabelle – Gruppe aus n Stichen: n Faktoren (n = 1 … 5)
       * @default [0.8,0.85,0.72,0.7,0.9,0.7,0.7,0.9,0.8,0.7,0.7,0.87,0.65,0.83,0.7]
       */
      faktoren?: number[];
      /**
       * 0x281b = 5: höchstens so viele Stiche je Gruppe (MOD1 0x10186ad0)
       * @default 5
       */
      max_gruppe?: number;
      /**
       * 0x2819 = 0,5: Stelle gilt als zu dicht, wenn zwei Nachbarstiche auf einer Seite näher als Schwelle × Abstand
       * zusammenkommen (MOD1 0x100a858a: Effekt +0x0C × Abstand)
       * @default 0.5
       */
      schwelle?: number;
      /**
       * 0x281a = 1: Faktor zufällig um −0,04 / 0 / +0,04 verändert (MOD1 0x10186a60, Tabelle 0x10240660)
       * @default true
       */
      zufall?: boolean;
    };
    /**
     * Zickzack als Deckstich (Stichart „Zigzag“ 0x2724, BERESENU STR 2236): Abstand 0x2725 = Weg zwischen zwei
     * Einstichen derselben Seite – Software Defaults 360 = 2,0 mm, alle 8 Stoffe 180 = 1,0 mm (Stil-Liste 0x27a6,
     * `Analyse/daten/entschluesselt/styles.txt`, hoch); 0x2726 = 3,0 mm (Bedeutung offen)
     * @default 2.0
     */
    zickzack_abstand?: number;
    /**
     * Handbuch S. 199 „automatically set at 0.2 mm“ (hoch)
     * @default 0.2
     */
    zugausgleich?: number;
  };
  /** @default "0 Software Defaults" */
  stoff?: string;
  tatami?: {
    /**
     * Abstand AMT 0x2716 (hoch) = Weg zwischen zwei **Hinreihen** (Handbuch PDF-S. 74: „distance between two forward
     * rows“); benachbarte Reihen liegen beim halben Abstand (an den Musterdesigns gemessen 0,49 × bzw. 0,98 × Abstand,
     * `Analyse_Teil3/06_messungen_tatami.md`)
     * @default 0.4
     */
    abstand?: number;
    /**
     * Stichlänge AMT 0x2717 (hoch)
     * @default 4.0
     */
    laenge?: number;
    /** Mindeststich und Sehnenabstand eines variablen Laufs. */
    laufweg?: {
      /** @default 0.4 */
      min_laenge?: number;
      /** @default 0.08888888888888889 */
      sehnenabstand?: number;
    };
    /**
     * Laufweg zwischen Teilbereichen der Fläche: Lauf 0x271c unter jeder Füllart (1,8 mm, variabel, Mindeststich
     * 0,40 mm, Sehnenabstand 0,089 mm; hoch, `Analyse_Teil3/01` §4). Das Original verbindet Teilbereiche fast immer
     * mit Laufstichen (92 % der Tatami-Blöcke ohne langen Stich), zu 55 % am Rand, 45 % durchs Innere
     * (`Analyse_Teil3/08_laufwege_flaechen.md`); hier am Rand entlang (Auswahl-Regel: eigene Festlegung).
     * @default 1.8
     */
    laufweg_laenge?: number;
    /**
     * Mindeststich am Rand AMT 0x2718 (hoch)
     * @default 0.4
     */
    min_rand?: number;
    /**
     * Reihenversatz AMT 0x2719 (hoch): Raster je Reihe um diesen Anteil der Stichlänge verschoben – gemessen
     * bei 0,25/0,25: Nachbarreihe 0,25, übernächste Reihe 0,50 (`Analyse_Teil3/06`)
     * @default 0.25
     */
    versatz?: number;
    /**
     * Zweiter Reihenversatz AMT 0x271a: die Reihen verschieben sich **abwechselnd** um `versatz` und `versatz2`
     * (gemessen an den Step-Mustern 1–11 der Musterdesigns, `Analyse_Teil3/06` §5). None = wie `versatz`.
     */
    versatz2?: number | null;
    /**
     * Stichwinkel (eigene Festlegung)
     * @default 45.0
     */
    winkel?: number;
    /**
     * Zufallsversatz AMT 0x271b: jeder innere Einstich um bis zu ± zufall/2 × Stichlänge verschoben (Musterdesigns
     * „Step stitch pattern 11–20“: Stiche 0,6 … 1,4 × Länge bei 0,5); Verteilung **eigene Festlegung**. Alle 20
     * Step-Muster des Dialogs haben 0.
     * @default 0.0
     */
    zufall?: number;
    /**
     * wie Satin (Handbuch S. 199)
     * @default 0.2
     */
    zugausgleich?: number;
  };
  unterlage?: {
    /**
     * **Auto Underlay** (Werkzeug „Auto Underlay“, Handbuch PDF-S. 197–198): In der Unterlagenliste 0x27a7 der Stile
     * tragen alle Einträge Bit 0x2000 (= aus) in „Software Defaults“ und den 8 Stoffen, bei Schrift keiner (= an);
     * MOD1 `DesignDocument::SetUnderlayOn` setzt die aktuelle Unterlage auf die vorgewählte. Sicherheit mittel-hoch
     * (Stil-Flags + Handbuch + Code), `Analyse_Teil3/05_effekte_und_unterlagen.md`.
     * @default false
     */
    auto_an?: boolean;
    /**
     * Vorgewählte Art der Auto-Unterlage: Eintrag mit Bit 0x0001 = Zickzack (alle Standard-, Stoff-, Schrift- und
     * EasyStitch-Stile).
     * @default ["zickzack"]
     */
    auto_art?: string[];
    /**
     * AMT 0x2745/0x2724/0x2725 (mittel)
     * @default 7.0
     */
    doppelzickzack_laenge?: number;
    /** Mindeststich und Sehnenabstand eines variablen Laufs. */
    doppelzickzack_lauf?: {
      /** @default 0.9944444444444445 */
      min_laenge?: number;
      /** @default 0.08888888888888889 */
      sehnenabstand?: number;
    };
    /**
     * AMT 0x2747 = „Edge Walk“ (hoch: DIALOG.DLL bindet 0x2747 an Text 2599 „Edge Walk“ in BERESENU.DLL,
     * Läufe liegen in den Musterdesigns am Rand; Analyse_Teil3/04)
     * @default 2.2
     */
    edge_walk_laenge?: number;
    /**
     * Läufe der Unterlagen sind im Original „variabel“ (0x271e = 1) mit eigenem Mindeststich (0x2767) und
     * Sehnenabstand (0x2768) je Unterlagenart (Stil „0 Software Defaults“, in allen Stilen gleich, hoch):
     */
    edge_walk_lauf?: {
      /** @default 0.4 */
      min_laenge?: number;
      /** @default 0.07222222222222222 */
      sehnenabstand?: number;
    };
    /**
     * AMT 0x2746 = „Center Run“ (hoch: Text 2598, Läufe liegen auf der Säulenmitte; Analyse_Teil3/04)
     * @default 2.2
     */
    mittellauf_laenge?: number;
    /** Mindeststich und Sehnenabstand eines variablen Laufs. */
    mittellauf_lauf?: {
      /** @default 0.7277777777777777 */
      min_laenge?: number;
      /** @default 0.06111111111111111 */
      sehnenabstand?: number;
    };
    /**
     * Unterlagenrand AMT 0x2769/0x276a (mittel)
     * @default 0.2
     */
    rand?: number;
    /**
     * „+ Lauf 3,0 mm“ zu Zickzack-Unterlagen (AMT 0x2744/0x271c/0x271d, mittel)
     * @default 3.0
     */
    rueck_lauf_laenge?: number;
    /**
     * Tatami-Unterlage AMT 0x275a (mittel)
     * @default 3.0
     */
    tatami_abstand?: number;
    /** @default 4.0 */
    tatami_laenge?: number;
    /** @default 2.5 */
    tatami_randlauf?: number;
    /** Mindeststich und Sehnenabstand eines variablen Laufs. */
    tatami_randlauf_lauf?: {
      /** @default 0.4 */
      min_laenge?: number;
      /** @default 0.07222222222222222 */
      sehnenabstand?: number;
    };
    /**
     * Winkel 90° zur Deckstichrichtung (AMT 0x2759)
     * @default 90.0
     */
    tatami_winkel_versatz?: number;
    /**
     * AMT 0x2744/0x2724/0x2725 (mittel); Weg zwischen Einstichen derselben Seite (an den Musterdesigns gemessen, mittel-hoch, `Analyse_Teil3/07` §2)
     * @default 5.0
     */
    zickzack_laenge?: number;
    /** Mindeststich und Sehnenabstand eines variablen Laufs. */
    zickzack_lauf?: {
      /** @default 0.4 */
      min_laenge?: number;
      /** @default 0.2 */
      sehnenabstand?: number;
    };
    /**
     * AMT 0x2743 (hoch)
     * @default 0.2
     */
    zugausgleich?: number;
  };
  /**
   * Verbinder (Connectors) – Stil-Container AMT 0x272a, in allen 22 Stilen gleich.
   * Zuordnung Tag ↔ Dialogfeld „Connectors“ aus DIALOG.DLL 0x10035e89–0x10036032 (hoch), Analyse_Teil3/01.
   */
  verbinder?: {
    /**
     * Tie In If: „Previous Connector Longer Than“ (0x272d = 1) …
     * @default true
     */
    anfang_bei_langem_verbinder?: boolean;
    /**
     * Tie In: „Always Tie In“ (0x2753 = 0)
     * @default false
     */
    anfang_immer?: boolean;
    /**
     * Tie In If: „After Trim / Color Change“ (0x2750 = 1)
     * @default true
     */
    anfang_nach_trim_farbwechsel?: boolean;
    /**
     * … Länge (0x2755 = 3,0 mm)
     * @default 3.0
     */
    anfang_verbinder_ab?: number;
    /**
     * Tie Off If: „Next Connector Longer Than“ (0x272c = 1) …
     * @default true
     */
    ende_bei_langem_verbinder?: boolean;
    /**
     * Tie Off: „Always Tie Off“ (0x2752 = 0)
     * @default false
     */
    ende_immer?: boolean;
    /**
     * … Länge (0x2754 = 3,0 mm)
     * @default 3.0
     */
    ende_verbinder_ab?: number;
    /**
     * Tie Off If: „Before Trim / Color Change“ (0x2733 = 1)
     * @default true
     */
    ende_vor_trim_farbwechsel?: boolean;
    /**
     * „Always Tie Off Last Object“ (0x276c = 0): letztes Objekt wird im Original nicht vernäht
     * @default false
     */
    letztes_objekt_vernaehen?: boolean;
    /**
     * Trim, wenn der nächste Verbinder länger ist als (0x272b = 1, 0x272e = 3,0 mm; Stoffe 6,0 mm).
     * Die artista 185 schneidet nicht (DFL trims_for_jump = 0); der Wert zählt nur für „nach Trim vernähen“.
     * @default 3.0
     */
    trim_ab?: number;
    /**
     * Anzahl der Vernähstiche (0x272f/0x275d = 2; Bedeutung aus Wert und Lage, mittel)
     * @default 2
     */
    vernaeh_stiche?: number;
    /**
     * Länge der Vernähstiche (0x2730/0x2731 = 1,0 mm; Bedeutung aus Wert und Lage, mittel)
     * @default 1.0
     */
    vernaeh_stichlaenge?: number;
  };
} | null;

/** Endpunkt des Designs (Dialog „Start and End“) */
export type Ende = ("rahmenmitte" | "designmitte" | "unten_mitte" | "letzter_stich" | "start");

/** Berechnetes Design: Stiche, Farben, Garne, Kennzahlen, Prüfungen. */
export type Ergebnis = {
  /** Zahl der Einstiche außerhalb des Stickfelds (muss 0 sein zum Senden) */
  ausserhalb: number;
  /** Nummern der Stiche (in `stiche`) außerhalb des Stickfelds – zum Markieren */
  ausserhalb_stiche: number[];
  /** umschließendes Rechteck der Stiche [x0, y0, x1, y1] in mm (für Einpassen und Mittig); fehlt ohne Stiche */
  begrenzung?: number[] | null;
  /** je Farbblock: Objekte, Garn von Hand, Lagen einer Applikation */
  bloecke: FarbBlock[];
  /**
   * je Stich die Nummer des Buchstabens bei Text/Monogramm (1 = erster ohne Leerraum und `^`, wie
   * `buchstaben[].nr`), 0 = kein Buchstabe – zum Anklicken einzelner Buchstaben
   */
  buchstabe: number[];
  /** Datengröße für die Maschine in Byte (Stichdaten + Anhang wie beim Senden) */
  daten_bytes: number;
  /** Grenze der Datengröße in Byte (Kapazität der artista 185, `DATEN_GRENZE`) */
  daten_grenze: number;
  /** Farbe je Farbblock „#rrggbb“ (Block i beginnt nach dem i-ten Farbwechsel) */
  farben: string[];
  /** Zeichen eines Texts, die das Alphabet nicht hat */
  fehlende_zeichen: string;
  /** Garn je Farbblock aus der Garnkarte des Designs (leer bei „keine“); Feld `garn` am Objekt hat Vorrang */
  garne: GarnInfo[];
  /** Hinweise mit Objektnummer (Felder, die der Kern nicht liest; fehlende Zeichen; außerhalb des Stickfelds) */
  hinweise: Hinweis[];
  name: string;
  /** je Stich die Nummer des Objekts (ab 0) – zum Auswählen und Hervorheben */
  objekt: number[];
  rahmen: RahmenInfo;
  statistik: Statistik;
  /** alle Stiche in Stickfolge */
  stiche: Stich[];
  stoff: string;
};

/** Design und Zieldatei. */
export type ExpEingabe = {
  design: DesignWert;
  /** Zieldatei (.exp) */
  pfad: string;
  rahmen?: string | null;
  stoff?: string | null;
};

/** Zieldatei. */
export type ExpZiel = {
  pfad: string;
};

/** Ein Farbblock (Abschnitt zwischen zwei Farbwechseln = ein Stopp an der Maschine). */
export type FarbBlock = {
  /**
   * Applikation: Lagen in diesem Block (Platzierung, Schnittlinie, Heften, Abdecken) – für die Anleitung
   * („nach diesem Block: Stoff auflegen / zuschneiden“); leer bei anderen Objekten
   */
  applikation: Lage[];
  /** Nummer des Objekts (ab 0), mit dem der Block beginnt */
  objekt: number;
  /** alle Objekte (ab 0), die in diesem Block Stiche haben */
  objekte: number[];
  /** Garn von Hand gewählt (Feld `garn`) statt nächstes Garn der Karte */
  von_hand: boolean;
};

/** Garnfarbbereich der Maschine (Schritt 12 des Originals). */
export type FarbInfo = {
  farben: MaschinenGarn[];
  groesse: number;
  /** höchste Farbanzahl der Tabelle (artista 185: 60) */
  max_farben: number;
  /** Zustand in Worten */
  text: string;
};

/**
 * Fehler eines Befehls: Meldung (deutsch) und, wenn er zu einem Objekt gehört, dessen Nummer (ab 0) – damit die
 * Oberfläche das Objekt markieren kann.
 */
export type Fehler = {
  /** Art des Fehlers bei Maschinen-Befehlen (für Zustände und nächste Schritte in der Oberfläche) */
  code?: (FehlerCode | null);
  meldung: string;
  /** Nummer des Objekts in `objekte` (ab 0), wenn bekannt */
  objekt?: number | null;
};

/** Art eines Fehlers der Maschine (ANFORDERUNGEN §4.2). */
export type FehlerCode = ("keine_verbindung" | "stickmodul" | "antwortet_nicht" | "zurueckgelesen_weicht_ab" | "farbtabelle" | "zu_gross" | "ausserhalb" | "abgebrochen" | "belegt");

/** Form statt Punktliste, z. B. `{"kreis": {"mitte": [0, 0], "radius": 10}}`. */
export type Form = ({
  kreis: {
    /** Mittelpunkt [x, y] in mm */
    mitte: Punkt;
    /** Radius in mm */
    radius: number;
    /**
     * Startwinkel in Grad
     * @default 0.0
     */
    start?: number;
  };
} | {
  rechteck: {
    breite: number;
    hoehe: number;
    mitte: Punkt;
  };
} | {
  oval: {
    breite: number;
    hoehe: number;
    mitte: Punkt;
    /** @default 0.0 */
    start?: number;
  };
} | {
  bogen: {
    bis: number;
    mitte: Punkt;
    radius: number;
    von: number;
  };
} | {
  bezier: Punkt[];
} | {
  verbinden: PunktAngabe[];
});

/** Bedeutung einer Linie in der Formansicht */
export type FormArt = ("umriss" | "loch" | "linie" | "seite_a" | "seite_b" | "winkellinie");

/** Eine Linie der Formansicht. */
export type FormLinie = {
  art: FormArt;
  /** letzter Punkt mit dem ersten verbunden */
  geschlossen: boolean;
  /** Punkte [x, y] in mm */
  punkte: Punkt[];
};

/** Fortschritt großer Lese-/Schreibvorgänge (Ereignis `maschine_fortschritt` der App). */
export type Fortschritt = {
  erledigt: number;
  gesamt: number;
  /** Teil („DATEN“, „ICON“, „Garnfarbtabelle“ …) */
  was: string;
  /** Wiederholungen bisher in diesem Teil (Übertragungsfehler werden automatisch wiederholt) */
  wiederholungen: number;
};

/** Garn einer Garnkarte. */
export type GarnInfo = {
  marke: string;
  name: string;
  /** Herstellernummer (führende Nullen bleiben) */
  nummer: string;
  /** Farbe „#rrggbb“ */
  rgb: string;
};

/** Name einer Garnkarte. */
export type GarnkartenName = {
  name: string;
};

/** Aus der Maschine gelesenes Design zum Ansehen (Vorschau wie `design_berechnen`). */
export type GelesenInfo = {
  bytes: number;
  ergebnis: Ergebnis;
  /** Garnfarben, die die Maschine nach dem Laden in die Garnfarbtabelle legt */
  garne: MaschinenGarn[];
  nr: number;
  sitzung: Sitzung;
};

/** Griffe einer Kante beim Verzerren: eine Zahl (Mitte) oder Liste [links, rechts] bzw. [links, Mitte, rechts] in mm. */
export type Griffe = (number | number[]);

/** Art der Grundlinie (Dialog „Layout“) */
export type GrundlinieArt = ("kreis" | "gerade" | "senkrecht" | "kurve" | "mehrfach");

/**
 * Grundlinie eines Texts (`schrift::Grundlinie`; Dialog „Layout“ des Originals). Beim Kreis ist `position` des Texts
 * die Kreismitte.
 */
export type GrundlinieJson = {
  art: GrundlinieArt;
  /** Arc Angle in Grad (nur ein Bogen; Text wird eingepasst) */
  bogen?: number | null;
  /** Length der geraden Grundlinie in mm (Text wird eingepasst) */
  laenge?: number | null;
  /** oben (Circle CW) | unten (Circle CCW) */
  lage?: (GrundlinieLage | null);
  /**
   * Tilt des Ovals in Grad
   * @default 0.0
   */
  neigung?: number;
  /** Punkte oder Form der eigenen Kurve (art = kurve) */
  punkte?: (PunktAngabe | null);
  /** Radius X in mm (Vorlage 50) */
  radius?: number | null;
  /** Radius Y in mm (Oval) */
  radius_y?: number | null;
  /** ueber | durch | unter (Standard: über der CW-, unter der CCW-Grundlinie) */
  stand?: (GrundlinieStand | null);
  /** Kreis: Justif. Angle (Grad); gerade: Line Angle */
  winkel?: number | null;
};

/** Lage auf der Kreis-Grundlinie */
export type GrundlinieLage = ("oben" | "unten");

/** Stand der Buchstaben zur Grundlinie (Position Above/Through/Below) */
export type GrundlinieStand = "ueber" | "durch" | "unter";

/** Hinweis (kein Fehler – das Design lässt sich berechnen) mit der Nummer des Objekts, wenn er zu einem gehört. */
export type Hinweis = {
  art: HinweisArt;
  meldung: string;
  /** Nummer des Objekts in `objekte` (ab 0), wenn bekannt */
  objekt?: number | null;
};

/** Art eines Hinweises. */
export type HinweisArt = ("feld" | "fehlende_zeichen" | "ausserhalb");

/** Form der Verzerrung (Elastic Lettering) */
export type HuelleArt = ("gerade" | "gebogen");

/**
 * Elastic Lettering: `{"art": "gerade", "oben": [links, rechts], "unten": [links, rechts]}` oder
 * `{"art": "gebogen", "oben": mitte | [links, mitte, rechts], "unten": …}` (mm, + nach oben).
 */
export type HuelleJson = {
  art: HuelleArt;
  /** Verschiebung der oberen Kante in mm */
  oben?: (Griffe | null);
  /** Verschiebung der unteren Kante in mm */
  unten?: (Griffe | null);
};

/** Knopfloch (Handbuch Kap. 23 „Adding buttonholes“): Vorlage aus `ButtonHoles.ESB`, auf Schlitzlänge gestreckt. */
export type KnopflochJson = {
  /**
   * Raupenabstand in mm, 0,2 … 2,0 („Bead Stitch Spacing“); gilt nur für die Raupen, nicht die Enden. Ohne Angabe
   * wie die Vorlage (0,5 mm; Heirloom 1,5 mm)
   */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /**
   * Schlitzlänge in mm, 3 … 60 („Slit Length“, Standard 20 wie das Original)
   * @default 20.0
   */
  laenge?: number;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Mitte des Schlitzes [x, y] in mm (ohne Angabe: Rahmenmitte) */
  position?: (Punkt | null);
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * Typ (Standard „standard“ wie das Original)
   * @default "standard"
   */
  typ?: KnopflochTyp;
  /**
   * Richtung des Schlitzes in Grad („Angle“; 90 = senkrecht, Standard wie das Original)
   * @default 90.0
   */
  winkel?: number;
};

/** Knopfloch-Typ (Dialog „Buttonhole Type“; Vorlagen aus `ButtonHoles.ESB`) */
export type KnopflochTyp = ("standard" | "standard_schmal" | "rund" | "dehnbar" | "heirloom" | "rundes_ende" | "auge_eckig" | "auge_rund" | "auge_spitz");

/**
 * Lage einer Applikation – für die Anleitung an den Farbstopps (Handbuch Kap. „Appliqué“, `Analyse_Teil3/12` §1):
 * nach der Platzierungslinie Stoff auflegen, nach der Schnittlinie bzw. dem Heften zuschneiden, dann Abdecken.
 */
export type Lage = ("platzierung" | "schnittlinie" | "heften" | "abdecken");

/** Laufstich bzw. Dreifachstich entlang einer Linie. */
export type LaufJson = {
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /** Dreifachstich (bei art „dreifach“ immer) */
  dreifach?: boolean | null;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /**
   * Linie schließen (zurück zum ersten Punkt)
   * @default false
   */
  geschlossen?: boolean;
  /** Stichlänge in mm (ohne Angabe: Stil, 2,5 mm) */
  laenge?: number | null;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  punkte: PunktAngabe;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
};

/** Keine Eingabe. */
export type Leer = Record<string, unknown>;

/** Design Nr. … aus der Maschine lesen. */
export type LeseEingabe = {
  /** Nummer in der Designliste (ab 1) */
  nr: number;
  /** @default "" */
  port?: string;
  /** @default false */
  simulation?: boolean;
};

/** Ergebnis von `maschine_liste`. */
export type ListenInfo = {
  designs: ListenZeile[];
  sitzung: Sitzung;
};

/** Eintrag der Designliste der Maschine. */
export type ListenZeile = {
  flags: number;
  name: string;
  nr: number;
};

/** Garn der Maschine (Eintrag der Garnfarbtabelle). */
export type MaschinenGarn = {
  marke: string;
  name: string;
  nummer: number;
  /** Farbe „#rrggbb“ */
  rgb: string;
};

/** Ergebnis von `maschine_pruefen`. */
export type MaschinenInfo = {
  farbbereich: FarbInfo;
  sitzung: Sitzung;
};

/**
 * Monogramm (Handbuch Kap. 21): 2 oder 3 Initialen mit den Alphabeten Diamond2/Diamond3, Rahmen wahlweise. Wird im
 * Kern zu einem Text – alle Schrift-Einstellungen gelten.
 */
export type MonogrammJson = {
  /** Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) */
  abstand?: number | null;
  /** @default "mitte" */
  ausrichtung?: Ausrichtung;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Breite in % der Höhe
   * @default 100.0
   */
  breite?: number;
  /** einzelne Buchstaben: Farbe, Transformation, Stichwinkel */
  buchstaben?: BuchstabeJson[];
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /** Verzerren (Elastic Lettering) */
  elastisch?: (HuelleJson | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * Farben für die Farbwechsel `^` im Text, in Reihenfolge
   * @default []
   */
  farben?: string[];
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** Größe des Füllmusters in % (Stichart „muster“, Standard 100) */
  groesse?: number | null;
  /** Grundlinie (ohne Angabe: gerade waagrecht) */
  grundlinie?: (GrundlinieJson | null);
  /**
   * Höhe in mm (18 = kleinste empfohlene Höhe von Diamond2/3)
   * @default 18.0
   */
  hoehe?: number;
  /** 2 oder 3 Buchstaben A–Z */
  initialen: string;
  /**
   * Kursiv in Grad (−45 … 45)
   * @default 0.0
   */
  kursiv?: number;
  /** Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“) */
  muster?: string | null;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte) */
  position?: (Punkt | null);
  rahmen?: (MonogrammRahmen | null);
  /** Farbe des Rahmens (nur mit Rahmen) */
  rahmen_farbe?: string | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stichart?: (SchriftStichart | null);
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  unterlage?: UnterlageJson;
  winkel?: number | null;
  /** Wortabstand in % der Höhe (Vorlage 25) */
  wortabstand?: number | null;
  /** Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text */
  zeilenabstand?: number | null;
};

/** Rahmen um ein Monogramm */
export type MonogrammRahmen = ("raute" | "zier" | "keine");

/** Ein Motiv mit Linien für die Vorschau (mm, Grundlinie von [0, 0] nach [laenge, 0], +y nach oben). */
export type MotivInfo = {
  /** Höhe quer zur Grundlinie (mm) */
  hoehe: number;
  /** Länge der Grundlinie = Größe im Motivsatz (mm) */
  laenge: number;
  linien: Punkt[][];
  /** Name wie im Design-Feld `motiv` */
  name: string;
};

/**
 * Motivlinie (Handbuch Kap. 25 „Pattern Run“): ein Motiv aus einem Motivsatz (`motive_liste`) wird entlang der Linie
 * wiederholt. Anzahl = Linienlänge / Abstand gerundet, die Motive werden gleichmäßig verteilt (wie das Original).
 */
export type MotivlinieJson = {
  /** Wiederholabstand in mm („Spacing“); ohne Angabe = Größe (Motive stoßen aneinander) */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /**
   * Linie schließen (Motive ringsum)
   * @default false
   */
  geschlossen?: boolean;
  /** Länge eines Motivs entlang der Linie in mm („Size“); ohne Angabe: Größe im Motivsatz */
  groesse?: number | null;
  /**
   * Motiv im Satz, z. B. „705“ (Standard „705“ – eigene Festlegung)
   * @default "705"
   */
  motiv?: string;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  punkte: PunktAngabe;
  /**
   * Motivsatz, z. B. „Heirloom“ (Befehl `motive_liste`; Standard „Heirloom“ – eigene Festlegung)
   * @default "Heirloom"
   */
  satz?: string;
  /**
   * Motive auf die andere Seite der Linie spiegeln
   * @default false
   */
  spiegeln?: boolean;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
};

/** Motivsatz (Pattern Set, Ordner `userletw/` der CD; Handbuch Kap. 25). */
export type MotivsatzInfo = {
  datei: string;
  /** Name wie im Design-Feld `satz` der Motivlinie */
  name: string;
};

/** Name eines Motivsatzes. */
export type MotivsatzName = {
  /** Name wie in `motive_liste` */
  satz: string;
};

/** Füllmuster mit den Linien einer Kachel (für Vorschau-Kacheln). */
export type MusterInfo = {
  /** Gittervektor A in mm (das Muster wiederholt sich um i·A + j·B) */
  a: number[];
  /** Gittervektor B in mm */
  b: number[];
  /** Name wie im Design-Feld `muster` („001 Diamond (90)“) */
  name: string;
  /** Nummer („001“) – auch als `muster` erlaubt */
  nummer: string;
  /** Stichwinkel aus dem Namen in Grad */
  winkel?: number | null;
  /** Linienzüge einer Kachel in mm */
  zuege: number[][][];
};

/** Fläche mit Füllmuster (Program Split / Fancy Fill; 242 Muster, Befehl `fuellmuster_liste`). */
export type MusterJson = {
  /**
   * Namen der Objekte, deren Fläche ausgespart wird (Knockout)
   * @default []
   */
  aussparen?: string[];
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /**
   * Größe des Musters in %
   * @default 100.0
   */
  groesse?: number;
  loecher?: PunktAngabe[];
  /** Name des Musters („001 Diamond (90)“) oder seine Nummer („001“) */
  muster: string;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * Überlappung der Aussparung in mm
   * @default 0.0
   */
  ueberlappung?: number;
  umriss: PunktAngabe;
  /** Unterlage an/aus (ohne Angabe: Stil) */
  unterlage?: boolean | null;
  /** Stichwinkel (ohne Angabe: Winkel aus dem Musternamen, sonst der Füllung) */
  winkel?: number | null;
};

/** Ein Objekt eines Designs. */
export type ObjektEingabe = {
  design: DesignWert;
  /** Nummer des Objekts (ab 0) */
  objekt: number;
};

/** Formen eines Objekts des Designs (`Design::formen`). */
export type ObjektForm = {
  /** Ausstich [x, y], wenn gesetzt */
  ausstich?: (Punkt | null);
  /** umschließendes Rechteck [x0, y0, x1, y1] in mm (None = keine Form) */
  begrenzung?: number[] | null;
  /** Einstich [x, y], wenn gesetzt */
  einstich?: (Punkt | null);
  farbe: string;
  linien: FormLinie[];
  name: string;
  /** Nummer des Objekts in `objekte` (0 = erstes) */
  objekt: number;
};

/** Ein Stickobjekt. `art` wählt die Objektart; Reihenfolge in `objekte` = Stickreihenfolge. */
export type ObjektJson = ((LaufJson & {
  art: "lauf";
}) | (LaufJson & {
  art: "dreifach";
}) | (SatinJson & {
  art: "satin";
}) | (SatinLinieJson & {
  art: "satinlinie";
}) | (BlanketJson & {
  art: "blanket";
}) | (TatamiJson & {
  art: "tatami";
}) | (SatinFlaecheJson & {
  art: "satinflaeche";
}) | (TextJson & {
  art: "text";
}) | (TtfJson & {
  art: "ttf";
}) | (MonogrammJson & {
  art: "monogramm";
}) | (ApplikationJson & {
  art: "applikation";
}) | (MusterJson & {
  art: "muster";
}) | (KnopflochJson & {
  art: "knopfloch";
}) | (MotivlinieJson & {
  art: "motivlinie";
}));

/** Ein Objekt im Format `design.schema.json` (Eintrag in `objekte`). */
export type ObjektWert = ObjektJson;

/** Ein einstellbarer Wert des Parameterkatalogs (`PARAMETER.md`). */
export type Parameter = {
  bereich?: number[] | null;
  /** design | objekt | beide | werkzeug */
  ebene: string;
  einheit: string;
  /** Objektarten (`art` im Design-JSON), für die der Parameter gilt */
  gilt_fuer: string[];
  gruppe: string;
  /** eindeutiger Schlüssel, z. B. „satin.abstand_fest“ */
  id: string;
  /** Pfad im Design-JSON (z. B. „einstellungen.satin.abstand_fest“, „transform.drehen“, „stoff“) */
  json_design?: string | null;
  /** Feld am Objekt (z. B. „abstand“, „auto_jump.max“) */
  json_objekt?: string | null;
  /** einfach | experte | automatisch */
  modus: string;
  name: string;
  /** Fundstelle im Original oder „eigene Festlegung“ */
  quelle: string;
  /** Standard in „0 Software Defaults“ (null = ohne Angabe / vom Stil) */
  standard: unknown;
  /** ändert sich mit dem Stoff (Werte je Stoff: `wert(&fuer_stoff(name)?, pfad)`) */
  stoff: boolean;
  werte: string[];
  wirkung: string;
};

export type Pfad = {
  pfad: string;
};

/** Ergebnis der Prüfung eines Designs (ohne Stiche zu berechnen). */
export type Pruefung = {
  /** Fehler, der das Berechnen verhindert (None = in Ordnung) */
  fehler?: (Fehler | null);
  /** Hinweise, z. B. Felder, die der Kern nicht liest (Tippfehler?) */
  hinweise: Hinweis[];
};

/** [x, y] in mm, Ursprung Rahmenmitte, +y nach oben */
export type Punkt = number[];

/** Punkte einer Linie oder Fläche: Punktliste `[[x, y], …]` in mm oder eine Form. */
export type PunktAngabe = (Punkt[] | Form);

/** Stickrahmen: Stickfeld und Umriss. */
export type RahmenInfo = {
  /** Stickfeld x in mm */
  breite: number;
  /** Umriss des Stickfelds [[x, y], …] in mm */
  form: Punkt[];
  hinweis: string;
  /** Stickfeld y in mm */
  hoehe: number;
  name: string;
};

/** Umordnen wie Menü „Arrange“ des Originals (Objektnummern ab 0). */
export type ReihenfolgeBefehl = ({
  art: "wie_ausgewaehlt";
  auswahl: number[];
} | {
  art: "an_anfang";
  auswahl: number[];
} | {
  art: "ans_ende";
  auswahl: number[];
} | {
  art: "vor";
  auswahl: number[];
  ziel: number;
} | {
  art: "nach";
  auswahl: number[];
  ziel: number;
} | {
  art: "nach_farbe";
  /** @default [] */
  auswahl?: number[];
  /** @default [] */
  farben?: string[];
  /** @default false */
  ueberdeckung?: boolean;
});

export type ReihenfolgeEingabe = {
  befehl: ReihenfolgeBefehl;
  design: DesignWert;
};

/** Ergebnis des Umordnens. */
export type ReihenfolgeErgebnis = {
  /** Design mit neuer Reihenfolge der `objekte` */
  design: DesignWert;
  farbwechsel_nachher: number;
  farbwechsel_vorher: number;
  /** an Stelle i steht das bisherige Objekt `reihenfolge[i]` */
  reihenfolge: number[];
  /** Warnungen: [oben, unten] (bisherige Nummern) – `oben` lag über `unten` und liegt jetzt darunter */
  vertauscht: number[][];
};

/** Eine Satinsäule der Zerlegung. */
export type SaeuleInfo = {
  /** Stichlinien (Richtung der Stiche) */
  linien: Punkt[][];
  seite_a: Punkt[];
  seite_b: Punkt[];
  umriss: Punkt[];
};

/** Fläche, die in Satinsäulen zerlegt werden soll. */
export type SaeulenEingabe = {
  /** Säulen an Knicken teilen (Innenwinkel in Grad, 0 = nie) */
  knickwinkel?: number | null;
  /** @default [] */
  loecher?: Punkt[][];
  /** Umriss [[x, y], …] in mm */
  umriss: Punkt[];
};

/** Fläche als Satinsäulen mit automatischer Stichrichtung (`saeulen.rs`, ROADMAP 4b.17). */
export type SatinFlaecheJson = {
  /** fester Satinabstand in mm statt Auto Spacing */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /** Auto-Spacing-Faktor */
  faktor?: number | null;
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** Säulen an Knicken teilen: Innenwinkel in Grad, 0 … 180 (0 = nie wie der Auto Digitizer; 87 = TrueType) */
  knickwinkel?: number | null;
  /** Kurzstiche an Innenkurven */
  kurzstiche?: boolean | null;
  loecher?: PunktAngabe[];
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stichart?: (SatinStichart | null);
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  umriss: PunktAngabe;
  unterlage?: UnterlageJson;
};

/** Satinsäule zwischen zwei Seiten (Punkt i von A gehört zu Punkt i von B). */
export type SatinJson = {
  /** fester Satinabstand in mm statt Auto Spacing (wie im Alphabet: 0x15) */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /** Auto-Spacing-Faktor des Objekts (Stil: 0x2736; Schrift 0,9) */
  faktor?: number | null;
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** Kurzstiche an Innenkurven (Effekt 0x273a; Software Defaults: aus) */
  kurzstiche?: boolean | null;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  seite_a: PunktAngabe;
  seite_b: PunktAngabe;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stichart?: (SatinStichart | null);
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  unterlage?: UnterlageJson;
};

/** Satinbahn konstanter Breite entlang einer Linie. */
export type SatinLinieJson = {
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /** Breite in mm (Standard 2) */
  breite?: number | null;
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  punkte: PunktAngabe;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stichart?: (SatinStichart | null);
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  unterlage?: UnterlageJson;
  versatz?: (Versatz | null);
};

/** Stichart einer Satinsäule, Satinlinie oder Satinfläche */
export type SatinStichart = ("satin" | "zickzack");

/** Computer-Schrift (TrueType/OpenType). */
export type SchriftInfo = {
  familie: string;
  /** Name wie im Design-Feld `schrift` („Arial Bold“) */
  name: string;
  pfad: string;
  stil: string;
};

/** Stichart einer Schrift mit Original-Alphabet */
export type SchriftStichart = ("satin" | "step" | "muster");

/** Design an die Maschine: Design, Anschluss, Wahl. */
export type SendeEingabe = {
  design: DesignWert;
  port: string;
  /** @default false */
  simulation?: boolean;
  wahl: SendeWahl;
};

/** Ergebnis von `maschine_senden`. */
export type SendeInfo = {
  /** Befehl 0x08 bestätigt (None = nicht gestickt) */
  gestickt?: boolean | null;
  schritte: string[];
  sitzung: Sitzung;
};

/** Plan des Sendens (nur gelesen) – zum Bestätigen. */
export type SendePlanInfo = {
  /** Größe der Teile in Byte: Daten (Puffer), Vorschaubild, Garnfarbtabelle (0 = wird nicht geschrieben) */
  bytes_daten: number;
  bytes_farben: number;
  bytes_icon: number;
  farbbereich?: (FarbInfo | null);
  /** Garne, die an der Maschine erscheinen (je Farbblock) */
  garne: MaschinenGarn[];
  hinweise: string[];
  /** geplante Schreibzugriffe in Worten (DATEN, ICON, INFO, Garnfarbtabelle, Befehl 0x08) */
  schritte: string[];
  sitzung: Sitzung;
};

/** Was beim Senden geschrieben wird (die INFO-Felder immer, wie das Original). */
export type SendeWahl = {
  /** Garnfarbtabelle mitsenden (Garnnamen am Display) */
  farben: boolean;
  /** Vorschaubild (ICON 72 × 62) mitsenden */
  icon: boolean;
  /**
   * danach sticken (Befehl 0x08, „sticken ohne speichern“) – **nur** nach dem Gefahrendialog („Person an der
   * Maschine“, Rahmen fährt)
   */
  sticken: boolean;
};

/** Angaben zu einer Sitzung mit der Maschine. */
export type Sitzung = {
  baud: number;
  /** Pfad des Mitschnitts */
  mitschnitt: string;
  /** Firmware der Nähmaschine (z. B. „04.04“) */
  naehmaschine: string;
  /** Port oder „Simulation“ */
  port: string;
  simulation: boolean;
  /** Firmware des Stickmoduls */
  stickmodul: string;
  /** PC-Modus mit 0x100 + T r M E verlassen */
  verlassen: boolean;
};

/** Skalierung in %: eine Zahl (beide Richtungen) oder [x, y] */
export type Skalierung = (number | number[]);

/** Spiegeln (Mirror) */
export type Spiegeln = ("links_rechts" | "oben_unten" | "beide");

/** Startpunkt des Designs (Dialog „Start and End“) */
export type Start = ("rahmenmitte" | "designmitte" | "unten_mitte" | "erster_stich");

/** Kennzahlen der Stichfolge. */
export type Statistik = {
  /** Breite der Stiche in mm */
  breite: number;
  farbwechsel: number;
  /** Höhe der Stiche in mm */
  hoehe: number;
  /** längster Stich in mm */
  max_stich: number;
  /** kürzester Stich in mm */
  min_stich: number;
  spruenge: number;
  stiche: number;
};

/** Step-Muster des Dialogs „Fill Stitch“ mit den Einstichen einer Probefläche. */
export type StepMusterInfo = {
  /** Einstiche einer Tatami-Probefläche 12 × 8 mm (Software Defaults, Winkel 0°), in mm – für Vorschau-Kacheln */
  einstiche: Punkt[];
  /** Nummer 1–20 (Design-Feld `step_muster`) */
  nr: number;
  /** Reihenversatz in Teilen der Stichlänge (AMT 0x2719) */
  versatz: number;
  /** zweiter Reihenversatz, abwechselnd (AMT 0x271a) */
  versatz2: number;
};

/** Ein Stich: Position in mm (Ursprung Rahmenmitte, +y oben) und Art. */
export type Stich = {
  art: Art;
  x: number;
  y: number;
};

/** Stoff mit den Werten, die er ändert (alle Werte je Stoff: Parameterkatalog). */
export type StoffInfo = {
  /** Auto-Spacing-Faktor */
  auto_spacing_faktor: number;
  name: string;
  /** fester Satinabstand in mm */
  satin_abstand_fest: number;
  /** Tatami-Abstand in mm */
  tatami_abstand: number;
  /** Zugausgleich in mm */
  zugausgleich: number;
};

/** SVG-Datei einlesen (in Inkscape zeichnen → laden → sticken). */
export type SvgEingabe = {
  /**
   * Füllflächen sparen aus, was später gezeichnet ist (wie die SVG aussieht)
   * @default true
   */
  aussparen?: boolean;
  /** Breite des Ergebnisses in mm (ohne Angabe: Größe aus der SVG, 1 px = 25,4/96 mm) */
  breite?: number | null;
  /**
   * Closest Join wie der Auto Digitizer
   * @default true
   */
  kuerzeste_verbindung?: boolean;
  /** SVG-Datei (Text in Inkscape vorher in Pfade umwandeln) */
  pfad: string;
  /** Rahmen des neuen Designs */
  rahmen?: string | null;
};

/** Ergebnis des SVG-Imports. */
export type SvgErgebnis = {
  /** Größe der Zeichnung in mm */
  breite: number;
  /** neues Design */
  design: DesignWert;
  /** Hinweise (Text, Bilder, Verläufe, zu breite Linien, größer als der Rahmen …) */
  hinweise: string[];
  hoehe: number;
  /** Zahl der Objekte je Stichart (Namen wie die EasyStitch-Stile des Originals) */
  klassen: Record<string, number>;
};

/** Tatami-Fläche (Step) mit Löchern; mit `winkellinien` Fläche mit mehreren Stichwinkeln (Turning). */
export type TatamiJson = {
  /**
   * Namen der Objekte, deren Fläche ausgespart wird (Knockout)
   * @default []
   */
  aussparen?: string[];
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** Kurzstiche an Innenkurven (nur mit Winkellinien wirksam; ohne Angabe: Stil) */
  kurzstiche?: boolean | null;
  loecher?: PunktAngabe[];
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * Überlappung der Aussparung in mm (Standard 0 = exakt wie das Original)
   * @default 0.0
   */
  ueberlappung?: number;
  umriss: PunktAngabe;
  /** Unterlage an/aus (ohne Angabe: Auto Underlay des Stils) */
  unterlage?: boolean | null;
  /** Stichwinkel in Grad (ohne Angabe: Stil, 45°) */
  winkel?: number | null;
  /**
   * mehrere Stichwinkel: Strecken [[x1, y1], [x2, y2]], die den Umriss je zweimal schneiden (ohne Löcher)
   * @default []
   */
  winkellinien?: Punkt[][];
};

export type TextDatei = {
  pfad: string;
  text: string;
};

/** Text mit einem Original-Alphabet (50 Alphabete, Befehl `alphabete_liste`). */
export type TextJson = {
  /** Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) */
  abstand?: number | null;
  /** Name des Alphabets („Drifter“) */
  alphabet: string;
  /** @default "mitte" */
  ausrichtung?: Ausrichtung;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Breite in % der Höhe
   * @default 100.0
   */
  breite?: number;
  /** einzelne Buchstaben: Farbe, Transformation, Stichwinkel */
  buchstaben?: BuchstabeJson[];
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /** Verzerren (Elastic Lettering) */
  elastisch?: (HuelleJson | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * Farben für die Farbwechsel `^` im Text, in Reihenfolge
   * @default []
   */
  farben?: string[];
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** Größe des Füllmusters in % (Stichart „muster“, Standard 100) */
  groesse?: number | null;
  /** Grundlinie (ohne Angabe: gerade waagrecht) */
  grundlinie?: (GrundlinieJson | null);
  /**
   * Buchstabenhöhe in mm (Original 1–200 mm; Vorlage 10)
   * @default 10.0
   */
  hoehe?: number;
  /**
   * Kursiv in Grad (−45 … 45)
   * @default 0.0
   */
  kursiv?: number;
  /** Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“) */
  muster?: string | null;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte) */
  position?: (Punkt | null);
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stichart?: (SchriftStichart | null);
  /** Text; `\n` = neue Zeile, `^` = Farbwechsel (Farben in `farben`) */
  text: string;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /** Unterlage (auto = wie Stil Schrift: Zickzack) */
  unterlage?: UnterlageJson;
  /** Stichwinkel der Musterfüllung (Stichart „muster“) */
  winkel?: number | null;
  /** Wortabstand in % der Höhe (Vorlage 25) */
  wortabstand?: number | null;
  /** Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text */
  zeilenabstand?: number | null;
};

/** Transformation in der JSON-Form (`"transform"` am Objekt oder am Design). */
export type Transformation = {
  /**
   * Drehung in Grad gegen den Uhrzeiger
   * @default 0.0
   */
  drehen?: number;
  /**
   * Schrägstellen in Grad (x verschiebt sich mit y, wie Skew im Original)
   * @default 0.0
   */
  schraeg?: number;
  /** Skalierung in %: eine Zahl (beide Richtungen) oder [x, y] */
  skalieren?: (Skalierung | null);
  spiegeln?: (Spiegeln | null);
  /** Verschieben in mm */
  verschieben?: (Punkt | null);
};

/** Text mit einer Computer-Schrift (TrueType/OpenType, Befehl `schriften_liste`). */
export type TtfJson = {
  /** Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) */
  abstand?: number | null;
  /** @default "mitte" */
  ausrichtung?: Ausrichtung;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /**
   * Breite in % der Höhe
   * @default 100.0
   */
  breite?: number;
  /** einzelne Buchstaben: Farbe, Transformation, Stichwinkel */
  buchstaben?: BuchstabeJson[];
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /** Verzerren (Elastic Lettering) */
  elastisch?: (HuelleJson | null);
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * Farben für die Farbwechsel `^` im Text, in Reihenfolge
   * @default []
   */
  farben?: string[];
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /** Größe des Füllmusters in % (Stichart „muster“, Standard 100) */
  groesse?: number | null;
  /** Grundlinie (ohne Angabe: gerade waagrecht) */
  grundlinie?: (GrundlinieJson | null);
  /**
   * Höhe der Großbuchstaben in mm
   * @default 10.0
   */
  hoehe?: number;
  /**
   * Kursiv in Grad (−45 … 45)
   * @default 0.0
   */
  kursiv?: number;
  /** Stichlänge bei Lauf/Dreifach in mm */
  laenge?: number | null;
  /** Füllmuster bei Stichart „muster“: Name („001 Diamond (90)“) oder Nummer („001“) */
  muster?: string | null;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Bezugspunkt auf der Grundlinie [x, y] in mm (Kreis: Kreismitte) */
  position?: (Punkt | null);
  /** Name der Schrift („Arial“, „Arial Bold“) oder Pfad einer .ttf/.otf-Datei */
  schrift: string;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** @default "tatami" */
  stichart?: TtfStichart;
  /** Text; `\n` = neue Zeile, `^` = Farbwechsel */
  text: string;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /** Unterlage an/aus (Tatami, Satin; ohne Angabe: Stil) */
  unterlage?: boolean | null;
  /** Stichwinkel (Tatami, Muster) */
  winkel?: number | null;
  /** Wortabstand in % der Höhe (Vorlage 25) */
  wortabstand?: number | null;
  /** Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text */
  zeilenabstand?: number | null;
};

/** Stichart einer Computer-Schrift */
export type TtfStichart = ("tatami" | "lauf" | "dreifach" | "satin" | "muster");

/** Art einer Unterlage (in Stickfolge) */
export type UnterlageArt = ("mittellauf" | "edge_walk" | "zickzack" | "doppelzickzack");

/**
 * Unterlage: `"auto"` (Standard, wie „Auto Underlay“ des Stils), `"keine"` oder eine Liste der Arten in Stickfolge.
 * Alte Schreibweise: Arten mit „+“ verbunden (`"edge_walk+zickzack"`).
 */
export type UnterlageJson = (UnterlageWahl | UnterlageArt[]);

/** Unterlage automatisch oder keine */
export type UnterlageWahl = ("auto" | "keine");

/** Lage einer Linie zur gezeichneten Linie */
export type Versatz = ("mitte" | "seite");

/** Ein Design (Wurzel von design.schema.json). */
export type Design = DesignJson;

/** Alle Befehle des Kerns mit Ein- und Ausgabe. */
export interface Befehle {
  /** Vorschau eines Original-Alphabets: Umrisse eines Probetexts und Zeichenvorrat */
  alphabet_vorschau: { eingabe: AlphabetVorschauEingabe; ausgabe: AlphabetVorschau };
  /** Original-Alphabete (50) */
  alphabete_liste: { eingabe: Leer; ausgabe: AlphabetInfo[] };
  /** Beispieldesign mit allen Sticharten der ersten Ausbaustufe */
  beispiel_design: { eingabe: Leer; ausgabe: DesignWert };
  /** Bild in Stickflächen umwandeln (neues Design) */
  bild_umwandeln: { eingabe: BildEingabe; ausgabe: BildErgebnis };
  /** Textdatei lesen */
  datei_lesen: { eingabe: Pfad; ausgabe: string };
  /** Stiche berechnen – bei jeder Änderung am Design aufrufen (Stiche werden nie skaliert, immer neu berechnet) */
  design_berechnen: { eingabe: DesignEingabe; ausgabe: Ergebnis };
  /** Formen je Objekt (Umrisse, Linien, Satinseiten, Ein-/Ausstich) zum Zeichnen und Bearbeiten mit der Maus */
  design_formen: { eingabe: DesignEingabe; ausgabe: ObjektForm[] };
  /** Design prüfen, ohne Stiche zu berechnen: Fehler (mit Objektnummer) und Hinweise */
  design_pruefen: { eingabe: DesignEingabe; ausgabe: Pruefung };
  /** JSON-Schema des Design-Formats (Felder, Standardwerte, Auswahlwerte, Beschreibungen) */
  design_schema: { eingabe: Leer; ausgabe: unknown };
  /** Stiche als EXP-Datei speichern (Ergebnis: Zahl der Bytes) */
  exp_speichern: { eingabe: ExpEingabe; ausgabe: number };
  /** Füllmuster (242) mit Linien einer Kachel für die Vorschau */
  fuellmuster_liste: { eingabe: Leer; ausgabe: MusterInfo[] };
  /** Garne einer Garnkarte (für die Auswahl von Hand) */
  garnkarte_garne: { eingabe: GarnkartenName; ausgabe: GarnInfo[] };
  /** Namen der Garnkarten (32 der CD) */
  garnkarten_liste: { eingabe: Leer; ausgabe: string[] };
  /** Stichdaten des zuletzt gelesenen Maschinendesigns als EXP speichern */
  gelesen_exp_speichern: { eingabe: ExpZiel; ausgabe: number };
  /** Objekt in Knoten umwandeln (Formen → Punkte/Bézier, Transformation angewandt) – für das Knoten-Werkzeug */
  in_knoten_umwandeln: { eingabe: ObjektEingabe; ausgabe: ObjektWert };
  /** Laufende Maschinen-Aktion abbrechen */
  maschine_abbrechen: { eingabe: Leer; ausgabe: boolean };
  /** Design aus der Maschine lesen (Befehl 0x400, nur lesen) zum Ansehen */
  maschine_lesen: { eingabe: LeseEingabe; ausgabe: GelesenInfo };
  /** Designliste der Maschine – nur lesen */
  maschine_liste: { eingabe: Anschluss; ausgabe: ListenInfo };
  /** Senden planen – nur lesen: Prüfungen wie das Original, geplante Schreibzugriffe zum Bestätigen */
  maschine_planen: { eingabe: SendeEingabe; ausgabe: SendePlanInfo };
  /** Maschine: serielle Ports (COM…) */
  maschine_ports: { eingabe: Leer; ausgabe: string[] };
  /** Maschine prüfen – nur lesen: Firmware, Garnfarbtabelle; PC-Modus wird verlassen */
  maschine_pruefen: { eingabe: Anschluss; ausgabe: MaschinenInfo };
  /** **Schreibt in die Maschine** (nur nach bestätigtem Plan): DATEN, ICON, INFO, Garnfarbtabelle; mit `wahl.sticken` Befehl 0x08 (Rahmen fährt) */
  maschine_senden: { eingabe: SendeEingabe; ausgabe: SendeInfo };
  /** Motivsätze (14 der CD) für die Motivlinie */
  motive_liste: { eingabe: Leer; ausgabe: MotivsatzInfo[] };
  /** Motive eines Motivsatzes mit Maßen und Linien für die Vorschau */
  motivsatz_motive: { eingabe: MotivsatzName; ausgabe: MotivInfo[] };
  /** Parameterkatalog: jeder einstellbare Wert mit Standard, Bereich, Quelle, Modus einfach/experte/automatisch */
  parameter_katalog: { eingabe: Leer; ausgabe: Parameter[] };
  /** Stickrahmen der artista 185 mit Stickfeld und Umriss */
  rahmen_liste: { eingabe: Leer; ausgabe: RahmenInfo[] };
  /** Objekte umordnen wie Menü „Arrange“: wie ausgewählt, an Anfang/Ende, vor/nach Nummer, nach Farbe */
  reihenfolge_aendern: { eingabe: ReihenfolgeEingabe; ausgabe: ReihenfolgeErgebnis };
  /** Fläche in Satinsäulen zerlegen (Vorschau der automatischen Stichrichtung) */
  saeulen_vorschau: { eingabe: SaeulenEingabe; ausgabe: SaeuleInfo[] };
  /** installierte Computer-Schriften (TrueType/OpenType) */
  schriften_liste: { eingabe: Leer; ausgabe: SchriftInfo[] };
  /** Step-Muster 1–20 (Tabelle des Originals) mit Einstichen einer Probefläche für Vorschau-Kacheln */
  step_muster_liste: { eingabe: Leer; ausgabe: StepMusterInfo[] };
  /** Stoffe (8 + Software Defaults) mit den wichtigsten Werten */
  stoffe_liste: { eingabe: Leer; ausgabe: StoffInfo[] };
  /** SVG-Datei einlesen (z. B. aus Inkscape): Stichart nach Breite wie EasyStitch, neues Design */
  svg_lesen: { eingabe: SvgEingabe; ausgabe: SvgErgebnis };
  /** Textdatei schreiben */
  text_speichern: { eingabe: TextDatei; ausgabe: null };
  /** Paare [früher, später] gestickter Objekte, deren Flächen sich überdecken */
  ueberdeckungen: { eingabe: DesignEingabe; ausgabe: number[][] };
}

/** Namen aller Befehle. */
export const BEFEHLE = [
  "alphabet_vorschau",
  "alphabete_liste",
  "beispiel_design",
  "bild_umwandeln",
  "datei_lesen",
  "design_berechnen",
  "design_formen",
  "design_pruefen",
  "design_schema",
  "exp_speichern",
  "fuellmuster_liste",
  "garnkarte_garne",
  "garnkarten_liste",
  "gelesen_exp_speichern",
  "in_knoten_umwandeln",
  "maschine_abbrechen",
  "maschine_lesen",
  "maschine_liste",
  "maschine_planen",
  "maschine_ports",
  "maschine_pruefen",
  "maschine_senden",
  "motive_liste",
  "motivsatz_motive",
  "parameter_katalog",
  "rahmen_liste",
  "reihenfolge_aendern",
  "saeulen_vorschau",
  "schriften_liste",
  "step_muster_liste",
  "stoffe_liste",
  "svg_lesen",
  "text_speichern",
  "ueberdeckungen",
] as const;

/** Fehler eines Befehls (Meldung deutsch, Objektnummer ab 0 wenn bekannt). */
export function istFehler(x: unknown): x is Fehler {
  return typeof x === "object" && x !== null && "meldung" in x;
}

/**
 * Befehl des Kerns aufrufen (Tauri-Befehl `kern`, siehe kern/stick-api/src/lib.rs).
 * Wirft bei Fehlern ein Objekt vom Typ `Fehler`.
 */
export function kern<N extends keyof Befehle>(befehl: N, daten: Befehle[N]["eingabe"]): Promise<Befehle[N]["ausgabe"]> {
  return invoke("kern", { befehl, daten });
}
