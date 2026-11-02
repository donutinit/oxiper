//! Navidad in the flock (see the module doc in `boids.rs`).
//!
//! The flock keeps its simulation and its random numbers; the holiday dresses
//! it: little angels with halos for the boids, Luzbel (the pastorela's devil)
//! for the hawk, and San Miguel arcángel, who now and then drops out of the
//! sky and drives Luzbel off. San Miguel and the angels' escarcha have a
//! random generator of their own, so ordinary days keep theirs.

use std::f32::consts::TAU;

use super::hues::like;
use super::{Boid, Boids, FLUTTER, Field, HEAD_LUT, Ribbon, Style, hash01};
use crate::color::{Color, mix_oklab, rgb};
use crate::render::{BOLD, Canvas};
use crate::scene::Ctx;

const NIEVE: u32 = 0xf4f1ea;
const PLATA: u32 = 0xc9ccd6;
const DORADO: u32 = 0xf2b632;
const ROJO: u32 = 0xc8102e;
const ROJO_VIVO: u32 = 0xff3b3b;
const AMARILLO: u32 = 0xffd23b;
/// The angels' wings, one color per flock: white, gold, silver and a warm
/// candlelit white.
const WING_COLORS: [u32; 4] = [NIEVE, DORADO, PLATA, 0xfff1c9];
/// An angel's wings seen from the front, left and right of its head: raised,
/// level, lowered.
const WINGS: [[char; 2]; 3] = [['\\', '/'], ['-', '-'], ['/', '\\']];
/// How bright each wing frame is.
const WING_LIGHT: [f32; 3] = [1.0, 0.9, 0.78];
/// The same in braille dots for `style = "dots"`, 5×3 around the head (the
/// halo is the dot above).
const WING_DOTS: [[&str; 3]; 3] = [
    ["#...#", ".#.#.", "..#.."],
    [".....", "##.##", "..#.."],
    [".....", "..#..", "##.##"],
];
/// Luzbel in place of the hawk: horns, his angry face, his cape and his
/// hooves. The second string colors each cell: `h` horns (gilded), `r` red,
/// `R` bright red, `s` shadow, `y` eyes, `d` nose.
const LUZBEL: [(&str, &str); 4] = [
    ("  ʌ   ʌ  ", "  h   h  "),
    ("  (◣▾◢)  ", "  ryydr  "),
    ("  ▟▓█▓▙  ", "  rsRsr  "),
    ("   ▐ ▌   ", "   r r   "),
];
/// Luzbel's trinche (pitchfork), on his facing side.
const TRINCHE: [char; 4] = ['Ψ', '┃', '┃', '┃'];
/// San Miguel's body: head, pauldrons and breastplate, belt, legs. `w` is
/// white, `g` gold, `h` the warm white of his face.
const MIGUEL: [(&str, &str); 4] = [
    ("  (◉)  ", "  hhh  "),
    (" ◢▟█▙◣ ", " gwwwg "),
    ("  ▐█▌  ", "  gwg  "),
    ("  ▝ ▘  ", "  g g  "),
];
/// Where the middle of San Miguel's sprite is, in cells from its left.
const MIGUEL_MID: i32 = 3;
/// Escarcha (tinsel glitter), twinkling as it drifts down.
const GLINTS: [char; 3] = ['·', '+', '✦'];
/// Sparks of Luzbel's brimstone puff, hottest first.
const EMBER_CHARS: [char; 3] = ['✶', '*', '·'];
const EMBERS: usize = 28;
/// How far the angels sense San Miguel, in units.
const GATHER: f32 = 36.0;
/// Seconds San Miguel takes to arrive, to chase, to hold his sword up and to
/// leave.
const ARRIVE: f32 = 1.0;
const RAISE: f32 = 1.4;
const LEAVE: f32 = 1.8;
/// Luzbel is struck at this distance, or when the chase runs this long.
const STRIKE: f32 = 5.0;
const CHASE: f32 = 5.5;

/// A grain of escarcha, in units.
#[derive(Clone, Copy, Default)]
struct Glint {
    x: f32,
    y: f32,
    fall: f32,
    phase: f32,
}

