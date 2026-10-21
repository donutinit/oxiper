//! Día de Muertos under the sea. The shark wears calavera face paint (a
//! sugar skull's white face, flower eye sockets, a stitched grin and gill
//! slits in papel picado colors; the whale shark's spots turn to confetti),
//! the plankton glows marigold orange, sunken ofrendas stand on the sand
//! with their veladoras lit, and cempasúchil petals float on the surface,
//! sink slowly through the water and settle on the sand.
//!
//! The ofrendas are built up with the holiday, the way a real one is: at
//! first a few candles, the photo and the marigolds; then sugar skulls, pan
//! de muerto, cockscomb and more candles; at its height an arch of marigolds,
//! copal, and a long path of petals for the souls to find their way.
//!
//! Like the floor, the ofrendas are dealt out by hashing their place, and
//! what moves on its own (the petals, the copal's bubbles) draws from its
//! own generator, so an ordinary day's sea is the same as ever.

use std::f32::consts::PI;

use super::art::{Kind, Sprite};
use super::life::Speck;
use super::sea::{SURF, Sea, View};
use super::{Dice, Rgb, Rng, col, hash, rgb_of, unit};
use crate::fiesta;
use crate::scene::Ctx;

const HUESO: Rgb = Rgb(244, 238, 226);
const SOCKET: Rgb = Rgb(44, 26, 52);
const CEMPASUCHIL: Rgb = Rgb(255, 140, 0);
const CEMPA_LIGHT: Rgb = Rgb(255, 176, 34);
const CEMPA_DARK: Rgb = Rgb(214, 98, 0);
const TERCIOPELO: Rgb = Rgb(176, 0, 72);
const FLAME_CORE: Rgb = Rgb(255, 232, 160);
const FLAME_EDGE: Rgb = Rgb(255, 150, 40);
/// Plankton glowing like embers, and the light of the candles in the water.
pub const GLOW: Rgb = Rgb(255, 158, 48);
/// The lanternfish's lights, become candlelight.
pub const LAMP: Rgb = Rgb(255, 206, 120);
/// The papel picado colors, as the face paint and the ofrenda's fringes
/// name them: rosa mexicano, amarillo, turquesa, verde limón, morado.
const ICING: [Rgb; 5] = [
    Rgb(228, 0, 124),
    Rgb(255, 210, 0),
    Rgb(0, 170, 184),
    Rgb(140, 200, 40),
    Rgb(112, 40, 160),
];
const ICING_KEYS: [u8; 5] = *b"rytgp";

/// The holiday's colors under the theme, and how far it has come.
#[derive(Clone, Copy)]
pub struct Dress {
    pub level: f32,
    /// The papel picado colors: the theme's accents, or the sea's own under
    /// `classic`. Flames, petals and marigolds keep their own.
    pub icing: [Rgb; 5],
}

impl Dress {
    /// The dress for today, if today is Día de Muertos.
    pub fn of(ctx: &Ctx) -> Option<Dress> {
        if !ctx.fiesta.is(fiesta::Kind::Muertos) {
            return None;
        }
        let mut icing = ICING;
        for (i, c) in icing.iter_mut().enumerate() {
            *c = rgb_of(ctx.pick(i, col(*c))).unwrap_or(*c);
        }
        Some(Dress {
            level: ctx.fiesta.level.clamp(0.0, 1.0),
            icing,
        })
    }

    /// The color of a face paint or ofrenda letter that has a fixed one.
    fn color(&self, ch: u8) -> Option<Rgb> {
        if let Some(i) = ICING_KEYS.iter().position(|&k| k == ch) {
            return Some(self.icing[i]);
        }
        Some(match ch {
            b'o' => HUESO,
            b'k' => SOCKET,
            b'c' => CEMPASUCHIL,
            b'C' => CEMPA_DARK,
            b'Y' => CEMPA_LIGHT,
            b'v' => TERCIOPELO,
            b'V' => Rgb(222, 40, 122),
            b'l' => Rgb(74, 136, 52),
            b'L' => Rgb(46, 94, 40),
            b'b' => Rgb(186, 106, 44),
            b'B' => Rgb(226, 150, 72),
            b'u' => Rgb(252, 230, 176),
            b'f' => Rgb(200, 156, 64),
            b'F' => Rgb(132, 92, 40),
            b'P' => Rgb(176, 146, 104),
            b'h' => Rgb(92, 66, 50),
            b'H' => Rgb(222, 190, 150),
            b'q' => Rgb(168, 84, 46),
            b'Q' => Rgb(116, 54, 32),
            b'W' => Rgb(252, 242, 222),
            _ => return None,
        })
    }
}

// ----- the face paint -----

/// Face paint for one of a shark's drawings, drawn by hand like the drawing
/// itself: a patch of letters whose top-left corner is at `(x, y)` of the
/// drawing. `.` leaves the shark as it is; `o` is the sugar white, `k` the
/// dark of the sockets and the stitches, `c` and `y` a marigold, and `r`,
/// `y`, `t`, `g`, `p` the icing.
pub struct Paint {
    x: i32,
    y: i32,
    rows: Sprite,
}

impl Paint {
    #[inline]
    fn at(&self, c: i32, r: i32) -> u8 {
        let (i, j) = (c - self.x, r - self.y);
        if i < 0 || j < 0 {
            return b'.';
        }
        self.rows
            .get(j as usize)
            .and_then(|row| row.as_bytes().get(i as usize))
            .copied()
            .unwrap_or(b'.')
    }
}

