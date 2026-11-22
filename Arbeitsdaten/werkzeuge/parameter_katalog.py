"""Parameterkatalog (ROADMAP M1) erzeugen: kern/stick-kern/daten/parameter.json.

Eine Zeile je Parameter. Der Rust-Test `stick_kern::parameter` prüft, dass jeder Wert unter `einstellungen` hier steht,
der Standard mit dem Code übereinstimmt und „stoffabhängig“ stimmt. Lesbare Fassung: `stick parameter --markdown`
(→ PARAMETER.md). Modus: einfach = im einfachen Modus sichtbar, experte = nur im Expertenmodus, automatisch = wird
nicht angezeigt (fester oder abgeleiteter Wert, dokumentiert).

Aufruf:  python parameter_katalog.py
"""
import json
import os

ZIEL = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'kern', 'stick-kern', 'daten', 'parameter.json')
K = []


def P(id_, gruppe, name, wirkung, *, ebene='design', gilt=(), d=None, o=None, einheit='', std=None, bereich=None,
      werte=(), stoff=False, quelle='', modus='experte'):
    assert modus in ('einfach', 'experte', 'automatisch'), id_
    assert ebene in ('design', 'objekt', 'beide', 'werkzeug'), id_
    e = {'id': id_, 'gruppe': gruppe, 'name': name, 'wirkung': wirkung, 'ebene': ebene, 'gilt_fuer': list(gilt),
         'json_design': d, 'json_objekt': o, 'einheit': einheit, 'standard': std, 'bereich': bereich,
         'werte': list(werte), 'stoff': stoff, 'quelle': quelle, 'modus': modus}
    K.append(e)


STOFFE = ['0 Software Defaults', 'Knit Light Weight', 'Knit Medium Weight', 'Knit Heavy Weight', 'Terry Medium Weight',
          'Terry Heavy Weight', 'Woven Light Weight', 'Woven Medium Weight', 'Woven Heavy Weight']
ALLE = ['lauf', 'dreifach', 'satin', 'satinlinie', 'satinflaeche', 'tatami', 'muster', 'blanket', 'applikation', 'text',
        'ttf', 'monogramm', 'knopfloch', 'motivlinie', 'motivfuellung', 'rueckstich', 'stielstich', 'stippling',
        'blackwork']
V = 'Vorschlag (eigene Festlegung)'

# ---------------------------------------------------------------- Design
G = 'Design'
P('design.rahmen', G, 'Rahmen', 'Stickrahmen; legt das Stickfeld fest', d='rahmen', einheit='Auswahl',
  std='artista 130 x 100', quelle='Rahmenliste der Software (Analyse/daten/hoops_table.md); Befehl rahmen_liste',
  modus='einfach')
P('design.stoff', G, 'Stoff', 'ändert Satin-/Step-Abstand, Zugausgleich, Unterlagen, Step-Versatz, Trim', d='stoff',
  einheit='Auswahl', std='0 Software Defaults', werte=STOFFE, quelle='Stile in NORMAL.AMT (Analyse/code/10 §E)',
  modus='einfach')
P('design.garnkarte', G, 'Garnkarte', 'Farben werden dem nächsten Garn der Karte zugeordnet', d='garnkarte',
  einheit='Auswahl', std='Default', quelle='Garnkarten .tch; die Maschine nutzt Default.tch; Befehl garnkarten_liste',
  modus='einfach')
P('objekt.garn', G, 'Garn von Hand', 'Garn der Karte für dieses Objekt (statt nächstes Garn zur Farbe)',
  ebene='objekt', gilt=ALLE, o='garn', einheit='Nummer', std=None,
  quelle='Feldform eigene Festlegung; Garnauswahl aus der Karte wie im Original (Handbuch Kap. 10)', modus='einfach')
P('design.start', G, 'Startpunkt', 'wo der erste Stich beginnt', d='start', einheit='Auswahl', std='rahmenmitte',
  werte=['rahmenmitte', 'designmitte', 'unten_mitte', 'erster_stich'],
  quelle='Dialog „Start and End“ (Handbuch PDF-S. 205); Standard bis Gerätetest V7 eigene Festlegung')
P('design.ende', G, 'Endpunkt', 'wohin der Rahmen nach dem letzten Stich fährt', d='ende', einheit='Auswahl',
  std='rahmenmitte', werte=['rahmenmitte', 'designmitte', 'unten_mitte', 'start', 'letzter_stich'],
  quelle='Dialog „Start and End“; 34 von 51 Maschinendesigns enden auf 0,0')

# ---------------------------------------------------------------- Größe und Lage
G = 'Größe und Lage'
Q = 'Handbuch Kap. 15; Stiche werden neu berechnet, nie skaliert'
P('transform.skalieren', G, 'Größe', 'skaliert um die Mitte (eine Zahl oder [x, y])', ebene='beide', gilt=ALLE,
  d='transform.skalieren', o='transform.skalieren', einheit='%', std=100, bereich=[10, 500], quelle=Q + '; Bereich ' + V,
  modus='einfach')
P('transform.drehen', G, 'Drehen', 'dreht um die Mitte, gegen den Uhrzeiger', ebene='beide', gilt=ALLE,
  d='transform.drehen', o='transform.drehen', einheit='°', std=0, bereich=[-360, 360], quelle=Q, modus='einfach')
P('transform.spiegeln', G, 'Spiegeln', 'spiegelt um die Mitte', ebene='beide', gilt=ALLE, d='transform.spiegeln',
  o='transform.spiegeln', einheit='Auswahl', std=None, werte=['links_rechts', 'oben_unten', 'beide'], quelle=Q,
  modus='einfach')
P('transform.schraeg', G, 'Schrägstellen', 'verschiebt x mit der Höhe (Skew)', ebene='beide', gilt=ALLE,
  d='transform.schraeg', o='transform.schraeg', einheit='°', std=0, bereich=[-60, 60], quelle=Q + '; Bereich ' + V)
P('transform.verschieben', G, 'Verschieben', 'Lage im Rahmen', ebene='beide', gilt=ALLE, d='transform.verschieben',
  o='transform.verschieben', einheit='mm [x, y]', std=[0, 0], quelle=Q, modus='einfach')

# ---------------------------------------------------------------- Satin
G = 'Satin'
P('satin.auto_spacing', G, 'Auto Spacing', 'Stichabstand automatisch nach der Säulenbreite (schmal = lockerer)',
  d='einstellungen.satin.auto_spacing', einheit='an/aus', std=True,
  quelle='Effekt Auto Spacing 0x21, Tabelle nach Breite (Analyse/code/10, hoch); am Objekt aus, wenn „abstand“ gesetzt')
P('satin.abstand_fest', G, 'Stichabstand (fest)', 'Weg zwischen zwei Einstichen derselben Seite, wenn Auto Spacing aus',
  ebene='beide', gilt=['satin'], d='einstellungen.satin.abstand_fest', o='abstand', einheit='mm', std=0.40,
  bereich=[0.2, 2.0], stoff=True, quelle='AMT 0x2723 (hoch); Bereich ' + V)
P('satin.faktor', G, 'Auto-Spacing-Faktor', 'Faktor auf die Tabelle (Stoff, Schrift 0,9)', ebene='beide', gilt=['satin'],
  d='einstellungen.satin.faktor', o='faktor', einheit='Faktor', std=1.0, bereich=[0.5, 2.0], stoff=True,
  quelle='AMT 0x2736 (hoch)')
P('satin.zugausgleich', G, 'Zugausgleich', 'verlängert jeden Stich an beiden Enden („Fett“ = mehr Zugausgleich)',
  d='einstellungen.satin.zugausgleich', einheit='mm', std=0.20, bereich=[0.0, 1.0], stoff=True,
  quelle='Handbuch S. 199 (hoch); Stile 0x273d')
P('satin.zickzack_abstand', G, 'Zickzack-Abstand', 'Abstand des Zickzack-Deckstichs (Stichart zickzack)',
  d='einstellungen.satin.zickzack_abstand', einheit='mm', std=2.0, bereich=[0.5, 6.0], stoff=True,
  quelle='Stichart Zigzag 0x2724/0x2725 (Stil-Liste 0x27a6, hoch)')
P('satin.kurzstiche.an', G, 'Kurzstiche', 'kürzt Stiche an Innenkurven, damit es dort nicht zu dicht wird',
  ebene='beide', gilt=['satin', 'tatami (mit Winkellinien)'], d='einstellungen.satin.kurzstiche.an', o='kurzstiche',
  einheit='an/aus', std=False, quelle='Effekt 0x273a (MOD1 ShortStitchesEffect); Stil aus, Schrift an')
P('satin.kurzstiche.schwelle', G, 'Kurzstiche: Schwelle', 'ab welcher Dichte gekürzt wird (× Abstand)',
  d='einstellungen.satin.kurzstiche.schwelle', einheit='Faktor', std=0.5, quelle='0x2819 (hoch)', modus='automatisch')
P('satin.kurzstiche.zufall', G, 'Kurzstiche: Zufall', 'Kürzung zufällig ±0,04 verändert',
  d='einstellungen.satin.kurzstiche.zufall', einheit='an/aus', std=True, quelle='0x281a (hoch)', modus='automatisch')
P('satin.kurzstiche.max_gruppe', G, 'Kurzstiche: Gruppe', 'höchstens so viele Stiche je Gruppe',
  d='einstellungen.satin.kurzstiche.max_gruppe', einheit='Anzahl', std=5, quelle='0x281b (hoch)', modus='automatisch')
P('satin.kurzstiche.faktoren', G, 'Kurzstiche: Faktoren', 'Kürzungsfaktoren als Dreieckstabelle',
  d='einstellungen.satin.kurzstiche.faktoren', einheit='Liste',
  std=[0.80, 0.85, 0.72, 0.70, 0.90, 0.70, 0.70, 0.90, 0.80, 0.70, 0.70, 0.87, 0.65, 0.83, 0.70], quelle='0x281c (hoch)',
  modus='automatisch')
P('objekt.stichart_satin', G, 'Stichart Säule/Linie', 'Satin (dicht) oder Zickzack (offen)', ebene='objekt',
  gilt=['satin', 'satinlinie'], o='stichart', einheit='Auswahl', std='satin', werte=['satin', 'zickzack'],
  quelle='Stichartliste STR 2234/2236')
