//! Chinelos of Morelos hop across the bottom of the screen during Carnaval.
//!
//! The chinelos are the masked dancers of Tepoztlán, Yautepec and Tlayacapan,
//! who take to the streets at Carnaval. Original oxiper pixel art: each one
//! in a long velvet robe with a short embroidered cape, flowers and rows of
//! gold sequins on it, a wide hat trimmed in sequins with a fan of ostrich
//! plumes, and the pale mask with its blue eyes, rosy cheeks, curled
//! mustache and black beard turned up at the tips. They go in a crowd, each
//! in its own color, and each does the brinco, the hop of the dance: it
//! springs off the ground and lands with a dip of the knees, the arms in
//! their wide sleeves and white gloves swinging up and down in turns, the
//! robe swaying from side to side, the plumes trailing behind the jump and
//! the sequins flashing. The crowd grows and the dancers scale with the
//! screen, and it crosses from either side within 30 s.

use super::blit::{Xf, blit, fit, hash, line, plume};
use super::carnaval::Palette;
use super::{Egg, Info, pixel};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::PI;

pub const INFO: Info = Info {
    name: "chinelos",
    start,
    fiesta: Kind::Carnaval,
    when: super::any_day,
};

/// Facing the front. `A` is the color of the robe, `a` its shadow and `E`
/// the color of the cape and the embroidery; each chinelo has its own.
const CHINELO: &[&str] = &[
    "..........AAAAA..........",
    ".........AAaAAAA.........",
    "........AALAAALAA........",
    "........LELELELEL........",
    "...AAAAAAAAAAAAAAAAAAA...",
    "...aEaEaEaEaEaEaEaEaEa...",
    "....aaaaaaaaaaaaaaaaa....",
    "....aAAAwWWWWWWWwAAAa....",
    "....AAALWkkWWWkkWLAAA....",
    "....aAALWBBWWWBBWLAAa....",
    "....AAaLWWWWWWWWWLaAA....",
    "....aAALWrrWwWrrWLAAa....",
    "....AAALWkWWWWWkWLAAA....",
    "....aAALWWkkUkkWWLAAa....",
    "....AAaLWWWWWWWWWLaAA....",
    "....aAALwWWkkkWWwLAAa....",
    "....LEEEEEkkkkkEEEEEL....",
    "....LEEEEEkkkkkEEEEEL....",
    "....LEEEEEEkkkEEEEEEL....",
    "...LEEEEEEEkkkEEEEEEEL...",
    "...LEEEEEEEEkEEEEEEEEL...",
    "...LEEEEEEEEEEEEEEEEEL...",
    "...ELELELELELELELELELE...",
    "....aAaAAAAAAAAAAAaAa....",
    "....aAaAAAAAEAAAAAaAa....",
    "...aAaAAAAAELEAAAAAaAa...",
    "...aAaEAAAELLLEAAAEaAa...",
    "...aAELEAAAELEAAAELEAa...",
    "...aAaEAAAAAEAAAAAEaAa...",
    "...aAaAAAAAAAAAAAAAaAa...",
    "...aAaAAAAAAAAAAAAAaAa...",
    "...aAaAAAAAAEAAAAAAaAa...",
    "...aAaAAAAAELEAAAAAaAa...",
    "...aAEAAAAELLLEAAAAEAa...",
    "..aAELEAAAAELEAAAAELEAa..",
    "..aAaEAAAAAAEAAAAAAEaAa..",
    "..aAaAAAAAAAAAAAAAAAaAa..",
    "..aAaAAAAAAAAAAAAAAAaAa..",
    "..aAaAAAAAAAEAAAAAAAaAa..",
    "..EEEEEEEEEEEEEEEEEEEEE..",
    ".ELELELELELELELELELELELE.",
    ".LELELELELELELELELELELEL.",
    ".AaAaAaAaAaAaAaAaAaAaAaA.",
    ".....NNNN.......NNNN.....",
    "......nn.........nn......",
];
const WIDTH: i32 = 25;
const HEIGHT: i32 = CHINELO.len() as i32;
/// The row the cape starts on, where the arms are joined, and the row the
/// robe starts to swing from.
const SHOULDER: i32 = 17;
const WAIST: i32 = 22;
/// The columns of the left and right shoulder, and the length of an arm.
const SHOULDERS: [f32; 2] = [5.0, 19.0];
const ARM: f32 = 13.0;
/// How high the plumes reach above the hat, and where they spring from.
const PLUMES: f32 = 13.0;
const CROWN: (f32, f32) = (12.0, 1.0);
/// How high a hop takes a chinelo, in art pixels.
const HOP: f32 = 5.0;

