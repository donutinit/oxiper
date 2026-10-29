//! The tiger: what it is up to, how it moves, and how it is put together on
//! screen. Nothing in the jungle can hurt it: hunts may fail, rivals may try,
//! but it always walks away, and only the saucer ever takes it anywhere.

use std::f32::consts::PI;

use super::art::{self, HEAD_AT, HEAD_PIVOT};
use super::body::{Gait, Legs, REAR, TailPose};
use super::fiestas::Visit;
use super::jungle::{FEET, Jungle};
use super::{Dice, Kind, Rgb, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mood {
    /// Walking its territory.
    Patrol,
    /// Reared up against a tree, raking its bark.
    Mark,
    /// Creeping up on prey, low, through the grass.
    Stalk,
    /// The burst.
    Charge,
    /// In the air.
    Pounce,
    /// Lying over its catch, behind the grass.
    Feed,
    /// At the water's edge, lapping.
    Drink,
    /// Asleep in the shade.
    Nap,
    /// Roaring.
    Roar,
    /// Seeing off a rival: a roar, then a rush at it.
    Fend,
    /// Watching the saucer come down.
    Look,
    /// In the saucer's beam, or inside it.
    Carried,
    /// On a holiday: standing before an ofrenda or a nacimiento in the
    /// ruins, head bowed.
    Visit,
}

/// Speeds, in world pixels per step.
const WALK: f32 = 0.55;
const TROT: f32 = 1.15;
const CREEP: f32 = 0.3;
const RUN: f32 = 2.9;
const SWIM: f32 = 0.42;
/// How deep it swims: its body sinks this far.
const SWIM_DEPTH: f32 = 11.0;
/// The body pixel at the middle of the torso, which is its position, and
/// the row its paws stand on.
const MID: i32 = 13;
const GROUND: i32 = 20;
/// How far ahead of its middle its jaws are.
const JAWS: f32 = 25.0;
/// How long a flight a pounce is, in steps, and the pull of gravity.
const FLIGHT: f32 = 18.0;
const GRAVITY: f32 = 0.16;

pub struct Tiger {
    pub kind: Kind,
    /// The middle of its torso, in local world pixels.
    pub x: f32,
    /// How high its feet are off the ground, and how fast it is rising.
    pub lift: f32,
    vy: f32,
    vx: f32,
    /// 1 facing right, -1 facing left (before any turn under way).
    pub dir: f32,
    /// How far through a turn it is, `(0, 1)`, or 0 when going straight.
    pub turn: f32,
    turn_rate: f32,
    /// World pixels per step.
    pub speed: f32,
    pub mood: Mood,
    /// Steps since the mood began.
    pub steps: u32,
    /// Real seconds left of the current mood, for moods that last a while.
    pub clock: f32,
    /// Real seconds before it is hungry, or thirsty, again.
    pub hunger: f32,
    pub thirst: f32,
    /// Real seconds before it next calls on an ofrenda, and where the ruin
    /// it is at stands.
    pub visit: f32,
    shrine: f32,
    /// Where it is going.
    pub tx: f32,
    trot: bool,
    /// The animal it is after, and the rival it is seeing off.
    pub prey: Option<u32>,
    pub rival: Option<u32>,
    /// The tree it is going to claw: where, and which.
    tree: Option<(f32, i64, i32)>,
    /// Whether it has reached the water's edge to drink.
    lapping: bool,
    gait: Gait,
    phase: f32,
    /// How low it crouches, how far it lies down, how deep it swims: 0 to 1.
    crouch: f32,
    lying: f32,
    pub sink: f32,
    /// Its head's tilt (nose down positive), how wide its jaws are open,
    /// and steps left of a blink.
    head: f32,
    pub mouth: f32,
    blink: u16,
    tail_phase: f32,
    lash: f32,
    /// In the beam: how far round it has spun.
    pub spin: f32,
    /// Inside the saucer.
    pub gone: bool,
    /// Its tilt in the air, nose up negative.
    pitch: f32,
    /// What it has caught and missed.
    pub caught: u32,
    pub missed: u32,
}

impl Tiger {
    pub fn new(kind: Kind, rng: &mut Rng) -> Tiger {
        let dir = rng.sign();
        Tiger {
            kind,
            x: 0.0,
            lift: 0.0,
            vy: 0.0,
            vx: 0.0,
            dir,
            turn: 0.0,
            turn_rate: 0.0,
            speed: 0.0,
            mood: Mood::Patrol,
            steps: 0,
            clock: 0.0,
            hunger: rng.rangef(15.0, 50.0),
            thirst: rng.rangef(40.0, 120.0),
            visit: 12.0,
            shrine: 0.0,
            tx: dir * 150.0,
            trot: false,
            prey: None,
            rival: None,
            tree: None,
            lapping: false,
            gait: Gait::Walk,
            phase: 0.0,
            crouch: 0.0,
            lying: 0.0,
            sink: 0.0,
            head: 0.0,
            mouth: 0.0,
            blink: 0,
            tail_phase: 0.0,
            lash: 0.0,
            spin: 0.0,
            gone: false,
            pitch: 0.0,
            caught: 0,
            missed: 0,
        }
    }

    pub fn clocks(&mut self, real: f32) {
        self.clock -= real;
        self.hunger -= real;
        self.thirst -= real;
        self.visit -= real;
    }

    pub fn shift(&mut self, dx: f32) {
        self.x -= dx;
        self.tx -= dx;
        self.shrine -= dx;
        if let Some(t) = &mut self.tree {
            t.0 -= dx;
        }
    }

    /// Which way it is going, easing from one side to the other in turns.
    pub fn heading(&self) -> f32 {
        if self.turn > 0.0 {
            self.dir * (PI * self.turn).cos()
        } else {
            self.dir
        }
    }

    /// Which way it faces on screen: its side views flip half way through a turn.
    fn face(&self) -> f32 {
        if self.turn >= 0.5 {
            -self.dir
        } else {
            self.dir
        }
    }

    /// Where its feet are, in world rows: on the ground, in the air, or
    /// sunk in the river.
    fn feet_y(&self) -> f32 {
        FEET as f32 - self.lift + self.sink * SWIM_DEPTH
    }

    /// The box its picture fits in: left, top, right, bottom in local world pixels.
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let f = self.face();
        let (back, front) = (33.0, 28.0);
        let (l, r) = if f > 0.0 {
            (self.x - back, self.x + front)
        } else {
            (self.x - front, self.x + back)
        };
        let tall = match self.mood {
            Mood::Mark => 42.0,
            Mood::Nap => 13.0,
            _ => 27.0,
        };
        let feet = self.feet_y();
        (l, feet - tall, r, feet)
    }

    /// Where its jaws are.
    pub fn jaws(&self) -> f32 {
        self.x + self.heading() * JAWS
    }

    /// Grass to hide the feeding tiger and its catch: from, to, how tall.
    pub fn cover(&self) -> Option<(f32, f32, f32)> {
        (self.mood == Mood::Feed && !self.gone).then_some((self.x - 30.0, self.x + 34.0, 12.0))
    }

    /// Standing on solid ground, free to be taken away.
    pub fn ready_for_ufo(&self) -> bool {
        !self.gone
            && self.lift == 0.0
            && self.sink < 0.05
            && !matches!(self.mood, Mood::Pounce | Mood::Carried)
    }

    /// Starts a mood.
    pub fn set(&mut self, mood: Mood) {
        self.mood = mood;
        self.steps = 0;
        self.lapping = false;
    }
}

