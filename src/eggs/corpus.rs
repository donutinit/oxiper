//! What the Jueves de Corpus visitors share: the colors.
//!
//! Under `classic` they are the colors of the Thursday of Corpus Christi:
//! hoja de maíz (the husk that the mulitas are made of, pale and toasted),
//! manta (the white cotton of the children's clothes), the blue of a rebozo,
//! the wood of a huacal, the fruit that fills it (mango, tuna, limón, naranja)
//! and the flowers of paper (rosa mexicano, amarillo, morado) among their
//! leaves; and, for the voladores of Papantla, the red of their trousers, the
//! white of their shirts, the mirrors and the ribbons of their headdresses in
//! every bright color. Under any other theme each hue becomes the theme
//! accent nearest to it, shades are taken from those, whites are the theme's
//! foreground and the dark inks and the silver of the mirrors are mixed from
//! its background.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Hoja de maíz, the husk of a mulita: pale, toasted and its lightest.
    (b'H', 0xd9c48a, Role::Hue),
    (b'h', 0xb08d4a, Role::Shade(b'H', 0.8)),
    (b'I', 0xece0b0, Role::Shade(b'H', 1.12)),
    // Manta, the white cotton of a shirt, and its shadow; plain white.
    (b'A', 0xefe6cf, Role::Bone(0.95)),
    (b'a', 0xc9bfa6, Role::Bone(0.72)),
    (b'W', 0xf4f1ea, Role::Bone(1.0)),
    (b'w', 0xbdb8ae, Role::Bone(0.72)),
    // The azul of a rebozo, its dark and its stripes.
    (b'B', 0x2a5caa, Role::Hue),
    (b'b', 0x1c3f78, Role::Shade(b'B', 0.65)),
    (b'C', 0x7da6e8, Role::Shade(b'B', 1.5)),
    // Fruit: naranja first (the wood is shaded from it), mango, tuna, limón.
    (b'O', 0xff8c1a, Role::Hue),
    (b'M', 0xffb02e, Role::Hue),
    (b'T', 0xc2185b, Role::Hue),
    (b'L', 0x7cb342, Role::Hue),
    // The wood of a huacal and of a puesto, its dark, and the straw of a hat
    // and the fiber of a rope.
    (b'D', 0xa0703a, Role::Shade(b'O', 0.8)),
    (b'd', 0x6a4520, Role::Shade(b'O', 0.5)),
    (b'E', 0xc9a96a, Role::Shade(b'H', 0.95)),
    // Paper flowers: rosa mexicano, amarillo and morado; their leaves.
    (b'P', 0xe4007c, Role::Hue),
    (b'Y', 0xffd200, Role::Hue),
    (b'V', 0x7028a0, Role::Hue),
    (b'G', 0x2f9a3a, Role::Hue),
    (b'g', 0x1c6a28, Role::Shade(b'G', 0.62)),
    // Skin and its shadow; hair and the dark inks.
    (b'K', 0xd7a066, Role::Hue),
    (b'k', 0xa87040, Role::Shade(b'K', 0.78)),
    (b'N', 0x2a1a14, Role::Ink(0.2)),
    (b'n', 0x1c1424, Role::Ink(0.06)),
    // The voladores: rojo of their trousers and its shade, turquesa of a
    // ribbon, the silver of a mirror and its shade, a glint.
    (b'R', 0xc8102e, Role::Hue),
    (b'r', 0x8a0a20, Role::Shade(b'R', 0.65)),
    (b'Q', 0x00aab8, Role::Hue),
    (b'S', 0xdadde6, Role::Ink(0.84)),
    (b's', 0x9ea3b2, Role::Ink(0.58)),
    (b'F', 0xfff6c8, Role::Bone(1.05)),
    // The bare trunk of a voladores' pole, and its shade.
    (b'U', 0xb89868, Role::Shade(b'O', 0.95)),
    (b'u', 0x7a6038, Role::Shade(b'O', 0.6)),
];

impl Palette {
    /// The Jueves de Corpus visitors' colors under a theme.
    pub fn corpus(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ALL, Eggs};
    use super::*;
    use crate::color::rgb;
    use crate::fiesta::{Fiesta, Kind};

    #[test]
    fn every_letter_has_a_color_under_every_theme() {
        for name in ["classic", "dracula", "nord", "gruvbox", "mono"] {
            let Some(t) = Theme::builtin(name) else {
                continue;
            };
            let pal = Palette::corpus(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::corpus(&Theme::classic());
        assert_eq!(pal.of(b'H'), rgb(0xd9c48a));
        assert_eq!(pal.of(b'B'), rgb(0x2a5caa));
        assert_eq!(pal.of(b'R'), rgb(0xc8102e));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::corpus(&t);
        assert!(t.colors.contains(&pal.of(b'P')));
        assert!(t.colors.contains(&pal.of(b'B')));
    }

    /// The Corpus visitors come on the Thursday of Corpus Christi (which moves
    /// with Easter), and never on an ordinary day or on another holiday's.
    #[test]
    fn the_visitors_keep_to_corpus() {
        for name in ["mulitas", "voladores"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Corpus, "{name}");
            let comes = |fiesta: Fiesta| {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                e.active()
            };
            for (y, m, d) in [(2027, 5, 27), (2028, 6, 15), (2026, 6, 4)] {
                let fiesta = Fiesta::of_date(y, m, d);
                assert!(fiesta.is(Kind::Corpus), "{y}-{m}-{d}");
                assert!(comes(fiesta), "{name} on {y}-{m}-{d}");
            }
            assert!(!comes(Fiesta::NONE), "{name} came on an ordinary day");
            for (y, m, d) in [
                (2027, 5, 26),
                (2027, 5, 28),
                (2027, 5, 3),
                (2027, 6, 15),
                (2027, 8, 20),
                (2027, 2, 2),
                (2027, 11, 2),
                (2027, 12, 24),
            ] {
                assert!(
                    !comes(Fiesta::of_date(y, m, d)),
                    "{name} came on {y}-{m}-{d}"
                );
            }
        }
    }
}
