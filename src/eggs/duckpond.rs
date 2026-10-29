//! The duck from duckpond.sh: a yellow duck paddles across a line of water.
//!
//! Ported from <https://github.com/gsobell/duckpond.sh> by Gavi Sobell (2023),
//! licensed GPL-3.0 (oxiper is GPL-3.0-only, which is compatible).
//!
//! The script draws the three lines of the duck in yellow and a full-width
//! line of `~` in cyan, and moves the duck one column to the right every
//! 0.1 s until it leaves the screen. As in the README, the duck sits in the
//! water; its "quack!" floats up beside it now and then.

use super::{Egg, Info};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{Canvas, Cell};
use crate::theme::Theme;

pub const INFO: Info = Info {
    name: "duckpond",
    start,
    fiesta: Kind::None,
    when: super::any_day,
};

fn start(w: u16, _h: u16, _rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Duck { t: 0.0, cols: w })
}

/// The script waits 0.1 s per column.
const COLS_PER_SEC: f32 = 10.0;
const DUCK: [&str; 3] = ["  _", "_(.)-", "\\__)"];
/// Seconds from the start of one quack to the next, and how long one shows.
const QUACK_EVERY: f32 = 3.5;
const QUACK_FOR: f32 = 1.4;

struct Duck {
    t: f32,
    cols: u16,
}

impl Duck {
    fn x(&self) -> i32 {
        (self.t * COLS_PER_SEC) as i32
    }
}

impl Egg for Duck {
    fn update(&mut self, dt: f32) {
        self.t += dt;
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let yellow = theme.pick(3, Color::Ansi(3));
        let cyan = theme.pick(0, Color::Ansi(6));
        let (x, bg) = (self.x(), c.bg());
        // The water on the second to last row; the duck sits in it.
        let water = c.h() as i32 - 2;
        let cell = |ch, fg| Cell {
            ch,
            fg,
            bg,
            attr: 0,
        };
        for cx in 0..c.w() as i32 {
            c.set_cell(cx, water, cell('~', cyan));
        }
        for (i, line) in DUCK.iter().enumerate() {
            for (dx, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    c.set_cell(x + dx as i32, water - 2 + i as i32, cell(ch, yellow));
                }
            }
        }
        // "quack!" floats up and away from the duck's head, now and then.
        let q = (self.t - 1.0).rem_euclid(QUACK_EVERY);
        if self.t >= 1.0 && q < QUACK_FOR {
            let rise = (q / QUACK_FOR * 2.0) as i32;
            c.text(x + 7, water - 2 - rise, "quack!", theme.fg);
        }
    }

    fn done(&self) -> bool {
        self.x() >= self.cols as i32
    }
}
