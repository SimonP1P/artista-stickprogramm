import math
import random

import pytest

from stickprogramm.einstellungen import TatamiEinstellung, UnterlageEinstellung
from stickprogramm.geometrie import (abstand, abstand_punkt_strecke, drehen, im_polygon, kreis, rand_abstand,
                                     rechteck)
from stickprogramm.sticharten.satin import seiten_aus_mittellinie
from stickprogramm.sticharten.tatami import tatami
from stickprogramm.sticharten.unterlagen import auto_satin_unterlage, flaechen_unterlage, satin_unterlage


def _stiche(wege):
    for w in wege:
        yield from zip(w, w[1:])


def test_tatami_rechteck_reihen_und_grenzen():
    e = TatamiEinstellung(winkel=0.0)
    wege = tatami(rechteck((0, 0), 20, 10), e=e)
    assert len(wege) == 1
    pts = wege[0]
    ys = sorted({round(p[1], 6) for p in pts})
    assert len(ys) == 25                                   # 10 mm / 0,4 mm
    assert all(b - a == pytest.approx(0.4) for a, b in zip(ys, ys[1:]))
    for x, y in pts:
        assert -10.2 - 1e-9 <= x <= 10.2 + 1e-9            # Zugausgleich 0,2 mm
        assert -5 <= y <= 5
    for a, b in _stiche(wege):
        assert abstand(a, b) <= e.laenge + e.min_rand + 1e-9


def test_tatami_versatz_verschiebt_raster():
    e = TatamiEinstellung(winkel=0.0, versatz=0.25)
    pts = tatami(rechteck((0, 0), 40, 2), e=e)[0]
    reihen = {}
    for x, y in pts:
        reihen.setdefault(round(y, 6), []).append(x)
    innen = [sorted(xs)[1:-1] for _, xs in sorted(reihen.items())]
    # Raster jeder Reihe um 0,25 × 4 mm = 1 mm gegenüber der vorigen verschoben
    for r0, r1 in zip(innen, innen[1:]):
        versch = (min(r1) - min(r0)) % 4.0
        assert versch == pytest.approx(1.0) or versch == pytest.approx(5.0 - 4.0)


def test_tatami_loch_bleibt_frei_und_flaeche_bedeckt():
    aussen = kreis((0, 0), 15)
    loch = kreis((0, 0), 6)
    e = TatamiEinstellung()
    wege = tatami(aussen, [loch], e)
    stiche = list(_stiche(wege))
    for a, b in stiche:
        for t in (0.0, 0.5, 1.0):
            p = (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)
            r = math.hypot(*p)
            assert r >= 6 - e.zugausgleich - 0.05, f"Stich im Loch: {a}->{b}"
            assert r <= 15 + e.zugausgleich + 0.05
    # Abdeckung: jeder Punkt im Ring liegt nah an einem Stich
    rnd = random.Random(1)
    for _ in range(200):
        r = rnd.uniform(7, 14)
        w = rnd.uniform(0, 2 * math.pi)
        p = (r * math.cos(w), r * math.sin(w))
        assert min(abstand_punkt_strecke(p, a, b) for a, b in stiche) <= e.abstand


def test_tatami_winkel():
    e = TatamiEinstellung(winkel=30.0)
    wege = tatami(rechteck((0, 0), 30, 30), e=e)
    lange = [(a, b) for a, b in _stiche(wege) if abstand(a, b) > 2.0]
    for a, b in lange:
        w = math.degrees(math.atan2(b[1] - a[1], b[0] - a[0])) % 180
        assert w == pytest.approx(30.0, abs=0.01)


def test_tatami_u_form_mehrere_bereiche():
    u = [(-10, -10), (10, -10), (10, 10), (5, 10), (5, -5), (-5, -5), (-5, 10), (-10, 10)]
    wege = tatami(u, e=TatamiEinstellung(winkel=0.0))
    assert len(wege) >= 2
    for a, b in _stiche(wege):
        m = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
        assert im_polygon(m, [u]) or rand_abstand(m, [u]) <= 0.25


def test_flaechen_unterlage_innerhalb():
    aussen = rechteck((0, 0), 30, 20)
    e = UnterlageEinstellung()
    wege = flaechen_unterlage(aussen, [], 45.0, e)
    for w in wege:
        for x, y in w:
            assert abs(x) <= 15 + 1e-6 and abs(y) <= 10 + 1e-6
    # Unterlage-Füllung quer zur Deckstichrichtung (45° + 90°)
    lange = [(a, b) for a, b in _stiche(wege[1:]) if abstand(a, b) > 2.0]
    winkel = [round(math.degrees(math.atan2(b[1] - a[1], b[0] - a[0])) % 180) for a, b in lange]
    assert max(set(winkel), key=winkel.count) == 135   # häufigste Richtung; Rest: Übergänge an den Kanten


@pytest.mark.parametrize("arten", [["mittellauf"], ["edge_walk"], ["zickzack"], ["doppelzickzack"],
                                   ["edge_walk", "zickzack"]])
def test_satin_unterlage_bleibt_innen_und_endet_am_anfang(arten):
    a, b = seiten_aus_mittellinie([(0.0, 0.0), (30.0, 0.0)], 6.0)
    pts = satin_unterlage(a, b, arten)
    assert pts
    for x, y in pts:
        assert -0.01 <= x <= 30.01 and abs(y) <= 3.0 - 0.2 + 1e-6
    assert pts[-1][0] <= 3.0 or arten == ["zickzack"] and pts[-1][0] <= 3.0
    for p, q in zip(pts, pts[1:]):
        assert abstand(p, q) <= 7.0


def test_auto_satin_unterlage():
    assert auto_satin_unterlage(1.0) == []
    assert auto_satin_unterlage(2.0) == ["mittellauf"]
    assert auto_satin_unterlage(4.0) == ["edge_walk"]
    assert auto_satin_unterlage(7.0) == ["edge_walk", "zickzack"]


def test_drehen_hilfsfunktion():
    assert drehen((1.0, 0.0), 90.0) == pytest.approx((0.0, 1.0))
