//! How passers-by, plants, props and bubbles meet the creatures.
//!
//! Everything here runs after the ecology and the motion of the step, and
//! only steers entities (positions, velocities, deaths); the engine moves
//! them. What has to survive between steps lives in [`Task`], on the entity
//! itself, so nothing is left to forget when `rebuild` clears the scene: ids
//! that no longer resolve are simply ignored.

use super::assets::{DEFENDED, FILTER, FISHING, Lane, NET, PLUNGE, PREDATOR, Swim};
use super::biome::{Food, between, hash};
use super::library::library;
use super::*;

/// Where a plunging bird is in its dive.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Plunge {
    /// Flying its row, looking for a fish.
    Fly,
    /// Going down after the fish `Task::Bird::target`.
    Dive,
    /// Back up to its row.
    Climb,
}

/// Where an angler is.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Angle {
    /// Sailing to its spot.
    Sail,
    /// Paying the line out.
    Lower,
    /// Waiting for a bite.
    Wait,
    /// Reeling in, with a fish or without.
    Reel,
}

/// What a visitor, plant or prop is up to.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub(super) enum Task {
    /// The usual: passing by, swaying, sitting there.
    #[default]
    Idle,
    /// A diving bird. `home` is its flying row, `left` the dives it may
    /// still make, `timer` the pause before the next one (or the dive's
    /// time limit).
    Bird {
        stage: Plunge,
        home: f32,
        left: u8,
        timer: f32,
        target: u32,
    },
    /// A visitor or a dolphin that bites: `cool` seconds to the next bite,
    /// `home` the row it keeps.
    Hunter { cool: f32, home: f32 },
    /// A trawler with its net (an entity id).
    Trawl { net: u32 },
    /// The net behind boat `boat`.
    Net { boat: u32 },
    /// A fish riding in a net, or on a line, at an offset from it.
    Caught { holder: u32, dx: f32, dy: f32 },
    /// A fishing boat: where it is, its clock, the speed to resume, its
    /// line (an id) and how deep the line goes.
    Angler {
        stage: Angle,
        timer: f32,
        vx: f32,
        line: u32,
        depth: f32,
    },
    /// The line of boat `boat`; its `h` is its length plus the hook.
    Line { boat: u32 },
    /// Nothing more to do (an angler that has put its line away).
    Done,
    /// A plant swaying faster for `left` seconds; `fps` is its own rate.
    Flutter { left: f32, fps: f32 },
    /// A plant that has just settled and ignores touches for a while.
    Calm(f32),
    /// A bubble that may pop on whatever it meets.
    Rising,
    /// A bubble that will not pop (it was born inside a creature).
    Safe,
    /// A crawler running from a floor visitor for `left` seconds.
    Scurry(f32),
}

/// The encounters' own state.
#[derive(Default)]
pub(super) struct Enc {
    /// The `interactions` option.
    pub(super) on: bool,
    /// Steps taken, to spread the dearer checks over time.
    tick: u32,
    /// Scratch lists of entity indices, reused every step.
    actors: Vec<u32>,
    plants: Vec<u32>,
    movers: Vec<u32>,
    drifters: Vec<u32>,
    bubbles: Vec<u32>,
    ties: Vec<u32>,
    /// Ids of the nets, lines and fishing and trawling boats alive now.
    live: Vec<u32>,
}

impl Enc {
    pub(super) fn from_opts(o: &crate::scene::Opts) -> Enc {
        Enc {
            on: o.b("interactions"),
            ..Enc::default()
        }
    }
}

/// Whether the boxes of `a` and `b` overlap on the screen (depth ignored:
/// creatures and passers-by swim at their own depths).
fn touch(a: &Ent, b: &Ent) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h
}

/// The middle of an entity's box.
fn mid(e: &Ent) -> (f32, f32) {
    (e.x + e.w / 2.0, e.y + e.h / 2.0)
}

/// Whether the point is inside the box of `e`, grown by `r` cells.
fn within(e: &Ent, x: f32, y: f32, r: f32) -> bool {
    x >= e.x - r && x < e.x + e.w + r && y >= e.y - r && y < e.y + e.h + r
}

/// The cells a thing covers: the library's figure, or its sprite's.
fn bulk(e: &Ent) -> u16 {
    if e.size > 1 {
        e.size
    } else {
        sprites()
            .get(e.spr as usize)
            .map_or(1, |s| s.frame(0).len().min(u16::MAX as usize) as u16)
    }
}

/// Something small enough, and loose enough, to be taken: a creature of the
/// main layer that is not defended and not already in somebody's grip.
fn takeable(e: &Ent, max: u16) -> bool {
    matches!(e.kind, Kind::Fish | Kind::Critter)
        && !e.dead
        && e.layer == 0
        && e.size <= max
        && e.traits & DEFENDED == 0
        && !matches!(e.task, Task::Caught { .. })
}

/// The classic big fish (a passer-by with no entry in the library).
fn is_big_fish(e: &Ent) -> bool {
    e.kind == Kind::Visitor && e.who == NOBODY && (e.spr == SP_BIG_FISH || e.spr == SP_BIG_FISH + 1)
}

/// The lane of a data visitor.
fn lane(e: &Ent) -> Option<Lane> {
    if e.kind != Kind::Visitor {
        return None;
    }
    library().visitors.get(e.who as usize).map(|v| v.def.lane)
}