P('objekt.satinlinie.breite', G, 'Breite der Satinlinie', 'Breite der Bahn um die Linie', ebene='objekt',
  gilt=['satinlinie'], o='breite', einheit='mm', std=2.0, bereich=[0.5, 12.0], quelle='Standard ' + V, modus='einfach')
P('objekt.satinlinie.versatz', G, 'Lage der Satinlinie', 'mittig um die Linie oder zur Seite (Linie = Kante)',
  ebene='objekt', gilt=['satinlinie'], o='versatz', einheit='Auswahl', std='mitte', werte=['mitte', 'seite'],
  quelle='Dialog „Outline Stitch“ Center/Offset; Seite links ' + V)

# ---------------------------------------------------------------- Tatami / Step
G = 'Tatami (Step)'
P('tatami.abstand', G, 'Stichabstand', 'Weg zwischen zwei Hinreihen (Nachbarreihen halber Abstand)',
  d='einstellungen.tatami.abstand', einheit='mm', std=0.40, bereich=[0.2, 2.0], stoff=True,
  quelle='AMT 0x2716 (hoch; Handbuch PDF-S. 74)')
P('tatami.laenge', G, 'Stichlänge', 'Länge der Step-Stiche', d='einstellungen.tatami.laenge', einheit='mm', std=4.0,
  bereich=[1.0, 12.7], quelle='AMT 0x2717 (hoch)')
P('tatami.min_rand', G, 'Mindeststich am Rand', 'Einstiche näher am Rand entfallen', d='einstellungen.tatami.min_rand',
  einheit='mm', std=0.40, bereich=[0.1, 2.0], quelle='AMT 0x2718 (hoch)')
P('tatami.versatz', G, 'Reihenversatz 1', 'Verschiebung des Rasters (über Step-Muster gewählt)',
  d='einstellungen.tatami.versatz', einheit='× Stichlänge', std=0.25, quelle='AMT 0x2719 (hoch)', modus='automatisch')
P('tatami.versatz2', G, 'Reihenversatz 2', 'zweiter Versatz, abwechselnd mit dem ersten (null = wie Versatz 1)',
  d='einstellungen.tatami.versatz2', einheit='× Stichlänge', std=None, stoff=True,
  quelle='AMT 0x271a (hoch); Stoffe 0,75', modus='automatisch')
P('tatami.zufall', G, 'Zufallsversatz', 'streut die Einstiche einer Reihe', d='einstellungen.tatami.zufall',
  einheit='× Stichlänge', std=0.0, quelle='AMT 0x271b; Verteilung ' + V, modus='automatisch')
P('tatami.winkel', G, 'Stichwinkel', 'Richtung der Stichreihen', ebene='beide', gilt=['tatami', 'muster', 'ttf'],
  d='einstellungen.tatami.winkel', o='winkel', einheit='°', std=45.0, bereich=[0, 180],
  quelle='Handbuch Kap. 16 (Stitch Angle); Standard 45° ' + V, modus='einfach')
P('tatami.zugausgleich', G, 'Zugausgleich', 'verlängert die Reihen an beiden Enden', d='einstellungen.tatami.zugausgleich',
  einheit='mm', std=0.20, bereich=[0.0, 1.0], stoff=True, quelle='Handbuch S. 199 (hoch)')
P('tatami.laufweg_laenge', G, 'Laufweg in der Fläche', 'Stichlänge der Wege zwischen Teilbereichen',
  d='einstellungen.tatami.laufweg_laenge', einheit='mm', std=1.8, quelle='Lauf 0x271c (hoch)', modus='automatisch')
P('tatami.laufweg.min_laenge', G, 'Laufweg: Mindeststich', 'variabler Lauf: kleinster Stich',
  d='einstellungen.tatami.laufweg.min_laenge', einheit='mm', std=0.4, quelle='0x2767 (hoch)', modus='automatisch')
P('tatami.laufweg.sehnenabstand', G, 'Laufweg: Sehnenabstand', 'variabler Lauf: Abweichung von der Kurve',
  d='einstellungen.tatami.laufweg.sehnenabstand', einheit='mm', std=16 / 180, quelle='0x2768 (hoch)',
  modus='automatisch')
P('objekt.step_muster', G, 'Step-Muster', 'Struktur der Fläche (1 = gleichmäßig)', ebene='objekt',
  gilt=['tatami', 'text (Step)'], o='step_muster', einheit='Nummer', std=1, bereich=[1, 20],
  quelle='Dialog „Fill Stitch“, Tabelle DIALOG.DLL (hoch)', modus='einfach')
P('objekt.winkellinien', G, 'Mehrere Stichwinkel', 'Winkellinien quer über die Fläche, die Stiche drehen mit',
  ebene='objekt', gilt=['tatami'], o='winkellinien', einheit='Linien', std=[],
  quelle='Handbuch Kap. 16; gemessen (Analyse_Teil3/06 §6)')
P('objekt.unterlage_flaeche', G, 'Unterlage der Fläche', 'Step-Unterlage an/aus (ohne Angabe: Stil)', ebene='objekt',
  gilt=['tatami', 'muster'], o='unterlage', einheit='an/aus', std=None, quelle='Auto Underlay des Stils')

# ---------------------------------------------------------------- Füllmuster
G = 'Füllmuster'
P('objekt.muster', G, 'Füllmuster', 'eines der 242 Muster (Program Split)', ebene='objekt', gilt=['muster', 'text'],
  o='muster', einheit='Auswahl', std=None, quelle='Bibliothek in NORMAL.AMT (Analyse_Teil3/13); Liste fuellmuster.json',
  modus='einfach')
P('objekt.muster.groesse', G, 'Mustergröße', 'skaliert die Musterkachel', ebene='objekt', gilt=['muster', 'text'],
  o='groesse', einheit='%', std=100, bereich=[25, 400], quelle='Analyse_Teil3/13; Bereich ' + V)

# ---------------------------------------------------------------- Lauf
G = 'Lauf'
P('lauf.laenge', G, 'Stichlänge', 'Länge der Laufstiche', ebene='beide', gilt=['lauf', 'dreifach', 'ttf'],
  d='einstellungen.lauf.laenge', o='laenge', einheit='mm', std=2.5, bereich=[0.5, 12.7], quelle='AMT 0x27a8 (hoch)')
P('lauf.min_laenge', G, 'Mindeststich', 'kleinster Stich (variable Länge)', d='einstellungen.lauf.min_laenge',
  einheit='mm', std=0.8, quelle='AMT 0x27a8 (hoch)', modus='automatisch')
P('lauf.variabel', G, 'Variable Stichlänge', 'in Kurven kürzere Stiche', d='einstellungen.lauf.variabel',
  einheit='an/aus', std=False, quelle='Run-Dialog: feste Länge Standard (hoch)')
P('lauf.sehnenabstand', G, 'Sehnenabstand', 'erlaubte Abweichung von der Kurve (variable Länge)',
  d='einstellungen.lauf.sehnenabstand', einheit='mm', std=0.09, quelle='Chord Gap (hoch)', modus='automatisch')
P('lauf.ecke_grad', G, 'Eckwinkel', 'ab diesem Knick Einstich an der Ecke (nur berechnete Linien)',
  d='einstellungen.lauf.ecke_grad', einheit='°', std=30.0, quelle=V, modus='automatisch')

# ---------------------------------------------------------------- Unterlage
G = 'Unterlage'
P('unterlage.auto_an', G, 'Unterlage automatisch', 'Auto Underlay an/aus (Schrift: an)', d='einstellungen.unterlage.auto_an',
  einheit='an/aus', std=False, quelle='Stil-Flags 0x2000 (Analyse_Teil3/05, mittel-hoch)', modus='einfach')
P('unterlage.auto_art', G, 'Art der Auto-Unterlage', 'vorgewählte Unterlage', d='einstellungen.unterlage.auto_art',
  einheit='Auswahl', std=['zickzack'], werte=['mittellauf', 'edge_walk', 'zickzack', 'doppelzickzack'],
  quelle='Stil-Bit 0x0001 = Zickzack')
P('objekt.unterlage_saeule', G, 'Unterlage der Säule/Schrift', 'auto, keine oder Arten (z. B. edge_walk+zickzack). '
  'Schrift (auch Computer-Schrift, Monogramm): auto = Stil „Schrift“ (Zickzack); die Oberfläche wählt für neue Texte '
  'Mittellauf + Zickzack vor (Entscheidung Nutzer 21.11.2026); einfacher Modus ein Häkchen „Stabilisieren“ '
  '(an = beide, aus = keine), Experte zwei Häkchen', ebene='objekt',
  gilt=['satin', 'satinlinie', 'text', 'ttf', 'monogramm'], o='unterlage', einheit='Auswahl', std='auto',
  werte=['auto', 'keine', 'mittellauf', 'edge_walk', 'zickzack', 'doppelzickzack'],
  quelle='Handbuch Kap. 22, S. 186 (kleine Schrift: aus); Stil Schrift 0x27a7 (Analyse_Teil3/05)', modus='einfach')
P('unterlage.rand', G, 'Abstand zum Rand', 'Unterlage bleibt so weit innen', d='einstellungen.unterlage.rand',
  einheit='mm', std=0.20, bereich=[0.0, 2.0], quelle='AMT 0x2769/0x276a (mittel)')
P('unterlage.zugausgleich', G, 'Zugausgleich der Unterlage', '', d='einstellungen.unterlage.zugausgleich', einheit='mm',
  std=0.20, quelle='AMT 0x2743 (hoch)')
P('unterlage.edge_walk_laenge', G, 'Edge Walk: Stichlänge', 'Lauf am Rand der Säule',
  d='einstellungen.unterlage.edge_walk_laenge', einheit='mm', std=2.2, quelle='AMT 0x2747 (hoch)')
P('unterlage.mittellauf_laenge', G, 'Mittellauf: Stichlänge', 'Lauf auf der Säulenmitte',
  d='einstellungen.unterlage.mittellauf_laenge', einheit='mm', std=2.2, quelle='AMT 0x2746 (hoch)')
P('unterlage.zickzack_laenge', G, 'Zickzack-Unterlage: Abstand', 'Weg zwischen Einstichen derselben Seite',
  d='einstellungen.unterlage.zickzack_laenge', einheit='mm', std=5.0, stoff=True, quelle='AMT 0x2744/0x2725 (mittel)')
P('unterlage.doppelzickzack_laenge', G, 'Doppel-Zickzack: Abstand', '',
  d='einstellungen.unterlage.doppelzickzack_laenge', einheit='mm', std=7.0, stoff=True, quelle='AMT 0x2745/0x2725 (mittel)')
