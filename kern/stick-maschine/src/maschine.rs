//! Abläufe mit der artista 185: verbinden, Designliste, Design lesen, Garnfarbbereich lesen, senden, PC-Modus
//! verlassen – wie die Testprogramme v3.6 (`Test-Port`, `Read-DesignList`, `Read-Design`) und v4.1 (`Send-Design`).
//! Geschrieben wird nur in [`Maschine::senden`]; die Rückfrage beim Nutzer macht der Aufrufer zwischen
//! [`Maschine::planen`] und [`Maschine::senden`] (CLAUDE.md Regel 1a).

use crate::leitung::Leitung;
use crate::protokoll::{Fortschritt, Status, Verbindung};
use stick_formate::artista::{
    FarbGrenzen, Farbe, ICON_KOPF, Puffer, Sendedaten, ZEIGER_FARBTABELLE, ZEIGER_ZUSATZBLOCK, ZUSATZBLOCK_LAENGE,
    farbtabelle_lesen,
};

/// Verwaltungsblock bei 0xFFFD24 (`Analyse_Teil2/02`); Werte der artista 185: N = 28, INFO 0x24080 …
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verwaltung {
    pub n: u8,
    pub byte1: u8,
    pub groesse: u32,
    pub info: u32,
    pub auswahl: u32,
    pub icon: u32,
    pub daten: u32,
    pub ende_karte: u32,
    pub ende_speicher: u32,
    pub roh: Vec<u8>,
}

impl Verwaltung {
    fn aus(b: &[u8]) -> Verwaltung {
        let u = |o: usize| u32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        Verwaltung {
            n: b[0],
            byte1: b[1],
            groesse: u(2),
            info: u(6),
            auswahl: u(10),
            icon: u(14),
            daten: u(18),
            ende_karte: u(22),
            ende_speicher: u(26),
            roh: b.to_vec(),
        }
    }

    /// Plausibel wie in den Testprogrammen (N ≤ 28, Byte 1 ≤ 60, Größe ≤ 0x249F0).
    pub fn plausibel(&self) -> bool {
        self.n <= 0x1C && self.byte1 <= 0x3C && self.groesse <= 0x249F0
    }

    /// Größengrenze für ein Design (Original Schritt 5 bzw. 0x100044ad): Maximum aus Größe und Speicherende.
    pub fn grenze(&self) -> u64 {
        u64::from(self.groesse.max(self.ende_speicher))
    }
}

/// Speicherziel der Designliste (am Gerät bestätigt: 0xA0 = Maschinenspeicher, 0x50 = Karte).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speicher {
    Maschine,
    Karte,
}

/// Eintrag der Designliste.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListenEintrag {
    pub nr: usize,
    pub name: String,
    pub flags: u8,
    /// INFO + N+1 + k (Bedeutung offen; Original schreibt dort Farbanzahl − 1)
    pub feld_farben: u8,
}

/// Aus der Maschine gelesenes Design (Befehl 0x400 + Puffer DATEN + ICON).
#[derive(Debug, Clone)]
pub struct GelesenesDesign {
    pub nr: usize,
    pub name: String,
    /// Puffer DATEN (Kopf 6N+8 + Stiche + Anhang), wie `Design_NrXX_*.bin`
    pub puffer: Vec<u8>,
    /// Puffer ICON (Kopf 0xAE + Bitmap)
    pub icon: Vec<u8>,
}

/// Garnfarbbereich (Schritt 12): Zusatzblock und Bereich T mit der Tabelle, falls gültig.
#[derive(Debug, Clone)]
pub struct FarbBereich {
    pub zusatz_adr: u32,
    pub zusatz: Vec<u8>,
    pub t: u32,
    pub grenzen: FarbGrenzen,
    /// gültige Tabelle (ab T + Vorspann), sonst `None`
    pub tabelle: Option<Vec<u8>>,
    pub farben: Vec<Farbe>,
    /// Zustand in Worten („keine Tabelle …“, „gültige Tabelle mit 3 Farben“)
    pub text: String,
}

