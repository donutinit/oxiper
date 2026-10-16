//! Every sprite the aquarium draws, built once: asciiquarium's own first, at
//! the indices `mod.rs` names, then the cast from `assets`.

use std::sync::OnceLock;

use super::assets::{self, Biomes, PlantArt, Tint};
use super::sprite::{self, Sprite};
use super::{BRIGHT, G, K, SP_CASTLE, SP_SEAWEED, W};

pub struct SpeciesEntry {
    pub def: &'static assets::Species,
    /// Facing right; `spr + 1` faces left.
    pub spr: u16,
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
}

pub struct VisitorEntry {
    pub def: &'static assets::Visitor,
    /// Moving right; `spr + 1` moves left.
    pub spr: u16,
    pub tint: Tint,
    pub color: u8,
}

pub struct Library {
    pub sprites: Vec<Sprite>,
    pub species: Vec<SpeciesEntry>,
    pub plants: Vec<PlantEntry>,
    pub decor: Vec<DecorEntry>,
    pub visitors: Vec<VisitorEntry>,
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
            let spr = push(Sprite::new(d.art, d.mask, true, '?'));
            let (l, lm) = assets::facing_left(d.art, d.mask, d.left, d.left_mask);
            push(Sprite::new(&l, &strs(&lm), true, '?'));
            SpeciesEntry {
                def: d,
                spr,
                tint: tint(d.tint),
                color: pen(d.color),
            }
        })
        .collect();
    // asciiquarium's seaweed grows in most places too.
    let mut plants = vec![PlantEntry {
        name: "seaweed",
        spr: SP_SEAWEED,
        stalk: Some((3, 6)),
        tint: Tint::default(),
        color: G,
        fps: 0.27,
        weight: 6,
        biomes: assets::TANK | assets::REEF | assets::KELP | assets::OCEAN | assets::FRESH,
    }];
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
        });
    }
    // asciiquarium's castle belongs in a home aquarium.
    let mut decor = vec![DecorEntry {
        name: "castle",
        spr: SP_CASTLE,
        tint: Tint::default(),
        color: K + BRIGHT,
        fps: 0.0,
        bubbles: None,
        weight: 7,
        biomes: assets::TANK,
    }];
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
        });
    }
    let visitors = assets::visitors()
        .map(|d| {
            let spr = push(Sprite::new(d.art, d.mask, true, '?'));
            let (l, lm) = assets::facing_left(d.art, d.mask, d.left, d.left_mask);
            push(Sprite::new(&l, &strs(&lm), true, '?'));
            VisitorEntry {
                def: d,
                spr,
                tint: tint(d.tint),
                color: pen(d.color),
            }
        })
        .collect();
    Library {
        sprites,
        species,
        plants,
        decor,
        visitors,
    }
}
