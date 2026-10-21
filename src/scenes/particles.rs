//! particles: thousands of particles riding a slowly changing flow field,
//! fireworks, or a fountain, all leaving fading braille trails.
//!
//! Every braille dot remembers when something last passed over it and with
//! which color, so trails fade by age without touching the whole buffer each
//! frame. The flow field is the curl of 3D simplex noise (time is the third
//! axis), sampled on a coarse grid and interpolated, so it swirls without
//! sinks and costs almost nothing per particle.

use std::f32::consts::TAU;

use crate::color::{Color, rgb};
use crate::render::{Braille, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "particles",
    aliases: &["flow"],
    about: "particles riding a slowly changing flow field",
    options: &[
        Opt {
            key: "mode",
            default: Val::S("flow"),
            help: "flow (a swirling flow field), fireworks or fountain",
        },
        Opt {
            key: "count",
            default: Val::I(400),
            help: "Particles on a 100x30 screen (scaled by the screen area)",
        },
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast particles move",
        },
        Opt {
            key: "scale",
            default: Val::F(1.0),
            help: "Flow noise scale: higher means smaller, busier swirls",
        },
        Opt {
            key: "trail",
            default: Val::F(1.0),
            help: "Trail persistence (multiplies each mode's trail length)",
        },
        Opt {
            key: "color_by",
            default: Val::S("angle"),
            help: "Flow coloring: angle (direction) or speed",
        },
        Opt {
            key: "fireworks_rate",
            default: Val::F(1.2),
            help: "Rockets per second per 100 columns (fireworks mode)",
        },
        Opt {
            key: "gravity",
            default: Val::F(1.0),
            help: "Gravity for fireworks and fountain",
        },
    ],
    make: |ctx| Box::new(Particles::new(ctx)),
};

/// Flow palette under `classic`: indigo through sky and turquoise to a warm glow.
const FLOW_PAL: [Color; 6] = [
    rgb(0x4c3fd9),
    rgb(0x2f8cff),
    rgb(0x1fd1e0),
    rgb(0x7af0c0),
    rgb(0xffd9a0),
    rgb(0xff7aa8),
];

/// Firework shell colors under `classic`.
const SHELLS: [Color; 8] = [
    rgb(0xff3b3b),
    rgb(0xff9f1c),
    rgb(0xffe14d),
    rgb(0x3ddc84),
    rgb(0x38c6ff),
    rgb(0x5b7cff),
    rgb(0xc86bff),
    rgb(0xff4fa3),
];

/// Fade steps per palette entry.
const FADE: usize = 16;
/// Gradient steps for the flow and fountain palettes.
const GRAD: usize = 64;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Flow,
    Fireworks,
    Fountain,
}

/// A numeric option clamped to its range; non-finite values get the default.
fn num(v: f32, default: f32, lo: f32, hi: f32) -> f32 {
    if v.is_finite() {
        v.clamp(lo, hi)
    } else {
        default
    }
}

// ---------------------------------------------------------------------------
// Noise

/// 3D simplex noise (after Stefan Gustavson's public-domain reference).
struct Simplex {
    perm: [u8; 512],
}

