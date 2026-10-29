//! metaballs: glossy blobs that merge like drops of gel.
//!
//! Adapted from the `metaballs` animation of tanim
//! (<https://github.com/SantiagoSchez/tanim>), which is under the MIT License:
//!
//! ```text
//! MIT License
//!
//! Copyright (c) 2026 Santiago Sánchez
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//! ```
//!
//! Every ball adds a field r²/d²; wherever the sum passes one a pixel is inside
//! the surface, so balls that approach grow a bridge and fuse. The field's exact
//! gradient gives each pixel a normal, so the drops are lit with a diffuse term
//! and a specular highlight, and a pixel takes the average of the ball colors
//! weighted by their contribution. tanim steps once per frame at 30 fps; here
//! the Lissajous motion is a function of continuous time at that same pace. The
//! space between blobs is transparent and their glow fades toward the terminal
//! background (`background = true` paints tanim's dark backdrop instead).
//!
//! On Día de Muertos the gel becomes veladora wax, in cempasúchil,
//! terciopelo, rosa mexicano, morado, turquesa and hueso, softer and lit
//! from above with a sheen the color of candlelight. The balls are candles:
//! a flickering flame stands on top of each (about half of them on October
//! 28, all of them on November 2), warming the wax around it, and candles
//! that fuse share their flames. Drops of wax swell from their rims, hang,
//! pinch off and fall, more of them as the holiday comes. One candle is a
//! calavera de azúcar with an icing face (two from October 31, three on
//! November 2). The backdrop, when painted, is the night of the vigil.

use crate::color::Color;
use crate::fiesta::{Fiesta, Kind};
use crate::render::{Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "metaballs",
    aliases: &["blobs", "gel"],
    about: "glossy blobs merging like drops of gel, after tanim",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast the blobs drift",
        },
        Opt {
            key: "blobs",
            default: Val::I(0),
            help: "Number of blobs (1-16, 0 = a random 5-8)",
        },
        Opt {
            key: "size",
            default: Val::F(1.0),
            help: "Blob size multiplier (0.4-2.5)",
        },
        Opt {
            key: "background",
            default: Val::B(false),
            help: "Paint a dark backdrop instead of the terminal's own",
        },
    ],
    make: |ctx| Box::new(Metaballs::new(ctx)),
};

type Rgb = [f32; 3];

/// tanim's five color sets of four blob colors each.
const SETS: [[Rgb; 4]; 5] = [
    [
        [255., 60., 140.],
        [255., 170., 40.],
        [90., 210., 255.],
        [170., 90., 255.],
    ],
    [
        [60., 240., 180.],
        [30., 150., 255.],
        [160., 255., 120.],
        [20., 200., 220.],
    ],
    [
        [255., 90., 40.],
        [255., 200., 60.],
        [230., 40., 60.],
        [255., 130., 90.],
    ],
    [
        [210., 215., 230.],
        [150., 170., 200.],
        [240., 200., 150.],
        [180., 190., 210.],
    ],
    [
        [250., 120., 200.],
        [120., 110., 250.],
        [90., 230., 240.],
        [250., 240., 130.],
    ],
];
const BG_TOP: Rgb = [10., 6., 24.];
const BG_BOT: Rgb = [24., 10., 36.];

/// Día de Muertos: five sets of colored veladora wax that take the place of
/// tanim's (cempasúchil, terciopelo and rosa mexicano, the papel picado
/// colors, morado and turquesa, hueso with marigold).
const WAX: [[Rgb; 4]; 5] = [
    [
        [255., 140., 0.],
        [255., 196., 40.],
        [214., 98., 0.],
        [255., 214., 10.],
    ],
    [
        [228., 0., 124.],
        [176., 0., 72.],
        [255., 140., 0.],
        [240., 84., 150.],
    ],
    [
        [228., 0., 124.],
        [0., 170., 184.],
        [255., 210., 0.],
        [112., 40., 160.],
    ],
    [
        [112., 40., 160.],
        [0., 170., 184.],
        [140., 200., 40.],
        [176., 0., 72.],
    ],
    [
        [244., 238., 226.],
        [255., 170., 30.],
        [228., 0., 124.],
        [255., 140., 0.],
    ],
];
/// The night of the vigil behind the candles (`background = true`).
const NOCHE_TOP: Rgb = [18., 14., 40.];
const NOCHE_BOT: Rgb = [40., 16., 34.];
/// The warm light a flame throws on the wax around it.
const FLAME_GLOW: Rgb = [255., 170., 60.];
/// At most this many drops of wax run at once.
const MAX_DRIPS: usize = 8;

