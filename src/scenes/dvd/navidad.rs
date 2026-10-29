//! The bouncing logo in the Navidad: a piñata de siete picos that bursts into
//! candy and fruit at a corner, and from January 2 a rosca de reyes.

use super::{Air, BitKind, Fall, Look, React, Size, Tint, Trim};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;

/// Papel de china, the colors of a piñata, ordered so that neighbors differ
/// (the seven picos take them in turn).
const PAPEL: [Color; 6] = [
    rgb(0xe4007c), // rosa mexicano
    rgb(0xffd200), // amarillo
    rgb(0x00aab8), // turquesa
    rgb(0xff8c00), // naranja
    rgb(0x7028a0), // morado
    rgb(0x8cc828), // verde limón
];
/// What the jewels of a rosca cycle through: ate and acitrón.
const JOYAS: [Color; 5] = [
    rgb(0xc8102e), // ate rojo
    rgb(0x3e9a4a), // acitrón verde
    rgb(0xf2b632), // dorado
    rgb(0xe8891c), // naranja
    rgb(0xe4007c), // rosa
];
const DORADO: Color = rgb(0xf2b632);
const BLANCO: Color = rgb(0xf4f1ea);
const CANA: Color = rgb(0xc9b26b);
const TEJOCOTE: Color = rgb(0xe8891c);
const MANDARINA: Color = rgb(0xff8c00);
const MORADO: Color = rgb(0x7028a0);

