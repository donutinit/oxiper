//! Día de Muertos on the waves (see the module doc in `waveform.rs`).
//!
//! The holiday only changes how a layer is dressed, never the waves
//! themselves: the same `ys` the ordinary scene draws from carry the garlands.
//! Everything that moves by itself (crests lighting up, petals, embers) is
//! stepped in `update`, so a frame is just drawing; the RNG is its own, so an
//! ordinary day draws the same numbers as ever.

use std::f32::consts::{PI, TAU};

use super::{Layer, SHADES, Style, Waveform, noise1};
use crate::color::{Color, rgb};
use crate::render::Braille;
use crate::scene::Ctx;

/// Papel picado colors for the waves, front to back: cempasúchil, rosa
/// mexicano, amarillo, turquesa, morado, verde limón.
const PICADO: [Color; 6] = [
    rgb(0xff8c00),
    rgb(0xe4007c),
    rgb(0xffd200),
    rgb(0x00aab8),
    rgb(0x7028a0),
    rgb(0x8cc828),
];
/// Sugar and bone, for the lighter petal of each pair.
const HUESO: Color = rgb(0xf4eee2);
/// A flame from the heart out: white-hot, gold, orange, and its dark tip.
const FLAMA: [Color; 4] = [rgb(0xfff6d8), rgb(0xffdc6e), rgb(0xffae28), rgb(0xff6a10)];
/// The same on a light terminal, where a pale flame would vanish.
const FLAMA_DIA: [Color; 4] = [rgb(0xffce46), rgb(0xff9600), rgb(0xf05f00), rgb(0xbe2800)];
/// The warm light a candle gives the petals around it.
const GLOW: Color = rgb(0xffbe3c);
/// Blossoms, petals then heart: cempasúchil and terciopelo (velvet cockscomb).
const BLOSSOM: [[Color; 2]; 2] = [
    [rgb(0xff8c00), rgb(0xffd23c)],
    [rgb(0xb00048), rgb(0xff5aa0)],
];
/// Loose petals, in pairs of shades that alternate as they tumble:
/// cempasúchil, terciopelo, and the embers that rise off the flames (deeper
/// on a light terminal).
const LOOSE: [[Color; 2]; 3] = [
    [rgb(0xffaa1e), rgb(0xd66200)],
    [rgb(0xe4007c), rgb(0xb00048)],
    [rgb(0xffe08a), rgb(0xffa030)],
];
const EMBERS_DIA: [Color; 2] = [rgb(0xeb780a), rgb(0xbe4600)];
/// Most petals and embers in the air at once.
const MAX_LOOSE: usize = 200;
/// Most candles lit at once.
const MAX_CANDLES: usize = 96;

/// A petal, ember or soul adrift, in dots.
#[derive(Clone, Copy)]
struct Loose {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// The speed it settles to vertically (negative: it keeps rising).
    fall: f32,
    /// Tumbling rate (frames per second) and sway phase.
    spin: f32,
    sway: f32,
    age: f32,
    life: f32,
    /// 0 = cempasúchil, 1 = terciopelo, 2 = ember.
    kind: usize,
}

/// `v` rounded to the nearest dot (a plain cast, for anything above -512).
#[inline]
fn rnd(v: f32) -> i32 {
    (v + 512.5) as i32 - 512
}

/// A hash of `n`, 0 to 1.
fn hash(n: u32) -> f32 {
    let n = n.wrapping_mul(0x9e37_79b9) ^ 0x85eb_ca6b;
    let n = (n ^ (n >> 15)).wrapping_mul(0x2c1b_3c6d);
    ((n ^ (n >> 12)) & 0xffff) as f32 / 65536.0
}

/// A veladora burning on a crest.
#[derive(Clone, Copy)]
struct Candle {
    id: u32,
    layer: usize,
    /// Dot column and the top of the layer's petals there.
    x: i32,
    top: f32,
    /// 0 (just caught, or going out) to 1 (burning bright).
    lit: f32,
    seed: f32,
    seen: bool,
}

/// A layer's flame colors, a little dimmer the farther back it burns.
#[derive(Clone, Copy)]
struct Flame {
    hot: Color,
    core: Color,
    edge: Color,
    tip: Color,
    /// The halo, near and far from the wick.
    halo: [Color; 2],
    /// The glass: lit by the flame, plain and shining, and the same on a reflection.
    glass: [Color; 3],
    ghost: [Color; 3],
    /// What the glow tints petals toward.
    glow: Color,
}

pub(super) struct Muertos {
    /// How many layers, from the front, carry candles and blossoms.
    lit: usize,
    level: f32,
    /// Per layer, `SHADES` colors of the lighter petal in each pair.
    tints: Vec<Color>,
    flames: Vec<Flame>,
    /// Loose petals by kind, shade and fade (0 = full .. 3 = nearly gone).
    loose: [[[Color; 4]; 2]; 3],
    rng: fastrand::Rng,
    petals: Vec<Loose>,
    candles: Vec<Candle>,
    /// This frame's candle-lit crests: layer, dot column, petals' top.
    found: Vec<(usize, i32, f32)>,
    /// The light each dot column of the layer being drawn gets.
    glow: Vec<f32>,
    /// The highest crest among the layers in front of it, per dot column.
    front: Vec<f32>,
    ids: u32,
    /// A petal's width along its length, 0 to 1 over 32 steps.
    petal: [f32; 32],
    /// Flicker clock in seconds, and how long it has burned (up to a few).
    t: f32,
    age: f32,
    /// Are `ys` this frame's already? (`update` finds the crests from them.)
    pub(super) fresh: bool,
}