/// Was beim Senden geschrieben wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SendeOptionen {
    pub icon: bool,
    pub info: bool,
    pub farben: bool,
    /// danach Befehl 0x08 (sticken ohne speichern) – nur mit Person an der Maschine
    pub sticken: bool,
}

impl Default for SendeOptionen {
    fn default() -> Self {
        SendeOptionen {
            icon: true,
            info: true,
            farben: true,
            sticken: false,
        }
    }
}

/// Ergebnis von [`Maschine::planen`]: geplante Schreibzugriffe (zum Bestätigen) und Hinweise.
#[derive(Debug, Clone)]
pub struct SendePlan {
    pub schritte: Vec<String>,
    pub hinweise: Vec<String>,
    pub farbbereich: Option<FarbBereich>,
    /// Garnfarbtabelle wird geschrieben (Optionen + Prüfung wie das Original)
    pub farben_schreiben: bool,
    pub icon_schreiben: bool,
}

/// Ergebnis von [`Maschine::senden`].
#[derive(Debug, Clone)]
pub struct SendeBericht {
    pub schritte: Vec<String>,
    /// Status von Befehl 0x08, falls gestickt werden sollte
    pub sticken: Option<Status>,
}

/// Verbundene Maschine (Stickmodul erreicht, Verwaltungsblock gelesen).
pub struct Maschine<L: Leitung> {
    pub v: Verbindung<L>,
    pub firmware_naehmaschine: String,
    pub firmware_stickmodul: String,
    pub verwaltung: Verwaltung,
}

/// Fehler beim Verbinden, mit dem Mitschnitt bis dahin.
#[derive(Debug)]
pub struct Abbruch {
    pub fehler: String,
    pub protokoll: Vec<String>,
}

fn text(b: &[u8]) -> String {
    let ende = b.iter().position(|&c| c == 0).unwrap_or(b.len());
    b[..ende]
        .iter()
        .map(|&c| c as char)
        .collect::<String>()
        .trim()
        .to_string()
}

impl<L: Leitung> Maschine<L> {
    /// Verbinden wie `Test-Port`: `T r M E` (bis zu 3 Versuche, dazwischen synchronisieren), Nähmaschine → Stickmodul
    /// (`J 0 5`, `Y Q`), Verwaltungsblock lesen und prüfen.
    pub fn verbinden(l: L, start_baud: u32) -> Result<Maschine<L>, Abbruch> {
        let mut v = Verbindung::neu(l, start_baud);
        match Self::verbinden_innen(&mut v) {
            Ok((nm, sr, verwaltung)) => Ok(Maschine {
                v,
                firmware_naehmaschine: nm,
                firmware_stickmodul: sr,
                verwaltung,
            }),
            Err(fehler) => Err(Abbruch {
                fehler,
                protokoll: v.protokoll,
            }),
        }
    }

    fn verbinden_innen(v: &mut Verbindung<L>) -> Result<(String, String, Verwaltung), String> {
        let mut verbunden = false;
        for versuch in 1..=3 {
            v.log(format!("Versuch {versuch}: Transfermodus (T r M E)"));
            if v.transfer() {
                verbunden = true;
                break;
            }
            v.synchronisieren();
        }
        if !verbunden {
            return Err("Keine Verbindung (T r M E nicht bestätigt). Kabel, Port und Maschine prüfen.".into());
        }
        let mut fw = v.lesen(0x200100, 64)?;
        let mut naehmaschine = String::new();
        if fw.starts_with(b"NMMV") {
            naehmaschine = text(&fw[4..16]);
            let mut ok = false;
            for _ in 0..2 {
                if v.zum_stickmodul().is_ok() {
                    ok = true;
                    break;
                }
            }
            if !ok {
                return Err("Stickmodul nicht erreicht (angesteckt? Maschine aus- und einschalten).".into());
            }
            fw = v.lesen(0x200100, 64)?;
        }
        if !fw.starts_with(b"SRMV") {
            return Err(format!("Kennung SRMV nicht gefunden ('{}')", text(&fw[..16])));
        }
        let stickmodul = text(&fw[4..16]);
        let blk = v.lesen(0xFFFD24, 32)?;
        let verwaltung = Verwaltung::aus(&blk);
        v.log(format!("Verwaltungsblock {verwaltung:?}"));
        if !verwaltung.plausibel() {
            return Err("Verwaltungsblock nicht plausibel".into());
        }
        Ok((naehmaschine, stickmodul, verwaltung))
    }

