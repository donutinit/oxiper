//! rain: rainfall in depth layers, with gusty wind, splashes and lightning.
//!
//! Near drops are bright, long and fast; far ones are short, slow and faded
//! toward the background. Every drop falls along the current wind slope, so
//! the glyphs follow the angle (`│ |` straight down, `\ /` when slanted).
//!
//! On Día de Muertos cempasúchil petals fall with the rain, tumbling and
//! carried further by the wind, and gather along the ground into the path
//! home; splashes throw up petals and leave some behind. Veladoras burn along
//! the ground and the drops falling past them catch their light. Lightning
//! strikes more often (more as the holiday builds up) and each flash catches
//! skeletons dancing in the dark, a new step at every flicker, fading back
//! into the night after it (see `muertos`).
//!
//! During the Navidad a string of foquitos hangs along the eaves, blinking in
//! chase, alternating, twinkling and fading patterns (more of them, and a
//! second string, as the posadas climb); the drops falling past catch the
//! colors of the bulbs and the puddles on the ground reflect them as they
//! blink; farolitos burn along the street. Lightning strikes more often, and
//! each flash catches a posada walking by in the rain: José with his staff
//! and lantern, María on the burro, peregrinos and little ones under
//! umbrellas, a candle in each hand, the candles still alight in the dark
//! after the rest has gone (see `navidad`).
//!
//! On the Fiestas Patrias it rains on a street of fondas and puestos under
//! tricolor toldos, banderas flying from their poles, strings of papel
//! picado hanging soaked across it and dripping onto the wet street, where
//! scraps of paper stick; the lit windows warm the drops falling past.
//! Lightning strikes more often and each flash catches charros galloping down
//! the street with the flag in hand, a new stride at every flicker. More of it
//! all as the mes patrio climbs to the Grito (see `patrias`).

mod art;
mod muertos;
mod navidad;
mod patrias;

