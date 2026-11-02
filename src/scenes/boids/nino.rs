//! Día del Niño in the flock (see the module doc in `boids.rs`).
//!
//! The flock keeps its simulation and its random numbers; the holiday dresses
//! it: papalotes (kites) in two toy colors for the boids, each with a long
//! tail that follows the way it flew and flutters, a big papalote for the hawk,
//! and a wind that blows the flock about. It keeps nothing but a clock, its
//! colors and the wisps of wind (placed by a hash), and draws no random
//! numbers, so ordinary days are as they ever were.

use std::f32::consts::TAU;

use super::hues::like;
use super::{Boids, HEAD_LUT, MAX_TRAIL, Style, hash01};
use crate::color::{Color, mix_oklab, rgb};
use crate::render::{BOLD, Canvas};
use crate::scene::Ctx;

/// The toys' colors: rojo, amarillo, azul, verde and naranja, and the madera
/// of a papalote's frame.
pub(super) const TOYS: [u32; 5] = [0xe8322a, 0xffd200, 0x1f7ae0, 0x2fb84a, 0xff8c00];
const MADERA: u32 = 0xc08a4a;
/// A papalote seen from below, two cells: its upper and its lower half.
const KITE: [[char; 2]; 2] = [['◢', '◣'], ['◥', '◤']];
/// The same in braille dots for `style = "dots"`, 5×5 around the boid.
const KITE_DOTS: [&str; 5] = ["..#..", ".###.", "#####", ".###.", "..#.."];
/// The bow of paper tied on a tail.
const BOW: char = '❖';
/// A big papalote's half-width, in pixels (a pixel is one cell wide and half a
/// cell tall): it is 11 cells wide and 6 tall.
const BIG: i32 = 5;
/// Wisps of wind drawn on the sky.
const WISPS: usize = 60;

pub(super) struct Nino {
    /// Scene seconds since the holiday came, for the wind and the flutter.
    pub(super) t: f32,
    toys: [Color; 5],
    wheel: [Color; HEAD_LUT],
    wood: Color,
    wisp: Color,
    /// Where the wisps of wind are, in units (their number grows with the area).
    wisps: Vec<(f32, f32)>,
}

impl Nino {
    pub(super) fn new() -> Nino {
        Nino {
            t: 0.0,
            toys: [Color::Reset; 5],
            wheel: [Color::Reset; HEAD_LUT],
            wood: Color::Reset,
            wisp: Color::Reset,
            wisps: Vec::new(),
        }
    }

    /// Colors from the theme: its accents are the toys' colors under any
    /// theme, and the frame the accent closest to wood.
    pub(super) fn retheme(&mut self, ctx: &Ctx) {
        for (i, (slot, &hex)) in self.toys.iter_mut().zip(&TOYS).enumerate() {
            *slot = ctx.pick(i, rgb(hex));
        }
        for (i, slot) in self.wheel.iter_mut().enumerate() {
            let p = i as f32 / HEAD_LUT as f32 * 5.0;
            let k = p as usize % 5;
            *slot = mix_oklab(self.toys[k], self.toys[(k + 1) % 5], p.fract());
        }
        self.wood = like(ctx.theme, rgb(MADERA));
        self.wisp = ctx.fade(like(ctx.theme, rgb(0xcfe4f2)), 0.65);
    }

    /// Scatters the wisps of wind over a field.
    pub(super) fn scatter(&mut self, fw: f32, fh: f32) {
        let n = ((fw * fh * 0.5 * 0.0035).round() as usize).min(WISPS);
        self.wisps.clear();
        for i in 0..n {
            self.wisps
                .push((hash01(i * 2 + 1) * fw, hash01(i * 2 + 2) * fh));
        }
    }

    /// The wind at (x, y), in units per second squared: it blows across the
    /// sky and turns with the height and the hour, in gusts.
    pub(super) fn wind(&self, x: f32, y: f32) -> (f32, f32) {
        let t = self.t;
        let a = 0.45 * (t * 0.13).sin()
            + 0.5 * (y * 0.06 + t * 0.21).sin()
            + 0.25 * (x * 0.04 - t * 0.17).sin();
        let k = 7.0 + 4.0 * (t * 0.37 + x * 0.05).sin();
        (a.cos() * k, a.sin() * k)
    }

    /// Extra steering for papalote `i`: the wind, and a bob as it dances.
    pub(super) fn force(&self, i: usize, x: f32, y: f32) -> (f32, f32) {
        let (wx, wy) = self.wind(x, y);
        let bob = ((self.t * (0.9 + 0.5 * hash01(i ^ 0x3c3c)) + hash01(i)) * TAU).sin() * 9.0;
        (wx, wy + bob)
    }

    /// The wisps of wind blow along.
    pub(super) fn drift(&mut self, dt: f32, fw: f32, fh: f32) {
        for k in 0..self.wisps.len() {
            let (x, y) = self.wisps[k];
            let (wx, wy) = self.wind(x, y);
            self.wisps[k] = (
                (x + wx * 1.4 * dt).rem_euclid(fw),
                (y + wy * 1.4 * dt).rem_euclid(fh),
            );
        }
    }

