//! The drawings, one character per pixel, facing right; `.` is see-through.
//! Animals use `B` body, `D` dark fins, `L` belly, `S` stripes or spots and
//! `F` tail, colored from one of their palettes, and `w`/`k` for the eye.
//!
//! tanim's tank comes first, as drawn there: its seven fish, the shark, the
//! crab and the castle. The rest are the sea's newcomers: sardines and
//! barracudas, the puffer blown up, orcas, dolphins and mantas.

use crate::color::Color;

pub type Sprite = &'static [&'static str];

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}

/// An animal's place in the food chain, which sets how it behaves.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    /// Small fish that swim in schools: everyone's dinner.
    School,
    /// Reef fish crossing on their own.
    Reef,
    /// Mid-size fish that snap up the smallest.
    Snapper,
    Shark,
    Dolphin,
    Orca,
    /// Glides through feeding on plankton; nobody hunts it.
    Manta,
}

pub struct Species {
    pub name: &'static str,
    /// Swimming frames in order: a tail beat for fish, a wingbeat for mantas.
    pub frames: &'static [Sprite],
    /// Color variants for `B`, `D`, `L`, `S` and `F`.
    pub palettes: &'static [[Color; 5]],
    pub role: Role,
    /// Cruising speed, pixels per tick.
    pub cruise: (f32, f32),
    /// Top speed, fleeing or lunging.
    pub burst: f32,
    /// How fast the frames turn over for the distance swum; big animals beat slower.
    pub beat: f32,
}

pub const TETRA: Species = Species {
    name: "tetra",
    frames: &[
        &[
            "....DD...",
            "F..BBBBB.",
            "FFSSSSSkB",
            "F..LLLLL.",
            ".....D...",
        ],
        &[
            "....DD...",
            ".F.BBBBB.",
            ".FSSSSSkB",
            ".F.LLLLL.",
            ".....D...",
        ],
    ],
    palettes: &[
        [
            rgb(176, 196, 214),
            rgb(130, 150, 170),
            rgb(226, 52, 62),
            rgb(60, 206, 244),
            rgb(196, 206, 220),
        ],
        [
            rgb(236, 196, 70),
            rgb(196, 150, 40),
            rgb(250, 230, 150),
            rgb(250, 140, 40),
            rgb(240, 180, 60),
        ],
        [
            rgb(130, 214, 120),
            rgb(80, 160, 80),
            rgb(210, 240, 190),
            rgb(40, 130, 70),
            rgb(150, 220, 130),
        ],
        [
            rgb(250, 150, 190),
            rgb(210, 100, 150),
            rgb(255, 214, 230),
            rgb(190, 60, 130),
            rgb(250, 170, 200),
        ],
    ],
    role: Role::School,
    cruise: (0.2, 0.32),
    burst: 0.85,
    beat: 1.0,
};

pub const CLOWN: Species = Species {
    name: "clownfish",
    frames: &[
        &[
            ".....DDD....",
            "F...BSBBBSB.",
            "FF.BBSBBBSkB",
            "FFBSBSBBBSBB",
            "F...BSBBBSB.",
            ".....D...D..",
        ],
        &[
            ".....DDD....",
            ".F..BSBBBSB.",
            ".FFBBSBBBSkB",
            ".FFSBSBBBSBB",
            ".F..BSBBBSB.",
            ".....D...D..",
        ],
    ],
    palettes: &[
        [
            rgb(250, 122, 30),
            rgb(40, 24, 20),
            rgb(250, 122, 30),
            rgb(250, 250, 250),
            rgb(240, 104, 26),
        ],
        [
            rgb(160, 30, 36),
            rgb(60, 14, 16),
            rgb(160, 30, 36),
            rgb(250, 236, 220),
            rgb(140, 24, 30),
        ],
        [
            rgb(250, 190, 40),
            rgb(200, 110, 20),
            rgb(250, 190, 40),
            rgb(250, 250, 240),
            rgb(250, 160, 30),
        ],
    ],
    role: Role::Reef,
    cruise: (0.08, 0.45),
    burst: 0.8,
    beat: 1.0,
};