impl Simplex {
    fn new(rng: &mut fastrand::Rng) -> Simplex {
        let mut p = [0u8; 256];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i as u8;
        }
        rng.shuffle(&mut p);
        let mut perm = [0u8; 512];
        for (i, v) in perm.iter_mut().enumerate() {
            *v = p[i & 255];
        }
        Simplex { perm }
    }

    #[inline]
    fn grad(hash: u8, x: f32, y: f32, z: f32) -> f32 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };
        (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
    }

    /// Smooth noise in roughly -1..1.
    fn get(&self, x: f32, y: f32, z: f32) -> f32 {
        const F3: f32 = 1.0 / 3.0;
        const G3: f32 = 1.0 / 6.0;
        let s = (x + y + z) * F3;
        let (i, j, k) = ((x + s).floor(), (y + s).floor(), (z + s).floor());
        let t = (i + j + k) * G3;
        let (x0, y0, z0) = (x - (i - t), y - (j - t), z - (k - t));
        let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
            if y0 >= z0 {
                (1, 0, 0, 1, 1, 0)
            } else if x0 >= z0 {
                (1, 0, 0, 1, 0, 1)
            } else {
                (0, 0, 1, 1, 0, 1)
            }
        } else if y0 < z0 {
            (0, 0, 1, 0, 1, 1)
        } else if x0 < z0 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        };
        let corners = [
            (x0, y0, z0, 0, 0, 0),
            (
                x0 - i1 as f32 + G3,
                y0 - j1 as f32 + G3,
                z0 - k1 as f32 + G3,
                i1,
                j1,
                k1,
            ),
            (
                x0 - i2 as f32 + 2.0 * G3,
                y0 - j2 as f32 + 2.0 * G3,
                z0 - k2 as f32 + 2.0 * G3,
                i2,
                j2,
                k2,
            ),
            (
                x0 - 1.0 + 3.0 * G3,
                y0 - 1.0 + 3.0 * G3,
                z0 - 1.0 + 3.0 * G3,
                1,
                1,
                1,
            ),
        ];
        // Lattice coordinates wrap at 256; `rem_euclid` keeps negatives valid.
        let (ii, jj, kk) = (
            (i as i64).rem_euclid(256) as usize,
            (j as i64).rem_euclid(256) as usize,
            (k as i64).rem_euclid(256) as usize,
        );
        let p = &self.perm;
        let mut n = 0.0;
        for (cx, cy, cz, di, dj, dk) in corners {
            let t = 0.6 - cx * cx - cy * cy - cz * cz;
            if t > 0.0 {
                let h = p[ii + di + p[jj + dj + p[kk + dk] as usize] as usize];
                let t2 = t * t;
                n += t2 * t2 * Simplex::grad(h, cx, cy, cz);
            }
        }
        32.0 * n
    }
}

/// The curl of a noise stream function, sampled on a coarse grid of dots.
struct Field {
    /// Grid spacing in dots.
    g: f32,
    gw: usize,
    gh: usize,
    psi: Vec<f32>,
    vel: Vec<(f32, f32)>,
    /// Which third of the rows gets fresh noise this frame.
    phase: usize,
}

impl Field {
    fn new() -> Field {
        Field {
            g: 8.0,
            gw: 0,
            gh: 0,
            psi: Vec::new(),
            vel: Vec::new(),
            phase: 0,
        }
    }

    fn resize(&mut self, pw: usize, ph: usize, g: f32) {
        self.g = g;
        self.gw = (pw as f32 / g).ceil() as usize + 3;
        self.gh = (ph as f32 / g).ceil() as usize + 3;
        let n = self.gw * self.gh;
        self.psi.clear();
        self.psi.resize(n, 0.0);
        self.vel.clear();
        self.vel.resize(n, (0.0, 0.0));
    }

    /// Refreshes the stream function (all rows, or a third of them) and the
    /// velocities derived from it. Velocities come out in "dots per unit", so
    /// typical speeds are around 1 whatever the noise scale.
    fn update(&mut self, noise: &Simplex, f: f32, z: f32, all: bool) {
        let (gw, gh, g) = (self.gw, self.gh, self.g);
        for j in 0..gh {
            if !all && j % 3 != self.phase {
                continue;
            }
            let y = (j as f32 - 1.0) * g * f;
            for i in 0..gw {
                let x = (i as f32 - 1.0) * g * f;
                let n = noise.get(x, y, z) + 0.35 * noise.get(x * 2.3 + 17.0, y * 2.3, z * 1.7);
                self.psi[j * gw + i] = n / f;
            }
        }
        self.phase = (self.phase + 1) % 3;
        let k = 0.5 / g;
        for j in 0..gh {
            let (ju, jd) = (j.saturating_sub(1), (j + 1).min(gh - 1));
            for i in 0..gw {
                let (il, ir) = (i.saturating_sub(1), (i + 1).min(gw - 1));
                let dy = self.psi[jd * gw + i] - self.psi[ju * gw + i];
                let dx = self.psi[j * gw + ir] - self.psi[j * gw + il];
                self.vel[j * gw + i] = (dy * k, -dx * k);
            }
        }
    }

