//! Trajineras, the flat painted boats of Xochimilco, and the buoys that
//! carry a star or a flag out on the water.
//!
//! A trajinera is a long low hull with raised ends, painted in bold
//! colors, and a tall arch (the portada) standing on it with a woman's
//! name across it: Lupita, Esperanza, Josefa. Here each is drawn side-on
//! in pixels, its arch in the middle, a pole at either end with a string of
//! decorations hung between the poles and the arch, and in a sea tall
//! enough the name in a 3x5 font. How a boat looks is one function that
//! plots pixels through a closure, so the very same drawing paints the boat
//! and, turned upside down, its reflection and the glow of its lights.

use super::gala::Paint;
use super::navidad;
use super::tank::mul;
use crate::color::Color;
use crate::fiesta::Kind;

/// The letters of the names, A to Z, 3 pixels wide and 5 tall: a byte a row
/// with the left pixel in bit 2.
const FONT: [[u8; 5]; 26] = [
    [2, 5, 7, 5, 5], // A
    [6, 5, 6, 5, 6], // B
    [3, 4, 4, 4, 3], // C
    [6, 5, 5, 5, 6], // D
    [7, 4, 6, 4, 7], // E
    [7, 4, 6, 4, 4], // F
    [3, 4, 5, 5, 3], // G
    [5, 5, 7, 5, 5], // H
    [7, 2, 2, 2, 7], // I
    [1, 1, 1, 5, 2], // J
    [5, 5, 6, 5, 5], // K
    [4, 4, 4, 4, 7], // L
    [5, 7, 7, 5, 5], // M
    [7, 5, 5, 5, 5], // N
    [7, 5, 5, 5, 7], // O
    [6, 5, 6, 4, 4], // P
    [2, 5, 5, 6, 3], // Q
    [6, 5, 6, 5, 5], // R
    [3, 4, 2, 1, 6], // S
    [7, 2, 2, 2, 2], // T
    [5, 5, 5, 5, 7], // U
    [5, 5, 5, 5, 2], // V
    [5, 5, 7, 7, 5], // W
    [5, 5, 2, 5, 5], // X
    [5, 5, 2, 2, 2], // Y
    [7, 1, 2, 4, 7], // Z
];

/// A letter's rows; a blank for anything but A to Z.
pub fn glyph(c: u8) -> [u8; 5] {
    match c {
        b'A'..=b'Z' => FONT[(c - b'A') as usize],
        _ => [0; 5],
    }
}

/// The width of `name` in pixels, a column between the letters.
pub fn text_width(name: &str) -> i32 {
    (name.len() as i32 * 4 - 1).max(0)
}

/// How big a boat is drawn, set by how much sky the sea has over the water.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Shape {
    /// Rows of hull and of arch.
    pub hull: i32,
    pub panel: i32,
    /// Whether the arch is tall enough to carry the name.
    pub named: bool,
    /// Hull beyond the arch at each end, for the poles and what hangs there.
    pub gap: i32,
    /// A piñata's size in pixels; 0 when there is no room for one.
    pub pinata: i32,
}

impl Shape {
    /// The boats a sea with `surf` rows of sky and `w` columns can hold;
    /// none when the sky is too thin.
    pub fn of(surf: i32, w: i32) -> Option<Shape> {
        if surf >= 12 && w >= 80 {
            Some(Shape {
                hull: 3,
                panel: 7,
                named: true,
                gap: 8,
                pinata: 5,
            })
        } else if surf >= 8 {
            Some(Shape {
                hull: 2,
                panel: 4,
                named: false,
                gap: 6,
                pinata: 3,
            })
        } else if surf >= 6 {
            Some(Shape {
                hull: 2,
                panel: 3,
                named: false,
                gap: 3,
                pinata: 0,
            })
        } else {
            None
        }
    }

    /// The arch's width for a boat called `name`.
    pub fn arch(&self, name: &str) -> i32 {
        if self.named {
            text_width(name) + 4
        } else {
            9 + 2 * self.panel
        }
    }

    /// A boat's width, hull end to hull end.
    pub fn width(&self, name: &str) -> i32 {
        self.arch(name) + 2 * self.gap
    }

    /// Rows from the hull's bottom to the top of the arch.
    pub fn rows(&self) -> i32 {
        self.hull + self.panel
    }
}

/// How far past the ends of its hull a boat's decorations reach, in pixels:
/// the arm with the piñata, the trajinero's pole.
pub const REACH: i32 = 9;

/// A trajinera on its way along the water.
#[derive(Clone, Copy)]
pub struct Boat {
    /// The middle of the hull and how fast it drifts, pixels and pixels per
    /// tick; `dir` is the way it heads, 1 for right.
    pub x: f32,
    pub vx: f32,
    pub dir: f32,
    pub name: &'static str,
    /// Hull width in pixels.
    pub w: i32,
    /// Which paints the arch, hull and trim wear (`Paint::boats`).
    pub panel: usize,
    pub hull: usize,
    pub trim: usize,
    pub phase: f32,
    /// Whether a piñata hangs at its bow.
    pub pinata: bool,
    /// Which way its lights blink.
    pub blink: u8,
}

