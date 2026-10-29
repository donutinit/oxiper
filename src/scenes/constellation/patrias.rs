//! The Fiestas Patrias in the night sky: now and then a handful of stars
//! leave their drift, glide together and draw a figure with their links (the
//! águila on the nopal with the serpent in its beak, the Campana de Dolores
//! in its belfry, the bandera rippling on its pole, a sombrero de charro), a
//! spark traces it, its feathers, pads, embroidery and cloth light up, and
//! after a while the stars let go and drift on. The bandera and the sombrero
//! come from the first of September, the águila a week into the mes patrio,
//! the campana ten days in, and the campana rings most on the night of the
//! Grito and the day of the parade.

use crate::fiesta::{Fiesta, Kind};

use super::holiday::{Anim, Dress, Figure};

// The holiday's colors (indices into the sky's palette).
const BLANCO: u8 = 0;
const VERDE: u8 = 1;
const ROJO: u8 = 2;
const CAFE: u8 = 3;
const NOPAL: u8 = 4;
const ORO: u8 = 5;
const ORO_L: u8 = 6;
const LAGO: u8 = 7;

/// The holiday's colors under `classic`, a little lighter than the flag's cloth
/// where they have to read as thin dotted lines on a night sky.
const NATIVE: [u32; 8] = [
    0xf4f1ea, // blanco
    0x11a46c, // verde bandera
    0xe4263a, // rojo
    0xcf8a43, // águila café
    0x6bb83a, // nopal
    0xf0bf2d, // oro
    0xffe08a, // oro, light
    0x5aa9d8, // lago
];

