import math

import pytest

from stickprogramm.einstellungen import (AUTO_SPACING_TABELLE, LaufEinstellung, SatinEinstellung, auto_spacing,
                                         fuer_stoff)
from stickprogramm.geometrie import abstand, abstand_punkt_strecke, bogen, kumuliert, punkt_bei
from stickprogramm.sticharten.lauf import dreifach, laufstich
from stickprogramm.sticharten.satin import paare_bilden, satin, seiten_aus_mittellinie


# ---------------------------------------------------------------- Einstellungen
@pytest.mark.parametrize("breite,abstand_mm", AUTO_SPACING_TABELLE)
def test_auto_spacing_stuetzstellen(breite, abstand_mm):
    assert auto_spacing(breite) == pytest.approx(abstand_mm)


def test_auto_spacing_interpoliert_und_begrenzt():
    assert auto_spacing(3.5) == pytest.approx(0.41)
    assert auto_spacing(0.0) == pytest.approx(0.54)
    assert auto_spacing(20.0) == pytest.approx(0.34)
    assert auto_spacing(1.0, faktor=0.9) == pytest.approx(0.49 * 0.9)   # Schrift


def test_stoffe():
    e = fuer_stoff("Woven Heavy Weight")
    assert e.satin.faktor == pytest.approx(1.13)
    assert e.satin.abstand(4.0) == pytest.approx(0.40 * 1.13)
    assert e.tatami.abstand == pytest.approx(0.38)
    assert e.unterlage.zickzack_laenge == pytest.approx(3.0)
    d = fuer_stoff()
    assert d.satin.zugausgleich == pytest.approx(0.20)
    assert d.unterlage.zickzack_laenge == pytest.approx(5.0)
    with pytest.raises(ValueError):
        fuer_stoff("Seide")


# ---------------------------------------------------------------- Laufstich
def test_lauf_gerade_gleichmaessig():
    pts = laufstich([(0, 0), (10, 0)])
    assert len(pts) == 5
    assert all(abstand(a, b) == pytest.approx(2.5) for a, b in zip(pts, pts[1:]))
    pts = laufstich([(0, 0), (11, 0)])
    assert len(pts) == 6 and all(abstand(a, b) == pytest.approx(2.2) for a, b in zip(pts, pts[1:]))


def test_lauf_sticht_an_ecken_ein():
    pts = laufstich([(0, 0), (10, 0), (10, 7)])
    assert (10.0, 0.0) in [(round(x, 9), round(y, 9)) for x, y in pts]
    assert pts[0] == (0.0, 0.0) and pts[-1] == (10.0, 7.0)
    assert max(abstand(a, b) for a, b in zip(pts, pts[1:])) <= 2.5 + 1e-9


def test_lauf_variabel_haelt_sehnenabstand():
    kurve = bogen((0, 0), 3.0, 0, 180, sehnenfehler=0.001)
    e = LaufEinstellung(variabel=True)
    pts = laufstich(kurve, e)
    kum = kumuliert(kurve)
    # jede Sehne weicht höchstens 0,09 mm vom Bogen ab (Radius 3 mm: Stiche werden kürzer als 2,5 mm)
    for a, b in zip(pts, pts[1:]):
        mitte_bogen = (0.0, 0.0)
        wa, wb = math.atan2(a[1], a[0]), math.atan2(b[1], b[0])
        wm = (wa + wb) / 2
        p = (3.0 * math.cos(wm), 3.0 * math.sin(wm))
        assert abstand_punkt_strecke(p, a, b) <= 0.09 + 1e-3
    assert max(abstand(a, b) for a, b in zip(pts, pts[1:])) < 2.5
    assert kum[-1] > 0


def test_dreifach():
    pts = laufstich([(0, 0), (5, 0)])
    d = dreifach(pts)
    assert len(d) == 1 + 3 * (len(pts) - 1)
    assert d[:4] == [pts[0], pts[1], pts[0], pts[1]]


# ---------------------------------------------------------------- Satin
def _gerade_saeule(breite, laenge=20.0):
    return [(0.0, breite / 2), (laenge, breite / 2)], [(0.0, -breite / 2), (laenge, -breite / 2)]


def test_satin_abstand_und_seitenwechsel():
    a, b = _gerade_saeule(4.0)
    pts = satin(a, b, SatinEinstellung())
    # Einstiche abwechselnd oben/unten, um den Zugausgleich 0,2 mm außerhalb der Seiten
    for i, p in enumerate(pts[:-1]):
        assert p[1] == pytest.approx(2.2 if i % 2 == 0 else -2.2)
    dx = [q[0] - p[0] for p, q in zip(pts[:-2], pts[1:-1])]
    assert all(d == pytest.approx(0.40) for d in dx)
    assert len(pts) == pytest.approx(20.0 / 0.40 + 2, abs=1)
    assert pts[-1][0] == pytest.approx(20.0)


def test_satin_schmale_bahn_lockerer():
    a, b = _gerade_saeule(0.5)
    pts = satin(a, b)
    dx = [q[0] - p[0] for p, q in zip(pts[:-2], pts[1:-1])]
    assert all(d == pytest.approx(0.51) for d in dx)


def test_satin_ohne_auto_spacing_und_stoff():
    a, b = _gerade_saeule(2.0)
    e = SatinEinstellung(auto_spacing=False, abstand_fest=0.40)
    pts = satin(a, b, e)
    dx = [q[0] - p[0] for p, q in zip(pts[:-2], pts[1:-1])]
    assert all(d == pytest.approx(0.40) for d in dx)
    e2 = fuer_stoff("Knit Medium Weight").satin
    pts2 = satin(a, b, e2)
    dx2 = [q[0] - p[0] for p, q in zip(pts2[:-2], pts2[1:-1])]
    assert all(d == pytest.approx(0.46 * 1.10) for d in dx2)


def test_satin_bogen_bleibt_in_der_bahn():
    mitte = bogen((0, 0), 20.0, 0, 180)
    a, b = seiten_aus_mittellinie(mitte, 3.0)
    pts = satin(a, b)
    for p in pts:
        r = math.hypot(*p)
        assert 20.0 - 1.5 - 0.2 - 0.03 <= r <= 20.0 + 1.5 + 0.2 + 0.03   # Bogen als Sehnenzug (Fehler 0,02 mm)


def test_paare_bilden_verschiedene_punktzahl():
    a = [(0.0, 1.0), (5.0, 1.0), (10.0, 1.0)]
    b = [(0.0, -1.0), (10.0, -1.0)]
    pa, pb = paare_bilden(a, b)
    assert len(pa) == len(pb) == 3
    assert pb[1] == pytest.approx((5.0, -1.0))
    assert punkt_bei(pa, 0.0) == (0.0, 1.0)
