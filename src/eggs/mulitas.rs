//! On the Jueves de Corpus children in manta and rebozo walk to the church
//! with huacales of fruit and flowers on their backs, leading mulitas de
//! hoja de maíz, past a stand that sells them.
//!
//! On the Thursday of Corpus Christi, in Oaxaca, Puebla and many other
//! towns, children dress as the little farmers of old, the boys in manta
//! (white cotton) and straw hats, the girls in rebozo and braids with ribbons,
//! and carry to the church the first fruit and flowers of the year in huacales
//! (wooden crates) on their backs, held by a strap across the forehead. The
//! mulitas (little mules) of hoja de maíz (corn husk) that every child leads
//! on a cord are bought at the church door. This draws it in original oxiper
//! pixel art that grows with the screen. A puesto (stand) rises from the
//! ground, with an awning in the colors of paper flowers and a table under a
//! cloth of rebozo blue; mulitas of every size are on it (the size of a hand
//! on the table, tiny ones hanging by strings from the awning and swaying, a
//! big one at its side) and the vendor stands behind it, one arm up, calling
//! out "¡Mulitas, mulitas de hoja!". In front of it come the children, a boy
//! and a girl in turn, each with a huacal of fruit (mango, tuna, limón,
//! naranja), of paper flowers or of both on the back, the load rocking as
//! they walk, and a mulita of hoja behind on a cord, a blanket on its back, a
//! load in its panniers, its husk legs keeping time. As many of them cross as
//! the screen has room for, in either direction, within 30 s, and the puesto
//! sinks back into the ground.

use super::blit::{Xf, blit, fit, line, whole};
use super::corpus::{Palette, fade};
use super::pen::fill;
use super::{Egg, Info};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "mulitas",
    start,
    fiesta: Kind::Corpus,
    when: super::any_day,
};

/// When things happen, in seconds: the first child comes on, the puesto is
/// risen, the last child has gone, the puesto sinks and it is over.
const FIRST: f32 = 0.8;
const RISEN: f32 = 2.4;
const WALK: f32 = 22.5;
const SINKING: f32 = FIRST + WALK + 0.4;
const END: f32 = SINKING + 2.8;
/// The vendor calls out between these seconds, a call and a rest in turn.
const CALLS: (f32, f32) = (3.0, 22.5);
const CALL: f32 = 3.4;
const REST: f32 = 1.6;

/// How tall the tallest thing is, in art pixels (the puesto's awning), and
/// how much room each child with a mulita takes.
const HEIGHT: i32 = 41;
const UNIT: f32 = 68.0;
/// How far a child goes in one step, in art pixels.
const STRIDE: f32 = 15.0;
/// The puesto: its width, and how high above the ground it stands (it is
/// behind the children), in art pixels.
const STAND: f32 = 50.0;
const DEPTH: f32 = 5.0;

/// The mulita: its husk body, facing right, without legs and load.
const MULITA: &[&str] = &[
    ".................hh..hh......",
    ".................hHh.hHh.....",
    ".................HIh.HIh.....",
    ".................HIhhHIh.....",
    ".................hHHHHHHh....",
    "................hHHHnHHHHhh..",
    "................HHHHHHHHHHHh.",
    "...............hHHHHHHHHIIIIn",
    ".....hhhhhhhhhhhHHHHHHHHIIIIn",
    "..hh.hBBYYBBYYBhHHHHHHHhhhhn.",
    ".hHhhhhBBYYBBYYhhHHHHHh......",
    ".hHhHhHHHHHHHHHHhHHHHh.......",
    ".hhhhhhHhHhHhHhHhHhHh........",
    "..hh.hHhHhHhHhHhHhHhh........",
    ".....hhHhHhHhHhHhHhHhh.......",
    ".....hhhhhhhhhhhhhhhh........",
];
/// A mulita the size of a hand, with its legs, and the load on its back
/// (`x`, a letter that the mulita's maker picks).
const MINI: &[&str] = &[
    "........hh.hh.",
    "........hHhhH.",
    "........HHHHh.",
    ".xxx....hHnHHI",
    ".xxxhhhhHHHHHI",
    "hHHHHHHHHHHH..",
    "hHhHhHhHhHh...",
    ".HH.Hh..HH.Hh.",
    ".hh.hh..hh.hh.",
];
/// What the mulita carries in its panniers, over its blanket.
const LOADS: [&[&str]; 3] = [
    &["...OO.LL..", ".MMOOTLLM.", ".MMOOTTLMM", "GGGGGGGGGG"],
    &["..P..Y..V.", ".PPP.YYVVV", "GPYPGYVYGP", "GGGGGGGGGG"],
    &[".M.P..OO..", "MMPPYYOOTT", "LMPYPYGOTL", "GGGGGGGGGG"],
];

