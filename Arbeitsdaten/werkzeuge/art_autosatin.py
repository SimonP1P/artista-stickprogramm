"""Automatisch erzeugte Satinsäulen (EmbroideryPatch, Klasse 0x06) der ART-Musterdesigns lesen und zeichnen.

Datensatz 0x06 (IMPORT.DLL 0x10018970): u16 Anzahl Stichlinien, je Linie zwei Umrissstellen (CurveIndex):
ReadBytePacked Segment, float t, ReadBytePacked Segment, float t. Danach folgt der Umriss (GeoCurve), aus dem MOD1
`EmbroideryPatch::SplitSides` die beiden Seiten macht. Von Hand digitalisierte Säulen haben keine Stichlinien, sondern
zwei Seitenkurven.

Aufruf:  python art_autosatin.py DESIGN.ART TWOFISH_EXE [ZIEL.svg]     Liste + SVG (Umriss, Stichlinien)
         python art_autosatin.py messen "MUSTER/*.ART" TWOFISH_EXE    Abstand/Winkel benachbarter Stichlinien
         python art_autosatin.py export "MUSTER/*.ART" TWOFISH_EXE FORMEN.json   Umrisse für `stick saeulen`
         python art_autosatin.py vergleich FORMEN.json FORMEN.saeulen.json        Kern gegen Original
"""
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_design import objekte, objekt_auswerten  # noqa: E402
from art_stiche import design_daten, kopf  # noqa: E402


def _bytepacked(b, i):
    """WESSYS ReadBytePacked: 7 Bit je Byte, Bit 7 = weiteres Byte folgt."""
    v, s = 0, 0
    while True:
        x = b[i]
        i += 1
        v |= (x & 0x7f) << s
        s += 7
        if not x & 0x80:
            return v, i


def stichlinien(roh):
    import struct
    n = struct.unpack_from("<H", roh, 0)[0]
    i, out = 2, []
    for _ in range(n):
        a, i = _bytepacked(roh, i)
        ta = struct.unpack_from("<f", roh, i)[0]
        i += 4
        b, i = _bytepacked(roh, i)
        tb = struct.unpack_from("<f", roh, i)[0]
        i += 4
        out.append(((a, ta), (b, tb)))
    return out, i


