//! Día de Muertos in the jungle.
//!
//! The tiger becomes an Oaxacan alebrije (see `art.rs`), and the jungle is
//! dressed around it: [`Dress`] holds the colors, from the brief's own under
//! `classic` and from the theme's accents otherwise; papel picado is strung
//! from tree to tree; marigolds grow in the grass and in beds of their own;
//! and here and there by the path stand the ruins of a stone shrine, an
//! ofrenda in its doorway, its candles flickering and a trail of petals
//! leading to it. The higher the holiday, the more of all of it. Everything
//! here is hashed from the seed, like the jungle it dresses, so the camera
//! can follow the tiger forever.

use super::art::Sprite;
use super::jungle::{FEET, Jungle, MID_CELL, MID_P, TREE_CELL, View};
use super::{Kind, Rgb, hash, rgb_of, unit};
use crate::scene::Ctx;

// ----- the colors of the holiday -----

const CEMPASUCHIL: [Rgb; 3] = [Rgb(214, 98, 0), Rgb(255, 140, 0), Rgb(255, 170, 30)];
const ROSA: Rgb = Rgb(228, 0, 124);
const TERCIOPELO: Rgb = Rgb(176, 0, 72);
const MORADO: Rgb = Rgb(112, 40, 160);
const TURQUESA: Rgb = Rgb(0, 170, 184);
const AMARILLO: Rgb = Rgb(255, 210, 0);
const LIMON: Rgb = Rgb(140, 200, 40);
const HUESO: Rgb = Rgb(244, 238, 226);
const DEEP: Rgb = Rgb(40, 16, 52);

/// The holiday's colors, and how far it has come.
#[derive(Clone, Copy, Debug)]
pub struct Dress {
    /// 0 to 1, as in `Fiesta::level`.
    pub level: f32,
    /// Cempasúchil: dark, middle, light.
    pub marigold: [Rgb; 3],
    /// What the Bengal alebrije is painted over: marigold, or the theme's
    /// nearest to it.
    pub coat: Rgb,
    pub rosa: Rgb,
    pub terciopelo: Rgb,
    pub morado: Rgb,
    pub turquesa: Rgb,
    pub amarillo: Rgb,
    pub limon: Rgb,
    pub hueso: Rgb,
    /// The ink of lines and eyes.
    pub deep: Rgb,
    /// The papel picado flags.
    pub flags: [Rgb; 6],
}

/// The distance between two hues, in degrees.
fn hue_gap(a: f32, b: f32) -> f32 {
    let d = (a - b).abs() % 360.0;
    d.min(360.0 - d)
}

/// Hue (degrees), saturation and value of a color.
fn hsv(c: Rgb) -> (f32, f32, f32) {
    let (r, g, b) = (c.0 as f32 / 255.0, c.1 as f32 / 255.0, c.2 as f32 / 255.0);
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    let h = if d <= 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    (h, if max <= 0.0 { 0.0 } else { d / max }, max)
}

