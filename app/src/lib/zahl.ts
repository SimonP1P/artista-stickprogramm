// Deutsches Zahlenformat (Komma) für Anzeige und Eingabe. Reine Darstellung, keine Berechnung.

/** Zahl mit höchstens `d` Nachkommastellen, z. B. 12,5 */
export const zahl = (v: number, d = 1) => Number(v).toLocaleString("de-DE", { maximumFractionDigits: d });

/** Zahl mit genau `d` Nachkommastellen, z. B. 86,0 */
export const zahlFest = (v: number, d = 1) =>
  Number(v).toLocaleString("de-DE", { minimumFractionDigits: d, maximumFractionDigits: d });

/** Ganze Zahl mit Tausenderpunkt, z. B. 4.387 */
export const ganz = (v: number) => Math.round(v).toLocaleString("de-DE");

/** Eingabe „12,5“ oder „12.5“ lesen; null bei ungültig */
export function lesen(s: string): number | null {
  const n = parseFloat(String(s).replace(/\s/g, "").replace(",", "."));
  return Number.isFinite(n) ? n : null;
}
