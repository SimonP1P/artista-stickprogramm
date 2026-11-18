# 07 – Das EXP-Format (Melco „Expanded“), wie BERNINA es liest

**Quelle:** BEAGLink.dll, EXP-Leser für Karten-Icons und Design-Infos (Schleife ab 0x10007c05, `A1xxGetEXPIcon`); dazu RESENU-Strings „Melco (*.EXP)“, „DOS Expanded (*.exp)“ (MagicBox) und der DFL-Eintrag „Bernina“
**Sicherheit:** hoch für die Leselogik (direkt aus dem Code); mittel für die Schreibseite (der Schreibcode liegt in MOD1/IMPORT und wurde nicht im Detail verfolgt)

## Aufbau
EXP hat **keinen Dateikopf**. Es ist eine reine Folge von **2-Byte-Datensätzen** `(dx, dy)`:
- `dx`, `dy` = **vorzeichenbehaftete 8-Bit-Werte** (−127…+127) in **0,1 mm**, relativ zum vorigen Stich.
- **+dy zeigt nach oben.** Der Leser rechnet `y -= dy` für Bildschirmkoordinaten.
- Größter Einzelschritt: 127 × 0,1 mm = **12,7 mm**. Genau das ist auch die maximale Stichlänge im BERNINA-Maschinenformat (DFL: 12,7 mm, Minimum 0,3 mm). Längere Stiche zerlegt die Software, siehe Auto Jump.

## Steuerbefehle (Escape 0x80)
Pseudo-Code des Lesers (Maschinencode übersetzt):
```c
read(b0, b1);
if (b0 == 0x80) {
    ctrl = b1;
    if (1 <= ctrl && ctrl <= 15) {
        code = (ctrl & 1) ? 0x01 : 0x04;     // ungerade → Farbwechsel/Stopp, gerade → Sprung (Move)
        read(dx, dy);                          // der folgende Datensatz ist die Bewegung
    } else if (ctrl == 0x82) {                 // erweiterter Sprung
        read(xh, xl, yh, yl);                  // 16 Bit, Big-Endian, vorzeichenbehaftet
        x += (xh<<8|xl); y -= (yh<<8|yl);
    } else { /* unbekannt: überspringen */ }
}
x += (int8)dx;  y -= (int8)dy;                 // Stich oder Sprung ausführen
bbox aktualisieren (für Breite/Höhe/Icon)
```
| Byte-Folge | Bedeutung |
|---|---|
| `dx dy` | normaler Stich |
| `80 01 dx dy` (allgemein: ungerade 1…15) | **Farbwechsel/Stopp**, danach Bewegung |
| `80 02 dx dy`, `80 04 dx dy` (gerade 2…14) | **Sprung** ohne Stich (beim Melco-Standard ist `80 02` Trim und `80 04` Sprung; der BERNINA-Leser behandelt beide als Sprung) |
| `80 82 xh xl yh yl` | **langer Sprung** mit 16-Bit-Koordinaten |

## Was die Software beim Schreiben beachtet (DEFAULTS.DFL, Format „Bernina“, Maschinentyp 17)
- Stiche **< 0,3 mm** werden entfernt, Stiche **> 12,7 mm** werden in Sprünge zerlegt.
- `trims_for_jump = 0`: kein Fadenschnitt über eine Sprungfolge (bei Melco 2).
- Zugelassene Ausgabeformate 16 und 7 (7 = EXP).
- Laut Handbuch (Anhang E, S. 368) ist **Melco EXP schreiben eine kostenpflichtige Option** („!“ = Cost option, Freischaltung über den Dongle). Lesen ist immer möglich.

## Begleitdateien
In dieser Version gibt es **keine INF-Datei** (keine Referenz in Code oder Strings). Farben und Vorschau holt ADX bei der artista 200 aus der ART-Datei (`GetARTFileInformation`, `GetARTFileThumbNail` → `\temp\icon.bmp`). Bei Karten der artista 165–185 erzeugt BEAGLink ein 1-Bit-Icon direkt aus den EXP-Stichen.
