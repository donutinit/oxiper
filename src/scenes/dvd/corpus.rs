//! The bouncing logo on the Jueves de Corpus: a mulita de hoja de maíz
//! trotting along with a huacal of flowers and fruit on its back, that
//! scatters petals and fruit at a corner.

use super::{Air, BitKind, Fall, Look, React, Size, Tint};
use crate::color::{Color, rgb};

/// What the paper flowers of the huacal cycle through, on every bounce: rosa
/// mexicano, amarillo, morado, naranja and blanco, the colors of the papel
/// de china of the day.
const PAPEL: [Color; 5] = [
    rgb(0xe4007c), // rosa mexicano
    rgb(0xffd200), // amarillo
    rgb(0x7028a0), // morado
    rgb(0xff8c1a), // naranja
    rgb(0xf4f1ea), // blanco
];
const HOJA: Color = rgb(0xd9c48a);
const TOSTADA: Color = rgb(0xb08d4a);
const MANTA: Color = rgb(0xefe6cf);
const REBOZO: Color = rgb(0x2a5caa);
const MADERA: Color = rgb(0xa0703a);
const VETA: Color = rgb(0x6a4520);
const HOJAS: Color = rgb(0x2f9a3a);
const AMARILLO: Color = rgb(0xffd200);
const OJO: Color = rgb(0x2a1a14);
const MANGO: Color = rgb(0xffb02e);
const NARANJA: Color = rgb(0xff8c1a);
const TUNA: Color = rgb(0xc2185b);
const LIMON: Color = rgb(0x7cb342);

