"""Unterlagen (Underlay) für Satinsäulen und Flächen.

Werte aus der Analyse (Analyse/code/10 §D, mittel). Jede Satin-Unterlage beginnt und endet am Anfang
der Säule, damit der Deckstich danach an der Startkante beginnt (eigene Festlegung).
"""

from __future__ import annotations

from typing import Sequence

from ..einstellungen import LaufEinstellung, TatamiEinstellung, UnterlageEinstellung
from ..geometrie import Linie, Punkt, abstand, richtung, verschieben, versatz_ring, zwischen
from .lauf import laufstich
from .satin import mittellinie, paare_bilden
from .tatami import tatami

SATIN_UNTERLAGEN = ("keine", "mittellauf", "edge_walk", "zickzack", "doppelzickzack")


def auto_satin_unterlage(mittlere_breite: float) -> list[str]:
    """Unterlage nach Säulenbreite (eigene Festlegung, in Phase 3 an Probestickereien prüfen)."""
    if mittlere_breite < 1.5:
        return []
    if mittlere_breite < 3.0:
        return ["mittellauf"]
    if mittlere_breite < 6.0:
        return ["edge_walk"]
    return ["edge_walk", "zickzack"]


def _eingerueckt(a: Linie, b: Linie, rand: float) -> tuple[Linie, Linie]:
    """Seiten um rand nach innen rücken; zu schmale Stellen fallen auf die Mitte zusammen."""
    ai, bi = [], []
    for p, q in zip(a, b):
        w = abstand(p, q)
        if w <= 2 * rand + 0.3:
            m = zwischen(p, q, 0.5)
            ai.append(m)
            bi.append(m)
        else:
            ai.append(verschieben(p, richtung(p, q), rand))
            bi.append(verschieben(q, richtung(q, p), rand))
    return ai, bi


def _lauf(linie: Sequence[Punkt], laenge: float) -> Linie:
    return laufstich(linie, LaufEinstellung(laenge=laenge, ecke_grad=30.0))


def _zickzack(a: Linie, b: Linie, laenge: float, phase: float = 0.0) -> Linie:
    """Zickzack zwischen a und b: Einstich abwechselnd auf a/b alle laenge/2 Weg (Mittel beider Seiten)."""
    halb = laenge / 2
    out = [a[0]]
    seite_b = True
    gelaufen = halb - phase * halb
    for i in range(len(a) - 1):
        seg = (abstand(a[i], a[i + 1]) + abstand(b[i], b[i + 1])) / 2
        if seg < 1e-9:
            continue
        t = 0.0
        while True:
            noetig = max(0.0, halb - gelaufen)
            if noetig > (1 - t) * seg + 1e-9:
                gelaufen += (1 - t) * seg
                break
            t += noetig / seg
            gelaufen = 0.0
            out.append(zwischen(b[i], b[i + 1], t) if seite_b else zwischen(a[i], a[i + 1], t))
            seite_b = not seite_b
    out.append(b[-1] if seite_b else a[-1])
    return out


def satin_unterlage(seite_a: Sequence[Punkt], seite_b: Sequence[Punkt], arten: Sequence[str],
                    e: UnterlageEinstellung | None = None) -> Linie:
    """Unterlage-Einstiche für eine Satinsäule; Start und Ende am Säulenanfang (Mitte bzw. Seite A)."""
    e = e or UnterlageEinstellung()
    a, b = paare_bilden(seite_a, seite_b)
    ai, bi = _eingerueckt(a, b, e.rand)
    mitte = mittellinie(a, b)
    out: Linie = []

    def anhaengen(pts: Linie) -> None:
        if out and pts and abstand(out[-1], pts[0]) < 1e-9:
            pts = pts[1:]
        out.extend(pts)

    for art in arten:
        if art == "keine":
            continue
        if art == "mittellauf":
            hin = _lauf(mitte, e.mittellauf_laenge)
            anhaengen(hin)
            anhaengen(hin[::-1])
        elif art == "edge_walk":
            anhaengen(_lauf(ai, e.edge_walk_laenge))
            anhaengen(_lauf(bi[::-1], e.edge_walk_laenge))
            anhaengen([ai[0]])
        elif art in ("zickzack", "doppelzickzack"):
            laenge = e.zickzack_laenge if art == "zickzack" else e.doppelzickzack_laenge
            anhaengen(_zickzack(ai, bi, laenge))
            if art == "zickzack":
                anhaengen(_lauf(mitte[::-1], e.rueck_lauf_laenge))    # „+ Lauf 3,0 mm“ zurück zum Anfang
            else:
                anhaengen(_zickzack(bi[::-1], ai[::-1], laenge, phase=0.5))
        else:
            raise ValueError(f"Unbekannte Satin-Unterlage '{art}'. Möglich: {', '.join(SATIN_UNTERLAGEN)}")
    return out


def flaechen_unterlage(umriss: Sequence[Punkt], loecher: Sequence[Sequence[Punkt]], deck_winkel: float,
                       e: UnterlageEinstellung | None = None, start: Punkt | None = None) -> list[Linie]:
    """Tatami-Unterlage: Randlauf (eingerückt) + Tatami 3,0/4,0 mm quer zur Deckstichrichtung."""
    e = e or UnterlageEinstellung()
    rand_umriss = versatz_ring(umriss, e.rand)
    rand_loecher = [versatz_ring(l, -e.rand) for l in loecher]
    wege: list[Linie] = []
    ring = rand_umriss + [rand_umriss[0]]
    if start is not None:
        i = min(range(len(rand_umriss)), key=lambda j: abstand(rand_umriss[j], start))
        ring = rand_umriss[i:] + rand_umriss[:i + 1]
    wege.append(_lauf(ring, e.tatami_randlauf))
    te = TatamiEinstellung(abstand=e.tatami_abstand, laenge=e.tatami_laenge, min_rand=0.4, versatz=0.0,
                           winkel=deck_winkel + e.tatami_winkel_versatz, zugausgleich=e.zugausgleich)
    wege.extend(tatami(rand_umriss, rand_loecher, te, start=wege[-1][-1]))
    return wege
