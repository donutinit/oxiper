//! The bouncing logo on the Virgen de Guadalupe, December 12: a ramo of rosas
//! de Castilla, the roses that Juan Diego carried in his tilma, wrapped in
//! manta and tied with a ribbon the color of her mantle. It is the bouquet
//! that bounces, never her image.

use super::{Air, BitKind, Fall, Look, React, Size, Tint};
use crate::color::{Color, rgb};

/// What the roses cycle through on every bounce, two at a time (the petals
/// and the swirl at the heart): the pinks of the rosas de Castilla and the
/// rose of her gown.
const ROSAS: [Color; 4] = [
    rgb(0xd81b60), // rosa de Castilla
    rgb(0xf06292), // its light
    rgb(0xe57373), // the rose of her gown
    rgb(0xad1457), // and the darkest
];
const HOJAS: Color = rgb(0x2e7d32);
const MANTA: Color = rgb(0xefe6cf);
const SOMBRA: Color = rgb(0xc9bfa6);
const MANTO: Color = rgb(0x00796b);
/// The gold of the stars of her mantle, which keeps its color under every
/// theme.
const ORO: Color = rgb(0xf2c14e);

/// The ramo, one char per pixel: `R` and `r` the petals and the heart of a
/// rose, `Q` and `q` the other roses with the two colors the other way round,
/// `G` leaves and `S` the stems, `w` the manta it is wrapped in and `x` its
/// shade, `T` the ribbon in the verde azulado of her mantle with `Y` stars of
/// gold on it. After each size, the same ramo with the stars of the mantle
/// glinting among its roses.
const RAMO: [&str; 28] = [
    "............QQQQQ.............",
    "............QQQQQ.G...........",
    "...RRRRRGGGQQqqQQQGGGGRRRRR...",
    "...RRRRRGGGQQqqQQQGGGGRRRRR...",
    "..RRrrRRRGGQQQqqQQGGGRRrrRRR..",
    "..RRrrRRRGGQQQqqQQGGGRRrrRRR..",
    ".GRRRrrRRGGGQQQQQGGGGRRRrrRRG.",
    ".GRRRrrRRGGGQQQQQGGGGRRRrrRRG.",
    "GGGRRRRQQQQQGGGGGRRRRRRRRRRGGG",
    ".GGRRRRQQQQQGGGGGRRRRRRRRRRGG.",
    "..GGGGQQqqQQQGGGRRrrRRRGGGGG..",
    "..GGGGQQqqQQQGGGRRrrRRRGGGGG..",
    "....GGQQQqqQQGGGRRRrrRRGGG....",
    "....GGQQQqqQQGGGRRRrrRRGGG....",
    ".......QQQQQwwwwwRRRRR........",
    ".......QQQQQwwwwwRRRRR........",
    ".........wTYTTTTTTTTx.........",
    ".........wTYTTTTTTTTx.........",
    "..........wwTTTTTwTx..........",
    "..........wwTTTTTwTx..........",
    "...........wwTwTwwx...........",
    "...........wwTwTwwx...........",
    "............wTSTwx............",
    "............wTSTwx............",
    "..............SS..............",
    "..............SS..............",
    "..............SS..............",
    "..............SS..............",
];
const RAMO_ESTRELLAS: [&str; 28] = [
    "............QQQQQ.............",
    "............QQQQQ.G...........",
    "...RRRRRGGGQQqYQQQGGGGRRRRR...",
    "...RRRRRGGGQQqYQQQGGGGRRRRR...",
    "..RRYrRRRGYQQQqqQQGGGRRrYRRR..",
    "..RRYrRRRGYQQQqqQQGGGRRrYRRR..",
    ".GRRRrrRRGGGQQQQQGYGGRRRrrRRG.",
    ".GRRRrrRRGGGQQQQQGYGGRRRrrRRG.",
    "GYGRRRRQQQQQGGGGGRRRRRRRRRRGYG",
    ".YGRRRRQQQQQGGGGGRRRRRRRRRRGY.",
    "..GGGGQQYqQQQGGGRRrYRRRGGGGG..",
    "..GGGGQQYqQQQGGGRRrYRRRGGGGG..",
    "....GGQQQqqQQGYGRRRrrRRGGG....",
    "....GGQQQqqQQGYGRRRrrRRGGG....",
    ".......QQQQQwwwwwRRRRR........",
    ".......QQQQQwwwwwRRRRR........",
    ".........wTYTTTTTTTTx.........",
    ".........wTYTTTTTTTTx.........",
    "..........wwTTTTTwTx..........",
    "..........wwTTTTTwTx..........",
    "...........wwTwTwwx...........",
    "...........wwTwTwwx...........",
    "............wTSTwx............",
    "............wTSTwx............",
    "..............SS..............",
    "..............SS..............",
    "..............SS..............",
    "..............SS..............",
];
const RAMILLETE: [&str; 20] = [
    "........QQQ.........",
    "........QQQG........",
    "..RRRGGQqqQQGGGRRR..",
    "..RRRGGQqqQQGGGRRR..",
    ".RrrRRGGQQQGGGRrrRR.",
    ".RrrRRGGQQQGGGRrrRR.",
    ".GRRQQQGGGGGRRRRRRG.",
    ".GRRQQQGGGGGRRRRRRG.",
    ".GGQqqQQGGGRrrRRGGG.",
    ".GGQqqQQGGGRrrRRGGG.",
    "....QQQwwwwwRRRx....",
    "....QQQwwwwwRRRx....",
    ".....wTYTTTTYTx.....",
    ".....wTYTTTTYTx.....",
    "......wwwTTwwx......",
    "......wwwTTwwx......",
    ".......wwSSwx.......",
    ".......wwSSwx.......",
    ".........SS.........",
    ".........SS.........",
];
const RAMILLETE_ESTRELLAS: [&str; 20] = [
    "........QQQ.........",
    "........QQQG........",
    "..RRRGGQqYQQGGGRRR..",
    "..RRRGGQqYQQGGGRRR..",
    ".RrrYRGGQQQGGGRrYRR.",
    ".RrrYRGGQQQGGGRrYRR.",
    "YGRRQQQGGGGGRRRRRRGY",
    "YGRRQQQGGGGGRRRRRRGY",
    ".GGQqqYQGGGRrYRRGGG.",
    ".GGQqqYQGGGRrYRRGGG.",
    "....QQQwwwwwRRRx....",
    "....QQQwwwwwRRRx....",
    ".....wTYTTTTYTx.....",
    ".....wTYTTTTYTx.....",
    "......wwwTTwwx......",
    "......wwwTTwwx......",
    ".......wwSSwx.......",
    ".......wwSSwx.......",
    ".........SS.........",
    ".........SS.........",
];
const RAMITO: [&str; 14] = [
    ".R.GG.Q.GG.R..",
    "RRRGGQQQGGRRR.",
    "RrRGGQqQGGRrR.",
    ".r.GG.q.GG.r..",
    ".G.Q.GGG.R.GG.",
    ".GQQQGGGRRRGG.",
    "..QqQGGGRrR...",
    "...q.GGG.r....",
    "....wTYTwx....",
    "....wTYTwx....",
    ".....wTTx.....",
    ".....wTTx.....",
    "......SS......",
    "......SS......",
];
const RAMITO_ESTRELLAS: [&str; 14] = [
    ".R.GG.Y.GG.R..",
    "RRRGGQYQGGRRR.",
    "RYRGGQqQGGRYR.",
    ".Y.GG.q.GG.Y..",
    ".G.Q.GGG.R.GG.",
    ".GQQQGGGRRRGG.",
    "..QYQGGGRYR...",
    "...Y.GGG.Y....",
    "....wTYTwx....",
    "....wTYTwx....",
    ".....wTTx.....",
    ".....wTTx.....",
    "......SS......",
    "......SS......",
];

