//! Navidad in the particles: the flow carries nochebuenas, escarcha and
//! colación down a river of rojo, verde and dorado.
//!
//! Nochebuena flowers (poinsettias, now and then a white one) open in the
//! current, drift with it and shed their red bracts into it, then let go of
//! them all at once and bloom again elsewhere. The bracts tumble along the
//! streams, each leaving a crimson wake in the trail buffer; strands of
//! escarcha (tinsel) glint silver and gold as they turn, flashing into
//! little stars; colación candies, small round sweets in pastel and bright
//! colors, drift along with the rest. On Año Nuevo bursts go off in the
//! stream. In the fireworks some shells burst into bracts, tinsel or candy
//! that flutter down, and the fountain springs from nochebuenas and tosses
//! them with its water. How much there is follows `fiesta.level`: a few on
//! the Virgen's day, more with every posada, the most on Nochebuena.
//!
//! Everything here draws from its own RNG, so an ordinary day's particles
//! are never disturbed.

use std::f32::consts::TAU;

use super::art::{
    Bit, Bits, Blossom, Burst, Cut, Life, Physics, Pts, Stars, Wind, atan2, draw_flower, fract, rnd,
};
use super::{Costume, Dots, Field, Mode, Particles};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::Braille;
use crate::scene::Ctx;

/// The flow (and fountain) ramp under `classic`: verde, dorado and rojo.
pub(super) const RAMP: [Color; 6] = [
    rgb(0x1f6f3a),
    rgb(0x2e9a55),
    rgb(0xf2b632),
    rgb(0xf2b632),
    rgb(0xc8102e),
    rgb(0xe0243a),
];

/// Shell colors under `classic`: the foquitos, the dorado and the plata.
pub(super) const SHELLS: [Color; 8] = [
    rgb(0xff3b3b),
    rgb(0x3bd16f),
    rgb(0xf2b632),
    rgb(0xc9ccd6),
    rgb(0xc8102e),
    rgb(0xf4f1ea),
    rgb(0xffd23b),
    rgb(0x1fa05a),
];

/// Wakes, appended to the trail palette: nochebuena, plata, dorado and the
/// white-hot heart of a burst.
pub(super) const WAKES: [Color; 4] = [rgb(0xe0243a), rgb(0xc9ccd6), rgb(0xf2b632), rgb(0xf4f1ea)];

/// Bract shades: seen face on, tilting, edge on.
const BRACTS: [Color; 3] = [rgb(0xff5a6c), rgb(0xff2d4a), rgb(0xd01632)];

/// Escarcha: plata, dorado and the white of its flash.
const TINSEL: [Color; 3] = [rgb(0xc9ccd6), rgb(0xf2b632), rgb(0xf4f1ea)];

/// Colación: pastel and bright sweets.
const CANDY: [Color; 8] = [
    rgb(0xffb3d1),
    rgb(0xf4f1ea),
    rgb(0xfff0a0),
    rgb(0xa8f0c8),
    rgb(0xc9b3ff),
    rgb(0x9fd8ff),
    rgb(0xff9a3c),
    rgb(0xff4fa3),
];

/// Nochebuena flowers, a bright heart, the inner bracts and the outer ones:
/// the red one, and the white one.
const FLOWERS: [[Color; 3]; 2] = [
    [rgb(0xf2b632), rgb(0xff5a6c), rgb(0xd01632)],
    [rgb(0xf2b632), rgb(0xf4f1ea), rgb(0xd9d2bd)],
];

/// What a bit is.
const BRACT: u8 = 0;
const GLINT: u8 = 1;
const CANDIED: u8 = 2;

/// How each kind of bit moves: bracts follow the flow, tinsel flutters in
/// it and the heavier sweets lag behind.
const PHYSICS: [Physics; 3] = [
    Physics {
        gain: 0.85,
        follow: 2.5,
        flutter: 0.0,
    },
    Physics {
        gain: 0.9,
        follow: 3.0,
        flutter: 7.0,
    },
    Physics {
        gain: 0.6,
        follow: 1.6,
        flutter: 0.0,
    },
];

/// A candy, round: two dots across, or a little plus.
const SWEETS: [&[(i32, i32)]; 2] = [
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
];

/// The flash of a strand of tinsel that catches the light: a star.
const SPARKLE: [(i32, i32); 7] = [(0, -2), (0, -1), (-1, 0), (0, 0), (1, 0), (0, 1), (0, 2)];

