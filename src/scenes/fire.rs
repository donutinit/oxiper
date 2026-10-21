//! fire: a roaring fire, after aalib's aafire.
//!
//! The flame simulation and the classic palette are a port of `aafire.c` from
//! AA-lib 1.4 (Copyright (C) 1997-2001 Jan Hubicka and Kamil Toman, LGPL-2.0
//! or later). Like the original it keeps a heat buffer two pixels per cell
//! wide and tall, feeds it with runs of random fuel along the bottom, and moves
//! every pixel up one row per step as the average of five neighbours below it
//! minus a decay ("minus") tuned to the screen height. The `ascii` style picks
//! characters the way aalib does: by matching the four quadrant brightnesses of
//! a cell against glyph coverage measured from aalib's 8×16 VGA font.

use std::sync::OnceLock;

use crate::color::{Color, Gradient, rgb};
use crate::render::{BOLD, Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "fire",
    aliases: &["aafire"],
    about: "a roaring fire, after aalib's aafire",
    options: &[
        Opt {
            key: "style",
            default: Val::S("blocks"),
            help: "blocks (smooth half-block pixels) or ascii (aalib characters)",
        },
        Opt {
            key: "intensity",
            default: Val::F(1.0),
            help: "Fuel: how hot and dense the flames burn (0.2-2)",
        },
        Opt {
            key: "height",
            default: Val::F(0.8),
            help: "How high the flames reach, relative to the screen (1 = aafire)",
        },
        Opt {
            key: "wind",
            default: Val::F(0.0),
            help: "Sideways lean, -2 (left) to 2 (right), with gentle gusts",
        },
        Opt {
            key: "cooling",
            default: Val::F(0.5),
            help: "Turbulent cooling that breaks flames into tongues (0 = pure aafire)",
        },
        Opt {
            key: "sparks",
            default: Val::B(true),
            help: "Embers drifting up from the flame tips",
        },
    ],
    make: |ctx| Box::new(Fire::new(ctx)),
};

