//! ASCII-Protokoll der artista 165–185 (Stickmodul), übernommen aus den am Gerät erprobten Testprogrammen v3.6
//! (Lesen) und v4.1 (Schreiben). Fundstellen im Original (BEAGLink.dll) stehen an den Funktionen; Beschreibung in
//! `Analyse_Teil2/01`, `02`, `03` und `08`.

use crate::leitung::Leitung;

/// Befehlsfeld (Mailbox) der Maschine.
pub const BEFEHLSFELD: u32 = 0xFFFED0;
/// Flash-Bereiche, die nie beschrieben werden (wie WriteComBytes 0x100029d0).
const FLASH: [(u32, u32); 2] = [(0x28_0000, 0x3F_FFFF), (0xC0_0000, 0xDF_FFFF)];

/// Ergebnis eines Befehls im Befehlsfeld.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub ok: bool,
    /// Statuswort (−1 = nicht gelesen)
    pub wert: i32,
}

impl Status {
    /// Fehlerbits im höheren Byte (Testprogramme `Status-Text`, `Analyse_Teil2/02`).
    pub fn text(&self) -> String {
        let ah = (self.wert >> 8) & 0xFF;
        let mut t = Vec::new();
        if self.wert >= 0 {
            for (bit, s) in [
                (0x10, "keine Speicherkarte im Stickmodul"),
                (0x20, "Karte nicht formatiert"),
                (0x40, "keine Flashkarte"),
                (0x80, "Karte voll / zu viele Designs"),
            ] {
                if ah & bit != 0 {
                    t.push(s);
                }
            }
        }
        let wert = if self.wert < 0 {
            "–".to_string()
        } else {
            format!("0x{:04X}", self.wert)
        };
        if t.is_empty() {
            wert
        } else {
            format!("{wert} ({})", t.join(", "))
        }
    }
}

/// Fortschritt großer Lese-/Schreibvorgänge: (was, erledigt, gesamt).
pub type Fortschritt<'a> = &'a mut dyn FnMut(&str, usize, usize);

/// Verbindung zur Maschine über eine [`Leitung`], mit Mitschnitt.
pub struct Verbindung<L: Leitung> {
    pub(crate) l: L,
    /// aktuelle Baudrate des PCs
    pub baud: u32,
    /// Mitschnitt (wie `Mitschnitt_*.log` der Testprogramme)
    pub protokoll: Vec<String>,
    stumm: bool,
    /// große Bereiche nur mit `R` (32 Byte) lesen
    pub nur_r: bool,
    /// nur mit `W` (32 Byte) schreiben, nicht mit `P S`
    pub nur_w: bool,
}

fn zeichen(b: u8) -> String {
    if (0x20..0x7f).contains(&b) {
        format!("{b:02X} '{}'", b as char)
    } else {
        format!("{b:02X}")
    }
}

impl<L: Leitung> Verbindung<L> {
    pub fn neu(l: L, baud: u32) -> Verbindung<L> {
        Verbindung {
            l,
            baud,
            protokoll: Vec::new(),
            stumm: false,
            nur_r: false,
            nur_w: false,
        }
    }

    pub fn log(&mut self, t: impl Into<String>) {
        let ms = self.l.jetzt_ms();
        self.protokoll
            .push(format!("{:>9.3}  {}", ms as f64 / 1000.0, t.into()));
    }

    fn tx(&mut self, daten: &[u8]) -> Result<(), String> {
        if !self.stumm {
            if daten.len() == 1 {
                self.log(format!("PC  -> MASCH  {}", zeichen(daten[0])));
            } else {
                self.log(format!("PC  -> MASCH  {} Byte binär", daten.len()));
            }
        }
        self.l.senden(daten)
    }

    fn rx(&mut self, ms: u64) -> Option<u8> {
        let b = self.l.empfangen(ms);
        if let (Some(b), false) = (b, self.stumm) {
            self.log(format!("MASCH -> PC  {}", zeichen(b)));
        }
        b
    }

    /// Eingang leeren; wie das Original (0x100027d0): ein druckbares Zeichen dabei = „Puffer nicht leer“ (Fehler 14).
    fn eingang_leeren(&mut self) -> bool {
        let mut sauber = true;
        while let Some(b) = self.rx(0) {
            if (0x20..=0x7f).contains(&b) {
                sauber = false;
            }
        }
        sauber
    }

