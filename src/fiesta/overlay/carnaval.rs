//! Carnaval: banderines in every bright color, confetti and curling
//! serpentinas falling.

use super::bunting::Bunting;
use super::drift::{Flecks, tone};
use super::ribbon::Ribbons;
use super::{Fiesta, Look, Themed};
use crate::color::rgb;

/// The pennants: rosa, turquesa, amarillo, verde limón, morado and naranja,
/// on a cord of lentejuela dorada.
const BUNTING: Bunting = Bunting {
    colors: [
        rgb(0xe4007c), // rosa mexicano
        rgb(0x00aab8), // turquesa
        rgb(0xffd200), // amarillo
        rgb(0x8cc828), // verde limón
        rgb(0x9b4fd0), // morado
        rgb(0xff8c00), // naranja
    ],
    ordered: false,
    cord: rgb(0xe6b830),
    two: true,
    themed: Themed::Accents,
};

/// Confetti in every bright color, with the gold of a sequin among it.
const CONFETTI: Flecks = Flecks {
    tones: &[
        tone(0xe4007c),
        tone(0x00aab8),
        tone(0x9b4fd0),
        tone(0x8cc828),
        tone(0xffd200),
        tone(0xff8c00),
        tone(0xe8322a),
        tone(0xffcc33),
    ],
    share: 0,
    cells: 100,
    themed: Themed::Accents,
};

/// And serpentinas.
const SERPENTINAS: Ribbons = Ribbons {
    colors: &[
        rgb(0xe4007c),
        rgb(0x00aab8),
        rgb(0xffd200),
        rgb(0x8cc828),
        rgb(0x9b4fd0),
        rgb(0xffcc33),
    ],
    cells: 600,
};

