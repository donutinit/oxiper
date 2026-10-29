//! A comparsa dances across the bottom of the screen during Carnaval, under
//! bursts of confetti.
//!
//! A comparsa is the troupe of dancers that parades through the streets at
//! Carnaval (as in Veracruz and Mazatlán) in costumes of feathers and
//! sequins. Original oxiper pixel art: each dancer in a sequined bodice and a
//! skirt of three tiers of feathers, a jeweled headband under a fan of
//! plumas in rosa, turquesa, morado, verde limón and gold, great plumed
//! wings spread behind the shoulders and gold bracelets. They come in a line,
//! each in her own colors and skin tone, step to the beat with the knees
//! lifting by turns, shimmy (shoulders and hips shaking against each other),
//! wave their arms over their heads, and every plume trembles. Now and then
//! a burst of confetti goes off over the troupe, with a few serpentinas
//! unrolling, and the pieces tumble down flashing in every bright color. The
//! crowd and the dancers scale with the screen and cross from either side
//! within 30 s.

use super::blit::{Xf, blit, block, fit, hash, line, plume};
use super::carnaval::Palette;
use super::{Egg, Info, pixel};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "comparsa",
    start,
    fiesta: Kind::Carnaval,
    when: super::any_day,
};

/// Facing the front, without arms, wings and plumes, which are drawn around
/// it, or legs, which are drawn below the skirt. `E` and `e` are the colors
/// of the costume, `X` and `x` its second color, `S` and `s` the skin (each
/// dancer has her own tone) and `L` the lentejuelas.
const DANCER: &[&str] = &[
    ".......KKKKKKK.......",
    "......KLKLKLKLK......",
    "......KKSSSSSKK......",
    "......KSSSSSSSK......",
    "......KSkSSSkSK......",
    "......KSSSSSSSK......",
    "......LSSUUUSSL......",
    ".......KSSSSSK.......",
    ".......K.sSs.K.......",
    "....sSEESSSSSEESs....",
    "....sSLEESSSEELSs....",
    "....eEEELESELEEEe....",
    ".....eLEEELEEELe.....",
    ".....eEELEEELEEe.....",
    "......eEEELEEEe......",
    "......eELEEELEe......",
    "......eEEELEEEe......",
    "......LLLLLLLLL......",
    "......eEEeEEeEe......",
    ".....eEEeEEeEEee.....",
    ".....xXxXXxXXxXx.....",
    "....xXxXXxXXxXXxx....",
    "....eeEEeEEeEEeEe....",
    "...eeEEeEEeEEeEEee...",
    "..aaLLaLLaLLaLLaLLa..",
    "..L.L.L.L.L.L.L.L.L..",
];
const WIDTH: i32 = 21;
/// The art, and the legs and feet below it: the dancer's height.
const BODY: i32 = DANCER.len() as i32;
const HEIGHT: i32 = BODY + 11;
/// The rows the bodice and the skirt sway on, and the shoulders and hips.
const BODICE: i32 = 9;
const SKIRT: i32 = 18;
const SHOULDERS: [f32; 2] = [5.0, 16.0];
const HIPS: [f32; 2] = [7.5, 13.5];
/// How far the plumes of the headdress and the wings reach above the art.
const REACH: f32 = 17.0;
/// How long an arm, a thigh and a shin are, in art pixels.
const ARM: f32 = 13.0;
const LEG: f32 = 6.9;

/// A costume: the colors of `E`, `e`, `X` and `x`.
const COSTUMES: [[u8; 4]; 6] = [*b"RTQq", *b"QqYa", *b"MmGg", *b"GgRT", *b"YaMm", *b"BbRT"];
/// Skin tones, light, medium and dark: the colors of `S` and `s`.
const SKINS: [[u8; 2]; 3] = [*b"Ss", *b"Hh", *b"Jj"];
/// The plumes' colors, with the lighter spine of each.
const PLUMES: [(u8, u8); 6] = [
    (b'R', b'r'),
    (b'Q', b'z'),
    (b'Y', b'L'),
    (b'G', b'l'),
    (b'M', b'P'),
    (b'O', b'o'),
];
/// The colors of the confetti.
const CONFETTI: [u8; 8] = *b"RQMGYOUB";

/// Steps per second, and seconds the troupe takes to cross.
const STEPS: f32 = 1.8;
const CROSSING: f32 = 21.0;