impl Aquarium {
    /// Once per step, after the ecology.
    pub(super) fn encounters(&mut self, dt: f32) {
        if !self.world.as_ref().is_some_and(|w| w.enc.on) {
            return;
        }
        // The state is taken out so the helpers can borrow `self` freely.
        let Some(mut enc) = self.world.as_mut().map(|w| std::mem::take(&mut w.enc)) else {
            return;
        };
        enc.on = true;
        self.encounter_step(&mut enc, dt);
        if let Some(w) = self.world.as_mut() {
            w.enc = enc;
        }
    }

    fn encounter_step(&mut self, enc: &mut Enc, dt: f32) {
        enc.tick = enc.tick.wrapping_add(1);
        for v in [
            &mut enc.actors,
            &mut enc.plants,
            &mut enc.movers,
            &mut enc.drifters,
            &mut enc.bubbles,
            &mut enc.ties,
            &mut enc.live,
        ] {
            v.clear();
        }
        let top = self.water_top() as f32;
        for (i, e) in self.ents.iter().enumerate() {
            if e.dead {
                continue;
            }
            let i = i as u32;
            match e.kind {
                Kind::Visitor => {
                    if e.who != NOBODY || is_big_fish(e) {
                        enc.actors.push(i);
                    }
                    if e.traits & (NET | FISHING) != 0 {
                        enc.live.push(e.id);
                    }
                    if matches!(lane(e), Some(Lane::Deep | Lane::Floor)) || is_big_fish(e) {
                        enc.movers.push(i);
                    }
                }
                Kind::Dolphin => enc.actors.push(i),
                Kind::Shark => enc.movers.push(i),
                Kind::Fish | Kind::Critter => {
                    if let Task::Caught { .. } = e.task {
                        enc.ties.push(i);
                    } else if e.layer == 0 {
                        enc.movers.push(i);
                        if matches!(e.task, Task::Scurry(_)) {
                            enc.actors.push(i);
                        }
                        if matches!(
                            e.motion,
                            Motion::Swim {
                                style: Swim::Drift,
                                ..
                            }
                        ) && e.size <= 20
                        {
                            enc.drifters.push(i);
                        }
                    }
                }
                Kind::Seaweed => enc.plants.push(i),
                Kind::Bubble => {
                    let pops = hash(e.id as u64) % 100 < 30;
                    if e.task != Task::Safe && (pops || e.z == Z_CASTLE - 1) {
                        enc.bubbles.push(i);
                    }
                }
                Kind::Splat if matches!(e.task, Task::Net { .. } | Task::Line { .. }) => {
                    enc.live.push(e.id);
                    enc.actors.push(i);
                }
                _ => {}
            }
        }

        for k in 0..enc.actors.len() {
            let i = enc.actors[k] as usize;
            let e = self.ents[i];
            if e.dead {
                continue;
            }
            match e.kind {
                Kind::Visitor => {
                    if e.traits & PLUNGE != 0 {
                        self.bird(i, dt);
                    }
                    if e.traits & PREDATOR != 0 || is_big_fish(&e) {
                        self.hunt(i, dt, top);
                    }
                    if e.traits & FILTER != 0 {
                        self.strain(i);
                    }
                    if e.traits & NET != 0 {
                        self.trawl(i);
                    }
                    if e.traits & FISHING != 0 {
                        self.angle(i, dt);
                    }
                    if lane(&e) == Some(Lane::Floor) {
                        self.scare_crawlers(i);
                    }
                }
                Kind::Dolphin => self.hunt(i, dt, top),
                Kind::Critter => self.settle(i, dt),
                Kind::Splat => match e.task {
                    Task::Net { boat } => self.haul(enc, i, boat),
                    Task::Line { boat } if !enc.live.contains(&boat) => self.ents[i].dead = true,
                    _ => {}
                },
                _ => {}
            }
        }

        // Whatever rode with a net or a line that is gone goes with it.
        for &i in &enc.ties {
            if let Task::Caught { holder, .. } = self.ents[i as usize].task
                && !enc.live.contains(&holder)
            {
                self.ents[i as usize].dead = true;
            }
        }

        self.sway_plants(enc, dt);
        self.pop_bubbles(enc, dt, top);
    }

