//! rain: rainfall in depth layers, with gusty wind, splashes and lightning.
//!
//! Near drops are bright, long and fast; far ones are short, slow and faded
//! toward the background. Every drop falls along the current wind slope, so
//! the glyphs follow the angle (`│ |` straight down, `\ /` when slanted).

use crate::color::{Color, Gradient, rgb};
use crate::render::{BOLD, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "rain",
    aliases: &[],
    about: "rainfall in layers, with gusty wind, splashes and lightning",
    options: &[
        Opt {
            key: "intensity",
            default: Val::F(1.0),
            help: "How hard it rains (drop count also scales with the screen)",
        },
        Opt {
            key: "wind",
            default: Val::F(0.2),
            help: "Steady wind: negative blows left, positive right (1 = columns move one per row)",
        },
        Opt {
            key: "gusts",
            default: Val::F(0.3),
            help: "How far the wind wanders from its steady value, slowly",
        },
        Opt {
            key: "layers",
            default: Val::I(3),
            help: "Depth layers (1-6): near drops are bright, long and fast, far ones dim and slow",
        },
        Opt {
            key: "splashes",
            default: Val::B(true),
            help: "Splash where drops hit the bottom row",
        },
        Opt {
            key: "puddle",
            default: Val::B(false),
            help: "Draw a puddle line along the bottom with ripples spreading from splashes",
        },
        Opt {
            key: "lightning",
            default: Val::F(0.5),
            help: "Lightning strikes per minute on average (0 = off)",
        },
        Opt {
            key: "lightning_bg",
            default: Val::B(false),
            help: "Light up the cell backgrounds during a flash (paints backgrounds)",
        },
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Fall speed multiplier",
        },
    ],
    make: |ctx| Box::new(Rain::new(ctx)),
};

/// Classic palette: slate to pale steel blue.
const CLASSIC_RAMP: [Color; 5] = [
    rgb(0x1f2a3a),
    rgb(0x3d5573),
    rgb(0x6d8fb6),
    rgb(0xaac4e0),
    rgb(0xe2ecf7),
];
const CLASSIC_FLASH: Color = rgb(0xf2f6ff);

const MAX_LAYERS: usize = 6;
const MAX_LEN: usize = 6;
/// Lightning flash length in seconds.
const FLASH_LEN: f32 = 1.1;

#[derive(Clone, Copy)]
struct Drop {
    x: f32,
    y: f32,
    vy: f32,
    layer: u8,
    len: u8,
    /// Far drops alternate between two looks.
    alt: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Big,
    Small,
    Ripple,
}

#[derive(Clone, Copy)]
struct Splash {
    x: i32,
    age: f32,
    kind: Kind,
}

#[derive(Clone, Copy)]
struct Layer {
    /// 0 = nearest, 1 = farthest.
    depth: f32,
    vy: f32,
    len: f32,
    density: f32,
    /// Glyphs for: straight, slanted right (`\`), slanted left (`/`), straight alt.
    glyphs: [char; 4],
    /// Above this slope the slanted glyphs are used.
    slant: f32,
    /// Glyph index in use: 0 straight, 1 right, 2 left (switched with hysteresis).
    lean: u8,
    splash: Option<Kind>,
    /// Head and tail colors (index 0 = head).
    colors: [Color; MAX_LEN],
}

#[derive(Clone, Copy)]
struct BoltPt {
    x: i16,
    y: i16,
    ch: char,
    main: bool,
}

struct Rain {
    rng: fastrand::Rng,
    intensity: f32,
    wind_base: f32,
    gusts: f32,
    nlayers: usize,
    splashes_on: bool,
    puddle: bool,
    lightning: f32,
    lightning_bg: bool,
    speed: f32,

    w: i32,
    h: i32,
    layers: [Layer; MAX_LAYERS],
    /// Drops grouped by layer: layer `i` owns `drops[span[i]..span[i + 1]]`.
    drops: Vec<Drop>,
    span: [usize; MAX_LAYERS + 1],
    splashes: Vec<Splash>,
    splash_cap: usize,

    wind: f32,
    wind_target: f32,
    gust_timer: f32,

    next_strike: f32,
    /// Seconds since the last strike (≥ FLASH_LEN when idle).
    flash_t: f32,
    flickers: [(f32, f32); 3],
    bolt: Vec<BoltPt>,
    bolt_cap: usize,

    flash_color: Color,
    puddle_color: Color,
    ripple_color: Color,
}

