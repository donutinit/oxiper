//! Carnaval in the particles: serpentinas and confetti swirling in the flow,
//! with lentejuelas flashing.
//!
//! The river runs in every bright color of the carnaval. Confetti swirls in
//! it, piece after piece in rosa, amarillo, naranja, turquesa, morado, verde
//! limón, rojo and azul, tumbling and leaving a wake of its own color;
//! lentejuelas (sequins) turn over in the current and flash gold, now and
//! then into a star; plumas drift slowly by. Serpentinas, long streamers of
//! curling paper in stripes, ride the current, twisting and uncurling as they
//! age. Bursts of confetti go off in the stream. In the fireworks some shells
//! burst into a shower of confetti, lentejuelas and serpentinas that flutter
//! down; the fountain tosses them with its water. How much there is follows
//! `fiesta.level`: a few on the Thursday of the quema del mal humor, more
//! every day, the most on Martes de Carnaval.
//!
//! Everything here draws from its own RNG, so an ordinary day's particles
//! are never disturbed.

use std::f32::consts::TAU;

use super::art::{Bit, Bits, Burst, Physics, Pts, Stars, Wind, Wire, rnd, turns};
use super::{Costume, Dots, Field, Mode, Particles};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::Braille;
use crate::scene::Ctx;

/// The bright colors of the carnaval: rosa mexicano, amarillo, naranja,
/// turquesa, morado, verde limón, rojo and azul.
const BRIGHTS: [Color; 8] = [
    rgb(0xe4007c),
    rgb(0xffd200),
    rgb(0xff8c00),
    rgb(0x00aab8),
    rgb(0x9a4fd0),
    rgb(0x8cc828),
    rgb(0xe8322a),
    rgb(0x1f7ae0),
];

/// Lentejuela dorada, and the white of its flash.
const ORO: Color = rgb(0xffcc33);
const BLANCO: Color = rgb(0xf4f1ea);

/// The flow (and fountain) ramp under `classic`: every bright color.
pub(super) const RAMP: [Color; 6] = [
    BRIGHTS[0], BRIGHTS[2], BRIGHTS[1], BRIGHTS[5], BRIGHTS[3], BRIGHTS[4],
];

/// Shell colors under `classic`.
pub(super) const SHELLS: [Color; 8] = [
    BRIGHTS[0], BRIGHTS[1], BRIGHTS[3], BRIGHTS[2], BRIGHTS[5], BRIGHTS[4], BRIGHTS[6], BRIGHTS[7],
];

/// Wakes, appended to the trail palette: each bright color, the oro of the
/// lentejuelas and the white-hot heart of a burst.
pub(super) const WAKES: [Color; 10] = [
    BRIGHTS[0], BRIGHTS[1], BRIGHTS[2], BRIGHTS[3], BRIGHTS[4], BRIGHTS[5], BRIGHTS[6], BRIGHTS[7],
    ORO, BLANCO,
];

/// The colors of the plumas: rosa, turquesa, morado and verde limón.
const PLUMAS: [usize; 4] = [0, 3, 4, 5];

/// What a bit is.
const CONFETTI: u8 = 0;
const LENTEJUELA: u8 = 1;
const PLUMA: u8 = 2;

/// How each kind of bit moves: confetti and lentejuelas swirl about in the
/// current, plumas float along with it slowly.
const PHYSICS: [Physics; 3] = [
    Physics {
        gain: 0.95,
        follow: 3.5,
        flutter: 6.0,
    },
    Physics {
        gain: 0.9,
        follow: 3.0,
        flutter: 5.0,
    },
    Physics {
        gain: 0.6,
        follow: 1.4,
        flutter: 3.0,
    },
];

/// A serpentina's head moves like confetti, only slower.
const STREAMER: Physics = Physics {
    gain: 0.8,
    follow: 2.0,
    flutter: 2.0,
};

/// A piece of confetti as it tumbles: face on, tilting, edge on, tilting
/// back (dot offsets).
const TUMBLE: [&[(i32, i32)]; 4] = [
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
    &[(0, 0), (1, 0), (1, 1)],
    &[(0, 0), (1, 0)],
    &[(1, 0), (0, 1), (1, 1)],
];