    #[inline]
    fn sample(&self, x: f32, y: f32) -> (f32, f32) {
        let fx = (x / self.g + 1.0).clamp(0.0, (self.gw - 2) as f32);
        let fy = (y / self.g + 1.0).clamp(0.0, (self.gh - 2) as f32);
        let (i, j) = (fx as usize, fy as usize);
        let (tx, ty) = (fx - i as f32, fy - j as f32);
        let r0 = j * self.gw + i;
        let r1 = r0 + self.gw;
        let (a, b, c, d) = (
            self.vel[r0],
            self.vel[r0 + 1],
            self.vel[r1],
            self.vel[r1 + 1],
        );
        let top = (a.0 + (b.0 - a.0) * tx, a.1 + (b.1 - a.1) * tx);
        let bot = (c.0 + (d.0 - c.0) * tx, c.1 + (d.1 - c.1) * tx);
        (top.0 + (bot.0 - top.0) * ty, top.1 + (bot.1 - top.1) * ty)
    }
}

// ---------------------------------------------------------------------------
// Trails

/// Per-dot "last lit" time and color: the trail buffer. Dots are stored cell
/// by cell (8 per braille cell, in `k = row * 2 + column` order), and every
/// cell remembers its newest dot so idle cells are skipped at a glance.
struct Dots {
    pw: usize,
    ph: usize,
    cw: usize,
    stamp: Vec<u32>,
    col: Vec<u8>,
    cell: Vec<u32>,
}

impl Dots {
    fn resize(&mut self, cols: usize, rows: usize) {
        self.pw = cols * 2;
        self.ph = rows * 4;
        self.cw = cols;
        let n = cols * rows;
        self.stamp.clear();
        self.stamp.resize(n * 8, 0);
        self.col.clear();
        self.col.resize(n * 8, 0);
        self.cell.clear();
        self.cell.resize(n, 0);
    }

    /// Lights a dot at time `t` (an older `t` draws a fainter dot), unless
    /// something brighter passed there more recently.
    #[inline]
    fn put(&mut self, x: f32, y: f32, t: u32, c: u8) {
        if !(x >= 0.0 && y >= 0.0) {
            return;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.pw || y >= self.ph {
            return;
        }
        let ci = (y >> 2) * self.cw + (x >> 1);
        let i = ci * 8 + (y & 3) * 2 + (x & 1);
        if t.wrapping_sub(self.stamp[i]) as i32 >= 0 {
            self.stamp[i] = t;
            self.col[i] = c;
            if t.wrapping_sub(self.cell[ci]) as i32 > 0 {
                self.cell[ci] = t;
            }
        }
    }

    /// Lights every dot from (x0, y0) (exclusive) to (x1, y1).
    #[inline]
    fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, t: u32, c: u8) {
        let (dx, dy) = (x1 - x0, y1 - y0);
        let n = dx.abs().max(dy.abs()).ceil().clamp(1.0, 64.0) as usize;
        let inv = 1.0 / n as f32;
        for k in 1..=n {
            let s = k as f32 * inv;
            self.put(x0 + dx * s, y0 + dy * s, t, c);
        }
    }
}

// ---------------------------------------------------------------------------
// The scene

#[derive(Clone, Copy)]
struct Mote {
    x: f32,
    y: f32,
    age: f32,
    life: f32,
}

#[derive(Clone, Copy)]
struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
    drag: f32,
    grav: f32,
    col: u8,
    /// Fireworks: flickers near the end. Fountain: has bounced once.
    flag: bool,
}

#[derive(Clone, Copy)]
struct Rocket {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    kind: u8,
    col: u8,
    col2: u8,
}

struct Particles {
    mode: Mode,
    by_speed: bool,
    count_opt: f32,
    speed: f32,
    scale: f32,
    trail: f32,
    rate: f32,
    gravity: f32,
    rng: fastrand::Rng,
    noise: Simplex,
    w: usize,
    h: usize,
    braille: Braille,
    dots: Dots,
    /// Milliseconds since start, offset so that zeroed stamps count as ancient.
    now: u32,
    t: f32,
    max_age: u32,
    field: Field,
    /// Noise frequency (per dot).
    freq: f32,
    motes: Vec<Mote>,
    sparks: Vec<Spark>,
    rockets: Vec<Rocket>,
    launch_in: f32,
    jets: usize,
    emit_acc: f32,
    /// Palette entry × fade step → color, built in `retheme`.
    lut: Vec<Color>,
    shells: usize,
}

