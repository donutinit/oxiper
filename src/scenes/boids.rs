//! boids: a flock that steers by separation, alignment and cohesion.
//!
//! Simulated in "units" of one cell width on both axes (a cell is 1×2 units,
//! a braille dot 0.5×0.5), so speeds and vision are the same in x and y.
//! Neighbors are found with a uniform grid rebuilt every step (counting
//! sort, no allocation), which keeps thousands of boids cheap.

use std::f32::consts::{PI, TAU};

use crate::color::{Color, mix_oklab, rgb};
use crate::render::{BOLD, Braille, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "boids",
    aliases: &["flock"],
    about: "a flock that steers by separation, alignment and cohesion",
    options: &[
        Opt {
            key: "count",
            default: Val::I(110),
            help: "Boids on a 100x30 screen; scales with the area",
        },
        Opt {
            key: "separation",
            default: Val::F(1.0),
            help: "How strongly boids keep their distance",
        },
        Opt {
            key: "alignment",
            default: Val::F(1.0),
            help: "How strongly boids match their neighbors' heading",
        },
        Opt {
            key: "cohesion",
            default: Val::F(1.0),
            help: "How strongly boids move toward their neighbors",
        },
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Flying speed multiplier",
        },
        Opt {
            key: "vision",
            default: Val::F(10.0),
            help: "How far a boid sees, in cells",
        },
        Opt {
            key: "predator",
            default: Val::B(true),
            help: "A hawk that hunts the flock",
        },
        Opt {
            key: "trails",
            default: Val::I(6),
            help: "Trail length in steps (0 = none)",
        },
        Opt {
            key: "style",
            default: Val::S("arrows"),
            help: "arrows (8-way heading glyphs) or dots (braille points)",
        },
        Opt {
            key: "color",
            default: Val::S("flock"),
            help: "Color by flock or by heading",
        },
        Opt {
            key: "bounce",
            default: Val::B(false),
            help: "Turn away from the edges instead of wrapping around",
        },
    ],
    make: |ctx| Box::new(Boids::new(ctx)),
};

/// Cruising speed in units per second at speed 1.
const MAX_SPEED: f32 = 13.0;
const MAX_FORCE: f32 = 26.0;
/// Boids closer than this (in units) push each other apart.
const SEP_RADIUS: f32 = 2.6;
const MAX_BOIDS: usize = 6000;
const MAX_TRAIL: usize = 24;
/// Trail samples per second, independent of the frame rate.
const TRAIL_HZ: f32 = 20.0;
const STEP: f32 = 1.0 / 30.0;
const FLOOR: f32 = 0.45;
const HEAD_LUT: usize = 64;

const ARROWS: [char; 8] = ['→', '↘', '↓', '↙', '←', '↖', '↑', '↗'];
const HAWK: [char; 8] = ['⇒', '⇘', '⇓', '⇙', '⇐', '⇖', '⇑', '⇗'];
const NATIVE_FLOCKS: [u32; 4] = [0x5ee6c4, 0xff8f7a, 0xffd166, 0x8ab4ff];
const NATIVE_WHEEL: [u32; 5] = [0x5ee6c4, 0x8ab4ff, 0xc79bff, 0xff8f7a, 0xffd166];
const NATIVE_HAWK: u32 = 0xff4d6d;

#[derive(Clone, Copy, Default)]
struct Boid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    flock: u8,
}

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Arrows,
    Dots,
}

pub struct Boids {
    rng: fastrand::Rng,
    density: f32,
    w_sep: f32,
    w_ali: f32,
    w_coh: f32,
    max_speed: f32,
    vision: f32,
    hawk_on: bool,
    trail_len: usize,
    style: Style,
    by_heading: bool,
    bounce: bool,
    /// Field size in units.
    fw: f32,
    fh: f32,
    boids: Vec<Boid>,
    acc: Vec<(f32, f32)>,
    flocks: u8,
    hawk: Boid,
    hawk_target: usize,
    hawk_retarget: f32,
    /// Ring buffers of past positions: `MAX_TRAIL` per boid, plus the hawk's at the end.
    trail: Vec<(f32, f32)>,
    trail_head: usize,
    trail_fill: usize,
    trail_clock: f32,
    gw: usize,
    gh: usize,
    starts: Vec<u32>,
    order: Vec<u32>,
    cell_of: Vec<u32>,
    br: Braille,
    flock_col: [Color; 4],
    wheel: [Color; HEAD_LUT],
    hawk_col: Color,
}