/// Pieces of confetti in the air at most, how many a burst throws, how
/// long one lasts and how it falls (pixels per second squared).
const PIECES: usize = 220;
const SERPENTINAS: usize = 10;
const LIFE: f32 = 3.2;
const GRAVITY: f32 = 16.0;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let mut rng = fastrand::Rng::with_seed(rng.u64(..));
    let k = fit(h, HEIGHT + REACH as i32, 0.8);
    let cell = (WIDTH as f32 * k * 1.15).round() as i32;
    let n = (w as i32 / cell.max(1)).clamp(2, 7) as usize;
    let first = rng.usize(..COSTUMES.len());
    let crowd = (0..n)
        .map(|i| Dancer {
            costume: COSTUMES[(first + i * 5) % COSTUMES.len()],
            skin: SKINS[rng.usize(..SKINS.len())],
            plumes: rng.usize(..PLUMES.len()),
            beat: rng.f32() * 0.3,
        })
        .collect();
    let span = n as i32 * cell;
    Box::new(Comparsa {
        t: 0.0,
        dist: 0.0,
        speed: ((w as i32 + span) as f32 / CROSSING).max(5.0),
        left: rng.bool(),
        w: w as i32,
        h: h as i32 * 2,
        k,
        cell,
        crowd,
        pieces: [Piece::default(); PIECES],
        streamers: [Streamer::default(); SERPENTINAS],
        next: 0,
        next_streamer: 0,
        burst_in: 1.2,
        flash: (0.0, 0.0, 0.0),
        rng,
    })
}

struct Dancer {
    /// The colors of `E`, `e`, `X` and `x`.
    costume: [u8; 4],
    /// The colors of `S` and `s`.
    skin: [u8; 2],
    /// Which plume color the fan starts with.
    plumes: usize,
    /// How far behind the beat she is, in seconds.
    beat: f32,
}

/// A piece of confetti, in pixels from the top-left corner.
#[derive(Clone, Copy, Default)]
struct Piece {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Seconds in the air; 0 means none.
    age: f32,
    life: f32,
    color: u8,
    spin: f32,
}

/// A serpentina unrolling from a burst: it flies like a piece of confetti
/// and leaves a wavering ribbon behind.
#[derive(Clone, Copy, Default)]
struct Streamer {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    color: u8,
    spin: f32,
}

struct Comparsa {
    t: f32,
    dist: f32,
    speed: f32,
    left: bool,
    w: i32,
    /// The screen's height in pixels.
    h: i32,
    k: f32,
    cell: i32,
    crowd: Vec<Dancer>,
    pieces: [Piece; PIECES],
    streamers: [Streamer; SERPENTINAS],
    next: usize,
    next_streamer: usize,
    /// Seconds until the next burst, and where the last one flashed and for
    /// how much longer.
    burst_in: f32,
    flash: (f32, f32, f32),
    rng: fastrand::Rng,
}

impl Comparsa {
    /// The column the troupe's back is at.
    fn origin(&self) -> i32 {
        let span = self.cell * self.crowd.len() as i32;
        let d = self.dist as i32;
        if self.left { self.w - d } else { d - span }
    }

    /// The left column of the art of dancer `i`.
    fn x(&self, i: usize) -> i32 {
        let slot = if self.left {
            i
        } else {
            self.crowd.len() - 1 - i
        } as i32;
        let art = (WIDTH as f32 * self.k).round() as i32;
        self.origin() + slot * self.cell + (self.cell - art) / 2
    }

    /// Has the troupe left the screen?
    fn gone(&self) -> bool {
        let span = self.cell * self.crowd.len() as i32;
        if self.left {
            self.origin() + span < 0
        } else {
            self.origin() > self.w
        }
    }

    /// Throws a burst of confetti and serpentinas from over the troupe.
    fn burst(&mut self) {
        // Bigger screens, bigger bursts.
        let k = (self.h as f32 / 70.0).clamp(1.0, 3.5);
        let span = self.cell * self.crowd.len() as i32;
        let (lo, hi) = (self.origin().max(0), (self.origin() + span).min(self.w - 1));
        if hi <= lo {
            return;
        }
        let x = lo as f32 + self.rng.f32() * (hi - lo) as f32;
        let y = self.h as f32 * (0.14 + 0.3 * self.rng.f32());
        self.flash = (x, y, FLASH);
        let n = ((self.w * self.h) / 90).clamp(30, 110);
        for _ in 0..n {
            let (ang, speed) = (
                self.rng.f32() * TAU,
                (10.0 + 34.0 * self.rng.f32().sqrt()) * k,
            );
            self.pieces[self.next] = Piece {
                x,
                y,
                vx: ang.cos() * speed,
                vy: ang.sin() * speed * 0.8 - 4.0,
                age: 1e-3,
                life: LIFE * (0.6 + 0.4 * self.rng.f32()),
                color: self.rng.u8(..CONFETTI.len() as u8),
                spin: self.rng.f32() * TAU,
            };
            self.next = (self.next + 1) % PIECES;
        }
        for _ in 0..3 {
            let (ang, speed) = (self.rng.f32() * TAU, (18.0 + 18.0 * self.rng.f32()) * k);
            self.streamers[self.next_streamer] = Streamer {
                x,
                y,
                vx: ang.cos() * speed,
                vy: ang.sin() * speed - 8.0,
                age: 1e-3,
                color: self.rng.u8(..CONFETTI.len() as u8),
                spin: self.rng.f32() * TAU,
            };
            self.next_streamer = (self.next_streamer + 1) % SERPENTINAS;
        }
    }
}

