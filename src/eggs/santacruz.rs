//! What the Día de la Santa Cruz visitor shares with its holiday: the colors.
//!
//! Under `classic` they are the colors of a building site on May 3: cal (the
//! lime of the walls and the gray of fresh concrete), the steel of the
//! varilla, tabique red, the wood of the scaffolding and the cross, and the
//! papel flowers (rosa mexicano, amarillo, morado and verde) that cover the
//! cross. Under any other theme each hue becomes the theme accent nearest to
//! it, whites are the theme's foreground and the steel and the dark inks are
//! mixed from its background.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Cal, and its shade.
    (b'L', 0xf2efe6, Role::Bone(0.95)),
    (b'l', 0xc4c0b2, Role::Bone(0.7)),
    // Varilla, the steel rods, and the darker ties around them.
    (b'V', 0x8a9098, Role::Ink(0.52)),
    (b'v', 0x555a62, Role::Ink(0.32)),
    // Tabique, the red brick, and the shade of its edges.
    (b'T', 0xb5532f, Role::Hue),
    (b't', 0x8a3c20, Role::Shade(b'T', 0.78)),
    // Wood.
    (b'D', 0xa05a28, Role::Shade(b'T', 0.95)),
    (b'd', 0x6a3a18, Role::Shade(b'T', 0.6)),
    // The papel flowers and their leaves.
    (b'P', 0xe4007c, Role::Hue),
    (b'Y', 0xffd200, Role::Hue),
    (b'M', 0x7028a0, Role::Hue),
    (b'G', 0x2fb84a, Role::Hue),
    (b'g', 0x1c7a2e, Role::Shade(b'G', 0.62)),
    // The builders' shirts, skin and trousers, and the sparks of the cohetes.
    (b'R', 0xd8402a, Role::Hue),
    (b'B', 0x2a5caa, Role::Hue),
    (b'O', 0xff8c00, Role::Hue),
    (b'K', 0xd7a066, Role::Hue),
    (b'W', 0xf4f1ea, Role::Bone(1.0)),
    (b'n', 0x3a4468, Role::Ink(0.22)),
    (b'k', 0x1c1424, Role::Ink(0.06)),
];

impl Palette {
    /// The Santa Cruz visitor's colors under a theme.
    pub fn santa_cruz(theme: &Theme) -> Palette {
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
            let pal = Palette::santa_cruz(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::santa_cruz(&Theme::classic());
        assert_eq!(pal.of(b'L'), rgb(0xf2efe6));
        assert_eq!(pal.of(b'T'), rgb(0xb5532f));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::santa_cruz(&t);
        assert!(t.colors.contains(&pal.of(b'P')));
        assert!(t.colors.contains(&pal.of(b'G')));
    }

    /// The Santa Cruz visitor comes on May 3, and never on an ordinary day
    /// or on another holiday's.
    #[test]
    fn the_visitors_keep_to_santa_cruz() {
        for name in ["cruz"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::SantaCruz, "{name}");
            let comes = |fiesta: Fiesta| {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                e.active()
            };
            assert!(comes(Fiesta::of_day(5, 3)), "{name} on Santa Cruz");
            assert!(!comes(Fiesta::NONE), "{name} came on an ordinary day");
            for (m, d) in [(5, 2), (5, 4), (6, 15), (8, 20), (2, 2), (4, 30), (11, 2)] {
                assert!(!comes(Fiesta::of_day(m, d)), "{name} came on {m}-{d}");
            }
        }
    }
}
