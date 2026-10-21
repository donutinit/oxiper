//! Día de Muertos in the sea: Janitzio's night on the water.
//!
//! Night falls over the sea: the sky goes from noche to the last glow of
//! the sunset with stars coming out, the water darkens under the moon, and
//! the ripples catch candlelight. Veladoras in colored glass float on
//! rafts of cempasúchil, flickering, haloed and mirrored in the water, as
//! on the lake at Janitzio, with marigold petals drifting among them. Now
//! and then a petal soaks through and sinks, tumbling, to rest on the sand
//! for a while. Marigolds, terciopelo and nube bloom on the floor; the
//! castle's windows and door glow with candles inside, candles stand in
//! its crenels, papel picado hangs between its towers and marigolds crowd
//! its foot. The rest is in the other files: alebrijes join the schools
//! and swim on their own (`life.rs`, `art.rs`), and sharks and orcas wear
//! calavera paint (`art.rs`).
//!
//! Everything here is drawn from its own generator, so the sea of an
//! ordinary day is untouched, and how much there is follows the holiday's
//! level: a few candles on October 28, the lake full of them on November 2.

use std::f32::consts::TAU;

use super::art::{CASTLE, rgb, size};
use super::sea::{Sea, hash};
use super::tank::mul;
use super::{Dice, Rng};
use crate::color::Color;
use crate::fiesta::{Fiesta, Kind};

pub const HUESO: Color = rgb(244, 238, 226);
pub const ROSA: Color = rgb(236, 30, 136);
pub const TURQUESA: Color = rgb(0, 186, 196);
pub const AMARILLO: Color = rgb(255, 214, 20);
/// Cempasúchil: the flower, its light and its shade.
pub const CEMPASUCHIL: [Color; 3] = [rgb(255, 140, 0), rgb(255, 176, 40), rgb(214, 98, 0)];
const TERCIOPELO: [Color; 2] = [rgb(176, 0, 72), rgb(226, 40, 112)];
const NUBE: Color = rgb(250, 250, 244);
const LEAF: Color = rgb(56, 124, 52);
const FLAME: Color = rgb(255, 214, 120);
const FLAME_EDGE: Color = rgb(255, 150, 40);
/// A flame's light deep in its reflection.
const EMBER: Color = rgb(232, 110, 30);
const STAR: Color = rgb(255, 246, 220);
const TWINE: Color = rgb(200, 190, 170);
/// The inside of the castle, lit by a candle.
const HEARTH: Color = rgb(90, 40, 24);
/// A crescent: offsets from its corner and how bright.
const MOON: [(i32, i32, f32); 8] = [
    (1, 0, 0.6),
    (2, 0, 0.9),
    (0, 1, 0.9),
    (1, 1, 0.4),
    (0, 2, 0.95),
    (1, 2, 0.4),
    (1, 3, 0.6),
    (2, 3, 0.9),
];

/// A veladora floating on a raft of marigolds.
struct Candle {
    x: f32,
    vx: f32,
    phase: f32,
    /// Its glass, one of the papel picado colors.
    glass: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Drift {
    /// On the surface, carried along.
    Float,
    /// Soaked through and sinking.
    Sink,
    /// On the sand, for `life` more ticks.
    Rest,
}

struct Petal {
    x: f32,
    y: f32,
    vy: f32,
    phase: f32,
    life: u16,
    drift: Drift,
}

#[derive(Clone, Copy)]
enum Flower {
    Cempasuchil,
    Terciopelo,
    Nube,
}

/// A flower on the floor, standing on the sand at (`x`, `y`).
struct Bloom {
    x: i32,
    y: i32,
    kind: Flower,
}

/// The holiday's dress for the sea.
pub struct Fest {
    pub on: bool,
    /// How far the holiday has come, 0 to 1.
    pub level: f32,
    /// The holiday's own dice, so an ordinary sea's are left alone.
    pub rng: Rng,
    candles: Vec<Candle>,
    petals: Vec<Petal>,
    blooms: Vec<Bloom>,
    /// Petals on the surface, and the most sinking or resting at once.
    floating: usize,
    most: usize,
    /// The current on the surface, pixels per tick.
    current: f32,
}

impl Fest {
    pub fn new(fiesta: Fiesta, seed: u64) -> Fest {
        let on = fiesta.is(Kind::Muertos);
        Fest {
            on,
            level: if on {
                fiesta.level.clamp(0.0, 1.0)
            } else {
                0.0
            },
            rng: Rng::with_seed(seed ^ 0x6d75_6572_746f_7321),
            candles: Vec::new(),
            petals: Vec::new(),
            blooms: Vec::new(),
            floating: 0,
            most: 0,
            current: 0.0,
        }
    }

