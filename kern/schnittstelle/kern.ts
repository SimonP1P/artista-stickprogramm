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
  /** Stile (Namen aus `schrift_stile`, eigene Einordnung nach dem Aussehen) */
  stile: string[];
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

/**
 * Applikation: Platzierung → Stopp → Heften → Stopp → Abdecken (`applikation.rs`). Ohne Angabe gelten die Vorgaben von
 * „Auto Appliqué“ im Original (`AutoAppliqueValues`).
 */
export type ApplikationJson = {
  abdeckung?: (Abdeckung | null);
  /** Abstand des Abdeckstichs in mm (0,2 … 10; Original Satin 0,5, Blanket 2,0) */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /** Breite des Abdeckstichs in mm (0,5 … 10; Original 5,0) */
  breite?: number | null;
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /**
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /** Abstand des Heftstichs in mm (0,5 … 12,7; Original 5,0) */
  heft_abstand?: number | null;
  /** Breite des Zickzack-Heftstichs in mm (0,5 … 10; Original 4,0) */
  heft_breite?: number | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  loecher?: PunktAngabe[];
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Stichlänge der Platzierungslinie in mm (0,5 … 12,7; Original 1,5) */
  platz_laenge?: number | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stil?: (ApplikationStil | null);
  /**
   * Teil-Abdeckung (Partial Appliqué): kein Abdeckstich, wo eine spätere Applikation im Design den Rand überdeckt –
   * überlappende Stoffteile ohne doppelte Ränder
   * @default false
   */
  teilabdeckung?: boolean;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  umriss: PunktAngabe;
  /**
   * Versatz von Abdeck- und Heftstich gegen die Platzierungslinie in % (Schieberegler „Inside ↔ Outside“ des
   * Originals): −100 = ganz außen, 0 = mittig (Standard), 100 = ganz im Stoff. Bei Blanket ohne Wirkung (immer
   * innen).
   */
  versatz?: number | null;
};

/** Arbeitsweise der Applikation */
export type ApplikationStil = ("vorgeschnitten" | "zuschneiden");

export type Art = ("stich" | "sprung" | "farbwechsel");

/** Ausrichten (Arrange-Leiste des Originals) – nach dem zuletzt gewählten Objekt */
export type AusrichtArt = ("links" | "rechts" | "oben" | "unten" | "mitte_senkrecht" | "mitte_waagrecht" | "mitte");

/** Eingabe von `ausrichten`. */
export type AusrichtenEingabe = {
  art: AusrichtArt;
  design: DesignWert;
  /**
   * gewählte Objekte in Klickfolge; das **zuletzt** gewählte ist der Bezug (wie im Original). Gruppen zählen als
   * Ganzes
   */
  objekte: number[];
};

/** Ergebnis von `ausrichten`. */
export type AusrichtenErgebnis = {
  /** geändertes Design (Objekte bzw. Gruppen verschoben) */
  design: DesignWert;
};

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

/**
 * Bild in Stickflächen umwandeln wie der „Auto Digitizer“ des Originals (Schritte nach Handbuch Kap. 12/13, Verfahren
 * eigene Festlegung – `stick-vektor`). Dieselbe Eingabe nimmt `bild_vorschau`.
 */
export type BildEingabe = {
  /** Breite des Ergebnisses in mm (Standard 80) */
  breite?: number | null;
  /**
   * Zahl der Farben im Bild, auch die weggelassenen (Standard 8; mit Garnkarte können es weniger werden, wenn zwei
   * Farben dasselbe Garn bekommen)
   */
  farben?: number | null;
  /**
   * Stichart der Flächen: „step“ (Standard, alles Step/Tatami – bei Fotos am saubersten) oder „auto“ (wie
   * EasyStitch/„Fills: Auto“: bis 8 mm dick Satinfläche, darüber Step – für Logos und Zeichnungen)
   */
  fuellung?: (BildFuellung | null);
  /** Farben auf die Garne der Karte ziehen (Standard an; aus = Farben des Bildes, die Garne nur als Hinweis) */
  garnfarben?: boolean | null;
  /**
   * Garnkarte (Name wie `garnkarten_liste`): nennt zu jeder Farbe das nächste Garn (CIEDE2000) und zieht die Farben
   * auf diese Garne (`garnfarben`); gleiche Garne werden eine Farbe, die Objekte bekommen `garn`. Ohne Angabe oder
   * „keine“: Farben des Bildes
   */
  garnkarte?: string | null;
  /** Glättung der Umrisse in mm (Standard 0,2) */
  glaettung?: number | null;
  /** kleinste Fläche in mm² (Standard 4) – kleinere gehen in der Nachbarfarbe auf (keine Löcher) */
  min_flaeche?: number | null;
  /**
   * veraltet: alles sticken (wie `weglassen: []`)
   * @default false
   */
  mit_hintergrund?: boolean;
  /** Bilddatei (PNG, JPG, BMP …) */
  pfad: string;
  /** Rahmen des neuen Designs */
  rahmen?: string | null;
  /**
   * Bild vereinfachen 0–5 (Standard 2): glättet Fotorauschen und feine Details vor der Farbreduktion, Kanten bleiben
   * scharf (wie „Artwork Preparation“ im Original: Rauschen entfernen, unnötige Details weglassen)
   */
  vereinfachen?: number | null;
  /**
   * Farben, die **nicht** gestickt werden („Omitted Colors“ im Original), als „#rrggbb“ wie in `bild_vorschau`.
   * Ohne Angabe: Vorschlag des Programms (weißer Hintergrund wie „white background omitted“); leere Liste = alles
   * sticken
   */
  weglassen?: string[] | null;
};

/** Ergebnis der Bildumwandlung. */
export type BildErgebnis = {
  /** Bildgröße in Punkten */
  breite_px: number;
  /** neues Design (Satin- und Step-Flächen nach Farben) */
  design: DesignWert;
  farben: number;
  flaechen: number;
  hoehe_px: number;
  /** Zahl der Objekte je Objektart (`satinflaeche`, `tatami`) */
  klassen: Record<string, number>;
};

/** Eine Farbe des vorbereiteten Bildes. */
export type BildFarbe = {
  /** Anteil am Bild in Prozent */
  anteil: number;
  /** „#rrggbb“ – genau so in der Vorschau (zum Weglassen in `weglassen` angeben) */
  farbe: string;
  /** nächstes Garn der Karte (mit `garnkarte`; mit `garnfarben` ist die Farbe genau dieses Garn) */
  garn?: (GarnInfo | null);
  /** wird weggelassen (Vorschlag oder `weglassen`) */
  weglassen: boolean;
};

/** Stichart der Flächen beim Bild umwandeln. */
export type BildFuellung = "auto" | "step";

/** Ergebnis von `bild_vorschau`. */
export type BildVorschau = {
  /**
   * das vereinfachte Bild mit den reduzierten Farben als PNG („data:image/png;base64,…“) – weggelassene Farben sind
   * enthalten, die Oberfläche blendet sie aus
   */
  bild: string;
  breite_px: number;
  /** Farben, häufigste zuerst */
  farben: BildFarbe[];
  hoehe_px: number;
};

/** Eingabe für `blackwork_lauf`. */
export type BlackworkEingabe = {
  design: DesignWert;
  /** Nummern der Objekte (ab 0); das erste geeignete gibt Stichart, Farbe und Effekte vor */
  objekte: number[];
};

/** Ergebnis von `blackwork_lauf`. */
export type BlackworkErgebnis = {
  /** geändertes Design */
  design: DesignWert;
  /** Hinweise (z. B. ausgelassene Objekte) */
  hinweise: string[];
  /** Nummer des neuen Blackwork-Objekts */
  objekt: number;
};

/**
 * Blackwork Run (Handbuch S. 131): mehrere sich überlappende Linien als **ein** Objekt; die Reihenfolge ergibt sich
 * automatisch (ein Zug je zusammenhängendem Netz, Abschnitte nur doppelt, wo es nötig ist, ein Ein- und ein
 * Ausstich). Jede Linie bleibt einzeln formbar. Werte der Stichart wie bei `lauf`, `rueckstich`, `stielstich`.
 */
export type BlackworkJson = {
  /** Stielstich: Abstand in mm (1,5) */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /** Stielstich: Linienstärke in mm (2,0) */
  breite?: number | null;
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /** Stielstich: dreifach (ja) */
  dreifach?: boolean | null;
  /**
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  /** Stichlänge in mm (Lauf/Dreifach: Stil 2,5; Rückstich 3,0) */
  laenge?: number | null;
  /** die Linien (mindestens eine) */
  linien: PunktAngabe[];
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Rückstich/Stielstich: Fadenstärke in mm (0,2) */
  staerke?: number | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /**
   * Stichart aller Linien (Standard Laufstich)
   * @default "lauf"
   */
  stich?: BlackworkStich;
  /** Rückstich: 3 oder 5 Striche (3) */
  striche?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /** Rückstich: Überlappung in % (50) */
  ueberlappung?: number | null;
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  /** Stielstich: Winkel in Grad (50) */
  winkel?: number | null;
};

/** Stichart eines Blackwork-Zugs */
export type BlackworkStich = ("lauf" | "dreifach" | "rueckstich" | "stielstich");

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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
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