    /// The two toy colors of papalote `i`, swapped as it turns over.
    fn colors(&self, i: usize) -> [Color; 2] {
        let a = (hash01(i ^ 0x1234) * 5.0) as usize % 5;
        let b = (a + 1 + (hash01(i ^ 0x4321) * 4.0) as usize % 4) % 5;
        let turn = (self.t * 0.8 + hash01(i) * 7.0).fract() < 0.5;
        if turn {
            [self.toys[a], self.toys[b]]
        } else {
            [self.toys[b], self.toys[a]]
        }
    }

    /// Is papalote `i` a small one, a single cell?
    fn small(i: usize) -> bool {
        hash01(i ^ 0x7777) < 0.3
    }

    /// How many points long the tail of papalote `i` is, given the full
    /// length: the small ones have shorter tails.
    fn reach(i: usize, len: usize) -> usize {
        if Nino::small(i) { len * 2 / 3 } else { len }
    }
}

impl Boids {
    /// The two colors of papalote `i`: its toy colors (or its heading's and
    /// the opposite one).
    fn kite_cols(&self, n: &Nino, i: usize) -> [Color; 2] {
        if self.by_heading {
            let b = &self.boids[i];
            let a = b.vy.atan2(b.vx).rem_euclid(TAU) / TAU;
            let at = |a: f32| {
                n.wheel[((a.rem_euclid(1.0) * HEAD_LUT as f32) as usize).min(HEAD_LUT - 1)]
            };
            [at(a), at(a + 0.5)]
        } else {
            n.colors(i)
        }
    }

    /// Where the tail of boid `i` (the hawk is `boids.len()`) is at its `j`th
    /// point, in dots, fluttering: `j = 0` is the boid itself, the rest the way
    /// it flew. `None` where the boid wrapped around, and past the end.
    fn kite_tail_at(&self, n: &Nino, i: usize, j: usize, amp: f32) -> Option<(f32, f32)> {
        let raw = |j: usize| -> Option<(f32, f32)> {
            if j == 0 {
                let b = if i < self.boids.len() {
                    &self.boids[i]
                } else {
                    &self.hawk
                };
                Some((b.x, b.y))
            } else if j <= self.trail_fill {
                let idx = (self.trail_head + MAX_TRAIL - (j - 1)) % MAX_TRAIL;
                Some(self.trail[i * MAX_TRAIL + idx])
            } else {
                None
            }
        };
        let p = raw(j)?;
        let (fw, fh) = (self.fw, self.fh);
        if j == 0 {
            return Some((p.0 * 2.0, p.1 * 2.0));
        }
        let q = raw(j - 1)?;
        let (dx, dy) = (p.0 - q.0, p.1 - q.1);
        // Do not draw across the screen where the boid wrapped around.
        if dx.abs() >= fw * 0.5 || dy.abs() >= fh * 0.5 {
            return None;
        }
        let l = (dx * dx + dy * dy).sqrt().max(1e-3);
        // The tail flutters across the way it flew, more toward its end.
        let s = j as f32 / MAX_TRAIL as f32;
        let w = (n.t * 8.0 - j as f32 * 0.9 + hash01(i) * TAU).sin() * amp * (0.2 + s);
        Some((p.0 * 2.0 - dy / l * w, p.1 * 2.0 + dx / l * w))
    }

    /// Draws the flutter of tail `i`, `len` points long, in braille.
    fn draw_kite_tail(&mut self, ctx: &Ctx, n: &Nino, i: usize, len: usize, amp: f32) {
        let (fw, fh) = (self.fw, self.fh);
        let head = if i < self.boids.len() {
            &self.boids[i]
        } else {
            &self.hawk
        };
        // The same points as `kite_tail_at`, one after the other.
        let (mut prev, mut dots) = ((head.x, head.y), (head.x * 2.0, head.y * 2.0));
        let (clock, phase) = (n.t * 8.0, hash01(i) * TAU);
        let mut col = Color::Reset;
        for j in 1..len.min(self.trail_fill + 1) {
            let idx = (self.trail_head + MAX_TRAIL - (j - 1)) % MAX_TRAIL;
            let p = self.trail[i * MAX_TRAIL + idx];
            let (dx, dy) = (p.0 - prev.0, p.1 - prev.1);
            // Do not draw across the screen where the boid wrapped around.
            if dx.abs() >= fw * 0.5 || dy.abs() >= fh * 0.5 {
                break;
            }
            let l = (dx * dx + dy * dy).sqrt().max(1e-3);
            let w =
                (clock - j as f32 * 0.9 + phase).sin() * amp * (0.2 + j as f32 / MAX_TRAIL as f32);
            let q = (p.0 * 2.0 - dy / l * w, p.1 * 2.0 + dx / l * w);
            // Rags of paper in the toys' colors, three points of each, fainter
            // toward the end.
            if j % 3 == 1 {
                let k = 0.1 + 0.5 * (j + 1) as f32 / len as f32;
                col = ctx.fade(n.toys[(j / 3 + i) % 5], k);
            }
            self.br.line(dots.0, dots.1, q.0, q.1, col);
            (prev, dots) = (p, q);
        }
    }

