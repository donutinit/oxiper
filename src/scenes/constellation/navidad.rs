//! Navidad in the night sky: now and then a handful of stars leave their
//! drift, glide together and draw a figure with their links (the Estrella de
//! Belén over its long tail, a piñata de siete picos, a flor de nochebuena,
//! a campana, a Rey Mago on his camel, the pesebre), a spark traces it, its
//! papel de china, hay and halos light up, and after a while the stars let
//! go and drift on. The Estrella and the nochebuena come from the start; the
//! campana and the piñata with the posadas, the pesebre a few days later,
//! and the Reyes Magos ride by most on their own days, January 2 to 6.

use crate::fiesta::{Fiesta, Kind};

use super::holiday::{Anim, Dress, Figure};

// The holiday's colors (indices into the sky's palette).
const BLANCO: u8 = 0;
const ROJO: u8 = 1;
const ROJO_L: u8 = 2;
const VERDE: u8 = 3;
const DORADO: u8 = 4;
const DORADO_L: u8 = 5;
const ROSA: u8 = 6;
const AMARILLO: u8 = 7;
const NARANJA: u8 = 8;
const TURQ: u8 = 9;
const MORADO: u8 = 10;
const LIMON: u8 = 11;
const CANELA: u8 = 12;
const CANA: u8 = 13;
const HENO: u8 = 14;

/// The holiday's colors under `classic`, lighter than the paper and the pine
/// where they have to read as thin dotted lines on a night sky.
const NATIVE: [u32; 15] = [
    0xf4f1ea, // blanco
    0xe0213c, // rojo nochebuena
    0xff4b4b, // rojo foquito
    0x2f9a54, // verde pino
    0xf2b632, // dorado
    0xffd978, // dorado, light
    0xe4007c, // rosa mexicano
    0xffd200, // amarillo
    0xff8c00, // naranja
    0x00aab8, // turquesa
    0x8a3cc0, // morado
    0x8cc828, // verde limón
    0xb8692e, // canela
    0xc9b26b, // caña
    0x9aa58a, // heno
];

/// The Estrella de Belén: an eight-pointed star shining over its long, fanning tail.
const ESTRELLA: Figure = Figure {
    nodes: &[
        // The star, clockwise from the top: points and the hollows between them. Its lower left point is the tail's head.
        (30.0, -1.8),
        (31.5, 5.3),
        (35.9, 3.1),
        (33.7, 7.5),
        (40.8, 9.0),
        (33.7, 10.5),
        (35.9, 14.9),
        (31.5, 12.7),
        (30.0, 19.8),
        (28.5, 12.7),
        (26.3, 10.5),
        (19.2, 9.0),
        (26.3, 7.5),
        (24.1, 3.1),
        (28.5, 5.3),
        // The tail, fanning out: its two edges, then the tip.
        (25.1, 16.8),
        (21.7, 13.4),
        (20.0, 23.1),
        (14.1, 17.1),
        (13.6, 27.9),
        (7.1, 21.4),
        (3.2, 28.9),
    ],
    edges: &[
        // The star, from the hollow before its tail around to the hollow after it.
        (10, 11, DORADO),
        (11, 12, DORADO),
        (12, 13, DORADO),
        (13, 14, DORADO),
        (14, 0, DORADO),
        (0, 1, DORADO),
        (1, 2, DORADO),
        (2, 3, DORADO),
        (3, 4, DORADO),
        (4, 5, DORADO),
        (5, 6, DORADO),
        (6, 7, DORADO),
        (7, 8, DORADO),
        (8, 9, DORADO),
        // The tail.
        (9, 15, DORADO_L),
        (10, 16, DORADO_L),
        (15, 17, DORADO_L),
        (17, 19, DORADO_L),
        (16, 18, DORADO_L),
        (18, 20, DORADO_L),
        (19, 21, DORADO_L),
        (20, 21, DORADO_L),
    ],
    decor: &[
        // The star's facets.
        (30.0, 9.0, 31.5, 5.3, DORADO_L, 2),
        (30.0, 9.0, 33.7, 7.5, DORADO_L, 2),
        (30.0, 9.0, 33.7, 10.5, DORADO_L, 2),
        (30.0, 9.0, 31.5, 12.7, DORADO_L, 2),
        (30.0, 9.0, 28.5, 12.7, DORADO_L, 2),
        (30.0, 9.0, 26.3, 10.5, DORADO_L, 2),
        (30.0, 9.0, 26.3, 7.5, DORADO_L, 2),
        (30.0, 9.0, 28.5, 5.3, DORADO_L, 2),
        // Streaks of light along the tail.
        (24.3, 16.0, 13.3, 26.8, DORADO, 3),
        (23.4, 15.1, 7.7, 26.3, BLANCO, 3),
        (22.5, 14.2, 8.4, 21.9, DORADO, 3),
    ],
    marks: &[
        (30.0, 9.0, '✦', BLANCO),
        (45.0, 1.0, '✧', DORADO_L),
        (26.0, -4.0, '✧', DORADO_L),
        (43.0, 19.0, '·', DORADO_L),
    ],
    anim: Anim::Pulse {
        cx: 30.0,
        cy: 9.0,
        r0: 4.0,
        amp: 0.06,
    },
    weight: 3,
};

