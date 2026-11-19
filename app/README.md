# Artista Stickprogramm – Programm (Tauri 2 + Svelte + Rust)

Stand: Grundgerüst aus der Tauri-Vorlage (19.11.2026), noch ohne Stickfunktionen. Aufbau und Plan: `../ARCHITECTURE.md`.

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
| `src/` | Oberfläche (SvelteKit, TypeScript) |
| `src-tauri/` | Rust-Teil des Programms (Fenster, Befehle an den Rechenkern) |
| `src-tauri/tauri.conf.json` | Programmname, Fenster, Kennung `de.artista.stickprogramm` |