impl Muertos {
    /// Dresses the waves for the holiday, or retouches the colors after a
    /// theme change. `shades` are the layers' colors, `reach` how far they fade.
    pub(super) fn dress(
        slot: &mut Option<Muertos>,
        ctx: &Ctx,
        layers: &[Layer],
        shades: &[Color],
        reach: f32,
        pw: usize,
    ) {
        let n = layers.len();
        let m = slot.get_or_insert_with(|| Muertos {
            lit: 0,
            level: 0.0,
            tints: Vec::with_capacity(n * SHADES),
            flames: Vec::with_capacity(n),
            loose: [[[Color::Reset; 4]; 2]; 3],
            rng: fastrand::Rng::with_seed(ctx.seed ^ 0x6d75_6572_746f_7321),
            petals: Vec::with_capacity(MAX_LOOSE),
            candles: Vec::with_capacity(MAX_CANDLES),
            found: Vec::with_capacity(MAX_CANDLES),
            glow: vec![0.0; pw],
            front: vec![0.0; pw],
            ids: 0,
            // Fills out fast, tapers to a point.
            petal: std::array::from_fn(|i| (PI * (i as f32 / 31.0).powf(0.75)).sin().powf(0.65)),
            t: 0.0,
            age: 0.0,
            fresh: false,
        });
        let light = matches!(ctx.bg, Color::Rgb(..)) && ctx.bg.luma() > 0.6;
        let level = ctx.fiesta.level.clamp(0.0, 1.0);
        m.level = level;
        // Candles on the front waves first (most of them from the first day),
        // on every wave by November 2.
        m.lit = ((n as f32 * (0.4 + 0.6 * level)).ceil() as usize).clamp(1, n);
        m.tints.clear();
        m.flames.clear();
        for (li, l) in layers.iter().enumerate() {
            let c = shades[li * SHADES].lerp(HUESO, 0.4);
            for s in 0..SHADES {
                let k = s as f32 / (SHADES - 1) as f32;
                m.tints.push(ctx.fade(c, k * reach));
            }
            // Candles on the back waves burn a little dimmer.
            let k = (1.0 - l.depth) * 0.35;
            let [hot, core, edge, tip] =
                if light { FLAMA_DIA } else { FLAMA }.map(|f| ctx.fade(f, k));
            let body = shades[li * SHADES + 2];
            m.flames.push(Flame {
                hot,
                core,
                edge,
                tip,
                halo: if light {
                    [
                        ctx.fade(rgb(0xc8641e), k + 0.1),
                        ctx.fade(rgb(0xa0643c), k + 0.1),
                    ]
                } else {
                    [
                        ctx.fade(rgb(0xd9902a), k + 0.2),
                        ctx.fade(rgb(0x8a4a12), k + 0.15),
                    ]
                },
                glass: [body.lerp(GLOW, 0.45), body, body.lerp(HUESO, 0.4)],
                ghost: [body, body, body].map(|c| ctx.fade(c, 0.55)),
                glow: ctx.fade(GLOW, k),
            });
        }
        for (kind, pair) in LOOSE.iter().enumerate() {
            let pair = if light && kind == 2 {
                &EMBERS_DIA
            } else {
                pair
            };
            for (shade, &c) in pair.iter().enumerate() {
                for f in 0..4 {
                    m.loose[kind][shade][f] = ctx.fade(c, f as f32 * 0.22);
                }
            }
        }
    }

    /// The screen changed size: nothing found so far still stands.
    pub(super) fn resized(&mut self, pw: usize) {
        self.candles.clear();
        self.petals.clear();
        self.glow.clear();
        self.glow.resize(pw, 0.0);
        self.front.clear();
        self.front.resize(pw, 0.0);
        self.fresh = false;
        self.age = 0.0;
    }
}

/// A layer's color under the holiday: papel picado, front to back, or the
/// theme's accents.
pub(super) fn picado(ctx: &Ctx, j: usize) -> Color {
    ctx.pick(j, PICADO[j % PICADO.len()])
}

/// The top of a layer on screen, from its crest height (the mirror's
/// signed offset folds up around the center line).
#[inline]
fn top_of(mirror: bool, center: f32, y: f32) -> f32 {
    if mirror { center - y.abs() } else { y }
}

/// A veladora's measures, in dots, from the biggest to the farthest.
struct Size {
    glass_w: i32,
    glass_h: i32,
    flame_h: i32,
    flame_w: f32,
}

const SIZES: [Size; 5] = [
    Size {
        glass_w: 8,
        glass_h: 9,
        flame_h: 14,
        flame_w: 8.0,
    },
    Size {
        glass_w: 6,
        glass_h: 7,
        flame_h: 10,
        flame_w: 6.0,
    },
    Size {
        glass_w: 4,
        glass_h: 5,
        flame_h: 8,
        flame_w: 4.0,
    },
    Size {
        glass_w: 4,
        glass_h: 3,
        flame_h: 6,
        flame_w: 3.4,
    },
    Size {
        glass_w: 0,
        glass_h: 0,
        flame_h: 5,
        flame_w: 2.4,
    },
];

/// Blossoms in dots (`o` petals, `c` heart): pompons of cempasúchil, big and
/// small, and a little cross for the beads.
const R4: [&str; 9] = [
    "...ooo...",
    "..ooooo..",
    ".ooooooo.",
    "oooocoooo",
    "ooocccooo",
    "oooocoooo",
    ".ooooooo.",
    "..ooooo..",
    "...ooo...",
];
const R3: [&str; 7] = [
    "..ooo..", ".ooooo.", "ooocooo", "oocccoo", "ooocooo", ".ooooo.", "..ooo..",
];
const R2: [&str; 5] = [".ooo.", "ooooo", "oocoo", "ooooo", ".ooo."];
const R1: [&str; 3] = [".o.", "oco", ".o."];
/// A calavera de azúcar, big and small: bone (`o`) with its sockets cleared (`.`).
const SKULL9: [&str; 9] = [
    "..ooooo..",
    ".ooooooo.",
    "ooooooooo",
    "oo..o..oo",
    "oo..o..oo",
    "ooooooooo",
    ".oo.o.oo.",
    "..o.o.o..",
    "..ooooo..",
];
const SKULL7: [&str; 7] = [
    ".ooooo.", "ooooooo", "o.ooo.o", "o.ooo.o", "ooooooo", ".o.o.o.", ".ooooo.",
];

