//! Día de la Revolución: a string of banderines in verde, blanco y rojo.

use super::bunting::Bunting;
use super::{Fiesta, Look, Themed};
use crate::color::rgb;

/// The pennants in verde bandera, blanco and rojo, in that order along the
/// string, on a cord the color of caqui.
const BUNTING: Bunting = Bunting {
    colors: [
        rgb(0x006847), // verde
        rgb(0xf4f1ea), // blanco
        rgb(0xce1126), // rojo
        rgb(0x006847),
        rgb(0xf4f1ea),
        rgb(0xce1126),
    ],
    ordered: true,
    cord: rgb(0x8b7d5b),
    two: false,
    themed: Themed::Tricolor,
};

/// What the Revolución hangs: one string of bunting, on its one day.
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 6,
        salt: 0x7265_766f_6c75_6369,
        bunting: Some(&BUNTING),
        ..Look::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::super::Overlay;
    use super::super::cord::Piece;
    use super::super::tests::{after, changed, scene};
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
    fn dresses_the_day_of_the_revolucion() {
        let fiesta = Fiesta::of_day(11, 20);
        assert!(fiesta.is(Kind::Revolucion));
        let mut o = Overlay::new(120, 36);
        let c = after(&mut o, fiesta, 4.0, &Theme::classic());
        let top = cells(&c, 120, 0, 3);
        assert!(top > 150, "{top} cells of banderines");
        // Nothing else: the bunting is all there is.
        assert_eq!(cells(&c, 120, 3, 36), 0);
    }

    #[test]
    fn the_banderines_follow_the_tricolor() {
        let mut o = Overlay::new(120, 36);
        o.update(0.0, Fiesta::of_day(11, 20));
        let pennants: Vec<u8> = o
            .hung
            .iter()
            .filter(|h| h.piece == Piece::Pennant)
            .map(|h| h.color)
            .collect();
        assert!(pennants.len() > 10);
        assert!(
            pennants
                .iter()
                .enumerate()
                .all(|(i, &c)| c as usize == i % 3)
        );
        // One string only.
        assert!(o.hung.iter().all(|h| h.len == 0));
        // Side by side, never overlapping.
        let xs: Vec<i32> = o.hung.iter().map(|h| h.x).collect();
        assert!(xs.windows(2).all(|p| p[1] - p[0] >= 8));
    }

    #[test]
    fn the_bunting_keeps_to_three_rows_and_sways() {
        let theme = Theme::classic();
        let fiesta = Fiesta::of_day(11, 20);
        for w in [9, 40, 80, 121, 200] {
            let mut o = Overlay::new(w, 30);
            o.update(0.0, fiesta);
            let mut frames = Vec::new();
            for step in 0..60 {
                o.update(0.17, fiesta);
                let mut c = scene(w, 30);
                o.draw(&mut c, &theme, fiesta);
                assert_eq!(cells(&c, w, 3, 30), 0, "{w} wide, step {step}");
                if w >= 40 {
                    assert!(cells(&c, w, 0, 3) > w as usize, "{w} wide");
                }
                frames.push(c.to_text());
            }
            if w >= 40 {
                frames.dedup();
                assert!(frames.len() > 10, "{w} wide: it does not sway");
            }
        }
    }

    #[test]
    fn the_tricolor_follows_the_theme() {
        let fiesta = Fiesta::of_day(11, 20);
        let classic = after(&mut Overlay::new(100, 30), fiesta, 2.0, &Theme::classic());
        let nord = Theme::builtin("nord").unwrap();
        let themed = after(&mut Overlay::new(100, 30), fiesta, 2.0, &nord);
        assert_ne!(classic.cells(), themed.cells());
        assert_eq!(cells(&themed, 100, 0, 30), cells(&classic, 100, 0, 30));
    }

    #[test]
    fn nothing_on_an_ordinary_day() {
        for fiesta in [Fiesta::NONE, Fiesta::of_day(6, 15), Fiesta::of_day(11, 19)] {
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
        assert!(on(&mut o, Fiesta::of_day(11, 20)));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, Fiesta::of_day(11, 20)));
        assert!(!on(&mut o, Fiesta::of_day(11, 21)));
        // Día de Muertos the day it is on, the bunting after it.
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert!(on(&mut o, Fiesta::of_day(11, 20)));
        assert!(o.hung.iter().any(|h| h.piece == Piece::Pennant));
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        let fiesta = Fiesta::of_day(11, 20);
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

    #[test]
    fn same_frames_every_time() {
        let theme = Theme::classic();
        let fiesta = Fiesta::of_day(11, 20);
        let frame = || after(&mut Overlay::new(90, 28), fiesta, 6.0, &theme).to_text();
        assert_eq!(frame(), frame());
    }
}
