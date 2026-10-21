//! dog: a dog runs, naps and chases butterflies in a meadow.
//!
//! Adapted from the `dog` animation of tanim,
//! <https://github.com/SantiagoSchez/tanim>, which is MIT licensed:
//!
//! ```text
//! MIT License
//!
//! Copyright (c) 2026 Santiago Sánchez
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//! ```
//!
//! The sprites, the palette, the world and the dog's moods follow tanim's. The
//! world is a fixed 128 pixels tall and a camera trailing the dog projects it
//! onto the terminal. tanim steps the world 20 times a second; here an
//! accumulator does the same at any frame rate, scaled by `speed`. The clocks
//! (the day, how long a nap lasts) follow the wall clock instead.
//!
//! On Día de Muertos the dog is a xoloitzcuintle, the hairless dog who guides
//! souls to Mictlán: slate grey, bat-eared, tufted, with a collar of
//! cempasúchil. A path of marigold petals, trodden clear of grass and lined
//! with veladoras, winds across the meadow to a small ofrenda by the fence
//! (three tiers in papel picado colors, candles, a photo, sugar skulls, pan de
//! muerto, water and salt, under an arch of cempasúchil that goes up as the
//! days go by). The meadow's flowers turn to cempasúchil, terciopelo and nube.
//! By day the souls are monarchs fluttering about the path; by night they are
//! small lights strung out behind the xolo, and when one has walked with it a
//! good while and passes the ofrenda it rises over the candles and is home.
//! The holiday rolls its own dice, so ordinary days are untouched.

use std::f32::consts::PI;

use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "dog",
    aliases: &["meadow"],
    about: "a dog runs, naps and chases butterflies in a meadow, after tanim",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Pace of the dog, the clouds and the grass (the day keeps real time)",
        },
        Opt {
            key: "day",
            default: Val::I(180),
            help: "Real seconds for a full day, from dawn through night to dawn (10-86400)",
        },
        Opt {
            key: "zoom",
            default: Val::I(2),
            help: "How close the camera is to the dog, 0 (whole meadow) to 6",
        },
        Opt {
            key: "butterflies",
            default: Val::B(true),
            help: "Let butterflies pass by by day for the dog to chase",
        },
        Opt {
            key: "sky",
            default: Val::B(true),
            help: "Paint the sky; off leaves it see-through above the hills",
        },
    ],
    make: |ctx| Box::new(DogScene::new(ctx)),
};

/// An sRGB color, as tanim's canvas has it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rgb(u8, u8, u8);

impl Rgb {
    const BLACK: Rgb = Rgb(0, 0, 0);
    const WHITE: Rgb = Rgb(255, 255, 255);

    /// Linear interpolation towards `o`; `t` is clamped to `[0, 1]`.
    #[inline]
    fn lerp(self, o: Rgb, t: f32) -> Rgb {
        let t = t.clamp(0.0, 1.0);
        let f = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t + 0.5) as u8;
        Rgb(f(self.0, o.0), f(self.1, o.1), f(self.2, o.2))
    }

    /// Multiplies every channel by `k` (saturating).
    #[inline]
    fn scale(self, k: f32) -> Rgb {
        let f = |a: u8| (a as f32 * k).clamp(0.0, 255.0) as u8;
        Rgb(f(self.0), f(self.1), f(self.2))
    }

    /// Perceived brightness in `[0, 255]`.
    #[inline]
    fn luma(self) -> u8 {
        ((self.0 as u32 * 54 + self.1 as u32 * 183 + self.2 as u32 * 19) >> 8) as u8
    }
}

/// The scene's color type for an [`Rgb`].
#[inline]
fn col(c: Rgb) -> Color {
    Color::Rgb(c.0, c.1, c.2)
}

/// An [`Rgb`] for a theme color, when it has one (palette entries do not).
fn rgb_of(c: Color) -> Option<Rgb> {
    match c {
        Color::Rgb(r, g, b) => Some(Rgb(r, g, b)),
        _ => None,
    }
}

/// Marks sky pixels of the world while the sky is see-through.
const KEY: Rgb = Rgb(255, 0, 255);

/// The dice tanim's code rolls, on top of fastrand.
trait Dice {
    /// An integer in `[lo, hi)`, or `lo` when the range is empty.
    fn range(&mut self, lo: i32, hi: i32) -> i32;
    fn rangef(&mut self, lo: f32, hi: f32) -> f32;
    fn chance(&mut self, p: f32) -> bool;
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T;
}

impl Dice for fastrand::Rng {
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        if hi <= lo { lo } else { self.i32(lo..hi) }
    }

    fn rangef(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.f32()
    }

    fn chance(&mut self, p: f32) -> bool {
        self.f32() < p
    }

    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.usize(..items.len())]
    }
}

type Rng = fastrand::Rng;

/// Pixel-art rows facing right, the last row standing on the ground.
/// `B` coat, `D` patches, `w` cream, `k` eye and nose, `c` collar, `p` tongue.
type Sprite = &'static [&'static str];

const SPRITE_W: i32 = 22;

const STAND: Sprite = &[
    "................BBB...",
    "...............BBBBB..",
    "..w...........DBBkBBB.",
    "..B...........DDBBBBBk",
    "..B...........DDBBwww.",
    "..BB..........ccBww...",
    "...BBDDDDDDBBBccBw....",
    "...BBBDDDDBBBBBBBw....",
    "...BBBBBBBBBBBBBBw....",
    "...BBwwwwwwwwwBBw.....",
    "...BB.BB.....BB.BB....",
    "...BB.BB.....BB.BB....",
    "...ww.ww.....ww.ww....",
];

const STAND_L: Sprite = &[
    "................BBB...",
    "...............BBBBB..",
    "w.............DBBkBBB.",
    ".B............DDBBBBBk",
    ".B............DDBBwww.",
    "..BB..........ccBww...",
    "...BBDDDDDDBBBccBw....",
    "...BBBDDDDBBBBBBBw....",
    "...BBBBBBBBBBBBBBw....",
    "...BBwwwwwwwwwBBw.....",
    "...BB.BB.....BB.BB....",
    "...BB.BB.....BB.BB....",
    "...ww.ww.....ww.ww....",
];

const STAND_R: Sprite = &[
    "................BBB...",
    "...............BBBBB..",
    "....w.........DBBkBBB.",
    "...B..........DDBBBBBk",
    "...B..........DDBBwww.",
    "..BB..........ccBww...",
    "...BBDDDDDDBBBccBw....",
    "...BBBDDDDBBBBBBBw....",
    "...BBBBBBBBBBBBBBw....",
    "...BBwwwwwwwwwBBw.....",
    "...BB.BB.....BB.BB....",
    "...BB.BB.....BB.BB....",
    "...ww.ww.....ww.ww....",
];

const RUN_A: Sprite = &[
    "................BBB...",
    "...............BBBBB..",
    "..............DBBkBBB.",
    "..............DDBBBBBk",
    "..............DDBBwww.",
    "wB............ccBww...",
    ".BBBBDDDDDDBBBccBw....",
    "...BBBDDDDBBBBBBBw....",
    "...BBBBBBBBBBBBBBw....",
    "...BBwwwwwwwwwBBw.....",
    "..BB...........BB.....",
    ".BB.............BB....",
    "ww...............ww...",
];

const RUN_B: Sprite = &[
    "................BBB...",
    "...............BBBBB..",
    "..............DBBkBBB.",
    "..............DDBBBBBk",
    "..............DDBBwww.",
    "wB............ccBww...",
    ".BBBBDDDDDDBBBccBw....",
    "...BBBDDDDBBBBBBBw....",
    "...BBBBBBBBBBBBBBw....",
    "...BBwwwwwwwwwBBw.....",
    "...BBB.......BBB......",
    "...B.BB.....BB.B......",
    "...w..ww...ww..w......",
];

const RUN_C: Sprite = &[
    "................BBB...",
    "...............BBBBB..",
    "..............DBBkBBB.",
    "..............DDBBBBBk",
    "..............DDBBwww.",
    "wB............ccBww...",
    ".BBBBDDDDDDBBBccBw....",
    "...BBBDDDDBBBBBBBw....",
    "...BBBBBBBBBBBBBBw....",
    "...BBwwwwwwwwwBBw.....",
    "....BB......BB........",
    ".....BB....BB.........",
    "......ww..ww..........",
];

const SIT: Sprite = &[
    "..............BBB.....",
    ".............BBBBB....",
    "............DBBkBBB...",
    "............DDBBBBBk..",
    "............DDBBwwww..",
    "............ccBBwp....",
    "...........BccBBwp....",
    "..........BBBBBBw.....",
    ".........BBBBBBBw.....",
    "........BBDDBBBBw.....",
    "........BDDDBBBBw.....",
    ".wBB...BBBDDBBBBw.....",
    "...BBBBBBBBBBwwww.....",
];

const SIT_PANT: Sprite = &[
    "..............BBB.....",
    ".............BBBBB....",
    "............DBBkBBB...",
    "............DDBBBBBk..",
    "............DDBBwwww..",
    "............ccBBwp....",
    "...........BccBBwpp...",
    "..........BBBBBBw.....",
    ".........BBBBBBBw.....",
    "........BBDDBBBBw.....",
    "........BDDDBBBBw.....",
    "..BB...BBBDDBBBBw.....",
    "wBBBBBBBBBBBwwww......",
];

const LIE: Sprite = &[
    "..............BBB.....",
    ".............DBBBB....",
    "........DDDDDDBDBBBk..",
    "wBB...BBDDDDBDBBBwww..",
    "..BBBBBBBBBBBBBBww....",
    "..BBBBBBBBBBBBBBw.....",
    "...BBBBwwwwwwwwwwww...",
];

const SNIFF: Sprite = &[
    "..w...................",
    "..B...................",
    "..B...................",
    "..BB..................",
    "...BBDDDDDDBBB........",
    "...BBBDDDDBBBBBcc.....",
    "...BBBBBBBBBBBBccBB...",
    "...BBwwwwwwwwwBBDBBB..",
    "...BB.BB.....BBDDBkBB.",
    "...BB.BB.....BBDDBBBB.",
    "...ww.ww.....ww...wwwk",
];

const SNIFF_WAG: Sprite = &[
    ".w....................",
    ".B....................",
    "..B...................",
    "..BB..................",
    "...BBDDDDDDBBB........",
    "...BBBDDDDBBBBBcc.....",
    "...BBBBBBBBBBBBccBB...",
    "...BBwwwwwwwwwBBDBBB..",
    "...BB.BB.....BBDDBkBB.",
    "...BB.BB.....BBDDBBBB.",
    "...ww.ww.....ww...wwwk",
];

const RUN: [Sprite; 4] = [RUN_A, RUN_B, RUN_C, RUN_B];
const WAG: [Sprite; 4] = [STAND, STAND_L, STAND, STAND_R];

// The xoloitzcuintle of Día de Muertos, frame for frame and row for row like
// the dog above. `B` slate skin, `h` its sheen, `D` the far legs, `E` and `e`
// the bat ears and their pink inside, `t` the tufts on the head and the tail,
// `k` eye and nose, `c` `o` `m` the cempasúchil collar, `p` tongue.

const XOLO_STAND: Sprite = &[
    ".............EE.......",
    ".............EeE.t....",
    ".............EeEttt...",
    ".............BeBBkBB..",
    ".............BBBBBBBBk",
    ".t...........ocoBBBB..",
    "..B.hhhhhhhhhmcoB.....",
    "...BBBBBBBBBBBBBBB....",
    "...BBBBB....BBBBBB....",
    "...DDBBB.....DDBB.....",
    "...DD.BB.....DD.BB....",
    "...DD.BB.....DD.BB....",
    "...DD.hh.....DD.hh....",
];

const XOLO_STAND_L: Sprite = &[
    ".............EE.......",
    ".............EeE.t....",
    ".............EeEttt...",
    ".............BeBBkBB..",
    "t............BBBBBBBBk",
    ".B...........ocoBBBB..",
    "..B.hhhhhhhhhmcoB.....",
    "...BBBBBBBBBBBBBBB....",
    "...BBBBB....BBBBBB....",
    "...DDBBB.....DDBB.....",
    "...DD.BB.....DD.BB....",
    "...DD.BB.....DD.BB....",
    "...DD.hh.....DD.hh....",
];

const XOLO_STAND_R: Sprite = &[
    ".............EE.......",
    ".............EeE.t....",
    ".............EeEttt...",
    ".............BeBBkBB..",
    "..t..........BBBBBBBBk",
    "..B..........ocoBBBB..",
    "..B.hhhhhhhhhmcoB.....",
    "...BBBBBBBBBBBBBBB....",
    "...BBBBB....BBBBBB....",
    "...DDBBB.....DDBB.....",
    "...DD.BB.....DD.BB....",
    "...DD.BB.....DD.BB....",
    "...DD.hh.....DD.hh....",
];

