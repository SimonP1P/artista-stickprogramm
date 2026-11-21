// Menü „Datei“: Neu, Öffnen, Speichern, Beispiel, Stickdatei (EXP). Dateien liest und schreibt der Kern
// (`datei_lesen`, `text_speichern`, `exp_speichern`), die Auswahl der Datei der Dialog von Tauri.

import { open, save } from "@tauri-apps/plugin-dialog";
import { kern, type DesignJson } from "$kern/kern";
import { arbeit, alsFehler, leeresDesign } from "./arbeit.svelte";

const DESIGN_FILTER = [{ name: "Stickdesign (JSON)", extensions: ["json"] }];

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
  const pfad = await open({ multiple: false, directory: false, filters: DESIGN_FILTER });
  if (typeof pfad !== "string") return;
  try {
    const text = await kern("datei_lesen", { pfad });
    const d = JSON.parse(text) as DesignJson;
    arbeit.ersetzen(d, pfad, false);
    arbeit.auswahl = [];
    arbeit.zeige(`Geöffnet: ${dateiname(pfad)}`, "ok");
  } catch (x) {
    arbeit.zeige(`Öffnen fehlgeschlagen: ${x instanceof SyntaxError ? x.message : alsFehler(x).meldung}`, "err");
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
