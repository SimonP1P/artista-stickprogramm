"""Objektteil des Designdatenblocks (ART-Musterdesigns) als Baum von Datensätzen lesen.

Rahmen eines Datensatzes (IMPORT.DLL 0x100d9b90 / 0x100d9bf0):
  gepackt u:  Kennung = u >> 1, Bit 0 = Verweis
  Kennung 0:  Ende der Ebene
  Verweis:    gepackt v = Nummer eines früheren Objekts
  sonst:      gepackt v: Länge = v >> 2, Schalter a = v & 1, b = (v >> 1) & 1; danach `Länge` Byte Nutzdaten
Gepackt = WESSYS ReadBytePacked (7 Bit je Byte, Bit 7 = Fortsetzung).

Aufruf:  python art_objekte.py DESIGN.ART [TWOFISH_EXE] [START_HEX]
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_stiche import design_daten, kopf, packed  # noqa: E402


def datensatz(d, i):
    """Einen Datensatzkopf lesen: (kennung, verweis, laenge|ziel, a, b, nutzdaten_start)."""
    u, i = packed(d, i)
    kennung, verweis = u >> 1, u & 1
    if kennung == 0:
        return 0, 0, 0, 0, 0, i
    v, i = packed(d, i)
    if verweis:
        return kennung, 1, v, 0, 0, i
    return kennung, 0, v >> 2, v & 1, (v >> 1) & 1, i


def packed_wort_s(d, i):
    """WESSYS ReadWordPacked (vorzeichenbehaftet, ZigZag)."""
    from art_stiche import packed_wort
    v, i = packed_wort(d, i)
    return (~(v >> 1) if v & 1 else v >> 1), i


KEIN = None  # fehlender Anfasser (im Original 0x80000000)


def knoten_lesen(d, i):
    """IMPORT.DLL SerializeKnots (0x1000fc20): Liste von Knoten (ein, punkt, aus, art), Einheit 1/180 mm.

    Je Knoten Kennbyte: Bit 0 = Anfasser ein, Bit 1 = Anfasser aus, Bit 2 = Knotenart. Koordinaten als
    vorzeichenbehaftete gepackte Wörter relativ zum zuletzt gelesenen Punkt (ein, Punkt, aus).
    """
    n, i = packed(d, i)
    px = py = 0
    knoten = []
    for _ in range(n):
        c = d[i]
        i += 1
        ein = aus = KEIN
        if c & 1:
            dx, i = packed_wort_s(d, i)
            dy, i = packed_wort_s(d, i)
            px, py = px + dx, py + dy
            ein = (px, py)
        dx, i = packed_wort_s(d, i)
        dy, i = packed_wort_s(d, i)
        px, py = px + dx, py + dy
        punkt = (px, py)
        if c & 2:
            dx, i = packed_wort_s(d, i)
            dy, i = packed_wort_s(d, i)
            px, py = px + dx, py + dy
            aus = (px, py)
        knoten.append({"ein": ein, "punkt": punkt, "aus": aus, "art": (c >> 2) & 1})
    return knoten, i


def geocurve(d, start):
    """Klasse 0x01 (GeoCurve, IMPORT.DLL 0x1000fe20): gepackt (Bit 0 = geschlossen), dann Knoten."""
    f, i = packed(d, start)
    knoten, i = knoten_lesen(d, i)
    return {"geschlossen": bool(f & 1), "knoten": knoten}, i


def baum(d, start, ende, tiefe=0, aus=sys.stdout, max_tiefe=12):
    """Datensätze von start bis ende ausgeben; Nutzdaten werden versuchsweise als Unterebene gelesen."""
    i = start
    while i < ende:
        k, verweis, laenge, a, b, j = datensatz(d, i)
        if k == 0:
            print("  " * tiefe + f"[{i:#x}] Ende", file=aus)
            i = j
            continue
        if verweis:
            print("  " * tiefe + f"[{i:#x}] 0x{k:x} -> Verweis {laenge}", file=aus)
            i = j
            continue
        roh = d[j:j + laenge]
        print("  " * tiefe + f"[{i:#x}] 0x{k:x} a={a} b={b} len={laenge}: {roh[:24].hex()}", file=aus)
        if False:  # Nutzdaten sind keine Unterebenen (Schalter a = für Verweise nummeriert)
            baum(d, j, j + laenge, tiefe + 1, aus, max_tiefe)
        i = j + laenge


if __name__ == "__main__":
    d = design_daten(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else "twofish_cfb1")
    h = kopf(d)
    start = int(sys.argv[3], 16) if len(sys.argv) > 3 else h[14]
    ende = start + h[15] if len(sys.argv) <= 3 else len(d)
    baum(d, start, ende)
