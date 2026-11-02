//! Navidad on the waves (see the module doc in `waveform.rs`).
//!
//! Like Día de Muertos, the holiday only dresses the waves: the same `ys` the
//! ordinary scene draws from carry the strings of lights. It keeps no state
//! but a clock and its colors; every bulb, esfera and piñata is found again
//! each frame from the swell and a hash, riding along with it, so a frame is
//! just drawing. It draws no random numbers at all, so an ordinary day is as
//! it ever was.

use std::f32::consts::{PI, TAU};

use super::hues::like;
use super::muertos::{hash, rnd};
use super::{BAYER, Layer, SHADES, Style, Waveform};
use crate::color::{Color, rgb};
use crate::render::Braille;
use crate::scene::Ctx;

/// The cords the foquitos hang from, front to back: verde pino and rojo
/// nochebuena, alternating.
const CORD: [Color; 2] = [rgb(0x1f6f3a), rgb(0xc8102e)];
/// Foquitos: rojo, verde, azul, amarillo, rosa and naranja.
const FOQUITOS: [Color; 6] = [
    rgb(0xff3b3b),
    rgb(0x3bd16f),
    rgb(0x3b7bff),
    rgb(0xffd23b),
    rgb(0xff5fb4),
    rgb(0xff8c1a),
];
/// Esferas: rojo nochebuena, verde pino, dorado, plata and blanco.
const ESFERAS: [Color; 5] = [
    rgb(0xc8102e),
    rgb(0x1f6f3a),
    rgb(0xf2b632),
    rgb(0xc9ccd6),
    rgb(0xf4f1ea),
];
const DORADO: Color = rgb(0xf2b632);
const PLATA: Color = rgb(0xc9ccd6);
/// A piñata's papel de china: rosa mexicano, amarillo, naranja, turquesa,
/// morado and verde limón.
const PAPEL: [Color; 6] = [
    rgb(0xe4007c),
    rgb(0xffd200),
    rgb(0xff8c00),
    rgb(0x00aab8),
    rgb(0x7028a0),
    rgb(0x8cc828),
];
/// The paper of each of a piñata's seven points (the seventh breaks the
/// round of six).
const POINTS: [usize; 7] = [0, 1, 2, 3, 4, 5, 1];
/// What a lit bulb's filament glows toward, and an esfera's shine.
const GLOW: Color = rgb(0xfffbe8);

/// Most esferas and piñatas on one wave at once.
const MAX_HUNG: usize = 48;

/// A foquito's colors: lit, unlit, the hot heart and the halo.
#[derive(Clone, Copy)]
struct Bulb {
    lit: Color,
    dim: Color,
    hot: Color,
    halo: Color,
}

/// An esfera's colors: the glass lit, plain and in shadow, and its cap.
#[derive(Clone, Copy)]
struct Esfera {
    hi: Color,
    body: Color,
    shade: Color,
    cap: Color,
}

/// A bulb or an ornament strung on a wave: where, which one, how bright.
#[derive(Clone, Copy)]
struct Lamp {
    x: i32,
    id: i64,
    glow: f32,
}

/// An esfera or a piñata hanging from a crest.
#[derive(Clone, Copy)]
struct Hung {
    x: f32,
    id: u32,
    pinata: bool,
}

pub(super) struct Navidad {
    /// How many layers, from the front, carry esferas and piñatas.
    lit: usize,
    level: f32,
    /// Per layer: its foquitos, esferas and piñata papers.
    bulbs: Vec<[Bulb; 6]>,
    esferas: Vec<[Esfera; 5]>,
    papel: Vec<[Color; 6]>,
    /// The bulbs of the layer being drawn, and what hangs from its crests.
    lamps: Vec<Lamp>,
    hung: Vec<Hung>,
    /// The highest crest among the layers in front of it, per dot column
    /// (filled waves hide what lies behind them).
    front: Vec<f32>,
    /// Blinking clock in seconds.
    t: f32,
}

/// A layer's cord under the holiday, `j` waves from the front: verde pino
/// and rojo nochebuena, or what the wave would be on an ordinary day.
pub(super) fn cord(ctx: &Ctx, j: usize, ordinary: Color) -> Color {
    if ctx.theme.native {
        CORD[j % CORD.len()]
    } else {
        ordinary
    }
}

