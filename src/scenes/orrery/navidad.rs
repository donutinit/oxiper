//! Navidad on the orrery (see the module doc in `orrery.rs`).
//!
//! Like Día de Muertos, the holiday only dresses what the orrery already
//! has: the sun, the planets, the belt and Saturn's ring. It keeps no random
//! numbers at all: the tinsel's glints and the star's pulse are found again
//! every frame from a clock and a hash, so an ordinary day is as it ever was.

use std::f32::consts::TAU;

use super::{hash01, nearest, warmest};
use crate::color::{Color, Gradient, mix_oklab, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Braille, Canvas};
use crate::scene::Ctx;

/// Rojo nochebuena, verde pino, dorado, plata and blanco.
pub(super) const ROJO: u32 = 0xc8102e;
pub(super) const VERDE: u32 = 0x1f6f3a;
pub(super) const DORADO: u32 = 0xf2b632;
pub(super) const PLATA: u32 = 0xc9ccd6;
pub(super) const BLANCO: u32 = 0xf4f1ea;
/// The brightest part of the star and the glint on the glass.
pub(super) const BRILLO: u32 = 0xfffbea;
/// A darker gold for the tips of the star's rays.
const ORO_VIEJO: u32 = 0xb8860b;

/// How much bigger the star is than the everyday sun, to give its rays room.
pub(super) const GROW: f32 = 1.55;

/// The Santos Inocentes prank repeats every `PRANK_EVERY` seconds of the wall
/// clock: the planets turn round at `PRANK_AT`, go backwards for `PRANK_FOR`
/// (the turns at either end included), and then the joke is owned up to for
/// `OWNED_FOR` more.
const PRANK_EVERY: f32 = 40.0;
const PRANK_AT: f32 = 8.0;
const PRANK_FOR: f32 = 14.0;
const OWNED_FOR: f32 = 4.5;
/// How long the planets take to slow down, stop and turn round.
const TURN: f32 = 1.6;

/// A planet's esfera: 0 is a cap over a ball, 1 a small ball with a gleam in
/// it, 2 a big glass ball with a hook; by how large the planet is.
const RANK: [i32; 9] = [0, 1, 1, 0, 2, 2, 1, 1, 0];
/// The glass of each planet: indices into `Navidad::glass` (rojo, verde,
/// dorado, plata), so that neighbors never match.
const GLASS: [usize; 9] = [3, 2, 1, 0, 2, 0, 3, 1, 0];
/// How far the esferas push their moons out, in units, per size.
const PAD: [f32; 3] = [0.0, 0.9, 2.2];

/// What a cell of an esfera is made of, and so which of its colors it takes.
#[derive(Clone, Copy)]
enum Part {
    Hook,
    Cap,
    Glint,
    Hi,
    Body,
    Shade,
}

/// One cell of an esfera: where (relative to the planet's cell) and what.
type Piece = (i8, i8, char, Part);

use Part::{Body, Cap, Glint, Hi, Hook, Shade};

/// A cap over a ball: the small planets, which are a dot in any case.
const SMALL: [Piece; 2] = [(0, -1, '╤', Cap), (0, 0, '●', Body)];
/// A little glass ball, lit from the left, under its cap.
const MIDDLE: [Piece; 4] = [
    (0, -1, '╤', Cap),
    (-1, 0, '◖', Hi),
    (0, 0, '█', Body),
    (1, 0, '◗', Shade),
];
/// A big glass ball in quadrant blocks, lit from the upper left, with its gold
/// cap and the hook it hangs by.
const BIG: [Piece; 17] = [
    (0, -3, '∩', Hook),
    (0, -2, '▄', Cap),
    (-2, -1, '▗', Hi),
    (-1, -1, '▟', Glint),
    (0, -1, '█', Hi),
    (1, -1, '▙', Body),
    (2, -1, '▖', Body),
    (-2, 0, '▐', Hi),
    (-1, 0, '█', Hi),
    (0, 0, '█', Body),
    (1, 0, '█', Body),
    (2, 0, '▌', Shade),
    (-2, 1, '▝', Body),
    (-1, 1, '▜', Body),
    (0, 1, '█', Body),
    (1, 1, '▛', Shade),
    (2, 1, '▘', Shade),
];

/// A braille dot in the star's rays: where, which ray it belongs to, how far
/// from the center and from the ray's axis, in units.
#[derive(Clone, Copy)]
struct StarDot {
    x: i32,
    y: i32,
    k: u8,
    d: f32,
    perp: f32,
}

