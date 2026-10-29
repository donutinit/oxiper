//! Fiestas Patrias: papel picado in verde, blanco y rojo with patriotic
//! cut-outs and banderitas between the flags, and tricolor confetti.

use super::drift::{Flecks, tone};
use super::papel::{Design, Papel};
use super::{Fiesta, Look, Themed};
use crate::color::rgb;

/// The cut-outs: the águila, the Dolores bell, a star, a sombrero de charro,
/// a nopal and the flag on its pole.
const DESIGNS: &[Design] = &[
    // águila
    [
        "XXXXXXX", "XXX.XXX", "X.....X", "XX...XX", "XXX.XXX", "XX.X.XX", "XXXXXXX", "X.X.X.X",
    ],
    // campana
    [
        "XXXXXXX", "XXX.XXX", "XX...XX", "XX...XX", "X.....X", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
    // estrella
    [
        "XXXXXXX", "XXX.XXX", "X.....X", "XX...XX", "XX.X.XX", "X.XXX.X", "XXXXXXX", "X.X.X.X",
    ],
    // sombrero
    [
        "XXXXXXX", "XXX.XXX", "XX...XX", "XX...XX", ".......", "XXXXXXX", "XXXXXXX", "X.X.X.X",
    ],
    // nopal
    [
        "XXXXXXX", "XXX.XXX", "XX.X.XX", "X.....X", "XX.X.XX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
    // bandera
    [
        "XXXXXXX", "X.....X", "X.....X", "X.....X", "X.XXXXX", "X.XXXXX", "XXXXXXX", "X.X.X.X",
    ],
];

/// The paper in verde bandera, blanco and rojo, in that order along the
/// strings, and the cord, natural string.
const PAPEL: Papel = Papel {
    designs: DESIGNS,
    colors: [
        rgb(0x006847), // verde
        rgb(0xf4f1ea), // blanco
        rgb(0xce1126), // rojo
        rgb(0x006847),
        rgb(0xf4f1ea),
        rgb(0xce1126),
    ],
    cord: rgb(0xbcae94),
    // Room for a banderita between two flags.
    gap: 7,
    ordered: true,
    banderitas: true,
    themed: Themed::Tricolor,
};

/// Confetti in the three colors.
const CONFETTI: Flecks = Flecks {
    tones: &[tone(0x00855a), tone(0xf4f1ea), tone(0xe0182d)],
    share: 0,
    cells: 150,
    themed: Themed::Tricolor,
};

/// What the Fiestas Patrias hang: the same on each day (the second string
/// comes with the Grito and the parade, the confetti with the days of the
/// month).
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 4,
        salt: 0x7061_7472_6961_7321,
        papel: Some(&PAPEL),
        flecks: Some(&CONFETTI),
        ..Look::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{after, changed, scene};
    use super::super::{LOWER_LEVEL, Overlay, cord::Piece, tricolor};
    use super::*;
    use crate::color::Color;
    use crate::fiesta::Kind;
    use crate::render::Canvas;
    use crate::theme::Theme;

    /// Cells the overlay changed in rows `from..to`.
    fn cells(c: &Canvas, w: u16, from: i32, to: i32) -> usize {
        changed(c, w)
            .iter()
            .filter(|&&(_, y)| (from..to).contains(&y))
            .count()
    }

    #[test]
    fn dresses_every_day_of_the_fiestas_patrias() {
        let theme = Theme::classic();
        for day in [1, 8, 14, 15, 16] {
            let fiesta = Fiesta::of_day(9, day);
            assert!(fiesta.is(Kind::Patrias));
            let mut o = Overlay::new(120, 36);
            let c = after(&mut o, fiesta, 4.0, &theme);
            let (top, lower, below) = (
                cells(&c, 120, 0, 4),
                cells(&c, 120, 4, 6),
                cells(&c, 120, 6, 36),
            );
            assert!(top > 250, "9-{day}: {top} cells of papel picado");
            assert!(below > 0, "9-{day}: no confetti");
            // The second string at the Grito and the parade.
            if fiesta.level >= LOWER_LEVEL {
                assert!(lower > 60, "9-{day}: {lower} cells");
            } else {
                assert!(lower < 15, "9-{day}: {lower} cells");
            }
        }
        // More confetti as the month goes by.
        let count = |day| {
            let mut o = Overlay::new(200, 60);
            let c = after(&mut o, Fiesta::of_day(9, day), 2.0, &theme);
            cells(&c, 200, 6, 60)
        };
        assert!(count(15) > count(1) + 10);
    }

    #[test]
    fn the_flags_follow_the_tricolor() {
        let mut o = Overlay::new(120, 36);
        for day in [3, 15] {
            o.update(0.0, Fiesta::of_day(9, day));
            let upper: Vec<u8> = o.flags[..o.upper].iter().map(|f| f.color).collect();
            assert!(upper.len() > 3);
            assert!(upper.iter().enumerate().all(|(i, &c)| c as usize == i % 3));
            // The lower string's flags are neither of the colors of the two
            // upper flags either side of them.
            for f in &o.flags[o.upper..] {
                let left = o.flags[..o.upper].iter().rfind(|u| u.x < f.x).unwrap();
                let right = o.flags[..o.upper].iter().find(|u| u.x > f.x).unwrap();
                assert!(f.color != left.color && f.color != right.color, "{}", f.x);
            }
            // A banderita between each two flags of the upper string.
            let banderitas = o.hung.iter().filter(|h| h.piece == Piece::Banderita);
            assert_eq!(banderitas.count(), upper.len() - 1);
        }
    }

    #[test]
    fn papel_picado_keeps_to_its_rows() {
        let theme = Theme::classic();
        for (fiesta, rows) in [(Fiesta::of_day(9, 8), 4), (Fiesta::of_day(9, 15), 6)] {
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
    fn confetti_touches_only_its_cells() {
        let fiesta = Fiesta::of_day(9, 15);
        let mut o = Overlay::new(120, 40);
        o.update(0.0, fiesta);
        o.flags.clear();
        o.upper = 0;
        o.hung.clear();
        o.cover.fill(super::super::UPPER);
        o.ties.clear();
        for _ in 0..50 {
            o.update(0.1, fiesta);
            let mut c = scene(120, 40);
            o.draw(&mut c, &Theme::classic(), fiesta);
            // Each piece covers two pixels at most.
            let n = changed(&c, 120).len();
            assert!(n > 0 && n <= 2 * o.petals.len(), "{n} cells");
        }
    }

    #[test]
    fn the_theme_paints_the_flag() {
        // Green, white and red under any theme: the accents closest to green
        // and red, the foreground for white.
        let classic = tricolor(&Theme::classic());
        assert_eq!(classic[0], rgb(0x006847));
        for name in ["nord", "dracula", "gruvbox", "catppuccin"] {
            let theme = Theme::builtin(name).unwrap();
            let [green, white, red] = tricolor(&theme);
            assert_eq!(white, theme.fg, "{name}");
            assert!(theme.colors.contains(&green) && theme.colors.contains(&red));
            let Color::Rgb(r, g, _) = green else { panic!() };
            assert!(g > r, "{name}: {green:?} is not green");
            let Color::Rgb(r, g, b) = red else { panic!() };
            assert!(r > g && r > b, "{name}: {red:?} is not red");
        }
    }

    #[test]
    fn nothing_on_an_ordinary_day() {
        for fiesta in [Fiesta::NONE, Fiesta::of_day(6, 15), Fiesta::of_day(8, 20)] {
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
        assert!(on(&mut o, Fiesta::of_day(9, 1)));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, Fiesta::of_day(9, 15)));
        assert!(!on(&mut o, Fiesta::of_day(8, 20)));
        // The second string from the Grito on, hung anew each day.
        o.update(0.0, Fiesta::of_day(9, 14));
        assert_eq!(o.upper, o.flags.len());
        o.update(0.0, Fiesta::of_day(9, 15));
        assert!(o.upper < o.flags.len());
        o.update(0.0, Fiesta::of_day(9, 14));
        assert_eq!(o.upper, o.flags.len());
        // Día de Muertos and the Navidad take their turn.
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert!(on(&mut o, Fiesta::of_day(12, 20)));
        assert!(on(&mut o, Fiesta::of_day(9, 16)));
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
                for fiesta in [Fiesta::of_day(9, 1), Fiesta::of_day(9, 15)] {
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
        let fiesta = Fiesta::of_day(9, 15);
        let frame = || after(&mut Overlay::new(90, 28), fiesta, 6.0, &theme).to_text();
        assert_eq!(frame(), frame());
    }
}
