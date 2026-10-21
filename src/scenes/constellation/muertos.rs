//! Día de Muertos in the night sky: now and then a handful of stars leave
//! their drift, glide together and draw a figure with their links (a
//! calavera de azúcar, La Catrina in her feathered hat, a cempasúchil, a
//! monarch butterfly, a veladora), a spark traces it, the icing and flowers
//! light up, and after a while the stars let go and drift on.
//!
//! Figures are drawn in design units (square, like the scene's own units)
//! and scaled to the screen. Every node is a real star; the icing (`decor`,
//! dotted lines) and the flowers (`marks`, glyphs) belong to the figure.

use std::f32::consts::{PI, TAU};

use crate::color::{Color, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Braille, Canvas};
use crate::scene::Ctx;

use super::{DISSOLVE, LINK_FLOOR, Star};

/// Most stars a figure takes.
pub(super) const MAX_NODES: usize = 32;
/// Most figures in the sky at once.
const MAX_DRAWINGS: usize = 3;
/// A star's `fig` while its figure lets it go: it drifts on its own again,
/// but no new figure may take it yet.
pub(super) const LETTING_GO: u8 = u8::MAX;

/// Real seconds the stars take to gather, the spark to trace the figure, the
/// figure to stay, and the stars to let go.
const GATHER: f32 = 4.2;
const TRACE: f32 = 2.6;
const HOLD: f32 = 9.0;
const LET_GO: f32 = 3.6;
const LIFE: f32 = GATHER + TRACE + HOLD + LET_GO;
/// How lit an edge is while its stars are still gathering.
const SKETCH: f32 = 0.22;

// The holiday's colors (indices into `Muertos::pal`).
const HUESO: u8 = 0;
const CEMPA: u8 = 1;
const CEMPA_L: u8 = 2;
const CEMPA_D: u8 = 3;
const ROSA: u8 = 4;
const TERCIO: u8 = 5;
const MORADO: u8 = 6;
const TURQ: u8 = 7;
const AMARILLO: u8 = 8;
const LIMON: u8 = 9;
const FLAMA: u8 = 10;
const HUES: usize = 11;

/// The holiday's colors under `classic`, a little lighter than the cut paper
/// where they have to read as thin dotted lines on a night sky.
const NATIVE: [u32; HUES] = [
    0xf4eee2, // hueso
    0xff8c00, // cempasúchil
    0xffaa1e, // cempasúchil, light
    0xd66200, // cempasúchil, dark
    0xf0168c, // rosa mexicano
    0xc8104f, // terciopelo
    0x9a50d0, // morado
    0x14bcc8, // turquesa
    0xffd200, // amarillo
    0x8cc828, // verde limón
    0xffd678, // flama
];

/// Star tints during the holiday (same weights as the ordinary ones):
/// turquoise, pale gold, bone, marigold, pink, deep marigold, violet.
const STAR_HUES: [u32; 7] = [
    0x7fd6dc, 0xffe3a0, 0xf4eee2, 0xffb347, 0xff6fb5, 0xff8c00, 0xc08ae6,
];
/// What the ordinary links lean toward during the holiday.
const LINK_HUE: u32 = 0x8a4fc8;

#[derive(Clone, Copy, PartialEq)]
enum Anim {
    Still,
    /// The wings open and close.
    Flap,
    /// Everything above `y` (the flame) flickers.
    Flicker(f32),
}

struct Figure {
    nodes: &'static [(f32, f32)],
    /// (from, to, hue), in the order the spark traces them.
    edges: &'static [(u8, u8, u8)],
    /// Icing: (x0, y0, x1, y1, hue, stride), dotted lines.
    decor: &'static [(f32, f32, f32, f32, u8, u8)],
    /// Flowers and details: (x, y, glyph, hue).
    marks: &'static [(f32, f32, char, u8)],
    anim: Anim,
    weight: u32,
}

