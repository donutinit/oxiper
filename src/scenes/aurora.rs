//! aurora: northern lights over mountains, mirrored in a still lake.
//!
//! Adapted from tanim's `aurora` <https://github.com/SantiagoSchez/tanim>,
//! licensed MIT. An aurora is a thin sheet of light seen almost edge on, so
//! each curtain is a wavy line (its track across the sky) walked in small
//! steps, every sample dropping light into the column it lands on. Where the
//! line folds back on itself many samples pile into the same columns, which is
//! where real curtains look brightest. Each column then glows upwards from the
//! curtain's sharp lower edge. The ridges, the pine forest, the stars, the
//! shooting stars, the substorms and the lake follow tanim; the night sky and
//! the lake are transparent here by default (`sky = true` paints tanim's).
//!
//! On top of tanim's night the sky changes its look every so often, fading
//! from one to the next: tanim's own green, teal and violet, an emerald
//! night, a crimson storm, ultraviolet, ice under a crescent moon, synthwave
//! with neon ridges, embers, a prism whose curtains run through every hue,
//! and now and then Van Gogh's starry night, with the sky whirling in
//! brushstrokes, ringed stars, a crescent moon, a cypress and a village. Now
//! and then Nyan Cat or sl's locomotive goes by (the easter eggs, sent by the
//! scene itself).
//!
//! On Día de Muertos the night moves to Lake Pátzcuaro, in Michoacán: the
//! curtains burn cempasúchil gold into rosa mexicano and morado over violet
//! mountains, Janitzio rises from the lake with candles on its terraces and
//! Morelos floodlit on its summit, canoes glide by with their butterfly nets
//! raised like wings and veladoras at bow and stern, more veladoras float on
//! the water, every flame trailing its streak down the lake, and monarchs
//! cross the sky on their way south. The cycle shows Pátzcuaro most of the
//! time while the holiday lasts (nearly always on November 2), more candles,
//! canoes, veladoras and monarchs as the holiday comes, and the holiday's own
//! visitors stop by most of the time; a fixed `look` wins, and
//! `look = "patzcuaro"` shows it on any day. Its randomness has its own RNG,
//! so ordinary days stay exactly as they were.
//!
//! During the Navidad the night moves to the desert hills of the nacimiento:
//! the little town of Belén lit on its hills with the portal at its heart,
//! the Estrella de Belén hanging over it with a long tail, pastores around
//! their fogata with their sheep, palms and dunes under the stars, and the
//! Reyes Magos on horse, camel and elephant crossing the sand, a little
//! closer each day of January until they reach the portal on the 6th. More
//! windows are lit and the star burns brighter as the posadas go, the Niño
//! Dios lies in the manger from Nochebuena on, and the cycle shows Belén
//! most of the time; `look = "belen"` shows it on any day.
//!
//! During the Fiestas Patrias the night moves to the valley of México, under
//! its volcanoes: Popocatépetl, a snowcapped cone with its crater glowing red
//! and a little smoke leaning in the wind, and Iztaccíhuatl, the sleeping
//! woman lying on her back from her head to her feet, snowcapped too, with
//! the curtains in green, white and red over them. Garlands of lights in the
//! flag's colors hang along the shore and a bandera waves from a pole, all
//! mirrored in the lake, and fireworks burst over the volcanoes: one now and
//! then at the start of the mes patrio and bursts all over the sky by the
//! night of the Grito. `look = "volcanes"` shows it on any day.
//!
//! On the spring equinox the night moves to Teotihuacán at dawn: the sun
//! rises behind the Pirámide del Sol, whose stairs and terraces stand dark
//! against the light with a gold rim, the Calzada de los Muertos runs to its
//! foot between low temple platforms, and people in white, their arms raised
//! to the sun, fill the avenue, the stairs and the summit, the raised arms
//! rolling through the crowd like a wave. The curtains burn in the sun's
//! gold, orange, sky blue and jade. More people have come on the 20th and
//! 21st than on the 19th and 22nd, when the sun also rises a little to one
//! side of the summit. `look = "teotihuacan"` shows it on any day.
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

use std::f32::consts::{PI, TAU};

use crate::color::{Color, ansi_rgb};
use crate::eggs::{self, Egg};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use crate::theme::Theme;

pub const INFO: SceneInfo = SceneInfo {
    name: "aurora",
    aliases: &["northern-lights", "borealis"],
    about: "northern lights over mountains, mirrored in a lake, after tanim",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast the lights ripple and drift (0.1-4)",
        },
        Opt {
            key: "curtains",
            default: Val::I(4),
            help: "Curtains of light in the sky at once (1-8)",
        },
        Opt {
            key: "look",
            default: Val::S("cycle"),
            help: "cycle, or one look to stay on: tanim, emerald, crimson, ultraviolet, ice, synthwave, ember, prism, starry, patzcuaro (Día de Muertos), belen (Navidad), volcanes (Fiestas Patrias), teotihuacan (Equinoccio)",
        },
        Opt {
            key: "stay",
            default: Val::F(90.0),
            help: "Seconds each look lasts while cycling (10-3600)",
        },
        Opt {
            key: "landscape",
            default: Val::B(true),
            help: "Mountains, a pine forest and the lake that mirrors the sky",
        },
        Opt {
            key: "stars",
            default: Val::B(true),
            help: "Twinkling stars and the odd shooting star",
        },
        Opt {
            key: "visitors",
            default: Val::F(3.0),
            help: "Times per hour Nyan Cat or sl's train goes by, 0 = never",
        },
        Opt {
            key: "sky",
            default: Val::B(false),
            help: "Paint the night sky and the lake instead of leaving them transparent",
        },
    ],
    make: |ctx| Box::new(Aurora::new(ctx)),
};

/// Color as red, green, blue in 0..1 (more than 1 is allowed while adding light).
type V3 = [f32; 3];

const fn c8(r: u8, g: u8, b: u8) -> V3 {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}

/// How much brighter the land is drawn over a transparent sky, which is
/// usually lighter than tanim's near-black one.
const LIFT: f32 = 1.8;
/// tanim's tick rate for this animation; one simulation step is this long.
const STEP: f32 = 1.0 / 20.0;
/// Steps per pixel in a curtain's glow profile.
const PROFILE_RES: f32 = 8.0;
/// Seconds one look takes to fade into the next.
const BLEND: f32 = 12.0;
/// A visitor still crossing after this long is sent away.
const VISIT_SECS: f32 = 40.0;

/// One look of the night: its colors, and how much each effect shows.
#[derive(Clone, Copy)]
struct Look {
    /// Aurora colors from the lower edge upwards.
    glow: [V3; 4],
    sky_top: V3,
    sky_low: V3,
    far: V3,
    near: V3,
    snow: V3,
    pine: V3,
    water: V3,
    star: V3,
    glint: V3,
    /// A bright outline along the far ridge.
    rim: V3,
    rim_k: f32,
    /// A crescent moon high on the right.
    moon: V3,
    moon_k: f32,
    /// How bright the curtains are, how much they run through every hue, and
    /// how much of Van Gogh's starry night shows.
    lights: f32,
    prism: f32,
    starry: f32,
    /// How much of Pátzcuaro's Día de Muertos night shows: Janitzio and its
    /// candles, the butterfly-net canoes and the monarchs.
    muertos: f32,
    /// How much of the pine forest on the near shore shows (the desert has
    /// none), and how much of Belén's Navidad night: the town, the star, the
    /// dunes and the Reyes Magos.
    forest: f32,
    belen: f32,
    /// How much of the Fiestas Patrias night shows: the volcanoes with their
    /// smoke, the garlands, the flag and the fireworks; and how far the
    /// curtains' colors reach up the sky (a tricolor needs less room).
    volcanes: f32,
    reach: f32,
    /// How much of the Equinoccio's dawn at Teotihuacán shows: the rising
    /// sun, the Pirámide del Sol, the Calzada de los Muertos and the people
    /// in white.
    teotihuacan: f32,
}

/// tanim's night.
const TANIM: Look = Look {
    glow: [
        c8(90, 255, 150),
        c8(40, 220, 170),
        c8(120, 90, 230),
        c8(210, 60, 120),
    ],
    sky_top: c8(2, 3, 12),
    sky_low: c8(10, 20, 36),
    far: c8(18, 24, 42),
    near: c8(8, 11, 20),
    snow: c8(58, 70, 92),
    pine: c8(4, 6, 10),
    water: c8(3, 7, 16),
    star: c8(230, 235, 255),
    glint: c8(150, 190, 220),
    rim: c8(0, 0, 0),
    rim_k: 0.0,
    moon: c8(240, 236, 214),
    moon_k: 0.0,
    lights: 1.0,
    prism: 0.0,
    starry: 0.0,
    muertos: 0.0,
    forest: 1.0,
    belen: 0.0,
    volcanes: 0.0,
    reach: 1.0,
    teotihuacan: 0.0,
};

/// The looks the cycle picks from on an ordinary day: all but the holidays'.
const ORDINARY: usize = 9;
/// The holidays' places in `LOOKS`: Pátzcuaro's, Belén's, the volcanoes' and
/// Teotihuacán's.
const PATZCUARO: usize = 9;
const BELEN: usize = 10;
const VOLCANES: usize = 11;
const TEOTIHUACAN: usize = 12;

const LOOKS: [(&str, Look); 13] = [
    ("tanim", TANIM),
    (
        "emerald",
        Look {
            glow: [
                c8(140, 255, 150),
                c8(60, 235, 110),
                c8(30, 170, 110),
                c8(200, 90, 170),
            ],
            sky_low: c8(6, 24, 30),
            moon_k: 0.7,
            ..TANIM
        },
    ),
    (
        "crimson",
        Look {
            glow: [
                c8(255, 110, 120),
                c8(235, 45, 90),
                c8(160, 30, 120),
                c8(90, 20, 100),
            ],
            sky_top: c8(6, 2, 10),
            sky_low: c8(32, 8, 26),
            far: c8(36, 18, 34),
            near: c8(14, 6, 14),
            snow: c8(96, 64, 84),
            water: c8(10, 3, 10),
            glint: c8(230, 140, 160),
            ..TANIM
        },
    ),
    (
        "ultraviolet",
        Look {
            glow: [
                c8(245, 150, 255),
                c8(175, 95, 255),
                c8(95, 70, 235),
                c8(45, 40, 170),
            ],
            sky_top: c8(4, 2, 14),
            sky_low: c8(22, 10, 44),
            far: c8(30, 20, 56),
            near: c8(12, 8, 24),
            snow: c8(90, 76, 130),
            glint: c8(200, 160, 255),
            ..TANIM
        },
    ),
    (
        "ice",
        Look {
            glow: [
                c8(225, 250, 255),
                c8(145, 222, 255),
                c8(85, 160, 240),
                c8(60, 90, 200),
            ],
            sky_top: c8(2, 8, 20),
            sky_low: c8(18, 38, 64),
            far: c8(40, 60, 92),
            near: c8(16, 26, 42),
            snow: c8(176, 204, 232),
            pine: c8(8, 14, 24),
            water: c8(6, 14, 28),
            glint: c8(210, 235, 255),
            rim: c8(180, 220, 255),
            rim_k: 0.35,
            moon: c8(236, 246, 255),
            moon_k: 1.0,
            ..TANIM
        },
    ),
    (
        "synthwave",
        Look {
            glow: [
                c8(0, 255, 240),
                c8(80, 180, 255),
                c8(255, 60, 200),
                c8(255, 40, 120),
            ],
            sky_top: c8(10, 0, 24),
            sky_low: c8(64, 10, 74),
            far: c8(42, 10, 62),
            near: c8(18, 4, 32),
            snow: c8(42, 10, 62),
            pine: c8(10, 2, 20),
            water: c8(8, 0, 22),
            star: c8(255, 200, 250),
            glint: c8(255, 90, 220),
            rim: c8(255, 60, 220),
            rim_k: 1.0,
            moon: c8(255, 190, 80),
            moon_k: 0.9,
            ..TANIM
        },
    ),
    (
        "ember",
        Look {
            glow: [
                c8(255, 205, 90),
                c8(255, 135, 40),
                c8(220, 60, 40),
                c8(120, 30, 70),
            ],
            sky_top: c8(6, 3, 6),
            sky_low: c8(32, 16, 12),
            far: c8(42, 26, 22),
            near: c8(16, 10, 10),
            snow: c8(110, 84, 70),
            water: c8(10, 5, 4),
            glint: c8(255, 170, 90),
            ..TANIM
        },
    ),
    (
        "prism",
        Look {
            prism: 1.0,
            ..TANIM
        },
    ),
    (
        "starry",
        Look {
            glow: [
                c8(250, 240, 170),
                c8(120, 190, 220),
                c8(60, 110, 190),
                c8(30, 60, 140),
            ],
            sky_top: c8(14, 30, 80),
            sky_low: c8(40, 70, 130),
            far: c8(40, 62, 112),
            near: c8(18, 32, 46),
            snow: c8(64, 88, 136),
            pine: c8(10, 22, 18),
            water: c8(10, 24, 60),
            star: c8(250, 230, 130),
            glint: c8(250, 220, 120),
            lights: 0.35,
            starry: 1.0,
            ..TANIM
        },
    ),
    (
        "patzcuaro",
        Look {
            glow: [
                c8(255, 176, 40),
                c8(240, 60, 110),
                c8(228, 0, 124),
                c8(120, 40, 170),
            ],
            sky_top: c8(8, 6, 22),
            sky_low: c8(34, 16, 48),
            far: c8(40, 24, 58),
            near: c8(16, 10, 26),
            snow: c8(40, 24, 58),
            pine: c8(8, 4, 14),
            water: c8(8, 6, 24),
            star: c8(255, 236, 200),
            glint: c8(255, 170, 70),
            rim: c8(228, 0, 124),
            rim_k: 0.3,
            muertos: 1.0,
            ..TANIM
        },
    ),
    (
        "belen",
        Look {
            // Dorado at the lower edge, plata, verde pino and a breath of
            // rojo nochebuena at the top, over a deep blue night.
            glow: [
                c8(242, 182, 50),
                c8(201, 204, 214),
                c8(46, 150, 84),
                c8(200, 16, 46),
            ],
            sky_top: c8(4, 6, 22),
            sky_low: c8(22, 26, 60),
            far: c8(40, 30, 28),
            near: c8(24, 17, 14),
            snow: c8(60, 45, 42),
            pine: c8(24, 17, 14),
            water: c8(6, 8, 20),
            star: c8(255, 244, 214),
            glint: c8(242, 182, 50),
            rim: c8(242, 182, 50),
            rim_k: 0.25,
            forest: 0.0,
            belen: 1.0,
            ..TANIM
        },
    ),
    (
        "volcanes",
        Look {
            // Verde bandera at the lower edge, blanco, then rojo: the flag
            // squeezed into the lower half of each curtain.
            glow: [
                c8(0, 200, 136),
                c8(244, 241, 234),
                c8(206, 17, 38),
                c8(130, 10, 30),
            ],
            reach: 0.5,
            sky_top: c8(4, 8, 18),
            sky_low: c8(16, 26, 44),
            far: c8(30, 38, 50),
            near: c8(14, 20, 26),
            snow: c8(54, 62, 78),
            pine: c8(6, 14, 12),
            water: c8(4, 10, 20),
            star: c8(244, 241, 234),
            glint: c8(244, 241, 234),
            rim: c8(206, 17, 38),
            rim_k: 0.3,
            volcanes: 1.0,
            ..TANIM
        },
    ),
    (
        "teotihuacan",
        Look {
            // The dawn: the sun's gold at the lower edge, orange, cielo and
            // jade.
            glow: [
                c8(255, 207, 74),
                c8(255, 140, 80),
                c8(126, 200, 227),
                c8(46, 139, 87),
            ],
            sky_top: c8(14, 18, 44),
            sky_low: c8(120, 72, 84),
            far: c8(50, 40, 40),
            near: c8(30, 22, 22),
            snow: c8(75, 60, 60),
            pine: c8(30, 22, 22),
            water: c8(30, 24, 30),
            star: c8(255, 240, 210),
            glint: c8(255, 207, 74),
            rim: c8(255, 207, 74),
            rim_k: 0.5,
            forest: 0.0,
            teotihuacan: 1.0,
            ..TANIM
        },
    ),
];

/// Van Gogh's brushstrokes, from the deepest blue to the palest yellow.
const STROKES: [V3; 6] = [
    c8(10, 22, 70),
    c8(24, 52, 130),
    c8(46, 92, 170),
    c8(80, 140, 190),
    c8(150, 190, 200),
    c8(225, 225, 170),
];
/// Rings around the stars and the moon, and their cores.
const STAR_RINGS: [V3; 2] = [c8(250, 230, 120), c8(200, 222, 210)];
const MOON_RINGS: [V3; 2] = [c8(255, 190, 60), c8(250, 230, 140)];
const STAR_CORE: V3 = c8(255, 250, 210);
const MOON_CORE: V3 = c8(255, 215, 80);
/// The cypress in its two strokes, and the village: walls, roofs, windows.
const CYPRESS: [V3; 2] = [c8(6, 16, 12), c8(34, 58, 36)];
const VILLAGE: [V3; 3] = [c8(46, 62, 96), c8(28, 36, 64), c8(250, 210, 90)];

impl Look {
    fn lerp(&self, o: &Look, t: f32) -> Look {
        let m = |a: V3, b: V3| mix(a, b, t);
        let f = |a: f32, b: f32| a + (b - a) * t;
        Look {
            glow: std::array::from_fn(|i| m(self.glow[i], o.glow[i])),
            sky_top: m(self.sky_top, o.sky_top),
            sky_low: m(self.sky_low, o.sky_low),
            far: m(self.far, o.far),
            near: m(self.near, o.near),
            snow: m(self.snow, o.snow),
            pine: m(self.pine, o.pine),
            water: m(self.water, o.water),
            star: m(self.star, o.star),
            glint: m(self.glint, o.glint),
            rim: m(self.rim, o.rim),
            rim_k: f(self.rim_k, o.rim_k),
            moon: m(self.moon, o.moon),
            moon_k: f(self.moon_k, o.moon_k),
            lights: f(self.lights, o.lights),
            prism: f(self.prism, o.prism),
            starry: f(self.starry, o.starry),
            muertos: f(self.muertos, o.muertos),
            forest: f(self.forest, o.forest),
            belen: f(self.belen, o.belen),
            volcanes: f(self.volcanes, o.volcanes),
            reach: f(self.reach, o.reach),
            teotihuacan: f(self.teotihuacan, o.teotihuacan),
        }
    }

    /// This look in a theme's colors: the effects stay, the colors change.
    fn themed(&self, t: &Theme) -> Look {
        let g = &t.grad;
        let (low, mid, high) = (v3(g.at(0.0)), v3(g.at(0.5)), v3(g.at(1.0)));
        let bg = v3(t.bg);
        Look {
            glow: [v3(g.at(0.9)), v3(g.at(0.7)), v3(g.at(0.5)), v3(g.at(0.3))],
            sky_top: scale(bg, 0.6),
            sky_low: mix(bg, mid, 0.15),
            far: mix(low, mid, 0.25),
            near: scale(low, 0.6),
            snow: mix(low, high, 0.35),
            pine: scale(low, 0.3),
            water: scale(bg, 0.5),
            star: high,
            glint: mix(mid, high, 0.5),
            rim: high,
            moon: high,
            ..*self
        }
    }
}

/// The colors of the starry night and of the prism, from the theme.
struct Paint {
    strokes: [V3; 6],
    star_rings: [V3; 2],
    moon_rings: [V3; 2],
    star_core: V3,
    moon_core: V3,
    cypress: [V3; 2],
    village: [V3; 3],
    /// Colors the prism runs through; empty for every hue.
    prism: Vec<V3>,
}

impl Paint {
    fn new(t: &Theme) -> Paint {
        if t.native {
            return Paint {
                strokes: STROKES,
                star_rings: STAR_RINGS,
                moon_rings: MOON_RINGS,
                star_core: STAR_CORE,
                moon_core: MOON_CORE,
                cypress: CYPRESS,
                village: VILLAGE,
                prism: Vec::new(),
            };
        }
        let g = &t.grad;
        let at = |k: f32| v3(g.at(k));
        let mut prism: Vec<V3> = t.colors.iter().map(|&c| v3(c)).collect();
        if prism.is_empty() {
            prism = vec![at(0.4), at(0.7), at(1.0)];
        }
        Paint {
            strokes: std::array::from_fn(|i| {
                scale(at(0.05 + 0.17 * i as f32), 0.5 + 0.1 * i as f32)
            }),
            star_rings: [at(1.0), at(0.8)],
            moon_rings: [at(0.9), at(1.0)],
            star_core: at(1.0),
            moon_core: at(1.0),
            cypress: [scale(at(0.0), 0.5), mix(at(0.0), at(0.5), 0.3)],
            village: [mix(at(0.0), at(0.5), 0.4), at(0.0), at(1.0)],
            prism,
        }
    }
}

/// The colors of Pátzcuaro's night. Candle flames keep their own hue under
/// every theme; the rest comes from the theme.
#[derive(Clone, Copy)]
struct Fest {
    /// A flame's edge and core.
    flame: [V3; 2],
    /// Janitzio: the dark island, where candles warm it, and the statue.
    isle: [V3; 3],
    /// A butterfly net's hoop and its mesh.
    net: [V3; 2],
    /// A canoe's hull, its shadow side, the fisherman and a candle's wax.
    hull: [V3; 2],
    rower: V3,
    wax: V3,
    /// A monarch's wings (bright and dark) and its veins and body.
    wing: [V3; 2],
    vein: V3,
}

impl Fest {
    fn new(t: &Theme) -> Fest {
        let flame = [c8(255, 150, 40), c8(255, 214, 120)];
        if t.native {
            return Fest {
                flame,
                isle: [c8(8, 5, 12), c8(96, 44, 22), c8(214, 200, 176)],
                net: [c8(176, 156, 128), c8(150, 130, 104)],
                hull: [c8(84, 48, 28), c8(40, 22, 16)],
                rower: c8(22, 12, 20),
                wax: c8(244, 238, 226),
                wing: [c8(255, 140, 0), c8(214, 98, 0)],
                vein: c8(24, 12, 8),
            };
        }
        let at = |k: f32| v3(t.grad.at(k));
        Fest {
            flame,
            isle: [
                scale(at(0.0), 0.2),
                mix(at(0.0), at(0.8), 0.4),
                mix(at(0.5), at(1.0), 0.7),
            ],
            net: [mix(at(0.5), at(1.0), 0.7), mix(at(0.3), at(1.0), 0.4)],
            hull: [mix(at(0.0), at(0.5), 0.35), scale(at(0.0), 0.6)],
            rower: scale(at(0.0), 0.3),
            wax: mix(at(0.5), at(1.0), 0.8),
            wing: [at(1.0), at(0.75)],
            vein: scale(at(0.0), 0.3),
        }
    }
}

