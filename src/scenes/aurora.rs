//! aurora: northern lights over mountains, mirrored in a still lake.
//!
//! Adapted from tanim's `aurora` <https://github.com/SantiagoSchez/tanim>,
//! licensed MIT. An aurora is a thin sheet of light seen almost edge on, so
//! each curtain is a wavy line (its track across the sky) walked in small
//! steps, every sample dropping light into the column it lands on. Where the
//! line folds back on itself many samples pile into the same columns, which is
//! where real curtains look brightest. Each column then glows upwards from the
//! curtain's sharp lower edge. The ridges, the pine forest, the stars, the
//! shooting stars, the substorms and the lake follow tanim; the night sky and
//! the lake are transparent here by default (`sky = true` paints tanim's).
//!
//! On top of tanim's night the sky changes its look every so often, fading
//! from one to the next: tanim's own green, teal and violet, an emerald
//! night, a crimson storm, ultraviolet, ice under a crescent moon, synthwave
//! with neon ridges, embers, a prism whose curtains run through every hue,
//! and now and then Van Gogh's starry night, with the sky whirling in
//! brushstrokes, ringed stars, a crescent moon, a cypress and a village. Now
//! and then Nyan Cat or sl's locomotive goes by (the easter eggs, sent by the
//! scene itself).
//!
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

use std::f32::consts::PI;

use crate::color::{Color, ansi_rgb};
use crate::eggs::{self, Egg};
use crate::render::{Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use crate::theme::Theme;

pub const INFO: SceneInfo = SceneInfo {
    name: "aurora",
    aliases: &["northern-lights", "borealis"],
    about: "northern lights over mountains, mirrored in a lake, after tanim",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast the lights ripple and drift (0.1-4)",
        },
        Opt {
            key: "curtains",
            default: Val::I(4),
            help: "Curtains of light in the sky at once (1-8)",
        },
        Opt {
            key: "look",
            default: Val::S("cycle"),
            help: "cycle, or one look to stay on: tanim, emerald, crimson, ultraviolet, ice, synthwave, ember, prism, starry",
        },
        Opt {
            key: "stay",
            default: Val::F(90.0),
            help: "Seconds each look lasts while cycling (10-3600)",
        },
        Opt {
            key: "landscape",
            default: Val::B(true),
            help: "Mountains, a pine forest and the lake that mirrors the sky",
        },
        Opt {
            key: "stars",
            default: Val::B(true),
            help: "Twinkling stars and the odd shooting star",
        },
        Opt {
            key: "visitors",
            default: Val::F(3.0),
            help: "Times per hour Nyan Cat or sl's train goes by, 0 = never",
        },
        Opt {
            key: "sky",
            default: Val::B(false),
            help: "Paint the night sky and the lake instead of leaving them transparent",
        },
    ],
    make: |ctx| Box::new(Aurora::new(ctx)),
};

/// Color as red, green, blue in 0..1 (more than 1 is allowed while adding light).
type V3 = [f32; 3];

const fn c8(r: u8, g: u8, b: u8) -> V3 {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}

/// How much brighter the land is drawn over a transparent sky, which is
/// usually lighter than tanim's near-black one.
const LIFT: f32 = 1.8;
/// tanim's tick rate for this animation; one simulation step is this long.
const STEP: f32 = 1.0 / 20.0;
/// Steps per pixel in a curtain's glow profile.
const PROFILE_RES: f32 = 8.0;
/// Seconds one look takes to fade into the next.
const BLEND: f32 = 12.0;
/// A visitor still crossing after this long is sent away.
const VISIT_SECS: f32 = 40.0;

/// One look of the night: its colors, and how much each effect shows.
#[derive(Clone, Copy)]
struct Look {
    /// Aurora colors from the lower edge upwards.
    glow: [V3; 4],
    sky_top: V3,
    sky_low: V3,
    far: V3,
    near: V3,
    snow: V3,
    pine: V3,
    water: V3,
    star: V3,
    glint: V3,
    /// A bright outline along the far ridge.
    rim: V3,
    rim_k: f32,
    /// A crescent moon high on the right.
    moon: V3,
    moon_k: f32,
    /// How bright the curtains are, how much they run through every hue, and
    /// how much of Van Gogh's starry night shows.
    lights: f32,
    prism: f32,
    starry: f32,
}

/// tanim's night.
const TANIM: Look = Look {
    glow: [
        c8(90, 255, 150),
        c8(40, 220, 170),
        c8(120, 90, 230),
        c8(210, 60, 120),
    ],
    sky_top: c8(2, 3, 12),
    sky_low: c8(10, 20, 36),
    far: c8(18, 24, 42),
    near: c8(8, 11, 20),
    snow: c8(58, 70, 92),
    pine: c8(4, 6, 10),
    water: c8(3, 7, 16),
    star: c8(230, 235, 255),
    glint: c8(150, 190, 220),
    rim: c8(0, 0, 0),
    rim_k: 0.0,
    moon: c8(240, 236, 214),
    moon_k: 0.0,
    lights: 1.0,
    prism: 0.0,
    starry: 0.0,
};

const LOOKS: [(&str, Look); 9] = [
    ("tanim", TANIM),
    (
        "emerald",
        Look {
            glow: [
                c8(140, 255, 150),
                c8(60, 235, 110),
                c8(30, 170, 110),
                c8(200, 90, 170),
            ],
            sky_low: c8(6, 24, 30),
            moon_k: 0.7,
            ..TANIM
        },
    ),
    (
        "crimson",
        Look {
            glow: [
                c8(255, 110, 120),
                c8(235, 45, 90),
                c8(160, 30, 120),
                c8(90, 20, 100),
            ],
            sky_top: c8(6, 2, 10),
            sky_low: c8(32, 8, 26),
            far: c8(36, 18, 34),
            near: c8(14, 6, 14),
            snow: c8(96, 64, 84),
            water: c8(10, 3, 10),
            glint: c8(230, 140, 160),
            ..TANIM
        },
    ),
    (
        "ultraviolet",
        Look {
            glow: [
                c8(245, 150, 255),
                c8(175, 95, 255),
                c8(95, 70, 235),
                c8(45, 40, 170),
            ],
            sky_top: c8(4, 2, 14),
            sky_low: c8(22, 10, 44),
            far: c8(30, 20, 56),
            near: c8(12, 8, 24),
            snow: c8(90, 76, 130),
            glint: c8(200, 160, 255),
            ..TANIM
        },
    ),
    (
        "ice",
        Look {
            glow: [
                c8(225, 250, 255),
                c8(145, 222, 255),
                c8(85, 160, 240),
                c8(60, 90, 200),
            ],
            sky_top: c8(2, 8, 20),
            sky_low: c8(18, 38, 64),
            far: c8(40, 60, 92),
            near: c8(16, 26, 42),
            snow: c8(176, 204, 232),
            pine: c8(8, 14, 24),
            water: c8(6, 14, 28),
            glint: c8(210, 235, 255),
            rim: c8(180, 220, 255),
            rim_k: 0.35,
            moon: c8(236, 246, 255),
            moon_k: 1.0,
            ..TANIM
        },
    ),
    (
        "synthwave",
        Look {
            glow: [
                c8(0, 255, 240),
                c8(80, 180, 255),
                c8(255, 60, 200),
                c8(255, 40, 120),
            ],
            sky_top: c8(10, 0, 24),
            sky_low: c8(64, 10, 74),
            far: c8(42, 10, 62),
            near: c8(18, 4, 32),
            snow: c8(42, 10, 62),
            pine: c8(10, 2, 20),
            water: c8(8, 0, 22),
            star: c8(255, 200, 250),
            glint: c8(255, 90, 220),
            rim: c8(255, 60, 220),
            rim_k: 1.0,
            moon: c8(255, 190, 80),
            moon_k: 0.9,
            ..TANIM
        },
    ),
    (
        "ember",
        Look {
            glow: [
                c8(255, 205, 90),
                c8(255, 135, 40),
                c8(220, 60, 40),
                c8(120, 30, 70),
            ],
            sky_top: c8(6, 3, 6),
            sky_low: c8(32, 16, 12),
            far: c8(42, 26, 22),
            near: c8(16, 10, 10),
            snow: c8(110, 84, 70),
            water: c8(10, 5, 4),
            glint: c8(255, 170, 90),
            ..TANIM
        },
    ),
    (
        "prism",
        Look {
            prism: 1.0,
            ..TANIM
        },
    ),
    (
        "starry",
        Look {
            glow: [
                c8(250, 240, 170),
                c8(120, 190, 220),
                c8(60, 110, 190),
                c8(30, 60, 140),
            ],
            sky_top: c8(14, 30, 80),
            sky_low: c8(40, 70, 130),
            far: c8(40, 62, 112),
            near: c8(18, 32, 46),
            snow: c8(64, 88, 136),
            pine: c8(10, 22, 18),
            water: c8(10, 24, 60),
            star: c8(250, 230, 130),
            glint: c8(250, 220, 120),
            lights: 0.35,
            starry: 1.0,
            ..TANIM
        },
    ),
];

