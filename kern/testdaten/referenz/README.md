# Referenz-EXP für den Rust-Kern

Der Test `stick-formate/tests/vergleich_referenz.rs` prüft, dass der Kern für `../vergleich_*.json` byte-genau diese
Dateien erzeugt. Jede **bewusste** Änderung der Stichberechnung ersetzt die Referenzen und wird hier eingetragen, damit
jede Abweichung vom Python-Prototyp nachvollziehbar bleibt.

| Datum | Stand | Änderung gegenüber dem vorigen Stand |
|---|---|---|
| 19.11.2026 | Python-Prototyp `stickprogramm/` | Ausgangspunkt: Rust-Kern erzeugt byte-genau dieselben Dateien wie Python (Commit 50be48d) |
| 19.11.2026 | Original-Verhalten Stufe 1 (`Analyse_Teil3/01_stile_und_verbinder.md`) | Vernähen nach Dialog „Connectors“: 2 Stiche à **1,0 mm** statt 0,8 mm; **letztes Objekt nicht vernäht** (`Always Tie Off Last Object` = 0); Unterlagen-Läufe **variabel** mit Mindeststich/Sehnenabstand je Art aus dem Stil. Geprüft mit einem Stichvergleich (difflib): nur Vernähpunkte, Designende und ein Edge-Walk-Stich am geknickten Satinkeil geändert |
| 19.11.2026 | Original-Verhalten Stufe 2 (`Analyse_Teil3/02_art_stichdaten.md`) | Vernähen wie in den ART-Musterdesigns gemessen: **2 Schritte vor, 2 zurück** entlang des ersten/letzten Stichs, Schrittlänge min(1,0 mm, Stich/2). Stichvergleich: nur zusätzliche Vernähpunkte |
| 19.11.2026 | Original-Verhalten Stufe 3 (`Analyse_Teil3/04_messungen_satin.md`) | **Satin-Abstand = Weg zwischen zwei Einstichen auf derselben Seite** (Handbuch PDF-S. 68, an 184 000 Satinstichen der Musterdesigns gemessen): Vorschub je Einstich halber Abstand statt ganzer Abstand – Satin doppelt so dicht wie bisher. Stichvergleich: nur Satinbahnen geändert, die bisherigen Einstiche bleiben als jeder zweite erhalten |
| 19.11.2026 | Original-Verhalten Stufe 4 (`Analyse_Teil3/04_messungen_satin.md` §2) | **Edge Walk = 0x2747, Mittellauf = 0x2746** (bisher vertauscht): Mindeststich/Sehnenabstand der beiden Läufe getauscht. Referenzen **unverändert** – die Testformen sind so glatt, dass beide Grenzen nicht greifen |
| 19.11.2026 | Original-Verhalten Stufe 5 (`Analyse_Teil3/05_effekte_und_unterlagen.md`) | Unterlage **„auto“ = Auto Underlay des Stils** statt eigener Breitenregel: in Software Defaults und den Stoffen **aus**, eingeschaltet (`unterlage.auto_an`) die vorgewählte Art **Zickzack**; Tatami ohne Angabe folgt demselben Schalter. `vergleich_beispiel`/`vergleich_stoff` jetzt ohne Unterlagen (Satin/Tatami ohne Angabe); in `vergleich_unterlagen.json` die zweite Tatami-Fläche ausdrücklich mit Unterlage – Referenz dort unverändert |
| 19.11.2026 | Original-Verhalten Stufe 6 (`Analyse_Teil3/04_messungen_satin.md` §4) | Satin-Fortschritt auf der **längeren Seite** (Außenseite im Bogen) statt Mittel beider Seiten – in Bögen innen dichter, außen genau der Abstand |
| 19.11.2026 | Original-Verhalten Stufe 7 (`Analyse_Teil3/06_messungen_tatami.md`) | **Tatami-Abstand = Weg zwischen zwei Hinreihen** (Handbuch PDF-S. 74, gemessen 0,49 × / 0,98 ×): Nachbarreihen beim halben Abstand – Tatami und Tatami-Unterlage doppelt so dicht wie bisher; Versatz 0,25 je Reihe bestätigt (unverändert) |
| 19.11.2026 | Original-Verhalten Stufe 8 (`Analyse_Teil3/07_lauf_ecken.md`) | Laufstich sticht an **jedem Eckpunkt** ein (Punktlisten, Rechteckecken, Enden von Kreis/Bogen/Bézier), an Kurvenpunkten nicht – statt „ab 30° Knick“. Geändert nur Lauf/Dreifach am Ende von `vergleich_beispiel` und der Dreifach-Kreis in `vergleich_unterlagen` |

Neu erzeugen: `F:\Entwicklung\target\release\stick.exe erzeugen ..\vergleich_X.json -o .` (danach die SVG löschen).
