//! The saucer. It comes rarely (the `ufo` option sets how often), hangs over
//! the tiger while the jungle falls silent and every animal freezes, lifts
//! the struggling tiger up its beam and zips off with it. A while later it
//! comes back and lowers the tiger to the ground, where it lands on its feet
//! and roars, none the worse for it.

use super::art;
use super::hunter::Mood;
use super::jungle::{FEET, Jungle, View};
use super::muertos::SAUCER;
use super::{Dice, Rgb, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Phase {
    /// Nowhere to be seen.
    #[default]
    Gone,
    /// Coming down over the tiger.
    Come,
    /// Lifting it up the beam.
    Beam,
    /// Off with it.
    Leave,
    /// Away, tiger and all.
    Away,
    /// Coming back.
    Return,
    /// Letting it down.
    Lower,
    /// Off again, empty.
    Go,
}

/// The saucer's size.
const W: f32 = 27.0;
const H: f32 = 10.0;
/// The row its top hovers at.
const HOVER: f32 = 36.0;

#[derive(Default)]
pub struct Ufo {
    pub phase: Phase,
    /// Its middle and its top, in local world pixels.
    pub x: f32,
    pub y: f32,
    vx: f32,
    vy: f32,
    steps: u32,
    /// Real seconds left away.
    clock: f32,
    /// A visit is due, as soon as the tiger is standing on solid ground.
    pub due: bool,
    /// How strong the beam is, and the eerie light about.
    beam: f32,
    glow: f32,
    /// How many times it has come, and brought the tiger back.
    pub visits: u32,
    pub returns: u32,
}

impl Ufo {
    pub fn shift(&mut self, dx: f32) {
        self.x -= dx;
    }

    /// Every animal holds still and silent while it hangs over the jungle.
    pub fn hushed(&self) -> bool {
        matches!(
            self.phase,
            Phase::Come | Phase::Beam | Phase::Return | Phase::Lower
        ) || (self.phase == Phase::Leave && self.steps < 30)
    }

    /// The camera pulls back to take it in.
    pub fn wide(&self) -> bool {
        !matches!(self.phase, Phase::Gone | Phase::Away)
    }

    /// Its top, while the camera should keep it in view.
    pub fn top(&self) -> Option<f32> {
        matches!(
            self.phase,
            Phase::Come | Phase::Beam | Phase::Return | Phase::Lower
        )
        .then_some(self.y.max(HOVER))
    }

    /// How much its light washes over the jungle, 0 to 1.
    pub fn glow(&self) -> f32 {
        self.glow
    }
}

impl Jungle {
    /// Brings the saucer as soon as it can come: for the tests.
    #[cfg(test)]
    pub fn summon(&mut self) {
        self.ufo.due = true;
    }

    /// The saucer's clocks: whether it is due, and how long it stays away.
    pub fn ufo_clocks(&mut self, real: f32, rng: &mut Rng) {
        let rate = self.set.ufo;
        if self.ufo.phase == Phase::Gone && rate > 0.0 && !self.ufo.due {
            let p = 1.0 - (-rate * real / 3600.0).exp();
            if rng.f32() < p {
                self.ufo.due = true;
            }
        }
        if self.ufo.phase == Phase::Away {
            self.ufo.clock -= real;
        }
    }

    fn ufo_phase(&mut self, p: Phase) {
        self.ufo.phase = p;
        self.ufo.steps = 0;
    }

    pub fn update_ufo(&mut self, rng: &mut Rng) {
        self.ufo.steps += 1;
        let steps = self.ufo.steps;
        let glow = if matches!(self.ufo.phase, Phase::Gone | Phase::Away) {
            0.0
        } else {
            1.0
        };
        self.ufo.glow += (glow - self.ufo.glow) * 0.03;
        if self.ufo.glow < 0.01 {
            self.ufo.glow = 0.0;
        }
        match self.ufo.phase {
            Phase::Gone => {
                if self.ufo.due && self.tiger.ready_for_ufo() {
                    self.ufo.due = false;
                    self.ufo.visits += 1;
                    let (x, y) = (self.tiger.x, -H - 6.0);
                    let u = &mut self.ufo;
                    (u.x, u.y, u.vx, u.vy, u.beam) = (x, y, 0.0, 0.0, 0.0);
                    self.ufo_phase(Phase::Come);
                    let t = &mut self.tiger;
                    t.prey = None;
                    t.rival = None;
                    t.set(Mood::Look);
                }
            }
            Phase::Come => {
                let tx = self.tiger.x;
                let u = &mut self.ufo;
                u.y += ((HOVER - u.y) * 0.04).max(0.08);
                u.x += (tx - u.x) * 0.05;
                if u.y >= HOVER - 0.3 {
                    u.y = HOVER;
                    if steps > 60 {
                        self.ufo_phase(Phase::Beam);
                    }
                }
            }
            Phase::Beam => {
                let u = &mut self.ufo;
                u.beam = (u.beam + 0.05).min(1.0);
                u.y = HOVER + 0.8 * (steps as f32 * 0.08).sin();
                if steps == 24 {
                    self.tiger.set(Mood::Carried);
                    self.tiger.speed = 0.0;
                }
                if steps > 24 {
                    let bottom = self.ufo.y + H;
                    let ux = self.ufo.x;
                    let t = &mut self.tiger;
                    t.lift += 0.3;
                    t.spin += 0.012;
                    t.x += (ux - t.x) * 0.05;
                    // Pulled in, feet last.
                    if FEET as f32 - t.lift - 8.0 <= bottom {
                        t.gone = true;
                        self.ufo_phase(Phase::Leave);
                    }
                }
            }
            Phase::Leave => {
                let u = &mut self.ufo;
                u.beam = (u.beam - 0.08).max(0.0);
                if steps > 20 {
                    if u.vx == 0.0 {
                        u.vx = rng.sign() * 0.2;
                    }
                    u.vy -= 0.06;
                    u.vx *= 1.08;
                    u.x += u.vx;
                    u.y += u.vy;
                }
                if u.y < -40.0 || steps > 30 * 8 {
                    self.ufo.clock = rng.rangef(18.0, 40.0);
                    self.ufo_phase(Phase::Away);
                }
            }
            Phase::Away => {
                if self.ufo.clock <= 0.0 {
                    // Back where the camera is, from one side.
                    let (vw, _) = self.view_size();
                    let v = self.view();
                    let cx = v.x + vw / 2.0;
                    let side = rng.sign();
                    // Somewhere dry to put it down.
                    let mut land = cx;
                    while let Some(s) = self.water(land) {
                        land = s.b + 30.0;
                    }
                    let u = &mut self.ufo;
                    (u.x, u.y, u.vx, u.vy) = (cx + side * vw * 0.8, -H - 4.0, 0.0, 0.0);
                    self.tiger.x = land;
                    self.ufo_phase(Phase::Return);
                }
            }
            Phase::Return => {
                let tx = self.tiger.x;
                let u = &mut self.ufo;
                u.x += (tx - u.x) * 0.06;
                u.y += (HOVER - u.y) * 0.06;
                if (u.x - tx).abs() < 1.0 && (u.y - HOVER).abs() < 0.6 {
                    u.y = HOVER;
                    self.ufo_phase(Phase::Lower);
                    let bottom = HOVER + H;
                    let t = &mut self.tiger;
                    t.gone = false;
                    t.lift = FEET as f32 - 8.0 - bottom;
                    t.sink = 0.0;
                    t.set(Mood::Carried);
                }
            }
            Phase::Lower => {
                let u = &mut self.ufo;
                u.beam = (u.beam + 0.08).min(1.0);
                let t = &mut self.tiger;
                if u.beam >= 1.0 {
                    t.lift -= 0.38;
                    t.spin += 0.012;
                }
                if t.lift <= 0.0 {
                    // On its feet, and furious.
                    t.lift = 0.0;
                    t.spin = 0.0;
                    t.set(Mood::Roar);
                    let x = t.x;
                    self.life.dust(x, FEET as f32, 12, rng);
                    self.ufo.returns += 1;
                    self.ufo_phase(Phase::Go);
                }
            }
            Phase::Go => {
                let u = &mut self.ufo;
                u.beam = (u.beam - 0.08).max(0.0);
                if steps > 40 {
                    if u.vx == 0.0 {
                        u.vx = rng.sign() * 0.3;
                    }
                    u.vy -= 0.07;
                    u.vx *= 1.1;
                    u.x += u.vx;
                    u.y += u.vy;
                }
                if u.y < -40.0 || steps > 30 * 8 {
                    self.ufo_phase(Phase::Gone);
                }
            }
        }
    }

    /// The beam, then the saucer itself, in its own light.
    pub fn draw_ufo(&mut self) {
        if matches!(self.ufo.phase, Phase::Gone | Phase::Away) {
            return;
        }
        let (x, y, beam) = (self.ufo.x, self.ufo.y, self.ufo.beam);
        // On Día de Muertos the beam is the amber of cempasúchil.
        let beam_color = self
            .pal
            .fest
            .map_or(Rgb(190, 255, 210), |d| d.marigold[2].lerp(d.hueso, 0.3));
        if beam > 0.0 {
            let top = (y + H) as i32;
            let tick = self.tick;
            for yy in top..FEET + 1 {
                let k = (yy - top) as f32 / (FEET + 1 - top).max(1) as f32;
                let hw = 4.0 + 12.0 * k;
                let band = if (yy as u32 + tick / 2).is_multiple_of(5) {
                    0.18
                } else {
                    0.0
                };
                for xx in (x - hw).round() as i32..=(x + hw).round() as i32 {
                    let edge = 1.0 - ((xx as f32 - x).abs() / hw).powi(4);
                    self.blend(xx, yy, beam_color, (0.28 + band) * beam * edge);
                }
            }
            self.beam_petals(x, top, beam);
        }
        let mut lut = [None; 128];
        let frame = ((self.tick / 8) % 2) as usize;
        match self.pal.fest {
            Some(d) => {
                #[cfg(test)]
                {
                    self.painted.saucer = true;
                }
                for &(ch, c) in &d.saucer() {
                    lut[ch as usize] = Some(c);
                }
                self.sprite(SAUCER[frame], x, y + H - 1.0, 1.0, &lut);
            }
            None => {
                for &(ch, c) in art::UFO_COLORS {
                    lut[ch as usize] = Some(c);
                }
                self.sprite(art::UFO[frame], x, y + H - 1.0, 1.0, &lut);
            }
        }
    }

    /// The rim lights' glow, straight onto the screen.
    pub fn draw_ufo_lights(&mut self, v: View) {
        if matches!(self.ufo.phase, Phase::Gone | Phase::Away) {
            return;
        }
        // The rim lights are a row lower on the skull's saucer, and marigold.
        let (rim, warm, hot) = match self.pal.fest {
            Some(d) => (6.0, d.marigold[2], d.marigold[1]),
            None => (5.0, Rgb(255, 240, 140), Rgb(255, 120, 120)),
        };
        let (x, y) = (self.ufo.x, self.ufo.y + rim);
        let (sx, sy) = v.to_screen(x, y);
        let r = (W / 2.0 * v.z) as i32 + 2;
        let tick = self.tick;
        for dx in -r..=r {
            if (dx + (tick / 4) as i32).rem_euclid(6) == 0 {
                let c = if (dx / 6).rem_euclid(2) == 0 {
                    warm
                } else {
                    hot
                };
                for dy in -1..=1 {
                    let a = if dy == 0 { 0.5 } else { 0.18 };
                    self.blend_out(sx + dx, sy + dy, c, a);
                }
            }
        }
    }
}