impl Boids {
    fn new(ctx: &Ctx) -> Boids {
        let o = ctx.opts;
        let style = match o.s("style") {
            "dots" | "braille" | "points" => Style::Dots,
            _ => Style::Arrows,
        };
        let mut trail_len = o.i("trails").clamp(0, MAX_TRAIL as i64) as usize;
        if style == Style::Dots && trail_len == 0 {
            // A lone dot cannot show where it is heading.
            trail_len = 3;
        }
        Boids {
            rng: ctx.rng(),
            density: o.f("count").clamp(0.0, 100_000.0) / 3000.0,
            w_sep: o.f("separation").clamp(0.0, 20.0),
            w_ali: o.f("alignment").clamp(0.0, 20.0),
            w_coh: o.f("cohesion").clamp(0.0, 20.0),
            max_speed: MAX_SPEED * o.f("speed").clamp(0.0, 20.0),
            vision: o.f("vision").clamp(1.0, 100.0),
            hawk_on: o.b("predator"),
            trail_len,
            style,
            by_heading: o.s("color") == "heading",
            bounce: o.b("bounce"),
            fw: 1.0,
            fh: 1.0,
            boids: Vec::new(),
            acc: Vec::new(),
            flocks: 1,
            hawk: Boid::default(),
            hawk_target: 0,
            hawk_retarget: 0.0,
            trail: Vec::new(),
            trail_head: 0,
            trail_fill: 0,
            trail_clock: 0.0,
            gw: 1,
            gh: 1,
            starts: Vec::new(),
            order: Vec::new(),
            cell_of: Vec::new(),
            br: Braille::new(1, 1),
            flock_col: [Color::Reset; 4],
            wheel: [Color::Reset; HEAD_LUT],
            hawk_col: Color::Reset,
        }
    }

    fn random_boid(&mut self, flock: u8) -> Boid {
        let a = self.rng.f32() * TAU;
        let s = self.max_speed * (0.6 + 0.4 * self.rng.f32());
        Boid {
            x: self.rng.f32() * self.fw,
            y: self.rng.f32() * self.fh,
            vx: a.cos() * s,
            vy: a.sin() * s,
            flock,
        }
    }

    /// Shortest offset from `a` to `b` on a wrapping axis of length `len`.
    #[inline]
    fn delta(a: f32, b: f32, len: f32, wrap: bool) -> f32 {
        let d = b - a;
        if wrap {
            if d > len * 0.5 {
                d - len
            } else if d < -len * 0.5 {
                d + len
            } else {
                d
            }
        } else {
            d
        }
    }

    fn build_grid(&mut self) {
        let (gw, gh) = (self.gw, self.gh);
        let inv = 1.0 / self.vision;
        self.starts.iter_mut().for_each(|s| *s = 0);
        self.cell_of.clear();
        for b in &self.boids {
            let cx = ((b.x * inv) as usize).min(gw - 1);
            let cy = ((b.y * inv) as usize).min(gh - 1);
            let c = cy * gw + cx;
            self.cell_of.push(c as u32);
            self.starts[c + 1] += 1;
        }
        for i in 0..gw * gh {
            self.starts[i + 1] += self.starts[i];
        }
        self.order.resize(self.boids.len(), 0);
        for (i, &c) in self.cell_of.iter().enumerate() {
            let slot = &mut self.starts[c as usize];
            self.order[*slot as usize] = i as u32;
            *slot += 1;
        }
        for c in (1..=gw * gh).rev() {
            self.starts[c] = self.starts[c - 1];
        }
        self.starts[0] = 0;
    }