P('unterlage.rueck_lauf_laenge', G, 'Zickzack: Rücklauf', 'Lauf zurück zur Säulenmitte',
  d='einstellungen.unterlage.rueck_lauf_laenge', einheit='mm', std=3.0, quelle='AMT 0x2744/0x271d (mittel)',
  modus='automatisch')
P('unterlage.tatami_abstand', G, 'Step-Unterlage: Abstand', '', d='einstellungen.unterlage.tatami_abstand', einheit='mm',
  std=3.0, quelle='AMT 0x275a (mittel)')
P('unterlage.tatami_laenge', G, 'Step-Unterlage: Stichlänge', '', d='einstellungen.unterlage.tatami_laenge',
  einheit='mm', std=4.0, quelle='AMT 0x275a (mittel)')
P('unterlage.tatami_randlauf', G, 'Step-Unterlage: Randlauf', 'Stichlänge des Laufs um die Fläche',
  d='einstellungen.unterlage.tatami_randlauf', einheit='mm', std=2.5, quelle='AMT 0x275a (mittel)', modus='automatisch')
P('unterlage.tatami_winkel_versatz', G, 'Step-Unterlage: Winkel', 'gegen die Deckrichtung gedreht',
  d='einstellungen.unterlage.tatami_winkel_versatz', einheit='°', std=90.0, quelle='AMT 0x2759', modus='automatisch')
for art, (mn, sa) in {'edge_walk_lauf': (72, 13), 'mittellauf_lauf': (131, 11), 'zickzack_lauf': (72, 36),
                      'doppelzickzack_lauf': (179, 16), 'tatami_randlauf_lauf': (72, 13)}.items():
    P(f'unterlage.{art}.min_laenge', G, f'{art}: Mindeststich', 'variabler Lauf der Unterlage',
      d=f'einstellungen.unterlage.{art}.min_laenge', einheit='mm', std=mn / 180, quelle='0x2767 (hoch)',
      modus='automatisch')
    P(f'unterlage.{art}.sehnenabstand', G, f'{art}: Sehnenabstand', 'variabler Lauf der Unterlage',
      d=f'einstellungen.unterlage.{art}.sehnenabstand', einheit='mm', std=sa / 180, quelle='0x2768 (hoch)',
      modus='automatisch')

# ---------------------------------------------------------------- Verbinder
G = 'Verbinder und Vernähen'
Q = 'Dialog „Connectors“, Stil 0x272a (DIALOG.DLL, hoch)'
for k, name, w, std, einh, tag in [
    ('anfang_immer', 'Immer vernähen (Anfang)', 'Tie In immer', False, 'an/aus', '0x2753'),
    ('anfang_nach_trim_farbwechsel', 'Vernähen nach Trim/Farbwechsel', 'Tie In If After Trim', True, 'an/aus', '0x2750'),
    ('anfang_bei_langem_verbinder', 'Vernähen nach langem Verbinder', 'Tie In If Previous Connector Longer', True, 'an/aus',
     '0x272d'),
    ('anfang_verbinder_ab', '… Verbinder länger als', '', 3.0, 'mm', '0x2755'),
    ('ende_immer', 'Immer vernähen (Ende)', 'Tie Off immer', False, 'an/aus', '0x2752'),
    ('ende_vor_trim_farbwechsel', 'Vernähen vor Trim/Farbwechsel', 'Tie Off If Before Trim', True, 'an/aus', '0x2733'),
    ('ende_bei_langem_verbinder', 'Vernähen vor langem Verbinder', 'Tie Off If Next Connector Longer', True, 'an/aus',
     '0x272c'),
    ('ende_verbinder_ab', '… Verbinder länger als', '', 3.0, 'mm', '0x2754'),
    ('letztes_objekt_vernaehen', 'Letztes Objekt vernähen', 'Always Tie Off Last Object', False, 'an/aus', '0x276c'),
    ('trim_ab', 'Trim ab', 'ab dieser Verbinderlänge gilt ein Sprung als Trim (die 185 schneidet nicht)', 3.0, 'mm',
     '0x272e'),
    ('vernaeh_stichlaenge', 'Länge der Vernähstiche', '', 1.0, 'mm', '0x2730/0x2731'),
    ('vernaeh_stiche', 'Zahl der Vernähstiche', '', 2, 'Anzahl', '0x272f/0x275d'),
]:
    P(f'verbinder.{k}', G, name, w, d=f'einstellungen.verbinder.{k}', einheit=einh, std=std, stoff=(k == 'trim_ab'),
      quelle=f'{Q}; {tag}')

# ---------------------------------------------------------------- Auto Jump, Maschine, Bearbeiten
G = 'Auto Jump und Maschine'
P('objekt.auto_jump', G, 'Auto Jump', 'lange Stiche als Sprünge statt geteilt (Standard an, Schrift aus)',
  ebene='objekt', gilt=ALLE, o='auto_jump', einheit='an/aus', std=True, quelle='Effects-Dialog; AMT 0x2758 (hoch)')
P('objekt.auto_jump.max', G, 'Auto Jump: Höchststich', 'darüber Sprünge; klein (z. B. 6 mm) = Satin mit mehr Loft',
  ebene='objekt', gilt=ALLE, o='auto_jump.max', einheit='mm', std=12.7, bereich=[0.3, 12.7], quelle='AMT 0x2711 (hoch)')
P('objekt.auto_jump.sprung', G, 'Auto Jump: Sprunglänge', 'Länge der Teilsprünge', ebene='objekt', gilt=ALLE,
  o='auto_jump.sprung', einheit='mm', std=7.0, bereich=[0.3, 12.7], quelle='AMT 0x2712 (hoch)')
P('maschine.min_stich', G, 'Kürzester Stich', 'kürzere Stiche werden entfernt', ebene='design', einheit='mm', std=0.3,
  quelle='DEFAULTS.DFL (hoch)', modus='automatisch')
P('maschine.max_stich', G, 'Längster Stich', 'Maschinengrenze artista 185', ebene='design', einheit='mm', std=12.7,
  quelle='DEFAULTS.DFL (hoch)', modus='automatisch')
G = 'Bearbeiten'
P('objekt.doppellauf', G, 'Verstärken (Double Run)', 'gleiche Stiche rückwärts noch einmal', ebene='objekt', gilt=ALLE,
  o='doppellauf', einheit='an/aus', std=False, quelle='Handbuch Kap. 23')
FL = ['tatami', 'satinflaeche', 'muster', 'satin', 'satinlinie', 'blanket', 'text', 'ttf', 'monogramm', 'applikation']
QU = 'eigene Erweiterung (Nutzer 21.11.2026); im Original eine zweite Linie von Hand (Handbuch S. 114)'
P('objekt.umrandung', G, 'Umrandung', 'Linie um die Fläche (Text: um jeden Buchstaben), gestickt nach dem Objekt; '
  'das Häkchen im einfachen Modus setzt eine dicke Satinkante außen (Text 6 % der Buchstabenhöhe, Flächen 2 mm)',
  ebene='objekt', gilt=FL, o='umrandung', einheit='an/aus', std=False, quelle=QU, modus='einfach')
P('objekt.umrandung.farbe', G, 'Umrandung: Garn', 'Farbe der Umrandung (eigener Farbblock)', ebene='objekt', gilt=FL,
  o='umrandung.farbe', einheit='#rrggbb', std='#000000', quelle=QU + '; ' + V, modus='einfach')
P('objekt.umrandung.art', G, 'Umrandung: Stichart', 'Lauf (dünn), Dreifach (kräftig), Satinkante', ebene='objekt',
  gilt=FL, o='umrandung.art', einheit='Auswahl', std='dreifach', werte=['lauf', 'dreifach', 'satin'],
  quelle=QU + '; ' + V)
P('objekt.umrandung.einzug', G, 'Umrandung: nach innen', 'Versatz der Linie in die Fläche, damit nach dem Sticken '
  'keine Lücke zur Füllung bleibt', ebene='objekt', gilt=FL, o='umrandung.einzug', einheit='mm', std=0.3,
  bereich=[0, 3], quelle=V + ' – am Kalibrierblatt (Vor Ort V12) prüfen')
P('objekt.umrandung.breite', G, 'Umrandung: Breite der Satinkante', 'nur bei Stichart Satinkante', ebene='objekt',
  gilt=FL, o='umrandung.breite', einheit='mm', std=2.0, bereich=[0.5, 10], quelle='wie Satinlinie (Standard 2 mm)')
P('objekt.umrandung.breite_anteil', G, 'Umrandung: Breite in % der Buchstabenhöhe', 'nur Text/Monogramm mit '
  'Satinkante, gilt statt der Breite in mm; Größe/Skalieren zählt mit; unter 1 mm wird daraus eine kräftige Linie '
  '(Dreifach, Stichlänge 12 % der Höhe)', ebene='objekt',
  gilt=['text', 'ttf', 'monogramm'], o='umrandung.breite_anteil', einheit='%', std=None, bereich=[1, 30],
  quelle=V + ' – Wunsch Nutzer 21.11.2026 (50 mm → 3 mm = 6 %, im einfachen Modus gesetzt)')
P('objekt.umrandung.aussen', G, 'Umrandung: Satinkante außen', 'Innenkante liegt „nach innen“ mm in der Fläche, die '
  'Kante wächst nach außen, die Farbe bleibt ganz sichtbar; sonst liegt die Mitte der Kante auf dem Rand',
  ebene='objekt', gilt=FL, o='umrandung.aussen', einheit='an/aus', std=False, quelle=V + ' (im einfachen Modus an)')
P('objekt.umrandung.laenge', G, 'Umrandung: Stichlänge', 'Lauf/Dreifach; ohne Angabe Stil', ebene='objekt', gilt=FL,
  o='umrandung.laenge', einheit='mm', std=None, bereich=[0.3, 12.7], quelle='Stil Lauf (Software Defaults 2,5 mm)')
P('objekt.aussparen', G, 'Aussparen (Knockout)', 'Fläche spart die genannten Objekte aus', ebene='objekt',
  gilt=['tatami', 'muster'], o='aussparen', einheit='Objektnamen', std=[], quelle='Remove Overlay (Analyse_Teil3/14)',
  modus='einfach')
P('objekt.ueberlappung', G, 'Überlappung beim Aussparen', 'Loch bleibt um so viel kleiner', ebene='objekt',
  gilt=['tatami', 'muster'], o='ueberlappung', einheit='mm', std=0.0, bereich=[0, 3], quelle='Analyse_Teil3/14')