/// How long the flash of a burst lasts.
const FLASH: f32 = 0.16;
/// How long a serpentina stays in the air, and how many pixels of ribbon it
/// has.
const STREAMER_LIFE: f32 = 2.4;
const RIBBON: i32 = 18;

impl Egg for Comparsa {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
        self.flash.2 = (self.flash.2 - dt).max(0.0);
        for p in self.pieces.iter_mut().filter(|p| p.age > 0.0) {
            p.age += dt;
            // The air slows it down and it flutters as it falls.
            let drag = (1.0 - 1.8 * dt).max(0.0);
            p.vx *= drag;
            p.vy = p.vy * drag + GRAVITY * dt;
            p.x += (p.vx + (p.age * 7.0 + p.spin).sin() * 5.0) * dt;
            p.y += p.vy * dt;
            if p.age > p.life || p.y > self.h as f32 + 2.0 {
                p.age = 0.0;
            }
        }
        for s in self.streamers.iter_mut().filter(|s| s.age > 0.0) {
            s.age += dt;
            let drag = (1.0 - 1.4 * dt).max(0.0);
            s.vx *= drag;
            s.vy = s.vy * drag + GRAVITY * 0.8 * dt;
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            if s.age > STREAMER_LIFE {
                s.age = 0.0;
            }
        }
        self.burst_in -= dt;
        if self.burst_in <= 0.0 {
            self.burst_in = 1.1 + 1.1 * self.rng.f32();
            // None as the troupe leaves.
            if !self.gone() && self.t < 25.0 {
                self.burst();
            }
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::carnaval(theme);
        let k = self.k;
        let ground = c.h() as i32 * 2;
        let wide = (WIDTH as f32 * k).round() as i32;
        let reach = (REACH * k) as i32 + (10.0 * k) as i32;
        for (i, d) in self.crowd.iter().enumerate() {
            let x = self.x(i);
            if x + wide + reach < 0 || x - reach > self.w {
                continue;
            }
            let beat = (self.t - d.beat) * STEPS;
            let phase = beat * PI;
            // The body drops a little as a foot comes down, and sways.
            let bob = (0.5 + 0.5 * (phase * 2.0).cos()).round();
            let sway = (phase.sin() * 1.5).round();
            let top = ground - (HEIGHT as f32 * k).round() as i32 + (bob * k) as i32;
            let x = x + (sway * k) as i32;
            // The shimmy: shoulders and hips shake against each other.
            let shake = ((self.t + d.beat) * 21.0).sin().round() as i32;
            let [e, shade, a, ax] = d.costume;
            let [s, ss] = d.skin;
            let color = |l: u8| match l {
                b'E' => pal.get(e),
                b'e' => pal.get(shade),
                b'X' => pal.get(a),
                b'x' => pal.get(ax),
                b'S' => pal.get(s),
                b's' => pal.get(ss),
                _ => pal.get(l),
            };
            let at = |ax: f32, ay: f32| (x as f32 + ax * k, top as f32 + ay * k);
            let thick = |t: f32| ((t * k).round() as i32).max(1);
            // The wings behind the shoulders, then the fan over the head.
            let trem = (self.t * 11.0 + i as f32).sin() * 0.06;
            for (side, base) in [(-1.0f32, at(6.5, 11.0)), (1.0, at(14.5, 11.0))] {
                for (n, (ang, len)) in [(0.55, 17.0), (0.85, 14.0), (1.15, 11.0)]
                    .into_iter()
                    .enumerate()
                {
                    let (cl, sp) =
                        PLUMES[(d.plumes + n * 2 + usize::from(side > 0.0)) % PLUMES.len()];
                    let ang = ang + trem + shake as f32 * 0.03;
                    let tip = (
                        base.0 + side * ang.sin() * len * k,
                        base.1 - ang.cos() * len * k,
                    );
                    plume(
                        c,
                        base,
                        tip,
                        side * 1.2 * k,
                        4.2 * k,
                        (pal.of(cl), pal.of(sp)),
                    );
                }
            }
            let crown = at(10.5, 1.0);
            for n in 0..9 {
                let f = n as f32 / 8.0 - 0.5;
                let (cl, sp) = PLUMES[(d.plumes + n) % PLUMES.len()];
                let ang = f * 2.5 + trem * 2.0 + shake as f32 * 0.03;
                let len = (REACH - 2.0) * (1.0 - 0.45 * (f * 2.0).powi(2));
                let tip = (crown.0 + ang.sin() * len * k, crown.1 - ang.cos() * len * k);
                plume(
                    c,
                    crown,
                    tip,
                    f * 2.0 * k,
                    3.4 * k,
                    (pal.of(cl), pal.of(sp)),
                );
            }
            // The arms over her head, waving, with gold bracelets and a
            // hand at the end.
            for (side, sx) in SHOULDERS.into_iter().enumerate() {
                let sgn = if side == 0 { -1.0 } else { 1.0 };
                let wave = (phase * 2.0 + if side == 0 { 0.0 } else { PI } + d.beat * 7.0).sin();
                let ang = 2.45 + 0.45 * wave;
                let from = at(sx + 0.5, 10.5);
                let to = (
                    from.0 + sgn * ang.sin() * ARM * k,
                    from.1 + ang.cos() * ARM * k,
                );
                line(c, from, to, thick(2.0), pal.of(s));
                let wrist = (
                    from.0 + (to.0 - from.0) * 0.82,
                    from.1 + (to.1 - from.1) * 0.82,
                );
                let cuff = (
                    wrist.0 + (to.0 - from.0) * 0.08,
                    wrist.1 + (to.1 - from.1) * 0.08,
                );
                line(c, wrist, cuff, thick(3.0), pal.of(b'L'));
                block(
                    c,
                    to.0.round() as i32 - thick(1.0),
                    to.1.round() as i32 - thick(1.0),
                    thick(2.0),
                    pal.of(s),
                );
            }
            // The legs, a knee lifting by turns.
            for (side, hip_x) in HIPS.into_iter().enumerate() {
                let up = (phase + if side == 0 { 0.0 } else { PI }).sin().max(0.0);
                let hip = at(hip_x, 24.0);
                let sgn = if side == 0 { -1.0 } else { 1.0 };
                let foot = at(
                    hip_x + sgn * (1.0 + 2.5 * up),
                    HEIGHT as f32 - 1.0 - 6.0 * up - bob,
                );
                let d = ((foot.0 - hip.0).powi(2) + (foot.1 - hip.1).powi(2))
                    .sqrt()
                    .max(0.01);
                let (l, half) = (LEG * k, d / 2.0);
                let h = (l * l - half * half).max(0.0).sqrt();
                let (vx, vy) = ((foot.0 - hip.0) / d, (foot.1 - hip.1) / d);
                let knee = (
                    (hip.0 + foot.0) / 2.0 + vy * h * sgn,
                    (hip.1 + foot.1) / 2.0 - vx * h * sgn,
                );
                line(c, hip, knee, thick(3.0), pal.of(s));
                line(c, knee, foot, thick(3.0), pal.of(e));
                let mid = ((knee.0 + foot.0) / 2.0, (knee.1 + foot.1) / 2.0);
                pixel(c, mid.0.round() as i32, mid.1.round() as i32, pal.of(b'L'));
                block(
                    c,
                    foot.0.round() as i32 - thick(2.0),
                    foot.1.round() as i32,
                    thick(2.0),
                    pal.of(b'N'),
                );
                block(
                    c,
                    foot.0.round() as i32,
                    foot.1.round() as i32,
                    thick(2.0),
                    pal.of(b'N'),
                );
            }
            let shift = |r: i32| {
                if r < BODICE {
                    0
                } else if r < SKIRT {
                    shake
                } else {
                    -shake + (sway as i32 / 2)
                }
            };
            blit(c, DANCER, Xf::new(x, top, k), shift, color);
            // Sequins, flashing.
            let tick = (self.t * 10.0 + i as f32 * 3.0) as u32;
            for g in 0..4u32 {
                let r = (hash(tick, g * 7 + i as u32) * BODY as f32) as usize;
                let col = (hash(tick + 99, g * 5 + i as u32) * WIDTH as f32) as usize;
                if DANCER[r].as_bytes().get(col) == Some(&b'L') {
                    let (px, py) = (x + (col as f32 * k) as i32, top + (r as f32 * k) as i32);
                    pixel(c, px, py, pal.of(b'W'));
                }
            }
        }
        // The confetti, tumbling: each piece flashes between its color and a
        // paler one as it turns.
        let size = (self.h / 50).clamp(1, 4);
        for p in self.pieces.iter().filter(|p| p.age > 0.0) {
            let col = pal.of(CONFETTI[p.color as usize]);
            let col = if (p.age * 9.0 + p.spin) as i32 % 2 == 0 {
                col
            } else {
                col.lerp(pal.of(b'W'), 0.5)
            };
            let col = fade_out(theme, col, p.age / p.life);
            block(c, p.x.round() as i32, p.y.round() as i32, size, col);
        }
        // The serpentinas: a ribbon that waves behind the head of each.
        for s in self.streamers.iter().filter(|s| s.age > 0.0) {
            let col = pal.of(CONFETTI[s.color as usize]);
            for j in 0..RIBBON {
                let back = j as f32 * 0.035;
                let a = (s.age - back).max(0.0);
                if a <= 0.0 {
                    break;
                }
                // Where it was `back` seconds ago, waving across its path.
                let (x, y) = (s.x - s.vx * back, s.y - s.vy * back);
                let wob = (a * 13.0 + j as f32 * 0.7 + s.spin).sin()
                    * 2.2
                    * (j as f32 / RIBBON as f32 + 0.2);
                let c2 = fade_out(theme, col, (s.age / STREAMER_LIFE).powi(2));
                block(c, (x + wob).round() as i32, y.round() as i32, size, c2);
            }
        }
        // The flash of the burst: a white star that shrinks away.
        let (fx, fy, left) = self.flash;
        if left > 0.0 {
            let r = (left / FLASH * 5.0 * k.max(1.0)).round() as i32;
            let (fx, fy) = (fx.round() as i32, fy.round() as i32);
            for d in -r..=r {
                pixel(c, fx + d, fy, pal.of(b'W'));
                pixel(c, fx, fy + d, pal.of(b'W'));
            }
        }
    }