/// A tumbling petal's four turns, in dots: face on, tilting, edge on, tilting.
const TUMBLE: [&[(i32, i32)]; 4] = [
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
    &[(1, 0), (0, 1), (1, 1)],
    &[(0, 0), (1, 1)],
    &[(0, 0), (1, 0), (0, 1)],
];

impl Waveform {
    // Stepping: crests, candles, petals.

    /// Moves the holiday on by `dt` seconds: which crests carry a candle,
    /// and the petals and embers adrift.
    pub(super) fn step_muertos(&mut self, dt: f32) {
        let Some(mut m) = self.muertos.take() else {
            return;
        };
        m.t = (m.t + dt) % 3600.0;
        if self.pw > 0 && m.glow.len() == self.pw && m.front.len() == self.pw {
            self.compute();
            m.fresh = true;
            self.find_candles(&mut m, dt);
            self.drift(&mut m, dt);
        }
        m.age = (m.age + dt).min(8.0);
        self.muertos = Some(m);
    }

    /// The size class of a candle on layer `li`: the nearest are the biggest,
    /// and a candle just lit (or going out) is smaller.
    fn candle_size(&self, li: usize, lit: f32) -> usize {
        let d = self.layers[li].depth;
        let base = match d {
            _ if self.layers.len() == 1 => 1,
            d if d > 0.6 => 1,
            d if d > 0.3 => 2,
            d if d > 0.1 => 3,
            _ => 4,
        };
        // On a tall screen everything is a size bigger, on a short one smaller.
        let big = (self.ph >= 180) as usize;
        let small = (self.ph < 110) as usize + (self.ph < 60) as usize + (lit < 0.5) as usize;
        (base + small).saturating_sub(big).min(4)
    }

    /// The dot row a candle's glass stands on above a crest: on the petals.
    fn foot(&self, li: usize, top: f32) -> f32 {
        top - self.petal_len(li).0 * 0.7
    }

    /// How big the petals are, next to what they are on a 36-row screen.
    fn scale(&self) -> f32 {
        (self.ph as f32 / 144.0).clamp(0.7, 1.4)
    }

    /// How far a layer's pattern has ridden along with the swell, in dots, as
    /// whole `period`s (the laps) and what is left over, so a long run never
    /// loses its precision.
    fn ride(&self, li: usize, period: f32) -> (i64, f32) {
        let d = self.layers[li].depth;
        let wv = self.swell[0];
        let kx = TAU / self.lambda()
            * if self.style == Style::Mirror {
                1.0
            } else {
                1.0 + 0.45 * (1.0 - d)
            };
        let full = wv.omega as f64 * self.t as f64 / (wv.k as f64 * kx as f64);
        let laps = (full / period as f64).floor();
        (laps as i64, (full - laps * period as f64) as f32)
    }

    /// Finds the crests that carry a candle (the highest point within a
    /// stretch of each lit layer, standing clear of the troughs around it) and
    /// lets the candles there catch, burn and go out.
    fn find_candles(&mut self, m: &mut Muertos, dt: f32) {
        let (pw, n) = (self.pw, self.layers.len());
        let mirror = self.style == Style::Mirror;
        let center = self.ph as f32 * 0.5 - 0.5;
        let reach = (self.lambda() * 0.2).max(6.0) as usize;
        m.found.clear();
        if pw >= 3 {
            let ys = &self.ys;
            let top = |li: usize, x: usize| top_of(mirror, center, ys[li * pw + x]);
            for li in n - m.lit.min(n)..n {
                for x in 1..pw - 1 {
                    let y = top(li, x);
                    if !(y <= top(li, x - 1) && y < top(li, x + 1)) || y < 4.0 {
                        continue;
                    }
                    let (lo, hi) = (x.saturating_sub(reach), (x + reach).min(pw - 1));
                    let mut low = y;
                    let highest = (lo..=hi).all(|j| {
                        let v = top(li, j);
                        low = low.max(v);
                        v > y || (v == y && j >= x)
                    });
                    // Filled waves hide what is behind the ones in front.
                    let hidden = self.style == Style::Filled
                        && (li + 1..n).any(|lj| ys[lj * pw + x] < y + 3.0);
                    if highest && low - y > 2.0 && !hidden && m.found.len() < MAX_CANDLES {
                        m.found.push((li, x as i32, y));
                    }
                }
            }
        }
        // Candles follow their crest, catch gently and go out gently.
        let k = (dt * 2.5).min(1.0);
        let start = if m.age < 0.2 { 1.0 } else { 0.0 };
        m.candles.iter_mut().for_each(|c| c.seen = false);
        for &(li, x, y) in &m.found {
            let near = m
                .candles
                .iter_mut()
                .filter(|c| c.layer == li && !c.seen && (c.x - x).abs() <= 10)
                .min_by_key(|c| (c.x - x).abs());
            if let Some(c) = near {
                (c.x, c.top, c.seen) = (x, y, true);
                c.lit = (c.lit + k).min(1.0);
            } else if m.candles.len() < MAX_CANDLES {
                m.ids = m.ids.wrapping_add(1);
                m.candles.push(Candle {
                    id: m.ids,
                    layer: li,
                    x,
                    top: y,
                    lit: start,
                    seed: (m.ids % 977) as f32 * 3.71,
                    seen: true,
                });
            }
        }
        for c in m.candles.iter_mut().filter(|c| !c.seen) {
            c.lit -= k;
        }
        m.candles.retain(|c| c.seen || c.lit > 0.0);
    }

