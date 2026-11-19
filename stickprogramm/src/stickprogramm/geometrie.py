"""Geometrie-Hilfen. Punkte sind Tupel (x, y) in mm, +y nach oben."""

from __future__ import annotations

import math
from typing import Iterable, Sequence

Punkt = tuple[float, float]
Linie = list[Punkt]


def abstand(a: Punkt, b: Punkt) -> float:
    return math.hypot(b[0] - a[0], b[1] - a[1])


def zwischen(a: Punkt, b: Punkt, t: float) -> Punkt:
    return (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)


def richtung(a: Punkt, b: Punkt) -> Punkt:
    """Einheitsvektor von a nach b (0,0 bei gleichen Punkten)."""
    d = abstand(a, b)
    if d == 0:
        return (0.0, 0.0)
    return ((b[0] - a[0]) / d, (b[1] - a[1]) / d)


def verschieben(p: Punkt, r: Punkt, d: float) -> Punkt:
    return (p[0] + r[0] * d, p[1] + r[1] * d)


def drehen(p: Punkt, winkel_grad: float, um: Punkt = (0.0, 0.0)) -> Punkt:
    w = math.radians(winkel_grad)
    c, s = math.cos(w), math.sin(w)
    x, y = p[0] - um[0], p[1] - um[1]
    return (um[0] + x * c - y * s, um[1] + x * s + y * c)


def laenge(linie: Sequence[Punkt]) -> float:
    return sum(abstand(a, b) for a, b in zip(linie, linie[1:]))


def kumuliert(linie: Sequence[Punkt]) -> list[float]:
    """Bogenlänge bis zu jedem Punkt."""
    s = [0.0]
    for a, b in zip(linie, linie[1:]):
        s.append(s[-1] + abstand(a, b))
    return s


def punkt_bei(linie: Sequence[Punkt], s: float, kum: Sequence[float] | None = None) -> Punkt:
    """Punkt bei Bogenlänge s (auf [0, Länge] begrenzt)."""
    if kum is None:
        kum = kumuliert(linie)
    if s <= 0:
        return linie[0]
    if s >= kum[-1]:
        return linie[-1]
    lo, hi = 0, len(kum) - 1
    while hi - lo > 1:
        mid = (lo + hi) // 2
        if kum[mid] <= s:
            lo = mid
        else:
            hi = mid
    seg = kum[hi] - kum[lo]
    t = 0.0 if seg == 0 else (s - kum[lo]) / seg
    return zwischen(linie[lo], linie[hi], t)


def teilstueck(linie: Sequence[Punkt], s0: float, s1: float, kum: Sequence[float] | None = None) -> Linie:
    """Teil der Linie zwischen Bogenlängen s0 < s1, inklusive der Eckpunkte dazwischen."""
    if kum is None:
        kum = kumuliert(linie)
    out = [punkt_bei(linie, s0, kum)]
    for p, s in zip(linie, kum):
        if s0 < s < s1:
            out.append(p)
    out.append(punkt_bei(linie, s1, kum))
    return out


def abstand_punkt_strecke(p: Punkt, a: Punkt, b: Punkt) -> float:
    dx, dy = b[0] - a[0], b[1] - a[1]
    l2 = dx * dx + dy * dy
    if l2 == 0:
        return abstand(p, a)
    t = max(0.0, min(1.0, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / l2))
    return abstand(p, (a[0] + t * dx, a[1] + t * dy))


def knick_grad(a: Punkt, b: Punkt, c: Punkt) -> float:
    """Richtungsänderung in b (0° = geradeaus, 180° = Umkehr)."""
    r1, r2 = richtung(a, b), richtung(b, c)
    if r1 == (0.0, 0.0) or r2 == (0.0, 0.0):
        return 0.0
    cosw = max(-1.0, min(1.0, r1[0] * r2[0] + r1[1] * r2[1]))
    return math.degrees(math.acos(cosw))


def ohne_doppelte(linie: Iterable[Punkt], eps: float = 1e-9) -> Linie:
    out: Linie = []
    for p in linie:
        if not out or abstand(out[-1], p) > eps:
            out.append((float(p[0]), float(p[1])))
    return out


def versatz_linie(linie: Sequence[Punkt], d: float, max_gehrung: float = 4.0) -> Linie:
    """Offene Linie um d parallel verschieben (d > 0 = nach links in Laufrichtung), Gehrungsecken.

    Ergibt genau so viele Punkte wie die Eingabe (Punktpaare bleiben einander zugeordnet).
    """
    n = len(linie)
    if n < 2:
        return list(linie)
    normalen = []
    for a, b in zip(linie, linie[1:]):
        r = richtung(a, b)
        normalen.append((-r[1], r[0]))
    out = []
    for i, p in enumerate(linie):
        if i == 0:
            nrm, f = normalen[0], 1.0
        elif i == n - 1:
            nrm, f = normalen[-1], 1.0
        else:
            n1, n2 = normalen[i - 1], normalen[i]
            m = (n1[0] + n2[0], n1[1] + n2[1])
            ml = math.hypot(*m)
            if ml < 1e-9:
                nrm, f = n1, 1.0
            else:
                nrm = (m[0] / ml, m[1] / ml)
                cosh = nrm[0] * n1[0] + nrm[1] * n1[1]
                f = min(1.0 / max(cosh, 1e-9), max_gehrung)
        out.append((p[0] + nrm[0] * d * f, p[1] + nrm[1] * d * f))
    return out


