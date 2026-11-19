"""Objekte eines ART-Musterdesigns lesen: Klasse, Form (Bézier-Knoten), Stichart mit Parametern, Unterlage,
Verbinder – und die zugehörigen Stiche. Grundlage: Analyse_Teil3/03_art_objekte.md.

Aufbau (IMPORT.DLL): Objektabschnitt = Folge von Datensätzen (art_objekte.datensatz). Abgeleitete Klassen
stehen vor ihren Basisklassen (z. B. Satin 0x16 -> 0x15 -> 0x12 -> 0x0d Kennung). Datensätze mit Bit a werden
fortlaufend nummeriert; „Verweis n“ meint den n-ten solchen Datensatz.

Aufruf:  python art_design.py DESIGN.ART [TWOFISH_EXE]
"""
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_objekte import datensatz, knoten_lesen  # noqa: E402
from art_stiche import design_daten, kopf, packed, segmente_dekodieren, stiche_dekodieren  # noqa: E402

# Klassennummer -> Name (Fabrik-Konstruktoren in IMPORT.DLL, Analyse_Teil3/03)
KLASSEN = {
    0x01: "GeoCurve", 0x02: "ActiveObject/DrawingProperties", 0x05: "EmbroideryCurve", 0x06: "EmbroideryPatch",
    0x07: "ComplexFillGroup", 0x08: "EmbroideryBoundGroup", 0x09: "ActiveObject", 0x0b: "ManualObject",
    0x0c: "InputCCurve", 0x18: "RunFill", 0x19: "RunFill(Dreifach)", 0x1b: "TatamiFill", 0x2f: "LetterObj",
    0x30: "OldAlphaText", 0x36: "EmbroideryEllipse", 0x37: "EmbroideryRing", 0x38: "EmbroideryStar",
    0x3d: "MotifFill", 0x49: "EmbroideryGroup", 0x4a: "RunFill", 0x71: "AutoStartEndObject",
    0x72: "AutoAppliqueGroup", 0x73: "AppliqueCurve", 0x74: "EdgeZigZagFill", 0x7a: "MotifText",
    0x7b: "LetterText", 0x93: "Applique", 0x95: "ButtonHole", 0x96: "StemStitchFill", 0x97: "BackStitchFill",
}
# Parameterklasse -> Stil-Kennung (Fabrik setzt sie in +0xc)
PARAM_TAG = {0x13: 0x2713, 0x14: 0x272a, 0x16: 0x2721, 0x17: 0x2724, 0x18: 0x271c, 0x19: 0x271f, 0x1a: 0x2727,
             0x1b: 0x2714, 0x1d: 0x2744, 0x1e: 0x2745, 0x1f: 0x2746, 0x20: 0x2747, 0x33: 0x277e, 0x65: 0x275a}
OBJEKT_KLASSEN = {0x05, 0x06, 0x07, 0x08, 0x0b, 0x0c, 0x2f, 0x30, 0x36, 0x37, 0x38, 0x49, 0x71, 0x72, 0x73, 0x7a,
                  0x7b, 0x93, 0x95}


def felder(k, roh):
    """Nutzdaten einer Parameterklasse in Felder zerlegen (Feldfolge aus den Lesefunktionen)."""
    try:
        if k == 0x18 or k == 0x19:   # Lauf: 0x271d, 0x271e, 0x2768, 0x275e, 0x2767 (+ double ab neuerer Version)
            if len(roh) == 14:
                L, var, gap, n, mn = struct.unpack("<IbIbI", roh)
            else:
                L, var, gap, _dbl, n, mn = struct.unpack("<IbIdbI", roh[:22])
            return {"laenge": L, "variabel": var, "sehnenabstand": gap, "durchgaenge": n, "min": mn}
        if k == 0x16:                # Satin: Auto-Split 0x2722, Auto Spacing (0x275e?)
            return {"auto_split": struct.unpack_from("<I", roh)[0], "auto_spacing": roh[4] if len(roh) > 4 else None}
        if k == 0x1b:                # Tatami: 0x2717 Länge, 0x2718 Mindeststich, 0x2719/0x271a/0x271b Versatz, 2 Schalter
            L, mn, v1, v2, v3 = struct.unpack_from("<IIddd", roh)
            return {"laenge": L, "min_rand": mn, "versatz": (v1, v2, v3), "schalter": tuple(roh[32:34])}
        if k == 0x15:
            return {"abstand": struct.unpack_from("<I", roh)[0]}
        if k == 0x12:                # gemeinsame Stichparameter: 0x2711 Höchststich, 0x2712 Sprunglänge, …
            a, b, c = struct.unpack_from("<III", roh)
            return {"max_stich": a, "sprung": b, "feld3": c, "schalter": tuple(roh[12:])}
        if k == 0x10:                # Unterlage: long, Winkel (double), drei long (Rand/Zugausgleich, Deutung offen)
            a, w, b, c, d = struct.unpack_from("<IdIII", roh)
            return {"f1": a, "winkel": w, "f3": b, "f4": c, "f5": d}
        if k == 0x14:                # Verbinder: 5 long + 5 ushort (Zuordnung offen)
            return {"long": struct.unpack_from("<5I", roh), "short": struct.unpack_from("<5H", roh, 20)}
        if k == 0x0d:
            return {"tag": struct.unpack_from("<H", roh)[0]}
        if k == 0x0e:
            return {"anzahl": roh[0]}
        if k in (0x17, 0x1a):
            return {"long": struct.unpack_from(f"<{len(roh) // 4}I", roh)}
    except struct.error:
        pass
    return {"roh": roh.hex()}


