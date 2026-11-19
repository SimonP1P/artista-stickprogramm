"""Stichliste: das Ergebnis der Berechnung und die Eingabe für Export und Vorschau."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum

from .geometrie import Punkt, abstand, begrenzung


class Art(Enum):
    STICH = "stich"              # Nadel sticht an (x, y) ein
    SPRUNG = "sprung"            # Rahmen fährt nach (x, y), ohne einzustechen
    FARBWECHSEL = "farbwechsel"  # Maschine hält an, nächste Farbe; (x, y) = aktuelle Position


@dataclass(frozen=True)
class Stich:
    x: float
    y: float
    art: Art = Art.STICH

    @property
    def p(self) -> Punkt:
        return (self.x, self.y)


@dataclass
class StichFolge:
    """Absolute Positionen in mm (Ursprung = Rahmenmitte). farben[i] gilt für Farbblock i."""

    stiche: list[Stich] = field(default_factory=list)
    farben: list[str] = field(default_factory=list)

    def stich(self, p: Punkt) -> None:
        self.stiche.append(Stich(p[0], p[1], Art.STICH))

    def sprung(self, p: Punkt) -> None:
        self.stiche.append(Stich(p[0], p[1], Art.SPRUNG))

    def farbwechsel(self, neue_farbe: str) -> None:
        pos = self.position() or (0.0, 0.0)
        self.stiche.append(Stich(pos[0], pos[1], Art.FARBWECHSEL))
        self.farben.append(neue_farbe)

    def position(self) -> Punkt | None:
        return self.stiche[-1].p if self.stiche else None

    def anzahl(self, art: Art = Art.STICH) -> int:
        return sum(1 for s in self.stiche if s.art == art)

    def begrenzung(self) -> tuple[float, float, float, float] | None:
        pts = [s.p for s in self.stiche if s.art == Art.STICH]
        return begrenzung(pts) if pts else None

    def stichlaengen(self) -> list[float]:
        """Längen aller echten Stiche (Einstich nach Einstich, ohne Sprünge davor)."""
        out = []
        vorher = None
        for s in self.stiche:
            if s.art == Art.STICH:
                if vorher is not None and vorher.art == Art.STICH:
                    out.append(abstand(vorher.p, s.p))
                vorher = s
            elif s.art == Art.SPRUNG:
                vorher = s
        return out

    def farbbloecke(self) -> list[tuple[str, list[Stich]]]:
        """Stiche je Farbe (Farbwechsel trennt die Blöcke)."""
        bloecke: list[tuple[str, list[Stich]]] = []
        idx = 0
        aktuell: list[Stich] = []
        for s in self.stiche:
            if s.art == Art.FARBWECHSEL:
                bloecke.append((self._farbe(idx), aktuell))
                idx += 1
                aktuell = []
            else:
                aktuell.append(s)
        bloecke.append((self._farbe(idx), aktuell))
        return bloecke

    def _farbe(self, i: int) -> str:
        return self.farben[i] if i < len(self.farben) else "#000000"