/// Candle flames in four sizes, three flickers each (one char per pixel):
/// `r` the tip, `o` the edge, `y` the core, `w` white heat, `b` the blue
/// root, `k` the wick, which stands in the wax.
const FLAMES: [[&[&str]; 3]; 4] = [
    [
        &[".r.", ".o.", "oyo", "oyo", ".w.", ".k."],
        &["..r", ".o.", "oyo", "oyo", ".w.", ".k."],
        &["...", "r..", ".o.", "oyo", ".w.", ".k."],
    ],
    [
        &[
            "..r..", "..o..", ".oyo.", ".oyo.", "oywyo", "oywyo", ".ywy.", ".bkb.", "..k..",
        ],
        &[
            "...r.", "..oo.", ".oyo.", ".oyyo", "oywyo", "oywyo", ".ywy.", ".bkb.", "..k..",
        ],
        &[
            ".....", ".r...", ".oo..", ".oyo.", "oywyo", "oywyo", ".ywy.", ".bkb.", "..k..",
        ],
    ],
    [
        &[
            "...r...", "...o...", "..ooo..", "..oyo..", ".oyyyo.", ".oyyyo.", "oyywyyo", "oywwwyo",
            "oywwwyo", ".ywwwy.", ".bywyb.", "..bkb..", "...k...",
        ],
        &[
            "....r..", "....o..", "...oo..", "..oyo..", "..oyyo.", ".oyyyo.", ".oyywyo", "oywwwyo",
            "oywwwyo", ".ywwwy.", ".bywyb.", "..bkb..", "...k...",
        ],
        &[
            ".......", "..r....", "..oo...", "..oyo..", ".oyyo..", ".oyyyo.", "oyywyyo", "oywwwyo",
            "oywwwyo", ".ywwwy.", ".bywyb.", "..bkb..", "...k...",
        ],
    ],
    [
        &[
            "....r....",
            "....r....",
            "....o....",
            "...ooo...",
            "...oyo...",
            "..ooyoo..",
            "..oyyyo..",
            ".ooyyyoo.",
            ".oyywyyo.",
            ".oyywyyo.",
            "oyywwwyyo",
            "oywwwwwyo",
            "oywwwwwyo",
            ".oywwwyo.",
            ".bbywybb.",
            "..bbkbb..",
            "....k....",
        ],
        &[
            ".....r...",
            ".....o...",
            "....oo...",
            "....oyo..",
            "...oyyo..",
            "..ooyyo..",
            "..oyyyoo.",
            ".ooyywyo.",
            ".oyywyyo.",
            ".oyywwyo.",
            "oyywwwyyo",
            "oywwwwwyo",
            "oywwwwwyo",
            ".oywwwyo.",
            ".bbywybb.",
            "..bbkbb..",
            "....k....",
        ],
        &[
            ".........",
            ".........",
            "...r.....",
            "...oo....",
            "...oyo...",
            "..ooyo...",
            "..oyyyo..",
            ".ooyyyoo.",
            ".oyywyyo.",
            ".oyywyyo.",
            "oyywwwyyo",
            "oywwwwwyo",
            "oywwwwwyo",
            ".oywwwyo.",
            ".bbywybb.",
            "..bbkbb..",
            "....k....",
        ],
    ],
];
/// Flicker: which of the three flames shows, a few times a second.
const FLICKER: [u8; 12] = [0, 0, 1, 0, 2, 0, 0, 1, 1, 0, 2, 2];

/// Some candles are calaveras de azúcar: white wax with a face in icing,
/// in three sizes (`k` the sockets, nose and stitched smile, `p` rosa
/// mexicano and `t` turquesa around the eyes and in the flower on the
/// forehead with `o`/`y`, `g` verde limón on the cheeks).
const SKULLS: [&[&str]; 3] = [
    &[
        "...y...", ".t...p.", "tkt.pkp", ".t...p.", "...k...", ".k.k.k.", "..kkk..",
    ],
    &[
        "....p.p....",
        ".....o.....",
        "....p.p....",
        ".ttt...ppp.",
        "tkkkt.pkkkp",
        "tkkkt.pkkkp",
        ".ttt...ppp.",
        "....k.k....",
        "g....k....g",
        "..k.k.k.k..",
        ".kkkkkkkkk.",
        "..k.k.k.k..",
    ],
    &[
        "......p.p......",
        ".......o.......",
        "......p.p......",
        "...............",
        "..ttt.....ppp..",
        ".tkkkt...pkkkp.",
        "tkkkkkt.pkkkkkp",
        "tkkkkkt.pkkkkkp",
        ".tkkkt...pkkkp.",
        "..ttt.....ppp..",
        "......k.k......",
        "g......k......g",
        "...............",
        "..k.k.k.k.k.k..",
        ".kkkkkkkkkkkkk.",
        "..k.k.k.k.k.k..",
    ],
];
/// Sugar-skull white.
const HUESO: Rgb = [244., 238., 226.];

/// An icing pixel's color.
fn icing(ch: u8) -> Option<Rgb> {
    Some(match ch {
        b'k' => [70., 28., 62.],
        b'p' => [228., 0., 124.],
        b't' => [0., 170., 184.],
        b'o' => [255., 140., 0.],
        b'y' => [255., 210., 0.],
        b'g' => [140., 200., 40.],
        _ => return None,
    })
}

/// A flame pixel's color (flames keep their own hue under every theme).
fn flame_px(ch: u8) -> Option<Color> {
    Some(match ch {
        b'r' => Color::Rgb(232, 92, 20),
        b'o' => Color::Rgb(255, 150, 40),
        b'y' => Color::Rgb(255, 214, 120),
        b'w' => Color::Rgb(255, 248, 224),
        b'b' => Color::Rgb(96, 120, 232),
        b'k' => Color::Rgb(44, 30, 28),
        _ => return None,
    })
}
/// tanim's frame rate: its per-frame rates become per-second ones through this.
const FPS: f32 = 30.0;
/// Seconds a color set lasts and how long the colors take to blend into the next.
const SET_LEN: f32 = 800.0 / FPS;
const BLEND: f32 = 120.0 / FPS;

struct Ball {
    /// Lissajous frequencies (radians per frame) and phases.
    fx: f32,
    fy: f32,
    px: f32,
    py: f32,
    /// Radius as a fraction of the screen's smaller side.
    r: f32,
    slot: usize,
}

/// A drop of wax that runs down a candle, hangs, pinches off and falls.
struct Drip {
    /// The candle it melts from.
    ball: usize,
    /// While it hangs: its place relative to the candle's center.
    ox: f32,
    oy: f32,
    hanging: bool,
    /// Once it falls: where it is.
    x: f32,
    y: f32,
    /// Speed downward, pixels per second.
    v: f32,
    /// Radius as a fraction of its candle's.
    r: f32,
}