/// aafire's palette, 6 bits per channel: black, a hint of blue, red, orange,
/// yellow, white.
#[rustfmt::skip]
const PAL: [(u8, u8, u8); 256] = [
    (0, 0, 0), (0, 0, 6), (0, 0, 6), (0, 0, 7), (0, 0, 8), (0, 0, 8), (0, 0, 9), (0, 0, 10),
    (2, 0, 10), (4, 0, 9), (6, 0, 9), (8, 0, 8), (10, 0, 7), (12, 0, 7), (14, 0, 6), (16, 0, 5),
    (18, 0, 5), (20, 0, 4), (22, 0, 4), (24, 0, 3), (26, 0, 2), (28, 0, 2), (30, 0, 1), (32, 0, 0),
    (32, 0, 0), (33, 0, 0), (34, 0, 0), (35, 0, 0), (36, 0, 0), (36, 0, 0), (37, 0, 0), (38, 0, 0),
    (39, 0, 0), (40, 0, 0), (40, 0, 0), (41, 0, 0), (42, 0, 0), (43, 0, 0), (44, 0, 0), (45, 0, 0),
    (46, 1, 0), (47, 1, 0), (48, 2, 0), (49, 2, 0), (50, 3, 0), (51, 3, 0), (52, 4, 0), (53, 4, 0),
    (54, 5, 0), (55, 5, 0), (56, 6, 0), (57, 6, 0), (58, 7, 0), (59, 7, 0), (60, 8, 0), (61, 8, 0),
    (63, 9, 0), (63, 9, 0), (63, 10, 0), (63, 10, 0), (63, 11, 0), (63, 11, 0), (63, 12, 0), (63, 12, 0),
    (63, 13, 0), (63, 13, 0), (63, 14, 0), (63, 14, 0), (63, 15, 0), (63, 15, 0), (63, 16, 0), (63, 16, 0),
    (63, 17, 0), (63, 17, 0), (63, 18, 0), (63, 18, 0), (63, 19, 0), (63, 19, 0), (63, 20, 0), (63, 20, 0),
    (63, 21, 0), (63, 21, 0), (63, 22, 0), (63, 22, 0), (63, 23, 0), (63, 24, 0), (63, 24, 0), (63, 25, 0),
    (63, 25, 0), (63, 26, 0), (63, 26, 0), (63, 27, 0), (63, 27, 0), (63, 28, 0), (63, 28, 0), (63, 29, 0),
    (63, 29, 0), (63, 30, 0), (63, 30, 0), (63, 31, 0), (63, 31, 0), (63, 32, 0), (63, 32, 0), (63, 33, 0),
    (63, 33, 0), (63, 34, 0), (63, 34, 0), (63, 35, 0), (63, 35, 0), (63, 36, 0), (63, 36, 0), (63, 37, 0),
    (63, 38, 0), (63, 38, 0), (63, 39, 0), (63, 39, 0), (63, 40, 0), (63, 40, 0), (63, 41, 0), (63, 41, 0),
    (63, 42, 0), (63, 42, 0), (63, 43, 0), (63, 43, 0), (63, 44, 0), (63, 44, 0), (63, 45, 0), (63, 45, 0),
    (63, 46, 0), (63, 46, 0), (63, 47, 0), (63, 47, 0), (63, 48, 0), (63, 48, 0), (63, 49, 0), (63, 49, 0),
    (63, 50, 0), (63, 50, 0), (63, 51, 0), (63, 52, 0), (63, 52, 0), (63, 52, 0), (63, 52, 0), (63, 52, 0),
    (63, 53, 0), (63, 53, 0), (63, 53, 0), (63, 53, 0), (63, 54, 0), (63, 54, 0), (63, 54, 0), (63, 54, 0),
    (63, 54, 0), (63, 55, 0), (63, 55, 0), (63, 55, 0), (63, 55, 0), (63, 56, 0), (63, 56, 0), (63, 56, 0),
    (63, 56, 0), (63, 57, 0), (63, 57, 0), (63, 57, 0), (63, 57, 0), (63, 57, 0), (63, 58, 0), (63, 58, 0),
    (63, 58, 0), (63, 58, 0), (63, 59, 0), (63, 59, 0), (63, 59, 0), (63, 59, 0), (63, 60, 0), (63, 60, 0),
    (63, 60, 0), (63, 60, 0), (63, 60, 0), (63, 61, 0), (63, 61, 0), (63, 61, 0), (63, 61, 0), (63, 62, 0),
    (63, 62, 0), (63, 62, 0), (63, 62, 0), (63, 63, 0), (63, 63, 1), (63, 63, 2), (63, 63, 3), (63, 63, 4),
    (63, 63, 5), (63, 63, 6), (63, 63, 7), (63, 63, 8), (63, 63, 9), (63, 63, 10), (63, 63, 10), (63, 63, 11),
    (63, 63, 12), (63, 63, 13), (63, 63, 14), (63, 63, 15), (63, 63, 16), (63, 63, 17), (63, 63, 18), (63, 63, 19),
    (63, 63, 20), (63, 63, 21), (63, 63, 21), (63, 63, 22), (63, 63, 23), (63, 63, 24), (63, 63, 25), (63, 63, 26),
    (63, 63, 27), (63, 63, 28), (63, 63, 29), (63, 63, 30), (63, 63, 31), (63, 63, 31), (63, 63, 32), (63, 63, 33),
    (63, 63, 34), (63, 63, 35), (63, 63, 36), (63, 63, 37), (63, 63, 38), (63, 63, 39), (63, 63, 40), (63, 63, 41),
    (63, 63, 42), (63, 63, 42), (63, 63, 43), (63, 63, 44), (63, 63, 45), (63, 63, 46), (63, 63, 47), (63, 63, 48),
    (63, 63, 49), (63, 63, 50), (63, 63, 51), (63, 63, 52), (63, 63, 52), (63, 63, 53), (63, 63, 54), (63, 63, 55),
    (63, 63, 56), (63, 63, 57), (63, 63, 58), (63, 63, 59), (63, 63, 60), (63, 63, 61), (63, 63, 62), (63, 63, 63),
];

