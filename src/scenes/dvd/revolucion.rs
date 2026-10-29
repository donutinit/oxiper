//! The bouncing logo on the Día de la Revolución: a sombrero over two
//! crossed cananas, shouting "¡Tierra y Libertad!" at a corner.

use super::{Air, BitKind, Fall, Look, React, Size, Tint};
use crate::color::{Color, rgb};

/// The colors of the band of the sombrero under `classic`: with the manta
/// of the hat itself, the tricolor.
const BANDA: [Color; 2] = [
    rgb(0xce1126), // rojo
    rgb(0x006847), // verde
];
const MANTA: Color = rgb(0xefe6cf);
const CAQUI: Color = rgb(0x8b7d5b);
const LATON: Color = rgb(0xb8913a);
const BRILLO: Color = rgb(0xffd77a);

/// The sombrero and the cananas, one char per pixel: `f` the manta of the
/// hat, `d` the shadow of its brim, `v` its band, `s` the straps of the
/// cananas and `b` the brass of the cartuchos in them. Each is followed by
/// the same raised, the brim curling up and the brass gleaming (`B`), for a
/// corner.
const REVOLUCION: [&str; 30] = [
    ".............fff.............",
    "............fffff............",
    "...........fffffff...........",
    "...........fffffff...........",
    "..........fffffffff..........",
    "..........fffffffff..........",
    ".........fffffffffff.........",
    ".........fffffffffff.........",
    "ff......vvvvvvvvvvvvv......ff",
    "fffffff.vvvvvvvvvvvvv.fffffff",
    "ddfffffffffffffffffffffffffdd",
    "..fffffffffffffffffffffffff..",
    ".......ddddddddddddddd.......",
    "....s...................s....",
    "....sbs...............sbs....",
    "....sbsss...........sssbs....",
    "....sbssbs.........sbssbs....",
    ".....bssbsss.....sssbssb.....",
    ".......sbssbss.ssbssbs.......",
    ".........ssbsssssbss.........",
    "..........sbssbssbs..........",
    "..........sbssbssbs..........",
    ".........ssbssbssbss.........",
    ".......ssssbss.ssbssss.......",
    ".....bssbssb.....bssbssb.....",
    "....sbssbs.........sbssbs....",
    "....sbssb...........bssbs....",
    "....sbs...............sbs....",
    "....s...................s....",
    ".............................",
];
const REVOLUCION_VIVA: [&str; 30] = [
    ".............fff.............",
    "............fffff............",
    "...........fffffff...........",
    "...........fffffff...........",
    "..........fffffffff..........",
    "f.........fffffffff.........f",
    "fff......fffffffffff......fff",
    "fffff....fffffffffff....fffff",
    ".ddffff.vvvvvvvvvvvvv.ffffdd.",
    "...fffffvvvvvvvvvvvvvfffff...",
    ".....ddfffffffffffffffdd.....",
    ".......fffffffffffffff.......",
    "..........ddddddddd..........",
    "....s...................s....",
    "....sBs...............sBs....",
    "....sBsss...........sssBs....",
    "....sBssbs.........sbssBs....",
    ".....Bssbsss.....sssbssB.....",
    ".......sbssBss.ssBssbs.......",
    ".........ssBsssssBss.........",
    "..........sBssbssBs..........",
    "..........sBssbssBs..........",
    ".........ssBssbssBss.........",
    ".......ssssBss.ssBssss.......",
    ".....BssbssB.....BssbssB.....",
    "....sBssbs.........sbssBs....",
    "....sBssb...........bssBs....",
    "....sBs...............sBs....",
    "....s...................s....",
    ".............................",
];
const REVOLUCIONCITA: [&str; 20] = [
    ".........f.........",
    ".......fffff.......",
    ".......fffff.......",
    "......fffffff......",
    "ff....vvvvvvv....ff",
    "fffffvvvvvvvvvfffff",
    "ddfffffffffffffffdd",
    "..fffffffffffffff..",
    ".....ddddddddd.....",
    "...s...........s...",
    "...sbs.......bss...",
    "...sbsss...ssbss...",
    "....sssbssbssss....",
    "......sbssbss......",
    "......sbssbss......",
    "....sssbssbssss....",
    "...sbsss...ssbss...",
    "...sbs.......bss...",
    "...s...........s...",
    "...................",
];
const REVOLUCIONCITA_VIVA: [&str; 20] = [
    ".........f.........",
    "f......fffff......f",
    "ff.....fffff.....ff",
    "fff...fffffff...fff",
    ".dfff.vvvvvvv.fffd.",
    "..fffvvvvvvvvvfff..",
    "...ddfffffffffdd...",
    ".....fffffffff.....",
    ".......ddddd.......",
    "...s...........s...",
    "...sBs.......bss...",
    "...sBsss...ssbss...",
    "....sssbssBssss....",
    "......sbssBss......",
    "......sbssBss......",
    "....sssbssBssss....",
    "...sBsss...ssbss...",
    "...sBs.......bss...",
    "...s...........s...",
    "...................",
];
const REVOLUCIONCILLA: [&str; 14] = [
    ".....f.....",
    "....fff....",
    "...vvvvv...",
    "ff.vvvvv.ff",
    "fffffffffff",
    "fffffffffff",
    "..ddddddd..",
    "..d.....d..",
    "..sss.sss..",
    "...sssss...",
    "...sssss...",
    "..sss.sss..",
    "..s.....s..",
    "...........",
];
const REVOLUCIONCILLA_VIVA: [&str; 14] = [
    "f....f....f",
    "ff..fff..ff",
    "df.vvvvv.fd",
    ".ffvvvvvff.",
    "..fffffff..",
    "..fffffff..",
    "....ddd....",
    "..s.....s..",
    "..sss.sss..",
    "...sssss...",
    "...sssss...",
    "..sss.sss..",
    "..s.....s..",
    "...........",
];
const SIZES: [Size; 3] = [
    Size {
        frames: &[&REVOLUCION],
        react: &REVOLUCION_VIVA,
        trim: None,
    },
    Size {
        frames: &[&REVOLUCIONCITA],
        react: &REVOLUCIONCITA_VIVA,
        trim: None,
    },
    Size {
        frames: &[&REVOLUCIONCILLA],
        react: &REVOLUCIONCILLA_VIVA,
        trim: None,
    },
];

