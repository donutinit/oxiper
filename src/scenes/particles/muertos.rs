//! Día de Muertos in the particles: the flow carries cempasúchil petals and
//! monarch butterflies.
//!
//! Marigold (and a few terciopelo) blossoms open in the current, drift with
//! it and shed their petals into it, then let go of them all at once and
//! bloom again elsewhere. The petals tumble along the streams, two shades
//! taking turns, each leaving a short wake in the trail buffer. Monarchs
//! migrate across on top of it all, riding the flow and flapping, now and
//! then gliding with their wings open. In the fireworks some shells burst
//! into petals that flutter down, and the fountain springs from marigolds
//! and tosses petals with its water. How much of it there is follows
//! `fiesta.level`, and the monarchs come in numbers on November 1 and 2.
//!
//! Everything here draws from its own RNG, so an ordinary day's particles
//! are never disturbed.

use std::f32::consts::TAU;

use super::{Dots, Field, Mode};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::Braille;
use crate::scene::Ctx;

/// The flow (and fountain) ramp under `classic`: morado through rosa
/// mexicano and cempasúchil to amarillo.
pub(super) const RAMP: [Color; 6] = [
    rgb(0x00aab8),
    rgb(0x2f5fc0),
    rgb(0x7028a0),
    rgb(0xa8208c),
    rgb(0xe4007c),
    rgb(0xff5a96),
];

/// Shell colors under `classic`: the papel picado and the ofrenda flowers.
pub(super) const SHELLS: [Color; 8] = [
    rgb(0xe4007c),
    rgb(0xff8c00),
    rgb(0xffd200),
    rgb(0x7028a0),
    rgb(0x00aab8),
    rgb(0x8cc828),
    rgb(0xb00048),
    rgb(0xffaa1e),
];

/// Petal wakes, appended to the trail palette: cempasúchil, terciopelo.
pub(super) const WAKES: [Color; 2] = [rgb(0xff9a14), rgb(0xd0006a)];

/// Petal shades (light, mid, dark) of a marigold and of terciopelo.
const PETALS: [[Color; 3]; 2] = [
    [rgb(0xffbe32), rgb(0xff8c00), rgb(0xd66200)],
    [rgb(0xff3c9a), rgb(0xe4007c), rgb(0xb00048)],
];

/// Blossom colors from the heart out to the rim.
const BLOOMS: [[Color; 3]; 2] = [
    [rgb(0xffd200), rgb(0xffa014), rgb(0xe06a00)],
    [rgb(0xff8cc6), rgb(0xff3c9a), rgb(0xc0005a)],
];

/// Monarch wings, a little different each.
const WINGS: [Color; 3] = [rgb(0xff8200), rgb(0xff9a10), rgb(0xf07000)];

/// A monarch seen from above, wings open, half raised and closed (one
/// character per braille dot; the black of its veins and body is the gaps).
const MONARCH: [[&str; 8]; 3] = [
    [
        "####....####",
        "#####..#####",
        ".#.##..##.#.",
        "..###..###..",
        "..###..###..",
        ".####..####.",
        ".##.#..#.##.",
        "..##....##..",
    ],
    [
        "..##....##..",
        "..###..###..",
        "...##..##...",
        "...##..##...",
        "...##..##...",
        "..###..###..",
        "...#....#...",
        "............",
    ],
    [
        ".....##.....",
        ".....##.....",
        ".....##.....",
        ".....##.....",
        ".....##.....",
        ".....##.....",
        "............",
        "............",
    ],
];

/// A monarch close by, on screens tall enough for it.
const LARGE: [[&str; 12]; 3] = [
    [
        "###..........###",
        "#####......#####",
        "#######..#######",
        ".#.####..####.#.",
        ".##.###..###.##.",
        "..##.##..##.##..",
        "...####..####...",
        "..#####..#####..",
        ".###.##..##.###.",
        ".##.###..###.##.",
        "..####....####..",
        "...##......##...",
    ],
    [
        "................",
        "...##......##...",
        "...###....###...",
        "....###..###....",
        "....##....##....",
        "....###..###....",
        "....##....##....",
        "...###....###...",
        "...##......##...",
        "....#......#....",
        "................",
        "................",
    ],
    [
        "................",
        ".......##.......",
        ".......##.......",
        ".......##.......",
        ".......##.......",
        ".......##.......",
        ".......##.......",
        ".......##.......",
        "................",
        "................",
        "................",
        "................",
    ],
];

/// Seconds a loose petal takes to fade in.
const FADE_IN: f32 = 0.5;

/// Wing beat: open, half, closed, half.
const BEAT: [usize; 4] = [0, 1, 2, 1];

