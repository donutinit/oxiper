//! aquarium: a port of asciiquarium.
//!
//! Asciiquarium 1.1 is Copyright (C) 2013 Kirk Baucom <kbaucom@schizoid.com>,
//! with most of the art by Joan Stark, and is free software under the GNU
//! General Public License version 2 or (at your option) any later version.
//! This module ports its code and art (`art.rs`) together with the parts of
//! Term::Animation it relies on (`sprite.rs` and the entity engine below), so
//! creatures look, move, collide and replace each other as they do there.
//!
//! Term::Animation moves every entity by fixed amounts once per frame, and
//! asciiquarium draws a frame every tenth of a second. Those per-frame amounts
//! are turned into per-second rates here (see [`TICK`]), so the pace is the
//! original's at any frame rate, just smoother.
//!
//! With `biome = "classic"` that is all there is. Every other biome goes
//! beyond asciiquarium with the cast in `assets` (brought to life in
//! `biome.rs`): hundreds of species that swim in schools, bob, dart and crawl,
//! plants, corals, props and passers-by, a sandy floor, and a sea that moves
//! from one place to another every few minutes. There, everything also
//! reacts to everything else (`ecology.rs`, `encounters.rs`), light, current
//! and weather change (`environment.rs`), and now and then something special
//! happens (`events.rs`).
//!
//! On Día de Muertos (`season = "auto"` follows `ctx.fiesta`; `muertos`
//! keeps it every day) the sea dresses up, still in asciiquarium's ASCII:
//! an altar comes first to the floor (the great ofrenda, or a small one on a
//! small screen) with tombs, copal, sugar skulls and pan de muerto beside it;
//! a path of marigold petals flows down from the surface to it, and at night
//! the souls come down it; veladoras on marigold rafts float on the waves;
//! calavera and alebrije fish, a sugar-skull octopus, marigold goldfish and
//! a catrina jellyfish swim; the xolo, La Catrina, trajineras and monarchs
//! pass by. How much of it comes follows how far the holiday has come
//! (`fiesta.level`); on an ordinary day nothing of it does.
//!
//! At the Navidad (`navidad` keeps it every day) a nacimiento is set on the
//! floor first (with a small one for small screens), beside a tree, a pot of
//! ponche, farolitos and a rosca; the kelp is hung with the series' blinking
//! lights, strung from plant to plant, and esferas rise like bubbles; red and
//! green fish, a nochebuena-tailed school, bulbs of the series, a glass
//! jellyfish swim, a piñata de siete picos bobs on the waves, the Estrella de
//! Belén crosses the sky and the Reyes Magos sail by. From Nochebuena the
//! Niño Dios lies in the manger, and at night during the posadas a procession
//! of small fish carrying candles swims across.

mod art;
pub mod assets;
mod biome;
mod ecology;
mod encounters;
mod environment;
mod events;
mod garlands;
mod library;
mod navidad;
mod sprite;

use std::cmp::Reverse;