    /// The distinct neighbor cell offsets along one axis (fewer when the grid is tiny).
    fn offsets(n: usize, wrap: bool) -> ([isize; 3], usize) {
        match (n, wrap) {
            (1, _) => ([0, 0, 0], 1),
            (2, true) => ([0, 1, 0], 2),
            _ => ([-1, 0, 1], 3),
        }
    }

    fn step(&mut self, dt: f32) {
        let n = self.boids.len();
        let wrap = !self.bounce;
        let (fw, fh) = (self.fw, self.fh);
        let vis2 = self.vision * self.vision;
        // Personal space: a couple of cells, whatever the vision.
        let sep_r = SEP_RADIUS.min(self.vision);
        let sep2 = sep_r * sep_r;
        let ms = self.max_speed.max(0.01);
        let min_speed = ms * 0.45;
        let flee_r = self.vision * 1.6;
        if n > 0 {
            self.build_grid();
        }
        let (gw, gh) = (self.gw, self.gh);
        let (ox, nox) = Self::offsets(gw, wrap);
        let (oy, noy) = Self::offsets(gh, wrap);
        let inv = 1.0 / self.vision;
        let hawk = self.hawk;
        let hawk_on = self.hawk_on;

        for i in 0..n {
            let b = self.boids[i];
            let (bcx, bcy) = (
                ((b.x * inv) as usize).min(gw - 1) as isize,
                ((b.y * inv) as usize).min(gh - 1) as isize,
            );
            let (mut sx, mut sy) = (0.0f32, 0.0f32);
            let (mut ax, mut ay, mut cx, mut cy) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
            let mut seen = 0u32;
            let speed = (b.vx * b.vx + b.vy * b.vy).sqrt().max(1e-4);
            let (hx, hy) = (b.vx / speed, b.vy / speed);
            for &dyo in &oy[..noy] {
                let mut gy = bcy + dyo;
                if wrap {
                    gy = gy.rem_euclid(gh as isize);
                } else if gy < 0 || gy >= gh as isize {
                    continue;
                }
                for &dxo in &ox[..nox] {
                    let mut gx = bcx + dxo;
                    if wrap {
                        gx = gx.rem_euclid(gw as isize);
                    } else if gx < 0 || gx >= gw as isize {
                        continue;
                    }
                    let c = gy as usize * gw + gx as usize;
                    for k in self.starts[c] as usize..self.starts[c + 1] as usize {
                        let j = self.order[k] as usize;
                        if j == i {
                            continue;
                        }
                        let o = &self.boids[j];
                        let dx = Self::delta(b.x, o.x, fw, wrap);
                        let dy = Self::delta(b.y, o.y, fh, wrap);
                        let d2 = dx * dx + dy * dy;
                        if d2 >= vis2 || d2 < 1e-8 {
                            continue;
                        }
                        if d2 < sep2 {
                            // Push away, harder when closer.
                            let k = 1.0 / d2;
                            sx -= dx * k;
                            sy -= dy * k;
                        }
                        // Only flockmates in front (a ~280° field of view) steer the rest.
                        if o.flock == b.flock && (dx * hx + dy * hy) > -0.6 * d2.sqrt() {
                            ax += o.vx;
                            ay += o.vy;
                            cx += dx;
                            cy += dy;
                            seen += 1;
                        }
                    }
                }
            }
            let (mut fx, mut fy) = (0.0f32, 0.0f32);
            let steer = |dx: f32, dy: f32, vx: f32, vy: f32| -> (f32, f32) {
                let l = (dx * dx + dy * dy).sqrt();
                if l < 1e-6 {
                    return (0.0, 0.0);
                }
                let (tx, ty) = (dx / l * ms - vx, dy / l * ms - vy);
                let f = (tx * tx + ty * ty).sqrt();
                if f > MAX_FORCE {
                    (tx / f * MAX_FORCE, ty / f * MAX_FORCE)
                } else {
                    (tx, ty)
                }
            };
            if sx != 0.0 || sy != 0.0 {
                let (x, y) = steer(sx, sy, b.vx, b.vy);
                fx += x * 1.6 * self.w_sep;
                fy += y * 1.6 * self.w_sep;
            }
            if seen > 0 {
                let (x, y) = steer(ax, ay, b.vx, b.vy);
                fx += x * self.w_ali;
                fy += y * self.w_ali;
                let (x, y) = steer(cx, cy, b.vx, b.vy);
                fx += x * self.w_coh;
                fy += y * self.w_coh;
            }
            if hawk_on {
                let dx = Self::delta(hawk.x, b.x, fw, wrap);
                let dy = Self::delta(hawk.y, b.y, fh, wrap);
                let d2 = dx * dx + dy * dy;
                if d2 < flee_r * flee_r {
                    let k = 1.0 - d2.sqrt() / flee_r;
                    let (x, y) = steer(dx, dy, b.vx, b.vy);
                    fx += x * 4.0 * k;
                    fy += y * 4.0 * k;
                }
            }
            if !wrap {
                let (x, y) = self.edge_force(b.x, b.y);
                fx += x;
                fy += y;
            }
            // A little restlessness so the flock never freezes into a lattice.
            fx += (self.rng.f32() - 0.5) * 6.0;
            fy += (self.rng.f32() - 0.5) * 6.0;
            self.acc[i] = (fx, fy);
        }

        for (b, &(fx, fy)) in self.boids.iter_mut().zip(&self.acc) {
            b.vx += fx * dt;
            b.vy += fy * dt;
            let s = (b.vx * b.vx + b.vy * b.vy).sqrt();
            let target = s.clamp(min_speed, ms * 1.25);
            if s > 1e-6 {
                b.vx *= target / s;
                b.vy *= target / s;
            } else {
                b.vx = min_speed;
            }
            b.x += b.vx * dt;
            b.y += b.vy * dt;
            Self::confine(b, fw, fh, wrap);
        }

        if hawk_on {
            self.step_hawk(dt);
        }
    }

