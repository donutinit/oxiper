//! The shark scene's sprites, drawn by hand one character per pixel.
//!
//! Every sprite faces right. `.` is see-through and any other character is a
//! palette entry. The sharks share their letters so that their parts line up
//! and each kind maps them to its own colors:
//!
//! - `D` dark back and fin edges, `B` back, `L` sheen, `R` flank, `G` gill slits;
//! - `W` belly, `w` belly in shade;
//! - `F` fins, `f` fin trailing edges;
//! - `e` eye, `h` its highlight, `n` nostril and mouth line;
//! - `m` mouth, `M` gums, `t` teeth.

use super::Rgb;

pub type Sprite = &'static [&'static str];

// ----- great white -----

/// The great white side on, mouth shut, tail at rest.
const GW_SIDE: Sprite = &[
    "...............................ff...............................",
    "...............................fFD..............................",
    "...............................fFBD.............................",
    "f..............................fFFBD............................",
    "fD.............................fFFFBD...........................",
    ".fD............................fFFFFBD..........................",
    ".fBD..........................fFFFFFFBD.........................",
    "..fBD........................fFFFFFFFBBDDDDDDDDDDD..............",
    "..fFBD....................DDDBBBBBBBBBBBBBBBBBBBBBDDD...........",
    "...fFBDD..............DDDDBBBBBBBBBBBBBBBBBBBBBBBBBBBDDD........",
    "...fFFBBD.......fD.DDDBBBBBBBRRLLLLLLLLLLLLLLLLLLRBBBBBBDDD.....",
    "....fFFFBDD.....DDDBBBBBBBRRRRRRRRRRRRRRRRRRRRRRRRLLLBBehBBDD...",
    ".....fFFFFBBDDDDBBBBBBRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRLLeeBBBBDD.",
    "......fFFFFBBBBBBBBRRRRRRRRRRRRRRRRRRRRRRGRGRGRGRGRRRRRRRRRBBBBD",
    "......fFFFFFWWRRRRRRRRRRRRRRRRRRRRRRRRRRRGRGRGRGRGRRRRRRRRRRRnRW",
    ".....fFFFFFFwwWWWWRRRRRRRRRRRRRRRRRRRRRRRGRGRGRGRGRRRRRRRRRnnWW.",
    "....fFFFFFf...wwwWWWWWWWRRRRRRRRRRRRRRRRRGRGRGRGRGRRnRRRRnnWWW..",
    "....fFFFFf.......wwwWWWWWWWWWWRRRRWWRRRRRwWwWwWwWwWWWnnnnWWWw...",
    "...fFFFf............wwwwWWWWWWWWWWWWWWWWfFFFFFFBWWWWWWWWWwww....",
    "...fFFf.................wwwwWWWWWWWWWWWfFFFFFFBWWWWwwwwww.......",
    "..fFFf................fFFFFFwwwwwWWWWWfFFFFFBwwwwww.............",
    "..fFf................fFFf........wwwwfFFFFFB....................",
    ".ff.................................fFFFFB......................",
    "f..................................fFFFFB.......................",
    "..................................fFFFB.........................",
    ".................................ffFB...........................",
    "................................fff.............................",
    "...............................ff...............................",
];

/// The caudal fin swung out of the picture plane, more and more
/// foreshortened; each replaces the first [`TAIL_W`] columns of the side view.
const GW_TAIL_1: Sprite = &[
    "............",
    "............",
    "............",
    "...f........",
    "...fD.......",
    "....fD......",
    "....fBD.....",
    ".....fBD....",
    ".....fFBD...",
    ".....fFBDD..",
    "......fFBBD.",
    "......fFFBDD",
    ".......fFFBB",
    ".......fFFFB",
    "........fFFF",
    ".......fFFFF",
    "......fFFFf.",
    "......fFFf..",
    ".....fFFf...",
    ".....fFf....",
    ".....fFf....",
    "....fFf.....",
    "....ff......",
    "...f........",
    "............",
    "............",
    "............",
    "............",
];

const GW_TAIL_2: Sprite = &[
    "............",
    "............",
    "............",
    "......f.....",
    "......fD....",
    "......fD....",
    ".......fD...",
    ".......fD...",
    ".......fBD..",
    ".......fBD..",
    "........fBD.",
    "........fBDD",
    "........fFBB",
    ".........fFB",
    ".........fFF",
    "........fFFF",
    "........fFf.",
    "........fFf.",
    ".......fFf..",
    ".......fF...",
    ".......ff...",
    ".......f....",
    "......ff....",
    "......f.....",
    "............",
    "............",
    "............",
    "............",
];

const GW_TAIL_3: Sprite = &[
    "............",
    "............",
    "............",
    "........f...",
    "........fD..",
    "........fD..",
    "........fBD.",
    "........fBD.",
    ".........fD.",
    ".........fBD",
    ".........fBD",
    ".........fFD",
    "..........fB",
    "..........fB",
    "..........fF",
    ".........fFF",
    ".........fFf",
    ".........fFf",
    ".........fF.",
    "........fFf.",
    "........fF..",
    "........ff..",
    "........f...",
    "........f...",
    "............",
    "............",
    "............",
    "............",
];

/// The mouth ajar, teeth showing, then gaping wide with the eye rolled back
/// as great whites do when they bite; each replaces the side view's columns
/// from `head_x` on.
const GW_AJAR: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "DDD...........",
    "BBBDDD........",
    "BBBBBBDDD.....",
    "LLLBBehBBDD...",
    "RRRLLeeBBBBDD.",
    "RRRRRRRRRBBBBD",
    "RRRRRRRRRRRnRW",
    "RRRRRRRRRMMWW.",
    "RRMMMMMtMtmW..",
    "WMmtmtmmmmM...",
    "WWMtMtMtMWW...",
    "WWWWWWWWww....",
    "wwwwwwww......",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
];

const GW_GAPE: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "DDDD..........",
    "BBBBDDDDD.....",
    "BBBBBBBBBDD...",
    "LLLBBwwBBBBDD.",
    "RRRLLwwBBBBBBD",
    "RRRRRRRRRRRRnW",
    "RRRRRRRRMMMWW.",
    "RRRRRMMMtMtM..",
    "RRMMMtmtmmm...",
    "WMmmmmmmmm....",
    "WWMtmmmmm.....",
    "WWWMtmtm......",
    "wWWWMtmtt.....",
    ".wwWWMMtMt....",
    "...wwWWMMMM...",
    ".....wwWWWWw..",
    "......wwwww...",
    "..............",
    "..............",
    "..............",
];