impl Navidad {
    /// Dresses the waves for the holiday, or retouches the colors after a
    /// theme change.
    pub(super) fn dress(slot: &mut Option<Navidad>, ctx: &Ctx, layers: &[Layer], pw: usize) {
        let n = layers.len();
        let m = slot.get_or_insert_with(|| Navidad {
            lit: 0,
            level: 0.0,
            bulbs: Vec::with_capacity(n),
            esferas: Vec::with_capacity(n),
            papel: Vec::with_capacity(n),
            lamps: Vec::with_capacity(256),
            hung: Vec::with_capacity(MAX_HUNG),
            front: vec![0.0; pw],
            t: 0.0,
        });
        let level = ctx.fiesta.level.clamp(0.0, 1.0);
        m.level = level;
        // Ornaments on the front waves first, on every wave at Nochebuena.
        m.lit = ((n as f32 * (0.35 + 0.65 * level)).ceil() as usize).clamp(1, n);
        m.bulbs.clear();
        m.esferas.clear();
        m.papel.clear();
        for l in layers {
            // The farther back, the dimmer.
            let k = (1.0 - l.depth) * 0.3;
            m.bulbs.push(std::array::from_fn(|i| {
                let lit = ctx.fade(like(ctx.theme, FOQUITOS[i]), k);
                Bulb {
                    lit,
                    dim: ctx.fade(lit, 0.62),
                    hot: lit.lerp(GLOW, 0.55),
                    halo: ctx.fade(lit, 0.4),
                }
            }));
            m.esferas.push(std::array::from_fn(|i| {
                let body = ctx.fade(like(ctx.theme, ESFERAS[i]), k);
                // A gold ball wears a silver cap.
                let cap = like(ctx.theme, if i == 2 { PLATA } else { DORADO });
                Esfera {
                    hi: body.lerp(GLOW, 0.5),
                    body,
                    shade: ctx.fade(body, 0.4),
                    cap: ctx.fade(cap, k),
                }
            }));
            m.papel.push(std::array::from_fn(|i| {
                ctx.fade(like(ctx.theme, PAPEL[i]), k)
            }));
        }
    }

    /// The screen changed size.
    pub(super) fn resized(&mut self, pw: usize) {
        self.front.clear();
        self.front.resize(pw, 0.0);
    }
}

/// How bright a foquito is, 0 to 1, in chase `pat` at time `t`. `i` is its
/// place on the string, 0 to 11: every pattern repeats within twelve bulbs,
/// so the string can wrap without a seam.
fn blink(pat: usize, i: i32, t: f32) -> f32 {
    let i = i.rem_euclid(12);
    let f = i as f32;
    match pat {
        // A comet running along the string: a bright head, a tail of
        // fainter bulbs behind it, one in every six.
        0 => (1.0 - 0.45 * (4.0 * t - f).rem_euclid(6.0)).max(0.0),
        // Odds and evens, taking turns.
        1 => (0.5 + 1.4 * (PI * f + 2.4 * t).cos()).clamp(0.05, 1.0),
        // A slow swell of light rolling along.
        2 => {
            let s = 0.5 + 0.5 * (f * PI / 3.0 - 3.0 * t).sin();
            s * s
        }
        // Every bulb twinkling on its own.
        3 => {
            let h = hash(i as u32 * 7 + 1);
            0.06 + 0.94 * (0.5 + 0.5 * (t * (1.6 + 2.4 * h) + h * 40.0).sin()).powi(3)
        }
        // Three groups, one after the other.
        4 => {
            if i.rem_euclid(3) == ((t * 1.3) as i32).rem_euclid(3) {
                1.0
            } else {
                0.1
            }
        }
        // All lit, breathing a little.
        _ => 0.8 + 0.2 * (2.0 * t + f).sin(),
    }
}