/// A piñata de siete picos: seven cones in papel de china around a faceted body, strips of paper at every tip, hanging from its rope.
const PINATA: Figure = Figure {
    nodes: &[
        // Rope: the knot above the first tip, and the loop that hangs it.
        (16.0, -6.0),
        (16.0, -1.6),
        // The seven cones, clockwise from the top: tip, then the hollow before the next one.
        (16.0, 4.4),
        (19.2, 10.3),
        (25.9, 9.1),
        (23.2, 15.4),
        (28.3, 19.8),
        (21.8, 21.6),
        (21.5, 28.4),
        (16.0, 24.4),
        (10.5, 28.4),
        (10.2, 21.6),
        (3.7, 19.8),
        (8.8, 15.4),
        (6.1, 9.1),
        (12.8, 10.3),
    ],
    edges: &[
        // The rope.
        (0, 1, CANA),
        (1, 2, CANA),
        // The cones, each in its own paper.
        (15, 2, ROSA),
        (2, 3, ROSA),
        (3, 4, AMARILLO),
        (4, 5, AMARILLO),
        (5, 6, TURQ),
        (6, 7, TURQ),
        (7, 8, MORADO),
        (8, 9, MORADO),
        (9, 10, NARANJA),
        (10, 11, NARANJA),
        (11, 12, LIMON),
        (12, 13, LIMON),
        (13, 14, ROJO),
        (14, 15, ROJO),
    ],
    decor: &[
        // The body's facets: from the center out to every hollow.
        (16.0, 17.0, 19.2, 10.3, BLANCO, 3),
        (16.0, 17.0, 23.2, 15.4, BLANCO, 3),
        (16.0, 17.0, 21.8, 21.6, BLANCO, 3),
        (16.0, 17.0, 16.0, 24.4, BLANCO, 3),
        (16.0, 17.0, 10.2, 21.6, BLANCO, 3),
        (16.0, 17.0, 8.8, 15.4, BLANCO, 3),
        (16.0, 17.0, 12.8, 10.3, BLANCO, 3),
        // Ridge of every cone.
        (16.0, 12.0, 16.0, 4.4, ROSA, 2),
        (19.9, 13.9, 25.9, 9.1, AMARILLO, 2),
        (20.9, 18.1, 28.3, 19.8, TURQ, 2),
        (18.2, 21.5, 21.5, 28.4, MORADO, 2),
        (13.8, 21.5, 10.5, 28.4, NARANJA, 2),
        (11.1, 18.1, 3.7, 19.8, LIMON, 2),
        (12.1, 13.9, 6.1, 9.1, ROJO, 2),
        // Strips of papel de china at every tip.
        (16.0, 4.4, 14.6, 1.5, ROSA, 2),
        (16.0, 4.4, 16.0, 0.2, ROSA, 2),
        (16.0, 4.4, 17.4, 1.5, ROSA, 2),
        (25.9, 9.1, 27.3, 6.2, AMARILLO, 2),
        (25.9, 9.1, 29.2, 6.5, AMARILLO, 2),
        (25.9, 9.1, 29.0, 8.4, AMARILLO, 2),
        (28.3, 19.8, 31.4, 19.1, TURQ, 2),
        (28.3, 19.8, 32.4, 20.7, TURQ, 2),
        (28.3, 19.8, 30.8, 21.8, TURQ, 2),
        (21.5, 28.4, 24.0, 30.4, MORADO, 2),
        (21.5, 28.4, 23.3, 32.2, MORADO, 2),
        (21.5, 28.4, 21.5, 31.6, MORADO, 2),
        (10.5, 28.4, 10.5, 31.6, NARANJA, 2),
        (10.5, 28.4, 8.7, 32.2, NARANJA, 2),
        (10.5, 28.4, 8.0, 30.4, NARANJA, 2),
        (3.7, 19.8, 1.2, 21.8, LIMON, 2),
        (3.7, 19.8, -0.4, 20.7, LIMON, 2),
        (3.7, 19.8, 0.6, 19.1, LIMON, 2),
        (6.1, 9.1, 3.0, 8.4, ROJO, 2),
        (6.1, 9.1, 2.8, 6.5, ROJO, 2),
        (6.1, 9.1, 4.7, 6.2, ROJO, 2),
        (19.2, 17.0, 18.3, 19.3, AMARILLO, 2),
        (18.3, 19.3, 16.0, 20.2, AMARILLO, 2),
        (16.0, 20.2, 13.7, 19.3, AMARILLO, 2),
        (13.7, 19.3, 12.8, 17.0, AMARILLO, 2),
        (12.8, 17.0, 13.7, 14.7, AMARILLO, 2),
        (13.7, 14.7, 16.0, 13.8, AMARILLO, 2),
        (16.0, 13.8, 18.3, 14.7, AMARILLO, 2),
        (18.3, 14.7, 19.2, 17.0, AMARILLO, 2),
    ],
    marks: &[(16.0, 17.0, '✦', AMARILLO)],
    anim: Anim::Swing {
        px: 16.0,
        py: -6.0,
        hw: 40.0,
        amp: 0.07,
    },
    weight: 3,
};

