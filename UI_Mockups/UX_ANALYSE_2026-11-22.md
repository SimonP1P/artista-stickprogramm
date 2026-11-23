# UX-Analyse der Oberfläche (22.11.2026)

Auftrag des Nutzers: „alle Buttons im Expertenmodus analysieren … UX roasten … nichts ändern, nur auflisten – dann
entscheide ich.“ Dazu kommt die Rückmeldung der Leute an der Maschine: „Ich will nur auf Übertragen drücken und dann
soll das Ding alles selber machen … ein Fortschrittsbalken reicht … die Extra-Genehmigungen sind obsolet.“

**Stand:** nur Analyse, an der App ist nichts geändert. Grundlage sind Bildschirmfotos (`app/werkzeuge/bildschirmfoto.py`)
und der Code (`app/src`).

## 1. Senden an die Maschine – der größte Ärger

### 1.1 Heute (Expertenmodus)
| Schritt | Was der Anwender tut / sieht |
|---|---|
| 1 | Kopf „Senden …“ |
| 2 | Stufe „1 Anschluss“: Port wählen, „Maschine prüfen (nur lesen)“, Anleitung in 5 Punkten zum Rahmenwechsel. Nach einem Rahmenwechsel blockiert ein Häkchen „erledigt – Rahmen ist erkannt“. Dann „Plan erstellen (nur lesen)“ |
| 3 | Stufe „2 Plan prüfen“: Tabelle mit Bytes (Design-Daten, Vorschaubild, Name und Infos, Garnfarbtabelle; Spalte „Ziel: Speicher der Maschine, danach zurücklesen“), Häkchen „Vorschaubild senden“ und „Garnfarben senden“, Hinweis „kein 0x10/0x200“, „Schritte im Einzelnen (aus dem Kern)“. Dann „Bestätigen und schreiben“ |
| 4 | Stufe „3 Übertragen“: ein Gesamtbalken **plus vier Einzelbalken** mit Byte-Zählern |
| 5 | Stufe „4 Ergebnis“: vier Prüfzeilen („DATEN geschrieben und zurückgelesen – stimmt“, „ICON …“, „INFO-Felder …“, „Garnfarbtabelle … 58 Byte“), „PC-Modus verlassen: bestätigt“, Pfad des Mitschnitts. Dann „Jetzt sticken …“ (oder „Fertig – nicht sticken“) |
| 6 | Gefahrendialog mit **drei Pflicht-Häkchen** („Es ist jemand an der Maschine“, „Rahmen eingesetzt, Nadel oben“, „Hände weg vom Stickbereich“), dann „Sticken starten – Rahmen fährt“ |
| 7 | Das Design wird **ein zweites Mal komplett übertragen** und zurückgelesen, erst dann kommt Befehl 0x08 |

Bilanz: **5 Klicks + 3 Häkchen** (nach einem Rahmenwechsel 4), **zwei vollständige Übertragungen** (bei ~1,6 KB/s sind das
bei 30 KB gut 40 s statt 20 s), dazu viele technische Begriffe.

**Einfacher Modus:** Davor kommen noch „Weiter: Senden“ und im Assistenten ein eigener Anschluss-Block (Port, „Maschine
prüfen“, Anleitung). Danach „Plan anzeigen …“, und der Dialog beginnt **noch einmal** bei „1 Anschluss“. Das ergibt
7 Klicks + 3 Häkchen, Port und Anleitung kommen doppelt.

### 1.2 Das Original (Handbuch S. 265–266, „Stitching out designs“)
1. „Write to Machine“ anklicken – der Dialog zeigt Name und Bild des Designs.
2. „Stitch out Selected Design“ anklicken – „Writing file(s) to device“ (Fortschritt), dann eine Bestätigung.

Das sind **2 Klicks** ohne Häkchen. Der einzige Hinweis im Handbuch lautet: „Make sure the needle arm is up while
downloading“.

### 1.3 Woher die Hürden kommen
Die Stufen stammen aus der Zeit, als **Claude aus der Ferne** testete und niemand wusste, ob Schreiben der Maschine
schadet: `CLAUDE.md` Regel 1/1a, `app/ANFORDERUNGEN.md` §4, Entwurf v3 §7. Seit V1 (20.11.) und V10 (21.11.) ist
belegt, dass Schreiben, Zurücklesen und Sticken sauber funktionieren.

Für die Person an der Maschine sind die Hürden überflüssig:
- Sie steht ohnehin davor.
- Die Maschine verlangt selbst Nähfuß unten und den Start-Knopf.
- Nach Befehl 0x08 fährt der Rahmen nur in die Startlage, gestickt wird erst nach „Start“ an der Maschine (Gerätetest 19.11.).

Regel 1 gilt weiter für **Claude-Sitzungen**: Claude drückt den Knopf nicht ohne Zustimmung. Das betrifft aber nicht
die Oberfläche für Menschen.