/// A spark of Luzbel's puff of brimstone, in units.
#[derive(Clone, Copy, Default)]
struct Ember {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Seconds left, and how many it had.
    life: f32,
    span: f32,
}

/// A ring of light spreading from where something happened (San Miguel's
/// arrival, Luzbel's end): its middle in units, its age, and whether it is
/// gold (0) or red (1).
#[derive(Clone, Copy, Default)]
struct Ring {
    x: f32,
    y: f32,
    age: f32,
    red: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    /// Not here: Luzbel hunts in peace.
    Away,
    /// Dropping out of the sky in a shaft of light.
    Arrive,
    /// Chasing Luzbel, faster the longer it lasts.
    Chase,
    /// Sword held up, Luzbel gone.
    Raise,
    /// Rising back into the sky.
    Leave,
}

/// San Miguel arcángel, in units.
struct Miguel {
    phase: Phase,
    /// Seconds in this phase.
    clock: f32,
    /// Seconds until he comes again.
    wait: f32,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// 1 faces right, -1 left.
    facing: f32,
}

impl Miguel {
    /// How much of him shows, 0 to 1.
    fn alpha(&self) -> f32 {
        match self.phase {
            Phase::Away => 0.0,
            Phase::Arrive => (self.clock / ARRIVE).min(1.0),
            Phase::Chase | Phase::Raise => 1.0,
            Phase::Leave => (1.0 - self.clock / LEAVE).max(0.0),
        }
    }

    /// The dot in the middle of his chest: the cell he stands in, one row
    /// below his head.
    fn dots(&self) -> (f32, f32) {
        (
            self.x.round() * 2.0 + 1.0,
            (self.y * 0.5).floor() * 4.0 + 1.0,
        )
    }
}

pub(super) struct Navidad {
    level: f32,
    /// Holiday randomness, apart from the flock's so ordinary days keep theirs.
    rng: fastrand::Rng,
    /// Scene seconds since the holiday came, for wings and flutter.
    t: f32,
    glints: Vec<Glint>,
    embers: [Ember; EMBERS],
    ember_next: usize,
    rings: [Ring; 2],
    ring_next: usize,
    wing: [Color; 4],
    wheel: [Color; HEAD_LUT],
    body: Color,
    halo: Color,
    /// Escarcha, silver and gold.
    escarcha: [Color; 2],
    /// Is Luzbel about, and in how many seconds is he back.
    here: bool,
    back_in: f32,
    suit: [Color; 3],
    gold: Color,
    eyes: [Color; 2],
    nose: Color,
    /// Embers, hottest first.
    sparks: [Color; 3],
    /// Luzbel faces where he flies: 1 right, -1 left.
    pub(super) facing: f32,
    /// Which way his tail streams (a unit vector), lagging his turns.
    pub(super) drape: (f32, f32),
    miguel: Miguel,
    /// San Miguel's white, his gold, and the warm white of his armor.
    light: [Color; 3],
}

impl Navidad {
    pub(super) fn new(ctx: &Ctx) -> Navidad {
        let level = ctx.fiesta.level.clamp(0.0, 1.0);
        let mut n = Navidad {
            level,
            rng: fastrand::Rng::with_seed(ctx.seed ^ 0x616e_6765_6c69_746f),
            t: 0.0,
            glints: Vec::new(),
            embers: [Ember::default(); EMBERS],
            ember_next: 0,
            rings: [Ring::default(); 2],
            ring_next: 0,
            wing: [Color::Reset; 4],
            wheel: [Color::Reset; HEAD_LUT],
            body: Color::Reset,
            halo: Color::Reset,
            escarcha: [Color::Reset; 2],
            here: true,
            back_in: 0.0,
            suit: [Color::Reset; 3],
            gold: Color::Reset,
            eyes: [Color::Reset; 2],
            nose: Color::Reset,
            sparks: [Color::Reset; 3],
            facing: 1.0,
            drape: (-1.0, 0.0),
            miguel: Miguel {
                phase: Phase::Away,
                clock: 0.0,
                wait: 0.0,
                x: 0.0,
                y: 0.0,
                vx: 0.0,
                vy: 0.0,
                facing: 1.0,
            },
            light: [Color::Reset; 3],
        };
        // The first visit comes sooner than the rest, so he is seen early.
        n.miguel.wait = 0.4 * n.gap();
        n
    }

