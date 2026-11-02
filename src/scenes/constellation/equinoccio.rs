//! The spring equinox in the night sky: now and then a handful of stars leave
//! their drift, glide together and draw a figure with their links
//! (Kukulcán, the feathered serpent rippling with its crest of plumes; El
//! Castillo of Chichén Itzá with its stairway and temple under the sun; the
//! serpiente de luz, seven triangles of sunlight running down the
//! balustrade to the stone serpent's head; the sun with its rays), a spark
//! traces it, its feathers, steps and flames light up, and after a while the
//! stars let go and drift on. The serpiente de luz comes most on the 20th
//! and 21st, when the serpent descends.

use crate::fiesta::{Fiesta, Kind};

use super::holiday::{Anim, Dress, Figure};

// The holiday's colors (indices into the sky's palette).
const PIEDRA: u8 = 0;
const SOMBRA: u8 = 1;
const SOL: u8 = 2;
const SOL_L: u8 = 3;
const JADE: u8 = 4;
const CIELO: u8 = 5;
const BLANCO: u8 = 6;

/// The holiday's colors under `classic`; the sombra is lifted from the shadow's
/// own #3b3226, which would vanish on a night sky.
const NATIVE: [u32; 7] = [
    0xc9b48a, // piedra caliza
    0x8a7452, // sombra, lifted from #3b3226
    0xffcf4a, // sol
    0xffe9a0, // sol, light
    0x3fb374, // jade
    0x7ec8e3, // cielo
    0xf4efe2, // blanco
];

