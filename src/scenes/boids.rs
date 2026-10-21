//! boids: a flock that steers by separation, alignment and cohesion.
//!
//! Simulated in "units" of one cell width on both axes (a cell is 1×2 units,
//! a braille dot 0.5×0.5), so speeds and vision are the same in x and y.
//! Neighbors are found with a uniform grid rebuilt every step (counting
//! sort, no allocation), which keeps thousands of boids cheap.
//!
//! On Día de Muertos the flock becomes monarch butterflies migrating home to
//! Michoacán: two-cell wings that beat in bouts and glide (braille wings in
//! `dots` style), a fluttering, bobbing flight drifting southwest, and among
//! them a few souls riding home, glowing candle-gold with golden trails (more
//! as the holiday comes). The hawk becomes La Catrina, Posada's calavera in
//! her wide plumed and flowered hat, her rebozo rippling behind her in two
//! stripes (braille, like the trails), the flock scattering before her and
//! marigold petals falling from her hat; cempasúchil petals and, from
//! October 31, whole blossoms drift down through the sky.

use std::f32::consts::{PI, TAU};

use crate::color::{Color, mix_oklab, rgb};
use crate::fiesta::Kind;
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

// Día de Muertos.
const CEMPASUCHIL: u32 = 0xff8c00;
const CEMPA_LIGHT: u32 = 0xffaa1e;
const CEMPA_DARK: u32 = 0xd66200;
const ROSA: u32 = 0xe4007c;
const TERCIOPELO: u32 = 0xb00048;
const MORADO: u32 = 0x7028a0;
const TURQUESA: u32 = 0x00aab8;
const HUESO: u32 = 0xf4eee2;
const FLAMA: u32 = 0xffd678;
const FLAMA_EDGE: u32 = 0xff9628;
/// Monarch oranges, one per flock.
const MONARCHS: [u32; 4] = [0xff8c00, 0xffaa1e, 0xec7410, 0xff9c3a];
/// Monarch wings seen from above, two cells: spread, half raised, folded.
const WINGS: [[char; 2]; 3] = [['Ƹ', 'Ʒ'], ['ɛ', 'ɜ'], [')', '(']];
/// How bright each wing frame is: folded wings show their dark edges.
const WING_LIGHT: [f32; 3] = [1.0, 0.86, 0.66];
/// The same in braille dots for `style = "dots"`, 5×3 around the body.
const WING_DOTS: [[&str; 3]; 3] = [
    ["##.##", ".###.", ".#.#."],
    [".#.#.", ".###.", "..#.."],
    ["..#..", "..#..", "..#.."],
];
/// La Catrina in place of the hawk: the flowers on her hat, the hat, her
/// sugar-skull face and its jaw. The second string colors each cell:
/// `r` rosa, `c` cempasúchil, `m`/`M` hat and brim, `h` bone, `e` eyes.
const CATRINA: [(&str, &str); 4] = [
    ("    ❀✿❀    ", "    rcr    "),
    ("▁▂▄▆███▆▄▂▁", "MMmmmmmmmMM"),
    ("   (◉▾◉)   ", "   heheh   "),
    ("    ╰┴╯    ", "    hhh    "),
];
/// A petal tumbling as it falls, and whole blossoms.
const TUMBLE: [char; 4] = ['◖', '•', '◗', '•'];
const BLOSSOMS: [char; 2] = ['✾', '✿'];
/// Petals falling from La Catrina's hat at once.
const SHED: usize = 24;
/// The flutter of a butterfly's flight: up-and-down push, units/s².
const FLUTTER: f32 = 45.0;
/// Where the monarchs migrate: southwest, toward Michoacán.
const COURSE: f32 = PI - 0.35;

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

/// A cempasúchil petal (or a whole blossom) drifting down, in units.
#[derive(Clone, Copy, Default)]
struct Petal {
    x: f32,
    y: f32,
    /// Falling speed, units per second.
    fall: f32,
    /// Phase of its sway and tumble.
    phase: f32,
    /// Seconds left for a petal shed by La Catrina; ambient ones never end.
    life: f32,
    blossom: bool,
    shade: u8,
}

