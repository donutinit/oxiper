//! Nyan Cat flies across whatever is playing, its rainbow trailing behind.
//!
//! The frames, the palette and the rainbow are the nyancat scene's, from
//! K. Lange's nyancat (NCSA license; the notice is kept in
//! `scenes/nyancat/frames.rs`). One cat pixel is one half-block pixel, the
//! frames advance every 90 ms like nyancat's, and the cat crosses from left to
//! right at a height picked at random. Only the cat and its rainbow are
//! painted: the sky is whatever the scene drew, and a cell the cat covers by
//! half keeps the scene's other half.

use super::{Egg, Info, paint};
use crate::color::{self, Color};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::scenes::nyancat::frames::{FRAME_HEIGHT, FRAME_WIDTH, FRAMES};
use crate::scenes::nyancat::{PALETTE, RAINBOW_CHARS, sample, strip_stars, theme_rainbow};
use crate::theme::Theme;

pub const INFO: Info = Info {
    name: "nyancat",
    start,
    fiesta: Kind::None,
};

/// nyancat's 90 ms per frame.
const DELAY: f32 = 0.09;
/// Seconds the cat takes to cross, whatever the width.
const CROSSING: f32 = 7.0;

type Stripped = [[u8; FRAME_WIDTH]; FRAME_HEIGHT];

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let frames: Vec<Stripped> = FRAMES.iter().map(strip_stars).collect();
    // The rows and the right edge that hold cat or rainbow in any frame.
    let (mut top, mut bot, mut right) = (FRAME_HEIGHT as i32, 0, 0);
    for f in &frames {
        for (y, row) in f.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch != b',' {
                    top = top.min(y as i32);
                    bot = bot.max(y as i32);
                    right = right.max(x as i32);
                }
            }
        }
    }
    // Somewhere in the upper two thirds, where there is sky in most scenes.
    let room = (h as i32 * 4 / 3 - (bot - top + 1)).max(0);
    let tail = (w as i32 / 2).max(40);
    Box::new(Cat {
        frames,
        t: 0.0,
        y0: rng.i32(0..=room) - top,
        top,
        bot,
        right,
        tail,
        speed: (w as f32 + (right + tail) as f32) / CROSSING,
        w: w as i32,
    })
}

struct Cat {
    frames: Vec<Stripped>,
    t: f32,
    /// Pixel row of the frame's first row.
    y0: i32,
    /// Frame rows and last column with cat or rainbow in them.
    top: i32,
    bot: i32,
    right: i32,
    /// How far the rainbow reaches behind the frame, in pixels.
    tail: i32,
    /// Pixels per second.
    speed: f32,
    w: i32,
}

impl Cat {
    /// Pixel column of the frame's first column: it starts just off the left edge.
    fn ox(&self) -> i32 {
        (self.t * self.speed) as i32 - self.right
    }
}

impl Egg for Cat {
    fn update(&mut self, dt: f32) {
        self.t += dt;
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let mut pal = [Color::Reset; 128];
        for &(ch, hex, _) in PALETTE {
            pal[ch as usize] = color::rgb(hex);
        }
        if !theme.native {
            for (ch, col) in RAINBOW_CHARS.iter().zip(theme_rainbow(theme)) {
                pal[*ch as usize] = col;
            }
        }
        let i = (self.t / DELAY) as usize % self.frames.len();
        let (f, flip) = (&self.frames[i], (i / 2) % 2 == 1);
        let ox = self.ox();
        let pixel = |px: i32, py: i32| -> Option<Color> {
            let (fx, fy) = (px - ox, py - self.y0);
            if fy < self.top || fy > self.bot || fx < -self.tail {
                return None;
            }
            match sample(f, flip, fx, fy) {
                b',' => None,
                ch => Some(pal[ch as usize & 127]),
            }
        };
        let rows = (self.y0 + self.top).div_euclid(2)..=(self.y0 + self.bot).div_euclid(2);
        for row in rows {
            for col in 0..self.w.min(c.w() as i32) {
                paint(c, col, row, pixel(col, row * 2), pixel(col, row * 2 + 1));
            }
        }
    }

    fn done(&self) -> bool {
        self.ox() - self.tail >= self.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cat shows up on the way across, then leaves.
    #[test]
    fn crosses_the_screen() {
        let theme = Theme::classic();
        let mut rng = fastrand::Rng::with_seed(1);
        let mut e = start(80, 24, &mut rng);
        let mut c = Canvas::new(80, 24, Color::Reset);
        let mut seen = 0;
        while !e.done() {
            e.update(0.5);
            c.clear();
            e.draw(&mut c, &theme);
            seen = seen.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
        }
        assert!(seen > 100, "{seen} cells");
    }
}
