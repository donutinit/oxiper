//! What the Equinoccio visitors share: their colors.
//!
//! Under `classic` they are the colors of Chichén Itzá on the day the sun
//! crosses the equator: piedra caliza (the limestone of El Castillo) with its
//! sombra, the sol itself, jade for the feathers of the serpent and the cielo
//! behind it. Under any other theme each hue becomes the theme accent nearest
//! to it, the stone is the theme's foreground, and the shadows are mixed from
//! its background.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Piedra caliza, lightest to darkest.
    (b'P', 0xc9b48a, Role::Bone(0.74)),
    (b'p', 0xa39068, Role::Shade(b'P', 0.8)),
    (b'q', 0x7a6a4c, Role::Shade(b'P', 0.58)),
    // Sombra.
    (b'S', 0x3b3226, Role::Ink(0.1)),
    (b's', 0x5a4c38, Role::Ink(0.22)),
    // Sol, its light, and the ember it turns into when it sets.
    (b'Y', 0xffcf4a, Role::Hue),
    (b'y', 0xfff0a8, Role::Shade(b'Y', 1.25)),
    (b'O', 0xff8a30, Role::Hue),
    // Jade, the plumes of Kukulcán, lighter and darker.
    (b'J', 0x2e8b57, Role::Hue),
    (b'j', 0x58c98a, Role::Shade(b'J', 1.4)),
    // Cielo.
    (b'C', 0x7ec8e3, Role::Hue),
    // The people who come to see it: white clothes, a red or yellow top,
    // skin, hair and trousers; and the serpent's red tongue.
    (b'W', 0xf4f1ea, Role::Bone(1.0)),
    (b'R', 0xc8402a, Role::Hue),
    (b'K', 0xd7a066, Role::Hue),
    (b'n', 0x3a3a52, Role::Ink(0.18)),
];

impl Palette {
    /// The Equinoccio visitors' colors under a theme.
    pub fn equinoccio(theme: &Theme) -> Palette {
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
            let pal = Palette::equinoccio(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::equinoccio(&Theme::classic());
        assert_eq!(pal.of(b'P'), rgb(0xc9b48a));
        assert_eq!(pal.of(b'J'), rgb(0x2e8b57));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::equinoccio(&t);
        assert!(t.colors.contains(&pal.of(b'Y')));
        assert!(t.colors.contains(&pal.of(b'J')));
    }

    /// The Equinoccio visitors come on March 19 to 22, and never on an
    /// ordinary day or on another holiday's.
    #[test]
    fn the_visitors_keep_to_equinoccio() {
        for name in ["kukulcan"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Equinoccio, "{name}");
            let comes = |fiesta: Fiesta| {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                e.active()
            };
            for (m, d) in [(3, 19), (3, 20), (3, 21), (3, 22)] {
                assert!(comes(Fiesta::of_day(m, d)), "{name} on {m}-{d}");
            }
            assert!(!comes(Fiesta::NONE), "{name} came on an ordinary day");
            for (m, d) in [
                (3, 18),
                (3, 23),
                (6, 15),
                (8, 20),
                (2, 2),
                (11, 2),
                (12, 24),
            ] {
                assert!(!comes(Fiesta::of_day(m, d)), "{name} came on {m}-{d}");
            }
        }
    }
}
