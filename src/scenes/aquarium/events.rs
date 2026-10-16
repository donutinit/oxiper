//! Now and then, something special: feeding time, a plankton bloom, a bait
//! ball, a migration, a rare sighting, a pod of whales; and the season's
//! decorations (Día de Muertos).
//!
//! One event runs at a time. The scheduler waits a quiet spell, picks an event
//! that fits the biome, and while it runs only spends a few cycles per step.
//! Everything an event brings carries its `tag`, and comes in groups that are
//! not replaced (`replace = false`), so the usual population is unchanged.

use std::f32::consts::TAU;

use super::assets::{
    ALWAYS, ARCTIC, Biomes, DEEP, FILTER, FRESH, KELP, LEGENDARY, Lane, MUERTOS, OCEAN, PREDATOR,
    REEF, Season, TANK, Traits,
};
use super::biome::{Food, Visit, between, hash, swim_motion, weighted};
use super::library::{SpeciesEntry, library};
use super::*;

/// Most entities the scene may hold before events hold back.
const ENTITY_CAP: usize = 440;
/// Least time between two rare sightings, in seconds.
const LEGEND_GAP: f32 = 1200.0;

/// What can happen.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Event {
    /// Someone sprinkles fish food from above.
    Feeding,
    /// A cloud of plankton, and those who feed on it.
    Bloom,
    /// Small fish ball up, then the hunters come.
    BaitBall,
    /// A long line of one species crosses.
    Migration,
    /// A legendary creature crosses once.
    Legend,
    /// Two or three big whales cross together.
    Pod,
}

/// How well an event suits a biome (0 if it does not).
fn fit(kind: Event, b: Biomes) -> u32 {
    let any = |m: Biomes| b & m != 0;
    match kind {
        Event::Feeding if any(TANK | FRESH) => 4,
        Event::Feeding if any(REEF) => 1,
        Event::Bloom if any(OCEAN | KELP | ARCTIC | REEF | DEEP) => 3,
        Event::BaitBall if any(OCEAN | REEF | KELP) => 3,
        Event::Migration => 2,
        Event::Pod if any(OCEAN | ARCTIC | KELP) => 2,
        _ => 0,
    }
}

/// The event that is going on.
struct Run {
    kind: Event,
    /// What its entities carry in `Ent::tag`.
    tag: u32,
    /// The biome it started in.
    biome: Biomes,
    /// Seconds since the start.
    t: f32,
    /// Feeding and bloom: how long the sprinkling lasts. Bait ball: when the
    /// hunters come. Migration and pod: seconds between spawns.
    a: f32,
    /// Bait ball: when it breaks up.
    b: f32,
    /// Countdown to the next spawn.
    timer: f32,
    /// How many to spawn in all (feeding, bloom, migration, pod) and how many
    /// have been. Bait ball: `done` says whether the hunters came.
    n: u32,
    done: u32,
    /// Feeding and bloom: where. Bait ball: the center's start (`x`), end
    /// (`x1`) and row (`y`). Migration: the row.
    x: f32,
    x1: f32,
    y: f32,
    /// Bloom: how wide the cloud is. Bait ball: seconds to swim in.
    spread: f32,
    /// Migration and pod: the way they go, and how fast (cells per second).
    left: bool,
    speed: f32,
    /// Migration: the species. Pod: a depth offset.
    si: usize,
    /// Migration: individuals in a group.
    size: u8,
}

impl Run {
    fn new(kind: Event, tag: u32, biome: Biomes) -> Run {
        Run {
            kind,
            tag,
            biome,
            t: 0.0,
            a: 0.0,
            b: 0.0,
            timer: 0.0,
            n: 0,
            done: 0,
            x: 0.0,
            x1: 0.0,
            y: 0.0,
            spread: 0.0,
            left: false,
            speed: 0.0,
            si: 0,
            size: 1,
        }
    }
}

/// The events' own state.
#[derive(Default)]
pub(super) struct Events {
    /// How often events happen (the `events` option); 0 never.
    pub(super) rate: f32,
    /// Seconds of quiet left (`None` before the first).
    wait: Option<f32>,
    run: Option<Run>,
    /// Last tag given.
    tags: u32,
    /// When the last rare sighting started.
    last_legend: Option<f32>,
    /// Events started so far.
    pub(super) started: u32,
    /// How far the current has carried the petals.
    drift: f32,
}

impl Events {
    pub(super) fn from_opts(o: &crate::scene::Opts) -> Events {
        let r = o.f("events");
        Events {
            rate: if r.is_finite() {
                r.clamp(0.0, 20.0)
            } else {
                1.0
            },
            ..Events::default()
        }
    }
}

/// A random legend among `items` (what, traits, weight); only LEGENDARY ones
/// are ever chosen.
fn pick_legend<T: Copy>(
    rng: &mut fastrand::Rng,
    items: impl Iterator<Item = (T, Traits, u8)>,
) -> Option<T> {
    let v: Vec<(T, u32)> = items
        .filter(|&(_, t, _)| t & LEGENDARY != 0)
        .map(|(x, _, w)| (x, w.max(1) as u32))
        .collect();
    weighted(rng, v.len(), |k| v[k].1).map(|k| v[k].0)
}

