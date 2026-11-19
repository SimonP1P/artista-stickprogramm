//! Stichfolge: Ergebnis der Berechnung, Eingabe für Export und Vorschau.

use crate::geometrie::{Punkt, abstand, begrenzung, p};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Art {
    /// Nadel sticht an (x, y) ein
    Stich,
    /// Rahmen fährt nach (x, y), ohne einzustechen
    Sprung,
    /// Maschine hält an, nächste Farbe; (x, y) = aktuelle Position
    Farbwechsel,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stich {
    pub x: f64,
    pub y: f64,
    pub art: Art,
}

impl Stich {
    pub fn p(&self) -> Punkt {
        p(self.x, self.y)
    }
}

/// Absolute Positionen in mm (Ursprung = Rahmenmitte). `farben[i]` gilt für Farbblock i.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StichFolge {
    pub stiche: Vec<Stich>,
    pub farben: Vec<String>,
}

impl StichFolge {
    pub fn stich(&mut self, q: Punkt) {
        self.stiche.push(Stich {
            x: q.x,
            y: q.y,
            art: Art::Stich,
        });
    }

    pub fn sprung(&mut self, q: Punkt) {
        self.stiche.push(Stich {
            x: q.x,
            y: q.y,
            art: Art::Sprung,
        });
    }

    pub fn farbwechsel(&mut self, neue_farbe: &str) {
        let pos = self.position().unwrap_or(p(0.0, 0.0));
        self.stiche.push(Stich {
            x: pos.x,
            y: pos.y,
            art: Art::Farbwechsel,
        });
        self.farben.push(neue_farbe.to_string());
    }

    pub fn position(&self) -> Option<Punkt> {
        self.stiche.last().map(|s| s.p())
    }

    pub fn anzahl(&self, art: Art) -> usize {
        self.stiche.iter().filter(|s| s.art == art).count()
    }

    pub fn begrenzung(&self) -> Option<(f64, f64, f64, f64)> {
        begrenzung(self.stiche.iter().filter(|s| s.art == Art::Stich).map(|s| s.p()))
    }

    /// Längen aller echten Stiche (Einstich nach Einstich, ohne Sprünge davor).
    pub fn stichlaengen(&self) -> Vec<f64> {
        let mut out = Vec::new();
        let mut vorher: Option<&Stich> = None;
        for s in &self.stiche {
            match s.art {
                Art::Stich => {
                    if let Some(v) = vorher
                        && v.art == Art::Stich
                    {
                        out.push(abstand(v.p(), s.p()));
                    }
                    vorher = Some(s);
                }
                Art::Sprung => vorher = Some(s),
                Art::Farbwechsel => {}
            }
        }
        out
    }

    /// Stiche je Farbe (Farbwechsel trennt die Blöcke).
    pub fn farbbloecke(&self) -> Vec<(String, Vec<Stich>)> {
        let mut bloecke = Vec::new();
        let mut aktuell = Vec::new();
        for s in &self.stiche {
            if s.art == Art::Farbwechsel {
                bloecke.push((self.farbe(bloecke.len()), std::mem::take(&mut aktuell)));
            } else {
                aktuell.push(*s);
            }
        }
        bloecke.push((self.farbe(bloecke.len()), aktuell));
        bloecke
    }

    fn farbe(&self, i: usize) -> String {
        self.farben.get(i).cloned().unwrap_or_else(|| "#000000".to_string())
    }
}
