// Formen mit der Maus zeichnen (Nachtauftrag N0, Handbuch S. 58–62): Referenzpunkte wie im Original – linke
// Maustaste = Eckpunkt, rechte = Kurvenpunkt; Enter schließt ab, ⌫ nimmt den letzten Punkt zurück, Esc bricht ab.
// Aus den Punkten entsteht ein Objekt im Design-JSON (Kern-Form `pfad`, Kreis, Oval, Rechteck, Satinsäule). Die
// echte Geometrie rechnet der Kern (`format::pfad_stuecke`); `pfadD` zeichnet nur die Vorschau beim Zeichnen.
import type { ObjektJson } from "$kern/kern";
import { neuesObjekt, STANDARD_FARBE, STICHART_NAME, type FormWerkzeug, type Vorwahl } from "./einfuegen";

export type ZeichenPunkt = { p: [number, number]; kurve: boolean };

const r2 = (v: number) => Math.round(v * 100) / 100;
const punkt = (q: [number, number]): [number, number] => [r2(q[0]), r2(q[1])];

/** Referenzpunkte als Punktangabe: nur Eckpunkte → Punktliste, sonst Pfad mit Kurvenpunkten */
export function punktAngabe(punkte: ZeichenPunkt[], geschlossen: boolean): unknown {
  if (punkte.every((z) => !z.kurve)) return punkte.map((z) => punkt(z.p));
  return { pfad: { punkte: punkte.map((z) => (z.kurve ? { kurve: punkt(z.p) } : punkt(z.p))), geschlossen } };
}

/** Wie viele Punkte eine Form mindestens braucht, bevor Enter sie abschließt */
export const MINDESTENS: Record<FormWerkzeug, number> = { linie: 2, frei: 3, saeule: 4, kreis: 2, rechteck: 2 };
/** Formen, die nach so vielen Klicks von selbst fertig sind (Rechteck: 2 Ecken, Oval: Mitte + 2 Radien) */
export const AUTOMATISCH_FERTIG: Partial<Record<FormWerkzeug, number>> = { rechteck: 2, kreis: 3 };

export const ZEICHEN_HILFE: Record<FormWerkzeug, string> = {
  linie: "Klick = Eckpunkt, Rechtsklick = Kurvenpunkt · Enter beendet",
  frei: "Klick = Eckpunkt, Rechtsklick = Kurvenpunkt · Enter schließt die Form",
  saeule: "Punkte abwechselnd links und rechts der Säule setzen (je Paar eine Stichrichtung) · Enter beendet",
  kreis: "Mitte klicken, dann Radius · Enter = Kreis, dritter Klick = Oval",
  rechteck: "Eine Ecke klicken, dann die gegenüberliegende · Strg/⌘ = Quadrat",
};

/**
 * Objekt aus den gezeichneten Punkten (Vorwahl Füllung/Umriss wie bei den Formen-Werkzeugen). `umriss` = Stichart,
 * in die der Aufrufer die Fläche über den Kern umwandelt (`stichart_aendern`), wenn Umriss vorgewählt ist.
 */