/// A boy in manta and a sombrero de palma, seen from the side, from his hat to
/// his sash (the legs and arms are drawn as he walks).
const BOY: &[&str] = &[
    ".....EEEEE.....",
    "....EEEEEEE....",
    "....RRRRRRR....",
    ".hEEEEEEEEEEEh.",
    "..hhhhhhhhhhhh.",
    "....NNKKKKK....",
    "....NKKKnKKK...",
    "....NKKKKkKK...",
    ".....KKKKKKK...",
    "......KKKKk....",
    ".....RRRRRR....",
    "....AAAAAAAA...",
    "...AAAAAAAAAa..",
    "...AAAAAAAAAa..",
    "...AAAAAAAAAa..",
    "...AAAAAAAAAa..",
    "...bbbbbbbbbb..",
];
/// A girl in a rebozo, with braids tied with ribbons, and her skirt (`Z`, with
/// its band `z`, are colors that her look picks).
const GIRL: &[&str] = &[
    ".....NNNNNN....",
    "....NNNNNNNN...",
    "...NNNNNNKKK...",
    "...NNNNKKKKKK..",
    "...NNNKKKnKKK..",
    "...NNNKKKKKKK..",
    "..PNNNKKKKKk...",
    "..NNN.KKKKk....",
    "..PN.BBBBBBB...",
    "..NBBBBCBBBBB..",
    "..PBBBBBBBCBBB.",
    "...BBCBBBBBBBB.",
    "...BBBBBBBBBBB.",
    "...BBBBBCBBBBB.",
    "...PBBBBBBBBBB.",
    "....bBbBbBbBbB.",
    "..ZZZZZZZZZZZZ.",
    ".ZZZZYZZZZZZYZZ",
    ".ZZZZZZZZZYZZZZ",
    "ZZYZZZZZZZZZZZZ",
    "zzzzzzzzzzzzzzz",
];
/// The loads of a huacal (fruit, flowers, both) and the crate under them.
const HUACAL_LOADS: [&[&str]; 3] = [
    &[
        "...OO..MM..",
        "..OOOLMMMT.",
        ".MMOOLLMTTT",
        ".MMMTTLLTTL",
        "GGGGGGGGGGG",
    ],
    &[
        "....Y...P..",
        "..P.YYY.PP.",
        ".PPGYPYGPPV",
        "GGGPPGGYGVG",
        "GGGGGGGGGGG",
    ],
    &[
        "...P..OO...",
        "..PPY.OOTL.",
        ".MPYPLOOTTL",
        "GMMGPGLLGTG",
        "GGGGGGGGGGG",
    ],
];
const CRATE: &[&str] = &[
    "ddddddddddd",
    "DEDDDDDDDED",
    "DDDDDDDDDDD",
    "ddddddddddd",
    "DEDDDDDDDED",
    "DDDDDDDDDDD",
    "ddddddddddd",
    "DEDDDDDDDED",
    "ddddddddddd",
];
/// The vendor, from the front, to her waist: braids tied with ribbons, a rebozo
/// crossed on her chest.
const VENDOR: &[&str] = &[
    "....NNNNNNN....",
    "...NNNNNNNNN...",
    "..NNNNNNNNNNN..",
    "..NNKKKKKKKNN..",
    "..NNKnKKKnKNN..",
    ".PNNKKKKKKKNNP.",
    ".NNNKKKkKKKNNN.",
    ".PN.BBKKKBB.NP.",
    "..BBBBBBBBBBB..",
    ".BBBBCBBBBCBBB.",
    "BBBCBBBBBBBBBBB",
    "BBBBBBBBBBCBBBB",
    "BBBBCBBBBBBBBBB",
    "BBbBbBbBbBbBbBB",
];

