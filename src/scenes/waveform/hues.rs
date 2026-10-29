//! Holiday colors under a theme: each classic color becomes the theme's
//! accent closest to it in hue (a Christmas bulb is red under any theme),
//! and grays and whites the theme's foreground.

use crate::color::Color;
use crate::theme::Theme;

/// The hue of a color in degrees, and its saturation (0 for a gray).
fn hue(c: Color) -> Option<(f32, f32)> {
    let Color::Rgb(r, g, b) = c else {
        return None;
    };
    let (r, g, b) = (r as f32, g as f32, b as f32);
    let (hi, lo) = (r.max(g).max(b), r.min(g).min(b));
    if hi <= 0.0 || hi == lo {
        return Some((0.0, 0.0));
    }
    let d = hi - lo;
    let h = if hi == r {
        (g - b) / d
    } else if hi == g {
        2.0 + (b - r) / d
    } else {
        4.0 + (r - g) / d
    };
    Some(((h * 60.0).rem_euclid(360.0), d / hi))
}

/// The theme's accent closest in hue to `target` degrees, among the ones that
/// have a color to speak of.
fn nearest_hue(theme: &Theme, target: f32) -> Option<Color> {
    let off = |c: &&Color| match hue(**c) {
        Some((h, s)) if s > 0.2 => {
            let d = (h - target).abs();
            (d.min(360.0 - d) * 1000.0) as u32
        }
        _ => u32::MAX,
    };
    theme
        .colors
        .iter()
        .min_by_key(off)
        .filter(|c| off(c) < u32::MAX)
        .copied()
}

/// A theme color that can be painted (`Reset` would be a hole).
fn solid(c: Option<&Color>, fallback: Color) -> Color {
    match c {
        Some(&c) if c != Color::Reset => c,
        _ => fallback,
    }
}

/// The theme color closest in hue to a classic one (the classic color itself
/// under `classic`, or where the theme has nothing like it): a gray or white
/// one is the theme's foreground.
pub(super) fn like(theme: &Theme, native: Color) -> Color {
    if theme.native {
        return native;
    }
    match hue(native) {
        Some((target, s)) if s > 0.2 => solid(nearest_hue(theme, target).as_ref(), native),
        Some(_) => solid(Some(&theme.fg), native),
        None => native,
    }
}
