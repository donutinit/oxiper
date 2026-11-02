//! Fiestas Patrias in the particles: a river in verde, blanco y rojo, with
//! tricolor confetti and banderitas tumbling in it.
//!
//! The flow runs in three bands like the flag, the green on one side, the
//! white in the middle and the red on the other, and their edges ripple as
//! if the wind were in the cloth (`band`). Tricolor confetti tumbles along
//! the streams, each piece leaving a green, white or red wake; banderitas
//! (little flags on their sticks) float in the river, waving. In the
//! fireworks shells burst in verde, blanco y rojo, now and then all three at
//! once, and some burst into confetti and banderitas that flutter down; the
//! fountain tosses them with its water. Toward the Grito bursts go off in the
//! stream too. How much there is follows `fiesta.level`: a few banderitas and
//! a trickle of confetti in early September, the most on the night of the
//! 15th.
//!
//! Everything here draws from its own RNG, so an ordinary day's particles
//! are never disturbed.

use std::f32::consts::TAU;

use super::art::{Bit, Bits, Burst, Physics, Pts, Stars, Wind, rnd, turns};
use super::{Costume, Dots, Field, Mode, Particles};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::Braille;
use crate::scene::Ctx;

/// The flag: verde bandera, blanco and rojo, and the oro of the sticks the
/// banderitas fly from (the bell, the embroidery).
const VERDE: Color = rgb(0x006847);
const BLANCO: Color = rgb(0xf4f1ea);
const ROJO: Color = rgb(0xce1126);
const ORO: Color = rgb(0xd4a017);

/// The flow (and fountain) ramp under `classic`: the flag's three bands.
pub(super) const RAMP: [Color; 3] = [VERDE, BLANCO, ROJO];

/// Shell colors under `classic`: the flag's (a livelier green, which shows up
/// better as sparks), and the oro.
pub(super) const SHELLS: [Color; 8] = [
    rgb(0x00a15f),
    BLANCO,
    rgb(0xe0152e),
    ORO,
    rgb(0x00a15f),
    BLANCO,
    rgb(0xe0152e),
    ORO,
];

/// Wakes, appended to the trail palette: verde, blanco, rojo and oro.
pub(super) const WAKES: [Color; 4] = [VERDE, BLANCO, ROJO, ORO];

/// What a bit is.
const CONFETTI: u8 = 0;
const BANDERITA: u8 = 1;

/// How each kind of bit moves: confetti flutters about in the current, a
/// banderita floats along with it slowly.
const PHYSICS: [Physics; 2] = [
    Physics {
        gain: 0.9,
        follow: 3.0,
        flutter: 4.0,
    },
    Physics {
        gain: 0.45,
        follow: 1.2,
        flutter: 0.0,
    },
];

/// A piece of confetti as it tumbles: face on, tilting, edge on, tilting
/// back (dot offsets).
const TUMBLE: [&[(i32, i32)]; 4] = [
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
    &[(0, 0), (1, 0), (1, 1)],
    &[(0, 0), (1, 0)],
    &[(1, 0), (0, 1), (1, 1)],
];