    pub(super) fn level(&self) -> f32 {
        self.level
    }

    /// How many angels there are next to an ordinary flock: more as the
    /// posadas climb.
    pub(super) fn crowd(&self) -> f32 {
        0.8 + 0.4 * self.level
    }

    /// Is Luzbel about?
    pub(super) fn luzbel(&self) -> bool {
        self.here
    }

    /// Is San Miguel here?
    pub(super) fn visiting(&self) -> bool {
        self.miguel.phase != Phase::Away
    }

    /// Where San Miguel is, while Luzbel is running from him.
    pub(super) fn fleeing(&self) -> Option<(f32, f32)> {
        let m = &self.miguel;
        let on = match m.phase {
            Phase::Chase => true,
            Phase::Arrive => m.clock > 0.5 * ARRIVE,
            _ => false,
        };
        (on && self.here).then_some((m.x, m.y))
    }

    /// Seconds between San Miguel's visits: shorter as the posadas climb.
    fn gap(&mut self) -> f32 {
        (46.0 - 30.0 * self.level) * (0.75 + 0.5 * self.rng.f32())
    }

    /// Colors from the theme; the gold, the red and the flames keep their
    /// own hue under any theme.
    pub(super) fn retheme(&mut self, ctx: &Ctx) {
        let th = ctx.theme;
        for (slot, &hex) in self.wing.iter_mut().zip(&WING_COLORS) {
            *slot = like(th, rgb(hex));
        }
        if !th.native {
            // Silver is the white turned down; the candlelit one, gold-tinged.
            self.wing[2] = ctx.fade(self.wing[0], 0.3);
            self.wing[3] = mix_oklab(self.wing[0], self.wing[1], 0.4);
        }
        let stops = [self.wing[0], self.wing[1], self.wing[3], self.wing[2]];
        for (i, slot) in self.wheel.iter_mut().enumerate() {
            let p = i as f32 / HEAD_LUT as f32 * stops.len() as f32;
            let k = p as usize % stops.len();
            *slot = mix_oklab(stops[k], stops[(k + 1) % stops.len()], p.fract());
        }
        self.body = like(th, rgb(NIEVE));
        self.halo = like(th, rgb(DORADO));
        self.escarcha = [like(th, rgb(PLATA)), like(th, rgb(DORADO))];
        let red = like(th, rgb(ROJO));
        let bright = like(th, rgb(ROJO_VIVO));
        self.suit = [red, bright, ctx.fade(red, 0.45)];
        self.gold = like(th, rgb(DORADO));
        let yellow = like(th, rgb(AMARILLO));
        self.eyes = [yellow, bright];
        self.nose = ctx.fade(red, 0.55);
        self.sparks = [yellow, mix_oklab(yellow, bright, 0.5), red];
        self.light = [like(th, rgb(0xffffff)), self.gold, like(th, rgb(0xfff1c9))];
    }

    /// Scatters the escarcha over a field, more as the holiday comes.
    pub(super) fn scatter(&mut self, fw: f32, fh: f32) {
        let cells = fw * fh * 0.5;
        let n = ((cells * 0.0035 * (0.3 + self.level)).round() as usize).min(300);
        self.glints.clear();
        for _ in 0..n {
            let r = &mut self.rng;
            self.glints.push(Glint {
                x: r.f32() * fw,
                y: r.f32() * fh,
                fall: 0.8 + 1.2 * r.f32(),
                phase: r.f32() * TAU,
            });
        }
    }

    /// Wing frame of angel `i` (an index into `WINGS`): a steady beat, one
    /// rhythm for each.
    fn wing_frame(&self, i: usize) -> usize {
        let h = hash01(i);
        let beats = 2.6 + 2.0 * hash01(i ^ 0x5555);
        match ((self.t * beats + h).fract() * 4.0) as usize {
            0 => 0,
            2 => 2,
            _ => 1,
        }
    }