/// A shark's face paint, side on and through a turn.
pub struct Face {
    pub side: &'static Paint,
    pub quarter: &'static Paint,
    pub front: &'static Paint,
}

pub fn face(kind: Kind) -> Face {
    match kind {
        Kind::GreatWhite => Face {
            side: &GW_SIDE,
            quarter: &GW_QUARTER,
            front: &GW_FRONT,
        },
        // A tiger shark's head is a great white's, give or take the mouth.
        Kind::Tiger => Face {
            side: &TG_SIDE,
            quarter: &GW_QUARTER,
            front: &GW_FRONT,
        },
        Kind::Hammerhead => Face {
            side: &HH_SIDE,
            quarter: &HH_QUARTER,
            front: &HH_FRONT,
        },
        Kind::WhaleShark => Face {
            side: &WS_SIDE,
            quarter: &WS_QUARTER,
            front: &WS_FRONT,
        },
    }
}

/// The face paint as one drawing is put on screen: its patch, and its
/// colors lit for the shark's depth.
pub struct Dab {
    paint: &'static Paint,
    lut: [Option<Rgb>; 128],
    /// The whale shark's pale spots become confetti.
    spots: bool,
}

impl Dab {
    /// The paint over letter `ch` at `(c, r)` of the drawing, if any. The
    /// eyes and the open mouth always show through.
    #[inline]
    pub fn over(&self, c: usize, r: usize, ch: u8) -> Option<Rgb> {
        if matches!(ch, b'e' | b'h' | b'm' | b'M' | b't') {
            return None;
        }
        let p = self.paint.at(c as i32, r as i32);
        if p != b'.' {
            return self.lut[(p & 127) as usize];
        }
        if self.spots && ch == b'o' {
            return self.lut[ICING_KEYS[(c * 3 + r * 7) / 2 % 5] as usize];
        }
        None
    }
}

// ----- the ofrendas -----

/// Width of the stretch of sand each ofrenda may stand in.
const SPAN: i64 = 230;
/// An ofrenda's width; its middle is between columns 18 and 19.
const OFR_W: i32 = 38;
/// The three steps, from the bottom: first and last column, and how high
/// above the foot the cloth on top lies.
const TIERS: [(i32, i32, i32); 3] = [(0, 37, 5), (3, 34, 13), (7, 30, 21)];

/// Something on a step: its drawing, its first column, the step, and the
/// level of the holiday from which it is there.
struct Item {
    sprite: Sprite,
    x: i32,
    tier: usize,
    from: f32,
}

/// A veladora: wax, and glass in one of the papel picado colors. Its flame
/// is lit above the middle column.
#[rustfmt::skip]
const CANDLE: Sprite = &[
    "wWw",
    "www",
    "www",
];

/// The photo of the one being remembered, in a gilt frame.
#[rustfmt::skip]
const PHOTO: Sprite = &[
    "fFFFFf",
    "fPhhPf",
    "fPHHPf",
    "fPhhPf",
    "fhhhhf",
    "ffffff",
];

#[rustfmt::skip]
const SKULL_A: Sprite = &[
    ".oro.",
    "okoko",
    "ookoo",
    ".oto.",
];

#[rustfmt::skip]
const SKULL_B: Sprite = &[
    ".oyo.",
    "okoko",
    "ookoo",
    ".opo.",
];

/// The big calavera de azúcar in the middle, a marigold on its brow.
#[rustfmt::skip]
const SKULL_BIG: Sprite = &[
    ".oooo.",
    "oocYoo",
    "okooko",
    "ookkoo",
    ".otro.",
];

/// Pan de muerto: the round loaf, its bones and the knob on top, sugared.
#[rustfmt::skip]
const PAN: Sprite = &[
    "...u...",
    ".bBuBb.",
    "bBbbbBb",
    "bbbbbbb",
];

/// Cempasúchil in a clay jar.
#[rustfmt::skip]
const MARIGOLDS: Sprite = &[
    ".cYc.",
    "YcCcY",
    "cCYCc",
    ".lLl.",
    "qqqqq",
    ".qQq.",
];

/// Terciopelo, the velvet cockscomb, in a smaller jar.
#[rustfmt::skip]
const COCKSCOMB: Sprite = &[
    ".Vv..",
    "vVvVv",
    ".vVv.",
    ".lLl.",
    ".qqq.",
    ".qQq.",
];

/// The sahumador, copal smouldering in it.
#[rustfmt::skip]
const COPAL: Sprite = &[
    ".YC.",
    "qqqq",
    ".qQ.",
    "qqqq",
];

const ITEMS: &[Item] = &[
    Item {
        sprite: PHOTO,
        x: 16,
        tier: 2,
        from: 0.0,
    },
    Item {
        sprite: SKULL_A,
        x: 11,
        tier: 2,
        from: 0.3,
    },
    Item {
        sprite: SKULL_B,
        x: 22,
        tier: 2,
        from: 0.3,
    },
    Item {
        sprite: PAN,
        x: 6,
        tier: 1,
        from: 0.4,
    },
    Item {
        sprite: PAN,
        x: 25,
        tier: 1,
        from: 0.4,
    },
    Item {
        sprite: SKULL_BIG,
        x: 16,
        tier: 1,
        from: 0.55,
    },
    Item {
        sprite: MARIGOLDS,
        x: 0,
        tier: 0,
        from: 0.0,
    },
    Item {
        sprite: MARIGOLDS,
        x: 33,
        tier: 0,
        from: 0.0,
    },
    Item {
        sprite: COCKSCOMB,
        x: 9,
        tier: 0,
        from: 0.4,
    },
    Item {
        sprite: COCKSCOMB,
        x: 24,
        tier: 0,
        from: 0.4,
    },
    Item {
        sprite: COPAL,
        x: 17,
        tier: 0,
        from: 0.75,
    },
];