const XOLO_RUN_A: Sprite = &[
    ".............EE.......",
    ".............EeE.t....",
    ".............EeEttt...",
    ".............BeBBkBB..",
    ".............BBBBBBBBk",
    ".............ocoBBBB..",
    "tBBBhhhhhhhhhmcoB.....",
    "...BBBBBBBBBBBBBBB....",
    "...BBBBB....BBBBBB....",
    "..DBBB........BBB.....",
    ".DBB...........DBB....",
    "DB..............DBB...",
    "h................Dhh..",
];

const XOLO_RUN_B: Sprite = &[
    ".............EE.......",
    ".............EeE.t....",
    ".............EeEttt...",
    ".............BeBBkBB..",
    ".............BBBBBBBBk",
    ".............ocoBBBB..",
    "tBBBhhhhhhhhhmcoB.....",
    "...BBBBBBBBBBBBBBB....",
    "...BBBBB....BBBBBB....",
    "...DBBB......DBBB.....",
    "...D.BB.....DD.BB.....",
    "...D..BB...DD..B......",
    "...h..hh...hh..h......",
];

const XOLO_RUN_C: Sprite = &[
    ".............EE.......",
    ".............EeE.t....",
    ".............EeEttt...",
    ".............BeBBkBB..",
    ".............BBBBBBBBk",
    ".............ocoBBBB..",
    "tBBBhhhhhhhhhmcoB.....",
    "...BBBBBBBBBBBBBBB....",
    "...BBBBB....BBBBBB....",
    "....DBB......DBB......",
    ".....DBB....DBB.......",
    "......DB...BD.........",
    "......hh..hh..........",
];

const XOLO_SIT: Sprite = &[
    "............EE........",
    "............EeE.t.....",
    "............EeEttt....",
    "............BeBBkBB...",
    "............BBBBBBBBk.",
    "............ocoBBBp...",
    "...........BmcoBBp....",
    "...........BBBBBB.....",
    "..........hBBBBBB.....",
    ".........hBBBBBBB.....",
    "........hBBBBDBB......",
    ".t.....BBBBBBDBB......",
    "..BBBBBBBBBBDDhhh.....",
];

const XOLO_SIT_PANT: Sprite = &[
    "............EE........",
    "............EeE.t.....",
    "............EeEttt....",
    "............BeBBkBB...",
    "............BBBBBBBBk.",
    "............ocoBBBpp..",
    "...........BmcoBBpp...",
    "...........BBBBBB.....",
    "..........hBBBBBB.....",
    ".........hBBBBBBB.....",
    "........hBBBBDBB......",
    ".......BBBBBBDBB......",
    "tBBBBBBBBBBBDDhhh.....",
];

const XOLO_LIE: Sprite = &[
    "..............EE......",
    "..............EeE.t...",
    "........hhhhh.BeBBtB..",
    "tBB...BBBBBBBBBBBBBBk.",
    "..BBBBBBBBBBBmcoBBB...",
    "..BBBBBBBBBBBBBBBB....",
    "...DDBBBBBBBBBBhhhhh..",
];

const XOLO_SNIFF: Sprite = &[
    "..t...................",
    "..B...................",
    "..B...................",
    "...B..................",
    "...BhhhhhhhhhB........",
    "...BBBBBBBBBBBBmc.....",
    "...BBBBB....BBBcoEE...",
    "...DDBBB.....DBBBEeEt.",
    "...DD.BB.....DD.BBBkBB",
    "...DD.BB.....DD..BBBBB",
    "...DD.hh.....DD...BBBk",
];

const XOLO_SNIFF_WAG: Sprite = &[
    ".t....................",
    ".B....................",
    "..B...................",
    "...B..................",
    "...BhhhhhhhhhB........",
    "...BBBBBBBBBBBBmc.....",
    "...BBBBB....BBBcoEE...",
    "...DDBBB.....DBBBEeEt.",
    "...DD.BB.....DD.BBBkBB",
    "...DD.BB.....DD..BBBBB",
    "...DD.hh.....DD...BBBk",
];

const XOLO_RUN: [Sprite; 4] = [XOLO_RUN_A, XOLO_RUN_B, XOLO_RUN_C, XOLO_RUN_B];
const XOLO_WAG: [Sprite; 4] = [XOLO_STAND, XOLO_STAND_L, XOLO_STAND, XOLO_STAND_R];

/// The ofrenda, seen from the front. `Y` `Z` `X` the arch of cempasúchil,
/// built up from the bottom as the holiday comes; `1` `2` `3` the cloths of
/// the three tiers and `W` their white edges; `P` the photo's frame, `q` and
/// `u` the photo; `v` candles and `f` their flames; `s` sugar skulls with `k`
/// eyes and `R` `t` `y` icing; `b` pan de muerto with `n` its bones (and the
/// salt); `a` a glass of water; `o` `O` cempasúchil and `l` their stems in `g`
/// clay pots.
const OFRENDA: Sprite = &[
    "..........XZYZYZX..........",
    ".......YZX.......XZY.......",
    ".....ZX.............XZ.....",
    "....Y.....PPPPPPP.....Y....",
    "...Z......PquuuqP......Z...",
    "...X....f.PqquqqP.f....X...",
    "..Y.....v.PuuuuuP.v.....Y..",
    "..Z.....v.PPPPPPP.v.....Z..",
    "..X....WWWWWWWWWWWWW....X..",
    "..Y....1111111111111....Y..",
    "..Z..f.1.1.1.1.1.1.1.f..Z..",
    "..X..v.sRs.obnbo.sRs.v..X..",
    "..Y..v.ksk.lbbbl.ksk.v..Y..",
    "..Z..WWWWWWWWWWWWWWWWW..Z..",
    "..X..22222222222222222..X..",
    "..Y..2.f.2.2.2.2.2.f.2..Y..",
    "..ZoOo.v.sts.a.sts.v.oOoZ..",
    "..Xggg.v.kyknankyk.v.gggX..",
    "..WWWWWWWWWWWWWWWWWWWWWWW..",
    "..33333333333333333333333..",
    "..33333333333333333333333..",
    "..3.3.3.3.3.3.3.3.3.3.3.3..",
];
const OFRENDA_W: i32 = 27;
/// Rows of the arch's posts and crown, counted from the top.
const ARCH_ROWS: i32 = 18;

/// Height of the world in pixels; its width follows the terminal's aspect.
const WORLD_H: i32 = 128;
/// Camera zoom levels, as the number of world pixel rows in view.
const ZOOMS: [f32; 7] = [128.0, 106.0, 88.0, 72.0, 58.0, 46.0, 36.0];
/// World pixels kept clear around the dog at every zoom level.
const CAM_MARGIN: f32 = 3.0;
/// Tallest the dog gets: the tallest sprite plus the running bob.
const DOG_H: f32 = 14.0;
/// Highest the dog leaps when chasing a butterfly.
const MAX_HOP: f32 = 11.0;

/// Coat, patches and cream.
const COATS: [(Rgb, Rgb, Rgb); 4] = [
    (Rgb(200, 134, 70), Rgb(122, 74, 40), Rgb(246, 230, 202)),
    (Rgb(226, 174, 94), Rgb(184, 124, 62), Rgb(250, 236, 200)),
    (Rgb(156, 158, 168), Rgb(86, 88, 100), Rgb(242, 242, 244)),
    (Rgb(136, 90, 58), Rgb(88, 56, 36), Rgb(234, 206, 174)),
];
const COLLARS: [Rgb; 4] = [
    Rgb(206, 40, 52),
    Rgb(40, 110, 210),
    Rgb(40, 160, 90),
    Rgb(240, 180, 30),
];
const EYE: Rgb = Rgb(28, 22, 20);
const TONGUE: Rgb = Rgb(236, 104, 128);

const NIGHT_TOP: Rgb = Rgb(6, 8, 26);
const NIGHT_BOT: Rgb = Rgb(24, 28, 62);
const NIGHT_TINT: Rgb = Rgb(78, 88, 140);
const DAY_TOP: Rgb = Rgb(70, 140, 225);
const DAY_BOT: Rgb = Rgb(186, 218, 246);

const FAR_HILL: Rgb = Rgb(118, 156, 138);
const MEADOW_TOP: Rgb = Rgb(106, 172, 74);
const MEADOW_BOT: Rgb = Rgb(56, 122, 42);
const BLADES: [Rgb; 4] = [
    Rgb(92, 170, 64),
    Rgb(74, 150, 54),
    Rgb(116, 184, 78),
    Rgb(62, 136, 48),
];
const PETALS: [Rgb; 6] = [
    Rgb(250, 250, 250),
    Rgb(250, 214, 70),
    Rgb(236, 72, 100),
    Rgb(170, 120, 236),
    Rgb(250, 150, 190),
    Rgb(250, 140, 60),
];
const WINGS: [Rgb; 4] = [
    Rgb(250, 150, 40),
    Rgb(250, 250, 240),
    Rgb(250, 220, 60),
    Rgb(90, 160, 250),
];
const WOOD: Rgb = Rgb(164, 122, 78);
const POST: Rgb = Rgb(132, 96, 60);
const TRUNK: Rgb = Rgb(104, 72, 46);
const LEAVES: Rgb = Rgb(52, 112, 52);
const LEAVES_LIT: Rgb = Rgb(84, 150, 70);

// Día de Muertos.
const XOLO_SKIN: Rgb = Rgb(78, 84, 98);
const XOLO_SHEEN: Rgb = Rgb(106, 113, 130);
const XOLO_FAR: Rgb = Rgb(50, 54, 66);
const XOLO_EAR: Rgb = Rgb(64, 68, 82);
const XOLO_EAR_IN: Rgb = Rgb(196, 128, 140);
const TUFT: Rgb = Rgb(220, 214, 204);
/// Cempasúchil: orange, light and dark.
const CEMPASUCHIL: [Rgb; 3] = [Rgb(255, 140, 0), Rgb(255, 176, 30), Rgb(214, 98, 0)];
const TERCIOPELO: Rgb = Rgb(176, 0, 72);
const HUESO: Rgb = Rgb(244, 238, 226);
const FLAMA: Rgb = Rgb(255, 214, 120);
const FLAMA_EDGE: Rgb = Rgb(255, 150, 40);
/// The ofrenda's cloths, in papel picado colors: rosa mexicano, morado, turquesa.
const CLOTHS: [Rgb; 3] = [Rgb(228, 0, 124), Rgb(112, 40, 160), Rgb(0, 170, 184)];
const MONARCH: Rgb = Rgb(244, 124, 14);
const MONARCH_VEIN: Rgb = Rgb(34, 24, 22);
const SOUL: Rgb = Rgb(255, 244, 220);
const SOUL_GLOW: Rgb = Rgb(255, 178, 92);
/// Seeds the holiday's own dice, so ordinary days keep theirs.
const MUERTOS_SEED: u64 = 0x0D1A_DE3E_3705;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Run,
    Stand,
    Sit,
    Nap,
    Sniff,
    Chase,
}

#[derive(Clone, Copy)]
struct Dog {
    /// Left edge of the sprite, in world pixels.
    x: f32,
    /// 1 facing right, -1 facing left.
    dir: f32,
    state: State,
    timer: u32,
    /// Real seconds of nap left.
    nap: f32,
    target: f32,
    /// Turn around when the current pause ends.
    turn: bool,
    hop: f32,
    vy: f32,
    anim: u32,
    coat: (Rgb, Rgb, Rgb),
    collar: Rgb,
}

struct Butterfly {
    x: f32,
    y: f32,
    tx: f32,
    ty: f32,
    /// Direction it flees in once the dog gives chase; 0 while wandering.
    flee: f32,
    speed: f32,
    life: u32,
    color: Rgb,
}

struct Blade {
    x: i32,
    y: i32,
    h: i32,
    color: Rgb,
    /// Petal color and whether it is a big (cross-shaped) bloom.
    flower: Option<(Rgb, bool)>,
}

struct Cloud {
    x: f32,
    y: f32,
    v: f32,
    puffs: Vec<(f32, f32, f32)>,
}

struct Tree {
    x: i32,
    trunk: i32,
    r: i32,
}

struct Star {
    x: i32,
    y: i32,
    phase: f32,
}

struct Firefly {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    phase: f32,
}

struct Zzz {
    x: f32,
    y: f32,
    age: u32,
}

/// A veladora beside the petal path: its glass stands on `(x, y)`.
struct Candle {
    x: i32,
    y: i32,
    glass: Rgb,
}

