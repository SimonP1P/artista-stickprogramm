"""Regeln des Maschinenformats „Bernina“ und Verbinden der Wege zu einer Stichfolge."""

from __future__ import annotations

import math
from typing import Sequence

from .einheiten import AUTO_JUMP_TEILLAENGE_MM, MAX_STICH_MM, MIN_STICH_MM
from .einstellungen import VerbinderEinstellung
from .geometrie import Linie, Punkt, abstand, kumuliert, punkt_bei
from .stiche import Art, Stich, StichFolge


def vernaehen_anfang(weg: Linie, e: VerbinderEinstellung) -> Linie:
    """Anfangs-Vernähstiche: ein kurzes Stück entlang des Weges vor und zurück (eigene Festlegung)."""
    if len(weg) < 2 or e.vernaeh_stiche <= 0:
        return weg
    kum = kumuliert(weg)
    l = min(e.vernaeh_stichlaenge, kum[-1])
    if l < MIN_STICH_MM:
        return weg
    q = punkt_bei(weg, l, kum)
    extra = []
    for i in range(e.vernaeh_stiche - 1):
        extra.append(q if i % 2 == 0 else weg[0])
    if extra and extra[-1] == q:
        extra.append(weg[0])
    return [weg[0]] + extra + weg[1:]


def vernaehen_ende(weg: Linie, e: VerbinderEinstellung) -> Linie:
    umgekehrt = vernaehen_anfang(weg[::-1], e)
    return umgekehrt[::-1]


def wege_verbinden(teile: Sequence[tuple[str, list[Linie]]], e: VerbinderEinstellung,
                   start: Punkt = (0.0, 0.0)) -> StichFolge:
    """Wege (je Objekt: Farbe + Wege) zu einer Stichfolge verbinden.

    Zwischen zwei Wegen: Sprung. Vernäht wird am Anfang und Ende, bei Farbwechsel und wenn der
    Sprung länger als e.vernaehen_ab ist (AMT 0x272a: 3,0 mm).
    """
    alle: list[tuple[str, Linie]] = [(farbe, w) for farbe, wege in teile for w in wege if len(w) >= 1]
    folge = StichFolge()
    if not alle:
        return folge
    folge.farben.append(alle[0][0])
    pos = start
    for i, (farbe, weg) in enumerate(alle):
        vorher = alle[i - 1] if i > 0 else None
        nachher = alle[i + 1] if i + 1 < len(alle) else None
        anfang_vernaehen = (vorher is None or vorher[0] != farbe
                            or abstand(vorher[1][-1], weg[0]) > e.vernaehen_ab)
        ende_vernaehen = (nachher is None or nachher[0] != farbe
                          or abstand(weg[-1], nachher[1][0]) > e.vernaehen_ab)
        if anfang_vernaehen:
            weg = vernaehen_anfang(weg, e)
        if ende_vernaehen:
            weg = vernaehen_ende(weg, e)
        if vorher is not None and vorher[0] != farbe:
            folge.farbwechsel(farbe)
        if abstand(pos, weg[0]) > 1e-9:
            folge.sprung(weg[0])
        for p in weg:
            folge.stich(p)
        pos = weg[-1]
    return folge


def kurze_stiche_entfernen(folge: StichFolge, min_mm: float = MIN_STICH_MM) -> StichFolge:
    """Stiche < 0,3 mm entfernen (DEFAULTS.DFL). Der letzte Stich vor Sprung/Farbwechsel bleibt."""
    neu = StichFolge(farben=list(folge.farben))
    st = folge.stiche
    letzter_einstich: Punkt | None = None
    for i, s in enumerate(st):
        if s.art != Art.STICH:
            neu.stiche.append(s)
            letzter_einstich = None          # erster Einstich nach Sprung/Farbwechsel bleibt immer
            continue
        naechster = st[i + 1] if i + 1 < len(st) else None
        blockende = naechster is None or naechster.art != Art.STICH
        if letzter_einstich is not None and abstand(letzter_einstich, s.p) < min_mm:
            if not blockende:
                continue
            # Blockende: lieber den vorletzten Einstich ersetzen, damit das Ende stimmt
            if neu.stiche and neu.stiche[-1].art == Art.STICH and len(neu.stiche) >= 2 \
                    and neu.stiche[-2].art == Art.STICH:
                neu.stiche[-1] = s
                letzter_einstich = s.p
                continue
            if abstand(letzter_einstich, s.p) < 1e-9 and neu.stiche and neu.stiche[-1].art == Art.STICH:
                continue
        neu.stiche.append(s)
        letzter_einstich = s.p
    return neu


def lange_stiche_teilen(folge: StichFolge, max_mm: float = MAX_STICH_MM,
                        teil_mm: float = AUTO_JUMP_TEILLAENGE_MM) -> StichFolge:
    """Auto Jump: Stiche > 12,7 mm in Sprünge zu ≤ 7,0 mm zerlegen, Einstich nur am Ende."""
    neu = StichFolge(farben=list(folge.farben))
    pos: Punkt | None = None
    for s in folge.stiche:
        if s.art == Art.STICH and pos is not None:
            d = abstand(pos, s.p)
            if d > max_mm + 1e-9:
                n = math.ceil(d / teil_mm - 1e-9)
                for j in range(1, n):
                    t = j / n
                    neu.stiche.append(Stich(pos[0] + (s.x - pos[0]) * t, pos[1] + (s.y - pos[1]) * t, Art.SPRUNG))
        neu.stiche.append(s)
        pos = s.p
    return neu