/// The veladoras: first column, step, and the level they are lit from.
const CANDLES: &[(i32, usize, f32)] = &[
    (7, 2, 0.0),
    (28, 2, 0.0),
    (5, 0, 0.0),
    (30, 0, 0.0),
    (3, 1, 0.3),
    (32, 1, 0.3),
    (13, 1, 0.75),
    (22, 1, 0.75),
];

/// The level from which the arch of marigolds stands over the ofrenda.
const ARCH_FROM: f32 = 0.55;
/// The copal's place: column and height above the foot.
const COPAL_AT: (f32, f32) = (18.5, 10.0);
const COPAL_FROM: f32 = 0.75;

// ----- the petals -----

#[derive(Clone, Copy, PartialEq, Eq)]
enum Drift {
    /// Floating on the surface.
    Float,
    /// Sinking, see-sawing as it goes.
    Sink,
    /// Lying on the sand.
    Rest,
}

#[derive(Clone, Copy)]
struct Petal {
    x: f32,
    y: f32,
    phase: f32,
    drift: Drift,
    /// Steps left on the sand.
    rest: u16,
}

/// The holiday's own state in the sea.
pub struct Muertos {
    pub dress: Dress,
    rng: Rng,
    petals: Vec<Petal>,
    /// The flames painted this frame, lit again on screen unless something
    /// swam in front of them: where, and the color they were painted.
    flames: Vec<(i32, i32, Rgb)>,
    /// Where the first ofrenda stands, just ahead of where the shark was
    /// when the holiday came.
    first: i64,
}

impl Muertos {
    pub fn new(dress: Dress, seed: u32, first: i64) -> Muertos {
        Muertos {
            dress,
            first,
            rng: Rng::with_seed(seed as u64 ^ 0x6d75_6572_746f_7321),
            petals: Vec::new(),
            flames: Vec::with_capacity(64),
        }
    }

    /// Whether the `i`th speck of marine snow glows like plankton.
    #[inline]
    pub fn plankton(&self, i: usize) -> bool {
        let roll = (hash(i as i32, 7, 0x51ed) % 100) as f32;
        roll < 30.0 + 50.0 * self.dress.level
    }
}

impl Sea {
    /// Puts on (or takes off) the holiday's dress, keeping what drifts in
    /// the sea if it was already on.
    pub fn dress(&mut self, dress: Option<Dress>) {
        match (dress, self.muertos.as_mut()) {
            (None, _) => self.muertos = None,
            (Some(d), Some(m)) => {
                let more = (d.level - m.dress.level).abs() > 1e-3;
                m.dress = d;
                if more {
                    self.scatter_petals();
                }
            }
            (Some(d), None) => {
                let first = self.base + (self.shark.x + self.shark.dir * 70.0) as i64;
                self.muertos = Some(Muertos::new(d, self.seed, first));
                self.scatter_petals();
            }
        }
    }

    /// Strews petals over the surface and through the water around the camera.
    fn scatter_petals(&mut self) {
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        let Some(m) = self.muertos.as_mut() else {
            return;
        };
        let n = (vw / 4.5 * (0.3 + 0.7 * m.dress.level)).clamp(6.0, 200.0) as usize;
        m.petals.clear();
        m.petals.reserve(n);
        for i in 0..n {
            let x = cam + m.rng.rangef(-1.0, 1.0) * vw;
            let sink = i % 5 < 2;
            m.petals.push(Petal {
                x,
                y: if sink {
                    m.rng.rangef(SURF as f32 + 4.0, 125.0)
                } else {
                    SURF as f32
                },
                phase: m.rng.rangef(0.0, 6.3),
                drift: if sink { Drift::Sink } else { Drift::Float },
                rest: 0,
            });
        }
    }

    /// The waterline's row above local column `x`, as the backdrop draws it.
    fn surface(&self, x: f32) -> f32 {
        let (u, t) = (self.base as f64 + x as f64, self.time() as f64);
        SURF as f32
            + (0.9 * (u * 0.11 + t * 0.9).sin() + 0.6 * (u * 0.29 - t * 1.6).sin()).round() as f32
    }

    /// The middle (local column) of every ofrenda within `reach` of local
    /// columns `lo..hi`, with its number, on a day of the holiday at
    /// `level`, the first standing at `first`.
    fn ofrendas(
        &self,
        (level, first): (f32, i64),
        (lo, hi): (f32, f32),
        reach: f32,
        out: &mut [(i64, i32); 4],
    ) -> usize {
        let p = 0.45 + 0.45 * level;
        let (a, b) = (
            (self.base as f64 + (lo - reach) as f64).floor() as i64,
            (self.base as f64 + (hi + reach) as f64).ceil() as i64,
        );
        let mut n = 0;
        for k in a.div_euclid(SPAN)..=b.div_euclid(SPAN) {
            // There is always one where the shark starts out.
            let start = k == first.div_euclid(SPAN);
            if !start && unit(k as i32, 400, self.seed) >= p {
                continue;
            }
            let u = if start {
                first
            } else {
                k * SPAN + SPAN / 4 + (unit(k as i32, 401, self.seed) * (SPAN / 2) as f32) as i64
            };
            if u >= a && u <= b && n < out.len() {
                out[n] = (k, (u - self.base) as i32);
                n += 1;
            }
        }
        n
    }