    /// A pelican, gull or gannet: dives on a fish near the surface, then
    /// climbs back to its row and flies on.
    fn bird(&mut self, i: usize, dt: f32) {
        let mut e = self.ents[i];
        let (sw, sh) = (self.w as f32, self.h as f32);
        let top = self.water_top() as f32;
        let (mut stage, home, mut left, mut timer, mut target) = match e.task {
            Task::Idle => (Plunge::Fly, e.y, 2, between(&mut self.rng, 0.0, 2.0), 0),
            Task::Bird {
                stage,
                home,
                left,
                timer,
                target,
            } => (stage, home, left, timer, target),
            _ => return,
        };
        let gone = e.x >= sw || e.x + e.w <= 0.0;
        match stage {
            Plunge::Fly => {
                timer -= dt;
                let room = if e.vx > 0.0 { sw - e.x - e.w } else { e.x };
                if timer <= 0.0 && left > 0 && self.surface() && !gone && room >= 28.0 {
                    let bottom = e.y + e.h;
                    let mut best = None;
                    for f in &self.ents {
                        if !takeable(f, 40) || f.kind != Kind::Fish || f.y < top || f.y >= top + 8.0
                        {
                            continue;
                        }
                        // Where bird and fish will be when it gets there.
                        let t = ((f.y - bottom) / 15.0).max(0.0) + 0.1;
                        let fx = f.x + f.w / 2.0 + f.vx * t;
                        let bx = e.x + e.vx * t;
                        if fx >= bx + e.w * 0.25 && fx <= bx + e.w * 0.75 {
                            best = Some(f.id);
                            break;
                        }
                    }
                    if let Some(id) = best {
                        target = id;
                        stage = Plunge::Dive;
                        timer = 3.0;
                        left -= 1;
                        e.vy = 15.0;
                    }
                }
            }
            Plunge::Dive => {
                timer -= dt;
                let bottom = e.y + e.h;
                let prey = self
                    .ents
                    .iter()
                    .position(|f| f.id == target && !f.dead && f.kind == Kind::Fish);
                let mut climb = timer <= 0.0 || bottom >= sh - 1.0 || prey.is_none();
                if let Some(j) = prey {
                    let f = self.ents[j];
                    let fx = f.x + f.w / 2.0;
                    if bottom > f.y && fx >= e.x && fx < e.x + e.w {
                        self.ents[j].dead = true;
                        // A splash at the surface.
                        for _ in 0..self.rng.usize(3..=5) {
                            let x = fx + self.rng.i32(-3..=3) as f32;
                            let y = top + self.rng.usize(0..2) as f32;
                            self.add_bubble_at(x, y, 1);
                        }
                        climb = true;
                    } else if bottom > f.y + f.h + 1.0 {
                        climb = true;
                    }
                }
                if climb {
                    stage = Plunge::Climb;
                    e.vy = -12.0;
                }
            }
            Plunge::Climb => {
                if e.y <= home {
                    e.y = home;
                    e.vy = 0.0;
                    stage = Plunge::Fly;
                    timer = between(&mut self.rng, 5.0, 9.0);
                }
            }
        }
        // Leaving the screen under water would strand it: back to its row.
        if gone && stage != Plunge::Fly {
            e.y = home;
            e.vy = 0.0;
            stage = Plunge::Fly;
            left = 0;
        }
        e.task = Task::Bird {
            stage,
            home,
            left,
            timer,
            target,
        };
        self.ents[i] = e;
    }

    /// A predator visitor or a dolphin: bites a small creature it overlaps,
    /// and swerves a few rows toward close prey.
    fn hunt(&mut self, i: usize, dt: f32, top: f32) {
        let mut e = self.ents[i];
        let dolphin = e.kind == Kind::Dolphin;
        let (mut cool, home) = match e.task {
            Task::Idle => (between(&mut self.rng, 1.0, 3.0), e.y),
            Task::Hunter { cool, home } => (cool, home),
            _ => return,
        };
        cool -= dt;
        // Dolphins take small fish; the others anything a quarter of their size.
        let max = if dolphin { 40 } else { bulk(&e) / 4 };
        let ahead = e.vx.signum();
        let (cx, cy) = mid(&e);
        let mut bite = None;
        // The nearest prey ahead: its distance and row.
        let mut near: Option<(f32, f32)> = None;
        for (j, f) in self.ents.iter().enumerate() {
            if j == i || !takeable(f, max) || (dolphin && f.kind != Kind::Fish) || f.y < top {
                continue;
            }
            let (fx, fy) = mid(f);
            if cool <= 0.0 && within(&e, fx, fy, 0.0) {
                bite = Some(j);
                break;
            }
            let d = (fx - cx) * ahead;
            if d > 0.0
                && d < e.w / 2.0 + 22.0
                && (fy - cy).abs() < 7.0
                && near.is_none_or(|(nd, _)| d < nd)
            {
                near = Some((d, fy));
            }
        }
        if let Some(j) = bite {
            let f = self.ents[j];
            self.ents[j].dead = true;
            let (fx, fy) = mid(&f);
            if dolphin {
                for _ in 0..self.rng.usize(2..=3) {
                    let x = fx + self.rng.i32(-2..=2) as f32;
                    self.add_bubble_at(x, fy, f.z - 1);
                }
                cool = 5.0;
            } else {
                for _ in 0..self.rng.usize(2..=4) {
                    let x = fx + self.rng.i32(-2..=2) as f32;
                    self.add_food(x, fy, Food::Scrap);
                }
                cool = between(&mut self.rng, 3.0, 5.0);
            }
        }
        // Lanes stay put; only the open-water swimmers swerve.
        if is_big_fish(&e) || lane(&e) == Some(Lane::Deep) {
            let want = match near {
                Some((_, fy)) => (fy - e.h / 2.0).clamp(home - 4.0, home + 4.0),
                None => home,
            }
            .clamp(top, (self.h as f32 - e.h).max(top));
            let d = want - e.y;
            if d.abs() < 0.15 && near.is_none() {
                e.y = want;
                e.vy = 0.0;
            } else {
                // Smoothly: speed follows the distance, within three rows a second.
                e.vy = (d * 1.5).clamp(-3.0, 3.0);
            }
        }
        e.task = Task::Hunter { cool, home };
        self.ents[i] = e;
    }