/// A flor de nochebuena: two rings of red bracts around the yellow flowers, two green leaves below.
const NOCHEBUENA: Figure = Figure {
    nodes: &[
        // Outer bracts, clockwise from the top: tip, then the hollow before the next one.
        (16.0, 1.0),
        (18.8, 7.3),
        (25.2, 4.8),
        (22.7, 11.2),
        (29.0, 14.0),
        (22.7, 16.8),
        (25.2, 23.2),
        (18.8, 20.7),
        (16.0, 27.0),
        (13.2, 20.7),
        (6.8, 23.2),
        (9.3, 16.8),
        (3.0, 14.0),
        (9.3, 11.2),
        (6.8, 4.8),
        (13.2, 7.3),
        // Inner bracts.
        (20.5, 7.9),
        (19.2, 12.9),
        (23.2, 16.3),
        (18.0, 16.8),
        (16.0, 21.6),
        (14.0, 16.8),
        (8.8, 16.3),
        (12.8, 12.9),
        (11.5, 7.9),
        (16.0, 10.6),
        // Leaves: the two sides and the tip of each, hanging below.
        (6.4, 21.0),
        (1.6, 28.4),
        (11.4, 25.6),
        (25.6, 21.0),
        (30.4, 28.4),
        (20.6, 25.6),
    ],
    edges: &[
        // Outer bracts.
        (0, 1, ROJO),
        (1, 2, ROJO),
        (2, 3, ROJO),
        (3, 4, ROJO),
        (4, 5, ROJO),
        (5, 6, ROJO),
        (6, 7, ROJO),
        (7, 8, ROJO),
        (8, 9, ROJO),
        (9, 10, ROJO),
        (10, 11, ROJO),
        (11, 12, ROJO),
        (12, 13, ROJO),
        (13, 14, ROJO),
        (14, 15, ROJO),
        (15, 0, ROJO),
        // Inner bracts.
        (16, 17, ROJO_L),
        (17, 18, ROJO_L),
        (18, 19, ROJO_L),
        (19, 20, ROJO_L),
        (20, 21, ROJO_L),
        (21, 22, ROJO_L),
        (22, 23, ROJO_L),
        (23, 24, ROJO_L),
        (24, 25, ROJO_L),
        (25, 16, ROJO_L),
        // Leaves.
        (26, 27, VERDE),
        (27, 28, VERDE),
        (29, 30, VERDE),
        (30, 31, VERDE),
    ],
    decor: &[
        // Midribs of the outer bracts.
        (16.0, 14.0, 16.0, 1.0, ROJO_L, 3),
        (16.0, 14.0, 25.2, 4.8, ROJO_L, 3),
        (16.0, 14.0, 29.0, 14.0, ROJO_L, 3),
        (16.0, 14.0, 25.2, 23.2, ROJO_L, 3),
        (16.0, 14.0, 16.0, 27.0, ROJO_L, 3),
        (16.0, 14.0, 6.8, 23.2, ROJO_L, 3),
        (16.0, 14.0, 3.0, 14.0, ROJO_L, 3),
        (16.0, 14.0, 6.8, 4.8, ROJO_L, 3),
        // Leaf veins.
        (11.0, 20.0, 1.6, 28.4, VERDE, 2),
        (21.0, 20.0, 30.4, 28.4, VERDE, 2),
        (6.4, 21.0, 11.4, 25.6, VERDE, 3),
        (25.6, 21.0, 20.6, 25.6, VERDE, 3),
    ],
    marks: &[
        (16.0, 12.0, '•', AMARILLO),
        (17.9, 13.4, '•', AMARILLO),
        (17.2, 15.6, '•', AMARILLO),
        (14.8, 15.6, '•', AMARILLO),
        (14.1, 13.4, '•', AMARILLO),
        (16.0, 14.0, '✿', AMARILLO),
    ],
    anim: Anim::Still,
    weight: 3,
};

