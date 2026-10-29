//! The bouncing logo in the Guelaguetza: the penacho of the Danza de la
//! Pluma, swaying as it dances, in a shower of flowers and feathers.

use super::{Air, BitKind, Fall, Look, React, Size, Tint};
use crate::color::{Color, rgb};

/// The colors of Oaxacan embroidery: the feathers of the penacho take them
/// in turn, and a bounce turns the whole fan a feather.
const OAXACA: [Color; 6] = [
    rgb(0x9b111e), // grana cochinilla
    rgb(0xffcc00), // amarillo
    rgb(0x1f8f4a), // verde
    rgb(0xe4007c), // rosa
    rgb(0x6a2c91), // morado
    rgb(0x00aab8), // turquesa
];
const ORO: Color = rgb(0xffcc00);
const BLANCO: Color = rgb(0xf4f1ea);
const ESPEJO: Color = rgb(0xffffff);

/// The penacho, one char per pixel: twelve feathers `a` to `f`, twice around,
/// a gold disc `g` with a mirror `h` in its middle and a ring of beads `j`.
/// Each size has three pictures of it turned a little further, which play
/// back and forth so that it sways.
const PENACHO: [&str; 28] = [
    "............................",
    "..........cc....dd..........",
    ".........cccc..dddd.........",
    ".........cccc..dddd.........",
    ".....bbb.cccc..dddd.eee.....",
    "....bbbb..ccc..ddd..eeee....",
    "....bbbbb..cc..dd..eeeee....",
    "....bbbbb..cc..dd..eeeee....",
    "......bbbb..gggj..eeee......",
    "..aaa...bb.jgggjg.ee...fff..",
    ".aaaaa....gggggggg....fffff.",
    ".aaaaaaa.jggggggggg.fffffff.",
    "..aaaaaagggghhhhgggjffffff..",
    "........gggghhhhgggj........",
    "........jggghhhhgggg........",
    "..ffffffjggghhhhggggaaaaaa..",
    ".fffffff.gggggggggj.aaaaaaa.",
    ".fffff....gggggggg....aaaaa.",
    "..fff...ee.gjgggj.bb...aaa..",
    "......eeee..jggg..bbbb......",
    "....eeeee..dd..cc..bbbbb....",
    "....eeeee..dd..cc..bbbbb....",
    "....eeee..ddd..ccc..bbbb....",
    ".....eee.dddd..cccc.bbb.....",
    ".........dddd..cccc.........",
    ".........dddd..cccc.........",
    "..........dd....cc..........",
    "............................",
];
const PENACHO_2: [&str; 28] = [
    "............................",
    "...........cccc.............",
    "...........cccc..ddd........",
    "......bbb..cccc.ddddd.......",
    ".....bbbb..cccc.ddddd.......",
    ".....bbbbb..cc..dddd..ee....",
    "......bbbb..cc..ddd..eeee...",
    "...aa..bbbb.cc..dd..eeeee...",
    "..aaaa...bb..gg.g...eeeee...",
    "..aaaaa...bjgggjg..eeee.....",
    "..aaaaaa..gggggggggee.......",
    "...aaaaaajggggggggg....ffff.",
    ".........ggghhhhggg.fffffff.",
    ".ffff...gggghhhhgggjfffffff.",
    ".fffffffjggghhhhgggg...ffff.",
    ".fffffff.ggghhhhggg.........",
    ".ffff....gggggggggjaaaaaa...",
    ".......eeggggggggg..aaaaaa..",
    ".....eeee..gjgggjb...aaaaa..",
    "...eeeee...g.gg..bb...aaaa..",
    "...eeeee..dd..cc.bbbb..aa...",
    "...eeee..ddd..cc..bbbb......",
    "....ee..dddd..cc..bbbbb.....",
    ".......ddddd.cccc..bbbb.....",
    ".......ddddd.cccc..bbb......",
    "........ddd..cccc...........",
    ".............cccc...........",
    "............................",
];
const PENACHO_3: [&str; 28] = [
    "............................",
    ".............ccc............",
    ".......bbb..ccccc...d.......",
    ".......bbbb..cccc.dddd......",
    ".......bbbb..ccc..dddd......",
    ".......bbbb..ccc..dddd......",
    "...aaa..bbb..cc..dddd.......",
    "..aaaaa..bbb..c..ddd..eeee..",
    "...aaaaa..bj.gg..dd..eeeee..",
    "...aaaaaa..jgggjgd..eeeeee..",
    "......aaajgggggggg.eeeeee...",
    "..ff.....jgggggggggee.......",
    ".fffff...ggghhhhggg......f..",
    ".fffffffgggghhhhgggj.ffffff.",
    ".ffffff.jggghhhhggggfffffff.",
    "..f......ggghhhhggg...fffff.",
    ".......eegggggggggj.....ff..",
    "...eeeeee.ggggggggjaaa......",
    "..eeeeee..dgjgggj..aaaaaa...",
    "..eeeee..dd..gg.jb..aaaaa...",
    "..eeee..ddd..c..bbb..aaaaa..",
    ".......dddd..cc..bbb..aaa...",
    "......dddd..ccc..bbbb.......",
    "......dddd..ccc..bbbb.......",
    "......dddd.cccc..bbbb.......",
    ".......d...ccccc..bbb.......",
    "............ccc.............",
    "............................",
];
const PENACHITO: [&str; 18] = [
    "..................",
    "......cc..dd......",
    "......cc..dd......",
    "...bb.cc..dd.ee...",
    "...bbb.j..g.eee...",
    "....bb.jgjg.ee....",
    ".aaa..gggggg..fff.",
    ".aaaajgggggggffff.",
    ".....ggghhggg.....",
    ".....ggghhggg.....",
    ".ffffgggggggjaaaa.",
    ".fff..gggggg..aaa.",
    "....ee.gjgj.bb....",
    "...eee.g..j.bbb...",
    "...ee.dd..cc.bb...",
    "......dd..cc......",
    "......dd..cc......",
    "..................",
];
const PENACHITO_2: [&str; 18] = [
    "..................",
    ".......ccc.dd.....",
    "....bb.ccc.dd.....",
    "....bb.cc.ddd.....",
    "....bbb.g.gd.eee..",
    ".aaa..bjgjg..eee..",
    ".aaaa.ggggggge....",
    "...aajggggggg.fff.",
    ".ff..ggghhgggffff.",
    ".ffffggghhggg..ff.",
    ".fff.gggggggjaa...",
    "....eggggggg.aaaa.",
    "..eee..gjgjb..aaa.",
    "..eee.dg.g.bbb....",
    ".....ddd.cc.bb....",
    ".....dd.ccc.bb....",
    ".....dd.ccc.......",
    "..................",
];
const PENACHITO_3: [&str; 18] = [
    "..................",
    ".....b..ccc.......",
    "....bbb.ccc.dd....",
    ".....bb.cc.ddd....",
    "..aa..b..j.dd..e..",
    "..aaa..jgjgd..eee.",
    "...aajgggggg.eee..",
    ".ff..jggggggg.....",
    ".ffffggghhggg.fff.",
    ".fff.ggghhgggffff.",
    ".....gggggggj..ff.",
    "..eee.ggggggjaa...",
    ".eee..dgjgj..aaa..",
    "..e..dd.j..b..aa..",
    "....ddd.cc.bb.....",
    "....dd.ccc.bbb....",
    ".......ccc..b.....",
    "..................",
];
const SIZES: [Size; 2] = [
    Size {
        frames: &[&PENACHO, &PENACHO_2, &PENACHO_3, &PENACHO_2],
        react: &[],
        trim: None,
    },
    Size {
        frames: &[&PENACHITO, &PENACHITO_2, &PENACHITO_3, &PENACHITO_2],
        react: &[],
        trim: None,
    },
];

