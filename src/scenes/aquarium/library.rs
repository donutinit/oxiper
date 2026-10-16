//! Every sprite the aquarium draws, built once: asciiquarium's own first, at
//! the indices `mod.rs` names, then the cast from `assets` and a few effects.

use std::sync::OnceLock;

use super::assets::{self, Biomes, PlantArt, Season, Tint, Traits};
use super::sprite::{self, Sprite};
use super::{BRIGHT, G, K, SP_CASTLE, SP_SEAWEED, W};

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

/// Small effects, each a sprite whose frames are variants (or, for ink,
/// stages from thick to thin).
pub struct Fx {
    /// Fish food: `*` `,` `~`.
    pub flake: u16,
    /// Plankton: `.` `:` `'` `` ` ``.
    pub plankton: u16,
    /// What is left when something is eaten: `,` `'` `~`.
    pub scrap: u16,
    /// A cloud of ink, 5x3, thinning out over four frames.
    pub ink: u16,
    /// A trawl net hanging behind a boat that moves right (`+ 1` for left):
    /// a rope from the stern and a mesh, two frames swaying.
    pub net: u16,
    /// A fishing line: frame `n` is `n + 1` bars and a hook `J`, up to 16.
    pub line: u16,
}

pub struct Library {
    pub sprites: Vec<Sprite>,
    pub species: Vec<SpeciesEntry>,
    pub plants: Vec<PlantEntry>,
    pub decor: Vec<DecorEntry>,
    pub visitors: Vec<VisitorEntry>,
    pub fx: Fx,
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
        traits: assets::SHELTER,
        season: assets::ALWAYS,
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
            traits: p.traits,
            season: p.season,
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
        traits: assets::SHELTER,
        season: assets::ALWAYS,
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
    let mut fx = |frames: &[&str]| push(Sprite::new(frames, &[], true, '?'));
    let fx = Fx {
        flake: fx(&["\n*", "\n,", "\n~"]),
        plankton: fx(&["\n.", "\n:", "\n'", "\n`"]),
        scrap: fx(&["\n,", "\n'", "\n~"]),
        ink: fx(&[
            "\n @@@\n@@@@@\n @@@",
            "\n @%@\n%@@@%\n %@%",
            "\n % %\n%  :%\n .%",
            "\n .  \n.   :\n  .",
        ]),
        net: {
            // `?` is transparent, so what the net holds shows through.
            let frames = [
                "\n            /\n           /\n  ,-\\/\\/\\/\\/\n  (?\\/\\/\\/\\/\n  `-/\\/\\/\\/\\\n    `-\\/\\/-'",
                "\n            /\n           /\n  ,-/\\/\\/\\/\\\n  (?/\\/\\/\\/\\\n  `-\\/\\/\\/\\/\n    `-/\\/\\-'",
            ];
            let (left, _) = sprite::mirror(&frames, &[]);
            let right = fx(&frames);
            fx(&strs(&left));
            right
        },
        line: {
            let frames: Vec<String> = (1..=16)
                .map(|n| format!("\n{}J", "|\n".repeat(n)))
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
    }
}