/// How bright bulb `i` of a string shines at tick `t`, 0.3 (dim) to 1.
pub fn blink(pattern: u8, i: i32, t: f32) -> f32 {
    let on = match pattern % 3 {
        // A chase: one in three is dark, and the dark one runs along.
        0 => (t / 4.0) as i32 + i,
        // Odd and even take turns.
        1 => (t / 14.0) as i32 + i,
        // Steady, with a twinkle now and then.
        _ => i + (super::sea::hash(i, (t / 5.0) as i32, 0x626f_6174) % 4) as i32,
    };
    let dark = match pattern % 3 {
        0 => on.rem_euclid(3) == 0,
        1 => on.rem_euclid(2) == 0,
        _ => on == i + 3,
    };
    if dark { 0.3 } else { 1.0 }
}

/// What a boat's lights are like for the day: how far apart the bulbs are
/// along the arch and whether its sides are outlined with them too.
#[derive(Clone, Copy)]
pub struct Lights {
    pub every: i32,
    pub outline: bool,
}

impl Lights {
    pub fn of(level: f32) -> Lights {
        Lights {
            every: if level > 0.7 {
                2
            } else if level > 0.45 {
                3
            } else {
                4
            },
            outline: level > 0.55,
        }
    }
}

/// A piñata de siete picos: a ball with seven cones, each digit a cone
/// (a papel de china color) and `B` the ball.
const PINATA: [&str; 5] = ["..0..", "1.B.2", ".BBB.", "3.B.4", ".5.6."];
const PINATA_SMALL: [&str; 3] = [".0.", "1B2", ".3."];