/// A Pátzcuaro canoe with its two butterfly nets raised like wings, and
/// candles at the bow and the stern. `o` hoop, `:` mesh, `f` fisherman,
/// `h`/`H` hull, `c` flame; the last row sits on the water.
const CANOE: [&str; 10] = [
    " oo         oo ",
    "o::oo     oo::o",
    "o::::o   o::::o",
    "o:::::o o:::::o",
    " o:::::o:::::o ",
    "  oo:::o:::oo  ",
    " c  ooofooo  c ",
    " w     f     w ",
    "hhhhhhhhhhhhhhh",
    " HHHHHHHHHHHHH ",
];
/// The same canoe farther away.
const CANOE_FAR: [&str; 6] = [
    "oo     oo",
    "o:o   o:o",
    "o::o o::o",
    " o::o::o ",
    "c  ofo  c",
    "hhhhhhhhh",
];
/// José María Morelos with his fist raised, on Janitzio's summit.
const STATUE: [&str; 7] = ["   S", " S S", "SSSS", " SS ", " SS ", " SS ", "SSSS"];
/// The statue on a smaller island.
const STATUE_SMALL: [&str; 4] = ["  S", " SS", " S ", "SSS"];
/// A monarch flapping: open, half, closed, half. `O`/`o` wing, `k` its
/// black veins, border and body.
const MONARCH: [[&str; 4]; 4] = [
    ["kO   Ok", "kOOkOOk", " oOkOo ", "  oko  "],
    ["       ", " kOkOk ", "  oko  ", "   k   "],
    ["       ", "  kOk  ", "   k   ", "   k   "],
    ["       ", " kOkOk ", "  oko  ", "   k   "],
];

/// One pixel of Janitzio: where, what (0 island, 1 candle, 2 statue, 3 its skyline), a
/// random number that decides when a candle is lit, and how much candlelight
/// warms it.
struct Isle {
    x: i32,
    y: i32,
    part: u8,
    r: f32,
    warm: f32,
}

struct Canoe {
    x: f32,
    /// The row it floats on.
    y: i32,
    vx: f32,
    far: bool,
    phase: f32,
    /// The canoes come out in this order as the holiday grows.
    rank: usize,
}

/// A veladora floating on the lake.
struct Float {
    x: f32,
    y: i32,
    vx: f32,
    phase: f32,
}

struct Monarch {
    x: f32,
    y: f32,
    vx: f32,
    /// How far it bobs, how fast, and where in the bob and the flap it is.
    bob: f32,
    rate: f32,
    phase: f32,
    flap: f32,
}

/// The share of looks that are the holiday's own while it is on: most of
/// them from the first day, nearly all at its height.
fn fiesta_share(level: f32) -> f32 {
    0.6 + 0.35 * level.clamp(0.0, 1.0)
}

/// The look a holiday moves the cycle to, for the holidays aurora has one for.
fn fiesta_look(kind: Kind) -> Option<usize> {
    match kind {
        Kind::Muertos => Some(PATZCUARO),
        Kind::Navidad => Some(BELEN),
        Kind::Patrias => Some(VOLCANES),
        Kind::Equinoccio => Some(TEOTIHUACAN),
        _ => None,
    }
}

/// The colors of Belén's night. Lamps, the fire and the Niño Dios keep their
/// own hue under every theme; the rest comes from the theme.
#[derive(Clone, Copy)]
struct Nacimiento {
    /// The sand, from the farthest dunes to the nearest.
    dune: [V3; 3],
    /// The town: walls in shade, and the parapets and domes the star lights.
    adobe: [V3; 2],
    /// A lit window and the Niño Dios' light.
    lamp: [V3; 2],
    /// The portal's roof and its straw.
    roof: V3,
    straw: V3,
    /// The Estrella de Belén: its heart and its tail.
    star: [V3; 2],
    /// A palm's fronds and trunk.
    palm: [V3; 2],
    /// A sheep's wool and a shepherd's cloak.
    wool: V3,
    cloak: V3,
    /// The Reyes: Melchor's, Gaspar's and Baltasar's robes; their crowns and
    /// gifts, the cloth under them, their skin and the elephant's tusks.
    robe: [V3; 3],
    gold: V3,
    cloth: V3,
    skin: V3,
    tusk: V3,
    /// The horse, the camel and the elephant.
    beast: [V3; 3],
}

impl Nacimiento {
    fn new(t: &Theme) -> Nacimiento {
        let lamp = [c8(255, 196, 84), c8(255, 226, 150)];
        if t.native {
            return Nacimiento {
                dune: [c8(58, 44, 40), c8(76, 53, 38), c8(100, 68, 40)],
                adobe: [c8(74, 52, 40), c8(130, 96, 64)],
                lamp,
                roof: c8(110, 84, 46),
                straw: c8(170, 146, 86),
                star: [c8(255, 248, 230), c8(242, 182, 50)],
                palm: [c8(31, 111, 58), c8(100, 60, 30)],
                wool: c8(228, 224, 214),
                cloak: c8(139, 74, 30),
                robe: [c8(200, 16, 46), c8(31, 111, 58), c8(201, 204, 214)],
                gold: c8(242, 182, 50),
                cloth: c8(180, 130, 40),
                skin: c8(150, 100, 70),
                tusk: c8(244, 241, 234),
                beast: [c8(120, 64, 28), c8(160, 138, 86), c8(130, 132, 140)],
            };
        }
        let at = |k: f32| v3(t.grad.at(k));
        let (low, mid, high, bg) = (at(0.0), at(0.5), at(1.0), v3(t.bg));
        Nacimiento {
            dune: [mix(bg, low, 0.5), mix(bg, mid, 0.3), mix(bg, mid, 0.5)],
            adobe: [mix(low, mid, 0.35), mix(mid, high, 0.5)],
            lamp,
            roof: scale(low, 0.6),
            straw: mix(mid, high, 0.6),
            star: [high, mix(mid, high, 0.5)],
            palm: [mix(low, mid, 0.5), scale(low, 0.6)],
            wool: mix(mid, high, 0.85),
            cloak: mix(low, mid, 0.5),
            robe: [at(1.0), at(0.5), at(0.75)],
            gold: high,
            cloth: at(0.4),
            skin: mix(mid, high, 0.4),
            tusk: high,
            beast: [
                scale(mix(low, mid, 0.3), 0.6),
                scale(mix(low, mid, 0.5), 0.7),
                scale(mix(low, mid, 0.4), 0.65),
            ],
        }
    }
}

/// One piece of a king or his mount, in units (a pixel at scale 1) with x
/// forward and y up from the feet (so negative).
#[derive(Clone, Copy)]
enum Part {
    /// A filled ellipse: center and radii.
    Ell(f32, f32, f32, f32),
    /// A thick line: from, to and thickness.
    Seg(f32, f32, f32, f32, f32),
    /// A leg swinging with the gait: hip, length, thickness and where in the
    /// step it is.
    Leg(f32, f32, f32, f32, f32),
}
use Part::{Ell, Leg, Seg};

/// What a part is painted in: the beast, the king's robe, gold, the cloth
/// under the king, skin, tusk, the ear.
const BEAST: u8 = 0;
const ROBE: u8 = 1;
const GOLD: u8 = 2;
const CLOTH: u8 = 3;
const SKIN: u8 = 4;
const TUSK: u8 = 5;
const EAR: u8 = 6;

/// Melchor on his horse, Gaspar on his camel and Baltasar on his elephant,
/// each crowned and carrying a gift.
const HORSE: [(Part, u8); 15] = [
    (Leg(-4.5, -6.0, 6.0, 1.2, 0.0), BEAST),
    (Leg(-3.0, -6.0, 6.0, 1.2, PI), BEAST),
    (Ell(0.0, -8.2, 6.4, 3.1), BEAST),
    (Seg(4.6, -9.6, 7.6, -14.0, 1.8), BEAST),
    (Seg(7.6, -14.2, 10.8, -12.8, 1.7), BEAST),
    (Seg(-6.0, -9.2, -8.8, -5.4, 0.9), BEAST),
    (Leg(3.5, -6.0, 6.0, 1.2, PI), BEAST),
    (Leg(5.0, -6.0, 6.0, 1.2, 0.0), BEAST),
    (Ell(0.0, -10.4, 3.4, 0.9), CLOTH),
    (Seg(-0.5, -12.4, -4.5, -10.2, 1.6), ROBE),
    (Ell(0.6, -12.6, 2.1, 2.3), ROBE),
    (Ell(0.8, -16.0, 1.2, 1.2), SKIN),
    (Ell(0.8, -17.4, 1.5, 0.8), GOLD),
    (Ell(3.2, -12.2, 0.9, 0.9), GOLD),
    (Ell(-0.8, -11.0, 1.0, 1.0), GOLD),
];
const CAMEL: [(Part, u8); 18] = [
    (Leg(-6.0, -7.5, 7.5, 1.3, 0.0), BEAST),
    (Leg(-3.8, -7.5, 7.5, 1.3, PI), BEAST),
    (Ell(0.0, -10.2, 7.6, 3.3), BEAST),
    (Ell(-1.2, -13.6, 2.6, 2.0), BEAST),
    (Seg(6.6, -11.6, 9.6, -15.8, 1.8), BEAST),
    (Seg(9.6, -15.8, 11.4, -17.4, 1.7), BEAST),
    (Seg(11.4, -17.2, 14.0, -16.6, 1.7), BEAST),
    (Seg(-7.4, -11.0, -8.6, -7.4, 0.9), BEAST),
    (Leg(4.6, -7.5, 7.5, 1.3, PI), BEAST),
    (Leg(6.6, -7.5, 7.5, 1.3, 0.0), BEAST),
    (Ell(-1.2, -15.0, 3.4, 1.1), CLOTH),
    (Seg(-2.6, -17.2, -6.5, -14.6, 1.4), ROBE),
    (Ell(-1.0, -17.4, 2.0, 2.3), ROBE),
    (Ell(-0.8, -20.6, 1.2, 1.2), SKIN),
    (Ell(-0.8, -22.0, 1.5, 0.8), GOLD),
    (Ell(1.8, -17.0, 0.9, 0.9), GOLD),
    (Ell(-1.6, -13.0, 0.9, 0.9), GOLD),
    (Ell(-4.0, -14.4, 0.7, 0.7), GOLD),
];
const ELEPHANT: [(Part, u8); 17] = [
    (Leg(-6.5, -6.0, 6.0, 2.6, 0.0), BEAST),
    (Leg(-3.6, -6.0, 6.0, 2.6, PI), BEAST),
    (Ell(0.0, -10.2, 9.0, 5.2), BEAST),
    (Ell(9.6, -11.4, 3.6, 4.2), BEAST),
    (Ell(7.0, -11.8, 2.0, 3.2), EAR),
    (Seg(12.2, -9.6, 13.8, -5.0, 2.2), BEAST),
    (Seg(13.8, -5.0, 13.0, -2.4, 1.5), BEAST),
    (Seg(11.8, -8.2, 14.6, -7.4, 0.9), TUSK),
    (Seg(-9.0, -10.5, -10.6, -5.2, 0.8), BEAST),
    (Leg(3.8, -6.0, 6.0, 2.6, PI), BEAST),
    (Leg(6.8, -6.0, 6.0, 2.6, 0.0), BEAST),
    (Ell(0.2, -14.6, 6.0, 1.3), CLOTH),
    (Ell(0.2, -17.2, 2.3, 2.2), ROBE),
    (Ell(0.4, -20.2, 1.2, 1.2), SKIN),
    (Ell(0.4, -21.6, 1.5, 0.8), GOLD),
    (Ell(3.0, -17.0, 0.9, 0.9), GOLD),
    (Ell(-3.2, -15.6, 0.8, 0.8), GOLD),
];

/// The portal de Belén: the stable's roof (`r`) and posts (`p`), the straw
/// (`h`), María (`m`), José (`j`) and the Niño Dios in the manger (`n`).
const PORTAL: [&str; 9] = [
    "      r      ",
    "     rrr     ",
    "    rrrrr    ",
    "   rrrrrrr   ",
    "  rrrrrrrrr  ",
    "p rrrrrrrrr p",
    "p  m  n  j  p",
    "p  m hhh j  p",
    "hhhhhhhhhhhhh",
];
/// The portal on a smaller screen.
const PORTAL_S: [&str; 5] = ["   r   ", "  rrr  ", "p m n j", "p mhhhj", "hhhhhhh"];

/// One pixel of Belén's town or portal: where, what (0 wall, 1 window, 2 a
/// lit parapet or dome, 4 roof, 5 post, 6 straw, 7 the Niño Dios, 8 José,
/// 9 María), and a random number that decides when a window is lit.
struct Spot {
    x: i32,
    y: i32,
    part: u8,
    r: f32,
}

/// A date palm on the dunes.
struct Palm {
    x: i32,
    y: i32,
    h: f32,
    lean: f32,
}

/// A sheep grazing by the shepherds' fire, and how it wanders.
struct Sheep {
    x: f32,
    y: i32,
    phase: f32,
}

/// The colors of the Fiestas Patrias night: the flag's green, white and red.
/// The crater's ember and its smoke keep their own hue under every theme;
/// the rest comes from the theme.
#[derive(Clone, Copy)]
struct Volcan {
    /// The volcanoes' rock and their snow, lit and in shade.
    rock: [V3; 2],
    snow: [V3; 2],
    /// The crater's ember, and the smoke near it and far from it.
    ember: V3,
    smoke: [V3; 2],
    /// Green, white and red as lights (garlands, fireworks) and as the
    /// flag's bands, with its eagle and nopal, and the pole.
    light: [V3; 3],
    flag: [V3; 3],
    eagle: V3,
    nopal: V3,
    pole: V3,
}

impl Volcan {
    fn new(t: &Theme) -> Volcan {
        let ember = c8(255, 96, 32);
        if t.native {
            return Volcan {
                rock: [c8(36, 44, 60), c8(16, 20, 30)],
                snow: [c8(204, 208, 220), c8(96, 104, 128)],
                ember,
                smoke: [c8(255, 150, 100), c8(140, 142, 156)],
                light: [c8(0, 168, 114), c8(244, 241, 234), c8(206, 17, 38)],
                flag: [c8(0, 104, 71), c8(244, 241, 234), c8(206, 17, 38)],
                eagle: c8(122, 74, 30),
                nopal: c8(79, 138, 42),
                pole: c8(150, 150, 156),
            };
        }
        let at = |k: f32| v3(t.grad.at(k));
        let (low, mid, high) = (at(0.0), at(0.5), at(1.0));
        let tri = [at(0.2), at(1.0), at(0.65)];
        Volcan {
            rock: [mix(low, mid, 0.35), scale(low, 0.5)],
            snow: [mix(mid, high, 0.8), mix(low, mid, 0.8)],
            ember,
            smoke: [c8(255, 150, 100), mix(low, mid, 0.7)],
            light: tri,
            flag: tri,
            eagle: mid,
            nopal: at(0.3),
            pole: mix(mid, high, 0.5),
        }
    }
}

/// The colors of Teotihuacán at the equinox. The sun keeps its own gold
/// under every theme; the rest comes from the theme.
#[derive(Clone, Copy)]
struct Piramide {
    /// The pyramid in contraluz, its stairs, and the gold the sun gives its
    /// edges.
    body: V3,
    stair: V3,
    rim: V3,
    /// The Calzada's paving, the ground around it, and the temple platforms
    /// along it: their lit tops and their sides.
    road: V3,
    ground: V3,
    platform: [V3; 2],
    /// The sun.
    sun: V3,
    /// The people in white and their faces, and the magueyes.
    robe: V3,
    skin: V3,
    jade: V3,
}

impl Piramide {
    fn new(t: &Theme) -> Piramide {
        let sun = c8(255, 207, 74);
        if t.native {
            // Piedra caliza, sombra, sol and jade.
            return Piramide {
                body: c8(66, 55, 42),
                stair: c8(140, 120, 88),
                rim: c8(224, 172, 60),
                road: c8(110, 96, 72),
                ground: c8(56, 46, 36),
                platform: [c8(150, 128, 94), c8(84, 70, 52)],
                sun,
                robe: c8(244, 241, 234),
                skin: c8(140, 92, 62),
                jade: c8(46, 139, 87),
            };
        }
        let at = |k: f32| v3(t.grad.at(k));
        let (low, mid, high, bg) = (at(0.0), at(0.5), at(1.0), v3(t.bg));
        Piramide {
            body: mix(low, mid, 0.3),
            stair: mix(mid, high, 0.2),
            rim: mix(mid, high, 0.8),
            road: mix(low, mid, 0.3),
            ground: mix(bg, low, 0.4),
            platform: [mix(mid, high, 0.2), scale(low, 0.8)],
            sun,
            robe: high,
            skin: mix(mid, high, 0.3),
            jade: at(0.3),
        }
    }
}

/// A person in white with arms raised to the sun, and with them down: `w`
/// the robe, `o` the head.
const PERSON: [[&str; 7]; 2] = [
    [
        "w...w", "w.o.w", ".www.", "..w..", "..w..", ".w.w.", ".w.w.",
    ],
    [
        "..o..", ".www.", ".www.", "..w..", "..w..", ".w.w.", ".w.w.",
    ],
];
/// The same, far away.
const PERSON_FAR: [[&str; 5]; 2] = [
    ["w.w", "wow", ".w.", ".w.", "w.w"],
    [".o.", "www", ".w.", ".w.", "w.w"],
];

/// Are the arms up? Most of the time, the raising rolling through the crowd
/// as a wave (`phase` goes with where they stand).
fn arms_up(t: f32, phase: f32) -> bool {
    (t * 0.03 + phase).sin() > -0.4
}

/// Someone at the equinox: where their feet are, whether they are near
/// enough to be drawn large, a random number that decides when they have
/// arrived, and where in the wave of raised arms they stand.
struct Person {
    x: i32,
    y: i32,
    big: bool,
    r: f32,
    phase: f32,
}

/// A low temple platform along the Calzada: its lower left corner, width and
/// height.
struct Platform {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

/// A maguey by the avenue: where it grows and how tall its leaves are.
struct Maguey {
    x: i32,
    y: i32,
    size: f32,
}

/// One pixel of a garland: where, what (0 to 2 a bulb of the flag's green,
/// white or red, 3 a pole), and a random number that decides when it is lit.
struct Bulb {
    x: i32,
    y: i32,
    c: u8,
    r: f32,
}

/// A burst of fireworks in the air: where, how old (seconds), how far its
/// sparks fly and the two colors (of the flag's three) they alternate.
struct Burst {
    x: f32,
    y: f32,
    age: f32,
    reach: f32,
    cols: (usize, usize),
}

/// How long a burst lasts.
const BURST_LIFE: f32 = 2.4;

/// How many fireworks slots are let off on a screen `w` wide when the mes
/// patrio has come `level` of the way: one at its start, all by the Grito.
fn firework_slots(w: i32, level: f32) -> usize {
    let slots = (w / 36).clamp(2, 7) as usize;
    1 + ((slots - 1) as f32 * level.clamp(0.0, 1.0)).round() as usize
}

/// What fireworks slot `slot` has in the air at `secs`, if any: each lets
/// one off every few seconds, where and in the colors its hash decides.
fn burst_of(slot: usize, secs: f32, w: i32, hz: i32, seed: u64) -> Option<Burst> {
    let period = 6.5 + 1.9 * slot as f32;
    let off = (hash(slot as i32, 7, seed) % 1000) as f32 / 1000.0 * period;
    let t = secs + off;
    let n = (t / period).floor();
    let age = t - n * period;
    if !(0.0..BURST_LIFE).contains(&age) {
        return None;
    }
    let h = hash(slot as i32, n as i32, seed ^ 0xF1AE);
    let u = |k: u32| ((h >> (k * 10)) & 1023) as f32 / 1023.0;
    let c = (h % 3) as usize;
    Some(Burst {
        x: (0.1 + 0.8 * u(0)) * w as f32,
        y: (0.08 + 0.35 * u(1)) * hz as f32,
        age,
        reach: (0.1 + 0.08 * u(2)) * hz as f32,
        cols: (c, (c + 1 + (h >> 40) as usize % 2) % 3),
    })
}

/// Iztaccíhuatl's silhouette from her head (0) to her feet (1), as a share
/// of the height of her chest: the head with its forehead, nose and chin,
/// the neck, the chest, the waist, the knees, the ankles and the feet.
fn izta(u: f32) -> f32 {
    const P: [(f32, f32); 20] = [
        (0.00, 0.00),
        (0.03, 0.22),
        (0.07, 0.50),
        (0.10, 0.62),
        (0.12, 0.56),
        (0.14, 0.62),
        (0.16, 0.50),
        (0.19, 0.42),
        (0.22, 0.40),
        (0.27, 0.78),
        (0.34, 1.00),
        (0.41, 0.94),
        (0.48, 0.66),
        (0.55, 0.62),
        (0.62, 0.84),
        (0.68, 0.86),
        (0.75, 0.66),
        (0.84, 0.58),
        (0.93, 0.34),
        (1.00, 0.00),
    ];
    let i = P.windows(2).position(|p| u < p[1].0).unwrap_or(P.len() - 2);
    let (a, b) = (P[i], P[i + 1]);
    let t = ((u - a.0) / (b.0 - a.0)).clamp(0.0, 1.0);
    a.1 + (b.1 - a.1) * (1.0 - (t * PI).cos()) / 2.0
}

/// Where Pátzcuaro's night draws: the picture, which pixels are solid, its
/// size, how far the night has faded in (`m`), how much brighter solid
/// things are drawn, and what reflections fade toward (`None`: they dim).
struct Layer<'a> {
    px: &'a mut [V3],
    solid: &'a mut [bool],
    w: i32,
    h: i32,
    m: f32,
    lift: f32,
    water: Option<V3>,
    /// Motion time and the horizon, for the lake's ripples.
    t: f32,
    hz: i32,
}

