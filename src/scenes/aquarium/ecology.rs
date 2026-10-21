//! How creatures react to each other and to what is around them.
//!
//! Every step the ecology takes a compact snapshot of what matters (creatures,
//! plants, props, threats, lures, hosts, food), then each creature in the near
//! layer moves according to its `Act`. An idle creature re-thinks only every
//! 0.2 to 0.4 s (staggered), so what is expensive happens a few times a second
//! while the moving happens every step.
//!
//! While a creature acts it is "taken over": its swim style is swapped for a
//! plain straight swim that `steer` and `drive` can move, and the original
//! motion waits in its `Mem` until the act is over (`give_back`). Steering y
//! means moving `base` as well as `y`, so the bob carries on from wherever the
//! creature ends up.

use std::collections::HashMap;
use std::f32::consts::{PI, TAU};
use std::hash::{BuildHasherDefault, Hasher};

use super::assets::{
    CLEANER, CURIOUS, DEFENDED, FILTER, GRAZER, INK, LURE, Lane, PREDATOR, PUFF, SCARY, SCAVENGER,
    SHELTER, SHY, STING, Swim, Traits,
};
use super::biome::{Food, between};
use super::library::library;
use super::*;

/// "No such entity" in an id slot.
const NONE: u32 = u32::MAX;

/// What a creature is up to.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub(super) enum Act {
    /// Following its swim style.
    #[default]
    Idle,
    /// Chasing the creature with id `prey`; gives up when `t` runs out.
    Hunt { prey: u32, t: f32 },
    /// Running from danger, toward row `ty`.
    Flee { t: f32, ty: f32 },
    /// Swimming back to the rest of its school.
    Regroup { t: f32 },
    /// Heading for the shelter `to` to hide.
    Shelter { to: u32, t: f32 },
    /// Tucked behind the shelter `by` (its own z was `z`).
    Hidden { by: u32, t: f32, z: u8 },
    /// Showing its `alt` shape.
    Puffed { t: f32 },
    /// Darting away from a sting, toward row `ty`.
    Stung { t: f32, ty: f32 },
    /// Swimming to its home, to join its ellipse at angle `phase`.
    ToHome { home: u32, t: f32, phase: f32 },
    /// Hovering around its home (a `Motion::Orbit`).
    AtHome { home: u32, t: f32 },
    /// Nibbling at a plant.
    Graze { plant: u32, t: f32 },
    /// Alongside a big host: at `fwd` cells ahead of its center, `dy` rows
    /// below its top (its own z was `z`).
    Clean {
        host: u32,
        t: f32,
        fwd: f32,
        dy: f32,
        z: u8,
    },
    /// Looking at a lure from `side` (-1 left, 1 right), `dy` rows off its center.
    Look {
        lure: u32,
        t: f32,
        dy: f32,
        side: f32,
    },
    /// Two crawlers nose to nose; `turn` says whether this one backs off.
    FaceOff { t: f32, turn: bool },
    /// Going for a bit of food.
    Eat { food: u32, t: f32 },
    /// Time ran out: swimming off to the nearest side.
    Leave,
}

impl Act {
    /// Out of sight behind something: nothing can see it, chase it or catch it.
    pub(super) fn hidden(self) -> bool {
        matches!(self, Act::Hidden { .. })
    }
}

/// An id hash: ids are small counters, so a multiply is all that is needed.
#[derive(Default)]
pub(super) struct IdHash(u64);

impl Hasher for IdHash {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 << 8 | b as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        }
    }
    fn write_u32(&mut self, i: u32) {
        self.0 = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
}

/// What one creature remembers between acts.
#[derive(Clone, Copy)]
struct Mem {
    /// The step it was last seen at, to forget the gone.
    stamp: u32,
    /// Seconds until it thinks again.
    think: f32,
    /// Its own motion while an act has replaced it.
    saved: Option<Motion>,
    /// Absolute times (`Aquarium::time`) before which it will not do something again.
    full: f32,
    ink: f32,
    home: f32,
    graze: f32,
    clean: f32,
    look: f32,
    sting: f32,
    /// The prop whose span a crawler is inside (so it only reacts on entering).
    inside: u32,
}

impl Mem {
    fn new(think: f32) -> Mem {
        Mem {
            stamp: 0,
            think,
            saved: None,
            full: 0.0,
            ink: 0.0,
            home: 0.0,
            graze: 0.0,
            clean: 0.0,
            look: 0.0,
            sting: 0.0,
            inside: NONE,
        }
    }
}

/// A box on the screen.
#[derive(Clone, Copy, Debug)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    fn of(e: &Ent) -> Rect {
        Rect {
            x: e.x,
            y: e.y,
            w: e.w,
            h: e.h,
        }
    }
    fn cx(&self) -> f32 {
        self.x + self.w * 0.5
    }
    fn cy(&self) -> f32 {
        self.y + self.h * 0.5
    }
    /// The empty space between two boxes along each axis (0 where they overlap).
    fn gap(&self, o: &Rect) -> (f32, f32) {
        let gx = (o.x - (self.x + self.w)).max(self.x - (o.x + o.w)).max(0.0);
        let gy = (o.y - (self.y + self.h)).max(self.y - (o.y + o.h)).max(0.0);
        (gx, gy)
    }
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

/// Whether all the bits of `mask` are set.
fn has(flags: u16, mask: u16) -> bool {
    flags & mask == mask
}

/// Whether a gap is inside an ellipse of `rx` columns by `ry` rows.
fn near(g: (f32, f32), rx: f32, ry: f32) -> bool {
    dist2(g, rx, ry) < 1.0
}

/// How far a gap is, in ellipse radii squared.
fn dist2(g: (f32, f32), rx: f32, ry: f32) -> f32 {
    (g.0 / rx).powi(2) + (g.1 / ry).powi(2)
}

// Snapshot flags.
/// On the screen (its center is).
const ON: u16 = 1;
const IDLE: u16 = 1 << 1;
const HIDDEN: u16 = 1 << 2;
const PUFFED: u16 = 1 << 3;
const HUNTING: u16 = 1 << 4;
const CRAWLER: u16 = 1 << 5;
/// Not ours to steer: an event's orbiter, or hooked.
const LOCKED: u16 = 1 << 6;
/// Not swimming at all (hooked).
const HOOKED: u16 = 1 << 7;
const PLANT: u16 = 1 << 8;
/// Food resting on the floor.
const SETTLED: u16 = 1 << 9;
const CLEANING: u16 = 1 << 10;
const EATING: u16 = 1 << 11;
const HOMING: u16 = 1 << 12;

/// One thing in the snapshot.
#[derive(Clone, Copy)]
struct Obj {
    /// Its index in `ents` (valid during the step: nothing is removed).
    i: u32,
    id: u32,
    r: Rect,
    vx: f32,
    size: u16,
    traits: Traits,
    who: u16,
    z: i32,
    group: u32,
    left: bool,
    flags: u16,
    /// What it is tied to: the host a cleaner is on, what a creature goes
    /// for, the creature that claimed a bit of food, the home it is at.
    tie: u32,
    name: &'static str,
}

/// The ecology's own state.
#[derive(Default)]
pub(super) struct Eco {
    /// The `interactions` option.
    pub(super) on: bool,
    mem: HashMap<u32, Mem, BuildHasherDefault<IdHash>>,
    /// Creatures in the near layer.
    crit: Vec<Obj>,
    /// Plants and props.
    scen: Vec<Obj>,
    /// What small creatures run from.
    threats: Vec<Obj>,
    /// What curious creatures come to see.
    lures: Vec<Obj>,
    /// Big things that cleaners keep company with (apart from big creatures).
    hosts: Vec<Obj>,
    food: Vec<Obj>,
    /// Clouds of ink.
    inks: Vec<Rect>,
    /// Things that sting (creatures and plants).
    stingers: Vec<Obj>,
    tick: u32,
    prune: f32,
    /// Seconds before the next hunt may begin, so kills stay rare.
    kill_cool: f32,
}

impl Eco {
    pub(super) fn from_opts(o: &crate::scene::Opts) -> Eco {
        Eco {
            on: o.b("interactions"),
            ..Eco::default()
        }
    }

    fn clear(&mut self) {
        self.crit.clear();
        self.scen.clear();
        self.threats.clear();
        self.lures.clear();
        self.hosts.clear();
        self.food.clear();
        self.inks.clear();
        self.stingers.clear();
    }
}

fn find(v: &[Obj], id: u32) -> Option<Obj> {
    v.iter().find(|o| o.id == id).copied()
}

/// Whether a prop or plant called `name` is one of the pieces in `home`.
fn is_home(home: &str, name: &str) -> bool {
    !home.is_empty()
        && home
            .split(',')
            .map(str::trim)
            .any(|p| !p.is_empty() && name.contains(p))
}

/// The cells a sprite covers.
fn cells(spr: u16) -> u16 {
    sprites()
        .get(spr as usize)
        .map_or(1, |s| s.frame(0).len().min(u16::MAX as usize) as u16)
}

fn is_crawler(e: &Ent) -> bool {
    library()
        .species
        .get(e.who as usize)
        .is_some_and(|s| s.def.swim == Swim::Crawl)
}

/// The pieces of plants and props a creature likes to stay at.
fn home_of(e: &Ent) -> &'static str {
    library()
        .species
        .get(e.who as usize)
        .map_or("", |s| s.def.home)
}

/// Turns a creature to face left or right and swims that way at `speed`.
fn go(e: &mut Ent, left: bool, speed: f32) {
    if Aquarium::facing_left(e) != left {
        Aquarium::face(e, left);
    }
    e.vx = if left { -speed.abs() } else { speed.abs() };
}

/// Heads toward a spot `dx` columns away at up to `speed`, easing off close
/// up so it stops on it instead of overshooting.
fn drive(e: &mut Ent, dx: f32, speed: f32) {
    if dx.abs() < 0.6 {
        e.vx = 0.0;
    } else {
        go(e, dx < 0.0, speed.min(dx.abs() * 5.0));
    }
}

/// Moves a creature up or down, taking its bob along.
fn shift(e: &mut Ent, dy: f32) {
    e.y += dy;
    if let Motion::Swim { base, .. } = &mut e.motion {
        *base += dy;
    }
}

/// Puts a creature under the ecology's control: a plain straight swim at its
/// own pace, which `drive` and `steer` can move. Its own motion is kept to
/// give back.
fn take_over(e: &mut Ent, m: &mut Mem) {
    let base_motion = match (e.motion, m.saved) {
        (Motion::Swim { .. }, None) => {
            m.saved = Some(e.motion);
            e.motion
        }
        (Motion::Orbit { .. }, Some(saved)) => saved,
        // Nothing to go back to: a plain swim will do.
        (Motion::Orbit { .. }, None) => Motion::Swim {
            style: Swim::Cruise,
            base: e.y,
            amp: 0.0,
            omega: 1.0,
            phase: 0.0,
            timer: 4.0,
        },
        _ => e.motion,
    };
    if let Motion::Swim {
        omega,
        phase,
        timer,
        ..
    } = base_motion
    {
        // Keep the phase running so a school stays in step meanwhile.
        let phase = match e.motion {
            Motion::Swim { phase, .. } => phase,
            _ => phase,
        };
        e.motion = Motion::Swim {
            style: Swim::Cruise,
            base: e.y,
            amp: 0.0,
            omega,
            phase,
            timer,
        };
    }
}