### 1.4 Vorschlag (zur Entscheidung)
| # | Vorschlag | Aufwand |
|---|---|---|
| S1 | **Ein Knopf „Übertragen“** im Kopf (und im Assistenten Schritt 4). Ein Klick erledigt alles: verbinden, schreiben (Daten, Vorschaubild, Name, Garnfarben), zurücklesen, Stickmodus (0x08), PC-Modus verlassen. **Ein** Balken „Übertrage … 45 %“ mit „Abbrechen“. Am Ende: „Fertig – die Maschine ist bereit“ + Farbfolge mit Garnen + Spruch + „Schließen“ | ~1 Schritt (Kern kann das schon: `maschine_senden` mit `sticken: true`) |
| S2 | **Nur eine Übertragung** – die zweite fällt mit S1 weg | in S1 |
| S3 | **Anschluss automatisch:** den zuletzt genutzten Port nehmen, sonst den, an dem eine artista antwortet. Die Port-Auswahl gibt es nur noch im Menü „Maschine“ | ~½ Schritt |
| S4 | **Weg damit:** Plan-Stufe mit Byte-Tabelle, Häkchen „Vorschaubild/Garnfarben senden“ (immer an, wie im Original), Gefahrendialog mit 3 Häkchen, blockierendes Rahmenwechsel-Häkchen, Anleitung in 5 Punkten, „Fertig – nicht sticken“ | in S1 |
| S5 | **Rahmenwechsel:** höchstens eine Zeile ohne Häkchen: „Rahmen gewechselt? An der Maschine einmal «check» drücken.“ | klein |
| S6 | **Fehler** bleiben ausführlich (was passiert ist, was zu tun ist, „Nochmal“) | bleibt |
| S7 | **Technisches** (Einzelschritte, Prüfzeilen DATEN/ICON/INFO, Mitschnitt-Pfad, PC-Modus) nur noch unter „Details“ zum Aufklappen oder nur im Mitschnitt | klein |
| S8 | „Nur übertragen, nicht sticken“ entfällt. Höchstens im Menü „Maschine“ für Tests | klein |

## 2. Buttons im Expertenmodus

### 2.1 Falsch / veraltet (Fehler)
| # | Wo | Problem |
|---|---|---|
| F1 | Einfügen → rechte Spalte „Kommt noch – Platz ist vorgesehen“ | Alle fünf Einträge gibt es längst (Kreuzstich, Zauberstab/PhotoSnap, Ausrichten/Kranz/Kaleidoskop, eigene Rahmen/fremde Stickdateien, Vorlagen). Sie stehen grau mit „kommt noch“ da |
| F2 | Werkzeugleiste, letzter Knopf ✦ (gestrichelt) „Weitere Werkzeuge – kommen noch“ | Der Tooltip nennt „mit der Maus zeichnen, Motivfüllung, Effekte, Kreuzstich“ – alles fertig. Der Knopf öffnet nur das Einfügen-Menü |
| F3 | Inspektor, Feld „Ausrichtung“ (Text) und andere Felder ohne eigenes Bedienelement | Statt eines Bedienelements steht dort „im Reiter JSON bzw. in der Formansicht“ – der Anwender soll Rohdaten bearbeiten |
| F4 | Inspektor, Ein-/Ausstich | „automatisch (Griff in der Formansicht: E5)“ – „E5“ ist eine interne Etappe der Entwicklung |
| F5 | Inspektor unten | „Form: … – in der Formansicht bzw. im Reiter JSON“ |

### 2.2 Ergibt keinen Sinn: Objekte, die als fester Kreis oder feste Zickzacklinie in der Mitte landen
| # | Knopf | Was passiert | Besser |
|---|---|---|---|
| K1 | **Applikation** (Leiste + Einfügen) | Ein Kreis Ø 40 mm landet in der Rahmenmitte, egal was man vorhat | Applikation ist eine **Stichart einer Form**: die Form zeichnen (oder eine gewählte Form nehmen) → „Applikation“ – wie Fläche/Umriss |
| K2 | **Craft Stitches** (8 Einträge im Einfügen-Menü) | Fester Kreis Ø 40 mm oder feste Zickzacklinie in der Mitte | Rückstich, Stielstich und Stippling stehen **schon** im Stichart-Menü (doppelt). Blackwork, Candlewicking und Lacework sind Motiv-Vorlagen und gehören dort als Auswahl unter „Motive“ hin. Dann kann der ganze Block weg |
| K3 | **Knopfloch** | 20 mm in der Mitte | dort einfügen, wohin man klickt (oder zwei Klicks = Anfang/Ende) |
| K4 | **Text (Alphabet) / Text (Computer-Schrift)** | zwei Einträge; im Schrift-Dialog wechselt man ohnehin zwischen beiden | ein Eintrag „Text“ |

### 2.3 Doppelt
- Bild umwandeln, Zauberstab, PhotoSnap, Kreuzstich aus Bild, SVG einlesen: im Menü **Datei und** im Menü **Einfügen**.
- „Designs aus der Maschine“: im Menü Datei **und** im Menü Maschine.
- Rückstich, Stielstich, Stippling: im Craft-Block **und** in der Stichart-Liste (K2).
- **Leeres Design:** gleich drei Meldungen auf einmal: Kasten „Leeres Design“ mitten im Rahmen (Experte) bzw.
  „Noch nichts im Rahmen …“ (Einfach, liegt über dem Mittelkreuz), dazu „Noch keine Objekte …“ in der Liste und
  „Kein Objekt gewählt …“ im Inspektor. → **Der Rahmen darf einfach leer sein** (Nutzer). Der Kasten kann weg.
