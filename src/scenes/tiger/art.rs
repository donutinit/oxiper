//! The tiger scene's sprites, drawn by hand one character per pixel, and how
//! each animal is built around them.
//!
//! Every sprite faces right. `.` is see-through and any other character is a
//! palette entry. Bodies are drawn without legs (see `body.rs`). The tiger's
//! letters:
//!
//! - `O` coat, `o` coat in shade, `S` stripes, `s` stripes in shade;
//! - `W` white fur, `w` white in shade;
//! - `E` the backs of the ears, `e` eye, `k` pupils, lips and nose leather,
//!   `n` nose;
//! - `m` the inside of the mouth, `t` teeth, `p` tongue.

use super::Rgb;
use super::body::{Build, LegSpec, TailSpec};

pub type Sprite = &'static [&'static str];

/// A sprite's colors: letter and color.
pub type Palette = &'static [(u8, Rgb)];

// ----- the tiger -----

/// Shoulders to rump, without legs, tail or head.
pub const TIGER_TORSO: Sprite = &[
    "..................oooo.....",
    "..oooSoooooSooooSoooSoo....",
    ".ooOSOOOoSOOOOSOOOSOOOSoo..",
    "oOOSOOSOOSOOOOSOOOOSOOOSOo.",
    "oOSOOSOOOOSOOOOSOOOSOOOOOOo",
    "OOSOSOOOOOSOOOOSOOOOSOOOOOO",
    "OSOOSOOOOOOSOOOOSOOOOOOOOOW",
    "OSOSOOOOOOOSOOOOOOOOOOOOOWW",
    "OOSOOOSOOOOWWWWWWWOOOOOOWWW",
    "OOOSOOOSOWWw...wWWWWOOOOWWW",
    ".OOOSOOOOw.......wwWWWWWWW.",
    "..OOOOOO...........wWWWWW..",
];

/// The head, mouth shut. It joins the torso at [`HEAD_AT`] and turns about
/// [`HEAD_PIVOT`].
pub const TIGER_HEAD: Sprite = &[
    "..EE..........",
    ".EWWE.........",
    ".EOOOooo......",
    "oOOSOOSOoo....",
    "OOOOSOOOOOo...",
    "OSOOOOWWWOOo..",
    "OOSOOOWkeWOOO.",
    "OOOSOOOWWOOOOo",
    "OSOOOOOOOOWWnn",
    "OOSOWWWWWWWWWk",
    ".OSWWSWWWWkkk.",
    "..WWWWSWWWW...",
    "...WWWWW......",
    "..............",
    "..............",
];

/// Eyes shut for a moment.
pub const TIGER_HEAD_BLINK: Sprite = &[
    "..EE..........",
    ".EWWE.........",
    ".EOOOooo......",
    "oOOSOOSOoo....",
    "OOOOSOOOOOo...",
    "OSOOOOWWWOOo..",
    "OOSOOOWkkWOOO.",
    "OOOSOOOWWOOOOo",
    "OSOOOOOOOOWWnn",
    "OOSOWWWWWWWWWk",
    ".OSWWSWWWWkkk.",
    "..WWWWSWWWW...",
    "...WWWWW......",
    "..............",
    "..............",
];

/// Lips drawn back, fangs showing: a snarl, or a roar starting.
pub const TIGER_HEAD_SNARL: Sprite = &[
    "..............",
    "..EE..........",
    ".EWWEooo......",
    "oEOSOOSOoo....",
    "OOOOSOOOOOo...",
    "OSOOOOWWkOOo..",
    "OOSOOOWWWWOOO.",
    "OOOSOOOWWOOOOo",
    "OSOOOOOOOOWWnn",
    "OOSOWWWWtWWWtk",
    ".OSWWmmmmmmmk.",
    "..WWWWtWWWt...",
    "...WWWWWW.....",
    "..............",
    "..............",
];

/// The roar: ears flat, eyes narrowed, jaws wide.
pub const TIGER_HEAD_ROAR: Sprite = &[
    "..............",
    "..............",
    ".EEEoooo......",
    "oEWOSOSOoo....",
    "OOOOSOOOOOo...",
    "OSOOOOWkkOOo..",
    "OOSOOWWWWWOOO.",
    "OOOSOOOWWOOOOo",
    "OSOOOOOOOOWWnn",
    "OOSOWWWWtWWWtk",
    ".OSWmmmmmmmm..",
    "..WWmmpppmm...",
    "..WWWmmmmmt...",
    "...WWWWtWWW...",
    "....WWWWW.....",
];