    /// Whether an ofrenda stands on the floor's cell `c` (local columns
    /// `x0..x0 + w`), which then stays bare.
    pub fn ofrenda_on(&self, x0: i32, w: i32) -> bool {
        let mut found = [(0, 0); 4];
        let lo = x0 as f32 - OFR_W as f32;
        let Some(m) = &self.muertos else {
            return false;
        };
        let hi = (x0 + w) as f32 + OFR_W as f32;
        self.ofrendas((m.dress.level, m.first), (lo, hi), 0.0, &mut found) > 0
    }

    /// One step of what drifts: petals on the surface, sinking and lying on
    /// the sand, and the copal's smoke, bubbling up.
    pub fn update_muertos(&mut self) {
        let Some(mut m) = self.muertos.take() else {
            return;
        };
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        let t = self.time();
        let reach = vw * 0.6 + 30.0;
        for i in 0..m.petals.len() {
            let mut p = m.petals[i];
            match p.drift {
                Drift::Float => {
                    p.phase += 0.03;
                    p.x += 0.03 + 0.05 * (t * 0.6 + p.phase).sin();
                    p.y = self.surface(p.x);
                    if m.rng.chance(0.0012) {
                        p.drift = Drift::Sink;
                    }
                }
                Drift::Sink => {
                    p.phase += 0.075;
                    p.x += 0.02 + 0.16 * p.phase.sin();
                    p.y += 0.06 + 0.03 * p.phase.cos().abs();
                    let bed = self.bed(p.x) as f32;
                    if p.y >= bed {
                        p.y = bed + m.rng.rangef(0.0, 2.0);
                        p.drift = Drift::Rest;
                        p.rest = m.rng.range(400, 1600) as u16;
                    }
                }
                Drift::Rest => {
                    p.rest = p.rest.saturating_sub(1);
                }
            }
            // Gone from sight or melted into the sand: one more falls on the
            // surface, ahead or behind.
            let off = p.x - cam;
            if off.abs() > reach || p.drift == Drift::Rest && p.rest == 0 {
                p.x = if off.abs() > reach {
                    cam - off.signum() * (reach - 4.0)
                } else {
                    cam + m.rng.rangef(-1.0, 1.0) * reach
                };
                p.drift = Drift::Float;
                p.y = self.surface(p.x);
            }
            m.petals[i] = p;
        }
        // Copal smoke, under water a thin stream of bubbles.
        if m.dress.level >= COPAL_FROM {
            let mut found = [(0, 0); 4];
            let n = self.ofrendas(
                (m.dress.level, m.first),
                (cam - reach, cam + reach),
                0.0,
                &mut found,
            );
            for &(_, cx) in &found[..n] {
                if m.rng.chance(0.12) {
                    let x = (cx - OFR_W / 2) as f32 + COPAL_AT.0 + m.rng.rangef(-0.6, 0.6);
                    let y = (self.bed(cx as f32) + 1) as f32 - COPAL_AT.1;
                    let phase = m.rng.rangef(0.0, 6.3);
                    self.life.bubble(x, y, m.rng.chance(0.15), phase);
                }
            }
        }
        self.muertos = Some(m);
    }

    /// Paints the ofrendas in view onto the floor, with the light of their
    /// candles in the water around them.
    pub fn paint_ofrendas(&mut self) {
        let Some(mut m) = self.muertos.take() else {
            return;
        };
        m.flames.clear();
        let mut found = [(0, 0); 4];
        let (lo, hi) = (self.bx as f32, (self.bx + self.bw) as f32);
        let n = self.ofrendas(
            (m.dress.level, m.first),
            (lo, hi),
            OFR_W as f32 + 140.0,
            &mut found,
        );
        for &(k, cx) in &found[..n] {
            self.ofrenda(&mut m, k, cx);
        }
        self.candlelight(&mut m);
        self.muertos = Some(m);
    }

    /// A pixel of the ofrenda, lit for its depth.
    #[inline]
    fn dot(&mut self, x: i32, y: i32, c: Rgb) {
        let c = self.shade(c, y, 0.05);
        self.put(x, y, c);
    }

