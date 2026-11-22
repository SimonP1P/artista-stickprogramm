// Effekte (Kern `effekte`, Nachtauftrag N2): Profile des Verlaufs mit Texten und derselben Formel wie
// `stick_kern::effekte::Profil::anteil` – nur für die Vorschau im Dialog „Farben mischen“; gestickt wird im Kern.
import type { Profil } from "$kern/kern";

/** Profile wie die Dialoge „Gradient Fill“ (erste vier) und „Color Blending“ des Originals */
export const PROFILE: [Profil, string][] = [
  ["linear", "Linear – dicht → offen"],
  ["abnehmend", "Abnehmend – offen → dicht"],
  ["konvex", "Konvex – schnell offen"],
  ["konkav", "Konkav – schnell dicht"],
  ["exp_zunehmend", "Exponentiell zunehmend"],
  ["exp_abnehmend", "Exponentiell abnehmend"],
  ["konvex_konkav", "S-Kurve zunehmend"],
  ["konkav_konvex", "S-Kurve abnehmend"],
];

/** Gegenstück für die obere Lage beim Farben mischen */
export const GEGENSTUECK: Record<Profil, Profil> = {
  linear: "abnehmend",
  abnehmend: "linear",
  konvex: "konkav",
  konkav: "konvex",
  exp_zunehmend: "exp_abnehmend",
  exp_abnehmend: "exp_zunehmend",
  konvex_konkav: "konkav_konvex",
  konkav_konvex: "konvex_konkav",
};

/** Anteil 0 … 1 zwischen dichtem und offenem Abstand an der Stelle t (0 … 1) */
export function anteil(p: Profil, t: number): number {
  t = Math.min(1, Math.max(0, t));
  const exp = (x: number) => (Math.exp(3 * x) - 1) / (Math.exp(3) - 1);
  const s = (x: number) => x * x * (3 - 2 * x);
  switch (p) {
    case "linear": return t;
    case "abnehmend": return 1 - t;
    case "konvex": return t * (2 - t);
    case "konkav": return (1 - t) * (1 - t);
    case "exp_zunehmend": return exp(t);
    case "exp_abnehmend": return exp(1 - t);
    case "konvex_konkav": return s(t);
    case "konkav_konvex": return 1 - s(t);
  }
}

/** Lagen der Reihen über eine Strecke 0 … laenge (mm), wie `Verlauf::reihen` im Kern */
export function reihen(p: Profil, min: number, max: number, laenge: number): number[] {
  const ab = (t: number) => min + (max - min) * anteil(p, t);
  const out: number[] = [];
  for (let y = ab(0) / 4; y < laenge; y += Math.max(0.05, ab(y / laenge) / 2)) out.push(y);
  return out;
}