impl Layer<'_> {
    /// How far row `y` of the lake ripples sideways, as in `compose`.
    fn ripple(&self, y: i32) -> i32 {
        let d = (y - self.hz) as f32 / (self.h - self.hz).max(1) as f32;
        ((y as f32 * 0.7 + self.t * 0.09).sin() * (0.4 + 2.2 * d)).round() as i32
    }

    /// Janitzio and its reflection, `lit` of its candles burning.
    fn janitzio(&mut self, isle: &[Isle], base: i32, f: &Fest, lit: f32) {
        for p in isle {
            let ry = 2 * base - p.y + 1;
            let col = if p.part == 2 {
                // The statue, floodlit.
                self.lamp(p.x, p.y, f.isle[2]);
                f.isle[2]
            } else {
                let ridge = if p.part == 3 { 0.45 } else { 0.0 };
                let col = mix(f.isle[0], f.isle[1], p.warm * lit + ridge);
                self.put(p.x, p.y, col);
                col
            };
            self.mirror(p.x + self.ripple(ry), ry, col, 0.55);
        }
        for p in isle.iter().filter(|p| p.part == 1 && p.r < lit) {
            let (c, k) = flicker(&f.flame, self.t, p.r * 40.0);
            self.candle(p.x, p.y, c, k, 0.0);
            // Each flame trails a shimmering streak down the water.
            let ry = 2 * base - p.y + 1;
            for (dy, a) in [(0, 0.5), (1, 0.35), (2, 0.2), (3, 0.1)] {
                self.shine(p.x + self.ripple(ry + dy), ry + dy, c, a * k);
            }
        }
    }

    /// A veladora on the water: its flame over the glass, and the flame's
    /// streak down the lake.
    fn veladora(&mut self, v: &Float, f: &Fest) {
        let x = v.x.round() as i32;
        let (fl, k) = flicker(&f.flame, self.t, v.phase);
        self.candle(x, v.y - 1, fl, k, 0.2);
        self.lamp(x, v.y, f.wax);
        for (dy, a) in [(1, 0.45), (2, 0.25), (3, 0.1)] {
            let y = v.y + dy;
            self.shine(x + self.ripple(y), y, fl, a * k);
        }
    }

    /// A canoe, its nets and candles, and its reflection.
    fn canoe(&mut self, c: &Canoe, f: &Fest) {
        let rows: &[&str] = if c.far { &CANOE_FAR } else { &CANOE };
        let (x0, n) = (c.x.round() as i32, rows.len() as i32);
        for (r, row) in rows.iter().enumerate() {
            let y = c.y - (n - 1 - r as i32);
            let ry = c.y + n - r as i32;
            let dx = self.ripple(ry);
            for (k, ch) in row.bytes().enumerate() {
                let x = x0 + k as i32;
                match ch {
                    b'o' => {
                        self.lamp(x, y, f.net[0]);
                        self.veil(x + dx, ry, f.net[0], 0.25);
                    }
                    b':' => {
                        self.veil(x, y, f.net[1], 0.22);
                        self.veil(x + dx, ry, f.net[1], 0.08);
                    }
                    b'f' | b'h' | b'H' => {
                        let col = match ch {
                            b'f' => f.rower,
                            b'h' => f.hull[0],
                            _ => f.hull[1],
                        };
                        self.put(x, y, col);
                        self.mirror(x + dx, ry, col, 0.5);
                    }
                    b'w' => {
                        self.lamp(x, y, f.wax);
                        self.veil(x + dx, ry, f.wax, 0.3);
                    }
                    b'c' => {
                        let (fl, kk) = flicker(&f.flame, self.t, c.phase + k as f32);
                        self.candle(x, y, fl, kk, 0.25);
                        for dy in 0..5 {
                            let yy = c.y + 1 + dy;
                            let a = kk * (0.6 - 0.11 * dy as f32);
                            self.shine(x + self.ripple(yy), yy, fl, a);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    fn at(&self, x: i32, y: i32) -> Option<usize> {
        (x >= 0 && y >= 0 && x < self.w && y < self.h).then(|| (y * self.w + x) as usize)
    }

    /// A solid pixel of something dark, drawn brighter over a transparent sky.
    fn put(&mut self, x: i32, y: i32, col: V3) {
        self.lamp(x, y, scale(col, self.lift));
    }

    /// A solid pixel in its own color: a flame, a wing.
    fn lamp(&mut self, x: i32, y: i32, col: V3) {
        if let Some(i) = self.at(x, y) {
            self.px[i] = mix(self.px[i], col, self.m);
            self.solid[i] |= self.m >= 0.5;
        }
    }

    /// Light added to whatever is there.
    fn shine(&mut self, x: i32, y: i32, col: V3, a: f32) {
        if let Some(i) = self.at(x, y) {
            self.px[i] = add(self.px[i], scale(col, a * self.m));
        }
    }

    /// A flame `k` bright, with a halo of `glow` around it.
    fn candle(&mut self, x: i32, y: i32, flame: V3, k: f32, glow: f32) {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            self.shine(x + dx, y + dy, flame, glow * k);
        }
        self.lamp(x, y, scale(flame, k));
    }

    /// See-through: blends over something solid, glows faintly over the sky.
    fn veil(&mut self, x: i32, y: i32, col: V3, a: f32) {
        if let Some(i) = self.at(x, y) {
            if self.solid[i] {
                self.px[i] = mix(self.px[i], scale(col, self.lift), a * self.m);
            } else {
                self.px[i] = add(self.px[i], scale(col, a * self.m));
            }
        }
    }

    /// A solid thing's reflection in the lake, darkened by `dim`.
    fn mirror(&mut self, x: i32, y: i32, col: V3, dim: f32) {
        let c = match self.water {
            Some(water) => mix(water, scale(col, self.lift), dim),
            None => scale(col, self.lift * dim),
        };
        if let Some(i) = self.at(x, y) {
            self.px[i] = mix(self.px[i], c, self.m);
            self.solid[i] |= self.m >= 0.5;
        }
    }
}

/// What the Navidad's night draws with: shapes, fires, palms and the star.
impl Layer<'_> {
    /// A dark thing's color, brighter over a transparent sky.
    fn dark(&self, col: V3) -> V3 {
        scale(col, self.lift)
    }

    /// A filled ellipse, never thinner than a pixel.
    fn ellipse(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, col: V3) {
        let (rx, ry) = (rx.max(0.7), ry.max(0.7));
        let (y0, y1) = (
            ((cy - ry).floor() as i32).max(0),
            ((cy + ry).ceil() as i32).min(self.h - 1),
        );
        let (x0, x1) = (
            ((cx - rx).floor() as i32).max(0),
            ((cx + rx).ceil() as i32).min(self.w - 1),
        );
        for y in y0..=y1 {
            for x in x0..=x1 {
                let (u, v) = ((x as f32 + 0.5 - cx) / rx, (y as f32 + 0.5 - cy) / ry);
                if u * u + v * v <= 1.0 {
                    self.lamp(x, y, col);
                }
            }
        }
    }

    /// A line `thick` pixels across, from one point to another.
    fn stroke(&mut self, (ax, ay): (f32, f32), (bx, by): (f32, f32), thick: f32, col: V3) {
        let r = thick.max(1.0) / 2.0;
        let (dx, dy) = (bx - ax, by - ay);
        let len2 = (dx * dx + dy * dy).max(1e-6);
        let (y0, y1) = (
            ((ay.min(by) - r).floor() as i32).max(0),
            ((ay.max(by) + r).ceil() as i32).min(self.h - 1),
        );
        let (x0, x1) = (
            ((ax.min(bx) - r).floor() as i32).max(0),
            ((ax.max(bx) + r).ceil() as i32).min(self.w - 1),
        );
        for y in y0..=y1 {
            for x in x0..=x1 {
                let (px, py) = (x as f32 + 0.5 - ax, y as f32 + 0.5 - ay);
                let t = ((px * dx + py * dy) / len2).clamp(0.0, 1.0);
                let (qx, qy) = (px - t * dx, py - t * dy);
                if qx * qx + qy * qy <= r * r {
                    self.lamp(x, y, col);
                }
            }
        }
    }

    /// A soft glow around (x, y), `rx` wide and `ry` high, `a` bright at its
    /// heart.
    fn halo(&mut self, x: i32, y: i32, rx: i32, ry: i32, col: V3, a: f32) {
        let (rx, ry) = (rx.max(1), ry.max(1));
        for dy in -ry..=ry {
            for dx in -rx..=rx {
                let d = (dx as f32 / rx as f32).powi(2) + (dy as f32 / ry as f32).powi(2);
                if d < 1.0 {
                    self.shine(x + dx, y + dy, col, a * (1.0 - d) * (1.0 - d));
                }
            }
        }
    }

    /// A date palm: a leaning trunk and a crown of drooping fronds.
    fn palm(&mut self, p: &Palm, n: &Nacimiento) {
        let (x, y) = (p.x as f32, p.y as f32);
        let top = (x + p.lean, y - p.h);
        let (trunk, frond) = (self.dark(n.palm[1]), self.dark(n.palm[0]));
        let mid = (x + p.lean * 0.3, y - p.h * 0.5);
        let thick = 1.0 + p.h * 0.05;
        self.stroke((x, y), mid, thick, trunk);
        self.stroke(mid, top, thick, trunk);
        let len = p.h * 0.5 + 1.0;
        for k in 0..7 {
            let a = PI * (0.08 + 0.84 * k as f32 / 6.0);
            let (c, s) = (a.cos(), a.sin());
            let tip = (
                top.0 + c * len,
                top.1 - s * len * 0.55 + c.abs() * len * 0.4,
            );
            self.stroke(top, tip, 1.0, frond);
        }
    }

    /// A king on his mount at (x, foot), `s` times its size, walking when
    /// `step` is more than 0 (`gait` is where in the stride it is).
    fn rey(
        &mut self,
        kind: usize,
        (x, foot): (f32, f32),
        s: f32,
        gait: f32,
        step: f32,
        n: &Nacimiento,
    ) {
        let parts: &[(Part, u8)] = match kind {
            0 => &HORSE,
            1 => &CAMEL,
            _ => &ELEPHANT,
        };
        let colors = [
            n.beast[kind.min(2)],
            n.robe[kind.min(2)],
            n.gold,
            n.cloth,
            n.skin,
            n.tusk,
            scale(n.beast[kind.min(2)], 0.8),
        ];
        // The rider rocks with the stride.
        let bob = step * 0.5 * gait.sin().abs();
        for &(part, c) in parts {
            let col = colors[c as usize];
            match part {
                Part::Ell(cx, cy, rx, ry) => {
                    self.ellipse(x + cx * s, foot + (cy + bob) * s, rx * s, ry * s, col);
                }
                Part::Seg(ax, ay, bx, by, th) => {
                    let (a, b) = (
                        (x + ax * s, foot + (ay + bob) * s),
                        (x + bx * s, foot + (by + bob) * s),
                    );
                    self.stroke(a, b, th * s, col);
                }
                Part::Leg(hx, hy, len, th, ph) => {
                    let swing = step * 1.8 * (gait + ph).sin();
                    let lift = step * 1.4 * (gait + ph).cos().max(0.0);
                    let a = (x + hx * s, foot + (hy + bob) * s);
                    let b = (x + (hx + swing) * s, foot + (hy + len - lift) * s);
                    self.stroke(a, b, th * s, col);
                }
            }
        }
    }

    /// The Estrella de Belén at (x, y), `k` bright: four long rays, a halo
    /// and a tail streaming away toward `from`, swaying like a comet's.
    fn estrella(&mut self, (x, y): (f32, f32), from: (f32, f32), k: f32, col: [V3; 2]) {
        let (xi, yi) = (x.round() as i32, y.round() as i32);
        let k = k * (0.9 + 0.1 * (self.t * 0.11).sin());
        let r = (self.h as f32 * 0.12).clamp(5.0, 14.0);
        self.halo(xi, yi, r as i32, r as i32, col[1], 0.5 * k);
        let reach = (r * 1.8) as i32;
        for s in 1..=reach {
            let a = k * (1.0 - s as f32 / (reach + 1) as f32).powi(2);
            for (dx, dy) in [(s, 0), (-s, 0), (0, s), (0, -s)] {
                self.shine(xi + dx, yi + dy, col[0], a);
            }
            if s <= reach / 3 {
                for (dx, dy) in [(s, s), (s, -s), (-s, s), (-s, -s)] {
                    self.shine(xi + dx, yi + dy, col[1], a * 0.7);
                }
            }
        }
        for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            self.lamp(
                xi + dx,
                yi + dy,
                scale(col[0], k.min(1.0) * if dx + dy == 0 { 1.0 } else { 0.85 }),
            );
        }
        for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            self.lamp(xi + dx, yi + dy, scale(col[0], k.min(1.0) * 0.6));
        }
        let (dx, dy) = (from.0 - x, from.1 - y);
        let len = dx.hypot(dy).max(1.0);
        let (ux, uy) = (dx / len, dy / len);
        for i in 3..len as i32 {
            let u = i as f32 / len;
            let sway = (i as f32 * 0.3 - self.t * 0.12).sin() * (0.3 + 1.6 * u);
            let (cx, cy) = (x + ux * i as f32 - uy * sway, y + uy * i as f32 + ux * sway);
            let a = k
                * (1.0 - u).powf(1.5)
                * 1.0
                * (0.85 + 0.15 * (i as f32 * 0.9 - self.t * 0.2).sin());
            let c = mix(col[0], col[1], u);
            let (cxi, cyi) = (cx.round() as i32, cy.round() as i32);
            self.shine(cxi, cyi, c, a);
            // The tail widens and thins as it goes.
            let side = if ux.abs() > uy.abs() { (0, 1) } else { (1, 0) };
            self.shine(cxi + side.0, cyi + side.1, c, a * 0.5);
            self.shine(cxi - side.0, cyi - side.1, c, a * 0.5);
            if u > 0.25 {
                self.shine(cxi + 2 * side.0, cyi + 2 * side.1, c, a * 0.2);
                self.shine(cxi - 2 * side.0, cyi - 2 * side.1, c, a * 0.2);
            }
        }
    }
}

/// What the Equinoccio's dawn draws with.
impl Layer<'_> {
    /// The rising sun at (x, y), `r` across, `k` bright: a halo, the disc
    /// and rays fanning out, slowly turning. All of it is light, so it shows
    /// over a transparent sky.
    fn sun(&mut self, (x, y): (f32, f32), r: f32, k: f32, col: V3) {
        let (xi, yi) = (x.round() as i32, y.round() as i32);
        let k = k * (0.95 + 0.05 * (self.t * 0.07).sin());
        let warm = mix(col, [1.0; 3], 0.25);
        self.halo(xi, yi, (r * 4.5) as i32, (r * 3.2) as i32, col, 0.5 * k);
        self.halo(xi, yi, (r * 2.2) as i32, (r * 2.2) as i32, warm, 0.7 * k);
        let reach = r.ceil() as i32;
        for dy in -reach..=reach {
            for dx in -reach..=reach {
                if (dx * dx + dy * dy) as f32 <= r * r {
                    self.shine(xi + dx, yi + dy, warm, 1.1 * k);
                }
            }
        }
        let rays = 18;
        for i in 0..rays {
            let a = TAU * i as f32 / rays as f32 + self.t * 0.002;
            let len = r * if i % 2 == 0 { 7.0 } else { 4.5 };
            let (c, s) = (a.cos(), a.sin());
            for step in (r as i32 + 1)..len as i32 {
                let a = 0.4 * k * (1.0 - step as f32 / len).powf(1.5);
                self.shine(
                    (x + c * step as f32).round() as i32,
                    (y + s * step as f32).round() as i32,
                    col,
                    a,
                );
            }
        }
    }

    /// A maguey: a rosette of pointed leaves.
    fn maguey(&mut self, m: &Maguey, col: V3) {
        let (x, y) = (m.x as f32, m.y as f32);
        for k in 0..9 {
            let a = PI * (0.08 + 0.84 * k as f32 / 8.0);
            let tip = (x + a.cos() * m.size, y - a.sin() * m.size);
            self.stroke((x, y), tip, 1.4, col);
        }
    }

    /// A person in white, their arms up or down.
    fn person(&mut self, p: &Person, up: bool, n: &Piramide) {
        let frame = usize::from(!up);
        let rows: &[&str] = if p.big {
            &PERSON[frame]
        } else {
            &PERSON_FAR[frame]
        };
        let (w, h) = (rows[0].len() as i32, rows.len() as i32);
        for (r, row) in rows.iter().enumerate() {
            for (k, ch) in row.bytes().enumerate() {
                let col = match ch {
                    b'w' => n.robe,
                    b'o' => n.skin,
                    _ => continue,
                };
                self.lamp(p.x - w / 2 + k as i32, p.y - h + 1 + r as i32, col);
            }
        }
    }
}

/// What the Fiestas Patrias' night draws with.
impl Layer<'_> {
    /// A puff of smoke: a soft disc `r` wide, `a` thick at its heart. It is
    /// painted over what is there (so it darkens a bright curtain behind it)
    /// without turning it solid.
    fn puff(&mut self, x: f32, y: f32, r: f32, col: V3, a: f32) {
        let r = r.max(1.0);
        let (xi, yi, reach) = (x.round() as i32, y.round() as i32, r.ceil() as i32);
        for dy in -reach..=reach {
            for dx in -reach..=reach {
                let d = (dx * dx + dy * dy) as f32 / (r * r);
                if d < 1.0
                    && let Some(i) = self.at(xi + dx, yi + dy)
                {
                    self.px[i] = mix(self.px[i], col, a * (1.0 - d) * self.m);
                }
            }
        }
    }

    /// A burst of fireworks: sparks flying out of its heart in two colors,
    /// sagging and fading, each with a short trail, after a flash.
    fn burst(&mut self, b: &Burst, cols: &[V3; 3]) {
        let k = 1.0 - b.age / BURST_LIFE;
        let r = b.reach * (1.0 - (-b.age * 3.0).exp());
        let fall = 2.2 * b.age * b.age * (b.reach / 12.0);
        let n = 20;
        for i in 0..n {
            let a = TAU * i as f32 / n as f32 + b.x;
            let c = cols[if i % 2 == 0 { b.cols.0 } else { b.cols.1 }];
            for (j, strength) in [(0.0, 1.0), (1.0, 0.55), (2.0, 0.25)] {
                let rr = (r - j * 1.3).max(0.0);
                let (x, y) = (b.x + a.cos() * rr, b.y + a.sin() * rr * 0.85 + fall);
                self.shine(
                    x.round() as i32,
                    y.round() as i32,
                    c,
                    k * k * strength * 0.9,
                );
            }
        }
        if b.age < 0.25 {
            let flash = 1.0 - b.age / 0.25;
            self.halo(b.x as i32, b.y as i32, 5, 5, cols[1], 0.7 * flash);
        }
    }
}

/// A candle flame flickering at time `t` (in tanim ticks): its color and
/// how bright it is.
fn flicker(flame: &[V3; 2], t: f32, phase: f32) -> (V3, f32) {
    let f = 0.55 * (t * 0.7 + phase).sin() + 0.45 * (t * 1.9 + 2.3 * phase).sin();
    (mix(flame[0], flame[1], 0.5 + 0.5 * f), 0.85 + 0.15 * f)
}

struct Curtain {
    /// Screen x where the track starts and its length.
    x0: f32,
    len: f32,
    /// Lower edge height (pixel row) and how far the light reaches upwards.
    base: f32,
    tall: f32,
    /// Ripples along the track: amplitude, spatial frequency, speed.
    ripples: [(f32, f32, f32); 3],
    drift: f32,
    bright: f32,
    age: u32,
    life: u32,
    phase: f32,
}

struct Star {
    x: i32,
    y: i32,
    b: f32,
    phase: f32,
}

struct Meteor {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: u32,
}

/// What Van Gogh's sky looks like at one pixel, worked out once per size.
#[derive(Clone, Copy, Default)]
struct Brush {
    /// Sine and cosine of the phase of the bands of the flowing sky and of
    /// the nearest whirl's spiral, and how much the whirl rules here.
    flow: (f32, f32),
    whirl: (f32, f32),
    pull: f32,
    /// A little jitter, so the bands break into dabs of paint.
    dab: f32,
    /// Phase of the rings around the nearest star or the moon, and how
    /// strongly they show.
    ring: f32,
    halo: f32,
    moon: bool,
    /// Inside a star (1) or the moon's crescent (2).
    core: u8,
}