/// A sugar skull: rounded cranium, cheekbones, flower eyes, icing scallops
/// on the forehead, a heart for a nose and a row of teeth.
const CALAVERA: Figure = Figure {
    nodes: &[
        // Outline, clockwise from the crown.
        (10.0, 0.0),
        (15.4, 1.2),
        (19.0, 4.8),
        (20.2, 9.6),
        (19.0, 14.2),
        (16.2, 16.8),
        (15.6, 21.2),
        (12.6, 23.4),
        (7.4, 23.4),
        (4.4, 21.2),
        (3.8, 16.8),
        (1.0, 14.2),
        (-0.2, 9.6),
        (1.0, 4.8),
        (4.6, 1.2),
        // Eyes.
        (6.0, 7.7),
        (8.6, 9.2),
        (8.6, 12.0),
        (6.0, 13.5),
        (3.4, 12.0),
        (3.4, 9.2),
        (14.0, 7.7),
        (16.6, 9.2),
        (16.6, 12.0),
        (14.0, 13.5),
        (11.4, 12.0),
        (11.4, 9.2),
    ],
    edges: &[
        (0, 1, HUESO),
        (1, 2, HUESO),
        (2, 3, HUESO),
        (3, 4, HUESO),
        (4, 5, HUESO),
        (5, 6, HUESO),
        (6, 7, HUESO),
        (7, 8, HUESO),
        (8, 9, HUESO),
        (9, 10, HUESO),
        (10, 11, HUESO),
        (11, 12, HUESO),
        (12, 13, HUESO),
        (13, 14, HUESO),
        (14, 0, HUESO),
        (15, 16, ROSA),
        (16, 17, ROSA),
        (17, 18, ROSA),
        (18, 19, ROSA),
        (19, 20, ROSA),
        (20, 15, ROSA),
        (21, 22, ROSA),
        (22, 23, ROSA),
        (23, 24, ROSA),
        (24, 25, ROSA),
        (25, 26, ROSA),
        (26, 21, ROSA),
    ],
    decor: &[
        // Teeth.
        (6.2, 19.8, 13.8, 19.8, HUESO, 1),
        (7.7, 18.8, 7.7, 20.8, HUESO, 1),
        (9.2, 18.6, 9.2, 21.0, HUESO, 1),
        (10.8, 18.6, 10.8, 21.0, HUESO, 1),
        (12.3, 18.8, 12.3, 20.8, HUESO, 1),
        // A garland of icing over the forehead flower.
        (6.4, 4.0, 8.2, 2.6, TURQ, 2),
        (8.2, 2.6, 11.8, 2.6, TURQ, 2),
        (11.8, 2.6, 13.6, 4.0, TURQ, 2),
        // Petals on the cheeks.
        (2.4, 16.2, 4.0, 18.4, CEMPA, 2),
        (17.6, 16.2, 16.0, 18.4, CEMPA, 2),
    ],
    marks: &[
        (6.0, 10.6, '✿', AMARILLO),
        (14.0, 10.6, '✿', AMARILLO),
        (10.0, 15.6, '♥', ROSA),
        (10.0, 4.6, '❀', CEMPA),
        (7.4, 5.2, '•', AMARILLO),
        (12.6, 5.2, '•', AMARILLO),
        (3.0, 18.6, '•', TERCIO),
        (17.0, 18.6, '•', TERCIO),
    ],
    anim: Anim::Still,
    weight: 3,
};

/// La Catrina: a skull under a wide-brimmed hat with two great plumes and
/// flowers on the band.
const CATRINA: Figure = Figure {
    nodes: &[
        // Brim: left tip, band left, band right, right tip, under right, under left.
        (0.0, 10.0),
        (8.0, 7.4),
        (18.0, 7.4),
        (26.0, 10.0),
        (17.6, 11.4),
        (8.4, 11.4),
        // Crown.
        (8.8, 3.4),
        (13.0, 1.8),
        (17.2, 3.4),
        // Plumes, curling out from the crown.
        (21.6, -0.6),
        (25.4, -3.0),
        (28.6, -2.4),
        (14.6, -3.0),
        (17.8, -6.8),
        (21.4, -8.2),
        // Face, under the brim.
        (19.0, 15.0),
        (16.8, 19.6),
        (13.0, 21.2),
        (9.2, 19.6),
        (7.0, 15.0),
        // Eyes.
        (10.6, 12.8),
        (12.0, 14.4),
        (10.6, 16.0),
        (9.2, 14.4),
        (15.4, 12.8),
        (16.8, 14.4),
        (15.4, 16.0),
        (14.0, 14.4),
    ],
    edges: &[
        (0, 1, MORADO),
        (1, 6, MORADO),
        (6, 7, MORADO),
        (7, 8, MORADO),
        (8, 2, MORADO),
        (2, 3, MORADO),
        (3, 4, MORADO),
        (4, 5, MORADO),
        (5, 0, MORADO),
        (1, 2, ROSA),
        (8, 9, TURQ),
        (9, 10, TURQ),
        (10, 11, TURQ),
        (7, 12, ROSA),
        (12, 13, ROSA),
        (13, 14, ROSA),
        (4, 15, HUESO),
        (15, 16, HUESO),
        (16, 17, HUESO),
        (17, 18, HUESO),
        (18, 19, HUESO),
        (19, 5, HUESO),
        (20, 21, TURQ),
        (21, 22, TURQ),
        (22, 23, TURQ),
        (23, 20, TURQ),
        (24, 25, TURQ),
        (25, 26, TURQ),
        (26, 27, TURQ),
        (27, 24, TURQ),
    ],
    decor: &[
        // Barbs of the turquoise plume.
        (19.0, 1.9, 20.9, 2.1, TURQ, 2),
        (19.0, 1.9, 18.9, -0.1, TURQ, 2),
        (19.8, 1.1, 21.7, 1.3, TURQ, 2),
        (19.8, 1.1, 19.7, -0.9, TURQ, 2),
        (20.6, 0.4, 22.5, 0.6, TURQ, 2),
        (20.6, 0.4, 20.5, -1.6, TURQ, 2),
        (21.4, -0.4, 23.3, -0.2, TURQ, 2),
        (21.4, -0.4, 21.3, -2.4, TURQ, 2),
        (22.3, -1.0, 24.2, -0.4, TURQ, 2),
        (22.3, -1.0, 22.5, -3.0, TURQ, 2),
        (23.2, -1.6, 25.1, -1.0, TURQ, 2),
        (23.2, -1.6, 23.5, -3.6, TURQ, 2),
        (24.1, -2.2, 26.0, -1.6, TURQ, 2),
        (24.1, -2.2, 24.4, -4.1, TURQ, 2),
        (25.0, -2.8, 26.9, -2.2, TURQ, 2),
        (25.0, -2.8, 25.3, -4.7, TURQ, 2),
        (26.2, -2.9, 27.2, -1.1, TURQ, 2),
        (26.2, -2.9, 27.7, -4.1, TURQ, 2),
        (27.2, -2.7, 28.2, -0.9, TURQ, 2),
        (27.2, -2.7, 28.8, -3.9, TURQ, 2),
        (28.3, -2.5, 29.3, -0.7, TURQ, 2),
        (28.3, -2.5, 29.9, -3.7, TURQ, 2),
        // Barbs of the pink plume.
        (13.6, -0.1, 15.5, -0.9, ROSA, 2),
        (13.6, -0.1, 12.6, -1.8, ROSA, 2),
        (14.0, -1.2, 15.8, -1.9, ROSA, 2),
        (14.0, -1.2, 13.0, -2.9, ROSA, 2),
        (14.3, -2.2, 16.2, -3.0, ROSA, 2),
        (14.3, -2.2, 13.3, -3.9, ROSA, 2),
        (15.1, -3.6, 17.1, -3.6, ROSA, 2),
        (15.1, -3.6, 14.8, -5.5, ROSA, 2),
        (15.8, -4.4, 17.8, -4.5, ROSA, 2),
        (15.8, -4.4, 15.5, -6.4, ROSA, 2),
        (16.5, -5.3, 18.5, -5.3, ROSA, 2),
        (16.5, -5.3, 16.2, -7.2, ROSA, 2),
        (17.2, -6.1, 19.2, -6.1, ROSA, 2),
        (17.2, -6.1, 16.9, -8.1, ROSA, 2),
        (18.5, -7.1, 20.3, -6.2, ROSA, 2),
        (18.5, -7.1, 19.2, -8.9, ROSA, 2),
        (19.5, -7.5, 21.3, -6.6, ROSA, 2),
        (19.5, -7.5, 20.2, -9.3, ROSA, 2),
        (20.6, -7.9, 22.3, -6.9, ROSA, 2),
        (20.6, -7.9, 21.2, -9.7, ROSA, 2),
        // Teeth.
        (11.0, 18.4, 15.0, 18.4, HUESO, 1),
        (12.3, 17.6, 12.3, 19.2, HUESO, 1),
        (13.7, 17.6, 13.7, 19.2, HUESO, 1),
    ],
    marks: &[
        (10.4, 5.4, '✿', TERCIO),
        (13.0, 4.6, '❀', AMARILLO),
        (15.6, 5.4, '✿', CEMPA),
        (4.0, 9.4, '•', TURQ),
        (22.0, 9.4, '•', TURQ),
        (13.0, 16.4, '♥', ROSA),
    ],
    anim: Anim::Still,
    weight: 3,
};

