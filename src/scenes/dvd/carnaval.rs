//! The bouncing logo in the Carnaval: a carnival mask with plumas and
//! lentejuelas, a burst of confeti and serpentinas at a corner.

use super::{Air, BitKind, Fall, Look, React, Size, Tint, Trim};
use crate::color::{Color, rgb};

/// Confetti colors: the mask takes them in turn, and the plumas the colors
/// after it.
const CONFETI: [Color; 5] = [
    rgb(0xe4007c), // rosa
    rgb(0x00aab8), // turquesa
    rgb(0x7028a0), // morado
    rgb(0x8cc828), // verde limón
    rgb(0xff6a13), // naranja
];
/// Lentejuela dorada, the gold of the sequins.
const LENTEJUELA: Color = rgb(0xffcc33);
const BLANCO: Color = rgb(0xffffff);

/// The mask, one char per pixel: `m` the mask itself, `g` its lentejuelas
/// and `w` a sparkle. Each is followed by the mask winking, a sparkle
/// catching the light, for a corner. The plumas are not in it: they come
/// with the days (see `PLUMAS`).
const MASCARA: [&str; 16] = [
    ".............................",
    "...........mmmmmmm...........",
    ".......mgmmmmmgmmmmgmm.......",
    "....mmmmgmmmmmgmmmmgmmmmm....",
    "..mgm......gmgggg......mgmm..",
    "mmmg.......gmgggg.......gmmmm",
    "mmg........mmmmmm........gmmm",
    "mm........mmmmmmmm........mmm",
    "mmg.....mmmmmmmmmmmm.....gmmm",
    "mmgmmmmmmmmmmmmmmmmmmmmmmgmmm",
    "ggggmmmmmmmmmgggmmmmmmmmmgggg",
    "..ggmmmmmmmmmg.gmmmmmmmmmgg..",
    "....ggggggmmg...gmmgggggg....",
    "......ggggmm.....mmgggg......",
    "..........gg.....gg..........",
    ".............................",
];
const MASCARA_GUINO: [&str; 16] = [
    ".............................",
    "...w.......mmmmmmm...........",
    ".......mmmmmmmgmmmmgmm...w...",
    ".mmmmmmmmmmmmmgmmmmgmmmmm....",
    ".mmmmmggmmmmmgggg......mgmm..",
    "mmmmmmggmmmmmgggg.......gmmmm",
    "mmggggmmgggmmmmmm........gmmm",
    "mmggggmmgggmmmmmmm........mmm",
    "mmmmmmmmmmmgmmmmmmmm.....gmmm",
    "mmmmmmmmmmmgmmmmmmmmmmmmmgmmm",
    "ggggmmmmmmmmmgggmmmmmmmmmgggg",
    "..ggmmmmmmmmmg.gmmmmmmmmmgg..",
    "....ggggggmmg...gmwgggggg....",
    "......ggggmm.....mwgggg......",
    "..........gg.....gg..........",
    ".............................",
];
const MASCARILLA: [&str; 10] = [
    "...................",
    ".....mmmmmmmmm.....",
    "..mm..gmmgmg..mmm..",
    "mm.....mmgm.....mmm",
    "m.....gmmmmg.....mm",
    "mm..gmgmmmmgmg..mmm",
    "ggggmmmmgggmmmmgggg",
    "..ggmmmmg.gmmmmgg..",
    "....gggg...gggg....",
    "...................",
];
const MASCARILLA_GUINO: [&str; 10] = [
    "...................",
    "mmmwmmmmmmmmmm.....",
    "mmmgggmmmgmg..mwm..",
    "mmmgggmmmgm.....mmm",
    "mggmmmggmmmg.....mm",
    "mggmmmggmmmgmg..mmm",
    "ggggmmmmgggmmwmgggg",
    "..ggmmmmg.gmmwmgg..",
    "....gggg...gggg....",
    "...................",
];
const MASCARITA: [&str; 6] = [
    ".....m.....",
    ".m.gmmg.mm.",
    "g...mg...mg",
    "gmmmmgmmmmg",
    ".gggg.gggg.",
    "...........",
];
const MASCARITA_GUINO: [&str; 6] = [
    "mmwmmm.....",
    "mmwmmmg.mm.",
    "ggmgmg...mg",
    "ggmgmgmmmmg",
    "mgggg.gggg.",
    "...........",
];
const SIZES: [Size; 3] = [
    Size {
        frames: &[&MASCARA],
        react: &MASCARA_GUINO,
        trim: Some(&PLUMAS),
    },
    Size {
        frames: &[&MASCARILLA],
        react: &MASCARILLA_GUINO,
        trim: Some(&PLUMITAS),
    },
    Size {
        frames: &[&MASCARITA],
        react: &MASCARITA_GUINO,
        trim: None,
    },
];