/// Día de Muertos: the monarchs' colors and clock, La Catrina's, the petals.
struct Muertos {
    level: f32,
    /// Holiday randomness, apart from the flock's so ordinary days keep theirs.
    rng: fastrand::Rng,
    /// Scene seconds since the holiday came, for wings and flutter.
    t: f32,
    petals: Vec<Petal>,
    shed: [Petal; SHED],
    shed_next: usize,
    shed_clock: f32,
    monarch: [Color; 4],
    wheel: [Color; HEAD_LUT],
    soul: Color,
    flame_edge: Color,
    petal: [Color; 3],
    blossom: [Color; 2],
    flowers: [Color; 2],
    hat: Color,
    brim: Color,
    bone: Color,
    eyes: Color,
    rebozo: [Color; 2],
    plumes: [Color; 2],
    /// La Catrina faces where she flies: 1 right, -1 left.
    facing: f32,
    /// Which way her rebozo streams (a unit vector), lagging her turns.
    drape: (f32, f32),
}

impl Muertos {
    fn new(ctx: &Ctx) -> Muertos {
        Muertos {
            level: ctx.fiesta.level.clamp(0.0, 1.0),
            rng: fastrand::Rng::with_seed(ctx.seed ^ 0x6d6f_6e61_7263_6173),
            t: 0.0,
            petals: Vec::new(),
            shed: [Petal::default(); SHED],
            shed_next: 0,
            shed_clock: 0.0,
            monarch: [Color::Reset; 4],
            wheel: [Color::Reset; HEAD_LUT],
            soul: Color::Reset,
            flame_edge: Color::Reset,
            petal: [Color::Reset; 3],
            blossom: [Color::Reset; 2],
            flowers: [Color::Reset; 2],
            hat: Color::Reset,
            brim: Color::Reset,
            bone: Color::Reset,
            eyes: Color::Reset,
            rebozo: [Color::Reset; 2],
            plumes: [Color::Reset; 2],
            facing: 1.0,
            drape: (-1.0, 0.0),
        }
    }

    /// Colors from the theme; the marigolds, the candlelight and the skull
    /// keep their own.
    fn retheme(&mut self, ctx: &Ctx) {
        let native = ctx.theme.native;
        for (i, &hex) in MONARCHS.iter().enumerate() {
            let mut c = ctx.pick(i, rgb(hex));
            if !native {
                // Still the theme's, warmed toward a monarch.
                c = mix_oklab(c, rgb(CEMPASUCHIL), 0.55);
            }
            self.monarch[i] = c;
        }
        let stops = [
            self.monarch[0],
            self.monarch[1],
            mix_oklab(self.monarch[1], rgb(FLAMA), 0.5),
            self.monarch[2],
        ];
        for (i, slot) in self.wheel.iter_mut().enumerate() {
            let p = i as f32 / HEAD_LUT as f32 * stops.len() as f32;
            let k = p as usize % stops.len();
            *slot = mix_oklab(stops[k], stops[(k + 1) % stops.len()], p.fract());
        }
        self.soul = rgb(FLAMA);
        self.flame_edge = rgb(FLAMA_EDGE);
        self.petal = [rgb(CEMPASUCHIL), rgb(CEMPA_LIGHT), rgb(CEMPA_DARK)];
        self.blossom = [rgb(CEMPASUCHIL), rgb(TERCIOPELO)];
        self.flowers = [rgb(ROSA), rgb(CEMPASUCHIL)];
        self.hat = ctx.pick(4, rgb(MORADO));
        self.brim = mix_oklab(self.hat, rgb(HUESO), 0.25);
        self.bone = rgb(HUESO);
        self.eyes = rgb(CEMPASUCHIL);
        self.rebozo = [ctx.pick(1, rgb(ROSA)), ctx.pick(2, rgb(TURQUESA))];
        self.plumes = [mix_oklab(self.rebozo[0], rgb(HUESO), 0.5), rgb(HUESO)];
    }

    /// Scatters the drifting petals over a field, more as the holiday comes.
    fn scatter(&mut self, fw: f32, fh: f32) {
        let cells = fw * fh * 0.5;
        let n = ((cells * 0.0045 * (0.35 + self.level)).round() as usize).min(400);
        self.petals.clear();
        for i in 0..n {
            let r = &mut self.rng;
            self.petals.push(Petal {
                x: r.f32() * fw,
                y: r.f32() * fh,
                fall: 1.6 + 1.8 * r.f32(),
                phase: r.f32() * TAU,
                life: f32::INFINITY,
                // Whole blossoms from October 31, when the angelitos come.
                blossom: self.level >= 0.6 && i % 5 == 4,
                shade: r.u8(..3),
            });
        }
        for p in &mut self.shed {
            p.life = 0.0;
        }
    }