/// A dot of the glow around the star: where, how far from the center in
/// units, and the dither threshold it has to beat to light up.
#[derive(Clone, Copy)]
struct GlowDot {
    x: i32,
    y: i32,
    d: f32,
    bay: f32,
}

/// An esfera's colors: the glass lit, plain and in shadow.
#[derive(Clone, Copy)]
struct Glass {
    hi: Color,
    body: Color,
    shade: Color,
}

/// What Navidad brings to the orrery. Nothing here runs on ordinary days.
pub(super) struct Navidad {
    pub(super) on: bool,
    pub(super) level: f32,
    /// December 28: the planets go backwards now and then.
    inocentes: bool,
    /// Seconds of holiday, for the star's pulse and the tinsel's glints.
    t: f32,
    /// Wall-clock seconds into the prank cycle.
    wall: f32,
    /// Which way the planets go: 1 forward, -1 backwards, between the two
    /// while they turn round.
    dir: f32,
    /// How many rays the star has: more as the posadas climb.
    pub(super) points: usize,
    /// The star's core and the reach of its longest rays, in units.
    core: f32,
    reach: f32,
    /// How wide a ray is at its base, in units.
    base: f32,
    /// The dots the rays can fall on, from the layout.
    rays: Vec<StarDot>,
    /// The dots of the glow round the star, a dither that thins out outward.
    glow: Vec<GlowDot>,
    /// The size of each planet's esfera (0 to 2), from the layout.
    size: [u8; 9],
    pub(super) star: Gradient,
    brillo: Color,
    glass: [Glass; 4],
    cap: Color,
    /// Plata, dorado, blanco, rojo and verde: the tinsel.
    escarcha: [Color; 5],
}

impl Navidad {
    pub(super) fn new() -> Navidad {
        let none = Glass {
            hi: Color::Reset,
            body: Color::Reset,
            shade: Color::Reset,
        };
        Navidad {
            on: false,
            level: 0.0,
            inocentes: false,
            t: 0.0,
            wall: 0.0,
            dir: 1.0,
            points: 8,
            core: 1.0,
            reach: 2.0,
            base: 0.5,
            rays: Vec::new(),
            glow: Vec::new(),
            size: [0; 9],
            star: Gradient::new(&[]),
            brillo: Color::Reset,
            glass: [none; 4],
            cap: Color::Reset,
            escarcha: [Color::Reset; 5],
        }
    }

    pub(super) fn set(&mut self, fiesta: Fiesta) {
        self.on = fiesta.is(crate::fiesta::Kind::Navidad);
        self.inocentes = fiesta.inocentes();
        self.level = if self.on {
            fiesta.level.clamp(0.0, 1.0)
        } else {
            0.0
        };
        self.points = match self.level {
            l if l >= 0.75 => 16,
            l if l >= 0.45 => 12,
            _ => 8,
        };
        if !self.inocentes {
            self.dir = 1.0;
        }
    }

    /// What the layout depends on: the number of rays (0 on ordinary days).
    pub(super) fn key(&self) -> usize {
        if self.on { self.points } else { 0 }
    }

    pub(super) fn retheme(&mut self, ctx: &Ctx) {
        let native = ctx.theme.native;
        let g = ctx.ramp(&[]);
        // Under other themes each Christmas color is the theme's nearest accent.
        let tint = |hex: u32| {
            if native {
                rgb(hex)
            } else {
                nearest(&ctx.theme.colors, hex).unwrap_or(g.at(0.8))
            }
        };
        let gold = if native {
            rgb(DORADO)
        } else {
            warmest(&ctx.theme.colors).unwrap_or(g.at(0.8))
        };
        let white = if native { rgb(BLANCO) } else { g.at(1.0) };
        let silver = if native {
            rgb(PLATA)
        } else {
            mix_oklab(white, g.at(0.55), 0.3)
        };
        self.brillo = if native { rgb(BRILLO) } else { g.at(1.0) };
        self.star = Gradient::new(&if native {
            [ORO_VIEJO, DORADO, 0xffe9a3, BRILLO].map(rgb)
        } else {
            [
                mix_oklab(gold, g.at(0.3), 0.45),
                gold,
                mix_oklab(gold, g.at(1.0), 0.5),
                g.at(1.0),
            ]
        });
        self.glass = [tint(ROJO), tint(VERDE), gold, silver].map(|body| Glass {
            hi: mix_oklab(body, white, 0.45),
            body,
            shade: ctx.fade(body, 0.4),
        });
        self.cap = if native { rgb(DORADO) } else { gold };
        self.escarcha = [silver, gold, white, tint(ROJO), tint(VERDE)];
    }

