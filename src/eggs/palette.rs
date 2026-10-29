//! What every holiday's visitors share: their colors.
//!
//! A holiday gives each letter of its visitors' pixel art a color under
//! `classic` and a role under the other themes ([`Role`]): each hue becomes
//! the theme accent nearest to it, shades are taken from those, bone is the
//! theme's foreground and the dark inks are mixed from its background, so the
//! visitors belong to the theme; a few colors (a candle flame) keep their own.

use crate::color::{self, Color, rgb};
use crate::theme::Theme;

/// How a letter is colored under a theme other than `classic`.
#[derive(Clone, Copy)]
pub enum Role {
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

/// A visitor's colors under a theme, by letter.
pub struct Palette([Color; 128]);

impl Palette {
    /// The colors of `colors` (letter, color under `classic`, role under
    /// other themes; letters that a `Shade` refers to come first) under
    /// `theme`.
    pub fn with(theme: &Theme, colors: &[(u8, u32, Role)]) -> Palette {
        let mut pal = [Color::Reset; 128];
        for &(ch, hex, role) in colors {
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

    const COLORS: &[(u8, u32, Role)] = &[
        (b'R', 0xe4007c, Role::Hue),
        (b'r', 0xff6eb4, Role::Shade(b'R', 1.4)),
        (b'W', 0xf4eee2, Role::Bone(1.0)),
        (b'k', 0x1c1424, Role::Ink(0.06)),
        (b'F', 0xffd678, Role::Keep),
    ];

    #[test]
    fn classic_keeps_its_colors_and_themes_their_own() {
        let pal = Palette::with(&Theme::classic(), COLORS);
        assert_eq!(pal.of(b'R'), rgb(0xe4007c));
        assert_eq!(pal.get(b'.'), None);
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::with(&t, COLORS);
        assert!(t.colors.contains(&pal.of(b'R')));
        assert_eq!(pal.of(b'r'), pal.of(b'R').scale(1.4));
        assert_eq!(pal.of(b'F'), rgb(0xffd678));
        for &(ch, ..) in COLORS {
            assert!(pal.get(ch).is_some(), "{}", ch as char);
        }
    }
}