pub const ANGEL: Species = Species {
    name: "angelfish",
    frames: &[
        &[
            "....D......",
            "....DD.....",
            "....DDB....",
            "...DBBBB...",
            "F..SBSBBB..",
            "FFBSBSBBBkB",
            "FFBSBSBBBBB",
            "F..SBSBBL..",
            "...DBSBL...",
            "....DDL....",
            "....DD.....",
            "....D......",
        ],
        &[
            "....D......",
            "....DD.....",
            "....DDB....",
            "...DBBBB...",
            ".F.SBSBBB..",
            ".FBSBSBBBkB",
            ".FBSBSBBBBB",
            ".F.SBSBBL..",
            "...DBSBL...",
            "....DDL....",
            "....DD.....",
            "....D......",
        ],
    ],
    palettes: &[
        [
            rgb(226, 230, 236),
            rgb(150, 156, 170),
            rgb(250, 250, 250),
            rgb(40, 40, 50),
            rgb(200, 206, 216),
        ],
        [
            rgb(250, 200, 60),
            rgb(220, 150, 40),
            rgb(250, 236, 150),
            rgb(230, 120, 30),
            rgb(250, 190, 60),
        ],
        [
            rgb(90, 90, 110),
            rgb(40, 40, 56),
            rgb(150, 150, 170),
            rgb(20, 20, 30),
            rgb(80, 80, 100),
        ],
    ],
    role: Role::Reef,
    // tanim slows the tall ones down.
    cruise: (0.048, 0.27),
    burst: 0.7,
    beat: 1.0,
};

pub const PUFFER: Species = Species {
    name: "pufferfish",
    frames: &[
        &[
            "....DDD...",
            "..BBBBBBB.",
            "F.BSBBSBwk",
            "FBBBBBBBBB",
            "FBSBBSBBBB",
            "F.LLLLLLL.",
            "...LLLLL..",
            "....D.D...",
        ],
        &[
            "....DDD...",
            "..BBBBBBB.",
            ".FBSBBSBwk",
            ".FBBBBBBBB",
            ".FSBBSBBBB",
            ".FLLLLLLL.",
            "...LLLLL..",
            "....D.D...",
        ],
    ],
    palettes: &[
        [
            rgb(230, 200, 80),
            rgb(170, 140, 50),
            rgb(250, 244, 210),
            rgb(120, 90, 40),
            rgb(210, 180, 70),
        ],
        [
            rgb(140, 190, 120),
            rgb(90, 130, 80),
            rgb(236, 240, 220),
            rgb(50, 80, 50),
            rgb(120, 170, 100),
        ],
        [
            rgb(236, 150, 90),
            rgb(180, 100, 60),
            rgb(252, 230, 210),
            rgb(130, 70, 40),
            rgb(220, 130, 80),
        ],
    ],
    role: Role::Reef,
    cruise: (0.08, 0.45),
    burst: 0.6,
    beat: 1.0,
};

/// A pufferfish blown up when something hungry comes too close.
pub const PUFFED: Sprite = &[
    "...D..D..D...",
    ".D.BBBBBBB.D.",
    "..BBSBBBSBB..",
    "DBBBBBBSBBwkD",
    "FBSBBSBBBBBBB",
    "FFBBBBBBSBBB.",
    "FBLLLLLLLLLLD",
    ".DLLLLLLLLLL.",
    "..DLLLLLLLD..",
    ".D..D..D..D..",
];