/// The theme's colorful accent whose hue is closest to `hue`, within `tol`
/// degrees, and not one of `avoid`.
fn by_hue(cols: &[Rgb], hue: f32, tol: f32, avoid: &[Rgb]) -> Option<Rgb> {
    cols.iter()
        .copied()
        .filter_map(|c| {
            let (h, s, v) = hsv(c);
            let gap = hue_gap(h, hue);
            (s >= 0.25 && v >= 0.3 && gap <= tol && !avoid.contains(&c)).then_some((gap, c))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

impl Dress {
    /// The brief's own colors.
    fn classic(level: f32) -> Dress {
        Dress {
            level,
            marigold: CEMPASUCHIL,
            coat: CEMPASUCHIL[1],
            rosa: ROSA,
            terciopelo: TERCIOPELO,
            morado: MORADO,
            turquesa: TURQUESA,
            amarillo: AMARILLO,
            limon: LIMON,
            hueso: HUESO,
            deep: DEEP,
            flags: [ROSA, Rgb(255, 120, 20), MORADO, TURQUESA, AMARILLO, LIMON],
        }
    }

    /// The dress for today: the holiday's colors, or the theme's own where
    /// it has something close. The marigolds keep their hue when it does not.
    pub fn new(ctx: &Ctx) -> Dress {
        let level = ctx.fiesta.level.clamp(0.0, 1.0);
        if ctx.theme.native {
            return Dress::classic(level);
        }
        let t = ctx.theme;
        let cols: Vec<Rgb> = t.colors.iter().filter_map(|&c| rgb_of(c)).collect();
        if cols.is_empty() {
            return Dress::classic(level);
        }
        let pick = |i: usize| cols[i % cols.len()];
        let near = by_hue(&cols, 32.0, 30.0, &[]);
        let mari = near.unwrap_or(CEMPASUCHIL[1]);
        // The flowers keep their hue whatever the theme; the tiger does not.
        let coat = near.unwrap_or_else(|| rgb_of(t.grad.at(0.7)).unwrap_or(mari));
        let rosa = by_hue(&cols, 335.0, 40.0, &[mari]).unwrap_or_else(|| pick(1));
        let turquesa = by_hue(&cols, 185.0, 40.0, &[mari, rosa]).unwrap_or_else(|| pick(2));
        let amarillo = by_hue(&cols, 52.0, 22.0, &[mari, rosa, turquesa]).unwrap_or(mari);
        let morado = by_hue(&cols, 280.0, 40.0, &[mari, rosa, turquesa]).unwrap_or_else(|| pick(0));
        let limon = by_hue(&cols, 85.0, 45.0, &[mari, rosa, turquesa]).unwrap_or_else(|| pick(3));
        let fg = rgb_of(t.fg).unwrap_or(HUESO);
        Dress {
            level,
            marigold: [mari.scale(0.8), mari, mari.lerp(Rgb::WHITE, 0.18)],
            coat,
            rosa,
            terciopelo: rosa.scale(0.72),
            morado,
            turquesa,
            amarillo,
            limon,
            hueso: fg,
            deep: morado.scale(0.3),
            flags: [rosa, mari, morado, turquesa, amarillo, limon],
        }
    }

    /// The alebrije's colors by letter, for the tiger of this `kind`: bone,
    /// marigold or sand for the coat, and the same stripes and dots on all.
    pub fn alebrije(&self, kind: Kind) -> [(u8, Rgb); 18] {
        let coat = match kind {
            Kind::Bengal => self.coat,
            Kind::Siberian => self.coat.lerp(self.hueso, 0.3),
            Kind::White => self.hueso,
        };
        let bone = self.hueso.lerp(self.amarillo, 0.15);
        [
            (b'O', coat),
            (b'o', coat.scale(0.8)),
            (b'S', self.rosa),
            (b's', self.terciopelo),
            (b'T', self.turquesa),
            (b'u', self.turquesa.scale(0.68)),
            (b'Y', self.amarillo),
            (b'y', self.amarillo.scale(0.82)),
            (b'W', bone),
            (b'w', bone.scale(0.84)),
            (b'K', self.deep),
            (b'E', self.morado),
            (b'e', self.limon),
            (b'k', self.deep),
            (b'n', self.rosa),
            (b'm', self.terciopelo.scale(0.65)),
            (b't', self.hueso),
            (b'p', self.rosa.lerp(Rgb::WHITE, 0.4)),
        ]
    }

    /// The saucer's colors by letter: sugar and icing, and marigold lights.
    pub fn saucer(&self) -> [(u8, Rgb); 12] {
        let [dark, mid, light] = self.marigold;
        [
            (b'W', self.hueso),
            (b'w', self.hueso.scale(0.84)),
            (b'm', self.hueso.scale(0.66)),
            (b'S', self.rosa),
            (b'T', self.turquesa),
            (b'Y', self.amarillo),
            (b'K', self.deep),
            (b'F', mid),
            (b'L', light),
            (b'l', dark.scale(0.5)),
            (b'R', light.lerp(self.hueso, 0.3)),
            (b'P', self.morado),
        ]
    }

    /// The ofrenda's colors by letter.
    pub fn ofrenda(&self) -> [(u8, Rgb); 19] {
        let [dark, mid, light] = self.marigold;
        let bread = dark.lerp(Rgb(170, 110, 50), 0.5);
        [
            (b'F', mid),
            (b'f', light),
            (b'D', dark),
            (b'R', self.terciopelo),
            (b'S', self.rosa),
            (b'T', self.turquesa),
            (b'u', self.turquesa.scale(0.68)),
            (b'Y', self.amarillo),
            (b'y', self.amarillo.scale(0.82)),
            (b'K', self.deep),
            (b'W', self.hueso),
            (b'w', self.hueso.scale(0.84)),
            (b'B', bread),
            (b'b', bread.scale(0.75)),
            (b'P', self.morado),
            (b'L', self.limon),
            (b'l', self.limon.scale(0.45)),
            (b'C', self.hueso.lerp(Rgb(240, 226, 190), 0.5)),
            (b'z', self.turquesa.lerp(Rgb::WHITE, 0.6)),
        ]
    }
}

/// A candle's flame: core and edge. They keep their own hue in every theme.
const FLAMA: (Rgb, Rgb) = (Rgb(255, 214, 120), Rgb(255, 150, 40));

/// The stone of the ruins, by letter.
const STONE: [(u8, Rgb); 8] = [
    (b'H', Rgb(172, 164, 142)),
    (b'G', Rgb(136, 128, 110)),
    (b'g', Rgb(96, 90, 78)),
    (b'M', Rgb(92, 128, 62)),
    (b'm', Rgb(58, 92, 46)),
    (b'x', Rgb(24, 18, 30)),
    (b'X', Rgb(38, 28, 44)),
    (b'c', Rgb(112, 104, 92)),
];

// ----- the drawings -----

/// The papel picado flags, 8 wide: `#` is paper and `.` is cut away. The top row is folded over
/// the string, the bottom two are the fringe.
pub const FLAG_DIAMONDS: Sprite = &[
    "########", "##.##.##", "#.####.#", ".######.", "#.####.#", "##.##.##", "########", "#.####.#",
    "..#..#..",
];

pub const FLAG_SKULL: Sprite = &[
    "########", "#.####.#", "#.#..#.#", "##....##", "#.####.#", "##.##.##", "#.#..#.#", "#.####.#",
    "..#..#..",
];

pub const FLAG_FLOWER: Sprite = &[
    "########", "##.##.##", "#.#..#.#", ".#....#.", "#.#..#.#", "##.##.##", "########", "#.####.#",
    "..#..#..",
];

pub const FLAG_CROSS: Sprite = &[
    "########", "###..###", "###..###", "#......#", "#......#", "###..###", "###..###", "#.####.#",
    "..#..#..",
];

pub const FLAG_BIRD: Sprite = &[
    "########", "#.####.#", "#..##..#", "##....##", "###..###", "##.##.##", "#.####.#", "#.#..#.#",
    "..#..#..",
];

pub const FLAG_LATTICE: Sprite = &[
    "########", "#.#..#.#", "##.##.##", "#.#..#.#", "##.##.##", "#.#..#.#", "########", "#.####.#",
    "..#..#..",
];

/// The ruin: a wall with a corbel doorway between a tall pillar and a broken one. `H` `G` `g`
/// stone from light to dark, `c` carving, `M` `m` moss, `X` `x` the dark inside.
pub const RUIN: Sprite = &[
    "mHHmHHHH................................................",
    "GGGGGGGG................................................",
    "gggggggg................................................",
    "gggggggggggggMgMmg......................................",
    "HGGGGgGHHGHGHGGgHHGMGHHmMgGH............................",
    "GGGGGgGGGGGGGGGgGGcGGGGcGgGGcGGG........................",
    "HGGccgGGGGgGGGGgGcGcHGcGcgGcGcGGc.cg.c..G...............",
    "gggcggggggggggggggcggggcggggcggggcggMgc.g...............",
    "gGGGGGHGHHgGGHGHHGHGgGHHXXXXXXXXGHHGGGHHg...............",
    "gGGGGGGGGGgGHgGGHGHGggGGxxxxxxxxGGGGGGGGg...............",
    "gGGGGGGGGGgGGGHGGGGGgGXXXXXXXXXXXXGgGGGGgGGmmGGG........",
    "gggccgggggggggggggggggxxxxxxxxxxxxgggggggggggggg........",
    "GHGcHgGHHGHGHHHgGHHHXXXXXXXXXXXXXXXXGGGHHGHHGgGH........",
    "GHGGGgGGGHGGGGGgGGGGxxxxxxxxxxxxxxxxGGGGGHGGGgGG........",
    "GGGHGgGGGGGGGGGgHGXXXXXXXXXXXXXXXXXXXXGGGGGGGgGG........",
    "ggggggggggggggggggxxxxxxxxxxxxxxxxxxxxgggggggggg........",
    "gGGccHHGGGgGGHHGxxxxxxxxxxxxxxxxxxxxxxxxgGGGGHHH.mg.Mm.G",
    "gGGcGGGGGGgGGGGGxxxxxxxxxxxxxxxxxxxxxxxxgGGGGGGGmGgMGGGG",
    "gGGGGGGGGGgGGgGGxxxxxxxxxxxxxxxxxxxxxxxxgGgGGGGGGGgGGGGG",
    "ggggggggggggggggxxxxxxxxxxxxxxxxxxxxxxxxgggggggggggggggg",
    "HHHHGgGHHHGGGHHgxxxxxxxxxxxxxxxxxxxxxxxxHHHGGgGGHHHHHHHg",
    "GGGccgGGGHGGGGGgxxxxxxxxxxxxxxxxxxxxxxxxgGGGGgGHGGGGGgGg",
    "GGGcGgGGGGGGGGGgxxxxxxxxxxxxxxxxxxxxxxxxGGGGGgGGGGGGGGGg",
    "ggggggggggggggggxxxxxxxxxxxxxxxxxxxxxxxxgggggggggggggggg",
    "gGHHHGHHHGgGGGHHxxxxxxxxxxxxxxxxxxxxxxxxgHHGHGHHHGgGGHHG",
    "gGGGGGGGGggGGGHGxxxxxxxxxxxxxxxxxxxxxxxxgGgGGGGGGggGGGGG",
    "gGGccGGGGGgGGHGGxxxxxxxxxxxxxxxxxxxxxxxxggHGGGGGGGgGGGGG",
    "gggcggggggggggggxxxxxxxxxxxxxxxxxxxxxxxxgggggggggggggggg",
    "HHGHHgGHHGHGHHHgxxxxxxxxxxxxxxxxxxxxxxxxHHGGHggHHHHGHHHg",
    "GHGGGgGGGGGGGGGgxxxxxxxxxxxxxxxxxxxxxxxxHGGGGgGHGHGGgGGg",
    "mmGGMgMGGGgGGGGMxxxxxxxxxxxxxxxxxxxxxxxxMMmMMmgMMGGGMGmg",
    "gmmggggmgggggggMxxxxxxxxxxxxxxxxxxxxxxxxggMMgMgggMMggggg",
    "gGGHHMHMHMgmmmHGxxxxxxxxxxxxxxxxxxxxxxxxgGHHHGmHGHgGGHMM",
    "gGGGGGGMGGgMGGmMxxxxxxxxxxxxxxxxxxxxxxxxgGGGGGGGGGMGGGGG",
    "gGGGMGGMGGgGGGGMGGGGMGGGmmGGMGgGMGGGGGGMgGGmGGGMGgGGMGMM",
    "gmGMGmGGGMgmmHMHHHHHHHmmHMHHMmMHHHHHHHMHHHHMHGGGGGMGGmmG",
];

/// The ofrenda in the doorway, in layers that go over one another as the holiday builds up. The
/// first: the tiers, the photo and the candles.
pub const OFRENDA_0: Sprite = &[
    "............................",
    "............................",
    "............................",
    "..........YYYYYYYY..........",
    "..........YPWWWWPY..........",
    "..........YWWWWWWY..........",
    "..........YWKWWKWY..........",
    ".......C..YWKWWKWY..C.......",
    ".......C..YWWKKWWY..C.......",
    ".......C..YPWKWKPY..C.......",
    ".......C..YYYYYYYY..C.......",
    ".....WWWWWWWWWWWWWWWWWW.....",
    ".....TSTSTSTSTSTSTSTSTS.....",
    "............................",
    "............................",
    "............................",
    "............................",
    "...WWWWWWWWWWWWWWWWWWWWWW...",
    "...STSTSTSTSTSTSTSTSTSTST...",
    "....fFf..............fFf....",
    "...FfDFf....BBBB....FfDFf...",
    "...FDFDF...BWBBWB...FDFDF...",
    "C...lFl....bbbbbb....lFl...C",
    "C.WWWWWWWWWWWWWWWWWWWWWWWW.C",
    "C.SSSSSSSSSSSSSSSSSSSSSSSS.C",
    "C.SSSSSSSSSSSSSSSSSSSSSSSS.C",
    "C.SSSSSSSSSSSSSSSSSSSSSSSS.C",
    "C.TSTSTSTSTSTSTSTSTSTSTSTS.C",
];

/// The second: sugar skulls, marigolds, oranges and a cup of water.
pub const OFRENDA_1: Sprite = &[
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "....WWWW............WWWW....",
    "....SWWS.....fF.....TWWT....",
    "....WKKW....FfDF....WKKW....",
    "....KWWK.....lF.....KWWK....",
    "............................",
    "............................",
    "............................",
    ".........f.......z.z........",
    "........FFF......zWz........",
    ".........F.......zzz........",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
];

/// The third: more candles, pan de muerto, terciopelo and papel picado over the arch.
pub const OFRENDA_2: Sprite = &[
    "..........STYFSTYF..........",
    "..........S.T.Y.F...........",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    ".......RR..........RR.......",
    "......RSRR........RSRR......",
    ".......RR..........RR.......",
    "............................",
    "............................",
    "............................",
    "...C................F...C...",
    "...C.....BB........bBb..C...",
    "...C....BBBB........b...C...",
    ".......bbbbbb...............",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
];

/// The last: marigolds all along the doorway, nube and petals on the floor.
pub const OFRENDA_3: Sprite = &[
    "..........F......D..........",
    "..........D......f..........",
    "........F..........f........",
    "........D..........F........",
    "......F..............F......",
    "......D..............D......",
    "....F..................D....",
    "....D..................f....",
    "..F...W.W................f..",
    ".......W....................",
    "..f...W.W................D..",
    "............................",
    "..D......................F..",
    "............................",
    "..F......................f..",
    "............................",
    "..f......................D..",
    "............................",
    "..D......................F..",
    "............................",
    "..F......................f..",
    "............................",
    "..f......................D..",
    "............................",
    "..D......................F..",
    "............................",
    "..F......................f..",
    "....f..f..f..f..f..f..f.....",
];

/// The saucer of Día de Muertos, in its two phases of rim lights: a calavera de azúcar for a
/// dome, its icing for a hull. `W` sugar and `w` `m` its shade, `S` `T` `Y` icing, `K` the dark
/// of the sockets, `F` marigold, `L` a lit light and `l` an unlit one, `R` the emitter.
pub const UFO_SKULL_0: Sprite = &[
    "..........wWWWWWw..........",
    "........wWWWWWWWWWw........",
    ".......wWWWWWFWWWWWw.......",
    ".....wWWWWWWFYFWWWWWWw.....",
    ".....wTSSSTWWWWWTSSSTw.....",
    ".....wSKKKSWWWWWSKKKSw.....",
    ".....wSKKKSWWKWWSKKKSw.....",
    ".....wTSSSTWKKKWTSSSTw.....",
    ".....wWWWYWWWWWWWYWWWw.....",
    ".....WKWKWKWKWKWKWKWKW.....",
    ".SYTSYTSYTSYTSTYSTYSTYSTYS.",
    "mLWWWlWWWLWWWlWWWLWWWlWWWLm",
    ".mWWWSWWWTWWWYWWWTWWWSWWWm.",
    "...mmmmmmmmWWWWWmmmmmmmm...",
    "..........RRRRRRR..........",
];

pub const UFO_SKULL_1: Sprite = &[
    "..........wWWWWWw..........",
    "........wWWWWWWWWWw........",
    ".......wWWWWWFWWWWWw.......",
    ".....wWWWWWWFYFWWWWWWw.....",
    ".....wTSSSTWWWWWTSSSTw.....",
    ".....wSKKKSWWWWWSKKKSw.....",
    ".....wSKKKSWWKWWSKKKSw.....",
    ".....wTSSSTWKKKWTSSSTw.....",
    ".....wWWWYWWWWWWWYWWWw.....",
    ".....WKWKWKWKWKWKWKWKW.....",
    ".SYTSYTSYTSYTSTYSTYSTYSTYS.",
    "mlWWWLWWWlWWWLWWWlWWWLWWWlm",
    ".mWWWSWWWTWWWYWWWTWWWSWWWm.",
    "...mmmmmmmmWWWWWmmmmmmmm...",
    "..........RRRRRRR..........",
];

/// The saucer, in its two phases of lights.
pub const SAUCER: [Sprite; 2] = [UFO_SKULL_0, UFO_SKULL_1];

/// How much taller the saucer is than the one it replaces.
pub const SAUCER_EXTRA: f32 = 5.0;

/// The flags' cut-outs.
const FLAGS: [Sprite; 6] = [
    FLAG_DIAMONDS,
    FLAG_SKULL,
    FLAG_FLOWER,
    FLAG_CROSS,
    FLAG_BIRD,
    FLAG_LATTICE,
];

/// The ofrenda's layers, and the level of the holiday each comes at.
const OFRENDA: [(Sprite, f32); 4] = [
    (OFRENDA_0, 0.0),
    (OFRENDA_1, 0.45),
    (OFRENDA_2, 0.8),
    (OFRENDA_3, 0.95),
];

/// How far down a flag swings at each of its rows, against its bottom's.
const SWING: [f32; 9] = [0.0, 0.03, 0.1, 0.2, 0.33, 0.5, 0.68, 0.85, 1.0];

// ----- what stands where -----

/// Ruins come in cells this wide; marigold beds in smaller ones.
const RUIN_CELL: i64 = 260;
const BED_CELL: i64 = 64;
/// A ruin's drawing: half its width, its height, and the row its stone stands on.
const RUIN_HALF: f32 = 28.0;
const RUIN_H: i32 = 36;
const RUIN_FOOT: i32 = FEET - 2;
/// How far from a ruin's middle its pillars stand, and how high they reach
/// (the tall one and the broken one) above its foot.
const PILLAR: f32 = 24.0;
const PILLAR_TOP: (i32, i32) = (34, 19);
/// Where the doorway's ofrenda sits on the ruin's drawing.
const OFRENDA_AT: (i32, i32) = (14, 8);

/// A ruin by the path, in local x.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ruin {
    pub x: f32,
    pub id: i32,
    /// Mirrored: the tall pillar on the right.
    pub flip: bool,
}

/// The ruins in a stretch of the jungle, kept on the stack.
#[derive(Default)]
pub struct Ruins {
    v: [Ruin; 8],
    n: usize,
}

impl Ruins {
    fn push(&mut self, r: Ruin) {
        if self.n < self.v.len() {
            self.v[self.n] = r;
            self.n += 1;
        }
    }

    pub fn as_slice(&self) -> &[Ruin] {
        &self.v[..self.n]
    }
}

/// A place a string of papel picado is tied to: a tree, or a ruin's pillar.
#[derive(Clone, Copy, Default)]
struct Anchor {
    x: f32,
    y: f32,
    id: i32,
    /// Which ruin, if it is a pillar; trees are 0.
    ruin: i32,
}

/// The row the strings are tied to on a pole: the trees' height, about.
const POLE_TOP: f32 = 54.0;

/// Anchors held at once, and how far apart two may be for a string.
const ANCHORS: usize = 32;
const STRING_MAX: f32 = 360.0;

/// What a frame painted, for the tests.
#[cfg(test)]
#[derive(Default, Debug)]
pub struct Painted {
    pub flags: u32,
    pub blossoms: u32,
    pub ruins: u32,
    pub candles: u32,
    pub petals: u32,
    /// The tiger was painted as an alebrije, and the saucer as a skull.
    pub alebrije: bool,
    pub saucer: bool,
}

impl Jungle {
    // ----- ruins and their ofrendas -----

    /// The ruin of a cell of the jungle, if it has one, on dry land and clear
    /// of the big trees.
    fn ruin_in(&self, cell: i64) -> Option<Ruin> {
        let d = self.pal.fest?;
        let (s, c) = (self.seed, cell as i32);
        if unit(c, 111, s) > 0.46 + 0.36 * d.level {
            return None;
        }
        let u = (cell * RUIN_CELL) as f64 + 46.0 + unit(c, 112, s) as f64 * (RUIN_CELL - 92) as f64;
        let x = (u - self.base as f64) as f32;
        if [x - 30.0, x, x + 30.0]
            .iter()
            .any(|&p| self.water(p).is_some())
        {
            return None;
        }
        let tree = |x: f64| ((u + x) / TREE_CELL as f64).floor() as i64;
        for tc in tree(-70.0)..=tree(70.0) {
            if let Some(t) = self.trunk_in(tc)
                && (t.x - x).abs() < RUIN_HALF + t.w as f32 / 2.0 + 9.0
            {
                return None;
            }
        }
        Some(Ruin {
            x,
            id: c,
            flip: unit(c, 113, s) < 0.5,
        })
    }

    /// The ruins with any part between local `x0` and `x1`, and a few
    /// cells' worth beyond.
    pub fn ruins(&self, x0: f32, x1: f32) -> Ruins {
        let mut out = Ruins::default();
        if self.pal.fest.is_none() {
            return out;
        }
        let cell = |x: f32, pad: f64| {
            ((self.base as f64 + x as f64 + pad) / RUIN_CELL as f64).floor() as i64
        };
        for c in cell(x0, -70.0)..=cell(x1, 70.0) {
            if let Some(r) = self.ruin_in(c) {
                out.push(r);
            }
        }
        out
    }

    /// A palette lit for the hour, then brought back toward its own colors
    /// by `k`: what stands in candlelight.
    fn lut_glow(&self, pal: &[(u8, Rgb)], k: f32) -> [Option<Rgb>; 128] {
        let mut lut = [None; 128];
        for &(ch, c) in pal {
            lut[(ch & 127) as usize] = Some(self.lit(c).lerp(c, k));
        }
        lut
    }

    /// A palette lit for the hour, and at night glowing a little of its own
    /// light: the alebrije is a spirit.
    pub fn lut_spirit(&self, pal: &[(u8, Rgb)]) -> [Option<Rgb>; 128] {
        let mut lut = self.lut(pal);
        let k = 0.12 + 0.3 * (1.0 - self.daylight);
        for &(ch, c) in pal {
            if let Some(l) = &mut lut[(ch & 127) as usize] {
                *l = l.lerp(c, k);
            }
        }
        lut
    }

    /// Draws a sprite with its top left corner on `(x0, y0)`.
    fn stamp(&mut self, s: Sprite, x0: i32, y0: i32, lut: &[Option<Rgb>; 128]) {
        for (r, row) in s.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                if let Some(col) = lut[(ch & 127) as usize] {
                    self.put(x0 + c as i32, y0 + r as i32, col);
                }
            }
        }
    }

    /// The ruins in view: a trail of petals leading to each, the stone, and
    /// the ofrenda in its doorway with its candles lit.
    pub fn paint_ruins(&mut self) {
        let Some(d) = self.pal.fest else { return };
        let ruins = self.ruins(self.bx as f32, (self.bx + self.bw) as f32);
        if ruins.n == 0 {
            return;
        }
        let night = 1.0 - self.daylight;
        let stone = self.lut(&STONE);
        let ofrenda = self.lut_glow(&d.ofrenda(), 0.25 + 0.6 * night);
        for r in ruins.as_slice() {
            self.trail(r.x, &d);
            let face = if r.flip { -1.0 } else { 1.0 };
            self.sprite(RUIN, r.x, RUIN_FOOT as f32, face, &stone);
            self.poles(r, &d);
            let (x0, y0) = (
                r.x.round() as i32 - RUIN_HALF as i32 + OFRENDA_AT.0,
                RUIN_FOOT - RUIN_H + 1 + OFRENDA_AT.1,
            );
            #[cfg(test)]
            {
                self.painted.ruins += 1;
            }
            for &(layer, at) in &OFRENDA {
                if d.level < at {
                    break;
                }
                self.stamp(layer, x0, y0, &ofrenda);
                self.candles(layer, x0, y0, r.id);
            }
            // Once the burner is there, its smoke curls up the doorway.
            if d.level >= OFRENDA[2].1 {
                self.copal(x0 + 20, y0 + 13, r.id, &d);
            }
        }
    }

    /// A bamboo pole on each pillar of a ruin, a ribbon tied at the top.
    fn poles(&mut self, r: &Ruin, d: &Dress) {
        let (tall, broken) = (PILLAR_TOP.0, PILLAR_TOP.1);
        let (left, right) = if r.flip {
            (broken, tall)
        } else {
            (tall, broken)
        };
        let bamboo = [Rgb(178, 150, 84), Rgb(132, 108, 60)].map(|c| self.lit(c));
        for (x, from, ribbon) in [
            (r.x - PILLAR, left, d.rosa),
            (r.x + PILLAR, right, d.turquesa),
        ] {
            let ix = x.round() as i32;
            let base = RUIN_FOOT - from;
            for y in POLE_TOP as i32..base {
                self.put(ix, y, bamboo[((y - POLE_TOP as i32) % 6 == 5) as usize]);
            }
            let ribbon = self.lit(ribbon);
            self.put(ix + 1, POLE_TOP as i32, ribbon);
            self.put(ix + 1, POLE_TOP as i32 + 1, ribbon);
        }
    }

    /// Copal smoke curling up from `(x, y)`, thinning as it climbs.
    fn copal(&mut self, x: i32, y: i32, id: i32, d: &Dress) {
        const RISE: i32 = 22;
        let smoke = self.lit(d.hueso).lerp(d.hueso, 0.3);
        let t = self.time();
        for k in 0..RISE {
            let h = k as f32;
            let sway = (t * 1.3 + h * 0.45 + id as f32).sin() * (0.5 + h * 0.16);
            let a = 0.6 * (1.0 - h / RISE as f32).powf(1.3);
            let px = x + sway.round() as i32;
            self.blend(px, y - (h * 1.5) as i32, smoke, a);
            if k < RISE / 2 {
                self.blend(px + 1, y - (h * 1.5) as i32, smoke, a * 0.5);
            }
        }
    }

    /// A flame over the top of every candle of `layer`.
    fn candles(&mut self, layer: Sprite, x0: i32, y0: i32, id: i32) {
        let t = self.tick / 3;
        for (r, row) in layer.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                if ch != b'C' || (r > 0 && layer[r - 1].as_bytes()[c] == b'C') {
                    continue;
                }
                let (x, y) = (x0 + c as i32, y0 + r as i32);
                // It gutters a little, each in its own time.
                let h = hash(x, id.wrapping_add(t as i32), self.seed);
                self.put(x, y - 1, FLAMA.0);
                self.put(x, y - 2, FLAMA.0.lerp(FLAMA.1, 0.5));
                if !h.is_multiple_of(5) {
                    self.put(x, y - 3, FLAMA.1);
                }
                if h % 7 == 3 {
                    self.put(x + 1 - 2 * ((h >> 5) & 1) as i32, y - 3, FLAMA.1);
                }
                self.blend(x - 1, y - 1, FLAMA.1, 0.5);
                self.blend(x + 1, y - 1, FLAMA.1, 0.5);
                #[cfg(test)]
                {
                    self.painted.candles += 1;
                }
            }
        }
    }

    /// Marigold petals strewn over the ground toward the ruin at `x`, thicker
    /// as they near it: the way the dead find home.
    fn trail(&mut self, x: f32, d: &Dress) {
        let petals =
            [d.marigold[0], d.marigold[1], d.marigold[2], d.marigold[1]].map(|c| self.lit(c));
        let reach = 130.0;
        let from = ((x - reach) as i32).max(self.bx);
        let to = ((x + reach) as i32).min(self.bx + self.bw);
        let k = 0.2 + 0.25 * d.level;
        for px in from..to {
            let near = 1.0 - (px as f32 - x).abs() / reach;
            let u = (self.base + px as i64) as i32;
            for i in 0..3 {
                let h = hash(u, i, self.seed ^ 0x7e7a);
                if (h & 0xff) as f32 / 255.0 > near * near * k {
                    continue;
                }
                let y = FEET - 3 + ((h >> 8) % 11) as i32;
                let c = petals[((h >> 16) % 4) as usize];
                self.put(px, y, c);
                if h >> 24 & 3 == 0 {
                    self.put(px + 1, y, c);
                }
                #[cfg(test)]
                {
                    self.painted.petals += 1;
                }
            }
        }
    }

    /// The warm light of the candles, over everything near them.
    pub fn glow_lights(&mut self, v: View) {
        let Some(d) = self.pal.fest else { return };
        let ruins = self.ruins(v.x, v.x + v.w);
        let night = 1.0 - self.daylight;
        let tick = self.tick as f32;
        for r in ruins.as_slice() {
            let (cx, cy) = v.to_screen(r.x, (RUIN_FOOT - 12) as f32);
            let rad = ((18.0 + 8.0 * d.level) * v.z).max(3.0);
            let flicker =
                0.88 + 0.12 * (tick * 0.23 + r.id as f32 * 1.7).sin() * (tick * 0.071).cos();
            let a = (0.1 + 0.4 * night) * flicker;
            let ri = rad.ceil() as i32;
            for dy in -ri..=ri {
                for dx in -ri..=ri {
                    let q = ((dx * dx + dy * dy) as f32).sqrt() / rad;
                    if q < 1.0 {
                        self.blend_out(cx + dx, cy + dy, FLAMA.1, a * (1.0 - q) * (1.0 - q));
                    }
                }
            }
        }
    }

    // ----- marigolds -----

    /// A blossom with its middle at `(x, y)`: a marigold for the most part,
    /// now and then a terciopelo or a puff of nube. `h` is its dice.
    pub fn blossom(&mut self, x: i32, y: i32, h: u32) {
        let Some(d) = self.pal.fest else { return };
        let [dark, mid, light] = d.marigold.map(|c| self.lit(c));
        match h % 9 {
            0 => {
                let (a, b) = (self.lit(d.rosa), self.lit(d.terciopelo));
                self.put(x, y, a);
                self.put(x + 1, y, b);
                self.put(x - 1, y + 1, b);
                self.put(x, y + 1, b);
                self.put(x + 1, y + 1, b);
            }
            1 => {
                let w = self.lit(d.hueso);
                self.put(x, y, w);
                self.put(x + 1, y - 1, w);
                self.put(x - 1, y + 1, w);
            }
            _ => {
                self.put(x, y - 1, mid);
                self.put(x - 1, y, mid);
                self.put(x + 1, y, mid);
                self.put(x, y + 1, dark);
                self.put(x, y, light);
                if h.is_multiple_of(3) {
                    self.put(x - 1, y - 1, dark);
                    self.put(x + 1, y - 1, dark);
                    self.put(x - 1, y + 1, dark);
                    self.put(x + 1, y + 1, dark);
                }
            }
        }
        #[cfg(test)]
        {
            self.painted.blossoms += 1;
        }
    }

    /// A bed of marigolds in a cell of the jungle: from, to.
    fn bed_in(&self, cell: i64) -> Option<(f32, f32)> {
        let d = self.pal.fest?;
        let (s, c) = (self.seed, cell as i32);
        if unit(c, 131, s) > 0.3 + 0.35 * d.level {
            return None;
        }
        let w = 14.0 + 18.0 * unit(c, 132, s);
        let u = (cell * BED_CELL) as f64 + unit(c, 133, s) as f64 * (BED_CELL as f32 - w) as f64;
        let a = (u - self.base as f64) as f32;
        if self.water(a - 2.0).is_some() || self.water(a + w + 2.0).is_some() {
            return None;
        }
        Some((a, a + w))
    }

    /// The beds of marigolds: the back row stands behind the animals, the
    /// front row, shorter, in front of their paws.
    pub fn paint_beds(&mut self, front: bool) {
        let Some(d) = self.pal.fest else { return };
        let (bx, bw) = (self.bx, self.bw);
        let (u0, u1) = (
            self.base as f64 + bx as f64,
            self.base as f64 + (bx + bw) as f64,
        );
        let leaf = self.lit(self.green(Rgb(70, 126, 48)));
        let leaf_dark = self.lit(self.green(Rgb(48, 98, 38)));
        let thick = 0.22 + 0.2 * d.level;
        let cells =
            (u0 / BED_CELL as f64).floor() as i64 - 1..=(u1 / BED_CELL as f64).floor() as i64 + 1;
        for cell in cells {
            let Some((a, b)) = self.bed_in(cell) else {
                continue;
            };
            for x in (a.ceil() as i32).max(bx)..(b.floor() as i32).min(bx + bw) {
                let u = (self.base + x as i64) as i32;
                let edge = ((x as f32 - a).min(b - x as f32) / 6.0).min(1.0);
                let h = hash(u, front as i32, self.seed ^ 0xbed5);
                if (h & 0xff) as f32 / 255.0 > thick * edge {
                    continue;
                }
                let (base, stem) = if front {
                    (FEET + 4 + ((h >> 8) % 6) as i32, 1 + ((h >> 12) % 2) as i32)
                } else {
                    (FEET - 5 + ((h >> 8) % 5) as i32, 4 + ((h >> 12) % 3) as i32)
                };
                for r in 0..stem {
                    self.put(x, base - r, if r % 2 == 0 { leaf } else { leaf_dark });
                }
                self.put(x + 1 - 2 * ((h >> 16) & 1) as i32, base - stem / 2, leaf);
                self.blossom(x, base - stem - 1, h >> 20);
            }
        }
    }

    // ----- papel picado -----

    /// Everything a string may be tied to, in view and a long way beyond it,
    /// left to right: the big trees, and the pillars of the ruins.
    fn anchors(&self, out: &mut [Anchor; ANCHORS]) -> usize {
        let (x0, x1) = ((self.bx - 360) as f32, (self.bx + self.bw + 360) as f32);
        let mut n = 0;
        let mut add = |a: Anchor| {
            if n < ANCHORS {
                out[n] = a;
                n += 1;
            }
        };
        let cell = |x: f32| ((self.base as f64 + x as f64) / TREE_CELL as f64).floor() as i64;
        for c in cell(x0 - 40.0)..=cell(x1 + 40.0) {
            if let Some(t) = self.trunk_in(c) {
                let id = t.id as i32;
                let y = 52.0 + 4.0 * unit(id, 121, self.seed);
                add(Anchor {
                    x: t.x,
                    y,
                    id,
                    ruin: 0,
                });
            }
        }
        for r in self.ruins(x0, x1).as_slice() {
            // Poles on the pillars, tall enough to string from.
            let id = 1_000_000 + r.id * 2;
            add(Anchor {
                x: r.x - PILLAR,
                y: POLE_TOP,
                id,
                ruin: r.id + 1,
            });
            add(Anchor {
                x: r.x + PILLAR,
                y: POLE_TOP,
                id: id + 1,
                ruin: r.id + 1,
            });
        }
        out[..n].sort_unstable_by(|a, b| a.x.total_cmp(&b.x));
        n
    }

    /// Strings of papel picado from tree to tree, and to the ruins' pillars,
    /// sagging, with flags that swing in the wind.
    pub fn paint_papel(&mut self) {
        let Some(d) = self.pal.fest else { return };
        let mut a = [Anchor::default(); ANCHORS];
        let n = self.anchors(&mut a);
        for w in a[..n].windows(2) {
            let (p, q) = (w[0], w[1]);
            let len = q.x - p.x;
            if (p.ruin != 0 && p.ruin == q.ruin) || !(24.0..=STRING_MAX).contains(&len) {
                continue;
            }
            // Not every pair is joined, but the higher the holiday, the more.
            let key = hash(p.id, q.id, self.seed ^ 0x5171);
            if (key & 0xff) as f32 / 255.0 > 0.55 + 0.4 * d.level {
                continue;
            }
            self.string(p, q, key, 0, None, &d);
            if d.level >= 0.8 && (key >> 8) & 3 == 0 {
                self.string(p, q, key >> 3, 1, None, &d);
            }
        }
    }

    /// One string from `p` to `q`; `row` 1 is a lower, slacker one. Far ones
    /// are lost in the haze of the distance, by `haze`.
    fn string(&mut self, p: Anchor, q: Anchor, key: u32, row: i32, haze: Option<f32>, d: &Dress) {
        let len = q.x - p.x;
        let (drop, slack) = if row == 0 { (0.0, 1.0) } else { (9.0, 0.6) };
        let sag = (len * 0.05).clamp(3.0, 12.0) * slack;
        let (ya, yb) = (p.y + drop, q.y + drop);
        let y_at = |x: f32| {
            let t = ((x - p.x) / len).clamp(0.0, 1.0);
            ya + (yb - ya) * t + sag * 4.0 * t * (1.0 - t)
        };
        let twine = self.shade(Rgb(214, 186, 132), haze);
        let (lo, hi) = (self.bx, self.bx + self.bw);
        for x in (p.x.ceil() as i32).max(lo)..(q.x.floor() as i32).min(hi) {
            self.put(x, y_at(x as f32 + 0.5).round() as i32, twine);
        }
        let time = self.time();
        let gust = 0.8 + 0.5 * (time * 0.23 + (key % 97) as f32).sin();
        let step = if row == 0 { 9.0 } else { 11.0 };
        let phase = (key % 13) as f32;
        for k in 0..((len - 12.0) / step) as i32 {
            let fx = p.x + 6.0 + k as f32 * step;
            let ix = fx.round() as i32;
            if ix + 8 < lo || ix >= hi {
                continue;
            }
            let h = hash(k, key as i32, self.seed ^ 0xf1a6);
            // Five along the six, so that neighbors differ.
            let color = self.shade(d.flags[((k as u32 * 5 + (key >> 4)) % 6) as usize], haze);
            let top = y_at(ix as f32 + 4.0).round() as i32 + 1;
            let sway = gust * (time * 1.9 + fx * 0.07 + phase).sin();
            self.flag(ix, top, ((h >> 8) % 6) as usize, color, sway);
        }
    }

    /// `c` as lit for the hour, and `haze` of the way into the distance.
    fn shade(&self, c: Rgb, haze: Option<f32>) -> Rgb {
        match haze {
            Some(k) => self.far(c, k),
            None => self.lit(c),
        }
    }

    /// The strings of the middle layer, among the thin trunks that scroll
    /// behind the tiger's own ground: shorter, in the haze, and always about.
    pub fn paint_papel_far(&mut self, v: View) {
        let Some(d) = self.pal.fest else { return };
        if self.kind.snowy() {
            return;
        }
        let sh = self.shift(v, MID_P);
        let s = self.seed;
        let u0 = self.bx as i64 + sh - 100;
        let u1 = (self.bx + self.bw) as i64 + sh + 100;
        let mut prev: Option<Anchor> = None;
        for c in u0.div_euclid(MID_CELL) - 1..=u1.div_euclid(MID_CELL) + 1 {
            let ci = c as i32;
            if unit(ci, 51, s) > 0.62 {
                continue;
            }
            let x = (c * MID_CELL - sh) as f32 + unit(ci, 52, s) * MID_CELL as f32;
            let here = Anchor {
                x,
                y: 40.0 + 5.0 * unit(ci, 122, s),
                id: ci,
                ruin: 0,
            };
            if let Some(p) = prev {
                let key = hash(p.id, ci, s ^ 0x7f1a);
                if x - p.x >= 14.0 && (key & 0xff) as f32 / 255.0 < 0.4 + 0.3 * d.level {
                    self.string(p, here, key, 0, Some(0.22), &d);
                }
            }
            prev = Some(here);
        }
    }

    /// One flag, its top left corner on `(x, y)`, swung `sway` pixels at its
    /// bottom.
    fn flag(&mut self, x: i32, y: i32, pattern: usize, color: Rgb, sway: f32) {
        for (r, row) in FLAGS[pattern].iter().enumerate() {
            let dx = (sway * SWING[r] * 1.7).round() as i32;
            for (c, ch) in row.bytes().enumerate() {
                if ch == b'#' {
                    self.put(x + c as i32 + dx, y + r as i32, color);
                }
            }
        }
        #[cfg(test)]
        {
            self.painted.flags += 1;
        }
    }

    // ----- the saucer -----

    /// Marigold petals spiralling up the beam from the ground to the saucer,
    /// as many as the beam is strong. `top` is the row the beam starts on.
    pub fn beam_petals(&mut self, x: f32, top: i32, beam: f32) {
        let Some(d) = self.pal.fest else { return };
        let n = (28.0 * beam) as i32;
        let tick = self.tick as f32;
        let height = (FEET + 1 - top).max(1) as f32;
        for k in 0..n {
            let age = (tick * 0.012 + k as f32 / 28.0).fract();
            // Up, so the older the petal, the nearer the saucer.
            let y = FEET as f32 - age * height;
            let depth = 1.0 - age;
            let hw = (4.0 + 12.0 * depth) * 0.85;
            let px = x + hw * (age * 11.0 + k as f32 * 1.9).sin();
            let c = d.marigold[(k % 3) as usize];
            let (ix, iy) = (px.round() as i32, y.round() as i32);
            self.put(ix, iy, c);
            if k % 2 == 0 {
                self.put(ix + 1, iy, c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::tiger::art;

    /// Every drawing is a rectangle, and every letter in it has a color.
    #[test]
    fn the_drawings_fit_together() {
        let dress = Dress::classic(1.0);
        let alebrije = [
            art::ALEBRIJE_TORSO,
            art::ALEBRIJE_FRONT,
            art::ALEBRIJE_SLEEP_0,
            art::ALEBRIJE_SLEEP_1,
        ]
        .into_iter()
        .chain(art::ALEBRIJE_SKIN.heads);
        type Set = (&'static str, Vec<Sprite>, Vec<(u8, Rgb)>);
        let sets: [Set; 5] = [
            (
                "alebrije",
                alebrije.collect(),
                dress.alebrije(Kind::Bengal).into(),
            ),
            ("flags", FLAGS.to_vec(), vec![(b'#', Rgb::WHITE)]),
            ("ruin", vec![RUIN], STONE.into()),
            (
                "ofrenda",
                OFRENDA.iter().map(|&(s, _)| s).collect(),
                dress.ofrenda().into(),
            ),
            ("saucer", SAUCER.to_vec(), dress.saucer().into()),
        ];
        for (name, sprites, pal) in sets {
            for s in sprites {
                let w = s[0].len();
                for row in s {
                    assert_eq!(row.len(), w, "{name}: {row}");
                    for ch in row.bytes().filter(|&c| c != b'.') {
                        assert!(pal.iter().any(|&(k, _)| k == ch), "{name}: {}", ch as char);
                    }
                }
            }
        }
        // The ruin and its ofrenda are the sizes the placement counts on.
        assert_eq!(
            (RUIN[0].len(), RUIN.len()),
            (2 * RUIN_HALF as usize, RUIN_H as usize)
        );
        for (layer, _) in OFRENDA {
            assert_eq!(
                (layer[0].len(), layer.len()),
                (OFRENDA_0[0].len(), OFRENDA_0.len())
            );
        }
        assert_eq!(
            (SAUCER[0].len(), SAUCER[0].len()),
            (SAUCER[1].len(), SAUCER[1].len())
        );
        // A flag's rows swing as far as it has rows.
        assert!(
            FLAGS
                .iter()
                .all(|f| f.len() == SWING.len() && f[0].len() == 8)
        );
    }

    /// The torso's crest and the saucer stand taller than what they replace.
    #[test]
    fn the_taller_drawings_are_accounted_for() {
        let (a, b) = (art::ALEBRIJE_SKIN, art::TIGER_SKIN);
        assert_eq!(
            a.torso.len() as i32 + a.torso_at.1,
            b.torso.len() as i32 + b.torso_at.1
        );
        assert_eq!(a.torso[0].len(), b.torso[0].len());
        assert_eq!(a.front.len(), b.front.len());
        assert_eq!(
            SAUCER[0].len() as f32 - art::UFO[0].len() as f32,
            SAUCER_EXTRA
        );
        assert_eq!(SAUCER[0][0].len(), art::UFO[0][0].len());
    }
}