- **Simulation:** gleichzeitig ein lila Band oben, ein Knopf „Simulation“ und ein weiteres Band im Dialog.

### 2.4 Unklar oder umständlich
| # | Wo | Problem |
|---|---|---|
| U1 | Knöpfe **Fläche / Umriss** in der Leiste | Ein Knopf mit zwei Aufgaben: Umschalter (neue Formen werden Fläche oder Umriss) **und** Menü (Stichart wählen, ändert auch die Auswahl). Die Kopfzeile im Einfügen-Menü „Form zeichnen – Fläche: Step (Tatami)“ verwirrt zusätzlich |
| U2 | **Kreis / Oval** zeichnen | „Mitte klicken, dann Radius · Enter = Kreis, dritter Klick = Oval“ – so arbeitet kein gängiges Programm. Erwartet wird **Ziehen** (drücken, ziehen, loslassen; Umschalt = Kreis). Beim Rechteck ebenso (heute zwei Klicks). Linie, freie Form und Säule bleiben Punkt für Punkt wie im Original |
| U3 | **E / A** (Einstich/Ausstich) in der Leiste | Die Symbole sind kryptisch, gebraucht werden sie selten → ins Rechtsklick-Menü oder in den Inspektor |
| U4 | **„X“-Kästchen** hinter vielen Feldnamen (Kennzeichen „nur Experte“) | Sieht aus wie ein Schließen- oder Löschen-Knopf. Im Expertenmodus ist ohnehin alles Experte → weglassen |
| U5 | Reiter **JSON** | Rohdaten-Editor mit „Felder: SCHNITTSTELLE.md“ – ein Entwicklerwerkzeug. Nur mit Entwicklerschalter zeigen (wie das Browser-Inspect, `ARTISTA_INSPEKTOR=1`) |
| U6 | Reiter **Einzelwerte** | „Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt. Leeres Feld = geerbter Wert“, „geerbt (an)“, „je Stoff“, Knöpfe „Stoff“/„Standard“ und „Bereich …“ unter jedem Feld – sehr technisch. Besser: nur Werte, die vom Standard abweichen, hervorheben, Rest einklappen |
| U7 | Menü **Anordnen** | Unter „Stickreihenfolge“ stehen auch Gruppieren, Motiv speichern, Blackwork Run und Teil-Abdeckung. „Vor Objekt Nr. …“, „Nach Objekt Nr. …“, „Wie ausgewählt (Klickfolge)“ sind Nischen (Ziehen in der Liste gibt es) → in Gruppen ordnen, Nischen nach unten |
| U8 | Menü **Datei** | Enthält „Darstellung hell/dunkel umschalten“ und „Beispiel laden“ (Vorführung) – beides gehört nicht hierher |
| U9 | **Kopfzeile** | „130 x 100 · Woven Medium Weight · datei.json“ ist nur Text. Rahmen und Stoff ändert man nur im Inspektor → Design. Besser: anklickbar |
| U10 | Leiste der Vorschau | „↻ 0 ms“ = Rechenzeit – eine Entwickleranzeige |

### 2.5 Sprache und Entwicklerspuren
- „wie das Original“ (z. B. „Sprung (wie das Original)“) – der Anwender kennt „das Original“ nicht.
- Englisch gemischt: „Rotated“, Stoffnamen „Woven Medium Weight“, „Terry Medium Weight“, Garnkarte „Default“, Farbnamen
  „Powder Blue“, „Closest Join“, „Mirror-Merge“, „Partial Appliqué“, „Blackwork Run“, „Auto Spacing“.
- Hex-Codes: „nächstes Garn zu #004780“.
- Grammatik: „1 Farben“ (Objektliste, Plan „Garnfarben senden (1 Farben)“, Ergebnis).
- Hinweise wie „Abspielen zeigt nur die Reihenfolge – es ist keine Maschinen-Simulation“ und „Mitschnitt wird
  trotzdem gespeichert“.

## 3. Reihenfolge, falls umgesetzt wird (Vorschlag)
1. **Senden S1–S8** – der Ärger an der Maschine, ~1½ Schritte.
2. **Schnell erledigt** (je klein):
   - F1–F5 veraltete und falsche Einträge;
   - Kasten „Leeres Design“ weg;
   - X-Kästchen;
   - Entwickleranzeigen (ms, E5, Mitschnitt);
   - „1 Farben“;
   - Doppelte Menüeinträge.
3. **Mittel:**
   - K1 Applikation als Stichart;
   - K2 Craft-Block auflösen;
   - U2 Kreis/Rechteck ziehen;
   - U1 Stichart-Vorwahl klarer;
   - U5/U6 JSON und Einzelwerte.
4. **Sprache:** deutsche Namen für Stoffe, Rahmen und Fachbegriffe (englischer Originalname höchstens im Tooltip).