pub const TANG: Species = Species {
    name: "tang",
    frames: &[
        &[
            "......DDDD....",
            "F...DDBBBBD...",
            "FF.BBSSSSBBB..",
            "FFBBSBBBBSBkB.",
            "FFBBSSBBBBBBBB",
            "FF.BBBSSBBBB..",
            "F...DDBBBBD...",
            "......DDD.....",
        ],
        &[
            "......DDDD....",
            ".F..DDBBBBD...",
            ".FFBBSSSSBBB..",
            ".FFBSBBBBSBkB.",
            ".FFBSSBBBBBBBB",
            ".FFBBBSSBBBB..",
            ".F..DDBBBBD...",
            "......DDD.....",
        ],
    ],
    palettes: &[
        [
            rgb(40, 110, 230),
            rgb(30, 60, 150),
            rgb(40, 110, 230),
            rgb(16, 20, 40),
            rgb(250, 210, 40),
        ],
        [
            rgb(250, 220, 40),
            rgb(220, 180, 30),
            rgb(250, 220, 40),
            rgb(250, 240, 150),
            rgb(250, 250, 230),
        ],
        [
            rgb(150, 90, 210),
            rgb(90, 50, 150),
            rgb(150, 90, 210),
            rgb(250, 200, 60),
            rgb(250, 200, 60),
        ],
    ],
    role: Role::Reef,
    cruise: (0.08, 0.45),
    burst: 0.8,
    beat: 1.0,
};

pub const GOLDFISH: Species = Species {
    name: "goldfish",
    frames: &[
        &[
            "......DD......",
            "FF...BBBBB....",
            "FFF.BBBBBBBB..",
            ".FFBBBBBBBwkB.",
            "..FBBBBBBBBBBB",
            ".FFBBLLLLLBBB.",
            "FFF.LLLLLLL...",
            "FF....D..D....",
        ],
        &[
            "......DD......",
            ".....BBBBB....",
            "F...BBBBBBBB..",
            "FFFBBBBBBBwkB.",
            ".FFBBBBBBBBBBB",
            "FFFBBLLLLLBBB.",
            "F...LLLLLLL...",
            "......D..D....",
        ],
    ],
    palettes: &[
        [
            rgb(250, 130, 30),
            rgb(230, 90, 20),
            rgb(250, 190, 90),
            rgb(0, 0, 0),
            rgb(250, 150, 60),
        ],
        [
            rgb(236, 236, 230),
            rgb(230, 80, 40),
            rgb(250, 250, 250),
            rgb(0, 0, 0),
            rgb(240, 110, 70),
        ],
        [
            rgb(240, 60, 40),
            rgb(190, 30, 30),
            rgb(250, 150, 110),
            rgb(0, 0, 0),
            rgb(250, 100, 80),
        ],
    ],
    role: Role::Reef,
    cruise: (0.08, 0.45),
    burst: 0.75,
    beat: 1.0,
};

pub const GROUPER: Species = Species {
    name: "grouper",
    frames: &[
        &[
            "..........DDDD......",
            "F......DDDDDDDDD....",
            "FF...BBBBBBBBBBBBB..",
            "FFF.BBSBBBSBBBBBwkB.",
            "FFFBBBBBBBBBSBBBBBBB",
            "FFFBBSBBSBBBBBBBBBBD",
            "FFF.BBBBBBSBBBBBBBB.",
            "FF...LLLLLLLLLLLLL..",
            "F.......LLLLLLLL....",
            "........DD...DD.....",
        ],
        &[
            "..........DDDD......",
            ".......DDDDDDDDD....",
            "F....BBBBBBBBBBBBB..",
            "FF..BBSBBBSBBBBBwkB.",
            "FFFBBBBBBBBBSBBBBBBB",
            "FFFBBSBBSBBBBBBBBBBD",
            "FF..BBBBBBSBBBBBBBB.",
            "F....LLLLLLLLLLLLL..",
            "........LLLLLLLL....",
            "........DD...DD.....",
        ],
    ],
    palettes: &[
        [
            rgb(120, 110, 90),
            rgb(80, 72, 60),
            rgb(196, 186, 160),
            rgb(70, 60, 50),
            rgb(100, 92, 76),
        ],
        [
            rgb(90, 130, 160),
            rgb(50, 80, 110),
            rgb(190, 214, 226),
            rgb(40, 60, 90),
            rgb(80, 116, 146),
        ],
        [
            rgb(200, 80, 60),
            rgb(140, 50, 40),
            rgb(240, 180, 150),
            rgb(250, 220, 200),
            rgb(180, 70, 50),
        ],
    ],
    role: Role::Snapper,
    cruise: (0.048, 0.27),
    burst: 1.3,
    beat: 1.0,
};

