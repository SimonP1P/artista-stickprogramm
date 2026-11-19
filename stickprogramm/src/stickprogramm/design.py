"""Design = Rahmen + Stoff + Stickobjekte. berechnen() liefert die fertige Stichfolge."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Sequence

from .einstellungen import Einstellungen, fuer_stoff
from .geometrie import Linie, Punkt, abstand, ohne_doppelte
from .nachbearbeitung import kurze_stiche_entfernen, lange_stiche_teilen, wege_verbinden
from .rahmen import Rahmen, rahmen
from .sticharten.lauf import dreifach, laufstich
from .sticharten.satin import paare_bilden, satin, seiten_aus_mittellinie
from .sticharten.tatami import tatami
from .sticharten.unterlagen import auto_satin_unterlage, flaechen_unterlage, satin_unterlage
from .stiche import StichFolge


@dataclass
class Objekt:
    farbe: str = "#000000"
    name: str = ""

    def wege(self, e: Einstellungen, start: Punkt | None) -> list[Linie]:
        raise NotImplementedError


@dataclass
class Lauf(Objekt):
    """Laufstich oder Dreifachstich entlang einer Linie."""
    punkte: list[Punkt] = field(default_factory=list)
    laenge: float | None = None
    dreifach: bool = False
    geschlossen: bool = False

    def wege(self, e: Einstellungen, start: Punkt | None) -> list[Linie]:
        pts = ohne_doppelte(self.punkte)
        if self.geschlossen and pts and abstand(pts[0], pts[-1]) > 1e-9:
            pts = pts + [pts[0]]
        lauf = laufstich(pts, e.lauf, self.laenge)
        return [dreifach(lauf) if self.dreifach else lauf]


@dataclass
class Satin(Objekt):
    """Satinsäule zwischen zwei Seiten (Punkt i von A gehört zu Punkt i von B)."""
    seite_a: list[Punkt] = field(default_factory=list)
    seite_b: list[Punkt] = field(default_factory=list)
    unterlage: str | list[str] = "auto"

    def _unterlagen(self, a: Linie, b: Linie) -> list[str]:
        if self.unterlage == "auto":
            breiten = [abstand(p, q) for p, q in zip(a, b)]
            return auto_satin_unterlage(sum(breiten) / len(breiten))
        if isinstance(self.unterlage, str):
            return [] if self.unterlage == "keine" else [u.strip() for u in self.unterlage.split("+")]
        return list(self.unterlage)

    def wege(self, e: Einstellungen, start: Punkt | None) -> list[Linie]:
        a, b = paare_bilden(self.seite_a, self.seite_b)
        unter = satin_unterlage(a, b, self._unterlagen(a, b), e.unterlage) if self._unterlagen(a, b) else []
        deck = satin(a, b, e.satin)
        if unter:
            return [unter + deck]   # in einem Zug: Unterlage endet am Säulenanfang
        return [deck]


@dataclass
class SatinLinie(Satin):
    """Satinbahn konstanter Breite entlang einer Mittellinie."""
    punkte: list[Punkt] = field(default_factory=list)
    breite: float = 2.0

    def wege(self, e: Einstellungen, start: Punkt | None) -> list[Linie]:
        self.seite_a, self.seite_b = seiten_aus_mittellinie(self.punkte, self.breite)
        return super().wege(e, start)


@dataclass
class Tatami(Objekt):
    """Tatami-Füllung einer Fläche mit optionalen Löchern."""
    umriss: list[Punkt] = field(default_factory=list)
    loecher: list[list[Punkt]] = field(default_factory=list)
    winkel: float | None = None
    unterlage: bool = True

    def wege(self, e: Einstellungen, start: Punkt | None) -> list[Linie]:
        w = e.tatami.winkel if self.winkel is None else self.winkel
        wege: list[Linie] = []
        if self.unterlage:
            wege.extend(flaechen_unterlage(self.umriss, self.loecher, w, e.unterlage, start))
            start = wege[-1][-1]
        wege.extend(tatami(self.umriss, self.loecher, e.tatami, w, start))
        return wege


@dataclass
class Design:
    name: str = "Design"
    rahmen: Rahmen = field(default_factory=lambda: rahmen("artista 130 x 100"))
    einstellungen: Einstellungen = field(default_factory=lambda: fuer_stoff("0 Software Defaults"))
    objekte: list[Objekt] = field(default_factory=list)

    def berechnen(self) -> StichFolge:
        teile: list[tuple[str, list[Linie]]] = []
        pos: Punkt = (0.0, 0.0)
        for obj in self.objekte:
            wege = [w for w in obj.wege(self.einstellungen, pos) if w]
            if wege:
                teile.append((obj.farbe, wege))
                pos = wege[-1][-1]
        folge = wege_verbinden(teile, self.einstellungen.verbinder)
        folge = kurze_stiche_entfernen(folge)
        folge = lange_stiche_teilen(folge)
        return folge

    def ausserhalb(self, folge: StichFolge) -> list[Punkt]:
        """Einstiche außerhalb des Stickfelds."""
        return [s.p for s in folge.stiche if not self.rahmen.enthaelt(s.p)]


def verschieben(objekte: Sequence[Objekt], dx: float, dy: float) -> None:
    """Alle Objekte verschieben (z. B. Design in der Rahmenmitte ausrichten)."""
    def v(pts: list[Punkt]) -> list[Punkt]:
        return [(x + dx, y + dy) for x, y in pts]
    for o in objekte:
        for name in ("punkte", "seite_a", "seite_b", "umriss"):
            if hasattr(o, name):
                setattr(o, name, v(getattr(o, name)))
        if hasattr(o, "loecher"):
            o.loecher = [v(l) for l in o.loecher]
