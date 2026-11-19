"""Stiche je Objekt und Stichfunktion aus ART-Musterdesigns ziehen und messen (Analyse_Teil3/04).

Zuordnung (Analyse_Teil3/04 §1): Segmente der Art 0 beginnen eine Stichfunktion; Feld +0x3c = Nummer der Funktion im
Objekt, springt sie auf 0 zurück, beginnt das nächste Objekt. Segmente der Art 1 enthalten die Stiche der Funktion.
Die Funktionen eines Objekts stehen als Verweisliste hinter seinem Eigenschaftsbaum (art_design).

Aufruf:
  python art_messen.py zuordnung DESIGN.ART [TWOFISH_EXE]     Funktionen und Stichblöcke je Objekt
  python art_messen.py satin --alle [TWOFISH_EXE]             Satin: gespeicherter Abstand gegen gemessenen
  python art_messen.py unterlagen --alle [TWOFISH_EXE]        Lauf je Unterlagenart: Säulenmitte oder Rand?
"""
import collections
import glob
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_design import objekte, objekt_auswerten  # noqa: E402
from art_stiche import design_daten, kopf, segmente_dekodieren, stiche_dekodieren  # noqa: E402

MUSTER = r"F:/Entwicklung/artista_ziel/Main/Samples/*/*.ART"
FUNKTIONS_KLASSEN = (0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x33)
UNTERLAGE_ELTERN = {0x1d: "0x2744 Zickzack", 0x1e: "0x2745 Doppel-Zickzack", 0x1f: "0x2746", 0x20: "0x2747",
                    0x65: "0x275a Step", 0x0f: "0x2748 Füllung (Laufweg)"}


def funktionen_je_objekt(segs, pts):
    """Liste Objekte -> Liste Funktionen -> Punktliste (mm)."""
    objs, idx, aktuell, funktion = [], 0, None, None
    for s in segs:
        m = s["stiche"]
        teil = pts[idx:idx + m + 1]
        if s["art"] == 0:
            if s["feld3c"] == 0 or aktuell is None:
                aktuell = []
                objs.append(aktuell)
            funktion = {"nr": s["feld3c"], "punkte": list(teil[1:]) if m else []}
            aktuell.append(funktion)
        elif funktion is not None:
            funktion["punkte"].extend(teil[1:] if funktion["punkte"] else teil)
        idx += m
    return objs


def funktions_verweise(o):
    """Verweisliste der Stichfunktionen eines Objekts (Datensätze mit Verweis auf 0x16/0x18/0x1b/…)."""
    return [e for e in o["datensaetze"] if e[2] and e[3] is not None and e[1] in FUNKTIONS_KLASSEN]


def stichfunktionen(o):
    return [e[1] for e in funktions_verweise(o)]


def laden(pfad, twofish):
    d = design_daten(pfad, twofish)
    h = kopf(d)
    pts, _ = stiche_dekodieren(d, h[6], h[5])
    pts = [(x / 180, y / 360) for x, y in pts]
    segs, _ = segmente_dekodieren(d, h[9], h[8], h[4])
    objs, _ = objekte(d, h)
    return d, h, pts, segs, objs


def zugeordnet(pfade, twofish):
    """Je Design und Objekt: (Design, Objekt, Verweise, Stichblöcke, Datensatzfolge) – nur wo die Zahl passt."""
    for pfad in pfade:
        try:
            d, h, pts, segs, objs = laden(pfad, twofish)
        except Exception as e:  # noqa: BLE001
            print("FEHLER", os.path.basename(pfad), e, file=sys.stderr)
            continue
        fo = funktionen_je_objekt(segs, pts)
        if len(fo) != len(objs):
            continue
        folge = objekte(d, h)[1]
        for o, f in zip(objs, fo):
            refs = funktions_verweise(o)
            if len(refs) == len(f):
                yield pfad, o, refs, f, folge