    /// Zeichen senden und Echo prüfen (0x100027d0).
    fn echo(&mut self, c: u8, ms: u64) -> Result<(), String> {
        if !self.eingang_leeren() {
            self.log("FEHLER 14 Puffer nicht leer");
            return Err("Fehler 14: Eingang nicht leer".into());
        }
        self.tx(&[c])?;
        match self.rx(ms) {
            Some(e) if e == c => Ok(()),
            e => {
                self.log(format!("FEHLER 15 falsches/kein Echo auf '{}' ({e:?})", c as char));
                Err(format!("Fehler 15: kein Echo auf '{}'", c as char))
            }
        }
    }

    /// Byte als zwei Hexziffern mit Echo (0x10002750).
    fn hex(&mut self, v: u8) -> Result<(), String> {
        let h = format!("{v:02X}");
        for c in h.bytes() {
            self.echo(c, 1000)?;
        }
        Ok(())
    }

    fn adresse(&mut self, adr: u32) -> Result<(), String> {
        self.hex((adr >> 16) as u8)?;
        self.hex((adr >> 8) as u8)?;
        self.hex(adr as u8)
    }

    /// Zwei Hexziffern empfangen (0x10002670, je 2000 ms).
    fn rx_hex(&mut self) -> Result<u8, String> {
        let mut v = 0u8;
        for _ in 0..2 {
            let c = self.rx(2000);
            let z = match c {
                Some(c @ b'0'..=b'9') => c - b'0',
                Some(c @ b'A'..=b'F') => c - b'A' + 10,
                _ => {
                    self.log(format!("FEHLER 16 keine Hexziffer ({c:?})"));
                    return Err("Fehler 16: keine Hexziffer".into());
                }
            };
            v = (v << 4) | z;
        }
        Ok(v)
    }

    fn abschluss_o(&mut self, ms: u64, wo: &str) -> Result<(), String> {
        match self.rx(ms) {
            Some(b'O') => Ok(()),
            o => {
                self.log(format!("FEHLER 16 'O' {wo} fehlt ({o:?})"));
                Err(format!("Fehler 16: 'O' {wo} fehlt"))
            }
        }
    }

    /// 32 Byte lesen: `R` + Adresse → 64 Hexziffern → `O` (Lese32Bytes 0x1000250a).
    pub fn lesen32(&mut self, adr: u32) -> Result<[u8; 32], String> {
        self.echo(b'R', 1000)?;
        self.adresse(adr)?;
        let mut buf = [0u8; 32];
        for b in buf.iter_mut() {
            *b = self.rx_hex()?;
        }
        self.abschluss_o(1200, "nach R")?;
        Ok(buf)
    }

    /// Beliebig viele Bytes mit `R` lesen.
    pub fn lesen(&mut self, adr: u32, len: usize) -> Result<Vec<u8>, String> {
        let mut v = Vec::with_capacity(len + 32);
        let mut a = adr;
        while v.len() < len {
            v.extend(self.lesen32(a)?);
            a += 32;
        }
        v.truncate(len);
        Ok(v)
    }

    /// 256 Byte lesen: `N` + Adresse → 256 Byte binär → `O` (0x10002880).
    pub fn lesen256(&mut self, adr: u32) -> Result<Vec<u8>, String> {
        self.echo(b'N', 1000)?;
        self.adresse(adr)?;
        let mut v = Vec::with_capacity(256);
        for i in 0..256 {
            match self.l.empfangen(3000) {
                Some(b) => v.push(b),
                None => {
                    self.log(format!("FEHLER 16 Block unvollständig ({i} von 256 Byte)"));
                    return Err("Fehler 16: Block unvollständig".into());
                }
            }
        }
        self.abschluss_o(1200, "nach N-Block")?;
        Ok(v)
    }