/// A cempasúchil: two ruffles of petals around a golden heart, a stem with
/// two leaves.
const FLOR: Figure = Figure {
    nodes: &[
        // Outer ruffle, from the top, clockwise (center 10, 9.4).
        (10.0, 0.6),
        (12.7, 2.9),
        (16.2, 3.2),
        (16.5, 6.7),
        (18.8, 9.4),
        (16.5, 12.1),
        (16.2, 15.6),
        (12.7, 15.9),
        (10.0, 18.2),
        (7.3, 15.9),
        (3.8, 15.6),
        (3.5, 12.1),
        (1.2, 9.4),
        (3.5, 6.7),
        (3.8, 3.2),
        (7.3, 2.9),
        // Inner ruffle.
        (10.0, 5.0),
        (11.6, 7.1),
        (14.2, 8.0),
        (12.7, 10.3),
        (12.6, 13.0),
        (10.0, 12.2),
        (7.4, 13.0),
        (7.3, 10.3),
        (5.8, 8.0),
        (8.4, 7.1),
        // Stem and leaves.
        (10.4, 21.4),
        (10.2, 24.4),
        (10.0, 27.4),
        (4.6, 20.4),
        (15.6, 23.2),
    ],
    edges: &[
        (0, 1, CEMPA),
        (1, 2, CEMPA),
        (2, 3, CEMPA),
        (3, 4, CEMPA),
        (4, 5, CEMPA),
        (5, 6, CEMPA),
        (6, 7, CEMPA),
        (7, 8, CEMPA),
        (8, 9, CEMPA),
        (9, 10, CEMPA),
        (10, 11, CEMPA),
        (11, 12, CEMPA),
        (12, 13, CEMPA),
        (13, 14, CEMPA),
        (14, 15, CEMPA),
        (15, 0, CEMPA),
        (16, 17, AMARILLO),
        (17, 18, AMARILLO),
        (18, 19, AMARILLO),
        (19, 20, AMARILLO),
        (20, 21, AMARILLO),
        (21, 22, AMARILLO),
        (22, 23, AMARILLO),
        (23, 24, AMARILLO),
        (24, 25, AMARILLO),
        (25, 16, AMARILLO),
        (8, 26, LIMON),
        (26, 27, LIMON),
        (27, 28, LIMON),
        (26, 29, LIMON),
        (29, 27, LIMON),
        (27, 30, LIMON),
        (30, 28, LIMON),
    ],
    decor: &[
        // Folds between the ruffles.
        (12.7, 2.9, 11.6, 5.4, CEMPA_L, 2),
        (16.5, 6.7, 14.0, 7.4, CEMPA_L, 2),
        (16.5, 12.1, 13.6, 11.4, CEMPA_L, 2),
        (12.7, 15.9, 11.8, 13.6, CEMPA_L, 2),
        (7.3, 15.9, 8.2, 13.6, CEMPA_L, 2),
        (3.5, 12.1, 6.4, 11.4, CEMPA_L, 2),
        (3.5, 6.7, 6.0, 7.4, CEMPA_L, 2),
        (7.3, 2.9, 8.4, 5.4, CEMPA_L, 2),
        // Leaf veins.
        (10.4, 21.4, 6.4, 21.0, LIMON, 2),
        (10.2, 24.4, 14.0, 23.6, LIMON, 2),
    ],
    marks: &[(10.0, 9.4, '❀', AMARILLO)],
    anim: Anim::Still,
    weight: 3,
};

