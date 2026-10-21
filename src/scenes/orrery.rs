//! orrery: planets and moons orbiting a sun.
//!
//! Positions are kept in "units" of one cell width on both axes (a cell is
//! 1×2 units, a braille dot 0.5×0.5), so orbits are true ellipses on screen.
//! Each orbit lies in a plane seen from above at an angle: its depth axis is
//! squashed by `tilt`, and planets on the far side pass behind the sun.
//!
//! On Día de Muertos the sun becomes a folk-art sun-calavera, hand drawn at
//! four sizes: a sugar skull with flowers in its eye sockets, icing on its
//! brow and a toothy grin, framed in gold and crowned with slowly turning
//! rays, straight ones in cempasúchil and wavy ones in rosa mexicano; now
//! and then it winks. The planets wear rings of marigold petals like
//! Saturn's, the asteroid belt turns into a path of petals (a few of them
//! velvety terciopelo), and comets swing around the sun shedding petals that
//! tumble away from it. How much of it there is follows the holiday's level:
//! on its first day only Saturn and Earth wear garlands and one comet comes
//! by; on November 2 every planet does, the belt is half petals and three
//! comets are out. The holiday has random numbers of its own, so ordinary
//! days are exactly as before.

use std::f32::consts::TAU;

use crate::color::{Color, Gradient, mix_oklab, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Braille, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "orrery",
    aliases: &["planets", "solar"],
    about: "planets and moons orbiting a sun",
    options: &[
        Opt {
            key: "planets",
            default: Val::I(8),
            help: "How many planets (1-9; fewer when the screen is small)",
        },
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Orbital speed multiplier",
        },
        Opt {
            key: "orbits",
            default: Val::B(true),
            help: "Show the dotted orbit paths",
        },
        Opt {
            key: "trails",
            default: Val::B(true),
            help: "Short fading trails behind the planets",
        },
        Opt {
            key: "belt",
            default: Val::B(true),
            help: "Asteroid belt between the rocky planets and the giants",
        },
        Opt {
            key: "moons",
            default: Val::B(true),
            help: "Moons around some planets",
        },
        Opt {
            key: "tilt",
            default: Val::F(0.5),
            help: "Orbit flattening for a 3/4 view (1 = straight from above)",
        },
    ],
    make: |ctx| Box::new(Orrery::new(ctx)),
};

struct Moon {
    /// Orbit radius in moon units.
    r: f32,
    /// Seconds per orbit; negative is retrograde.
    period: f32,
    color: u32,
}

struct Kind {
    glyph: char,
    color: u32,
    ecc: f32,
    bold: bool,
    ring: bool,
    moons: &'static [Moon],
}

const KINDS: [Kind; 9] = [
    // Mercury
    Kind {
        glyph: '•',
        color: 0xb8aea4,
        ecc: 0.11,
        bold: false,
        ring: false,
        moons: &[],
    },
    // Venus
    Kind {
        glyph: '●',
        color: 0xead08f,
        ecc: 0.02,
        bold: false,
        ring: false,
        moons: &[],
    },
    // Earth
    Kind {
        glyph: '●',
        color: 0x4f94ee,
        ecc: 0.03,
        bold: false,
        ring: false,
        moons: &[Moon {
            r: 1.0,
            period: 2.7,
            color: 0xd6d6d6,
        }],
    },
    // Mars
    Kind {
        glyph: '•',
        color: 0xe0673d,
        ecc: 0.08,
        bold: false,
        ring: false,
        moons: &[],
    },
    // Jupiter
    Kind {
        glyph: '◉',
        color: 0xdcab76,
        ecc: 0.05,
        bold: true,
        ring: false,
        moons: &[
            Moon {
                r: 0.85,
                period: 1.3,
                color: 0xf0dc8a,
            },
            Moon {
                r: 1.2,
                period: 2.1,
                color: 0xd8c8aa,
            },
            Moon {
                r: 1.6,
                period: 3.3,
                color: 0xb8a894,
            },
            Moon {
                r: 2.05,
                period: 5.2,
                color: 0x9c9084,
            },
        ],
    },
    // Saturn
    Kind {
        glyph: '●',
        color: 0xe8cf8e,
        ecc: 0.05,
        bold: true,
        ring: true,
        moons: &[Moon {
            r: 2.3,
            period: 4.6,
            color: 0xe6b45c,
        }],
    },
    // Uranus
    Kind {
        glyph: '●',
        color: 0x9ee0e6,
        ecc: 0.04,
        bold: false,
        ring: false,
        moons: &[Moon {
            r: 1.0,
            period: 2.3,
            color: 0xc8d4d8,
        }],
    },
    // Neptune
    Kind {
        glyph: '●',
        color: 0x5577ee,
        ecc: 0.02,
        bold: false,
        ring: false,
        moons: &[Moon {
            r: 1.1,
            period: -3.0,
            color: 0xd0c8c0,
        }],
    },
    // Pluto
    Kind {
        glyph: '∙',
        color: 0xc2ab92,
        ecc: 0.22,
        bold: false,
        ring: false,
        moons: &[],
    },
];

/// Which planets survive when fewer are wanted, most iconic first.
const PRIORITY: [usize; 9] = [5, 2, 4, 3, 1, 7, 0, 6, 8];
const MAX_MOONS: usize = 4;
const MAX_ASTEROIDS: usize = 3000;
/// Marks the asteroid belt's slot in the layout, and how many planet slots wide it is.
const BELT: usize = usize::MAX;
const BELT_W: f32 = 1.8;
/// Smallest gap between neighboring orbits, in units.
const MIN_GAP: f32 = 2.6;
/// Distance between orbit dots, in braille dots.
const ORBIT_SPACING: f32 = 3.2;
/// Brightness below which faint dots thin out instead of dimming further.
const FLOOR: f32 = 0.45;
/// Suns smaller than this (in units) are drawn as a fixed 4×2 sprite.
const SPRITE_R: f32 = 2.1;

const SUN_STOPS: [u32; 5] = [0x7a1d00, 0xff5a0a, 0xffa31a, 0xffe27a, 0xfffbea];
const BELT_SHADES: [u32; 3] = [0x8f8270, 0xa89a86, 0x6f665a];
const NATIVE_ORBIT: u32 = 0x56637f;
const NATIVE_RING: u32 = 0xcdb57e;

/// The persistent state of one planet; it survives resizes.
#[derive(Clone, Copy)]
struct Body {
    /// Mean anomaly.
    m: f32,
    /// Argument of periapsis.
    peri: f32,
    moon_phase: [f32; MAX_MOONS],
}

#[derive(Clone, Copy, Default)]
struct Slot {
    kind: usize,
    a: f32,
    b: f32,
    /// Mean motion in radians per second.
    n: f32,
    cos_p: f32,
    sin_p: f32,
}

#[derive(Clone, Copy)]
struct Rock {
    /// Radial position across the belt, 0..1.
    u: f32,
    theta: f32,
    shade: u8,
    bright: f32,
    /// Angular speed, set by the layout.
    w: f32,
    r: f32,
}

#[derive(Clone, Copy)]
struct Spark {
    angle: f32,
    off: f32,
    phase: f32,
    freq: f32,
}

pub struct Orrery {
    want: usize,
    speed: f32,
    show_orbits: bool,
    show_trails: bool,
    show_belt: bool,
    show_moons: bool,
    tilt: f32,
    /// The tilt actually used, adapted to the screen's shape.
    view: f32,
    bodies: [Body; 9],
    slots: Vec<Slot>,
    rocks: Vec<Rock>,
    n_rocks: usize,
    belt: (f32, f32),
    sparks: Vec<Spark>,
    n_sparks: usize,
    t: f32,
    /// Sun position and size in units.
    cx: f32,
    cy: f32,
    sun_r: f32,
    moon_unit: f32,
    ring_r: f32,
    /// Dotted orbit paths in dot coordinates, with the slot each belongs to.
    orbit_dots: Vec<(i16, i16, u8)>,
    br: Braille,
    planet_col: [Color; 9],
    orbit_col: [Color; 9],
    moon_col: [[Color; MAX_MOONS]; 9],
    ring_col: Color,
    belt_col: [Color; 3],
    sun: Gradient,
    /// The screen in units, for laying out again when the holiday comes or goes.
    size: (f32, f32),
    dm: Muertos,
}

