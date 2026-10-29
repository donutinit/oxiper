//! What the Navidad visitors share: their colors.
//!
//! Every visitor's pixel art uses the same letters. Under `classic` they are
//! the colors of the Guadalupe–Reyes: rojo nochebuena, verde pino, dorado,
//! the papel de china of a piñata (rosa mexicano, amarillo, naranja,
//! turquesa, morado, verde limón), skin, wool and wood, sparks and candle
//! flames. Under any other theme each hue becomes the theme accent nearest
//! to it, shades are taken from those, white is the theme's foreground and
//! the dark inks are mixed from its background, so the visitors belong to
//! the theme; only the flames and sparks keep their own colors.

use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::color::Color;
use crate::render::{Canvas, Cell};
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Rojo nochebuena.
    (b'R', 0xc8102e, Role::Hue),
    (b'r', 0xf0506a, Role::Shade(b'R', 1.4)),
    (b'D', 0x7c0a1c, Role::Shade(b'R', 0.6)),
    // Verde pino.
    (b'G', 0x1f6f3a, Role::Hue),
    (b'g', 0x3fa05a, Role::Shade(b'G', 1.45)),
    (b'h', 0x124625, Role::Shade(b'G', 0.62)),
    // Dorado.
    (b'Y', 0xf2b632, Role::Hue),
    (b'y', 0xffdc78, Role::Shade(b'Y', 1.25)),
    (b'd', 0xa8741c, Role::Shade(b'Y', 0.62)),
    // Papel de china: rosa mexicano, naranja, amarillo, turquesa, morado
    // and verde limón.
    (b'P', 0xe4007c, Role::Hue),
    (b'p', 0xff6eb4, Role::Shade(b'P', 1.4)),
    (b'O', 0xff8c00, Role::Hue),
    (b'o', 0xffb228, Role::Shade(b'O', 1.3)),
    (b'Z', 0xffd200, Role::Hue),
    (b'Q', 0x00aab8, Role::Hue),
    (b'q', 0x00697d, Role::Shade(b'Q', 0.62)),
    (b'M', 0x7028a0, Role::Hue),
    (b'm', 0xaa64dc, Role::Shade(b'M', 1.35)),
    (b'L', 0x8cc828, Role::Hue),
    // Azul: María's mantle, San Miguel's wings.
    (b'A', 0x2a5caa, Role::Hue),
    (b'a', 0x1a3c78, Role::Shade(b'A', 0.62)),
    (b'I', 0x6a98e0, Role::Shade(b'A', 1.4)),
    // Skin, and the tejocote.
    (b'e', 0xe0a878, Role::Hue),
    (b'E', 0xb07848, Role::Shade(b'e', 0.78)),
    (b'T', 0xe8891c, Role::Hue),
    // Wood, straw (the caña) and camel hide.
    (b'B', 0x8b4a1e, Role::Shade(b'O', 0.62)),
    (b'b', 0x5a2e12, Role::Shade(b'O', 0.4)),
    (b'C', 0xc9b26b, Role::Shade(b'Y', 0.95)),
    (b'c', 0xc49a58, Role::Shade(b'Y', 0.85)),
    // Gray: the burro and the elephant.
    (b'U', 0x8d8a94, Role::Ink(0.5)),
    (b'V', 0xc9c6cc, Role::Ink(0.72)),
    (b'v', 0x5f5c68, Role::Ink(0.3)),
    // White wool and silver.
    (b'W', 0xf4f1ea, Role::Bone(1.0)),
    (b'w', 0xbdb9ae, Role::Bone(0.75)),
    (b'S', 0xc9ccd6, Role::Ink(0.78)),
    (b's', 0x8a8e9c, Role::Ink(0.55)),
    // Inks: black, and the shadows of the night.
    (b'k', 0x1c1424, Role::Ink(0.06)),
    (b'N', 0x2c2238, Role::Ink(0.14)),
    (b'n', 0x5a4c6e, Role::Ink(0.32)),
    // Candlelight and sparks.
    (b'F', 0xffd678, Role::Keep),
    (b'f', 0xff9628, Role::Keep),
    (b'X', 0xfff4c0, Role::Keep),
];

impl Palette {
    /// The Navidad visitors' colors under a theme.
    pub fn navidad(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

/// Splits `text` in two lines at the space nearest its middle, for a
/// screen too narrow for it in one.
pub(super) fn split_in_two(text: &str) -> [&str; 2] {
    let mid = text.chars().count() as i32 / 2;
    let nearest = text
        .char_indices()
        .filter(|&(_, ch)| ch == ' ')
        .min_by_key(|&(i, _)| (text[..i].chars().count() as i32 - mid).abs());
    match nearest {
        Some((i, _)) => [&text[..i], &text[i + 1..]],
        None => [text, ""],
    }
}

/// Types `lines` out, centered on the screen from row `y` down, as far as
/// `shown` letters (counted across the lines) have come, each letter on the
/// screen's own background so it reads over any scene; `cursor` puts a
/// block after the last letter typed.
pub(super) fn type_out(
    c: &mut Canvas,
    y: i32,
    lines: &[&str],
    shown: usize,
    (ink, cursor): (Color, Option<Color>),
    attr: u8,
) {
    let (w, bg) = (c.w() as i32, c.bg());
    let put = |c: &mut Canvas, x: i32, y: i32, ch: char, fg: Color| {
        c.set_cell(x, y, Cell { ch, fg, bg, attr });
    };
    let mut left = shown;
    for (row, line) in lines.iter().enumerate() {
        let len = line.chars().count();
        if len == 0 {
            continue;
        }
        let (n, y) = (left.min(len), y + row as i32);
        // Centered with a space of margin, or from the edge on a narrow screen.
        let x = (w - len as i32).div_euclid(2).max(1);
        if n > 0 {
            put(c, x - 1, y, ' ', ink);
        }
        for (i, ch) in line.chars().take(n).enumerate() {
            put(c, x + i as i32, y, ch, ink);
        }
        if n == len {
            put(c, x + len as i32, y, ' ', ink);
        }
        if left < len {
            if let Some(cur) = cursor {
                put(c, x + n as i32, y, '▌', cur);
            }
            return;
        }
        left -= len;
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
            let pal = Palette::navidad(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn letters_are_not_used_twice() {
        for (i, a) in COLORS.iter().enumerate() {
            assert!(
                COLORS[i + 1..].iter().all(|b| a.0 != b.0),
                "{}",
                a.0 as char
            );
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::navidad(&Theme::classic());
        assert_eq!(pal.of(b'R'), rgb(0xc8102e));
        assert_eq!(pal.of(b'P'), rgb(0xe4007c));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::navidad(&t);
        assert!(t.colors.contains(&pal.of(b'G')));
        assert!(t.colors.contains(&pal.of(b'P')));
        assert_eq!(pal.of(b'F'), rgb(0xffd678));
    }
}