/// Foquito shapes by size, nearest first: socket (`s`), glass (`o`) and the
/// filament's heart (`h`).
const BULB: [&[&str]; 4] = [
    &[
        "..s..", ".sss.", ".ooo.", "ooooo", "oohoo", "ooooo", ".ooo.", "..o..",
    ],
    &[".s.", "sss", "ooo", "oho", "ooo", ".o."],
    &[".s.", "ooo", "oho", ".o."],
    &["s", "o", "o"],
];
/// How far from a bulb's center its halo reaches, in dots.
const HALO: [f32; 4] = [5.0, 4.0, 3.0, 2.0];
/// The spacing of the foquitos, in dots, at the first posada and at
/// Nochebuena, by size.
const GAP: [[f32; 2]; 4] = [[14.0, 8.5], [10.5, 6.5], [8.0, 5.5], [6.0, 4.5]];
/// An esfera's radius, the string it hangs by, and a piñata's radius and
/// rope, by size, in dots.
const BALL: [f32; 4] = [4.0, 3.0, 2.2, 1.5];
const DROP: [i32; 4] = [5, 4, 3, 2];
const STAR: [f32; 4] = [8.0, 6.0, 4.5, 3.0];
const ROPE: [i32; 4] = [4, 3, 2, 2];

impl Waveform {
    /// Moves the holiday on by `dt` seconds: only the lights blink.
    pub(super) fn step_navidad(&mut self, dt: f32) {
        if let Some(m) = self.navidad.as_mut() {
            m.t = (m.t + dt) % 3600.0;
        }
    }

    /// The size class of what hangs on layer `li`, 0 (the nearest and
    /// biggest) to 3 (the farthest); a smaller screen makes everything
    /// smaller, a tall one bigger.
    pub(super) fn tier(&self, li: usize) -> usize {
        let base = match self.layers[li].depth {
            _ if self.layers.len() == 1 => 0,
            d if d > 0.6 => 0,
            d if d > 0.3 => 1,
            d if d > 0.1 => 2,
            _ => 3,
        };
        let big = (self.ph >= 180) as usize;
        let small = (self.ph < 110) as usize + (self.ph < 60) as usize;
        (base + small).saturating_sub(big).min(3)
    }

    /// Calls `f(crest, x, period)` for each crest of layer `li`'s main swell
    /// that shows (with `margin` dots to spare): the number of the crest
    /// stays the same as it rides along, `x` is where it is now and `period`
    /// how far apart the crests are.
    fn crests(&self, li: usize, margin: f32, mut f: impl FnMut(i64, f32, f32)) {
        let l = &self.layers[li];
        let mirror = self.style == Style::Mirror;
        let kx = TAU / self.lambda()
            * if mirror {
                1.0
            } else {
                1.0 + 0.45 * (1.0 - l.depth)
            };
        let lag = if mirror { 0.0 } else { l.lag };
        let wv = self.swell[0];
        // The phase of the swell at column 0, and how fast it turns per dot:
        // a crest (the top of the screen is the smallest y) is where the sine
        // is at its lowest.
        let kk = wv.k as f64 * kx as f64;
        let a0 = wv.k as f64 * lag as f64 - wv.omega as f64 * self.t as f64 + wv.phase as f64;
        let (quarter, turn) = (std::f64::consts::FRAC_PI_2, std::f64::consts::TAU);
        let mut c = ((a0 + quarter - margin as f64 * kk) / turn).ceil() as i64;
        let period = (turn / kk) as f32;
        loop {
            let x = (turn * c as f64 - quarter - a0) / kk;
            if x > self.pw as f64 + margin as f64 {
                break;
            }
            f(c, x as f32, period);
            c += 1;
        }
    }

    /// Fills `front` with the highest crest among the layers in front of
    /// `li`.
    pub(super) fn crests_in_front(&self, front: &mut [f32], li: usize) {
        let pw = self.pw;
        front.fill(f32::INFINITY);
        for lj in li + 1..self.layers.len() {
            for (f, &y) in front.iter_mut().zip(&self.ys[lj * pw..(lj + 1) * pw]) {
                *f = f.min(y);
            }
        }
    }

    pub(super) fn draw_navidad(&mut self) {
        let Some(mut m) = self.navidad.take() else {
            return;
        };
        let n = self.layers.len();
        let center = self.ph as f32 * 0.5 - 0.5;
        if self.style == Style::Filled {
            self.draw_filled();
        }
        for li in 0..n {
            if self.style == Style::Filled {
                self.crests_in_front(&mut m.front, li);
            }
            match self.style {
                Style::Lines => {
                    self.stroke(li, 1.0, 0.0);
                    self.lights(&mut m, li, (1.0, 0.0), true);
                }
                Style::Filled => self.lights(&mut m, li, (1.0, 0.0), true),
                Style::Mirror => {
                    for sign in [1.0, -1.0] {
                        self.stroke(li, sign, center);
                        self.lights(&mut m, li, (sign, center), true);
                    }
                }
                // No cord: the beads become foquitos and esferas.
                Style::Dots => self.lights(&mut m, li, (1.0, 0.0), false),
            }
        }
        self.navidad = Some(m);
    }

