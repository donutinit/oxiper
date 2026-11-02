//! What rain's holidays share: figures drawn in characters with an ink map,
//! a cord strung across the screen, and the light a glowing thing throws on
//! the drops falling around it.

use crate::color::Color;
use crate::render::Canvas;

/// A figure in characters, with a parallel ink map: a space is the default
/// ink, a letter one of the holiday's colors (see [`Sprite::draw`]). The art
/// faces right.
pub(super) struct Sprite {
    pub art: &'static [&'static str],
    pub ink: &'static [&'static str],
    pub w: i32,
}

impl Sprite {
    /// Draws the figure with its left edge at `x` and its bottom row at
    /// `bottom`, mirrored when `flip` (then `x` is still its left edge).
    /// `letters` names the inks of the map and `inks` their colors: the
    /// default first, then one per letter. `only` limits it to the cells in
    /// those inks, for what stays alight after the rest has faded.
    pub fn draw(
        &self,
        c: &mut Canvas,
        (x, bottom): (i32, i32),
        flip: bool,
        (letters, inks): (&str, &[Color]),
        only: Option<&str>,
    ) {
        let top = bottom + 1 - self.art.len() as i32;
        for (row, line) in self.art.iter().enumerate() {
            let mut map = self.ink.get(row).copied().unwrap_or("").chars();
            for (i, ch) in line.chars().enumerate() {
                let ink = map.next().unwrap_or(' ');
                if ch == ' ' || only.is_some_and(|o| !o.contains(ink)) {
                    continue;
                }
                let k = letters.find(ink).map_or(0, |k| k + 1);
                let (cx, ch) = if flip {
                    (x + self.w - 1 - i as i32, mirror(ch))
                } else {
                    (x + i as i32, ch)
                };
                c.set(cx, top + row as i32, ch, inks[k.min(inks.len() - 1)]);
            }
        }
    }
}

/// A cell of a cord strung across the screen.
#[derive(Clone, Copy)]
pub(super) struct Cord {
    pub x: i16,
    pub y: i16,
    pub ch: char,
}

/// Strings a cord across `w` columns at row `y0`, sagging `depth` rows
/// between ties `swag` columns apart (the first one `shift` columns along),
/// and pushes a cell for each column.
pub(super) fn hang(cords: &mut Vec<Cord>, w: i32, y0: i32, (depth, swag, shift): (f32, i32, i32)) {
    let y = |x: i32| {
        let u = ((x + shift) % swag) as f32 / swag as f32;
        y0 + (depth * 4.0 * u * (1.0 - u)).round() as i32
    };
    for x in 0..w {
        let (here, next) = (y(x), y((x + 1).min(w - 1)));
        let ch = match next.cmp(&here) {
            std::cmp::Ordering::Greater => '╲',
            std::cmp::Ordering::Less => '╱',
            std::cmp::Ordering::Equal => '─',
        };
        cords.push(Cord {
            x: x as i16,
            y: here as i16,
            ch,
        });
    }
}

/// The character seen in a mirror.
pub(super) fn mirror(c: char) -> char {
    match c {
        '/' => '\\',
        '\\' => '/',
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '‹' => '›',
        '›' => '‹',
        '`' => '´',
        '´' => '`',
        '╱' => '╲',
        '╲' => '╱',
        '▌' => '▐',
        '▐' => '▌',
        '◖' => '◗',
        '◗' => '◖',
        c => c,
    }
}

/// The light of glowing things on the drops around them: for every cell the
/// strongest light that reaches it, as a strength and the index of the thing
/// it comes from.
pub(super) struct Glow {
    w: i32,
    cells: Vec<(u8, u8)>,
}

impl Glow {
    pub const fn new() -> Glow {
        Glow {
            w: 0,
            cells: Vec::new(),
        }
    }

    /// Clears it for a screen of `w` by `h` cells (nothing lights it up at
    /// first).
    pub fn reset(&mut self, w: i32, h: i32) {
        self.w = w;
        self.cells.clear();
        self.cells.resize((w.max(0) * h.max(0)) as usize, (0, 0));
    }

    /// Lights an ellipse `rx` columns and `ry` rows out from (`cx`, `cy`),
    /// brightest in the middle, for the thing `i`.
    pub fn add(&mut self, i: u8, (cx, cy): (i32, i32), (rx, ry): (f32, f32)) {
        if self.w <= 0 {
            return;
        }
        let h = (self.cells.len() / self.w as usize) as i32;
        for y in (cy - ry as i32).max(0)..=(cy + ry as i32).min(h - 1) {
            for x in (cx - rx as i32).max(0)..=(cx + rx as i32).min(self.w - 1) {
                let dx = (x - cx) as f32 / rx;
                let dy = (y - cy) as f32 / ry;
                let d = (dx * dx + dy * dy).sqrt();
                if d >= 1.0 {
                    continue;
                }
                let s = ((1.0 - d).powf(1.2) * 255.0) as u8;
                let slot = &mut self.cells[(y * self.w + x) as usize];
                if s > slot.0 {
                    *slot = (s, i);
                }
            }
        }
    }

    /// The light on a cell: its strength from 0 to 1 and what it comes from.
    #[inline]
    pub fn at(&self, x: i32, y: i32) -> Option<(f32, usize)> {
        if x < 0 || y < 0 || x >= self.w {
            return None;
        }
        match self.cells.get((y * self.w + x) as usize) {
            Some(&(s, i)) if s > 0 => Some((s as f32 / 255.0, i as usize)),
            _ => None,
        }
    }
}