# ---------------------------------------------------------------- Reihenfolge, Ein-/Ausstich (ROADMAP 4b.5)
G = 'Reihenfolge'
Q = 'Analyse_Teil3/16_reihenfolge.md'
P('design.kuerzeste_verbindung', G, 'Kürzeste Verbindung (Closest Join)',
  'jedes Objekt ohne eigenen Einstich beginnt am Punkt nächst dem vorigen Ende (Linien umgedreht, geschlossene Formen '
  'gedreht; Schrift ausgenommen)', d='kuerzeste_verbindung', einheit='an/aus', std=False,
  quelle='MOD1 ResequenceByClosestJoin 0x1006d0c0, GEOMETRY FindExitEntry (hoch); im Original Option von Resequence '
  'und Auto Digitizer, sonst „wie digitalisiert“ (Messung ' + Q + ')')
P('objekt.einstich', G, 'Einstich (entry point)', 'Objekt beginnt hier bzw. am nächsten Punkt der Form', ebene='objekt',
  gilt=ALLE, o='einstich', einheit='mm [x, y]', std=None,
  quelle='Handbuch Kap. 7/14 (entry point, Reshape); Umsetzung je Objektart eigene Festlegung (' + Q + ')')
P('objekt.ausstich', G, 'Ausstich (exit point)', 'Objekt endet hier bzw. so nah wie die Form erlaubt', ebene='objekt',
  gilt=ALLE, o='ausstich', einheit='mm [x, y]', std=None,
  quelle='Handbuch Kap. 7/14 (exit point); Flächen: Beginn gegenüber, eigene Festlegung (' + Q + ')')
P('objekt.kuerzeste', G, 'Closest Join je Objekt', 'wie „Kürzeste Verbindung“, nur für dieses Objekt; setzen Bild '
  'umwandeln, Zauberstab und SVG einlesen beim Einfügen', ebene='objekt', gilt=ALLE + ['photosnap'], o='kuerzeste',
  einheit='an/aus', std=False, quelle='Handbuch S. 116 (Auto Digitizer: closest join); je Objekt eigene Festlegung '
  '(Analyse_Teil3/25 §1)', modus='automatisch')
P('reihenfolge.farben', G, 'Farbreihenfolge beim Sortieren', 'gewünschte Folge der Farben (Move Up/Down), übrige '
  'Farben in der Reihenfolge ihres ersten Auftretens', ebene='werkzeug', einheit='Farbliste', std=[],
  quelle='Dialog „Sequence By Color“ (Handbuch Kap. 14); Befehl reihenfolge --nach-farbe')
P('reihenfolge.ueberdeckung', G, 'Überdeckung beachten', 'beim Sortieren nach Farbe behalten Objekte, die sich '
  'überdecken, ihre Reihenfolge', ebene='werkzeug', einheit='an/aus', std=False,
  quelle='eigene Erweiterung (das Original prüft nicht, ' + Q + ')', modus='einfach')
P('reihenfolge.min_ueberdeckung', G, 'Überdeckung ab', 'kleinere gemeinsame Flächen zählen nicht', ebene='werkzeug',
  einheit='mm²', std=0.5, quelle='eigene Festlegung', modus='automatisch')
P('reihenfolge.linie_breite', G, 'Breite einer Linie bei der Prüfung', 'Laufstich zählt als Band dieser Breite',
  ebene='werkzeug', einheit='mm', std=0.5, quelle='eigene Festlegung', modus='automatisch')

# Anordnen (Nachtauftrag N6, Analyse_Teil3/26)
Q6 = 'DESVIEW MirrorMergeView*::GetReflectionTransforms (Analyse_Teil3/26)'
P('ausrichten.art', 'Anordnen', 'Ausrichten', 'Kanten oder Mitten der gewählten Objekte an das zuletzt gewählte',
  ebene='werkzeug', einheit='Auswahl', std=None,
  werte=['links', 'rechts', 'oben', 'unten', 'mitte_senkrecht', 'mitte_waagrecht', 'mitte'],
  quelle='Arrange-Leiste BERESENU STR 33233–33239; Handbuch S. 135 („aligned with the last object selected“)')
P('spiegeln.art', 'Anordnen', 'Mirror-Merge Richtung', 'waagrecht = Kopie links/rechts, senkrecht = oben/unten, '
  'beides = drei Kopien', ebene='werkzeug', einheit='Auswahl', std='links_rechts',
  werte=['links_rechts', 'oben_unten', 'beide'], quelle=Q6 + ' (hoch); Vorgabe eigene Festlegung')
P('spiegeln.achse', 'Anordnen', 'Ankerpunkt', 'Punkt, durch den die Spiegellinie geht; Vorschlag rechte bzw. untere '
  'Kante der Auswahl', ebene='werkzeug', einheit='mm', std=None,
  quelle='Handbuch S. 141 (Bezugslinie, Klick setzt Anker); Vorschlag eigene Festlegung')
P('kranz.punkte', 'Anordnen', 'Punkte des Kranzes', 'Original + Kopien rundherum, je 360°/n gedreht (im '
  'Uhrzeigersinn)', ebene='werkzeug', einheit='Anzahl', std=6, bereich=[2, 36],
  quelle=Q6 + ' (Drehung −i·2π/n, hoch); Vorgabe Handbuch-Beispiel „six-point wreath“, Bereich eigene Festlegung')
P('kranz.kaleidoskop', 'Anordnen', 'Kaleidoskop', 'jede zweite Kopie an der Bezugslinie gespiegelt; nur gerade Zahl',
  ebene='werkzeug', einheit='an/aus', std=False, quelle=Q6 + ' (Mirror bei ungeradem i, hoch); Handbuch S. 142')
P('kranz.mitte', 'Anordnen', 'Mittelpunkt', 'Mitte des Kranzes; Vorschlag unter der Auswahl, Kopien berühren sich '
  'nicht', ebene='werkzeug', einheit='mm', std=None, quelle='Handbuch S. 141; Vorschlag eigene Festlegung')
P('anordnen.zusammenfuehren', 'Anordnen', 'Überlappende zusammenführen', 'Step-, Muster-, Stippling- und '
  'Satinflächen, die sich mit ihrer Kopie überlappen, werden ein Objekt', ebene='werkzeug', einheit='an/aus',
  std=False, quelle='RESENU STR 4159 (Rückfrage, hoch); Verfahren eigene Festlegung (Analyse_Teil3/26 §4)')
P('anordnen.ueberlappung_min', 'Anordnen', 'Überlappung ab', 'kleinere gemeinsame Flächen zählen nicht als '
  'Überlappung', ebene='werkzeug', einheit='mm²', std=0.5, quelle='eigene Festlegung', modus='automatisch')

# Fremde Stickdateien (Nachtauftrag N7, Analyse_Teil3/27)
P('stickdatei.rahmen', 'Dateien', 'Rahmen beim Öffnen', 'kleinster Rahmen der 185, in den das Design passt '
  '(Freiarm nur, wenn nichts anderes passt)', ebene='werkzeug', einheit='Auswahl', std=None,
  quelle='eigene Festlegung (Analyse_Teil3/27 §2)', modus='automatisch')
P('stiche.skalieren_hinweis', 'Dateien', 'Hinweis beim Skalieren fester Stiche', 'ab dieser Größenänderung meldet '
  'das Programm, dass sich die Dichte ändert (Stichzahl bleibt)', ebene='werkzeug', einheit='%', std=5,
  quelle='Handbuch Kap. 28 S. 256 („more than ±5 %“)', modus='automatisch')
P('stiche.vernaehen', 'Dateien', 'Vernähen fester Stiche', 'Stichdaten aus Dateien werden nicht zusätzlich '
  'vernäht (die Datei hat ihr eigenes)', ebene='werkzeug', einheit='an/aus', std=False,
  quelle='eigene Festlegung (Grade D: Stiche unverändert, Handbuch S. 256)', modus='automatisch')

# ---------------------------------------------------------------- Blanket, Applikation
G = 'Blanket'
P('objekt.blanket.abstand', G, 'Zahnabstand', '', ebene='objekt', gilt=['blanket'], o='abstand', einheit='mm', std=2.0,
  bereich=[0.5, 10], quelle='Stil 0x2727/0x2728')
P('objekt.blanket.breite', G, 'Zahnbreite', '', ebene='objekt', gilt=['blanket'], o='breite', einheit='mm', std=3.0,
  bereich=[0.5, 12], quelle='Stil 0x2729', modus='einfach')
P('objekt.blanket.versatz', G, 'Lage', 'mittig oder zur Seite', ebene='objekt', gilt=['blanket'], o='versatz',
  einheit='Auswahl', std='mitte', werte=['mitte', 'seite'], quelle='Dialog Center/Offset')
G = 'Applikation'
Q = 'AutoAppliqueValues (MOD1 0x101f3c70 = NORMAL.AMT 0x3f19), Dialog DLG 1455 (Analyse_Teil3/12 §8, hoch)'
P('objekt.appl.abdeckung', G, 'Abdeckung', 'Satin oder Blanket', ebene='objekt', gilt=['applikation'], o='abdeckung',
  einheit='Auswahl', std='satin', werte=['satin', 'blanket'], quelle=Q + ': 0x3f2f = 0x2721 Satin', modus='einfach')
P('objekt.appl.stil', G, 'Stoff zuschneiden', 'vorgeschnitten oder im Rahmen zuschneiden (mit Schnittlinie)',
  ebene='objekt', gilt=['applikation'], o='stil', einheit='Auswahl', std='vorgeschnitten',
  werte=['vorgeschnitten', 'zuschneiden'], quelle=Q + ': 0x3f31 = 1 = Pre-Cut', modus='einfach')
P('objekt.appl.versatz', G, 'Versatz', 'Abdeck- und Heftstich gegen die Platzierungslinie: −100 ganz außen, 0 mittig, '
  '100 ganz im Stoff (bei Blanket immer innen)', ebene='objekt', gilt=['applikation'], o='versatz', einheit='%', std=0,
  bereich=[-100, 100], quelle=Q + ': 0x3f28/0x3f1e = 0, Schieberegler −1 … +1 in 21 Stufen', modus='einfach')
