//! Simulierte artista 185 mit Stickmodul – wie die Simulation der Testprogramme (`Sim-Rx`, v3.6/v4.1), Werte wie
//! am echten Gerät gelesen (Verwaltungsblock 18./19.11.2026, Zusatzblock und Garnfarbtabelle 20.11.2026).
//! Die Uhr ist virtuell: Warten kostet keine echte Zeit.

use crate::leitung::Leitung;
use std::collections::{HashMap, VecDeque};
use stick_formate::artista::{
    CHUNK_FARBEN, CHUNK_KOPF, Chunk, Container, Farbe, Icon, Kopf, Puffer, farbtabelle_maschine,
};
use stick_formate::exp::exp_lesen;

/// Einstellungen der Simulation.
#[derive(Debug, Clone)]
pub struct SimOptionen {
    /// Baudrate, auf der die Maschine anfangs steht (nach Aus/Ein 19200; nach einer Sitzung 57600)
    pub baud: u32,
    /// Stickmodul fehlt
    pub ohne_modul: bool,
    /// Anzahl Designs im Maschinenspeicher
    pub designs: usize,
    /// Karte im Stickmodul
    pub mit_karte: bool,
    /// beim n-ten `N` geht ein Echo verloren (0 = nie)
    pub stoerung_n: usize,
    /// beim n-ten `P S` fehlt das `E` (0 = nie)
    pub stoerung_ps: usize,
    /// höchste Farbanzahl im Zusatzblock (echte 185: 60)
    pub max_farben: u8,
}

impl Default for SimOptionen {
    fn default() -> Self {
        SimOptionen {
            baud: 19200,
            ohne_modul: false,
            designs: 30,
            mit_karte: false,
            stoerung_n: 0,
            stoerung_ps: 0,
            max_farben: 60,
        }
    }
}

const INFO: u32 = 0x24080;
const AUSWAHL: u32 = 0x201EA;
const ICON: u32 = 0x24480;
const DATEN: u32 = 0x28E9A;
const ZUSATZ: u32 = 0x20772;
const T: u32 = 0x2083A;
const N: u32 = 28;

pub struct Simulation {
    o: SimOptionen,
    pc_baud: u32,
    masch_baud: u32,
    baud_wartet: Option<(u32, u64)>,
    uhr: u64,
    q: VecDeque<u8>,
    mem: HashMap<u32, u8>,
    mem_nm: HashMap<u32, u8>,
    stickmodul: bool,
    transfer: bool,
    letzte: Vec<u8>,
    adr_ziffern: usize,
    hex: String,
    block: bool,
    n_zahl: usize,
    w: bool,
    w_hex: String,
    p: u8,
    p_hex: String,
    p_adr: u32,
    pub p_zahl: usize,
    p_daten: Vec<u8>,
    ziel: u16,
    /// Befehl 0x08 erhalten
    pub gestickt: bool,
    /// alle Befehle ans Befehlsfeld
    pub befehle: Vec<u16>,
}

impl Simulation {
    pub fn neu(o: SimOptionen) -> Simulation {
        let mut s = Simulation {
            pc_baud: 19200,
            masch_baud: o.baud,
            baud_wartet: None,
            uhr: 0,
            q: VecDeque::new(),
            mem: HashMap::new(),
            mem_nm: HashMap::new(),
            stickmodul: false,
            transfer: false,
            letzte: Vec::new(),
            adr_ziffern: 0,
            hex: String::new(),
            block: false,
            n_zahl: 0,
            w: false,
            w_hex: String::new(),
            p: 0,
            p_hex: String::new(),
            p_adr: 0,
            p_zahl: 0,
            p_daten: Vec::new(),
            ziel: 0,
            gestickt: false,
            befehle: Vec::new(),
            o,
        };
        let mut nm = b"NMMV04.04\0German      \0Fritz Gegauf SIM".to_vec();
        nm.resize(64, 0);
        for (i, b) in nm.iter().enumerate() {
            s.mem_nm.insert(0x200100 + i as u32, *b);
        }
        s.put(0x200100, b"SRMV04.04\0Fritz Gegauf SIM");
        // Verwaltungsblock wie an der echten artista 185 gelesen
        s.put(
            0xFFFD24,
            &[
                0x1C, 0x3C, 0, 1, 0xB3, 0x42, 0, 2, 0x40, 0x80, 0, 2, 1, 0xEA, 0, 2, 0x44, 0x80, 0, 2, 0x8E, 0x9A, 0,
                1, 0xFD, 0x7A, 0, 2, 0x38, 0x12, 1, 0,
            ],
        );
        s.put(0x024004, &[0, 0xFF, 0xFE, 0xD4]);
        s.put(0xFFFED4, &[0, 0, 0, 0, 0, 0x86, 0, 0x04]);
        s.put(0x024040, &[0, 2, 0x06, 0xAE]);
        s.put(0x024044, &ZUSATZ.to_be_bytes());
        s.put(0x024048, &T.to_be_bytes());
        let mut z = vec![
            0, 1, 0xB3, 0x42, 0, 1, 0xFD, 0x7A, 0, 2, 0x38, 0x12, 0x3C, 0, 0, 0, 0x17, 0x70,
        ];
        z[0x0C] = s.o.max_farben;
        s.put(ZUSATZ, &z);
        s
    }