impl Rain {
    fn new(ctx: &Ctx) -> Rain {
        let o = ctx.opts;
        let mut r = Rain {
            rng: ctx.rng(),
            intensity: o.f("intensity").clamp(0.0, 20.0),
            wind_base: o.f("wind").clamp(-3.0, 3.0),
            gusts: o.f("gusts").clamp(0.0, 3.0),
            nlayers: o.i("layers").clamp(1, MAX_LAYERS as i64) as usize,
            splashes_on: o.b("splashes"),
            puddle: o.b("puddle"),
            lightning: o.f("lightning").clamp(0.0, 600.0),
            lightning_bg: o.b("lightning_bg"),
            speed: o.f("speed").clamp(0.0, 20.0),
            w: 1,
            h: 1,
            layers: [Layer {
                depth: 0.0,
                vy: 0.0,
                len: 1.0,
                density: 0.0,
                glyphs: ['|'; 4],
                slant: 0.45,
                lean: 0,
                splash: None,
                colors: [Color::Reset; MAX_LEN],
            }; MAX_LAYERS],
            drops: Vec::new(),
            span: [0; MAX_LAYERS + 1],
            splashes: Vec::new(),
            splash_cap: 0,
            wind: 0.0,
            wind_target: 0.0,
            gust_timer: 0.0,
            next_strike: 0.0,
            flash_t: FLASH_LEN,
            flickers: [(0.0, 0.0); 3],
            bolt: Vec::new(),
            bolt_cap: 0,
            flash_color: CLASSIC_FLASH,
            puddle_color: Color::Reset,
            ripple_color: Color::Reset,
        };
        r.wind = r.wind_base;
        r.wind_target = r.wind_base;
        r.next_strike = r.strike_gap() * 0.5;
        for i in 0..r.nlayers {
            let t = if r.nlayers == 1 {
                0.0
            } else {
                i as f32 / (r.nlayers - 1) as f32
            };
            let l = &mut r.layers[i];
            l.depth = t;
            l.vy = 13.0 + 27.0 * (1.0 - t);
            l.len = 1.0 + 3.0 * (1.0 - t).powf(1.5);
            l.density = 0.009 + 0.022 * t;
            (l.glyphs, l.slant) = match t {
                t if t < 0.25 => (['│', '\\', '/', '│'], 0.42),
                t if t < 0.6 => (['|', '\\', '/', '|'], 0.42),
                t if t < 0.85 => (['¦', '`', ',', '¦'], 0.3),
                _ => (['\'', '`', ',', '.'], 0.3),
            };
            l.splash = match t {
                t if t < 0.25 => Some(Kind::Big),
                t if t < 0.6 => Some(Kind::Small),
                _ => None,
            };
            l.lean = if r.wind > l.slant {
                1
            } else if r.wind < -l.slant {
                2
            } else {
                0
            };
        }
        r
    }

    /// Seconds until the next strike: exponential around the configured rate.
    fn strike_gap(&mut self) -> f32 {
        if self.lightning <= 0.0 {
            return f32::INFINITY;
        }
        let mean = 60.0 / self.lightning;
        (-(1.0 - self.rng.f32()).max(1e-6).ln() * mean).max(mean.min(3.0))
    }

    fn ground(&self) -> i32 {
        self.h - 1
    }

    fn spawn(&mut self, d: &mut Drop, anywhere: bool) {
        let l = &self.layers[d.layer as usize];
        let len = if l.len >= 3.0 {
            (l.len + self.rng.f32() * 2.0 - 1.0).round()
        } else {
            l.len.round()
        };
        d.len = (len as usize).clamp(1, MAX_LEN) as u8;
        d.vy = l.vy * (0.88 + self.rng.f32() * 0.24);
        d.x = self.rng.f32() * self.w as f32;
        d.y = if anywhere {
            self.rng.f32() * (self.h as f32 + d.len as f32) - d.len as f32
        } else {
            -self.rng.f32() * 3.0
        };
        d.alt = self.rng.bool();
    }

    fn add_splash(&mut self, x: i32, kind: Kind) {
        if self.splashes.len() < self.splash_cap {
            self.splashes.push(Splash { x, age: 0.0, kind });
        }
    }

