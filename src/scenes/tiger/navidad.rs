//! Navidad in the jungle.
//!
//! The Guadalupe–Reyes in the forest: [`Dress`] holds the colors, from the
//! brief's own under `classic` and from the theme's accents otherwise. Strings
//! of foquitos run from tree to tree, their bulbs blinking in chase patterns,
//! with heno (Spanish moss, which is what a nacimiento's roof and hills are
//! made of) draped from the cords and the limbs; nochebuenas, which grow wild in
//! Mexico's forests, flower red in the grass and in beds of their own; and here
//! and there by the path stand the ruins with a nacimiento in their doorway
//! that the tiger calls on, farolitos lighting the way. The saucer is the
//! Estrella de Belén, a comet with a long tail, and from January 2 to 6 the
//! Reyes Magos cross the hills behind on a horse, a camel and an elephant. The
//! higher the holiday, the more of all of it: the posadas climb to Nochebuena,
//! when the Niño Dios is laid in his manger. Everything here is hashed from the
//! seed, like the jungle it dresses.

use super::art::Sprite;
use super::fiestas::by_hue;
use super::jungle::{FEET, Jungle, MID_CELL, MID_P, View};
use super::muertos::{
    ANCHORS, Anchor, FLAMA, OFRENDA_AT, RUIN, RUIN_FOOT, RUIN_H, RUIN_HALF, STONE, STRING_MAX, Sag,
};
use super::{Rgb, hash, rgb_of, unit};
use crate::scene::Ctx;

// ----- the colors of the holiday -----

const ROJO: Rgb = Rgb(200, 16, 46);
const VERDE: Rgb = Rgb(31, 111, 58);
const DORADO: Rgb = Rgb(242, 182, 50);
const PLATA: Rgb = Rgb(201, 204, 214);
const BLANCO: Rgb = Rgb(244, 241, 234);
const HENO: Rgb = Rgb(154, 165, 138);
const MUSGO: Rgb = Rgb(79, 122, 40);
const TEJOCOTE: Rgb = Rgb(232, 137, 28);
const CANA: Rgb = Rgb(201, 178, 107);
const CANELA: Rgb = Rgb(139, 74, 30);
/// The foquitos: rojo, verde, azul, amarillo, rosa, naranja.
const FOQUITOS: [Rgb; 6] = [
    Rgb(255, 59, 59),
    Rgb(59, 209, 111),
    Rgb(59, 123, 255),
    Rgb(255, 210, 59),
    Rgb(255, 95, 180),
    Rgb(255, 140, 26),
];
const TINTA: Rgb = Rgb(30, 22, 24);

/// The holiday's colors, and how far it has come.
#[derive(Clone, Copy, Debug)]
pub struct Dress {
    /// 0 to 1, as in `Fiesta::level`.
    pub level: f32,
    /// The Niño Dios lies in his manger: from Nochebuena on.
    pub nino: bool,
    /// The day of January the Reyes are on their way (2 to 6), and whether
    /// they have arrived (the 6th).
    pub reyes: Option<u8>,
    pub rojo: Rgb,
    pub verde: Rgb,
    pub dorado: Rgb,
    pub plata: Rgb,
    pub blanco: Rgb,
    pub azul: Rgb,
    /// The foquitos.
    pub bulbs: [Rgb; 6],
    pub heno: Rgb,
    pub musgo: Rgb,
    pub tejocote: Rgb,
    pub cana: Rgb,
    pub canela: Rgb,
    /// The ink of eyes and hair.
    pub deep: Rgb,
}

impl Dress {
    /// The brief's own colors.
    fn classic(level: f32, nino: bool, reyes: Option<u8>) -> Dress {
        Dress {
            level,
            nino,
            reyes,
            rojo: ROJO,
            verde: VERDE,
            dorado: DORADO,
            plata: PLATA,
            blanco: BLANCO,
            azul: FOQUITOS[2].scale(0.7),
            bulbs: FOQUITOS,
            heno: HENO,
            musgo: MUSGO,
            tejocote: TEJOCOTE,
            cana: CANA,
            canela: CANELA,
            deep: TINTA,
        }
    }

    /// The dress for today: the holiday's colors, or the theme's own where
    /// it has something close.
    pub fn new(ctx: &Ctx) -> Dress {
        let f = ctx.fiesta;
        let level = f.level.clamp(0.0, 1.0);
        let (month, day) = f.date();
        let nino = (month == 12 && day >= 24) || month == 1;
        let reyes = f.reyes().then_some(day);
        if ctx.theme.native {
            return Dress::classic(level, nino, reyes);
        }
        let t = ctx.theme;
        let cols: Vec<Rgb> = t.colors.iter().filter_map(|&c| rgb_of(c)).collect();
        if cols.is_empty() {
            return Dress::classic(level, nino, reyes);
        }
        let pick = |i: usize| cols[i % cols.len()];
        let rojo = by_hue(&cols, 355.0, 35.0, &[]).unwrap_or_else(|| pick(1));
        let verde = by_hue(&cols, 130.0, 55.0, &[rojo]).unwrap_or_else(|| pick(2));
        let dorado = by_hue(&cols, 45.0, 25.0, &[rojo, verde]).unwrap_or_else(|| pick(3));
        let azul = by_hue(&cols, 220.0, 45.0, &[rojo, verde, dorado]).unwrap_or_else(|| pick(4));
        let rosa = by_hue(&cols, 325.0, 35.0, &[rojo, verde, dorado, azul])
            .unwrap_or_else(|| rojo.lerp(Rgb::WHITE, 0.4));
        let naranja = by_hue(&cols, 28.0, 15.0, &[rojo, verde, dorado, azul, rosa])
            .unwrap_or_else(|| dorado.lerp(rojo, 0.5));
        let fg = rgb_of(t.fg).unwrap_or(BLANCO);
        Dress {
            level,
            nino,
            reyes,
            rojo,
            verde,
            dorado,
            plata: fg.scale(0.8),
            blanco: fg,
            azul: azul.scale(0.8),
            bulbs: [rojo, verde, azul, dorado, rosa, naranja],
            heno: verde.lerp(fg, 0.55),
            musgo: verde.scale(0.8),
            tejocote: naranja,
            cana: dorado.lerp(fg, 0.3),
            canela: dorado.scale(0.5),
            deep: verde.scale(0.18),
        }
    }

