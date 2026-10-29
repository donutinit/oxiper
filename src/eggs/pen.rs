//! Small pixel-art helpers the holiday visitors share: lines, rectangles and
//! a tiny font.
//!
//! Like the rest of the visitors' art it all works in half-block pixels (see
//! [`pixel`](super::pixel)): `x` counts columns and `y` half rows, and
//! anything off the screen is simply not drawn.

use super::pixel;
use crate::color::Color;
use crate::render::Canvas;

/// Calls `f` with every pixel of the line from `a` to `b`, ends included
/// (Bresenham), so a caller can mirror or tint what it draws.
pub(crate) fn trace((mut x, mut y): (i32, i32), (x1, y1): (i32, i32), mut f: impl FnMut(i32, i32)) {
    let (dx, dy) = ((x1 - x).abs(), -(y1 - y).abs());
    let (sx, sy) = (if x < x1 { 1 } else { -1 }, if y < y1 { 1 } else { -1 });
    let mut err = dx + dy;
    loop {
        f(x, y);
        if x == x1 && y == y1 {
            return;
        }
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

/// The 3x5 font's letters and digits, five rows of three bits each.
const FONT: &[(char, [u8; 5])] = &[
    ('A', [0b010, 0b101, 0b111, 0b101, 0b101]),
    ('B', [0b110, 0b101, 0b110, 0b101, 0b110]),
    ('C', [0b011, 0b100, 0b100, 0b100, 0b011]),
    ('D', [0b110, 0b101, 0b101, 0b101, 0b110]),
    ('E', [0b111, 0b100, 0b110, 0b100, 0b111]),
    ('F', [0b111, 0b100, 0b110, 0b100, 0b100]),
    ('G', [0b011, 0b100, 0b101, 0b101, 0b011]),
    ('H', [0b101, 0b101, 0b111, 0b101, 0b101]),
    ('I', [0b111, 0b010, 0b010, 0b010, 0b111]),
    ('J', [0b001, 0b001, 0b001, 0b101, 0b010]),
    ('K', [0b101, 0b101, 0b110, 0b101, 0b101]),
    ('L', [0b100, 0b100, 0b100, 0b100, 0b111]),
    ('M', [0b101, 0b111, 0b111, 0b101, 0b101]),
    ('N', [0b110, 0b101, 0b101, 0b101, 0b101]),
    ('O', [0b010, 0b101, 0b101, 0b101, 0b010]),
    ('P', [0b110, 0b101, 0b110, 0b100, 0b100]),
    ('Q', [0b010, 0b101, 0b101, 0b110, 0b011]),
    ('R', [0b110, 0b101, 0b110, 0b101, 0b101]),
    ('S', [0b011, 0b100, 0b010, 0b001, 0b110]),
    ('T', [0b111, 0b010, 0b010, 0b010, 0b010]),
    ('U', [0b101, 0b101, 0b101, 0b101, 0b111]),
    ('V', [0b101, 0b101, 0b101, 0b101, 0b010]),
    ('W', [0b101, 0b101, 0b111, 0b111, 0b101]),
    ('X', [0b101, 0b101, 0b010, 0b101, 0b101]),
    ('Y', [0b101, 0b101, 0b010, 0b010, 0b010]),
    ('Z', [0b111, 0b001, 0b010, 0b100, 0b111]),
    ('0', [0b111, 0b101, 0b101, 0b101, 0b111]),
    ('1', [0b010, 0b110, 0b010, 0b010, 0b111]),
    ('2', [0b110, 0b001, 0b010, 0b100, 0b111]),
    ('3', [0b110, 0b001, 0b010, 0b001, 0b110]),
    ('4', [0b101, 0b101, 0b111, 0b001, 0b001]),
    ('5', [0b111, 0b100, 0b110, 0b001, 0b110]),
    ('6', [0b011, 0b100, 0b111, 0b101, 0b111]),
    ('7', [0b111, 0b001, 0b010, 0b010, 0b010]),
    ('8', [0b111, 0b101, 0b111, 0b101, 0b111]),
    ('9', [0b111, 0b101, 0b111, 0b001, 0b110]),
    ('!', [0b010, 0b010, 0b010, 0b000, 0b010]),
    ('¡', [0b010, 0b000, 0b010, 0b010, 0b010]),
];

/// The rows of a letter of the font (the accented vowels are their plain
/// letters; [`text`] adds the accent) or `None` for a character it lacks.
fn glyph(ch: char) -> Option<[u8; 5]> {
    let plain = match ch {
        'Á' => 'A',
        'É' => 'E',
        'Í' => 'I',
        'Ó' => 'O',
        'Ú' => 'U',
        ch => ch.to_ascii_uppercase(),
    };
    FONT.iter()
        .find(|(c, _)| *c == plain)
        .map(|&(_, rows)| rows)
}

/// Draws `s` in the 3x5 font with its top-left pixel at (`x`, `y`), every
/// font pixel `k` pixels across; unknown characters and spaces are gaps.
pub(crate) fn text(c: &mut Canvas, (x, y): (i32, i32), s: &str, k: i32, color: Color) {
    for (i, ch) in s.chars().enumerate() {
        let Some(rows) = glyph(ch) else {
            continue;
        };
        let gx = x + i as i32 * 4 * k;
        for (r, bits) in rows.iter().enumerate() {
            for b in 0..3 {
                if bits >> (2 - b) & 1 == 1 {
                    fill(c, (gx + b * k, y + r as i32 * k), (k, k), color);
                }
            }
        }
        if "ÁÉÍÓÚ".contains(ch) {
            fill(c, (gx + 2 * k, y - 2 * k), (k, k), color);
        }
    }
}

/// Fills a rectangle of pixels with a color (nothing if it is empty).
pub(crate) fn fill(c: &mut Canvas, (x, y): (i32, i32), (w, h): (i32, i32), color: Color) {
    // Only the part on the screen is walked, however big the rectangle.
    let (x0, x1) = (x.max(0), (x + w).min(c.w() as i32));
    let (y0, y1) = (y.max(0), (y + h).min(c.h() as i32 * 2));
    for py in y0..y1 {
        for px in x0..x1 {
            pixel(c, px, py, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;

    #[test]
    fn lines_run_from_end_to_end() {
        for (a, b) in [((0, 0), (5, 2)), ((3, 3), (3, -4)), ((-2, 1), (-2, 1))] {
            let mut pts = Vec::new();
            trace(a, b, |x, y| pts.push((x, y)));
            assert_eq!((pts[0], *pts.last().unwrap()), (a, b));
            let steps = (b.0 - a.0).abs().max((b.1 - a.1).abs()) as usize;
            assert_eq!(pts.len(), steps + 1);
        }
    }

    #[test]
    fn the_font_draws_what_it_says() {
        let red = rgb(0xff0000);
        let mut c = Canvas::new(20, 4, Color::Reset);
        text(&mut c, (0, 0), "A", 1, red);
        let lit = |c: &Canvas, x: i32, y: i32| {
            let cell = c.get(x, y / 2).unwrap();
            let (up, down) = super::super::halves(cell);
            (if y % 2 == 0 { up } else { down }) == red
        };
        // The A: a point on top, then two legs.
        assert!(lit(&c, 1, 0) && !lit(&c, 0, 0) && !lit(&c, 2, 0));
        assert!(lit(&c, 0, 4) && lit(&c, 2, 4) && !lit(&c, 1, 4));
        // Spaces and strangers are gaps, and nothing goes wrong off screen.
        text(&mut c, (-30, -9), "¡AÑO Ó!", 3, red);
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!¡ÁÉÍÓÚ".chars() {
            assert!(glyph(ch).is_some(), "{ch}");
        }
    }
}