    /// How many candles, petals and flowers there are.
    #[cfg(test)]
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.candles.len(), self.petals.len(), self.blooms.len())
    }
}

/// `x` brought back into `lo..hi` from either side.
fn wrap(x: f32, lo: f32, hi: f32) -> f32 {
    if x < lo {
        x + (hi - lo)
    } else if x >= hi {
        x - (hi - lo)
    } else {
        x
    }
}

/// How bright a flame burns at tick `t`, about 0.5 to 1.
fn flicker(t: f32, phase: f32) -> f32 {
    0.78 + 0.22 * (t * 0.9 + phase).sin() * (t * 0.37 + phase * 2.0).sin().abs().max(0.4)
}

impl Sea {
    /// Lays out the holiday: candles and petals on the water, flowers on the
    /// floor and around the castle.
    pub fn dress(&mut self) {
        let (w, surf) = (self.w, self.surf);
        let wf = w as f32;
        let sand = &self.sand_top;
        let floor = |x: i32| sand[x.clamp(0, w - 1) as usize];
        let f = &mut self.fest;
        let lvl = f.level;
        let rng = &mut f.rng;
        f.current = rng.sign() * rng.rangef(0.012, 0.025);

        if surf >= 5 {
            let n = ((wf / 16.0) * (0.45 + 0.55 * lvl)).round().max(1.0) as usize;
            for k in 0..n {
                f.candles.push(Candle {
                    x: (k as f32 + rng.rangef(0.2, 0.8)) * wf / n as f32,
                    vx: f.current * rng.rangef(0.8, 1.2),
                    phase: rng.rangef(0.0, TAU),
                    glass: rng.usize(..5),
                });
            }
        }

        f.floating = ((wf * (0.06 + 0.14 * lvl)).round() as usize).max(2);
        f.most = f.floating + (wf * (0.03 + 0.09 * lvl)) as usize + 2;
        f.petals.reserve(f.most + 1);
        for _ in 0..f.floating {
            f.petals.push(Petal {
                x: rng.rangef(0.0, wf),
                y: surf as f32,
                vy: 0.0,
                phase: rng.rangef(0.0, TAU),
                life: 0,
                drift: Drift::Float,
            });
        }

        let n = ((wf / 13.0) * (0.35 + 0.65 * lvl)).round().max(1.0) as usize;
        for _ in 0..n {
            let x = rng.range(1, w - 1);
            let roll = rng.f32();
            let kind = if roll < 0.6 {
                Flower::Cempasuchil
            } else if roll < 0.85 {
                Flower::Terciopelo
            } else {
                Flower::Nube
            };
            f.blooms.push(Bloom {
                x,
                y: floor(x),
                kind,
            });
        }
        // Marigolds crowd the castle's foot, terciopelo at its door.
        if let Some((x0, _)) = self.castle {
            let cw = size(CASTLE).0;
            let foot = [
                (-5, Flower::Nube),
                (-2, Flower::Cempasuchil),
                (2, Flower::Cempasuchil),
                (4, Flower::Terciopelo),
                (10, Flower::Terciopelo),
                (12, Flower::Cempasuchil),
                (cw + 1, Flower::Cempasuchil),
                (cw + 4, Flower::Nube),
            ];
            for (dx, kind) in foot {
                let x = x0 + dx;
                if x >= 0 && x < w {
                    f.blooms.push(Bloom {
                        x,
                        y: floor(x),
                        kind,
                    });
                }
            }
        }
    }