/// Halfway round in a turn: three-quarters towards the viewer.
const GW_QUARTER: Sprite = &[
    "....................f.......................",
    "....................fD......................",
    "....................fFD.....................",
    "....................fFBD....................",
    "....................fFFBD...................",
    "f...................fFFFBD..................",
    "fD.................fFFFFBD..................",
    ".fD...............fFFFFFBBDDDDDDDD..........",
    ".fBD..............BBBBBBBBBBBBBBBBDD........",
    "..fBD...........DDBBBBBBBBBBBBBBBBBBBDD.....",
    "..fFBD........DDBBBBBLLLLLLLLLLLLLBBBBBDD...",
    "...fFBD.....DDBBBBRRRRRRRRRRRRRRRRLLLehBBD..",
    "....fFFBDDDDBBBBRRRRRRRRRRRRRRRRRRRRReeBBBD.",
    "....fFFFBBBBRRRRRRRRRRRRRRRRRGRGRGRGRRRRRBBD",
    "....fFFFWWWWRRRRRRRRRRRRRRRRRGRGRGRGRRRRRnRW",
    "...fFFFFwwwwWWWRRRRRRRRRRRRRRGRGRGRGRRRRnnWW",
    "...fFFf.....wwWWWWRRRRRRRRRRRGRGRGRGRnnnWWWw",
    "..fFFFf.......wWWWWWWWRRRRRRRGRGRGRGnWWWWWWw",
    "..fFFf..........wwWWWWWWWWWWWwWwWwWwWWWWWWw.",
    ".fFFf.............wwWWWWWWWWWfFFFFBWWWWWWw..",
    ".fFf................wwwWWWWWfFFFFBWWWWWwwFf.",
    ".ff....................wwwwfFFFFBWWWwwwFFff.",
    "ff........................fFFFBwwwww....Fff.",
    ".........................fFFFB...........ff.",
    "........................fFFB..............f.",
    ".......................fFFB.................",
    ".......................ffB..................",
    "......................ff....................",
];

/// Facing the viewer, in the middle of a turn.
const GW_FRONT: Sprite = &[
    ".................D.................",
    ".................D.................",
    "................fD.................",
    "................fD.................",
    "................fDD................",
    "................fDD................",
    "...............ffDBD...............",
    ".............DDBBBBBDD.............",
    "...........DDBBBBBBBBBDD...........",
    "..........DBBBBBBBBBBBBBD..........",
    ".........DBBBBLLLLLLLBBBBD.........",
    ".........DehRRRLLLLLRRRheD.........",
    ".........DeeRRRRLLLRRRReeD.........",
    ".........DRRRRRRRLRRRRRRRD.........",
    ".........DRRRRRnRRRnRRRRRD.........",
    ".........RRnRRRRRRRRRRRnRR.........",
    ".........RRRnnnnnnnnnnnRRR.........",
    ".....BFFFRWWWWWWWWWWWWWWWRFFFB.....",
    "...BFFFFFfwWWWWWWWWWWWWWwfFFFFFB...",
    ".BFFFFf....wWWWWWWWWWWWw....fFFFFB.",
    "BFFf........wwWWWWWWWww........fFFB",
    "ff...........fwwwwwwwf...........ff",
    ".................f.................",
    ".................f.................",
    ".................f.................",
    "...................................",
    "...................................",
    "...................................",
];

const GW_PALETTE: &[(u8, Rgb)] = &[
    (b'D', Rgb(74, 86, 100)),
    (b'B', Rgb(96, 110, 124)),
    (b'L', Rgb(140, 152, 164)),
    (b'R', Rgb(126, 138, 150)),
    (b'G', Rgb(104, 116, 130)),
    (b'W', Rgb(236, 240, 242)),
    (b'w', Rgb(192, 200, 208)),
    (b'F', Rgb(88, 100, 114)),
    (b'f', Rgb(58, 68, 80)),
    (b'e', Rgb(8, 10, 14)),
    (b'h', Rgb(240, 248, 255)),
    (b'n', Rgb(70, 76, 84)),
    (b'm', Rgb(54, 28, 34)),
    (b'M', Rgb(168, 74, 86)),
    (b't', Rgb(250, 248, 236)),
];

/// Columns of the side view that the tail frames replace.
pub const TAIL_W: usize = 12;

/// A kind of shark: its sprites, colors and manners.
pub struct Rig {
    /// Side on, mouth shut, tail at rest.
    pub side: Sprite,
    /// The caudal fin swung further and further out of the picture plane.
    pub tails: [Sprite; 3],
    /// The mouth opening, from `head_x` on.
    pub heads: [Sprite; 2],
    pub head_x: usize,
    /// Turning: three-quarters towards the viewer, then facing it.
    pub quarter: Sprite,
    pub front: Sprite,
    pub palette: &'static [(u8, Rgb)],
    /// Row of the body's axis, about which the shark pitches.
    pub axis: i32,
    /// Where the jaws meet in the side view.
    pub mouth: (i32, i32),
    /// Where riders hold on under the belly, in the side view.
    pub belly: [(i32, i32); 2],
    /// Pace when cruising, in world pixels per step, and how fast it can burst.
    pub cruise: f32,
    pub burst: f32,
    /// How briskly its tail beats, 1 being a great white's.
    pub beat: f32,
}

pub const GREAT_WHITE: Rig = Rig {
    side: GW_SIDE,
    tails: [GW_TAIL_1, GW_TAIL_2, GW_TAIL_3],
    heads: [GW_AJAR, GW_GAPE],
    head_x: 50,
    quarter: GW_QUARTER,
    front: GW_FRONT,
    palette: GW_PALETTE,
    axis: 14,
    mouth: (60, 17),
    belly: [(29, 18), (20, 16)],
    cruise: 0.9,
    beat: 1.0,
    burst: 3.4,
};

// ----- tiger shark -----
//
// Blunt square snout, a big eye, dark bars (`S`, fading to `T` towards the
// head) and a long upper tail lobe.

const TG_SIDE: Sprite = &[
    "................................................................",
    "f............................ff.................................",
    "fD...........................fFD................................",
    ".fD..........................fFBD...............................",
    ".fBD.........................fFFBD..............................",
    "..fBD........................fFFFBD.............................",
    "..fFBD......................fFFFFFBD............................",
    "...fFBD....................fFFFFFFFBD...........................",
    "...fFFBD...................fFFFFFFFFFDDDDDDDDDDDDDDDD...........",
    "....fFFBD...............DDDBBBBBBBBBBBBBBBBBBBBBBBBBBDDDD.......",
    ".....fFFBD......fD..DDDDBBBSBBSBBBSBBBTBBBTBBBBBBBBBBBBBBDDD....",
    "......fFFBDD....DDDDBBBSBBBSBBBBBBSBBBTLLLTLLLTLLLLLLBBehBBBDDD.",
    ".......fFFBBDDDDSBBSBBBSRRSRRRSRRSRRRRRRRRRRRRRRRRRRRLLeeBBBBBBD",
    "........fFFBBSBBSRRRRRSRRRSRRRSRRSRRRRRRRGRGRGRGRGRRRRRRRRRRRRnB",
    "........fFFFWWWWRRRSRRSRRRSRRRSRRRRRRRRRRGRGRGRGRGRRRRRRRRRRRRRW",
    ".......fFFFFwwwwWWWWWWRRRRRRRRRRRRRRRRRRRGRGRGRGRGRRRRRRRnnnnRWw",
    "......fFFFf.....wwwwWWWWWWWWWWWRRRRRRRRRRGRGRGRGRGRRnnnnnWWWWWw.",
    ".....fFFFf......fF..wwwwWWWWWWWWWWWWWWWWWwWwWwWwWwWnWWWWWWwwww..",
    "....fFFf............fFFFwwwwWWWWWWWWWWWWWfFFFFBWWWWWWwwwww......",
    "....fFf............ffF......wwwwwwWWWWWWfFFFFFBwwwwww...........",
    "...fFf............................wwwwwfFFFFFFB.................",
    "...ff.................................fFFFFB....................",
    ".....................................fFFFFB.....................",
    "....................................fFFFB.......................",
    "...................................ffFB.........................",
    "..................................fff...........................",
    "................................................................",
    "................................................................",
];