# ---------------------------------------------------------------- Polygone
def flaeche_vorzeichen(poly: Sequence[Punkt]) -> float:
    """Fläche mit Vorzeichen (> 0 = gegen den Uhrzeigersinn)."""
    s = 0.0
    for i in range(len(poly)):
        x1, y1 = poly[i]
        x2, y2 = poly[(i + 1) % len(poly)]
        s += x1 * y2 - x2 * y1
    return s / 2


def geschlossen_ohne_endpunkt(poly: Sequence[Punkt]) -> Linie:
    p = ohne_doppelte(poly)
    if len(p) > 1 and abstand(p[0], p[-1]) < 1e-9:
        p = p[:-1]
    return p


def gegen_uhrzeiger(poly: Sequence[Punkt]) -> Linie:
    p = geschlossen_ohne_endpunkt(poly)
    return p if flaeche_vorzeichen(p) >= 0 else p[::-1]


def im_polygon(p: Punkt, ringe: Sequence[Sequence[Punkt]]) -> bool:
    """Gerade-Ungerade-Regel über alle Ringe (Umriss + Löcher)."""
    x, y = p
    innen = False
    for ring in ringe:
        n = len(ring)
        for i in range(n):
            x1, y1 = ring[i]
            x2, y2 = ring[(i + 1) % n]
            if (y1 > y) != (y2 > y):
                xs = x1 + (y - y1) * (x2 - x1) / (y2 - y1)
                if xs > x:
                    innen = not innen
    return innen


def rand_abstand(p: Punkt, ringe: Sequence[Sequence[Punkt]]) -> float:
    """Kleinster Abstand von p zu einer Kante der Ringe."""
    best = math.inf
    for ring in ringe:
        n = len(ring)
        for i in range(n):
            best = min(best, abstand_punkt_strecke(p, ring[i], ring[(i + 1) % n]))
    return best


def versatz_ring(ring: Sequence[Punkt], d: float, max_gehrung: float = 3.0) -> Linie:
    """Geschlossenen Ring um d nach innen versetzen (d > 0), Umlaufsinn beliebig.

    Einfaches Gehrungsverfahren; für kleine Abstände (Unterlagenrand, Randlauf) gedacht.
    """
    p = geschlossen_ohne_endpunkt(ring)
    vorzeichen = 1.0 if flaeche_vorzeichen(p) >= 0 else -1.0
    n = len(p)
    normalen = []
    for i in range(n):
        r = richtung(p[i], p[(i + 1) % n])
        normalen.append((-r[1] * vorzeichen, r[0] * vorzeichen))  # zeigt nach innen
    out = []
    for i in range(n):
        n1, n2 = normalen[i - 1], normalen[i]
        m = (n1[0] + n2[0], n1[1] + n2[1])
        ml = math.hypot(*m)
        if ml < 1e-9:
            out.append((p[i][0] + n2[0] * d, p[i][1] + n2[1] * d))
            continue
        nrm = (m[0] / ml, m[1] / ml)
        cosh = nrm[0] * n1[0] + nrm[1] * n1[1]
        f = min(1.0 / max(cosh, 1e-9), max_gehrung)
        out.append((p[i][0] + nrm[0] * d * f, p[i][1] + nrm[1] * d * f))
    return out


def begrenzung(punkte: Iterable[Punkt]) -> tuple[float, float, float, float]:
    xs, ys = [], []
    for x, y in punkte:
        xs.append(x)
        ys.append(y)
    return (min(xs), min(ys), max(xs), max(ys))


# ---------------------------------------------------------------- Grundformen
def kreis(mitte: Punkt, radius: float, sehnenfehler: float = 0.02, start_grad: float = 0.0,
          gegen_uhrzeiger_sinn: bool = True) -> Linie:
    """Geschlossener Kreis als Ring (ohne doppelten Endpunkt)."""
    n = max(12, math.ceil(math.pi / math.acos(max(-1.0, 1 - sehnenfehler / max(radius, 1e-9)))))
    vz = 1 if gegen_uhrzeiger_sinn else -1
    return [(mitte[0] + radius * math.cos(math.radians(start_grad) + vz * 2 * math.pi * i / n),
             mitte[1] + radius * math.sin(math.radians(start_grad) + vz * 2 * math.pi * i / n)) for i in range(n)]


def bogen(mitte: Punkt, radius: float, von_grad: float, bis_grad: float, sehnenfehler: float = 0.02) -> Linie:
    """Offener Kreisbogen von von_grad nach bis_grad (mathematisch positiv, wenn bis > von)."""
    spanne = math.radians(bis_grad - von_grad)
    schritt = 2 * math.acos(max(-1.0, 1 - sehnenfehler / max(radius, 1e-9)))
    n = max(2, math.ceil(abs(spanne) / schritt))
    return [(mitte[0] + radius * math.cos(math.radians(von_grad) + spanne * i / n),
             mitte[1] + radius * math.sin(math.radians(von_grad) + spanne * i / n)) for i in range(n + 1)]


def rechteck(mitte: Punkt, breite: float, hoehe: float) -> Linie:
    x, y = mitte
    b, h = breite / 2, hoehe / 2
    return [(x - b, y - h), (x + b, y - h), (x + b, y + h), (x - b, y + h)]


def bezier(p0: Punkt, p1: Punkt, p2: Punkt, p3: Punkt, n: int = 32) -> Linie:
    """Kubische Bézierkurve als Linie mit n+1 Punkten."""
    out = []
    for i in range(n + 1):
        t = i / n
        a, b, c, d = (1 - t) ** 3, 3 * t * (1 - t) ** 2, 3 * t * t * (1 - t), t ** 3
        out.append((a * p0[0] + b * p1[0] + c * p2[0] + d * p3[0],
                    a * p0[1] + b * p1[1] + c * p2[1] + d * p3[1]))
    return out