    fn put(&mut self, adr: u32, d: &[u8]) {
        for (i, b) in d.iter().enumerate() {
            self.mem.insert(adr + i as u32, *b);
        }
    }

    /// Speicher der Maschine lesen (für Tests).
    pub fn speicher(&self, adr: u32, len: usize) -> Vec<u8> {
        (0..len as u32)
            .map(|i| *self.mem.get(&(adr + i)).unwrap_or(&0))
            .collect()
    }

    fn anzahl(&self) -> usize {
        if self.ziel == 0x50 { 3 } else { self.o.designs }
    }

    fn namen(&mut self, bank: usize) {
        let anzahl = self.anzahl().saturating_sub(27 * bank).min(27);
        for k in 0..27u32 {
            let mut nm = [0u8; 32];
            if (k as usize) < anzahl {
                let t = format!("SIM DESIGN {:02}", 27 * bank + k as usize + 1);
                nm[..t.len()].copy_from_slice(t.as_bytes());
            }
            self.put(INFO + 3 * N + 1 + 32 * k, &nm);
            self.put(INFO + 2 * N + 1 + k, &[0xA4]);
            self.put(INFO + N + 1 + k, &[2]);
        }
    }

    /// Befehl 0x400: ein kleines Design mit Anhang (Kopf + 1 Garnfarbe) in DATEN/ICON legen und – wie die echte 185
    /// (20.11.2026) – die Garnfarbtabelle an T anlegen.
    fn design_laden(&mut self) {
        let mut exp = vec![0x80, 0x04, 0xCE, 0xCE];
        for _ in 0..4 {
            exp.extend([50, 0, 50, 0]);
        }
        for _ in 0..4 {
            exp.extend([0, 50, 0, 50]);
        }
        for _ in 0..4 {
            exp.extend([0xCE, 0, 0xCE, 0]);
        }
        for _ in 0..4 {
            exp.extend([0, 0xCE, 0, 0xCE]);
        }
        let farbe = Farbe {
            rgb: [0xff, 0xff, 0xfa],
            nummer: 10,
            marke: "Isacord 40".into(),
            name: "white".into(),
        };
        let c = Container {
            kennung: 0,
            wort: 0,
            chunks: vec![
                Chunk {
                    id: CHUNK_KOPF,
                    daten: Kopf::neu("SIM QUADRAT", 1, 400, 400, 16).chunk_daten(),
                },
                Chunk {
                    id: CHUNK_FARBEN,
                    daten: farbe.bytes().to_vec(),
                },
            ],
        };
        let p = Puffer::neu(&exp, Some(&c));
        self.put(DATEN, &p.bytes());
        self.put(ICON, &Icon::aus_stichen(&exp_lesen(&exp)).bytes());
        let mut t = vec![0, 0, 0, 0, 0x17, 0x70];
        t.resize(self.o.max_farben as usize + 6, 0);
        t.extend(farbtabelle_maschine(&[farbe]));
        self.put(T, &t);
    }

    fn schreiben_fertig(&mut self) {
        let h = std::mem::take(&mut self.w_hex);
        if h.len() < 6 {
            return;
        }
        let adr = u32::from_str_radix(&h[..6], 16).unwrap_or(0);
        let d: Vec<u8> = (6..h.len() - 1)
            .step_by(2)
            .filter_map(|i| u8::from_str_radix(&h[i..i + 2], 16).ok())
            .collect();
        self.put(adr, &d);
        if adr != 0xFFFED0 || d.len() < 2 {
            return;
        }
        let cmd = (u16::from(d[0]) << 8 | u16::from(d[1])).wrapping_sub(1);
        self.befehle.push(cmd);
        let bank = *self.mem.get(&(AUSWAHL + 5)).unwrap_or(&0) as usize;
        let status: [u8; 2] = match cmd {
            0x50 | 0xA0 => {
                self.ziel = cmd;
                [0, 2]
            }
            0x100 => [0, 0], // wie am Gerät (v3.3): 0x0000
            0x400 => {
                self.design_laden();
                [0, 2]
            }
            0x08 => {
                self.gestickt = true;
                [0, 2]
            }
            0x30 | 0x20 | 0x60 | 0xC0 if self.ziel == 0x50 && !self.o.mit_karte => [0x10, 5],
            0x20 => {
                let n = self.anzahl().min(255) as u8;
                self.put(INFO, &[n]);
                self.namen(0);
                [0, 2]
            }
            0x30 | 0x60 | 0xC0 => {
                self.namen(bank);
                [0, 2]
            }
            _ => [0, 2],
        };
        self.put(0xFFFED0, &status);
    }