/// The mulita, facing right, one char per pixel: `h` the hoja de maíz and `H`
/// its toasted folds, `n` the eye, `b` the rebozo thrown over its back and `m`
/// its manta stripe and fringe. On it the huacal, `w` its wood and `W` the
/// gaps between its slats, full of `g` leaves, flowers of papel (`p`, `q` and
/// `r` in the day's colors, `y` their hearts) and fruit: `o` naranjas, `M`
/// mangos and `T` tunas. Each size has two frames, its legs a step apart.
const MULITA_A: [&str; 28] = [
    "............ggrrrgg...................",
    "..........ooggrrrggoo.................",
    ".........googrryrrgoogg...............",
    ".........g..grryrrg..gg.....hH..hH....",
    "........gpppggrrrgggqqqg....hH..hH....",
    "........gpppggrrrgggqqqg....hH..hH....",
    ".......gppypp.M..T.qqyqqg...hH..hH....",
    ".......gppyppMMMTTTqqyqqg...hH..hH....",
    ".......ggpppgMMMTTTgqqqgg...hhhhhhh...",
    ".......ggpppg.M..T.gqqqgg...hhhhhhh...",
    "........WwwwwwwwwwwwwwwW...hhhhnhhhh..",
    "........WwwwwwwwwwwwwwwW...hhhh.hhhh..",
    "........WWWWWWWWWWWWWWWW..Hhhhhhhhhhh.",
    "........WWWWWWWWWWWWWWWW..HhhhhhhhhhhH",
    "........WwwwwwwwwwwwwwwW.Hhhhhhhhhhhh.",
    "........WwwwwwwwwwwwwwwW.Hhhhhhhhhhhh.",
    "...h...hhbbbbbbbbbbbbbbHhhhhhhhhhH....",
    "...h...hhbbbbbbbbbbbbbbHhhhhhhhhhH....",
    "..hHhhhHhmmmmmmmmmmmmmmhhhhhhhh.......",
    "..hHhhhHhmmmmmmmmmmmmmmhhhhhhhh.......",
    ".hHhhHhhhbbbbbbbbbbbbbbhhHhh..........",
    ".hHhhHhhhbbbbbbbbbbbbbbhhHhh..........",
    "hHh..hhhHmhmhmhmhmHmhmhHhhh...........",
    "hHh..hhhH.h.h.h.h.H.h.hHhhh...........",
    "......Hh...Hh........Hh..Hh...........",
    "......Hh...Hh........Hh..Hh...........",
    "......HH...HH........HH..HH...........",
    "......HH...HH........HH..HH...........",
];
const MULITA_B: [&str; 28] = [
    "............ggrrrgg...................",
    "..........ooggrrrggoo.................",
    ".........googrryrrgoogg...............",
    ".........g..grryrrg..gg.....hH..hH....",
    "........gpppggrrrgggqqqg....hH..hH....",
    "........gpppggrrrgggqqqg....hH..hH....",
    ".......gppypp.M..T.qqyqqg...hH..hH....",
    ".......gppyppMMMTTTqqyqqg...hH..hH....",
    ".......ggpppgMMMTTTgqqqgg...hhhhhhh...",
    ".......ggpppg.M..T.gqqqgg...hhhhhhh...",
    "........WwwwwwwwwwwwwwwW...hhhhnhhhh..",
    "........WwwwwwwwwwwwwwwW...hhhh.hhhh..",
    "........WWWWWWWWWWWWWWWW..Hhhhhhhhhhh.",
    "........WWWWWWWWWWWWWWWW..HhhhhhhhhhhH",
    "........WwwwwwwwwwwwwwwW.Hhhhhhhhhhhh.",
    "........WwwwwwwwwwwwwwwW.Hhhhhhhhhhhh.",
    "...h...hhbbbbbbbbbbbbbbHhhhhhhhhhH....",
    "...h...hhbbbbbbbbbbbbbbHhhhhhhhhhH....",
    "..hHhhhHhmmmmmmmmmmmmmmhhhhhhhh.......",
    "..hHhhhHhmmmmmmmmmmmmmmhhhhhhhh.......",
    ".hHhhHhhhbbbbbbbbbbbbbbhhHhh..........",
    ".hHhhHhhhbbbbbbbbbbbbbbhhHhh..........",
    "hHh..hhhHmhmhmhmhmHmhmhHhhh...........",
    "hHh..hhhH.h.h.h.h.H.h.hHhhh...........",
    ".......Hh...Hh......Hh..Hh............",
    ".......Hh...Hh......Hh..Hh............",
    ".......HH...HH......HH..HH............",
    ".......HH...HH......HH..HH............",
];
const MULITA_MEDIA_A: [&str; 20] = [
    "........g.r.g.............",
    ".......ggrrrgg......hH.hH.",
    "......p.gryrg.q.....hH.hH.",
    ".....pppgryrgqqq....hH.hH.",
    ".....pypMMoTTqyq...hhhhhh.",
    "....gpypMMoTTqyqg..hhhhhh.",
    ".....WwwwwwwwwwW..Hhhhnhh.",
    ".....WwwwwwwwwwW..Hhhh.hhH",
    ".....WWWWWWWWWWW.Hhhhhhhhh",
    ".....WWWWWWWWWWW.Hhhhhhhhh",
    "..hH.HbbbbbbbbbHHhhhhhhh..",
    "..hH.HbbbbbbbbbHHhhhhhhh..",
    ".hHhhhmmmmmmmmmHhhhhhh....",
    ".hHhhhmmmmmmmmmHhhhhhh....",
    "hHhhhhbbbbbbbbbhHhhh......",
    "hHhhhhbbbbbbbbbhHhhh......",
    "...Hh..Hh.....Hh.Hh.......",
    "...Hh..Hh.....Hh.Hh.......",
    "...HH..HH.....HH.HH.......",
    "...HH..HH.....HH.HH.......",
];
const MULITA_MEDIA_B: [&str; 20] = [
    "........g.r.g.............",
    ".......ggrrrgg......hH.hH.",
    "......p.gryrg.q.....hH.hH.",
    ".....pppgryrgqqq....hH.hH.",
    ".....pypMMoTTqyq...hhhhhh.",
    "....gpypMMoTTqyqg..hhhhhh.",
    ".....WwwwwwwwwwW..Hhhhnhh.",
    ".....WwwwwwwwwwW..Hhhh.hhH",
    ".....WWWWWWWWWWW.Hhhhhhhhh",
    ".....WWWWWWWWWWW.Hhhhhhhhh",
    "..hH.HbbbbbbbbbHHhhhhhhh..",
    "..hH.HbbbbbbbbbHHhhhhhhh..",
    ".hHhhhmmmmmmmmmHhhhhhh....",
    ".hHhhhmmmmmmmmmHhhhhhh....",
    "hHhhhhbbbbbbbbbhHhhh......",
    "hHhhhhbbbbbbbbbhHhhh......",
    "....Hh..Hh...Hh.Hh........",
    "....Hh..Hh...Hh.Hh........",
    "....HH..HH...HH.HH........",
    "....HH..HH...HH.HH........",
];
const MULITA_CHICA_A: [&str; 14] = [
    "....pp..qq........",
    "....pprrqq..hH.hH.",
    "...gppooqqg.hhhhh.",
    "....ppooqq..hhhhh.",
    "...WwwwwwwWhhhhnh.",
    "...WwwwwwwWhhhh.hH",
    "...hbbbbbbbhhhhhh.",
    "..hhbbbbbbbhhhhhh.",
    "HhhhHhhhHhhhhhh...",
    "HhhhHhhhHhhhhhh...",
    "...h..h...h..h....",
    "...h..h...h..h....",
    "...H..H...H..H....",
    "...H..H...H..H....",
];
const MULITA_CHICA_B: [&str; 14] = [
    "....pp..qq........",
    "....pprrqq..hH.hH.",
    "...gppooqqg.hhhhh.",
    "....ppooqq..hhhhh.",
    "...WwwwwwwWhhhhnh.",
    "...WwwwwwwWhhhh.hH",
    "...hbbbbbbbhhhhhh.",
    "..hhbbbbbbbhhhhhh.",
    "HhhhHhhhHhhhhhh...",
    "HhhhHhhhHhhhhhh...",
    "....h..h.h..h.....",
    "....h..h.h..h.....",
    "....H..H.H..H.....",
    "....H..H.H..H.....",
];

