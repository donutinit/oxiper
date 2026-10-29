//! Quema de Judas: a cartonería Judas over the fire.
//!
//! A Judas stands on its pole in the middle of the screen: a devil of
//! cartón, red and black with yellow trim, horns, a grin and a trinche in its
//! fist, with a bandolier, bundles of firecrackers and two cohetes tied to
//! it and its feet in a pyre of leña and straw. Torches burn on both sides.
//!
//! On Viernes Santo it waits, unlit, in the torchlight. On Sábado de Gloria
//! it is raised, waits, and a fuse runs along the ground to the pyre; then
//! the Judas catches. aafire's flames climb it as the cardboard burns from
//! its feet up, its firecrackers pop (and the cohetes shoot up and burst),
//! and it spins faster and faster on its pole until it falls apart in
//! embers. Then a new Judas is put up.

use super::{Fuel, Piece, Rect, Sparks, Steam, hash};
use crate::color::{Color, rgb};
use crate::render::Pixels;
use crate::scene::Ctx;

/// Seconds the new Judas takes to be raised.
const RAISE: f32 = 4.0;
/// Seconds it stands unlit before the fuse is lit.
const WAIT: f32 = 7.0;
/// Seconds the fuse takes to reach the pyre.
const FUSE: f32 = 3.5;
/// Seconds it burns, and of them how long the flames take to climb from its
/// feet to its horns.
const BURN: f32 = 17.0;
const CLIMB: f32 = 12.5;
/// Seconds the embers take to die down.
const ASHES: f32 = 6.0;
/// How long a scrap of cardboard burns bright, and how long until it has
/// smoldered away.
const BRIGHT: f32 = 0.45;
const SMOLDER: f32 = 1.3;

// What each pixel of the figure is made of.
/// Cartón rojo, in light and in shade.
const R: u8 = 1;
const L: u8 = 2;
const D: u8 = 3;
/// Black, yellow, white (the teeth), the pole.
const K: u8 = 4;
const Y: u8 = 5;
const W: u8 = 6;
const P: u8 = 7;
/// What is on its face (brows, mouth) and its eyes: red on its back.
const F: u8 = 8;
const E: u8 = 9;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Raise,
    Wait,
    Fuse,
    Burn,
    Ashes,
}

#[derive(Clone, Copy)]
struct Figure {
    cx: i32,
    /// The pixel row of its horns when it stands, its height and width, and
    /// the ground it stands on.
    top: i32,
    h: i32,
    w: i32,
    ground: i32,
    /// The pyre at its feet: half its width, and how tall it is.
    base: i32,
    pyre: i32,
}

#[derive(Clone, Copy)]
struct Cracker {
    /// Where it is tied, in units of the figure's height from its middle
    /// column and from its top.
    u: f32,
    v: f32,
    /// A cohete, which flies off when it goes.
    rocket: bool,
    /// Seconds after its cardboard catches that it goes off.
    delay: f32,
    popped: bool,
}

#[derive(Clone, Copy)]
struct Torch {
    x: i32,
    /// Pixel row of its head.
    y: i32,
}

/// A flash where something went off.
#[derive(Clone, Copy)]
struct Flash {
    x: i32,
    y: i32,
    age: f32,
}

/// A cohete flying up, to burst.
#[derive(Clone, Copy)]
struct Rocket {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
    hue: u8,
}

/// Colors of the holiday, cached in `recolor`.
struct Pal {
    red: [Color; 3],
    black: Color,
    yellow: Color,
    white: Color,
    wood: Color,
    straw: [Color; 2],
    ember: Color,
}

pub(super) struct Quema {
    seed: u64,
    rng: fastrand::Rng,
    /// Sábado de Gloria: it burns. On Viernes Santo it only waits.
    burning: bool,
    fig: Option<Figure>,
    /// What each pixel of the figure is made of, when it catches (as a share
    /// of the climb) and when it did (seconds into the burn, or infinity).
    mat: Vec<u8>,
    thr: Vec<f32>,
    born: Vec<f32>,
    crackers: Vec<Cracker>,
    torches: Vec<Torch>,
    phase: Phase,
    clock: f32,
    /// Seconds since the fuse reached the pyre, through the burn and the
    /// ashes: what the cardboard's age is told by.
    fire_t: f32,
    /// How far round its pole it has turned, and how fast it turns.
    angle: f32,
    omega: f32,
    flashes: Vec<Flash>,
    rockets: Vec<Rocket>,
    sparks: Sparks,
    t: f32,
    pal: Pal,
}