    /// The foquitos strung along layer `li` (as `sign * ys + center`),
    /// blinking in a chase, and the esferas and piñatas hanging from its
    /// crests. Without a `cord` they float, centered on the wave.
    fn lights(&mut self, m: &mut Navidad, li: usize, (sign, center): (f32, f32), cord: bool) {
        let (pw, n) = (self.pw, self.layers.len());
        let (tier, th, t, level) = (self.tier(li), self.thickness, m.t, m.level);
        let depth = self.layers[li].depth;
        let filled = self.style == Style::Filled;
        let [far, near] = GAP[tier];
        let gap = near + (far - near) * (1.0 - level);
        let (laps, ride) = self.ride(li, 12.0 * gap);
        // Which chase the string runs, for a while; each wave its own.
        let epoch = (t / 18.0 + li as f32 * 0.37) as u32;
        let pat = (hash(epoch.wrapping_mul(131).wrapping_add(li as u32 * 17 + 3)) * 6.0) as usize;
        // What hangs from the crests: more of it, and a piñata now and then,
        // as the posadas climb.
        m.hung.clear();
        if li + m.lit >= n {
            let rate = 0.08 + 0.1 * level;
            let hung = &mut m.hung;
            self.crests(li, 14.0, |c, x, period| {
                let id = (c.rem_euclid(1 << 20) as u32)
                    .wrapping_mul(131)
                    .wrapping_add(li as u32 * 7919);
                for (j, off) in [0.0, 0.3, -0.3].into_iter().enumerate() {
                    let id = id.wrapping_add(j as u32 * 977);
                    if (j == 0 || hash(id ^ 0x5bd1) < (level - 0.45) * 1.1) && hung.len() < MAX_HUNG
                    {
                        hung.push(Hung {
                            x: x + off * period,
                            id,
                            pinata: hash(id) < rate,
                        });
                    }
                }
            });
        }
        // The bulbs, but where an ornament hangs.
        m.lamps.clear();
        let clear = BALL[tier].max(STAR[tier]) + 2.0;
        let mut k = ((-6.0 - ride) / gap).ceil() as i64;
        loop {
            let x = ride + k as f32 * gap;
            if x >= pw as f32 + 6.0 {
                break;
            }
            let id = k - 12 * laps;
            k += 1;
            if m.hung.iter().any(|h| (h.x - x).abs() < clear) {
                continue;
            }
            let glow = blink(pat, (id % 12) as i32, t);
            m.lamps.push(Lamp {
                x: x.round() as i32,
                id,
                glow,
            });
        }

        let row = &self.ys[li * pw..(li + 1) * pw];
        let wire = |x: i32| {
            let xf = (x as f32).clamp(0.0, pw as f32 - 1.001);
            let i = xf as usize;
            let (a, b) = (row[i], row[(i + 1).min(pw - 1)]);
            rnd(center + sign * (a + (b - a) * (xf - i as f32)))
        };
        let Navidad {
            bulbs,
            esferas,
            papel,
            hung,
            lamps,
            front,
            ..
        } = m;
        let skip =
            |x: i32, y: i32| filled && x >= 0 && x < pw as i32 && front[x as usize] <= y as f32;
        let base = ((1.0 - depth) * 5.0) as usize;
        let socket = self.shades[li * SHADES + (base + 3).min(SHADES - 1)];
        let spr = BULB[tier];
        let b = &mut self.braille;
        // The halos first, so the glass shows through them.
        for l in lamps.iter().filter(|l| l.glow > 0.3) {
            let bulb = &bulbs[li][(l.id + 2 * li as i64).rem_euclid(6) as usize];
            let y = wire(l.x) + if cord { th + spr.len() as i32 / 2 } else { 0 };
            halo(b, (l.x, y), HALO[tier], l.glow, bulb.halo, &skip);
        }
        for l in lamps.iter() {
            let bulb = &bulbs[li][(l.id + 2 * li as i64).rem_euclid(6) as usize];
            let lean = (t * 1.6 + l.id as f32 * 0.7).sin() * 0.9;
            let y = wire(l.x) + if cord { th } else { -(spr.len() as i32) / 2 };
            let cols = [
                if cord { socket } else { bulb.dim },
                bulb.dim.lerp(bulb.lit, l.glow),
                bulb.dim.lerp(bulb.hot, l.glow),
            ];
            foquito_sprite(b, (l.x, y), spr, lean, cols, &skip);
        }
        for h in hung.iter() {
            let y = wire(h.x.round() as i32) + if cord { th } else { 0 };
            let x = h.x.round() as i32;
            let swing = (t * 1.1 + h.id as f32 * 0.37).sin();
            if h.pinata {
                let papel = &papel[li];
                pinata(b, (x, y), tier, (0.3 * swing, socket), cord, papel, &skip);
            } else {
                let e = &esferas[li][(hash(h.id) * 5.0) as usize % 5];
                let band = tier <= 1 && hash(h.id ^ 0x77) < 0.35;
                let glint = (t * 0.9 + hash(h.id ^ 0x3c) * TAU).sin() > 0.97;
                esfera(b, (x, y), tier, (e, socket), (band, glint, cord), &skip);
            }
        }
    }
}