    fn ofrenda(&mut self, m: &mut Muertos, k: i64, cx: i32) {
        let d = m.dress;
        let left = cx - OFR_W / 2;
        let foot = self.bed(cx as f32) + 1;
        let seed = self.seed ^ k as u32;
        // The path of petals that leads the souls home, strewn on the sand.
        let side = if unit(k as i32, 402, self.seed) < 0.5 {
            -1
        } else {
            1
        };
        let len = (24.0 + 120.0 * d.level) as i32;
        for s in -6..len {
            let x = if side > 0 {
                left + OFR_W + s
            } else {
                left - 1 - s
            };
            let bed = self.bed(x as f32);
            let thin = (s.max(0) as f32 / len as f32).powf(0.7);
            for y in bed..bed + 3 {
                let h = hash(x, y, seed);
                if (h % 100) as f32 >= 85.0 * (1.0 - thin) - (y - bed) as f32 * 20.0 {
                    continue;
                }
                let c = [CEMPASUCHIL, CEMPA_LIGHT, CEMPA_DARK][(h >> 8) as usize % 3];
                self.dot(x, y, c);
            }
        }
        // A few on the other side too.
        for s in 0..6 {
            let x = if side > 0 {
                left - 1 - s
            } else {
                left + OFR_W + s
            };
            let bed = self.bed(x as f32);
            let h = hash(x, bed, seed);
            if h.is_multiple_of(3) {
                self.dot(x, bed + (h >> 4) as i32 % 2, CEMPASUCHIL);
            }
        }
        // A dark cloth hung behind the steps, so the candles and the skulls
        // stand out from the water; its top follows the arch.
        let cloth_rgb = d.icing[4].scale(0.4);
        let (ox, oy) = (left as f32 + 18.5, (foot - 1) as f32);
        let ry = if d.level >= ARCH_FROM { 29.0 } else { 26.0 };
        for x in left - 1..left + OFR_W + 1 {
            let k = (x as f32 - ox) / 19.5;
            if k.abs() >= 1.0 {
                continue;
            }
            let top = (oy - ry * (1.0 - k * k).sqrt()).round() as i32;
            for y in top..foot {
                // Folds, falling straight.
                let fold = (x - left).rem_euclid(4) == 0;
                self.dot(x, y, cloth_rgb.lerp(HUESO, if fold { 0.1 } else { 0.0 }));
            }
        }
        // The steps, the top one first, each a stone block under a cloth
        // with a fringe of papel picado, and what stands on it.
        for tier in (0..3).rev() {
            let (x0, x1, top) = TIERS[tier];
            let cloth = foot - top;
            for c in x0..=x1 {
                let x = left + c;
                let bottom = if tier == 0 {
                    self.bed(x as f32) + 1
                } else {
                    foot
                };
                for y in cloth + 1..=bottom {
                    // Weathered stone, a little overgrown after a year in the sea.
                    let h = hash(x, y, seed);
                    let stone = if y == bottom || y == cloth + 4 {
                        // The fringe's shadow, and the sand's.
                        Rgb(40, 30, 40)
                    } else if h.is_multiple_of(37) {
                        Rgb(84, 122, 70)
                    } else if h.is_multiple_of(89) {
                        Rgb(196, 116, 130)
                    } else if (c + (foot - y) / 3 * 5) % 9 == 0 {
                        Rgb(66, 50, 62)
                    } else {
                        Rgb(84, 64, 78)
                    };
                    self.dot(x, y, stone);
                }
                // The cloth and its fringe of flags, three columns each,
                // cut to a point.
                self.dot(x, cloth, HUESO);
                let flag = d.icing[((c - x0) / 3 % 5) as usize];
                self.dot(x, cloth + 1, flag);
                self.dot(x, cloth + 2, flag);
                if (c - x0) % 3 == 1 {
                    self.dot(x, cloth + 3, flag);
                }
            }
            for item in ITEMS.iter().filter(|i| i.tier == tier && d.level >= i.from) {
                self.item(item.sprite, left + item.x, cloth - 1, &d, Rgb::BLACK);
            }
            for (i, &(c, t, from)) in CANDLES.iter().enumerate() {
                if t != tier || d.level < from {
                    continue;
                }
                let glass = d.icing[(i as i64 + k).rem_euclid(5) as usize].lerp(HUESO, 0.45);
                self.item(CANDLE, left + c, cloth - 1, &d, glass);
                if m.flames.len() < m.flames.capacity() {
                    m.flames
                        .push((left + c + 1, cloth - 1 - CANDLE.len() as i32, Rgb::BLACK));
                }
            }
        }
        // The arch of marigolds over it all.
        if d.level >= ARCH_FROM {
            const N: usize = 140;
            for i in 0..=N {
                let a = PI * i as f32 / N as f32;
                for w in 0..2 {
                    let (rx, ry) = (21.0 - w as f32, 31.0 - w as f32);
                    let (x, y) = (
                        (ox - rx * a.cos()).round() as i32,
                        (oy - ry * a.sin()).round() as i32,
                    );
                    let h = hash(x, y, seed ^ 0xa5c);
                    let c = if h.is_multiple_of(11) {
                        TERCIOPELO
                    } else {
                        [CEMPASUCHIL, CEMPA_LIGHT, CEMPA_DARK, CEMPASUCHIL][(h >> 4) as usize % 4]
                    };
                    self.dot(x, y, c);
                }
            }
        }
    }

    /// Draws one of the ofrenda's things with its bottom row at `bottom`.
    fn item(&mut self, sprite: Sprite, left: i32, bottom: i32, d: &Dress, glass: Rgb) {
        let top = bottom - sprite.len() as i32 + 1;
        for (j, row) in sprite.iter().enumerate() {
            for (i, ch) in row.bytes().enumerate() {
                let c = if ch == b'w' { Some(glass) } else { d.color(ch) };
                if let Some(c) = c {
                    self.dot(left + i as i32, top + j as i32, c);
                }
            }
        }
    }