impl Particles {
    fn new(ctx: &Ctx) -> Particles {
        let o = ctx.opts;
        let mode = match o.s("mode").to_ascii_lowercase().as_str() {
            "fireworks" | "firework" | "fw" => Mode::Fireworks,
            "fountain" | "fountains" => Mode::Fountain,
            _ => Mode::Flow,
        };
        let mut rng = ctx.rng();
        let noise = Simplex::new(&mut rng);
        Particles {
            mode,
            by_speed: o.s("color_by").eq_ignore_ascii_case("speed"),
            count_opt: (o.i("count").clamp(0, 100_000)) as f32,
            speed: num(o.f("speed"), 1.0, 0.05, 10.0),
            scale: num(o.f("scale"), 1.0, 0.1, 10.0),
            trail: num(o.f("trail"), 1.0, 0.05, 10.0),
            rate: num(o.f("fireworks_rate"), 1.2, 0.0, 20.0),
            gravity: num(o.f("gravity"), 1.0, 0.0, 10.0),
            rng,
            noise,
            w: 0,
            h: 0,
            braille: Braille::new(1, 1),
            dots: Dots {
                pw: 0,
                ph: 0,
                cw: 0,
                stamp: Vec::new(),
                col: Vec::new(),
                cell: Vec::new(),
            },
            now: 1 << 24,
            t: 0.0,
            max_age: 1000,
            field: Field::new(),
            freq: 0.01,
            motes: Vec::new(),
            sparks: Vec::new(),
            rockets: Vec::new(),
            launch_in: 0.3,
            jets: 1,
            emit_acc: 0.0,
            lut: Vec::new(),
            shells: SHELLS.len(),
        }
    }

    fn pw(&self) -> f32 {
        self.dots.pw as f32
    }

    fn ph(&self) -> f32 {
        self.dots.ph as f32
    }

    /// How many particles this screen gets: `count` per 100×30 cells (capped).
    fn target(&self) -> usize {
        ((self.count_opt * (self.w * self.h) as f32 / 3000.0).round() as usize).min(200_000)
    }

    fn spawn_mote(&mut self) -> Mote {
        Mote {
            x: self.rng.f32() * self.pw(),
            y: self.rng.f32() * self.ph(),
            age: 0.0,
            life: 2.5 + self.rng.f32() * 5.0,
        }
    }

    /// The color slot for a gradient position.
    fn grad_slot(t: f32) -> u8 {
        (t.clamp(0.0, 1.0) * (GRAD - 1) as f32).round() as u8
    }

    fn update_flow(&mut self, dt: f32) {
        let z = self.t * 0.045 * self.speed.sqrt();
        self.field.update(&self.noise, self.freq, z, false);
        let v = 13.0 * self.speed;
        let (pw, ph) = (self.pw(), self.ph());
        let span = self.max_age as f32;
        // The direction that maps to the top of the ramp turns slowly.
        let (rc, rs) = ((self.t * 0.05).cos(), (self.t * 0.05).sin());
        for i in 0..self.motes.len() {
            let mut m = self.motes[i];
            m.age += dt;
            let (vx, vy) = self.field.sample(m.x, m.y);
            let (nx, ny) = (m.x + vx * v * dt, m.y + vy * v * dt);
            if m.age >= m.life || !(0.0..pw).contains(&nx) || !(0.0..ph).contains(&ny) {
                self.motes[i] = self.spawn_mote();
                continue;
            }
            let sp = (vx * vx + vy * vy).sqrt();
            let tcol = if self.by_speed {
                (sp / 3.2).min(1.0)
            } else if sp > 1e-6 {
                0.5 + 0.5 * (vx * rc + vy * rs) / sp
            } else {
                0.5
            };
            // Fade in and out over a life: stamp "older" so it looks fainter.
            let env = (m.age / 0.6).min(1.0).min((m.life - m.age) / 0.8).max(0.0);
            let stamp = self.now.wrapping_sub(((1.0 - env) * span * 0.7) as u32);
            self.dots
                .line(m.x, m.y, nx, ny, stamp, Particles::grad_slot(tcol));
            m.x = nx;
            m.y = ny;
            self.motes[i] = m;
        }
    }