use muertos::Muertos;
use navidad::Navidad;
use patrias::Patrias;

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
    /// Throws up marigold petals (Día de Muertos).
    petal: bool,
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

    /// Día de Muertos, Navidad and the Fiestas Patrias, off on ordinary days.
    muertos: Muertos,
    navidad: Navidad,
    patrias: Patrias,
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
            muertos: Muertos::new(ctx.seed),
            navidad: Navidad::new(ctx.seed),
            patrias: Patrias::new(ctx.seed),
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
        let storm = self.muertos.storm() * self.navidad.storm() * self.patrias.storm();
        let mean = 60.0 / (self.lightning * storm);
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
            let petal = kind == Kind::Big && self.muertos.on() && self.muertos.splash(x);
            self.splashes.push(Splash {
                x,
                age: 0.0,
                kind,
                petal,
            });
        }
    }

    /// Dresses up for the holiday on today (or undresses), and lets the
    /// first lightning come soon once Día de Muertos, the Navidad or the
    /// Fiestas Patrias arrive.
    fn dress(&mut self, ctx: &Ctx) {
        if self.muertos.set(ctx.fiesta, self.w, self.h) && self.lightning > 0.0 {
            self.next_strike = self.next_strike.min(self.muertos.soon());
        }
        if self.navidad.set(ctx.fiesta, self.w, self.h) && self.lightning > 0.0 {
            self.next_strike = self.next_strike.min(self.navidad.soon());
        }
        if self.patrias.set(ctx.fiesta, self.w, self.h) && self.lightning > 0.0 {
            self.next_strike = self.next_strike.min(self.patrias.soon());
        }
    }

    /// Which flicker of the flash is showing (0 = the first).
    fn beat(&self) -> usize {
        let lit = self
            .flickers
            .iter()
            .filter(|&&(t0, a)| a > 0.0 && self.flash_t >= t0);
        lit.count().saturating_sub(1)
    }

    fn strike(&mut self) {
        self.muertos.strike();
        self.navidad.strike();
        self.patrias.strike();
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
        let warm = self.muertos.on();
        let lamps = self.navidad.on();
        let windows = self.patrias.on();
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
                let x = x.rem_euclid(w);
                if warm {
                    c.set(x, y, ch, self.muertos.warm(x, y, cols[k as usize]));
                } else if lamps {
                    c.set(x, y, ch, self.navidad.tint(x, y, cols[k as usize]));
                } else if windows {
                    c.set(x, y, ch, self.patrias.tint(x, y, cols[k as usize]));
                } else {
                    c.set(x, y, ch, cols[k as usize]);
                }
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

    fn draw_ground(&self, ctx: &Ctx, c: &mut Canvas, flash: f32) {
        let g = self.ground();
        if self.puddle {
            let pc = self.puddle_color.lerp(self.flash_color, flash * 0.5);
            for x in 0..self.w {
                c.set(x, g, '_', pc);
            }
        }
        let muertos = self.muertos.on();
        let navidad = self.navidad.on();
        let patrias = self.patrias.on();
        if muertos {
            self.muertos.draw_fallen(ctx, c, flash);
        }
        if navidad {
            self.navidad.draw_puddles(c, flash);
        }
        if patrias {
            self.patrias.draw_scraps(c, flash);
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
                    // A splash that throws up petals: the spray is orange.
                    let spray = if s.petal {
                        self.muertos.spray(s.age)
                    } else {
                        col
                    };
                    match (s.age / 0.08) as i32 {
                        0 => c.set(s.x, g, '‿', col),
                        1 => {
                            c.set(s.x, g, '^', col);
                            c.set(s.x - 1, g - 1, '.', spray);
                            c.set(s.x + 1, g - 1, '.', spray);
                        }
                        2 => {
                            c.set(s.x - 2, g - 1, '˙', spray);
                            c.set(s.x + 2, g - 1, '˙', spray);
                            c.set(s.x, g, '.', col);
                        }
                        _ => {
                            c.set(s.x - 2, g, '.', spray);
                            c.set(s.x + 2, g, '.', spray);
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
        if muertos {
            self.muertos.draw_candles(c);
        }
        if navidad {
            self.navidad.draw_farolitos(c, flash);
        }
        if patrias {
            self.patrias.draw_drips(c);
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

        self.muertos.step(dt, s, speed);
        self.navidad.step(dt);
        self.patrias.step(dt, s);

        self.flash_t = (self.flash_t + dt).min(FLASH_LEN);
        self.next_strike -= dt;
        if self.next_strike <= 0.0 {
            self.strike();
            self.next_strike = self.strike_gap();
        }
    }
}

impl Scene for Rain {
    fn resize(&mut self, ctx: &Ctx, w: u16, h: u16) {
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
        self.dress(ctx);
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
        self.dress(ctx);
        self.muertos.retheme(ctx, self.flash_color);
        let water = self.ripple_color.lerp(self.puddle_color, 0.4);
        self.navidad.retheme(ctx, self.flash_color, water);
        self.patrias
            .retheme(ctx, self.flash_color, self.ripple_color);
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
        // Far to near, with the bolt behind the nearest layer, petals at three
        // depths among the rain and the dancers (or the posada, or the charros) the
        // bolt lights up in front; the lights and the street stand behind it.
        let muertos = self.muertos.on();
        let navidad = self.navidad.on();
        let patrias = self.patrias.on();
        if navidad {
            self.navidad.draw_lights(c, flash);
        }
        if patrias {
            self.patrias.draw_street(c, self.wind, flash);
        }
        if muertos {
            self.muertos.draw_petals(c, 2, flash);
        }
        for li in (0..self.nlayers).rev() {
            if li == 0 {
                self.draw_bolt(c, flash);
                if muertos {
                    self.muertos.draw_petals(c, 1, flash);
                }
            }
            self.draw_layer(c, li, flash);
        }
        if muertos {
            self.muertos.draw_dancers(ctx, c, flash, self.beat());
        }
        if navidad {
            self.navidad.draw_procession(ctx, c, flash, self.beat());
        }
        if patrias {
            self.patrias.draw_riders(ctx, c, flash, self.beat());
        }
        self.draw_ground(ctx, c, flash);
        if muertos {
            self.muertos.draw_petals(c, 0, flash);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;

    /// Runs the scene with `opts` through odd sizes, huge steps and themes.
    fn torture(opts: &str, fiesta: Fiesta) {
        let table: toml::Table = opts.parse().unwrap();
        let o = crate::scene::Opts::new(INFO.options, table);
        for (theme, bg) in [("classic", Color::Reset), ("nord", Color::Rgb(20, 20, 30))] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = Ctx {
                theme: &th,
                opts: &o,
                bg,
                seed: 11,
                fiesta,
            };
            let mut s = (INFO.make)(&ctx);
            for &(w, h) in &[
                (1u16, 1u16),
                (2, 3),
                (500, 150),
                (3, 1),
                (1, 40),
                (37, 11),
                (100, 30),
                (45, 15),
            ] {
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
        for fiesta in [
            Fiesta::NONE,
            Fiesta::of_day(11, 2),
            Fiesta::of_day(10, 28),
            Fiesta::of_day(12, 24),
            Fiesta::of_day(12, 12),
            Fiesta::of_day(1, 6),
            Fiesta::of_day(9, 1),
            Fiesta::of_day(9, 15),
            Fiesta::of_day(9, 16),
        ] {
            torture("", fiesta);
            torture(
                "intensity = 20.0\nlayers = 6\nwind = -3.0\ngusts = 3.0\nlightning = 600.0\npuddle = true\nspeed = 20.0",
                fiesta,
            );
            torture(
                "intensity = 0.0\nlayers = 1\nsplashes = false\nlightning = 0.0",
                fiesta,
            );
            torture(
                "layers = -2\nlightning = 60.0\nlightning_bg = true\nwind = 1.0",
                fiesta,
            );
        }
    }

    /// The glyphs of Día de Muertos (petals, flowers, candles) and of a
    /// dancer; of the Navidad (bulbs, puddles, farolitos) and of a posada,
    /// whose figures are in half blocks.
    const MUERTOS: (&str, &[&str]) = ("♦◆✿❀●◖◗•◦°█", &["(o.o)"]);
    const NAVIDAD: (&str, &[&str]) = ("●•·≈♦◆▐▌", &["▀", "▄"]);

    /// What `frames` frames at 120×36 show of a holiday: cells drawn in its
    /// glyphs and frames with one of its figures.
    fn seen(
        ctx: &Ctx,
        s: &mut dyn Scene,
        frames: usize,
        (glyphs, figures): (&str, &[&str]),
    ) -> (usize, usize) {
        let mut c = Canvas::new(120, 36, Color::Reset);
        let (mut cells, mut walking) = (0, 0);
        for _ in 0..frames {
            s.update(ctx, 1.0 / 30.0);
            c.clear();
            s.draw(ctx, &mut c);
            let ours = |cell: &&crate::render::Cell| glyphs.contains(cell.ch);
            cells += c.cells().iter().filter(ours).count();
            let text = c.to_text();
            if figures.iter().any(|f| text.contains(f)) {
                walking += 1;
            }
        }
        (cells, walking)
    }

    /// What `frames` frames show of the Fiestas Patrias: cells in the glyphs
    /// of the street (shop fronts, papel picado, scraps) and frames in which
    /// riders gallop by, many more cells in half blocks than the street ever
    /// shows.
    fn street(ctx: &Ctx, s: &mut dyn Scene, frames: usize) -> (usize, usize) {
        let mut c = Canvas::new(120, 36, Color::Reset);
        let (mut cells, mut blocks) = (0, Vec::new());
        for _ in 0..frames {
            s.update(ctx, 1.0 / 30.0);
            c.clear();
            s.draw(ctx, &mut c);
            let ours = |cell: &&crate::render::Cell| "█▀▄░▪".contains(cell.ch);
            let n = c.cells().iter().filter(ours).count();
            cells += n;
            blocks.push(n);
        }
        let least = blocks.iter().copied().min().unwrap_or(0);
        (cells, blocks.iter().filter(|&&n| n > least + 150).count())
    }

    /// What `frames` frames show of Día de Muertos.
    fn holiday(ctx: &Ctx, s: &mut dyn Scene, frames: usize) -> (usize, usize) {
        seen(ctx, s, frames, MUERTOS)
    }

    fn opts(lightning: f32) -> crate::scene::Opts {
        let table: toml::Table = format!("lightning = {lightning:?}").parse().unwrap();
        crate::scene::Opts::new(INFO.options, table)
    }

    #[test]
    fn muertos_comes_on_its_days() {
        let o = opts(30.0);
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for (fiesta, on) in [
                (Fiesta::of_day(11, 2), true),
                (Fiesta::of_day(10, 28), true),
                (Fiesta::NONE, false),
            ] {
                let ctx = Ctx {
                    theme: &th,
                    opts: &o,
                    bg: Color::Rgb(13, 17, 23),
                    seed: 5,
                    fiesta,
                };
                let mut s = (INFO.make)(&ctx);
                s.resize(&ctx, 120, 36);
                s.retheme(&ctx);
                let (cells, dancing) = holiday(&ctx, s.as_mut(), 600);
                assert_eq!(cells > 0, on, "{theme} {fiesta:?}: {cells} cells");
                assert_eq!(dancing > 0, on, "{theme} {fiesta:?}: {dancing} frames");
            }
        }
    }

    #[test]
    fn retheme_switches_the_holiday() {
        let o = opts(30.0);
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let mut ctx = Ctx {
            theme: &th,
            opts: &o,
            bg: Color::Reset,
            seed: 9,
            fiesta: Fiesta::NONE,
        };
        let mut s = (INFO.make)(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        assert_eq!(holiday(&ctx, s.as_mut(), 30), (0, 0));
        ctx.fiesta = Fiesta::of_day(11, 1);
        s.retheme(&ctx);
        // The first strike comes soon after the holiday does.
        let (cells, dancing) = holiday(&ctx, s.as_mut(), 300);
        assert!(cells > 0 && dancing > 0, "{cells} {dancing}");
        s.resize(&ctx, 90, 30);
        assert!(holiday(&ctx, s.as_mut(), 5).0 > 0);
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert_eq!(holiday(&ctx, s.as_mut(), 600), (0, 0));
    }

    /// A rain scene under `fiesta`, 120×36, at a few seconds in.
    fn scene_on(ctx: &Ctx) -> Box<dyn Scene> {
        let mut s = (INFO.make)(ctx);
        s.resize(ctx, 120, 36);
        s.retheme(ctx);
        s
    }

    fn ctx_for<'a>(
        th: &'a crate::theme::Theme,
        o: &'a crate::scene::Opts,
        fiesta: Fiesta,
    ) -> Ctx<'a> {
        Ctx {
            theme: th,
            opts: o,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta,
        }
    }

    #[test]
    fn navidad_dresses_the_rain() {
        let o = opts(30.0);
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for (fiesta, on) in [
                (Fiesta::of_day(12, 24), true),
                (Fiesta::of_day(12, 12), true),
                (Fiesta::of_day(12, 16), true),
                (Fiesta::of_day(1, 6), true),
                (Fiesta::NONE, false),
            ] {
                let ctx = ctx_for(&th, &o, fiesta);
                let mut s = scene_on(&ctx);
                let (cells, walking) = seen(&ctx, s.as_mut(), 600, NAVIDAD);
                assert_eq!(cells > 0, on, "{theme} {fiesta:?}: {cells} cells");
                assert_eq!(walking > 0, on, "{theme} {fiesta:?}: {walking} frames");
            }
        }
    }

    #[test]
    fn navidad_strings_more_lights_as_the_posadas_climb() {
        let o = crate::scene::Opts::new(INFO.options, "lightning = 0.0".parse().unwrap());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let lights = |fiesta: Fiesta| {
            let ctx = ctx_for(&th, &o, fiesta);
            let mut s = scene_on(&ctx);
            let mut c = Canvas::new(120, 36, Color::Reset);
            s.update(&ctx, 1.0 / 30.0);
            s.draw(&ctx, &mut c);
            // The bulbs hang in the upper half; the cord is drawn with them.
            let top: Vec<_> = c.cells().chunks(120).take(18).flatten().collect();
            top.iter().filter(|cell| "●•·".contains(cell.ch)).count()
        };
        let (first, nochebuena) = (
            lights(Fiesta::of_day(12, 12)),
            lights(Fiesta::of_day(12, 24)),
        );
        assert!(first > 10, "{first} bulbs on the first day");
        assert!(nochebuena > first * 3 / 2, "{first} then {nochebuena}");
    }

    #[test]
    fn holidays_the_rain_does_not_know_leave_it_ordinary() {
        let o = opts(30.0);
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let text = |fiesta: Fiesta| {
            let ctx = ctx_for(&th, &o, fiesta);
            let mut s = scene_on(&ctx);
            let mut c = Canvas::new(120, 36, Color::Reset);
            let mut out = String::new();
            for _ in 0..400 {
                s.update(&ctx, 1.0 / 30.0);
                c.clear();
                s.draw(&ctx, &mut c);
                out += &c.to_text();
            }
            out
        };
        let ordinary = text(Fiesta::NONE);
        assert!(ordinary == text(Fiesta::of_day(2, 2)));
        assert!(ordinary == text(Fiesta::of_day(5, 3)));
        assert!(ordinary == text(Fiesta::of_day(11, 20)));
        assert!(ordinary != text(Fiesta::of_day(12, 24)));
    }

    #[test]
    fn retheme_switches_navidad() {
        let o = opts(30.0);
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let mut ctx = ctx_for(&th, &o, Fiesta::NONE);
        let mut s = scene_on(&ctx);
        assert_eq!(seen(&ctx, s.as_mut(), 30, NAVIDAD), (0, 0));
        ctx.fiesta = Fiesta::of_day(12, 20);
        s.retheme(&ctx);
        // The first strike comes soon after the holiday does.
        let (cells, walking) = seen(&ctx, s.as_mut(), 300, NAVIDAD);
        assert!(cells > 0 && walking > 0, "{cells} {walking}");
        s.resize(&ctx, 90, 30);
        assert!(seen(&ctx, s.as_mut(), 5, NAVIDAD).0 > 0);
        // Another holiday takes its place, and then none.
        ctx.fiesta = Fiesta::of_day(11, 2);
        s.retheme(&ctx);
        assert_eq!(seen(&ctx, s.as_mut(), 600, NAVIDAD).1, 0);
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert_eq!(seen(&ctx, s.as_mut(), 600, NAVIDAD), (0, 0));
    }

    #[test]
    fn patrias_dresses_the_rain() {
        let o = opts(30.0);
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for (fiesta, on) in [
                (Fiesta::of_day(9, 15), true),
                (Fiesta::of_day(9, 1), true),
                (Fiesta::of_day(9, 16), true),
                (Fiesta::NONE, false),
                (Fiesta::of_day(8, 20), false),
            ] {
                let ctx = ctx_for(&th, &o, fiesta);
                let mut s = scene_on(&ctx);
                let (cells, riding) = street(&ctx, s.as_mut(), 600);
                assert_eq!(cells > 0, on, "{theme} {fiesta:?}: {cells} cells");
                assert_eq!(riding > 0, on, "{theme} {fiesta:?}: {riding} frames");
            }
        }
    }

    #[test]
    fn patrias_builds_up_toward_the_grito() {
        let o = crate::scene::Opts::new(INFO.options, "lightning = 0.0".parse().unwrap());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let cells = |fiesta: Fiesta| {
            let ctx = ctx_for(&th, &o, fiesta);
            let mut s = scene_on(&ctx);
            street(&ctx, s.as_mut(), 2).0
        };
        let (first, grito) = (cells(Fiesta::of_day(9, 1)), cells(Fiesta::of_day(9, 15)));
        assert!(first > 100, "{first} cells on the first of September");
        assert!(grito > first * 3 / 2, "{first} then {grito}");
    }

    #[test]
    fn retheme_switches_patrias() {
        let o = opts(30.0);
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let mut ctx = ctx_for(&th, &o, Fiesta::NONE);
        let mut s = scene_on(&ctx);
        assert_eq!(street(&ctx, s.as_mut(), 30), (0, 0));
        ctx.fiesta = Fiesta::of_day(9, 12);
        s.retheme(&ctx);
        // The first strike comes soon after the holiday does.
        let (cells, riding) = street(&ctx, s.as_mut(), 300);
        assert!(cells > 0 && riding > 0, "{cells} {riding}");
        s.resize(&ctx, 90, 30);
        assert!(street(&ctx, s.as_mut(), 5).0 > 0);
        // Another holiday takes its place, and then none.
        ctx.fiesta = Fiesta::of_day(11, 2);
        s.retheme(&ctx);
        assert_eq!(street(&ctx, s.as_mut(), 600).1, 0);
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert_eq!(street(&ctx, s.as_mut(), 600), (0, 0));
    }
}
