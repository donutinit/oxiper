//! The aquarium's cast beyond asciiquarium's own: fish and other creatures,
//! plants and corals, props for the sea floor and passers-by. Data only; the
//! engine in `../mod.rs` brings them to life.
//!
//! Every shape follows asciiquarium's conventions (see `../sprite.rs`):
//!
//! - a raw string that starts with a newline, one line per row;
//! - shapes are "auto transparent": the blanks that start a line are not drawn,
//!   nor is anything past the end of a line, but every other space is opaque,
//!   so a fish hides the plant behind it; `?` is an explicitly transparent cell;
//! - a color mask has the same layout as its shape. A letter is a fixed color
//!   (`r g y b m c w k`, uppercase = bright/bold), a space means the default
//!   color, and a digit `1`-`9` is a slot filled when the creature spawns,
//!   from its `tint` or, without one, at random like asciiquarium's fish.
//!   Slot `4` (the eye) is bright white unless tinted;
//! - creatures and passers-by are drawn facing right; the left-facing version
//!   is mirrored (`( ) [ ] { } < > / \` swap, and so do `` ` `` and `'`) unless
//!   one is drawn by hand.
//!
//! Speeds are in cells per original frame, as in asciiquarium (which draws ten
//! frames a second): its fish swim at 0.25 to 2.25, its ship at 1, the shark
//! at 2. Animation rates are frames advanced per original frame (0.25 = a new
//! frame every 0.4 s).

mod critters;
mod decor;
mod deep;
mod floor;
mod fresh;
mod ocean;
mod oddballs;
mod plants;
mod reef;
mod visitors;

use super::art;
use super::sprite::{self, Sprite};

/// Places a thing belongs to, as a set of bits.
pub type Biomes = u16;
/// Tropical coral reef.
pub const REEF: Biomes = 1 << 0;
/// Open ocean, the blue water far from shore.
pub const OCEAN: Biomes = 1 << 1;
/// Temperate coasts and kelp forests.
pub const KELP: Biomes = 1 << 2;
/// The deep sea: no surface, no sunlight.
pub const DEEP: Biomes = 1 << 3;
/// Rivers and lakes.
pub const FRESH: Biomes = 1 << 4;
/// Polar seas.
pub const ARCTIC: Biomes = 1 << 5;
/// A home aquarium, castle and all.
pub const TANK: Biomes = 1 << 6;
pub const ALL: Biomes = REEF | OCEAN | KELP | DEEP | FRESH | ARCTIC | TANK;

pub const BIOMES: [(&str, Biomes); 7] = [
    ("reef", REEF),
    ("ocean", OCEAN),
    ("kelp", KELP),
    ("deep", DEEP),
    ("fresh", FRESH),
    ("arctic", ARCTIC),
    ("tank", TANK),
];

/// Names of the biomes in a set, comma separated.
pub fn biome_names(b: Biomes) -> String {
    BIOMES
        .iter()
        .filter(|(_, bit)| b & bit != 0)
        .map(|(n, _)| *n)
        .collect::<Vec<_>>()
        .join(",")
}

/// Which band of the water a creature keeps to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Zone {
    /// Just below the surface.
    Top,
    Upper,
    Middle,
    Lower,
    /// On the sea floor: the bottom row of the shape touches the bottom of
    /// the screen. Use it with `Swim::Crawl`.
    Bottom,
    Any,
}

/// How a creature moves.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Swim {
    /// Straight across, like asciiquarium's fish.
    Cruise,
    /// Across while gently rising and sinking.
    Wave,
    /// In bursts: quick dashes and slow glides.
    Dart,
    /// Barely forward, bobbing up and down a few rows (jellyfish).
    Drift,
    /// Slowly, with a small bob (seahorses).
    Hover,
    /// Along the sea floor with pauses (crabs, snails). Animated only while moving.
    Crawl,
}