    /// Blows petals off the crests, lifts embers off the flames, and carries
    /// them all downwind.
    fn drift(&mut self, m: &mut Muertos, dt: f32) {
        let (pw, ph) = (self.pw as f32, self.ph as f32);
        let n = self.layers.len();
        let mirror = self.style == Style::Mirror;
        let center = ph * 0.5 - 0.5;
        let target = ((pw * ph / 500.0) * (0.3 + 0.7 * m.level)).min(160.0) as usize;
        let mut left = dt;
        while left > 0.0 {
            let h = left.min(0.05);
            left -= h;
            m.petals.retain_mut(|p| {
                p.age += h;
                p.vy += (p.fall - p.vy) * (0.8 * h).min(1.0);
                p.x += p.vx * h;
                p.y += p.vy * h;
                p.age < p.life && p.x < pw + 4.0 && p.x > -4.0 && p.y < ph + 4.0 && p.y > -8.0
            });
            let room = m.petals.len() < MAX_LOOSE;
            let r = &mut m.rng;
            // Petals off a lit wave, near its crests: the highest of a few tries.
            let blown = m.petals.iter().filter(|p| p.kind < 2).count();
            if room && blown < target && r.f32() < h * target as f32 / 3.0 {
                let li = n - 1 - r.usize(..m.lit.clamp(1, n)).min(r.usize(..n));
                let (mut x, mut y) = (0, f32::INFINITY);
                for _ in 0..4 {
                    let xi = r.usize(..self.pw);
                    let yi = top_of(mirror, center, self.ys[li * self.pw + xi]);
                    if yi < y {
                        (x, y) = (xi, yi);
                    }
                }
                m.petals.push(Loose {
                    x: x as f32,
                    y: y - 3.0,
                    vx: 5.0 + 9.0 * r.f32(),
                    vy: -(2.0 + 5.0 * r.f32()),
                    fall: 2.5 + 3.5 * r.f32(),
                    spin: 2.0 + 5.0 * r.f32(),
                    sway: r.f32() * TAU,
                    age: 0.0,
                    life: 6.0 + 6.0 * r.f32(),
                    kind: (r.f32() < 0.22) as usize,
                });
            }
            // Embers: a few for every burning flame, rising and twinkling out.
            let rate = (0.7 + 1.5 * m.level) * h;
            for ci in 0..m.candles.len() {
                let c = m.candles[ci];
                if c.lit < 0.6 || m.petals.len() >= MAX_LOOSE || m.rng.f32() >= rate {
                    continue;
                }
                let sz = &SIZES[self.candle_size(c.layer, c.lit)];
                let r = &mut m.rng;
                m.petals.push(Loose {
                    x: c.x as f32 + r.f32() * 2.0 - 1.0,
                    y: self.foot(c.layer, c.top) - (sz.glass_h + sz.flame_h) as f32 - 1.0,
                    vx: 1.0 + 3.0 * r.f32(),
                    vy: -(7.0 + 6.0 * r.f32()),
                    fall: -(2.0 + 2.0 * r.f32()),
                    spin: 5.0 + 6.0 * r.f32(),
                    sway: r.f32() * TAU,
                    age: 0.0,
                    life: 1.3 + 1.7 * r.f32(),
                    kind: 2,
                });
            }
        }
    }

    // Drawing.

    pub(super) fn draw_muertos(&mut self) {
        let Some(mut m) = self.muertos.take() else {
            return;
        };
        let n = self.layers.len();
        let center = self.ph as f32 * 0.5 - 0.5;
        if self.style == Style::Filled {
            self.cutwork();
        }
        for li in 0..n {
            self.lay_out(&mut m, li);
            self.halos(&m, li);
            match self.style {
                Style::Lines | Style::Filled => {
                    self.garland(&m, li, 1.0, 0.0, self.style == Style::Filled);
                    self.blossoms(&m, li, 1.0, 0.0);
                }
                Style::Mirror => {
                    for sign in [1.0, -1.0] {
                        self.garland(&m, li, sign, center, false);
                        self.blossoms(&m, li, sign, center);
                    }
                }
                Style::Dots => self.beads(&m, li),
            }
        }
        // The candles last, so no wave hides a flame.
        for li in 0..n {
            self.candles(&m, li);
        }
        self.draw_loose(&m);
        self.muertos = Some(m);
    }

    /// The filled style's paper: under each garland, tissue with a diamond
    /// lattice cut into it, thinning out to a scatter of holes farther down.
    fn cutwork(&mut self) {
        let (pw, ph) = (self.pw, self.ph as i32);
        let n = self.layers.len();
        // How far below its crest a layer's paper reaches.
        let reach = (self.ph as f32 * 0.26).max(4.0);
        for li in 0..n {
            let base = ((1.0 - self.layers[li].depth) * 5.0) as usize;
            for x in 0..pw {
                let top = self.ys[li * pw + x];
                // Hidden behind the crests of the layers in front.
                let mut bottom = ph as f32;
                for lj in li + 1..n {
                    bottom = bottom.min(self.ys[lj * pw + x]);
                }
                let (t0, b0) = (rnd(top), rnd(bottom));
                let end = b0.min(ph).min(t0 + reach as i32 + 1);
                let from = t0.max(0);
                if from >= end {
                    continue;
                }
                // The two diagonals' phases at the bottom, then walking up.
                let (mut a, mut b) = (
                    (x as i32 + end - 1).rem_euclid(6),
                    (x as i32 - end + 1).rem_euclid(6),
                );
                for y in (from..end).rev() {
                    let depth = (y - t0) as f32 / reach;
                    // Two diagonals of cuts crossing: bars first, then a
                    // mesh, then only the knots, then a few.
                    let on = match depth {
                        d if d < 0.3 => a < 2 || b < 2,
                        d if d < 0.6 => a == 0 || b == 0,
                        d if d < 0.85 => a == 0 && b == 0,
                        _ => a == 3 && b == 3 && x % 2 == 0,
                    };
                    if on {
                        let s = (base + (1.0 + depth * 8.0) as usize).min(SHADES - 1);
                        self.braille.set(x as i32, y, self.shades[li * SHADES + s]);
                    }
                    a = if a == 0 { 5 } else { a - 1 };
                    b = if b == 5 { 0 } else { b + 1 };
                }
            }
        }
    }