    fn launch(&mut self) {
        let (pw, ph) = (self.pw(), self.ph());
        let g = ph * 0.55 * self.gravity.max(0.05);
        let height = ph * (0.45 + self.rng.f32() * 0.35);
        let shells = self.shells.max(1) as u8;
        let col = self.rng.u8(0..shells);
        let col2 = (col + 1 + self.rng.u8(0..shells.max(2) - 1)) % shells;
        let x = pw * (0.12 + 0.76 * self.rng.f32());
        self.rockets.push(Rocket {
            x,
            y: ph - 1.0,
            vx: (pw * 0.5 - x) * 0.08 * self.rng.f32() + (self.rng.f32() - 0.5) * 6.0,
            vy: -(2.0 * g * height).sqrt(),
            kind: self.rng.u8(0..5),
            col,
            col2,
        });
    }

    fn burst(&mut self, r: Rocket) {
        let (pw, ph) = (self.pw(), self.ph());
        let room = self.sparks.capacity() - self.sparks.len();
        let size = (self.w * self.h) as f32 / 3000.0;
        let n = ((60.0 + self.rng.f32() * 60.0) * size.sqrt().clamp(0.3, 3.0)) as usize;
        let n = n.min(room);
        let radius = ph.min(pw * 0.6) * (0.2 + self.rng.f32() * 0.12);
        let g = ph * 0.55 * self.gravity;
        let white = self.shells as u8;
        for k in 0..n {
            let a = self.rng.f32() * TAU;
            let (drag, life, grav, mut speed, col) = match r.kind {
                // Ring: every star at the same speed.
                1 => (1.9, 1.1 + self.rng.f32() * 0.4, 0.3, 1.0, r.col),
                // Willow: golden, slow to stop, long and drooping.
                2 => (1.1, 1.9 + self.rng.f32() * 0.9, 0.45, 1.0, white + 1),
                // Two-tone peony.
                3 => (
                    1.8,
                    1.0 + self.rng.f32() * 0.6,
                    0.3,
                    1.0,
                    if k % 2 == 0 { r.col } else { r.col2 },
                ),
                // Peony and crackle.
                _ => (1.8, 1.0 + self.rng.f32() * 0.7, 0.3, 1.0, r.col),
            };
            if r.kind != 1 {
                // A sphere of stars seen from the side: denser toward the rim.
                let u = self.rng.f32() * 2.0 - 1.0;
                speed = (1.0 - u * u).sqrt() * (0.85 + 0.15 * self.rng.f32());
            }
            let v = radius * drag * speed;
            self.sparks.push(Spark {
                x: r.x,
                y: r.y,
                vx: a.cos() * v + r.vx * 0.3,
                vy: a.sin() * v + r.vy * 0.2,
                age: 0.0,
                life,
                drag,
                grav: g * grav,
                col,
                flag: r.kind == 4,
            });
        }
    }