impl Quema {
    pub fn new(seed: u64) -> Quema {
        let none = Color::Reset;
        Quema {
            seed,
            rng: fastrand::Rng::with_seed(seed ^ 0x7ada5),
            burning: false,
            fig: None,
            mat: Vec::new(),
            thr: Vec::new(),
            born: Vec::new(),
            crackers: Vec::new(),
            torches: Vec::new(),
            phase: Phase::Wait,
            clock: 0.0,
            fire_t: 0.0,
            angle: 0.0,
            omega: 0.0,
            flashes: Vec::new(),
            rockets: Vec::new(),
            sparks: Sparks::new(seed ^ 0x5ba2),
            t: 0.0,
            pal: Pal {
                red: [none; 3],
                black: none,
                yellow: none,
                white: none,
                wood: none,
                straw: [none; 2],
                ember: none,
            },
        }
    }

    /// A new Judas is put up.
    fn reset(&mut self) {
        self.born.fill(f32::INFINITY);
        for c in &mut self.crackers {
            c.popped = false;
        }
        self.fire_t = 0.0;
        self.angle = 0.0;
        self.omega = 0.0;
        self.rockets.clear();
        self.flashes.clear();
    }

    /// Cosine of the turn: how wide the Judas shows (negative: its back).
    fn turn(&self) -> f32 {
        self.angle.cos()
    }

    /// How far below the ground the Judas still is while it is being raised.
    fn rise(&self, f: &Figure) -> i32 {
        if self.phase != Phase::Raise {
            return 0;
        }
        let k = (self.clock / RAISE).clamp(0.0, 1.0);
        let k = k * k * (3.0 - 2.0 * k);
        ((1.0 - k) * (f.h + f.pyre) as f32) as i32
    }

    /// Where, on the screen, the figure's pixel (`sx`, `sy`) is.
    fn project(&self, f: &Figure, sx: i32, sy: i32) -> (i32, i32) {
        let dx = ((sx - f.w / 2) as f32 * self.turn()).round() as i32;
        (f.cx + dx, f.top + sy + self.rise(f))
    }

    /// How hot the burning figure is at a pixel that caught `age` seconds ago.
    fn glow(age: f32) -> f32 {
        if age < BRIGHT {
            1.0
        } else {
            (1.0 - (age - BRIGHT) / (SMOLDER - BRIGHT)).max(0.0) * 0.7
        }
    }

    /// Something goes off at cracker `i`.
    fn pop(&mut self, i: usize) {
        let Some(f) = self.fig else { return };
        let c = self.crackers[i];
        self.crackers[i].popped = true;
        let sx = f.w / 2 + (c.u * f.h as f32).round() as i32;
        let sy = (c.v * f.h as f32).round() as i32;
        let (x, y) = self.project(&f, sx, sy);
        self.flashes.push(Flash { x, y, age: 0.0 });
        for _ in 0..12 {
            self.sparks.spawn(x as f32, y as f32, 30.0, 0);
        }
        // It blows a hole in the cardboard round it.
        let r = (f.h / 22).max(2);
        for yy in (sy - r).max(0)..(sy + r + 1).min(f.h) {
            for xx in (sx - r).max(0)..(sx + r + 1).min(f.w) {
                let k = (yy * f.w + xx) as usize;
                if (xx - sx).pow(2) + (yy - sy).pow(2) <= r * r
                    && self.mat[k] != 0
                    && self.born[k] > self.fire_t
                {
                    self.born[k] = self.fire_t - BRIGHT;
                }
            }
        }
        self.omega += 1.4;
        if c.rocket && self.rockets.len() < self.rockets.capacity() {
            let up = 55.0 + 30.0 * self.rng.f32();
            self.rockets.push(Rocket {
                x: x as f32,
                y: y as f32,
                vx: (self.rng.f32() - 0.5) * 24.0,
                vy: -up * (f.h as f32 / 60.0).sqrt().clamp(0.7, 2.0),
                age: 0.0,
                life: 0.6 + 0.35 * self.rng.f32(),
                hue: (i % 2) as u8,
            });
        }
    }