/// A flame on top of a candle this frame: the pixel its wick stands on,
/// its size and which candle it is (for the flicker).
#[derive(Clone, Copy)]
struct Flame {
    x: i32,
    y: i32,
    size: usize,
    id: usize,
    /// Its candle's radius in pixels.
    r: f32,
}

/// A ball's state for one frame.
#[derive(Clone, Copy, Default)]
struct B {
    x: f32,
    y: f32,
    r2: f32,
    col: Rgb,
}

fn rgb_of(c: Color) -> Rgb {
    match c {
        Color::Rgb(r, g, b) => [r as f32, g as f32, b as f32],
        _ => [200., 200., 200.],
    }
}

/// Sums every ball's field, gradient and color at row height `fy` and at every
/// second pixel (`hw` samples) into `out`, as six runs of `hw`. Ball by ball,
/// so the inner loop is simple enough to vectorize.
fn accumulate(live: &[B], fy: f32, hw: usize, out: &mut [f32]) {
    out.fill(0.0);
    let (rf, rest) = out.split_at_mut(hw);
    let (rgx, rest) = rest.split_at_mut(hw);
    let (rgy, rest) = rest.split_at_mut(hw);
    let (rc0, rest) = rest.split_at_mut(hw);
    let (rc1, rc2) = rest.split_at_mut(hw);
    for b in live {
        let dy = fy - b.y;
        let dy2 = dy * dy;
        for i in 0..hw {
            let dx = (2 * i) as f32 + 0.5 - b.x;
            let inv = 1.0 / (dx * dx + dy2).max(0.25);
            let v = b.r2 * inv;
            // d/dx of r²/d² is -2 r² dx / d⁴.
            let g = -2.0 * v * inv;
            rf[i] += v;
            rgx[i] += g * dx;
            rgy[i] += g * dy;
            rc0[i] += v * b.col[0];
            rc1[i] += v * b.col[1];
            rc2[i] += v * b.col[2];
        }
    }
}

/// A drop's field is 1.6 r²/(d² + 0.6 r²): as wide as a ball's, but round
/// at the core, so a small drop inside a candle leaves no dent in the light.
/// (strength, softness) for a squared radius.
fn soft(r2: f32) -> (f32, f32) {
    (1.6 * r2, 0.6 * r2 + 0.25)
}

/// Adds drops of wax to the sums like `accumulate`, but only where they
/// reach, so a few small drops cost little.
fn accumulate_near(drops: &[B], fy: f32, hw: usize, out: &mut [f32]) {
    let (rf, rest) = out.split_at_mut(hw);
    let (rgx, rest) = rest.split_at_mut(hw);
    let (rgy, rest) = rest.split_at_mut(hw);
    let (rc0, rest) = rest.split_at_mut(hw);
    let (rc1, rc2) = rest.split_at_mut(hw);
    for b in drops {
        let (k, c) = soft(b.r2);
        let reach = 8.0 * b.r2.sqrt() + 2.0;
        let dy = fy - b.y;
        if dy.abs() > reach {
            continue;
        }
        let dy2 = dy * dy;
        let i0 = (((b.x - reach) * 0.5).max(0.0) as usize).min(hw);
        let i1 = (((b.x + reach) * 0.5 + 1.0).max(0.0) as usize).min(hw);
        for i in i0..i1 {
            let dx = (2 * i) as f32 + 0.5 - b.x;
            let inv = 1.0 / (dx * dx + dy2 + c);
            let v = k * inv;
            let g = -2.0 * v * inv;
            rf[i] += v;
            rgx[i] += g * dx;
            rgy[i] += g * dy;
            rc0[i] += v * b.col[0];
            rc1[i] += v * b.col[1];
            rc2[i] += v * b.col[2];
        }
    }
}

/// The field of the balls and the drops at one point.
fn field(balls: &[B], drops: &[B], x: f32, y: f32) -> f32 {
    let d2 = |b: &B| (x - b.x).powi(2) + (y - b.y).powi(2);
    let balls: f32 = balls.iter().map(|b| b.r2 / d2(b).max(0.25)).sum();
    let drops: f32 = drops
        .iter()
        .map(|b| {
            let (k, c) = soft(b.r2);
            k / (d2(b) + c)
        })
        .sum();
    balls + drops
}

/// A sample run and the same run shifted by `par` samples, both `n` long.
fn pair(r: &[f32], par: usize, n: usize) -> (&[f32], &[f32]) {
    (&r[..n], &r[par..par + n])
}

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// Colors snap to steps of 4 so that a slowly shifting glow stays identical
/// from frame to frame and the renderer has fewer cells to send.
fn color(c: Rgb) -> Color {
    let q = |v: f32| ((v + 2.0) as u32 & !3).min(252) as u8;
    Color::Rgb(q(c[0]), q(c[1]), q(c[2]))
}

struct Metaballs {
    speed: f32,
    size: f32,
    backdrop: bool,
    rng: fastrand::Rng,
    balls: Vec<Ball>,
    /// Per-frame ball state, kept to avoid allocating every draw.
    live: Vec<B>,
    w: usize,
    ph: usize,
    px: Pixels,
    /// One row of field, gradient and color sums (six runs of `w`).
    acc: Vec<f32>,
    /// Per-pixel lighting of one row (seven runs of `2 * hw`: interior color, base color, field).
    shade: Vec<f32>,
    /// Motion time in tanim frames.
    t: f32,
    /// Wall-clock seconds, for the color schedule.
    clock: f32,
    /// How many color sets have already been shown.
    epoch: u32,
    set: usize,
    next: usize,
    /// Backdrop gradient, native or from the theme.
    bg_top: Rgb,
    bg_bot: Rgb,
    /// The four slot colors of the current and next set.
    cur: [Rgb; 4],
    nxt: [Rgb; 4],
    /// The holiday, as of the last `retheme`.
    fiesta: Fiesta,
    /// Middle and reach of the balls' vertical swing, as fractions of the
    /// height (lower for candles, to leave room for their flames).
    band: (f32, f32),
    /// Día de Muertos' own dice, so ordinary days roll exactly as before.
    wax_rng: fastrand::Rng,
    drips: Vec<Drip>,
    /// Per-frame state of the drops and the flames.
    drops: Vec<B>,
    flames: Vec<Flame>,
    /// Wall-clock seconds, for the flicker.
    flicker: f32,
    /// The calaveras' wax: hueso, or the theme's foreground.
    bone: Rgb,
}

