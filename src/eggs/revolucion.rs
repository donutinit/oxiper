//! What the Día de la Revolución visitors share: their colors.
//!
//! Every visitor's pixel art uses the same letters. Under `classic` they are
//! the colors of the photographs and the flag: caqui, sepia, manta, the
//! brass of the cananas, the tricolor, soot and smoke. Under any other theme
//! each hue becomes the theme accent nearest to it, shades are taken from
//! those, manta is the theme's foreground and the dark inks are mixed from
//! its background, so the visitors belong to the theme; only the sparks of
//! the locomotive keep their own fire.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Caqui: the uniform, the straw and the dust.
    (b'C', 0x8b7d5b, Role::Hue),
    (b'c', 0x5e5238, Role::Shade(b'C', 0.68)),
    (b'h', 0xb8a87e, Role::Shade(b'C', 1.3)),
    // Sepia: wood, leather, the old photographs.
    (b'B', 0x704214, Role::Hue),
    (b'b', 0x452708, Role::Shade(b'B', 0.6)),
    (b'u', 0xa06a30, Role::Shade(b'B', 1.4)),
    // Manta: the unbleached cotton of blouses, banners and rebozos.
    (b'M', 0xefe6cf, Role::Bone(1.0)),
    (b'm', 0xbfb497, Role::Bone(0.78)),
    // Brass: the cananas, the locomotive's bell and dome.
    (b'A', 0xb8913a, Role::Hue),
    (b'a', 0x7a5f1e, Role::Shade(b'A', 0.66)),
    (b'o', 0xe6c466, Role::Shade(b'A', 1.3)),
    // The tricolor.
    (b'G', 0x006847, Role::Hue),
    (b'g', 0x00402c, Role::Shade(b'G', 0.62)),
    (b'R', 0xce1126, Role::Hue),
    (b'r', 0xf0505e, Role::Shade(b'R', 1.4)),
    (b't', 0x8a0a1a, Role::Shade(b'R', 0.62)),
    // Ocher, for a skirt or a blanket.
    (b'O', 0xc9892c, Role::Hue),
    (b'd', 0x8c5e18, Role::Shade(b'O', 0.7)),
    // Skin.
    (b'F', 0xcf9468, Role::Shade(b'B', 1.55)),
    (b'f', 0xa06a42, Role::Shade(b'B', 1.1)),
    // Inks: black, soot and steel.
    (b'k', 0x1c1a1a, Role::Ink(0.06)),
    (b'K', 0x34302c, Role::Ink(0.16)),
    (b'S', 0x9aa0aa, Role::Ink(0.6)),
    // Smoke, dark and pale.
    (b'x', 0x8a8a90, Role::Ink(0.42)),
    (b'X', 0xc4c4ca, Role::Ink(0.7)),
    // Sparks from the stack.
    (b'Y', 0xffb23a, Role::Keep),
    (b'y', 0xff6a1a, Role::Keep),
];

impl Palette {
    /// The Día de la Revolución visitors' colors under a theme.
    pub fn revolucion(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;

    #[test]
    fn every_letter_has_a_color_under_every_theme() {
        for name in ["classic", "dracula", "nord", "gruvbox", "mono"] {
            let Some(t) = Theme::builtin(name) else {
                continue;
            };
            let pal = Palette::revolucion(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::revolucion(&Theme::classic());
        assert_eq!(pal.of(b'C'), rgb(0x8b7d5b));
        assert_eq!(pal.of(b'A'), rgb(0xb8913a));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::revolucion(&t);
        assert!(t.colors.contains(&pal.of(b'G')));
        assert!(t.colors.contains(&pal.of(b'R')));
        assert_eq!(pal.of(b'Y'), rgb(0xffb23a));
    }
}