/// A legend that can come.
#[derive(Clone, Copy)]
enum Legend {
    Species(usize),
    Visitor(u16),
}

/// The season the options ask for: `auto` goes by today's date.
pub(super) fn season_from_opts(o: &crate::scene::Opts) -> Season {
    match o.s("season").trim().to_ascii_lowercase().as_str() {
        "muertos" | "dia-de-muertos" | "día de muertos" => MUERTOS,
        "auto" | "" => season_on(today()),
        _ => ALWAYS,
    }
}

/// The season on a (month, day).
fn season_on((month, day): (u32, u32)) -> Season {
    match (month, day) {
        (10, 28..) | (11, ..=3) => MUERTOS,
        _ => ALWAYS,
    }
}

/// Today's (month, day), in UTC.
fn today() -> (u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    civil((secs / 86_400) as i64)
}

/// (month, day) of a day counted from 1970-01-01 (Howard Hinnant's `civil_from_days`).
fn civil(days: i64) -> (u32, u32) {
    let z = days + 719_468;
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (month, day)
}

impl Aquarium {
    /// Once per step, before anything moves.
    pub(super) fn events(&mut self, dt: f32) {
        let (cur, _) = self.current();
        let Some(w) = self.world.as_mut() else {
            return;
        };
        if w.season == MUERTOS {
            // Kept small so that the petals' positions stay precise.
            w.ev.drift = (w.ev.drift + cur * dt) % 4096.0;
        }
        let rate = w.ev.rate;
        if rate <= 0.0 || w.biome == 0 {
            return;
        }
        if let Some(mut run) = w.ev.run.take() {
            if w.biome != run.biome {
                // The sea moved on: let the event's creatures go their way.
                self.release_ball(run.tag);
                self.finish_event(rate);
            } else if self.run_event(&mut run, dt) {
                if let Some(w) = self.world.as_mut() {
                    w.ev.run = Some(run);
                }
            } else {
                self.finish_event(rate);
            }
            return;
        }
        if w.ev.wait.is_none() {
            let first = between(&mut self.rng, 60.0, 120.0) / rate;
            w.ev.wait = Some(first);
        }
        let wait = w.ev.wait.get_or_insert(0.0);
        *wait -= dt;
        if *wait <= 0.0 {
            self.start_random_event(rate);
        }
    }

    /// An event is over: a quiet spell follows.
    fn finish_event(&mut self, rate: f32) {
        let wait = between(&mut self.rng, 120.0, 300.0) / rate.max(0.05);
        if let Some(w) = self.world.as_mut() {
            w.ev.run = None;
            w.ev.wait = Some(wait);
        }
    }

    /// Starts one event that fits the biome, or waits a little and tries again.
    fn start_random_event(&mut self, rate: f32) {
        let Some(w) = self.world.as_ref() else {
            return;
        };
        let biome = w.biome;
        let legend_ok = w.ev.last_legend.is_none_or(|t| self.time - t >= LEGEND_GAP);
        // About one event in six is a rare sighting.
        if legend_ok && self.rng.u32(0..6) == 0 && self.start_event(Event::Legend) {
            return;
        }
        let mut kinds: Vec<(Event, u32)> = [
            Event::Feeding,
            Event::Bloom,
            Event::BaitBall,
            Event::Migration,
            Event::Pod,
        ]
        .iter()
        .map(|&k| (k, fit(k, biome)))
        .filter(|&(_, f)| f > 0)
        .collect();
        while let Some(k) = weighted(&mut self.rng, kinds.len(), |k| kinds[k].1) {
            let (kind, _) = kinds.swap_remove(k);
            if self.start_event(kind) {
                return;
            }
        }
        // Nothing fits (empty pools, a tiny screen): look again soon.
        let wait = between(&mut self.rng, 20.0, 60.0) / rate.max(0.05);
        if let Some(w) = self.world.as_mut() {
            w.ev.wait = Some(wait);
        }
    }

    /// Starts `kind` now; returns whether it could (needs the right pools and room).
    pub(super) fn start_event(&mut self, kind: Event) -> bool {
        if self.ents.len() > ENTITY_CAP {
            return false;
        }
        let Some(w) = self.world.as_mut() else {
            return false;
        };
        if w.ev.run.is_some() || w.biome == 0 || (kind != Event::Legend && fit(kind, w.biome) == 0)
        {
            return false;
        }
        w.ev.tags = w.ev.tags.wrapping_add(1).max(1);
        let mut run = Run::new(kind, w.ev.tags, w.biome);
        let ok = match kind {
            Event::Feeding => self.begin_feeding(&mut run),
            Event::Bloom => self.begin_bloom(&mut run),
            Event::BaitBall => self.begin_ball(&mut run),
            Event::Migration => self.begin_migration(&mut run),
            Event::Legend => self.begin_legend(&mut run),
            Event::Pod => self.begin_pod(&mut run),
        };
        if ok {
            let now = self.time;
            if let Some(w) = self.world.as_mut() {
                w.ev.run = Some(run);
                w.ev.started += 1;
                if kind == Event::Legend {
                    w.ev.last_legend = Some(now);
                }
            }
        }
        ok
    }

