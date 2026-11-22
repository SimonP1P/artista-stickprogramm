#!/usr/bin/env python3
"""Katalog der Monogramm-Rahmen für den Rechenkern erzeugen (eigene Erweiterung, Nutzer 21.11.2026).

Quelle: `Arbeitsdaten/monogramm_rahmen/` – 47 SVG-Rahmen (90 × 90 mm, nur Pfade, Linien 2–4 mm, höchstens 3 Farben)
in 8 Kategorien-Ordnern, dazu `KATALOG_47_RAHMEN.csv` (Name, Kategorie, Google Font, Anzahl Initialen, Versalhöhen
links/Mitte/rechts, Beschreibung). Vom Nutzer mit ChatGPT entworfen; Original-Software hat nur Diamond2/3 mit Raute
und Zierwinkel (Handbuch Kap. 21).

Ziel: `kern/stick-vektor/daten/monogramm_rahmen.json` (wird in `stick-vektor` eingebunden, `monogramm.rs`). Neue
Rahmen: SVG in einen Kategorien-Ordner, Zeile in die CSV, dieses Skript laufen lassen, Tests im Kern.

Aufruf (aus dem Projektordner):  python3 Arbeitsdaten/werkzeuge/monogramm_rahmen_katalog.py
"""

import csv
import json
import pathlib
import re
import sys

WURZEL = pathlib.Path(__file__).resolve().parents[2]
QUELLE = WURZEL / "Arbeitsdaten" / "monogramm_rahmen"
ZIEL = WURZEL / "kern" / "stick-vektor" / "daten" / "monogramm_rahmen.json"


def main() -> int:
    zeilen = list(csv.DictReader((QUELLE / "KATALOG_47_RAHMEN.csv").open(encoding="utf-8-sig"), delimiter=";"))
    dateien = {p.name: p for p in QUELLE.glob("*/*.svg")}
    # Kategorien in der Reihenfolge der Ordner (01 … 08), Name aus der CSV
    ordner_kat: dict[str, str] = {}
    for z in zeilen:
        p = dateien.get(z["Dateiname"])
        if p is None:
            print(f"fehlt: {z['Dateiname']}", file=sys.stderr)
            return 1
        ordner_kat.setdefault(p.parent.name, z["Kategorie"].strip())
    kategorien = [ordner_kat[o] for o in sorted(ordner_kat)]

    rahmen = []
    for z in zeilen:
        p = dateien[z["Dateiname"]]
        svg = p.read_text(encoding="utf-8").strip()
        if re.search(r"<(text|image)\b", svg):
            print(f"{p.name}: Text/Bild in der SVG", file=sys.stderr)
            return 1
        hoehen = [float(h) for h in z["Buchstabenhoehen_mm (links/mitte/rechts)"].split("/")]
        if len(hoehen) != int(z["Buchstaben"]):
            print(f"{p.name}: {len(hoehen)} Höhen für {z['Buchstaben']} Buchstaben", file=sys.stderr)
            return 1
        rahmen.append(
            {
                "nr": int(z["Nr."]),
                "name": z["Motiv"].strip(),
                "kategorie": z["Kategorie"].strip(),
                "datei": p.name,
                "schrift": z["Google Font (exakter Name)"].strip(),
                "hoehen": hoehen,
                "beschreibung": z["Beschreibung"].strip(),
                "svg": svg,
            }
        )
    namen = [r["name"].lower() for r in rahmen]
    if len(set(namen)) != len(namen):
        print("Namen doppelt", file=sys.stderr)
        return 1
    # Reihenfolge: Kategorie, dann Nummer
    rahmen.sort(key=lambda r: (kategorien.index(r["kategorie"]), r["nr"]))
    aus = {
        "quelle": "Arbeitsdaten/monogramm_rahmen (47 SVG, 8 Kategorien; Entwurf des Nutzers mit ChatGPT, 21.11.2026)",
        "kategorien": kategorien,
        "rahmen": rahmen,
    }
    ZIEL.parent.mkdir(parents=True, exist_ok=True)
    ZIEL.write_text(json.dumps(aus, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"{len(rahmen)} Rahmen, {len(kategorien)} Kategorien → {ZIEL.relative_to(WURZEL)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