    /// The cardboard catches, burns and falls away, and the firecrackers go
    /// off when the fire reaches them.
    fn burn(&mut self, dt: f32, f: &Figure) {
        let b = self.fire_t / CLIMB;
        let c = self.turn();
        for sy in 0..f.h {
            for sx in 0..f.w {
                let k = (sy * f.w + sx) as usize;
                if self.mat[k] == 0 {
                    continue;
                }
                if self.born[k].is_infinite() {
                    if b >= self.thr[k] {
                        self.born[k] = self.fire_t;
                    }
                    continue;
                }
                // A scrap of ember breaks off as it turns to ash.
                let age = self.fire_t - self.born[k];
                if (BRIGHT..BRIGHT + dt).contains(&age) && self.rng.f32() < 0.12 {
                    let x = f.cx + ((sx - f.w / 2) as f32 * c).round() as i32;
                    self.sparks.spawn(x as f32, (f.top + sy) as f32, 7.0, 2);
                }
            }
        }
        for i in 0..self.crackers.len() {
            let c = self.crackers[i];
            if c.popped {
                continue;
            }
            let sx = (f.w / 2 + (c.u * f.h as f32).round() as i32).clamp(0, f.w - 1);
            let sy = ((c.v * f.h as f32).round() as i32).clamp(0, f.h - 1);
            let k = (sy * f.w + sx) as usize;
            if self.born[k].is_finite() && self.fire_t - self.born[k] >= c.delay {
                self.pop(i);
            }
        }
        // It spins faster as it burns, and a pop sets it going.
        let target = 1.4 + 5.0 * (b / 0.8).min(1.0);
        self.omega += (target - self.omega) * (dt * 1.5).min(1.0);
        self.angle += self.omega * dt;
    }
}

impl Piece for Quema {
    fn recolor(&mut self, ctx: &Ctx) {
        let themed = !ctx.theme.native;
        let fg = ctx.theme.fg;
        let g = &ctx.theme.grad;
        let or = |native: u32, theme: Color| if themed { theme } else { rgb(native) };
        let p = &mut self.pal;
        let red = or(0xd1262b, ctx.pick(6, Color::Reset));
        p.red = [red, red.lerp(fg, 0.22), red.scale(0.62)];
        p.black = or(0x1c1c1c, ctx.fade(fg, 0.9));
        p.yellow = or(0xffd23b, ctx.pick(4, Color::Reset));
        p.white = or(0xf4f1ea, fg);
        p.wood = or(0x9a6a3a, g.at(0.45));
        p.straw = [or(0xc4a456, g.at(0.7)), or(0x8a6a30, g.at(0.4))];
        // Fire and embers keep their own hues under any theme.
        p.ember = rgb(0xff7a1a);
    }

    fn layout(&mut self, w: usize, ph: usize, level: f32) {
        self.fig = None;
        self.mat.clear();
        self.thr.clear();
        self.born.clear();
        self.crackers.clear();
        self.torches.clear();
        self.flashes.clear();
        self.rockets.clear();
        self.sparks.clear();
        let (w, ph) = (w as i32, ph as i32);
        // Sábado de Gloria burns; Viernes Santo, which comes before it, waits.
        self.burning = level >= 1.0;
        self.phase = if self.burning {
            Phase::Raise
        } else {
            Phase::Wait
        };
        self.clock = 0.0;
        self.fire_t = 0.0;
        self.angle = 0.0;
        self.omega = 0.0;
        if ph < 24 || w < 20 {
            return;
        }
        let mut rng = fastrand::Rng::with_seed(self.seed ^ 0x5eed);
        let h = (ph * 6 / 10).clamp(14, 150);
        let (fw, mat) = raster(h);
        let ground = ph - 1;
        let pyre = (h / 11).max(3);
        let f = Figure {
            cx: w / 2,
            // Its feet are buried in the straw.
            top: ground - pyre + 2 - h + 1,
            h,
            w: fw,
            ground,
            base: (h * 3 / 10).max(4),
            pyre,
        };
        self.thr = (0..mat.len() as i32)
            .map(|i| {
                let (x, y) = (i % fw, i / fw);
                let noise = hash(x, y, self.seed as u32) as f32 / u32::MAX as f32;
                (1.0 - (y as f32 + 0.5) / h as f32) * 0.8 + 0.2 * noise
            })
            .collect();
        self.born = vec![f32::INFINITY; mat.len()];
        self.mat = mat;
        self.fig = Some(f);

        // Firecrackers tied all over it, and two cohetes on its shoulders.
        let spots: [(f32, f32, bool); 15] = [
            (-0.12, 0.24, false),
            (-0.07, 0.31, false),
            (-0.02, 0.38, false),
            (0.03, 0.44, false),
            (0.08, 0.5, false),
            (-0.09, 0.52, false),
            (-0.22, 0.28, false),
            (0.2, 0.31, false),
            (-0.065, 0.68, false),
            (0.065, 0.7, false),
            (-0.17, 0.2, true),
            (0.17, 0.2, true),
            (0.3, 0.46, false),
            (-0.3, 0.22, false),
            (0.0, 0.58, false),
        ];
        let n = if h >= 40 { spots.len() } else { 9 };
        for &(u, v, rocket) in &spots[..n] {
            self.crackers.push(Cracker {
                u,
                v,
                rocket,
                delay: rng.f32() * 0.4,
                popped: false,
            });
        }
        // The cohetes are the last of the short list, too.
        if n < spots.len() {
            self.crackers[n - 1].rocket = true;
        }
        self.flashes.reserve_exact(32);
        self.rockets.reserve_exact(4);
        self.sparks.reserve(700);

        // Torches on both sides, lit through the whole night.
        let th = (h * 4 / 10).max(5);
        let near = f.cx - (h * 3 / 4 + fw / 4);
        let far = near - h * 6 / 10;
        for (x, depth) in [
            (near, th),
            (f.cx + (f.cx - near), th),
            (far, th * 8 / 10),
            (f.cx + (f.cx - far), th * 8 / 10),
        ] {
            if x >= 2 && x < w - 2 && (depth == th || ph >= 60) {
                self.torches.push(Torch {
                    x,
                    y: ground - depth,
                });
            }
        }
    }

