//! The bouncing logo on Candelaria: a tamal in its hoja de maíz, steaming,
//! that opens at a corner.

use super::{Air, BitKind, Fall, Look, React, Size, Tint};
use crate::color::{Color, rgb};

/// What the tie around the tamal cycles through: the Niño Dios' gown, azul,
/// dorado and blanco.
const NINO: [Color; 3] = [
    rgb(0x2a5caa), // azul
    rgb(0xf2b632), // dorado
    rgb(0xf4f1ea), // blanco
];
const HOJA: Color = rgb(0xd9c48a);
const VETA: Color = rgb(0xb59a5c);
const MASA: Color = rgb(0xf2e2b0);
const SALSA: Color = rgb(0xc8102e);
const VAPOR: Color = rgb(0xc8d0d8);
const MAIZ: Color = rgb(0xe8b923);
/// The flame of a vela, which keeps its color under every theme.
const FLAMA: Color = rgb(0xffb347);

/// The tamal, one char per pixel: `h` the hoja de maíz and `H` its veins, `c`
/// the tie at each end, `s` the steam. Each size has two frames of steam and
/// the tamal open: `m` the masa with a stripe of `r` salsa roja.
const TAMAL: [&str; 16] = [
    "........s......s.....s.......",
    ".......s......s.....s........",
    "........s.....s......s.......",
    "..........s.....s.....s......",
    "........hhhhhhhhhhhhh........",
    "h...hhhhhhhhhhhhhhhhhhhhh...h",
    "hh..hHHHHHHHHHHHHHHHHHHHh..hh",
    "hhcchHHHHHHHHHHHHHHHHHHHhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchHHHHHHHHHHHHHHHHHHHhcchh",
    "hh..hHHHHHHHHHHHHHHHHHHHh..hh",
    "h...hhhhhhhhhhhhhhhhhhhhh...h",
    "........hhhhhhhhhhhhh........",
];
const TAMAL_B: [&str; 16] = [
    "..........s.....s......s.....",
    ".........s......s.....s......",
    "........s.....s......s.......",
    ".......s......s.....s........",
    "........hhhhhhhhhhhhh........",
    "h...hhhhhhhhhhhhhhhhhhhhh...h",
    "hh..hHHHHHHHHHHHHHHHHHHHh..hh",
    "hhcchHHHHHHHHHHHHHHHHHHHhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchhhhhhhhhhhhhhhhhhhhhcchh",
    "hhcchHHHHHHHHHHHHHHHHHHHhcchh",
    "hh..hHHHHHHHHHHHHHHHHHHHh..hh",
    "h...hhhhhhhhhhhhhhhhhhhhh...h",
    "........hhhhhhhhhhhhh........",
];
const TAMAL_ABIERTO: [&str; 16] = [
    "........s.....s......s.......",
    "..........s.....s.....s......",
    "..........s.....s......s.....",
    ".........s.....s......s......",
    "........mmmmmmmmmmmmm........",
    "h...hhhhmmmmmmmmmmmmmhhhh...h",
    "hh..hHHmmmmmmmmmmmmmmmHHh..hh",
    "hhcchHHmmmmmmmmmmmmmmmHHhcchh",
    "hhcchhmmrrrrrrrrrrrrrmmhhcchh",
    "hhcchhmmrrrrrrrrrrrrrmmhhcchh",
    "hhcchhmmrrrrrrrrrrrrrmmhhcchh",
    "hhcchhmmrrrrrrrrrrrrrmmhhcchh",
    "hhcchHHmmmmmmmmmmmmmmmHHhcchh",
    "hh..hHHmmmmmmmmmmmmmmmHHh..hh",
    "h...hhhhmmmmmmmmmmmmmhhhh...h",
    "........mmmmmmmmmmmmm........",
];
const TAMALITO: [&str; 12] = [
    "......s.s...s....",
    "......s.s.....s..",
    ".....s..s.....s..",
    "....s.....s..s...",
    "......hhhhh......",
    "hh..hhhhhhhhh..hh",
    "hhcchhhhhhhhhcchh",
    "hhcchhhhhhhhhcchh",
    "hhcchhhhhhhhhcchh",
    "hhcchhhhhhhhhcchh",
    "hh..hHHHHHHHh..hh",
    "......HHHHH......",
];
const TAMALITO_B: [&str; 12] = [
    "....s.....s.s....",
    ".....s....ss.....",
    "......s.s....s...",
    "......s.s.....s..",
    "......hhhhh......",
    "hh..hhhhhhhhh..hh",
    "hhcchhhhhhhhhcchh",
    "hhcchhhhhhhhhcchh",
    "hhcchhhhhhhhhcchh",
    "hhcchhhhhhhhhcchh",
    "hh..hHHHHHHHh..hh",
    "......HHHHH......",
];
const TAMALITO_ABIERTO: [&str; 12] = [
    ".....s..s.....s..",
    "....s.....s..s...",
    "....s.....ss.....",
    "......s..s..s....",
    "......mmmmm......",
    "hh..hhmmmmmhh..hh",
    "hhcchmrrrrrmhcchh",
    "hhcchmrrrrrmhcchh",
    "hhcchmrrrrrmhcchh",
    "hhcchmrrrrrmhcchh",
    "hh..hHmmmmmHh..hh",
    "......mmmmm......",
];
const SIZES: [Size; 2] = [
    Size {
        frames: &[&TAMAL, &TAMAL_B],
        react: &TAMAL_ABIERTO,
        trim: None,
    },
    Size {
        frames: &[&TAMALITO, &TAMALITO_B],
        react: &TAMALITO_ABIERTO,
        trim: None,
    },
];