/// Kukulcán, the feathered serpent: a body of jade rippling from its tail to the head with its open jaws and crest of quetzal plumes.
const KUKULCAN: Figure = Figure {
    nodes: &[
        // The tail's tip, then the body: upper edge and lower edge at every step from tail to neck.
        (2.0, 17.0),
        (4.8, 11.3),
        (6.4, 12.8),
        (7.9, 8.8),
        (8.7, 11.4),
        (11.8, 9.1),
        (10.2, 11.9),
        (15.2, 12.0),
        (12.2, 14.3),
        (18.2, 15.8),
        (14.6, 18.2),
        (20.9, 19.4),
        (17.3, 22.3),
        (23.1, 21.2),
        (20.5, 25.7),
        (23.7, 21.2),
        (25.3, 26.5),
        (25.0, 19.9),
        (29.4, 24.0),
        (27.2, 16.6),
        (32.6, 20.2),
        (29.8, 12.3),
        (35.4, 16.3),
        (32.9, 8.4),
        (37.7, 14.0),
        (38.0, 6.1),
        (38.0, 13.9),
        // The head: skull, snout, the tip of the snout, the lower jaw's tip and its corner.
        (40.6, 5.2),
        (45.0, 6.0),
        (49.4, 8.8),
        (46.8, 13.2),
        (41.0, 14.6),
    ],
    edges: &[
        // The back, the head and the belly: one outline from the tail's tip.
        (0, 1, JADE),
        (1, 3, JADE),
        (3, 5, JADE),
        (5, 7, JADE),
        (7, 9, JADE),
        (9, 11, JADE),
        (11, 13, JADE),
        (13, 15, JADE),
        (15, 17, JADE),
        (17, 19, JADE),
        (19, 21, JADE),
        (21, 23, JADE),
        (23, 25, JADE),
        (25, 27, JADE),
        (27, 28, JADE),
        (28, 29, JADE),
        (29, 30, JADE),
        (30, 31, JADE),
        (31, 26, JADE),
        (26, 24, JADE),
        (24, 22, JADE),
        (22, 20, JADE),
        (20, 18, JADE),
        (18, 16, JADE),
        (16, 14, JADE),
        (14, 12, JADE),
        (12, 10, JADE),
        (10, 8, JADE),
        (8, 6, JADE),
        (6, 4, JADE),
        (4, 2, JADE),
        (2, 0, JADE),
    ],
    decor: &[
        // The stripe along the body.
        (5.6, 12.1, 6.6, 11.1, SOL_L, 3),
        (6.6, 11.1, 7.6, 10.4, SOL_L, 3),
        (7.6, 10.4, 8.6, 10.1, SOL_L, 3),
        (8.6, 10.1, 9.7, 10.0, SOL_L, 3),
        (9.7, 10.0, 10.7, 10.4, SOL_L, 3),
        (10.7, 10.4, 11.7, 11.0, SOL_L, 3),
        (11.7, 11.0, 12.7, 11.9, SOL_L, 3),
        (12.7, 11.9, 13.7, 13.1, SOL_L, 3),
        (13.7, 13.1, 14.7, 14.5, SOL_L, 3),
        (14.7, 14.5, 15.7, 16.0, SOL_L, 3),
        (15.7, 16.0, 16.7, 17.5, SOL_L, 3),
        (16.7, 17.5, 17.8, 19.0, SOL_L, 3),
        (17.8, 19.0, 18.8, 20.5, SOL_L, 3),
        (18.8, 20.5, 19.8, 21.7, SOL_L, 3),
        (19.8, 21.7, 20.8, 22.7, SOL_L, 3),
        (20.8, 22.7, 21.8, 23.5, SOL_L, 3),
        (21.8, 23.5, 22.8, 23.9, SOL_L, 3),
        (22.8, 23.9, 23.8, 24.0, SOL_L, 3),
        (23.8, 24.0, 24.8, 23.7, SOL_L, 3),
        (24.8, 23.7, 25.8, 23.2, SOL_L, 3),
        (25.8, 23.2, 26.9, 22.3, SOL_L, 3),
        (26.9, 22.3, 27.9, 21.2, SOL_L, 3),
        (27.9, 21.2, 28.9, 19.8, SOL_L, 3),
        (28.9, 19.8, 29.9, 18.4, SOL_L, 3),
        (29.9, 18.4, 30.9, 16.8, SOL_L, 3),
        (30.9, 16.8, 31.9, 15.3, SOL_L, 3),
        (31.9, 15.3, 32.9, 13.9, SOL_L, 3),
        (32.9, 13.9, 34.0, 12.6, SOL_L, 3),
        (34.0, 12.6, 35.0, 11.5, SOL_L, 3),
        (35.0, 11.5, 36.0, 10.7, SOL_L, 3),
        (36.0, 10.7, 37.0, 10.2, SOL_L, 3),
        (37.0, 10.2, 38.0, 10.0, SOL_L, 3),
        // Feathers along the back.
        (4.8, 11.3, 3.2, 8.7, CIELO, 2),
        (4.8, 11.3, 5.2, 8.5, CIELO, 2),
        (7.9, 8.8, 6.3, 6.2, CIELO, 2),
        (7.9, 8.8, 8.3, 6.0, CIELO, 2),
        (11.8, 9.1, 10.2, 6.5, CIELO, 2),
        (11.8, 9.1, 12.2, 6.3, CIELO, 2),
        (15.2, 12.0, 13.6, 9.4, CIELO, 2),
        (15.2, 12.0, 15.6, 9.2, CIELO, 2),
        (18.2, 15.8, 16.6, 13.2, CIELO, 2),
        (18.2, 15.8, 18.6, 13.0, CIELO, 2),
        (20.9, 19.4, 19.3, 16.8, CIELO, 2),
        (20.9, 19.4, 21.3, 16.6, CIELO, 2),
        (23.1, 21.2, 21.5, 18.6, CIELO, 2),
        (23.1, 21.2, 23.5, 18.4, CIELO, 2),
        (23.7, 21.2, 22.1, 18.6, CIELO, 2),
        (23.7, 21.2, 24.1, 18.4, CIELO, 2),
        (25.0, 19.9, 23.4, 17.3, CIELO, 2),
        (25.0, 19.9, 25.4, 17.1, CIELO, 2),
        (27.2, 16.6, 25.6, 14.0, CIELO, 2),
        (27.2, 16.6, 27.6, 13.8, CIELO, 2),
        (29.8, 12.3, 28.2, 9.7, CIELO, 2),
        (29.8, 12.3, 30.2, 9.5, CIELO, 2),
        (32.9, 8.4, 31.3, 5.8, CIELO, 2),
        (32.9, 8.4, 33.3, 5.6, CIELO, 2),
        (38.0, 6.1, 36.4, 3.5, CIELO, 2),
        (38.0, 6.1, 38.4, 3.3, CIELO, 2),
        // Bands across the body.
        (7.9, 8.8, 8.7, 11.4, PIEDRA, 2),
        (15.2, 12.0, 12.2, 14.3, PIEDRA, 2),
        (20.9, 19.4, 17.3, 22.3, PIEDRA, 2),
        (23.7, 21.2, 25.3, 26.5, PIEDRA, 2),
        (27.2, 16.6, 32.6, 20.2, PIEDRA, 2),
        (32.9, 8.4, 37.7, 14.0, PIEDRA, 2),
        // The crest: plumes fanning back from the skull.
        (40.6, 5.2, 37.2, 1.4, CIELO, 2),
        (40.6, 5.2, 39.2, -0.6, JADE, 2),
        (40.6, 5.2, 42.0, -1.4, CIELO, 2),
        (40.6, 5.2, 35.6, 3.4, JADE, 2),
        (40.6, 5.2, 34.6, 5.6, CIELO, 2),
        // The jaws and their fangs.
        (49.4, 8.8, 41.0, 10.6, SOL, 1),
        (48.4, 9.2, 48.0, 11.4, BLANCO, 1),
        (46.0, 9.8, 45.8, 12.0, BLANCO, 1),
        // The tail's rattle.
        (2.0, 17.0, 0.4, 18.8, SOL, 2),
    ],
    marks: &[(43.6, 7.8, '•', SOL), (50.6, 6.0, '·', SOL_L)],
    anim: Anim::Wave {
        from: 44.0,
        span: -36.0,
        amp: 1.5,
    },
    weight: 3,
};

