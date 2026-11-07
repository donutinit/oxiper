//! What the Virgen de Guadalupe visitors share: the colors, the way they scale
//! their art, and the few things that more than one of them draws (the
//! devotional image, the Basílica, roses and veladoras).
//!
//! Under `classic` the colors are those of the twelfth of December: the rosas
//! de Castilla that Juan Diego carried in his tilma, in two pinks among their
//! leaves; the Virgen's mantle verde azulado with its gold stars, her rose
//! gown and the golden rays around her; the veladoras, a flame in a glass of
//! rojo, verde or white; the cantera of the Basílica with its roofs gone
//! copper-green; and, for the matachines, red and white skirts and penachos of
//! feathers in every color. Under any other theme each hue becomes the theme
//! accent nearest to it, shades are taken from those, whites and the stone are
//! the theme's foreground and the dark inks and the silver are mixed from its
//! background.
//!
//! The Virgen's image only ever appears as what it is for the people who carry
//! it: the picture on an estandarte, a framed image on a pilgrim's back, in the
//! niche of the Basílica or on an arch of roses. It is drawn still and upright,
//! never moved about or made into anything else.

use super::blit::{Xf, blit};
use super::paint;
use super::palette::Role;
pub use super::palette::{Palette, fade};
use crate::color::Color;
use crate::render::Canvas;
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Rosas de Castilla: rosa, its light and its dark.
    (b'R', 0xd81b60, Role::Hue),
    (b'r', 0xf06292, Role::Shade(b'R', 1.45)),
    (b'q', 0x9c1346, Role::Shade(b'R', 0.72)),
    // The rose's leaves: verde, its dark and its light.
    (b'G', 0x2e7d32, Role::Hue),
    (b'g', 0x1b5e20, Role::Shade(b'G', 0.62)),
    (b'H', 0x5da85a, Role::Shade(b'G', 1.4)),
    // The Virgen's mantle verde azulado, its dark and its light.
    (b'T', 0x00796b, Role::Hue),
    (b't', 0x004d44, Role::Shade(b'T', 0.62)),
    (b'U', 0x26a69a, Role::Shade(b'T', 1.4)),
    // Gold: the stars of the mantle, the rays, a glint and brass in shade.
    (b'Y', 0xf2c14e, Role::Hue),
    (b'y', 0xf2b632, Role::Shade(b'Y', 0.95)),
    (b'Z', 0xfff0b0, Role::Shade(b'Y', 1.35)),
    (b'd', 0xa8741c, Role::Shade(b'Y', 0.62)),
    // Her gown, rose.
    (b'E', 0xe57373, Role::Hue),
    // The flame of a veladora and its glint; the glass in rojo and verde.
    (b'F', 0xffcc4d, Role::Keep),
    (b'X', 0xfff4c0, Role::Keep),
    (b'V', 0xc62828, Role::Hue),
    (b'v', 0x2e9a5a, Role::Hue),
    // White: wax, manta and a shirt, and its shade.
    (b'W', 0xf4f1ea, Role::Bone(1.0)),
    (b'w', 0xbdb8ae, Role::Bone(0.72)),
    // The cantera of the Basílica: lit, in shade and in its darkest, and the
    // dark of its doors and windows.
    (b'C', 0xbfa98a, Role::Bone(0.82)),
    (b'c', 0xa08a6c, Role::Bone(0.62)),
    (b'S', 0x4a3f36, Role::Ink(0.2)),
    // Copper gone green: the domes, and their shade.
    (b'J', 0x4f8f7a, Role::Hue),
    (b'j', 0x36685a, Role::Shade(b'J', 0.7)),
    // Manta (cotton) and its shade.
    (b'A', 0xefe6cf, Role::Bone(0.95)),
    (b'a', 0xc9bfa6, Role::Bone(0.72)),
    // The blue of a rebozo or a shirt, and its dark.
    (b'B', 0x2a5caa, Role::Hue),
    (b'b', 0x1c3f78, Role::Shade(b'B', 0.65)),
    // Skin, and its shade.
    (b'K', 0xd7a066, Role::Hue),
    (b'k', 0xa87040, Role::Shade(b'K', 0.78)),
    // The dark inks: hair, a charro suit and its highlights.
    (b'N', 0x2a1a14, Role::Ink(0.2)),
    (b'n', 0x1c1424, Role::Ink(0.06)),
    (b'z', 0x4a3c5e, Role::Ink(0.3)),
    // The matachines: rojo and its shade.
    (b'M', 0xc8102e, Role::Hue),
    (b'm', 0x8a0a20, Role::Shade(b'M', 0.65)),
    // Penachos and ribbons in every color: rosa mexicano, amarillo, turquesa,
    // morado and naranja.
    (b'P', 0xe4007c, Role::Hue),
    (b'L', 0xffd200, Role::Hue),
    (b'Q', 0x00aab8, Role::Hue),
    (b'I', 0x7028a0, Role::Hue),
    (b'O', 0xff8c1a, Role::Hue),
    // Wood, its dark, and straw.
    (b'D', 0xa0703a, Role::Shade(b'O', 0.8)),
    (b'u', 0x6a4520, Role::Shade(b'O', 0.5)),
    (b'h', 0xc9a96a, Role::Shade(b'Y', 0.9)),
    // Steel, and silver.
    (b's', 0x9ea3b2, Role::Ink(0.58)),
    (b'i', 0xdadde6, Role::Ink(0.84)),
];

