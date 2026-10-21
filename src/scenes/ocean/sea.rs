//! The sea as a whole: what it holds, a tick of it, the clock of the visits
//! and the picture.

use std::f32::consts::TAU;

use super::art::{Sprite, each_pixel, size};
use super::life::{Group, Swimmer};
use super::tank::{Bit, Bubble, Crab, Weed};
use super::{Dice, Pal, Rng, Settings};
use crate::color::Color;
use crate::render::Pixels;

pub struct Sea {
    /// Size in pixels: the columns, and twice the rows.
    pub w: i32,
    pub ph: i32,
    /// Pixel rows of sky above the water; 0 without a surface strip.
    pub surf: i32,
    pub set: Settings,
    pub pal: Pal,
    pub scr: Pixels,
    /// The terminal's background, when known, for blending over see-through water.
    pub bg: Option<Color>,
    /// Ticks so far.
    pub t: u32,
    /// Where the sand starts in each column, and the highest of those.
    pub sand_top: Vec<i32>,
    pub floor: i32,
    /// Sand, rocks and ornaments, laid over the water every frame.
    pub scenery: Vec<Option<Color>>,
    pub rays: Vec<f32>,
    pub weeds: Vec<Weed>,
    /// Where bubbles stream from.
    pub vents: Vec<(i32, i32)>,
    pub crab: Option<Crab>,
    pub bubbles: Vec<Bubble>,
    pub bits: Vec<Bit>,
    pub swim: Vec<Swimmer>,
    pub groups: Vec<Group>,
    pub next_id: u32,
    /// Animals eaten so far, and how many of them were sharks.
    pub eaten: u32,
    pub sharks_taken: u32,
    /// Real seconds until the next visit and the next mantas.
    pub visit_in: f32,
    pub manta_in: f32,
}