impl Jungle {
    pub fn update_tiger(&mut self, rng: &mut Rng) {
        if self.tiger.gone {
            return;
        }
        self.tiger.steps += 1;
        self.think(rng);
        self.move_tiger(rng);
        self.tiger_effects(rng);
    }

    /// The holiday is over: a tiger on its way to an ofrenda has nowhere to go.
    pub fn undress(&mut self, rng: &mut Rng) {
        if self.tiger.mood == Mood::Visit {
            self.plan(rng);
        }
    }

    /// Picks what to do next, once it has got where it was going.
    fn plan(&mut self, rng: &mut Rng) {
        let (vw, _) = self.view_size();
        let x = self.tiger.x;
        let night = self.daylight < 0.3;
        let r = rng.f32();
        self.tiger.prey = None;
        self.tiger.tree = None;
        self.tiger.trot = false;
        if self.tiger.hunger <= 0.0
            && let Some(id) = self.life.prey_near(x, vw * 1.3)
        {
            self.tiger.prey = Some(id);
            self.tiger.clock = rng.rangef(25.0, 45.0);
            self.tiger.set(super::hunter::Mood::Stalk);
            return;
        }
        if self.tiger.thirst <= 0.0
            && !self.kind.snowy()
            && let Some(s) = self.water_near(x, vw * 1.6)
        {
            // To the near bank, facing the water.
            let bank = if (s.a - x).abs() < (s.b - x).abs() {
                s.a - JAWS + 3.0
            } else {
                s.b + JAWS - 3.0
            };
            self.tiger.tx = bank;
            self.tiger.set(Mood::Drink);
            return;
        }
        // On a holiday with ruins by the path, now and then it calls on them.
        if let Some(site) = self.pal.site()
            && self.tiger.visit <= 0.0
            && let Some(r) = self
                .ruins(x - vw * 1.2, x + vw * 1.2)
                .as_slice()
                .iter()
                .filter(|r| (r.x - x).abs() > 30.0)
                .min_by(|a, b| (a.x - x).abs().total_cmp(&(b.x - x).abs()))
                .copied()
        {
            // On the side it is on, its jaws at the doorway.
            let side = if r.x > x { -1.0 } else { 1.0 };
            self.tiger.shrine = r.x;
            self.tiger.tx = r.x + side * (site.stand + JAWS);
            self.tiger.visit = rng.rangef(60.0, 140.0) * self.set.rest_scale;
            self.tiger.set(Mood::Visit);
            return;
        }
        let sleepy = if night { 0.04 } else { 0.16 * self.daylight };
        if r < sleepy && self.water(x).is_none() {
            self.tiger.clock = rng.rangef(20.0, 45.0) * self.set.rest_scale;
            self.tiger.set(Mood::Nap);
            return;
        }
        if r < sleepy + 0.08 {
            let dir = self.tiger.dir;
            let ahead = self
                .trunks(x - 150.0, x + 150.0)
                .into_iter()
                .filter(|t| (t.x - x) * dir > 30.0 && self.water(t.x - dir * 30.0).is_none())
                .min_by(|a, b| (a.x - x).abs().total_cmp(&(b.x - x).abs()));
            if let Some(t) = ahead {
                self.tiger.tree = Some((t.x, t.id, t.w));
                self.tiger.tx = t.x - dir * (t.w as f32 / 2.0 + 20.0);
                self.tiger.set(Mood::Mark);
                return;
            }
        }
        if r < sleepy + 0.15 {
            self.tiger.set(Mood::Roar);
            return;
        }
        let back = rng.chance(0.25);
        let d = if back {
            -self.tiger.dir
        } else {
            self.tiger.dir
        };
        let far = rng.rangef(100.0, 340.0);
        self.tiger.tx = x + d * far;
        self.tiger.trot = far > 240.0 && rng.chance(0.5);
        self.tiger.set(Mood::Patrol);
    }