    /// Extra steering for angel `i`: it bobs as it beats its wings, and
    /// gathers round San Miguel while he is here.
    pub(super) fn force(&self, i: usize, b: &Boid, f: Field) -> (f32, f32) {
        let beat = 0.9 + 0.5 * hash01(i ^ 0x3c3c);
        let (mut fx, mut fy) = (
            0.0,
            ((self.t * beat + hash01(i)) * TAU).sin() * FLUTTER * 0.3,
        );
        let m = &self.miguel;
        if m.phase != Phase::Away {
            let dx = Boids::delta(b.x, m.x, f.fw, f.wrap);
            let dy = Boids::delta(b.y, m.y, f.fh, f.wrap);
            let d = (dx * dx + dy * dy).sqrt();
            if d > 6.0 && d < GATHER {
                let k = 9.0 * m.alpha() * (1.0 - d / GATHER) / d;
                fx += dx * k;
                fy += dy * k;
            }
        }
        (fx, fy)
    }

    /// A ring of light spreads from (x, y).
    fn ring(&mut self, x: f32, y: f32, red: bool) {
        self.rings[self.ring_next] = Ring {
            x,
            y,
            age: 0.0,
            red,
        };
        self.ring_next = (self.ring_next + 1) % self.rings.len();
    }

    /// `count` embers fly from (x, y), `speed` units a second at most.
    fn puff(&mut self, x: f32, y: f32, count: usize, speed: f32) {
        for _ in 0..count {
            let r = &mut self.rng;
            let a = r.f32() * TAU;
            let s = speed * (0.3 + 0.7 * r.f32());
            let life = 0.9 + 0.9 * r.f32();
            self.embers[self.ember_next] = Ember {
                x,
                y,
                vx: a.cos() * s,
                vy: a.sin() * s * 0.6,
                life,
                span: life,
            };
            self.ember_next = (self.ember_next + 1) % EMBERS;
        }
    }

    /// Luzbel comes back from the edge of the screen, hunting.
    fn enter(&mut self, hawk: &mut Boid, retarget: &mut f32, f: Field) {
        let right = self.rng.bool();
        hawk.x = if right { f.fw - 0.5 } else { 0.5 };
        hawk.y = self.rng.f32() * f.fh;
        let s = f.ms * 0.9;
        hawk.vx = if right { -s } else { s };
        hawk.vy = (self.rng.f32() - 0.5) * s * 0.4;
        *retarget = 0.0;
        self.here = true;
        self.puff(hawk.x, hawk.y, 8, 7.0);
    }