def satin_messen(pfade, twofish):
    """Gespeicherter Satinabstand (0x15) gegen gemessenen Weg zwischen gleichgerichteten Stichen (Säulenmitte)."""
    mitte = lambda a, b: ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)  # noqa: E731
    eimer = collections.defaultdict(list)
    for _, o, refs, f, _ in zugeordnet(pfade, twofish):
        ab = [p["abstand"] for k, t, p in objekt_auswerten(o)["params"] if k == 0x15]
        for e, blk in zip(refs, f):
            if e[1] != 0x16 or not ab:
                continue
            P = blk["punkte"]
            for i in range(4, len(P) - 6):
                w, w2 = math.dist(P[i], P[i + 1]), math.dist(P[i + 2], P[i + 3])
                if w < 0.3 or abs(w - w2) > 0.15:
                    continue                                    # nur gleichmäßige Stellen
                eimer[(ab[0], round(w * 2) / 2)].append(math.dist(mitte(P[i], P[i + 1]), mitte(P[i + 2], P[i + 3])))
    for (ab, w), v in sorted(eimer.items()):
        if len(v) >= 80:
            v.sort()
            print(f"Abstand {ab:4} ({ab / 180:.3f} mm), Breite {w:4.1f} mm: n={len(v):6}  gemessen Median "
                  f"{v[len(v) // 2]:.3f} mm, Quartile {v[len(v) // 4]:.3f}–{v[3 * len(v) // 4]:.3f}")


def unterlagen_messen(pfade, twofish):
    """Lauf je Unterlagenart: liegt er näher an der Säulenmitte (Satin-Stichmitten) oder am Rand (Einstiche)?"""
    stat = collections.defaultdict(lambda: [0, 0])
    for _, o, refs, f, folge in zugeordnet(pfade, twofish):
        pos = {e[0]: n for n, e in enumerate(folge)}
        P = [p for e, blk in zip(refs, f) if e[1] == 0x16 for p in blk["punkte"]]
        mitten = [((a[0] + b[0]) / 2, (a[1] + b[1]) / 2) for a, b in zip(P, P[1:]) if math.dist(a, b) > 1.2]
        if len(mitten) < 10:
            continue
        for e, blk in zip(refs, f):
            if e[1] != 0x18:
                continue
            n, eltern = pos.get(e[3][0][0]), None
            while n:                                          # Unterlage, unter der der Lauf steht
                n -= 1
                if folge[n][1] in UNTERLAGE_ELTERN and not folge[n][2]:
                    eltern = UNTERLAGE_ELTERN[folge[n][1]]
                    break
            for q in blk["punkte"][1:-1]:
                dm = min(math.dist(q, m) for m in mitten)
                dr = min(math.dist(q, r) for r in P)
                stat[eltern][0 if dm < dr else 1] += 1
    for k, (m, r) in sorted(stat.items(), key=lambda x: str(x[0])):
        print(f"Lauf unter {k}: {m + r} Punkte, näher an der Säulenmitte {100 * m / (m + r):.0f} %, "
              f"näher am Rand {100 * r / (m + r):.0f} %")


if __name__ == "__main__":
    modus, ziel = sys.argv[1], sys.argv[2]
    tw = sys.argv[3] if len(sys.argv) > 3 else "twofish_cfb1"
    pfade = sorted({os.path.normcase(p): p for p in glob.glob(MUSTER)}.values()) if ziel == "--alle" else [ziel]
    if modus == "satin":
        satin_messen(pfade, tw)
    elif modus == "unterlagen":
        unterlagen_messen(pfade, tw)
    else:
        d, h, pts, segs, objs = laden(ziel, tw)
        fo = funktionen_je_objekt(segs, pts)
        print(f"{len(objs)} Objekte im Objektteil, {len(fo)} Objekte in den Stichsegmenten")
        for n, (o, f) in enumerate(zip(objs, fo)):
            print(f"#{n} {o['klasse']}: Funktionen {[hex(k) for k in stichfunktionen(o)]}, "
                  f"Stichblöcke {[len(x['punkte']) for x in f]}")
