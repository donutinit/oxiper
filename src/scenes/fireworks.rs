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

use std::f32::consts::TAU;

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
    // Día de Muertos.
    seed: u64,
    fiesta: Fiesta,
    /// The holiday's own random numbers (shell shapes and colors).
    frng: fastrand::Rng,
    dress: Dress,
    /// The panteón that stands in for the city, per pixel: 0 is sky, 1 a
    /// grave, 2 a candle flame; empty when there is none.
    tomb: Vec<u8>,
    tomb_col: Vec<V3>,
    candles: Vec<Candle>,
    /// Steps until the gran final, or 0.
    gran: i32,
    /// Where copal burns (pixel x and y).
    copal: Vec<(i32, i32)>,
    /// Candlelight falling on the graves this frame.
    warm: Vec<f32>,
}

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
            tomb: Vec::new(),
            tomb_col: Vec::new(),
            candles: Vec::new(),
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

    /// Día de Muertos ends a finale with a calavera between two marigolds
    /// (just the calavera when there is no room for them).
    fn gran_final(&mut self) {
        let (w, hh) = (self.w as f32, self.ph as f32);
        let r = w.min(hh) * SHAPE_SIZE.1;
        let apex = (r * 1.15 + 2.0).max(hh * 0.4).min(hh * 0.6);
        let vy = -(2.0 * ROCKET_G * (hh - apex).max(1.0)).sqrt();
        let shells: &[(f32, Shell)] = if w > r * 6.0 {
            &[
                (0.5, Shell::Calavera),
                (0.2, Shell::Marigold),
                (0.8, Shell::Marigold),
            ]
        } else {
            &[(0.5, Shell::Calavera)]
        };
        for &(x, shell) in shells {
            self.born.push(P {
                x: w * x,
                y: hh,
                vx: 0.0,
                // The marigolds burst a moment after the calavera.
                vy: if shell == Shell::Calavera {
                    vy
                } else {
                    vy * 0.97
                },
                life: 1000.0,
                max: 1000.0,
                color: self.dress.bone,
                drag: 1.0,
                grav: ROCKET_G,
                flags: ROCKET,
                shell,
            });
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
        if matches!(r.shell, Shell::Calavera | Shell::Marigold) {
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
        let pal = &self.pal;
        let dress = &self.dress;
        let gold = if self.fiesta.is(Kind::Muertos) {
            dress.gold
        } else {
            pal.gold
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
                        pal.shell_color(rng)
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
                let second = pal.shell_color(rng);
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
            Shell::Calavera | Shell::Marigold => {}
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
        let d = &self.dress;
        let (strokes, colors): (&[Stroke], [V3; 5]) = if r.shell == Shell::Calavera {
            (
                CALAVERA,
                [d.bone, d.icing[0], d.icing[1], d.icing[2], d.bone],
            )
        } else {
            (
                MARIGOLD,
                [d.mari[0], d.mari[1], d.mari[2], d.mari[3], d.leaf],
            )
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
                    color: colors[color as usize % 5],
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

    /// Lays out the panteón that stands in for the city during Día de
    /// Muertos. The layout depends only on the seed and the size; the level
    /// decides how many graves are dressed and how many candles are lit.
    fn build_panteon(&mut self) {
        self.tomb.clear();
        self.tomb_col.clear();
        self.candles.clear();
        self.copal.clear();
        let (w, ph) = (self.w, self.ph);
        if !self.fiesta.is(Kind::Muertos) || !self.show_city || w == 0 || ph < 12 {
            return;
        }
        self.tomb.resize(w * ph, 0);
        self.tomb_col.resize(w * ph, [0.0; 3]);
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
                self.tomb[i] = 1;
                let (petal, shade) = (rng.f32() < 0.35 * level, rng.usize(0..3));
                self.tomb_col[i] = match y {
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
                self.tomb[i] = 2;
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
                            self.tomb[i] = 1;
                            self.tomb_col[i] = if ch == b'+' {
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
                                self.tomb[i] = 2;
                                self.candles.push(Candle {
                                    x: px,
                                    y: py,
                                    phase,
                                    power: 1.0,
                                });
                            }
                            continue;
                        }
                        b'v' if py > 0 && self.tomb[i - w] == 2 => d.glass[px as usize % 3],
                        _ => continue,
                    };
                    self.tomb[i] = 1;
                    // Marigolds and candle glass are drawn a little dim:
                    // the candles light them up.
                    self.tomb_col[i] = match ch {
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
            self.finale = self.rng.i32(90..150);
            self.until_finale = self.rng.i32(900..1600);
        }
        self.timer -= 1;
        if self.timer <= 0 {
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
                if self.fiesta.is(Kind::Muertos) && self.rate > 0.0 {
                    // Once the last of the finale has faded.
                    self.gran = 45;
                }
            }
        }
        if self.gran > 0 {
            self.gran -= 1;
            if self.gran == 0 && self.fiesta.is(Kind::Muertos) {
                self.gran_final();
            }
        }
        if !self.tomb.is_empty() {
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
        self.build_panteon();
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
        self.build_panteon();
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
        // Día de Muertos: a panteón stands where the city was.
        let panteon = !self.tomb.is_empty();
        if panteon {
            self.light_candles();
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
                    self.tomb[i] == 0
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
                    let t = self.tomb_col[i];
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
                    self.tomb[i] == 0
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
                for fiesta in [Fiesta::NONE, Fiesta::of_day(11, 2)] {
                    let (mut f, o) = make(v, &theme);
                    let ctx = Ctx {
                        theme: &theme,
                        opts: &o,
                        bg: Color::Reset,
                        seed: 3,
                        fiesta,
                    };
                    // The holiday has more to draw: fewer frames of it do.
                    let frames = if fiesta.is(Kind::Muertos) { 200 } else { 400 };
                    for (w, h) in [(80, 24), (1, 1), (2, 3), (7, 2), (300, 90)] {
                        f.resize(&ctx, w, h);
                        f.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
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
                assert!(f.tomb.contains(&1), "{m}-{d}");
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
        assert!(f.tomb.is_empty() && f.candles.is_empty() && f.copal.is_empty());
        let (shells, wisps) = watch(&mut f, 3000);
        assert!(!wisps);
        for s in [Shell::Calavera, Shell::Marigold, Shell::Petals] {
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
        let (first, tomb) = (f.candles.len(), f.tomb.clone());
        holiday(&mut f, &theme, &o, Fiesta::of_day(11, 2));
        assert!(f.candles.len() > first, "{first} {}", f.candles.len());
        // Arches come with the dressing; nothing that stood is gone.
        assert!(tomb.iter().zip(&f.tomb).all(|(a, b)| *a == 0 || *b != 0));
        holiday(&mut f, &theme, &o, Fiesta::NONE);
        assert!(f.tomb.is_empty() && f.candles.is_empty());
        // Without the skyline there is no panteón either.
        let (mut f, o) = make("skyline = false", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(11, 2));
        assert!(f.tomb.is_empty());
    }

    /// A shaped shell's sparks share drag and gravity, so it keeps its shape.
    #[test]
    fn shapes_hold_together() {
        let theme = Theme::classic();
        let (mut f, o) = make("", &theme);
        holiday(&mut f, &theme, &o, Fiesta::of_day(11, 2));
        for shell in [Shell::Calavera, Shell::Marigold] {
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
            assert!(f.born.len() > 80, "{}", f.born.len());
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