pub(super) struct Navidad {
    fiesta: Fiesta,
    rng: fastrand::Rng,
    mode: Mode,
    pw: f32,
    ph: f32,
    jets: usize,
    bits: Bits,
    flowers: Vec<Blossom>,
    /// How each flower was last cut (one per flower).
    cuts: Vec<Cut>,
    /// Año Nuevo: the bursts in the stream.
    stars: Stars,
    toss_acc: f32,
    /// First trail palette slot of the wakes.
    slot: u8,
    /// The slots a burst picks its colors from: the river's colors, silver and gold.
    burst_cols: [u8; 6],
    bract_cols: [Color; 3],
    tinsel_cols: [Color; 3],
    candy_cols: [Color; 8],
    flower_cols: [[Color; 3]; 2],
}

impl Navidad {
    pub fn new(seed: u64) -> Navidad {
        Navidad {
            fiesta: Fiesta::NONE,
            rng: fastrand::Rng::with_seed(seed ^ 0x4E41_5649_4441_4400),
            mode: Mode::Flow,
            pw: 0.0,
            ph: 0.0,
            jets: 1,
            bits: Bits::default(),
            flowers: Vec::new(),
            cuts: Vec::new(),
            stars: Stars::default(),
            toss_acc: 0.0,
            slot: 0,
            burst_cols: [0; 6],
            bract_cols: BRACTS,
            tinsel_cols: TINSEL,
            candy_cols: CANDY,
            flower_cols: FLOWERS,
        }
    }

