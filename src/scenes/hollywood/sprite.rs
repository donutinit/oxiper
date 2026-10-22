//! Half-block pixel sprites for the holiday widgets (the altar, the sugar
//! skull): two pixels to a cell, drawn into a pane.

use super::Rect;
use crate::color::Color;
use crate::render::{Canvas, Cell};

/// A grid of pixels; `Color::Reset` is see-through.
pub struct Sprite {
    pub w: usize,
    pub h: usize,
    px: Vec<Color>,
}

impl Sprite {
    pub fn new(w: usize, h: usize) -> Sprite {
        Sprite {
            w,
            h: h + h % 2,
            px: vec![Color::Reset; w * (h + h % 2)],
        }
    }

    pub fn clear(&mut self) {
        self.px.fill(Color::Reset);
    }

    /// A filled rectangle (clipped).
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, col: Color) {
        for yy in y.max(0)..(y + h).min(self.h as i32) {
            for xx in x.max(0)..(x + w).min(self.w as i32) {
                self.px[yy as usize * self.w + xx as usize] = col;
            }
        }
    }

    pub fn dot(&mut self, x: i32, y: i32, col: Color) {
        self.rect(x, y, 1, 1, col);
    }

    pub fn is_set(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y >= 0
            && (x as usize) < self.w
            && (y as usize) < self.h
            && self.px[y as usize * self.w + x as usize] != Color::Reset
    }

    /// Lets `f` change every pixel that is set, given its row.
    pub fn recolor(&mut self, mut f: impl FnMut(i32, Color) -> Color) {
        for (i, px) in self.px.iter_mut().enumerate() {
            if *px != Color::Reset {
                *px = f((i / self.w) as i32, *px);
            }
        }
    }

    /// Cells it takes: columns and rows.
    pub fn cells(&self) -> (i32, i32) {
        (self.w as i32, self.h as i32 / 2)
    }

    /// Composites it onto the canvas with its top left cell at (`x0`, `y0`),
    /// in the same way as `render::Pixels`, keeping to the pane `r`: a cell
    /// with one empty half only gets a foreground, so the empty half stays
    /// transparent.
    pub fn blit(&self, c: &mut Canvas, r: Rect, x0: i32, y0: i32) {
        let bg = c.bg();
        let (cols, rows) = self.cells();
        for row in 0..rows {
            let y = y0 + row;
            if y < r.y || y >= r.y + r.h {
                continue;
            }
            for col in 0..cols {
                let x = x0 + col;
                if x < r.x || x >= r.x + r.w {
                    continue;
                }
                let top = self.px[(row as usize * 2) * self.w + col as usize];
                let bot = self.px[(row as usize * 2 + 1) * self.w + col as usize];
                let (ch, fg, cell_bg) = match (top, bot) {
                    (Color::Reset, Color::Reset) => continue,
                    (t, Color::Reset) => ('\u{2580}', t, bg),
                    (Color::Reset, b) => ('\u{2584}', b, bg),
                    (t, b) if t == b => ('\u{2588}', t, bg),
                    (t, b) if c.strict => ('\u{2588}', t.lerp(b, 0.5), bg),
                    (t, b) => ('\u{2580}', t, b),
                };
                c.set_cell(
                    x,
                    y,
                    Cell {
                        ch,
                        fg,
                        bg: cell_bg,
                        attr: 0,
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;

    #[test]
    fn blits_two_pixels_to_a_cell_and_stays_in_its_pane() {
        let mut s = Sprite::new(6, 5);
        assert_eq!(s.cells(), (6, 3));
        s.rect(0, 0, 6, 6, rgb(0xff0000));
        s.dot(1, 3, rgb(0x00ff00));
        s.dot(2, 2, Color::Reset);
        let mut c = Canvas::new(20, 10, Color::Reset);
        let pane = Rect {
            x: 4,
            y: 2,
            w: 3,
            h: 2,
        };
        s.blit(&mut c, pane, 3, 1);
        for y in 0..10 {
            for x in 0..20 {
                let cell = c.get(x, y).unwrap();
                let inside = (4..7).contains(&x) && (2..4).contains(&y);
                assert_eq!(cell.ch != ' ', inside, "{x},{y}");
            }
        }
        // Two different pixels: the top as foreground, the bottom as background.
        let cell = c.get(4, 2).unwrap();
        assert_eq!(
            (cell.ch, cell.fg, cell.bg),
            ('\u{2580}', rgb(0xff0000), rgb(0x00ff00))
        );
        // An empty half stays transparent.
        let cell = c.get(5, 2).unwrap();
        assert_eq!((cell.ch, cell.fg), ('\u{2584}', rgb(0xff0000)));
        // Strict transparency never paints a background.
        let mut strict = Canvas::new(20, 10, Color::Reset);
        strict.strict = true;
        s.blit(&mut strict, pane, 3, 1);
        assert!(strict.cells().iter().all(|c| c.bg == Color::Reset));
    }
}
