//! Echter serieller Port (Crate `serialport`): 8N1, kein Handshake, wie die Testprogramme (`Open-Port`).

use crate::leitung::Leitung;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::time::{Duration, Instant};

pub struct SeriellerPort {
    port: Box<dyn serialport::SerialPort>,
    eingang: VecDeque<u8>,
    start: Instant,
}

impl SeriellerPort {
    /// Port öffnen. `dtr_rts`: DTR und RTS setzen (Schalter `-DtrRts` der Testprogramme; an der 185 nicht nötig).
    pub fn oeffnen(name: &str, baud: u32, dtr_rts: bool) -> Result<SeriellerPort, String> {
        let mut port = serialport::new(name, baud)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(5))
            .open()
            .map_err(|e| format!("Port {name} lässt sich nicht öffnen: {e}"))?;
        let _ = port.write_data_terminal_ready(dtr_rts);
        let _ = port.write_request_to_send(dtr_rts);
        Ok(SeriellerPort {
            port,
            eingang: VecDeque::new(),
            start: Instant::now(),
        })
    }

    /// Namen der vorhandenen seriellen Ports.
    pub fn namen() -> Vec<String> {
        let mut v: Vec<String> = serialport::available_ports()
            .map(|l| l.into_iter().map(|p| p.port_name).collect())
            .unwrap_or_default();
        v.sort();
        v.dedup();
        v
    }

    fn abholen(&mut self) {
        let n = self.port.bytes_to_read().unwrap_or(0) as usize;
        if n > 0 {
            let mut buf = vec![0u8; n];
            if let Ok(k) = self.port.read(&mut buf) {
                self.eingang.extend(&buf[..k]);
            }
        }
    }
}

impl Leitung for SeriellerPort {
    fn senden(&mut self, daten: &[u8]) -> Result<(), String> {
        self.port.write_all(daten).map_err(|e| format!("Senden: {e}"))?;
        self.port.flush().map_err(|e| format!("Senden: {e}"))
    }

    fn empfangen(&mut self, ms: u64) -> Option<u8> {
        let anfang = Instant::now();
        loop {
            if let Some(b) = self.eingang.pop_front() {
                return Some(b);
            }
            self.abholen();
            if !self.eingang.is_empty() {
                continue;
            }
            let vergangen = anfang.elapsed();
            if vergangen >= Duration::from_millis(ms) {
                return None;
            }
            // wie die Testprogramme: in den ersten 40 ms nicht schlafen (Windows-Schlaf ist bis zu 15 ms grob)
            if vergangen < Duration::from_millis(40) {
                std::thread::yield_now();
            } else {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }

    fn baud_setzen(&mut self, baud: u32) -> Result<(), String> {
        self.port
            .set_baud_rate(baud)
            .map_err(|e| format!("Baudrate {baud}: {e}"))?;
        let _ = self.port.clear(serialport::ClearBuffer::Input);
        self.eingang.clear();
        Ok(())
    }

    fn warten(&mut self, ms: u64) {
        std::thread::sleep(Duration::from_millis(ms));
    }

    fn jetzt_ms(&mut self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}
