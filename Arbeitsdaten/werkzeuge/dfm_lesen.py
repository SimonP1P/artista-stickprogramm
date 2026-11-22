"""Formulare (Delphi/C++Builder-DFM, Kennung „TPF0“) aus einer EXE/DLL lesen und als Text ausgeben – für die
Kreuzstich-Programmteile BLOT.EXE / XSLang.DLL (Analyse_Teil3/29). Ohne pefile: sucht „TPF0“ direkt in den Bytes.

Aufruf:  python3 -I dfm_lesen.py DATEI [FORMULARNAME …]
"""
import struct
import sys


class Leser:
    def __init__(self, d, o):
        self.d, self.o = d, o

    def b(self):
        v = self.d[self.o]
        self.o += 1
        return v

    def s(self):
        n = self.b()
        v = self.d[self.o:self.o + n].decode('latin1')
        self.o += n
        return v

    def wert(self):
        t = self.b()
        d, o = self.d, self.o
        if t == 0:
            return None
        if t == 1:
            liste = []
            while self.d[self.o] != 0:
                liste.append(self.wert())
            self.o += 1
            return liste
        if t == 2:
            self.o += 1
            return struct.unpack_from('<b', d, o)[0]
        if t == 3:
            self.o += 2
            return struct.unpack_from('<h', d, o)[0]
        if t == 4:
            self.o += 4
            return struct.unpack_from('<i', d, o)[0]
        if t == 5:                      # Extended (80 Bit)
            self.o += 10
            m, e = struct.unpack_from('<QH', d, o)
            s = -1 if e & 0x8000 else 1
            e &= 0x7fff
            return 0.0 if m == 0 and e == 0 else s * m / (1 << 63) * 2.0 ** (e - 16383)
        if t in (6, 7):
            return self.s()
        if t == 8:
            return False
        if t == 9:
            return True
        if t == 10:
            n = struct.unpack_from('<I', d, o)[0]
            self.o += 4 + n
            return f'<binär {n} Byte>'
        if t == 11:
            menge = []
            while True:
                x = self.s()
                if not x:
                    break
                menge.append(x)
            return set(menge)
        if t == 12:
            n = struct.unpack_from('<I', d, o)[0]
            self.o += 4 + n
            return d[o + 4:o + 4 + n].decode('latin1')
        if t == 13:
            return 'nil'
        if t == 14:
            eintraege = []
            while self.d[self.o] != 0:
                # je Eintrag: optional Reihenfolge (vaInt…), dann vaList, Eigenschaften, Listenende
                if self.d[self.o] in (2, 3, 4):
                    self.wert()
                if self.d[self.o] == 1:
                    self.o += 1
                props = {}
                while self.d[self.o] != 0:
                    k = self.s()
                    props[k] = self.wert()
                self.o += 1
                eintraege.append(props)
            self.o += 1
            return eintraege
        if t in (16, 17):               # Currency, Date (8 Byte)
            self.o += 8
            return struct.unpack_from('<q' if t == 16 else '<d', d, o)[0]
        if t == 15:
            self.o += 4
            return struct.unpack_from('<f', d, o)[0]
        if t == 18:
            n = struct.unpack_from('<I', d, o)[0]
            self.o += 4 + 2 * n
            return d[o + 4:o + 4 + 2 * n].decode('utf-16le')
        if t == 19:
            self.o += 8
            return struct.unpack_from('<q', d, o)[0]
        if t == 20:
            n = struct.unpack_from('<I', d, o)[0]
            self.o += 4 + n
            return d[o + 4:o + 4 + n].decode('utf-8', 'replace')
        raise ValueError(f'Werttyp {t} bei {o - 1:#x}')

    def objekt(self, tiefe, aus):
        if self.d[self.o] & 0xf0 == 0xf0:
            fl = self.b()
            if fl & 2:
                self.wert()
        klasse = self.s()
        name = self.s()
        aus.append('  ' * tiefe + f'{name}: {klasse}')
        while self.d[self.o] != 0:
            k = self.s()
            v = self.wert()
            if isinstance(v, str) and len(v) > 120:
                v = v[:120] + '…'
            aus.append('  ' * tiefe + f'  .{k} = {v!r}')
        self.o += 1
        while self.d[self.o] != 0:
            self.objekt(tiefe + 1, aus)
        self.o += 1


def formulare(d):
    o = 0
    while True:
        o = d.find(b'TPF0', o)
        if o < 0:
            return
        r = Leser(d, o + 4)
        aus = []
        try:
            r.objekt(0, aus)
            yield aus
        except (ValueError, IndexError, struct.error) as e:
            yield [f'?? Fehler bei {o:#x}: {e}']
        o += 4


if __name__ == '__main__':
    daten = open(sys.argv[1], 'rb').read()
    wahl = [x.lower() for x in sys.argv[2:]]
    for f in formulare(daten):
        if wahl and not any(f[0].lower().endswith(': ' + w) or f[0].lower().startswith(w) for w in wahl):
            continue
        print('\n'.join(f))
        print()
