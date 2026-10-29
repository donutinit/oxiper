//! La Catrina strolls across the bottom of the screen during Día de Muertos.
//!
//! Original oxiper pixel art, after José Guadalupe Posada's Calavera
//! Garbancera: a sugar skull with turquoise brows and a flower on the
//! forehead, a wide morado hat with rosa and white plumes and marigolds on
//! its crown, a terciopelo gown embroidered with cempasúchil, one hand on her
//! hip and a bouquet of marigolds in the other. She walks at a stroll, her
//! hem swinging with each step and her plumes stirring, and the bouquet
//! drops a petal now and then, which tumbles down and rests on the ground a
//! while behind her. She comes from either side and is gone within 30 s.

use super::muertos::{Palette, fade};
use super::{Egg, Info, pixel, sprite};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "catrina",
    start,
    fiesta: Kind::Muertos,
    when: super::any_day,
};

/// Facing right. Rows 0 to 7 are the plumes, 34 on the skirt.
const CATRINA: &[&str] = &[
    "......r.r.r...................",
    "....r.rRrRrr..................",
    "...rRRRRRRRRr.....W.W.........",
    "..rRRrr...rRRr...WWWWW........",
    ".rRRr.......RRr.WWwwWWw.......",
    ".rRr.........RrWWw...wW.......",
    "r.Rr.......rRRWWw.....w.......",
    "..r........rRWWwMMMMP.........",
    "...........RWWMMMMMPPMMoOo....",
    "..........WWwMMMMMMPMMOYYOr...",
    "...........MMMMMMMMMMMMoOORRr.",
    "...........QQYQQQRQQQYQQQQRr..",
    "....MMMMPPPPPPPPPPPPPPPPPPPPMM",
    "..mMMMMMMMMMMMMMMMMMMMMMMMMMMMm",
    "...mmmmmmmmmmmmmmmmmmmmmmmmmmm.",
    ".......mmmm.wWWWWWWWw.mmmmm...",
    "...........WWWWWYWWWWW........",
    "...........WWWWRYRWWWW........",
    "...........QQQWWYWWQQQ........",
    "..........QkkkQWWWQkkkQ.......",
    "..........WkkkkWWWkkkkW.......",
    "...........WkkWWkWWkkW........",
    "...........wWWWkkkWWWw........",
    "...........wWkWkWkWkWw........",
    "............wWkWkWkWw.........",
    ".............wwwwwww..........",
    "...............WWW............",
    "..........RRRWWWWWWWRRR.......",
    ".........RrRTRWOOWRTRrRR......",
    ".........RRRTTOYYOTTTRRWW.....",
    "........WW..TTTOOTTTT..WW.oOo.",
    "........W...TTTTTTTTT...WWOYO.",
    ".........WW..TTTTTTT.....gOo..",
    "...........WWOOOOOOO.....g....",
    "............TTTTTTTTT.........",
    "...........TTtTTTTTtTT........",
    "..........TTtTTTOTTTTtTT......",
    "..........TtTTTOYOTTTTtTT.....",
    ".........TTtTTTTOTTTTTtTTT....",
    ".........TtTTTTTTTTTTTTtTTT...",
    "........TTtTTOTTTTTTOTTTtTTT..",
    "........TtTTOYOTTTTOYOTTTtTT..",
    ".......TTtTTTOTTTTTTOTTTTtTTT.",
    ".......RRRRRRRRRRRRRRRRRRRRRRR",
    ".......rRrRrRrRrRrRrRrRrRrRrRr",
];
const WIDTH: i32 = 31;
/// Her height in pixels, with the row her shoes show in.
const HEIGHT: i32 = CATRINA.len() as i32 + 1;
const PLUMES: i32 = 8;
const SKIRT: i32 = 34;
/// Where the bouquet is, facing right.
const BOUQUET: (i32, i32) = (27, 30);

/// Columns per second at a stroll; faster on wide screens so she crosses in time.
const STROLL: f32 = 5.0;
const CROSSING: f32 = 26.0;
/// Pixels walked per stride (two steps).
const STRIDE: f32 = 16.0;

/// Petals the bouquet drops, how often, and how long one rests on the ground.
const PETALS: usize = 24;
const DROP_EVERY: f32 = 0.45;
const REST: f32 = 2.5;

fn start(w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Catrina {
        t: 0.0,
        dist: 0.0,
        speed: STROLL.max((w as f32 + WIDTH as f32 + 2.0) / CROSSING),
        left: rng.bool(),
        w: w as i32,
        petals: [Petal::default(); PETALS],
        next: 0,
        drop_in: DROP_EVERY,
        rng: fastrand::Rng::with_seed(rng.u64(..)),
    })
}

#[derive(Clone, Copy, Default)]
struct Petal {
    /// Pixels from the left edge and up from the ground.
    x: f32,
    up: f32,
    vx: f32,
    /// Seconds since it fell from the bouquet; 0 means none.
    age: f32,
    /// Seconds resting on the ground.
    rest: f32,
    spin: f32,
}

struct Catrina {
    t: f32,
    /// Columns walked.
    dist: f32,
    speed: f32,
    /// Walking right to left (mirrored)?
    left: bool,
    w: i32,
    petals: [Petal; PETALS],
    next: usize,
    drop_in: f32,
    rng: fastrand::Rng,
}