    /// A tick of the holiday: the candles and petals drift, and petals
    /// sink and settle.
    pub fn fest_tick(&mut self) {
        let (w, surf) = (self.w, self.surf as f32);
        let wf = w as f32;
        let t = self.t as f32;
        let sand = &self.sand_top;
        let f = &mut self.fest;
        for c in &mut f.candles {
            c.x = wrap(c.x + c.vx, -4.0, wf + 4.0);
        }
        let current = f.current;
        f.petals.retain_mut(|p| match p.drift {
            Drift::Float => {
                let eddy = (t * 0.03 + p.phase).sin() * 0.01;
                p.x = wrap(p.x + current * 1.3 + eddy, -2.0, wf + 2.0);
                true
            }
            Drift::Sink => {
                p.y += p.vy;
                p.x += (t * 0.06 + p.phase).sin() * 0.06;
                let bed = sand[(p.x.round() as i32).clamp(0, w - 1) as usize] as f32 - 1.0;
                if p.y >= bed {
                    p.y = bed;
                    p.drift = Drift::Rest;
                }
                true
            }
            Drift::Rest => {
                p.life = p.life.saturating_sub(1);
                p.life > 0
            }
        });
        // Now and then one soaks through and goes down.
        let rate = (0.01 + 0.035 * f.level) * wf / 100.0;
        if f.petals.len() < f.most && f.rng.chance(rate) {
            let rng = &mut f.rng;
            f.petals.push(Petal {
                x: rng.rangef(0.0, wf),
                y: surf + 1.0,
                vy: rng.rangef(0.03, 0.07),
                phase: rng.rangef(0.0, TAU),
                life: rng.range(300, 1000) as u16,
                drift: Drift::Sink,
            });
        }
    }

    /// Where a candle's raft sits: its column and the row just above the
    /// water, bobbing on the ripples.
    fn raft(&self, i: usize, t: f32) -> (i32, i32, f32) {
        let c = &self.fest.candles[i];
        let bob = if (t * 0.05 + c.phase).sin() > 0.55 {
            1
        } else {
            0
        };
        (c.x.round() as i32, self.surf - 1 - bob, c.phase)
    }

    /// Adds light `c` by `a` to what is there, as a flame lights the night;
    /// over see-through water it blends instead.
    fn glow(&mut self, x: i32, y: i32, c: Color, a: f32) {
        if x < 0 || y < 0 || x >= self.w || y >= self.ph {
            return;
        }
        match (self.scr.get(x, y), c) {
            (Color::Rgb(r, g, b), Color::Rgb(cr, cg, cb)) => {
                let add = |u: u8, v: u8| (u as f32 + v as f32 * a).min(255.0) as u8;
                self.put(x, y, Color::Rgb(add(r, cr), add(g, cg), add(b, cb)));
            }
            _ => self.blend(x, y, c, a),
        }
    }

    /// Stars in the sky, and the candles' glow on the air and on the water.
    pub fn paint_night(&mut self, t: f32) {
        let (w, top) = (self.w, self.surf);
        if self.set.water && top > 2 {
            for y in 0..top * 2 / 3 {
                for x in 0..w {
                    let h = hash(x, y, 0x5eed_57a2);
                    if !h.is_multiple_of(53) {
                        continue;
                    }
                    let tw = (t * 0.05 + (h >> 8) as f32).sin() * 0.5 + 0.5;
                    let a = (0.3 + 0.65 * tw) * (1.0 - y as f32 / top as f32);
                    self.blend(x, y, STAR, a);
                }
            }
            // A thin moon, when the sky has room for it.
            if top >= 8 {
                let mx = self.w * 4 / 5;
                for (dx, dy, a) in MOON {
                    self.blend(mx + dx, 1 + dy, STAR, a);
                }
            }
        }
        // Candles far across the lake, more as the holiday comes.
        if top > 2 {
            let every = (9.0 - 5.0 * self.fest.level) as u32;
            for x in 0..w {
                let h = hash(x, 77, 0x1a9e_0001);
                if !h.is_multiple_of(every) {
                    continue;
                }
                let a = 0.35 + 0.4 * flicker(t, (h >> 8) as f32);
                self.blend(x, top - 2, FLAME_EDGE, a);
            }
        }
        for i in 0..self.fest.candles.len() {
            let (cx, base, ph) = self.raft(i, t);
            let glow = flicker(t, ph);
            // A halo round the flame.
            let fy = base - 3;
            for dy in -3..=3 {
                for dx in -4..=4 {
                    let d = ((dx * dx) as f32 * 0.6 + (dy * dy) as f32).sqrt();
                    if d <= 3.6 {
                        let a = 0.3 * (1.0 - d / 3.8) * glow;
                        self.glow(cx + dx, fy + dy, FLAME_EDGE, a);
                    }
                }
            }
            // Its reflection: a column of glints on the ripples, each
            // ripple a dash that wavers, longer and dimmer with depth.
            const DASH: [i32; 7] = [1, 0, 1, 2, 1, 2, 3];
            for (k, &half) in DASH.iter().enumerate() {
                let kf = k as f32;
                let wave = (t * 0.13 + kf * 2.3 + ph).sin();
                if wave < -0.75 {
                    continue;
                }
                let xo = (wave * 0.8).round() as i32;
                let c = FLAME
                    .lerp(FLAME_EDGE, kf / 3.0)
                    .lerp(EMBER, (kf - 3.0).max(0.0) / 4.0);
                let c = c.lerp(FLAME, (glow - 0.8).max(0.0));
                // Glints rather than a wash: whole pixels that come and go,
                // fewer the deeper they are.
                let tick = (t * 0.25 + ph) as i32;
                for dx in -half..=half {
                    let (x, y) = (cx + xo + dx, top + 1 + k as i32);
                    if hash(x, y + tick * 7, 0x6c75_7a00) % 10 < k as u32 + dx.unsigned_abs() * 2 {
                        continue;
                    }
                    self.blend(x, y, c, if dx == 0 { 0.92 } else { 0.8 });
                }
            }
        }
    }

