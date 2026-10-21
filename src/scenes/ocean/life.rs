//! Who lives in the sea, and who eats whom.
//!
//! Sardines and tetras swim in schools that keep their spacing, close ranks
//! when something hungry comes near and scatter from it. tanim's reef fish
//! cross on their own and flee too, except the pufferfish, which blows up
//! and is left alone. Groupers lie in wait and snap up the smallest fish
//! passing their mouths; barracudas stalk sardines and strike. A shark
//! stalks the schools and lunges into them. Dolphin pods come in near the
//! surface, round up fish, dash through them and leap. An orca pod circles a
//! school until it balls up and drifts to the surface, then the orcas take
//! turns through it; a shark that meets them runs, and is rarely quick
//! enough. Mantas glide past feeding on plankton, and now and then loop over
//! backwards. Everything is in pixels per tick at 20 ticks a second.

use std::f32::consts::TAU;

use super::art::{self, PUFFED, Role, Species, size};
use super::sea::Sea;
use super::tank::{Bit, BitKind, EYE, EYE_WHITE, PLANKTON};
use super::{Dice, Rng};
use crate::color::Color;

/// Ticks a manta's loop takes.
const LOOP: u32 = 120;
/// Pull back to the water of a dolphin in the air, pixels per tick per tick.
const GRAVITY: f32 = 0.05;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Act {
    /// Going about its business, or its pod's.
    Cruise,
    /// Closing in on `prey`.
    Stalk { prey: u32, t: u32 },
    /// The final burst at `prey`.
    Lunge { prey: u32, t: u32 },
    /// Running from a hunter.
    Flee,
    /// A manta's backward somersault, `t` ticks in, of radius `r`.
    Loop { t: u32, r: f32 },
    /// A dolphin racing up to leap.
    Leap,
}

pub struct Swimmer {
    pub sp: &'static Species,
    pub colors: [Color; 5],
    pub id: u32,
    /// Centre, in pixels.
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    /// Facing: 1 to the right, -1 to the left.
    pub dir: f32,
    /// Half the drawing's width and height.
    pub hw: f32,
    pub hh: f32,
    pub cruise: f32,
    pub phase: f32,
    /// Frames turned over so far.
    pub anim: f32,
    /// Tilt as drawn, eased toward the heading.
    pub slope: f32,
    /// The depth it likes to swim at.
    pub wy: f32,
    /// Its school or pod; 0 for none.
    pub group: u32,
    pub act: Act,
    /// Ticks until hungry again.
    pub full: u32,
    /// Ticks a pufferfish stays blown up.
    pub puffed: u32,
    /// Ticks before it heads off.
    pub stay: u32,
    pub leaving: bool,
    /// A dolphin above the surface.
    pub air: bool,
    pub alive: bool,
}

impl Swimmer {
    fn mouth(&self) -> (f32, f32) {
        (self.x + self.dir * self.hw * 0.9, self.y + self.hh * 0.2)
    }

    fn hungry(&self) -> bool {
        self.full == 0 && !self.leaving
    }
}

/// What a pod is up to.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Plan {
    /// Swimming along together.
    Travel,
    /// Circling the school `school` into a bait ball.
    Herd { school: u32 },
    /// All after one animal.
    Rush { quarry: u32 },
}

/// A school of fish or a pod of dolphins or orcas.
pub struct Group {
    pub id: u32,
    pub pod: bool,
    /// Heading and preferred depth.
    pub dir: f32,
    pub wy: f32,
    pub plan: Plan,
    /// Where the circling has got to.
    pub angle: f32,
    /// Ticks until the next one dashes through the ball.
    pub dash_in: u32,
    /// Ticks until the pod is hungry again, and before it heads off.
    pub full: u32,
    pub stay: u32,
    pub leaving: bool,
    // Measured every tick.
    pub members: Vec<usize>,
    pub cx: f32,
    pub cy: f32,
    pub spread: f32,
    /// How pressed a school is, 0 to 1: it closes ranks the more it is.
    pub alarm: f32,
}

/// One swimmer's wishes for a tick.
struct Steer {
    vx: f32,
    vy: f32,
    /// How quickly it gets there: 1 at once, small for big animals.
    k: f32,
    act: Act,
    catch: Option<usize>,
    leave: bool,
    puff: bool,
    /// Ticks to rest after a miss.
    rest: u32,
}

impl Steer {
    fn to(vx: f32, vy: f32, k: f32, act: Act) -> Steer {
        Steer {
            vx,
            vy,
            k,
            act,
            catch: None,
            leave: false,
            puff: false,
            rest: 0,
        }
    }
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    ((ax - bx) * (ax - bx) + (ay - by) * (ay - by)).sqrt()
}

/// A velocity of `speed` from (`ax`, `ay`) toward (`bx`, `by`).
fn toward(ax: f32, ay: f32, bx: f32, by: f32, speed: f32) -> (f32, f32) {
    let d = dist(ax, ay, bx, by).max(1e-3);
    ((bx - ax) / d * speed, (by - ay) / d * speed)
}

fn hunts(role: Role) -> bool {
    matches!(
        role,
        Role::Snapper | Role::Shark | Role::Dolphin | Role::Orca
    )
}

/// Whether `h` would eat `p`: the food chain.
fn eats(h: &Swimmer, p: &Swimmer) -> bool {
    if !p.alive || p.puffed > 0 || p.air || h.id == p.id {
        return false;
    }
    match (h.sp.role, p.sp.role) {
        (Role::Snapper, Role::School | Role::Reef) => p.hh <= h.hh * 0.75,
        (Role::Shark, Role::School | Role::Reef | Role::Snapper) => true,
        (Role::Dolphin, Role::School) => true,
        (Role::Dolphin, Role::Reef) => p.hh <= 3.0,
        (Role::Orca, Role::School | Role::Snapper | Role::Shark) => true,
        _ => false,
    }
}

