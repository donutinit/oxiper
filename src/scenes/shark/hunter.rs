//! The shark: what it is up to, how it swims, and how its hand-drawn parts
//! are put together on screen.

use std::f32::consts::{PI, TAU};

use super::art::{self, Kind, Rig, Sprite, TAIL_W};
use super::life::Speck;
use super::sea::{SURF, Sea};
use super::{Dice, Rgb, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mood {
    /// Patrolling, rising and sinking gently.
    Cruise,
    /// Nosing at something on the floor, or following a ray or a turtle.
    Investigate,
    /// Creeping up on a school from below, out of its sight.
    Stalk,
    /// The burst and the snap.
    Strike,
    /// Gliding just under the surface, the dorsal fin cutting the waterline.
    Glide,
    /// Slowly patrolling the bottom at night.
    Rest,
    /// Filter feeding: cruising under the surface, mouth wide.
    Feed,
}

/// A fish riding along: a remora under the belly or a pilot fish ahead.
#[derive(Clone, Copy)]
pub struct Rider {
    pub x: f32,
    pub y: f32,
    /// Where it keeps station, in the side view's pixels.
    at: (f32, f32),
    pub pilot: bool,
    pub phase: f32,
    pub face: f32,
}

pub struct Shark {
    pub kind: Kind,
    pub rig: &'static Rig,
    /// Middle of the body on its axis, in local world pixels.
    pub x: f32,
    pub y: f32,
    /// 1 facing right, -1 facing left (before any turn under way).
    pub dir: f32,
    /// How far through a turn it is, `(0, 1)`, or 0 when swimming straight.
    pub turn: f32,
    turn_rate: f32,
    /// World pixels per step.
    pub speed: f32,
    /// Radians, nose up positive.
    pub pitch: f32,
    /// How much the body bends, in pixels at the ends.
    pub bend: f32,
    /// The tail beat's phase and how wide it swings, 0 to 1.
    pub phase: f32,
    pub amp: f32,
    /// How open the mouth is, 0 to 1, and how open it is going.
    pub gape: f32,
    gape_to: f32,
    pub mood: Mood,
    /// Where it is heading.
    pub tx: f32,
    pub ty: f32,
    /// The school it hunts.
    prey: usize,
    /// Steps since the mood began.
    steps: u32,
    /// Real seconds left of the current mood, for moods that last a while.
    pub clock: f32,
    /// Real seconds before it feels like hunting again.
    pub hunger: f32,
    pub riders: [Rider; 4],
}

impl Shark {
    /// Size of the side view.
    pub const W: i32 = 64;
    pub const H: i32 = 28;

    pub fn new(kind: Kind, rng: &mut Rng) -> Shark {
        let y = rng.rangef(60.0, 100.0);
        let rig = kind.rig();
        let mut riders = [Rider {
            x: 0.0,
            y,
            at: (0.0, 0.0),
            pilot: false,
            phase: 0.0,
            face: 1.0,
        }; 4];
        // Two remoras under the belly, two pilot fish by the snout.
        for (i, r) in riders.iter_mut().enumerate() {
            r.pilot = i >= 2;
            r.at = if r.pilot {
                (
                    rig.mouth.0 as f32 + rng.rangef(-2.0, 8.0),
                    rig.mouth.1 as f32 + rng.rangef(4.0, 11.0),
                )
            } else {
                let (c, row) = rig.belly[i];
                (c as f32, row as f32)
            };
            r.phase = rng.rangef(0.0, TAU);
        }
        let dir = rng.sign();
        let mut s = Shark {
            kind,
            rig,
            x: 0.0,
            y,
            dir,
            turn: 0.0,
            turn_rate: 0.0,
            speed: rig.cruise,
            pitch: 0.0,
            bend: 0.0,
            phase: 0.0,
            amp: 0.5,
            gape: 0.0,
            gape_to: 0.0,
            mood: Mood::Cruise,
            tx: dir * 200.0,
            ty: y,
            prey: 0,
            steps: 0,
            clock: 0.0,
            hunger: rng.rangef(8.0, 30.0),
            riders,
        };
        for i in 0..4 {
            let (x, y) = s.anchor(s.riders[i].at);
            s.riders[i].x = x;
            s.riders[i].y = y;
            s.riders[i].face = dir;
        }
        s
    }

    pub fn clocks(&mut self, real: f32) {
        self.clock -= real;
        self.hunger -= real;
    }

    pub fn shift(&mut self, dx: f32) {
        self.x -= dx;
        self.tx -= dx;
        for r in &mut self.riders {
            r.x -= dx;
        }
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    /// Which way it is swimming, easing from one side to the other in turns.
    pub fn heading(&self) -> f32 {
        if self.turn > 0.0 {
            self.dir * (PI * self.turn).cos()
        } else {
            self.dir
        }
    }

    /// A point of the side view (column, row) in local world pixels, as it
    /// is foreshortened in turns and tilted with the pitch.
    pub fn anchor(&self, (c, r): (f32, f32)) -> (f32, f32) {
        let along = c + 0.5 - Shark::W as f32 / 2.0;
        let h = self.heading();
        (
            self.x + h * along * self.pitch.cos(),
            self.y + r - self.rig.axis as f32 - along * h.abs() * self.pitch.sin(),
        )
    }

    /// The box its picture fits in, tilted as it is: left, top, right,
    /// bottom in local world pixels.
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let (hw, axis) = (Shark::W as f32 / 2.0, self.rig.axis as f32);
        let (up, down) = (axis, Shark::H as f32 - axis);
        let (c, s) = (self.pitch.cos(), self.pitch.sin().abs());
        let (w, top, bottom) = (
            hw * c + down.max(up) * s,
            up * c + hw * s,
            down * c + hw * s,
        );
        (
            self.x - w - 1.0,
            self.y - top - 1.0,
            self.x + w + 1.0,
            self.y + bottom + 1.0,
        )
    }

    /// Where its jaws meet, in local world pixels.
    pub fn mouth(&self) -> (f32, f32) {
        let (c, r) = self.rig.mouth;
        self.anchor((c as f32, r as f32))
    }
}

impl Sea {
    pub fn update_shark(&mut self, rng: &mut Rng) {
        self.think(rng);
        self.swim();
        self.ride();
        self.wake(rng);
    }

    /// Picks somewhere to cruise to: mostly ahead, in the lower water.
    fn cruise_to(&mut self, rng: &mut Rng) {
        let s = &self.shark;
        let back = rng.chance(0.22);
        let d = if back { -s.dir } else { s.dir };
        let tx = s.x + d * rng.rangef(110.0, 340.0);
        let bed = self.bed(tx) as f32;
        let ty = rng.rangef(SURF as f32 + 40.0, bed - 18.0);
        let s = &mut self.shark;
        (s.tx, s.ty) = (tx, ty);
    }

    fn set_mood(&mut self, mood: Mood) {
        self.shark.mood = mood;
        self.shark.steps = 0;
    }

    /// Moods: what the shark is up to and where it is going.
    fn think(&mut self, rng: &mut Rng) {
        self.shark.steps += 1;
        let (vw, _) = self.view_size();
        let night = self.daylight < 0.25;
        let steps = self.shark.steps;
        match self.shark.mood {
            Mood::Cruise => {
                let s = &self.shark;
                if (s.tx - s.x) * s.dir < 12.0 && s.turn == 0.0 || steps > 1200 {
                    self.cruise_to(rng);
                    self.shark.steps = 1;
                }
                // Now and then, something else; checked once a second.
                if steps.is_multiple_of(30) && self.shark.turn == 0.0 {
                    let (sx, sdir, hunger) = (self.shark.x, self.shark.dir, self.shark.hunger);
                    // Hunting is best at dawn and dusk, still worth a try at night.
                    let appetite = if night {
                        0.12
                    } else if self.daylight < 0.8 {
                        0.6
                    } else {
                        0.35
                    };
                    let hunts = hunger <= 0.0 && !self.shark.kind.filters() && rng.chance(appetite);
                    let school = if hunts {
                        self.nearest_school(vw * 1.4)
                    } else {
                        None
                    };
                    if let Some(i) = school {
                        self.shark.prey = i;
                        self.shark.clock = rng.rangef(18.0, 30.0);
                        self.set_mood(Mood::Stalk);
                    } else if night && rng.chance(0.3) {
                        self.set_mood(Mood::Rest);
                        self.shark.clock = rng.rangef(40.0, 100.0) * self.set.rest_scale;
                        let x = sx + sdir * rng.rangef(80.0, 200.0);
                        self.shark.tx = x;
                    } else if !night && hunger <= 0.0 && self.shark.kind.filters() {
                        if rng.chance(0.3) {
                            self.set_mood(Mood::Feed);
                            self.shark.clock = rng.rangef(15.0, 30.0);
                            self.shark.tx = sx + sdir * 400.0;
                        }
                    } else if !night && rng.chance(0.05) {
                        self.set_mood(Mood::Glide);
                        self.shark.clock = rng.rangef(10.0, 22.0);
                        self.shark.tx = sx + sdir * 400.0;
                    } else if self.visitor_near(180.0).is_some() && rng.chance(0.3) {
                        // Something new about: a closer look.
                        self.set_mood(Mood::Investigate);
                    } else if rng.chance(0.06) {
                        let x = sx + sdir * rng.rangef(70.0, 150.0);
                        let bed = self.bed(x) as f32;
                        self.set_mood(Mood::Investigate);
                        (self.shark.tx, self.shark.ty) = (x, bed - 14.0);
                    }
                }
            }
            Mood::Investigate => {
                // Following a ray or a turtle, a little above it.
                if let Some((vx, vy)) = self.visitor_near(240.0) {
                    let bed = self.bed(vx) as f32;
                    (self.shark.tx, self.shark.ty) = (vx, (vy - 12.0).min(bed - 15.0));
                }
                let s = &mut self.shark;
                let near = (s.tx - s.x).abs() < 30.0;
                // A close look, mouth open a little, then off again.
                s.gape_to = if near { 0.45 } else { 0.0 };
                if near && s.steps > 40 || s.steps > 30 * 14 {
                    s.gape_to = 0.0;
                    self.set_mood(Mood::Cruise);
                    self.cruise_to(rng);
                }
            }
            Mood::Stalk => {
                let Some((cx, cy)) = self.life.center(self.shark.prey) else {
                    self.set_mood(Mood::Cruise);
                    return;
                };
                let bed = self.bed(cx) as f32;
                let s = &mut self.shark;
                let side = if cx > s.x { 1.0 } else { -1.0 };
                // Closing in from below, out of the school's sight.
                s.tx = cx + side * 20.0;
                s.ty = (cy + 26.0).min(bed - 16.0);
                let facing = (cx - s.x) * s.heading() > 0.0;
                let close = (cx - s.x).abs() < 90.0 && s.y > cy + 6.0;
                if facing && close && s.turn == 0.0 {
                    s.tx = cx;
                    s.ty = cy;
                    self.set_mood(Mood::Strike);
                } else if s.clock <= 0.0 {
                    s.hunger = rng.rangef(10.0, 25.0);
                    self.set_mood(Mood::Cruise);
                    self.cruise_to(rng);
                }
            }
            Mood::Strike => self.strike(rng),
            Mood::Glide => {
                let s = &mut self.shark;
                s.ty = SURF as f32 + 11.0;
                if (s.tx - s.x) * s.dir < 40.0 {
                    s.tx = s.x + s.dir * 300.0;
                }
                // The glide's time only runs once it is up there.
                if s.y > SURF as f32 + 16.0 {
                    s.clock = s.clock.max(8.0);
                }
                if s.clock <= 0.0 || night || s.steps > 30 * 90 {
                    self.set_mood(Mood::Cruise);
                    self.cruise_to(rng);
                }
            }
            Mood::Feed => {
                let s = &mut self.shark;
                s.ty = SURF as f32 + 22.0;
                if (s.tx - s.x) * s.dir < 40.0 {
                    s.tx = s.x + s.dir * 300.0;
                }
                let there = (s.y - s.ty).abs() < 10.0;
                s.gape_to = if there { 1.0 } else { 0.3 };
                if there {
                    // Plankton drawn in at the open mouth.
                    let (mx, my) = s.mouth();
                    let h = s.heading();
                    for _ in 0..2 {
                        let (px, py) = (mx + h * rng.rangef(6.0, 22.0), my + rng.rangef(-6.0, 6.0));
                        let life = rng.range(18, 30) as u16;
                        let v = (
                            (mx - px) / life as f32 + h * s.speed,
                            (my - py) / life as f32,
                        );
                        self.life.fleck(Speck::Plankton, (px, py), v, life);
                    }
                }
                if self.shark.clock <= 0.0 || night {
                    let s = &mut self.shark;
                    s.gape_to = 0.0;
                    s.hunger = rng.rangef(30.0, 60.0);
                    self.set_mood(Mood::Cruise);
                    self.cruise_to(rng);
                }
            }
            Mood::Rest => {
                let bed = self.bed(self.shark.tx) as f32;
                let s = &mut self.shark;
                s.ty = bed - 15.0;
                if (s.tx - s.x) * s.dir < 10.0 && s.turn == 0.0 {
                    let d = if rng.chance(0.4) { -s.dir } else { s.dir };
                    s.tx = s.x + d * rng.rangef(80.0, 220.0);
                }
                if s.clock <= 0.0 || self.daylight > 0.35 {
                    self.set_mood(Mood::Cruise);
                    self.cruise_to(rng);
                }
            }
        }
    }

    /// Where a ray or turtle within `range` of the shark is, if there is one.
    fn visitor_near(&self, range: f32) -> Option<(f32, f32)> {
        let x = self.shark.x;
        self.life
            .visitors
            .iter()
            .find(|v| (v.x - x).abs() < range)
            .map(|v| (v.x, v.y))
    }

    /// The school nearest the shark within `range`, if it has fish left.
    fn nearest_school(&self, range: f32) -> Option<usize> {
        let (x, y) = self.shark.center();
        (0..self.life.schools.len())
            .filter_map(|i| {
                let (cx, cy) = self.life.center(i)?;
                let d = (cx - x).abs() + (cy - y).abs();
                (d < range).then_some((i, d))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }

    /// The burst: straight at the nearest fish, jaws opening on the way in,
    /// and the snap.
    fn strike(&mut self, rng: &mut Rng) {
        let (mx, my) = self.shark.mouth();
        let h = self.shark.heading();
        // The nearest fish ahead of the jaws.
        let mut best: Option<(usize, f32)> = None;
        for (i, f) in self.life.fish.iter().enumerate() {
            if !f.alive || (f.x - mx) * h < -4.0 {
                continue;
            }
            let d = (f.x - mx).abs() + (f.y - my).abs() * 1.4;
            if best.is_none_or(|(_, b)| d < b) {
                best = Some((i, d));
            }
        }
        let s = &mut self.shark;
        let done = |s: &mut Shark, rng: &mut Rng, hunger: (f32, f32)| {
            s.gape_to = 0.0;
            s.hunger = rng.rangef(hunger.0, hunger.1);
        };
        match best {
            Some((i, d)) if s.steps < 75 => {
                let f = self.life.fish[i];
                (s.tx, s.ty) = (f.x + h * 10.0, f.y);
                s.gape_to = if d < 34.0 { 1.0 } else { 0.0 };
                if d < 7.0 && s.gape > 0.6 {
                    // Caught: the fish is gone in a puff of bubbles and scales.
                    self.life.fish[i].alive = false;
                    self.life.puff(f.x, f.y, rng);
                    done(&mut self.shark, rng, (40.0, 90.0));
                    self.scatter(f.x, f.y, 44.0);
                    self.set_mood(Mood::Cruise);
                    self.cruise_to(rng);
                }
            }
            _ => {
                // Missed: the school bolts and the shark gives up for now.
                done(s, rng, (15.0, 35.0));
                self.scatter(mx, my, 44.0);
                self.set_mood(Mood::Cruise);
                self.cruise_to(rng);
            }
        }
    }

    /// Speed, pitch, turns and the tail: the swimming itself.
    fn swim(&mut self) {
        let bed_ahead = {
            let s = &self.shark;
            self.bed(s.x + s.heading() * 24.0).min(self.bed(s.x)) as f32
        };
        let s = &mut self.shark;
        let rig = s.rig;
        let (want, accel) = match s.mood {
            Mood::Cruise => (rig.cruise, 0.015),
            Mood::Investigate => {
                if (s.tx - s.x).abs() < 40.0 {
                    (rig.cruise * 0.45, 0.03)
                } else {
                    (rig.cruise * 0.75, 0.02)
                }
            }
            Mood::Stalk => (rig.cruise * 0.8, 0.02),
            Mood::Strike => (rig.burst, 0.12),
            Mood::Glide => (rig.cruise * 0.8, 0.015),
            Mood::Rest => (rig.cruise * 0.4, 0.015),
            Mood::Feed => (rig.cruise * 0.8, 0.015),
        };
        s.speed += (want - s.speed) * accel;
        // Turn round when where it is going is behind it.
        if s.turn == 0.0 && (s.tx - s.x) * s.dir < -8.0 && s.mood != Mood::Strike {
            s.turn = 1e-3;
            s.turn_rate = if s.mood == Mood::Stalk {
                1.0 / 32.0
            } else {
                1.0 / 46.0
            };
        }
        // Pitch towards the target, more gently in turns.
        let h = s.heading();
        let ahead = ((s.tx - s.x) * h).max(30.0);
        let lim = if s.mood == Mood::Strike { 0.42 } else { 0.3 };
        // Level off near the surface and the sand rather than poking the
        // snout out of the one or into the other.
        let half = Shark::W as f32 / 2.0;
        let axis = rig.axis as f32;
        let up = ((s.y - axis - (SURF as f32 - 3.0)) / half).clamp(0.0, lim);
        let down = ((bed_ahead - 1.0 - (s.y + Shark::H as f32 - axis)) / half).clamp(0.0, lim);
        let mut want_pitch = (-(s.ty - s.y) / ahead).atan().clamp(-down, up);
        if s.turn > 0.0 {
            want_pitch *= 0.3;
        }
        let rate = if s.mood == Mood::Strike { 0.02 } else { 0.006 };
        let old = s.pitch;
        s.pitch += (want_pitch - s.pitch).clamp(-rate, rate);
        // The body lags behind changes of pitch.
        s.bend += ((s.pitch - old) * -110.0 - s.bend) * 0.1;
        if s.turn > 0.0 {
            s.turn += s.turn_rate;
            if s.turn >= 1.0 {
                s.turn = 0.0;
                s.dir = -s.dir;
            }
        }
        let h = s.heading();
        // In a turn it swings round more than it moves on.
        let pace = if s.turn > 0.0 {
            s.speed * (0.35 + 0.65 * h.abs())
        } else {
            s.speed
        };
        s.x += h * pace * s.pitch.cos();
        s.y -= pace * s.pitch.sin();
        let top = SURF as f32 + if s.mood == Mood::Glide { 10.0 } else { 17.0 };
        let bottom = bed_ahead - 14.0;
        if s.y < top || s.y > bottom {
            s.y = s.y.clamp(top, bottom.max(top));
            s.pitch *= 0.9;
        }
        // The tail beats faster the faster it swims, hardest in turns and
        // bursts; gliding, it hardly moves at all.
        let hz = rig.beat * (0.55 + 0.5 * s.speed + if s.turn > 0.0 { 0.6 } else { 0.0 });
        s.phase = (s.phase + TAU * hz / 30.0) % TAU;
        let swing = match s.mood {
            Mood::Glide => 0.18,
            Mood::Rest => 0.4,
            Mood::Strike => 1.0,
            _ => 0.48 + 0.12 * s.speed,
        };
        let swing = if s.turn > 0.0 { 0.95 } else { swing };
        s.amp += (swing - s.amp) * 0.06;
        // The jaws open steadily and snap shut.
        let k = if s.gape_to > s.gape { 0.18 } else { 0.4 };
        s.gape += (s.gape_to - s.gape) * k;
    }

    /// Remoras keep to the belly, pilot fish to the snout, catching up
    /// when the shark turns.
    fn ride(&mut self) {
        let t = self.time();
        let s = &self.shark;
        let h = s.heading();
        let mut riders = s.riders;
        for (i, r) in riders.iter_mut().enumerate() {
            let (ax, ay) = s.anchor(r.at);
            r.phase += if r.pilot { 0.35 } else { 0.22 };
            // The second remora has let go for a while and swims alongside.
            let (k, wob) = if r.pilot {
                (0.07, 1.6)
            } else if i == 1 {
                (0.1, 1.2)
            } else {
                (0.3, 0.3)
            };
            let (tx, ty) = (
                ax + wob * (t * 0.9 + r.phase).sin(),
                ay + wob * 0.5 * (t * 1.3 + r.phase).cos(),
            );
            let vx = (tx - r.x) * k;
            r.x += vx;
            r.y += (ty - r.y) * k;
            if h.abs() > 0.3 {
                r.face = h.signum();
            }
        }
        self.shark.riders = riders;
    }

    /// What the shark leaves behind: foam where its fin cuts the surface,
    /// and, at night, plankton glowing in its wake.
    fn wake(&mut self, rng: &mut Rng) {
        let s = &self.shark;
        let fin_top = s.y - s.rig.axis as f32;
        if fin_top < SURF as f32 + 1.0 && rng.chance(0.5) {
            let x = s.x + s.heading() * rng.rangef(-4.0, 6.0);
            let (vx, vy) = (-s.heading() * rng.rangef(0.0, 0.3), rng.rangef(-0.05, 0.0));
            let y = SURF as f32 + rng.rangef(-1.0, 0.5);
            self.life
                .fleck(Speck::Foam, (x, y), (vx, vy), rng.range(20, 45) as u16);
        }
        let dark = 1.0 - self.daylight;
        if dark > 0.5 && rng.chance((dark - 0.5) * 1.2 * (0.25 + s.speed * 0.5).min(1.0)) {
            let (tx, ty) = s.anchor((rng.rangef(2.0, 14.0), rng.rangef(6.0, 22.0)));
            let (vx, vy) = (rng.rangef(-0.15, 0.15), rng.rangef(-0.1, 0.1));
            self.life
                .fleck(Speck::Spark, (tx, ty), (vx, vy), rng.range(20, 50) as u16);
            if rng.chance(0.3) {
                let (px, py) = s.anchor((rng.rangef(30.0, 46.0), rng.rangef(20.0, 27.0)));
                self.life
                    .fleck(Speck::Spark, (px, py), (vx, vy), rng.range(15, 35) as u16);
            }
        }
    }

    /// How far the tail is swung out of the picture plane, -1 to 1.
    fn swing(&self) -> f32 {
        self.shark.amp * self.shark.phase.sin()
    }

    /// Which tail frame shows: 0 at rest, up to 3 swung furthest; and
    /// whether it is swung towards the viewer.
    fn tail_frame(&self) -> (usize, bool) {
        let swing = self.swing();
        let a = swing.abs();
        let f = if a < 0.24 {
            0
        } else if a < 0.52 {
            1
        } else if a < 0.8 {
            2
        } else {
            3
        };
        (f, swing > 0.0)
    }

    pub fn draw_shark(&mut self) {
        let s = &self.shark;
        let rig = s.rig;
        // The shark's colors as lit at its depth.
        let mut lut = [None; 128];
        for &(ch, c) in rig.palette {
            lut[ch as usize] = Some(self.shade(c, s.y as i32, 0.08));
        }
        if s.turn > 0.0 {
            let p = s.turn;
            let (sprite, face) = if p < 0.16 {
                (None, s.dir)
            } else if p < 0.38 {
                (Some(rig.quarter), s.dir)
            } else if p < 0.62 {
                (Some(rig.front), s.dir)
            } else if p < 0.84 {
                (Some(rig.quarter), -s.dir)
            } else {
                (None, -s.dir)
            };
            match sprite {
                Some(sp) => {
                    let pitch = if (0.38..0.62).contains(&p) {
                        0.0
                    } else {
                        s.pitch
                    };
                    self.draw_whole(sp, &lut, face, pitch);
                }
                None => self.draw_side(&lut, face),
            }
        } else {
            self.draw_side(&lut, s.dir);
        }
        self.draw_riders();
    }

    /// Where a sprite of width `w` is put, facing `face`, tilted by `pitch`.
    fn pose(&self, w: usize, face: f32, pitch: f32, bend: f32) -> Pose {
        let s = &self.shark;
        let half = w as i32 / 2;
        Pose {
            squeeze: 0.0,
            ox: s.x.round() as i32,
            oy: s.y.round() as i32,
            half,
            axis: s.rig.axis,
            flip: w as i32 - 1 - 2 * half,
            face,
            a: (pitch / 2.0).tan(),
            b: -pitch.sin(),
            bend,
        }
    }

    /// The side view, put together from its parts: the tail frame for the
    /// beat and the head frame for the mouth, tilted and bent.
    fn draw_side(&mut self, lut: &[Option<Rgb>; 128], face: f32) {
        let s = &self.shark;
        let rig = s.rig;
        let (tail, near) = self.tail_frame();
        // A tail swung away from the viewer is in the body's shade.
        let mut far = *lut;
        if !near {
            for c in far.iter_mut().flatten() {
                *c = c.scale(0.84);
            }
        }
        let head = if s.gape > 0.62 {
            Some(rig.heads[1])
        } else if s.gape > 0.2 {
            Some(rig.heads[0])
        } else {
            None
        };
        let w = rig.side[0].len();
        let mut pose = self.pose(w, face, s.pitch, s.bend);
        // The rear of the body bends out of the picture with the tail, and
        // looks that much shorter.
        pose.squeeze = 2.0 * self.swing().abs();
        for c in 0..w {
            let (src, sc, pal): (Sprite, usize, &[Option<Rgb>; 128]) = if c < TAIL_W {
                (
                    if tail > 0 {
                        rig.tails[tail - 1]
                    } else {
                        rig.side
                    },
                    c,
                    &far,
                )
            } else if c >= rig.head_x
                && let Some(h) = head
            {
                (h, c - rig.head_x, lut)
            } else {
                (rig.side, c, lut)
            };
            for (r, row) in src.iter().enumerate() {
                if let Some(color) = pal[(row.as_bytes()[sc] & 127) as usize] {
                    let (x, y) = pose.place(c as i32, r as i32);
                    self.put(x, y, color);
                }
            }
        }
    }

    /// A whole sprite centred on the shark, for the turn.
    fn draw_whole(&mut self, sprite: Sprite, lut: &[Option<Rgb>; 128], face: f32, pitch: f32) {
        let pose = self.pose(sprite[0].len(), face, pitch, 0.0);
        for (r, row) in sprite.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                if let Some(color) = lut[(ch & 127) as usize] {
                    let (x, y) = pose.place(c as i32, r as i32);
                    self.put(x, y, color);
                }
            }
        }
    }

    fn draw_riders(&mut self) {
        for i in 0..self.shark.riders.len() {
            let r = self.shark.riders[i];
            let tail = (r.phase.sin() > 0.0) as usize;
            if r.pilot {
                self.draw_small(art::PILOT[tail], &art::PILOT_COLORS, r.x, r.y, r.face, 0.06);
            } else {
                self.draw_small(
                    art::REMORA[tail],
                    &art::REMORA_COLORS,
                    r.x,
                    r.y,
                    r.face,
                    0.06,
                );
            }
        }
    }
}

/// How a sprite's pixels land in the world: centred on the shark, rotated by
/// its pitch, bent, and mirrored when it faces left. The rotation is three
/// shears, each moving whole rows or columns by whole pixels, so no pixel of
/// the drawing is lost or doubled.
#[derive(Clone, Copy)]
struct Pose {
    /// World pixel of the sprite's middle column on the axis.
    ox: i32,
    oy: i32,
    half: i32,
    axis: i32,
    /// What mirroring adds to a column: 0 for odd widths, -1 for even ones.
    flip: i32,
    face: f32,
    /// The shears: `tan(pitch / 2)` along rows, `-sin(pitch)` along columns.
    a: f32,
    b: f32,
    bend: f32,
    /// How many pixels shorter the rear of the body looks.
    squeeze: f32,
}

/// Columns, from the tail end, that shorten as the tail swings.
const REAR: i32 = 24;

impl Pose {
    #[inline]
    fn place(&self, c: i32, r: i32) -> (i32, i32) {
        let pull = if c < REAR {
            (self.squeeze * (REAR - c) as f32 / REAR as f32).round() as i32
        } else {
            0
        };
        let (x, y) = (c + pull - self.half, r - self.axis);
        let x1 = x + (self.a * y as f32).round() as i32;
        let f = x1 as f32 / self.half.max(1) as f32;
        let y2 = y + (self.b * x1 as f32 + self.bend * f * f).round() as i32;
        let x3 = x1 + (self.a * y2 as f32).round() as i32;
        let x = if self.face > 0.0 { x3 } else { self.flip - x3 };
        (self.ox + x, self.oy + y2)
    }
}