use crate::color::{self, Color, Gradient};
use crate::render::{BOLD, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use sprite::{DEFAULT, DIGIT, Sprite};

pub const INFO: SceneInfo = SceneInfo {
    name: "aquarium",
    aliases: &["asciiquarium", "fish"],
    about: "asciiquarium, and then some: hundreds of species in seven biomes",
    options: &[
        Opt {
            key: "fish",
            default: Val::F(1.0),
            help: "Fish density; 1 is the original's one fish per 350 cells of water",
        },
        Opt {
            key: "seaweed",
            default: Val::F(1.0),
            help: "Plant density; 1 is the original's one seaweed per 15 columns",
        },
        Opt {
            key: "biome",
            default: Val::S("random"),
            help: "random, mixed (all at once), classic (exactly asciiquarium), or some of \
                   reef, ocean, kelp, deep, fresh, arctic, tank, comma separated",
        },
        Opt {
            key: "biome_time",
            default: Val::F(300.0),
            help: "Seconds before the sea moves on to another biome; 0 stays",
        },
        Opt {
            key: "species",
            default: Val::S(""),
            help: "Only these species, anywhere (comma separated; -name leaves one out). \
                   `oxiper list species` names them",
        },
        Opt {
            key: "plants",
            default: Val::S(""),
            help: "Only these plants (comma separated; -name leaves one out)",
        },
        Opt {
            key: "decor",
            default: Val::S("auto"),
            help: "Props on the floor: auto (the biome's), none, or names (comma separated; -name leaves one out)",
        },
        Opt {
            key: "visitors",
            default: Val::S(""),
            help: "Only these passers-by (comma separated; -name leaves one out)",
        },
        Opt {
            key: "schools",
            default: Val::B(true),
            help: "Schooling fish swim in groups",
        },
        Opt {
            key: "floor",
            default: Val::B(true),
            help: "Sand and pebbles along the bottom (not in classic)",
        },
        Opt {
            key: "water",
            default: Val::S("clear"),
            help: "clear (the terminal's own background shows through) or deep (the sea painted: \
                   a gradient, slow rays of light, sandy dunes; not in classic)",
        },
        Opt {
            key: "snow",
            default: Val::B(true),
            help: "Marine snow drifting down in the deep sea",
        },
        Opt {
            key: "interactions",
            default: Val::B(true),
            help: "Everything reacts to everything: predators hunt, prey flees or hides, birds dive, \
                   nets and lines catch fish, cleaners ride along (not in classic)",
        },
        Opt {
            key: "day_length",
            default: Val::F(900.0),
            help: "Seconds for a whole day and night; 0 is always day (not in classic)",
        },
        Opt {
            key: "depth",
            default: Val::F(0.3),
            help: "Share of creatures swimming far back, dim and behind the plants",
        },
        Opt {
            key: "current",
            default: Val::F(1.0),
            help: "Strength of the current that sways plants and carries bubbles; 0 is still water",
        },
        Opt {
            key: "weather",
            default: Val::B(true),
            help: "Storms now and then: wind, rain and lightning over the water",
        },
        Opt {
            key: "events",
            default: Val::F(1.0),
            help: "How often something special happens (feeding time, a bait ball, \
                   a rare sighting...); 0 never",
        },
        Opt {
            key: "season",
            default: Val::S("auto"),
            help: "auto (the holidays, see `fiestas`), none, muertos (Día de Muertos every day) or navidad (the Navidad every day)",
        },
        Opt {
            key: "castle",
            default: Val::B(true),
            help: "Show the castle (in classic and in the tank)",
        },
        Opt {
            key: "random_objects",
            default: Val::B(true),
            help: "Send ships, whales, sharks, divers, submarines, birds... one at a time",
        },
        Opt {
            key: "bubbles",
            default: Val::B(true),
            help: "Fish blow bubbles",
        },
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Pace relative to the original (motion only: the biome's stay, days and events keep real time)",
        },
        Opt {
            key: "smooth",
            default: Val::B(true),
            help: "Move continuously; false steps at the original 10 frames per second",
        },
        Opt {
            key: "waves",
            default: Val::B(false),
            help: "Let the water surface drift (it is still in the original)",
        },
    ],
    make: |ctx| Box::new(Aquarium::new(ctx)),
};

/// Frames per second of the original: asciiquarium calls `halfdelay(1)`, so
/// each frame waits a tenth of a second for a key before animating.
pub(crate) const TICK: f32 = 10.0;
/// Longest simulation step, so fast things never tunnel through each other.
const MAX_STEP: f32 = 0.05;

// Depths from asciiquarium. Term::Animation draws higher z first, so lower z is in front.
const Z_SHARK: i32 = 2;
const Z_FISH_START: i32 = 3;
const Z_FISH_END: i32 = 20;
const Z_SEAWEED: i32 = 21;
const Z_CASTLE: i32 = 22;
/// The far layer (beyond classic): behind the plants and the props.
const Z_FAR: i32 = 24;
const Z_WATER_LINE: [i32; 4] = [8, 6, 4, 2];
const Z_WATER_GAP: [i32; 4] = [9, 7, 5, 3];

// Pens: the ANSI index of a Term::Animation color letter, +8 for the uppercase (bold) one.
const K: u8 = 0;
const R: u8 = 1;
const G: u8 = 2;
const Y: u8 = 3;
const B: u8 = 4;
const M: u8 = 5;
const C: u8 = 6;
const W: u8 = 7;
const BRIGHT: u8 = 8;

/// `rand_color`'s list. It picks with `int(rand($#colors))`, so the last one never comes up.
const RAND_COLORS: [u8; 12] = [
    C,
    C + BRIGHT,
    R,
    R + BRIGHT,
    Y,
    Y + BRIGHT,
    B,
    B + BRIGHT,
    G,
    G + BRIGHT,
    M,
    M + BRIGHT,
];

// Indices into `sprites()`.
const SP_CASTLE: u16 = 0;
const SP_FISH: u16 = 1; // 16 of them
const SP_SPLAT: u16 = 17;
const SP_SHARK: u16 = 18; // right, left
const SP_TEETH: u16 = 20;
const SP_HOOK: u16 = 21;
const SP_LINE: u16 = 22;
const SP_POINT: u16 = 23;
const SP_SHIP: u16 = 24;
const SP_WHALE: u16 = 26;
const SP_MONSTER: u16 = 28;
const SP_BIG_FISH: u16 = 30;
const SP_DUCKS: u16 = 32;
const SP_DOLPHIN: u16 = 34;
const SP_SWAN: u16 = 36;
const SP_BUBBLE: u16 = 38;
const SP_SEAWEED: u16 = 39; // heights 3 to 6
const SP_COUNT: usize = 43;
/// Water lines are drawn by tiling, not from a sprite.
const SP_NONE: u16 = u16::MAX;
/// `Ent::who` of something that is not from the library.
const NOBODY: u16 = u16::MAX;

/// Everything the aquarium can show, by group, with the biomes each belongs
/// to: the names the `species`, `plants`, `decor` and `visitors` options take.
pub fn roster() -> Vec<(&'static str, Vec<(&'static str, assets::Biomes)>)> {
    let lib = library::library();
    let species = |fish: bool| {
        assets::species()
            .filter(|s| s.fish == fish)
            .map(|s| (s.name, s.biomes))
            .collect()
    };
    let visitors = biome::CLASSIC_VISITORS
        .iter()
        .map(|&(n, b, _)| (n, b))
        .chain(assets::visitors().map(|v| (v.name, v.biomes)))
        .collect();
    vec![
        ("fish", species(true)),
        ("critters", species(false)),
        (
            "plants",
            lib.plants.iter().map(|p| (p.name, p.biomes)).collect(),
        ),
        (
            "decor",
            lib.decor.iter().map(|d| (d.name, d.biomes)).collect(),
        ),
        ("visitors", visitors),
    ]
}

fn sprites() -> &'static [Sprite] {
    &library::library().sprites
}

