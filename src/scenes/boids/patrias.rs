//! Fiestas Patrias in the flock (see the module doc in `boids.rs`).
//!
//! The flock keeps its simulation and its random numbers; the holiday dresses
//! it: palomas for the boids, each one verde, blanco or rojo, and the águila
//! real for the hawk. It keeps nothing but a clock and its colors (each bird's
//! color is a hash of its index), and draws no random numbers, so ordinary
//! days are as they ever were.

use std::f32::consts::TAU;

use super::hues::{like, tricolor};
use super::{Boid, Boids, FRONT_DOTS, FRONT_LIGHT, FRONT_WINGS, HEAD_LUT, Ribbon, Style, hash01};
use crate::color::{Color, mix_oklab, rgb};
use crate::render::{BOLD, Canvas};
use crate::scene::Ctx;

/// The águila's café (the plumage), the oro of her beak and the gold of her
/// nape, and the serpent in her beak.
const AGUILA: u32 = 0x7a4a1e;
const ORO: u32 = 0xd4a017;
const SERPIENTE: u32 = 0xb9b4a6;
/// The águila real in place of the hawk, facing right (her head and her beak
/// are mirrored when she faces left): head and beak above her front, then
/// back, chest and tail. The second string colors each cell: `g` the gold of
/// her nape, `o` her beak, `p` plumage, `c` her golden chest.
const AGUILA_BODY: [(&str, &str); 3] = [("   ◉▸", "   go"), ("▟▇▇▇▙", "ppcpp"), ("  ▝▀▘", "  ppp")];

/// How strong the flag's wave runs through the flock.
const WAVE: f32 = 3.0;
/// A pluma (feather) of the águila's drifting down, turning over as it goes.
const PLUMA: [char; 4] = ['/', '|', '\\', '|'];
/// Plumas in the air at most.
const PLUMAS: usize = 14;

/// A feather falling from the águila, in units.
#[derive(Clone, Copy, Default)]
struct Pluma {
    x: f32,
    y: f32,
    /// Phase of its sway and turning.
    phase: f32,
    /// Seconds left.
    life: f32,
}

pub(super) struct Patrias {
    level: f32,
    /// Scene seconds since the holiday came, for the wings and the wave.
    pub(super) t: f32,
    plumas: [Pluma; PLUMAS],
    pluma_next: usize,
    pluma_clock: f32,
    /// Verde, blanco and rojo.
    flag: [Color; 3],
    wheel: [Color; HEAD_LUT],
    plumage: Color,
    feathers: [Color; 2],
    gold: Color,
    serpent: Color,
    /// The águila faces where she flies: 1 right, -1 left.
    pub(super) facing: f32,
    /// Which way her banner streams (a unit vector), lagging her turns.
    pub(super) drape: (f32, f32),
}

