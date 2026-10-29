//! What the Candelaria visitors share: their colors.
//!
//! Under `classic` they are the colors of February 2: hoja de maíz (the corn
//! husk the tamales come wrapped in), masa, atole, the blue, white and gold of
//! the Niño Dios' gown, and the aluminum of the tamalera. Under any other
//! theme each hue becomes the theme accent nearest to it, shades are taken
//! from those, whites are the theme's foreground and the metal and the dark
//! inks are mixed from its background.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Hoja de maíz.
    (b'Y', 0xd9c48a, Role::Hue),
    (b'y', 0xa8924f, Role::Shade(b'Y', 0.78)),
    // Masa and the cream of the sign.
    (b'M', 0xf2e2b0, Role::Bone(0.95)),
    (b'm', 0xc8b888, Role::Bone(0.72)),
    // Atole.
    (b'A', 0xc89a6a, Role::Hue),
    // Wood.
    (b'D', 0xa05a28, Role::Shade(b'A', 0.8)),
    (b'd', 0x6e3b18, Role::Shade(b'A', 0.5)),
    // The gown's azul, and its dorado.
    (b'B', 0x2a5caa, Role::Hue),
    (b'b', 0x1b3d75, Role::Shade(b'B', 0.62)),
    (b'G', 0xf2b632, Role::Hue),
    // Rojo, for a cap and the letters on the sign.
    (b'R', 0xd8402a, Role::Hue),
    (b'r', 0x8f2414, Role::Shade(b'R', 0.65)),
    // White: the apron, the steam.
    (b'W', 0xf4f1ea, Role::Bone(1.0)),
    (b'w', 0xc4c0b8, Role::Bone(0.76)),
    // Aluminum, lightest to darkest.
    (b'S', 0xdadde6, Role::Ink(0.84)),
    (b's', 0x9ea3b2, Role::Ink(0.58)),
    (b'z', 0x60667a, Role::Ink(0.36)),
    // Skin.
    (b'K', 0xd7a066, Role::Hue),
    // Inks: black, the tires, and the trousers (the far leg in the shade).
    (b'k', 0x1c1424, Role::Ink(0.06)),
    (b'N', 0x3a3650, Role::Ink(0.2)),
    (b'J', 0x5a6ca2, Role::Ink(0.34)),
    (b'j', 0x3b4878, Role::Ink(0.22)),
];

impl Palette {
    /// The Candelaria visitors' colors under a theme.
    pub fn candelaria(theme: &Theme) -> Palette {
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
            let pal = Palette::candelaria(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::candelaria(&Theme::classic());
        assert_eq!(pal.of(b'Y'), rgb(0xd9c48a));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::candelaria(&t);
        assert!(t.colors.contains(&pal.of(b'B')));
        assert!(t.colors.contains(&pal.of(b'R')));
    }

    /// The Candelaria visitors come on February 2, and never on an ordinary
    /// day or on another holiday's.
    #[test]
    fn the_visitors_keep_to_candelaria() {
        for name in ["tamalero"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Candelaria, "{name}");
            let comes = |fiesta: Fiesta| {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                e.active()
            };
            assert!(comes(Fiesta::of_day(2, 2)), "{name} on Candelaria");
            assert!(!comes(Fiesta::NONE), "{name} came on an ordinary day");
            for (m, d) in [(6, 15), (8, 20), (11, 2), (9, 15), (12, 24), (5, 3)] {
                assert!(!comes(Fiesta::of_day(m, d)), "{name} came on {m}-{d}");
            }
        }
    }
}