/// A lentejuela catching the light: a little star.
const SPARKLE: [(i32, i32); 5] = [(0, -1), (-1, 0), (0, 0), (1, 0), (0, 1)];

/// Points along a serpentina behind its head.
const NODES: usize = 20;

/// A serpentina: a streamer of curling paper. Its head rides the flow (or
/// falls) and the body follows the head's path.
#[derive(Clone, Copy)]
struct Streamer {
    head: Bit,
    /// Where the head has been, nearest first.
    body: [(f32, f32); NODES],
}

pub(super) struct Carnaval {
    fiesta: Fiesta,
    rng: fastrand::Rng,
    mode: Mode,
    pw: f32,
    ph: f32,
    jets: usize,
    bits: Bits,
    streamers: Vec<Streamer>,
    /// How many serpentinas ride the flow.
    streamer_target: usize,
    /// Bursts of confetti in the stream.
    stars: Stars,
    toss_acc: f32,
    /// First trail palette slot of the wakes.
    slot: u8,
    /// The slots a burst picks its colors from: the river's colors and oro.
    burst_cols: [u8; 7],
    confetti_cols: [Color; 8],
    oro: Color,
    blanco: Color,
}

impl Carnaval {
    pub fn new(seed: u64) -> Carnaval {
        Carnaval {
            fiesta: Fiesta::NONE,
            rng: fastrand::Rng::with_seed(seed ^ 0x4341_524E_4156_414C),
            mode: Mode::Flow,
            pw: 0.0,
            ph: 0.0,
            jets: 1,
            bits: Bits::default(),
            streamers: Vec::new(),
            streamer_target: 0,
            stars: Stars::default(),
            toss_acc: 0.0,
            slot: 0,
            burst_cols: [0; 7],
            confetti_cols: BRIGHTS,
            oro: ORO,
            blanco: BLANCO,
        }
    }

    /// The dots between the points of a serpentina, by the size of the screen.
    fn gap(&self) -> f32 {
        (self.ph * 0.012).clamp(2.0, 4.5)
    }

    /// A new serpentina at `at`, laid out straight behind its head.
    fn streamer(&mut self, at: (f32, f32), v: (f32, f32), falls: bool) -> Streamer {
        let tone = self.rng.u8(0..BRIGHTS.len() as u8);
        let mut head = Bit::cast(&mut self.rng, at, v, 0, tone, falls);
        head.life += 4.0;
        let a = self.rng.f32() * TAU;
        let gap = self.gap();
        let mut body = [at; NODES];
        for (i, p) in body.iter_mut().enumerate() {
            let d = gap * (i + 1) as f32;
            *p = (at.0 - a.cos() * d, at.1 - a.sin() * d);
        }
        Streamer { head, body }
    }

    /// A serpentina somewhere on the screen, fading in.
    fn loose_streamer(&mut self) -> Streamer {
        let at = (self.rng.f32() * self.pw, self.rng.f32() * self.ph);
        let mut s = self.streamer(at, (0.0, 0.0), false);
        s.head.age = 0.0;
        s
    }

    /// Fountain: the jets toss confetti and serpentinas with the water.
    fn toss(&mut self, dt: f32) {
        let level = self.fiesta.level;
        self.toss_acc += dt * (1.5 + 4.0 * level) * self.jets as f32;
        while self.toss_acc >= 1.0 {
            self.toss_acc -= 1.0;
            let jet = self.rng.usize(0..self.jets);
            let x = self.pw * (jet as f32 + 0.5) / self.jets as f32;
            let v = (2.0 * 1.1 * self.ph * self.ph * (0.5 + 0.3 * self.rng.f32())).sqrt();
            let a = (self.rng.f32() - 0.5) * 0.7;
            let (at, vel) = ((x, self.ph - 2.0), (a.sin() * v, -a.cos() * v));
            if self.rng.f32() < 0.12 {
                if self.streamers.len() < self.streamers.capacity() {
                    let s = self.streamer(at, vel, true);
                    self.streamers.push(s);
                }
            } else if !self.bits.full() {
                let (kind, tone) = pick(&mut self.rng);
                let b = Bit::cast(&mut self.rng, at, vel, kind, tone, true);
                self.bits.add(b);
            }
        }
    }