/// A campana de Navidad in gold with a red bow, swinging from its hook; holly berries at the knot.
const CAMPANA: Figure = Figure {
    nodes: &[
        // Bow and hook: the knot, then the loops of the bow.
        (14.0, 0.0),
        (7.6, -3.6),
        (8.4, 1.6),
        (20.4, -3.6),
        (19.6, 1.6),
        // The bell's crown, then its left side from the shoulder down to the flare, and its right.
        (14.0, 3.6),
        (9.8, 5.4),
        (7.4, 9.4),
        (6.2, 14.4),
        (5.4, 19.0),
        (3.2, 22.6),
        (0.8, 25.2),
        (18.2, 5.4),
        (20.6, 9.4),
        (21.8, 14.4),
        (22.6, 19.0),
        (24.8, 22.6),
        (27.2, 25.2),
        // The lip, in front, and the clapper.
        (3.0, 26.8),
        (14.0, 28.2),
        (25.0, 26.8),
        (14.0, 31.6),
    ],
    edges: &[
        // The bow.
        (0, 1, ROJO),
        (1, 2, ROJO),
        (2, 0, ROJO),
        (0, 3, ROJO),
        (3, 4, ROJO),
        (4, 0, ROJO),
        // The bell.
        (0, 5, DORADO),
        (5, 6, DORADO),
        (6, 7, DORADO),
        (7, 8, DORADO),
        (8, 9, DORADO),
        (9, 10, DORADO),
        (10, 11, DORADO),
        (5, 12, DORADO),
        (12, 13, DORADO),
        (13, 14, DORADO),
        (14, 15, DORADO),
        (15, 16, DORADO),
        (16, 17, DORADO),
        (11, 17, DORADO),
        // The lip and the clapper.
        (11, 18, DORADO_L),
        (18, 19, DORADO_L),
        (19, 20, DORADO_L),
        (20, 17, DORADO_L),
        (19, 21, DORADO),
    ],
    decor: &[
        // Bands around the bell.
        (7.2, 10.0, 20.8, 10.0, DORADO_L, 2),
        (6.1, 15.4, 21.9, 15.4, DORADO_L, 2),
        (4.5, 20.6, 23.5, 20.6, DORADO_L, 2),
        // Ribbon tails of the bow.
        (10.6, 0.6, 8.6, 8.0, ROJO, 2),
        (17.4, 0.6, 19.4, 8.0, ROJO, 2),
        // Holly leaves.
        (10.4, -1.8, 11.8, -4.2, VERDE, 2),
        (11.8, -4.2, 13.4, -2.0, VERDE, 2),
        (13.4, -2.0, 14.6, -4.4, VERDE, 2),
        (14.6, -4.4, 16.0, -2.0, VERDE, 2),
        (16.0, -2.0, 17.4, -4.0, VERDE, 2),
        (17.4, -4.0, 17.6, -1.4, VERDE, 2),
    ],
    marks: &[
        (14.0, -1.8, '•', ROJO),
        (12.4, -0.6, '•', ROJO),
        (15.6, -0.6, '•', ROJO),
        (14.0, 18.0, '✦', BLANCO),
    ],
    anim: Anim::Swing {
        px: 14.0,
        py: 0.0,
        hw: 20.0,
        amp: 0.12,
    },
    weight: 2,
};