/// tanim's reef fish, the ones that cross on their own.
pub const REEF: [&Species; 5] = [&CLOWN, &ANGEL, &PUFFER, &TANG, &GOLDFISH];

pub const SHARK: Species = Species {
    name: "shark",
    frames: &[
        &[
            "..................DD..............",
            ".................DDD..............",
            "DD..............DDDD..............",
            "DDD............DDDDD..............",
            ".DDD.......BBBBBBBBBBBBBBB........",
            "..DDDBBBBBBBBBBBBBBBBBBSBSBBBB....",
            "...DDBBBBBBBBBBBBBBBBBBSBSBBkBBBB.",
            "...DDLLLLLLLLLLLLLLLLLLLLLLLLLBBBB",
            "..DDD...LLLLLLLLLLLLLLLLLLLLSSSL..",
            ".DDD.........DDDD....LLLLLLL......",
            "DD..............DDD...............",
        ],
        &[
            "..................DD..............",
            ".................DDD..............",
            "................DDDD..............",
            ".DD............DDDDD..............",
            ".DDD.......BBBBBBBBBBBBBBB........",
            "..DDDBBBBBBBBBBBBBBBBBBSBSBBBB....",
            "...DDBBBBBBBBBBBBBBBBBBSBSBBkBBBB.",
            "..DDDLLLLLLLLLLLLLLLLLLLLLLLLLBBBB",
            ".DDD....LLLLLLLLLLLLLLLLLLLLSSSL..",
            ".DD..........DDDD....LLLLLLL......",
            "................DDD...............",
        ],
    ],
    palettes: &[[
        rgb(126, 138, 156),
        rgb(96, 106, 124),
        rgb(222, 226, 232),
        rgb(70, 76, 90),
        rgb(0, 0, 0),
    ]],
    role: Role::Shark,
    // tanim's shark cruises at 0.18.
    cruise: (0.18, 0.18),
    burst: 1.5,
    beat: 1.0,
};

#[rustfmt::skip]
pub const SARDINE: Species = Species {
    name: "sardine",
    frames: &[
        &[
            "F.BBBB.",
            "FFSSSkB",
            "F.LLLL.",
        ],
        &[
            ".FBBBB.",
            ".FSSSkB",
            ".FLLLL.",
        ],
    ],
    palettes: &[
        [rgb(70, 110, 150), rgb(50, 80, 110), rgb(214, 222, 232), rgb(150, 178, 204), rgb(110, 140, 170)],
        [rgb(90, 130, 120), rgb(60, 96, 90), rgb(222, 226, 216), rgb(176, 196, 186), rgb(110, 146, 136)],
    ],
    role: Role::School,
    cruise: (0.3, 0.42),
    burst: 1.0,
    beat: 1.0,
};

pub const BARRACUDA: Species = Species {
    name: "barracuda",
    frames: &[
        &[
            "F...........D.....D.....",
            "FF..BSBBSBBSBBSBBSBBBB..",
            ".FFBBBBBBBBBBBBBBBBBBkBB",
            "FF..LLLLLLLLLLLLLLLLLLLL",
            "F.........D....D........",
        ],
        &[
            "............D.....D.....",
            ".F..BSBBSBBSBBSBBSBBBB..",
            ".FFBBBBBBBBBBBBBBBBBBkBB",
            ".F..LLLLLLLLLLLLLLLLLLLL",
            "..........D....D........",
        ],
    ],
    palettes: &[[
        rgb(150, 165, 175),
        rgb(90, 100, 110),
        rgb(222, 228, 232),
        rgb(60, 70, 82),
        rgb(120, 130, 140),
    ]],
    role: Role::Snapper,
    cruise: (0.16, 0.26),
    burst: 1.7,
    beat: 0.8,
};

