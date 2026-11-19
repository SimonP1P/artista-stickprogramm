"""Standardwerte der Sticharten – nach dem Vorbild BERNINA artista V4.

Quellen (Sicherheit in Klammern):
- Analyse/ANALYSE.md §5 und Analyse/code/10_algorithmen_schmale_bahnen.md: entschlüsselte NORMAL.AMT,
  Stil „0 Software Defaults“, DEFAULTS.DFL, Handbuch.
- Werte, die nicht aus dem Original stammen, sind als „eigene Festlegung“ markiert und werden in
  Phase 4 an Probestickereien kalibriert.
"""

from __future__ import annotations

import copy
from dataclasses import dataclass, field

# Satin „Auto Spacing“: Säulenbreite (mm) -> Stichabstand (mm), Stil „0 Software Defaults“
# (AMT 0x2734/0x2735, hoch). Zwischen den Stützstellen linear interpoliert (mittel-hoch).
AUTO_SPACING_TABELLE: list[tuple[float, float]] = [
    (0.01, 0.54),
    (0.5, 0.51),
    (1.0, 0.49),
    (1.5, 0.47),
    (2.0, 0.46),
    (3.0, 0.42),
    (4.0, 0.40),
    (5.0, 0.37),
    (6.0, 0.36),
    (7.0, 0.34),
    (8.0, 0.34),
]

# Schrift wird 10 % dichter gestickt (PRESET_LETTERING, Faktor 0,9, hoch)
SCHRIFT_FAKTOR = 0.9


def auto_spacing(breite_mm: float, faktor: float = 1.0) -> float:
    """Satin-Stichabstand für eine Säulenbreite (Tabelle, linear interpoliert, außen konstant)."""
    t = AUTO_SPACING_TABELLE
    if breite_mm <= t[0][0]:
        abstand = t[0][1]
    elif breite_mm >= t[-1][0]:
        abstand = t[-1][1]
    else:
        abstand = t[-1][1]
        for (b0, a0), (b1, a1) in zip(t, t[1:]):
            if b0 <= breite_mm <= b1:
                abstand = a0 + (a1 - a0) * (breite_mm - b0) / (b1 - b0)
                break
    return abstand * faktor


@dataclass
class LaufEinstellung:
    laenge: float = 2.5          # Kontur-Lauf 2,5 mm (AMT 0x27a8, hoch)
    min_laenge: float = 0.8      # min. 0,8 mm (AMT 0x27a8, hoch)
    variabel: bool = False       # „feste Länge“ ist Standard (hoch)
    sehnenabstand: float = 0.09  # Chord Gap bei variabler Länge (Lauf in Flächen, hoch)
    ecke_grad: float = 30.0      # ab diesem Knick wird an der Ecke eingestochen (eigene Festlegung)


@dataclass
class SatinEinstellung:
    auto_spacing: bool = True
    abstand_fest: float = 0.40   # AMT 0x2723 (hoch)
    faktor: float = 1.0          # Stoff-/Stilfaktor AMT 0x2736 (hoch)
    zugausgleich: float = 0.20   # Handbuch S. 199 „automatically set at 0.2 mm“ (hoch)

    def abstand(self, breite_mm: float) -> float:
        if self.auto_spacing:
            return auto_spacing(breite_mm, self.faktor)
        return self.abstand_fest


@dataclass
class TatamiEinstellung:
    abstand: float = 0.40        # Reihenabstand AMT 0x2716 (hoch)
    laenge: float = 4.0          # Stichlänge AMT 0x2717 (hoch)
    min_rand: float = 0.40       # Mindeststich am Rand AMT 0x2718 (hoch)
    versatz: float = 0.25        # Reihenversatz AMT 0x2719 (hoch: Wert; Wirkung als Anteil der Stichlänge je Reihe: mittel)
    winkel: float = 45.0         # Stichwinkel (eigene Festlegung)
    zugausgleich: float = 0.20   # wie Satin (Handbuch S. 199)


