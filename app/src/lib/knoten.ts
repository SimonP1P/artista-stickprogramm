// Knoten-Werkzeug (W5): Knoten aus dem JSON eines Objekts lesen und zurückschreiben. Nur Daten des JSON – Eckpunkte
// der Punktlisten, Referenzpunkte gezeichneter Pfade (`{"pfad": {"punkte": [[x, y] | {"kurve": [x, y]}]}}`, Ecke ↔
// Kurve per Rechtsklick) und Bézier-Knoten mit Anfassern (`{"bezier": [p0, p1, p2, p3]}`, auch unter `"verbinden"`);
// keine Geometrie. Formen (Kreis, Oval, Bogen, Rechteck) und Transformationen wandelt der Kern um
// (`in_knoten_umwandeln`).
import { pfadD, type ZeichenPunkt } from "./zeichnen";

export type Pfad = (string | number)[];
export type Knoten = {
  p: [number, number];
  /** alle Stellen im Objekt-JSON mit diesem Punkt (Nahtstelle zweier Bézierstücke = ein Knoten) */
  pfade: Pfad[];
  /** Anfasser, die beim Ziehen des Knotens mitwandern */
  mit: Pfad[];
  anfasser: boolean;
  /** bei Anfassern: Lage des zugehörigen Knotens (für die Hilfslinie) */
  anker?: [number, number];
  /** Knoten einer Punktliste oder eines Pfads: Liste und Stelle (Entf löscht, Einfg fügt dahinter ein); `ring` =
   *  geschlossene Kontur (Umriss, Loch); `pfad` = Liste der Referenzpunkte eines Pfads */
  liste?: { pfad: Pfad; index: number; ring?: boolean; istPfad?: boolean };
  /** Kurvenpunkt eines Pfads (rund gezeichnet) */
  kurve?: boolean;
};
export type KnotenBild = { knoten: Knoten[]; linien: string[]; umwandeln: boolean };

const FELDER = ["punkte", "umriss", "seite_a", "seite_b"];
type Baum = Record<string, unknown>;

const istPunkt = (p: unknown): p is [number, number] =>
  Array.isArray(p) && p.length === 2 && p.every((z) => typeof z === "number");
const istPunktliste = (v: unknown): v is [number, number][] => Array.isArray(v) && v.length > 0 && v.every(istPunkt);
const istBezier = (v: unknown): v is [number, number][] => Array.isArray(v) && v.length === 4 && v.every(istPunkt);

/** Transformation ohne Wirkung? */
export function ohneTransformation(t: unknown): boolean {
  if (!t || typeof t !== "object") return true;
  const x = t as Baum;
  const v = x.verschieben as number[] | undefined;
  return (
    (x.skalieren == null || x.skalieren === 100) && !x.drehen && !x.schraeg && !x.spiegeln &&
    (!Array.isArray(v) || (v[0] === 0 && v[1] === 0))
  );
}

function anker(out: Knoten[], q: [number, number], pfad: Pfad, mit: Pfad | null, liste?: Knoten["liste"], kurve?: boolean) {
  const da = out.find((k) => !k.anfasser && k.p[0] === q[0] && k.p[1] === q[1]);
  if (da) {
    da.pfade.push(pfad);
    if (mit) da.mit.push(mit);
    return;
  }
  out.push({ p: [q[0], q[1]], pfade: [pfad], mit: mit ? [mit] : [], anfasser: false, liste, kurve });
}

/** Referenzpunkt eines Pfads: Eckpunkt [x, y] oder Kurvenpunkt {"kurve": [x, y]} */
const pfadPunkt = (v: unknown): ZeichenPunkt | null =>
  istPunkt(v) ? { p: v, kurve: false } : v && typeof v === "object" && istPunkt((v as Baum).kurve) ? { p: (v as Baum).kurve as [number, number], kurve: true } : null;
/** Lage (x, y) eines Listeneintrags (Punkt oder Kurvenpunkt) */
export function lage(v: unknown): [number, number] {
  return pfadPunkt(v)?.p ?? [0, 0];
}

/** Knoten einer Punktangabe sammeln; `false`, wenn eine Form darin steckt (erst umwandeln). */
function sammeln(v: unknown, pfad: Pfad, out: Knoten[], d: string[], ring = false): boolean {
  const m = (q: number[]) => `${q[0]} ${-q[1]}`;
  if (istPunktliste(v)) {
    v.forEach((q, i) => {
      anker(out, q, [...pfad, i], null, { pfad, index: i, ring });
      d.push(`${d.length ? "L" : "M"}${m(q)}`);
    });
    return true;
  }
  if (v && typeof v === "object" && !Array.isArray(v)) {
    const o = v as Baum;
    const pf = o.pfad as Baum | undefined;
    if (pf && Array.isArray(pf.punkte)) {
      const punkte = pf.punkte.map(pfadPunkt);
      if (punkte.some((x) => !x)) return false;
      const lp = [...pfad, "pfad", "punkte"];
      (punkte as ZeichenPunkt[]).forEach((z, i) =>
        anker(out, z.p, z.kurve ? [...lp, i, "kurve"] : [...lp, i], null, { pfad: lp, index: i, ring: !!pf.geschlossen, istPfad: true }, z.kurve),
      );
      // Hilfslinie: Kurve durch die Referenzpunkte (nur Anzeige, gleiche Regel wie im Kern)
      d.push(pfadD(punkte as ZeichenPunkt[], !!pf.geschlossen).replace(/^M/, d.length ? "L" : "M"));
      return true;
    }
    if (istBezier(o.bezier)) {
      const q = o.bezier;
      const b = [...pfad, "bezier"];
      anker(out, q[0], [...b, 0], [...b, 1]);
      anker(out, q[3], [...b, 3], [...b, 2]);
      out.push({ p: [q[1][0], q[1][1]], pfade: [[...b, 1]], mit: [], anfasser: true, anker: q[0] });
      out.push({ p: [q[2][0], q[2][1]], pfade: [[...b, 2]], mit: [], anfasser: true, anker: q[3] });
      d.push(`${d.length ? "L" : "M"}${m(q[0])}C${m(q[1])} ${m(q[2])} ${m(q[3])}`);
      return true;
    }
    if (Array.isArray(o.verbinden)) {
      let ok = true;
      o.verbinden.forEach((t, i) => (ok = sammeln(t, [...pfad, "verbinden", i], out, d) && ok));
      return ok;
    }
  }
  return false;
}