fn build_sprites() -> Vec<Sprite> {
    use art::*;
    let mut v = Vec::with_capacity(SP_COUNT);
    v.push(Sprite::new(&[CASTLE], &[CASTLE_MASK], false, '?'));
    for (shape, mask) in FISH {
        v.push(Sprite::new(&[shape], &[mask], true, '?'));
    }
    v.push(Sprite::new(&SPLAT, &[], false, ' '));
    for d in 0..2 {
        v.push(Sprite::new(&[SHARK[d]], &[SHARK_MASK[d]], true, '?'));
    }
    v.push(Sprite::new(&["*"], &[], false, '?'));
    v.push(Sprite::new(&[HOOK], &[], true, '?'));
    let line = "|\n".repeat(50) + &" \n".repeat(6);
    v.push(Sprite::new(&[line], &[], true, '?'));
    v.push(Sprite::new(&[HOOK_POINT], &[], false, '?'));
    for d in 0..2 {
        v.push(Sprite::new(&[SHIP[d]], &[SHIP_MASK[d]], true, '?'));
    }
    for d in 0..2 {
        // Five frames of whale, then the spout rising over the blowhole.
        let align = " ".repeat(if d == 1 { 1 } else { 11 });
        let mut frames: Vec<String> = (0..5).map(|_| format!("\n\n\n{}", WHALE[d])).collect();
        for spout in WATER_SPOUT {
            let mut lines: Vec<&str> = spout.split('\n').collect();
            while lines.last().is_some_and(|l| l.is_empty()) {
                lines.pop();
            }
            frames.push(lines.join(&format!("\n{align}")) + WHALE[d]);
        }
        v.push(Sprite::new(&frames, &[WHALE_MASK[d]], true, '?'));
    }
    v.push(Sprite::new(&MONSTER_R, &[MONSTER_MASK[0]], true, '?'));
    v.push(Sprite::new(&MONSTER_L, &[MONSTER_MASK[1]], true, '?'));
    for d in 0..2 {
        v.push(Sprite::new(&[BIG_FISH[d]], &[BIG_FISH_MASK[d]], true, '?'));
    }
    v.push(Sprite::new(&DUCKS_R, &[DUCKS_MASK[0]], true, '?'));
    v.push(Sprite::new(&DUCKS_L, &[DUCKS_MASK[1]], true, '?'));
    v.push(Sprite::new(&DOLPHIN_R, &[DOLPHIN_MASK[0]], true, '?'));
    v.push(Sprite::new(&DOLPHIN_L, &[DOLPHIN_MASK[1]], true, '?'));
    v.push(Sprite::new(&[SWAN_R], &[SWAN_MASK[0]], true, '?'));
    v.push(Sprite::new(&[SWAN_L], &[SWAN_MASK[1]], true, '?'));
    v.push(Sprite::new(&[".", "o", "O", "O", "O"], &[], false, '?'));
    for height in 3..=6 {
        // Two frames of alternating "(" and " )", swapped.
        let mut frames = [String::new(), String::new()];
        for i in 1..=height {
            let left = i % 2;
            frames[left].push_str("(\n");
            frames[1 - left].push_str(" )\n");
        }
        v.push(Sprite::new(&frames, &[], false, '?'));
    }
    debug_assert_eq!(v.len(), SP_COUNT);
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Water(u8),
    Castle,
    /// Plants of every kind.
    Seaweed,
    /// A prop on the floor (beyond classic).
    Decor,
    Fish,
    /// A creature that is not a fish: no bubbles, no collisions.
    Critter,
    Bubble,
    /// Something to eat drifting or sinking (beyond classic); `who` is a [`biome::Food`].
    Food,
    /// A cloud of ink (beyond classic).
    Ink,
    Splat,
    Shark,
    Teeth,
    Hook,
    Line,
    Point,
    Dolphin,
    /// Ships, whales, monsters, big fish, ducks, swans: they just pass by.
    Visitor,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Motion {
    /// Constant velocity (Term::Animation's `move_entity`).
    Linear,
    /// The dolphins' up-glide-down-glide path; `phase` counts original frames.
    Path { phase: f32 },
    /// asciiquarium's `fishhook_cb`: sink to 3/4 of the screen, or reel in once hooked.
    Hook { hooked: bool },
    /// A creature beyond classic: forward at `vx` (paced by its style),
    /// bobbing `amp` rows around `base`. `timer` paces a crawler: walking
    /// while positive, resting while negative.
    Swim {
        style: assets::Swim,
        base: f32,
        amp: f32,
        omega: f32,
        phase: f32,
        timer: f32,
    },
    /// Round and round an ellipse centered on (`cx`, `cy`), radii `rx` and
    /// `ry`, at `omega` radians a second, facing the way it goes (a bait
    /// ball, a clownfish around its anemone).
    Orbit {
        cx: f32,
        cy: f32,
        rx: f32,
        ry: f32,
        omega: f32,
        phase: f32,
    },
}

/// What happens when an entity dies (its `death_cb`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Death {
    None,
    /// `add_fish`
    Fish,
    /// `add_seaweed`
    Seaweed,
    /// `random_object`
    Random,
    /// `group_death(teeth)`: the teeth go with the shark.
    Shark,
    /// `group_death(teeth, fishline)`, plus the barb (left behind by the original).
    Hook,
    /// The lead dolphin lets the other two die offscreen.
    DolphinLead,
    /// One of a group (a school, or a loner) is gone; the last one is replaced.
    Group(u32),
}

#[derive(Clone, Copy, Debug)]
struct Ent {
    id: u32,
    kind: Kind,
    spr: u16,
    x: f32,
    y: f32,
    z: i32,
    /// Collision box (Term::Animation's width, height, depth).
    w: f32,
    h: f32,
    depth: i32,
    /// Cells per second.
    vx: f32,
    vy: f32,
    frame: f32,
    /// Animation frames per second.
    fps: f32,
    /// Default pen, and the pens that replace mask digits 1-9.
    def: u8,
    pal: [u8; 10],
    physical: bool,
    die_offscreen: bool,
    /// Seconds left to live.
    ttl: f32,
    death: Death,
    motion: Motion,
    dead: bool,
    reaped: bool,
    /// Has been on screen. Things that start out of sight (the back of a
    /// school) only die offscreen after they have been seen.
    seen: bool,
    /// Where a prop blows bubbles from, relative to its corner.
    emit: Option<(i8, i8)>,
    /// What it is in the library (beyond classic): a species, plant, prop or
    /// visitor index, by `kind`, or `NOBODY`.
    who: u16,
    /// Its `assets` traits.
    traits: assets::Traits,
    /// The cells it covers, to compare sizes.
    size: u16,
    /// Its own cruising speed in cells per second (creatures).
    speed: f32,
    /// 0 swims with everyone else; 1 is far back, out of reach.
    layer: u8,
    /// What a creature is up to (`ecology.rs`).
    act: ecology::Act,
    /// What a visitor, plant or prop is up to (`encounters.rs`).
    task: encounters::Task,
    /// The event it belongs to, if any (`events.rs`); 0 for none.
    tag: u32,
}

impl Ent {
    fn new(kind: Kind, spr: u16, x: f32, y: f32, z: i32, def: u8) -> Ent {
        let (w, h) = match sprites().get(spr as usize) {
            Some(s) => (s.w as f32, s.h as f32),
            None => (1.0, 1.0),
        };
        Ent {
            id: 0,
            kind,
            spr,
            x,
            y,
            z,
            w,
            h,
            depth: 1,
            vx: 0.0,
            vy: 0.0,
            frame: 0.0,
            fps: 0.0,
            def,
            pal: [DEFAULT; 10],
            physical: false,
            die_offscreen: false,
            ttl: f32::INFINITY,
            death: Death::None,
            motion: Motion::Linear,
            dead: false,
            reaped: false,
            seen: true,
            emit: None,
            who: NOBODY,
            traits: 0,
            size: 1,
            speed: 0.0,
            layer: 0,
            act: ecology::Act::default(),
            task: encounters::Task::default(),
            tag: 0,
        }
    }

    /// Velocity and animation speed from the original's per-frame callback arguments.
    fn per_frame(mut self, dx: f32, dy: f32, frames: f32) -> Ent {
        self.vx = dx * TICK;
        self.vy = dy * TICK;
        self.fps = frames * TICK;
        self
    }

    fn physical(mut self, depth: i32) -> Ent {
        self.physical = true;
        self.depth = depth;
        self
    }

    fn die_offscreen(mut self) -> Ent {
        self.die_offscreen = true;
        self
    }

    fn on_death(mut self, d: Death) -> Ent {
        self.death = d;
        self
    }

    fn motion(mut self, m: Motion) -> Ent {
        self.motion = m;
        self
    }

    #[inline]
    fn pen(&self, code: u8) -> u8 {
        let p = if code > DIGIT {
            self.pal[(code - DIGIT) as usize % 10]
        } else {
            code
        };
        if p >= DEFAULT { self.def } else { p }
    }
}

/// Term::Animation's collision test: boxes overlap on x, y and z.
fn overlap(a: &Ent, b: &Ent) -> bool {
    fn axis(p: f32, s: f32, q: f32, t: f32) -> bool {
        (p <= q && q < p + s) || (q <= p && p < q + t)
    }
    axis(a.x, a.w, b.x, b.w)
        && axis(a.y, a.h, b.y, b.h)
        && axis(a.z as f32, a.depth as f32, b.z as f32, b.depth as f32)
}