fn v3(c: Color) -> V3 {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => ansi_rgb(n),
        Color::Reset => (255, 255, 255),
    };
    c8(r, g, b)
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

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// A color between evenly spaced stops, `t` in 0..1.
fn ramp(stops: &[V3], t: f32) -> V3 {
    let n = stops.len();
    let t = t.clamp(0.0, 1.0) * (n - 1) as f32;
    let i = (t as usize).min(n - 2);
    mix(stops[i], stops[i + 1], t - i as f32)
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

/// Colors snap to steps of 4 so that slowly shifting light stays identical
/// from frame to frame and the renderer has fewer cells to send.
fn to_color(c: V3) -> Color {
    let b = |x: f32| ((x.clamp(0.0, 1.0) * 255.0 + 2.0) as u32 & !3).min(252) as u8;
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

/// An integer in `lo..hi`, or `lo` when the range is empty.
fn range(rng: &mut fastrand::Rng, lo: i32, hi: i32) -> i32 {
    if hi <= lo { lo } else { rng.i32(lo..hi) }
}

fn hash(x: i32, y: i32, seed: u64) -> u64 {
    let mut h = seed
        ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^ (h >> 29)
}

/// 1D midpoint displacement: a jagged ridge line of `n` heights in `[0, 1]`.
fn ridge(n: usize, rough: f32, rng: &mut fastrand::Rng) -> Vec<f32> {
    let mut size = 1;
    while size + 1 < n {
        size *= 2;
    }
    let mut v = vec![0.0f32; size + 1];
    v[0] = rangef(rng, 0.2, 0.8);
    v[size] = rangef(rng, 0.2, 0.8);
    let mut step = size;
    let mut amp = 0.6;
    while step > 1 {
        let half = step / 2;
        let mut i = half;
        while i < size {
            v[i] = (v[i - half] + v[i + half]) / 2.0 + rangef(rng, -amp, amp);
            i += step;
        }
        amp *= rough;
        step = half;
    }
    let (lo, hi) = v
        .iter()
        .fold((f32::MAX, f32::MIN), |(a, b), &x| (a.min(x), b.max(x)));
    let span = (hi - lo).max(1e-3);
    (0..n)
        .map(|i| (v[i * size / n.max(1)] - lo) / span)
        .collect()
}

struct Aurora {
    rng: fastrand::Rng,
    seed: u64,
    speed: f32,
    count: usize,
    landscape: bool,
    show_stars: bool,
    paint_sky: bool,
    /// The look to stay on, or `None` to cycle.
    fixed: Option<usize>,
    stay: f32,
    /// The look showing and the one it fades into, by index into `LOOKS`, in
    /// the theme's colors, and real seconds since the look began.
    which: usize,
    next: usize,
    cur: Look,
    nxt: Look,
    clock: f32,
    paint: Paint,
    /// Pixel size of the screen (width, twice the rows) and the horizon row.
    w: usize,
    ph: usize,
    horizon: usize,
    /// What tanim's sizes are fractions of: the screen's pixel height, or more
    /// when there is no landscape and the sky reaches the bottom.
    span: f32,
    far: Vec<i32>,
    near: Vec<i32>,
    /// Pine silhouettes on the near shore: (x, height).
    pines: Vec<(i32, i32)>,
    stars: Vec<Star>,
    curtains: Vec<Curtain>,
    meteor: Option<Meteor>,
    /// Van Gogh's sky, cypress (pixel and stroke) and village (pixel and part).
    brush: Vec<Brush>,
    cypress: Vec<(usize, f32)>,
    village: Vec<(usize, u8)>,
    /// Per-column light and weighted lower edge for the curtain being drawn.
    light: Vec<f32>,
    edge: Vec<f32>,
    /// A curtain's glow by distance above its lower edge: strength and color.
    profile: Vec<(f32, V3)>,
    /// Additive aurora light per sky pixel.
    glow: Vec<V3>,
    /// The picture before it becomes colors, and which pixels are solid; the
    /// others hold light over a transparent sky.
    px: Vec<V3>,
    solid: Vec<bool>,
    out: Pixels,
    /// Motion time in tanim ticks.
    t: f32,
    /// Occasional substorm: brightens and quickens everything, then decays.
    surge: f32,
    acc: f32,
    /// Nyan Cat or the train, while one goes by, and for how long it has.
    visitor: Option<Box<dyn Egg>>,
    visiting: f32,
    visitors: f32,
    cols: u16,
    rows: u16,
    /// The holiday, and its own RNG so that ordinary days draw the same
    /// random numbers they always have.
    fiesta: Fiesta,
    hrng: fastrand::Rng,
    fest: Fest,
    /// Pátzcuaro: Janitzio and the row its shore meets the water on, the
    /// canoes and the monarchs (as many as the holiday's height brings).
    isle: Vec<Isle>,
    isle_base: i32,
    canoes: Vec<Canoe>,
    floats: Vec<Float>,
    monarchs: Vec<Monarch>,
    /// Belén: its colors, the town on the hills and the portal, where the
    /// star hangs, the dunes' crests (three layers, far to near), the palms,
    /// the shepherds' fire and flock, and the Reyes' walk.
    nac: Nacimiento,
    town: Vec<Spot>,
    portal: Vec<Spot>,
    star: (f32, f32),
    crests: Vec<i32>,
    /// Where the wind has rippled the sand.
    ripples: Vec<bool>,
    palms: Vec<Palm>,
    camp: Option<(i32, i32)>,
    flock: Vec<Sheep>,
    reyes_x: f32,
    gait: f32,
    /// The aurora's faint light on the land, as `compose` last worked it out.
    amb: V3,
    /// The Fiestas Patrias: its colors, the volcanoes' silhouette by column
    /// (the row of the top and whose it is: 1 Iztaccíhuatl, 2 Popocatépetl)
    /// and where their snow begins, the crater, the garlands along the shore
    /// and the flag (pole, foot, top and width).
    volcan: Volcan,
    volc: Vec<(i32, u8)>,
    snowline: Vec<i32>,
    crater: (f32, f32),
    bulbs: Vec<Bulb>,
    flag: Option<(i32, i32, i32, i32)>,
    /// Teotihuacán: its colors, the Pirámide del Sol (half its width on each
    /// row from the summit down, the same for its stairs, and where its
    /// summit is), the sun behind it (x, y, radius), the platforms along the
    /// Calzada, the crowd and the magueyes.
    piramide: Piramide,
    pyr: Vec<i32>,
    stairs: Vec<i32>,
    pyr_at: (i32, i32),
    sun: (f32, f32, f32),
    platforms: Vec<Platform>,
    crowd: Vec<Person>,
    magueyes: Vec<Maguey>,
}

impl Aurora {
    fn new(ctx: &Ctx) -> Aurora {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let mut hrng = fastrand::Rng::with_seed(ctx.seed ^ 0x0102_1102_0028_1103);
        let t = rangef(&mut rng, 0.0, 1000.0);
        let look = match o.s("look").trim() {
            "pátzcuaro" | "muertos" => "patzcuaro",
            "belén" | "navidad" | "nacimiento" => "belen",
            "volcán" | "patrias" | "independencia" => "volcanes",
            "teotihuacán" | "equinoccio" | "pirámide" => "teotihuacan",
            l => l,
        };
        let fixed = LOOKS.iter().position(|(n, _)| *n == look);
        let which = fixed.unwrap_or_else(|| rng.usize(0..ORDINARY));
        // The holiday opens on its own look.
        let which = match fiesta_look(ctx.fiesta.kind) {
            Some(i) if fixed.is_none() => i,
            _ => which,
        };
        let next = Self::pick_next(&mut rng, &mut hrng, which, fixed, ctx.fiesta);
        Aurora {
            seed: ctx.seed,
            speed: num(o.f("speed"), 1.0, 0.1, 4.0),
            count: o.i("curtains").clamp(1, 8) as usize,
            landscape: o.b("landscape"),
            show_stars: o.b("stars"),
            paint_sky: o.b("sky"),
            fixed,
            stay: num(o.f("stay"), 90.0, 10.0, 3600.0),
            which,
            next,
            cur: LOOKS[which].1,
            nxt: LOOKS[next].1,
            clock: 0.0,
            paint: Paint::new(ctx.theme),
            w: 0,
            ph: 0,
            horizon: 0,
            span: 0.0,
            far: Vec::new(),
            near: Vec::new(),
            pines: Vec::new(),
            stars: Vec::new(),
            curtains: Vec::new(),
            meteor: None,
            brush: Vec::new(),
            cypress: Vec::new(),
            village: Vec::new(),
            light: Vec::new(),
            edge: Vec::new(),
            profile: Vec::new(),
            glow: Vec::new(),
            px: Vec::new(),
            solid: Vec::new(),
            out: Pixels::new(1, 1),
            t,
            surge: 0.0,
            acc: 0.0,
            visitor: None,
            visiting: 0.0,
            visitors: num(o.f("visitors"), 3.0, 0.0, eggs::FORCE_RATE as f32),
            cols: 0,
            rows: 0,
            fiesta: ctx.fiesta,
            fest: Fest::new(ctx.theme),
            isle: Vec::new(),
            isle_base: 0,
            canoes: Vec::new(),
            floats: Vec::new(),
            monarchs: Vec::new(),
            nac: Nacimiento::new(ctx.theme),
            town: Vec::new(),
            portal: Vec::new(),
            star: (0.0, 0.0),
            crests: Vec::new(),
            ripples: Vec::new(),
            palms: Vec::new(),
            camp: None,
            flock: Vec::new(),
            reyes_x: 0.0,
            gait: 0.0,
            amb: [0.0; 3],
            volcan: Volcan::new(ctx.theme),
            volc: Vec::new(),
            snowline: Vec::new(),
            crater: (0.0, 0.0),
            bulbs: Vec::new(),
            flag: None,
            piramide: Piramide::new(ctx.theme),
            pyr: Vec::new(),
            stairs: Vec::new(),
            pyr_at: (0, 0),
            sun: (0.0, 0.0, 0.0),
            platforms: Vec::new(),
            crowd: Vec::new(),
            magueyes: Vec::new(),
            rng,
            hrng,
        }
    }

    /// The look after `which`: any other one, or the same when it is fixed.
    /// During a holiday with a look of its own it is mostly that one,
    /// decided by the holiday's own RNG.
    fn pick_next(
        rng: &mut fastrand::Rng,
        hrng: &mut fastrand::Rng,
        which: usize,
        fixed: Option<usize>,
        fiesta: Fiesta,
    ) -> usize {
        if let Some(i) = fixed {
            return i;
        }
        if let Some(i) = fiesta_look(fiesta.kind)
            && hrng.f32() < fiesta_share(fiesta.level)
        {
            return i;
        }
        if which >= ORDINARY {
            rng.usize(0..ORDINARY)
        } else {
            (which + 1 + rng.usize(0..ORDINARY - 1)) % ORDINARY
        }
    }

    /// How much of `kind`'s holiday has come: all of it when its look is
    /// asked for on another day.
    fn level(&self, kind: Kind) -> f32 {
        if self.fiesta.is(kind) {
            self.fiesta.level.clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    /// How far the Reyes Magos have come, from just seen on the far dunes
    /// (0, January 2) to at the portal (1, January 6); not yet in December.
    /// Halfway when Belén is asked for on another day.
    fn reyes_way(&self) -> Option<f32> {
        let f = self.fiesta;
        if f.reyes() {
            Some((f32::from(f.day) - 2.0).clamp(0.0, 4.0) / 4.0)
        } else if f.is(Kind::Navidad) {
            None
        } else {
            Some(0.5)
        }
    }

    /// Does the Niño Dios lie in the manger? From Nochebuena on.
    fn nino(&self) -> bool {
        let f = self.fiesta;
        !f.is(Kind::Navidad) || f.month == 1 || f.day >= 24
    }

    fn look_of(i: usize, theme: &Theme) -> Look {
        let l = LOOKS[i].1;
        if theme.native { l } else { l.themed(theme) }
    }

    /// The look on screen now, partway into the next one near the end of its stay.
    fn look(&self) -> Look {
        let into = (self.clock - (self.stay - BLEND)) / BLEND;
        let k = into.clamp(0.0, 1.0);
        self.cur.lerp(&self.nxt, k * k * (3.0 - 2.0 * k))
    }

    /// The ridges, the forest, the stars and a sky already full of curtains.
    fn build(&mut self) {
        let (w, ph) = (self.w, self.ph);
        self.horizon = if self.landscape { ph * 62 / 100 } else { ph };
        self.span = if self.landscape {
            ph as f32
        } else {
            ph as f32 / 0.62
        };
        let (hz, phf) = (self.horizon as f32, ph as f32);
        let rng = &mut self.rng;
        self.far = ridge(w, 0.55, rng)
            .iter()
            .map(|v| (hz - phf * (0.06 + 0.22 * v)) as i32)
            .collect();
        self.near = ridge(w, 0.45, rng)
            .iter()
            .map(|v| (hz - phf * (0.02 + 0.05 * v)) as i32)
            .collect();
        // A pine forest along the top of the near hills, dark against the
        // mountains behind.
        self.pines.clear();
        let mut x = range(rng, 0, 3);
        while (x as usize) < w {
            if rng.f32() < 0.7 {
                self.pines.push((x, range(rng, 2, (ph as i32 / 14).max(3))));
            }
            x += range(rng, 1, 4);
        }
        let ceiling = if self.landscape {
            self.horizon as i32 - 2
        } else {
            self.horizon as i32
        };
        self.stars = (0..(w * self.horizon / 22).max(1))
            .map(|_| Star {
                x: range(rng, 0, w as i32),
                y: range(rng, 0, ceiling.max(1)),
                b: rangef(rng, 0.25, 1.0),
                phase: rangef(rng, 0.0, 6.3),
            })
            .collect();
        self.curtains.clear();
        for i in 0..self.count {
            let mut c = self.curtain();
            // Start mid-life so the sky is not empty at first.
            c.age = c.life / 3 + i as u32 * 40;
            self.curtains.push(c);
        }
        self.meteor = None;
        self.build_starry();
        self.build_patzcuaro();
        self.build_belen();
        self.build_volcanes();
        self.build_teotihuacan();
    }

    /// Pátzcuaro's night for this size, from the holiday's own RNG: the
    /// monarchs, Janitzio rising from the lake with candles on its terraces
    /// and Morelos on its summit, and the canoes on the water in front.
    fn build_patzcuaro(&mut self) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (wf, hzf) = (w as f32, hz as f32);
        let rng = &mut self.hrng;
        self.monarchs.clear();
        let dir = if rng.bool() { 1.0 } else { -1.0 };
        let n = ((w * hz) as f32 / 450.0).ceil().clamp(2.0, 60.0) as usize;
        for _ in 0..n {
            self.monarchs.push(Monarch {
                x: rangef(rng, -0.2, 1.2) * wf,
                y: rangef(rng, 0.06, 0.8) * hzf,
                vx: dir * rangef(rng, 4.0, 8.0),
                bob: rangef(rng, 1.0, 3.0),
                rate: rangef(rng, 0.6, 1.4),
                phase: rangef(rng, 0.0, 6.3),
                flap: rangef(rng, 0.0, 4.0),
            });
        }
        self.isle.clear();
        self.canoes.clear();
        self.floats.clear();
        let depth = ph - hz;
        if !self.landscape || w < 16 || hz < 6 || depth < 6 {
            return;
        }
        // Janitzio: a round island out on the lake, its terraces lined with
        // candles.
        let base = hz + (depth as f32 * 0.3).round().max(1.0) as i32;
        let half = (wf * 0.11).clamp(7.0, 30.0);
        let tall = (depth as f32 * 0.42).clamp(4.0, 18.0);
        let cx = wf * rangef(rng, 0.55, 0.7);
        let mut summit = base;
        for x in (cx - half) as i32..=(cx + half) as i32 {
            let u = (x as f32 + 0.5 - cx) / half;
            if u.abs() >= 1.0 {
                continue;
            }
            let rise = tall * (1.0 - u * u).powf(0.65) * (0.92 + 0.08 * (x as f32 * 0.9).sin());
            let top = base - rise.round() as i32;
            if x == cx as i32 {
                summit = top;
            }
            for y in top..=base {
                let terrace = (base - y) % 2 == 1 && y > top;
                let candle = terrace && hash(x, y, self.seed ^ 0x3A).is_multiple_of(3);
                self.isle.push(Isle {
                    x,
                    y,
                    part: if y == top { 3 } else { u8::from(candle) },
                    r: rng.f32(),
                    warm: 0.0,
                });
            }
        }
        if tall >= 6.0 {
            let statue: &[&str] = if tall >= 11.0 { &STATUE } else { &STATUE_SMALL };
            let x0 = cx as i32 - 2;
            for (dy, row) in statue.iter().enumerate() {
                for (dx, ch) in row.bytes().enumerate() {
                    if ch == b'S' {
                        self.isle.push(Isle {
                            x: x0 + dx as i32,
                            y: summit - statue.len() as i32 + dy as i32 + 2,
                            part: 2,
                            r: 0.0,
                            warm: 0.0,
                        });
                    }
                }
            }
        }
        // Candlelight warms the island around each candle (as if all were lit).
        for i in 0..self.isle.len() {
            let (x, y) = (self.isle[i].x, self.isle[i].y);
            let near = self
                .isle
                .iter()
                .filter(|o| o.part == 1 && (o.x - x).abs() <= 1 && (o.y - y).abs() <= 1)
                .count();
            self.isle[i].warm = (near as f32 / 3.0).min(1.0) * 0.6;
        }
        self.isle_base = base;
        // Canoes in lanes from the far water to the near, the far ones
        // smaller and slower; those beyond the island pass behind it.
        let lanes = ((wf / 30.0).round() as i32).clamp(1, 8);
        let (first, last) = (hz + 3, ph - 1);
        let mut ranks: Vec<usize> = (0..lanes as usize).collect();
        rng.shuffle(&mut ranks);
        for (k, rank) in (0..lanes).zip(ranks) {
            let y = first + ((k as f32 + 0.5) / lanes as f32 * (last - first) as f32) as i32;
            let far = y - hz < depth * 45 / 100 || depth < 24;
            let dir = if rng.bool() { 1.0 } else { -1.0 };
            self.canoes.push(Canoe {
                x: rangef(rng, 0.0, wf),
                y: y.min(last),
                vx: dir * rangef(rng, 1.0, 2.5) * if far { 0.6 } else { 1.0 },
                far,
                phase: rangef(rng, 0.0, 6.3),
                rank,
            });
        }
        // Veladoras floating on the water, half of them around the island.
        self.floats.clear();
        let n = ((wf / 5.0).round() as usize).clamp(2, 60);
        for i in 0..n {
            let (x, y) = if i % 2 == 0 {
                (
                    cx + rangef(rng, -1.8, 1.8) * half,
                    range(rng, base - 2, base + depth / 3),
                )
            } else {
                (rangef(rng, 0.0, wf), range(rng, hz + 2, ph))
            };
            self.floats.push(Float {
                x,
                y: y.clamp(hz + 2, ph - 1),
                vx: rangef(rng, -0.4, 0.4),
                phase: rangef(rng, 0.0, 6.3),
            });
        }
    }

    /// Belén's night for this size, from its own RNG so that the layout only
    /// depends on the size and the seed: the star over the portal, the town
    /// stepping down the hill with its portal in front, three layers of
    /// dunes, the shepherds' fire and flock, and the palms.
    fn build_belen(&mut self) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (wf, hzf) = (w as f32, hz as f32);
        let mut rng = fastrand::Rng::with_seed(self.seed ^ 0x0B31_E412);
        self.town.clear();
        self.portal.clear();
        self.crests.clear();
        self.ripples.clear();
        self.palms.clear();
        self.flock.clear();
        self.camp = None;
        let cx = wf * rangef(&mut rng, 0.55, 0.7);
        self.star = (cx, hzf * 0.42);
        self.reyes_x = wf * 0.4;
        let depth = ph - hz;
        if !self.landscape || w < 16 || hz < 6 || depth < 6 {
            return;
        }
        // The town: flat-roofed houses stepping down the hill from its
        // crest, some with a dome, each with a window or two.
        let ground = |x: i32| self.far[x.clamp(0, w - 1) as usize];
        let span = (wf * 0.15).clamp(5.0, 36.0);
        for tier in 0..2 {
            let mut x = (cx - span) as i32 + tier * 2;
            while (x as f32) < cx + span {
                let bw = range(&mut rng, 3, 7);
                let bh = range(&mut rng, 3, 6);
                let (hi, lo) = (x..x + bw).fold((i32::MAX, i32::MIN), |(hi, lo), c| {
                    (hi.min(ground(c)), lo.max(ground(c)))
                });
                let top = hi - bh + 2 + tier * 3;
                let bottom = (top + bh).max(lo + 1 + tier * 2);
                for c in x..x + bw {
                    for y in top..=bottom {
                        let part = if y == top { 2 } else { 0 };
                        self.town.push(Spot {
                            x: c,
                            y,
                            part,
                            r: 1.0,
                        });
                    }
                }
                for _ in 0..range(&mut rng, 1, 3) {
                    let wx = x + range(&mut rng, 0, bw);
                    let wy = top + 1 + range(&mut rng, 0, (bottom - top - 1).max(1));
                    self.town.push(Spot {
                        x: wx,
                        y: wy,
                        part: 1,
                        r: rng.f32(),
                    });
                }
                if bw >= 5 && rng.f32() < 0.3 {
                    let r = bw as f32 / 2.0 - 0.5;
                    for dx in 1..bw - 1 {
                        let u = (dx as f32 + 0.5 - bw as f32 / 2.0) / r;
                        let rise = (r * (1.0 - u * u).max(0.0).sqrt()).round() as i32;
                        for dy in 1..=rise.max(1) {
                            self.town.push(Spot {
                                x: x + dx,
                                y: top - dy,
                                part: 2,
                                r: 1.0,
                            });
                        }
                    }
                }
                x += bw + range(&mut rng, 0, 2);
            }
        }
        // The portal in front of the town, a little down the slope.
        let sprite: &[&str] = if hz >= 40 { &PORTAL } else { &PORTAL_S };
        let (pw, pth) = (sprite[0].len() as i32, sprite.len() as i32);
        let base = (ground(cx as i32) + 9).min(hz - 1);
        for (r, row) in sprite.iter().enumerate() {
            for (k, ch) in row.bytes().enumerate() {
                let part = match ch {
                    b'r' => 4,
                    b'p' => 5,
                    b'h' => 6,
                    b'n' => 7,
                    b'j' => 8,
                    b'm' => 9,
                    _ => continue,
                };
                self.portal.push(Spot {
                    x: cx as i32 - pw / 2 + k as i32,
                    y: base - pth + 1 + r as i32,
                    part,
                    r: 0.0,
                });
            }
        }
        // Three layers of dunes, rolling, from the far to the near.
        let d = depth as f32;
        for (i, (off, amp)) in [(-1.0, 2.0), (0.3 * d, 0.09 * d), (0.62 * d, 0.1 * d)]
            .into_iter()
            .enumerate()
        {
            let wl = wf * rangef(&mut rng, 0.3, 0.7) / (1.0 + 0.4 * (2 - i) as f32);
            let (k1, p1) = (TAU / wl.max(8.0), rangef(&mut rng, 0.0, TAU));
            let p2 = rangef(&mut rng, 0.0, TAU);
            for x in 0..w {
                let (xf, k2) = (x as f32, k1 * 2.3);
                let v = ((k1 * xf + p1).sin() + 0.45 * (k2 * xf + p2).sin()) / 1.45;
                self.crests.push((hzf + off + amp * v).round() as i32);
            }
        }
        self.ripples = (0..ph * w)
            .map(|i| ((i % w) as f32 * 0.4 + (i / w) as f32 * 2.1).sin() > 0.93)
            .collect();
        let crest = |i: usize, x: i32| self.crests[i * w as usize + x.clamp(0, w - 1) as usize];
        // The shepherds' fire on the middle dunes, and the sheep around it.
        if depth >= 12 {
            let fx = (wf * rangef(&mut rng, 0.07, 0.17)) as i32;
            self.camp = Some((fx, (crest(1, fx) + 3).min(crest(2, fx) - 2).min(ph - 2)));
            let n = (w / 18).clamp(3, 9);
            let step = (wf * 0.22 / n as f32).clamp(6.0, 14.0);
            for k in 0..n {
                let x = fx + 8 + (k as f32 * step + rangef(&mut rng, -1.5, 1.5)) as i32;
                self.flock.push(Sheep {
                    x: x as f32,
                    y: (crest(1, x) + 2 + k % 3).min(crest(2, x) - 2),
                    phase: rangef(&mut rng, 0.0, TAU),
                });
            }
        }
        // Palms around the town and by the fire.
        let tall = (ph as f32 * 0.07).clamp(5.0, 18.0);
        for (x, i, h) in [
            (cx - wf * 0.24, 0, 0.8),
            (cx + wf * 0.2, 0, 1.0),
            (cx + wf * 0.27, 0, 0.7),
            (wf * 0.06, 1, 1.0),
            (wf * 0.36, 1, 1.15),
        ] {
            let x = (x as i32).clamp(1, w - 2);
            self.palms.push(Palm {
                x,
                y: crest(i, x) + 1,
                h: tall * h,
                lean: rangef(&mut rng, -2.0, 2.0),
            });
        }
    }

    /// The Fiestas Patrias' night for this size, from its own RNG so that the
    /// layout only depends on the size and the seed: Iztaccíhuatl and
    /// Popocatépetl across the valley, and the garlands and the flag on the
    /// near shore.
    fn build_volcanes(&mut self) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (wf, hzf) = (w as f32, hz as f32);
        let mut rng = fastrand::Rng::with_seed(self.seed ^ 0x9A71_1810);
        self.volc.clear();
        self.snowline.clear();
        self.bulbs.clear();
        self.flag = None;
        if !self.landscape || w < 24 || hz < 10 {
            return;
        }
        // Popocatépetl on the right, a cone cut by its crater; Iztaccíhuatl
        // on the left, lying on her back from the head to the feet. Where
        // they overlap the higher one wins: the saddle between them.
        let hp = (0.64 * hzf).min(0.27 * wf);
        let (bp, xp, hi) = (hp * 1.25, wf * 0.76, hp * 0.84);
        let (xi0, xi1) = (wf * 0.02, wf * 0.58);
        let crater = (0.05 * bp).max(2.0);
        let (p1, p2) = (rangef(&mut rng, 0.0, TAU), rangef(&mut rng, 0.0, TAU));
        for x in 0..w {
            let xf = x as f32 + 0.5;
            let dx = (xf - xp).abs();
            let mut vp = if dx < bp {
                hp * (1.0 - dx / bp).powf(1.3)
            } else {
                0.0
            };
            vp = vp.min(hp * 0.94);
            if dx < crater {
                vp -= 2.0 * (1.0 - dx / crater);
            }
            let u = (xf - xi0) / (xi1 - xi0);
            let mut vi = if (0.0..1.0).contains(&u) {
                hi * izta(u)
            } else {
                0.0
            };
            // Crags along her body.
            vi *= 1.0 + 0.02 * ((xf * 0.9 + p1).sin() + 0.6 * (xf * 2.3 + p2).sin());
            let (v, id) = if vp >= vi { (vp, 2) } else { (vi, 1) };
            if v < 1.0 {
                self.volc.push((hz, 0));
                self.snowline.push(hz);
            } else {
                self.volc.push((hz - v as i32, id));
                let alt = if id == 2 { 0.62 * hp } else { 0.5 * hi };
                let alt = alt * (1.0 + 0.16 * (xf * 0.6 + p1).sin() + 0.07 * (xf * 1.9).sin());
                self.snowline.push(hz - alt as i32);
            }
        }
        self.crater = (xp, hzf - hp * 0.94 + 1.0);
        // Garlands of lights between poles along the near shore.
        let section = (wf * 0.12).clamp(14.0, 34.0) as i32;
        let hang = (hzf * 0.07).clamp(3.0, 8.0);
        let shore = |x: i32| self.near[x.clamp(0, w - 1) as usize];
        let mut x0 = range(&mut rng, 0, section / 2);
        while x0 + section < w {
            let x1 = x0 + section;
            let (ya, yb) = (shore(x0) as f32 - hang - 3.0, shore(x1) as f32 - hang - 3.0);
            for (x, y0) in [(x0, ya), (x1, yb)] {
                for y in y0 as i32..=shore(x) {
                    self.bulbs.push(Bulb { x, y, c: 3, r: 0.0 });
                }
            }
            let mut x = x0 + 2;
            let mut k = range(&mut rng, 0, 3) as u8;
            while x < x1 - 1 {
                let s = (x - x0) as f32 / section as f32;
                let y = ya + (yb - ya) * s + 0.4 * hang * (PI * s).sin();
                self.bulbs.push(Bulb {
                    x,
                    y: y.round() as i32,
                    c: k % 3,
                    r: rng.f32(),
                });
                x += 3;
                k += 1;
            }
            x0 = x1 + range(&mut rng, 6, 20);
        }
        // The bandera on its pole, in front of Popocatépetl's foot.
        let fw = (ph as f32 * 0.09).clamp(7.0, 20.0) as i32;
        let fx = ((wf * 0.88) as i32).min(w - fw - 2).max(1);
        let base = shore(fx) + 1;
        let top = base - (hzf * 0.28).clamp(7.0, 26.0) as i32;
        self.flag = Some((fx, base, top, fw));
    }

    /// Half the width of the Calzada de los Muertos on row `y`: as wide as the
    /// stairs at the pyramid's foot, widening toward the viewer.
    fn road_half(&self, y: i32) -> f32 {
        let (ph, hz) = (self.ph as i32, self.horizon as i32);
        let f = ((y - hz - 2) as f32 / (ph - hz - 2).max(1) as f32).clamp(0.0, 1.0);
        let foot = self.stairs.last().map_or(2, |s| *s) as f32 + 1.0;
        foot + (0.36 * self.w as f32 - foot) * f
    }

    /// Where the sun rises as a share of the pyramid's half width: behind the
    /// summit on the 20th and 21st, a little to one side on the 19th and 22nd.
    fn sun_shift(&self) -> f32 {
        let f = self.fiesta;
        match f.date() {
            (3, 19) if f.is(Kind::Equinoccio) => -0.25,
            (3, 20) if f.is(Kind::Equinoccio) => -0.05,
            (3, 21) if f.is(Kind::Equinoccio) => 0.05,
            (3, 22) if f.is(Kind::Equinoccio) => 0.25,
            _ => 0.0,
        }
    }

    /// Teotihuacán at dawn for this size, from its own RNG so that the layout
    /// only depends on the size and the seed: the Pirámide del Sol with its
    /// stairs, the sun behind it, the temple platforms along the Calzada de
    /// los Muertos, the crowd and the magueyes.
    fn build_teotihuacan(&mut self) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (wf, hzf) = (w as f32, hz as f32);
        let mut rng = fastrand::Rng::with_seed(self.seed ^ 0x7E07_1204);
        self.pyr.clear();
        self.stairs.clear();
        self.platforms.clear();
        self.crowd.clear();
        self.magueyes.clear();
        self.sun = (0.0, 0.0, 0.0);
        let depth = ph - hz;
        if !self.landscape || w < 24 || hz < 12 || depth < 8 {
            return;
        }
        // The pyramid: four terraces, each sloping a little and then stepping
        // in, with a staircase up the middle.
        let xc = (wf * 0.5) as i32;
        let bw = (wf * 0.3).clamp(10.0, 80.0);
        let h = ((bw * 0.62).min(hzf * 0.55) as i32).max(6);
        let top = hz + 2 - h;
        for r in 0..=h {
            let t = r as f32 / (h + 1) as f32 * 4.0;
            let hw = bw * (0.26 + 0.74 * (t.floor() + 0.75 * t.fract()) / 4.0);
            self.pyr.push(hw.round() as i32);
            let st = bw * 0.17 * (0.6 + 0.4 * r as f32 / h as f32);
            self.stairs.push(st.round().max(1.0) as i32);
        }
        self.pyr_at = (xc, top);
        // The sun, behind the summit and above the hills.
        let r = (hzf * 0.07).clamp(3.0, 9.0);
        let ridge = self.far[(xc as usize).min(w as usize - 1)] as f32;
        self.sun = (
            xc as f32,
            (top as f32 - 3.0).min(ridge - 2.0).max(r + 2.0),
            r,
        );
        // Low temple platforms along both sides of the avenue, in perspective.
        for side in [-1, 1] {
            for k in 0..4 {
                let f = ((k as f32 + 0.5 + rangef(&mut rng, -0.2, 0.2)) / 4.0).powf(1.6);
                let y = hz + 3 + (f * (depth - 4) as f32) as i32;
                let sc = 0.3 + 0.9 * f;
                let pw = (wf * 0.13 * sc).max(3.0) as i32;
                let edge = self.road_half(y) as i32 + 2;
                let x = if side < 0 { xc - edge - pw } else { xc + edge };
                self.platforms.push(Platform {
                    x,
                    y,
                    w: pw,
                    h: (hzf * 0.13 * sc).max(2.0) as i32,
                });
            }
        }
        // The crowd, near and far on the avenue, up the stairs and on the
        // summit, the farthest drawn first.
        let near = (w / 16).clamp(4, 16);
        let far = (w / 6).clamp(6, 40);
        let climbing = (h / 2).clamp(3, 16);
        for k in 0..near + far + climbing + 3 {
            let (x, y, big) = if k < near {
                let y = ph - 2 - range(&mut rng, 0, (depth * 35 / 100).max(2));
                let half = self.road_half(y);
                (
                    (xc as f32 + rangef(&mut rng, -1.0, 1.0) * half) as i32,
                    y,
                    true,
                )
            } else if k < near + far {
                let y = hz + 4 + range(&mut rng, 0, (depth / 2).max(2));
                let half = self.road_half(y);
                (
                    (xc as f32 + rangef(&mut rng, -0.9, 0.9) * half) as i32,
                    y,
                    false,
                )
            } else if k < near + far + climbing {
                let row = range(&mut rng, 3, h + 1) as usize;
                let half = self.stairs[row] as f32;
                (
                    (xc as f32 + rangef(&mut rng, -0.8, 0.8) * half) as i32,
                    top + row as i32,
                    false,
                )
            } else {
                (xc + (k - near - far - climbing) * 3 - 3, top, false)
            };
            let r = rng.f32();
            self.crowd.push(Person {
                x,
                y,
                big: big && ph >= 80,
                r,
                phase: x as f32 * 0.08 + r * 3.0,
            });
        }
        self.crowd.sort_by_key(|p| p.y);
        // Magueyes in the foreground corners.
        let size = (ph as f32 * 0.07).clamp(3.0, 14.0);
        for (fx, k) in [(0.04, 1.0), (0.1, 0.7), (0.93, 1.0), (0.87, 0.7)] {
            self.magueyes.push(Maguey {
                x: (wf * fx) as i32,
                y: ph - 1 - range(&mut rng, 0, 2),
                size: size * k,
            });
        }
    }

    /// Paddles the canoes, drifts the veladoras and flies the monarchs `dt`
    /// seconds on, sending whoever leaves the screen around again from the
    /// other side.
    fn row(&mut self, dt: f32) {
        let (w, hz) = (self.w as f32, self.horizon as f32);
        let rng = &mut self.hrng;
        for c in &mut self.canoes {
            c.x += c.vx * dt;
            let len = if c.far {
                CANOE_FAR[0].len()
            } else {
                CANOE[0].len()
            } as f32;
            if c.vx > 0.0 && c.x > w + 1.0 {
                c.x = -len - rangef(rng, 0.0, 0.3 * w);
            } else if c.vx < 0.0 && c.x < -len - 1.0 {
                c.x = w + rangef(rng, 0.0, 0.3 * w);
            }
        }
        for v in &mut self.floats {
            v.x += v.vx * dt;
            if v.x < -1.0 {
                v.x += w + 2.0;
            } else if v.x > w + 1.0 {
                v.x -= w + 2.0;
            }
        }
        for m in &mut self.monarchs {
            m.x += m.vx * dt;
            m.phase += m.rate * dt;
            m.flap = (m.flap + dt * 9.0) % 4.0;
            let gone = if m.vx > 0.0 {
                m.x > w + 6.0
            } else {
                m.x < -6.0
            };
            if gone {
                let back = rangef(rng, 6.0, 6.0 + 0.6 * w);
                m.x = if m.vx > 0.0 { -back } else { w + back };
                m.y = rangef(rng, 0.06, 0.8) * hz;
            }
        }
    }

    /// Van Gogh's sky: a great double whirl and a smaller one in a sky
    /// flowing in bands, eleven ringed stars, a crescent moon, a cypress on
    /// the left and a village with its church on the hills.
    fn build_starry(&mut self) {
        let (w, ph, hz) = (self.w, self.ph, self.horizon);
        let (wf, hf) = (w as f32, hz.max(1) as f32);
        let r = (0.13 * wf).min(0.45 * hf).max(3.0);
        let whirls = [
            (0.40 * wf, 0.34 * hf, r, 1.0),
            (0.56 * wf, 0.42 * hf, r * 0.75, -1.0),
            (0.16 * wf, 0.22 * hf, r * 0.55, 1.0),
        ];
        let moon = (0.86 * wf, 0.18 * hf, (0.07 * hf).clamp(2.0, 8.0));
        let star_r = (0.035 * hf).clamp(1.0, 4.0);
        let mut stars: Vec<(f32, f32)> = Vec::new();
        let mut tries = 0;
        while stars.len() < 11 && tries < 400 {
            tries += 1;
            let (x, y) = (
                rangef(&mut self.rng, 0.04, 0.96) * wf,
                rangef(&mut self.rng, 0.06, 0.7) * hf,
            );
            let clear = |(ax, ay): (f32, f32), d: f32| (x - ax).hypot(y - ay) > d;
            if whirls
                .iter()
                .all(|&(wx, wy, wr, _)| clear((wx, wy), wr * 0.7))
                && clear((moon.0, moon.1), moon.2 * 4.0)
                && stars.iter().all(|&s| clear(s, star_r * 6.0))
            {
                stars.push((x, y));
            }
        }
        self.brush = vec![Brush::default(); w * hz];
        for y in 0..hz {
            for x in 0..w {
                let (xf, yf) = (x as f32, y as f32);
                let mut b = Brush {
                    flow: (yf * 0.55
                        + 2.2 * (xf * 0.045 + yf * 0.03).sin()
                        + 1.3 * (xf * 0.11 - yf * 0.07).sin())
                    .sin_cos(),
                    dab: ((hash(x as i32 / 3, y as i32, self.seed) % 100) as f32 / 100.0 - 0.5)
                        * 0.2,
                    ..Brush::default()
                };
                for &(wx, wy, wr, dir) in &whirls {
                    let (dx, dy) = (xf - wx, yf - wy);
                    let d = dx.hypot(dy);
                    let pull = (-(d / wr).powi(2)).exp();
                    if pull > b.pull {
                        b.pull = pull;
                        // A two-armed logarithmic spiral.
                        b.whirl = (6.0 * (d + 1.0).ln() - dir * 2.0 * dy.atan2(dx)).sin_cos();
                    }
                }
                let (dmx, dmy) = (xf - moon.0, yf - moon.1);
                let dm = dmx.hypot(dmy);
                if dm < moon.2 * 3.2 {
                    b.moon = true;
                    b.ring = dm * 1.4;
                    b.halo = (1.0 - dm / (moon.2 * 3.2)).powf(0.6);
                    // Lit except where a second disc shades it.
                    let shade = (xf - moon.0 - 0.55 * moon.2).hypot(yf - moon.1 + 0.2 * moon.2);
                    if dm < moon.2 && shade > moon.2 * 0.85 {
                        b.core = 2;
                    }
                }
                for &(sx, sy) in &stars {
                    let d = (xf - sx).hypot(yf - sy);
                    let halo = (1.0 - d / (star_r * 3.5)).max(0.0).powf(0.7) * 0.85;
                    if halo > b.halo {
                        b.moon = false;
                        b.halo = halo;
                        b.ring = d * 1.6;
                        b.core = u8::from(d < star_r);
                    }
                }
                self.brush[y * w + x] = b;
            }
        }
        // The cypress, a dark flame from the bottom of the screen nearly to the top.
        self.cypress.clear();
        let (cx, top) = (0.13 * wf, (ph as f32 * 0.04) as usize);
        let girth = (0.07 * wf).max(0.06 * ph as f32);
        for y in top..ph {
            let yf = y as f32;
            let u = (yf - top as f32) / (ph - top).max(1) as f32;
            // Widest low down, a point at the top, and tongues of flame along the sides.
            let half = girth
                * (PI * (u * 1.4).min(1.0) / 2.0).sin().powf(0.8)
                * (0.8 + 0.2 * (yf * 0.55 + 1.3).sin());
            let lean = 0.035 * wf * (u * PI * 1.5).sin();
            let mid = cx + lean;
            let (x0, x1) = ((mid - half).round() as i32, (mid + half).round() as i32);
            for x in x0.max(0)..=x1.min(w as i32 - 1) {
                let xf = x as f32 - mid;
                // Strokes running up the tree and curling with it.
                let stroke = 0.5 + 0.5 * (xf * 1.1 + yf * 0.12 + 1.5 * (yf * 0.08).sin()).sin();
                self.cypress.push((y * w + x as usize, stroke * stroke));
            }
        }
        // The village on the near hills, its church in the middle.
        self.village.clear();
        if !self.landscape || w < 12 {
            return;
        }
        let (church, mut built) = ((0.58 * wf) as i32, false);
        let mut x = (0.3 * wf) as i32;
        while x < (0.92 * wf) as i32 {
            let is_church = !built && x + 9 > church;
            built |= is_church;
            let (bw, bh) = if is_church {
                (5, 5)
            } else {
                (range(&mut self.rng, 3, 7), range(&mut self.rng, 2, 4))
            };
            let foot = self.near[(x + bw / 2).clamp(0, w as i32 - 1) as usize] + 2;
            let put = |px: i32, py: i32, part: u8, v: &mut Vec<(usize, u8)>| {
                if px >= 0 && (px as usize) < w && py >= 0 && (py as usize) < hz {
                    v.push((py as usize * w + px as usize, part));
                }
            };
            for dy in 0..bh {
                for dx in 0..bw {
                    let window = dy > 0 && hash(x + dx, dy, self.seed ^ 0x77).is_multiple_of(4);
                    put(
                        x + dx,
                        foot - dy,
                        if window { 2 } else { 0 },
                        &mut self.village,
                    );
                }
            }
            // A roof narrowing to its ridge.
            for k in 0..=bw / 2 {
                for dx in k..bw - k {
                    put(x + dx, foot - bh - k, 1, &mut self.village);
                }
            }
            if is_church {
                let spire = (ph as i32 / 9).max(3);
                for k in 0..spire {
                    put(x + bw / 2, foot - bh - bw / 2 - k, 1, &mut self.village);
                }
            }
            x += bw + range(&mut self.rng, 1, 4);
        }
    }

    fn curtain(&mut self) -> Curtain {
        let (w, span) = (self.w as f32, self.span);
        let rng = &mut self.rng;
        let len = w * rangef(rng, 0.5, 1.1);
        // The first ripple is strong enough (amplitude x frequency > 1) to
        // make the track double back on itself: those are the folds.
        let k0 = rangef(rng, 0.02, 0.05) * 80.0 / w.max(20.0);
        let a0 = rangef(rng, 0.9, 1.6) / k0;
        Curtain {
            x0: rangef(rng, -0.2 * w, 0.7 * w),
            len,
            base: span * rangef(rng, 0.2, 0.38),
            tall: span * rangef(rng, 0.1, 0.2),
            ripples: [
                (a0.min(w * 0.3), k0, rangef(rng, 0.004, 0.012)),
                (
                    rangef(rng, 2.0, 6.0),
                    rangef(rng, 0.08, 0.2),
                    rangef(rng, -0.03, 0.03),
                ),
                (
                    rangef(rng, 0.5, 2.0),
                    rangef(rng, 0.3, 0.6),
                    rangef(rng, 0.02, 0.06),
                ),
            ],
            drift: rangef(rng, -0.03, 0.03),
            bright: rangef(rng, 0.6, 1.0),
            age: 0,
            life: range(rng, 900, 2200) as u32,
            phase: rangef(rng, 0.0, 6.3),
        }
    }

    /// One tanim tick of everything but the clock of the ripples, which
    /// `update` advances smoothly.
    fn step(&mut self) {
        let quick = 1.0 + 1.5 * self.surge;
        self.surge *= 0.994;
        if self.rng.f32() < 0.0008 {
            self.surge = 1.0;
        }
        for i in 0..self.curtains.len() {
            let c = &mut self.curtains[i];
            c.age += 1;
            c.x0 += c.drift * quick;
            if c.age >= c.life {
                self.curtains[i] = self.curtain();
            }
        }
        match &mut self.meteor {
            Some(m) => {
                m.x += m.vx;
                m.y += m.vy;
                m.age += 1;
                if m.age > 26 {
                    self.meteor = None;
                }
            }
            None => {
                if self.show_stars && self.rng.f32() < 0.003 {
                    let right = self.rng.bool();
                    self.meteor = Some(Meteor {
                        x: rangef(&mut self.rng, 0.1, 0.9) * self.w as f32,
                        y: rangef(&mut self.rng, 0.0, 0.3) * self.horizon as f32,
                        vx: if right { 2.2 } else { -2.2 },
                        vy: 0.9,
                        age: 0,
                    });
                }
            }
        }
    }

    /// Sends Nyan Cat or the train across now and then, `real` seconds later.
    fn visit(&mut self, real: f32) {
        if let Some(v) = &mut self.visitor {
            v.update(real);
            self.visiting += real;
            if v.done() || self.visiting > VISIT_SECS {
                self.visitor = None;
            }
            return;
        }
        let rate = self.visitors as f64;
        if rate <= 0.0 || self.cols == 0 {
            return;
        }
        if rate >= eggs::FORCE_RATE || self.rng.f64() < 1.0 - (-rate / 3600.0 * real as f64).exp() {
            // During a holiday its own visitors come by most of the time.
            let (kind, today) = (self.fiesta.kind, self.fiesta);
            let ours = || {
                eggs::ALL
                    .iter()
                    .filter(move |i| i.fiesta == kind && (i.when)(today))
            };
            let seasonal = ours().count();
            let dressed = fiesta_look(kind).is_some();
            let info = if dressed && seasonal > 0 && self.hrng.f32() < 0.8 {
                ours()
                    .nth(self.hrng.usize(0..seasonal))
                    .unwrap_or(&&eggs::nyan::INFO)
            } else if self.rng.bool() {
                &eggs::nyan::INFO
            } else {
                &eggs::sl::INFO
            };
            self.visitor = Some((info.start)(self.cols, self.rows, &mut self.rng));
            self.visiting = 0.0;
        }
    }

    /// Accumulates every curtain's light into `glow`.
    fn draw_curtains(&mut self, look: &Look) {
        self.glow.fill([0.0; 3]);
        let (w, hz) = (self.w, self.horizon);
        let t = self.t;
        let surge = 1.0 + 1.2 * self.surge;
        for ci in 0..self.curtains.len() {
            self.light.fill(0.0);
            self.edge.fill(0.0);
            let c = &self.curtains[ci];
            let fade = {
                let a = c.age as f32 / c.life as f32;
                (a * 6.0).min(1.0).min((1.0 - a) * 6.0).max(0.0)
            };
            let bright =
                c.bright * fade * surge * look.lights * (0.75 + 0.25 * (t * 0.013 + c.phase).sin());
            if bright <= 0.01 {
                continue;
            }
            let steps = (c.len * 3.0) as usize;
            let (base, tall) = (c.base, c.tall);
            let reach = tall * 3.0 * look.reach;
            for i in 0..steps {
                let s = i as f32 / 3.0;
                let mut x = c.x0 + s;
                for &(amp, k, speed) in &c.ripples {
                    x += amp * (k * s + speed * t + c.phase).sin();
                }
                // Taper both ends of the curtain.
                let u = s / c.len;
                let wgt = (PI * u).sin().max(0.0).sqrt() / 3.0;
                let e = base + 0.03 * self.span * (0.05 * s + 0.01 * t + c.phase).sin();
                let xi = x.floor();
                let fr = x - xi;
                for (xx, part) in [(xi as i32, 1.0 - fr), (xi as i32 + 1, fr)] {
                    if xx >= 0 && (xx as usize) < w {
                        self.light[xx as usize] += wgt * part;
                        self.edge[xx as usize] += e * wgt * part;
                    }
                }
            }
            // The glow by distance d above the edge, from 3 below it to the
            // top, in eighths of a pixel: sharp lower edge, long exponential
            // fade upwards.
            self.profile.clear();
            let n = ((tall * 3.5 + 3.0) * PROFILE_RES) as usize + 2;
            for k in 0..n {
                let d = k as f32 / PROFILE_RES - 3.0;
                let prof = if d < 0.0 {
                    (d * 1.2).exp()
                } else {
                    (-d / tall).exp() * (1.0 - (-(d + 1.0) / 1.5).exp())
                };
                let u = (d / reach).clamp(0.0, 1.0);
                self.profile.push((prof, ramp(&look.glow, u)));
            }
            for x in 0..w {
                let l = self.light[x];
                if l < 1e-3 {
                    continue;
                }
                let edge = self.edge[x] / l;
                // Vertical rays: the light shimmers column by column.
                let xf = x as f32;
                let rays = 0.7 + 0.3 * (xf * 0.9 + t * 0.07).sin() * (xf * 0.23 - t * 0.031).sin();
                let power = l.min(3.0) * bright * rays;
                let top = (edge - tall * 3.5).max(0.0) as usize;
                let bottom = ((edge + 3.0).max(0.0) as usize).min(hz);
                // The prism's hue for this column, drifting along the sky.
                let hue = xf * 0.011 + t * 0.002 + c.phase * 0.1;
                for y in top..bottom {
                    let d = edge - y as f32;
                    let k = (((d + 3.0) * PROFILE_RES) as usize).min(n - 1);
                    let (prof, mut col) = self.profile[k];
                    if look.prism > 0.01 {
                        let u = (d / reach).clamp(0.0, 1.0);
                        let h = hue + u * 0.35;
                        let p = if self.paint.prism.is_empty() {
                            hsv(h, 0.75, 1.0)
                        } else {
                            let n = self.paint.prism.len();
                            let k = (h - h.floor()) * n as f32;
                            let i = k as usize % n;
                            mix(
                                self.paint.prism[i],
                                self.paint.prism[(i + 1) % n],
                                k.fract(),
                            )
                        };
                        col = mix(col, p, look.prism);
                    }
                    let a = prof * power * 0.9;
                    let g = &mut self.glow[y * w + x];
                    *g = add(*g, scale(col, a));
                }
            }
        }
    }

    /// Paints the starry night's sky over row `y`, weighted by `s`.
    fn paint_starry_row(&mut self, y: usize, s: f32) {
        let w = self.w;
        // The paint flows in steps of two ticks, plenty for brushstrokes and
        // half the cells to resend.
        let t = (self.t * 0.5).floor() * 2.0;
        let p = &self.paint;
        let (flow_t, whirl_t, ring_t) = ((t * 0.02).sin_cos(), (t * 0.035).sin_cos(), t * 0.06);
        for x in 0..w {
            let i = y * w + x;
            let b = self.brush[i];
            // sin(a - t) without a sine per pixel: a's is kept, t's is shared.
            let (sf, cf) = b.flow;
            let band_f = sf * flow_t.1 - cf * flow_t.0;
            let band = if b.pull > 0.02 {
                let (sw, cw) = b.whirl;
                let band_w = sw * whirl_t.1 - cw * whirl_t.0;
                band_f + (band_w - band_f) * b.pull
            } else {
                band_f
            };
            let mut col = ramp(&p.strokes, 0.42 + 0.4 * band + b.dab);
            if b.halo > 0.0 {
                let rings = if b.moon { &p.moon_rings } else { &p.star_rings };
                let ring = 0.5 + 0.5 * (b.ring - ring_t).sin();
                col = mix(col, mix(rings[0], rings[1], ring), b.halo);
            }
            match b.core {
                1 => col = p.star_core,
                2 => col = p.moon_core,
                _ => {}
            }
            self.px[i] = mix(self.px[i], col, s);
        }
    }

    /// Paints sky, stars, aurora, land and lake into `px`. Over a transparent
    /// sky, `bg` is the terminal's background when known, which the lake's
    /// reflections of the land fade toward.
    fn compose(&mut self, look: &Look, bg: Option<V3>) {
        let (w, ph, hz) = (self.w, self.ph, self.horizon);
        let t = self.t;
        let s = look.starry;
        // Sky, stars and aurora light.
        for y in 0..hz {
            let base = if self.paint_sky {
                mix(look.sky_top, look.sky_low, y as f32 / hz.max(1) as f32)
            } else {
                [0.0; 3]
            };
            self.px[y * w..(y + 1) * w].fill(base);
            if s > 0.01 {
                self.paint_starry_row(y, s);
            }
        }
        self.solid.fill(self.paint_sky);
        if self.show_stars && s < 0.99 {
            for st in &self.stars {
                let tw = 0.7 + 0.3 * (t * 0.05 + st.phase).sin();
                let i = st.y as usize * w + st.x as usize;
                self.px[i] = mix(self.px[i], look.star, st.b * tw * (1.0 - s));
            }
        }
        if look.moon_k > 0.01 && hz > 4 {
            // A crescent: a disc with a second one shading its left side.
            let (mx, my) = (0.82 * w as f32, 0.2 * hz as f32);
            let r = (0.09 * hz as f32).clamp(2.0, 9.0);
            let reach = (r * 3.0).ceil() as i32;
            for y in (my as i32 - reach).max(0)..(my as i32 + reach).min(hz as i32) {
                for x in (mx as i32 - reach).max(0)..(mx as i32 + reach).min(w as i32) {
                    let (xf, yf) = (x as f32 + 0.5, y as f32 + 0.5);
                    let d = (xf - mx).hypot(yf - my);
                    let shade = (xf - mx + 0.6 * r).hypot(yf - my + 0.25 * r);
                    let i = y as usize * w + x as usize;
                    if d < r && shade > r * 0.9 {
                        self.px[i] = mix(self.px[i], look.moon, look.moon_k);
                    } else {
                        let halo = (1.0 - d / (r * 3.0)).max(0.0).powi(2) * 0.18;
                        self.px[i] = add(self.px[i], scale(look.moon, halo * look.moon_k));
                    }
                }
            }
        }
        if let Some(m) = &self.meteor {
            // Between ticks, so it streaks smoothly at any frame rate.
            let ahead = self.acc / STEP;
            let (mx, my) = (m.x + m.vx * ahead, m.y + m.vy * ahead);
            for k in 0..8 {
                let x = (mx - m.vx * k as f32 * 0.8) as i32;
                let y = (my - m.vy * k as f32 * 0.8) as i32;
                if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < hz {
                    let i = y as usize * w + x as usize;
                    let a = (1.0 - k as f32 / 8.0) * (1.0 - m.age as f32 / 27.0);
                    self.px[i] = mix(self.px[i], mix(look.star, [1.0; 3], 0.6), a);
                }
            }
        }
        if look.teotihuacan > 0.01 && self.sun.2 > 0.0 {
            self.rising_sun(look);
        }
        let mut ambient = [0.0f32; 3];
        for (p, g) in self.px.iter_mut().zip(&self.glow) {
            *p = add(*p, *g);
            ambient = add(ambient, *g);
        }
        if look.volcanes > 0.01 && hz > 4 {
            self.fireworks(look);
        }
        if !self.landscape {
            return;
        }
        // The aurora faintly lights the snow and the mountainsides.
        let n = (w * hz).max(1) as f32;
        let amb = ambient.map(|a| (a / n * 1.5).min(60.0 / 255.0));
        self.amb = amb;
        let lift = if self.paint_sky { 1.0 } else { LIFT };
        let (far, near, snow) = (
            scale(look.far, lift),
            scale(look.near, lift),
            scale(look.snow, lift.min(1.2)),
        );
        // Snow caps only the peaks that rise above the snow line.
        let snowline = hz as f32 - ph as f32 * 0.17;
        for x in 0..w {
            let (f, nr) = (self.far[x].max(0) as usize, self.near[x].max(0) as usize);
            let cap = ((snowline - f as f32) * 0.7).max(0.0) as usize;
            for y in f..hz {
                let mut base = if y < f + cap { snow } else { far };
                if y == f {
                    base = mix(base, look.rim, look.rim_k);
                }
                self.px[y * w + x] = add(base, amb);
                self.solid[y * w + x] = true;
            }
            if look.volcanes > 0.01 {
                self.volcano_column(x, nr.min(hz), look.volcanes, lift.min(1.5), amb);
            }
            for y in nr..hz {
                self.px[y * w + x] = add(near, scale(amb, 1.0 / 3.0));
                self.solid[y * w + x] = true;
            }
        }
        let pine = scale(look.pine, lift);
        for &(x, hgt) in &self.pines {
            let foot = self.near[(x.max(0) as usize).min(w - 1)] + 1;
            // The desert has no forest: the pines shrink away with the look.
            let hgt = (hgt as f32 * look.forest).round() as i32;
            for k in 0..hgt {
                let half = (hgt - k) / 3;
                for dx in -half..=half {
                    let (xx, yy) = (x + dx, foot - k);
                    if xx >= 0 && (xx as usize) < w && yy >= 0 && (yy as usize) < ph {
                        let i = yy as usize * w + xx as usize;
                        self.px[i] = pine;
                        self.solid[i] = true;
                    }
                }
            }
        }
        if s > 0.01 {
            for &(i, part) in &self.village {
                self.px[i] = mix(
                    self.px[i],
                    scale(self.paint.village[part as usize], lift.min(1.3)),
                    s,
                );
                self.solid[i] |= s >= 0.5;
            }
        }
        if look.volcanes > 0.01 {
            self.light_volcanes(look, lift.min(1.5));
        }
        // The lake: the scene above, flipped, rippled and darkened.
        let depth = (ph - hz).max(1) as f32;
        for y in hz..ph {
            let d = (y - hz) as f32 / depth;
            let src_y = hz as i32 - 1 - (y - hz) as i32;
            let dim = 0.6 - 0.3 * d;
            // Rows ripple sideways, more as the water comes closer.
            let wobble = (y as f32 * 0.7 + t * 0.09).sin() * (0.4 + 2.2 * d);
            for x in 0..w {
                let i = y * w + x;
                let glint = (x as f32 * 0.11 + y as f32 * 1.7 + t * 0.05).sin() > 0.985;
                if hz == 0 {
                    self.px[i] = if self.paint_sky { look.water } else { [0.0; 3] };
                    continue;
                }
                let sx = (x as f32 + wobble).round().clamp(0.0, w as f32 - 1.0) as usize;
                let sy = src_y.clamp(0, hz as i32 - 1) as usize;
                let (a, b) = (sy * w + sx, sy.saturating_sub(1) * w + sx);
                // Average two rows so stars soften instead of sparkling noise;
                // over a transparent sky only light mixes with light.
                let src = if self.solid[a] == self.solid[b] {
                    mix(self.px[a], self.px[b], 0.5)
                } else {
                    self.px[a]
                };
                let (c, solid) = if self.paint_sky {
                    (mix(look.water, src, dim), true)
                } else if self.solid[a] {
                    match bg {
                        Some(bg) => (mix(bg, src, dim), true),
                        None => (scale(src, dim), true),
                    }
                } else {
                    (scale(src, dim), false)
                };
                // Glints on the ripples.
                self.px[i] = match (glint, solid) {
                    (false, _) => c,
                    (true, true) => mix(c, look.glint, 0.25),
                    (true, false) => add(c, scale(look.glint, 0.25)),
                };
                self.solid[i] = solid;
            }
        }
        if s > 0.01 {
            let cyp = self.paint.cypress;
            for &(i, stroke) in &self.cypress {
                self.px[i] = mix(
                    self.px[i],
                    scale(mix(cyp[0], cyp[1], stroke), lift.min(1.3)),
                    s,
                );
                self.solid[i] |= s >= 0.5;
            }
        }
        if look.teotihuacan > 0.01 {
            self.paint_teotihuacan(look, lift.min(1.5));
        }
    }

    /// Pátzcuaro's night over the picture: Janitzio with its candles, the
    /// canoes, everything mirrored in the lake, and the monarchs above.
    fn draw_patzcuaro(&mut self, look: &Look, bg: Option<V3>) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (f, level) = (self.fest, self.level(Kind::Muertos));
        let mut l = Layer {
            px: &mut self.px,
            solid: &mut self.solid,
            w,
            h: ph,
            m: look.muertos,
            lift: if self.paint_sky { 1.0 } else { LIFT.min(1.3) },
            water: if self.paint_sky { Some(look.water) } else { bg },
            t: self.t,
            hz,
        };
        // The canoes, far to near, more of them as the holiday comes; those
        // beyond Janitzio pass behind it.
        let shown = ((self.canoes.len() as f32 * (0.3 + 0.7 * level)).round() as usize).max(1);
        let base = self.isle_base;
        let canoes = || self.canoes.iter().filter(|c| c.rank < shown);
        let shown_floats = (self.floats.len() as f32 * (0.2 + 0.8 * level)).ceil() as usize;
        let floats = || self.floats.iter().take(shown_floats);
        for v in floats().filter(|v| v.y <= base) {
            l.veladora(v, &f);
        }
        for c in canoes().filter(|c| c.y <= base) {
            l.canoe(c, &f);
        }
        l.janitzio(&self.isle, base, &f, 0.45 + 0.55 * level);
        for v in floats().filter(|v| v.y > base) {
            l.veladora(v, &f);
        }
        for c in canoes().filter(|c| c.y > base) {
            l.canoe(c, &f);
        }
        // Monarchs on their way south, over everything.
        let shown = (self.monarchs.len() as f32 * (0.3 + 0.7 * level)).ceil() as usize;
        for mo in &self.monarchs[..shown.min(self.monarchs.len())] {
            let glide = (mo.phase * 1.7).sin() > 0.4;
            let frame = &MONARCH[if glide { 0 } else { mo.flap as usize % 4 }];
            let x0 = mo.x.round() as i32 - 3;
            let y0 = (mo.y + mo.bob * mo.phase.sin()).round() as i32 - 2;
            for (r, row) in frame.iter().enumerate() {
                for (k, ch) in row.bytes().enumerate() {
                    let col = match ch {
                        b'O' => f.wing[0],
                        b'o' => f.wing[1],
                        b'k' => f.vein,
                        _ => continue,
                    };
                    l.lamp(x0 + k as i32, y0 + r as i32, col);
                }
            }
        }
    }

    /// Belén's night over the picture: dunes in place of the lake, the town
    /// on its hills with the portal below it, palms, the shepherds' fire and
    /// flock, the Reyes Magos on their way and, over it all, the Estrella
    /// de Belén.
    fn draw_belen(&mut self, look: &Look) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (n, f, level) = (self.nac, self.fest, self.level(Kind::Navidad));
        let (way, nino) = (self.reyes_way(), self.nino());
        let (wf, sc) = (w as f32, (ph as f32 / 120.0).clamp(0.45, 1.3));
        let mut l = Layer {
            px: &mut self.px,
            solid: &mut self.solid,
            w,
            h: ph,
            m: look.belen,
            lift: if self.paint_sky { 1.0 } else { LIFT.min(1.3) },
            water: None,
            t: self.t,
            hz,
        };
        let (sx, sy) = self.star;
        if !self.crests.is_empty() {
            // The dunes take the lake's place, their slopes lit where they
            // face the star.
            let wu = self.w;
            let glow = scale(self.amb, 0.5);
            for x in 0..wu {
                for i in 0..3 {
                    let row = &self.crests[i * wu..(i + 1) * wu];
                    let top = row[x];
                    let bottom = if i < 2 {
                        self.crests[(i + 1) * wu + x]
                    } else {
                        ph
                    };
                    // How steeply the dune faces the star (negative: away).
                    let slope = row[(x + 1).min(wu - 1)] - row[x.saturating_sub(1)];
                    let face =
                        (if sx > x as f32 { slope } else { -slope }).clamp(-3, 3) as f32 / 3.0;
                    for y in top.max(0)..bottom.min(ph) {
                        let d = (y - top) as f32;
                        let mut k = (1.0 - 0.015 * d).max(0.65);
                        if d < 4.0 {
                            k *= 1.0 + (0.3 * face + 0.12) * (1.0 - d / 4.0);
                        }
                        if self.ripples[y as usize * wu + x] {
                            k *= 1.18;
                        }
                        l.lamp(x as i32, y, add(scale(n.dune[i], k * l.lift), glow));
                    }
                }
            }
            for p in &self.palms {
                l.palm(p, &n);
            }
            // The town's lamps glow over its hills; more are lit as the
            // posadas go by.
            let lit = 0.25 + 0.75 * level;
            let ridge = self.far[(sx as usize).min(self.w - 1)];
            l.halo(
                sx as i32,
                ridge,
                (wf * 0.2) as i32,
                (hz / 6).max(2),
                n.lamp[0],
                0.1 + 0.12 * level,
            );
            for p in &self.town {
                match p.part {
                    1 if p.r < lit => {
                        let k = 0.8 + 0.2 * (l.t * 0.2 + p.r * 50.0).sin();
                        l.lamp(p.x, p.y, scale(n.lamp[0], k));
                        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                            l.shine(p.x + dx, p.y + dy, n.lamp[0], 0.12 * k);
                        }
                    }
                    2 => l.put(p.x, p.y, n.adobe[1]),
                    _ => l.put(p.x, p.y, n.adobe[0]),
                }
            }
            // The portal: the Niño Dios lies in his manger from Nochebuena,
            // and a ray of the star comes down to him.
            let mut crib = None;
            for p in &self.portal {
                match p.part {
                    4 => l.put(p.x, p.y, n.roof),
                    5 => l.put(p.x, p.y, scale(n.roof, 0.55)),
                    6 => l.put(p.x, p.y, scale(n.straw, 0.7)),
                    7 => {
                        crib = Some((p.x, p.y));
                        let k = 0.9 + 0.1 * (l.t * 0.15).sin();
                        if nino {
                            l.halo(p.x, p.y, 4, 3, n.lamp[1], 0.4 * k);
                            l.lamp(p.x, p.y, scale(n.lamp[1], k));
                        } else {
                            l.lamp(p.x, p.y, scale(n.lamp[0], 0.35));
                        }
                    }
                    8 => l.put(p.x, p.y, n.cloak),
                    _ => l.lamp(p.x, p.y, scale(n.robe[2], 0.8)),
                }
            }
            if let (Some((bx, by)), true) = (crib, nino) {
                for y in sy as i32 + 6..by - 2 {
                    let half = 1.0 + (y as f32 - sy) * 0.04;
                    for dx in -(half as i32 + 1)..=half as i32 + 1 {
                        let a = 0.09 * (1.0 - dx.abs() as f32 / (half + 1.0)).max(0.0);
                        l.shine(bx + dx, y, n.star[1], a);
                    }
                }
            }
            // The shepherds' fire, a shepherd by it and the flock around.
            if let Some((cx, cy)) = self.camp {
                let (fl, k) = flicker(&f.flame, l.t, 3.0);
                l.halo(
                    cx,
                    cy - 1,
                    (10.0 * sc) as i32,
                    (6.0 * sc) as i32,
                    fl,
                    0.6 * k,
                );
                let logs = l.dark(n.roof);
                l.stroke(
                    (cx as f32 - 2.5, cy as f32 + 0.5),
                    (cx as f32 + 2.5, cy as f32 - 0.2),
                    1.2,
                    logs,
                );
                for (dx, dy, b) in [
                    (0, -1, 1.0),
                    (-1, -1, 0.8),
                    (1, -1, 0.8),
                    (0, -2, 0.9),
                    (0, -3, 0.6),
                ] {
                    let c = if dy < -1 { f.flame[1] } else { fl };
                    l.lamp(cx + dx, cy + dy, scale(c, k * b));
                }
                // The shepherd stands by the fire with his staff.
                let hx = cx as f32 - 6.0 * sc;
                let (cloak, staff) = (l.dark(n.cloak), l.dark(n.roof));
                l.stroke(
                    (hx + 2.8 * sc, cy as f32 + 0.5),
                    (hx + 2.8 * sc, cy as f32 - 10.0 * sc),
                    1.0,
                    staff,
                );
                l.ellipse(hx, cy as f32 - 2.2 * sc, 1.9 * sc, 2.8 * sc, cloak);
                l.ellipse(hx, cy as f32 - 5.8 * sc, 1.1 * sc, 1.1 * sc, n.skin);
                let shown = (self.flock.len() as f32 * (0.4 + 0.6 * level)).ceil() as usize;
                for sh in self.flock.iter().take(shown) {
                    let x = sh.x + 1.2 * (l.t * 0.02 + sh.phase).sin();
                    let y = sh.y as f32;
                    l.stroke(
                        (x - 1.2 * sc, y + 1.0),
                        (x - 1.2 * sc, y + 2.4 * sc),
                        1.0,
                        cloak,
                    );
                    l.stroke(
                        (x + 1.2 * sc, y + 1.0),
                        (x + 1.2 * sc, y + 2.4 * sc),
                        1.0,
                        cloak,
                    );
                    l.ellipse(x, y, 2.6 * sc, 1.6 * sc, n.wool);
                    l.ellipse(
                        x + 2.8 * sc,
                        y - 0.2 * sc,
                        1.0 * sc,
                        1.0 * sc,
                        scale(cloak, 0.45),
                    );
                }
            }
            // The Reyes Magos, a little closer each day of January: far on
            // the dunes and small on the 2nd, at the portal on the 6th.
            if let Some(p) = way.filter(|_| ph - hz >= 14) {
                let s = sc * (0.32 + 0.58 * p);
                let foot =
                    (hz as f32 + 5.0 + p.powf(0.8) * ((ph - hz) as f32 - 8.0)).min(ph as f32 - 2.0);
                let (gap, arrived) = (25.0 * s, p >= 0.999);
                let lead = if arrived { sx + gap } else { self.reyes_x };
                for k in 0..3 {
                    let at = (lead - gap * k as f32, foot);
                    l.rey(k, at, s, self.gait, if arrived { 0.0 } else { 1.0 }, &n);
                }
            }
        }
        // The Estrella de Belén, its tail longer and its light brighter as
        // the holiday comes.
        let reach = (0.4 * wf).min(sx - 2.0) * (0.55 + 0.45 * level);
        let from = (sx - reach, sy - (0.45 * reach).min(sy - 3.0));
        l.estrella((sx, sy), from, 0.6 + 0.4 * level, n.star);
    }

    /// Walks the Reyes Magos `dt` seconds on, sending them around again from
    /// the left once they have crossed.
    fn stroll(&mut self, dt: f32) {
        self.gait = (self.gait + dt * 6.0) % TAU;
        let Some(p) = self.reyes_way().filter(|p| *p < 0.999) else {
            return;
        };
        let s = (self.ph as f32 / 120.0).clamp(0.45, 1.3) * (0.32 + 0.58 * p);
        self.reyes_x += dt * 5.0 * s;
        if self.reyes_x - 50.0 * s > self.w as f32 + 14.0 * s {
            self.reyes_x = -14.0 * s;
        }
    }

    /// The sun rising behind the Pirámide del Sol, before the land is drawn
    /// over it.
    fn rising_sun(&mut self, look: &Look) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (xc, y, r) = self.sun;
        let bw = self.pyr.last().map_or(0.0, |v| *v as f32);
        let x = xc + self.sun_shift() * bw;
        let (sun, k) = (self.piramide.sun, 0.7 + 0.3 * self.level(Kind::Equinoccio));
        let mut l = Layer {
            px: &mut self.px,
            solid: &mut self.solid,
            w,
            h: ph,
            m: look.teotihuacan,
            lift: 1.0,
            water: None,
            t: self.t,
            hz,
        };
        l.sun((x, y), r, k, sun);
    }

    /// The ground in place of the lake, the Calzada de los Muertos running to
    /// the foot of the Pirámide del Sol between low temple platforms, the
    /// pyramid itself in contraluz with the gold of the sun on its edges, the
    /// magueyes and the crowd in white, arms raised.
    fn paint_teotihuacan(&mut self, look: &Look, lift: f32) {
        if self.pyr.is_empty() {
            return;
        }
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (n, level) = (self.piramide, self.level(Kind::Equinoccio));
        let ((xc, top), seed) = (self.pyr_at, self.seed);
        let mut l = Layer {
            px: &mut self.px,
            solid: &mut self.solid,
            w,
            h: ph,
            m: look.teotihuacan,
            lift,
            water: None,
            t: self.t,
            hz,
        };
        // The ground, the paving between its joints and the kerb.
        let depth = (ph - hz).max(2) as f32;
        let joints: [i32; 12] = std::array::from_fn(|k| {
            hz + 2 + ((depth - 2.0) * ((k + 1) as f32 / 12.0).powf(1.8)) as i32
        });
        let (foot, wide) = (
            self.stairs.last().map_or(2, |s| *s) as f32 + 1.0,
            0.36 * w as f32,
        );
        for y in (hz - 1).max(0)..ph {
            let f = ((y - hz - 2) as f32 / (depth - 2.0)).clamp(0.0, 1.0);
            let half = foot + (wide - foot) * f;
            let near = 0.85 + 0.3 * f;
            let joint = joints.contains(&y);
            for x in 0..w {
                let grain = 0.9 + 0.2 * (hash(x, y, seed ^ 0x6C) % 16) as f32 / 16.0;
                let d = (x - xc).abs() as f32;
                let col = if d <= half - 1.0 {
                    scale(n.road, grain * near * if joint { 0.75 } else { 1.0 })
                } else if d <= half {
                    scale(n.platform[0], 0.8 * near)
                } else {
                    scale(n.ground, grain * near)
                };
                l.put(x, y, col);
            }
        }
        // The temple platforms: a lit top over a sloping side.
        for p in &self.platforms {
            for r in 0..p.h {
                let y = p.y - r;
                let (x0, x1) = (p.x + r / 2, p.x + p.w - 1 - r / 2);
                let col = if r == p.h - 1 {
                    n.platform[0]
                } else {
                    n.platform[1]
                };
                for x in x0..=x1 {
                    l.put(x, y, col);
                }
            }
        }
        // The pyramid: gold along each terrace's edge and down its sides,
        // the stairs lit between their balustrades.
        let rows = self.pyr.len() as i32;
        for (r, (&hw, &st)) in self.pyr.iter().zip(&self.stairs).enumerate() {
            let y = top + r as i32;
            let terrace = (r as i32 * 4) % rows < 4;
            // Each terrace is a slope under a lighter vertical band.
            let band = (r as f32 / rows as f32 * 4.0).fract() > 0.72;
            for x in xc - hw..=xc + hw {
                let d = (x - xc).abs();
                let col = if d < st {
                    scale(n.stair, if r % 2 == 0 { 1.0 } else { 0.82 })
                } else if d == st {
                    scale(n.body, 0.6)
                } else if d >= hw - 1 {
                    scale(n.rim, 0.75)
                } else if terrace {
                    scale(n.rim, 0.55)
                } else if band {
                    scale(n.body, 1.25)
                } else {
                    n.body
                };
                l.put(x, y, col);
            }
        }
        for m in &self.magueyes {
            l.maguey(m, scale(n.jade, 0.6 * lift));
        }
        // The crowd, more of it as the equinox comes, their arms rising
        // and falling in a wave.
        let arrived = 0.35 + 0.65 * level;
        for p in self.crowd.iter().filter(|p| p.r < arrived) {
            l.person(p, arms_up(l.t, p.phase), &n);
        }
    }

    /// One column of the volcanoes, weighted by `k` as the look fades in: the
    /// rock lit on the sides that face left, the snow above its line, grain
    /// in both and a little darker toward the foot.
    fn volcano_column(&mut self, x: usize, stop: usize, k: f32, lift: f32, amb: V3) {
        let Some(&(top, id)) = self.volc.get(x) else {
            return;
        };
        if id == 0 {
            return;
        }
        let (w, hz) = (self.w, self.horizon);
        let left = self.volc[x.saturating_sub(1)].0;
        let right = self.volc[(x + 1).min(w - 1)].0;
        // Rising to the right: the slope faces the left and is lit.
        let face = ((left - right) as f32 / 4.0).clamp(-1.0, 1.0) * 0.5 + 0.5;
        let v = self.volcan;
        let (rock, snow) = (
            mix(v.rock[1], v.rock[0], face),
            mix(v.snow[1], v.snow[0], face),
        );
        let snowline = self.snowline[x];
        let span = (hz as i32 - top).max(1) as f32;
        for y in (top.max(0) as usize)..stop {
            let grain = 0.92 + 0.16 * (hash(x as i32, y as i32, self.seed ^ 0x5A) % 8) as f32 / 8.0;
            let down = 1.0 - 0.3 * (y as i32 - top) as f32 / span;
            let base = if (y as i32) < snowline {
                scale(snow, grain * down.max(0.8))
            } else {
                scale(rock, lift * grain * down)
            };
            let i = y * w + x;
            self.px[i] = mix(self.px[i], add(base, amb), k);
            self.solid[i] |= k >= 0.5;
        }
    }

    /// The fireworks over the volcanoes: one slot at the start of the mes
    /// patrio, every slot by the night of the Grito.
    fn fireworks(&mut self, look: &Look) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let on = firework_slots(w, self.level(Kind::Patrias));
        let (secs, seed, light) = (self.t * STEP, self.seed, self.volcan.light);
        let mut l = Layer {
            px: &mut self.px,
            solid: &mut self.solid,
            w,
            h: ph,
            m: look.volcanes,
            lift: 1.0,
            water: None,
            t: self.t,
            hz,
        };
        for slot in 0..on {
            if let Some(b) = burst_of(slot, secs, w, hz, seed) {
                l.burst(&b, &light);
            }
        }
    }

    /// What stands in front of the volcanoes, drawn before the lake so that
    /// it mirrors them: Popocatépetl's ember and its smoke, the garlands of
    /// lights along the shore and the flag.
    fn light_volcanes(&mut self, look: &Look, lift: f32) {
        let (w, ph, hz) = (self.w as i32, self.ph as i32, self.horizon as i32);
        let (v, level) = (self.volcan, self.level(Kind::Patrias));
        let sc = (ph as f32 / 120.0).clamp(0.45, 1.3);
        let mut l = Layer {
            px: &mut self.px,
            solid: &mut self.solid,
            w,
            h: ph,
            m: look.volcanes,
            lift,
            water: None,
            t: self.t,
            hz,
        };
        if !self.volc.is_empty() {
            // The crater glows, and a little smoke leans away in the wind;
            // more of it as the month goes.
            let (cx, cy) = self.crater;
            let (fl, k) = flicker(&[v.ember, mix(v.ember, [1.0, 0.85, 0.5], 0.6)], l.t, 1.3);
            let glow = 0.3 + 0.3 * level;
            l.halo(
                cx as i32,
                cy as i32,
                (9.0 * sc) as i32,
                (6.0 * sc) as i32,
                fl,
                glow * k,
            );
            for dx in -1..=1 {
                l.lamp(
                    cx as i32 + dx,
                    cy as i32 + 1,
                    scale(fl, k * (1.0 - 0.2 * dx.abs() as f32)),
                );
            }
            l.lamp(cx as i32, cy as i32, scale(mix(fl, [1.0; 3], 0.4), k));
            let puffs = 5 + (9.0 * level) as usize;
            let rise = 0.42 * hz as f32;
            for i in 0..puffs {
                let u = (l.t * 0.0045 + i as f32 / puffs as f32).fract();
                let x = cx + 0.6 * u * rise + (u * 9.0 + i as f32).sin() * 1.5 * u * sc;
                let y = cy - u * rise;
                let a = 0.5 * (1.0 - u).powf(1.2) * (u / 0.06).min(1.0);
                let col = mix(v.smoke[0], v.smoke[1], (u * 4.0).min(1.0));
                l.puff(x, y, (1.5 + 5.0 * u) * sc, col, (1.6 * a).min(0.85));
            }
        }
        // The garlands: bulbs of the flag's three colors twinkling, more of
        // them lit as the month goes.
        let lit = 0.3 + 0.7 * level;
        for b in &self.bulbs {
            if b.c == 3 {
                l.put(b.x, b.y, scale(v.pole, 0.5));
            } else if b.r < lit {
                let k = 0.75 + 0.25 * (l.t * 0.2 + b.r * 40.0).sin();
                let col = v.light[b.c as usize];
                l.lamp(b.x, b.y, scale(col, k));
                for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    l.shine(b.x + dx, b.y + dy, col, 0.18 * k);
                }
            }
        }
        // The bandera waves from its pole: green, white with the eagle on
        // its nopal, and red.
        if let Some((fx, base, top, fw)) = self.flag {
            let fh = (fw * 4 / 7).max(3);
            l.put(fx, top - 1, v.pole);
            for y in top..=base {
                l.put(fx, y, scale(v.pole, 0.7));
            }
            for c in 0..fw {
                let amp = 0.12 * fh as f32 * c as f32 / fw as f32;
                let wave = ((c as f32 * 0.9 - l.t * 0.18).sin() * amp).round() as i32;
                let band = (3 * c / fw).min(2) as usize;
                for r in 0..fh {
                    let col = if band == 1 && (c - fw / 2).abs() <= 1 && (r - fh / 2).abs() <= 1 {
                        if r > fh / 2 { v.nopal } else { v.eagle }
                    } else {
                        v.flag[band]
                    };
                    l.lamp(fx + 1 + c, top + r + wave, col);
                }
            }
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
        // Hue at full strength; bright light spills over into white.
        let over = (m - 1.0).max(0.0) * 0.3;
        let hue = [l[0] / m + over, l[1] / m + over, l[2] / m + over];
        let alpha = m.min(1.0);
        let c = to_color(hue);
        if known {
            to_color(v3(ctx.fade(c, 1.0 - alpha)))
        } else {
            c.scale(0.4 + 0.6 * alpha)
        }
    }
}

