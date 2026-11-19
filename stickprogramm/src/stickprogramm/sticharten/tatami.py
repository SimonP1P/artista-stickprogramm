"""Tatami-/Step-Füllung einer Fläche (Umriss mit optionalen Löchern).

Verfahren:
1. Fläche so drehen, dass die Stichreihen waagerecht liegen (Stichwinkel).
2. Reihen im Abstand e.abstand mit allen Kanten schneiden -> Abschnitte je Reihe (Gerade-Ungerade-Regel).
3. Abschnitte zu zusammenhängenden, „einfachen“ Bereichen verbinden (je Reihe genau ein Abschnitt),
   damit jeder Bereich hin und her in einem Zug gestickt werden kann.
4. Einstiche je Reihe auf einem festen Raster der Stichlänge; das Raster verschiebt sich je Reihe um
   e.versatz × Stichlänge (Tatami-Muster). Am Rand: Abschnittsende, Rasterpunkte näher als e.min_rand
   am Ende entfallen. Zugausgleich verlängert jede Reihe an beiden Enden.
5. Bereiche in der Reihenfolge des geringsten Weges; zwischen Bereichen entsteht ein Sprung
   (Laufwege durch die Fläche: Phase 3).
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field
from typing import Sequence

from ..einstellungen import TatamiEinstellung
from ..geometrie import Linie, Punkt, abstand, drehen, gegen_uhrzeiger, geschlossen_ohne_endpunkt


@dataclass
class _Bereich:
    reihen: list[tuple[int, float, float]] = field(default_factory=list)  # (Reihennummer, x0, x1)


def _schnitte(ringe: Sequence[Linie], y: float) -> list[float]:
    xs = []
    for ring in ringe:
        n = len(ring)
        for i in range(n):
            x1, y1 = ring[i]
            x2, y2 = ring[(i + 1) % n]
            if (y1 > y) != (y2 > y):
                xs.append(x1 + (y - y1) * (x2 - x1) / (y2 - y1))
    xs.sort()
    return xs


def _bereiche(reihen: list[tuple[int, list[tuple[float, float]]]]) -> list[_Bereich]:
    fertig: list[_Bereich] = []
    offen: list[_Bereich] = []
    for k, abschnitte in reihen:
        ueberlapp = []  # je Abschnitt: Liste offener Bereiche, die er berührt
        for x0, x1 in abschnitte:
            ueberlapp.append([b for b in offen
                              if b.reihen[-1][0] == k - 1 and b.reihen[-1][1] < x1 and x0 < b.reihen[-1][2]])
        zaehler: dict[int, int] = {}
        for liste in ueberlapp:
            for b in liste:
                zaehler[id(b)] = zaehler.get(id(b), 0) + 1
        neu_offen = []
        for (x0, x1), liste in zip(abschnitte, ueberlapp):
            if len(liste) == 1 and zaehler[id(liste[0])] == 1:
                b = liste[0]
            else:
                b = _Bereich()
            b.reihen.append((k, x0, x1))
            neu_offen.append(b)
        for b in offen:
            if all(b is not n for n in neu_offen):
                fertig.append(b)
        offen = neu_offen
    return fertig + offen


def _reihe(k: int, x0: float, x1: float, y: float, e: TatamiEinstellung, vorwaerts: bool) -> Linie:
    L = e.laenge
    phase = ((k * e.versatz) % 1.0) * L
    m0 = math.ceil((x0 + e.min_rand - phase) / L)
    raster = []
    m = m0
    while True:
        x = phase + m * L
        if x > x1 - e.min_rand:
            break
        raster.append(x)
        m += 1
    xs = [x0] + raster + [x1]
    if not vorwaerts:
        xs.reverse()
    return [(x, y) for x in xs]


def _bereich_sticken(b: _Bereich, y0: float, e: TatamiEinstellung, von_oben: bool, links_start: bool) -> Linie:
    reihen = b.reihen[::-1] if von_oben else b.reihen
    out: Linie = []
    vorwaerts = links_start
    for k, x0, x1 in reihen:
        y = y0 + k * e.abstand
        pts = _reihe(k, x0, x1, y, e, vorwaerts)
        if out:
            # Übergang zur nächsten Reihe: ggf. in Stiche ≤ Stichlänge teilen
            a, z = out[-1], pts[0]
            n = math.ceil(abstand(a, z) / e.laenge - 1e-9)
            for j in range(1, n):
                out.append((a[0] + (z[0] - a[0]) * j / n, a[1] + (z[1] - a[1]) * j / n))
        out.extend(pts)
        vorwaerts = not vorwaerts
    return out


def tatami(umriss: Sequence[Punkt], loecher: Sequence[Sequence[Punkt]] = (), e: TatamiEinstellung | None = None,
           winkel: float | None = None, start: Punkt | None = None) -> list[Linie]:
    """Füllstiche als Liste von Wegen (je zusammenhängender Bereich ein Weg, in Stickreihenfolge)."""
    e = e or TatamiEinstellung()
    w = e.winkel if winkel is None else winkel
    ringe = [gegen_uhrzeiger(umriss)] + [geschlossen_ohne_endpunkt(l) for l in loecher]
    gedreht = [[drehen(p, -w) for p in r] for r in ringe]
    ys = [p[1] for p in gedreht[0]]
    y_min, y_max = min(ys), max(ys)
    y0 = y_min + e.abstand / 2
    reihen = []
    k = 0
    while y0 + k * e.abstand < y_max:
        y = y0 + k * e.abstand
        xs = _schnitte(gedreht, y)
        abschnitte = []
        for i in range(0, len(xs) - 1, 2):
            x0, x1 = xs[i] - e.zugausgleich, xs[i + 1] + e.zugausgleich
            if x1 - x0 >= 0.3:
                abschnitte.append((x0, x1))
        if abschnitte:
            reihen.append((k, abschnitte))
        k += 1
    bereiche = _bereiche(reihen)

    pos = drehen(start, -w) if start is not None else None
    wege: list[Linie] = []
    while bereiche:
        best = None
        for b in bereiche:
            for von_oben in (False, True):
                kz, x0, x1 = b.reihen[-1] if von_oben else b.reihen[0]
                y = y0 + kz * e.abstand
                for links in (True, False):
                    p = (x0, y) if links else (x1, y)
                    d = 0.0 if pos is None else abstand(pos, p)
                    if best is None or d < best[0]:
                        best = (d, b, von_oben, links)
        _, b, von_oben, links = best
        bereiche.remove(b)
        weg = _bereich_sticken(b, y0, e, von_oben, links)
        pos = weg[-1]
        wege.append([drehen(p, w) for p in weg])
    return wege