    /// The flowers on the floor, the petals resting there, and the castle
    /// dressed for the night.
    pub fn draw_altar(&mut self, t: f32) {
        for i in 0..self.fest.blooms.len() {
            let Bloom { x, y, kind } = self.fest.blooms[i];
            self.bloom(x, y, kind);
        }
        for i in 0..self.fest.petals.len() {
            let p = &self.fest.petals[i];
            if p.drift == Drift::Rest {
                let (x, y) = (p.x.round() as i32, p.y.round() as i32);
                let a = (p.life as f32 / 60.0).min(0.9);
                let c = if p.phase > 3.0 {
                    CEMPASUCHIL[2]
                } else {
                    CEMPASUCHIL[0]
                };
                self.blend(x, y, c, a);
            }
        }
        if let Some((x0, y0)) = self.castle {
            self.dress_castle(x0, y0, t);
        }
    }

    fn bloom(&mut self, x: i32, y: i32, kind: Flower) {
        let [c0, c1, c2] = CEMPASUCHIL;
        match kind {
            Flower::Cempasuchil => {
                // A ruffled pompom on a short stem.
                self.put(x, y - 4, c1);
                self.put(x - 1, y - 3, c0);
                self.put(x, y - 3, c1);
                self.put(x + 1, y - 3, c0);
                self.put(x - 2, y - 3, c2);
                self.put(x + 2, y - 3, c2);
                self.put(x - 1, y - 2, c2);
                self.put(x, y - 2, c0);
                self.put(x + 1, y - 2, c2);
                self.put(x, y - 1, LEAF);
                self.put(x + 1, y - 1, mul(LEAF, 0.8));
            }
            Flower::Terciopelo => {
                let [t0, t1] = TERCIOPELO;
                self.put(x, y - 3, t1);
                self.put(x - 1, y - 2, t0);
                self.put(x, y - 2, t1);
                self.put(x + 1, y - 2, t0);
                self.put(x, y - 1, LEAF);
                self.put(x - 1, y - 1, mul(LEAF, 0.8));
            }
            Flower::Nube => {
                self.put(x, y - 1, LEAF);
                self.put(x - 1, y - 2, NUBE);
                self.put(x + 1, y - 2, NUBE);
                self.put(x, y - 3, NUBE);
                self.put(x + 1, y - 1, mul(NUBE, 0.85));
            }
        }
    }