@dataclass
class UnterlageEinstellung:
    rand: float = 0.20               # Unterlagenrand AMT 0x2769/0x276a (mittel)
    zugausgleich: float = 0.20       # AMT 0x2743 (hoch)
    edge_walk_laenge: float = 2.2    # AMT 0x2746 (mittel: Zuordnung des Namens)
    mittellauf_laenge: float = 2.2   # AMT 0x2747 (mittel)
    zickzack_laenge: float = 5.0     # AMT 0x2744 (mittel); Bedeutung hier: Weg entlang der Säule je Zick+Zack (niedrig)
    doppelzickzack_laenge: float = 7.0  # AMT 0x2745 (mittel)
    rueck_lauf_laenge: float = 3.0   # „+ Lauf 3,0 mm“ zu Zickzack-Unterlagen (mittel)
    tatami_abstand: float = 3.0      # Tatami-Unterlage AMT 0x275a (mittel)
    tatami_laenge: float = 4.0
    tatami_randlauf: float = 2.5
    tatami_winkel_versatz: float = 90.0  # Winkel 90° zur Deckstichrichtung (AMT 0x27a7)


@dataclass
class VerbinderEinstellung:
    vernaehen_ab: float = 3.0        # Vernähen, wenn Verbinder > 3,0 mm (AMT 0x272a, hoch)
    vernaeh_stichlaenge: float = 0.8 # eigene Festlegung (≥ 0,3 mm Mindeststich)
    vernaeh_stiche: int = 3          # eigene Festlegung


@dataclass
class Einstellungen:
    lauf: LaufEinstellung = field(default_factory=LaufEinstellung)
    satin: SatinEinstellung = field(default_factory=SatinEinstellung)
    tatami: TatamiEinstellung = field(default_factory=TatamiEinstellung)
    unterlage: UnterlageEinstellung = field(default_factory=UnterlageEinstellung)
    verbinder: VerbinderEinstellung = field(default_factory=VerbinderEinstellung)
    stoff: str = "0 Software Defaults"

    def kopie(self) -> Einstellungen:
        return copy.deepcopy(self)


@dataclass(frozen=True)
class Stoff:
    name: str
    satin_abstand_fest: float
    tatami_abstand: float
    auto_spacing_faktor: float
    zugausgleich: float
    verbinder_trim: float


# Stoffe (Settings → Fabric Settings), Analyse/code/10 §E (hoch).
# Zugausgleich „0 Software Defaults“: Tabelle 0,17 (mittel), Handbuch 0,20 (hoch) -> 0,20.
STOFFE: dict[str, Stoff] = {s.name: s for s in [
    Stoff("0 Software Defaults", 0.40, 0.40, 1.00, 0.20, 3.0),
    Stoff("Knit Light Weight", 0.43, 0.45, 1.08, 0.20, 6.0),
    Stoff("Knit Medium Weight", 0.44, 0.45, 1.10, 0.25, 6.0),
    Stoff("Knit Heavy Weight", 0.38, 0.40, 1.00, 0.25, 6.0),
    Stoff("Terry Medium Weight", 0.42, 0.42, 1.05, 0.25, 6.0),
    Stoff("Terry Heavy Weight", 0.38, 0.38, 0.95, 0.25, 6.0),
    Stoff("Woven Light Weight", 0.42, 0.45, 1.05, 0.20, 6.0),
    Stoff("Woven Medium Weight", 0.43, 0.40, 1.08, 0.25, 6.0),
    Stoff("Woven Heavy Weight", 0.38, 0.38, 1.13, 0.25, 6.0),
]}


def fuer_stoff(name: str = "0 Software Defaults") -> Einstellungen:
    """Einstellungen für einen der 8 Stoffe (oder die Software Defaults)."""
    if name not in STOFFE:
        raise ValueError(f"Unbekannter Stoff '{name}'. Bekannt: {', '.join(STOFFE)}")
    s = STOFFE[name]
    e = Einstellungen(stoff=name)
    e.satin.abstand_fest = s.satin_abstand_fest
    e.satin.faktor = s.auto_spacing_faktor
    e.satin.zugausgleich = s.zugausgleich
    e.tatami.abstand = s.tatami_abstand
    e.tatami.zugausgleich = s.zugausgleich
    if name != "0 Software Defaults":
        # Bei allen 8 Stoffen gleich, anders als in den Defaults (Analyse/code/10 §E, hoch)
        e.unterlage.zickzack_laenge = 3.0
        e.unterlage.doppelzickzack_laenge = 3.0
    return e