/// Where along the flag's ramp (0 verde, a half blanco, 1 rojo) the river
/// is at dot (x, y) of a screen `pw`×`ph` dots big at time `t`: three bands
/// side by side, their edges rippling.
pub(super) fn band(x: f32, y: f32, (pw, ph): (f32, f32), t: f32) -> f32 {
    let u = x / pw.max(1.0);
    let wave = 0.035 * (y / ph.max(1.0) * 7.0 - t * 1.1 + u * 4.0).sin();
    let b = ((u + wave) * 3.0).clamp(0.0, 2.999);
    // Mostly the band's own color, blending into the next near the edge.
    let (k, f) = (b.floor(), b.fract());
    let blend = if k < 2.0 {
        ((f - 0.8) * 5.0).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (k + blend) * 0.5
}

pub(super) struct Patrias {
    fiesta: Fiesta,
    rng: fastrand::Rng,
    mode: Mode,
    pw: f32,
    ph: f32,
    jets: usize,
    bits: Bits,
    /// Toward the Grito: the bursts in the stream.
    stars: Stars,
    toss_acc: f32,
    /// First trail palette slot of the wakes.
    slot: u8,
    /// The slots a burst picks its colors from: the three bands and oro.
    burst_cols: [u8; 4],
    confetti_cols: [Color; 3],
    pole: Color,
}

impl Patrias {
    pub fn new(seed: u64) -> Patrias {
        Patrias {
            fiesta: Fiesta::NONE,
            rng: fastrand::Rng::with_seed(seed ^ 0x5041_5452_4941_5300),
            mode: Mode::Flow,
            pw: 0.0,
            ph: 0.0,
            jets: 1,
            bits: Bits::default(),
            stars: Stars::default(),
            toss_acc: 0.0,
            slot: 0,
            burst_cols: [0; 4],
            confetti_cols: RAMP,
            pole: ORO,
        }
    }

    /// Fountain: the jets toss bits with the water.
    fn toss(&mut self, dt: f32) {
        let level = self.fiesta.level;
        self.toss_acc += dt * (1.5 + 4.0 * level) * self.jets as f32;
        while self.toss_acc >= 1.0 {
            self.toss_acc -= 1.0;
            if self.bits.full() {
                continue;
            }
            let jet = self.rng.usize(0..self.jets);
            let x = self.pw * (jet as f32 + 0.5) / self.jets as f32;
            let v = (2.0 * 1.1 * self.ph * self.ph * (0.5 + 0.3 * self.rng.f32())).sqrt();
            let a = (self.rng.f32() - 0.5) * 0.7;
            let (kind, tone) = pick(&mut self.rng, level);
            let b = Bit::cast(
                &mut self.rng,
                (x, self.ph - 2.0),
                (a.sin() * v, -a.cos() * v),
                kind,
                tone,
                true,
            );
            self.bits.add(b);
        }
    }
}

/// The kind and color of a bit that rides the flow: confetti mostly, and
/// more banderitas as the month goes.
fn pick(rng: &mut fastrand::Rng, level: f32) -> (u8, u8) {
    let kind = if rng.f32() < 0.03 + 0.1 * level {
        BANDERITA
    } else {
        CONFETTI
    };
    (kind, rng.u8(0..3))
}

/// The wake a bit leaves: confetti its own color's.
fn wake(slot: u8, b: &Bit) -> Option<u8> {
    (b.kind == CONFETTI).then(|| slot + b.tone.min(2))
}

impl Costume for Patrias {
    /// Dresses the screen for the holiday (or undresses it): called on every
    /// resize and whenever the holiday changes.
    fn dress(&mut self, fiesta: Fiesta, mode: Mode, w: usize, h: usize, jets: usize) {
        self.fiesta = fiesta;
        self.mode = mode;
        self.bits.reset(0, 0);
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
        self.stars.reset(level > 0.5 && mode != Mode::Fireworks);
        match mode {
            Mode::Flow => {
                let target = count(24.0 + 66.0 * level);
                self.bits.reset(target, target);
                for _ in 0..target {
                    let (kind, tone) = pick(&mut self.rng, level);
                    let mut p = Bit::loose(&mut self.rng, (self.pw, self.ph), kind, tone);
                    p.age = self.rng.f32() * p.life;
                    self.bits.list.push(p);
                }
            }
            Mode::Fountain => self.bits.reset(0, count(30.0 + 70.0 * level).max(8)),
            Mode::Fireworks => self.bits.reset(0, count(60.0 + 120.0 * level).max(16)),
        }
    }

    /// Caches the holiday's colors. Under other themes the trails and shells
    /// take the theme's colors; the confetti and banderitas keep their own, a
    /// little drawn toward it.
    fn retheme(&mut self, ctx: &Ctx, slot: usize) {
        self.slot = slot.min(246) as u8;
        let tint = |c: Color, i: usize| {
            if ctx.theme.native {
                c
            } else {
                c.lerp(ctx.pick(i, c), 0.2)
            }
        };
        for (i, c) in self.confetti_cols.iter_mut().enumerate() {
            *c = tint(RAMP[i], i);
        }
        self.pole = tint(ORO, 3);
        // Verde, blanco and rojo are the ramp's ends and middle; oro is a wake.
        for (i, c) in self.burst_cols.iter_mut().enumerate() {
            *c = if i < 3 {
                Particles::grad_slot(i as f32 / 2.0)
            } else {
                self.slot + 3
            };
        }
    }

    /// Fireworks: some shells burst into confetti and banderitas as well.
    fn burst(&mut self, x: f32, y: f32, radius: f32) {
        if self.rng.f32() > 0.3 + 0.35 * self.fiesta.level {
            return;
        }
        let n = 8 + self.rng.usize(0..8) + (self.fiesta.level * 10.0) as usize;
        for _ in 0..n {
            if self.bits.full() {
                break;
            }
            let a = self.rng.f32() * TAU;
            let v = radius * (0.6 + 0.9 * self.rng.f32());
            let tone = self.rng.u8(0..3);
            // A banderita now and then falls like a parachute.
            let kind = if self.rng.f32() < 0.08 + 0.1 * self.fiesta.level {
                BANDERITA
            } else {
                CONFETTI
            };
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
    }

    /// Now and then a shell bursts in all three colors at once, more often
    /// toward the Grito.
    fn tricolor(&mut self) -> bool {
        self.rng.f32() < 0.2 + 0.4 * self.fiesta.level
    }

    fn bands(&self) -> bool {
        true
    }

    /// Only the bursts toward the Grito ride the flow outside the flow itself.
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
            // None until the second half of the month, then more and more.
            let level = ((self.fiesta.level - 0.5) * 2.0).max(0.0);
            let burst = Burst {
                rate: 0.5 * level * level,
                cols: &self.burst_cols,
                hot: self.slot + 1,
            };
            self.stars.update(dt, &mut self.rng, &wind, dots, &burst);
        }
        let (dims, slot, level) = ((self.pw, self.ph), self.slot, self.fiesta.level);
        let rng = &mut self.rng;
        self.bits.update(
            dt,
            &wind,
            dots,
            &PHYSICS,
            |b| wake(slot, b),
            || {
                let (kind, tone) = pick(rng, level);
                Bit::loose(rng, dims, kind, tone)
            },
        );
    }

    fn draw(&mut self, ctx: &Ctx, br: &mut Braille) {
        // Banderitas twice the size on tall screens.
        let k = if br.ph() >= 200 { 2 } else { 1 };
        for p in &self.bits.list {
            let show = p.show();
            let c = ctx.fade(self.confetti_cols[p.tone as usize % 3], 1.0 - show);
            if p.kind == CONFETTI {
                let f = (turns(p.spin) * 4.0) as usize % 4;
                let mut pts = Pts::new();
                pts.shape(rnd(p.x), rnd(p.y), TUMBLE[f]);
                pts.draw(br, c, show > 0.6);
            } else {
                let cloth = self.confetti_cols.map(|c| ctx.fade(c, 1.0 - show));
                let pole = ctx.fade(self.pole, 1.0 - show);
                banderita(br, p, (cloth, pole), k, show > 0.6);
            }
        }
    }
}

