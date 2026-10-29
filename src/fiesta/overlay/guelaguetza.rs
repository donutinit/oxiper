//! Guelaguetza: papel picado in the colors of Oaxacan embroidery with
//! cut-outs of the fiesta, and flower petals drifting.

use super::drift::{Flecks, tone};
use super::papel::{Design, Papel};
use super::{Fiesta, Look, Themed};
use crate::color::rgb;

/// The cut-outs: a piña, the pluma headdress of the Danza de la Pluma, a
/// flower, a marmota (the great paper globe of the calenda) and a bordado
/// (the stepped embroidery of a huipil).
const DESIGNS: &[Design] = &[
    // piña: its crown over the lattice of its skin
    [
        "XXXXXXX", "XXX.XXX", "XX.X.XX", "X.X.X.X", "XX.X.XX", "X.X.X.X", "XXXXXXX", "X.X.X.X",
    ],
    // pluma: the fan of feathers over its band
    [
        "XXXXXXX", "X.X.X.X", "XX.X.XX", "XXX.XXX", "XX...XX", "XXXXXXX", "XXXXXXX", "X.X.X.X",
    ],
    // flor
    [
        "XXXXXXX", "XXX.XXX", "XX...XX", "X..X..X", "XX...XX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
    // marmota: a globe and its meridians
    [
        "XXXXXXX", "XX...XX", "X.X.X.X", "X.X.X.X", "X.X.X.X", "XX...XX", "XXXXXXX", "X.X.X.X",
    ],
    // bordado
    [
        "XXXXXXX", "XX.X.XX", "X.XXX.X", ".XXXXX.", "X.XXX.X", "XX.X.XX", "XXXXXXX", "X.X.X.X",
    ],
];

/// Papel picado colors under `classic`: grana cochinilla, amarillo, verde,
/// rosa, morado and turquesa.
const PAPEL: Papel = Papel {
    designs: DESIGNS,
    colors: [
        rgb(0x9b111e),
        rgb(0xffcc00),
        rgb(0x1f8f4a),
        rgb(0xe4007c),
        rgb(0x6a2c91),
        rgb(0x00aab8),
    ],
    // The cord, natural string.
    cord: rgb(0xbcae94),
    gap: 2,
    ordered: false,
    banderitas: false,
    themed: Themed::Accents,
};

/// Flower petals: rosa, amarillo, morado and rojo.
const PETALS: Flecks = Flecks {
    tones: &[
        tone(0xe4007c),
        tone(0xffcc00),
        tone(0x9b4fd0),
        tone(0xd8283c),
    ],
    share: 0,
    cells: 200,
    themed: Themed::Accents,
};

/// What the Guelaguetza hangs: the same on each of its days (the second string
/// comes on the Lunes del Cerro, when the fiesta is at its height, and the
/// petals with the level of the day).
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 8,
        salt: 0x6775_656c_6167_7565,
        papel: Some(&PAPEL),
        flecks: Some(&PETALS),
        ..Look::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{after, changed, scene};
    use super::super::{LOWER_LEVEL, Overlay, UPPER};
    use super::*;
    use crate::color::Color;
    use crate::fiesta::Kind;
    use crate::render::Canvas;
    use crate::theme::Theme;

    /// A day of the Guelaguetza of 2027: the Saturday the 17th of July to the
    /// second Lunes del Cerro, the 26th (the first was the 19th).
    fn day(d: u32) -> Fiesta {
        let f = Fiesta::of_date(2027, 7, d);
        assert!(f.is(Kind::Guelaguetza), "7-{d}");
        f
    }

    /// Cells the overlay changed in rows `from..to`.
    fn cells(c: &Canvas, w: u16, from: i32, to: i32) -> usize {
        changed(c, w)
            .iter()
            .filter(|&&(_, y)| (from..to).contains(&y))
            .count()
    }

    #[test]
    fn the_cutouts_are_whole() {
        for design in DESIGNS {
            assert!(design.iter().all(|r| r.len() == 7));
            assert!(
                design
                    .iter()
                    .all(|r| r.bytes().all(|b| b == b'X' || b == b'.'))
            );
            assert_eq!(design[0], "XXXXXXX", "a folded hem");
            assert_eq!(design[7], "X.X.X.X", "the fringe");
        }
    }

    #[test]
    fn dresses_every_day_of_the_guelaguetza() {
        let theme = Theme::classic();
        for d in 17..=26 {
            let fiesta = day(d);
            let mut o = Overlay::new(120, 36);
            let c = after(&mut o, fiesta, 4.0, &theme);
            let (top, lower, below) = (
                cells(&c, 120, 0, 4),
                cells(&c, 120, 4, 6),
                cells(&c, 120, 6, 36),
            );
            assert!(top > 250, "7-{d}: {top} cells of papel picado");
            assert!(below > 0, "7-{d}: no petals");
            // The second string on the Lunes del Cerro, the 19th and the 26th.
            if fiesta.level >= LOWER_LEVEL {
                assert!(matches!(d, 19 | 26));
                assert!(lower > 60, "7-{d}: {lower} cells");
            } else {
                assert!(lower < 15, "7-{d}: {lower} cells");
            }
        }
        assert!(Fiesta::height(Kind::Guelaguetza).is(Kind::Guelaguetza));
    }

    #[test]
    fn more_petals_as_the_fiesta_builds_up() {
        let theme = Theme::classic();
        let count = |d| {
            let mut o = Overlay::new(200, 60);
            let c = after(&mut o, day(d), 2.0, &theme);
            cells(&c, 200, 6, 60)
        };
        // A Tuesday, the Sunday before the Lunes del Cerro, and the Lunes.
        assert!(
            count(19) > count(20) + 5,
            "{} then {}",
            count(20),
            count(19)
        );
        assert!(count(25) > count(20));
    }

    #[test]
    fn papel_picado_keeps_to_its_rows() {
        let theme = Theme::classic();
        for (fiesta, rows) in [(day(20), 4), (day(19), 6)] {
            for w in [9, 40, 80, 121, 200] {
                let mut o = Overlay::new(w, 30);
                o.update(0.0, fiesta);
                o.petals.clear();
                for step in 0..60 {
                    o.update(0.17, fiesta);
                    let mut c = scene(w, 30);
                    o.draw(&mut c, &theme, fiesta);
                    assert_eq!(cells(&c, w, rows, 30), 0, "{w} wide, step {step}");
                    if w >= 40 {
                        assert!(cells(&c, w, 0, rows) > w as usize, "{w} wide");
                    }
                }
            }
        }
    }

    #[test]
    fn petals_touch_only_their_cells() {
        let fiesta = day(19);
        let mut o = Overlay::new(120, 40);
        o.update(0.0, fiesta);
        o.flags.clear();
        o.upper = 0;
        o.cover.fill(UPPER);
        o.ties.clear();
        for _ in 0..50 {
            o.update(0.1, fiesta);
            let mut c = scene(120, 40);
            o.draw(&mut c, &Theme::classic(), fiesta);
            // Each petal covers two pixels at most.
            let n = changed(&c, 120).len();
            assert!(n > 0 && n <= 2 * o.petals.len(), "{n} cells");
        }
    }

    #[test]
    fn nothing_on_an_ordinary_day() {
        for fiesta in [
            Fiesta::NONE,
            Fiesta::of_day(6, 15),
            Fiesta::of_date(2027, 7, 16),
            Fiesta::of_date(2027, 7, 27),
        ] {
            let mut o = Overlay::new(80, 24);
            let c = after(&mut o, fiesta, 3.0, &Theme::classic());
            assert!(changed(&c, 80).is_empty());
        }
    }

    #[test]
    fn comes_and_goes_with_the_holiday() {
        let theme = Theme::builtin("dracula").unwrap();
        let mut o = Overlay::new(100, 30);
        let on = |o: &mut Overlay, f| !changed(&after(o, f, 1.0, &theme), 100).is_empty();
        assert!(on(&mut o, day(17)));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, day(19)));
        assert!(!on(&mut o, Fiesta::of_date(2027, 7, 27)));
        // The second string only on the Lunes del Cerro, hung anew each day.
        o.update(0.0, day(20));
        assert_eq!(o.upper, o.flags.len());
        o.update(0.0, day(26));
        assert!(o.upper < o.flags.len());
        o.update(0.0, day(25));
        assert_eq!(o.upper, o.flags.len());
        // The other holidays take their turn.
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert!(on(&mut o, Fiesta::of_day(9, 15)));
        assert!(on(&mut o, day(19)));
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (w, h) in [
                (1, 1),
                (2, 3),
                (7, 4),
                (8, 1),
                (16, 2),
                (300, 90),
                (500, 150),
            ] {
                for fiesta in [day(20), day(19)] {
                    let mut o = Overlay::new(w, h);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for dt in [5.0, f32::NAN, -3.0, f32::INFINITY, 0.0, 5.0, 1e9] {
                        o.update(dt, fiesta);
                        c.clear();
                        o.draw(&mut c, &theme, fiesta);
                    }
                    o.resize(h, w);
                    c.resize(h, w);
                    c.strict = true;
                    o.update(5.0, fiesta);
                    o.draw(&mut c, &theme, fiesta);
                }
            }
        }
    }

    #[test]
    fn same_frames_every_time() {
        let theme = Theme::classic();
        let fiesta = day(19);
        let frame = || after(&mut Overlay::new(90, 28), fiesta, 6.0, &theme).to_text();
        assert_eq!(frame(), frame());
    }
}