/// The águila of the escudo, in profile with a wing raised, a serpent in its beak and claw, standing on a nopal that grows from a rock in the lake.
const AGUILA: Figure = Figure {
    nodes: &[
        // The águila from the tip of its beak over the head and down the back to the tail, and back up the belly and breast to the beak.
        (3.0, 11.0),
        (6.2, 8.0),
        (8.8, 5.4),
        (12.2, 4.0),
        (14.8, 6.6),
        (16.8, 11.0),
        (19.6, 14.4),
        (22.2, 21.0),
        (23.8, 26.0),
        (30.4, 33.4),
        (25.6, 36.0),
        (20.6, 33.4),
        (17.6, 29.0),
        (13.6, 26.0),
        (11.4, 20.4),
        (11.2, 14.6),
        (7.2, 12.8),
        // The raised wing from the shoulder along its leading edge to the tip, around the feathers' tips and back to the body.
        (24.2, 7.2),
        (30.2, 1.8),
        (37.8, 0.0),
        (41.8, 5.4),
        (42.2, 11.6),
        (39.6, 16.8),
        (34.4, 19.6),
        (28.4, 22.2),
        // The serpent from the beak down to the claw, and the claw.
        (1.0, 15.6),
        (4.2, 19.8),
        (1.2, 24.2),
        (4.8, 28.2),
        (11.6, 30.8),
        // The nopal: its pads, and the rock it grows from.
        (9.8, 34.4),
        (17.0, 38.6),
        (25.0, 33.6),
        (3.0, 43.0),
        (33.0, 43.0),
    ],
    edges: &[
        // The águila: head, back, tail, belly, breast.
        (0, 1, CAFE),
        (1, 2, CAFE),
        (2, 3, CAFE),
        (3, 4, CAFE),
        (4, 5, CAFE),
        (5, 6, CAFE),
        (6, 7, CAFE),
        (7, 8, CAFE),
        (8, 9, CAFE),
        (9, 10, CAFE),
        (10, 11, CAFE),
        (11, 12, CAFE),
        (12, 13, CAFE),
        (13, 14, CAFE),
        (14, 15, CAFE),
        (15, 16, CAFE),
        (16, 0, CAFE),
        // The wing.
        (6, 17, ORO),
        (17, 18, ORO),
        (18, 19, ORO),
        (19, 20, ORO),
        (20, 21, ORO),
        (21, 22, ORO),
        (22, 23, ORO),
        (23, 24, ORO),
        (24, 7, ORO),
        // The serpent, from the beak to the claw.
        (0, 25, VERDE),
        (25, 26, VERDE),
        (26, 27, VERDE),
        (27, 28, VERDE),
        (28, 29, VERDE),
        // Leg to the nopal.
        (13, 29, CAFE),
        (29, 30, CAFE),
        // The nopal and its rock.
        (30, 31, NOPAL),
        (31, 32, NOPAL),
        (33, 31, CAFE),
        (31, 34, CAFE),
    ],
    decor: &[
        // Primaries: long feathers from the wing's tips toward its root.
        (30.2, 1.8, 24.0, 12.0, ORO, 3),
        (37.8, 0.0, 28.0, 14.0, ORO, 3),
        (41.8, 5.4, 31.0, 16.0, ORO, 3),
        (42.2, 11.6, 32.0, 19.0, ORO, 3),
        (39.6, 16.8, 30.0, 21.0, ORO, 3),
        // Feathers of the tail.
        (23.8, 26.0, 30.0, 32.6, CAFE, 3),
        (23.8, 26.0, 25.6, 35.0, CAFE, 3),
        (23.8, 26.0, 21.0, 32.6, CAFE, 3),
        // Crest.
        (12.2, 4.0, 14.4, 1.4, CAFE, 2),
        (14.0, 5.0, 17.4, 3.0, CAFE, 2),
        // The hooked beak.
        (3.0, 11.0, 5.4, 11.4, CAFE, 1),
        // Pads of the nopal.
        (23.2, 35.8, 22.4, 37.7, NOPAL, 2),
        (22.4, 37.7, 20.1, 39.1, NOPAL, 2),
        (20.1, 39.1, 17.0, 39.6, NOPAL, 2),
        (17.0, 39.6, 13.9, 39.1, NOPAL, 2),
        (13.9, 39.1, 11.6, 37.7, NOPAL, 2),
        (11.6, 37.7, 10.8, 35.8, NOPAL, 2),
        (10.8, 35.8, 11.6, 33.9, NOPAL, 2),
        (11.6, 33.9, 13.9, 32.5, NOPAL, 2),
        (13.9, 32.5, 17.0, 32.0, NOPAL, 2),
        (17.0, 32.0, 20.1, 32.5, NOPAL, 2),
        (20.1, 32.5, 22.4, 33.9, NOPAL, 2),
        (22.4, 33.9, 23.2, 35.8, NOPAL, 2),
        (13.4, 32.6, 12.7, 34.4, NOPAL, 2),
        (12.7, 34.4, 10.9, 35.5, NOPAL, 2),
        (10.9, 35.5, 8.7, 35.5, NOPAL, 2),
        (8.7, 35.5, 6.9, 34.4, NOPAL, 2),
        (6.9, 34.4, 6.2, 32.6, NOPAL, 2),
        (6.2, 32.6, 6.9, 30.8, NOPAL, 2),
        (6.9, 30.8, 8.7, 29.7, NOPAL, 2),
        (8.7, 29.7, 10.9, 29.7, NOPAL, 2),
        (10.9, 29.7, 12.7, 30.8, NOPAL, 2),
        (12.7, 30.8, 13.4, 32.6, NOPAL, 2),
        (28.8, 31.4, 28.1, 33.3, NOPAL, 2),
        (28.1, 33.3, 26.2, 34.4, NOPAL, 2),
        (26.2, 34.4, 23.8, 34.4, NOPAL, 2),
        (23.8, 34.4, 21.9, 33.3, NOPAL, 2),
        (21.9, 33.3, 21.2, 31.4, NOPAL, 2),
        (21.2, 31.4, 21.9, 29.5, NOPAL, 2),
        (21.9, 29.5, 23.8, 28.4, NOPAL, 2),
        (23.8, 28.4, 26.2, 28.4, NOPAL, 2),
        (26.2, 28.4, 28.1, 29.5, NOPAL, 2),
        (28.1, 29.5, 28.8, 31.4, NOPAL, 2),
        // The lake.
        (-0.5, 44.6, 5.0, 44.6, LAGO, 2),
        (30.0, 44.6, 36.0, 44.6, LAGO, 2),
        (1.5, 46.2, 9.0, 46.2, LAGO, 3),
        (27.0, 46.2, 34.5, 46.2, LAGO, 3),
    ],
    marks: &[
        (7.8, 8.2, '•', BLANCO),
        (8.6, 31.0, '•', ROJO),
        (27.0, 29.4, '•', ROJO),
        (15.4, 33.2, '•', ROJO),
        (30.4, 11.4, '✦', ORO_L),
    ],
    anim: Anim::Wave {
        from: 18.0,
        span: 16.0,
        amp: 0.9,
    },
    weight: 3,
};