impl Orrery {
    fn new(ctx: &Ctx) -> Orrery {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let bodies = std::array::from_fn(|_| Body {
            m: rng.f32() * TAU,
            peri: rng.f32() * TAU,
            moon_phase: std::array::from_fn(|_| rng.f32() * TAU),
        });
        let rocks = (0..MAX_ASTEROIDS)
            .map(|_| {
                // Denser toward the middle of the belt.
                let u = (rng.f32() + rng.f32() + rng.f32()) / 3.0;
                Rock {
                    u,
                    theta: rng.f32() * TAU,
                    shade: rng.u8(0..3),
                    bright: 0.55 + 0.45 * rng.f32(),
                    w: 0.0,
                    r: 0.0,
                }
            })
            .collect();
        let sparks = (0..160)
            .map(|_| Spark {
                angle: rng.f32() * TAU,
                off: rng.f32() * rng.f32(),
                phase: rng.f32() * TAU,
                freq: 1.5 + 4.0 * rng.f32(),
            })
            .collect();
        Orrery {
            want: o.i("planets").clamp(1, 9) as usize,
            speed: o.f("speed").clamp(0.0, 100.0),
            show_orbits: o.b("orbits"),
            show_trails: o.b("trails"),
            show_belt: o.b("belt"),
            show_moons: o.b("moons"),
            tilt: o.f("tilt").clamp(0.1, 1.0),
            view: 0.5,
            bodies,
            slots: Vec::with_capacity(9),
            rocks,
            n_rocks: 0,
            belt: (0.0, 0.0),
            sparks,
            n_sparks: 0,
            t: 0.0,
            cx: 0.0,
            cy: 0.0,
            sun_r: 1.0,
            moon_unit: 1.0,
            ring_r: 2.0,
            orbit_dots: Vec::new(),
            br: Braille::new(1, 1),
            planet_col: [Color::Reset; 9],
            orbit_col: [Color::Reset; 9],
            moon_col: [[Color::Reset; MAX_MOONS]; 9],
            ring_col: Color::Reset,
            belt_col: [Color::Reset; 3],
            sun: Gradient::new(&[]),
            size: (1.0, 2.0),
            dm: Muertos::new(ctx),
        }
    }

    /// Lays the system out to fit a `w`×`h` unit screen.
    fn layout(&mut self, w: f32, h: f32) {
        let margin = 2.0;
        // The tilt that would fill the screen exactly. Lean toward it: look
        // from higher up on tall screens, a bit flatter on very wide ones.
        let (hw, hh) = ((w * 0.5 - margin).max(0.5), (h * 0.5 - margin).max(0.5));
        let fill = hh / hw;
        self.view = if fill > self.tilt {
            fill.min(1.0)
        } else {
            fill.max(self.tilt * 0.7)
        };
        let t = self.view;
        // Largest semi-major axis whose projected ellipse still fits.
        let big = hw.min(hh / t).max(1.0);
        if self.dm.on {
            // The sun-calavera: a hand-drawn face a size up from the everyday
            // sun, centered on a cell so it comes out symmetric.
            let want = big * 0.075 * FACE_GROW;
            self.dm.face = FACES.iter().rposition(|f| want >= f.from);
            self.sun_r = self.dm.face.map_or(1.0, |f| FACES[f].r);
            self.cx = (w * 0.5).floor() + 0.5;
            self.cy = (h * 0.25).floor() * 2.0 + 1.0;
        } else {
            self.sun_r = (big * 0.075).clamp(0.9, 4.5);
            self.cx = w * 0.5;
            self.cy = h * 0.5;
        }
        if self.sun_r < SPRITE_R && !self.dm.on {
            if self.sun_r >= 1.2 {
                // The sprite's size, roughly.
                self.sun_r = 2.0;
            }
            // Small suns are a sprite or glyph, which needs a cell-aligned center.
            self.cx = self.cx.round();
            self.cy = (self.cy * 0.5).round() * 2.0;
        }
        self.moon_unit = (big * 0.04).clamp(1.3, 2.8);
        self.ring_r = (big * 0.06).clamp(2.0, 4.0);
        // The innermost orbit clears the sun even when seen nearly edge-on.
        let inner = (self.sun_r + (big * 0.1).clamp(2.2, 7.0)).max((self.sun_r + 1.2) / t);
        self.n_sparks =
            ((self.sun_r + 1.0) * TAU * 4.0).clamp(8.0, self.sparks.len() as f32) as usize;

        // Pick planets by priority until the orbits would get too crowded.
        let mut chosen = [false; 9];
        for &k in PRIORITY.iter().take(self.want) {
            chosen[k] = true;
            if !self.fits(&chosen, inner, big) && chosen.iter().filter(|&&c| c).count() > 1 {
                chosen[k] = false;
                break;
            }
        }
        // Slots from the inside out; a planet is one unit wide, the belt wider.
        let belt_at = if self.show_belt {
            self.belt_slot(&chosen)
        } else {
            None
        };
        let mut kinds = [BELT; 10];
        let mut n = 0;
        for k in (0..9).filter(|&k| chosen[k]) {
            if belt_at == Some(k) {
                n += 1;
            }
            kinds[n] = k;
            n += 1;
        }
        if belt_at == Some(9) {
            n += 1;
        }
        let width = |k: usize| if k == BELT { BELT_W } else { 1.0 };
        let mut centers = [0.0f32; 10];
        let mut pos = 0.0;
        for i in 1..n {
            pos += (width(kinds[i - 1]) + width(kinds[i])) * 0.5;
            centers[i] = pos;
        }
        let span = pos.max(1.0);
        let last_e = (0..n)
            .rev()
            .find(|&i| kinds[i] < 9)
            .map_or(0.0, |i| KINDS[kinds[i]].ecc);
        let outer = (big / (1.0 + last_e)).max(inner + 0.5);
        let radius = |c: f32| -> f32 {
            let u = if n > 1 { c / span } else { 0.5 };
            // Gaps widen a little outward, like the real thing.
            inner + (outer - inner) * (0.75 * u + 0.25 * u * u)
        };

        self.slots.clear();
        self.belt = (0.0, 0.0);
        let mut a_min = f32::MAX;
        for i in 0..n {
            let r = radius(centers[i]);
            if kinds[i] == BELT {
                // Half-width of the belt, from the gap to its neighbors.
                let step = (outer - inner) / span.max(1.0);
                self.belt = (r - step * 0.55, r + step * 0.55);
                continue;
            }
            let kd = &KINDS[kinds[i]];
            a_min = a_min.min(r);
            let body = &self.bodies[kinds[i]];
            self.slots.push(Slot {
                kind: kinds[i],
                a: r,
                b: r * (1.0 - kd.ecc * kd.ecc).sqrt(),
                n: 0.0,
                cos_p: body.peri.cos(),
                sin_p: body.peri.sin(),
            });
        }
        // Kepler's third law: the period grows with a^1.5.
        let a0 = if a_min < f32::MAX {
            a_min.max(1.0)
        } else {
            1.0
        };
        let base = TAU / 9.0;
        for s in &mut self.slots {
            s.n = base * (s.a / a0).powf(-1.5);
        }
        let (b0, b1) = self.belt;
        self.n_rocks = if b1 > b0 {
            // Roughly one rock per cell of screen.
            (((b0 + b1) * 0.5 * TAU) * (b1 - b0) * t * 0.7).clamp(20.0, MAX_ASTEROIDS as f32)
                as usize
        } else {
            0
        };
        for rk in &mut self.rocks[..self.n_rocks] {
            rk.r = b0 + (b1 - b0) * rk.u;
            rk.w = base * (rk.r / a0).powf(-1.5);
        }
        if self.dm.on {
            // Rays reach most of the way to the first orbit.
            let face_r = self.sun_r;
            self.dm.ray = (0.75 * face_r + 0.8).min((inner - face_r) * 0.8).max(0.8);
            self.dm
                .lay_rays(self.br.pw(), self.br.ph(), self.cx, self.cy, face_r);
            for c in &mut self.dm.comets {
                // Swinging close around the sun, out to well past the screen.
                let q = face_r + 1.0 + (inner - face_r) * (0.5 + 1.5 * c.near);
                let far = big * (1.25 + 0.5 * c.far);
                let a = (q + far) * 0.5;
                c.ecc = (far - q) / (far + q);
                c.orbit = Slot {
                    kind: 0,
                    a,
                    b: a * (1.0 - c.ecc * c.ecc).sqrt() * c.dir,
                    n: base * (a / a0).powf(-1.5) * COMET_HURRY,
                    cos_p: c.peri.cos(),
                    sin_p: c.peri.sin(),
                };
            }
        }

        // Dotted orbit paths: a dot every few dots of arc, evenly spaced on screen.
        self.orbit_dots.clear();
        for (si, s) in self.slots.iter().enumerate() {
            let ecc = KINDS[s.kind].ecc;
            let fine = ((s.a * TAU * 8.0) as usize).clamp(48, 20000);
            let mut prev = self.project(s, 0.0, ecc);
            let mut run = ORBIT_SPACING;
            for i in 0..fine {
                let p = self.project(s, i as f32 / fine as f32 * TAU, ecc);
                // Arc length in dots (a dot is half a unit).
                run += (p.0 - prev.0).hypot(p.1 - prev.1) * 2.0;
                prev = p;
                if run >= ORBIT_SPACING {
                    run -= ORBIT_SPACING;
                    self.orbit_dots
                        .push(((p.0 * 2.0) as i16, (p.1 * 2.0) as i16, si as u8));
                }
            }
        }
    }