    /// A filter feeder eats the plankton it passes through.
    fn strain(&mut self, i: usize) {
        let e = self.ents[i];
        for f in &mut self.ents {
            if f.kind == Kind::Food && f.who == Food::Plankton as u16 && !f.dead {
                let (fx, fy) = mid(f);
                if within(&e, fx, fy, 1.0) {
                    f.dead = true;
                }
            }
        }
    }

    /// A trawler: puts its net out behind and below the stern, once.
    fn trawl(&mut self, i: usize) {
        let e = self.ents[i];
        if e.task != Task::Idle {
            return;
        }
        let spr = library().fx.net + Self::facing_left(&e) as u16;
        let mut n = Ent::new(Kind::Splat, spr, 0.0, 0.0, 1, W);
        n.vx = e.vx;
        n.fps = 2.0;
        n.die_offscreen = true;
        // Out of sight at first, like the boat.
        n.seen = false;
        n.task = Task::Net { boat: e.id };
        Self::place_net(&e, &mut n);
        let k = self.spawn(n);
        self.ents[i].task = Task::Trawl {
            net: self.ents[k].id,
        };
    }

    /// Where a boat's net hangs: behind the stern, from the row under the hull.
    fn place_net(boat: &Ent, net: &mut Ent) {
        net.x = if Self::facing_left(boat) {
            boat.x + boat.w - 2.0
        } else {
            boat.x + 2.0 - net.w
        };
        net.y = boat.y + boat.h;
    }

    /// The net follows its boat (and carries on its way once the boat is
    /// gone), catches what it touches, and keeps its catch inside.
    fn haul(&mut self, enc: &Enc, i: usize, boat: u32) {
        let mut n = self.ents[i];
        if let Some(b) = self.ents.iter().find(|b| b.id == boat && !b.dead) {
            Self::place_net(b, &mut n);
            n.vx = b.vx;
        }
        self.ents[i] = n;
        let mut held = 0;
        for &t in &enc.ties {
            if let Task::Caught { holder, dx, dy } = self.ents[t as usize].task
                && holder == n.id
            {
                held += 1;
                let f = &mut self.ents[t as usize];
                f.x = n.x + dx;
                f.y = n.y + dy;
            }
        }
        // The mesh is the lower part of the sprite.
        let (x0, x1) = (n.x + 1.0, n.x + n.w - 3.0);
        let (y0, y1) = (n.y + 2.0, n.y + n.h);
        for f in &mut self.ents {
            if held >= 6 {
                break;
            }
            if !takeable(f, 60) || f.kind != Kind::Fish {
                continue;
            }
            let (fx, fy) = mid(f);
            if fx >= x0 && fx < x1 && fy >= y0 && fy < y1 {
                f.task = Task::Caught {
                    holder: n.id,
                    dx: (f.x - n.x).clamp(1.0, (n.w - 3.0 - f.w).max(1.0)),
                    dy: (f.y - n.y).clamp(2.0, (n.h - f.h).max(2.0)),
                };
                f.motion = Motion::Linear;
                f.vx = 0.0;
                f.vy = 0.0;
                f.physical = false;
                held += 1;
            }
        }
    }

    /// Where the rod points: the far end of the top row of the boat.
    fn rod_tip(e: &Ent) -> (f32, f32) {
        let Some(s) = sprites().get(e.spr as usize) else {
            return (e.x, e.y);
        };
        let cells = s.frame(0);
        let top = cells.iter().map(|c| c.dy).min().unwrap_or(0);
        let row = cells.iter().filter(|c| c.dy == top && c.ch != ' ');
        let dx = if Self::facing_left(e) {
            row.map(|c| c.dx).min()
        } else {
            row.map(|c| c.dx).max()
        };
        // One cell out, so the line hangs clear of the hull.
        let out = if Self::facing_left(e) { -1.0 } else { 1.0 };
        (e.x + dx.unwrap_or(0) as f32 + out, e.y + top as f32)
    }