/// The Campana de Dolores: the great bell swinging in its belfry, under a pediment with its cross.
const DOLORES: Figure = Figure {
    nodes: &[
        // The belfry: the apex of the pediment, then the left post (top, foot) and the right.
        (14.0, -6.0),
        (0.0, 1.0),
        (0.0, 37.0),
        (28.0, 1.0),
        (28.0, 37.0),
        // The yoke and the bell, from the crown down its left side to the flare, and its right.
        (14.0, 3.0),
        (14.0, 7.6),
        (10.4, 9.4),
        (8.2, 13.4),
        (7.0, 18.4),
        (6.2, 23.4),
        (4.4, 28.0),
        (1.8, 31.6),
        (17.6, 9.4),
        (19.8, 13.4),
        (21.0, 18.4),
        (21.8, 23.4),
        (23.6, 28.0),
        (26.2, 31.6),
        (14.0, 35.0),
    ],
    edges: &[
        // The belfry.
        (2, 1, BLANCO),
        (1, 3, BLANCO),
        (3, 4, BLANCO),
        (1, 0, BLANCO),
        (0, 3, BLANCO),
        // The bell.
        (5, 6, ORO),
        (6, 7, ORO),
        (7, 8, ORO),
        (8, 9, ORO),
        (9, 10, ORO),
        (10, 11, ORO),
        (11, 12, ORO),
        (6, 13, ORO),
        (13, 14, ORO),
        (14, 15, ORO),
        (15, 16, ORO),
        (16, 17, ORO),
        (17, 18, ORO),
        (12, 18, ORO_L),
        (12, 19, ORO_L),
        (18, 19, ORO_L),
    ],
    decor: &[
        // Beam the bell hangs from.
        (0.0, 3.0, 28.0, 3.0, BLANCO, 2),
        // The cross over the pediment.
        (14.0, -12.0, 14.0, -6.4, ORO, 1),
        (11.8, -10.2, 16.2, -10.2, ORO, 1),
        // Bands around the bell.
        (9.0, 13.0, 19.0, 13.0, ORO_L, 2),
        (6.6, 20.0, 21.4, 20.0, ORO_L, 2),
        (5.4, 26.0, 22.6, 26.0, ORO_L, 2),
    ],
    marks: &[(14.0, 17.0, '✦', ORO_L), (14.0, -2.6, '•', ROJO)],
    anim: Anim::Swing {
        px: 14.0,
        py: 3.0,
        hw: 9.8,
        amp: 0.14,
    },
    weight: 2,
};

