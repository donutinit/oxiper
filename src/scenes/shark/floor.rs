//! What grows and lies on the floor of the sea: kelp in the haze, and on the
//! sand under the shark rocks, coral, sponges, anemones, seagrass and kelp,
//! with tufts passing right by the camera. Everything is dealt out by
//! hashing the position, so the floor goes on forever and is always the same
//! in the same place.

use super::art::{self, Sprite};
use super::sea::{Sea, View};
use super::water::ROCK;
use super::{Rgb, WORLD_H, hash, unit};

/// How fast the near reef and the tufts by the camera scroll, the shark's
/// plane being 1.
const MID_P: f64 = 0.6;
const FRONT_P: f64 = 1.35;
/// Width of the cells the floor's features are dealt by.
const CELL: i32 = 48;
/// How far a feature may reach out of its cell.
const REACH: i32 = 40;

const STALK: Rgb = Rgb(104, 96, 40);
const FROND: Rgb = Rgb(134, 142, 54);
const FROND_DARK: Rgb = Rgb(98, 110, 40);
const GRASS: [Rgb; 3] = [Rgb(70, 150, 84), Rgb(96, 170, 96), Rgb(52, 124, 72)];

/// What grows or lies in one cell of the floor.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Feature {
    Bare,
    Rocks,
    Garden,
    Meadow,
    Kelp,
}

impl Sea {
    /// What cell `c` of the shark's plane holds.
    fn feature(&self, c: i32) -> Feature {
        let r = unit(c, 51, self.seed);
        let kelp = if self.tropical { 0.0 } else { 0.12 };
        let garden = if self.tropical { 0.3 } else { 0.12 };
        let bands = [
            (0.2, Feature::Rocks),
            (garden, Feature::Garden),
            (0.18, Feature::Meadow),
            (kelp, Feature::Kelp),
        ];
        let mut acc = 0.0;
        for (p, f) in bands {
            acc += p;
            if r < acc {
                return f;
            }
        }
        Feature::Bare
    }

    /// Whether a holiday has a sunken spot on the floor's cell at local
    /// columns `x0..x0 + w`, which then stays bare.
    fn sunken_on(&self, x0: i32, w: i32) -> bool {
        self.ofrenda_on(x0, w) || self.portal_on(x0, w)
    }

    /// What grows and lies on the floor: kelp standing in the haze on the
    /// near reef, then, on the sand under the shark, cell by cell, rocks,
    /// gardens, meadows, kelp and the odd starfish.
    pub fn paint_floor(&mut self, v: View) {
        let s = self.seed;
        if !self.tropical && self.set.water {
            let shift = self.shift(v, MID_P);
            let lift = self.lift(v, MID_P);
            const STAND: i64 = 60;
            let (lo, hi) = (
                self.bx as i64 + shift - 20,
                (self.bx + self.bw) as i64 + shift + 20,
            );
            for c in lo.div_euclid(STAND) as i32..=hi.div_euclid(STAND) as i32 {
                if unit(c, 36, s) > 0.28 {
                    continue;
                }
                let n = 2 + (unit(c, 37, s) * 4.0) as i32;
                for k in 0..n {
                    let u = c * STAND as i32 + (unit(c, 40 + k, s) * STAND as f32) as i32;
                    let foot = 121 - lift;
                    let h = 40 + (unit(c, 50 + k, s) * 50.0) as i32;
                    let x = u - shift as i32;
                    self.kelp(x, foot, h, unit(c, 60 + k, s) * 6.3, 0.5);
                }
            }
        }
        let shift = self.shift(v, 1.0);
        let (lo, hi) = (
            self.bx as i64 + shift - REACH as i64,
            (self.bx + self.bw) as i64 + shift + REACH as i64,
        );
        for c in lo.div_euclid(CELL as i64) as i32..=hi.div_euclid(CELL as i64) as i32 {
            let x0 = c * CELL - shift as i32;
            // On a holiday the sand is cleared for its sunken spots: the
            // ofrendas, the portales.
            if self.sunken_on(x0, CELL) {
                continue;
            }
            match self.feature(c) {
                Feature::Rocks => self.rocks(c, x0),
                Feature::Garden => self.garden(c, x0),
                Feature::Meadow => self.meadow(c, x0, 0.0),
                Feature::Kelp => {
                    let n = 2 + (unit(c, 70, s) * 4.0) as i32;
                    for k in 0..n {
                        let x = x0 + 6 + (unit(c, 71 + k, s) * (CELL - 12) as f32) as i32;
                        let foot = self.bed(x as f32) + 1;
                        let h = 50 + (unit(c, 80 + k, s) * 50.0) as i32;
                        self.kelp(x, foot, h, unit(c, 90 + k, s) * 6.3, 0.1);
                    }
                    self.meadow(c, x0, 0.4);
                }
                Feature::Bare => {
                    if unit(c, 100, s) < 0.3 {
                        let x = x0 + (unit(c, 101, s) * CELL as f32) as i32;
                        let colors = self.reef_colors(c, 102);
                        let sprite = if self.tropical || unit(c, 103, s) < 0.5 {
                            art::STARFISH
                        } else {
                            art::URCHIN
                        };
                        self.piece(sprite, x, &colors, 0.1);
                    }
                }
            }
        }
    }