/// Petal shapes as it tumbles (dot offsets; face on, tilting, edge on).
const TUMBLE: [&[(i32, i32)]; 4] = [
    &[(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)],
    &[(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)],
    &[(0, 0), (1, 1), (2, 2)],
    &[(1, 0), (2, 0), (0, 1), (1, 1), (0, 2)],
];

#[derive(Clone, Copy)]
struct Petal {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    spin: f32,
    spin_rate: f32,
    age: f32,
    life: f32,
    /// 0 cempasúchil, 1 terciopelo.
    kind: u8,
    /// Falls (fireworks, fountain) instead of riding the flow.
    falls: bool,
}

#[derive(Clone, Copy)]
struct Blossom {
    x: f32,
    y: f32,
    /// Radius when open, in dots.
    size: f32,
    /// Negative while waiting to bloom.
    age: f32,
    life: f32,
    rot: f32,
    kind: u8,
    /// Has let go of its petals.
    scattered: bool,
}

#[derive(Clone, Copy)]
struct Monarch {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Wing beat phase (radians).
    beat: f32,
    rate: f32,
    /// Seconds left gliding with the wings open.
    glide: f32,
    large: bool,
    tone: u8,
}

/// A blossom cut into braille cells (column, row, dots lit, tone), kept
/// until it has moved, opened or turned enough to look different.
#[derive(Default)]
struct Cut {
    key: Option<(i32, i32, u32, i32)>,
    cells: Vec<(i32, i32, u8, u8)>,
}

pub(super) struct Muertos {
    fiesta: Fiesta,
    rng: fastrand::Rng,
    mode: Mode,
    w: usize,
    h: usize,
    pw: f32,
    ph: f32,
    jets: usize,
    petals: Vec<Petal>,
    /// How many petals ride the flow.
    petal_target: usize,
    blossoms: Vec<Blossom>,
    /// How each blossom was last cut (one per blossom).
    cuts: Vec<Cut>,
    monarchs: Vec<Monarch>,
    /// The way the monarchs migrate (radians), turning slowly.
    heading: f32,
    toss_acc: f32,
    t: f32,
    /// First trail palette slot of the petal wakes.
    pub slot: u8,
    petal_cols: [[Color; 3]; 2],
    bloom_cols: [[Color; 3]; 2],
    wing_cols: [Color; 3],
}

impl Muertos {
    pub fn new(seed: u64) -> Muertos {
        Muertos {
            fiesta: Fiesta::NONE,
            rng: fastrand::Rng::with_seed(seed ^ 0xD1A0_DE00_0EB0_1102),
            mode: Mode::Flow,
            w: 0,
            h: 0,
            pw: 0.0,
            ph: 0.0,
            jets: 1,
            petals: Vec::new(),
            petal_target: 0,
            blossoms: Vec::new(),
            cuts: Vec::new(),
            monarchs: Vec::new(),
            heading: 0.0,
            toss_acc: 0.0,
            t: 0.0,
            slot: 0,
            petal_cols: PETALS,
            bloom_cols: BLOOMS,
            wing_cols: WINGS,
        }
    }

    pub fn on(&self) -> bool {
        self.fiesta.on()
    }

    pub fn fiesta(&self) -> Fiesta {
        self.fiesta
    }