    fn strike(&mut self) {
        self.flash_t = 0.0;
        let r = &mut self.rng;
        self.flickers[0] = (0.0, 1.0);
        self.flickers[1] = (0.1 + r.f32() * 0.1, 0.55 + r.f32() * 0.35);
        self.flickers[2] = if r.bool() {
            (0.3 + r.f32() * 0.2, 0.3 + r.f32() * 0.3)
        } else {
            (0.0, 0.0)
        };
        self.bolt.clear();
        let (w, h) = (self.w, self.h);
        if h < 3 || w < 3 {
            return;
        }
        let x0 = self.rng.i32(w / 8..(w - w / 8).max(w / 8 + 1));
        let end = self.rng.i32((h * 6 / 10).max(1)..h);
        self.bolt_path(x0, 0, end, 0, true);
    }

    /// A jagged path going down from (`x`, `y0`) to row `y1`. `bias` leans a
    /// branch to one side; the main bolt may sprout up to three branches.
    fn bolt_path(&mut self, mut x: i32, y0: i32, y1: i32, bias: i32, main: bool) {
        let mut dir = if self.rng.bool() { 1 } else { -1 };
        let mut run = 0;
        let mut branches = 0;
        for y in y0..y1 {
            if run == 0 {
                dir = match self.rng.u32(0..10) {
                    0..=2 => 0,
                    3..=6 => 1,
                    _ => -1,
                };
                if bias != 0 && self.rng.u32(0..3) > 0 {
                    dir = bias;
                }
                run = self.rng.i32(1..4);
            }
            run -= 1;
            let ch = match dir {
                1 => '╲',
                -1 => '╱',
                _ => '│',
            };
            if self.bolt.len() >= self.bolt_cap {
                return;
            }
            self.bolt.push(BoltPt {
                x: x as i16,
                y: y as i16,
                ch,
                main,
            });
            x += dir;
            if main && branches < 3 && y > y0 + 2 && self.rng.u32(0..100) < 9 {
                branches += 1;
                let len = self.rng.i32(3..(self.h / 3).max(4));
                let b = if self.rng.bool() { 1 } else { -1 };
                self.bolt_path(x, y + 1, (y + 1 + len).min(y1), b, false);
            }
        }
    }

    /// Current flash brightness in 0..1: a few quickly decaying flickers.
    fn flash(&self) -> f32 {
        if self.flash_t >= FLASH_LEN {
            return 0.0;
        }
        let mut f: f32 = 0.0;
        for &(t0, a) in &self.flickers {
            let dt = self.flash_t - t0;
            if a > 0.0 && dt >= 0.0 {
                f = f.max(a * (-dt / 0.08).exp());
            }
        }
        f
    }

    fn draw_layer(&self, c: &mut Canvas, li: usize, flash: f32) {
        let l = &self.layers[li];
        let s = self.wind;
        let w = self.w;
        let mut cols = l.colors;
        if flash > 0.0 {
            for col in cols.iter_mut() {
                *col = col.lerp(self.flash_color, flash * 0.75);
            }
        }
        let gi = l.lean as usize;
        // Straight glyphs keep their streak upright; slanted ones follow the slope.
        let lean = if gi == 0 { 0.0 } else { s };
        for d in &self.drops[self.span[li]..self.span[li + 1]] {
            let ch = if gi == 0 && d.alt {
                l.glyphs[3]
            } else {
                l.glyphs[gi]
            };
            let hy = d.y.floor() as i32;
            for k in 0..d.len as i32 {
                let y = hy - k;
                if y < 0 {
                    break;
                }
                let x = (d.x - lean * k as f32).floor() as i32;
                c.set(x.rem_euclid(w), y, ch, cols[k as usize]);
            }
        }
    }

    fn draw_bolt(&self, c: &mut Canvas, flash: f32) {
        if flash < 0.3 {
            return;
        }
        let branch = self.flash_color.lerp(self.ripple_color, 0.35);
        for p in &self.bolt {
            if p.main {
                c.set_attr(p.x as i32, p.y as i32, p.ch, self.flash_color, BOLD);
            } else {
                c.set(p.x as i32, p.y as i32, p.ch, branch);
            }
        }
    }