    /// The nacimiento's colors by letter: heno and musgo, wood, gold, white
    /// and the figures' robes.
    pub fn belen(&self) -> [(u8, Rgb); 31] {
        let skin = self.canela.lerp(self.blanco, 0.45);
        [
            (b'H', self.heno),
            (b'h', self.heno.scale(0.72)),
            (b'M', self.musgo),
            (b'm', self.musgo.scale(0.7)),
            (b'B', self.canela),
            (b'b', self.canela.scale(0.62)),
            (b'D', self.dorado),
            (b'd', self.dorado.scale(0.7)),
            (b'W', self.blanco),
            (b'w', self.blanco.scale(0.82)),
            (b'S', skin),
            (b's', skin.scale(0.78)),
            (b'R', self.rojo),
            (b'r', self.rojo.scale(0.7)),
            (b'G', self.verde),
            (b'g', self.verde.scale(0.65)),
            (b'A', self.azul),
            (b'a', self.azul.scale(0.7)),
            (b'K', self.deep),
            (b'O', self.canela.scale(0.9)),
            (b'o', self.canela.scale(0.6)),
            (b'U', self.plata.scale(0.62)),
            (b'u', self.plata.scale(0.42)),
            (b'P', self.plata),
            (b'T', self.tejocote),
            (b'n', self.rojo.lerp(self.blanco, 0.5)),
            (b'F', FLAMA.0),
            (b'f', FLAMA.1),
            (b'z', self.dorado.lerp(self.blanco, 0.6)),
            (b'C', self.cana),
            (b'L', self.dorado.lerp(self.blanco, 0.35)),
        ]
    }

    /// The Reyes' caravan by letter.
    pub fn caravan(&self) -> [(u8, Rgb); 13] {
        let skin = self.canela.lerp(self.blanco, 0.45);
        [
            (b'W', self.blanco),
            (b'w', self.blanco.scale(0.8)),
            (b'K', self.deep),
            (b'R', self.rojo),
            (b'G', self.verde),
            (b'D', self.dorado),
            (b'S', skin),
            (b'C', self.cana),
            (b'c', self.cana.scale(0.75)),
            (b'U', self.plata.scale(0.62)),
            (b'u', self.plata.scale(0.42)),
            (b'A', self.azul),
            (b'n', self.rojo.lerp(self.blanco, 0.5)),
        ]
    }

    /// The Estrella de Belén by letter: white-hot at the heart, gold at the
    /// tips.
    pub fn star(&self) -> [(u8, Rgb); 4] {
        [
            (b'W', Rgb::WHITE.lerp(self.blanco, 0.5)),
            (b'L', self.dorado.lerp(self.blanco, 0.55)),
            (b'D', self.dorado),
            (b'd', self.dorado.scale(0.72)),
        ]
    }
}

// ----- the drawings -----

/// The stable under its star: the roof of heno, the crib, María kneeling and José with his staff.
pub const BELEN_0: Sprite = &[
    "............................",
    ".............DD.............",
    ".............DD.............",
    "...........DDWWDD...........",
    ".............DD.............",
    ".............DD.............",
    "............................",
    "..........HHHHHHHH..........",
    "........HhHHHHhHHHhH........",
    "......HHhHHHhHHHHhHHHH......",
    ".....HHhHHhHHHhHHHhHHhHH....",
    "....HhHhHhH.hH.HhHhH.hHh....",
    ".....B................B.....",
    ".....B..............D.B.....",
    ".....BWWW.........KK.DB.....",
    ".....WWSW.........SS.BB.....",
    ".....BWWS.........GG.BB.....",
    ".....BAAAS.......GGGGBB.....",
    ".....AAAAA.......GgGGBB.....",
    ".....AaAAA.......GGGGBB.....",
    ".....AAAAA.hHHHHhbBBGBB.....",
    ".....aAAAa.BBBBBB.BB.BB.....",
    ".....AAAAA..B..B..BB.bB.....",
    "....HB..H..BH...BHbb.HB.H...",
    "..MHMMHMMHMMHMMHMMHMMHMMHM..",
    "..MMmMMMmMMMmMMMmMMMmMMMmM..",
    "..mMmmMmmMmmMmmMmmMmmMmmMm..",
    "............................",
];

/// The ox and the mule, a shepherd and a sheep.
pub const BELEN_1: Sprite = &[
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
    "............................",
    "............................",
    "............................",
    "............................",
    "..........OO....UU..........",
    "...BB....OOOO..UUUU.........",
    "...SS....oOWK..KWUU.........",
    "..RRRR...oWWn..nWUu.........",
    "..RRRR..............wWWw....",
    "..RrRR..............WWWWK...",
    "...KK...............KWWWK...",
    ".....................K.K....",
    "............................",
    "............................",
    "............................",
];