/// How far the dolphins' path has risen (negative) after `p` original frames:
/// 14 frames up half a row each, 2 gliding, 14 down, 6 gliding.
fn dolphin_rise(p: f32) -> f32 {
    let q = p.rem_euclid(36.0);
    if q < 14.0 {
        -0.5 * q
    } else if q < 16.0 {
        -7.0
    } else if q < 30.0 {
        -7.0 + 0.5 * (q - 16.0)
    } else {
        0.0
    }
}

/// A passer-by: it swims across once and makes room for the next random object.
fn visitor(spr: u16, x: f32, y: f32, z: i32, def: u8) -> Ent {
    Ent::new(Kind::Visitor, spr, x, y, z, def)
        .die_offscreen()
        .on_death(Death::Random)
}

struct Aquarium {
    rng: fastrand::Rng,
    w: i32,
    h: i32,
    ents: Vec<Ent>,
    /// Scratch buffers, reused every frame.
    order: Vec<u32>,
    targets: Vec<u32>,
    next_id: u32,
    time: f32,
    pending: f32,
    fish_density: f32,
    seaweed_density: f32,
    castle: bool,
    random_objects: bool,
    bubbles: bool,
    speed: f32,
    smooth: bool,
    waves: bool,
    pens: [(Color, u8); 16],
    water: [(Color, u8); 4],
    /// Everything beyond classic; `None` is asciiquarium exactly.
    world: Option<biome::World>,
    /// Real seconds per second of scene time, from the last tick: the
    /// world's clocks (the biome's stay, the day, storms, events, a plant's
    /// life) follow the wall clock however fast the scene runs.
    clock: f32,
}

impl Aquarium {
    fn new(ctx: &Ctx) -> Aquarium {
        let o = ctx.opts;
        let num = |k: &str, max: f32| {
            let v = o.f(k);
            if v.is_finite() {
                v.clamp(0.0, max)
            } else {
                1.0
            }
        };
        Aquarium {
            rng: ctx.rng(),
            w: 0,
            h: 0,
            ents: Vec::with_capacity(128),
            order: Vec::with_capacity(128),
            targets: Vec::with_capacity(8),
            next_id: 0,
            time: 0.0,
            pending: 0.0,
            fish_density: num("fish", 20.0),
            seaweed_density: num("seaweed", 20.0),
            castle: o.b("castle"),
            random_objects: o.b("random_objects"),
            bubbles: o.b("bubbles"),
            speed: num("speed", 50.0),
            smooth: o.b("smooth"),
            waves: o.b("waves"),
            pens: classic_pens(),
            water: [(Color::Ansi(C), 0); 4],
            world: biome::World::from_opts(o, ctx.fiesta),
            clock: 1.0,
        }
    }

    /// Perl's `rand($n)`: `rand(0)` is `rand(1)`, and a negative `$n` gives a negative number.
    fn rand(&mut self, n: f32) -> f32 {
        let n = if n == 0.0 { 1.0 } else { n };
        self.rng.f32() * n
    }

    /// Perl's `int(rand($n))`.
    fn rand_int(&mut self, n: f32) -> f32 {
        self.rand(n).trunc()
    }

    fn rand_dir(&mut self) -> usize {
        self.rand_int(2.0) as usize
    }

    /// `rand_color`: a random color for each mask digit.
    fn rand_pal(&mut self) -> [u8; 10] {
        let mut pal = [DEFAULT; 10];
        for slot in pal.iter_mut().skip(1) {
            *slot = RAND_COLORS[self.rand_int(11.0) as usize];
        }
        pal
    }

    fn spawn(&mut self, mut e: Ent) -> usize {
        e.id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.ents.push(e);
        self.ents.len() - 1
    }

    /// What asciiquarium does at start and after every resize.
    fn rebuild(&mut self) {
        self.ents.clear();
        if self.world.is_some() {
            self.rebuild_world();
            return;
        }
        self.add_environment();
        if self.castle {
            self.add_castle();
        }
        self.add_all_seaweed();
        self.add_all_fish();
        self.random_object();
    }

    fn add_environment(&mut self) {
        // The segments are tiled past the right edge, which matters for collisions.
        let tiled = (art::WATER[0].len() as i32 * (self.w / 33 + 1)) as f32;
        for i in 0..4u8 {
            let mut e = Ent::new(
                Kind::Water(i),
                SP_NONE,
                0.0,
                i as f32 + 5.0,
                Z_WATER_LINE[i as usize],
                C,
            )
            .physical(22);
            e.w = tiled;
            self.spawn(e);
        }
    }

    fn add_castle(&mut self) {
        let (x, y) = ((self.w - 32) as f32, (self.h - 13) as f32);
        self.spawn(Ent::new(
            Kind::Castle,
            SP_CASTLE,
            x,
            y,
            Z_CASTLE,
            K + BRIGHT,
        ));
    }

    fn add_all_seaweed(&mut self) {
        let n = ((self.w / 15) as f32 * self.seaweed_density) as usize;
        for _ in 0..n {
            self.add_seaweed();
        }
    }

    fn add_seaweed(&mut self) {
        let height = 3 + self.rand_int(4.0) as i32;
        let x = self.rand_int((self.w - 2) as f32) + 1.0;
        let y = (self.h - height) as f32;
        let sway = self.rand(0.05) + 0.25;
        // Seaweed lives for 8 to 12 minutes.
        let ttl = self.rand_int(4.0 * 60.0) + 8.0 * 60.0;
        let spr = SP_SEAWEED + (height - 3) as u16;
        let mut e = Ent::new(Kind::Seaweed, spr, x, y, Z_SEAWEED, G)
            .per_frame(0.0, 0.0, sway)
            .on_death(Death::Seaweed);
        e.ttl = ttl;
        self.spawn(e);
    }

    fn add_all_fish(&mut self) {
        // One fish per 350 cells below the surface.
        let water = ((self.h - 9) * self.w).max(0) as f32;
        let n = (water / 350.0).floor() * self.fish_density;
        for _ in 0..n as usize {
            self.add_fish();
        }
    }

    fn add_fish(&mut self) {
        // `int(rand($#fish_image / 2))` over 32 entries: the last fish is half as likely.
        let num = (self.rand_int(15.5) as u16).min(15);
        let speed = self.rand(2.0) + 0.25;
        let z = self.rand_int((Z_FISH_END - Z_FISH_START) as f32) as i32 + Z_FISH_START;
        let mut pal = self.rand_pal();
        pal[4] = W + BRIGHT; // eyes
        let spr = SP_FISH + num;
        let s = &sprites()[spr as usize];
        let (fw, fh) = (s.w, s.h);
        let y = self.rand_int((self.h - fh - 9) as f32) + 9.0;
        let left = num % 2 == 1;
        let (x, dx) = if left {
            ((self.w - 2) as f32, -speed)
        } else {
            ((1 - fw) as f32, speed)
        };
        let mut e = Ent::new(Kind::Fish, spr, x, y, z, W)
            .per_frame(dx, 0.0, 0.0)
            .physical(1)
            .die_offscreen()
            .on_death(Death::Fish);
        e.pal = pal;
        self.spawn(e);
    }