    fn fuel(&self, sw: usize) -> Option<Fuel> {
        let f = self.fig?;
        // The pyre: nothing while the Judas waits, a roar as it catches, and
        // the embers after it.
        let k = match self.phase {
            Phase::Burn => {
                let up = (self.clock / 1.5).min(1.0);
                let down = ((BURN - self.clock) / 3.0).clamp(0.0, 1.0);
                (0.15 + 0.85 * up) * (0.4 + 0.6 * down)
            }
            Phase::Ashes => 0.4 * (1.0 - self.clock / ASHES).max(0.0),
            _ => 0.0,
        };
        Some(Fuel::span(f.cx - f.base, f.cx + f.base, sw, k))
    }

    fn reach(&self) -> f32 {
        if self.fig.is_some() { 0.8 } else { 1.0 }
    }

    fn feed(&mut self, heat: &mut [u16], stride: usize, sw: usize, intensity: f32) {
        let Some(f) = self.fig else { return };
        let mut put = |rng: &mut fastrand::Rng, x: i32, y: i32, v: f32| {
            if y < 0 {
                return;
            }
            let v = (v * (0.85 + 0.15 * rng.f32()) * intensity).clamp(0.0, 255.0) as u16
                * super::FP as u16;
            let row = y as usize * stride + super::PAD;
            for sx in [2 * x, 2 * x + 1] {
                if sx >= 0 && (sx as usize) < sw && row + (sx as usize) < heat.len() {
                    heat[row + sx as usize] = v;
                }
            }
        };
        // The torches: a flame at each head, breathing.
        let t = self.t;
        for (i, tr) in self.torches.iter().enumerate() {
            let flick = 0.8 + 0.2 * (t * 9.0 + i as f32 * 2.1).sin();
            for dy in 0..2 {
                for dx in -1..=1 {
                    let edge = if dx == 0 { 1.0 } else { 0.75 };
                    put(
                        &mut self.rng,
                        tr.x + dx,
                        tr.y - 1 + dy,
                        230.0 * flick * edge,
                    );
                }
            }
        }
        if !matches!(self.phase, Phase::Burn | Phase::Ashes) {
            return;
        }
        // The cardboard burning: a flame from every pixel that has caught.
        let c = self.turn();
        let rise = self.rise(&f);
        let mut last = (i32::MIN, i32::MIN);
        for sy in 0..f.h {
            for sx in 0..f.w {
                let k = (sy * f.w + sx) as usize;
                if self.mat[k] == 0 || self.born[k].is_infinite() {
                    continue;
                }
                let age = self.fire_t - self.born[k];
                if age >= SMOLDER {
                    continue;
                }
                let x = f.cx + ((sx - f.w / 2) as f32 * c).round() as i32;
                let y = f.top + sy + rise;
                if (x, y) == last {
                    continue;
                }
                last = (x, y);
                put(&mut self.rng, x, y, 235.0 * Quema::glow(age));
            }
        }
        // A flash of flame where something went off.
        for fl in &self.flashes {
            for dy in -2..=2 {
                for dx in -2..=2 {
                    if dx * dx + dy * dy <= 5 {
                        put(&mut self.rng, fl.x + dx, fl.y + dy, 255.0);
                    }
                }
            }
        }
    }