/// A banderita: a stick and three bands of cloth, verde, blanco and rojo,
/// each a column of whole cells so every cell has one color, the cloth
/// waving as it flies.
fn banderita(br: &mut Braille, p: &Bit, (cloth, pole): ([Color; 3], Color), k: i32, front: bool) {
    // The cloth starts on a cell's edge, the stick in the cell before it.
    let (x0, y0) = (rnd(p.x) & !1, rnd(p.y));
    let mut stick = Pts::new();
    for j in 0..6 * k {
        stick.add(x0 - 1, y0 + j);
    }
    stick.draw(br, pole, front);
    for (i, &c) in cloth.iter().enumerate() {
        let wave = (p.spin * 0.7 + i as f32 * 1.3).sin() * k as f32;
        let mut band = Pts::new();
        for dx in 0..2 * k {
            for dy in 0..4 * k {
                band.add(x0 + i as i32 * 2 * k + dx, y0 + rnd(wave) + dy);
            }
        }
        band.draw(br, c, front);
    }
}

#[cfg(test)]
mod tests {
    use super::super::rig::{self, MODES};
    use super::*;
    use crate::fiesta::Kind;
    use crate::theme::Theme;

    /// What the costume paints that the river does not: the banderitas'
    /// sticks (the confetti is in the river's own colors).
    fn marks() -> Vec<Color> {
        vec![ORO]
    }

    #[test]
    fn dresses_up_on_its_days() {
        let theme = Theme::classic();
        for mode in MODES {
            for day in [(9, 1), (9, 15), (9, 16)] {
                let o = rig::opts(mode, "");
                let ctx = rig::ctx(&theme, &o, Fiesta::of_day(day.0, day.1));
                let (mut s, mut c) = rig::scene(&ctx, 120, 36);
                assert!(s.costume.is_some(), "{mode} {day:?}");
                // Banderitas fly in the flow and fall from the shells and the jets.
                assert!(
                    rig::shows(&mut s, &ctx, &mut c, 900, &marks()),
                    "{mode} {day:?}: no banderita on screen"
                );
            }
        }
    }