    fn info_n(&self) -> (u32, u32, u32) {
        (
            self.verwaltung.info,
            u32::from(self.verwaltung.n),
            self.verwaltung.auswahl,
        )
    }

    /// Designliste lesen (0x100064c0 + 0x10004d10): Ziel, Auswahl, Bank, 0x20, je Bank Flags und Namen.
    pub fn designliste(&mut self, speicher: Speicher) -> Result<Vec<ListenEintrag>, String> {
        let (info, n, auswahl) = self.info_n();
        let ziel = if speicher == Speicher::Maschine { 0xA0 } else { 0x50 };
        let r = self.v.befehl(ziel, 1000, 4000, true);
        if !r.ok {
            return Err(format!("Ziel 0x{ziel:X} nicht bestätigt (Status {})", r.text()));
        }
        self.v.schreiben(auswahl, &[1], 1000)?;
        self.v.schreiben(auswahl + 5, &[0], 1000)?;
        self.v.befehl(0x30, 1000, 4000, true);
        let r = self.v.befehl(0x20, 1000, 2000, false);
        if !r.ok {
            return Err(format!("Liste laden nicht bestätigt (Status {})", r.text()));
        }
        let anzahl = self.v.lesen(info, 1)?[0] as usize;
        let mut liste = Vec::new();
        let mut bank = 0;
        while liste.len() < anzahl {
            self.v.schreiben(auswahl + 5, &[bank as u8], 1000)?;
            if bank >= 1 {
                let r = self.v.befehl(if bank == 1 { 0x60 } else { 0xC0 }, 1000, 4000, true);
                if !r.ok {
                    return Err(format!("Bank {bank} nicht geladen (Status {})", r.text()));
                }
            }
            let k = (anzahl - liste.len()).min(27);
            let farb = self.v.lesen(info + n + 1, 27)?;
            let flg = self.v.lesen(info + 2 * n + 1, 27)?;
            let nam = self.v.lesen(info + 3 * n + 1, k * 32)?;
            for i in 0..k {
                liste.push(ListenEintrag {
                    nr: 27 * bank + i + 1,
                    name: text(&nam[32 * i..32 * i + 32]),
                    flags: flg[i],
                    feld_farben: farb[i],
                });
            }
            bank += 1;
        }
        Ok(liste)
    }

