import json

import pytest

from stickprogramm.__main__ import beispiel_dict, main
from stickprogramm.design import Design, Lauf, Satin, Tatami
from stickprogramm.eingabe import design_aus_dict
from stickprogramm.einheiten import MAX_STICH_MM, MIN_STICH_MM
from stickprogramm.einstellungen import VerbinderEinstellung
from stickprogramm.exp import exp_bytes, exp_lesen_bytes
from stickprogramm.geometrie import abstand, kreis
from stickprogramm.nachbearbeitung import kurze_stiche_entfernen, lange_stiche_teilen, wege_verbinden
from stickprogramm.rahmen import RAHMEN, rahmen
from stickprogramm.stiche import Art, Stich, StichFolge


# ---------------------------------------------------------------- EXP
def test_exp_bytes_einfach():
    f = StichFolge(farben=["#000"])
    f.sprung((1.0, 0.0))
    f.stich((1.0, 0.0))
    f.stich((2.0, -0.5))
    assert exp_bytes(f) == bytes([0x80, 0x04, 10, 0, 0, 0, 10, 0xFB])


def test_exp_farbwechsel_und_langer_sprung():
    f = StichFolge(farben=["#000"])
    f.stich((0.0, 0.0))
    f.farbwechsel("#f00")
    f.sprung((30.0, 0.0))
    f.stich((30.0, 0.0))
    b = exp_bytes(f)
    assert b[:2] == bytes([0, 0])
    assert b[2:6] == bytes([0x80, 0x01, 0, 0])
    spruenge = [b[i:i + 4] for i in range(6, len(b) - 2, 4)]
    assert len(spruenge) == 3 and all(s[:2] == bytes([0x80, 0x04]) for s in spruenge)
    assert sum(s[2] for s in spruenge) == 300


def test_exp_rundreise_beispiel():
    design = design_aus_dict(beispiel_dict())
    folge = design.berechnen()
    b = exp_bytes(folge)
    zurueck = exp_lesen_bytes(b)
    orig = [s for s in folge.stiche if s.art == Art.STICH]
    neu = [s for s in zurueck.stiche if s.art == Art.STICH]
    assert len(neu) == len(orig)
    for s, t in zip(orig, neu):
        assert abs(s.x - t.x) <= 0.05 + 1e-9 and abs(s.y - t.y) <= 0.05 + 1e-9
    assert zurueck.anzahl(Art.FARBWECHSEL) == folge.anzahl(Art.FARBWECHSEL)
    # Normale Datensätze nie 0x80 als dx (sonst Verwechslung mit Steuerung)
    i = 0
    while i < len(b):
        if b[i] == 0x80:
            i += 4
        else:
            i += 2


# ---------------------------------------------------------------- Nachbearbeitung
def test_kurze_stiche_entfernt_erster_nach_sprung_bleibt():
    f = StichFolge(farben=["#000"])
    f.sprung((0.0, 0.0))
    f.stich((0.0, 0.0))
    f.stich((0.1, 0.0))
    f.stich((2.0, 0.0))
    f.stich((2.1, 0.0))
    g = kurze_stiche_entfernen(f)
    pts = [s.p for s in g.stiche if s.art == Art.STICH]
    assert pts[0] == (0.0, 0.0)
    assert (0.1, 0.0) not in pts
    assert pts[-1] == (2.1, 0.0)                  # Blockende bleibt erhalten
    assert all(l >= MIN_STICH_MM for l in g.stichlaengen())


def test_lange_stiche_auto_jump():
    f = StichFolge(farben=["#000"])
    f.stich((0.0, 0.0))
    f.stich((20.0, 0.0))
    g = lange_stiche_teilen(f)
    arten = [s.art for s in g.stiche]
    assert arten == [Art.STICH, Art.SPRUNG, Art.SPRUNG, Art.STICH]
    xs = [s.x for s in g.stiche]
    assert all(b - a <= 7.0 + 1e-9 for a, b in zip(xs, xs[1:]))