/// A monarch, the souls' butterfly, gently opening and closing its wings.
const MONARCA: Figure = Figure {
    nodes: &[
        // Body: head, thorax, waist, tail.
        (10.0, 4.4),
        (10.0, 7.0),
        (10.0, 10.8),
        (10.0, 16.4),
        // Left wings: forewing tip, outer corners, the corner between the
        // wings, hindwing lobe and tail.
        (2.8, 0.2),
        (-0.6, 3.2),
        (0.8, 7.6),
        (4.8, 9.8),
        (2.0, 13.8),
        (5.6, 17.0),
        // Right wings.
        (17.2, 0.2),
        (20.6, 3.2),
        (19.2, 7.6),
        (15.2, 9.8),
        (18.0, 13.8),
        (14.4, 17.0),
        // Antennae.
        (7.2, 0.0),
        (12.8, 0.0),
    ],
    edges: &[
        (16, 0, HUESO),
        (0, 17, HUESO),
        (0, 1, HUESO),
        (1, 2, HUESO),
        (2, 3, HUESO),
        (1, 4, CEMPA),
        (4, 5, CEMPA),
        (5, 6, CEMPA),
        (6, 7, CEMPA),
        (7, 1, CEMPA),
        (7, 8, CEMPA_L),
        (8, 9, CEMPA_L),
        (9, 2, CEMPA_L),
        (1, 10, CEMPA),
        (10, 11, CEMPA),
        (11, 12, CEMPA),
        (12, 13, CEMPA),
        (13, 1, CEMPA),
        (13, 14, CEMPA_L),
        (14, 15, CEMPA_L),
        (15, 2, CEMPA_L),
    ],
    decor: &[
        // Veins.
        (8.0, 6.0, 3.4, 2.4, CEMPA_D, 2),
        (7.6, 7.6, 2.4, 6.4, CEMPA_D, 2),
        (8.8, 11.6, 4.4, 14.2, CEMPA_D, 2),
        (12.0, 6.0, 16.6, 2.4, CEMPA_D, 2),
        (12.4, 7.6, 17.6, 6.4, CEMPA_D, 2),
        (11.2, 11.6, 15.6, 14.2, CEMPA_D, 2),
    ],
    marks: &[
        // The white spots along the black borders.
        (1.2, 1.4, '•', HUESO),
        (-0.2, 5.4, '•', HUESO),
        (2.6, 8.8, '•', HUESO),
        (2.2, 16.0, '•', HUESO),
        (18.8, 1.4, '•', HUESO),
        (20.2, 5.4, '•', HUESO),
        (17.4, 8.8, '•', HUESO),
        (17.8, 16.0, '•', HUESO),
    ],
    anim: Anim::Flap,
    weight: 2,
};

/// A veladora: a candle in a tall glass, its flame flickering.
const VELADORA: Figure = Figure {
    nodes: &[
        // Glass: rim left, rim right, bottom right, bottom left, rim front, rim back.
        (2.0, 12.0),
        (14.0, 12.0),
        (13.2, 25.0),
        (2.8, 25.0),
        (8.0, 13.4),
        (8.0, 10.6),
        // Flame, round at the bottom and drawn up to a point.
        (8.0, 9.0),
        (6.2, 7.6),
        (5.8, 5.0),
        (8.0, -0.4),
        (10.2, 5.0),
        (9.8, 7.6),
    ],
    edges: &[
        (0, 3, ROSA),
        (3, 2, ROSA),
        (2, 1, ROSA),
        (1, 4, ROSA),
        (4, 0, ROSA),
        (0, 5, ROSA),
        (5, 1, ROSA),
        (6, 7, FLAMA),
        (7, 8, FLAMA),
        (8, 9, FLAMA),
        (9, 10, FLAMA),
        (10, 11, FLAMA),
        (11, 6, FLAMA),
    ],
    decor: &[
        // Bands on the glass.
        (2.4, 16.0, 13.6, 16.0, TURQ, 2),
        (2.7, 22.6, 13.3, 22.6, TURQ, 2),
        // Wick and glow.
        (8.0, 8.8, 8.0, 11.2, CEMPA_D, 1),
        (3.2, 1.4, 1.6, 0.0, AMARILLO, 2),
        (12.8, 1.4, 14.4, 0.0, AMARILLO, 2),
        (2.6, 6.0, 0.6, 6.0, AMARILLO, 2),
        (13.4, 6.0, 15.4, 6.0, AMARILLO, 2),
    ],
    marks: &[(8.0, 19.4, '✿', CEMPA), (8.0, 6.0, '•', AMARILLO)],
    anim: Anim::Flicker(9.5),
    weight: 2,
};

const FIGURES: [&Figure; 5] = [&CALAVERA, &CATRINA, &FLOR, &MONARCA, &VELADORA];

/// One figure in the sky.
#[derive(Clone, Copy)]
struct Drawing {
    on: bool,
    fig: usize,
    /// Real seconds since the stars were called.
    age: f32,
    /// Where the figure's center is, in field units, and how it drifts.
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Field units per design unit.
    k: f32,
    /// The figure's design-space center.
    cx: f32,
    cy: f32,
    /// Half its size in field units, for keeping figures apart.
    hw: f32,
    hh: f32,
    /// The star on every node, where it was called from, when it sets off
    /// and how its path bends.
    stars: [u32; MAX_NODES],
    from: [(f32, f32); MAX_NODES],
    delay: [f32; MAX_NODES],
    bend: [f32; MAX_NODES],
}