    fn empfange_byte(&mut self, b: u8) {
        if self.pc_baud != self.masch_baud {
            return; // falsche Baudrate: Maschine versteht nichts
        }
        if self.p == 3 {
            // 'P' 'S': 256 Byte binär, ohne Echo
            self.p_daten.push(b);
            if self.p_daten.len() == 256 {
                let d = std::mem::take(&mut self.p_daten);
                self.put(self.p_adr, &d);
                self.p = 0;
                self.q.push_back(b'O');
            }
            return;
        }
        let c = b as char;
        if self.o.stoerung_n > 0
            && self.block
            && self.adr_ziffern > 0
            && self.hex.is_empty()
            && self.n_zahl == self.o.stoerung_n
        {
            self.n_zahl += 1;
            self.adr_ziffern = 0;
            self.block = false;
            return; // Echo verloren, Befehl abgebrochen
        }
        self.q.push_back(b); // Echo
        let ist_hex = c.is_ascii_digit() || ('A'..='F').contains(&c);
        match self.p {
            1 => {
                if c == 'S' {
                    self.p = 2;
                    self.p_hex.clear();
                } else {
                    self.p = 0;
                }
                return;
            }
            2 => {
                if !ist_hex {
                    self.p = 0;
                    return;
                }
                self.p_hex.push(c);
                if self.p_hex.len() == 4 {
                    self.p_zahl += 1;
                    self.p_adr = u32::from_str_radix(&self.p_hex, 16).unwrap_or(0) << 8;
                    self.q.push_back(b'O');
                    if self.o.stoerung_ps > 0 && self.p_zahl == self.o.stoerung_ps {
                        self.p = 0; // Störung: kein 'E'
                    } else {
                        self.p = 3;
                        self.q.push_back(b'E');
                    }
                }
                return;
            }
            _ => {}
        }
        if self.w {
            if c == '?' {
                self.w_hex.push('?');
                self.schreiben_fertig();
                self.w = false;
            } else if ist_hex {
                self.w_hex.push(c);
            }
            return;
        }
        if self.adr_ziffern > 0 {
            if !ist_hex {
                self.adr_ziffern = 0;
                self.block = false;
                return;
            }
            self.hex.push(c);
            if self.hex.len() == self.adr_ziffern {
                let adr = u32::from_str_radix(&self.hex, 16).unwrap_or(0);
                self.adr_ziffern = 0;
                let mem = if self.stickmodul { &self.mem } else { &self.mem_nm };
                let werte: Vec<u8> = (0..if self.block { 256 } else { 32 })
                    .map(|i| *mem.get(&(adr + i)).unwrap_or(&0))
                    .collect();
                if self.block {
                    self.q.extend(werte);
                } else {
                    for v in werte {
                        self.q.extend(format!("{v:02X}").bytes());
                    }
                }
                self.q.push_back(b'O');
                self.block = false;
            }
            return;
        }
        self.letzte.push(b);
        if self.letzte.len() > 8 {
            self.letzte.remove(0);
        }
        if self.letzte.ends_with(b"TrME") {
            self.transfer = true;
            return;
        }
        if !self.transfer {
            return;
        }
        for (folge, baud) in [(&b"J05"[..], 57600), (&b"J02"[..], 115200), (&b"J11"[..], 19200)] {
            if self.letzte.ends_with(folge) {
                self.baud_wartet = Some((baud, self.uhr));
                return;
            }
        }
        if self.letzte.ends_with(b"YQ") && !self.stickmodul && !self.o.ohne_modul {
            self.stickmodul = true;
            self.q.push_back(b'O');
            return;
        }
        match c {
            'J' => {}
            'W' => {
                self.w = true;
                self.w_hex.clear();
            }
            'P' => self.p = 1,
            'R' => {
                self.adr_ziffern = 6;
                self.hex.clear();
                self.block = false;
            }
            'N' if self.masch_baud != 4800 => {
                self.adr_ziffern = 6;
                self.hex.clear();
                self.block = true;
                self.n_zahl += 1;
            }
            _ => {}
        }
    }

    fn baud_ereignis(&mut self) {
        if let Some((baud, seit)) = self.baud_wartet
            && self.uhr - seit >= 250
            && self.pc_baud == baud
        {
            self.baud_wartet = None;
            self.masch_baud = baud;
            self.q.extend(b"BOS");
        }
    }
}

impl Leitung for Simulation {
    fn senden(&mut self, daten: &[u8]) -> Result<(), String> {
        for &b in daten {
            self.empfange_byte(b);
        }
        Ok(())
    }

    fn empfangen(&mut self, ms: u64) -> Option<u8> {
        self.baud_ereignis();
        if let Some(b) = self.q.pop_front() {
            return Some(b);
        }
        // nichts da: die Wartezeit vergeht (virtuell), vielleicht kommt dabei etwas
        if ms > 0 {
            self.uhr += ms;
            self.baud_ereignis();
            return self.q.pop_front();
        }
        None
    }

    fn baud_setzen(&mut self, baud: u32) -> Result<(), String> {
        self.pc_baud = baud;
        self.q.clear();
        Ok(())
    }

    fn warten(&mut self, ms: u64) {
        self.uhr += ms;
    }

    fn jetzt_ms(&mut self) -> u64 {
        self.uhr
    }
}