    fn update_streamers(&mut self, dt: f32, wind: &Wind, dots: &mut Dots) {
        let (pw, ph) = (dots.pw as f32, dots.ph as f32);
        let follow = 1.0 - (-STREAMER.follow * dt).exp();
        let (g, fall) = (ph * 1.1, (ph * 0.06).max(2.0));
        let gap = self.gap();
        let tail = wind.now.wrapping_sub((wind.span as f32 * 0.05) as u32);
        let mut i = 0;
        while i < self.streamers.len() {
            let mut s = self.streamers[i];
            let (ox, oy) = (s.head.x, s.head.y);
            s.head.drift(dt, wind, Some(&STREAMER), follow, (g, fall));
            if s.head.gone(pw, ph) {
                if s.head.falls || self.streamers.len() > self.streamer_target {
                    self.streamers.swap_remove(i);
                } else {
                    self.streamers[i] = self.loose_streamer();
                    i += 1;
                }
                continue;
            }
            // The body follows the head's path, a point at a time.
            let (dx, dy) = (s.head.x - s.body[0].0, s.head.y - s.body[0].1);
            if dx * dx + dy * dy >= gap * gap {
                s.body.copy_within(0..NODES - 1, 1);
                s.body[0] = (s.head.x, s.head.y);
            }
            dots.line(ox, oy, s.head.x, s.head.y, tail, self.slot + s.head.tone);
            self.streamers[i] = s;
            i += 1;
        }
    }
}

/// The kind and color of a bit that rides the flow: confetti mostly, then
/// lentejuelas and now and then a pluma.
fn pick(rng: &mut fastrand::Rng) -> (u8, u8) {
    let r = rng.f32();
    if r < 0.78 {
        (CONFETTI, rng.u8(0..BRIGHTS.len() as u8))
    } else if r < 0.93 {
        (LENTEJUELA, 0)
    } else {
        (PLUMA, rng.u8(0..PLUMAS.len() as u8))
    }
}

/// The wake a bit leaves: confetti its own color's, lentejuelas gold.
fn wake(slot: u8, b: &Bit) -> Option<u8> {
    match b.kind {
        CONFETTI => Some(slot + b.tone.min(7)),
        LENTEJUELA => Some(slot + 8),
        _ => None,
    }
}

impl Costume for Carnaval {
    /// Dresses the screen for the holiday (or undresses it): called on every
    /// resize and whenever the holiday changes.
    fn dress(&mut self, fiesta: Fiesta, mode: Mode, w: usize, h: usize, jets: usize) {
        self.fiesta = fiesta;
        self.mode = mode;
        self.bits.reset(0, 0);
        self.streamers.clear();
        self.streamer_target = 0;
        self.toss_acc = 0.0;
        self.stars.reset(false);
        if w == 0 || h == 0 {
            return;
        }
        let level = fiesta.level.clamp(0.0, 1.0);
        (self.pw, self.ph) = (w as f32 * 2.0, h as f32 * 4.0);
        self.jets = jets.max(1);
        // Counts per 100×30 cells, scaled by the screen area.
        let area = (w * h) as f32 / 3000.0;
        let count = |n: f32| ((n * area).round() as usize).min(4000);
        self.stars.reset(mode != Mode::Fireworks);
        match mode {
            Mode::Flow => {
                let target = count(30.0 + 90.0 * level);
                self.bits.reset(target, target);
                for _ in 0..target {
                    let (kind, tone) = pick(&mut self.rng);
                    let mut p = Bit::loose(&mut self.rng, (self.pw, self.ph), kind, tone);
                    p.age = self.rng.f32() * p.life;
                    self.bits.list.push(p);
                }
                self.streamer_target = count(1.5 + 5.0 * level).max(1);
                self.streamers.reserve_exact(self.streamer_target);
                for _ in 0..self.streamer_target {
                    let mut s = self.loose_streamer();
                    s.head.age = self.rng.f32() * s.head.life * 0.8;
                    self.streamers.push(s);
                }
            }
            Mode::Fountain => {
                self.bits.reset(0, count(40.0 + 80.0 * level).max(8));
                self.streamers
                    .reserve_exact(count(2.0 + 6.0 * level).max(2));
            }
            Mode::Fireworks => {
                self.bits.reset(0, count(80.0 + 140.0 * level).max(16));
                self.streamers
                    .reserve_exact(count(3.0 + 7.0 * level).max(3));
            }
        }
    }