/// A fish or another creature that swims (or crawls) across the screen.
#[derive(Clone, Copy, Debug)]
pub struct Species {
    pub name: &'static str,
    /// Frames facing right.
    pub art: &'static [&'static str],
    /// One mask for all frames, one per frame, or none.
    pub mask: &'static [&'static str],
    /// Hand-drawn left-facing frames and masks; empty to mirror `art`.
    pub left: &'static [&'static str],
    pub left_mask: &'static [&'static str],
    /// Choices for mask digits, like `"1:yY 2:wW 6:rR"`: slot 1 is yellow or
    /// bright yellow, and so on. Untinted slots get a random color.
    pub tint: &'static str,
    /// Color of unmasked cells, a mask letter.
    pub color: char,
    pub swim: Swim,
    pub zone: Zone,
    /// Speed range in cells per original frame.
    pub speed: (f32, f32),
    /// Animation frames per original frame.
    pub fps: f32,
    /// How many swim together (min, max); (1, 1) for loners.
    pub school: (u8, u8),
    /// Relative frequency, 1 (rare) to 10 (common).
    pub weight: u8,
    pub biomes: Biomes,
    /// Fish blow bubbles and can be eaten by the shark or caught on the hook.
    pub fish: bool,
}

impl Species {
    pub const fn new(
        name: &'static str,
        art: &'static [&'static str],
        mask: &'static [&'static str],
    ) -> Species {
        Species {
            name,
            art,
            mask,
            left: &[],
            left_mask: &[],
            tint: "",
            color: 'w',
            swim: Swim::Cruise,
            zone: Zone::Any,
            speed: (0.25, 2.25),
            fps: 0.0,
            school: (1, 1),
            weight: 5,
            biomes: 0,
            fish: true,
        }
    }
    pub const fn left(
        mut self,
        art: &'static [&'static str],
        mask: &'static [&'static str],
    ) -> Species {
        self.left = art;
        self.left_mask = mask;
        self
    }
    pub const fn tint(mut self, t: &'static str) -> Species {
        self.tint = t;
        self
    }
    pub const fn color(mut self, c: char) -> Species {
        self.color = c;
        self
    }
    pub const fn swim(mut self, s: Swim) -> Species {
        self.swim = s;
        self
    }
    pub const fn zone(mut self, z: Zone) -> Species {
        self.zone = z;
        self
    }
    pub const fn speed(mut self, min: f32, max: f32) -> Species {
        self.speed = (min, max);
        self
    }
    pub const fn fps(mut self, f: f32) -> Species {
        self.fps = f;
        self
    }
    pub const fn school(mut self, min: u8, max: u8) -> Species {
        self.school = (min, max);
        self
    }
    pub const fn weight(mut self, w: u8) -> Species {
        self.weight = w;
        self
    }
    pub const fn biomes(mut self, b: Biomes) -> Species {
        self.biomes = b;
        self
    }
    /// Not a fish: no bubbles, and the shark and the hook leave it alone.
    pub const fn critter(mut self) -> Species {
        self.fish = false;
        self
    }
}

/// How a plant is drawn.
#[derive(Clone, Copy, Debug)]
pub enum PlantArt {
    /// Fixed frames (the plant sways by cycling them).
    Sprite {
        frames: &'static [&'static str],
        masks: &'static [&'static str],
    },
    /// A stalk grown to a random height like asciiquarium's seaweed, which is
    /// `stalk("(", " )", 3, 6)`: row by row it alternates the two pieces, and
    /// the second frame swaps them, so it sways.
    Stalk {
        a: &'static str,
        b: &'static str,
        min: u8,
        max: u8,
    },
}

/// Seaweed, kelp, grass, anemones, corals, sponges: rooted in the floor.
#[derive(Clone, Copy, Debug)]
pub struct Plant {
    pub name: &'static str,
    pub art: PlantArt,
    pub tint: &'static str,
    pub color: char,
    /// Sway rate in frames per original frame (asciiquarium's seaweed: 0.25-0.3).
    pub fps: f32,
    pub weight: u8,
    pub biomes: Biomes,
}

