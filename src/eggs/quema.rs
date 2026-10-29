//! What the Quema de Judas visitor shares: its colors.
//!
//! The pixel art uses these letters. Under `classic` they are the colors of
//! a cartonería Judas: cartón rojo and black with yellow trim, the bright
//! papel de china of its fringe, newsprint white, rope, smoke, and the
//! white-gold of sparks. Under any other theme each hue becomes the theme
//! accent nearest to it, shades are taken from those, the dark inks are
//! mixed from the theme's background and the smoke from its foreground, so
//! the Judas belongs to the theme; only the fire keeps its own color.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Cartón rojo, with its highlights and shadows.
    (b'R', 0xd1262b, Role::Hue),
    (b'r', 0xee5a55, Role::Shade(b'R', 1.3)),
    (b'D', 0x8f1a1e, Role::Shade(b'R', 0.68)),
    (b'd', 0x5c1014, Role::Shade(b'R', 0.45)),
    // Amarillo.
    (b'Y', 0xffd23b, Role::Hue),
    (b'y', 0xb8901c, Role::Shade(b'Y', 0.68)),
    // Papel de china.
    (b'P', 0xe4007c, Role::Hue),
    (b'Q', 0x00aab8, Role::Hue),
    (b'G', 0x8cc828, Role::Hue),
    (b'M', 0x7028a0, Role::Hue),
    // Newsprint and teeth.
    (b'W', 0xf4eee2, Role::Bone(1.0)),
    (b'w', 0xc8bfae, Role::Bone(0.8)),
    // Black, with its highlights.
    (b'k', 0x1c1c1c, Role::Ink(0.04)),
    (b'N', 0x4a4a52, Role::Ink(0.26)),
    (b'n', 0x7a7a84, Role::Ink(0.5)),
    // Rope.
    (b'C', 0xc9a06a, Role::Shade(b'Y', 0.8)),
    (b'c', 0x8a6a3a, Role::Shade(b'Y', 0.55)),
    // Smoke, light and dark.
    (b'S', 0xb4b4bc, Role::Ink(0.6)),
    (b's', 0x6e6e78, Role::Ink(0.34)),
    // Fire: white-gold, orange and hot red.
    (b'F', 0xfff2b0, Role::Keep),
    (b'f', 0xff9628, Role::Keep),
    (b'H', 0xff4a1c, Role::Keep),
];

impl Palette {
    /// The Quema de Judas visitor's colors under a theme.
    pub fn quema(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;
    use crate::eggs::{ALL, Eggs};
    use crate::fiesta::{Fiesta, Kind};

    /// The Judas comes on Viernes Santo and Sábado de Gloria (which move
    /// with Easter), and never on an ordinary day or during another holiday.
    #[test]
    fn the_visitors_keep_to_the_quema_de_judas() {
        let info = ALL.iter().find(|i| i.name == "judas").unwrap();
        assert_eq!(info.fiesta, Kind::Judas);
        let others = [
            Fiesta::NONE,
            Fiesta::of_date(2027, 6, 15),
            Fiesta::of_date(2027, 2, 9),
            Fiesta::of_date(2027, 3, 20),
            Fiesta::of_date(2027, 7, 19),
            Fiesta::of_date(2027, 11, 2),
            Fiesta::of_date(2027, 12, 24),
            // The days around: Easter itself, and Jueves Santo.
            Fiesta::of_date(2027, 3, 25),
            Fiesta::of_date(2027, 3, 28),
        ];
        for fiesta in others {
            let mut e = Eggs::new(80, 24);
            e.start(&[info], fiesta);
            assert!(!e.active(), "the Judas came on {:?}", fiesta.kind);
        }
        for (y, m, d) in [(2027, 3, 26), (2027, 3, 27), (2026, 4, 3), (2026, 4, 4)] {
            let fiesta = Fiesta::of_date(y, m, d);
            assert!(fiesta.is(Kind::Judas), "{y}-{m}-{d}");
            let mut e = Eggs::new(80, 24);
            e.start(&[info], fiesta);
            assert!(e.active(), "the Judas on {y}-{m}-{d}");
        }
    }

    #[test]
    fn every_letter_has_a_color_under_every_theme() {
        for name in ["classic", "dracula", "nord", "gruvbox", "mono"] {
            let Some(t) = Theme::builtin(name) else {
                continue;
            };
            let pal = Palette::quema(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::quema(&Theme::classic());
        assert_eq!(pal.of(b'R'), rgb(0xd1262b));
        assert_eq!(pal.of(b'k'), rgb(0x1c1c1c));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::quema(&t);
        assert!(t.colors.contains(&pal.of(b'R')));
        assert_eq!(pal.of(b'F'), rgb(0xfff2b0));
    }
}
