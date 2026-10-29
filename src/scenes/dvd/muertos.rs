//! The bouncing logo on Día de Muertos: a calavera de azúcar crowned with
//! cempasúchil, shedding petals that lie along the bottom row.

use super::{Air, BitKind, Fall, Look, React, Size, Tint, Trim};
use crate::color::{Color, rgb};

/// Día de Muertos: the icing cycles through papel picado colors, ordered so
/// that each one and the one three places on (the eye flowers) contrast.
const PAPEL: [Color; 6] = [
    rgb(0xe4007c), // rosa mexicano
    rgb(0xff8c00), // naranja
    rgb(0x7028a0), // morado
    rgb(0xffd200), // amarillo
    rgb(0x00aab8), // turquesa
    rgb(0x8cc828), // verde limón
];
pub(super) const HUESO: Color = rgb(0xf4eee2);
/// Cempasúchil and terciopelo, in pairs a tumbling petal alternates between.
const MARIGOLD: [Color; 6] = [
    rgb(0xff8c00),
    rgb(0xd66200),
    rgb(0xffaa1e),
    rgb(0xffc83c),
    rgb(0xb00048),
    rgb(0xd61e64),
];
const LEAF: Color = rgb(0x4e9a28);

/// The sugar skulls, one char per pixel and two pixel rows per cell: `w`
/// bone, `i` icing (the bounce color), `j` the eye flowers' petals, `.`
/// nothing. A cell never holds two colors: where bone meets icing in one,
/// the bone half stays dark, which outlines the icing.
const CALAVERA: [&str; 24] = [
    "..........wwwwwwwww..........",
    ".......wwwwwwwwwwwwwww.......",
    ".....wwwwwwwwwwwwwwwwwww.....",
    "....wwwwwwwwwwwwwwwwwwwww....",
    "...wwwiiiiiiiiiiiiiiiiiwww...",
    "..wwwii...ii.iii.i...iiiwww..",
    "..wwwii.ii.i.iii.i.ii.iiwww..",
    ".wwwwii.ii.ii.i.ii.ii.iiwwww.",
    ".wwwwii...iiii.iii...iiiwwww.",
    ".wwwwwiiiiiiiiiiiiiiiiiwwwww.",
    ".ww.......wwwwwwwww.......ww.",
    "ww....j....wwwwwww....j....ww",
    "ww...jjj...wwwwwww...jjj...ww",
    "ww..jjijj..wwwwwww..jjijj..ww",
    "ww..jjijj..wwwwwww..jjijj..ww",
    "ww...jjj...wwwwwww...jjj...ww",
    ".ww...j...wwiiwiiww...j...ww.",
    ".www.....wwwiiiiiwww.....www.",
    ".wwjwiwjwwwwwiiiwwwwwjwiwjww.",
    "..wwwwwwwwwwwwiwwwwwwwwwwww..",
    "...www.w.w.w.w.w.w.w.w.www...",
    "...www.w.w.w.w.w.w.w.w.www...",
    "....wwwwwwwwwwwwwwwwwwwww....",
    "......wwwwwwwwwwwwwwwww......",
];
const CALAVERITA: [&str; 20] = [
    "......wwwwwww......",
    "...wwwwwwwwwwwww...",
    "..wiiiiiiiiiiiiiw..",
    ".wii..ii.i.ii..iiw.",
    ".wii.i.i.i.ii.i.iw.",
    ".wii.i.i.i.ii.i.iw.",
    ".wii..iii.iii..iiw.",
    ".wwiiiiiiiiiiiiiww.",
    ".ww....wwwww....ww.",
    "ww..jj..www..jj..ww",
    "ww.jiij.www.jiij.ww",
    "ww.jiij.www.jiij.ww",
    ".w..jj..w.w..jj..w.",
    ".ww....ww.ww....ww.",
    ".wjwwwwwwwwwwwwwjw.",
    "..wwwwwwwwwwwwwww..",
    "...w.w.w.w.w.w.w...",
    "...w.w.w.w.w.w.w...",
    "....wwwwwwwwwww....",
    "......wwwwwww......",
];
const CALAQUITA: [&str; 10] = [
    "...wwwww...",
    ".wwwwwwwww.",
    ".wiiiiiiiw.",
    "wwiiiiiiiww",
    "w...www...w",
    "w.j.www.j.w",
    "w...w.w...w",
    ".wwwwwwwww.",
    "..w.w.w.w..",
    "...wwwww...",
];
/// A skull's pictures: the rows that replace its last ones when it laughs
/// (the mouth open), and its crown.
const SIZES: [Size; 3] = [
    Size {
        frames: &[&CALAVERA],
        react: &[
            "...www.w.w.w.w.w.w.w.w.www...",
            "...ww...................ww...",
            "....ww.w.w.w.w.w.w.w.w.ww....",
            "......wwwwwwwwwwwwwwwww......",
        ],
        trim: Some(&CROWN),
    },
    Size {
        frames: &[&CALAVERITA],
        react: &[
            "...w.w.w.w.w.w.w...",
            "...w...........w...",
            "....w.w.w.w.w.w....",
            "......wwwwwww......",
        ],
        trim: Some(&CROWNLET),
    },
    Size {
        frames: &[&CALAQUITA],
        react: &["..w.....w..", "...wwwww..."],
        trim: None,
    },
];