#[inline]
pub fn hash(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = (x as u32).wrapping_mul(374_761_393) ^ (y as u32).wrapping_mul(668_265_263) ^ seed;
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

impl Sea {
    pub fn new(cols: u16, rows: u16, rng: &mut Rng, pal: Pal, set: Settings) -> Sea {
        let (cols, rows) = (cols.max(1), rows.max(1));
        let (w, ph) = (cols as i32, 2 * rows as i32);
        let surf = if set.surface && ph >= 40 {
            (ph as f32 * 0.12).round() as i32
        } else {
            0
        };
        let sand_h = (ph as f32 / 7.0).max(2.0);
        let base = ph as f32 - sand_h;
        let (p1, p2) = (rng.rangef(0.0, TAU), rng.rangef(0.0, TAU));
        let sand_top: Vec<i32> = (0..w)
            .map(|x| {
                let x = x as f32;
                let y = base + (x * 0.05 + p1).sin() * 1.6 + (x * 0.13 + p2).sin() * 0.7;
                (y.round() as i32).min(ph - 1).max(surf + 1)
            })
            .collect();
        let floor = sand_top.iter().copied().min().unwrap_or(ph);
        let n = (w * ph) as usize;
        let visits = set.visits.max(0.1);
        let mut sea = Sea {
            w,
            ph,
            surf,
            set,
            pal,
            scr: Pixels::new(cols, rows),
            bg: None,
            t: 0,
            sand_top,
            floor,
            scenery: vec![None; n],
            rays: vec![0.0; (w + (ph - surf) / 3 + 1).max(1) as usize],
            weeds: Vec::new(),
            vents: Vec::new(),
            crab: None,
            bubbles: Vec::new(),
            bits: Vec::new(),
            swim: Vec::new(),
            groups: Vec::new(),
            next_id: 1,
            eaten: 0,
            sharks_taken: 0,
            // Something comes by soon after the start, then every so often.
            visit_in: rng.rangef(3.0, 8.0) / visits,
            manta_in: rng.rangef(6.0, 16.0) / visits,
        };
        sea.build_scenery(rng);
        sea.plant(rng);
        if sea.lively() {
            sea.populate(rng);
        }
        sea
    }

    /// Whether the sea is big enough for anything to swim in it.
    pub fn lively(&self) -> bool {
        self.w >= 12 && self.floor - self.surf >= 10
    }

    /// One tick: tanim's tank runs at 20 a second.
    pub fn step(&mut self, rng: &mut Rng) {
        self.t = self.t.wrapping_add(1);
        if self.lively() {
            self.life(rng);
        }
        self.tank(rng);
    }

    /// The wall clock: when the big animals come by. Returns whether
    /// something was sent.
    pub fn clocks(&mut self, real: f32, rng: &mut Rng) -> bool {
        if self.set.visits <= 0.0 || !self.lively() {
            return false;
        }
        let mut sent = false;
        self.visit_in -= real;
        if self.visit_in <= 0.0 {
            sent |= self.visit(rng);
            self.visit_in = rng.rangef(15.0, 40.0) / self.set.visits;
        }
        self.manta_in -= real;
        if self.manta_in <= 0.0 {
            if self.set.mantas {
                sent |= self.send_mantas(rng);
            }
            self.manta_in = rng.rangef(25.0, 60.0) / self.set.visits;
        }
        sent
    }

    pub fn render(&mut self, bg: Option<Color>) {
        self.bg = bg;
        self.scr.clear();
        let t = self.t as f32;
        self.paint_water(t);
        self.paint_surface(t);
        self.draw_weeds(t);
        self.draw_crab();
        self.draw_life(t);
        self.draw_bits();
        self.draw_bubbles(t);
    }

    #[inline]
    pub fn put(&mut self, x: i32, y: i32, c: Color) {
        self.scr.set(x, y, c);
    }

    /// Mixes `c` into what is there by `a`. Over see-through water it mixes
    /// with the terminal's background, or without one shows only when strong.
    pub fn blend(&mut self, x: i32, y: i32, c: Color, a: f32) {
        if x < 0 || y < 0 || x >= self.w || y >= self.ph {
            return;
        }
        let under = match self.scr.get(x, y) {
            Color::Reset => match self.bg {
                Some(b) => b,
                None => {
                    if a >= 0.5 {
                        self.put(x, y, c);
                    }
                    return;
                }
            },
            u => u,
        };
        self.put(x, y, under.lerp(c, a));
    }

    /// Draws `s` centred on (`cx`, `cy`) facing `dir`, its columns sheared
    /// by `slope` (nose down when positive): whole pixels moved, none lost.
    pub fn sprite(
        &mut self,
        s: Sprite,
        (cx, cy): (f32, f32),
        dir: f32,
        slope: f32,
        paint: impl Fn(u8) -> Color,
    ) {
        let (sw, sh) = size(s);
        let half = (sw - 1) as f32 * 0.5;
        let x0 = (cx - half).round() as i32;
        let y0 = (cy - (sh - 1) as f32 * 0.5).round() as i32;
        let flip = dir < 0.0;
        each_pixel(s, flip, |x, y, ch| {
            let ahead = if flip {
                half - x as f32
            } else {
                x as f32 - half
            };
            let dy = (ahead * slope).round() as i32;
            self.scr.set(x0 + x, y0 + y + dy, paint(ch));
        });
    }

    /// Draws `s` centred on (`cx`, `cy`) facing `dir`, turned nose up by `a`
    /// radians: a manta's loop.
    pub fn sprite_turned(
        &mut self,
        s: Sprite,
        (cx, cy): (f32, f32),
        dir: f32,
        a: f32,
        paint: impl Fn(u8) -> Color,
    ) {
        let (sw, sh) = size(s);
        let (hw, hh) = ((sw - 1) as f32 * 0.5, (sh - 1) as f32 * 0.5);
        let r = (hw * hw + hh * hh).sqrt().ceil() as i32 + 1;
        let (sn, cs) = a.sin_cos();
        let (ix, iy) = (cx.round() as i32, cy.round() as i32);
        for oy in -r..=r {
            for ox in -r..=r {
                // Back from the screen into the drawing, facing right.
                let (fx, fy) = (ox as f32 * dir, oy as f32);
                let u = (fx * cs - fy * sn + hw).round();
                let v = (fx * sn + fy * cs + hh).round();
                if u < 0.0 || v < 0.0 || u >= sw as f32 || v >= sh as f32 {
                    continue;
                }
                let ch = s[v as usize].as_bytes()[u as usize];
                if ch != b'.' {
                    self.scr.set(ix + ox, iy + oy, paint(ch));
                }
            }
        }
    }
}
