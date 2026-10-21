//! The tank, after tanim's aquarium: water with slow rays of light, a
//! rippling surface, sandy dunes with rocks, pebbles, shells, a castle and
//! small ornaments, swaying weeds, the crab and rising bubbles. Here the sky
//! shows over the surface, and a catch or a splash leaves a few bits behind.

use std::f32::consts::TAU;

use super::art::{CASTLE, CRAB, CRAB_W, each_pixel, rgb, size};
use super::sea::{Sea, hash};
use super::{Dice, Rng};
use crate::color::Color;

const SAND: Color = rgb(206, 182, 128);
const SAND_DARK: Color = rgb(150, 126, 84);
const BUBBLE: Color = rgb(180, 226, 248);
pub const EYE: Color = rgb(16, 16, 24);
pub const EYE_WHITE: Color = rgb(245, 245, 245);
const WHITE: Color = rgb(255, 255, 255);
const CRAB_SHELL: Color = rgb(226, 78, 48);
const CRAB_LEG: Color = rgb(180, 54, 36);
const STONE: Color = rgb(150, 142, 150);
const ROOF: Color = rgb(196, 90, 70);
const WEED_TIP: Color = rgb(150, 226, 110);
const WEED_COLORS: [Color; 4] = [
    rgb(30, 120, 60),
    rgb(56, 140, 50),
    rgb(24, 100, 80),
    rgb(90, 120, 40),
];
const PEBBLES: [Color; 6] = [
    rgb(120, 116, 110),
    rgb(170, 160, 150),
    rgb(110, 84, 60),
    rgb(230, 226, 214),
    rgb(100, 120, 140),
    rgb(170, 100, 80),
];
/// Spray over the surface and plankton drifting into a manta's mouth.
const SPRAY: Color = rgb(226, 244, 252);
pub const PLANKTON: Color = rgb(170, 222, 168);

pub struct Weed {
    pub x: i32,
    pub height: i32,
    pub phase: f32,
    pub color: Color,
    /// Broad kelp with side leaves rather than a thin strand.
    pub kelp: bool,
}

pub struct Crab {
    pub x: f32,
    pub dir: f32,
    pub pause: u32,
}

pub struct Bubble {
    pub x: f32,
    pub y: f32,
    pub age: u32,
    pub phase: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BitKind {
    /// A scale catching the light where something was caught.
    Glint,
    /// A drop of spray, falling back to the water.
    Drop,
    /// A speck of plankton.
    Speck,
}

pub struct Bit {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: u16,
    pub max: u16,
    pub c: Color,
    pub kind: BitKind,
}

/// tanim's scaling: every channel times `k`, saturating.
pub fn mul(c: Color, k: f32) -> Color {
    match c {
        Color::Rgb(r, g, b) => {
            let f = |a: u8| (a as f32 * k).clamp(0.0, 255.0) as u8;
            Color::Rgb(f(r), f(g), f(b))
        }
        c => c,
    }
}

/// A multi-stop gradient sampled at `t` in `[0, 1]`.
pub fn ramp(stops: &[Color], t: f32) -> Color {
    let n = stops.len();
    let t = t.clamp(0.0, 1.0) * (n - 1) as f32;
    let i = (t as usize).min(n - 2);
    stops[i].lerp(stops[i + 1], t - i as f32)
}

impl Sea {
    pub fn floor_at(&self, x: i32) -> i32 {
        self.sand_top[x.clamp(0, self.w - 1) as usize]
    }

    /// Highest sand surface under `x..x + width`.
    pub fn floor_under(&self, x: i32, width: i32) -> i32 {
        (x..x + width)
            .map(|xx| self.floor_at(xx))
            .min()
            .unwrap_or(self.ph)
    }

    fn set_scenery(&mut self, x: i32, y: i32, c: Color) {
        if x >= 0 && y >= 0 && x < self.w && y < self.ph {
            self.scenery[(y * self.w + x) as usize] = Some(c);
        }
    }