    /// A reef piece's colors, as the theme tints living things.
    fn reef_colors(&self, c: i32, salt: i32) -> [Rgb; 4] {
        let i = (hash(c, salt, self.seed) % art::REEF_COLORS.len() as u32) as usize;
        let mut cs = art::REEF_COLORS[i];
        for p in &mut cs {
            *p = p.lerp(self.pal.life, self.pal.life_k);
        }
        cs
    }

    /// Draws a reef piece standing on the sand at local column `x` (its middle).
    fn piece(&mut self, sprite: Sprite, x: i32, colors: &[Rgb; 4], haze: f32) {
        let w = sprite[0].len() as i32;
        let left = x - w / 2;
        let foot = self.bed(x as f32) + 1;
        let top = foot - sprite.len() as i32 + 1;
        for (j, row) in sprite.iter().enumerate() {
            let y = top + j as i32;
            for (i, ch) in row.bytes().enumerate() {
                let c = match ch {
                    b'a' => colors[0],
                    b'b' => colors[1],
                    b'c' => colors[2],
                    b'd' => colors[3],
                    _ => continue,
                };
                let c = self.shade(c, y, haze);
                self.put(left + i as i32, y, c);
            }
        }
    }

    /// A few boulders, lit from above, with patches of growth.
    fn rocks(&mut self, c: i32, x0: i32) {
        let s = self.seed;
        let n = 1 + (unit(c, 110, s) * 3.0) as i32;
        for k in 0..n {
            let cx = x0 + 8 + (unit(c, 111 + k, s) * (CELL - 16) as f32) as i32;
            let rw = 4 + (unit(c, 120 + k, s) * 10.0) as i32;
            let rh = (2 + (unit(c, 130 + k, s) * 8.0) as i32).min(rw);
            self.boulder(cx, rw, rh, hash(c, k, s));
            if unit(c, 140 + k, s) < 0.45 {
                let colors = self.reef_colors(c, 150 + k);
                let sprite = self.pick_reef(hash(c, 160 + k, s));
                let w = sprite[0].len() as i32;
                if w <= 2 * rw + 2 {
                    // On top of the boulder.
                    let foot = self.bed(cx as f32) - rh + 1;
                    self.piece_at(sprite, cx, foot, &colors, 0.08);
                }
            }
        }
    }