/// A dithered glow round dot (`x`, `y`), `r` dots across, as bright as `glow`.
fn halo(
    b: &mut Braille,
    (x, y): (i32, i32),
    r: f32,
    glow: f32,
    c: Color,
    skip: &impl Fn(i32, i32) -> bool,
) {
    let ri = r.ceil() as i32;
    for dy in -ri..=ri {
        for dx in -ri..=ri {
            let d = ((dx * dx + dy * dy) as f32).sqrt() / r;
            if d > 1.0 {
                continue;
            }
            let density = (1.0 - d).powf(1.5) * 0.75 * glow;
            let (px, py) = (x + dx, y + dy);
            if (BAYER[(py & 3) as usize][(px & 3) as usize] as f32) < density * 16.0
                && !skip(px, py)
            {
                b.set(px, py, c);
            }
        }
    }
}

/// A foquito of `spr` with its top row at `y`, swaying `lean` dots at its
/// tip. `cols` are the socket, the glass and the heart; the heart is drawn
/// last, so it lights its whole cell.
fn foquito_sprite(
    b: &mut Braille,
    (x, y): (i32, i32),
    spr: &[&str],
    lean: f32,
    cols: [Color; 3],
    skip: &impl Fn(i32, i32) -> bool,
) {
    let w = spr[0].len() as i32 / 2;
    for pass in 0..2 {
        for (r, row) in spr.iter().enumerate() {
            let dx = (lean * r as f32 / spr.len() as f32).round() as i32;
            for (i, ch) in row.bytes().enumerate() {
                let c = match (pass, ch) {
                    (0, b's') => cols[0],
                    (0, b'o') => cols[1],
                    (1, b'h') => cols[2],
                    _ => continue,
                };
                let (px, py) = (x + i as i32 - w + dx, y + r as i32);
                if !skip(px, py) {
                    b.set(px, py, c);
                }
            }
        }
    }
}

/// An esfera hanging by its string from dot (`x`, `y`) (or, without a
/// `hang`, floating with its center there): a gold cap, a glass ball lit
/// from the upper left, and now and then a band round it or a glint.
fn esfera(
    b: &mut Braille,
    (x, y): (i32, i32),
    tier: usize,
    (e, string): (&Esfera, Color),
    (band, glint, hang): (bool, bool, bool),
    skip: &impl Fn(i32, i32) -> bool,
) {
    let r = BALL[tier];
    let ri = r.ceil() as i32;
    let cy = if hang { y + DROP[tier] + 2 + ri } else { y };
    let cap_y = cy - ri - 2;
    if hang {
        for yy in y..cap_y {
            if !skip(x, yy) {
                b.set(x, yy, string);
            }
        }
    }
    let cap_w = if tier == 3 { 1 } else { 3 };
    for yy in cap_y..cap_y + 2 {
        for xx in x - cap_w / 2..=x + cap_w / 2 {
            if !skip(xx, yy) {
                b.set(xx, yy, e.cap);
            }
        }
    }
    let (cx0, cy0) = (x.div_euclid(2), cy.div_euclid(4));
    for dy in -ri..=ri {
        for dx in -ri..=ri {
            if (dx * dx + dy * dy) as f32 > r * r + 0.3 || skip(x + dx, cy + dy) {
                continue;
            }
            // Colors follow the braille cells: the upper left is lit, the
            // lower right in shadow.
            let (cdx, cdy) = ((x + dx).div_euclid(2) - cx0, (cy + dy).div_euclid(4) - cy0);
            let c = match (cdx.signum() + cdy.signum(), cdy) {
                _ if band && cdy == 0 => e.cap,
                (s, _) if s < 0 => e.hi,
                (s, _) if s > 0 => e.shade,
                _ => e.body,
            };
            b.set(x + dx, cy + dy, c);
        }
    }
    if glint && tier <= 1 {
        // A little four-pointed twinkle on the shoulder.
        let (gx, gy) = (x - ri / 2 - 1, cy - ri / 2 - 1);
        for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            b.set(gx + dx, gy + dy, GLOW);
        }
    }
}