    fn add_bubble(&mut self, fish: &Ent) {
        let x = fish.x + if fish.vx > 0.0 { fish.w } else { 0.0 };
        let y = fish.y + (fish.h / 2.0).floor();
        self.add_bubble_at(x, y, fish.z - 1);
    }

    fn add_bubble_at(&mut self, x: f32, y: f32, z: i32) {
        self.spawn(
            Ent::new(Kind::Bubble, SP_BUBBLE, x, y, z, C + BRIGHT)
                .per_frame(0.0, -1.0, 0.1)
                .physical(1)
                .die_offscreen(),
        );
    }

    fn add_splat(&mut self, x: f32, y: f32, z: i32) {
        let mut e = Ent::new(Kind::Splat, SP_SPLAT, x - 4.0, y - 2.0, z - 2, R + BRIGHT)
            .per_frame(0.0, 0.0, 0.25);
        // `die_frame => 15`: drawn for 16 frames.
        e.ttl = 16.0 / TICK - 0.01;
        self.spawn(e);
    }

    fn random_object(&mut self) {
        if !self.random_objects {
            return;
        }
        if self.world.is_some() {
            self.world_visitor();
            return;
        }
        let mut pick = self.rand_int(9.0) as u32;
        // The hook waits for a bite, so without fish (fish = 0, or a tiny
        // window) it would stay forever and nothing else would ever come.
        if pick == 5 && !self.ents.iter().any(|e| e.kind == Kind::Fish && !e.dead) {
            pick = self.rand_int(8.0) as u32;
            pick += u32::from(pick >= 5);
        }
        match pick {
            0 => self.add_ship(),
            1 => self.add_whale(),
            2 => self.add_monster(),
            3 => self.add_big_fish(),
            4 => self.add_shark(),
            5 => self.add_fishhook(),
            6 => self.add_swan(),
            7 => self.add_ducks(),
            _ => self.add_dolphins(),
        }
    }

    /// The usual start for a passer-by: from the left edge going right, or the
    /// right edge going left.
    fn start(&self, dir: usize, from_left: f32, from_right: i32, speed: f32) -> (f32, f32) {
        if dir == 1 {
            ((self.w + from_right) as f32, -speed)
        } else {
            (from_left, speed)
        }
    }

    fn add_ship(&mut self) {
        let dir = self.rand_dir();
        let (x, dx) = self.start(dir, -24.0, -2, 1.0);
        let spr = SP_SHIP + dir as u16;
        self.spawn(visitor(spr, x, 0.0, Z_WATER_GAP[1], W + BRIGHT).per_frame(dx, 0.0, 0.0));
    }

    fn add_whale(&mut self) {
        let dir = self.rand_dir();
        let (x, dx) = self.start(dir, -18.0, -2, 1.0);
        let spr = SP_WHALE + dir as u16;
        self.spawn(visitor(spr, x, 0.0, Z_WATER_GAP[2], W + BRIGHT).per_frame(dx, 0.0, 1.0));
    }

    fn add_monster(&mut self) {
        let dir = self.rand_dir();
        let (x, dx) = self.start(dir, -64.0, -2, 2.0);
        let spr = SP_MONSTER + dir as u16;
        self.spawn(visitor(spr, x, 2.0, Z_WATER_GAP[2], G + BRIGHT).per_frame(dx, 0.0, 0.25));
    }

    fn add_big_fish(&mut self) {
        let dir = self.rand_dir();
        let (x, dx) = self.start(dir, -34.0, -1, 3.0);
        let y = self.rand_int((self.h - 15 - 9) as f32) + 9.0;
        let pal = self.rand_pal();
        let spr = SP_BIG_FISH + dir as u16;
        let mut e = visitor(spr, x, y, Z_SHARK, Y + BRIGHT).per_frame(dx, 0.0, 0.0);
        e.pal = pal;
        self.spawn(e);
    }

    fn add_ducks(&mut self) {
        let dir = self.rand_dir();
        let (x, dx) = self.start(dir, -30.0, -2, 1.0);
        let spr = SP_DUCKS + dir as u16;
        self.spawn(visitor(spr, x, 5.0, Z_WATER_GAP[3], W + BRIGHT).per_frame(dx, 0.0, 0.25));
    }

    fn add_swan(&mut self) {
        let dir = self.rand_dir();
        let (x, dx) = self.start(dir, -10.0, -2, 1.0);
        let spr = SP_SWAN + dir as u16;
        self.spawn(visitor(spr, x, 1.0, Z_WATER_GAP[3], W + BRIGHT).per_frame(dx, 0.0, 0.25));
    }

    fn add_shark(&mut self) {
        let dir = self.rand_dir();
        let y = self.rand_int((self.h - (10 + 9)) as f32) + 9.0;
        let (x, dx) = self.start(dir, -53.0, -2, 2.0);
        // The invisible "teeth" do the eating; they ride along inside the mouth.
        let teeth_x = if dir == 1 { x + 9.0 } else { -9.0 };
        self.spawn(
            Ent::new(Kind::Teeth, SP_TEETH, teeth_x, y + 7.0, Z_SHARK + 1, W)
                .per_frame(dx, 0.0, 0.0)
                .physical(Z_FISH_END - Z_FISH_START),
        );
        let spr = SP_SHARK + dir as u16;
        self.spawn(
            Ent::new(Kind::Shark, spr, x, y, Z_SHARK, C + BRIGHT)
                .per_frame(dx, 0.0, 0.0)
                .die_offscreen()
                .on_death(Death::Shark),
        );
    }

    fn add_fishhook(&mut self) {
        let x = 10.0 + self.rand_int((self.w - 20) as f32);
        let y = -4.0;
        let sink = Motion::Hook { hooked: false };
        let z = Z_WATER_LINE[1];
        self.spawn(Ent::new(Kind::Line, SP_LINE, x + 7.0, y - 50.0, z, W).motion(sink));
        self.spawn(
            Ent::new(Kind::Hook, SP_HOOK, x, y, z, G + BRIGHT)
                .motion(sink)
                .die_offscreen()
                .on_death(Death::Hook),
        );
        self.spawn(
            Ent::new(
                Kind::Point,
                SP_POINT,
                x + 1.0,
                y + 2.0,
                Z_SHARK + 1,
                G + BRIGHT,
            )
            .motion(sink)
            .physical(Z_FISH_END - Z_FISH_START),
        );
    }