    /// Sand, rocks, a castle and small ornaments. Static, so laid once.
    pub fn build_scenery(&mut self, rng: &mut Rng) {
        let (w, ph) = (self.w, self.ph);
        let seed = rng.u32(..);
        let big = w >= 60 && ph >= 40;

        // Rocks and the castle stand half buried, so they go in first.
        for _ in 0..(w / 45).max(1) {
            let (rx, ry) = (rng.rangef(2.5, 6.5), rng.rangef(2.0, 4.5));
            let cx = rng.rangef(0.0, w as f32);
            let cy = self.floor_at(cx as i32) as f32 + 1.0;
            let tone = rng.rangef(0.8, 1.15);
            for y in (cy - ry) as i32..=(cy + ry) as i32 {
                for x in (cx - rx) as i32..=(cx + rx) as i32 {
                    let (dx, dy) = ((x as f32 - cx) / rx, (y as f32 - cy) / ry);
                    if dx * dx + dy * dy <= 1.0 {
                        let lit = 1.0 - (dx + dy) * 0.25;
                        let speck = if hash(x, y, seed).is_multiple_of(6) {
                            0.85
                        } else {
                            1.0
                        };
                        self.set_scenery(x, y, mul(rgb(104, 104, 114), tone * lit * speck));
                    }
                }
            }
        }
        if big {
            let (cw, chh) = size(CASTLE);
            let x0 = rng.range(4, (w - cw - 4).max(5));
            let y0 = self.floor_under(x0, cw) + 2 - chh;
            each_pixel(CASTLE, false, |x, y, ch| {
                let c = match ch {
                    b'G' => STONE,
                    b'g' => mul(STONE, 0.72),
                    b'r' => ROOF,
                    b'R' => mul(ROOF, 0.7),
                    _ => rgb(20, 24, 40),
                };
                self.set_scenery(x0 + x, y0 + y, c);
            });
            // Bubbles stream out of the door.
            self.vents.push((x0 + cw / 2, y0 + chh - 3));
        }

        for x in 0..w {
            let top = self.floor_at(x);
            for y in top..ph {
                let depth = (y - top) as f32 / (ph - top).max(1) as f32;
                let mut c = if y == top {
                    SAND.lerp(WHITE, 0.15)
                } else {
                    SAND.lerp(SAND_DARK, depth * 0.7)
                };
                match hash(x, y, seed) % 9 {
                    0 => c = mul(c, 0.86),
                    1 => c = c.lerp(WHITE, 0.12),
                    _ => {}
                }
                // Keep what already pokes out of the sand in front of it.
                let i = (y * w + x) as usize;
                if self.scenery[i].is_none() || y > top + 1 {
                    self.scenery[i] = Some(c);
                }
            }
        }

        for _ in 0..w / 5 {
            let x = rng.range(0, w);
            let y = self.floor_at(x) + rng.range(0, 3);
            let c = rng.pick(&PEBBLES);
            self.set_scenery(x, y, c);
            if rng.chance(0.4) {
                self.set_scenery(x + 1, y, mul(c, 0.8));
            }
        }
        for _ in 0..w / 30 {
            let x = rng.range(0, w - 3);
            let y = self.floor_under(x, 3) - 1;
            let c = rng.pick(&[rgb(246, 214, 200), rgb(250, 236, 220), rgb(236, 180, 170)]);
            // A scallop: fanned top edge over a solid base.
            self.set_scenery(x, y, c);
            self.set_scenery(x + 2, y, c);
            for dx in 0..3 {
                self.set_scenery(x + dx, y + 1, mul(c, 0.85));
            }
            self.set_scenery(x + 1, y, c.lerp(WHITE, 0.4));
        }
        for _ in 0..w / 40 {
            let x = rng.range(1, w - 1);
            let y = self.floor_under(x - 1, 3);
            let c = rng.pick(&[rgb(240, 120, 60), rgb(236, 90, 110), rgb(250, 170, 60)]);
            for (dx, dy) in [(0, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (1, 1)] {
                self.set_scenery(x + dx, y + dy, c);
            }
            self.set_scenery(x, y, c.lerp(WHITE, 0.3));
        }
        for _ in 0..(w / 40).max(1) {
            let x = rng.range(0, w);
            self.vents.push((x, self.floor_at(x) - 1));
        }
    }

    /// Weeds along the floor and, in a tank big enough, the crab.
    pub fn plant(&mut self, rng: &mut Rng) {
        let (w, ph) = (self.w, self.ph);
        let tall = ((self.floor - self.surf) * 2 / 3).max(5);
        for _ in 0..(w / 9).max(1) {
            let kelp = rng.chance(0.4);
            self.weeds.push(Weed {
                x: rng.range(0, w),
                height: rng.range(4, tall.min(ph / 2).max(5)),
                phase: rng.rangef(0.0, TAU),
                color: rng.pick(&WEED_COLORS),
                kelp,
            });
        }
        if w >= 20 && ph >= 16 {
            self.crab = Some(Crab {
                x: rng.rangef(0.0, (w - CRAB_W) as f32),
                dir: 1.0,
                pause: 0,
            });
        }
    }

    /// A tick of the tank: the crab, the bubbles and the bits.
    pub fn tank(&mut self, rng: &mut Rng) {
        let w = self.w;
        if let Some(cr) = &mut self.crab {
            if cr.pause > 0 {
                cr.pause -= 1;
            } else {
                cr.x += cr.dir * 0.12;
                if cr.x < 0.0 || cr.x > (w - CRAB_W) as f32 {
                    cr.dir = -cr.dir;
                    cr.x = cr.x.min((w - CRAB_W) as f32).max(0.0);
                }
                if rng.chance(0.006) {
                    cr.pause = rng.range(20, 100) as u32;
                }
            }
        }

        // Bubbles from vents, rising and growing until they reach the surface.
        for i in 0..self.vents.len() {
            let (vx, vy) = self.vents[i];
            if (self.t / 40).wrapping_add(vx as u32).is_multiple_of(3) && rng.chance(0.08) {
                self.bubble(vx as f32, vy as f32, rng);
            }
        }
        let top = (self.surf + 2) as f32;
        self.bubbles.retain_mut(|b| {
            b.age += 1;
            b.y -= 0.28;
            b.y >= top
        });

        let surf = self.surf as f32;
        self.bits.retain_mut(|b| {
            b.life = b.life.saturating_sub(1);
            b.x += b.vx;
            b.y += b.vy;
            match b.kind {
                BitKind::Glint => {
                    // Scales flutter to a stop and sink.
                    b.vx *= 0.88;
                    b.vy = b.vy * 0.88 + 0.006;
                }
                BitKind::Drop => {
                    b.vy += 0.06;
                    if b.y > surf + 1.0 && b.vy > 0.0 {
                        b.life = 0;
                    }
                }
                BitKind::Speck => {}
            }
            b.life > 0
        });
    }

    pub fn bubble(&mut self, x: f32, y: f32, rng: &mut Rng) {
        if self.bubbles.len() < 400 {
            self.bubbles.push(Bubble {
                x,
                y,
                age: 0,
                phase: rng.rangef(0.0, TAU),
            });
        }
    }

    pub fn bit(&mut self, bit: Bit) {
        if self.bits.len() < 1500 {
            self.bits.push(bit);
        }
    }

    /// What is left where something was caught: its scales glinting as they
    /// flutter down, and a burst of bubbles. No more than that.
    pub fn puff(&mut self, (x, y): (f32, f32), colors: [Color; 3], n: usize, rng: &mut Rng) {
        for _ in 0..n {
            let a = rng.rangef(0.0, TAU);
            let s = rng.rangef(0.1, 0.6);
            let max = rng.range(12, 30) as u16;
            self.bit(Bit {
                x: x + rng.rangef(-1.5, 1.5),
                y: y + rng.rangef(-1.0, 1.0),
                vx: a.cos() * s,
                vy: a.sin() * s * 0.7,
                life: max,
                max,
                c: rng.pick(&colors).lerp(WHITE, 0.35),
                kind: BitKind::Glint,
            });
        }
        for _ in 0..(n / 3).max(2) {
            let (bx, by) = (x + rng.rangef(-2.0, 2.0), y + rng.rangef(-1.5, 1.5));
            self.bubble(bx, by, rng);
        }
    }

    /// Spray where something breaks the surface at `x`.
    pub fn splash(&mut self, x: f32, n: usize, rng: &mut Rng) {
        let y = self.surf as f32;
        for _ in 0..n {
            let max = rng.range(14, 26) as u16;
            self.bit(Bit {
                x: x + rng.rangef(-3.0, 3.0),
                y: y - 0.5,
                vx: rng.rangef(-0.35, 0.35),
                vy: -rng.rangef(0.3, 0.9),
                life: max,
                max,
                c: SPRAY,
                kind: BitKind::Drop,
            });
        }
    }

    pub fn paint_water(&mut self, t: f32) {
        let (w, ph, top) = (self.w, self.ph, self.surf);
        // Slow diagonal light rays that fade with depth.
        for (u, r) in self.rays.iter_mut().enumerate() {
            let u = u as f32;
            let s = ((u * 0.1 + t * 0.011).sin() + (u * 0.043 - t * 0.006).sin()) * 0.5;
            *r = s.max(0.0).powi(3) * 0.36;
        }
        let water = self.set.water;
        if water {
            for y in 0..top {
                let c = self.pal.sky[0].lerp(self.pal.sky[1], y as f32 / top.max(1) as f32);
                for x in 0..w {
                    self.put(x, y, c);
                }
            }
        }
        let depth = (ph - top).max(1);
        let ray_depth = depth as f32 * 0.7;
        for y in top..ph {
            let dy = y - top;
            let base = ramp(&self.pal.water, dy as f32 / depth as f32);
            let fall = (1.0 - dy as f32 / ray_depth).max(0.0);
            // Coarse steps: a cell is only resent when its level changes.
            let mut levels = [base; 8];
            if fall > 0.0 {
                for (k, l) in levels.iter_mut().enumerate().skip(1) {
                    *l = base.lerp(self.pal.ray, k as f32 / 14.0);
                }
            }
            let row = (y * w) as usize;
            for x in 0..w {
                let c = match self.scenery[row + x as usize] {
                    Some(c) => c,
                    None if !water => continue,
                    None if fall > 0.0 => {
                        let r = self.rays[(x + dy / 3) as usize];
                        levels[((r * fall * 14.0).round() as usize).min(7)]
                    }
                    None => base,
                };
                self.put(x, y, c);
            }
        }
    }

    /// The rippling surface in three shades, so only wave edges change.
    pub fn paint_surface(&mut self, t: f32) {
        let (top, c) = (self.surf, self.pal.surface);
        for x in 0..self.w {
            let v = (x as f32 * 0.35 + t * 0.1).sin() + (x as f32 * 0.13 - t * 0.06).sin();
            let a = if v > 0.6 {
                0.7
            } else if v > -0.4 {
                0.45
            } else {
                0.25
            };
            self.blend(x, top, c, a);
            if v > 1.2 {
                self.blend(x, top + 1, c, 0.3);
            }
        }
    }

    pub fn draw_weeds(&mut self, t: f32) {
        for i in 0..self.weeds.len() {
            let Weed {
                x: wx,
                height,
                phase,
                color,
                kelp,
            } = self.weeds[i];
            let base = self.floor_at(wx) + 1;
            let mut prev = wx;
            for j in 0..height {
                let k = j as f32 / height as f32;
                let sway = (t * 0.045 + phase + j as f32 * 0.16).sin() * k * 3.2;
                let x = wx + sway.round() as i32;
                let y = base - j;
                let c = color.lerp(WEED_TIP, k * 0.55);
                // Keep the strand joined where it leans more than a pixel.
                for xx in prev.min(x) + 1..prev.max(x) {
                    self.put(xx, y, c);
                }
                self.put(x, y, c);
                if kelp {
                    if k < 0.75 {
                        self.put(x + 1, y, mul(c, 0.8));
                    }
                    if j % 4 == 2 && k < 0.9 {
                        let side = if j % 8 == 2 { -1 } else { 2 };
                        let leaf = c.lerp(WEED_TIP, 0.3);
                        self.put(x + side, y, leaf);
                        self.put(x + side + side.signum(), y - 1, leaf);
                    }
                }
                prev = x;
            }
        }
    }

    pub fn draw_crab(&mut self) {
        let Some(cr) = &self.crab else { return };
        let walking = cr.pause == 0;
        let x0 = cr.x.round() as i32;
        let frame = if walking && (self.t / 6).is_multiple_of(2) {
            0
        } else {
            1
        };
        let y0 = self.floor_under(x0, CRAB_W) - size(CRAB[frame]).1;
        each_pixel(CRAB[frame], false, |x, y, ch| {
            let c = match ch {
                b'R' => CRAB_SHELL,
                b'r' => CRAB_LEG,
                _ => EYE,
            };
            self.put(x0 + x, y0 + y, c);
        });
    }

    pub fn draw_bubbles(&mut self, t: f32) {
        let hi = BUBBLE.lerp(WHITE, 0.6);
        for i in 0..self.bubbles.len() {
            let Bubble { x, y, age, phase } = self.bubbles[i];
            let x = (x + (t * 0.09 + phase).sin() * 0.7).round() as i32;
            let y = y.round() as i32;
            match age {
                0..=29 => self.blend(x, y, BUBBLE, 0.8),
                30..=89 => {
                    self.blend(x, y, hi, 0.9);
                    self.blend(x + 1, y, BUBBLE, 0.55);
                    self.blend(x, y + 1, BUBBLE, 0.55);
                    self.blend(x + 1, y + 1, BUBBLE, 0.4);
                }
                _ => {
                    // A ring with a see-through middle and a glint.
                    for (dx, dy) in [(0, -1), (1, 0), (-1, 1), (0, 1), (1, 1), (-1, 0)] {
                        self.blend(x + dx, y + dy, BUBBLE, 0.65);
                    }
                    self.blend(x - 1, y - 1, hi, 0.5);
                    self.blend(x + 1, y - 1, BUBBLE, 0.4);
                    self.blend(x, y, BUBBLE, 0.12);
                }
            }
        }
    }

    pub fn draw_bits(&mut self) {
        for i in 0..self.bits.len() {
            let b = &self.bits[i];
            let (x, y, c) = (b.x.round() as i32, b.y.round() as i32, b.c);
            let fade = b.life as f32 / b.max.max(1) as f32;
            let a = match b.kind {
                // Glints twinkle as they turn.
                BitKind::Glint => {
                    fade * if (b.life / 3).is_multiple_of(2) {
                        1.0
                    } else {
                        0.6
                    }
                }
                BitKind::Drop => 0.9,
                BitKind::Speck => 0.55 * fade.min(1.0),
            };
            self.blend(x, y, c, a);
        }
    }
}