/// The look of a girl: the colors of her skirt and of its band.
const SKIRTS: [(u8, u8); 3] = [(b'P', b'Y'), (b'V', b'P'), (b'G', b'Y')];
/// The blanket's colors on a mulita (`B` and `Y` of its art).
const BLANKETS: [(u8, u8); 3] = [(b'B', b'Y'), (b'P', b'A'), (b'V', b'O')];

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Mulitas::new(w, h, rng))
}

struct Mulitas {
    t: f32,
    /// Art pixels walked per second.
    speed: f32,
    /// Going right to left?
    left: bool,
    w: i32,
    /// Screen pixels to an art pixel.
    k: f32,
    /// How many children cross, and the room they take together.
    n: usize,
    span: f32,
    /// Where the puesto's left edge is, in art pixels.
    stand: f32,
    seed: u32,
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

impl Mulitas {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Mulitas {
        let mut rng = fastrand::Rng::with_seed(rng.u64(..));
        let k = fit(h, HEIGHT, 0.85).min(whole(w as f32 / 100.0));
        let wide = w as f32 / k;
        let n = ((wide / 60.0).round() as usize).clamp(1, 5);
        let span = n as f32 * UNIT;
        Mulitas {
            t: 0.0,
            speed: (wide + span) / WALK,
            left: rng.bool(),
            w: w as i32,
            k,
            n,
            span,
            stand: (wide - STAND) * (0.35 + 0.3 * rng.f32()),
            seed: rng.u32(..),
        }
    }

    /// How far the puesto has risen: 0 under the ground, 1 standing.
    fn rise(&self) -> f32 {
        let up = ease_out(self.t / RISEN);
        let down = ((self.t - SINKING) / (END - SINKING)).clamp(0.0, 1.0);
        (up - down * down).clamp(0.0, 1.0)
    }

    /// Art pixels walked by the head of the group.
    fn dist(&self) -> f32 {
        self.speed * (self.t - FIRST).max(0.0)
    }
}

/// Where a thing is drawn: local coordinates are art pixels along the way a
/// walker goes (mirrored when it goes left) and up from the ground, at the
/// screen's scale.
#[derive(Clone, Copy)]
struct Place<'a> {
    pal: &'a Palette,
    k: f32,
    /// +1 to go right, -1 to go left (everything mirrored).
    s: f32,
    /// Screen column of local x 0, and the pixel row of the ground.
    ox: f32,
    ground: f32,
}