/// A piñata of siete picos hanging by its rope from dot (`x`, `y`), turned
/// `swing` radians: a ball with seven cones of papel de china. Without a
/// `hang` it floats with its center there.
fn pinata(
    b: &mut Braille,
    (x, y): (i32, i32),
    tier: usize,
    (swing, rope): (f32, Color),
    hang: bool,
    papel: &[Color; 6],
    skip: &impl Fn(i32, i32) -> bool,
) {
    let r = STAR[tier];
    let ri = r.ceil() as i32;
    let cy = if hang { y + ROPE[tier] + ri } else { y };
    if hang {
        for yy in y..cy - ri {
            if !skip(x, yy) {
                b.set(x, yy, rope);
            }
        }
    }
    let r0 = r * 0.5;
    // Turned so that a point stands straight up when it hangs still.
    let turn = swing - PI / 2.0 - TAU / 14.0;
    let sector = |fx: f32, fy: f32| ((fy.atan2(fx) - turn) / TAU * 7.0).rem_euclid(7.0);
    for dy in -ri..=ri {
        for dx in -ri..=ri {
            let (fx, fy) = (dx as f32, dy as f32);
            let d = (fx * fx + fy * fy).sqrt();
            if d > r + 0.3 || skip(x + dx, cy + dy) {
                continue;
            }
            // Each point is a cone, wide at the ball and narrowing to its tip.
            let u = sector(fx, fy);
            if d > r0 && (u - u.floor() - 0.5).abs() > 0.5 * (r + 0.3 - d) / (r - r0) {
                continue;
            }
            // Colors follow the braille cells: a cell takes the paper of the
            // point its center lies on, and the heart of the ball its own.
            let (px, py) = (x + dx, cy + dy);
            let (cfx, cfy) = (
                (px.div_euclid(2) * 2) as f32 + 0.5 - x as f32,
                (py.div_euclid(4) * 4) as f32 + 1.5 - cy as f32,
            );
            let point = POINTS[(sector(cfx, cfy) as usize).min(6)];
            let c = if cfx * cfx + cfy * cfy < r0 * r0 * 0.3 {
                papel[(point + 3) % 6]
            } else {
                papel[point]
            };
            b.set(px, py, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::INFO;
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::render::Canvas;
    use crate::scene::{Opts, Scene};
    use crate::theme::Theme;

    /// A scene of `style` at 120×36 under `theme`, `secs` seconds in, and what it drew.
    fn run(theme: &Theme, style: &str, fiesta: Fiesta, secs: usize) -> (Waveform, Canvas) {
        let opts = Opts::new(INFO.options, format!("style = '{style}'").parse().unwrap());
        let ctx = Ctx {
            theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta,
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        for _ in 0..secs * 30 {
            s.update(&ctx, 1.0 / 30.0);
        }
        let mut c = Canvas::new(120, 36, Color::Reset);
        s.draw(&ctx, &mut c);
        (s, c)
    }

    fn colors(c: &Canvas) -> Vec<Color> {
        c.cells()
            .iter()
            .filter(|c| c.ch != ' ')
            .map(|c| c.fg)
            .collect()
    }

    /// How many frames, of `frames`, show at least one of `of`.
    fn seen(s: &mut Waveform, ctx: &Ctx, frames: usize, of: &[Color]) -> usize {
        let mut c = Canvas::new(120, 36, Color::Reset);
        let mut count = 0;
        for _ in 0..frames {
            s.update(ctx, 1.0 / 30.0);
            c.clear();
            s.draw(ctx, &mut c);
            let on = colors(&c);
            count += of.iter().any(|f| on.contains(f)) as usize;
        }
        count
    }

    /// Navidad strings the waves with foquitos of every color, blinking, and
    /// hangs esferas on the crests (and a piñata now and then); an ordinary
    /// day none of it.
    #[test]
    fn navidad_dresses_the_waves() {
        let classic = Theme::classic();
        for style in ["lines", "filled", "dots", "mirror"] {
            for day in [(12, 12), (12, 24), (1, 6)] {
                let (mut s, _) = run(&classic, style, Fiesta::of_day(day.0, day.1), 12);
                assert!(s.navidad.is_some() && s.muertos.is_none());
                // The front string's foquitos, lit in their own colors one
                // moment or another.
                let front = s.navidad.as_ref().unwrap().bulbs[s.layers.len() - 1];
                let theme = Theme::classic();
                let opts = Opts::new(INFO.options, format!("style = '{style}'").parse().unwrap());
                let ctx = Ctx {
                    theme: &theme,
                    opts: &opts,
                    bg: Color::Rgb(13, 17, 23),
                    seed: 5,
                    fiesta: Fiesta::of_day(day.0, day.1),
                };
                let lit = front
                    .iter()
                    .filter(|b| seen(&mut s, &ctx, 120, &[b.lit]) > 0)
                    .count();
                assert!(lit >= 5, "{style} {day:?}: {lit} colors of bulbs");
            }
            let (s, c) = run(&classic, style, Fiesta::NONE, 12);
            assert!(s.navidad.is_none());
            let plain = colors(&c);
            for c in FOQUITOS.iter().chain(&ESFERAS).chain(&PAPEL).chain(&CORD) {
                assert!(
                    !plain.contains(c),
                    "{style}: holiday color on an ordinary day"
                );
            }
        }
    }

    /// The bulbs really blink: from one moment to the next the string
    /// changes, and each of the chases lights a different set of bulbs.
    #[test]
    fn the_lights_blink_in_chases() {
        // Every pattern is within 0..=1, repeats in twelve, and is not constant.
        for pat in 0..6 {
            let at = |t: f32| (0..12).map(|i| blink(pat, i, t)).collect::<Vec<_>>();
            let (a, b) = (at(0.0), at(1.7));
            assert!(a.iter().chain(&b).all(|v| (0.0..=1.0).contains(v)));
            let all = a
                .iter()
                .chain(&b)
                .chain(&at(3.1))
                .chain(&at(4.3))
                .copied()
                .collect::<Vec<_>>();
            let (lo, hi) = all
                .iter()
                .fold((1.0f32, 0.0f32), |(l, h), &v| (l.min(v), h.max(v)));
            assert!(hi - lo > 0.15, "pattern {pat} never changes: {lo}..{hi}");
        }
        // A pattern repeats along the string every twelve bulbs, in time too.
        for pat in 0..6 {
            for i in 0..12 {
                assert!((blink(pat, i, 2.3) - blink(pat, i + 12, 2.3)).abs() < 1e-3);
            }
        }
        // On screen, a lit bulb's color comes and goes over a few seconds.
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta: Fiesta::of_day(12, 24),
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let bulbs = s.navidad.as_ref().unwrap().bulbs[4];
        for b in bulbs {
            let n = seen(&mut s, &ctx, 240, &[b.lit]);
            assert!(n > 3 && n < 235, "a bulb color lit on {n} of 240 frames");
        }
    }

    /// Esferas (and the piñata now and then) hang from the crests.
    #[test]
    fn esferas_and_pinatas_hang_from_the_crests() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta: Fiesta::of_day(12, 24),
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let front = s.layers.len() - 1;
        let (caps, papers) = {
            let m = s.navidad.as_ref().unwrap();
            (m.esferas[front].map(|e| e.cap), m.papel[front].to_vec())
        };
        // A gold cap on most frames, a piñata's papel on some.
        assert!(seen(&mut s, &ctx, 300, &caps) > 250);
        let pinatas = seen(&mut s, &ctx, 900, &papers);
        assert!(pinatas > 60, "a piñata on {pinatas} of 900 frames");
    }

    /// More bulbs and more ornaments as the posadas climb.
    #[test]
    fn more_of_it_as_the_posadas_climb() {
        let classic = Theme::classic();
        let count = |day: (u32, u32), style: &str| {
            let (s, c) = run(&classic, style, Fiesta::of_day(day.0, day.1), 20);
            let m = s.navidad.as_ref().unwrap();
            (m.lit, m.lamps.len(), colors(&c).len())
        };
        for style in ["lines", "mirror"] {
            let (early, late) = (count((12, 12), style), count((12, 24), style));
            assert!(early.0 < late.0, "{style}: ornaments on {early:?} {late:?}");
            assert!(
                early.2 < late.2,
                "{style}: nothing grows: {early:?} {late:?}"
            );
        }
        // The spacing tightens: more bulbs on the front wave.
        let lamps = |day: (u32, u32)| {
            let (s, _) = run(&classic, "lines", Fiesta::of_day(day.0, day.1), 5);
            s.navidad.unwrap().lamps.len()
        };
        assert!(lamps((12, 12)) < lamps((12, 24)));
    }

    /// The holiday comes and goes through `retheme`, and leaves the ordinary
    /// picture exactly as it was.
    #[test]
    fn navidad_comes_and_goes() {
        let theme = Theme::builtin("dracula").unwrap();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let mut ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: theme.bg,
            seed: 9,
            fiesta: Fiesta::NONE,
        };
        let frame = |s: &mut Waveform, ctx: &Ctx| {
            let mut c = Canvas::new(90, 30, Color::Reset);
            s.draw(ctx, &mut c);
            c.cells().to_vec()
        };
        let mut plain = Waveform::new(&ctx);
        let mut s = Waveform::new(&ctx);
        for w in [&mut plain, &mut s] {
            w.resize(&ctx, 90, 30);
            w.retheme(&ctx);
        }
        let before = frame(&mut s, &ctx);
        ctx.fiesta = Fiesta::of_day(12, 20);
        s.retheme(&ctx);
        for _ in 0..90 {
            s.update(&ctx, 1.0 / 30.0);
        }
        assert!(s.navidad.is_some());
        assert_ne!(frame(&mut s, &ctx), before);
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert!(s.navidad.is_none());
        for _ in 0..90 {
            plain.update(&ctx, 1.0 / 30.0);
        }
        assert!(frame(&mut s, &ctx) == frame(&mut plain, &ctx));
        // And back again, with a different day: the same scene, redressed.
        ctx.fiesta = Fiesta::of_day(12, 12);
        s.retheme(&ctx);
        assert_eq!(s.navidad.as_ref().map(|m| m.level), Some(0.3));
        ctx.fiesta = Fiesta::of_day(12, 24);
        s.retheme(&ctx);
        assert_eq!(s.navidad.as_ref().map(|m| m.level), Some(1.0));
        // Another holiday leaves it ordinary.
        ctx.fiesta = Fiesta::of_day(11, 20);
        s.retheme(&ctx);
        assert!(s.navidad.is_none() && s.muertos.is_none());
    }

    /// The same seed paints the same holiday, and it follows the theme.
    #[test]
    fn navidad_is_deterministic_and_themed() {
        let day = Fiesta::of_day(12, 24);
        let nord = Theme::builtin("nord").unwrap();
        let (_, a) = run(&nord, "lines", day, 15);
        let (_, b) = run(&nord, "lines", day, 15);
        assert!(a.cells() == b.cells());
        let (_, classic) = run(&Theme::classic(), "lines", day, 15);
        assert!(a.cells() != classic.cells());
        // Under another theme the cord and the bulbs are the theme's.
        let on = colors(&a);
        for c in CORD.iter().chain(&FOQUITOS).chain(&PAPEL) {
            assert!(!on.contains(c), "a classic color under nord");
        }
    }

    /// A resize leaves the holiday standing on the new size.
    #[test]
    fn navidad_survives_a_resize() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: theme.bg,
            seed: 2,
            fiesta: Fiesta::of_day(12, 24),
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        for _ in 0..60 {
            s.update(&ctx, 1.0 / 30.0);
        }
        s.resize(&ctx, 40, 12);
        let mut c = Canvas::new(40, 12, Color::Reset);
        s.draw(&ctx, &mut c);
        assert!(!s.navidad.as_ref().unwrap().lamps.is_empty());
    }
}