    /// Dresses the screen for the holiday (or undresses it): called on every
    /// resize and whenever the holiday changes.
    pub fn dress(&mut self, fiesta: Fiesta, mode: Mode, w: usize, h: usize, jets: usize) {
        self.fiesta = fiesta;
        self.mode = mode;
        self.petals.clear();
        self.blossoms.clear();
        self.cuts.clear();
        self.monarchs.clear();
        self.toss_acc = 0.0;
        if !fiesta.on() || w == 0 || h == 0 {
            return;
        }
        let level = fiesta.level.clamp(0.0, 1.0);
        (self.w, self.h) = (w, h);
        (self.pw, self.ph) = (w as f32 * 2.0, h as f32 * 4.0);
        self.jets = jets.max(1);
        // Counts per 100×30 cells, scaled by the screen area.
        let area = (w * h) as f32 / 3000.0;
        let count = |n: f32| ((n * area).round() as usize).min(4000);
        self.heading = if self.rng.bool() { 0.0 } else { TAU / 2.0 } + self.rng.f32() - 0.5;

        // Fewer in the night of the fireworks.
        let few = match mode {
            Mode::Flow => 1.0,
            Mode::Fountain => 0.7,
            Mode::Fireworks => 0.4,
        };
        let monarchs = count((1.5 + 6.5 * level) * few).max(1);
        self.monarchs.reserve_exact(monarchs);
        for _ in 0..monarchs {
            let (x, y) = (self.rng.f32() * self.pw, self.rng.f32() * self.ph);
            let m = self.new_monarch(x, y);
            self.monarchs.push(m);
        }

        match mode {
            Mode::Flow => {
                let blossoms = count(1.0 + 2.0 * level).max(1);
                self.blossoms.reserve_exact(blossoms);
                for i in 0..blossoms {
                    let mut b = self.new_blossom();
                    // The first ones are already open; the rest come one by one.
                    b.age = if i * 2 < blossoms + 1 {
                        self.rng.f32() * b.life * 0.6
                    } else {
                        -self.rng.f32() * 6.0
                    };
                    self.blossoms.push(b);
                }
                self.petal_target = count(24.0 + 66.0 * level);
                // Room for the petals a blossom lets go of all at once.
                self.petals
                    .reserve_exact(self.petal_target + count(24.0).max(12));
                for _ in 0..self.petal_target {
                    let mut p = self.loose_petal();
                    p.age = self.rng.f32() * p.life;
                    self.petals.push(p);
                }
            }
            Mode::Fountain => {
                // The jets spring from marigolds.
                self.blossoms.reserve_exact(self.jets);
                for j in 0..self.jets {
                    let mut b = self.new_blossom();
                    b.size = (self.ph * 0.07).clamp(3.0, 9.0);
                    b.x = self.pw * (j as f32 + 0.5) / self.jets as f32;
                    b.y = self.ph - b.size * 0.6;
                    b.age = 3.0;
                    b.life = f32::INFINITY;
                    b.kind = 0;
                    self.blossoms.push(b);
                }
                self.petal_target = 0;
                self.petals.reserve_exact(count(30.0 + 70.0 * level).max(8));
            }
            Mode::Fireworks => {
                self.petal_target = 0;
                self.petals
                    .reserve_exact(count(60.0 + 120.0 * level).max(16));
            }
        }
        // Room for the cells of the biggest blossom, so cutting never allocates.
        self.cuts.clear();
        self.cuts.resize_with(self.blossoms.len(), || Cut {
            key: None,
            cells: Vec::with_capacity(224),
        });
    }

    /// Caches the holiday's colors. Under other themes the trails and shells
    /// take the theme's colors; the petals, blossoms and monarchs keep their
    /// own, a little drawn toward it.
    pub fn retheme(&mut self, ctx: &Ctx, slot: usize) {
        self.slot = slot.min(250) as u8;
        let tint = |c: Color, i: usize| {
            if ctx.theme.native {
                c
            } else {
                c.lerp(ctx.pick(i, c), 0.2)
            }
        };
        for k in 0..2 {
            for s in 0..3 {
                self.petal_cols[k][s] = tint(PETALS[k][s], k * 3 + s);
                self.bloom_cols[k][s] = tint(BLOOMS[k][s], k * 3 + s);
            }
        }
        for (i, c) in self.wing_cols.iter_mut().enumerate() {
            *c = tint(WINGS[i], i);
        }
    }

    fn new_monarch(&mut self, x: f32, y: f32) -> Monarch {
        Monarch {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            beat: self.rng.f32() * TAU,
            rate: 9.0 + self.rng.f32() * 5.0,
            glide: 0.0,
            large: self.h >= 30 && self.mode != Mode::Fireworks && self.rng.f32() < 0.4,
            tone: self.rng.u8(0..WINGS.len() as u8),
        }
    }

    fn new_blossom(&mut self) -> Blossom {
        let big = (self.ph * 0.075).clamp(3.0, 14.0);
        let size = big * (0.75 + self.rng.f32() * 0.45);
        let m = size + 2.0;
        Blossom {
            x: m + self.rng.f32() * (self.pw - 2.0 * m).max(0.0),
            y: m + self.rng.f32() * (self.ph - 2.0 * m).max(0.0),
            size,
            age: 0.0,
            life: 12.0 + self.rng.f32() * 10.0,
            rot: self.rng.f32() * TAU,
            kind: (self.rng.f32() < 0.2) as u8,
            scattered: false,
        }
    }

    /// A petal somewhere on the screen, fading in.
    fn loose_petal(&mut self) -> Petal {
        let (x, y) = (self.rng.f32() * self.pw, self.rng.f32() * self.ph);
        let kind = (self.rng.f32() < 0.2) as u8;
        let mut p = self.petal(x, y, 0.0, 0.0, kind, false);
        p.age = 0.0;
        p
    }

    fn petal(&mut self, x: f32, y: f32, vx: f32, vy: f32, kind: u8, falls: bool) -> Petal {
        Petal {
            x,
            y,
            vx,
            vy,
            spin: self.rng.f32() * TAU,
            spin_rate: (3.0 + self.rng.f32() * 6.0) * if self.rng.bool() { 1.0 } else { -1.0 },
            // Cast off by something: bright from the start (only the loose
            // ones fade in).
            age: FADE_IN,
            life: if falls {
                9.0 + self.rng.f32() * 6.0
            } else {
                6.0 + self.rng.f32() * 9.0
            },
            kind,
            falls,
        }
    }