impl Metaballs {
    fn new(ctx: &Ctx) -> Metaballs {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let n = match o.i("blobs") {
            n if n >= 1 => n.min(16) as usize,
            _ => rng.usize(5..9),
        };
        let balls = (0..n)
            .map(|i| Ball {
                fx: 0.004 + rng.f32() * 0.007,
                fy: 0.005 + rng.f32() * 0.008,
                px: rng.f32() * 6.3,
                py: rng.f32() * 6.3,
                r: 0.09 + rng.f32() * 0.08,
                slot: i % 4,
            })
            .collect();
        let set = rng.usize(0..SETS.len());
        let next = (set + 1 + rng.usize(0..SETS.len() - 1)) % SETS.len();
        let speed = o.f("speed");
        let size = o.f("size");
        Metaballs {
            speed: if speed.is_finite() {
                speed.clamp(0.0, 20.0)
            } else {
                1.0
            },
            size: if size.is_finite() {
                size.clamp(0.4, 2.5)
            } else {
                1.0
            },
            backdrop: o.b("background"),
            live: vec![B::default(); n],
            balls,
            w: 0,
            ph: 0,
            px: Pixels::new(1, 1),
            acc: Vec::new(),
            shade: Vec::new(),
            t: rng.f32() * 5000.0,
            clock: 0.0,
            epoch: 0,
            set,
            next,
            bg_top: BG_TOP,
            bg_bot: BG_BOT,
            cur: SETS[set],
            nxt: SETS[next],
            rng,
            fiesta: Fiesta::NONE,
            band: (0.5, 0.38),
            wax_rng: fastrand::Rng::with_seed(ctx.seed ^ 0x7e1a_d0a5_c1e5),
            drips: Vec::with_capacity(MAX_DRIPS),
            drops: Vec::with_capacity(MAX_DRIPS),
            flames: Vec::with_capacity(n),
            flicker: 0.0,
            bone: HUESO,
        }
    }

    /// Slot colors of color set `s`: tanim's own (or, on Día de Muertos,
    /// colored wax), or the theme's accents rotated by one per set so the
    /// blobs still change hands over time.
    fn slots(ctx: &Ctx, s: usize) -> [Rgb; 4] {
        let sets = if ctx.fiesta.is(Kind::Muertos) {
            &WAX
        } else {
            &SETS
        };
        let mut out = sets[s];
        for (i, c) in out.iter_mut().enumerate() {
            *c = rgb_of(ctx.pick(i + s, color(sets[s][i])));
        }
        out
    }

    /// Where a ball is now, in pixels.
    fn center(&self, b: &Ball) -> (f32, f32) {
        let (wf, phf) = (self.w as f32, self.ph as f32);
        (
            wf * (0.5 + 0.4 * (self.t * b.fx + b.px).sin()),
            phf * (self.band.0 + self.band.1 * (self.t * b.fy + b.py).sin()),
        )
    }

    /// A ball's radius in pixels.
    fn radius(&self, b: &Ball) -> f32 {
        let side = (self.w as f32).min(self.ph as f32).max(1.0);
        b.r * self.size * side
    }

    /// How many of the balls are candles with a flame: about half of them
    /// on October 28, all of them on November 2.
    fn candles(&self) -> usize {
        if !self.fiesta.is(Kind::Muertos) {
            return 0;
        }
        let n = self.balls.len();
        ((n as f32 * (0.35 + 0.65 * self.fiesta.level)).ceil() as usize).clamp(1, n)
    }

    /// Is ball `i` a calavera? One from the first day; two from October 31,
    /// three on November 2 (every third ball, and at most a third of them).
    fn calavera(&self, i: usize) -> bool {
        if !self.fiesta.is(Kind::Muertos) || !i.is_multiple_of(3) {
            return false;
        }
        let level = self.fiesta.level;
        let n = 1 + (level >= 0.6) as usize + (level >= 1.0) as usize;
        i / 3 < n.min(self.balls.len().div_ceil(3))
    }

    /// The candles melt: drops of wax run down their sides, hang, pinch off
    /// and fall (`ds` seconds of motion).
    fn melt(&mut self, ds: f32) {
        let phf = self.ph as f32;
        let level = self.fiesta.level;
        let candles = self.candles();
        let cap = ((2.0 + 6.0 * level) as usize).min(MAX_DRIPS);
        let mut left = ds;
        while left > 0.0 {
            let h = left.min(1.0 / FPS);
            left -= h;
            for ball in 0..candles {
                if self.drips.len() < cap && self.wax_rng.f32() < (0.04 + 0.12 * level) * h {
                    self.drips.push(Drip {
                        ball,
                        ox: (self.wax_rng.f32() - 0.5) * 0.9,
                        oy: 0.8,
                        hanging: true,
                        x: 0.0,
                        y: 0.0,
                        v: 0.1,
                        r: 0.26 + self.wax_rng.f32() * 0.1,
                    });
                }
            }
            for i in 0..self.drips.len() {
                let d = &self.drips[i];
                let b = &self.balls[d.ball];
                let (r, (cx, cy)) = (self.radius(b), self.center(b));
                let d = &mut self.drips[i];
                if d.hanging {
                    // Wax creeps down slowly (in radii per second), then lets go.
                    d.v += 0.3 * h;
                    d.oy += d.v * h;
                    if d.oy > 1.4 {
                        d.hanging = false;
                        (d.x, d.y) = (cx + d.ox * r, cy + d.oy * r);
                        d.v *= r;
                    }
                } else {
                    d.v = (d.v + 0.5 * phf * h).min(0.8 * phf);
                    d.y += d.v * h;
                    d.r *= 1.0 - 0.25 * h;
                    if d.y - d.r * r > phf + 2.0 {
                        d.r = 0.0;
                    }
                }
            }
            self.drips.retain(|d| d.r > 0.05);
        }
    }