    /// Synchronisieren (0x10001d00 ohne Binärprobe): `R` `F` `?` mit Echo, alle 6 Versuche nächste Baudrate.
    pub fn synchronisieren(&mut self) -> bool {
        for v in 0..48 {
            if v % 6 == 5 {
                self.baud = match self.baud {
                    4800 => 19200,
                    19200 => 57600,
                    57600 => 115200,
                    _ => 4800,
                };
                self.log(format!("SYNC wechsle auf {} Baud", self.baud));
                if self.l.baud_setzen(self.baud).is_err() {
                    return false;
                }
            }
            if v > 0 {
                self.l.warten(50);
            }
            self.eingang_leeren();
            if self.tx(b"R").is_err() {
                return false;
            }
            let mut antwort = None;
            let mut versuche = 5;
            loop {
                match self.rx(0) {
                    Some(b'Q') => continue, // 'Q' wird übersprungen
                    Some(b) => {
                        antwort = Some(b);
                        break;
                    }
                    None => {
                        self.l.warten(20);
                        versuche -= 1;
                        if versuche == 0 {
                            break;
                        }
                    }
                }
            }
            if antwort == Some(b'R')
                && self.tx(b"F").is_ok()
                && self.rx(100) == Some(b'F')
                && self.tx(b"?").is_ok()
                && self.rx(100) == Some(b'?')
            {
                self.log(format!("SYNC ok nach {} Versuchen bei {} Baud", v + 1, self.baud));
                return true;
            }
        }
        false
    }

    /// Transfermodus: `T` `r` `M` `E` mit Echo (0x10005e30).
    pub fn transfer(&mut self) -> bool {
        b"TrME".iter().all(|&c| self.echo(c, 1000).is_ok())
    }

    /// Baudrate mit der Maschine wechseln (ChangeBaudrate 0x100057b0): `J` + Code, neue Baudrate, `B` `O` `S`, `E` `B`.
    pub fn baud_wechseln(&mut self, neu: u32) -> Result<(), String> {
        let code = match neu {
            19200 => 0x11,
            57600 => 0x05,
            115200 => 0x02,
            _ => return Err(format!("Baudrate {neu} gibt es nicht")),
        };
        if self.baud == neu {
            return Ok(());
        }
        self.echo(b'J', 1000)?;
        self.hex(code)?;
        self.baud = neu;
        self.l.baud_setzen(neu)?;
        let mut b_da = false;
        for _ in 0..20 {
            while self.rx(0).is_some() {}
            self.l.warten(100);
            if self.rx(0) == Some(b'B') {
                b_da = true;
                break;
            }
        }
        if !b_da {
            self.log("FEHLER 17 kein 'B' nach Baudwechsel");
            return Err("Fehler 17: kein 'B' nach dem Baudwechsel".into());
        }
        if self.rx(1200) != Some(b'O') || self.rx(1200) != Some(b'S') {
            self.log("FEHLER 16 'O' 'S' fehlen nach Baudwechsel");
            return Err("Fehler 16: 'O' 'S' fehlen nach dem Baudwechsel".into());
        }
        self.echo(b'E', 1000)?;
        self.echo(b'B', 1000)?;
        self.log(format!("BAUD jetzt {neu}"));
        Ok(())
    }

    /// Zum Stickmodul durchschalten (0x10005e30 mit Parameter 1): `T r M E`, 57600 Baud, `Y` `Q` → `O`.
    pub fn zum_stickmodul(&mut self) -> Result<(), String> {
        if !self.transfer() {
            return Err("T r M E vor dem Umschalten nicht bestätigt".into());
        }
        self.baud_wechseln(57600)?;
        self.l.warten(250);
        self.echo(b'Y', 1000)?;
        self.echo(b'Q', 1000)?;
        match self.rx(3000) {
            Some(b'O') => Ok(()),
            o => {
                self.log(format!("FEHLER 30 Stickmodul antwortet nicht mit 'O' ({o:?})"));
                Err("Stickmodul antwortet nicht (ist es angesteckt?)".into())
            }
        }
    }

    /// Bytes schreiben (0x10002b20): `W` + Adresse + Daten (Hex mit Echo) + `?`.
    pub fn schreiben(&mut self, adr: u32, daten: &[u8], ende_ms: u64) -> Result<(), String> {
        flash_pruefen(adr, daten.len())?;
        self.echo(b'W', 1000)?;
        self.adresse(adr)?;
        for &d in daten {
            self.hex(d)?;
        }
        self.echo(b'?', ende_ms)
    }

    /// 256 Byte schreiben (0x10002ce0): `P` `S` + Adressbits 23–8 → `O` `E` → 256 Byte binär → `O`.
    pub fn schreiben256(&mut self, adr: u32, daten: &[u8]) -> Result<(), String> {
        if daten.len() != 256 || adr & 0xFF != 0 {
            return Err("Seite nicht ausgerichtet".into());
        }
        flash_pruefen(adr, 256)?;
        self.echo(b'P', 1000)?;
        self.echo(b'S', 1000)?;
        self.hex((adr >> 16) as u8)?;
        self.hex((adr >> 8) as u8)?;
        self.abschluss_o(1200, "nach P S")?;
        if self.rx(1200) != Some(b'E') {
            self.log("FEHLER 16 'E' nach P S fehlt");
            return Err("Fehler 16: 'E' nach P S fehlt".into());
        }
        self.tx(daten)?;
        self.abschluss_o(3000, "nach dem 256-Byte-Block")
    }

