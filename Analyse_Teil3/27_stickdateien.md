# 27 – Fremde Stickdateien, eigene Rahmen, ART mit Objekten

Stand 22.11.2026 (Nachtauftrag N7, ROADMAP 4b.13).

Quellen:
- Handbuch Kap. 28 „Processing Files of Different Formats“ (gedruckte S. 255–257) und Kap. 30 (Formatliste Magic
  Box);
- Strings von `gnfilter.dll` (Lese-/Schreibfilter des Originals, aus `setup.z`);
- Garnkarten `Thread Colors - *.tch` des Originals.

Kern:
- `stick-formate::stickdatei` (Leser);
- `stick-kern::stichdaten` (Objektart `stiche`);
- Befehle `stickdatei_lesen`, `stickdatei_endungen` (`stick-api/src/stickdateien.rs`).

## 1. Was das Original kann — Sicherheit: hoch (Handbuch, Strings)
- **Formate:** `gnfilter.dll` nennt `*.xxx`, `*.csd`, `*.sew`, `*.pes`, `*.hus`, `*.dst`, `*.exp`, `*.pcs` (STR
  0x52d5c ff.), dazu PEC („#PEC0001LA:“). Das Handbuch Kap. 30 listet dieselben (CSD, DST, EXP, HUS, PCM, PCS,
  PES/PEC, XXX, SEW).
  - **JEF und VP3 kennt die Version 4 nicht.** Sie kommen auf Wunsch des Nutzers dazu (heute verbreitete Formate,
    Entscheidung 22.11.2026).
- **Öffnen mit oder ohne Objektumwandlung** (S. 255–257):
  - „By default, stitch files are converted upon opening“ – „converted to a Grade C ART file“; nicht erkannte Teile
    werden „an object with general property only“.
  - Ohne Umwandlung: „The design opens as a collection of stitches with only general properties“ (Grade D). So bleibt
    das Design unverändert, einzelne Stiche lassen sich bearbeiten, „minor scaling and transforming actions“ gehen.
  - Empfehlung: Ohne Umwandlung „not … scale the design by more than ±5 %“.
- **PES speichern** mit Rahmenwahl (S. 257): kein Ziel. Nutzer: Schreiben bleibt EXP und Senden an die 185.

## 2. Umsetzung: Grade D — Entscheidung des Nutzers „nur Stiche, ohne Objekte“
**Objektart `stiche`** (`format.rs` `SticheJson`, `stichdaten.rs`):
- je Farbblock der Datei ein Objekt mit den Einstichen in Zügen; ein Zug endet an einem Sprung oder Fadenschnitt;
- gespeichert als Base64 (i16 in 0,1 mm, Zug-Trenner −32768; eigene Festlegung);
- die Mitte des Designs liegt auf der Rahmenmitte.

Berechnen:
- Die Züge werden unverändert Stiche.
- **Kein zusätzliches Vernähen**, weil die Datei ihr eigenes hat: Verbinder des Teils mit `vernaeh_stiche = 0`.
- Es gelten die Bernina-Regeln wie für jedes Design: Mindeststich 0,3 mm, Auto Jump über 12,7 mm (`bernina_regeln`).
  Sie entsprechen dem Maschinenformat des Originals.

Transformationen bewegen die Einstiche, Effekte wie die Welle verformen sie.
- Ab ±5 % Größenänderung (Singulärwerte der Abbildung aus Objekt, Gruppe und Design) meldet `design_berechnen` den
  Hinweis `stichdaten_skaliert` wie im Handbuch.
- Form für die Oberfläche: konvexe Hülle der Einstiche (Auswahl, Griffe).

Rahmen beim Öffnen: der kleinste der 185, in den das Rechteck des Designs passt. Der Freiarm-Rahmen kommt nur, wenn
nichts anderes passt (eigene Festlegung). Passt nichts, gilt der größte Rahmen mit Hinweis.