    /// Whether the chosen planets fit between `inner` and `big` with room to spare.
    fn fits(&self, chosen: &[bool; 9], inner: f32, big: f32) -> bool {
        let n = chosen.iter().filter(|&&c| c).count() as f32;
        let belt = if self.show_belt && self.belt_slot(chosen).is_some() {
            BELT_W
        } else {
            0.0
        };
        let gaps = (n - 1.0 + belt).max(1.0);
        // The innermost gap is the narrowest (radii grow quadratically).
        (big / 1.12 - inner) * 0.75 / gaps >= MIN_GAP
    }

    /// Where the belt goes: before the first chosen giant (index 4+), or after
    /// the last rocky planet when there are no giants. `Some(9)` means "at the end".
    fn belt_slot(&self, chosen: &[bool; 9]) -> Option<usize> {
        let rocky = chosen[..4].iter().any(|&c| c);
        let giant = (4..9).find(|&k| chosen[k]);
        match (rocky, giant) {
            (_, Some(g)) => Some(g),
            (true, None) => Some(9),
            (false, None) => None,
        }
    }

    /// Screen position in units at eccentric anomaly `e`.
    #[inline]
    fn project(&self, s: &Slot, e: f32, ecc: f32) -> (f32, f32) {
        project(s, e, ecc, self.cx, self.cy, self.view)
    }

    fn ecc_anomaly(m: f32, e: f32) -> f32 {
        let mut x = m + e * m.sin();
        for _ in 0..4 {
            x -= (x - e * x.sin() - m) / (1.0 - e * x.cos());
        }
        x
    }

    /// Draws a dot whose brightness `v` fades toward the background; very
    /// faint dots are thinned out (by `seq`) rather than darkened.
    #[inline]
    fn faint_dot(br: &mut Braille, ctx: &Ctx, x: f32, y: f32, col: Color, v: f32, seq: usize) {
        if v < FLOOR {
            let stride = (FLOOR / v.max(1e-3)).round() as usize;
            if stride > 5 || !seq.is_multiple_of(stride) {
                return;
            }
        }
        br.setf(x, y, ctx.fade(col, 1.0 - v.clamp(FLOOR, 1.0)));
    }

    fn draw_sun(&self, c: &mut Canvas) {
        let r = self.sun_r;
        let (cx, cy) = (self.cx, self.cy);
        if r < 1.2 {
            c.set_attr(cx as i32, (cy * 0.5) as i32, '●', self.sun.at(0.95), BOLD);
            return;
        }
        if r < SPRITE_R {
            // A disc this small rasterizes into a plus sign; an octagon reads rounder.
            let (x, y) = (cx.round() as i32 - 2, (cy * 0.5).round() as i32 - 1);
            let (hot, rim) = (self.sun.at(0.97), self.sun.at(0.8));
            c.text(x, y, "▟██▙", hot);
            c.text(x, y + 1, "▜██▛", hot);
            for (dx, dy) in [(0, 0), (3, 0), (0, 1), (3, 1)] {
                if let Some(cell) = c.get_mut(x + dx, y + dy) {
                    cell.fg = rim;
                }
            }
            return;
        }
        let x0 = (cx - r - 1.0).floor() as i32;
        let x1 = (cx + r + 1.0).ceil() as i32;
        let y0 = ((cy - r) * 0.5 - 1.0).floor() as i32;
        let y1 = ((cy + r) * 0.5 + 1.0).ceil() as i32;
        let rr = r + 0.05;
        // Quadrant blocks: 2×2 solid sub-pixels per cell for a rounder disc.
        const QUAD: [char; 16] = [
            ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
        ];
        const SUB: [(f32, f32); 4] = [(0.25, 0.5), (0.75, 0.5), (0.25, 1.5), (0.75, 1.5)];
        for j in y0..=y1 {
            for i in x0..=x1 {
                let mut bits = 0;
                let mut near = f32::MAX;
                for (q, &(ox, oy)) in SUB.iter().enumerate() {
                    let d = (i as f32 + ox - cx).hypot((2 * j) as f32 + oy - cy);
                    if d < rr {
                        bits |= 1 << q;
                        near = near.min(d);
                    }
                }
                if bits == 0 {
                    continue;
                }
                let d = near / r;
                c.set(i, j, QUAD[bits], self.sun.at(1.0 - 0.45 * d * d));
            }
        }
    }

    fn draw_planet(&self, c: &mut Canvas, s: &Slot, x: f32, y: f32) {
        let kd = &KINDS[s.kind];
        let attr = if kd.bold { BOLD } else { 0 };
        c.set_attr(
            x.floor() as i32,
            (y * 0.5).floor() as i32,
            kd.glyph,
            self.planet_col[s.kind],
            attr,
        );
    }

    /// Which planets wear a marigold garland: more of them as the holiday
    /// builds up, the most iconic first.
    fn garlands(&self) -> [bool; 9] {
        let mut on = [false; 9];
        if !self.dm.on {
            return on;
        }
        let n = (self.slots.len() as f32 * self.dm.level).ceil() as usize;
        let here = |k: usize| self.slots.iter().any(|s| s.kind == k);
        for &k in PRIORITY.iter().filter(|&&k| here(k)).take(n.max(1)) {
            on[k] = true;
        }
        on
    }

    /// Moves the comets along and lets them shed petals.
    fn update_muertos(&mut self, flow: f32, dt: f32) {
        let dm = &mut self.dm;
        dm.t = (dm.t + flow).rem_euclid(1000.0 * TAU);
        let live = dm.comets_live();
        let (w, h) = self.size;
        for (i, c) in dm.comets.iter_mut().enumerate() {
            let s = c.orbit;
            c.m = (c.m + s.n * dt).rem_euclid(TAU);
            let e = kepler(c.m, c.ecc);
            let (x, y) = project(&s, e, c.ecc, self.cx, self.cy, self.view);
            // Velocity from the derivative of the orbit: dE/dt = n / (1 - e cos E).
            let rate = s.n / (1.0 - c.ecc * e.cos()).max(0.05);
            let (se, ce) = e.sin_cos();
            let (dx, dz) = (-s.a * se * rate, -s.b * ce * rate);
            let vx = dx * s.cos_p - dz * s.sin_p;
            let vy = (dx * s.sin_p + dz * s.cos_p) * self.view;
            c.pos = (x, y);
            // Only comets near the screen shed petals, the more the nearer the sun.
            let near = x > -4.0 && x < w + 4.0 && y > -8.0 && y < h + 8.0;
            if i >= live || !near {
                c.owed = 0.0;
                continue;
            }
            let (rx, ry) = (x - self.cx, y - self.cy);
            let dist = rx.hypot(ry).max(0.5);
            let heat = (self.sun_r * 2.5 / dist).clamp(0.15, 1.5);
            c.owed = (c.owed + flow * PETAL_RATE * heat).min(40.0);
            while c.owed >= 1.0 && dm.petals.len() < MAX_PETALS {
                c.owed -= 1.0;
                let rng = &mut dm.rng;
                // Blown away from the sun, keeping a little of the comet's drift.
                let push = 1.2 + 2.2 * rng.f32() * heat.sqrt();
                let spread = (rng.f32() - 0.5) * 0.9;
                let (ux, uy) = (rx / dist, ry / dist);
                dm.petals.push(Petal {
                    x: x + (rng.f32() - 0.5) * 0.8,
                    y: y + (rng.f32() - 0.5) * 0.8,
                    vx: (ux - uy * spread) * push + vx * 0.25,
                    vy: (uy + ux * spread) * push + vy * 0.25,
                    age: 0.0,
                    life: 2.2 + 2.4 * rng.f32(),
                    spin: rng.f32() * TAU,
                    spin_rate: 3.0 + 5.0 * rng.f32(),
                    shade: if rng.f32() < 0.12 { 3 } else { rng.u8(0..3) },
                });
            }
        }
        let drag = (-0.6 * flow).exp();
        let mut i = 0;
        while i < dm.petals.len() {
            let p = &mut dm.petals[i];
            p.age += flow;
            if p.age >= p.life {
                dm.petals.swap_remove(i);
                continue;
            }
            p.x += p.vx * flow;
            p.y += p.vy * flow;
            p.vx *= drag;
            p.vy *= drag;
            p.spin = (p.spin + p.spin_rate * flow).rem_euclid(TAU);
            i += 1;
        }
    }
}