/// Where the head's top left corner sits on the torso, and the pixel of the
/// head it turns about (the back of the neck).
pub const HEAD_AT: (i32, i32) = (25, -5);
pub const HEAD_PIVOT: (i32, i32) = (2, 10);

/// Face on, for the middle of a turn: head, chest and fore legs. Its bottom
/// row stands on the ground.
pub const TIGER_FRONT: Sprite = &[
    "..EE.........EE..",
    ".EWWE.......EWWE.",
    ".EOOOoooooooOOOE.",
    "..OOSOSOSOSOSOO..",
    ".OOOOOSOSOSOOOOO.",
    ".OSOWWOOSOOWWOSO.",
    "OOSOWkeOOOekWOSOO",
    "OSOOOWWOOOWWOOOSO",
    "OOSOOOOOnOOOOOSOO",
    ".OSOOWWWnWWWOOSO.",
    ".WOSWWWkkkWWWSOW.",
    "..WWWWkWWWkWWWW..",
    "...WWWWWWWWWWW...",
    "...OOOWWWWWOOO...",
    "..OOSOWWWWWOSOO..",
    "..OSOOOWWWOOOSO..",
    "..OOOOOWWWOOOOO..",
    "..OOS.oWWWo.SOO..",
    "..OOO.oo.oo.OOO..",
    "..OSO.os.so.OSO..",
    "..OOO.oo.oo.OOO..",
    "..OSO.ww.ww.OSO..",
    ".WWWW......WWWW..",
    ".WWWW......WWWW..",
];

/// Asleep on its side, curled, breathing in and out.
pub const TIGER_SLEEP: [Sprite; 2] = [
    &[
        "...................................",
        "...................................",
        "....oooSoooSooooSoooo...EE.........",
        "..ooOOSOOOSOOOSOOOSOOoooEWWEoo.....",
        ".oOSOOOSOOOSOOOOSOOOOOOSOOSOOOo....",
        "oOSOOSOOOOSOOOOSOOOOOOSOOOOOOOOo...",
        "OSOOSOOOOOSOOOOSOOOOOOOOOOkkWOOOo..",
        "OOSOOOOOOOOOOOOOOOOOOOSOOOWWWWWOOn.",
        "OSOOOOWWWWWWWWWWWOOOOOOOOWWWWWWWWk.",
        "SOOOWWWWWWWWWWWWWWWOOOOOWWSWWWkkk..",
        "SSOOOOOOOOOOOOOOOOSOOOOOOOOSWWWWW..",
        ".SSSOOOOOOOOOOO..WWWWOOSOOOOWWWW...",
    ],
    &[
        "...................................",
        "....oooSoooSooooSoooo..............",
        "..ooOOSOOOSOOOSOOOSOOoo.EE.........",
        ".oOSOOOSOOOSOOOOSOOOOOooEWWEoo.....",
        "oOSOOSOOOSOOOOOSOOOOOOSOOSOOOOo....",
        "OSOOSOOOOOSOOOOSOOOOOOSOOOOOOOOo...",
        "OSOOSOOOOOSOOOOSOOOOOOOOOOkkWOOOo..",
        "OOSOOOOOOOOOOOOOOOOOOOSOOOWWWWWOOn.",
        "OSOOOOWWWWWWWWWWWOOOOOOOOWWWWWWWWk.",
        "SOOOWWWWWWWWWWWWWWWOOOOOWWSWWWkkk..",
        "SSOOOOOOOOOOOOOOOOSOOOOOOOOSWWWWW..",
        ".SSSOOOOOOOOOOO..WWWWOOSOOOOWWWW...",
    ],
];

const fn tiger_build(coat: u8) -> Build {
    Build {
        hind: LegSpec {
            hip: (4.5, 7.5),
            upper: 6.5,
            lower: 6.5,
            w_up: 4.2,
            w_lo: 3.0,
            fore: false,
        },
        fore: LegSpec {
            hip: (21.0, 8.5),
            upper: 6.0,
            lower: 5.5,
            w_up: 4.2,
            w_lo: 3.4,
            fore: true,
        },
        far_dx: 2.0,
        ground: 20.0,
        stride: 9.0,
        lift: 3.0,
        leg: (coat, b'o'),
        stripe: Some((b'S', b's')),
        paw: (b'W', b'w'),
        paw_w: 5,
        tail: Some(TailSpec {
            root: (1.0, 2.5),
            links: 19,
            w0: 3.0,
            w1: 2.0,
            coat,
            ring: Some(b'S'),
            tip: b'S',
            tip_len: 3,
        }),
    }
}