    /// Lays the star out once the sun and the first orbit are known, so that
    /// drawing its rays takes no trigonometry: the dots around the sun that
    /// can fall on a ray, each with the ray it is nearest to.
    pub(super) fn lay(&mut self, br: &Braille, cx: f32, cy: f32, sun_r: f32, inner: f32, big: f32) {
        self.core = sun_r * 0.5;
        self.reach = (sun_r * 2.0 + 1.0).min(inner - 0.6).max(sun_r + 0.8);
        self.base = (0.36 * sun_r).clamp(0.8, 2.4);
        let (pw, ph) = (br.pw(), br.ph());
        let n = self.points;
        let (r0, r1) = (self.core * 0.6, self.reach);
        self.rays.clear();
        self.glow.clear();
        let x0 = (((cx - r1) * 2.0).floor() as i32).max(0);
        let x1 = (((cx + r1) * 2.0).ceil() as i32).min(pw - 1);
        let y0 = (((cy - r1) * 2.0).floor() as i32).max(0);
        let y1 = (((cy + r1) * 2.0).ceil() as i32).min(ph - 1);
        let per = n as f32 / TAU;
        for y in y0..=y1 {
            let uy = (y as f32 + 0.5) * 0.5 - cy;
            for x in x0..=x1 {
                let ux = (x as f32 + 0.5) * 0.5 - cx;
                let d2 = ux * ux + uy * uy;
                if d2 < r0 * r0 || d2 > r1 * r1 {
                    continue;
                }
                let d = d2.sqrt();
                self.glow.push(GlowDot {
                    x,
                    y,
                    d,
                    bay: BAYER[(y & 3) as usize][(x & 3) as usize] as f32 / 16.0,
                });
                // Clockwise from straight up, in ray spacings.
                let rel = ux.atan2(-uy) * per;
                let k = rel.round();
                let off = (rel - k) / per;
                self.rays.push(StarDot {
                    x,
                    y,
                    k: (k as i32).rem_euclid(n as i32) as u8,
                    d,
                    perp: d * off.sin().abs(),
                });
            }
        }
        // Esferas grow with the screen, and with the planet.
        let bonus = if big >= 70.0 {
            1
        } else if big < 30.0 {
            -1
        } else {
            0
        };
        for (k, size) in self.size.iter_mut().enumerate() {
            *size = (RANK[k] + bonus).clamp(0, 2) as u8;
        }
    }

    /// Moves the holiday's clocks along: the star's pulse and the tinsel's glints.
    pub(super) fn step(&mut self, flow: f32) {
        self.t = (self.t + flow).rem_euclid(1000.0 * TAU);
    }

    /// The Santos Inocentes prank keeps the wall clock, however slowly the
    /// orbits go: every so often the planets slow down, turn round and go
    /// backwards for a while.
    pub(super) fn tick(&mut self, real: f32) {
        if !self.inocentes {
            return;
        }
        self.wall = (self.wall + real.clamp(0.0, 30.0)).rem_euclid(PRANK_EVERY);
        let p = self.wall - PRANK_AT;
        // 0 while going forward, 1 backwards.
        let back = if p < 0.0 || p >= PRANK_FOR {
            0.0
        } else if p < TURN {
            smooth(p / TURN)
        } else if p < PRANK_FOR - TURN {
            1.0
        } else {
            1.0 - smooth((p - (PRANK_FOR - TURN)) / TURN)
        };
        self.dir = 1.0 - 2.0 * back;
    }

    /// Which way the planets go now, 1 forward, -1 backwards.
    pub(super) fn dir(&self) -> f32 {
        self.dir
    }