    /// Caches the holiday's colors. Under other themes the trails and shells
    /// take the theme's colors; the confetti, lentejuelas and serpentinas
    /// keep their own, a little drawn toward it.
    fn retheme(&mut self, ctx: &Ctx, slot: usize) {
        self.slot = slot.min(240) as u8;
        let tint = |c: Color, i: usize| {
            if ctx.theme.native {
                c
            } else {
                c.lerp(ctx.pick(i, c), 0.2)
            }
        };
        for (i, c) in self.confetti_cols.iter_mut().enumerate() {
            *c = tint(BRIGHTS[i], i);
        }
        self.oro = tint(ORO, 8);
        self.blanco = tint(BLANCO, 9);
        // The river's colors make the bursts, and oro.
        for (i, c) in self.burst_cols.iter_mut().take(6).enumerate() {
            *c = Particles::grad_slot(i as f32 / 5.0);
        }
        self.burst_cols[6] = self.slot + 8;
    }

    /// Fireworks: some shells burst into confetti, lentejuelas and
    /// serpentinas as well.
    fn burst(&mut self, x: f32, y: f32, radius: f32) {
        if self.rng.f32() > 0.35 + 0.35 * self.fiesta.level {
            return;
        }
        let n = 10 + self.rng.usize(0..10) + (self.fiesta.level * 10.0) as usize;
        for _ in 0..n {
            if self.bits.full() {
                break;
            }
            let a = self.rng.f32() * TAU;
            let v = radius * (0.6 + 0.9 * self.rng.f32());
            let (kind, tone) = pick(&mut self.rng);
            let p = Bit::cast(
                &mut self.rng,
                (x, y),
                (a.cos() * v, a.sin() * v * 0.8),
                kind,
                tone,
                true,
            );
            self.bits.add(p);
        }
        for _ in 0..1 + self.rng.usize(0..3) {
            if self.streamers.len() >= self.streamers.capacity() {
                break;
            }
            let a = self.rng.f32() * TAU;
            let v = radius * (0.5 + 0.6 * self.rng.f32());
            let s = self.streamer((x, y), (a.cos() * v, a.sin() * v * 0.8), true);
            self.streamers.push(s);
        }
    }

    /// Only the bursts of confetti ride the flow outside the flow itself
    /// (what falls does not).
    fn flows(&self) -> bool {
        self.mode == Mode::Flow || self.stars.on()
    }

    fn update(&mut self, dt: f32, field: &Field, dots: &mut Dots, now: u32, span: u32, v: f32) {
        if self.pw <= 0.0 {
            return;
        }
        if self.mode == Mode::Fountain {
            self.toss(dt);
        }
        let wind = Wind {
            field,
            v,
            now,
            span,
        };
        if self.stars.on() {
            let burst = Burst {
                rate: 0.1 + 0.5 * self.fiesta.level * self.fiesta.level,
                cols: &self.burst_cols,
                hot: self.slot + 9,
            };
            self.stars.update(dt, &mut self.rng, &wind, dots, &burst);
        }
        let (dims, slot) = ((self.pw, self.ph), self.slot);
        let rng = &mut self.rng;
        self.bits.update(
            dt,
            &wind,
            dots,
            &PHYSICS,
            |b| wake(slot, b),
            || {
                let (kind, tone) = pick(rng);
                Bit::loose(rng, dims, kind, tone)
            },
        );
        self.update_streamers(dt, &wind, dots);
    }