/// Printable ASCII with the ink in each quadrant (top-left, top-right,
/// bottom-left, bottom-right), measured from aalib's 8×16 font and normalized
/// the way aalib's `__aa_calcparams` does.
#[rustfmt::skip]
const GLYPHS: [(char, [u8; 4]); 95] = [
    (' ', [0, 0, 0, 0]), ('!', [177, 177, 59, 59]), ('"', [137, 137, 0, 0]), ('#', [235, 216, 196, 177]),
    ('$', [255, 216, 157, 196]), ('%', [98, 118, 137, 78]), ('&', [255, 196, 177, 157]), ('\'', [157, 0, 0, 0]),
    ('(', [177, 59, 98, 59]), (')', [59, 177, 59, 98]), ('*', [157, 157, 78, 78]), ('+', [98, 98, 39, 39]),
    (',', [0, 0, 98, 59]), ('-', [59, 59, 0, 0]), ('.', [0, 0, 39, 39]), ('/', [20, 118, 137, 0]),
    ('0', [255, 255, 196, 157]), ('1', [177, 118, 118, 118]), ('2', [157, 177, 196, 98]), ('3', [137, 235, 98, 157]),
    ('4', [216, 255, 20, 177]), ('5', [255, 137, 98, 157]), ('6', [255, 98, 177, 157]), ('7', [137, 235, 157, 0]),
    ('8', [255, 235, 177, 157]), ('9', [235, 255, 59, 137]), (':', [39, 39, 39, 39]), (';', [39, 39, 78, 39]),
    ('<', [98, 98, 59, 98]), ('=', [59, 59, 59, 59]), ('>', [98, 98, 98, 59]), ('?', [177, 196, 59, 59]),
    ('@', [255, 255, 216, 137]), ('A', [255, 196, 157, 157]), ('B', [255, 235, 196, 157]), ('C', [235, 98, 157, 98]),
    ('D', [255, 216, 196, 137]), ('E', [255, 177, 196, 118]), ('F', [255, 177, 196, 0]), ('G', [255, 157, 157, 157]),
    ('H', [255, 255, 157, 157]), ('I', [137, 137, 98, 98]), ('J', [20, 255, 177, 137]), ('K', [255, 196, 177, 157]),
    ('L', [255, 0, 196, 118]), ('M', [255, 255, 157, 157]), ('N', [255, 255, 157, 157]), ('O', [255, 235, 177, 157]),
    ('P', [255, 196, 196, 0]), ('Q', [255, 235, 216, 255]), ('R', [255, 235, 177, 157]), ('S', [216, 177, 137, 157]),
    ('T', [235, 235, 98, 98]), ('U', [235, 235, 177, 157]), ('V', [235, 235, 137, 137]), ('W', [255, 255, 216, 216]),
    ('X', [196, 196, 157, 157]), ('Y', [216, 216, 98, 98]), ('Z', [196, 216, 196, 137]), ('[', [235, 39, 157, 39]),
    ('\\', [216, 20, 20, 157]), (']', [39, 235, 39, 157]), ('^', [137, 98, 0, 0]), ('_', [0, 0, 78, 78]),
    ('`', [98, 20, 0, 0]), ('a', [118, 98, 177, 157]), ('b', [255, 98, 177, 157]), ('c', [137, 78, 177, 78]),
    ('d', [137, 235, 177, 157]), ('e', [177, 137, 177, 78]), ('f', [255, 78, 196, 0]), ('g', [137, 118, 255, 255]),
    ('h', [255, 118, 177, 157]), ('i', [118, 98, 98, 98]), ('j', [0, 216, 118, 255]), ('k', [255, 98, 196, 137]),
    ('l', [137, 118, 98, 98]), ('m', [196, 177, 235, 235]), ('n', [137, 118, 157, 157]), ('o', [137, 118, 177, 157]),
    ('p', [137, 118, 255, 157]), ('q', [137, 118, 196, 255]), ('r', [157, 118, 196, 0]), ('s', [137, 78, 137, 137]),
    ('t', [255, 39, 137, 78]), ('u', [118, 118, 177, 157]), ('v', [118, 118, 137, 137]), ('w', [118, 118, 235, 235]),
    ('x', [118, 118, 137, 137]), ('y', [118, 118, 255, 255]), ('z', [137, 118, 196, 98]), ('{', [137, 137, 59, 118]),
    ('|', [98, 98, 78, 78]), ('}', [137, 137, 118, 59]), ('~', [118, 78, 0, 0]),
];