/// Gives a creature its own motion back, from where it is.
fn give_back(e: &mut Ent, m: &mut Mem) {
    let Some(Motion::Swim {
        style,
        amp,
        omega,
        phase: saved_phase,
        timer,
        ..
    }) = m.saved.take()
    else {
        return;
    };
    let phase = match e.motion {
        Motion::Swim { phase, .. } => phase,
        _ => saved_phase,
    };
    e.motion = Motion::Swim {
        style,
        base: e.y - amp * phase.sin(),
        amp,
        omega,
        phase,
        timer,
    };
}

/// An act is over: its motion comes back and it swims on the way it faces.
fn resume(e: &mut Ent, m: &mut Mem) {
    give_back(e, m);
    e.act = Act::Idle;
    go(e, Aquarium::facing_left(e), e.speed);
}

/// Nothing may keep a creature on screen forever, but holding acts take a while.
fn hold(e: &mut Ent, t: f32) {
    e.ttl = e.ttl.max(t + 60.0);
}

/// The snapshot of an entity.
fn obj(i: usize, e: &Ent) -> Obj {
    Obj {
        i: i as u32,
        id: e.id,
        r: Rect::of(e),
        vx: e.vx,
        size: e.size,
        traits: e.traits,
        who: e.who,
        z: e.z,
        group: NONE,
        left: e.vx < 0.0,
        flags: 0,
        tie: NONE,
        name: "",
    }
}

impl Aquarium {
    /// Once per step, after everything has moved: creatures notice each
    /// other and what is around them, and decide what to do.
    pub(super) fn ecology(&mut self, dt: f32) {
        let Some(w) = self.world.as_mut() else {
            return;
        };
        if !w.eco.on {
            return;
        }
        // Taken out so the ecology can borrow the scene freely.
        let mut eco = std::mem::take(&mut w.eco);
        eco.on = true;
        self.eco_step(&mut eco, dt);
        if let Some(w) = self.world.as_mut() {
            w.eco = eco;
        }
    }

    fn eco_step(&mut self, eco: &mut Eco, dt: f32) {
        eco.tick = eco.tick.wrapping_add(1);
        eco.kill_cool = (eco.kill_cool - dt).max(0.0);
        self.snapshot(eco);
        for k in 0..eco.crit.len() {
            self.creature(eco, k, dt);
        }
        eco.prune -= dt;
        if eco.prune <= 0.0 {
            eco.prune = 5.0;
            let tick = eco.tick;
            eco.mem.retain(|_, m| tick.wrapping_sub(m.stamp) < 200);
        }
    }

    /// A creature's `ttl` ran out; returns whether it dies now.
    pub(super) fn time_is_up(&mut self, e: &mut Ent, offscreen: bool) -> bool {
        if offscreen || !matches!(e.motion, Motion::Swim { .. } | Motion::Orbit { .. }) {
            return true;
        }
        // Drop whatever it was doing.
        if let Some(w) = self.world.as_mut() {
            w.eco.mem.remove(&e.id);
        }
        match e.act {
            Act::Hidden { z, .. } | Act::Clean { z, .. } => e.z = z as i32,
            Act::Puffed { .. } => self.reshape(e, false),
            _ => {}
        }
        let style = if is_crawler(e) {
            Swim::Crawl
        } else {
            Swim::Cruise
        };
        e.motion = Motion::Swim {
            style,
            base: e.y,
            amp: 0.0,
            omega: 1.0,
            phase: 0.0,
            timer: 4.0,
        };
        let left = e.x + e.w * 0.5 < self.w as f32 * 0.5;
        go(e, left, e.speed.max(3.0));
        e.act = Act::Leave;
        e.die_offscreen = true;
        e.seen = true;
        e.ttl = 60.0;
        false
    }

    /// The first and last row a creature may be in.
    fn bounds(&self, e: &Ent) -> (f32, f32) {
        let top = self.water_top() as f32;
        (top, (self.h as f32 - e.h).max(top))
    }

    /// Moves a creature toward row `ty` at up to `rate` rows a second.
    fn steer(&self, e: &mut Ent, ty: f32, rate: f32, dt: f32) {
        if is_crawler(e) {
            return;
        }
        let (top, bottom) = self.bounds(e);
        let dy = (ty.clamp(top, bottom) - e.y).clamp(-rate * dt, rate * dt);
        shift(e, dy);
    }

    fn on_screen(&self, r: &Rect) -> bool {
        let c = r.cx();
        c >= 1.0 && c < self.w as f32 - 1.0 && r.y + r.h > 0.0 && r.y < self.h as f32
    }

    /// Whether a snapshot entry is still there.
    fn alive(&self, o: &Obj) -> bool {
        self.ents
            .get(o.i as usize)
            .is_some_and(|e| e.id == o.id && !e.dead)
    }