    /// Petals drift down, swaying; ambient ones wrap around, shed ones fade.
    fn drift(&mut self, dt: f32, fw: f32, fh: f32) {
        let sway = |p: &Petal, t: f32| (t * 0.9 + p.phase).sin() * 1.4;
        let t = self.t;
        for p in self.petals.iter_mut().chain(self.shed.iter_mut()) {
            if p.life <= 0.0 {
                continue;
            }
            p.y += p.fall * dt;
            p.x += (sway(p, t) - 0.6) * dt;
            if p.life.is_finite() {
                p.life -= dt;
            } else {
                p.x = p.x.rem_euclid(fw);
                if p.y >= fh {
                    p.y -= fh + 1.0;
                }
            }
        }
    }

    /// A petal slips from La Catrina's hat at (x, y).
    fn shed(&mut self, x: f32, y: f32) {
        let r = &mut self.rng;
        self.shed[self.shed_next] = Petal {
            x: x + (r.f32() - 0.5) * 9.0,
            y: y - 1.0 + r.f32(),
            fall: 2.0 + 2.0 * r.f32(),
            phase: r.f32() * TAU,
            life: 2.5 + 1.5 * r.f32(),
            blossom: false,
            shade: r.u8(..3),
        };
        self.shed_next = (self.shed_next + 1) % SHED;
    }

    /// Wing frame of monarch `i` (an index into `WINGS`): bouts of beating,
    /// then a glide on spread wings.
    fn wing(&self, i: usize) -> usize {
        let h = hash01(i);
        if (self.t * 0.35 + h * 5.0).fract() > 0.6 {
            return 0;
        }
        let beats = 3.0 + 2.5 * hash01(i ^ 0x5555);
        match ((self.t * beats + h).fract() * 4.0) as usize {
            0 => 0,
            2 => 2,
            _ => 1,
        }
    }

    /// Is monarch `i` a soul riding home? A few, more as the holiday comes.
    fn soul(&self, i: usize) -> bool {
        hash01(i ^ 0xa5a5) < 0.03 + 0.09 * self.level
    }
}