impl Scene for Orrery {
    fn resize(&mut self, ctx: &Ctx, w: u16, h: u16) {
        self.br.resize(w, h);
        self.dm.set(ctx.fiesta);
        self.size = (w.max(1) as f32, h.max(1) as f32 * 2.0);
        self.layout(self.size.0, self.size.1);
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let native = ctx.theme.native;
        let g = ctx.ramp(&[]);
        self.sun = if native {
            Gradient::new(&SUN_STOPS.map(rgb))
        } else {
            // The theme accent closest to sunlight, glowing up into the ramp's top.
            let warm = warmest(&ctx.theme.colors).unwrap_or(g.at(0.8));
            Gradient::new(&[
                g.at(0.3),
                mix_oklab(g.at(0.5), warm, 0.6),
                warm,
                mix_oklab(warm, g.at(1.0), 0.5),
                g.at(1.0),
            ])
        };
        let orbit_base = if native {
            rgb(NATIVE_ORBIT)
        } else {
            g.at(0.45)
        };
        for (i, kd) in KINDS.iter().enumerate() {
            let pc = ctx.pick(i, rgb(kd.color));
            self.planet_col[i] = pc;
            self.orbit_col[i] = ctx.fade(mix_oklab(pc, orbit_base, 0.6), 0.35);
            for (m, mn) in kd.moons.iter().enumerate().take(MAX_MOONS) {
                self.moon_col[i][m] = if native {
                    rgb(mn.color)
                } else {
                    mix_oklab(pc, g.at(1.0), 0.5 + 0.1 * m as f32)
                };
            }
        }
        self.ring_col = if native {
            rgb(NATIVE_RING)
        } else {
            mix_oklab(self.planet_col[5], g.at(0.8), 0.4)
        };
        for (i, &hex) in BELT_SHADES.iter().enumerate() {
            self.belt_col[i] = if native {
                rgb(hex)
            } else {
                ctx.fade(g.at(0.6 + 0.12 * i as f32), 0.3)
            };
        }
        // The holiday came or went: the sun changes size, so lay out again.
        let was = self.dm.on;
        self.dm.set(ctx.fiesta);
        if self.dm.on != was {
            self.layout(self.size.0, self.size.1);
        }
        self.dm.retheme(ctx);
    }

    /// The wink is a pause between winks, so it keeps the wall clock.
    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        if self.dm.on && real.is_finite() {
            self.dm.wall = (self.dm.wall + real.clamp(0.0, 30.0)).rem_euclid(WINK_EVERY);
        }
        self.update(ctx, dt);
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        // Petals and rays move with the pace of the app, not the orbits' own speed.
        let flow = if dt.is_finite() {
            dt.clamp(0.0, 30.0)
        } else {
            0.0
        };
        let dt = flow * self.speed;
        if self.dm.on {
            self.update_muertos(flow, dt);
        }
        self.t = (self.t + dt).rem_euclid(1000.0 * TAU);
        for s in &self.slots {
            let b = &mut self.bodies[s.kind];
            b.m = (b.m + s.n * dt).rem_euclid(TAU);
            for (m, mn) in KINDS[s.kind].moons.iter().enumerate().take(MAX_MOONS) {
                b.moon_phase[m] = (b.moon_phase[m] + TAU / mn.period * dt).rem_euclid(TAU);
            }
        }
        for rk in &mut self.rocks[..self.n_rocks] {
            rk.theta = (rk.theta + rk.w * dt).rem_euclid(TAU);
        }
        for sp in &mut self.sparks {
            sp.phase = (sp.phase + sp.freq * dt).rem_euclid(TAU);
            sp.angle = (sp.angle + 0.12 * dt).rem_euclid(TAU);
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        self.br.clear();
        let t = self.view;

        if self.show_orbits {
            // On Día de Muertos the sun's rays outshine the orbits close to it.
            let glare = if self.dm.on {
                (self.dm.reach(self.sun_r) * 2.0).powi(2)
            } else {
                0.0
            };
            let (gx, gy) = (self.cx * 2.0, self.cy * 2.0);
            for &(x, y, si) in &self.orbit_dots {
                if glare > 0.0 {
                    let (dx, dy) = (x as f32 - gx, y as f32 - gy);
                    if dx * dx + dy * dy < glare {
                        continue;
                    }
                }
                let kind = self.slots[si as usize].kind;
                self.br.set(x as i32, y as i32, self.orbit_col[kind]);
            }
        }

        if self.show_belt {
            for (i, rk) in self.rocks[..self.n_rocks].iter().enumerate() {
                let (s, co) = rk.theta.sin_cos();
                let x = self.cx + rk.r * co;
                let y = self.cy - rk.r * s * t;
                // On Día de Muertos more and more of the belt is marigold petals.
                let base = self
                    .dm
                    .belt_petal(i, rk.shade)
                    .unwrap_or(self.belt_col[rk.shade as usize]);
                let col = ctx.fade(base, 1.0 - rk.bright);
                self.br.setf(x * 2.0, y * 2.0, col);
            }
        }

        // Corona: flickering sparks around the sun's rim, or the sun-calavera's rays.
        let r = self.sun_r;
        if self.dm.on {
            self.dm.draw_rays(&mut self.br, r);
        }
        let n_sparks = if self.dm.on { 0 } else { self.n_sparks };
        for (i, sp) in self.sparks[..n_sparks].iter().enumerate() {
            let v = 0.5 + 0.5 * sp.phase.sin();
            let reach = r * 0.5 + 1.2;
            let d = r + 0.2 + sp.off * reach;
            let (s, co) = sp.angle.sin_cos();
            let fall = 1.0 - sp.off;
            let col = self.sun.at(0.45 + 0.4 * fall * v);
            Self::faint_dot(
                &mut self.br,
                ctx,
                (self.cx + co * d) * 2.0,
                (self.cy + s * d) * 2.0,
                col,
                v * fall + 0.15,
                i,
            );
        }

        // Planet positions this frame.
        let mut pos = [(0.0f32, 0.0f32, 0.0f32); 9];
        for (si, s) in self.slots.iter().enumerate() {
            let kd = &KINDS[s.kind];
            let e = Self::ecc_anomaly(self.bodies[s.kind].m, kd.ecc);
            let (x, y) = self.project(s, e, kd.ecc);
            pos[si] = (x, y, e);
        }

        if self.show_trails {
            for (si, s) in self.slots.iter().enumerate() {
                let kd = &KINDS[s.kind];
                let e = pos[si].2;
                // A trail of roughly constant length, longer for big orbits.
                let len = (s.a * 0.45).clamp(3.0, 12.0);
                let span = (len / s.a).min(1.2);
                let steps = (len * 2.5) as usize + 2;
                let col = self.planet_col[s.kind];
                for k in 1..=steps {
                    let f = k as f32 / steps as f32;
                    let (x, y) = self.project(s, e - span * f, kd.ecc);
                    let v = 0.75 * (1.0 - f).powf(1.3);
                    Self::faint_dot(&mut self.br, ctx, x * 2.0, y * 2.0, col, v, k);
                }
            }
        }

        if self.dm.on {
            self.dm.draw_tails(&mut self.br, ctx);
        }

        // Saturn's ring and the moons; on Día de Muertos, garlands of marigolds.
        let garlands = self.garlands();
        for (si, s) in self.slots.iter().enumerate() {
            let kd = &KINDS[s.kind];
            // Rings and moons circle the glyph's cell center, so they move with it.
            let px = pos[si].0.floor() + 0.5;
            let py = (pos[si].1 * 0.5).floor() * 2.0 + 1.0;
            if garlands[s.kind] {
                self.dm
                    .draw_garland(&mut self.br, px, py, self.ring_r, kd.ring, t, si);
            } else if kd.ring {
                let rx = self.ring_r;
                let ry = rx * (0.12 + 0.25 * t);
                let tip: f32 = -0.2;
                let (st, ct) = tip.sin_cos();
                let n = (rx * 9.0) as usize;
                for k in 0..n {
                    let a = k as f32 / n as f32 * TAU;
                    let (sa, ca) = a.sin_cos();
                    let (ex, ey) = (ca * rx, sa * ry);
                    let x = px + ex * ct - ey * st;
                    let y = py + ex * st + ey * ct;
                    self.br.setf(x * 2.0, y * 2.0, self.ring_col);
                }
            }
            if self.show_moons {
                let b = &self.bodies[s.kind];
                for (m, mn) in kd.moons.iter().enumerate().take(MAX_MOONS) {
                    let rr = mn.r * self.moon_unit + if kd.ring { self.ring_r * 0.6 } else { 0.0 };
                    let (sa, ca) = b.moon_phase[m].sin_cos();
                    let x = ((px + ca * rr) * 2.0).floor() as i32;
                    let y = ((py - sa * rr * t) * 2.0).floor() as i32;
                    // A moon owns its braille cell, so orbit dots there do not take its color.
                    let (bx, by) = (x & !1, y & !3);
                    for dy in 0..4 {
                        self.br.unset(bx, by + dy);
                        self.br.unset(bx + 1, by + dy);
                    }
                    self.br.set(x, y, self.moon_col[s.kind][m]);
                }
            }
        }

        self.br.blit(c);

        // Far side first, then the sun, then the near side.
        for (si, s) in self.slots.iter().enumerate() {
            let (x, y, _) = pos[si];
            if y < self.cy {
                self.draw_planet(c, s, x, y);
            }
        }
        if self.dm.on {
            self.dm.draw_heads(c, self.cy, true);
            self.dm.draw_face(c, self.cx, self.cy);
        } else {
            self.draw_sun(c);
        }
        for (si, s) in self.slots.iter().enumerate() {
            let (x, y, _) = pos[si];
            if y >= self.cy {
                self.draw_planet(c, s, x, y);
            }
        }
        if self.dm.on {
            self.dm.draw_heads(c, self.cy, false);
        }
    }
}

