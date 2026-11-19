# 06 – Das „DataItem“-Format und die Bedeutung der Tags

**Dateien:** [`dataitem.py`](dataitem.py), [`scan_dataitems.py`](scan_dataitems.py), [`find_tagtables.py`](find_tagtables.py), [`tag_labels.py`](tag_labels.py)
**Daten:** [`../daten/entschluesselt/tagnames.tsv`](../daten/entschluesselt/tagnames.tsv) (343 Namen aus Programm-Tabellen), [`tag_labels.txt`](../daten/entschluesselt/tag_labels.txt) (Zuordnung Tag → Dialogfeld)

## Binärformat (rekonstruiert aus WESSYS.DLL 0x1003a6c0)
Jeder Knoten hat einen 12-Byte-Kopf, Little-Endian:
```
u16 id_low | u16 id_high | u16 flags | u16 typ | u32 anzahl | Nutzdaten
```
| Typ | Nutzdaten | Bedeutung |
|---|---|---|
| 0 | – | leer/undefiniert |
| 1 | 4 Byte | long (vorzeichenbehaftet) |
| 2 | 4 Byte | ulong; **Längen in Wilcom-Einheiten** |
| 3 | 8 Byte | double |
| 4 | `anzahl` Byte | ANSI-String mit 0 am Ende |
| 5 | 2·`anzahl` Byte | UTF-16-String |
| 6 | `anzahl` Kindknoten | Liste |
| 7 | 1 Byte | bool/byte |

Ist `anzahl` gleich 0, folgen keine Nutzdaten. **flags**: 0x2000 = Wert gesetzt/gültig, 0x8000 = „Standardauswahl“ in Auswahllisten, 0x0001 = bisher als „deaktiviert“ gedeutet (z. B. 0x2001 bei der Zickzack-Unterlage). **Korrektur 19.11.2026 (mittel):** In jeder Liste eines Stils trägt genau ein Eintrag Bit 0x0001 (Unterlage Zickzack bzw. Tatami, Füllart Satin, Kontur Lauf) – wahrscheinlicher „ausgewählter Eintrag“, siehe [`../../Analyse_Teil3/01_stile_und_verbinder.md`](../../Analyse_Teil3/01_stile_und_verbinder.md) §5.

## Die Einheit: 1 mm = 180 Einheiten
Belege: 12,7 mm = 2286 (Maximalstich, passt zu DFL und Handbuch), Zugausgleich 0,2 mm = 36 (Handbuch S. 201), 7 mm Sprung = 1260 (Handbuch S. 204). Alle Längenwerte sind Vielfache von 9 (= 0,05 mm). Winkel stehen im Bogenmaß als double (1,5708 = 90°). Flächen stehen in Einheiten² (32 400 = 1 mm²).

## Benannte Tags (Namenstabellen in IMPORT/LIBINTF/AESLINK/WESSYS)
Auszug; die vollständige Liste steht in `tagnames.tsv`:
| Tag | Name | Tag | Name |
|---|---|---|---|
| 0x03e8 | no_stitches | 0x0461 | max_stitch_len |
| 0x03ef | no_colours | 0x0462 | max_jump_len |
| 0x03f4 | no_trims | 0x0465 | min_stitch_len |
| 0x03fe/03ff | thread_len / bobbin_Len | 0x0408 | fabric_thickness |
| 0x0403/0404 | height / width | 0x07d1 | emch_machine_type |
| 0x07d2 | machine_name | 0x07dd | trims_for_jump |
| 0x0cb6–0cbb | thrd_color / code / brand / description / density / idx | 0x19ab–19af | satin/step/fancy_spacing_factor, run_stitch_len_delta, pull_comp_delta (Stoff-„change_request“) |
| 0x1965–197f | let_params: let_text, let_height, let_width, let_alp_name, let_italic, let_spacing, let_stitch_len, let_stitch_spacing, let_stitch_type, let_underlay, let_pull_comp | 0x3fac–3fb9 | colorways … |

## Tags der Stich-Eigenschaften (0x27xx), selbst zugeordnet
Für die Stich-Eigenschaften gibt es **keine** Namenstabelle. Ich habe sie auf drei Wegen bestimmt:
**(U)** über den Dialogcode in DIALOG.DLL (Tag → Steuerelement → Beschriftung in RESENU.DLL, Werkzeug `tag_labels.py`); **(H)** über einen Wertevergleich mit dem Handbuch; **(S)** über die Struktur und Plausibilität der Werte.