    fn add_dolphins(&mut self) {
        let dir = self.rand_dir();
        let (x, dx) = self.start(dir, -13.0, -2, 1.0);
        // How far apart the dolphins are.
        let distance = if dir == 1 { -15.0 } else { 15.0 };
        let spr = SP_DOLPHIN + dir as u16;
        let z = Z_WATER_GAP[3];
        // They follow the same path a few frames apart. The original's path
        // also names a frame, but as 0.5, which always shows the first one.
        let dolphin = |x: f32, y: f32, phase: f32, def: u8| {
            Ent::new(Kind::Dolphin, spr, x, y, z, def)
                .per_frame(dx, 0.0, 0.0)
                .motion(Motion::Path { phase })
        };
        self.spawn(dolphin(x - distance * 2.0, 8.0, 0.0, B).on_death(Death::Random));
        self.spawn(dolphin(x - distance, 2.0, 12.0, B + BRIGHT));
        // The lead tells the others to die offscreen, since they start offscreen.
        self.spawn(
            dolphin(x, 5.0, 24.0, C + BRIGHT)
                .die_offscreen()
                .on_death(Death::DolphinLead),
        );
    }

    /// The fish bit the hook: reel in the fish, the hook, the line and the barb.
    fn retract(&mut self, fish: usize, point: usize) {
        let hooked = Motion::Hook { hooked: true };
        let p = &mut self.ents[point];
        p.physical = false;
        p.motion = hooked;
        let f = &mut self.ents[fish];
        f.physical = false;
        f.z = Z_WATER_GAP[2];
        f.motion = hooked;
        for e in &mut self.ents {
            if matches!(e.kind, Kind::Hook | Kind::Line) {
                e.motion = hooked;
            }
        }
    }

    /// One Term::Animation `animate()`: callbacks (with automatic deaths),
    /// collisions, then deaths and their callbacks.
    fn step(&mut self, dt: f32) {
        self.time += dt;
        // Clocks run on real time (see `clock`), motion on scene time.
        self.world_clock(dt * self.clock);
        if self.world.is_some() {
            self.environment(dt);
            self.events(dt);
        }
        let (sw, sh) = (self.w as f32, self.h as f32);
        // `int(rand(100)) > 97` once per original frame.
        let bubble_p = if self.bubbles {
            1.0 - 0.98f32.powf(TICK * dt)
        } else {
            0.0
        };
        let n = self.ents.len();
        for i in 0..n {
            let mut e = self.ents[i];
            if e.dead {
                continue;
            }
            // A plant's life is a clock; everything else's ttl is part of its motion.
            e.ttl -= if e.kind == Kind::Seaweed {
                dt * self.clock
            } else {
                dt
            };
            let offscreen = e.x >= sw || e.y >= sh || e.x < -e.w || e.y < -e.h;
            let time_up = e.ttl <= 0.0
                && (!matches!(e.kind, Kind::Fish | Kind::Critter)
                    || self.world.is_none()
                    || self.time_is_up(&mut e, offscreen));
            if time_up || (e.die_offscreen && offscreen && e.seen) {
                self.ents[i].dead = true;
                continue;
            }
            e.seen |= !offscreen;
            if let Some((dx, dy)) = e.emit
                && bubble_p > 0.0
                && self.rng.f32() < bubble_p * 1.5
            {
                self.add_bubble_at(e.x + dx as f32, e.y + dy as f32, e.z - 1);
            }
            let mut animate = true;
            match e.motion {
                Motion::Linear => {
                    if e.kind == Kind::Fish && bubble_p > 0.0 && self.rng.f32() < bubble_p {
                        self.add_bubble(&e);
                    }
                    e.x += e.vx * dt;
                    e.y += e.vy * dt;
                    // Food settles on the floor.
                    if e.kind == Kind::Food && e.y > sh - 1.0 {
                        e.y = sh - 1.0;
                        e.vx = 0.0;
                        e.vy = 0.0;
                    }
                }
                Motion::Orbit { .. } => self.orbit(&mut e, dt),
                Motion::Path { phase } => {
                    let next = phase + TICK * dt;
                    e.x += e.vx * dt;
                    e.y += dolphin_rise(next) - dolphin_rise(phase);
                    e.motion = Motion::Path {
                        phase: next.rem_euclid(36.0),
                    };
                }
                Motion::Swim { .. } => {
                    if e.kind == Kind::Fish && bubble_p > 0.0 && self.rng.f32() < bubble_p {
                        self.add_bubble(&e);
                    }
                    animate = self.swim(&mut e, dt);
                }
                Motion::Hook { hooked: true } => e.y -= TICK * dt,
                Motion::Hook { hooked: false } => {
                    // One row per frame until the bottom reaches 3/4 of the screen.
                    let limit = sh * 0.75;
                    if e.y + e.h < limit {
                        e.y = (e.y + TICK * dt).min((limit - e.h).ceil());
                    }
                }
            }
            if e.fps != 0.0 && animate {
                let frames = sprites()
                    .get(e.spr as usize)
                    .map_or(1, |s| s.frame_count())
                    .max(1) as f32;
                e.frame = (e.frame + e.fps * dt).rem_euclid(frames);
            }
            self.ents[i] = e;
        }
        if self.world.is_some() {
            self.ecology(dt);
            self.encounters(dt);
        }
        self.collide();
        self.reap();
    }

    /// The collisions asciiquarium reacts to: bubbles pop at the surface, fish
    /// touching the shark's teeth die, fish touching the barb get hooked.
    fn collide(&mut self) {
        self.targets.clear();
        for (i, e) in self.ents.iter().enumerate() {
            if e.physical && !e.dead && matches!(e.kind, Kind::Water(_) | Kind::Teeth | Kind::Point)
            {
                self.targets.push(i as u32);
            }
        }
        if self.targets.is_empty() {
            return;
        }
        let n = self.ents.len();
        for i in 0..n {
            let e = self.ents[i];
            if !e.physical || e.dead || !matches!(e.kind, Kind::Bubble | Kind::Fish) {
                continue;
            }
            for t in 0..self.targets.len() {
                let ti = self.targets[t] as usize;
                let o = self.ents[ti];
                if !o.physical || !overlap(&e, &o) {
                    continue;
                }
                match (e.kind, o.kind) {
                    (Kind::Bubble, Kind::Water(_)) => self.ents[i].dead = true,
                    (Kind::Fish, Kind::Teeth) => {
                        self.add_splat(o.x, o.y, o.z);
                        self.ents[i].dead = true;
                    }
                    (Kind::Fish, Kind::Point) => self.retract(i, ti),
                    _ => continue,
                }
                break;
            }
        }
    }