P('objekt.appl.teilabdeckung', G, 'Teil-Abdeckung', 'kein Abdeckstich, wo eine spätere Applikation den Rand überdeckt '
  '(Partial Appliqué)', ebene='objekt', gilt=['applikation'], o='teilabdeckung', einheit='an/aus', std=False,
  quelle=Q + ': 0x3f26 = 0; Handbuch S. 248–249; Schnitt eigene Festlegung', modus='einfach')
P('objekt.appl.breite', G, 'Breite der Abdeckung', '', ebene='objekt', gilt=['applikation'], o='breite', einheit='mm',
  std=5.0, bereich=[0.5, 10], quelle=Q + ': 0x3f27 = 900')
P('objekt.appl.abstand', G, 'Abstand der Abdeckung', 'Satinabstand bzw. Abstand der Blanket-Zähne', ebene='objekt',
  gilt=['applikation'], o='abstand', einheit='mm', std=0.5, bereich=[0.2, 10],
  quelle=Q + ': 0x3f30 = 90 (Satin); Blanket ohne Angabe 2,0 (Stil 0x2728, eigene Festlegung)')
P('objekt.appl.heft_breite', G, 'Heftstich: Breite', 'Zickzack', ebene='objekt', gilt=['applikation'], o='heft_breite',
  einheit='mm', std=4.0, bereich=[0.5, 10], quelle=Q + ': 0x3f1d = 720')
P('objekt.appl.heft_abstand', G, 'Heftstich: Abstand', '', ebene='objekt', gilt=['applikation'], o='heft_abstand',
  einheit='mm', std=5.0, bereich=[0.5, 12.7], quelle=Q + ': 0x3f21 = 900')
P('objekt.appl.platz_laenge', G, 'Platzierungslinie: Stichlänge', '', ebene='objekt', gilt=['applikation'],
  o='platz_laenge', einheit='mm', std=1.5, bereich=[0.5, 12.7], quelle=Q + ': 0x3f1a = 270')
G = 'Knopfloch'
Q = 'MOD1 AllButtonHolesValues 0x10028060 (hoch); Bericht Analyse_Teil3/19'
P('objekt.knopfloch.typ', G, 'Knopfloch-Typ', 'Vorlage aus ButtonHoles.ESB (9 Typen)', ebene='objekt',
  gilt=['knopfloch'], o='typ', einheit='Auswahl', std='standard',
  werte=['standard', 'standard_schmal', 'rund', 'dehnbar', 'heirloom', 'rundes_ende', 'auge_eckig', 'auge_rund',
         'auge_spitz'], quelle=Q + ', Typ 0x3ff9', modus='einfach')
P('objekt.knopfloch.position', G, 'Lage', 'Mitte des Schlitzes [x, y]', ebene='objekt', gilt=['knopfloch'],
  o='position', einheit='mm', std=None, quelle=V, modus='einfach')
P('objekt.knopfloch.laenge', G, 'Schlitzlänge', 'Raupen werden gestreckt, oberes Ende rückt nach (Slit Length)',
  ebene='objekt', gilt=['knopfloch'], o='laenge', einheit='mm', std=20.0, bereich=[3, 60],
  quelle=Q + ', 3600 = 20 mm; Bereich eigene Festlegung', modus='einfach')
P('objekt.knopfloch.winkel', G, 'Winkel', 'Richtung des Schlitzes, 90° = senkrecht (Angle)', ebene='objekt',
  gilt=['knopfloch'], o='winkel', einheit='Grad', std=90.0, quelle=Q + ', π/2', modus='einfach')
P('objekt.knopfloch.abstand', G, 'Raupenabstand', 'Dichte der Raupen (Bead Stitch Spacing); Enden bleiben wie die '
  'Vorlage; ohne Angabe wie die Vorlage (0,5 mm, Heirloom 1,5 mm)', ebene='objekt', gilt=['knopfloch'], o='abstand',
  einheit='mm', std=None, bereich=[0.2, 2.0], quelle=Q + ', 90 = 0,5 mm; Musterdesign Buttonholes.ART')
G = 'Motivlinie'
Q = 'Musterdesigns „Sampler - Pattern Outlines“ (Analyse_Teil3/20, hoch); Handbuch Kap. 25'
P('objekt.motiv.satz', G, 'Motivsatz', 'einer der 14 Motivsätze der CD (userletw)', ebene='objekt', gilt=['motivlinie'],
  o='satz', einheit='Auswahl', std='NP004a',
  quelle='Befehl motive_liste; Standard NORMAL.AMT <PRESET_STANDARD_MOTIF_RUN> 0x2798 (Analyse_Teil3/24 §1)',
  modus='einfach')
P('objekt.motiv.motiv', G, 'Motiv', 'Motiv im Satz (Auswahl mit Vorschau)', ebene='objekt', gilt=['motivlinie'],
  o='motiv', einheit='Auswahl', std='K0001a',
  quelle='Befehl motivsatz_motive; Standard <PRESET_STANDARD_MOTIF_RUN> (Analyse_Teil3/24 §1)',
  modus='einfach')
P('objekt.motiv.groesse', G, 'Größe', 'Länge eines Motivs entlang der Linie (Size); Höhe wächst mit',
  ebene='objekt', gilt=['motivlinie'], o='groesse', einheit='mm', std=None,
  quelle=Q + ': Größe = Grundlinie der Kopie, ohne Angabe wie im Motivsatz', modus='einfach')
P('objekt.motiv.abstand', G, 'Abstand', 'Wiederholabstand (Spacing); Anzahl = Linienlänge / Abstand gerundet, '
  'Motive gleichmäßig verteilt', ebene='objekt', gilt=['motivlinie'], o='abstand', einheit='mm', std=None,
  quelle=Q + ': Verteilung gemessen; Abstand ≠ Größe nicht gemessen (mittel)', modus='experte')
P('objekt.motiv.spiegeln', G, 'Spiegeln', 'Motive auf die andere Seite der Linie', ebene='objekt',
  gilt=['motivlinie'], o='spiegeln', einheit='an/aus', std=False, quelle=V + ' (Handbuch: Orientation)',
  modus='einfach')
G = 'Motivfüllung'
Q = 'NORMAL.AMT <PRESET_STANDARD_MOTIF_FILL> 0x27ee (Analyse_Teil3/24 §1, korrigiert Bericht 20); Handbuch S. 227–229'
P('objekt.motivfuellung.satz', G, 'Motivsatz', 'Motivsatz der CD oder eigener Satz', ebene='objekt',
  gilt=['motivfuellung'], o='satz', einheit='Auswahl', std='NP004a', quelle=Q + ': Satz NP004a',
  modus='einfach')
P('objekt.motivfuellung.motiv', G, 'Motiv', 'Motiv im Satz (Auswahl mit Vorschau)', ebene='objekt',
  gilt=['motivfuellung'], o='motiv', einheit='Auswahl', std='K0001a', quelle=Q + ': Motiv K0001a', modus='einfach')
P('objekt.motivfuellung.groesse', G, 'Motivgröße', 'Länge der Grundlinie eines Motivs; Höhe und Abstände wachsen mit',
  ebene='objekt', gilt=['motivfuellung'], o='groesse', einheit='mm', std=None,
  quelle=Q + ': 0x3ab8 = 2158 (12,0 mm, Motiv K0001a); ohne Angabe wie im Motivsatz', modus='einfach')
P('objekt.motivfuellung.spaltenabstand', G, 'Spaltenabstand', 'Abstand der Motive in einer Reihe (column spacing)',
  ebene='objekt', gilt=['motivfuellung'], o='spaltenabstand', einheit='mm', std=None,
  quelle=Q + ': 0x27b9 = 2158 = Kante an Kante; ohne Angabe Breite des Motivs', modus='experte')
P('objekt.motivfuellung.reihenabstand', G, 'Reihenabstand', 'Abstand der Reihen (row spacing)', ebene='objekt',
  gilt=['motivfuellung'], o='reihenabstand', einheit='mm', std=None,
  quelle=Q + ': 0x27b6 = 2158 = Kante an Kante; ohne Angabe Höhe des Motivs', modus='experte')
P('objekt.motivfuellung.versatz', G, 'Reihenversatz', 'jede Reihe um so viel weiter verschoben (row offset)',
  ebene='objekt', gilt=['motivfuellung'], o='versatz', einheit='mm', std=0,
  quelle=Q + ': Leitmotiv-Matrix 0x3abe = Einheitsmatrix (kein Versatz)', modus='experte')
P('objekt.motivfuellung.winkel', G, 'Drehung der Reihen', 'dreht das Raster (Stichwinkel wirkt nicht)',
  ebene='objekt', gilt=['motivfuellung'], o='winkel', einheit='°', std=0, bereich=[-180, 180],
  quelle=Q + ': „The stitch angle has no effect on pattern layout“', modus='experte')
G = 'Craft Stitches'
Q = 'NORMAL.AMT Stil <0 Software Defaults>, MOD1 Rückstich 0x1001d960 (Analyse_Teil3/24)'
P('objekt.rueckstich.laenge', G, 'Länge', 'Länge eines Rückstichs (Abstand der Punkte entlang der Linie)',
  ebene='objekt', gilt=['rueckstich'], o='laenge', einheit='mm', std=3.0, bereich=[0.5, 12.7],
  quelle=Q + ': 0x2851 = 540 = 3,0 mm (hoch)', modus='einfach')
P('objekt.rueckstich.striche', G, 'Striche', 'Fäden je Stich: 3 (Hin, Rück, Hin) oder 5', ebene='objekt',
  gilt=['rueckstich'], o='striche', einheit='Anzahl', std=3, bereich=[3, 5], quelle=Q + ': 0x2854 = 3 (hoch)',
  modus='experte')
P('objekt.rueckstich.staerke', G, 'Stärke', 'Seitenversatz der Striche – wie dick die Linie wirkt', ebene='objekt',
  gilt=['rueckstich'], o='staerke', einheit='mm', std=0.2, bereich=[0, 1], quelle=Q + ': 0x2852 = 36 = 0,2 mm (hoch)',
  modus='experte')
P('objekt.rueckstich.ueberlappung', G, 'Überlappung', 'wie weit ein Stich in den vorigen zurückreicht',
  ebene='objekt', gilt=['rueckstich'], o='ueberlappung', einheit='%', std=50, bereich=[0, 100],
  quelle=Q + ': 0x2853 = 0,5 (hoch)', modus='experte')
