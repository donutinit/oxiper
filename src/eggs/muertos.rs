//! What the Día de Muertos visitors share: their colors.
//!
//! Every visitor's pixel art uses the same letters. Under `classic` they are
//! the colors of the holiday: cempasúchil, rosa mexicano, terciopelo, morado,
//! turquesa, amarillo, verde limón, sugar-white bone and candlelight. Under
//! any other theme each hue becomes the theme accent nearest to it, shades
//! are taken from those, bone is the theme's foreground and the dark inks are
//! mixed from its background, so the visitors belong to the theme; only the
//! candle flame keeps its own color.

use crate::color::{self, Color, rgb};
use crate::theme::Theme;

/// How a letter is colored under a theme other than `classic`.
#[derive(Clone, Copy)]
enum Role {
    /// The theme accent nearest in hue.
    Hue,
    /// Another letter's color, darker (below 1) or lighter (above 1).
    Shade(u8, f32),
    /// The theme's foreground, scaled.
    Bone(f32),
    /// Mixed from the theme's background (0) toward its foreground (1).
    Ink(f32),
    /// Its own color under every theme.
    Keep,
}

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

/// The visitors' colors under a theme, by letter.
pub struct Palette([Color; 128]);

impl Palette {
    pub fn new(theme: &Theme) -> Palette {
        let mut pal = [Color::Reset; 128];
        for &(ch, hex, role) in COLORS {
            let native = rgb(hex);
            pal[ch as usize] = if theme.native {
                native
            } else {
                match role {
                    Role::Hue => nearest(theme, native),
                    Role::Shade(of, k) => pal[of as usize].scale(k),
                    Role::Bone(k) => match theme.fg {
                        Color::Rgb(..) => theme.fg.scale(k),
                        _ => native,
                    },
                    Role::Ink(k) => match (theme.bg, theme.fg) {
                        (Color::Rgb(..), Color::Rgb(..)) => theme.bg.scale(0.6).lerp(theme.fg, k),
                        _ => native,
                    },
                    Role::Keep => native,
                }
            };
        }
        Palette(pal)
    }

    /// The color of a letter; `None` for letters with none.
    #[inline]
    pub fn get(&self, ch: u8) -> Option<Color> {
        match self.0[ch as usize & 127] {
            Color::Reset => None,
            c => Some(c),
        }
    }

    /// Fades every color toward the theme's background (see [`fade`]).
    pub fn fade_all(&mut self, theme: &Theme, k: f32) {
        for c in self.0.iter_mut().filter(|c| **c != Color::Reset) {
            *c = fade(theme, *c, k);
        }
    }

    /// The color of a letter, which must have one.
    #[inline]
    pub fn of(&self, ch: u8) -> Color {
        self.0[ch as usize & 127]
    }
}

/// Fades a color toward the theme's background (`k = 1` is gone); without a
/// known background, darkening is the closest thing.
pub fn fade(theme: &Theme, c: Color, k: f32) -> Color {
    match theme.bg {
        Color::Rgb(..) => c.lerp(theme.bg, k),
        _ => c.scale(1.0 - k),
    }
}

/// The theme accent nearest in hue to `c`, for a theme other than `classic`.
fn nearest(theme: &Theme, c: Color) -> Color {
    let Some(want) = hue(c) else {
        return theme.fg;
    };
    theme
        .colors
        .iter()
        .filter_map(|&t| {
            let h = hue(t)?;
            let d = (h - want).abs();
            Some((d.min(360.0 - d), t))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map_or_else(|| theme.pick(0, c), |(_, t)| t)
}

/// Hue in degrees; `None` for grays. Palette colors use their usual RGB.
fn hue(c: Color) -> Option<f32> {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => color::ansi_rgb(n),
        Color::Reset => return None,
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    if d < 0.08 {
        return None;
    }
    let h = if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    Some(h)
}

#[cfg(test)]
mod tests {
    use super::*;

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