    /// For layer `li`: how much light each dot column gets from its candles,
    /// and (on filled waves, which hide what lies behind) the crests in front.
    fn lay_out(&self, m: &mut Muertos, li: usize) {
        if self.style == Style::Filled {
            let pw = self.pw;
            m.front.fill(f32::INFINITY);
            for lj in li + 1..self.layers.len() {
                for (f, &y) in m.front.iter_mut().zip(&self.ys[lj * pw..(lj + 1) * pw]) {
                    *f = f.min(y);
                }
            }
        }
        m.glow.fill(0.0);
        let reach = 8.0 + 12.0 * self.layers[li].depth;
        let r = reach as i32;
        for c in m.candles.iter().filter(|c| c.layer == li) {
            let flick = 0.8 + 0.2 * noise1(m.t * 5.0 + c.seed);
            for x in (c.x - r).max(0)..=(c.x + r).min(self.pw as i32 - 1) {
                let g = (1.0 - (x - c.x).abs() as f32 / reach).powi(2) * c.lit * flick;
                let slot = &mut m.glow[x as usize];
                *slot = slot.max(g);
            }
        }
    }

    /// The light around each flame of layer `li`: motes of gold that twinkle
    /// in and out around it, thinning out with the distance.
    fn halos(&mut self, m: &Muertos, li: usize) {
        let fl = &m.flames[li];
        let d = self.layers[li].depth;
        for c in m.candles.iter().filter(|c| c.layer == li) {
            let sz = &SIZES[self.candle_size(li, c.lit)];
            let r = (10.0 + 9.0 * d) * (0.4 + 0.6 * c.lit);
            let cy = self.foot(li, c.top) - (sz.glass_h + sz.flame_h / 2) as f32;
            for i in 0..16u32 {
                let h = |k: u32| hash(c.id.wrapping_mul(31).wrapping_add(i * 7 + k));
                // Each mote drifts slowly round the flame and twinkles on its own.
                let a = h(0) * TAU + m.t * (h(1) - 0.5) * 0.8;
                let rho = r * (0.3 + 0.7 * h(2).sqrt());
                if noise1(m.t * 2.2 + i as f32 * 5.1 + c.seed) < -0.15 {
                    continue;
                }
                let (x, y) = (c.x as f32 + a.cos() * rho, cy + a.sin() * rho * 1.15);
                self.braille.set(
                    x.round() as i32,
                    y.round() as i32,
                    fl.halo[(rho > r * 0.6) as usize],
                );
            }
        }
    }

    /// How far a layer's petals reach from its stem, in dots (and the
    /// length of one).
    fn petal_len(&self, li: usize) -> (f32, f32) {
        let d = self.layers[li].depth;
        let len = (6.0 + (2.0 * d).round()) * 1.1 * self.scale();
        let wide = (1.3 + 0.7 * d) * (0.75 + 0.25 * self.thickness as f32) * self.scale();
        // On the oscilloscope the traces crowd together: slimmer petals.
        if self.style == Style::Mirror {
            (len * 0.8, wide * 0.7)
        } else {
            (len, wide)
        }
    }

    /// A layer as a garland: a thread along the wave with a petal every few
    /// dots, one up, one down, each pointing the way the swell rolls,
    /// shades alternating, warmed by the candles on its crests. With `clip`,
    /// only what shows above the layers in front.
    fn garland(&mut self, m: &Muertos, li: usize, sign: f32, center: f32, clip: bool) {
        let pw = self.pw;
        let depth = self.layers[li].depth;
        let base = ((1.0 - depth) * 5.0) as usize;
        let gap = (6.0 + (2.0 * depth).round()) * self.scale();
        let (reach, wide) = self.petal_len(li);
        // The sides and shades repeat every four petals; count the laps to
        // know each petal.
        let (laps, ride) = self.ride(li, 4.0 * gap);
        let glow = m.flames[li].glow;
        let row = &self.ys[li * pw..(li + 1) * pw];
        let at = |x: f32| {
            let xi = x.clamp(0.0, pw as f32 - 1.001);
            let i = xi as usize;
            let (a, b) = (row[i], row[(i + 1).min(pw - 1)]);
            center + sign * (a + (b - a) * (xi - i as f32))
        };
        // The thread, a shade dimmer than the petals.
        let thread = self.shades[li * SHADES + (base + 2).min(SHADES - 1)];
        let mut y = rnd(center + sign * row[0]);
        for x in 0..pw {
            let y1 = rnd(center + sign * row[(x + 1).min(pw - 1)]);
            Waveform::span(&mut self.braille, x as i32, y.min(y1), y.max(y1), thread);
            y = y1;
        }
        let mut k = ((-reach - ride) / gap).ceil() as i64;
        loop {
            let x = ride + k as f32 * gap;
            if x >= pw as f32 + reach {
                break;
            }
            // Each petal is a little different from the next, and stays so.
            let id = (k - 4 * laps) as u32;
            let kk = id & 3;
            let jit = |j: u32| hash(id.wrapping_mul(7919).wrapping_add(li as u32 * 104_729 + j));
            k += 1;
            let (sin_f, cos_f) = (0.62 + (jit(0) - 0.5) * 0.3).sin_cos();
            let (reach, wide) = (reach * (0.85 + 0.3 * jit(1)), wide * (0.8 + 0.4 * jit(2)));
            let y = at(x);
            let slope = (at(x + 2.0) - at(x - 2.0)) * 0.25;
            let norm = (1.0 + slope * slope).sqrt().recip();
            let (tx, ty) = (norm, slope * norm);
            // Up (-) for the even petals, down (+) for the odd ones.
            let side = if kk & 1 == 0 { -1.0 } else { 1.0 };
            let (ax, ay) = (
                tx * cos_f - side * ty * sin_f,
                side * tx * sin_f + ty * cos_f,
            );
            let light = kk >= 2;
            let xi = (x.round() as i32).clamp(0, pw as i32 - 1) as usize;
            let glint =
                0.5 + 0.5 * (x * 0.045 - self.t * (0.6 + depth * 0.5) + li as f32 * 1.7).sin();
            let s = (base + (glint * 3.0 + 2.0 * jit(3)) as usize).min(SHADES - 1);
            let mut c = if light {
                m.tints[li * SHADES + s]
            } else {
                self.shades[li * SHADES + s]
            };
            let g = m.glow[xi];
            if g > 0.0 {
                c = c.lerp(glow, g * 0.9);
            }
            let front = &m.front;
            let hidden = |dx: i32, dy: i32| {
                clip && dx >= 0 && dx < pw as i32 && front[dx as usize] <= dy as f32
            };
            petal(
                &mut self.braille,
                &m.petal,
                (x, y),
                (ax, ay),
                (reach, wide),
                c,
                hidden,
            );
        }
    }