    /// Steering away from the walls in bounce mode.
    fn edge_force(&self, x: f32, y: f32) -> (f32, f32) {
        let m = (self.vision * 0.8)
            .min(self.fw * 0.25)
            .min(self.fh * 0.25)
            .max(0.5);
        let push = |p: f32, len: f32| -> f32 {
            if p < m {
                (m - p) / m
            } else if p > len - m {
                -(p - (len - m)) / m
            } else {
                0.0
            }
        };
        let k = MAX_FORCE * 2.5;
        (push(x, self.fw) * k, push(y, self.fh) * k)
    }

    fn confine(b: &mut Boid, fw: f32, fh: f32, wrap: bool) {
        if wrap {
            b.x = b.x.rem_euclid(fw);
            b.y = b.y.rem_euclid(fh);
            if b.x >= fw {
                b.x = 0.0;
            }
            if b.y >= fh {
                b.y = 0.0;
            }
        } else {
            if b.x < 0.0 || b.x >= fw {
                b.vx = -b.vx;
                b.x = b.x.clamp(0.0, fw - 0.001);
            }
            if b.y < 0.0 || b.y >= fh {
                b.vy = -b.vy;
                b.y = b.y.clamp(0.0, fh - 0.001);
            }
        }
    }

    fn step_hawk(&mut self, dt: f32) {
        let wrap = !self.bounce;
        let (fw, fh) = (self.fw, self.fh);
        let ms = self.max_speed.max(0.01);
        self.hawk_retarget -= dt;
        if !self.boids.is_empty()
            && (self.hawk_retarget <= 0.0 || self.hawk_target >= self.boids.len())
        {
            // Pick the closest of a few random boids: hunting, not homing.
            let mut best = (f32::MAX, 0);
            for _ in 0..6 {
                let j = self.rng.usize(..self.boids.len());
                let o = &self.boids[j];
                let dx = Self::delta(self.hawk.x, o.x, fw, wrap);
                let dy = Self::delta(self.hawk.y, o.y, fh, wrap);
                let d2 = dx * dx + dy * dy;
                if d2 < best.0 {
                    best = (d2, j);
                }
            }
            self.hawk_target = best.1;
            self.hawk_retarget = 2.5 + 2.0 * self.rng.f32();
        }
        let (mut fx, mut fy) = if wrap {
            (0.0, 0.0)
        } else {
            self.edge_force(self.hawk.x, self.hawk.y)
        };
        let h = &mut self.hawk;
        if let Some(t) = self.boids.get(self.hawk_target) {
            let dx = Self::delta(h.x, t.x, fw, wrap);
            let dy = Self::delta(h.y, t.y, fh, wrap);
            let l = (dx * dx + dy * dy).sqrt().max(1e-4);
            let sp = ms * 1.2;
            fx += (dx / l * sp - h.vx) * 1.4;
            fy += (dy / l * sp - h.vy) * 1.4;
        }
        // Heavier than a boid: it turns wide, which gives the flock a chance.
        let f = (fx * fx + fy * fy).sqrt();
        let lim = MAX_FORCE * 0.9;
        if f > lim {
            fx *= lim / f;
            fy *= lim / f;
        }
        h.vx += fx * dt;
        h.vy += fy * dt;
        let s = (h.vx * h.vx + h.vy * h.vy).sqrt();
        let target = s.clamp(ms * 0.5, ms * 1.3);
        if s > 1e-6 {
            h.vx *= target / s;
            h.vy *= target / s;
        } else {
            h.vx = ms * 0.5;
        }
        h.x += h.vx * dt;
        h.y += h.vy * dt;
        Self::confine(h, fw, fh, wrap);
    }

