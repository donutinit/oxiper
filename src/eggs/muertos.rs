//! What the Día de Muertos visitors share: their colors.
//!
//! Every visitor's pixel art uses the same letters. Under `classic` they are
//! the colors of the holiday: cempasúchil, rosa mexicano, terciopelo, morado,
//! turquesa, amarillo, verde limón, sugar-white bone and candlelight. Under
//! any other theme each hue becomes the theme accent nearest to it, shades
//! are taken from those, bone is the theme's foreground and the dark inks are
//! mixed from its background, so the visitors belong to the theme; only the
//! candle flame keeps its own color.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Cempasúchil.
    (b'O', 0xff8c00, Role::Hue),
    (b'o', 0xffb228, Role::Shade(b'O', 1.3)),
    (b'd', 0xc85a00, Role::Shade(b'O', 0.75)),
    // Rosa mexicano and terciopelo.
    (b'R', 0xe4007c, Role::Hue),
    (b'r', 0xff6eb4, Role::Shade(b'R', 1.4)),
    (b'T', 0xb00048, Role::Shade(b'R', 0.72)),
    (b't', 0x780032, Role::Shade(b'R', 0.5)),
    // Morado.
    (b'M', 0x7028a0, Role::Hue),
    (b'm', 0x46166e, Role::Shade(b'M', 0.6)),
    (b'P', 0xaa64dc, Role::Shade(b'M', 1.35)),
    // Turquesa.
    (b'Q', 0x00aab8, Role::Hue),
    (b'q', 0x00697d, Role::Shade(b'Q', 0.62)),
    // Amarillo, and the brass of a trumpet.
    (b'Y', 0xffd200, Role::Hue),
    (b'A', 0xf0c850, Role::Shade(b'Y', 0.95)),
    (b'a', 0xa87820, Role::Shade(b'Y', 0.62)),
    // Verde limón and leaves.
    (b'G', 0x8cc828, Role::Hue),
    (b'g', 0x327828, Role::Shade(b'G', 0.55)),
    // Wood.
    (b'B', 0xa05a28, Role::Shade(b'O', 0.62)),
    (b'b', 0x64371a, Role::Shade(b'O', 0.4)),
    // Bone and sugar.
    (b'W', 0xf4eee2, Role::Bone(1.0)),
    (b'w', 0xbeb4a8, Role::Bone(0.75)),
    // Inks: black, the charro suit and its silver.
    (b'k', 0x1c1424, Role::Ink(0.06)),
    (b'N', 0x2c2238, Role::Ink(0.14)),
    (b'n', 0x5a4c6e, Role::Ink(0.32)),
    (b'S', 0xc8c8d7, Role::Ink(0.78)),
    // Candlelight.
    (b'F', 0xffd678, Role::Keep),
    (b'f', 0xff9628, Role::Keep),
];

impl Palette {
    /// The Día de Muertos visitors' colors under a theme.
    pub fn new(theme: &Theme) -> Palette {
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
            let pal = Palette::new(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::new(&Theme::classic());
        assert_eq!(pal.of(b'O'), rgb(0xff8c00));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::new(&t);
        assert!(t.colors.contains(&pal.of(b'R')));
        assert!(t.colors.contains(&pal.of(b'Q')));
        assert_eq!(pal.of(b'F'), rgb(0xffd678));
    }
}