impl Plant {
    pub const fn sprite(
        name: &'static str,
        frames: &'static [&'static str],
        masks: &'static [&'static str],
    ) -> Plant {
        Plant {
            name,
            art: PlantArt::Sprite { frames, masks },
            tint: "",
            color: 'g',
            fps: 0.25,
            weight: 5,
            biomes: 0,
        }
    }
    pub const fn stalk(
        name: &'static str,
        a: &'static str,
        b: &'static str,
        min: u8,
        max: u8,
    ) -> Plant {
        Plant {
            name,
            art: PlantArt::Stalk { a, b, min, max },
            tint: "",
            color: 'g',
            fps: 0.25,
            weight: 5,
            biomes: 0,
        }
    }
    pub const fn tint(mut self, t: &'static str) -> Plant {
        self.tint = t;
        self
    }
    pub const fn color(mut self, c: char) -> Plant {
        self.color = c;
        self
    }
    pub const fn fps(mut self, f: f32) -> Plant {
        self.fps = f;
        self
    }
    pub const fn weight(mut self, w: u8) -> Plant {
        self.weight = w;
        self
    }
    pub const fn biomes(mut self, b: Biomes) -> Plant {
        self.biomes = b;
        self
    }
}

/// A prop that sits on the sea floor: wrecks, ruins, chests, rocks.
#[derive(Clone, Copy, Debug)]
pub struct Decor {
    pub name: &'static str,
    pub art: &'static [&'static str],
    pub mask: &'static [&'static str],
    pub tint: &'static str,
    pub color: char,
    /// Animation frames per original frame (0 = still).
    pub fps: f32,
    /// Where bubbles rise from, relative to the top left corner, if anywhere.
    pub bubbles: Option<(i8, i8)>,
    pub weight: u8,
    pub biomes: Biomes,
}

impl Decor {
    pub const fn new(
        name: &'static str,
        art: &'static [&'static str],
        mask: &'static [&'static str],
    ) -> Decor {
        Decor {
            name,
            art,
            mask,
            tint: "",
            color: 'w',
            fps: 0.0,
            bubbles: None,
            weight: 5,
            biomes: 0,
        }
    }
    pub const fn tint(mut self, t: &'static str) -> Decor {
        self.tint = t;
        self
    }
    pub const fn color(mut self, c: char) -> Decor {
        self.color = c;
        self
    }
    pub const fn fps(mut self, f: f32) -> Decor {
        self.fps = f;
        self
    }
    pub const fn bubbles(mut self, dx: i8, dy: i8) -> Decor {
        self.bubbles = Some((dx, dy));
        self
    }
    pub const fn weight(mut self, w: u8) -> Decor {
        self.weight = w;
        self
    }
    pub const fn biomes(mut self, b: Biomes) -> Decor {
        self.biomes = b;
        self
    }
}

/// Where a passer-by travels.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lane {
    /// Flies over the water, in the five rows above the waves (at most 5 rows tall).
    Sky,
    /// Sails the surface: its last row sits on the first wave row, like
    /// asciiquarium's ship (at most 6 rows tall).
    Boat,
    /// Floats low in the water: its last row sits on the third wave row, like
    /// asciiquarium's ducks (at most 8 rows tall).
    Float,
    /// Swims underwater at a random depth, like asciiquarium's big fish.
    Deep,
    /// Travels along the sea floor.
    Floor,
}

/// Something big that crosses the screen once: boats, birds, whales, divers.
#[derive(Clone, Copy, Debug)]
pub struct Visitor {
    pub name: &'static str,
    /// Frames facing right.
    pub art: &'static [&'static str],
    pub mask: &'static [&'static str],
    /// Hand-drawn left-facing frames and masks; empty to mirror `art`.
    pub left: &'static [&'static str],
    pub left_mask: &'static [&'static str],
    pub tint: &'static str,
    pub color: char,
    pub lane: Lane,
    pub speed: (f32, f32),
    pub fps: f32,
    pub weight: u8,
    pub biomes: Biomes,
}

impl Visitor {
    pub const fn new(
        name: &'static str,
        lane: Lane,
        art: &'static [&'static str],
        mask: &'static [&'static str],
    ) -> Visitor {
        Visitor {
            name,
            art,
            mask,
            left: &[],
            left_mask: &[],
            tint: "",
            color: 'w',
            lane,
            speed: (1.0, 1.0),
            fps: 0.0,
            weight: 5,
            biomes: 0,
        }
    }
    pub const fn left(
        mut self,
        art: &'static [&'static str],
        mask: &'static [&'static str],
    ) -> Visitor {
        self.left = art;
        self.left_mask = mask;
        self
    }
    pub const fn tint(mut self, t: &'static str) -> Visitor {
        self.tint = t;
        self
    }
    #[allow(dead_code)] // asset API
    pub const fn color(mut self, c: char) -> Visitor {
        self.color = c;
        self
    }
    pub const fn speed(mut self, min: f32, max: f32) -> Visitor {
        self.speed = (min, max);
        self
    }
    pub const fn fps(mut self, f: f32) -> Visitor {
        self.fps = f;
        self
    }
    pub const fn weight(mut self, w: u8) -> Visitor {
        self.weight = w;
        self
    }
    pub const fn biomes(mut self, b: Biomes) -> Visitor {
        self.biomes = b;
        self
    }
}