const SIZES: [Size; 3] = [
    Size {
        frames: &[&MULITA_A, &MULITA_B],
        react: &[],
        trim: None,
    },
    Size {
        frames: &[&MULITA_MEDIA_A, &MULITA_MEDIA_B],
        react: &[],
        trim: None,
    },
    Size {
        frames: &[&MULITA_CHICA_A, &MULITA_CHICA_B],
        react: &[],
        trim: None,
    },
];

/// What the mulita leaves along its way and shakes off at a corner: petals of
/// papel de china, fruit from the huacal (mangos, naranjas, limones) and
/// hojas de maíz.
const BITS: [BitKind; 7] = [
    BitKind::new(['✿', '❀'], [Tint::Cycle(0), Tint::Cycle(0)], 2.4),
    BitKind::new(['✿', '❀'], [Tint::Cycle(1), Tint::Cycle(1)], 2.4),
    BitKind::new(['✿', '❀'], [Tint::Cycle(2), Tint::Cycle(2)], 2.4),
    BitKind::new(
        ['●', '●'],
        [Tint::Accent(0, MANGO), Tint::Accent(0, MANGO)],
        5.0,
    ),
    BitKind::new(
        ['●', '◉'],
        [Tint::Accent(3, NARANJA), Tint::Accent(3, NARANJA)],
        5.0,
    ),
    BitKind::new(
        ['●', '•'],
        [Tint::Accent(2, LIMON), Tint::Accent(2, LIMON)],
        4.4,
    ),
    BitKind::new(
        ['╱', '╲'],
        [Tint::Accent(4, HOJA), Tint::Accent(4, HOJA)],
        3.0,
    ),
];

/// Petals and fruit and hojas at a corner; on the way mostly petals, with
/// now and then a hoja.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=2 => 0,
            3..=5 => 1,
            6..=8 => 2,
            9..=10 => 3,
            11..=12 => 4,
            13..=14 => 5,
            _ => 6,
        }
    } else {
        match n {
            0..=4 => 0,
            5..=9 => 1,
            10..=13 => 2,
            _ => 6,
        }
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    parts: &[
        (b'h', Tint::Accent(4, HOJA)),
        (b'H', Tint::Dim(TOSTADA)),
        (b'n', Tint::Own(OJO)),
        (b'b', Tint::Accent(5, REBOZO)),
        (b'm', Tint::Ink(MANTA)),
        (b'w', Tint::Accent(1, MADERA)),
        (b'W', Tint::Dim(VETA)),
        (b'g', Tint::Accent(2, HOJAS)),
        (b'p', Tint::Cycle(0)),
        (b'q', Tint::Cycle(1)),
        (b'r', Tint::Cycle(2)),
        (b'y', Tint::Own(AMARILLO)),
        (b'o', Tint::Accent(3, NARANJA)),
        (b'M', Tint::Accent(0, MANGO)),
        (b'T', Tint::Accent(1, TUNA)),
    ],
    back: b'h',
    solid: false,
    classic: &PAPEL,
    banner: &["¡Feliz Jueves", "de Corpus!"],
    react: React::Spin,
    // It trots four steps a second, and three times as fast at a corner.
    beat: 4.0,
    fall: Fall {
        kinds: &BITS,
        pick,
        cap: 240,
        lie: (10.0, 30.0),
        shed: (0.8, 1.6),
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