    /// Advances the holiday by `h` seconds: the embers and rings, Luzbel's
    /// comings and goings and San Miguel's visits.
    pub(super) fn tick(
        &mut self,
        h: f32,
        hawk: &mut Boid,
        retarget: &mut f32,
        f: Field,
        hawk_on: bool,
    ) {
        self.t += h;
        for e in self.embers.iter_mut().filter(|e| e.life > 0.0) {
            e.life -= h;
            e.x += e.vx * h;
            e.y += e.vy * h;
            e.vx *= (1.0 - 1.8 * h).max(0.0);
            e.vy = e.vy * (1.0 - 1.8 * h).max(0.0) - 5.0 * h;
        }
        for r in &mut self.rings {
            r.age += h;
        }
        if !self.here && hawk_on {
            self.back_in -= h;
            if self.back_in <= 0.0 {
                self.enter(hawk, retarget, f);
            }
        }
        let wrap_to = |p: f32, len: f32| {
            if f.wrap {
                p.rem_euclid(len)
            } else {
                p.max(0.0).min(len)
            }
        };
        self.miguel.clock += h;
        match self.miguel.phase {
            Phase::Away => {
                self.miguel.wait -= h;
                if self.miguel.wait <= 0.0 && self.here && hawk_on {
                    // Down from the sky, a little way off.
                    let side = if self.rng.bool() { 1.0 } else { -1.0 };
                    let (dx, dy) = (12.0 + 8.0 * self.rng.f32(), 10.0 + 8.0 * self.rng.f32());
                    let m = &mut self.miguel;
                    m.x = wrap_to(hawk.x + side * dx, f.fw);
                    m.y = (hawk.y - dy).max(7.0).min(f.fh - 3.0);
                    (m.vx, m.vy, m.facing) = (0.0, 0.0, -side);
                    (m.phase, m.clock) = (Phase::Arrive, 0.0);
                    let (x, y) = (m.x, m.y);
                    self.ring(x, y, false);
                }
            }
            Phase::Arrive | Phase::Chase => {
                let m = &mut self.miguel;
                let dx = Boids::delta(m.x, hawk.x, f.fw, f.wrap);
                let dy = Boids::delta(m.y, hawk.y, f.fh, f.wrap);
                let d = (dx * dx + dy * dy).sqrt().max(1e-3);
                // Faster and faster: a dash at the end of the chase.
                let dash = ((m.clock - 1.8) / 1.0).clamp(0.0, 1.0);
                let sp = f.ms
                    * if m.phase == Phase::Arrive {
                        0.4
                    } else {
                        1.5 + dash
                    };
                let k = (h * 5.0).min(1.0);
                m.vx += (dx / d * sp - m.vx) * k;
                m.vy += (dy / d * sp - m.vy) * k;
                m.x = wrap_to(m.x + m.vx * h, f.fw);
                m.y = wrap_to(m.y + m.vy * h, f.fh);
                if dx.abs() > 1.0 {
                    m.facing = dx.signum();
                }
                if m.phase == Phase::Arrive && m.clock >= ARRIVE {
                    (m.phase, m.clock) = (Phase::Chase, 0.0);
                } else if m.phase == Phase::Chase && (d < STRIKE || m.clock > CHASE) {
                    // The sword finds Luzbel: a ring of red light, a puff of
                    // brimstone, and he is gone for a while.
                    (m.phase, m.clock) = (Phase::Raise, 0.0);
                    self.here = false;
                    self.back_in = 7.0 + 8.0 * (1.0 - self.level) + 4.0 * self.rng.f32();
                    let (x, y) = (hawk.x, hawk.y);
                    self.ring(x, y, true);
                    self.puff(x, y, 22, 14.0);
                }
            }
            Phase::Raise | Phase::Leave => {
                let m = &mut self.miguel;
                let k = (h * 4.0).min(1.0);
                if m.phase == Phase::Raise {
                    m.vx -= m.vx * k;
                    m.vy -= m.vy * k;
                    if m.clock >= RAISE {
                        (m.phase, m.clock) = (Phase::Leave, 0.0);
                    }
                } else {
                    m.vy += (-9.0 - m.vy) * k;
                    m.vx -= m.vx * k;
                    if m.clock >= LEAVE {
                        m.phase = Phase::Away;
                        self.miguel.wait = self.gap();
                    }
                }
                let m = &mut self.miguel;
                m.x = wrap_to(m.x + m.vx * h, f.fw);
                m.y = wrap_to(m.y + m.vy * h, f.fh);
            }
        }
    }

    /// The escarcha drifts down.
    pub(super) fn drift(&mut self, dt: f32, f: Field) {
        let t = self.t;
        for g in &mut self.glints {
            g.y += g.fall * dt;
            g.x = (g.x + (t * 0.7 + g.phase).sin() * 0.5 * dt).rem_euclid(f.fw);
            if g.y >= f.fh {
                g.y -= f.fh + 1.0;
            }
        }
    }
}

impl Boids {
    /// The color of angel `i`'s wings: its flock's (or its heading's).
    fn wing_col(&self, n: &Navidad, i: usize) -> Color {
        let b = &self.boids[i];
        if self.by_heading {
            let a = b.vy.atan2(b.vx).rem_euclid(TAU) / TAU;
            n.wheel[((a * HEAD_LUT as f32) as usize).min(HEAD_LUT - 1)]
        } else {
            n.wing[b.flock as usize & 3]
        }
    }

