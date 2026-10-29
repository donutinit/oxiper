//! Now and then, something special: feeding time, a plankton bloom, a bait
//! ball, a migration, a rare sighting, a pod of whales; and the season's
//! decorations. Día de Muertos: petals on the waves, veladoras floating
//! among them, the path of petals down to the altar and the souls walking
//! it, candles on the floor at night. Navidad (`navidad.rs`): the series'
//! lights hung between the plants, esferas rising, the Niño Dios in the
//! manger, the posadas' procession at night. The season follows the holidays
//! (`season_of`), and comes and goes with them in `retheme`.
//!
//! One event runs at a time. The scheduler waits a quiet spell, picks an event
//! that fits the biome, and while it runs only spends a few cycles per step.
//! Everything an event brings carries its `tag`, and comes in groups that are
//! not replaced (`replace = false`), so the usual population is unchanged.

use std::f32::consts::{PI, TAU};

use super::assets::{
    ALWAYS, ARCTIC, Biomes, DEEP, FILTER, FRESH, KELP, LEGENDARY, Lane, MUERTOS, NAVIDAD, OCEAN,
    PREDATOR, REEF, Season, TANK, Traits,
};
use super::biome::{Food, Visit, between, hash, swim_motion, weighted};
use super::library::{SpeciesEntry, library};
use super::*;
use crate::fiesta::{self, Fiesta};

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

/// The season the options ask for, and how far it has come (0 to 1): `auto`
/// follows the holidays (`ctx.fiesta`, so the local date or the `fiestas`
/// setting), `muertos` and `navidad` are that holiday every day (as far as
/// it has come, or at its height), and anything else is no season at all.
pub(super) fn season_of(o: &crate::scene::Opts, f: Fiesta) -> (Season, f32) {
    // How far the holiday has come, or its height on another day.
    let level = |kind| {
        if f.is(kind) && f.level.is_finite() {
            f.level.clamp(0.0, 1.0)
        } else {
            1.0
        }
    };
    let (muertos, navidad) = (f.is(fiesta::Kind::Muertos), f.is(fiesta::Kind::Navidad));
    match o.s("season").trim().to_ascii_lowercase().as_str() {
        "muertos" | "dia-de-muertos" | "día de muertos" => (MUERTOS, level(fiesta::Kind::Muertos)),
        "navidad" | "posadas" | "guadalupe-reyes" => (NAVIDAD, level(fiesta::Kind::Navidad)),
        "auto" | "" if muertos => (MUERTOS, level(fiesta::Kind::Muertos)),
        "auto" | "" if navidad => (NAVIDAD, level(fiesta::Kind::Navidad)),
        _ => (ALWAYS, 0.0),
    }
}

/// The Navidad's days, when the season is the Navidad: whether it is the
/// posadas (a procession at night) and whether the Niño Dios has been born
/// (Nochebuena on). Asked for on another day it is the height of the
/// holiday: both.
pub(super) fn season_days(o: &crate::scene::Opts, f: Fiesta) -> (bool, bool) {
    if season_of(o, f).0 != NAVIDAD {
        (false, false)
    } else if f.is(fiesta::Kind::Navidad) {
        (f.posadas(), f.month == 1 || (f.month == 12 && f.day >= 24))
    } else {
        (true, true)
    }
}

/// How much of the plants are the season's own: all of the ofrenda's flowers,
/// but the Navidad leaves room for its kelp, which wears the lights.
pub(super) fn plant_share(season: Season, level: f32) -> f32 {
    match season {
        NAVIDAD => 0.12 + 0.25 * level,
        _ => 0.3 + 0.35 * level,
    }
}

/// What is set on the floor first in a season, the biggest that fits: the
/// name every such prop starts with.
pub(super) fn centerpiece(season: Season) -> &'static str {
    match season {
        MUERTOS => "ofrend",
        NAVIDAD => "nacimient",
        _ => "",
    }
}