/// Brightness levels per quadrant in the glyph lookup table.
const LEVELS: usize = 8;

/// For every combination of four quadrant levels (3 bits each), the index of
/// the best-matching glyph, using aalib's distance (quadrants weigh double,
/// plus the total ink).
fn glyph_table() -> &'static [u8; 4096] {
    static TABLE: OnceLock<[u8; 4096]> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = [0u8; 4096];
        for (idx, slot) in t.iter_mut().enumerate() {
            let want = [idx >> 9, (idx >> 6) & 7, (idx >> 3) & 7, idx & 7]
                .map(|l| (l * 255 / (LEVELS - 1)) as i32);
            let want_sum: i32 = want.iter().sum();
            let mut best = (i32::MAX, 0u8);
            for (gi, (_, q)) in GLYPHS.iter().enumerate() {
                let mut d = 0;
                let mut sum = 0;
                for k in 0..4 {
                    let e = q[k] as i32 - want[k];
                    d += 2 * e * e;
                    sum += q[k] as i32;
                }
                d += (sum - want_sum).pow(2);
                if d < best.0 {
                    best = (d, gi as u8);
                }
            }
            *slot = best.1;
        }
        t
    })
}

/// Extra columns on each side of the heat buffer, so the kernel can read its
/// neighbours (shifted by up to two columns of wind) without bounds checks.
const PAD: usize = 3;
/// Simulated rows hidden below the screen, so the raw fuel noise has been
/// smoothed once or twice before it shows.
const HIDDEN: usize = 2;
/// Fuel rows below those.
const FUEL: usize = 2;
/// Heat is kept in fixed point (×8), so the per-row decay keeps its precision
/// on tall screens instead of being swamped by rounding.
const FP: u32 = 8;
/// aafire's `minus = 800 / YSIZ` plus its rounding loss, as heat lost over the
/// whole height: flames at `height = 1` reach about as high as aafire's.
const REACH: f32 = 180.0;
/// Side of the tiling cooling map (a power of two).
const CMAP: usize = 64;

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Blocks,
    Ascii,
}

/// A numeric option clamped to its range; non-finite values get the default.
fn num(v: f32, default: f32, lo: f32, hi: f32) -> f32 {
    if v.is_finite() {
        v.clamp(lo, hi)
    } else {
        default
    }
}

#[derive(Clone, Copy)]
struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    heat: f32,
    cool: f32,
    phase: f32,
}

struct Fire {
    style: Style,
    intensity: f32,
    height: f32,
    wind: f32,
    cooling: f32,
    sparks_on: bool,
    rng: fastrand::Rng,
    w: usize,
    h: usize,
    /// Simulation width in pixels (two per cell, like aafire's image).
    sw: usize,
    stride: usize,
    /// Visible simulation rows (two per cell).
    vis: usize,
    heat: Vec<u16>,
    minus: u32,
    /// Fuel envelope slope near the side edges (aafire: 4 per pixel).
    taper: u32,
    /// aafire's start-up ramp: the fuel cap grows step by step.
    warm: u32,
    /// Smooth random field subtracted as flames rise (scrolls with them).
    cmap: Vec<u8>,
    cscroll: usize,
    cool_k: u32,
    acc: f32,
    step_dt: f32,
    t: f32,
    sparks: Vec<Spark>,
    spawn_acc: f32,
    px: Pixels,
    /// Heat → display color, already faded toward the background; `Reset` when cold.
    lut: [Color; 256],
    /// Heat → quadrant brightness level for glyph matching.
    level: [u8; 256],
}