pub const TIGER: Build = tiger_build(b'O');

/// The tiger's colors, by kind: Bengal, Siberian and white.
pub const TIGER_COLORS: [Palette; 3] = [
    &[
        (b'O', Rgb(226, 124, 38)),
        (b'o', Rgb(172, 86, 26)),
        (b'S', Rgb(30, 22, 20)),
        (b's', Rgb(54, 36, 26)),
        (b'W', Rgb(246, 240, 226)),
        (b'w', Rgb(206, 186, 156)),
        (b'E', Rgb(34, 26, 22)),
        (b'e', Rgb(232, 198, 62)),
        (b'k', Rgb(24, 18, 16)),
        (b'n', Rgb(196, 112, 110)),
        (b'm', Rgb(112, 22, 30)),
        (b't', Rgb(250, 248, 236)),
        (b'p', Rgb(226, 110, 130)),
    ],
    &[
        (b'O', Rgb(222, 150, 88)),
        (b'o', Rgb(174, 112, 66)),
        (b'S', Rgb(52, 36, 30)),
        (b's', Rgb(80, 58, 46)),
        (b'W', Rgb(250, 246, 238)),
        (b'w', Rgb(210, 194, 170)),
        (b'E', Rgb(46, 34, 30)),
        (b'e', Rgb(214, 196, 90)),
        (b'k', Rgb(30, 22, 20)),
        (b'n', Rgb(190, 120, 116)),
        (b'm', Rgb(112, 22, 30)),
        (b't', Rgb(250, 248, 236)),
        (b'p', Rgb(226, 110, 130)),
    ],
    &[
        (b'O', Rgb(240, 236, 226)),
        (b'o', Rgb(190, 186, 180)),
        (b'S', Rgb(48, 42, 44)),
        (b's', Rgb(96, 90, 92)),
        (b'W', Rgb(255, 255, 255)),
        (b'w', Rgb(214, 214, 220)),
        (b'E', Rgb(48, 42, 44)),
        (b'e', Rgb(104, 172, 232)),
        (b'k', Rgb(28, 24, 26)),
        (b'n', Rgb(224, 150, 160)),
        (b'm', Rgb(112, 22, 30)),
        (b't', Rgb(250, 248, 236)),
        (b'p', Rgb(226, 110, 130)),
    ],
];

// ----- what it hunts -----

/// A chital stag, head up. `B` coat, `b` back, `W` spots, belly and rump,
/// `k` eye, nose and hooves, `A` antlers.
pub const CHITAL: Sprite = &[
    "......................A..A.",
    "......................A.A..",
    ".....................AAA...",
    "......................Ab...",
    ".....................bBBk..",
    ".....................BBBBBk",
    ".....................BBW...",
    "....................BBW....",
    "..bbbbbbbbbbbbbbbbbBBW.....",
    "WbBBWBBWBBBWBBWBBBBBW......",
    "WBBBBBWBBBWBBBBWBBBBW......",
    ".BBWBBBBWBBBBWBBBBBW.......",
    "..BBBBWWWWWWWWWBBBW........",
    "...BBBB.......BBB..........",
];

/// The chital grazing, head down in the grass.
pub const CHITAL_GRAZE: Sprite = &[
    "...........................",
    "...........................",
    "...........................",
    "...........................",
    "...........................",
    "...........................",
    "...........................",
    "...........................",
    "..bbbbbbbbbbbbbbbbbbb......",
    "WbBBWBBWBBBWBBWBBBBBBB.....",
    "WBBBBBWBBBWBBBBWBBBBBBBA...",
    ".BBWBBBBWBBBBWBBBBBWBBBBAA.",
    "..BBBBWWWWWWWWWBBBW..WBBkA.",
    "...BBBB.......BBB.....BBB..",
];