    fn draw(&mut self, ctx: &Ctx, br: &mut Braille) {
        for p in &self.bits.list {
            let show = p.show();
            let (x, y) = (rnd(p.x), rnd(p.y));
            let mut pts = Pts::new();
            match p.kind {
                CONFETTI => {
                    let f = (turns(p.spin) * 4.0) as usize % 4;
                    pts.shape(x, y, TUMBLE[f]);
                    let c = ctx.fade(self.confetti_cols[p.tone as usize % 8], 1.0 - show);
                    pts.draw(br, c, show > 0.6);
                }
                LENTEJUELA => {
                    // Turning over, it catches the light now and then: a
                    // flash of white and a little star.
                    let flash = (p.spin * 0.9 + p.x * 0.37).sin().abs().powi(8);
                    if flash > 0.7 {
                        pts.shape(x, y, &SPARKLE);
                        let c = ctx.fade(self.oro.lerp(self.blanco, 0.8), 1.0 - show);
                        pts.draw(br, c, true);
                    } else {
                        let f = (turns(p.spin) * 4.0) as usize % 4;
                        pts.shape(x, y, TUMBLE[f]);
                        let c = ctx.fade(self.oro, 1.0 - show * (0.55 + 0.45 * flash));
                        pts.draw(br, c, show > 0.6);
                    }
                }
                _ => {
                    // A pluma turning over as it drifts.
                    let flip = p.spin.cos().abs();
                    pts.lens(p.x, p.y, p.spin * 0.3, 7.0, 3.0 * (0.4 + 0.6 * flip), 0.3);
                    let c = self.confetti_cols[PLUMAS[p.tone as usize % PLUMAS.len()]];
                    let c = c.lerp(self.blanco, 0.25 * flip);
                    pts.draw(br, ctx.fade(c, 1.0 - show), show > 0.6);
                }
            }
        }
        let amp = (self.ph * 0.03).clamp(4.0, 10.0);
        for s in &self.streamers {
            let show = s.head.show();
            let c = self.confetti_cols[s.head.tone as usize % 8];
            let shades = [c, c.lerp(self.blanco, 0.45)].map(|c| ctx.fade(c, 1.0 - show));
            serpentina(br, s, amp, shades, show > 0.6);
        }
    }
}

/// The points of a serpentina from its head to its tail: its path, curled
/// into waves and loops that open out as it ages and twist as it goes.
fn serpentina_path(s: &Streamer, amp: f32) -> [(f32, f32); NODES + 1] {
    let h = &s.head;
    let curl = (1.0 - h.age / h.life * 0.85).max(0.12);
    let twist = h.spin * 0.45;
    let at = |i: usize| if i == 0 { (h.x, h.y) } else { s.body[i - 1] };
    let mut path = [(0.0, 0.0); NODES + 1];
    for (i, node) in path.iter_mut().enumerate() {
        let (p, before, after) = (at(i), at(i.saturating_sub(1)), at((i + 1).min(NODES)));
        // Along the path here, and across it.
        let (tx, ty) = (before.0 - after.0, before.1 - after.1);
        let n = (tx * tx + ty * ty).sqrt().max(1e-3);
        let (tx, ty) = (tx / n, ty / n);
        // The tail swings wider than the head, and the paper twists.
        let w = amp * curl * (0.25 + 0.75 * i as f32 / NODES as f32);
        let (sn, cs) = (twist + i as f32 * 0.9).sin_cos();
        *node = (
            p.0 - ty * w * sn + tx * w * 0.5 * cs,
            p.1 + tx * w * sn + ty * w * 0.5 * cs,
        );
    }
    path
}

/// Draws a serpentina in stripes of two shades, in front of the trails if
/// `front`.
fn serpentina(br: &mut Braille, s: &Streamer, amp: f32, shades: [Color; 2], front: bool) {
    let path = serpentina_path(s, amp);
    let mut stripes = [Wire::new(), Wire::new()];
    for (i, w) in path.windows(2).enumerate() {
        // A ribbon two dots wide.
        let (dx, dy) = (w[1].0 - w[0].0, w[1].1 - w[0].1);
        let off = if dy.abs() > dx.abs() {
            (1.0, 0.0)
        } else {
            (0.0, 1.0)
        };
        let stripe = &mut stripes[(i / 2) % 2];
        stripe.line(w[0], w[1]);
        stripe.line(
            (w[0].0 + off.0, w[0].1 + off.1),
            (w[1].0 + off.0, w[1].1 + off.1),
        );
    }
    for (stripe, c) in stripes.iter().zip(shades) {
        stripe.draw(br, c, front);
    }
}

#[cfg(test)]
mod tests {
    use super::super::rig::{self, MODES};
    use super::*;
    use crate::fiesta::Kind;
    use crate::render::{Braille, Canvas};
    use crate::theme::Theme;

    /// What the costume paints that the river does not: the rojo and azul
    /// confetti and the lentejuelas.
    fn marks() -> Vec<Color> {
        vec![BRIGHTS[6], BRIGHTS[7], ORO]
    }