    /// A petal leaving a blossom's rim, or a loose one if none is open.
    fn shed(&mut self) -> Petal {
        let n = self.blossoms.len();
        if n > 0 {
            let b = self.blossoms[self.rng.usize(0..n)];
            if b.age > 1.0 && !b.scattered && b.age < b.life {
                let a = self.rng.f32() * TAU;
                let (c, s) = (a.cos(), a.sin());
                let r = b.size * 0.9;
                let kind = if self.rng.f32() < 0.1 {
                    1 - b.kind
                } else {
                    b.kind
                };
                return self.petal(b.x + c * r, b.y + s * r, c * 4.0, s * 4.0, kind, false);
            }
        }
        self.loose_petal()
    }

    /// Fireworks: some shells burst into petals as well.
    pub fn burst(&mut self, x: f32, y: f32, radius: f32) {
        if !self.on() || self.rng.f32() > 0.3 + 0.35 * self.fiesta.level {
            return;
        }
        let kind = (self.rng.f32() < 0.3) as u8;
        let n = 6 + self.rng.usize(0..8) + (self.fiesta.level * 8.0) as usize;
        for _ in 0..n {
            if self.petals.len() >= self.petals.capacity() {
                break;
            }
            let a = self.rng.f32() * TAU;
            let v = radius * (0.6 + 0.9 * self.rng.f32());
            let p = self.petal(x, y, a.cos() * v, a.sin() * v * 0.8, kind, true);
            self.petals.push(p);
        }
    }

    pub fn update(&mut self, dt: f32, field: &Field, dots: &mut Dots, now: u32, span: u32, v: f32) {
        if !self.on() || self.pw <= 0.0 {
            return;
        }
        self.t += dt;
        self.heading += (self.t * 0.13).sin() * 0.05 * dt;
        match self.mode {
            Mode::Flow => self.update_blossoms(dt, field, v),
            Mode::Fountain => self.toss(dt),
            Mode::Fireworks => {}
        }
        self.update_petals(dt, field, dots, now, span, v);
        self.update_monarchs(dt, field, v);
    }

    fn update_blossoms(&mut self, dt: f32, field: &Field, v: f32) {
        for i in 0..self.blossoms.len() {
            let mut b = self.blossoms[i];
            b.age += dt;
            if b.age >= b.life {
                let mut nb = self.new_blossom();
                nb.age = -0.5 - self.rng.f32() * 2.0;
                self.blossoms[i] = nb;
                continue;
            }
            b.rot += dt * 0.25;
            if b.age > 0.0 {
                // Drift with the current, much slower than the petals.
                let (fx, fy) = field.sample(b.x, b.y);
                let m = b.size + 1.0;
                b.x = (b.x + fx * v * 0.12 * dt).clamp(m, (self.pw - m).max(m));
                b.y = (b.y + fy * v * 0.12 * dt).clamp(m, (self.ph - m).max(m));
            }
            // Near its end it lets go of its petals all at once.
            if !b.scattered && b.age > b.life - 1.6 {
                b.scattered = true;
                let n = (b.size * 2.2) as usize;
                for k in 0..n {
                    if self.petals.len() >= self.petals.capacity() {
                        break;
                    }
                    let a = (k as f32 + self.rng.f32()) / n as f32 * TAU;
                    let (c, s) = (a.cos(), a.sin());
                    let r = b.size * (0.3 + 0.6 * self.rng.f32());
                    let sp = 6.0 + self.rng.f32() * 8.0;
                    let kind = if self.rng.f32() < 0.1 {
                        1 - b.kind
                    } else {
                        b.kind
                    };
                    let mut p = self.petal(b.x + c * r, b.y + s * r, c * sp, s * sp, kind, false);
                    p.life *= 0.6;
                    self.petals.push(p);
                }
            }
            self.blossoms[i] = b;
        }
    }

    /// Fountain: the jets toss petals with the water.
    fn toss(&mut self, dt: f32) {
        let level = self.fiesta.level;
        self.toss_acc += dt * (1.5 + 4.0 * level) * self.jets as f32;
        while self.toss_acc >= 1.0 {
            self.toss_acc -= 1.0;
            if self.petals.len() >= self.petals.capacity() {
                continue;
            }
            let jet = self.rng.usize(0..self.jets);
            let x = self.pw * (jet as f32 + 0.5) / self.jets as f32;
            let v = (2.0 * 1.1 * self.ph * self.ph * (0.5 + 0.3 * self.rng.f32())).sqrt();
            let a = (self.rng.f32() - 0.5) * 0.7;
            let kind = (self.rng.f32() < 0.15) as u8;
            let p = self.petal(x, self.ph - 2.0, a.sin() * v, -a.cos() * v, kind, true);
            self.petals.push(p);
        }
    }