    /// The candles' flames, flickering, and their warm light in the water:
    /// faint by day, a glow at night.
    fn candlelight(&mut self, m: &mut Muertos) {
        let t = self.time();
        // Flicker in steps, so the water around a candle does not change
        // every frame.
        let tt = (t * 12.0).floor() / 12.0;
        let dark = 1.0 - self.daylight;
        let reach = 6.0 + 3.0 * m.dress.level;
        for i in 0..m.flames.len() {
            let (fx, fy, _) = m.flames[i];
            let h = hash(fx, fy, self.seed) as f32 / u32::MAX as f32 * 40.0;
            let flicker = 0.78 + 0.22 * ((tt * 7.3 + h).sin() * (tt * 11.9 + h * 1.7).sin());
            let k = (0.16 + 0.5 * dark) * flicker;
            let r = reach.ceil() as i32;
            for dy in -r..=r {
                for dx in -r..=r {
                    // Rounder for the half-block pixels' squareness.
                    let d = ((dx * dx + dy * dy) as f32).sqrt() / reach;
                    if d < 1.0 {
                        let a = k * (1.0 - d) * (1.0 - d);
                        self.blend(fx + dx, fy + dy, GLOW, a);
                    }
                }
            }
        }
        for i in 0..m.flames.len() {
            let (fx, fy, _) = m.flames[i];
            let h = hash(fx, fy, self.seed);
            let lean = ((tt * 5.0 + (h % 7) as f32).sin() * 1.4).round() as i32;
            self.put(fx, fy, FLAME_CORE);
            self.put(
                fx + lean.signum() * (lean.abs() > 1) as i32,
                fy - 1,
                FLAME_EDGE,
            );
            m.flames[i].2 = FLAME_CORE;
        }
    }

    /// What goes straight onto the screen, crisp at any zoom: the flames
    /// (unless something swam in front of them) and the petals.
    pub fn draw_muertos(&mut self, v: View) {
        let Some(m) = self.muertos.take() else {
            return;
        };
        for &(fx, fy, c) in &m.flames {
            if self.get(fx, fy) == Some(c) {
                let (sx, sy) = v.to_screen(fx as f32 + 0.5, fy as f32 + 0.5);
                self.scr.set(sx, sy, col(FLAME_CORE));
                if v.z < 1.0 {
                    self.blend_out(sx, sy - 1, FLAME_EDGE, 0.7);
                }
            }
        }
        let tint = self.light.tint;
        for (i, p) in m.petals.iter().enumerate() {
            let (sx, sy) = v.to_screen(p.x, p.y);
            let c = match p.drift {
                // Tumbling, it shows one face and then the other.
                Drift::Sink if (p.phase * 1.3).sin() > 0.0 => CEMPA_LIGHT,
                Drift::Sink => CEMPA_DARK,
                Drift::Float => [CEMPASUCHIL, CEMPA_LIGHT][i % 2],
                Drift::Rest => [CEMPASUCHIL, CEMPA_DARK][i % 2],
            };
            let lit = if p.drift == Drift::Float { 0.7 } else { 0.4 };
            let c = col(c.mul(tint.lerp(Rgb::WHITE, lit)));
            self.scr.set(sx, sy, c);
            // Lying flat on the water, a petal is wider than it is tall.
            if p.drift == Drift::Float {
                self.scr.set(sx + 1, sy, c);
            }
        }
        self.muertos = Some(m);
    }

    /// The colors of the face paint, lit for the shark's depth, for one of
    /// its drawings.
    pub fn dab(&self, paint: &'static Paint) -> Dab {
        let d = self.muertos.as_ref().map(|m| m.dress);
        let mut lut = [None; 128];
        if let Some(d) = d {
            let y = self.shark.y as i32;
            for ch in *b"okcCYvrytgp" {
                lut[ch as usize] = d.color(ch).map(|c| self.shade(c, y, 0.08));
            }
        }
        Dab {
            paint,
            lut,
            spots: self.shark.kind == Kind::WhaleShark,
        }
    }

    /// How the holiday colors a fleck of light.
    pub fn fleck_color(&self, kind: Speck) -> Option<Rgb> {
        self.muertos.as_ref()?;
        match kind {
            Speck::Spark | Speck::Plankton => Some(GLOW),
            _ => None,
        }
    }
}

// ----- the face paint, kind by kind -----

const GW_SIDE: Paint = Paint {
    x: 40,
    y: 8,
    rows: &[
        "........................",
        "...........oo...........",
        "..........oooyrrrry.....",
        "..........ocyrkkkkro....",
        "...........ycrkkkkroo...",
        ".r.y.t.g.pooorkkkkrokko.",
        ".r.y.t.g.p.oooorroookko.",
        ".r.y.t.g.pooooooookkk...",
        ".r.y.t.g.pookokokkk.k...",
        ".r.y.t.g.p...kkkk.k.....",
        "..............k.k.......",
    ],
};

const TG_SIDE: Paint = Paint {
    x: 40,
    y: 8,
    rows: &[
        "........................",
        "...........oo...........",
        "..........oooyrrrry.....",
        "..........ocyrkkkkro....",
        "...........ycrkkkkrooo..",
        ".r.y.t.g.poooyrkkryookk.",
        ".r.y.t.g.p.oooorrookook.",
        ".r.y.t.g.poookokokkkko..",
        ".r.y.t.g.pookkkkk..k....",
        ".r.y.t.g.p.k.k.k........",
    ],
};

const HH_SIDE: Paint = Paint {
    x: 40,
    y: 7,
    rows: &[
        "...................rrr..",
        "..................rkkr..",
        "..................yrry..",
        "..................oooo..",
        "..................oooo..",
        "..................oooo..",
        "..................oooo..",
        ".r.y.t.g..oooooooooccoo.",
        ".r.y.t.g..oooooooocyyc..",
        ".r.y.t.g..ookokooooocco.",
        ".r.y.t.g....kkkk..oooo..",
        "............k.k...oooo..",
        "..................oooo..",
        "..................oooo..",
        "..................yrry..",
        "..................rkkr..",
        "...................rrr..",
    ],
};

