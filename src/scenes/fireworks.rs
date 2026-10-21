//! fireworks: rockets burst into colorful sparks over a city skyline.
//!
//! Adapted from tanim's `fireworks`
//! <https://github.com/SantiagoSchez/tanim>, licensed MIT. The shell patterns
//! (sphere, ring, willow, crackle, multicolor, palm), the physics, the glow a
//! burst casts on the rooftops, the lit windows, the blinking beacons and the
//! calm-then-finale launch schedule follow tanim; the night sky is transparent
//! here by default (`sky = true` paints tanim's gradient) and the colors come
//! from the theme.
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

use std::f32::consts::TAU;

use crate::color::{Color, ansi_rgb, rgb};
use crate::render::{Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "fireworks",
    aliases: &["rockets"],
    about: "rockets bursting into colorful sparks over a city, after tanim",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast everything moves (0.1-4)",
        },
        Opt {
            key: "rate",
            default: Val::F(1.0),
            help: "How often rockets launch, 0 = never (0-5)",
        },
        Opt {
            key: "skyline",
            default: Val::B(true),
            help: "Draw the city with its lit windows",
        },
        Opt {
            key: "stars",
            default: Val::B(true),
            help: "Faint stars in the night sky",
        },
        Opt {
            key: "sky",
            default: Val::B(false),
            help: "Paint the night sky instead of leaving it transparent",
        },
    ],
    make: |ctx| Box::new(Fireworks::new(ctx)),
};

/// Color as red, green, blue in 0..1 (more than 1 is allowed while adding light).
type V3 = [f32; 3];

const SKY_TOP: V3 = [1.0 / 255.0, 1.0 / 255.0, 8.0 / 255.0];
const SKY_LOW: V3 = [12.0 / 255.0, 8.0 / 255.0, 28.0 / 255.0];
const BUILDING: V3 = [5.0 / 255.0, 5.0 / 255.0, 11.0 / 255.0];
const STAR: V3 = [190.0 / 255.0, 200.0 / 255.0, 240.0 / 255.0];
const BEACON: V3 = [200.0 / 255.0, 30.0 / 255.0, 20.0 / 255.0];
const EMBER: V3 = [110.0 / 255.0, 25.0 / 255.0, 10.0 / 255.0];
const GOLD: V3 = [1.0, 190.0 / 255.0, 90.0 / 255.0];
const SHELL_WHITE: V3 = [235.0 / 255.0, 235.0 / 255.0, 1.0];
const ROCKET_FIRE: V3 = [1.0, 220.0 / 255.0, 160.0 / 255.0];
const TRAIL_FIRE: V3 = [1.0, 170.0 / 255.0, 80.0 / 255.0];
const ROCKET_G: f32 = 0.03;
const SPARK_G: f32 = 0.011;
const MAX_PARTICLES: usize = 7000;
/// tanim's tick rate; one simulation step is this long.
const STEP: f32 = 1.0 / 30.0;

const TRAIL: u8 = 1;
const CRACKLE: u8 = 2;
const STROBE: u8 = 4;
const ROCKET: u8 = 8;

#[derive(Clone, Copy)]
enum Shell {
    Sphere,
    Ring,
    Willow,
    Crackle,
    Multi,
    Palm,
}

#[derive(Clone, Copy)]
struct P {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max: f32,
    color: V3,
    drag: f32,
    grav: f32,
    flags: u8,
    shell: Shell,
}

/// The fading glow a burst casts on the rooftops (and the sky when painted).
struct Flash {
    x: f32,
    y: f32,
    radius: f32,
    color: V3,
    power: f32,
}

/// The colors the scene draws with, from the theme.
struct Palette {
    native: bool,
    /// Burst colors besides gold and white (never empty).
    accents: Vec<V3>,
    gold: V3,
    white: V3,
    sky_top: V3,
    sky_low: V3,
    building: V3,
    star: V3,
    windows: [V3; 3],
}

impl Palette {
    /// A shell color, tanim's mix: gold, white, or a random hue.
    fn shell_color(&self, rng: &mut fastrand::Rng) -> V3 {
        match rng.u32(0..8) {
            0 => self.gold,
            1 => self.white,
            _ if self.native => hsv(rng.f32(), 0.55 + 0.35 * rng.f32(), 1.0),
            _ => self.accents[rng.usize(0..self.accents.len())],
        }
    }
}