/// An angel, the garland under the eaves, lanterns and a second sheep.
pub const BELEN_2: Sprite = &[
    "............................",
    "............................",
    "............................",
    "............................",
    ".......D.WW.D...............",
    ".......DDWWDD...............",
    "........DWWD................",
    ".........WW.................",
    ".........WW.................",
    "............................",
    "............................",
    ".....GgGgGgGgGgGgGgGgGg.....",
    "......R..D..R..D..R..D......",
    "....B..................B....",
    "....f..................f....",
    "....F..................F....",
    "....f..................f....",
    "............................",
    "............................",
    "............................",
    "............................",
    "..................wWWw......",
    ".................WWWWK......",
    ".................KWWWK......",
    "..................K.K.......",
    "............................",
    "............................",
    "............................",
];

/// Nochebuenas in the doorway and esferas on the ground.
pub const BELEN_3: Sprite = &[
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
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "..R.R.R...............R.R.R.",
    "...RTR.................RTR..",
    "..RRRRR...............RRRRR.",
    "...GRG.................GRG..",
    "..GGGGG...............GGGGG.",
    ".........RD......DR.........",
    "............................",
    "............................",
];

/// The Niño Dios, in the crib.
pub const BELEN_NINO: Sprite = &[
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
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............DSSD............",
    "............WWWW............",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
    "............................",
];

/// The three Reyes Magos arrived at the pesebre on the 6th: Melchor in red, Gaspar in green and
/// Baltasar in blue, each with a gift.
pub const REYES: Sprite = &[
    ".DD..DD..DD.",
    ".SS..SS..SS.",
    "RRRRGGGGAAAA",
    "RDDRGDDGADDA",
    "RRRRGGGGAAAA",
    "RRRRGGGGAAAA",
    "RRRRGGGGAAAA",
    ".KK..KK..KK.",
];

/// Melchor's horse, facing right: `W` white, `K` its mane and hooves, `G` the saddle blanket, `R` the king's cape, `S` his face, `D` his crown.
pub const HORSE: Sprite = &[
    "......DD..........",
    "......SS.....WW...",
    ".....RRRR...KWWWW.",
    ".....RRRR.KWWWWWW.",
    ".....RRRR.KWWWWWWW",
    ".....RRRR.KWWW..Wn",
    ".KKWWGGGGWKWWW....",
    ".KKWWWWWWWWWW.....",
    ".KKWWWWWWWWWW.....",
    "...wwwwwwwwww.....",
    "...WW.w.w.WW......",
    "...WW.w.w.WW......",
    "...WW.w.w.WW......",
    "...KK.K.K.KK......",
];

/// Gaspar's camel: `C` and `c` its coat, `G` the king in green.
pub const CAMEL: Sprite = &[
    "........GDDG..........",
    "........GSSG.......CCC",
    "......GGGGGG.......CCC",
    "......GGGGGG......CCCc",
    ".....CGG....CCC...CCC.",
    ".....CCC....CCC...CC..",
    "...CCCCCCCCCCCCCCC....",
    ".ccCCCCCCCCCCCCCCC....",
    ".ccCCCCCCCCCCCCCCC....",
    ".ccCCCCCCCCCCCCCCC....",
    "...ccccccccccccccc....",
    "....Cc.Cc....Cc.Cc....",
    "....Cc.Cc....Cc.Cc....",
    "....Cc.Cc....Cc.Cc....",
    "....KK.KK....KK.KK....",
];

/// Baltasar's elephant: `U` and `u` its hide, `W` a tusk, `D` the howdah and `A` the king in blue.
pub const ELEPHANT: Sprite = &[
    "..........DD..............",
    ".........DDDD.............",
    ".........ASSA.............",
    ".........AAAA.............",
    ".....DDDDAAAADDDUUUUUUU...",
    "....UDDDDDDDDDDDUUUUUUU...",
    ".uuuUUUUUUUUUUUuuuUUUUU...",
    ".uuuUUUUUUUUUUUuuuUUUUU...",
    ".uuuUUUUUUUUUUUuuuUUUKU...",
    ".uuuUUUUUUUUUUUuuuUUWWU...",
    "....UUUUUUUUUUUUUUUUWWU...",
    "....UUUUuUUUuUUUUUUUWWUU..",
    ".....UUUuUUUuUUU.UUU..UU..",
    ".....UUUuUUUuUUU.UUU.UUU..",
    ".....uuu.uuu.uuu.uuu.uuU..",
];

/// The caravan, in the order it walks: the horse leads.
pub const CARAVAN: [Sprite; 3] = [HORSE, CAMEL, ELEPHANT];

/// The Estrella de Belén in one of its two phases of twinkle: `W` its heart, `L` `D` `d` gold toward the tips of its rays.
pub const STAR_0: Sprite = &[
    ".......d.......",
    ".......D.......",
    ".......D.......",
    "...d...L...d...",
    "....D..L..D....",
    ".....LLLLL.....",
    ".....LWWWL.....",
    "dDDLLLWWWLLLDDd",
    ".....LWWWL.....",
    ".....LLLLL.....",
    "....D..L..D....",
    "...d...L...d...",
    ".......D.......",
    ".......D.......",
    ".......d.......",
];

/// The Estrella de Belén in one of its two phases of twinkle: `W` its heart, `L` `D` `d` gold toward the tips of its rays.
pub const STAR_1: Sprite = &[
    "...............",
    ".d...........d.",
    "..D....d....D..",
    "...D...L...D...",
    "....D..L..D....",
    ".....LLLLL.....",
    ".....LWWWL.....",
    "..dLLLWWWLLLd..",
    ".....LWWWL.....",
    ".....LLLLL.....",
    "....D..L..D....",
    "...D...L...D...",
    "..D....d....D..",
    ".d...........d.",
    "...............",
];