    fn record_trails(&mut self) {
        if self.trail_len == 0 {
            return;
        }
        self.trail_head = (self.trail_head + 1) % MAX_TRAIL;
        self.trail_fill = (self.trail_fill + 1).min(MAX_TRAIL);
        let head = self.trail_head;
        for (i, b) in self
            .boids
            .iter()
            .chain(std::iter::once(&self.hawk))
            .enumerate()
        {
            self.trail[i * MAX_TRAIL + head] = (b.x, b.y);
        }
    }

    #[inline]
    fn heading_index(vx: f32, vy: f32) -> usize {
        let a = vy.atan2(vx);
        ((a / (PI / 4.0)).round() as i32).rem_euclid(8) as usize
    }

    fn color_of(&self, b: &Boid) -> Color {
        if self.by_heading {
            let a = b.vy.atan2(b.vx).rem_euclid(TAU) / TAU;
            self.wheel[((a * HEAD_LUT as f32) as usize).min(HEAD_LUT - 1)]
        } else {
            self.flock_col[b.flock as usize & 3]
        }
    }

    /// Draws the trail of boid `i` (the hawk is index `boids.len()`).
    fn draw_trail(&mut self, ctx: &Ctx, i: usize, col: Color, bright: f32) {
        let len = self.trail_len.min(self.trail_fill);
        if len < 2 {
            return;
        }
        let base = i * MAX_TRAIL;
        let (fw, fh) = (self.fw, self.fh);
        let mut prev = self.trail[base + self.trail_head];
        for k in 1..len {
            let idx = (self.trail_head + MAX_TRAIL - k) % MAX_TRAIL;
            let p = self.trail[base + idx];
            // Do not draw across the screen where the boid wrapped around.
            if (p.0 - prev.0).abs() < fw * 0.5 && (p.1 - prev.1).abs() < fh * 0.5 {
                let v = bright * (1.0 - k as f32 / len as f32);
                if v >= FLOOR * 0.5 {
                    let c = ctx.fade(col, 1.0 - v.max(FLOOR));
                    self.br
                        .line(prev.0 * 2.0, prev.1 * 2.0, p.0 * 2.0, p.1 * 2.0, c);
                }
            }
            prev = p;
        }
    }
}

