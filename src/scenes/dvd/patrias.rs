//! The bouncing logo in the Fiestas Patrias: a sombrero de charro with a
//! tricolor band, throwing papel picado and sparks at a corner.

use super::{Air, BitKind, Fall, Look, React, Size, Tint, Trim};
use crate::color::{Color, rgb};

/// The flag, which the band of the sombrero takes in turn: a bounce turns
/// the band around the crown by a stripe.
const BANDERA: [Color; 3] = [
    rgb(0x006847), // verde bandera
    rgb(0xf4f1ea), // blanco
    rgb(0xce1126), // rojo
];
const CAFE: Color = rgb(0x7a4a1e);
const SOMBRA: Color = rgb(0x4a2c10);
const ORO: Color = rgb(0xd4a017);

/// The sombrero de charro, one char per pixel: `f` its felt, `d` the shadow
/// under its brim, `g` the gold embroidery, and the band of the flag `v`, `w`
/// and `r`. Each is followed by the sombrero tipped, its brim curling up in a
/// smile, for a corner.
const SOMBRERO: [&str; 16] = [
    "..........fffffffff..........",
    ".........fffffffffff.........",
    ".........fffffffffff.........",
    "........fffffffffffff........",
    "........ffgfffgfffgff........",
    "........ffgfffgfffgff........",
    "g.......fffffffffffff.......g",
    "gf......fffffffffffff......fg",
    "ffgf....fffffffffffff....fgff",
    "ffgfg..fffffffffffffff..gfgff",
    ".dfffffvvvvvwwwwwrrrrrfffffd.",
    "..fffffvvvvvwwwwwrrrrrfffff..",
    "....dfffffffffffffffffffd....",
    ".....fffffffffffffffffff.....",
    "........ddddddddddddd........",
    "..........ddddddddd..........",
];
const SOMBRERO_ALEGRE: [&str; 16] = [
    "..........fffffffff..........",
    ".........fffffffffff.........",
    ".........fffffffffff.........",
    "g.......fffffffffffff.......g",
    "ff......ffgfffgfffgff......ff",
    "ffg.....ffgfffgfffgff.....gff",
    "dfff....fffffffffffff....fffd",
    ".fffg...fffffffffffff...gfff.",
    "..dfff..fffffffffffff..fffd..",
    "...fffffffffffffffffffffff...",
    "....dffvvvvvwwwwwrrrrrffd....",
    ".....ffvvvvvwwwwwrrrrrff.....",
    "......dfdfffffffffffdfd......",
    ".......fdfffffffffffdf.......",
    ".........ddddddddddd.........",
    "...........ddddddd...........",
];
const SOMBRERITO: [&str; 12] = [
    "........fffff........",
    "......fffffffff......",
    "......fgffgffgf......",
    "......fgffgffgf......",
    "g.....fffffffff.....g",
    "gfg..fffffffffff..gfg",
    "dfff.vvvvwwwwrrr.fffd",
    ".ffffvvvvwwwwrrrffff.",
    "...dfffffffffffffd...",
    "....fffffffffffff....",
    ".......ddddddd.......",
    ".....................",
];
const SOMBRERITO_ALEGRE: [&str; 12] = [
    "........fffff........",
    "g.....fffffffff.....g",
    "ff....fgffgffgf....ff",
    "ff....fgffgffgf....ff",
    ".dg...fffffffff...gd.",
    "..gfffffffffffffffg..",
    "..dffvvvvwwwwrrrffd..",
    "...ffvvvvwwwwrrrff...",
    ".....dfffffffffd.....",
    "......fffffffff......",
    "........ddddd........",
    ".....................",
];
const SOMBRERILLO: [&str; 10] = [
    ".....f.f.....",
    "....fffff....",
    "....fgggf....",
    "...ffgggff...",
    "g..vvvwwrr..g",
    "gffvvvwwrrffg",
    "dfffffffffffd",
    ".fffffffffff.",
    "...ddddddd...",
    ".............",
];
const SOMBRERILLO_ALEGRE: [&str; 10] = [
    ".....f.f.....",
    "g...fffff...g",
    "f...fgggf...f",
    "ff.ffgggff.ff",
    ".ffvvvwwrrff.",
    ".ffvvvwwrrff.",
    "..dfffffffd..",
    "...fffffff...",
    ".....ddd.....",
    ".............",
];
const SIZES: [Size; 3] = [
    Size {
        frames: &[&SOMBRERO],
        react: &SOMBRERO_ALEGRE,
        trim: Some(&BANDERITAS),
    },
    Size {
        frames: &[&SOMBRERITO],
        react: &SOMBRERITO_ALEGRE,
        trim: Some(&BANDERITAS_CHICAS),
    },
    Size {
        frames: &[&SOMBRERILLO],
        react: &SOMBRERILLO_ALEGRE,
        trim: None,
    },
];

/// Banderitas planted on top of the crown, one at the start of the mes
/// patrio and all of them from the second week: a pole (`g`) with a flag of
/// the band's colors, the two at the sides a little lower.
const BANDERITAS: Trim = Trim {
    piece: &["gvwr", "gvwr", "g...", "g..."],
    pieces: &[(16, 0, b'x'), (12, 2, b'x'), (20, 2, b'x')],
    leaves: &[],
    leaf: b'g',
    above: 4,
};
const BANDERITAS_CHICAS: Trim = Trim {
    piece: &["gvwr", "gvwr", "g...", "g..."],
    pieces: &[(10, 0, b'x'), (14, 0, b'x')],
    leaves: &[],
    leaf: b'g',
    above: 4,
};

/// What flies from the sombrero: papelitos in the colors of the flag, and
/// sparks of gold.
const PAPELITOS: [BitKind; 4] = [
    BitKind::new(['•', '·'], [Tint::Cycle(0), Tint::Cycle(0)], 2.6),
    BitKind::new(['•', '·'], [Tint::Cycle(1), Tint::Cycle(1)], 2.6),
    BitKind::new(['•', '·'], [Tint::Cycle(2), Tint::Cycle(2)], 2.6),
    BitKind::new(['✦', '✧'], [Tint::Ink(ORO), Tint::Ink(ORO)], 3.4),
];

/// Papelitos of each color, and now and then a spark of gold when thrown.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    match n {
        0..=5 => 0,
        6..=11 => 1,
        12..=17 => 2,
        _ if thrown => 3,
        _ => n % 3,
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    parts: &[
        (b'f', Tint::Accent(5, CAFE)),
        (b'd', Tint::Dim(SOMBRA)),
        (b'g', Tint::Ink(ORO)),
        (b'v', Tint::Cycle(0)),
        (b'w', Tint::Cycle(1)),
        (b'r', Tint::Cycle(2)),
    ],
    back: b'f',
    solid: false,
    classic: &BANDERA,
    banner: &["¡VIVA", "MÉXICO!"],
    react: React::Hold,
    beat: 0.0,
    fall: Fall {
        kinds: &PAPELITOS,
        pick,
        cap: 300,
        lie: (12.0, 40.0),
        shed: (0.9, 3.6),
        burst: (0.3, 0.3),
        speed: (4.0, 26.0),
        ttl: ((5.0, 5.0), (10.0, 8.0)),
        air: Air {
            drag: 1.4,
            settle: 1.0,
            gravity: 0.0,
            sway: 2.2,
        },
    },
    joke: None,
};
