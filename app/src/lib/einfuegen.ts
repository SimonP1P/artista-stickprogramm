// Menü „Einfügen“ und Werkzeugleiste: neue Objekte mit Startwerten in der Rahmenmitte. Formen sind Formen des Kerns
// (`kreis`, `rechteck` …) bzw. wenige Startpunkte – die Oberfläche rechnet keine Geometrie. Danach im Inspektor bzw. in
// der Formansicht anpassen.

import type { ObjektJson, UnterlageArt, UnterlageJson } from "$kern/kern";
import { IC } from "./icons";

export const STANDARD_FARBE = "#004780";
/** Unterlage neuer Texte: Mittellauf + Zickzack (Entscheidung des Nutzers 21.11.2026; das Original wählt im Stil
 *  „Schrift“ nur Zickzack vor – `Analyse_Teil3/10` §3). */
export const SCHRIFT_UNTERLAGE: UnterlageArt[] = ["mittellauf", "zickzack"];

/** Unterlage eines Texts als Häkchen: „auto“/ohne Angabe = Stil „Schrift“ (Zickzack), true/false = alte Schreibweise */
export function unterlageHaken(u: unknown): { mittellauf: boolean; zickzack: boolean; andere: UnterlageArt[] } {
  if (Array.isArray(u)) {
    const l = u as UnterlageArt[];
    return { mittellauf: l.includes("mittellauf"), zickzack: l.includes("zickzack"), andere: l.filter((x) => x !== "mittellauf" && x !== "zickzack") };
  }
  const aus = u === "keine" || u === false;
  return { mittellauf: false, zickzack: !aus, andere: [] };
}

/** Häkchen → Wert für `unterlage` (Mittellauf zuerst, dann übrige Arten, Zickzack zuletzt; nichts = „keine“) */
export function unterlageAusHaken(h: { mittellauf: boolean; zickzack: boolean; andere: UnterlageArt[] }): UnterlageJson {
  const l: UnterlageArt[] = [...(h.mittellauf ? ["mittellauf" as const] : []), ...h.andere, ...(h.zickzack ? ["zickzack" as const] : [])];
  return l.length ? l : "keine";
}

/**
 * Werkzeugleiste und Menü „Einfügen“ nach dem Original (Wunsch Nutzer 21.11.2026): **Formen** wie die Design-Leiste
 * (Open Curve, Circle/Oval, Rectangle, Closed Curve, Block Digitizing – Handbuch S. 58) und die **Stichart** getrennt
 * davon wie „Outline/Fill“ + Stichart-Liste (S. 63/64). Neue Formen bekommen die vorgewählte Stichart; gewechselt wird
 * über den Kern (`stichart_aendern`).
 */
export type FormWerkzeug = "linie" | "kreis" | "rechteck" | "frei" | "saeule" | "knopfloch";
export const FORMEN: { form: FormWerkzeug; name: string; icon: string; hilfe: string }[] = [
  { form: "linie", name: "Linie", icon: IC.formLinie, hilfe: "Offene Linie – bekommt die Umriss-Stichart" },
  { form: "kreis", name: "Kreis / Oval", icon: IC.formKreis, hilfe: "Kreis – Füllung oder Umriss je nach Vorwahl" },
  { form: "rechteck", name: "Rechteck", icon: IC.formRechteck, hilfe: "Rechteck – Füllung oder Umriss je nach Vorwahl" },
  { form: "frei", name: "Freie Form", icon: IC.formFrei, hilfe: "Geschlossene Form – Punkte danach mit dem Knoten-Werkzeug ziehen" },
  { form: "saeule", name: "Säule", icon: IC.formSaeule, hilfe: "Satinsäule mit wechselnder Breite (Block Digitizing)" },
];
/** Schrift (ein Eintrag „Text“: Stick-Alphabet oder Computer-Schrift wählt man im Dialog „Schrift wählen“) */
export const SCHRIFT = ["text", "monogramm"] as const;