/// Screen position in units at eccentric anomaly `e` on an orbit around (`cx`, `cy`).
#[inline]
fn project(s: &Slot, e: f32, ecc: f32, cx: f32, cy: f32, view: f32) -> (f32, f32) {
    let (se, ce) = e.sin_cos();
    let x = s.a * (ce - ecc);
    let z = -s.b * se;
    let xr = x * s.cos_p - z * s.sin_p;
    let zr = x * s.sin_p + z * s.cos_p;
    (cx + xr, cy + zr * view)
}

// Día de Muertos.

const CEMPASUCHIL: u32 = 0xff8c00;
const CEMPASUCHIL_LIGHT: u32 = 0xffaa1e;
const CEMPASUCHIL_DARK: u32 = 0xd66200;
const TERCIOPELO: u32 = 0xb00048;
const ROSA: u32 = 0xe4007c;
/// Morado, lightened to show on a dark eye socket.
const MORADO: u32 = 0x9a50d0;
const TURQUESA: u32 = 0x00aab8;
const AMARILLO: u32 = 0xffd200;
const HUESO: u32 = 0xf4eee2;

/// Seeds the holiday's own random numbers, so ordinary days draw the same ones as ever.
const MUERTOS_SEED: u64 = 0x6d75_6572_746f_7321;
/// How much bigger the sun-calavera is than the everyday sun.
const FACE_GROW: f32 = 1.45;
const MAX_COMETS: usize = 3;
/// Comets go round faster than Kepler would have them, so they come back often.
const COMET_HURRY: f32 = 1.6;
/// The calavera winks for the last `WINK_FOR` seconds of every `WINK_EVERY` (wall clock).
const WINK_FOR: f32 = 0.45;
const WINK_EVERY: f32 = 11.0;
const MAX_PETALS: usize = 400;
/// Petals a comet sheds per second at its brightest.
const PETAL_RATE: f32 = 40.0;

/// Icing colors, indices into `Muertos::icing`.
const TURQ: u8 = 0;
const ROSE: u8 = 1;
const PLUM: u8 = 2;

/// A sun-calavera drawn by hand in quadrant pixels, two by two to a cell:
/// `o` is the sun's golden rim, `b` sugar-white bone, `.` a hole in the face
/// (eye sockets, nose, the gaps between the teeth) and a space is sky. Faces
/// are an odd number of cells across and down, so they sit centered on one.
struct Face {
    rows: &'static [&'static str],
    /// Icing drawn as glyphs over whole cells: x, y, glyph, color, from which level on.
    icing: &'static [(u8, u8, char, u8, f32)],
    /// Radius in units, for the layout and the rays.
    r: f32,
    /// The smallest sun (in units, as the everyday sun would be grown by
    /// `FACE_GROW`) that wears this face.
    from: f32,
}

const FACES: [Face; 4] = [
    Face {
        rows: &[
            "  bbbbbbbbbb  ",
            " bbbbbbbbbbbb ",
            "bbbb..bb..bbbb",
            " bbb..  ..bbb ",
            "  b.b.bb.b.b  ",
            "   bbbbbbbb   ",
        ],
        icing: &[(2, 1, '✿', TURQ, 0.0), (4, 1, '✿', ROSE, 0.0)],
        r: 3.25,
        from: 2.6,
    },
    Face {
        rows: &[
            "     oooooooo     ",
            "  oooobbbbbboooo  ",
            " oobbbbbbbbbbbboo ",
            "oobb..bbbbbb..bboo",
            "oob....bbbb....boo",
            "oob....bbbb....boo",
            "oobb..bb..bb..bboo",
            " oobbbbbbbbbbbboo ",
            "  oob.b.bb.b.boo  ",
            "   obbbbbbbbbbo   ",
        ],
        icing: &[
            (2, 2, '✿', TURQ, 0.0),
            (6, 2, '✿', ROSE, 0.0),
            (4, 1, '❀', PLUM, 0.0),
        ],
        r: 4.75,
        from: 4.0,
    },
    Face {
        rows: &[
            "        oooooooooo        ",
            "     oooooobbbboooooo     ",
            "   oooobbbbbbbbbbbboooo   ",
            " oooobbbbbbbbbbbbbbbboooo ",
            " ooobbbbbbbbbbbbbbbbbbooo ",
            "ooobb....bbbbbbbb....bbooo",
            "ooob......bbbbbb......booo",
            "ooob......bbbbbb......booo",
            "ooobb....bb....bb....bbooo",
            " ooobbbbbbbb..bbbbbbbbooo ",
            " oooob.bbbbbbbbbbbb.boooo ",
            "   ooob.b.b.bb.b.b.booo   ",
            "     ooobbbbbbbbbbooo     ",
            "        oooooooooo        ",
        ],
        icing: &[
            (3, 3, '✿', TURQ, 0.0),
            (9, 3, '✿', ROSE, 0.0),
            (6, 1, '❀', PLUM, 0.0),
            (5, 1, '•', TURQ, 0.6),
            (7, 1, '•', TURQ, 0.6),
        ],
        r: 6.75,
        from: 6.0,
    },
    Face {
        rows: &[
            "          oooooooooooooo          ",
            "       oooooooooooooooooooo       ",
            "     oooooobbbbbbbbbbbboooooo     ",
            "   ooooobbbbbbbbbbbbbbbbbbooooo   ",
            "  oooobbbbbbbbbbbbbbbbbbbbbboooo  ",
            " oooobbbbbbbbbbbbbbbbbbbbbbbboooo ",
            "oooobbb....bbbbbbbbbbbb....bbboooo",
            "oooobb......bbbbbbbbbb......bboooo",
            "ooobb........bbbbbbbb........bbooo",
            "ooobb........bbbbbbbb........bbooo",
            "ooobbb......bbb....bbb......bbbooo",
            "oooobbb....bbbbb..bbbbb....bbboooo",
            " ooobbbb.bbbbbbbbbbbbbbbb.bbbbooo ",
            " oooobbbb.b.b.b.bb.b.b.b.bbbboooo ",
            "  oooobbb.b.b.b.bb.b.b.b.bbboooo  ",
            "   ooooobbbbbbbbbbbbbbbbbbooooo   ",
            "       oooooooooooooooooooo       ",
            "          oooooooooooooo          ",
        ],
        icing: &[
            (4, 4, '✿', TURQ, 0.0),
            (12, 4, '✿', ROSE, 0.0),
            (8, 2, '❀', PLUM, 0.0),
            (6, 2, '•', TURQ, 0.6),
            (10, 2, '•', TURQ, 0.6),
            (3, 6, '•', ROSE, 0.8),
            (13, 6, '•', ROSE, 0.8),
        ],
        r: 8.75,
        from: 8.2,
    },
];

/// A braille dot in the ring around the sun-calavera, in dots, with how far
/// from its center (in units) and at which angle it lies.
#[derive(Clone, Copy)]
struct RayDot {
    x: i32,
    y: i32,
    d: f32,
    angle: f32,
}

/// A comet on a long, eccentric orbit.
#[derive(Clone, Copy, Default)]
struct Comet {
    /// Drawn once: how close it swings by the sun and how far out it goes (0..1),
    /// which way round it goes (±1) and where its perihelion points.
    near: f32,
    far: f32,
    dir: f32,
    peri: f32,
    /// Mean anomaly.
    m: f32,
    ecc: f32,
    /// Its orbit, from the layout (`b` is negative when it goes the other way round).
    orbit: Slot,
    /// Where its head is now, in units.
    pos: (f32, f32),
    /// Petals due but not yet shed.
    owed: f32,
}