pub const ORCA: Species = Species {
    name: "orca",
    frames: &[ORCA_UP, ORCA_MID, ORCA_DOWN, ORCA_MID],
    palettes: &[[
        rgb(12, 14, 20),
        rgb(22, 24, 32),
        rgb(240, 242, 246),
        rgb(110, 116, 130),
        rgb(16, 18, 24),
    ]],
    role: Role::Orca,
    cruise: (0.4, 0.48),
    burst: 1.6,
    beat: 0.3,
};

pub const DOLPHIN: Species = Species {
    name: "dolphin",
    frames: &[DOLPHIN_UP, DOLPHIN_MID, DOLPHIN_DOWN, DOLPHIN_MID],
    palettes: &[
        // Bottlenose, and a common dolphin with its pale hourglass.
        [
            rgb(110, 124, 140),
            rgb(80, 92, 108),
            rgb(206, 212, 220),
            rgb(150, 162, 176),
            rgb(90, 102, 118),
        ],
        [
            rgb(64, 74, 94),
            rgb(46, 54, 70),
            rgb(222, 226, 232),
            rgb(196, 178, 124),
            rgb(56, 64, 82),
        ],
    ],
    role: Role::Dolphin,
    cruise: (0.55, 0.7),
    burst: 1.8,
    beat: 0.55,
};

pub const MANTA: Species = Species {
    name: "manta",
    frames: &[
        MANTA_FLAT, MANTA_MID, MANTA_HIGH, MANTA_TOP, MANTA_HIGH, MANTA_MID,
    ],
    palettes: &[
        [
            rgb(28, 32, 46),
            rgb(48, 54, 72),
            rgb(226, 230, 236),
            rgb(190, 196, 206),
            rgb(40, 44, 58),
        ],
        // The black morph, dark on both sides.
        [
            rgb(18, 20, 28),
            rgb(36, 40, 52),
            rgb(96, 100, 116),
            rgb(66, 70, 84),
            rgb(30, 32, 42),
        ],
    ],
    role: Role::Manta,
    cruise: (0.16, 0.24),
    burst: 0.4,
    beat: 0.9,
};

/// `R` shell, `r` legs and joints, `k` eyes; claws open and closed.
pub const CRAB: [Sprite; 2] = [
    &[
        "R.R.......R.R",
        "RRR.k...k.RRR",
        ".R..R...R..R.",
        "..RRRRRRRRR..",
        ".RRRRRRRRRRR.",
        "r.r.r...r.r.r",
    ],
    &[
        ".RR.......RR.",
        "RRR.k...k.RRR",
        ".R..R...R..R.",
        "..RRRRRRRRR..",
        ".RRRRRRRRRRR.",
        ".r.r.....r.r.",
    ],
];
pub const CRAB_W: i32 = 13;

/// `G` stone, `g` mortar, `r`/`R` roof, `k` openings.
pub const CASTLE: Sprite = &[
    "..r.........r..",
    ".rrR.......rrR.",
    ".rrR.......rrR.",
    "rrrRR.....rrrRR",
    "GGGGG.....GGGGG",
    "GGkGG.....GGkGG",
    "GGkGGG.G.GGGkGG",
    "GGGGGGGGGGGGGGG",
    "gGGgGGgGGGgGGgG",
    "GGGGGGkkkGGGGGG",
    "GGgGGkkkkkGGgGG",
    "GGGGGkkkkkGGGGG",
    "gGGGGkkkkkGGGGg",
];

/// Width and height of a sprite.
pub fn size(s: Sprite) -> (i32, i32) {
    (s.first().map_or(0, |r| r.len()) as i32, s.len() as i32)
}