/// A soul come home for the holiday: a monarch by day, and by night a
/// glowing light that follows the xolo along the path to the ofrenda.
struct Spirit {
    x: f32,
    y: f32,
    /// How far it moved across the last step.
    vx: f32,
    /// Where it flutters to by day.
    tx: f32,
    ty: f32,
    phase: f32,
    /// Steps since it set out; it only goes home after a while with the xolo.
    age: u32,
    /// Steps into its homecoming over the ofrenda, 0 until then.
    home: u32,
}

/// Steps a soul rises over the ofrenda before it is home.
const HOMECOMING: u32 = 90;

/// Día de Muertos in the meadow, laid out for one world: a path of
/// cempasúchil petals from the edge of the world to a small ofrenda, candles
/// along it, and the souls that come home by it.
struct Muertos {
    level: f32,
    /// The ofrenda's left edge and bottom row, in world pixels.
    ox: i32,
    oy: i32,
    /// Where the path enters the world, off its edge.
    start: f32,
    /// The path's centre row at every world x, and how far its petals
    /// reach either side (0 where there is no path).
    path: Vec<i32>,
    width: Vec<u8>,
    petals: Vec<(i32, i32, Rgb)>,
    candles: Vec<Candle>,
    /// Every candle flame, in world pixels.
    flames: Vec<(i32, i32)>,
    spirits: Vec<Spirit>,
    /// Rows of the arch still missing while the holiday comes.
    arch_from: i32,
    cloths: [Rgb; 3],
    /// The holiday's own dice, so ordinary days keep theirs.
    rng: Rng,
}

/// The path's centre row at world x.
#[inline]
fn path_at(path: &[i32], x: f32) -> f32 {
    path[(x as i32).clamp(0, path.len() as i32 - 1) as usize] as f32
}

impl Muertos {
    /// The holiday for this world, if it is on.
    fn of(ctx: &Ctx, m: &Meadow) -> Option<Muertos> {
        ctx.fiesta
            .is(Kind::Muertos)
            .then(|| Muertos::new(ctx, m.w, m.ph, m.fence, m.ground))
    }

    fn new(ctx: &Ctx, w: i32, ph: i32, fence: i32, ground: i32) -> Muertos {
        let level = ctx.fiesta.level.clamp(0.0, 1.0);
        let mut rng = Rng::with_seed(ctx.seed ^ MUERTOS_SEED);
        // The ofrenda stands by the fence in one half of the meadow, behind
        // the xolo's track, and the path comes in from the far side.
        let right = rng.bool();
        let at = if right {
            rng.rangef(0.62, 0.78)
        } else {
            rng.rangef(0.22, 0.38)
        };
        let ox = ((w as f32 * at) as i32 - OFRENDA_W / 2).clamp(0, (w - OFRENDA_W).max(0));
        let cx = (ox + OFRENDA_W / 2) as f32;
        let oy = fence + 2;
        let start = if right { -3.0 } else { w as f32 + 2.0 };
        // It winds across the meadow, then turns back to the ofrenda's foot.
        let (a, b) = (rng.rangef(0.0, 6.3), rng.rangef(0.0, 6.3));
        let foot = (oy + 2) as f32;
        let path: Vec<i32> = (0..w.max(1))
            .map(|x| {
                let xf = x as f32;
                let wave = ground as f32
                    + 3.0
                    + 4.0 * (xf * 0.045 + a).sin()
                    + 1.5 * (xf * 0.12 + b).sin();
                let t = (((xf - cx).abs() - OFRENDA_W as f32 / 2.0) / 50.0).clamp(0.0, 1.0);
                (foot + (wave - foot) * t * t * (3.0 - 2.0 * t)).round() as i32
            })
            .collect();
        // Narrower as it goes away.
        let half =
            |y: i32| 1.2 + 2.6 * ((y - fence) as f32 / (ph - fence).max(1) as f32).clamp(0.0, 1.0);
        let (from, to) = if right {
            (0, ox + OFRENDA_W - 3)
        } else {
            (ox + 3, w)
        };
        let (from, to) = (from.clamp(0, w), to.clamp(0, w));
        let mut width = vec![0u8; path.len()];
        // Petals, thicker as the holiday comes: mostly cempasúchil, some
        // terciopelo, now and then a white nube.
        let dens = 0.62 + 0.38 * level;
        let mut petals = Vec::new();
        for x in from..to {
            let yc = path[x as usize];
            let hw = half(yc);
            let r = hw.ceil() as i32;
            width[x as usize] = r as u8;
            for y in yc - r..=yc + r {
                let e = (y - yc) as f32 / hw;
                if rng.f32() < dens * (1.0 - 0.8 * e * e) {
                    let k = rng.f32();
                    let c = if k < 0.4 {
                        CEMPASUCHIL[0]
                    } else if k < 0.66 {
                        CEMPASUCHIL[1]
                    } else if k < 0.86 {
                        CEMPASUCHIL[2]
                    } else if k < 0.96 {
                        TERCIOPELO
                    } else {
                        HUESO
                    };
                    petals.push((x, y, c));
                }
            }
        }
        // Veladoras along it, more as the holiday comes, alternating sides.
        let len = to - from;
        let n = (len as f32 / 40.0 * (0.35 + 0.65 * level)).round() as i32;
        let mut candles = Vec::new();
        for i in 0..n {
            let x = from + ((i as f32 + 0.5) * len as f32 / n as f32) as i32;
            if (x as f32 - cx).abs() < (OFRENDA_W / 2 + 3) as f32 {
                continue;
            }
            let yc = path[x as usize];
            let side = if i % 2 == 0 { -1 } else { 1 };
            candles.push(Candle {
                x,
                y: yc + side * (half(yc).ceil() as i32 + 1),
                glass: if rng.chance(0.5) { HUESO } else { TERCIOPELO },
            });
        }
        let top = oy - OFRENDA.len() as i32 + 1;
        let mut flames: Vec<(i32, i32)> = candles.iter().map(|c| (c.x, c.y - 2)).collect();
        for (ry, row) in OFRENDA.iter().enumerate() {
            for (rx, ch) in row.bytes().enumerate() {
                if ch == b'f' {
                    flames.push((ox + rx as i32, top + ry as i32));
                }
            }
        }
        // More souls come home as the holiday does.
        let spirits = (0..2 + (5.0 * level).round() as usize)
            .map(|_| {
                let x = rng.rangef(0.0, w as f32);
                let y = path_at(&path, x) - rng.rangef(5.0, 20.0);
                Spirit {
                    x,
                    y,
                    vx: 0.0,
                    tx: x,
                    ty: y,
                    phase: rng.rangef(0.0, 6.3),
                    age: rng.u32(0..600),
                    home: 0,
                }
            })
            .collect();
        // The arch goes up its posts as the days go by and closes over the
        // ofrenda on November 1.
        let built = (0.45 + 0.55 * (level - 0.25) / 0.55).clamp(0.45, 1.0);
        let cloths =
            std::array::from_fn(|i| rgb_of(ctx.pick(i, col(CLOTHS[i]))).unwrap_or(CLOTHS[i]));
        Muertos {
            level,
            ox,
            oy,
            start,
            path,
            width,
            petals,
            candles,
            flames,
            spirits,
            arch_from: ((1.0 - built) * ARCH_ROWS as f32).round() as i32,
            cloths,
            rng,
        }
    }

    /// Whether a blade of grass grows where the path has trodden it down.
    #[inline]
    fn on_path(&self, x: i32, y: i32) -> bool {
        let Some(&r) = self.width.get(x as usize) else {
            return false;
        };
        r > 0 && (y - self.path[x as usize]).abs() <= r as i32
    }

    /// Whether the meadow's flower at a blade is a cempasúchil (or a
    /// terciopelo): a few at first, nearly all of them on November 2.
    #[inline]
    fn blooms(&self, x: i32, y: i32) -> bool {
        ((hash(x, y, 0xC3A5) % 100) as f32) < 30.0 + 68.0 * self.level
    }

    /// One step of the souls: by day monarchs fluttering about the path near
    /// the xolo; by night lights strung out behind it, going home when they
    /// pass the ofrenda after a good while on the way.
    fn update(&mut self, dog_cx: f32, dir: f32, daylight: f32, w: f32, ph: f32) {
        let night = daylight < 0.5;
        let home_x = (self.ox + OFRENDA_W / 2) as f32;
        let mut lead = dog_cx - dir * 12.0;
        for i in 0..self.spirits.len() {
            let s = &mut self.spirits[i];
            let x0 = s.x;
            s.age = s.age.wrapping_add(1);
            if s.home > 0 {
                s.home += 1;
                s.y -= 0.22;
                s.x += 0.15 * (s.home as f32 * 0.12 + s.phase).sin();
                if s.home > HOMECOMING {
                    // Another soul sets out from the start of the path.
                    s.x = self.start;
                    s.y = path_at(&self.path, s.x) - 8.0;
                    (s.age, s.home) = (0, 0);
                }
                continue;
            }
            if night {
                // Strung out behind the xolo, each a little behind the last.
                let ty = path_at(&self.path, lead) - 7.0;
                s.x += ((lead - s.x) * 0.15).clamp(-1.7, 1.7);
                s.y += ((ty - s.y) * 0.12).clamp(-0.8, 0.8);
                lead = s.x - dir * 6.0;
                if s.age > 600 && (s.x - home_x).abs() < 1.5 {
                    s.home = 1;
                }
            } else {
                if ((s.tx - s.x).abs() < 3.0 && (s.ty - s.y).abs() < 3.0) || self.rng.chance(0.006)
                {
                    s.tx = (dog_cx + self.rng.rangef(-70.0, 70.0)).clamp(0.0, w);
                    s.ty = path_at(&self.path, s.tx) - self.rng.rangef(5.0, 24.0);
                }
                let t = s.age as f32;
                s.x += (s.tx - s.x).clamp(-0.55, 0.55) + 0.25 * (t * 0.3 + s.phase).sin();
                s.y += (s.ty - s.y).clamp(-0.35, 0.35) + 0.45 * (t * 0.45 + s.phase).sin();
            }
            s.y = s.y.clamp(2.0, (ph - 2.0).max(2.0));
            s.vx = s.x - x0;
        }
    }
}

/// What part of the world is on screen: top-left corner in world pixels and
/// screen pixels per world pixel.
#[derive(Clone, Copy)]
struct View {
    x: f32,
    y: f32,
    z: f32,
}

impl View {
    fn to_screen(self, x: f32, y: f32) -> (i32, i32) {
        (
            ((x - self.x) * self.z).floor() as i32,
            ((y - self.y) * self.z).floor() as i32,
        )
    }
}
/// A candle flame's colour this step: it flickers between core and edge.
#[inline]
fn flame(x: i32, y: i32, tick: u32) -> Rgb {
    if hash(x, y ^ (tick / 2) as i32, 0xF1A3).is_multiple_of(3) {
        FLAMA_EDGE
    } else {
        FLAMA
    }
}