    /// Marigolds and cockscomb strung on the front garlands, and now and then
    /// a sugar skull, riding the swell.
    fn blossoms(&mut self, m: &Muertos, li: usize, sign: f32, center: f32) {
        let (pw, n) = (self.pw, self.layers.len());
        if li + m.lit < n {
            return;
        }
        let d = self.layers[li].depth;
        let (spr, bones, gap): (&[&str], &[&str], f32) = if d > 0.7 || n == 1 {
            (&R4, &SKULL9, 44.0)
        } else if d > 0.3 {
            (&R3, &SKULL7, 34.0)
        } else {
            return;
        };
        let (laps, ride) = self.ride(li, 5.0 * gap);
        let mut k = ((-4.0 - ride) / gap).ceil() as i64;
        loop {
            let x = ride + k as f32 * gap;
            if x >= pw as f32 + 4.0 {
                break;
            }
            let xi = (x.round() as i32).clamp(0, pw as i32 - 1);
            let y = (center + sign * self.ys[li * pw + xi as usize]).round() as i32;
            let hidden = self.style == Style::Filled && m.front[xi as usize] <= y as f32;
            if !hidden {
                // Marigolds, with cockscomb between, and a sugar skull now and then.
                let kind = [0, 1, 0, 1, 2][(k - 5 * laps + li as i64).rem_euclid(5) as usize];
                let g = m.glow[xi as usize];
                let lit = |c: Color| {
                    if g > 0.0 {
                        c.lerp(m.flames[li].glow, g * 0.5)
                    } else {
                        c
                    }
                };
                if kind == 2 {
                    skull(&mut self.braille, x.round() as i32, y, bones, lit(HUESO));
                } else {
                    blossom(
                        &mut self.braille,
                        x.round() as i32,
                        y,
                        spr,
                        BLOSSOM[kind].map(lit),
                    );
                }
            }
            k += 1;
        }
    }

    /// The beads as little flowers riding the swell, in alternating shades.
    fn beads(&mut self, m: &Muertos, li: usize) {
        let pw = self.pw;
        let l = &self.layers[li];
        let gap = 7.0 + (1.0 - l.depth) * 1.5;
        let (laps, ride) = self.ride(li, 2.0 * gap);
        let spr: &[&str] = if l.depth > 0.5 { &R2 } else { &R1 };
        let s = ((1.0 - l.depth) * 6.0) as usize;
        let mut k = ((-3.0 - ride) / gap).ceil() as i64;
        loop {
            let x = ride + k as f32 * gap;
            if x >= pw as f32 + 3.0 {
                break;
            }
            let xi = (x.round() as i32).clamp(0, pw as i32 - 1);
            let y = self.ys[li * pw + xi as usize].round() as i32;
            let c = if (k - 2 * laps) & 1 == 0 {
                self.shades[li * SHADES + s]
            } else {
                m.tints[li * SHADES + s]
            };
            blossom(&mut self.braille, x.round() as i32, y, spr, [c, FLAMA[1]]);
            k += 1;
        }
    }

    /// The veladoras on layer `li`, and their reflections under the mirrored trace.
    fn candles(&mut self, m: &Muertos, li: usize) {
        let fl = &m.flames[li];
        let center = self.ph as f32 * 0.5 - 0.5;
        for c in m.candles.iter().filter(|c| c.layer == li) {
            let sz = &SIZES[self.candle_size(li, c.lit)];
            // The flame sways and licks up and down.
            let flick = noise1(m.t * 5.0 + c.seed);
            let lean = flick * (0.7 + 0.14 * sz.flame_h as f32);
            let grow = rnd(noise1(m.t * 11.0 + c.seed * 1.7) * 1.2);
            let ax = c.x.div_euclid(2) * 2;
            let foot = self.foot(li, c.top);
            veladora(
                &mut self.braille,
                (ax, rnd(foot)),
                -1,
                sz,
                (lean, grow),
                fl,
                false,
            );
            if self.style == Style::Mirror {
                // As on still water: upside down and dimmer.
                let y1 = rnd(2.0 * center - foot);
                veladora(&mut self.braille, (ax, y1), 1, sz, (lean, grow), fl, true);
            }
        }
    }

    /// The loose petals tumbling, and the embers twinkling up.
    fn draw_loose(&mut self, m: &Muertos) {
        for p in &m.petals {
            let frame = (p.age * p.spin) as usize & 3;
            let x = (p.x + 1.5 * (p.age * 1.3 + p.sway).sin()).round() as i32;
            let y = p.y.round() as i32;
            if p.kind == 2 {
                // Embers wink out in their last second, and twinkle all along.
                let fade = ((1.0 - (p.life - p.age)) * 4.0).clamp(0.0, 3.0) as usize;
                if (p.age * p.spin + p.sway).sin() > -0.5 {
                    self.braille.set(x, y, m.loose[2][frame & 1][fade]);
                }
                continue;
            }
            let fade = ((1.5 - (p.life - p.age)) / 1.5 * 4.0).clamp(0.0, 3.0) as usize;
            let c = m.loose[p.kind][frame & 1][fade];
            for &(dx, dy) in TUMBLE[frame] {
                self.braille.set(x + dx, y + dy, c);
            }
        }
    }
}