/// A marigold petal shed by a comet, in units.
#[derive(Clone, Copy)]
struct Petal {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
    /// Tumbling: which way it shows (flat or edge-on) and how fast it turns.
    spin: f32,
    spin_rate: f32,
    /// Index into `Muertos::marigold`.
    shade: u8,
}

/// What Día de Muertos brings to the orrery. Nothing here runs on ordinary days,
/// and it has random numbers of its own.
struct Muertos {
    on: bool,
    level: f32,
    rng: fastrand::Rng,
    /// Seconds of holiday, for the turning rays and the tumbling petals.
    t: f32,
    /// Wall-clock seconds into the wink cycle.
    wall: f32,
    /// The face the sun wears (an index into `FACES`), or a single glyph when tiny.
    face: Option<usize>,
    /// Length of the long rays in units.
    ray: f32,
    /// The dots the rays can fall on, from the layout.
    ring: Vec<RayDot>,
    comets: [Comet; MAX_COMETS],
    petals: Vec<Petal>,
    bone: Color,
    glow: Color,
    rim: [Color; 2],
    /// Straight rays, wavy rays.
    rays: [Color; 2],
    icing: [Color; 3],
    /// Dark, middle and light marigold, and a velvety terciopelo.
    marigold: [Color; 4],
}

impl Muertos {
    fn new(ctx: &Ctx) -> Muertos {
        let mut rng = fastrand::Rng::with_seed(ctx.seed ^ MUERTOS_SEED);
        let comets = std::array::from_fn(|i| Comet {
            near: rng.f32(),
            far: rng.f32(),
            dir: if rng.bool() { 1.0 } else { -1.0 },
            peri: rng.f32() * TAU,
            // Spread around their orbits, so they come by one at a time.
            m: (i as f32 / MAX_COMETS as f32 + 0.2 * rng.f32()) * TAU,
            ..Comet::default()
        });
        Muertos {
            on: false,
            level: 0.0,
            rng,
            t: 0.0,
            wall: 0.0,
            face: None,
            ray: 1.0,
            ring: Vec::new(),
            comets,
            petals: Vec::with_capacity(MAX_PETALS),
            bone: Color::Reset,
            glow: Color::Reset,
            rim: [Color::Reset; 2],
            rays: [Color::Reset; 2],
            icing: [Color::Reset; 3],
            marigold: [Color::Reset; 4],
        }
    }

    fn set(&mut self, fiesta: Fiesta) {
        self.on = fiesta.is(crate::fiesta::Kind::Muertos);
        self.level = if self.on {
            fiesta.level.clamp(0.0, 1.0)
        } else {
            0.0
        };
        if !self.on {
            self.petals.clear();
        }
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let native = ctx.theme.native;
        let g = ctx.ramp(&[]);
        // Under other themes each holiday color becomes the theme's nearest accent.
        let tint = |hex: u32| {
            if native {
                rgb(hex)
            } else {
                nearest(&ctx.theme.colors, hex).unwrap_or(g.at(0.8))
            }
        };
        // The sun's gold is the theme's sunlight, as on ordinary days.
        let gold = if native {
            rgb(CEMPASUCHIL)
        } else {
            warmest(&ctx.theme.colors).unwrap_or(g.at(0.8))
        };
        self.bone = if native { rgb(HUESO) } else { g.at(1.0) };
        self.glow = tint(AMARILLO);
        self.rim = if native {
            [rgb(CEMPASUCHIL_LIGHT), rgb(CEMPASUCHIL)]
        } else {
            [mix_oklab(gold, self.glow, 0.4), gold]
        };
        self.rays = [gold, tint(ROSA)];
        self.icing = [tint(TURQUESA), tint(ROSA), tint(MORADO)];
        self.marigold = if native {
            [CEMPASUCHIL_DARK, CEMPASUCHIL, CEMPASUCHIL_LIGHT, TERCIOPELO].map(rgb)
        } else {
            [
                mix_oklab(gold, g.at(0.3), 0.3),
                gold,
                mix_oklab(gold, self.glow, 0.5),
                tint(TERCIOPELO),
            ]
        };
    }

    /// How many comets are out: one at first, three on November 2.
    fn comets_live(&self) -> usize {
        match self.level {
            l if l >= 0.8 => 3,
            l if l >= 0.45 => 2,
            _ => 1,
        }
    }

    /// The color of the `i`th rock of the belt when it is a petal: more of
    /// them as the holiday builds up, one in ten a velvety terciopelo. `None`
    /// when it stays a rock.
    fn belt_petal(&self, i: usize, shade: u8) -> Option<Color> {
        if !self.on {
            return None;
        }
        let share = 0.1 + 0.45 * self.level;
        let p = hash01(i as u32);
        (p < share).then(|| self.marigold[if p < share * 0.1 { 3 } else { shade as usize }])
    }

    /// How far the rays reach from the sun's center, in units.
    fn reach(&self, r: f32) -> f32 {
        r + 0.3 + self.ray * (0.8 + 0.2 * self.level)
    }

    /// Finds the dots of the ring around the face where the rays can fall, once
    /// the layout is known, so that drawing them takes no trigonometry.
    fn lay_rays(&mut self, pw: i32, ph: i32, cx: f32, cy: f32, r: f32) {
        self.ring.clear();
        // Out as far as the rays reach at the height of the holiday.
        let (r0, r1) = (r + 0.3, r + 0.3 + self.ray);
        let x0 = (((cx - r1) * 2.0).floor() as i32).max(0);
        let x1 = (((cx + r1) * 2.0).ceil() as i32).min(pw - 1);
        let y0 = (((cy - r1) * 2.0).floor() as i32).max(0);
        let y1 = (((cy + r1) * 2.0).ceil() as i32).min(ph - 1);
        for y in y0..=y1 {
            let uy = (y as f32 + 0.5) * 0.5 - cy;
            for x in x0..=x1 {
                let ux = (x as f32 + 0.5) * 0.5 - cx;
                let d2 = ux * ux + uy * uy;
                if d2 >= r0 * r0 && d2 <= r1 * r1 {
                    self.ring.push(RayDot {
                        x,
                        y,
                        d: d2.sqrt(),
                        angle: uy.atan2(ux),
                    });
                }
            }
        }
    }

    /// The rays around the sun-calavera, turning slowly: straight ones in
    /// cempasúchil, wavy ones in rosa mexicano between them.
    fn draw_rays(&self, br: &mut Braille, r: f32) {
        let n = match self.face {
            Some(f) if f >= 2 => 16,
            Some(_) => 12,
            None => 8,
        };
        let turn = self.t * 0.07;
        let long = self.reach(r) - r - 0.3;
        let short = long * 0.65;
        // A ripple of light runs round the rays.
        let mut cols = [Color::Reset; 16];
        for (k, col) in cols.iter_mut().enumerate().take(n) {
            let lit = 0.5 + 0.5 * (self.t * 1.7 - k as f32 * 0.8).sin();
            *col = self.rays[k % 2].lerp(self.glow, 0.3 * lit);
        }
        // Every dot of the ring around the face, tested against the ray it
        // falls under: a straight spike, or a thin flame that sways.
        let (r0, per) = (r + 0.3, n as f32 / TAU);
        for dot in &self.ring {
            let into = dot.d - r0;
            if into > long {
                continue;
            }
            // Which ray, and how far off its middle (in ray spacings).
            let rel = (dot.angle - turn) * per;
            let k = rel.round();
            let off = rel - k;
            let k = (k as i32).rem_euclid(n as i32) as usize;
            let inside = if k.is_multiple_of(2) {
                off.abs() < 0.4 * (1.0 - into / long)
            } else {
                let f = into / short;
                let sway = 0.13 * (f * 5.0 - self.t * 3.0 + k as f32).sin() * f;
                f < 1.0 && (off - sway).abs() < 0.14 * (1.0 - 0.5 * f)
            };
            if inside {
                br.set(dot.x, dot.y, cols[k]);
            }
        }
    }