/// asciiquarium's own eight fish, colored at random as there.
const fn classic(
    name: &'static str,
    r: &'static [&'static str],
    rm: &'static [&'static str],
    l: &'static [&'static str],
    lm: &'static [&'static str],
) -> Species {
    Species::new(name, r, rm)
        .left(l, lm)
        .biomes(TANK | REEF | KELP)
}

const CLASSIC: &[Species] = &[
    classic(
        "classic-1",
        &[art::FISH_0R],
        &[art::FISH_0R_MASK],
        &[art::FISH_0L],
        &[art::FISH_0L_MASK],
    ),
    classic(
        "classic-2",
        &[art::FISH_1R],
        &[art::FISH_1R_MASK],
        &[art::FISH_1L],
        &[art::FISH_1L_MASK],
    ),
    classic(
        "classic-3",
        &[art::FISH_2R],
        &[art::FISH_2R_MASK],
        &[art::FISH_2L],
        &[art::FISH_2L_MASK],
    ),
    classic(
        "classic-4",
        &[art::FISH_3R],
        &[art::FISH_3R_MASK],
        &[art::FISH_3L],
        &[art::FISH_3L_MASK],
    ),
    classic(
        "classic-5",
        &[art::FISH_4R],
        &[art::FISH_4R_MASK],
        &[art::FISH_4L],
        &[art::FISH_4L_MASK],
    ),
    classic(
        "classic-6",
        &[art::FISH_5R],
        &[art::FISH_5R_MASK],
        &[art::FISH_5L],
        &[art::FISH_5L_MASK],
    ),
    classic(
        "classic-7",
        &[art::FISH_6R],
        &[art::FISH_6R_MASK],
        &[art::FISH_6L],
        &[art::FISH_6L_MASK],
    ),
    classic(
        "classic-8",
        &[art::FISH_7R],
        &[art::FISH_7R_MASK],
        &[art::FISH_7L],
        &[art::FISH_7L_MASK],
    ),
];

/// All the species, in a stable order.
pub fn species() -> impl Iterator<Item = &'static Species> {
    [
        CLASSIC,
        reef::SPECIES,
        ocean::SPECIES,
        fresh::SPECIES,
        deep::SPECIES,
        oddballs::SPECIES,
        critters::SPECIES,
        floor::SPECIES,
    ]
    .into_iter()
    .flatten()
}

pub fn plants() -> impl Iterator<Item = &'static Plant> {
    plants::PLANTS.iter()
}

pub fn decor() -> impl Iterator<Item = &'static Decor> {
    decor::DECOR.iter()
}

pub fn visitors() -> impl Iterator<Item = &'static Visitor> {
    visitors::VISITORS.iter()
}

/// Mask digit choices, parsed from a `tint` string.
#[derive(Clone, Copy, Debug, Default)]
pub struct Tint {
    pens: [[u8; 6]; 10],
    len: [u8; 10],
}

impl Tint {
    /// Parses `"1:yY 2:w"`. Malformed parts are reported, not ignored.
    pub fn parse(s: &str) -> Result<Tint, String> {
        let mut t = Tint::default();
        for part in s.split_whitespace() {
            let (d, letters) = part
                .split_once(':')
                .ok_or_else(|| format!("'{part}' is not slot:colors"))?;
            let d = match d.as_bytes() {
                [c @ b'1'..=b'9'] => (c - b'0') as usize,
                _ => return Err(format!("'{d}' is not a slot 1-9")),
            };
            if letters.is_empty() || letters.chars().count() > 6 {
                return Err(format!("slot {d} needs 1 to 6 colors"));
            }
            for (i, l) in letters.chars().enumerate() {
                t.pens[d][i] = sprite::pen(l).ok_or_else(|| format!("'{l}' is not a color"))?;
            }
            t.len[d] = letters.chars().count() as u8;
        }
        Ok(t)
    }

