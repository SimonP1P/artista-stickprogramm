"""Vorschau einer Stichfolge als SVG (ohne Zusatzpakete, im Browser anzusehen)."""

from __future__ import annotations

from html import escape
from pathlib import Path

from .rahmen import Rahmen
from .stiche import Art, StichFolge


def svg_text(folge: StichFolge, rahmen: Rahmen | None = None, titel: str = "", fadenbreite: float = 0.35,
             einstiche: bool = False, rand: float = 5.0, pixel_pro_mm: float = 4.0) -> str:
    """SVG in mm-Koordinaten (1 Einheit = 1 mm), y nach unten gespiegelt."""
    pts = [s.p for s in folge.stiche]
    if rahmen is not None:
        pts += rahmen.form
    if not pts:
        pts = [(0.0, 0.0)]
    x0 = min(p[0] for p in pts) - rand
    x1 = max(p[0] for p in pts) + rand
    y0 = min(p[1] for p in pts) - rand
    y1 = max(p[1] for p in pts) + rand
    b, h = x1 - x0, y1 - y0
    teile = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{x0:.2f} {-y1:.2f} {b:.2f} {h:.2f}" '
        f'width="{b * pixel_pro_mm:.0f}" height="{h * pixel_pro_mm:.0f}">',
        f'<title>{escape(titel)}</title>',
        f'<rect x="{x0:.2f}" y="{-y1:.2f}" width="{b:.2f}" height="{h:.2f}" fill="#f7f4ee"/>',
    ]
    if rahmen is not None:
        d = " ".join(f"{x:.2f},{-y:.2f}" for x, y in rahmen.form)
        teile.append(f'<polygon points="{d}" fill="#ffffff" stroke="#9aa5b1" stroke-width="0.4" '
                     f'stroke-dasharray="2 1.5"/>')
        teile.append('<path d="M -3 0 H 3 M 0 -3 V 3" stroke="#9aa5b1" stroke-width="0.25"/>')
    for farbe, stiche in folge.farbbloecke():
        linien: list[list[tuple[float, float]]] = []
        spruenge: list[tuple[tuple[float, float], tuple[float, float]]] = []
        aktuell: list[tuple[float, float]] = []
        vorher = None
        for s in stiche:
            if s.art == Art.SPRUNG:
                if len(aktuell) > 1:
                    linien.append(aktuell)
                if vorher is not None:
                    spruenge.append((vorher, s.p))
                aktuell = [s.p]
            else:
                aktuell.append(s.p)
            vorher = s.p
        if len(aktuell) > 1:
            linien.append(aktuell)
        for a, z in spruenge:
            teile.append(f'<line x1="{a[0]:.2f}" y1="{-a[1]:.2f}" x2="{z[0]:.2f}" y2="{-z[1]:.2f}" '
                         f'stroke="#b0b0b0" stroke-width="0.15" stroke-dasharray="0.8 0.8"/>')
        for lin in linien:
            d = " ".join(f"{x:.2f},{-y:.2f}" for x, y in lin)
            teile.append(f'<polyline points="{d}" fill="none" stroke="{escape(farbe)}" '
                         f'stroke-width="{fadenbreite}" stroke-linejoin="round" stroke-linecap="round" '
                         f'stroke-opacity="0.9"/>')
            if einstiche:
                for x, y in lin:
                    teile.append(f'<circle cx="{x:.2f}" cy="{-y:.2f}" r="0.12" fill="#000"/>')
    teile.append("</svg>")
    return "\n".join(teile)


def svg_schreiben(folge: StichFolge, pfad: str | Path, **kw) -> None:
    Path(pfad).write_text(svg_text(folge, **kw), encoding="utf-8")