    /// A ring of marigolds around a planet (Saturn's own ring turns into one),
    /// its petals slowly going round.
    #[allow(clippy::too_many_arguments)]
    fn draw_garland(
        &self,
        br: &mut Braille,
        px: f32,
        py: f32,
        ring_r: f32,
        saturn: bool,
        view: f32,
        si: usize,
    ) {
        let rx = if saturn {
            ring_r
        } else {
            (ring_r * 0.62).max(1.4)
        };
        let ry = rx * (0.12 + 0.25 * view);
        let tip: f32 = if saturn { -0.2 } else { 0.15 };
        let (st, ct) = tip.sin_cos();
        let n = (rx * 9.0) as usize;
        let turn = self.t * 0.5 + si as f32;
        // Two rows of petals make a band thick enough to read as flowers.
        for (band, k_r) in [1.0, 0.8].into_iter().enumerate() {
            for k in 0..n {
                let a = (k as f32 + 0.5 * band as f32) / n as f32 * TAU + turn;
                let (sa, ca) = a.sin_cos();
                let (ex, ey) = (ca * rx * k_r, sa * ry * k_r);
                let x = px + ex * ct - ey * st;
                let y = py + ex * st + ey * ct;
                // Blossoms of light, middle and dark petals, now and then a terciopelo.
                let col = match (k + si) % 7 {
                    0 | 3 => self.marigold[2],
                    1 | 4 => self.marigold[1],
                    5 if (k / 7 + si).is_multiple_of(3) => self.marigold[3],
                    _ => self.marigold[0],
                };
                br.setf(x * 2.0, y * 2.0, col);
            }
        }
    }

    /// The comets' tails: petals tumbling away from the sun, and a glow
    /// around each head.
    fn draw_tails(&self, br: &mut Braille, ctx: &Ctx) {
        for p in &self.petals {
            let k = p.age / p.life;
            let col = ctx.fade(self.marigold[p.shade as usize], 0.85 * k * k);
            let (x, y) = ((p.x * 2.0).floor() as i32, (p.y * 2.0).floor() as i32);
            br.set(x, y, col);
            // A young petal shows two dots, flat or edge-on as it tumbles.
            if k < 0.6 {
                if p.spin.sin() > 0.0 {
                    br.set(x + 1, y, col);
                } else {
                    br.set(x, y + 1, col);
                }
            }
        }
        for (i, c) in self.comets[..self.comets_live()].iter().enumerate() {
            let (x, y) = c.pos;
            for k in 0..6 {
                let a = k as f32 / 6.0 * TAU + self.t * 1.3 + i as f32;
                let d = 0.75 + 0.25 * (self.t * 4.0 + k as f32 * 2.0).sin();
                br.setf((x + a.cos() * d) * 2.0, (y + a.sin() * d) * 2.0, self.glow);
            }
        }
    }

    /// The comets' heads on the far (`far`) or near side of the sun.
    fn draw_heads(&self, c: &mut Canvas, cy: f32, far: bool) {
        for cm in &self.comets[..self.comets_live()] {
            let (x, y) = cm.pos;
            if (y < cy) == far {
                c.set_attr(
                    x.floor() as i32,
                    (y * 0.5).floor() as i32,
                    '✿',
                    self.glow,
                    BOLD,
                );
            }
        }
    }

    /// The sun-calavera, centered on (`cx`, `cy`) in units.
    fn draw_face(&self, c: &mut Canvas, cx: f32, cy: f32) {
        let (col, row) = (cx.floor() as i32, (cy * 0.5).floor() as i32);
        let Some(f) = self.face else {
            c.set_attr(col, row, '☻', self.bone, BOLD);
            return;
        };
        let face = &FACES[f];
        let (sw, sh) = (face.rows[0].len() / 2, face.rows.len() / 2);
        let (x0, y0) = (col - sw as i32 / 2, row - sh as i32 / 2);
        const QUAD: [char; 16] = [
            ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
        ];
        for j in 0..sh {
            let (top, bottom) = (face.rows[2 * j].as_bytes(), face.rows[2 * j + 1].as_bytes());
            for i in 0..sw {
                let px = [top[2 * i], top[2 * i + 1], bottom[2 * i], bottom[2 * i + 1]];
                let (mut bits, mut bone, mut rim) = (0, 0, 0);
                for (q, &p) in px.iter().enumerate() {
                    match p {
                        b'b' => bone += 1,
                        b'o' => rim += 1,
                        _ => continue,
                    }
                    bits |= 1 << q;
                }
                if bits == 0 {
                    continue;
                }
                // Bone warms toward the rim; the rim deepens outward.
                let u = (i as f32 + 0.5) / sw as f32 * 2.0 - 1.0;
                let v = (j as f32 + 0.5) / sh as f32 * 2.0 - 1.0;
                let d = u.hypot(v);
                let fg = if bone >= rim {
                    self.bone.lerp(self.glow, (d - 0.5) * 1.6)
                } else {
                    self.rim[0].lerp(self.rim[1], (d - 0.75) * 4.0)
                };
                c.set(x0 + i as i32, y0 + j as i32, QUAD[bits], fg);
            }
        }
        // Now and then the calavera winks its rosa eye.
        let wink = self.wall > WINK_EVERY - WINK_FOR;
        for &(i, j, ch, k, from) in face.icing {
            if self.level >= from {
                let ch = if wink && ch == '✿' && k == ROSE {
                    '‿'
                } else {
                    ch
                };
                c.set_attr(
                    x0 + i as i32,
                    y0 + j as i32,
                    ch,
                    self.icing[k as usize],
                    BOLD,
                );
            }
        }
    }
}

/// Solves Kepler's equation for the eccentric anomaly, robust up to e ≈ 0.99.
fn kepler(m: f32, e: f32) -> f32 {
    let m = (m + std::f32::consts::PI).rem_euclid(TAU) - std::f32::consts::PI;
    let mut x = m + 0.85 * e * if m >= 0.0 { 1.0 } else { -1.0 };
    for _ in 0..8 {
        x -= (x - e * x.sin() - m) / (1.0 - e * x.cos());
    }
    x
}

/// A stable pseudo-random number in 0..1 for `i`.
#[inline]
fn hash01(i: u32) -> f32 {
    (i.wrapping_mul(0x9e37_79b1)
        .rotate_right(13)
        .wrapping_mul(0x85eb_ca6b)
        >> 8) as f32
        / (1u32 << 24) as f32
}