    /// Befehl ins Befehlsfeld 0xFFFED0 (0x10002f00): Wort (Befehl | 1) schreiben, Status abfragen.
    pub fn befehl(&mut self, cmd: u16, t1: u64, t2: u64, flag: bool) -> Status {
        let w = cmd | 1;
        self.log(format!("BEFEHL 0x{cmd:X} an 0xFFFED0 (Flag {})", u8::from(flag)));
        if self.schreiben(BEFEHLSFELD, &w.to_be_bytes(), t1 + 5000).is_err() {
            return Status { ok: false, wert: -1 };
        }
        let start = self.l.jetzt_ms();
        let mut st = -1;
        while self.l.jetzt_ms() - start < t2 {
            let Ok(r) = self.lesen(BEFEHLSFELD, 2) else {
                return Status { ok: false, wert: -1 };
            };
            st = i32::from(r[0]) << 8 | i32::from(r[1]);
            let (al, ah) = (st & 0xFF, (st >> 8) & 0xFF);
            if al & 6 != 0 || (cmd == 0x100 && ah & 1 == 0) {
                self.log(format!("STATUS 0x{st:04X}"));
                let ok = if cmd == 8 || flag {
                    al & 2 != 0
                } else {
                    ah & 0xF0 == 0 && al & 4 == 0
                };
                return Status { ok, wert: st };
            }
            self.l.warten(100);
        }
        self.log(format!("STATUS Zeitüberschreitung, letzter Wert 0x{st:04X}"));
        Status { ok: false, wert: st }
    }

    /// Nach einem Fehler neu synchronisieren (`R` `F` `?` bei derselben Baudrate; bricht einen angefangenen Befehl ab).
    pub fn resync(&mut self) -> bool {
        let alt = self.stumm;
        self.stumm = false;
        let mut ok = false;
        for v in 0..10u64 {
            self.l.warten(200 + 100 * v);
            while self.rx(0).is_some() {}
            if self.tx(b"R").is_err() {
                break;
            }
            let mut a = self.rx(300);
            while a == Some(b'Q') {
                a = self.rx(300);
            }
            if a != Some(b'R') {
                continue;
            }
            if self.tx(b"F").is_err() || self.rx(300) != Some(b'F') {
                continue;
            }
            if self.tx(b"?").is_err() || self.rx(300) != Some(b'?') {
                continue;
            }
            self.l.warten(50);
            while self.rx(0).is_some() {}
            self.log(format!("RESYNC ok nach {} Versuchen", v + 1));
            ok = true;
            break;
        }
        if !ok {
            self.log("RESYNC fehlgeschlagen");
        }
        self.stumm = alt;
        ok
    }

    /// Großen Bereich lesen wie ReadComBytes (0x10002040): erstes Stück mit `R`, dann `N`-Blöcke, Rest mit `R`;
    /// jedes Stück bis zu 5-mal wiederholt.
    pub fn lesen_gross(&mut self, adr: u32, len: usize, was: &str, f: Fortschritt) -> Result<Vec<u8>, String> {
        let mut v: Vec<u8> = Vec::with_capacity(len);
        let start = self.l.jetzt_ms();
        let mut wdh = 0;
        self.stumm = true;
        while v.len() < len {
            let rest = len - v.len();
            let a = adr + v.len() as u32;
            let block = !self.nur_r && self.baud != 4800 && !v.is_empty() && rest >= 256;
            let mut stueck = None;
            for versuch in 0..=5 {
                if versuch > 0 {
                    wdh += 1;
                    self.log(format!(
                        "WIEDERHOLUNG {versuch} bei 0x{a:X} ({})",
                        if block { "N" } else { "R" }
                    ));
                    if !self.resync() {
                        break;
                    }
                }
                let r = if block {
                    self.lesen256(a)
                } else {
                    self.lesen32(a).map(|b| b.to_vec())
                };
                if let Ok(b) = r {
                    stueck = Some(b);
                    break;
                }
            }
            let Some(b) = stueck else {
                self.stumm = false;
                self.log(format!("FEHLER beim Lesen bei 0x{a:X} (Offset {})", v.len()));
                return Err(format!("{was}: Lesen bei Offset {} fehlgeschlagen", v.len()));
            };
            v.extend(&b[..b.len().min(rest)]);
            f(was, v.len(), len);
        }
        self.stumm = false;
        let s = (self.l.jetzt_ms() - start) as f64 / 1000.0;
        self.log(format!(
            "GELESEN {was}: {len} Byte ab 0x{adr:X} in {s:.1} s, Wiederholungen {wdh}"
        ));
        Ok(v)
    }