/// Ticks a catch keeps a hunter from the next.
fn digest(h: Role, p: Role, rng: &mut Rng) -> u32 {
    let (lo, hi) = match (h, p) {
        (Role::Orca, Role::Shark) => (1500, 2500),
        (Role::Snapper, _) => (200, 500),
        (Role::Shark, _) => (250, 600),
        (Role::Dolphin, _) => (40, 120),
        _ => (60, 160),
    };
    rng.range(lo, hi) as u32
}

/// Most ticks a lunge lasts before it is a miss.
fn lunge_len(role: Role) -> u32 {
    match role {
        Role::Snapper => 12,
        Role::Shark => 24,
        Role::Dolphin => 22,
        _ => 30,
    }
}

impl Sea {
    pub fn find(&self, id: u32) -> Option<usize> {
        self.swim.iter().position(|s| s.id == id && s.alive)
    }

    fn group_at(&self, id: u32) -> Option<usize> {
        self.groups.iter().position(|g| g.id == id)
    }

    /// The rows the centre of something `hh` half tall may swim in.
    fn band(&self, hh: f32) -> (f32, f32) {
        let top = self.surf as f32 + hh + 2.0;
        let bottom = self.floor as f32 - hh - 1.0;
        if top > bottom {
            let m = (top + bottom) * 0.5;
            (m, m)
        } else {
            (top, bottom)
        }
    }

    /// How well something `sw` by `sh` suits this sea, 0 to 1.
    fn room(&self, sp: &Species) -> f32 {
        let (sw, sh) = size(sp.frames[0]);
        let wh = (self.floor - self.surf) as f32;
        (self.w as f32 * 0.5 / sw as f32)
            .min(wh * 0.55 / sh as f32)
            .min(1.0)
    }

    pub fn swimmer(
        &mut self,
        sp: &'static Species,
        (x, y): (f32, f32),
        dir: f32,
        rng: &mut Rng,
    ) -> Swimmer {
        let (sw, sh) = size(sp.frames[0]);
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        let cruise = rng.rangef(sp.cruise.0, sp.cruise.1);
        Swimmer {
            sp,
            colors: rng.pick(sp.palettes),
            id,
            x,
            y,
            vx: dir * cruise,
            vy: 0.0,
            dir,
            hw: sw as f32 * 0.5,
            hh: sh as f32 * 0.5,
            cruise,
            phase: rng.rangef(0.0, TAU),
            anim: 0.0,
            slope: 0.0,
            wy: y,
            group: 0,
            act: Act::Cruise,
            full: rng.range(40, 300) as u32,
            puffed: 0,
            stay: u32::MAX,
            leaving: false,
            air: false,
            alive: true,
        }
    }

    /// Just off the side it swims in from, or anywhere across when `on`.
    fn entry(&self, hw: f32, dir: f32, on: bool, rng: &mut Rng) -> f32 {
        if on {
            rng.rangef(0.0, self.w as f32)
        } else if dir > 0.0 {
            -hw - rng.rangef(1.0, 16.0)
        } else {
            self.w as f32 + hw + rng.rangef(1.0, 16.0)
        }
    }