const TG_TAIL_1: Sprite = &[
    "............",
    "...f........",
    "...fD.......",
    "....fD......",
    "....fBD.....",
    ".....fBD....",
    ".....fFBD...",
    "......fFBD..",
    "......fFBD..",
    ".......fFBD.",
    ".......fFBDD",
    "........fFBB",
    "........fFBB",
    ".........fFB",
    ".........fFF",
    "........fFFF",
    "........fFf.",
    ".......fFFf.",
    ".......fFf..",
    "......fFf...",
    "......ff....",
    "......f.....",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
];

const TG_TAIL_2: Sprite = &[
    "............",
    "......f.....",
    "......fD....",
    "......fD....",
    ".......fD...",
    ".......fBD..",
    ".......fBD..",
    "........fBD.",
    "........fBD.",
    "........fFBD",
    ".........fBD",
    ".........fBB",
    ".........fFB",
    "..........fB",
    "..........fF",
    ".........fFF",
    ".........fF.",
    ".........fF.",
    "........fF..",
    "........ff..",
    "........f...",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
];

const TG_TAIL_3: Sprite = &[
    "............",
    "........f...",
    "........fD..",
    "........fD..",
    "........fD..",
    ".........fD.",
    ".........fD.",
    ".........fD.",
    ".........fBD",
    ".........fBD",
    "..........fD",
    "..........fB",
    "..........fB",
    "..........fB",
    "..........fF",
    ".........fFF",
    ".........fF.",
    ".........fF.",
    ".........ff.",
    ".........f..",
    ".........f..",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
];

/// Its teeth are notched like cockscombs, and when it bites a membrane
/// slides over the eye.
const TG_AJAR: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "DDD...........",
    "BBBDDDD.......",
    "BBBBBBBDDD....",
    "LLLBBehBBBDDD.",
    "RRRLLeeBBBBBBD",
    "RRRRRRRRRRRRnB",
    "RRRRRRRRRRRRRW",
    "RRRRRRRMMMMRWw",
    "RRMtMtMtMtMWw.",
    "WnmmmmmmmmMw..",
    "WWMtMtMtMWw...",
    "WWWWWWWWww....",
    "wwwwwww.......",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
];

const TG_GAPE: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "DDDD..........",
    "BBBBDDDDD.....",
    "BBBBBBBBBDDD..",
    "LLLBBwwBBBBBDD",
    "RRRLLwwBBBBBBB",
    "RRRRRRRRRRRRnW",
    "RRRRRRRRMMMMWw",
    "RRRRRMMMtMtMt.",
    "RRMMMtmtmmmm..",
    "WMmmmmmmmmm...",
    "WWMtmmmmmm....",
    "WWWMtmmmmt....",
    "wWWWMtmtmtt...",
    ".wwWWMMtMtM...",
    "...wwWWMMMMM..",
    ".....wwWWWWWw.",
    "......wwwwww..",
    "..............",
    "..............",
    "..............",
];

const TG_QUARTER: Sprite = &[
    "....................f.......................",
    "....................fD......................",
    "....................fFD.....................",
    "....................fFBD....................",
    "....................fFFBD...................",
    "f...................fFFFBD..................",
    "fD.................fFFFFBD..................",
    ".fD...............fFFFFFBBDDDDDDDD..........",
    ".fBD..............BBBBBBBBBBBBBBBBDD........",
    "..fBD...........DDBBBBBBBBBBBBBBBBBBBDD.....",
    "..fFBD........DDBSBBBSLLLSLLLTLLLLBBBBBDD...",
    "...fFBD.....DDBSBBRRSRRRSRRRSRRRTRLLLehBBD..",
    "....fFFBDDDDBSBBRRRSRRRSRRRSRRRRRRRRReeBBBD.",
    "....fFFFBBSBRRSRRRSRRRSRRRTRRGRGRGRGRRRRRBBD",
    "....fFFFWWWWRRRRRSRRRSRRRRRRRGRGRGRGRRRRRnRW",
    "...fFFFFwwwwWWWRRRRRRRRRRRRRRGRGRGRGRRRRnnWW",
    "...fFFf.....wwWWWWRRRRRRRRRRRGRGRGRGRnnnWWWw",
    "..fFFFf.......wWWWWWWWRRRRRRRGRGRGRGnWWWWWWw",
    "..fFFf..........wwWWWWWWWWWWWwWwWwWwWWWWWWw.",
    ".fFFf.............wwWWWWWWWWWfFFFFBWWWWWWw..",
    ".fFf................wwwWWWWWfFFFFBWWWWWwwFf.",
    ".ff....................wwwwfFFFFBWWWwwwFFff.",
    "ff........................fFFFBwwwww....Fff.",
    ".........................fFFFB...........ff.",
    "........................fFFB..............f.",
    ".......................fFFB.................",
    ".......................ffB..................",
    "......................ff....................",
];

const TG_FRONT: Sprite = &[
    ".................D.................",
    ".................D.................",
    "................fD.................",
    "................fD.................",
    "................fDD................",
    "................fDD................",
    "...............ffDBD...............",
    ".............DDBBBBBDD.............",
    "...........DDBSBBBBBSBDD...........",
    "..........DBBSBBBBBBBSBBD..........",
    ".........DBBBBLLLLLLLBBBBD.........",
    ".........DehRRRLLLLLRRRheD.........",
    ".........DeeRRRRLLLRRRReeD.........",
    ".........DRRRRRRRLRRRRRRRD.........",
    ".........DRRRRRnRRRnRRRRRD.........",
    ".........RnRRRRRRRRRRRRRnR.........",
    ".........RRnnnnnnnnnnnnnRR.........",
    ".....BFFFRWWWWWWWWWWWWWWWRFFFB.....",
    "...BFFFFFfwWWWWWWWWWWWWWwfFFFFFB...",
    ".BFFFFf....wWWWWWWWWWWWw....fFFFFB.",
    "BFFf........wwWWWWWWWww........fFFB",
    "ff...........fwwwwwwwf...........ff",
    ".................f.................",
    ".................f.................",
    ".................f.................",
    "...................................",
    "...................................",
    "...................................",
];

const TG_PALETTE: &[(u8, Rgb)] = &[
    (b'D', Rgb(70, 78, 76)),
    (b'B', Rgb(102, 110, 104)),
    (b'L', Rgb(148, 154, 144)),
    (b'R', Rgb(128, 134, 124)),
    (b'G', Rgb(98, 104, 98)),
    (b'S', Rgb(60, 64, 60)),
    (b'T', Rgb(92, 98, 92)),
    (b'W', Rgb(234, 230, 212)),
    (b'w', Rgb(194, 190, 174)),
    (b'F', Rgb(88, 96, 92)),
    (b'f', Rgb(58, 64, 62)),
    (b'e', Rgb(6, 8, 8)),
    (b'h', Rgb(230, 240, 240)),
    (b'n', Rgb(70, 72, 70)),
    (b'm', Rgb(54, 28, 34)),
    (b'M', Rgb(168, 78, 86)),
    (b't', Rgb(250, 246, 230)),
];