/// Van Gogh's brushstrokes, from the deepest blue to the palest yellow.
const STROKES: [V3; 6] = [
    c8(10, 22, 70),
    c8(24, 52, 130),
    c8(46, 92, 170),
    c8(80, 140, 190),
    c8(150, 190, 200),
    c8(225, 225, 170),
];
/// Rings around the stars and the moon, and their cores.
const STAR_RINGS: [V3; 2] = [c8(250, 230, 120), c8(200, 222, 210)];
const MOON_RINGS: [V3; 2] = [c8(255, 190, 60), c8(250, 230, 140)];
const STAR_CORE: V3 = c8(255, 250, 210);
const MOON_CORE: V3 = c8(255, 215, 80);
/// The cypress in its two strokes, and the village: walls, roofs, windows.
const CYPRESS: [V3; 2] = [c8(6, 16, 12), c8(34, 58, 36)];
const VILLAGE: [V3; 3] = [c8(46, 62, 96), c8(28, 36, 64), c8(250, 210, 90)];

impl Look {
    fn lerp(&self, o: &Look, t: f32) -> Look {
        let m = |a: V3, b: V3| mix(a, b, t);
        let f = |a: f32, b: f32| a + (b - a) * t;
        Look {
            glow: std::array::from_fn(|i| m(self.glow[i], o.glow[i])),
            sky_top: m(self.sky_top, o.sky_top),
            sky_low: m(self.sky_low, o.sky_low),
            far: m(self.far, o.far),
            near: m(self.near, o.near),
            snow: m(self.snow, o.snow),
            pine: m(self.pine, o.pine),
            water: m(self.water, o.water),
            star: m(self.star, o.star),
            glint: m(self.glint, o.glint),
            rim: m(self.rim, o.rim),
            rim_k: f(self.rim_k, o.rim_k),
            moon: m(self.moon, o.moon),
            moon_k: f(self.moon_k, o.moon_k),
            lights: f(self.lights, o.lights),
            prism: f(self.prism, o.prism),
            starry: f(self.starry, o.starry),
        }
    }

    /// This look in a theme's colors: the effects stay, the colors change.
    fn themed(&self, t: &Theme) -> Look {
        let g = &t.grad;
        let (low, mid, high) = (v3(g.at(0.0)), v3(g.at(0.5)), v3(g.at(1.0)));
        let bg = v3(t.bg);
        Look {
            glow: [v3(g.at(0.9)), v3(g.at(0.7)), v3(g.at(0.5)), v3(g.at(0.3))],
            sky_top: scale(bg, 0.6),
            sky_low: mix(bg, mid, 0.15),
            far: mix(low, mid, 0.25),
            near: scale(low, 0.6),
            snow: mix(low, high, 0.35),
            pine: scale(low, 0.3),
            water: scale(bg, 0.5),
            star: high,
            glint: mix(mid, high, 0.5),
            rim: high,
            moon: high,
            ..*self
        }
    }
}

/// The colors of the starry night and of the prism, from the theme.
struct Paint {
    strokes: [V3; 6],
    star_rings: [V3; 2],
    moon_rings: [V3; 2],
    star_core: V3,
    moon_core: V3,
    cypress: [V3; 2],
    village: [V3; 3],
    /// Colors the prism runs through; empty for every hue.
    prism: Vec<V3>,
}

impl Paint {
    fn new(t: &Theme) -> Paint {
        if t.native {
            return Paint {
                strokes: STROKES,
                star_rings: STAR_RINGS,
                moon_rings: MOON_RINGS,
                star_core: STAR_CORE,
                moon_core: MOON_CORE,
                cypress: CYPRESS,
                village: VILLAGE,
                prism: Vec::new(),
            };
        }
        let g = &t.grad;
        let at = |k: f32| v3(g.at(k));
        let mut prism: Vec<V3> = t.colors.iter().map(|&c| v3(c)).collect();
        if prism.is_empty() {
            prism = vec![at(0.4), at(0.7), at(1.0)];
        }
        Paint {
            strokes: std::array::from_fn(|i| {
                scale(at(0.05 + 0.17 * i as f32), 0.5 + 0.1 * i as f32)
            }),
            star_rings: [at(1.0), at(0.8)],
            moon_rings: [at(0.9), at(1.0)],
            star_core: at(1.0),
            moon_core: at(1.0),
            cypress: [scale(at(0.0), 0.5), mix(at(0.0), at(0.5), 0.3)],
            village: [mix(at(0.0), at(0.5), 0.4), at(0.0), at(1.0)],
            prism,
        }
    }
}

struct Curtain {
    /// Screen x where the track starts and its length.
    x0: f32,
    len: f32,
    /// Lower edge height (pixel row) and how far the light reaches upwards.
    base: f32,
    tall: f32,
    /// Ripples along the track: amplitude, spatial frequency, speed.
    ripples: [(f32, f32, f32); 3],
    drift: f32,
    bright: f32,
    age: u32,
    life: u32,
    phase: f32,
}

struct Star {
    x: i32,
    y: i32,
    b: f32,
    phase: f32,
}

struct Meteor {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: u32,
}