/// The piñata, one char per pixel: the seven picos `a` to `g` clockwise
/// from the top, the ball in the middle `h`, and the rope `r` it hangs
/// from. After it, what is left of it once it has burst: the picos flung
/// apart and the middle gone.
const PINATA: [&str; 28] = [
    "...........r...........",
    "...........r...........",
    "...........r...........",
    "...........r...........",
    "...........r...........",
    "...........a...........",
    "...........a...........",
    "...........a...........",
    "..........aaa..........",
    "...g......aaa......b...",
    "....ggg..aaaaa..bbb....",
    "....gggggaaaaabbbbb....",
    ".....ggghhhhhhhbbb.....",
    "......gghhhhhhhbb......",
    "......ghhhhhhhhhb......",
    ".....fghhhhhhhhhbc.....",
    "....fffhhhhhhhhhccc....",
    "..fffffhhhhhhhhhccccc..",
    ".fffffffhhhhhhhccccccc.",
    ".......fhhhhhhhc.......",
    ".......eeeeedddd.......",
    ".......eeee.dddd.......",
    ".......eee...ddd.......",
    ".......ee.....dd.......",
    ".......e.......d.......",
    ".......................",
    ".......................",
    ".......................",
];
const PINATA_ROTA: [&str; 28] = [
    "...........r...........",
    "...........r...........",
    "...........r...........",
    "...........r...........",
    "...........a...........",
    "..........aaa..........",
    "..........aaa..........",
    "..........aaa..........",
    ".ggg.....aaaaa.....bbb.",
    "..ggggg..aaaaa..bbbbb..",
    "...gggggggaaabbbbbbb...",
    "...gggggggaaabbbbbbb...",
    "....ggggg.....bbbbb....",
    "....gggg.......bbbb....",
    ".....gg.........cc.....",
    "....fgg.........ccc....",
    "..fffff.........ccccc..",
    "fffffff.........ccccccc",
    "ffffffff.......cccccccc",
    ".fffffffe.....dccccccc.",
    "......eeeee.ddddd......",
    "......eeeeeeddddd......",
    "......eeeee.ddddd......",
    "......eeeee.ddddd......",
    "......eeee...dddd......",
    "......eee.....ddd......",
    "......ee.......dd......",
    "......e.........d......",
];
const PINATILLA: [&str; 22] = [
    "........r........",
    "........r........",
    "........r........",
    "........r........",
    "........r........",
    "........a........",
    "........a........",
    ".......aaa.......",
    "..gg...aaa...bb..",
    "...ggggaaabbbb...",
    "....gghhhhhbb....",
    "....gghhhhhbb....",
    "....fhhhhhhhc....",
    "..fffhhhhhhhccc..",
    ".fffffhhhhhdcccc.",
    ".....fhhhhhd.....",
    ".....eeeeddd.....",
    ".....eee.ddd.....",
    ".....ee...dd.....",
    ".....e.....d.....",
    ".................",
    ".................",
];
const PINATILLA_ROTA: [&str; 22] = [
    "........r........",
    "........r........",
    "........r........",
    "........r........",
    "........a........",
    ".......aaa.......",
    ".......aaa.......",
    ".gg....aaa....bb.",
    "..ggggaaaaabbbb..",
    "..ggggaaaaabbbb..",
    "...ggg.....bbb...",
    "...ggg.....bbb...",
    "...fg.......cc...",
    ".fffg.......cccc.",
    "fffff.......ccccc",
    "ffffff.....dccccc",
    ".....eee.ddd.....",
    "....eeeeedddd....",
    "....eeee.dddd....",
    "....eee...ddd....",
    "....ee.....dd....",
    "....e.......d....",
];
const PINATITA: [&str; 14] = [
    ".....r.....",
    ".....r.....",
    ".....r.....",
    ".....a.....",
    ".....a.....",
    "..ggaaabb..",
    "..ghhhhhb..",
    "...hhhhh...",
    ".ffhhhhhcc.",
    "..fhhhhhc..",
    "...eeedd...",
    "...ee.dd...",
    "...e...d...",
    "...........",
];
const PINATITA_ROTA: [&str; 14] = [
    ".....r.....",
    ".....r.....",
    ".....a.....",
    ".....a.....",
    ".g..aaa..b.",
    ".gggaaabbb.",
    "..gg...bb..",
    "..g.....b..",
    "fff.....ccc",
    "ffff...dccc",
    "...eeedd...",
    "...ee.dd...",
    "...e...d...",
    "...........",
];
const ROSCA: [&str; 20] = [
    "............bbbbbbb............",
    "........bssbbbbbbbbbssb........",
    ".....bbbbbssbbbbbbbssbbbbb.....",
    "....bbbbbbssbbbbbbbssbbbbbb....",
    "..ssbbbbbbbssbbbbbssbbbbbbbss..",
    ".sssbbbbbbbssbbbbbssbbbbbbbsss.",
    ".bbssssssbdddddddddddbssssssbb.",
    "bbbssssssbdd.......ddbssssssbbb",
    "bbbbbbbbsd...........dsbbbbbbbb",
    "bbbbbbbbs.............sbbbbbbbb",
    "dssssssssb...........bssssssssd",
    "dssssssssbbbs.....sbbbssssssssd",
    "ddbbbbbbbbbssbbbbbssbbbbbbbbbdd",
    ".dbbbbbbbbbssbbbbbssbbbbbbbbbd.",
    ".ddddbbbbsssbbbbbbbsssbbbbdddd.",
    "..dddbbbbsssbbbbbbbsssbbbbddd..",
    "...ddddddddbbbbbbbbbdddddddd...",
    ".....ddddddbbbbbbbbbdddddd.....",
    ".......ddddddddddddddddd.......",
    "...........ddddddddd...........",
];
const ROSCA_MUNECO: [&str; 20] = [
    "............bbbbbbb............",
    "........bssbbbbbbbbbssb........",
    ".....bbbbbssbbbbbbbssbbbbb.....",
    "....bbbbbbssbbbbbbbssbbbbbb....",
    "..ssbbbbbbbssbbbbbssbbbbbbbss..",
    ".sssbbbbbbbssbbbbbssbbbbbbbsss.",
    ".bbssssssbdddddddddddbssssssbb.",
    "bbbssssssbdd.......ddbssssssbbb",
    "bbbbbbbbsd...........dsbbbbbbbb",
    "bbbbbbbbs.............sbbbbbbbb",
    "dssssssssb..ppqqqqq..bssssssssd",
    "dssssssssbbbppqqqqqbbbssssssssd",
    "ddbbbbbbbbbssbbbbbssbbbbbbbbbdd",
    ".dbbbbbbbbbssbbbbbssbbbbbbbbbd.",
    ".ddddbbbbsssbbbbbbbsssbbbbdddd.",
    "..dddbbbbsssbbbbbbbsssbbbbddd..",
    "...ddddddddbbbbbbbbbdddddddd...",
    ".....ddddddbbbbbbbbbdddddd.....",
    ".......ddddddddddddddddd.......",
    "...........ddddddddd...........",
];
const ROSQUITA: [&str; 14] = [
    "........bbsbb........",
    "....bbbbbbsbbbbbb....",
    "...sbbbbbbsbbbbbbs...",
    ".sssbbbbbbsbbbbbbsss.",
    ".bbssssdddddddssssbb.",
    "bbbssss.......ssssbbb",
    "bbbbss.........ssbbbb",
    "bbbbsss.......sssbbbb",
    "dssssbbbbbsbbbbbssssd",
    ".ssssbbbbbsbbbbbssss.",
    ".dddbbbbbbsbbbbbbddd.",
    "..ddbbbbbbsbbbbbbdd..",
    "....ddddddddddddd....",
    ".......ddddddd.......",
];
const ROSQUITA_MUNECO: [&str; 14] = [
    "........bbsbb........",
    "....bbbbbbsbbbbbb....",
    "...sbbbbbbsbbbbbbs...",
    ".sssbbbbbbsbbbbbbsss.",
    ".bbssssdddddddssssbb.",
    "bbbssss.......ssssbbb",
    "bbbbss..pqqqq..ssbbbb",
    "bbbbsss.pqqqq.sssbbbb",
    "dssssbbbbbsbbbbbssssd",
    ".ssssbbbbbsbbbbbssss.",
    ".dddbbbbbbsbbbbbbddd.",
    "..ddbbbbbbsbbbbbbdd..",
    "....ddddddddddddd....",
    ".......ddddddd.......",
];