impl Scene for Boids {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        let (fw, fh) = (w.max(1) as f32, h.max(1) as f32 * 2.0);
        // Keep the flock where it was, relative to the screen.
        let (kx, ky) = (fw / self.fw, fh / self.fh);
        for b in &mut self.boids {
            b.x = (b.x * kx).clamp(0.0, fw - 0.001);
            b.y = (b.y * ky).clamp(0.0, fh - 0.001);
        }
        self.hawk.x = (self.hawk.x * kx).clamp(0.0, fw - 0.001);
        self.hawk.y = (self.hawk.y * ky).clamp(0.0, fh - 0.001);
        self.fw = fw;
        self.fh = fh;
        let cells = w.max(1) as f32 * h.max(1) as f32;
        let want = ((cells * self.density).round() as usize).clamp(2, MAX_BOIDS);
        self.flocks = ((want as f32 / 35.0).round() as u8).clamp(1, 4);
        self.boids.truncate(want);
        while self.boids.len() < want {
            let f = (self.boids.len() % self.flocks as usize) as u8;
            let b = self.random_boid(f);
            self.boids.push(b);
        }
        for (i, b) in self.boids.iter_mut().enumerate() {
            b.flock = (i % self.flocks as usize) as u8;
        }
        if self.hawk.vx == 0.0 && self.hawk.vy == 0.0 {
            self.hawk = self.random_boid(0);
        }
        self.acc.clear();
        self.acc.resize(want, (0.0, 0.0));
        self.gw = ((fw / self.vision).floor() as usize).max(1);
        self.gh = ((fh / self.vision).floor() as usize).max(1);
        self.starts.clear();
        self.starts.resize(self.gw * self.gh + 1, 0);
        self.order.clear();
        self.order.reserve(want);
        self.cell_of.clear();
        self.cell_of.reserve(want);
        // Restart trails: old positions are meaningless after a resize.
        self.trail.clear();
        self.trail.resize((want + 1) * MAX_TRAIL, (0.0, 0.0));
        self.trail_fill = 0;
        self.br.resize(w, h);
    }

    fn retheme(&mut self, ctx: &Ctx) {
        for (i, &hex) in NATIVE_FLOCKS.iter().enumerate() {
            self.flock_col[i] = ctx.pick(i, rgb(hex));
        }
        let stops: [Color; 5] = if ctx.theme.native {
            NATIVE_WHEEL.map(rgb)
        } else {
            std::array::from_fn(|i| ctx.pick(i, Color::Reset))
        };
        for (i, slot) in self.wheel.iter_mut().enumerate() {
            let p = i as f32 / HEAD_LUT as f32 * stops.len() as f32;
            let k = p as usize % stops.len();
            *slot = mix_oklab(stops[k], stops[(k + 1) % stops.len()], p.fract());
        }
        self.hawk_col = ctx.pick(4, rgb(NATIVE_HAWK));
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.1)
        } else {
            0.0
        };
        let steps = (dt / STEP).ceil().max(1.0) as usize;
        let h = dt / steps as f32;
        for _ in 0..steps {
            self.step(h);
            self.trail_clock += h;
            if self.trail_clock >= 1.0 / TRAIL_HZ {
                self.trail_clock %= 1.0 / TRAIL_HZ;
                self.record_trails();
            }
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        self.br.clear();
        let n = self.boids.len();
        if self.trail_len > 0 {
            for i in 0..n {
                let col = self.color_of(&self.boids[i]);
                let bright = if self.style == Style::Dots { 0.9 } else { 0.75 };
                self.draw_trail(ctx, i, col, bright);
            }
            if self.hawk_on {
                self.draw_trail(ctx, n, self.hawk_col, 0.8);
            }
        }
        if self.style == Style::Dots {
            for i in 0..n {
                let b = self.boids[i];
                let col = self.color_of(&b);
                let s = (b.vx * b.vx + b.vy * b.vy).sqrt().max(1e-4);
                // A two-dot head pointing where the boid flies.
                self.br.setf(b.x * 2.0, b.y * 2.0, col);
                self.br
                    .setf(b.x * 2.0 - b.vx / s, b.y * 2.0 - b.vy / s, col);
            }
        }
        self.br.blit(c);

        if self.style == Style::Arrows {
            for b in &self.boids {
                let ch = ARROWS[Self::heading_index(b.vx, b.vy)];
                c.set(b.x as i32, (b.y * 0.5) as i32, ch, self.color_of(b));
            }
        }
        if self.hawk_on {
            let h = &self.hawk;
            let ch = HAWK[Self::heading_index(h.vx, h.vy)];
            c.set_attr(h.x as i32, (h.y * 0.5) as i32, ch, self.hawk_col, BOLD);
        }
    }
}