    /// An angler: stops about a third of the way across, lowers a line,
    /// reels in what bites (or nothing), and sails on.
    fn angle(&mut self, i: usize, dt: f32) {
        let mut e = self.ents[i];
        let (sw, sh) = (self.w as f32, self.h as f32);
        let top = self.water_top() as f32;
        let (mut stage, mut timer, vx, mut line, mut depth) = match e.task {
            Task::Idle => (Angle::Sail, 0.0, e.vx, 0, 0.0),
            Task::Angler {
                stage,
                timer,
                vx,
                line,
                depth,
            } => (stage, timer, vx, line, depth),
            _ => return,
        };
        let (cx, _) = mid(&e);
        let (tx, ty) = Self::rod_tip(&e);
        if stage == Angle::Sail {
            let progress = if Self::facing_left(&e) {
                1.0 - cx / sw
            } else {
                cx / sw
            };
            if progress >= 0.33 && cx > 0.0 && cx < sw && self.surface() {
                e.vx = 0.0;
                let mut l = Ent::new(Kind::Splat, library().fx.line, tx, ty + 1.0, 1, W);
                l.h = 1.0;
                l.task = Task::Line { boat: e.id };
                let k = self.spawn(l);
                line = self.ents[k].id;
                // Not past the floor.
                let room = (sh - ty - 3.0).clamp(1.0, 15.0);
                depth = between(&mut self.rng, 6.0, 15.0).min(room);
                timer = between(&mut self.rng, 20.0, 40.0);
                stage = Angle::Lower;
            }
        } else if let Some(k) = self.ents.iter().position(|l| l.id == line && !l.dead) {
            let mut l = self.ents[k];
            l.x = tx;
            l.y = ty + 1.0;
            let mut landed = false;
            match stage {
                Angle::Lower => {
                    l.h = (l.h + 8.0 * dt).min(depth + 1.0);
                    if l.h >= depth + 1.0 {
                        stage = Angle::Wait;
                    }
                }
                Angle::Wait => {
                    timer -= dt;
                    let (hx, hy) = (l.x, l.y + (l.h - 1.0).floor());
                    let bite = self.ents.iter_mut().find(|f| {
                        takeable(f, 40)
                            && f.kind == Kind::Fish
                            && hy >= f.y
                            && hy < f.y + f.h
                            && hx >= f.x - 1.0
                            && hx <= f.x + f.w
                    });
                    if let Some(f) = bite
                        && self.rng.f32() < 3.0 * dt
                    {
                        f.task = Task::Caught {
                            holder: line,
                            dx: 0.0,
                            dy: 0.0,
                        };
                        f.motion = Motion::Linear;
                        f.vx = 0.0;
                        f.vy = 0.0;
                        f.physical = false;
                        timer = 0.0;
                    }
                    if timer <= 0.0 {
                        stage = Angle::Reel;
                    }
                }
                Angle::Reel => {
                    l.h -= 10.0 * dt;
                    landed = l.h <= 1.0;
                }
                Angle::Sail => {}
            }
            l.frame = (l.h - 1.0).floor().clamp(0.0, 15.0);
            if stage == Angle::Reel {
                // What bit rides the hook up and is landed at the surface.
                let (hx, hy) = (l.x, l.y + l.frame);
                for f in &mut self.ents {
                    if let Task::Caught { holder, .. } = f.task
                        && holder == line
                    {
                        f.x = hx - f.w / 2.0;
                        f.y = hy - f.h / 2.0;
                        f.dead |= hy < top || landed;
                    }
                }
            }
            if landed {
                l.dead = true;
                e.vx = vx;
                e.task = Task::Done;
            }
            self.ents[k] = l;
        } else {
            // The line is gone (cut short by a rebuild): sail on.
            e.vx = vx;
            e.task = Task::Done;
        }
        if e.task != Task::Done {
            e.task = Task::Angler {
                stage,
                timer,
                vx,
                line,
                depth,
            };
        }
        self.ents[i] = e;
    }

    /// A rover on the floor: crawlers a few cells ahead of it turn away.
    fn scare_crawlers(&mut self, i: usize) {
        let e = self.ents[i];
        let ahead = e.vx.signum();
        let (cx, _) = mid(&e);
        for c in &mut self.ents {
            if c.kind != Kind::Critter
                || c.dead
                || c.layer != 0
                || !matches!(
                    c.motion,
                    Motion::Swim {
                        style: Swim::Crawl,
                        ..
                    }
                )
                || c.task != Task::Idle
                || c.y + c.h <= e.y
            {
                continue;
            }
            let d = (mid(c).0 - cx) * ahead;
            if d > 0.0 && d < e.w / 2.0 + 6.0 {
                let left = ahead < 0.0;
                c.vx = if left { -c.speed.abs() } else { c.speed.abs() } * 1.8;
                Self::face(c, left);
                if let Motion::Swim { timer, .. } = &mut c.motion
                    && *timer <= 0.0
                {
                    // Even a resting crab runs.
                    *timer = 2.0;
                }
                c.task = Task::Scurry(2.5);
            }
        }
    }

    /// A crawler that scurried settles back to its own pace.
    fn settle(&mut self, i: usize, dt: f32) {
        let c = &mut self.ents[i];
        if let Task::Scurry(left) = c.task {
            if left > dt {
                c.task = Task::Scurry(left - dt);
            } else {
                c.vx = c.vx.signum() * c.speed.abs();
                c.task = Task::Idle;
            }
        }
    }

    /// Plants sway faster for a moment when something swims through them.
    fn sway_plants(&mut self, enc: &Enc, dt: f32) {
        let check = enc.tick.is_multiple_of(2);
        for &p in &enc.plants {
            let mut e = self.ents[p as usize];
            match e.task {
                Task::Flutter { left, fps } => {
                    if left > dt {
                        e.task = Task::Flutter {
                            left: left - dt,
                            fps,
                        };
                    } else {
                        e.fps = fps;
                        e.task = Task::Calm(2.0);
                    }
                }
                Task::Calm(left) => {
                    e.task = if left > dt {
                        Task::Calm(left - dt)
                    } else {
                        Task::Idle
                    };
                }
                Task::Idle if check && e.fps > 0.0 => {
                    let touched = enc.movers.iter().find_map(|&m| {
                        let m = &self.ents[m as usize];
                        // Big things sway them longer.
                        touch(&e, m).then_some(if bulk(m) >= 60 || m.kind == Kind::Visitor {
                            2.5
                        } else {
                            1.2
                        })
                    });
                    if let Some(left) = touched {
                        e.task = Task::Flutter { left, fps: e.fps };
                        e.fps *= 3.0;
                    }
                }
                _ => {}
            }
            self.ents[p as usize] = e;
        }
    }

