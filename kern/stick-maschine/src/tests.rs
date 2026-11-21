//! Tests gegen die Simulation (wie die Selbsttests der Testprogramme).

use crate::maschine::{Maschine, SendeOptionen, Speicher};
use crate::simulation::{SimOptionen, Simulation};
use stick_formate::artista::{Puffer, Sendedaten};

fn verbunden(o: SimOptionen) -> Maschine<Simulation> {
    Maschine::verbinden(Simulation::neu(o), 19200)
        .unwrap_or_else(|a| panic!("{}\n{}", a.fehler, a.protokoll.join("\n")))
}

fn testdesign() -> Sendedaten {
    let mut exp = vec![0x80, 0x04, 0x9C, 0x9C];
    for (dx, dy) in [(20i8, 0i8), (0, 20), (-20, 0), (0, -20)] {
        for _ in 0..5 {
            exp.extend([dx as u8, dy as u8]);
        }
    }
    Sendedaten::aus_exp("Testquadrat", &exp, false)
}

fn ohne_fortschritt() -> impl FnMut(&str, usize, usize, usize) {
    |_: &str, _: usize, _: usize, _: usize| {}
}

#[test]
fn verbinden_frisch_und_nach_sitzung() {
    for baud in [19200, 57600] {
        let m = verbunden(SimOptionen {
            baud,
            ..Default::default()
        });
        assert_eq!(m.firmware_naehmaschine, "04.04");
        assert_eq!(m.firmware_stickmodul, "04.04");
        assert_eq!(m.verwaltung.n, 28);
        assert_eq!(m.verwaltung.daten, 0x28E9A);
        assert_eq!(m.verwaltung.icon, 0x24480);
        assert_eq!(m.v.baud, 57600);
        let (ok, v) = m.verlassen();
        assert!(ok);
        assert_eq!(v.l.befehle, vec![0x100]);
    }
}

#[test]
fn ohne_stickmodul() {
    let a = Maschine::verbinden(
        Simulation::neu(SimOptionen {
            ohne_modul: true,
            ..Default::default()
        }),
        19200,
    )
    .err()
    .expect("muss scheitern");
    assert!(a.fehler.contains("Stickmodul"), "{}", a.fehler);
}

#[test]
fn designliste_zwei_baenke() {
    let mut m = verbunden(SimOptionen::default());
    let l = m.designliste(Speicher::Maschine).unwrap();
    assert_eq!(l.len(), 30);
    assert_eq!(l[0].name, "SIM DESIGN 01");
    assert_eq!(l[29].name, "SIM DESIGN 30");
    assert_eq!(l[29].nr, 30);
    assert_eq!(l[0].flags, 0xA4);
    // Karte fehlt: Status 0x1005
    assert!(m.designliste(Speicher::Karte).is_err());
}

#[test]
fn design_und_farbbereich_lesen() {
    let mut m = verbunden(SimOptionen {
        stoerung_n: 2,
        ..Default::default()
    });
    let fb = m.farbbereich_lesen().unwrap();
    assert_eq!((fb.zusatz_adr, fb.t), (0x20772, 0x2083A));
    assert_eq!((fb.grenzen.max_farben, fb.grenzen.groesse), (60, 6000));
    assert!(fb.tabelle.is_none(), "{}", fb.text);
    let mut f = ohne_fortschritt();
    let d = m.design_lesen(5, &mut f).unwrap();
    assert_eq!(d.name, "SIM DESIGN 05");
    let p = Puffer::lesen(&d.puffer, 28).unwrap();
    assert!(p.endkennung() && p.anhang_gueltig());
    assert_eq!(d.icon.len(), 0xAE + 9 * 62);
    assert_eq!(Sendedaten::aus_gelesenem(&d.puffer, false).unwrap().name, "SIM QUADRAT");
    // wie die echte 185: nach 0x400 steht die Garnfarbtabelle in T
    let fb = m.farbbereich_lesen().unwrap();
    assert_eq!(fb.farben.len(), 1, "{}", fb.text);
    assert_eq!(fb.farben[0].name, "white");
    assert!(
        m.v.protokoll.iter().any(|z| z.contains("WIEDERHOLUNG")),
        "Störung nicht ausgelöst"
    );
}

#[test]
fn senden_wie_das_original() {
    for (stoerung_n, stoerung_ps, sticken) in [(0, 0, false), (3, 2, true)] {
        let mut m = verbunden(SimOptionen {
            stoerung_n,
            stoerung_ps,
            ..Default::default()
        });
        let s = testdesign();
        let o = SendeOptionen {
            sticken,
            ..Default::default()
        };
        let plan = m.planen(&s, &o).unwrap();
        assert!(plan.farben_schreiben && plan.icon_schreiben);
        assert_eq!(plan.schritte.len(), if sticken { 5 } else { 4 });
        let mut f = ohne_fortschritt();
        let b = m.senden(&s, &o, &mut f).unwrap();
        assert_eq!(b.sticken.is_some_and(|r| r.ok), sticken);
        let (_, v) = m.verlassen();
        let sim = &v.l;
        assert_eq!(sim.speicher(0x28E9A, s.puffer.bytes().len()), s.puffer.bytes());
        assert_eq!(sim.speicher(0x24480, 732), s.icon.bytes());
        let t = s.farbtabelle().unwrap();
        assert_eq!(sim.speicher(0x2083A + 66, t.len()), t);
        assert_eq!(sim.speicher(0x2083A, 3), [0, 0, 0]);
        assert_eq!(sim.speicher(0x24080 + 29, 1), [s.info.farben_minus_1]);
        assert_eq!(sim.gestickt, sticken);
        assert_eq!(*sim.befehle.last().unwrap(), 0x100);
        if stoerung_ps > 0 {
            assert!(v.protokoll.iter().any(|z| z.contains("WIEDERHOLUNG")));
        }
    }
}

#[test]
fn farbtabelle_passt_nicht() {
    let mut m = verbunden(SimOptionen {
        max_farben: 0,
        ..Default::default()
    });
    let s = testdesign();
    let plan = m.planen(&s, &SendeOptionen::default()).unwrap();
    assert!(!plan.farben_schreiben);
    assert!(
        plan.hinweise.iter().any(|h| h.contains("erlaubt 0")),
        "{:?}",
        plan.hinweise
    );
    let mut f = ohne_fortschritt();
    m.senden(&s, &SendeOptionen::default(), &mut f).unwrap();
    let (_, v) = m.verlassen();
    assert_eq!(v.l.speicher(0x2083A + 6, 4), [0, 0, 0, 0]);
}

#[test]
fn flash_wird_nie_beschrieben() {
    let mut m = verbunden(SimOptionen::default());
    let mut f = ohne_fortschritt();
    assert!(m.v.schreiben_gross(0x27_FFF0, &[0; 32], "Test", &mut f).is_err());
    assert!(m.v.schreiben(0xC0_0000, &[1], 1000).is_err());
}

#[test]
fn zu_grosses_design() {
    let mut m = verbunden(SimOptionen::default());
    let mut s = testdesign();
    s.puffer.stiche = vec![0; 150_000];
    s.puffer.stiche.extend([0x80, 0x81]);
    assert!(m.planen(&s, &SendeOptionen::default()).unwrap_err().contains("zu groß"));
}
