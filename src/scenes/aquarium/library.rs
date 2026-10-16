//! Every sprite the aquarium draws, built once: asciiquarium's own first, at
//! the indices `mod.rs` names, then the cast from `assets`, the twins that
//! stand in for asciiquarium's sprites beyond classic, and a few effects.

use std::sync::OnceLock;

use super::assets::{self, Biomes, PlantArt, Season, Tint, Traits};
use super::sprite::{self, Sprite};
use super::{SP_COUNT, SP_NONE, W};

pub struct SpeciesEntry {
    pub def: &'static assets::Species,
    /// Facing right; `spr + 1` faces left.
    pub spr: u16,
    /// The `alt` shape facing right (`+ 1` faces left), if it has one.
    pub alt: Option<u16>,
    /// How big it is: the cells it covers.
    pub size: u16,
    pub tint: Tint,
    pub color: u8,
}

pub struct PlantEntry {
    pub name: &'static str,
    /// A sprite plant's sprite, or a stalk's at height `min` (height `h` is `spr + h - min`).
    pub spr: u16,
    pub stalk: Option<(u8, u8)>,
    pub tint: Tint,
    pub color: u8,
    pub fps: f32,
    pub weight: u8,
    pub biomes: Biomes,
    pub traits: Traits,
    pub season: Season,
}

pub struct DecorEntry {
    pub name: &'static str,
    pub spr: u16,
    pub tint: Tint,
    pub color: u8,
    pub fps: f32,
    pub bubbles: Option<(i8, i8)>,
    pub weight: u8,
    pub biomes: Biomes,
    pub traits: Traits,
    pub season: Season,
}

pub struct VisitorEntry {
    pub def: &'static assets::Visitor,
    /// Moving right; `spr + 1` moves left.
    pub spr: u16,
    /// How big it is: the cells it covers.
    pub size: u16,
    pub tint: Tint,
    pub color: u8,
}

/// Small effects (drawn in `assets::fx`), each a sprite whose frames are
/// variants (or, for ink, stages from thick to thin).
pub struct Fx {
    /// Fish food.
    pub flake: u16,
    /// Plankton.
    pub plankton: u16,
    /// What is left when something is eaten.
    pub scrap: u16,
    /// A cloud of ink, thinning out frame by frame.
    pub ink: u16,
    /// A trawl net hanging behind a boat that moves right (`+ 1` for left).
    pub net: u16,
    /// A fishing line: frame `n` is `n + 1` pieces of line and a hook, up to 16.
    pub line: u16,
}

pub struct Library {
    pub sprites: Vec<Sprite>,
    pub species: Vec<SpeciesEntry>,
    pub plants: Vec<PlantEntry>,
    pub decor: Vec<DecorEntry>,
    pub visitors: Vec<VisitorEntry>,
    pub fx: Fx,
    /// For each of asciiquarium's sprites, the twin that stands in for it
    /// beyond classic, or `SP_NONE`.
    pub twin: Vec<u16>,
    /// The rows of the water's surface beyond classic.
    pub water: [Vec<char>; 4],
}

#[cfg(test)]
impl Library {
    pub fn species_named(&self, name: &str) -> Option<usize> {
        self.species.iter().position(|s| s.def.name == name)
    }
    pub fn visitor_named(&self, name: &str) -> Option<usize> {
        self.visitors.iter().position(|v| v.def.name == name)
    }
    pub fn plant_named(&self, name: &str) -> Option<usize> {
        self.plants.iter().position(|p| p.name == name)
    }
    pub fn decor_named(&self, name: &str) -> Option<usize> {
        self.decor.iter().position(|d| d.name == name)
    }
}

pub fn library() -> &'static Library {
    static LIB: OnceLock<Library> = OnceLock::new();
    LIB.get_or_init(build)
}

fn pen(c: char) -> u8 {
    sprite::pen(c).unwrap_or(W)
}

fn tint(t: &str) -> Tint {
    Tint::parse(t).unwrap_or_default()
}

fn strs(v: &[String]) -> Vec<&str> {
    v.iter().map(String::as_str).collect()
}