/// Plots `boat` through `put(x, up, color, glow)`: `x` runs along the boat
/// from its left end, `up` counts rows over the bottom of the hull (below
/// zero is under the water) and `glow` is how brightly a bulb shines, 0 for
/// anything that is only painted.
pub fn plot(
    b: &Boat,
    sh: &Shape,
    p: &Paint,
    lights: Lights,
    t: f32,
    put: &mut impl FnMut(i32, i32, Color, f32),
) {
    let w = b.w;
    let (hull_c, trim_c, panel_c) = (p.boats[b.hull], p.boats[b.trim], p.boats[b.panel]);
    let x0 = sh.gap;
    let aw = w - 2 * sh.gap;
    let top = sh.rows() - 1;
    let bow_right = b.dir >= 0.0;
    let bulb_at = |put: &mut dyn FnMut(i32, i32, Color, f32), x: i32, u: i32, i: i32| {
        let c = p.bulbs[(i.rem_euclid(6) as usize + b.blink as usize) % 6];
        let k = blink(b.blink, i, t);
        put(
            x,
            u,
            mul(c, 0.35 + 0.65 * k),
            if k > 0.9 { 1.0 } else { 0.0 },
        );
    };

    // A string hung from `xa` to `xb` at the height of the arch, sagging
    // `sag` in the middle, a bulb every third pixel.
    let sag = if sh.gap >= 8 {
        2
    } else if sh.gap >= 5 {
        1
    } else {
        0
    };
    for (xa, xb) in [(1, x0 + 1), (x0 + aw - 2, w - 2)] {
        let len = (xb - xa).max(1) as f32;
        for x in xa..=xb {
            let s = (x - xa) as f32 / len;
            let u = top - (sag as f32 * 4.0 * s * (1.0 - s)).round() as i32;
            if (x - xa) % 3 == 1 && sh.gap >= 5 {
                bulb_at(put, x, u, x + xa);
            } else {
                put(x, u, p.cord, 0.0);
            }
        }
    }
    // The poles the strings hang from.
    for u in sh.hull..=top {
        put(1, u, p.wood, 0.0);
        put(w - 2, u, p.wood, 0.0);
    }
    // At the bow an arm reaches out over the water, a piñata hanging from
    // its end and swinging a little.
    if b.pinata && sh.pinata > 0 {
        let (out, pole) = if bow_right { (1, w - 2) } else { (-1, 1) };
        let reach = 1 + sh.pinata / 2;
        for i in 1..=reach {
            put(pole + out * i, top, p.wood, 0.0);
        }
        let cx = pole + out * reach;
        put(cx, top - 1, p.cord, 0.0);
        let sway = match ((t * 0.07 + b.phase).sin() * 1.6).round() as i32 {
            i32::MIN..=-1 => -1,
            0 => 0,
            _ => 1,
        };
        let art: &[&str] = if sh.pinata >= 5 {
            &PINATA
        } else {
            &PINATA_SMALL
        };
        let half = art.len() as i32 / 2;
        for (r, row) in art.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                let color = match ch {
                    b'.' => continue,
                    b'B' => p.boats[(b.panel + 3) % 6],
                    d => p.boats[((d - b'0') as usize + b.phase as usize) % 6],
                };
                put(cx + sway + c as i32 - half, top - 2 - r as i32, color, 0.0);
            }
        }
    }

    // The hull: a bottom row tucked in, the painted side with a border of
    // small diamonds, the gunwale, and the ends raised.
    for x in 0..w {
        if x >= 2 && x < w - 2 {
            put(x, 0, mul(hull_c, 0.7), 0.0);
        }
        for u in 1..sh.hull {
            let gunwale = u == sh.hull - 1 && sh.hull >= 3;
            let c = if gunwale || (x % 4 == 2) {
                trim_c
            } else {
                hull_c
            };
            put(x, u, c, 0.0);
        }
    }
    for x in [0, w - 1] {
        put(x, sh.hull, trim_c, 0.0);
    }

    // The arch: trim all round, a crest on top with its corners cut, the
    // name or a painted pattern inside.
    for x in x0..x0 + aw {
        put(x, sh.hull, trim_c, 0.0);
    }
    for u in sh.hull + 1..top {
        for x in x0..x0 + aw {
            let edge = x == x0 || x == x0 + aw - 1;
            let c = if edge || (!sh.named && (x + u) % 3 == 0) {
                trim_c
            } else {
                panel_c
            };
            put(x, u, c, 0.0);
        }
    }
    for x in x0 + 1..x0 + aw - 1 {
        put(x, top, trim_c, 0.0);
    }
    if sh.named {
        let ink = if panel_c.luma() < 0.5 {
            p.ink[0]
        } else {
            p.ink[1]
        };
        let mut x = x0 + 2;
        for ch in b.name.bytes() {
            for (r, row) in glyph(ch).iter().enumerate() {
                for c in 0..3 {
                    if row >> (2 - c) & 1 == 1 {
                        put(x + c, top - 1 - r as i32, ink, 0.0);
                    }
                }
            }
            x += 4;
        }
    }

    // The lights along the crest, and down the sides.
    for x in x0 + 1..x0 + aw - 1 {
        if (x - x0) % lights.every == 1 {
            bulb_at(put, x, top, x);
        }
    }
    if lights.outline {
        for u in sh.hull + 1..top {
            if (u - sh.hull) % 2 == 1 {
                bulb_at(put, x0, u, u + 3);
                bulb_at(put, x0 + aw - 1, u, u + 5);
            }
        }
    }

    // The trajinero at the stern, poling the boat along.
    if sh.named {
        let sx = if bow_right {
            sh.gap / 2 + 1
        } else {
            w - 1 - sh.gap / 2 - 1
        };
        let out = if bow_right { -1 } else { 1 };
        let h = sh.hull;
        for (dx, u, c) in [
            (0, h + 4, p.straw),
            (-1, h + 3, p.straw),
            (0, h + 3, p.straw),
            (1, h + 3, p.straw),
            (0, h + 2, p.wood),
            (-1, h + 1, p.ink[0]),
            (0, h + 1, p.ink[0]),
            (1, h + 1, p.ink[0]),
            (0, h, p.ink[1]),
        ] {
            put(sx + dx, u, c, 0.0);
        }
        // His pole, down and out into the water.
        for i in 0..h + 4 {
            put(sx + out * (2 + i), h + 1 - i, p.wood, 0.0);
        }
    }
}

/// A buoy out on the water, with a star or a flag on its mast.
#[derive(Clone, Copy)]
pub struct Buoy {
    pub x: i32,
    pub phase: f32,
}

/// Plots a buoy, centered on column 0 of `put(dx, up, color, glow)`: a can
/// in three bands, a mast of `mast` rows and the holiday's topper of `size`
/// pixels on top.
pub fn plot_buoy(
    p: &Paint,
    kind: Kind,
    mast: i32,
    size: i32,
    t: f32,
    put: &mut dyn FnMut(i32, i32, Color, f32),
) {
    let bands = [p.rojo, p.blanco, p.verde];
    for dx in -3..=3 {
        put(dx, 0, mul(bands[0], 0.75), 0.0);
        put(dx, 1, bands[0], 0.0);
    }
    for dx in -2..=2 {
        put(dx, 2, bands[1], 0.0);
    }
    for dx in -1..=1 {
        put(dx, 3, bands[2], 0.0);
    }
    for u in 4..4 + mast {
        put(0, u, p.wood, 0.0);
    }
    let foot = 4 + mast;
    if kind == Kind::Navidad {
        navidad::star(p, size, t, &mut |dx, u, c, g| put(dx, foot + u, c, g));
    }
}