impl Place<'_> {
    /// The screen's column and pixel row of local (`x`, `y`).
    fn at(&self, x: f32, y: f32) -> (f32, f32) {
        (self.ox + self.s * x * self.k, self.ground - y * self.k)
    }

    /// A length of art pixels, in pixels of at least one.
    fn px(&self, art: f32) -> i32 {
        ((art * self.k).round() as i32).max(1)
    }

    /// Pixel art with its left edge at local `x` and its top `top` art pixels
    /// over the ground; turned `rot` radians about the middle of its top (a
    /// thing that hangs) and `size` times as big.
    fn sprite(
        &self,
        c: &mut Canvas,
        rows: &[&str],
        (x, top): (f32, f32),
        (rot, size): (f32, f32),
        color: impl Fn(u8) -> Option<Color>,
    ) {
        let art_w = rows.iter().map(|r| r.len()).max().unwrap_or(0) as f32;
        let w = art_w * size * self.k;
        let sx = self.ox + self.s * x * self.k - if self.s < 0.0 { w } else { 0.0 };
        let xf = Xf {
            pivot: (art_w * 0.5, 0.0),
            at: (sx + w * 0.5, self.ground - top * self.k),
            scale: (self.k * size, self.k * size),
            rot: rot * self.s,
            flip: self.s < 0.0,
        };
        blit(c, rows, xf, |_| 0, color);
    }

    /// A limb or a rope, `thick` art pixels across, between two local points.
    fn limb(&self, c: &mut Canvas, a: (f32, f32), b: (f32, f32), thick: f32, color: Color) {
        line(
            c,
            self.at(a.0, a.1),
            self.at(b.0, b.1),
            self.px(thick),
            color,
        );
    }

    /// A block of `w` by `h` art pixels, its left edge at local `x` and its
    /// bottom `y` art pixels over the ground.
    fn block(&self, c: &mut Canvas, (x, y): (f32, f32), (w, h): (f32, f32), color: Color) {
        let (x0, top) = self.at(x, y + h);
        let (x1, bottom) = self.at(x + w, y);
        let (left, right) = (x0.min(x1).round() as i32, x0.max(x1).round() as i32);
        let (top, bottom) = (top.round() as i32, bottom.round() as i32);
        fill(
            c,
            (left, top),
            ((right - left).max(1), (bottom - top).max(1)),
            color,
        );
    }

    /// A child with a huacal on the back and the mulita it leads, walking:
    /// `who` tells which (a boy or a girl, what they carry) and `phase` where
    /// in its step they are.
    fn child(&self, c: &mut Canvas, who: usize, phase: f32) {
        let pal = self.pal;
        let girl = who % 2 == 1;
        let load = (who / 2) % 3;
        let (skirt, band) = SKIRTS[(who / 3) % SKIRTS.len()];
        // A little rock with each step, one way for the child and the other
        // for the mulita.
        let bob = 0.9 * (0.5 - 0.5 * (phase * 2.0).cos());
        let bob_m = 0.5 - 0.5 * (phase * 2.0 + 1.2).cos();
        let (cx, huacal) = (40.0, 32.0);
        let hips = if girl { 3.5 } else { 10.0 };
        let top = hips + bob + if girl { GIRL.len() } else { BOY.len() } as f32;
        let colors = |l: u8| match l {
            b'Z' => pal.get(skirt),
            b'z' => pal.get(band),
            _ => pal.get(l),
        };

        // The mulita, behind.
        self.mulita(c, who, phase, bob_m);

        // The huacal on the child's back, rocking a little more than the child.
        let rock = 0.05 * (phase * 2.0 + 2.0).sin();
        let crate_top = 19.0 + bob;
        self.sprite(c, CRATE, (huacal, crate_top), (rock, 1.0), |l| pal.get(l));
        let loaded = HUACAL_LOADS[load];
        let load_top = crate_top + loaded.len() as f32 - 0.4;
        self.sprite(c, loaded, (huacal, load_top), (rock * 1.6, 1.0), |l| {
            pal.get(l)
        });

        // The legs: trousers of manta (a skirt hides most of a girl's), and
        // huaraches.
        let hip = (cx + 7.0, hips + bob);
        let reach = if girl { 3.0 } else { 3.6 };
        for j in [1.0, 0.0] {
            let psi = phase + j * PI;
            let foot = (hip.0 + psi.sin() * reach, psi.cos().max(0.0) * 2.0);
            if girl {
                self.limb(c, hip, (foot.0, foot.1 + 1.0), 1.4, pal.of(b'K'));
            } else {
                let cloth = if j > 0.5 { b'a' } else { b'A' };
                self.limb(c, hip, foot, 2.4, pal.of(cloth));
            }
            self.block(c, (foot.0 - 1.0, foot.1), (3.6, 1.4), pal.of(b'd'));
        }

        // The child, over the legs, and the strap of the huacal across the
        // forehead.
        let art = if girl { GIRL } else { BOY };
        self.sprite(c, art, (cx, top), (0.0, 1.0), colors);
        self.limb(
            c,
            (huacal + 10.0, crate_top - 1.0),
            (cx + 4.0, top - 6.0),
            1.0,
            pal.of(b'D'),
        );

        // The arm, back to the cord, which runs to the mulita's halter.
        let shoulder = (cx + 6.5, top - if girl { 9.5 } else { 12.0 });
        let hand = (cx - 3.0 + phase.sin() * 1.2, 8.5 + bob);
        let sleeve = if girl { b'B' } else { b'A' };
        self.limb(c, shoulder, hand, 2.6, pal.of(sleeve));
        self.block(c, (hand.0 - 1.0, hand.1 - 1.0), (2.0, 2.0), pal.of(b'K'));
        let snout = (27.0, 12.3 + 0.8 * bob_m);
        let mid = ((snout.0 + hand.0) / 2.0, (snout.1 + hand.1) / 2.0 - 2.0);
        self.limb(c, snout, mid, 0.8, pal.of(b'E'));
        self.limb(c, mid, hand, 0.8, pal.of(b'E'));
    }

    /// A mulita de hoja de maíz with a blanket and a load in its panniers,
    /// trotting on its husk legs.
    fn mulita(&self, c: &mut Canvas, who: usize, phase: f32, bob: f32) {
        let pal = self.pal;
        let (b1, b2) = BLANKETS[(who / 5) % BLANKETS.len()];
        let colors = |l: u8| match l {
            b'B' => pal.get(b1),
            b'Y' => pal.get(b2),
            _ => pal.get(l),
        };
        let legs = 5.0;
        let top = legs + MULITA.len() as f32 + 0.8 * bob;
        // Four legs, a diagonal pair at a time, the far ones in the shade.
        for (x, ph, far) in [
            (5.5, 0.0, true),
            (8.5, PI, false),
            (16.0, PI, true),
            (19.0, 0.0, false),
        ] {
            let psi = phase * 1.3 + ph;
            let foot = (x + psi.sin() * 2.2, psi.cos().max(0.0) * 1.6);
            let color = pal.of(if far { b'h' } else { b'H' });
            self.limb(c, (x, legs + 0.8 * bob), foot, 2.0, color);
            self.block(c, (foot.0 - 0.5, foot.1), (2.6, 1.0), pal.of(b'd'));
        }
        self.sprite(c, MULITA, (0.0, top), (0.0, 1.0), colors);
        // The load rides on the blanket, over the back.
        let load = LOADS[(who / 7) % LOADS.len()];
        self.sprite(c, load, (5.0, top - 4.0), (0.0, 1.0), |l| pal.get(l));
        // A flower between its ears and a ribbon at the halter.
        self.block(c, (20.2, top - 4.0), (1.5, 1.5), pal.of(b'P'));
        self.block(c, (23.5, top - 10.0), (1.0, 3.0), pal.of(b'P'));
    }
}