    fn done(&self) -> bool {
        let idle =
            self.pieces.iter().all(|p| p.age == 0.0) && self.streamers.iter().all(|s| s.age == 0.0);
        self.gone() && idle || self.t > 29.5
    }
}

/// A color fading out toward the end of a piece's life.
fn fade_out(theme: &Theme, c: Color, k: f32) -> Color {
    super::carnaval::fade(theme, c, ((k - 0.7) / 0.3).clamp(0.0, 1.0) * 0.85)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        assert!(DANCER.iter().all(|r| r.len() as i32 == WIDTH));
        const { assert!(BODICE < SKIRT && SKIRT < BODY) };
        let pal = Palette::carnaval(&Theme::classic());
        for l in COSTUMES.iter().flatten().chain(SKINS.iter().flatten()) {
            assert!(pal.get(*l).is_some(), "{}", *l as char);
        }
        for &b in DANCER.iter().flat_map(|r| r.as_bytes()) {
            assert!(
                b == b'.' || b"EeXxSs".contains(&b) || pal.get(b).is_some(),
                "{}",
                b as char
            );
        }
    }

    /// The dancers cross the screen, and confetti bursts over them: pieces
    /// of every color show up above their heads, and they are all gone by
    /// the end.
    #[test]
    fn dances_across_under_confetti() {
        let theme = Theme::classic();
        for seed in 0..4 {
            let mut e = start(120, 36, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(120, 36, Color::Reset);
            let (mut secs, mut high, mut colors) = (0.0, 0, std::collections::HashSet::new());
            while !e.done() {
                e.update(0.05);
                secs += 0.05;
                c.clear();
                e.draw(&mut c, &theme);
                // The top of the screen is above the dancers' plumes.
                for cell in c.cells().iter().take(120 * 7).filter(|cell| cell.ch != ' ') {
                    high += 1;
                    colors.insert(format!("{:?}", cell.fg));
                }
            }
            assert!(secs < 29.0, "{secs}");
            assert!(high > 40, "{high} cells of confetti");
            assert!(colors.len() >= 5, "{} colors", colors.len());
            assert!(c.cells().iter().all(|cell| cell.ch == ' '));
        }
    }
}