/** Knoten und Hilfslinien eines Objekts; `umwandeln` = Formen oder Transformation vorhanden (Kern wandelt um). */
export function knotenBild(objekt: unknown): KnotenBild {
  const leer: KnotenBild = { knoten: [], linien: [], umwandeln: false };
  if (!objekt || typeof objekt !== "object") return leer;
  const o = objekt as Baum;
  const felder: [Pfad, unknown, boolean][] = FELDER.filter((f) => o[f] != null).map((f) => [
    [f],
    o[f],
    f === "umriss",
  ]);
  if (Array.isArray(o.loecher)) o.loecher.forEach((l, i) => felder.push([["loecher", i], l, true]));
  // Leitlinie des Wave-Effekts (Kern `effekte.welle`) – wie ein offener Pfad bearbeitbar
  const fx = o.effekte as Baum | null | undefined;
  if (fx && fx.welle != null) felder.push([["effekte", "welle"], fx.welle, false]);
  if (!felder.length) return leer;
  if (!ohneTransformation(o.transform)) return { ...leer, umwandeln: true };
  const knoten: Knoten[] = [];
  const linien: string[] = [];
  let umwandeln = false;
  for (const [pfad, v, ring] of felder) {
    const k: Knoten[] = [];
    const d: string[] = [];
    if (!sammeln(v, pfad, k, d, ring)) {
      umwandeln = true;
      continue;
    }
    knoten.push(...k);
    linien.push(d.join("") + (ring ? "Z" : ""));
  }
  return { knoten, linien, umwandeln };
}

function holen(o: unknown, pfad: Pfad): unknown {
  let x = o;
  for (const t of pfad) x = (x as Baum)?.[t as string];
  return x;
}

/** Knoten `k` nach `ziel` setzen (alle Stellen), mitwandernde Anfasser um dieselbe Strecke verschieben. */
export function knotenSetzen(objekt: unknown, k: Knoten, ziel: [number, number], runden = (v: number) => v) {
  const [dx, dy] = [ziel[0] - k.p[0], ziel[1] - k.p[1]];
  const setzen = (pfad: Pfad, q: [number, number]) => {
    const eltern = holen(objekt, pfad.slice(0, -1)) as Baum;
    eltern[pfad[pfad.length - 1] as string] = [runden(q[0]), runden(q[1])];
  };
  for (const p of k.pfade) setzen(p, ziel);
  for (const p of k.mit) {
    const q = holen(objekt, p) as [number, number];
    setzen(p, [q[0] + dx, q[1] + dy]);
  }
}

/** Liste, in der ein Knoten steht – nur, wenn er dort allein steht (sonst gehört er auch zu einem Bézierstück).
 *  Einträge: Punkte [x, y] oder bei Pfaden auch Kurvenpunkte {"kurve": [x, y]}. */
export function knotenListe(objekt: unknown, k: Knoten): unknown[] | null {
  if (!k.liste || k.pfade.length !== 1 || k.mit.length) return null;
  const l = holen(objekt, k.liste.pfad);
  return istPunktliste(l) || (k.liste.istPfad && Array.isArray(l) && l.every((x) => !!pfadPunkt(x))) ? (l as unknown[]) : null;
}

/**
 * Knoten zwischen Eck- und Kurvenpunkt umschalten (Rechtsklick, wie die Punktarten im Original). Eine einfache
 * Punktliste wird dafür zum Pfad (Ringe geschlossen). `false`, wenn der Knoten nicht umschaltbar ist (Bézier).
 */
export function knotenUmschalten(objekt: unknown, k: Knoten): boolean {
  if (!k.liste || k.pfade.length !== 1 || k.mit.length) return false;
  const lp = k.liste.pfad;
  const i = k.liste.index;
  if (!k.liste.istPfad) {
    // Punktliste → Pfad: das Feld (lp) wird {"pfad": {"punkte": …, "geschlossen": ring}}
    const eltern = holen(objekt, lp.slice(0, -1)) as Baum;
    const schluessel = lp[lp.length - 1] as string;
    const liste = eltern[schluessel];
    if (!istPunktliste(liste)) return false;
    const punkte: unknown[] = liste.map((q) => [q[0], q[1]]);
    punkte[i] = { kurve: punkte[i] };
    eltern[schluessel] = { pfad: { punkte, geschlossen: !!k.liste.ring } };
    return true;
  }
  const liste = holen(objekt, lp) as unknown[];
  const z = pfadPunkt(liste[i]);
  if (!z) return false;
  liste[i] = z.kurve ? [z.p[0], z.p[1]] : { kurve: [z.p[0], z.p[1]] };
  return true;
}