/// The star, in its two phases.
pub const STAR: [Sprite; 2] = [STAR_0, STAR_1];

/// How much taller the star is than the saucer it replaces.
pub const COMET_EXTRA: f32 = 5.0;

/// The nacimiento's layers, and the level of the holiday each comes at.
const BELEN: [(Sprite, f32); 4] = [
    (BELEN_0, 0.0),
    (BELEN_1, 0.45),
    (BELEN_2, 0.8),
    (BELEN_3, 0.95),
];

/// The corbel doorway's edge, as the rows and the columns of the ruin it begins at on its left:
/// the right side is the same, mirrored.
const ARCH: [(i32, i32); 10] = [
    (8, 23),
    (10, 21),
    (12, 19),
    (14, 17),
    (16, 15),
    (19, 15),
    (22, 15),
    (25, 15),
    (28, 15),
    (31, 15),
];

/// Heno hanging from the corbels of the doorway: column, row, length.
const DRAPES: [(i32, i32, i32); 9] = [
    (25, 8, 4),
    (28, 8, 3),
    (30, 8, 4),
    (22, 10, 3),
    (33, 10, 3),
    (20, 12, 3),
    (35, 12, 3),
    (18, 14, 3),
    (37, 14, 3),
];

/// How long the Reyes take to cross the view, in seconds, and how far apart they walk.
const CARAVAN_PERIOD: f32 = 150.0;
const CARAVAN_GAP: f32 = 34.0;

impl Jungle {
    // ----- nacimientos in the ruins -----

    /// The ruins in view: farolitos lighting the way, the stone, and the nacimiento in its
    /// doorway, under a string of foquitos and heno.
    pub fn paint_belenes(&mut self) {
        let Some(d) = self.pal.navidad else { return };
        let ruins = self.ruins(self.bx as f32, (self.bx + self.bw) as f32);
        if ruins.as_slice().is_empty() {
            return;
        }
        let night = 1.0 - self.daylight;
        let stone = self.lut(&STONE);
        let belen = self.lut_glow(&d.belen(), 0.25 + 0.6 * night);
        for r in ruins.as_slice() {
            self.farolitos(r.x, &d);
            let face = if r.flip { -1.0 } else { 1.0 };
            self.sprite(RUIN, r.x, RUIN_FOOT as f32, face, &stone);
            self.poles(r, [d.rojo, d.verde]);
            let (x0, y0) = (
                r.x.round() as i32 - RUIN_HALF as i32 + OFRENDA_AT.0,
                RUIN_FOOT - RUIN_H + 1 + OFRENDA_AT.1,
            );
            #[cfg(test)]
            {
                self.painted.ruins += 1;
            }
            for &(layer, at) in &BELEN {
                if d.level < at {
                    break;
                }
                self.stamp(layer, x0, y0, &belen);
                #[cfg(test)]
                {
                    self.painted.belenes += 1;
                }
            }
            if d.nino {
                self.stamp(BELEN_NINO, x0, y0, &belen);
                #[cfg(test)]
                {
                    self.painted.ninos += 1;
                }
            }
            if d.reyes == Some(6) {
                // The kings kneel before the pesebre, a little way off.
                let kx = r.x.round() as i32 - RUIN_HALF as i32 - 20;
                self.stamp(REYES, kx, FEET - 8, &belen);
                #[cfg(test)]
                {
                    self.painted.reyes += 3;
                }
            }
            self.drapes(r.x, r.id, &d);
            if d.level >= 0.4 {
                self.arch_lights(r.x, r.id, &d);
            }
        }
    }

    /// Heno hanging from the corbels of the doorway.
    fn drapes(&mut self, rx: f32, id: i32, d: &Dress) {
        let (x0, y0) = (rx.round() as i32 - RUIN_HALF as i32, RUIN_FOOT - RUIN_H + 1);
        for (k, &(c, r, len)) in DRAPES.iter().enumerate() {
            self.tuft(x0 + c, y0 + r, len, id + k as i32, d, None);
        }
    }

    /// A string of foquitos around the doorway, blinking.
    fn arch_lights(&mut self, rx: f32, id: i32, d: &Dress) {
        let (x0, y0) = (rx.round() as i32 - RUIN_HALF as i32, RUIN_FOOT - RUIN_H + 1);
        let t = self.tick / 9;
        let mut k = 0u32;
        for &(r, c) in &ARCH {
            // The doorway is symmetric: its right edge is the left one mirrored.
            for x in [x0 + c, x0 + 2 * RUIN_HALF as i32 - 1 - c] {
                let color = d.bulbs[(k.wrapping_add(id as u32) % 6) as usize];
                let lit = !(t + k).is_multiple_of(3);
                self.bulb(x, y0 + r - 2, color, lit, None, d);
                k += 1;
            }
        }
    }