pub const DEER: Build = Build {
    hind: LegSpec {
        hip: (5.0, 12.0),
        upper: 4.5,
        lower: 5.0,
        w_up: 2.4,
        w_lo: 1.2,
        fore: false,
    },
    fore: LegSpec {
        hip: (16.0, 12.0),
        upper: 4.5,
        lower: 4.5,
        w_up: 2.0,
        w_lo: 1.2,
        fore: true,
    },
    far_dx: 1.5,
    ground: 22.0,
    stride: 7.0,
    lift: 2.5,
    leg: (b'B', b'b'),
    stripe: None,
    paw: (b'k', b'k'),
    paw_w: 1,
    tail: None,
};

pub const CHITAL_COLORS: Palette = &[
    (b'B', Rgb(196, 120, 60)),
    (b'b', Rgb(128, 76, 40)),
    (b'W', Rgb(246, 238, 222)),
    (b'k', Rgb(30, 22, 18)),
    (b'A', Rgb(214, 196, 160)),
];

/// A sambar: bigger, darker and shaggier, with no spots.
pub const SAMBAR_COLORS: Palette = &[
    (b'B', Rgb(108, 82, 62)),
    (b'b', Rgb(70, 52, 40)),
    (b'W', Rgb(112, 86, 66)),
    (b'k', Rgb(24, 18, 14)),
    (b'A', Rgb(176, 154, 116)),
];

/// A red deer of the taiga.
pub const RED_DEER_COLORS: Palette = &[
    (b'B', Rgb(150, 92, 58)),
    (b'b', Rgb(98, 60, 40)),
    (b'W', Rgb(154, 96, 62)),
    (b'k', Rgb(26, 20, 16)),
    (b'A', Rgb(196, 176, 140)),
];

/// A wild boar. `B` bristles, `b` darker, `W` the pale snout and cheek,
/// `t` tusks, `k` eye and nose.
pub const BOAR: Sprite = &[
    "...b.b.bbb...........",
    "..bBbBbBBBbbb........",
    ".bBBBBBBBBBBBbb......",
    "bBBBBBBBBBBBBBBbk....",
    "BBBBBBBBBBBBBBBBBWW..",
    "BBBBBBBBBBBBBBBBWWWWk",
    ".BBBBBBBBBBBBBBBBWtW.",
    "..BBBBBBBBBBBBBBB.t..",
];

pub const PIG: Build = Build {
    hind: LegSpec {
        hip: (4.0, 6.5),
        upper: 2.5,
        lower: 2.5,
        w_up: 2.6,
        w_lo: 1.6,
        fore: false,
    },
    fore: LegSpec {
        hip: (14.0, 6.5),
        upper: 2.5,
        lower: 2.5,
        w_up: 2.4,
        w_lo: 1.6,
        fore: true,
    },
    far_dx: 1.5,
    ground: 12.0,
    stride: 4.0,
    lift: 1.5,
    leg: (b'B', b'b'),
    stripe: None,
    paw: (b'k', b'k'),
    paw_w: 1,
    tail: Some(TailSpec {
        root: (0.0, 2.0),
        links: 4,
        w0: 1.0,
        w1: 1.0,
        coat: b'b',
        ring: None,
        tip: b'b',
        tip_len: 1,
    }),
};

pub const BOAR_COLORS: Palette = &[
    (b'B', Rgb(86, 72, 64)),
    (b'b', Rgb(52, 42, 38)),
    (b'W', Rgb(150, 134, 118)),
    (b't', Rgb(236, 228, 200)),
    (b'k', Rgb(20, 16, 14)),
];

/// A peacock, walking (two frames) and taking off. `U` blue neck, `u` its
/// shade, `V` the green train, `v` its shade, `Y` eyespots, `B` wings,
/// `k` legs and eye.
pub const PEACOCK: [Sprite; 3] = [
    &[
        "..................Uk.",
        "..................UUk",
        "..................U..",
        "..................U..",
        "VVvVV............UU..",
        "VYVVYVVvVV.....BBUU..",
        "vVVYVVYVVVVVBBBBBU...",
        ".vVVvVVVvVVVBBBBB....",
        "....vvvVVvvvvBBB.....",
        "..............kk.....",
        ".............k..k....",
        ".............k...k...",
    ],
    &[
        "..................Uk.",
        "..................UUk",
        "..................U..",
        "..................U..",
        "VVvVV............UU..",
        "VYVVYVVvVV.....BBUU..",
        "vVVYVVYVVVVVBBBBBU...",
        ".vVVvVVVvVVVBBBBB....",
        "....vvvVVvvvvBBB.....",
        "...............k.....",
        "...............k.....",
        "..............k......",
    ],
    &[
        "...........BB.......Uk",
        "..........BBBB.....UUk",
        "VVvVV.....BBBBB...UU..",
        "VYVVYVVvVVVBBBBBUUU...",
        "vVVYVVYVVVVVBBBBUU....",
        ".vVVvVVVvVVVVBBBB.....",
        "....vvvVVvvvvv........",
        "...........kk.........",
        "............kk........",
        "......................",
        "......................",
        "......................",
    ],
];

