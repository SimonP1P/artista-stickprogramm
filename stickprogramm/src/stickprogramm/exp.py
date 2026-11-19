"""EXP-Format (Melco „Expanded“), wie BERNINA es liest – Analyse/code/07_exp_format.md.

Kein Dateikopf; 2-Byte-Datensätze (dx, dy) in 0,1 mm, vorzeichenbehaftet, +dy nach oben.
Steuerung: 80 01 dx dy = Farbwechsel/Stopp (danach Bewegung), 80 04 dx dy = Sprung,
80 82 xh xl yh yl = langer Sprung (16 Bit). Lesen: hoch (aus dem Code); Schreiben: mittel.
"""

from __future__ import annotations

import math
from pathlib import Path

from .einheiten import EXP_MAX_SCHRITT
from .stiche import Art, Stich, StichFolge

# Standardfarben beim Lesen (EXP enthält keine Farben)
LESE_FARBEN = ["#1f3a93", "#c0392b", "#27ae60", "#f39c12", "#8e44ad", "#16a085", "#2c3e50", "#d35400"]


def _b(v: int) -> int:
    return v & 0xFF


def exp_bytes(folge: StichFolge) -> bytes:
    """Stichfolge (mm, Ursprung = Rahmenmitte) als EXP. Start ist die Rahmenmitte."""
    out = bytearray()
    cx = cy = 0                         # aktuelle Position in 0,1 mm
    vorher_art: Art | None = Art.SPRUNG # am Anfang: erster Einstich gilt als „nach Sprung“
    for s in folge.stiche:
        if s.art == Art.FARBWECHSEL:
            out += bytes([0x80, 0x01, 0, 0])
            vorher_art = Art.FARBWECHSEL
            continue
        tx, ty = round(s.x * 10), round(s.y * 10)
        dx, dy = tx - cx, ty - cy
        if s.art == Art.SPRUNG:
            n = max(1, math.ceil(max(abs(dx), abs(dy)) / EXP_MAX_SCHRITT))
            if dx == 0 and dy == 0:
                continue
            px = py = 0
            for j in range(1, n + 1):
                qx, qy = round(dx * j / n), round(dy * j / n)
                out += bytes([0x80, 0x04, _b(qx - px), _b(qy - py)])
                px, py = qx, qy
            cx, cy = tx, ty
            vorher_art = Art.SPRUNG
            continue
        # Stich
        if dx == 0 and dy == 0 and vorher_art == Art.STICH:
            continue                    # doppelter Einstich
        if max(abs(dx), abs(dy)) > EXP_MAX_SCHRITT:
            # Sicherheitsnetz (sollte nach lange_stiche_teilen nicht vorkommen): Sprünge, dann Einstich
            n = math.ceil(max(abs(dx), abs(dy)) / EXP_MAX_SCHRITT)
            px = py = 0
            for j in range(1, n):
                qx, qy = round(dx * j / n), round(dy * j / n)
                out += bytes([0x80, 0x04, _b(qx - px), _b(qy - py)])
                px, py = qx, qy
            dx, dy = dx - px, dy - py
        out += bytes([_b(dx), _b(dy)])
        cx, cy = tx, ty
        vorher_art = Art.STICH
    return bytes(out)


def exp_schreiben(folge: StichFolge, pfad: str | Path) -> None:
    Path(pfad).write_bytes(exp_bytes(folge))


def _s8(v: int) -> int:
    return v - 256 if v > 127 else v


def exp_lesen_bytes(daten: bytes) -> StichFolge:
    """EXP lesen wie der BERNINA-Leser (BEAGLink.dll ab 0x10007c05)."""
    folge = StichFolge(farben=[LESE_FARBEN[0]])
    x = y = 0
    i = 0
    n = len(daten)
    while i + 1 < n:
        b0, b1 = daten[i], daten[i + 1]
        i += 2
        art = Art.STICH
        if b0 == 0x80:
            if 1 <= b1 <= 15:
                if b1 & 1:
                    folge.stiche.append(Stich(x / 10, y / 10, Art.FARBWECHSEL))
                    folge.farben.append(LESE_FARBEN[len(folge.farben) % len(LESE_FARBEN)])
                art = Art.SPRUNG
                if i + 1 >= n:
                    break
                b0, b1 = daten[i], daten[i + 1]
                i += 2
            elif b1 == 0x82:
                if i + 3 >= n:
                    break
                xh, xl, yh, yl = daten[i:i + 4]
                i += 4
                vx = (xh << 8) | xl
                vy = (yh << 8) | yl
                x += vx - 0x10000 if vx > 0x7FFF else vx
                y += vy - 0x10000 if vy > 0x7FFF else vy
                folge.stiche.append(Stich(x / 10, y / 10, Art.SPRUNG))
                continue
            else:
                continue                # unbekannt: überspringen
        x += _s8(b0)
        y += _s8(b1)
        if art == Art.SPRUNG and _s8(b0) == 0 and _s8(b1) == 0:
            continue                    # Farbwechsel ohne Bewegung
        folge.stiche.append(Stich(x / 10, y / 10, art))
    return folge


def exp_lesen(pfad: str | Path) -> StichFolge:
    return exp_lesen_bytes(Path(pfad).read_bytes())