pub const TIGER: Rig = Rig {
    side: TG_SIDE,
    tails: [TG_TAIL_1, TG_TAIL_2, TG_TAIL_3],
    heads: [TG_AJAR, TG_GAPE],
    head_x: 50,
    quarter: TG_QUARTER,
    front: TG_FRONT,
    palette: TG_PALETTE,
    axis: 14,
    mouth: (61, 16),
    belly: [(29, 17), (21, 16)],
    cruise: 0.8,
    beat: 1.0,
    burst: 3.2,
};

// ----- hammerhead -----
//
// A very tall sickle of a dorsal fin, a slim body, a long upper tail lobe
// with a notch, and the hammer: drawn across the head as side-view pictures
// of hammerheads do, an eye at either tip, so it reads at a glance.

const HH_SIDE: Sprite = &[
    "...............................ff...............................",
    "...............................fFD..............................",
    "f..............................fFBD.............................",
    "fD.............................fFFBD............................",
    ".fD............................fFFFBD...........................",
    ".fBD...........................fFFFFBD..........................",
    "..fBD.........................ffFFFFFBD.........................",
    "..fFBD........................fFFFFFFFBD...................fDD..",
    "...fFBD......................ffFFFFFFFFBD.................fBeD..",
    "...fFFBD......................fFFFFFFFFFBB................fBBBD.",
    "....ffFBD...................DDBBBBBBBBBBBBDDDDDDD........fBBBBD.",
    ".....fFFBD..............DDDDBBBBBBBBBBBBBBBBBBBBBD.......fBBBBD.",
    "......fFFBD.......fDDDDDBBBBBBBBLLLLLLLLLLLLLLLBBBDD.....fBBBBD.",
    ".......fFFBD....DDDDBBBBBBBBRRRRRRRRRRRRRRRRRRRRRRBBDDDDDBBBBBD.",
    "........fFFBDDDDBBBBRRRRRRRRRRRRRRRRRRRRRGRGRGRGRRRRBBBBBRRRRRD.",
    "........fFFFBBBBRRRRRRRRRRRRRRRRRRRRRRRRRGRGRGRGRRRRRRRRRRRRRRD.",
    ".......fFFFFWWWWWWWWRRRRRRRRRRRRRRRRRRRRRGRGRGRGRRRRRRRRRRRRRRD.",
    "......fFFFf.wwwwWWWWWWWWWWWWWWWWWWWWWWWWWwWwWwWwWWWWnnnnWRRRRRD.",
    ".....fFFFf......wwwwWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWwwwwwwBBBBBD.",
    ".....fFf.........fF.wwwwwwwwWWWWWWWWWWWWfFFFBWWWWww......fBBBBD.",
    "....fFf.................fFFFwwwwwwwwwwwfFFFFBwwww........fBBBBD.",
    "....ff.................ffF............fFFFB...............fBBBD.",
    ".....................................fFFFB................fBeD..",
    "....................................fFFB...................fDD..",
    "...................................ffB..........................",
    "..................................ff............................",
    "................................................................",
    "................................................................",
];

const HH_TAIL_1: Sprite = &[
    "............",
    "............",
    "...f........",
    "...fD.......",
    "....fD......",
    "....fBD.....",
    ".....fBD....",
    ".....fFBD...",
    "......fFBD..",
    "......fFBD..",
    ".......ffBD.",
    ".......fFBD.",
    "........fFBD",
    "........fFBD",
    ".........fFB",
    ".........fFF",
    "........fFFF",
    "........fFf.",
    ".......fFf..",
    ".......fF...",
    "......fF....",
    "......f.....",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
];

const HH_TAIL_2: Sprite = &[
    "............",
    "............",
    "......f.....",
    "......fD....",
    ".......fD...",
    ".......fD...",
    ".......fBD..",
    "........fBD.",
    "........fBD.",
    "........fBD.",
    "........ffBD",
    ".........fBD",
    ".........fBD",
    ".........fFB",
    "..........fB",
    "..........fF",
    ".........fFF",
    ".........fF.",
    ".........fF.",
    "........fF..",
    "........f...",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
];

const HH_TAIL_3: Sprite = &[
    "............",
    "............",
    "........f...",
    "........fD..",
    "........fD..",
    ".........fD.",
    ".........fD.",
    ".........fD.",
    ".........fD.",
    ".........fBD",
    ".........fBD",
    "..........fD",
    "..........fD",
    "..........fB",
    "..........fB",
    "..........fF",
    ".........fFF",
    ".........fF.",
    ".........fF.",
    ".........f..",
    ".........f..",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
];

/// The mouth is small and well back under the hammer.
const HH_AJAR: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    ".........fDD..",
    "........fBeD..",
    "........fBBBD.",
    ".......fBBBBD.",
    ".......fBBBBD.",
    "DD.....fBBBBD.",
    "BBDDDDDBBBBBD.",
    "RRBBBBBRRRRRD.",
    "RRRRRRRRRRRRD.",
    "RRRRRRRRRRRRD.",
    "WMtMtMWRRRRRD.",
    "WmmmmmwBBBBBD.",
    "wMtMtM.fBBBBD.",
    ".wwwww.fBBBBD.",
    "........fBBBD.",
    "........fBeD..",
    ".........fDD..",
    "..............",
    "..............",
    "..............",
    "..............",
];

const HH_GAPE: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    ".........fDD..",
    "........fBeD..",
    "........fBBBD.",
    ".......fBBBBD.",
    ".......fBBBBD.",
    "DD.....fBBBBD.",
    "BBDDDDDBBBBBD.",
    "RRBBBBBRRRRRD.",
    "RRRRRRRRRRRRD.",
    "RRRRRRRRRRRRD.",
    "WMMMMMWRRRRRD.",
    "MtmtmtMBBBBBD.",
    "MmmmmmmfBBBBD.",
    "MtmtmtMfBBBBD.",
    "WMMMMMw.fBBBD.",
    ".wwwww..fBeD..",
    ".........fDD..",
    "..............",
    "..............",
    "..............",
    "..............",
];