/// 0 below `a`, 1 above `b`, smooth in between.
fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl Patrias {
    pub(super) fn new(ctx: &Ctx) -> Patrias {
        Patrias {
            level: ctx.fiesta.level.clamp(0.0, 1.0),
            t: 0.0,
            plumas: [Pluma::default(); PLUMAS],
            pluma_next: 0,
            pluma_clock: 0.0,
            flag: [Color::Reset; 3],
            wheel: [Color::Reset; HEAD_LUT],
            plumage: Color::Reset,
            feathers: [Color::Reset; 2],
            gold: Color::Reset,
            serpent: Color::Reset,
            facing: 1.0,
            drape: (-1.0, 0.0),
        }
    }

    pub(super) fn level(&self) -> f32 {
        self.level
    }

    /// Colors from the theme: the flag's stripes and the águila's browns and
    /// golds each the closest accent.
    pub(super) fn retheme(&mut self, ctx: &Ctx) {
        let th = ctx.theme;
        self.flag = tricolor(th);
        for (i, slot) in self.wheel.iter_mut().enumerate() {
            let p = i as f32 / HEAD_LUT as f32 * 3.0;
            let k = p as usize % 3;
            *slot = mix_oklab(self.flag[k], self.flag[(k + 1) % 3], p.fract());
        }
        self.plumage = like(th, rgb(AGUILA));
        self.gold = like(th, rgb(ORO));
        // Her feathers go from a golden brown at the arm to the dark of the
        // flight feathers' tips.
        self.feathers = [
            mix_oklab(self.plumage, self.gold, 0.55),
            self.plumage.scale(0.9),
        ];
        self.serpent = like(th, rgb(SERPIENTE));
    }

    /// The color of paloma `i`: tricolor from the first of September and
    /// more toward the Grito, white in between.
    fn bird(&self, i: usize) -> Color {
        let h = hash01(i ^ 0x7a7a);
        let share = 0.34 + 0.66 * self.level;
        if h < share {
            self.flag[((h / share * 3.0) as usize).min(2)]
        } else {
            self.flag[1]
        }
    }

    /// Wing frame of paloma `i` (an index into `FRONT_WINGS`): bouts of
    /// beating, then a glide on level wings.
    fn wing_frame(&self, i: usize) -> usize {
        let h = hash01(i);
        if (self.t * 0.3 + h * 5.0).fract() > 0.7 {
            return 1;
        }
        let beats = 2.8 + 2.0 * hash01(i ^ 0x5555);
        match ((self.t * beats + h).fract() * 4.0) as usize {
            0 => 0,
            2 => 2,
            _ => 1,
        }
    }

    /// Extra steering for paloma `i`: a wave runs through the flock like the
    /// wind through the flag, higher toward the Grito.
    pub(super) fn force(&self, i: usize, b: &Boid) -> (f32, f32) {
        let wave = (b.x * 0.09 - self.t * 1.6).sin() * (WAVE + 7.0 * self.level);
        let bob = ((self.t * (1.4 + 0.6 * hash01(i ^ 0x3c3c)) + hash01(i)) * TAU).sin() * 4.0;
        (0.0, wave + bob)
    }

    /// The plumas drift down, swaying, and the águila sheds a new one every
    /// so often, oftener toward the Grito.
    pub(super) fn drift(&mut self, dt: f32, hawk: &Boid, hawk_on: bool) {
        for p in self.plumas.iter_mut().filter(|p| p.life > 0.0) {
            p.life -= dt;
            p.y += 2.4 * dt;
            p.x += (self.t * 1.3 + p.phase).sin() * 1.6 * dt;
        }
        if !hawk_on {
            return;
        }
        let every = 0.55 - 0.3 * self.level;
        self.pluma_clock += dt;
        while self.pluma_clock >= every {
            self.pluma_clock -= every;
            // Where it falls from is a hash of how many have fallen.
            let n = (self.t * 1000.0) as usize + self.pluma_next * 7919;
            self.plumas[self.pluma_next] = Pluma {
                x: hawk.x + (hash01(n) - 0.5) * 18.0,
                y: hawk.y + (hash01(n ^ 0x2f2f) - 0.4) * 5.0,
                phase: hash01(n ^ 0x6a6a) * TAU,
                life: 3.0 + 2.0 * hash01(n ^ 0x1b1b),
            };
            self.pluma_next = (self.pluma_next + 1) % PLUMAS;
        }
    }

    /// How raised the águila's wings are, in radians: bouts of slow beats,
    /// then a long soar.
    fn lift(&self) -> f32 {
        let cycle = (self.t / 9.0).fract();
        let beating = smooth(0.5, 0.58, cycle) * (1.0 - smooth(0.92, 1.0, cycle));
        let soar = 0.2 + 0.06 * (self.t * 0.8).sin();
        let beat = 0.3 + 0.7 * (self.t * 4.2).sin();
        soar + (beat - soar) * beating
    }
}

impl Boids {
    /// The color of paloma `i`: its flag color (or its heading's).
    fn paloma_col(&self, p: &Patrias, i: usize) -> Color {
        let b = &self.boids[i];
        if self.by_heading {
            let a = b.vy.atan2(b.vx).rem_euclid(TAU) / TAU;
            p.wheel[((a * HEAD_LUT as f32) as usize).min(HEAD_LUT - 1)]
        } else {
            p.bird(i)
        }
    }

    /// The flock as palomas, the águila real for the hawk.
    pub(super) fn draw_patrias(&mut self, ctx: &Ctx, c: &mut Canvas, p: &Patrias) {
        self.br.clear();
        let count = self.boids.len();
        if self.hawk_on {
            // Her banner: the flag streaming behind her, longer toward the Grito.
            let banner = Ribbon {
                cols: &p.flag,
                anchor: (
                    self.hawk.x.round() * 2.0 + 1.0,
                    ((self.hawk.y * 0.5) as i32 + 1) as f32 * 4.0 + 2.0,
                ),
                drape: p.drape,
                t: p.t,
                n: 14,
                len: 14.0 + 16.0 * p.level,
                fade: 0.5,
            };
            self.draw_ribbon(ctx, &banner);
        }
        if self.trail_len > 0 {
            // Each paloma trails a ribbon of its own color.
            for i in 0..count {
                let col = self.paloma_col(p, i);
                self.draw_trail(ctx, i, col, 0.7, 0.0);
            }
        }
        if self.style == Style::Dots {
            for i in 0..count {
                let b = self.boids[i];
                let w = p.wing_frame(i);
                let col = self.paloma_col(p, i).scale(FRONT_LIGHT[w]);
                let (x, y) = (
                    (b.x * 2.0).floor() as i32 - 2,
                    (b.y * 2.0).floor() as i32 - 1,
                );
                for (r, row) in FRONT_DOTS[w].iter().enumerate() {
                    for (k, d) in row.bytes().enumerate() {
                        if d == b'#' {
                            self.br.set(x + k as i32, y + r as i32, col);
                        }
                    }
                }
            }
        }
        if self.hawk_on {
            self.draw_wings(p);
        }
        self.br.blit(c);

        // Plumas falling from the águila, turning over.
        for pl in p.plumas.iter().filter(|pl| pl.life > 0.0) {
            let mut col = p.feathers[0];
            if pl.life < 1.0 {
                col = ctx.fade(col, 1.0 - pl.life);
            }
            let spin = (p.t * 2.0 + pl.phase) as usize % PLUMA.len();
            c.set(
                pl.x.floor() as i32,
                (pl.y * 0.5).floor() as i32,
                PLUMA[spin],
                col,
            );
        }
        if self.style == Style::Arrows {
            for i in 0..count {
                let b = &self.boids[i];
                let w = p.wing_frame(i);
                let col = self.paloma_col(p, i).scale(FRONT_LIGHT[w]);
                let (x, y) = (b.x.round() as i32, (b.y * 0.5) as i32);
                c.set_attr(x - 1, y, FRONT_WINGS[w][0], col, BOLD);
                c.set_attr(x, y, 'v', col, BOLD);
                c.set_attr(x + 1, y, FRONT_WINGS[w][1], col, BOLD);
            }
        }
        if self.hawk_on {
            self.draw_aguila(c, p);
        }
    }