const WS_SIDE: Paint = Paint {
    x: 40,
    y: 8,
    rows: &[
        "........................",
        "..............ooooo.....",
        "..............ooccoooo..",
        "....r.y.t.g.p.oocyoooo..",
        "....r.y.t.g.p.ooooooooo.",
        "....r.y.t.g.p.oyrrryooko",
        "....r.y.t.g.p.orkkkrooko",
        "....r.y.t.g.p.orkkkrkoko",
        "....r.y.t.g.pookkkkkkkkk",
        "....r.y.t.g.p...k.k.k.k.",
    ],
};

const GW_FRONT: Paint = Paint {
    x: 9,
    y: 7,
    rows: &[
        "......ooooo......",
        "....oooocoooo....",
        "..ooooocycooooo..",
        ".rryoooocooooyrr.",
        "rkkrooooooooorkkr",
        "rkkrooooooooorkkr",
        ".rryoookkkoooyrr.",
        ".oooooookooooooo.",
        ".okokokokokokoko.",
        ".ookkkkkkkkkkkoo.",
        "....k.k.k.k.k....",
    ],
};

const GW_QUARTER: Paint = Paint {
    x: 28,
    y: 9,
    rows: &[
        "......ooo.......",
        "......ooyrry....",
        "......oorkkro...",
        "......oorkkroo..",
        ".r.y.t.gyrryoko.",
        ".r.y.t.gooookko.",
        ".r.y.t.gookokk..",
        ".r.y.t.gokkkk...",
        ".r.y.t.gk.k.....",
        ".r.y.t.g........",
    ],
};

const HH_FRONT: Paint = Paint {
    x: 2,
    y: 10,
    rows: &[
        ".............oocoo.............",
        "...........ooocycooo...........",
        "..........ooooocooooo..........",
        "..yr......ooooooooooo......ry..",
        "rkkrooooooooooooooooooooooorkkr",
        "rryoookoooooookkkooooooookooyrr",
        "............k.k.k.k............",
        ".........ookkkkkkkkkoo.........",
        "............k.k.k.k............",
    ],
};

const HH_QUARTER: Paint = Paint {
    x: 25,
    y: 7,
    rows: &[
        "........rr.........",
        "........krr........",
        "........ryo........",
        ".....oo...oo.......",
        ".....oooccooo......",
        ".....ooocyoooo.....",
        ".r.y.tooooooooo....",
        ".r.y.tooooooooo....",
        ".r.y.tooookooooo...",
        ".r.y.t...kkkoooo...",
        "..........k.ooooo..",
        ".............oooo..",
        "................oo.",
        "................ry.",
        "................rr.",
        "................kr.",
        ".................rr",
    ],
};

const WS_QUARTER: Paint = Paint {
    x: 30,
    y: 9,
    rows: &[
        "......ooo.....",
        "......ooccoo..",
        ".r.y.toocyooo.",
        ".r.y.tooooooo.",
        ".r.y.yryooooko",
        ".r.y.rkrooooko",
        ".r.y.rkrokokoo",
        ".r.y.tkkkkkkkk",
        ".r.y.t.k.k.k..",
    ],
};

const WS_FRONT: Paint = Paint {
    x: 2,
    y: 8,
    rows: &[
        "...............................",
        "......ooooooooocooooooooo......",
        "...ooooooooooocycooooooooooo...",
        "..ooooooooooooocooooooooooooo..",
        ".ryoooooooooooooooooooooooooyr.",
        "rkroooooooooookkkooooooooooorkr",
        "rrykookookookookookookookookyrr",
        ".kkkkkkkkkkkkkkkkkkkkkkkkkkkkk.",
        "...k..k..k..k..k..k..k..k..k...",
    ],
};