/// A Rey Mago on his camel, crowned, a gift in his hand, walking toward the star.
const REY: Figure = Figure {
    nodes: &[
        // The camel from the rump along its back (hump, then the saddle), up its neck to the head, and down the throat to the chest and belly.
        (2.4, 14.6),
        (6.6, 13.2),
        (9.6, 8.4),
        (12.8, 12.6),
        (15.4, 13.8),
        (22.0, 13.8),
        (28.6, 10.0),
        (31.0, 4.8),
        (36.6, 6.8),
        (34.6, 8.8),
        (30.6, 9.8),
        (27.0, 18.8),
        (17.5, 19.4),
        (9.0, 18.8),
        (3.4, 17.6),
        // Hooves: front, then back.
        (28.0, 28.4),
        (23.4, 27.8),
        (8.8, 27.8),
        (4.0, 28.4),
        // The king: head, shoulders and the hand with the gift.
        (18.6, 2.2),
        (16.0, 6.6),
        (21.4, 6.8),
        (25.6, 7.6),
    ],
    edges: &[
        // The camel's outline.
        (0, 1, CANA),
        (1, 2, CANA),
        (2, 3, CANA),
        (3, 4, CANA),
        (4, 5, CANA),
        (5, 6, CANA),
        (6, 7, CANA),
        (7, 8, CANA),
        (8, 9, CANA),
        (9, 10, CANA),
        (10, 11, CANA),
        (11, 12, CANA),
        (12, 13, CANA),
        (13, 14, CANA),
        (14, 0, CANA),
        // Legs.
        (11, 15, CANA),
        (12, 16, CANA),
        (13, 17, CANA),
        (14, 18, CANA),
        // The king's cloak and arm.
        (4, 20, MORADO),
        (20, 19, MORADO),
        (19, 21, MORADO),
        (21, 5, MORADO),
        (20, 21, MORADO),
        (21, 22, MORADO),
    ],
    decor: &[
        // Crown.
        (16.9, -0.2, 17.2, -2.4, DORADO, 2),
        (17.2, -2.4, 17.9, -1.2, DORADO, 2),
        (17.9, -1.2, 18.6, -2.8, DORADO, 2),
        (18.6, -2.8, 19.3, -1.2, DORADO, 2),
        (19.3, -1.2, 20.0, -2.4, DORADO, 2),
        (20.0, -2.4, 20.3, -0.2, DORADO, 2),
        (20.3, -0.2, 16.9, -0.2, DORADO, 2),
        // Head of the king.
        (20.2, 2.6, 19.7, 3.7, BLANCO, 3),
        (19.7, 3.7, 18.6, 4.2, BLANCO, 3),
        (18.6, 4.2, 17.5, 3.7, BLANCO, 3),
        (17.5, 3.7, 17.0, 2.6, BLANCO, 3),
        (17.0, 2.6, 17.5, 1.5, BLANCO, 3),
        (17.5, 1.5, 18.6, 1.0, BLANCO, 3),
        (18.6, 1.0, 19.7, 1.5, BLANCO, 3),
        (19.7, 1.5, 20.2, 2.6, BLANCO, 3),
        // The gift: a little chest.
        (24.4, 7.6, 24.4, 10.6, DORADO_L, 2),
        (24.4, 10.6, 27.6, 10.6, DORADO_L, 2),
        (27.6, 10.6, 27.6, 7.6, DORADO_L, 2),
        (24.2, 6.4, 27.8, 6.4, DORADO, 2),
        // Saddle blanket and its fringe.
        (14.8, 13.8, 14.4, 17.8, ROJO, 2),
        (14.4, 17.8, 22.4, 18.0, ROJO, 2),
        (22.4, 18.0, 22.2, 13.8, ROJO, 2),
        (14.4, 17.9, 14.2, 19.9, ROJO, 1),
        (17.0, 17.9, 16.8, 19.9, ROJO, 1),
        (19.6, 17.9, 19.4, 19.9, ROJO, 1),
        (22.4, 17.9, 22.2, 19.9, ROJO, 1),
        // Hump blanket.
        (7.0, 13.4, 12.4, 13.0, ROJO, 2),
        // Tail and ear.
        (2.4, 14.6, 0.6, 19.4, CANA, 2),
        (30.4, 4.4, 29.8, 2.2, CANA, 1),
        // The star ahead.
        (33.0, -7.0, 33.0, -1.0, DORADO_L, 1),
        (30.0, -4.0, 36.0, -4.0, DORADO_L, 1),
    ],
    marks: &[(33.0, -4.0, '✦', DORADO_L), (33.0, 5.6, '•', BLANCO)],
    anim: Anim::Walk {
        top: 19.0,
        amp: 1.1,
    },
    weight: 1,
};