pub const PEACOCK_COLORS: Palette = &[
    (b'U', Rgb(36, 92, 204)),
    (b'u', Rgb(28, 60, 140)),
    (b'V', Rgb(44, 150, 92)),
    (b'v', Rgb(24, 96, 62)),
    (b'Y', Rgb(230, 190, 64)),
    (b'B', Rgb(150, 116, 80)),
    (b'k', Rgb(40, 34, 30)),
];

/// A grey langur sitting on a branch (and barking, mouth open), and
/// leaping. `G` fur, `g` its shade, `K` the black face and hands, `W` the
/// pale chest, `k` the mouth.
pub const LANGUR: [Sprite; 3] = [
    &[
        "....GGG.", "...GKKKG", "...GKKK.", "..GGGG..", ".GGGWG..", ".GGGWGK.", "GGGGGG..",
        "gGGgGGK.", "g...gg..", "g.......", ".g......", "..g.....",
    ],
    &[
        "....GGG.", "...GKKKG", "...GKkK.", "..GGGG..", ".GGGWGK.", ".GGGWG.K", "GGGGGG..",
        "gGGgGGK.", "g...gg..", "g.......", "g.......", ".g......",
    ],
    &[
        "...........GGG.",
        "..........GKKKG",
        ".......GGGGKKK.",
        "gg...GGGGWGGK..",
        "..ggGGGGGGG..K.",
        "....gG.....G...",
        "....g.......G..",
        "...g...........",
        "...............",
        "...............",
        "...............",
        "...............",
    ],
];

pub const LANGUR_COLORS: Palette = &[
    (b'G', Rgb(176, 172, 162)),
    (b'g', Rgb(124, 120, 114)),
    (b'K', Rgb(30, 28, 32)),
    (b'W', Rgb(226, 222, 212)),
    (b'k', Rgb(150, 50, 50)),
];

/// A small bird hopping, and flying with its wings up and down. `B` back,
/// `b` wing, `W` breast, `Y` bill, `k` eye.
pub const BIRD: [Sprite; 3] = [
    &["..Bk.", "bBBBY", ".WW..", ".k.k."],
    &["b...b", ".bBbY", "..W..", "....."],
    &[".....", "bBBBY", "b.W.b", "....."],
];

pub const BIRD_COLORS: [Palette; 3] = [
    &[
        (b'B', Rgb(112, 84, 60)),
        (b'b', Rgb(76, 56, 42)),
        (b'W', Rgb(226, 214, 190)),
        (b'Y', Rgb(234, 196, 70)),
        (b'k', Rgb(20, 20, 20)),
    ],
    &[
        (b'B', Rgb(40, 110, 170)),
        (b'b', Rgb(28, 70, 120)),
        (b'W', Rgb(234, 160, 70)),
        (b'Y', Rgb(30, 30, 30)),
        (b'k', Rgb(20, 20, 20)),
    ],
    &[
        (b'B', Rgb(50, 140, 70)),
        (b'b', Rgb(30, 96, 50)),
        (b'W', Rgb(220, 220, 120)),
        (b'Y', Rgb(214, 70, 50)),
        (b'k', Rgb(20, 20, 20)),
    ],
];

// ----- those who try it -----

/// A mugger crocodile: lurking with eyes and nostrils above the water, and
/// lunging, jaws open. `C` hide, `c` its shade, `Y` belly, `e` eye, `t`
/// teeth, `k` mouth.
pub const CROC_LURK: Sprite = &[
    "...............................ee.....kk.",
    "c.c.c.c.c.c.cCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
    "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC",
];