    fn update(&mut self, dt: f32, _wind: f32) {
        self.t += dt;
        let Some(f) = self.fig else { return };
        self.sparks.update(dt);
        self.flashes.retain_mut(|fl| {
            fl.age += dt;
            fl.age < 0.18
        });
        for r in &mut self.rockets {
            r.age += dt;
            r.x += r.vx * dt;
            r.y += r.vy * dt;
            r.vy += 25.0 * dt;
            self.sparks.spawn(r.x, r.y, 4.0, 2);
        }
        let mut i = 0;
        while i < self.rockets.len() {
            if self.rockets[i].age >= self.rockets[i].life {
                let r = self.rockets.swap_remove(i);
                self.flashes.push(Flash {
                    x: r.x as i32,
                    y: r.y as i32,
                    age: 0.05,
                });
                for _ in 0..28 {
                    self.sparks.spawn(r.x, r.y, 42.0, r.hue);
                }
            } else {
                i += 1;
            }
        }
        // Waiting, it sways a little on its pole.
        let sway = 0.14 * (self.t * 0.8).sin();
        if !self.burning {
            self.angle = sway;
            return;
        }
        self.clock += dt;
        if matches!(self.phase, Phase::Burn | Phase::Ashes) {
            self.fire_t += dt;
        }
        match self.phase {
            Phase::Raise => {
                self.angle = 0.0;
                if self.clock >= RAISE {
                    self.phase = Phase::Wait;
                    self.clock = 0.0;
                }
            }
            Phase::Wait => {
                self.angle = sway;
                if self.clock >= WAIT {
                    self.phase = Phase::Fuse;
                    self.clock = 0.0;
                }
            }
            Phase::Fuse => {
                self.angle = sway;
                // The spark runs along the ground to the pyre.
                let k = (self.clock / FUSE).clamp(0.0, 1.0);
                let x0 = (f.cx - f.h) as f32;
                let x = x0 + ((f.cx - f.base - 1) as f32 - x0) * k;
                self.sparks.spawn(x, (f.ground - 1) as f32, 14.0, 0);
                if self.clock >= FUSE {
                    self.phase = Phase::Burn;
                    self.clock = 0.0;
                    self.omega = 0.5;
                }
            }
            Phase::Burn => {
                self.burn(dt, &f);
                if self.clock >= BURN {
                    self.phase = Phase::Ashes;
                    self.clock = 0.0;
                }
            }
            Phase::Ashes => {
                self.angle += self.omega * dt;
                if self.clock >= ASHES {
                    self.phase = Phase::Raise;
                    self.clock = 0.0;
                    self.reset();
                }
            }
        }
    }

    fn steam(&self) -> Option<(&Steam, Color)> {
        None
    }

    fn draw(&self, _ctx: &Ctx, px: &mut Pixels) {
        let Some(f) = &self.fig else { return };
        self.draw_pyre(px, f);
        self.draw_figure(px, f);
    }

    fn lick(&self) -> Option<Rect> {
        let f = self.fig?;
        // The flames of the pyre lick up its legs.
        Some(Rect {
            x0: f.cx - f.base,
            x1: f.cx + f.base,
            y0: f.ground - f.pyre - f.h / 5,
            y1: f.ground,
        })
    }