    /// A new flower, a white one now and then.
    fn new_flower(&mut self) -> Blossom {
        Blossom::new(&mut self.rng, self.pw, self.ph, 0.3)
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
            let (kind, tone) = pick(&mut self.rng);
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

    fn update_flowers(&mut self, dt: f32, field: &Field, v: f32) {
        for i in 0..self.flowers.len() {
            let mut b = self.flowers[i];
            let life = b.grow(dt, field, v, (self.pw, self.ph));
            if matches!(life, Life::Over) {
                let mut nb = self.new_flower();
                nb.age = -0.5 - self.rng.f32() * 2.0;
                self.flowers[i] = nb;
                continue;
            }
            if matches!(life, Life::Scatter) {
                let n = (b.size * 2.2) as usize;
                for k in 0..n {
                    if self.bits.full() {
                        break;
                    }
                    let a = (k as f32 + self.rng.f32()) / n as f32 * TAU;
                    let (c, s) = (a.cos(), a.sin());
                    let r = b.size * (0.3 + 0.6 * self.rng.f32());
                    let sp = 6.0 + self.rng.f32() * 8.0;
                    let mut p = Bit::cast(
                        &mut self.rng,
                        (b.x + c * r, b.y + s * r),
                        (c * sp, s * sp),
                        BRACT,
                        0,
                        false,
                    );
                    p.life *= 0.6;
                    self.bits.add(p);
                }
            }
            self.flowers[i] = b;
        }
    }
}

/// The kind and color of a bit that rides the flow (bracts most often).
fn pick(rng: &mut fastrand::Rng) -> (u8, u8) {
    let r = rng.f32();
    if r < 0.4 {
        (BRACT, 0)
    } else if r < 0.7 {
        (GLINT, rng.bool() as u8)
    } else {
        (CANDIED, rng.u8(0..CANDY.len() as u8))
    }
}

/// A bit leaving a flower's rim if one is open and it is a bract, else
/// somewhere on the screen, fading in.
fn renew(rng: &mut fastrand::Rng, flowers: &[Blossom], dims: (f32, f32)) -> Bit {
    let (kind, tone) = pick(rng);
    if kind == BRACT && !flowers.is_empty() {
        let b = flowers[rng.usize(0..flowers.len())];
        if b.age > 1.0 && !b.scattered && b.age < b.life {
            let a = rng.f32() * TAU;
            let (c, s) = (a.cos(), a.sin());
            let r = b.size * 0.9;
            return Bit::cast(
                rng,
                (b.x + c * r, b.y + s * r),
                (c * 4.0, s * 4.0),
                kind,
                0,
                false,
            );
        }
    }
    Bit::loose(rng, dims, kind, tone)
}

/// The wake a bit leaves: bracts crimson, tinsel silver or gold, sweets none.
fn wake(slot: u8, b: &Bit) -> Option<u8> {
    match b.kind {
        BRACT => Some(slot),
        GLINT => Some(slot + 1 + b.tone.min(1)),
        _ => None,
    }
}

/// A nochebuena: pointed red bracts around a small golden heart, an inner
/// ring set between the outer ones.
fn cut_nochebuena(cut: &mut Cut, r: f32, (cx, cy): (i32, i32), rot: f32, (pw, ph): (i32, i32)) {
    const N: f32 = 7.0;
    // 0 in the middle of a bract, 1 at the notch between two.
    let off = |a: f32, turn: f32| (fract((a - turn) * (N / TAU)) - 0.5).abs() * 2.0;
    let veins = r >= 6.0;
    let lit = |x: i32, y: i32| {
        let (dx, dy) = ((x - cx) as f32, (y - cy) as f32);
        let d = (dx * dx + dy * dy).sqrt();
        if d > r + 0.5 {
            return None;
        }
        if d < r * 0.2 {
            return Some(true);
        }
        let a = atan2(dy, dx);
        let o = off(a, -rot);
        if d > r * 0.64 {
            return Some(d <= r * (1.0 - 0.8 * o) && !(veins && o < 0.08));
        }
        // The inner bracts lie over the outer ones, with a seam between.
        let i = off(a, -rot + 0.5 * TAU / N);
        let inner = r * 0.64 * (1.0 - 0.7 * i);
        if d <= inner {
            // A vein down the middle of each bract.
            return Some(!(veins && i < 0.1 && d > r * 0.34));
        }
        Some(d <= r * (1.0 - 0.8 * o) && d - inner > 1.0)
    };
    // The trails are cleared around it, so a red flower stands out from a
    // red river.
    cut.fill(r, (cx, cy), (pw, ph), ((0.22, 0.66), 2.2), lit);
}

impl Costume for Navidad {
    /// Dresses the screen for the holiday (or undresses it): called on every
    /// resize and whenever the holiday changes.
    fn dress(&mut self, fiesta: Fiesta, mode: Mode, w: usize, h: usize, jets: usize) {
        self.fiesta = fiesta;
        self.mode = mode;
        self.bits.reset(0, 0);
        self.flowers.clear();
        self.cuts.clear();
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
        self.stars
            .reset(fiesta.ano_nuevo() && mode != Mode::Fireworks);
        match mode {
            Mode::Flow => {
                let flowers = count(1.0 + 2.0 * level).max(1);
                self.flowers.reserve_exact(flowers);
                for i in 0..flowers {
                    let mut b = self.new_flower();
                    // The first ones are already open; the rest come one by one.
                    b.age = if i * 2 < flowers + 1 {
                        self.rng.f32() * b.life * 0.6
                    } else {
                        -self.rng.f32() * 6.0
                    };
                    self.flowers.push(b);
                }
                let target = count(24.0 + 66.0 * level);
                // Room for the bracts a flower lets go of all at once.
                self.bits.reset(target, target + count(24.0).max(12));
                for _ in 0..target {
                    let (kind, tone) = pick(&mut self.rng);
                    let mut p = Bit::loose(&mut self.rng, (self.pw, self.ph), kind, tone);
                    p.age = self.rng.f32() * p.life;
                    self.bits.list.push(p);
                }
            }
            Mode::Fountain => {
                // The jets spring from nochebuenas.
                self.flowers.reserve_exact(self.jets);
                for j in 0..self.jets {
                    let mut b = self.new_flower();
                    b.size = (self.ph * 0.07).clamp(3.0, 9.0);
                    b.x = self.pw * (j as f32 + 0.5) / self.jets as f32;
                    b.y = self.ph - b.size * 0.6;
                    b.age = 3.0;
                    b.life = f32::INFINITY;
                    b.kind = 0;
                    self.flowers.push(b);
                }
                self.bits.reset(0, count(30.0 + 70.0 * level).max(8));
            }
            Mode::Fireworks => {
                self.bits.reset(0, count(60.0 + 120.0 * level).max(16));
            }
        }
        self.cuts.resize_with(self.flowers.len(), Cut::roomy);
    }