## 3. Formate (Aufbau: allgemein bekannte Formatbeschreibungen; Prüfung mit selbst gebauten Dateien)
| Format | Aufbau | Farben | Sicherheit |
|---|---|---|---|
| DST (Tajima) | 512 Byte Kopf (`LA:` Name), Datensätze 3 Byte mit Bits ±1/3/9/27/81 je Achse, Byte 3: `F3` Ende, `C3` Farbwechsel (Stopp), `83` Sprung; 0,1 mm, +y oben | keine → Standardfolge `Default.tch` | hoch (Kodierung für alle Werte −121 … 121 geprüft) |
| PES / PEC (Brother) | `#PES` + Version, Verweis (u32 bei 8) auf den PEC-Teil; PEC: `LA:` Name, bei 48 Zahl der Farbwechsel, ab 49 Farbnummern, ab 0x210 Stiche: 7-Bit-Werte oder 12-Bit mit Kennbits (0x80 lang, 0x20 Schnitt, 0x10 Sprung), `FE B0 xx` Farbwechsel, `FF 00` Ende; +y unten | PEC-Nummer 1–64 → Name (Reihenfolge des PEC-Formats) → RGB aus **`Thread Colors - PES.tch` des Originals** (alle 64 Namen dort vorhanden) | hoch (Aufbau), Farben hoch |
| JEF (Janome) | u32 Anfang der Stiche, u32 bei 24 Zahl der Farben, ab 116 Farbnummern (u32); Stiche 2 Byte (+y oben), `80 01 dx dy` Farbwechsel, `80 02 dx dy` Sprung, `80 10` Ende | Janome-Nummer 1–30 → Name; RGB aus **`Thread Colors - SEW.tch` des Originals** (Janome) | Aufbau hoch, Farbnummern mittel (ab 31: Standardfolge mit Hinweis) |
| VP3 (Viking/Pfaff) | `%vsm%`, Texte mit 16-Bit-Länge, Designmitte (1/1000 mm), Farbblöcke `00 05 00` + Länge + Anfang + Garn (RGB, Nummer, Name, Marke) + Stiche (2 Byte, `80 01` langer Stich mit 16 Bit, `80 03` Schnitt) | RGB aus der Datei | **niedrig: ohne echte Beispieldatei** – Lesefehler melden statt raten |
| EXP | wie bisher (`stick-formate::exp`) | keine → Standardfolge | hoch |
| HUS (Viking) | Stiche, x und y getrennt **komprimiert** (eigenes Verfahren) | `Thread Colors - HUS.tch` (29 Farben) | **offen** – ohne Beispieldatei nicht nachprüfbar; Frage an den Nutzer |

Eine freie Formatbibliothek (pyembroidery) als Gegenprobe ließ sich in dieser Sitzung nicht installieren (Freigabe
fehlt). Mit je einer echten Datei pro Format lassen sich die Leser endgültig prüfen.

## 4. Offen (Stickdateien)
- HUS lesen (Dekompression) – braucht eine Beispieldatei.
- VP3 an einer echten Datei prüfen; JEF-Farbnummern über 30.
- Objektumwandlung (Grade C) – vom Nutzer nicht gewünscht (Entscheidung 22.11.2026).
- Einzelne Stiche bearbeiten (Stitch Edit) – nicht umgesetzt.

## 5. Eigene Rahmen (N7b) — Handbuch Kap. 6 S. 49–52 (Sicherheit hoch für die Bedienung)
Original: Options > Hoop, „New Hoop“, Format **Rectangle** (Height, Width), **Oval** (Rectangle Height, Total Height,
Width), **Sliding** (Sewing Field, Total Height, Width) und **3-Position** (Höhen je Position); „Save Hoop As“ mit Namen.
Rechteck: gestrichelte schwarze Linie; Oval: schwarzer gestrichelter Umriss.

Umsetzung:
- **Rechteck und Oval.** Sliding und 3-Position sind Rahmen anderer Maschinen und für die 185 kein Ziel.
- Oval im Kern (`Rahmen::eigener`): gerader Mittelteil `gerade` mm hoch, oben und unten je eine Halbellipse bis zur
  Gesamthöhe; `gerade` = 0 ergibt eine Ellipse. Die genaue Rundung des Originals steht nicht im Handbuch (eigene
  Festlegung, mittel). Der Oval-Rahmen der 185 selbst kommt weiter aus `hooplibr.dsc`.
- Maße 10 … 1000 mm (eigene Festlegung). Gestalten geht damit auch größer als der MEGA-Rahmen.
- Ablage `<Datenordner>/rahmen.json` (eigene Festlegung; das Original schreibt `hooplibr.dsc`). Ein Design mit eigenem
  Rahmen trägt dessen Maße im Feld `eigener_rahmen` mit und lässt sich darum auch ohne die Ablage öffnen.
- **Senden (Entscheidung des Nutzers 22.11.2026):**
  - Die artista 185 kennt eigene Rahmen nicht. Das Handbuch sagt sinngemäß schon zum gedrehten Rahmen: „When you
    send the design to the machine, however, it does not recognize the hoop in the rotated position“ (S. 49).
  - Deshalb sucht das Programm den kleinsten Rahmen der 185, in den alle Einstiche passen (Freiarm zuletzt), und nennt
    ihn im Plan: „bitte diesen Rahmen einspannen“.
  - Passt keiner, wird nicht gesendet (Fehlercode `ausserhalb`).
  - Gesendet werden dieselben Daten wie sonst: die Maschine bekommt Stiche relativ zur Mitte, keinen Rahmen.
- Oberfläche (Modus E):
  - Assistent Schritt 1: eigene Rahmen in der Liste, Knopf „Eigener Rahmen …“;
  - Inspektor, Reiter Design: Auswahl mit den Gruppen „artista 185“ und „Eigene Rahmen“, Knopf „Eigene …“;
  - Dialog mit Vorschau neben dem MEGA-Rahmen.