/// El Castillo of Chichén Itzá: the stepped pyramid with its stairway between the balustrades and the temple on top, the sun above.
const CASTILLO: Figure = Figure {
    nodes: &[
        // The outline up its left side: foot of every step and its corner.
        (0.0, 34.0),
        (0.0, 30.6),
        (3.0, 30.6),
        (3.0, 27.2),
        (6.0, 27.2),
        (6.0, 23.8),
        (9.0, 23.8),
        (9.0, 20.4),
        (12.0, 20.4),
        (12.0, 17.0),
        (14.6, 17.0),
        (14.6, 13.6),
        // The temple's corners on top, left then right.
        (14.6, 6.8),
        (25.4, 6.8),
        // The outline down its right side.
        (40.0, 34.0),
        (40.0, 30.6),
        (37.0, 30.6),
        (37.0, 27.2),
        (34.0, 27.2),
        (34.0, 23.8),
        (31.0, 23.8),
        (31.0, 20.4),
        (28.0, 20.4),
        (28.0, 17.0),
        (25.4, 17.0),
        (25.4, 13.6),
        // The stairway: foot and top of each balustrade.
        (17.4, 34.0),
        (17.4, 13.6),
        (22.6, 34.0),
        (22.6, 13.6),
    ],
    edges: &[
        // The pyramid, from the left foot over the temple to the right foot.
        (0, 1, PIEDRA),
        (1, 2, PIEDRA),
        (2, 3, PIEDRA),
        (3, 4, PIEDRA),
        (4, 5, PIEDRA),
        (5, 6, PIEDRA),
        (6, 7, PIEDRA),
        (7, 8, PIEDRA),
        (8, 9, PIEDRA),
        (9, 10, PIEDRA),
        (10, 11, PIEDRA),
        (11, 12, PIEDRA),
        (12, 13, PIEDRA),
        (13, 25, PIEDRA),
        (25, 24, PIEDRA),
        (24, 23, PIEDRA),
        (23, 22, PIEDRA),
        (22, 21, PIEDRA),
        (21, 20, PIEDRA),
        (20, 19, PIEDRA),
        (19, 18, PIEDRA),
        (18, 17, PIEDRA),
        (17, 16, PIEDRA),
        (16, 15, PIEDRA),
        (15, 14, PIEDRA),
        (11, 12, PIEDRA),
        // The ground and the stairway.
        (0, 26, PIEDRA),
        (28, 14, PIEDRA),
        (26, 27, SOL),
        (28, 29, SOL),
    ],
    decor: &[
        // The terraces: a ledge across every tier.
        (0.0, 30.6, 40.0, 30.6, SOMBRA, 2),
        (3.0, 27.2, 37.0, 27.2, SOMBRA, 2),
        (6.0, 23.8, 34.0, 23.8, SOMBRA, 2),
        (9.0, 20.4, 31.0, 20.4, SOMBRA, 2),
        (12.0, 17.0, 28.0, 17.0, SOMBRA, 2),
        (14.6, 13.6, 25.4, 13.6, SOMBRA, 2),
        // Steps of the stairway.
        (17.4, 34.0, 22.6, 34.0, PIEDRA, 1),
        (17.4, 32.3, 22.6, 32.3, PIEDRA, 1),
        (17.4, 30.6, 22.6, 30.6, PIEDRA, 1),
        (17.4, 28.9, 22.6, 28.9, PIEDRA, 1),
        (17.4, 27.2, 22.6, 27.2, PIEDRA, 1),
        (17.4, 25.5, 22.6, 25.5, PIEDRA, 1),
        (17.4, 23.8, 22.6, 23.8, PIEDRA, 1),
        (17.4, 22.1, 22.6, 22.1, PIEDRA, 1),
        (17.4, 20.4, 22.6, 20.4, PIEDRA, 1),
        (17.4, 18.7, 22.6, 18.7, PIEDRA, 1),
        (17.4, 17.0, 22.6, 17.0, PIEDRA, 1),
        (17.4, 15.3, 22.6, 15.3, PIEDRA, 1),
        // The temple: door, cornice and its columns.
        (17.8, 13.6, 17.8, 9.2, SOMBRA, 1),
        (17.8, 9.2, 22.2, 9.2, SOMBRA, 1),
        (22.2, 9.2, 22.2, 13.6, SOMBRA, 1),
        (14.2, 6.0, 25.8, 6.0, PIEDRA, 1),
        (14.6, 8.0, 25.4, 8.0, SOMBRA, 2),
        (16.0, 8.0, 16.0, 13.6, SOMBRA, 2),
        (24.0, 8.0, 24.0, 13.6, SOMBRA, 2),
        // The sun and its rays.
        (22.6, -4.2, 24.4, -4.2, SOL, 1),
        (21.8, -2.4, 22.5, -1.7, SOL, 1),
        (20.0, -1.6, 20.0, 0.2, SOL, 1),
        (18.2, -2.4, 17.5, -1.7, SOL, 1),
        (17.4, -4.2, 15.6, -4.2, SOL, 1),
        (18.2, -6.0, 17.5, -6.7, SOL, 1),
        (20.0, -6.8, 20.0, -8.6, SOL, 1),
        (21.8, -6.0, 22.5, -6.7, SOL, 1),
    ],
    marks: &[(20.0, -4.2, '✦', SOL), (20.0, 11.4, '•', SOL_L)],
    anim: Anim::Still,
    weight: 3,
};