impl Palette {
    /// The Virgen de Guadalupe visitors' colors under a theme.
    pub fn guadalupe(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

/// The image of the Virgen, as it is printed on an estandarte, framed on a
/// pilgrim's back or set in a niche: the mantle verde azulado with its gold
/// stars, her rose gown, hands together, the crescent at her feet and the rays
/// of the mandorla all around.
pub const VIRGEN: &[&str] = &[
    "......ZZyyy......",
    "....yZtttttyZ....",
    "...yytTTTTTtZZ...",
    "...yytTTYTTtZZ...",
    "..ZyytTkkkTtZZy..",
    ".ZZZytTkKKTtZyyy.",
    ".ZZZytTTkKTtZyyy.",
    ".ZZZtTTTkTTTtyyy.",
    "yyZZtTTTTTTTtyyZZ",
    "yyytTTYEEETYTtZZZ",
    "yyytTTEEEEETTtZZZ",
    "yyytTYEEKEEYTtZZZ",
    "yyytTTEEEEETTtyyy",
    "ZZZtTYEEEEEYTtyyy",
    "ZZZtTTEEEEETTtyyy",
    "ZZZtTTEEEEETTtyyy",
    "ZZytTYTEEETYTtZyy",
    ".yytTTTEEETTTtZZ.",
    ".yytTTYEEETYTtZZ.",
    ".yyytTTEEETTtZZZ.",
    "..yZZnnnnnnnyyZ..",
    "...ZnNNNNNNNny...",
    "...ZZnNyyZNnyy...",
    "....ZyyyyZZZy....",
    "......yyyZZ......",
];
/// The image small enough for a niche in the Basílica's front.
const NICHO: &[&str] = &[
    "...y...", "..yTy..", ".yTkTy.", ".yTKTy.", "yTTETTy", "yTEEETy", "yTTETTy", ".nnnnn.",
];
/// A rosa de Castilla in bloom, with its leaf.
pub const ROSA: &[&str] = &[".rRr.", "rRqRr", "RqRqR", ".RRR.", "..G.."];
/// A bouquet of roses in their leaves, wrapped at the stems.
pub const RAMO: &[&str] = &[
    "...rr.rr...",
    "..rRRrRRr..",
    "..RqRRRqR..",
    ".GRRqRqRRG.",
    "GG.GRRRG.GG",
    ".G..GGG..G.",
    "....uuu....",
    "....AAA....",
];
/// A veladora's glass (`x` is a letter that its maker picks).
const VELADORA: &[&str] = &["xxx", "xWx", "xxx", "xxx"];

/// Fills the pixels from `from` up to (not including) `to`, painting both
/// halves of a cell at once; whatever is off the screen is left out.
fn fill_px(c: &mut Canvas, (x0, y0): (i32, i32), (x1, y1): (i32, i32), color: Color) {
    let (x0, x1) = (x0.max(0), x1.min(c.w() as i32));
    let (y0, y1) = (y0.max(0), y1.min(c.h() as i32 * 2));
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    for row in y0.div_euclid(2)..=(y1 - 1).div_euclid(2) {
        let up = (y0..y1).contains(&(row * 2)).then_some(color);
        let down = (y0..y1).contains(&(row * 2 + 1)).then_some(color);
        for col in x0..x1 {
            paint(c, col, row, up, down);
        }
    }
}

/// The scale to draw art `art_h` pixels tall and at least `art_w` wide on a
/// screen of `cols` by `rows` cells: the biggest one that keeps it on the
/// screen (whole numbers from 1 to 6, fourths below 1), so the cross on top
/// of an estandarte is never cut off.
pub fn fit_down(rows: u16, cols: u16, art_h: i32, art_w: f32) -> f32 {
    let k = (rows as f32 * 2.0 / art_h.max(1) as f32).min(cols as f32 / art_w.max(1.0));
    if k >= 1.0 {
        k.floor().min(6.0)
    } else {
        ((k * 4.0).floor() / 4.0).max(0.25)
    }
}

/// How wide and tall the Basílica is when drawn whole, in art pixels.
pub const BASILICA: (f32, f32) = (74.0, 59.0);

/// Where a thing is drawn: local coordinates are art pixels along the way a
/// walker goes (mirrored when it goes left) and up from the ground, at the
/// screen's scale.
#[derive(Clone, Copy)]
pub struct Place<'a> {
    pub pal: &'a Palette,
    /// Screen pixels to an art pixel.
    pub k: f32,
    /// +1 to go right, -1 to go left (everything mirrored).
    pub s: f32,
    /// Screen column of local x 0, and the pixel row of the ground.
    pub ox: f32,
    pub ground: f32,
}

impl Place<'_> {
    /// The screen's column and pixel row of local (`x`, `y`).
    pub fn at(&self, x: f32, y: f32) -> (f32, f32) {
        (self.ox + self.s * x * self.k, self.ground - y * self.k)
    }

