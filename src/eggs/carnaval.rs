//! What the Carnaval visitors share: their colors.
//!
//! Every visitor's pixel art uses the same letters. Under `classic` they are
//! the colors of Carnaval: the plumas in rosa mexicano, turquesa, morado and
//! verde limón, lentejuela dorada, confetti in every bright color, the velvet
//! of a chinelo's robe, the pale mask with its black beard and the three
//! skin tones of the comparsa. Under any other theme each hue becomes the
//! theme accent nearest to it, shades are taken from those, bone is the
//! theme's foreground and the dark inks are mixed from its background, so
//! the visitors belong to the theme.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Rosa mexicano.
    (b'R', 0xe4007c, Role::Hue),
    (b'r', 0xff6eb4, Role::Shade(b'R', 1.4)),
    (b'T', 0xa8005c, Role::Shade(b'R', 0.7)),
    // Turquesa.
    (b'Q', 0x00aab8, Role::Hue),
    (b'z', 0x6fe0e8, Role::Shade(b'Q', 1.4)),
    (b'q', 0x00697d, Role::Shade(b'Q', 0.62)),
    // Morado.
    (b'M', 0x7028a0, Role::Hue),
    (b'P', 0xaa64dc, Role::Shade(b'M', 1.35)),
    (b'm', 0x46166e, Role::Shade(b'M', 0.6)),
    // Verde limón.
    (b'G', 0x8cc828, Role::Hue),
    (b'l', 0xc4ee60, Role::Shade(b'G', 1.3)),
    (b'g', 0x327828, Role::Shade(b'G', 0.55)),
    // Amarillo, and the lentejuela dorada of the sequins.
    (b'Y', 0xffd200, Role::Hue),
    (b'L', 0xffcc33, Role::Shade(b'Y', 1.0)),
    (b'a', 0xa87820, Role::Shade(b'Y', 0.62)),
    // Naranja, rojo and azul.
    (b'O', 0xff8c00, Role::Hue),
    (b'o', 0xffb228, Role::Shade(b'O', 1.3)),
    (b'U', 0xd92b2b, Role::Hue),
    (b'u', 0x8e1a1a, Role::Shade(b'U', 0.62)),
    (b'B', 0x2a62d0, Role::Hue),
    (b'b', 0x173c80, Role::Shade(b'B', 0.6)),
    // Skin, light, medium and dark, each with its shadow.
    (b'S', 0xf0c09a, Role::Shade(b'O', 1.35)),
    (b's', 0xc88a5e, Role::Shade(b'O', 0.95)),
    (b'H', 0xc98a58, Role::Shade(b'O', 1.05)),
    (b'h', 0x9a5f38, Role::Shade(b'O', 0.75)),
    (b'J', 0x8a5230, Role::Shade(b'O', 0.7)),
    (b'j', 0x5e3420, Role::Shade(b'O', 0.5)),
    // Bone: the white of a mask, a glove, a collar.
    (b'W', 0xf4eee2, Role::Bone(1.0)),
    (b'w', 0xc9bfb0, Role::Bone(0.78)),
    // Inks: the beard, the hair, the shadows.
    (b'k', 0x1c1424, Role::Ink(0.06)),
    (b'N', 0x2c2238, Role::Ink(0.14)),
    (b'n', 0x5a4c6e, Role::Ink(0.32)),
    // Dark brown hair.
    (b'K', 0x40261a, Role::Ink(0.2)),
];

impl Palette {
    /// The Carnaval visitors' colors under a theme.
    pub fn carnaval(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;
    use crate::eggs::{ALL, Eggs};
    use crate::fiesta::{Fiesta, Kind};

    /// The Carnaval visitors come on the days of Carnaval (Thursday to
    /// Martes de Carnaval, which move with Easter), and never on an ordinary
    /// day or during another holiday.
    #[test]
    fn the_visitors_keep_to_carnaval() {
        let days = [
            (2027, 2, 4),
            (2027, 2, 7),
            (2027, 2, 9),
            (2026, 2, 12),
            (2026, 2, 17),
        ];
        for name in ["chinelos", "comparsa"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Carnaval, "{name}");
            let others = [
                Fiesta::NONE,
                Fiesta::of_date(2027, 6, 15),
                Fiesta::of_date(2027, 11, 2),
                Fiesta::of_date(2027, 12, 24),
                Fiesta::of_date(2027, 3, 26),
                Fiesta::of_date(2027, 7, 19),
            ];
            for fiesta in others {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                assert!(!e.active(), "{name} came on {:?}", fiesta.kind);
            }
            for (y, m, d) in days {
                let fiesta = Fiesta::of_date(y, m, d);
                assert!(fiesta.is(Kind::Carnaval), "{y}-{m}-{d}");
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                assert!(e.active(), "{name} on {y}-{m}-{d}");
            }
        }
    }

    #[test]
    fn every_letter_has_a_color_under_every_theme() {
        for name in ["classic", "dracula", "nord", "gruvbox", "mono"] {
            let Some(t) = Theme::builtin(name) else {
                continue;
            };
            let pal = Palette::carnaval(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::carnaval(&Theme::classic());
        assert_eq!(pal.of(b'R'), rgb(0xe4007c));
        assert_eq!(pal.of(b'L'), rgb(0xffcc33));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::carnaval(&t);
        assert!(t.colors.contains(&pal.of(b'R')));
        assert!(t.colors.contains(&pal.of(b'Q')));
    }
}