#[inline]
fn hash(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = (x as u32).wrapping_mul(374_761_393) ^ (y as u32).wrapping_mul(668_265_263) ^ seed;
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

/// Seconds per step of the world: tanim runs its animations at 20 fps.
const STEP: f64 = 1.0 / 20.0;
/// The most steps one frame may catch up on (after a suspend, say).
const MAX_STEPS: u32 = 200;
/// Real seconds of a day at which tanim's nap lengths were tuned (3600 frames at 20 fps).
const BASE_DAY: f32 = 180.0;
/// How much the theme tints the greens and the daytime sky.
const TINT: f32 = 0.3;

/// The colors a theme changes: the daytime sky and the greens. Under the
/// native theme they are tanim's own.
#[derive(Clone, Copy)]
struct Pal {
    day_top: Rgb,
    day_bot: Rgb,
    green: Rgb,
    green_k: f32,
}

impl Pal {
    fn new(ctx: &Ctx) -> Pal {
        let native = Pal {
            day_top: DAY_TOP,
            day_bot: DAY_BOT,
            green: Rgb::BLACK,
            green_k: 0.0,
        };
        if ctx.theme.native {
            return native;
        }
        let g = &ctx.theme.grad;
        let mix = |base: Rgb, c: Color| rgb_of(c).map_or(base, |c| base.lerp(c, TINT));
        match rgb_of(g.at(0.7)) {
            Some(green) => Pal {
                day_top: mix(DAY_TOP, g.at(0.55)),
                day_bot: mix(DAY_BOT, g.at(0.9)),
                green,
                green_k: TINT,
            },
            None => native,
        }
    }
}

/// Time of day keyframes: (t, sky top, sky bottom, light tint on the scene).
fn keys(p: &Pal) -> [(f32, Rgb, Rgb, Rgb); 8] {
    [
        (0.0, NIGHT_TOP, NIGHT_BOT, NIGHT_TINT),
        (0.17, NIGHT_TOP, NIGHT_BOT, NIGHT_TINT),
        (
            0.23,
            Rgb(72, 72, 142),
            Rgb(250, 162, 122),
            Rgb(232, 184, 172),
        ),
        (0.3, p.day_top, p.day_bot, Rgb::WHITE),
        (0.68, p.day_top, p.day_bot, Rgb::WHITE),
        (
            0.75,
            Rgb(82, 60, 132),
            Rgb(252, 140, 80),
            Rgb(240, 172, 142),
        ),
        (0.82, NIGHT_TOP, NIGHT_BOT, NIGHT_TINT),
        (1.0, NIGHT_TOP, NIGHT_BOT, NIGHT_TINT),
    ]
}

/// What the options choose.
#[derive(Clone, Copy)]
struct Settings {
    sky: bool,
    butterflies: bool,
    /// Index into [`ZOOMS`].
    zoom: usize,
    /// Real seconds per day over [`BASE_DAY`]: naps last as long a part of the day.
    nap_scale: f32,
}

struct Meadow {
    /// World size in pixels.
    w: i32,
    ph: i32,
    /// Screen size in half-block pixels.
    sw: i32,
    sph: i32,
    horizon: i32,
    fence: i32,
    ground: i32,
    px: Vec<Rgb>,
    /// The projected picture, kept between steps.
    scr: Pixels,
    far: Vec<i32>,
    near: Vec<i32>,
    seed: u32,
    trees: Vec<Tree>,
    clouds: Vec<Cloud>,
    stars: Vec<Star>,
    blades: Vec<Blade>,
    fireflies: Vec<Firefly>,
    zs: Vec<Zzz>,
    dog: Dog,
    butterfly: Option<Butterfly>,
    /// Time of day, `[0, 1)`.
    t: f32,
    tick: u32,
    sky_top: Rgb,
    sky_bot: Rgb,
    tint: Rgb,
    /// World pixel rows in view.
    view_h: f32,
    /// World x the camera centres on, trailing the dog.
    cam_x: f32,
    pal: Pal,
    set: Settings,
    /// Día de Muertos, when it is on; the dog is then a xoloitzcuintle.
    muertos: Option<Muertos>,
    xolo: bool,
}

impl Meadow {
    fn new(cols: u16, rows: u16, rng: &mut Rng, pal: Pal, set: Settings) -> Meadow {
        let (sw, sph) = (cols.max(1) as i32, 2 * rows.max(1) as i32);
        let ph = WORLD_H;
        let w = (WORLD_H * sw / sph).max(1);
        // The dog runs close to the horizon so that zooming in on it still
        // shows some sky; the rest of the meadow is foreground.
        let horizon = ph * 62 / 100;
        let ground = horizon + 22;
        let fence = horizon + 8;
        let (a, b, c) = (
            rng.rangef(0.0, 6.3),
            rng.rangef(0.0, 6.3),
            rng.rangef(0.0, 6.3),
        );
        let far = (0..w)
            .map(|x| {
                let x = x as f32;
                let v = 0.55 + 0.25 * (x * 0.02 + a).sin() + 0.2 * (x * 0.05 + b).sin();
                horizon - (ph as f32 * 0.09 * v) as i32
            })
            .collect();
        let near = (0..w)
            .map(|x| {
                horizon - (ph as f32 * 0.03 * (0.5 + 0.5 * (x as f32 * 0.03 + c).sin())) as i32
            })
            .collect();

        let mut trees = Vec::new();
        let count = (w / 70).clamp(1, 5);
        for i in 0..count {
            let slot = w / count;
            trees.push(Tree {
                x: i * slot + rng.range(slot / 4, slot * 3 / 4),
                trunk: ph / 14 + rng.range(0, 4),
                r: ph / 13 + rng.range(0, 5),
            });
        }

        let clouds = (0..(w / 40).max(1))
            .map(|_| Self::cloud(rng, w, horizon, true))
            .collect();
        let stars = (0..(w * horizon / 90).max(1))
            .map(|_| Star {
                x: rng.range(0, w),
                y: rng.range(0, (horizon - 2).max(1)),
                phase: rng.rangef(0.0, 6.3),
            })
            .collect();

        let mut blades: Vec<Blade> = (0..(w * (ph - fence) / 8))
            .map(|_| {
                let y = rng.range(fence + 1, ph + 2);
                // No flowers right in front of the dog, where they would cover it.
                let flower = rng.chance(0.1) && !(ground - 2..ground + 8).contains(&y);
                // Nearer blades stand taller.
                let near = 1.0 + 1.5 * (y - fence) as f32 / (ph - fence) as f32;
                let h = if flower {
                    rng.range(2, 6)
                } else {
                    rng.range(1, 4)
                };
                Blade {
                    x: rng.range(0, w),
                    y,
                    h: (h as f32 * near).round() as i32,
                    color: *rng.pick(&BLADES),
                    flower: flower.then(|| (*rng.pick(&PETALS), rng.chance(0.4))),
                }
            })
            .collect();
        blades.sort_by_key(|b| b.y);

        let fireflies = (0..(w / 14).max(1))
            .map(|_| Firefly {
                x: rng.rangef(0.0, w as f32),
                y: rng.rangef((fence - ph / 8) as f32, ground as f32),
                vx: 0.0,
                vy: 0.0,
                phase: rng.rangef(0.0, 6.3),
            })
            .collect();

        let dog = Dog {
            x: rng.rangef(0.0, (w - SPRITE_W).max(1) as f32),
            dir: if rng.chance(0.5) { 1.0 } else { -1.0 },
            state: State::Stand,
            timer: 20,
            nap: 0.0,
            target: 0.0,
            turn: false,
            hop: 0.0,
            vy: 0.0,
            anim: 0,
            coat: *rng.pick(&COATS),
            collar: *rng.pick(&COLLARS),
        };
        let cam_x = dog.x + SPRITE_W as f32 / 2.0;

        let mut m = Meadow {
            w,
            ph,
            sw,
            sph,
            horizon,
            fence,
            ground,
            px: vec![Rgb::BLACK; (w * ph) as usize],
            scr: Pixels::new(cols.max(1), rows.max(1)),
            far,
            near,
            seed: rng.u32(..),
            trees,
            clouds,
            stars,
            blades,
            fireflies,
            zs: Vec::new(),
            dog,
            butterfly: None,
            // Start somewhere in the morning or early afternoon.
            t: rng.rangef(0.3, 0.5),
            tick: 0,
            sky_top: DAY_TOP,
            sky_bot: DAY_BOT,
            tint: Rgb::WHITE,
            view_h: ZOOMS[set.zoom.min(ZOOMS.len() - 1)],
            cam_x,
            pal,
            set,
            muertos: None,
            xolo: false,
        };
        m.update_sky();
        m
    }

    /// Dresses the meadow for the holiday, or takes the dress off.
    fn dress(&mut self, ctx: &Ctx) {
        self.muertos = Muertos::of(ctx, self);
        self.xolo = self.muertos.is_some();
    }

    fn cloud(rng: &mut Rng, w: i32, horizon: i32, anywhere: bool) -> Cloud {
        let n = rng.range(3, 6);
        let r = WORLD_H as f32 / 24.0;
        let mut puffs = Vec::new();
        let mut x = 0.0;
        for i in 0..n {
            let big = i > 0 && i < n - 1;
            let pr = r * if big {
                rng.rangef(1.2, 1.8)
            } else {
                rng.rangef(0.8, 1.1)
            };
            puffs.push((x, -pr * 0.3, pr));
            x += pr * rng.rangef(0.9, 1.3);
        }
        Cloud {
            x: if anywhere {
                rng.rangef(0.0, w as f32)
            } else {
                -x - r
            },
            y: rng.rangef(r * 2.0, (horizon as f32 * 0.45).max(r * 2.0 + 1.0)),
            v: rng.rangef(0.03, 0.09),
            puffs,
        }
    }

    /// Horizontal range the dog's sprite can occupy.
    fn limits(&self) -> (f32, f32) {
        let lo = 1.0;
        let hi = (self.w - SPRITE_W - 1) as f32;
        (lo, hi.max(lo))
    }

    fn daylight(&self) -> f32 {
        ((self.tint.luma() as f32 - NIGHT_TINT.luma() as f32) / (255.0 - NIGHT_TINT.luma() as f32))
            .clamp(0.0, 1.0)
    }

    #[inline]
    fn shade(&self, c: Rgb) -> Rgb {
        let f = |a: u8, t: u8| (a as u32 * t as u32 / 255) as u8;
        Rgb(
            f(c.0, self.tint.0),
            f(c.1, self.tint.1),
            f(c.2, self.tint.2),
        )
    }

    #[inline]
    fn put(&mut self, x: i32, y: i32, c: Rgb) {
        if x >= 0 && y >= 0 && x < self.w && y < self.ph {
            self.px[(y * self.w + x) as usize] = c;
        }
    }

    #[inline]
    fn blend(&mut self, x: i32, y: i32, c: Rgb, a: f32) {
        if x >= 0 && y >= 0 && x < self.w && y < self.ph {
            let p = &mut self.px[(y * self.w + x) as usize];
            *p = p.lerp(c, a);
        }
    }

    /// Like `blend`, but on the screen buffer.
    #[inline]
    fn blend_out(&mut self, x: i32, y: i32, c: Rgb, a: f32) {
        if let Some(p) = rgb_of(self.scr.get(x, y)) {
            self.scr.set(x, y, col(p.lerp(c, a)));
        }
    }

    /// A theme's greens: tanim's own under the native theme.
    #[inline]
    fn green(&self, c: Rgb) -> Rgb {
        c.lerp(self.pal.green, self.pal.green_k)
    }

    // ----- simulation -----

    /// One step of the world, 20 of which make a second.
    fn update(&mut self, rng: &mut Rng) {
        self.tick = self.tick.wrapping_add(1);
        for i in 0..self.clouds.len() {
            let cl = &mut self.clouds[i];
            cl.x += cl.v;
            if cl.x > self.w as f32 + 2.0 {
                self.clouds[i] = Self::cloud(rng, self.w, self.horizon, false);
            }
        }
        let top = (self.fence - self.ph / 8) as f32;
        let bot = (self.ground as f32).max(top);
        for f in self.fireflies.iter_mut() {
            f.vx = (f.vx + rng.rangef(-0.04, 0.04)).clamp(-0.3, 0.3);
            f.vy = (f.vy + rng.rangef(-0.03, 0.03)).clamp(-0.15, 0.15);
            f.x = (f.x + f.vx).rem_euclid(self.w as f32);
            f.y += f.vy;
            if f.y < top || f.y > bot {
                f.vy = -f.vy;
                f.y = f.y.clamp(top, bot);
            }
        }
        self.update_butterfly(rng);
        self.update_dog(rng);
        if let Some(mut mu) = self.muertos.take() {
            let cx = self.dog.x + SPRITE_W as f32 / 2.0;
            let (w, ph) = (self.w as f32, self.ph as f32);
            mu.update(cx, self.dog.dir, self.daylight(), w, ph);
            self.muertos = Some(mu);
        }
        for z in self.zs.iter_mut() {
            z.age += 1;
            z.y -= 0.18;
            z.x += 0.12 * (z.age as f32 * 0.15).sin();
        }
        self.zs.retain(|z| z.age < 70 && z.y > 0.0);
        // Only pan once the dog leaves the middle of the frame, and never let
        // it out of frame.
        let center = self.dog.x + SPRITE_W as f32 / 2.0;
        let slack = ((self.view_size().0 - SPRITE_W as f32) / 2.0 - CAM_MARGIN).max(0.0);
        let target = self.cam_x.clamp(center - slack * 0.5, center + slack * 0.5);
        self.cam_x += (target - self.cam_x) * 0.15;
        self.cam_x = self.cam_x.clamp(center - slack, center + slack);
    }

    fn update_sky(&mut self) {
        let t = self.t;
        for win in keys(&self.pal).windows(2) {
            let (t0, a0, b0, c0) = win[0];
            let (t1, a1, b1, c1) = win[1];
            if t >= t0 && t <= t1 {
                let f = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
                self.sky_top = a0.lerp(a1, f);
                self.sky_bot = b0.lerp(b1, f);
                self.tint = c0.lerp(c1, f);
                return;
            }
        }
    }

    fn update_butterfly(&mut self, rng: &mut Rng) {
        let (w, tick) = (self.w as f32, self.tick as f32);
        let (lo_y, hi_y) = (
            (self.horizon - self.ph / 6) as f32,
            (self.ground - 6) as f32,
        );
        match &mut self.butterfly {
            Some(b) => {
                if b.flee != 0.0 {
                    b.x += b.flee * b.speed;
                    b.y += (tick * 0.4).sin() * 0.5 - 0.12;
                    b.y = b.y.max(2.0);
                } else {
                    b.x += (b.tx - b.x).clamp(-0.5, 0.5);
                    b.y += (b.ty - b.y).clamp(-0.3, 0.3) + (tick * 0.5).sin() * 0.4;
                    if (b.tx - b.x).abs() < 2.0 && (b.ty - b.y).abs() < 2.0 {
                        b.tx = rng.rangef(0.0, w);
                        b.ty = rng.rangef(lo_y, hi_y.max(lo_y + 1.0));
                    }
                    b.life = b.life.saturating_sub(1);
                    if b.life == 0 {
                        // Wander off through the nearest side.
                        b.flee = if b.x < w / 2.0 { -1.0 } else { 1.0 };
                        b.speed = 0.5;
                    }
                }
                if b.x < -6.0 || b.x > w + 6.0 {
                    self.butterfly = None;
                }
            }
            None => {
                if self.set.butterflies && self.daylight() > 0.7 && rng.chance(0.004) {
                    let left = rng.chance(0.5);
                    let hi_y = hi_y.max(lo_y + 1.0);
                    // Enter near the dog so the camera has a chance to see it.
                    let near = self.dog.x + SPRITE_W as f32 / 2.0 + if left { -60.0 } else { 60.0 };
                    self.butterfly = Some(Butterfly {
                        x: near.clamp(-3.0, w + 3.0),
                        y: rng.rangef(lo_y, hi_y),
                        tx: rng.rangef(w * 0.2, w * 0.8),
                        ty: rng.rangef(lo_y, hi_y),
                        flee: 0.0,
                        speed: 0.8,
                        life: rng.range(250, 450) as u32,
                        color: *rng.pick(&WINGS),
                    });
                }
            }
        }
    }

    fn start_run(&self, d: &mut Dog, rng: &mut Rng) {
        let (lo, hi) = self.limits();
        let edge = |dir: f32| if dir > 0.0 { hi } else { lo };
        if (edge(d.dir) - d.x).abs() < 6.0 {
            d.dir = -d.dir;
        }
        let e = edge(d.dir);
        d.target = if rng.chance(0.5) {
            e
        } else {
            d.x + (e - d.x) * rng.rangef(0.3, 0.8)
        };
        if (d.target - d.x).abs() < 8.0 {
            d.target = e;
        }
        d.state = State::Run;
    }

    fn rest(&self, d: &mut Dog, rng: &mut Rng, at_edge: bool) {
        d.turn = at_edge;
        let nap = if self.daylight() < 0.5 { 0.6 } else { 0.15 };
        let r = rng.f32();
        (d.state, d.timer) = if r < nap {
            {
                d.nap = rng.rangef(11.0, 21.0) * self.set.nap_scale;
                (State::Nap, 1)
            }
        } else if r < nap + (1.0 - nap) * 0.4 {
            (State::Sit, rng.range(90, 200) as u32)
        } else if r < nap + (1.0 - nap) * 0.8 {
            (State::Sniff, rng.range(40, 100) as u32)
        } else {
            (State::Stand, rng.range(30, 60) as u32)
        };
    }

    fn update_dog(&mut self, rng: &mut Rng) {
        let mut d = self.dog;
        let (lo, hi) = self.limits();
        d.anim = d.anim.wrapping_add(1);
        if d.hop > 0.0 || d.vy > 0.0 {
            d.hop += d.vy;
            d.vy -= 0.3;
            if d.hop <= 0.0 {
                d.hop = 0.0;
                d.vy = 0.0;
            }
        }
        match d.state {
            State::Run => {
                d.x += d.dir * 1.1;
                if (d.target - d.x) * d.dir <= 0.0 {
                    d.x = d.target;
                    let at_edge = d.x <= lo + 0.5 || d.x >= hi - 0.5;
                    if at_edge && !rng.chance(0.35) {
                        (d.state, d.timer, d.turn) = (State::Stand, rng.range(10, 25) as u32, true);
                    } else {
                        self.rest(&mut d, rng, at_edge);
                    }
                }
            }
            State::Chase => {
                d.x = (d.x + d.dir * 1.5).clamp(lo, hi);
                let head = d.x + if d.dir > 0.0 { 19.0 } else { 2.0 };
                match &mut self.butterfly {
                    Some(b) => {
                        if d.hop == 0.0 && (b.x - head).abs() < 5.0 {
                            // Leap at it; it startles and speeds away.
                            d.vy = 2.4;
                            d.hop = 0.1;
                            b.speed = 1.8;
                        }
                        if d.x <= lo || d.x >= hi {
                            (d.state, d.timer, d.turn) = (State::Stand, 40, true);
                        }
                    }
                    None => (d.state, d.timer) = (State::Stand, 30),
                }
            }
            _ => {
                if d.state == State::Nap && d.anim.is_multiple_of(30) {
                    let hx = if d.dir > 0.0 { 17.0 } else { 4.0 };
                    self.zs.push(Zzz {
                        x: d.x + hx,
                        y: (self.ground - LIE.len() as i32) as f32,
                        age: 0,
                    });
                }
                // A nap runs on the wall clock (see `DogScene::tick`).
                let waiting = if d.state == State::Nap {
                    d.nap > 0.0
                } else if d.timer > 0 {
                    d.timer -= 1;
                    true
                } else {
                    false
                };
                if !waiting {
                    match d.state {
                        // Waking up: sit for a moment first.
                        State::Nap => (d.state, d.timer) = (State::Sit, rng.range(30, 60) as u32),
                        State::Stand => {
                            if d.turn {
                                d.dir = -d.dir;
                                d.turn = false;
                            }
                            self.start_run(&mut d, rng);
                        }
                        _ => (d.state, d.timer) = (State::Stand, rng.range(10, 30) as u32),
                    }
                }
            }
        }
        // A wandering butterfly close by is impossible to resist.
        if matches!(d.state, State::Stand | State::Sit | State::Sniff)
            && self.daylight() > 0.5
            && let Some(b) = &mut self.butterfly
        {
            let center = d.x + SPRITE_W as f32 / 2.0;
            if b.flee == 0.0 && (b.x - center).abs() < 35.0 && b.x > 0.0 && b.x < self.w as f32 {
                d.dir = if b.x >= center { 1.0 } else { -1.0 };
                d.state = State::Chase;
                d.turn = false;
                b.flee = d.dir;
                b.speed = 0.85;
            }
        }
        self.dog = d;
    }

    /// Width and height of the view in world pixels. Zooming in stops short
    /// of the point where the whole dog, leaps included, would not fit.
    fn view_size(&self) -> (f32, f32) {
        let aspect = self.sw as f32 / self.sph as f32;
        let fit_w = (SPRITE_W as f32 + 2.0 * CAM_MARGIN) / aspect;
        let fit_h = DOG_H + MAX_HOP + 2.0 * CAM_MARGIN;
        let vh = self.view_h.max(fit_w).max(fit_h).min(self.ph as f32);
        (vh * aspect, vh)
    }

    fn view(&self) -> View {
        let (vw, vh) = self.view_size();
        let z = self.sph as f32 / vh;
        // Snap to whole screen pixels so the picture only shifts in visible
        // steps instead of every cell's colour creeping with sub-pixel moves.
        let snap = |v: f32| (v * z).round() / z;
        let x = snap((self.cam_x - vw / 2.0).clamp(0.0, (self.w as f32 - vw).max(0.0)));
        // The dog's ground line sits at the same height in every zoom level,
        // unless the camera has to tilt up to keep a leaping dog in frame.
        let top = self.ground as f32 - self.dog.hop - DOG_H - CAM_MARGIN;
        let y = (self.ground as f32 - 0.78 * vh)
            .min(top)
            .clamp(0.0, (self.ph as f32 - vh).max(0.0));
        View { x, y: snap(y), z }
    }

    // ----- drawing -----

    fn draw(&mut self) {
        let (w, ph) = (self.w, self.ph);
        // Sky, or the key color where it is see-through.
        for y in 0..self.horizon {
            let c = if self.set.sky {
                self.sky_top
                    .lerp(self.sky_bot, y as f32 / self.horizon as f32)
            } else {
                KEY
            };
            self.px[(y * w) as usize..((y + 1) * w) as usize].fill(c);
        }
        if self.set.sky {
            self.draw_sun_moon();
            self.draw_clouds();
        }
        // Hills and meadow.
        let far_c = self.shade(self.green(FAR_HILL));
        for x in 0..w {
            let (f, n) = (self.far[x as usize], self.near[x as usize]);
            for y in f.max(0)..n.min(ph) {
                self.put(x, y, far_c);
            }
            for y in n.max(0)..ph {
                // Flat rows (no per-pixel texture): identical neighbouring
                // cells need no colour escapes when the camera pans.
                let t = ((y - n) as f32 / (ph - n).max(1) as f32).powf(0.8);
                let c = self.shade(self.green(MEADOW_TOP.lerp(MEADOW_BOT, t)));
                self.put(x, y, c);
            }
        }
        self.draw_trees();
        self.draw_fence();
        let split = self.blades.partition_point(|b| b.y < self.ground);
        let mu = self.muertos.take();
        match &mu {
            None => {
                self.draw_blades(0, split, None);
                self.draw_dog();
                self.draw_blades(split, self.blades.len(), None);
                self.draw_butterfly();
            }
            Some(mu) => {
                // The petals lie on the ground; the ofrenda stands just
                // behind the xolo's track, the candles on either side of it.
                self.draw_path(mu);
                let behind = self.blades.partition_point(|b| b.y <= mu.oy).min(split);
                self.draw_blades(0, behind, Some(mu));
                self.draw_ofrenda(mu);
                self.draw_candles(mu, true);
                self.draw_blades(behind, split, Some(mu));
                self.draw_dog();
                self.draw_candles(mu, false);
                self.draw_blades(split, self.blades.len(), Some(mu));
                self.draw_butterfly();
                self.draw_monarchs(mu);
            }
        }
        self.muertos = mu;
    }

    fn draw_sun_moon(&mut self) {
        let (w, hz) = (self.w as f32, self.horizon as f32);
        let r = self.ph as f32 / 16.0;
        // Sun from 0.19 to 0.81, moon for the rest of the day.
        let (u, sun) = if (0.19..0.81).contains(&self.t) {
            ((self.t - 0.19) / 0.62, true)
        } else {
            ((self.t - 0.81).rem_euclid(1.0) / 0.38, false)
        };
        let arc = (PI * u).sin();
        let cx = u * (w + 4.0 * r) - 2.0 * r;
        let cy = hz + 1.5 * r - arc * (hz * 0.85);
        let color = if sun {
            Rgb(255, 150, 60).lerp(Rgb(255, 238, 160), (arc * 1.6).min(1.0))
        } else {
            Rgb(236, 232, 212)
        };
        let glow_r = r * if sun { 2.6 } else { 1.8 };
        for y in (cy - glow_r) as i32..=(cy + glow_r) as i32 {
            if y >= self.horizon {
                break;
            }
            for x in (cx - glow_r) as i32..=(cx + glow_r) as i32 {
                let (dx, dy) = (x as f32 - cx, y as f32 - cy);
                let d = (dx * dx + dy * dy).sqrt();
                if d <= r {
                    // Crescent: carve out a shifted disc.
                    let (ex, ey) = (dx - r * 0.55, dy + r * 0.2);
                    if sun || ex * ex + ey * ey > r * r * 0.8 {
                        self.put(x, y, color);
                    }
                } else if d < glow_r {
                    let a = (1.0 - (d - r) / (glow_r - r)) * if sun { 0.4 } else { 0.18 };
                    self.blend(x, y, color, a);
                }
            }
        }
    }

    fn draw_clouds(&mut self) {
        let night = 1.0 - self.daylight();
        let white = self
            .shade(Rgb(252, 252, 255))
            .lerp(self.sky_top, 0.4 * night);
        let under = white.lerp(self.sky_bot, 0.25);
        for i in 0..self.clouds.len() {
            let (x0, y0) = (self.clouds[i].x, self.clouds[i].y);
            for p in 0..self.clouds[i].puffs.len() {
                let (px, py, r) = self.clouds[i].puffs[p];
                let (cx, cy) = (x0 + px, y0 + py);
                for y in (cy - r) as i32..=(cy + r) as i32 {
                    if y >= self.horizon {
                        break;
                    }
                    for x in (cx - r) as i32..=(cx + r) as i32 {
                        let (dx, dy) = (x as f32 - cx, y as f32 - cy);
                        // Flat-bottomed puffs.
                        if dx * dx + dy * dy <= r * r && y as f32 <= y0 + r * 0.5 {
                            let c = if y as f32 > y0 + r * 0.1 {
                                under
                            } else {
                                white
                            };
                            self.put(x, y, c);
                        }
                    }
                }
            }
        }
    }

    fn draw_trees(&mut self) {
        for i in 0..self.trees.len() {
            let Tree { x, trunk, r } = self.trees[i];
            let base = self.near[x.clamp(0, self.w - 1) as usize] + 1;
            let tw = (r / 3).max(1);
            let trunk_c = self.shade(TRUNK);
            for y in base - trunk - r / 2..=base {
                for dx in 0..tw {
                    self.put(x - tw / 2 + dx, y, trunk_c);
                }
            }
            let (cx, cy) = (x as f32, (base - trunk - r / 2) as f32);
            let rf = r as f32;
            let blobs = [
                (0.0, 0.0, 1.0),
                (-0.75, 0.35, 0.75),
                (0.75, 0.35, 0.75),
                (0.0, -0.55, 0.75),
            ];
            let (dark, lit) = (
                self.shade(self.green(LEAVES)),
                self.shade(self.green(LEAVES_LIT)),
            );
            for y in (cy - 1.4 * rf) as i32..=(cy + 1.2 * rf) as i32 {
                for xx in (cx - 1.6 * rf) as i32..=(cx + 1.6 * rf) as i32 {
                    let (dx, dy) = (xx as f32 - cx, y as f32 - cy);
                    let inside = blobs.iter().any(|&(bx, by, br)| {
                        let (ex, ey) = (dx - bx * rf, dy - by * rf);
                        ex * ex + ey * ey <= (br * rf) * (br * rf)
                    });
                    if inside {
                        let speckle = hash(xx, y, self.seed ^ 0x5eed).is_multiple_of(5);
                        let c = if dx + dy < -0.3 * rf && !speckle {
                            lit
                        } else if speckle {
                            dark.scale(0.8)
                        } else {
                            dark
                        };
                        self.put(xx, y, c);
                    }
                }
            }
        }
    }

    fn draw_fence(&mut self) {
        let bottom = self.fence;
        let top = bottom - 6;
        let (rail, post) = (self.shade(WOOD), self.shade(POST));
        for x in 0..self.w {
            self.put(x, top + 1, rail);
            self.put(x, top + 3, rail);
        }
        for x in (4..self.w).step_by(9) {
            for y in top..=bottom {
                self.put(x, y, post);
            }
        }
    }

    /// Blades from `from` to `to`; during Día de Muertos the meadow's
    /// flowers turn to cempasúchil, terciopelo and nube.
    fn draw_blades(&mut self, from: usize, to: usize, mu: Option<&Muertos>) {
        let tick = self.tick as f32;
        let wind = 0.9 + 0.7 * (tick * 0.004).sin();
        let center = self.shade(Rgb(250, 210, 60));
        for i in from..to {
            let (x, y, h, color, flower) = {
                let b = &self.blades[i];
                (b.x, b.y, b.h, b.color, b.flower)
            };
            // The path of petals is trodden clear of grass.
            if mu.is_some_and(|mu| mu.on_path(x, y)) {
                continue;
            }
            let sway =
                (tick * 0.06 + x as f32 * 0.35 + y as f32 * 0.2).sin() * wind * h as f32 / 3.0;
            let color = self.shade(self.green(color));
            for j in 0..h {
                let f = if h > 1 {
                    j as f32 / (h - 1) as f32
                } else {
                    1.0
                };
                self.put(x + (sway * f).round() as i32, y - j, color);
            }
            if let Some((petal, big)) = flower {
                let (tx, ty) = (x + sway.round() as i32, y - h);
                if let Some(mu) = mu
                    && mu.blooms(x, y)
                {
                    self.draw_bloom(tx, ty, big, hash(x, y, 0xB100));
                    continue;
                }
                let petal = self.shade(petal);
                if big {
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        self.put(tx + dx, ty + dy, petal);
                    }
                    self.put(tx, ty, center);
                } else {
                    self.put(tx, ty, petal);
                }
            }
        }
    }

    fn sprite(&self) -> (Sprite, i32) {
        let d = &self.dog;
        let a = d.anim;
        let x = self.xolo;
        match d.state {
            State::Run | State::Chase => {
                let f = (a / 2 % 4) as usize;
                (
                    if x { XOLO_RUN[f] } else { RUN[f] },
                    if f == 0 && d.hop == 0.0 { -1 } else { 0 },
                )
            }
            State::Stand => {
                let f = (a / 3 % 4) as usize;
                (if x { XOLO_WAG[f] } else { WAG[f] }, 0)
            }
            State::Sit => (
                match ((a / 4).is_multiple_of(2), x) {
                    (true, false) => SIT,
                    (false, false) => SIT_PANT,
                    (true, true) => XOLO_SIT,
                    (false, true) => XOLO_SIT_PANT,
                },
                0,
            ),
            State::Sniff => (
                match ((a / 5).is_multiple_of(2), x) {
                    (true, false) => SNIFF,
                    (false, false) => SNIFF_WAG,
                    (true, true) => XOLO_SNIFF,
                    (false, true) => XOLO_SNIFF_WAG,
                },
                0,
            ),
            State::Nap => (if x { XOLO_LIE } else { LIE }, 0),
        }
    }

    fn draw_dog(&mut self) {
        let d = self.dog;
        let (sprite, bob) = self.sprite();
        let (coat, dark, cream) = d.coat;
        let x0 = d.x.round() as i32;
        // Shadow, fainter while airborne.
        let shadow = 0.3 / (1.0 + d.hop * 0.15);
        for x in x0 + 3..x0 + 19 {
            self.blend(x, self.ground + 1, Rgb::BLACK, shadow);
        }
        let bottom = self.ground - d.hop.round() as i32 + bob;
        let top = bottom - sprite.len() as i32 + 1;
        for (ry, row) in sprite.iter().enumerate() {
            for (rx, ch) in row.bytes().enumerate() {
                let c = if self.xolo {
                    match ch {
                        b'B' => XOLO_SKIN,
                        b'h' => XOLO_SHEEN,
                        b'D' => XOLO_FAR,
                        b'E' => XOLO_EAR,
                        b'e' => XOLO_EAR_IN,
                        b't' => TUFT,
                        b'k' => EYE,
                        b'c' => CEMPASUCHIL[0],
                        b'o' => CEMPASUCHIL[1],
                        b'm' => CEMPASUCHIL[2],
                        b'p' => TONGUE,
                        _ => continue,
                    }
                } else {
                    match ch {
                        b'B' => coat,
                        b'D' => dark,
                        b'w' => cream,
                        b'k' => EYE,
                        b'c' => d.collar,
                        b'p' => TONGUE,
                        _ => continue,
                    }
                };
                // The cempasúchil collar keeps some of its glow at night.
                let c = if self.xolo && matches!(ch, b'c' | b'o' | b'm') {
                    self.candlelit(c)
                } else {
                    self.shade(c)
                };
                let sx = if d.dir > 0.0 {
                    rx as i32
                } else {
                    SPRITE_W - 1 - rx as i32
                };
                self.put(x0 + sx, top + ry as i32, c);
            }
        }
    }

    fn draw_butterfly(&mut self) {
        let Some(b) = &self.butterfly else { return };
        let (x, y, color) = (b.x.round() as i32, b.y.round() as i32, b.color);
        if self.xolo {
            let open = (self.tick / 3).is_multiple_of(2);
            self.draw_monarch(x, y, open);
            return;
        }
        let wing = self.shade(color);
        let body = self.shade(Rgb(50, 36, 30));
        let wy = if (self.tick / 3).is_multiple_of(2) {
            y - 1
        } else {
            y
        };
        self.put(x - 1, wy, wing);
        self.put(x + 1, wy, wing);
        self.put(x, y, body);
    }

    // ----- Día de Muertos -----

    /// How much candlelight brightens what stands near a flame: nothing by
    /// day, most of its own colour back by night.
    fn candlelit(&self, c: Rgb) -> Rgb {
        self.shade(c).lerp(c, 0.55 * (1.0 - self.daylight()))
    }

    /// A flower of the holiday on a blade: mostly cempasúchil, a pompom of
    /// orange; some terciopelo, velvet crimson; now and then white nube.
    fn draw_bloom(&mut self, x: i32, y: i32, big: bool, h: u32) {
        let (petal, lit, heart) = match h % 10 {
            0..=6 => (CEMPASUCHIL[0], CEMPASUCHIL[1], CEMPASUCHIL[2]),
            7 | 8 => (TERCIOPELO, Rgb(214, 30, 110), Rgb(110, 0, 46)),
            _ => (HUESO, HUESO, Rgb(214, 210, 200)),
        };
        let (petal, lit, heart) = (self.shade(petal), self.shade(lit), self.shade(heart));
        if big && matches!(h % 10, 7 | 8) {
            // Terciopelo: a velvet crest rather than a round bloom.
            self.put(x, y + 1, heart);
            self.put(x - 1, y, petal);
            self.put(x, y, petal);
            self.put(x, y - 1, petal);
            self.put(x + 1, y - 1, lit);
            self.put(x, y - 2, lit);
        } else if big {
            self.put(x - 1, y, petal);
            self.put(x + 1, y, petal);
            self.put(x, y - 1, lit);
            self.put(x, y + 1, heart);
            self.put(x, y, petal);
        } else {
            self.put(x, y, petal);
        }
    }

    /// The path of petals; marigolds keep a little of their glow at night.
    fn draw_path(&mut self, mu: &Muertos) {
        let glow = 0.3 * (1.0 - self.daylight());
        for &(x, y, c) in &mu.petals {
            let c = self.shade(c).lerp(c, glow);
            self.put(x, y, c);
        }
    }

    fn draw_ofrenda(&mut self, mu: &Muertos) {
        let top = mu.oy - OFRENDA.len() as i32 + 1;
        for (ry, row) in OFRENDA.iter().enumerate() {
            let y = top + ry as i32;
            for (rx, ch) in row.bytes().enumerate() {
                let x = mu.ox + rx as i32;
                let c = match ch {
                    b'Y' | b'Z' | b'X' if (ry as i32) < mu.arch_from => continue,
                    b'Y' | b'o' => CEMPASUCHIL[0],
                    b'Z' | b'O' => CEMPASUCHIL[1],
                    b'X' => CEMPASUCHIL[2],
                    b'1' => mu.cloths[0],
                    b'2' => mu.cloths[1],
                    b'3' => mu.cloths[2],
                    b'W' | b'v' | b's' => HUESO,
                    b'P' => Rgb(150, 96, 40),
                    b'q' => Rgb(206, 178, 136),
                    b'u' => Rgb(96, 70, 52),
                    b'k' => Rgb(40, 28, 40),
                    b'R' => CLOTHS[0],
                    b't' => CLOTHS[2],
                    b'y' => Rgb(255, 210, 0),
                    b'b' => Rgb(196, 120, 50),
                    b'n' => Rgb(240, 204, 140),
                    b'a' => Rgb(150, 200, 230),
                    b'l' => Rgb(140, 200, 40),
                    b'g' => Rgb(176, 92, 52),
                    b'f' => {
                        self.put(x, y, flame(x, y, self.tick));
                        continue;
                    }
                    _ => continue,
                };
                let c = self.candlelit(c);
                self.put(x, y, c);
            }
        }
    }

    /// The veladoras along the path, those behind the xolo's track or those
    /// in front of it.
    fn draw_candles(&mut self, mu: &Muertos, behind: bool) {
        for c in &mu.candles {
            if (c.y <= self.ground) != behind {
                continue;
            }
            let glass = self.candlelit(c.glass);
            self.put(c.x, c.y, glass);
            self.put(c.x, c.y - 1, glass);
            self.put(c.x, c.y - 2, flame(c.x, c.y, self.tick));
        }
    }

    /// A monarch, wings spread or raised, its body at `(x, y)`: orange
    /// forewings with dark tips, darker hindwings.
    fn draw_monarch(&mut self, x: i32, y: i32, open: bool) {
        let wing = self.shade(MONARCH);
        let hind = self.shade(CEMPASUCHIL[2]);
        let vein = self.shade(MONARCH_VEIN);
        if open {
            self.put(x - 2, y - 1, wing);
            self.put(x - 1, y - 1, wing);
            self.put(x + 1, y - 1, wing);
            self.put(x + 2, y - 1, wing);
            self.put(x - 3, y - 2, vein);
            self.put(x + 3, y - 2, vein);
            self.put(x - 1, y, hind);
            self.put(x + 1, y, hind);
        } else {
            self.put(x - 1, y - 2, wing);
            self.put(x + 1, y - 2, wing);
            self.put(x - 1, y - 1, hind);
            self.put(x + 1, y - 1, hind);
            self.put(x - 1, y - 3, vein);
            self.put(x + 1, y - 3, vein);
        }
        self.put(x, y - 1, vein);
        self.put(x, y, vein);
    }

    /// By day the souls are monarchs.
    fn draw_monarchs(&mut self, mu: &Muertos) {
        if self.daylight() < 0.45 {
            return;
        }
        for (i, s) in mu.spirits.iter().enumerate() {
            let open = (self.tick / 3 + i as u32).is_multiple_of(2);
            self.draw_monarch(s.x.round() as i32, s.y.round() as i32, open);
        }
    }

    /// Candlelight, straight onto the screen like the other lights: the
    /// ofrenda's warm glow, the flames that nothing stands in front of, and
    /// by night the souls.
    fn draw_candlelight(&mut self, mu: &Muertos, v: View, night: f32) {
        let tick = self.tick as f32;
        let k = ((night - 0.2) / 0.5).clamp(0.0, 1.0);
        let flicker = 0.85 + 0.15 * (tick * 0.7).sin() * (tick * 0.23).cos();
        if k > 0.0 {
            let (cx, cy) = v.to_screen((mu.ox + OFRENDA_W / 2) as f32, (mu.oy - 9) as f32);
            self.glow(cx, cy, (26.0 * v.z).max(2.0), FLAMA_EDGE, 0.3 * k * flicker);
        }
        for (i, &(fx, fy)) in mu.flames.iter().enumerate() {
            if fx < 0 || fy < 0 || fx >= self.w || fy >= self.ph {
                continue;
            }
            let p = self.px[(fy * self.w + fx) as usize];
            if p != FLAMA && p != FLAMA_EDGE {
                continue;
            }
            let (x, y) = v.to_screen(fx as f32 + 0.5, fy as f32 + 0.5);
            if k > 0.0 {
                let f = 0.8 + 0.2 * (tick * 0.9 + i as f32 * 1.7).sin();
                self.glow(x, y, 2.5 + 2.0 * v.z, FLAMA_EDGE, 0.5 * k * f);
            }
            self.blend_out(x, y, p, 1.0);
        }
        let souls = ((night - 0.4) / 0.3).clamp(0.0, 1.0);
        if souls > 0.0 {
            for s in &mu.spirits {
                let a = souls * (1.0 - s.home as f32 / HOMECOMING as f32);
                let bob = 1.2 * (tick * 0.09 + s.phase).sin();
                let (x, y) = v.to_screen(s.x, s.y + bob);
                self.glow(x, y, 2.5 + 1.6 * v.z, SOUL_GLOW, 0.75 * a);
                // A wisp trailing behind it as it goes, or below it, like a
                // flame, while it waits.
                let (bx, by) = if s.vx > 0.05 {
                    (-1, 0)
                } else if s.vx < -0.05 {
                    (1, 0)
                } else {
                    (0, 1)
                };
                for j in 1..=3 {
                    self.blend_out(x + bx * j, y + by * j, SOUL, a * 0.55 / j as f32);
                }
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    self.blend_out(x + dx, y + dy, SOUL, 0.6 * a);
                }
                self.blend_out(x, y, SOUL, a);
            }
        }
    }

    /// A soft round light on the screen, `a` at its centre and fading out at radius `r`.
    fn glow(&mut self, x: i32, y: i32, r: f32, c: Rgb, a: f32) {
        let ri = r.ceil() as i32;
        for dy in -ri..=ri {
            for dx in -ri..=ri {
                let d = ((dx * dx + dy * dy) as f32).sqrt() / r;
                if d < 1.0 {
                    self.blend_out(x + dx, y + dy, c, a * (1.0 - d) * (1.0 - d));
                }
            }
        }
    }

    /// Resamples the visible part of the world onto the screen: box-filtered
    /// when zoomed out, nearest pixel when zoomed in. Sky pixels stay empty
    /// when the sky is see-through.
    fn project(&mut self, v: View) {
        let inv = 1.0 / v.z;
        let (w, ph) = (self.w, self.ph);
        for sy in 0..self.sph {
            let y0 = v.y + sy as f32 * inv;
            let y1 = y0 + inv;
            for sx in 0..self.sw {
                let x0 = v.x + sx as f32 * inv;
                let x1 = x0 + inv;
                let c = if v.z >= 1.0 {
                    let x = (((x0 + x1) * 0.5) as i32).clamp(0, w - 1);
                    let y = (((y0 + y1) * 0.5) as i32).clamp(0, ph - 1);
                    let p = self.px[(y * w + x) as usize];
                    (p != KEY).then_some(p)
                } else {
                    let (mut r, mut g, mut b, mut sum, mut key) = (0.0, 0.0, 0.0, 0.0, 0.0);
                    for y in (y0 as i32).max(0)..(y1.ceil() as i32).min(ph) {
                        let wy = (y1.min(y as f32 + 1.0) - y0.max(y as f32)).max(0.0);
                        for x in (x0 as i32).max(0)..(x1.ceil() as i32).min(w) {
                            let wt = wy * (x1.min(x as f32 + 1.0) - x0.max(x as f32)).max(0.0);
                            let p = self.px[(y * w + x) as usize];
                            sum += wt;
                            if p == KEY {
                                key += wt;
                            } else {
                                r += p.0 as f32 * wt;
                                g += p.1 as f32 * wt;
                                b += p.2 as f32 * wt;
                            }
                        }
                    }
                    let lit = sum - key;
                    if lit <= 0.0 || key > lit {
                        None
                    } else {
                        let k = 1.0 / lit;
                        Some(Rgb(
                            (r * k + 0.5) as u8,
                            (g * k + 0.5) as u8,
                            (b * k + 0.5) as u8,
                        ))
                    }
                };
                self.scr.set(sx, sy, c.map_or(Color::Reset, col));
            }
        }
    }

    /// Point lights go straight onto the screen so zooming out cannot blur
    /// them away.
    fn draw_lights(&mut self, v: View) {
        let tick = self.tick as f32;
        let night = 1.0 - self.daylight();
        // Stars only come out once the sky is properly dark.
        let dark = ((night - 0.5) * 2.0).clamp(0.0, 1.0);
        if dark > 0.0 && self.set.sky {
            for i in 0..self.stars.len() {
                let st = &self.stars[i];
                let a = dark * (0.55 + 0.45 * (tick * 0.07 + st.phase).sin());
                let (wx, wy) = (st.x, st.y);
                // Skip stars hidden behind clouds or the sun and moon's glow.
                let behind = self.px[(wy * self.w + wx) as usize].luma()
                    > self.sky_top.lerp(self.sky_bot, 0.9).luma();
                if !behind {
                    let (x, y) = v.to_screen(wx as f32 + 0.5, wy as f32 + 0.5);
                    self.blend_out(x, y, Rgb(230, 230, 255), a);
                }
            }
        }
        if night > 0.05 {
            let glow = Rgb(210, 255, 110);
            for i in 0..self.fireflies.len() {
                let f = &self.fireflies[i];
                let b = night * (tick * 0.05 + f.phase).sin().max(0.0).powi(3);
                let (x, y) = v.to_screen(f.x, f.y);
                self.blend_out(x, y, glow, b);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    self.blend_out(x + dx, y + dy, glow, b * 0.25);
                }
            }
        }
        if let Some(mu) = self.muertos.take() {
            self.draw_candlelight(&mu, v, night);
            self.muertos = Some(mu);
        }
    }

    /// The sleeping dog's z's, as letters over the picture.
    fn draw_zs(&self, c: &mut Canvas, v: View) {
        let zcol = Rgb(235, 240, 255);
        for z in &self.zs {
            let (x, y) = v.to_screen(z.x, z.y);
            let below = self.scr.get(x, y);
            let fade = if z.age > 50 {
                (70 - z.age) as f32 / 20.0
            } else {
                1.0
            };
            let ch = if z.age < 25 { 'z' } else { 'Z' };
            let fg = rgb_of(below).map_or(zcol, |b| b.lerp(zcol, fade));
            if below != Color::Reset {
                c.set_bg(x, y >> 1, below);
            }
            c.set(x, y >> 1, ch, col(fg));
        }
    }

    /// Paints the world, then projects it through the camera.
    fn render(&mut self) {
        self.draw();
        let v = self.view();
        self.project(v);
        self.draw_lights(v);
    }
}

