//! What the Día del Niño visitors share: their colors.
//!
//! Under `classic` they are the bright colors of the toys of every Mexican
//! childhood: rojo, amarillo, azul, verde, naranja, morado, rosa mexicano,
//! turquesa, the madera of the old wooden ones and the cream of a lotería
//! card. Under any other theme each hue becomes the theme accent nearest to
//! it, shades are taken from those, whites are the theme's foreground and
//! the metal and the dark inks are mixed from its background.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Rojo.
    (b'R', 0xe8322a, Role::Hue),
    (b'r', 0x9a1c16, Role::Shade(b'R', 0.65)),
    // Amarillo.
    (b'Y', 0xffd200, Role::Hue),
    (b'y', 0xc89a00, Role::Shade(b'Y', 0.75)),
    // Azul.
    (b'B', 0x1f7ae0, Role::Hue),
    (b'b', 0x124a94, Role::Shade(b'B', 0.62)),
    // Verde.
    (b'G', 0x2fb84a, Role::Hue),
    (b'g', 0x1c7a2e, Role::Shade(b'G', 0.62)),
    // Naranja, and the madera of old toys.
    (b'O', 0xff8c00, Role::Hue),
    (b'D', 0xc08a4a, Role::Shade(b'O', 0.8)),
    (b'd', 0x8a5a28, Role::Shade(b'O', 0.5)),
    // Morado, rosa mexicano and turquesa.
    (b'M', 0x7028a0, Role::Hue),
    (b'P', 0xe4007c, Role::Hue),
    (b'T', 0x00aab8, Role::Hue),
    // Skin.
    (b'K', 0xd7a066, Role::Hue),
    // Cream, the paper of a card.
    (b'C', 0xf4e9c8, Role::Bone(0.95)),
    (b'c', 0xd8c898, Role::Bone(0.72)),
    // White and its shadow.
    (b'W', 0xf4f1ea, Role::Bone(1.0)),
    (b'w', 0xbdb8ae, Role::Bone(0.72)),
    // Steel, and the ink.
    (b'S', 0xdadde6, Role::Ink(0.84)),
    (b's', 0x9ea3b2, Role::Ink(0.58)),
    (b'k', 0x1c1424, Role::Ink(0.06)),
];

impl Palette {
    /// The Día del Niño visitors' colors under a theme.
    pub fn nino(theme: &Theme) -> Palette {
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
            let pal = Palette::nino(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_toy_colors_and_themes_their_own() {
        let pal = Palette::nino(&Theme::classic());
        assert_eq!(pal.of(b'R'), rgb(0xe8322a));
        assert_eq!(pal.of(b'D'), rgb(0xc08a4a));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::nino(&t);
        assert!(t.colors.contains(&pal.of(b'B')));
        assert!(t.colors.contains(&pal.of(b'G')));
    }

    /// The Día del Niño visitors come on April 30, and never on an ordinary
    /// day or on another holiday's.
    #[test]
    fn the_visitors_keep_to_nino() {
        for name in ["juguetes", "loteria"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Nino, "{name}");
            let comes = |fiesta: Fiesta| {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                e.active()
            };
            assert!(comes(Fiesta::of_day(4, 30)), "{name} on Día del Niño");
            assert!(!comes(Fiesta::NONE), "{name} came on an ordinary day");
            for (m, d) in [(4, 29), (5, 1), (6, 15), (8, 20), (2, 2), (5, 3), (11, 2)] {
                assert!(!comes(Fiesta::of_day(m, d)), "{name} came on {m}-{d}");
            }
        }
    }
}
