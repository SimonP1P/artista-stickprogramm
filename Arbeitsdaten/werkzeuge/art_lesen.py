"""ART-Design (BERNINA artista V4 / Wilcom) lesen: Streams auflisten, Contents und DesignDocument entschlüsseln,
DataItem-Bäume als Text ausgeben.

Aufruf:  python art_lesen.py DESIGN.ART AUSGABEORDNER [TWOFISH_EXE]
  - Contents:        Substitution (Format-Schlüssel 0xD0), siehe Analyse/code/05
  - DesignDocument:  Twofish-128-CFB1, Schlüssel 1d43f2ae01c0d9320000000000000000 (twofish_cfb1.c)
Ergebnis: <name>.contents.bin/.txt und <name>.designdocument.bin/.txt im Ausgabeordner.
"""
import os
import struct
import subprocess
import sys
import tempfile
import zlib

import olefile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_decrypt import try_all  # noqa: E402
from dataitem import dump, parse  # noqa: E402

ART_SCHLUESSEL = "1d43f2ae01c0d9320000000000000000"
IV = "a885f60d87e90b71c20a81bcc3498af1"


def baeume(daten, start=4):
    """Alle DataItem-Bäume ab `start` hintereinander parsen (bis ein Parse-Fehler kommt)."""
    out, o = [], start
    while o + 12 <= len(daten):
        try:
            n, o2 = parse(daten, o)
        except Exception:
            break
        out.append(n)
        if o2 <= o:
            break
        o = o2
    return out, o


def designdocument(roh, twofish):
    with tempfile.TemporaryDirectory() as t:
        ein, aus = os.path.join(t, "ein"), os.path.join(t, "aus")
        open(ein, "wb").write(roh)
        subprocess.run([twofish, ART_SCHLUESSEL, IV, ein, aus], check=True)
        d = open(aus, "rb").read()
    groesse = struct.unpack_from("<I", d, 0)[0]
    klar = zlib.decompress(d[4:])
    if len(klar) != groesse:
        raise ValueError(f"Größe {len(klar)} statt {groesse}")
    return klar


def main():
    art, ziel = sys.argv[1], sys.argv[2]
    twofish = sys.argv[3] if len(sys.argv) > 3 else "twofish_cfb1"
    os.makedirs(ziel, exist_ok=True)
    name = os.path.splitext(os.path.basename(art))[0]
    ole = olefile.OleFileIO(art)
    print("Streams:", ["/".join(s) for s in ole.listdir()])
    if ole.exists("Contents"):
        r = try_all(ole.openstream("Contents").read())
        if r:
            klar = r[2]
            open(os.path.join(ziel, name + ".contents.bin"), "wb").write(klar)
            with open(os.path.join(ziel, name + ".contents.txt"), "w", encoding="utf-8") as f:
                b, ende = baeume(klar, 0)
                for n in b:
                    dump(n, out=f)
                print(f"Contents: {len(klar)} Byte, {len(b)} Bäume, gelesen bis {ende}")
    if ole.exists("DesignDocument"):
        klar = designdocument(ole.openstream("DesignDocument").read(), twofish)
        open(os.path.join(ziel, name + ".designdocument.bin"), "wb").write(klar)
        with open(os.path.join(ziel, name + ".designdocument.txt"), "w", encoding="utf-8") as f:
            b, ende = baeume(klar, 0)
            for n in b:
                dump(n, out=f)
            print(f"DesignDocument: {len(klar)} Byte, {len(b)} Bäume, gelesen bis {ende}")


if __name__ == "__main__":
    main()