export function formAusPunkten(
  form: FormWerkzeug,
  punkte: ZeichenPunkt[],
  v: Vorwahl,
  quadrat = false,
): { objekt: ObjektJson; umriss: string | null } | null {
  const farbe = STANDARD_FARBE;
  const fuell = v.fuellung;
  const umriss = v.modus === "umriss" ? v.umriss : null;
  const flaeche = (u: unknown, extra: Record<string, unknown> = {}) =>
    ({
      art: fuell,
      name: STICHART_NAME[fuell] === "Satin" ? "Satinfläche" : "Fläche",
      umriss: u,
      farbe,
      ...(fuell === "muster" ? { muster: "001" } : {}),
      ...extra,
    }) as ObjektJson;
  const n = punkte.length;
  switch (form) {
    case "linie": {
      if (n < 2) return null;
      const o = (neuesObjekt(v.umriss) ?? neuesObjekt("lauf")) as ObjektJson & { punkte?: unknown };
      o.punkte = punktAngabe(punkte, false);
      return { objekt: o, umriss: null };
    }
    case "frei":
      return n < 3 ? null : { objekt: flaeche(punktAngabe(punkte, true)), umriss };
    case "kreis": {
      if (n < 2) return null;
      const [c, r1] = [punkte[0].p, punkte[1].p];
      const a = Math.hypot(r1[0] - c[0], r1[1] - c[1]);
      if (a < 0.5) return null;
      const grad = (Math.atan2(r1[1] - c[1], r1[0] - c[0]) * 180) / Math.PI;
      // Stichwinkel senkrecht zur Linie Mitte → Radiuspunkt (Handbuch S. 61), nur für Step-Flächen
      const winkel = fuell === "tatami" ? { winkel: r2((((grad + 90) % 180) + 180) % 180) } : {};
      if (n < 3) return { objekt: flaeche({ kreis: { mitte: punkt(c), radius: r2(a), start: r2(grad) } }, winkel), umriss };
      // Oval: erster Radius = Halbachse in Richtung r1, zweiter = Abstand von r2 zu dieser Achse
      const r2p = punkte[2].p;
      const [ux, uy] = [(r1[0] - c[0]) / a, (r1[1] - c[1]) / a];
      const b = Math.abs((r2p[0] - c[0]) * -uy + (r2p[1] - c[1]) * ux);
      if (b < 0.5) return null;
      const dreh = r2(grad);
      return {
        objekt: flaeche(
          { oval: { mitte: punkt(c), breite: r2(2 * a), hoehe: r2(2 * b) } },
          { ...winkel, ...(dreh ? { transform: { drehen: dreh } } : {}) },
        ),
        umriss,
      };
    }
    case "rechteck": {
      if (n < 2) return null;
      const [p0, p1] = [punkte[0].p, punkte[1].p];
      let [dx, dy] = [p1[0] - p0[0], p1[1] - p0[1]];
      if (quadrat) {
        const s = Math.max(Math.abs(dx), Math.abs(dy));
        [dx, dy] = [Math.sign(dx || 1) * s, Math.sign(dy || 1) * s];
      }
      if (Math.abs(dx) < 0.5 || Math.abs(dy) < 0.5) return null;
      return {
        objekt: flaeche({ rechteck: { mitte: punkt([p0[0] + dx / 2, p0[1] + dy / 2]), breite: r2(Math.abs(dx)), hoehe: r2(Math.abs(dy)) } }),
        umriss,
      };
    }
    case "saeule": {
      if (n < 4) return null;
      const a = punkte.filter((_, i) => i % 2 === 0);
      const b = punkte.filter((_, i) => i % 2 === 1);
      const o = neuesObjekt("satin") as ObjektJson & { seite_a?: unknown; seite_b?: unknown };
      o.seite_a = punktAngabe(a, false);
      o.seite_b = punktAngabe(b, false);
      return { objekt: o, umriss: null };
    }
  }
}

/**
 * SVG-Pfad (y nach unten gespiegelt) durch Referenzpunkte – nur Anzeige beim Zeichnen und im Knoten-Werkzeug; gleiche
 * Regel wie im Kern (`format::pfad_stuecke`: Ecke–Ecke gerade, sonst Bézier mit Catmull-Rom-Tangenten).
 */
export function pfadD(punkte: ZeichenPunkt[], geschlossen: boolean): string {
  const n = punkte.length;
  if (n === 0) return "";
  const m = (q: [number, number]) => `${q[0]} ${-q[1]}`;
  if (n === 1) return `M${m(punkte[0].p)}`;
  const t = (i: number): [number, number] => {
    if (!punkte[i].kurve) return [0, 0];
    const vor = geschlossen ? (i + n - 1) % n : Math.max(0, i - 1);
    const nach = geschlossen ? (i + 1) % n : Math.min(n - 1, i + 1);
    const f = !geschlossen && (i === 0 || i === n - 1) ? 1 : 0.5;
    return [(punkte[nach].p[0] - punkte[vor].p[0]) * f, (punkte[nach].p[1] - punkte[vor].p[1]) * f];
  };
  let d = `M${m(punkte[0].p)}`;
  const stuecke = geschlossen ? n : n - 1;
  for (let i = 0; i < stuecke; i++) {
    const j = (i + 1) % n;
    const [a, b] = [punkte[i].p, punkte[j].p];
    if (!punkte[i].kurve && !punkte[j].kurve) d += `L${m(b)}`;
    else {
      const [ti, tj] = [t(i), t(j)];
      d += `C${m([a[0] + ti[0] / 3, a[1] + ti[1] / 3])} ${m([b[0] - tj[0] / 3, b[1] - tj[1] / 3])} ${m(b)}`;
    }
  }
  return geschlossen ? d + "Z" : d;
}