    fn update_petals(
        &mut self,
        dt: f32,
        field: &Field,
        dots: &mut Dots,
        now: u32,
        span: u32,
        v: f32,
    ) {
        let (pw, ph) = (self.pw, self.ph);
        let follow = 1.0 - (-2.5 * dt).exp();
        let wake = now.wrapping_sub((span as f32 * 0.05) as u32);
        // Falling petals: gravity like the fountain's water at first, then
        // the air holds them and they sway down.
        let g = ph * 1.1;
        let fall = (ph * 0.06).max(2.0);
        let mut i = 0;
        while i < self.petals.len() {
            let mut p = self.petals[i];
            p.age += dt;
            p.spin += p.spin_rate * dt;
            let (ox, oy) = (p.x, p.y);
            if p.falls {
                let air = (p.age * 1.4).min(1.0);
                p.vy += g * dt * (1.0 - air);
                let drag = 1.0 - (-(0.6 + 2.5 * air) * dt).exp();
                p.vy += (fall - p.vy) * drag * air;
                p.vx -= p.vx * drag;
                p.vx += (p.spin * 0.35).cos() * fall * 1.6 * air * dt;
            } else {
                let (fx, fy) = field.sample(p.x, p.y);
                p.vx += (fx * v * 0.85 - p.vx) * follow;
                p.vy += (fy * v * 0.85 - p.vy) * follow;
            }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            let gone = p.age >= p.life
                || p.x < -4.0
                || p.x > pw + 4.0
                || p.y > ph + 4.0
                || (!p.falls && p.y < -4.0);
            if gone {
                if p.falls || self.petals.len() > self.petal_target {
                    self.petals.swap_remove(i);
                } else {
                    self.petals[i] = self.shed();
                    i += 1;
                }
                continue;
            }
            dots.line(ox, oy, p.x, p.y, wake, self.slot + p.kind);
            self.petals[i] = p;
            i += 1;
        }
    }

    fn update_monarchs(&mut self, dt: f32, field: &Field, v: f32) {
        let (pw, ph) = (self.pw, self.ph);
        let follow = 1.0 - (-1.2 * dt).exp();
        let (hx, hy) = (self.heading.cos(), self.heading.sin() * 0.3);
        let cruise = (pw * 0.02).clamp(5.0, 12.0);
        let m = 8.0;
        for i in 0..self.monarchs.len() {
            let mut b = self.monarchs[i];
            let k = if b.large { 1.15 } else { 1.0 };
            let (fx, fy) = field.sample(b.x.clamp(0.0, pw), b.y.clamp(0.0, ph));
            // Wings beating lift it a little; gliding it sinks.
            let lift = if b.glide > 0.0 {
                2.0
            } else {
                -b.beat.sin() * 6.0
            };
            b.vx += (fx * v * 0.45 + hx * cruise * k - b.vx) * follow;
            b.vy += (fy * v * 0.35 + hy * cruise * k + lift - b.vy) * follow;
            b.x += b.vx * dt;
            b.y += b.vy * dt;
            if b.glide > 0.0 {
                b.glide -= dt;
            } else {
                let before = (b.beat / TAU) as i32;
                b.beat += b.rate * dt;
                if (b.beat / TAU) as i32 != before {
                    b.beat %= TAU;
                    if self.rng.f32() < 0.12 {
                        b.glide = 0.4 + self.rng.f32() * 1.0;
                    }
                }
            }
            // Out one side, back in from the other.
            if b.x < -m || b.x > pw + m || b.y < -m || b.y > ph + m {
                let (x, y) = if b.x < -m || b.x > pw + m {
                    (if b.x < 0.0 { pw + m } else { -m }, self.rng.f32() * ph)
                } else {
                    (self.rng.f32() * pw, if b.y < 0.0 { ph + m } else { -m })
                };
                b = self.new_monarch(x, y);
                b.vx = hx * cruise;
            }
            self.monarchs[i] = b;
        }
    }

    pub fn draw(&mut self, ctx: &Ctx, br: &mut Braille) {
        if !self.on() {
            return;
        }
        for (b, cut) in self.blossoms.iter().zip(&mut self.cuts) {
            draw_blossom(b, cut, &self.bloom_cols[b.kind as usize], br);
        }
        for p in &self.petals {
            let f = (turns(p.spin) * 4.0) as usize % 4;
            let shade = [0, 1, 2, 1][f];
            let fade = (p.age / FADE_IN)
                .min((p.life - p.age) / 1.2)
                .clamp(0.0, 1.0);
            let c = ctx.fade(self.petal_cols[p.kind as usize][shade], 1.0 - fade);
            let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
            if fade > 0.6 {
                // Each cell the petal touches (at most 2×2), emptied once.
                let mut done = 0u8;
                for &(dx, dy) in TUMBLE[f] {
                    let (cx, cy) = (((x + dx) >> 1) - (x >> 1), ((y + dy) >> 2) - (y >> 2));
                    if done & (1 << (cy * 2 + cx)) == 0 {
                        done |= 1 << (cy * 2 + cx);
                        clear_cell(br, x + dx, y + dy);
                    }
                }
            }
            for &(dx, dy) in TUMBLE[f] {
                br.set(x + dx, y + dy, c);
            }
        }
        for b in &self.monarchs {
            let frame = if b.glide > 0.0 {
                0
            } else {
                BEAT[(turns(b.beat) * 4.0) as usize % 4]
            };
            let c = self.wing_cols[b.tone as usize % WINGS.len()];
            if b.large {
                sprite(br, &LARGE[frame], b.x, b.y, c);
            } else {
                sprite(br, &MONARCH[frame], b.x, b.y, c);
            }
        }
    }
}

