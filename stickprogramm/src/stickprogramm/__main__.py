"""Kommandozeile:  py -m stickprogramm <befehl> …

  erzeugen DESIGN.json [-o ORDNER]     Stiche berechnen -> DESIGN.exp + DESIGN.svg
  anzeigen DATEI.exp [-o DATEI.svg]    vorhandene EXP-Datei als SVG darstellen
  beispiel [-o ORDNER]                 Beispieldesign mit allen Sticharten erzeugen
  rahmen                               bekannte Rahmen auflisten
  stoffe                               Stoffe und ihre Werte auflisten
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from .design import Design
from .eingabe import design_aus_dict, design_laden
from .einstellungen import STOFFE
from .exp import exp_bytes, exp_lesen
from .rahmen import RAHMEN, rahmen
from .stiche import Art, StichFolge
from .vorschau import svg_text


def statistik(folge: StichFolge) -> str:
    laengen = folge.stichlaengen()
    bb = folge.begrenzung()
    zeilen = [
        f"  Stiche:       {folge.anzahl(Art.STICH)}",
        f"  Sprünge:      {folge.anzahl(Art.SPRUNG)}",
        f"  Farbwechsel:  {folge.anzahl(Art.FARBWECHSEL)}  (Farben: {', '.join(folge.farben)})",
    ]
    if bb:
        zeilen.append(f"  Größe:        {bb[2] - bb[0]:.1f} × {bb[3] - bb[1]:.1f} mm  "
                      f"(x {bb[0]:.1f} … {bb[2]:.1f}, y {bb[1]:.1f} … {bb[3]:.1f})")
    if laengen:
        zeilen.append(f"  Stichlänge:   min {min(laengen):.2f} mm, max {max(laengen):.2f} mm")
    return "\n".join(zeilen)


def _ausgeben(design: Design, ordner: Path) -> int:
    folge = design.berechnen()
    ordner.mkdir(parents=True, exist_ok=True)
    basis = ordner / design.name.replace(" ", "_")
    daten = exp_bytes(folge)
    basis.with_suffix(".exp").write_bytes(daten)
    basis.with_suffix(".svg").write_text(svg_text(folge, design.rahmen, design.name), encoding="utf-8")
    print(f"Design '{design.name}' – Rahmen {design.rahmen.name}, Stoff {design.einstellungen.stoff}")
    print(statistik(folge))
    print(f"  EXP:          {basis.with_suffix('.exp')} ({len(daten)} Byte)")
    print(f"  Vorschau:     {basis.with_suffix('.svg')}")
    aussen = design.ausserhalb(folge)
    if aussen:
        print(f"  WARNUNG: {len(aussen)} Einstiche liegen außerhalb des Stickfelds!")
        return 1
    return 0


def beispiel_dict() -> dict:
    """Beispiel mit allen Sticharten aus Phase 1 (passt in den Rahmen 130 × 100)."""
    return {
        "name": "Beispiel Phase 1",
        "rahmen": "artista 130 x 100",
        "stoff": "0 Software Defaults",
        "objekte": [
            {"art": "tatami", "name": "Ring", "farbe": "#2e86c1", "winkel": 45,
             "umriss": {"kreis": {"mitte": [0, 22], "radius": 18}},
             "loecher": [{"kreis": {"mitte": [0, 22], "radius": 8}}]},
            {"art": "satinlinie", "name": "Bogen", "farbe": "#c0392b", "breite": 3.5,
             "punkte": {"bogen": {"mitte": [0, 22], "radius": 22, "von": 200, "bis": 340}}},
            {"art": "satin", "name": "Keil", "farbe": "#c0392b",
             "seite_a": [[-35, -10], [-10, -6], [15, -10]], "seite_b": [[-35, -12], [-10, -14], [15, -12]]},
            {"art": "satinlinie", "name": "Schmale Bahn", "farbe": "#27ae60", "breite": 1.0,
             "punkte": {"bezier": [[-35, -22], [-15, -32], [5, -12], [25, -22]]}},
            {"art": "lauf", "name": "Kontur", "farbe": "#1b2631",
             "punkte": {"rechteck": {"mitte": [0, -18], "breite": 80, "hoehe": 20}}, "geschlossen": True},
            {"art": "dreifach", "name": "Dreifach", "farbe": "#1b2631",
             "punkte": [[-40, -40], [-20, -36], [0, -40], [20, -36], [40, -40]]},
        ],
    }


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(prog="stickprogramm", description="Stickprogramm für die BERNINA artista 185")
    sub = p.add_subparsers(dest="befehl", required=True)
    a = sub.add_parser("erzeugen", help="Design (JSON) berechnen -> EXP + SVG")
    a.add_argument("design")
    a.add_argument("-o", "--ordner", default=".")
    a = sub.add_parser("anzeigen", help="EXP-Datei als SVG darstellen")
    a.add_argument("exp")
    a.add_argument("-o", "--svg")
    a.add_argument("--rahmen", default=None)
    a = sub.add_parser("beispiel", help="Beispieldesign erzeugen")
    a.add_argument("-o", "--ordner", default="ausgabe")
    sub.add_parser("rahmen", help="Rahmen auflisten")
    sub.add_parser("stoffe", help="Stoffe auflisten")
    args = p.parse_args(argv)

    if args.befehl == "erzeugen":
        return _ausgeben(design_laden(args.design), Path(args.ordner))
    if args.befehl == "beispiel":
        ordner = Path(args.ordner)
        ordner.mkdir(parents=True, exist_ok=True)
        d = beispiel_dict()
        (ordner / "beispiel.json").write_text(json.dumps(d, indent=2, ensure_ascii=False), encoding="utf-8")
        return _ausgeben(design_aus_dict(d), ordner)
    if args.befehl == "anzeigen":
        folge = exp_lesen(args.exp)
        r = rahmen(args.rahmen) if args.rahmen else None
        ziel = Path(args.svg) if args.svg else Path(args.exp).with_suffix(".svg")
        ziel.write_text(svg_text(folge, r, Path(args.exp).name), encoding="utf-8")
        print(statistik(folge))
        print(f"  Vorschau:     {ziel}")
        return 0
    if args.befehl == "rahmen":
        for r in RAHMEN.values():
            print(f"  {r.name:<36} {r.breite:6.1f} × {r.hoehe:6.1f} mm  {r.hinweis}")
        return 0
    if args.befehl == "stoffe":
        print(f"  {'Stoff':<22} Satin fest  Tatami  Faktor  Zugausgl.")
        for s in STOFFE.values():
            print(f"  {s.name:<22} {s.satin_abstand_fest:9.2f}  {s.tatami_abstand:6.2f}  "
                  f"{s.auto_spacing_faktor:6.2f}  {s.zugausgleich:9.2f}")
        return 0
    return 2


if __name__ == "__main__":
    sys.exit(main())