    /// Moods: what the tiger is up to and where it is going.
    fn think(&mut self, rng: &mut Rng) {
        let (vw, _) = self.view_size();
        let steps = self.tiger.steps;
        match self.tiger.mood {
            Mood::Patrol => {
                let t = &self.tiger;
                let there = (t.tx - t.x) * t.dir < 6.0 && t.turn == 0.0;
                if there || steps > 30 * 50 {
                    self.plan(rng);
                    return;
                }
                // Now and then, something else; checked once a second.
                if steps.is_multiple_of(30) && self.tiger.sink < 0.05 {
                    let x = self.tiger.x;
                    if let Some(id) = self.life.rival_near(x, 130.0) {
                        self.tiger.rival = Some(id);
                        self.tiger.set(Mood::Fend);
                    } else if self.tiger.hunger <= 0.0 && rng.chance(0.5) {
                        self.plan(rng);
                    }
                }
            }
            Mood::Stalk => {
                let Some(b) = self.tiger.prey.and_then(|id| self.life.beast(id)) else {
                    self.plan(rng);
                    return;
                };
                let t = &mut self.tiger;
                t.tx = b.x;
                let ahead = (b.x - t.jaws()) * t.heading();
                let close = (b.x - t.x).abs();
                if (b.fleeing && close < 110.0) || (ahead < 34.0 && ahead > -12.0) {
                    t.set(Mood::Charge);
                } else if t.clock <= 0.0 || close > vw * 2.0 {
                    t.hunger = rng.rangef(15.0, 30.0);
                    self.plan(rng);
                }
            }
            Mood::Charge => {
                let Some(b) = self.tiger.prey.and_then(|id| self.life.beast(id)) else {
                    self.tiger.missed += 1;
                    self.tiger.hunger = rng.rangef(20.0, 40.0);
                    self.plan(rng);
                    return;
                };
                let t = &mut self.tiger;
                t.tx = b.x + b.vx * 10.0;
                let ahead = (b.x - t.jaws()) * t.heading();
                if t.turn == 0.0 && ahead < 26.0 && ahead > -8.0 {
                    // Leap: far enough to land on it, wherever it is going.
                    t.vx = t.dir * ((ahead + 8.0) / FLIGHT + b.vx * t.dir).clamp(1.2, 4.0);
                    t.vy = GRAVITY * FLIGHT / 2.0;
                    t.lift = 0.01;
                    t.set(Mood::Pounce);
                    let (x, y) = (t.x, FEET as f32);
                    self.life.dust(x, y, 6, rng);
                } else if steps > 30 * 6 || (b.x - t.x).abs() > 260.0 {
                    // Outrun, for once. Next time.
                    t.missed += 1;
                    t.hunger = rng.rangef(20.0, 45.0);
                    self.plan(rng);
                }
            }
            Mood::Pounce => {
                if self.tiger.lift <= 0.0 && self.tiger.steps > 2 {
                    self.land(rng);
                }
            }
            Mood::Feed => {
                if self.tiger.clock <= 0.0 {
                    self.tiger.hunger = rng.rangef(120.0, 260.0) * self.set.rest_scale;
                    self.tiger.thirst = self.tiger.thirst.min(10.0);
                    self.plan(rng);
                }
            }
            Mood::Drink => {
                let near = self.water_near(self.tiger.x, 60.0);
                let t = &mut self.tiger;
                if !t.lapping {
                    if (t.tx - t.x).abs() < 2.0 && t.turn == 0.0 {
                        // Face the water and lower the head to it.
                        t.lapping = true;
                        t.clock = rng.rangef(6.0, 11.0);
                        if let Some(s) = near {
                            let mid = (s.a + s.b) / 2.0;
                            t.dir = if mid > t.x { 1.0 } else { -1.0 };
                        }
                    } else if steps > 30 * 40 {
                        self.plan(rng);
                    }
                } else if t.clock <= 0.0 {
                    t.thirst = rng.rangef(150.0, 320.0) * self.set.rest_scale;
                    // On across the water, now and then.
                    let d = if rng.chance(0.5) { t.dir } else { -t.dir };
                    t.tx = t.x + d * rng.rangef(140.0, 260.0);
                    t.set(Mood::Patrol);
                }
            }
            Mood::Nap => {
                if self.tiger.clock <= 0.0 {
                    self.plan(rng);
                }
            }
            Mood::Roar => {
                if steps == 16 {
                    self.roared(rng);
                }
                if steps > 70 {
                    self.plan(rng);
                }
            }
            Mood::Fend => {
                let rival = self.tiger.rival.and_then(|id| self.life.rival(id));
                let Some(rx) = rival else {
                    self.plan(rng);
                    return;
                };
                let t = &mut self.tiger;
                t.tx = rx;
                if steps == 16 {
                    self.roared(rng);
                }
                if steps == 50
                    && let Some(id) = self.tiger.rival
                {
                    // It has had enough of them: they had better run.
                    self.life.rout(id);
                }
                if steps > 120 || (rx - self.tiger.x).abs() > 260.0 {
                    self.tiger.rival = None;
                    self.plan(rng);
                }
            }
            Mood::Mark => {
                let Some((tree_x, id, w)) = self.tiger.tree else {
                    self.plan(rng);
                    return;
                };
                let t = &mut self.tiger;
                let dir = if tree_x > t.x { 1.0 } else { -1.0 };
                let spot = tree_x - dir * (w as f32 / 2.0 + 20.0);
                if t.clock > 0.0 && t.lapping {
                    // Rearing and raking.
                    if t.steps == 40 && !self.marks.contains(&id) {
                        self.marks.push(id);
                        if self.marks.len() > 64 {
                            self.marks.remove(0);
                        }
                    }
                } else if t.lapping {
                    t.tx = t.x - dir * 120.0;
                    t.set(Mood::Patrol);
                } else {
                    t.tx = spot;
                    if (spot - t.x).abs() < 2.0 && t.turn == 0.0 {
                        t.dir = dir;
                        t.lapping = true;
                        t.steps = 0;
                        t.clock = 3.0;
                    } else if steps > 30 * 30 {
                        self.plan(rng);
                    }
                }
            }
            Mood::Visit => {
                let cheer = self.pal.visit() == Visit::Cheer;
                let t = &mut self.tiger;
                let mut shout = false;
                if !t.lapping {
                    if (t.tx - t.x).abs() < 2.0 && t.turn == 0.0 {
                        // Face the shrine and bow to it, or look up at the emblem and roar.
                        t.lapping = true;
                        t.clock = rng.rangef(7.0, 12.0);
                        t.dir = if t.shrine > t.x { 1.0 } else { -1.0 };
                        if cheer {
                            t.steps = 0;
                        } else {
                            let (hx, y) = (t.jaws(), FEET as f32 - 26.0);
                            self.life.petals(hx, y, 10, rng);
                        }
                    } else if steps > 30 * 40 {
                        self.plan(rng);
                    }
                } else if t.clock <= 0.0 {
                    // On its way, either way.
                    let d = if rng.chance(0.5) { t.dir } else { -t.dir };
                    t.tx = t.x + d * rng.rangef(140.0, 260.0);
                    t.set(Mood::Patrol);
                } else if cheer && t.steps % 150 == 16 {
                    shout = true;
                }
                if shout && let Some(d) = self.pal.patrias {
                    self.cheer(&d, rng);
                }
            }
            Mood::Look | Mood::Carried => {}
        }
    }