    /// One step of the running event; returns whether it goes on.
    fn run_event(&mut self, run: &mut Run, dt: f32) -> bool {
        run.t += dt;
        match run.kind {
            Event::Feeding => {
                self.sprinkle(run, Food::Flake);
                run.t < run.a
            }
            Event::Bloom => {
                self.sprinkle(run, Food::Plankton);
                // The plankton thins out by itself in well under a minute.
                run.t < 60.0
            }
            Event::BaitBall => self.ball_step(run),
            Event::Migration => {
                run.timer -= dt;
                if run.timer <= 0.0 && run.done < run.n {
                    self.spawn_migrants(run);
                    run.done += 1;
                    run.timer += run.a;
                }
                run.done < run.n
            }
            Event::Legend => self.until_gone(run),
            Event::Pod => {
                run.timer -= dt;
                if run.timer <= 0.0 && run.done < run.n {
                    self.spawn_whale(run);
                    run.done += 1;
                    run.timer += run.a;
                }
                run.done < run.n || self.until_gone(run)
            }
        }
    }

    /// Whether anything of the event is still about (after its first moments).
    fn until_gone(&self, run: &Run) -> bool {
        run.t < 600.0 && (run.t < 1.0 || self.ents.iter().any(|e| e.tag == run.tag && !e.dead))
    }

    /// Marks everything spawned since `from` as the event's.
    fn tag_since(&mut self, from: usize, tag: u32) {
        for e in self.ents.get_mut(from..).unwrap_or_default() {
            e.tag = tag;
        }
    }

    /// A species from the pool that `ok` likes, by weight.
    fn pick_species(&mut self, ok: impl Fn(&SpeciesEntry) -> bool) -> Option<usize> {
        let lib = library();
        let w = self.world.as_ref()?;
        let k = weighted(&mut self.rng, w.swimmers.len(), |k| {
            let sp = &lib.species[w.swimmers[k] as usize];
            if ok(sp) {
                sp.def.weight.max(1) as u32
            } else {
                0
            }
        })?;
        Some(w.swimmers[k] as usize)
    }

    /// A data visitor from the pool that `ok` likes.
    fn pick_visitor(&mut self, ok: impl Fn(u16) -> bool) -> Option<u16> {
        let w = self.world.as_ref()?;
        let k = weighted(&mut self.rng, w.visit_pool.len(), |k| {
            match w.visit_pool[k] {
                Visit::Data(i) if ok(i) => 1,
                _ => 0,
            }
        })?;
        match w.visit_pool[k] {
            Visit::Data(i) => Some(i),
            Visit::Classic(_) => None,
        }
    }

    /// A group of species `si` that belongs to the event.
    fn event_group(&mut self, si: usize, n: u8, tag: u32) -> bool {
        let n0 = self.ents.len();
        let made = self.spawn_group(si, n, false, false).is_some();
        self.tag_since(n0, tag);
        made
    }

    /// A data visitor that belongs to the event. It does not bring the next
    /// passer-by when it leaves: the usual one is still on its way.
    fn event_visitor(&mut self, i: u16, tag: u32) -> Option<usize> {
        let n0 = self.ents.len();
        self.add_visitor(i);
        if self.ents.len() == n0 {
            return None;
        }
        self.tag_since(n0, tag);
        self.ents[n0].death = Death::None;
        Some(n0)
    }

    /// Spawns food in step with the event's clock: `run.n` pieces over `run.a` seconds.
    fn sprinkle(&mut self, run: &mut Run, what: Food) {
        let target = (run.n as f32 * (run.t / run.a.max(0.1)).min(1.0)) as u32;
        let top = self.water_top() as f32;
        let wmax = (self.w - 1).max(0) as f32;
        while run.done < target && self.ents.len() < ENTITY_CAP + 40 {
            let n0 = self.ents.len();
            match what {
                Food::Plankton => {
                    let x = run.x + between(&mut self.rng, -0.5, 0.5) * run.spread;
                    let room = (self.h as f32 - top - 3.0).max(1.0);
                    let y = top + 1.0 + self.rng.f32() * room;
                    self.add_food(x.clamp(0.0, wmax), y, what);
                }
                _ => {
                    let x = run.x + between(&mut self.rng, -5.0, 5.0);
                    let y = top + self.rng.f32();
                    self.add_food(x.clamp(0.0, wmax), y, what);
                }
            }
            self.tag_since(n0, run.tag);
            run.done += 1;
        }
    }

    // ---- feeding time ----