/// What flies from the logo: confeti in the colors of the band, a spark of
/// brass and the dust of the road.
const BITS: [BitKind; 4] = [
    BitKind::new(['•', '·'], [Tint::Cycle(0), Tint::Cycle(0)], 2.6),
    BitKind::new(['•', '·'], [Tint::Cycle(1), Tint::Cycle(1)], 2.6),
    BitKind::new(
        ['✦', '✧'],
        [Tint::Accent(2, BRILLO), Tint::Accent(2, BRILLO)],
        3.2,
    ),
    BitKind::new(['·', '∙'], [Tint::Dim(CAQUI), Tint::Dim(CAQUI)], 1.8),
];

/// Confetti, brass and dust when thrown; mostly dust when shed, as the
/// logo leaves it behind.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=5 => 0,
            6..=11 => 1,
            12..=15 => 2,
            _ => 3,
        }
    } else {
        match n {
            0..=13 => 3,
            _ => 2,
        }
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    parts: &[
        (b'f', Tint::Ink(MANTA)),
        (b'd', Tint::Dim(CAQUI)),
        (b'v', Tint::Cycle(0)),
        (b's', Tint::Dim(CAQUI)),
        (b'b', Tint::Accent(2, LATON)),
        (b'B', Tint::Accent(2, BRILLO)),
    ],
    back: b'f',
    solid: false,
    classic: &BANDA,
    banner: &["¡Tierra y", "Libertad!"],
    react: React::Hold,
    beat: 0.0,
    fall: Fall {
        kinds: &BITS,
        pick,
        cap: 260,
        lie: (10.0, 25.0),
        shed: (0.6, 1.2),
        burst: (0.3, 0.3),
        speed: (4.0, 26.0),
        ttl: ((5.0, 5.0), (8.0, 6.0)),
        air: Air {
            drag: 1.3,
            settle: 1.0,
            gravity: 0.0,
            sway: 1.8,
        },
    },
    joke: None,
};