impl Drawing {
    const OFF: Drawing = Drawing {
        on: false,
        fig: 0,
        age: 0.0,
        x: 0.0,
        y: 0.0,
        vx: 0.0,
        vy: 0.0,
        k: 1.0,
        cx: 0.0,
        cy: 0.0,
        hw: 0.0,
        hh: 0.0,
        stars: [0; MAX_NODES],
        from: [(0.0, 0.0); MAX_NODES],
        delay: [0.0; MAX_NODES],
        bend: [0.0; MAX_NODES],
    };

    /// Design units to field units, with the figure's own motion.
    fn place(&self, x: f32, y: f32) -> (f32, f32) {
        let f = FIGURES[self.fig];
        let (mut dx, mut dy) = (x - self.cx, y - self.cy);
        match f.anim {
            Anim::Still => {}
            Anim::Flap => dx *= 0.78 + 0.22 * (self.age * 1.7).cos(),
            Anim::Flicker(top) if y < top => {
                let up = (top - y) / top.max(1.0);
                dx += up * up * 0.9 * ((self.age * 5.3).sin() + 0.5 * (self.age * 8.9).sin());
                dy -= up * 0.5 * (self.age * 6.1).sin();
            }
            Anim::Flicker(_) => {}
        }
        (self.x + dx * self.k, self.y + dy * self.k)
    }

    /// How lit the finished figure is: up as the spark traces it, down as
    /// the stars let go.
    fn lit(&self) -> f32 {
        let a = self.age;
        if a < GATHER {
            0.0
        } else if a < GATHER + TRACE + HOLD {
            1.0
        } else {
            1.0 - smooth((a - GATHER - TRACE - HOLD) / LET_GO)
        }
    }

    /// How much the figure is there, from the stars' call to their release.
    fn here(&self) -> f32 {
        smooth(self.age / GATHER) * (1.0 - smooth((self.age - GATHER - TRACE - HOLD) / LET_GO))
    }

    /// How much of the icing and flowers shows.
    fn trim(&self) -> f32 {
        let t = self.age - GATHER - TRACE;
        let gone = self.age - GATHER - TRACE - HOLD;
        (smooth(t / 1.4) * (1.0 - smooth(gone / (LET_GO * 0.5)))).clamp(0.0, 1.0)
    }
}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// The holiday's part of the scene.
pub(super) struct Muertos {
    pub on: bool,
    level: f32,
    /// The holiday's own dice, so ordinary days roll exactly as before.
    rng: fastrand::Rng,
    drawings: [Drawing; MAX_DRAWINGS],
    /// Real seconds until the next figure may begin.
    next: f32,
    last: usize,
    pal: [Color; HUES],
    /// The spark that traces figures.
    pen: Color,
}

impl Muertos {
    pub fn new(seed: u64) -> Muertos {
        Muertos {
            on: false,
            level: 0.0,
            rng: fastrand::Rng::with_seed(seed ^ 0x4d55_4552_544f_5321),
            drawings: [Drawing::OFF; MAX_DRAWINGS],
            next: 0.0,
            last: usize::MAX,
            pal: [Color::Reset; HUES],
            pen: Color::Reset,
        }
    }

