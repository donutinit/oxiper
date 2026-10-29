//! What the Fiestas Patrias visitors share: their colors.
//!
//! Every visitor's pixel art uses the same letters. Under `classic` they are
//! the colors of the holiday: verde bandera, blanco, rojo, the águila's café,
//! the nopal's green, oro for the bell and the embroidery, the charro's black
//! and silver, and stone, skin and lake for the rest. Under any other theme
//! each hue becomes the theme accent nearest to it, shades are taken from
//! those, white is the theme's foreground and the dark inks are mixed from
//! its background, so the visitors belong to the theme; only the glow of the
//! águila's sunburst keeps its own gold.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Verde bandera.
    (b'G', 0x006847, Role::Hue),
    (b'g', 0x00402c, Role::Shade(b'G', 0.62)),
    (b'l', 0x1faa72, Role::Shade(b'G', 1.65)),
    // Blanco.
    (b'W', 0xf4eee2, Role::Bone(1.0)),
    (b'w', 0xbeb4a8, Role::Bone(0.75)),
    // Rojo.
    (b'R', 0xce1126, Role::Hue),
    (b'r', 0xf0505e, Role::Shade(b'R', 1.4)),
    (b't', 0x8a0a1a, Role::Shade(b'R', 0.62)),
    // The águila's café, and wood.
    (b'E', 0x7a4a1e, Role::Hue),
    (b'e', 0xa8702e, Role::Shade(b'E', 1.4)),
    (b'd', 0x4a2a10, Role::Shade(b'E', 0.6)),
    // Oro: the bell, the embroidery, the águila's nape.
    (b'O', 0xd4a017, Role::Hue),
    (b'o', 0xf6cf4a, Role::Shade(b'O', 1.25)),
    (b'a', 0x8e6a0c, Role::Shade(b'O', 0.66)),
    // Nopal, with its spines and its tunas.
    (b'N', 0x4f8a2a, Role::Hue),
    (b'n', 0x2f5a1a, Role::Shade(b'N', 0.62)),
    (b'm', 0x9ccc58, Role::Shade(b'N', 1.45)),
    (b'P', 0xc8265a, Role::Hue),
    // Skin.
    (b'F', 0xd89a6a, Role::Shade(b'E', 1.6)),
    (b'f', 0xa8683e, Role::Shade(b'E', 1.15)),
    // Inks: black, charcoal and silver.
    (b'k', 0x1a1a22, Role::Ink(0.06)),
    (b'K', 0x2e2c38, Role::Ink(0.14)),
    (b'S', 0xc9ccd6, Role::Ink(0.78)),
    (b's', 0x8a8e9c, Role::Ink(0.5)),
    // Stone of the palace and the rock.
    (b'Z', 0xb4a48c, Role::Bone(0.72)),
    (b'z', 0x85775f, Role::Bone(0.5)),
    // The lake the rock rises from.
    (b'Q', 0x2a8fb0, Role::Hue),
    (b'q', 0x1e6a86, Role::Shade(b'Q', 0.7)),
    // The glow of the águila's sunburst.
    (b'Y', 0xffe27a, Role::Keep),
];

impl Palette {
    /// The Fiestas Patrias visitors' colors under a theme.
    pub fn patrias(theme: &Theme) -> Palette {
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
            let pal = Palette::patrias(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_flag_and_themes_their_own() {
        let pal = Palette::patrias(&Theme::classic());
        assert_eq!(pal.of(b'G'), rgb(0x006847));
        assert_eq!(pal.of(b'R'), rgb(0xce1126));
        assert_eq!(pal.of(b'W'), rgb(0xf4eee2));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::patrias(&t);
        assert!(t.colors.contains(&pal.of(b'G')));
        assert!(t.colors.contains(&pal.of(b'R')));
        assert_eq!(pal.of(b'Y'), rgb(0xffe27a));
    }
}