/** Sticharten für Linien („Outline“) und Flächen („Fill“) – wie `UMRISS_ARTEN`/`FUELL_ARTEN` im Kern */
export const UMRISS = ["lauf", "dreifach", "satinlinie", "blanket", "motivlinie", "rueckstich", "stielstich"] as const;
export const FUELLUNG = ["tatami", "satinflaeche", "muster", "motivfuellung", "stippling", "kreuzstich", "applikation"] as const;
export const STICHART_NAME: Record<string, string> = {
  lauf: "Laufstich",
  dreifach: "Dreifachstich",
  satinlinie: "Satin",
  blanket: "Blanket",
  motivlinie: "Motive",
  tatami: "Step (Tatami)",
  satinflaeche: "Satin",
  muster: "Füllmuster",
  motivfuellung: "Motive",
  rueckstich: "Rückstich",
  stielstich: "Stielstich",
  stippling: "Stippling",
  kreuzstich: "Kreuzstich",
  applikation: "Applikation",
};
export const STICHART_HILFE: Record<string, string> = {
  lauf: "eine Stichreihe – dünne Linien",
  dreifach: "jeder Stich dreimal – kräftige Linie",
  satinlinie: "Satinband entlang der Linie (Breite einstellbar)",
  blanket: "Langettenstich – Kamm-Wirkung, z. B. Applikationsrand",
  motivlinie: "kleine Motive in einer Reihe",
  tatami: "für große, unregelmäßige Flächen",
  satinflaeche: "für schmale Flächen bis ~8 mm, glänzend",
  muster: "Fläche mit Muster (242 Muster)",
  motivfuellung: "Motive in Reihen über die Fläche",
  rueckstich: "feine Linie wie von Hand – Vor- und Rückstiche",
  stielstich: "Kordel aus schrägen Stichen – Stiele, Ranken",
  stippling: "mäandernde Laufstichlinie über die Fläche",
  kreuzstich: "Kreuze auf einem Raster wie von Hand gestickt",
  applikation: "Stoff aufnähen: Platzierung, Halt zum Auflegen, Heften, Abdeckstich",
};
/** Vorwahl für neue Formen; `vorlage` = Handstich-Vorlage (Craft, `craft_vorlagen`) zur Stichart */
export type Vorwahl = { modus: "fuellung" | "umriss"; fuellung: string; umriss: string; vorlage: { fuellung?: string; umriss?: string } };

/** Neues Objekt mit Startwerten; `schrift` = erste Computer-Schrift (für `ttf`) */
export function neuesObjekt(art: string, schrift?: string): ObjektJson | null {
  const farbe = STANDARD_FARBE;
  const linie: [number, number][] = [
    [-20, -5],
    [-7, 5],
    [7, -5],
    [20, 5],
  ];
  switch (art) {
    case "text":
      return { art, name: "Text", text: "Text", alphabet: "Swiss Block", hoehe: 15, stichart: "auto", unterlage: SCHRIFT_UNTERLAGE, farbe };
    case "ttf":
      return schrift ? { art, name: "Text", text: "Text", schrift, hoehe: 15, stichart: "auto", unterlage: SCHRIFT_UNTERLAGE, farbe } : null;
    case "monogramm":
      return { art, name: "Monogramm", initialen: "AB", hoehe: 30, rahmen: "raute", stichart: "auto", unterlage: SCHRIFT_UNTERLAGE, farbe };
    case "lauf":
    case "dreifach":
      return { art, name: art === "lauf" ? "Laufstich" : "Dreifachstich", punkte: linie, farbe };
    case "satinlinie":
      return { art, name: "Satinlinie", punkte: linie, breite: 3, farbe };
    case "satin":
      return {
        art,
        name: "Satinsäule",
        seite_a: [
          [-20, 2],
          [20, 2],
        ],
        seite_b: [
          [-20, -2],
          [20, -2],
        ],
        farbe,
      };
    case "tatami":
      return { art, name: "Fläche", umriss: { kreis: { mitte: [0, 0], radius: 15 } }, farbe };
    case "satinflaeche":
      return { art, name: "Satinfläche", umriss: { rechteck: { mitte: [0, 0], breite: 30, hoehe: 6 } }, farbe };
    case "muster":
      return { art, name: "Füllmuster", umriss: { kreis: { mitte: [0, 0], radius: 15 } }, muster: "001", farbe };
    case "motivfuellung":
      return { art, name: "Motivfüllung", umriss: { kreis: { mitte: [0, 0], radius: 20 } }, farbe };
    case "blanket":
      return { art, name: "Blanket", punkte: linie, farbe };
    case "rueckstich":
      return { art, name: "Rückstich", punkte: linie, farbe };
    case "stielstich":
      return { art, name: "Stielstich", punkte: linie, farbe };
    case "stippling":
      return { art, name: "Stippling", umriss: { kreis: { mitte: [0, 0], radius: 20 } }, farbe };
    case "kreuzstich":
      return { art, name: "Kreuzstich", umriss: { kreis: { mitte: [0, 0], radius: 15 } }, farbe };
    case "applikation":
      return { art, name: "Applikation", umriss: { kreis: { mitte: [0, 0], radius: 20 } }, farbe };
    case "knopfloch":
      return { art, name: "Knopfloch", typ: "standard", position: [0, 0], laenge: 20, farbe };
    case "motivlinie":
      return {
        art,
        name: "Motivlinie",
        punkte: [
          [-30, 0],
          [30, 0],
        ],
        farbe,
      };
    default:
      return null;
  }
}
