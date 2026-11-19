"""Stickrahmen der artista (Stickfeld in mm, Ursprung = Rahmenmitte).

Quelle: RES/hooplibr.dsc -> Analyse/daten/hoops_table.md, Form des Ovals Analyse/code/08_rahmen_dsc.md (hoch).
Welche Rahmen an der artista 185 tatsächlich passen, ist noch nicht am Gerät geprüft.
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field

from .geometrie import Linie, Punkt, drehen, im_polygon, rechteck


def _bogen_durch_drei(p1: Punkt, p2: Punkt, p3: Punkt, schritte: int = 24) -> Linie:
    """Kreisbogen von p1 über p2 nach p3 (ohne p1, mit p3)."""
    ax, ay = p1
    bx, by = p2
    cx, cy = p3
    d = 2 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by))
    ux = ((ax * ax + ay * ay) * (by - cy) + (bx * bx + by * by) * (cy - ay) + (cx * cx + cy * cy) * (ay - by)) / d
    uy = ((ax * ax + ay * ay) * (cx - bx) + (bx * bx + by * by) * (ax - cx) + (cx * cx + cy * cy) * (bx - ax)) / d
    r = math.hypot(ax - ux, ay - uy)
    w1 = math.atan2(ay - uy, ax - ux)
    w2 = math.atan2(by - uy, bx - ux)
    w3 = math.atan2(cy - uy, cx - ux)
    # Drehsinn so wählen, dass p2 auf dem Bogen liegt
    def norm(w: float) -> float:
        return w % (2 * math.pi)
    ccw_span = norm(w3 - w1)
    if norm(w2 - w1) <= ccw_span:
        span = ccw_span
    else:
        span = ccw_span - 2 * math.pi
    return [(ux + r * math.cos(w1 + span * i / schritte), uy + r * math.sin(w1 + span * i / schritte))
            for i in range(1, schritte + 1)]


def _oval_145x255() -> Linie:
    oben = _bogen_durch_drei((65.0, 90.0), (0.0, 127.5), (-65.0, 90.0))
    unten = _bogen_durch_drei((-65.0, -90.0), (0.0, -127.5), (65.0, -90.0))
    return [(72.5, -60.0), (72.5, 60.0), (65.0, 90.0)] + oben + [(-72.5, 60.0), (-72.5, -60.0), (-65.0, -90.0)] + unten


@dataclass
class Rahmen:
    name: str
    breite: float                 # Stickfeld x (mm)
    hoehe: float                  # Stickfeld y (mm)
    form: Linie = field(default_factory=list)   # Umriss des Stickfelds; leer = Rechteck
    hinweis: str = ""

    def __post_init__(self) -> None:
        if not self.form:
            self.form = rechteck((0.0, 0.0), self.breite, self.hoehe)

    def enthaelt(self, p: Punkt, toleranz: float = 1e-6) -> bool:
        if abs(p[0]) > self.breite / 2 + toleranz or abs(p[1]) > self.hoehe / 2 + toleranz:
            return False
        return im_polygon(p, [self.form]) or any(
            im_polygon((p[0] + dx, p[1] + dy), [self.form]) for dx, dy in ((toleranz, 0), (-toleranz, 0), (0, toleranz), (0, -toleranz)))

    def gedreht(self) -> Rahmen:
        return Rahmen(self.name + " Rotated", self.hoehe, self.breite, [drehen(p, 90.0) for p in self.form], self.hinweis)


_GRUND = [
    Rahmen("artista 255 x 145 Oval", 145.0, 255.0, _oval_145x255(), "großer Ovalrahmen"),
    Rahmen("artista 200 x 155", 155.0, 200.0),
    Rahmen("artista 130 x 100", 100.0, 130.0, hinweis="mittlerer Rahmen"),
    Rahmen("artista 50 x 72 Free Arm", 72.0, 50.0, hinweis="eine Seite gerundet – hier als Rechteck genähert"),
    Rahmen("artista 50 x 72 Normal", 72.0, 50.0, hinweis="eine Seite gerundet – hier als Rechteck genähert"),
    Rahmen("artista 40 x 60", 60.0, 40.0),
    Rahmen("artista 400 x 150 MEGA", 150.0, 400.0, hinweis="Mega-Rahmen, mehrere Positionen"),
]

RAHMEN: dict[str, Rahmen] = {}
for _r in _GRUND:
    RAHMEN[_r.name] = _r
    if _r.breite != _r.hoehe:
        _g = _r.gedreht()
        RAHMEN[_g.name] = _g


def rahmen(name: str) -> Rahmen:
    if name not in RAHMEN:
        raise ValueError(f"Unbekannter Rahmen '{name}'. Bekannt: {', '.join(RAHMEN)}")
    return RAHMEN[name]