/// Each chinelo's robe, shadow and embroidery.
const ROBES: [[u8; 3]; 6] = [*b"MmR", *b"RTQ", *b"QqY", *b"BbG", *b"UuY", *b"GgM"];
/// The colors the plumes come in, with the lighter spine of each.
const PLUME_COLORS: [(u8, u8); 6] = [
    (b'R', b'r'),
    (b'Q', b'z'),
    (b'Y', b'L'),
    (b'G', b'l'),
    (b'M', b'P'),
    (b'O', b'o'),
];

/// Hops per second, and seconds the crowd takes to cross.
const HOPS: f32 = 1.9;
const CROSSING: f32 = 19.0;

/// For a screen `w` columns by `h` rows: the scale of the art, the columns
/// each chinelo takes, and how many there are.
fn layout(w: u16, h: u16) -> (f32, i32, usize) {
    let k = fit(h, HEIGHT + PLUMES as i32, 0.8);
    let cell = (WIDTH as f32 * k * 0.92).round() as i32;
    (k, cell, (w as i32 / (cell + 3)).clamp(2, 6) as usize)
}

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let mut rng = fastrand::Rng::with_seed(rng.u64(..));
    let (k, cell, n) = layout(w, h);
    let first = rng.usize(..ROBES.len());
    let crowd = (0..n)
        .map(|i| Chinelo {
            robe: ROBES[(first + i * 5) % ROBES.len()],
            plumes: rng.usize(..PLUME_COLORS.len()),
            phase: rng.f32() * 2.0,
            pace: 0.94 + 0.12 * rng.f32(),
        })
        .collect();
    let span = n as i32 * cell;
    Box::new(Chinelos {
        t: 0.0,
        dist: 0.0,
        speed: ((w as i32 + span) as f32 / CROSSING).max(5.0),
        left: rng.bool(),
        w: w as i32,
        k,
        cell,
        crowd,
    })
}

struct Chinelo {
    robe: [u8; 3],
    /// Which plume colors it starts the fan with.
    plumes: usize,
    /// Where in its hopping it starts, and how fast it hops.
    phase: f32,
    pace: f32,
}

struct Chinelos {
    t: f32,
    /// Columns walked.
    dist: f32,
    speed: f32,
    /// Going right to left?
    left: bool,
    w: i32,
    /// Pixels to a pixel of art, and columns to a chinelo.
    k: f32,
    cell: i32,
    crowd: Vec<Chinelo>,
}

impl Chinelos {
    /// The column the crowd's back is at.
    fn origin(&self) -> i32 {
        let span = self.cell * self.crowd.len() as i32;
        let d = self.dist as i32;
        if self.left { self.w - d } else { d - span }
    }

    /// The left column of the art of chinelo `i` (0 leads).
    fn x(&self, i: usize) -> i32 {
        let slot = if self.left {
            i
        } else {
            self.crowd.len() - 1 - i
        } as i32;
        let art = (WIDTH as f32 * self.k).round() as i32;
        self.origin() + slot * self.cell + (self.cell - art) / 2
    }
}