/// The pesebre: a thatched portal, the manger with the Niño Dios shining in it, María kneeling and José beside him, the star over the roof.
const PESEBRE: Figure = Figure {
    nodes: &[
        // The roof: the star at the apex, then the eaves.
        (15.0, 2.0),
        (-1.0, 12.0),
        (31.0, 12.0),
        // Posts: top and foot of each.
        (2.4, 10.4),
        (2.4, 28.0),
        (27.6, 10.4),
        (27.6, 28.0),
        // The manger, rim left, rim right, foot right, foot left, and the Niño Dios in it.
        (10.8, 21.4),
        (19.2, 21.4),
        (17.6, 27.0),
        (12.4, 27.0),
        (15.0, 19.0),
        // María, kneeling: head, shoulders, hem left, hem right, her hand toward the Niño.
        (7.2, 12.4),
        (7.2, 15.6),
        (4.0, 28.0),
        (10.4, 28.0),
        (10.4, 22.4),
        // José: head, shoulders, hem left, hem right.
        (23.0, 11.6),
        (23.0, 14.8),
        (19.6, 28.0),
        (26.2, 28.0),
    ],
    edges: &[
        // The roof.
        (1, 0, HENO),
        (0, 2, HENO),
        // The posts.
        (3, 4, CANELA),
        (5, 6, CANELA),
        // The manger.
        (7, 8, CANELA),
        (8, 9, CANELA),
        (9, 10, CANELA),
        (10, 7, CANELA),
        (11, 7, DORADO_L),
        (11, 8, DORADO_L),
        // María.
        (12, 13, TURQ),
        (13, 14, TURQ),
        (14, 15, TURQ),
        (15, 13, TURQ),
        (13, 16, TURQ),
        // José.
        (17, 18, MORADO),
        (18, 19, MORADO),
        (19, 20, MORADO),
        (20, 18, MORADO),
    ],
    decor: &[
        // Thatch.
        (1.9, 11.8, -0.7, 12.8, HENO, 2),
        (28.1, 11.8, 30.7, 12.8, HENO, 2),
        (4.8, 10.0, 2.2, 11.0, HENO, 2),
        (25.2, 10.0, 27.8, 11.0, HENO, 2),
        (7.6, 8.2, 5.0, 9.2, HENO, 2),
        (22.4, 8.2, 25.0, 9.2, HENO, 2),
        (10.5, 6.4, 7.9, 7.4, HENO, 2),
        (19.5, 6.4, 22.1, 7.4, HENO, 2),
        // Hay in the manger.
        (10.4, 21.4, 11.8, 20.2, HENO, 1),
        (11.8, 20.2, 13.0, 21.4, HENO, 1),
        (13.0, 21.4, 14.4, 20.0, HENO, 1),
        (14.4, 20.0, 15.8, 21.4, HENO, 1),
        (15.8, 21.4, 17.2, 20.2, HENO, 1),
        (17.2, 20.2, 18.4, 21.4, HENO, 1),
        (18.4, 21.4, 19.6, 20.4, HENO, 1),
        // The halo of the Niño Dios.
        (18.0, 19.0, 17.4, 20.8, DORADO_L, 2),
        (17.4, 20.8, 15.9, 21.9, DORADO_L, 2),
        (15.9, 21.9, 14.1, 21.9, DORADO_L, 2),
        (14.1, 21.9, 12.6, 20.8, DORADO_L, 2),
        (12.6, 20.8, 12.0, 19.0, DORADO_L, 2),
        (12.0, 19.0, 12.6, 17.2, DORADO_L, 2),
        (12.6, 17.2, 14.1, 16.1, DORADO_L, 2),
        (14.1, 16.1, 15.9, 16.1, DORADO_L, 2),
        (15.9, 16.1, 17.4, 17.2, DORADO_L, 2),
        (17.4, 17.2, 18.0, 19.0, DORADO_L, 2),
        // The star's rays.
        (15.0, -4.0, 15.0, 0.4, DORADO_L, 1),
        (11.6, 2.0, 13.4, 2.0, DORADO_L, 1),
        (16.6, 2.0, 18.4, 2.0, DORADO_L, 1),
        (12.6, -1.0, 13.6, 0.2, DORADO_L, 1),
        (17.4, -1.0, 16.4, 0.2, DORADO_L, 1),
        // Heads.
        (8.7, 12.4, 8.3, 13.5, BLANCO, 3),
        (8.3, 13.5, 7.2, 13.9, BLANCO, 3),
        (7.2, 13.9, 6.1, 13.5, BLANCO, 3),
        (6.1, 13.5, 5.7, 12.4, BLANCO, 3),
        (5.7, 12.4, 6.1, 11.3, BLANCO, 3),
        (6.1, 11.3, 7.2, 10.9, BLANCO, 3),
        (7.2, 10.9, 8.3, 11.3, BLANCO, 3),
        (8.3, 11.3, 8.7, 12.4, BLANCO, 3),
        (24.5, 11.6, 24.1, 12.7, BLANCO, 3),
        (24.1, 12.7, 23.0, 13.1, BLANCO, 3),
        (23.0, 13.1, 21.9, 12.7, BLANCO, 3),
        (21.9, 12.7, 21.5, 11.6, BLANCO, 3),
        (21.5, 11.6, 21.9, 10.5, BLANCO, 3),
        (21.9, 10.5, 23.0, 10.1, BLANCO, 3),
        (23.0, 10.1, 24.1, 10.5, BLANCO, 3),
        (24.1, 10.5, 24.5, 11.6, BLANCO, 3),
        // José's staff and lantern.
        (27.4, 28.0, 27.4, 13.0, CANELA, 2),
        // Ground.
        (-1.0, 28.6, 31.0, 28.6, VERDE, 2),
    ],
    marks: &[
        (15.0, 19.0, '✦', BLANCO),
        (15.0, 2.0, '✦', BLANCO),
        (27.4, 12.0, '•', DORADO),
        (5.0, 12.2, '•', DORADO_L),
        (7.2, 22.0, '·', TURQ),
    ],
    anim: Anim::Still,
    weight: 2,
};