    /// Runs the death callbacks of everything that died, then removes it.
    fn reap(&mut self) {
        loop {
            let mut any = false;
            for i in 0..self.ents.len() {
                let e = self.ents[i];
                if !e.dead || e.reaped {
                    continue;
                }
                any = true;
                self.ents[i].reaped = true;
                match e.death {
                    Death::None => {}
                    Death::Fish => self.add_fish(),
                    Death::Seaweed if self.world.is_some() => self.add_plant(),
                    Death::Seaweed => self.add_seaweed(),
                    Death::Group(g) => self.group_gone(g),
                    Death::Random => self.random_object(),
                    Death::Shark => {
                        self.kill_kinds(&[Kind::Teeth]);
                        self.random_object();
                    }
                    Death::Hook => {
                        self.kill_kinds(&[Kind::Teeth, Kind::Line, Kind::Point]);
                        self.random_object();
                    }
                    Death::DolphinLead => {
                        for d in &mut self.ents {
                            if d.kind == Kind::Dolphin {
                                d.die_offscreen = true;
                            }
                        }
                    }
                }
            }
            if !any {
                break;
            }
        }
        self.ents.retain(|e| !e.dead);
    }

    fn kill_kinds(&mut self, kinds: &[Kind]) {
        for e in &mut self.ents {
            if kinds.contains(&e.kind) {
                e.dead = true;
            }
        }
    }

    fn draw_ent(&self, e: &Ent, c: &mut Canvas) {
        let x0 = e.x.floor().clamp(-1e6, 1e6) as i32;
        let y0 = e.y.floor().clamp(-1e6, 1e6) as i32;
        if let Kind::Water(i) = e.kind {
            let row = art::WATER[i as usize].as_bytes();
            let (col, attr) = self.water_pen(i as usize);
            let shift = if self.waves {
                // Neighboring rows drift in opposite directions.
                let v = [0.0, 1.2, -0.9, 0.6][i as usize];
                (self.time * v).floor() as i64
            } else {
                0
            } + self.storm_shift(i as usize);
            for x in 0..self.w {
                let ch = row[(x as i64 - shift).rem_euclid(row.len() as i64) as usize] as char;
                if ch == ' ' {
                    c.set_attr(x, y0, ' ', Color::Reset, 0);
                } else {
                    c.set_attr(x, y0, ch, col, attr);
                }
            }
            return;
        }
        let Some(s) = sprites().get(e.spr as usize) else {
            return;
        };
        // Skip sprites entirely off the canvas.
        if x0 >= self.w || y0 >= self.h || x0 + s.w <= 0 || y0 + s.h <= 0 {
            return;
        }
        let pens = self.pen_table(e);
        for cell in s.frame(e.frame as usize) {
            let (x, y) = (x0 + cell.dx as i32, y0 + cell.dy as i32);
            if cell.ch == ' ' {
                // Opaque but invisible: erase what is behind. `set_attr` leaves the cell's
                // background alone, so the terminal's shows, or the painted sea's.
                c.set_attr(x, y, ' ', Color::Reset, 0);
            } else {
                let (col, attr) = pens[e.pen(cell.code) as usize];
                c.set_attr(x, y, cell.ch, col, attr);
            }
        }
        // At the Navidad the plants wear the series' lights.
        if e.kind == Kind::Seaweed
            && self
                .world
                .as_ref()
                .is_some_and(|w| w.season == assets::NAVIDAD)
        {
            self.dress_plant(e, c);
        }
    }
}

/// Under `classic`, the terminal's own palette, like curses: color n, bold for uppercase.
fn classic_pens() -> [(Color, u8); 16] {
    let mut p = [(Color::Reset, 0); 16];
    for (i, slot) in p.iter_mut().enumerate() {
        let n = (i & 7) as u8;
        *slot = (Color::Ansi(n), if i >= 8 { BOLD } else { 0 });
    }
    // Bold black (the castle) shows as grey where bold means bright; ask for
    // bright black directly so it is not black on black elsewhere.
    p[K as usize] = (Color::Ansi(8), 0);
    p[(K + BRIGHT) as usize] = (Color::Ansi(8), BOLD);
    p
}

fn rgb_of(c: Color) -> Option<(u8, u8, u8)> {
    match c {
        Color::Rgb(r, g, b) => Some((r, g, b)),
        Color::Ansi(n) => Some(color::ansi_rgb(n)),
        Color::Reset => None,
    }
}