impl Catrina {
    /// Her left column.
    fn x(&self) -> i32 {
        let d = self.dist as i32;
        if self.left { self.w - d } else { d - WIDTH }
    }

    /// Where she is in her stride, in radians.
    fn phase(&self) -> f32 {
        self.dist / STRIDE * TAU
    }

    /// The bouquet's column, and its height above the ground in pixels.
    fn bouquet(&self) -> (f32, f32) {
        let bx = if self.left {
            WIDTH - 1 - BOUQUET.0
        } else {
            BOUQUET.0
        };
        (
            (self.x() + bx) as f32 + 0.5,
            (HEIGHT - 1 - BOUQUET.1) as f32,
        )
    }
}

impl Egg for Catrina {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
        for p in self.petals.iter_mut().filter(|p| p.age > 0.0) {
            p.age += dt;
            if p.up > 0.0 {
                // Tumbling down, swaying on the way.
                p.up = (p.up - 7.0 * dt).max(0.0);
                p.x += (p.vx + (p.age * 5.0 + p.spin).sin() * 2.5) * dt;
            } else {
                p.rest += dt;
                if p.rest > REST {
                    p.age = 0.0;
                }
            }
        }
        self.drop_in -= dt;
        while self.drop_in <= 0.0 {
            self.drop_in += DROP_EVERY * (0.6 + self.rng.f32());
            let (x, up) = self.bouquet();
            if x < 0.0 || x >= self.w as f32 {
                continue;
            }
            let back = if self.left { 1.0 } else { -1.0 };
            self.petals[self.next] = Petal {
                x,
                up,
                vx: back * (0.5 + self.rng.f32() * 1.5),
                age: 1e-3,
                rest: 0.0,
                spin: self.rng.f32() * TAU,
            };
            self.next = (self.next + 1) % PETALS;
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::new(theme);
        let ground = c.h() as i32 * 2 - 1;
        let phase = self.phase();
        // A little lift at each step, the hem swinging, the plumes stirring.
        let lift = (phase * 2.0).sin().max(0.0).round() as i32;
        let swing = (phase.sin() * 1.6).round() as i32;
        let stir = (self.t * 2.3).sin() * 1.2;
        let back = if self.left { -1 } else { 1 };
        let (x, y) = (self.x(), ground + 1 - HEIGHT - lift);
        let shift = |r: i32| {
            if r < PLUMES {
                (stir * (PLUMES - r) as f32 / PLUMES as f32).round() as i32 * -back
            } else if r >= SKIRT {
                -swing * back * (r - SKIRT + 1) / (CATRINA.len() as i32 - SKIRT)
            } else {
                0
            }
        };
        // Her shoes, stepping out from under the hem by turns.
        let shoe = pal.of(b't');
        for (k, sgn) in [(0, 1.0f32), (1, -1.0)] {
            let step = (phase.sin() * sgn * 3.0).round() as i32;
            let fx = 17 + 4 * k + step;
            let fx = if self.left { WIDTH - 2 - fx } else { fx };
            for dx in 0..2 {
                pixel(c, x + fx + dx, ground, shoe);
            }
        }
        sprite(c, (x, y), CATRINA, self.left, shift, |ch| pal.get(ch));
        for p in self.petals.iter().filter(|p| p.age > 0.0) {
            let tumble = (p.age * 6.0 + p.spin) as i32 % 2 == 0;
            let col = pal.of(if tumble { b'O' } else { b'o' });
            let col = fade(theme, col, (p.rest / REST).powi(2));
            pixel(c, p.x.floor() as i32, ground - p.up.round() as i32, col);
        }
    }

    fn done(&self) -> bool {
        // Gone once she has left and her last petals have faded.
        let gone = if self.left {
            self.x() + WIDTH < 0
        } else {
            self.x() > self.w
        };
        gone && self.petals.iter().all(|p| p.age == 0.0) || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        assert!(CATRINA.iter().all(|r| r.len() as i32 <= WIDTH));
        assert!(CATRINA.iter().any(|r| r.len() as i32 == WIDTH));
        let at = CATRINA[BOUQUET.1 as usize].as_bytes()[BOUQUET.0 as usize];
        assert_eq!(at, b'O', "the bouquet");
    }

    /// She walks from one side to the other, standing on the bottom row.
    #[test]
    fn strolls_across_on_the_ground() {
        let theme = Theme::classic();
        for seed in 0..4 {
            let mut e = start(100, 30, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(100, 30, Color::Reset);
            let mut cols = Vec::new();
            for _ in 0..40 {
                e.update(0.5);
                c.clear();
                e.draw(&mut c, &theme);
                let drawn: Vec<usize> = (0..100)
                    .filter(|&x| (0..30).any(|y| c.get(x, y).unwrap().ch != ' '))
                    .map(|x| x as usize)
                    .collect();
                if drawn.len() > 10 {
                    cols.push(drawn.iter().sum::<usize>() / drawn.len());
                }
                // Nothing above her hat.
                assert!(
                    c.to_text()
                        .lines()
                        .take(30 - 24)
                        .all(|l| l.trim().is_empty())
                );
            }
            assert!(cols.len() > 10);
            let moved = *cols.last().unwrap() as i32 - cols[0] as i32;
            assert!(moved.abs() > 40, "{cols:?}");
        }
    }
}