const FIGURES: [&Figure; 6] = [&ESTRELLA, &PINATA, &NOCHEBUENA, &CAMPANA, &REY, &PESEBRE];

// The figures, by index.
const PINATA_I: usize = 1;
const CAMPANA_I: usize = 3;
const REY_I: usize = 4;
const PESEBRE_I: usize = 5;

/// Star tints during the holiday (same weights as the ordinary ones): verde,
/// pale gold, blanco, dorado, rojo, plata, azul, like a string of foquitos.
const STAR_HUES: [u32; 7] = [
    0x7fe3a0, 0xffe9a8, 0xf4f1ea, 0xf2b632, 0xff6b6b, 0xc9ccd6, 0x7aa8ff,
];
/// What the ordinary links lean toward during the holiday: the pine.
const LINK_HUE: u32 = 0x3aa862;

/// How often a figure comes: the Estrella and the nochebuena from the
/// start, the campana and the piñata once the posadas begin, the pesebre
/// a few days into them, the piñata most in the posadas, and the Reyes
/// Magos on their own days.
fn weigh(weight: u32, fig: usize, fiesta: Fiesta) -> u32 {
    match fig {
        REY_I if fiesta.reyes() => weight * 8,
        PINATA_I if fiesta.posadas() => weight * 2,
        PINATA_I | CAMPANA_I if fiesta.level < 0.35 => 0,
        PESEBRE_I if fiesta.level < 0.5 => 0,
        _ => weight,
    }
}