impl Fire {
    fn new(ctx: &Ctx) -> Fire {
        let o = ctx.opts;
        let style = match o.s("style").to_ascii_lowercase().as_str() {
            "ascii" | "aa" | "text" | "chars" => Style::Ascii,
            _ => Style::Blocks,
        };
        let mut rng = ctx.rng();
        // A smooth random cooling map, tiling in both directions.
        let mut cmap: Vec<u8> = (0..CMAP * CMAP).map(|_| rng.u8(..)).collect();
        let mut tmp = vec![0u8; CMAP * CMAP];
        for _ in 0..6 {
            for y in 0..CMAP {
                for x in 0..CMAP {
                    let at = |dx: usize, dy: usize| {
                        cmap[((y + dy) % CMAP) * CMAP + (x + dx) % CMAP] as u32
                    };
                    let s = at(0, 0) * 4 + at(1, 0) + at(CMAP - 1, 0) + at(0, 1) + at(0, CMAP - 1);
                    tmp[y * CMAP + x] = (s / 8) as u8;
                }
            }
            std::mem::swap(&mut cmap, &mut tmp);
        }
        // Stretch the smoothed field back to the full range, keeping only the peaks.
        let (lo, hi) = cmap
            .iter()
            .fold((255u8, 0u8), |(l, h), &v| (l.min(v), h.max(v)));
        let span = (hi.saturating_sub(lo)).max(1) as f32;
        for v in &mut cmap {
            let t = ((*v - lo) as f32 / span - 0.45).max(0.0) / 0.55;
            *v = (t * t * 255.0) as u8;
        }
        let mut level = [0u8; 256];
        for (i, l) in level.iter_mut().enumerate() {
            let (r, g, b) = PAL[i];
            // aalib's gray: 0.30 R + 0.59 G + 0.11 B of the palette entry.
            let gray = (r as f32 * 0.30 + g as f32 * 0.59 + b as f32 * 0.11) / 63.0;
            *l = ((gray * 1.1) * (LEVELS - 1) as f32)
                .round()
                .clamp(0.0, (LEVELS - 1) as f32) as u8;
        }
        Fire {
            style,
            intensity: num(o.f("intensity"), 1.0, 0.05, 4.0),
            height: num(o.f("height"), 0.8, 0.05, 3.0),
            wind: num(o.f("wind"), 0.0, -2.0, 2.0),
            cooling: num(o.f("cooling"), 0.5, 0.0, 4.0),
            sparks_on: o.b("sparks"),
            rng,
            w: 0,
            h: 0,
            sw: 0,
            stride: 0,
            vis: 0,
            heat: Vec::new(),
            minus: 0,
            taper: 4,
            warm: 0,
            cmap,
            cscroll: 0,
            cool_k: 0,
            acc: 0.0,
            step_dt: 1.0 / 36.0,
            t: 0.0,
            sparks: Vec::new(),
            spawn_acc: 0.0,
            px: Pixels::new(1, 1),
            lut: [Color::Reset; 256],
            level,
        }
    }