    fn draw_front(&self, _ctx: &Ctx, px: &mut Pixels) {
        let Some(f) = &self.fig else { return };
        let p = &self.pal;
        // The fuse, a cord along the ground.
        if self.burning && matches!(self.phase, Phase::Wait | Phase::Fuse) {
            for x in (f.cx - f.h).max(1)..f.cx - f.base {
                if x % 2 == 0 {
                    px.set(x, f.ground - 1, p.wood.scale(0.7));
                }
            }
        }
        for tr in &self.torches {
            // The pole and the rag wrapped round its head.
            for y in tr.y + 2..=f.ground {
                px.set(tr.x, y, p.wood);
            }
            for dx in -1..=1 {
                px.set(tr.x + dx, tr.y, p.black.lerp(p.ember, 0.5));
                px.set(tr.x + dx, tr.y + 1, p.black);
            }
        }
        if self.phase != Phase::Ashes {
            for c in self.crackers.iter().filter(|c| !c.popped) {
                self.draw_cracker(px, f, c);
            }
        }
        for r in &self.rockets {
            px.set(r.x as i32, r.y as i32, rgb(0xfffbe8));
        }
        self.sparks.draw(px);
        for fl in &self.flashes {
            let k = fl.age / 0.18;
            let col = rgb(0xfffbe8).lerp(rgb(0xffb030), k);
            let r = 1 + (k * 2.0) as i32;
            for d in -r..=r {
                px.set(fl.x + d, fl.y, col);
                px.set(fl.x, fl.y + d, col);
            }
        }
    }

    #[cfg(test)]
    fn counts(&self) -> (usize, usize, usize) {
        (
            self.crackers.iter().filter(|c| !c.popped).count(),
            self.torches.len(),
            self.born.iter().filter(|b| b.is_finite()).count(),
        )
    }
}

impl Quema {
    fn draw_pyre(&self, px: &mut Pixels, f: &Figure) {
        let p = &self.pal;
        let (hw, ph) = (f.base as f32, f.pyre as f32);
        // Cold, then catching, then coals.
        let fire = match self.phase {
            Phase::Burn => (self.clock / 1.5).min(1.0),
            Phase::Ashes => 1.0 - self.clock / ASHES,
            _ => 0.0,
        }
        .clamp(0.0, 1.0);
        let rise = self.rise(f);
        for dx in -f.base..=f.base {
            let u = dx as f32 / hw;
            let height = (ph * (1.0 - u * u).max(0.0).powf(0.6)).round() as i32;
            for k in 0..height {
                let y = f.ground - k + rise;
                let h = hash(dx, k, 17);
                let mut col = if h.is_multiple_of(5) {
                    p.wood.scale(0.8)
                } else {
                    p.straw[((dx + k) & 1) as usize]
                };
                if fire > 0.0 {
                    let flick = 0.6 + 0.4 * (h % 7) as f32 / 6.0;
                    // The straw burns to coals, which glow and die down.
                    col = if self.phase == Phase::Ashes {
                        let coal = p.black.lerp(p.ember, flick * (0.25 + 0.75 * fire));
                        col.lerp(coal, 0.6 + 0.4 * fire)
                    } else {
                        col.lerp(col.lerp(p.ember, flick), fire)
                    };
                }
                px.set(f.cx + dx, y, col);
            }
        }
    }

    fn draw_figure(&self, px: &mut Pixels, f: &Figure) {
        let p = &self.pal;
        let c = self.turn();
        let back = c < 0.0;
        let ac = c.abs();
        let half = ((f.w as f32 / 2.0) * ac).round() as i32;
        let rise = self.rise(f);
        let b = if self.phase == Phase::Burn {
            self.fire_t / CLIMB
        } else {
            0.0
        };
        // Edge on, it is dimmer; facing us, it is as it was painted.
        let shade = (0.7 + 0.3 * ac / 0.9).min(1.0);
        let frame = (self.t * 20.0) as u32;
        for sy in 0..f.h {
            let y = f.top + sy + rise;
            for dx in -half..=half {
                let sxf = if ac < 0.001 { 0.0 } else { dx as f32 / c };
                let sx = f.w / 2 + sxf.round() as i32;
                if sx < 0 || sx >= f.w {
                    continue;
                }
                let k = (sy * f.w + sx) as usize;
                let m = self.mat[k];
                if m == 0 {
                    continue;
                }
                let col = if self.born[k].is_infinite() {
                    let mut col = match m {
                        R => p.red[0],
                        L => p.red[1],
                        D => p.red[2],
                        K => p.black,
                        Y => p.yellow,
                        W if back => p.red[0],
                        W => p.white,
                        P => p.wood,
                        F if back => p.red[0],
                        F => p.black,
                        E if back => p.red[0],
                        _ => p.yellow,
                    };
                    // Cardboard about to catch glows.
                    let ahead = self.thr[k] - b;
                    if b > 0.0 && ahead < 0.08 {
                        col = col.lerp(p.ember, 0.5 * (1.0 - ahead / 0.08));
                    }
                    if shade < 1.0 { col.scale(shade) } else { col }
                } else {
                    let age = self.fire_t - self.born[k];
                    if age >= SMOLDER {
                        continue;
                    }
                    let r = hash(sx, sy, frame) % 100;
                    if age < BRIGHT {
                        let col = rgb(0xfff0a0).lerp(rgb(0xff8a1a), age / BRIGHT);
                        if r < 18 {
                            rgb(0xffffff).lerp(col, 0.5)
                        } else {
                            col
                        }
                    } else {
                        // Charred, with embers still alive in it.
                        let k = (age - BRIGHT) / (SMOLDER - BRIGHT);
                        let ash = rgb(0x3a1e12).lerp(rgb(0x0e0a0a), k);
                        if r < (12.0 * (1.0 - k)) as u32 {
                            p.ember
                        } else {
                            ash
                        }
                    }
                };
                px.set(f.cx + dx, y, col);
            }
        }
    }

