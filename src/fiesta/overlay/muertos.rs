//! Día de Muertos: papel picado with calaveras and cempasúchil petals.

use super::drift::Flecks;
use super::papel::{Design, Papel};
use super::{Fiesta, Look, Themed};
use crate::color::rgb;

/// The cut-outs: a calavera, cempasúchil, a dove, a cross, a heart, a candle,
/// a lattice.
const DESIGNS: &[Design] = &[
    // calavera
    [
        "XXXXXXX", "XX...XX", "X.X.X.X", "X.....X", "XX.X.XX", "XX...XX", "XXXXXXX", "X.X.X.X",
    ],
    // calavera con quijada
    [
        "XXXXXXX", "XX...XX", "X.....X", "X.X.X.X", "XX...XX", "XX.X.XX", "XXXXXXX", "X.X.X.X",
    ],
    // cempasúchil
    [
        "XXXXXXX", "XX...XX", "X..X..X", "X.XXX.X", "X..X..X", "XX...XX", "XXXXXXX", "X.X.X.X",
    ],
    // paloma
    [
        "XXXXXXX", "XXXXXXX", "X..XXXX", "XX....X", "XXX..XX", "XXXXXXX", "XXXXXXX", "X.X.X.X",
    ],
    // cruz
    [
        "XXXXXXX", "XXX.XXX", "X.....X", "XXX.XXX", "XXX.XXX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
    // corazón
    [
        "XXXXXXX", "XXXXXXX", "XX.X.XX", "X.....X", "XX...XX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
    // veladora
    [
        "XXXXXXX", "XXX.XXX", "XX...XX", "XXX.XXX", "XX...XX", "XX...XX", "XXXXXXX", "X.X.X.X",
    ],
    // celosía
    [
        "XXXXXXX", "X.X.X.X", "XX.X.XX", "X.X.X.X", "XX.X.XX", "X.X.X.X", "XXXXXXX", "X.X.X.X",
    ],
    // rombo
    [
        "XXXXXXX", "XXX.XXX", "XX.X.XX", "X.XXX.X", "XX.X.XX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
];

/// Papel picado colors under `classic`.
const PAPEL: Papel = Papel {
    designs: DESIGNS,
    colors: [
        rgb(0xe4007c), // rosa mexicano
        rgb(0xff8c00), // naranja
        rgb(0x7028a0), // morado
        rgb(0x00aab8), // turquesa
        rgb(0xffd200), // amarillo
        rgb(0x8cc828), // verde limón
    ],
    // The cord, natural string.
    cord: rgb(0xbcae94),
    gap: 2,
    ordered: false,
    banderitas: false,
    themed: Themed::Accents,
};

/// Cempasúchil petals: the face in the light and the face in shade of the
/// marigold ones, and of the golden ones.
const PETALS: Flecks = Flecks {
    tones: &[
        (rgb(0xffaa1e), rgb(0xd66200)),
        (rgb(0xffd23c), rgb(0xe09a00)),
    ],
    share: 50,
    cells: 240,
    themed: Themed::Never,
};

/// What Día de Muertos hangs: the same on each of its days (the second string
/// and the petals come with its level).
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 1,
        papel: Some(&PAPEL),
        flecks: Some(&PETALS),
        ..Look::NONE
    }
}