/// The hammer swings round from across the head to a bar facing the viewer.
const HH_QUARTER: Sprite = &[
    "..................ff........................",
    "..................fFD.......................",
    "..................fFBD......................",
    "f.................fFFBD.....................",
    "fD................fFFFBD....................",
    ".fD...............fFFFFBD...................",
    ".fBD.............ffFFFFFBD..................",
    "..fBD............fFFFFFFFBD......fD.........",
    "..fFBD..........ffFFFFFFFFBD.....eBD........",
    "...fFBD.........fFFFFFFFFFBB.....fBBD.......",
    "...fFFBD............DDBBBBBBBBBBDDfBBD......",
    "....ffFBD.......DDDDBBBBBBBBBBBBBBBBBBD.....",
    ".....fFFBDDDDDDDBBBBLLLLLLLLLLLLBBBBBBBD....",
    "......fFFBBBBBBBRRRRRRRRRRGRGRGRRRRRRRRRD...",
    "......fFFFWWWWWWRRRRRRRRRRGRGRGRRRRRRRRRD...",
    ".....fFFFFwwwwwwWWWWRRRRRRGRGRGRRRRRRRRRRD..",
    "....fFFFf.......wwwwWWWWWWwWwWwWWWnnnRRRRD..",
    "...fFFf.............wwwwWWWWWWWWWWWWWRRRRRD.",
    "...fFf..............fFF.wwwwwwwfFFFBwwBBBBD.",
    "..fFf..................fFFFFFFFFFFB.....fBBD",
    "..ff..................fFFFFFFB..........fBBD",
    "......................fFFFFB.............fBD",
    ".....................ffFFB...............eBD",
    ".....................ffB..................fD",
    "............................................",
    "............................................",
    "............................................",
    "............................................",
];

const HH_FRONT: Sprite = &[
    ".................D.................",
    ".................D.................",
    "................fD.................",
    "................fD.................",
    "................fD.................",
    "................fDD................",
    "................fDD................",
    "...............ffDD................",
    "...............fFDBD...............",
    "..............ffFDBDD..............",
    ".............DDBBBBBDD.............",
    "............DBBBBBBBBBD............",
    "...........DBBBBLLLBBBBD...........",
    "....DDDDDDDDBBBBLLLBBBBDDDDDDDD....",
    "..DeBBBBBBBBRRRRRRRRRRRBBBBBBBBeD..",
    "..DRRRRRnRRRRRRRRRRRRRRRRRRnRRRRD..",
    "....wwwwwwwwWWWWWWWWWWWwwwwwwww....",
    "...........RRnnnnnnnnnRR...........",
    "......BFFFFfWWWWWWWWWWWfFFFFB......",
    "...BFFFf.....wWWWWWWWw.....fFFFB...",
    ".BFf..........wwwwwww..........fFB.",
    "..............f.....f..............",
    ".................f.................",
    ".................f.................",
    "...................................",
    "...................................",
    "...................................",
    "...................................",
];

const HH_PALETTE: &[(u8, Rgb)] = &[
    (b'D', Rgb(80, 86, 80)),
    (b'B', Rgb(112, 116, 104)),
    (b'L', Rgb(152, 156, 142)),
    (b'R', Rgb(132, 136, 124)),
    (b'G', Rgb(110, 114, 104)),
    (b'W', Rgb(236, 236, 226)),
    (b'w', Rgb(196, 196, 186)),
    (b'F', Rgb(100, 106, 96)),
    (b'f', Rgb(66, 72, 66)),
    (b'e', Rgb(8, 8, 8)),
    (b'h', Rgb(230, 236, 236)),
    (b'n', Rgb(82, 84, 78)),
    (b'm', Rgb(54, 28, 34)),
    (b'M', Rgb(168, 78, 86)),
    (b't', Rgb(250, 246, 232)),
];

pub const HAMMERHEAD: Rig = Rig {
    side: HH_SIDE,
    tails: [HH_TAIL_1, HH_TAIL_2, HH_TAIL_3],
    heads: [HH_AJAR, HH_GAPE],
    head_x: 50,
    quarter: HH_QUARTER,
    front: HH_FRONT,
    palette: HH_PALETTE,
    axis: 15,
    mouth: (54, 18),
    belly: [(29, 18), (21, 17)],
    cruise: 0.85,
    beat: 1.1,
    burst: 3.3,
};

// ----- whale shark -----
//
// The biggest fish in the sea: a broad flat head with the mouth right at
// the front, small eyes low behind it, ridges along the flanks, and a dark
// skin spotted pale (`o`). It filter feeds, mouth wide and gill rakers (`k`)
// showing, and has no use for teeth.

const WS_SIDE: Sprite = &[
    "................................................................",
    "f.......................ff......................................",
    "fD......................fFD.....................................",
    ".fD.....................fFBD....................................",
    ".fBD...................ffFFBD...................................",
    "..fBD..................fFFFFBD..................................",
    "..fFBD................ffFFFFFBD.................................",
    "...fFBD...............fFFFFFFFBD................................",
    "...fFFBD.............fFFFFFFFFFFDDDDDDDDDDDDDDDDDDDDDDDDDDD.....",
    "....fFFBD............BBBBBBBBBBBBoBBBoBBBoBBBoBBBoBBBoBBBoBDDD..",
    "....fFFFBD..........DBBBBBBoBBBBBBBoBBBoBBBoBBBoBBBoBBBoBBBBoBD.",
    ".....fFFFBD.....DDDDLLLLLLLLLLLLLLLLLLLLLLLLGBGBGBGBGBBoBBBBBBD.",
    "......fFFFBDDDDDBBBBRRoRRRoRRRoRRRoRRRoRRRoRGRGRGRGRGRoRRoRRRoRB",
    ".......fFFFFBBBBRRRRRRRRoRRRoRRRoRRRoRRRoRRRGRGRGRGRGRRRRRRRoRRR",
    ".......fFFFFRRRRLLLLLLLLLLLLLLLLLLLLLLLLLLLLGRGRGRGRGRRRRRRRRRRR",
    "......fFFFFFWWWWRRRRRRoRRRoRRRoRRRoRRRoRRRoRGRGRGRGRGRRRReRRRRRR",
    ".....fFFFFFfwwwwWWWWWWWWWRRRRRRRRRRRRRRRRRRRGRGRGRGRGRRnnnnnnnnn",
    ".....fFFFFf.....wwwwWWWWWWWWWWWWWWWWWWWWWWWWwWwWwWwWwWWWWWWWWWWw",
    "....fFFFFf..........wwwwwWWWWWWWWWWWWWWWWWWWwWwWwWwWwWWWWWWWWWw.",
    "....fFFFf............fFFFwwwwwwWWWWWWWWWWWWWfFFFFFBWWWWWWWWWWWw.",
    "...fFFFf............ffF........wwwwwwwwwwwwfFFFFFFBwwwwwwwwwww..",
    "...fFFf...................................fFFFFFB...............",
    "...fFf...................................fFFFFB.................",
    "...ff...................................fFFFB...................",
    ".......................................fFFB.....................",
    "......................................ffFB......................",
    ".....................................fff........................",
    "....................................ff..........................",
];

const WS_TAIL_1: Sprite = &[
    "............",
    "...f........",
    "...fD.......",
    "....fD......",
    "....fBD.....",
    ".....fBD....",
    ".....fFBD...",
    "......fFBD..",
    "......fFBD..",
    ".......fFBD.",
    ".......fFFBD",
    "........fFBD",
    "........fFFB",
    ".........fFF",
    ".........fFF",
    "........fFFF",
    "........fFFf",
    "........fFf.",
    ".......fFFf.",
    ".......fFf..",
    "......fFf...",
    "......fFf...",
    "......ff....",
    "......f.....",
    "............",
    "............",
    "............",
    "............",
];