/// The scene: the meadow, stepped at tanim's pace.
struct DogScene {
    m: Meadow,
    rng: Rng,
    set: Settings,
    speed: f64,
    /// Real seconds for a day.
    day: f32,
    /// Scene seconds not yet spent on steps.
    acc: f64,
    /// Whether the picture needs painting again.
    dirty: bool,
    /// The time of day last painted.
    drawn_t: f32,
}

impl DogScene {
    fn new(ctx: &Ctx) -> DogScene {
        let o = ctx.opts;
        let day = o.i("day").clamp(10, 86_400) as f32;
        let set = Settings {
            sky: o.b("sky"),
            butterflies: o.b("butterflies"),
            zoom: o.i("zoom").clamp(0, ZOOMS.len() as i64 - 1) as usize,
            nap_scale: day / BASE_DAY,
        };
        let mut rng = ctx.rng();
        let mut m = Meadow::new(80, 24, &mut rng, Pal::new(ctx), set);
        m.dress(ctx);
        DogScene {
            m,
            rng,
            set,
            speed: o.f64("speed").clamp(0.0, 50.0),
            day,
            acc: 0.0,
            dirty: true,
            drawn_t: 0.0,
        }
    }
}

impl Scene for DogScene {
    fn resize(&mut self, ctx: &Ctx, w: u16, h: u16) {
        // The world is built for the screen's shape, so a resize starts a
        // fresh meadow; the time of day carries over.
        let t = self.m.t;
        self.m = Meadow::new(w, h, &mut self.rng, self.m.pal, self.set);
        self.m.dress(ctx);
        self.m.t = t;
        self.m.update_sky();
        self.dirty = true;
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.m.pal = Pal::new(ctx);
        // The holiday may have come or gone (or the theme changed its cloths).
        self.m.dress(ctx);
        self.m.update_sky();
        self.dirty = true;
    }