impl Scene for Aurora {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        (self.cols, self.rows) = (w, h);
        self.w = w as usize;
        self.ph = h as usize * 2;
        self.out.resize(w, h);
        let n = self.w * self.ph;
        self.light = vec![0.0; self.w];
        self.edge = vec![0.0; self.w];
        self.glow = vec![[0.0; 3]; n];
        self.px = vec![[0.0; 3]; n];
        self.solid = vec![false; n];
        self.visitor = None;
        self.build();
    }

    fn retheme(&mut self, ctx: &Ctx) {
        if ctx.fiesta != self.fiesta {
            let (was, now) = (fiesta_look(self.fiesta.kind), fiesta_look(ctx.fiesta.kind));
            self.fiesta = ctx.fiesta;
            if self.fixed.is_none() && was != now {
                // The holiday comes or goes: fade into its look, or out of
                // it, starting now.
                match now {
                    Some(i) => self.next = i,
                    None if self.next >= ORDINARY => self.next = self.rng.usize(0..ORDINARY),
                    None => {}
                }
                self.clock = self.clock.max(self.stay - BLEND);
            }
        }
        self.cur = Self::look_of(self.which, ctx.theme);
        self.nxt = Self::look_of(self.next, ctx.theme);
        self.paint = Paint::new(ctx.theme);
        self.fest = Fest::new(ctx.theme);
        self.nac = Nacimiento::new(ctx.theme);
        self.volcan = Volcan::new(ctx.theme);
        self.piramide = Piramide::new(ctx.theme);
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.25)
        } else {
            0.0
        };
        // The ripples move smoothly; everything else ticks at tanim's rate.
        self.t += dt * self.speed / STEP * (1.0 + 1.5 * self.surge);
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
        if self.cur.muertos > 0.0 || self.nxt.muertos > 0.0 {
            self.row(dt);
        }
        if self.cur.belen > 0.0 || self.nxt.belen > 0.0 {
            self.stroll(dt);
        }
    }

    /// Motion follows the speed setting; the looks and the visitors keep real time.
    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        if !real.is_finite() {
            return;
        }
        let real = real.clamp(0.0, 5.0);
        self.clock += real;
        if self.clock >= self.stay {
            self.clock -= self.stay;
            self.which = self.next;
            self.next = Self::pick_next(
                &mut self.rng,
                &mut self.hrng,
                self.which,
                self.fixed,
                self.fiesta,
            );
            self.cur = Self::look_of(self.which, ctx.theme);
            self.nxt = Self::look_of(self.next, ctx.theme);
        }
        self.visit(real);
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let (w, ph) = (self.w, self.ph);
        if w == 0 || ph == 0 {
            return;
        }
        let bg = match ctx.bg {
            Color::Rgb(..) => Some(v3(ctx.bg)),
            _ => None,
        };
        let look = self.look();
        self.draw_curtains(&look);
        self.compose(&look, bg);
        if look.muertos > 0.01 {
            self.draw_patzcuaro(&look, bg);
        }
        if look.belen > 0.01 {
            self.draw_belen(&look);
        }
        self.out.clear();
        for y in 0..ph {
            for x in 0..w {
                let i = y * w + x;
                let col = if self.solid[i] {
                    to_color(self.px[i])
                } else {
                    Self::sky_light(ctx, self.px[i])
                };
                if col != Color::Reset {
                    self.out.set(x as i32, y as i32, col);
                }
            }
        }
        self.out.blit(c);
        if let Some(v) = &self.visitor {
            v.draw(c, ctx.theme);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;

    fn make(opts: &str, theme: &Theme) -> (Aurora, Opts) {
        let o = Opts::new(INFO.options, opts.parse().unwrap());
        let ctx = ctx_with(theme, &o, Color::Reset);
        let mut a = Aurora::new(&ctx);
        a.resize(&ctx, 80, 24);
        a.retheme(&ctx);
        (a, o)
    }

    fn ctx_with<'a>(theme: &'a Theme, opts: &'a Opts, bg: Color) -> Ctx<'a> {
        ctx_on(theme, opts, bg, Fiesta::NONE)
    }

    fn ctx_on<'a>(theme: &'a Theme, opts: &'a Opts, bg: Color, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg,
            seed: 3,
            fiesta,
        }
    }

    fn make_on(opts: &str, theme: &Theme, fiesta: Fiesta, w: u16, h: u16) -> (Aurora, Opts) {
        let o = Opts::new(INFO.options, opts.parse().unwrap());
        let ctx = ctx_on(theme, &o, Color::Reset, fiesta);
        let mut a = Aurora::new(&ctx);
        a.resize(&ctx, w, h);
        a.retheme(&ctx);
        (a, o)
    }

    /// Candle flames drawn in the picture: solid pixels whose color lies
    /// between a flame's edge and core, at some brightness.
    fn flames(a: &Aurora) -> usize {
        let [edge, core] = a.fest.flame;
        let is_flame = |p: &V3| {
            let k = p[0];
            if k < 0.6 {
                return false;
            }
            let u = (p[1] / k - edge[1]) / (core[1] - edge[1]);
            let b = edge[2] + u * (core[2] - edge[2]);
            (-0.02..=1.02).contains(&u) && (p[2] / k - b).abs() < 0.01
        };
        a.px.iter()
            .zip(&a.solid)
            .filter(|(p, s)| **s && is_flame(p))
            .count()
    }

    /// Pixels painted in a monarch's bright wing color.
    fn wings(a: &Aurora) -> usize {
        a.px.iter().filter(|p| **p == a.fest.wing[0]).count()
    }

    /// Odd options and sizes survive long runs and huge steps.
    #[test]
    fn variants_survive() {
        let variants = [
            "",
            "sky = true\ncurtains = 8\nspeed = 4.0\nlook = 'starry'",
            "landscape = false\nstars = false\ncurtains = 1\nlook = 'prism'",
            "speed = nan\ncurtains = -5\nstay = nan\nvisitors = inf",
            "speed = inf\ncurtains = 99\nsky = true\nlandscape = false\nlook = 'bogus'\nstay = 0.0",
            "look = 'synthwave'\nvisitors = 1000000.0",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                let (mut a, o) = make(v, &theme);
                for bg in [Color::Reset, theme.bg] {
                    let ctx = ctx_with(&theme, &o, bg);
                    for (w, h) in [(80, 24), (1, 1), (2, 3), (7, 2), (300, 90)] {
                        a.resize(&ctx, w, h);
                        a.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for k in 0..200 {
                            let dt = if k % 97 == 96 { 5.0 } else { 1.0 / 30.0 };
                            a.tick(&ctx, dt, dt);
                            c.clear();
                            a.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    /// Over a transparent sky the lights, the land and the lake show and much
    /// of the sky stays empty; `sky = true` paints every cell.
    #[test]
    fn transparent_and_painted_sky() {
        let theme = Theme::classic();
        let (mut a, o) = make("look = 'tanim'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&ctx, &mut c);
        let lit = c.cells().iter().filter(|cell| cell.ch != ' ').count();
        assert!(lit > 80 * 24 / 4 && lit < 80 * 24, "{lit} lit cells");

        let (mut a, o) = make("sky = true", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        c.clear();
        a.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|cell| cell.ch != ' '));
    }

    /// Without the landscape nothing solid is drawn: only light.
    #[test]
    fn sky_without_land() {
        let theme = Theme::classic();
        let (mut a, o) = make("landscape = false\nlook = 'tanim'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&ctx, &mut c);
        assert!(a.solid.iter().all(|&s| !s));
        assert_eq!(a.horizon, a.ph);
    }

    /// Curtains fade out and new ones take their place, so the sky never
    /// runs out of lights.
    #[test]
    fn curtains_come_and_go() {
        let theme = Theme::classic();
        let (mut a, o) = make("curtains = 3\nlook = 'tanim'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let first: Vec<f32> = a.curtains.iter().map(|c| c.phase).collect();
        for _ in 0..2400 {
            a.update(&ctx, 1.0 / 20.0);
        }
        assert_eq!(a.curtains.len(), 3);
        let now: Vec<f32> = a.curtains.iter().map(|c| c.phase).collect();
        assert!(first.iter().zip(&now).all(|(a, b)| a != b));
        let look = a.look();
        a.draw_curtains(&look);
        assert!(a.glow.iter().any(|g| g[1] > 0.05));
    }

    /// Cycling moves on to another look after `stay` real seconds, whatever
    /// the speed; a fixed look stays.
    #[test]
    fn looks_cycle_on_the_wall_clock() {
        let theme = Theme::classic();
        let (mut a, o) = make("stay = 30.0\nspeed = 0.1", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        let mut seen = vec![a.which];
        for _ in 0..(30 * 30 * 8) {
            a.tick(&ctx, 0.1 / 30.0, 1.0 / 30.0);
            if seen.last() != Some(&a.which) {
                seen.push(a.which);
            }
        }
        assert!(seen.len() >= 7, "{seen:?}");
        assert!(seen.windows(2).all(|p| p[0] != p[1]));

        let (mut a, o) = make("stay = 10.0\nlook = 'ice'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        for _ in 0..(30 * 60) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_eq!(LOOKS[a.which].0, "ice");
    }

    /// The starry night paints its whirling sky, the cypress and the village.
    #[test]
    fn the_starry_night() {
        let theme = Theme::classic();
        let (mut a, o) = make("look = 'starry'", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        assert!(!a.cypress.is_empty() && !a.village.is_empty());
        assert!(a.brush.iter().any(|b| b.core == 1) && a.brush.iter().any(|b| b.core == 2));
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&ctx, &mut c);
        // Every pixel of open sky shows paint.
        let n = a.w * a.horizon;
        let open = a.px[..n].iter().zip(&a.solid[..n]).filter(|(_, s)| !**s);
        assert!(open.map(|(p, _)| p).all(|p| p[2] > 0.1));
        let i = a.cypress[a.cypress.len() / 2].0;
        assert!(a.solid[i]);
    }

    /// Visitors come when asked to, and never with `visitors = 0`.
    #[test]
    fn visitors_go_by() {
        let theme = Theme::classic();
        let (mut a, o) = make("visitors = 1000000.0", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        assert!(a.visitor.is_some());
        for _ in 0..(45 * 30) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            assert!(a.visiting <= VISIT_SECS + 0.1);
        }

        let (mut a, o) = make("visitors = 0.0", &theme);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        for _ in 0..(600 * 30) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            assert!(a.visitor.is_none());
        }
    }

    /// Día de Muertos opens on Pátzcuaro, from its first day: Janitzio's
    /// candles, the canoes and the veladoras on the lake, the monarchs, and
    /// more of everything at the holiday's height.
    #[test]
    fn patzcuaro_comes_with_the_holiday() {
        let mut lit = Vec::new();
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (m, d) in [(10, 28), (11, 2)] {
                let fiesta = Fiesta::of_day(m, d);
                let (mut a, o) = make_on("", &theme, fiesta, 120, 36);
                let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
                assert_eq!(a.which, PATZCUARO);
                assert!(!a.isle.is_empty() && !a.canoes.is_empty() && !a.floats.is_empty());
                let mut c = Canvas::new(120, 36, Color::Reset);
                a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                a.draw(&ctx, &mut c);
                assert!(flames(&a) > 10, "{m}-{d}: {} flames", flames(&a));
                assert!(wings(&a) > 0, "{m}-{d}: no monarchs");
                lit.push(flames(&a));
            }
        }
        assert!(lit[0] < lit[1] && lit[2] < lit[3], "{lit:?}");
    }

    /// The holiday shows Pátzcuaro most of the time; ordinary days never.
    #[test]
    fn patzcuaro_only_on_the_holiday() {
        let theme = Theme::classic();
        for (fiesta, lo, hi) in [(Fiesta::of_day(11, 2), 0.75, 1.0), (Fiesta::NONE, 0.0, 0.0)] {
            let (mut a, o) = make_on("stay = 10.0", &theme, fiesta, 80, 24);
            let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
            let mut c = Canvas::new(80, 24, Color::Reset);
            let (mut on, n) = (0, 30 * 600);
            for k in 0..n {
                a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                on += usize::from(a.which == PATZCUARO);
                if k % 300 == 0 && !fiesta.is(Kind::Muertos) {
                    a.draw(&ctx, &mut c);
                    assert_eq!(flames(&a), 0);
                }
            }
            let share = on as f32 / n as f32;
            assert!((lo..=hi).contains(&share), "{share}");
        }
    }

    /// When the holiday comes (at midnight, through `retheme`) the sky fades
    /// into Pátzcuaro, and out of it when the holiday goes.
    #[test]
    fn the_holiday_comes_and_goes() {
        let theme = Theme::classic();
        let (mut a, o) = make_on("", &theme, Fiesta::NONE, 80, 24);
        let off = ctx_on(&theme, &o, Color::Reset, Fiesta::NONE);
        let on = ctx_on(&theme, &o, Color::Reset, Fiesta::of_day(10, 31));
        assert_ne!(a.which, PATZCUARO);
        a.retheme(&on);
        for _ in 0..(30 * 15) {
            a.tick(&on, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_eq!(a.which, PATZCUARO);
        assert!(a.look().muertos > 0.99);
        a.retheme(&off);
        for _ in 0..(30 * 15) {
            a.tick(&off, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_ne!(a.which, PATZCUARO);
        assert_eq!(a.look().muertos, 0.0);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&off, &mut c);
        assert_eq!(flames(&a), 0);
    }

    /// A fixed look wins over the holiday, and Pátzcuaro can be fixed on any day.
    #[test]
    fn a_fixed_look_wins() {
        let theme = Theme::classic();
        let fiesta = Fiesta::of_day(11, 2);
        let (mut a, o) = make_on("look = 'ice'\nstay = 10.0", &theme, fiesta, 80, 24);
        let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
        for _ in 0..(30 * 60) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            assert_eq!(LOOKS[a.which].0, "ice");
        }
        let (mut a, o) = make_on("look = 'pátzcuaro'", &theme, Fiesta::NONE, 80, 24);
        let ctx = ctx_with(&theme, &o, Color::Reset);
        assert_eq!(a.which, PATZCUARO);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&ctx, &mut c);
        assert!(flames(&a) > 0);
    }

    /// The holiday survives odd options and sizes, long runs and huge steps.
    #[test]
    fn the_holiday_survives() {
        let variants = [
            "",
            "sky = true\nlook = 'patzcuaro'\nspeed = 4.0",
            "landscape = false\nstars = false\nlook = 'patzcuaro'",
            "speed = nan\ncurtains = -5\nstay = nan\nvisitors = inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                for fiesta in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
                    let (mut a, o) = make_on(v, &theme, fiesta, 80, 24);
                    let ctx = ctx_on(&theme, &o, theme.bg, fiesta);
                    for (w, h) in [(1, 1), (2, 3), (16, 4), (17, 9), (300, 90)] {
                        a.resize(&ctx, w, h);
                        a.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for k in 0..120 {
                            let dt = if k % 50 == 49 { 5.0 } else { 1.0 / 30.0 };
                            a.tick(&ctx, dt, dt);
                            c.clear();
                            a.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    /// Solid pixels painted in the hue of `col`, at some brightness: lit
    /// windows, the Niño Dios' light, a king's robe.
    fn hued(a: &Aurora, col: V3) -> usize {
        let m = col[0].max(col[1]).max(col[2]);
        a.px.iter()
            .zip(&a.solid)
            .filter(|(p, solid)| {
                let k = p[0].max(p[1]).max(p[2]);
                **solid && k > 0.35 && (0..3).all(|i| (p[i] / k - col[i] / m).abs() < 0.05)
            })
            .count()
    }

    /// One frame of the holiday, a moment after it opens.
    fn frame(opts: &str, theme: &Theme, fiesta: Fiesta, w: u16, h: u16) -> Aurora {
        let (mut a, o) = make_on(opts, theme, fiesta, w, h);
        let ctx = ctx_on(theme, &o, Color::Reset, fiesta);
        let mut c = Canvas::new(w, h, Color::Reset);
        a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        a.draw(&ctx, &mut c);
        a
    }

    /// The Navidad opens on Belén from its first day: the town with its
    /// windows lit, the star, the dunes, and more windows as the posadas go.
    #[test]
    fn belen_comes_with_the_holiday() {
        let mut lit = Vec::new();
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (m, d) in [(12, 12), (12, 24)] {
                let a = frame("", &theme, Fiesta::of_day(m, d), 120, 36);
                assert_eq!(a.which, BELEN);
                assert!(!a.town.is_empty() && !a.crests.is_empty() && !a.portal.is_empty());
                let windows = hued(&a, a.nac.lamp[0]);
                assert!(windows > 0, "{m}-{d}: {windows} windows");
                // The star's heart is lit.
                let (sx, sy) = (a.star.0 as usize, a.star.1 as usize);
                assert!(a.px[sy * a.w + sx].iter().all(|c| *c > 0.5));
                lit.push(windows);
            }
        }
        assert!(lit[0] < lit[1] && lit[2] < lit[3], "{lit:?}");
    }

    /// The holiday shows Belén most of the time; ordinary days never.
    #[test]
    fn belen_only_on_the_holiday() {
        let theme = Theme::classic();
        for (fiesta, lo, hi) in [
            (Fiesta::of_day(12, 24), 0.75, 1.0),
            (Fiesta::NONE, 0.0, 0.0),
        ] {
            let (mut a, o) = make_on("stay = 10.0", &theme, fiesta, 80, 24);
            let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
            let mut c = Canvas::new(80, 24, Color::Reset);
            let (mut on, n) = (0, 30 * 600);
            for k in 0..n {
                a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                on += usize::from(a.which == BELEN);
                if k % 300 == 0 && !fiesta.is(Kind::Navidad) {
                    a.draw(&ctx, &mut c);
                    assert_eq!(hued(&a, a.nac.lamp[0]), 0);
                }
            }
            let share = on as f32 / n as f32;
            assert!((lo..=hi).contains(&share), "{share}");
        }
    }

    /// Through `retheme` the sky fades into Belén when the Navidad comes
    /// (at midnight) and out of it when it goes.
    #[test]
    fn the_navidad_comes_and_goes() {
        let theme = Theme::classic();
        let (mut a, o) = make_on("", &theme, Fiesta::NONE, 80, 24);
        let off = ctx_on(&theme, &o, Color::Reset, Fiesta::NONE);
        let on = ctx_on(&theme, &o, Color::Reset, Fiesta::of_day(12, 16));
        assert_ne!(a.which, BELEN);
        a.retheme(&on);
        for _ in 0..(30 * 15) {
            a.tick(&on, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_eq!(a.which, BELEN);
        assert!(a.look().belen > 0.99);
        // The next day is the same holiday: nothing changes.
        let next = ctx_on(&theme, &o, Color::Reset, Fiesta::of_day(12, 17));
        a.retheme(&next);
        assert_eq!(a.which, BELEN);
        a.retheme(&off);
        for _ in 0..(30 * 15) {
            a.tick(&off, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert!(a.which < ORDINARY);
        assert_eq!(a.look().belen, 0.0);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&off, &mut c);
        assert_eq!(hued(&a, a.nac.lamp[0]), 0);
    }

    /// The Niño Dios lies in the manger from Nochebuena on, not before.
    #[test]
    fn the_nino_comes_on_nochebuena() {
        let theme = Theme::classic();
        let before = frame("", &theme, Fiesta::of_day(12, 23), 160, 48);
        let after = frame("", &theme, Fiesta::of_day(12, 24), 160, 48);
        assert_eq!(hued(&before, before.nac.lamp[1]), 0);
        assert!(hued(&after, after.nac.lamp[1]) > 0);
    }

    /// The Reyes Magos are not there in December, show far away and small on
    /// January 2 and come closer every day, to the portal on the 6th.
    #[test]
    fn the_reyes_come_closer() {
        let theme = Theme::classic();
        let robes = |m, d| {
            let a = frame("", &theme, Fiesta::of_day(m, d), 160, 48);
            hued(&a, a.nac.robe[0])
        };
        assert_eq!(robes(12, 25), 0);
        assert_eq!(robes(1, 1), 0);
        let seen: Vec<usize> = (2..=6).map(|d| robes(1, d)).collect();
        assert!(seen[0] > 0, "{seen:?}");
        assert!(seen.windows(2).all(|p| p[0] < p[1]), "{seen:?}");
    }

    /// The Reyes walk across the dunes, and go around again from the left.
    #[test]
    fn the_reyes_cross_the_dunes() {
        let theme = Theme::classic();
        let fiesta = Fiesta::of_day(1, 4);
        let (mut a, o) = make_on("", &theme, fiesta, 100, 30);
        let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
        let (mut seen, mut last) = (0, a.reyes_x);
        for _ in 0..(30 * 600) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            if a.reyes_x < last {
                seen += 1;
            }
            last = a.reyes_x;
        }
        assert!(seen >= 1, "they never went around");
        // On the 6th they stand at the portal.
        let fiesta = Fiesta::of_day(1, 6);
        let (mut a, o) = make_on("", &theme, fiesta, 100, 30);
        let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
        let x = a.reyes_x;
        for _ in 0..300 {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_eq!(a.reyes_x, x);
    }

    /// A fixed look wins over the Navidad, and Belén can be asked for on any
    /// day: with the Niño Dios in the manger and the Reyes on their way.
    #[test]
    fn belen_on_any_day() {
        let theme = Theme::classic();
        let fiesta = Fiesta::of_day(12, 24);
        let (mut a, o) = make_on("look = 'ice'\nstay = 10.0", &theme, fiesta, 80, 24);
        let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
        for _ in 0..(30 * 60) {
            a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            assert_eq!(LOOKS[a.which].0, "ice");
        }
        for name in ["belen", "belén", "navidad"] {
            let a = frame(&format!("look = '{name}'"), &theme, Fiesta::NONE, 160, 48);
            assert_eq!(a.which, BELEN);
            assert!(hued(&a, a.nac.lamp[0]) > 5);
            assert!(hued(&a, a.nac.lamp[1]) > 0);
            assert!(hued(&a, a.nac.robe[0]) > 0);
        }
    }

    /// Holidays aurora has no look for leave it exactly as on an ordinary day.
    #[test]
    fn other_holidays_are_ordinary_days() {
        let theme = Theme::classic();
        let run = |fiesta: Fiesta| {
            let (mut a, o) = make_on("stay = 10.0", &theme, fiesta, 80, 24);
            let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
            let mut c = Canvas::new(80, 24, Color::Reset);
            let mut frames = Vec::new();
            for k in 0..(30 * 120) {
                a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                if k % 300 == 0 {
                    c.clear();
                    a.draw(&ctx, &mut c);
                    frames.push(c.cells().to_vec());
                }
            }
            frames
        };
        let plain = run(Fiesta::NONE);
        for kind in [
            Kind::Candelaria,
            Kind::Nino,
            Kind::SantaCruz,
            Kind::Revolucion,
        ] {
            assert!(plain == run(Fiesta::height(kind)), "{kind:?}");
        }
    }

    /// The Navidad survives odd options and sizes, long runs and huge steps,
    /// from Nochebuena to the Reyes.
    #[test]
    fn the_navidad_survives() {
        let variants = [
            "",
            "sky = true\nlook = 'belen'\nspeed = 4.0",
            "landscape = false\nstars = false\nlook = 'belen'",
            "speed = nan\ncurtains = -5\nstay = nan\nvisitors = inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                for fiesta in [
                    Fiesta::of_day(12, 12),
                    Fiesta::of_day(12, 24),
                    Fiesta::of_day(1, 2),
                    Fiesta::of_day(1, 6),
                ] {
                    let (mut a, o) = make_on(v, &theme, fiesta, 80, 24);
                    let ctx = ctx_on(&theme, &o, theme.bg, fiesta);
                    for (w, h) in [(1, 1), (2, 3), (16, 4), (17, 9), (40, 12), (300, 90)] {
                        a.resize(&ctx, w, h);
                        a.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for k in 0..120 {
                            let dt = if k % 50 == 49 { 5.0 } else { 1.0 / 30.0 };
                            a.tick(&ctx, dt, dt);
                            c.clear();
                            a.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    /// The Fiestas Patrias open on the volcanoes from the first day: both of
    /// them rise from the valley, the flag waves, and more lights are lit as
    /// the month goes.
    #[test]
    fn volcanes_come_with_the_holiday() {
        let mut lit = Vec::new();
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (m, d) in [(9, 1), (9, 15)] {
                let a = frame("", &theme, Fiesta::of_day(m, d), 120, 36);
                assert_eq!(a.which, VOLCANES);
                for id in [1, 2] {
                    assert!(a.volc.iter().any(|v| v.1 == id), "{m}-{d}: no volcano {id}");
                }
                assert!(!a.bulbs.is_empty() && a.flag.is_some());
                let (cx, cy) = (a.crater.0 as usize, a.crater.1 as usize);
                let ember = a.px[(cy + 1) * a.w + cx];
                assert!(ember[0] > 0.5 && ember[0] > ember[2], "{ember:?}");
                lit.push(hued(&a, a.volcan.light[2]));
            }
        }
        assert!(lit[0] > 0 && lit[0] < lit[1], "{lit:?}");
    }

    /// The flag waves from its pole in green, white and red.
    #[test]
    fn the_flag_waves() {
        let theme = Theme::classic();
        let a = frame("", &theme, Fiesta::of_day(9, 16), 160, 48);
        let v = a.volcan;
        for band in [0, 2] {
            assert!(hued(&a, v.flag[band]) > 5, "band {band}");
        }
        assert!(hued(&a, v.eagle) > 0 && hued(&a, v.nopal) > 0);
    }

    /// The holiday shows the volcanoes most of the time; ordinary days never.
    #[test]
    fn volcanes_only_on_the_holiday() {
        let theme = Theme::classic();
        for (fiesta, lo, hi) in [(Fiesta::of_day(9, 15), 0.75, 1.0), (Fiesta::NONE, 0.0, 0.0)] {
            let (mut a, o) = make_on("stay = 10.0", &theme, fiesta, 80, 24);
            let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
            let mut c = Canvas::new(80, 24, Color::Reset);
            let (mut on, n) = (0, 30 * 600);
            for k in 0..n {
                a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                on += usize::from(a.which == VOLCANES);
                if k % 300 == 0 && !fiesta.is(Kind::Patrias) {
                    a.draw(&ctx, &mut c);
                    assert_eq!(hued(&a, a.volcan.flag[0]), 0);
                }
            }
            let share = on as f32 / n as f32;
            assert!((lo..=hi).contains(&share), "{share}");
        }
    }

    /// Through `retheme` the sky fades into the volcanoes when September
    /// comes and out of them when the mes patrio goes.
    #[test]
    fn the_mes_patrio_comes_and_goes() {
        let theme = Theme::classic();
        let (mut a, o) = make_on("", &theme, Fiesta::NONE, 80, 24);
        let off = ctx_on(&theme, &o, Color::Reset, Fiesta::NONE);
        let on = ctx_on(&theme, &o, Color::Reset, Fiesta::of_day(9, 3));
        assert_ne!(a.which, VOLCANES);
        a.retheme(&on);
        for _ in 0..(30 * 15) {
            a.tick(&on, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_eq!(a.which, VOLCANES);
        assert!(a.look().volcanes > 0.99);
        a.retheme(&off);
        for _ in 0..(30 * 15) {
            a.tick(&off, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert!(a.which < ORDINARY);
        assert_eq!(a.look().volcanes, 0.0);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&off, &mut c);
        assert_eq!(hued(&a, a.volcan.flag[0]), 0);
    }

    /// Fireworks go off now and then and are all over the sky by the night of
    /// the Grito, a slot or two at the start of the month.
    #[test]
    fn fireworks_over_the_volcanoes() {
        let seen = |slots: usize| {
            (0..600)
                .filter(|k| (0..slots).any(|s| burst_of(s, *k as f32 * 0.5, 160, 44, 3).is_some()))
                .count()
        };
        assert!(seen(1) > 100 && seen(1) < 300, "{}", seen(1));
        assert!(seen(4) > seen(1));
        for s in 0..4 {
            for k in 0..400 {
                if let Some(b) = burst_of(s, k as f32 * 0.37, 160, 44, 3) {
                    assert!((0.0..BURST_LIFE).contains(&b.age));
                    assert!((0.0..160.0).contains(&b.x) && (0.0..44.0).contains(&b.y));
                    assert!(b.cols.0 != b.cols.1 && b.cols.0 < 3 && b.cols.1 < 3);
                }
            }
        }
        // More slots are let off as the month goes.
        assert!(firework_slots(160, 0.2) < firework_slots(160, 1.0));
        assert_eq!(firework_slots(160, 1.0), 4);
    }

    /// Iztaccíhuatl lies on her back: her head at one end, her chest the
    /// highest, a waist, knees, and her feet at the other end.
    #[test]
    fn the_sleeping_woman() {
        assert_eq!(izta(0.0), 0.0);
        assert_eq!(izta(1.0), 0.0);
        let best = (0..100)
            .max_by(|a, b| izta(*a as f32 / 100.0).total_cmp(&izta(*b as f32 / 100.0)))
            .unwrap();
        assert!((30..40).contains(&best), "highest at {best}");
        assert!(
            izta(0.10) < 0.7 * izta(0.34),
            "the head is lower than the chest"
        );
        assert!(
            izta(0.5) < izta(0.34) && izta(0.5) < izta(0.65),
            "a waist between chest and knees"
        );
        assert!(izta(0.93) < izta(0.65), "the feet are lower than the knees");
        assert!((0..=100).all(|k| (0.0..=1.0).contains(&izta(k as f32 / 100.0))));
    }

    /// Popocatépetl smokes more as the month goes: the plume darkens more of
    /// the sky over the crater.
    #[test]
    fn popocatepetl_smokes() {
        let theme = Theme::classic();
        let fumes = |f: Fiesta| {
            let a = frame("", &theme, f, 160, 48);
            let (cx, cy) = (a.crater.0 as i32, a.crater.1 as i32);
            let mut n = 0;
            for y in (cy - 20).max(0)..cy {
                for x in cx - 6..cx + 20 {
                    let p = a.px[y as usize * a.w + x as usize];
                    let (hi, lo) = (p[0].max(p[1]).max(p[2]), p[0].min(p[1]).min(p[2]));
                    n += usize::from(hi > 0.1 && hi - lo < 0.15 * hi);
                }
            }
            n
        };
        assert!(fumes(Fiesta::of_day(9, 15)) > 0);
    }

    /// The volcanoes can be asked for on any day.
    #[test]
    fn volcanes_on_any_day() {
        let theme = Theme::classic();
        for name in ["volcanes", "volcán", "patrias"] {
            let a = frame(&format!("look = '{name}'"), &theme, Fiesta::NONE, 160, 48);
            assert_eq!(a.which, VOLCANES);
            assert!(!a.volc.is_empty() && a.flag.is_some());
            assert!(hued(&a, a.volcan.flag[0]) > 0);
        }
    }

    /// The Fiestas Patrias survive odd options and sizes, long runs and huge
    /// steps, from the first of the month to the parade.
    #[test]
    fn the_mes_patrio_survives() {
        let variants = [
            "",
            "sky = true\nlook = 'volcanes'\nspeed = 4.0",
            "landscape = false\nstars = false\nlook = 'volcanes'",
            "speed = nan\ncurtains = -5\nstay = nan\nvisitors = inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                for fiesta in [
                    Fiesta::of_day(9, 1),
                    Fiesta::of_day(9, 15),
                    Fiesta::of_day(9, 16),
                ] {
                    let (mut a, o) = make_on(v, &theme, fiesta, 80, 24);
                    let ctx = ctx_on(&theme, &o, theme.bg, fiesta);
                    for (w, h) in [
                        (1, 1),
                        (2, 3),
                        (16, 4),
                        (17, 9),
                        (23, 12),
                        (24, 20),
                        (300, 90),
                    ] {
                        a.resize(&ctx, w, h);
                        a.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for k in 0..120 {
                            let dt = if k % 50 == 49 { 5.0 } else { 1.0 / 30.0 };
                            a.tick(&ctx, dt, dt);
                            c.clear();
                            a.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }

    /// The Equinoccio opens on Teotihuacán from its first day: the pyramid
    /// with its stairs and the avenue between its platforms, the sun rising
    /// behind it and the crowd in white, more of it on the 20th and 21st.
    #[test]
    fn teotihuacan_comes_with_the_holiday() {
        let mut crowd = Vec::new();
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (m, d) in [(3, 19), (3, 20)] {
                let a = frame("", &theme, Fiesta::of_day(m, d), 120, 36);
                assert_eq!(a.which, TEOTIHUACAN);
                assert!(!a.pyr.is_empty() && !a.crowd.is_empty() && !a.platforms.is_empty());
                // The sun's heart is lit, and the people arrive.
                let (xc, y, _) = a.sun;
                let bw = *a.pyr.last().unwrap() as f32;
                let x = (xc + a.sun_shift() * bw) as usize;
                assert!(a.px[y as usize * a.w + x][0] > 0.8, "{m}-{d}: no sun");
                crowd.push(hued(&a, a.piramide.robe));
            }
        }
        assert!(crowd[0] > 0 && crowd[0] < crowd[1], "{crowd:?}");
    }

    /// The sun rises behind the summit on the 20th and 21st, a little to one
    /// side on the 19th and 22nd.
    #[test]
    fn the_sun_rises_beside_the_summit() {
        let theme = Theme::classic();
        let shift = |m, d| frame("", &theme, Fiesta::of_day(m, d), 120, 36).sun_shift();
        assert!(shift(3, 19) < -0.2 && shift(3, 22) > 0.2);
        assert!(shift(3, 20).abs() < 0.1 && shift(3, 21).abs() < 0.1);
        assert_eq!(
            frame("look = 'teotihuacan'", &theme, Fiesta::NONE, 120, 36).sun_shift(),
            0.0
        );
    }

    /// The holiday shows Teotihuacán most of the time; ordinary days never.
    #[test]
    fn teotihuacan_only_on_the_holiday() {
        let theme = Theme::classic();
        for (fiesta, lo, hi) in [(Fiesta::of_day(3, 20), 0.75, 1.0), (Fiesta::NONE, 0.0, 0.0)] {
            let (mut a, o) = make_on("stay = 10.0", &theme, fiesta, 80, 24);
            let ctx = ctx_on(&theme, &o, Color::Reset, fiesta);
            let mut c = Canvas::new(80, 24, Color::Reset);
            let (mut on, n) = (0, 30 * 600);
            for k in 0..n {
                a.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                on += usize::from(a.which == TEOTIHUACAN);
                if k % 300 == 0 && !fiesta.is(Kind::Equinoccio) {
                    a.draw(&ctx, &mut c);
                    assert_eq!(a.look().teotihuacan, 0.0);
                }
            }
            let share = on as f32 / n as f32;
            assert!((lo..=hi).contains(&share), "{share}");
        }
    }

    /// Through `retheme` the sky fades into the dawn at Teotihuacán when the
    /// equinox comes and out of it when it goes.
    #[test]
    fn the_equinoccio_comes_and_goes() {
        let theme = Theme::classic();
        let (mut a, o) = make_on("", &theme, Fiesta::NONE, 80, 24);
        let off = ctx_on(&theme, &o, Color::Reset, Fiesta::NONE);
        let on = ctx_on(&theme, &o, Color::Reset, Fiesta::of_day(3, 19));
        assert_ne!(a.which, TEOTIHUACAN);
        a.retheme(&on);
        for _ in 0..(30 * 15) {
            a.tick(&on, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert_eq!(a.which, TEOTIHUACAN);
        assert!(a.look().teotihuacan > 0.99);
        a.retheme(&off);
        for _ in 0..(30 * 15) {
            a.tick(&off, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert!(a.which < ORDINARY);
        assert_eq!(a.look().teotihuacan, 0.0);
        let mut c = Canvas::new(80, 24, Color::Reset);
        a.draw(&off, &mut c);
        assert_eq!(a.look().teotihuacan, 0.0);
    }

    /// The crowd's arms are up most of the time and come down now and then,
    /// the wave rolling from one person to the next.
    #[test]
    fn the_crowd_raises_its_arms() {
        let up = (0..2000).filter(|k| arms_up(*k as f32 * 0.5, 1.0)).count();
        assert!(up > 1000 && up < 1800, "{up}");
        let apart = (0..400)
            .filter(|k| arms_up(*k as f32, 0.0) != arms_up(*k as f32, 2.5))
            .count();
        assert!(apart > 20, "{apart}");
    }

    /// Teotihuacán can be asked for on any day.
    #[test]
    fn teotihuacan_on_any_day() {
        let theme = Theme::classic();
        for name in ["teotihuacan", "teotihuacán", "equinoccio"] {
            let a = frame(&format!("look = '{name}'"), &theme, Fiesta::NONE, 160, 48);
            assert_eq!(a.which, TEOTIHUACAN);
            assert!(!a.pyr.is_empty() && hued(&a, a.piramide.robe) > 0);
        }
    }

    /// The Equinoccio survives odd options and sizes, long runs and huge
    /// steps, from the 19th to the 22nd.
    #[test]
    fn the_equinoccio_survives() {
        let variants = [
            "",
            "sky = true\nlook = 'teotihuacan'\nspeed = 4.0",
            "landscape = false\nstars = false\nlook = 'teotihuacan'",
            "speed = nan\ncurtains = -5\nstay = nan\nvisitors = inf",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                for fiesta in [
                    Fiesta::of_day(3, 19),
                    Fiesta::of_day(3, 20),
                    Fiesta::of_day(3, 22),
                ] {
                    let (mut a, o) = make_on(v, &theme, fiesta, 80, 24);
                    let ctx = ctx_on(&theme, &o, theme.bg, fiesta);
                    for (w, h) in [
                        (1, 1),
                        (2, 3),
                        (16, 4),
                        (17, 9),
                        (24, 12),
                        (25, 20),
                        (300, 90),
                    ] {
                        a.resize(&ctx, w, h);
                        a.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for k in 0..120 {
                            let dt = if k % 50 == 49 { 5.0 } else { 1.0 / 30.0 };
                            a.tick(&ctx, dt, dt);
                            c.clear();
                            a.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }
}
