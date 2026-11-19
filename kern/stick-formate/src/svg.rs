//! Vorschau einer Stichfolge als SVG (mm-Koordinaten, y nach unten gespiegelt).

use std::fmt::Write;
use stick_kern::geometrie::Punkt;
use stick_kern::rahmen::Rahmen;
use stick_kern::{Art, StichFolge};

pub struct SvgOptionen {
    pub titel: String,
    pub fadenbreite: f64,
    pub einstiche: bool,
    pub rand: f64,
    pub pixel_pro_mm: f64,
}

impl Default for SvgOptionen {
    fn default() -> Self {
        SvgOptionen {
            titel: String::new(),
            fadenbreite: 0.35,
            einstiche: false,
            rand: 5.0,
            pixel_pro_mm: 4.0,
        }
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn svg_text(folge: &StichFolge, rahmen: Option<&Rahmen>, o: &SvgOptionen) -> String {
    let mut pts: Vec<Punkt> = folge.stiche.iter().map(|s| s.p()).collect();
    if let Some(r) = rahmen {
        pts.extend(&r.form);
    }
    if pts.is_empty() {
        pts.push(Punkt::new(0.0, 0.0));
    }
    let x0 = pts.iter().map(|q| q.x).fold(f64::INFINITY, f64::min) - o.rand;
    let x1 = pts.iter().map(|q| q.x).fold(f64::NEG_INFINITY, f64::max) + o.rand;
    let y0 = pts.iter().map(|q| q.y).fold(f64::INFINITY, f64::min) - o.rand;
    let y1 = pts.iter().map(|q| q.y).fold(f64::NEG_INFINITY, f64::max) + o.rand;
    let (b, h) = (x1 - x0, y1 - y0);
    let mut s = String::new();
    let _ = writeln!(
        s,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{x0:.2} {:.2} {b:.2} {h:.2}" width="{:.0}" height="{:.0}">"#,
        -y1,
        b * o.pixel_pro_mm,
        h * o.pixel_pro_mm
    );
    let _ = writeln!(s, "<title>{}</title>", escape(&o.titel));
    let _ = writeln!(
        s,
        r##"<rect x="{x0:.2}" y="{:.2}" width="{b:.2}" height="{h:.2}" fill="#f7f4ee"/>"##,
        -y1
    );
    if let Some(r) = rahmen {
        let d: Vec<String> = r.form.iter().map(|q| format!("{:.2},{:.2}", q.x, -q.y)).collect();
        let _ = writeln!(
            s,
            r##"<polygon points="{}" fill="#ffffff" stroke="#9aa5b1" stroke-width="0.4" stroke-dasharray="2 1.5"/>"##,
            d.join(" ")
        );
        let _ = writeln!(
            s,
            r##"<path d="M -3 0 H 3 M 0 -3 V 3" stroke="#9aa5b1" stroke-width="0.25"/>"##
        );
    }
    for (farbe, stiche) in folge.farbbloecke() {
        let mut linien: Vec<Vec<Punkt>> = Vec::new();
        let mut spruenge: Vec<(Punkt, Punkt)> = Vec::new();
        let mut aktuell: Vec<Punkt> = Vec::new();
        let mut vorher: Option<Punkt> = None;
        for st in &stiche {
            if st.art == Art::Sprung {
                if aktuell.len() > 1 {
                    linien.push(std::mem::take(&mut aktuell));
                }
                if let Some(v) = vorher {
                    spruenge.push((v, st.p()));
                }
                aktuell = vec![st.p()];
            } else {
                aktuell.push(st.p());
            }
            vorher = Some(st.p());
        }
        if aktuell.len() > 1 {
            linien.push(aktuell);
        }
        for (a, z) in spruenge {
            let _ = writeln!(
                s,
                r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#b0b0b0" stroke-width="0.15" stroke-dasharray="0.8 0.8"/>"##,
                a.x, -a.y, z.x, -z.y
            );
        }
        for lin in linien {
            let d: Vec<String> = lin.iter().map(|q| format!("{:.2},{:.2}", q.x, -q.y)).collect();
            let _ = writeln!(
                s,
                r#"<polyline points="{}" fill="none" stroke="{}" stroke-width="{}" stroke-linejoin="round" stroke-linecap="round" stroke-opacity="0.9"/>"#,
                d.join(" "),
                escape(&farbe),
                o.fadenbreite
            );
            if o.einstiche {
                for q in &lin {
                    let _ = writeln!(
                        s,
                        r##"<circle cx="{:.2}" cy="{:.2}" r="0.12" fill="#000"/>"##,
                        q.x, -q.y
                    );
                }
            }
        }
    }
    s.push_str("</svg>\n");
    s
}