/// A well-mixed number in [0, 1) for an index.
#[inline]
fn hash01(i: usize) -> f32 {
    let mut x = (i as u32).wrapping_mul(0x9e37_79b9) ^ 0x5bd1_e995;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2c1b_3c6d);
    x ^= x >> 12;
    (x >> 8) as f32 / (1u32 << 24) as f32
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
    /// Día de Muertos, when it is on.
    muertos: Option<Box<Muertos>>,
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
            muertos: None,
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
        // Personal space: a couple of cells, whatever the vision (wider for
        // two-cell butterflies).
        let wings = if self.muertos.is_some() { 1.3 } else { 1.0 };
        let sep_r = (SEP_RADIUS * wings).min(self.vision);
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
        // Monarchs flutter, and drift southwest on the way home (not when
        // bouncing, where it would only pile them on a wall).
        let monarchs = self.muertos.as_ref().map(|m| {
            let course = COURSE + 0.5 * (m.t * 0.06).sin();
            let pull = if wrap { 0.3 + 0.4 * m.level } else { 0.0 };
            (m.t, course.cos(), course.sin(), pull)
        });

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
            if let Some((t, cx, cy, pull)) = monarchs {
                let (x, y) = steer(cx, cy, b.vx, b.vy);
                fx += x * pull;
                fy += y * pull;
                let beat = 1.0 + 0.6 * hash01(i ^ 0x3c3c);
                fy += ((t * beat + hash01(i)) * TAU).sin() * FLUTTER;
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

    /// The color of monarch `i`: its flock's orange (or its heading's), or
    /// flickering candlelight for a soul riding home.
    fn monarch_col(&self, m: &Muertos, i: usize) -> Color {
        let b = &self.boids[i];
        if m.soul(i) {
            let flicker = 0.5 + 0.5 * (m.t * 9.0 + i as f32 * 1.7).sin();
            m.soul.lerp(m.flame_edge, flicker * 0.45)
        } else if self.by_heading {
            let a = b.vy.atan2(b.vx).rem_euclid(TAU) / TAU;
            m.wheel[((a * HEAD_LUT as f32) as usize).min(HEAD_LUT - 1)]
        } else {
            m.monarch[b.flock as usize & 3]
        }
    }

    /// Draws the trail of boid `i` (the hawk is index `boids.len()`), `dy`
    /// dots below where it flew.
    fn draw_trail(&mut self, ctx: &Ctx, i: usize, col: Color, bright: f32, dy: f32) {
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
                    let (y0, y1) = (prev.1 * 2.0 + dy, p.1 * 2.0 + dy);
                    self.br.line(prev.0 * 2.0, y0, p.0 * 2.0, y1, c);
                }
            }
            prev = p;
        }
    }

    /// La Catrina's rebozo: two stripes streaming back from her shoulders,
    /// rippling, swinging round late when she turns.
    fn draw_rebozo(&mut self, ctx: &Ctx, m: &Muertos) {
        const N: usize = 14;
        const LEN: f32 = 22.0;
        let (ax, ay) = (
            self.hawk.x.round() * 2.0 + 1.0,
            ((self.hawk.y * 0.5) as i32 + 1) as f32 * 4.0 + 2.0,
        );
        let (dx, dy) = m.drape;
        // Side by side across it, a braille cell apart so each keeps its color.
        let (nx, ny) = if dx >= 0.0 { (-dy, dx) } else { (dy, -dx) };
        let gap = 4.0 * ny.abs() + 2.0 * nx.abs();
        let at = |k: usize, o: f32| {
            let s = k as f32 / N as f32;
            let w = (s * 3.0 * PI - m.t * 5.0).sin() * (0.5 + 2.5 * s) + o;
            (ax + dx * s * LEN + nx * w, ay + dy * s * LEN + ny * w)
        };
        for k in 1..=N {
            let c = 1.0 - k as f32 / N as f32;
            for (j, &col) in m.rebozo.iter().enumerate() {
                let col = ctx.fade(col, 0.8 * (1.0 - c));
                for o in [0.0, 0.9] {
                    let o = (j as f32 - 0.5) * gap + o;
                    let (p, q) = (at(k - 1, o), at(k, o));
                    self.br.line(p.0, p.1, q.0, q.1, col);
                }
            }
        }
    }

    /// The flock as monarchs, La Catrina for the hawk, and the petals.
    fn draw_muertos(&mut self, ctx: &Ctx, c: &mut Canvas, m: &Muertos) {
        self.br.clear();
        let n = self.boids.len();
        if self.hawk_on {
            self.draw_rebozo(ctx, m);
        }
        if self.trail_len > 0 {
            // Butterflies leave no wake; the souls riding them leave a golden one.
            for i in (0..n).filter(|&i| m.soul(i)) {
                let col = self.monarch_col(m, i);
                self.draw_trail(ctx, i, col, 0.95, 0.0);
            }
        }
        if self.style == Style::Dots {
            for i in 0..n {
                let b = self.boids[i];
                let w = m.wing(i);
                let col = self.monarch_col(m, i).scale(WING_LIGHT[w]);
                let (x, y) = (
                    (b.x * 2.0).floor() as i32 - 2,
                    (b.y * 2.0).floor() as i32 - 1,
                );
                for (r, row) in WING_DOTS[w].iter().enumerate() {
                    for (k, d) in row.bytes().enumerate() {
                        if d == b'#' {
                            self.br.set(x + k as i32, y + r as i32, col);
                        }
                    }
                }
            }
        }
        self.br.blit(c);

        for p in m.petals.iter().chain(&m.shed) {
            if p.life <= 0.0 {
                continue;
            }
            let (x, y) = (p.x.floor() as i32, (p.y * 0.5).floor() as i32);
            let shade = p.shade as usize;
            if p.blossom {
                c.set(x, y, BLOSSOMS[shade & 1], m.blossom[shade & 1]);
                continue;
            }
            // Tumbling: the petal turns, showing its two faces.
            let spin = (m.t * 2.2 + p.phase) as usize;
            let mut col = m.petal[(shade + spin / 2) % 3];
            if p.life < 1.0 {
                col = ctx.fade(col, 1.0 - p.life);
            }
            c.set(x, y, TUMBLE[spin % 4], col);
        }

        if self.style == Style::Arrows {
            for i in 0..n {
                let b = &self.boids[i];
                let w = m.wing(i);
                let col = self.monarch_col(m, i).scale(WING_LIGHT[w]);
                let (x, y) = (b.x.round() as i32, (b.y * 0.5) as i32);
                let attr = if m.soul(i) { BOLD } else { 0 };
                c.set_attr(x - 1, y, WINGS[w][0], col, attr);
                c.set_attr(x, y, WINGS[w][1], col, attr);
            }
        }
        if self.hawk_on {
            self.draw_catrina(c, m);
        }
    }

    fn draw_catrina(&self, c: &mut Canvas, m: &Muertos) {
        let (x0, y0) = (
            self.hawk.x.round() as i32 - 5,
            (self.hawk.y * 0.5) as i32 - 2,
        );
        for (r, (art, keys)) in CATRINA.iter().enumerate() {
            for (k, (ch, key)) in art.chars().zip(keys.chars()).enumerate() {
                if ch == ' ' {
                    continue;
                }
                let col = match key {
                    'r' => m.flowers[0],
                    'c' => m.flowers[1],
                    'm' => m.hat,
                    'M' => m.brim,
                    'e' => m.eyes,
                    _ => m.bone,
                };
                let attr = if r >= 2 { BOLD } else { 0 };
                c.set_attr(x0 + k as i32, y0 + r as i32, ch, col, attr);
            }
        }
        // Two plumes on her hat, behind the flowers.
        let px = if m.facing > 0.0 { x0 + 2 } else { x0 + 7 };
        for (j, &col) in m.plumes.iter().enumerate() {
            c.set(px + j as i32, y0, 'ϡ', col);
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
        if let Some(m) = &mut self.muertos {
            m.scatter(fw, fh);
        }
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
        if !ctx.fiesta.is(Kind::Muertos) {
            self.muertos = None;
            return;
        }
        let level = ctx.fiesta.level.clamp(0.0, 1.0);
        if self.muertos.as_ref().is_none_or(|m| m.level != level) {
            let mut m = Box::new(Muertos::new(ctx));
            m.scatter(self.fw, self.fh);
            self.muertos = Some(m);
        }
        if let Some(m) = &mut self.muertos {
            m.retheme(ctx);
        }
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
            if let Some(m) = &mut self.muertos {
                m.t += h;
            }
            self.trail_clock += h;
            if self.trail_clock >= 1.0 / TRAIL_HZ {
                self.trail_clock %= 1.0 / TRAIL_HZ;
                self.record_trails();
            }
        }
        if let Some(m) = &mut self.muertos {
            m.drift(dt, self.fw, self.fh);
            if self.hawk_on {
                // She turns round only when really heading the other way.
                let h = &self.hawk;
                let s = (h.vx * h.vx + h.vy * h.vy).sqrt();
                if h.vx * m.facing < -0.3 * s {
                    m.facing = -m.facing;
                }
                if s > 1e-4 {
                    let k = (dt * 2.5).min(1.0);
                    let (dx, dy) = m.drape;
                    let (dx, dy) = (dx + (-h.vx / s - dx) * k, dy + (-h.vy / s - dy) * k);
                    let l = (dx * dx + dy * dy).sqrt();
                    if l > 1e-3 {
                        m.drape = (dx / l, dy / l);
                    }
                }
                // Petals slip from her hat as she flies, more on the great days.
                let every = 0.32 - 0.14 * m.level;
                m.shed_clock += dt;
                while m.shed_clock >= every {
                    m.shed_clock -= every;
                    m.shed(self.hawk.x, self.hawk.y - 2.0);
                }
            }
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        if let Some(m) = self.muertos.take() {
            self.draw_muertos(ctx, c, &m);
            self.muertos = Some(m);
            return;
        }
        self.br.clear();
        let n = self.boids.len();
        if self.trail_len > 0 {
            for i in 0..n {
                let col = self.color_of(&self.boids[i]);
                let bright = if self.style == Style::Dots { 0.9 } else { 0.75 };
                self.draw_trail(ctx, i, col, bright, 0.0);
            }
            if self.hawk_on {
                self.draw_trail(ctx, n, self.hawk_col, 0.8, 0.0);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;

    fn ctx<'a>(th: &'a crate::theme::Theme, o: &'a crate::scene::Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme: th,
            opts: o,
            bg: Color::Rgb(13, 17, 23),
            seed: 7,
            fiesta,
        }
    }

    fn opts(s: &str) -> crate::scene::Opts {
        crate::scene::Opts::new(INFO.options, s.parse().unwrap())
    }

    /// Runs three seconds and returns the last frame.
    fn frame(s: &mut dyn Scene, ctx: &Ctx, w: u16, h: u16) -> Canvas {
        let mut c = Canvas::new(w, h, Color::Reset);
        for _ in 0..90 {
            s.update(ctx, 1.0 / 30.0);
        }
        s.draw(ctx, &mut c);
        c
    }

    fn has(c: &Canvas, chars: &[char]) -> bool {
        c.cells().iter().any(|cell| chars.contains(&cell.ch))
    }

    const MONARCH: [char; 6] = ['Ƹ', 'Ʒ', 'ɛ', 'ɜ', ')', '('];

    #[test]
    fn monarchs_and_la_catrina_come_for_the_holiday() {
        let th = crate::theme::Theme::classic();
        let o = opts("");
        for fiesta in [Fiesta::of_day(11, 2), Fiesta::of_day(10, 28)] {
            let ctx = ctx(&th, &o, fiesta);
            let mut s = (INFO.make)(&ctx);
            s.resize(&ctx, 120, 36);
            s.retheme(&ctx);
            let c = frame(s.as_mut(), &ctx, 120, 36);
            assert!(has(&c, &MONARCH), "{fiesta:?}: no monarchs");
            assert!(has(&c, &['◉', '▾', '❀']), "{fiesta:?}: no Catrina");
            assert!(has(&c, &TUMBLE), "{fiesta:?}: no petals");
            assert!(!has(&c, &ARROWS) && !has(&c, &HAWK), "{fiesta:?}");
        }
        let ctx = ctx(&th, &o, Fiesta::NONE);
        let mut s = (INFO.make)(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let c = frame(s.as_mut(), &ctx, 120, 36);
        assert!(has(&c, &ARROWS) && has(&c, &HAWK));
        assert!(!has(
            &c,
            &['Ƹ', 'Ʒ', 'ɛ', 'ɜ', '◉', '❀', '✾', '◖', '◗', 'ϡ']
        ));
    }

    #[test]
    fn the_holiday_comes_and_goes_through_retheme() {
        let th = crate::theme::Theme::classic();
        let o = opts("style = 'dots'");
        let plain = ctx(&th, &o, Fiesta::NONE);
        let fiesta = ctx(&th, &o, Fiesta::of_day(11, 1));
        let mut s = Boids::new(&plain);
        s.resize(&plain, 80, 24);
        s.retheme(&plain);
        assert!(s.muertos.is_none());
        s.retheme(&fiesta);
        assert!(s.muertos.as_ref().is_some_and(|m| !m.petals.is_empty()));
        let c = frame(&mut s, &fiesta, 80, 24);
        assert!(has(&c, &['◉']));
        s.retheme(&plain);
        assert!(s.muertos.is_none());
        let c = frame(&mut s, &plain, 80, 24);
        assert!(!has(&c, &['◉', '❀']));
        // A new day of the holiday brings more petals.
        s.retheme(&ctx(&th, &o, Fiesta::of_day(10, 28)));
        let few = s.muertos.as_ref().map_or(0, |m| m.petals.len());
        s.retheme(&ctx(&th, &o, Fiesta::of_day(11, 2)));
        let many = s.muertos.as_ref().map_or(0, |m| m.petals.len());
        assert!(few < many, "{few} {many}");
    }

    #[test]
    fn the_holiday_survives_odd_sizes_and_steps() {
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for o in [
                "",
                "style = 'dots'\nbounce = true",
                "predator = false\ntrails = 0\ncolor = 'heading'",
                "count = 0\nvision = 1.0\nspeed = 0.0",
                "count = 400\nspeed = 20.0",
            ] {
                let o = opts(o);
                let ctx = ctx(&th, &o, Fiesta::of_day(11, 2));
                let mut s = (INFO.make)(&ctx);
                for &(w, h) in &[(1u16, 1u16), (2, 3), (300, 90), (3, 1), (1, 40), (37, 11)] {
                    s.resize(&ctx, w, h);
                    s.retheme(&ctx);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for i in 0..20 {
                        s.update(&ctx, if i % 10 == 9 { 5.0 } else { 1.0 / 30.0 });
                        c.clear();
                        s.draw(&ctx, &mut c);
                        assert!(c.cells().iter().all(|cell| cell.bg == Color::Reset));
                    }
                }
            }
        }
    }
}