| Tag | Liegt in | Bedeutung | Beleg | Sicherheit |
|---|---|---|---|---|
| 0x2710 | jede Stichart | Container „Stichart“ | S | hoch |
| 0x2711 | jede Stichart | Auto Jump: **maximale Stichlänge** (12,7 mm) | H S.204 | mittel-hoch |
| 0x2712 | jede Stichart | Auto Jump: **Sprunglänge** (7,0 mm) | H S.204 | mittel-hoch |
| 0x2758 | jede Stichart | Auto Jump ein/aus | S | mittel |
| **0x271c** | | **Laufstich (Single/Run)** | U | hoch |
| 0x271d | 0x271c | **Stichlänge („Run Length“)** | U (Feld 1332) | hoch |
| 0x271e | 0x271c | „Variable Run Length“ ein/aus | U (1819) | hoch |
| 0x2767 | 0x271c | **Min Length** (bei variabler Länge) | U (1821) | hoch |
| 0x2768 | 0x271c | **Chord Gap** (Sehnenabstand) | S + Dialog 1024 | mittel |
| **0x2721** | | **Satinfüllung** | S | hoch |
| 0x2723 | 0x2721 | **Satin-Stichabstand** (Spacing) | S; ändert sich je Stoff | hoch |
| 0x2722 | 0x2721 | 10 mm, vermutlich Auto-Split-Länge (Satin wird ab 10 mm geteilt) | S | niedrig-mittel |
| 0x275e | diverse | Auto Spacing ein (1) | S | niedrig |
| **0x2714** | | **Step-/Tatami-Füllung** | S | hoch |
| 0x2716 | 0x2714 | **Reihenabstand (Spacing)** | ändert sich je Stoff; Unterlage 3,0 mm | hoch |
| 0x2717 | 0x2714 | **Stichlänge** (4,0 mm) | H S.74–75 | hoch |
| 0x2718 | 0x2714 | Mindeststichlänge am Rand (0,4 mm) | S | mittel |
| 0x2719/0x271a | 0x2714 | Versatz-Anteile der Reihen (0,25 / 0,25 bzw. 0,75) | S | mittel |
| 0x27ae | 0x2714 | Step-Muster-Nr. (2) | S | mittel |
| **0x2724** | | **Zickzack** (Füllung und Unterlage) | S | mittel |
| 0x2725 / 0x2726 | 0x2724 | Abstand/Länge (2,0–7,0 mm) / Breite bzw. Rand (3,0 mm) | S | niedrig-mittel |
| **0x2727** | | vermutlich **Blanket** | S | niedrig |
| **0x272a** | | **Verbinder (Connectors)** | U | hoch |
| 0x272e | 0x272a | „Trim If Next Connector Longer Than“ | U | hoch |
| 0x2754 / 0x2755 | 0x272a | „Next / Previous Connector Longer Than“ (Vernähen) | U | hoch |
| 0x272c / 0x272d | 0x272a | Kontrollkästchen „Next / Previous Connector Longer Than“ (Bindung im Code nachgeprüft, [Teil3/01](../../Analyse_Teil3/01_stile_und_verbinder.md)) | Code | hoch |
| 0x2750 / 0x2733 / 0x276c | 0x272a | „After Trim / Color Change“ (Tie In), „Before Trim / Color Change“ (Tie Off), „Always Tie Off Last Object“ | Code | hoch |
| 0x2753 / 0x2752 / 0x2751 / 0x272b | 0x272a | Always Tie In / Always Tie Off / Always Trim / Trim If … | Code | hoch |
| 0x276c | 0x272a | „Always Tie Off Last Object“ | U | hoch |
| 0x2743 | Unterlage | **Zugausgleich** (0,2 mm) | H S.201 | hoch |
| 0x2759 | Unterlage | Winkel der Unterlage (90°) | S | mittel |
| 0x2769 / 0x276a | Unterlage | Unterlagen-Ränder (0,2 mm) | H S.199 | mittel |
| 0x2746 / 0x2744 / 0x2745 / 0x2747 / 0x275a | 0x27a7 | Unterlagearten: **Center Run/Mittellauf** (2,2 mm), Zickzack, Doppel-Zickzack, **Edge Walk**, Tatami-Unterlage („Step“) – korrigiert 19.11.2026 (Dialogcode + BERESENU-Texte + Stichgeometrie, `Analyse_Teil3/04`) | S + Code + Messung | hoch |
| 0x2735 / 0x2776 | 0x27ab | **Auto-Spacing-Tabelle Satin** (Breite → Abstand), siehe [10](10_algorithmen_schmale_bahnen.md) | S | hoch |
| 0x2736 | 0x2735 | **Abstandsfaktor** (je Stoff 0,95–1,13) | S | hoch |
| 0x273d | 0x27ab/0x273b | Zugausgleich je Stoff (0,20/0,25 mm) | S | mittel |
| 0x3f54 … 0x3f77 | EasyStitch | Auswahl der Stichart nach Objektbreite | S | hoch |
| 0x3eb3 | Stil | Name des Stils | S | hoch |

## Werkzeuge
- `dataitem.py`: Parser (`parse`) und Baum-Ausgabe (`dump`).
- `scan_dataitems.py`: sucht in beliebigen Binärdaten alle gültigen Bäume.
- `find_tagtables.py`: sucht in DLLs Tabellen der Form (Tag, Zeiger auf Namen, Typ).
- `tag_labels.py`: verknüpft DIALOG.DLL-Code (Member ↔ Steuerelement-ID, Member ↔ Tag) mit den Dialogbeschriftungen aus RESENU.DLL.