    fn boulder(&mut self, cx: i32, rw: i32, rh: i32, h: u32) {
        let foot = self.bed(cx as f32) + 1;
        let growth = if self.tropical {
            Rgb(196, 120, 140)
        } else {
            Rgb(84, 116, 66)
        };
        for dx in -rw..=rw {
            let f = dx as f32 / (rw as f32 + 0.5);
            let lump = (hash(cx + dx, 7, h) % 3) as i32 - 1;
            let height = (rh as f32 * (1.0 - f * f).max(0.0).sqrt()).round() as i32
                + if dx.abs() < rw { lump.max(0) } else { 0 };
            let top = foot - height;
            for y in top..=foot {
                // Lit from above: a bright rim on top, darker towards the sand,
                // and the sand's own shadow along the bottom.
                let up = (foot - y) as f32 / height.max(1) as f32 - 0.2 * f;
                let tone = if y == top {
                    1.2
                } else if y == foot {
                    0.55
                } else if up > 0.62 {
                    1.0
                } else if up > 0.3 {
                    0.84
                } else {
                    0.7
                };
                let speck = hash(cx + dx, y, h);
                let base = if up > 0.45 && speck.is_multiple_of(9) {
                    growth
                } else if speck.is_multiple_of(17) {
                    ROCK.scale(0.7)
                } else {
                    ROCK
                };
                let c = self.shade(base.scale(tone), y, 0.06);
                self.put(cx + dx, y, c);
            }
        }
        // Its shadow on the sand.
        for dx in -rw - 1..=rw + 2 {
            let x = cx + dx;
            let y = self.bed(x as f32);
            if let Some(p) = self.get(x, y) {
                self.put(x, y, p.scale(0.82));
            }
        }
    }

    fn pick_reef(&self, h: u32) -> Sprite {
        if self.tropical {
            [
                art::BRAIN,
                art::STAGHORN,
                art::SEA_FAN,
                art::TABLE,
                art::SPONGES,
            ][(h % 5) as usize]
        } else {
            [art::SPONGES, art::ANEMONE[0], art::STARFISH, art::URCHIN][(h % 4) as usize]
        }
    }

    /// Like [`Sea::piece`], with the foot at a given row.
    fn piece_at(&mut self, sprite: Sprite, x: i32, foot: i32, colors: &[Rgb; 4], haze: f32) {
        let w = sprite[0].len() as i32;
        let left = x - w / 2;
        let top = foot - sprite.len() as i32 + 1;
        for (j, row) in sprite.iter().enumerate() {
            let y = top + j as i32;
            for (i, ch) in row.bytes().enumerate() {
                let c = match ch {
                    b'a' => colors[0],
                    b'b' => colors[1],
                    b'c' => colors[2],
                    b'd' => colors[3],
                    _ => continue,
                };
                let c = self.shade(c, y, haze);
                self.put(left + i as i32, y, c);
            }
        }
    }

    /// A cluster of coral, sponges or anemones.
    fn garden(&mut self, c: i32, x0: i32) {
        let s = self.seed;
        let n = 2 + (unit(c, 170, s) * 3.0) as i32;
        let sway = (self.time() * 0.8 + c as f32).sin() > 0.0;
        for k in 0..n {
            let x = x0 + 6 + (unit(c, 171 + k, s) * (CELL - 12) as f32) as i32;
            let h = hash(c, 180 + k, s);
            let sprite = if !self.tropical && h % 4 == 1 {
                art::ANEMONE[sway as usize]
            } else {
                self.pick_reef(h)
            };
            let colors = self.reef_colors(c, 190 + k);
            self.piece(sprite, x, &colors, 0.1);
        }
        if unit(c, 199, s) < 0.5 {
            self.meadow(c, x0, 0.0);
        }
    }

