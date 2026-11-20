"""Ein- und Ausstich der Objekte in den ART-Musterdesigns messen (Analyse_Teil3/16, ROADMAP 4b.5).

Fragen:
  paare     Liegen Ausstich eines Objekts und Einstich des nächsten auf den nächsten Punkten der beiden Umrisse
            („closest join“, MOD1 ResequenceByClosestJoin / GEOMETRY FindExitEntry)?
  lauf      Laufstich-Linien: offen vom ersten zum letzten Knoten? geschlossen: Start = Ende, am ersten Knoten?
  flaechen  Flächen (ComplexFillGroup mit Tatami): Ein-/Ausstich am ersten Knoten bzw. auf dem Rand?
  farben    Farbwechsel: wie oft kehrt eine Farbe wieder (nicht nach Farbe sortiert)?

Einstich = erster Stich des Objekts (der Endpunkt des Verbinders, z. B. der erste Knoten einer Linie, steht in den
Stichdaten nicht eigens – der erste Stich liegt eine Stichlänge weiter; deshalb Toleranz `TOL` = 3 mm, etwas über der
Lauflänge 2,5 mm). Ausstich = letzter Stich, beim Vernähen am Ende (letzter Stich kehrt auf den vorvorletzten zurück)
der vorletzte.

Aufruf:  python art_reihenfolge.py paare|lauf|flaechen --alle|DESIGN.ART [TWOFISH_EXE]
"""
import bisect
import collections
import glob
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from art_design import objekt_auswerten  # noqa: E402
from art_messen import MUSTER, _umriss, funktions_verweise, laden  # noqa: E402

SCHRIFT = ("LetterObj", "MotifText", "LetterText")
TOL = 3.0


def _d_seg(q, a, b):
    vx, vy = b[0] - a[0], b[1] - a[1]
    l2 = vx * vx + vy * vy
    t = 0 if l2 == 0 else max(0, min(1, ((q[0] - a[0]) * vx + (q[1] - a[1]) * vy) / l2))
    return math.dist(q, (a[0] + t * vx, a[1] + t * vy))


def _d_umriss(q, ringe):
    return min((_d_seg(q, a, b) for r in ringe for a, b in zip(r, r[1:])), default=math.inf)


def _min_abstand(ra, rb):
    """Kleinster Abstand zweier Umrisse: nächstes Punktpaar (nach x sortiert), dann Strecken in der Nähe."""
    pb = sorted(q for r in rb for q in r)
    xs = [q[0] for q in pb]
    best = math.inf
    for q in (q for r in ra for q in r):
        i = bisect.bisect_left(xs, q[0] - best)
        while i < len(pb) and pb[i][0] <= q[0] + best:
            best = min(best, math.dist(q, pb[i]))
            i += 1
    # Feinabgleich gegen die Strecken (Punktabstand der Abtastung ist gröber als die Strecken)
    grenze = best + 0.01
    fein = best
    for x, y in ((ra, rb), (rb, ra)):
        segs = [(a, b) for r in y for a, b in zip(r, r[1:])]
        for q in (q for r in x for q in r):
            for a, b in segs:
                if min(math.dist(q, a), math.dist(q, b)) <= grenze + math.dist(a, b):
                    fein = min(fein, _d_seg(q, a, b))
    return fein


def _ausstich(p):
    """Letzter Stich ohne Vernähen: kehrt der letzte Stich auf den vorvorletzten zurück, gilt der vorletzte."""
    if len(p) >= 3 and math.dist(p[-1], p[-3]) < 0.3:
        return p[-2]
    return p[-1]


def objekte_mit_stichen(pfade, twofish):
    """Je Design: Liste (Objekt, Auswertung, Ringe in mm, Einstich, Stichpunkte, Stichfunktionen)."""
    for pfad in pfade:
        try:
            d, h, pts, segs, objs = laden(pfad, twofish)
        except Exception as e:  # noqa: BLE001
            print("FEHLER", os.path.basename(pfad), e, file=sys.stderr)
            continue
        # wie art_messen.funktionen_je_objekt, dazu der Punkt vor dem ersten Stich (Einstich)
        fo, idx = [], 0
        for s in segs:
            m = s["stiche"]
            teil = pts[idx:idx + m + 1]
            if s["art"] == 0:
                if s["feld3c"] == 0 or not fo:
                    fo.append({"einstich": None, "punkte": []})
                fo[-1]["punkte"].extend(teil[1:])
            elif fo:
                fo[-1]["punkte"].extend(teil[1:] if fo[-1]["punkte"] else teil)
            idx += m
        if len(fo) != len(objs):
            continue
        liste = []
        for o, f in zip(objs, fo):
            r = objekt_auswerten(o)
            ringe = [_umriss(fm, 8) for fm in r["formen"] if len(fm["knoten"]) >= 2]
            liste.append((o, r, ringe, f["punkte"][0] if f["punkte"] else None, f["punkte"], [e[1] for e in funktions_verweise(o)]))
        yield pfad, liste


def _zeile(name, werte):
    if not werte:
        print(f"{name}: keine")
        return
    w = sorted(werte)
    print(f"{name}: n={len(w)}, Median {w[len(w) // 2]:.2f}, 10 % {w[len(w) // 10]:.2f}, 90 % {w[9 * len(w) // 10]:.2f}")