    /// The pounce comes down: on its prey, mostly.
    fn land(&mut self, rng: &mut Rng) {
        let t = &mut self.tiger;
        t.lift = 0.0;
        t.vy = 0.0;
        let jaws = t.jaws();
        let (x, y) = (t.x, FEET as f32);
        let caught = t
            .prey
            .and_then(|id| self.life.beast(id).map(|b| (id, b)))
            .filter(|(_, b)| {
                // A deer that saw it coming may still twist away.
                (b.x - jaws).abs() < 14.0 && rng.chance(if b.fleeing { 0.6 } else { 0.9 })
            });
        self.life.dust(x, y, 10, rng);
        match caught {
            Some((id, _)) => {
                self.life.catch(id, rng);
                let t = &mut self.tiger;
                t.caught += 1;
                t.speed = 0.0;
                t.clock = rng.rangef(25.0, 45.0) * self.set.rest_scale;
                t.set(Mood::Feed);
            }
            None => {
                let t = &mut self.tiger;
                t.missed += 1;
                t.hunger = rng.rangef(20.0, 45.0);
                t.speed *= 0.3;
                t.prey = None;
                self.plan(rng);
            }
        }
    }

    /// What a roar does: the ground shakes, birds go up, every beast near
    /// enough runs, and the langurs bark about it.
    fn roared(&mut self, rng: &mut Rng) {
        self.shake = 0.9;
        let x = self.tiger.x;
        self.life.scare(x, 320.0, true);
        self.life.leaves(x, 14, rng);
    }