/// What the dance scatters: flowers, petals, feathers and sparks, in the
/// colors of the huipiles.
const BITS: [BitKind; 6] = [
    BitKind::new(['✿', '❀'], [Tint::Cycle(3), Tint::Cycle(3)], 2.6),
    BitKind::new(['✿', '❀'], [Tint::Cycle(1), Tint::Cycle(1)], 2.6),
    BitKind::new(['◖', '◗'], [Tint::Cycle(0), Tint::Cycle(4)], 2.2),
    BitKind::new(['◖', '◗'], [Tint::Cycle(5), Tint::Cycle(2)], 2.2),
    BitKind::new(['╱', '╲'], [Tint::Cycle(2), Tint::Cycle(2)], 3.0),
    BitKind::new(['✦', '✧'], [Tint::Ink(ORO), Tint::Ink(ORO)], 3.4),
];

/// Everything in turn when thrown; mostly petals and now and then a flower
/// on the way.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=3 => 0,
            4..=6 => 1,
            7..=9 => 2,
            10..=12 => 3,
            13..=16 => 4,
            _ => 5,
        }
    } else {
        match n {
            0..=7 => 2,
            8..=13 => 3,
            14..=16 => 0,
            _ => 1,
        }
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    parts: &[
        (b'a', Tint::Cycle(0)),
        (b'b', Tint::Cycle(1)),
        (b'c', Tint::Cycle(2)),
        (b'd', Tint::Cycle(3)),
        (b'e', Tint::Cycle(4)),
        (b'f', Tint::Cycle(5)),
        (b'g', Tint::Ink(ORO)),
        (b'h', Tint::Dim(ESPEJO)),
        (b'j', Tint::Ink(BLANCO)),
    ],
    back: b'h',
    solid: false,
    classic: &OAXACA,
    banner: &["¡Viva", "Oaxaca!"],
    react: React::Spin,
    // It sways three times a second, and dances three times as fast at a
    // corner.
    beat: 3.0,
    fall: Fall {
        kinds: &BITS,
        pick,
        cap: 300,
        lie: (10.0, 35.0),
        shed: (0.8, 3.2),
        burst: (0.3, 0.3),
        speed: (4.0, 26.0),
        ttl: ((5.0, 5.0), (10.0, 8.0)),
        air: Air {
            drag: 1.4,
            settle: 1.0,
            gravity: 0.0,
            sway: 2.4,
        },
    },
    joke: None,
};
