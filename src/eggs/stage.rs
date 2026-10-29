//! A stage for the holiday visitors: the screen as a plane of art pixels.
//!
//! The Fiestas Patrias and Revolución visitors are drawn in art pixels that
//! become `k` half-block pixels across and down on a big terminal, so a
//! visitor keeps its proportions on a 200x60 screen as on a 100x30 one (and
//! the same hand-drawn art serves both). The stage is anchored at the bottom
//! of the screen: row `h - 1` is the last pixel row, the ground.
//!
//! It also has what these visitors need that plain `sprite` and `pixel`
//! don't: filled rectangles and lines, and a 5x7 block font with the
//! accents Spanish wants, for letters big enough to read across a room.

use super::{paint, sprite};
use crate::color::Color;
use crate::render::Canvas;

/// Screen pixels (half blocks) per art pixel, and the stage's size in art
/// pixels: wide enough and tall enough to cover the whole screen.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Stage {
    pub k: i32,
    pub w: i32,
    pub h: i32,
    /// The screen pixel row of the stage's row 0: zero or a little above the
    /// screen, so the last row is the screen's last.
    top: i32,
}

impl Stage {
    /// The stage for a screen of `cols` x `rows` cells: art pixels grow with
    /// the screen's height (a stage about 60 to 100 pixels tall) as long as
    /// the width still leaves it about 100 pixels across.
    pub fn new(cols: u16, rows: u16) -> Stage {
        let (cols, px) = (i32::from(cols), i32::from(rows) * 2);
        let k = ((px + 36) / 72).min(cols / 100).max(1);
        let h = (px + k - 1) / k;
        Stage {
            k,
            w: (cols + k - 1) / k,
            h,
            top: px - h * k,
        }
    }

    /// The terminal cell (column, row) that art pixel (`x`, `y`) is in, for
    /// the text a visitor writes with the terminal's own font.
    pub fn cell(&self, x: i32, y: i32) -> (i32, i32) {
        (x * self.k, (y * self.k + self.top).div_euclid(2))
    }

    /// Fills `w` x `h` art pixels with `color`, the top-left one at (`x`, `y`).
    pub fn rect(&self, c: &mut Canvas, x: i32, y: i32, w: i32, h: i32, color: Color) {
        if w <= 0 || h <= 0 {
            return;
        }
        let k = self.k;
        let (x0, x1) = ((x * k).max(0), ((x + w) * k).min(c.w() as i32));
        let (y0, y1) = (
            (y * k + self.top).max(0),
            ((y + h) * k + self.top).min(c.h() as i32 * 2),
        );
        if x0 >= x1 || y0 >= y1 {
            return;
        }
        for row in y0.div_euclid(2)..=(y1 - 1).div_euclid(2) {
            let covers = |p: i32| (y0..y1).contains(&p).then_some(color);
            let (up, down) = (covers(row * 2), covers(row * 2 + 1));
            for col in x0..x1 {
                paint(c, col, row, up, down);
            }
        }
    }

    /// One art pixel.
    #[inline]
    pub fn px(&self, c: &mut Canvas, x: i32, y: i32, color: Color) {
        self.rect(c, x, y, 1, 1, color);
    }