pub const CROC_LUNGE: Sprite = &[
    "..................................tttt..",
    "..............................eeCCCCCCt.",
    "...........................cCCCCCCCCCCt.",
    "........................cCCCCCC.kkkkkk..",
    "c.c.c.c.c.c.c.c.cCCCCCCCYYYYYtYtYtYt....",
    "CCCCCCCCCCCCCCCCCCCCCCCCCYYYYYYYYY......",
    "cCCCCCCCCCCCCCCCCCCYYYYYYYYYYYY.........",
];

pub const CROC_COLORS: Palette = &[
    (b'C', Rgb(92, 108, 62)),
    (b'c', Rgb(60, 72, 42)),
    (b'Y', Rgb(190, 180, 124)),
    (b'e', Rgb(220, 200, 60)),
    (b't', Rgb(240, 236, 220)),
    (b'k', Rgb(70, 30, 30)),
];

/// A sloth bear: shaggy, black, the pale V on its chest and a pale snout.
/// `K` fur, `k` its shade, `W` the chest mark, `T` snout, `e` eye.
pub const BEAR: Sprite = &[
    "..........kkkk...........",
    ".....kKKKKKKKKKKk...kk...",
    "...kKKKKKKKKKKKKKKkKKKk..",
    "..kKKKKKKKKKKKKKKKKKKeKk.",
    ".kKKKKKKKKKKKKKKKKKKKKTTT",
    "kKKKKKKKKKKKKKKKKKKKKKTTk",
    "KKKKKKKKKKKKKKKKKKWKWKK..",
    "KKKKKKKKKKKKKKKKKKKWKKK..",
    "kKKKKKKKKKKKKKKKKKKKKK...",
    ".kKKKKKKKKKKKKKKKKKKk....",
];

pub const BEAR_BUILD: Build = Build {
    hind: LegSpec {
        hip: (5.0, 7.0),
        upper: 4.0,
        lower: 4.0,
        w_up: 5.0,
        w_lo: 4.0,
        fore: false,
    },
    fore: LegSpec {
        hip: (18.0, 7.0),
        upper: 4.0,
        lower: 4.0,
        w_up: 4.6,
        w_lo: 4.0,
        fore: true,
    },
    far_dx: 2.0,
    ground: 15.0,
    stride: 6.0,
    lift: 2.0,
    leg: (b'K', b'k'),
    stripe: None,
    paw: (b'c', b'c'),
    paw_w: 5,
    tail: None,
};

/// The sloth bear of the jungle, and the brown bear of the taiga.
pub const BEAR_COLORS: [Palette; 2] = [
    &[
        (b'K', Rgb(36, 32, 32)),
        (b'k', Rgb(20, 18, 18)),
        (b'W', Rgb(226, 220, 206)),
        (b'T', Rgb(184, 156, 128)),
        (b'e', Rgb(8, 8, 8)),
        (b'c', Rgb(200, 194, 180)),
    ],
    &[
        (b'K', Rgb(104, 72, 46)),
        (b'k', Rgb(70, 48, 32)),
        (b'W', Rgb(110, 76, 50)),
        (b'T', Rgb(140, 104, 72)),
        (b'e', Rgb(10, 8, 8)),
        (b'c', Rgb(60, 44, 34)),
    ],
];

/// A dhole: a lean red wild dog. `R` coat, `r` its shade, `W` throat and
/// belly, `k` eye and nose, `K` ears' edges.
pub const DHOLE: Sprite = &[
    "................RK..",
    "...............RRRR.",
    "...............RRkRk",
    "..rrrrrrrrrrrrRRRWW.",
    ".RRRRRRRRRRRRRRRW...",
    ".RRRRRRRRRRRRRRWW...",
    "..RRRWWWWWWWRRRW....",
];

pub const DOG: Build = Build {
    hind: LegSpec {
        hip: (3.5, 5.0),
        upper: 3.5,
        lower: 3.5,
        w_up: 2.4,
        w_lo: 1.4,
        fore: false,
    },
    fore: LegSpec {
        hip: (14.0, 5.5),
        upper: 3.5,
        lower: 3.0,
        w_up: 2.0,
        w_lo: 1.4,
        fore: true,
    },
    far_dx: 1.5,
    ground: 12.0,
    stride: 5.0,
    lift: 2.0,
    leg: (b'R', b'r'),
    stripe: None,
    paw: (b'R', b'r'),
    paw_w: 2,
    tail: Some(TailSpec {
        root: (1.0, 3.5),
        links: 8,
        w0: 2.4,
        w1: 2.0,
        coat: b'R',
        ring: None,
        tip: b'K',
        tip_len: 3,
    }),
};

