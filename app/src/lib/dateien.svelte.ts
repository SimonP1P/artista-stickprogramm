// Menü „Datei“: Neu, Öffnen, Speichern, Beispiel, Stickdatei (EXP). Dateien liest und schreibt der Kern
// (`datei_lesen`, `text_speichern`, `exp_speichern`), die Auswahl der Datei der Dialog von Tauri. Fremde Stickdateien
// (DST, PES, JEF, VP3, EXP) öffnet `stickdatei_lesen` als feste Stiche je Farbe (Objektart `stiche`).

import { open, save } from "@tauri-apps/plugin-dialog";
import { kern, type DesignJson } from "$kern/kern";
import { arbeit, alsFehler, leeresDesign } from "./arbeit.svelte";

const DESIGN_FILTER = [{ name: "Stickdesign (JSON)", extensions: ["json"] }];
/** Endungen von `stickdatei_lesen` (Kern `stickdatei_endungen`) */
const STICK_ENDUNGEN = ["dst", "pes", "pec", "jef", "vp3", "exp"];
const OEFFNEN_FILTER = [
  { name: "Stickdesign oder Stickdatei", extensions: ["json", ...STICK_ENDUNGEN, ...STICK_ENDUNGEN.map((e) => e.toUpperCase())] },
  ...DESIGN_FILTER,
  { name: "Stickdatei (DST, PES, JEF, VP3, EXP)", extensions: [...STICK_ENDUNGEN, ...STICK_ENDUNGEN.map((e) => e.toUpperCase())] },
];

const istStickdatei = (pfad: string) => STICK_ENDUNGEN.includes(pfad.split(".").pop()?.toLowerCase() ?? "");

/** Fremde Stickdatei lesen; Meldung mit Format, Stichzahl, Farben und Hinweisen */
async function stickdatei(pfad: string): Promise<{ design: DesignJson; meldung: string }> {
  const r = await kern("stickdatei_lesen", { pfad });
  const mm = (x: number) => x.toLocaleString("de-DE", { maximumFractionDigits: 1 });
  const meldung =
    `${r.format}: ${r.stiche.toLocaleString("de-DE")} Stiche, ${r.farben} ${r.farben === 1 ? "Farbe" : "Farben"}, ${mm(r.breite)} × ${mm(r.hoehe)} mm` +
    (r.hinweise.length ? ` – ${r.hinweise.join(" · ")}` : "");
  return { design: r.design, meldung };
}

function nachfragenVerwerfen(): boolean {
  return !arbeit.geaendert || confirm("Ungespeicherte Änderungen verwerfen?");
}

export function neu() {
  if (!nachfragenVerwerfen()) return;
  const d = leeresDesign();
  // Rahmen und Stoff behalten – sie hängen an der Maschine bzw. am Material, nicht am Design
  d.rahmen = arbeit.design.rahmen ?? d.rahmen;
  d.stoff = arbeit.design.stoff ?? d.stoff;
  arbeit.ersetzen(d, null, false);
  arbeit.auswahl = [];
}

export async function oeffnen() {
  if (!nachfragenVerwerfen()) return;
  const pfad = await open({ multiple: false, directory: false, filters: OEFFNEN_FILTER });
  if (typeof pfad !== "string") return;
  try {
    if (istStickdatei(pfad)) {
      // feste Stiche: als neues, noch nicht gespeichertes Design (Speichern fragt nach dem JSON-Namen)
      const { design, meldung } = await stickdatei(pfad);
      arbeit.ersetzen(design, null, false);
      arbeit.auswahl = [];
      arbeit.zeige(`Geöffnet: ${dateiname(pfad)} · ${meldung}`, "ok");
      return;
    }
    const text = await kern("datei_lesen", { pfad });
    const d = JSON.parse(text) as DesignJson;
    arbeit.ersetzen(d, pfad, false);
    arbeit.auswahl = [];
    arbeit.zeige(`Geöffnet: ${dateiname(pfad)}`, "ok");
  } catch (x) {
    arbeit.zeige(`Öffnen fehlgeschlagen: ${x instanceof SyntaxError ? x.message : alsFehler(x).meldung}`, "err");
  }
}

/** Gespeichertes Design als **ein Element** (Gruppe) ins aktuelle einfügen – „Insert Design“ des Originals */
export async function designEinfuegen() {
  const pfad = await open({ multiple: false, directory: false, filters: OEFFNEN_FILTER });
  if (typeof pfad !== "string") return;
  try {
    const fremd = istStickdatei(pfad) ? await stickdatei(pfad) : null;
    const einfuegen = fremd ? fremd.design : (JSON.parse(await kern("datei_lesen", { pfad })) as DesignJson);
    const r = await kern("design_einfuegen", {
      design: $state.snapshot(arbeit.design) as DesignJson,
      einfuegen,
      gruppe: { name: dateiname(pfad).replace(/\.[a-z0-9]+$/i, ""), herkunft: "gruppiert" },
    });
    arbeit.ersetzen(r.design, arbeit.pfad, true);
    arbeit.auswahl = r.objekte;
    arbeit.zeige(`Eingefügt: ${dateiname(pfad)}${fremd ? ` · ${fremd.meldung}` : ""}`, "ok");
  } catch (x) {
    arbeit.zeige(`Einfügen fehlgeschlagen: ${x instanceof SyntaxError ? x.message : alsFehler(x).meldung}`, "err");
  }
}

export async function speichern(unter = false) {
  let pfad = arbeit.pfad;
  if (unter || !pfad) {
    const p = await save({ filters: DESIGN_FILTER, defaultPath: `${arbeit.design.name ?? "Design"}.json` });
    if (!p) return;
    pfad = p;
  }
  try {
    const text = JSON.stringify($state.snapshot(arbeit.design), null, 2) + "\n";
    await kern("text_speichern", { pfad, text });
    arbeit.pfad = pfad;
    arbeit.geaendert = false;
    arbeit.zeige(`Gespeichert: ${dateiname(pfad)}`, "ok");
  } catch (x) {
    arbeit.zeige(`Speichern fehlgeschlagen: ${alsFehler(x).meldung}`, "err");
  }
}

export async function beispiel() {
  if (!nachfragenVerwerfen()) return;
  try {
    const d = await kern("beispiel_design", {});
    arbeit.ersetzen(d, null, false);
    arbeit.auswahl = [];
  } catch (x) {
    arbeit.zeige(alsFehler(x).meldung, "err");
  }
}

export async function expExportieren() {
  const p = await save({
    filters: [{ name: "Stickdatei (EXP)", extensions: ["exp"] }],
    defaultPath: `${arbeit.design.name ?? "Design"}.exp`,
  });
  if (!p) return;
  try {
    const bytes = await kern("exp_speichern", { design: $state.snapshot(arbeit.design) as DesignJson, pfad: p });
    arbeit.zeige(`EXP gespeichert: ${dateiname(p)} (${bytes.toLocaleString("de-DE")} Byte)`, "ok");
  } catch (x) {
    arbeit.zeige(`EXP-Export fehlgeschlagen: ${alsFehler(x).meldung}`, "err");
  }
}

export function dateiname(p: string) {
  return p.split(/[\\/]/).pop() ?? p;
}