    /// Dressed on its own, the costume carries more as the month goes.
    #[test]
    fn builds_up_toward_the_grito() {
        let room = |n: &Patrias| n.bits.list.capacity() + n.bits.target;
        for mode in [Mode::Flow, Mode::Fountain, Mode::Fireworks] {
            let mut last = 0;
            for day in [(9, 1), (9, 8), (9, 15)] {
                let mut n = Patrias::new(7);
                n.dress(Fiesta::of_day(day.0, day.1), mode, 120, 36, 2);
                assert!(
                    room(&n) > last,
                    "{day:?}: {} is not more than {last}",
                    room(&n)
                );
                last = room(&n);
            }
        }
    }

    #[test]
    fn the_flag_has_three_bands() {
        let (pw, ph) = (400.0, 240.0);
        let at = |x: f32| band(x, ph / 2.0, (pw, ph), 3.0);
        // Verde, blanco and rojo, with their edges a third of the way across.
        assert!(at(0.08 * pw) < 0.02, "{}", at(0.08 * pw));
        assert!((at(0.5 * pw) - 0.5).abs() < 0.02, "{}", at(0.5 * pw));
        assert!(at(0.92 * pw) > 0.98, "{}", at(0.92 * pw));
        for i in 0..1000 {
            let t = band(
                i as f32 * 0.4,
                (i * 7 % 240) as f32,
                (pw, ph),
                i as f32 * 0.05,
            );
            assert!((0.0..=1.0).contains(&t), "{t}");
        }
    }

    /// The river is green on one side, white in the middle and red on the
    /// other.
    #[test]
    fn the_river_runs_in_bands() {
        let theme = Theme::classic();
        let o = rig::opts("flow", "");
        let ctx = rig::ctx(&theme, &o, Fiesta::of_day(9, 15));
        let (mut s, mut c) = rig::scene(&ctx, 120, 36);
        rig::run(&mut s, &ctx, &mut c, 240);
        // Which of the three a cell is drawn in, by its strongest channel.
        let kinds = |x0: i32, x1: i32| {
            let mut n = [0usize; 3];
            for y in 0..36 {
                for x in x0..x1 {
                    if let Color::Rgb(r, g, b) = c.get(x, y).unwrap().fg {
                        let (r, g, b) = (r as i32, g as i32, b as i32);
                        if g > r + 20 && g > b {
                            n[0] += 1;
                        } else if r > g + 40 && r > b + 40 {
                            n[2] += 1;
                        } else if r.min(g).min(b) > 60 {
                            n[1] += 1;
                        }
                    }
                }
            }
            n
        };
        let (left, mid, right) = (kinds(0, 30), kinds(45, 75), kinds(90, 120));
        assert!(left[0] > 3 * left[2] && left[0] > 20, "{left:?}");
        assert!(
            mid[1] > 3 * mid[0] && mid[1] > 3 * mid[2] && mid[1] > 20,
            "{mid:?}"
        );
        assert!(right[2] > 3 * right[0] && right[2] > 20, "{right:?}");
    }

    /// Bursts go off in the stream only toward the Grito, and more of the
    /// shells burst in three colors then.
    #[test]
    fn the_grito_brings_bursts_and_tricolor_shells() {
        let theme = Theme::classic();
        let o = rig::opts("flow", "");
        let ctx = rig::ctx(&theme, &o, Fiesta::NONE);
        let mut most = Vec::new();
        for day in [(9, 1), (9, 8), (9, 15)] {
            let (field, mut dots) = rig::flow(100, 30);
            let mut n = Patrias::new(7);
            n.dress(Fiesta::of_day(day.0, day.1), Mode::Flow, 100, 30, 1);
            n.retheme(&ctx, 0);
            let mut sparks = 0;
            for _ in 0..900 {
                n.update(1.0 / 30.0, &field, &mut dots, 1 << 24, 900, 13.0);
                sparks = sparks.max(n.stars.sparks());
            }
            let tricolor = (0..1000).filter(|_| n.tricolor()).count();
            most.push((sparks, tricolor));
        }
        assert_eq!(most[0].0 + most[1].0, 0, "{most:?}");
        assert!(most[2].0 > 0, "{most:?}");
        assert!(most[0].1 < most[1].1 && most[1].1 < most[2].1, "{most:?}");
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
        rig::comes_and_goes(Fiesta::of_day(9, 15), &marks());
        rig::comes_and_goes(Fiesta::of_day(9, 5), &marks());
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        for day in [(9, 1), (9, 15), (9, 16)] {
            rig::survives(Fiesta::of_day(day.0, day.1));
        }
    }
}