impl Egg for Chinelos {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::carnaval(theme);
        let k = self.k;
        let ground = c.h() as i32 * 2;
        let wide = (WIDTH as f32 * k).round() as i32;
        let reach = (PLUMES * k) as i32;
        for (i, ch) in self.crowd.iter().enumerate() {
            let x = self.x(i);
            if x + wide + reach < 0 || x - reach > self.w {
                continue;
            }
            let hops = self.t * HOPS * ch.pace + ch.phase;
            let u = hops.fract();
            // Up and down in a parabola, a dip of the knees on landing.
            let lift = 4.0 * u * (1.0 - u) * HOP * k;
            let dip = if u < 0.1 || u > 0.9 { k } else { 0.0 };
            let top =
                ground - (HEIGHT as f32 * k).round() as i32 - lift.round() as i32 + dip as i32;
            // The robe swings from side to side every two hops.
            let swing = (hops * PI).sin();
            let [robe, shade, trim] = ch.robe;
            let color = |l: u8| match l {
                b'A' => pal.get(robe),
                b'a' => pal.get(shade),
                b'E' => pal.get(trim),
                _ => pal.get(l),
            };
            // The plumes, trailing a little behind the jump and fluttering.
            let air = 4.0 * (1.0 - 2.0 * u);
            for (p, (dx, len)) in [
                (-0.9, 0.8),
                (-0.45, 0.95),
                (0.0, 1.0),
                (0.45, 0.95),
                (0.9, 0.8),
            ]
            .into_iter()
            .enumerate()
            {
                let (cl, sp) = PLUME_COLORS[(ch.plumes + p) % PLUME_COLORS.len()];
                let flutter = (self.t * 4.3 + p as f32 * 1.7 + i as f32).sin() * 0.12;
                let ang = dx * 1.0 + flutter + swing * 0.12;
                let reach = PLUMES * len * (1.0 - 0.05 * air);
                let base = (
                    x as f32 + (CROWN.0 + dx * 3.0 + 0.5) * k,
                    top as f32 + (CROWN.1 + 1.0) * k,
                );
                let tip = (
                    base.0 + ang.sin() * reach * k,
                    base.1 - ang.cos() * reach * k,
                );
                plume(
                    c,
                    base,
                    tip,
                    dx * 1.6 * k - swing * k,
                    3.4 * k,
                    (pal.of(cl), pal.of(sp)),
                );
            }
            let shift = |r: i32| {
                let f = ((r - WAIST).max(0) as f32 / (HEIGHT - WAIST) as f32).powf(1.4);
                (swing * 2.4 * f).round() as i32
            };
            blit(c, CHINELO, Xf::new(x, top, k), shift, color);
            // The arms: the left and the right swing in turns, up and out
            // and down again, in their wide sleeves and white gloves.
            for (side, sx) in SHOULDERS.into_iter().enumerate() {
                let sign = if side == 0 { -1.0 } else { 1.0 };
                let up = (hops * PI + if side == 0 { PI } else { 0.0 }).sin();
                let ang = 0.95 + 0.95 * up;
                let from = (
                    x as f32 + (sx + 0.5) * k,
                    top as f32 + (SHOULDER as f32 + 1.0) * k,
                );
                let at = |f: f32| {
                    (
                        from.0 + sign * ang.sin() * ARM * f * k,
                        from.1 + ang.cos() * ARM * f * k,
                    )
                };
                let t = ((4.0 * k).round() as i32).max(1);
                line(c, from, at(0.7), t, pal.of(robe));
                line(c, at(0.7), at(0.85), t, pal.of(trim));
                line(
                    c,
                    at(0.85),
                    at(1.0),
                    ((3.0 * k).round() as i32).max(1),
                    pal.of(b'W'),
                );
            }
            // Sequins catching the light.
            let tick = (self.t * 9.0 + i as f32 * 3.0) as u32;
            for g in 0..3u32 {
                let r = (hash(tick, g * 7 + i as u32) * (HEIGHT - 4) as f32) as usize;
                let col = (hash(tick + 99, g * 5 + i as u32) * WIDTH as f32) as usize;
                if CHINELO[r].as_bytes().get(col) == Some(&b'L') {
                    let (px, py) = (x + (col as f32 * k) as i32, top + (r as f32 * k) as i32);
                    pixel(c, px, py, pal.of(b'W'));
                }
            }
        }
    }

    fn done(&self) -> bool {
        let span = self.cell * self.crowd.len() as i32;
        let gone = if self.left {
            self.origin() + span + self.cell < 0
        } else {
            self.origin() > self.w + self.cell
        };
        gone || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        assert!(CHINELO.iter().all(|r| r.len() as i32 == WIDTH));
        const { assert!(SHOULDER < WAIST && WAIST < HEIGHT) };
        let pal = Palette::carnaval(&Theme::classic());
        for letter in ROBES
            .iter()
            .flatten()
            .chain(PLUME_COLORS.iter().flat_map(|(a, b)| [a, b]))
        {
            assert!(pal.get(*letter).is_some(), "{}", *letter as char);
        }
        // Every letter of the art has a color.
        for &b in CHINELO.iter().flat_map(|r| r.as_bytes()) {
            assert!(
                b == b'.' || b"AaE".contains(&b) || pal.get(b).is_some(),
                "{}",
                b as char
            );
        }
    }

    /// The crowd and its dancers scale with the screen.
    #[test]
    fn the_crowd_scales_with_the_screen() {
        let (small, big) = (layout(40, 12), layout(400, 120));
        assert!(small.0 < big.0, "{small:?} {big:?}");
        assert!(small.2 >= 2 && big.2 > small.2 && big.2 <= 6);
        assert!(layout(1, 1).2 >= 2);
    }

    /// They cross from one side to the other, hopping: the top of the crowd
    /// rises and falls while its feet stay on the bottom row.
    #[test]
    fn hop_across_on_the_ground() {
        let theme = Theme::classic();
        for seed in 0..6 {
            let mut e = start(100, 30, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(100, 30, Color::Reset);
            let (mut tops, mut seen, mut grounded, mut secs) = (Vec::new(), 0, 0, 0.0);
            while !e.done() {
                e.update(0.05);
                secs += 0.05;
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                let rows: Vec<bool> = text.lines().map(|l| !l.trim().is_empty()).collect();
                if rows.iter().filter(|r| **r).count() > 10 {
                    seen += 1;
                    tops.push(rows.iter().position(|r| *r).unwrap());
                    grounded += i32::from(rows[29]);
                }
            }
            assert!(secs < 29.0, "{secs}");
            assert!(seen > 100, "{seen} frames with chinelos on the screen");
            assert!(
                tops.iter().min() != tops.iter().max(),
                "they never hopped: {tops:?}"
            );
            // They land on the bottom row between hops, and spend a while there.
            assert!(grounded > seen / 8, "{grounded} of {seen} frames");
        }
    }
}