    /// Ties the bows on tail `i`, one every `every` points.
    fn draw_bows(&self, c: &mut Canvas, n: &Nino, i: usize, len: usize, amp: f32, every: usize) {
        for j in (every..len.min(self.trail_fill + 1)).step_by(every) {
            let Some(p) = self.kite_tail_at(n, i, j, amp) else {
                break;
            };
            let col = n.toys[(j / every + i + 2) % 5];
            c.set_attr(
                (p.0 / 2.0).floor() as i32,
                (p.1 / 4.0).floor() as i32,
                BOW,
                col,
                BOLD,
            );
        }
    }

    /// The flock as papalotes, a big one for the hawk, the wind around them.
    pub(super) fn draw_nino(&mut self, ctx: &Ctx, c: &mut Canvas, n: &Nino) {
        self.br.clear();
        let count = self.boids.len();
        // The wisps of wind: a short curl along the way they blow.
        for &(x, y) in &n.wisps {
            let (wx, wy) = n.wind(x, y);
            let l = (wx * wx + wy * wy).sqrt().max(1e-3);
            let (dx, dy) = (wx / l, wy / l);
            for k in 0..7 {
                let s = k as f32;
                let bend = (s * 0.7).sin() * 0.8;
                self.br.setf(
                    x * 2.0 + dx * s * 1.4 - dy * bend,
                    y * 2.0 + dy * s * 1.4 + dx * bend,
                    n.wisp,
                );
            }
        }
        // Tails: long, and fluttering; the big papalote's longest.
        if self.trail_len > 0 {
            let len = (self.trail_len + 8).min(MAX_TRAIL);
            for i in 0..count {
                self.draw_kite_tail(ctx, n, i, Nino::reach(i, len), 3.0);
            }
            if self.hawk_on {
                self.draw_kite_tail(ctx, n, count, MAX_TRAIL, 4.5);
            }
        }
        if self.style == Style::Dots {
            for i in 0..count {
                let b = self.boids[i];
                let [left, right] = self.kite_cols(n, i);
                let (x, y) = (
                    (b.x * 2.0).floor() as i32 - 2,
                    (b.y * 2.0).floor() as i32 - 2,
                );
                for (r, row) in KITE_DOTS.iter().enumerate() {
                    for (k, d) in row.bytes().enumerate() {
                        if d == b'#' {
                            let col = if k < 2 { left } else { right };
                            self.br.set(x + k as i32, y + r as i32, col);
                        }
                    }
                }
            }
        }
        self.br.blit(c);

        if self.trail_len > 0 {
            let len = (self.trail_len + 8).min(MAX_TRAIL);
            for i in 0..count {
                self.draw_bows(c, n, i, Nino::reach(i, len), 3.0, 6);
            }
            if self.hawk_on {
                self.draw_bows(c, n, count, MAX_TRAIL, 4.5, 3);
            }
        }
        if self.style == Style::Arrows {
            for i in 0..count {
                let b = &self.boids[i];
                let [left, right] = self.kite_cols(n, i);
                let (x, y) = (b.x.round() as i32, (b.y * 0.5) as i32);
                if Nino::small(i) {
                    c.set_attr(x, y, '◆', left, BOLD);
                    continue;
                }
                for (r, row) in KITE.iter().enumerate() {
                    c.set_attr(x - 1, y - 1 + r as i32, row[0], left, BOLD);
                    c.set_attr(x, y - 1 + r as i32, row[1], right, BOLD);
                }
            }
        }
        if self.hawk_on {
            self.draw_big_papalote(c, n);
        }
    }

    /// The big papalote that scatters the flock: a diamond of four colors in a
    /// frame of madera, drawn in half blocks.
    fn draw_big_papalote(&self, c: &mut Canvas, n: &Nino) {
        let (cx, cy) = (self.hawk.x.round() as i32, self.hawk.y.floor() as i32);
        // What color the pixel (px, py) is: None outside the diamond.
        let pixel = |px: i32, py: i32| -> Option<Color> {
            let (dx, dy) = (px - cx, py - cy);
            let d = dx.abs() + dy.abs();
            if d > BIG {
                return None;
            }
            // The frame: its rim and its two spars.
            if d == BIG || dx == 0 || dy == 0 {
                return Some(n.wood);
            }
            let q = usize::from(dx > 0) + 2 * usize::from(dy > 0);
            Some(n.toys[q])
        };
        for row in (cy - BIG).div_euclid(2)..=(cy + BIG).div_euclid(2) {
            for x in cx - BIG..=cx + BIG {
                let (top, bottom) = (pixel(x, 2 * row), pixel(x, 2 * row + 1));
                let (ch, col) = match (top, bottom) {
                    (None, None) => continue,
                    (Some(t), None) => ('▀', t),
                    (None, Some(b)) => ('▄', b),
                    (Some(t), Some(_)) => ('█', t),
                };
                c.set(x, row, ch, col);
            }
        }
    }
}