    /// What the planets shout: "¡Inocente!" while they go backwards, and then,
    /// once they are back, the joke owned up to.
    pub(super) fn shout(&self) -> Option<&'static str> {
        if !self.on || !self.inocentes {
            return None;
        }
        let p = self.wall - PRANK_AT;
        if (TURN * 0.5..PRANK_FOR - TURN * 0.5).contains(&p) {
            Some("¡INOCENTE!")
        } else if (PRANK_FOR..PRANK_FOR + OWNED_FOR).contains(&p) {
            Some("¡Inocente palomita!")
        } else {
            None
        }
    }

    /// How far an esfera pushes the planet's moons out, in units.
    pub(super) fn pad(&self, kind: usize) -> f32 {
        if self.on {
            PAD[self.size[kind] as usize]
        } else {
            0.0
        }
    }

    /// The color of the `i`th rock of the belt when it is tinsel (escarcha):
    /// more of the belt as the posadas climb, mostly plata and dorado, now and
    /// then a strand of rojo or verde. `None` when it stays a rock.
    pub(super) fn belt(&self, i: usize) -> Option<Color> {
        let share = 0.2 + 0.7 * self.level;
        if hash01(i as u32) >= share {
            return None;
        }
        let q = hash01(i as u32 ^ 0x5bd1_e995);
        Some(
            self.escarcha[match q {
                q if q < 0.42 => 0,
                q if q < 0.82 => 1,
                q if q < 0.94 => 2,
                q if q < 0.97 => 3,
                _ => 4,
            }],
        )
    }

    /// A rock of the belt as tinsel: a glint, and now and then a short strand
    /// of it lying along the orbit. `at` is its place in dots, (`s`, `co`) the
    /// sine and cosine of its angle round the sun and `t` the orbits' squash.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn draw_rock(
        &self,
        br: &mut Braille,
        ctx: &Ctx,
        i: usize,
        base: Color,
        bright: f32,
        at: (f32, f32),
        angle: (f32, f32),
        squash: f32,
    ) {
        let (x, y) = (at.0.floor() as i32, at.1.floor() as i32);
        // Rocks of the belt glint now and then: a little cross of light.
        let h = hash01(i as u32 ^ 0x27d4_eb2f);
        let lit = if h < 0.2 {
            0.5 + 0.5 * (self.t * (1.5 + 4.0 * h) + h * 97.0).sin()
        } else {
            0.0
        };
        let col = ctx.fade(base, (1.0 - bright) * 0.5);
        if lit > 0.82 {
            let k = (lit - 0.82) / 0.18;
            let glow = base.lerp(self.brillo, 0.4 + 0.6 * k);
            br.set(x, y, glow);
            if k > 0.3 {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    br.set(x + dx, y + dy, col.lerp(self.brillo, 0.5 * k));
                }
            }
            return;
        }
        br.set(x, y, col);
        // A strand of tinsel: two more dots along the way the rock is going.
        if h > 0.78 {
            let (s, co) = angle;
            let (tx, ty) = (-s, -co * squash);
            let len = tx.hypot(ty).max(1e-3);
            let (tx, ty) = (tx / len * 1.2, ty / len * 1.2);
            br.setf(at.0 + tx, at.1 + ty, col);
            br.setf(at.0 - tx, at.1 - ty, col);
        }
    }

    /// Saturn's ring is a band of tinsel, plata and dorado catching the light.
    pub(super) fn ring(&self, k: usize) -> Color {
        let glint = ((self.t * 2.0 + k as f32 * 0.9).sin() * 0.5 + 0.5).powi(3);
        let base = self.escarcha[(k / 2) % 2];
        base.lerp(self.brillo, 0.55 * glint)
    }

    /// The glow round the star: a dither of old gold, thick near the core and
    /// thinning out, breathing a little.
    pub(super) fn draw_glow(&self, br: &mut Braille) {
        let breathe = 0.8 + 0.2 * (self.t * 1.1 + 1.0).sin();
        let inv = 1.0 / self.reach;
        for dot in &self.glow {
            let f = 1.0 - dot.d * inv;
            if f * f * breathe * 0.5 > dot.bay {
                br.set(dot.x, dot.y, self.star.at(0.05 + 0.3 * f));
            }
        }
    }

    /// The Estrella de Belén's rays: tapering spikes, the longest straight up,
    /// down and to either side, shorter ones between them, each pulsing on its
    /// own beat. White-hot at the base, going to old gold at the tip.
    pub(super) fn draw_star(&self, br: &mut Braille) {
        let n = self.points;
        let breathe = 0.9 + 0.1 * (self.t * 1.1).sin();
        let mut len = [0.0f32; 16];
        let mut wide = [0.0f32; 16];
        let mut shine = [0.0f32; 16];
        for k in 0..n {
            let (l, w) = ray(k, n);
            let beat = (self.t * 2.4 + k as f32 * 2.1).sin();
            len[k] = self.reach * l * breathe * (0.8 + 0.2 * beat);
            wide[k] = self.base * w;
            shine[k] = 0.85 + 0.15 * (self.t * 3.1 + k as f32 * 1.3).sin();
        }
        for dot in &self.rays {
            let k = dot.k as usize;
            if dot.d > len[k] {
                continue;
            }
            let u = dot.d / len[k];
            // A spike: as wide as `wide` at the base, a dot at the tip.
            let half = 0.15 + (wide[k] - 0.15) * (1.0 - u);
            if dot.perp > half {
                continue;
            }
            let v = 1.0 - u;
            br.set(dot.x, dot.y, self.star.at((0.55 + 0.45 * v) * shine[k]));
        }
    }

    /// The star's core: a bright disc in quadrant blocks, breathing a little.
    pub(super) fn draw_core(&self, c: &mut Canvas, cx: f32, cy: f32) {
        let r = self.core * (0.96 + 0.04 * (self.t * 2.7).sin());
        super::quad_disc(c, cx, cy, r, |d| self.star.at(1.0 - 0.45 * d * d));
    }

    /// An esfera in place of the planet at (`x`, `y`) in units. `glow` is a
    /// glint on the glass now and then.
    pub(super) fn draw_esfera(&self, c: &mut Canvas, kind: usize, x: f32, y: f32) {
        let (col, row) = (x.floor() as i32, (y * 0.5).floor() as i32);
        let glass = &self.glass[GLASS[kind]];
        let size = self.size[kind];
        let pieces: &[Piece] = match size {
            0 => &SMALL,
            1 => &MIDDLE,
            _ => &BIG,
        };
        // A glint wanders over each ball in turn.
        let gleam = (self.t * 1.3 + kind as f32 * 2.3).sin() > 0.93;
        for &(dx, dy, ch, part) in pieces {
            let (fg, ch, attr) = match part {
                Hook => (self.escarcha[0], ch, 0),
                Cap => (self.cap_for(kind), ch, BOLD),
                Glint => (
                    self.brillo,
                    if gleam { '✦' } else { ch },
                    if gleam { BOLD } else { 0 },
                ),
                Hi => (glass.hi, ch, 0),
                Body => (glass.body, ch, 0),
                Shade => (glass.shade, ch, 0),
            };
            c.set_attr(col + dx as i32, row + dy as i32, ch, fg, attr);
        }
    }

    /// A planet's cap: gold, or silver on a golden ball.
    fn cap_for(&self, kind: usize) -> Color {
        if GLASS[kind] == 2 {
            self.escarcha[0]
        } else {
            self.cap
        }
    }

    /// "¡INOCENTE!" while the planets go backwards, spaced out, big and
    /// festive along the bottom of the screen.
    pub(super) fn draw_shout(&self, c: &mut Canvas) {
        let Some(text) = self.shout() else {
            return;
        };
        // The shout is letter by letter with a space between; the joke owned
        // up to is plain text.
        let gap = if text.len() < 14 { 2 } else { 1 };
        let len = (text.chars().count() as i32 - 1) * gap + 1;
        let (w, h) = (c.w() as i32, c.h() as i32);
        let (x0, y) = ((w - len) / 2, h - 2);
        // Every letter takes a color in turn, and they bounce a little.
        let flash = (self.t * 2.0).floor() as usize;
        for (i, ch) in text.chars().enumerate() {
            if ch == ' ' {
                continue;
            }
            let g = &self.glass[(i + flash) % 4];
            let bounce = ((self.t * 6.0 - i as f32 * 0.7).sin() > 0.85) as i32;
            c.set_attr(x0 + i as i32 * gap, y - bounce, ch, g.hi, BOLD);
        }
    }
}

/// A 4×4 Bayer matrix, for dithering.
const BAYER: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

/// Smoothstep: 0 to 1 with a gentle start and finish.
fn smooth(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// How long and how wide (next to the biggest) the `k`th of `n` rays is: the
/// ones pointing up, right, down and left are longest, those halfway between
/// shorter, the rest shortest.
fn ray(k: usize, n: usize) -> (f32, f32) {
    let q = n / 4;
    let j = k % q;
    if j == 0 {
        // Straight up and down a little longer than to the sides.
        if (k / q).is_multiple_of(2) {
            (1.0, 1.0)
        } else {
            (0.88, 0.95)
        }
    } else if q.is_multiple_of(2) && j == q / 2 {
        (0.72, 0.85)
    } else {
        (0.54, 0.7)
    }
}
