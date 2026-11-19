"""Einheiten und feste Grenzen des Maschinenformats.

Koordinaten im Programm sind Millimeter (float), Ursprung = Rahmenmitte, +y zeigt nach oben
(wie im EXP-Format, Analyse/code/07_exp_format.md). Erst beim Export wird gerundet.
"""

# Interne Einheit der Original-Software: 1 mm = 180 Einheiten (Analyse/code/06_dataitem_und_tags.md).
# Wird gebraucht, sobald ART-Dateien gelesen/geschrieben werden.
EINHEITEN_PRO_MM = 180

# EXP: Schritte in 0,1 mm, vorzeichenbehaftet 8 Bit (Analyse/code/07_exp_format.md, hoch)
EXP_AUFLOESUNG_MM = 0.1
EXP_MAX_SCHRITT = 127

# Maschinenformat „Bernina“, Maschinentyp 17 (DEFAULTS.DFL, hoch)
MIN_STICH_MM = 0.3
MAX_STICH_MM = 12.7

# Auto Jump: Stiche > 12,7 mm werden in Sprünge zu 7,0 mm zerlegt (AMT 0x2711/0x2712, Handbuch S. 203, hoch)
AUTO_JUMP_TEILLAENGE_MM = 7.0


def mm_zu_einheiten(mm: float) -> int:
    return round(mm * EINHEITEN_PRO_MM)


def einheiten_zu_mm(einheiten: int) -> float:
    return einheiten / EINHEITEN_PRO_MM