const PINATAS: [Size; 3] = [
    Size {
        frames: &[&PINATA],
        react: &PINATA_ROTA,
        trim: None,
    },
    Size {
        frames: &[&PINATILLA],
        react: &PINATILLA_ROTA,
        trim: None,
    },
    Size {
        frames: &[&PINATITA],
        react: &PINATITA_ROTA,
        trim: None,
    },
];

/// The rosca: golden bread, `d` its side in shadow, `s` the strips of sugar
/// paste across it, and with the muñeco in the hole (the Niño Dios, `p`
/// skin and `q` swaddled) when someone finds it.
const ROSCAS: [Size; 2] = [
    Size {
        frames: &[&ROSCA],
        react: &ROSCA_MUNECO,
        trim: Some(&JOYAS_GRANDES),
    },
    Size {
        frames: &[&ROSQUITA],
        react: &ROSQUITA_MUNECO,
        trim: Some(&JOYAS_CHICAS),
    },
];

/// The candied fruit that goes on the rosca (`j` ate, `l` acitrón, `m` a
/// higo), more of it as Reyes comes.
const JOYAS_GRANDES: Trim = Trim {
    piece: &["xxx", "xxx"],
    pieces: &[
        (15, 2, b'j'),
        (25, 10, b'l'),
        (5, 10, b'm'),
        (25, 4, b'j'),
        (5, 4, b'l'),
        (15, 12, b'm'),
        (26, 8, b'j'),
        (4, 8, b'l'),
        (21, 2, b'm'),
    ],
    leaves: &[],
    leaf: b'j',
    above: 0,
};
const JOYAS_CHICAS: Trim = Trim {
    piece: &["xx", "xx"],
    pieces: &[
        (11, 0, b'j'),
        (17, 6, b'l'),
        (4, 6, b'm'),
        (17, 2, b'j'),
        (4, 2, b'l'),
        (11, 8, b'm'),
    ],
    leaves: &[],
    leaf: b'j',
    above: 0,
};

/// What a piñata holds: colación, tejocotes, mandarinas, cañas, cacahuates
/// and caramelos.
const CANDY: [BitKind; 6] = [
    BitKind::new(['•', '·'], [Tint::Cycle(0), Tint::Cycle(0)], 3.0),
    BitKind::new(
        ['●', '●'],
        [Tint::Accent(5, TEJOCOTE), Tint::Accent(5, TEJOCOTE)],
        6.0,
    ),
    BitKind::new(
        ['◉', '●'],
        [Tint::Accent(4, MANDARINA), Tint::Accent(4, MANDARINA)],
        6.0,
    ),
    BitKind::new(
        ['╱', '╲'],
        [Tint::Accent(3, CANA), Tint::Accent(3, CANA)],
        5.0,
    ),
    BitKind::new(
        ['◖', '◗'],
        [Tint::Accent(2, CANA), Tint::Accent(2, CANA)],
        5.0,
    ),
    BitKind::new(['♦', '◆'], [Tint::Cycle(1), Tint::Cycle(3)], 4.0),
];

/// Thrown out of a piñata, a bit of everything; shed on the way, mostly
/// colación.
fn pick_candy(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=5 => 0,
            6..=8 => 1,
            9..=11 => 2,
            12..=14 => 3,
            15..=16 => 4,
            _ => 5,
        }
    } else {
        match n {
            0..=11 => 0,
            12..=16 => 5,
            _ => 4,
        }
    }
}

/// The air a piñata's contents fall through: thrown with some weight.
const HEAVY: Air = Air {
    drag: 0.7,
    settle: 0.0,
    gravity: 20.0,
    sway: 0.0,
};