    /// Ein gespeichertes Design lesen (0x10004310): Auswahl, Bank, 0x20, Befehl 0x400, Puffer DATEN und ICON.
    pub fn design_lesen(&mut self, nr: usize, f: Fortschritt) -> Result<GelesenesDesign, String> {
        let (info, n, auswahl) = self.info_n();
        if nr == 0 {
            return Err("Designnummer beginnt bei 1".into());
        }
        let idx = nr - 1;
        let bank = idx / 27;
        let bank_cmd = match idx {
            0..=26 => 0x30,
            27..=53 => 0x60,
            _ => 0xC0,
        };
        self.v.befehl(0xA0, 1000, 4000, true);
        self.v.schreiben(auswahl, &[(idx + 1) as u8], 1000)?;
        self.v.schreiben(auswahl + 5, &[bank as u8], 1000)?;
        self.v.befehl(bank_cmd, 1000, 4000, true);
        let r = self.v.befehl(0x20, 1000, 2000, false);
        if !r.ok {
            return Err(format!("Liste laden nicht bestätigt (Status {})", r.text()));
        }
        let anzahl = self.v.lesen(info, 1)?[0] as usize;
        if nr > anzahl {
            return Err(format!("Design Nr. {nr} gibt es nicht (Anzahl {anzahl})"));
        }
        let name = text(&self.v.lesen(info + 3 * n + 1 + 32 * (idx % 27) as u32, 32)?);
        self.v.schreiben(auswahl, &[(idx + 1) as u8], 1000)?;
        self.v.schreiben(auswahl + 5, &[bank as u8], 1000)?;
        let r = self.v.befehl(0x400, 3000, 4000, false);
        if !r.ok {
            return Err(format!("Befehl 0x400 nicht bestätigt (Status {})", r.text()));
        }
        let kopf_len = 6 * n as usize + 8;
        let kopf = self.v.lesen(self.verwaltung.daten, kopf_len)?;
        let ic = self.v.lesen(self.verwaltung.icon, 6)?;
        let o = 6 * n as usize;
        let len1 = u32::from_be_bytes([kopf[o], kopf[o + 1], kopf[o + 2], kopf[o + 3]]) as u64;
        let len2 = u32::from_be_bytes([kopf[o + 4], kopf[o + 5], kopf[o + 6], kopf[o + 7]]) as u64;
        // ganzes Icon (Kopf 0xAE + Bitmap; die Längenformel des Originals ergibt zu wenig, Analyse_Teil2/07)
        let icon_len = ICON_KOPF + ic[2] as usize * ic[3] as usize;
        if len1 <= 2 || len1 + len2 + icon_len as u64 + 0x142 >= self.verwaltung.grenze() {
            return Err(format!(
                "Längen nicht plausibel (Stiche {len1}, Anhang {len2}; Original: Fehler 19)"
            ));
        }
        let daten = self.v.lesen_gross(
            self.verwaltung.daten + kopf_len as u32,
            (len1 + len2) as usize,
            "Designdaten",
            f,
        )?;
        let icon = self.v.lesen_gross(self.verwaltung.icon, icon_len, "Icon", f)?;
        let mut puffer = kopf;
        puffer.extend(daten);
        Ok(GelesenesDesign { nr, name, puffer, icon })
    }

    /// Garnfarbbereich lesen (Schritt 12, Lesen 0x10004850): Zusatzblock und Bereich T, Tabelle prüfen.
    pub fn farbbereich_lesen(&mut self) -> Result<FarbBereich, String> {
        let pz = self.v.lesen(ZEIGER_ZUSATZBLOCK, 4)?;
        let pt = self.v.lesen(ZEIGER_FARBTABELLE, 4)?;
        let zusatz_adr = u32::from_be_bytes([pz[0], pz[1], pz[2], pz[3]]);
        let t = u32::from_be_bytes([pt[0], pt[1], pt[2], pt[3]]);
        let zusatz = self.v.lesen(zusatz_adr, ZUSATZBLOCK_LAENGE)?;
        let grenzen = FarbGrenzen::aus_zusatzblock(&zusatz)?;
        let k = grenzen.vorspann();
        let mut fb = FarbBereich {
            zusatz_adr,
            zusatz,
            t,
            grenzen,
            tabelle: None,
            farben: Vec::new(),
            text: String::new(),
        };
        let kopf = self.v.lesen(t, k + 0x14)?;
        let u = |o: usize| u32::from_be_bytes([kopf[o], kopf[o + 1], kopf[o + 2], kopf[o + 3]]);
        let (k1, k2, len, anz) = (u(k), u(k + 4), u(k + 8) as usize, u(k + 12) as usize);
        fb.text = if kopf[0] != 0 {
            format!("keine gültige Tabelle (Byte 0 von T = 0x{:02X})", kopf[0])
        } else if k1 != 1 || k2 != 8 {
            format!("keine Tabelle (Kennung {k1}/{k2} statt 1/8)")
        } else if len >= grenzen.groesse as usize || grenzen.groesse >= 0xFD00 || anz >= 0xFF {
            format!("Tabelle außerhalb der Grenzen (Länge {len}, Anzahl {anz})")
        } else {
            let mut tab = kopf[k..k + 8].to_vec();
            tab.extend(self.v.lesen(t + k as u32 + 8, len)?);
            let mut bereich = kopf[..k].to_vec();
            bereich.extend(&tab);
            fb.farben = farbtabelle_lesen(&bereich, grenzen)?;
            fb.tabelle = Some(tab);
            format!("gültige Tabelle mit {anz} Farben ({} Byte)", len + 8)
        };
        Ok(fb)
    }