    fn begin_feeding(&mut self, run: &mut Run) -> bool {
        if !self.surface() || self.w < 12 || self.h < 14 {
            return false;
        }
        run.a = between(&mut self.rng, 3.0, 6.0);
        run.n = self.rng.u32(15..=40);
        run.x = between(&mut self.rng, 8.0, (self.w - 8) as f32);
        true
    }

    // ---- plankton bloom ----

    fn begin_bloom(&mut self, run: &mut Run) -> bool {
        if self.w < 10 || self.h - self.water_top() < 5 {
            return false;
        }
        run.a = 5.0;
        run.n = self.rng.u32(40..=80);
        run.spread = (self.w as f32 * 0.6).clamp(10.0, self.w as f32);
        run.x = between(&mut self.rng, 0.0, self.w as f32);
        let tag = run.tag;
        // Those who come to feed: a filterer, and a school of small fish.
        if let Some(si) = self.pick_species(|s| s.def.traits & FILTER != 0) {
            let (lo, hi) = library().species[si].def.school;
            let n = self.rng.u8(lo..=hi.max(lo)).clamp(1, 4);
            self.event_group(si, n, tag);
        } else if let Some(i) =
            self.pick_visitor(|i| library().visitors[i as usize].def.traits & FILTER != 0)
        {
            self.event_visitor(i, tag);
        }
        if let Some(si) = self.pick_species(|s| s.def.school.1 >= 4 && s.size <= 30) {
            let (lo, hi) = library().species[si].def.school;
            let n = self.rng.u8(lo..=hi.max(lo));
            self.event_group(si, n, tag);
        }
        true
    }

    // ---- bait ball ----

    fn begin_ball(&mut self, run: &mut Run) -> bool {
        let top = self.water_top();
        if self.w < 30 || self.h - top < 16 {
            return false;
        }
        let Some(si) = self.pick_species(|s| s.def.school.1 >= 6 && s.size <= 30) else {
            return false;
        };
        let lib = library();
        let (_, hi) = lib.species[si].def.school;
        let want = self.rng.usize(25..=45);
        let tag = run.tag;
        let mut have = 0;
        for _ in 0..10 {
            if have >= want || self.ents.len() > ENTITY_CAP - 40 {
                break;
            }
            let n0 = self.ents.len();
            self.event_group(si, hi, tag);
            have += self.ents.len() - n0;
        }
        if have == 0 {
            return false;
        }
        run.n = have as u32;
        // They swim in from a side to a spot in the middle of the water.
        let (w, h) = (self.w as f32, self.h as f32);
        let from_left = self.rng.bool();
        run.x = if from_left { -14.0 } else { w + 14.0 };
        run.x1 = between(&mut self.rng, w * 0.3, w * 0.7);
        run.y = between(&mut self.rng, top as f32 + 7.0, h - 7.0);
        run.spread = (run.x1 - run.x).abs() / 5.0;
        run.a = between(&mut self.rng, 10.0, 20.0);
        run.b = between(&mut self.rng, 40.0, 70.0);
        let sign = if self.rng.bool() { 1.0 } else { -1.0 };
        for e in &mut self.ents {
            if e.tag != tag || e.layer > 0 {
                continue;
            }
            e.motion = Motion::Orbit {
                cx: run.x,
                cy: run.y,
                rx: between(&mut self.rng, 3.0, 12.0),
                ry: between(&mut self.rng, 2.0, 5.0),
                omega: sign * between(&mut self.rng, 0.35, 0.6),
                phase: self.rng.f32() * TAU,
            };
            // Part of the ball is out of sight while it comes in.
            e.die_offscreen = false;
            e.ttl = 400.0;
        }
        true
    }

    fn ball_step(&mut self, run: &mut Run) -> bool {
        let t = run.t;
        let cx = if t < run.spread {
            run.x + (run.x1 - run.x).signum() * 5.0 * t
        } else {
            run.x1 + 5.0 * (0.15 * (t - run.spread)).sin()
        };
        let cy = run.y + 1.5 * (0.11 * t).sin();
        let mut orbiting = 0;
        for e in &mut self.ents {
            if e.tag == run.tag
                && !e.dead
                && let Motion::Orbit { cx: ox, cy: oy, .. } = &mut e.motion
            {
                *ox = cx;
                *oy = cy;
                orbiting += 1;
            }
        }
        if orbiting == 0 && t > 2.0 {
            return false;
        }
        if run.done == 0 && t >= run.a {
            run.done = 1;
            self.call_hunters(run.tag);
        }
        if t >= run.b {
            self.release_ball(run.tag);
            return false;
        }
        true
    }