    /// What is around, as a few short lists.
    fn snapshot(&mut self, eco: &mut Eco) {
        eco.clear();
        let lib = library();
        for (i, e) in self.ents.iter().enumerate() {
            if e.dead {
                continue;
            }
            let mut o = obj(i, e);
            if self.on_screen(&o.r) {
                o.flags |= ON;
            }
            match e.kind {
                Kind::Fish | Kind::Critter => {
                    if e.layer != 0 {
                        continue;
                    }
                    o.group = match e.death {
                        Death::Group(g) => g,
                        _ => NONE,
                    };
                    o.left = Self::facing_left(e);
                    o.name = lib.species.get(e.who as usize).map_or("", |s| s.def.name);
                    let swimming = matches!(e.motion, Motion::Swim { .. } | Motion::Orbit { .. });
                    o.flags |= match e.act {
                        Act::Idle => IDLE,
                        Act::Hidden { .. } => HIDDEN,
                        Act::Puffed { .. } => PUFFED,
                        Act::Hunt { prey, .. } => {
                            o.tie = prey;
                            HUNTING
                        }
                        Act::Clean { host, .. } => {
                            o.tie = host;
                            CLEANING
                        }
                        Act::Eat { food, .. } => {
                            o.tie = food;
                            EATING
                        }
                        Act::ToHome { home, .. } | Act::AtHome { home, .. } => {
                            o.tie = home;
                            HOMING
                        }
                        _ => 0,
                    };
                    if is_crawler(e) {
                        o.flags |= CRAWLER;
                    }
                    if !swimming {
                        o.flags |= HOOKED | LOCKED;
                    } else if e.tag != 0
                        && matches!(e.motion, Motion::Orbit { .. })
                        && e.act == Act::Idle
                    {
                        o.flags |= LOCKED;
                    }
                    if e.traits & STING != 0 {
                        eco.stingers.push(o);
                    }
                    eco.crit.push(o);
                }
                Kind::Seaweed | Kind::Decor => {
                    o.name = if e.kind == Kind::Seaweed {
                        lib.plants.get(e.who as usize).map_or("", |p| p.name)
                    } else {
                        lib.decor.get(e.who as usize).map_or("", |d| d.name)
                    };
                    if e.kind == Kind::Seaweed {
                        o.flags |= PLANT;
                    }
                    if e.traits & STING != 0 {
                        eco.stingers.push(o);
                    }
                    eco.scen.push(o);
                }
                Kind::Food => {
                    // The glyph is in the last row of its sprite.
                    o.r = Rect {
                        x: e.x,
                        y: e.y + (e.h - 1.0).max(0.0),
                        w: 1.0,
                        h: 1.0,
                    };
                    if e.vy == 0.0 && e.vx == 0.0 && e.y >= self.h as f32 - 1.5 {
                        o.flags |= SETTLED;
                    }
                    eco.food.push(o);
                }
                Kind::Ink => eco.inks.push(o.r),
                Kind::Shark | Kind::Dolphin => {
                    o.size = cells(e.spr);
                    eco.threats.push(o);
                    if e.kind == Kind::Shark {
                        eco.hosts.push(o);
                    }
                }
                Kind::Visitor => {
                    let Some(v) = lib.visitors.get(e.who as usize).filter(|_| e.who != NOBODY)
                    else {
                        // asciiquarium's big fish: something to keep company with.
                        if e.spr == SP_BIG_FISH || e.spr == SP_BIG_FISH + 1 {
                            o.size = cells(e.spr);
                            eco.hosts.push(o);
                        }
                        continue;
                    };
                    o.left = Self::facing_left(e);
                    if e.traits & (PREDATOR | SCARY) != 0 {
                        eco.threats.push(o);
                    }
                    match v.def.lane {
                        Lane::Deep => {
                            eco.hosts.push(o);
                            if e.traits & LURE != 0 {
                                eco.lures.push(o);
                            }
                        }
                        Lane::Floor | Lane::Float if e.traits & LURE != 0 => eco.lures.push(o),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // Hunters scare the small as much as any visitor does.
        for k in 0..eco.crit.len() {
            let o = eco.crit[k];
            if o.flags & HUNTING != 0 {
                eco.threats.push(o);
            }
            if o.flags & EATING != 0
                && let Some(f) = eco.food.iter_mut().find(|f| f.id == o.tie)
            {
                f.tie = o.id;
            }
        }
    }

    /// One creature's step: touching things, its act, and now and then a think.
    fn creature(&mut self, eco: &mut Eco, k: usize, dt: f32) {
        let o = eco.crit[k];
        let Some(mut e) = self.ents.get(o.i as usize).copied() else {
            return;
        };
        if e.dead || e.id != o.id {
            return;
        }
        let mut m = match eco.mem.get(&e.id) {
            Some(m) => *m,
            None => Mem::new(self.rng.f32() * 0.4),
        };
        m.stamp = eco.tick;
        if o.flags & LOCKED != 0 {
            // Hooked, or an event's own: leave it be, but let go of any act.
            if e.act != Act::Idle {
                e.act = Act::Idle;
                m.saved = None;
                self.ents[o.i as usize] = e;
            }
            eco.mem.insert(e.id, m);
            return;
        }
        let on = o.flags & ON != 0;
        if on || e.act != Act::Idle {
            self.touches(eco, &mut e, &mut m);
            self.run_act(eco, &mut e, &mut m, dt);
            m.think -= dt;
            if m.think <= 0.0 && on {
                m.think = between(&mut self.rng, 0.2, 0.4);
                self.think(eco, &mut e, &mut m);
            }
        }
        self.ents[o.i as usize] = e;
        eco.mem.insert(e.id, m);
    }

    // ---- Reactions to what it touches (every step) ----

    fn touches(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem) {
        let r = Rect::of(e);
        // Stings.
        if e.traits & STING == 0
            && self.time >= m.sting
            && !eco.stingers.is_empty()
            && !matches!(
                e.act,
                Act::Stung { .. }
                    | Act::Hidden { .. }
                    | Act::Clean { .. }
                    | Act::Puffed { .. }
                    | Act::Leave
            )
        {
            let home = home_of(e);
            // The empty margins of a sprite do not sting.
            let body = Rect {
                x: r.x + 1.0,
                w: (r.w - 2.0).max(1.0),
                ..r
            };
            let hit = eco
                .stingers
                .iter()
                .find(|s| s.id != e.id && body.gap(&s.r) == (0.0, 0.0) && !is_home(home, s.name));
            if let Some(&s) = hit {
                self.sting(e, m, &s);
                return;
            }
        }
        // Filter feeders swallow the plankton they swim through.
        if e.traits & FILTER != 0 {
            let mut left = 3;
            for f in &eco.food {
                if f.who == Food::Plankton as u16
                    && left > 0
                    && r.gap(&f.r) == (0.0, 0.0)
                    && let Some(fe) = self.ents.get_mut(f.i as usize)
                    && fe.id == f.id
                    && !fe.dead
                {
                    fe.dead = true;
                    left -= 1;
                }
            }
        }
        // A crawler that walks into a prop turns around, or settles behind it.
        if e.act == Act::Idle && is_crawler(e) {
            let lead = if Self::facing_left(e) { r.x } else { r.x + r.w };
            let mut inside = NONE;
            for p in eco.scen.iter().filter(|p| p.flags & PLANT == 0) {
                if p.r.h >= 2.0 && lead >= p.r.x + 1.0 && lead < p.r.x + p.r.w - 1.0 {
                    inside = p.id;
                    if m.inside != p.id {
                        self.crawler_meets(e, m, p);
                    }
                    break;
                }
            }
            m.inside = inside;
        }
    }

    fn crawler_meets(&mut self, e: &mut Ent, m: &mut Mem, prop: &Obj) {
        let home = home_of(e);
        if (e.traits & SHY != 0 || is_home(home, prop.name)) && self.rng.f32() < 0.7 {
            take_over(e, m);
            let t = between(&mut self.rng, 8.0, 25.0);
            self.tuck(e, prop, t);
        } else {
            let left = Self::facing_left(e);
            go(e, !left, e.speed);
            // Facing away, it is out of the prop before it can bump it again.
        }
    }

    /// Slips behind `s`: out of sight for `t` seconds.
    fn tuck(&mut self, e: &mut Ent, s: &Obj, t: f32) {
        let (x, y) = self.tuck_spot(e, s);
        e.x = x;
        let dy = y - e.y;
        shift(e, dy);
        e.vx = 0.0;
        let z0 = e.z.clamp(0, 255) as u8;
        e.z = s.z + 1;
        e.act = Act::Hidden { by: s.id, t, z: z0 };
        hold(e, t);
    }

    /// Where a creature goes to be hidden by `s`.
    fn tuck_spot(&self, e: &Ent, s: &Obj) -> (f32, f32) {
        let (top, bottom) = self.bounds(e);
        let y = if is_crawler(e) {
            bottom
        } else {
            (s.r.cy() - e.h * 0.5).clamp(top, bottom)
        };
        (s.r.cx() - e.w * 0.5, y)
    }

    fn sting(&mut self, e: &mut Ent, m: &mut Mem, s: &Obj) {
        take_over(e, m);
        let r = Rect::of(e);
        let left = r.cx() < s.r.cx();
        let crawl = is_crawler(e);
        let speed = if crawl {
            (e.speed * 3.0).clamp(6.0, 12.0)
        } else {
            (e.speed * 3.0).clamp(10.0, 35.0)
        };
        go(e, left, speed);
        let d = between(&mut self.rng, 3.0, 6.0);
        let ty = self.away_row(e, s.r.cy(), d);
        e.act = Act::Stung {
            t: between(&mut self.rng, 1.0, 2.0),
            ty,
        };
        m.sting = self.time + 12.0;
    }

    /// A row `d` rows away from `from`, up or down, inside the water.
    fn away_row(&mut self, e: &Ent, from: f32, d: f32) -> f32 {
        let (top, bottom) = self.bounds(e);
        let cy = e.y + e.h * 0.5;
        let down = if (cy - from).abs() < 1.0 {
            self.rng.bool()
        } else {
            cy > from
        };
        let ty = if down { e.y + d } else { e.y - d };
        if ty > bottom {
            (e.y - d).max(top)
        } else if ty < top {
            (e.y + d).min(bottom)
        } else {
            ty
        }
    }

    // ---- Acts (every step) ----

    fn run_act(&mut self, eco: &mut Eco, e: &mut Ent, m: &mut Mem, dt: f32) {
        match e.act {
            Act::Idle | Act::Leave => {}
            Act::Hunt { prey, t } => self.act_hunt(eco, e, m, prey, t - dt, dt),
            Act::Flee { t, ty } => {
                let rate = (e.vx.abs() * 0.3).max(4.0);
                self.steer(e, ty, rate, dt);
                if t - dt <= 0.0 {
                    self.calm(eco, e, m);
                } else {
                    e.act = Act::Flee { t: t - dt, ty };
                }
            }
            Act::Regroup { t } => self.act_regroup(eco, e, m, t - dt, dt),
            Act::Shelter { to, t } => self.act_shelter(eco, e, m, to, t - dt, dt),
            Act::Hidden { by, t, z } => {
                let t = t - dt;
                let gone = !find(&eco.scen, by).is_some_and(|s| self.alive(&s));
                if t <= 0.0 || gone {
                    e.z = z as i32;
                    resume(e, m);
                } else {
                    e.act = Act::Hidden { by, t, z };
                    e.vx = 0.0;
                }
            }
            Act::Puffed { t } => {
                if t - dt <= 0.0 {
                    self.reshape(e, false);
                    e.act = Act::Idle;
                } else {
                    e.act = Act::Puffed { t: t - dt };
                }
            }
            Act::Stung { t, ty } => {
                self.steer(e, ty, 6.0, dt);
                if t - dt <= 0.0 {
                    self.calm(eco, e, m);
                } else {
                    e.act = Act::Stung { t: t - dt, ty };
                }
            }
            Act::ToHome { home, t, phase } => self.act_to_home(eco, e, m, home, t - dt, phase, dt),
            Act::AtHome { home, t } => {
                let gone = !find(&eco.scen, home).is_some_and(|s| self.alive(&s));
                if t - dt <= 0.0 || gone {
                    m.home = self.time + between(&mut self.rng, 150.0, 300.0);
                    resume(e, m);
                } else {
                    e.act = Act::AtHome { home, t: t - dt };
                }
            }
            Act::Graze { plant, t } => self.act_graze(eco, e, m, plant, t - dt, dt),
            Act::Clean {
                host,
                t,
                fwd,
                dy,
                z,
            } => self.act_clean(eco, e, m, (host, t - dt, fwd, dy, z), dt),
            Act::Look { lure, t, dy, side } => {
                self.act_look(eco, e, m, (lure, t - dt, dy, side), dt)
            }
            Act::FaceOff { t, turn } => {
                if t - dt <= 0.0 {
                    give_back(e, m);
                    e.act = Act::Idle;
                    let left = Self::facing_left(e);
                    go(e, left != turn, e.speed);
                } else {
                    e.act = Act::FaceOff { t: t - dt, turn };
                    e.vx = 0.0;
                }
            }
            Act::Eat { food, t } => self.act_eat(eco, e, m, food, t - dt, dt),
        }
    }

    /// A flight or a sting is over: back to the school, or on its way.
    fn calm(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem) {
        let g = group_of(e);
        if g != NONE
            && eco
                .crit
                .iter()
                .any(|o| o.id != e.id && o.group == g && has(o.flags, IDLE | ON))
        {
            e.act = Act::Regroup { t: 14.0 };
        } else {
            resume(e, m);
        }
    }

    fn act_regroup(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, t: f32, dt: f32) {
        let r = Rect::of(e);
        let g = group_of(e);
        let mut anchor: Option<(f32, Obj)> = None;
        for o in &eco.crit {
            if o.id == e.id || o.group != g || !has(o.flags, IDLE | ON) {
                continue;
            }
            let d = (o.r.cx() - r.cx()).abs();
            if anchor.is_none_or(|(b, _)| d < b) {
                anchor = Some((d, *o));
            }
        }
        let Some((_, a)) = anchor.filter(|_| t > 0.0) else {
            resume(e, m);
            return;
        };
        // Get its direction first, then its place.
        if Self::facing_left(e) != a.left {
            go(e, a.left, e.speed);
        }
        let dir = if Self::facing_left(e) { -1.0 } else { 1.0 };
        let lead = (a.r.cx() - r.cx()) * dir;
        let dy = a.r.cy() - r.cy();
        // Back in line a little behind it, not on top of it: they swim on at
        // the same pace, so where it ends up is where it stays.
        let gap = e.w + 1.0;
        if (gap..gap + 8.0).contains(&lead) && dy.abs() < 1.5 && Self::facing_left(e) == a.left {
            resume(e, m);
            return;
        }
        let mult = if lead > gap {
            1.0 + ((lead - gap) / 30.0).min(0.8)
        } else {
            0.5
        };
        e.vx = dir * e.speed * mult;
        self.steer(e, a.r.cy() - e.h * 0.5, 2.5, dt);
        e.act = Act::Regroup { t };
    }

    fn act_hunt(&mut self, eco: &mut Eco, e: &mut Ent, m: &mut Mem, prey: u32, t: f32, dt: f32) {
        let lost = |this: &mut Aquarium, e: &mut Ent, m: &mut Mem| {
            // Tired for a few seconds.
            m.full = m.full.max(this.time + between(&mut this.rng, 6.0, 12.0));
            resume(e, m);
        };
        let Some(p) = find(&eco.crit, prey) else {
            return lost(self, e, m);
        };
        let pe = self.ents[p.i as usize];
        let hid = pe.act.hidden() || matches!(pe.act, Act::Puffed { .. });
        let pr = Rect::of(&pe);
        let inked = eco
            .inks
            .iter()
            .any(|k| k.contains(pr.cx(), pr.cy()) || k.contains(pr.x, pr.y));
        if pe.id != prey
            || pe.dead
            || hid
            || inked
            || t <= 0.0
            || !matches!(pe.motion, Motion::Swim { .. } | Motion::Orbit { .. })
            || !self.on_screen(&pr)
        {
            return lost(self, e, m);
        }
        let r = Rect::of(e);
        if r.gap(&pr) == (0.0, 0.0) {
            // Eaten.
            self.ents[p.i as usize].dead = true;
            for _ in 0..self.rng.usize(2..=4) {
                let x = pr.cx() + between(&mut self.rng, -1.5, 1.5);
                let y = pr.cy() + between(&mut self.rng, -0.5, 0.5);
                self.add_food(x, y, Food::Scrap);
            }
            m.full = self.time + between(&mut self.rng, 30.0, 90.0);
            eco.kill_cool = between(&mut self.rng, 8.0, 14.0);
            resume(e, m);
            return;
        }
        let speed = (e.speed * 1.6).max(6.0);
        let dx = pr.cx() - r.cx();
        let want_left = dx < 0.0;
        // Turn only when it is clearly behind, so it does not flicker.
        if (want_left != Self::facing_left(e) && dx.abs() > 2.5) || e.vx == 0.0 {
            go(e, want_left, speed);
        } else {
            e.vx = e.vx.signum() * speed;
        }
        self.steer(e, pr.cy() - e.h * 0.5, (speed * 0.3).max(3.0), dt);
        e.act = Act::Hunt { prey, t };
    }

    fn act_shelter(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, to: u32, t: f32, dt: f32) {
        let Some(s) = find(&eco.scen, to).filter(|s| self.alive(s) && t > 0.0) else {
            // No refuge after all: run.
            self.calm(eco, e, m);
            return;
        };
        let (tx, ty) = self.tuck_spot(e, &s);
        let speed = (e.speed * 2.0).max(8.0);
        drive(e, tx - e.x, speed);
        self.steer(e, ty, speed * 0.4, dt);
        if (tx - e.x).abs() < 1.2 && (ty - e.y).abs() < 1.0 {
            let t = between(&mut self.rng, 4.0, 8.0);
            self.tuck(e, &s, t);
        } else {
            e.act = Act::Shelter { to, t };
        }
    }

    /// Where a creature circles around its home: center and radii.
    fn home_ellipse(&self, e: &Ent, h: &Obj) -> (f32, f32, f32, f32) {
        // Wide enough that two residents across from each other do not overlap.
        let rx = (h.r.w * 0.3).clamp(2.0, 7.0).max(e.w * 0.6 + 1.0);
        let top = self.water_top() as f32;
        if is_crawler(e) {
            return (h.r.cx(), self.h as f32 - e.h * 0.5, rx, 0.0);
        }
        // And tall enough that they pass above and below each other.
        let ry = (h.r.h * 0.2).clamp(0.5, 1.5).max(e.h * 0.5);
        let lo = top + e.h * 0.5 + ry;
        let hi = (self.h as f32 - e.h * 0.5 - ry).max(lo);
        let cy = (h.r.y + h.r.h * 0.35).clamp(lo, hi);
        (h.r.cx(), cy, rx, ry)
    }

    #[allow(clippy::too_many_arguments)]
    fn act_to_home(
        &mut self,
        eco: &Eco,
        e: &mut Ent,
        m: &mut Mem,
        home: u32,
        t: f32,
        phase: f32,
        dt: f32,
    ) {
        let Some(h) = find(&eco.scen, home).filter(|h| self.alive(h) && t > 0.0) else {
            m.home = self.time + 30.0;
            resume(e, m);
            return;
        };
        let (cx, cy, rx, ry) = self.home_ellipse(e, &h);
        let tx = cx + rx * phase.cos() - e.w * 0.5;
        let ty = cy + ry * phase.sin() - e.h * 0.5;
        let speed = (e.speed * 1.3).max(5.0);
        drive(e, tx - e.x, speed);
        self.steer(e, ty, speed * 0.4, dt);
        if (tx - e.x).abs() < 1.5 && (ty - e.y).abs() < 1.2 {
            // Residents of one home all circle the same way at the same pace,
            // so they keep apart.
            let omega = if home.is_multiple_of(2) { 0.7 } else { -0.7 };
            e.motion = Motion::Orbit {
                cx,
                cy,
                rx,
                ry,
                omega,
                phase,
            };
            let t = between(&mut self.rng, 30.0, 90.0);
            e.act = Act::AtHome { home, t };
            hold(e, t);
        } else {
            e.act = Act::ToHome { home, t, phase };
        }
    }

    fn act_graze(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, plant: u32, t: f32, dt: f32) {
        let Some(p) = find(&eco.scen, plant).filter(|p| self.alive(p) && t > 0.0) else {
            m.graze = self.time + 20.0;
            resume(e, m);
            return;
        };
        // Hover in front of it at some height, bobbing and nudging forward.
        let k = (hash_id(e.id) % 100) as f32 / 100.0;
        let phase = self.time * 1.7 + e.id as f32;
        let dir = if p.r.cx() < e.x + e.w * 0.5 {
            -1.0
        } else {
            1.0
        };
        let nudge = dir * 1.5 * phase.sin().max(0.0);
        let tx = p.r.cx() - e.w * 0.5 - dir * (e.w * 0.25) + nudge;
        let ty = p.r.y + p.r.h * (0.3 + 0.5 * k) - e.h * 0.5
            + 0.5 * (self.time * 2.3 + e.id as f32).sin();
        let (dx, dy) = (tx - e.x, ty - e.y);
        let speed = (e.speed * 1.2).max(4.0);
        if dx.abs() + dy.abs() > 2.0 {
            drive(e, dx, speed);
            self.steer(e, ty, speed * 0.4, dt);
        } else {
            // Close: follow the target smoothly.
            e.vx = 0.0;
            e.x += dx * (6.0 * dt).min(1.0);
            let step = dy * (6.0 * dt).min(1.0);
            if !is_crawler(e) {
                shift(e, step);
            }
            let left = p.r.cx() < e.x + e.w * 0.5;
            if left != Self::facing_left(e) {
                Self::face(e, left);
            }
        }
        e.act = Act::Graze { plant, t };
    }

    fn act_clean(
        &mut self,
        eco: &Eco,
        e: &mut Ent,
        m: &mut Mem,
        (host, t, fwd, dy, z): (u32, f32, f32, f32, u8),
        dt: f32,
    ) {
        let h = find(&eco.hosts, host).or_else(|| find(&eco.crit, host));
        let h = h.filter(|h| {
            self.alive(h) && t > 0.0 && h.r.cx() > -4.0 && h.r.cx() < self.w as f32 + 4.0
        });
        let Some(h) = h else {
            e.z = z as i32;
            m.clean = self.time + between(&mut self.rng, 30.0, 60.0);
            resume(e, m);
            return;
        };
        let sgn = if h.left { -1.0 } else { 1.0 };
        let (top, bottom) = self.bounds(e);
        let tx = h.r.cx() + sgn * fwd - e.w * 0.5;
        let ty = (h.r.y + dy).clamp(top, bottom);
        let (ex, ey) = (tx - e.x, ty - e.y);
        e.vx = 0.0;
        e.z = (h.z - 1).max(0);
        if ex.abs() + ey.abs() > 2.0 {
            // Catching up.
            let s = (e.speed * 2.0).max(10.0) + h.vx.abs();
            e.x += ex.clamp(-s * dt, s * dt);
            shift(e, ey.clamp(-s * 0.5 * dt, s * 0.5 * dt));
            let left = ex < 0.0;
            if ex.abs() > 1.0 && left != Self::facing_left(e) {
                Self::face(e, left);
            }
        } else {
            e.x = tx;
            shift(e, ey);
            if h.left != Self::facing_left(e) {
                Self::face(e, h.left);
            }
        }
        e.act = Act::Clean {
            host,
            t,
            fwd,
            dy,
            z,
        };
    }

    fn act_look(
        &mut self,
        eco: &Eco,
        e: &mut Ent,
        m: &mut Mem,
        (lure, t, dy, side): (u32, f32, f32, f32),
        dt: f32,
    ) {
        let l = find(&eco.lures, lure).filter(|l| {
            self.alive(l) && t > 0.0 && l.r.cx() > -4.0 && l.r.cx() < self.w as f32 + 4.0
        });
        let Some(l) = l else {
            // Loses interest: swims off away from it.
            m.look = self.time + between(&mut self.rng, 40.0, 80.0);
            let away = find(&eco.lures, lure).map(|l| e.x + e.w * 0.5 < l.r.cx());
            resume(e, m);
            if let Some(left) = away {
                go(e, left, e.speed);
            }
            return;
        };
        let (top, bottom) = self.bounds(e);
        let tx = l.r.cx() + side * (l.r.w * 0.5 + 3.5) - e.w * 0.5;
        let ty = (l.r.cy() + dy - e.h * 0.5).clamp(top, bottom);
        let (ex, ey) = (tx - e.x, ty - e.y);
        let s = (e.speed * 1.5).max(6.0) + l.vx.abs();
        e.vx = 0.0;
        e.x += ex.clamp(-s * dt, s * dt);
        if !is_crawler(e) {
            shift(e, ey.clamp(-s * 0.5 * dt, s * 0.5 * dt));
        }
        let left = l.r.cx() < e.x + e.w * 0.5;
        if left != Self::facing_left(e) {
            Self::face(e, left);
        }
        e.act = Act::Look { lure, t, dy, side };
    }

    fn act_eat(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, food: u32, t: f32, dt: f32) {
        let Some(f) = find(&eco.food, food) else {
            return resume(e, m);
        };
        let fe = self.ents[f.i as usize];
        if fe.id != food || fe.dead || t <= 0.0 {
            return resume(e, m);
        }
        // Where its glyph is now (it sinks).
        let fr = Rect {
            x: fe.x,
            y: fe.y + (fe.h - 1.0).max(0.0),
            w: 1.0,
            h: 1.0,
        };
        let r = Rect::of(e);
        if r.gap(&fr) == (0.0, 0.0) {
            self.ents[f.i as usize].dead = true;
            return resume(e, m);
        }
        let speed = (e.speed * 1.5).max(6.0);
        let dx = fr.x - r.cx();
        let v = (dx.abs() * 3.0).clamp(1.0, speed);
        if dx.abs() > 1.5 || e.vx == 0.0 {
            go(e, dx < 0.0, v);
        } else {
            e.vx = e.vx.signum() * v;
        }
        self.steer(e, fr.y - e.h * 0.5, (speed * 0.3).max(3.0), dt);
        e.act = Act::Eat { food, t };
    }

    // ---- Thinking (a few times a second) ----

    fn think(&mut self, eco: &mut Eco, e: &mut Ent, m: &mut Mem) {
        let r = Rect::of(e);
        // Hidden creatures watch from further off.
        let k = if matches!(e.act, Act::Hidden { .. } | Act::Shelter { .. }) {
            1.5
        } else {
            1.0
        };
        let busy = matches!(
            e.act,
            Act::Clean { .. } | Act::Stung { .. } | Act::Leave | Act::FaceOff { .. }
        );
        if !busy && let Some(t) = danger(eco, e, &r, k) {
            self.alarm(eco, e, m, &r, &t);
            return;
        }
        match e.act {
            // The danger has passed: let the flight run out soon.
            Act::Flee { t, ty } => {
                e.act = Act::Flee { t: t.min(0.8), ty };
                return;
            }
            Act::Idle => {}
            _ => return,
        }
        let t = e.traits;
        if t & PREDATOR != 0
            && self.time >= m.full
            && eco.kill_cool <= 0.0
            && self.try_hunt(eco, e, m, &r)
        {
            return;
        }
        if self.try_eat(eco, e, m, &r)
            || self.try_home(eco, e, m, &r)
            || (t & CLEANER != 0 && self.try_clean(eco, e, m, &r))
            || (t & CURIOUS != 0 && self.try_look(eco, e, m, &r))
            || (t & GRAZER != 0 && self.try_graze(eco, e, m, &r))
        {
            return;
        }
        if t & FILTER != 0 {
            self.filter_drift(eco, e, &r);
        }
        if is_crawler(e) {
            self.try_face_off(eco, e, m, &r);
        } else {
            self.keep_apart(eco, e, &r);
        }
    }

    /// Two of a kind swimming on top of each other (after a scare, a meal,
    /// a visit) drift apart by a row or so; they swim at the same pace, so
    /// otherwise they would stay stacked.
    fn keep_apart(&self, eco: &Eco, e: &mut Ent, r: &Rect) {
        if !matches!(e.motion, Motion::Swim { .. }) {
            return;
        }
        let Some(o) = eco.crit.iter().find(|o| {
            o.id < e.id
                && o.who == e.who
                && has(o.flags, IDLE | ON)
                && (o.r.cx() - r.cx()).abs() < r.w * 0.6
                && (o.r.cy() - r.cy()).abs() < r.h * 0.6
        }) else {
            return;
        };
        let top = self.water_top() as f32;
        let bottom = self.h as f32 - e.h;
        let down = r.cy() >= o.r.cy();
        let dy = match (down, e.y + 0.5 <= bottom, e.y - 0.5 >= top) {
            (true, true, _) | (false, true, false) => 0.5,
            (_, _, true) => -0.5,
            _ => return,
        };
        shift(e, dy);
    }

    /// Something scary is near: hide, puff up, or run.
    fn alarm(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, r: &Rect, t: &Obj) {
        if let Act::Hidden { by, t: left, z } = e.act {
            e.act = Act::Hidden {
                by,
                t: left.max(4.0),
                z,
            };
            return;
        }
        if matches!(e.act, Act::Puffed { .. } | Act::Stung { .. }) {
            return;
        }
        let lib = library();
        let sp = lib.species.get(e.who as usize);
        let home = home_of(e);
        let g = r.gap(&t.r);
        // Puffing up works on what is about to bite, even on the run.
        if e.traits & PUFF != 0 && sp.is_some_and(|s| s.alt.is_some()) && near(g, 10.0, 4.0) {
            if e.act != Act::Idle {
                resume(e, m);
            }
            self.reshape(e, true);
            let secs = between(&mut self.rng, 3.0, 6.0);
            e.act = Act::Puffed { t: secs };
            self.ink_cloud(e, m);
            return;
        }
        match e.act {
            Act::Flee { t: left, ty } => {
                e.act = Act::Flee {
                    t: left.max(1.5),
                    ty,
                };
                return;
            }
            Act::Shelter { .. } => return,
            _ => {}
        }
        if e.traits & SHY != 0
            && let Some(s) = self.pick_shelter(eco, e, r, home)
        {
            take_over(e, m);
            e.act = Act::Shelter { to: s.id, t: 9.0 };
            self.ink_cloud(e, m);
            return;
        }
        self.flee(e, m, r, t);
    }

    fn flee(&mut self, e: &mut Ent, m: &mut Mem, r: &Rect, t: &Obj) {
        take_over(e, m);
        let left = r.cx() < t.r.cx();
        let room = if left { r.cx() } else { self.w as f32 - r.cx() };
        let blocked = room < 12.0;
        let speed = if is_crawler(e) {
            (e.speed * 2.0).clamp(5.0, 14.0)
        } else {
            (e.speed * 2.0).max(7.0)
        };
        // Against a wall it dashes up or down instead.
        go(e, left, if blocked { speed * 0.35 } else { speed });
        let far = if blocked { 5.0..9.0 } else { 3.0..8.0 };
        let d = between(&mut self.rng, far.start, far.end);
        let ty = self.away_row(e, t.r.cy(), d);
        e.act = Act::Flee {
            t: between(&mut self.rng, 2.5, 4.5),
            ty,
        };
        self.ink_cloud(e, m);
    }

    /// Squirts ink at its tail, if it is an ink squirter and not just did.
    fn ink_cloud(&mut self, e: &Ent, m: &mut Mem) {
        if e.traits & INK == 0 || self.time < m.ink {
            return;
        }
        let tail = if Self::facing_left(e) { e.x + e.w } else { e.x };
        self.add_ink(tail, e.y + e.h * 0.5, e.z - 1);
        m.ink = self.time + between(&mut self.rng, 10.0, 14.0);
    }

    /// A shape change (puffing up or down) around the same center.
    fn reshape(&self, e: &mut Ent, alt: bool) {
        let lib = library();
        let (cx, cy, y0) = (e.x + e.w * 0.5, e.y + e.h * 0.5, e.y);
        if !Self::set_alt(e, alt) {
            return;
        }
        let Some(sp) = lib.species.get(e.who as usize) else {
            return;
        };
        e.size = match (alt, sp.alt) {
            (true, Some(a)) => cells(a),
            _ => sp.size,
        };
        e.x = cx - e.w * 0.5;
        let (top, bottom) = self.bounds(e);
        let y = if is_crawler(e) {
            bottom
        } else {
            (cy - e.h * 0.5).clamp(top, bottom)
        };
        shift(e, y - y0);
    }

    fn pick_shelter(&self, eco: &Eco, e: &Ent, r: &Rect, home: &str) -> Option<Obj> {
        let mut best: Option<(f32, Obj)> = None;
        for o in &eco.scen {
            let own = is_home(home, o.name);
            if o.flags & ON == 0 || !(o.traits & SHELTER != 0 || own) {
                continue;
            }
            if o.traits & STING != 0 && !own {
                continue;
            }
            let (dx, dy) = ((o.r.cx() - r.cx()).abs(), (o.r.cy() - r.cy()).abs());
            if dx > 70.0 {
                continue;
            }
            let d = dx + 2.0 * dy;
            if best.is_none_or(|(b, _)| d < b) {
                best = Some((d, *o));
            }
        }
        let _ = e;
        best.map(|(_, o)| o)
    }

    fn try_hunt(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, r: &Rect) -> bool {
        let dir = if Self::facing_left(e) { -1.0 } else { 1.0 };
        let mut best: Option<(f32, u32)> = None;
        for o in &eco.crit {
            if o.id == e.id
                || o.flags & (ON | HIDDEN | PUFFED | HOOKED) != ON
                || o.traits & (DEFENDED | STING) != 0
                || o.who == e.who
                || o.size as u32 * 3 > e.size as u32
                || (o.flags & CLEANING != 0 && o.tie == e.id)
            {
                continue;
            }
            let g = r.gap(&o.r);
            if !near(g, 25.0, 8.0) {
                continue;
            }
            let ahead = (o.r.cx() - r.cx()) * dir > 0.0;
            if !ahead && g.0 > 8.0 {
                continue;
            }
            let d = dist2(g, 25.0, 8.0) + if ahead { 0.0 } else { 0.5 };
            if best.is_none_or(|(b, _)| d < b) && self.alive(o) {
                best = Some((d, o.id));
            }
        }
        let Some((_, prey)) = best else {
            return false;
        };
        // It does not always bother.
        if self.rng.f32() > 0.6 {
            return false;
        }
        take_over(e, m);
        e.act = Act::Hunt {
            prey,
            t: between(&mut self.rng, 4.0, 8.0),
        };
        true
    }

    fn try_eat(&mut self, eco: &mut Eco, e: &mut Ent, m: &mut Mem, r: &Rect) -> bool {
        if eco.food.is_empty() {
            return false;
        }
        let scav = e.traits & SCAVENGER != 0;
        let plain = e.kind == Kind::Fish && e.traits & PREDATOR == 0;
        if !scav && !plain {
            return false;
        }
        let mut best: Option<(f32, usize)> = None;
        for (k, f) in eco.food.iter().enumerate() {
            if f.tie != NONE || f.flags & ON == 0 {
                continue;
            }
            let g = r.gap(&f.r);
            let flake = f.who == Food::Flake as u16;
            let ok = (plain && flake && near(g, 20.0, 8.0))
                || (scav && f.flags & SETTLED != 0 && f.who != Food::Plankton as u16 && g.0 < 40.0);
            if ok {
                let d = dist2(g, 40.0, 16.0);
                if best.is_none_or(|(b, _)| d < b) {
                    best = Some((d, k));
                }
            }
        }
        let Some((_, k)) = best else {
            return false;
        };
        if self.rng.f32() > 0.8 {
            return false;
        }
        // One at a time: nobody else goes for this one now.
        eco.food[k].tie = e.id;
        take_over(e, m);
        e.act = Act::Eat {
            food: eco.food[k].id,
            t: 10.0,
        };
        true
    }

    /// Filter feeders drift toward the thickest plankton about.
    fn filter_drift(&mut self, eco: &Eco, e: &mut Ent, r: &Rect) {
        let (mut n, mut sx, mut sy) = (0.0, 0.0, 0.0);
        for f in &eco.food {
            if f.who == Food::Plankton as u16 && near(r.gap(&f.r), 30.0, 10.0) {
                n += 1.0;
                sx += f.r.x;
                sy += f.r.y;
            }
        }
        if n < 3.0 || is_crawler(e) {
            return;
        }
        let (cx, cy) = (sx / n, sy / n);
        // Into the cloud's band, not all to its middle (a school would stack up).
        let dy = cy - r.cy();
        if dy.abs() > 4.0 {
            shift(e, dy.clamp(-0.4, 0.4));
        }
        // Behind it and far: turn around for it.
        let dir = if Self::facing_left(e) { -1.0 } else { 1.0 };
        if (cx - r.cx()) * dir < -10.0 {
            go(e, !Self::facing_left(e), e.speed);
        }
    }

    fn try_home(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, r: &Rect) -> bool {
        let home = home_of(e);
        if home.is_empty() || self.time < m.home || self.rng.f32() > 0.5 {
            return false;
        }
        let mut best: Option<(f32, Obj)> = None;
        for o in &eco.scen {
            if o.flags & ON == 0 || !is_home(home, o.name) {
                continue;
            }
            // Plants and props can stand close together: everyone near one
            // spot shares one home, so they circle together without overlapping.
            // Crawlers pace along a line, so one per spot.
            let (crowd, used) = self.spot(eco, o);
            let room = if is_crawler(e) { 1 } else { 2 };
            if crowd >= room || used.is_some_and(|u| u != o.id) {
                continue;
            }
            let d = (o.r.cx() - r.cx()).abs() + 2.0 * (o.r.cy() - r.cy()).abs();
            if best.is_none_or(|(b, _)| d < b) {
                best = Some((d, *o));
            }
        }
        let Some((_, h)) = best else {
            return false;
        };
        take_over(e, m);
        // Across from whoever lives there already (they circle together at
        // the same pace), else anywhere around it.
        let neighbor = self.residents(h.id).find_map(|n| match (n.motion, n.act) {
            (Motion::Orbit { phase, .. }, _) | (_, Act::ToHome { phase, .. }) => Some(phase),
            _ => None,
        });
        let phase = match neighbor {
            Some(p) => (p + PI) % TAU,
            None => self.rng.f32() * TAU,
        };
        e.act = Act::ToHome {
            home: h.id,
            t: 25.0,
            phase,
        };
        hold(e, 25.0);
        true
    }

    fn try_graze(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, r: &Rect) -> bool {
        if self.time < m.graze || self.rng.f32() > 0.15 {
            return false;
        }
        let mut best: Option<(f32, Obj)> = None;
        for o in &eco.scen {
            if !has(o.flags, PLANT | ON) || o.traits & STING != 0 || o.r.h < 2.0 {
                continue;
            }
            let g = r.gap(&o.r);
            if !near(g, 12.0, 5.0) {
                continue;
            }
            let d = dist2(g, 12.0, 5.0);
            if best.is_none_or(|(b, _)| d < b) {
                best = Some((d, *o));
            }
        }
        let Some((_, p)) = best else {
            return false;
        };
        take_over(e, m);
        let t = between(&mut self.rng, 4.0, 12.0);
        e.act = Act::Graze { plant: p.id, t };
        hold(e, t);
        m.graze = self.time + between(&mut self.rng, 25.0, 55.0);
        true
    }

    fn try_clean(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, r: &Rect) -> bool {
        if self.time < m.clean || self.rng.f32() > 0.4 {
            return false;
        }
        let mut best: Option<(f32, Obj)> = None;
        let big = |o: &Obj| {
            o.id != e.id
                && o.flags & (ON | HIDDEN | HOOKED) == ON
                && o.traits & CLEANER == 0
                && o.size as u32 >= e.size as u32 * 4
                && o.who != e.who
        };
        for o in eco.hosts.iter().filter(|o| o.flags & ON != 0).chain(
            eco.crit
                .iter()
                .filter(|o| big(o) && o.flags & (PUFFED | EATING) == 0),
        ) {
            if o.id == e.id {
                continue;
            }
            if (o.size as u32) < e.size as u32 * 4 {
                continue;
            }
            let g = r.gap(&o.r);
            if !near(g, 35.0, 12.0) {
                continue;
            }
            // Two cleaners at a time is plenty.
            let crowd = eco
                .crit
                .iter()
                .filter(|c| c.flags & CLEANING != 0 && c.tie == o.id)
                .count();
            if crowd >= 2 {
                continue;
            }
            let d = dist2(g, 35.0, 12.0);
            if best.is_none_or(|(b, _)| d < b) {
                best = Some((d, *o));
            }
        }
        let Some((_, h)) = best else {
            return false;
        };
        // By its belly, or just ahead of its head.
        let (fwd, dy) = if self.rng.bool() {
            (
                between(&mut self.rng, -0.25, 0.25) * h.r.w,
                h.r.h - e.h * 0.5,
            )
        } else {
            (
                h.r.w * 0.5 + e.w * 0.5 + 0.5,
                (h.r.h - e.h) * 0.5 + between(&mut self.rng, -1.0, 1.0),
            )
        };
        take_over(e, m);
        let t = between(&mut self.rng, 10.0, 40.0);
        let z0 = e.z.clamp(0, 255) as u8;
        e.z = (h.z - 1).max(0);
        e.act = Act::Clean {
            host: h.id,
            t,
            fwd,
            dy,
            z: z0,
        };
        hold(e, t);
        true
    }

    fn try_look(&mut self, eco: &Eco, e: &mut Ent, m: &mut Mem, r: &Rect) -> bool {
        if self.time < m.look || self.rng.f32() > 0.25 {
            return false;
        }
        let mut best: Option<(f32, Obj)> = None;
        for o in eco.lures.iter().filter(|o| o.flags & ON != 0) {
            let g = r.gap(&o.r);
            if !near(g, 40.0, 14.0) || self.lookers(o.id).count() >= 2 {
                continue;
            }
            let d = dist2(g, 40.0, 14.0);
            if best.is_none_or(|(b, _)| d < b) {
                best = Some((d, *o));
            }
        }
        let Some((_, l)) = best else {
            return false;
        };
        take_over(e, m);
        let t = between(&mut self.rng, 15.0, 40.0);
        // One on each side, so two onlookers do not pile up.
        let taken = self.lookers(l.id).find_map(|o| match o.act {
            Act::Look { side, .. } => Some(side),
            _ => None,
        });
        let side = match taken {
            Some(s) => -s,
            None if r.cx() < l.r.cx() => -1.0,
            None => 1.0,
        };
        e.act = Act::Look {
            lure: l.id,
            t,
            dy: between(&mut self.rng, -2.0, 2.0),
            side,
        };
        hold(e, t);
        true
    }

    /// Two crawlers walking toward each other stop nose to nose.
    fn try_face_off(&mut self, eco: &mut Eco, e: &mut Ent, m: &mut Mem, r: &Rect) {
        let left = Self::facing_left(e);
        let other = eco.crit.iter().find(|o| {
            o.id != e.id
                && has(o.flags, CRAWLER | IDLE | ON)
                && o.left != left
                && (o.r.cx() < r.cx()) == left
                && r.gap(&o.r).0 <= 2.5
                && ((o.r.y + o.r.h) - (r.y + r.h)).abs() < 0.5
        });
        let Some(&o) = other else {
            self.wait_behind(eco, e, r, left);
            return;
        };
        // The one with the lower id always backs off; the other maybe.
        let mine = e.id < o.id || self.rng.f32() < 0.4;
        let theirs = !mine || (o.id < e.id || self.rng.f32() < 0.4);
        take_over(e, m);
        e.vx = 0.0;
        e.act = Act::FaceOff {
            t: between(&mut self.rng, 1.0, 2.0),
            turn: mine,
        };
        let Some(mut pe) = self.ents.get(o.i as usize).copied() else {
            return;
        };
        let mut pm = match eco.mem.get(&pe.id) {
            Some(m) => *m,
            None => Mem::new(0.3),
        };
        take_over(&mut pe, &mut pm);
        pe.vx = 0.0;
        pe.act = Act::FaceOff {
            t: between(&mut self.rng, 1.0, 2.0),
            turn: theirs,
        };
        self.ents[o.i as usize] = pe;
        eco.mem.insert(pe.id, pm);
    }
}

impl Aquarium {
    /// The creatures living at (or heading to) home `id`. From the entities
    /// themselves, not the snapshot, so choices made earlier in this step count.
    fn residents(&self, id: u32) -> impl Iterator<Item = &Ent> {
        self.ents.iter().filter(move |n| {
            !n.dead
                && matches!(n.act, Act::ToHome { home, .. } | Act::AtHome { home, .. } if home == id)
        })
    }

    /// The creatures having a look at lure `id`.
    fn lookers(&self, id: u32) -> impl Iterator<Item = &Ent> {
        self.ents
            .iter()
            .filter(move |n| !n.dead && matches!(n.act, Act::Look { lure, .. } if lure == id))
    }

    /// How many creatures live at (or head to) homes near `o`, and the home
    /// they use.
    fn spot(&self, eco: &Eco, o: &Obj) -> (usize, Option<u32>) {
        let (mut n, mut used) = (0, None);
        for r in &self.ents {
            let (Act::ToHome { home, .. } | Act::AtHome { home, .. }) = r.act else {
                continue;
            };
            if r.dead {
                continue;
            }
            if let Some(h) = find(&eco.scen, home)
                && (h.r.cx() - o.r.cx()).abs() < 8.0
                && (h.r.cy() - o.r.cy()).abs() < 4.0
            {
                n += 1;
                used = Some(home);
            }
        }
        (n, used)
    }

    /// A crawler catching up with one going the same way rests a moment
    /// instead of walking over it (they all walk the same row).
    fn wait_behind(&mut self, eco: &Eco, e: &mut Ent, r: &Rect, left: bool) {
        let blocked = eco.crit.iter().any(|o| {
            o.id != e.id
                && has(o.flags, CRAWLER | IDLE | ON)
                && o.left == left
                && (o.r.cx() < r.cx()) == left
                && r.gap(&o.r).0 <= 1.0
                && ((o.r.y + o.r.h) - (r.y + r.h)).abs() < 0.5
        });
        if let Motion::Swim { timer, .. } = &mut e.motion
            && blocked
            && *timer > 0.0
        {
            *timer = -between(&mut self.rng, 1.0, 3.0);
        }
    }
}

/// The group a creature belongs to.
fn group_of(e: &Ent) -> u32 {
    match e.death {
        Death::Group(g) => g,
        _ => NONE,
    }
}

fn hash_id(id: u32) -> u64 {
    biome::hash(id as u64)
}

/// The nearest thing this creature has reason to run from.
fn danger(eco: &Eco, e: &Ent, r: &Rect, k: f32) -> Option<Obj> {
    let mut best: Option<(f32, Obj)> = None;
    for t in &eco.threats {
        if t.id == e.id {
            continue;
        }
        let hunter = t.flags & HUNTING != 0;
        let (rx, ry) = if hunter {
            // Hunters leave the defended and the stinging alone, so those need not run.
            if t.who == e.who
                || e.size as u32 * 3 > t.size as u32
                || e.traits & (DEFENDED | STING) != 0
            {
                continue;
            }
            (16.0 * k, 6.0 * k)
        } else {
            if e.size as u32 * 2 > t.size as u32 {
                continue;
            }
            (22.0 * k, 8.0 * k)
        };
        let g = r.gap(&t.r);
        if !near(g, rx, ry) {
            continue;
        }
        let d = dist2(g, rx, ry);
        if best.is_none_or(|(b, _)| d < b) {
            best = Some((d, *t));
        }
    }
    best.map(|(_, t)| t)
}

#[cfg(test)]
mod tests {
    use super::super::assets::{self as a, Traits};
    use super::*;
    use std::collections::HashSet;

    /// A quiet reef: no fish, plants, props, visitors, events or weather of its own.
    fn quiet(f: impl FnOnce(&Ctx, &mut Aquarium)) {
        let theme = crate::theme::Theme::classic();
        let mut t = toml::Table::new();
        t.insert("biome".into(), "reef".into());
        t.insert("fish".into(), toml::Value::Float(0.0));
        t.insert("seaweed".into(), toml::Value::Float(0.0));
        t.insert("decor".into(), "none".into());
        t.insert("random_objects".into(), toml::Value::Boolean(false));
        t.insert("events".into(), toml::Value::Float(0.0));
        t.insert("weather".into(), toml::Value::Boolean(false));
        t.insert("day_length".into(), toml::Value::Float(0.0));
        t.insert("depth".into(), toml::Value::Float(0.0));
        t.insert("biome_time".into(), toml::Value::Float(0.0));
        let opts = crate::scene::Opts::new(INFO.options, t);
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 9,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut aq = Aquarium::new(&ctx);
        aq.retheme(&ctx);
        aq.resize(&ctx, 120, 40);
        aq.ents.retain(|e| matches!(e.kind, Kind::Water(_)));
        aq.world.as_mut().unwrap().groups.clear();
        f(&ctx, &mut aq);
    }

    /// A creature of species `name`, swimming at `speed` cells a second, with `traits`.
    fn put(
        aq: &mut Aquarium,
        name: &str,
        (x, y): (f32, f32),
        left: bool,
        speed: f32,
        traits: Traits,
    ) -> u32 {
        let lib = library();
        let si = lib.species_named(name).unwrap();
        let sp = &lib.species[si];
        let spr = sp.spr + left as u16;
        let kind = if sp.def.fish {
            Kind::Fish
        } else {
            Kind::Critter
        };
        let mut e = Ent::new(kind, spr, x, y, 10, sp.color);
        e.vx = if left { -speed } else { speed };
        e.die_offscreen = true;
        e.who = si as u16;
        e.traits = traits;
        e.size = sp.size;
        e.speed = speed;
        let style = if sp.def.swim == Swim::Crawl {
            Swim::Crawl
        } else {
            Swim::Cruise
        };
        e.motion = biome::swim_motion(&mut aq.rng, style, y);
        let i = aq.spawn(e);
        aq.ents[i].id
    }

    fn set_size(aq: &mut Aquarium, id: u32, size: u16) {
        aq.ents.iter_mut().find(|e| e.id == id).unwrap().size = size;
    }

    /// A plant of `name`, rooted at column `x`.
    fn plant(aq: &mut Aquarium, name: &str, x: f32, traits: Traits) -> u32 {
        let lib = library();
        let pi = lib.plant_named(name).unwrap();
        let p = &lib.plants[pi];
        let s = &lib.sprites[p.spr as usize];
        let (y, z) = ((aq.h - s.h) as f32, Z_SEAWEED);
        let mut e = Ent::new(Kind::Seaweed, p.spr, x, y, z, p.color);
        e.who = pi as u16;
        e.traits = traits;
        e.size = s.frame(0).len() as u16;
        e.ttl = 1e6;
        let i = aq.spawn(e);
        aq.ents[i].id
    }

    /// A prop of `name`, on the floor at column `x`.
    fn prop(aq: &mut Aquarium, name: &str, x: f32, traits: Traits) -> u32 {
        let lib = library();
        let di = lib.decor_named(name).unwrap();
        let d = &lib.decor[di];
        let s = &lib.sprites[d.spr as usize];
        let (y, z) = ((aq.h - s.h) as f32, Z_CASTLE);
        let mut e = Ent::new(Kind::Decor, d.spr, x, y, z, d.color);
        e.who = di as u16;
        e.traits = traits;
        e.size = s.frame(0).len() as u16;
        let i = aq.spawn(e);
        aq.ents[i].id
    }

    /// A passer-by of `name` that goes `vx` cells a second from where it is.
    fn visit(aq: &mut Aquarium, name: &str, (x, y): (f32, f32), vx: f32, traits: Traits) -> u32 {
        let lib = library();
        let vi = lib.visitor_named(name).unwrap();
        aq.add_visitor(vi as u16);
        let e = aq.ents.last_mut().unwrap();
        let spr = lib.visitors[vi].spr + (vx < 0.0) as u16;
        e.spr = spr;
        e.x = x;
        e.y = y;
        e.vx = vx;
        e.traits = traits;
        e.die_offscreen = false;
        e.id
    }

    fn get(aq: &Aquarium, id: u32) -> Option<Ent> {
        aq.ents.iter().find(|e| e.id == id && !e.dead).copied()
    }

    fn kill(aq: &mut Aquarium, id: u32) {
        aq.ents.iter_mut().find(|e| e.id == id).unwrap().dead = true;
    }

    fn mem(aq: &Aquarium, id: u32) -> Mem {
        aq.world.as_ref().unwrap().eco.mem[&id]
    }

    /// Runs `secs` seconds at 30 fps, calling `each` after every frame.
    fn run(ctx: &Ctx, aq: &mut Aquarium, secs: f32, mut each: impl FnMut(&mut Aquarium)) {
        for _ in 0..(secs * 30.0) as usize {
            aq.update(ctx, 1.0 / 30.0);
            each(aq);
        }
    }

    #[test]
    fn a_predator_eats_prey_and_is_then_full() {
        quiet(|ctx, aq| {
            let pred = put(
                aq,
                "great-white-shark",
                (20.0, 20.0),
                false,
                6.0,
                a::PREDATOR,
            );
            let prey = put(aq, "classic-3", (55.0, 22.0), false, 3.0, 0);
            set_size(aq, pred, 600);
            set_size(aq, prey, 20);
            let mut scraps = 0;
            for _ in 0..30 * 25 {
                aq.update(ctx, 1.0 / 30.0);
                let n = aq
                    .ents
                    .iter()
                    .filter(|e| e.kind == Kind::Food && e.who == Food::Scrap as u16)
                    .count();
                scraps = scraps.max(n);
                if get(aq, prey).is_none() {
                    break;
                }
            }
            assert!(get(aq, prey).is_none(), "the prey was eaten");
            assert!((2..=4).contains(&scraps), "{scraps} scraps");
            assert!(mem(aq, pred).full > aq.time + 20.0, "the predator is full");
            assert!(get(aq, pred).is_some());
        });
    }

    #[test]
    fn a_full_predator_leaves_prey_alone() {
        quiet(|ctx, aq| {
            let pred = put(
                aq,
                "great-white-shark",
                (20.0, 20.0),
                false,
                3.0,
                a::PREDATOR,
            );
            set_size(aq, pred, 600);
            let prey = put(aq, "classic-3", (60.0, 22.0), true, 1.0, 0);
            set_size(aq, prey, 20);
            aq.update(ctx, 0.05);
            let mut m = mem(aq, pred);
            m.full = 1e6;
            aq.world.as_mut().unwrap().eco.mem.insert(pred, m);
            run(ctx, aq, 8.0, |aq| {
                assert_eq!(get(aq, pred).unwrap().act, Act::Idle);
            });
        });
    }

    #[test]
    fn prey_flees_fast_and_rejoins_its_school() {
        quiet(|ctx, aq| {
            let gid = 77;
            aq.world.as_mut().unwrap().groups.push(biome::Group {
                id: gid,
                alive: 3,
                crawler: false,
                replace: false,
            });
            let a1 = put(aq, "classic-2", (55.0, 20.0), false, 3.0, 0);
            let m1 = put(aq, "classic-2", (20.0, 19.0), false, 3.0, 0);
            let m2 = put(aq, "classic-2", (28.0, 21.0), false, 3.0, 0);
            for id in [a1, m1, m2] {
                let e = aq.ents.iter_mut().find(|e| e.id == id).unwrap();
                e.death = Death::Group(gid);
                e.size = 30;
            }
            let orca = visit(aq, "orca", (70.0, 18.0), 0.0, a::PREDATOR | a::SCARY);
            let x0 = get(aq, a1).unwrap().x;
            let mut fastest = 0.0f32;
            let mut fled = false;
            run(ctx, aq, 2.0, |aq| {
                let e = get(aq, a1).unwrap();
                fastest = fastest.max(e.vx.abs());
                fled |= matches!(e.act, Act::Flee { .. });
            });
            let e = get(aq, a1).unwrap();
            assert!(fled, "it ran");
            assert!(fastest >= 2.0 * 3.0, "{fastest}");
            assert!(e.x < x0, "away from the threat");
            // The mates, out of range, carried on.
            assert_eq!(get(aq, m1).unwrap().act, Act::Idle);
            kill(aq, orca);
            run(ctx, aq, 22.0, |_| {});
            let (e, f) = (get(aq, a1).unwrap(), get(aq, m1).unwrap());
            assert_eq!(e.act, Act::Idle);
            assert_eq!(Aquarium::facing_left(&e), Aquarium::facing_left(&f));
            assert_eq!(e.vx, f.vx, "same pace again");
            assert!((e.x - f.x).abs() < 30.0, "{} {}", e.x, f.x);
        });
    }

    #[test]
    fn a_shy_creature_hides_then_comes_out() {
        quiet(|ctx, aq| {
            let weed = plant(aq, "seaweed", 70.0, a::SHELTER);
            let fish = put(aq, "classic-4", (50.0, 30.0), false, 4.0, a::SHY);
            set_size(aq, fish, 20);
            let orca = visit(aq, "orca", (30.0, 28.0), 0.0, a::PREDATOR | a::SCARY);
            let mut hid = false;
            run(ctx, aq, 12.0, |aq| {
                let e = get(aq, fish).unwrap();
                if let Act::Hidden { by, .. } = e.act {
                    hid = true;
                    assert_eq!(by, weed);
                    let w = get(aq, weed).unwrap();
                    assert!(e.z > w.z, "behind the plant");
                    assert!((e.x + e.w / 2.0 - (w.x + w.w / 2.0)).abs() < 1.5);
                    assert_eq!(e.vx, 0.0);
                }
            });
            assert!(hid, "it hid");
            // Still hidden while the threat stays.
            assert!(get(aq, fish).unwrap().act.hidden());
            kill(aq, orca);
            run(ctx, aq, 15.0, |_| {});
            let e = get(aq, fish).unwrap();
            assert_eq!(e.act, Act::Idle);
            assert_eq!(e.z, 10, "in front of the plants again");
            assert!(e.vx != 0.0);
        });
    }

    #[test]
    fn a_homebody_goes_home_and_stays_near_it() {
        quiet(|ctx, aq| {
            let home = plant(aq, "pink-anemone", 70.0, a::SHELTER | a::STING);
            let fish = put(aq, "ocellaris-clownfish", (20.0, 25.0), false, 6.0, 0);
            let mut arrived = None;
            let mut stung = false;
            let mut far = 0.0f32;
            let mut left_at = None;
            run(ctx, aq, 320.0, |aq| {
                // (Once it has left it swims off the screen and is gone.)
                let Some(e) = get(aq, fish) else { return };
                stung |= matches!(e.act, Act::Stung { .. });
                let h = get(aq, home).unwrap();
                match e.act {
                    Act::AtHome { .. } => {
                        arrived.get_or_insert(aq.time);
                        far = far.max((e.x + e.w / 2.0 - (h.x + h.w / 2.0)).abs());
                    }
                    _ if arrived.is_some() && left_at.is_none() => left_at = Some(aq.time),
                    _ => {}
                }
            });
            let arrived = arrived.expect("it went home");
            assert!(arrived < 30.0, "{arrived}");
            assert!(!stung, "its home never stings it");
            assert!(far < 12.0, "{far}");
            let left_at = left_at.expect("it left after a while");
            let stay = left_at - arrived;
            assert!((55.0..=300.0).contains(&stay), "{stay}");
        });
    }

    #[test]
    fn a_cleaner_rides_along_with_a_big_host() {
        quiet(|ctx, aq| {
            let host = put(
                aq,
                "great-white-shark",
                (20.0, 15.0),
                false,
                2.0,
                a::PREDATOR,
            );
            let c = put(
                aq,
                "bluestreak-cleaner-wrasse",
                (45.0, 18.0),
                false,
                3.0,
                a::CLEANER,
            );
            set_size(aq, host, 600);
            set_size(aq, c, 20);
            let mut offsets: Vec<f32> = Vec::new();
            let mut same_way = false;
            run(ctx, aq, 25.0, |aq| {
                let (e, h) = (get(aq, c).expect("never eaten"), get(aq, host).unwrap());
                if matches!(e.act, Act::Clean { .. }) {
                    offsets.push(e.x + e.w / 2.0 - (h.x + h.w / 2.0));
                    same_way = Aquarium::facing_left(&e) == Aquarium::facing_left(&h);
                    assert!(e.z < h.z);
                }
            });
            assert!(offsets.len() > 100, "it attached ({})", offsets.len());
            assert!(same_way, "it faces the way its host does");
            // Once attached the offset is fixed.
            let tail = &offsets[offsets.len() - 60..];
            let hi = tail.iter().copied().fold(f32::MIN, f32::max);
            let lo = tail.iter().copied().fold(f32::MAX, f32::min);
            assert!(hi - lo < 1.0, "{}", hi - lo);
        });
    }

    #[test]
    fn time_is_up_on_screen_makes_it_leave() {
        quiet(|ctx, aq| {
            let fish = put(aq, "classic-1", (40.0, 20.0), false, 5.0, 0);
            aq.ents.iter_mut().find(|e| e.id == fish).unwrap().ttl = 0.01;
            aq.update(ctx, 0.05);
            let e = get(aq, fish).expect("still there");
            assert_eq!(e.act, Act::Leave);
            assert!(e.ttl > 30.0);
            assert!(e.vx.abs() > 0.0);
            // Off screen it just dies.
            let mut off = e;
            off.x = -100.0;
            assert!(aq.time_is_up(&mut off, true));
            // And it does go away, toward the nearest side.
            assert!(Aquarium::facing_left(&e), "40 is nearer the left side");
            run(ctx, aq, 30.0, |_| {});
            assert!(get(aq, fish).is_none());
        });
    }

    #[test]
    fn a_creature_touching_a_stinger_darts_away() {
        quiet(|ctx, aq| {
            let coral = plant(aq, "pink-anemone", 60.0, a::STING);
            let w = get(aq, coral).unwrap();
            let fish = put(aq, "classic-5", (w.x - 3.0, w.y), false, 3.0, 0);
            let mut stung = false;
            run(ctx, aq, 3.0, |aq| {
                let e = get(aq, fish).unwrap();
                if matches!(e.act, Act::Stung { .. }) {
                    stung = true;
                    assert!(e.vx.abs() >= 9.0, "{}", e.vx);
                }
            });
            assert!(stung);
        });
    }

    #[test]
    fn plain_fish_share_flakes_and_scavengers_clear_the_floor() {
        quiet(|ctx, aq| {
            let f1 = put(aq, "classic-1", (40.0, 20.0), false, 3.0, 0);
            let f2 = put(aq, "classic-2", (44.0, 22.0), false, 3.0, 0);
            aq.add_food(50.0, 15.0, Food::Flake);
            aq.add_food(30.0, 15.0, Food::Flake);
            run(ctx, aq, 1.5, |_| {});
            let foods: Vec<u32> = [f1, f2]
                .into_iter()
                .filter_map(|id| match get(aq, id).unwrap().act {
                    Act::Eat { food, .. } => Some(food),
                    _ => None,
                })
                .collect();
            let unique: HashSet<u32> = foods.iter().copied().collect();
            assert!(!foods.is_empty());
            assert_eq!(unique.len(), foods.len(), "no two on the same flake");
            run(ctx, aq, 25.0, |_| {});
            let left = |aq: &Aquarium, what: Food| {
                aq.ents
                    .iter()
                    .filter(|e| e.kind == Kind::Food && e.who == what as u16)
                    .count()
            };
            assert_eq!(left(aq, Food::Flake), 0, "all eaten");
            // A crab walks to scrap lying on the floor.
            let crab = put(aq, "red-rock-crab", (20.0, 36.0), false, 3.0, a::SCAVENGER);
            aq.add_food(60.0, 38.0, Food::Scrap);
            for _ in 0..30 * 40 {
                aq.update(ctx, 1.0 / 30.0);
                if left(aq, Food::Scrap) == 0 {
                    break;
                }
            }
            assert_eq!(left(aq, Food::Scrap), 0, "the crab ate it");
            assert!(get(aq, crab).is_some());
        });
    }

    #[test]
    fn crawlers_face_off_and_turn_at_props() {
        quiet(|ctx, aq| {
            let c1 = put(aq, "red-rock-crab", (30.0, 38.0), false, 2.0, 0);
            let c2 = put(aq, "red-rock-crab", (50.0, 38.0), true, 2.0, 0);
            let mut faced = false;
            run(ctx, aq, 14.0, |aq| {
                faced |= matches!(get(aq, c1).unwrap().act, Act::FaceOff { .. });
            });
            assert!(faced);
            let (e1, e2) = (get(aq, c1).unwrap(), get(aq, c2).unwrap());
            assert!(
                Aquarium::facing_left(&e1) || !Aquarium::facing_left(&e2),
                "at least one turned around"
            );
            // A crawler walking into a prop (not its home) turns around.
            let rock = prop(aq, "barrel-stack", 80.0, 0);
            let r = get(aq, rock).unwrap();
            let c3 = put(aq, "red-rock-crab", (r.x - 12.0, 38.0), false, 3.0, 0);
            let mut turned = false;
            run(ctx, aq, 15.0, |aq| {
                turned |= Aquarium::facing_left(&get(aq, c3).unwrap());
            });
            assert!(turned);
        });
    }

    #[test]
    fn a_grazer_nibbles_at_a_plant_then_moves_on() {
        quiet(|ctx, aq| {
            let weed = plant(aq, "seaweed", 60.0, a::SHELTER);
            let w = get(aq, weed).unwrap();
            let fish = put(
                aq,
                "blue-tang",
                (w.x - 6.0, w.y + 2.0),
                false,
                3.0,
                a::GRAZER,
            );
            let mut grazed = 0.0;
            run(ctx, aq, 90.0, |aq| {
                if matches!(get(aq, fish).map(|e| e.act), Some(Act::Graze { .. })) {
                    grazed += 1.0 / 30.0;
                }
            });
            assert!((3.0..16.0).contains(&grazed), "{grazed}");
        });
    }

    #[test]
    fn the_curious_follow_a_lure() {
        quiet(|ctx, aq| {
            let diver = visit(aq, "scuba-diver", (60.0, 20.0), 1.0, a::LURE);
            let fish = put(aq, "classic-6", (40.0, 25.0), false, 3.0, a::CURIOUS);
            let mut near_for = 0;
            run(ctx, aq, 25.0, |aq| {
                let (e, d) = (get(aq, fish).unwrap(), get(aq, diver).unwrap());
                if matches!(e.act, Act::Look { .. }) && (e.x - d.x).abs() < d.w + 8.0 {
                    near_for += 1;
                }
            });
            assert!(near_for > 60, "{near_for}");
        });
    }

    #[test]
    fn a_pufferfish_puffs_up_and_is_left_alone() {
        let lib = library();
        // Only once some species has an alt shape.
        let Some(si) = lib.species.iter().position(|s| s.alt.is_some()) else {
            return;
        };
        let name = lib.species[si].def.name;
        quiet(|ctx, aq| {
            let pred = put(
                aq,
                "great-white-shark",
                (20.0, 20.0),
                false,
                15.0,
                a::PREDATOR,
            );
            set_size(aq, pred, 900);
            // Clear of the shark's nose, which reaches column 50.
            let fish = put(aq, name, (70.0, 22.0), false, 3.0, a::PUFF);
            set_size(aq, fish, 20);
            let w0 = get(aq, fish).unwrap().w;
            let mut puffed = false;
            run(ctx, aq, 6.0, |aq| {
                if let Some(e) = get(aq, fish) {
                    puffed |= matches!(e.act, Act::Puffed { .. }) && e.w > w0;
                }
            });
            assert!(puffed);
            assert!(get(aq, fish).is_some(), "it was not eaten");
        });
    }

    fn act_name(act: Act) -> &'static str {
        match act {
            Act::Idle => "idle",
            Act::Hunt { .. } => "hunt",
            Act::Flee { .. } => "flee",
            Act::Regroup { .. } => "regroup",
            Act::Shelter { .. } => "shelter",
            Act::Hidden { .. } => "hidden",
            Act::Puffed { .. } => "puffed",
            Act::Stung { .. } => "stung",
            Act::ToHome { .. } => "tohome",
            Act::AtHome { .. } => "athome",
            Act::Graze { .. } => "graze",
            Act::Clean { .. } => "clean",
            Act::Look { .. } => "look",
            Act::FaceOff { .. } => "faceoff",
            Act::Eat { .. } => "eat",
            Act::Leave => "leave",
        }
    }

    #[test]
    fn five_minutes_in_every_biome_keep_the_population_and_never_panic() {
        let mut seen: HashSet<&'static str> = HashSet::new();
        for (name, _) in a::BIOMES.iter().chain(&[("mixed", a::ALL)]) {
            let theme = crate::theme::Theme::classic();
            let mut t = toml::Table::new();
            t.insert("biome".into(), (*name).into());
            t.insert("events".into(), toml::Value::Float(0.0));
            let opts = crate::scene::Opts::new(INFO.options, t);
            let ctx = Ctx {
                theme: &theme,
                opts: &opts,
                bg: Color::Reset,
                seed: 21,
                fiesta: crate::fiesta::Fiesta::NONE,
            };
            let mut aq = Aquarium::new(&ctx);
            aq.retheme(&ctx);
            aq.resize(&ctx, 120, 40);
            let start = aq.world.as_ref().unwrap().groups.len();
            let mut c = Canvas::new(120, 40, Color::Reset);
            for i in 0..30 * 300 {
                if i % (30 * 40) == 0 {
                    // Feeding time now and then.
                    for k in 0..6 {
                        aq.add_food(30.0 + 12.0 * k as f32, 10.0, Food::Flake);
                    }
                }
                aq.update(&ctx, 1.0 / 30.0);
                if i % 211 == 0 {
                    c.clear();
                    aq.draw(&ctx, &mut c);
                }
                for e in &aq.ents {
                    if matches!(e.kind, Kind::Fish | Kind::Critter) {
                        seen.insert(act_name(e.act));
                        assert!(e.x.is_finite() && e.y.is_finite(), "{name}");
                    }
                }
            }
            let w = aq.world.as_ref().unwrap();
            let lasting = w.groups.iter().filter(|g| g.replace).count();
            assert_eq!(lasting, start, "{name}: groups");
        }
        eprintln!("acts seen: {seen:?}");
    }
}