/// La serpiente de luz: seven triangles of sunlight run down the balustrade of the stairway to the stone serpent's head at its foot, on the pyramid's steps.
const LUZ: Figure = Figure {
    nodes: &[
        // The pyramid's profile: the temple's corner at the top, and the foot of the wall below it.
        (36.0, 4.0),
        (36.0, 34.0),
        // The balustrade, with a triangle of light hanging from it at every step: the points on the balustrade and the points of the triangles.
        (34.8, 5.0),
        (35.6, 9.8),
        (31.0, 8.1),
        (31.8, 12.9),
        (27.2, 11.3),
        (28.0, 16.1),
        (23.4, 14.4),
        (24.2, 19.2),
        (19.6, 17.6),
        (20.4, 22.4),
        (15.8, 20.7),
        (16.6, 25.5),
        (12.0, 23.9),
        (12.7, 28.7),
        (8.2, 27.0),
        // The serpent's head: top of the skull, snout, the tip of the snout, the jaw, the neck.
        (3.4, 28.0),
        (-1.0, 30.2),
        (-0.8, 33.4),
        (3.6, 34.2),
        (8.4, 32.4),
    ],
    edges: &[
        // The balustrade from the temple to the head.
        (2, 4, PIEDRA),
        (4, 6, PIEDRA),
        (6, 8, PIEDRA),
        (8, 10, PIEDRA),
        (10, 12, PIEDRA),
        (12, 14, PIEDRA),
        (14, 16, PIEDRA),
        // The seven triangles of light, tracing the zigzag down.
        (2, 3, SOL),
        (3, 4, SOL),
        (4, 5, SOL),
        (5, 6, SOL),
        (6, 7, SOL),
        (7, 8, SOL),
        (8, 9, SOL),
        (9, 10, SOL),
        (10, 11, SOL),
        (11, 12, SOL),
        (12, 13, SOL),
        (13, 14, SOL),
        (14, 15, SOL),
        (15, 16, SOL),
        // The wall of the pyramid and its foot.
        (2, 0, PIEDRA),
        (0, 1, PIEDRA),
        (1, 21, PIEDRA),
        // The head of the stone serpent.
        (16, 17, JADE),
        (17, 18, JADE),
        (18, 19, JADE),
        (19, 20, JADE),
        (20, 21, JADE),
        (21, 16, JADE),
    ],
    decor: &[
        // The light, filling every triangle.
        (35.6, 9.8, 32.9, 6.5, SOL_L, 2),
        (35.6, 9.8, 35.2, 7.4, SOL_L, 2),
        (35.6, 9.8, 33.3, 8.9, SOL_L, 2),
        (31.8, 12.9, 29.1, 9.7, SOL_L, 2),
        (31.8, 12.9, 31.4, 10.5, SOL_L, 2),
        (31.8, 12.9, 29.5, 12.1, SOL_L, 2),
        (28.0, 16.1, 25.3, 12.9, SOL_L, 2),
        (28.0, 16.1, 27.6, 13.7, SOL_L, 2),
        (28.0, 16.1, 25.7, 15.2, SOL_L, 2),
        (24.2, 19.2, 21.5, 16.0, SOL_L, 2),
        (24.2, 19.2, 23.8, 16.8, SOL_L, 2),
        (24.2, 19.2, 21.9, 18.4, SOL_L, 2),
        (20.4, 22.4, 17.7, 19.1, SOL_L, 2),
        (20.4, 22.4, 20.0, 20.0, SOL_L, 2),
        (20.4, 22.4, 18.1, 21.5, SOL_L, 2),
        (16.6, 25.5, 13.9, 22.3, SOL_L, 2),
        (16.6, 25.5, 16.2, 23.1, SOL_L, 2),
        (16.6, 25.5, 14.3, 24.7, SOL_L, 2),
        (12.7, 28.7, 10.1, 25.4, SOL_L, 2),
        (12.7, 28.7, 12.3, 26.3, SOL_L, 2),
        (12.7, 28.7, 10.4, 27.9, SOL_L, 2),
        // The terraces of the pyramid, behind.
        (36.0, 9.0, 29.5, 9.0, SOMBRA, 2),
        (36.0, 14.0, 29.0, 14.0, SOMBRA, 2),
        (36.0, 19.0, 28.5, 19.0, SOMBRA, 2),
        (36.0, 24.0, 28.0, 24.0, SOMBRA, 2),
        (36.0, 29.0, 27.5, 29.0, SOMBRA, 2),
        // Steps along the balustrade.
        (35.7, 4.8, 36.4, 5.7, PIEDRA, 1),
        (34.6, 5.6, 35.4, 6.6, PIEDRA, 1),
        (33.6, 6.5, 34.4, 7.4, PIEDRA, 1),
        (32.6, 7.3, 33.3, 8.3, PIEDRA, 1),
        (31.6, 8.2, 32.3, 9.1, PIEDRA, 1),
        (30.5, 9.1, 31.3, 10.0, PIEDRA, 1),
        (29.5, 9.9, 30.3, 10.8, PIEDRA, 1),
        (28.5, 10.8, 29.2, 11.7, PIEDRA, 1),
        (27.4, 11.6, 28.2, 12.5, PIEDRA, 1),
        (26.4, 12.5, 27.2, 13.4, PIEDRA, 1),
        (25.4, 13.3, 26.1, 14.2, PIEDRA, 1),
        (24.3, 14.2, 25.1, 15.1, PIEDRA, 1),
        (23.3, 15.0, 24.1, 16.0, PIEDRA, 1),
        (22.3, 15.9, 23.0, 16.8, PIEDRA, 1),
        (21.2, 16.7, 22.0, 17.7, PIEDRA, 1),
        (20.2, 17.6, 21.0, 18.5, PIEDRA, 1),
        (19.2, 18.4, 19.9, 19.4, PIEDRA, 1),
        (18.1, 19.3, 18.9, 20.2, PIEDRA, 1),
        (17.1, 20.1, 17.9, 21.1, PIEDRA, 1),
        (16.1, 21.0, 16.8, 21.9, PIEDRA, 1),
        (15.1, 21.9, 15.8, 22.8, PIEDRA, 1),
        (14.0, 22.7, 14.8, 23.6, PIEDRA, 1),
        (13.0, 23.6, 13.8, 24.5, PIEDRA, 1),
        (12.0, 24.4, 12.7, 25.3, PIEDRA, 1),
        (10.9, 25.3, 11.7, 26.2, PIEDRA, 1),
        (9.9, 26.1, 10.7, 27.0, PIEDRA, 1),
        (8.9, 27.0, 9.6, 27.9, PIEDRA, 1),
        (7.8, 27.8, 8.6, 28.8, PIEDRA, 1),
        // The head's jaws and teeth.
        (-0.8, 31.6, 4.4, 31.6, BLANCO, 1),
        (0.4, 31.6, 0.4, 32.8, BLANCO, 1),
        (2.0, 31.6, 2.0, 32.8, BLANCO, 1),
        // Plumes of the serpent's head.
        (3.4, 28.0, 2.0, 24.8, CIELO, 2),
        (5.0, 28.6, 5.4, 25.6, CIELO, 2),
    ],
    marks: &[
        (2.6, 30.2, '•', SOL),
        (35.4, 2.2, '✦', SOL),
        (15.0, 16.0, '✧', SOL_L),
    ],
    anim: Anim::Still,
    weight: 2,
};