    /// The choices for slot `d` (empty when untinted).
    pub fn choices(&self, d: usize) -> &[u8] {
        &self.pens[d][..self.len[d] as usize]
    }
}

/// The left-facing frames and masks of a creature: hand-drawn, or mirrored.
pub fn facing_left(
    art: &'static [&'static str],
    mask: &'static [&'static str],
    left: &'static [&'static str],
    left_mask: &'static [&'static str],
) -> (Vec<String>, Vec<String>) {
    if left.is_empty() {
        sprite::mirror(art, mask)
    } else {
        (
            left.iter().map(|s| s.to_string()).collect(),
            left_mask.iter().map(|s| s.to_string()).collect(),
        )
    }
}

/// The two frames of a stalk plant of the given height.
pub fn stalk_frames(a: &str, b: &str, height: usize) -> [String; 2] {
    let mut frames = [String::from("\n"), String::from("\n")];
    for i in 1..=height {
        let left = i % 2;
        frames[left].push_str(a);
        frames[left].push('\n');
        frames[1 - left].push_str(b);
        frames[1 - left].push('\n');
    }
    frames
}

/// Renders assets as text for checking art: each one's name and biomes, then
/// its frames side by side (creatures facing right, then left). `filter`
/// matches names (substring) or biome names; `color` keeps ANSI colors.
pub fn gallery(filter: &str, color: bool) -> String {
    let biome = BIOMES.iter().find(|(n, _)| *n == filter).map(|b| b.1);
    let wanted = |name: &str, b: Biomes| match biome {
        Some(bit) => b & bit != 0,
        None => filter.is_empty() || name.contains(filter),
    };
    let mut out = String::new();
    let mut section = |title: &str, items: Vec<Item>| {
        if items.is_empty() {
            return;
        }
        out.push_str(&format!("\n=== {title} ({}) ===\n", items.len()));
        for it in items {
            out.push_str(&format!(
                "\n{} [{}] {}\n",
                it.name,
                biome_names(it.biomes),
                it.note
            ));
            out.push_str(&side_by_side(&it.sprites, it.color, it.tint, color));
        }
    };
    let mut fish = Vec::new();
    let mut critters = Vec::new();
    for s in species().filter(|s| wanted(s.name, s.biomes)) {
        let (l, lm) = facing_left(s.art, s.mask, s.left, s.left_mask);
        let mut sprites: Vec<Sprite> = (0..s.art.len())
            .map(|f| frame_sprite(s.art, s.mask, f))
            .collect();
        sprites.extend((0..l.len()).map(|f| frame_sprite(&l, &lm, f)));
        let note = format!(
            "{:?} {:?} speed {}-{} school {}-{} weight {}",
            s.swim, s.zone, s.speed.0, s.speed.1, s.school.0, s.school.1, s.weight
        );
        let item = Item::new(s.name, s.biomes, note, sprites, s.color, s.tint);
        if s.fish {
            fish.push(item);
        } else {
            critters.push(item);
        }
    }
    section("fish", fish);
    section("critters", critters);
    let plants = plants()
        .filter(|p| wanted(p.name, p.biomes))
        .map(|p| {
            let sprites = match p.art {
                PlantArt::Sprite { frames, masks } => (0..frames.len())
                    .map(|f| frame_sprite(frames, masks, f))
                    .collect(),
                PlantArt::Stalk { a, b, max, .. } => stalk_frames(a, b, max as usize)
                    .iter()
                    .map(|f| Sprite::new(&[f], &[], false, '?'))
                    .collect(),
            };
            Item::new(
                p.name,
                p.biomes,
                format!("weight {}", p.weight),
                sprites,
                p.color,
                p.tint,
            )
        })
        .collect();
    section("plants", plants);
    let decor = decor()
        .filter(|d| wanted(d.name, d.biomes))
        .map(|d| {
            let sprites = (0..d.art.len())
                .map(|f| frame_sprite(d.art, d.mask, f))
                .collect();
            let note = match d.bubbles {
                Some((x, y)) => format!("bubbles at {x},{y}"),
                None => String::new(),
            };
            Item::new(d.name, d.biomes, note, sprites, d.color, d.tint)
        })
        .collect();
    section("decor", decor);
    let visitors = visitors()
        .filter(|v| wanted(v.name, v.biomes))
        .map(|v| {
            let (l, lm) = facing_left(v.art, v.mask, v.left, v.left_mask);
            let mut sprites: Vec<Sprite> = (0..v.art.len())
                .map(|f| frame_sprite(v.art, v.mask, f))
                .collect();
            sprites.extend((0..l.len()).map(|f| frame_sprite(&l, &lm, f)));
            let note = format!("{:?} speed {}-{}", v.lane, v.speed.0, v.speed.1);
            Item::new(v.name, v.biomes, note, sprites, v.color, v.tint)
        })
        .collect();
    section("visitors", visitors);
    out
}