/** Eine Vorlage der Craft-Stitch-Leiste. */
export type CraftVorlage = {
  /** kurze Beschreibung (Handbuch) */
  beschreibung: string;
  /** Felder des neuen Objekts ohne Form (`art` und Werte); die Oberfläche ergänzt `punkte` bzw. `umriss` */
  felder: unknown;
  /** „linie“ (offene oder geschlossene Linie) oder „flaeche“ (geschlossene Form) */
  form: string;
  /** Schlüssel (stabil), z. B. „blackwork_rand“ */
  id: string;
  /** Name für die Oberfläche */
  name: string;
  /** Name im Original (Werkzeugleiste) */
  original: string;
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
  /** **Eigener Rahmen** (Rechteck/Oval nach eigenen Maßen): ist das Feld gesetzt, gilt es statt `rahmen` */
  eigener_rahmen?: (EigenerRahmenJson | null);
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
   * **Gruppen** („Group“ im Original): Schlüssel → Name und Transformation. Objekte nennen ihre Gruppe im Feld
   * `gruppe`. Reihenfolge der Transformationen: je Objekt → je Gruppe (um die Mitte ihrer Objekte) → ganzes Design
   */
  gruppen?: Record<string, GruppeJson>;
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
   * Stickrahmen (Befehl `rahmen_liste`); bei einem eigenen Rahmen sein Name (gilt dann `eigener_rahmen`)
   * @default "artista 130 x 100"
   */
  rahmen?: string;
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
  /**
   * Vorlage, aus der das Design entstand (Handbuch Kap. 9: „the template from which the design was created“) – nur
   * zur Anzeige und als Vorschlag bei „Werte in Vorlage speichern“; die Werte selbst stehen in `einstellungen`
   */
  vorlage?: string | null;
};

/** Ein Design im Format `design.schema.json` (als JSON-Objekt, nicht als Text). */
export type DesignWert = DesignJson;

/** Effekte eines Objekts (`stick_kern::effekte`, Bericht `Analyse_Teil3/23_effekte.md`). */
export type EffekteJson = {
  /** **Ausgefranster Rand** (Textured Edge): Reihen- bzw. Stichenden streuen um den Rand (Handbuch S. 213–214) */
  rand?: (RandJson | null);
  /**
   * **Verlauf** (Gradient Fill): Abstand wechselt quer über die Fläche bzw. entlang der Säule zwischen `min` und
   * `max`; der Abstand des Objekts gilt dann nicht (Handbuch S. 218)
   */
  verlauf?: (VerlaufJson | null);
  /**
   * **Welle** (Wave Effect): Leitlinie [[x, y], …] in mm quer durch die Fläche, die Reihen folgen ihr (nur
   * `tatami` und `muster`; Handbuch S. 217–218: Stichlänge ≤ 4 mm empfohlen)
   */
  welle?: (PunktAngabe | null);
};

/**
 * **Eigener Rahmen** (Handbuch Kap. 6 S. 49–51 „Creating custom hoops“): Stickfeld nach eigenen Maßen. Zum Gestalten
 * frei; beim Senden prüft das Programm, in welchen Rahmen der artista 185 die Stiche passen, und nennt ihn (passt
 * keiner, wird nicht gesendet). Die Liste der eigenen Rahmen verwalten `rahmen_speichern`/`rahmen_loeschen`.
 */
export type EigenerRahmenJson = {
  /** Breite des Stickfelds in mm (10 … 1000) */
  breite: number;
  /** @default "rechteck" */
  form?: RahmenForm;
  /**
   * Oval: Höhe des geraden Mittelteils in mm („Rectangle Height“; 0 = Ellipse)
   * @default 0.0
   */
  gerade?: number;
  /** Höhe des Stickfelds in mm (10 … 1000; Oval: Gesamthöhe „Total Height“) */
  hoehe: number;
  /** Name (Rahmenliste; nicht der Name eines Rahmens der 185) */
  name: string;
};

/** Ein Design in ein anderes einfügen. */
export type EinfuegenEingabe = {
  /** Design, in das eingefügt wird */
  design: DesignWert;
  /** Design, dessen Objekte angehängt werden (z. B. Ergebnis von `bild_umwandeln`/`svg_lesen` oder eine Datei) */
  einfuegen: DesignWert;
  /**
   * Alle eingefügten Objekte zu **einer** Gruppe zusammenfassen (Transformation des eingefügten Designs wird die
   * der Gruppe). Hat das eingefügte Design schon Gruppen, bleiben diese stattdessen erhalten (keine verschachtelten
   * Gruppen)
   */
  gruppe?: (NeueGruppe | null);
};

/** Ergebnis von `design_einfuegen`. */
export type EinfuegenErgebnis = {
  /** zusammengefügtes Design */
  design: DesignWert;
  /** Schlüssel der neuen Gruppe (wenn gruppiert) */
  gruppe?: string | null;
  /** Nummern der eingefügten Objekte im neuen Design */
  objekte: number[];
};

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

/** Eingabe für `farben_mischen`. */
export type FarbenMischenEingabe = {
  design: DesignWert;
  /** Farbe der oberen Lage „#rrggbb“ */
  farbe: string;
  /** größter Abstand der oberen Lage in mm (Standard 2,2 wie der Stil) */
  max_oben?: number | null;
  /** größter Abstand der unteren Lage in mm (Standard 2,0 wie der Stil) */
  max_unten?: number | null;
  /** kleinster Abstand beider Lagen in mm (Standard 0,4) */
  min?: number | null;
  /** Nummer des Objekts (ab 0): Step-, Muster- oder Satinfläche, Satinsäule oder Satinlinie */
  objekt: number;
  /**
   * Profil der unteren Lage (die obere bekommt das Gegenstück; Dialog „Color Blending“)
   * @default "linear"
   */
  profil?: Profil;
};

