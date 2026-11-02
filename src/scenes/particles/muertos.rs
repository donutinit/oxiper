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

use super::art::{Blossom, Cut, Life, atan2, clear_cell, draw_flower, fract, sprite, turns};
use super::{Dots, Field, Mode};
use crate::color::{Color, rgb};
use crate::fiesta::{Fiesta, Kind};
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
        self.fiesta.is(Kind::Muertos)
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
        if !fiesta.is(Kind::Muertos) || w == 0 || h == 0 {
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
        self.cuts.resize_with(self.blossoms.len(), Cut::roomy);
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
        Blossom::new(&mut self.rng, self.pw, self.ph, 0.2)
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
            let life = b.grow(dt, field, v, (self.pw, self.ph));
            if matches!(life, Life::Over) {
                let mut nb = self.new_blossom();
                nb.age = -0.5 - self.rng.f32() * 2.0;
                self.blossoms[i] = nb;
                continue;
            }
            if matches!(life, Life::Scatter) {
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
            draw_flower(
                b,
                cut,
                &self.bloom_cols[b.kind as usize],
                br,
                (cut_blossom, 25.0),
            );
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
fn cut_blossom(cut: &mut Cut, r: f32, (cx, cy): (i32, i32), rot: f32, (pw, ph): (i32, i32)) {
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
    cut.fill(r, (cx, cy), (pw, ph), ((0.4, 0.78), 0.0), lit);
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
