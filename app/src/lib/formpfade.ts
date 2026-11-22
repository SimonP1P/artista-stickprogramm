// Formen des Kerns (`design_formen`, `alphabet_vorschau` …) als SVG-Pfade: Satinsäulen als Paare Seite A/B gefüllt,
// geschlossene Umrisse nach Nonzero-Regel, offene Linien als Strich. Nur Anzeige – die Geometrie kommt aus dem Kern.
// y wird umgedreht (Kern: +y oben, SVG: +y unten).
import type { FormLinie } from "$kern/kern";

export const pfad = (p: number[][], zu: boolean) =>
  p.map(([x, y], j) => `${j ? "L" : "M"}${x.toFixed(2)} ${(-y).toFixed(2)}`).join("") + (zu ? "Z" : "");

export function formPfade(linien: FormLinie[]): { saeulen: string[]; flaechen: string; offen: string[] } {
  const saeulen: string[] = [];
  const offen: string[] = [];
  let flaechen = "";
  for (let i = 0; i < linien.length; i++) {
    const l = linien[i];
    const n = linien[i + 1];
    if (l.art === "winkellinie") continue;
    if (l.art === "seite_a" && n?.art === "seite_b") {
      saeulen.push(pfad([...l.punkte, ...[...n.punkte].reverse()], true));
      i++;
    } else if (l.geschlossen) flaechen += pfad(l.punkte, true);
    else offen.push(pfad(l.punkte, false));
  }
  return { saeulen, flaechen, offen };
}