const RAMOS: [Size; 3] = [
    Size {
        frames: &[&RAMO],
        react: &RAMO_ESTRELLAS,
        trim: None,
    },
    Size {
        frames: &[&RAMILLETE],
        react: &RAMILLETE_ESTRELLAS,
        trim: None,
    },
    Size {
        frames: &[&RAMITO],
        react: &RAMITO_ESTRELLAS,
        trim: None,
    },
];

/// What falls from the ramo: petals of rosa, rosas, petals in the other
/// pink, stars of gold and leaves.
const BITS: [BitKind; 5] = [
    BitKind::new(['◖', '◗'], [Tint::Cycle(0), Tint::Cycle(1)], 2.4),
    BitKind::new(['◗', '◖'], [Tint::Cycle(1), Tint::Cycle(0)], 2.4),
    BitKind::new(['✿', '❀'], [Tint::Cycle(0), Tint::Cycle(1)], 2.8),
    BitKind::new(['✦', '✧'], [Tint::Own(ORO), Tint::Own(ORO)], 3.4),
    BitKind::new(
        ['╱', '╲'],
        [Tint::Accent(2, HOJAS), Tint::Accent(2, HOJAS)],
        3.0,
    ),
];

/// Mostly petals, with now and then a rosa, a star or a leaf.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=5 => 0,
            6..=9 => 1,
            10..=12 => 2,
            13..=16 => 3,
            _ => 4,
        }
    } else {
        match n {
            0..=8 => 0,
            9..=14 => 1,
            15..=16 => 2,
            17..=18 => 3,
            _ => 4,
        }
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &RAMOS,
    parts: &[
        (b'R', Tint::Cycle(0)),
        (b'r', Tint::Cycle(1)),
        (b'Q', Tint::Cycle(1)),
        (b'q', Tint::Cycle(0)),
        (b'G', Tint::Accent(2, HOJAS)),
        (b'S', Tint::Accent(2, HOJAS)),
        (b'w', Tint::Ink(MANTA)),
        (b'x', Tint::Dim(SOMBRA)),
        (b'T', Tint::Accent(5, MANTO)),
        (b'Y', Tint::Own(ORO)),
    ],
    back: b'G',
    solid: false,
    classic: &ROSAS,
    banner: &["¡Felicidades", "a todas", "las Lupitas!"],
    // At a corner the stars of her mantle glint among the roses.
    react: React::Hold,
    beat: 0.0,
    fall: Fall {
        kinds: &BITS,
        pick,
        cap: 220,
        lie: (10.0, 30.0),
        shed: (0.9, 1.8),
        burst: (0.3, 0.3),
        speed: (4.0, 22.0),
        ttl: ((5.0, 4.0), (9.0, 6.0)),
        air: Air {
            drag: 1.4,
            settle: 1.0,
            gravity: 0.0,
            sway: 2.2,
        },
    },
    joke: None,
};
