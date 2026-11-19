"""Stichpunkte aus ART-Designs (BERNINA artista V4 / Wilcom) dekodieren.

Nachbau von IMPORT.DLL 0x10037330 (Stichleser, Abschnitt 1 des Designdatenblocks) und WESSYS.DLL
ReadBytePacked/X_ReadBytePacked (0x10029de0/0x1002a140). Kopf des Datenblocks: IMPORT.DLL 0x10078b10
(0x74 Byte, Magic 0x193866de, Prüfsumme an 0x70 = Summe der ersten 28 DWORDs).

Kodierung je Punkt: 1 Byte, unteres Halbbyte = x-Code, oberes = y-Code; ist Bit 3 eines Codes gesetzt, folgt ein
gepacktes Byte (7 Bit je Byte, Bit 7 = Fortsetzung), dessen Wert << 4 dazukommt.
  Code < 8:   ZigZag-Delta der Schrittweite: d = c>>1 (bei c&1: ~d); Schritt = alter Schritt + d.
  Code >= 8:  Bit 3 entfernen -> c'; Betragsänderung m = c'>>2 (bei c'&2: ~m); Betrag += m; c'&1 kehrt das Vorzeichen um.
Ergebnis: absolute Punkte (x, y) in Wilcom-Einheiten (vermutlich 1 mm = 180 bzw. Designeinheit, siehe Analyse_Teil3/02).

Aufruf:  python art_stiche.py DESIGN.ART [TWOFISH_EXE]   -> Kopf, Anzahl, Begrenzung, erste Punkte
"""
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))


def packed(d, i):
    """WESSYS ReadBytePacked (vorzeichenlos): 7 Bit je Byte, Bit 7 = weiteres Byte folgt."""
    b = d[i]
    i += 1
    v = b
    shift = 7
    while b & 0x80:
        b = d[i]
        i += 1
        v = (v ^ (1 << shift)) | (b << shift)
        shift += 7
    return v, i


class Achse:
    """Zustand einer Achse im Stichleser (Position, Schritt, Betrag, Vorzeichen)."""

    def __init__(self):
        self.pos = 0
        self.schritt = 0
        self.betrag = 0
        self.vorzeichen = 1

    def anwenden(self, c):
        if c < 8:
            d = c >> 1
            if c & 1:
                d = ~d
            s = self.schritt + d
            if s >= 0:
                self.betrag, self.vorzeichen = s, 1
            else:
                self.betrag, self.vorzeichen = ~s, -1
        else:
            c2 = (c & 7) | ((c >> 1) & 0x7FFFFFF8)
            m = c2 >> 2
            if c2 & 2:
                m = ~m
            self.betrag = m + self.betrag
            if c2 & 1:
                self.vorzeichen = -self.vorzeichen
            s = self.betrag if self.vorzeichen != -1 else ~self.betrag
        self.schritt = s
        self.pos += s
        return self.pos


def stiche_dekodieren(d, start, anzahl):
    """Abschnitt 1 ab `start` lesen: Liste absoluter Punkte (x, y) und Endoffset."""
    ax, ay = Achse(), Achse()
    punkte = []
    i = start
    for _ in range(anzahl):
        b = d[i]
        i += 1
        cx, cy = b & 0xF, b >> 4
        if cx & 8:
            v, i = packed(d, i)
            cx |= v << 4
        if cy & 8:
            v, i = packed(d, i)
            cy |= v << 4
        punkte.append((ax.anwenden(cx), ay.anwenden(cy)))
    return punkte, i


def kopf(d):
    """Kopf des Designdatenblocks als Liste von 29 DWORDs (geprüft: Magic und Prüfsumme)."""
    h = struct.unpack_from("<29I", d, 0)
    if h[0] != 0x193866DE:
        raise ValueError("kein Designdatenblock (Magic fehlt)")
    if sum(h[:28]) & 0xFFFFFFFF != h[28]:
        raise ValueError("Prüfsumme des Kopfes falsch")
    return h


def design_daten(art, twofish="twofish_cfb1"):
    """Klartext des Designdatenblocks: DesignDocument (neuere ART) oder Contents (ältere ART)."""
    import olefile
    from art_decrypt import try_all
    from art_lesen import designdocument
    ole = olefile.OleFileIO(art)
    if ole.exists("DesignDocument"):
        return designdocument(ole.openstream("DesignDocument").read(), twofish)
    return try_all(ole.openstream("Contents").read())[2]


if __name__ == "__main__":
    d = design_daten(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else "twofish_cfb1")
    h = kopf(d)
    print("Kopf:", " ".join(f"{x:x}" for x in h))
    pts, ende = stiche_dekodieren(d, h[6], h[5])
    xs, ys = [p[0] for p in pts], [p[1] for p in pts]
    print(f"{len(pts)} Punkte, gelesen bis {ende:#x} (Abschnitt endet bei {h[6] + h[7]:#x})")
    print(f"x {min(xs)} … {max(xs)}, y {min(ys)} … {max(ys)}")
    print("erste Punkte:", pts[:12])
