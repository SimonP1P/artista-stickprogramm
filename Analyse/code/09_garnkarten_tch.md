# 09 – Garnkarten (`PROGRAM/RES/*.tch`)

**Daten:** [`../daten/garnkarten/`](../daten/garnkarten/) (alle 32 Dateien unverändert) + [`UEBERSICHT.md`](../daten/garnkarten/UEBERSICHT.md)
**Sicherheit:** hoch (Klartext)

## Format
CSV-Text, Zeilenende CR LF, Zeichensatz Latin-1, kein Kopf:
```
Nummer,Marke,Farbname,Dichte,R,G,B
1000,Madeira 40,Black,A,0,0,0
```
| Feld | Bedeutung |
|---|---|
| Nummer | Herstellernummer als Text (führende Nullen bleiben, z. B. `0010`) |
| Marke | Anzeigename der Karte |
| Farbname | |
| Dichte | `A` in allen Zeilen außer einer (`026,HUS,Black,B`). Entspricht dem Feld `thrd_density` (Tag 0x0cba) |
| R,G,B | Bildschirmfarbe 0–255 |

Die Software lädt `default.tch` als Standard und listet `*.tch` aus `RES` (Strings in BLOT.EXE). Eigene Karten lassen sich in der Software anlegen (Handbuch Kap. 10, S. 83–92).

## Enthaltene Karten
| Marke | Dateien (Farben) |
|---|---|
| **Isacord 40** | `Isacord 40 wt-numerical.tch` / `-shades.tch` (je 315, nach Nummer bzw. Farbton sortiert) |
| **Madeira** | `Madeira 40.tch` (288), `Madeira 40-numerical/-shades` (220), Neon/Poly Neon (150/230) |
| **Mettler** | Poly Sheen (243), Silky Sheen (108), Cotton Silk Finish (151); **`Mettler 40 wt-*.tch` enthalten in Wahrheit die Isacord-Daten** (Marke im Feld = „Isacord 40“) |
| **Sulky** | `Sulky-numerical/-shades` (266) |
| **Robison-Anton** | Super Brite Polyester 122 (452), Super Strength Rayon 122 (417) und 152 (417), Quilting 25 wt (10) und 50 wt (32) |
| **DMC** | 360 |
| Format-Paletten | CSD (16), HUS (29), PCS (16), PES (65), SEW (36), XXX (99): Farbtabellen der Fremdformate |
| Sonstige | `Default.tch` (20 Grundfarben), `Applique.tch` (3: Führungslinie, Platzieren & Heften, Zuschneiden & Abdecken) |

Kein Hersteller bietet hier 60er Garn an. Alle Stickgarne sind 40er bzw. 25/50 wt (Quilten).
