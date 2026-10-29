//! fireworks: rockets burst into colorful sparks over a city skyline.
//!
//! Adapted from tanim's `fireworks`
//! <https://github.com/SantiagoSchez/tanim>, licensed MIT. The shell patterns
//! (sphere, ring, willow, crackle, multicolor, palm), the physics, the glow a
//! burst casts on the rooftops, the lit windows, the blinking beacons and the
//! calm-then-finale launch schedule follow tanim; the night sky is transparent
//! here by default (`sky = true` paints tanim's gradient) and the colors come
//! from the theme.
//!
//! On Día de Muertos the city becomes a panteón: painted graves, crosses,
//! tombs, marigold arches and a whitewashed chapel, the graves covered in
//! cempasúchil, veladoras flickering on them and warming the stone, copal
//! smoke curling up, and far candles on the hill behind. The shells burst
//! in cempasúchil and rosa mexicano, some as sugar-skull calaveras, as
//! marigolds on their stems or as showers of tumbling petals, and a finale
//! ends with a calavera between two marigolds. The further the holiday has
//! come, the more graves are dressed, candles lit and shells shaped.
//!
//! On Navidad the city becomes a town plaza: a lit kiosco (bandstand) with
//! pines either side, nochebuenas in their pots, farolitos along the ground
//! and strings of foquitos (series lights) draped from the rooftops to the
//! kiosco, blinking in chase patterns. The shells burst in rojo nochebuena,
//! verde pino, dorado and plata, some as estrellas de siete picos (the star
//! of a piñata), flores de nochebuena or bells, and a finale ends with a
//! star between two bells. The posadas bring more lights, strings and
//! shapes, day by day. On Año Nuevo the show counts down from 10 in big
//! digits, then the finale writes "FELIZ AÑO NUEVO" in sparks.
//!
//! MIT License
//!
//! Copyright (c) 2026 Santiago Sánchez
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use crate::color::{Color, ansi_rgb, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "fireworks",
    aliases: &["rockets"],
    about: "rockets bursting into colorful sparks over a city, after tanim",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast everything moves (0.1-4)",
        },
        Opt {
            key: "rate",
            default: Val::F(1.0),
            help: "How often rockets launch, 0 = never (0-5)",
        },
        Opt {
            key: "skyline",
            default: Val::B(true),
            help: "Draw the city with its lit windows",
        },
        Opt {
            key: "stars",
            default: Val::B(true),
            help: "Faint stars in the night sky",
        },
        Opt {
            key: "sky",
            default: Val::B(false),
            help: "Paint the night sky instead of leaving it transparent",
        },
    ],
    make: |ctx| Box::new(Fireworks::new(ctx)),
};

/// Color as red, green, blue in 0..1 (more than 1 is allowed while adding light).
type V3 = [f32; 3];

const SKY_TOP: V3 = [1.0 / 255.0, 1.0 / 255.0, 8.0 / 255.0];
const SKY_LOW: V3 = [12.0 / 255.0, 8.0 / 255.0, 28.0 / 255.0];
const BUILDING: V3 = [5.0 / 255.0, 5.0 / 255.0, 11.0 / 255.0];
const STAR: V3 = [190.0 / 255.0, 200.0 / 255.0, 240.0 / 255.0];
const BEACON: V3 = [200.0 / 255.0, 30.0 / 255.0, 20.0 / 255.0];
const EMBER: V3 = [110.0 / 255.0, 25.0 / 255.0, 10.0 / 255.0];
const GOLD: V3 = [1.0, 190.0 / 255.0, 90.0 / 255.0];
const SHELL_WHITE: V3 = [235.0 / 255.0, 235.0 / 255.0, 1.0];
const ROCKET_FIRE: V3 = [1.0, 220.0 / 255.0, 160.0 / 255.0];
const TRAIL_FIRE: V3 = [1.0, 170.0 / 255.0, 80.0 / 255.0];
const ROCKET_G: f32 = 0.03;
const SPARK_G: f32 = 0.011;
const MAX_PARTICLES: usize = 7000;
/// tanim's tick rate; one simulation step is this long.
const STEP: f32 = 1.0 / 30.0;

const TRAIL: u8 = 1;
const CRACKLE: u8 = 2;
const STROBE: u8 = 4;
const ROCKET: u8 = 8;
/// A marigold petal: it tumbles (two shades) and sways as it falls.
const PETAL: u8 = 16;
/// A wisp of copal smoke, rising and swaying.
const SMOKE: u8 = 32;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Shell {
    Sphere,
    Ring,
    Willow,
    Crackle,
    Multi,
    Palm,
    /// Día de Muertos: a sugar skull drawn in sparks.
    Calavera,
    /// Día de Muertos: a cempasúchil blossom on its stem.
    Marigold,
    /// Día de Muertos: a shower of marigold petals.
    Petals,
    /// Navidad: an estrella de siete picos, the star of a piñata.
    Estrella,
    /// Navidad: a flor de nochebuena (poinsettia).
    Nochebuena,
    /// Navidad: a bell.
    Campana,
    /// A phrase written in sparks: its number in `TEXTS`.
    Text(u8),
}

impl Shell {
    /// Is it drawn in sparks that hold their shape, rather than thrown?
    fn shaped(self) -> bool {
        matches!(
            self,
            Shell::Calavera
                | Shell::Marigold
                | Shell::Estrella
                | Shell::Nochebuena
                | Shell::Campana
                | Shell::Text(_)
        )
    }
}

#[derive(Clone, Copy)]
struct P {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max: f32,
    color: V3,
    drag: f32,
    grav: f32,
    flags: u8,
    shell: Shell,
}

/// The fading glow a burst casts on the rooftops (and the sky when painted).
struct Flash {
    x: f32,
    y: f32,
    radius: f32,
    color: V3,
    power: f32,
}

/// The colors the scene draws with, from the theme.
struct Palette {
    native: bool,
    /// Burst colors besides gold and white (never empty).
    accents: Vec<V3>,
    gold: V3,
    white: V3,
    sky_top: V3,
    sky_low: V3,
    building: V3,
    star: V3,
    windows: [V3; 3],
}

impl Palette {
    /// A shell color, tanim's mix: gold, white, or a random hue.
    fn shell_color(&self, rng: &mut fastrand::Rng) -> V3 {
        match rng.u32(0..8) {
            0 => self.gold,
            1 => self.white,
            _ if self.native => hsv(rng.f32(), 0.55 + 0.35 * rng.f32(), 1.0),
            _ => self.accents[rng.usize(0..self.accents.len())],
        }
    }
}

fn v3(c: Color) -> V3 {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => ansi_rgb(n),
        Color::Reset => (255, 255, 255),
    };
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}

fn mix(a: V3, b: V3, t: f32) -> V3 {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn scale(a: V3, k: f32) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}

/// HSV to RGB, `h` wrapping around 0..1.
fn hsv(h: f32, s: f32, v: f32) -> V3 {
    let h = (h - h.floor()) * 6.0;
    let i = h as i32;
    let f = h - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    match i {
        0 => [v, t, p],
        1 => [q, v, p],
        2 => [p, v, t],
        3 => [p, q, v],
        4 => [t, p, v],
        _ => [v, p, q],
    }
}