    /// A line of art pixels, `thick` of them across (1 or 2).
    pub fn line(
        &self,
        c: &mut Canvas,
        (x0, y0): (i32, i32),
        (x1, y1): (i32, i32),
        thick: i32,
        color: Color,
    ) {
        let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let (mut x, mut y, mut err) = (x0, y0, dx + dy);
        // Never more steps than the line is long (and so never forever).
        for _ in 0..=dx.max(-dy) {
            self.rect(c, x, y, thick, thick, color);
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draws pixel art like [`sprite`], `k` times as large: the art's
    /// top-left pixel at (`x`, `y`).
    pub fn sprite(
        &self,
        c: &mut Canvas,
        (x, y): (i32, i32),
        rows: &[&str],
        flip: bool,
        shift: impl Fn(i32) -> i32,
        color: impl Fn(u8) -> Option<Color>,
    ) {
        if self.k == 1 {
            return sprite(c, (x, y), rows, flip, shift, color);
        }
        let k = self.k;
        let w = rows.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
        let h = rows.len() as i32;
        let at = |r: i32, vx: i32| -> Option<Color> {
            let line = rows.get(usize::try_from(r).ok()?)?.as_bytes();
            let sx = vx - x - shift(r);
            if !(0..w).contains(&sx) {
                return None;
            }
            match *line.get((if flip { w - 1 - sx } else { sx }) as usize)? {
                b'.' | b' ' => None,
                ch => color(ch),
            }
        };
        let first = (y * k + self.top).div_euclid(2).max(0);
        let last = ((y + h) * k + self.top - 1)
            .div_euclid(2)
            .min(c.h() as i32 - 1);
        for row in first..=last {
            let art_row = |half: i32| (row * 2 + half - self.top).div_euclid(k) - y;
            let (r0, r1) = (art_row(0), art_row(1));
            let span = |r: i32| (0..h).contains(&r).then(|| x + shift(r));
            let (lo, hi) = match (span(r0), span(r1)) {
                (Some(a), Some(b)) => (a.min(b), a.max(b) + w),
                (Some(a), None) | (None, Some(a)) => (a, a + w),
                (None, None) => continue,
            };
            for col in (lo * k).max(0)..(hi * k).min(c.w() as i32) {
                let vx = col.div_euclid(k);
                paint(c, col, row, at(r0, vx), at(r1, vx));
            }
        }
    }

    /// Draws one letter of the block font, `s` art pixels to a font pixel,
    /// its top-left at (`x`, `y`); with an `edge`, a pixel-wide outline
    /// around it so it reads on any background. Letters the font does not
    /// have draw nothing.
    pub fn glyph(
        &self,
        c: &mut Canvas,
        (x, y): (i32, i32),
        ch: char,
        s: i32,
        color: Color,
        edge: Option<Color>,
    ) {
        let Some((rows, accent)) = glyph(ch) else {
            return;
        };
        let mut draw = |dx: i32, dy: i32, col: Color| {
            for (r, bits) in rows.iter().enumerate() {
                for b in 0..5 {
                    if bits >> (4 - b) & 1 == 1 {
                        let (px, py) = (x + b * s + dx, y + r as i32 * s + dy);
                        self.rect(c, px, py, s, s, col);
                    }
                }
            }
            if accent {
                self.rect(c, x + 3 * s + dx, y - 2 * s + dy, s, s, col);
                self.rect(c, x + 2 * s + dx, y - s + dy, s, s, col);
            }
        };
        if let Some(edge) = edge {
            for (dx, dy) in [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (-1, -1),
                (1, 1),
                (-1, 1),
                (1, -1),
            ] {
                draw(dx, dy, edge);
            }
        }
        draw(0, 0, color);
    }
}

/// A cheap, repeatable mix of two numbers: the same crowd member, the same
/// confetto, always the same dice, with no state to keep.
pub(crate) fn hash(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9e37_79b1) ^ b.wrapping_mul(0x85eb_ca6b) ^ 0x27d4_eb2f;
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297a_2d39);
    h ^ h >> 15
}

/// [`hash`] as a number from 0 up to (not including) 1.
pub(crate) fn unit(a: u32, b: u32) -> f32 {
    (hash(a, b) & 0xffff) as f32 / 65536.0
}

/// How far the pen moves after a letter, in font pixels.
pub(crate) fn advance(ch: char) -> i32 {
    if ch == ' ' { 3 } else { 6 }
}

/// The width of `text` in the block font at `s` art pixels to a font pixel.
pub(crate) fn text_width(text: &str, s: i32) -> i32 {
    (text.chars().map(advance).sum::<i32>() - 1).max(0) * s
}

