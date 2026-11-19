"""Stiche je Objekt und Stichfunktion aus ART-Musterdesigns ziehen und messen (Analyse_Teil3/04).

Zuordnung (Analyse_Teil3/04 §1): Segmente der Art 0 beginnen eine Stichfunktion; Feld +0x3c = Nummer der Funktion im
Objekt, springt sie auf 0 zurück, beginnt das nächste Objekt. Segmente der Art 1 enthalten die Stiche der Funktion.
Die Funktionen eines Objekts stehen als Verweisliste hinter seinem Eigenschaftsbaum (art_design).

Aufruf:
  python art_messen.py zuordnung DESIGN.ART [TWOFISH_EXE]     Funktionen und Stichblöcke je Objekt
  python art_messen.py satin --alle [TWOFISH_EXE]             Satin: gespeicherter Abstand gegen gemessenen
  python art_messen.py unterlagen --alle [TWOFISH_EXE]        Lauf je Unterlagenart: Säulenmitte oder Rand?
  python art_messen.py boegen --alle [TWOFISH_EXE]            Satin in Bögen: Abstand außen / Mitte / innen
  python art_messen.py effekte --alle [TWOFISH_EXE]           Satin mit Auto-Spacing-Effekt 0x21: Tabelle × Faktor?
  python art_messen.py tatami --alle [TWOFISH_EXE]            Tatami: Reihenabstand und Versatz der Einstiche
  python art_messen.py ecken --alle [TWOFISH_EXE]             Laufstich: Einstich an Eck- und Kurvenknoten?
"""
import collections
import glob
import math
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_design import felder, objekte, objekt_auswerten  # noqa: E402
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


def kette(e):
    """Datensatz samt Basisklassen; Verweise aufgelöst."""
    if e[2]:
        return [x for y in (e[3] or []) for x in kette(y)]
    return [e]


def _alle(o):
    return [x for e in o["datensaetze"] for x in kette(e)]


def _mitte(a, b):
    return ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)


def _zeile(name, v):
    v = sorted(v)
    n = len(v)
    print(f"{name}: n={n:6}  Median {v[n // 2]:.3f}  Quartile {v[n // 4]:.3f}–{v[3 * n // 4]:.3f}")


def boegen_messen(pfade, twofish):
    """Satin mit festem Abstand: Weg zwischen Einstichen derselben Seite außen/innen/Mitte je Krümmung (04 §4)."""
    erg = collections.defaultdict(list)
    for _, o, refs, f, _ in zugeordnet(pfade, twofish):
        alle = _alle(o)
        ab = [struct.unpack_from("<I", x[3])[0] / 180 for x in alle if x[1] == 0x15]
        if any(x[1] == 0x21 for x in alle) or not ab or ab[0] > 2:
            continue
        for e, blk in zip(refs, f):
            if e[1] != 0x16:
                continue
            P = blk["punkte"]
            for i in range(4, len(P) - 6, 2):
                a0, b0, a1, b1 = P[i], P[i + 1], P[i + 2], P[i + 3]
                if math.dist(a0, b0) < 1.0 or abs(math.dist(a0, b0) - math.dist(a1, b1)) > 0.15:
                    continue
                dA, dB = math.dist(a0, a1), math.dist(b0, b1)
                r = max(dA, dB) / max(min(dA, dB), 1e-6)
                kl = "1 gerade" if r < 1.1 else "2 leicht" if r < 1.5 else "3 stark" if r < 3 else "4 sehr stark"
                erg[(kl, "außen")].append(max(dA, dB) / ab[0])
                erg[(kl, "innen")].append(min(dA, dB) / ab[0])
                erg[(kl, "Mitte")].append(math.dist(_mitte(a0, b0), _mitte(a1, b1)) / ab[0])
    for k in sorted(erg):
        _zeile(f"{k[0]:13} {k[1]:6} gemessen/Abstand", erg[k])