/// Dholes in the jungle, wolves in the taiga.
pub const DOG_COLORS: [Palette; 2] = [
    &[
        (b'R', Rgb(190, 94, 42)),
        (b'r', Rgb(138, 64, 30)),
        (b'W', Rgb(238, 220, 190)),
        (b'K', Rgb(32, 24, 20)),
        (b'k', Rgb(22, 16, 14)),
    ],
    &[
        (b'R', Rgb(150, 146, 140)),
        (b'r', Rgb(100, 98, 96)),
        (b'W', Rgb(226, 224, 218)),
        (b'K', Rgb(44, 42, 42)),
        (b'k', Rgb(22, 20, 20)),
    ],
];

// ----- the visitors -----

/// The saucer, its rim lights in two phases. `M` hull, `m` its shade, `D`
/// the dome, `d` its glint, `L` a lit light, `l` an unlit one, `R` the
/// emitter underneath.
pub const UFO: [Sprite; 2] = [
    &[
        "...........dDDDD...........",
        ".........dDDDDDDDD.........",
        "........DDDDDDDDDDD........",
        "......mmMMMMMMMMMMMmm......",
        "...mmMMMMMMMMMMMMMMMMMmm...",
        ".mMMLMMMlMMMLMMMlMMMLMMMm..",
        "mMMMMMMMMMMMMMMMMMMMMMMMMm.",
        ".mmmmmmmmmmmmmmmmmmmmmmmm..",
        "......mmmmmRRRRRmmmmm......",
        "..........RRRRRRR..........",
    ],
    &[
        "...........dDDDD...........",
        ".........dDDDDDDDD.........",
        "........DDDDDDDDDDD........",
        "......mmMMMMMMMMMMMmm......",
        "...mmMMMMMMMMMMMMMMMMMmm...",
        ".mMMlMMMLMMMlMMMLMMMlMMMm..",
        "mMMMMMMMMMMMMMMMMMMMMMMMMm.",
        ".mmmmmmmmmmmmmmmmmmmmmmmm..",
        "......mmmmmRRRRRmmmmm......",
        "..........RRRRRRR..........",
    ],
];

pub const UFO_COLORS: Palette = &[
    (b'M', Rgb(186, 196, 206)),
    (b'm', Rgb(110, 120, 136)),
    (b'D', Rgb(112, 222, 204)),
    (b'd', Rgb(222, 255, 250)),
    (b'L', Rgb(255, 238, 120)),
    (b'l', Rgb(120, 104, 56)),
    (b'R', Rgb(150, 255, 170)),
];

/// Every sprite with its palette, for the tests.
#[cfg(test)]
pub fn all() -> Vec<(&'static str, Sprite, Palette)> {
    let mut v: Vec<(&'static str, Sprite, Palette)> = Vec::new();
    for p in TIGER_COLORS {
        v.push(("torso", TIGER_TORSO, p));
        v.push(("head", TIGER_HEAD, p));
        v.push(("blink", TIGER_HEAD_BLINK, p));
        v.push(("snarl", TIGER_HEAD_SNARL, p));
        v.push(("roar", TIGER_HEAD_ROAR, p));
        v.push(("front", TIGER_FRONT, p));
        for s in TIGER_SLEEP {
            v.push(("sleep", s, p));
        }
    }
    for p in [CHITAL_COLORS, SAMBAR_COLORS, RED_DEER_COLORS] {
        v.push(("chital", CHITAL, p));
        v.push(("graze", CHITAL_GRAZE, p));
    }
    v.push(("boar", BOAR, BOAR_COLORS));
    for s in PEACOCK {
        v.push(("peacock", s, PEACOCK_COLORS));
    }
    for s in LANGUR {
        v.push(("langur", s, LANGUR_COLORS));
    }
    for s in BIRD {
        for p in BIRD_COLORS {
            v.push(("bird", s, p));
        }
    }
    v.push(("croc", CROC_LURK, CROC_COLORS));
    v.push(("lunge", CROC_LUNGE, CROC_COLORS));
    for p in BEAR_COLORS {
        v.push(("bear", BEAR, p));
    }
    for p in DOG_COLORS {
        v.push(("dhole", DHOLE, p));
    }
    for s in UFO {
        v.push(("ufo", s, UFO_COLORS));
    }
    v
}