/// Calls `f(x, y, ch)` for every opaque pixel, mirrored when `flip`.
pub fn each_pixel(s: Sprite, flip: bool, mut f: impl FnMut(i32, i32, u8)) {
    let sw = size(s).0;
    for (ry, row) in s.iter().enumerate() {
        for (rx, ch) in row.bytes().enumerate() {
            if ch != b'.' {
                let x = if flip { sw - 1 - rx as i32 } else { rx as i32 };
                f(x, ry as i32, ch);
            }
        }
    }
}

// The newcomers' frames: a stroke of the flukes for orcas and dolphins
// (up, level, down, level), and a wingbeat for mantas, seen from below as
// they pass overhead: the white belly, the dark rims of the wings, the gill
// slits and the horns, the span narrowing as the wings rise.

const ORCA_UP: Sprite = &[
    "..................D....................",
    "..................DD...................",
    "..................DDD..................",
    ".................DDDDD.................",
    ".................DDDDDDBBBBBBB.........",
    "FF............SSSBBBBBBBBBBBBBBBB......",
    ".FFF........SSSSBBBBBBBBBBBBBBBLLLBB...",
    "...FFBBBBBBBBBBBBBBBBBBBBBBBBBLLLLBBBB.",
    ".....BBBBBBLLLBBBBBBBBBBBBBBBBBBBBBBBBB",
    ".....BBBBBLLLLLLBBBBBBBBBBBBBBBBBBBLLLL",
    "............LLLLLLBBBBBBBBBBBBBBBLLLLL.",
    "..............BBLLLLLLLLLLLLLLLLB......",
    "..................BBLLLLLLDDDD.........",
    ".........................DDDD..........",
];
const ORCA_MID: Sprite = &[
    "..................D....................",
    "..................DD...................",
    "..................DDD..................",
    ".................DDDDD.................",
    ".................DDDDDDBBBBBBB.........",
    "..............SSSBBBBBBBBBBBBBBBB......",
    "............SSSSBBBBBBBBBBBBBBBLLLBB...",
    ".....BBBBBBBBBBBBBBBBBBBBBBBBBLLLLBBBB.",
    "FFFFFBBBBBBLLLBBBBBBBBBBBBBBBBBBBBBBBBB",
    ".....BBBBBLLLLLLBBBBBBBBBBBBBBBBBBBLLLL",
    "............LLLLLLBBBBBBBBBBBBBBBLLLLL.",
    "..............BBLLLLLLLLLLLLLLLLB......",
    "..................BBLLLLLLDDDD.........",
    ".........................DDDD..........",
];
const ORCA_DOWN: Sprite = &[
    "..................D....................",
    "..................DD...................",
    "..................DDD..................",
    ".................DDDDD.................",
    ".................DDDDDDBBBBBBB.........",
    "..............SSSBBBBBBBBBBBBBBBB......",
    "............SSSSBBBBBBBBBBBBBBBLLLBB...",
    ".....BBBBBBBBBBBBBBBBBBBBBBBBBLLLLBBBB.",
    ".....BBBBBBLLLBBBBBBBBBBBBBBBBBBBBBBBBB",
    "...FFBBBBBLLLLLLBBBBBBBBBBBBBBBBBBBLLLL",
    ".FFF........LLLLLLBBBBBBBBBBBBBBBLLLLL.",
    "FF............BBLLLLLLLLLLLLLLLLB......",
    "..................BBLLLLLLDDDD.........",
    ".........................DDDD..........",
];
const DOLPHIN_UP: Sprite = &[
    "..........DD..............",
    "..........DDD.............",
    ".........DDDDBBBB.........",
    "FF.....BBBBBBBBBBBBBBB....",
    ".FFFBBBBBBBBBBBBBBBBBkBB..",
    "....BBBSSSSSSSSSSSSSBBBBBB",
    "......LLLLLLLLLLLLLLLLLLL.",
    ".........LLLLLLLLLLL......",
    "...............DD.........",
];
const DOLPHIN_MID: Sprite = &[
    "..........DD..............",
    "..........DDD.............",
    ".........DDDDBBBB.........",
    ".......BBBBBBBBBBBBBBB....",
    "....BBBBBBBBBBBBBBBBBkBB..",
    "FFFFBBBSSSSSSSSSSSSSBBBBBB",
    "......LLLLLLLLLLLLLLLLLLL.",
    ".........LLLLLLLLLLL......",
    "...............DD.........",
];
const DOLPHIN_DOWN: Sprite = &[
    "..........DD..............",
    "..........DDD.............",
    ".........DDDDBBBB.........",
    ".......BBBBBBBBBBBBBBB....",
    "....BBBBBBBBBBBBBBBBBkBB..",
    "....BBBSSSSSSSSSSSSSBBBBBB",
    ".FFF..LLLLLLLLLLLLLLLLLLL.",
    "FF.......LLLLLLLLLLL......",
    "...............DD.........",
];
const MANTA_FLAT: Sprite = &[
    "......D.............",
    ".......DD...........",
    "........DD..........",
    "........DBD.........",
    ".........DBLD.......",
    ".........DBLLD......",
    ".........DBLLLD.....",
    ".........DBLLLLD....",
    ".........DBLLLLD....",
    "........DBSLSLSLD..D",
    ".....DBLLLSLSLSLLDDD",
    ".....DBLLLLLLLLLLD..",
    "FFFFFDBLLLLLLLLLLD..",
    ".....DBLLLLLLLLLLD..",
    ".....DBLLLSLSLSLLDDD",
    "........DBSLSLSLD..D",
    ".........DBLLLLD....",
    ".........DBLLLLD....",
    ".........DBLLLD.....",
    ".........DBLLD......",
    ".........DBLD.......",
    "........DBD.........",
    "........DD..........",
    ".......DD...........",
    "......D.............",
];
const MANTA_MID: Sprite = &[
    "....................",
    "....................",
    ".......D............",
    "........DD..........",
    ".........DD.........",
    ".........DBBD.......",
    ".........DBLLD......",
    ".........DBLLLD.....",
    ".........DBLLLLD....",
    ".........DBLSLSLD..D",
    ".....DBLLLSLSLSLLDDD",
    ".....DBLLLLLLLLLLD..",
    "FFFFFDBLLLLLLLLLLD..",
    ".....DBLLLLLLLLLLD..",
    ".....DBLLLSLSLSLLDDD",
    ".........DBLSLSLD..D",
    ".........DBLLLLD....",
    ".........DBLLLD.....",
    ".........DBLLD......",
    ".........DBBD.......",
    ".........DD.........",
    "........DD..........",
    ".......D............",
    "....................",
    "....................",
];
const MANTA_HIGH: Sprite = &[
    "....................",
    "....................",
    "....................",
    "....................",
    "........D...........",
    ".........DD.........",
    "..........DBD.......",
    "..........DBBBD.....",
    "..........DBLLLD....",
    ".........DBLSLSLD..D",
    ".....DBLLLSLSLSLLDDD",
    ".....DBLLLLLLLLLLD..",
    "FFFFFDBLLLLLLLLLLD..",
    ".....DBLLLLLLLLLLD..",
    ".....DBLLLSLSLSLLDDD",
    ".........DBLSLSLD..D",
    "..........DBLLLD....",
    "..........DBBBD.....",
    "..........DBD.......",
    ".........DD.........",
    "........D...........",
    "....................",
    "....................",
    "....................",
    "....................",
];
const MANTA_TOP: Sprite = &[
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    ".........D..........",
    "..........DD........",
    "..........DBBD......",
    "...........DBBBD....",
    "..........DBSLSLD..D",
    ".....DBLLLSLSLSLLDDD",
    ".....DBLLLLLLLLLLD..",
    "FFFFFDBLLLLLLLLLLD..",
    ".....DBLLLLLLLLLLD..",
    ".....DBLLLSLSLSLLDDD",
    "..........DBSLSLD..D",
    "...........DBBBD....",
    "..........DBBD......",
    "..........DD........",
    ".........D..........",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
];