    fn update(&mut self, ctx: &Ctx, dt: f32) {
        self.tick(ctx, dt, dt);
    }

    fn tick(&mut self, _ctx: &Ctx, dt: f32, real: f32) {
        // The clocks first: they run on the wall clock, speed or not.
        if real.is_finite() && real > 0.0 {
            self.m.t = (self.m.t + real / self.day).rem_euclid(1.0);
            self.m.update_sky();
            if self.m.dog.state == State::Nap {
                self.m.dog.nap -= real;
            }
            if (self.m.t - self.drawn_t).abs() >= 0.0004 {
                self.dirty = true;
            }
        }
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.acc += dt as f64 * self.speed;
        let n = ((self.acc / STEP) as u32).min(MAX_STEPS);
        // Beyond the cap the time is dropped rather than caught up on.
        self.acc = if n == MAX_STEPS {
            0.0
        } else {
            self.acc - n as f64 * STEP
        };
        for _ in 0..n {
            self.m.update(&mut self.rng);
        }
        self.dirty |= n > 0;
    }

    fn next_change(&self) -> f32 {
        if self.speed <= 0.0 {
            return 1.0;
        }
        ((STEP - self.acc).max(0.0) / self.speed) as f32
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        if self.dirty {
            self.m.render();
            self.drawn_t = self.m.t;
            self.dirty = false;
        }
        self.m.scr.blit(c);
        let v = self.m.view();
        self.m.draw_zs(c, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::scene::Opts;
    use crate::theme::{TermColors, Theme};

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: Color::Reset,
            seed: 3,
            fiesta: crate::fiesta::Fiesta::NONE,
        }
    }