def objekte(d, h):
    """Objektabschnitt flach lesen und in Objekte gruppieren."""
    start, ende = h[14], h[14] + h[15]
    i = start
    geteilt = []          # Positionen (in folge) der Datensätze mit Bit a, in Lesereihenfolge (Ziel von Verweisen)
    folge = []            # (offset, klasse, verweis, nutzdaten | Basiskette des Verweisziels)
    while i < ende:
        k, verw, l, a, b, j = datensatz(d, i)
        if (k > 0x9d and k != 0x2a2) or (k and not verw and l > ende - j):
            break                                     # Klasse mit eigenem Rohdatenteil: Rahmen verlassen
        if k == 0:
            folge.append((i, 0, False, b""))
            i = j
            continue
        if verw:
            ziel = None
            if 0 < l <= len(geteilt):              # Ziel samt Basisklassen bis zur Kennung 0x0d
                n = geteilt[l - 1]
                ziel = [folge[n]]
                while folge[n][1] not in (0, 0x0d) and n + 1 < len(folge):
                    n += 1
                    ziel.append(folge[n])
            folge.append((i, k, True, ziel))
            i = j
            continue
        roh = d[j:j + l]
        eintrag = (i, k, False, roh)
        if a:
            geteilt.append(len(folge))
        folge.append(eintrag)
        i = j + l
    # Objektliste beginnt mit Klasse 0x0a (Anzahl)
    try:
        pos = next(n for n, e in enumerate(folge) if e[1] == 0x0a and not e[2])
    except StopIteration:
        return [], folge
    anzahl = struct.unpack_from("<I", folge[pos][3])[0] if len(folge[pos][3]) >= 4 else 0
    objs, aktuell = [], None
    for e in folge[pos + 1:]:
        off, k, verw, inhalt = e
        if k in (0x85, 0x41, 0x43, 0x3a) and not verw and len(objs) >= anzahl:
            break                                     # Ende der Objektliste (Garnbibliothek usw.)
        if k in OBJEKT_KLASSEN and not verw:
            aktuell = {"offset": off, "klasse": KLASSEN.get(k, hex(k)), "datensaetze": []}
            objs.append(aktuell)
        if aktuell is not None:
            aktuell["datensaetze"].append(e)
    return objs, folge


def objekt_auswerten(o):
    """Stichart, Parameter, Unterlage, Formen eines Objekts zusammenfassen."""
    res = {"klasse": o["klasse"], "offset": o["offset"], "formen": [], "params": []}
    eintraege = []

    def aufloesen(liste, tiefe=0):
        for e in liste:
            if e[2]:
                if tiefe < 8:
                    aufloesen(e[3] or [], tiefe + 1)
            else:
                eintraege.append(e)
    aufloesen(o["datensaetze"])
    for off, k, verw, inhalt in eintraege:
        if k == 0x01:
            try:
                f, i = packed(inhalt, 0)
                kn, _ = knoten_lesen(inhalt, i)
            except IndexError:                        # leere Form (Länge 0)
                continue
            res["formen"].append({"geschlossen": bool(f & 1), "knoten": kn})
        elif k in PARAM_TAG or k in (0x10, 0x12, 0x15, 0x0d, 0x0e):
            res["params"].append((k, PARAM_TAG.get(k), felder(k, inhalt)))
    return res


if __name__ == "__main__":
    d = design_daten(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else "twofish_cfb1")
    h = kopf(d)
    objs, folge = objekte(d, h)
    print(f"{len(objs)} Objekte")
    for n, o in enumerate(objs):
        r = objekt_auswerten(o)
        arten = [f"0x{t:04x}" for k, t, f in r["params"] if t]
        print(f"#{n} {r['klasse']} @{r['offset']:#x}: Formen {[len(f['knoten']) for f in r['formen']]}, Stichparameter {arten}")
        for k, t, f in r["params"]:
            if k in (0x16, 0x18, 0x19, 0x1b, 0x15, 0x10, 0x1f, 0x1d, 0x1e, 0x20, 0x65):
                print(f"      0x{k:02x} {('0x%04x' % t) if t else '':7} {f}")
