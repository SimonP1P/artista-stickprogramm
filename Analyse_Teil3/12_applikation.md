# 12 – Applikation im Original (20.11.2026)

**Frage:** Wie erzeugt die Original-Software eine Applikation (Stoffteil aufnähen), mit welchen Lagen und Werten?
**Quellen:** Handbuch Kap. 27 „Digitizing for Appliqué“ (PDF-S. 245–250), Garnkarte `Applique.tch`, Original-Alphabet
„Easy Appliqué.esb“ (gelesen mit `Arbeitsdaten/werkzeuge/esb_lesen.py` / `esb_satin.py`), Symbole in MOD1.DLL.

## 1. Lagen und Stopps — Sicherheit: hoch (Handbuch)
Auto Appliqué erzeugt je Applikation bis zu vier Lagen: **Platzierungslinie**, **Schnittlinie** (nur bei „Trim in
Place“; bei „Pre-cut“ keine), **Heftstich** (Zickzack) und **Abdeckstich** (Satin oder Blanket). Die Maschine hält
zwischen den Lagen an: Stoff auflegen nach der Platzierungslinie, zuschneiden nach Schnitt-/Heftlinie, dann Abdecken.
Einstellungen (Objekt-Eigenschaften, Reiter Appliqué): Abdeckart Satin/Blanket, Breite, Abstand, Heftstich-Breite und
-Abstand, Versatz von Abdeck- und Heftstich gegen die Platzierungslinie (Schieberegler, innen/außen; Blanket immer
innen), Stil „Trim in Place“/„Pre-cut“. Teil-Applikation („Partial Appliqué“) für überlappende Stoffteile.

## 2. Die Stopps als „Garne“ — Sicherheit: hoch
Die Original-Garnkarte `Applique.tch` enthält genau drei Einträge: 1 „Applique Guide“ (255,60,172), 2 „Place
Applique & Tack“ (26,140,103), 3 „Trim Applique & Cover“ (85,0,170). Als Farben der Lagen erzeugen sie die Stopps und
sagen an der Maschine (Display, Garnfarbtabelle), was als Nächstes zu tun ist.

## 3. Werte aus „Easy Appliqué“ — Sicherheit: mittel (ein Alphabet, nicht die Auto-Appliqué-Vorgabe)
Buchstabe „A“ (Referenzgröße des Alphabets): je Umriss (außen und Loch)
| Lage | Objekt | Wert |
|---|---|---|
| Platzierung | EmbroideryCurve, Lauf 0x271c | Stichlänge 360 = **2,0 mm** (variabel) |
| Heften | EmbroideryCurve, Zickzack 0x2724 + Abstand 0x15 | Breite 540 = **3,0 mm**, Abstand 360 = **2,0 mm** |
| Abdecken | EmbroideryPatch (Säule mit Winkellinien entlang des Umrisses) | Breite **3,0 mm** bzw. **4,2 mm** (zweite Lage) |
Die Vorgabe von Auto Appliqué selbst steht in `AutoAppliqueValues` (MOD1: `DesignDocument::GetCurrentAppliqueValues`)
und ist noch nicht gelesen.

## 4. Umsetzung im Kern — `kern/stick-kern/src/applikation.rs`
Objekt `"art": "applikation"` (Umriss, Löcher, Farbe): alle Platzierungslinien (Lauf 2,0 mm, „Applique Guide“) →
alle Heftstiche (Zickzack 3,0 mm / 2,0 mm, „Place Applique & Tack“) → alle Abdeckstiche (Satin **2,5 mm** mittig auf
dem Umriss – Handbuch-Beispiel, eigene Festlegung – in der gewählten Farbe). Beim Senden bekommen die Lagen die Garne
der Karte `Applique.tch` (genaue Farbgleichheit), die Abdeckung das nächste Garn der gewählten Karte.

## 5. Offen
Schnittlinie („Trim in Place“), Blanket-Abdeckung, Versatz innen/außen, Teil-Applikation, Reihenfolge mehrerer
Applikationen (Original: `AppliqueOverlapProcessor`), Vorgabewerte aus `AutoAppliqueValues`.