    /// Fully zoomed in, the whole dog stays on screen through runs, leaps and naps.
    #[test]
    fn camera_keeps_the_dog_in_frame() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        for (sw, sh) in [(80u16, 24u16), (200, 50), (40, 30), (24, 40), (120, 12)] {
            let mut rng = Rng::with_seed(sw as u64 * 7 + sh as u64);
            let set = Settings {
                sky: true,
                butterflies: true,
                zoom: ZOOMS.len() - 1,
                nap_scale: 1.0,
            };
            let mut m = Meadow::new(sw, sh, &mut rng, Pal::new(&ctx(&theme, &opts)), set);
            let mut max_hop: f32 = 0.0;
            for _ in 0..20_000 {
                // Naps are timed in real seconds, so run that clock by hand.
                m.dog.nap -= STEP as f32;
                m.update(&mut rng);
                let v = m.view();
                let (vw, _) = m.view_size();
                let d = m.dog;
                let (sprite, bob) = m.sprite();
                let top =
                    (m.ground - d.hop.round() as i32 + bob) as f32 - sprite.len() as f32 + 1.0;
                max_hop = max_hop.max(d.hop);
                assert!(
                    d.x >= v.x && d.x + SPRITE_W as f32 <= v.x + vw,
                    "{sw}x{sh}: dog cut horizontally"
                );
                assert!(top >= v.y, "{sw}x{sh}: dog cut at the top");
            }
            assert!(max_hop <= MAX_HOP, "leap of {max_hop} exceeds MAX_HOP");
        }
    }

    /// The day follows the wall clock: a tenth of the speed moves the dog
    /// slower but not the sun.
    #[test]
    fn the_day_keeps_real_time() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, "day = 100".parse().unwrap());
        let c = ctx(&theme, &opts);
        let (mut fast, mut slow) = (DogScene::new(&c), DogScene::new(&c));
        fast.resize(&c, 80, 24);
        slow.resize(&c, 80, 24);
        let t0 = (fast.m.t, slow.m.t);
        for _ in 0..300 {
            fast.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            slow.tick(&c, 0.1 / 30.0, 1.0 / 30.0);
        }
        // Ten real seconds of a hundred-second day.
        for (a, b) in [(fast.m.t, t0.0), (slow.m.t, t0.1)] {
            assert!(((a - b).rem_euclid(1.0) - 0.1).abs() < 1e-3, "{a} {b}");
        }
        assert!(
            fast.m.tick > 190 && slow.m.tick < 25,
            "{} {}",
            fast.m.tick,
            slow.m.tick
        );
    }

    /// A nap lasts real seconds, however slowly the dog's world moves.
    #[test]
    fn naps_follow_the_wall_clock() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let c = ctx(&theme, &opts);
        let mut s = DogScene::new(&c);
        s.m.dog.state = State::Nap;
        s.m.dog.nap = 5.0;
        for _ in 0..100 {
            s.tick(&c, 0.001, 0.05);
        }
        assert!(s.m.dog.nap < 0.01, "{}", s.m.dog.nap);
    }

    /// With the sky off nothing is painted above the hills.
    #[test]
    fn a_see_through_sky_stays_empty() {
        let theme = Theme::classic();
        for (sky, painted) in [(false, false), (true, true)] {
            let opts = Opts::new(INFO.options, format!("sky = {sky}").parse().unwrap());
            let c = ctx(&theme, &opts);
            let mut s = DogScene::new(&c);
            s.resize(&c, 80, 24);
            let mut cv = Canvas::new(80, 24, Color::Reset);
            s.draw(&c, &mut cv);
            let top = (0..80)
                .filter(|&x| cv.get(x, 0).is_some_and(|c| c.ch != ' '))
                .count();
            assert_eq!(top > 0, painted, "sky = {sky}");
            // The meadow itself is always there.
            assert_ne!(cv.get(40, 23).map(|c| c.ch), Some(' '));
        }
    }

    /// Odd option values, sizes and steps must not panic (the registry test covers the defaults).
    #[test]
    fn extreme_options_survive() {
        let cases = [
            "speed = 50.0\nday = 1\nzoom = 99\nsky = false",
            "speed = 0.0\nday = 999999999\nzoom = -4\nbutterflies = false",
        ];
        let themes = [
            Theme::classic(),
            Theme::builtin("mono").unwrap(),
            Theme::builtin("dracula").unwrap(),
            Theme::terminal(&TermColors::default()),
        ];
        for case in cases {
            let opts = Opts::new(INFO.options, case.parse().unwrap());
            for theme in &themes {
                let c = ctx(theme, &opts);
                let mut s = (INFO.make)(&c);
                for &(w, h) in &[(1u16, 1u16), (2, 3), (3, 1), (1, 40), (500, 150), (80, 24)] {
                    s.resize(&c, w, h);
                    s.retheme(&c);
                    let mut cv = Canvas::new(w, h, Color::Reset);
                    for dt in [0.0, f32::NAN, -1.0, f32::INFINITY, 0.05, 5.0, 1e9] {
                        s.tick(&c, dt, 0.05);
                        s.next_change();
                        cv.clear();
                        s.draw(&c, &mut cv);
                    }
                }
            }
        }
    }

    /// A few short days: the dog visits its moods and stays inside the world.
    #[test]
    fn a_long_run_stays_in_bounds() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, "day = 20".parse().unwrap());
        let c = ctx(&theme, &opts);
        let mut s = DogScene::new(&c);
        s.resize(&c, 100, 30);
        let mut cv = Canvas::new(100, 30, Color::Reset);
        let mut seen = [false; 6];
        for _ in 0..30 * 60 * 3 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let d = s.m.dog;
            let (lo, hi) = s.m.limits();
            assert!(d.x >= lo - 1.0 && d.x <= hi + 1.0, "dog at {}", d.x);
            seen[d.state as usize] = true;
            cv.clear();
            s.draw(&c, &mut cv);
        }
        assert!(seen.iter().filter(|&&b| b).count() >= 4, "{seen:?}");
    }

    fn holiday<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            fiesta,
            ..ctx(theme, opts)
        }
    }

    /// Whether the painted world holds a pixel of this color.
    fn painted(m: &Meadow, c: Rgb) -> bool {
        m.px.contains(&c)
    }

    /// From the first petals to Fieles Difuntos the meadow is dressed: a
    /// xolo, the path, the ofrenda, its candles and the souls; more of
    /// everything on November 2. On an ordinary day, none of it.
    #[test]
    fn muertos_dresses_the_meadow() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let mut counts = Vec::new();
        for fiesta in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
            let c = holiday(&theme, &opts, fiesta);
            let mut s = DogScene::new(&c);
            s.resize(&c, 120, 36);
            s.retheme(&c);
            let mut cv = Canvas::new(120, 36, Color::Reset);
            s.tick(&c, 0.05, 0.05);
            s.draw(&c, &mut cv);
            assert!(s.m.xolo);
            let mu = s.m.muertos.as_ref().expect("dressed");
            assert!(!mu.petals.is_empty() && !mu.flames.is_empty());
            for cloth in CLOTHS {
                assert!(painted(&s.m, cloth), "{fiesta:?}: ofrenda cloth missing");
            }
            assert!(painted(&s.m, s.m.candlelit(CEMPASUCHIL[0])) || painted(&s.m, CEMPASUCHIL[0]));
            assert!(painted(&s.m, s.m.shade(XOLO_SKIN)), "{fiesta:?}: no xolo");
            counts.push((
                mu.petals.len(),
                mu.candles.len(),
                mu.spirits.len(),
                mu.arch_from,
            ));
        }
        let (early, full) = (counts[0], counts[1]);
        assert!(
            full.0 > early.0 && full.1 >= early.1 && full.2 > early.2,
            "{counts:?}"
        );
        assert!(
            full.3 == 0 && early.3 > 0,
            "the arch closes on Nov 2: {counts:?}"
        );

        let c = ctx(&theme, &opts);
        let mut s = DogScene::new(&c);
        s.resize(&c, 120, 36);
        let mut cv = Canvas::new(120, 36, Color::Reset);
        s.draw(&c, &mut cv);
        assert!(s.m.muertos.is_none() && !s.m.xolo);
        for cloth in CLOTHS {
            assert!(!painted(&s.m, cloth));
        }
        assert!(!painted(&s.m, s.m.shade(XOLO_SKIN)));
    }

    /// The holiday comes and goes through `retheme`, and once it has gone the
    /// meadow is exactly what it would have been without it: the holiday
    /// rolls its own dice.
    #[test]
    fn the_holiday_comes_and_goes() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, "day = 30".parse().unwrap());
        let plain = ctx(&theme, &opts);
        let fiesta = holiday(&theme, &opts, Fiesta::of_day(11, 1));
        let (mut a, mut b) = (DogScene::new(&plain), DogScene::new(&plain));
        a.resize(&plain, 100, 30);
        b.resize(&plain, 100, 30);
        let (mut ca, mut cb) = (
            Canvas::new(100, 30, Color::Reset),
            Canvas::new(100, 30, Color::Reset),
        );
        for i in 0..900 {
            if i == 100 {
                b.retheme(&fiesta);
                assert!(b.m.muertos.is_some() && b.m.xolo);
            }
            if i == 600 {
                b.retheme(&plain);
                assert!(b.m.muertos.is_none() && !b.m.xolo);
            }
            let c = if (100..600).contains(&i) {
                &fiesta
            } else {
                &plain
            };
            a.tick(&plain, 1.0 / 30.0, 1.0 / 30.0);
            b.tick(c, 1.0 / 30.0, 1.0 / 30.0);
            ca.clear();
            cb.clear();
            a.draw(&plain, &mut ca);
            b.draw(c, &mut cb);
            let same = ca.cells() == cb.cells();
            assert_eq!(same, !(100..600).contains(&i), "step {i}");
        }
    }

    /// By night the souls string out behind the xolo, and now and then one
    /// goes home over the ofrenda and another sets out along the path.
    #[test]
    fn souls_follow_the_xolo_home() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let c = holiday(&theme, &opts, Fiesta::of_day(11, 2));
        let mut rng = Rng::with_seed(9);
        let set = Settings {
            sky: true,
            butterflies: true,
            zoom: 2,
            nap_scale: 1.0,
        };
        let mut m = Meadow::new(120, 36, &mut rng, Pal::new(&c), set);
        m.dress(&c);
        m.t = 0.0;
        m.update_sky();
        assert!(m.daylight() < 0.1);
        let mut homecomings = 0;
        for _ in 0..20_000 {
            m.dog.nap -= STEP as f32;
            m.update(&mut rng);
            let mu = m.muertos.as_ref().unwrap();
            homecomings += mu.spirits.iter().filter(|s| s.home == 1).count();
            for s in &mu.spirits {
                assert!(s.x.is_finite() && s.y >= 0.0 && s.y <= m.ph as f32);
            }
        }
        assert!(homecomings > 0);
        // After a while they keep close to the xolo.
        let mu = m.muertos.as_ref().unwrap();
        let cx = m.dog.x + SPRITE_W as f32 / 2.0;
        let near = mu
            .spirits
            .iter()
            .filter(|s| s.home == 0 && (s.x - cx).abs() < 80.0)
            .count();
        assert!(near > 0, "no soul near the xolo");
    }

    /// With `sky = false` the holiday's glows and souls stay out of the sky
    /// too: nothing is painted above the hills, night or day.
    #[test]
    fn the_holiday_keeps_a_see_through_sky_empty() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, "sky = false".parse().unwrap());
        let c = holiday(&theme, &opts, Fiesta::of_day(11, 2));
        for t in [0.0, 0.5] {
            let mut s = DogScene::new(&c);
            s.resize(&c, 120, 36);
            s.m.t = t;
            s.m.update_sky();
            let mut cv = Canvas::new(120, 36, Color::Reset);
            for _ in 0..60 {
                s.tick(&c, 0.05, 0.05);
                cv.clear();
                s.draw(&c, &mut cv);
                for x in 0..120 {
                    assert_eq!(cv.get(x, 0).map(|c| c.ch), Some(' '), "t = {t}, x = {x}");
                }
            }
        }
    }

    /// Odd sizes, steps and options survive with the holiday on, under the
    /// native theme and another.
    #[test]
    fn the_holiday_survives_extremes() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        for case in ["", "speed = 50.0\nday = 1\nzoom = 99\nsky = false"] {
            let opts = Opts::new(INFO.options, case.parse().unwrap());
            for theme in &themes {
                for fiesta in [
                    Fiesta::of_day(10, 28),
                    Fiesta::of_day(11, 2),
                    Fiesta::of_day(11, 3),
                ] {
                    let c = holiday(theme, &opts, fiesta);
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (2, 3), (3, 1), (1, 40), (300, 90), (80, 24)] {
                        s.resize(&c, w, h);
                        s.retheme(&c);
                        let mut cv = Canvas::new(w, h, Color::Reset);
                        for dt in [0.0, f32::NAN, 0.05, 5.0, 1e9] {
                            s.tick(&c, dt, 5.0);
                            cv.clear();
                            s.draw(&c, &mut cv);
                        }
                    }
                }
            }
        }
    }
}