// Día de Muertos: alebrijes, Oaxaca's wildly painted creatures, swimming in
// schools and on their own, and the calavera paint the sharks and orcas wear
// for the holiday.

const ROSA: Color = rgb(236, 30, 136);
const TURQUESA: Color = rgb(0, 186, 196);
const AMARILLO: Color = rgb(255, 214, 20);
const MORADO: Color = rgb(150, 70, 206);
const LIMON: Color = rgb(150, 210, 50);
const NARANJA: Color = rgb(255, 140, 0);

/// Every alebrije wears its own: no two in a school alike.
const ALEBRIJE_PALETTES: &[[Color; 5]] = &[
    [ROSA, TURQUESA, AMARILLO, TURQUESA, MORADO],
    [TURQUESA, ROSA, AMARILLO, ROSA, AMARILLO],
    [AMARILLO, MORADO, ROSA, MORADO, ROSA],
    [MORADO, AMARILLO, LIMON, AMARILLO, TURQUESA],
    [LIMON, ROSA, AMARILLO, MORADO, ROSA],
    [NARANJA, TURQUESA, AMARILLO, ROSA, TURQUESA],
];

pub const ALEBRIJITO: Species = Species {
    name: "alebrijito",
    frames: &[
        &[
            "....DSD...",
            "F..BSBSBB.",
            "FFSLLLLLkB",
            "F..BSBSBB.",
            ".....D....",
        ],
        &[
            "....DSD...",
            ".F.BSBSBB.",
            ".FSLLLLLkB",
            ".F.BSBSBB.",
            ".....D....",
        ],
    ],
    palettes: ALEBRIJE_PALETTES,
    role: Role::School,
    cruise: (0.2, 0.32),
    burst: 0.85,
    beat: 1.0,
};

