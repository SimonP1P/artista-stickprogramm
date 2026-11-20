# Artista Stickprogramm – Programm (Tauri 2 + Svelte + Rust)

Stand 19.11.2026: erste Oberfläche mit Rust-Kern (`../kern/`): **1 Rahmen** und **2 Stoff** wählen, **3 Design anpassen** (**Schrift** mit den 50 Original-Alphabeten: Text, Alphabet, Höhe, Breite, Farbe – Vorschau beim Tippen; dazu JSON-Editor, Neuberechnung beim Tippen), Stichvorschau (Zoom, Verschieben, Sprünge/Einstiche), Statistik, Farben, Prüfung „passt in den Rahmen“, Design öffnen/speichern, **EXP speichern**. **4 An die Maschine** (20.11.2026, Phase 5): Anschluss wählen (COM-Port oder **Simulation**), **Maschine prüfen** (nur lesen: Firmware, Garnfarbtabelle), **Senden …** = erst planen (nur lesen, Prüfungen wie das Original), dann Dialog mit allen Schreibzugriffen und **Jetzt schreiben** (DATEN, ICON, INFO-Felder, Garnfarbtabelle, zurücklesen); optional danach sticken (Befehl 0x08) nur mit Häkchen „Es ist jemand an der Maschine“. Mitschnitte unter `%LOCALAPPDATA%\de.artista.stickprogramm\logs`. Am Gerät erprobt: prüfen und planen (lesend); Schreiben aus dem Programm noch nicht (Liste „Vor Ort“ in `../ROADMAP.md`). Aufbau und Plan: `../ARCHITECTURE.md`.

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
| `src-tauri/` | Rust-Teil des Programms (Fenster, Befehle an den Rechenkern): `src/lib.rs` – rahmen_liste, stoffe_liste, beispiel_design, design_berechnen, datei_lesen, text_speichern, exp_speichern, **maschine_ports, maschine_pruefen, maschine_planen, maschine_senden** (über `kern/stick-maschine`, Fortschritt als Ereignis `senden_fortschritt`) |
| `src-tauri/alphabete/` | die 50 Original-Alphabete (ESB) der CD, als Ressource mitgeliefert (`HERKUNFT.md`) |
| `src/lib/alphabet_hoehen.ts` | empfohlene Buchstabenhöhen je Alphabet (Handbuch) |
| `src-tauri/tauri.conf.json` | Programmname, Fenster, Kennung `de.artista.stickprogramm` |