    /// One aafire frame: new fuel at the bottom, then everything moves up a row.
    fn step(&mut self) {
        let (sw, stride, rows) = (self.sw, self.stride, self.vis + HIDDEN);
        // aafire grows its fuel cap by one per frame; we warm up a little faster.
        self.warm = (self.warm + 5).min(256);
        let f0 = rows * stride + PAD;
        let f1 = f0 + stride;
        let mut x = 0;
        while x < sw {
            // Runs of 0-5 pixels share a hot spot that wanders as it goes.
            let edge = (1 + self.taper * x as u32).min(1 + self.taper * (sw - 1 - x) as u32);
            let cap = edge.min(self.warm).clamp(1, 256);
            let mut last = (self.rng.u32(0..cap) as f32 * self.intensity) as i32;
            for _ in 0..self.rng.u32(0..6) {
                if x >= sw {
                    break;
                }
                self.heat[f0 + x] = last.clamp(0, 255) as u16 * FP as u16;
                last += self.rng.i32(-2..4);
                self.heat[f1 + x] = last.clamp(0, 255) as u16 * FP as u16;
                last += self.rng.i32(-2..4);
                x += 1;
            }
            // Like aafire, the pixel after a run keeps last frame's fuel.
            x += 1;
        }

        let wind = if self.wind == 0.0 {
            0.0
        } else {
            // Gusts: the lean breathes a little.
            let g = (self.t * 0.37).sin() * 0.6 + (self.t * 1.13).sin() * 0.4;
            (self.wind * (0.8 + 0.25 * g)).clamp(-2.0, 2.0)
        };
        let aw = wind.abs();
        let minus = self.minus;
        let cool_k = self.cool_k;
        self.cscroll = (self.cscroll + 1) % CMAP;
        for y in 0..rows {
            let d = {
                let whole = aw.floor();
                let extra = (self.rng.f32() < aw - whole) as i32;
                ((whole as i32 + extra).min(2)) * if wind < 0.0 { -1 } else { 1 }
            };
            // Each row reads the two rows below it, which still hold the last
            // step's heat: the whole fire rises by one row per step.
            let (top, below) = self.heat.split_at_mut((y + 1) * stride);
            let out = &mut top[y * stride + PAD..y * stride + PAD + sw];
            let o = (PAD as i32 + d - 1) as usize;
            let r1 = &below[o..o + sw + 2];
            let r2 = &below[stride + o..stride + o + sw + 2];
            // Turbulence bites harder the higher the flames get, so the base
            // stays solid while the tips break into tongues.
            let row_k = cool_k * (rows - y) as u32 / rows as u32;
            if row_k == 0 {
                for (x, px) in out.iter_mut().enumerate() {
                    let s = r1[x] as u32
                        + r1[x + 1] as u32
                        + r1[x + 2] as u32
                        + r2[x] as u32
                        + r2[x + 2] as u32;
                    *px = (s.saturating_sub(minus) / 5) as u16;
                }
            } else {
                let crow = &self.cmap[((y + self.cscroll) % CMAP) * CMAP..][..CMAP];
                for (x, px) in out.iter_mut().enumerate() {
                    let s = r1[x] as u32
                        + r1[x + 1] as u32
                        + r1[x + 2] as u32
                        + r2[x] as u32
                        + r2[x + 2] as u32;
                    let c = crow[x & (CMAP - 1)] as u32 * row_k;
                    *px = (s.saturating_sub(minus + c) / 5) as u16;
                }
            }
        }
    }

    #[inline]
    fn heat_at(&self, x: usize, y: usize) -> u32 {
        self.heat[y * self.stride + PAD + x] as u32 / FP
    }

    fn update_sparks(&mut self, dt: f32) {
        if !self.sparks_on || self.vis < 4 {
            self.sparks.clear();
            return;
        }
        // The flames rise one row per step; embers start out about as fast.
        let px_per_s = 1.0 / self.step_dt;
        // Embers break off the flame tips.
        self.spawn_acc += dt * self.w as f32 * 0.12 * self.intensity.min(2.0);
        while self.spawn_acc >= 1.0 {
            self.spawn_acc -= 1.0;
            if self.sparks.len() >= self.sparks.capacity() {
                continue;
            }
            let x = self.rng.usize(0..self.sw);
            let Some(y) = (0..self.vis).find(|&y| self.heat_at(x, y) > 60) else {
                continue;
            };
            if y + 2 >= self.vis {
                continue;
            }
            let heat = 170.0 + self.rng.f32() * 85.0;
            self.sparks.push(Spark {
                x: x as f32,
                y: y as f32,
                vx: (self.rng.f32() - 0.5) * 6.0,
                vy: -px_per_s * (0.5 + self.rng.f32() * 0.6),
                heat: heat.min(255.0),
                cool: 45.0 + self.rng.f32() * 80.0,
                phase: self.rng.f32() * std::f32::consts::TAU,
            });
        }
        let t = self.t;
        let wind = self.wind * px_per_s * 0.45;
        let (sw, vis) = (self.sw as f32, self.vis as f32);
        self.sparks.retain_mut(|s| {
            s.phase += dt * 3.0;
            let sway = s.phase.sin() * 5.0 + (t * 0.9 + s.phase * 0.3).sin() * 3.0;
            s.vx += (wind + sway - s.vx) * (dt * 1.5).min(1.0);
            // Buoyant while hot; they slow down as they cool.
            s.vy += px_per_s * 4.0 * dt * (1.0 - s.heat / 255.0);
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            s.heat -= s.cool * dt;
            s.heat > 8.0 && s.y > -2.0 && s.x > -2.0 && s.x < sw + 2.0 && s.y < vis
        });
    }

