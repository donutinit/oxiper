//! orrery: planets and moons orbiting a sun.
//!
//! Positions are kept in "units" of one cell width on both axes (a cell is
//! 1×2 units, a braille dot 0.5×0.5), so orbits are true ellipses on screen.
//! Each orbit lies in a plane seen from above at an angle: its depth axis is
//! squashed by `tilt`, and planets on the far side pass behind the sun.

use std::f32::consts::TAU;

use crate::color::{Color, Gradient, mix_oklab, rgb};
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
        self.sun_r = (big * 0.075).clamp(0.9, 4.5);
        self.cx = w * 0.5;
        self.cy = h * 0.5;
        if self.sun_r < SPRITE_R {
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
        let (se, ce) = e.sin_cos();
        let x = s.a * (ce - ecc);
        let z = -s.b * se;
        let xr = x * s.cos_p - z * s.sin_p;
        let zr = x * s.sin_p + z * s.cos_p;
        (self.cx + xr, self.cy + zr * self.view)
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
}

impl Scene for Orrery {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.br.resize(w, h);
        self.layout(w.max(1) as f32, h.max(1) as f32 * 2.0);
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
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 30.0) * self.speed
        } else {
            0.0
        };
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
            for &(x, y, si) in &self.orbit_dots {
                let kind = self.slots[si as usize].kind;
                self.br.set(x as i32, y as i32, self.orbit_col[kind]);
            }
        }

        if self.show_belt {
            for rk in &self.rocks[..self.n_rocks] {
                let (s, co) = rk.theta.sin_cos();
                let x = self.cx + rk.r * co;
                let y = self.cy - rk.r * s * t;
                let col = ctx.fade(self.belt_col[rk.shade as usize], 1.0 - rk.bright);
                self.br.setf(x * 2.0, y * 2.0, col);
            }
        }

        // Corona: flickering sparks around the sun's rim.
        let r = self.sun_r;
        for (i, sp) in self.sparks[..self.n_sparks].iter().enumerate() {
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

        // Saturn's ring and the moons.
        for (si, s) in self.slots.iter().enumerate() {
            let kd = &KINDS[s.kind];
            // Rings and moons circle the glyph's cell center, so they move with it.
            let px = pos[si].0.floor() + 0.5;
            let py = (pos[si].1 * 0.5).floor() * 2.0 + 1.0;
            if kd.ring {
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
        self.draw_sun(c);
        for (si, s) in self.slots.iter().enumerate() {
            let (x, y, _) = pos[si];
            if y >= self.cy {
                self.draw_planet(c, s, x, y);
            }
        }
    }
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