    /// Caches the holiday's colors. Under other themes the trails and shells
    /// take the theme's colors; the bracts, tinsel, candy and flowers keep
    /// their own, a little drawn toward it.
    fn retheme(&mut self, ctx: &Ctx, slot: usize) {
        self.slot = slot.min(246) as u8;
        for (c, t) in self.burst_cols.iter_mut().zip([0.2, 0.5, 0.8, 1.0]) {
            *c = Particles::grad_slot(t);
        }
        self.burst_cols[4] = self.slot + 1;
        self.burst_cols[5] = self.slot + 2;
        let tint = |c: Color, i: usize| {
            if ctx.theme.native {
                c
            } else {
                c.lerp(ctx.pick(i, c), 0.2)
            }
        };
        for (i, c) in self.bract_cols.iter_mut().enumerate() {
            *c = tint(BRACTS[i], i);
        }
        for (i, c) in self.tinsel_cols.iter_mut().enumerate() {
            *c = tint(TINSEL[i], i);
        }
        for (i, c) in self.candy_cols.iter_mut().enumerate() {
            *c = tint(CANDY[i], i);
        }
        for (k, row) in self.flower_cols.iter_mut().enumerate() {
            for (s, c) in row.iter_mut().enumerate() {
                *c = tint(FLOWERS[k][s], k * 3 + s);
            }
        }
    }