    fn draw_blocks(&mut self, c: &mut Canvas) {
        self.px.clear();
        let (w, vis, stride) = (self.w, self.vis, self.stride);
        for y in 0..vis {
            let row = &self.heat[y * stride + PAD..y * stride + PAD + 2 * w];
            for x in 0..w {
                let v = (row[2 * x] as u32 + row[2 * x + 1] as u32) / (2 * FP);
                let col = self.lut[v.min(255) as usize];
                if col != Color::Reset {
                    self.px.set(x as i32, y as i32, col);
                }
            }
        }
        for s in &self.sparks {
            let (x, y) = (s.x as i32, s.y as i32);
            if x < 0 || y < 0 || x as usize >= self.sw || y as usize >= vis {
                continue;
            }
            // Only where they stand out against the flames behind them.
            if (self.heat_at(x as usize, y as usize) as f32) < s.heat * 0.6 {
                self.px.set(x / 2, y, self.lut[s.heat as usize]);
            }
        }
        self.px.blit(c);
    }

    fn draw_ascii(&self, c: &mut Canvas) {
        let table = glyph_table();
        let stride = self.stride;
        for cy in 0..self.h {
            let r0 = &self.heat[2 * cy * stride + PAD..][..2 * self.w];
            let r1 = &self.heat[(2 * cy + 1) * stride + PAD..][..2 * self.w];
            for cx in 0..self.w {
                let q = [r0[2 * cx], r0[2 * cx + 1], r1[2 * cx], r1[2 * cx + 1]]
                    .map(|v| (v as u32 / FP).min(255) as usize);
                let hot = q[0].max(q[1]).max(q[2]).max(q[3]);
                // Thin glyphs look darker than solid blocks: color them a little hotter.
                let col = self.lut[(hot + hot / 5 + 8).min(255)];
                if col == Color::Reset {
                    continue;
                }
                let l = q.map(|v| self.level[v] as usize);
                let g = table[(l[0] << 9) | (l[1] << 6) | (l[2] << 3) | l[3]] as usize;
                if g == 0 {
                    continue;
                }
                let attr = if hot > 235 { BOLD } else { 0 };
                c.set_attr(cx as i32, cy as i32, GLYPHS[g].0, col, attr);
            }
        }
        for s in &self.sparks {
            let (x, y) = (s.x.floor() as i32, s.y.floor() as i32);
            if x < 0 || y < 0 || x as usize >= self.sw || y as usize >= self.vis {
                continue;
            }
            let (cx, cy) = (x / 2, y / 2);
            if c.get(cx, cy).is_some_and(|cell| cell.ch != ' ') {
                continue;
            }
            let ch = match (s.heat > 200.0, y & 1) {
                (true, _) => '*',
                (false, 0) => '\'',
                _ => '.',
            };
            c.set(cx, cy, ch, self.lut[s.heat as usize]);
        }
    }
}