/// A ruffled pompom: scalloped rim, rings of petals, gold at the heart.
fn draw_blossom(b: &Blossom, cut: &mut Cut, cols: &[Color; 3], br: &mut Braille) {
    if b.age <= 0.0 {
        return;
    }
    let open = (b.age / 1.8).min(1.0);
    let close = ((b.life - b.age) / 1.6).clamp(0.0, 1.0);
    let r = b.size * (1.0 - (1.0 - open).powi(3)) * close.sqrt();
    if r < 0.8 {
        return;
    }
    let (cx, cy) = (b.x.round() as i32, b.y.round() as i32);
    // It turns slowly: cutting it again every few frames looks the same.
    let turn = (b.rot * 25.0) as i32;
    let key = (cx, cy, r.to_bits(), turn);
    if cut.key != Some(key) {
        cut.key = Some(key);
        cut_blossom(cut, r, (cx, cy), turn as f32 / 25.0, (br.pw(), br.ph()));
    }
    for &(gx, gy, mask, tone) in &cut.cells {
        clear_cell(br, gx * 2, gy * 4);
        let c = cols[tone as usize];
        for k in 0..8 {
            if mask & (1 << k) != 0 {
                br.set(gx * 2 + (k & 1), gy * 4 + (k >> 1), c);
            }
        }
    }
}

/// Cuts a blossom of radius `r` dots centered on `(cx, cy)` into cells of a
/// screen `(pw, ph)` dots big.
fn cut_blossom(cut: &mut Cut, r: f32, (cx, cy): (i32, i32), rot: f32, (pw, ph): (i32, i32)) {
    cut.cells.clear();
    let reach = r.ceil() as i32 + 1;
    let (x0, x1) = ((cx - reach).max(0), (cx + reach).min(pw - 1));
    let (y0, y1) = ((cy - reach).max(0), (cy + reach).min(ph - 1));
    // A rosette: rings of rounded petals around a golden heart, each
    // ring turning the other way and set between the petals of the last.
    let rings = if r >= 7.0 { 3 } else { 2 };
    let lit = |x: i32, y: i32| {
        let (dx, dy) = ((x - cx) as f32, (y - cy) as f32);
        let d = (dx * dx + dy * dy).sqrt();
        if d > r + 0.5 {
            return None;
        }
        if d < r * 0.22 {
            return Some(true);
        }
        let u = d / r * rings as f32;
        let k = (u as usize).min(rings - 1);
        let n = (5 + 4 * k) as f32;
        let turn = if k % 2 == 1 {
            rot + 0.5 * TAU / n
        } else {
            -rot
        };
        let t = fract((atan2(dy, dx) - turn) * (n / TAU));
        // 0 in the middle of a petal, 1 at the notch between two.
        let off = (t - 0.5).abs() * 2.0;
        if u > (k + 1) as f32 - 0.45 * off * off {
            return if k == rings - 1 { None } else { Some(false) };
        }
        Some(off < 0.86 || u < k as f32 + 0.3)
    };
    let far = (r + 2.2) * (r + 2.2);
    for gy in (y0 >> 2)..=(y1 >> 2) {
        for gx in (x0 >> 1)..=(x1 >> 1) {
            // Cells in the corners of the box are nowhere near the rim.
            let (fx, fy) = ((gx * 2 - cx) as f32 + 0.5, (gy * 4 - cy) as f32 + 1.5);
            if fx * fx + fy * fy > far {
                continue;
            }
            let (mut covered, mut mask) = (false, 0u8);
            for k in 0..8 {
                if let Some(on) = lit(gx * 2 + (k & 1), gy * 4 + (k >> 1)) {
                    covered = true;
                    mask |= (on as u8) << k;
                }
            }
            if !covered {
                continue;
            }
            // Colored by where the cell's middle is, so cells stay whole.
            let (mx, my) = (gx * 2 + 1 - cx, gy * 4 + 2 - cy);
            let d = ((mx * mx + my * my) as f32).sqrt() / r;
            let tone = if d < 0.4 {
                0
            } else if d < 0.78 {
                1
            } else {
                2
            };
            cut.cells.push((gx, gy, mask, tone));
        }
    }
}

