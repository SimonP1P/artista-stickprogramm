"""Verschleierung der BERNINA-Designcontainer ("BE"-Container) aus BEAGLink.dll.

Rekonstruiert aus:
  0x1000a370  Schlüssel-Initialisierung (Startzustand 8 Byte, beginnt mit 'B','E')
  0x1000a4c0  Byte-weise Ver-/Entschlüsselung (rollierendes XOR über 8-Byte-Zustand)
  0x1000a640  ganzen Container ab Offset 8 umkodieren (mode=1: verschlüsseln, mode=0: entschlüsseln)

Container-Aufbau: Byte 0..1 (u16), Byte 2..5 = u32 Gesamtlänge L (Little-Endian), Byte 6..7 (u16),
ab Byte 8 verschleiert: Folge von Chunks {u16 id, u32 länge_inkl_6_byte_kopf, daten}.
"""
import struct

def init_state(total_len):
    n = total_len - 8                       # Anzahl verschleierter Bytes
    a = n % 255                             # bl  (idiv 0xff)
    b = (n >> 8) % 255                      # al / [esp+0x18]
    s = [0] * 8
    s[0], s[1] = 0x42, 0x45                 # 'B', 'E'
    if n <= 0xfff:
        s[4] = a ^ 0x76
        s[2] = (a ^ 0x76) ^ 0xb3
        s[3] = 0xb3
        s[5] = ((a ^ 0x0d) << 2) & 0xff
        s[6] = ((a << 4) & 0xff) ^ 0x73
        s[7] = ((a << 5) & 0xff) ^ 0xd5
    elif n <= 0x3fff:
        s[4] = b ^ 0xea
        s[7] = ((a << 1) & 0xff) ^ 0xc9
        s[6] = ((a << 6) & 0xff) ^ 0x30
        s[5] = a ^ 0x56
        s[3] = 0x24
        s[2] = (b ^ 0xea) ^ (a ^ 0x56) ^ 0x24
    elif n <= 0xcfff:
        s[7] = ((a << 1) & 0xff) ^ 0xc9
        s[4] = b ^ 0x4c
        s[6] = a ^ 0x27
        s[5] = a ^ 0xf3
        s[3] = 0xd1
        s[2] = (b ^ 0x4c) ^ (a ^ 0xf3) ^ (a ^ 0x27) ^ 0xd1
    else:
        s[5] = a ^ 0x11
        s[6] = a ^ 0x89
        s[7] = a ^ 0x52
        s[4] = b ^ 0x6a
        s[3] = 0x38
        s[2] = (b ^ 0x6a) ^ (a ^ 0x11) ^ (a ^ 0x89) ^ (a ^ 0x52) ^ 0x38
    s = [(v ^ (i << 3)) & 0xff for i, v in enumerate(s)]
    idx = (n >> 1) & 7
    return s, idx

def transform(buf, encrypt):
    """buf: kompletter Container (bytes). Liefert neuen Container; Bytes 0..7 bleiben unverändert."""
    buf = bytearray(buf)
    L = struct.unpack_from('<I', buf, 2)[0]
    s, i = init_state(L)
    for p in range(8, L):
        inp = buf[p]
        k = s[(i + 2) & 7] ^ s[(i - 1) & 7] ^ s[(i + 1) & 7]
        out = inp ^ k
        s[i] = out if encrypt else inp      # Zustand nimmt immer den Chiffretext auf
        buf[p] = out
        i = (i - 3) & 7
    return bytes(buf)

def chunks(plain):
    """Zerlegt einen entschlüsselten Container in (id, daten)."""
    L = struct.unpack_from('<I', plain, 2)[0]; o = 8; out = []
    while o + 6 <= L:
        cid, ln = struct.unpack_from('<HI', plain, o)
        if ln <= 6 or o + ln > L: break
        out.append((cid, plain[o + 6:o + ln])); o += ln
    return out

if __name__ == '__main__':
    import os
    for L in (40, 3000, 9000, 40000, 70000):
        body = os.urandom(L - 8)
        c = struct.pack('<HIH', 0, L, 0) + body
        enc = transform(c, True); dec = transform(enc, False)
        assert dec == c and enc != c
    print('Selbsttest Hin/Rück ok (alle 4 Längenbereiche)')