    /// Carnaval on the days of 2027 (Carnaval falls from February 4 to 9).
    fn day(d: u32) -> Fiesta {
        Fiesta::of_date(2027, 2, d)
    }

    #[test]
    fn dresses_up_on_its_days() {
        let theme = Theme::classic();
        for mode in MODES {
            for d in [4, 9] {
                let o = rig::opts(mode, "");
                let ctx = rig::ctx(&theme, &o, day(d));
                assert!(ctx.fiesta.is(Kind::Carnaval), "February {d}");
                let (mut s, mut c) = rig::scene(&ctx, 120, 36);
                assert!(s.costume.is_some(), "{mode} {d}");
                assert!(
                    rig::shows(&mut s, &ctx, &mut c, 900, &marks()),
                    "{mode} February {d}: nothing of the Carnaval on screen"
                );
            }
        }
    }

    /// Dressed on its own, the costume carries more with each day.
    #[test]
    fn builds_up_to_martes_de_carnaval() {
        let room = |n: &Carnaval| {
            n.bits.list.capacity() + n.bits.target + n.streamers.capacity() + n.streamer_target
        };
        for mode in [Mode::Flow, Mode::Fountain, Mode::Fireworks] {
            let mut last = 0;
            for d in [4, 6, 9] {
                let mut n = Carnaval::new(7);
                n.dress(day(d), mode, 120, 36, 2);
                assert!(
                    room(&n) > last,
                    "February {d}: {} is not more than {last}",
                    room(&n)
                );
                last = room(&n);
            }
        }
    }

    /// The serpentinas are drawn, and every one is a whole streamer of
    /// paper that lets go of its curl as it ages.
    #[test]
    fn serpentinas_curl_and_uncurl() {
        let theme = Theme::classic();
        let o = rig::opts("flow", "");
        let ctx = rig::ctx(&theme, &o, day(9));
        let (field, mut dots) = rig::flow(120, 36);
        let mut n = Carnaval::new(7);
        n.dress(day(9), Mode::Flow, 120, 36, 1);
        n.retheme(&ctx, 0);
        let wind_v = 13.0;
        for _ in 0..120 {
            n.update(1.0 / 30.0, &field, &mut dots, 1 << 24, 900, wind_v);
        }
        assert!(!n.streamers.is_empty());
        // Lay one out straight, and see how far its path strays from the line.
        let mut s = n.streamers[0];
        for (i, p) in s.body.iter_mut().enumerate() {
            *p = (50.0 - 3.0 * i as f32, 60.0);
        }
        s.head.x = 53.0;
        s.head.y = 60.0;
        s.head.life = 10.0;
        let stray = |age: f32| {
            let mut t = s;
            t.head.age = age;
            serpentina_path(&t, 5.0)
                .iter()
                .map(|p| (p.1 - 60.0).abs())
                .fold(0.0f32, f32::max)
        };
        assert!(
            stray(0.0) > 2.0 * stray(9.0),
            "{} {}",
            stray(0.0),
            stray(9.0)
        );
        assert!(stray(9.0) > 0.0);
        // And they are drawn: with nothing else about, a few cells light up.
        n.bits.list.clear();
        let mut br = Braille::new(120, 36);
        n.draw(&ctx, &mut br);
        let mut c = Canvas::new(120, 36, Color::Reset);
        br.blit(&mut c);
        let lit = c.cells().iter().filter(|x| x.ch != ' ').count();
        assert!(
            lit > 10 * n.streamers.len() / 2,
            "{lit} cells for {} serpentinas",
            n.streamers.len()
        );
    }

    #[test]
    fn ordinary_days_stay_ordinary() {
        rig::is_ordinary(Fiesta::NONE);
        for kind in [
            Kind::Candelaria,
            Kind::Equinoccio,
            Kind::Judas,
            Kind::Revolucion,
        ] {
            rig::is_ordinary(Fiesta::height(kind));
        }
    }

    #[test]
    fn comes_and_goes_with_retheme() {
        rig::comes_and_goes(day(9), &marks());
        rig::comes_and_goes(day(4), &marks());
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        for d in [4, 7, 9] {
            rig::survives(day(d));
        }
    }
}