    fn update_fireworks(&mut self, dt: f32) {
        let (pw, ph) = (self.pw(), self.ph());
        let span = self.max_age as f32;
        self.launch_in -= dt;
        if self.launch_in <= 0.0 {
            let per_s = self.rate * (self.w as f32 / 100.0).max(0.3);
            if per_s > 0.0 {
                // Poisson-ish gaps, never too regular.
                self.launch_in = -(1.0 - self.rng.f32()).max(1e-3).ln() / per_s;
                self.launch_in = self.launch_in.clamp(0.08, (2.5 / per_s).max(0.08));
                if self.rockets.len() < self.rockets.capacity() {
                    self.launch();
                }
            } else {
                self.launch_in = 1.0;
            }
        }
        let g = ph * 0.55 * self.gravity.max(0.05);
        let trail = self.shells as u8 + 1;
        let mut i = 0;
        while i < self.rockets.len() {
            let mut r = self.rockets[i];
            let (ox, oy) = (r.x, r.y);
            r.vy += g * dt;
            r.x += r.vx * dt;
            r.y += r.vy * dt;
            let wobble = (self.rng.f32() - 0.5) * 0.8;
            self.dots.line(
                ox,
                oy,
                r.x + wobble,
                r.y,
                self.now.wrapping_sub(span as u32 / 3),
                trail,
            );
            // Burst at the apex (or near the top); anything odd just bursts too.
            if !(r.vy < -g * 0.12 && r.y >= ph * 0.08) {
                self.rockets.swap_remove(i);
                self.burst(r);
            } else {
                self.rockets[i] = r;
                i += 1;
            }
        }
        let white = self.shells as u8;
        let mut i = 0;
        while i < self.sparks.len() {
            let mut s = self.sparks[i];
            s.age += dt;
            if s.age >= s.life || s.y >= ph || s.x < -pw * 0.2 || s.x > pw * 1.2 {
                self.sparks.swap_remove(i);
                continue;
            }
            let (ox, oy) = (s.x, s.y);
            let k = (-s.drag * dt).exp();
            s.vx *= k;
            s.vy = s.vy * k + s.grav * dt;
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            let f = s.age / s.life;
            let dim = (f * f * span * 0.85) as u32;
            if s.flag && f > 0.55 {
                // Crackle: bright white flickers.
                if self.rng.f32() < 0.35 {
                    self.dots.put(s.x, s.y, self.now, white);
                }
            } else {
                self.dots
                    .line(ox, oy, s.x, s.y, self.now.wrapping_sub(dim), s.col);
            }
            self.sparks[i] = s;
            i += 1;
        }
    }

    fn update_fountain(&mut self, dt: f32) {
        let (pw, ph) = (self.pw(), self.ph());
        let g = ph * 1.1 * self.gravity.max(0.05);
        let target = (self.target() / 2).max(1);
        // Each drop lives about one flight; emit enough to keep `target` aloft.
        let flight = 2.0 * (2.0 * 0.75 / 1.1f32 / self.gravity.max(0.05)).sqrt();
        self.emit_acc += dt * target as f32 / flight * self.speed.sqrt();
        let span = self.max_age as f32;
        while self.emit_acc >= 1.0 {
            self.emit_acc -= 1.0;
            if self.sparks.len() >= self.sparks.capacity() {
                continue;
            }
            let jet = self.rng.usize(0..self.jets);
            let x = pw * (jet as f32 + 0.5) / self.jets as f32;
            let height = ph * (0.55 + 0.3 * self.rng.f32());
            let v = (2.0 * g * height).sqrt();
            let a = (self.rng.f32() - 0.5) * 0.45 + (self.t * 0.7 + jet as f32).sin() * 0.06;
            self.sparks.push(Spark {
                x: x + (self.rng.f32() - 0.5) * 2.0,
                y: ph - 0.5,
                vx: a.sin() * v,
                vy: -a.cos() * v,
                age: 0.0,
                life: 10.0,
                drag: 0.0,
                grav: g,
                col: 0,
                flag: false,
            });
        }
        let v0 = (2.0 * g * ph * 0.85).sqrt();
        let mut i = 0;
        while i < self.sparks.len() {
            let mut s = self.sparks[i];
            s.age += dt;
            let (ox, oy) = (s.x, s.y);
            s.vy += s.grav * dt;
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            if s.y >= ph - 0.5 && s.vy > 0.0 {
                if s.flag {
                    self.sparks.swap_remove(i);
                    continue;
                }
                // One small splash.
                s.flag = true;
                s.y = ph - 0.5;
                s.vy *= -0.22;
                s.vx = s.vx * 0.5 + (self.rng.f32() - 0.5) * v0 * 0.15;
            }
            if s.x < -2.0 || s.x > pw + 2.0 || s.age > s.life {
                self.sparks.swap_remove(i);
                continue;
            }
            // Fast drops at the top of the ramp, slow ones at the apex lower.
            let sp = (s.vx * s.vx + s.vy * s.vy).sqrt() / v0;
            let dim = if s.flag { (span * 0.5) as u32 } else { 0 };
            self.dots.line(
                ox,
                oy,
                s.x,
                s.y,
                self.now.wrapping_sub(dim),
                Particles::grad_slot(0.15 + 0.85 * sp),
            );
            self.sparks[i] = s;
            i += 1;
        }
    }
}