/// What comes out of a tamal that is opened: hojas, maíz, steam, a spark of
/// the vela the day is for, and salsa.
const BITS: [BitKind; 5] = [
    BitKind::new(
        ['╱', '╲'],
        [Tint::Accent(4, HOJA), Tint::Accent(4, HOJA)],
        3.0,
    ),
    BitKind::new(
        ['●', '•'],
        [Tint::Accent(2, MAIZ), Tint::Accent(2, MAIZ)],
        5.0,
    ),
    BitKind::new(['°', '○'], [Tint::Dim(VAPOR), Tint::Dim(VAPOR)], 1.2),
    BitKind::new(['✦', '✧'], [Tint::Own(FLAMA), Tint::Own(FLAMA)], 2.4),
    BitKind::new(
        ['•', '·'],
        [Tint::Accent(1, SALSA), Tint::Accent(1, SALSA)],
        4.0,
    ),
];

/// Hojas, maíz, steam, sparks and salsa when thrown; mostly steam when shed.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=5 => 0,
            6..=10 => 1,
            11..=13 => 2,
            14..=16 => 3,
            _ => 4,
        }
    } else {
        match n {
            0..=11 => 2,
            12..=15 => 1,
            _ => 0,
        }
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    parts: &[
        (b'h', Tint::Accent(4, HOJA)),
        (b'H', Tint::Dim(VETA)),
        (b'm', Tint::Ink(MASA)),
        (b'r', Tint::Accent(1, SALSA)),
        (b'c', Tint::Cycle(0)),
        (b's', Tint::Dim(VAPOR)),
    ],
    back: b'h',
    solid: false,
    classic: &NINO,
    banner: &["¡Te tocan", "los tamales!"],
    react: React::Hold,
    // The steam curls up twice a second or so.
    beat: 2.5,
    fall: Fall {
        kinds: &BITS,
        pick,
        cap: 220,
        lie: (10.0, 25.0),
        shed: (0.5, 1.0),
        burst: (0.3, 0.3),
        speed: (4.0, 24.0),
        ttl: ((4.0, 4.0), (8.0, 6.0)),
        air: Air {
            drag: 1.2,
            settle: 1.0,
            gravity: 0.0,
            sway: 1.8,
        },
    },
    joke: None,
};