    /// Senden planen (nur lesen): Daten prüfen wie das Original (Schritt 5, Fehler 31), Garnfarbbereich lesen und die
    /// Tabelle prüfen (0x10003abb ff.). Liefert die geplanten Schreibzugriffe zum Bestätigen.
    pub fn planen(&mut self, s: &Sendedaten, o: &SendeOptionen) -> Result<SendePlan, String> {
        let b = s.puffer.bytes();
        let n = usize::from(self.verwaltung.n);
        let p = Puffer::lesen(&b, n).map_err(|e| format!("Puffer passt nicht zu N = {n}: {e}"))?;
        if !p.endkennung() {
            return Err("Stichdaten enden nicht mit 80 81".into());
        }
        if !p.anhang.is_empty() && !p.anhang_gueltig() {
            return Err("Anhang ungültig (Containerlänge ≠ Länge 2)".into());
        }
        let groesse = (p.stiche.len() + p.anhang.len() + 2) as u64;
        if groesse >= self.verwaltung.grenze() {
            return Err(format!(
                "Design zu groß: {groesse} Byte ≥ {} (Original: Fehler 31 „Design is too big“)",
                self.verwaltung.grenze()
            ));
        }
        let mut plan = SendePlan {
            schritte: Vec::new(),
            hinweise: Vec::new(),
            farbbereich: None,
            farben_schreiben: false,
            icon_schreiben: o.icon,
        };
        let vw = &self.verwaltung;
        plan.schritte.push(format!(
            "DATEN 0x{:06X}: {} Byte (Kopf {}, Stiche {}, Anhang {})",
            vw.daten,
            b.len(),
            6 * n + 8,
            p.stiche.len(),
            p.anhang.len()
        ));
        if o.icon {
            plan.schritte.push(format!(
                "ICON 0x{:06X}: {} Byte (Kopf 0xAE + Bitmap 72 × 62)",
                vw.icon,
                s.icon.bytes().len()
            ));
        }
        if o.info {
            plan.schritte.push(format!(
                "INFO-Felder 0x{:06X}: Farbanzahl−1 = 0x{:02X}, Flags = 0x{:02X}, Name „{}“",
                vw.info + vw.n as u32 + 1,
                s.info.farben_minus_1,
                s.info.flags,
                s.name
            ));
        }
        let fb = self.farbbereich_lesen()?;
        if o.farben {
            match s.farbtabelle() {
                None => plan
                    .hinweise
                    .push("Design ohne Garnfarben – keine Garnfarbtabelle (wie das Original)".into()),
                Some(t) => match fb.grenzen.pruefen(s.farben.len(), &t) {
                    Err(e) => plan.hinweise.push(format!(
                        "Garnfarbtabelle wird nicht geschrieben (wie das Original): {e}"
                    )),
                    Ok(()) => {
                        plan.farben_schreiben = true;
                        plan.schritte.push(format!(
                            "Garnfarbtabelle 0x{:06X} (T + {}): {} Byte, {} Farben; danach 00 00 00 an T 0x{:06X}",
                            fb.t + fb.grenzen.vorspann() as u32,
                            fb.grenzen.vorspann(),
                            t.len(),
                            s.farben.len(),
                            fb.t
                        ));
                    }
                },
            }
        }
        if o.sticken {
            plan.schritte
                .push("Befehl 0x08: sticken ohne speichern (Rahmen fährt!)".into());
        }
        plan.hinweise
            .push("Es wird nichts dauerhaft gespeichert (kein 0x10/0x200).".into());
        plan.farbbereich = Some(fb);
        Ok(plan)
    }