/// The bandera on its pole: verde, blanco and rojo with the escudo in the middle, rippling in the wind.
const BANDERA: Figure = Figure {
    nodes: &[
        // The pole: its finial and its foot.
        (1.0, -1.5),
        (1.0, 33.0),
        // The flag's top edge from the pole to the fly.
        (2.0, 2.0),
        (6.5, 2.0),
        (11.0, 2.0),
        (15.5, 2.0),
        (20.0, 2.0),
        (24.5, 2.0),
        (29.0, 2.0),
        // Its bottom edge.
        (2.0, 17.5),
        (6.5, 17.5),
        (11.0, 17.5),
        (15.5, 17.5),
        (20.0, 17.5),
        (24.5, 17.5),
        (29.0, 17.5),
    ],
    edges: &[
        // The pole.
        (0, 1, CAFE),
        // The top edge: verde, blanco, rojo.
        (2, 3, VERDE),
        (3, 4, VERDE),
        (4, 5, BLANCO),
        (5, 6, BLANCO),
        (6, 7, ROJO),
        (7, 8, ROJO),
        // The bottom edge.
        (9, 10, VERDE),
        (10, 11, VERDE),
        (11, 12, BLANCO),
        (12, 13, BLANCO),
        (13, 14, ROJO),
        (14, 15, ROJO),
        // The edges of the bands.
        (2, 9, VERDE),
        (4, 11, BLANCO),
        (6, 13, BLANCO),
        (8, 15, ROJO),
    ],
    decor: &[
        // Cloth: verde.
        (3.4, 2.4, 3.4, 17.1, VERDE, 3),
        (4.8, 2.4, 4.8, 17.1, VERDE, 3),
        (6.2, 2.4, 6.2, 17.1, VERDE, 3),
        (7.6, 2.4, 7.6, 17.1, VERDE, 3),
        (9.0, 2.4, 9.0, 17.1, VERDE, 3),
        (10.2, 2.4, 10.2, 17.1, VERDE, 3),
        // Cloth: rojo.
        (21.0, 2.4, 21.0, 17.1, ROJO, 3),
        (22.4, 2.4, 22.4, 17.1, ROJO, 3),
        (23.8, 2.4, 23.8, 17.1, ROJO, 3),
        (25.2, 2.4, 25.2, 17.1, ROJO, 3),
        (26.6, 2.4, 26.6, 17.1, ROJO, 3),
        (28.0, 2.4, 28.0, 17.1, ROJO, 3),
        // The escudo on the white band: its ring, the eagle's wings, and the nopal.
        (18.9, 9.8, 18.4, 11.5, CAFE, 2),
        (18.4, 11.5, 17.2, 12.7, CAFE, 2),
        (17.2, 12.7, 15.5, 13.2, CAFE, 2),
        (15.5, 13.2, 13.8, 12.7, CAFE, 2),
        (13.8, 12.7, 12.6, 11.5, CAFE, 2),
        (12.6, 11.5, 12.1, 9.8, CAFE, 2),
        (12.1, 9.8, 12.6, 8.1, CAFE, 2),
        (12.6, 8.1, 13.8, 6.9, CAFE, 2),
        (13.8, 6.9, 15.5, 6.4, CAFE, 2),
        (15.5, 6.4, 17.2, 6.9, CAFE, 2),
        (17.2, 6.9, 18.4, 8.1, CAFE, 2),
        (18.4, 8.1, 18.9, 9.8, CAFE, 2),
        (12.6, 8.2, 14.0, 7.2, ORO, 1),
        (14.0, 7.2, 15.5, 8.2, ORO, 1),
        (15.5, 8.2, 17.0, 7.2, ORO, 1),
        (17.0, 7.2, 18.4, 8.2, ORO, 1),
        (14.2, 8.6, 15.5, 10.6, CAFE, 1),
        (15.5, 10.6, 16.8, 8.6, CAFE, 1),
        (13.4, 12.6, 15.5, 11.6, NOPAL, 1),
        (15.5, 11.6, 17.6, 12.6, NOPAL, 1),
        // Foot of the pole.
        (-0.6, 33.4, 2.6, 33.4, CAFE, 1),
        // Lance at the head of the pole.
        (1.0, -1.5, 0.0, 1.2, ORO, 1),
        (1.0, -1.5, 2.0, 1.2, ORO, 1),
    ],
    marks: &[(1.0, -2.4, '●', ORO), (15.5, 9.8, '•', ROJO)],
    anim: Anim::Wave {
        from: 3.0,
        span: 12.0,
        amp: 1.5,
    },
    weight: 3,
};