    /// Follows the holiday; lets go of every figure when it ends.
    pub fn set(&mut self, fiesta: Fiesta, stars: &mut [Star]) {
        let on = fiesta.is(Kind::Muertos);
        if on && !self.on {
            // The first figure soon after the holiday arrives.
            self.next = 0.8;
        }
        if !on {
            self.cancel(stars);
        }
        self.on = on;
        self.level = if on {
            fiesta.level.clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    /// Releases every star at once (the screen changed size, or the holiday
    /// ended).
    pub fn cancel(&mut self, stars: &mut [Star]) {
        for s in stars.iter_mut() {
            s.fig = 0;
            s.glow = 0.0;
            s.gain = 1.0;
        }
        for d in &mut self.drawings {
            d.on = false;
        }
    }

    /// Dresses the sky: holiday star tints, links leaning violet, marigold
    /// meteors.
    pub fn retheme(
        &mut self,
        ctx: &Ctx,
        tints: &mut [Color; 7],
        link_tints: &mut [Color; 7],
        meteor: &mut Color,
        meteor_tail: &mut Color,
    ) {
        let top = ctx.ramp(&[]).at(1.0);
        let tone = |hex: u32| -> Color {
            if ctx.theme.native {
                rgb(hex)
            } else {
                nearest(ctx, rgb(hex))
            }
        };
        for (i, &hex) in NATIVE.iter().enumerate() {
            self.pal[i] = tone(hex);
        }
        if !ctx.theme.native {
            // Bone is the theme's brightest; a flame keeps its own hue.
            self.pal[HUESO as usize] = top;
            self.pal[FLAMA as usize] = rgb(NATIVE[FLAMA as usize]);
        }
        let link = tone(LINK_HUE);
        for (i, &hex) in STAR_HUES.iter().enumerate() {
            tints[i] = if ctx.theme.native {
                rgb(hex)
            } else if i == 1 || i == 2 {
                // The pale ones lean on the theme's brightest.
                crate::color::mix_oklab(top, tone(hex), 0.3)
            } else {
                tone(hex)
            };
            link_tints[i] = crate::color::mix_oklab(tints[i], link, 0.55);
        }
        *meteor = if ctx.theme.native { rgb(0xffe6a8) } else { top };
        *meteor_tail = self.pal[CEMPA as usize];
        self.pen = rgb(NATIVE[FLAMA as usize]);
    }

    /// Runs the figures' clocks (`real` seconds) and moves their stars
    /// (`dt`, scene time).
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        stars: &mut [Star],
        sw: f32,
        sh: f32,
        margin: f32,
        dt: f32,
        real: f32,
        lines: bool,
    ) {
        if !self.on {
            return;
        }
        let real = if real.is_finite() {
            real.clamp(0.0, 1.0)
        } else {
            0.0
        };
        // Stars a figure called fade their ordinary links out; stars let go
        // fade them back in.
        for s in stars.iter_mut() {
            if s.fig == 0 || s.fig == LETTING_GO {
                if s.gain < 1.0 {
                    s.gain = (s.gain + real / 2.5).min(1.0);
                }
            } else {
                s.gain = (s.gain - real / 0.6).max(0.0);
            }
        }
        for slot in 0..MAX_DRAWINGS {
            let d = &mut self.drawings[slot];
            if !d.on {
                continue;
            }
            let was = d.age;
            d.age += real;
            // A slow drift with the sky, never off the screen.
            d.x = (d.x + d.vx * dt).clamp(
                margin + d.hw + 1.0,
                margin + (sw - d.hw - 1.0).max(d.hw + 1.0),
            );
            d.y = (d.y + d.vy * dt).clamp(
                margin + d.hh + 1.0,
                margin + (sh - d.hh - 1.0).max(d.hh + 1.0),
            );
            let f = FIGURES[d.fig];
            let n = f.nodes.len();
            if d.age >= LIFE {
                for &i in &d.stars[..n] {
                    let s = &mut stars[i as usize];
                    s.fig = 0;
                    s.glow = 0.0;
                }
                d.on = false;
                continue;
            }
            let letting = d.age >= GATHER + TRACE + HOLD;
            if letting && was < GATHER + TRACE + HOLD {
                for &i in &d.stars[..n] {
                    stars[i as usize].fig = LETTING_GO;
                }
            }
            let lit = d.lit();
            for (j, &(nx, ny)) in f.nodes.iter().enumerate() {
                let s = &mut stars[d.stars[j] as usize];
                if letting {
                    s.glow = lit;
                    continue;
                }
                let (tx, ty) = d.place(nx, ny);
                let (fx, fy) = d.from[j];
                let go = smooth((d.age - d.delay[j]) / (GATHER - d.delay[j]));
                // A gentle arc on the way in.
                let (ox, oy) = (tx - fx, ty - fy);
                let arc = d.bend[j] * (go * PI).sin();
                s.x = fx + ox * go - oy * arc;
                s.y = fy + oy * go + ox * arc;
                s.glow = if d.age < GATHER { 0.55 * go } else { 1.0 };
            }
        }

        self.next -= real;
        let busy = self.drawings.iter().filter(|d| d.on).count();
        let most = if self.level >= 0.95 {
            3
        } else if self.level >= 0.6 {
            2
        } else {
            1
        };
        if self.next <= 0.0 && busy < most && lines {
            self.next = if self.spawn(stars, sw, sh, margin) {
                let gap = 30.0 + (7.0 - 30.0) * self.level;
                gap * (0.8 + 0.4 * self.rng.f32())
            } else {
                1.5
            };
        }
    }

    /// Calls the stars for a new figure, if there is room for one.
    fn spawn(&mut self, stars: &mut [Star], sw: f32, sh: f32, margin: f32) -> bool {
        let Some(slot) = self.drawings.iter().position(|d| !d.on) else {
            return false;
        };
        let r = &mut self.rng;
        let total: u32 = FIGURES.iter().map(|f| f.weight).sum();
        let mut fig = 0;
        for _ in 0..4 {
            let mut pick = r.u32(0..total);
            for (i, f) in FIGURES.iter().enumerate() {
                if pick < f.weight {
                    fig = i;
                    break;
                }
                pick -= f.weight;
            }
            if fig != self.last {
                break;
            }
        }
        let f = FIGURES[fig];
        let n = f.nodes.len();
        if stars.iter().filter(|s| s.fig == 0).count() < n {
            return false;
        }
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        let ends = f.decor.iter().flat_map(|d| [(d.0, d.1), (d.2, d.3)]);
        let marks = f.marks.iter().map(|m| (m.0, m.1));
        for (x, y) in f.nodes.iter().copied().chain(ends).chain(marks) {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
        let (dw, dh) = (x1 - x0, y1 - y0);
        // Tall enough to read, never wider than most of the screen.
        let want = (sh * 0.62).clamp(14.0, 58.0) * (0.88 + 0.24 * r.f32());
        let k = (want / dh).min(sw * 0.8 / dw);
        let (hw, hh) = (dw * k * 0.5, dh * k * 0.5);
        if hh * 2.0 < 12.0 || hw * 2.0 + 4.0 > sw || hh * 2.0 + 2.0 > sh {
            return false;
        }
        let mut spot = None;
        for _ in 0..12 {
            let x = hw + 2.0 + r.f32() * (sw - 2.0 * hw - 4.0).max(0.0);
            let y = hh + 1.0 + r.f32() * (sh - 2.0 * hh - 2.0).max(0.0);
            let clear = self.drawings.iter().all(|o| {
                !o.on || (x - o.x).abs() > hw + o.hw + 3.0 || (y - o.y).abs() > hh + o.hh + 2.0
            });
            if clear {
                spot = Some((x + margin, y + margin));
                break;
            }
        }
        let Some((x, y)) = spot else {
            return false;
        };
        let mut d = Drawing {
            on: true,
            fig,
            x,
            y,
            vx: (r.f32() - 0.5) * 0.35,
            vy: (r.f32() - 0.5) * 0.2,
            k,
            cx: (x0 + x1) * 0.5,
            cy: (y0 + y1) * 0.5,
            hw,
            hh,
            ..Drawing::OFF
        };
        // Nearest free star to every node, the nodes in a random order so no
        // part of the figure always gets the far ones.
        let mut order = [0u8; MAX_NODES];
        for (i, o) in order.iter_mut().enumerate().take(n) {
            *o = i as u8;
        }
        r.shuffle(&mut order[..n]);
        for &j in &order[..n] {
            let j = j as usize;
            let (tx, ty) = d.place(f.nodes[j].0, f.nodes[j].1);
            let mut best = (f32::MAX, 0);
            for (i, s) in stars.iter().enumerate() {
                if s.fig != 0 {
                    continue;
                }
                let d2 = (s.x - tx).powi(2) + (s.y - ty).powi(2);
                if d2 < best.0 {
                    best = (d2, i);
                }
            }
            let s = &mut stars[best.1];
            s.fig = slot as u8 + 1;
            d.stars[j] = best.1 as u32;
            d.from[j] = (s.x, s.y);
            d.delay[j] = r.f32() * 1.4;
            d.bend[j] = (r.f32() - 0.5) * 0.5;
        }
        self.drawings[slot] = d;
        self.last = fig;
        true
    }

    /// The figures' links and icing, in braille. Returns how many dots of
    /// finished (traced) links it drew.
    pub fn draw_lines(&self, ctx: &Ctx, stars: &[Star], br: &mut Braille, margin: f32) -> usize {
        let mut traced = 0;
        let pos = |i: u32| {
            let s = &stars[i as usize];
            ((s.x - margin) * 2.0, (s.y - margin) * 2.0)
        };
        for d in self.drawings.iter().filter(|d| d.on) {
            let f = FIGURES[d.fig];
            let lit = d.lit();
            // How far the spark has gone, in dots along the edges.
            let mut total = 0.0;
            for &(a, b, _) in f.edges {
                let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                total += (x1 - x0).hypot(y1 - y0);
            }
            let pen = if d.age < GATHER {
                0.0
            } else {
                smooth((d.age - GATHER) / TRACE) * total
            };
            let sketch = SKETCH * smooth(d.age / GATHER * 1.6 - 0.3);
            let mut run = 0.0;
            for (e, &(a, b, hue)) in f.edges.iter().enumerate() {
                let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                let len = (x1 - x0).hypot(y1 - y0);
                // The part the spark has traced is lit, the rest sketched.
                let done = if d.age >= GATHER + TRACE {
                    1.0
                } else {
                    ((pen - run) / len.max(1e-3)).clamp(0.0, 1.0)
                };
                run += len;
                let col = self.pal[hue as usize];
                let phase = d.age * 2.2 + e as f32 * 0.9;
                let n = (len.ceil() as usize).clamp(1, 4096);
                let inv = 1.0 / n as f32;
                for s in 0..=n {
                    let t = s as f32 * inv;
                    let alpha = if t <= done { lit.max(sketch) } else { sketch };
                    if alpha <= 0.0 {
                        continue;
                    }
                    if t <= done && lit >= 1.0 {
                        traced += 1;
                    }
                    if alpha < DISSOLVE {
                        let stride = (DISSOLVE / alpha.max(1e-3)).round() as usize;
                        if stride > 6 || s % stride != 0 {
                            continue;
                        }
                    }
                    // A glimmer runs along the finished figure.
                    let shine = 0.86 + 0.14 * (phase - t * len * 0.15).sin();
                    let v = LINK_FLOOR
                        + (1.0 - LINK_FLOOR)
                            * ((alpha - DISSOLVE) / (1.0 - DISSOLVE)).clamp(0.0, 1.0);
                    br.setf(
                        x0 + (x1 - x0) * t + 0.5,
                        y0 + (y1 - y0) * t + 0.5,
                        ctx.fade(col, 1.0 - v * shine),
                    );
                }
            }
            let trim = d.trim();
            if trim > 0.05 {
                for &(x0, y0, x1, y1, hue, stride) in f.decor {
                    let (a, b) = (d.place(x0, y0), d.place(x1, y1));
                    let (a, b) = (
                        ((a.0 - margin) * 2.0, (a.1 - margin) * 2.0),
                        ((b.0 - margin) * 2.0, (b.1 - margin) * 2.0),
                    );
                    let stride = stride.max(1) as usize * if trim < 0.5 { 2 } else { 1 };
                    let col = ctx.fade(self.pal[hue as usize], 1.0 - trim.max(LINK_FLOOR) * 0.92);
                    let n =
                        ((b.0 - a.0).abs().max((b.1 - a.1).abs()).ceil() as usize).clamp(1, 4096);
                    for s in (0..=n).step_by(stride) {
                        let t = s as f32 / n as f32;
                        br.setf(
                            a.0 + (b.0 - a.0) * t + 0.5,
                            a.1 + (b.1 - a.1) * t + 0.5,
                            col,
                        );
                    }
                }
            }
        }
        traced
    }

    /// The figures' flowers and the tracing spark's head, over everything.
    pub fn draw_marks(&self, ctx: &Ctx, stars: &[Star], c: &mut Canvas, margin: f32) {
        for d in self.drawings.iter().filter(|d| d.on) {
            let f = FIGURES[d.fig];
            let trim = d.trim();
            if trim > 0.3 {
                for &(x, y, ch, hue) in f.marks {
                    let (px, py) = d.place(x, y);
                    let col = ctx.fade(self.pal[hue as usize], 1.0 - trim);
                    let attr = if trim > 0.8 && ch != '·' { BOLD } else { 0 };
                    c.set_attr(
                        (px - margin).floor() as i32,
                        ((py - margin) * 0.5).floor() as i32,
                        ch,
                        col,
                        attr,
                    );
                }
            }
            if d.age >= GATHER && d.age < GATHER + TRACE {
                // Where the spark is along the edges.
                let pos = |i: u32| {
                    let s = &stars[i as usize];
                    (s.x - margin, s.y - margin)
                };
                let mut total = 0.0;
                for &(a, b, _) in f.edges {
                    let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                    total += (x1 - x0).hypot(y1 - y0);
                }
                let mut left = smooth((d.age - GATHER) / TRACE) * total;
                for &(a, b, _) in f.edges {
                    let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                    let len = (x1 - x0).hypot(y1 - y0);
                    if left <= len {
                        let t = left / len.max(1e-3);
                        let (x, y) = (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t);
                        c.set_attr(
                            x.floor() as i32,
                            (y * 0.5).floor() as i32,
                            '✦',
                            self.pen,
                            BOLD,
                        );
                        break;
                    }
                    left -= len;
                }
            }
        }
    }

    /// How much an ordinary link from (ax, ay) to (bx, by) shows: the sky
    /// clears around the figures.
    pub fn clear(&self, ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
        let mut k = 1.0;
        for d in self.drawings.iter().filter(|d| d.on) {
            let here = d.here();
            if here <= 0.0 {
                continue;
            }
            let (mx, my) = ((ax + bx) * 0.5, (ay + by) * 0.5);
            let mut inside: f32 = 0.0;
            for (x, y) in [(ax, ay), (mx, my), (bx, by)] {
                let ox = ((x - d.x).abs() - d.hw - 1.0).max(0.0);
                let oy = ((y - d.y).abs() - d.hh - 1.0).max(0.0);
                inside = inside.max(1.0 - (ox.max(oy) / 4.0).min(1.0));
            }
            k *= 1.0 - here * inside;
        }
        k
    }

    /// How a star of a figure shows: its color leaning to its edges' hue.
    pub fn node_color(&self, s: &Star) -> Color {
        self.pal[s.hue as usize]
    }

    /// Gives each called star the hue of its first edge, for `node_color`.
    pub fn paint_nodes(&self, stars: &mut [Star]) {
        for d in self.drawings.iter().filter(|d| d.on) {
            let f = FIGURES[d.fig];
            for &(a, b, hue) in f.edges.iter().rev() {
                stars[d.stars[a as usize] as usize].hue = hue;
                stars[d.stars[b as usize] as usize].hue = hue;
            }
        }
    }

    #[cfg(test)]
    pub fn figures_on(&self) -> usize {
        self.drawings.iter().filter(|d| d.on).count()
    }
}

#[cfg(test)]
pub(super) fn check_figures() {
    for (i, f) in FIGURES.iter().enumerate() {
        let n = f.nodes.len();
        assert!((3..=MAX_NODES).contains(&n), "figure {i}: {n} nodes");
        for &(a, b, hue) in f.edges {
            assert!((a as usize) < n && (b as usize) < n && a != b, "figure {i}");
            assert!((hue as usize) < HUES);
        }
        // Every node is on some edge, or its star would just sit there.
        for j in 0..n as u8 {
            assert!(
                f.edges.iter().any(|e| e.0 == j || e.1 == j),
                "figure {i} node {j}"
            );
        }
        assert!(f.decor.iter().all(|d| (d.4 as usize) < HUES && d.5 >= 1));
        assert!(
            f.marks
                .iter()
                .all(|m| (m.3 as usize) < HUES && !crate::render::is_wide(m.2))
        );
    }
}

/// The theme accent closest in hue to a holiday color (theme colors that
/// are not RGB leave the holiday color as it is).
fn nearest(ctx: &Ctx, want: Color) -> Color {
    let Some((h0, s0, v0)) = hsv(want) else {
        return want;
    };
    let mut best = (f32::MAX, want);
    for &c in &ctx.theme.colors {
        let Some((h, s, v)) = hsv(c) else {
            continue;
        };
        let dh = (h - h0).abs().min(TAU - (h - h0).abs());
        let score = dh + (s - s0).abs() * 0.8 + (v - v0).abs() * 0.4;
        if score < best.0 {
            best = (score, c);
        }
    }
    best.1
}

/// Hue (radians), saturation and value of an RGB color.
fn hsv(c: Color) -> Option<(f32, f32, f32)> {
    let Color::Rgb(r, g, b) = c else {
        return None;
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let (mx, mn) = (r.max(g).max(b), r.min(g).min(b));
    let d = mx - mn;
    let h = if d <= 1e-6 {
        0.0
    } else if mx == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if mx == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    Some((h / 6.0 * TAU, if mx > 0.0 { d / mx } else { 0.0 }, mx))
}