const WS_TAIL_2: Sprite = &[
    "............",
    "......f.....",
    "......fD....",
    "......fD....",
    ".......fD...",
    ".......fBD..",
    ".......fBD..",
    "........fBD.",
    "........fBD.",
    "........fBD.",
    ".........fBD",
    ".........fBD",
    ".........fFB",
    "..........fF",
    "..........fF",
    ".........fFF",
    ".........fFf",
    ".........fF.",
    ".........fF.",
    "........fF..",
    "........fF..",
    "........ff..",
    "........f...",
    "............",
    "............",
    "............",
    "............",
    "............",
];

const WS_TAIL_3: Sprite = &[
    "............",
    "........f...",
    "........fD..",
    "........fD..",
    "........fD..",
    ".........fD.",
    ".........fD.",
    ".........fD.",
    ".........fBD",
    ".........fBD",
    "..........fD",
    "..........fB",
    "..........fB",
    "..........fF",
    "..........fF",
    ".........fFF",
    ".........fFf",
    ".........fF.",
    ".........fF.",
    ".........ff.",
    ".........f..",
    ".........f..",
    ".........f..",
    "............",
    "............",
    "............",
    "............",
    "............",
];

/// Feeding: the mouth opens on the gill rakers, then wide.
const WS_AJAR: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "DDDDDDDDD.....",
    "BBBoBBBoBDDD..",
    "BoBBBoBBBBoBD.",
    "GBGBBoBBBBBBD.",
    "GRGRoRRoRRRoRB",
    "GRGRRRRRRRoRRR",
    "GRGRRRRRRRRRRR",
    "GRGRRRReRRRRRR",
    "GRGRRnmmmmmmmm",
    "wWwWWnmkmkmkmm",
    "wWwWWWnnnnnnnn",
    "BWWWWWWWWWWWWw",
    "Bwwwwwwwwwwww.",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
];

const WS_GAPE: Sprite = &[
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
    "DDDDDDDDD.....",
    "BBBoBBBoBDDD..",
    "BoBBBoBBBBoBD.",
    "GBGBBoBBBBBBD.",
    "GRGRoRRoRRRoRB",
    "GRGRRRRRRRoRRR",
    "GRGRRRRRRRRRRR",
    "GRGRRRReRRRRRn",
    "GRGRRnmmmmmmmm",
    "wWwWnmkmkmkmkm",
    "wWwWnmmmmmmmmm",
    "BWWWWnkmkmkmkm",
    "BwwwWWnnnnnnnn",
    "....wwWWWWWWWw",
    "......wwwwwww.",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
];

const WS_QUARTER: Sprite = &[
    "............................................",
    "f..............ff...........................",
    "fD.............fFD..........................",
    ".fD............fFBD.........................",
    ".fBD..........ffFFBD........................",
    "..fBD.........fFFFFBD.......................",
    "..fFBD.......ffFFFFFBD......................",
    "...fFBD......fFFFFFFFBD.....................",
    "...fFFBD....fFFFFFFFFFFDDDDDDDDDDDDDDDD.....",
    "....fFFBD......BBBBBBBBBBoBBBoBBBoBBBoBDDD..",
    "....fFFFBD..DDDBBBBoBBBBBBoBBBoBBBoBBBoBBBDD",
    ".....fFFBDDDLLLLLLLLLLLLLLLLLLLGBGBGBoBBBBBD",
    "......fFFBBBRRoRRRoRRRoRRRoRRRoGRGRGRoRRoRRB",
    ".......fFFRRRRRRoRRRoRRRoRRRoRRGRGRGRRRRRoRR",
    ".......fFFRRLLLLLLLLLLLLLLLLLLLGRGRGRRRRRRRR",
    "......fFFFWWRRoRRRoRRRoRRRoRRRoGRGRGeRRRRRRR",
    ".....fFFFfwwWWWRRRRRRRRRRRRRRRRGRGRGnnnnnnnn",
    ".....fFFf...wwwWWWWWWWWWWWWWWWWwWwWwWWWWWWWw",
    "....fFFf.......wwwwwwwwWWWWWWWWwWwWwWWWWWWw.",
    "....fFf................wwwwwfFFFFBWWWWWWwFf.",
    "...fFf....................fFFFFFBwwwwwwFFff.",
    "...ff....................fFFFFB........Fff..",
    "........................fFFFB...........ff..",
    ".......................fFFB.................",
    "......................ffB...................",
    ".....................ff.....................",
    "............................................",
    "............................................",
];

const WS_FRONT: Sprite = &[
    "...................................",
    ".................f.................",
    "................fD.................",
    "................fD.................",
    "................fDD................",
    "................fDD................",
    "...............ffDD................",
    "...............fFDBD...............",
    "........DDDDDDDDDDDDDDDDDDD........",
    "......DBBoBBBoBBBBBBBoBBBoBBD......",
    "....DBoBBBoBBBoBBBBBoBBBoBBBoBD....",
    "...DRRRoRRRRoRRRLLLRRRoRRRRoRRRD...",
    "..DRoRRRRoRRRRRRLLLRRRRRRoRRRRoRD..",
    "..DeRRRRRRRRRRRRRRRRRRRRRRRRRRReD..",
    "..DRRRRRRRRRRRRRRRRRRRRRRRRRRRRRD..",
    "..DnnnnnnnnnnnnnnnnnnnnnnnnnnnnnD..",
    "...wWWWWWWWWWWWWWWWWWWWWWWWWWWWw...",
    "BFFFFfwWWWWWWWWWWWWWWWWWWWWWwfFFFFB",
    "BFf.....wwWWWWWWWWWWWWWWWww.....fFB",
    "..........wwwwwwwwwwwwwww..........",
    ".............f.......f.............",
    ".................f.................",
    ".................f.................",
    "................fff................",
    "...................................",
    "...................................",
    "...................................",
    "...................................",
];

const WS_PALETTE: &[(u8, Rgb)] = &[
    (b'D', Rgb(36, 52, 76)),
    (b'B', Rgb(54, 74, 102)),
    (b'L', Rgb(92, 114, 140)),
    (b'R', Rgb(70, 92, 120)),
    (b'G', Rgb(38, 54, 78)),
    (b'o', Rgb(214, 222, 226)),
    (b'W', Rgb(222, 226, 228)),
    (b'w', Rgb(178, 186, 194)),
    (b'F', Rgb(50, 68, 94)),
    (b'f', Rgb(30, 42, 62)),
    (b'e', Rgb(8, 10, 14)),
    (b'n', Rgb(26, 34, 48)),
    (b'm', Rgb(30, 22, 34)),
    (b'k', Rgb(140, 150, 160)),
];

pub const WHALE_SHARK: Rig = Rig {
    side: WS_SIDE,
    tails: [WS_TAIL_1, WS_TAIL_2, WS_TAIL_3],
    heads: [WS_AJAR, WS_GAPE],
    head_x: 50,
    quarter: WS_QUARTER,
    front: WS_FRONT,
    palette: WS_PALETTE,
    axis: 14,
    mouth: (62, 17),
    belly: [(31, 18), (23, 17)],
    cruise: 0.5,
    beat: 0.6,
    burst: 0.9,
};

