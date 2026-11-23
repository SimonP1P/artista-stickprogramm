// Texte für Anwender (UX-Analyse 2.5, Paket C4, 22.11.2026): Die Beschreibungen der Felder kommen aus den
// Doc-Kommentaren des Kerns (`format.rs`) und sind auch für Entwickler geschrieben – mit Fundstellen, Hexwerten,
// JSON-Schreibweise und „wie das Original“. Für Tooltips wird daraus ein Anwendertext.

/** Hinweise für Entwickler: Klammern und Sätze mit diesen Merkmalen fallen weg */
const ENTWICKLER =
  /`[^`]*[{}[\]"][^`]*`|`(?:true|false)`|0x[0-9a-f]|Original|Handbuch|Analyse_|MOD1|\bAMT\b|\bART-Format|DIALOG\.DLL|IMPORT\.DLL|Festlegung|Erweiterung|Nachtauftrag|\bN\d+\b|\b4b\.\d|Nutzer|§|Musterdesign|<[A-Z_ 0-9]+>|\bStil „|\bPRESET|\bSoftware Defaults|Schreibweise|\bKern\b|\bJSON\b/;
/** Sicherheitsstufe der Analyse in Klammern */
const SICHERHEIT = /^\s*(mittel|hoch|niedrig|mittel-hoch|sicher)\s*$/i;
const ORIGINAL = /,?\s*(wie (das|im) Original|des Originals|im Original|aus dem Original)\b/g;

/** Beschreibung aus dem Schema → Erklärung für den Anwender (leer, wenn nichts übrig bleibt) */
export function anwenderText(roh: string | undefined): string {
  if (!roh) return "";
  let s = roh
    .replace(/\s*\n\s*/g, " ")
    .replace(/\*\*([^*]+)\*\*/g, "$1")
    .replace(/Original-Alphabet/g, "Stick-Alphabet")
    // Werte des Originals sind die Standardwerte; Fundstellen (Stil/AMT 0x…) fallen weg
    .replace(/\bOriginal\s+(?=(?:[A-ZÄÖÜ][a-zäöü]+\s+)?\d)/g, "Standard ")
    .replace(/\s*[–-]?\s*\b(?:(?:Stil|AMT|MOD1|Tag)\s+)?0x[0-9a-f]+(?:\s*[/–-]\s*0x[0-9a-f]+)*(?:\s*=\s*[\d,]+)?/gi, "")
    // Abkürzungen sind kein Satzende
    .replace(/\b(z|d|u|o)\. (B|h|a|ä)\./g, "$1.\u2060$2.")
    .replace(/\b(S|Nr|ca|bzw|vgl|max|min)\. /g, "$1.\u2060");
  // innerste Klammern zuerst, bis sich nichts mehr ändert
  for (let alt = ""; alt !== s; ) {
    alt = s;
    s = s.replace(/\s*\(([^()]*)\)/g, (m, innen: string) => {
      if (SICHERHEIT.test(innen)) return "";
      if (!ENTWICKLER.test(innen)) return m;
      // nur die Teile mit Entwicklerhinweisen weglassen, Bereich und Standard bleiben
      const teile = innen.split(/;\s*/).filter((t) => t.trim() && !ENTWICKLER.test(t) && !SICHERHEIT.test(t));
      return teile.length ? ` (${teile.join("; ")})` : "";
    });
  }
  s = s.replace(ORIGINAL, "");
  // Sätze mit verbleibenden Entwicklerhinweisen weglassen
  const saetze = s.split(/(?<=\.)\s+(?=[A-ZÄÖÜ„])/);
  const bleibt = saetze.filter((x) => !ENTWICKLER.test(x));
  // nur Entwicklertext: keine Erklärung (die Beschriftung reicht)
  s = bleibt.join(" ");
  s = s
    .replace(/`([^`]*)`/g, "„$1“")
    .replace(/\s+([,.;:)])/g, "$1")
    .replace(/\(\s*\)/g, "")
    .replace(/[,;:–-]\s*$/, "")
    .replace(/\s{2,}/g, " ")
    .replace(/^[,:;–-]\s*/, "")
    .replace(/\u2060/g, " ")
    .trim();
  return s;
}