impl Scene for Particles {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        let (w, h) = (w.max(1) as usize, h.max(1) as usize);
        self.w = w;
        self.h = h;
        self.braille.resize(w as u16, h as u16);
        self.dots.resize(w, h);
        let (pw, ph) = (self.dots.pw, self.dots.ph);
        // Swirls about 110 dots across at scale 1.
        self.freq = self.scale / 110.0;
        let g = (1.0 / self.freq / 12.0).clamp(3.0, 16.0);
        self.field.resize(pw, ph, g);
        self.field.update(
            &self.noise,
            self.freq,
            self.t * 0.045 * self.speed.sqrt(),
            true,
        );
        let base = match self.mode {
            Mode::Flow => 0.9,
            Mode::Fireworks => 0.38,
            Mode::Fountain => 0.3,
        };
        self.max_age = ((base * self.trail * 1000.0) as u32).max(30);
        let target = self.target();
        self.motes.clear();
        self.sparks.clear();
        self.rockets.clear();
        match self.mode {
            Mode::Flow => {
                self.motes.reserve_exact(target);
                for _ in 0..target {
                    let mut m = self.spawn_mote();
                    // Stagger lives so they do not all respawn together.
                    m.age = self.rng.f32() * m.life;
                    self.motes.push(m);
                }
            }
            Mode::Fireworks => {
                self.sparks.reserve_exact((target * 3).max(200));
                self.rockets.reserve_exact((w / 8).clamp(4, 64));
            }
            Mode::Fountain => {
                self.sparks.reserve_exact((target / 2).max(8));
                self.jets = (w / 60).clamp(1, 5);
            }
        }
    }

    fn retheme(&mut self, ctx: &Ctx) {
        // Without a known background fading can only darken, and near-black
        // dots would show as specks on a lighter or transparent terminal.
        let reach = if matches!(ctx.bg, Color::Rgb(..)) {
            0.92
        } else {
            0.6
        };
        let fade = |c: Color, l: usize| {
            let k = l as f32 / (FADE - 1) as f32;
            ctx.fade(c, k.powf(1.3) * reach)
        };
        self.lut.clear();
        match self.mode {
            Mode::Flow | Mode::Fountain => {
                let g = ctx.ramp(&FLOW_PAL);
                // Theme ramps start near the background; keep particles visible.
                let lo = if ctx.theme.native { 0.0 } else { 0.3 };
                for i in 0..GRAD {
                    let c = g.at(lo + (1.0 - lo) * i as f32 / (GRAD - 1) as f32);
                    for l in 0..FADE {
                        self.lut.push(fade(c, l));
                    }
                }
            }
            Mode::Fireworks => {
                let n = if ctx.theme.native {
                    SHELLS.len()
                } else {
                    ctx.theme.colors.len().clamp(1, 12)
                };
                self.shells = n;
                let hot = if ctx.theme.native {
                    rgb(0xffffff)
                } else {
                    ctx.ramp(&[]).at(1.0)
                };
                let gold = ctx.pick(4, rgb(0xffc35a));
                for i in 0..n + 2 {
                    let c = match i {
                        _ if i == n => hot,
                        _ if i == n + 1 => gold,
                        _ => ctx.pick(i, SHELLS[i % SHELLS.len()]),
                    };
                    for l in 0..FADE {
                        // The freshest dot of each star glows hot.
                        let c = if l == 0 { c.lerp(hot, 0.45) } else { c };
                        self.lut.push(fade(c, l));
                    }
                }
            }
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.1)
        } else {
            0.0
        };
        self.t += dt;
        self.now = self.now.wrapping_add((dt * 1000.0).round() as u32);
        match self.mode {
            Mode::Flow => self.update_flow(dt),
            Mode::Fireworks => self.update_fireworks(dt),
            Mode::Fountain => self.update_fountain(dt),
        }
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        self.braille.clear();
        let (now, max_age) = (self.now, self.max_age);
        let palette_len = self.lut.len() / FADE;
        if palette_len == 0 {
            return;
        }
        // Per-dot lifetimes between 55% and 100% of the trail length.
        let mut thr = [0u32; 32];
        for (h, t) in thr.iter_mut().enumerate() {
            *t = (max_age as u64 * (55 + h as u64 * 45 / 31) / 100) as u32;
        }
        // age × fade_mul >> 32 = fade step.
        let fade_mul = ((FADE as u64) << 32) / max_age.max(1) as u64;
        let dots = &self.dots;
        for cy in 0..self.h {
            for cx in 0..self.w {
                let ci = cy * dots.cw + cx;
                if now.wrapping_sub(dots.cell[ci]) >= max_age {
                    continue;
                }
                // Branch-free: which dots are lit, and the freshest one's age
                // and color packed into one key.
                let stamps = &dots.stamp[ci * 8..ci * 8 + 8];
                let cols = &dots.col[ci * 8..ci * 8 + 8];
                let mut mask = 0u32;
                let mut key = u32::MAX;
                for k in 0..8 {
                    let age = now.wrapping_sub(stamps[k]);
                    // Each dot has its own lifetime, so tails dissolve rather
                    // than being cut off.
                    let h = ((ci * 8 + k) as u32).wrapping_mul(2_654_435_761) >> 27;
                    let on = age < thr[(h & 31) as usize];
                    mask |= (on as u32) << k;
                    let kk = (age.min(0xff_ffff) << 8) | cols[k] as u32;
                    key = key.min(if on { kk } else { u32::MAX });
                }
                if mask == 0 {
                    continue;
                }
                let level = (((key >> 8) as u64 * fade_mul) >> 32).min(FADE as u64 - 1);
                let col = ((key & 0xff) as usize).min(palette_len - 1);
                let color = self.lut[col * FADE + level as usize];
                while mask != 0 {
                    let k = mask.trailing_zeros() as i32;
                    mask &= mask - 1;
                    self.braille
                        .set(cx as i32 * 2 + (k & 1), cy as i32 * 4 + (k >> 1), color);
                }
            }
        }
        self.braille.blit(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::Theme;

    /// Every mode and odd option survives tiny and huge screens and huge steps.
    #[test]
    fn variants_survive() {
        let variants = [
            "mode = 'flow'\ncolor_by = 'speed'",
            "mode = 'fireworks'\nfireworks_rate = 20.0",
            "mode = 'fountain'\ngravity = 0.0",
            "mode = 'fireworks'\ngravity = 10.0\ntrail = 10.0\ncount = 5000",
            "count = 0\nspeed = 10.0\nscale = 10.0",
            "mode = 'fireworks'\nspeed = nan\ngravity = nan\nfireworks_rate = inf\ntrail = -1.0",
            "mode = 'fountain'\ncount = -10\nscale = nan",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                let opts = Opts::new(INFO.options, v.parse().unwrap());
                let ctx = Ctx {
                    theme: &theme,
                    opts: &opts,
                    bg: theme.bg,
                    seed: 9,
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
                    for f in 0..60 {
                        s.update(&ctx, if f % 13 == 12 { 5.0 } else { 1.0 / 30.0 });
                        c.clear();
                        s.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }

    #[test]
    fn noise_is_smooth_and_bounded() {
        let mut rng = fastrand::Rng::with_seed(1);
        let n = Simplex::new(&mut rng);
        let mut prev = n.get(-300.0, 157.5, 0.25);
        for i in 1..20_000 {
            let x = -300.0 + i as f32 * 0.03;
            let v = n.get(x, 7.5 - x * 0.5, 0.25);
            assert!(v.abs() <= 1.2, "{v}");
            assert!((v - prev).abs() < 0.2, "jump at x={x}: {prev} -> {v}");
            prev = v;
        }
    }
}