    /// Bubbles that rise into a creature pop (some of them, decided by the
    /// bubble's id), and a prop's stream nudges small drifters up.
    fn pop_bubbles(&mut self, enc: &Enc, dt: f32, top: f32) {
        for &b in &enc.bubbles {
            let mut e = self.ents[b as usize];
            if e.z == Z_CASTLE - 1 {
                for &d in &enc.drifters {
                    let c = &mut self.ents[d as usize];
                    if touch(&e, c) {
                        let up = 2.0 * dt;
                        c.y = (c.y - up).max(top);
                        if let Motion::Swim { base, .. } = &mut c.motion {
                            *base = (*base - up).max(top);
                        }
                    }
                }
            }
            if hash(e.id as u64) % 100 >= 30 {
                continue;
            }
            let hit = enc
                .movers
                .iter()
                .any(|&m| touch(&e, &self.ents[m as usize]));
            match (e.task, hit) {
                // Born in somebody's mouth: it was never rising into it.
                (Task::Idle, true) => e.task = Task::Safe,
                (Task::Idle, false) => e.task = Task::Rising,
                (Task::Rising, true) => e.dead = true,
                _ => {}
            }
            self.ents[b as usize] = e;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::assets::Traits;
    use super::*;

    fn scene(opts: &[(&str, toml::Value)], f: impl FnOnce(&Ctx, &mut Aquarium)) {
        let theme = crate::theme::Theme::classic();
        let mut t = toml::Table::new();
        for (k, v) in opts {
            t.insert(k.to_string(), v.clone());
        }
        let opts = crate::scene::Opts::new(INFO.options, t);
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 5,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut a = Aquarium::new(&ctx);
        a.retheme(&ctx);
        f(&ctx, &mut a);
    }

    fn s(v: &str) -> toml::Value {
        toml::Value::String(v.into())
    }

    /// A sea of the given size with only the waves in it.
    fn empty(ctx: &Ctx, a: &mut Aquarium, w: u16, h: u16) {
        a.resize(ctx, w, h);
        a.ents.retain(|e| matches!(e.kind, Kind::Water(_)));
    }

    /// A small fish that stays where it is.
    fn fish(a: &mut Aquarium, x: f32, y: f32) -> u32 {
        let sp = &library().species[0];
        let mut e = Ent::new(Kind::Fish, sp.spr, x, y, 5, W);
        e.size = 10;
        let k = a.spawn(e);
        a.ents[k].id
    }

    /// A passer-by by name with the given traits, facing right (or left) at `x`.
    fn visit(a: &mut Aquarium, name: &str, traits: Traits, x: f32, left: bool) -> u32 {
        let i = library().visitor_named(name).unwrap();
        a.add_visitor(i as u16);
        let e = a.ents.last_mut().unwrap();
        Aquarium::face(e, left);
        e.vx = if left { -e.vx.abs() } else { e.vx.abs() };
        e.x = x;
        e.traits = traits;
        e.id
    }

    fn get(a: &Aquarium, id: u32) -> Option<Ent> {
        a.ents.iter().find(|e| e.id == id && !e.dead).copied()
    }

    fn named(a: &Aquarium, name: &str) -> usize {
        let i = library().visitor_named(name).unwrap();
        a.ents
            .iter()
            .filter(|e| e.kind == Kind::Visitor && e.who == i as u16)
            .count()
    }

    #[test]
    fn a_bird_plunges_catches_and_flies_on() {
        scene(
            &[("biome", s("ocean")), ("visitors", s("pelican"))],
            |ctx, a| {
                empty(ctx, a, 100, 30);
                let f = fish(a, 50.0, 11.0);
                let b = visit(a, "pelican", assets::PLUNGE, 0.0, false);
                let home = get(a, b).unwrap().y;
                let (mut deepest, mut last) = (0.0f32, get(a, b).unwrap());
                for _ in 0..2400 {
                    a.step(0.05);
                    match get(a, b) {
                        Some(e) => {
                            deepest = deepest.max(e.y + e.h);
                            last = e;
                        }
                        None => break,
                    }
                }
                assert!(get(a, f).is_none(), "the fish was caught");
                assert!(deepest > 9.0, "the bird went into the water");
                assert!(get(a, b).is_none(), "it left");
                assert_eq!(last.y, home, "back on its row");
                assert_eq!(named(a, "pelican"), 1, "the next visitor came");
            },
        );
    }

    #[test]
    fn a_bird_leaves_empty_handed_without_fish() {
        scene(
            &[("biome", s("ocean")), ("visitors", s("pelican"))],
            |ctx, a| {
                empty(ctx, a, 100, 30);
                let b = visit(a, "pelican", assets::PLUNGE, 0.0, true);
                for _ in 0..2400 {
                    a.step(0.05);
                }
                assert!(get(a, b).is_none());
                assert_eq!(named(a, "pelican"), 1);
            },
        );
    }

    #[test]
    fn a_predator_bites_small_things_and_leaves_scraps() {
        scene(
            &[("biome", s("ocean")), ("random_objects", false.into())],
            |ctx, a| {
                empty(ctx, a, 120, 40);
                let o = visit(a, "orca", assets::PREDATOR, 20.0, false);
                let e = a.ents.last_mut().unwrap();
                e.vx = 0.0;
                e.y = 15.0;
                let (cx, cy) = mid(e);
                let (f1, f2) = (fish(a, cx, cy), fish(a, cx + 2.0, cy));
                let mut t = 0.0;
                let mut first = None;
                let mut second = None;
                while t < 20.0 {
                    a.step(0.05);
                    t += 0.05;
                    if first.is_none() && (get(a, f1).is_none() || get(a, f2).is_none()) {
                        first = Some(t);
                    }
                    if second.is_none() && get(a, f1).is_none() && get(a, f2).is_none() {
                        second = Some(t);
                    }
                }
                let (first, second) = (first.expect("a bite"), second.expect("a second bite"));
                assert!(second - first >= 2.9, "bites {first} and {second}");
                assert!(get(a, o).is_some());
                let scraps = a
                    .ents
                    .iter()
                    .filter(|e| e.kind == Kind::Food && e.who == Food::Scrap as u16)
                    .count();
                assert!(scraps >= 2, "{scraps} scraps");
            },
        );
    }

    #[test]
    fn a_big_fish_leaves_big_things_alone() {
        scene(
            &[("biome", s("ocean")), ("random_objects", false.into())],
            |ctx, a| {
                empty(ctx, a, 120, 40);
                let k = a.spawn(Ent::new(Kind::Visitor, SP_BIG_FISH, 20.0, 15.0, Z_SHARK, W));
                a.ents[k].vx = 0.0;
                let (cx, cy) = mid(&a.ents[k]);
                let small = fish(a, cx, cy);
                let big = fish(a, cx + 1.0, cy);
                a.ents.last_mut().unwrap().size = 500;
                for _ in 0..200 {
                    a.step(0.05);
                }
                assert!(get(a, small).is_none());
                assert!(get(a, big).is_some());
            },
        );
    }

    #[test]
    fn a_filter_feeder_clears_plankton() {
        scene(
            &[("biome", s("ocean")), ("random_objects", false.into())],
            |ctx, a| {
                empty(ctx, a, 120, 40);
                let w = visit(a, "giant-whale-shark", assets::FILTER, 20.0, false);
                let e = a.ents.last_mut().unwrap();
                e.vx = 0.0;
                e.y = 15.0;
                let (cx, cy) = mid(e);
                for k in 0..5 {
                    a.add_food(cx + k as f32, cy, Food::Plankton);
                }
                a.add_food(110.0, 20.0, Food::Plankton);
                a.add_food(cx, cy, Food::Flake);
                a.step(0.05);
                let left = a.ents.iter().filter(|e| e.kind == Kind::Food).count();
                assert_eq!(left, 2, "only the far plankton and the flake remain");
                assert!(get(a, w).is_some());
            },
        );
    }

    #[test]
    fn a_trawler_catches_and_takes_its_net_away() {
        scene(
            &[("biome", s("ocean")), ("visitors", s("trawler"))],
            |ctx, a| {
                empty(ctx, a, 100, 30);
                let t = visit(a, "trawler", assets::NET, 30.0, false);
                a.step(0.05);
                let net = a
                    .ents
                    .iter()
                    .find(|e| matches!(e.task, Task::Net { .. }))
                    .copied()
                    .expect("a net");
                let boat = get(a, t).unwrap();
                assert!(net.y >= boat.y + boat.h);
                let f = fish(a, 0.0, 0.0);
                let fe = a.ents.last_mut().unwrap();
                fe.x = net.x + 6.0 - fe.w / 2.0;
                fe.y = net.y + 3.5 - fe.h / 2.0;
                a.step(0.05);
                assert!(matches!(get(a, f).unwrap().task, Task::Caught { .. }));
                let mut carried = 0;
                for _ in 0..4000 {
                    a.step(0.05);
                    if get(a, t).is_none() {
                        break;
                    }
                    if let (Some(fe), Some(n)) = (
                        get(a, f),
                        a.ents.iter().find(|e| matches!(e.task, Task::Net { .. })),
                    ) && within(n, fe.x, fe.y, 0.0)
                    {
                        carried += 1;
                    }
                }
                assert!(carried > 100, "the fish rode inside the net: {carried}");
                for _ in 0..1000 {
                    a.step(0.05);
                }
                assert!(get(a, f).is_none(), "the catch is gone");
                // The next trawler brings a net of its own.
                assert!(
                    !a.ents
                        .iter()
                        .any(|e| matches!(e.task, Task::Net { boat } if boat == t)),
                    "so is the net"
                );
                assert_eq!(named(a, "trawler"), 1, "the next visitor came");
            },
        );
    }

    #[test]
    fn an_angler_lowers_a_line_and_reels_in_a_fish() {
        scene(
            &[
                ("biome", s("fresh")),
                ("visitors", s("fishing-boat-angler")),
            ],
            |ctx, a| {
                empty(ctx, a, 100, 30);
                let b = visit(a, "fishing-boat-angler", assets::FISHING, 0.0, false);
                let speed = get(a, b).unwrap().vx;
                let mut f = None;
                let (mut longest, mut stopped) = (0.0f32, false);
                for _ in 0..8000 {
                    a.step(0.05);
                    let Some(e) = get(a, b) else { break };
                    stopped |= e.vx == 0.0;
                    if let Some(l) = a.ents.iter().find(|l| matches!(l.task, Task::Line { .. })) {
                        longest = longest.max(l.h - 1.0);
                        if f.is_none()
                            && matches!(
                                e.task,
                                Task::Angler {
                                    stage: Angle::Wait,
                                    ..
                                }
                            )
                        {
                            let hook = (l.x, l.y + l.frame);
                            f = Some(fish(a, hook.0 - 2.0, hook.1 - 1.0));
                        }
                    }
                }
                let f = f.expect("it reached the waiting stage");
                assert!(
                    stopped && longest >= 5.0,
                    "stopped {stopped}, line {longest}"
                );
                assert!(get(a, f).is_none(), "the fish was landed");
                assert!(get(a, b).is_none(), "the boat sailed on and out");
                assert!(!a.ents.iter().any(|l| matches!(l.task, Task::Line { .. })));
                assert_eq!(named(a, "fishing-boat-angler"), 1);
                assert!(speed != 0.0);
            },
        );
    }

    #[test]
    fn plants_sway_faster_when_touched_then_settle() {
        scene(
            &[("biome", s("tank")), ("random_objects", false.into())],
            |ctx, a| {
                empty(ctx, a, 100, 30);
                a.add_plant();
                let p = a.ents.iter().position(|e| e.kind == Kind::Seaweed).unwrap();
                a.ents[p].fps = 0.5;
                let (px, py) = (a.ents[p].x, a.ents[p].y);
                let f = fish(a, px, py);
                for _ in 0..4 {
                    a.step(0.05);
                }
                assert!((a.ents[p].fps - 1.5).abs() < 1e-4, "{}", a.ents[p].fps);
                a.ents.retain(|e| e.id != f);
                for _ in 0..100 {
                    a.step(0.05);
                }
                assert!((a.ents[p].fps - 0.5).abs() < 1e-4, "{}", a.ents[p].fps);
            },
        );
    }

    #[test]
    fn some_bubbles_pop_on_what_they_rise_into() {
        scene(
            &[("biome", s("tank")), ("random_objects", false.into())],
            |ctx, a| {
                empty(ctx, a, 100, 40);
                let id = fish(a, 40.0, 20.0);
                let f = a.ents.iter().position(|e| e.id == id).unwrap();
                a.ents[f].w = 30.0;
                a.ents[f].h = 3.0;
                let n = 300;
                for i in 0..n {
                    a.add_bubble_at(40.0 + (i % 30) as f32, 26.0, 5);
                }
                for _ in 0..16 {
                    a.step(0.05);
                }
                let past = a
                    .ents
                    .iter()
                    .filter(|e| e.kind == Kind::Bubble && e.y < 20.0)
                    .count();
                assert!(past > 150 && past < 260, "{past} of {n} got past");
            },
        );
    }

    #[test]
    fn crawlers_scurry_from_a_rover() {
        scene(
            &[("biome", s("ocean")), ("random_objects", false.into())],
            |ctx, a| {
                empty(ctx, a, 100, 30);
                let r = visit(a, "sea-rover", 0, 10.0, false);
                let re = get(a, r).unwrap();
                let mut c = Ent::new(Kind::Critter, SP_FISH, re.x + re.w / 2.0, 28.0, 20, W);
                c.vx = 2.0;
                c.speed = 2.0;
                c.motion = biome::swim_motion(&mut a.rng, Swim::Crawl, 28.0);
                let k = a.spawn(c);
                let id = a.ents[k].id;
                a.step(0.05);
                let c = get(a, id).unwrap();
                assert!(
                    c.vx.abs() > 2.5 && matches!(c.task, Task::Scurry(_)),
                    "{c:?}"
                );
                // Once the rover is gone it settles back to its own pace.
                a.ents.retain(|e| e.kind != Kind::Visitor);
                for _ in 0..80 {
                    a.step(0.05);
                }
                let c = get(a, id).unwrap();
                assert!((c.vx.abs() - 2.0).abs() < 1e-3);
            },
        );
    }

    #[test]
    fn five_minutes_in_every_biome() {
        let names = "pelican,seagulls,orca,trawler,fishing-boat-angler,giant-whale-shark,\
                     sea-rover,dolphins,big-fish";
        for (biome, _) in assets::BIOMES.iter().chain(&[("mixed", assets::ALL)]) {
            scene(
                &[
                    ("biome", s(biome)),
                    ("visitors", s(names)),
                    ("biome_time", toml::Value::Float(40.0)),
                ],
                |ctx, a| {
                    a.resize(ctx, 120, 40);
                    let mut c = Canvas::new(120, 40, Color::Reset);
                    let mut most = 0;
                    for i in 0..6000 {
                        a.update(ctx, 0.05);
                        most = most.max(a.ents.len());
                        if i % 97 == 0 {
                            c.clear();
                            a.draw(ctx, &mut c);
                        }
                    }
                    assert!(most < 500, "{biome}: {most} entities");
                    // A new size forgets every id.
                    a.resize(ctx, 60, 20);
                    for _ in 0..100 {
                        a.update(ctx, 0.05);
                    }
                },
            );
        }
    }
}