    /// The flock as angels, Luzbel for the hawk, San Miguel when he comes.
    pub(super) fn draw_navidad(&mut self, ctx: &Ctx, c: &mut Canvas, n: &Navidad) {
        self.br.clear();
        let count = self.boids.len();
        let luzbel = self.hawk_on && n.here;
        if luzbel {
            self.draw_tail(ctx, n);
        }
        if self.trail_len > 0 {
            // A faint wake of light behind each angel.
            for i in 0..count {
                let col = self.wing_col(n, i);
                self.draw_trail(ctx, i, col, 0.5, 0.0);
            }
        }
        if self.style == Style::Dots {
            for i in 0..count {
                let b = self.boids[i];
                let w = n.wing_frame(i);
                let col = self.wing_col(n, i).scale(WING_LIGHT[w]);
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
                self.br.set(x + 2, y - 1, n.halo);
            }
        }
        let miguel = n.visiting();
        if miguel {
            self.draw_light(ctx, n);
        }
        self.br.blit(c);

        // Escarcha, twinkling.
        for (i, g) in n.glints.iter().enumerate() {
            let tw = (n.t * 2.2 + g.phase).sin();
            let (ch, k) = if tw > 0.85 {
                (GLINTS[2], 0.0)
            } else if tw > 0.35 {
                (GLINTS[1], 0.25)
            } else {
                (GLINTS[0], 0.5)
            };
            let col = ctx.fade(n.escarcha[i & 1], k);
            c.set(g.x.floor() as i32, (g.y * 0.5).floor() as i32, ch, col);
        }
        for e in n.embers.iter().filter(|e| e.life > 0.0) {
            let k = e.life / e.span;
            let (ch, col) = match k {
                k if k > 0.66 => (EMBER_CHARS[0], n.sparks[0]),
                k if k > 0.33 => (EMBER_CHARS[1], n.sparks[1]),
                k => (EMBER_CHARS[2], ctx.fade(n.sparks[2], 0.5 - k)),
            };
            c.set(e.x.floor() as i32, (e.y * 0.5).floor() as i32, ch, col);
        }

        if self.style == Style::Arrows {
            for i in 0..count {
                let b = &self.boids[i];
                let w = n.wing_frame(i);
                let col = self.wing_col(n, i).scale(WING_LIGHT[w]);
                let (x, y) = (b.x.round() as i32, (b.y * 0.5) as i32);
                // The halo glows a little, each at its own pace.
                let glow = 0.7 + 0.3 * (n.t * 2.0 + i as f32 * 1.3).sin();
                c.set(x, y - 1, '°', n.halo.scale(glow));
                c.set(x - 1, y, WINGS[w][0], col);
                c.set_attr(x, y, 'o', n.body, BOLD);
                c.set(x + 1, y, WINGS[w][1], col);
            }
        }
        if luzbel {
            self.draw_luzbel(c, n);
        }
        if miguel {
            self.draw_miguel(ctx, c, n);
        }
    }

    /// Luzbel's tail: a red ribbon streaming behind him, ending in a spade.
    fn draw_tail(&mut self, ctx: &Ctx, n: &Navidad) {
        let tail = Ribbon {
            cols: &n.suit[1..2],
            anchor: (
                self.hawk.x.round() * 2.0 + 1.0,
                ((self.hawk.y * 0.5) as i32 + 1) as f32 * 4.0 + 2.0,
            ),
            drape: n.drape,
            t: n.t,
            n: 10,
            len: 11.0,
            fade: 0.0,
        };
        let (tx, ty) = self.draw_ribbon(ctx, &tail);
        // The spade: two barbs back from the tip.
        let (dx, dy) = n.drape;
        for s in [-0.6f32, 0.6] {
            let (co, si) = (s.cos(), s.sin());
            let (bx, by) = (dx * co - dy * si, dx * si + dy * co);
            self.br
                .line(tx, ty, tx - bx * 3.5, ty - by * 3.5, n.suit[1]);
        }
    }