    /// Paper-bag lanterns along the ground either side of the ruin at `x`, lit from within: more
    /// as the holiday builds up.
    fn farolitos(&mut self, x: f32, d: &Dress) {
        let n = 1 + (4.0 * d.level) as i32;
        let night = 1.0 - self.daylight;
        let paper = d.blanco.lerp(d.dorado, 0.4);
        let (day, glow) = (self.lit(paper), paper.lerp(d.dorado, 0.35));
        let bag = day.lerp(glow, 0.3 + 0.7 * night);
        let tick = self.tick / 3;
        for side in [-1, 1] {
            for i in 0..n {
                let px = x.round() as i32 + side * (RUIN_HALF as i32 + 9 + 11 * i);
                if self.water(px as f32).is_some() {
                    continue;
                }
                let base = FEET - 1 + i % 2;
                let h = hash(px, tick as i32, self.seed ^ 0xfa01);
                let flame = FLAMA.0.lerp(FLAMA.1, (h % 3) as f32 * 0.3);
                for dx in -1..=1 {
                    self.put(px + dx, base - 2, bag);
                    self.put(px + dx, base, bag.scale(0.8));
                    self.put(px + dx, base - 1, if dx == 0 { flame } else { bag });
                }
                self.put(px, base - 3, flame);
                // The light spills out.
                let a = (0.1 + 0.3 * night) * (0.85 + 0.15 * (h >> 8 & 3) as f32 / 3.0);
                for (dx, dy) in [
                    (-2, -1),
                    (2, -1),
                    (-2, 0),
                    (2, 0),
                    (0, -4),
                    (-1, -3),
                    (1, -3),
                ] {
                    self.blend(px + dx, base + dy, d.dorado, a);
                }
                #[cfg(test)]
                {
                    self.painted.candles += 1;
                }
            }
        }
    }

    /// The warm light of the nacimientos, over everything near them.
    pub fn glow_belenes(&mut self, v: View) {
        let Some(d) = self.pal.navidad else { return };
        let ruins = self.ruins(v.x, v.x + v.w);
        let night = 1.0 - self.daylight;
        let tick = self.tick as f32;
        for r in ruins.as_slice() {
            let (cx, cy) = v.to_screen(r.x, (RUIN_FOOT - 12) as f32);
            let rad = ((18.0 + 8.0 * d.level) * v.z).max(3.0);
            let flicker =
                0.9 + 0.1 * (tick * 0.19 + r.id as f32 * 1.7).sin() * (tick * 0.067).cos();
            let a = (0.08 + 0.34 * night) * flicker;
            let ri = rad.ceil() as i32;
            for dy in -ri..=ri {
                for dx in -ri..=ri {
                    let q = ((dx * dx + dy * dy) as f32).sqrt() / rad;
                    if q < 1.0 {
                        self.blend_out(cx + dx, cy + dy, d.dorado, a * (1.0 - q) * (1.0 - q));
                    }
                }
            }
        }
    }

    // ----- foquitos and heno -----

    /// A bulb of a string with its socket on `(x, y + 1)`: lit, it shines in its own color and
    /// spills a little light; unlit, it is a dim glass.
    fn bulb(&mut self, x: i32, y: i32, c: Rgb, lit: bool, haze: Option<f32>, d: &Dress) {
        let socket = self.shade(d.verde.scale(0.5), haze);
        self.put(x, y + 1, socket);
        // A round glass: a plus sign, brightest at the middle.
        let glass = [(0, 2), (-1, 3), (0, 3), (1, 3), (0, 4)];
        if lit {
            let night = 1.0 - self.daylight;
            let bright = match haze {
                Some(k) => c.lerp(self.light.bot, k * 0.4),
                None => c,
            };
            for (dx, dy) in glass {
                self.put(x + dx, y + dy, bright);
            }
            self.put(x, y + 3, bright.lerp(Rgb::WHITE, 0.4));
            let a = 0.1 + 0.34 * night;
            for (dx, dy) in [(-2, 3), (2, 3), (-1, 2), (1, 2), (-1, 4), (1, 4), (0, 5)] {
                self.blend(x + dx, y + dy, c, a);
            }
            #[cfg(test)]
            {
                self.painted.bulbs += 1;
            }
        } else {
            let dim = self.shade(c.scale(0.4), haze);
            for (dx, dy) in glass {
                self.put(x + dx, y + dy, dim);
            }
        }
    }

    /// A tuft of heno hanging from `(x, y)`, `len` long, stirring in the wind.
    fn tuft(&mut self, x: i32, y: i32, len: i32, key: i32, d: &Dress, haze: Option<f32>) {
        let (a, b) = (
            self.shade(d.heno, haze),
            self.shade(d.heno.scale(0.74), haze),
        );
        let time = self.time();
        // Three strands, the middle one longest and lightest.
        for i in -1..=1i32 {
            let l = if i == 0 {
                len
            } else {
                len - 1 - ((key >> (i + 1)) & 1)
            };
            let color = if i == 0 { a } else { b };
            for r in 0..l {
                let sway = (time * 0.9 + key as f32 * 0.7 + r as f32 * 0.45 + i as f32).sin();
                let dx = (sway * r as f32 * 0.18).round() as i32;
                self.put(x + i + dx, y + r, if r == l - 1 { a } else { color });
            }
        }
        #[cfg(test)]
        {
            self.painted.heno += 1;
        }
    }

    /// A glass esfera hung from `(x, y)` on a thread.
    fn esfera(&mut self, x: i32, y: i32, c: Rgb, haze: Option<f32>) {
        let thread = self.shade(Rgb(214, 186, 132), haze);
        self.put(x, y + 1, thread);
        self.put(x, y + 2, thread);
        let body = self.shade(c, haze);
        let shine = self.shade(c.lerp(Rgb::WHITE, 0.6), haze);
        for dy in 0..3 {
            for dx in -1..=1 {
                self.put(x + dx, y + 3 + dy, body);
            }
        }
        self.put(x - 1, y + 3, shine);
    }