/// Plumas fanning out over the mask: three from Thursday, all five from
/// Sunday on (`p`, `q`, `r`, `t` and `u` take the colors after the mask's).
const PLUMAS: Trim = Trim {
    piece: &[".x.", "xxx", "xxx", "xxx", ".x.", ".x."],
    pieces: &[
        (14, 0, b'p'),
        (8, 1, b'q'),
        (20, 1, b'r'),
        (3, 3, b't'),
        (25, 3, b'u'),
    ],
    leaves: &[],
    leaf: b'p',
    above: 6,
};
const PLUMITAS: Trim = Trim {
    piece: &[".x.", "xxx", "xxx", ".x."],
    pieces: &[(9, 0, b'p'), (4, 1, b'q'), (14, 1, b'r')],
    leaves: &[],
    leaf: b'p',
    above: 4,
};

/// What a carnival throws: confeti of every color, lentejuelas and
/// serpentinas.
const BITS: [BitKind; 6] = [
    BitKind::new(['•', '·'], [Tint::Cycle(0), Tint::Cycle(0)], 2.4),
    BitKind::new(['•', '·'], [Tint::Cycle(1), Tint::Cycle(1)], 2.4),
    BitKind::new(['•', '·'], [Tint::Cycle(2), Tint::Cycle(2)], 2.4),
    BitKind::new(['•', '·'], [Tint::Cycle(3), Tint::Cycle(3)], 2.4),
    BitKind::new(
        ['✦', '✧'],
        [Tint::Ink(LENTEJUELA), Tint::Ink(LENTEJUELA)],
        3.0,
    ),
    BitKind::new(['~', '-'], [Tint::Cycle(4), Tint::Cycle(4)], 1.6),
];

/// Confeti of each color in turn, with now and then a lentejuela or a
/// serpentina; a shower of confeti on the way.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=11 => n / 3,
            12..=15 => 4,
            _ => 5,
        }
    } else {
        match n {
            0..=15 => n % 4,
            _ => 5,
        }
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    parts: &[
        (b'm', Tint::Cycle(0)),
        (b'g', Tint::Ink(LENTEJUELA)),
        (b'w', Tint::Ink(BLANCO)),
        (b'p', Tint::Cycle(1)),
        (b'q', Tint::Cycle(2)),
        (b'r', Tint::Cycle(3)),
        (b't', Tint::Cycle(4)),
        (b'u', Tint::Cycle(2)),
    ],
    back: b'm',
    solid: false,
    classic: &CONFETI,
    banner: &["¡Viva el", "Carnaval!"],
    react: React::Hold,
    beat: 0.0,
    fall: Fall {
        kinds: &BITS,
        pick,
        cap: 320,
        lie: (10.0, 40.0),
        shed: (0.5, 3.0),
        burst: (0.3, 0.3),
        speed: (4.0, 26.0),
        ttl: ((5.0, 5.0), (10.0, 8.0)),
        air: Air {
            drag: 1.4,
            settle: 1.0,
            gravity: 0.0,
            sway: 2.6,
        },
    },
    joke: None,
};
