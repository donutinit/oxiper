//! The fire at the fiestas: what the holidays other than Día de Muertos put
//! on aafire's flames.
//!
//! Each holiday is a [`Piece`]: the Navidad's fogata at a posada with an olla
//! of ponche steaming over it (`navidad.rs`), the fogón of the Grito with its
//! pozolera, its comal and its luces de bengala (`patrias.rs`), the tamalera
//! of the Candelaria with its jarro of atole (`candelaria.rs`), and the
//! cartonería Judas of the quema (`judas.rs`). A
//! piece sets its things out for the screen (`layout`), says where the fire's
//! fuel burns (`fuel`) and how high its flames reach, and draws: first what
//! stays behind the flames (steam), then the vessels and the like, over which
//! the flames lick, then what stands in front of them (the stones of the
//! hearth, the farolitos). The flames themselves stay aafire's own.
//!
//! The `ascii` style draws the same pictures with characters picked by
//! brightness, the way aalib would draw them.
//!
//! Everything here rolls its own dice (seeded from the scene's seed), so the
//! fire's random numbers on ordinary days are untouched.

mod candelaria;
mod judas;
mod navidad;
mod patrias;

use super::{FP, PAD};
use crate::color::{Color, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{Canvas, Pixels};
use crate::scene::Ctx;

/// Mixed into the scene's seed for the holidays' own dice.
const SALT: u64 = 0x666f_676f_6e21_f1e5;
/// Heat from which flames lick over a vessel (0 to 255).
const LICK_MIN: u32 = 96;

/// Where aafire's fuel burns, in simulation pixels (two per column).
pub(super) struct Fuel {
    pub lo: usize,
    pub hi: usize,
    /// How much fuel, relative to aafire's.
    pub k: f32,
    /// The fuel envelope's slope near its edges.
    pub taper: u32,
}

impl Fuel {
    /// A bed of fuel from column `lo` to column `hi` (cut to the screen's
    /// `sw` simulation pixels), tapering off over its outer sixth like
    /// aafire's does over the screen's fifth.
    pub fn span(lo: i32, hi: i32, sw: usize, k: f32) -> Fuel {
        let lo = (2 * lo).clamp(0, sw as i32) as usize;
        let hi = (2 * hi).clamp(lo as i32, sw as i32) as usize;
        let taper = (256.0 / ((hi - lo).max(1) as f32 * 0.14)).ceil().max(4.0) as u32;
        Fuel { lo, hi, k, taper }
    }
}

/// The flames, as the fire has them this frame.
pub(super) struct Flames<'a> {
    pub heat: &'a [u16],
    pub lut: &'a [Color; 256],
    pub stride: usize,
    pub w: usize,
    pub vis: usize,
}

impl Flames<'_> {
    /// Heat at pixel (`x`, `y`), 0 to 255.
    #[inline]
    pub fn heat(&self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 || x as usize >= self.w || y as usize >= self.vis {
            return 0;
        }
        let i = y as usize * self.stride + PAD + 2 * x as usize;
        (self.heat[i] as u32 + self.heat[i + 1] as u32) / (2 * FP)
    }

    /// The color flames of heat `v` are drawn in.
    #[inline]
    pub fn color(&self, v: u32) -> Color {
        self.lut[v.min(255) as usize]
    }
}

/// A box of pixels, corners included.
#[derive(Clone, Copy)]
pub(super) struct Rect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

/// What a holiday adds to the fire.
pub(super) trait Piece {
    /// Takes the holiday's colors from the theme.
    fn recolor(&mut self, ctx: &Ctx);
    /// Sets everything out for a screen of `w` columns and `ph` pixel rows,
    /// as much of it as the holiday's `level` brings.
    fn layout(&mut self, w: usize, ph: usize, level: f32);
    /// Where the fuel burns instead of all along the bottom, if the screen
    /// has room for the piece.
    fn fuel(&self, sw: usize) -> Option<Fuel>;
    /// How high the flames reach, relative to the bonfire's.
    fn reach(&self) -> f32;
    fn update(&mut self, dt: f32, wind: f32);
    /// Cools the flames where something stands in them: `heat` is the
    /// fire's buffer, `stride` its row length.
    fn block(&self, _heat: &mut [u16], _stride: usize) {}
    /// Sets flames of its own in the fire's buffer, `sw` simulation pixels
    /// wide, once a simulation step: torches, a burning figure.
    fn feed(&mut self, _heat: &mut [u16], _stride: usize, _sw: usize, _intensity: f32) {}
    /// The steam that rises behind the flames, and its color.
    fn steam(&self) -> Option<(&Steam, Color)>;
    /// Vessels and the like, over the flames.
    fn draw(&self, ctx: &Ctx, px: &mut Pixels);
    /// Where the flames lick over what `draw` painted.
    fn lick(&self) -> Option<Rect>;
    /// In front of the flames: the hearth's stones, what stands around it.
    fn draw_front(&self, ctx: &Ctx, px: &mut Pixels);
    /// How many of its things are out, for the tests.
    #[cfg(test)]
    fn counts(&self) -> (usize, usize, usize);
}