    fn schreiben_pruefen(&mut self, adr: u32, d: &[u8], was: &str, f: Fortschritt) -> Result<(), String> {
        self.v.schreiben_gross(adr, d, was, f)?;
        let zurueck = self.v.lesen_gross(adr, d.len(), &format!("{was} zurücklesen"), f)?;
        if let Some(i) = zurueck.iter().zip(d).position(|(a, b)| a != b) {
            return Err(format!(
                "{was}: zurückgelesen weicht ab bei Offset {i} (gesendet 0x{:02X}, gelesen 0x{:02X})",
                d[i], zurueck[i]
            ));
        }
        Ok(())
    }

    /// Senden wie das Original (0x100032b0 ohne 0x10/0x200, „Embroider“): DATEN, ICON, INFO-Felder, Garnfarbtabelle
    /// (+ Freigabe an T), jeweils zurücklesen; optional Befehl 0x08. **Schreibt in die Maschine** – vorher
    /// [`Maschine::planen`] zeigen und bestätigen lassen.
    pub fn senden(&mut self, s: &Sendedaten, o: &SendeOptionen, f: Fortschritt) -> Result<SendeBericht, String> {
        let plan = self.planen(s, o)?;
        let mut bericht = SendeBericht {
            schritte: Vec::new(),
            sticken: None,
        };
        let vw = self.verwaltung.clone();
        let n = u32::from(vw.n);
        self.v.log("SENDEN beginnt");
        self.schreiben_pruefen(vw.daten, &s.puffer.bytes(), "DATEN", f)?;
        bericht
            .schritte
            .push("DATEN geschrieben und zurückgelesen – stimmt".into());
        if plan.icon_schreiben {
            self.schreiben_pruefen(vw.icon, &s.icon.bytes(), "ICON", f)?;
            bericht
                .schritte
                .push("ICON geschrieben und zurückgelesen – stimmt".into());
        }
        if o.info {
            self.v.schreiben(vw.info + n + 1, &[s.info.farben_minus_1], 1000)?;
            self.v.schreiben(vw.info + 2 * n + 1, &[s.info.flags], 1000)?;
            self.v.schreiben(vw.info + 3 * n + 1, &s.info.name, 1000)?;
            let a = self.v.lesen(vw.info + n + 1, 1)?;
            let b = self.v.lesen(vw.info + 2 * n + 1, 1)?;
            let c = self.v.lesen(vw.info + 3 * n + 1, 32)?;
            if a[0] != s.info.farben_minus_1 || b[0] != s.info.flags || c != s.info.name {
                return Err("INFO-Felder: zurückgelesen weicht ab".into());
            }
            bericht
                .schritte
                .push("INFO-Felder geschrieben und zurückgelesen – stimmen".into());
        }
        if plan.farben_schreiben
            && let (Some(fb), Some(t)) = (&plan.farbbereich, s.farbtabelle())
        {
            let (tz, ta) = (fb.t + fb.grenzen.vorspann() as u32, fb.t);
            self.schreiben_pruefen(tz, &t, "Garnfarbtabelle", f)?;
            self.v.schreiben(ta, &[0, 0, 0], 1000)?;
            let nach = self.farbbereich_lesen()?;
            if nach.tabelle.as_deref() != Some(t.as_slice()) {
                return Err(format!(
                    "Garnfarbtabelle nach dem Schreiben nicht gültig: {}",
                    nach.text
                ));
            }
            bericht.schritte.push(format!(
                "Garnfarbtabelle geschrieben und freigegeben – Maschine meldet: {}",
                nach.text
            ));
        }
        if o.sticken {
            let r = self.v.befehl(0x08, 3000, 7000, false);
            bericht.schritte.push(format!(
                "Befehl 0x08 (sticken ohne speichern): Status {}{}",
                r.text(),
                if r.ok {
                    " – Maschine hat das Design übernommen"
                } else {
                    " – nicht bestätigt"
                }
            ));
            bericht.sticken = Some(r);
        }
        Ok(bericht)
    }

    /// PC-Modus verlassen (0x100 + `T r M E`) und die Leitung zurückgeben.
    pub fn verlassen(mut self) -> (bool, Verbindung<L>) {
        let ok = self.v.verlassen();
        (ok, self.v)
    }
}