    /// The predators arrive: a PREDATOR species group from the biome, or one of
    /// asciiquarium's own hunters.
    fn call_hunters(&mut self, tag: u32) {
        if let Some(si) = self.pick_species(|s| s.def.traits & PREDATOR != 0) {
            let (lo, hi) = library().species[si].def.school;
            let n = self.rng.u8(lo..=hi.max(lo)).min(3);
            self.event_group(si, n, tag);
            return;
        }
        if !self.random_objects || self.h < 30 || self.w < 60 {
            return;
        }
        let n0 = self.ents.len();
        if self.surface() && self.rng.bool() {
            self.add_dolphins();
        } else {
            self.add_shark();
        }
        let w = self.w as f32;
        for e in self.ents.get_mut(n0..).unwrap_or_default() {
            e.tag = tag;
            match e.kind {
                // The usual passer-by is still on its way: these bring none.
                Kind::Dolphin if e.death != Death::DolphinLead => e.death = Death::None,
                Kind::Shark => e.death = Death::None,
                // The teeth ride along with the shark and leave on their own.
                Kind::Teeth => e.ttl = (w + 70.0) / e.vx.abs().max(0.5) + 5.0,
                _ => {}
            }
        }
    }

    /// The ball breaks up: whoever still circles streams off to the nearest side.
    fn release_ball(&mut self, tag: u32) {
        let lib = library();
        let half = self.w as f32 / 2.0;
        let w = self.w as f32;
        for e in &mut self.ents {
            if e.tag != tag {
                continue;
            }
            let Motion::Orbit { cx, .. } = e.motion else {
                continue;
            };
            let left = cx < half;
            let style = lib
                .species
                .get(e.who as usize)
                .map_or(assets::Swim::Cruise, |s| s.def.swim);
            e.motion = swim_motion(&mut self.rng, style, e.y);
            let v = e.speed.max(2.0) * 1.4;
            e.vx = if left { -v } else { v };
            Self::face(e, left);
            e.die_offscreen = true;
            e.ttl = (w + 2.0 * e.w) / v * 3.0 + 60.0;
        }
    }

    // ---- migration ----

    fn begin_migration(&mut self, run: &mut Run) -> bool {
        let lib = library();
        let Some(si) = self.pick_species(|_| true) else {
            return false;
        };
        let sp = &lib.species[si];
        let d = sp.def;
        let count = self.rng.u32(20..=40);
        let size = (d.school.1.max(3) as i32).min(self.w / 10).max(1) as u32;
        run.n = count.div_ceil(size).min(20);
        run.size = size as u8;
        run.si = si;
        run.a = 60.0 / run.n as f32;
        run.left = self.rng.bool();
        run.speed = between(&mut self.rng, d.speed.0, d.speed.1) * TICK;
        let (ymin, ymax) = self.zone_range(d.zone, lib.sprites[sp.spr as usize].h);
        run.y = between(&mut self.rng, ymin, ymax.max(ymin));
        true
    }

    /// One more group of the procession: same side, same way, same pace and row.
    fn spawn_migrants(&mut self, run: &Run) {
        if self.ents.len() > ENTITY_CAP {
            return;
        }
        let n0 = self.ents.len();
        self.event_group(run.si, run.size, run.tag);
        let (w, top) = (self.w as f32, self.water_top() as f32);
        let h = self.h as f32;
        for (k, e) in self.ents.iter_mut().enumerate().skip(n0) {
            if e.layer > 0 {
                continue;
            }
            let behind = (k - n0) as f32 * (e.w + 1.0) + self.rng.f32() * (e.w * 0.5 + 3.0);
            e.x = if run.left {
                w - 2.0 + behind
            } else {
                1.0 - e.w - behind
            };
            e.y = (run.y + self.rng.i32(-2..=2) as f32).clamp(top, (h - e.h).max(top));
            e.vx = if run.left { -run.speed } else { run.speed };
            e.speed = run.speed;
            Self::face(e, run.left);
            if let Motion::Swim { base, .. } = &mut e.motion {
                *base = e.y;
            }
            e.ttl = (w + 2.0 * e.w + behind) / run.speed.max(0.5) * 3.0 + 60.0;
        }
    }

    // ---- rare sighting ----

    /// The legends that may come here now.
    fn legend_candidate(&mut self) -> Option<Legend> {
        let lib = library();
        let surface = self.surface();
        let room = self.h - self.water_top();
        let w = self.world.as_ref()?;
        let b = w.biome;
        let season = w.season;
        let species = lib
            .species
            .iter()
            .enumerate()
            .filter(|(_, sp)| {
                let d = sp.def;
                d.biomes & b != 0
                    && (d.season == ALWAYS || d.season == season)
                    && w.species.allows(d.name, true)
                    && lib.sprites[sp.spr as usize].h <= room
            })
            .map(|(i, sp)| (Legend::Species(i), sp.def.traits, sp.def.weight));
        let visitors = lib
            .visitors
            .iter()
            .enumerate()
            .filter(|(_, v)| {
                let d = v.def;
                let fits = match d.lane {
                    Lane::Sky | Lane::Boat | Lane::Float => surface,
                    Lane::Deep | Lane::Floor => lib.sprites[v.spr as usize].h < room,
                };
                fits && d.biomes & b != 0
                    && (d.season == ALWAYS || d.season == season)
                    && w.visitors.allows(d.name, true)
            })
            .map(|(i, v)| (Legend::Visitor(i as u16), v.def.traits, v.def.weight));
        pick_legend(&mut self.rng, species.chain(visitors))
    }