/// A petal with its base at `(x, y)` pointing along `(ax, ay)`: it fills out
/// fast and tapers to a point, `reach` dots long and `wide` dots across at
/// its fullest (`prof` is its shape). Dots where `skip` says so are left out.
fn petal(
    b: &mut Braille,
    prof: &[f32; 32],
    (x, y): (f32, f32),
    (ax, ay): (f32, f32),
    (reach, wide): (f32, f32),
    c: Color,
    skip: impl Fn(i32, i32) -> bool,
) {
    let steps = (reach * 1.3) as usize;
    for i in 0..=steps {
        let f = i as f32 / steps.max(1) as f32;
        let w = prof[(f * 31.0) as usize] * wide;
        let (px, py) = (x + ax * f * reach, y + ay * f * reach);
        let jw = w as i32 + 1;
        for j in -jw..=jw {
            if j.abs() as f32 > w + 0.3 {
                continue;
            }
            let (dx, dy) = (rnd(px - ay * j as f32), rnd(py + ax * j as f32));
            if !skip(dx, dy) {
                b.set(dx, dy, c);
            }
        }
    }
}

/// A blossom of `spr` centered on dot (`cx`, `cy`): petals first, then the heart.
fn blossom(b: &mut Braille, cx: i32, cy: i32, spr: &[&str], cols: [Color; 2]) {
    let h = spr.len() as i32 / 2;
    for (pass, &want) in b"oc".iter().enumerate() {
        for (r, row) in spr.iter().enumerate() {
            let w = row.len() as i32 / 2;
            for (i, ch) in row.bytes().enumerate() {
                if ch == want {
                    b.set(cx + i as i32 - w, cy + r as i32 - h, cols[pass]);
                }
            }
        }
    }
}

/// A sugar skull of `spr` centered on dot (`cx`, `cy`), with a dark margin
/// round it so it stands out of the petals behind.
fn skull(b: &mut Braille, cx: i32, cy: i32, spr: &[&str], c: Color) {
    let (h, w) = (spr.len() as i32 / 2, spr[0].len() as i32 / 2);
    for dy in -h - 1..=h + 1 {
        for dx in -w - 1..=w + 1 {
            b.unset(cx + dx, cy + dy);
        }
    }
    for (r, row) in spr.iter().enumerate() {
        for (i, ch) in row.bytes().enumerate() {
            if ch == b'o' {
                b.set(cx + i as i32 - w, cy + r as i32 - h, c);
            }
        }
    }
}