    fn draw_ground(&self, c: &mut Canvas, flash: f32) {
        let g = self.ground();
        if self.puddle {
            let pc = self.puddle_color.lerp(self.flash_color, flash * 0.5);
            for x in 0..self.w {
                c.set(x, g, '_', pc);
            }
        }
        let big = self.layers[0].colors[0];
        let small = self.layers[(self.nlayers / 2).min(self.nlayers - 1)].colors[0];
        for s in &self.splashes {
            match s.kind {
                Kind::Ripple => {
                    let r = 1 + (s.age * 10.0) as i32;
                    let col = self.ripple_color.lerp(self.puddle_color, s.age / 0.6);
                    c.set(s.x - r, g, '~', col);
                    c.set(s.x + r, g, '~', col);
                }
                Kind::Big => {
                    let col = big.lerp(self.flash_color, flash * 0.6);
                    match (s.age / 0.08) as i32 {
                        0 => c.set(s.x, g, '‿', col),
                        1 => {
                            c.set(s.x, g, '^', col);
                            c.set(s.x - 1, g - 1, '.', col);
                            c.set(s.x + 1, g - 1, '.', col);
                        }
                        2 => {
                            c.set(s.x - 2, g - 1, '˙', col);
                            c.set(s.x + 2, g - 1, '˙', col);
                            c.set(s.x, g, '.', col);
                        }
                        _ => {
                            c.set(s.x - 2, g, '.', col);
                            c.set(s.x + 2, g, '.', col);
                        }
                    }
                }
                Kind::Small => {
                    let col = small.lerp(self.flash_color, flash * 0.6);
                    if s.age < 0.08 {
                        c.set(s.x, g, '.', col);
                    } else {
                        c.set(s.x - 1, g, '˙', col);
                        c.set(s.x + 1, g, '˙', col);
                    }
                }
            }
        }
    }

    fn step(&mut self, dt: f32) {
        // Wind: drift toward a slowly re-picked gust target.
        self.gust_timer -= dt;
        if self.gust_timer <= 0.0 {
            self.wind_target = self.wind_base + self.gusts * (self.rng.f32() * 2.0 - 1.0);
            self.gust_timer = 2.0 + self.rng.f32() * 5.0;
        }
        self.wind += (self.wind_target - self.wind) * (1.0 - (-dt / 1.6).exp());
        self.wind = self.wind.clamp(-2.5, 2.5);
        let s = self.wind;
        for l in self.layers.iter_mut() {
            // Hysteresis keeps a wind hovering at the threshold from flickering.
            l.lean = match l.lean {
                0 if s > l.slant + 0.04 => 1,
                0 if s < -l.slant - 0.04 => 2,
                1 if s < l.slant - 0.04 => 0,
                2 if s > -l.slant + 0.04 => 0,
                k => k,
            };
        }

        let ground = self.ground() as f32;
        let w = self.w as f32;
        let speed = self.speed;
        for i in 0..self.drops.len() {
            let mut d = self.drops[i];
            let v = d.vy * speed * dt;
            d.y += v;
            d.x = (d.x + s * v).rem_euclid(w);
            if d.y >= ground + 1.0
                || (d.y >= ground && self.layers[d.layer as usize].splash.is_some())
            {
                if let (true, Some(kind)) = (self.splashes_on, self.layers[d.layer as usize].splash)
                {
                    let chance = if kind == Kind::Big { 0.85 } else { 0.4 };
                    if self.rng.f32() < chance {
                        let x = d.x.floor() as i32;
                        self.add_splash(x, kind);
                        if self.puddle && kind == Kind::Big {
                            self.add_splash(x, Kind::Ripple);
                        }
                    }
                }
                self.spawn(&mut d, false);
            }
            self.drops[i] = d;
        }

        for sp in self.splashes.iter_mut() {
            sp.age += dt;
        }
        self.splashes.retain(|s| {
            s.age
                < match s.kind {
                    Kind::Big => 0.32,
                    Kind::Small => 0.16,
                    Kind::Ripple => 0.6,
                }
        });

        self.flash_t = (self.flash_t + dt).min(FLASH_LEN);
        self.next_strike -= dt;
        if self.next_strike <= 0.0 {
            self.strike();
            self.next_strike = self.strike_gap();
        }
    }
}

