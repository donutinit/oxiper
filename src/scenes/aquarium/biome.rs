//! Beyond asciiquarium: biomes, and the cast from `assets` brought to life.
//!
//! A biome decides which creatures, plants, props and passers-by come. The
//! sea starts in one and every `biome_time` seconds moves on to another: the
//! floor changes at once, while the creatures already swimming finish their
//! crossing and are replaced by the new place's as they leave. Creatures come
//! in groups (a school, or a loner) that are replaced when their last member
//! is gone, so the population stays as steady as asciiquarium's.

use std::f32::consts::TAU;

use super::assets::{
    self, ALL, ARCTIC, BIOMES, Biomes, DEEP, FRESH, KELP, Lane, OCEAN, REEF, Swim, TANK, Zone,
};
use super::library::library;
use super::*;

/// A comma separated list of names from the options; `-name` leaves one out.
#[derive(Default)]
struct Filter {
    only: Vec<String>,
    not: Vec<String>,
}

impl Filter {
    fn parse(s: &str) -> Filter {
        let mut f = Filter::default();
        for n in s.split(',').map(str::trim) {
            match n.strip_prefix('-') {
                Some(x) => f.not.push(x.trim().to_string()),
                None if n.is_empty() || n == "auto" => {}
                None => f.only.push(n.to_string()),
            }
        }
        f
    }

    /// Listed names come anywhere; without a list, whatever belongs `here` does.
    fn allows(&self, name: &str, here: bool) -> bool {
        if self.not.iter().any(|n| n == name) {
            false
        } else if self.only.is_empty() {
            here
        } else {
            self.only.iter().any(|n| n == name)
        }
    }
}

/// asciiquarium's own passers-by: their name for the `visitors` option, where
/// they belong, and whether they need a surface. In `random_object`'s order.
pub(super) const CLASSIC_VISITORS: [(&str, Biomes, bool); 9] = [
    ("ship", OCEAN | KELP | REEF | ARCTIC, true),
    ("whale", OCEAN | KELP | ARCTIC, true),
    ("monster", OCEAN | KELP | FRESH, true),
    ("big-fish", REEF | OCEAN | KELP | TANK, false),
    ("shark", REEF | OCEAN | KELP | DEEP | ARCTIC, false),
    ("fishhook", OCEAN | KELP | FRESH | TANK, true),
    ("swan", FRESH | TANK, true),
    ("ducks", FRESH | TANK, true),
    ("dolphins", OCEAN | REEF, true),
];
const FISHHOOK: u8 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Visit {
    Classic(u8),
    Data(u16),
}

struct Group {
    id: u32,
    alive: u16,
    crawler: bool,
}

pub(super) struct World {
    /// The biomes the sea moves between, and where it is now (0 before the start).
    biomes: Vec<Biomes>,
    biome: Biomes,
    stay: f32,
    clock: f32,
    species: Filter,
    plants: Filter,
    decor: Filter,
    visitors: Filter,
    no_decor: bool,
    castle: bool,
    schools: bool,
    floor: bool,
    snow: bool,
    /// What may come now: indices into the library.
    swimmers: Vec<u16>,
    crawlers: Vec<u16>,
    plant_pool: Vec<u16>,
    decor_pool: Vec<u16>,
    visit_pool: Vec<Visit>,
    groups: Vec<Group>,
    next_group: u32,
    /// Picks the sand pattern and the marine snow.
    seed: u64,
}

impl World {
    /// The world the options describe, or `None` for classic asciiquarium.
    pub(super) fn from_opts(o: &crate::scene::Opts) -> Option<World> {
        let all = || BIOMES.iter().map(|b| b.1).collect::<Vec<_>>();
        let biomes = match o.s("biome").trim().to_ascii_lowercase().as_str() {
            "classic" | "asciiquarium" => return None,
            "mixed" | "all" => vec![ALL],
            "random" | "" => all(),
            list => {
                let v: Vec<Biomes> = list
                    .split(',')
                    .filter_map(|n| BIOMES.iter().find(|b| b.0 == n.trim()).map(|b| b.1))
                    .collect();
                if v.is_empty() { all() } else { v }
            }
        };
        let stay = o.f("biome_time");
        Some(World {
            biomes,
            biome: 0,
            stay: if stay.is_finite() {
                stay.max(0.0)
            } else {
                300.0
            },
            clock: 0.0,
            species: Filter::parse(o.s("species")),
            plants: Filter::parse(o.s("plants")),
            decor: Filter::parse(o.s("decor")),
            visitors: Filter::parse(o.s("visitors")),
            no_decor: o.s("decor").trim() == "none",
            castle: o.b("castle"),
            schools: o.b("schools"),
            floor: o.b("floor"),
            snow: o.b("snow"),
            swimmers: Vec::new(),
            crawlers: Vec::new(),
            plant_pool: Vec::new(),
            decor_pool: Vec::new(),
            visit_pool: Vec::new(),
            groups: Vec::new(),
            next_group: 0,
            seed: 0,
        })
    }
}