/// A veladora standing on dot row `y` (its foot) at the cell column of dot
/// `ax`, an even one: the flame rises (`up` = -1) or, as a reflection, hangs
/// down (`up` = 1). The flame bends `lean` dots at its tip and `grow`s (or
/// shrinks) by that many. The shape is drawn dot by dot, but the colors follow
/// the cells, counted from the tip, so a cell never mixes two of them.
fn veladora(
    b: &mut Braille,
    (ax, y): (i32, i32),
    up: i32,
    sz: &Size,
    (lean, grow): (f32, i32),
    fl: &Flame,
    ghost: bool,
) {
    let glass = if ghost { fl.ghost } else { fl.glass };
    let h = (sz.flame_h + grow).max(3);
    let tip_row = (y + up * (sz.glass_h + h - 1)).div_euclid(4);
    let base_row = (y + up * sz.glass_h).div_euclid(4);
    let cell = ax.div_euclid(2);
    for k in 0..sz.glass_h {
        // The glass tapers a little toward its foot.
        let w = sz.glass_w - 2 * (k == 0 && sz.glass_w >= 6) as i32;
        let yy = y + up * k;
        // The cell the flame stands in is lit by it; the middle one shines.
        let lit = yy.div_euclid(4) == base_row;
        for i in 0..w {
            let x = ax + 1 - w / 2 + i;
            let c = match (lit, x.div_euclid(2) - cell) {
                (true, _) => glass[0],
                (_, 0) => glass[2],
                _ => glass[1],
            };
            b.set(x, yy, c);
        }
    }
    let cols = if ghost {
        [fl.tip, fl.edge, fl.edge.lerp(fl.core, 0.4), fl.edge]
    } else {
        [fl.tip, fl.edge, fl.core, fl.hot]
    };
    for r in 0..h {
        let u = r as f32 / (h - 1) as f32;
        // A teardrop: narrow at the wick, fullest low down, pointed at the tip.
        let hw = (sz.flame_w
            * 0.5
            * (PI * u.powf(0.6)).sin().powf(0.6)
            * (0.6 + 0.4 * (u * 5.0).min(1.0)))
        .max(0.5);
        let mid = ax as f32 + 0.5 + lean * u * u;
        let yy = y + up * (sz.glass_h + r);
        let row = yy.div_euclid(4);
        for x in (mid - hw).ceil() as i32..=(mid + hw).floor() as i32 {
            // The tip, the orange body, the gold heart and, low in the flame, a white-hot core.
            let c = match (row - tip_row, x.div_euclid(2) - cell) {
                (0, _) => cols[0],
                (_, 0) if row == base_row => cols[3],
                (_, 0) => cols[2],
                _ => cols[1],
            };
            b.set(x, yy, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::INFO;
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::render::Canvas;
    use crate::scene::{Opts, Scene};
    use crate::theme::Theme;

    /// A scene of `style` at 120×36 under `theme`, `secs` seconds in, and what it drew.
    fn run(theme: &Theme, style: &str, fiesta: Fiesta, secs: usize) -> (Waveform, Canvas) {
        let opts = Opts::new(INFO.options, format!("style = '{style}'").parse().unwrap());
        let ctx = Ctx {
            theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta,
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        for _ in 0..secs * 30 {
            s.update(&ctx, 1.0 / 30.0);
        }
        let mut c = Canvas::new(120, 36, Color::Reset);
        s.draw(&ctx, &mut c);
        (s, c)
    }

    fn colors(c: &Canvas) -> Vec<Color> {
        c.cells()
            .iter()
            .filter(|c| c.ch != ' ')
            .map(|c| c.fg)
            .collect()
    }

    /// Día de Muertos brings garlands in papel picado colors, veladoras on the
    /// crests with embers rising off them, and loose petals, from its first
    /// day; an ordinary day none of it.
    #[test]
    fn muertos_dresses_the_waves() {
        let classic = Theme::classic();
        for style in ["lines", "filled", "dots", "mirror"] {
            for day in [(10, 28), (11, 2)] {
                let (s, c) = run(&classic, style, Fiesta::of_day(day.0, day.1), 12);
                let m = s.muertos.as_ref().expect("dressed up");
                let on = colors(&c);
                assert!(
                    m.candles.iter().any(|c| c.lit > 0.9),
                    "{style} {day:?}: no candles"
                );
                assert!(
                    m.petals.iter().any(|p| p.kind < 2),
                    "{style} {day:?}: no petals"
                );
                assert!(
                    m.petals.iter().any(|p| p.kind == 2),
                    "{style} {day:?}: no embers"
                );
                // Flames in their own colors, and the front wave in cempasúchil.
                let front = m.flames[s.layers.len() - 1];
                for flame in [front.tip, front.edge] {
                    assert!(on.contains(&flame), "{style} {day:?}: no flame");
                }
                assert!(
                    on.contains(&s.shades[(s.layers.len() - 1) * SHADES])
                        || on.contains(&PICADO[0])
                );
            }
            let (s, c) = run(&classic, style, Fiesta::NONE, 12);
            assert!(s.muertos.is_none());
            let plain = colors(&c);
            for c in FLAMA.iter().chain(&[GLOW, HUESO]).chain(&PICADO) {
                assert!(
                    !plain.contains(c),
                    "{style}: holiday color on an ordinary day"
                );
            }
        }
        // A sugar skull rides the garlands now and then.
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta: Fiesta::of_day(11, 2),
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let mut c = Canvas::new(120, 36, Color::Reset);
        let mut skulls = 0;
        for _ in 0..300 {
            s.update(&ctx, 1.0 / 30.0);
            c.clear();
            s.draw(&ctx, &mut c);
            skulls += colors(&c).contains(&HUESO) as usize;
        }
        assert!(skulls > 150, "skulls on {skulls} of 300 frames");
        // More of everything as the holiday comes.
        let (early, _) = run(&classic, "lines", Fiesta::of_day(10, 28), 20);
        let (height, _) = run(&classic, "lines", Fiesta::of_day(11, 2), 20);
        let (e, h) = (early.muertos.unwrap(), height.muertos.unwrap());
        assert!(e.lit < h.lit);
        assert!(e.petals.len() < h.petals.len());
        assert!(e.candles.len() < h.candles.len());
    }

    /// The holiday comes and goes through `retheme`, and leaves the ordinary
    /// picture exactly as it was.
    #[test]
    fn muertos_comes_and_goes() {
        let theme = Theme::builtin("dracula").unwrap();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let mut ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: theme.bg,
            seed: 9,
            fiesta: Fiesta::NONE,
        };
        let frame = |s: &mut Waveform, ctx: &Ctx| {
            let mut c = Canvas::new(90, 30, Color::Reset);
            s.draw(ctx, &mut c);
            c.cells().to_vec()
        };
        let mut plain = Waveform::new(&ctx);
        let mut s = Waveform::new(&ctx);
        for w in [&mut plain, &mut s] {
            w.resize(&ctx, 90, 30);
            w.retheme(&ctx);
        }
        let before = frame(&mut s, &ctx);
        ctx.fiesta = Fiesta::of_day(11, 1);
        s.retheme(&ctx);
        for _ in 0..90 {
            s.update(&ctx, 1.0 / 30.0);
        }
        assert!(s.muertos.is_some());
        assert_ne!(frame(&mut s, &ctx), before);
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert!(s.muertos.is_none());
        for _ in 0..90 {
            plain.update(&ctx, 1.0 / 30.0);
        }
        assert!(frame(&mut s, &ctx) == frame(&mut plain, &ctx));
        // And back again, with a different day: the same scene, redressed.
        ctx.fiesta = Fiesta::of_day(10, 28);
        s.retheme(&ctx);
        assert_eq!(s.muertos.as_ref().map(|m| m.level), Some(0.25));
        ctx.fiesta = Fiesta::of_day(11, 2);
        s.retheme(&ctx);
        assert_eq!(s.muertos.as_ref().map(|m| m.level), Some(1.0));
    }

    /// The same seed paints the same holiday, and it follows the theme.
    #[test]
    fn muertos_is_deterministic_and_themed() {
        let day = Fiesta::of_day(11, 2);
        let nord = Theme::builtin("nord").unwrap();
        let (_, a) = run(&nord, "lines", day, 15);
        let (_, b) = run(&nord, "lines", day, 15);
        assert!(a.cells() == b.cells());
        let (_, classic) = run(&Theme::classic(), "lines", day, 15);
        assert!(a.cells() != classic.cells());
        // Under another theme the waves are the theme's, not papel picado.
        assert!(!colors(&a).contains(&PICADO[2]));
    }

    /// A resize drops the candles and petals of the old size and the next
    /// frames fill the new one.
    #[test]
    fn muertos_survives_a_resize() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: theme.bg,
            seed: 2,
            fiesta: Fiesta::of_day(11, 2),
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        for _ in 0..300 {
            s.update(&ctx, 1.0 / 30.0);
        }
        s.resize(&ctx, 40, 12);
        let m = s.muertos.as_ref().unwrap();
        assert!(m.candles.is_empty() && m.petals.is_empty());
        // Drawn before an update: the crests are found again from scratch.
        let mut c = Canvas::new(40, 12, Color::Reset);
        s.draw(&ctx, &mut c);
        for _ in 0..90 {
            s.update(&ctx, 1.0 / 30.0);
        }
        s.draw(&ctx, &mut c);
        assert!(!s.muertos.as_ref().unwrap().candles.is_empty());
    }
}
