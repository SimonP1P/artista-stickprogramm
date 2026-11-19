"""Design aus einer JSON-Beschreibung laden.

Beispiel (Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben):
{
  "name": "Test", "rahmen": "artista 130 x 100", "stoff": "0 Software Defaults",
  "einstellungen": {"satin": {"zugausgleich": 0.3}},
  "objekte": [
    {"art": "lauf", "punkte": [[-20, 0], [20, 0]], "farbe": "#c00000", "dreifach": true},
    {"art": "satinlinie", "punkte": {"bogen": {"mitte": [0, 0], "radius": 20, "von": 0, "bis": 180}}, "breite": 3},
    {"art": "satin", "seite_a": [[0, 0], [10, 0]], "seite_b": [[0, -4], [10, -4]]},
    {"art": "tatami", "umriss": {"kreis": {"mitte": [0, 0], "radius": 15}}, "loecher": [], "winkel": 45}
  ]
}
Statt einer Punktliste geht überall eine Form: {"kreis": {...}}, {"rechteck": {"mitte", "breite", "hoehe"}},
{"bogen": {"mitte", "radius", "von", "bis"}}, {"bezier": [p0, p1, p2, p3]} oder {"verbinden": [Form, Form, …]}.
"""

from __future__ import annotations

import json
from dataclasses import fields, is_dataclass
from pathlib import Path
from typing import Any

from .design import Design, Lauf, Objekt, Satin, SatinLinie, Tatami
from .einstellungen import fuer_stoff
from .geometrie import Linie, bezier, bogen, kreis, rechteck
from .rahmen import rahmen


def punkte(wert: Any) -> Linie:
    if isinstance(wert, list):
        return [(float(p[0]), float(p[1])) for p in wert]
    if isinstance(wert, dict) and len(wert) == 1:
        art, a = next(iter(wert.items()))
        if art == "kreis":
            return kreis(tuple(a["mitte"]), float(a["radius"]), start_grad=float(a.get("start", 0.0)))
        if art == "rechteck":
            return rechteck(tuple(a["mitte"]), float(a["breite"]), float(a["hoehe"]))
        if art == "bogen":
            return bogen(tuple(a["mitte"]), float(a["radius"]), float(a["von"]), float(a["bis"]))
        if art == "bezier":
            p = [tuple(map(float, q)) for q in a]
            return bezier(p[0], p[1], p[2], p[3])
        if art == "verbinden":
            out: Linie = []
            for teil in a:
                pts = punkte(teil)
                if out and pts and out[-1] == pts[0]:
                    pts = pts[1:]
                out.extend(pts)
            return out
    raise ValueError(f"Unbekannte Punktangabe: {wert!r}")


def _objekt(d: dict) -> Objekt:
    art = d.get("art")
    gemein = {"farbe": d.get("farbe", "#000000"), "name": d.get("name", "")}
    if art in ("lauf", "dreifach"):
        return Lauf(**gemein, punkte=punkte(d["punkte"]), laenge=d.get("laenge"),
                    dreifach=bool(d.get("dreifach", art == "dreifach")), geschlossen=bool(d.get("geschlossen", False)))
    if art == "satin":
        return Satin(**gemein, seite_a=punkte(d["seite_a"]), seite_b=punkte(d["seite_b"]),
                     unterlage=d.get("unterlage", "auto"))
    if art == "satinlinie":
        return SatinLinie(**gemein, punkte=punkte(d["punkte"]), breite=float(d.get("breite", 2.0)),
                          unterlage=d.get("unterlage", "auto"))
    if art == "tatami":
        return Tatami(**gemein, umriss=punkte(d["umriss"]), loecher=[punkte(l) for l in d.get("loecher", [])],
                      winkel=d.get("winkel"), unterlage=bool(d.get("unterlage", True)))
    raise ValueError(f"Unbekannte Objektart '{art}'. Möglich: lauf, dreifach, satin, satinlinie, tatami")


def _einstellungen_anwenden(ziel: Any, werte: dict) -> None:
    namen = {f.name for f in fields(ziel)}
    for k, v in werte.items():
        if k not in namen:
            raise ValueError(f"Unbekannte Einstellung '{k}' in {type(ziel).__name__}")
        aktuell = getattr(ziel, k)
        if is_dataclass(aktuell) and isinstance(v, dict):
            _einstellungen_anwenden(aktuell, v)
        else:
            setattr(ziel, k, v)


def design_aus_dict(d: dict) -> Design:
    e = fuer_stoff(d.get("stoff", "0 Software Defaults"))
    if "einstellungen" in d:
        _einstellungen_anwenden(e, d["einstellungen"])
    return Design(name=d.get("name", "Design"), rahmen=rahmen(d.get("rahmen", "artista 130 x 100")),
                  einstellungen=e, objekte=[_objekt(o) for o in d.get("objekte", [])])


def design_laden(pfad: str | Path) -> Design:
    return design_aus_dict(json.loads(Path(pfad).read_text(encoding="utf-8")))
