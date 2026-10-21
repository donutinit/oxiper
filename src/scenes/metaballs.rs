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

use crate::color::Color;
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
        }
    }

    /// Slot colors of color set `s`: tanim's own, or the theme's accents
    /// rotated by one per set so the blobs still change hands over time.
    fn slots(ctx: &Ctx, s: usize) -> [Rgb; 4] {
        let mut out = SETS[s];
        for (i, c) in out.iter_mut().enumerate() {
            *c = rgb_of(ctx.pick(i + s, color(SETS[s][i])));
        }
        out
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
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.cur = Self::slots(ctx, self.set);
        self.nxt = Self::slots(ctx, self.next);
        if ctx.theme.native {
            (self.bg_top, self.bg_bot) = (BG_TOP, BG_BOT);
        } else {
            let b = rgb_of(ctx.theme.bg);
            (self.bg_top, self.bg_bot) = (b, b);
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        if dt.is_finite() {
            self.t += dt.clamp(0.0, 5.0) * FPS * self.speed;
        }
    }

    /// Motion follows the speed setting; the color schedule keeps real time.
    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        if !real.is_finite() {
            return;
        }
        self.clock += real.clamp(0.0, 5.0);
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
        for (l, b) in self.live.iter_mut().zip(&self.balls) {
            l.x = wf * (0.5 + 0.4 * (self.t * b.fx + b.px).sin());
            l.y = phf * (0.5 + 0.38 * (self.t * b.fy + b.py).sin());
            l.r2 = (b.r * self.size * side).powi(2);
            l.col = mix(self.cur[b.slot], self.nxt[b.slot], blend);
        }

        // Light from the upper left, towards the viewer.
        let (lx, ly, lz) = (-0.45f32, -0.55f32, 0.70f32);
        let norm = (lx * lx + ly * ly + lz * lz).sqrt();
        let (lx, ly, lz) = (lx / norm, ly / norm, lz / norm);
        // Halfway vector between the light and the viewer, for highlights.
        let (hx, hy, hz) = (lx, ly, lz + 1.0);
        let hn = (hx * hx + hy * hy + hz * hz).sqrt();
        let (hx, hy, hz) = (hx / hn, hy / hn, hz / hn);
        // How steeply the domes curve at their rims.
        let steep = side * 0.13 * self.size;

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
                    let lit = 0.25 + 0.85 * diff;
                    let k = s12 * s12 * 0.85;
                    let b0 = (c00[i] + (c01[i] - c00[i]) * t) * inv;
                    let b1 = (c10[i] + (c11[i] - c10[i]) * t) * inv;
                    let b2 = (c20[i] + (c21[i] - c20[i]) * t) * inv;
                    rb0[i] = b0;
                    rb1[i] = b1;
                    rb2[i] = b2;
                    ro0[i] = b0 * lit + (255.0 - b0 * lit) * k;
                    ro1[i] = b1 * lit + (255.0 - b1 * lit) * k;
                    ro2[i] = b2 * lit + (255.0 - b2 * lit) * k;
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
            fiesta: crate::fiesta::Fiesta::NONE,
        }
    }

    /// Odd options and sizes survive long runs and huge steps, in every color mode.
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
                for bg in [Color::Reset, theme.bg] {
                    let opts = Opts::new(INFO.options, v.parse().unwrap());
                    let ctx = ctx_with(&theme, &opts, bg);
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
}
