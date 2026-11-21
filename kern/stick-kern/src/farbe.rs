//! Farbrechnung für Garnzuordnung und Bildumwandlung: sRGB → CIE-Lab und Farbabstand CIEDE2000 (eigene Festlegung,
//! siehe `stick_formate::garnkarte::Garnkarte::naechstes`).

/// sRGB → CIE-Lab (D65).
pub fn lab(c: [u8; 3]) -> [f64; 3] {
    let lin = |u: u8| {
        let u = f64::from(u) / 255.0;
        if u > 0.04045 {
            ((u + 0.055) / 1.055).powf(2.4)
        } else {
            u / 12.92
        }
    };
    let (r, g, b) = (lin(c[0]), lin(c[1]), lin(c[2]));
    let x = (0.4124 * r + 0.3576 * g + 0.1805 * b) / 0.95047;
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let z = (0.0193 * r + 0.1192 * g + 0.9505 * b) / 1.08883;
    let f = |t: f64| {
        if t > 0.008856 {
            t.cbrt()
        } else {
            7.787 * t + 16.0 / 116.0
        }
    };
    [116.0 * f(y) - 16.0, 500.0 * (f(x) - f(y)), 200.0 * (f(y) - f(z))]
}

/// Farbabstand CIEDE2000 (Sharma, Wu, Dalal 2005).
pub fn de2000(p: [f64; 3], q: [f64; 3]) -> f64 {
    let ([l1, a1, b1], [l2, a2, b2]) = (p, q);
    let cm = (a1.hypot(b1) + a2.hypot(b2)) / 2.0;
    let g = 0.5 * (1.0 - (cm.powi(7) / (cm.powi(7) + 25f64.powi(7))).sqrt());
    let (a1p, a2p) = ((1.0 + g) * a1, (1.0 + g) * a2);
    let (c1p, c2p) = (a1p.hypot(b1), a2p.hypot(b2));
    let winkel = |b: f64, a: f64| b.atan2(a).to_degrees().rem_euclid(360.0);
    let (h1, h2) = (winkel(b1, a1p), winkel(b2, a2p));
    let dl = l2 - l1;
    let dc = c2p - c1p;
    let dh = if c1p * c2p == 0.0 {
        0.0
    } else if (h2 - h1).abs() <= 180.0 {
        h2 - h1
    } else if h2 - h1 > 180.0 {
        h2 - h1 - 360.0
    } else {
        h2 - h1 + 360.0
    };
    let dhh = 2.0 * (c1p * c2p).sqrt() * (dh / 2.0).to_radians().sin();
    let lm = (l1 + l2) / 2.0;
    let cmp = (c1p + c2p) / 2.0;
    let hm = if c1p * c2p == 0.0 {
        h1 + h2
    } else if (h1 - h2).abs() <= 180.0 {
        (h1 + h2) / 2.0
    } else if h1 + h2 < 360.0 {
        (h1 + h2 + 360.0) / 2.0
    } else {
        (h1 + h2 - 360.0) / 2.0
    };
    let cos = |grad: f64| grad.to_radians().cos();
    let t =
        1.0 - 0.17 * cos(hm - 30.0) + 0.24 * cos(2.0 * hm) + 0.32 * cos(3.0 * hm + 6.0) - 0.20 * cos(4.0 * hm - 63.0);
    let dt = 30.0 * (-((hm - 275.0) / 25.0).powi(2)).exp();
    let rc = 2.0 * (cmp.powi(7) / (cmp.powi(7) + 25f64.powi(7))).sqrt();
    let sl = 1.0 + 0.015 * (lm - 50.0).powi(2) / (20.0 + (lm - 50.0).powi(2)).sqrt();
    let sc = 1.0 + 0.045 * cmp;
    let sh = 1.0 + 0.015 * cmp * t;
    let rt = -(2.0 * dt).to_radians().sin() * rc;
    ((dl / sl).powi(2) + (dc / sc).powi(2) + (dhh / sh).powi(2) + rt * (dc / sc) * (dhh / sh)).sqrt()
}