/// The sun at the equinox: a disc with sixteen rays, long and short, the four cardinal ones longest, its glow breathing.
const DISCO: Figure = Figure {
    nodes: &[
        // The disc: a ring of sixteen points.
        (16.0, 9.6),
        (18.4, 10.1),
        (20.5, 11.5),
        (21.9, 13.6),
        (22.4, 16.0),
        (21.9, 18.4),
        (20.5, 20.5),
        (18.4, 21.9),
        (16.0, 22.4),
        (13.6, 21.9),
        (11.5, 20.5),
        (10.1, 18.4),
        (9.6, 16.0),
        (10.1, 13.6),
        (11.5, 11.5),
        (13.6, 10.1),
        // The rays, one from every point of the ring: cardinal, long and short.
        (16.0, 0.6),
        (19.7, 7.1),
        (24.3, 7.7),
        (24.9, 12.3),
        (31.4, 16.0),
        (24.9, 19.7),
        (24.3, 24.3),
        (19.7, 24.9),
        (16.0, 31.4),
        (12.3, 24.9),
        (7.7, 24.3),
        (7.1, 19.7),
        (0.6, 16.0),
        (7.1, 12.3),
        (7.7, 7.7),
        (12.3, 7.1),
    ],
    edges: &[
        // The disc.
        (0, 1, SOL),
        (1, 2, SOL),
        (2, 3, SOL),
        (3, 4, SOL),
        (4, 5, SOL),
        (5, 6, SOL),
        (6, 7, SOL),
        (7, 8, SOL),
        (8, 9, SOL),
        (9, 10, SOL),
        (10, 11, SOL),
        (11, 12, SOL),
        (12, 13, SOL),
        (13, 14, SOL),
        (14, 15, SOL),
        (15, 0, SOL),
        // The rays.
        (0, 16, SOL),
        (1, 17, SOL_L),
        (2, 18, SOL),
        (3, 19, SOL_L),
        (4, 20, SOL),
        (5, 21, SOL_L),
        (6, 22, SOL),
        (7, 23, SOL_L),
        (8, 24, SOL),
        (9, 25, SOL_L),
        (10, 26, SOL),
        (11, 27, SOL_L),
        (12, 28, SOL),
        (13, 29, SOL_L),
        (14, 30, SOL),
        (15, 31, SOL_L),
    ],
    decor: &[
        // The inner ring.
        (19.4, 16.0, 18.9, 17.7, SOL_L, 2),
        (18.9, 17.7, 17.7, 18.9, SOL_L, 2),
        (17.7, 18.9, 16.0, 19.4, SOL_L, 2),
        (16.0, 19.4, 14.3, 18.9, SOL_L, 2),
        (14.3, 18.9, 13.1, 17.7, SOL_L, 2),
        (13.1, 17.7, 12.6, 16.0, SOL_L, 2),
        (12.6, 16.0, 13.1, 14.3, SOL_L, 2),
        (13.1, 14.3, 14.3, 13.1, SOL_L, 2),
        (14.3, 13.1, 16.0, 12.6, SOL_L, 2),
        (16.0, 12.6, 17.7, 13.1, SOL_L, 2),
        (17.7, 13.1, 18.9, 14.3, SOL_L, 2),
        (18.9, 14.3, 19.4, 16.0, SOL_L, 2),
        // Flames between the rays.
        (17.4, 8.7, 17.6, 7.8, SOL, 1),
        (20.1, 9.8, 21.2, 8.2, SOL, 1),
        (22.2, 11.9, 23.0, 11.3, SOL, 1),
        (23.3, 14.6, 25.2, 14.2, SOL, 1),
        (23.3, 17.4, 24.2, 17.6, SOL, 1),
        (22.2, 20.1, 23.8, 21.2, SOL, 1),
        (20.1, 22.2, 20.7, 23.0, SOL, 1),
        (17.4, 23.3, 17.8, 25.2, SOL, 1),
        (14.6, 23.3, 14.4, 24.2, SOL, 1),
        (11.9, 22.2, 10.8, 23.8, SOL, 1),
        (9.8, 20.1, 9.0, 20.7, SOL, 1),
        (8.7, 17.4, 6.8, 17.8, SOL, 1),
        (8.7, 14.6, 7.8, 14.4, SOL, 1),
        (9.8, 11.9, 8.2, 10.8, SOL, 1),
        (11.9, 9.8, 11.3, 9.0, SOL, 1),
        (14.6, 8.7, 14.2, 6.8, SOL, 1),
    ],
    marks: &[
        (16.0, 16.0, '✦', SOL_L),
        (16.0, -1.6, '·', SOL_L),
        (33.6, 16.0, '·', SOL_L),
        (16.0, 33.6, '·', SOL_L),
        (-1.6, 16.0, '·', SOL_L),
    ],
    anim: Anim::Pulse {
        cx: 16.0,
        cy: 16.0,
        r0: 6.5,
        amp: 0.07,
    },
    weight: 2,
};