fn v3(c: Color) -> V3 {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => ansi_rgb(n),
        Color::Reset => (255, 255, 255),
    };
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
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

fn to_color(c: V3) -> Color {
    let b = |x: f32| (x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
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

struct Fireworks {
    rng: fastrand::Rng,
    speed: f32,
    rate: f32,
    show_city: bool,
    show_stars: bool,
    paint_sky: bool,
    pal: Palette,
    /// Pixel size of the screen (width, twice the rows).
    w: usize,
    ph: usize,
    ps: Vec<P>,
    /// Sparks born during a step, merged afterwards (kept to avoid allocating).
    born: Vec<P>,
    flashes: Vec<Flash>,
    /// Painted sky gradient with its stars, only when `sky = true`.
    sky: Vec<V3>,
    /// Faint stars: pixel index and brightness.
    stars: Vec<(usize, f32)>,
    /// Top pixel row of the skyline in each column.
    skyline: Vec<i32>,
    /// Brightness factor of the building in each column.
    shade: Vec<f32>,
    /// Window pixels: index, color kind and whether the light is on.
    windows: Vec<(usize, u8, bool)>,
    /// Blinking lights on top of antennas: index and phase.
    beacons: Vec<(usize, u32)>,
    light: Vec<V3>,
    glow: Vec<V3>,
    px: Pixels,
    acc: f32,
    frame: u32,
    timer: i32,
    finale: i32,
    until_finale: i32,
}

impl Fireworks {
    fn new(ctx: &Ctx) -> Fireworks {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let until_finale = rng.i32(500..1100);
        Fireworks {
            rng,
            speed: num(o.f("speed"), 1.0, 0.1, 4.0),
            rate: num(o.f("rate"), 1.0, 0.0, 5.0),
            show_city: o.b("skyline"),
            show_stars: o.b("stars"),
            paint_sky: o.b("sky"),
            pal: Palette {
                native: true,
                accents: vec![GOLD],
                gold: GOLD,
                white: SHELL_WHITE,
                sky_top: SKY_TOP,
                sky_low: SKY_LOW,
                building: BUILDING,
                star: STAR,
                windows: [[0.0; 3]; 3],
            },
            w: 0,
            ph: 0,
            ps: Vec::with_capacity(2048),
            born: Vec::with_capacity(256),
            flashes: Vec::new(),
            sky: Vec::new(),
            stars: Vec::new(),
            skyline: Vec::new(),
            shade: Vec::new(),
            windows: Vec::new(),
            beacons: Vec::new(),
            light: Vec::new(),
            glow: Vec::new(),
            px: Pixels::new(1, 1),
            acc: 0.0,
            frame: 0,
            timer: 5,
            finale: 0,
            until_finale,
        }
    }

    fn launch(&mut self) {
        let (w, hh) = (self.w as f32, self.ph as f32);
        let rng = &mut self.rng;
        let x = w * rangef(rng, 0.1, 0.9);
        let apex = rangef(rng, 0.12, 0.5) * hh;
        let vy = -(2.0 * ROCKET_G * (hh - apex).max(1.0)).sqrt();
        let shell = match rng.u32(0..6) {
            0 => Shell::Sphere,
            1 => Shell::Ring,
            2 => Shell::Willow,
            3 => Shell::Crackle,
            4 => Shell::Multi,
            _ => Shell::Palm,
        };
        let color = self.pal.shell_color(rng);
        let vx = rangef(rng, -0.15, 0.15);
        self.born.push(P {
            x,
            y: hh,
            vx,
            vy,
            life: 1000.0,
            max: 1000.0,
            color,
            drag: 1.0,
            grav: ROCKET_G,
            flags: ROCKET,
            shell,
        });
    }

    /// Spawns a shell's sparks into `born`.
    fn burst(&mut self, r: &P) {
        let size = (self.w as f32).min(self.ph as f32);
        let rng = &mut self.rng;
        let radius = size * rangef(rng, 0.18, 0.33);
        let base = (radius * 4.0).clamp(16.0, 220.0) as usize;
        self.flashes.push(Flash {
            x: r.x,
            y: r.y,
            radius: radius * 1.1 + 2.0,
            color: r.color,
            power: 0.2,
        });
        let (x, y) = (r.x, r.y);
        let mut room = MAX_PARTICLES.saturating_sub(self.ps.len() + self.born.len());
        let out = &mut self.born;
        let mut spark = |vx: f32, vy: f32, life: f32, color: V3, drag: f32, flags: u8| {
            if room > 0 {
                room -= 1;
                out.push(P {
                    x,
                    y,
                    vx,
                    vy,
                    life,
                    max: life,
                    color,
                    drag,
                    grav: SPARK_G,
                    flags,
                    shell: Shell::Sphere,
                });
            }
        };
        let pal = &self.pal;
        match r.shell {
            Shell::Sphere | Shell::Multi | Shell::Crackle => {
                let drag = 0.95;
                let v0 = radius * (1.0 - drag);
                let flags = if matches!(r.shell, Shell::Crackle) {
                    CRACKLE | STROBE
                } else {
                    0
                };
                for _ in 0..base {
                    // Project a 3D sphere: denser towards the rim.
                    let a = rng.f32() * TAU;
                    let u = rangef(rng, -1.0, 1.0);
                    let s = v0 * (1.0 - u * u).sqrt() * rangef(rng, 0.9, 1.0);
                    let col = if matches!(r.shell, Shell::Multi) {
                        pal.shell_color(rng)
                    } else {
                        r.color
                    };
                    let life = rangef(rng, 40.0, 65.0);
                    spark(a.cos() * s, a.sin() * s, life, col, drag, flags);
                }
            }
            Shell::Ring => {
                let drag = 0.95;
                let v0 = radius * (1.0 - drag);
                let tilt = rangef(rng, 0.25, 1.0);
                let rot = rng.f32() * TAU;
                let (sr, cr) = rot.sin_cos();
                let second = pal.shell_color(rng);
                let n = base.max(16);
                for k in 0..n {
                    let a = k as f32 / n as f32 * TAU;
                    let (ex, ey) = (a.cos() * v0, a.sin() * v0 * tilt);
                    let (vx, vy) = (ex * cr - ey * sr, ex * sr + ey * cr);
                    let life = rangef(rng, 45.0, 60.0);
                    spark(vx, vy, life, r.color, drag, 0);
                    if k % 3 == 0 {
                        let life = rangef(rng, 35.0, 50.0);
                        spark(vx * 0.45, vy * 0.45, life, second, drag, 0);
                    }
                }
            }
            Shell::Willow => {
                let drag = 0.965;
                let v0 = radius * (1.0 - drag) * 0.9;
                for _ in 0..base {
                    let a = rng.f32() * TAU;
                    let s = v0 * rangef(rng, 0.3, 1.0).sqrt();
                    let life = rangef(rng, 90.0, 140.0);
                    spark(a.cos() * s, a.sin() * s, life, pal.gold, drag, TRAIL);
                }
            }
            Shell::Palm => {
                let drag = 0.955;
                let v0 = radius * (1.0 - drag) * 1.1;
                let n = rng.usize(7..12);
                for k in 0..n {
                    let a = k as f32 / n as f32 * TAU + rangef(rng, -0.1, 0.1);
                    let life = rangef(rng, 60.0, 80.0);
                    spark(a.cos() * v0, a.sin() * v0 - 0.2, life, r.color, drag, TRAIL);
                }
            }
        }
    }

    /// Lays out the city and the stars for the current size.
    fn build_city(&mut self) {
        let (w, ph) = (self.w, self.ph);
        self.stars.clear();
        self.windows.clear();
        self.beacons.clear();
        self.skyline.clear();
        self.skyline.resize(w, ph as i32);
        self.shade.clear();
        self.shade.resize(w, 1.0);
        if w == 0 || ph == 0 {
            return;
        }
        let rng = &mut self.rng;
        if self.show_stars {
            for _ in 0..w * ph / 150 {
                let i = rng.usize(0..(w * ph * 6 / 10).max(1));
                self.stars.push((i, rangef(rng, 0.05, 0.2)));
            }
        }
        let max_h = (ph / 6) as i32;
        if !self.show_city || max_h < 2 {
            return;
        }
        let mut x = 0;
        while x < w {
            let bw = rng.usize(3..9);
            let bh = rng.i32(2..max_h + 1);
            let top = ph as i32 - bh;
            let shade = rangef(rng, 0.7, 1.4);
            let x1 = (x + bw).min(w);
            for xx in x..x1 {
                self.skyline[xx] = top;
                self.shade[xx] = shade;
            }
            // Windows on a grid, leaving a margin around the edges.
            let kind = rng.u8(0..3);
            for yy in (top + 2..ph as i32).step_by(2) {
                for xx in (x + 1..x1.saturating_sub(1)).step_by(2) {
                    self.windows
                        .push((yy as usize * w + xx, kind, rng.f32() < 0.3));
                }
            }
            // Tall buildings get an antenna with a blinking light.
            if bh > max_h * 3 / 4 && bw >= 5 && rng.f32() < 0.35 {
                let ax = x + bw / 2;
                let tall = rng.i32(2..5);
                if ax < w && top - tall > 0 {
                    self.skyline[ax] = top - tall;
                    self.shade[ax] = shade;
                    self.beacons
                        .push((((top - tall) as usize) * w + ax, rng.u32(0..90)));
                }
            }
            x += bw + rng.usize(0..2);
        }
    }

    /// Fills the painted sky (gradient plus stars) when `sky = true`.
    fn build_sky(&mut self) {
        self.sky.clear();
        if !self.paint_sky {
            return;
        }
        let (w, ph) = (self.w, self.ph);
        let (top, low) = (self.pal.sky_top, self.pal.sky_low);
        let denom = ph.max(2) as f32;
        self.sky
            .extend((0..w * ph).map(|i| mix(top, low, (i / w.max(1)) as f32 / denom)));
        for &(i, a) in &self.stars {
            if let Some(p) = self.sky.get_mut(i) {
                *p = mix(*p, self.pal.star, a);
            }
        }
    }

    /// Add light to a single pixel.
    #[inline]
    fn plot(&mut self, x: i32, y: i32, c: V3, a: f32) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.ph {
            let p = &mut self.light[y as usize * self.w + x as usize];
            for (d, s) in p.iter_mut().zip(c) {
                *d += s * a;
            }
        }
    }

    /// Spread light at a sub-pixel position over the four nearest pixels.
    #[inline]
    fn splat(&mut self, x: f32, y: f32, c: V3, a: f32) {
        let (fx, fy) = (x - 0.5, y - 0.5);
        let (x0, y0) = (fx.floor(), fy.floor());
        let (tx, ty) = (fx - x0, fy - y0);
        let (x0, y0) = (x0 as i32, y0 as i32);
        self.plot(x0, y0, c, a * (1.0 - tx) * (1.0 - ty));
        self.plot(x0 + 1, y0, c, a * tx * (1.0 - ty));
        self.plot(x0, y0 + 1, c, a * (1.0 - tx) * ty);
        self.plot(x0 + 1, y0 + 1, c, a * tx * ty);
    }

    /// A hot pixel with an anti-aliased streak behind it, `len` steps of
    /// motion long, fading towards the tail.
    fn streak(&mut self, p: &P, c: V3, a: f32, len: f32) {
        let (dx, dy) = (p.vx * len, p.vy * len);
        let n = dx.abs().max(dy.abs()).min(16.0).ceil() as i32;
        for j in 1..n {
            let u = j as f32 / n as f32;
            self.splat(p.x - dx * u, p.y - dy * u, c, a * 0.4 * (1.0 - u));
        }
        self.plot(p.x.floor() as i32, p.y.floor() as i32, c, a);
    }

    fn draw_particles(&mut self) {
        self.light.fill([0.0; 3]);
        let native = self.pal.native;
        for i in 0..self.ps.len() {
            let p = self.ps[i];
            let r = (p.life / p.max).clamp(0.0, 1.0);
            if p.flags & STROBE != 0 && r < 0.6 && self.rng.f32() < 0.45 {
                continue;
            }
            if p.flags & ROCKET != 0 {
                let c = if native {
                    ROCKET_FIRE
                } else {
                    mix(self.pal.gold, self.pal.white, 0.5)
                };
                self.streak(&p, c, 1.2, 3.0);
                continue;
            }
            let ember = if native { EMBER } else { scale(p.color, 0.3) };
            let mut col = scale(mix(p.color, ember, (1.0 - r) * 1.1), 0.35 + 0.65 * r);
            // Short-lived bits (trail embers, rocket sparks, crackles) are
            // dimmer and never white-hot; real stars flash white, then bloom.
            if p.max <= 20.0 {
                self.streak(&p, col, 0.45, 2.0);
                continue;
            }
            if r > 0.94 {
                col = mix(col, [1.0; 3], (r - 0.94) * 10.0);
            }
            self.streak(&p, col, 1.0, 6.0);
            if r > 0.6 && r < 0.95 {
                let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
                let b = (r - 0.6) * 0.6;
                for (ox, oy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    self.plot(x + ox, y + oy, col, b);
                }
            }
        }
    }

    fn draw_glow(&mut self) {
        self.glow.fill([0.0; 3]);
        let (w, ph) = (self.w as i32, self.ph as i32);
        for f in &self.flashes {
            let r = f.radius;
            let (x0, x1) = (((f.x - r) as i32).max(0), ((f.x + r) as i32 + 1).min(w));
            let (y0, y1) = (((f.y - r) as i32).max(0), ((f.y + r) as i32 + 1).min(ph));
            for y in y0..y1 {
                for x in x0..x1 {
                    let (dx, dy) = ((x as f32 + 0.5 - f.x) / r, (y as f32 + 0.5 - f.y) / r);
                    let d = dx * dx + dy * dy;
                    if d >= 1.0 {
                        continue;
                    }
                    let g = f.power * (1.0 - d) * (1.0 - d);
                    let p = &mut self.glow[(y * w + x) as usize];
                    for (d, s) in p.iter_mut().zip(f.color) {
                        *d += s * g;
                    }
                }
            }
        }
    }

    /// One tanim tick: scheduling, then moving every particle.
    fn step(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        self.born.clear();

        // Launch scheduling: calm stretches, then an occasional finale.
        self.until_finale -= 1;
        if self.until_finale <= 0 {
            self.finale = self.rng.i32(90..150);
            self.until_finale = self.rng.i32(900..1600);
        }
        self.timer -= 1;
        if self.timer <= 0 {
            // A higher rate shortens the gaps between launches.
            let k = self.rate.max(0.05);
            let gap = |rng: &mut fastrand::Rng, a: i32, b: i32| {
                ((rng.i32(a..b) as f32 / k).round() as i32).max(1)
            };
            if self.finale > 0 {
                if self.rate > 0.0 {
                    for _ in 0..self.rng.i32(1..3) {
                        self.launch();
                    }
                }
                self.timer = gap(&mut self.rng, 3, 7);
            } else {
                if self.rate > 0.0 {
                    self.launch();
                }
                self.timer = gap(&mut self.rng, 12, 55);
            }
        }
        if self.finale > 0 {
            self.finale -= 1;
            if self.finale == 0 {
                self.timer = 70;
            }
        }

        let ph = self.ph as f32;
        let mut i = 0;
        while i < self.ps.len() {
            let p = &mut self.ps[i];
            p.vx *= p.drag;
            p.vy = p.vy * p.drag + p.grav;
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 1.0;
            let p = *p;
            let mut dead = p.life <= 0.0 || p.y > ph + 2.0;
            if p.flags & ROCKET != 0 {
                let rng = &mut self.rng;
                if rng.f32() < 0.8 {
                    self.born.push(P {
                        x: p.x + rangef(rng, -0.3, 0.3),
                        y: p.y + 1.0,
                        vx: rangef(rng, -0.08, 0.08),
                        vy: rangef(rng, 0.0, 0.15),
                        life: rangef(rng, 6.0, 14.0),
                        max: 14.0,
                        color: if self.pal.native {
                            TRAIL_FIRE
                        } else {
                            self.pal.gold
                        },
                        drag: 0.9,
                        grav: 0.005,
                        flags: 0,
                        shell: Shell::Sphere,
                    });
                }
                if p.vy > -0.25 {
                    self.burst(&p);
                    dead = true;
                }
            } else if p.flags & TRAIL != 0 && self.rng.f32() < 0.5 {
                self.born.push(P {
                    vx: 0.0,
                    vy: 0.02,
                    life: 18.0,
                    max: 18.0,
                    drag: 0.9,
                    grav: 0.002,
                    flags: 0,
                    ..p
                });
            }
            if dead && p.flags & CRACKLE != 0 {
                for _ in 0..2 {
                    let rng = &mut self.rng;
                    self.born.push(P {
                        vx: rangef(rng, -0.25, 0.25),
                        vy: rangef(rng, -0.25, 0.25),
                        life: rangef(rng, 4.0, 9.0),
                        max: 9.0,
                        color: [1.0; 3],
                        drag: 0.85,
                        flags: 0,
                        ..p
                    });
                }
            }
            if dead {
                self.ps.swap_remove(i);
            } else {
                i += 1;
            }
        }
        let room = MAX_PARTICLES.saturating_sub(self.ps.len());
        let n = self.born.len().min(room);
        self.ps.extend_from_slice(&self.born[..n]);

        for f in &mut self.flashes {
            f.power *= 0.82;
        }
        self.flashes.retain(|f| f.power > 0.02);

        // Occasionally someone turns a light on or off.
        if !self.windows.is_empty() && self.rng.f32() < 0.05 {
            let k = self.rng.usize(0..self.windows.len());
            self.windows[k].2 = !self.windows[k].2;
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
        // Hue at full strength; hot light spills over into white.
        let over = (m - 1.0).max(0.0) * 0.3;
        let hue = [l[0] / m + over, l[1] / m + over, l[2] / m + over];
        let alpha = m.min(1.0);
        let c = to_color(hue);
        if known {
            ctx.fade(c, 1.0 - alpha)
        } else {
            c.scale(0.4 + 0.6 * alpha)
        }
    }
}

impl Scene for Fireworks {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.w = w as usize;
        self.ph = h as usize * 2;
        self.px.resize(w, h);
        self.light = vec![[0.0; 3]; self.w * self.ph];
        self.glow = vec![[0.0; 3]; self.w * self.ph];
        self.ps.clear();
        self.flashes.clear();
        self.build_city();
        self.build_sky();
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let t = ctx.theme;
        self.pal = if t.native {
            Palette {
                native: true,
                accents: vec![GOLD],
                gold: GOLD,
                white: SHELL_WHITE,
                sky_top: SKY_TOP,
                sky_low: SKY_LOW,
                // Lighter than tanim's, which has a painted sky to stand out against.
                building: if self.paint_sky {
                    BUILDING
                } else {
                    scale(BUILDING, 3.0)
                },
                star: STAR,
                windows: [v3(rgb(0x463a1a)), v3(rgb(0x5f4e22)), v3(rgb(0x283448))],
            }
        } else {
            let g = &t.grad;
            let low = v3(g.at(0.0));
            let mid = v3(g.at(0.5));
            let mut accents: Vec<V3> = t.colors.iter().map(|&c| v3(c)).collect();
            if accents.is_empty() {
                accents.push(v3(g.at(0.8)));
            }
            Palette {
                native: false,
                accents,
                gold: v3(g.at(0.8)),
                white: v3(g.at(1.0)),
                sky_top: scale(v3(t.bg), 0.6),
                sky_low: mix(v3(t.bg), mid, 0.15),
                building: mix(low, mid, if self.paint_sky { 0.1 } else { 0.25 }),
                star: v3(g.at(1.0)),
                windows: [
                    scale(v3(g.at(0.85)), 0.6),
                    scale(v3(g.at(0.6)), 0.7),
                    scale(v3(g.at(0.3)), 0.9),
                ],
            }
        };
        self.build_sky();
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.25)
        } else {
            0.0
        };
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

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let (w, ph) = (self.w, self.ph);
        if w == 0 || ph == 0 {
            return;
        }
        self.draw_glow();
        self.draw_particles();
        self.px.clear();
        let known = matches!(ctx.bg, Color::Rgb(..));
        let b = self.pal.building;
        for y in 0..ph {
            for x in 0..w {
                let i = y * w + x;
                let g = self.glow[i];
                let col = if (y as i32) < self.skyline[x] {
                    let l = self.light[i];
                    if self.paint_sky {
                        // Hot light spills over into white where a channel clips.
                        let over = (l[0].max(l[1]).max(l[2]) - 1.0).max(0.0) * 0.3;
                        let s = self.sky[i];
                        to_color([
                            s[0] + (l[0] + over).min(1.0) + g[0],
                            s[1] + (l[1] + over).min(1.0) + g[1],
                            s[2] + (l[2] + over).min(1.0) + g[2],
                        ])
                    } else {
                        Self::sky_light(ctx, l)
                    }
                } else {
                    // Buildings hide the sparks but catch a little of the glow.
                    let k = self.shade[x];
                    to_color([
                        b[0] * k + g[0] * 0.3,
                        b[1] * k + g[1] * 0.3,
                        b[2] * k + g[2] * 0.3,
                    ])
                };
                self.px.set(x as i32, y as i32, col);
            }
        }
        if !self.paint_sky {
            // Stars shine where the sky is empty.
            let star = to_color(self.pal.star);
            for &(i, a) in &self.stars {
                let (x, y) = ((i % w) as i32, (i / w) as i32);
                if y < self.skyline[x as usize] && self.px.get(x, y) == Color::Reset {
                    let alpha = 0.2 + a * 1.5;
                    let col = if known {
                        ctx.fade(star, 1.0 - alpha)
                    } else {
                        star.scale(alpha)
                    };
                    self.px.set(x, y, col);
                }
            }
        }
        for &(i, kind, on) in &self.windows {
            if on {
                let (x, y) = ((i % w) as i32, (i / w) as i32);
                self.px
                    .set(x, y, to_color(self.pal.windows[kind as usize % 3]));
            }
        }
        for &(i, phase) in &self.beacons {
            if (self.frame.wrapping_add(phase)) % 90 < 45 {
                let (x, y) = ((i % w) as i32, (i / w) as i32);
                let c = if self.pal.native {
                    BEACON
                } else {
                    self.pal.accents[0]
                };
                self.px.set(x, y, to_color(c));
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

    fn make(opts: &str, theme: &Theme) -> (Fireworks, Opts) {
        let o = Opts::new(INFO.options, opts.parse().unwrap());
        let ctx = Ctx {
            theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut f = Fireworks::new(&ctx);
        f.resize(&ctx, 80, 24);
        f.retheme(&ctx);
        (f, o)
    }

    /// Odd options and sizes survive long runs and huge steps.
    #[test]
    fn variants_survive() {
        let variants = [
            "",
            "sky = true\nrate = 5.0\nspeed = 4.0",
            "skyline = false\nstars = false\nrate = 0.0",
            "speed = nan\nrate = inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                let (mut f, o) = make(v, &theme);
                let ctx = Ctx {
                    theme: &theme,
                    opts: &o,
                    bg: Color::Reset,
                    seed: 3,
                    fiesta: crate::fiesta::Fiesta::NONE,
                };
                for (w, h) in [(80, 24), (1, 1), (2, 3), (7, 2), (300, 90)] {
                    f.resize(&ctx, w, h);
                    f.retheme(&ctx);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for k in 0..400 {
                        f.update(&ctx, if k % 97 == 96 { 5.0 } else { 1.0 / 30.0 });
                        c.clear();
                        f.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }

    /// With the sky transparent nothing is painted before the first launch,
    /// and most of the screen stays empty afterwards.
    #[test]
    fn sky_is_transparent() {
        let theme = Theme::classic();
        let (mut f, o) = make("skyline = false\nstars = false", &theme);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut c = Canvas::new(80, 24, Color::Reset);
        f.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch == ' '));
        let mut painted = 0;
        for _ in 0..600 {
            f.update(&ctx, 1.0 / 30.0);
            c.clear();
            f.draw(&ctx, &mut c);
            painted = painted.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
        }
        assert!(painted > 0);
        assert!(painted < 80 * 24 / 2);
    }

    /// `sky = true` paints every cell, and the city is on by default.
    #[test]
    fn painted_sky_and_city() {
        let theme = Theme::classic();
        let (mut f, o) = make("sky = true", &theme);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut c = Canvas::new(80, 24, Color::Reset);
        f.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch != ' '));
        assert!(!f.windows.is_empty());
    }

    /// Rockets burst into hundreds of sparks, within the particle cap.
    #[test]
    fn bursts_happen() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut peak = 0;
        for _ in 0..900 {
            f.update(&ctx, 1.0 / 30.0);
            peak = peak.max(f.ps.len());
        }
        assert!(peak > 60, "peak {peak}");
        assert!(f.ps.len() <= MAX_PARTICLES);
    }
}