/// A random index, each `i` with probability proportional to `w(i)`.
fn weighted(rng: &mut fastrand::Rng, n: usize, w: impl Fn(usize) -> u32) -> Option<usize> {
    let total: u32 = (0..n).map(&w).sum();
    if total == 0 {
        return None;
    }
    let mut r = rng.u32(0..total);
    for i in 0..n {
        let wi = w(i);
        if r < wi {
            return Some(i);
        }
        r -= wi;
    }
    None
}

/// Colors for the mask digits: the tint's choices, or random like asciiquarium's
/// fish, with white eyes.
fn tint_pal(rng: &mut fastrand::Rng, t: &assets::Tint) -> [u8; 10] {
    let mut pal = [DEFAULT; 10];
    for (d, slot) in pal.iter_mut().enumerate().skip(1) {
        let c = t.choices(d);
        *slot = if !c.is_empty() {
            c[rng.usize(0..c.len())]
        } else if d == 4 {
            W + BRIGHT
        } else {
            RAND_COLORS[rng.usize(0..11)]
        };
    }
    pal
}

fn between(rng: &mut fastrand::Rng, a: f32, b: f32) -> f32 {
    a + rng.f32() * (b - a)
}

fn swim_motion(rng: &mut fastrand::Rng, style: Swim, base: f32) -> Motion {
    let (amp, period) = match style {
        Swim::Cruise | Swim::Crawl => (0.0, 1.0),
        Swim::Wave => (between(rng, 0.5, 1.3), between(rng, 4.0, 9.0)),
        Swim::Dart => (between(rng, 0.0, 0.6), between(rng, 2.5, 5.0)),
        Swim::Drift => (between(rng, 1.5, 3.5), between(rng, 8.0, 16.0)),
        Swim::Hover => (between(rng, 0.4, 0.9), between(rng, 3.0, 6.0)),
    };
    Motion::Swim {
        style,
        base,
        amp,
        omega: TAU / period,
        phase: rng.f32() * TAU,
        timer: between(rng, 2.0, 8.0),
    }
}

