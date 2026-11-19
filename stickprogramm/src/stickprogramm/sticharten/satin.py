"""Satinsäule zwischen zwei Seitenlinien (wie „Satin Column A“ im Original).

Die Säule wird durch zwei Seiten A und B beschrieben, beide in Stickrichtung. Punkt i von A gehört zu
Punkt i von B (Stichrichtungs-Linie). Haben A und B verschieden viele Punkte, werden sie über die
relative Bogenlänge einander zugeordnet.
"""

from __future__ import annotations

from typing import Sequence

from ..einstellungen import SatinEinstellung
from ..geometrie import (Linie, Punkt, abstand, kumuliert, ohne_doppelte, punkt_bei, richtung,
                         verschieben, versatz_linie, zwischen)


def paare_bilden(seite_a: Sequence[Punkt], seite_b: Sequence[Punkt]) -> tuple[Linie, Linie]:
    a, b = ohne_doppelte(seite_a), ohne_doppelte(seite_b)
    if len(a) < 2 or len(b) < 2:
        raise ValueError("Satinsäule: jede Seite braucht mindestens 2 Punkte")
    if len(a) == len(b):
        return a, b
    ka, kb = kumuliert(a), kumuliert(b)
    ta = [s / ka[-1] for s in ka]
    tb = [s / kb[-1] for s in kb]
    ts = sorted(set(round(t, 9) for t in ta + tb))
    return ([punkt_bei(a, t * ka[-1], ka) for t in ts], [punkt_bei(b, t * kb[-1], kb) for t in ts])


def seiten_aus_mittellinie(mitte: Sequence[Punkt], breite: float) -> tuple[Linie, Linie]:
    """Seiten einer Satinbahn konstanter Breite um eine Mittellinie (A links, B rechts)."""
    m = ohne_doppelte(mitte)
    return versatz_linie(m, breite / 2), versatz_linie(m, -breite / 2)


def mittellinie(a: Linie, b: Linie) -> Linie:
    return [zwischen(p, q, 0.5) for p, q in zip(a, b)]


def satin(seite_a: Sequence[Punkt], seite_b: Sequence[Punkt], e: SatinEinstellung | None = None) -> Linie:
    """Einstichpunkte der Satinsäule, abwechselnd auf Seite A und B, beginnend bei A[0].

    Abstand je Stich aus Auto Spacing (Breite an der Stelle) bzw. fest; Fortschritt gemessen als
    Mittel der Wege auf beiden Seiten (eigene Festlegung, mittel). Zugausgleich verlängert jeden
    Stich an beiden Enden.
    """
    e = e or SatinEinstellung()
    a, b = paare_bilden(seite_a, seite_b)
    zg = e.zugausgleich

    def einstich(pa: Punkt, pb: Punkt, seite_a_: bool) -> Punkt:
        if seite_a_:
            return verschieben(pa, richtung(pb, pa), zg)
        return verschieben(pb, richtung(pa, pb), zg)

    out: Linie = [einstich(a[0], b[0], True)]
    seite = False                      # nächster Einstich auf B
    gelaufen = 0.0                     # Weg seit dem letzten Einstich (über Segmentgrenzen hinweg)
    for i in range(len(a) - 1):
        seg = (abstand(a[i], a[i + 1]) + abstand(b[i], b[i + 1])) / 2
        if seg < 1e-9:
            continue
        t = 0.0
        while True:
            pa, pb = zwischen(a[i], a[i + 1], t), zwischen(b[i], b[i + 1], t)
            noetig = max(0.0, e.abstand(abstand(pa, pb)) - gelaufen)
            frei = (1.0 - t) * seg
            if noetig > frei + 1e-9:
                gelaufen += frei
                break
            t += noetig / seg
            gelaufen = 0.0
            pa, pb = zwischen(a[i], a[i + 1], t), zwischen(b[i], b[i + 1], t)
            out.append(einstich(pa, pb, seite))
            seite = not seite
    # Ende sauber abschließen: letzter Stich auf die Endkante
    ende = einstich(a[-1], b[-1], seite)
    if abstand(out[-1], ende) > 1e-6:
        out.append(ende)
    return out
