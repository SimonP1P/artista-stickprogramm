//! Byte-Leitung zur Maschine: echter serieller Port ([`crate::port::SeriellerPort`]) oder Simulation
//! ([`crate::simulation::Simulation`]). Alle Wartezeiten laufen über die Leitung, damit die Simulation ohne echtes
//! Warten auskommt (virtuelle Uhr).

pub trait Leitung {
    /// Bytes senden.
    fn senden(&mut self, daten: &[u8]) -> Result<(), String>;
    /// Ein Byte empfangen, höchstens `ms` Millisekunden warten (0 = nur nachsehen). `None` = nichts gekommen.
    fn empfangen(&mut self, ms: u64) -> Option<u8>;
    /// Baudrate des PCs umstellen (wie das Original: Port mit neuer Baudrate, Eingang leer).
    fn baud_setzen(&mut self, baud: u32) -> Result<(), String>;
    /// Warten.
    fn warten(&mut self, ms: u64);
    /// Uhr in Millisekunden (nur Differenzen zählen).
    fn jetzt_ms(&mut self) -> u64;
}

/// Eine Leitung, die erst zur Laufzeit gewählt wird (Port oder Simulation).
impl<T: Leitung + ?Sized> Leitung for Box<T> {
    fn senden(&mut self, daten: &[u8]) -> Result<(), String> {
        (**self).senden(daten)
    }
    fn empfangen(&mut self, ms: u64) -> Option<u8> {
        (**self).empfangen(ms)
    }
    fn baud_setzen(&mut self, baud: u32) -> Result<(), String> {
        (**self).baud_setzen(baud)
    }
    fn warten(&mut self, ms: u64) {
        (**self).warten(ms)
    }
    fn jetzt_ms(&mut self) -> u64 {
        (**self).jetzt_ms()
    }
}