    fn begin_legend(&mut self, run: &mut Run) -> bool {
        match self.legend_candidate() {
            Some(Legend::Species(si)) => self.event_group(si, 1, run.tag),
            Some(Legend::Visitor(i)) => self.event_visitor(i, run.tag).is_some(),
            None => false,
        }
    }

    // ---- a pod ----

    /// Big visitors of the biome that live in whale country.
    fn is_whale(i: u16) -> bool {
        let n = library().visitors[i as usize].def.name;
        ["whale", "orca", "beluga", "narwhal"]
            .iter()
            .any(|k| n.contains(k))
    }

    fn begin_pod(&mut self, run: &mut Run) -> bool {
        if self.pick_visitor(Self::is_whale).is_none() {
            return false;
        }
        run.n = self.rng.u32(2..=3);
        run.a = between(&mut self.rng, 3.0, 6.0);
        run.left = self.rng.bool();
        run.si = self.rng.usize(0..run.n as usize);
        true
    }

    /// One more of the pod, at its own depth.
    fn spawn_whale(&mut self, run: &Run) {
        let lib = library();
        let Some(i) = self.pick_visitor(Self::is_whale) else {
            return;
        };
        let Some(k) = self.event_visitor(i, run.tag) else {
            return;
        };
        let v = &lib.visitors[i as usize];
        let (w, top) = (self.w as f32, self.water_top() as f32);
        let deep = v.def.lane == Lane::Deep;
        let slot = (run.done as usize + run.si) % run.n as usize;
        let jitter = self.rng.f32() * 0.6 + 0.2;
        let h = self.h as f32;
        let e = &mut self.ents[k];
        e.x = if run.left { w - 2.0 } else { 1.0 - e.w };
        let speed = e.vx.abs();
        e.vx = if run.left { -speed } else { speed };
        Self::face(e, run.left);
        if deep {
            let span = (h - e.h - top).max(0.0);
            e.y = (top + span * (slot as f32 + jitter) / run.n as f32).round();
        }
    }

    // ---- drawing ----

    /// Over everything else.
    pub(super) fn draw_event_overlay(&self, c: &mut Canvas) {
        let Some(w) = self.world.as_ref() else {
            return;
        };
        if let Some(run) = &w.ev.run
            && run.kind == Event::Feeding
        {
            self.draw_hand(c, run);
        }
        if w.season == MUERTOS {
            self.draw_petals(c, w.seed, w.ev.drift);
            let light = self.light();
            if light < 0.4 {
                self.draw_candles(c, w.seed, w.floor, light);
            }
        }
    }

    /// A hand over the water, pinching food into it.
    fn draw_hand(&self, c: &mut Canvas, run: &Run) {
        if run.t >= run.a || !self.surface() {
            return;
        }
        let x = run.x.round() as i32;
        let (col, attr) = self.pens[(W + BRIGHT) as usize];
        c.text_attr(x - 4, 2, "===\\", col, attr);
        c.text_attr(x - 1, 3, "()", col, attr);
        let (col, attr) = self.pens[Y as usize];
        let beat = (self.time * 8.0) as u64;
        for (k, dx) in [-1, 0, 1].into_iter().enumerate() {
            let r = hash(beat.wrapping_mul(3).wrapping_add(k as u64));
            if !r.is_multiple_of(3) {
                c.set_attr(
                    x + dx,
                    4,
                    [',', '\'', '.'][(r >> 8) as usize % 3],
                    col,
                    attr,
                );
            }
        }
    }

    /// Marigold petals on the waves, drifting with the current; sinking in the deep.
    fn draw_petals(&self, c: &mut Canvas, seed: u64, drift: f32) {
        let (w, h) = (self.w, self.h);
        if w <= 0 || h <= 0 {
            return;
        }
        let pens = [Y + BRIGHT, Y, R + BRIGHT];
        let petal = |r: u64| {
            let (col, attr) = self.pens[pens[(r >> 8) as usize % 3] as usize];
            (b"*',"[(r >> 12) as usize % 3] as char, col, attr)
        };
        if self.surface() {
            for i in 0..(w / 5).max(1) as u64 {
                let r = hash(seed ^ 0x4d41_5249 ^ i.wrapping_mul(0x9e37));
                let speed = 0.3 + (r >> 20 & 63) as f32 / 63.0 * 0.5;
                let x =
                    ((r % w as u64) as f32 + self.time * speed + drift * 0.6).rem_euclid(w as f32);
                let y = 5 + (r >> 32) as i32 % 4;
                let (ch, col, attr) = petal(r);
                c.set_attr(x as i32, y, ch, col, attr);
            }
        } else {
            for i in 0..((w * h) / 500).max(2) as u64 {
                let r = hash(seed ^ 0x4d41_5249 ^ i.wrapping_mul(0x9e37));
                let fall = 0.25 + (r >> 20 & 63) as f32 / 63.0 * 0.3;
                let y = ((r >> 32) as f32 % h as f32 + self.time * fall).rem_euclid(h as f32);
                let x = (r % w as u64) as f32 + (self.time * 0.4 + i as f32).sin() * 1.5 + drift;
                let (ch, col, attr) = petal(r);
                c.set_attr(x.rem_euclid(w as f32) as i32, y as i32, ch, col, attr);
            }
        }
    }