    /// Seagrass swaying in the current.
    fn meadow(&mut self, c: i32, x0: i32, sparse: f32) {
        let s = self.seed;
        let time = self.time();
        let width = 14 + (unit(c, 200, s) * 26.0) as i32;
        let start = x0 + (unit(c, 201, s) * (CELL - width / 2) as f32) as i32;
        let n = ((width as f32 * 0.8) * (1.0 - sparse)) as i32;
        let current = 0.9 + 0.6 * (time * 0.13).sin();
        for b in 0..n {
            let h = hash(c, 300 + b, s);
            let x = start + (h % width as u32) as i32;
            let tall = 3 + ((h >> 8) % 7) as i32;
            let color = GRASS[((h >> 16) % 3) as usize];
            let foot = self.bed(x as f32) + 1;
            let sway = (time * 1.1 + x as f32 * 0.35).sin() * current * tall as f32 / 4.0;
            for j in 0..tall {
                let f = j as f32 / tall as f32;
                let y = foot - j;
                let c = self.shade(color, y, 0.08);
                self.put(x + (sway * f * f).round() as i32, y, c);
            }
        }
    }

    /// A kelp stalk rising from `(x, foot)`, swaying, with fronds on
    /// alternate sides and a float at the top.
    fn kelp(&mut self, x: i32, foot: i32, h: i32, phase: f32, haze: f32) {
        let time = self.time();
        let current = 2.2 + 1.4 * (time * 0.11).sin();
        let mut prev = x;
        for j in 0..h {
            let f = j as f32 / h as f32;
            let sway =
                (time * 0.5 + phase - j as f32 * 0.045).sin() * 3.0 * f.powf(1.3) + current * f * f;
            let sx = x + sway.round() as i32;
            let y = foot - j;
            if self.bed_painted(sx).is_some_and(|b| y >= b) {
                // Behind the sand.
                prev = sx;
                continue;
            }
            let c = self.shade(STALK, y, haze);
            self.put(sx, y, c);
            // Keep the stalk joined where it leans.
            if (sx - prev).abs() > 1 {
                self.put((sx + prev) / 2, y, c);
            }
            prev = sx;
            if j > 3 && j % 4 == 0 {
                let side = if (j / 4) % 2 == 0 { 1 } else { -1 };
                let len = 2 + ((j as u32).wrapping_mul(2_654_435_761) >> 30) as i32;
                let droop = (time * 0.9 + phase + j as f32 * 0.2).sin();
                for k in 1..=len {
                    let ly = y - (k + 1) / 2 + if droop > 0.3 && k == len { 1 } else { 0 };
                    let lc = if k == len { FROND_DARK } else { FROND };
                    let lc = self.shade(lc, ly, haze);
                    self.put(sx + side * k, ly, lc);
                }
            }
        }
        // The float.
        let top = foot - h;
        let c = self.shade(FROND, top, haze);
        self.put(prev, top, c);
        self.put(prev + 1, top, c);
    }

    /// Dark tufts close to the camera, passing in front of everything.
    pub fn paint_front(&mut self, v: View) {
        let shift = self.shift(v, FRONT_P);
        let s = self.seed;
        let time = self.time();
        const SPAN: i64 = 90;
        let (lo, hi) = (
            self.bx as i64 + shift - 12,
            (self.bx + self.bw) as i64 + shift + 12,
        );
        let blade = Rgb(40, 82, 58);
        for c in lo.div_euclid(SPAN) as i32..=hi.div_euclid(SPAN) as i32 {
            if unit(c, 41, s) > 0.45 {
                continue;
            }
            let u0 = c * SPAN as i32 + (unit(c, 42, s) * SPAN as f32) as i32;
            let n = 5 + (unit(c, 43, s) * 9.0) as i32;
            for b in 0..n {
                let u = u0 + b * 2 + (hash(c, b, s) % 2) as i32;
                let h = 6 + (hash(c, b + 50, s) % 10) as i32;
                let x = u - shift as i32;
                let foot = WORLD_H + 1;
                let sway = (time * 0.9 + u as f32 * 0.3).sin() * 1.6;
                for j in 0..h {
                    let f = j as f32 / h as f32;
                    let dx = (sway * f * f).round() as i32;
                    let y = foot - j;
                    let c = self.shade(blade, y, 0.0).scale(0.85);
                    self.put(x + dx, y, c);
                }
            }
        }
    }
}