def effekte_messen(pfade, twofish):
    """Satin mit Effekt 0x21: Verhältnis gemessener Abstand zu Tabelle(Stichlänge) × Faktor 0x22 (05 §1)."""
    def tab(w, W, S):
        if w <= W[0]:
            return S[0]
        for i in range(len(W) - 1):
            if w <= W[i + 1]:
                return S[i] + (S[i + 1] - S[i]) * (w - W[i]) / (W[i + 1] - W[i])
        return S[-1]
    erg = collections.defaultdict(list)
    for _, o, refs, f, _ in zugeordnet(pfade, twofish):
        alle = _alle(o)
        eff = [x for x in alle if x[1] == 0x21]
        fak = [struct.unpack("<f", x[3][:4])[0] for x in alle if x[1] == 0x22 and len(x[3]) >= 4]
        ab = [struct.unpack_from("<I", x[3])[0] / 180 for x in alle if x[1] == 0x15]
        if not ab:
            continue
        W = S = None
        if eff:
            v = struct.unpack("<22f", eff[0][3][:88])
            W, S = [x / 180 for x in v[:11]], [x / 180 for x in v[11:]]
        for e, blk in zip(refs, f):
            if e[1] != 0x16:
                continue
            P = blk["punkte"]
            for i in range(4, len(P) - 6):
                w = math.dist(P[i], P[i + 1])
                if w < 0.3 or abs(w - math.dist(P[i + 2], P[i + 3])) > 0.15:
                    continue
                s = math.dist(_mitte(P[i], P[i + 1]), _mitte(P[i + 2], P[i + 3]))
                if eff:
                    erg["mit 0x21: gemessen/(Tabelle × Faktor)"].append(s / (tab(w, W, S) * (fak[0] if fak else 1.0)))
                else:
                    erg["ohne 0x21: gemessen/fester Abstand"].append(s / ab[0])
    for k in sorted(erg):
        _zeile(k, erg[k])


def tatami_messen(pfade, twofish):
    """Tatami: Abstand benachbarter und übernächster Reihen, Versatz der Einstiche (06)."""
    def fit(r):
        n = len(r)
        mx, my = sum(x for x, _ in r) / n, sum(y for _, y in r) / n
        sxx = sum((x - mx) ** 2 for x, _ in r)
        b = sum((x - mx) * (y - my) for x, y in r) / sxx if sxx > 1e-9 else 0.0
        return lambda x: my + b * (x - mx)

    def phase(r, L):
        z = sum(complex(math.cos(2 * math.pi * x / L), math.sin(2 * math.pi * x / L)) for x, _ in r[1:-1])
        return (math.atan2(z.imag, z.real) / (2 * math.pi)) % 1.0, abs(z) / (len(r) - 2)

    gap = collections.defaultdict(list)
    ph = collections.defaultdict(collections.Counter)
    for _, o, refs, f, _ in zugeordnet(pfade, twofish):
        for e, blk in zip(refs, f):
            if e[1] != 0x1b:
                continue
            ch = kette(e)
            t = next((felder(0x1b, x[3]) for x in ch if x[1] == 0x1b), None)
            s = next((struct.unpack_from("<I", x[3])[0] for x in ch if x[1] == 0x15), None)
            if not t or "versatz" not in t or not s:
                continue
            L, sp = t["laenge"] / 180, s / 180
            key = tuple(round(v, 2) for v in t["versatz"][:2])
            P = blk["punkte"]
            cx = cy = 0.0
            for a, b in zip(P, P[1:]):                      # Reihenrichtung aus Stichen voller Länge
                if abs(math.dist(a, b) - L) < 0.02:
                    w = math.atan2(b[1] - a[1], b[0] - a[0])
                    cx, cy = cx + math.cos(2 * w), cy + math.sin(2 * w)
            if cx == 0 and cy == 0:
                continue
            th = math.atan2(cy, cx) / 2
            c, sn = math.cos(-th), math.sin(-th)
            R = [(x * c - y * sn, x * sn + y * c) for x, y in P]
            reihen, akt = [], [R[0]]
            for a, b in zip(R, R[1:]):
                d = math.dist(a, b)
                gerade = d > 0.25 and abs(b[1] - a[1]) / d < math.sin(math.radians(4))
                if gerade and (len(akt) < 2 or (b[0] - a[0]) * (akt[-1][0] - akt[-2][0]) > 0):
                    akt.append(b)
                else:
                    if len(akt) >= 4:
                        reihen.append(akt)
                    akt = [a, b] if gerade else [b]
            if len(akt) >= 4:
                reihen.append(akt)
            for i in range(len(reihen) - 2):
                r1, r2, r3 = reihen[i:i + 3]
                lo = max(min(x for x, _ in r) for r in (r1, r2, r3))
                hi = min(max(x for x, _ in r) for r in (r1, r2, r3))
                if hi - lo < 2 * L:
                    continue
                xm = (lo + hi) / 2
                g1, g2 = abs(fit(r2)(xm) - fit(r1)(xm)) / sp, abs(fit(r3)(xm) - fit(r1)(xm)) / sp
                if g2 > 3:
                    continue
                gap["Nachbarreihe"].append(g1)
                gap["übernächste Reihe"].append(g2)
                (p1, k1), (p2, k2), (p3, k3) = phase(r1, L), phase(r2, L), phase(r3, L)
                if min(k1, k2, k3) > 0.9:
                    ph[(key, "Nachbarreihe")][round(min((p2 - p1) % 1, (p1 - p2) % 1), 2)] += 1
                    ph[(key, "übernächste")][round(min((p3 - p1) % 1, (p1 - p3) % 1), 2)] += 1
    for k in sorted(gap):
        _zeile(f"Reihenabstand {k} / eingestellter Abstand", gap[k])
    for (key, art), c in sorted(ph.items()):
        g = sum(c.values())
        if g >= 40:
            print(f"Versatz {key}, {art} ({g}): " + ", ".join(f"{v:.2f}: {100 * n / g:.0f} %" for v, n in c.most_common(4)))