/// A sombrero de charro: the wide brim turned up at its edges, the high crown, the band with its embroidery in oro and the barboquejo.
const SOMBRERO: Figure = Figure {
    nodes: &[
        // The brim's upper edge, from the left tip to the right.
        (0.0, 14.4),
        (9.0, 12.6),
        (18.0, 11.8),
        (27.0, 12.6),
        (36.0, 14.4),
        // The brim's front edge, back from the right.
        (27.0, 18.4),
        (18.0, 19.8),
        (9.0, 18.4),
        // The crown, up its left side to the peak and down the right.
        (10.4, 6.0),
        (13.4, 1.4),
        (18.0, 0.0),
        (22.6, 1.4),
        (25.6, 6.0),
        // The barboquejo, hanging from the brim to the front.
        (18.0, 24.4),
    ],
    edges: &[
        // The brim.
        (0, 1, ROJO),
        (1, 2, ROJO),
        (2, 3, ROJO),
        (3, 4, ROJO),
        (4, 5, ROJO),
        (5, 6, ROJO),
        (6, 7, ROJO),
        (7, 0, ROJO),
        // The crown.
        (1, 8, VERDE),
        (8, 9, VERDE),
        (9, 10, VERDE),
        (10, 11, VERDE),
        (11, 12, VERDE),
        (12, 3, VERDE),
        // The barboquejo.
        (7, 13, ORO),
        (13, 5, ORO),
    ],
    decor: &[
        // The band around the crown.
        (9.8, 8.4, 26.2, 8.4, BLANCO, 1),
        (9.4, 11.0, 26.6, 11.0, BLANCO, 1),
        // Embroidery on the band: a greca of oro.
        (10.6, 9.7, 12.2, 8.9, ORO, 1),
        (12.2, 8.9, 13.8, 9.7, ORO, 1),
        (13.8, 9.7, 15.4, 8.9, ORO, 1),
        (15.4, 8.9, 17.0, 9.7, ORO, 1),
        (17.0, 9.7, 18.6, 8.9, ORO, 1),
        (18.6, 8.9, 20.2, 9.7, ORO, 1),
        (20.2, 9.7, 21.8, 8.9, ORO, 1),
        (21.8, 8.9, 23.4, 9.7, ORO, 1),
        (23.4, 9.7, 25.0, 8.9, ORO, 1),
        // Embroidery along the brim.
        (3.0, 14.8, 4.4, 15.8, ORO, 1),
        (4.4, 15.8, 5.8, 14.8, ORO, 1),
        (5.8, 14.8, 7.2, 15.9, ORO, 1),
        (33.0, 14.8, 31.6, 15.8, ORO, 1),
        (31.6, 15.8, 30.2, 14.8, ORO, 1),
        (30.2, 14.8, 28.8, 15.9, ORO, 1),
        (11.0, 18.4, 12.4, 19.3, ORO, 1),
        (12.4, 19.3, 13.8, 18.4, ORO, 1),
        (13.8, 18.4, 15.2, 19.4, ORO, 1),
        (15.2, 19.4, 16.6, 18.6, ORO, 1),
        (25.0, 18.4, 23.6, 19.3, ORO, 1),
        (23.6, 19.3, 22.2, 18.4, ORO, 1),
        (22.2, 18.4, 20.8, 19.4, ORO, 1),
        (20.8, 19.4, 19.4, 18.6, ORO, 1),
        // The brim's thickness.
        (1.0, 15.6, 9.0, 19.6, ROJO, 2),
        (9.0, 19.6, 18.0, 21.0, ROJO, 2),
        (18.0, 21.0, 27.0, 19.6, ROJO, 2),
        (27.0, 19.6, 35.0, 15.6, ROJO, 2),
        // Pinch of the crown.
        (15.0, 1.0, 17.0, 5.6, VERDE, 2),
        (21.0, 1.0, 19.0, 5.6, VERDE, 2),
    ],
    marks: &[(18.0, 9.7, '✿', ORO), (18.0, -1.4, '•', ORO)],
    anim: Anim::Still,
    weight: 2,
};