    /// Lights the candles: finds where each flame stands, warms the wax
    /// around it and draws it.
    fn candlelight(&mut self, ctx: &Ctx) {
        let ph = self.ph as i32;
        self.flames.clear();
        for id in 0..self.candles() {
            let b = self.live[id];
            let r = b.r2.sqrt();
            let x = b.x.floor();
            // Climb from the ball's center to the top of the wax around it.
            let mut y = b.y.floor();
            let mut steps = 0;
            while steps < ph && field(&self.live, &self.drops, x + 0.5, y - 0.5) >= 1.0 {
                y -= 1.0;
                steps += 1;
            }
            let (x, y) = (x as i32, y as i32);
            // Fused candles share one flame.
            if self
                .flames
                .iter()
                .any(|f| (f.x - x).abs() < 10 && (f.y - y).abs() < 6)
            {
                continue;
            }
            let size = match r {
                r if r < 5.0 => 0,
                r if r < 9.0 => 1,
                r if r < 15.0 => 2,
                _ => 3,
            };
            self.flames.push(Flame { x, y, size, id, r });
        }

        // The calaveras' faces, shaded like the wax they are drawn on.
        let lum = |c: Rgb| 0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2];
        let bone = lum(self.bone).max(1.0);
        for i in 0..self.balls.len() {
            let b = self.live[i];
            let r = b.r2.sqrt();
            if !self.calavera(i) || r < 4.5 {
                continue;
            }
            let face = SKULLS[(r >= 8.0) as usize + (r >= 14.0) as usize];
            let (left, top) = (
                b.x as i32 - face[0].len() as i32 / 2,
                b.y as i32 - face.len() as i32 / 2,
            );
            for (j, row) in face.iter().enumerate() {
                for (k, ch) in row.bytes().enumerate() {
                    let (x, y) = (left + k as i32, top + j as i32);
                    let (Some(ic), Color::Rgb(..)) = (icing(ch), self.px.get(x, y)) else {
                        continue;
                    };
                    let shade = (lum(rgb_of(self.px.get(x, y))) / bone * 1.1).clamp(0.5, 1.0);
                    self.px
                        .set(x, y, color([ic[0] * shade, ic[1] * shade, ic[2] * shade]));
                }
            }
        }

        // The glow fades into the backdrop or the terminal's known background;
        // over a transparent terminal it only warms the wax.
        let known = match ctx.bg {
            Color::Rgb(..) => Some(rgb_of(ctx.bg)),
            _ => None,
        };
        let tick = (self.flicker * 7.0) as usize;
        let sprite =
            |f: &Flame| FLAMES[f.size][FLICKER[(tick + f.id * 5) % FLICKER.len()] as usize];
        for f in &self.flames {
            let fh = sprite(f).len() as i32;
            let (gx, gy) = (f.x as f32 + 0.5, (f.y - fh / 2) as f32);
            // The flame warms the wax down to its candle's middle, and the
            // air only close around it.
            let reach = (fh as f32 * 1.8 + 2.0).max(f.r * 1.3);
            let air = fh as f32 * 1.5 + 2.0;
            let k = self.flicker + f.id as f32 * 1.7;
            let strength = 0.8 + 0.2 * (k * 11.0).sin() * (k * 6.3).sin();
            let ri = reach as i32 + 1;
            for py in (gy as i32 - ri).max(0)..(gy as i32 + ri).min(ph) {
                let dy2 = (py as f32 + 0.5 - gy).powi(2);
                // Only the span of this row inside the circle.
                let span = (reach * reach - dy2).max(0.0).sqrt() as i32 + 1;
                for px in (f.x - span).max(0)..(f.x + span).min(self.w as i32) {
                    let d = ((px as f32 + 0.5 - gx).powi(2) + dy2).sqrt();
                    if d >= reach {
                        continue;
                    }
                    let g = (1.0 - d / reach).powi(2) * strength;
                    let lit = match self.px.get(px, py) {
                        Color::Reset => {
                            let g = (1.0 - d / air).max(0.0).powi(2) * strength * 0.22;
                            match known {
                                Some(bg) if g >= 0.04 => mix(bg, FLAME_GLOW, g),
                                _ => continue,
                            }
                        }
                        c => {
                            let c = rgb_of(c);
                            let (a, b) = (1.0 + 0.4 * g, 0.12 * g);
                            [
                                (c[0] * a + FLAME_GLOW[0] * b).min(255.0),
                                (c[1] * a + FLAME_GLOW[1] * b).min(255.0),
                                (c[2] * a + FLAME_GLOW[2] * b).min(255.0),
                            ]
                        }
                    };
                    self.px.set(px, py, color(lit));
                }
            }
        }
        for f in &self.flames {
            let sprite = sprite(f);
            let (left, top) = (
                f.x - sprite[0].len() as i32 / 2,
                f.y + 1 - sprite.len() as i32,
            );
            for (j, row) in sprite.iter().enumerate() {
                for (i, ch) in row.bytes().enumerate() {
                    if let Some(c) = flame_px(ch) {
                        self.px.set(left + i as i32, top + j as i32, c);
                    }
                }
            }
        }
    }
}