    /// The águila's wings, in braille: the arm, the secondaries hanging back
    /// from it and the primaries splayed from the wrist like fingers.
    fn draw_wings(&mut self, p: &Patrias) {
        let a = p.lift();
        let (cx, cy) = (
            self.hawk.x.round() * 2.0 + 1.0,
            (self.hawk.y * 0.5).floor() * 4.0 + 1.0,
        );
        let [arm, tip] = p.feathers;
        for side in [-1.0f32, 1.0] {
            let sh = (cx + side * 4.0, cy - 1.0);
            let dir = |el: f32| (side * el.cos(), -el.sin());
            let (ax, ay) = dir(a);
            let wrist = (sh.0 + ax * 11.0, sh.1 + ay * 11.0);
            // The arm, thick: the leading edge.
            for o in [0.0, 1.0, 2.0] {
                self.br.line(sh.0, sh.1 + o, wrist.0, wrist.1 + o, arm);
            }
            // The secondaries hang from the arm, a feather and its shadow.
            let (hx, hy) = dir(a - 1.2);
            for k in 0..8 {
                let t = (k as f32 + 1.0) / 8.0;
                let from = (sh.0 + (wrist.0 - sh.0) * t, sh.1 + (wrist.1 - sh.1) * t);
                let l = 9.0 - 2.5 * t;
                for o in [0.0, side] {
                    self.br.line_with(
                        from.0 + o,
                        from.1,
                        from.0 + o + hx * l,
                        from.1 + hy * l,
                        |s| arm.lerp(tip, s * s),
                    );
                }
            }
            // The primaries, splayed like fingers.
            for (k, len) in [9.0, 12.0, 14.0, 14.0, 12.0, 9.0].into_iter().enumerate() {
                let (dx, dy) = dir(a - 0.55 + 0.2 * k as f32);
                for o in [0.0, side] {
                    self.br.line_with(
                        wrist.0 + o,
                        wrist.1,
                        wrist.0 + o + dx * len,
                        wrist.1 + dy * len,
                        |s| arm.lerp(tip, s),
                    );
                }
            }
        }
    }

    /// The águila's body, in cells, with the serpent writhing in her beak.
    fn draw_aguila(&self, c: &mut Canvas, p: &Patrias) {
        let right = p.facing > 0.0;
        let (x, y) = (self.hawk.x.round() as i32, (self.hawk.y * 0.5) as i32);
        for (r, (art, keys)) in AGUILA_BODY.iter().enumerate() {
            let n = art.chars().count() as i32;
            for (k, (ch, key)) in art.chars().zip(keys.chars()).enumerate() {
                if ch == ' ' {
                    continue;
                }
                // Facing left, the whole bird is mirrored.
                let (dx, ch) = if right {
                    (k as i32, ch)
                } else {
                    (n - 1 - k as i32, mirror(ch))
                };
                let col = match key {
                    'g' => p.gold.lerp(Color::Rgb(255, 255, 255), 0.15),
                    'o' => p.gold,
                    'c' => mix_oklab(p.plumage, p.gold, 0.5),
                    _ => p.plumage,
                };
                let attr = if r >= 1 { BOLD } else { 0 };
                c.set_attr(x - 2 + dx, y - 1 + r as i32, ch, col, attr);
            }
        }
        // The serpent hangs from her beak, writhing.
        let ch = ['~', '≈', '~', '∽'][(p.t * 6.0) as usize % 4];
        let sx = if right { x + 3 } else { x - 3 };
        c.set_attr(sx, y, ch, p.serpent, BOLD);
    }
}

/// The mirror image of a glyph of the águila.
fn mirror(ch: char) -> char {
    match ch {
        '▸' => '◂',
        '▟' => '▙',
        '▙' => '▟',
        '▝' => '▘',
        '▘' => '▝',
        ch => ch,
    }
}