const FIGURES: [&Figure; 4] = [&AGUILA, &DOLORES, &BANDERA, &SOMBRERO];

// The figures, by index.
const AGUILA_I: usize = 0;
const DOLORES_I: usize = 1;

/// Star tints during the holiday (same weights as the ordinary ones): half
/// of them white, the rest verde and rojo like the flag.
const STAR_HUES: [u32; 7] = [
    0x66d9a0, 0xf8f4ec, 0xf4f1ea, 0xff6b78, 0x3fc486, 0xf0bf2d, 0xff8f98,
];
/// What the ordinary links lean toward during the holiday: the flag's verde.
const LINK_HUE: u32 = 0x1fa06a;

/// How often a figure comes: the bandera and the sombrero from the start of
/// September, the águila once the month is a week old, the campana at ten
/// days, and the campana three times as often on the Grito and the parade.
fn weigh(weight: u32, fig: usize, fiesta: Fiesta) -> u32 {
    match fig {
        AGUILA_I if fiesta.level < 0.4 => 0,
        DOLORES_I if fiesta.level >= 0.8 => weight * 3,
        DOLORES_I if fiesta.level < 0.48 => 0,
        _ => weight,
    }
}

/// The Fiestas Patrias's figures and colors: verde, blanco y rojo stars,
/// links leaning verde, meteors whose white heads fade to rojo, an oro
/// spark tracing the figures, and under another theme the nearest theme
/// colors. A figure every 30 seconds in the first days of September, every
/// 7 on the night of the Grito, never the same one twice at once.
pub(super) static DRESS: Dress = Dress {
    kind: Kind::Patrias,
    salt: 0x5041_5452_4941_5321,
    figures: &FIGURES,
    weigh,
    distinct: true,
    native: &NATIVE,
    bright: &[],
    keep: &[],
    stars: STAR_HUES,
    pale: &[1, 2],
    link: LINK_HUE,
    meteor: 0xfff4d8,
    tail: ROJO,
    pen: ORO_L,
    gap: (30.0, 7.0),
    first: 0.8,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn weights(fiesta: Fiesta) -> Vec<u32> {
        (0..FIGURES.len())
            .map(|i| weigh(FIGURES[i].weight, i, fiesta))
            .collect()
    }

    /// The bandera (figure 2) and the sombrero (3) can come every day of the
    /// holiday, the águila and the campana join as the mes patrio goes, and
    /// the campana comes most on the Grito.
    #[test]
    fn figures_follow_the_days() {
        for day in 1..=16 {
            let fiesta = Fiesta::of_day(9, day);
            assert!(fiesta.is(Kind::Patrias), "{day}");
            let w = weights(fiesta);
            assert!(w[2] > 0 && w[3] > 0, "{day}");
        }
        assert_eq!(weights(Fiesta::of_day(9, 2)), [0, 0, 3, 2]);
        let week = weights(Fiesta::of_day(9, 9));
        assert!(week[AGUILA_I] > 0 && week[DOLORES_I] == 0);
        let late = weights(Fiesta::of_day(9, 12));
        assert!(late.iter().all(|&w| w > 0));
        let grito = weights(Fiesta::of_day(9, 15));
        assert!(grito[DOLORES_I] >= 3 * late[DOLORES_I]);
        assert!(weights(Fiesta::of_day(9, 16))[DOLORES_I] > late[DOLORES_I]);
    }
}