P('objekt.stielstich.breite', G, 'Breite', 'Breite der Kordel', ebene='objekt', gilt=['stielstich'], o='breite',
  einheit='mm', std=2.0, bereich=[0.5, 10], quelle=Q + ': 0x2857 = 360 = 2,0 mm (Zuordnung mittel)', modus='einfach')
P('objekt.stielstich.abstand', G, 'Abstand', 'Abstand der Stiche entlang der Linie', ebene='objekt',
  gilt=['stielstich'], o='abstand', einheit='mm', std=1.5, bereich=[0.2, 10],
  quelle=Q + ': 0x2859 = 270 = 1,5 mm (mittel)', modus='einfach')
P('objekt.stielstich.winkel', G, 'Stichwinkel', 'Neigung der Stiche zur Linie; in engen Kurven steiler',
  ebene='objekt', gilt=['stielstich'], o='winkel', einheit='°', std=50, bereich=[10, 90],
  quelle=Q + ': 0x285a = 50° (mittel)', modus='experte')
P('objekt.stielstich.dreifach', G, 'Dreifach', 'jeder Stich mit Hin-Rück-Hin – Seil-Optik', ebene='objekt',
  gilt=['stielstich'], o='dreifach', einheit='an/aus', std=True, quelle=Q + ': 0x2858 = 3 (mittel)', modus='experte')
P('objekt.stielstich.staerke', G, 'Stärke', 'Versatz der Dreifach-Striche', ebene='objekt', gilt=['stielstich'],
  o='staerke', einheit='mm', std=0.2, bereich=[0, 1], quelle=Q + ': 0x2856 = 36 = 0,2 mm (mittel)', modus='experte')
P('objekt.stippling.abstand', G, 'Abstand', 'Abstand der Bahnen der mäandernden Linie', ebene='objekt',
  gilt=['stippling'], o='abstand', einheit='mm', std=3.0, bereich=[0.5, 20],
  quelle='Handbuch S. 242–243 (Beispielwerte, niedrig); Verfahren ' + V, modus='einfach')
P('objekt.stippling.laenge', G, 'Stichlänge', 'Länge der Laufstiche', ebene='objekt', gilt=['stippling'],
  o='laenge', einheit='mm', std=2.0, bereich=[0.5, 12.7], quelle='Handbuch S. 242–243 (niedrig)', modus='einfach')
P('objekt.blackwork.stich', G, 'Stichart (Blackwork)', 'Stichart der verbundenen Linien; Werte wie beim Objekt '
  'dieser Art', ebene='objekt', gilt=['blackwork'], o='stich', einheit='Auswahl', std='lauf',
  werte=['lauf', 'dreifach', 'rueckstich', 'stielstich'],
  quelle='Handbuch S. 131–132 (Blackwork Run: Einfach-/Dreifachlauf, Stielstich, Rückstich); Reihenfolge ' + V,
  modus='einfach')
G = 'PhotoSnap'
Q = 'MOD1 PhotoValues 0x10122d50 / GenerateComponents 0x10120aa0, NORMAL.AMT 0x3ee4 (Analyse_Teil3/25 §3)'
PS = ['photosnap']
P('objekt.photosnap.aufloesung', G, 'Auflösung', 'Breite der Stichreihen: grob 3, mittel 2, fein 1 mm', ebene='objekt',
  gilt=PS, o='aufloesung', einheit='Auswahl', std='mittel', werte=['grob', 'mittel', 'fein'],
  quelle='Handbuch S. 119 (Coarse/Medium/Fine); mittel 2,0 mm = 0x3ee5 = 360 (niedrig); grob/fein ' + V,
  modus='einfach')
P('objekt.photosnap.hintergrund', G, 'Stoff', 'hell: dunkle Stellen bekommen viel Faden; dunkel: helle Stellen',
  ebene='objekt', gilt=PS, o='hintergrund', einheit='Auswahl', std='hell', werte=['hell', 'dunkel'],
  quelle=Q + ': 0x3ee8 = 1; Handbuch S. 120 Background Light/Dark', modus='einfach')
P('objekt.photosnap.winkel', G, 'Winkel der Reihen', 'Richtung der Stichreihen', ebene='objekt', gilt=PS, o='winkel',
  einheit='°', std=0.0, bereich=[-360, 360], quelle=Q + ': 0x3eeb, Vorgabe 0 im Konstruktor (Handbuch-Bild 45°)',
  modus='einfach')
P('objekt.photosnap.lagen', G, 'Farblagen', 'gekreuzt: je Lage 45° mehr; versetzt: gleicher Winkel, Reihen je Lage '
  '0,4 mm verschoben', ebene='objekt', gilt=PS, o='lagen', einheit='Auswahl', std='gekreuzt',
  werte=['gekreuzt', 'versetzt'], quelle=Q + ': Modus 1 = 0/45/90/135° (0x10120890), Modus 2 = 72/144/216 '
  '(0x101209e0) (hoch)', modus='einfach')
P('objekt.photosnap.abstand_min', G, 'Abstand dunkel', 'Abstand der Zickzackstiche an der dunkelsten Stelle',
  ebene='objekt', gilt=PS, o='abstand_min', einheit='mm', std=0.8, bereich=[0.4, 5],
  quelle='Stil 0x2828 Feld 0x282a = 144 (Zuordnung niedrig)')
P('objekt.photosnap.abstand_max', G, 'Abstand hell', 'Abstand an der hellsten Stelle (größter Abstand)',
  ebene='objekt', gilt=PS, o='abstand_max', einheit='mm', std=10.0, bereich=[1, 12],
  quelle='Stil 0x2828 Feld 0x282f = 1800 (Zuordnung niedrig); Handbuch „Max Width … lightest part“')
P('photosnap.farbig', G, 'Farbe', 'einfarbig (Graustufen) oder vier Farblagen Cyan, Magenta, Gelb, Schwarz',
  ebene='werkzeug', einheit='an/aus', std=False, quelle=Q + ': 0x3ee9 = 1 einfarbig; farbig Split_CMYK (hoch)',
  modus='einfach')
P('photosnap.raster', G, 'Rasterpunkt', 'Größe eines Bildpunkts im gespeicherten Raster', ebene='werkzeug',
  einheit='mm', std=0.5, quelle=V, modus='automatisch')
P('zauberstab.stichart', 'Zauberstab', 'Stichart des nächsten Klicks', 'Fläche Step/Satin/Auto oder Umriss '
  'Lauf/Dreifach/Satinlinie', ebene='werkzeug', einheit='Auswahl', std='step',
  werte=['step', 'satin', 'auto', 'lauf', 'dreifach', 'satinlinie'],
  quelle='Handbuch S. 114–115 (Magic Wand: Stichart vor dem Klick)', modus='einfach')
G = 'Effekte'
Q = 'IMPORT.DLL Effekt-Datensätze (Analyse_Teil3/23)'
FX = ['tatami', 'muster', 'satin', 'satinlinie', 'satinflaeche']
P('objekt.effekte.verlauf.profil', G, 'Verlauf: Profil', 'Verlauf des Abstands über die Fläche bzw. entlang der Säule '
  '(Gradient Fill)', ebene='objekt', gilt=FX, o='effekte.verlauf.profil', einheit='Auswahl', std='linear',
  werte=['linear', 'abnehmend', 'konvex', 'konkav', 'exp_zunehmend', 'exp_abnehmend', 'konvex_konkav',
         'konkav_konvex'],
  quelle=Q + ': GradientSpacingEffect 0x34, Profil-Byte 1–4, Stil 4800680101 = Profil 1; Formeln und Richtung ' + V,
  modus='experte')
P('objekt.effekte.verlauf.min', G, 'Verlauf: Abstand dicht', 'kleinster Abstand', ebene='objekt', gilt=FX,
  o='effekte.verlauf.min', einheit='mm', std=0.4, bereich=[0.1, 10],
  quelle=Q + ': Stil 0x48 = 72 = 0,4 mm; Tomatos.ART gemessen (linear)', modus='experte')
P('objekt.effekte.verlauf.max', G, 'Verlauf: Abstand offen', 'größter Abstand (Maximum Spacing)', ebene='objekt',
  gilt=FX, o='effekte.verlauf.max', einheit='mm', std=2.0, bereich=[0.1, 10],
  quelle=Q + ': Stil 0x168 = 360 = 2,0 mm (0x2789/0x278a)', modus='experte')
P('objekt.effekte.rand.seite', G, 'Ausgefranster Rand: Seite', 'links, beide, rechts (in Stichrichtung; Satin: links = '
  'Seite A)', ebene='objekt', gilt=FX, o='effekte.rand.seite', einheit='Auswahl', std='rechts',
  werte=['links', 'rechts', 'beide'], quelle=Q + ': JaggedEdgeEffect 0x32, Seite 2 = rechts (Tomatos.ART: eine Seite '
  'gestreut)', modus='experte')
P('objekt.effekte.rand.spanne', G, 'Ausgefranster Rand: Spanne', 'so weit streuen die Enden (Span)', ebene='objekt',
  gilt=FX, o='effekte.rand.spanne', einheit='mm', std=1.5, bereich=[0.1, 10],
  quelle=Q + ': Stil 270 = 1,5 mm', modus='experte')
P('objekt.effekte.rand.glaette', G, 'Ausgefranster Rand: Glätte', '0 = rau (jede Reihe frei), 1 = glatt (Texture)',
  ebene='objekt', gilt=FX, o='effekte.rand.glaette', einheit='0 … 1', std=0.1, bereich=[0, 1],
  quelle=Q + ': Stil 0,1; Zufall/Glättung ' + V, modus='experte')
P('objekt.effekte.welle', G, 'Welle: Leitlinie', 'Reihen folgen der Linie (Wave Effect); Einschalten legt eine Linie '
  'mit Bogen an (welle_vorschlag)', ebene='objekt', gilt=['tatami', 'muster'], o='effekte.welle', einheit='Linie',
  std=None, quelle=Q + ': CurveFillEffect 0x6b/0x6d + OneCurveD2Wrap 0x6e; Vorschlag ' + V, modus='experte')

# ---------------------------------------------------------------- Schrift
G = 'Schrift'
T = ['text', 'ttf', 'monogramm']
P('schrift.alphabet', G, 'Alphabet', 'eines der 50 Original-Alphabete', ebene='objekt', gilt=['text'], o='alphabet',
  einheit='Auswahl', std=None, quelle='ESB-Alphabete (Analyse/daten/alphabets.md); Befehl alphabete_liste',
  modus='einfach')