/// The holiday on the fire, if it is one of the holidays that dress it this
/// way.
pub(super) struct Fogon {
    kind: Kind,
    level: f32,
    seed: u64,
    ascii: bool,
    piece: Option<Box<dyn Piece>>,
    /// The piece's pictures before they turn into characters (`ascii`).
    scratch: Pixels,
}

impl Fogon {
    pub fn new(seed: u64, ascii: bool) -> Fogon {
        Fogon {
            kind: Kind::None,
            level: 0.0,
            seed,
            ascii,
            piece: None,
            scratch: Pixels::new(1, 1),
        }
    }

    pub fn on(&self) -> bool {
        self.piece.is_some()
    }

    /// How high the flames reach, relative to the bonfire's.
    pub fn reach(&self) -> f32 {
        self.piece.as_ref().map_or(1.0, |p| p.reach())
    }

    #[cfg(test)]
    pub fn counts(&self) -> (usize, usize, usize) {
        self.piece.as_ref().map_or((0, 0, 0), |p| p.counts())
    }

    /// Follows the holiday; returns whether it changed.
    pub fn set(&mut self, fiesta: Fiesta) -> bool {
        let level = if fiesta.level.is_finite() {
            fiesta.level.clamp(0.0, 1.0)
        } else {
            1.0
        };
        let kind = if fiesta.on() { fiesta.kind } else { Kind::None };
        if kind == self.kind && level == self.level {
            return false;
        }
        if kind != self.kind {
            let seed = self.seed ^ SALT;
            self.piece = match kind {
                Kind::Navidad => Some(Box::new(navidad::Posada::new(seed))),
                Kind::Judas => Some(Box::new(judas::Quema::new(seed))),
                Kind::Candelaria => Some(Box::new(candelaria::Tamales::new(seed))),
                Kind::Patrias => Some(Box::new(patrias::Grito::new(seed))),
                _ => None,
            };
        }
        self.kind = kind;
        self.level = level;
        true
    }

    pub fn recolor(&mut self, ctx: &Ctx) {
        if let Some(p) = &mut self.piece {
            p.recolor(ctx);
        }
    }

    pub fn layout(&mut self, w: usize, ph: usize) {
        if self.ascii {
            self.scratch.resize(w as u16, (ph / 2) as u16);
        }
        if let Some(p) = &mut self.piece {
            p.layout(w, ph, self.level);
        }
    }

    pub fn fuel(&self, sw: usize) -> Option<Fuel> {
        self.piece.as_ref().and_then(|p| p.fuel(sw))
    }

    pub fn update(&mut self, dt: f32, wind: f32) {
        if let Some(p) = &mut self.piece {
            p.update(dt, wind);
        }
    }

    /// Flames of its own, fed in before each fire step.
    pub fn feed(&mut self, heat: &mut [u16], stride: usize, sw: usize, intensity: f32) {
        if let Some(p) = &mut self.piece {
            p.feed(heat, stride, sw, intensity);
        }
    }

    /// Flames go round what stands in them: called after each fire step.
    pub fn block(&self, heat: &mut [u16], stride: usize) {
        if let Some(p) = &self.piece {
            p.block(heat, stride);
        }
    }

    /// Steam, drawn first: the flames are in front of it.
    pub fn draw_back(&self, ctx: &Ctx, px: &mut Pixels) {
        if let Some((steam, col)) = self.piece.as_ref().and_then(|p| p.steam()) {
            steam.draw(ctx, px, col);
        }
    }