/** Ergebnis von `farben_mischen`. */
export type FarbenMischenErgebnis = {
  /** geändertes Design */
  design: DesignWert;
  /** Schlüssel der neuen Gruppe */
  gruppe: string;
  /** Nummern der beiden Lagen (unten, oben) */
  objekte: number[];
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
} | {
  pfad: {
    /**
     * letzter Punkt mit dem ersten verbunden (Flächen, geschlossene Linien)
     * @default false
     */
    geschlossen?: boolean;
    /** Referenzpunkte: [x, y] = Eckpunkt, {"kurve": [x, y]} = Kurvenpunkt */
    punkte: PfadPunkt[];
  };
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

/** Katalog der Google Fonts (freie Lizenzen; nach Beliebtheit sortiert). */
export type GoogleKatalog = {
  schriften: GoogleSchriftInfo[];
  /** Stand des Katalogs (Datum) */
  stand: string;
  /** alle Stile in Anzeigereihenfolge (Filter der Oberfläche) */
  stile: string[];
};

/** Eingabe von `google_schrift_laden`. */
export type GoogleLadenEingabe = {
  /** Familie wie im Katalog („Parisienne“) */
  familie: string;
};

/** Eine Schrift aus dem Katalog der Google Fonts. */
export type GoogleSchriftInfo = {
  /** Name der Familie – nach dem Herunterladen auch der Name für das Design-Feld `schrift` */
  familie: string;
  /** schon heruntergeladen */
  geladen: boolean;
  /** Größe der Datei in Byte */
  groesse: number;
  /** Ohne Serifen, Mit Serifen, Zierschrift, Handschrift, Schreibmaschine */
  kategorie: string;
  lizenz: string;
  /** Stile (deutsche Namen aus `GoogleKatalog::stile`) */
  stile: string[];
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

/** Eine Gruppe von Objekten (Feld `gruppen` des Designs). */
export type GruppeJson = {
  /** Herkunft (nur Anzeige) */
  herkunft?: (GruppenHerkunft | null);
  /**
   * Name der Gruppe (Elementliste der Oberfläche), z. B. der Dateiname des Bilds
   * @default ""
   */
  name?: string;
  /** Verschieben, Skalieren, Drehen … der ganzen Gruppe um die Mitte ihrer Objekte – Stiche werden neu berechnet */
  transform?: (Transformation | null);
};

/** Woher eine Gruppe stammt (nur für die Anzeige in der Oberfläche) */
export type GruppenHerkunft = ("gruppiert" | "bild" | "svg");

/** Hinweis (kein Fehler – das Design lässt sich berechnen) mit der Nummer des Objekts, wenn er zu einem gehört. */
export type Hinweis = {
  art: HinweisArt;
  meldung: string;
  /** Nummer des Objekts in `objekte` (ab 0), wenn bekannt */
  objekt?: number | null;
};

/** Art eines Hinweises. */
export type HinweisArt = ("feld" | "fehlende_zeichen" | "ausserhalb" | "schrift_step" | "lange_satinstiche" | "stichdaten_skaliert");

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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  /**
   * Richtung des Schlitzes in Grad („Angle“; 90 = senkrecht, Standard wie das Original)
   * @default 90.0
   */
  winkel?: number;
};

/** Knopfloch-Typ (Dialog „Buttonhole Type“; Vorlagen aus `ButtonHoles.ESB`) */
export type KnopflochTyp = ("standard" | "standard_schmal" | "rund" | "dehnbar" | "heirloom" | "rundes_ende" | "auge_eckig" | "auge_rund" | "auge_spitz");

/** Ergebnis von `spiegeln_kopieren` und `kranz`. */
export type KopienErgebnis = {
  /** geändertes Design mit den Kopien (direkt hinter der Auswahl) */
  design: DesignWert;
  /** Nummern der neuen Objekte (für die Auswahl danach) */
  neu: number[];
  /**
   * eine Kopie überlappt eine Fläche – die Oberfläche fragt wie das Original „Überlappende Objekte
   * zusammenführen?“
   */
  ueberlappt: boolean;
  /** Zahl der zusammengeführten Flächen */
  zusammengefuehrt: number;
};

/** Eingabe von `kranz` (Wreath). */
export type KranzEingabe = {
  design: DesignWert;
  /**
   * Kaleidoskop: jede zweite Kopie gespiegelt (nur gerade Punktzahl)
   * @default false
   */
  kaleidoskop?: boolean;
  /** Mittelpunkt [x, y] in mm (Anzeige) */
  mitte: Punkt;
  /** gewählte Objekte (Gruppen als Ganzes) */
  objekte: number[];
  /**
   * Punkte des Kranzes (Original + Kopien), 2 bis 36; Standard 6 wie im Handbuch-Beispiel
   * @default 6
   */
  punkte?: number;
  /**
   * überlappende Flächen vereinigen (wie `spiegeln_kopieren`)
   * @default false
   */
  zusammenfuehren?: boolean;
};

/** Art des Kreuzes (Liste „FillStyle“ des Kreuzstich-Programms des Originals) */
export type KreuzArt = ("voll" | "dreiviertel" | "halb" | "viertel" | "mini" | "aufrecht" | "doppelt" | "laengs" | "quer" | "knoten");

/** Ecke bzw. Viertel einer Zelle („Select the quadrants to be filled“) */
export type KreuzEcke = "unten_links" | "unten_rechts" | "oben_links" | "oben_rechts";

/** Kontur eines Kreuzstich-Objekts (Liste „OutlineStyle“ des Originals) */
export type KreuzKontur = ("keine" | "linie" | "kreuz");

/** Richtung des Halbkreuzes */
export type KreuzRichtung = ("steigend" | "fallend");

/** Eingabe von `kreuzstich_vorschau` und `kreuzstich_bild`. */
export type KreuzstichBildEingabe = {
  /** Breite des Ergebnisses in mm (Standard 80; die Höhe folgt dem Bild) */
  breite?: number | null;
  /** Fäden 2, 4, 6 (Standard 2) */
  faeden?: number | null;
  /** höchstens so viele Farben (2 … 64; Original „Restricted“ 2 … 255; Standard 8) */
  farben?: number | null;
  /** Farben auf die Garne der Karte ziehen (Standard an, wenn eine Garnkarte angegeben ist) */
  garnfarben?: boolean | null;
  /** Garnkarte („Specific Thread Chart“) */
  garnkarte?: string | null;
  /** Kreuzgröße (Rasterweite) in mm (Standard 2,54 = 10 Kreuze je Zoll) */
  groesse?: number | null;
  /** Art der Kreuze (Standard voll) */
  kreuz?: (KreuzArt | null);
  /** Bilddatei (PNG, JPG, BMP, GIF) */
  pfad: string;
  /** Rahmen des neuen Designs */
  rahmen?: string | null;
  /**
   * Farben, die nicht gestickt werden („#rrggbb“ aus der Vorschau); ohne Angabe: Vorschlag (weißer Hintergrund –
   * „Omit Background“); `[]` = alles sticken
   */
  weglassen?: string[] | null;
};

/** Ergebnis von `kreuzstich_bild`. */
export type KreuzstichBildErgebnis = {
  /** Größe in mm */
  breite: number;
  /** neues Design: je Farbe ein Kreuzstich-Objekt (`zellen`), Mitte des Bildes auf der Rahmenmitte */
  design: DesignWert;
  farben: number;
  hoehe: number;
  /** Zahl der Kreuze */
  kreuze: number;
  reihen: number;
  spalten: number;
};

/**
 * **Kreuzstich** (Handbuch Kap. 34–37, Programmteil „Cross Stitch“ des Originals): Kreuze auf einem festen Raster aus
 * Quadraten. Gefüllt werden die Zellen, deren Mitte in der Fläche liegt, dazu einzelne Zellen aus `zellen`. Das Raster
 * liegt fest am Design (Zelle [i, j] = i·groesse … (i+1)·groesse); Verschieben und Drehen bewegen die Fläche, die
 * Kreuze bleiben auf dem Raster.
 */
export type KreuzstichJson = {
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
   * Viertel bzw. Seite für Viertel-, Dreiviertel-, Mini- und längliche Kreuze
   * @default "unten_links"
   */
  ecke?: KreuzEcke;
  /**
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
  /**
   * **Werte je Objekt** (Objekteigenschaften): Stil-Werte nur für dieses Objekt überschreiben, Aufbau wie
   * `einstellungen` des Designs (lauf, satin, tatami, unterlage, verbinder), z. B. {"tatami": {"abstand": 0.5}}.
   * Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt (z. B. `abstand`, `laenge`, `winkel`)
   */
  einstellungen?: (EinstellungenJson | null);
  /** Einstich („entry point“) [x, y] in mm: das Objekt beginnt dort */
  einstich?: (Punkt | null);
  /**
   * Fäden („Threads“) 2, 4 oder 6 = jedes Kreuz einmal, zweimal, dreimal gestickt
   * @default 2
   */
  faeden?: number;
  /**
   * Garnfarbe „#rrggbb“ (wird der Garnkarte zugeordnet)
   * @default "#000000"
   */
  farbe?: string;
  /**
   * Fläche füllen („Fill“); aus = nur der Umriss
   * @default true
   */
  fuellen?: boolean;
  /**
   * **Garn von Hand:** Nummer des Garns in der Garnkarte des Designs (Befehl `garnkarte_garne`, Feld `nummer`, z. B.
   * „0010“). Ohne Angabe wird `farbe` dem nächsten Garn der Karte zugeordnet. Die Oberfläche setzt `farbe` dabei auf
   * die Farbe des Garns. Feldform: eigene Festlegung (im Original wählt man das Garn je Farbe aus der Karte)
   */
  garn?: string | null;
  /**
   * Kreuzgröße = Rasterweite (Kantenlänge eines Kreuzes) in mm, 1 … 8; Standard 2,54 = 10 Kreuze je Zoll
   * („Fabric Count“ des Originals)
   * @default 2.54
   */
  groesse?: number;
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * Kontur („Outline“): keine, Linie auf dem Raster, Kreuze entlang des Umrisses
   * @default "keine"
   */
  kontur?: KreuzKontur;
  /**
   * Art des Kreuzes (Standard voll)
   * @default "voll"
   */
  kreuz?: KreuzArt;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  loecher?: PunktAngabe[];
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /**
   * Halbkreuz: „/“ oder „\“
   * @default "steigend"
   */
  richtung?: KreuzRichtung;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  /** Fläche (ohne Fläche: nur die Zellen aus `zellen`) */
  umriss?: (PunktAngabe | null);
  /**
   * einzelne Zellen [Spalte, Reihe] zusätzlich zur Fläche (Spalte i = x von i·groesse bis (i+1)·groesse)
   * @default []
   */
  zellen?: number[][];
};

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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
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
 * Kern zu einem Text – alle Schrift-Einstellungen gelten. **Mit `vorlage`** (eigene Erweiterung): Rahmen aus dem
 * Katalog der Monogramm-Rahmen, Initialen in einer Computer-Schrift oder einem Alphabet (`monogramm.rs`).
 */
export type MonogrammJson = {
  /** Buchstabenabstand in mm (ohne Angabe: Anteil des Alphabets × Höhe) */
  abstand?: number | null;
  /** Original-Alphabet der Initialen (nur mit `vorlage`, statt `schrift`) */
  alphabet?: string | null;
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * Höhe in mm (18 = kleinste empfohlene Höhe von Diamond2/3)
   * @default 18.0
   */
  hoehe?: number;
  /** 2 oder 3 Buchstaben A–Z (Diamond); mit `vorlage` 1–3 Zeichen */
  initialen: string;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * Farben des Rahmens statt der Farben der Vorlage: 1., 2., 3. Farbe der Vorlage (nur mit `vorlage`; fehlende
   * bleiben, wie sie sind)
   * @default []
   */
  rahmen_farben?: string[];
  /**
   * Größe des Rahmens in mm: längere Seite (nur mit `vorlage`; ohne Angabe wie gezeichnet, 70–88 mm). Die
   * Initialen wachsen mit
   */
  rahmen_groesse?: number | null;
  /** Computer-Schrift der Initialen (nur mit `vorlage`; ohne Angabe: Schriftvorschlag der Vorlage, Google Fonts) */
  schrift?: string | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  stichart?: (SchriftStichart | null);
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  unterlage?: UnterlageJson;
  /**
   * **Rahmen aus dem Katalog** (eigene Erweiterung): Name eines Monogramm-Rahmens (Befehl `monogramm_rahmen`, z. B.
   * „Doppeloval“). Dann stehen die Initialen in der Computer-Schrift `schrift` (ohne Angabe: Schrift der Vorlage)
   * oder im Alphabet `alphabet`, Höhen nach der Vorlage, mittig im Rahmen und so weit verkleinert, dass rundum 2 mm
   * frei bleiben. `rahmen`, `rahmen_farbe` und `hoehe` gelten dann nicht
   */
  vorlage?: string | null;
  winkel?: number | null;
  /** Wortabstand in % der Höhe (Vorlage 25) */
  wortabstand?: number | null;
  /** Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text */
  zeilenabstand?: number | null;
};

/** Katalog der Monogramm-Rahmen. */
export type MonogrammKatalog = {
  /** Kategorien in der Reihenfolge des Entwurfs */
  kategorien: string[];
  rahmen: MonogrammRahmenInfo[];
};

/** Rahmen um ein Monogramm */
export type MonogrammRahmen = ("raute" | "zier" | "keine");

/** Monogramm-Rahmen aus dem Katalog (eigene Erweiterung, `stick_vektor::monogramm`). */
export type MonogrammRahmenInfo = {
  beschreibung: string;
  /** Anzahl Initialen, für die der Rahmen gedacht ist (2 oder 3) */
  buchstaben: number;
  /** Farben der Vorlage („#rrggbb“, Reihenfolge wie `rahmen_farben`) */
  farben: string[];
  /** Größe wie gezeichnet: längere Seite in mm (Design-Feld `rahmen_groesse` ohne Angabe) */
  groesse: number;
  /** Versalhöhen der Initialen in mm bei gezeichneter Größe: links, rechts bzw. links, Mitte, rechts */
  hoehen: number[];
  kategorie: string;
  /** Name wie im Design-Feld `vorlage` des Monogramms */
  name: string;
  nr: number;
  /** Schriftvorschlag (Google Fonts, Familie) – Design-Feld `schrift`, wenn der Anwender nichts anderes wählt */
  schrift: string;
  /** Zeichnung als SVG-Text (Vorschau) */
  svg: string;
};

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

/** Eingabe für `motiv_loeschen`. */
export type MotivLoeschenEingabe = {
  /** Motiv im Satz */
  motiv: string;
  /** eigener Motivsatz */
  satz: string;
};

/** Eingabe für `motiv_speichern`. */
export type MotivSpeichernEingabe = {
  design: DesignWert;
  /**
   * Bezugspunkte [[x1, y1], [x2, y2]] in mm (Handbuch: „Reference points set the standard orientation … and the
   * spacing“). Ohne Angabe: linke und rechte Kante der Objekte auf halber Höhe (eigene Festlegung)
   */
  grundlinie?: number[][] | null;
  /** Name des Motivs im Satz (vorhandenes gleichen Namens wird ersetzt – vorher nachfragen) */
  name: string;
  /** Nummern der Objekte (ab 0), die zusammen das Motiv bilden (Läufe, Satinsäulen, Flächen) */
  objekte: number[];
  /** Name des eigenen Motivsatzes (neu oder vorhanden; nicht der Name eines Satzes der CD) */
  satz: string;
};

/**
 * Motivfüllung (Handbuch Kap. 25 „Pattern Fill“): ein Motiv aus einem Motivsatz wird in parallelen Reihen über die
 * Fläche wiederholt. Motive, die mehr als zur Hälfte über den Rand ragen, werden am Rand abgeschnitten.
 */
export type MotivfuellungJson = {
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
  /** Länge eines Motivs (Grundlinie) in mm; ohne Angabe: Größe im Motivsatz */
  groesse?: number | null;
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  loecher?: PunktAngabe[];
  /**
   * Motiv im Satz (Standard „K0001a“ wie das Original)
   * @default "K0001a"
   */
  motiv?: string;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Abstand der Reihen in mm („row spacing“); ohne Angabe: Höhe des Motivs */
  reihenabstand?: number | null;
  /**
   * Motivsatz (Befehl `motive_liste`; Standard „NP004a“ wie das Original, `<PRESET_STANDARD_MOTIF_FILL>`)
   * @default "NP004a"
   */
  satz?: string;
  /** Abstand der Motive in einer Reihe in mm („column spacing“); ohne Angabe: Breite des Motivs */
  spaltenabstand?: number | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  umriss: PunktAngabe;
  /**
   * Reihenversatz in mm („row offset“): jede Reihe um so viel weiter verschoben als die vorige (0 = Spalten
   * genau untereinander; halber Spaltenabstand = versetzt wie Ziegel)
   * @default 0.0
   */
  versatz?: number;
  /**
   * Drehung der Reihen in Grad (der Stichwinkel wirkt hier nicht, Handbuch S. 227)
   * @default 0.0
   */
  winkel?: number;
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  /**
   * Motiv im Satz, z. B. „705“ (Standard „K0001a“ wie das Original)
   * @default "K0001a"
   */
  motiv?: string;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  punkte: PunktAngabe;
  /**
   * Motivsatz, z. B. „Heirloom“ (Befehl `motive_liste`; Standard „NP004a“ wie das Original,
   * `<PRESET_STANDARD_MOTIF_RUN>`)
   * @default "NP004a"
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
};

/** Motivsatz (Pattern Set, Ordner `userletw/` der CD; Handbuch Kap. 25). */
export type MotivsatzInfo = {
  datei: string;
  /** eigener Motivsatz (Datenordner; Motive lassen sich löschen) statt Satz der CD */
  eigen: boolean;
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  umriss: PunktAngabe;
  /** Unterlage an/aus (ohne Angabe: Stil) */
  unterlage?: boolean | null;
  /** Stichwinkel (ohne Angabe: Winkel aus dem Musternamen, sonst der Füllung) */
  winkel?: number | null;
};

/** Neue Gruppe beim Einfügen (`design_einfuegen`). */
export type NeueGruppe = {
  /** Herkunft (nur Anzeige) */
  herkunft?: (GruppenHerkunft | null);
  /**
   * Name der Gruppe (Elementliste), z. B. der Dateiname
   * @default ""
   */
  name?: string;
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
}) | (MotivfuellungJson & {
  art: "motivfuellung";
}) | (RueckstichJson & {
  art: "rueckstich";
}) | (StielstichJson & {
  art: "stielstich";
}) | (StipplingJson & {
  art: "stippling";
}) | (KreuzstichJson & {
  art: "kreuzstich";
}) | (BlackworkJson & {
  art: "blackwork";
}) | (PhotosnapJson & {
  art: "photosnap";
}) | (SticheJson & {
  art: "stiche";
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

/** Referenzpunkt eines Pfads: Eckpunkt `[x, y]` oder Kurvenpunkt `{"kurve": [x, y]}`. */
export type PfadPunkt = (Punkt | {
  kurve: Punkt;
});

/** Auflösung von PhotoSnap = Breite der Stichreihen */
export type PhotoAufloesung = ("grob" | "mittel" | "fein");

/** Stoff hinter PhotoSnap */
export type PhotoHintergrund = ("hell" | "dunkel");

/** Eine Farblage von PhotoSnap. */
export type PhotoLageJson = {
  /** Garnfarbe der Lage „#rrggbb“ (bei nur einer Lage gilt `farbe` des Objekts) */
  farbe: string;
  /**
   * Deckung je Bildpunkt als Base64 (ein Byte je Punkt, zeilenweise von oben; 0 = kein Faden, 255 = voll – für
   * hellen Stoff)
   */
  werte: string;
};

/** Lage der Farbschichten von PhotoSnap zueinander (nur mit mehreren Lagen) */
export type PhotoLagen = ("gekreuzt" | "versetzt");

/** Bildraster von PhotoSnap (vom Befehl `photosnap` erzeugt, nicht von Hand). */
export type PhotoRasterJson = {
  /** Bildpunkte waagrecht */
  breite: number;
  /** Bildpunkte senkrecht */
  hoehe: number;
  /** Farblagen in Stickreihenfolge: eine (Graustufen) oder vier (Cyan, Magenta, Gelb, Schwarz) */
  lagen: PhotoLageJson[];
  /** Größe eines Bildpunkts in mm */
  mm: number;
};

/** Eingabe von `photosnap`. */
export type PhotosnapEingabe = {
  aufloesung?: (PhotoAufloesung | null);
  /** Breite in mm (Standard 80) */
  breite?: number | null;
  /**
   * farbig in vier Lagen (Cyan, Magenta, Gelb, Schwarz) statt einfarbig (Standard einfarbig wie im Original)
   * @default false
   */
  farbig?: boolean;
  /** Garnkarte: die Lagen bekommen die nächsten Garne (CIEDE2000); ohne Angabe die Druckfarben */
  garnkarte?: string | null;
  hintergrund?: (PhotoHintergrund | null);
  /** Bilddatei (PNG, JPG, BMP …) */
  pfad: string;
  /** Rahmen des neuen Designs */
  rahmen?: string | null;
  /** Winkel der Reihen in Grad (Standard 0) */
  winkel?: number | null;
};

/** Ergebnis von `photosnap`. */
export type PhotosnapErgebnis = {
  /** Größe in mm */
  breite: number;
  /** neues Design mit einem Objekt `photosnap` */
  design: DesignWert;
  hoehe: number;
  /** Zahl der Farblagen (1 oder 4) */
  lagen: number;
};

/**
 * PhotoSnap (Handbuch S. 119–120): Foto als Reihen von Zickzackstichen, deren Abstand der Helligkeit folgt („line
 * printer“). Bildmitte = Mitte des Objekts; Lage und Größe über `transform`.
 */
export type PhotosnapJson = {
  /** größter Abstand in mm an den hellsten Stellen (1 … 12; Standard 10 – Stil 0x2828) */
  abstand_max?: number | null;
  /** kleinster Abstand in mm an den dunkelsten Stellen (0,4 … 5; Standard 0,8 – Stil 0x2828) */
  abstand_min?: number | null;
  /** Auflösung = Breite der Reihen (Standard mittel 2,0 mm) */
  aufloesung?: (PhotoAufloesung | null);
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /** Stoff hell (Standard) oder dunkel */
  hintergrund?: (PhotoHintergrund | null);
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  /** Farblagen gekreuzt (Standard) oder versetzt */
  lagen?: (PhotoLagen | null);
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  raster: PhotoRasterJson;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  /** Winkel der Reihen in Grad (Standard 0 wie im Original; das Handbuch-Beispiel zeigt 45°) */
  winkel?: number | null;
};

/**
 * Profil des Verlaufs (Dialog „Gradient Fill“: Linear, Dec-Linear, Convex, Concave; „Color Blending“ zusätzlich
 * Inc_Exp/Dec-Exp und Convex-Concave/Concave-Convex). Abstand vom kleinsten zum größten Wert über die Fläche
 */
export type Profil = ("linear" | "abnehmend" | "konvex" | "konkav" | "exp_zunehmend" | "exp_abnehmend" | "konvex_konkav" | "konkav_konvex");

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

/** Form eines eigenen Rahmens (Handbuch Kap. 6: Format Rectangle / Oval) */
export type RahmenForm = ("rechteck" | "oval");

/** Stickrahmen: Stickfeld und Umriss. */
export type RahmenInfo = {
  /** Stickfeld x in mm */
  breite: number;
  /** eigener Rahmen: seine Angabe (für `eigener_rahmen` im Design); ohne = Rahmen der artista 185 */
  eigen?: (EigenerRahmenJson | null);
  /** Umriss des Stickfelds [[x, y], …] in mm */
  form: Punkt[];
  hinweis: string;
  /** Stickfeld y in mm */
  hoehe: number;
  name: string;
};

/** Eingabe von `rahmen_loeschen`. */
export type RahmenLoeschenEingabe = {
  name: string;
};

/** Eingabe von `rahmen_speichern`. */
export type RahmenSpeichernEingabe = {
  rahmen: EigenerRahmenJson;
};

/**
 * Ausgefranster Rand (Textured Edge, ART-Datensatz 0x32: Seite, Spanne, Texture). Standard wie der Effekt-Datensatz
 * im Stil der Musterdesigns (Seite 2, Spanne 1,5 mm).
 */
export type RandJson = {
  /**
   * Glätte 0 … 1 („Texture“-Regler: 0 = rau, jede Reihe frei; 1 = glatt)
   * @default 0.1
   */
  glaette?: number;
  /**
   * Seite in Stichrichtung (links = Anfang der Reihen bzw. Satin-Seite A)
   * @default "rechts"
   */
  seite?: RandSeite;
  /**
   * Spanne in mm („Span“): Enden streuen bis so weit nach innen und außen (0,1 … 10)
   * @default 1.5
   */
  spanne?: number;
};

/** Seite des ausgefransten Rands, in Stichrichtung gesehen (Dialog „Textured Edge“: Left, Both, Right) */
export type RandSeite = ("links" | "rechts" | "beide");

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

/**
 * Rückstich (Backstitch, Craft Stitch 0x2850): Stiche vor und zurück mit Überlappung, wie von Hand gestickt – eine
 * feine, durchgehende Linie. Ohne Angabe gelten die Werte des Originals (Stil `<0 Software Defaults>`).
 */
export type RueckstichJson = {
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  /** Stichlänge in mm (0,5 … 12,7; Original 3,0) */
  laenge?: number | null;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  punkte: PunktAngabe;
  /** Fadenstärke in mm = Abstand der Hin- und Rückstiche (0 … 1; Original 0,2) */
  staerke?: number | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Striche je Stich: 3 oder 5 (Original 3) */
  striche?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /** Überlappung in % des Stichs, um die jeder Rückstich in den vorigen greift (0 … 100; Original 50) */
  ueberlappung?: number | null;
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /** Säulen an Knicken teilen: Innenwinkel in Grad, 0 … 180 (0 = nie wie der Auto Digitizer; 87 = TrueType) */
  knickwinkel?: number | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  unterlage?: UnterlageJson;
  versatz?: (Versatz | null);
};

/** Stichart einer Satinsäule, Satinlinie oder Satinfläche */
export type SatinStichart = ("satin" | "zickzack");

/** Computer-Schrift (TrueType/OpenType). */
export type SchriftInfo = {
  familie: string;
  /** aus Google Fonts heruntergeladen (liegt in der Ablage des Programms) */
  heruntergeladen: boolean;
  /** Name wie im Design-Feld `schrift` („Arial Bold“) */
  name: string;
  pfad: string;
  stil: string;
  /** Stile (Namen aus `schrift_stile`): eigene Einordnung gängiger Schriften, sonst die von Google, sonst leer */
  stile: string[];
};

/** Stichart einer Schrift mit Original-Alphabet */
export type SchriftStichart = ("satin" | "auto" | "step" | "muster");

/** Stile der Schriftauswahl (eigene Einordnung, `Analyse_Teil3/21_schriften.md` §6). */
export type SchriftStile = {
  /** Woher die Einordnung stammt */
  quelle: string;
  /** alle Stile in Anzeigereihenfolge (Filter der Oberfläche) */
  stile: StilInfo[];
};

/** Vorschau einer Computer-Schrift. */
export type SchriftVorschau = {
  /** umschließendes Rechteck [x0, y0, x1, y1] in mm */
  begrenzung?: number[] | null;
  /** Zeichen des Probetexts, die die Schrift nicht hat */
  fehlende_zeichen: string;
  /** Umrisse des Probetexts in mm (geschlossene Linien, füllen nach Nonzero-Regel) */
  linien: FormLinie[];
  /** fehlt, wenn die Schrift nicht messbar ist */
  stickbarkeit?: (Stickbarkeit | null);
};

/** Eingabe von `schrift_vorschau`. */
export type SchriftVorschauEingabe = {
  /** Höhe in mm (Standard 10) */
  hoehe?: number | null;
  /** Name wie im Design-Feld `schrift` (installiert oder heruntergeladen) */
  schrift: string;
  /** Probetext (Standard „Abc“) */
  text?: string | null;
};

/** Eingabe von `schrift_wechseln`: genau eins von `alphabet` und `schrift`. */
export type SchriftWechselEingabe = {
  /** neues Original-Alphabet → Objekt wird `text` */
  alphabet?: string | null;
  design: DesignWert;
  /** Nummer des Objekts (Text oder Computer-Schrift, ab 0) */
  objekt: number;
  /** neue Computer-Schrift → Objekt wird `ttf` */
  schrift?: string | null;
};

/** Eingabe von `schriften_liste`. */
export type SchriftenListeEingabe = {
  /**
   * Schriftordner neu lesen (z. B. nach dem Installieren einer Schrift im System)
   * @default false
   */
  neu_einlesen?: boolean;
};

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

/** Eingabe von `spiegeln_kopieren` (Mirror-Merge). */
export type SpiegelnKopierenEingabe = {
  /** Ankerpunkt [x, y] in mm (Anzeige): die Spiegellinie(n) gehen durch ihn */
  achse: Punkt;
  /**
   * `links_rechts` = an einer senkrechten Linie (Mirror-Merge Horizontal), `oben_unten` = an einer waagrechten
   * (Vertical), `beide` = drei Kopien (Horizontal & Vertical)
   */
  art: Spiegeln;
  design: DesignWert;
  /** gewählte Objekte (Gruppen als Ganzes) */
  objekte: number[];
  /**
   * überlappende Flächen (Step, Muster, Stippling, Satinfläche) mit dem Original zu einem Objekt vereinigen
   * @default false
   */
  zusammenfuehren?: boolean;
};

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

/** Stichart eines Objekts wechseln. */
export type StichartEingabe = {
  /**
   * neue Objektart: Umriss lauf, dreifach, satinlinie, blanket, motivlinie, rueckstich, stielstich · Füllung tatami,
   * satinflaeche, muster, motivfuellung, stippling, kreuzstich
   */
  art: string;
  design: DesignWert;
  /** Nummer des Objekts (ab 0) */
  objekt: number;
};

/** Ergebnis von `stichart_aendern`. */
export type StichartErgebnis = {
  /** Hinweise (z. B. Löcher entfallen beim Wechsel zur Linie) */
  hinweise: string[];
  /** das Objekt mit neuer Stichart (ersetzt das alte; Stiche danach neu berechnen) */
  objekt: ObjektWert;
};

/**
 * **Stichdaten** aus einer fremden Stickdatei (DST, PES, JEF, VP3, EXP; Befehl `stickdatei_lesen`): feste Einstiche
 * wie „Grade D“ im Original – werden nicht neu berechnet und nicht zusätzlich vernäht. Transformationen bewegen die
 * Einstiche; Skalieren ändert die Dichte, nicht die Stichzahl (Hinweis ab ±5 % wie im Handbuch S. 256).
 */
export type SticheJson = {
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  /** Herkunft, z. B. „rose.pes (PES)“ – nur zur Anzeige */
  quelle?: string | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /**
   * Einstiche, Base64 (vom Befehl `stickdatei_lesen` erzeugt – nicht von Hand bearbeiten): Datensätze x, y als i16
   * in 0,1 mm, +y oben; (−32768, 0) beginnt einen neuen Zug (Sprung)
   */
  stiche: string;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
};

/** Wie gut sich eine Schrift sticken lässt (eigene Erweiterung, `stick_kern::stickbarkeit`). */
export type Stickbarkeit = {
  /** dicke Striche in Teilen der Höhe (10 % der Strichlänge sind dicker) */
  dick: number;
  /** dünne Striche in Teilen der Höhe (10 % der Strichlänge sind dünner) */
  duenn: number;
  /**
   * kleinste Höhe der Großbuchstaben in mm, ab der die dünnen Striche mindestens 0,8 mm breit sind (kalibriert an
   * den empfohlenen Höhen der Original-Alphabete)
   */
  stickbar_ab: number;
};

/** Eingabe von `stickdatei_lesen`. */
export type StickdateiEingabe = {
  /** Stickdatei (Endung .dst, .pes, .pec, .jef, .vp3, .exp) */
  pfad: string;
  /** Rahmen des neuen Designs (ohne Angabe: der kleinste Rahmen der 185, in den alle Stiche passen) */
  rahmen?: string | null;
};

/** Ergebnis von `stickdatei_lesen`. */
export type StickdateiErgebnis = {
  /** Größe in mm */
  breite: number;
  /** neues Design: ein Objekt `stiche` je Farbe, Mitte des Designs = Rahmenmitte */
  design: DesignWert;
  /** Zahl der Farben */
  farben: number;
  /** Format („DST“, „PES“ …) */
  format: string;
  /** Hinweise (unbekannte Farbnummer, größer als jeder Rahmen …) */
  hinweise: string[];
  hoehe: number;
  /** Zahl der Einstiche */
  stiche: number;
};

/**
 * Stielstich (Stemstitch, Craft Stitch 0x2855): schräge, sich überlappende Stiche zu einer kordelartigen Linie.
 * Ohne Angabe gelten die Werte des Originals (Stil `<0 Software Defaults>`).
 */
export type StielstichJson = {
  /** Abstand der Stiche in mm, quer zu den Stichen gemessen (0,2 … 10; Original 1,5) */
  abstand?: number | null;
  /** Ausstich („exit point“) [x, y] in mm: das Objekt endet dort */
  ausstich?: (Punkt | null);
  /** Auto Jump (ohne Angabe: Stil – bei Schrift aus, sonst an mit 12,7 / 7,0 mm) */
  auto_jump?: (AutoJumpJson | null);
  /** Linienstärke in mm = Breite der Kordel (0,5 … 10; Original 2,0) */
  breite?: number | null;
  /**
   * Verstärken (Double Run): dieselben Stiche rückwärts noch einmal, gleiche Farbe, ohne Verbinder
   * @default false
   */
  doppellauf?: boolean;
  /** dreifach („Triple“: drei Fäden nebeneinander) statt einfach (Original: dreifach) */
  dreifach?: boolean | null;
  /**
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
  /**
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  /**
   * Name des Objekts (Objektliste; `aussparen` nennt Objekte beim Namen). Bei Text ohne Angabe: der Text
   * @default ""
   */
  name?: string;
  punkte: PunktAngabe;
  /** Fadenstärke in mm = Abstand der drei Fäden bei dreifach (0 … 1; Original 0,2) */
  staerke?: number | null;
  /** Step-Muster 1–20 für Tatami-Flächen und Step-Schrift (ohne Angabe: Stoff/Stil) */
  step_muster?: number | null;
  /** Skalieren, Drehen, Spiegeln, Schrägstellen, Verschieben um die Mitte des Objekts – Stiche werden neu berechnet */
  transform?: (Transformation | null);
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  /** Winkel der Stiche zur Linie in Grad (10 … 90; Original 50) */
  winkel?: number | null;
};

/** Ein Stil der Schriftauswahl. */
export type StilInfo = {
  /** kurze Erklärung für den Anwender (Tooltip) */
  beschreibung: string;
  /** „Gotisch“, „Schreibschrift“ … – so in den Feldern `stile` der Listen */
  name: string;
};

/**
 * Stippling (Craft Stitch „Stipple“): die Fläche wird mit einer mäandernden Laufstichlinie gefüllt, die sich nicht
 * kreuzt.
 */
export type StipplingJson = {
  /** Schlaufenabstand in mm = Abstand benachbarter Bahnen (0,5 … 20; Handbuch-Beispiel 3,0) */
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
  /** Stichlänge in mm (0,5 … 12,7; Handbuch-Beispiel 2,0) */
  laenge?: number | null;
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
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  umriss: PunktAngabe;
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * Buchstabenhöhe in mm (Original 1–200 mm; Vorlage 10)
   * @default 10.0
   */
  hoehe?: number;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
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
   * **Effekte** (Handbuch Kap. 24, Objekteigenschaften → „Effects“): Verlauf, ausgefranster Rand, Welle. Wirken bei
   * Step- und Musterflächen (`tatami`, `muster`) und Satin (`satin`, `satinlinie`, `satinflaeche`; Welle nur bei
   * Flächen), sonst ohne Wirkung. An = Feld vorhanden
   */
  effekte?: (EffekteJson | null);
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
   * **Gruppe** („Group“ im Original, Handbuch S. 134; im ART-Format `EmbroideryGroup`): Schlüssel in `gruppen` des
   * Designs. Objekte derselben Gruppe werden in der Oberfläche zusammen gewählt und mit der Transformation der
   * Gruppe gemeinsam verschoben, skaliert und gedreht (z. B. ein umgewandeltes Bild als ein Element)
   */
  gruppe?: string | null;
  /**
   * Höhe der Großbuchstaben in mm
   * @default 10.0
   */
  hoehe?: number;
  /**
   * **Closest Join nur für dieses Objekt:** ohne eigenen Einstich beginnt es nahe dem vorigen Ende (wie
   * `kuerzeste_verbindung` des Designs). „Bild umwandeln“ und „SVG einlesen“ setzen es, wenn sie in ein vorhandenes
   * Design einfügen (Auto Digitizer: „determines the stitching sequence based on closest join“, Handbuch S. 116)
   * @default false
   */
  kuerzeste?: boolean;
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
  /**
   * **Umrandung** um die Fläche (eigene Erweiterung): an = Feld vorhanden, z. B. `{}` für Dreifachstich schwarz,
   * 0,3 mm nach innen
   */
  umrandung?: (UmrandungJson | null);
  /**
   * Unterlage: `"auto"` (Standard, Stil „Schrift“ wie im Original: Zickzack unter den Satinsäulen, Step unter
   * Flächen), `"keine"` oder Arten in Stickfolge für Satin/auto, z. B. `["mittellauf", "zickzack"]` (Tatami: jede
   * Liste = an). `true`/`false` (alte Schreibweise) = auto/keine.
   */
  unterlage?: UnterlageJson;
  /** Stichwinkel (Tatami, Muster) */
  winkel?: number | null;
  /** Wortabstand in % der Höhe (Vorlage 25) */
  wortabstand?: number | null;
  /** Zeilenabstand (Lücke) in % der Höhe (Vorlage 60); Zeilen trennt `\n` im Text */
  zeilenabstand?: number | null;
};

/** Stichart einer Computer-Schrift */
export type TtfStichart = ("tatami" | "lauf" | "dreifach" | "satin" | "auto" | "muster");

/** Stichart der Umrandung */
export type UmrandungArt = ("lauf" | "dreifach" | "satin");

/**
 * **Umrandung** – eigene Erweiterung (Wunsch Nutzer 21.11.2026; im Original legt man dafür von Hand eine zweite Linie
 * an, Handbuch S. 114): eine Linie um die Fläche des Objekts, gestickt gleich nach dem Objekt in eigener Farbe. Bei
 * Text läuft sie um jeden Buchstaben, bei mehreren Teilen um jeden Teil (Löcher bekommen auch eine Linie). Sie liegt um
 * `einzug` nach innen, damit nach dem Zusammenziehen des Stoffs keine Lücke zwischen Füllung und Umrandung bleibt.
 * Wirkt bei Objekten mit Fläche (Flächen, Satin, Satinlinie, Text, Monogramm, Applikation); offene Linien haben keine.
 */
export type UmrandungJson = {
  /**
   * Stichart der Linie (Standard Dreifach)
   * @default "dreifach"
   */
  art?: UmrandungArt;
  /**
   * Satinkante **außen** um die Fläche (nur bei `art` satin): die Innenkante liegt `einzug` mm in der Fläche, die
   * Kante wächst nach außen, die Farbe des Objekts bleibt ganz sichtbar (dicke Umrandung im einfachen Modus). Ohne:
   * Mitte der Kante `einzug` mm in der Fläche (eigene Festlegung, Wunsch Nutzer 21.11.2026)
   * @default false
   */
  aussen?: boolean;
  /**
   * Breite der Satinkante in mm, 0,5 … 10 (nur bei `art` satin; Standard 2 wie die Satinlinie). Die Kante ist ein Band
   * um die Fläche, in Satinsäulen zerlegt wie „Satin automatisch“ (`satinflaeche`)
   * @default 2.0
   */
  breite?: number;
  /**
   * Breite der Satinkante in **Prozent der Buchstabenhöhe**, 1 … 30 (nur Text und Monogramm, gilt statt `breite`;
   * Größe/Skalieren zählt mit). Einfacher Modus: 6 → 3 mm bei 50 mm hohen Buchstaben; unter 1 mm wird daraus eine
   * kräftige Linie (Dreifach, Stichlänge 12 % der Höhe, 1–2,5 mm) wie bei EasyStitch (Wunsch Nutzer 21.11.2026,
   * eigene Festlegung)
   */
  breite_anteil?: number | null;
  /**
   * Versatz der Linie nach innen in mm, 0 … 3 (Standard 0,3 – eigene Festlegung, am Kalibrierblatt V12 prüfen); bei
   * `aussen`: wie weit die Innenkante der Satinkante in der Fläche liegt
   * @default 0.3
   */
  einzug?: number;
  /**
   * Garnfarbe „#rrggbb“ (Standard schwarz)
   * @default "#000000"
   */
  farbe?: string;
  /** Stichlänge in mm bei Lauf/Dreifach, 0,3 … 12,7 (ohne Angabe: Stil) */
  laenge?: number | null;
};

/** Art einer Unterlage (in Stickfolge) */
export type UnterlageArt = ("mittellauf" | "edge_walk" | "zickzack" | "doppelzickzack");

/**
 * Unterlage: `"auto"` (Standard, wie „Auto Underlay“ des Stils), `"keine"` oder eine Liste der Arten in Stickfolge.
 * Alte Schreibweise: Arten mit „+“ verbunden (`"edge_walk+zickzack"`).
 */
export type UnterlageJson = (UnterlageWahl | UnterlageArt[] | boolean);

/** Unterlage automatisch oder keine */
export type UnterlageWahl = ("auto" | "keine");

/**
 * Verlauf (Gradient Fill, ART-Datensatz 0x34). Standard wie der Effekt-Datensatz im Stil der Musterdesigns
 * (0,4 / 2,0 mm, Profil 1).
 */
export type VerlaufJson = {
  /**
   * größter Abstand in mm (offenste Stelle, „Maximum Spacing“; 0,1 … 10)
   * @default 2.0
   */
  max?: number;
  /**
   * kleinster Abstand in mm (dichteste Stelle; 0,1 … 10)
   * @default 0.4
   */
  min?: number;
  /**
   * Profil (Dialog „Gradient Fill“ und „Color Blending“)
   * @default "linear"
   */
  profil?: Profil;
};

/** Lage einer Linie zur gezeichneten Linie */
export type Versatz = ("mitte" | "seite");

/** Eintrag der Vorlagenliste. */
export type VorlageInfo = {
  garnkarte?: string | null;
  name: string;
  /** Zahl der Objekte (z. B. Mustertext) */
  objekte: number;
  rahmen?: string | null;
  /** Standardvorlage für „Neu“ */
  standard: boolean;
  stoff?: string | null;
  /** Werkseinstellung (keine Datei – nur bei Normal) */
  werk: boolean;
  /** Zahl der gespeicherten Einzelwerte (`einstellungen`) */
  werte: number;
};

/** Eingabe von `vorlage_neu`/`vorlage_loeschen`. */
export type VorlageName = {
  name: string;
};

/** Eingabe von `vorlage_speichern`. */
export type VorlageSpeichernEingabe = {
  design: DesignWert;
  /** Name der Vorlage („Normal“ = Standardvorlage für „Neu“) */
  name: string;
  /**
   * bei `ganz`: Objekte mitspeichern (z. B. Mustertext); ohne = nur Werte, Rahmen, Stoff, Garnkarte
   * @default true
   */
  objekte?: boolean;
  umfang?: VorlageUmfang;
};

/** Was in die Vorlage kommt */
export type VorlageUmfang = ("ganz" | "werte");

/** Wert eines Felds, mit dem der Kern rechnet, wenn das Feld leer ist. */
export type WirksamerWert = {
  /** Feld des Objekts wie im Design-JSON (z. B. „winkel“, „step_muster“) */
  feld: string;
  /**
   * Herkunft: `stil` (Stoff/Design/Objekt-Einstellungen), `alphabet`, `muster` (Füllmuster), `motivsatz`,
   * `vorlage` (feste Vorlage des Originals), `schrift` (Computer-Schrift), `handbuch` (Beispielwert des Handbuchs)
   */
  herkunft: string;
  /** Anzeige statt `wert`, z. B. „auto 0,40–0,51“ */
  text?: string | null;
  /**
   * Wert (Zahl in der Einheit des Felds, Wahrheitswert oder Text); fehlt, wenn es nicht **ein** Wert ist (z. B.
   * Auto Spacing ändert den Abstand mit der Breite) – dann steht die Anzeige in `text`
   */
  wert?: unknown;
};

/** Ein Klick des Zauberstabs. */
export type ZauberKlick = {
  stichart: ZauberStich;
  /** Lage im Vorschaubild von `bild_vorschau`, 0 … 1 von links */
  x: number;
  /** Lage im Vorschaubild, 0 … 1 von oben */
  y: number;
};

/**
 * Stichart einer angeklickten Fläche (wie die gewählte Stichart vor dem Klick im Original: Füllstich → Fläche,
 * Konturstich → Umriss).
 */
export type ZauberStich = ("step" | "satin" | "auto" | "lauf" | "dreifach" | "satinlinie");

/** Eingabe von `zauberstab`: das Bild wie bei `bild_vorschau` und die Klicks. */
export type ZauberstabEingabe = {
  bild: BildEingabe;
  klicks: ZauberKlick[];
};

/** Ergebnis von `zauberstab`. */
export type ZauberstabErgebnis = {
  /** neues Design mit einem Objekt je getroffener Fläche (Koordinaten wie bei `bild_umwandeln`: Bildmitte = 0) */
  design: DesignWert;
  /** Klicks ohne Fläche (durchsichtig oder zu klein) */
  hinweise: string[];
  /** Zahl der Objekte */
  objekte: number;
};

/** Ein Design (Wurzel von design.schema.json). */
export type Design = DesignJson;

/** Alle Befehle des Kerns mit Ein- und Ausgabe. */
export interface Befehle {
  /** Vorschau eines Original-Alphabets: Umrisse eines Probetexts und Zeichenvorrat */
  alphabet_vorschau: { eingabe: AlphabetVorschauEingabe; ausgabe: AlphabetVorschau };
  /** Original-Alphabete (50) */
  alphabete_liste: { eingabe: Leer; ausgabe: AlphabetInfo[] };
  /** Ausrichten (links, rechts, oben, unten, Mitten) nach dem zuletzt gewählten Objekt – Gruppen als Ganzes */
  ausrichten: { eingabe: AusrichtenEingabe; ausgabe: AusrichtenErgebnis };
  /** Beispieldesign mit allen Sticharten der ersten Ausbaustufe */
  beispiel_design: { eingabe: Leer; ausgabe: DesignWert };
  /** Bild in Stickflächen umwandeln (neues Design) */
  bild_umwandeln: { eingabe: BildEingabe; ausgabe: BildErgebnis };
  /** Vorschau der Bildumwandlung: vereinfachtes Bild (PNG) und Farbliste zum Weglassen – vor dem Umwandeln */
  bild_vorschau: { eingabe: BildEingabe; ausgabe: BildVorschau };
  /** Blackwork Run: gewählte Linien zu einem automatisch geordneten Objekt zusammenfassen (Stichart des ersten) */
  blackwork_lauf: { eingabe: BlackworkEingabe; ausgabe: BlackworkErgebnis };
  /** Vorlagen der Craft-Stitch-Leiste (Rückstich, Stielstich, Blackwork, Candlewicking, Lacework, Stippling) mit den Werten des Originals */
  craft_vorlagen: { eingabe: Leer; ausgabe: CraftVorlage[] };
  /** Textdatei lesen */
  datei_lesen: { eingabe: Pfad; ausgabe: string };
  /** Stiche berechnen – bei jeder Änderung am Design aufrufen (Stiche werden nie skaliert, immer neu berechnet) */
  design_berechnen: { eingabe: DesignEingabe; ausgabe: Ergebnis };
  /** Design einfügen wie „Insert Design“: Objekte anhängen, auf Wunsch als eine Gruppe (Bild, SVG, Datei als Element) */
  design_einfuegen: { eingabe: EinfuegenEingabe; ausgabe: EinfuegenErgebnis };
  /** Formen je Objekt (Umrisse, Linien, Satinseiten, Ein-/Ausstich) zum Zeichnen und Bearbeiten mit der Maus */
  design_formen: { eingabe: DesignEingabe; ausgabe: ObjektForm[] };
  /** Design prüfen, ohne Stiche zu berechnen: Fehler (mit Objektnummer) und Hinweise */
  design_pruefen: { eingabe: DesignEingabe; ausgabe: Pruefung };
  /** JSON-Schema des Design-Formats (Felder, Standardwerte, Auswahlwerte, Beschreibungen) */
  design_schema: { eingabe: Leer; ausgabe: unknown };
  /** Stiche als EXP-Datei speichern (Ergebnis: Zahl der Bytes) */
  exp_speichern: { eingabe: ExpEingabe; ausgabe: number };
  /** Farben mischen (Color Blending): Objekt in zwei Lagen mit gegenläufigem Verlauf teilen, obere Lage in der zweiten Farbe, beide gruppiert */
  farben_mischen: { eingabe: FarbenMischenEingabe; ausgabe: FarbenMischenErgebnis };
  /** Füllmuster (242) mit Linien einer Kachel für die Vorschau */
  fuellmuster_liste: { eingabe: Leer; ausgabe: MusterInfo[] };
  /** Garne einer Garnkarte (für die Auswahl von Hand) */
  garnkarte_garne: { eingabe: GarnkartenName; ausgabe: GarnInfo[] };
  /** Namen der Garnkarten (32 der CD) */
  garnkarten_liste: { eingabe: Leer; ausgabe: string[] };
  /** Stichdaten des zuletzt gelesenen Maschinendesigns als EXP speichern */
  gelesen_exp_speichern: { eingabe: ExpZiel; ausgabe: number };
  /** Google-Schrift herunterladen (Internet) in die Ablage des Programms – danach wie eine installierte Schrift */
  google_schrift_laden: { eingabe: GoogleLadenEingabe; ausgabe: SchriftInfo };
  /** Katalog der Google Fonts (1 833 Schriften, freie Lizenzen) mit Stilen zum Filtern und „geladen“ */
  google_schriften_liste: { eingabe: Leer; ausgabe: GoogleKatalog };
  /** Objekt in Knoten umwandeln (Formen → Punkte/Bézier, Transformation angewandt) – für das Knoten-Werkzeug */
  in_knoten_umwandeln: { eingabe: ObjektEingabe; ausgabe: ObjektWert };
  /** Kranz (Wreath): Kopien der Auswahl rund um einen Mittelpunkt, mit Kaleidoskop jede zweite gespiegelt */
  kranz: { eingabe: KranzEingabe; ausgabe: KopienErgebnis };
  /** Bild → Kreuzstich: neues Design, je Farbe ein Kreuzstich-Objekt mit seinen Zellen */
  kreuzstich_bild: { eingabe: KreuzstichBildEingabe; ausgabe: KreuzstichBildErgebnis };
  /** Bild → Kreuzstich wie „Auto Stitch Image“: Vorschau auf dem Raster (ein Bildpunkt je Kreuz) und Farbliste */
  kreuzstich_vorschau: { eingabe: KreuzstichBildEingabe; ausgabe: BildVorschau };
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
  /** Monogramm-Rahmen aus dem Katalog (47 in 8 Kategorien, eigene Erweiterung) mit Zeichnung, Farben und Schriftvorschlag */
  monogramm_rahmen: { eingabe: Leer; ausgabe: MonogrammKatalog };
  /** Motiv aus einem eigenen Motivsatz löschen (Motive der CD bleiben); liefert die Motivsätze danach */
  motiv_loeschen: { eingabe: MotivLoeschenEingabe; ausgabe: MotivsatzInfo[] };
  /** Ausgewählte Objekte als Motiv in einen eigenen Motivsatz speichern (Create Pattern; Ablage im Datenordner) */
  motiv_speichern: { eingabe: MotivSpeichernEingabe; ausgabe: MotivsatzInfo };
  /** Motivsätze (14 der CD und eigene) für Motivlinie und Motivfüllung */
  motive_liste: { eingabe: Leer; ausgabe: MotivsatzInfo[] };
  /** Motive eines Motivsatzes mit Maßen und Linien für die Vorschau */
  motivsatz_motive: { eingabe: MotivsatzName; ausgabe: MotivInfo[] };
  /** Wirksame Werte eines Objekts: für leere Felder der Wert, mit dem der Kern rechnet (Stil/Stoff, Alphabet, Füllmuster, Motivsatz, Vorlage) – Anzeige im Inspektor statt „Stil“ */
  objekt_werte: { eingabe: ObjektEingabe; ausgabe: WirksamerWert[] };
  /** Parameterkatalog: jeder einstellbare Wert mit Standard, Bereich, Quelle, Modus einfach/experte/automatisch */
  parameter_katalog: { eingabe: Leer; ausgabe: Parameter[] };
  /** PhotoSnap: Foto als Reihen von Zickzackstichen (einfarbig oder vier Farblagen) – neues Design mit einem Objekt */
  photosnap: { eingabe: PhotosnapEingabe; ausgabe: PhotosnapErgebnis };
  /** Stickrahmen der artista 185 mit Stickfeld und Umriss, danach die eigenen Rahmen (Feld `eigen`) */
  rahmen_liste: { eingabe: Leer; ausgabe: RahmenInfo[] };
  /** Eigenen Rahmen löschen; Ergebnis: neue Rahmenliste */
  rahmen_loeschen: { eingabe: RahmenLoeschenEingabe; ausgabe: RahmenInfo[] };
  /** Eigenen Rahmen speichern (Rechteck/Oval, Handbuch Kap. 6) – gleicher Name ersetzt; Ergebnis: neue Rahmenliste */
  rahmen_speichern: { eingabe: RahmenSpeichernEingabe; ausgabe: RahmenInfo[] };
  /** Objekte umordnen wie Menü „Arrange“: wie ausgewählt, an Anfang/Ende, vor/nach Nummer, nach Farbe */
  reihenfolge_aendern: { eingabe: ReihenfolgeEingabe; ausgabe: ReihenfolgeErgebnis };
  /** Fläche in Satinsäulen zerlegen (Vorschau der automatischen Stichrichtung) */
  saeulen_vorschau: { eingabe: SaeulenEingabe; ausgabe: SaeuleInfo[] };
  /** Stile der Schriftauswahl (Gotisch, Schreibschrift, Modern … mit Erklärung) in Anzeigereihenfolge */
  schrift_stile: { eingabe: Leer; ausgabe: SchriftStile };
  /** Vorschau einer Computer-Schrift (Umrisse eines Probetexts) mit „stickbar ab … mm“ */
  schrift_vorschau: { eingabe: SchriftVorschauEingabe; ausgabe: SchriftVorschau };
  /** Schrift eines Texts wechseln: Original-Alphabet (`text`) ↔ Computer-Schrift (`ttf`), Text und Satz bleiben */
  schrift_wechseln: { eingabe: SchriftWechselEingabe; ausgabe: ObjektWert };
  /** installierte Computer-Schriften (TrueType/OpenType) und heruntergeladene Google Fonts */
  schriften_liste: { eingabe: SchriftenListeEingabe; ausgabe: SchriftInfo[] };
  /** Mirror-Merge: gespiegelte Kopie(n) der Auswahl an einer Linie durch den Ankerpunkt, auf Wunsch zusammengeführt */
  spiegeln_kopieren: { eingabe: SpiegelnKopierenEingabe; ausgabe: KopienErgebnis };
  /** Step-Muster 1–20 (Tabelle des Originals) mit Einstichen einer Probefläche für Vorschau-Kacheln */
  step_muster_liste: { eingabe: Leer; ausgabe: StepMusterInfo[] };
  /** Stichart wechseln (Umriss: lauf, dreifach, satinlinie, blanket, motivlinie, rueckstich, stielstich · Füllung: tatami, satinflaeche, muster, motivfuellung, stippling) – Form bleibt */
  stichart_aendern: { eingabe: StichartEingabe; ausgabe: StichartErgebnis };
  /** Endungen der Stickdateien, die `stickdatei_lesen` kennt (für den Dateidialog) */
  stickdatei_endungen: { eingabe: Leer; ausgabe: string[] };
  /** Fremde Stickdatei öffnen (DST, PES, PEC, JEF, VP3, EXP): neues Design mit festen Stichen je Farbe (Objektart `stiche`, wie „Grade D“ im Original), kleinster passender Rahmen der 185 */
  stickdatei_lesen: { eingabe: StickdateiEingabe; ausgabe: StickdateiErgebnis };
  /** Stoffe (8 + Software Defaults) mit den wichtigsten Werten */
  stoffe_liste: { eingabe: Leer; ausgabe: StoffInfo[] };
  /** SVG-Datei einlesen (z. B. aus Inkscape): Stichart nach Breite wie EasyStitch, neues Design */
  svg_lesen: { eingabe: SvgEingabe; ausgabe: SvgErgebnis };
  /** Textdatei schreiben */
  text_speichern: { eingabe: TextDatei; ausgabe: null };
  /** Paare [früher, später] gestickter Objekte, deren Flächen sich überdecken */
  ueberdeckungen: { eingabe: DesignEingabe; ausgabe: number[][] };
  /** Vorlage löschen (Normal → zurück zur Werkseinstellung); Ergebnis: neue Liste */
  vorlage_loeschen: { eingabe: VorlageName; ausgabe: VorlageInfo[] };
  /** Neues Design aus einer Vorlage (Kopie mit Name „Design“) */
  vorlage_neu: { eingabe: VorlageName; ausgabe: DesignWert };
  /** Design als Vorlage speichern (`umfang` ganz) oder nur seine Werte in eine Vorlage (`werte`); Ergebnis: neue Liste */
  vorlage_speichern: { eingabe: VorlageSpeichernEingabe; ausgabe: VorlageInfo[] };
  /** Vorlagen (Handbuch Kap. 9): Normal zuerst (ohne Datei = Werkseinstellung) */
  vorlagen_liste: { eingabe: Leer; ausgabe: VorlageInfo[] };
  /** Leitlinie für den Wave-Effekt vorschlagen (Objektkoordinaten, für `effekte.welle`) */
  welle_vorschlag: { eingabe: ObjektEingabe; ausgabe: unknown };
  /** Zauberstab (Magic Wand): angeklickte Flächen des vorbereiteten Bildes als Fläche oder Umriss (neues Design) */
  zauberstab: { eingabe: ZauberstabEingabe; ausgabe: ZauberstabErgebnis };
}

/** Namen aller Befehle. */
export const BEFEHLE = [
  "alphabet_vorschau",
  "alphabete_liste",
  "ausrichten",
  "beispiel_design",
  "bild_umwandeln",
  "bild_vorschau",
  "blackwork_lauf",
  "craft_vorlagen",
  "datei_lesen",
  "design_berechnen",
  "design_einfuegen",
  "design_formen",
  "design_pruefen",
  "design_schema",
  "exp_speichern",
  "farben_mischen",
  "fuellmuster_liste",
  "garnkarte_garne",
  "garnkarten_liste",
  "gelesen_exp_speichern",
  "google_schrift_laden",
  "google_schriften_liste",
  "in_knoten_umwandeln",
  "kranz",
  "kreuzstich_bild",
  "kreuzstich_vorschau",
  "maschine_abbrechen",
  "maschine_lesen",
  "maschine_liste",
  "maschine_planen",
  "maschine_ports",
  "maschine_pruefen",
  "maschine_senden",
  "monogramm_rahmen",
  "motiv_loeschen",
  "motiv_speichern",
  "motive_liste",
  "motivsatz_motive",
  "objekt_werte",
  "parameter_katalog",
  "photosnap",
  "rahmen_liste",
  "rahmen_loeschen",
  "rahmen_speichern",
  "reihenfolge_aendern",
  "saeulen_vorschau",
  "schrift_stile",
  "schrift_vorschau",
  "schrift_wechseln",
  "schriften_liste",
  "spiegeln_kopieren",
  "step_muster_liste",
  "stichart_aendern",
  "stickdatei_endungen",
  "stickdatei_lesen",
  "stoffe_liste",
  "svg_lesen",
  "text_speichern",
  "ueberdeckungen",
  "vorlage_loeschen",
  "vorlage_neu",
  "vorlage_speichern",
  "vorlagen_liste",
  "welle_vorschlag",
  "zauberstab",
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