def test_verbinden_vernaehen_und_farbwechsel():
    e = VerbinderEinstellung()
    teile = [("#000", [[(0.0, 0.0), (5.0, 0.0)]]), ("#000", [[(6.0, 0.0), (10.0, 0.0)]]),
             ("#f00", [[(10.0, 5.0), (20.0, 5.0)]])]
    f = wege_verbinden(teile, e)
    assert f.anzahl(Art.FARBWECHSEL) == 1
    assert f.farben == ["#000", "#f00"]
    pts = [s.p for s in f.stiche if s.art == Art.STICH]
    assert pts[:3] == [(0.0, 0.0), (0.8, 0.0), (0.0, 0.0)]     # Anfangs-Vernähen
    # Verbinder 1 mm < 3 mm: kein Vernähen am Ende von Weg 1
    i5 = pts.index((5.0, 0.0))
    assert pts[i5 + 1] == (6.0, 0.0)


# ---------------------------------------------------------------- Rahmen und Design
def test_oval_rahmen():
    r = rahmen("artista 255 x 145 Oval")
    assert r.enthaelt((0.0, 127.0))
    assert r.enthaelt((72.0, 0.0))
    assert not r.enthaelt((72.0, 120.0))
    assert not r.enthaelt((0.0, 128.0))
    g = rahmen("artista 255 x 145 Oval Rotated")
    assert g.enthaelt((127.0, 0.0)) and not g.enthaelt((0.0, 127.0))
    assert "artista 130 x 100 Rotated" in RAHMEN


def test_beispiel_design_regeln():
    design = design_aus_dict(beispiel_dict())
    folge = design.berechnen()
    assert design.ausserhalb(folge) == []
    laengen = folge.stichlaengen()
    assert min(laengen) >= MIN_STICH_MM - 1e-9
    assert max(laengen) <= MAX_STICH_MM + 1e-9
    assert folge.anzahl(Art.FARBWECHSEL) == len(folge.farben) - 1


def test_breite_satinsaeule_auto_jump():
    d = Design(objekte=[Satin(seite_a=[(0, 10), (10, 10)], seite_b=[(0, -10), (10, -10)], unterlage="keine")])
    folge = d.berechnen()
    assert folge.anzahl(Art.SPRUNG) > 10           # 20 mm breite Stiche werden zerlegt
    assert max(folge.stichlaengen()) <= MAX_STICH_MM


def test_json_einstellungen_und_formen(tmp_path):
    d = {"name": "t", "rahmen": "artista 130 x 100", "stoff": "Knit Light Weight",
         "einstellungen": {"satin": {"zugausgleich": 0.35}},
         "objekte": [{"art": "tatami", "umriss": {"kreis": {"mitte": [0, 0], "radius": 5}}, "unterlage": False},
                     {"art": "dreifach", "punkte": [[0, 0], [5, 0]]}]}
    design = design_aus_dict(d)
    assert design.einstellungen.satin.zugausgleich == 0.35
    assert design.einstellungen.satin.faktor == pytest.approx(1.08)
    assert isinstance(design.objekte[0], Tatami) and isinstance(design.objekte[1], Lauf)
    assert design.objekte[1].dreifach
    p = tmp_path / "t.json"
    p.write_text(json.dumps(d), encoding="utf-8")
    assert main(["erzeugen", str(p), "-o", str(tmp_path)]) == 0
    assert (tmp_path / "t.exp").stat().st_size > 0
    assert (tmp_path / "t.svg").read_text(encoding="utf-8").startswith("<svg")


def test_cli_anzeigen_und_listen(tmp_path, capsys):
    assert main(["beispiel", "-o", str(tmp_path)]) == 0
    exp = next(tmp_path.glob("*.exp"))
    assert main(["anzeigen", str(exp), "--rahmen", "artista 130 x 100"]) == 0
    assert main(["rahmen"]) == 0 and main(["stoffe"]) == 0
    assert "Woven Heavy Weight" in capsys.readouterr().out


def test_design_ausserhalb_meldet():
    d = Design(objekte=[Lauf(punkte=[(0, 0), (70, 0)])])     # 130 x 100: x nur bis 50
    folge = d.berechnen()
    assert d.ausserhalb(folge)
    assert kreis((0, 0), 1)