#[cfg(test)]
mod tests {
    use super::super::{INFO, SharkScene};
    use super::*;
    use crate::color::Color;
    use crate::fiesta::Fiesta;
    use crate::render::Canvas;
    use crate::scene::{Opts, Scene};
    use crate::theme::Theme;

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: Color::Reset,
            seed: 5,
            fiesta,
        }
    }

    fn opts(table: &str) -> Opts {
        Opts::new(INFO.options, table.parse().unwrap())
    }

    /// Cells showing a candle's flame.
    fn flames(cv: &Canvas) -> usize {
        let flame = col(FLAME_CORE);
        let mut n = 0;
        for y in 0..cv.h() as i32 {
            for x in 0..cv.w() as i32 {
                n += cv.get(x, y).is_some_and(|c| c.fg == flame || c.bg == flame) as usize;
            }
        }
        n
    }

    /// A fresh sea and its first picture.
    fn first(c: &Ctx) -> (SharkScene, Canvas) {
        let mut s = SharkScene::new(c);
        s.resize(c, 120, 36);
        s.retheme(c);
        s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
        let mut cv = Canvas::new(120, 36, Color::Reset);
        s.draw(c, &mut cv);
        (s, cv)
    }

    /// From the first day to its height, there is an ofrenda by the shark
    /// with its candles lit, petals in the water and paint on its face; on
    /// an ordinary day none of it.
    #[test]
    fn the_holiday_comes_to_the_sea() {
        let theme = Theme::classic();
        let opts = opts("");
        for (m, d) in [(10, 28), (11, 2)] {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (s, cv) = first(&c);
            let md = s.sea.muertos.as_ref().expect("dressed");
            assert!(!md.petals.is_empty() && !md.flames.is_empty());
            assert!(flames(&cv) > 0, "{m}-{d}: no candle burning");
            let paint = face(s.sea.shark.kind).side;
            let dab = s.sea.dab(paint);
            assert!(dab.over(55, 13, b'R').is_some(), "{m}-{d}: no face paint");
        }
        let c = ctx(&theme, &opts, Fiesta::NONE);
        let (s, cv) = first(&c);
        assert!(s.sea.muertos.is_none());
        assert_eq!(flames(&cv), 0);
        let dab = s.sea.dab(face(s.sea.shark.kind).side);
        assert!(
            dab.over(55, 13, b'R').is_none(),
            "face paint on an ordinary day"
        );
        assert!(s.sea.fleck_color(Speck::Plankton).is_none());
    }

    /// Every face paint lies on its drawing and uses only letters with colors.
    #[test]
    fn the_face_paint_fits() {
        for kind in Kind::ALL {
            let (rig, f) = (kind.rig(), face(kind));
            for (sprite, paint) in [
                (rig.side, f.side),
                (rig.quarter, f.quarter),
                (rig.front, f.front),
            ] {
                let w = paint.rows[0].len() as i32;
                assert!(paint.rows.iter().all(|r| r.len() as i32 == w), "{kind:?}");
                assert!(paint.x + w <= sprite[0].len() as i32, "{kind:?}");
                assert!(
                    paint.y + paint.rows.len() as i32 <= sprite.len() as i32,
                    "{kind:?}"
                );
                for ch in paint
                    .rows
                    .iter()
                    .flat_map(|r| r.bytes())
                    .filter(|&c| c != b'.')
                {
                    assert!(b"okcCYvrytgp".contains(&ch), "{kind:?}: {}", ch as char);
                }
            }
        }
    }

    /// The fuller the holiday, the more candles.
    #[test]
    fn the_ofrenda_grows_with_the_days() {
        let theme = Theme::classic();
        let opts = opts("");
        let lit = |m, d| {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (s, _) = first(&c);
            s.sea.muertos.as_ref().unwrap().flames.len()
        };
        assert!(lit(10, 28) < lit(11, 2));
    }

    /// The holiday comes and goes through `retheme`, as at midnight.
    #[test]
    fn retheme_dresses_and_undresses() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts, Fiesta::NONE);
        let on = ctx(&theme, &opts, Fiesta::of_day(11, 1));
        let (mut s, _) = first(&off);
        let mut cv = Canvas::new(120, 36, Color::Reset);
        for (c, dressed) in [(&on, true), (&off, false), (&on, true)] {
            s.retheme(c);
            assert_eq!(s.sea.muertos.is_some(), dressed);
            for _ in 0..90 {
                s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
            }
            cv.clear();
            s.draw(c, &mut cv);
            assert_eq!(flames(&cv) > 0, dressed);
            // A resize keeps the dress.
            s.resize(c, 120, 36);
            assert_eq!(s.sea.muertos.is_some(), dressed);
        }
    }

    /// Odd sizes, options and steps with the holiday on, under two themes.
    #[test]
    fn the_holiday_survives_extremes() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        let cases = [
            "",
            "kind = 'whale-shark'\nzoom = 0\nwater = false",
            "kind = 'hammerhead'\nzoom = 6\nspeed = 50.0\nday = 10",
            "kind = 'tiger'\nfish = 0\nrays = false",
        ];
        for theme in &themes {
            for case in cases {
                let opts = opts(case);
                for day in [(10, 28), (11, 2)] {
                    let c = ctx(theme, &opts, Fiesta::of_day(day.0, day.1));
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (3, 2), (1, 40), (300, 90), (80, 24)] {
                        s.resize(&c, w, h);
                        s.retheme(&c);
                        let mut cv = Canvas::new(w, h, Color::Reset);
                        for dt in [0.0, f32::NAN, 5.0, 0.05, 1e9, 1.0 / 30.0] {
                            s.tick(&c, dt, 0.05);
                            cv.clear();
                            s.draw(&c, &mut cv);
                        }
                    }
                }
            }
        }
    }

    /// Long enough for petals to sink and settle, the copal to smoke and
    /// the shark to swim far from its first ofrenda: it keeps finding more.
    #[test]
    fn a_long_swim_passes_more_ofrendas() {
        let theme = Theme::classic();
        let opts = opts("day = 60");
        let c = ctx(&theme, &opts, Fiesta::of_day(11, 2));
        let (mut s, mut cv) = first(&c);
        let mut seen = std::collections::HashSet::new();
        let mut rested = false;
        for i in 0..30 * 240 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            if i % 30 == 0 {
                cv.clear();
                s.draw(&c, &mut cv);
                let mut found = [(0, 0); 4];
                let m = s.sea.muertos.as_ref().unwrap();
                let (lo, hi) = (s.sea.bx as f32, (s.sea.bx + s.sea.bw) as f32);
                let n = s
                    .sea
                    .ofrendas((m.dress.level, m.first), (lo, hi), 0.0, &mut found);
                seen.extend(found[..n].iter().map(|f| f.0));
                rested |= m.petals.iter().any(|p| p.drift == Drift::Rest);
            }
        }
        assert!(seen.len() >= 2, "{seen:?}");
        assert!(rested, "no petal ever reached the sand");
    }
}