def paare(pfade, tw):
    """Ausstich A → Einstich B gegen kleinsten Umrissabstand A–B (nur getrennte Umrisse)."""
    stat = collections.Counter()
    ueber = []
    for _, liste in objekte_mit_stichen(pfade, tw):
        for (oa, _, ga, _, pa, _), (ob, _, gb, eb, pb, _) in zip(liste, liste[1:]):
            if not (ga and gb and pa and pb and eb) or oa["klasse"] in SCHRIFT or ob["klasse"] in SCHRIFT:
                continue
            aus = _ausstich(pa)
            # nur Paare, deren Stiche zu ihren Formen passen (Ausstich auf dem Umriss von A, Einstich höchstens eine
            # Stichlänge vom Umriss von B) – sonst liegen Form und Stiche in verschiedenen Lagen (Gruppen u. a.)
            if _d_umriss(aus, ga) > 0.5 or _d_umriss(eb, gb) > TOL:
                stat["Ausstich/Einstich nicht auf dem Umriss (nicht gewertet)"] += 1
                continue
            d = math.dist(aus, eb)
            dmin = _min_abstand(ga, gb)
            if dmin < 0.3:
                stat["Umrisse berühren sich (< 0,3 mm)"] += 1
                continue
            stat["getrennt (≥ 0,3 mm)"] += 1
            ueber.append(d - dmin)
            if d <= dmin + TOL:
                stat["  Verbindung ≤ kleinster Abstand + 3 mm (closest join)"] += 1
            elif d <= 2 * dmin + TOL:
                stat["  Verbindung ≤ 2 × kleinster Abstand + 3 mm"] += 1
            else:
                stat["  Verbindung deutlich länger"] += 1
    for k, v in stat.items():
        print(f"{k}: {v}")
    _zeile("Verbindung minus kleinster Abstand (mm), getrennte Paare", ueber)


def _k(knoten):
    return knoten["punkt"][0] / 180, knoten["punkt"][1] / 180


def lauf(pfade, tw):
    """Lauf-Linien (EmbroideryCurve, nur Lauf/Dreifach)."""
    stat = collections.Counter()
    se, k0 = [], []
    for _, liste in objekte_mit_stichen(pfade, tw):
        for o, r, ringe, ein, pkt, fk in liste:
            if o["klasse"] != "EmbroideryCurve" or not fk or any(k not in (0x18, 0x19) for k in fk) or not ein:
                continue
            formen = [f for f in r["formen"] if len(f["knoten"]) >= 2]
            if len(formen) != 1 or len(pkt) < 4:
                continue
            fm = formen[0]
            erster, letzter = _k(fm["knoten"][0]), _k(fm["knoten"][-1])
            aus = _ausstich(pkt)
            if fm["geschlossen"]:
                stat["geschlossen"] += 1
                se.append(math.dist(ein, aus))
                k0.append(math.dist(ein, erster))
                stat["geschlossen: Ausstich = Einstich (< 3 mm)" if se[-1] < TOL else "geschlossen: Ausstich ≠ Einstich"] += 1
                stat["geschlossen: Einstich am ersten Knoten (< 3 mm)" if k0[-1] < TOL else
                     "geschlossen: Einstich anderswo"] += 1
            else:
                stat["offen"] += 1
                ok = math.dist(ein, erster) < TOL and math.dist(aus, letzter) < TOL
                um = math.dist(ein, letzter) < TOL and math.dist(aus, erster) < TOL
                stat["offen: vom ersten zum letzten Knoten" if ok else "offen: vom letzten zum ersten Knoten" if um
                     else "offen: anders"] += 1
    for k, v in sorted(stat.items()):
        print(f"{k}: {v}")
    _zeile("geschlossen: Abstand Einstich–Ausstich (mm)", se)
    _zeile("geschlossen: Abstand Einstich – erster Knoten (mm)", k0)


def flaechen(pfade, tw):
    """Flächen mit Tatami: Ein-/Ausstich am ersten Knoten bzw. auf dem Rand?"""
    stat = collections.Counter()
    for _, liste in objekte_mit_stichen(pfade, tw):
        for o, r, ringe, ein, pkt, fk in liste:
            if 0x1b not in fk or not ringe or len(pkt) < 4 or not ein:
                continue
            fm = next(f for f in r["formen"] if len(f["knoten"]) >= 2)
            erster = _k(fm["knoten"][0])
            aus = _ausstich(pkt)
            stat["Flächen"] += 1
            stat["Einstich am ersten Knoten (< 3 mm)" if math.dist(ein, erster) < TOL else "Einstich anderswo"] += 1
            stat["Ausstich am ersten Knoten (< 3 mm)" if math.dist(aus, erster) < TOL else "Ausstich anderswo"] += 1
            stat["Ausstich = Einstich (< 3 mm)" if math.dist(aus, ein) < TOL else "Ausstich ≠ Einstich"] += 1
            stat["Einstich auf dem Rand (< 0,3 mm)" if _d_umriss(ein, ringe) < 0.3 else "Einstich im Inneren"] += 1
            stat["Ausstich auf dem Rand (< 0,3 mm)" if _d_umriss(aus, ringe) < 0.3 else "Ausstich im Inneren"] += 1
    for k, v in sorted(stat.items()):
        print(f"{k}: {v}")


if __name__ == "__main__":
    modus, ziel = sys.argv[1], sys.argv[2]
    tw = sys.argv[3] if len(sys.argv) > 3 else "twofish_cfb1"
    pfade = sorted({os.path.normcase(p): p for p in glob.glob(MUSTER)}.values()) if ziel == "--alle" else [ziel]
    {"paare": paare, "lauf": lauf, "flaechen": flaechen}[modus](pfade, tw)