// ----- the reef -----
//
// `a` lit, `b` body, `c` shade, `d` dark; each placed piece picks colors
// from [`REEF_COLORS`].

pub const BRAIN: Sprite = &[
    "...aaaaa...",
    ".aabbbabba.",
    "abbaaabbaab",
    "baabbbaabbc",
    "bbbaabbbacc",
    "cbcbbccbbcc",
    ".ccccccccc.",
];

pub const STAGHORN: Sprite = &[
    "..a.....a...",
    "..b..a..b.a.",
    "a.b..b.ab.b.",
    "b.bb.b.b..b.",
    ".b.b.bb..bb.",
    ".bb..b..bb..",
    "..bb.b.bb...",
    "...bbbbb....",
    "....cbc.....",
    "...ccccc....",
];

pub const SEA_FAN: Sprite = &[
    "..a..a..a..",
    ".aba.b.aba.",
    "a.b.aba.b.a",
    "ab.b.b.b.ba",
    ".bab.b.bab.",
    "b.b.bab.b.b",
    ".b.b.b.b.b.",
    "..bb.b.bb..",
    "...bbbbb...",
    ".....b.....",
    ".....c.....",
    "....ccc....",
];

pub const TABLE: Sprite = &[
    ".aaaaaaaaaaa.",
    "abbbbbbbbbbbb",
    ".ccccccccccc.",
    ".....bcb.....",
    "......b......",
    ".....ccc.....",
];

pub const SPONGES: Sprite = &[
    ".ada......",
    ".abc......",
    ".abc...ada",
    ".abc...abc",
    ".abcadaabc",
    ".abcabcabc",
    ".abcabcabc",
    ".abcabcabc",
    ".abcabcabc",
    ".cccccccc.",
];

/// Two frames, tentacles swaying.
pub const ANEMONE: [Sprite; 2] = [
    &[
        "..a.a.a..",
        ".a.aaa.a.",
        "a.aaaaa.a",
        ".aaaaaaa.",
        "..bbbbb..",
        "..bcccb..",
    ],
    &[
        "...a.a.a.",
        "..a.aaa.a",
        ".a.aaaaa.",
        ".aaaaaaa.",
        "..bbbbb..",
        "..bcccb..",
    ],
];

#[rustfmt::skip]
pub const STARFISH: Sprite = &[
    "..a..",
    "aabaa",
    ".bab.",
    "b...b",
];

#[rustfmt::skip]
pub const URCHIN: Sprite = &[
    ".d.d.d.",
    "d.ddd.d",
    ".ddddd.",
    "ddddddd",
];

/// Colors of reef pieces: lit, body, shade, dark.
pub const REEF_COLORS: [[Rgb; 4]; 6] = [
    [
        Rgb(250, 170, 180),
        Rgb(226, 120, 140),
        Rgb(170, 80, 100),
        Rgb(110, 50, 70),
    ],
    [
        Rgb(255, 196, 120),
        Rgb(240, 140, 70),
        Rgb(190, 96, 50),
        Rgb(120, 60, 40),
    ],
    [
        Rgb(210, 160, 240),
        Rgb(160, 100, 200),
        Rgb(110, 70, 150),
        Rgb(70, 40, 100),
    ],
    [
        Rgb(250, 236, 140),
        Rgb(226, 196, 80),
        Rgb(170, 140, 50),
        Rgb(110, 90, 40),
    ],
    [
        Rgb(150, 230, 210),
        Rgb(80, 190, 170),
        Rgb(50, 130, 120),
        Rgb(30, 80, 80),
    ],
    [
        Rgb(250, 130, 110),
        Rgb(210, 70, 60),
        Rgb(150, 40, 40),
        Rgb(90, 30, 30),
    ],
];

// ----- small fish -----
//
// `b` back, `w` belly, `s` flank, `y` stripe, `t` tail, `k` dark marks,
// `h` the head's glint. Two frames each: tail spread and tail closed.

#[rustfmt::skip]
pub const SARDINE: [Sprite; 2] = [
    &[
        "t..bbb..",
        ".bbbbbbh",
        "t..www..",
    ],
    &[
        "...bbb..",
        "tbbbbbbh",
        "...www..",
    ],
];

#[rustfmt::skip]
pub const FUSILIER: [Sprite; 2] = [
    &[
        "y..bbb..",
        ".yyyyyyb",
        "y..www..",
    ],
    &[
        "...bbb..",
        "yyyyyyyb",
        "...www..",
    ],
];

#[rustfmt::skip]
pub const JACK: [Sprite; 2] = [
    &[
        "t..bbb...",
        ".bbbbbbb.",
        ".bssssssh",
        "t..www...",
    ],
    &[
        "...bbb...",
        "tbbbbbbb.",
        "tbssssssh",
        "...www...",
    ],
];

/// A remora riding under the shark: dark, with its sucker on top of the head.
#[rustfmt::skip]
pub const REMORA: [Sprite; 2] = [
    &[
        "k....hhhh.",
        ".kkkkkkkke",
        "k.wwwwwwk.",
    ],
    &[
        ".....hhhh.",
        "kkkkkkkkke",
        "..wwwwwwk.",
    ],
];

/// A pilot fish, banded dark and light.
#[rustfmt::skip]
pub const PILOT: [Sprite; 2] = [
    &[
        "b.bkbbkb.",
        ".bbkbbkbb",
        "b.wkwwkw.",
    ],
    &[
        "..bkbbkb.",
        "bbbkbbkbb",
        "..wkwwkw.",
    ],
];

/// A school's colors: back, belly, flank, stripe, dark marks, glint.
pub struct Species {
    pub frames: &'static [Sprite; 2],
    pub colors: [(u8, Rgb); 7],
    /// World pixels per step at ease.
    pub pace: f32,
}

const fn fish_colors(back: Rgb, belly: Rgb, flank: Rgb, stripe: Rgb) -> [(u8, Rgb); 7] {
    [
        (b'b', back),
        (b't', back),
        (b'w', belly),
        (b's', flank),
        (b'y', stripe),
        (b'k', Rgb(40, 44, 52)),
        (b'h', Rgb(236, 244, 250)),
    ]
}

pub const COOL_SCHOOLS: [Species; 2] = [
    Species {
        frames: &SARDINE,
        colors: fish_colors(
            Rgb(50, 82, 120),
            Rgb(196, 210, 222),
            Rgb(132, 158, 182),
            Rgb(120, 150, 180),
        ),
        pace: 0.55,
    },
    Species {
        frames: &JACK,
        colors: fish_colors(
            Rgb(96, 116, 136),
            Rgb(222, 228, 234),
            Rgb(176, 188, 198),
            Rgb(150, 160, 170),
        ),
        pace: 0.65,
    },
];

pub const WARM_SCHOOLS: [Species; 2] = [
    Species {
        frames: &FUSILIER,
        colors: fish_colors(
            Rgb(52, 110, 196),
            Rgb(224, 232, 244),
            Rgb(150, 190, 236),
            Rgb(246, 210, 64),
        ),
        pace: 0.6,
    },
    Species {
        frames: &SARDINE,
        colors: fish_colors(
            Rgb(64, 150, 170),
            Rgb(220, 236, 236),
            Rgb(170, 214, 214),
            Rgb(120, 190, 190),
        ),
        pace: 0.55,
    },
];