const PINATA_LOOK: Look = Look {
    sizes: &PINATAS,
    // The picos take the papel de china colors in turn, the seventh one
    // skipping ahead so that it differs from both of its neighbors.
    parts: &[
        (b'a', Tint::Cycle(0)),
        (b'b', Tint::Cycle(1)),
        (b'c', Tint::Cycle(2)),
        (b'd', Tint::Cycle(3)),
        (b'e', Tint::Cycle(4)),
        (b'f', Tint::Cycle(5)),
        (b'g', Tint::Cycle(2)),
        (b'h', Tint::Ink(DORADO)),
        (b'r', Tint::Dim(CANA)),
    ],
    back: b'h',
    solid: false,
    classic: &PAPEL,
    banner: &["¡Dale, dale,", "dale!"],
    react: React::Pop,
    beat: 0.0,
    fall: Fall {
        kinds: &CANDY,
        pick: pick_candy,
        cap: 220,
        lie: (10.0, 30.0),
        shed: (0.25, 1.0),
        burst: (0.2, 0.2),
        speed: (4.0, 26.0),
        ttl: ((4.0, 3.0), (6.0, 4.0)),
        air: HEAVY,
    },
    joke: None,
};

/// Santos Inocentes, December 28: the piñata slows near a corner and misses
/// it by a cell.
const INOCENTES: Look = Look {
    joke: Some(&["¡Inocente", "palomita!"]),
    ..PINATA_LOOK
};

/// What the new year brings to the piñata: uvas, confeti and sparks of gold.
const NUEVO_BITS: [BitKind; 4] = [
    BitKind::new(
        ['●', '●'],
        [Tint::Accent(4, MORADO), Tint::Accent(4, MORADO)],
        6.0,
    ),
    BitKind::new(['•', '·'], [Tint::Cycle(0), Tint::Cycle(2)], 3.0),
    BitKind::new(
        ['✦', '✧'],
        [Tint::Accent(2, DORADO), Tint::Accent(2, DORADO)],
        4.0,
    ),
    BitKind::new(['♦', '◆'], [Tint::Cycle(1), Tint::Cycle(3)], 4.0),
];

fn pick_nuevo(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=5 => 0,
            6..=12 => 1,
            13..=16 => 2,
            _ => 3,
        }
    } else {
        match n {
            0..=13 => 1,
            _ => 2,
        }
    }
}

/// Año Nuevo, December 31 and January 1.
const NUEVO: Look = Look {
    banner: &["¡Feliz Año", "Nuevo!"],
    fall: Fall {
        kinds: &NUEVO_BITS,
        pick: pick_nuevo,
        ..PINATA_LOOK.fall
    },
    ..PINATA_LOOK
};

/// Sugar, acitrón, ate and figs flying from a rosca.
const ROSCA_BITS: [BitKind; 4] = [
    BitKind::new(['·', '∙'], [Tint::Ink(BLANCO), Tint::Ink(BLANCO)], 2.0),
    BitKind::new(['◆', '◇'], [Tint::Cycle(1), Tint::Cycle(1)], 4.0),
    BitKind::new(['◆', '◇'], [Tint::Cycle(0), Tint::Cycle(0)], 4.0),
    BitKind::new(['●', '○'], [Tint::Cycle(2), Tint::Cycle(2)], 5.0),
];

fn pick_rosca(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=8 => 0,
            9..=12 => 1,
            13..=17 => 2,
            _ => 3,
        }
    } else {
        match n {
            0..=15 => 0,
            _ => 2,
        }
    }
}

/// Reyes, January 2 to 6: a rosca, and at a corner the muñeco shows.
const REYES: Look = Look {
    sizes: &ROSCAS,
    parts: &[
        (b'b', Tint::Accent(3, rgb(0xd9a04a))),
        (b'd', Tint::Dim(rgb(0xa8692a))),
        (b's', Tint::Ink(BLANCO)),
        (b'p', Tint::Ink(rgb(0xf2c4a0))),
        (b'q', Tint::Cycle(4)),
        (b'j', Tint::Cycle(0)),
        (b'l', Tint::Cycle(1)),
        (b'm', Tint::Cycle(2)),
    ],
    back: b'd',
    solid: false,
    classic: &JOYAS,
    banner: &["¡Te salió", "el muñeco!"],
    react: React::Hold,
    beat: 0.0,
    fall: Fall {
        kinds: &ROSCA_BITS,
        pick: pick_rosca,
        air: Air {
            drag: 1.0,
            settle: 0.0,
            gravity: 14.0,
            sway: 0.0,
        },
        ..PINATA_LOOK.fall
    },
    joke: None,
};

/// What the logo is during the Navidad, by the day.
pub(super) fn look(f: Fiesta) -> &'static Look {
    if f.reyes() {
        &REYES
    } else if f.ano_nuevo() {
        &NUEVO
    } else if f.inocentes() {
        &INOCENTES
    } else {
        &PINATA_LOOK
    }
}