P('schrift.ttf', G, 'Computer-Schrift', 'TrueType/OpenType-Schrift', ebene='objekt', gilt=['ttf'], o='schrift',
  einheit='Auswahl', std=None, quelle='Handbuch Kap. 21; Stichart ' + V, modus='einfach')
P('schrift.hoehe', G, 'Höhe', 'Buchstabenhöhe', ebene='objekt', gilt=T, o='hoehe', einheit='mm', std=10.0,
  bereich=[1, 200], quelle='Handbuch S. 165; Vorlage 0x3e88 (Monogramm 18 mm)', modus='einfach')
P('schrift.breite', G, 'Breite', 'Breite in % der Höhe', ebene='objekt', gilt=T, o='breite', einheit='%', std=100,
  bereich=[10, 400], quelle='Vorlage 0x3e89; Bereich ' + V)
P('schrift.abstand', G, 'Buchstabenabstand', 'ohne Angabe: Wert des Alphabets', ebene='objekt', gilt=T, o='abstand',
  einheit='mm', std=None, quelle='Alphabet-Kopf (Analyse_Teil3/09)')
P('schrift.wortabstand', G, 'Wortabstand', 'Breite des Leerzeichens', ebene='objekt', gilt=T, o='wortabstand',
  einheit='% der Höhe', std=25, bereich=[0, 200], quelle='Vorlage 0x3e85')
P('schrift.zeilenabstand', G, 'Zeilenabstand', 'Lücke zwischen den Zeilen', ebene='objekt', gilt=T, o='zeilenabstand',
  einheit='% der Höhe', std=60, bereich=[0, 300], quelle='Vorlage 0x3f2d')
P('schrift.ausrichtung', G, 'Ausrichtung', 'links, mitte, rechts, Blocksatz (nur mit fester Grundlinienlänge)',
  ebene='objekt', gilt=T, o='ausrichtung', einheit='Auswahl', std='mitte', werte=['links', 'mitte', 'rechts', 'block'],
  quelle='Lettering-Dialog Justification', modus='einfach')
P('schrift.kursiv', G, 'Kursiv', 'Neigung der Buchstaben', ebene='objekt', gilt=T, o='kursiv', einheit='°', std=0,
  bereich=[-45, 45], quelle='Lettering-Dialog Italic; Vorlage 0x3e8a', modus='einfach')
P('schrift.grundlinie.art', G, 'Grundlinie', 'gerade, Kreis/Oval/Bogen, senkrecht, eigene Kurve, Wappen (mehrfach)',
  ebene='objekt', gilt=T, o='grundlinie.art', einheit='Auswahl', std=None,
  werte=['kreis', 'gerade', 'senkrecht', 'kurve', 'mehrfach'], quelle='Dialog „Layout“ (DLG 1039)', modus='einfach')
P('schrift.grundlinie.radius', G, 'Radius', 'Kreisradius (Oval: Radius X)', ebene='objekt', gilt=T,
  o='grundlinie.radius', einheit='mm', std=50.0, bereich=[5, 200], quelle='Vorlage 0x3e8b (mittel)', modus='einfach')
P('schrift.grundlinie.lage', G, 'Oben/unten am Kreis', 'oben im Uhrzeiger, unten gegen den Uhrzeiger', ebene='objekt',
  gilt=T, o='grundlinie.lage', einheit='Auswahl', std='oben', werte=['oben', 'unten'], quelle='Circle CW/CCW',
  modus='einfach')
for k, name, einh, std in [('radius_y', 'Radius Y (Oval)', 'mm', None), ('winkel', 'Lage auf dem Kreis / Linienwinkel', '°', None),
                           ('neigung', 'Neigung des Ovals', '°', 0), ('bogen', 'Bogenwinkel', '°', None),
                           ('laenge', 'Länge (gerade)', 'mm', None),
                           ('stand', 'Text über/auf/unter der Linie', 'Auswahl', None),
                           ('punkte', 'Eigene Kurve', 'Punkte/Form', None)]:
    P(f'schrift.grundlinie.{k}', G, name, '', ebene='objekt', gilt=T, o=f'grundlinie.{k}', einheit=einh, std=std,
      werte=(['ueber', 'durch', 'unter'] if k == 'stand' else []), quelle='Dialog „Layout“ (DLG 1039)')
P('schrift.farben', G, 'Farbwechsel im Text', '^ im Text wechselt zur nächsten Farbe der Liste', ebene='objekt', gilt=T,
  o='farben', einheit='Farbliste', std=[], quelle='Handbuch S. 158', modus='einfach')
P('schrift.buchstaben', G, 'Einzelne Buchstaben', 'Farbe, verschieben, drehen, skalieren, Winkellinien je Buchstabe',
  ebene='objekt', gilt=T, o='buchstaben', einheit='Liste', std=[], quelle='Handbuch Kap. 19/20')
P('schrift.stichart', G, 'Stichart der Schrift', 'Satin (Alphabet), Step, Füllmuster oder automatisch (Satin, ab 8 mm '
  'Stichlänge Step); Computer-Schrift: Tatami, Lauf, Dreifach, Satin (Säulen wie die TrueType-Umwandlung) oder auto. '
  'Einfacher Modus: neue Texte „auto“, Schalter „Trotzdem Satin“', ebene='objekt',
  gilt=['text', 'monogramm', 'ttf'], o='stichart', einheit='Auswahl', std='satin',
  werte=['satin', 'auto', 'step', 'muster', 'tatami', 'lauf', 'dreifach'],
  quelle='Handbuch Kap. 20; Verteilung ' + V + '; ttf satin: EZSTITCH Turning (Analyse_Teil3/17); auto = eigene '
         'Erweiterung', modus='einfach')
P('schrift.auto_step_ab', G, 'Step ab Stichlänge (Stichart auto)', 'wird ein Satinstich der Schrift (nach Größe und '
  'Transformation) länger, stickt der ganze Text Step; darüber ohne „auto“ ein Hinweis', ebene='objekt',
  gilt=['text', 'monogramm', 'ttf'], einheit='mm', std=8.0,
  quelle='eigene Erweiterung; Grenze wie EasyStitch (NORMAL.AMT 0x3f54, Analyse/code/10 §C), Handbuch S. 118 '
         '(Satin nicht über 7 mm); Konstante schrift::AUTO_STEP_AB_MM', modus='automatisch')
P('schrift.elastisch', G, 'Verzerren (Elastic)', 'gerade (Wimpel) oder gebogen (Bogen), Griffe oben/unten',
  ebene='objekt', gilt=T, o='elastisch', einheit='Griffe mm', std=None, quelle='Handbuch Kap. 21; Kantenform ' + V,
  modus='einfach')
P('schrift.monogramm.initialen', G, 'Initialen', '2 oder 3 Buchstaben', ebene='objekt', gilt=['monogramm'],
  o='initialen', einheit='Text', std=None, quelle='Diamond2/Diamond3 (Analyse_Teil3/09 §7)', modus='einfach')
P('schrift.monogramm.rahmen', G, 'Monogramm-Rahmen', 'Raute, Zier oder keiner', ebene='objekt', gilt=['monogramm'],
  o='rahmen', einheit='Auswahl', std='keine', werte=['raute', 'zier', 'keine'], quelle='Analyse_Teil3/09 §7',
  modus='einfach')
P('schrift.monogramm.rahmen_farbe', G, 'Rahmenfarbe', '', ebene='objekt', gilt=['monogramm'], o='rahmen_farbe',
  einheit='Farbe', std=None, quelle='Farbwechsel ^', modus='einfach')
# Monogramm mit Rahmen aus dem Katalog (eigene Erweiterung, Nutzer 21.11.2026; stick-kern::monogramm,
# stick-vektor::monogramm, Rahmen Arbeitsdaten/monogramm_rahmen/)
QM = 'eigene Erweiterung (Katalog Arbeitsdaten/monogramm_rahmen, Entwurf des Nutzers 21.11.2026)'
P('schrift.monogramm.vorlage', G, 'Rahmen aus dem Katalog', 'einer der 47 Rahmen (Befehl monogramm_rahmen); mit '
  'Vorlage 1–3 Zeichen, Schrift der Vorlage, Initialen automatisch eingepasst', ebene='objekt', gilt=['monogramm'],
  o='vorlage', einheit='Name', std=None, quelle=QM, modus='einfach')
P('schrift.monogramm.schrift', G, 'Schrift der Initialen', 'Computer-Schrift (TTF/OTF, Google Fonts); ohne Angabe der '
  'Schriftvorschlag der Vorlage', ebene='objekt', gilt=['monogramm'], o='schrift', einheit='Name', std=None,
  quelle=QM, modus='einfach')
P('schrift.monogramm.alphabet', G, 'Stick-Alphabet der Initialen', 'statt der Computer-Schrift ein Original-Alphabet',
  ebene='objekt', gilt=['monogramm'], o='alphabet', einheit='Name', std=None, quelle=QM, modus='einfach')
P('schrift.monogramm.rahmen_groesse', G, 'Größe des Rahmens', 'längere Seite in mm; die Initialen wachsen mit; ohne '
  'Angabe wie gezeichnet', ebene='objekt', gilt=['monogramm'], o='rahmen_groesse', einheit='mm', std=None,
  bereich=[10, 200], quelle=QM + '; Rahmen gezeichnet 70–88 mm', modus='einfach')
P('schrift.monogramm.rahmen_farben', G, 'Garne des Rahmens', 'je Farbe der Vorlage (1–3) ein Garn; leer = Farben der '
  'Vorlage', ebene='objekt', gilt=['monogramm'], o='rahmen_farben', einheit='Farbliste', std=[], quelle=QM,
  modus='einfach')
P('schrift.monogramm.luecke', G, 'Lücke zwischen den Initialen', 'Anteil der größten Versalhöhe', ebene='objekt',
  gilt=['monogramm'], einheit='%', std=8, quelle='eigene Festlegung (monogramm::LUECKE_ANTEIL)', modus='automatisch')
P('schrift.monogramm.abstand', G, 'Abstand Initialen – Rahmen', 'die Initialen werden verkleinert, bis sie mit diesem '
  'Abstand in den Rahmen passen (höchstens auf 30 %)', ebene='objekt', gilt=['monogramm'], einheit='mm', std=2.0,
  quelle='eigene Festlegung (monogramm::ABSTAND, MIN_ANTEIL)', modus='automatisch')
