//! Día de Muertos in the night sky: now and then a handful of stars leave
//! their drift, glide together and draw a figure with their links (a
//! calavera de azúcar, La Catrina in her feathered hat, a cempasúchil, a
//! monarch butterfly, a veladora), a spark traces it, the icing and flowers
//! light up, and after a while the stars let go and drift on.

use crate::fiesta::Kind;

use super::holiday::{Anim, Dress, Figure};

// The holiday's colors (indices into the sky's palette).
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

/// The holiday's colors under `classic`, a little lighter than the cut paper
/// where they have to read as thin dotted lines on a night sky.
const NATIVE: [u32; 11] = [
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

/// Día de Muertos's figures and colors: bone is the theme's brightest under
/// any other theme and a flame keeps its own hue; marigold meteors, and a
/// flame-colored spark traces the figures. A figure every 30 seconds at the
/// start, every 7 at the height.
pub(super) static DRESS: Dress = Dress {
    kind: Kind::Muertos,
    salt: 0x4d55_4552_544f_5321,
    figures: &FIGURES,
    weigh: |weight, _, _| weight,
    distinct: false,
    native: &NATIVE,
    bright: &[HUESO],
    keep: &[FLAMA],
    stars: STAR_HUES,
    pale: &[1, 2],
    link: LINK_HUE,
    meteor: 0xffe6a8,
    tail: CEMPA,
    pen: FLAMA,
    gap: (30.0, 7.0),
    first: 0.8,
};