    fn new_group(&mut self, pod: bool, dir: f32, wy: f32, rng: &mut Rng) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        self.groups.push(Group {
            id,
            pod,
            dir,
            wy,
            plan: Plan::Travel,
            angle: rng.rangef(0.0, TAU),
            dash_in: rng.range(20, 60) as u32,
            full: 0,
            stay: u32::MAX,
            leaving: false,
            members: Vec::new(),
            cx: 0.0,
            cy: 0.0,
            spread: 0.0,
            alarm: 0.0,
        });
        id
    }

    /// How many of each the sea keeps: reef fish, fish in schools, snappers.
    fn targets(&self) -> (usize, usize, usize) {
        // tanim's count of fish for the tank's size.
        let count = (self.w as f32 * (self.ph / 2) as f32 / 260.0).clamp(2.0, 40.0);
        let f = self.set.fish;
        let snappers = if self.w >= 50 && f > 0.0 {
            1 + (count >= 20.0) as usize + (count >= 36.0) as usize
        } else {
            0
        };
        (
            (count * 0.6 * f).round() as usize,
            (count * 2.2 * f).round() as usize,
            snappers,
        )
    }

    /// The fish that are there from the start, spread across.
    pub fn populate(&mut self, rng: &mut Rng) {
        let (reef, schooled, snappers) = self.targets();
        let mut n = 0;
        while n < schooled {
            let k = self.spawn_school(schooled - n, true, rng);
            if k == 0 {
                break;
            }
            n += k;
        }
        for _ in 0..reef {
            self.spawn_reef(true, rng);
        }
        for _ in 0..snappers {
            self.spawn_snapper(true, rng);
        }
    }

    fn spawn_reef(&mut self, on: bool, rng: &mut Rng) {
        let sp = rng.pick(&art::REEF);
        let dir = rng.sign();
        let (sw, sh) = size(sp.frames[0]);
        let (top, bottom) = self.band(sh as f32 * 0.5);
        let x = self.entry(sw as f32 * 0.5, dir, on, rng);
        let y = rng.rangef(top, bottom);
        let s = self.swimmer(sp, (x, y), dir, rng);
        self.swim.push(s);
    }

    fn spawn_snapper(&mut self, on: bool, rng: &mut Rng) {
        // Groupers only in roomy seas, as in tanim.
        let roomy = self.w >= 60 && self.ph >= 40;
        let sp = if roomy && rng.chance(0.5) {
            &art::GROUPER
        } else {
            &art::BARRACUDA
        };
        let dir = rng.sign();
        let (sw, sh) = size(sp.frames[0]);
        let (top, bottom) = self.band(sh as f32 * 0.5);
        let x = self.entry(sw as f32 * 0.5, dir, on, rng);
        let y = rng.rangef(top, bottom);
        let s = self.swimmer(sp, (x, y), dir, rng);
        self.swim.push(s);
    }

    /// A school of up to `most` sardines or tetras; returns how many.
    fn spawn_school(&mut self, most: usize, on: bool, rng: &mut Rng) -> usize {
        let sp = if rng.chance(0.65) {
            &art::SARDINE
        } else {
            &art::TETRA
        };
        let n = (rng.range(8, 22) as usize).min(most);
        if n < 3 {
            return 0;
        }
        let dir = rng.sign();
        let (sw, sh) = size(sp.frames[0]);
        let (top, bottom) = self.band(sh as f32 * 0.5 + 4.0);
        let y0 = rng.rangef(top, bottom);
        let x0 = self.entry(sw as f32 * 3.0, dir, on, rng);
        let g = self.new_group(false, dir, y0, rng);
        let colors = rng.pick(sp.palettes);
        let cruise = rng.rangef(sp.cruise.0, sp.cruise.1);
        for k in 0..n {
            let (col, row) = ((k % 6) as f32, (k / 6) as f32);
            let x = x0 - dir * (col * (sw as f32 + 2.0) + rng.rangef(0.0, 2.0));
            let y = y0 + (row - 1.0) * (sh as f32 + 1.5) + rng.rangef(-0.5, 0.5);
            let mut s = self.swimmer(sp, (x, y), dir, rng);
            s.colors = colors;
            s.cruise = cruise * rng.rangef(0.95, 1.05);
            s.vx = dir * s.cruise;
            s.group = g;
            self.swim.push(s);
        }
        n
    }

    /// Something big comes by: a shark, a dolphin pod or an orca pod.
    /// Returns whether one came.
    pub fn visit(&mut self, rng: &mut Rng) -> bool {
        let here = |role: Role| self.swim.iter().any(|s| s.alive && s.sp.role == role);
        let (shark, dolphins, orcas) = (here(Role::Shark), here(Role::Dolphin), here(Role::Orca));
        // A small sea takes one big visitor at a time, except for orcas
        // coming for a shark.
        let small = self.w * (self.floor - self.surf) < 5000;
        let busy = shark || dolphins || orcas;
        let mut options: Vec<(u8, f32)> = Vec::new();
        if self.set.sharks && !shark && !(small && busy) {
            options.push((0, 4.0));
        }
        if self.set.dolphins && !dolphins && !(small && busy) {
            options.push((1, 3.0));
        }
        if self.set.orcas && !orcas && (!small || !busy || shark) {
            options.push((2, if shark { 3.0 } else { 2.0 }));
        }
        let total: f32 = options.iter().map(|o| o.1).sum();
        let mut roll = rng.f32() * total;
        let pick = options.iter().find(|o| {
            roll -= o.1;
            roll < 0.0
        });
        let Some(&(kind, _)) = pick.or(options.last()) else {
            return false;
        };
        let sp = [&art::SHARK, &art::DOLPHIN, &art::ORCA][kind as usize];
        // Big things come rarely to a small window.
        let room = self.room(sp);
        if room < 0.4 || !rng.chance(room * room) {
            return false;
        }
        let area = (self.w * (self.floor - self.surf)) as f32;
        match kind {
            0 => self.send_shark(rng),
            1 => {
                let most = (area / 1500.0).clamp(2.0, 6.0) as i32;
                self.send_pod(&art::DOLPHIN, rng.range(2, most + 1), rng);
            }
            _ => {
                let most = (area / 3500.0).clamp(2.0, 5.0) as i32;
                self.send_pod(&art::ORCA, rng.range(2, most + 1), rng);
            }
        }
        true
    }

    fn send_shark(&mut self, rng: &mut Rng) {
        let sp = &art::SHARK;
        let dir = rng.sign();
        let (sw, sh) = size(sp.frames[0]);
        let (top, bottom) = self.band(sh as f32 * 0.5);
        let x = self.entry(sw as f32 * 0.5, dir, false, rng);
        let y = rng.rangef(top, bottom);
        let mut s = self.swimmer(sp, (x, y), dir, rng);
        s.stay = rng.range(900, 2000) as u32;
        s.full = rng.range(20, 120) as u32;
        self.swim.push(s);
    }

    pub fn send_pod(&mut self, sp: &'static Species, n: i32, rng: &mut Rng) {
        let dir = rng.sign();
        let (sw, sh) = size(sp.frames[0]);
        let (top, bottom) = self.band(sh as f32 * 0.5 + 2.0);
        // Dolphins keep near the surface; orcas take the middle.
        let wy = if sp.role == Role::Dolphin {
            rng.rangef(top, top + (bottom - top) * 0.35)
        } else {
            rng.rangef(top + (bottom - top) * 0.2, top + (bottom - top) * 0.7)
        };
        let g = self.new_group(true, dir, wy, rng);
        if let Some(gi) = self.group_at(g) {
            self.groups[gi].stay = rng.range(1400, 2600) as u32;
            self.groups[gi].full = rng.range(0, 80) as u32;
        }
        let x0 = self.entry(sw as f32 * 0.5, dir, false, rng);
        let cruise = rng.rangef(sp.cruise.0, sp.cruise.1);
        for k in 0..n.max(1) {
            let k = k as f32;
            let x = x0 - dir * k * (sw as f32 * 0.75 + rng.rangef(0.0, 4.0));
            let y =
                wy + if k as i32 % 2 == 1 {
                    sh as f32 * 0.8
                } else {
                    0.0
                } + rng.rangef(-1.0, 1.0);
            let mut s = self.swimmer(sp, (x, y), dir, rng);
            s.cruise = cruise * rng.rangef(0.95, 1.05);
            s.vx = dir * s.cruise;
            s.group = g;
            s.wy = wy;
            self.swim.push(s);
        }
    }

    /// One manta glides by, or two in a roomy sea. Returns whether any came.
    pub fn send_mantas(&mut self, rng: &mut Rng) -> bool {
        let sp = &art::MANTA;
        if self
            .swim
            .iter()
            .any(|s| s.alive && s.sp.role == Role::Manta)
        {
            return false;
        }
        let room = self.room(sp);
        if room < 0.4 || !rng.chance(room * room) {
            return false;
        }
        let n = if room >= 1.0 && self.w >= 140 && rng.chance(0.4) {
            2
        } else {
            1
        };
        let dir = rng.sign();
        let (sw, sh) = size(sp.frames[0]);
        let (top, bottom) = self.band(sh as f32 * 0.5);
        for k in 0..n {
            let x =
                self.entry(sw as f32 * 0.5, dir, false, rng) - dir * k as f32 * (sw as f32 + 10.0);
            let y = rng.rangef(top, bottom);
            let s = self.swimmer(sp, (x, y), dir, rng);
            self.swim.push(s);
        }
        true
    }

    /// One tick of everything that swims.
    pub fn life(&mut self, rng: &mut Rng) {
        let hunters = self.measure();
        self.pods(rng);
        let n = self.swim.len();
        let mut steers = Vec::with_capacity(n);
        for i in 0..n {
            steers.push(self.steer(i, &hunters, rng));
        }
        let surf = self.surf as f32;
        let mut splashes = Vec::new();
        for (i, st) in steers.iter().enumerate() {
            let band = self.band(self.swim[i].hh);
            let s = &mut self.swim[i];
            s.act = st.act;
            s.leaving |= st.leave;
            s.full = s.full.max(st.rest);
            if st.puff && s.puffed == 0 {
                s.puffed = rng.range(60, 140) as u32;
            }
            s.puffed = s.puffed.saturating_sub(1);
            s.full = s.full.saturating_sub(1);
            s.stay = s.stay.saturating_sub(1);
            if s.stay == 0 {
                s.leaving = true;
            }
            if s.air {
                s.vy += GRAVITY;
            } else {
                s.vx += (st.vx - s.vx) * st.k;
                s.vy += (st.vy - s.vy) * st.k;
            }
            let cap = s.sp.burst * 1.3;
            let v = (s.vx * s.vx + s.vy * s.vy).sqrt();
            if v > cap && !s.air {
                s.vx *= cap / v;
                s.vy *= cap / v;
            }
            s.x += s.vx;
            s.y += s.vy;
            // Dolphins break the surface and fall back.
            if s.sp.role == Role::Dolphin {
                if s.act == Act::Leap && !s.air && s.y < surf {
                    s.air = true;
                    let apex = rng.rangef(0.5, 1.0) * surf;
                    s.vy = -(2.0 * GRAVITY * apex).sqrt();
                    splashes.push((s.x, 6));
                } else if s.air && s.y > surf && s.vy > 0.0 {
                    s.air = false;
                    s.act = Act::Cruise;
                    s.vy *= 0.4;
                    splashes.push((s.x, 10));
                }
            }
            let (top, bottom) = band;
            if !s.air && s.act != Act::Leap {
                s.y = s.y.clamp(top, bottom);
            } else {
                s.y = s.y.min(bottom);
            }
            if s.vx * s.dir < -0.03 {
                s.dir = -s.dir;
            }
            let v = (s.vx * s.vx + s.vy * s.vy).sqrt();
            s.anim += (0.05 + v * 0.5) * s.sp.beat;
            // Big bodies tilt less than small fish; a leaping dolphin most.
            let lean = match s.sp.role {
                Role::Dolphin if s.air || s.act == Act::Leap => 0.7,
                Role::Dolphin | Role::Snapper => 0.35,
                Role::Orca | Role::Shark | Role::Manta => 0.25,
                _ => 0.5,
            };
            let want = (s.vy / s.vx.abs().max(0.3)).clamp(-1.0, 1.0) * lean;
            s.slope += (want - s.slope) * 0.15;
            if s.group == 0 && rng.chance(0.002) {
                s.wy = rng.rangef(top, bottom);
            }
        }
        for (x, n) in splashes {
            self.splash(x, n, rng);
        }
        // Catches, one at a time: a fish only goes once.
        for (i, st) in steers.iter().enumerate() {
            let Some(j) = st.catch else { continue };
            if !self.swim[i].alive || !self.swim[j].alive {
                continue;
            }
            let (h, p) = (self.swim[i].sp.role, self.swim[j].sp.role);
            self.swim[j].alive = false;
            self.eaten += 1;
            if p == Role::Shark {
                self.sharks_taken += 1;
            }
            self.swim[i].full = digest(h, p, rng);
            if h == Role::Orca && p == Role::Shark {
                // The pod has eaten: it lingers a little, then moves on.
                let g = self.swim[i].group;
                if let Some(gi) = self.group_at(g) {
                    let gr = &mut self.groups[gi];
                    gr.full = self.swim[i].full;
                    gr.stay = gr.stay.min(400);
                    gr.plan = Plan::Travel;
                }
            }
            let q = &self.swim[j];
            let n = (q.hw * q.hh * 0.5).clamp(5.0, 40.0) as usize;
            let at = (q.x, q.y);
            let colors = [q.colors[0], q.colors[3], q.colors[2]];
            self.puff(at, colors, n, rng);
        }
        // Mantas feed: plankton drifts into their mouths.
        for i in 0..self.swim.len() {
            let s = &self.swim[i];
            if s.sp.role != Role::Manta || matches!(s.act, Act::Loop { .. }) || !rng.chance(0.4) {
                continue;
            }
            let (mx, my) = s.mouth();
            let life = rng.range(10, 22) as u16;
            let (sx, sy) = (
                mx + s.dir * rng.rangef(4.0, 14.0),
                my + rng.rangef(-5.0, 5.0),
            );
            // Aim where the mouth will be when the speck gets there.
            let (tx, ty) = (mx + s.vx * life as f32, my + s.vy * life as f32);
            self.bit(Bit {
                x: sx,
                y: sy,
                vx: (tx - sx) / life as f32,
                vy: (ty - sy) / life as f32,
                life,
                max: life,
                c: PLANKTON,
                kind: BitKind::Speck,
            });
        }
        // Reef fish blow the odd bubble, as in tanim.
        for i in 0..self.swim.len() {
            let s = &self.swim[i];
            if matches!(s.sp.role, Role::Reef | Role::Snapper) && rng.chance(0.006) {
                let (mx, my) = s.mouth();
                let x = mx + s.dir;
                self.bubble(x, my, rng);
            }
        }
        self.cull();
        self.refill(rng);
    }

    /// Measures every group and finds the hunters; returns them, with
    /// whether each is hunting right now.
    fn measure(&mut self) -> Vec<(usize, bool)> {
        let (swim, groups) = (&self.swim, &mut self.groups);
        for g in groups.iter_mut() {
            g.members.clear();
        }
        for (i, s) in swim.iter().enumerate() {
            if s.alive
                && s.group != 0
                && let Some(g) = groups.iter_mut().find(|g| g.id == s.group)
            {
                g.members.push(i);
            }
        }
        for g in groups.iter_mut() {
            let n = g.members.len().max(1) as f32;
            let (sx, sy) = g
                .members
                .iter()
                .fold((0.0, 0.0), |(a, b), &i| (a + swim[i].x, b + swim[i].y));
            g.cx = sx / n;
            g.cy = sy / n;
            g.spread = g
                .members
                .iter()
                .map(|&i| dist(swim[i].x, swim[i].y, g.cx, g.cy))
                .fold(0.0, f32::max);
        }
        let mut hunters = Vec::new();
        for (i, s) in swim.iter().enumerate() {
            if !s.alive || !hunts(s.sp.role) {
                continue;
            }
            let pod = groups.iter().find(|g| g.id == s.group && g.pod);
            let hunting = matches!(s.act, Act::Stalk { .. } | Act::Lunge { .. })
                || pod.is_some_and(|g| g.plan != Plan::Travel)
                || (s.group == 0 && s.hungry());
            hunters.push((i, hunting));
        }
        // A school closes ranks while something hunts nearby.
        for g in groups.iter_mut().filter(|g| !g.pod) {
            g.alarm *= 0.97;
            for &(j, hunting) in &hunters {
                let h = &swim[j];
                if hunting && dist(h.x, h.y, g.cx, g.cy) < g.spread + h.hw + 30.0 {
                    g.alarm = 1.0;
                }
            }
        }
        hunters
    }

    /// The pods decide what to do: travel, herd a school, rush a shark,
    /// and who dashes through the ball next.
    fn pods(&mut self, rng: &mut Rng) {
        let w = self.w as f32;
        let shark = self
            .swim
            .iter()
            .find(|s| s.alive && s.sp.role == Role::Shark && s.x > 0.0 && s.x < w)
            .map(|s| s.id);
        let surf = self.surf as f32;
        for gi in 0..self.groups.len() {
            if !self.groups[gi].pod {
                continue;
            }
            let Some(&first) = self.groups[gi].members.first() else {
                continue;
            };
            let role = self.swim[first].sp.role;
            let g = &mut self.groups[gi];
            g.full = g.full.saturating_sub(1);
            g.stay = g.stay.saturating_sub(1);
            if g.stay == 0 && !g.leaving {
                g.leaving = true;
                g.plan = Plan::Travel;
                // Off the way most of it is already facing.
                g.dir = if g.cx < w * 0.5 { -1.0 } else { 1.0 };
            }
            if g.leaving {
                continue;
            }
            let on_screen = g.cx > 0.0 && g.cx < w;
            match g.plan {
                Plan::Travel => {
                    if g.full > 0 || !on_screen {
                        continue;
                    }
                    if let (Role::Orca, Some(q)) = (role, shark) {
                        self.groups[gi].plan = Plan::Rush { quarry: q };
                        continue;
                    }
                    let (cx, cy) = (self.groups[gi].cx, self.groups[gi].cy);
                    let best = self
                        .groups
                        .iter()
                        .filter(|s| !s.pod && s.members.len() >= 4 && s.cx > 0.0 && s.cx < w)
                        .min_by(|a, b| {
                            let da = dist(a.cx, a.cy, cx, cy);
                            let db = dist(b.cx, b.cy, cx, cy);
                            da.total_cmp(&db)
                        })
                        .map(|s| s.id);
                    if let Some(school) = best {
                        self.groups[gi].plan = Plan::Herd { school };
                    }
                }
                Plan::Herd { school } => {
                    let sg = self
                        .group_at(school)
                        .filter(|&s| self.groups[s].members.len() >= 3);
                    let Some(sg) = sg else {
                        let g = &mut self.groups[gi];
                        g.plan = Plan::Travel;
                        g.full = 100;
                        continue;
                    };
                    if let (Role::Orca, Some(q)) = (role, shark)
                        && rng.chance(0.01)
                    {
                        self.groups[gi].plan = Plan::Rush { quarry: q };
                        continue;
                    }
                    // The ball is driven up toward the surface.
                    let (top, _) = self.band(4.0);
                    let s = &mut self.groups[sg];
                    s.wy = (s.wy - 0.03).max(top.max(surf + 6.0));
                    let spin = if role == Role::Orca { 0.022 } else { 0.035 };
                    let g = &mut self.groups[gi];
                    g.angle = (g.angle + spin * g.dir).rem_euclid(TAU);
                    if g.dash_in > 0 {
                        g.dash_in -= 1;
                        continue;
                    }
                    let next = if role == Role::Orca {
                        rng.range(25, 60)
                    } else {
                        rng.range(15, 40)
                    };
                    g.dash_in = next as u32;
                    // The one nearest the ball goes through it.
                    let (bx, by) = (self.groups[sg].cx, self.groups[sg].cy);
                    let dasher = self.groups[gi]
                        .members
                        .iter()
                        .copied()
                        .filter(|&m| {
                            let s = &self.swim[m];
                            s.act == Act::Cruise && !s.air
                        })
                        .min_by(|&a, &b| {
                            let da = dist(self.swim[a].x, self.swim[a].y, bx, by);
                            let db = dist(self.swim[b].x, self.swim[b].y, bx, by);
                            da.total_cmp(&db)
                        });
                    let Some(d) = dasher else { continue };
                    let prey = self.groups[sg]
                        .members
                        .iter()
                        .copied()
                        .filter(|&p| eats(&self.swim[d], &self.swim[p]))
                        .min_by(|&a, &b| {
                            let h = &self.swim[d];
                            let da = dist(self.swim[a].x, self.swim[a].y, h.x, h.y);
                            let db = dist(self.swim[b].x, self.swim[b].y, h.x, h.y);
                            da.total_cmp(&db)
                        });
                    if let Some(p) = prey {
                        let id = self.swim[p].id;
                        self.swim[d].act = Act::Lunge { prey: id, t: 0 };
                    }
                }
                Plan::Rush { quarry } => {
                    let gone = match self.find(quarry) {
                        Some(q) => {
                            let s = &self.swim[q];
                            s.x < -s.hw - 20.0 || s.x > w + s.hw + 20.0
                        }
                        None => true,
                    };
                    if gone {
                        self.groups[gi].plan = Plan::Travel;
                    }
                }
            }
        }
    }

    /// The nearest hunter that `i` should run from: its position, how far
    /// it is and from how far `i` notices it.
    fn threat(&self, i: usize, hunters: &[(usize, bool)]) -> Option<(f32, f32, f32, f32)> {
        let p = &self.swim[i];
        let mut best: Option<(f32, f32, f32, f32)> = None;
        for &(j, hunting) in hunters {
            let h = &self.swim[j];
            if !eats(h, p) {
                continue;
            }
            let sense = 10.0 + h.hw * 1.2 + p.hw;
            let reach = if hunting { sense } else { sense * 0.5 };
            let d = dist(h.x, h.y, p.x, p.y);
            if d < reach && best.is_none_or(|b| d < b.2) {
                best = Some((h.x, h.y, d, sense));
            }
        }
        best
    }

    /// Cohesion, alignment and spacing within a school or pod, as a velocity.
    fn flock(&self, i: usize, g: &Group, cruise: f32, rng: &mut Rng) -> (f32, f32) {
        let s = &self.swim[i];
        let tight = if g.pod { 0.01 } else { 0.006 + 0.03 * g.alarm };
        let mut vx = g.dir * cruise + (g.cx - s.x) * tight;
        let mut vy = (g.cy - s.y) * tight * 1.6 + (g.wy - s.y) * 0.006;
        let (sx, sy) = if g.pod {
            (s.hw * 1.6 + 2.0, s.hh * 1.6 + 1.0)
        } else {
            (s.hw * 2.0 + 1.0, s.hh * 2.0 + 0.5)
        };
        for &j in &g.members {
            if j == i {
                continue;
            }
            let o = &self.swim[j];
            let (dx, dy) = (s.x - o.x, s.y - o.y);
            if dx.abs() < sx && dy.abs() < sy {
                let side = if dy == 0.0 { rng.sign() } else { dy.signum() };
                vx += dx.signum() * (sx - dx.abs()) * 0.03;
                vy += side * (sy - dy.abs()) * 0.05;
            }
        }
        (vx, vy)
    }

    /// Chasing `prey`: closing in, then the burst, then the catch or a miss.
    fn chase(&self, i: usize, act: Act) -> Steer {
        let s = &self.swim[i];
        let (prey, t, lunging) = match act {
            Act::Stalk { prey, t } => (prey, t, false),
            Act::Lunge { prey, t } => (prey, t, true),
            _ => unreachable!(),
        };
        let target = self.find(prey).filter(|&j| eats(s, &self.swim[j]));
        let Some(j) = target else {
            return Steer::to(s.dir * s.cruise, 0.0, 0.1, Act::Cruise);
        };
        let p = &self.swim[j];
        let (mx, my) = s.mouth();
        let d = dist(mx, my, p.x, p.y);
        if lunging && d < p.hh.max(1.0) + 2.0 {
            let mut st = Steer::to(s.vx, s.vy, 0.0, Act::Cruise);
            st.catch = Some(j);
            return st;
        }
        // Aim a little ahead of where it is going.
        let (ax, ay) = (p.x + p.vx * 4.0, p.y + p.vy * 4.0);
        let from = (s.x + (mx - s.x) * 0.5, s.y);
        if lunging {
            if t > lunge_len(s.sp.role) {
                // A miss: a short rest before trying again.
                let mut st = Steer::to(s.dir * s.cruise, 0.0, 0.1, Act::Cruise);
                st.rest = 30;
                return st;
            }
            let (vx, vy) = toward(from.0, from.1, ax, ay, s.sp.burst);
            return Steer::to(vx, vy, 0.35, Act::Lunge { prey, t: t + 1 });
        }
        if t > 300 {
            let mut st = Steer::to(s.dir * s.cruise, 0.0, 0.1, Act::Cruise);
            st.rest = 60;
            return st;
        }
        let strike = s.hw * 0.6 + 8.0;
        // Orcas are faster than the sharks they run down.
        let close = match s.sp.role {
            Role::Orca => 2.8,
            Role::Shark => 2.0,
            _ => 1.8,
        };
        let (vx, vy) = toward(from.0, from.1, ax, ay, s.cruise * close);
        let next = if d < strike {
            Act::Lunge { prey, t: 0 }
        } else {
            Act::Stalk { prey, t: t + 1 }
        };
        Steer::to(vx, vy, 0.15, next)
    }

    /// The nearest prey `i` would go after within `reach`, ahead of it if
    /// `ahead`.
    fn prey_near(&self, i: usize, reach: f32, ahead: bool) -> Option<usize> {
        let s = &self.swim[i];
        let mut best: Option<(usize, f32)> = None;
        for (j, p) in self.swim.iter().enumerate() {
            if !eats(s, p) || p.x < 0.0 || p.x > self.w as f32 {
                continue;
            }
            let fwd = (p.x - s.x) * s.dir;
            if ahead && (fwd < 0.0 || (p.y - s.y).abs() > s.hh + 3.0) {
                continue;
            }
            let d = dist(s.x, s.y, p.x, p.y) + if fwd < 0.0 { 15.0 } else { 0.0 };
            if d < reach && best.is_none_or(|b| d < b.1) {
                best = Some((j, d));
            }
        }
        best.map(|b| b.0)
    }

    fn steer(&self, i: usize, hunters: &[(usize, bool)], rng: &mut Rng) -> Steer {
        let s = &self.swim[i];
        let role = s.sp.role;
        let group = (s.group != 0)
            .then(|| self.groups.iter().find(|g| g.id == s.group))
            .flatten();

        // Running for its life comes first.
        let edible = matches!(
            role,
            Role::School | Role::Reef | Role::Snapper | Role::Shark
        );
        if edible
            && !matches!(s.act, Act::Lunge { .. })
            && let Some((hx, hy, d, sense)) = self.threat(i, hunters)
        {
            if s.sp.name == art::PUFFER.name {
                // A pufferfish blows up and drifts.
                let mut st = Steer::to(s.dir * s.cruise * 0.2, 0.0, 0.1, Act::Cruise);
                st.puff = true;
                return st;
            }
            let urgency = (1.0 - d / sense).clamp(0.3, 1.0);
            let speed = s.sp.burst
                * if role == Role::Shark {
                    0.6
                } else {
                    0.5 + 0.5 * urgency
                };
            let (mut vx, mut vy) = toward(hx, hy, s.x, s.y, speed);
            if role == Role::Shark {
                // A shark makes for open water, away and out.
                vx = if s.x > hx { speed } else { -speed };
                vy *= 0.3;
            }
            if let Some(g) = group {
                // Fleeing into the middle of the school: the ball forms.
                let tight = 0.02 + 0.04 * g.alarm;
                vx += (g.cx - s.x) * tight;
                vy += (g.cy - s.y) * tight;
            }
            let mut st = Steer::to(vx, vy * 0.8, 0.3, Act::Flee);
            st.leave = role == Role::Shark;
            return st;
        }
        let calm = if s.act == Act::Flee {
            Act::Cruise
        } else {
            s.act
        };

        match role {
            Role::School => {
                let Some(g) = group else {
                    return Steer::to(s.dir * s.cruise, (s.wy - s.y) * 0.01, 0.1, Act::Cruise);
                };
                let (vx, vy) = self.flock(i, g, s.cruise, rng);
                Steer::to(vx, vy, 0.2, Act::Cruise)
            }
            Role::Reef => Steer::to(s.dir * s.cruise, (s.wy - s.y) * 0.01, 0.1, Act::Cruise),
            Role::Snapper | Role::Shark => {
                if matches!(calm, Act::Stalk { .. } | Act::Lunge { .. }) {
                    return self.chase(i, calm);
                }
                let cruise = Steer::to(s.dir * s.cruise, (s.wy - s.y) * 0.01, 0.08, Act::Cruise);
                if !s.hungry() {
                    return cruise;
                }
                let ambush = s.sp.name == art::GROUPER.name;
                let reach = match role {
                    Role::Shark => 60.0,
                    _ if ambush => 18.0,
                    _ => 40.0,
                };
                match self.prey_near(i, reach, ambush) {
                    Some(j) if ambush => self.chase(
                        i,
                        Act::Lunge {
                            prey: self.swim[j].id,
                            t: 0,
                        },
                    ),
                    Some(j) => self.chase(
                        i,
                        Act::Stalk {
                            prey: self.swim[j].id,
                            t: 0,
                        },
                    ),
                    None => cruise,
                }
            }
            Role::Dolphin | Role::Orca => self.pod_steer(i, group, calm, rng),
            Role::Manta => match calm {
                Act::Loop { t, r } => {
                    let a = TAU * t as f32 / LOOP as f32;
                    let v = TAU * r / LOOP as f32;
                    let next = if t + 1 >= LOOP {
                        Act::Cruise
                    } else {
                        Act::Loop { t: t + 1, r }
                    };
                    Steer::to(s.dir * (v * a.cos() + 0.04), -v * a.sin(), 1.0, next)
                }
                _ => {
                    let bob = ((self.t as f32) * 0.02 + s.phase).sin() * 0.03;
                    let mut act = Act::Cruise;
                    // Room overhead for the whole of it, turned on end.
                    let r = ((s.y - s.hw - self.surf as f32 - 2.0) * 0.5).min(10.0);
                    let inside = s.x > s.hw && s.x < self.w as f32 - s.hw;
                    if r >= 5.0 && inside && !s.leaving && rng.chance(0.0015) {
                        act = Act::Loop { t: 0, r };
                    }
                    Steer::to(s.dir * s.cruise, (s.wy - s.y) * 0.004 + bob, 0.03, act)
                }
            },
        }
    }

    fn pod_steer(&self, i: usize, group: Option<&Group>, act: Act, rng: &mut Rng) -> Steer {
        let s = &self.swim[i];
        if matches!(act, Act::Stalk { .. } | Act::Lunge { .. }) {
            return self.chase(i, act);
        }
        if s.air {
            return Steer::to(s.vx, s.vy, 0.0, Act::Leap);
        }
        if act == Act::Leap {
            // Up at the surface, fast and steep.
            return Steer::to(s.dir * s.sp.burst * 0.7, -s.sp.burst * 0.9, 0.2, Act::Leap);
        }
        let Some(g) = group else {
            return Steer::to(s.dir * s.cruise, 0.0, 0.05, Act::Cruise);
        };
        let k = if s.sp.role == Role::Orca { 0.06 } else { 0.1 };
        // Dolphins near the surface leap now and then.
        let surf = self.surf as f32;
        let leap = s.sp.role == Role::Dolphin
            && self.surf >= 6
            && s.y - s.hh < surf + 8.0
            && s.x > 0.0
            && s.x < self.w as f32
            && rng.chance(0.012);
        let next = if leap { Act::Leap } else { Act::Cruise };
        if g.leaving {
            let (vx, vy) = self.flock(i, g, s.cruise * 1.3, rng);
            return Steer::to(vx, vy, k, next);
        }
        match g.plan {
            Plan::Travel => {
                let (vx, vy) = self.flock(i, g, s.cruise, rng);
                Steer::to(vx, vy, k, next)
            }
            Plan::Herd { school } => {
                let Some(sg) = self.groups.iter().find(|x| x.id == school) else {
                    return Steer::to(g.dir * s.cruise, 0.0, k, next);
                };
                // A place on a ring around the ball, turning with the pod.
                let slot = g.members.iter().position(|&m| m == i).unwrap_or(0);
                let a = g.angle + TAU * slot as f32 / g.members.len().max(1) as f32;
                let rx = (sg.spread + s.hw + 6.0).max(16.0);
                let ry = (rx * 0.45).max(s.hh + 4.0);
                let (tx, ty) = (sg.cx + rx * a.cos(), sg.cy + ry * a.sin());
                // Mostly side to side: they swing round the ball, not up and down.
                let most = s.cruise * 2.5;
                let vx = ((tx - s.x) * 0.08).clamp(-most, most);
                let vy = ((ty - s.y) * 0.06).clamp(-most * 0.4, most * 0.4);
                Steer::to(vx, vy, k * 1.5, next)
            }
            Plan::Rush { quarry } => self.chase(i, Act::Stalk { prey: quarry, t: 0 }),
        }
    }

    /// Takes away the eaten and those gone off the sides, and groups left empty.
    fn cull(&mut self) {
        let w = self.w as f32;
        let groups = &self.groups;
        let busy = |s: &Swimmer| {
            groups
                .iter()
                .any(|g| g.id == s.group && g.pod && g.plan != Plan::Travel && !g.leaving)
        };
        self.swim.retain(|s| {
            let left = s.x + s.hw < -4.0 && s.vx <= 0.0;
            let right = s.x - s.hw > w + 4.0 && s.vx >= 0.0;
            let lost = s.x < -2.0 * w - 100.0 || s.x > 3.0 * w + 100.0;
            s.alive && !lost && (!(left || right) || busy(s))
        });
        let swim = &self.swim;
        self.groups.retain(|g| swim.iter().any(|s| s.group == g.id));
    }

    /// New fish swim in from the sides to take the place of the eaten.
    fn refill(&mut self, rng: &mut Rng) {
        let (reef_t, school_t, snap_t) = self.targets();
        let count = |role: Role| self.swim.iter().filter(|s| s.sp.role == role).count();
        let (reef, schooled, snappers) =
            (count(Role::Reef), count(Role::School), count(Role::Snapper));
        if reef < reef_t && rng.chance(0.03) {
            self.spawn_reef(false, rng);
        }
        // What is left of a school after a raid no longer counts as one.
        let schools = self
            .groups
            .iter()
            .filter(|g| !g.pod && g.members.len() >= 5)
            .count();
        if schooled + 4 <= school_t && schools < 1 + school_t / 12 && rng.chance(0.02) {
            self.spawn_school(school_t - schooled, false, rng);
        }
        if snappers < snap_t && rng.chance(0.004) {
            self.spawn_snapper(false, rng);
        }
    }

    /// Everything that swims, the big ones in front.
    pub fn draw_life(&mut self, t: f32) {
        const ORDER: [Role; 7] = [
            Role::Manta,
            Role::School,
            Role::Reef,
            Role::Snapper,
            Role::Shark,
            Role::Dolphin,
            Role::Orca,
        ];
        for role in ORDER {
            for i in 0..self.swim.len() {
                let s = &self.swim[i];
                if s.sp.role != role {
                    continue;
                }
                let frames = s.sp.frames;
                let frame = if s.puffed > 0 {
                    PUFFED
                } else {
                    frames[((s.anim + s.phase) as usize) % frames.len()]
                };
                let bob = match (role, s.act) {
                    (Role::Reef, Act::Cruise) => (t * 0.07 + s.phase).sin() * 0.8,
                    (Role::School, Act::Cruise) => (t * 0.07 + s.phase).sin() * 0.4,
                    _ => 0.0,
                };
                let at = (s.x, s.y + bob);
                let (dir, slope, act) = (s.dir, s.slope, s.act);
                let c = s.colors;
                let paint = move |ch: u8| match ch {
                    b'B' => c[0],
                    b'D' => c[1],
                    b'L' => c[2],
                    b'S' => c[3],
                    b'F' => c[4],
                    b'w' => EYE_WHITE,
                    _ => EYE,
                };
                match act {
                    Act::Loop { t, .. } => {
                        let a = TAU * t as f32 / LOOP as f32;
                        self.sprite_turned(frame, at, dir, a, paint);
                    }
                    _ => self.sprite(frame, at, dir, slope, paint),
                }
            }
        }
    }
}