/// What Carnaval hangs: the same on each day (the second string, the confetti
/// and the serpentinas build up toward Martes de Carnaval).
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 7,
        salt: 0x6361_726e_6176_616c,
        bunting: Some(&BUNTING),
        flecks: Some(&CONFETTI),
        ribbons: Some(&SERPENTINAS),
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

    /// A day of Carnaval 2027: Thursday the 4th of February to Martes de
    /// Carnaval, the 9th.
    fn day(d: u32) -> Fiesta {
        let f = Fiesta::of_date(2027, 2, d);
        assert!(f.is(Kind::Carnaval), "2-{d}");
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
    fn dresses_every_day_of_carnaval() {
        let theme = Theme::classic();
        for d in 4..=9 {
            let fiesta = day(d);
            let mut o = Overlay::new(120, 36);
            let c = after(&mut o, fiesta, 4.0, &theme);
            let (top, lower, below) = (
                cells(&c, 120, 0, 3),
                cells(&c, 120, 3, 6),
                cells(&c, 120, 6, 36),
            );
            assert!(top > 150, "2-{d}: {top} cells of banderines");
            assert!(below > 10, "2-{d}: {below} cells of confetti");
            // The second string from Domingo on.
            if fiesta.level >= LOWER_LEVEL {
                assert!(lower > 60, "2-{d}: {lower} cells");
            } else {
                assert!(lower < 15, "2-{d}: {lower} cells");
            }
        }
    }

    #[test]
    fn builds_up_to_martes_de_carnaval() {
        let theme = Theme::classic();
        let count = |d| {
            let mut o = Overlay::new(200, 60);
            let c = after(&mut o, day(d), 3.0, &theme);
            cells(&c, 200, 6, 60)
        };
        // More confetti and serpentinas each day.
        assert!(count(9) > count(4) + 20, "{} then {}", count(4), count(9));
        assert!(count(7) > count(4));
        // The banderines of the second string come with Domingo.
        let strings = |d| {
            let mut o = Overlay::new(120, 36);
            o.update(0.0, day(d));
            o.hung.iter().filter(|h| h.len == 1).count()
        };
        assert_eq!((strings(4), strings(6)), (0, 0));
        assert!(strings(7) > 5 && strings(9) > 5);
    }

    #[test]
    fn the_banderines_keep_to_three_rows_and_six() {
        let theme = Theme::classic();
        for (fiesta, rows) in [(day(4), 3), (day(6), 3), (day(7), 6), (day(9), 6)] {
            for w in [9, 40, 80, 121, 200] {
                let mut o = Overlay::new(w, 30);
                o.update(0.0, fiesta);
                // What falls over the whole screen aside.
                o.petals.clear();
                o.ribbons.clear();
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
    fn neighbors_never_share_a_color() {
        let mut o = Overlay::new(160, 36);
        o.update(0.0, day(9));
        for string in [0, 1] {
            let colors: Vec<u8> = o
                .hung
                .iter()
                .filter(|h| h.len == string)
                .map(|h| h.color)
                .collect();
            assert!(colors.len() > 10);
            assert!(colors.windows(2).all(|p| p[0] != p[1]), "{colors:?}");
            assert!(colors.windows(3).all(|p| p[0] != p[2]), "{colors:?}");
        }
    }

    #[test]
    fn confetti_and_serpentinas_touch_only_their_cells() {
        let fiesta = day(9);
        let theme = Theme::classic();
        let mut o = Overlay::new(120, 40);
        o.update(0.0, fiesta);
        o.hung.clear();
        o.ties.clear();
        o.cover.fill(UPPER);
        for _ in 0..50 {
            o.update(0.1, fiesta);
            let mut c = scene(120, 40);
            o.draw(&mut c, &theme, fiesta);
            // A fleck covers two pixels at most, a ribbon its length (and a
            // sideways step now and then).
            let ribbons: usize = o.ribbons.iter().map(|r| 3 * r.len as usize).sum();
            let n = changed(&c, 120).len();
            assert!(n > 0 && n <= 2 * o.petals.len() + ribbons, "{n} cells");
        }
    }

    #[test]
    fn the_serpentinas_curl_as_they_fall() {
        let fiesta = day(9);
        let mut o = Overlay::new(100, 40);
        o.update(0.0, fiesta);
        o.hung.clear();
        o.petals.clear();
        o.ties.clear();
        let theme = Theme::classic();
        // Over a few seconds they move down and the curls move too.
        let first = {
            let mut c = scene(100, 40);
            o.draw(&mut c, &theme, fiesta);
            c.to_text()
        };
        o.update(1.0, fiesta);
        let mut c = scene(100, 40);
        o.draw(&mut c, &theme, fiesta);
        assert_ne!(first, c.to_text());
        assert!(o.ribbons.iter().all(|r| r.len >= 10 && r.fall > 0.0));
    }

    #[test]
    fn nothing_on_an_ordinary_day() {
        for fiesta in [
            Fiesta::NONE,
            Fiesta::of_day(6, 15),
            Fiesta::of_date(2027, 2, 3),
            Fiesta::of_date(2027, 2, 10),
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
        assert!(on(&mut o, day(4)));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, day(9)));
        assert!(!on(&mut o, Fiesta::of_date(2027, 2, 10)));
        // The second string and what falls come and go with the days.
        o.update(0.0, day(5));
        assert!(o.hung.iter().all(|h| h.len == 0));
        o.update(0.0, day(8));
        assert!(o.hung.iter().any(|h| h.len == 1));
        o.update(0.0, day(5));
        assert!(o.hung.iter().all(|h| h.len == 0));
        // Día de Muertos and the Navidad take their turn.
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert!(on(&mut o, Fiesta::of_day(12, 31)));
        assert!(on(&mut o, day(9)));
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
                for fiesta in [day(4), day(9)] {
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
        let fiesta = day(8);
        let frame = || after(&mut Overlay::new(90, 28), fiesta, 6.0, &theme).to_text();
        assert_eq!(frame(), frame());
    }
}
