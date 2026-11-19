# Artista Stickprogramm – Programm (Tauri 2 + Svelte + Rust)

Stand 19.11.2026: erste Oberfläche mit Rust-Kern (`../kern/`): **1 Rahmen** und **2 Stoff** wählen, **3 Design anpassen** (JSON-Editor, Neuberechnung beim Tippen), Stichvorschau (Zoom, Verschieben, Sprünge/Einstiche), Statistik, Farben, Prüfung „passt in den Rahmen“, Design öffnen/speichern, **EXP speichern**. **4 Senden** folgt mit Phase 5. Aufbau und Plan: `../ARCHITECTURE.md`.

## Starten (Windows-PC)
Voraussetzungen siehe `../ARCHITECTURE.md` §5 (Rust und Node auf F:\Entwicklung, VS Build Tools 2019, Windows SDK im PATH).
```powershell
cd app
npm install                                   # einmalig
npm run tauri dev                             # Entwicklungsmodus mit Live-Neuladen
npm run tauri build -- --debug --no-bundle    # Programm bauen -> F:\Entwicklung\target\debug\artista-stickprogramm.exe
```
Hinweis: Nach dem Einrichten der Umgebungsvariablen ein neues Terminal öffnen, sonst fehlen `cargo`/`node` im PATH.

## Ordner
| Pfad | Inhalt |
|---|---|
| `src/` | Oberfläche (SvelteKit, TypeScript): `routes/+page.svelte` (Hauptfenster), `lib/Vorschau.svelte` (Stichvorschau), `lib/typen.ts` |
| `src-tauri/` | Rust-Teil des Programms (Fenster, Befehle an den Rechenkern): `src/lib.rs` – rahmen_liste, stoffe_liste, beispiel_design, design_berechnen, datei_lesen, text_speichern, exp_speichern |
| `src-tauri/tauri.conf.json` | Programmname, Fenster, Kennung `de.artista.stickprogramm` |