/// The 5x7 block font: seven rows per letter, bit 4 the leftmost pixel; and
/// whether the letter wears an accent (drawn above its top).
fn glyph(ch: char) -> Option<([u8; 7], bool)> {
    let (base, accent) = match ch {
        'Á' => ('A', true),
        'É' => ('E', true),
        'Í' => ('I', true),
        'Ó' => ('O', true),
        'Ú' => ('U', true),
        ch => (ch.to_ascii_uppercase(), false),
    };
    let rows = match base {
        'A' => [0x0e, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'B' => [0x1e, 0x11, 0x11, 0x1e, 0x11, 0x11, 0x1e],
        'C' => [0x0e, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0e],
        'D' => [0x1e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1e],
        'E' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x1f],
        'F' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x10],
        'G' => [0x0e, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0f],
        'H' => [0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'I' => [0x0e, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0e],
        'J' => [0x07, 0x02, 0x02, 0x02, 0x02, 0x12, 0x0c],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1f],
        'M' => [0x11, 0x1b, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x11, 0x19, 0x15, 0x13, 0x11, 0x11],
        'O' => [0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'P' => [0x1e, 0x11, 0x11, 0x1e, 0x10, 0x10, 0x10],
        'Q' => [0x0e, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0d],
        'R' => [0x1e, 0x11, 0x11, 0x1e, 0x14, 0x12, 0x11],
        'S' => [0x0f, 0x10, 0x10, 0x0e, 0x01, 0x01, 0x1e],
        'T' => [0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0a, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1b, 0x11],
        'X' => [0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0a, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1f],
        '0' => [0x0e, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0e],
        '1' => [0x04, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x0e],
        '2' => [0x0e, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1f],
        '3' => [0x1f, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0e],
        '4' => [0x02, 0x06, 0x0a, 0x12, 0x1f, 0x02, 0x02],
        '5' => [0x1f, 0x10, 0x1e, 0x01, 0x01, 0x11, 0x0e],
        '6' => [0x06, 0x08, 0x10, 0x1e, 0x11, 0x11, 0x0e],
        '7' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e],
        '9' => [0x0e, 0x11, 0x11, 0x0f, 0x01, 0x02, 0x0c],
        '!' => [0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x04],
        '¡' => [0x04, 0x00, 0x04, 0x04, 0x04, 0x04, 0x04],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04],
        _ => return None,
    };
    Some((rows, accent))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;
    use crate::eggs::halves;

    #[test]
    fn the_stage_covers_the_screen_and_sits_on_the_floor() {
        for (cols, rows) in [(1, 1), (20, 8), (80, 24), (100, 30), (200, 60), (500, 150)] {
            let s = Stage::new(cols, rows);
            assert!(s.k >= 1);
            assert!(s.w * s.k >= i32::from(cols), "{cols}x{rows}");
            assert!(s.h * s.k >= i32::from(rows) * 2, "{cols}x{rows}");
            assert!(s.top <= 0 && s.top > -s.k, "{cols}x{rows}");
        }
        // Big screens get bigger pixels, narrow ones do not.
        assert_eq!(Stage::new(80, 24).k, 1);
        assert_eq!(Stage::new(200, 60).k, 2);
        assert_eq!(Stage::new(40, 150).k, 1);
    }

    /// The bottom pixel row of the stage is the bottom half of the last cell
    /// row, however big the pixels are.
    #[test]
    fn rects_and_sprites_scale_from_the_floor() {
        let red = rgb(0xff0000);
        for (cols, rows) in [(10, 4), (200, 60), (300, 91)] {
            let s = Stage::new(cols, rows);
            let mut c = Canvas::new(cols, rows, Color::Reset);
            s.px(&mut c, 3, s.h - 1, red);
            let low = c.get(3 * s.k, i32::from(rows) - 1).unwrap();
            assert_eq!(halves(low).1, red, "{cols}x{rows}");
            c.clear();
            s.sprite(
                &mut c,
                (2, s.h - 2),
                &["RR", "RR"],
                false,
                |_| 0,
                |_| Some(red),
            );
            let low = c.get(2 * s.k, i32::from(rows) - 1).unwrap();
            assert_eq!(halves(low), (red, red), "{cols}x{rows}");
            assert_eq!(c.get(2 * s.k - 1, i32::from(rows) - 1).unwrap().ch, ' ');
            // Off the screen is no trouble.
            s.rect(&mut c, -5, -5, 3, 3, red);
            s.rect(&mut c, s.w, s.h, 9, 9, red);
            s.line(&mut c, (-9, -9), (s.w + 9, s.h + 9), 2, red);
            s.glyph(&mut c, (-3, -3), 'É', 3, red, Some(red));
        }
    }

    #[test]
    fn the_font_has_what_the_calls_need() {
        for ch in "¡VIVA MÉXICO! ¡VIVA LA REVOLUCIÓN! TIERRA Y LIBERTAD 1910".chars() {
            assert!(ch == ' ' || glyph(ch).is_some(), "{ch}");
        }
        assert_eq!(text_width("A A", 1), 6 + 3 + 5);
        assert_eq!(text_width("", 3), 0);
    }
}