impl Scene for Rain {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.w = w.max(1) as i32;
        self.h = h.max(1) as i32;
        let area = (self.w * self.h) as f32;
        self.drops.clear();
        let mut want = [0usize; MAX_LAYERS];
        let mut total = 0;
        for (i, n) in want.iter_mut().enumerate().take(self.nlayers) {
            *n = (self.layers[i].density * area * self.intensity).round() as usize;
            total += *n;
        }
        // Keep absurd intensities bounded.
        let scale = if total > 60_000 {
            60_000.0 / total as f32
        } else {
            1.0
        };
        self.drops.reserve_exact(total);
        self.span = [0; MAX_LAYERS + 1];
        for (li, &n) in want.iter().enumerate().take(self.nlayers) {
            self.span[li] = self.drops.len();
            for _ in 0..(n as f32 * scale) as usize {
                let mut d = Drop {
                    x: 0.0,
                    y: 0.0,
                    vy: 0.0,
                    layer: li as u8,
                    len: 1,
                    alt: false,
                };
                self.spawn(&mut d, true);
                self.drops.push(d);
            }
        }
        for slot in self.span.iter_mut().skip(self.nlayers) {
            *slot = self.drops.len();
        }
        self.splash_cap = 64 + self.w as usize * 2;
        self.splashes = Vec::with_capacity(self.splash_cap);
        self.bolt_cap = self.h as usize * 3 + 8;
        self.bolt = Vec::with_capacity(self.bolt_cap);
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let ramp: Gradient = ctx.ramp(&CLASSIC_RAMP);
        for l in self.layers.iter_mut().take(self.nlayers) {
            let t = l.depth;
            let head = ctx.fade(ramp.at(0.95 - 0.4 * t), 0.4 * t);
            let n = (l.len.round() as usize).clamp(1, MAX_LEN);
            for (k, slot) in l.colors.iter_mut().enumerate() {
                let f = if k == 0 {
                    0.0
                } else {
                    (0.15 + 0.6 * k as f32 / (n as f32 + 1.0)).min(0.85)
                };
                *slot = ctx.fade(head, f);
            }
        }
        self.flash_color = if ctx.theme.native {
            CLASSIC_FLASH
        } else {
            ramp.at(1.0)
        };
        self.puddle_color = ctx.fade(ramp.at(0.6), 0.45);
        self.ripple_color = ramp.at(0.85);
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        // Sub-step so long frames neither tunnel splashes nor stall the clock.
        let mut left = dt.clamp(0.0, 1.0);
        while left > 0.0 {
            let h = left.min(0.05);
            self.step(h);
            left -= h;
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let flash = self.flash();
        if self.lightning_bg && flash > 0.02 {
            let base = match ctx.bg {
                Color::Rgb(..) => ctx.bg,
                _ => Color::Rgb(0, 0, 0),
            };
            let bg = base.lerp(self.flash_color, flash * 0.22);
            for y in 0..self.h {
                for x in 0..self.w {
                    c.set_bg(x, y, bg);
                }
            }
        }
        // Far to near, with the bolt behind the nearest layer.
        for li in (0..self.nlayers).rev() {
            if li == 0 {
                self.draw_bolt(c, flash);
            }
            self.draw_layer(c, li, flash);
        }
        self.draw_ground(c, flash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the scene with `opts` through odd sizes, huge steps and themes.
    fn torture(opts: &str) {
        let table: toml::Table = opts.parse().unwrap();
        let o = crate::scene::Opts::new(INFO.options, table);
        for (theme, bg) in [("classic", Color::Reset), ("nord", Color::Rgb(20, 20, 30))] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = Ctx {
                theme: &th,
                opts: &o,
                bg,
                seed: 11,
                fiesta: crate::fiesta::Fiesta::NONE,
            };
            let mut s = (INFO.make)(&ctx);
            for &(w, h) in &[(1u16, 1u16), (2, 3), (500, 150), (3, 1), (1, 40), (37, 11)] {
                s.resize(&ctx, w, h);
                s.retheme(&ctx);
                let mut c = Canvas::new(w, h, Color::Reset);
                for i in 0..60 {
                    s.update(&ctx, if i % 20 == 19 { 5.0 } else { 1.0 / 30.0 });
                    c.clear();
                    s.draw(&ctx, &mut c);
                    let clean = c.cells().iter().all(|cell| cell.bg == Color::Reset);
                    assert!(clean || opts.contains("_bg = true"), "{opts}");
                }
            }
        }
    }

    #[test]
    fn survives_options() {
        torture("");
        torture(
            "intensity = 20.0\nlayers = 6\nwind = -3.0\ngusts = 3.0\nlightning = 600.0\npuddle = true\nspeed = 20.0",
        );
        torture("intensity = 0.0\nlayers = 1\nsplashes = false\nlightning = 0.0");
        torture("layers = -2\nlightning = 60.0\nlightning_bg = true\nwind = 1.0");
    }
}