def ecken_messen(pfade, twofish):
    """Laufstich-Kurven: Einstich an den Knoten der Form? Getrennt nach Knotenart (Ecke/Kurve) und Knickwinkel (07)."""
    stat = collections.defaultdict(lambda: [0, 0])
    for _, o, refs, f, _ in zugeordnet(pfade, twofish):
        if o["klasse"] != "EmbroideryCurve" or [e[1] for e in refs] != [0x18]:
            continue
        r = objekt_auswerten(o)
        if len(r["formen"]) != 1:
            continue
        P, kn = f[0]["punkte"], r["formen"][0]["knoten"]
        for i in range(1, len(kn) - 1):
            q = (kn[i]["punkt"][0] / 180, kn[i]["punkt"][1] / 180)
            ein, aus = kn[i]["ein"] or kn[i - 1]["punkt"], kn[i]["aus"] or kn[i + 1]["punkt"]
            v1 = (kn[i]["punkt"][0] - ein[0], kn[i]["punkt"][1] - ein[1])
            v2 = (aus[0] - kn[i]["punkt"][0], aus[1] - kn[i]["punkt"][1])
            if math.hypot(*v1) < 1e-6 or math.hypot(*v2) < 1e-6:
                continue
            w = abs(math.degrees(math.atan2(v1[0] * v2[1] - v1[1] * v2[0], v1[0] * v2[0] + v1[1] * v2[1])))
            treffer = min(math.dist(q, s) for s in P) < 0.05
            stat[("Ecke" if kn[i]["art"] else "Kurve", int(w // 30) * 30)][0 if treffer else 1] += 1
    for (art, b), (j, n) in sorted(stat.items()):
        print(f"{art:5} Knick {b:3}–{b + 30:3}°: {j:6} von {j + n:6} Knoten mit Einstich ({100 * j / (j + n):.0f} %)")


if __name__ == "__main__":
    modus, ziel = sys.argv[1], sys.argv[2]
    tw = sys.argv[3] if len(sys.argv) > 3 else "twofish_cfb1"
    pfade = sorted({os.path.normcase(p): p for p in glob.glob(MUSTER)}.values()) if ziel == "--alle" else [ziel]
    if modus == "satin":
        satin_messen(pfade, tw)
    elif modus == "unterlagen":
        unterlagen_messen(pfade, tw)
    elif modus == "boegen":
        boegen_messen(pfade, tw)
    elif modus == "effekte":
        effekte_messen(pfade, tw)
    elif modus == "tatami":
        tatami_messen(pfade, tw)
    elif modus == "ecken":
        ecken_messen(pfade, tw)
    else:
        d, h, pts, segs, objs = laden(ziel, tw)
        fo = funktionen_je_objekt(segs, pts)
        print(f"{len(objs)} Objekte im Objektteil, {len(fo)} Objekte in den Stichsegmenten")
        for n, (o, f) in enumerate(zip(objs, fo)):
            print(f"#{n} {o['klasse']}: Funktionen {[hex(k) for k in stichfunktionen(o)]}, "
                  f"Stichblöcke {[len(x['punkte']) for x in f]}")