impl Egg for Mulitas {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::corpus(theme);
        let ground = c.h() as f32 * 2.0;
        let rise = self.rise();
        if rise > 0.0 {
            self.puesto(c, theme, &pal, ground, rise);
        }
        let dist = self.dist();
        let wide = self.w as f32 / self.k;
        for i in (0..self.n).rev() {
            let back = dist - self.span + (self.n - 1 - i) as f32 * UNIT;
            if back + UNIT < 0.0 || back > wide {
                continue;
            }
            // Local 0 is the back of the unit: the mulita's tail.
            let (ox, s) = if self.left {
                ((wide - back) * self.k, -1.0)
            } else {
                (back * self.k, 1.0)
            };
            let at = Place {
                pal: &pal,
                k: self.k,
                s,
                ox,
                ground,
            };
            let phase = dist / STRIDE * TAU + i as f32;
            at.child(c, (self.seed >> 3) as usize + i, phase);
        }
    }

    fn done(&self) -> bool {
        self.t > END || self.t > 29.5
    }
}

impl Mulitas {
    /// The puesto: an awning in the colors of paper flowers over a table in a
    /// cloth of rebozo blue, mulitas of every size on it and hanging from it,
    /// a big one beside it, and the vendor behind, calling out.
    fn puesto(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, ground: f32, rise: f32) {
        let k = self.k;
        let base = ground - DEPTH * k + (1.0 - rise) * (HEIGHT as f32 * k + 14.0);
        let at = Place {
            pal,
            k,
            s: 1.0,
            ox: self.stand * k,
            ground: base,
        };
        let stripes = *b"PYV";
        // The roof, behind everything: it narrows to its ridge.
        for r in 0..6 {
            let inset = 1.0 + r as f32 * 2.2;
            for i in 0..((STAND - 2.0 * inset) / 5.0).ceil() as i32 {
                let x = inset + i as f32 * 5.0;
                let w = 5.0f32.min(STAND - inset - x);
                let color = pal.of(stripes[(i as usize + 1) % 3]).scale(0.78);
                at.block(c, (x, 34.0 + r as f32), (w, 1.0), color);
            }
        }
        // The vendor behind the table, one arm up as she calls.
        let call = self.calling();
        let wave = (self.t * 3.1).sin() * if call.is_some() { 1.0 } else { 0.25 };
        let vendor = (STAND * 0.5 - 7.5, 12.0 + VENDOR.len() as f32);
        let hand = (vendor.0 + 19.0 + wave * 1.5, 22.0 + wave * 2.0);
        at.limb(
            c,
            (vendor.0 + 13.0, vendor.1 - 8.5),
            hand,
            3.0,
            pal.of(b'B'),
        );
        at.sprite(c, VENDOR, vendor, (0.0, 1.0), |l| pal.get(l));
        at.block(c, (hand.0 - 1.0, hand.1), (2.5, 2.5), pal.of(b'K'));
        // The posts, the table's board and its cloth.
        for x in [1.0, STAND - 3.0] {
            at.block(c, (x, 0.0), (1.0, 34.0), pal.of(b'D'));
            at.block(c, (x + 1.0, 0.0), (1.0, 34.0), pal.of(b'd'));
        }
        at.block(c, (3.0, 0.0), (STAND - 6.0, 11.0), pal.of(b'B'));
        at.block(c, (3.0, 4.0), (STAND - 6.0, 1.0), pal.of(b'C'));
        for x in 0..(STAND as i32 - 6) {
            let px = 3.0 + x as f32;
            if x % 3 == 0 {
                at.block(c, (px, 0.0), (1.0, 1.0), pal.of(b'b'));
            }
            // A band of embroidery: a zigzag in amarillo with rosa between.
            let up = (x % 4 < 2) as i32 as f32;
            at.block(c, (px, 7.0 + up), (1.0, 1.0), pal.of(b'Y'));
            if x % 4 == 0 {
                at.block(c, (px, 6.0), (1.0, 1.0), pal.of(b'P'));
            }
        }
        at.block(c, (0.0, 11.0), (STAND, 2.0), pal.of(b'D'));
        at.block(c, (0.0, 12.0), (STAND, 1.0), pal.of(b'E'));
        // Mulitas of the size of a hand on the table, each its own color.
        for (i, x) in [(0usize, 3.0), (1, STAND - 17.0)] {
            let (own, _) = BLANKETS[(i + self.seed as usize) % BLANKETS.len()];
            let paint = |l: u8| pal.get(if l == b'x' { own } else { l });
            at.sprite(c, MINI, (x, 13.0 + MINI.len() as f32), (0.0, 1.0), paint);
        }
        // The awning's front, in stripes of five with scalloped edges.
        for i in 0..(STAND / 5.0).ceil() as i32 {
            let x = 5.0 * i as f32;
            let w = 5.0f32.min(STAND - x);
            let color = pal.of(stripes[i as usize % 3]);
            at.block(c, (x, 29.0), (w, 5.0), color);
            at.block(c, (x + 1.0, 28.0), ((w - 2.0).max(1.0), 1.0), color);
        }
        at.block(c, (0.0, 33.0), (STAND, 1.0), pal.of(b'd'));
        // Tiny mulitas on strings from the scallops, swaying.
        for (n, i) in [0, 1, 2, 7, 8, 9].into_iter().enumerate() {
            let len = [3.0, 6.0, 4.0][n % 3];
            let sway = 0.22 * (self.t * 1.7 + n as f32 * 1.9).sin();
            let x = 5.0 * i as f32 + 2.5;
            at.limb(c, (x, 28.0), (x, 28.0 - len), 0.6, pal.of(b'E'));
            let paint = |l: u8| pal.get(if l == b'x' { b'P' } else { l });
            let w = 14.0 * 0.25;
            at.sprite(c, MINI, (x - w, 28.0 - len), (sway, 0.5), paint);
        }
        // A big one beside the puesto, if there is room for it, looking at it.
        if self.stand + STAND + 34.0 < self.w as f32 / k {
            let paint = |l: u8| pal.get(if l == b'B' { b'P' } else { l });
            let beside = Place {
                s: -1.0,
                ox: at.ox + (STAND + 6.0 + 29.0) * k,
                ..at
            };
            beside.sprite(c, MULITA, (0.0, 21.0), (0.0, 1.0), paint);
        }
        // Her call, over the awning, rising a little as she says it.
        if let Some(age) = call {
            let text = if self.w >= 40 {
                "¡Mulitas, mulitas de hoja!"
            } else {
                "¡Mulitas de hoja!"
            };
            let (mid, peak) = at.at(STAND * 0.5, 41.0);
            let row = (peak / 2.0).floor() as i32 - 1 - (age * 2.0).min(1.0) as i32;
            let gone = ((age - (CALL - 0.5)) / 0.5).clamp(0.0, 1.0);
            let color = fade(theme, pal.of(b'Y'), gone);
            let left = mid.round() as i32 - text.chars().count() as i32 / 2;
            c.text_attr(left, row.max(0), text, color, BOLD);
        }
    }