/// The color in `colors` nearest in hue to `hex`, passing over grays.
fn nearest(colors: &[Color], hex: u32) -> Option<Color> {
    let want = hue(rgb(hex))?.0;
    colors
        .iter()
        .filter_map(|&c| {
            let (h, sat) = hue(c)?;
            let d = (h - want).abs();
            let d = d.min(360.0 - d);
            // A gray has no hue to speak of.
            Some((d + (0.35 - sat).max(0.0) * 400.0, c))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

/// Hue in degrees and saturation (0..1) of an RGB color.
fn hue(c: Color) -> Option<(f32, f32)> {
    let Color::Rgb(r, g, b) = c else {
        return None;
    };
    let (r, g, b) = (r as f32, g as f32, b as f32);
    let (hi, lo) = (r.max(g).max(b), r.min(g).min(b));
    let span = hi - lo;
    if hi <= 0.0 || span <= 0.0 {
        return Some((0.0, 0.0));
    }
    let h = if hi == r {
        ((g - b) / span).rem_euclid(6.0)
    } else if hi == g {
        (b - r) / span + 2.0
    } else {
        (r - g) / span + 4.0
    };
    Some((h * 60.0, span / hi))
}

/// The color in `colors` nearest to a warm sunlight yellow.
fn warmest(colors: &[Color]) -> Option<Color> {
    let score = |c: &Color| match *c {
        Color::Rgb(r, g, b) => {
            let d = |x: u8, y: i32| (x as i32 - y).pow(2);
            Some(2 * d(r, 0xff) + 3 * d(g, 0xc4) + 2 * d(b, 0x50))
        }
        _ => None,
    };
    colors
        .iter()
        .filter_map(|c| score(c).map(|s| (s, *c)))
        .min_by_key(|&(s, _)| s)
        .map(|(_, c)| c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::Theme;

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: theme.bg,
            seed: 7,
            fiesta,
        }
    }

    /// Runs a scene for `frames` frames of `dt`, drawing each.
    fn run(s: &mut Box<dyn Scene>, ctx: &Ctx, c: &mut Canvas, frames: usize, dt: f32) {
        for _ in 0..frames {
            s.update(ctx, dt);
            c.clear();
            s.draw(ctx, c);
        }
    }

    fn has(c: &Canvas, f: impl Fn(&crate::render::Cell) -> bool) -> bool {
        c.cells().iter().any(f)
    }

    #[test]
    fn muertos_dresses_the_sun_and_planets() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        for (day, comets) in [((10, 28), 1), ((11, 2), 3)] {
            let fiesta = Fiesta::of_day(day.0, day.1);
            let ctx = ctx(&theme, &opts, fiesta);
            let mut s = (INFO.make)(&ctx);
            s.resize(&ctx, 120, 36);
            s.retheme(&ctx);
            let mut c = Canvas::new(120, 36, Color::Reset);
            run(&mut s, &ctx, &mut c, 40, 1.0 / 30.0);
            // The sun-calavera's flower eyes and its grin.
            assert!(has(&c, |k| k.ch == '✿' && k.fg == rgb(TURQUESA)), "{day:?}");
            assert!(has(&c, |k| k.ch == '✿' && k.fg == rgb(ROSA)), "{day:?}");
            assert!(has(&c, |k| k.ch == '▙' || k.ch == '▟'), "{day:?}");
            // Marigold garlands and petals.
            assert!(has(&c, |k| k.fg == rgb(CEMPASUCHIL_LIGHT)), "{day:?}");
            let mut dm = Muertos::new(&ctx);
            dm.set(fiesta);
            assert_eq!(dm.comets_live(), comets, "{day:?}");
        }
    }

    #[test]
    fn comets_shed_petals() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = ctx(&theme, &opts, Fiesta::of_day(11, 2));
        let mut s = Orrery::new(&ctx);
        s.resize(&ctx, 160, 48);
        s.retheme(&ctx);
        let mut most = 0;
        for _ in 0..30 * 60 {
            s.update(&ctx, 1.0 / 30.0);
            most = most.max(s.dm.petals.len());
        }
        assert!(most > 40, "{most}");
        assert!(most <= MAX_PETALS);
    }

    /// The belt turns to marigold petals, with a few velvety terciopelo among
    /// them, and more of them as the holiday builds up.
    #[test]
    fn the_belt_turns_to_petals() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let mut shares = [0.0; 2];
        for (k, day) in [(10, 28), (11, 2)].into_iter().enumerate() {
            let ctx = ctx(&theme, &opts, Fiesta::of_day(day.0, day.1));
            let mut dm = Muertos::new(&ctx);
            dm.set(ctx.fiesta);
            dm.retheme(&ctx);
            let (mut petals, mut velvet) = (0, 0);
            for i in 0..3000 {
                if let Some(c) = dm.belt_petal(i, (i % 3) as u8) {
                    petals += 1;
                    velvet += (c == rgb(TERCIOPELO)) as usize;
                }
            }
            assert!(velvet > 0 && velvet * 5 < petals, "{day:?}");
            shares[k] = petals as f32 / 3000.0;
        }
        assert!(shares[0] > 0.1 && shares[0] < shares[1], "{shares:?}");
        let mut dm = Muertos::new(&ctx(&theme, &opts, Fiesta::NONE));
        dm.set(Fiesta::NONE);
        assert!((0..3000).all(|i| dm.belt_petal(i, 0).is_none()));
    }

    /// The sun-calavera winks now and then, on the wall clock however slowly
    /// the orbits go.
    #[test]
    fn the_calavera_winks() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = ctx(&theme, &opts, Fiesta::of_day(11, 2));
        let mut s = (INFO.make)(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let mut c = Canvas::new(120, 36, Color::Reset);
        let (mut open, mut shut) = (0, 0);
        for _ in 0..30 * 24 {
            // A tenth of the pace for the orbits, the wall clock as usual.
            s.tick(&ctx, 0.1 / 30.0, 1.0 / 30.0);
            c.clear();
            s.draw(&ctx, &mut c);
            if has(&c, |k| k.ch == '‿') {
                shut += 1;
            } else if has(&c, |k| k.ch == '✿' && k.fg == rgb(ROSA)) {
                open += 1;
            }
        }
        // About 0.45 s in every 11, so two or three winks in 24 s.
        assert!((20..=60).contains(&shut), "{shut}");
        assert!(open > 600, "{open}");
    }

    #[test]
    fn ordinary_days_have_none_of_it() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = ctx(&theme, &opts, Fiesta::NONE);
        let mut s = (INFO.make)(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let mut c = Canvas::new(120, 36, Color::Reset);
        run(&mut s, &ctx, &mut c, 40, 1.0 / 30.0);
        assert!(!has(&c, |k| matches!(k.ch, '✿' | '❀' | '☻')));
        let holiday = [CEMPASUCHIL_LIGHT, ROSA, TERCIOPELO, TURQUESA].map(rgb);
        assert!(!has(&c, |k| holiday.contains(&k.fg)));
    }

    /// Turning the holiday on and off through `retheme` dresses the scene and
    /// undresses it again. (The bigger sun moves the orbits while it lasts, so
    /// the holiday frames here take no time, and the scene comes back exactly
    /// as if it had never come.)
    #[test]
    fn holiday_comes_and_goes() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let plain = ctx(&theme, &opts, Fiesta::NONE);
        let fiesta = ctx(&theme, &opts, Fiesta::of_day(11, 1));
        let mut a = (INFO.make)(&plain);
        let mut b = (INFO.make)(&plain);
        for s in [&mut a, &mut b] {
            s.resize(&plain, 100, 30);
            s.retheme(&plain);
        }
        let mut ca = Canvas::new(100, 30, Color::Reset);
        let mut cb = Canvas::new(100, 30, Color::Reset);
        run(&mut a, &plain, &mut ca, 20, 1.0 / 30.0);
        run(&mut b, &plain, &mut cb, 20, 1.0 / 30.0);
        b.retheme(&fiesta);
        run(&mut b, &fiesta, &mut cb, 3, 0.0);
        assert!(has(&cb, |k| k.ch == '✿'));
        assert_ne!(ca.to_text(), cb.to_text());
        b.retheme(&plain);
        run(&mut a, &plain, &mut ca, 20, 1.0 / 30.0);
        run(&mut b, &plain, &mut cb, 20, 1.0 / 30.0);
        assert!(ca.cells() == cb.cells());

        // A holiday that lasts a while is gone without a trace too.
        b.retheme(&fiesta);
        run(&mut b, &fiesta, &mut cb, 90, 1.0 / 30.0);
        b.retheme(&plain);
        run(&mut b, &plain, &mut cb, 1, 1.0 / 30.0);
        assert!(!has(&cb, |k| matches!(k.ch, '✿' | '❀' | '☻')));
        let holiday = [CEMPASUCHIL_LIGHT, ROSA, TERCIOPELO, TURQUESA].map(rgb);
        assert!(!has(&cb, |k| holiday.contains(&k.fg)));
    }

    /// Every face and option survives tiny and huge screens and huge steps, on
    /// several days of the holiday, under classic and another theme.
    #[test]
    fn holiday_survives() {
        let variants = [
            "",
            "planets = 1\nbelt = false\nmoons = false",
            "planets = 9\ntilt = 1.0\nspeed = 50.0",
            "tilt = 0.0\nspeed = 0.0\norbits = false\ntrails = false",
        ];
        let sizes = [
            (80, 24),
            (1, 1),
            (2, 3),
            (7, 2),
            (40, 12),
            (60, 20),
            (160, 48),
            (300, 90),
            (3, 40),
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                for (m, d) in [(10, 28), (10, 31), (11, 2), (11, 3)] {
                    let opts = Opts::new(INFO.options, v.parse().unwrap());
                    let ctx = ctx(&theme, &opts, Fiesta::of_day(m, d));
                    let mut s = (INFO.make)(&ctx);
                    for (i, &(w, h)) in sizes.iter().enumerate() {
                        s.resize(&ctx, w, h);
                        if i == 0 {
                            s.retheme(&ctx);
                        }
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for f in 0..40 {
                            s.update(&ctx, if f % 13 == 12 { 5.0 } else { 1.0 / 30.0 });
                            c.clear();
                            s.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn faces_are_symmetric_and_whole() {
        for f in &FACES {
            let w = f.rows[0].len();
            assert!(f.rows.len() % 4 == 2 && w % 4 == 2, "odd cells each way");
            for row in f.rows {
                assert_eq!(row.len(), w, "{row:?}");
                let rev: String = row.chars().rev().collect();
                assert_eq!(*row, rev, "symmetric");
            }
            for &(x, y, ..) in f.icing {
                assert!((x as usize) < w / 2 && (y as usize) < f.rows.len() / 2);
            }
        }
        assert!(
            FACES
                .windows(2)
                .all(|p| p[0].from < p[1].from && p[0].r < p[1].r)
        );
    }

    #[test]
    fn helpers_behave() {
        let theme = Theme::builtin("dracula").unwrap();
        assert_eq!(nearest(&theme.colors, TURQUESA), Some(rgb(0x8be9fd)));
        assert_eq!(nearest(&theme.colors, ROSA), Some(rgb(0xff79c6)));
        assert_eq!(nearest(&[Color::Ansi(3)], ROSA), None);
        assert!((0..1000).all(|i| (0.0..1.0).contains(&hash01(i))));
        for e in [0.0, 0.5, 0.9, 0.97] {
            for i in 0..100 {
                let m = i as f32 * 0.37 - 20.0;
                let x = kepler(m, e);
                let d = (x - e * x.sin() - m).rem_euclid(TAU);
                assert!(d.min(TAU - d) < 1e-3, "e={e} m={m}");
            }
        }
    }
}