fn to_color(c: V3) -> Color {
    let b = |x: f32| (x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    Color::Rgb(b(c[0]), b(c[1]), b(c[2]))
}

/// A numeric option clamped to its range; non-finite values get the default.
fn num(v: f32, default: f32, lo: f32, hi: f32) -> f32 {
    if v.is_finite() {
        v.clamp(lo, hi)
    } else {
        default
    }
}

fn rangef(rng: &mut fastrand::Rng, a: f32, b: f32) -> f32 {
    a + (b - a) * rng.f32()
}

const fn c8(r: u8, g: u8, b: u8) -> V3 {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}

// Día de Muertos under `classic`; the shell colors are brightened to burn
// in the night sky.
const CEMPASUCHIL: V3 = c8(255, 140, 0);
const CEMPA_LIGHT: V3 = c8(255, 178, 36);
const CEMPA_DEEP: V3 = c8(228, 92, 0);
const CEMPA_HEART: V3 = c8(255, 212, 60);
const ROSA: V3 = c8(255, 36, 150);
const TERCIOPELO: V3 = c8(196, 0, 82);
const MORADO: V3 = c8(160, 80, 230);
const TURQUESA: V3 = c8(30, 205, 215);
const AMARILLO: V3 = c8(255, 214, 20);
const LIMON: V3 = c8(140, 200, 40);
const HUESO: V3 = c8(244, 238, 226);
const FLAMA: V3 = c8(255, 214, 120);
const FLAMA_EDGE: V3 = c8(255, 150, 40);

/// A shaped shell's radius, as a share of the screen's smaller side.
const SHAPE_SIZE: (f32, f32) = (0.2, 0.26);

/// Seeds the holiday's own random numbers, so the ordinary ones never change.
const MUERTOS_SEED: u64 = 0x6d75_6572_746f_7321;

/// What Día de Muertos is drawn with: theme colors, except for the
/// marigolds and the candle flames, which keep their own.
struct Dress {
    /// Shell colors, repeated by weight (never empty).
    shells: Vec<V3>,
    gold: V3,
    /// The sugar skull and its icing (eyes, nose and forehead, eye hearts).
    bone: V3,
    icing: [V3; 3],
    /// Marigold shades: deep, mid, light and the heart; then the stem.
    mari: [V3; 4],
    leaf: V3,
    terciopelo: V3,
    nube: V3,
    stone: V3,
    /// Painted crosses and stone faces caught by the moon.
    face: V3,
    earth: V3,
    cypress: V3,
    window: V3,
    smoke: V3,
    /// The colors graves are painted in, seen by night.
    paint: [V3; 5],
    /// Veladora glass.
    glass: [V3; 3],
}

impl Dress {
    fn new(ctx: &Ctx, pal: &Palette) -> Dress {
        let t = ctx.theme;
        let pick = |i: usize, native: V3| v3(ctx.pick(i, to_color(native)));
        let mari = [CEMPA_DEEP, CEMPASUCHIL, CEMPA_LIGHT, CEMPA_HEART];
        if t.native {
            let stone = scale(pal.building, 1.25);
            Dress {
                // Mostly cempasúchil and rosa mexicano.
                shells: vec![
                    CEMPASUCHIL,
                    CEMPASUCHIL,
                    CEMPA_LIGHT,
                    ROSA,
                    ROSA,
                    ROSA,
                    AMARILLO,
                    MORADO,
                    TURQUESA,
                    LIMON,
                    HUESO,
                ],
                gold: CEMPA_LIGHT,
                bone: HUESO,
                icing: [ROSA, TURQUESA, AMARILLO],
                mari,
                leaf: LIMON,
                terciopelo: TERCIOPELO,
                nube: HUESO,
                stone,
                face: mix(stone, HUESO, 0.3),
                earth: mix(stone, c8(40, 22, 16), 0.5),
                cypress: c8(20, 52, 44),
                window: scale(FLAMA_EDGE, 0.75),
                smoke: c8(150, 150, 175),
                paint: [TURQUESA, ROSA, c8(60, 80, 220), HUESO, c8(70, 180, 100)],
                glass: [TERCIOPELO, ROSA, HUESO],
            }
        } else {
            let g = &t.grad;
            let stone = pal.building;
            let white = v3(g.at(1.0));
            let shells: Vec<V3> = t.colors.iter().map(|&c| v3(c)).collect();
            Dress {
                shells: if shells.is_empty() {
                    vec![pal.gold]
                } else {
                    shells
                },
                gold: pal.gold,
                bone: white,
                icing: [pick(0, ROSA), pick(1, TURQUESA), pick(2, AMARILLO)],
                mari,
                leaf: LIMON,
                terciopelo: pick(1, TERCIOPELO),
                nube: white,
                stone,
                face: mix(stone, white, 0.22),
                earth: scale(stone, 0.8),
                cypress: mix(stone, v3(g.at(0.2)), 0.4),
                window: scale(FLAMA_EDGE, 0.75),
                smoke: mix(stone, white, 0.5),
                paint: [
                    pick(0, TURQUESA),
                    pick(1, ROSA),
                    pick(2, HUESO),
                    pick(3, HUESO),
                    white,
                ],
                glass: [pick(0, TERCIOPELO), pick(1, ROSA), white],
            }
        }
    }
}

/// Seeds Navidad's own random numbers.
const NAVIDAD_SEED: u64 = 0x6e61_7669_6461_6421;

// Navidad under `classic`; the shell colors are brightened to burn in the
// night sky.
const SH_ROJO: V3 = c8(255, 48, 72);
const SH_VERDE: V3 = c8(52, 230, 110);
const SH_DORADO: V3 = c8(255, 204, 72);
const SH_PLATA: V3 = c8(214, 220, 240);
const SH_BLANCO: V3 = c8(244, 241, 234);
const ROJO_NOCHEBUENA: V3 = c8(200, 16, 46);
const VERDE_PINO: V3 = c8(31, 111, 58);
/// Series lights (foquitos): rojo, verde, azul, amarillo, rosa, naranja.
const FOQUITOS: [V3; 6] = [
    c8(255, 59, 59),
    c8(59, 209, 111),
    c8(59, 123, 255),
    c8(255, 210, 59),
    c8(255, 95, 180),
    c8(255, 140, 26),
];

/// What the new holidays are drawn with: the holiday's own colors under
/// `classic`, the theme's accents under any other theme.
struct Fest {
    /// Shell colors, repeated by weight (never empty).
    shells: Vec<V3>,
    gold: V3,
    white: V3,
    red: V3,
    green: V3,
    /// The colors a shaped shell's strokes pick from: rojo, verde, dorado,
    /// plata, blanco and a deeper rojo.
    shape: [V3; 8],
    /// The series lights, in the order they come on the string.
    hues: [V3; 6],
    stone: V3,
    /// Plaster of the facades by night.
    wall: [V3; 3],
    ground: V3,
    /// A lit window.
    window: V3,
    roof: V3,
    pine: V3,
    wood: V3,
    /// The wire the lights hang from.
    cable: V3,
}

impl Fest {
    fn new(ctx: &Ctx, pal: &Palette) -> Fest {
        let t = ctx.theme;
        if t.native {
            let stone = scale(pal.building, 1.25);
            Fest {
                // Mostly rojo and verde, with dorado and plata between.
                shells: vec![
                    SH_ROJO, SH_ROJO, SH_VERDE, SH_VERDE, SH_DORADO, SH_DORADO, SH_PLATA, SH_BLANCO,
                ],
                gold: SH_DORADO,
                white: SH_BLANCO,
                red: SH_ROJO,
                green: SH_VERDE,
                shape: [
                    SH_ROJO,
                    SH_VERDE,
                    SH_DORADO,
                    SH_PLATA,
                    SH_BLANCO,
                    scale(SH_ROJO, 0.7),
                    SH_DORADO,
                    SH_DORADO,
                ],
                hues: FOQUITOS,
                stone,
                wall: [
                    mix(stone, c8(230, 200, 140), 0.3),
                    mix(stone, c8(220, 110, 120), 0.25),
                    mix(stone, c8(90, 140, 200), 0.25),
                ],
                ground: mix(stone, c8(110, 90, 80), 0.4),
                window: scale(FLAMA, 0.7),
                roof: scale(ROJO_NOCHEBUENA, 0.6),
                pine: scale(VERDE_PINO, 0.8),
                wood: c8(70, 40, 20),
                cable: c8(30, 80, 45),
            }
        } else {
            let colors: Vec<V3> = t.colors.iter().map(|&c| v3(c)).collect();
            let hue = |i: usize| {
                if colors.is_empty() {
                    pal.gold
                } else {
                    colors[i % colors.len()]
                }
            };
            let stone = pal.building;
            let (red, green) = (hue(0), hue(1));
            let silver = mix(pal.white, stone, 0.2);
            Fest {
                shells: if colors.is_empty() {
                    vec![pal.gold]
                } else {
                    colors.clone()
                },
                gold: pal.gold,
                white: pal.white,
                red,
                green,
                shape: [
                    red,
                    green,
                    pal.gold,
                    silver,
                    pal.white,
                    scale(red, 0.7),
                    pal.gold,
                    pal.gold,
                ],
                hues: [hue(0), hue(1), hue(2), hue(3), hue(4), hue(5)],
                stone,
                wall: [
                    mix(stone, hue(2), 0.22),
                    mix(stone, hue(0), 0.22),
                    mix(stone, hue(3), 0.22),
                ],
                ground: mix(stone, pal.white, 0.12),
                window: scale(pal.gold, 0.7),
                roof: scale(red, 0.55),
                pine: scale(green, 0.55),
                wood: mix(stone, pal.gold, 0.3),
                cable: scale(green, 0.5),
            }
        }
    }
}

/// A stroke of a shaped shell, in units of its radius (y grows downward),
/// with the index of its color.
enum Stroke {
    Line(f32, f32, f32, f32, u8),
    /// Center, radius, and the arc from one angle to another (in turns,
    /// counterclockwise from the right).
    Arc(f32, f32, f32, f32, f32, u8),
    /// A scalloped ring: center, radius, how deep the scallops are and how
    /// many there are.
    Bloom(f32, f32, f32, f32, f32, u8),
    /// A regular star: center, outer and inner radius, how many points,
    /// the color of the first, and how many colors the points go through
    /// (an odd point left over takes the next color).
    Star(f32, f32, f32, f32, u8, u8, u8),
    /// Pointed leaves around a center: center, length, half the width (as
    /// a share of the length), how many, the turn of the first (0 points
    /// up) and the color.
    Leaf(f32, f32, f32, f32, u8, f32, u8),
}

/// A calavera de azúcar: the skull in bone, flower eyes and a heart nose in
/// icing, a stitched smile and a little flower on the forehead.
const CALAVERA: &[Stroke] = &[
    Stroke::Arc(0.0, -0.2, 0.85, -0.06, 0.56, 0),
    Stroke::Line(0.79, 0.11, 0.6, 0.42, 0),
    Stroke::Line(-0.79, 0.11, -0.6, 0.42, 0),
    Stroke::Line(0.6, 0.42, 0.5, 0.8, 0),
    Stroke::Line(-0.6, 0.42, -0.5, 0.8, 0),
    Stroke::Line(0.5, 0.8, 0.26, 0.96, 0),
    Stroke::Line(-0.5, 0.8, -0.26, 0.96, 0),
    Stroke::Line(-0.26, 0.96, 0.26, 0.96, 0),
    Stroke::Bloom(0.33, 0.07, 0.21, 0.3, 6.0, 1),
    Stroke::Bloom(-0.33, 0.07, 0.21, 0.3, 6.0, 1),
    Stroke::Arc(0.33, 0.07, 0.05, 0.0, 1.0, 3),
    Stroke::Arc(-0.33, 0.07, 0.05, 0.0, 1.0, 3),
    Stroke::Line(0.0, 0.46, -0.1, 0.33, 2),
    Stroke::Line(0.0, 0.46, 0.1, 0.33, 2),
    Stroke::Line(-0.1, 0.33, 0.0, 0.36, 2),
    Stroke::Line(0.1, 0.33, 0.0, 0.36, 2),
    Stroke::Line(-0.32, 0.67, 0.32, 0.67, 0),
    Stroke::Line(-0.21, 0.59, -0.21, 0.75, 0),
    Stroke::Line(-0.07, 0.59, -0.07, 0.75, 0),
    Stroke::Line(0.07, 0.59, 0.07, 0.75, 0),
    Stroke::Line(0.21, 0.59, 0.21, 0.75, 0),
    Stroke::Bloom(0.0, -0.5, 0.13, 0.4, 5.0, 2),
];

/// A cempasúchil: ruffled rings of petals around a golden heart, on a stem
/// with a leaf.
const MARIGOLD: &[Stroke] = &[
    Stroke::Bloom(0.0, 0.0, 0.95, 0.2, 15.0, 0),
    Stroke::Bloom(0.0, 0.0, 0.72, 0.25, 12.0, 1),
    Stroke::Bloom(0.0, 0.0, 0.48, 0.3, 9.0, 2),
    Stroke::Bloom(0.0, 0.0, 0.22, 0.3, 5.0, 3),
    Stroke::Line(0.0, 0.98, 0.06, 1.75, 4),
    Stroke::Line(0.03, 1.4, -0.32, 1.18, 4),
    Stroke::Line(-0.32, 1.18, -0.06, 1.28, 4),
];

/// An estrella de siete picos, each point a color of its own, a ring of
/// white at its heart.
const ESTRELLA: &[Stroke] = &[
    Stroke::Star(0.0, 0.05, 1.0, 0.46, 7, 0, 3),
    Stroke::Arc(0.0, 0.05, 0.16, 0.0, 1.0, 4),
];

/// A flor de nochebuena: green leaves under two rings of red bracts around
/// a yellow heart.
const NOCHEBUENA: &[Stroke] = &[
    Stroke::Leaf(0.0, 0.0, 1.0, 0.14, 5, 0.1, 1),
    Stroke::Leaf(0.0, 0.0, 0.9, 0.17, 7, 0.0, 0),
    Stroke::Leaf(0.0, 0.0, 0.56, 0.2, 7, 0.07, 5),
    Stroke::Bloom(0.0, 0.0, 0.15, 0.3, 5.0, 2),
    Stroke::Arc(0.0, 0.0, 0.05, 0.0, 1.0, 2),
];

/// A bell: its loop, dome, flared lip and clapper.
const CAMPANA: &[Stroke] = &[
    Stroke::Arc(0.0, -0.8, 0.13, 0.0, 1.0, 3),
    Stroke::Arc(0.0, -0.15, 0.58, 0.0, 0.5, 2),
    Stroke::Line(0.58, -0.15, 0.62, 0.32, 2),
    Stroke::Line(-0.58, -0.15, -0.62, 0.32, 2),
    Stroke::Line(0.62, 0.32, 0.88, 0.62, 2),
    Stroke::Line(-0.62, 0.32, -0.88, 0.62, 2),
    Stroke::Line(-0.88, 0.62, 0.88, 0.62, 3),
    Stroke::Line(-0.9, 0.72, 0.9, 0.72, 3),
    Stroke::Line(0.88, 0.62, 0.9, 0.72, 3),
    Stroke::Line(-0.88, 0.62, -0.9, 0.72, 3),
    Stroke::Arc(0.0, -0.15, 0.4, 0.12, 0.3, 4),
    Stroke::Line(-0.6, 0.12, 0.6, 0.12, 0),
    Stroke::Arc(0.0, 0.92, 0.12, 0.0, 1.0, 3),
];

/// Length of one edge of a star, from an outer point to the next inner one.
fn star_edge(ro: f32, ri: f32, points: u8) -> f32 {
    let a = PI / points.max(1) as f32;
    (ro - ri * a.cos()).hypot(ri * a.sin())
}

/// The point `u` (0 to 1) of the way around a star's outline (it points
/// up), and which of `cycle` colors its point takes (0 without a cycle).
fn star_point(ro: f32, ri: f32, points: u8, cycle: u8, u: f32) -> (f32, f32, u8) {
    let p = points.max(2) as usize;
    let edges = p * 2;
    let e = u * edges as f32;
    let i = (e as usize).min(edges - 1);
    let t = e - i as f32;
    let vertex = |v: usize| {
        let a = -FRAC_PI_2 + (v % edges) as f32 * PI / p as f32;
        let r = if v.is_multiple_of(2) { ro } else { ri };
        (r * a.cos(), r * a.sin())
    };
    let ((x0, y0), (x1, y1)) = (vertex(i), vertex(i + 1));
    // Each edge belongs to the point it climbs to or falls from.
    let point = i.div_ceil(2) % p;
    let c = match cycle as usize {
        0 => 0,
        c if point + 1 == p && !p.is_multiple_of(c) => c as u8,
        c => (point % c) as u8,
    };
    (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t, c)
}

/// The point `u` of the way around the outlines of `count` leaves in turn,
/// each a pointed oval from near the center out, widest past its middle.
fn leaf_point(len: f32, half: f32, count: u8, turn: f32, u: f32) -> (f32, f32) {
    let n = count.max(1) as f32;
    let e = u * n;
    let k = e.floor();
    let t = e - k;
    let (d, side) = if t < 0.5 {
        (t * 2.0, 1.0)
    } else {
        ((1.0 - t) * 2.0, -1.0)
    };
    let along = len * (0.1 + 0.9 * d);
    let wide = side * half * len * (PI * d.powf(1.4)).sin().max(0.0);
    let (sa, ca) = ((turn + k / n) * TAU).sin_cos();
    (sa * along + ca * wide, -ca * along + sa * wide)
}

/// A letter or digit of the font the sparks write with: five rows of five
/// columns (a bit each, the leftmost the highest), and the accent drawn in
/// a row above them, if any. Anything else is blank.
fn glyph(c: char) -> ([u8; 5], u8) {
    let rows = match c {
        'A' | 'Á' => [0b01110, 0b10001, 0b11111, 0b10001, 0b10001],
        'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b01111],
        'E' | 'É' => [0b11111, 0b10000, 0b11110, 0b10000, 0b11111],
        'F' => [0b11111, 0b10000, 0b11110, 0b10000, 0b10000],
        'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b11111],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10001, 0b10001],
        'N' | 'Ñ' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b01110],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => [0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'X' => [0b10001, 0b01010, 0b00100, 0b01010, 0b10001],
        'Z' => [0b11111, 0b00010, 0b00100, 0b01000, 0b11111],
        '0' => [0b01110, 0b10011, 0b10101, 0b11001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00110, 0b01000, 0b11111],
        '3' => [0b11110, 0b00001, 0b01110, 0b00001, 0b11110],
        '4' => [0b00110, 0b01010, 0b10010, 0b11111, 0b00010],
        '5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b11110],
        '6' => [0b01110, 0b10000, 0b11110, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b00100],
        '8' => [0b01110, 0b10001, 0b01110, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b01111, 0b00001, 0b01110],
        _ => [0; 5],
    };
    let accent = match c {
        'Ñ' => 0b01101,
        'Á' | 'É' => 0b00010,
        _ => 0,
    };
    (rows, accent)
}

/// How many columns a line of text takes: letters are five wide with a gap
/// of one, a space is three wide.
fn text_cols(s: &str) -> i32 {
    let n: i32 = s.chars().map(|c| if c == ' ' { 4 } else { 6 }).sum();
    (n - 1).max(1)
}

/// Rows of a font line (the accent's and the letter's) and the gap between
/// two lines.
const LINE_ROWS: i32 = 6;
const LINE_GAP: i32 = 2;

/// A phrase laid out on the screen: its lines, and how big one font pixel is.
struct TextFit {
    lines: [&'static str; 2],
    count: usize,
    cols: i32,
    cell: f32,
}

impl TextFit {
    fn rows(&self) -> i32 {
        LINE_ROWS * self.count as i32 + LINE_GAP * (self.count as i32 - 1)
    }
}

/// Lays `text` out on a screen of `w` by `ph` pixels, on one line or on two
/// (split at its first space), whichever comes out bigger; `None` when it
/// would be too small to read.
fn fit_text(text: &'static str, w: f32, ph: f32) -> Option<TextFit> {
    let one = TextFit {
        lines: [text, ""],
        count: 1,
        cols: 0,
        cell: 0.0,
    };
    let two = text.split_once(' ').map(|(a, b)| TextFit {
        lines: [a, b],
        count: 2,
        cols: 0,
        cell: 0.0,
    });
    let size = |t: TextFit| {
        let cols = t.lines[..t.count]
            .iter()
            .map(|l| text_cols(l))
            .max()
            .unwrap_or(1);
        let cell = (w * 0.9 / cols as f32).min(ph * 0.42 / t.rows() as f32);
        TextFit { cols, cell, ..t }
    };
    let best = [Some(one), two]
        .into_iter()
        .flatten()
        .map(size)
        .max_by(|a, b| a.cell.total_cmp(&b.cell))?;
    (best.cell >= 0.9).then_some(best)
}

/// The panteón is drawn from these, one char per pixel, standing on the
/// ground. `#` stone, `+` a painted cross or a stone face, `%` cypress, `=`
/// earth; `o O x` marigolds, `t` terciopelo and `n` nube cover a grave once
/// it is dressed (earth until then); `a A b` are a marigold arch (nothing
/// until then); `f` a veladora's flame over its glass `v`, there once lit;
/// `w` a lit doorway.
const CROSS_MOUND: &[&str] = &[
    "    +    ", //
    "   +++   ",
    "    +    ",
    "    +    ",
    "f xoOox f",
    "voOxtOxov",
];
const HEADSTONE: &[&str] = &[
    "  ###  ", //
    " ##### ", " ##+## ", " #+++# ", " ##+## ", "f##+##f", "v#####v", "xoOnOox",
];
const ANGELITO: &[&str] = &[
    " + ", //
    "+++", " + ", "nnn",
];
const SMALL_CROSS: &[&str] = &[
    "  +  ", //
    " +++ ", "  +  ", "f +  ", "vxoO ",
];
const TALL_CROSS: &[&str] = &[
    "   +   ", //
    "   +   ", " +++++ ", " a + a ", "  bAb  ", "   +   ", "   +   ", "  ###  ", "f #+# f",
    "v#####v", "xoOtOox",
];
const TOMB: &[&str] = &[
    "      +      ", //
    "     +++     ",
    "      +      ",
    "   xoO+Oox   ",
    " ########### ",
    " #+++++++++# ",
    " #+#######+# ",
    "f#+++++++++#f",
    "v###########v",
];
const ARCO: &[&str] = &[
    "  aAbAa  ", //
    " a     a ",
    "b       b",
    "A   +   A",
    "a  +++  a",
    "b   +   b",
    "A f + f A",
    "a v + v a",
    "oxoOtOoxo",
];
const MAUSOLEUM: &[&str] = &[
    "      +      ", //
    "     +++     ",
    "      +      ",
    "     ###     ",
    "    #####    ",
    "   #######   ",
    "  #########  ",
    " ########### ",
    "  #+#####+#  ",
    "  #+#www#+#  ",
    "  #+wwwww+#  ",
    "f #+wwwww+# f",
    "v #+wwwww+# v",
    "xo#########ox",
];
const CYPRESS: &[&str] = &[
    "  %  ", //
    "  %  ", " %%% ", " %%% ", " %%% ", "%%%%%", "%%%%%", "%%%%%", " %%% ", "  #  ",
];
const CHAPEL: &[&str] = &[
    "    +                 ", //
    "   +++                ",
    "    +                 ",
    "   ###                ",
    "  #####               ",
    "  #w#w#               ",
    "  #####       +       ",
    "  #####      +++      ",
    "  #####       +       ",
    "  #####      ###      ",
    "  #####    #######    ",
    "  #####  ###########  ",
    "  ##### ############# ",
    "  #####+++++++++++++# ",
    "  #####+###########+# ",
    "  ##w##+####www####+# ",
    "  ##w##+###wwwww###+# ",
    "  #####+###wwwww###+# ",
    " f#####+###wwwww###+#f",
    " v#####+###wwwww###+#v",
];

/// Far-off graves on the hill.
const FAR_CROSS: &[&str] = &[" + ", "+++", " + "];
const FAR_STONE: &[&str] = &[" # ", "###"];

/// Panteón pieces and how often each comes.
const GRAVES: &[(&[&str], u32)] = &[
    (CROSS_MOUND, 5),
    (HEADSTONE, 4),
    (ANGELITO, 2),
    (SMALL_CROSS, 3),
    (TALL_CROSS, 2),
    (TOMB, 2),
    (ARCO, 3),
    (MAUSOLEUM, 1),
    (CYPRESS, 2),
];

/// The kiosco of the plaza on Navidad, one char per pixel: `r` its roof,
/// `+` the trim, `|` an iron column, `#` its stone base, `o` a bulb on the
/// eaves, `w` a lamp inside and `*` the star on its finial.
const KIOSCO: &[&str] = &[
    "         *         ", //
    "        rrr        ",
    "      rrrrrrr      ",
    "    rrrrrrrrrrr    ",
    "  rrrrrrrrrrrrrrr  ",
    " +o+o+o+o+o+o+o+o+ ",
    " | w | w | w | w | ",
    " |   |   |   |   | ",
    " |   |   |   |   | ",
    " +++++++++++++++++ ",
    " ################# ",
    "###################",
];
/// A pine hung with lights: `%` needles, `t` the trunk, `o` a bulb and `*`
/// the star on top.
const ABETO: &[&str] = &[
    "    *    ", //
    "    %    ",
    "   %%%   ",
    "   %o%   ",
    "  %%%%%  ",
    "  %o%%%  ",
    " %%%%%%% ",
    " %o%%%o% ",
    "%%%%%%%%%",
    "    t    ",
    "    t    ",
];
/// A nochebuena in its pot: `r` bracts, `g` leaves, `y` the heart, `p` clay.
const NOCHEBUENA_POT: &[&str] = &[
    " r r ", //
    "rgygr", " ggg ", " ppp ",
];

/// A light that blinks in the town: a foquito on a string, a bulb on the
/// kiosco.
#[derive(Clone, Copy)]
struct Bulb {
    x: i32,
    y: i32,
    /// Which of the series lights it is, by its place on the string.
    c: u8,
    /// How it blinks: 0 steady, 1 in a chase along its string, 2 twinkling.
    kind: u8,
    phase: f32,
}

impl Bulb {
    /// How bright it is at this frame, 0.2 to 1.
    fn glow(&self, frame: u32) -> f32 {
        match self.kind {
            0 => 1.0,
            1 => {
                if (frame / 8 + self.c as u32).is_multiple_of(3) {
                    1.0
                } else {
                    0.2
                }
            }
            _ => 0.6 + 0.4 * (frame as f32 * 0.1 + self.phase).sin(),
        }
    }
}

/// Stamps `art` into the town, drawn in blocks of `k` pixels, its left edge
/// on `x` and its bottom row on `base`. `paint` gets each char with the
/// top-left pixel of its block and answers what stands there: 1 solid or 2 a
/// candle flame (a single pixel), and its color; `None` leaves the block as
/// it was.
fn stamp(
    town: &mut [u8],
    cols: &mut [V3],
    (w, ph): (usize, usize),
    (art, k): (&[&str], usize),
    (x, base): (i32, i32),
    mut paint: impl FnMut(u8, i32, i32) -> Option<(u8, V3)>,
) {
    let k = k.max(1) as i32;
    let top = base - art.len() as i32 * k + 1;
    for (sy, row) in art.iter().enumerate() {
        for (sx, ch) in row.bytes().enumerate() {
            let (px, py) = (x + sx as i32 * k, top + sy as i32 * k);
            if ch == b' ' || px < 0 || py < 0 || px >= w as i32 || py >= ph as i32 {
                continue;
            }
            let Some((kind, col)) = paint(ch, px, py) else {
                continue;
            };
            for (dx, dy) in (0..k).flat_map(|i| (0..k).map(move |j| (i, j))) {
                let (bx, by) = (px + dx, py + dy);
                if bx >= w as i32 || by >= ph as i32 || (kind == 2 && (dx, dy) != (0, 0)) {
                    continue;
                }
                let i = by as usize * w + bx as usize;
                town[i] = kind;
                cols[i] = col;
            }
        }
    }
}

/// A veladora on a grave: its flame pixel and flicker phase.
#[derive(Clone, Copy)]
struct Candle {
    x: i32,
    y: i32,
    phase: f32,
    /// 1 for a candle on a grave up front, less for one far away.
    power: f32,
}

/// How bright a candle is now, 0.55 to 1.
fn flicker(t: f32, phase: f32) -> f32 {
    0.78 + 0.12 * (t * 9.1 + phase).sin() + 0.1 * (t * 23.7 + phase * 1.7).sin()
}

struct Fireworks {
    rng: fastrand::Rng,
    speed: f32,
    rate: f32,
    show_city: bool,
    show_stars: bool,
    paint_sky: bool,
    pal: Palette,
    /// Pixel size of the screen (width, twice the rows).
    w: usize,
    ph: usize,
    ps: Vec<P>,
    /// Sparks born during a step, merged afterwards (kept to avoid allocating).
    born: Vec<P>,
    flashes: Vec<Flash>,
    /// Painted sky gradient with its stars, only when `sky = true`.
    sky: Vec<V3>,
    /// Faint stars: pixel index and brightness.
    stars: Vec<(usize, f32)>,
    /// Top pixel row of the skyline in each column.
    skyline: Vec<i32>,
    /// Brightness factor of the building in each column.
    shade: Vec<f32>,
    /// Window pixels: index, color kind and whether the light is on.
    windows: Vec<(usize, u8, bool)>,
    /// Blinking lights on top of antennas: index and phase.
    beacons: Vec<(usize, u32)>,
    light: Vec<V3>,
    glow: Vec<V3>,
    px: Pixels,
    acc: f32,
    frame: u32,
    timer: i32,
    finale: i32,
    until_finale: i32,
    // The holidays.
    seed: u64,
    fiesta: Fiesta,
    /// The holiday's own random numbers (shell shapes and colors).
    frng: fastrand::Rng,
    /// Día de Muertos's colors, and the other holidays'.
    dress: Dress,
    fest: Fest,
    /// What stands in for the city on a holiday (the panteón, the plaza),
    /// per pixel: 0 is sky, 1 is solid (a grave, a wall), 2 a candle flame;
    /// empty when there is none.
    town: Vec<u8>,
    town_col: Vec<V3>,
    candles: Vec<Candle>,
    /// Lights that blink in the town.
    bulbs: Vec<Bulb>,
    /// Steps left of the countdown to the new year, or 0.
    count: i32,
    /// Steps until the gran final, or 0.
    gran: i32,
    /// Where copal burns (pixel x and y).
    copal: Vec<(i32, i32)>,
    /// Candlelight falling on the graves this frame.
    warm: Vec<f32>,
}

/// Where a rocket aimed at `x` and `apex` should rise to burst a whole
/// shaped shell on screen: its x, pulled in from the edges, and its launch
/// speed, so that the burst is low enough and clear of the edges.
fn fit_shape(w: f32, hh: f32, x: f32, apex: f32) -> (f32, f32) {
    let r = w.min(hh) * SHAPE_SIZE.1;
    let apex = apex.max(r * 1.15 + 2.0).min(hh * 0.6);
    let vy = -(2.0 * ROCKET_G * (hh - apex).max(1.0)).sqrt();
    let x = if w > r * 2.2 {
        x.max(r * 1.1).min(w - r * 1.1)
    } else {
        x
    };
    (x, vy)
}

/// The countdown to the new year, in steps: ten seconds.
const COUNT_STEPS: i32 = 300;
/// The phrases the sparks write, by the number a `Shell::Text` carries.
const TEXTS: [&str; 1] = ["FELIZ AÑO NUEVO"];
/// The phrase the new year is greeted with.
const FELIZ_ANO: u8 = 0;

impl Fireworks {
    fn new(ctx: &Ctx) -> Fireworks {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let until_finale = rng.i32(500..1100);
        let pal = Palette {
            native: true,
            accents: vec![GOLD],
            gold: GOLD,
            white: SHELL_WHITE,
            sky_top: SKY_TOP,
            sky_low: SKY_LOW,
            building: BUILDING,
            star: STAR,
            windows: [[0.0; 3]; 3],
        };
        Fireworks {
            seed: ctx.seed,
            fiesta: ctx.fiesta,
            frng: fastrand::Rng::with_seed(ctx.seed ^ MUERTOS_SEED),
            dress: Dress::new(ctx, &pal),
            fest: Fest::new(ctx, &pal),
            town: Vec::new(),
            town_col: Vec::new(),
            candles: Vec::new(),
            bulbs: Vec::new(),
            count: 0,
            copal: Vec::new(),
            gran: 0,
            warm: Vec::new(),
            rng,
            speed: num(o.f("speed"), 1.0, 0.1, 4.0),
            rate: num(o.f("rate"), 1.0, 0.0, 5.0),
            show_city: o.b("skyline"),
            show_stars: o.b("stars"),
            paint_sky: o.b("sky"),
            pal,
            w: 0,
            ph: 0,
            ps: Vec::with_capacity(2048),
            born: Vec::with_capacity(256),
            flashes: Vec::new(),
            sky: Vec::new(),
            stars: Vec::new(),
            skyline: Vec::new(),
            shade: Vec::new(),
            windows: Vec::new(),
            beacons: Vec::new(),
            light: Vec::new(),
            glow: Vec::new(),
            px: Pixels::new(1, 1),
            acc: 0.0,
            frame: 0,
            timer: 5,
            finale: 0,
            until_finale,
        }
    }

    fn launch(&mut self) {
        let (w, hh) = (self.w as f32, self.ph as f32);
        let rng = &mut self.rng;
        let mut x = w * rangef(rng, 0.1, 0.9);
        let apex = rangef(rng, 0.12, 0.5) * hh;
        let mut vy = -(2.0 * ROCKET_G * (hh - apex).max(1.0)).sqrt();
        let mut shell = match rng.u32(0..6) {
            0 => Shell::Sphere,
            1 => Shell::Ring,
            2 => Shell::Willow,
            3 => Shell::Crackle,
            4 => Shell::Multi,
            _ => Shell::Palm,
        };
        let mut color = self.pal.shell_color(rng);
        let vx = rangef(rng, -0.15, 0.15);
        if self.fiesta.is(Kind::Muertos) {
            // Día de Muertos: its own colors, and more calaveras, marigolds
            // and showers of petals as the holiday comes.
            let f = &mut self.frng;
            let level = self.fiesta.level.clamp(0.0, 1.0);
            color = self.dress.shells[f.usize(0..self.dress.shells.len())];
            if f.f32() < 0.14 + 0.22 * level {
                shell = if f.bool() {
                    Shell::Calavera
                } else {
                    Shell::Marigold
                };
                // Burst low enough and far enough from the edges to show
                // the whole shape.
                let r = w.min(hh) * SHAPE_SIZE.1;
                let apex = apex.max(r * 1.15 + 2.0).min(hh * 0.6);
                vy = -(2.0 * ROCKET_G * (hh - apex).max(1.0)).sqrt();
                if w > r * 2.2 {
                    x = x.max(r * 1.1).min(w - r * 1.1);
                }
            } else if matches!(shell, Shell::Willow | Shell::Palm) && f.f32() < 0.5 {
                shell = Shell::Petals;
            }
        } else if self.fiesta.is(Kind::Navidad) {
            // Navidad: its own colors, and more estrellas, nochebuenas and
            // bells as the posadas go by.
            let f = &mut self.frng;
            let level = self.fiesta.level.clamp(0.0, 1.0);
            color = self.fest.shells[f.usize(0..self.fest.shells.len())];
            if f.f32() < 0.14 + 0.24 * level {
                shell = match f.u32(0..5) {
                    0 | 1 => Shell::Estrella,
                    2 | 3 => Shell::Nochebuena,
                    _ => Shell::Campana,
                };
                (x, vy) = fit_shape(w, hh, x, apex);
            }
        }
        self.born.push(P {
            x,
            y: hh,
            vx,
            vy,
            life: 1000.0,
            max: 1000.0,
            color,
            drag: 1.0,
            grav: ROCKET_G,
            flags: ROCKET,
            shell,
        });
    }

    /// Does this holiday end a finale with a gran final?
    fn has_gran(&self) -> bool {
        self.fiesta.is(Kind::Muertos) || self.fiesta.is(Kind::Navidad)
    }

    /// The holiday's gran final, a big shaped shell between two smaller
    /// ones (just the big one when there is no room for them): Día de
    /// Muertos ends with a calavera between two marigolds, Navidad with an
    /// estrella de siete picos between two bells, and on Año Nuevo with the
    /// year's greeting in sparks.
    fn gran_final(&mut self) {
        if self.fiesta.ano_nuevo() && self.write(FELIZ_ANO) {
            return;
        }
        let (mid, side, color) = if self.fiesta.is(Kind::Muertos) {
            (Shell::Calavera, Shell::Marigold, self.dress.bone)
        } else if self.fiesta.is(Kind::Navidad) {
            (Shell::Estrella, Shell::Campana, self.fest.gold)
        } else {
            return;
        };
        let (w, hh) = (self.w as f32, self.ph as f32);
        let r = w.min(hh) * SHAPE_SIZE.1;
        let apex = (r * 1.15 + 2.0).max(hh * 0.4).min(hh * 0.6);
        let vy = -(2.0 * ROCKET_G * (hh - apex).max(1.0)).sqrt();
        let shells: &[(f32, Shell)] = if w > r * 6.0 {
            &[(0.5, mid), (0.2, side), (0.8, side)]
        } else {
            &[(0.5, mid)]
        };
        for &(x, shell) in shells {
            self.born.push(P {
                x: w * x,
                y: hh,
                vx: 0.0,
                // The smaller ones burst a moment after the big one.
                vy: if shell == mid { vy } else { vy * 0.97 },
                life: 1000.0,
                max: 1000.0,
                color,
                drag: 1.0,
                grav: ROCKET_G,
                flags: ROCKET,
                shell,
            });
        }
    }

    /// Sends up a rocket that writes `text` across the sky; false when the
    /// screen is too small to read it.
    fn write(&mut self, text: u8) -> bool {
        let (w, hh) = (self.w as f32, self.ph as f32);
        let Some(fit) = fit_text(TEXTS[text as usize % TEXTS.len()], w, hh) else {
            return false;
        };
        let half = fit.rows() as f32 * fit.cell * 0.5;
        let cy = (hh * 0.38).max(half + 3.0);
        let vy = -(2.0 * ROCKET_G * (hh - cy).max(1.0)).sqrt();
        self.born.push(P {
            x: w * 0.5,
            y: hh,
            vx: 0.0,
            vy,
            life: 1000.0,
            max: 1000.0,
            color: self.fest.gold,
            drag: 1.0,
            grav: ROCKET_G,
            flags: ROCKET,
            shell: Shell::Text(text),
        });
        true
    }

    /// Spawns a phrase's sparks into `born`: every pixel of every letter is
    /// a spark or a few, thrown so that they come to rest in place (as in
    /// `burst_shape`) and hold the writing for a few seconds as they sink.
    fn burst_text(&mut self, r: &P, text: u8) {
        let text = TEXTS[text as usize % TEXTS.len()];
        let Some(fit) = fit_text(text, self.w as f32, self.ph as f32) else {
            return;
        };
        let ft = &self.fest;
        let ink = [ft.gold, ft.red, ft.green];
        self.flashes.push(Flash {
            x: r.x,
            y: r.y,
            radius: fit.cols as f32 * fit.cell * 0.6 + 2.0,
            color: ft.gold,
            power: 0.3,
        });
        let f = &mut self.frng;
        let drag = 0.93;
        let throw = (1.0 - drag) / drag;
        // About one spark per pixel and a half of letter.
        let n = (fit.cell / 1.5).ceil().max(1.0) as i32;
        let sub = fit.cell / n as f32;
        let mut room = MAX_PARTICLES.saturating_sub(self.ps.len() + self.born.len());
        for (li, line) in fit.lines[..fit.count].iter().enumerate() {
            let x0 = -(text_cols(line) as f32) / 2.0;
            let y0 = -(fit.rows() as f32) / 2.0 + (li as i32 * (LINE_ROWS + LINE_GAP)) as f32;
            let (mut at, mut letter) = (0, 0);
            for ch in line.chars() {
                if ch == ' ' {
                    at += 4;
                    continue;
                }
                let (rows, accent) = glyph(ch);
                let color = ink[letter % ink.len()];
                letter += 1;
                for ry in 0..LINE_ROWS {
                    let bits = if ry == 0 {
                        accent
                    } else {
                        rows[ry as usize - 1]
                    };
                    for col in 0..5 {
                        if bits >> (4 - col) & 1 == 0 {
                            continue;
                        }
                        for (sx, sy) in (0..n).flat_map(|i| (0..n).map(move |j| (i, j))) {
                            if room == 0 {
                                return;
                            }
                            room -= 1;
                            let tx = (x0 + (at + col) as f32) * fit.cell + (sx as f32 + 0.5) * sub;
                            let ty = (y0 + ry as f32) * fit.cell + (sy as f32 + 0.5) * sub;
                            let j = rangef(f, 0.99, 1.01);
                            let life = rangef(f, 115.0, 140.0);
                            self.born.push(P {
                                x: r.x,
                                y: r.y,
                                vx: tx * throw * j,
                                vy: ty * throw * j,
                                life,
                                max: life,
                                color,
                                drag,
                                grav: SPARK_G * 0.3,
                                flags: 0,
                                shell: r.shell,
                            });
                        }
                    }
                }
                at += 6;
            }
        }
    }

    /// Copal smoke curls up from a few graves.
    fn burn_copal(&mut self) {
        let f = &mut self.frng;
        for k in 0..self.copal.len() {
            let (x, y) = self.copal[k];
            if f.f32() < 0.12 && self.ps.len() + self.born.len() < MAX_PARTICLES {
                let life = rangef(f, 70.0, 110.0);
                self.born.push(P {
                    x: x as f32 + 0.5 + rangef(f, -0.3, 0.3),
                    y: y as f32,
                    vx: rangef(f, -0.03, 0.03),
                    vy: rangef(f, -0.14, -0.08),
                    life,
                    max: life,
                    color: self.dress.smoke,
                    drag: 0.995,
                    grav: 0.0004,
                    flags: SMOKE,
                    shell: Shell::Sphere,
                });
            }
        }
    }

    /// Spawns a shell's sparks into `born`.
    fn burst(&mut self, r: &P) {
        if let Shell::Text(text) = r.shell {
            return self.burst_text(r, text);
        }
        if r.shell.shaped() {
            return self.burst_shape(r);
        }
        let size = (self.w as f32).min(self.ph as f32);
        let rng = &mut self.rng;
        let radius = size * rangef(rng, 0.18, 0.33);
        let base = (radius * 4.0).clamp(16.0, 220.0) as usize;
        self.flashes.push(Flash {
            x: r.x,
            y: r.y,
            radius: radius * 1.1 + 2.0,
            color: r.color,
            power: 0.2,
        });
        let (x, y) = (r.x, r.y);
        let mut room = MAX_PARTICLES.saturating_sub(self.ps.len() + self.born.len());
        let out = &mut self.born;
        let mut spark = |vx: f32, vy: f32, life: f32, color: V3, drag: f32, flags: u8| {
            if room > 0 {
                room -= 1;
                out.push(P {
                    x,
                    y,
                    vx,
                    vy,
                    life,
                    max: life,
                    color,
                    drag,
                    grav: SPARK_G,
                    flags,
                    shell: Shell::Sphere,
                });
            }
        };
        let (pal, dress, ft) = (&self.pal, &self.dress, &self.fest);
        let navidad = self.fiesta.is(Kind::Navidad);
        let gold = if self.fiesta.is(Kind::Muertos) {
            dress.gold
        } else if navidad {
            ft.gold
        } else {
            pal.gold
        };
        // A color for a multicolor shell or a second ring.
        let shell_color = |rng: &mut fastrand::Rng| {
            if navidad {
                ft.shells[rng.usize(0..ft.shells.len())]
            } else {
                pal.shell_color(rng)
            }
        };
        match r.shell {
            Shell::Sphere | Shell::Multi | Shell::Crackle => {
                let drag = 0.95;
                let v0 = radius * (1.0 - drag);
                let flags = if matches!(r.shell, Shell::Crackle) {
                    CRACKLE | STROBE
                } else {
                    0
                };
                for _ in 0..base {
                    // Project a 3D sphere: denser towards the rim.
                    let a = rng.f32() * TAU;
                    let u = rangef(rng, -1.0, 1.0);
                    let s = v0 * (1.0 - u * u).sqrt() * rangef(rng, 0.9, 1.0);
                    let col = if matches!(r.shell, Shell::Multi) {
                        shell_color(rng)
                    } else {
                        r.color
                    };
                    let life = rangef(rng, 40.0, 65.0);
                    spark(a.cos() * s, a.sin() * s, life, col, drag, flags);
                }
            }
            Shell::Ring => {
                let drag = 0.95;
                let v0 = radius * (1.0 - drag);
                let tilt = rangef(rng, 0.25, 1.0);
                let rot = rng.f32() * TAU;
                let (sr, cr) = rot.sin_cos();
                let second = shell_color(rng);
                let n = base.max(16);
                for k in 0..n {
                    let a = k as f32 / n as f32 * TAU;
                    let (ex, ey) = (a.cos() * v0, a.sin() * v0 * tilt);
                    let (vx, vy) = (ex * cr - ey * sr, ex * sr + ey * cr);
                    let life = rangef(rng, 45.0, 60.0);
                    spark(vx, vy, life, r.color, drag, 0);
                    if k % 3 == 0 {
                        let life = rangef(rng, 35.0, 50.0);
                        spark(vx * 0.45, vy * 0.45, life, second, drag, 0);
                    }
                }
            }
            Shell::Willow => {
                let drag = 0.965;
                let v0 = radius * (1.0 - drag) * 0.9;
                for _ in 0..base {
                    let a = rng.f32() * TAU;
                    let s = v0 * rangef(rng, 0.3, 1.0).sqrt();
                    let life = rangef(rng, 90.0, 140.0);
                    spark(a.cos() * s, a.sin() * s, life, gold, drag, TRAIL);
                }
            }
            Shell::Petals => {
                // Slow and light, so they drift down for a long while.
                let drag = 0.93;
                let v0 = radius * (1.0 - drag) * 0.9;
                for _ in 0..base / 2 {
                    let a = rng.f32() * TAU;
                    let s = v0 * rangef(rng, 0.2, 1.0).sqrt();
                    let life = rangef(rng, 110.0, 170.0);
                    // Mostly marigold, some terciopelo.
                    let col = match rng.u32(0..5) {
                        0 => dress.terciopelo,
                        1 | 2 => dress.mari[1],
                        _ => dress.mari[2],
                    };
                    spark(a.cos() * s, a.sin() * s - 0.1, life, col, drag, PETAL);
                }
            }
            Shell::Calavera
            | Shell::Marigold
            | Shell::Estrella
            | Shell::Nochebuena
            | Shell::Campana
            | Shell::Text(_) => {}
            Shell::Palm => {
                let drag = 0.955;
                let v0 = radius * (1.0 - drag) * 1.1;
                let n = rng.usize(7..12);
                for k in 0..n {
                    let a = k as f32 / n as f32 * TAU + rangef(rng, -0.1, 0.1);
                    let life = rangef(rng, 60.0, 80.0);
                    spark(a.cos() * v0, a.sin() * v0 - 0.2, life, r.color, drag, TRAIL);
                }
            }
        }
    }

    /// Spawns a calavera's or a marigold's sparks into `born`. They all
    /// share drag and gravity, so the shape grows from the burst and sinks
    /// whole as it fades.
    fn burst_shape(&mut self, r: &P) {
        let size = (self.w as f32).min(self.ph as f32);
        let f = &mut self.frng;
        let radius = size * rangef(f, SHAPE_SIZE.0, SHAPE_SIZE.1);
        self.flashes.push(Flash {
            x: r.x,
            y: r.y,
            radius: radius * 1.3 + 2.0,
            color: r.color,
            power: 0.25,
        });
        let (d, ft) = (&self.dress, &self.fest);
        let (strokes, colors): (&[Stroke], [V3; 8]) = match r.shell {
            Shell::Calavera => (
                CALAVERA,
                [
                    d.bone, d.icing[0], d.icing[1], d.icing[2], d.bone, d.bone, d.bone, d.bone,
                ],
            ),
            Shell::Estrella => (ESTRELLA, ft.shape),
            Shell::Nochebuena => (NOCHEBUENA, ft.shape),
            Shell::Campana => (CAMPANA, ft.shape),
            _ => (
                MARIGOLD,
                [
                    d.mari[0], d.mari[1], d.mari[2], d.mari[3], d.leaf, d.leaf, d.leaf, d.leaf,
                ],
            ),
        };
        let drag = 0.93;
        let v0 = radius * (1.0 - drag);
        // About one spark per pixel and a quarter of outline.
        let step = 1.25 / radius.max(1.0);
        let mut room = MAX_PARTICLES.saturating_sub(self.ps.len() + self.born.len());
        for s in strokes {
            let (n, color) = match *s {
                Stroke::Line(x0, y0, x1, y1, c) => ((x1 - x0).hypot(y1 - y0), c),
                Stroke::Arc(_, _, rr, a0, a1, c) => (rr * (a1 - a0).abs() * TAU, c),
                Stroke::Bloom(_, _, rr, _, _, c) => (rr * TAU * 1.15, c),
                Stroke::Star(_, _, ro, ri, p, c, _) => (star_edge(ro, ri, p) * p as f32 * 2.0, c),
                Stroke::Leaf(_, _, len, _, count, _, c) => (len * 2.2 * count as f32, c),
            };
            let n = ((n / step).ceil() as usize).clamp(3, 400);
            for k in 0..n {
                if room == 0 {
                    return;
                }
                room -= 1;
                let u = k as f32 / n as f32;
                let (px, py) = match *s {
                    Stroke::Line(x0, y0, x1, y1, _) => {
                        let u = k as f32 / (n - 1) as f32;
                        (x0 + (x1 - x0) * u, y0 + (y1 - y0) * u)
                    }
                    Stroke::Arc(cx, cy, rr, a0, a1, _) => {
                        let a = (a0 + (a1 - a0) * u) * TAU;
                        (cx + rr * a.cos(), cy - rr * a.sin())
                    }
                    Stroke::Bloom(cx, cy, rr, depth, petals, _) => {
                        let a = u * TAU;
                        let rr = rr * (1.0 - depth + depth * (a * petals * 0.5).cos().abs());
                        (cx + rr * a.cos(), cy - rr * a.sin())
                    }
                    Stroke::Star(cx, cy, ro, ri, p, _, _) => {
                        let (x, y, _) = star_point(ro, ri, p, 0, u);
                        (cx + x, cy + y)
                    }
                    Stroke::Leaf(cx, cy, len, half, count, turn, _) => {
                        let (x, y) = leaf_point(len, half, count, turn, u);
                        (cx + x, cy + y)
                    }
                };
                let color = match *s {
                    Stroke::Star(_, _, ro, ri, p, c0, cycle) => {
                        c0 + star_point(ro, ri, p, cycle, u).2
                    }
                    _ => color,
                };
                let j = rangef(f, 0.97, 1.03);
                let life = rangef(f, 75.0, 95.0);
                self.born.push(P {
                    x: r.x,
                    y: r.y,
                    vx: px * v0 * j,
                    vy: py * v0 * j,
                    life,
                    max: life,
                    color: colors[color as usize % 8],
                    drag,
                    grav: SPARK_G * 0.3,
                    flags: 0,
                    shell: r.shell,
                });
            }
        }
    }

    /// Lays out the city and the stars for the current size.
    fn build_city(&mut self) {
        let (w, ph) = (self.w, self.ph);
        self.stars.clear();
        self.windows.clear();
        self.beacons.clear();
        self.skyline.clear();
        self.skyline.resize(w, ph as i32);
        self.shade.clear();
        self.shade.resize(w, 1.0);
        if w == 0 || ph == 0 {
            return;
        }
        let rng = &mut self.rng;
        if self.show_stars {
            for _ in 0..w * ph / 150 {
                let i = rng.usize(0..(w * ph * 6 / 10).max(1));
                self.stars.push((i, rangef(rng, 0.05, 0.2)));
            }
        }
        let max_h = (ph / 6) as i32;
        if !self.show_city || max_h < 2 {
            return;
        }
        let mut x = 0;
        while x < w {
            let bw = rng.usize(3..9);
            let bh = rng.i32(2..max_h + 1);
            let top = ph as i32 - bh;
            let shade = rangef(rng, 0.7, 1.4);
            let x1 = (x + bw).min(w);
            for xx in x..x1 {
                self.skyline[xx] = top;
                self.shade[xx] = shade;
            }
            // Windows on a grid, leaving a margin around the edges.
            let kind = rng.u8(0..3);
            for yy in (top + 2..ph as i32).step_by(2) {
                for xx in (x + 1..x1.saturating_sub(1)).step_by(2) {
                    self.windows
                        .push((yy as usize * w + xx, kind, rng.f32() < 0.3));
                }
            }
            // Tall buildings get an antenna with a blinking light.
            if bh > max_h * 3 / 4 && bw >= 5 && rng.f32() < 0.35 {
                let ax = x + bw / 2;
                let tall = rng.i32(2..5);
                if ax < w && top - tall > 0 {
                    self.skyline[ax] = top - tall;
                    self.shade[ax] = shade;
                    self.beacons
                        .push((((top - tall) as usize) * w + ax, rng.u32(0..90)));
                }
            }
            x += bw + rng.usize(0..2);
        }
    }

    /// Lays out what stands in for the city on the day's holiday, when it
    /// has something: the panteón on Día de Muertos, the plaza on Navidad.
    fn build_town(&mut self) {
        self.bulbs.clear();
        // Each of them starts by clearing the town, and builds on it only on
        // its own holiday.
        self.build_panteon();
        self.build_plaza();
    }

    /// Puts a solid pixel into the town.
    fn put(&mut self, x: i32, y: i32, c: V3) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.ph {
            let i = y as usize * self.w + x as usize;
            self.town[i] = 1;
            self.town_col[i] = c;
        }
    }

    /// Hangs a string of lights from `a` to `b`, sagging `sag` pixels in the
    /// middle: the wire is part of the town, and a foquito hangs from it
    /// every `gap` pixels, each on with chance `lit` and blinking as `kind`.
    #[allow(clippy::too_many_arguments)]
    fn hang_lights(
        &mut self,
        a: (i32, i32),
        b: (i32, i32),
        sag: f32,
        gap: i32,
        kind: u8,
        lit: f32,
        rng: &mut fastrand::Rng,
    ) {
        let n = (b.0 - a.0).abs().max((b.1 - a.1).abs()).max(1);
        // Never lower than the ground.
        let sag = sag.min(((self.ph as i32 - 8) - a.1.max(b.1)).max(0) as f32);
        let cable = self.fest.cable;
        let mut slot = 0u8;
        for i in 0..=n {
            let u = i as f32 / n as f32;
            let x = (a.0 as f32 + (b.0 - a.0) as f32 * u).round() as i32;
            let y =
                (a.1 as f32 + (b.1 - a.1) as f32 * u + sag * 4.0 * u * (1.0 - u)).round() as i32;
            self.put(x, y, cable);
            if i % gap == gap / 2 {
                // Every place on the string has its color, lit or not.
                if rng.f32() < lit && x >= 0 && y >= 0 && (y as usize) + 1 < self.ph {
                    self.bulbs.push(Bulb {
                        x,
                        y: y + 1,
                        c: slot % 6,
                        kind,
                        phase: rng.f32() * TAU,
                    });
                }
                slot = slot.wrapping_add(1);
            }
        }
    }

    /// Lays out the town plaza that stands in for the city during Navidad:
    /// a row of facades with lit windows, a kiosco in the middle with pines
    /// either side, nochebuenas in their pots, farolitos along the ground
    /// and strings of foquitos from the rooftops to the kiosco. The layout
    /// depends only on the seed and the size; the level decides how many
    /// lights are on and how many strings hang.
    fn build_plaza(&mut self) {
        let (w, ph) = (self.w, self.ph);
        if !self.fiesta.is(Kind::Navidad) || !self.show_city || w == 0 || ph < 12 {
            return;
        }
        self.town.resize(w * ph, 0);
        self.town_col.resize(w * ph, [0.0; 3]);
        let rng = &mut fastrand::Rng::with_seed(self.seed ^ NAVIDAD_SEED);
        let level = self.fiesta.level.clamp(0.0, 1.0);
        let lit = 0.3 + 0.7 * level;
        let strings = 1 + (level * 3.0).round() as usize;
        let ft = &self.fest;
        let (wall, window, ground_c) = (ft.wall, ft.window, ft.ground);
        let (stone, white, wood) = (ft.stone, ft.white, ft.wood);
        let (roof, pine, red, green) = (ft.roof, ft.pine, ft.red, ft.green);
        let trim = mix(stone, white, 0.35);
        let iron = scale(stone, 1.5);
        let wi = w as i32;
        // On a big screen everything is drawn bigger, in blocks of `k`.
        let k = (ph / 50).clamp(1, 3);

        // The cobbled ground.
        let ground = (if ph >= 40 { 2 } else { 1 }) * k;
        let gy = ph - ground;
        for y in gy..ph {
            for x in 0..w {
                self.town[y * w + x] = 1;
                self.town_col[y * w + x] = scale(ground_c, rangef(rng, 0.8, 1.15));
            }
        }

        // The facades on the far side of the plaza, most windows lit, a few
        // roofs strung with lights.
        let max_h = (ph / 6 / k).clamp(3, 14);
        let mut tops = vec![gy as i32; w];
        let mut x = 0;
        while x < w {
            let bw = (rng.usize(7..15) * k).min(w - x);
            let top = gy - rng.usize(3..max_h + 1) * k;
            let base = scale(wall[rng.usize(0..3)], rangef(rng, 0.8, 1.15));
            tops[x..x + bw].fill(top as i32);
            for xx in x..x + bw {
                for y in top..gy {
                    self.town[y * w + xx] = 1;
                    self.town_col[y * w + xx] = base;
                }
                // The parapet catches the light.
                for y in top..top + k {
                    self.town_col[y * w + xx] = mix(base, white, 0.18);
                }
            }
            for y in (top + 2 * k..gy.saturating_sub(k)).step_by(3 * k) {
                for xx in (x + k..(x + bw).saturating_sub(k)).step_by(2 * k) {
                    if rng.f32() < lit {
                        for (dx, dy) in (0..k).flat_map(|i| (0..k).map(move |j| (i, j))) {
                            self.town_col[(y + dy) * w + xx + dx] = window;
                        }
                    }
                }
            }
            if bw >= 6 * k && top > 0 && rng.f32() < 0.5 {
                let kind = 1 + rng.u8(0..2);
                for xx in (x + k..x + bw - k).step_by(2 * k) {
                    if rng.f32() < lit {
                        self.bulbs.push(Bulb {
                            x: xx as i32,
                            y: top as i32 - 1,
                            c: (xx / (2 * k) % 6) as u8,
                            kind,
                            phase: rng.f32() * TAU,
                        });
                    }
                }
            }
            x += bw;
        }

        // The kiosco in the middle, lit from inside, with pines either side.
        let (ki, wd) = (k as i32, 19 * k as i32);
        let base = gy as i32 - 1;
        let mut stands: Vec<(i32, i32)> = Vec::new();
        let mut eaves = None;
        let mut pines = Vec::new();
        if wi >= wd + 25 && ph >= 30 {
            let x0 = (wi / 2 - wd / 2 + rng.i32(-(wi / 10)..wi / 10 + 1)).clamp(0, wi - wd);
            stamp(
                &mut self.town,
                &mut self.town_col,
                (w, ph),
                (KIOSCO, k),
                (x0, base),
                |ch, px, py| match ch {
                    b'*' => {
                        self.bulbs.push(Bulb {
                            x: px,
                            y: py,
                            c: 3,
                            kind: 2,
                            phase: rng.f32() * TAU,
                        });
                        None
                    }
                    b'o' => {
                        if rng.f32() < lit {
                            self.bulbs.push(Bulb {
                                x: px,
                                y: py,
                                c: (px / (2 * ki) % 6) as u8,
                                kind: 1,
                                phase: 0.0,
                            });
                        }
                        Some((1, iron))
                    }
                    b'w' => {
                        self.candles.push(Candle {
                            x: px,
                            y: py,
                            phase: rng.f32() * TAU,
                            power: 0.9,
                        });
                        Some((2, [0.0; 3]))
                    }
                    b'r' => Some((1, scale(roof, if px / ki % 2 == 0 { 1.0 } else { 0.8 }))),
                    b'+' => Some((1, trim)),
                    b'|' => Some((1, iron)),
                    _ => Some((1, mix(stone, white, 0.22))),
                },
            );
            stands.push((x0 - 1, x0 + wd));
            // The strings hang from its eaves.
            let eave = base - (KIOSCO.len() as i32 - 6) * ki;
            eaves = Some((x0 + ki, x0 + wd - ki, eave));
            pines.extend([
                x0 - 12 * ki - rng.i32(0..5 * ki),
                x0 + wd + rng.i32(0..5 * ki),
            ]);
            if wi >= 110 * ki {
                pines.extend([
                    x0 - 25 * ki - rng.i32(0..8 * ki),
                    x0 + wd + 14 * ki + rng.i32(0..8 * ki),
                ]);
            }
        } else if w >= 30 {
            pines.extend([wi / 5, wi * 3 / 4]);
        }
        if ph >= 24 {
            for tx in pines {
                if tx < 0 || tx + 9 * ki > wi {
                    continue;
                }
                stamp(
                    &mut self.town,
                    &mut self.town_col,
                    (w, ph),
                    (ABETO, k),
                    (tx, base),
                    |ch, px, py| match ch {
                        b'*' | b'o' => {
                            if ch == b'*' || rng.f32() < lit {
                                self.bulbs.push(Bulb {
                                    x: px,
                                    y: py,
                                    c: if ch == b'*' {
                                        3
                                    } else {
                                        ((px * 3 + py) % 6) as u8
                                    },
                                    kind: 2,
                                    phase: rng.f32() * TAU,
                                });
                            }
                            Some((1, scale(pine, 0.9)))
                        }
                        b't' => Some((1, wood)),
                        _ => Some((1, scale(pine, rangef(rng, 0.75, 1.1)))),
                    },
                );
                stands.push((tx - 1, tx + 9 * ki));
            }
        }

        // Nochebuenas in their pots, and farolitos along the ground.
        let free = |stands: &[(i32, i32)], x: i32, wd: i32| {
            stands.iter().all(|&(a, b)| x + wd < a || x > b)
        };
        let mut x = rng.i32(2..9);
        while x + 5 * ki < wi {
            if ph >= 24 && free(&stands, x, 5 * ki) && rng.f32() < 0.45 {
                stamp(
                    &mut self.town,
                    &mut self.town_col,
                    (w, ph),
                    (NOCHEBUENA_POT, k),
                    (x, base),
                    |ch, _, _| match ch {
                        b'r' => Some((1, scale(red, 0.8))),
                        b'g' => Some((1, scale(green, 0.45))),
                        b'y' => Some((1, scale(ft.gold, 0.7))),
                        _ => Some((1, mix(wood, c8(140, 74, 30), 0.6))),
                    },
                );
                x += 6 * ki;
            }
            x += rng.i32(5..11) * ki;
        }
        let mut x = rng.i32(1..6);
        while x < wi && ph >= 20 {
            if free(&stands, x, ki) {
                for (dx, dy) in (0..ki).flat_map(|i| (0..ki).map(move |j| (i, j))) {
                    self.put(x + dx, base - dy, scale(window, 0.5));
                }
                self.candles.push(Candle {
                    x,
                    y: base - ki,
                    phase: rng.f32() * TAU,
                    power: 0.8,
                });
                self.town[(base - ki) as usize * w + x as usize] = 2;
            }
            x += rng.i32(6..13) * ki;
        }

        // Strings of foquitos: from the kiosco's eaves to the rooftops either
        // side, and from roof to roof across the plaza, more as the posadas
        // go by.
        if let Some((left, right, eave)) = eaves {
            for (from, side) in [(left, -1), (right, 1)] {
                let far = (from + side * rng.i32(wi / 6..wi / 2 + 1)).clamp(0, wi - 1);
                let sag = (far - from).abs() as f32 * 0.1;
                self.hang_lights(
                    (from, eave),
                    (far, tops[far as usize] - 1),
                    sag,
                    4 * ki,
                    1,
                    lit,
                    rng,
                );
            }
        }
        for s in 0..strings {
            let a = rng.i32(0..wi / 2 + 1);
            let b = (a + rng.i32(wi / 5..wi / 2 + 1)).min(wi - 1);
            let sag = (b - a) as f32 * 0.12;
            let kind = if s % 2 == 0 { 1 } else { 2 };
            let ends = ((a, tops[a as usize] - 1), (b, tops[b as usize] - 1));
            self.hang_lights(ends.0, ends.1, sag, 4 * ki, kind, lit, rng);
        }
    }

    /// Lays out the panteón that stands in for the city during Día de
    /// Muertos. The layout depends only on the seed and the size; the level
    /// decides how many graves are dressed and how many candles are lit.
    fn build_panteon(&mut self) {
        self.town.clear();
        self.town_col.clear();
        self.candles.clear();
        self.copal.clear();
        let (w, ph) = (self.w, self.ph);
        if !self.fiesta.is(Kind::Muertos) || !self.show_city || w == 0 || ph < 12 {
            return;
        }
        self.town.resize(w * ph, 0);
        self.town_col.resize(w * ph, [0.0; 3]);
        let rng = &mut fastrand::Rng::with_seed(self.seed ^ MUERTOS_SEED);
        let level = self.fiesta.level.clamp(0.0, 1.0);
        let (dressed, lit) = (0.3 + 0.7 * level, 0.4 + 0.6 * level);
        let d = &self.dress;

        // The ground, strewn with petals, rising behind into a low hill
        // crowded with far-off crosses and candles.
        let ground = if ph >= 40 { 2 } else { 1 };
        let gy = ph - ground;
        let hill = (ph / 14).max(1) as f32;
        let (p1, p2) = (rng.f32() * TAU, rng.f32() * TAU);
        let tops: Vec<usize> = (0..w)
            .map(|x| {
                let u = x as f32;
                let rise = 0.5 + 0.3 * (u * 0.045 + p1).sin() + 0.2 * (u * 0.13 + p2).sin();
                gy - (rise * hill).round() as usize
            })
            .collect();
        let far = scale(d.stone, 0.5);
        for (x, &top) in tops.iter().enumerate() {
            for y in top..ph {
                let i = y * w + x;
                self.town[i] = 1;
                let (petal, shade) = (rng.f32() < 0.35 * level, rng.usize(0..3));
                self.town_col[i] = match y {
                    _ if y < gy => far,
                    _ if y == gy && petal => scale(d.mari[shade], 0.75),
                    _ => d.earth,
                };
            }
        }
        let mut x = 0;
        while x < w {
            let top = tops[x];
            let roll = rng.f32();
            let (on, phase) = (rng.f32() < lit, rng.f32() * TAU);
            if roll < 0.22 && on && top > 0 {
                let i = (top - 1) * w + x;
                self.town[i] = 2;
                self.candles.push(Candle {
                    x: x as i32,
                    y: (top - 1) as i32,
                    phase,
                    power: 0.3,
                });
            } else if roll > 0.7 && x + 3 < w {
                let art = if roll > 0.88 { FAR_CROSS } else { FAR_STONE };
                let far = scale(d.stone, 0.75);
                let base = tops[x..x + 3].iter().max().copied().unwrap_or(top);
                for (sy, row) in art.iter().enumerate() {
                    for (sx, ch) in row.bytes().enumerate() {
                        let py = base as i32 - art.len() as i32 + sy as i32;
                        if ch != b' ' && py >= 0 {
                            let i = py as usize * w + x + sx;
                            self.town[i] = 1;
                            self.town_col[i] = if ch == b'+' {
                                mix(far, d.face, 0.5)
                            } else {
                                far
                            };
                        }
                    }
                }
                x += 4;
                continue;
            }
            x += 1;
        }

        let max_h = ph / 5;
        let total: u32 = GRAVES.iter().map(|g| g.1).sum();
        let mut chapel = (w >= 70 && ph >= 64).then(|| rng.usize(w / 8..w * 2 / 3));
        let mut x = rng.usize(0..3);
        while x < w {
            let art = match chapel {
                Some(cx) if x >= cx && CHAPEL.len() <= ph * 3 / 10 => {
                    chapel = None;
                    CHAPEL
                }
                _ => {
                    let mut k = rng.u32(0..total);
                    let mut art = GRAVES[0].0;
                    for &(a, n) in GRAVES {
                        if k < n {
                            art = a;
                            break;
                        }
                        k -= n;
                    }
                    if art.len() > max_h {
                        x += 2;
                        continue;
                    }
                    art
                }
            };
            let shade = rangef(rng, 0.8, 1.2);
            let dress = rng.f32() < dressed;
            // Most graves are painted, faded by the night.
            let paint = d.paint[rng.usize(0..d.paint.len())];
            let mut stone = mix(d.stone, paint, rangef(rng, 0.0, 0.3));
            if std::ptr::eq(art, CHAPEL) {
                // The chapel is whitewashed.
                stone = mix(d.stone, d.bone, 0.12);
            }
            let top = gy as i32 - art.len() as i32;
            let mut aw = 0;
            for (sy, row) in art.iter().enumerate() {
                aw = aw.max(row.len());
                for (sx, ch) in row.bytes().enumerate() {
                    let (px, py) = ((x + sx) as i32, top + sy as i32);
                    if ch == b' ' || px >= w as i32 || py < 0 {
                        continue;
                    }
                    let i = py as usize * w + px as usize;
                    let earth = d.earth;
                    let col = match ch {
                        b'#' => scale(stone, shade),
                        b'+' => scale(d.face, shade),
                        b'%' => scale(d.cypress, shade),
                        b'=' => earth,
                        b'w' => d.window,
                        b'o' | b'a' if dress => d.mari[1],
                        b'O' | b'A' if dress => d.mari[2],
                        b'x' if dress => d.mari[0],
                        b't' | b'b' if dress => d.terciopelo,
                        b'n' if dress => d.nube,
                        b'o' | b'O' | b'x' | b't' | b'n' => earth,
                        b'f' => {
                            // Each veladora has its own night to be lit.
                            let on = rng.f32() < lit;
                            let phase = rng.f32() * TAU;
                            // Some have a sahumerio of copal beside them.
                            if rng.f32() < 0.2 && on && dress {
                                self.copal.push((px, py - 1));
                            }
                            if on {
                                self.town[i] = 2;
                                self.candles.push(Candle {
                                    x: px,
                                    y: py,
                                    phase,
                                    power: 1.0,
                                });
                            }
                            continue;
                        }
                        b'v' if py > 0 && self.town[i - w] == 2 => d.glass[px as usize % 3],
                        _ => continue,
                    };
                    self.town[i] = 1;
                    // Marigolds and candle glass are drawn a little dim:
                    // the candles light them up.
                    self.town_col[i] = match ch {
                        b'o' | b'O' | b'x' | b't' | b'n' | b'a' | b'A' | b'b' if dress => {
                            scale(col, 0.62)
                        }
                        b'v' => scale(col, 0.5),
                        _ => col,
                    };
                }
            }
            x += aw + rng.usize(1..4);
        }
    }

    /// The time candles flicker by, in seconds: it moves on ten times a
    /// second, so a still sky does not redraw every frame.
    fn candle_clock(&self) -> f32 {
        (self.frame / 3) as f32 * STEP * 3.0
    }

    /// Candlelight for this frame: it warms the graves around each flame
    /// and leaves a faint halo in the sky above it.
    fn light_candles(&mut self) {
        self.warm.fill(0.0);
        let t = self.candle_clock();
        let (w, ph) = (self.w as i32, self.ph as i32);
        const R: i32 = 5;
        for k in 0..self.candles.len() {
            let c = self.candles[k];
            let f = flicker(t, c.phase) * c.power;
            self.plot(c.x, c.y - 1, FLAMA_EDGE, 0.25 * f * f);
            if c.power < 0.5 {
                // Too far away to light anything up.
                continue;
            }
            for y in (c.y - R).max(0)..(c.y + R + 1).min(ph) {
                for x in (c.x - R).max(0)..(c.x + R + 1).min(w) {
                    let d2 = ((x - c.x).pow(2) + (y - c.y).pow(2)) as f32;
                    if d2 <= (R * R) as f32 {
                        self.warm[(y * w + x) as usize] += f * 0.8 / (1.0 + d2 * 0.3);
                    }
                }
            }
        }
    }

    /// The faint halo each lit bulb leaves in the sky around it.
    fn light_bulbs(&mut self) {
        for k in 0..self.bulbs.len() {
            let b = self.bulbs[k];
            let g = b.glow(self.frame);
            let c = self.fest.hues[b.c as usize % 6];
            for (ox, oy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                self.plot(b.x + ox, b.y + oy, c, 0.07 * g);
            }
        }
    }

    /// Draws the bulbs over the town.
    fn draw_bulbs(&mut self) {
        for b in &self.bulbs {
            let c = scale(
                self.fest.hues[b.c as usize % 6],
                0.35 + 0.65 * b.glow(self.frame),
            );
            self.px.set(b.x, b.y, to_color(c));
        }
    }

    /// The countdown to the new year: the number in big digits of light,
    /// bright as it changes and dimming through the second.
    fn draw_count(&mut self) {
        if self.count <= 0 {
            return;
        }
        let digit = ((self.count + 29) / 30) as u32;
        let phase = ((self.count - 1) % 30) as f32 / 30.0;
        let (w, ph) = (self.w as i32, self.ph as i32);
        let cell = (ph / 16).min(w * 6 / 10 / 11).clamp(1, 6);
        let color = mix(self.fest.gold, self.fest.white, 0.35);
        let alpha = 0.55 + 0.45 * phase;
        let digits: &[u32] = if digit >= 10 { &[1, 0] } else { &[digit] };
        let width = digits.len() as i32 * 6 - 1;
        let (x0, y0) = (w / 2 - width * cell / 2, ph * 34 / 100 - 5 * cell / 2);
        for (k, &d) in digits.iter().enumerate() {
            let (rows, _) = glyph(char::from_digit(d, 10).unwrap_or('0'));
            for (ry, bits) in rows.iter().enumerate() {
                for col in 0..5 {
                    if bits >> (4 - col) & 1 == 0 {
                        continue;
                    }
                    let (gx, gy) = (x0 + (k as i32 * 6 + col) * cell, y0 + ry as i32 * cell);
                    // Big digits are a grid of lights, with a gap between.
                    let block = if cell >= 3 { cell - 1 } else { cell };
                    for (dx, dy) in (0..block).flat_map(|i| (0..block).map(move |j| (i, j))) {
                        self.plot(gx + dx, gy + dy, color, alpha);
                    }
                }
            }
        }
    }

    /// Fills the painted sky (gradient plus stars) when `sky = true`.
    fn build_sky(&mut self) {
        self.sky.clear();
        if !self.paint_sky {
            return;
        }
        let (w, ph) = (self.w, self.ph);
        let (top, low) = (self.pal.sky_top, self.pal.sky_low);
        let denom = ph.max(2) as f32;
        self.sky
            .extend((0..w * ph).map(|i| mix(top, low, (i / w.max(1)) as f32 / denom)));
        for &(i, a) in &self.stars {
            if let Some(p) = self.sky.get_mut(i) {
                *p = mix(*p, self.pal.star, a);
            }
        }
    }

    /// Add light to a single pixel.
    #[inline]
    fn plot(&mut self, x: i32, y: i32, c: V3, a: f32) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.ph {
            let p = &mut self.light[y as usize * self.w + x as usize];
            for (d, s) in p.iter_mut().zip(c) {
                *d += s * a;
            }
        }
    }

    /// Spread light at a sub-pixel position over the four nearest pixels.
    #[inline]
    fn splat(&mut self, x: f32, y: f32, c: V3, a: f32) {
        let (fx, fy) = (x - 0.5, y - 0.5);
        let (x0, y0) = (fx.floor(), fy.floor());
        let (tx, ty) = (fx - x0, fy - y0);
        let (x0, y0) = (x0 as i32, y0 as i32);
        self.plot(x0, y0, c, a * (1.0 - tx) * (1.0 - ty));
        self.plot(x0 + 1, y0, c, a * tx * (1.0 - ty));
        self.plot(x0, y0 + 1, c, a * (1.0 - tx) * ty);
        self.plot(x0 + 1, y0 + 1, c, a * tx * ty);
    }

    /// A hot pixel with an anti-aliased streak behind it, `len` steps of
    /// motion long, fading towards the tail.
    fn streak(&mut self, p: &P, c: V3, a: f32, len: f32) {
        let (dx, dy) = (p.vx * len, p.vy * len);
        let n = dx.abs().max(dy.abs()).min(16.0).ceil() as i32;
        for j in 1..n {
            let u = j as f32 / n as f32;
            self.splat(p.x - dx * u, p.y - dy * u, c, a * 0.4 * (1.0 - u));
        }
        self.plot(p.x.floor() as i32, p.y.floor() as i32, c, a);
    }

    fn draw_particles(&mut self) {
        self.light.fill([0.0; 3]);
        let native = self.pal.native;
        for i in 0..self.ps.len() {
            let p = self.ps[i];
            let r = (p.life / p.max).clamp(0.0, 1.0);
            if p.flags & STROBE != 0 && r < 0.6 && self.rng.f32() < 0.45 {
                continue;
            }
            if p.flags & ROCKET != 0 {
                let c = if native {
                    ROCKET_FIRE
                } else {
                    mix(self.pal.gold, self.pal.white, 0.5)
                };
                self.streak(&p, c, 1.2, 3.0);
                continue;
            }
            if p.flags & SMOKE != 0 {
                // Fades in, then thins out as it rises.
                let a = (1.0 - r).min(0.15) * 6.0 * r;
                self.splat(p.x, p.y, p.color, a);
                continue;
            }
            if p.flags & PETAL != 0 {
                // A petal tumbles, showing its lit face and its shaded one.
                let face = ((p.life * 0.2 + p.max) as i32) & 1 == 0;
                let fade = r.min(0.25) * 4.0;
                let col = scale(p.color, if face { 1.0 } else { 0.55 });
                self.plot(p.x.floor() as i32, p.y.floor() as i32, col, fade);
                continue;
            }
            let ember = if native { EMBER } else { scale(p.color, 0.3) };
            let mut col = scale(mix(p.color, ember, (1.0 - r) * 1.1), 0.35 + 0.65 * r);
            // Short-lived bits (trail embers, rocket sparks, crackles) are
            // dimmer and never white-hot; real stars flash white, then bloom.
            if p.max <= 20.0 {
                self.streak(&p, col, 0.45, 2.0);
                continue;
            }
            if r > 0.94 {
                col = mix(col, [1.0; 3], (r - 0.94) * 10.0);
            }
            self.streak(&p, col, 1.0, 6.0);
            if r > 0.6 && r < 0.95 {
                let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
                let b = (r - 0.6) * 0.6;
                for (ox, oy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    self.plot(x + ox, y + oy, col, b);
                }
            }
        }
    }

    fn draw_glow(&mut self) {
        self.glow.fill([0.0; 3]);
        let (w, ph) = (self.w as i32, self.ph as i32);
        for f in &self.flashes {
            let r = f.radius;
            let (x0, x1) = (((f.x - r) as i32).max(0), ((f.x + r) as i32 + 1).min(w));
            let (y0, y1) = (((f.y - r) as i32).max(0), ((f.y + r) as i32 + 1).min(ph));
            for y in y0..y1 {
                for x in x0..x1 {
                    let (dx, dy) = ((x as f32 + 0.5 - f.x) / r, (y as f32 + 0.5 - f.y) / r);
                    let d = dx * dx + dy * dy;
                    if d >= 1.0 {
                        continue;
                    }
                    let g = f.power * (1.0 - d) * (1.0 - d);
                    let p = &mut self.glow[(y * w + x) as usize];
                    for (d, s) in p.iter_mut().zip(f.color) {
                        *d += s * g;
                    }
                }
            }
        }
    }

    /// One tanim tick: scheduling, then moving every particle.
    fn step(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        self.born.clear();

        // Launch scheduling: calm stretches, then an occasional finale.
        self.until_finale -= 1;
        if self.until_finale <= 0 {
            if self.fiesta.ano_nuevo() && self.rate > 0.0 {
                // On Año Nuevo the countdown comes first; the finale breaks
                // out when it reaches zero.
                self.count = COUNT_STEPS;
                self.until_finale = self.rng.i32(900..1600) + COUNT_STEPS;
            } else {
                self.finale = self.rng.i32(90..150);
                self.until_finale = self.rng.i32(900..1600);
            }
        }
        if self.count > 0 {
            self.count -= 1;
            // A rocket every second, rising to the next number.
            if self.count % 30 == 0 && self.count > 0 {
                self.launch();
            }
            if self.count == 0 {
                self.finale = self.rng.i32(90..150);
                self.timer = 1;
            }
        } else {
            self.timer -= 1;
        }
        if self.timer <= 0 && self.count == 0 {
            // A higher rate shortens the gaps between launches.
            let k = self.rate.max(0.05);
            let gap = |rng: &mut fastrand::Rng, a: i32, b: i32| {
                ((rng.i32(a..b) as f32 / k).round() as i32).max(1)
            };
            if self.finale > 0 {
                if self.rate > 0.0 {
                    for _ in 0..self.rng.i32(1..3) {
                        self.launch();
                    }
                }
                self.timer = gap(&mut self.rng, 3, 7);
            } else {
                if self.rate > 0.0 {
                    self.launch();
                }
                self.timer = gap(&mut self.rng, 12, 55);
            }
        }
        if self.finale > 0 {
            self.finale -= 1;
            if self.finale == 0 {
                self.timer = 70;
                if self.has_gran() && self.rate > 0.0 {
                    // Once the last of the finale has faded.
                    self.gran = 45;
                }
            }
        }
        if self.gran > 0 {
            self.gran -= 1;
            if self.gran == 0 && self.has_gran() {
                self.gran_final();
            }
        }
        if !self.town.is_empty() {
            self.burn_copal();
        }

        let ph = self.ph as f32;
        let mut i = 0;
        while i < self.ps.len() {
            let p = &mut self.ps[i];
            p.vx *= p.drag;
            p.vy = p.vy * p.drag + p.grav;
            if p.flags & PETAL != 0 {
                // Petals sway from side to side as they fall.
                p.vx += 0.012 * (p.life * 0.11 + p.max).sin();
                p.vy = p.vy.min(0.22);
            } else if p.flags & SMOKE != 0 {
                p.vx += 0.004 * (p.life * 0.07 + p.x * 0.3).sin();
            }
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 1.0;
            let p = *p;
            let mut dead = p.life <= 0.0 || p.y > ph + 2.0;
            if p.flags & ROCKET != 0 {
                let rng = &mut self.rng;
                if rng.f32() < 0.8 {
                    self.born.push(P {
                        x: p.x + rangef(rng, -0.3, 0.3),
                        y: p.y + 1.0,
                        vx: rangef(rng, -0.08, 0.08),
                        vy: rangef(rng, 0.0, 0.15),
                        life: rangef(rng, 6.0, 14.0),
                        max: 14.0,
                        color: if self.pal.native {
                            TRAIL_FIRE
                        } else {
                            self.pal.gold
                        },
                        drag: 0.9,
                        grav: 0.005,
                        flags: 0,
                        shell: Shell::Sphere,
                    });
                }
                if p.vy > -0.25 {
                    self.burst(&p);
                    dead = true;
                }
            } else if p.flags & TRAIL != 0 && self.rng.f32() < 0.5 {
                self.born.push(P {
                    vx: 0.0,
                    vy: 0.02,
                    life: 18.0,
                    max: 18.0,
                    drag: 0.9,
                    grav: 0.002,
                    flags: 0,
                    ..p
                });
            }
            if dead && p.flags & CRACKLE != 0 {
                for _ in 0..2 {
                    let rng = &mut self.rng;
                    self.born.push(P {
                        vx: rangef(rng, -0.25, 0.25),
                        vy: rangef(rng, -0.25, 0.25),
                        life: rangef(rng, 4.0, 9.0),
                        max: 9.0,
                        color: [1.0; 3],
                        drag: 0.85,
                        flags: 0,
                        ..p
                    });
                }
            }
            if dead {
                self.ps.swap_remove(i);
            } else {
                i += 1;
            }
        }
        let room = MAX_PARTICLES.saturating_sub(self.ps.len());
        let n = self.born.len().min(room);
        self.ps.extend_from_slice(&self.born[..n]);

        for f in &mut self.flashes {
            f.power *= 0.82;
        }
        self.flashes.retain(|f| f.power > 0.02);

        // Occasionally someone turns a light on or off.
        if !self.windows.is_empty() && self.rng.f32() < 0.05 {
            let k = self.rng.usize(0..self.windows.len());
            self.windows[k].2 = !self.windows[k].2;
        }
    }

    /// Turns accumulated light on a transparent sky into a visible pixel.
    /// Weak light fades toward the terminal background; with no known
    /// background it dims instead, and the faintest part is dropped so
    /// there are no dark halos.
    fn sky_light(ctx: &Ctx, l: V3) -> Color {
        let m = l[0].max(l[1]).max(l[2]);
        let known = matches!(ctx.bg, Color::Rgb(..));
        if m < if known { 0.04 } else { 0.1 } {
            return Color::Reset;
        }
        // Hue at full strength; hot light spills over into white.
        let over = (m - 1.0).max(0.0) * 0.3;
        let hue = [l[0] / m + over, l[1] / m + over, l[2] / m + over];
        let alpha = m.min(1.0);
        let c = to_color(hue);
        if known {
            ctx.fade(c, 1.0 - alpha)
        } else {
            c.scale(0.4 + 0.6 * alpha)
        }
    }
}