/// What Van Gogh's sky looks like at one pixel, worked out once per size.
#[derive(Clone, Copy, Default)]
struct Brush {
    /// Sine and cosine of the phase of the bands of the flowing sky and of
    /// the nearest whirl's spiral, and how much the whirl rules here.
    flow: (f32, f32),
    whirl: (f32, f32),
    pull: f32,
    /// A little jitter, so the bands break into dabs of paint.
    dab: f32,
    /// Phase of the rings around the nearest star or the moon, and how
    /// strongly they show.
    ring: f32,
    halo: f32,
    moon: bool,
    /// Inside a star (1) or the moon's crescent (2).
    core: u8,
}

fn v3(c: Color) -> V3 {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => ansi_rgb(n),
        Color::Reset => (255, 255, 255),
    };
    c8(r, g, b)
}

fn mix(a: V3, b: V3, t: f32) -> V3 {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn scale(a: V3, k: f32) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// A color between evenly spaced stops, `t` in 0..1.
fn ramp(stops: &[V3], t: f32) -> V3 {
    let n = stops.len();
    let t = t.clamp(0.0, 1.0) * (n - 1) as f32;
    let i = (t as usize).min(n - 2);
    mix(stops[i], stops[i + 1], t - i as f32)
}

/// HSV to RGB, `h` wrapping around 0..1.
fn hsv(h: f32, s: f32, v: f32) -> V3 {
    let h = (h - h.floor()) * 6.0;
    let i = h as i32;
    let f = h - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    match i {
        0 => [v, t, p],
        1 => [q, v, p],
        2 => [p, v, t],
        3 => [p, q, v],
        4 => [t, p, v],
        _ => [v, p, q],
    }
}

/// Colors snap to steps of 4 so that slowly shifting light stays identical
/// from frame to frame and the renderer has fewer cells to send.
fn to_color(c: V3) -> Color {
    let b = |x: f32| ((x.clamp(0.0, 1.0) * 255.0 + 2.0) as u32 & !3).min(252) as u8;
    Color::Rgb(b(c[0]), b(c[1]), b(c[2]))
}

/// A numeric option clamped to its range; non-finite values get the default.
fn num(v: f32, default: f32, lo: f32, hi: f32) -> f32 {
    if v.is_finite() {
        v.clamp(lo, hi)
    } else {
        default
    }
}

fn rangef(rng: &mut fastrand::Rng, a: f32, b: f32) -> f32 {
    a + (b - a) * rng.f32()
}

/// An integer in `lo..hi`, or `lo` when the range is empty.
fn range(rng: &mut fastrand::Rng, lo: i32, hi: i32) -> i32 {
    if hi <= lo { lo } else { rng.i32(lo..hi) }
}

fn hash(x: i32, y: i32, seed: u64) -> u64 {
    let mut h = seed
        ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^ (h >> 29)
}

/// 1D midpoint displacement: a jagged ridge line of `n` heights in `[0, 1]`.
fn ridge(n: usize, rough: f32, rng: &mut fastrand::Rng) -> Vec<f32> {
    let mut size = 1;
    while size + 1 < n {
        size *= 2;
    }
    let mut v = vec![0.0f32; size + 1];
    v[0] = rangef(rng, 0.2, 0.8);
    v[size] = rangef(rng, 0.2, 0.8);
    let mut step = size;
    let mut amp = 0.6;
    while step > 1 {
        let half = step / 2;
        let mut i = half;
        while i < size {
            v[i] = (v[i - half] + v[i + half]) / 2.0 + rangef(rng, -amp, amp);
            i += step;
        }
        amp *= rough;
        step = half;
    }
    let (lo, hi) = v
        .iter()
        .fold((f32::MAX, f32::MIN), |(a, b), &x| (a.min(x), b.max(x)));
    let span = (hi - lo).max(1e-3);
    (0..n)
        .map(|i| (v[i * size / n.max(1)] - lo) / span)
        .collect()
}

struct Aurora {
    rng: fastrand::Rng,
    seed: u64,
    speed: f32,
    count: usize,
    landscape: bool,
    show_stars: bool,
    paint_sky: bool,
    /// The look to stay on, or `None` to cycle.
    fixed: Option<usize>,
    stay: f32,
    /// The look showing and the one it fades into, by index into `LOOKS`, in
    /// the theme's colors, and real seconds since the look began.
    which: usize,
    next: usize,
    cur: Look,
    nxt: Look,
    clock: f32,
    paint: Paint,
    /// Pixel size of the screen (width, twice the rows) and the horizon row.
    w: usize,
    ph: usize,
    horizon: usize,
    /// What tanim's sizes are fractions of: the screen's pixel height, or more
    /// when there is no landscape and the sky reaches the bottom.
    span: f32,
    far: Vec<i32>,
    near: Vec<i32>,
    /// Pine silhouettes on the near shore: (x, height).
    pines: Vec<(i32, i32)>,
    stars: Vec<Star>,
    curtains: Vec<Curtain>,
    meteor: Option<Meteor>,
    /// Van Gogh's sky, cypress (pixel and stroke) and village (pixel and part).
    brush: Vec<Brush>,
    cypress: Vec<(usize, f32)>,
    village: Vec<(usize, u8)>,
    /// Per-column light and weighted lower edge for the curtain being drawn.
    light: Vec<f32>,
    edge: Vec<f32>,
    /// A curtain's glow by distance above its lower edge: strength and color.
    profile: Vec<(f32, V3)>,
    /// Additive aurora light per sky pixel.
    glow: Vec<V3>,
    /// The picture before it becomes colors, and which pixels are solid; the
    /// others hold light over a transparent sky.
    px: Vec<V3>,
    solid: Vec<bool>,
    out: Pixels,
    /// Motion time in tanim ticks.
    t: f32,
    /// Occasional substorm: brightens and quickens everything, then decays.
    surge: f32,
    acc: f32,
    /// Nyan Cat or the train, while one goes by, and for how long it has.
    visitor: Option<Box<dyn Egg>>,
    visiting: f32,
    visitors: f32,
    cols: u16,
    rows: u16,
}

impl Aurora {
    fn new(ctx: &Ctx) -> Aurora {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let t = rangef(&mut rng, 0.0, 1000.0);
        let fixed = LOOKS.iter().position(|(n, _)| *n == o.s("look").trim());
        let which = fixed.unwrap_or_else(|| rng.usize(0..LOOKS.len()));
        let next = Self::pick_next(&mut rng, which, fixed);
        Aurora {
            seed: ctx.seed,
            speed: num(o.f("speed"), 1.0, 0.1, 4.0),
            count: o.i("curtains").clamp(1, 8) as usize,
            landscape: o.b("landscape"),
            show_stars: o.b("stars"),
            paint_sky: o.b("sky"),
            fixed,
            stay: num(o.f("stay"), 90.0, 10.0, 3600.0),
            which,
            next,
            cur: LOOKS[which].1,
            nxt: LOOKS[next].1,
            clock: 0.0,
            paint: Paint::new(ctx.theme),
            w: 0,
            ph: 0,
            horizon: 0,
            span: 0.0,
            far: Vec::new(),
            near: Vec::new(),
            pines: Vec::new(),
            stars: Vec::new(),
            curtains: Vec::new(),
            meteor: None,
            brush: Vec::new(),
            cypress: Vec::new(),
            village: Vec::new(),
            light: Vec::new(),
            edge: Vec::new(),
            profile: Vec::new(),
            glow: Vec::new(),
            px: Vec::new(),
            solid: Vec::new(),
            out: Pixels::new(1, 1),
            t,
            surge: 0.0,
            acc: 0.0,
            visitor: None,
            visiting: 0.0,
            visitors: num(o.f("visitors"), 3.0, 0.0, eggs::FORCE_RATE as f32),
            cols: 0,
            rows: 0,
            rng,
        }
    }

    /// The look after `which`: any other one, or the same when it is fixed.
    fn pick_next(rng: &mut fastrand::Rng, which: usize, fixed: Option<usize>) -> usize {
        match fixed {
            Some(i) => i,
            None => (which + 1 + rng.usize(0..LOOKS.len() - 1)) % LOOKS.len(),
        }
    }

    fn look_of(i: usize, theme: &Theme) -> Look {
        let l = LOOKS[i].1;
        if theme.native { l } else { l.themed(theme) }
    }

    /// The look on screen now, partway into the next one near the end of its stay.
    fn look(&self) -> Look {
        let into = (self.clock - (self.stay - BLEND)) / BLEND;
        let k = into.clamp(0.0, 1.0);
        self.cur.lerp(&self.nxt, k * k * (3.0 - 2.0 * k))
    }

    /// The ridges, the forest, the stars and a sky already full of curtains.
    fn build(&mut self) {
        let (w, ph) = (self.w, self.ph);
        self.horizon = if self.landscape { ph * 62 / 100 } else { ph };
        self.span = if self.landscape {
            ph as f32
        } else {
            ph as f32 / 0.62
        };
        let (hz, phf) = (self.horizon as f32, ph as f32);
        let rng = &mut self.rng;
        self.far = ridge(w, 0.55, rng)
            .iter()
            .map(|v| (hz - phf * (0.06 + 0.22 * v)) as i32)
            .collect();
        self.near = ridge(w, 0.45, rng)
            .iter()
            .map(|v| (hz - phf * (0.02 + 0.05 * v)) as i32)
            .collect();
        // A pine forest along the top of the near hills, dark against the
        // mountains behind.
        self.pines.clear();
        let mut x = range(rng, 0, 3);
        while (x as usize) < w {
            if rng.f32() < 0.7 {
                self.pines.push((x, range(rng, 2, (ph as i32 / 14).max(3))));
            }
            x += range(rng, 1, 4);
        }
        let ceiling = if self.landscape {
            self.horizon as i32 - 2
        } else {
            self.horizon as i32
        };
        self.stars = (0..(w * self.horizon / 22).max(1))
            .map(|_| Star {
                x: range(rng, 0, w as i32),
                y: range(rng, 0, ceiling.max(1)),
                b: rangef(rng, 0.25, 1.0),
                phase: rangef(rng, 0.0, 6.3),
            })
            .collect();
        self.curtains.clear();
        for i in 0..self.count {
            let mut c = self.curtain();
            // Start mid-life so the sky is not empty at first.
            c.age = c.life / 3 + i as u32 * 40;
            self.curtains.push(c);
        }
        self.meteor = None;
        self.build_starry();
    }

    /// Van Gogh's sky: a great double whirl and a smaller one in a sky
    /// flowing in bands, eleven ringed stars, a crescent moon, a cypress on
    /// the left and a village with its church on the hills.
    fn build_starry(&mut self) {
        let (w, ph, hz) = (self.w, self.ph, self.horizon);
        let (wf, hf) = (w as f32, hz.max(1) as f32);
        let r = (0.13 * wf).min(0.45 * hf).max(3.0);
        let whirls = [
            (0.40 * wf, 0.34 * hf, r, 1.0),
            (0.56 * wf, 0.42 * hf, r * 0.75, -1.0),
            (0.16 * wf, 0.22 * hf, r * 0.55, 1.0),
        ];
        let moon = (0.86 * wf, 0.18 * hf, (0.07 * hf).clamp(2.0, 8.0));
        let star_r = (0.035 * hf).clamp(1.0, 4.0);
        let mut stars: Vec<(f32, f32)> = Vec::new();
        let mut tries = 0;
        while stars.len() < 11 && tries < 400 {
            tries += 1;
            let (x, y) = (
                rangef(&mut self.rng, 0.04, 0.96) * wf,
                rangef(&mut self.rng, 0.06, 0.7) * hf,
            );
            let clear = |(ax, ay): (f32, f32), d: f32| (x - ax).hypot(y - ay) > d;
            if whirls
                .iter()
                .all(|&(wx, wy, wr, _)| clear((wx, wy), wr * 0.7))
                && clear((moon.0, moon.1), moon.2 * 4.0)
                && stars.iter().all(|&s| clear(s, star_r * 6.0))
            {
                stars.push((x, y));
            }
        }
        self.brush = vec![Brush::default(); w * hz];
        for y in 0..hz {
            for x in 0..w {
                let (xf, yf) = (x as f32, y as f32);
                let mut b = Brush {
                    flow: (yf * 0.55
                        + 2.2 * (xf * 0.045 + yf * 0.03).sin()
                        + 1.3 * (xf * 0.11 - yf * 0.07).sin())
                    .sin_cos(),
                    dab: ((hash(x as i32 / 3, y as i32, self.seed) % 100) as f32 / 100.0 - 0.5)
                        * 0.2,
                    ..Brush::default()
                };
                for &(wx, wy, wr, dir) in &whirls {
                    let (dx, dy) = (xf - wx, yf - wy);
                    let d = dx.hypot(dy);
                    let pull = (-(d / wr).powi(2)).exp();
                    if pull > b.pull {
                        b.pull = pull;
                        // A two-armed logarithmic spiral.
                        b.whirl = (6.0 * (d + 1.0).ln() - dir * 2.0 * dy.atan2(dx)).sin_cos();
                    }
                }
                let (dmx, dmy) = (xf - moon.0, yf - moon.1);
                let dm = dmx.hypot(dmy);
                if dm < moon.2 * 3.2 {
                    b.moon = true;
                    b.ring = dm * 1.4;
                    b.halo = (1.0 - dm / (moon.2 * 3.2)).powf(0.6);
                    // Lit except where a second disc shades it.
                    let shade = (xf - moon.0 - 0.55 * moon.2).hypot(yf - moon.1 + 0.2 * moon.2);
                    if dm < moon.2 && shade > moon.2 * 0.85 {
                        b.core = 2;
                    }
                }
                for &(sx, sy) in &stars {
                    let d = (xf - sx).hypot(yf - sy);
                    let halo = (1.0 - d / (star_r * 3.5)).max(0.0).powf(0.7) * 0.85;
                    if halo > b.halo {
                        b.moon = false;
                        b.halo = halo;
                        b.ring = d * 1.6;
                        b.core = u8::from(d < star_r);
                    }
                }
                self.brush[y * w + x] = b;
            }
        }
        // The cypress, a dark flame from the bottom of the screen nearly to the top.
        self.cypress.clear();
        let (cx, top) = (0.13 * wf, (ph as f32 * 0.04) as usize);
        let girth = (0.07 * wf).max(0.06 * ph as f32);
        for y in top..ph {
            let yf = y as f32;
            let u = (yf - top as f32) / (ph - top).max(1) as f32;
            // Widest low down, a point at the top, and tongues of flame along the sides.
            let half = girth
                * (PI * (u * 1.4).min(1.0) / 2.0).sin().powf(0.8)
                * (0.8 + 0.2 * (yf * 0.55 + 1.3).sin());
            let lean = 0.035 * wf * (u * PI * 1.5).sin();
            let mid = cx + lean;
            let (x0, x1) = ((mid - half).round() as i32, (mid + half).round() as i32);
            for x in x0.max(0)..=x1.min(w as i32 - 1) {
                let xf = x as f32 - mid;
                // Strokes running up the tree and curling with it.
                let stroke = 0.5 + 0.5 * (xf * 1.1 + yf * 0.12 + 1.5 * (yf * 0.08).sin()).sin();
                self.cypress.push((y * w + x as usize, stroke * stroke));
            }
        }
        // The village on the near hills, its church in the middle.
        self.village.clear();
        if !self.landscape || w < 12 {
            return;
        }
        let (church, mut built) = ((0.58 * wf) as i32, false);
        let mut x = (0.3 * wf) as i32;
        while x < (0.92 * wf) as i32 {
            let is_church = !built && x + 9 > church;
            built |= is_church;
            let (bw, bh) = if is_church {
                (5, 5)
            } else {
                (range(&mut self.rng, 3, 7), range(&mut self.rng, 2, 4))
            };
            let foot = self.near[(x + bw / 2).clamp(0, w as i32 - 1) as usize] + 2;
            let put = |px: i32, py: i32, part: u8, v: &mut Vec<(usize, u8)>| {
                if px >= 0 && (px as usize) < w && py >= 0 && (py as usize) < hz {
                    v.push((py as usize * w + px as usize, part));
                }
            };
            for dy in 0..bh {
                for dx in 0..bw {
                    let window = dy > 0 && hash(x + dx, dy, self.seed ^ 0x77).is_multiple_of(4);
                    put(
                        x + dx,
                        foot - dy,
                        if window { 2 } else { 0 },
                        &mut self.village,
                    );
                }
            }
            // A roof narrowing to its ridge.
            for k in 0..=bw / 2 {
                for dx in k..bw - k {
                    put(x + dx, foot - bh - k, 1, &mut self.village);
                }
            }
            if is_church {
                let spire = (ph as i32 / 9).max(3);
                for k in 0..spire {
                    put(x + bw / 2, foot - bh - bw / 2 - k, 1, &mut self.village);
                }
            }
            x += bw + range(&mut self.rng, 1, 4);
        }
    }

    fn curtain(&mut self) -> Curtain {
        let (w, span) = (self.w as f32, self.span);
        let rng = &mut self.rng;
        let len = w * rangef(rng, 0.5, 1.1);
        // The first ripple is strong enough (amplitude x frequency > 1) to
        // make the track double back on itself: those are the folds.
        let k0 = rangef(rng, 0.02, 0.05) * 80.0 / w.max(20.0);
        let a0 = rangef(rng, 0.9, 1.6) / k0;
        Curtain {
            x0: rangef(rng, -0.2 * w, 0.7 * w),
            len,
            base: span * rangef(rng, 0.2, 0.38),
            tall: span * rangef(rng, 0.1, 0.2),
            ripples: [
                (a0.min(w * 0.3), k0, rangef(rng, 0.004, 0.012)),
                (
                    rangef(rng, 2.0, 6.0),
                    rangef(rng, 0.08, 0.2),
                    rangef(rng, -0.03, 0.03),
                ),
                (
                    rangef(rng, 0.5, 2.0),
                    rangef(rng, 0.3, 0.6),
                    rangef(rng, 0.02, 0.06),
                ),
            ],
            drift: rangef(rng, -0.03, 0.03),
            bright: rangef(rng, 0.6, 1.0),
            age: 0,
            life: range(rng, 900, 2200) as u32,
            phase: rangef(rng, 0.0, 6.3),
        }
    }

    /// One tanim tick of everything but the clock of the ripples, which
    /// `update` advances smoothly.
    fn step(&mut self) {
        let quick = 1.0 + 1.5 * self.surge;
        self.surge *= 0.994;
        if self.rng.f32() < 0.0008 {
            self.surge = 1.0;
        }
        for i in 0..self.curtains.len() {
            let c = &mut self.curtains[i];
            c.age += 1;
            c.x0 += c.drift * quick;
            if c.age >= c.life {
                self.curtains[i] = self.curtain();
            }
        }
        match &mut self.meteor {
            Some(m) => {
                m.x += m.vx;
                m.y += m.vy;
                m.age += 1;
                if m.age > 26 {
                    self.meteor = None;
                }
            }
            None => {
                if self.show_stars && self.rng.f32() < 0.003 {
                    let right = self.rng.bool();
                    self.meteor = Some(Meteor {
                        x: rangef(&mut self.rng, 0.1, 0.9) * self.w as f32,
                        y: rangef(&mut self.rng, 0.0, 0.3) * self.horizon as f32,
                        vx: if right { 2.2 } else { -2.2 },
                        vy: 0.9,
                        age: 0,
                    });
                }
            }
        }
    }

    /// Sends Nyan Cat or the train across now and then, `real` seconds later.
    fn visit(&mut self, real: f32) {
        if let Some(v) = &mut self.visitor {
            v.update(real);
            self.visiting += real;
            if v.done() || self.visiting > VISIT_SECS {
                self.visitor = None;
            }
            return;
        }
        let rate = self.visitors as f64;
        if rate <= 0.0 || self.cols == 0 {
            return;
        }
        if rate >= eggs::FORCE_RATE || self.rng.f64() < 1.0 - (-rate / 3600.0 * real as f64).exp() {
            let info = if self.rng.bool() {
                &eggs::nyan::INFO
            } else {
                &eggs::sl::INFO
            };
            self.visitor = Some((info.start)(self.cols, self.rows, &mut self.rng));
            self.visiting = 0.0;
        }
    }

    /// Accumulates every curtain's light into `glow`.
    fn draw_curtains(&mut self, look: &Look) {
        self.glow.fill([0.0; 3]);
        let (w, hz) = (self.w, self.horizon);
        let t = self.t;
        let surge = 1.0 + 1.2 * self.surge;
        for ci in 0..self.curtains.len() {
            self.light.fill(0.0);
            self.edge.fill(0.0);
            let c = &self.curtains[ci];
            let fade = {
                let a = c.age as f32 / c.life as f32;
                (a * 6.0).min(1.0).min((1.0 - a) * 6.0).max(0.0)
            };
            let bright =
                c.bright * fade * surge * look.lights * (0.75 + 0.25 * (t * 0.013 + c.phase).sin());
            if bright <= 0.01 {
                continue;
            }
            let steps = (c.len * 3.0) as usize;
            let (base, tall) = (c.base, c.tall);
            for i in 0..steps {
                let s = i as f32 / 3.0;
                let mut x = c.x0 + s;
                for &(amp, k, speed) in &c.ripples {
                    x += amp * (k * s + speed * t + c.phase).sin();
                }
                // Taper both ends of the curtain.
                let u = s / c.len;
                let wgt = (PI * u).sin().max(0.0).sqrt() / 3.0;
                let e = base + 0.03 * self.span * (0.05 * s + 0.01 * t + c.phase).sin();
                let xi = x.floor();
                let fr = x - xi;
                for (xx, part) in [(xi as i32, 1.0 - fr), (xi as i32 + 1, fr)] {
                    if xx >= 0 && (xx as usize) < w {
                        self.light[xx as usize] += wgt * part;
                        self.edge[xx as usize] += e * wgt * part;
                    }
                }
            }
            // The glow by distance d above the edge, from 3 below it to the
            // top, in eighths of a pixel: sharp lower edge, long exponential
            // fade upwards.
            self.profile.clear();
            let n = ((tall * 3.5 + 3.0) * PROFILE_RES) as usize + 2;
            for k in 0..n {
                let d = k as f32 / PROFILE_RES - 3.0;
                let prof = if d < 0.0 {
                    (d * 1.2).exp()
                } else {
                    (-d / tall).exp() * (1.0 - (-(d + 1.0) / 1.5).exp())
                };
                let u = (d / (tall * 3.0)).clamp(0.0, 1.0);
                self.profile.push((prof, ramp(&look.glow, u)));
            }
            for x in 0..w {
                let l = self.light[x];
                if l < 1e-3 {
                    continue;
                }
                let edge = self.edge[x] / l;
                // Vertical rays: the light shimmers column by column.
                let xf = x as f32;
                let rays = 0.7 + 0.3 * (xf * 0.9 + t * 0.07).sin() * (xf * 0.23 - t * 0.031).sin();
                let power = l.min(3.0) * bright * rays;
                let top = (edge - tall * 3.5).max(0.0) as usize;
                let bottom = ((edge + 3.0).max(0.0) as usize).min(hz);
                // The prism's hue for this column, drifting along the sky.
                let hue = xf * 0.011 + t * 0.002 + c.phase * 0.1;
                for y in top..bottom {
                    let d = edge - y as f32;
                    let k = (((d + 3.0) * PROFILE_RES) as usize).min(n - 1);
                    let (prof, mut col) = self.profile[k];
                    if look.prism > 0.01 {
                        let u = (d / (tall * 3.0)).clamp(0.0, 1.0);
                        let h = hue + u * 0.35;
                        let p = if self.paint.prism.is_empty() {
                            hsv(h, 0.75, 1.0)
                        } else {
                            let n = self.paint.prism.len();
                            let k = (h - h.floor()) * n as f32;
                            let i = k as usize % n;
                            mix(
                                self.paint.prism[i],
                                self.paint.prism[(i + 1) % n],
                                k.fract(),
                            )
                        };
                        col = mix(col, p, look.prism);
                    }
                    let a = prof * power * 0.9;
                    let g = &mut self.glow[y * w + x];
                    *g = add(*g, scale(col, a));
                }
            }
        }
    }

    /// Paints the starry night's sky over row `y`, weighted by `s`.
    fn paint_starry_row(&mut self, y: usize, s: f32) {
        let w = self.w;
        // The paint flows in steps of two ticks, plenty for brushstrokes and
        // half the cells to resend.
        let t = (self.t * 0.5).floor() * 2.0;
        let p = &self.paint;
        let (flow_t, whirl_t, ring_t) = ((t * 0.02).sin_cos(), (t * 0.035).sin_cos(), t * 0.06);
        for x in 0..w {
            let i = y * w + x;
            let b = self.brush[i];
            // sin(a - t) without a sine per pixel: a's is kept, t's is shared.
            let (sf, cf) = b.flow;
            let band_f = sf * flow_t.1 - cf * flow_t.0;
            let band = if b.pull > 0.02 {
                let (sw, cw) = b.whirl;
                let band_w = sw * whirl_t.1 - cw * whirl_t.0;
                band_f + (band_w - band_f) * b.pull
            } else {
                band_f
            };
            let mut col = ramp(&p.strokes, 0.42 + 0.4 * band + b.dab);
            if b.halo > 0.0 {
                let rings = if b.moon { &p.moon_rings } else { &p.star_rings };
                let ring = 0.5 + 0.5 * (b.ring - ring_t).sin();
                col = mix(col, mix(rings[0], rings[1], ring), b.halo);
            }
            match b.core {
                1 => col = p.star_core,
                2 => col = p.moon_core,
                _ => {}
            }
            self.px[i] = mix(self.px[i], col, s);
        }
    }

    /// Paints sky, stars, aurora, land and lake into `px`. Over a transparent
    /// sky, `bg` is the terminal's background when known, which the lake's
    /// reflections of the land fade toward.
    fn compose(&mut self, look: &Look, bg: Option<V3>) {
        let (w, ph, hz) = (self.w, self.ph, self.horizon);
        let t = self.t;
        let s = look.starry;
        // Sky, stars and aurora light.
        for y in 0..hz {
            let base = if self.paint_sky {
                mix(look.sky_top, look.sky_low, y as f32 / hz.max(1) as f32)
            } else {
                [0.0; 3]
            };
            self.px[y * w..(y + 1) * w].fill(base);
            if s > 0.01 {
                self.paint_starry_row(y, s);
            }
        }
        self.solid.fill(self.paint_sky);
        if self.show_stars && s < 0.99 {
            for st in &self.stars {
                let tw = 0.7 + 0.3 * (t * 0.05 + st.phase).sin();
                let i = st.y as usize * w + st.x as usize;
                self.px[i] = mix(self.px[i], look.star, st.b * tw * (1.0 - s));
            }
        }
        if look.moon_k > 0.01 && hz > 4 {
            // A crescent: a disc with a second one shading its left side.
            let (mx, my) = (0.82 * w as f32, 0.2 * hz as f32);
            let r = (0.09 * hz as f32).clamp(2.0, 9.0);
            let reach = (r * 3.0).ceil() as i32;
            for y in (my as i32 - reach).max(0)..(my as i32 + reach).min(hz as i32) {
                for x in (mx as i32 - reach).max(0)..(mx as i32 + reach).min(w as i32) {
                    let (xf, yf) = (x as f32 + 0.5, y as f32 + 0.5);
                    let d = (xf - mx).hypot(yf - my);
                    let shade = (xf - mx + 0.6 * r).hypot(yf - my + 0.25 * r);
                    let i = y as usize * w + x as usize;
                    if d < r && shade > r * 0.9 {
                        self.px[i] = mix(self.px[i], look.moon, look.moon_k);
                    } else {
                        let halo = (1.0 - d / (r * 3.0)).max(0.0).powi(2) * 0.18;
                        self.px[i] = add(self.px[i], scale(look.moon, halo * look.moon_k));
                    }
                }
            }
        }
        if let Some(m) = &self.meteor {
            // Between ticks, so it streaks smoothly at any frame rate.
            let ahead = self.acc / STEP;
            let (mx, my) = (m.x + m.vx * ahead, m.y + m.vy * ahead);
            for k in 0..8 {
                let x = (mx - m.vx * k as f32 * 0.8) as i32;
                let y = (my - m.vy * k as f32 * 0.8) as i32;
                if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < hz {
                    let i = y as usize * w + x as usize;
                    let a = (1.0 - k as f32 / 8.0) * (1.0 - m.age as f32 / 27.0);
                    self.px[i] = mix(self.px[i], mix(look.star, [1.0; 3], 0.6), a);
                }
            }
        }
        let mut ambient = [0.0f32; 3];
        for (p, g) in self.px.iter_mut().zip(&self.glow) {
            *p = add(*p, *g);
            ambient = add(ambient, *g);
        }
        if !self.landscape {
            return;
        }
        // The aurora faintly lights the snow and the mountainsides.
        let n = (w * hz).max(1) as f32;
        let amb = ambient.map(|a| (a / n * 1.5).min(60.0 / 255.0));
        let lift = if self.paint_sky { 1.0 } else { LIFT };
        let (far, near, snow) = (
            scale(look.far, lift),
            scale(look.near, lift),
            scale(look.snow, lift.min(1.2)),
        );
        // Snow caps only the peaks that rise above the snow line.
        let snowline = hz as f32 - ph as f32 * 0.17;
        for x in 0..w {
            let (f, nr) = (self.far[x].max(0) as usize, self.near[x].max(0) as usize);
            let cap = ((snowline - f as f32) * 0.7).max(0.0) as usize;
            for y in f..hz {
                let mut base = if y < f + cap { snow } else { far };
                if y == f {
                    base = mix(base, look.rim, look.rim_k);
                }
                self.px[y * w + x] = add(base, amb);
                self.solid[y * w + x] = true;
            }
            for y in nr..hz {
                self.px[y * w + x] = add(near, scale(amb, 1.0 / 3.0));
                self.solid[y * w + x] = true;
            }
        }
        let pine = scale(look.pine, lift);
        for &(x, hgt) in &self.pines {
            let foot = self.near[(x.max(0) as usize).min(w - 1)] + 1;
            for k in 0..hgt {
                let half = (hgt - k) / 3;
                for dx in -half..=half {
                    let (xx, yy) = (x + dx, foot - k);
                    if xx >= 0 && (xx as usize) < w && yy >= 0 && (yy as usize) < ph {
                        let i = yy as usize * w + xx as usize;
                        self.px[i] = pine;
                        self.solid[i] = true;
                    }
                }
            }
        }
        if s > 0.01 {
            for &(i, part) in &self.village {
                self.px[i] = mix(
                    self.px[i],
                    scale(self.paint.village[part as usize], lift.min(1.3)),
                    s,
                );
                self.solid[i] |= s >= 0.5;
            }
        }
        // The lake: the scene above, flipped, rippled and darkened.
        let depth = (ph - hz).max(1) as f32;
        for y in hz..ph {
            let d = (y - hz) as f32 / depth;
            let src_y = hz as i32 - 1 - (y - hz) as i32;
            let dim = 0.6 - 0.3 * d;
            // Rows ripple sideways, more as the water comes closer.
            let wobble = (y as f32 * 0.7 + t * 0.09).sin() * (0.4 + 2.2 * d);
            for x in 0..w {
                let i = y * w + x;
                let glint = (x as f32 * 0.11 + y as f32 * 1.7 + t * 0.05).sin() > 0.985;
                if hz == 0 {
                    self.px[i] = if self.paint_sky { look.water } else { [0.0; 3] };
                    continue;
                }
                let sx = (x as f32 + wobble).round().clamp(0.0, w as f32 - 1.0) as usize;
                let sy = src_y.clamp(0, hz as i32 - 1) as usize;
                let (a, b) = (sy * w + sx, sy.saturating_sub(1) * w + sx);
                // Average two rows so stars soften instead of sparkling noise;
                // over a transparent sky only light mixes with light.
                let src = if self.solid[a] == self.solid[b] {
                    mix(self.px[a], self.px[b], 0.5)
                } else {
                    self.px[a]
                };
                let (c, solid) = if self.paint_sky {
                    (mix(look.water, src, dim), true)
                } else if self.solid[a] {
                    match bg {
                        Some(bg) => (mix(bg, src, dim), true),
                        None => (scale(src, dim), true),
                    }
                } else {
                    (scale(src, dim), false)
                };
                // Glints on the ripples.
                self.px[i] = match (glint, solid) {
                    (false, _) => c,
                    (true, true) => mix(c, look.glint, 0.25),
                    (true, false) => add(c, scale(look.glint, 0.25)),
                };
                self.solid[i] = solid;
            }
        }
        if s > 0.01 {
            let cyp = self.paint.cypress;
            for &(i, stroke) in &self.cypress {
                self.px[i] = mix(
                    self.px[i],
                    scale(mix(cyp[0], cyp[1], stroke), lift.min(1.3)),
                    s,
                );
                self.solid[i] |= s >= 0.5;
            }
        }
    }

    /// Turns accumulated light on a transparent sky into a visible pixel.
    /// Weak light fades toward the terminal background; with no known
    /// background it dims instead, and the faintest part is dropped so
    /// there are no dark halos.
    fn sky_light(ctx: &Ctx, l: V3) -> Color {
        let m = l[0].max(l[1]).max(l[2]);
        let known = matches!(ctx.bg, Color::Rgb(..));
        if m < if known { 0.04 } else { 0.1 } {
            return Color::Reset;
        }
        // Hue at full strength; bright light spills over into white.
        let over = (m - 1.0).max(0.0) * 0.3;
        let hue = [l[0] / m + over, l[1] / m + over, l[2] / m + over];
        let alpha = m.min(1.0);
        let c = to_color(hue);
        if known {
            to_color(v3(ctx.fade(c, 1.0 - alpha)))
        } else {
            c.scale(0.4 + 0.6 * alpha)
        }
    }
}

impl Scene for Aurora {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        (self.cols, self.rows) = (w, h);
        self.w = w as usize;
        self.ph = h as usize * 2;
        self.out.resize(w, h);
        let n = self.w * self.ph;
        self.light = vec![0.0; self.w];
        self.edge = vec![0.0; self.w];
        self.glow = vec![[0.0; 3]; n];
        self.px = vec![[0.0; 3]; n];
        self.solid = vec![false; n];
        self.visitor = None;
        self.build();
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.cur = Self::look_of(self.which, ctx.theme);
        self.nxt = Self::look_of(self.next, ctx.theme);
        self.paint = Paint::new(ctx.theme);
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.25)
        } else {
            0.0
        };
        // The ripples move smoothly; everything else ticks at tanim's rate.
        self.t += dt * self.speed / STEP * (1.0 + 1.5 * self.surge);
        self.acc += dt * self.speed;
        let mut n = 0;
        while self.acc >= STEP {
            self.acc -= STEP;
            self.step();
            n += 1;
            if n >= 8 {
                self.acc = 0.0;
                break;
            }
        }
    }

    /// Motion follows the speed setting; the looks and the visitors keep real time.
    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        if !real.is_finite() {
            return;
        }
        let real = real.clamp(0.0, 5.0);
        self.clock += real;
        if self.clock >= self.stay {
            self.clock -= self.stay;
            self.which = self.next;
            self.next = Self::pick_next(&mut self.rng, self.which, self.fixed);
            self.cur = Self::look_of(self.which, ctx.theme);
            self.nxt = Self::look_of(self.next, ctx.theme);
        }
        self.visit(real);
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let (w, ph) = (self.w, self.ph);
        if w == 0 || ph == 0 {
            return;
        }
        let bg = match ctx.bg {
            Color::Rgb(..) => Some(v3(ctx.bg)),
            _ => None,
        };
        let look = self.look();
        self.draw_curtains(&look);
        self.compose(&look, bg);
        self.out.clear();
        for y in 0..ph {
            for x in 0..w {
                let i = y * w + x;
                let col = if self.solid[i] {
                    to_color(self.px[i])
                } else {
                    Self::sky_light(ctx, self.px[i])
                };
                if col != Color::Reset {
                    self.out.set(x as i32, y as i32, col);
                }
            }
        }
        self.out.blit(c);
        if let Some(v) = &self.visitor {
            v.draw(c, ctx.theme);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;

    fn make(opts: &str, theme: &Theme) -> (Aurora, Opts) {
        let o = Opts::new(INFO.options, opts.parse().unwrap());
        let ctx = ctx_with(theme, &o, Color::Reset);
        let mut a = Aurora::new(&ctx);
        a.resize(&ctx, 80, 24);
        a.retheme(&ctx);
        (a, o)
    }

    fn ctx_with<'a>(theme: &'a Theme, opts: &'a Opts, bg: Color) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg,
            seed: 3,
        }
    }

    /// Odd options and sizes survive long runs and huge steps.
    #[test]
    fn variants_survive() {
        let variants = [
            "",
            "sky = true\ncurtains = 8\nspeed = 4.0\nlook = 'starry'",
            "landscape = false\nstars = false\ncurtains = 1\nlook = 'prism'",
            "speed = nan\ncurtains = -5\nstay = nan\nvisitors = inf",
            "speed = inf\ncurtains = 99\nsky = true\nlandscape = false\nlook = 'bogus'\nstay = 0.0",
            "look = 'synthwave'\nvisitors = 1000000.0",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                let (mut a, o) = make(v, &theme);
                for bg in [Color::Reset, theme.bg] {
                    let ctx = ctx_with(&theme, &o, bg);
                    for (w, h) in [(80, 24), (1, 1), (2, 3), (7, 2), (300, 90)] {
                        a.resize(&ctx, w, h);
                        a.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for k in 0..200 {
                            let dt = if k % 97 == 96 { 5.0 } else { 1.0 / 30.0 };
                            a.tick(&ctx, dt, dt);
                            c.clear();
                            a.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    /// Over a transparent sky the lights, the land and the lake show and much
    /// of the sky stays empty; `sky = true` paints every cell.
    #[test]
    fn transparent_and_painted_sky() {
        let theme = Theme::classic();
        let (mut a, o) = make("look = 'tanim'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&ctx, &mut c);
        let lit = c.cells().iter().filter(|cell| cell.ch != ' ').count();
        assert!(lit > 80 * 24 / 4 && lit < 80 * 24, "{lit} lit cells");

        let (mut a, o) = make("sky = true", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        c.clear();
        a.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch != ' '));
    }

    /// Without the landscape nothing solid is drawn: only light.
    #[test]
    fn sky_without_land() {
        let theme = Theme::classic();
        let (mut a, o) = make("landscape = false\nlook = 'tanim'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&ctx, &mut c);
        assert!(a.solid.iter().all(|&s| !s));
        assert_eq!(a.horizon, a.ph);
    }

    /// Curtains fade out and new ones take their place, so the sky never
    /// runs out of lights.
    #[test]
    fn curtains_come_and_go() {
        let theme = Theme::classic();
        let (mut a, o) = make("curtains = 3\nlook = 'tanim'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let first: Vec<f32> = a.curtains.iter().map(|c| c.phase).collect();
        for _ in 0..2400 {
            a.update(&ctx, 1.0 / 20.0);
        }
        assert_eq!(a.curtains.len(), 3);
        let now: Vec<f32> = a.curtains.iter().map(|c| c.phase).collect();
        assert!(first.iter().zip(&now).all(|(a, b)| a != b));
        let look = a.look();
        a.draw_curtains(&look);
        assert!(a.glow.iter().any(|g| g[1] > 0.05));
    }

    /// Cycling moves on to another look after `stay` real seconds, whatever
    /// the speed; a fixed look stays.
    #[test]
    fn looks_cycle_on_the_wall_clock() {
        let theme = Theme::classic();
        let (mut a, o) = make("stay = 30.0\nspeed = 0.1", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let mut seen = vec![a.which];
        for _ in 0..(30 * 30 * 8) {
            a.tick(&ctx, 0.1 / 30.0, 1.0 / 30.0);
            if seen.last() != Some(&a.which) {
                seen.push(a.which);
            }
        }
        assert!(seen.len() >= 7, "{seen:?}");
        assert!(seen.windows(2).all(|p| p[0] != p[1]));

        let (mut a, o) = make("stay = 10.0\nlook = 'ice'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        for _ in 0..(30 * 60) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_eq!(LOOKS[a.which].0, "ice");
    }

    /// The starry night paints its whirling sky, the cypress and the village.
    #[test]
    fn the_starry_night() {
        let theme = Theme::classic();
        let (mut a, o) = make("look = 'starry'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        assert!(!a.cypress.is_empty() && !a.village.is_empty());
        assert!(a.brush.iter().any(|b| b.core == 1) && a.brush.iter().any(|b| b.core == 2));
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&ctx, &mut c);
        // Every pixel of open sky shows paint.
        let n = a.w * a.horizon;
        let open = a.px[..n].iter().zip(&a.solid[..n]).filter(|(_, s)| !**s);
        assert!(open.map(|(p, _)| p).all(|p| p[2] > 0.1));
        let i = a.cypress[a.cypress.len() / 2].0;
        assert!(a.solid[i]);
    }

    /// Visitors come when asked to, and never with `visitors = 0`.
    #[test]
    fn visitors_go_by() {
        let theme = Theme::classic();
        let (mut a, o) = make("visitors = 1000000.0", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        assert!(a.visitor.is_some());
        for _ in 0..(45 * 30) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            assert!(a.visiting <= VISIT_SECS + 0.1);
        }

        let (mut a, o) = make("visitors = 0.0", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        for _ in 0..(600 * 30) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            assert!(a.visitor.is_none());
        }
    }
}
