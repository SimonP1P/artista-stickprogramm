// Menü „Einfügen“ und Werkzeugleiste: neue Objekte mit Startwerten in der Rahmenmitte. Formen sind Formen des Kerns
// (`kreis`, `rechteck` …) bzw. wenige Startpunkte – die Oberfläche rechnet keine Geometrie. Danach im Inspektor bzw. in
// der Formansicht anpassen.

import type { ObjektJson } from "$kern/kern";

export const STANDARD_FARBE = "#004780";

/** Objektarten in der Reihenfolge des Entwurfs (Menü Einfügen, Werkzeugleiste) */
export const EINFUEGEN: { art: string; gruppe: 1 | 2 }[] = [
  { art: "text", gruppe: 1 },
  { art: "ttf", gruppe: 1 },
  { art: "monogramm", gruppe: 1 },
  { art: "lauf", gruppe: 2 },
  { art: "dreifach", gruppe: 2 },
  { art: "satinlinie", gruppe: 2 },
  { art: "satin", gruppe: 2 },
  { art: "tatami", gruppe: 2 },
  { art: "satinflaeche", gruppe: 2 },
  { art: "muster", gruppe: 2 },
  { art: "blanket", gruppe: 2 },
  { art: "applikation", gruppe: 2 },
  { art: "knopfloch", gruppe: 2 },
];

/** FUNKTIONEN.md §7 – Platz vorgesehen, deaktiviert */
export const KOMMT_NOCH = [
  "Motivlinien und Motivfüllungen",
  "Effekte (Fransen, Welle, Farbverlauf, Farben mischen)",
  "Craft Stitches (Blackwork, Rückstich, Stielstich, Candlewicking, Stippling)",
  "Kreuzstich",
  "Magic Wand, PhotoSnap",
  "Ausrichten, Gruppieren, Kranz, Kaleidoskop",
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
    default:
      return null;
  }
}