/// A crown of flowers for a skull: the shape of one flower (`x` marks it),
/// where each one sits (center column, top pixel row, counting two rows
/// above the skull; and its color: `o` cempasúchil, `y` light cempasúchil,
/// `r` terciopelo) in the order they come as the holiday builds up, and
/// the leaves (`g`) of the full crown.
const CROWN: Trim = Trim {
    piece: &[".xxx.", "xxxxx", "xxxxx", ".xxx."],
    pieces: &[
        (8, 2, b'o'),
        (14, 0, b'y'),
        (20, 2, b'o'),
        (3, 4, b'r'),
        (25, 4, b'r'),
    ],
    leaves: &[(11, 2), (17, 2), (5, 3), (23, 3)],
    leaf: b'g',
    above: 2,
};
const CROWNLET: Trim = Trim {
    piece: &[".x.", "xxx", "xxx", ".x."],
    pieces: &[
        (5, 0, b'o'),
        (9, 0, b'y'),
        (13, 0, b'o'),
        (2, 2, b'r'),
        (16, 2, b'r'),
    ],
    leaves: &[(7, 1), (11, 1)],
    leaf: b'g',
    above: 2,
};

const PETAL_GLYPHS: [char; 2] = ['◖', '◗'];
const BLOOM_GLYPHS: [char; 2] = ['✿', '❀'];

/// Petals and blossoms in their three pairs of shades (see `MARIGOLD`); a
/// kind is its shade, plus three for a blossom.
const BITS: [BitKind; 6] = [
    BitKind::new(
        PETAL_GLYPHS,
        [Tint::Own(MARIGOLD[0]), Tint::Own(MARIGOLD[1])],
        2.2,
    ),
    BitKind::new(
        PETAL_GLYPHS,
        [Tint::Own(MARIGOLD[2]), Tint::Own(MARIGOLD[3])],
        2.2,
    ),
    BitKind::new(
        PETAL_GLYPHS,
        [Tint::Own(MARIGOLD[4]), Tint::Own(MARIGOLD[5])],
        2.2,
    ),
    BitKind::new(
        BLOOM_GLYPHS,
        [Tint::Own(MARIGOLD[0]), Tint::Own(MARIGOLD[1])],
        3.2,
    ),
    BitKind::new(
        BLOOM_GLYPHS,
        [Tint::Own(MARIGOLD[2]), Tint::Own(MARIGOLD[3])],
        3.2,
    ),
    BitKind::new(
        BLOOM_GLYPHS,
        [Tint::Own(MARIGOLD[4]), Tint::Own(MARIGOLD[5])],
        3.2,
    ),
];

/// A petal's pair of shades: mostly cempasúchil, some terciopelo.
fn petal_shade(r: &mut fastrand::Rng) -> u8 {
    match r.u8(..20) {
        0..=8 => 0,
        9..=16 => 1,
        _ => 2,
    }
}

/// A petal, or now and then a whole blossom (more of them in a burst).
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let shade = petal_shade(r);
    let bloom = r.f32() < if thrown { 0.4 } else { 0.12 };
    shade + 3 * bloom as u8
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    // The bone is the theme's own ink; the marigolds keep their hue.
    parts: &[
        (b'w', Tint::Ink(HUESO)),
        (b'i', Tint::Cycle(0)),
        (b'j', Tint::Across),
        (b'o', Tint::Own(MARIGOLD[0])),
        (b'y', Tint::Own(MARIGOLD[3])),
        (b'r', Tint::Own(MARIGOLD[4])),
        (b'g', Tint::Own(LEAF)),
    ],
    back: b'w',
    solid: true,
    classic: &PAPEL,
    banner: &["¡Ya se lo llevó", "la Catrina!"],
    react: React::Chatter,
    beat: 0.0,
    fall: Fall {
        kinds: &BITS,
        pick,
        cap: 300,
        // Seconds a fallen petal stays at the start of the holiday, and at
        // its height.
        lie: (14.0, 45.0),
        shed: (0.8, 3.6),
        burst: (0.3, 0.3),
        speed: (3.0, 24.0),
        ttl: ((5.0, 6.0), (10.0, 8.0)),
        air: Air {
            drag: 1.6,
            settle: 1.2,
            gravity: 0.0,
            sway: 2.4,
        },
    },
    joke: None,
};