def _bez(p0, p1, p2, p3, t):
    k = ((1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3)
    return (sum(c * q[0] for c, q in zip(k, (p0, p1, p2, p3))), sum(c * q[1] for c, q in zip(k, (p0, p1, p2, p3))))


def segment_punkt(form, seg, t):
    """Punkt auf Segment `seg` (Knoten seg → seg+1, geschlossen) bei Parameter t, in mm."""
    kn = form["knoten"]
    n = len(kn)
    a, b = kn[seg % n], kn[(seg + 1) % n]
    p0, p3 = (a["punkt"][0] / 180, a["punkt"][1] / 180), (b["punkt"][0] / 180, b["punkt"][1] / 180)
    p1 = (a["aus"][0] / 180, a["aus"][1] / 180) if a["aus"] else p0
    p2 = (b["ein"][0] / 180, b["ein"][1] / 180) if b["ein"] else p3
    return _bez(p0, p1, p2, p3, t)


def umriss(form, n=8):
    kn = form["knoten"]
    return [segment_punkt(form, s, i / n) for s in range(len(kn)) for i in range(n)]


def auto_saeulen(pfad, tw):
    d = design_daten(pfad, tw)
    objs, _ = objekte(d, kopf(d))
    for nr, o in enumerate(objs):
        if o["klasse"] != "EmbroideryPatch":
            continue
        roh = next((e[3] for e in o["datensaetze"] if e[1] == 0x06 and not e[2]), None)
        if not roh or len(roh) < 2:
            continue
        linien, ende = stichlinien(roh)
        r = objekt_auswerten(o)
        formen = [f for f in r["formen"] if len(f["knoten"]) >= 2]
        if not linien or len(formen) != 1:
            continue
        yield nr, formen[0], linien


def svg(saeulen, ziel):
    teile, xs, ys = [], [], []
    for nr, form, linien in saeulen:
        u = umriss(form)
        xs += [q[0] for q in u]
        ys += [q[1] for q in u]
        teile.append('<polygon points="%s" fill="#eef" stroke="#448" stroke-width="0.08"/>' %
                     " ".join(f"{x:.2f},{-y:.2f}" for x, y in u))
        for (a, ta), (b, tb) in linien:
            p, q = segment_punkt(form, a, ta), segment_punkt(form, b, tb)
            teile.append(f'<line x1="{p[0]:.2f}" y1="{-p[1]:.2f}" x2="{q[0]:.2f}" y2="{-q[1]:.2f}" '
                         'stroke="#c00" stroke-width="0.06"/>')
        kn = form["knoten"][0]["punkt"]
        teile.append(f'<text x="{kn[0] / 180:.1f}" y="{-kn[1] / 180:.1f}" font-size="1.5">{nr}</text>')
    x0, x1, y0, y1 = min(xs), max(xs), min(ys), max(ys)
    with open(ziel, "w", encoding="utf-8") as f:
        f.write(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{x0 - 2} {-y1 - 2} {x1 - x0 + 4} {y1 - y0 + 4}" '
                f'width="{(x1 - x0 + 4) * 12:.0f}" height="{(y1 - y0 + 4) * 12:.0f}">'
                f'<rect x="{x0 - 2}" y="{-y1 - 2}" width="{x1 - x0 + 4}" height="{y1 - y0 + 4}" fill="white"/>'
                + "".join(teile) + "</svg>")


def messen(pfade, tw):
    """Je Paar benachbarter Stichlinien: Abstand der Mitten, Winkeländerung, Breite (Linienlänge)."""
    import collections
    abst, rel, winkel, breite, anzahl = [], [], [], [], collections.Counter()
    for pfad in pfade:
        try:
            liste = list(auto_saeulen(pfad, tw))
        except Exception as e:  # noqa: BLE001
            print("FEHLER", os.path.basename(pfad), e, file=sys.stderr)
            continue
        for nr, form, linien in liste:
            anzahl[os.path.basename(pfad)] += 1
            seg = [(segment_punkt(form, a, ta), segment_punkt(form, b, tb)) for (a, ta), (b, tb) in linien]
            seg = [(p, q) for p, q in seg if math.dist(p, q) > 0.2]          # Spitzen (Länge 0) weglassen
            for (p1, q1), (p2, q2) in zip(seg, seg[1:]):
                m1 = ((p1[0] + q1[0]) / 2, (p1[1] + q1[1]) / 2)
                m2 = ((p2[0] + q2[0]) / 2, (p2[1] + q2[1]) / 2)
                w = (math.dist(p1, q1) + math.dist(p2, q2)) / 2
                a1 = math.atan2(q1[1] - p1[1], q1[0] - p1[0])
                a2 = math.atan2(q2[1] - p2[1], q2[0] - p2[0])
                dw = abs((math.degrees(a2 - a1) + 90) % 180 - 90)
                abst.append(math.dist(m1, m2))
                rel.append(math.dist(m1, m2) / w)
                winkel.append(dw)
                breite.append(w)
    def z(n, v):
        v = sorted(v)
        print(f"{n}: n={len(v)}, Median {v[len(v) // 2]:.2f}, 10 % {v[len(v) // 10]:.2f}, 25 % {v[len(v) // 4]:.2f}, "
              f"75 % {v[3 * len(v) // 4]:.2f}, 90 % {v[9 * len(v) // 10]:.2f}")
    print(dict(anzahl))
    z("Breite (mm)", breite)
    z("Abstand der Linienmitten (mm)", abst)
    z("Abstand / Breite", rel)
    z("Winkeländerung (Grad)", winkel)


def export(pfade, tw, ziel):
    """Umrisse der automatisch erzeugten Säulen als Eingabe für `stick saeulen` (mit den Original-Stichlinien)."""
    import json
    formen = []
    for pfad in pfade:
        for nr, form, linien in auto_saeulen(pfad, tw):
            ring = umriss(form, 8)
            orig = [[segment_punkt(form, a, ta), segment_punkt(form, b, tb)] for (a, ta), (b, tb) in linien]
            formen.append({"datei": os.path.basename(pfad), "nr": nr, "umriss": ring, "original": orig})
    with open(ziel, "w", encoding="utf-8") as f:
        json.dump({"formen": formen}, f)
    print(f"{len(formen)} Formen -> {ziel}")


def vergleich(eingabe, ergebnis):
    """Stichrichtung des Kerns an den Original-Stichlinien: Winkelabweichung, Zahl der Säulen und Linien."""
    import json
    e = json.load(open(eingabe, encoding="utf-8"))["formen"]
    r = json.load(open(ergebnis, encoding="utf-8"))["formen"]
    abw, saeulen, linien_o, linien_k, abst_k = [], [], 0, 0, []
    for f, g in zip(e, r):
        saeulen.append(len(g["saeulen"]))
        paare = [(a, b) for s in g["saeulen"] for a, b in zip(s["seite_a"], s["seite_b"]) if math.dist(a, b) > 0.05]
        linien_k += sum(len(s["linien"]) for s in g["saeulen"])
        for s in g["saeulen"]:
            ls = [l for l in s["linien"] if math.dist(*l) > 0.2]
            for l1, l2 in zip(ls, ls[1:]):
                m1 = ((l1[0][0] + l1[1][0]) / 2, (l1[0][1] + l1[1][1]) / 2)
                m2 = ((l2[0][0] + l2[1][0]) / 2, (l2[0][1] + l2[1][1]) / 2)
                abst_k.append(math.dist(m1, m2) / ((math.dist(*l1) + math.dist(*l2)) / 2))
        for p_, q in f["original"]:
            if math.dist(p_, q) < 0.2 or not paare:
                continue
            linien_o += 1
            m = ((p_[0] + q[0]) / 2, (p_[1] + q[1]) / 2)
            a, b = min(paare, key=lambda x: math.dist(m, ((x[0][0] + x[1][0]) / 2, (x[0][1] + x[1][1]) / 2)))
            w1 = math.atan2(q[1] - p_[1], q[0] - p_[0])
            w2 = math.atan2(b[1] - a[1], b[0] - a[0])
            abw.append(abs((math.degrees(w2 - w1) + 90) % 180 - 90))
    abw.sort()
    n = len(abw)
    print(f"Formen {len(e)}, Säulen je Form: {dict(sorted(__import__('collections').Counter(saeulen).items()))}")
    print(f"Stichlinien Original {linien_o} (ohne Spitzen), Kern {linien_k}")
    print(f"Winkelabweichung an den Original-Linien: Median {abw[n // 2]:.1f}°, 75 % {abw[3 * n // 4]:.1f}°, "
          f"90 % {abw[9 * n // 10]:.1f}°, unter 10°: {sum(x < 10 for x in abw) / n:.0%}")
    abst_k.sort()
    if abst_k:
        print(f"Kern: Abstand benachbarter Linien / Breite: Median {abst_k[len(abst_k) // 2]:.2f}")


if __name__ == "__main__" and sys.argv[1] == "export":
    import glob
    export(sorted(glob.glob(sys.argv[2])), sys.argv[3], sys.argv[4])
elif __name__ == "__main__" and sys.argv[1] == "vergleich":
    vergleich(sys.argv[2], sys.argv[3])
elif __name__ == "__main__" and sys.argv[1] == "messen":
    import glob
    messen(sorted(glob.glob(sys.argv[2])), sys.argv[3])
elif __name__ == "__main__":
    s = list(auto_saeulen(sys.argv[1], sys.argv[2]))
    for nr, form, linien in s:
        print(f"#{nr}: {len(form['knoten'])} Knoten, {len(linien)} Stichlinien: "
              + " ".join(f"{a}:{ta:.2f}-{b}:{tb:.2f}" for (a, ta), (b, tb) in linien[:8]) + " …")
    if len(sys.argv) > 3:
        svg(s, sys.argv[3])