fn build() -> Library {
    let mut sprites = super::build_sprites();
    let mut push = |s: Sprite| {
        sprites.push(s);
        (sprites.len() - 1) as u16
    };
    let species = assets::species()
        .map(|d| {
            let right = Sprite::new(d.art, d.mask, true, '?');
            let size = right.frame(0).len().min(u16::MAX as usize) as u16;
            let spr = push(right);
            let (l, lm) = assets::facing_left(d.art, d.mask, d.left, d.left_mask);
            push(Sprite::new(&l, &strs(&lm), true, '?'));
            let alt = (!d.alt.is_empty()).then(|| {
                let a = push(Sprite::new(d.alt, d.alt_mask, true, '?'));
                let (l, lm) = sprite::mirror(d.alt, d.alt_mask);
                push(Sprite::new(&l, &strs(&lm), true, '?'));
                a
            });
            SpeciesEntry {
                def: d,
                spr,
                alt,
                size,
                tint: tint(d.tint),
                color: pen(d.color),
            }
        })
        .collect();
    let mut plants = Vec::new();
    for p in assets::plants() {
        let (spr, stalk) = match p.art {
            PlantArt::Sprite { frames, masks } => {
                (push(Sprite::new(frames, masks, true, '?')), None)
            }
            PlantArt::Stalk { a, b, min, max } => {
                let first = (min..=max)
                    .map(|h| {
                        push(Sprite::new(
                            &assets::stalk_frames(a, b, h as usize),
                            &[],
                            true,
                            '?',
                        ))
                    })
                    .min()
                    .unwrap_or(0);
                (first, Some((min, max)))
            }
        };
        plants.push(PlantEntry {
            name: p.name,
            spr,
            stalk,
            tint: tint(p.tint),
            color: pen(p.color),
            fps: p.fps,
            weight: p.weight,
            biomes: p.biomes,
            traits: p.traits,
            season: p.season,
        });
    }
    let mut decor = Vec::new();
    for d in assets::decor() {
        decor.push(DecorEntry {
            name: d.name,
            spr: push(Sprite::new(d.art, d.mask, true, '?')),
            tint: tint(d.tint),
            color: pen(d.color),
            fps: d.fps,
            bubbles: d.bubbles,
            weight: d.weight,
            biomes: d.biomes,
            traits: d.traits,
            season: d.season,
        });
    }
    let visitors = assets::visitors()
        .map(|d| {
            let right = Sprite::new(d.art, d.mask, true, '?');
            let size = right.frame(0).len().min(u16::MAX as usize) as u16;
            let spr = push(right);
            let (l, lm) = assets::facing_left(d.art, d.mask, d.left, d.left_mask);
            push(Sprite::new(&l, &strs(&lm), true, '?'));
            VisitorEntry {
                def: d,
                spr,
                size,
                tint: tint(d.tint),
                color: pen(d.color),
            }
        })
        .collect();
    let mut twin = vec![SP_NONE; SP_COUNT];
    for &(id, frames, masks) in assets::twins::SPRITES {
        let Some(slot) = twin.get_mut(id as usize) else {
            continue;
        };
        *slot = push(Sprite::new(frames, masks, true, '?'));
        let pair = assets::twins::WANTED
            .iter()
            .any(|&(w, pair)| w == id && pair);
        if pair && (id as usize + 1) < SP_COUNT {
            let (l, lm) = sprite::mirror(frames, masks);
            twin[id as usize + 1] = push(Sprite::new(&l, &strs(&lm), true, '?'));
        }
    }
    let water = assets::twins::WATER.map(|row| row.chars().collect());
    let fx_art = &assets::fx::NET;
    let mut fx = |frames: &[&str]| push(Sprite::new(frames, &[], true, '?'));
    let fx = Fx {
        flake: fx(assets::fx::FLAKE),
        plankton: fx(assets::fx::PLANKTON),
        scrap: fx(assets::fx::SCRAP),
        ink: fx(assets::fx::INK),
        net: {
            let (left, _) = sprite::mirror(fx_art, &[]);
            let right = fx(fx_art);
            fx(&strs(&left));
            right
        },
        line: {
            let (bar, hook) = (assets::fx::LINE_BAR, assets::fx::LINE_HOOK);
            let frames: Vec<String> = (1..=16)
                .map(|n| format!("\n{}{hook}", format!("{bar}\n").repeat(n)))
                .collect();
            fx(&strs(&frames))
        },
    };
    Library {
        sprites,
        species,
        plants,
        decor,
        visitors,
        fx,
        twin,
        water,
    }
}
