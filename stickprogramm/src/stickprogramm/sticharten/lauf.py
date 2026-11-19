"""Laufstich (Geradstich) und Dreifachstich entlang einer Linie."""

from __future__ import annotations

import math
from typing import Sequence

from ..einstellungen import LaufEinstellung
from ..geometrie import (Linie, Punkt, abstand_punkt_strecke, knick_grad, kumuliert, ohne_doppelte,
                         punkt_bei)


def _ecken(pts: Linie, ecke_grad: float) -> list[int]:
    idx = [0]
    for i in range(1, len(pts) - 1):
        if knick_grad(pts[i - 1], pts[i], pts[i + 1]) >= ecke_grad:
            idx.append(i)
    idx.append(len(pts) - 1)
    return idx


def _sehnen_ok(teil: Linie, kum: list[float], s_punkte: list[float], gap: float) -> bool:
    for s0, s1 in zip(s_punkte, s_punkte[1:]):
        p0, p1 = punkt_bei(teil, s0, kum), punkt_bei(teil, s1, kum)
        for p, s in zip(teil, kum):
            if s0 < s < s1 and abstand_punkt_strecke(p, p0, p1) > gap:
                return False
    return True


def laufstich(linie: Sequence[Punkt], e: LaufEinstellung | None = None, laenge: float | None = None) -> Linie:
    """Einstichpunkte eines Laufstichs entlang der Linie (erster und letzter Punkt inklusive).

    Wie „feste Länge“ im Original: An Ecken (Knick ≥ e.ecke_grad) wird eingestochen, dazwischen wird
    die Strecke gleichmäßig in Stiche ≤ laenge geteilt. Bei e.variabel werden Stiche an Kurven so lange
    verkürzt, bis die Sehne höchstens e.sehnenabstand von der Linie abweicht (nicht unter e.min_laenge).
    """
    e = e or LaufEinstellung()
    L = laenge or e.laenge
    pts = ohne_doppelte(linie)
    if len(pts) < 2:
        return pts
    out = [pts[0]]
    ecken = _ecken(pts, e.ecke_grad)
    for a, b in zip(ecken, ecken[1:]):
        teil = pts[a:b + 1]
        kum = kumuliert(teil)
        gesamt = kum[-1]
        if gesamt < 1e-9:
            continue
        n = max(1, math.ceil(gesamt / L - 1e-9))
        while True:
            s_punkte = [gesamt * k / n for k in range(n + 1)]
            if not e.variabel or gesamt / (n + 1) < e.min_laenge or _sehnen_ok(teil, kum, s_punkte, e.sehnenabstand):
                break
            n += 1
        out.extend(punkt_bei(teil, s, kum) for s in s_punkte[1:])
    return out


def dreifach(punkte: Sequence[Punkt]) -> Linie:
    """Dreifachstich: jeder Stich vor, zurück, vor (Stichpunkte eines Laufstichs als Eingabe)."""
    pts = list(punkte)
    if len(pts) < 2:
        return pts
    out = [pts[0]]
    for a, b in zip(pts, pts[1:]):
        out.extend([b, a, b])
    return out