    /// Großen Bereich schreiben wie WriteComBytes (0x100029d0): bis zur 256-Byte-Grenze `W` mit ≤ 32 Byte, dann
    /// `P S`-Seiten, Rest mit `W`; jedes Stück bis zu 5-mal wiederholt. Flash-Bereiche nie.
    pub fn schreiben_gross(&mut self, adr: u32, daten: &[u8], was: &str, f: Fortschritt) -> Result<(), String> {
        flash_pruefen(adr, daten.len())?;
        let start = self.l.jetzt_ms();
        let (mut pos, mut wdh, mut seiten, mut kleine) = (0usize, 0, 0, 0);
        self.stumm = true;
        while pos < daten.len() {
            let a = adr + pos as u32;
            let rest = daten.len() - pos;
            let seite = !self.nur_w && self.baud != 4800 && a & 0xFF == 0 && rest >= 256;
            let n = if seite {
                256
            } else {
                let bis_grenze = 0x100 - (a & 0xFF) as usize;
                let n = rest.min(32);
                if rest >= bis_grenze + 0x100 && bis_grenze < n && bis_grenze > 0 {
                    bis_grenze
                } else {
                    n
                }
            };
            let stueck = &daten[pos..pos + n];
            let mut ok = false;
            for versuch in 0..=5 {
                if versuch > 0 {
                    wdh += 1;
                    self.log(format!(
                        "WIEDERHOLUNG {versuch} beim Schreiben bei 0x{a:X} ({})",
                        if seite { "P S" } else { "W" }
                    ));
                    if !self.resync() {
                        break;
                    }
                }
                let r = if seite {
                    self.schreiben256(a, stueck)
                } else {
                    self.schreiben(a, stueck, 1000)
                };
                if r.is_ok() {
                    ok = true;
                    break;
                }
            }
            if !ok {
                self.stumm = false;
                self.log(format!("FEHLER beim Schreiben bei 0x{a:X} (Offset {pos})"));
                return Err(format!("{was}: Schreiben bei Offset {pos} fehlgeschlagen"));
            }
            if seite {
                seiten += 1;
            } else {
                kleine += 1;
            }
            pos += n;
            f(was, pos, daten.len());
        }
        self.stumm = false;
        let s = (self.l.jetzt_ms() - start) as f64 / 1000.0;
        self.log(format!(
            "GESCHRIEBEN {was}: {} Byte ab 0x{adr:X} in {s:.1} s, {seiten} Seiten P S, {kleine} W-Stücke, Wiederholungen {wdh}",
            daten.len()
        ));
        Ok(())
    }

    /// PC-Modus verlassen wie das Original bei der artista 185 (TransparentMode(0) → 0x10005e30(0,1)): Befehl 0x100,
    /// danach `T r M E` mit Echo – erst dann ist das Display wieder an (Gerätetest 19.11.2026).
    pub fn verlassen(&mut self) -> bool {
        let r = self.befehl(0x100, 2000, 2000, true);
        self.log(format!("Befehl 0x100 Status {}", r.text()));
        let t = self.transfer();
        self.log(format!(
            "Abschluss T r M E: {}",
            if t { "bestätigt" } else { "NICHT bestätigt" }
        ));
        (r.ok || r.wert == 0) && t
    }
}

fn flash_pruefen(adr: u32, len: usize) -> Result<(), String> {
    let ende = adr as u64 + len as u64;
    for (von, bis) in FLASH {
        if (adr as u64) <= bis as u64 && ende > von as u64 {
            return Err(format!(
                "Bereich 0x{adr:X}–0x{ende:X} berührt Flash – wird nie beschrieben"
            ));
        }
    }
    Ok(())
}