    /// How long the vendor has been calling out, if she is.
    fn calling(&self) -> Option<f32> {
        let t = self.t - CALLS.0;
        if !(0.0..=CALLS.1 - CALLS.0).contains(&t) {
            return None;
        }
        let age = t % (CALL + REST);
        (age < CALL).then_some(age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;

    /// The egg of a screen, going the way it is told.
    fn egg(w: u16, h: u16, left: bool) -> Mulitas {
        let mut m = Mulitas::new(w, h, &mut fastrand::Rng::with_seed(5));
        m.left = left;
        m
    }

    #[test]
    fn the_art_is_whole() {
        for (name, art) in [
            ("mulita", MULITA),
            ("mini", MINI),
            ("boy", BOY),
            ("girl", GIRL),
            ("crate", CRATE),
            ("vendor", VENDOR),
        ] {
            let w = art[0].len();
            assert!(art.iter().all(|r| r.len() == w), "{name}");
        }
        for loads in [LOADS.as_slice(), HUACAL_LOADS.as_slice()] {
            for art in loads {
                assert!(art.iter().all(|r| r.len() == art[0].len()));
            }
            assert!(loads.iter().all(|a| a[0].len() == loads[0][0].len()));
        }
        // Every letter of the art has a color (but `x`, `Z` and `z`, which the
        // look picks, and the shared letters).
        let pal = Palette::corpus(&Theme::classic());
        let mut letters = Vec::new();
        for art in [MULITA, MINI, BOY, GIRL, CRATE, VENDOR] {
            letters.extend(art.iter().flat_map(|r| r.bytes()));
        }
        for art in LOADS.iter().chain(&HUACAL_LOADS) {
            letters.extend(art.iter().flat_map(|r| r.bytes()));
        }
        for l in letters {
            assert!(
                matches!(l, b'.' | b'x' | b'Z' | b'z') || pal.get(l).is_some(),
                "{}",
                l as char
            );
        }
    }

    /// The puesto rises, stands through the walk and sinks again.
    #[test]
    fn the_puesto_rises_and_sinks() {
        let mut m = egg(100, 30, false);
        let mut rise = |t: f32| {
            m.t = t;
            m.rise()
        };
        assert_eq!(rise(0.0), 0.0);
        assert!(rise(RISEN) > 0.99);
        assert!(rise(FIRST + WALK / 2.0) > 0.99);
        assert!(rise(SINKING + 1.0) < rise(SINKING));
        assert_eq!(rise(END), 0.0);
    }

    /// The vendor calls out in turns, and not before or after the walk.
    #[test]
    fn the_vendor_calls_in_turns() {
        let mut m = egg(100, 30, false);
        let mut calls = |t: f32| {
            m.t = t;
            m.calling()
        };
        assert!(calls(0.5).is_none());
        assert!(calls(CALLS.0 + 0.1).is_some());
        assert!(calls(CALLS.0 + CALL + 0.1).is_none());
        assert!(calls(CALLS.0 + CALL + REST + 0.1).is_some());
        assert!(calls(CALLS.1 + 1.0).is_none());
        assert!(calls(END).is_none());
    }

    /// Children, mulitas and puesto show their colors, in either direction,
    /// the call is written, everyone has crossed by the end and nothing is
    /// left on the screen.
    #[test]
    fn the_children_cross_and_nothing_is_left() {
        let theme = Theme::classic();
        let wanted = [
            0xefe6cf, 0x2a5caa, 0xa0703a, 0xd9c48a, 0xffb02e, 0xc2185b, 0x7cb342, 0xff8c1a,
            0xe4007c, 0xffd200, 0x7028a0,
        ];
        for left in [false, true] {
            let mut m = egg(120, 36, left);
            let mut c = Canvas::new(120, 36, Color::Reset);
            let mut seen = [false; 11];
            let mut called = false;
            let (mut least, mut most) = (i32::MAX, 0);
            while !m.done() {
                m.update(0.1);
                c.clear();
                m.draw(&mut c, &theme);
                called |= c.to_text().contains("¡Mulitas, mulitas de hoja!");
                for (i, hex) in wanted.iter().enumerate() {
                    seen[i] |= c.cells().iter().any(|cell| cell.fg == rgb(*hex));
                }
                // The skin of a child: where the walkers are.
                for (i, cell) in c.cells().iter().enumerate() {
                    if cell.fg == rgb(0xd7a066) {
                        least = least.min(i as i32 % 120);
                        most = most.max(i as i32 % 120);
                    }
                }
            }
            assert!(seen.iter().all(|&s| s), "{left}: {seen:?}");
            assert!(called, "{left}: no call");
            assert!(least < 10 && most > 110, "{left}: {least} to {most}");
            let wide = 120.0 / m.k;
            assert!(m.dist() >= wide + m.span - 1.0, "{left}");
            assert!(c.cells().iter().all(|cell| cell.ch == ' '), "{left}");
        }
    }

    /// A narrow screen gets the short call, and children and mulita are
    /// scaled to it.
    #[test]
    fn a_narrow_screen_gets_the_short_call() {
        let mut m = egg(30, 12, false);
        let mut c = Canvas::new(30, 12, Color::Reset);
        m.t = CALLS.0 + 0.6;
        m.draw(&mut c, &Theme::classic());
        let text = c.to_text();
        assert!(text.contains("¡Mulitas de hoja!"));
        assert!(!text.contains("mulitas de hoja"));
        assert!(m.k < 1.0);
    }

    /// The walk sets the pace of the steps: not a step without the group
    /// moving.
    #[test]
    fn the_group_moves_at_one_pace() {
        let m = egg(200, 60, false);
        let wide = 200.0 / m.k;
        let mut at = Mulitas {
            t: FIRST + 3.0,
            ..m
        };
        let before = at.dist();
        at.t += 1.0;
        assert!((at.dist() - before - at.speed).abs() < 1e-3);
        assert!(at.speed * WALK >= wide, "{} {wide}", at.speed);
        assert!(at.n >= 2);
    }
}