    /// Candle flames flickering on the floor row, in the gaps of the sand; more
    /// of them as it gets darker.
    fn draw_candles(&self, c: &mut Canvas, seed: u64, floor: bool, light: f32) {
        let (w, h) = (self.w, self.h);
        if w <= 0 || h < 3 {
            return;
        }
        let dark = (1.0 - light / 0.4).clamp(0.0, 1.0);
        let n = ((w / 18).max(1) as f32 * dark).ceil() as u64;
        for i in 0..n {
            let r = hash(seed ^ 0x00ca_4d1e ^ i.wrapping_mul(0x51ed));
            let mut x = (r % w as u64) as i32;
            if floor {
                // Where the backdrop leaves the cell empty.
                for _ in 0..w {
                    if hash(seed ^ x as u64) % 100 <= 34 {
                        break;
                    }
                    x = (x + 1) % w;
                }
            }
            let flick =
                (self.time * 5.0 + i as f32 * 1.7).sin() + (self.time * 13.0 + i as f32).sin();
            let pen = if flick > 0.3 { Y + BRIGHT } else { R + BRIGHT };
            let (col, attr) = self.pens[pen as usize];
            c.set_attr(x, h - 1, 'i', col, attr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_and_seasons() {
        assert_eq!(civil(0), (1, 1));
        // 2026-10-15 and 2024-02-29.
        assert_eq!(civil(20_741), (10, 15));
        assert_eq!(civil(19_782), (2, 29));
        assert_eq!(season_on((11, 2)), MUERTOS);
        assert_eq!(season_on((10, 31)), MUERTOS);
        assert_eq!(season_on((10, 15)), ALWAYS);
        assert_eq!(season_on((12, 25)), ALWAYS);
    }

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
        };
        let mut a = Aquarium::new(&ctx);
        a.retheme(&ctx);
        f(&ctx, &mut a);
    }

    fn s(v: &str) -> toml::Value {
        toml::Value::String(v.into())
    }

    fn rate(r: f32) -> toml::Value {
        toml::Value::Float(r as f64)
    }

    fn tagged(a: &Aquarium) -> usize {
        a.ents.iter().filter(|e| e.tag != 0).count()
    }

    fn running(a: &Aquarium) -> bool {
        a.world.as_ref().unwrap().ev.run.is_some()
    }

    #[test]
    fn rate_zero_never_starts() {
        scene(&[("biome", s("ocean")), ("events", rate(0.0))], |ctx, a| {
            a.resize(ctx, 120, 40);
            for _ in 0..600 * 10 {
                a.update(ctx, 0.1);
            }
            assert_eq!(a.world.as_ref().unwrap().ev.started, 0);
            assert_eq!(tagged(a), 0);
        });
    }

    #[test]
    fn a_high_rate_brings_several() {
        scene(
            &[("biome", s("ocean")), ("events", rate(20.0))],
            |ctx, a| {
                a.resize(ctx, 120, 40);
                for _ in 0..600 * 10 {
                    a.update(ctx, 0.1);
                }
                assert!(a.world.as_ref().unwrap().ev.started >= 3);
            },
        );
    }

    /// Starts `kind` in `biome`, runs it out and checks nothing of it stays.
    fn run_out(kind: Event, biome: &str) -> bool {
        let mut started = false;
        scene(&[("biome", s(biome)), ("events", rate(1.0))], |ctx, a| {
            a.resize(ctx, 140, 45);
            started = a.start_event(kind);
            if !started {
                return;
            }
            let mut t = 0.0;
            while running(a) {
                a.world.as_mut().unwrap().ev.wait = Some(1e9);
                a.update(ctx, 0.1);
                t += 0.1;
                assert!(t < 300.0, "{kind:?} in {biome} never ended");
                assert!(a.ents.len() < 500);
            }
            // Whatever is left leaves by itself.
            for _ in 0..4000 {
                a.world.as_mut().unwrap().ev.wait = Some(1e9);
                a.update(ctx, 0.1);
            }
            assert_eq!(tagged(a), 0, "{kind:?} in {biome} left things behind");
            let w = a.world.as_ref().unwrap();
            assert_eq!(w.groups.iter().filter(|g| !g.replace).count(), 0);
        });
        started
    }

    #[test]
    fn every_event_runs_out() {
        let cases: &[(Event, &[&str])] = &[
            (Event::Feeding, &["tank", "fresh", "reef"]),
            (Event::Bloom, &["ocean", "kelp", "arctic", "reef", "deep"]),
            (Event::BaitBall, &["ocean", "reef", "kelp"]),
            (Event::Migration, &["ocean", "deep", "tank"]),
            (Event::Pod, &["ocean", "arctic", "kelp"]),
        ];
        for (kind, biomes) in cases {
            let any = biomes
                .iter()
                .map(|b| run_out(*kind, b))
                .fold(false, |x, y| x | y);
            assert!(any, "{kind:?} never started");
        }
    }

    #[test]
    fn nothing_fits_where_it_should_not() {
        scene(&[("biome", s("deep"))], |ctx, a| {
            a.resize(ctx, 120, 40);
            assert!(!a.start_event(Event::Feeding));
            assert!(!a.start_event(Event::BaitBall));
        });
    }

    #[test]
    fn the_bait_ball_orbits_then_disperses() {
        scene(&[("biome", s("ocean")), ("events", rate(1.0))], |ctx, a| {
            a.resize(ctx, 140, 45);
            assert!(a.start_event(Event::BaitBall));
            let orbiting = |a: &Aquarium| {
                a.ents
                    .iter()
                    .filter(|e| matches!(e.motion, Motion::Orbit { .. }))
                    .count()
            };
            let n = orbiting(a);
            assert!(n >= 20, "{n} in the ball");
            for _ in 0..300 {
                a.world.as_mut().unwrap().ev.wait = Some(1e9);
                a.update(ctx, 0.1);
            }
            // Thirty seconds in: the ball is on screen, circling.
            assert!(orbiting(a) >= n / 2);
            let seen = a
                .ents
                .iter()
                .filter(|e| matches!(e.motion, Motion::Orbit { .. }) && e.x >= 0.0 && e.x < 140.0)
                .count();
            assert!(seen >= n / 3, "{seen} of {n} on screen");
            for _ in 0..500 {
                a.world.as_mut().unwrap().ev.wait = Some(1e9);
                a.update(ctx, 0.1);
            }
            assert_eq!(orbiting(a), 0);
            assert!(!running(a));
        });
    }

    #[test]
    fn legends_are_only_legendary() {
        let mut rng = fastrand::Rng::with_seed(1);
        let items = [
            ('a', 0, 5),
            ('b', LEGENDARY | PREDATOR, 3),
            ('c', FILTER, 9),
            ('d', LEGENDARY, 1),
        ];
        for _ in 0..200 {
            let p = pick_legend(&mut rng, items.iter().copied()).unwrap();
            assert!(p == 'b' || p == 'd');
        }
        assert!(pick_legend(&mut rng, items[..1].iter().copied()).is_none());
    }

    #[test]
    fn long_runs_stay_bounded_in_every_biome() {
        for (name, _) in assets::BIOMES.iter().chain(&[("mixed", assets::ALL)]) {
            scene(&[("biome", s(name)), ("events", rate(10.0))], |ctx, a| {
                a.resize(ctx, 120, 40);
                let mut c = Canvas::new(120, 40, Color::Reset);
                let mut most = 0;
                for i in 0..600 * 10 {
                    a.update(ctx, 0.1);
                    most = most.max(a.ents.len());
                    if i % 53 == 0 {
                        c.clear();
                        a.draw(ctx, &mut c);
                    }
                }
                assert!(most < 500, "{name}: {most} entities");
                assert!(a.world.as_ref().unwrap().ev.started > 0, "{name}");
            });
        }
    }

    #[test]
    fn tiny_screens_cope() {
        for (w, h) in [(1, 1), (8, 5), (30, 12), (200, 14)] {
            for name in ["ocean", "tank", "deep"] {
                scene(&[("biome", s(name)), ("events", rate(20.0))], |ctx, a| {
                    a.resize(ctx, w, h);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for _ in 0..1500 {
                        a.update(ctx, 0.2);
                        a.draw(ctx, &mut c);
                    }
                });
            }
        }
    }

    #[test]
    fn petals_only_in_season() {
        let count = |season: &str, biome: &str| {
            let mut n = 0;
            scene(
                &[
                    ("biome", s(biome)),
                    ("season", s(season)),
                    ("events", rate(0.0)),
                ],
                |ctx, a| {
                    a.resize(ctx, 120, 40);
                    a.update(ctx, 0.1);
                    let mut c = Canvas::new(120, 40, Color::Reset);
                    a.draw_event_overlay(&mut c);
                    n = c.to_text().chars().filter(|ch| "*',".contains(*ch)).count();
                },
            );
            n
        };
        assert!(count("muertos", "ocean") >= 10);
        assert!(count("muertos", "deep") >= 2);
        assert_eq!(count("none", "ocean"), 0);
    }

    #[test]
    fn candles_sit_on_the_floor_row() {
        scene(
            &[("biome", s("deep")), ("season", s("muertos"))],
            |ctx, a| {
                a.resize(ctx, 120, 40);
                let mut c = Canvas::new(120, 40, Color::Reset);
                a.draw_candles(&mut c, 7, true, 0.0);
                let text = c.to_text();
                let rows: Vec<&str> = text.lines().collect();
                assert!(rows[39].contains('i'));
                assert!(rows[..39].iter().all(|r| !r.contains('i')));
            },
        );
    }
}