struct Item {
    name: &'static str,
    biomes: Biomes,
    note: String,
    sprites: Vec<Sprite>,
    color: char,
    tint: &'static str,
}

impl Item {
    fn new(
        name: &'static str,
        biomes: Biomes,
        note: String,
        sprites: Vec<Sprite>,
        color: char,
        tint: &'static str,
    ) -> Item {
        Item {
            name,
            biomes,
            note,
            sprites,
            color,
            tint,
        }
    }
}

/// Frame `f` of a shape as its own sprite, with its mask (one for all, or per frame).
fn frame_sprite<S: AsRef<str>>(frames: &[S], masks: &[S], f: usize) -> Sprite {
    let mask: Vec<&str> = masks
        .get(f % masks.len().max(1))
        .map(|m| m.as_ref())
        .into_iter()
        .collect();
    Sprite::new(&[frames[f].as_ref()], &mask, true, '?')
}

/// Frames next to each other, transparent cells shown as `·` so the outline is visible.
fn side_by_side(sprites: &[Sprite], def: char, tint: &str, color: bool) -> String {
    let def = sprite::pen(def).unwrap_or(7);
    let tint = Tint::parse(tint).unwrap_or_default();
    let h = sprites.iter().map(|s| s.h).max().unwrap_or(0) as usize;
    let mut grid: Vec<Vec<(char, u8)>> = vec![Vec::new(); h];
    for s in sprites {
        let w = s.w as usize;
        let start = grid.first().map_or(0, |r| r.len());
        for row in grid.iter_mut() {
            row.resize(start + w + 3, ('·', 255));
        }
        for c in s.frame(0) {
            let pen = if c.code == sprite::DEFAULT {
                def
            } else if c.code > sprite::DIGIT {
                let d = (c.code - sprite::DIGIT) as usize;
                // The first choice, or a fixed color per slot when untinted.
                tint.choices(d).first().copied().unwrap_or(match d {
                    4 => 15,
                    _ => [9, 11, 10, 14, 12, 13, 3, 6, 5][d - 1],
                })
            } else {
                c.code
            };
            grid[c.dy as usize][start + c.dx as usize] = (c.ch, pen);
        }
        for row in grid.iter_mut() {
            for cell in &mut row[start + w..] {
                *cell = (' ', 255);
            }
        }
    }
    let mut out = String::new();
    for row in grid {
        let mut last = 255;
        for (ch, pen) in row {
            if color && pen != last {
                if pen == 255 {
                    out.push_str("\x1b[0;2m");
                } else {
                    let base = 30 + (pen & 7);
                    let bold = if pen >= 8 { "1;" } else { "" };
                    out.push_str(&format!("\x1b[0;{bold}{base}m"));
                }
                last = pen;
            }
            out.push(ch);
        }
        if color {
            out.push_str("\x1b[0m");
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Shape lines without the leading newline, as the sprite builder sees them.
    fn lines(s: &str) -> Vec<&str> {
        let s = s.strip_prefix('\n').unwrap_or(s);
        let mut v: Vec<&str> = s.split('\n').collect();
        while v.last().is_some_and(|l| l.is_empty()) {
            v.pop();
        }
        v
    }

    /// Checks one shape and its masks; returns (width, height).
    fn check_shape(
        who: &str,
        frames: &[&str],
        masks: &[&str],
        max_w: usize,
        max_h: usize,
    ) -> (usize, usize) {
        assert!(!frames.is_empty(), "{who}: no frames");
        assert!(
            masks.len() <= 1 || masks.len() == frames.len(),
            "{who}: {} masks for {} frames",
            masks.len(),
            frames.len()
        );
        let mut height = None;
        let mut width = 0;
        for (fi, f) in frames.iter().enumerate() {
            assert!(
                f.starts_with('\n'),
                "{who}: frame {fi} must start with a newline"
            );
            for ch in f.chars() {
                assert!(
                    ch == '\n' || (' '..='~').contains(&ch),
                    "{who}: frame {fi} has {ch:?}; only printable ASCII is allowed"
                );
            }
            let ls = lines(f);
            assert!(
                ls.iter().any(|l| l.trim().chars().any(|c| c != '?')),
                "{who}: frame {fi} is empty"
            );
            match height {
                None => height = Some(ls.len()),
                Some(h) => assert_eq!(h, ls.len(), "{who}: frame {fi} has a different height"),
            }
            width = width.max(ls.iter().map(|l| l.len()).max().unwrap_or(0));
            if let Some(m) = masks.get(fi % masks.len().max(1)) {
                assert!(
                    m.starts_with('\n'),
                    "{who}: mask {fi} must start with a newline"
                );
                let ms = lines(m);
                assert!(
                    ms.len() <= ls.len(),
                    "{who}: mask {fi} has {} rows, the shape {}",
                    ms.len(),
                    ls.len()
                );
                for (y, (ml, sl)) in ms.iter().zip(&ls).enumerate() {
                    for (x, mc) in ml.chars().enumerate() {
                        assert!(
                            mc == ' '
                                || mc.is_ascii_digit() && mc != '0'
                                || sprite::pen(mc).is_some(),
                            "{who}: mask {fi} row {y} has {mc:?}"
                        );
                        if mc != ' ' {
                            let sc = sl.chars().nth(x);
                            assert!(
                                sc.is_some_and(|c| c != ' ' || sl[..x].trim() != ""),
                                "{who}: mask {fi} row {y} col {x} colors nothing \
                                 (past the line end or in its leading blanks); misaligned?"
                            );
                        }
                    }
                }
            }
        }
        let h = height.unwrap_or(0);
        assert!(
            width <= max_w,
            "{who}: {width} columns wide, at most {max_w}"
        );
        assert!(h <= max_h, "{who}: {h} rows tall, at most {max_h}");
        (width, h)
    }

    fn check_common(who: &str, names: &mut HashSet<&'static str>, name: &'static str) {
        assert!(
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "{who}: names are lowercase-with-dashes"
        );
        assert!(names.insert(name), "{who}: the name is taken");
    }

    fn check_meta(who: &str, color: char, tint: &str, weight: u8, biomes: Biomes) {
        assert!(sprite::pen(color).is_some(), "{who}: color {color:?}");
        if let Err(e) = Tint::parse(tint) {
            panic!("{who}: tint: {e}");
        }
        assert!((1..=10).contains(&weight), "{who}: weight {weight}");
        assert!(biomes != 0 && biomes & !ALL == 0, "{who}: biomes");
    }

    #[test]
    fn assets_are_well_formed() {
        let mut names = HashSet::new();
        for s in species() {
            let who = format!("species {}", s.name);
            check_common(&who, &mut names, s.name);
            check_meta(&who, s.color, s.tint, s.weight, s.biomes);
            let (_, h) = check_shape(&who, s.art, s.mask, 30, 10);
            if !s.left.is_empty() {
                let (_, lh) = check_shape(&format!("{who} (left)"), s.left, s.left_mask, 30, 10);
                assert_eq!(h, lh, "{who}: left and right heights differ");
            }
            let (a, b) = s.speed;
            assert!(a > 0.0 && a <= b && b <= 4.0, "{who}: speed {a}-{b}");
            assert!((0.0..=2.0).contains(&s.fps), "{who}: fps");
            assert!(
                s.fps > 0.0 || s.art.len() == 1,
                "{who}: several frames need an fps"
            );
            assert!(
                1 <= s.school.0 && s.school.0 <= s.school.1 && s.school.1 <= 12,
                "{who}: school"
            );
            assert_eq!(
                s.zone == Zone::Bottom,
                s.swim == Swim::Crawl,
                "{who}: crawlers and only crawlers live on the bottom"
            );
        }
        for p in plants() {
            let who = format!("plant {}", p.name);
            check_common(&who, &mut names, p.name);
            check_meta(&who, p.color, p.tint, p.weight, p.biomes);
            assert!((0.0..=2.0).contains(&p.fps), "{who}: fps");
            match p.art {
                PlantArt::Sprite { frames, masks } => {
                    check_shape(&who, frames, masks, 24, 16);
                }
                PlantArt::Stalk { a, b, min, max } => {
                    assert!(
                        1 <= min && min <= max && max <= 24,
                        "{who}: height {min}-{max}"
                    );
                    for piece in [a, b] {
                        assert!(
                            !piece.is_empty()
                                && piece.len() <= 8
                                && piece.chars().all(|c| (' '..='~').contains(&c)),
                            "{who}: piece {piece:?}"
                        );
                    }
                }
            }
        }
        for d in decor() {
            let who = format!("decor {}", d.name);
            check_common(&who, &mut names, d.name);
            check_meta(&who, d.color, d.tint, d.weight, d.biomes);
            let (w, h) = check_shape(&who, d.art, d.mask, 60, 16);
            assert!((0.0..=2.0).contains(&d.fps), "{who}: fps");
            assert!(
                d.fps > 0.0 || d.art.len() == 1,
                "{who}: several frames need an fps"
            );
            if let Some((x, y)) = d.bubbles {
                assert!(
                    (0..w as i8).contains(&x) && (0..h as i8).contains(&y),
                    "{who}: bubbles"
                );
            }
        }
        for v in visitors() {
            let who = format!("visitor {}", v.name);
            check_common(&who, &mut names, v.name);
            check_meta(&who, v.color, v.tint, v.weight, v.biomes);
            let max_h = match v.lane {
                Lane::Sky => 5,
                Lane::Boat => 6,
                Lane::Float => 8,
                Lane::Deep | Lane::Floor => 16,
            };
            let (_, h) = check_shape(&who, v.art, v.mask, 80, max_h);
            if !v.left.is_empty() {
                let (_, lh) = check_shape(&format!("{who} (left)"), v.left, v.left_mask, 80, max_h);
                assert_eq!(h, lh, "{who}: left and right heights differ");
            }
            let (a, b) = v.speed;
            assert!(a > 0.0 && a <= b && b <= 6.0, "{who}: speed {a}-{b}");
            assert!(
                v.fps > 0.0 || v.art.len() == 1,
                "{who}: several frames need an fps"
            );
            assert!(
                v.biomes & DEEP == 0 || matches!(v.lane, Lane::Deep | Lane::Floor),
                "{who}: the deep sea has no surface or sky"
            );
        }
    }

    #[test]
    fn mirrored_creatures_build() {
        for s in species() {
            let (l, lm) = facing_left(s.art, s.mask, s.left, s.left_mask);
            let r = Sprite::new(s.art, s.mask, true, '?');
            let left = Sprite::new(
                &l,
                &lm.iter().map(|m| m.as_str()).collect::<Vec<_>>(),
                true,
                '?',
            );
            assert_eq!(
                (r.w, r.h),
                (left.w, left.h),
                "{}: mirror changed the size",
                s.name
            );
        }
    }

    #[test]
    fn tint_parsing() {
        let t = Tint::parse("1:yY 4:w").unwrap();
        assert_eq!(t.choices(1), &[3, 11]);
        assert_eq!(t.choices(4), &[7]);
        assert!(t.choices(2).is_empty());
        assert!(Tint::parse("1yY").is_err());
        assert!(Tint::parse("0:y").is_err());
        assert!(Tint::parse("1:q").is_err());
    }

    #[test]
    fn stalk_matches_asciiquarium_seaweed() {
        let f = stalk_frames("(", " )", 3);
        assert_eq!(f[1], "\n(\n )\n(\n");
        assert_eq!(f[0], "\n )\n(\n )\n");
    }
}