impl Scene for Fireworks {
    fn resize(&mut self, ctx: &Ctx, w: u16, h: u16) {
        self.w = w as usize;
        self.ph = h as usize * 2;
        self.px.resize(w, h);
        self.light = vec![[0.0; 3]; self.w * self.ph];
        self.glow = vec![[0.0; 3]; self.w * self.ph];
        self.ps.clear();
        self.flashes.clear();
        self.build_city();
        self.build_sky();
        self.fiesta = ctx.fiesta;
        self.warm = vec![0.0; self.w * self.ph];
        self.build_town();
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let t = ctx.theme;
        self.pal = if t.native {
            Palette {
                native: true,
                accents: vec![GOLD],
                gold: GOLD,
                white: SHELL_WHITE,
                sky_top: SKY_TOP,
                sky_low: SKY_LOW,
                // Lighter than tanim's, which has a painted sky to stand out against.
                building: if self.paint_sky {
                    BUILDING
                } else {
                    scale(BUILDING, 3.0)
                },
                star: STAR,
                windows: [v3(rgb(0x463a1a)), v3(rgb(0x5f4e22)), v3(rgb(0x283448))],
            }
        } else {
            let g = &t.grad;
            let low = v3(g.at(0.0));
            let mid = v3(g.at(0.5));
            let mut accents: Vec<V3> = t.colors.iter().map(|&c| v3(c)).collect();
            if accents.is_empty() {
                accents.push(v3(g.at(0.8)));
            }
            Palette {
                native: false,
                accents,
                gold: v3(g.at(0.8)),
                white: v3(g.at(1.0)),
                sky_top: scale(v3(t.bg), 0.6),
                sky_low: mix(v3(t.bg), mid, 0.15),
                building: mix(low, mid, if self.paint_sky { 0.1 } else { 0.25 }),
                star: v3(g.at(1.0)),
                windows: [
                    scale(v3(g.at(0.85)), 0.6),
                    scale(v3(g.at(0.6)), 0.7),
                    scale(v3(g.at(0.3)), 0.9),
                ],
            }
        };
        self.build_sky();
        self.fiesta = ctx.fiesta;
        self.dress = Dress::new(ctx, &self.pal);
        self.fest = Fest::new(ctx, &self.pal);
        if self.fiesta.ano_nuevo() {
            // The new year is not far off.
            self.until_finale = self.until_finale.min(240);
        }
        self.build_town();
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.25)
        } else {
            0.0
        };
        self.acc += dt * self.speed;
        let mut n = 0;
        while self.acc >= STEP {
            self.acc -= STEP;
            self.step();
            n += 1;
            if n >= 8 {
                self.acc = 0.0;
                break;
            }
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let (w, ph) = (self.w, self.ph);
        if w == 0 || ph == 0 {
            return;
        }
        self.draw_glow();
        self.draw_particles();
        self.draw_count();
        // On a holiday a town stands where the city was: the panteón, the
        // plaza.
        let panteon = !self.town.is_empty();
        if panteon {
            self.light_candles();
            self.light_bulbs();
        }
        self.px.clear();
        let known = matches!(ctx.bg, Color::Rgb(..));
        let b = self.pal.building;
        let warm = self.dress.window;
        for y in 0..ph {
            for x in 0..w {
                let i = y * w + x;
                let g = self.glow[i];
                let sky = if panteon {
                    self.town[i] == 0
                } else {
                    (y as i32) < self.skyline[x]
                };
                let col = if sky {
                    let l = self.light[i];
                    if self.paint_sky {
                        // Hot light spills over into white where a channel clips.
                        let over = (l[0].max(l[1]).max(l[2]) - 1.0).max(0.0) * 0.3;
                        let s = self.sky[i];
                        to_color([
                            s[0] + (l[0] + over).min(1.0) + g[0],
                            s[1] + (l[1] + over).min(1.0) + g[1],
                            s[2] + (l[2] + over).min(1.0) + g[2],
                        ])
                    } else {
                        Self::sky_light(ctx, l)
                    }
                } else if panteon {
                    // Graves hide the sparks; bursts and candles light them.
                    let t = self.town_col[i];
                    let k = self.warm[i];
                    to_color([
                        t[0] * (1.0 + k * 0.7) + warm[0] * k * 0.55 + g[0] * 0.3,
                        t[1] * (1.0 + k * 0.7) + warm[1] * k * 0.55 + g[1] * 0.3,
                        t[2] * (1.0 + k * 0.7) + warm[2] * k * 0.55 + g[2] * 0.3,
                    ])
                } else {
                    // Buildings hide the sparks but catch a little of the glow.
                    let k = self.shade[x];
                    to_color([
                        b[0] * k + g[0] * 0.3,
                        b[1] * k + g[1] * 0.3,
                        b[2] * k + g[2] * 0.3,
                    ])
                };
                self.px.set(x as i32, y as i32, col);
            }
        }
        if !self.paint_sky {
            // Stars shine where the sky is empty.
            let star = to_color(self.pal.star);
            for &(i, a) in &self.stars {
                let (x, y) = ((i % w) as i32, (i / w) as i32);
                let sky = if panteon {
                    self.town[i] == 0
                } else {
                    y < self.skyline[x as usize]
                };
                if sky && self.px.get(x, y) == Color::Reset {
                    let alpha = 0.2 + a * 1.5;
                    let col = if known {
                        ctx.fade(star, 1.0 - alpha)
                    } else {
                        star.scale(alpha)
                    };
                    self.px.set(x, y, col);
                }
            }
        }
        if panteon {
            let t = self.candle_clock();
            for k in &self.candles {
                let f = flicker(t, k.phase);
                let flame = mix(FLAMA_EDGE, FLAMA, f * 1.4 - 0.4);
                self.px
                    .set(k.x, k.y, to_color(scale(flame, 0.4 + 0.6 * k.power)));
            }
            self.draw_bulbs();
            self.px.blit(c);
            return;
        }
        for &(i, kind, on) in &self.windows {
            if on {
                let (x, y) = ((i % w) as i32, (i / w) as i32);
                self.px
                    .set(x, y, to_color(self.pal.windows[kind as usize % 3]));
            }
        }
        for &(i, phase) in &self.beacons {
            if (self.frame.wrapping_add(phase)) % 90 < 45 {
                let (x, y) = ((i % w) as i32, (i / w) as i32);
                let c = if self.pal.native {
                    BEACON
                } else {
                    self.pal.accents[0]
                };
                self.px.set(x, y, to_color(c));
            }
        }
        self.px.blit(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::Theme;

    fn make(opts: &str, theme: &Theme) -> (Fireworks, Opts) {
        let o = Opts::new(INFO.options, opts.parse().unwrap());
        let ctx = Ctx {
            theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::NONE,
        };
        let mut f = Fireworks::new(&ctx);
        f.resize(&ctx, 80, 24);
        f.retheme(&ctx);
        (f, o)
    }

    /// Odd options and sizes survive long runs and huge steps.
    #[test]
    fn variants_survive() {
        let variants = [
            "",
            "sky = true\nrate = 5.0\nspeed = 4.0",
            "skyline = false\nstars = false\nrate = 0.0",
            "speed = nan\nrate = inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                for fiesta in [
                    Fiesta::NONE,
                    Fiesta::of_day(11, 2),
                    Fiesta::of_day(12, 24),
                    Fiesta::of_day(12, 31),
                ] {
                    let (mut f, o) = make(v, &theme);
                    let ctx = Ctx {
                        theme: &theme,
                        opts: &o,
                        bg: Color::Reset,
                        seed: 3,
                        fiesta,
                    };
                    // The holiday has more to draw: fewer frames of it do.
                    let frames = match fiesta {
                        f if f.ano_nuevo() => 700,
                        f if f.on() => 200,
                        _ => 400,
                    };
                    for (w, h) in [(80, 24), (1, 1), (2, 3), (7, 2), (300, 90)] {
                        f.resize(&ctx, w, h);
                        f.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        // A big screen has more to draw: fewer frames of it.
                        let frames = if w * h > 5000 { frames / 4 } else { frames };
                        for k in 0..frames {
                            f.update(&ctx, if k % 97 == 96 { 5.0 } else { 1.0 / 30.0 });
                            c.clear();
                            f.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    fn holiday(f: &mut Fireworks, theme: &Theme, o: &Opts, fiesta: Fiesta) {
        let ctx = Ctx {
            theme,
            opts: o,
            bg: Color::Reset,
            seed: 3,
            fiesta,
        };
        f.resize(&ctx, 120, 36);
        f.retheme(&ctx);
    }

    /// Runs the show for a while: the shells launched, and whether any
    /// petal or copal smoke was in the air.
    fn watch(f: &mut Fireworks, steps: usize) -> (Vec<Shell>, bool) {
        let mut shells = Vec::new();
        let mut wisps = false;
        for _ in 0..steps {
            f.step();
            for p in &f.ps {
                if p.flags & ROCKET != 0 && !shells.contains(&p.shell) {
                    shells.push(p.shell);
                }
                wisps |= p.flags & (PETAL | SMOKE) != 0;
            }
        }
        (shells, wisps)
    }

    /// Día de Muertos turns the city into a panteón with lit candles and
    /// bursts calaveras, marigolds and petals, from its first day.
    #[test]
    fn muertos_dresses_up() {
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (m, d) in [(11, 2), (10, 28)] {
                let fiesta = Fiesta::of_day(m, d);
                let (mut f, o) = make("", &theme);
                holiday(&mut f, &theme, &o, fiesta);
                assert!(f.town.contains(&1), "{m}-{d}");
                assert!(!f.candles.is_empty(), "{m}-{d}");
                let (shells, wisps) = watch(&mut f, 3000);
                for s in [Shell::Calavera, Shell::Marigold, Shell::Petals] {
                    assert!(shells.contains(&s), "{m}-{d} {shells:?}");
                }
                assert!(wisps);
                // The ground is drawn all along the bottom.
                let ctx = Ctx {
                    theme: &theme,
                    opts: &o,
                    bg: Color::Reset,
                    seed: 3,
                    fiesta,
                };
                let mut c = Canvas::new(120, 36, Color::Reset);
                f.draw(&ctx, &mut c);
                assert!(c.cells()[35 * 120..].iter().all(|cell| cell.ch != ' '));
            }
        }
    }

    /// On an ordinary day there is no panteón and nothing of the holiday.
    #[test]
    fn ordinary_days_are_plain() {
        let theme = Theme::classic();
        let (mut f, _) = make("", &theme);
        assert!(f.town.is_empty() && f.candles.is_empty() && f.copal.is_empty());
        assert!(f.bulbs.is_empty() && f.count == 0);
        let (shells, wisps) = watch(&mut f, 3000);
        assert!(!wisps);
        for s in [
            Shell::Calavera,
            Shell::Marigold,
            Shell::Petals,
            Shell::Estrella,
            Shell::Nochebuena,
            Shell::Campana,
        ] {
            assert!(!shells.contains(&s), "{shells:?}");
        }
    }

    /// The holiday comes and goes through `retheme`, more candles are lit
    /// as it comes, and the panteón stays where it was.
    #[test]
    fn holiday_comes_and_goes() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(10, 28));
        let (first, tomb) = (f.candles.len(), f.town.clone());
        holiday(&mut f, &theme, &o, Fiesta::of_day(11, 2));
        assert!(f.candles.len() > first, "{first} {}", f.candles.len());
        // Arches come with the dressing; nothing that stood is gone.
        assert!(tomb.iter().zip(&f.town).all(|(a, b)| *a == 0 || *b != 0));
        holiday(&mut f, &theme, &o, Fiesta::NONE);
        assert!(f.town.is_empty() && f.candles.is_empty());
        // Without the skyline there is no panteón either.
        let (mut f, o) = make("skyline = false", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(11, 2));
        assert!(f.town.is_empty());
    }

    /// A shaped shell's sparks share drag and gravity, so it keeps its shape.
    #[test]
    fn shapes_hold_together() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(11, 2));
        for shell in [
            Shell::Calavera,
            Shell::Marigold,
            Shell::Estrella,
            Shell::Nochebuena,
            Shell::Campana,
            Shell::Text(FELIZ_ANO),
        ] {
            f.born.clear();
            let r = P {
                x: 60.0,
                y: 30.0,
                vx: 0.0,
                vy: 0.0,
                life: 1.0,
                max: 1.0,
                color: GOLD,
                drag: 1.0,
                grav: 0.0,
                flags: ROCKET,
                shell,
            };
            f.burst(&r);
            assert!(f.born.len() > 80, "{shell:?} {}", f.born.len());
            assert!(
                f.born
                    .iter()
                    .all(|p| p.drag == 0.93 && p.grav == SPARK_G * 0.3)
            );
        }
    }

    /// The gran final is a calavera between two marigolds, or just the
    /// calavera on a narrow screen.
    #[test]
    fn gran_final_is_a_calavera() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(11, 2));
        f.born.clear();
        f.gran_final();
        let shells: Vec<Shell> = f.born.iter().map(|p| p.shell).collect();
        assert_eq!(shells, [Shell::Calavera, Shell::Marigold, Shell::Marigold]);
        assert!(f.born.iter().all(|p| p.flags & ROCKET != 0));
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::of_day(11, 2),
        };
        f.resize(&ctx, 20, 12);
        f.born.clear();
        f.gran_final();
        assert_eq!(f.born.len(), 1);
    }

    /// Navidad turns the city into a plaza with a kiosco, lights and pines,
    /// and bursts estrellas, nochebuenas and bells, from its first day.
    #[test]
    fn navidad_dresses_up() {
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (m, d) in [(12, 24), (12, 12), (1, 6)] {
                let fiesta = Fiesta::of_day(m, d);
                let (mut f, o) = make("", &theme);
                holiday(&mut f, &theme, &o, fiesta);
                assert!(f.town.contains(&1), "{m}-{d}");
                assert!(!f.bulbs.is_empty() && !f.candles.is_empty(), "{m}-{d}");
                let (shells, _) = watch(&mut f, 3000);
                for s in [Shell::Estrella, Shell::Nochebuena, Shell::Campana] {
                    assert!(shells.contains(&s), "{m}-{d} {shells:?}");
                }
                // The ground is drawn all along the bottom.
                let ctx = Ctx {
                    theme: &theme,
                    opts: &o,
                    bg: Color::Reset,
                    seed: 3,
                    fiesta,
                };
                let mut c = Canvas::new(120, 36, Color::Reset);
                f.draw(&ctx, &mut c);
                assert!(c.cells()[35 * 120..].iter().all(|cell| cell.ch != ' '));
            }
        }
    }

    /// The lights, the strings and the shapes grow through the posadas, and
    /// the plaza comes and goes with the holiday.
    #[test]
    fn navidad_comes_and_goes() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(12, 12));
        let (first, wires) = (f.bulbs.len(), f.town.iter().filter(|&&t| t == 1).count());
        holiday(&mut f, &theme, &o, Fiesta::of_day(12, 24));
        assert!(f.bulbs.len() > first, "{first} {}", f.bulbs.len());
        assert!(f.town.iter().filter(|&&t| t == 1).count() > wires);
        holiday(&mut f, &theme, &o, Fiesta::NONE);
        assert!(f.town.is_empty() && f.bulbs.is_empty() && f.candles.is_empty());
        // Another holiday leaves it alone, and so does a missing skyline.
        holiday(&mut f, &theme, &o, Fiesta::of_day(5, 3));
        assert!(f.town.is_empty() && f.bulbs.is_empty());
        let (mut f, o) = make("skyline = false", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(12, 24));
        assert!(f.town.is_empty() && f.bulbs.is_empty());
    }

    /// The plaza fits any size: no light is outside the screen, and the
    /// kiosco only stands where there is room for it.
    #[test]
    fn plaza_fits_any_size() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::of_day(12, 24),
        };
        for (w, h) in [
            (1, 1),
            (3, 6),
            (20, 8),
            (44, 14),
            (80, 24),
            (200, 60),
            (500, 150),
        ] {
            f.resize(&ctx, w, h);
            f.retheme(&ctx);
            let (wi, hi) = (w as i32, h as i32 * 2);
            assert!(f.town.is_empty() || f.town.len() == (wi * hi) as usize);
            assert!(
                f.bulbs
                    .iter()
                    .all(|b| b.x >= 0 && b.y >= 0 && b.x < wi && b.y < hi),
                "{w}x{h}"
            );
        }
        f.resize(&ctx, 80, 24);
        f.retheme(&ctx);
        // The lamps of the kiosco are candles on the town.
        assert!(f.candles.iter().filter(|c| c.power > 0.85).count() >= 4);
    }

    /// On Año Nuevo the show counts down from ten, then the finale writes
    /// the year's greeting; on the other days of the Navidad it does not.
    #[test]
    fn ano_nuevo_counts_down() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        let fiesta = Fiesta::of_day(12, 31);
        holiday(&mut f, &theme, &o, fiesta);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta,
        };
        let mut numbers = Vec::new();
        let mut written = false;
        let mut c = Canvas::new(120, 36, Color::Reset);
        for _ in 0..3000 {
            f.step();
            if f.count > 0 {
                let n = (f.count + 29) / 30;
                if numbers.last() != Some(&n) {
                    numbers.push(n);
                }
                // The number is drawn in light.
                f.draw(&ctx, &mut c);
                assert!(f.light.iter().any(|l| l[0] > 0.3));
            }
            written |=
                f.ps.iter()
                    .any(|p| p.flags & ROCKET != 0 && p.shell == Shell::Text(FELIZ_ANO));
        }
        // It comes again every so often.
        assert!(
            numbers.len() >= 10 && numbers.len() % 10 == 0,
            "{numbers:?}"
        );
        assert!(
            numbers
                .chunks(10)
                .all(|c| c == [10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
        );
        assert!(written);
        for day in [(12, 24), (12, 25), (1, 6)] {
            let (mut f, o) = make("", &theme);
            holiday(&mut f, &theme, &o, Fiesta::of_day(day.0, day.1));
            let mut counted = false;
            let mut wrote = false;
            for _ in 0..3000 {
                f.step();
                counted |= f.count > 0;
                wrote |= f.ps.iter().any(|p| matches!(p.shell, Shell::Text(_)));
            }
            assert!(!counted && !wrote, "{day:?}");
        }
    }

    /// The greeting is written in a font that has every one of its letters,
    /// and it fits a screen only when it can be read.
    #[test]
    fn greeting_is_legible() {
        for ch in TEXTS[0].chars().filter(|&c| c != ' ') {
            assert!(glyph(ch).0.iter().any(|&r| r != 0), "{ch}");
        }
        for n in '0'..='9' {
            assert!(glyph(n).0.iter().any(|&r| r != 0), "{n}");
        }
        let fit = fit_text(TEXTS[0], 100.0, 60.0).unwrap();
        assert_eq!(fit.count, 2);
        assert!(fit.cell >= 0.9 && fit.cols as f32 * fit.cell <= 100.0);
        assert!(fit.rows() as f32 * fit.cell <= 60.0);
        // On a wide, short screen it goes on one line.
        assert_eq!(fit_text(TEXTS[0], 300.0, 24.0).unwrap().count, 1);
        assert!(fit_text(TEXTS[0], 20.0, 12.0).is_none());
        assert!(fit_text(TEXTS[0], 0.0, 0.0).is_none());
    }

    /// The pixel pictures are rectangles.
    #[test]
    fn pictures_are_rectangles() {
        for art in [KIOSCO, ABETO, NOCHEBUENA_POT] {
            let w = art[0].chars().count();
            assert!(art.iter().all(|r| r.chars().count() == w), "{art:?}");
        }
    }

    /// The Navidad's gran final is an estrella between two bells, or just
    /// the estrella on a narrow screen; on Año Nuevo it is the greeting.
    #[test]
    fn gran_final_is_a_star() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(12, 24));
        f.born.clear();
        f.gran_final();
        let shells: Vec<Shell> = f.born.iter().map(|p| p.shell).collect();
        assert_eq!(shells, [Shell::Estrella, Shell::Campana, Shell::Campana]);
        holiday(&mut f, &theme, &o, Fiesta::of_day(1, 1));
        f.born.clear();
        f.gran_final();
        let shells: Vec<Shell> = f.born.iter().map(|p| p.shell).collect();
        assert_eq!(shells, [Shell::Text(FELIZ_ANO)]);
        // Too small to read it, it is the star.
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::of_day(1, 1),
        };
        f.resize(&ctx, 20, 12);
        f.born.clear();
        f.gran_final();
        assert_eq!(f.born.len(), 1);
        assert_eq!(f.born[0].shell, Shell::Estrella);
    }

    /// With the sky transparent nothing is painted before the first launch,
    /// and most of the screen stays empty afterwards.
    #[test]
    fn sky_is_transparent() {
        let theme = Theme::classic();
        let (mut f, o) = make("skyline = false\nstars = false", &theme);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::NONE,
        };
        let mut c = Canvas::new(80, 24, Color::Reset);
        f.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch == ' '));
        let mut painted = 0;
        for _ in 0..600 {
            f.update(&ctx, 1.0 / 30.0);
            c.clear();
            f.draw(&ctx, &mut c);
            painted = painted.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
        }
        assert!(painted > 0);
        assert!(painted < 80 * 24 / 2);
    }

    /// `sky = true` paints every cell, and the city is on by default.
    #[test]
    fn painted_sky_and_city() {
        let theme = Theme::classic();
        let (mut f, o) = make("sky = true", &theme);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::NONE,
        };
        let mut c = Canvas::new(80, 24, Color::Reset);
        f.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch != ' '));
        assert!(!f.windows.is_empty());
    }

    /// Rockets burst into hundreds of sparks, within the particle cap.
    #[test]
    fn bursts_happen() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::NONE,
        };
        let mut peak = 0;
        for _ in 0..900 {
            f.update(&ctx, 1.0 / 30.0);
            peak = peak.max(f.ps.len());
        }
        assert!(peak > 60, "peak {peak}");
        assert!(f.ps.len() <= MAX_PARTICLES);
    }
}