impl Scene for Fire {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        let (w, h) = (w.max(1) as usize, h.max(1) as usize);
        self.w = w;
        self.h = h;
        self.sw = 2 * w;
        self.stride = self.sw + 2 * PAD;
        self.vis = 2 * h;
        self.heat.clear();
        self.heat
            .resize(self.stride * (self.vis + HIDDEN + FUEL), 0);
        // aafire: minus = 800 / YSIZ, so flames always reach about as high
        // relative to the screen. Heat lost per row, in the kernel's units
        // (a sum of five fixed-point pixels).
        let per_row = REACH / (self.vis as f32 * self.height);
        self.minus = (per_row * 5.0 * FP as f32).round() as u32;
        // Turbulent cooling, scaled with the decay so it bites at any height.
        self.cool_k = (self.cooling * per_row * 0.16 * FP as f32).round() as u32;
        // Fuel tapers off near the sides: aafire's 4 per pixel, but never over
        // more than a fifth of the width.
        self.taper = (256.0 / (self.sw as f32 * 0.2)).ceil().max(4.0) as u32;
        // Taller fires rise faster, like real flames (and aafire on a fast machine).
        self.step_dt = 1.0 / (36.0 * (self.vis as f32 / 60.0).sqrt().clamp(0.6, 3.0));
        self.px.resize(w as u16, h as u16);
        self.sparks.clear();
        self.sparks.reserve_exact((w / 2).max(4));
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let native: Vec<Color> = PAL
            .iter()
            .map(|&(r, g, b)| Color::Rgb(r * 4 + r / 16, g * 4 + g / 16, b * 4 + b / 16))
            .collect();
        let grad: Option<Gradient> = (!ctx.theme.native).then(|| {
            ctx.ramp(&[
                rgb(0x200000),
                rgb(0xb01000),
                rgb(0xff6000),
                rgb(0xffd000),
                rgb(0xffffff),
            ])
        });
        let base = |i: usize| match &grad {
            // Spend the theme ramp the way aafire spends its palette:
            // most of the heat range on the warm middle.
            Some(g) => g.at(((i as f32 - 10.0) / 200.0).clamp(0.0, 1.0).powf(0.75)),
            None => native[i],
        };
        // With a known background the coolest part fades into it. Without one
        // (a transparent terminal that did not say what is behind it), fading
        // could only darken toward black and leave dark halos, so the flames
        // end at a visible red instead. Zero heat is transparent either way.
        let known_bg = matches!(ctx.bg, Color::Rgb(..));
        const CUT: usize = 26;
        const FIRST: usize = 30;
        for (i, slot) in self.lut.iter_mut().enumerate() {
            *slot = if known_bg {
                let t = ((i as f32 - 10.0) / 70.0).clamp(0.0, 1.0);
                let alpha = t * t * (3.0 - 2.0 * t);
                if alpha < 0.04 {
                    Color::Reset
                } else {
                    ctx.fade(base(i), 1.0 - alpha)
                }
            } else if i < CUT {
                Color::Reset
            } else {
                base(FIRST + (i - CUT) * (255 - FIRST) / (255 - CUT))
            };
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.25)
        } else {
            0.0
        };
        self.t += dt;
        self.acc += dt;
        let mut n = 0;
        while self.acc >= self.step_dt {
            self.acc -= self.step_dt;
            self.step();
            n += 1;
            if n >= 8 {
                self.acc = 0.0;
                break;
            }
        }
        self.update_sparks(dt);
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        match self.style {
            Style::Blocks => self.draw_blocks(c),
            Style::Ascii => self.draw_ascii(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::Theme;

    /// Both styles and odd options survive tiny and huge screens and huge steps.
    #[test]
    fn variants_survive() {
        let variants = [
            "style = 'blocks'",
            "style = 'ascii'\nwind = -2.0",
            "wind = 2.0\ncooling = 4.0\nintensity = 4.0\nheight = 3.0",
            "intensity = 0.0\nheight = 0.0\nsparks = false",
            "intensity = nan\nheight = inf\nwind = nan\ncooling = -inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("ember").unwrap()] {
                let opts = Opts::new(INFO.options, v.parse().unwrap());
                let ctx = Ctx {
                    theme: &theme,
                    opts: &opts,
                    bg: theme.bg,
                    seed: 5,
                    fiesta: crate::fiesta::Fiesta::NONE,
                };
                let mut s = (INFO.make)(&ctx);
                let sizes = [(80, 24), (1, 1), (2, 3), (7, 2), (500, 150), (3, 40)];
                for (i, &(w, h)) in sizes.iter().enumerate() {
                    s.resize(&ctx, w, h);
                    if i == 0 {
                        s.retheme(&ctx);
                    }
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for f in 0..40 {
                        s.update(&ctx, if f % 13 == 12 { 5.0 } else { 1.0 / 30.0 });
                        c.clear();
                        s.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }

    /// Zero heat is never painted: the terminal background shows through.
    #[test]
    fn cold_is_transparent() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut f = Fire::new(&ctx);
        f.resize(&ctx, 40, 12);
        f.retheme(&ctx);
        assert_eq!(f.lut[0], Color::Reset);
        let mut c = Canvas::new(40, 12, Color::Reset);
        f.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch == ' '));
        assert!(c.cells().iter().all(|cell| cell.bg == Color::Reset));
    }
}
