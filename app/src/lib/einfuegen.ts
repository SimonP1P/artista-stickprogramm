// Menü „Einfügen“ und Werkzeugleiste: neue Objekte mit Startwerten in der Rahmenmitte. Formen sind Formen des Kerns
// (`kreis`, `rechteck` …) bzw. wenige Startpunkte – die Oberfläche rechnet keine Geometrie. Danach im Inspektor bzw. in
// der Formansicht anpassen.

import type { ObjektJson } from "$kern/kern";
import { IC } from "./icons";

export const STANDARD_FARBE = "#004780";

/**
 * Werkzeugleiste und Menü „Einfügen“ nach dem Original (Wunsch Nutzer 21.11.2026): **Formen** wie die Design-Leiste
 * (Open Curve, Circle/Oval, Rectangle, Closed Curve, Block Digitizing – Handbuch S. 58) und die **Stichart** getrennt
 * davon wie „Outline/Fill“ + Stichart-Liste (S. 63/64). Neue Formen bekommen die vorgewählte Stichart; gewechselt wird
 * über den Kern (`stichart_aendern`).
 */
export type FormWerkzeug = "linie" | "kreis" | "rechteck" | "frei" | "saeule";
export const FORMEN: { form: FormWerkzeug; name: string; icon: string; hilfe: string }[] = [
  { form: "linie", name: "Linie", icon: IC.formLinie, hilfe: "Offene Linie – bekommt die Umriss-Stichart" },
  { form: "kreis", name: "Kreis / Oval", icon: IC.formKreis, hilfe: "Kreis – Füllung oder Umriss je nach Vorwahl" },
  { form: "rechteck", name: "Rechteck", icon: IC.formRechteck, hilfe: "Rechteck – Füllung oder Umriss je nach Vorwahl" },
  { form: "frei", name: "Freie Form", icon: IC.formFrei, hilfe: "Geschlossene Form – Punkte danach mit dem Knoten-Werkzeug ziehen" },
  { form: "saeule", name: "Säule", icon: IC.formSaeule, hilfe: "Satinsäule mit wechselnder Breite (Block Digitizing)" },
];
/** Schrift und besondere Objekte (eigene Werkzeuge wie im Original) */
export const SCHRIFT = ["text", "ttf", "monogramm"] as const;
export const BESONDERE = ["applikation", "knopfloch"] as const;

/** Sticharten für Linien („Outline“) und Flächen („Fill“) – wie `UMRISS_ARTEN`/`FUELL_ARTEN` im Kern */
export const UMRISS = ["lauf", "dreifach", "satinlinie", "blanket", "motivlinie"] as const;
export const FUELLUNG = ["tatami", "satinflaeche", "muster"] as const;
export const STICHART_NAME: Record<string, string> = {
  lauf: "Laufstich",
  dreifach: "Dreifachstich",
  satinlinie: "Satin",
  blanket: "Blanket",
  motivlinie: "Motive",
  tatami: "Step (Tatami)",
  satinflaeche: "Satin",
  muster: "Füllmuster",
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
};
export type Vorwahl = { modus: "fuellung" | "umriss"; fuellung: string; umriss: string };

/**
 * Neues Objekt für ein Form-Werkzeug. Geschlossene Formen entstehen als Fläche der Vorwahl; soll es ein Umriss werden,
 * wandelt der Aufrufer sie mit `stichart_aendern` um (Form-Logik nur im Kern).
 */
export function formObjekt(form: FormWerkzeug, v: Vorwahl): { objekt: ObjektJson; umriss: string | null } {
  const farbe = STANDARD_FARBE;
  const fuell = v.fuellung;
  const flaeche = (umriss: unknown) =>
    ({ art: fuell, name: STICHART_NAME[fuell] === "Satin" ? "Satinfläche" : "Fläche", umriss, farbe, ...(fuell === "muster" ? { muster: "001" } : {}) }) as ObjektJson;
  const umriss = v.modus === "umriss" ? v.umriss : null;
  switch (form) {
    case "linie": {
      const o = neuesObjekt(v.umriss);
      return { objekt: o ?? (neuesObjekt("lauf") as ObjektJson), umriss: null };
    }
    case "kreis":
      return { objekt: flaeche({ kreis: { mitte: [0, 0], radius: 15 } }), umriss };
    case "rechteck":
      return { objekt: flaeche({ rechteck: { mitte: [0, 0], breite: 40, hoehe: 25 } }), umriss };
    case "frei":
      return {
        objekt: flaeche([
          [-18, -8],
          [-4, -14],
          [16, -10],
          [20, 4],
          [6, 14],
          [-14, 10],
        ]),
        umriss,
      };
    case "saeule":
      return { objekt: neuesObjekt("satin") as ObjektJson, umriss: null };
  }
}

/** FUNKTIONEN.md §7 – Platz vorgesehen, deaktiviert */
export const KOMMT_NOCH = [
  "Formen mit der Maus zeichnen (Punkt für Punkt)",
  "Motivfüllungen",
  "Effekte (Fransen, Welle, Farbverlauf, Farben mischen)",
  "Craft Stitches (Blackwork, Rückstich, Stielstich, Candlewicking, Stippling)",
  "Kreuzstich",
  "Magic Wand, PhotoSnap",
  "Ausrichten, Kranz, Kaleidoskop",
  "Eigene Rahmen, fremde Stickdateien",
  "Vorlagen / Stile speichern",
];

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
      return { art, name: "Text", text: "Text", alphabet: "Swiss Block", hoehe: 15, farbe };
    case "ttf":
      return schrift ? { art, name: "Text", text: "Text", schrift, hoehe: 15, farbe } : null;
    case "monogramm":
      return { art, name: "Monogramm", initialen: "AB", hoehe: 30, rahmen: "raute", farbe };
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
    case "blanket":
      return { art, name: "Blanket", punkte: linie, farbe };
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
        satz: "Heirloom",
        motiv: "705",
        farbe,
      };
    default:
      return null;
  }
}