/// Hue in degrees and saturation (0-1) of a color.
fn hue_sat(c: Color) -> (f32, f32) {
    let Some((r, g, b)) = rgb_of(c) else {
        return (0.0, 0.0);
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    if d <= f32::EPSILON {
        return (0.0, 0.0);
    }
    let h = if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    (h, d / max)
}

/// For each of red, yellow, green, cyan, blue and magenta, the theme accent
/// closest in hue, keeping them distinct while there are enough accents.
fn assign_accents(colors: &[Color]) -> [usize; 6] {
    const HUES: [f32; 6] = [0.0, 60.0, 120.0, 180.0, 240.0, 300.0];
    let n = colors.len().max(1);
    let hs: Vec<(f32, f32)> = colors.iter().map(|&c| hue_sat(c)).collect();
    let dist = |k: usize, j: usize| {
        let (h, s) = hs.get(j).copied().unwrap_or((0.0, 0.0));
        let dh = (h - HUES[k]).abs();
        dh.min(360.0 - dh) / 180.0 + (1.0 - s) * 0.6
    };
    let mut out = [usize::MAX; 6];
    let mut used = vec![false; n];
    for _ in 0..6 {
        if used.iter().all(|&u| u) {
            used.fill(false);
        }
        let mut best = (f32::MAX, 0, 0);
        for k in (0..6).filter(|&k| out[k] == usize::MAX) {
            for j in (0..n).filter(|&j| !used[j]) {
                let d = dist(k, j);
                if d < best.0 {
                    best = (d, k, j);
                }
            }
        }
        out[best.1] = best.2;
        used[best.2] = true;
    }
    out
}

impl Scene for Aquarium {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.w = w as i32;
        self.h = h as i32;
        self.rebuild();
    }

    fn retheme(&mut self, ctx: &Ctx) {
        // The holiday may have come or gone (at midnight, or a new config).
        self.retheme_season(ctx);
        let th = ctx.theme;
        if let Some(w) = self.world.as_mut() {
            w.tint = (!th.native).then(|| {
                let g: Gradient = ctx.ramp(&[]);
                (g.at(0.6), g.at(0.1))
            });
        }
        if th.native {
            self.pens = classic_pens();
            self.water = [(Color::Ansi(C), 0); 4];
            self.retheme_world(ctx);
            return;
        }
        let grad: Gradient = ctx.ramp(&[]);
        let fg = th.fg;
        let mut base = [Color::Reset; 8];
        base[W as usize] = fg;
        base[K as usize] = if fg.is_rgb() {
            ctx.fade(fg, 0.5)
        } else {
            grad.at(0.3)
        };
        let idx = assign_accents(&th.colors);
        for (k, pen) in [R, Y, G, C, B, M].into_iter().enumerate() {
            base[pen as usize] = if th.colors.is_empty() {
                fg
            } else {
                ctx.pick(idx[k], Color::Ansi(pen))
            };
        }
        // Uppercase (bright) is the accent itself in bold; lowercase sinks a
        // little toward the background, so two-tone creatures stay two-tone.
        for (i, &b) in base.iter().enumerate() {
            self.pens[i] = (ctx.fade(b, 0.3), 0);
            self.pens[i + BRIGHT as usize] = (b, BOLD);
        }
        // The surface follows the theme ramp, brightest on top. The lower middle
        // of a ramp is usually its cool, watery part.
        for (i, slot) in self.water.iter_mut().enumerate() {
            *slot = (grad.at(0.55 - 0.08 * i as f32), 0);
        }
        self.retheme_world(ctx);
    }

    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        let scene = dt * self.speed;
        if scene.is_finite() && scene > 0.0 && real.is_finite() && real > 0.0 {
            self.clock = real / scene;
        }
        self.update(ctx, dt);
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = dt * self.speed;
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        // After a long pause, catch up a second at most.
        let dt = dt.min(1.0);
        if self.smooth {
            let n = (dt / MAX_STEP).ceil().max(1.0);
            for _ in 0..n as usize {
                self.step(dt / n);
            }
        } else {
            self.pending = (self.pending + dt).min(1.0);
            while self.pending >= 1.0 / TICK {
                self.pending -= 1.0 / TICK;
                self.step(1.0 / TICK);
            }
        }
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        if self.world.is_some() {
            self.prepare_draw();
            self.draw_sky(c);
        }
        self.draw_backdrop(c);
        if self.world.is_some() {
            self.draw_event_backdrop(c);
        }
        let ents = &self.ents;
        self.order.clear();
        self.order.extend(0..ents.len() as u32);
        self.order
            .sort_unstable_by_key(|&i| (Reverse(ents[i as usize].z), ents[i as usize].id));
        for &i in &self.order {
            self.draw_ent(&self.ents[i as usize], c);
        }
        if self.world.is_some() {
            self.draw_env_overlay(c);
            self.draw_event_overlay(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_sizes_match_term_animation() {
        // (sprite, width, height, frames) as Term::Animation builds them.
        let expect: &[(u16, i32, i32, usize)] = &[
            (SP_CASTLE, 31, 13, 1),
            (SP_FISH, 14, 6, 1),
            (SP_FISH + 4, 16, 5, 1),
            (SP_FISH + 15, 5, 3, 1),
            (SP_SPLAT, 9, 5, 4),
            (SP_SHARK, 53, 10, 1),
            (SP_HOOK, 8, 6, 1),
            (SP_LINE, 1, 56, 1),
            (SP_POINT, 1, 4, 1),
            (SP_SHIP, 25, 6, 1),
            (SP_WHALE, 19, 7, 12),
            (SP_MONSTER + 1, 66, 5, 4),
            (SP_BIG_FISH, 34, 14, 1),
            (SP_DUCKS, 31, 3, 3),
            (SP_DOLPHIN, 14, 4, 2),
            (SP_SWAN + 1, 11, 7, 1),
            (SP_BUBBLE, 1, 1, 5),
            (SP_SEAWEED + 3, 2, 6, 2),
        ];
        assert_eq!(build_sprites().len(), SP_COUNT);
        let s = sprites();
        for &(i, w, h, f) in expect {
            let sp = &s[i as usize];
            assert_eq!((sp.w, sp.h, sp.frame_count()), (w, h, f), "sprite {i}");
        }
    }

    fn with_scene(f: impl FnOnce(&Ctx, &mut Aquarium)) {
        let theme = crate::theme::Theme::classic();
        let mut t = toml::Table::new();
        t.insert("biome".into(), "classic".into());
        let opts = crate::scene::Opts::new(INFO.options, t);
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 11,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut a = Aquarium::new(&ctx);
        a.retheme(&ctx);
        f(&ctx, &mut a);
    }

    #[test]
    fn spaces_are_opaque_except_leading_ones() {
        with_scene(|_, a| {
            a.w = 20;
            a.h = 8;
            let mut c = Canvas::new(20, 8, Color::Reset);
            for y in 0..8 {
                c.text(0, y, &"#".repeat(20), Color::Reset);
            }
            // "    \\", "\\ /--\\", ">=  (o>", ...
            a.draw_ent(&Ent::new(Kind::Fish, SP_FISH + 2, 0.0, 0.0, 5, W), &mut c);
            let ch = |x, y| c.get(x, y).unwrap().ch;
            assert_eq!(ch(0, 0), '#', "leading space is transparent");
            assert_eq!(ch(4, 0), '\\');
            assert_eq!(ch(5, 0), '#', "nothing past the end of a line");
            assert_eq!(ch(2, 2), ' ', "inner space erases");
            assert_eq!(c.get(2, 2).unwrap().bg, Color::Reset);
        });
    }

    #[test]
    fn population_is_stable_and_visitors_keep_coming() {
        with_scene(|ctx, a| {
            a.resize(ctx, 120, 40);
            let mut last = None;
            let mut visitors = 0;
            for _ in 0..30 * 300 {
                a.update(ctx, 1.0 / 30.0);
                let fish = a.ents.iter().filter(|e| e.kind == Kind::Fish).count();
                assert_eq!(fish, 10);
                let now = a
                    .ents
                    .iter()
                    .find(|e| {
                        matches!(
                            e.kind,
                            Kind::Visitor | Kind::Shark | Kind::Hook | Kind::Dolphin
                        )
                    })
                    .map(|e| e.id);
                if now.is_some() && now != last {
                    visitors += 1;
                }
                last = now;
            }
            // Five minutes see a good dozen ships, whales, sharks and friends.
            assert!(visitors >= 12, "{visitors}");
        });
    }

    #[test]
    fn dolphin_path_is_periodic() {
        assert_eq!(dolphin_rise(0.0), 0.0);
        assert_eq!(dolphin_rise(14.0), -7.0);
        assert_eq!(dolphin_rise(30.0), 0.0);
        assert_eq!(dolphin_rise(36.0 + 7.0), -3.5);
    }

    #[test]
    fn accents_are_distinct_when_possible() {
        let th = crate::theme::Theme::builtin("dracula").unwrap();
        let idx = assign_accents(&th.colors);
        let mut seen = idx.to_vec();
        seen.sort();
        seen.dedup();
        assert_eq!(seen.len(), 6);
        // Dracula's red is #ff5555.
        assert_eq!(th.colors[idx[0]], Color::Rgb(0xff, 0x55, 0x55));
    }
}