impl Scene for Metaballs {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        let (w, h) = (w.max(1), h.max(1));
        self.w = w as usize;
        self.ph = 2 * h as usize;
        self.px.resize(w, h);
        self.acc.resize(6 * (self.w / 2 + 2), 0.0);
        self.shade.resize(14 * (self.w / 2 + 2), 0.0);
        self.drips.clear();
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.fiesta = ctx.fiesta;
        self.band = match self.fiesta.is(Kind::Muertos) {
            true => (0.6, 0.3),
            false => (0.5, 0.38),
        };
        if !self.fiesta.is(Kind::Muertos) {
            self.drips.clear();
        }
        self.cur = Self::slots(ctx, self.set);
        self.nxt = Self::slots(ctx, self.next);
        if ctx.theme.native && self.fiesta.is(Kind::Muertos) {
            (self.bg_top, self.bg_bot) = (NOCHE_TOP, NOCHE_BOT);
        } else if ctx.theme.native {
            (self.bg_top, self.bg_bot) = (BG_TOP, BG_BOT);
        } else {
            let b = rgb_of(ctx.theme.bg);
            (self.bg_top, self.bg_bot) = (b, b);
        }
        self.bone = match ctx.theme.native {
            true => HUESO,
            false => rgb_of(ctx.theme.fg),
        };
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        if dt.is_finite() {
            self.t += dt.clamp(0.0, 5.0) * FPS * self.speed;
            if self.fiesta.is(Kind::Muertos) {
                self.melt(dt.clamp(0.0, 5.0) * self.speed);
            }
        }
    }

    /// Motion follows the speed setting; the color schedule keeps real time.
    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        if !real.is_finite() {
            return;
        }
        self.clock += real.clamp(0.0, 5.0);
        self.flicker = (self.flicker + real.clamp(0.0, 5.0)) % 3600.0;
        let epoch = (self.clock / SET_LEN) as u32;
        if epoch != self.epoch {
            self.epoch = epoch;
            self.set = self.next;
            self.next = (self.set + 1 + self.rng.usize(0..SETS.len() - 1)) % SETS.len();
            self.cur = Self::slots(ctx, self.set);
            self.nxt = Self::slots(ctx, self.next);
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let (w, ph) = (self.w, self.ph);
        let (wf, phf) = (w as f32, ph as f32);
        let hw = w / 2 + 2;
        let side = wf.min(phf).max(1.0);

        // Crossfade the ball colors toward the next set at the end of each one.
        let into = self.clock % SET_LEN;
        let blend = ((into - (SET_LEN - BLEND)) / BLEND).clamp(0.0, 1.0);
        for i in 0..self.balls.len() {
            let b = &self.balls[i];
            let (x, y) = self.center(b);
            self.live[i] = B {
                x,
                y,
                r2: (b.r * self.size * side).powi(2),
                col: match self.calavera(i) {
                    true => self.bone,
                    false => mix(self.cur[b.slot], self.nxt[b.slot], blend),
                },
            };
        }
        self.drops.clear();
        for d in &self.drips {
            let p = self.live[d.ball];
            let r = p.r2.sqrt();
            // A hanging drop swells out of the candle's rim.
            let (x, y, swell) = match d.hanging {
                true => (p.x + d.ox * r, p.y + d.oy * r, (d.oy - 0.8) / 0.3),
                false => (d.x, d.y, 1.0),
            };
            self.drops.push(B {
                x,
                y,
                r2: (d.r * swell.clamp(0.0, 1.0) * r).powi(2),
                col: p.col,
            });
        }

        // Gel is lit from the upper left, towards the viewer, with a white
        // highlight; candle wax from above, softer and warmer, with a sheen
        // the color of the flames.
        let wax = self.fiesta.is(Kind::Muertos);
        let (lx, ly, lz) = match wax {
            false => (-0.45f32, -0.55f32, 0.70f32),
            true => (-0.25f32, -0.75f32, 0.62f32),
        };
        let (amb, dif, ka, kb, sp) = match wax {
            false => (0.25, 0.85, 0.85, 0.0, [255.0f32; 3]),
            true => (0.45, 0.62, 0.45, 0.25, [255.0, 226.0, 170.0]),
        };
        let norm = (lx * lx + ly * ly + lz * lz).sqrt();
        let (lx, ly, lz) = (lx / norm, ly / norm, lz / norm);
        // Halfway vector between the light and the viewer, for highlights.
        let (hx, hy, hz) = (lx, ly, lz + 1.0);
        let hn = (hx * hx + hy * hy + hz * hz).sqrt();
        let (hx, hy, hz) = (hx / hn, hy / hn, hz / hn);
        // How steeply the domes curve at their rims (wax is softer than gel).
        let curve = if wax { 0.08 } else { 0.13 };
        let steep = side * curve * self.size;

        // The glow fades into the terminal's background when it is known; with
        // a transparent terminal of unknown color a fade could only darken, so
        // the glow is cut off cleanly instead.
        let known = match ctx.bg {
            Color::Rgb(..) => Some(rgb_of(ctx.bg)),
            _ => None,
        };

        // Smallest field that still paints anything (see the glow below).
        let min_f = match (self.backdrop, known) {
            (true, _) => 0.0,
            (false, Some(_)) => 0.41,
            (false, None) => 0.63,
        };

        self.px.clear();
        for y in 0..ph {
            let fy = y as f32 + 0.5;
            let backdrop = self
                .backdrop
                .then(|| mix(self.bg_top, self.bg_bot, fy / phf));
            // The balls are summed at every second pixel (see `accumulate`):
            // field, gradient and colors vary smoothly over two pixels, so the
            // pixels in between are averages.
            let st = 6 * hw;
            accumulate(&self.live, fy, hw, &mut self.acc[..st]);
            accumulate_near(&self.drops, fy, hw, &mut self.acc[..st]);
            let src = &self.acc[..st];
            let (rf, rest) = src.split_at(hw);
            let (rgx, rest) = rest.split_at(hw);
            let (rgy, rest) = rest.split_at(hw);
            let (rc0, rest) = rest.split_at(hw);
            let (rc1, rc2) = rest.split_at(hw);
            // Light both pixels of each sample, in branch-free loops that
            // vectorize: parity 0 is the sample itself, parity 1 the average
            // with the next one. Each parity gets its own runs of `hw`.
            let mut chunks = self.shade.chunks_exact_mut(hw);
            let mut next = || chunks.next().unwrap();
            let mut lit: [[&mut [f32]; 7]; 2] = [
                [next(), next(), next(), next(), next(), next(), next()],
                [next(), next(), next(), next(), next(), next(), next()],
            ];
            let n = hw - 1;
            for (par, [ro0, ro1, ro2, rb0, rb1, rb2, rff]) in lit.iter_mut().enumerate() {
                let t = 0.5 * par as f32;
                let l = |r| pair(r, par, n);
                let ((f0, f1), (x0, x1), (y0, y1)) = (l(rf), l(rgx), l(rgy));
                let ((c00, c01), (c10, c11), (c20, c21)) = (l(rc0), l(rc1), l(rc2));
                for i in 0..n {
                    let f = f0[i] + (f1[i] - f0[i]) * t;
                    let gx = x0[i] + (x1[i] - x0[i]) * t;
                    let gy = y0[i] + (y1[i] - y0[i]) * t;
                    let inv = 1.0 / f.max(1e-6);
                    let k2 = -inv * inv * steep;
                    let (nx, ny) = (gx * k2, gy * k2);
                    let rn = 1.0 / (nx * nx + ny * ny + 1.0).sqrt();
                    let (nx, ny, nz) = (nx * rn, ny * rn, rn);
                    let diff = (nx * lx + ny * ly + nz * lz).max(0.0);
                    let s = (nx * hx + ny * hy + nz * hz).max(0.0);
                    // s^24 without powi.
                    let s4 = s * s * s * s;
                    let s12 = s4 * s4 * s4;
                    let lit = amb + dif * diff;
                    let k = s12 * s12 * ka + s12 * kb;
                    let b0 = (c00[i] + (c01[i] - c00[i]) * t) * inv;
                    let b1 = (c10[i] + (c11[i] - c10[i]) * t) * inv;
                    let b2 = (c20[i] + (c21[i] - c20[i]) * t) * inv;
                    rb0[i] = b0;
                    rb1[i] = b1;
                    rb2[i] = b2;
                    ro0[i] = b0 * lit + (sp[0] - b0 * lit) * k;
                    ro1[i] = b1 * lit + (sp[1] - b1 * lit) * k;
                    ro2[i] = b2 * lit + (sp[2] - b2 * lit) * k;
                    rff[i] = f;
                }
            }
            for x in 0..w {
                let [ro0, ro1, ro2, rb0, rb1, rb2, rff] = &lit[x & 1];
                let j = x / 2;
                let f = rff[j];
                // Too faint to show: the glow is f³ * 0.55.
                if f < min_f {
                    continue;
                }
                let base = [rb0[j], rb1[j], rb2[j]];
                let out = if f >= 1.0 {
                    // Height h = 1 - 1/f: flat on top of each drop and steep at
                    // its rim, like a dome (normal from the field's gradient).
                    Some([ro0[j], ro1[j], ro2[j]])
                } else {
                    // Outside: a soft glow that thickens near the surface.
                    let glow = f * f * f * 0.55;
                    match (backdrop, known) {
                        (Some(bg), _) => Some(mix(bg, base, glow)),
                        (None, Some(bg)) if glow >= 0.04 => Some(mix(bg, base, glow)),
                        (None, None) if glow >= 0.14 => {
                            let k = 0.4 + glow;
                            Some([base[0] * k, base[1] * k, base[2] * k])
                        }
                        _ => None,
                    }
                };
                match (out, backdrop) {
                    (Some(p), _) => self.px.set(x as i32, y as i32, color(p)),
                    (None, Some(bg)) => self.px.set(x as i32, y as i32, color(bg)),
                    _ => {}
                }
            }
        }
        if wax {
            self.candlelight(ctx);
        }
        self.px.blit(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::Theme;

    fn ctx_with<'a>(theme: &'a Theme, opts: &'a Opts, bg: Color) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg,
            seed: 3,
            fiesta: Fiesta::NONE,
        }
    }

    /// Odd options and sizes survive long runs and huge steps, in every color
    /// mode, on ordinary days and on Día de Muertos.
    #[test]
    fn variants_survive() {
        let variants = [
            "",
            "blobs = 16\nsize = 2.5\nbackground = true",
            "blobs = 1\nsize = 0.4\nspeed = 20.0",
            "blobs = -3\nsize = nan\nspeed = inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                for (bg, fiesta) in [Color::Reset, theme.bg].into_iter().flat_map(|bg| {
                    [Fiesta::NONE, Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)].map(|f| (bg, f))
                }) {
                    let opts = Opts::new(INFO.options, v.parse().unwrap());
                    let ctx = Ctx {
                        fiesta,
                        ..ctx_with(&theme, &opts, bg)
                    };
                    let mut s = (INFO.make)(&ctx);
                    for &(w, h) in &[(80, 24), (1, 1), (2, 3), (7, 2), (200, 60)] {
                        s.resize(&ctx, w, h);
                        s.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for f in 0..30 {
                            let dt = if f % 11 == 10 { 40.0 } else { 1.0 / 30.0 };
                            s.tick(&ctx, dt, dt);
                            c.clear();
                            s.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    /// By default the space between blobs is transparent, and some of the
    /// screen is blob.
    #[test]
    fn background_is_transparent() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = ctx_with(&theme, &opts, Color::Reset);
        let mut s = Metaballs::new(&ctx);
        s.resize(&ctx, 100, 30);
        s.retheme(&ctx);
        let mut c = Canvas::new(100, 30, Color::Reset);
        s.draw(&ctx, &mut c);
        let lit = c.cells().iter().filter(|cell| cell.ch != ' ').count();
        assert!(lit > 0 && lit < 100 * 30, "{lit} lit cells");
    }

    /// With `background = true` every cell is painted.
    #[test]
    fn backdrop_fills_the_screen() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, "background = true".parse().unwrap());
        let ctx = ctx_with(&theme, &opts, Color::Reset);
        let mut s = Metaballs::new(&ctx);
        s.resize(&ctx, 40, 12);
        s.retheme(&ctx);
        let mut c = Canvas::new(40, 12, Color::Reset);
        s.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch != ' '));
    }

    /// A frame after `secs` seconds at 120x36, as the scene and its canvas.
    fn frame(ctx: &Ctx, secs: usize) -> (Metaballs, Canvas) {
        let mut s = Metaballs::new(ctx);
        s.resize(ctx, 120, 36);
        s.retheme(ctx);
        let mut c = Canvas::new(120, 36, Color::Reset);
        for _ in 0..secs * 10 {
            s.tick(ctx, 0.1, 0.1);
        }
        s.draw(ctx, &mut c);
        (s, c)
    }

    /// Flame pixels: wax and gel snap to steps of 4 and never reach 255.
    fn flames(c: &Canvas) -> usize {
        let lit = |col: Color| matches!(col, Color::Rgb(r, g, _) if r == 255 || g == 255);
        c.cells().iter().filter(|k| lit(k.fg) || lit(k.bg)).count()
    }

    /// On Día de Muertos the balls are candles: lit, melting, some of them
    /// calaveras; from the first day to the height of the holiday.
    #[test]
    fn muertos_lights_the_candles() {
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            let opts = Opts::new(INFO.options, toml::Table::new());
            for (day, n) in [((10, 28), 1), ((11, 2), 3)] {
                let ctx = Ctx {
                    fiesta: Fiesta::of_day(day.0, day.1),
                    ..ctx_with(&theme, &opts, Color::Reset)
                };
                let (s, c) = frame(&ctx, 6);
                assert!(flames(&c) > 0, "no flames on {day:?}");
                assert!(!s.flames.is_empty() && s.candles() >= s.balls.len() / 2);
                let skulls = (0..s.balls.len()).filter(|&i| s.calavera(i)).count();
                assert_eq!(skulls, n.min(s.balls.len().div_ceil(3)), "{day:?}");
                if theme.native {
                    assert_eq!(s.live[0].col, HUESO);
                }
            }
            // Ordinary days: no flames, no drops, the gel as always.
            let ctx = ctx_with(&theme, &opts, Color::Reset);
            let (s, c) = frame(&ctx, 6);
            assert_eq!(flames(&c), 0);
            assert!(s.flames.is_empty() && s.drips.is_empty() && s.candles() == 0);
            assert!(!s.calavera(0));
        }
    }

    /// Wax runs down the candles, never more than `MAX_DRIPS` drops at once.
    #[test]
    fn candles_drip() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, "blobs = 16".parse().unwrap());
        let ctx = Ctx {
            fiesta: Fiesta::of_day(11, 2),
            ..ctx_with(&theme, &opts, Color::Reset)
        };
        let mut s = Metaballs::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let (mut most, mut fell) = (0, false);
        for _ in 0..1200 {
            s.tick(&ctx, 0.05, 0.05);
            most = most.max(s.drips.len());
            fell |= s.drips.iter().any(|d| !d.hanging);
        }
        assert!(most > 0 && most <= MAX_DRIPS && fell, "{most} drops");
    }

    /// The holiday comes and goes through `retheme`, and goes without a trace.
    #[test]
    fn holiday_switches_on_and_off() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let none = ctx_with(&theme, &opts, Color::Reset);
        let muertos = Ctx {
            fiesta: Fiesta::of_day(11, 1),
            ..ctx_with(&theme, &opts, Color::Reset)
        };
        let mut s = Metaballs::new(&none);
        s.resize(&none, 100, 30);
        s.retheme(&none);
        let mut c = Canvas::new(100, 30, Color::Reset);
        let run = |s: &mut Metaballs, ctx: &Ctx, c: &mut Canvas| {
            for _ in 0..60 {
                s.tick(ctx, 0.1, 0.1);
            }
            c.clear();
            s.draw(ctx, c);
            flames(c)
        };
        assert_eq!(run(&mut s, &none, &mut c), 0);
        s.retheme(&muertos);
        assert!(run(&mut s, &muertos, &mut c) > 0);
        assert_eq!(s.cur, Metaballs::slots(&muertos, s.set));
        s.retheme(&none);
        assert!(s.drips.is_empty());
        assert_eq!(s.cur, Metaballs::slots(&none, s.set));
        assert_eq!(run(&mut s, &none, &mut c), 0);
    }

    /// The same seed gives the same candles, drops and flicker.
    #[test]
    fn muertos_is_deterministic() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            fiesta: Fiesta::of_day(11, 2),
            ..ctx_with(&theme, &opts, Color::Rgb(13, 17, 23))
        };
        let (_, a) = frame(&ctx, 20);
        let (_, b) = frame(&ctx, 20);
        assert!(a.cells() == b.cells());
    }
}