const FIGURES: [&Figure; 4] = [&KUKULCAN, &CASTILLO, &LUZ, &DISCO];

// The figures, by index.
const LUZ_I: usize = 2;

/// Star tints during the holiday (same weights as the ordinary ones): cielo,
/// pale sun, pale piedra, sol, jade, piedra, light cielo.
const STAR_HUES: [u32; 7] = [
    0x7ec8e3, 0xfff0b8, 0xf4ecd8, 0xffcf4a, 0x5fd09b, 0xc9b48a, 0x9fe0f2,
];
/// What the ordinary links lean toward during the holiday: the serpent's jade.
const LINK_HUE: u32 = 0x3fae78;

/// How often a figure comes: the serpiente de luz three times as often on
/// the days the serpent descends, March 20 and 21.
fn weigh(weight: u32, fig: usize, fiesta: Fiesta) -> u32 {
    match fig {
        LUZ_I if fiesta.level >= 0.95 => weight * 3,
        _ => weight,
    }
}

/// The equinox's figures and colors: stars in cielo, sol, jade and piedra,
/// links leaning jade, sun-colored meteors, a pale sun spark tracing the
/// figures, and under another theme the nearest theme colors. A figure
/// every 30 seconds on the edges of the holiday, every 7 when the serpent
/// descends, never the same one twice at once.
pub(super) static DRESS: Dress = Dress {
    kind: Kind::Equinoccio,
    salt: 0x4551_5549_4e4f_4343,
    figures: &FIGURES,
    weigh,
    distinct: true,
    native: &NATIVE,
    bright: &[],
    keep: &[],
    stars: STAR_HUES,
    pale: &[1, 2],
    link: LINK_HUE,
    meteor: 0xfff0b0,
    tail: SOL,
    pen: SOL_L,
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

    /// Every figure comes on every day of the holiday, and the serpiente de
    /// luz most on the 20th and 21st.
    #[test]
    fn figures_follow_the_days() {
        for day in 19..=22 {
            let fiesta = Fiesta::of_day(3, day);
            assert!(fiesta.is(Kind::Equinoccio), "{day}");
            assert!(weights(fiesta).iter().all(|&w| w > 0), "{day}");
        }
        let edge = weights(Fiesta::of_day(3, 19));
        let height = weights(Fiesta::of_day(3, 20));
        assert!(height[LUZ_I] >= 3 * edge[LUZ_I]);
        assert_eq!(edge[0], height[0]);
    }
}