/// splitmix64, for patterns that must stay put from frame to frame.
fn hash(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

fn frames(spr: u16) -> f32 {
    sprites().get(spr as usize).map_or(1, |s| s.frame_count()) as f32
}

impl Aquarium {
    /// The deep sea has no surface: no waves, no boats, no sky.
    fn surface(&self) -> bool {
        self.world.as_ref().is_none_or(|w| w.biome != DEEP)
    }

    /// The first row of water.
    fn water_top(&self) -> i32 {
        if self.surface() { 9 } else { 1 }
    }

    /// The rows a creature `sh` tall may swim in, for its zone.
    fn zone_range(&self, zone: Zone, sh: i32) -> (f32, f32) {
        let top = self.water_top();
        let bottom = (self.h - sh).max(top);
        let span = (bottom - top) as f32;
        let (a, b) = match zone {
            Zone::Top => (0.0, 0.2),
            Zone::Upper => (0.0, 0.45),
            Zone::Middle => (0.25, 0.75),
            Zone::Lower => (0.55, 1.0),
            Zone::Bottom => (1.0, 1.0),
            Zone::Any => (0.0, 1.0),
        };
        (
            (top as f32 + span * a).round(),
            (top as f32 + span * b).round(),
        )
    }

    /// Start (or, after a resize, restart) the current biome.
    pub(super) fn rebuild_world(&mut self) {
        let Some(w) = self.world.as_mut() else {
            return;
        };
        if w.biome == 0 {
            w.biome = w.biomes[self.rng.usize(0..w.biomes.len())];
            w.seed = self.rng.u64(..);
        }
        w.groups.clear();
        self.refresh_pools();
        if self.surface() {
            self.add_environment();
        }
        self.add_decor();
        self.add_all_plants();
        self.add_all_creatures();
        self.random_object();
    }

    /// What may come in this biome, given the filters and the screen size.
    fn refresh_pools(&mut self) {
        let lib = library();
        let surface = self.surface();
        let room = self.h - self.water_top();
        let width = self.w;
        let Some(w) = self.world.as_mut() else {
            return;
        };
        let b = w.biome;
        w.swimmers.clear();
        w.crawlers.clear();
        w.plant_pool.clear();
        w.decor_pool.clear();
        w.visit_pool.clear();
        for (i, sp) in lib.species.iter().enumerate() {
            let d = sp.def;
            if !w.species.allows(d.name, d.biomes & b != 0) || lib.sprites[sp.spr as usize].h > room
            {
                continue;
            }
            if d.swim == Swim::Crawl {
                w.crawlers.push(i as u16);
            } else {
                w.swimmers.push(i as u16);
            }
        }
        for (i, p) in lib.plants.iter().enumerate() {
            let h = match p.stalk {
                Some((min, _)) => min as i32,
                None => lib.sprites[p.spr as usize].h,
            };
            if h < room && w.plants.allows(p.name, p.biomes & b != 0) {
                w.plant_pool.push(i as u16);
            }
        }
        if !w.no_decor {
            for (i, d) in lib.decor.iter().enumerate() {
                let s = &lib.sprites[d.spr as usize];
                if (d.name != "castle" || w.castle)
                    && s.h < room
                    && s.w <= width
                    && w.decor.allows(d.name, d.biomes & b != 0)
                {
                    w.decor_pool.push(i as u16);
                }
            }
        }
        for (k, &(name, biomes, needs_surface)) in CLASSIC_VISITORS.iter().enumerate() {
            if (surface || !needs_surface) && w.visitors.allows(name, biomes & b != 0) {
                w.visit_pool.push(Visit::Classic(k as u8));
            }
        }
        for (i, v) in lib.visitors.iter().enumerate() {
            let d = v.def;
            let fits = match d.lane {
                Lane::Sky | Lane::Boat | Lane::Float => surface,
                Lane::Deep | Lane::Floor => lib.sprites[v.spr as usize].h < room,
            };
            if fits && w.visitors.allows(d.name, d.biomes & b != 0) {
                w.visit_pool.push(Visit::Data(i as u16));
            }
        }
    }

    fn add_all_creatures(&mut self) {
        // Groups instead of asciiquarium's single fish, a little sparser since
        // schools bring several.
        let water = ((self.h - self.water_top()) * self.w).max(0) as f32;
        let groups = (water / 400.0 * self.fish_density).round() as usize;
        for _ in 0..groups {
            self.add_group(false, true);
        }
        let crawlers = (self.w as f32 / 40.0 * self.fish_density).round() as usize;
        for _ in 0..crawlers {
            self.add_group(true, true);
        }
    }

    /// A school (or a loner) of one species. At the start they are scattered
    /// over the screen; later they swim in from a side.
    fn add_group(&mut self, crawler: bool, scatter: bool) {
        let lib = library();
        let max_school = (self.w / 10).clamp(1, 255) as u8;
        let (si, n, id) = {
            let Some(w) = self.world.as_mut() else {
                return;
            };
            let pool = if crawler { &w.crawlers } else { &w.swimmers };
            let Some(k) = weighted(&mut self.rng, pool.len(), |k| {
                lib.species[pool[k] as usize].def.weight as u32
            }) else {
                return;
            };
            let si = pool[k] as usize;
            let (lo, hi) = lib.species[si].def.school;
            let n = if w.schools && hi > 1 {
                self.rng.u8(lo..=hi)
            } else {
                1
            };
            let n = n.min(max_school).max(1);
            let id = w.next_group;
            w.next_group = w.next_group.wrapping_add(1);
            w.groups.push(Group {
                id,
                alive: n as u16,
                crawler,
            });
            (si, n, id)
        };
        let sp = &lib.species[si];
        let d = sp.def;
        let left = self.rng.bool();
        let spr = sp.spr + left as u16;
        let s = &lib.sprites[spr as usize];
        let fw = s.w as f32;
        let speed = between(&mut self.rng, d.speed.0, d.speed.1);
        let pal = tint_pal(&mut self.rng, &sp.tint);
        let (ymin, ymax) = self.zone_range(d.zone, s.h);
        let z = if crawler {
            Z_FISH_END
        } else {
            self.rng.i32(Z_FISH_START..Z_FISH_END)
        };
        let x0 = if scatter {
            self.rng.f32() * (self.w as f32 - fw).max(0.0)
        } else if left {
            (self.w - 2) as f32
        } else {
            1.0 - fw
        };
        let y0 = (ymin + (self.rng.f32() * (ymax - ymin + 1.0)).floor()).min(ymax);
        let kind = if d.fish { Kind::Fish } else { Kind::Critter };
        let nframes = frames(spr);
        // A school keeps formation: one speed and one rhythm for all.
        let motion = swim_motion(&mut self.rng, d.swim, y0);
        for m in 0..n {
            // The rest of the school follows, a little spread out.
            let behind = if m == 0 {
                0.0
            } else {
                m as f32 * (fw + 1.0) + self.rng.f32() * (fw * 0.5 + 3.0)
            };
            let x = if left { x0 + behind } else { x0 - behind };
            let y = if m == 0 {
                y0
            } else {
                (y0 + self.rng.i32(-2..=2) as f32).clamp(ymin, ymax)
            };
            let mut e = Ent::new(kind, spr, x, y, z, sp.color)
                .per_frame(if left { -speed } else { speed }, 0.0, d.fps)
                .die_offscreen()
                .on_death(Death::Group(id));
            if d.fish {
                e = e.physical(1);
            }
            e.pal = pal;
            e.seen = false;
            e.frame = self.rng.f32() * nframes;
            e.motion = motion;
            if let Motion::Swim { base, .. } = &mut e.motion {
                *base = y;
            }
            // In case it never shows up (a crawler lost off the edge).
            e.ttl = (self.w as f32 + 2.0 * fw + behind) / (speed * TICK) * 3.0 + 60.0;
            self.spawn(e);
        }
    }

    /// One of group `g` is gone; when it was the last, another group comes.
    pub(super) fn group_gone(&mut self, g: u32) {
        let Some(w) = self.world.as_mut() else {
            return;
        };
        let Some(k) = w.groups.iter().position(|x| x.id == g) else {
            return;
        };
        w.groups[k].alive = w.groups[k].alive.saturating_sub(1);
        if w.groups[k].alive == 0 {
            let crawler = w.groups[k].crawler;
            w.groups.swap_remove(k);
            self.add_group(crawler, false);
        }
    }

    /// Moves a creature one step; returns whether it should animate.
    pub(super) fn swim(&mut self, e: &mut Ent, dt: f32) -> bool {
        let Motion::Swim {
            style,
            base,
            amp,
            omega,
            mut phase,
            mut timer,
        } = e.motion
        else {
            return true;
        };
        phase = (phase + omega * dt) % TAU;
        let mut pace = 1.0;
        let mut animate = true;
        match style {
            Swim::Dart => {
                let s = 0.5 + 0.5 * (2.0 * phase).sin();
                pace = 0.25 + 2.2 * s.powi(4);
            }
            Swim::Crawl => {
                // Walk a while, rest a while.
                if timer > 0.0 {
                    timer -= dt;
                    if timer <= 0.0 {
                        timer = -between(&mut self.rng, 1.0, 5.0);
                    }
                } else {
                    timer += dt;
                    if timer >= 0.0 {
                        timer = between(&mut self.rng, 2.0, 10.0);
                    }
                }
                if timer < 0.0 {
                    pace = 0.0;
                    animate = false;
                }
            }
            _ => {}
        }
        e.x += e.vx * pace * dt;
        if amp > 0.0 {
            let top = self.water_top() as f32;
            let bottom = (self.h as f32 - e.h).max(top);
            e.y = (base + amp * phase.sin()).clamp(top, bottom);
        }
        e.motion = Motion::Swim {
            style,
            base,
            amp,
            omega,
            phase,
            timer,
        };
        animate
    }

    fn add_all_plants(&mut self) {
        let n = (self.w as f32 / 12.0 * self.seaweed_density) as usize;
        for _ in 0..n {
            self.add_plant();
        }
    }

    pub(super) fn add_plant(&mut self) {
        let lib = library();
        let room = self.h - self.water_top();
        let Some(w) = self.world.as_ref() else {
            return;
        };
        let pool = &w.plant_pool;
        let Some(k) = weighted(&mut self.rng, pool.len(), |k| {
            lib.plants[pool[k] as usize].weight as u32
        }) else {
            return;
        };
        let p = &lib.plants[pool[k] as usize];
        let spr = match p.stalk {
            Some((min, max)) => {
                let (min, max) = (min as i32, (max as i32).min(room - 1).max(min as i32));
                p.spr + (self.rng.i32(min..=max) - min) as u16
            }
            None => p.spr,
        };
        let s = &lib.sprites[spr as usize];
        let x = self.rng.i32(0..=(self.w - s.w).max(0)) as f32;
        let sway = p.fps * between(&mut self.rng, 0.85, 1.15);
        let mut e = Ent::new(
            Kind::Seaweed,
            spr,
            x,
            (self.h - s.h) as f32,
            Z_SEAWEED,
            p.color,
        )
        .per_frame(0.0, 0.0, sway)
        .on_death(Death::Seaweed);
        e.pal = tint_pal(&mut self.rng, &p.tint);
        // Plants live 8 to 12 minutes, like asciiquarium's seaweed.
        e.ttl = between(&mut self.rng, 8.0, 12.0) * 60.0;
        e.frame = self.rng.f32() * frames(spr);
        self.spawn(e);
    }

    /// Props along the floor, apart from each other: one per 70 columns, or
    /// every one the `decor` option names.
    fn add_decor(&mut self) {
        let lib = library();
        let Some(w) = self.world.as_ref() else {
            return;
        };
        let mut cands = w.decor_pool.clone();
        let want = if w.decor.only.is_empty() {
            (1 + self.w as usize / 70).min(cands.len())
        } else {
            cands.len()
        };
        let mut taken: Vec<(i32, i32)> = Vec::new();
        for _ in 0..want {
            let Some(k) = weighted(&mut self.rng, cands.len(), |k| {
                lib.decor[cands[k] as usize].weight as u32
            }) else {
                break;
            };
            let d = &lib.decor[cands.swap_remove(k) as usize];
            let s = &lib.sprites[d.spr as usize];
            let spot = (0..16)
                .map(|_| self.rng.i32(0..=(self.w - s.w).max(0)))
                .find(|&x| taken.iter().all(|&(a, b)| x + s.w + 1 < a || x > b + 1));
            let Some(x) = spot else {
                continue;
            };
            taken.push((x, x + s.w));
            let mut e = Ent::new(
                Kind::Decor,
                d.spr,
                x as f32,
                (self.h - s.h) as f32,
                Z_CASTLE,
                d.color,
            )
            .per_frame(0.0, 0.0, d.fps);
            e.pal = tint_pal(&mut self.rng, &d.tint);
            e.emit = d.bubbles;
            self.spawn(e);
        }
    }

    /// The next passer-by, from asciiquarium's own and the biome's.
    pub(super) fn world_visitor(&mut self) {
        let lib = library();
        let has_fish = self.ents.iter().any(|e| e.kind == Kind::Fish && !e.dead);
        let Some(w) = self.world.as_ref() else {
            return;
        };
        let pool = &w.visit_pool;
        let Some(k) = weighted(&mut self.rng, pool.len(), |k| match pool[k] {
            // The hook waits for a bite; without fish it would wait forever.
            Visit::Classic(FISHHOOK) if !has_fish => 0,
            Visit::Classic(_) => 6,
            Visit::Data(i) => lib.visitors[i as usize].def.weight as u32,
        }) else {
            return;
        };
        let v = pool[k];
        match v {
            Visit::Classic(0) => self.add_ship(),
            Visit::Classic(1) => self.add_whale(),
            Visit::Classic(2) => self.add_monster(),
            Visit::Classic(3) => self.add_big_fish(),
            Visit::Classic(4) => self.add_shark(),
            Visit::Classic(FISHHOOK) => self.add_fishhook(),
            Visit::Classic(6) => self.add_swan(),
            Visit::Classic(7) => self.add_ducks(),
            Visit::Classic(_) => self.add_dolphins(),
            Visit::Data(i) => self.add_visitor(i),
        }
    }

    fn add_visitor(&mut self, i: u16) {
        let lib = library();
        let v = &lib.visitors[i as usize];
        let d = v.def;
        let left = self.rng.bool();
        let spr = v.spr + left as u16;
        let s = &lib.sprites[spr as usize];
        let top = self.water_top();
        // Sky: in the rows over the waves; boats on the first wave row;
        // floaters on the third, like the ducks.
        let (y, z) = match d.lane {
            Lane::Sky => (self.rng.i32(0..=(5 - s.h).max(0)), 1),
            Lane::Boat => (6 - s.h, Z_WATER_GAP[1]),
            Lane::Float => (8 - s.h, Z_WATER_GAP[3]),
            Lane::Deep => {
                let bottom = self.h - s.h - 1;
                let y = if bottom <= top {
                    top
                } else {
                    self.rng.i32(top..=bottom)
                };
                (y, Z_SHARK)
            }
            Lane::Floor => (self.h - s.h, Z_SHARK),
        };
        let speed = between(&mut self.rng, d.speed.0, d.speed.1);
        let (x, dx) = if left {
            ((self.w - 2) as f32, -speed)
        } else {
            ((1 - s.w) as f32, speed)
        };
        let mut e = visitor(spr, x, y as f32, z, v.color).per_frame(dx, 0.0, d.fps);
        e.pal = tint_pal(&mut self.rng, &v.tint);
        e.frame = self.rng.f32() * frames(spr);
        self.spawn(e);
    }

    /// Counts down to the next biome.
    pub(super) fn world_clock(&mut self, dt: f32) {
        let Some(w) = self.world.as_mut() else {
            return;
        };
        if w.stay <= 0.0 || w.biomes.len() < 2 || w.biome == 0 {
            return;
        }
        w.clock += dt;
        if w.clock < w.stay {
            return;
        }
        w.clock = 0.0;
        let had_surface = w.biome != DEEP;
        let here = w.biome;
        let others: Vec<Biomes> = w.biomes.iter().copied().filter(|&b| b != here).collect();
        if let Some(&b) = others.get(self.rng.usize(0..others.len().max(1))) {
            w.biome = b;
            self.migrate(had_surface);
        }
    }

    /// The sea moves on: a new floor at once; the creatures already swimming
    /// finish their crossing. Diving into (or rising out of) the deep, the
    /// surface and everything that needs it come or go too.
    fn migrate(&mut self, had_surface: bool) {
        self.refresh_pools();
        self.ents
            .retain(|e| !matches!(e.kind, Kind::Seaweed | Kind::Decor));
        if had_surface != self.surface() {
            let top = self.water_top() as f32;
            for e in &mut self.ents {
                e.dead |= match e.kind {
                    Kind::Fish | Kind::Critter => e.y < top,
                    Kind::Seaweed | Kind::Decor | Kind::Castle => false,
                    _ => true,
                };
            }
            if self.surface() {
                self.add_environment();
            }
            self.reap();
        }
        self.add_decor();
        self.add_all_plants();
    }

    /// Sand along the bottom and, in the deep, marine snow; both behind everything.
    pub(super) fn draw_backdrop(&self, c: &mut Canvas) {
        let Some(w) = self.world.as_ref() else {
            return;
        };
        if w.floor && self.h > self.water_top() + 2 {
            let y = self.h - 1;
            let (sand, stone) = match w.biome {
                DEEP => (K, K + BRIGHT),
                ARCTIC => (W, K + BRIGHT),
                FRESH => (Y, K + BRIGHT),
                _ => (Y, W),
            };
            for x in 0..self.w {
                let (ch, pen) = match hash(w.seed ^ x as u64) % 100 {
                    0..=34 => continue,
                    35..=59 => ('.', sand),
                    60..=71 => (',', sand),
                    72..=81 => ('_', sand),
                    82..=86 => ('-', sand),
                    87..=91 => ('`', sand),
                    92..=97 => ('o', stone),
                    _ => ('O', stone),
                };
                let (col, attr) = self.pens[pen as usize];
                c.set_attr(x, y, ch, col, attr);
            }
        }
        if w.snow && w.biome == DEEP && self.w > 0 && self.h > 0 {
            let (col, attr) = self.pens[W as usize];
            let n = (self.w * self.h / 80).max(1) as u64;
            for i in 0..n {
                let r = hash(w.seed.wrapping_add(i.wrapping_mul(0x9e37_79b9)));
                let x0 = (r % self.w as u64) as f32;
                let y0 = ((r >> 16) % self.h as u64) as f32;
                let fall = 0.2 + ((r >> 32) % 100) as f32 / 100.0 * 0.6;
                let y = (y0 + self.time * fall).rem_euclid(self.h as f32);
                let x = x0 + (self.time * 0.3 + i as f32).sin() * 1.2;
                let ch = b"..'`,"[((r >> 40) % 5) as usize] as char;
                c.set_attr(x.round() as i32, y as i32, ch, col, attr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
        };
        let mut a = Aquarium::new(&ctx);
        a.retheme(&ctx);
        f(&ctx, &mut a);
    }

    fn s(v: &str) -> toml::Value {
        toml::Value::String(v.into())
    }

    #[test]
    fn every_biome_lives_and_stays_populated() {
        for (name, _) in BIOMES.iter().chain(&[("mixed", ALL)]) {
            scene(&[("biome", s(name))], |ctx, a| {
                a.resize(ctx, 140, 45);
                let start = a.world.as_ref().unwrap().groups.len();
                let mut c = Canvas::new(140, 45, Color::Reset);
                for i in 0..30 * 120 {
                    a.update(ctx, 1.0 / 30.0);
                    if i % 97 == 0 {
                        c.clear();
                        a.draw(ctx, &mut c);
                    }
                }
                let w = a.world.as_ref().unwrap();
                assert_eq!(w.groups.len(), start, "{name}: groups");
                assert!(a.ents.len() < 400, "{name}: {} entities", a.ents.len());
                if !w.swimmers.is_empty() {
                    assert!(start > 0, "{name}: nobody came");
                }
            });
        }
    }

    #[test]
    fn the_sea_moves_on() {
        scene(&[("biome_time", toml::Value::Float(5.0))], |ctx, a| {
            a.resize(ctx, 120, 40);
            let mut seen = std::collections::HashSet::new();
            for _ in 0..30 * 120 {
                a.update(ctx, 1.0 / 30.0);
                seen.insert(a.world.as_ref().unwrap().biome);
            }
            assert!(seen.len() >= 4, "{seen:?}");
            // Every visit to the deep removed the waves; every return brought them back.
            let water = a
                .ents
                .iter()
                .filter(|e| matches!(e.kind, Kind::Water(_)))
                .count();
            assert_eq!(water, if a.surface() { 4 } else { 0 });
        });
    }

    #[test]
    fn filters_pick_species_and_props() {
        scene(
            &[
                ("biome", s("deep")),
                ("species", s("classic-2")),
                ("decor", s("castle")),
                ("schools", toml::Value::Boolean(false)),
            ],
            |ctx, a| {
                a.resize(ctx, 100, 40);
                let lib = library();
                let w = a.world.as_ref().unwrap();
                let names: Vec<&str> = w
                    .swimmers
                    .iter()
                    .map(|&i| lib.species[i as usize].def.name)
                    .collect();
                assert_eq!(names, ["classic-2"], "listed species come anywhere");
                assert!(
                    a.ents
                        .iter()
                        .any(|e| e.kind == Kind::Decor && e.spr == SP_CASTLE)
                );
            },
        );
        scene(&[("decor", s("none")), ("biome", s("tank"))], |ctx, a| {
            a.resize(ctx, 100, 40);
            assert!(!a.ents.iter().any(|e| e.kind == Kind::Decor));
        });
    }

    #[test]
    fn crawlers_rest_and_walk() {
        scene(&[], |_, a| {
            a.w = 100;
            a.h = 30;
            let mut e = Ent::new(Kind::Critter, SP_FISH, 10.0, 20.0, 5, W);
            e.vx = 5.0;
            e.motion = swim_motion(&mut a.rng, Swim::Crawl, 20.0);
            let (mut moving, mut resting) = (0, 0);
            for _ in 0..2000 {
                let x = e.x;
                a.swim(&mut e, 0.05);
                if e.x == x {
                    resting += 1
                } else {
                    moving += 1
                }
                assert_eq!(e.y, 20.0);
            }
            assert!(moving > 500 && resting > 200, "{moving} {resting}");
        });
    }

    #[test]
    fn tiny_and_odd_screens() {
        for (w, h) in [(1, 1), (5, 3), (20, 10), (40, 12), (300, 12), (30, 100)] {
            for name in ["random", "deep", "mixed", "tank"] {
                scene(
                    &[("biome", s(name)), ("biome_time", toml::Value::Float(1.0))],
                    |ctx, a| {
                        a.resize(ctx, w, h);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for _ in 0..200 {
                            a.update(ctx, 0.1);
                            a.draw(ctx, &mut c);
                        }
                        a.update(ctx, 5.0);
                    },
                );
            }
        }
    }
}