    /// Speed, turns, gaits and poses: the moving itself.
    fn move_tiger(&mut self, rng: &mut Rng) {
        let water = self.water(self.tiger.x);
        let snowy = self.kind.snowy();
        let cheer = self.pal.visit() == Visit::Cheer;
        let (river, pool) = match water {
            Some(s) if !snowy => (s.river, !s.river),
            _ => (false, false),
        };
        let t = &mut self.tiger;
        let far = (t.tx - t.x).abs();
        let (mut want, accel) = match t.mood {
            Mood::Patrol => (if t.trot { TROT } else { WALK }, 0.06),
            Mood::Stalk => (CREEP, 0.05),
            Mood::Charge => (RUN, 0.16),
            Mood::Fend if t.steps > 40 && t.steps < 90 => (RUN * 0.8, 0.14),
            Mood::Drink | Mood::Mark | Mood::Visit if !t.lapping => (WALK, 0.06),
            Mood::Pounce => (t.speed, 1.0),
            _ => (0.0, 0.12),
        };
        // Slow down to stop where it is going.
        if matches!(t.mood, Mood::Drink | Mood::Mark | Mood::Visit) && !t.lapping {
            want = want.min(far * 0.08 + 0.08);
        }
        if river {
            want = want.min(SWIM);
        } else if pool {
            want *= 0.7;
        }
        // Turn round when where it is going is behind it.
        if t.turn == 0.0 && want > 0.0 && t.mood != Mood::Pounce && (t.tx - t.x) * t.dir < -3.0 {
            t.turn = 1e-3;
            t.turn_rate = if t.mood == Mood::Charge {
                1.0 / 12.0
            } else {
                1.0 / 22.0
            };
        }
        // Facing a rival it is seeing off, or the water it drinks.
        if t.mood == Mood::Fend && t.turn == 0.0 && (t.tx - t.x) * t.dir < -3.0 {
            t.turn = 1e-3;
            t.turn_rate = 1.0 / 14.0;
        }
        t.speed += (want - t.speed) * accel;
        if t.turn > 0.0 {
            t.turn += t.turn_rate;
            if t.turn >= 1.0 {
                t.turn = 0.0;
                t.dir = -t.dir;
            }
        }
        if t.mood == Mood::Pounce {
            t.x += t.vx;
            t.lift += t.vy;
            t.vy -= GRAVITY;
            t.pitch = (-t.vy * 0.25).clamp(-0.35, 0.3);
            if t.lift <= 0.0 {
                t.lift = 0.0;
            }
        } else if t.mood != Mood::Carried {
            let h = t.heading();
            let pace = if t.turn > 0.0 {
                t.speed * (0.3 + 0.7 * h.abs())
            } else {
                t.speed
            };
            t.x += h * pace;
            t.pitch *= 0.8;
        }
        // The gait follows the speed; the stride follows the ground covered.
        t.gait = if river && t.mood != Mood::Pounce {
            Gait::Paddle
        } else if t.speed > 1.9 {
            Gait::Gallop
        } else if t.speed > 0.85 {
            Gait::Trot
        } else {
            Gait::Walk
        };
        let (duty, stride) = match t.gait {
            Gait::Walk => (0.62, 9.0),
            Gait::Trot => (0.5, 11.2),
            Gait::Gallop => (0.34, 17.0),
            Gait::Bound => (0.3, 16.0),
            Gait::Paddle => (0.5, 8.0),
        };
        t.phase = (t.phase + t.speed * duty / stride + if river { 0.02 } else { 0.0 }).fract();
        let ease = |v: &mut f32, to: f32, k: f32| *v += (to - *v) * k;
        let crouch = match t.mood {
            Mood::Stalk => 1.0,
            Mood::Drink if t.lapping => 0.7,
            Mood::Visit if t.lapping && !cheer => 0.3,
            Mood::Look => 0.35,
            _ => 0.0,
        };
        ease(&mut t.crouch, crouch, 0.08);
        ease(
            &mut t.lying,
            if t.mood == Mood::Feed { 1.0 } else { 0.0 },
            0.07,
        );
        let sink = if river && t.mood != Mood::Pounce {
            1.0
        } else {
            0.0
        };
        ease(&mut t.sink, sink, 0.06);
        if t.sink < 0.01 {
            t.sink = 0.0;
        }
        let chew = (t.steps as f32 * 0.35).sin();
        let head = match t.mood {
            Mood::Stalk => 0.2,
            Mood::Drink if t.lapping => 0.85 + 0.06 * (t.steps as f32 * 0.6).sin(),
            Mood::Feed => 0.62 + 0.07 * chew,
            Mood::Look => -0.4,
            Mood::Visit if t.lapping && cheer => -0.32 + 0.04 * (t.steps as f32 * 0.07).sin(),
            Mood::Visit if t.lapping => 0.55 + 0.05 * (t.steps as f32 * 0.07).sin(),
            Mood::Roar | Mood::Fend if t.steps < 60 => -0.18,
            Mood::Carried => -0.3,
            _ if river => -0.15,
            _ => 0.0,
        };
        ease(&mut t.head, head, 0.15);
        let roaring = |s: u32| {
            if s < 10 {
                s as f32 / 10.0 * 0.5
            } else if s < 46 {
                1.0
            } else {
                (1.0 - (s - 46) as f32 / 12.0).max(0.0)
            }
        };
        let mouth = match t.mood {
            Mood::Roar => roaring(t.steps),
            Mood::Fend if t.steps < 60 => roaring(t.steps),
            Mood::Visit if t.lapping && cheer => roaring(t.steps % 150),
            Mood::Look => 0.45,
            Mood::Carried => 0.6 + 0.4 * (t.steps as f32 * 0.2).sin().abs(),
            Mood::Feed => 0.3 * chew.max(0.0),
            _ => 0.0,
        };
        t.mouth = mouth;
        if t.blink > 0 {
            t.blink -= 1;
        } else if rng.chance(0.012) {
            t.blink = 4;
        }
        let lash = match t.mood {
            Mood::Roar | Mood::Fend | Mood::Look | Mood::Carried => 1.0,
            Mood::Visit if t.lapping && cheer => 1.0,
            Mood::Stalk => 0.5,
            _ => 0.25,
        };
        ease(&mut t.lash, lash, 0.05);
        t.tail_phase += 0.06 + t.speed * 0.04 + t.lash * 0.12;
    }