    /// Fireworks: some shells burst into bracts, tinsel or candy as well.
    fn burst(&mut self, x: f32, y: f32, radius: f32) {
        let more = if self.fiesta.ano_nuevo() { 0.1 } else { 0.0 };
        if self.rng.f32() > 0.3 + 0.35 * self.fiesta.level + more {
            return;
        }
        let (kind, tone) = pick(&mut self.rng);
        let n = 6 + self.rng.usize(0..8) + (self.fiesta.level * 8.0) as usize;
        for _ in 0..n {
            if self.bits.full() {
                break;
            }
            let a = self.rng.f32() * TAU;
            let v = radius * (0.6 + 0.9 * self.rng.f32());
            // Candies are all of one color in a burst, tinsel of both.
            let tone = if kind == GLINT {
                self.rng.bool() as u8
            } else {
                tone
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

    /// Only the bursts of Año Nuevo ride the flow outside the flow itself.
    fn flows(&self) -> bool {
        self.mode == Mode::Flow || self.stars.on()
    }

    fn update(&mut self, dt: f32, field: &Field, dots: &mut Dots, now: u32, span: u32, v: f32) {
        if self.pw <= 0.0 {
            return;
        }
        match self.mode {
            Mode::Flow => self.update_flowers(dt, field, v),
            Mode::Fountain => self.toss(dt),
            Mode::Fireworks => {}
        }
        let wind = Wind {
            field,
            v,
            now,
            span,
        };
        if self.stars.on() {
            let burst = Burst {
                rate: 0.25 + 0.5 * self.fiesta.level,
                cols: &self.burst_cols,
                hot: self.slot + 3,
            };
            self.stars.update(dt, &mut self.rng, &wind, dots, &burst);
        }
        let (dims, slot) = ((self.pw, self.ph), self.slot);
        let (rng, flowers) = (&mut self.rng, &self.flowers);
        self.bits.update(
            dt,
            &wind,
            dots,
            &PHYSICS,
            |b| wake(slot, b),
            || renew(rng, flowers, dims),
        );
    }

    fn draw(&mut self, ctx: &Ctx, br: &mut Braille) {
        for (b, cut) in self.flowers.iter().zip(&mut self.cuts) {
            // It turns slowly, and seven bracts look alike from one to the next.
            let cols = &self.flower_cols[b.kind as usize];
            draw_flower(b, cut, cols, br, (cut_nochebuena, 12.0));
        }
        for p in &self.bits.list {
            let (x, y) = (rnd(p.x), rnd(p.y));
            let show = p.show();
            let mut pts = Pts::new();
            match p.kind {
                BRACT => {
                    // A pointed bract turning over as it tumbles.
                    let flip = p.spin.cos().abs();
                    pts.lens(p.x, p.y, p.spin * 0.35, 5.0, 3.0 * (0.4 + 0.6 * flip), 0.2);
                    let shade = if flip > 0.7 {
                        0
                    } else if flip > 0.35 {
                        1
                    } else {
                        2
                    };
                    let c = ctx.fade(self.bract_cols[shade], 1.0 - show);
                    pts.draw(br, c, show > 0.6);
                }
                GLINT => {
                    // Tinsel catches the light now and then: it flashes
                    // into a star, silver or gold.
                    let flash = (p.spin * 0.8 + p.tone as f32 * 2.0).sin().abs().powi(6);
                    let base = self.tinsel_cols[p.tone.min(1) as usize];
                    if flash > 0.8 {
                        pts.shape(x, y, &SPARKLE);
                        let c = ctx.fade(base.lerp(self.tinsel_cols[2], 0.7), 1.0 - show);
                        pts.draw(br, c, true);
                    } else {
                        pts.lens(p.x, p.y, p.spin * 0.5, 4.0, 0.0, 0.0);
                        let c = ctx.fade(base, 1.0 - show * (0.35 + 0.65 * flash));
                        pts.draw(br, c, false);
                    }
                }
                _ => {
                    pts.shape(x, y, SWEETS[(p.tone & 1) as usize]);
                    let c = ctx.fade(self.candy_cols[p.tone as usize % CANDY.len()], 1.0 - show);
                    pts.draw(br, c, show > 0.6);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::rig::{self, MODES};
    use super::*;
    use crate::fiesta::Kind;
    use crate::theme::Theme;

    /// What the costume paints that the river does not: bracts, candy and
    /// the red flower's bracts.
    fn marks() -> Vec<Color> {
        let candy = [0, 2, 3, 4, 5, 6, 7].map(|i| CANDY[i]);
        let mut m = BRACTS.to_vec();
        m.extend(candy);
        m.extend(&FLOWERS[0][1..]);
        m
    }

    #[test]
    fn dresses_up_on_its_days() {
        let theme = Theme::classic();
        for mode in MODES {
            for day in [(12, 12), (12, 24), (12, 31), (1, 6)] {
                let o = rig::opts(mode, "");
                let ctx = rig::ctx(&theme, &o, Fiesta::of_day(day.0, day.1));
                let (mut s, mut c) = rig::scene(&ctx, 120, 36);
                assert!(s.costume.is_some(), "{mode} {day:?}");
                assert!(
                    rig::shows(&mut s, &ctx, &mut c, 600, &marks()),
                    "{mode} {day:?}: nothing of the Navidad on screen"
                );
            }
        }
    }

    /// Dressed on its own, the costume carries more as the posadas climb.
    #[test]
    fn builds_up_with_the_posadas() {
        let room = |n: &Navidad| n.bits.list.capacity() + n.flowers.len() + n.bits.target;
        for mode in [Mode::Flow, Mode::Fountain, Mode::Fireworks] {
            let mut last = 0;
            for day in [(12, 12), (12, 18), (12, 24)] {
                let mut n = Navidad::new(7);
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

    /// Año Nuevo sends bursts through the stream, and only then.
    #[test]
    fn bursts_go_off_in_the_stream_on_ano_nuevo() {
        let theme = Theme::classic();
        let hot = |day: (u32, u32), mode: &str| {
            let o = rig::opts(mode, "");
            let ctx = rig::ctx(&theme, &o, Fiesta::of_day(day.0, day.1));
            let (mut s, mut c) = rig::scene(&ctx, 100, 30);
            // The white-hot heart of a burst is the last color of the palette.
            let last = (s.lut.len() / super::super::FADE - 1) as u8;
            let mut lit = false;
            for _ in 0..40 {
                rig::run(&mut s, &ctx, &mut c, 15);
                let d = &s.dots;
                lit |= (0..d.col.len())
                    .any(|i| d.col[i] == last && s.now.wrapping_sub(d.stamp[i]) < s.max_age);
            }
            lit
        };
        for mode in ["flow", "fountain"] {
            assert!(hot((12, 31), mode) && hot((1, 1), mode), "{mode}");
            assert!(!hot((12, 24), mode) && !hot((1, 6), mode), "{mode}");
        }
    }

    #[test]
    fn ordinary_days_stay_ordinary() {
        rig::is_ordinary(Fiesta::NONE);
        // The holidays the scene does not dress for are ordinary days too.
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
        rig::comes_and_goes(Fiesta::of_day(12, 24), &marks());
        rig::comes_and_goes(Fiesta::of_day(12, 31), &marks());
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        for day in [(12, 12), (12, 24), (12, 31), (1, 6)] {
            rig::survives(Fiesta::of_day(day.0, day.1));
        }
    }

    /// Cutting a nochebuena never outgrows the room set aside for it.
    #[test]
    fn flowers_are_cut_without_allocating() {
        let mut cut = Cut::roomy();
        for i in 0..170 {
            let r = 1.0 + i as f32 * 0.1;
            for (cx, cy) in [(0, 0), (37, 21), (150, 90), (-3, 5)] {
                cut_nochebuena(&mut cut, r, (cx, cy), i as f32 * 0.37, (300, 180));
                assert_eq!(cut.cells.capacity(), 224, "r = {r}");
            }
        }
    }
}