/// The fractional part, always in 0..1.
fn fract(x: f32) -> f32 {
    x - x.floor()
}

/// How far around the circle an angle is, in turns (0..1).
fn turns(a: f32) -> f32 {
    fract(a / TAU)
}

/// The angle of (x, y) in radians, to within 0.0003: all the
/// blossoms need to cut their petals, for a fraction of `f32::atan2`.
fn atan2(y: f32, x: f32) -> f32 {
    let (ax, ay) = (x.abs(), y.abs());
    let (lo, hi) = if ax < ay { (ax, ay) } else { (ay, ax) };
    if hi == 0.0 {
        return 0.0;
    }
    let a = lo / hi;
    let s = a * a;
    let mut r = (((-0.046_496_474 * s + 0.159_314_22) * s - 0.327_622_77) * s) * a + a;
    if ay > ax {
        r = TAU / 4.0 - r;
    }
    if x < 0.0 {
        r = TAU / 2.0 - r;
    }
    if y < 0.0 { -r } else { r }
}

/// Empties the braille cell holding dot (x, y), so what is drawn there next
/// stands in front of the trails.
fn clear_cell(br: &mut Braille, x: i32, y: i32) {
    let (x0, y0) = (x & !1, y & !3);
    if x0 < 0 || y0 < 0 || x0 >= br.pw() || y0 >= br.ph() {
        return;
    }
    for k in 0..8 {
        br.unset(x0 + (k & 1), y0 + (k >> 1));
    }
}