pub const ALEBRIJE: Species = Species {
    name: "alebrije",
    frames: &[
        &[
            ".......D.D.D.....",
            "F.....DSDSDSD....",
            "FF...BBBBBBBBBB..",
            "FSF.BSLSBSLSBBwB.",
            "FFSBBLSLBLSLBwkBB",
            "FSF.BSLSBSLSBBBBB",
            "FF...BBBBBBBBBB..",
            "F.....DSDSD......",
            ".......D.D.......",
        ],
        &[
            ".......D.D.D.....",
            ".F....DSDSDSD....",
            ".FF..BBBBBBBBBB..",
            ".SF.BSLSBSLSBBwB.",
            ".FSBBLSLBLSLBwkBB",
            ".SF.BSLSBSLSBBBBB",
            ".FF..BBBBBBBBBB..",
            ".F....DSDSD......",
            ".......D.D.......",
        ],
    ],
    palettes: ALEBRIJE_PALETTES,
    role: Role::Reef,
    cruise: (0.08, 0.4),
    burst: 0.8,
    beat: 1.0,
};

/// Calavera paint laid over a shark or an orca, the size of its frames:
/// `h` sugar white, `o` the dark of a socket or a stitch, `p` rosa, `t`
/// turquesa, `y` amarillo and `m` cempasúchil.
pub fn face_paint(sp: &Species) -> Option<Sprite> {
    match sp.role {
        Role::Shark => Some(SHARK_PAINT),
        Role::Orca => Some(ORCA_PAINT),
        _ => None,
    }
}

const SHARK_PAINT: Sprite = &[
    "..................................",
    "..................................",
    "..................................",
    ".........................m........",
    "........................mym.......",
    "........................tmhhph....",
    "........................pthpophoh.",
    ".........................phhphhhhh",
    "...........................hohoh..",
    "..................................",
    "..................................",
];

const ORCA_PAINT: Sprite = &[
    ".......................................",
    ".......................................",
    ".......................................",
    ".......................................",
    "...........................m...........",
    "..........................mym.p.t......",
    "...........................m..toohp....",
    ".............................phooht....",
    "...............................t.p.hhoh",
    "...................................ohoh",
    ".......................................",
    ".......................................",
    ".......................................",
    ".......................................",
];