    fn draw_luzbel(&self, c: &mut Canvas, n: &Navidad) {
        let (x0, y0) = (
            self.hawk.x.round() as i32 - 4,
            (self.hawk.y * 0.5) as i32 - 2,
        );
        // His eyes smolder.
        let eyes = n.eyes[0].lerp(n.eyes[1], 0.5 + 0.5 * (n.t * 6.0).sin());
        for (r, (art, keys)) in LUZBEL.iter().enumerate() {
            for (k, (ch, key)) in art.chars().zip(keys.chars()).enumerate() {
                if ch == ' ' {
                    continue;
                }
                let col = match key {
                    'h' => n.gold,
                    'R' => n.suit[1],
                    's' => n.suit[2],
                    'y' => eyes,
                    'd' => n.nose,
                    _ => n.suit[0],
                };
                let attr = if r >= 1 { BOLD } else { 0 };
                c.set_attr(x0 + k as i32, y0 + r as i32, ch, col, attr);
            }
        }
        // The trinche in his hand, on the side he faces.
        let hx = x0 + 4 + 4 * n.facing as i32;
        for (r, &ch) in TRINCHE.iter().enumerate() {
            c.set_attr(hx, y0 + r as i32, ch, n.gold, BOLD);
        }
        c.set_attr(hx - n.facing as i32, y0 + 2, '─', n.suit[0], BOLD);
    }

    /// Where San Miguel's sword is, in dots: its hand and the way it points
    /// (a unit vector). It points at Luzbel while he chases him, and straight
    /// up when he holds it high.
    fn sword(&self, m: &Miguel) -> ((f32, f32), (f32, f32)) {
        let (cx, cy) = m.dots();
        let el: f32 = if matches!(m.phase, Phase::Arrive | Phase::Chase) {
            let dx = Boids::delta(m.x, self.hawk.x, self.fw, !self.bounce);
            let dy = Boids::delta(m.y, self.hawk.y, self.fh, !self.bounce);
            (-dy).atan2(dx.abs().max(1.0)).clamp(-0.9, 0.15)
        } else {
            1.35
        };
        let hand = (cx + m.facing * 6.0, cy + 3.0);
        (hand, (m.facing * el.cos(), -el.sin()))
    }