    /// The vessels, the flames licking over them, and what stands in front.
    pub fn draw(&self, ctx: &Ctx, px: &mut Pixels, flames: &Flames) {
        let Some(p) = &self.piece else { return };
        p.draw(ctx, px);
        if let Some(r) = p.lick() {
            for y in r.y0..=r.y1 {
                for x in r.x0..=r.x1 {
                    let v = flames.heat(x, y);
                    if v >= LICK_MIN && px.get(x, y) != Color::Reset {
                        px.set(x, y, flames.color(v));
                    }
                }
            }
        }
        p.draw_front(ctx, px);
    }

    /// The `ascii` style: the same pictures in characters.
    pub fn draw_ascii(&mut self, ctx: &Ctx, c: &mut Canvas, flames: &Flames) {
        let Some(p) = &self.piece else { return };
        if let Some((steam, col)) = p.steam() {
            steam.chars(ctx, c, col);
        }
        let px = &mut self.scratch;
        px.clear();
        p.draw(ctx, px);
        let lick = p.lick();
        // The flames that would lick over it stay in front.
        chars(px, c, |x, y| {
            lick.is_some_and(|r| {
                (r.x0..=r.x1).contains(&x)
                    && (r.y0..=r.y1).contains(&(2 * y))
                    && flames.heat(x, 2 * y).max(flames.heat(x, 2 * y + 1)) >= LICK_MIN
            })
        });
        px.clear();
        p.draw_front(ctx, px);
        chars(px, c, |_, _| false);
    }
}

/// Turns a layer of pixels into characters on the canvas: two pixels a cell,
/// the character picked by their brightness, except in the cells `skip` says
/// to leave alone.
fn chars(px: &Pixels, c: &mut Canvas, skip: impl Fn(i32, i32) -> bool) {
    const RAMP: &[u8] = b".:-=+*#%@";
    for y in 0..px.ph() / 2 {
        for x in 0..px.pw() {
            let (top, bot) = (px.get(x, 2 * y), px.get(x, 2 * y + 1));
            if (top == Color::Reset && bot == Color::Reset) || skip(x, y) {
                continue;
            }
            let (ch, col) = match (top, bot) {
                (t, Color::Reset) => ('"', t),
                (Color::Reset, b) => ('_', b),
                (t, b) => {
                    let col = t.lerp(b, 0.5);
                    let i = (col.luma() * 1.3 * (RAMP.len() - 1) as f32).round() as usize;
                    (RAMP[i.min(RAMP.len() - 1)] as char, col)
                }
            };
            c.set(x, y, ch, col);
        }
    }
}

/// Ordered dither: how much each pixel of a 4x4 tile needs to be shown.
const BAYER: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

/// A stone of the hearth, a tenamaxtle, filling the box `at`, warmed on the
/// side that faces the fire (`side` is where the fire is: -1 to its left, 1
/// to its right).
fn stone(px: &mut Pixels, at: Rect, side: i32, cols: &[Color; 3], ember: Color) {
    let (w, h) = (at.x1 - at.x0 + 1, at.y1 - at.y0 + 1);
    for r in 0..h {
        let y = at.y0 + r;
        let inset = match r {
            0 => (w / 4).max(1),
            1 if h > 3 => 1,
            _ => 0,
        };
        for x in at.x0 + inset..at.x0 + w - inset {
            let k = r as f32 / h as f32;
            let facing = ((x - at.x0) as f32 / w as f32 - 0.5) * side as f32 * 2.0;
            let col = cols[0].lerp(cols[2], k * 0.9);
            px.set(x, y, col.lerp(ember, (facing * 0.5 + 0.5).powi(2) * 0.55));
        }
    }
}

/// Cools the flames in columns `x0` to `x1` of pixel row `y` to a share
/// `keep` of their heat: something stands there.
fn cool(heat: &mut [u16], stride: usize, y: i32, x0: i32, x1: i32, keep: f32) {
    if y < 0 || x1 < x0 {
        return;
    }
    let sw = stride - 2 * PAD;
    let lo = (2 * x0).max(0) as usize;
    let hi = ((2 * x1 + 2) as usize).min(sw);
    let at = y as usize * stride + PAD;
    if lo >= hi || at + hi > heat.len() {
        return;
    }
    for v in &mut heat[at + lo..at + hi] {
        *v = (*v as f32 * keep) as u16;
    }
}

/// A hash of three small numbers, for dither that needs no dice.
#[inline]
fn hash(a: i32, b: i32, c: u32) -> u32 {
    let mut h = (a as u32).wrapping_mul(0x9e37_79b1) ^ (b as u32).wrapping_mul(0x85eb_ca6b) ^ c;
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^ (h >> 13)
}