/// Navidad's figures and colors: pine-green links, golden meteors, a golden
/// spark tracing the figures, and under another theme the nearest theme
/// colors. A figure every 30 seconds on the first days, every 7 at
/// Nochebuena, never the same one twice at once.
pub(super) static DRESS: Dress = Dress {
    kind: Kind::Navidad,
    salt: 0x4e41_5649_4441_4421,
    figures: &FIGURES,
    weigh,
    distinct: true,
    native: &NATIVE,
    bright: &[],
    keep: &[],
    stars: STAR_HUES,
    pale: &[1, 2],
    link: LINK_HUE,
    meteor: 0xfff0c8,
    tail: DORADO,
    pen: DORADO_L,
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

    /// The Estrella (figure 0) and the nochebuena (2) can come every day of
    /// the holiday, the rest arrive with the posadas, and the Reyes Magos
    /// ride most on their days.
    #[test]
    fn figures_follow_the_days() {
        for (month, day) in (12..=31).map(|d| (12, d)).chain((1..=6).map(|d| (1, d))) {
            let fiesta = Fiesta::of_day(month, day);
            assert!(fiesta.is(Kind::Navidad), "{month}-{day}");
            let w = weights(fiesta);
            assert!(w[0] > 0 && w[2] > 0, "{month}-{day}");
        }
        let early = weights(Fiesta::of_day(12, 13));
        assert_eq!(early, [3, 0, 3, 0, 1, 0]);
        let posada = weights(Fiesta::of_day(12, 16));
        assert!(posada[PINATA_I] > 0 && posada[CAMPANA_I] > 0 && posada[PESEBRE_I] == 0);
        let nochebuena = weights(Fiesta::of_day(12, 24));
        assert!(nochebuena.iter().all(|&w| w > 0));
        let reyes = weights(Fiesta::of_day(1, 4));
        assert!(reyes[REY_I] >= 4 * nochebuena[REY_I]);
        assert!(reyes[PESEBRE_I] > 0);
    }
}