pub const REMORA_COLORS: [(u8, Rgb); 4] = [
    (b'k', Rgb(86, 92, 98)),
    (b'w', Rgb(184, 190, 194)),
    (b'h', Rgb(124, 130, 136)),
    (b'e', Rgb(30, 32, 36)),
];

pub const PILOT_COLORS: [(u8, Rgb); 3] = [
    (b'b', Rgb(150, 170, 196)),
    (b'w', Rgb(214, 224, 234)),
    (b'k', Rgb(36, 40, 56)),
];

// ----- passers-by -----

/// A spotted eagle ray, seen a little from above: wings level, raised,
/// level, lowered. `D` back, `d` wing edges, `s` spots, `w` underside, `t`
/// tail, `h` head, `e` eye.
pub const RAY: [Sprite; 4] = [RAY_LEVEL, RAY_UP, RAY_LEVEL, RAY_DOWN];

const RAY_LEVEL: Sprite = &[
    "........dD..........",
    "......dDDDDd........",
    "....dDDsDDsDDd......",
    "ttttDDDDDDDDDDDDDDhe",
    "....wDDsDDDsDDw.....",
    "......wDDDDw........",
    "........wD..........",
];

const RAY_UP: Sprite = &[
    ".......dd...........",
    "......dDDd..........",
    ".....dDsDDDd........",
    "ttttDDDDDDDDDDDDDDhe",
    "....wDDsDDDsDDw.....",
    ".......wDDw.........",
    "....................",
];

const RAY_DOWN: Sprite = &[
    "....................",
    ".......dDDd.........",
    "....dDDsDDsDDd......",
    "ttttDDDDDDDDDDDDDDhe",
    ".....wDDsDDDsDw.....",
    "......wDDDDDw.......",
    ".......wDDw.........",
];

pub const RAY_COLORS: [(u8, Rgb); 7] = [
    (b'D', Rgb(48, 56, 80)),
    (b'd', Rgb(32, 38, 58)),
    (b's', Rgb(220, 228, 236)),
    (b'w', Rgb(190, 198, 210)),
    (b't', Rgb(38, 44, 62)),
    (b'h', Rgb(66, 74, 96)),
    (b'e', Rgb(12, 12, 16)),
];

/// A green turtle, its front flipper up, level, down and level again.
/// `o` shell, `O` its scutes, `k` skin, `p` plastron, `e` eye.
pub const TURTLE: [Sprite; 4] = [TURTLE_UP, TURTLE_MID, TURTLE_DOWN, TURTLE_MID];

const TURTLE_UP: Sprite = &[
    "............kk........",
    ".......ooooookkk......",
    ".....ooOOoOOookkk.....",
    "...ooOOoOOOOookkk.kkk.",
    "..oOOOoOOOOOoOOkkokkek",
    "...pppppppppppppppkkk.",
    "..kk..................",
    ".kk...................",
    "......................",
];

const TURTLE_MID: Sprite = &[
    "......................",
    ".......ooooooo........",
    ".....ooOOoOOoOoo......",
    "...ooOOoOOOOoOOOo.kkk.",
    "..oOOOoOOOOOoOOOOokkek",
    "...ppppppppppkkkkpkkk.",
    "..kk.....kkkkkk.......",
    ".kk......kk...........",
    "......................",
];

const TURTLE_DOWN: Sprite = &[
    "......................",
    ".......ooooooo........",
    ".....ooOOoOOoOoo......",
    "...ooOOoOOOOoOOOo.kkk.",
    "..oOOOoOOOOOoOOOOokkek",
    "...pppppppppppkkppkkk.",
    "..kk.........kkk......",
    ".kk.........kkk.......",
    "...........kkk........",
];

pub const TURTLE_COLORS: [(u8, Rgb); 5] = [
    (b'o', Rgb(92, 100, 56)),
    (b'O', Rgb(138, 140, 80)),
    (b'k', Rgb(120, 128, 92)),
    (b'p', Rgb(206, 192, 134)),
    (b'e', Rgb(16, 16, 12)),
];

/// A jellyfish, bell open and closed. `a` rim, `A` bell, `g` the four rings
/// inside, `t` tentacles; drawn see-through.
pub const JELLY: [Sprite; 2] = [
    &[
        "..aaaaa..",
        ".aAAAAAa.",
        "aAAgAgAAa",
        "aa.....aa",
        ".t.t.t.t.",
        "t..t.t..t",
        ".t..t..t.",
        "..t.t.t..",
        ".t..t..t.",
        "....t....",
    ],
    &[
        "...aaa...",
        "..aAAAa..",
        ".aAgAgAa.",
        ".aAAAAAa.",
        "..a...a..",
        "..t.t.t..",
        ".t..t..t.",
        "..t.t.t..",
        "..t...t..",
        "...t.t...",
    ],
];

pub const JELLY_COLORS: [(u8, Rgb); 4] = [
    (b'a', Rgb(214, 176, 232)),
    (b'A', Rgb(236, 212, 246)),
    (b'g', Rgb(252, 230, 252)),
    (b't', Rgb(224, 196, 238)),
];

/// A lanternfish: small and dark, its lights drawn on top.
#[rustfmt::skip]
pub const LANTERN: [Sprite; 2] = [
    &[
        "..kkk.",
        "tkkkke",
    ],
    &[
        "t.kkk.",
        ".kkkke",
    ],
];

pub const LANTERN_COLORS: [(u8, Rgb); 3] = [
    (b'k', Rgb(40, 46, 60)),
    (b't', Rgb(40, 46, 60)),
    (b'e', Rgb(150, 170, 190)),
];

/// The sharks there are.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    GreatWhite,
    Hammerhead,
    Tiger,
    WhaleShark,
}

impl Kind {
    pub const ALL: [Kind; 4] = [
        Kind::GreatWhite,
        Kind::Hammerhead,
        Kind::Tiger,
        Kind::WhaleShark,
    ];

    pub fn parse(s: &str) -> Option<Kind> {
        match s.replace(['_', ' '], "-").as_str() {
            "great-white" | "greatwhite" | "white" => Some(Kind::GreatWhite),
            "hammerhead" | "hammer" | "hammerhead-shark" => Some(Kind::Hammerhead),
            "tiger" | "tiger-shark" => Some(Kind::Tiger),
            "whale-shark" | "whaleshark" | "whale" => Some(Kind::WhaleShark),
            _ => None,
        }
    }

    /// Whether it filter feeds rather than hunting.
    pub fn filters(self) -> bool {
        self == Kind::WhaleShark
    }

    /// Whether it lives on warm coral reefs rather than in cool kelp forests.
    pub fn tropical(self) -> bool {
        !matches!(self, Kind::GreatWhite)
    }

    pub fn rig(self) -> &'static Rig {
        match self {
            Kind::GreatWhite => &GREAT_WHITE,
            Kind::Hammerhead => &HAMMERHEAD,
            Kind::Tiger => &TIGER,
            Kind::WhaleShark => &WHALE_SHARK,
        }
    }
}
