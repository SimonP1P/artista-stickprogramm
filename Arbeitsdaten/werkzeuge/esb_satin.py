"""Satinsäule eines ESB-Buchstabens (Umriss + Winkellinien) in zwei Seiten A/B mit Punkt-zu-Punkt-Zuordnung umwandeln.

Grundlage: Analyse_Teil3/09 §2. Winkellinie k verbindet P_k = Punkt(Segment a_k, Anteil x_k) mit
Q_k = Punkt(Segment c_k, Anteil y_k). Seite A läuft über den Umriss von P_0 nach P_n (Richtung der a-Werte),
Seite B in Gegenrichtung von Q_0 nach Q_n. Zwischen zwei Winkellinien werden beide Teilwege mit gleich vielen
Punkten nach relativer Bogenlänge abgetastet → A[i] gehört zu B[i] (wie die Satinsäule des Rechenkerns erwartet).

Aufruf (Vergleich mit dem Maschinen-Design):  python esb_satin.py ALPHABET.ESB ZEICHEN MASSSTAB [--png AUS.png]
"""
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from esb_lesen import segment_punkt  # noqa: E402


def _umriss_stuecke(form, n=16):
    """Feine Punktliste des geschlossenen Umrisses mit Parameter pos = Segment + t."""
    kn = form['knoten']
    nseg = len(kn) if form['geschlossen'] else len(kn) - 1
    pts = []
    for s in range(nseg):
        for i in range(n):
            t = i / n
            pts.append((s + t, segment_punkt(form, s, t)))
    return pts, nseg


def _weg(form, von, bis, richtung, n=16):
    """Punkte des Umrisses von Parameter `von` nach `bis` in Richtung +1/-1 (über das Ende hinweg)."""
    kn = form['knoten']
    nseg = len(kn) if form['geschlossen'] else len(kn) - 1
    if richtung > 0:
        d = (bis - von) % nseg if form['geschlossen'] else bis - von
    else:
        d = (von - bis) % nseg if form['geschlossen'] else von - bis
    schritte = max(1, int(math.ceil(abs(d) * n)))
    out = []
    for i in range(schritte + 1):
        p = von + richtung * d * i / schritte
        if form['geschlossen']:
            p %= nseg
        s = min(int(p), nseg - 1)
        out.append(segment_punkt(form, s, p - s))
    return out


def _kum(linie):
    k = [0.0]
    for a, b in zip(linie, linie[1:]):
        k.append(k[-1] + math.hypot(b[0] - a[0], b[1] - a[1]))
    return k


def _bei(linie, kum, s):
    if s <= 0:
        return linie[0]
    for i in range(1, len(linie)):
        if kum[i] >= s:
            t = (s - kum[i - 1]) / (kum[i] - kum[i - 1]) if kum[i] > kum[i - 1] else 0
            a, b = linie[i - 1], linie[i]
            return (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)
    return linie[-1]


def satin_seiten(objekt, punkte_je_mm=1.0, einheit=180.0):
    """EmbroideryPatch → (Seite A, Seite B) in mm. Gleich viele Punkte je Abschnitt zwischen zwei Winkellinien."""
    form = objekt['formen'][0]
    w = objekt['winkel']
    kn = form['knoten']
    nseg = len(kn) if form['geschlossen'] else len(kn) - 1
    pos_a = [a + x for a, x, c, y in w]
    pos_b = [c + y for a, x, c, y in w]
    # Richtung von Seite A aus den ersten beiden Linien; B läuft entgegengesetzt
    d = (pos_a[1] - pos_a[0]) % nseg if len(w) > 1 else 1
    ra = 1 if d <= nseg / 2 else -1
    A, B = [], []
    for k in range(len(w) - 1):
        wa = _weg(form, pos_a[k], pos_a[k + 1], ra)
        wb = _weg(form, pos_b[k], pos_b[k + 1], -ra)
        ka, kb = _kum(wa), _kum(wb)
        laenge = max(ka[-1], kb[-1]) / einheit
        n = max(1, int(math.ceil(laenge * punkte_je_mm)))
        for i in range(n + (1 if k == len(w) - 2 else 0)):
            t = i / n
            pa, pb = _bei(wa, ka, t * ka[-1]), _bei(wb, kb, t * kb[-1])
            A.append((pa[0] / einheit, pa[1] / einheit))
            B.append((pb[0] / einheit, pb[1] / einheit))
    return A, B


if __name__ == '__main__':
    import argparse
    import esb_lesen as e
    sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'stickprogramm', 'src'))
    from stickprogramm.einstellungen import SatinEinstellung
    from stickprogramm.sticharten.satin import satin
    ap = argparse.ArgumentParser()
    ap.add_argument('esb'); ap.add_argument('zeichen'); ap.add_argument('massstab', type=float)
    a = ap.parse_args()
    alpha = e.alphabet_lesen(a.esb)
    g = alpha['zeichen']['%04X' % ord(a.zeichen)]
    gl = g['grundlinie']
    for o in g['objekte']:
        if o['klasse_nr'] != 0x06:
            continue
        A, B = satin_seiten(o)
        A = [((p[0] - gl[0] / 180) * a.massstab, (p[1] - gl[1] / 180) * a.massstab) for p in A]
        B = [((p[0] - gl[0] / 180) * a.massstab, (p[1] - gl[1] / 180) * a.massstab) for p in B]
        st = satin(A, B, SatinEinstellung(auto_spacing=False, abstand_fest=0.40))
        print(f'Säule: {len(A)} Seitenpunkte, {len(st)} Einstiche, A {A[0]} -> {A[-1]}, B {B[0]} -> {B[-1]}')