    /// A length of art pixels, in pixels of at least one.
    pub fn px(&self, art: f32) -> i32 {
        ((art * self.k).round() as i32).max(1)
    }

    /// The same place `f` times as big, from the same ground.
    pub fn scaled(&self, f: f32) -> Self {
        Place {
            k: self.k * f,
            ..*self
        }
    }

    /// Pixel art with its left edge at local `x` and its top `top` art pixels
    /// over the ground; turned `rot` radians about the middle of its top (a
    /// thing that hangs) and `size` times as big.
    pub fn sprite(
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

    /// A limb or a rope, `thick` art pixels across, between two local points:
    /// squares of that size laid along it, each overlapping half the last.
    pub fn limb(&self, c: &mut Canvas, a: (f32, f32), b: (f32, f32), thick: f32, color: Color) {
        let ((x0, y0), (x1, y1)) = (self.at(a.0, a.1), self.at(b.0, b.1));
        let side = self.px(thick);
        let step = (side as f32 * 0.5).max(1.0);
        let steps = ((x1 - x0).abs().max((y1 - y0).abs()) / step)
            .ceil()
            .max(1.0) as i32;
        let back = side / 2;
        for s in 0..=steps {
            let t = s as f32 / steps as f32;
            let (x, y) = (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t);
            let (x, y) = (x.round() as i32 - back, y.round() as i32 - back);
            fill_px(c, (x, y), (x + side, y + side), color);
        }
    }

    /// A block of `w` by `h` art pixels, its left edge at local `x` and its
    /// bottom `y` art pixels over the ground.
    pub fn block(&self, c: &mut Canvas, (x, y): (f32, f32), (w, h): (f32, f32), color: Color) {
        let (x0, top) = self.at(x, y + h);
        let (x1, bottom) = self.at(x + w, y);
        let (left, right) = (x0.min(x1).round() as i32, x0.max(x1).round() as i32);
        let (top, bottom) = (top.round() as i32, bottom.round() as i32);
        fill_px(
            c,
            (left, top),
            (right.max(left + 1), bottom.max(top + 1)),
            color,
        );
    }

    /// A filled disc of `r` art pixels about local (`x`, `y`).
    pub fn disc(&self, c: &mut Canvas, (x, y): (f32, f32), r: f32, color: Color) {
        let r = r.max(0.5);
        let rows = (2.0 * r).ceil() as i32;
        for i in 0..rows {
            let dy = (i as f32 + 0.5) - r;
            let half = (r * r - dy * dy).max(0.0).sqrt();
            self.block(
                c,
                (x - half, y + r - 1.0 - i as f32),
                (2.0 * half, 1.0),
                color,
            );
        }
    }

    /// Pixel art that is never mirrored, whichever way this place faces (the
    /// image of the Virgen is not): its left edge at local `x` as for
    /// [`Place::sprite`], its top `top` art pixels over the ground.
    fn upright(&self, c: &mut Canvas, rows: &[&str], (x, top): (f32, f32), size: f32) {
        let pal = self.pal;
        let w = rows.iter().map(|r| r.len()).max().unwrap_or(0) as f32 * size;
        let left = if self.s < 0.0 { -(x + w) } else { x };
        let here = Place {
            s: 1.0,
            ox: self.ox + left * self.k,
            ..*self
        };
        here.sprite(c, rows, (0.0, top), (0.0, size), |l| pal.get(l));
    }

    /// The image of the Virgen, whole, its top `top` art pixels over the
    /// ground and its left edge at local `x`. It is only ever drawn upright
    /// and still, as it is on an estandarte, a cuadro or a niche.
    pub fn virgen(&self, c: &mut Canvas, (x, top): (f32, f32), size: f32) {
        self.upright(c, VIRGEN, (x, top), size);
    }

    /// The image in a frame of wood with a gilded edge: `VIRGEN` is 17 by 25
    /// and the frame two more on each side. Its top-left corner at local
    /// (`x`, `top`).
    pub fn framed(&self, c: &mut Canvas, (x, top): (f32, f32), size: f32) {
        let pal = self.pal;
        let (w, h) = (21.0 * size, 29.0 * size);
        let bottom = top - h;
        self.block(c, (x, bottom), (w, h), pal.of(b'u'));
        let e = size.max(0.5);
        self.block(
            c,
            (x + e, bottom + e),
            (w - 2.0 * e, h - 2.0 * e),
            pal.of(b'D'),
        );
        self.block(
            c,
            (x + 1.6 * size, bottom + 1.6 * size),
            (w - 3.2 * size, h - 3.2 * size),
            pal.of(b'd'),
        );
        self.block(
            c,
            (x + 2.0 * size, bottom + 2.0 * size),
            (17.0 * size, 25.0 * size),
            pal.of(b'S'),
        );
        self.virgen(c, (x + 2.0 * size, top - 2.0 * size), size);
    }

    /// A veladora: a glass of the color of the letter `glass` with a flame
    /// that flickers with `t` (a phase in seconds), its base on the ground
    /// at local `x`.
    pub fn veladora(&self, c: &mut Canvas, (x, base): (f32, f32), glass: u8, t: f32) {
        let pal = self.pal;
        self.sprite(c, VELADORA, (x, base + 4.0), (0.0, 1.0), |l| {
            pal.get(if l == b'x' { glass } else { l })
        });
        let lick = (t * 9.0 + x).sin() * 0.5 + (t * 5.3 + x * 2.0).sin() * 0.5;
        self.block(c, (x + 1.0, base + 4.0), (1.0, 1.0), pal.of(b'u'));
        self.block(
            c,
            (x + 1.0, base + 5.0),
            (1.0, 1.4 + 0.4 * lick),
            pal.of(b'F'),
        );
        self.block(c, (x + 1.0, base + 5.0), (1.0, 0.8), pal.of(b'X'));
    }

    /// The Basílica de Guadalupe seen from the front, centered on local `x`,
    /// whole when `size` is 1 (see [`BASILICA`]): the nave with its portal and
    /// the niche of the Virgen over it, the two bell towers with their clocks
    /// and bells under domes gone green, and the great dome behind. `glow` (0
    /// to 1) lights its windows and its door from within.
    pub fn basilica(&self, c: &mut Canvas, x: f32, glow: f32) {
        let pal = self.pal;
        let (stone, shade, dark) = (pal.of(b'C'), pal.of(b'c'), pal.of(b'S'));
        let lit = dark.lerp(pal.of(b'F'), glow.clamp(0.0, 1.0));
        let (roof, roof_dark, gold) = (pal.of(b'J'), pal.of(b'j'), pal.of(b'Y'));
        let blk = |c: &mut Canvas, dx: f32, y: f32, w: f32, h: f32, color: Color| {
            self.block(c, (x + dx, y), (w, h), color);
        };
        // A dome of `r` art pixels with its base at `y`, in two shades, with
        // a rib, a lantern and a gold cross.
        let dome = |c: &mut Canvas, dx: f32, y: f32, r: f32| {
            for i in 0..r.ceil() as i32 {
                let dy = i as f32 + 0.5;
                let half = (r * r - dy * dy).max(0.0).sqrt();
                blk(c, dx - half, y + i as f32, 2.0 * half, 1.0, roof);
                blk(
                    c,
                    dx + half * 0.35,
                    y + i as f32,
                    half * 0.65,
                    1.0,
                    roof_dark,
                );
                blk(c, dx - 0.5, y + i as f32, 1.0, 1.0, roof_dark);
            }
            let top = y + r;
            blk(c, dx - 1.5, top, 3.0, 3.0, stone);
            blk(c, dx - 0.5, top + 0.8, 1.0, 1.4, dark);
            blk(c, dx - 2.0, top + 3.0, 4.0, 1.0, shade);
            blk(c, dx - 1.0, top + 4.0, 2.0, 1.0, roof);
            blk(c, dx - 0.5, top + 5.0, 1.0, 4.0, gold);
            blk(c, dx - 1.5, top + 7.0, 3.0, 1.0, gold);
        };

        // The great dome behind: its drum with arched windows, then the dome.
        blk(c, -9.0, 26.0, 18.0, 11.0, stone);
        blk(c, 4.0, 26.0, 5.0, 11.0, shade);
        for dx in [-6.0, -1.0, 4.0] {
            blk(c, dx - 1.0, 29.0, 2.0, 5.0, lit);
            blk(c, dx - 0.5, 34.0, 1.0, 1.0, lit);
        }
        blk(c, -10.0, 36.0, 20.0, 2.0, shade);
        dome(c, 0.0, 38.0, 9.5);

        // The nave: courses of stone, a cornice, the gable with its oculus.
        blk(c, -17.0, 2.0, 34.0, 24.0, stone);
        for row in 0..6 {
            let y = 4.0 + row as f32 * 4.0;
            for seg in 0..7 {
                let x0 = -17.0 + seg as f32 * 5.0 + (row % 2) as f32 * 2.5;
                blk(c, x0.max(-17.0), y, 4.0f32.min(17.0 - x0), 1.0, shade);
            }
        }
        blk(c, -19.0, 26.0, 38.0, 2.0, shade);
        blk(c, -18.0, 28.0, 36.0, 1.0, stone);
        for r in 0..8 {
            let half = 17.0 - r as f32 * 2.1;
            blk(c, -half, 29.0 + r as f32, 2.0 * half, 1.0, stone);
            blk(c, -half, 29.0 + r as f32, 1.2, 1.0, shade);
            blk(c, half - 1.2, 29.0 + r as f32, 1.2, 1.0, shade);
        }
        self.disc(c, (x, 32.5), 2.2, shade);
        self.disc(c, (x, 32.5), 1.5, lit);

        // The portal with its door lit from within, flanked by columns, and
        // the niche of the Virgen over it.
        blk(c, -6.0, 2.0, 12.0, 14.0, shade);
        blk(c, -4.5, 2.0, 9.0, 11.0, lit);
        blk(c, -3.5, 13.0, 7.0, 1.0, lit);
        blk(c, -2.0, 14.0, 4.0, 1.0, lit);
        blk(c, -6.0, 15.0, 12.0, 1.0, stone);
        blk(c, -4.0, 17.0, 8.0, 10.0, shade);
        blk(c, -3.5, 17.5, 7.0, 8.0, dark);
        self.upright(c, NICHO, (x - 3.5, 25.5), 1.0);
        // Windows either side of the portal.
        for dx in [-12.0, 10.0] {
            blk(c, dx - 0.5, 9.0, 4.0, 11.0, shade);
            blk(c, dx, 9.5, 3.0, 9.0, lit);
            blk(c, dx + 0.5, 18.5, 2.0, 1.0, lit);
        }

        // The towers: pilasters, a belfry with its bell, a clock at twelve.
        for side in [-1.0f32, 1.0] {
            let x0 = if side < 0.0 { -31.0 } else { 18.0 };
            let mid = x0 + 6.5;
            blk(c, x0, 2.0, 13.0, 34.0, stone);
            blk(
                c,
                x0 + if side < 0.0 { 9.5 } else { 7.0 },
                2.0,
                3.5,
                34.0,
                shade,
            );
            blk(c, x0, 2.0, 1.0, 34.0, shade);
            for row in 0..8 {
                let y = 6.0 + row as f32 * 4.0;
                if !(24.0..34.0).contains(&y) {
                    blk(c, x0 + 1.0, y, 11.0, 1.0, shade);
                }
            }
            blk(c, mid - 2.5, 24.0, 5.0, 8.0, dark);
            blk(c, mid - 1.5, 32.0, 3.0, 1.0, dark);
            blk(c, mid - 1.0, 28.0, 2.0, 2.0, gold);
            blk(c, mid - 0.5, 30.0, 1.0, 2.0, gold);
            self.disc(c, (mid, 17.0), 3.3, shade);
            self.disc(c, (mid, 17.0), 2.7, pal.of(b'W'));
            blk(c, mid - 0.5, 17.0, 1.0, 2.2, pal.of(b'n'));
            blk(c, x0 - 1.0, 36.0, 15.0, 2.0, shade);
            blk(c, x0, 38.0, 13.0, 1.0, stone);
            dome(c, mid, 39.0, 6.5);
        }

        // Steps across the front, the atrium's stone at their foot.
        blk(c, -9.0, 0.0, 18.0, 2.0, shade);
        blk(c, -37.0, 0.0, 74.0, 1.0, shade);
    }

    /// What the faithful leave at the Basílica's door, for a Basílica
    /// centered on local x 0: veladoras along the steps, burning (`t` makes
    /// their flames flicker), and a ramo of roses at each side of the portal.
    pub fn offerings(&self, c: &mut Canvas, t: f32) {
        let pal = self.pal;
        for (i, glass) in b"VWvWV".iter().enumerate() {
            self.veladora(c, (-9.0 + i as f32 * 4.0, 2.0), *glass, t + i as f32 * 0.7);
        }
        for x in [-18.0, 11.0] {
            self.sprite(c, RAMO, (x, 10.0), (0.0, 1.0), |l| pal.get(l));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ALL, Eggs};
    use super::*;
    use crate::color::rgb;
    use crate::fiesta::{Fiesta, Kind};

    #[test]
    fn every_letter_has_a_color_under_every_theme() {
        for name in ["classic", "dracula", "nord", "gruvbox", "mono"] {
            let Some(t) = Theme::builtin(name) else {
                continue;
            };
            let pal = Palette::guadalupe(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::guadalupe(&Theme::classic());
        assert_eq!(pal.of(b'R'), rgb(0xd81b60));
        assert_eq!(pal.of(b'T'), rgb(0x00796b));
        assert_eq!(pal.of(b'Y'), rgb(0xf2c14e));
        assert_eq!(pal.of(b'C'), rgb(0xbfa98a));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::guadalupe(&t);
        assert!(t.colors.contains(&pal.of(b'R')));
        assert!(t.colors.contains(&pal.of(b'T')));
    }

    #[test]
    fn art_fits_the_screen_it_is_given() {
        assert_eq!(fit_down(30, 100, 60, 100.0), 1.0);
        assert_eq!(fit_down(30, 100, 62, 100.0), 0.75);
        assert_eq!(fit_down(24, 80, 58, 100.0), 0.75);
        assert_eq!(fit_down(60, 200, 58, 100.0), 2.0);
        assert_eq!(fit_down(150, 500, 58, 100.0), 5.0);
        assert_eq!(fit_down(1, 1, 58, 100.0), 0.25);
        assert_eq!(fit_down(0, 0, 58, 100.0), 0.25);
        for (rows, cols) in [(20, 100), (24, 80), (30, 100), (50, 200), (90, 300)] {
            let k = fit_down(rows, cols, 58, 100.0);
            assert!(58.0 * k <= rows as f32 * 2.0 + 0.01, "{rows}x{cols}");
            assert!(100.0 * k <= cols as f32 + 0.01, "{rows}x{cols}");
        }
    }

    #[test]
    fn the_art_is_whole_and_colored() {
        let pal = Palette::guadalupe(&Theme::classic());
        for (name, art) in [
            ("virgen", VIRGEN),
            ("nicho", NICHO),
            ("rosa", ROSA),
            ("ramo", RAMO),
            ("veladora", VELADORA),
        ] {
            let w = art[0].len();
            assert!(art.iter().all(|r| r.len() == w), "{name}");
            for l in art.iter().flat_map(|r| r.bytes()) {
                assert!(
                    matches!(l, b'.' | b'x') || pal.get(l).is_some(),
                    "{name}: {}",
                    l as char
                );
            }
        }
        assert_eq!((VIRGEN[0].len(), VIRGEN.len()), (17, 25));
    }

    /// The Basílica is on the canvas where it should be: stone, green domes,
    /// gold crosses, and it stays inside its stated size.
    #[test]
    fn the_basilica_stands_inside_its_size() {
        let pal = Palette::guadalupe(&Theme::classic());
        let (w, h) = (100u16, 40u16);
        let mut c = Canvas::new(w, h, Color::Reset);
        let at = Place {
            pal: &pal,
            k: 1.0,
            s: 1.0,
            ox: 0.0,
            ground: h as f32 * 2.0,
        };
        at.basilica(&mut c, 50.0, 1.0);
        let mut seen = [false; 4];
        let (mut lo, mut hi) = ((i32::MAX, i32::MAX), (0, 0));
        for (i, cell) in c.cells().iter().enumerate() {
            let (x, row) = (i as i32 % w as i32, i as i32 / w as i32);
            if cell.ch == ' ' {
                continue;
            }
            for (j, hex) in [0xbfa98a, 0x4f8f7a, 0xf2c14e, 0xffcc4d]
                .into_iter()
                .enumerate()
            {
                seen[j] |= cell.fg == rgb(hex) || cell.bg == rgb(hex);
            }
            lo = (lo.0.min(x), lo.1.min(row * 2));
            hi = (hi.0.max(x), hi.1.max(row * 2 + 1));
        }
        assert!(seen.iter().all(|&s| s), "{seen:?}");
        assert!(hi.0 - lo.0 <= BASILICA.0 as i32 + 2, "{lo:?} {hi:?}");
        assert!(hi.1 - lo.1 <= BASILICA.1 as i32 + 2, "{lo:?} {hi:?}");
        assert!(hi.1 - lo.1 >= BASILICA.1 as i32 - 4, "{lo:?} {hi:?}");
    }

    /// The image of the Virgen is drawn the same way round whichever way the
    /// pilgrims who carry it are going.
    #[test]
    fn the_image_is_never_mirrored() {
        let pal = Palette::guadalupe(&Theme::classic());
        let paint = |s: f32, ox: f32| {
            let mut c = Canvas::new(40, 20, Color::Reset);
            let at = Place {
                pal: &pal,
                k: 1.0,
                s,
                ox,
                ground: 40.0,
            };
            at.virgen(&mut c, (0.0, 30.0), 1.0);
            c
        };
        // Facing right from column 4, or facing left from the far end of the
        // same 17 pixels: the same picture.
        let (a, b) = (paint(1.0, 4.0), paint(-1.0, 4.0 + 17.0));
        assert!(a.cells().iter().any(|cell| cell.ch != ' '));
        assert!(a.cells().iter().zip(b.cells()).all(|(x, y)| x == y));
    }

    /// Whatever the screen, the shared drawing never panics.
    #[test]
    fn drawing_survives_odd_sizes() {
        let pal = Palette::guadalupe(&Theme::classic());
        for (w, h) in [(1, 1), (3, 40), (40, 3), (500, 150)] {
            let mut c = Canvas::new(w, h, Color::Reset);
            for k in [0.25, 1.0, 6.0] {
                for s in [1.0, -1.0] {
                    let at = Place {
                        pal: &pal,
                        k,
                        s,
                        ox: w as f32 / 2.0,
                        ground: h as f32 * 2.0,
                    };
                    at.basilica(&mut c, 0.0, 0.5);
                    at.framed(&mut c, (3.0, 40.0), 1.0);
                    at.virgen(&mut c, (-9.0, 40.0), 0.5);
                    at.veladora(&mut c, (2.0, 0.0), b'V', 1.0);
                    at.disc(&mut c, (1.0, 1.0), 4.0, pal.of(b'R'));
                    at.sprite(&mut c, RAMO, (0.0, 30.0), (0.3, 1.5), |l| pal.get(l));
                }
            }
        }
    }

    /// On the twelfth of December the visitors of the Navidad that already
    /// came keep coming and the Virgen's join them; the day after, only the
    /// others come.
    #[test]
    fn they_join_the_navidad_visitors_on_the_twelfth() {
        let names = |day: Fiesta| -> Vec<&str> {
            ALL.iter()
                .filter(|i| i.fiesta == Kind::Navidad && (i.when)(day))
                .map(|i| i.name)
                .collect()
        };
        let (twelfth, thirteenth) = (names(Fiesta::of_day(12, 12)), names(Fiesta::of_day(12, 13)));
        for old in ["posada", "pinata", "pastorela"] {
            assert!(twelfth.contains(&old) && thirteenth.contains(&old), "{old}");
        }
        for new in ["peregrinos", "mananitas", "matachines"] {
            assert!(
                twelfth.contains(&new) && !thirteenth.contains(&new),
                "{new}"
            );
        }
    }

    /// The Virgen de Guadalupe visitors come on the twelfth of December, in
    /// any year, and never the day before, the day after, on an ordinary day
    /// or on another holiday's.
    #[test]
    fn the_visitors_keep_to_guadalupe() {
        for name in ["peregrinos", "mananitas", "matachines"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Navidad, "{name}");
            let comes = |fiesta: Fiesta| {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                e.active()
            };
            for (y, m, d) in [(2026, 12, 12), (2027, 12, 12), (2028, 12, 12)] {
                let fiesta = Fiesta::of_date(y, m, d);
                assert!(fiesta.guadalupe(), "{y}-{m}-{d}");
                assert!(comes(fiesta), "{name} on {y}-{m}-{d}");
            }
            assert!(!comes(Fiesta::NONE), "{name} came on an ordinary day");
            for (y, m, d) in [
                (2026, 12, 11),
                (2026, 12, 13),
                (2027, 12, 11),
                (2027, 12, 13),
                (2027, 12, 16),
                (2027, 12, 24),
                (2027, 12, 28),
                (2027, 1, 1),
                (2027, 1, 6),
                (2027, 11, 2),
                (2027, 9, 15),
                (2027, 5, 27),
                (2027, 7, 26),
                (2027, 6, 15),
            ] {
                assert!(
                    !comes(Fiesta::of_date(y, m, d)),
                    "{name} came on {y}-{m}-{d}"
                );
            }
        }
    }
}