    /// What the tiger leaves behind: splashes and wakes, dust, z's.
    fn tiger_effects(&mut self, rng: &mut Rng) {
        let t = &self.tiger;
        let (x, h, steps) = (t.x, t.heading(), t.steps);
        if t.sink > 0.5 && rng.chance(0.25) {
            let wx = x + h * rng.rangef(-6.0, 22.0);
            self.life.splash(wx, FEET as f32 - 2.0, rng);
        }
        if t.mood == Mood::Drink && t.lapping && steps.is_multiple_of(9) {
            let jx = t.jaws();
            self.life.splash(jx, FEET as f32 - 1.0, rng);
        }
        if t.mood == Mood::Charge && t.sink < 0.1 && rng.chance(0.5) {
            self.life.dust(x - h * 18.0, FEET as f32, 1, rng);
        }
        if t.mood == Mood::Nap && steps % 40 == 1 {
            let hx = x + t.dir * 14.0;
            self.note(hx, FEET as f32 - 12.0, "z", Rgb(235, 240, 255), 80, 0.18);
        }
    }

    pub fn draw_tiger(&mut self) {
        let t = &self.tiger;
        if t.gone {
            return;
        }
        let kind = match t.kind {
            Kind::Bengal => 0,
            Kind::Siberian => 1,
            Kind::White => 2,
        };
        // On Día de Muertos it is an alebrije, painted from the holiday's colors,
        // and on Fiestas Patrias a jaguar.
        let (skin, lut) = if let Some(dress) = self.pal.fest {
            #[cfg(test)]
            {
                self.painted.alebrije = true;
            }
            (
                &art::ALEBRIJE_SKIN,
                self.lut_spirit(&dress.alebrije(t.kind)),
            )
        } else if let Some(dress) = self.pal.patrias {
            #[cfg(test)]
            {
                self.painted.jaguar = true;
            }
            (&art::JAGUAR_SKIN, self.lut(&dress.jaguar(t.kind)))
        } else {
            (&art::TIGER_SKIN, self.lut(art::TIGER_COLORS[kind]))
        };
        let water = (t.sink > 0.05).then(|| (FEET - 2, self.water_color()));
        let feet = self.tiger.feet_y();
        let t = &self.tiger;
        let face = t.face();
        // In the beam it turns slowly round and round.
        let spun = t.mood == Mood::Carried && (t.spin.fract() - 0.5).abs() < 0.17;
        let fronting = (t.turn > 0.33 && t.turn < 0.67) || spun;
        self.fig.clear();
        if t.mood == Mood::Nap {
            let breath = ((self.tick / 45) % 2) as usize;
            self.fig.stamp(skin.sleep[breath], MID - 17, GROUND - 12);
            self.place(
                t.x,
                feet,
                (MID, GROUND),
                face,
                0.0,
                (MID, GROUND),
                &lut,
                water,
            );
            return;
        }
        if fronting {
            self.fig.stamp(skin.front, MID - 8, GROUND - 24);
            let tilt = if t.mood == Mood::Carried {
                0.15 * (t.steps as f32 * 0.1).sin()
            } else {
                0.0
            };
            self.place(
                t.x,
                feet,
                (MID, GROUND),
                face,
                tilt,
                (MID, GROUND - 12),
                &lut,
                water,
            );
            return;
        }
        let bob = match t.gait {
            Gait::Walk if t.speed > 0.05 => 0.5 * (1.0 - (4.0 * PI * t.phase).cos()),
            Gait::Trot if t.speed > 0.05 => 0.6 * (1.0 - (4.0 * PI * t.phase).cos()),
            Gait::Gallop => 1.2 * (1.0 - (2.0 * PI * t.phase).cos()),
            _ => 0.0,
        };
        let drop = (t.crouch * 4.0 + t.lying * 9.0 + bob).round();
        let legs = match t.mood {
            Mood::Pounce => Legs::Reach((0.5 + t.vy * 0.4).clamp(0.0, 1.0)),
            Mood::Carried => Legs::Dangle(t.steps as f32 * 0.04),
            Mood::Mark if t.lapping => Legs::Rear(t.steps as f32 * 0.05),
            _ if t.lying > 0.5 => Legs::Fold,
            _ if t.sink > 0.5 => Legs::Gait(Gait::Paddle, t.phase),
            _ if t.speed > 0.04 => Legs::Gait(t.gait, t.phase),
            _ => Legs::Stand,
        };
        let pose = match t.mood {
            Mood::Charge | Mood::Pounce => TailPose {
                base: PI - 0.12,
                curl: -0.015,
                sway: 0.2,
                phase: t.tail_phase,
            },
            Mood::Stalk => TailPose {
                base: 0.86 * PI,
                curl: 0.004,
                sway: 0.45,
                phase: t.tail_phase,
            },
            Mood::Feed => TailPose {
                base: 0.96 * PI,
                curl: -0.025,
                sway: 0.25,
                phase: t.tail_phase,
            },
            Mood::Carried => TailPose {
                base: 0.5 * PI,
                curl: 0.04,
                sway: 1.0,
                phase: t.tail_phase,
            },
            _ => TailPose {
                base: 0.6 * PI,
                curl: 0.075,
                sway: 0.3 + 0.6 * t.lash,
                phase: t.tail_phase,
            },
        };
        let head_s = if t.mouth > 0.62 {
            skin.heads[3]
        } else if t.mouth > 0.22 {
            skin.heads[2]
        } else if t.blink > 0 || t.mood == Mood::Feed {
            skin.heads[1]
        } else {
            skin.heads[0]
        };
        let (head, crouch) = (t.head, t.crouch);
        let (angle, pivot) = match t.mood {
            Mood::Pounce => (t.pitch, (MID, 6)),
            Mood::Mark if t.lapping => (
                REAR,
                (skin.build.hind.hip.0 as i32, skin.build.hind.hip.1 as i32),
            ),
            Mood::Carried => (0.12 * (t.steps as f32 * 0.07).sin(), (MID, 6)),
            // Galloping, the body rocks from the hind legs to the fore legs.
            _ if t.gait == Gait::Gallop && t.speed > 1.5 => {
                (0.09 * (2.0 * PI * t.phase).sin(), (MID, 8))
            }
            _ => (0.0, (MID, 6)),
        };
        let fig = &mut self.fig;
        fig.far_legs(skin.build, legs, drop);
        fig.near_legs(skin.build, legs, drop);
        if let Some(tail) = &skin.build.tail {
            fig.tail(tail, pose);
        }
        fig.stamp(skin.torso, skin.torso_at.0, skin.torso_at.1);
        let (hx, hy) = (
            HEAD_AT.0 + (crouch * 1.5) as i32,
            HEAD_AT.1 + (crouch * 2.5) as i32,
        );
        fig.stamp_turned(head_s, hx, hy, HEAD_PIVOT, head);
        self.place(
            t.x,
            feet + drop,
            (MID, GROUND),
            face,
            angle,
            pivot,
            &lut,
            water,
        );
    }
}