/// A disc of radius `r` around (`cx`, `cy`).
fn disc(px: &mut Pixels, cx: i32, cy: i32, r: f32, col: Color) {
    let n = r.ceil() as i32;
    for dy in -n..=n {
        for dx in -n..=n {
            if (dx * dx + dy * dy) as f32 <= r * r + 0.3 {
                px.set(cx + dx, cy + dy, col);
            }
        }
    }
}

/// A line of pixels, ends included.
fn line(px: &mut Pixels, x0: i32, y0: i32, x1: i32, y1: i32, col: Color) {
    let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
    let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
    let (mut x, mut y, mut err) = (x0, y0, dx + dy);
    loop {
        px.set(x, y, col);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

/// A piecewise linear profile: the value at `t` among `(t, value)` stops.
fn profile(stops: &[(f32, f32)], t: f32) -> f32 {
    let mut prev = stops[0];
    for &s in stops {
        if t <= s.0 {
            let span = (s.0 - prev.0).max(1e-6);
            return prev.1 + (s.1 - prev.1) * ((t - prev.0) / span).clamp(0.0, 1.0);
        }
        prev = s;
    }
    prev.1
}

#[derive(Clone, Copy)]
struct Puff {
    x: f32,
    y: f32,
    age: f32,
    life: f32,
    /// Which mouth it came from, for its own curl.
    from: u8,
    dither: u8,
}

/// Where steam comes out: the mouth of a pot, a jarro, a tamalera.
#[derive(Clone, Copy)]
struct Mouth {
    x: f32,
    y: f32,
    /// Half its width.
    half: f32,
    /// Puffs a second.
    rate: f32,
    acc: f32,
}

/// Steam rising from a few mouths, drawn as soft dithered puffs.
pub(super) struct Steam {
    rng: fastrand::Rng,
    mouths: Vec<Mouth>,
    puffs: Vec<Puff>,
    t: f32,
    /// Pixels a second it climbs.
    rise: f32,
}

impl Steam {
    fn new(seed: u64) -> Steam {
        Steam {
            rng: fastrand::Rng::with_seed(seed),
            mouths: Vec::new(),
            puffs: Vec::new(),
            t: 0.0,
            rise: 8.0,
        }
    }

    /// Forgets the mouths and the puffs, and makes room for what the new
    /// mouths will breathe out.
    fn reset(&mut self, rise: f32) {
        self.mouths.clear();
        self.puffs.clear();
        self.rise = rise;
    }

    fn reserve(&mut self) {
        let rate: f32 = self.mouths.iter().map(|m| m.rate).sum();
        self.puffs.reserve_exact((rate * 7.0) as usize + 8);
    }

    fn update(&mut self, dt: f32, wind: f32) {
        self.t += dt;
        let t = self.t;
        for (i, m) in self.mouths.iter_mut().enumerate() {
            m.acc += dt * m.rate;
            while m.acc >= 1.0 {
                m.acc -= 1.0;
                if self.puffs.len() >= self.puffs.capacity() {
                    continue;
                }
                self.puffs.push(Puff {
                    x: m.x + (self.rng.f32() * 2.0 - 1.0) * m.half,
                    y: m.y,
                    age: 0.0,
                    life: 3.0 + self.rng.f32() * 2.5,
                    from: i as u8,
                    dither: self.rng.u8(..),
                });
            }
        }
        let rise = self.rise;
        self.puffs.retain_mut(|p| {
            p.age += dt;
            let k = p.age / p.life;
            // Straight up out of the pot, then swaying as it climbs and slows.
            let a = (p.age * 0.9).min(2.5);
            let f = p.from as f32 * 1.7 + p.dither as f32 * 0.02;
            let curl = (p.y * 0.19 + t * 0.7 + f).sin() + 0.5 * (p.y * 0.41 - t * 1.1 + f).sin();
            p.x += (a * curl + wind * 3.0) * dt;
            p.y -= rise * (1.0 - 0.5 * k) * dt;
            p.age < p.life && p.y > -2.0
        });
    }

    fn draw(&self, ctx: &Ctx, px: &mut Pixels, col: Color) {
        let known_bg = matches!(ctx.bg, Color::Rgb(..));
        for p in &self.puffs {
            let k = p.age / p.life;
            let alpha = (1.0 - k).powf(1.2) * (p.age * 3.0).min(1.0);
            // It thickens as it spreads: a soft cloud, not a single dot.
            let r = 1.0 + k * 2.4;
            let (x, y) = (p.x.round() as i32, p.y.round() as i32);
            let n = r.ceil() as i32;
            // With a known background it fades into it; without one it
            // thins out into dither instead.
            let faded = ctx.fade(col, 1.0 - alpha * 0.85);
            for dy in -n / 2..=n / 2 {
                for dx in -n..=n {
                    let d = (dx * dx) as f32 * 0.5 + (dy * dy) as f32 * 2.0;
                    if d > r * r * 0.6 {
                        continue;
                    }
                    let (sx, sy) = (x + dx, y + dy);
                    if known_bg {
                        px.set(sx, sy, faded);
                    } else if BAYER[(sy & 3) as usize][(sx & 3) as usize] as f32 / 16.0 < alpha {
                        px.set(sx, sy, col.scale(0.85));
                    }
                }
            }
        }
    }
}

impl Steam {
    /// The same steam in characters, in the empty cells of the canvas.
    fn chars(&self, ctx: &Ctx, c: &mut Canvas, col: Color) {
        let known_bg = matches!(ctx.bg, Color::Rgb(..));
        for p in &self.puffs {
            let k = p.age / p.life;
            let alpha = (1.0 - k).powf(1.2) * (p.age * 3.0).min(1.0);
            if !known_bg && p.dither as f32 >= alpha * 300.0 {
                continue;
            }
            let (x, y) = (p.x.round() as i32, (p.y / 2.0).floor() as i32);
            if c.get(x, y).is_none_or(|cell| cell.ch != ' ') {
                continue;
            }
            let ch = match (k * 4.0) as u8 {
                0 => '|',
                1 => ['(', ')'][(p.dither & 1) as usize],
                2 => '~',
                _ => '.',
            };
            c.set(x, y, ch, ctx.fade(col, 1.0 - alpha * 0.8));
        }
    }
}

#[derive(Clone, Copy)]
struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
    /// Its colors: 0 white-gold, 1 red and gold, 2 an ember.
    hue: u8,
}

/// Sparks flying from luces de bengala and the like, in a pool that never
/// grows: when it is full, no more sparks come.
pub(super) struct Sparks {
    rng: fastrand::Rng,
    v: Vec<Spark>,
}

/// The colors a spark of each hue goes through as it burns out.
const SPARK_COLORS: [[u32; 4]; 3] = [
    [0xfffbe8, 0xffd23b, 0xff8c1a, 0x8a2a08],
    [0xffd23b, 0xff4a3a, 0xc02020, 0x601010],
    [0xff8a1a, 0xff5a10, 0xa02a08, 0x400e04],
];

impl Sparks {
    fn new(seed: u64) -> Sparks {
        Sparks {
            rng: fastrand::Rng::with_seed(seed),
            v: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.v.clear();
    }

    /// Makes room for `n` sparks at once.
    fn reserve(&mut self, n: usize) {
        self.v.reserve_exact(n);
    }

    /// A spark leaves (`x`, `y`) in a random direction at up to `speed`
    /// pixels a second.
    fn spawn(&mut self, x: f32, y: f32, speed: f32, hue: u8) {
        if self.v.len() >= self.v.capacity() {
            return;
        }
        let a = self.rng.f32() * std::f32::consts::TAU;
        let s = speed * (0.3 + 0.7 * self.rng.f32());
        let life = if hue == 2 { 0.8 } else { 0.3 } + self.rng.f32() * 0.5;
        self.v.push(Spark {
            x,
            y,
            vx: a.cos() * s,
            vy: a.sin() * s - speed * 0.15,
            age: 0.0,
            life,
            hue,
        });
    }

    fn update(&mut self, dt: f32) {
        self.v.retain_mut(|s| {
            s.age += dt;
            // They fall, and the air slows them.
            s.vy += if s.hue == 2 { 18.0 } else { 45.0 } * dt;
            let drag = (1.0 - 1.4 * dt).max(0.0);
            s.vx *= drag;
            s.vy *= drag;
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            s.age < s.life
        });
    }

    fn draw(&self, px: &mut Pixels) {
        for s in &self.v {
            let k = (s.age / s.life).clamp(0.0, 1.0) * 3.0;
            let i = (k as usize).min(2);
            let stops = &SPARK_COLORS[s.hue as usize % 3];
            let col = rgb(stops[i]).lerp(rgb(stops[i + 1]), k - i as f32);
            px.set(s.x.round() as i32, s.y.round() as i32, col);
        }
    }
}