    fn draw_cracker(&self, px: &mut Pixels, f: &Figure, c: &Cracker) {
        let p = &self.pal;
        let sx = f.w / 2 + (c.u * f.h as f32).round() as i32;
        let sy = (c.v * f.h as f32).round() as i32;
        let (x, y) = self.project(f, sx, sy);
        let tall = if c.rocket { 3 } else { (f.h / 30).clamp(1, 2) };
        for dy in 0..tall {
            px.set(x, y + dy, if c.rocket { p.white } else { p.yellow });
        }
        if c.rocket {
            px.set(x, y - 1, p.red[0]);
            px.set(x - 1, y + tall - 1, p.red[2]);
            px.set(x + 1, y + tall - 1, p.red[2]);
        } else {
            px.set(x, y - 1, p.white);
        }
    }
}

/// Paints the Judas for a figure `h` pixels tall: its width, and what each
/// pixel is made of.
fn raster(h: i32) -> (i32, Vec<u8>) {
    let fw = ((h as f32 * 0.78).ceil() as i32) | 1;
    let mut mat = vec![0u8; (fw * h) as usize];
    let hf = h as f32;
    // Fills what `shape` says is inside it, in units of the height: `u` from
    // the middle column, `v` from the top.
    let mut fill = |shape: &dyn Fn(f32, f32) -> Option<u8>| {
        for y in 0..h {
            for x in 0..fw {
                let u = (x as f32 + 0.5 - fw as f32 / 2.0) / hf;
                let v = (y as f32 + 0.5) / hf;
                if let Some(m) = shape(u, v) {
                    mat[(y * fw + x) as usize] = m;
                }
            }
        }
    };
    // Nothing is thinner than a pixel.
    let thin = 1.0 / hf;
    let seg = move |u: f32, v: f32, a: (f32, f32), b: (f32, f32), t: f32| {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let len2 = (dx * dx + dy * dy).max(1e-9);
        let k = (((u - a.0) * dx + (v - a.1) * dy) / len2).clamp(0.0, 1.0);
        let (px, py) = (a.0 + k * dx, a.1 + k * dy);
        ((u - px).powi(2) + (v - py).powi(2)).sqrt() <= t.max(thin) / 2.0
    };
    let dot = move |u: f32, v: f32, c: (f32, f32), r: f32| {
        ((u - c.0).powi(2) + (v - c.1).powi(2)).sqrt() <= r.max(thin * 0.6)
    };
    // The pole it hangs on, and its tail.
    fill(&|u, v| (u.abs() <= 0.012f32.max(thin * 0.6) && v >= 0.2).then_some(P));
    fill(&|u, v| {
        let tail = seg(u, v, (-0.07, 0.53), (-0.2, 0.6), 0.02)
            || seg(u, v, (-0.2, 0.6), (-0.27, 0.5), 0.02);
        if dot(u, v, (-0.285, 0.48), 0.028) {
            Some(Y)
        } else {
            tail.then_some(K)
        }
    });
    // Legs in black, boots in shade, soles in yellow.
    fill(&|u, v| {
        let leg = seg(u, v, (-0.05, 0.5), (-0.085, 0.86), 0.07)
            || seg(u, v, (0.05, 0.5), (0.085, 0.86), 0.07);
        let foot = |dv: f32| dv.abs() <= 0.06;
        let boot = (0.86..0.95).contains(&v) && (foot(u + 0.095) || foot(u - 0.095));
        let sole = (0.95..0.985).contains(&v) && (foot(u + 0.095) || foot(u - 0.095));
        if sole {
            Some(Y)
        } else if boot {
            Some(D)
        } else {
            leg.then_some(K)
        }
    });
    // The trinche behind the arm that holds it.
    fill(&|u, v| {
        let shaft = seg(u, v, (0.305, 0.1), (0.305, 0.8), 0.014);
        let tines = seg(u, v, (0.27, 0.06), (0.27, 0.11), 0.012)
            || seg(u, v, (0.305, 0.035), (0.305, 0.11), 0.012)
            || seg(u, v, (0.34, 0.06), (0.34, 0.11), 0.012)
            || seg(u, v, (0.265, 0.108), (0.345, 0.108), 0.012);
        (shaft || tines).then_some(K)
    });
    // The torso: lit on the left, shaded on the right, yellow trim, a black belt.
    fill(&|u, v| {
        if !(0.2..=0.52).contains(&v) {
            return None;
        }
        let half = 0.15 - (v - 0.2) / 0.32 * 0.05;
        if u.abs() > half {
            return None;
        }
        let buttons = dot(u, v, (0.0, 0.28), 0.014)
            || dot(u, v, (0.0, 0.36), 0.014)
            || dot(u, v, (0.0, 0.44), 0.014);
        Some(if (0.49..0.53).contains(&v) {
            if u.abs() < 0.016 { Y } else { K }
        } else if v < 0.215 || u.abs() < 0.011 || buttons {
            Y
        } else if u < -0.05 {
            L
        } else if u > 0.07 {
            D
        } else {
            R
        })
    });
    // Arms: one raised in a fist, one holding the trinche.
    fill(&|u, v| {
        let left = seg(u, v, (-0.14, 0.23), (-0.27, 0.31), 0.07)
            || seg(u, v, (-0.27, 0.31), (-0.33, 0.19), 0.06)
            || dot(u, v, (-0.335, 0.175), 0.035);
        let right = seg(u, v, (0.14, 0.23), (0.25, 0.33), 0.07)
            || seg(u, v, (0.25, 0.33), (0.3, 0.43), 0.06)
            || dot(u, v, (0.305, 0.44), 0.035);
        let cuff = dot(u, v, (-0.32, 0.21), 0.022) || dot(u, v, (0.295, 0.415), 0.022);
        if cuff {
            Some(Y)
        } else if left {
            Some(L)
        } else if right {
            Some(D)
        } else {
            None
        }
    });
    // Horns, head, and the face on it.
    fill(&|u, v| {
        let horn = seg(u, v, (-0.04, 0.075), (-0.075, 0.03), 0.03)
            || seg(u, v, (-0.075, 0.03), (-0.115, 0.005), 0.024)
            || seg(u, v, (0.04, 0.075), (0.075, 0.03), 0.03)
            || seg(u, v, (0.075, 0.03), (0.115, 0.005), 0.024);
        let tip = dot(u, v, (-0.118, 0.01), 0.014) || dot(u, v, (0.118, 0.01), 0.014);
        let eyes = seg(u, v, (-0.04, 0.102), (-0.012, 0.11), 0.016)
            || seg(u, v, (0.012, 0.11), (0.04, 0.102), 0.016);
        let brows = seg(u, v, (-0.05, 0.088), (-0.01, 0.099), 0.011)
            || seg(u, v, (0.01, 0.099), (0.05, 0.088), 0.011);
        let mouth = seg(u, v, (-0.03, 0.14), (0.03, 0.14), 0.015);
        let teeth = seg(u, v, (-0.018, 0.14), (0.018, 0.14), 0.007);
        let beard = seg(u, v, (0.0, 0.15), (0.0, 0.185), 0.016);
        let head =
            dot(u, v, (0.0, 0.115), 0.058) || (u.abs() < 0.028 && (0.165..0.21).contains(&v));
        if tip {
            Some(Y)
        } else if eyes {
            Some(E)
        } else if teeth {
            Some(W)
        } else if brows || mouth || beard {
            Some(F)
        } else if head {
            Some(if u < -0.02 { L } else { R })
        } else if horn {
            Some(K)
        } else {
            None
        }
    });
    (fw, mat)
}