P('schrift.monogramm.seite_anteil', G, 'Äußere Initialen bei 3 in einem Rahmen für 2', 'Höhe der äußeren Initialen '
  'im Verhältnis zur mittleren', ebene='objekt', gilt=['monogramm'], einheit='%', std=68,
  quelle='eigene Festlegung (monogramm::SEITE_ANTEIL; Verhältnis der Vorlagen mit 3 Initialen)', modus='automatisch')
P('schrift.faktor', G, 'Schrift: Auto-Spacing-Faktor', 'Schrift 10 % dichter', ebene='design', einheit='Faktor', std=0.9,
  quelle='Stil „Schrift“ (Analyse/code/10)', modus='automatisch')
P('schrift.unterlage', G, 'Schrift: Unterlage', 'Zickzack, wenn das Alphabet keine eigene hat', ebene='design',
  einheit='Auswahl', std='zickzack', quelle='Stil „Schrift“', modus='automatisch')
P('schrift.laufweg', G, 'Schrift: Laufweg', 'Stichlänge der Wege im Buchstaben', ebene='design', einheit='mm', std=3.0,
  quelle='Lauf 0x271c im Alphabet', modus='automatisch')

# ---------------------------------------------------------------- Satin automatisch (ROADMAP 4b.17, Analyse_Teil3/17)
G = 'Satinfläche (Stichrichtung automatisch)'
Q = 'Analyse_Teil3/17_auto_satin.md'
P('satinflaeche.knickwinkel', G, 'Säulen am Knick teilen', 'Innenwinkel, unter dem eine Säule am Knick geteilt wird '
  '(0 = nie)', ebene='objekt', gilt=['satinflaeche'], o='knickwinkel', einheit='°', std=0.0, bereich=[0, 179],
  quelle='Auto Digitizer teilt nicht (gemessen, ' + Q + '); TrueType-Umwandlung 87° (Break Angle, MOD1 0x101f5bc0)')
for k, name, w, einh, std, quelle in [
    ('abtast_grob', 'Randabstand höchstens', 'Randpunkte für die Zerlegung höchstens so weit auseinander', 'mm', 1.0,
     'EZSTITCH ShapeSample „low precision“ 180 (AutoFont-Generate)'),
    ('abtast_fein', 'Randabstand mindestens', 'feiner wird nicht abgetastet', 'mm', 0.05,
     'EZSTITCH ShapeSample „high precision“ 9'),
    ('abtast_anteil', 'Randabstand je Breite', 'Randpunkte im Abstand Anteil × örtliche Breite', 'Faktor', 0.1,
     'EZSTITCH ShapeSample „proportional precision“ 0,1'),
    ('eckwinkel', 'Eckerkennung', 'Randecken spitzer als das bekommen eine Stichlinie; Eck-Zweige werden abgeschnitten',
     '°', 120.0, 'Corner Detection Angle (TurningConversionValues, MOD1 0x101f5bc0)'),
    ('linien_toleranz', 'Dichte der Stichlinien', 'Abweichung der Mittellinie (× Breite), ab der eine neue Stichlinie '
     'kommt', 'Faktor', 0.3, 'eigene Festlegung, an 54 Säulen des Auto Digitizers eingestellt (' + Q + ')'),
]:
    P(f'satinflaeche.{k}', G, name, w, ebene='design', einheit=einh, std=std, quelle=quelle, modus='automatisch')

# ---------------------------------------------------------------- Bild → Umrisse
G = 'Bild → Umrisse'
for k, name, w, einh, std, modus, quelle in [
    ('breite_mm', 'Breite', 'Breite des Ergebnisses (Höhe folgt)', 'mm', 80.0, 'einfach', 'stick-vektor (eigene Festlegung)'),
    ('farben', 'Farben', 'Zahl der Farben im Bild nach der Reduktion (k-Means im Lab-Raum), auch die weggelassenen',
     'Anzahl', 8, 'einfach', 'stick-vektor (eigene Festlegung)'),
    ('vereinfachen', 'Vereinfachen', 'kantenerhaltendes Glätten vor der Farbreduktion (Kuwahara); 0 = aus',
     'Stufe 0–5', 2, 'einfach', 'Schritt nach Handbuch Kap. 12 „Image clean up“, Verfahren eigene Festlegung'),
    ('weglassen', 'Farben weglassen', 'Farben, die nicht gestickt werden; ohne Wahl: fast weiße häufigste Randfarbe',
     'Farbliste', 'weißer Hintergrund', 'einfach', 'Auto Digitizer „Omitted Colors“, „white background omitted“ '
     '(Handbuch S. 117)'),
    ('garnfarben', 'Garnfarben', 'Farben auf die nächsten Garne der Garnkarte des Designs ziehen (CIEDE2000); '
     'Oberfläche: aus bei der Karte „Default“ (nur 19 Grundfarben)', 'an/aus', True, 'einfach',
     'eigene Festlegung (Zuordnung wie `garnkarte`)'),
    ('fuellung', 'Stichart der Flächen', '„step“ alles Tatami, „auto“ wie EasyStitch (bis 8 mm Satin)', 'Auswahl',
     'step', 'experte', 'Auto Digitizer „Fills: Auto/Step“ (Handbuch S. 118); Standard Step eigene Festlegung'),
    ('min_flaeche', 'Kleinste Fläche', 'kleinere Flächen gehen in der Nachbarfarbe auf (keine Löcher)', 'mm²', 4.0,
     'experte', 'stick-vektor (eigene Festlegung)'),
    ('glaettung', 'Glättung', 'Toleranz der Vereinfachung der Umrisse', 'mm', 0.2, 'experte',
     'stick-vektor (eigene Festlegung)'),
    ('zugausgleich', 'Zugausgleich', 'für alle umgewandelten Flächen (fest)', 'mm', 0.40, 'automatisch',
     'Handbuch S. 116 „Change the pull compensation to 0.4mm when using Auto Digitizer“; EasyStitch-Stile 0x273d'),
    ('unterlage', 'Unterlage', 'an für alle umgewandelten Flächen (fest)', 'an/aus', True, 'automatisch',
     'Handbuch S. 116 „Always remember to apply underlays“'),
]:
    P(f'bild.{k}', G, name, w, ebene='werkzeug', einheit=einh, std=std, quelle=quelle, modus=modus)

# ---------------------------------------------------------------- SVG einlesen (ROADMAP 4b.17, Analyse_Teil3/18)
G = 'SVG einlesen'
for k, name, w, einh, std, modus, quelle in [
    ('breite', 'Breite', 'Breite des Ergebnisses (ohne Angabe: Größe aus der SVG, 1 px = 25,4/96 mm)', 'mm', None,
     'einfach', 'SVG-Norm (CSS 96 px je Zoll)'),
    ('aussparen', 'Überdeckung aussparen', 'Füllflächen sparen aus, was in der SVG später (darüber) gezeichnet ist',
     'an/aus', True, 'experte', 'eigene Festlegung (Bedeutung der SVG), exakt wie „Remove Overlay Stitching“'),
    ('kuerzeste_verbindung', 'Kürzeste Verbindung', 'Closest Join wie der Auto Digitizer', 'an/aus', True, 'experte',
     'Handbuch PDF-S. 207 (Auto Digitizer: closest join)'),
]:
    P(f'svg.{k}', G, name, w, ebene='werkzeug', einheit=einh, std=std, quelle=quelle, modus=modus)
Q = 'EasyStitch-Tabelle 0x3f54 in NORMAL.AMT (Analyse/code/10 §C)'
for k, name, w, einh, std, quelle in [
    ('linie_duenn', 'Dünne Linie bis', 'Laufstich (Thin Lines)', 'mm', 0.3, Q + ', 0x3f77 = 54 (hoch)'),
    ('linie_kraeftig', 'Kräftige Linie bis', 'Laufstich (Thick Lines)', 'mm', 2.0, Q + ', 360 (hoch)'),
    ('satin_bis', 'Satin bis', 'Satinfläche bzw. Satinlinie (Narrow Objects)', 'mm', 8.0, Q + ', 1440 (hoch)'),
    ('tatami1_bis', 'Tatami 1 bis', 'Tatami (Wide Objects), darüber Tatami (Other Objects)', 'mm', 12.0,
     Q + ', 2160 (hoch)'),
    ('min_flaeche', 'Kleinste Fläche', 'kleinere Flächen entfallen', 'mm²', 1.0,
     Q + ', 0x3f57 = 32 400 (Rolle nicht bestimmt – Annahme)'),
    ('lauf_laenge', 'Laufstich der Linien', 'Stichlänge (Thin/Thick Lines)', 'mm', 2.0,
     'Stile <PRESET_EASYSTITCH>Thin/Thick Lines 0x271d (hoch)'),
    ('tatami_abstand', 'Tatami-Abstand', 'Wide/Other Objects', 'mm', 0.45,
     'Stile <PRESET_EASYSTITCH>Wide/Other Objects 0x2716 (hoch)'),
    ('zugausgleich', 'Zugausgleich der Flächen', 'Narrow/Wide/Other Objects', 'mm', 0.40,
     'Stile <PRESET_EASYSTITCH>Narrow/Wide/Other 0x273d = 72 (hoch); Handbuch „Use 0.4 when using automatic digitizing“'),
]:
    P(f'easystitch.{k}', G, name, w, ebene='werkzeug', einheit=einh, std=std, quelle=quelle, modus='automatisch')

# ROADMAP 4b.16 Werte je Objekt: jeder Stil-Wert (`einstellungen.…`) lässt sich auch am Objekt überschreiben
# (`"einstellungen"` am Objekt, gleicher Pfad) – im Original trägt jedes Objekt eine eigene Kopie des Stil-Baums
# (Analyse_Teil3/03 §4, hoch). Felder direkt am Objekt (z. B. `abstand`) bleiben als Kurzform in `json_objekt`.
for e in K:
    d = e['json_design'] or ''
    if d.startswith('einstellungen.') and not d.startswith('einstellungen.stoff'):
        if e['ebene'] == 'design':
            e['ebene'] = 'beide'
        if e['json_objekt'] is None:
            e['json_objekt'] = d
        if not e['gilt_fuer']:
            e['gilt_fuer'] = list(ALLE)

ids = [e['id'] for e in K]
assert len(ids) == len(set(ids)), 'doppelte IDs'
with open(ZIEL, 'w', encoding='utf-8') as f:
    json.dump({'stand': '22.11.2026', 'parameter': K}, f, ensure_ascii=False, indent=1)
print(len(K), 'Parameter ->', os.path.normpath(ZIEL))