    /// Strings of foquitos from tree to tree, and to the ruins' pillars, with heno draped from the
    /// cords and an esfera here and there; and heno on the limbs of the big trees.
    pub fn paint_luces(&mut self) {
        let Some(d) = self.pal.navidad else { return };
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
            self.luces(p, q, key, 0, None, &d);
            if d.level >= 0.8 && (key >> 8) & 3 == 0 {
                self.luces(p, q, key >> 3, 1, None, &d);
            }
        }
        self.paint_henos(&d);
    }

    /// One string of foquitos from `p` to `q`; `row` 1 is a lower, slacker one. Far ones are lost
    /// in the haze of the distance, by `haze`.
    fn luces(&mut self, p: Anchor, q: Anchor, key: u32, row: i32, haze: Option<f32>, d: &Dress) {
        let len = q.x - p.x;
        let sag = Sag::new(p, q, row);
        let cord = self.shade(d.verde.scale(0.8), haze);
        let (lo, hi) = (self.bx, self.bx + self.bw);
        for x in (p.x.ceil() as i32).max(lo)..(q.x.floor() as i32).min(hi) {
            self.put(x, sag.at(x as f32 + 0.5).round() as i32, cord);
        }
        // Each string blinks in its own way: a chase of three, odds and evens, or at random.
        let tick = self.tick;
        let pattern = (key >> 5) % 3;
        let step = if row == 0 { 7.0 } else { 8.0 };
        for k in 0..((len - 8.0) / step) as i32 {
            let fx = p.x + 4.0 + k as f32 * step;
            let ix = fx.round() as i32;
            if ix < lo - 1 || ix > hi {
                continue;
            }
            let y = sag.at(fx + 0.5).round() as i32;
            let kk = k as u32;
            let lit = match pattern {
                0 => !(tick / 9 + kk).is_multiple_of(3),
                1 => (tick / 14 + kk).is_multiple_of(2),
                _ => {
                    hash(k, (tick / 11) as i32, self.seed ^ 0x11).is_multiple_of(3)
                        || kk.is_multiple_of(4)
                }
            };
            // Five along the six, so that neighbors differ.
            let color = d.bulbs[((kk * 5 + (key >> 4)) % 6) as usize];
            self.bulb(ix, y, color, lit, haze, d);
        }
        // Heno draped between the bulbs, and an esfera now and then.
        let drape = if row == 0 { 13.0 } else { 15.0 };
        for k in 0..((len - 10.0) / drape) as i32 {
            let fx = p.x + 6.5 + k as f32 * drape;
            let ix = fx.round() as i32;
            if ix < lo - 2 || ix >= hi + 2 {
                continue;
            }
            let h = hash(k, key as i32, self.seed ^ 0x4e0);
            let y = sag.at(fx + 0.5).round() as i32 + 1;
            if haze.is_some() {
                self.tuft(ix, y, 3 + (h % 3) as i32, h as i32, d, haze);
            } else if h.is_multiple_of(4) {
                let c = [d.rojo, d.dorado, d.plata, d.azul][((h >> 4) % 4) as usize];
                self.esfera(ix, y, c, haze);
            } else {
                self.tuft(ix, y, 4 + (h % 5) as i32, h as i32, d, haze);
            }
        }
    }

    /// The strings of the middle layer, among the thin trunks that scroll behind the tiger's own
    /// ground: shorter, in the haze, and always about.
    pub fn paint_luces_far(&mut self, v: View) {
        let Some(d) = self.pal.navidad else { return };
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
                    self.luces(p, here, key, 0, Some(0.22), &d);
                }
            }
            prev = Some(here);
        }
    }

    /// Heno hanging from the limbs of the big trees.
    fn paint_henos(&mut self, d: &Dress) {
        let s = self.seed;
        let (u0, u1) = (
            self.base as f64 + self.bx as f64 - 40.0,
            self.base as f64 + (self.bx + self.bw) as f64 + 40.0,
        );
        let cells = (u0 / super::jungle::TREE_CELL as f64).floor() as i64
            ..=(u1 / super::jungle::TREE_CELL as f64).floor() as i64;
        for cell in cells {
            let Some(t) = self.trunk_in(cell) else {
                continue;
            };
            let x0 = t.x.round() as i32 - t.w / 2;
            for (l, &(h, len, side)) in t.limbs.iter().enumerate() {
                let y0 = FEET - h - 10;
                let start = if side > 0 { x0 + t.w } else { x0 };
                let mut k = 5;
                while k < len {
                    let key = hash(t.id as i32 * 3 + l as i32, k, s ^ 0x4e1);
                    if (key & 0xff) as f32 / 255.0 < 0.5 + 0.4 * d.level {
                        let thick = if k < len / 2 { 3 } else { 2 };
                        let tuft = 4 + (key >> 8) as i32 % 8;
                        self.tuft(
                            start + side * k,
                            y0 - k / 3 + thick,
                            tuft,
                            key as i32,
                            d,
                            None,
                        );
                    }
                    k += 4;
                }
            }
        }
    }

    // ----- nochebuenas -----

    /// A nochebuena with its middle at `(x, y)`: the red bracts around the little yellow flowers,
    /// now and then a white or a pink one. The big ones grow in beds, the small ones in the grass.
    /// `h` is its dice.
    pub fn nochebuena(&mut self, x: i32, y: i32, h: u32, big: bool, d: &Dress) {
        // The wild ones are red; a white one now and then, and a rosa rarely.
        let (bract, shade) = match h % 16 {
            0 | 1 => (d.blanco, d.blanco.scale(0.8)),
            2 => (d.bulbs[4], d.bulbs[4].scale(0.75)),
            _ => (d.rojo, d.rojo.scale(0.72)),
        };
        let (bract, shade) = (self.lit(bract), self.lit(shade));
        let heart = self.lit(d.dorado);
        let leaf = self.lit(self.green(Rgb(46, 104, 52)));
        if big {
            for (dx, dy) in [(-2, -2), (0, -2), (2, -2), (-2, 0), (-1, 0), (1, 0), (2, 0)] {
                self.put(x + dx, y + dy, bract);
            }
            for (dx, dy) in [(-1, -1), (0, -1), (1, -1), (-1, 1), (0, 1), (1, 1)] {
                self.put(x + dx, y + dy, if dy > 0 { shade } else { bract });
            }
            self.put(x, y, heart);
            self.put(x - 1, y + 2, leaf);
            self.put(x + 1, y + 2, leaf);
        } else {
            self.put(x, y - 1, bract);
            self.put(x - 1, y, bract);
            self.put(x + 1, y, bract);
            self.put(x, y + 1, shade);
            self.put(x, y, heart);
        }
        #[cfg(test)]
        {
            self.painted.blossoms += 1;
        }
    }

    // ----- the Reyes Magos -----

    /// The Reyes Magos on their way, January 2 to 6: Melchor on a horse, Gaspar on a camel and
    /// Baltasar on an elephant, following their star over the hills behind the forest from one
    /// side of the view to the other, a little clearer each day.
    pub fn paint_reyes(&mut self, v: View) {
        let Some(d) = self.pal.navidad else { return };
        let Some(day) = d.reyes else { return };
        if self.kind.snowy() {
            return;
        }
        let haze = 0.52 - 0.06 * day.saturating_sub(2) as f32;
        let mut lut = [None; 128];
        for &(ch, c) in &d.caravan() {
            lut[ch as usize] = Some(self.far(c, haze));
        }
        let time = self.time();
        let from = (self.seed % 97) as f32 / 97.0;
        let u = (time / CARAVAN_PERIOD + from).fract();
        let reach = v.w + 40.0 + 3.0 * CARAVAN_GAP;
        let lead = v.x - 20.0 + u * reach;
        for (i, s) in CARAVAN.iter().enumerate() {
            let x = lead - i as f32 * CARAVAN_GAP;
            let ix = x.round() as i32;
            let feet = self.ridge(ix) + 12;
            // They bob along, each in its own step.
            let bob = ((time * 3.0 + i as f32 * 2.1).sin() * 0.6).round() as i32;
            let (w, h) = (s[0].len() as i32, s.len() as i32);
            let mut seen = false;
            for (r, row) in s.iter().enumerate() {
                for (c, ch) in row.bytes().enumerate() {
                    let Some(col) = lut[(ch & 127) as usize] else {
                        continue;
                    };
                    let (px, py) = (ix - w / 2 + c as i32, feet - h + r as i32 + bob);
                    if !self.behind_forest(px, py) {
                        self.put(px, py, col);
                        seen = true;
                    }
                }
            }
            #[cfg(test)]
            {
                self.painted.reyes += seen as u32;
            }
            let _ = seen;
        }
        // The star ahead of them, a little comet with its tail behind.
        let (sx, sy) = (lead + 30.0, (self.ridge(lead.round() as i32) - 18) as f32);
        let (gold, white) = (d.dorado, d.blanco);
        for i in 0..22 {
            let k = 1.0 - i as f32 / 22.0;
            let (px, py) = (
                (sx - 0.95 * i as f32).round() as i32,
                (sy - 0.32 * i as f32).round() as i32,
            );
            self.blend(px, py, gold, 0.8 * k * k);
            self.blend(px, py + 1, gold, 0.35 * k * k);
        }
        let (sx, sy) = (sx.round() as i32, sy.round() as i32);
        for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            self.put(
                sx + dx,
                sy + dy,
                if dx == 0 && dy == 0 { white } else { gold },
            );
        }
        for (dx, dy) in [(2, 0), (-2, 0), (0, 2), (0, -2)] {
            self.blend(sx + dx, sy + dy, gold, 0.6);
        }
    }

    // ----- the Estrella de Belén -----

    /// The tail of the comet from the star at `(x, y)`: broad and gold, thinning to nothing,
    /// glittering. It trails behind the star, so it follows where it has been.
    pub fn comet_tail(&mut self, d: &Dress, x: f32, y: f32) {
        let (vx, vy) = self.ufo.drift();
        let speed = (vx * vx + vy * vy).sqrt();
        // Hanging over the tiger it leans to one side, ten degrees over the horizon.
        let side = if self.ufo.visits.is_multiple_of(2) {
            1.0
        } else {
            -1.0
        };
        let rest = (0.82 * side, -0.57);
        let k = (speed / 1.2).min(1.0);
        let (ux, uy) = if speed > 1e-3 {
            (
                rest.0 * (1.0 - k) - vx / speed * k,
                rest.1 * (1.0 - k) - vy / speed * k,
            )
        } else {
            rest
        };
        let norm = (ux * ux + uy * uy).sqrt().max(1e-3);
        let (ux, uy) = (ux / norm, uy / norm);
        let len = 66.0 + 30.0 * k;
        let tick = self.tick;
        let sway = 0.03 * (tick as f32 * 0.05).sin();
        let (head, mid, end) = (
            d.dorado.lerp(Rgb::WHITE, 0.6),
            d.dorado,
            d.dorado.lerp(d.rojo, 0.4),
        );
        for i in 0..len as i32 {
            let t = i as f32 / len;
            // The tail bows a little, as if blown.
            let bow = 7.0 * t * t * side;
            let (px, py) = (
                x + ux * i as f32 - uy * bow * 0.3,
                y + uy * i as f32 + ux.abs() * bow * 0.3,
            );
            let (px, py) = (px + sway * i as f32, py);
            let w = 7.0 * (1.0 - t).powf(0.85) + 0.8;
            let c = if t < 0.25 {
                head.lerp(mid, t * 4.0)
            } else {
                mid.lerp(end, (t - 0.25) / 0.75)
            };
            let fade = (1.0 - t).powf(1.3);
            let (nx, ny) = (-uy, ux);
            let wi = w.ceil() as i32;
            for j in -wi..=wi {
                let across = 1.0 - (j as f32).abs() / w.max(0.5);
                if across <= 0.0 {
                    continue;
                }
                let a = 0.9 * fade * across.powf(0.7);
                let (qx, qy) = (px + nx * j as f32, py + ny * j as f32);
                self.blend(qx.round() as i32, qy.round() as i32, c, a);
            }
            // Glitter in it.
            let g = hash(i, (tick / 2) as i32, self.seed ^ 0x61);
            if i % 3 == 0 && g.is_multiple_of(4) {
                let off = ((g >> 4) % 5) as f32 - 2.0;
                let (qx, qy) = (px + nx * off, py + ny * off);
                self.put(qx.round() as i32, qy.round() as i32, head.lerp(c, t));
            }
        }
    }

    /// Escarcha and little stars rising up the beam from the ground to the star, as many as the
    /// beam is strong. `top` is the row the beam starts on.
    pub fn beam_glints(&mut self, x: f32, top: i32, beam: f32) {
        let Some(d) = self.pal.navidad else { return };
        let n = (24.0 * beam) as i32;
        let tick = self.tick as f32;
        let height = (FEET + 1 - top).max(1) as f32;
        for k in 0..n {
            let age = (tick * 0.012 + k as f32 / 24.0).fract();
            // Up, so the older the glint, the nearer the star.
            let y = FEET as f32 - age * height;
            let depth = 1.0 - age;
            let hw = (4.0 + 12.0 * depth) * 0.85;
            let px = x + hw * (age * 9.0 + k as f32 * 1.9).sin();
            let c = [d.blanco, d.dorado, d.dorado.lerp(d.blanco, 0.5)][(k % 3) as usize];
            let (ix, iy) = (px.round() as i32, y.round() as i32);
            self.put(ix, iy, c);
            if k % 3 == 0 && (self.tick / 4 + k as u32).is_multiple_of(3) {
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    self.blend(ix + dx, iy + dy, c, 0.6);
                }
            }
        }
    }

    /// The star's own glow and its four rays, straight onto the screen so that zooming out cannot
    /// blur them away.
    pub fn estrella_lights(&mut self, v: View, d: &Dress) {
        let (x, y) = (self.ufo.x, self.ufo.y + 5.0);
        let (sx, sy) = v.to_screen(x, y);
        let tick = self.tick as f32;
        let r = ((10.0 * v.z) as i32 + 2).min(80);
        for dy in -r..=r {
            for dx in -r..=r {
                let q = ((dx * dx + dy * dy) as f32).sqrt() / r as f32;
                if q < 1.0 {
                    self.blend_out(sx + dx, sy + dy, d.dorado, 0.45 * (1.0 - q) * (1.0 - q));
                }
            }
        }
        let reach = ((9.0 + 3.0 * (tick * 0.17).sin()) * v.z) as i32 + 3;
        for i in 1..=reach.min(120) {
            let a = 0.6 * (1.0 - i as f32 / reach as f32);
            for (dx, dy) in [(i, 0), (-i, 0), (0, i), (0, -i)] {
                self.blend_out(sx + dx, sy + dy, d.blanco, a);
            }
        }
    }

    /// The saucer's place is taken by the Estrella de Belén. First its tail, streaming behind it
    /// from the heart of the star, a few rows into the sprite that hangs from the beam's top.
    pub fn draw_estrella_tail(&mut self, d: &Dress) {
        let (x, y) = (self.ufo.x, self.ufo.y);
        self.comet_tail(d, x, y + 4.0);
        #[cfg(test)]
        {
            self.painted.comet = true;
        }
    }

    /// Then the star itself, over its beam.
    pub fn draw_estrella(&mut self, d: &Dress) {
        let (x, y) = (self.ufo.x, self.ufo.y);
        let mut lut = [None; 128];
        for &(ch, c) in &d.star() {
            lut[ch as usize] = Some(c);
        }
        let frame = ((self.tick / 8) % 2) as usize;
        self.sprite(STAR[frame], x, y + 11.0, 1.0, &lut);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::tiger::art;

    /// Every drawing is a rectangle, and every letter in it has a color.
    #[test]
    fn the_drawings_fit_together() {
        let dress = Dress::classic(1.0, true, Some(6));
        type Set = (&'static str, Vec<Sprite>, Vec<(u8, Rgb)>);
        let sets: [Set; 3] = [
            (
                "belen",
                BELEN
                    .iter()
                    .map(|&(s, _)| s)
                    .chain([BELEN_NINO, REYES])
                    .collect(),
                dress.belen().into(),
            ),
            ("caravan", CARAVAN.to_vec(), dress.caravan().into()),
            ("star", STAR.to_vec(), dress.star().into()),
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
        // The layers of the nacimiento are the size of the doorway they stand in.
        for (layer, _) in BELEN {
            assert_eq!((layer[0].len(), layer.len()), (28, 28));
        }
        assert_eq!((BELEN_NINO[0].len(), BELEN_NINO.len()), (28, 28));
        // The star stands taller than the saucer, and as the camera counts on.
        assert_eq!(STAR[0].len() as f32 - art::UFO[0].len() as f32, COMET_EXTRA);
        assert_eq!(STAR[0].len(), STAR[1].len());
        assert_eq!(STAR[0][0].len(), STAR[0].len());
    }
}