    /// San Miguel's light, in braille: the shaft of light he comes down in,
    /// the rings, his wings, his halo and his sword.
    fn draw_light(&mut self, ctx: &Ctx, n: &Navidad) {
        let m = &n.miguel;
        let a = m.alpha();
        // `k` is how much of its color a thing loses: 0 shows it in full.
        let fade = |col: Color, k: f32| ctx.fade(col, 1.0 - a * (1.0 - k));
        let [white, gold, warm] = n.light;
        let (cx, cy) = m.dots();
        // A shaft of light from the sky while he comes and goes.
        if matches!(m.phase, Phase::Arrive | Phase::Leave) {
            let beat = (n.t * 14.0) as i32;
            for y in 0..(cy as i32 - 10).max(0) {
                if (y / 2 + beat).rem_euclid(3) == 0 {
                    let k = 0.55 - 0.4 * (y as f32 / cy.max(1.0));
                    self.br.set(cx as i32 - 4, y, fade(warm, k));
                    self.br.set(cx as i32 + 4, y, fade(warm, k));
                }
            }
        }
        for r in n.rings.iter().filter(|r| r.age < 1.0) {
            let col = if r.red { n.suit[1] } else { warm };
            let col = ctx.fade(col, r.age.clamp(0.0, 1.0));
            let rad = 3.0 + r.age * 34.0;
            let steps = (rad * 2.0) as usize + 8;
            for k in 0..steps {
                let th = k as f32 / steps as f32 * TAU;
                self.br
                    .setf(r.x * 2.0 + th.cos() * rad, r.y * 2.0 + th.sin() * rad, col);
            }
        }
        // His wings: feathers fanning from each shoulder, beating fast while
        // he flies and slowly when he stands.
        let rate = if m.phase == Phase::Raise { 3.0 } else { 8.0 };
        let flap = 0.35 * (n.t * rate).sin();
        for side in [-1.0f32, 1.0] {
            let (sx, sy) = (cx + side * 4.0, cy - 1.0);
            for i in 0..7 {
                // The outer feathers are the longest, as on a swept wing.
                let th = 0.25 + 0.2 * i as f32 + flap;
                let len = 20.0 - 1.8 * i as f32;
                let (dx, dy) = (side * th.cos(), -th.sin());
                // White, turning to gold at the tips.
                let col = |t: f32| fade(white.lerp(gold, ((t - 0.5) * 2.0).clamp(0.0, 1.0)), 0.0);
                for o in [0.0, side * 1.0] {
                    self.br.line_with(
                        sx + dx * 2.0 + o,
                        sy + dy * 2.0,
                        sx + dx * len + o,
                        sy + dy * len,
                        col,
                    );
                }
            }
        }
        // His sword of light: a blade of white in a glow of gold.
        let (hand, (sdx, sdy)) = self.sword(m);
        let up = m.phase == Phase::Raise;
        let tip = (hand.0 + sdx * 22.0, hand.1 + sdy * 22.0);
        let (px, py) = (-sdy, sdx);
        for (o, k) in [(-1.0f32, 0.4), (1.0, 0.4), (0.0, 0.0)] {
            let col = if o == 0.0 { white } else { gold };
            self.br.line(
                hand.0 + sdx * 2.0 + px * o,
                hand.1 + sdy * 2.0 + py * o,
                tip.0 + px * o,
                tip.1 + py * o,
                fade(col, k),
            );
        }
        // The crossguard.
        let g = (hand.0 + sdx * 2.0, hand.1 + sdy * 2.0);
        self.br.line(
            g.0 - px * 3.0,
            g.1 - py * 3.0,
            g.0 + px * 3.0,
            g.1 + py * 3.0,
            fade(gold, 0.0),
        );
        // Rays from the tip while the sword is held up.
        if up {
            let pulse = 0.5 + 0.5 * (n.t * 9.0).sin();
            for k in 0..8 {
                let th = k as f32 / 8.0 * TAU + n.t;
                let l = 3.0 + 4.0 * pulse;
                self.br.line(
                    tip.0 + th.cos() * 2.0,
                    tip.1 + th.sin() * 2.0,
                    tip.0 + th.cos() * l,
                    tip.1 + th.sin() * l,
                    fade(warm, 0.2),
                );
            }
        }
        // His halo, over his head.
        for k in 0..16 {
            let th = k as f32 / 16.0 * TAU;
            self.br.setf(
                cx + th.cos() * 3.5,
                cy - 7.0 + th.sin() * 1.5,
                fade(gold, 0.0),
            );
        }
    }

    /// San Miguel's body, in cells, and the glint on the point of his sword.
    fn draw_miguel(&self, ctx: &Ctx, c: &mut Canvas, n: &Navidad) {
        let m = &n.miguel;
        let a = m.alpha();
        let fade = |col: Color| ctx.fade(col, 1.0 - a);
        let (x0, y0) = (
            m.x.round() as i32 - MIGUEL_MID,
            (m.y * 0.5).floor() as i32 - 1,
        );
        for (r, (art, keys)) in MIGUEL.iter().enumerate() {
            for (k, (ch, key)) in art.chars().zip(keys.chars()).enumerate() {
                if ch == ' ' {
                    continue;
                }
                let col = match key {
                    'g' => n.light[1],
                    'h' => n.light[2],
                    _ => n.light[0],
                };
                c.set_attr(x0 + k as i32, y0 + r as i32, ch, fade(col), BOLD);
            }
        }
        // The glint at the sword's point.
        let (hand, (sdx, sdy)) = self.sword(m);
        let (tx, ty) = (hand.0 + sdx * 22.0, hand.1 + sdy * 22.0);
        let glint = ['✦', '✧', '+', '✧'][(n.t * 7.0) as usize % 4];
        c.set_attr(
            (tx / 2.0).floor() as i32,
            (ty / 4.0).floor() as i32,
            glint,
            fade(n.light[0]),
            BOLD,
        );
    }
}