impl Aquarium {
    /// Once per step, before anything moves.
    pub(super) fn events(&mut self, dt: f32) {
        let (cur, _) = self.current();
        let Some(w) = self.world.as_mut() else {
            return;
        };
        if w.season != ALWAYS {
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
        // The quiet spell between events is a clock: real time.
        let clock = self.clock;
        let Some(w) = self.world.as_mut() else {
            return;
        };
        let wait = w.ev.wait.get_or_insert(0.0);
        *wait -= dt * clock;
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
        // Swimmers that get somewhere: jellies drifting along make no procession.
        let goes = |sp: &SpeciesEntry| {
            matches!(
                sp.def.swim,
                assets::Swim::Cruise | assets::Swim::Wave | assets::Swim::Dart
            ) && sp.def.speed.1 >= 0.5
        };
        // In season, more and more often it is the season's own procession.
        let level = self
            .world
            .as_ref()
            .and_then(|w| (w.season != ALWAYS).then_some(w.level));
        let ours = match level {
            Some(l) if self.rng.f32() < 0.25 + 0.5 * l => {
                self.pick_species(|sp| goes(sp) && sp.def.season != ALWAYS)
            }
            _ => None,
        };
        let Some(si) = ours.or_else(|| self.pick_species(goes)) else {
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
        match w.season {
            MUERTOS => {
                self.draw_petals(c, w.seed, w.ev.drift, w.level);
                if self.surface() {
                    self.draw_floating_candles(c, w.seed, w.ev.drift, w.level);
                }
                let light = self.light();
                if light < 0.4 {
                    self.draw_candles(c, w.seed, w.floor, light, w.level);
                }
            }
            NAVIDAD => {
                if w.nino {
                    self.draw_nino(c);
                }
                if w.posadas && self.light() < 0.45 {
                    self.draw_procession(c, w.seed, w.level);
                }
            }
            _ => {}
        }
    }

    /// Behind everything but the backdrop: the season's path of petals, or
    /// the Navidad's esferas and the series hung between the plants.
    pub(super) fn draw_event_backdrop(&self, c: &mut Canvas) {
        let Some(w) = self.world.as_ref() else {
            return;
        };
        match w.season {
            MUERTOS => self.draw_path(c, w.seed, w.level),
            NAVIDAD => {
                self.draw_esferas(c, w.seed, w.ev.drift, w.level);
                self.draw_lights(c, w.seed, w.level);
            }
            _ => {}
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

    /// Marigold petals on the waves, drifting with the current; sinking in the
    /// deep. More of them as the holiday comes.
    fn draw_petals(&self, c: &mut Canvas, seed: u64, drift: f32, level: f32) {
        let (w, h) = (self.w, self.h);
        if w <= 0 || h <= 0 {
            return;
        }
        let pens = [Y + BRIGHT, Y, R + BRIGHT];
        let petal = |r: u64| {
            let (col, attr) = self.pens[pens[(r >> 8) as usize % 3] as usize];
            (b"*',"[(r >> 12) as usize % 3] as char, col, attr)
        };
        let more = 0.5 + 0.5 * level;
        if self.surface() {
            for i in 0..((w / 5) as f32 * more).max(1.0) as u64 {
                let r = hash(seed ^ 0x4d41_5249 ^ i.wrapping_mul(0x9e37));
                let speed = 0.3 + (r >> 20 & 63) as f32 / 63.0 * 0.5;
                let x =
                    ((r % w as u64) as f32 + self.time * speed + drift * 0.6).rem_euclid(w as f32);
                let y = 5 + (r >> 32) as i32 % 4;
                let (ch, col, attr) = petal(r);
                c.set_attr(x as i32, y, ch, col, attr);
            }
        } else {
            for i in 0..(((w * h) / 500) as f32 * more).max(2.0) as u64 {
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
    fn draw_candles(&self, c: &mut Canvas, seed: u64, floor: bool, light: f32, level: f32) {
        let (w, h) = (self.w, self.h);
        if w <= 0 || h < 3 {
            return;
        }
        let dark = (1.0 - light / 0.4).clamp(0.0, 1.0);
        let n = ((w / 18).max(1) as f32 * dark * (0.5 + level)).ceil() as u64;
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
            let (col, attr) = self.glow(pen);
            c.set_attr(x, h - 1, 'i', col, attr);
        }
    }

    /// Veladoras floating on the waves, as on the lake of Pátzcuaro on the
    /// night of the dead: a candle on a raft of marigolds, its flame
    /// flickering, drifting with the current; at night the flame lights the
    /// water under it. They pass behind the boats and the birds.
    fn draw_floating_candles(&self, c: &mut Canvas, seed: u64, drift: f32, level: f32) {
        let w = self.w;
        if w < 8 || self.h < 9 {
            return;
        }
        let n = ((w as f32 / 22.0) * (0.4 + 0.8 * level)).round().max(1.0) as u64;
        let night = self.light() < 0.45;
        let lap = (w + 8) as f32;
        // Marigold rafts, and the colored glass of the veladora.
        const RAFT: [(char, u8); 5] = [
            ('.', Y),
            ('@', Y + BRIGHT),
            ('|', W + BRIGHT),
            ('@', Y + BRIGHT),
            ('.', Y),
        ];
        let glass = [M + BRIGHT, R + BRIGHT, C + BRIGHT, G + BRIGHT, Y + BRIGHT];
        'candles: for i in 0..n {
            let r = hash(seed ^ 0x7e1a_7e1a ^ i.wrapping_mul(0x51ed));
            // Spread out along the water, drifting together, each its own way a little.
            let gap = lap / n as f32;
            let own = ((r >> 20) & 255) as f32 / 255.0 * 0.5 * gap
                + (self.time * 0.05 + i as f32 * 2.1).sin() * 0.2 * gap;
            let x =
                (i as f32 * gap + own + self.time * 0.2 + drift * 0.5).rem_euclid(lap) as i32 - 4;
            // Out of the way of whatever sails or swims on the surface.
            for e in &self.ents {
                let surface = matches!(
                    e.kind,
                    Kind::Visitor | Kind::Dolphin | Kind::Hook | Kind::Line
                ) && e.y < 7.0
                    && e.y + e.h > 3.0;
                if surface && !e.dead && e.x < (x + 6) as f32 && e.x + e.w > (x - 1) as f32 {
                    continue 'candles;
                }
            }
            let flick =
                (self.time * 6.0 + i as f32 * 2.3).sin() + (self.time * 15.0 + i as f32).sin();
            let flame = if flick > 0.8 {
                '('
            } else if flick < -0.8 {
                ')'
            } else {
                '|'
            };
            let (col, attr) = self.glow(if flick > -0.2 { Y + BRIGHT } else { R + BRIGHT });
            c.set_attr(x + 2, 3, flame, col, attr);
            let (col, attr) = self.glow(glass[(r >> 40) as usize % glass.len()]);
            c.set_attr(x + 1, 4, '[', col, attr);
            c.set_attr(x + 3, 4, ']', col, attr);
            let (col, attr) = self.glow(W + BRIGHT);
            c.set_attr(x + 2, 4, '#', col, attr);
            for (dx, &(ch, pen)) in RAFT.iter().enumerate() {
                let (col, attr) = self.glow(pen);
                let ch = if dx == 2 { '_' } else { ch };
                c.set_attr(x + dx as i32, 5, ch, col, attr);
            }
            if night {
                // The flame's reflection, trembling on the water.
                let (col, attr) = self.glow(if flick > 0.0 { Y + BRIGHT } else { Y });
                c.set_attr(x + 2 + (flick > 1.2) as i32, 6, '~', col, attr);
                c.set_attr(x + 2, 7, '.', col, attr);
            }
        }
    }

    /// The path of marigold petals that leads the souls home: from the
    /// surface down to the altar on the floor, the petals flowing along it.
    /// At night the souls themselves come down it, small lights.
    fn draw_path(&self, c: &mut Canvas, seed: u64, level: f32) {
        let lib = library();
        let Some(altar) = self.ents.iter().find(|e| {
            e.kind == Kind::Decor
                && !e.dead
                && lib
                    .decor
                    .get(e.who as usize)
                    .is_some_and(|d| d.name.starts_with("ofrend"))
        }) else {
            return;
        };
        let (w, h) = (self.w as f32, self.h as f32);
        let y0 = self.water_top() as f32;
        let y1 = altar.y.floor() - 1.0;
        let rows = y1 - y0;
        if rows < 3.0 || w < 10.0 || h < 6.0 {
            return;
        }
        let x1 = altar.x + altar.w / 2.0;
        // It comes in from one side or the other, and meanders.
        let side = if hash(seed ^ 0xca31) & 1 == 0 {
            -1.0
        } else {
            1.0
        };
        let mut x0 = x1 + side * (w / 4.0).min(28.0);
        if x0 < 2.0 || x0 > w - 3.0 {
            x0 = x1 - side * (w / 4.0).min(28.0);
        }
        let x0 = x0.clamp(1.0, (w - 2.0).max(1.0));
        let amp = (w / 18.0).min(6.0);
        let ph = (hash(seed ^ 0xca32) % 628) as f32 / 100.0;
        let at = |t: f32| {
            let s = t * t * (3.0 - 2.0 * t);
            let x = x0 + (x1 - x0) * s + amp * (t * PI * 1.5 + ph).sin() * (t * PI).sin();
            (x, y0 + rows * t)
        };
        // A ribbon three petals wide, thicker in the middle and as the
        // holiday comes, a step for every cell along it; its pattern flows
        // down toward the altar a step at a time.
        let steps = (rows + (x1 - x0).abs() + 2.0 * amp).min(2000.0) as i64;
        let flow = (self.time * 0.8 * steps as f32 / rows).floor() as i64;
        let dense = [0.25 + 0.3 * level, 0.5 + 0.35 * level, 0.25 + 0.3 * level];
        let pens = [Y + BRIGHT, Y + BRIGHT, Y, R + BRIGHT];
        for i in 0..steps {
            let (x, y) = at(i as f32 / steps as f32);
            let key = ((i - flow) as u64).wrapping_mul(0x9e37);
            for (k, &p) in dense.iter().enumerate() {
                let r = hash(seed ^ 0x9e7a_1000 ^ key ^ k as u64);
                if ((r & 1023) as f32) >= p * 1024.0 {
                    continue;
                }
                let ch = b"**,'@."[(r >> 16) as usize % 6] as char;
                let (col, attr) = self.glow(pens[(r >> 24) as usize % 4]);
                c.set_attr(x.round() as i32 + k as i32 - 1, y as i32, ch, col, attr);
            }
        }
        // The souls walk it in the dark, more of them as the holiday comes.
        if self.light() < 0.45 {
            let souls = 1 + (level * 4.0) as u64;
            for j in 0..souls {
                let t = ((j as f32 + 0.37) / souls as f32 + self.time * 0.008).fract();
                let (x, y) = at(t);
                let pulse = (self.time * 2.0 + j as f32 * 1.9).sin();
                let (col, attr) = self.glow(W + BRIGHT);
                c.set_attr(
                    x.round() as i32,
                    y as i32,
                    if pulse > 0.3 { '*' } else { 'o' },
                    col,
                    attr,
                );
                let (tx, ty) = at((t - 0.6 / rows).max(0.0));
                if (ty as i32) < y as i32 {
                    let (col, attr) = self.glow(Y);
                    c.set_attr(tx.round() as i32, ty as i32, '.', col, attr);
                }
            }
        }
    }

    /// After a theme change, or when the holiday comes or goes: when the
    /// season changes, the floor is set again (its plants and props) and
    /// what may come follows; whoever is swimming finishes the crossing.
    pub(super) fn retheme_season(&mut self, ctx: &Ctx) {
        let Some(w) = self.world.as_mut() else {
            return;
        };
        let (season, level) = season_of(ctx.opts, ctx.fiesta);
        w.level = level;
        (w.posadas, w.nino) = season_days(ctx.opts, ctx.fiesta);
        if season == w.season {
            return;
        }
        w.season = season;
        if w.biome == 0 {
            return;
        }
        self.refresh_pools();
        self.ents
            .retain(|e| !matches!(e.kind, Kind::Seaweed | Kind::Decor));
        self.add_decor();
        self.add_all_plants();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_season_follows_the_holidays() {
        let opts = |season: &str| {
            let mut t = toml::Table::new();
            t.insert("season".into(), s(season));
            crate::scene::Opts::new(INFO.options, t)
        };
        let (auto, none, muertos) = (opts("auto"), opts("none"), opts("muertos"));
        let height = Fiesta::of_day(11, 2);
        let first = Fiesta::of_day(10, 28);
        assert_eq!(season_of(&auto, Fiesta::NONE), (ALWAYS, 0.0));
        assert_eq!(season_of(&auto, height), (MUERTOS, 1.0));
        assert_eq!(season_of(&auto, first), (MUERTOS, 0.25));
        assert_eq!(season_of(&none, height), (ALWAYS, 0.0));
        // Asked for by name, it comes on any day: at its height, or as far
        // as the holiday has come.
        assert_eq!(season_of(&muertos, Fiesta::NONE), (MUERTOS, 1.0));
        assert_eq!(season_of(&muertos, first), (MUERTOS, 0.25));
        // The Navidad, from the Guadalupe to the Reyes; Muertos is still Muertos.
        let navidad = opts("navidad");
        assert_eq!(season_of(&auto, Fiesta::of_day(12, 24)), (NAVIDAD, 1.0));
        assert_eq!(season_of(&auto, Fiesta::of_day(1, 3)), (NAVIDAD, 0.5));
        assert_eq!(season_of(&none, Fiesta::of_day(12, 24)), (ALWAYS, 0.0));
        assert_eq!(season_of(&navidad, Fiesta::NONE), (NAVIDAD, 1.0));
        assert_eq!(season_of(&navidad, Fiesta::of_day(1, 3)), (NAVIDAD, 0.5));
        assert_eq!(season_of(&muertos, Fiesta::of_day(12, 24)), (MUERTOS, 1.0));
        assert_eq!(season_of(&navidad, height), (NAVIDAD, 1.0));
        // Holidays this scene has no costume for are ordinary days.
        for day in [(9, 15), (2, 2), (11, 20), (4, 30)] {
            assert_eq!(
                season_of(&auto, Fiesta::of_day(day.0, day.1)),
                (ALWAYS, 0.0)
            );
        }
    }

    #[test]
    fn the_navidads_days() {
        let o = |season: &str| {
            let mut t = toml::Table::new();
            t.insert("season".into(), s(season));
            crate::scene::Opts::new(INFO.options, t)
        };
        let (auto, navidad) = (o("auto"), o("navidad"));
        let days = |o: &crate::scene::Opts, m, d| season_days(o, Fiesta::of_day(m, d));
        // (the posadas, the Niño Dios born)
        assert_eq!(days(&auto, 12, 13), (false, false));
        assert_eq!(days(&auto, 12, 20), (true, false));
        assert_eq!(days(&auto, 12, 24), (true, true));
        assert_eq!(days(&auto, 12, 28), (false, true));
        assert_eq!(days(&auto, 1, 3), (false, true));
        assert_eq!(days(&auto, 11, 2), (false, false));
        assert_eq!(days(&auto, 6, 15), (false, false));
        // Asked for on an ordinary day, it is Nochebuena.
        assert_eq!(days(&navidad, 6, 15), (true, true));
        assert_eq!(days(&o("none"), 12, 24), (false, false));
    }

    fn scene(opts: &[(&str, toml::Value)], f: impl FnOnce(&Ctx, &mut Aquarium)) {
        festive_scene(opts, Fiesta::NONE, "classic", f);
    }

    /// A scene on a holiday (or none), under a theme.
    fn festive_scene(
        opts: &[(&str, toml::Value)],
        fiesta: Fiesta,
        theme: &str,
        f: impl FnOnce(&Ctx, &mut Aquarium),
    ) {
        let theme = crate::theme::Theme::builtin(theme).unwrap();
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
            fiesta,
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
            // The ball's members (creatures circling their homes do not count).
            let orbiting = |a: &Aquarium| {
                a.ents
                    .iter()
                    .filter(|e| e.tag != 0 && matches!(e.motion, Motion::Orbit { .. }))
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
                .filter(|e| {
                    e.tag != 0
                        && matches!(e.motion, Motion::Orbit { .. })
                        && e.x >= 0.0
                        && e.x < 140.0
                })
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
                a.draw_candles(&mut c, 7, true, 0.0, 1.0);
                let text = c.to_text();
                let rows: Vec<&str> = text.lines().collect();
                assert!(rows[39].contains('i'));
                assert!(rows[..39].iter().all(|r| !r.contains('i')));
            },
        );
    }

    // ---- Día de Muertos ----

    /// Names of the props on the floor, and whether each is of the season.
    fn props(a: &Aquarium) -> Vec<(&'static str, bool)> {
        let lib = library();
        a.ents
            .iter()
            .filter(|e| e.kind == Kind::Decor && e.who != NOBODY)
            .map(|e| {
                let d = &lib.decor[e.who as usize];
                (d.name, d.season != ALWAYS)
            })
            .collect()
    }

    /// How many of the creatures, plants and props are of the season.
    fn seasonal(a: &Aquarium) -> usize {
        let lib = library();
        a.ents
            .iter()
            .filter(|e| e.who != NOBODY)
            .filter(|e| match e.kind {
                Kind::Fish | Kind::Critter => lib.species[e.who as usize].def.season != ALWAYS,
                Kind::Seaweed => lib.plants[e.who as usize].season != ALWAYS,
                Kind::Decor => lib.decor[e.who as usize].season != ALWAYS,
                Kind::Visitor => lib.visitors[e.who as usize].def.season != ALWAYS,
                _ => false,
            })
            .count()
    }

    /// A frame after `secs` seconds (at night when `night`), as text.
    fn frame(ctx: &Ctx, a: &mut Aquarium, w: u16, h: u16, secs: f32, night: bool) -> String {
        for _ in 0..(secs * 10.0) as usize {
            a.update(ctx, 0.1);
        }
        if night && let Some(world) = a.world.as_mut() {
            world.env.force_night();
        }
        let mut c = Canvas::new(w, h, Color::Reset);
        a.draw(ctx, &mut c);
        c.to_text()
    }

    #[test]
    fn the_day_of_the_dead_comes_on_its_days() {
        for (day, (w, h)) in [
            ((11, 2), (120, 40)),
            ((10, 28), (120, 40)),
            ((11, 2), (80, 24)),
        ] {
            for biome in ["reef", "tank", "ocean", "fresh", "kelp", "arctic"] {
                let fiesta = Fiesta::of_day(day.0, day.1);
                festive_scene(&[("biome", s(biome))], fiesta, "classic", |ctx, a| {
                    a.resize(ctx, w, h);
                    let text = frame(ctx, a, w, h, 2.0, true);
                    // The altar comes first, the path of petals leads to it,
                    // and veladoras float on the water.
                    let p = props(a);
                    assert!(
                        p.first().is_some_and(|d| d.0.starts_with("ofrend")),
                        "{biome} {day:?}: {p:?}"
                    );
                    assert!(text.contains("[#]"), "{biome} {day:?}: no veladoras");
                    assert!(seasonal(a) >= 2, "{biome} {day:?}");
                });
            }
        }
        // The deep has no surface, but an altar and souls coming down to it.
        festive_scene(
            &[("biome", s("deep"))],
            Fiesta::of_day(11, 2),
            "dracula",
            |ctx, a| {
                a.resize(ctx, 100, 40);
                let text = frame(ctx, a, 100, 40, 2.0, false);
                assert!(
                    props(a).iter().any(|d| d.0 == "ofrendita"),
                    "{:?}",
                    props(a)
                );
                assert!(!text.contains("[#]"));
                assert!(text.contains('o') || text.contains('*'));
            },
        );
    }

    #[test]
    fn ordinary_days_have_none_of_it() {
        for biome in ["reef", "tank", "ocean", "deep", "mixed"] {
            festive_scene(&[("biome", s(biome))], Fiesta::NONE, "classic", |ctx, a| {
                a.resize(ctx, 120, 40);
                for _ in 0..10 {
                    let text = frame(ctx, a, 120, 40, 30.0, true);
                    assert!(!text.contains("[#]"), "{biome}");
                    assert_eq!(seasonal(a), 0, "{biome}");
                }
                assert_eq!(a.world.as_ref().unwrap().season, ALWAYS);
            });
        }
    }

    #[test]
    fn the_holiday_comes_and_goes_with_retheme() {
        let theme = crate::theme::Theme::classic();
        let mut t = toml::Table::new();
        t.insert("biome".into(), s("tank"));
        let opts = crate::scene::Opts::new(INFO.options, t);
        let ctx = |fiesta| Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 5,
            fiesta,
        };
        let (off, on) = (ctx(Fiesta::NONE), ctx(Fiesta::of_day(11, 2)));
        let mut a = Aquarium::new(&off);
        a.resize(&off, 120, 40);
        a.retheme(&off);
        assert_eq!(seasonal(&a), 0);
        // Midnight, October 31 to November 1... and the floor is set at once.
        a.retheme(&on);
        assert!(props(&a).first().is_some_and(|d| d.0.starts_with("ofrend")));
        let text = frame(&on, &mut a, 120, 40, 120.0, true);
        assert!(text.contains("[#]"));
        let lib = library();
        let creatures = |a: &Aquarium| {
            a.ents
                .iter()
                .filter(|e| matches!(e.kind, Kind::Fish | Kind::Critter))
                .filter(|e| lib.species[e.who as usize].def.season != ALWAYS)
                .count()
        };
        assert!(creatures(&a) > 0, "the season's creatures came");
        // And on November 4 it is all gone; the last of the season's
        // creatures finish their crossing.
        a.retheme(&off);
        assert!(props(&a).iter().all(|d| !d.1));
        let text = frame(&off, &mut a, 120, 40, 600.0, true);
        assert!(!text.contains("[#]"));
        assert_eq!(seasonal(&a), 0);
        // A theme change on an ordinary day changes nothing.
        let n = a.ents.len();
        a.retheme(&off);
        assert_eq!(a.ents.len(), n);
    }

    #[test]
    fn the_path_of_petals_leads_down_to_the_altar() {
        let lib = library();
        for day in [(11, 2), (10, 28)] {
            for biome in ["reef", "tank", "deep"] {
                let fiesta = Fiesta::of_day(day.0, day.1);
                let opts = [("biome", s(biome)), ("day_length", rate(0.0))];
                festive_scene(&opts, fiesta, "classic", |ctx, a| {
                    a.resize(ctx, 120, 40);
                    a.update(ctx, 0.1);
                    let altar = a
                        .ents
                        .iter()
                        .find(|e| {
                            e.kind == Kind::Decor
                                && lib
                                    .decor
                                    .get(e.who as usize)
                                    .is_some_and(|d| d.name.starts_with("ofrend"))
                        })
                        .unwrap_or_else(|| panic!("{biome} {day:?}: no altar"));
                    let (top, bottom) = (a.water_top() as usize, altar.y.floor() as usize);
                    let mut c = Canvas::new(120, 40, Color::Reset);
                    a.draw_event_backdrop(&mut c);
                    let text = c.to_text();
                    let rows: Vec<&str> = text.lines().collect();
                    // Petals from the water to the altar, and not a cell above or below.
                    let petals = |r: &str| r.chars().filter(|ch| "*',.@".contains(*ch)).count();
                    assert!(
                        rows[..top].iter().all(|r| petals(r) == 0),
                        "{biome} {day:?}"
                    );
                    assert!(
                        rows[bottom..].iter().all(|r| petals(r) == 0),
                        "{biome} {day:?}"
                    );
                    let lit = rows[top..bottom].iter().filter(|r| petals(r) > 0).count();
                    assert!(lit * 4 >= (bottom - top) * 3, "{biome} {day:?}: {lit} rows");
                    // At night the souls come down it, small white lights, and only then.
                    let white = a.glow(W + BRIGHT).0;
                    let souls = |c: &Canvas| {
                        let cells = (0..40).flat_map(|y| (0..120).map(move |x| (x, y)));
                        cells
                            .filter_map(|(x, y)| c.get(x, y))
                            .filter(|cell| cell.fg == white && "*o".contains(cell.ch))
                            .count()
                    };
                    let by_day = souls(&c);
                    a.world.as_mut().unwrap().env.force_night();
                    let mut c = Canvas::new(120, 40, Color::Reset);
                    a.draw_event_backdrop(&mut c);
                    let by_night = souls(&c);
                    // (The deep is always dark.)
                    assert_eq!(by_day > 0, biome == "deep", "{biome} {day:?}");
                    assert!(by_night >= 1, "{biome} {day:?}: no souls");
                });
            }
        }
        // An ordinary day has no path, and a sea without an altar none either.
        festive_scene(
            &[("biome", s("reef"))],
            Fiesta::NONE,
            "classic",
            |ctx, a| {
                a.resize(ctx, 120, 40);
                let mut c = Canvas::new(120, 40, Color::Reset);
                a.draw_event_backdrop(&mut c);
                assert!(c.to_text().trim().is_empty());
            },
        );
        festive_scene(
            &[("biome", s("reef")), ("decor", s("none"))],
            Fiesta::of_day(11, 2),
            "classic",
            |ctx, a| {
                a.resize(ctx, 120, 40);
                let mut c = Canvas::new(120, 40, Color::Reset);
                a.draw_event_backdrop(&mut c);
                assert!(c.to_text().trim().is_empty());
            },
        );
    }

    #[test]
    fn more_of_it_as_the_holiday_comes() {
        // The share of new groups that are of the season.
        let share = |day: (u32, u32)| {
            let mut ours = 0;
            festive_scene(
                &[("biome", s("reef"))],
                Fiesta::of_day(day.0, day.1),
                "classic",
                |ctx, a| {
                    a.resize(ctx, 160, 50);
                    let lib = library();
                    for _ in 0..800 {
                        let n = a.ents.len();
                        a.add_group(false, true);
                        ours += usize::from(
                            a.ents
                                .get(n)
                                .is_some_and(|e| lib.species[e.who as usize].def.season != ALWAYS),
                        );
                    }
                },
            );
            ours as f32 / 800.0
        };
        let (first, height) = (share((10, 28)), share((11, 2)));
        assert!(
            0.15 < first && first < height && height < 0.75,
            "{first} {height}"
        );
    }

    #[test]
    fn the_holiday_survives_odd_screens_and_long_steps() {
        for theme in ["classic", "dracula"] {
            for (w, h) in [
                (1, 1),
                (5, 3),
                (12, 9),
                (30, 12),
                (40, 14),
                (300, 90),
                (300, 10),
            ] {
                for biome in ["random", "deep", "tank", "mixed"] {
                    let opts = [("biome", s(biome)), ("events", rate(20.0))];
                    festive_scene(&opts, Fiesta::of_day(11, 2), theme, |ctx, a| {
                        a.resize(ctx, w, h);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for _ in 0..40 {
                            a.update(ctx, 5.0);
                            a.draw(ctx, &mut c);
                        }
                        a.retheme(ctx);
                        a.draw(ctx, &mut c);
                    });
                }
            }
        }
    }
    // ---- Navidad ----

    /// The nacimiento on the floor, if any: its top left corner and its name.
    fn nacimiento(a: &Aquarium) -> Option<(i32, i32, &'static str)> {
        let lib = library();
        a.ents
            .iter()
            .filter(|e| e.kind == Kind::Decor && e.who != NOBODY)
            .find_map(|e| {
                let name = lib.decor[e.who as usize].name;
                name.starts_with("nacimient")
                    .then(|| (e.x.floor() as i32, e.y.floor() as i32, name))
            })
    }

    #[test]
    fn the_navidad_comes_on_its_days() {
        for (day, (w, h)) in [
            ((12, 24), (120, 40)),
            ((12, 16), (120, 40)),
            ((1, 6), (80, 24)),
            ((12, 12), (140, 45)),
        ] {
            for biome in ["reef", "tank", "ocean", "fresh", "kelp", "arctic", "deep"] {
                let fiesta = Fiesta::of_day(day.0, day.1);
                festive_scene(&[("biome", s(biome))], fiesta, "classic", |ctx, a| {
                    a.resize(ctx, w, h);
                    frame(ctx, a, w, h, 2.0, true);
                    // The nacimiento is the first thing set on the floor, and
                    // esferas rise behind the fish.
                    let p = props(a);
                    assert!(
                        p.first().is_some_and(|d| d.0.starts_with("nacimient")),
                        "{biome} {day:?}: {p:?}"
                    );
                    // (The deep sea has few of the season's own creatures.)
                    let own = 1 + usize::from(biome != "deep");
                    assert!(seasonal(a) >= own, "{biome} {day:?}");
                    let mut c = Canvas::new(w, h, Color::Reset);
                    a.draw_event_backdrop(&mut c);
                    assert!(c.to_text().contains('@'), "{biome} {day:?}: no esferas");
                });
            }
        }
    }

    #[test]
    fn the_manger_is_where_the_nino_goes() {
        let lib = library();
        for name in ["nacimiento", "nacimientito"] {
            let (dx, dy) = navidad::manger(name).unwrap();
            let d = &lib.decor[lib.decor_named(name).unwrap()];
            let spr = &lib.sprites[d.spr as usize];
            for f in 0..spr.frame_count() {
                let cell = spr
                    .frame(f)
                    .iter()
                    .find(|c| (c.dx as i32, c.dy as i32) == (dx, dy));
                assert_eq!(cell.map(|c| c.ch), Some('~'), "{name} frame {f}");
            }
        }
    }

    #[test]
    fn the_nino_dios_is_born_on_nochebuena() {
        for (day, born) in [
            ((12, 20), false),
            ((12, 23), false),
            ((12, 24), true),
            ((12, 28), true),
            ((1, 6), true),
        ] {
            let opts = [("biome", s("tank"))];
            festive_scene(&opts, Fiesta::of_day(day.0, day.1), "classic", |ctx, a| {
                a.resize(ctx, 140, 45);
                let (x, y, name) = nacimiento(a).expect("a nacimiento");
                let (dx, dy) = navidad::manger(name).unwrap();
                let mut c = Canvas::new(140, 45, Color::Reset);
                a.draw_event_overlay(&mut c);
                let lies = c.get(x + dx, y + dy).is_some_and(|cell| cell.ch == 'o');
                assert_eq!(lies, born, "{day:?}");
            });
        }
    }

    #[test]
    fn the_posadas_walk_at_night() {
        // Fish with candles, counted on the overlay alone.
        let walking = |day: (u32, u32), night: bool, w: u16, h: u16| {
            let opts = [
                ("biome", s("tank")),
                ("day_length", rate(if night { 900.0 } else { 0.0 })),
            ];
            let mut n = 0;
            festive_scene(&opts, Fiesta::of_day(day.0, day.1), "classic", |ctx, a| {
                a.resize(ctx, w, h);
                for _ in 0..300 {
                    a.update(ctx, 0.1);
                }
                if night {
                    a.world.as_mut().unwrap().env.force_night();
                }
                let mut c = Canvas::new(w, h, Color::Reset);
                a.draw_event_overlay(&mut c);
                let t = c.to_text();
                n = t.matches("<(o>").count() + t.matches("<o)>").count();
            });
            n
        };
        assert!(walking((12, 20), true, 140, 45) >= 3);
        assert!(walking((12, 24), true, 60, 20) >= 2);
        assert_eq!(walking((12, 20), false, 140, 45), 0, "by day");
        assert_eq!(walking((12, 13), true, 140, 45), 0, "before the posadas");
        assert_eq!(walking((12, 28), true, 140, 45), 0, "after Nochebuena");
        assert_eq!(walking((1, 5), true, 140, 45), 0, "in January");
        // More of them as the posadas go.
        assert!(walking((12, 23), true, 200, 60) > walking((12, 16), true, 200, 60));
    }

    #[test]
    fn the_kelp_wears_the_series() {
        let opts = [("biome", s("kelp")), ("day_length", rate(0.0))];
        festive_scene(&opts, Fiesta::of_day(12, 24), "classic", |ctx, a| {
            a.resize(ctx, 160, 40);
            for _ in 0..50 {
                a.update(ctx, 0.1);
            }
            // Strings between the plants, a bulb every third cell...
            let mut c = Canvas::new(160, 40, Color::Reset);
            a.draw_event_backdrop(&mut c);
            let text = c.to_text();
            let bulbs = text.chars().filter(|ch| "Oo".contains(*ch)).count();
            assert!(bulbs >= 3, "{bulbs} bulbs on the strings");
            assert!(text.contains('-'), "no cord");
            // ...and bulbs on the plants themselves.
            let mut on_plants = 0;
            for e in a.ents.iter().filter(|e| e.kind == Kind::Seaweed) {
                let mut c = Canvas::new(160, 40, Color::Reset);
                a.dress_plant(e, &mut c);
                on_plants += c.to_text().chars().filter(|ch| "Oo.".contains(*ch)).count();
            }
            assert!(on_plants >= 3, "{on_plants} bulbs on the plants");
        });
        // The lights chase: a moment later the same strings are not the same.
        festive_scene(&opts, Fiesta::of_day(12, 24), "classic", |ctx, a| {
            a.resize(ctx, 160, 40);
            a.update(ctx, 0.1);
            let shot = |a: &Aquarium| {
                let mut c = Canvas::new(160, 40, Color::Reset);
                a.draw_event_backdrop(&mut c);
                c.to_text()
            };
            let before = shot(a);
            for _ in 0..4 {
                a.update(ctx, 0.1);
            }
            assert_ne!(before, shot(a));
        });
    }

    #[test]
    fn ordinary_days_have_no_navidad() {
        // Nor do the holidays this scene has no costume for.
        for fiesta in [
            Fiesta::NONE,
            Fiesta::of_day(6, 15),
            Fiesta::of_day(9, 15),
            Fiesta::of_day(2, 2),
            Fiesta::of_day(11, 20),
        ] {
            for biome in ["reef", "tank", "kelp", "deep"] {
                festive_scene(&[("biome", s(biome))], fiesta, "classic", |ctx, a| {
                    a.resize(ctx, 140, 45);
                    let text = frame(ctx, a, 140, 45, 60.0, true);
                    assert_eq!(seasonal(a), 0, "{biome} {fiesta:?}");
                    assert!(nacimiento(a).is_none(), "{biome} {fiesta:?}");
                    let mut c = Canvas::new(140, 45, Color::Reset);
                    a.draw_event_backdrop(&mut c);
                    a.draw_event_overlay(&mut c);
                    assert!(c.to_text().trim().is_empty(), "{biome} {fiesta:?}");
                    assert!(!text.contains("<(o>") && !text.contains("<o)>"));
                });
            }
        }
    }

    #[test]
    fn the_navidad_comes_and_goes_with_retheme() {
        let theme = crate::theme::Theme::classic();
        let mut t = toml::Table::new();
        t.insert("biome".into(), s("tank"));
        let opts = crate::scene::Opts::new(INFO.options, t);
        let ctx = |fiesta| Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 5,
            fiesta,
        };
        let (off, on) = (ctx(Fiesta::NONE), ctx(Fiesta::of_day(12, 24)));
        let mut a = Aquarium::new(&off);
        a.resize(&off, 140, 45);
        a.retheme(&off);
        assert_eq!(seasonal(&a), 0);
        // Midnight, December 11 to 12: the floor is set at once.
        a.retheme(&on);
        assert!(nacimiento(&a).is_some());
        assert!(
            props(&a)
                .first()
                .is_some_and(|d| d.0.starts_with("nacimient"))
        );
        frame(&on, &mut a, 140, 45, 120.0, true);
        let lib = library();
        let creatures = a
            .ents
            .iter()
            .filter(|e| matches!(e.kind, Kind::Fish | Kind::Critter))
            .filter(|e| lib.species[e.who as usize].def.season != ALWAYS)
            .count();
        assert!(creatures > 0, "the season's creatures came");
        // And after the Reyes it is all gone, but for the last creatures'
        // crossing.
        a.retheme(&off);
        assert!(nacimiento(&a).is_none());
        assert!(props(&a).iter().all(|d| !d.1));
        frame(&off, &mut a, 140, 45, 600.0, true);
        assert_eq!(seasonal(&a), 0);
        let mut c = Canvas::new(140, 45, Color::Reset);
        a.draw_event_backdrop(&mut c);
        assert!(c.to_text().trim().is_empty());
        // The posadas begin on the 16th without the floor being set again.
        a.retheme(&ctx(Fiesta::of_day(12, 13)));
        let n = a.ents.len();
        a.retheme(&ctx(Fiesta::of_day(12, 16)));
        assert!(a.world.as_ref().unwrap().posadas);
        assert_eq!(a.ents.len(), n);
    }

    #[test]
    fn more_navidad_as_the_holiday_comes() {
        // The share of new groups that are of the season.
        let share = |day: (u32, u32)| {
            let mut ours = 0;
            festive_scene(
                &[("biome", s("reef"))],
                Fiesta::of_day(day.0, day.1),
                "classic",
                |ctx, a| {
                    a.resize(ctx, 160, 50);
                    let lib = library();
                    for _ in 0..800 {
                        let n = a.ents.len();
                        a.add_group(false, true);
                        ours += usize::from(
                            a.ents
                                .get(n)
                                .is_some_and(|e| lib.species[e.who as usize].def.season != ALWAYS),
                        );
                    }
                },
            );
            ours as f32 / 800.0
        };
        let (first, height) = (share((12, 13)), share((12, 24)));
        assert!(
            0.15 < first && first < height && height < 0.75,
            "{first} {height}"
        );
    }

    #[test]
    fn the_navidad_survives_odd_screens_and_long_steps() {
        for theme in ["classic", "dracula"] {
            for day in [(12, 20), (12, 24), (1, 6)] {
                for (w, h) in [
                    (1, 1),
                    (5, 3),
                    (12, 9),
                    (30, 12),
                    (40, 14),
                    (300, 90),
                    (300, 10),
                ] {
                    for biome in ["random", "deep", "tank", "mixed"] {
                        let opts = [("biome", s(biome)), ("events", rate(20.0))];
                        festive_scene(&opts, Fiesta::of_day(day.0, day.1), theme, |ctx, a| {
                            a.resize(ctx, w, h);
                            let mut c = Canvas::new(w, h, Color::Reset);
                            for k in 0..40 {
                                a.update(ctx, 5.0);
                                if k == 20
                                    && let Some(world) = a.world.as_mut()
                                {
                                    world.env.force_night();
                                }
                                a.draw(ctx, &mut c);
                            }
                            a.retheme(ctx);
                            a.draw(ctx, &mut c);
                        });
                    }
                }
            }
        }
    }
}