/// Draws a sprite centered on (x, y) in front of whatever is there.
fn sprite(br: &mut Braille, rows: &[&str], x: f32, y: f32, c: Color) {
    let w = rows.first().map_or(0, |r| r.len()) as i32;
    let (x0, y0) = (
        (x - w as f32 / 2.0).round() as i32,
        (y - rows.len() as f32 / 2.0).round() as i32,
    );
    // Each cell it touches is emptied once (the biggest takes 9×4 of them).
    let mut done = 0u64;
    for (j, row) in rows.iter().enumerate() {
        for (i, ch) in row.bytes().enumerate() {
            if ch != b'#' {
                continue;
            }
            let (px, py) = (x0 + i as i32, y0 + j as i32);
            let bit = ((py >> 2) - (y0 >> 2)) * 10 + (px >> 1) - (x0 >> 1);
            if done & (1 << bit) == 0 {
                done |= 1 << bit;
                clear_cell(br, px, py);
            }
        }
    }
    for (j, row) in rows.iter().enumerate() {
        for (i, ch) in row.bytes().enumerate() {
            if ch == b'#' {
                br.set(x0 + i as i32, y0 + j as i32, c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{INFO, Particles};
    use super::*;
    use crate::render::Canvas;
    use crate::scene::{Opts, Scene};
    use crate::theme::Theme;

    const MODES: [&str; 3] = ["flow", "fireworks", "fountain"];

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: theme.bg,
            seed: 7,
            fiesta,
        }
    }

    fn opts(mode: &str) -> Opts {
        Opts::new(INFO.options, format!("mode = '{mode}'").parse().unwrap())
    }

    /// Runs a scene for `frames` and draws it.
    fn run(s: &mut Particles, ctx: &Ctx, c: &mut Canvas, frames: usize) {
        for _ in 0..frames {
            s.update(ctx, 1.0 / 30.0);
        }
        c.clear();
        s.draw(ctx, c);
    }

    fn has_monarch(c: &Canvas) -> bool {
        c.cells().iter().any(|x| WINGS.contains(&x.fg))
    }

    /// A blossom's colors on screen (its cells keep one of the three).
    fn has_blossom(c: &Canvas) -> bool {
        c.cells()
            .iter()
            .any(|x| BLOOMS.iter().any(|k| k.contains(&x.fg)))
    }

    #[test]
    fn dresses_up_on_its_days() {
        let theme = Theme::classic();
        for mode in MODES {
            let o = opts(mode);
            let mut counts = Vec::new();
            for day in [(10, 28), (11, 2)] {
                let ctx = ctx(&theme, &o, Fiesta::of_day(day.0, day.1));
                let mut s = Particles::new(&ctx);
                s.resize(&ctx, 120, 36);
                s.retheme(&ctx);
                let mut c = Canvas::new(120, 36, Color::Reset);
                run(&mut s, &ctx, &mut c, 240);
                let m = &s.muertos;
                assert!(m.on(), "{mode} {day:?}");
                assert!(!m.monarchs.is_empty(), "{mode} {day:?}");
                assert!(has_monarch(&c), "{mode} {day:?}: no monarch on screen");
                if mode != "fireworks" {
                    assert!(!m.blossoms.is_empty(), "{mode} {day:?}");
                    assert!(!m.petals.is_empty(), "{mode} {day:?}");
                    assert!(has_blossom(&c), "{mode} {day:?}: no blossom on screen");
                }
                counts.push(m.monarchs.len() + m.petal_target);
            }
            assert!(counts[0] < counts[1], "{mode}: more on November 2");
        }
    }

    #[test]
    fn ordinary_days_stay_ordinary() {
        let theme = Theme::classic();
        for mode in MODES {
            let o = opts(mode);
            let ctx = ctx(&theme, &o, Fiesta::NONE);
            let mut s = Particles::new(&ctx);
            s.resize(&ctx, 120, 36);
            s.retheme(&ctx);
            let mut c = Canvas::new(120, 36, Color::Reset);
            run(&mut s, &ctx, &mut c, 120);
            let m = &s.muertos;
            assert!(!m.on() && m.monarchs.is_empty() && m.petals.is_empty());
            assert!(m.blossoms.is_empty() && !has_monarch(&c), "{mode}");
        }
    }

    /// The holiday comes and goes through `retheme` and leaves no trace: a
    /// scene that had it ends up drawing exactly what one that never did does.
    #[test]
    fn comes_and_goes_with_retheme() {
        let theme = Theme::classic();
        for mode in MODES {
            let o = opts(mode);
            let plain = ctx(&theme, &o, Fiesta::NONE);
            let fiesta = ctx(&theme, &o, Fiesta::of_day(11, 1));
            let mut a = Particles::new(&plain);
            let mut b = Particles::new(&plain);
            let (mut ca, mut cb) = (
                Canvas::new(90, 30, Color::Reset),
                Canvas::new(90, 30, Color::Reset),
            );
            for s in [&mut a, &mut b] {
                s.resize(&plain, 90, 30);
                s.retheme(&plain);
            }
            run(&mut a, &plain, &mut ca, 30);
            run(&mut b, &plain, &mut cb, 30);
            b.retheme(&fiesta);
            assert!(b.muertos.on(), "{mode}");
            run(&mut a, &plain, &mut ca, 90);
            run(&mut b, &fiesta, &mut cb, 90);
            assert!(has_monarch(&cb), "{mode}");
            b.retheme(&plain);
            assert!(!b.muertos.on() && b.muertos.monarchs.is_empty(), "{mode}");
            run(&mut a, &plain, &mut ca, 90);
            run(&mut b, &plain, &mut cb, 90);
            assert!(ca.cells() == cb.cells(), "{mode}: the holiday left a trace");
        }
    }

    #[test]
    fn the_cheap_arctangent_is_close_enough() {
        for i in 0..720 {
            let a = i as f32 / 720.0 * TAU - TAU / 2.0;
            let (y, x) = (a.sin() * 7.5, a.cos() * 7.5);
            assert!((atan2(y, x) - y.atan2(x)).abs() < 3e-4, "{a}");
        }
        assert_eq!(atan2(0.0, 0.0), 0.0);
    }

    /// Cutting a blossom never outgrows the room set aside for it, at any size.
    #[test]
    fn blossoms_are_cut_without_allocating() {
        let theme = Theme::classic();
        for mode in ["flow", "fountain"] {
            let o = opts(mode);
            let ctx = ctx(&theme, &o, Fiesta::of_day(11, 2));
            let mut s = Particles::new(&ctx);
            for (w, h) in [(300, 90), (120, 36), (40, 12), (5, 200)] {
                s.resize(&ctx, w, h);
                s.retheme(&ctx);
                let mut c = Canvas::new(w, h, Color::Reset);
                run(&mut s, &ctx, &mut c, 400);
                let m = &s.muertos;
                assert_eq!(m.cuts.len(), m.blossoms.len(), "{mode} {w}x{h}");
                for cut in &m.cuts {
                    assert_eq!(cut.cells.capacity(), 224, "{mode} {w}x{h}");
                }
            }
        }
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        for theme in &themes {
            for mode in MODES {
                for v in ["", "\ncount = 0\nspeed = 10.0\nscale = 10.0"] {
                    let o = Opts::new(INFO.options, format!("mode = '{mode}'{v}").parse().unwrap());
                    let ctx = ctx(theme, &o, Fiesta::of_day(11, 2));
                    let mut s = Particles::new(&ctx);
                    let sizes = [(80, 24), (1, 1), (2, 3), (7, 2), (300, 90), (3, 40)];
                    for (i, &(w, h)) in sizes.iter().enumerate() {
                        s.resize(&ctx, w, h);
                        if i == 0 {
                            s.retheme(&ctx);
                        }
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for f in 0..30 {
                            s.update(&ctx, if f % 13 == 12 { 5.0 } else { 1.0 / 30.0 });
                            c.clear();
                            s.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }
}