    /// Candlelight in the castle's windows and door, candles in its
    /// crenels and papel picado between its towers.
    fn dress_castle(&mut self, x0: i32, y0: i32, t: f32) {
        let glow = flicker(t, 1.3);
        let lit = |k: f32| HEARTH.lerp(FLAME_EDGE, k * glow);
        for (wx, ph) in [(2, 0.0), (12, 2.1)] {
            let g = flicker(t, ph);
            self.put(
                x0 + wx,
                y0 + 5,
                FLAME.lerp(FLAME_EDGE, 0.5).lerp(HEARTH, 1.0 - g),
            );
            self.put(x0 + wx, y0 + 6, FLAME_EDGE.lerp(HEARTH, 1.1 - g));
        }
        // The door, a veladora burning just inside it.
        for (dx, dy) in [(1, 9), (2, 9), (3, 9)] {
            self.put(x0 + 4 + dx, y0 + dy, lit(0.15));
        }
        for dy in 10..=12 {
            for dx in 5..=9 {
                let d = ((dx - 7) as f32).abs() + (12 - dy) as f32 * 0.6;
                self.put(x0 + dx, y0 + dy, lit((0.75 - d * 0.22).max(0.1)));
            }
        }
        self.put(x0 + 7, y0 + 12, HUESO);
        self.put(x0 + 7, y0 + 11, FLAME.lerp(FLAME_EDGE, 1.0 - glow));
        // Candles in the crenels.
        for (cx, ph) in [(6, 0.7), (8, 2.9)] {
            self.put(x0 + cx, y0 + 6, HUESO);
            let g = flicker(t, ph);
            self.put(x0 + cx, y0 + 5, FLAME.lerp(FLAME_EDGE, 1.0 - g));
            if g > 0.8 {
                self.blend(x0 + cx, y0 + 4, FLAME_EDGE, 0.5);
            }
        }
        // Papel picado on a twine sagging between the roofs.
        for x in 4..=10 {
            let sag = if (6..=8).contains(&x) { 2 } else { 1 };
            self.put(x0 + x, y0 + sag, TWINE);
            if x % 2 == 1 {
                let c = self.pal.papel[(x as usize / 2) % 5];
                self.put(x0 + x, y0 + sag + 1, c);
                self.put(x0 + x, y0 + sag + 2, mul(c, 0.85));
            }
        }
    }

    /// The petals going down through the water, tumbling.
    pub fn draw_sinking(&mut self, t: f32) {
        for i in 0..self.fest.petals.len() {
            let p = &self.fest.petals[i];
            if p.drift != Drift::Sink {
                continue;
            }
            let (x, y) = (p.x.round() as i32, p.y.round() as i32);
            // Flat to the light, then edge on in its shade.
            if ((t * 0.1 + p.phase) as i32) & 1 == 0 {
                self.blend(x, y, CEMPASUCHIL[1], 0.9);
                self.blend(x + 1, y, CEMPASUCHIL[0], 0.8);
            } else {
                self.blend(x, y, CEMPASUCHIL[2], 0.85);
            }
        }
    }

    /// The candles on their rafts, and the petals floating between them.
    pub fn draw_veladoras(&mut self, t: f32) {
        let [c0, c1, c2] = CEMPASUCHIL;
        for i in 0..self.fest.petals.len() {
            let p = &self.fest.petals[i];
            if p.drift == Drift::Float {
                let (x, y) = (p.x.round() as i32, p.y.round() as i32);
                let big = p.phase > 3.5;
                self.put(x, y, if big { c0 } else { c1 });
                if big {
                    self.put(x + 1, y, c2);
                }
            }
        }
        for i in 0..self.fest.candles.len() {
            let (cx, base, ph) = self.raft(i, t);
            let glass = self.pal.papel[self.fest.candles[i].glass];
            let glow = flicker(t, ph);
            for (dx, c) in [(-2, c2), (-1, c0), (0, c1), (1, c0), (2, c2)] {
                self.put(cx + dx, base, c);
            }
            self.put(cx - 1, base - 1, mul(glass, 0.7));
            self.put(cx, base - 1, glass.lerp(FLAME, 0.35 * glow));
            self.put(cx + 1, base - 1, mul(glass, 0.7));
            self.put(cx - 1, base - 2, glass);
            self.put(cx, base - 2, glass.lerp(FLAME, 0.7 * glow));
            self.put(cx + 1, base - 2, glass);
            self.put(cx, base - 3, FLAME.lerp(FLAME_EDGE, 1.0 - glow));
            let sway = (t * 0.21 + ph).sin();
            let tip = if sway > 0.6 {
                1
            } else if sway < -0.6 {
                -1
            } else {
                0
            };
            if glow > 0.62 {
                self.put(cx + tip, base - 4, FLAME_EDGE);
            }
        }
    }
}
