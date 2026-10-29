//! The spring equinox in the jungle.
//!
//! On March 20 and 21 the sun, going down in the afternoon, makes the serpent
//! of light at Chichén Itzá: the corners of the nine terraces of El Castillo
//! throw their shadows on the balustrade of the northern stairway, seven
//! triangles of light that slither down it to the stone head of Kukulcán at its
//! foot. Here the ruins by the path become El Castillo, stone steps and
//! terraces and its temple on top, and the triangles come down the stairway
//! as the afternoon goes, one after another, the further the higher the
//! holiday, until the eyes of the stone serpent light up. The tiger comes to
//! sit before it and watch. [`Dress`] holds the colors, from the brief's own
//! under `classic` and from the theme's accents otherwise. Everything here is
//! hashed from the seed, like the jungle it dresses.

use super::art::Sprite;
use super::fiestas::by_hue;
use super::jungle::{Jungle, View};
use super::muertos::RUIN_FOOT;
use super::{Rgb, hash, rgb_of};
use crate::scene::Ctx;

// ----- the colors of the holiday -----

const CALIZA: Rgb = Rgb(201, 180, 138);
const SOMBRA: Rgb = Rgb(59, 50, 38);
const SOL: Rgb = Rgb(255, 207, 74);
const JADE: Rgb = Rgb(46, 139, 87);
const CIELO: Rgb = Rgb(126, 200, 227);

/// The holiday's colors, and how far it has come.
#[derive(Clone, Copy, Debug)]
pub struct Dress {
    /// 0 to 1, as in `Fiesta::level`.
    pub level: f32,
    /// The limestone, the shade of its shadows, the sun's gold, the jade of
    /// the quetzal's feathers and the sky.
    pub caliza: Rgb,
    pub sombra: Rgb,
    pub sol: Rgb,
    pub jade: Rgb,
    pub cielo: Rgb,
}

impl Dress {
    /// The brief's own colors.
    fn classic(level: f32) -> Dress {
        Dress {
            level,
            caliza: CALIZA,
            sombra: SOMBRA,
            sol: SOL,
            jade: JADE,
            cielo: CIELO,
        }
    }

    /// The dress for today: the holiday's colors, or the theme's own where
    /// it has something close.
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
        let sol = by_hue(&cols, 45.0, 25.0, &[]).unwrap_or_else(|| pick(3));
        let jade = by_hue(&cols, 140.0, 45.0, &[sol]).unwrap_or_else(|| pick(2));
        let cielo = by_hue(&cols, 200.0, 40.0, &[sol, jade]).unwrap_or_else(|| pick(4));
        let fg = rgb_of(t.fg).unwrap_or(CALIZA);
        let caliza = fg.lerp(sol, 0.3).scale(0.85);
        Dress {
            level,
            caliza,
            sombra: caliza.scale(0.22),
            sol,
            jade,
            cielo,
        }
    }

    /// The stone serpent's colors by letter: limestone in light and shade,
    /// the dark of its mouth, its fangs, the jade of its feathers, and its eye.
    pub fn serpiente(&self) -> [(u8, Rgb); 6] {
        [
            (b'H', self.caliza.lerp(Rgb::WHITE, 0.2)),
            (b'h', self.caliza.scale(0.58)),
            (b'K', self.sombra),
            (b'W', self.caliza.lerp(Rgb::WHITE, 0.7)),
            (b'J', self.jade),
            (b'E', self.sombra),
        ]
    }
}

// ----- the drawings -----

/// The stone head of Kukulcán at the foot of the balustrade, facing out: feathers of jade over
/// its brow, an eye, the mouth open on its fangs. `H` `h` limestone in light and shade, `K` the
/// dark of its mouth, `W` a fang, `J` jade, `E` the eye, which the sun lights at the height of the
/// holiday.
pub const CABEZA: Sprite = &[
    "..J.J.......",
    ".JJJJJ......",
    "JJHHHHHHH...",
    ".HHHHEHHHHH.",
    ".HHHHHHHHHHH",
    ".HhKKKKKKKWK",
    ".HhKWKWKKK..",
    ".hhhhhhhhhh.",
    "..hhhhhhhh..",
];

// ----- El Castillo -----

/// How tall a terrace is and how many there are, the half width of the base and how much each
/// terrace gives up on either side.
const TERRACE_H: i32 = 4;
const TERRACES: i32 = 9;
const BASE_HALF: f32 = 50.0;
const INSET: f32 = 3.2;
/// The stairway's half width, the width of its balustrades, and where the temple stands: its half
/// width and height.
const STAIR_HALF: i32 = 8;
const RAIL: i32 = 4;
const TEMPLE: (i32, i32) = (13, 11);
/// The triangles of light of the serpent, and how many rows each takes.
const TRIANGLES: i32 = 7;
const TRIANGLE_H: i32 = 5;
/// The afternoon the serpent comes down in, as the fractions of the day: it begins to form, the
/// last triangle reaches the head, it begins to fade, and the sun has set.
const AFTERNOON: [f32; 4] = [0.55, 0.68, 0.76, 0.82];
/// The jungle's cells hold these half-width castles; the tiger sits this far from the middle.
pub const CASTILLO_HALF: f32 = 52.0;
pub const SIT_AT: f32 = 62.0;

/// 0 to 1 and back: a smooth step.
fn smooth(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

impl Jungle {
    /// How far Kukulcán has come down the stairway, 0 to 1: it forms as the afternoon sun goes
    /// down, holds, and fades as the sun sets. The higher the holiday, the further it gets.
    pub fn serpent(&self, d: &Dress) -> f32 {
        let t = self.t;
        let up = smooth((t - AFTERNOON[0]) / (AFTERNOON[1] - AFTERNOON[0]));
        let down = 1.0 - smooth((t - AFTERNOON[2]) / (AFTERNOON[3] - AFTERNOON[2]));
        up * down * (0.4 + 0.6 * d.level)
    }

    /// Which side the sun lights, -1 on the left in the morning to 1 on the right in the
    /// afternoon, and 0 at night.
    fn sun_side(&self) -> f32 {
        if (0.19..0.81).contains(&self.t) {
            (((self.t - 0.19) / 0.62) * 2.0 - 1.0).clamp(-1.0, 1.0)
        } else {
            0.0
        }
    }

    /// How soon the tiger goes to sit before El Castillo, in seconds: the more eagerly the
    /// further the serpent has come down, so that it is there for the afternoon. 0 on any other
    /// day.
    pub fn eagerness(&self) -> f32 {
        match self.pal.equinoccio {
            Some(d) if self.serpent(&d) > 0.25 => 45.0,
            _ => 0.0,
        }
    }

    /// El Castillo in each ruin's place: nine terraces and the temple on top, the stairway up the
    /// north face with its balustrades, and the stone serpents at their feet. In the afternoon
    /// the light comes down the right balustrade as seven triangles.
    pub fn paint_castillos(&mut self) {
        let Some(d) = self.pal.equinoccio else { return };
        let ruins = self.ruins(self.bx as f32, (self.bx + self.bw) as f32);
        for r in ruins.as_slice() {
            self.castillo(r.x.round() as i32, &d);
        }
    }

    /// One Castillo, its middle on column `x0` and its feet on the ruin's foot.
    fn castillo(&mut self, x0: i32, d: &Dress) {
        let foot = RUIN_FOOT;
        let side = self.sun_side();
        let stone = self.lit(d.caliza);
        let (lo, hi) = (self.bx, self.bx + self.bw);
        let seed = self.seed;
        // The terraces, from the ground up: a wall and the ledge on its top, narrower each time,
        // the sun's side lighter. The stairway cuts through all of them.
        for i in 0..TERRACES {
            let half = (BASE_HALF - INSET * i as f32).round() as i32;
            let bottom = foot - TERRACE_H * i;
            for dy in 0..TERRACE_H {
                let y = bottom - dy;
                // The top row is the ledge, in the light; the rest the wall.
                let ledge = dy == TERRACE_H - 1;
                for dx in -half..=half {
                    let x = x0 + dx;
                    if x < lo - 1 || x > hi {
                        continue;
                    }
                    let face = dx as f32 / half as f32;
                    let k = if ledge {
                        1.0
                    } else {
                        0.74 + 0.2 * (0.5 + 0.5 * side * face)
                    };
                    // Blocks: a joint every few pixels, an odd stone darker.
                    let h = hash(x, y, seed ^ 0xca57);
                    let joint = (x + (y & 1) * 3).rem_euclid(7) == 0;
                    let k = k * if joint {
                        0.9
                    } else if h.is_multiple_of(11) {
                        0.93
                    } else {
                        1.0
                    };
                    self.put(x, y, stone.scale(k));
                }
            }
        }
        let top = foot - TERRACE_H * TERRACES + 1;
        // The temple on its platform, its dark doorway, a frieze and the roof comb.
        let (th, tt) = (TEMPLE.0, TEMPLE.1);
        for dy in 0..tt {
            let y = top - 1 - dy;
            for dx in -th..=th {
                let x = x0 + dx;
                let door = dx.abs() <= 4 && dy < 8;
                let k = 0.82 + 0.16 * (0.5 + 0.5 * side * dx as f32 / th as f32);
                let c = if door {
                    self.lit(d.sombra)
                } else if dy == tt - 2 && (dx + 100) % 3 == 0 {
                    // The frieze under the roof, studded with jade.
                    self.lit(d.jade)
                } else {
                    stone.scale(k)
                };
                self.put(x, y, c);
            }
        }
        for dy in 0..3 {
            let y = top - tt - dy;
            for dx in -(th - 3)..=(th - 3) {
                if dy == 2 && (dx + 100) % 3 != 0 {
                    continue;
                }
                self.put(x0 + dx, y, stone.scale(0.9));
            }
        }
        // The north stairway: steps up all the nine terraces, a shadow along each side of it, and
        // the balustrades beside it, the right one for the serpent.
        let shadow = self.lit(d.sombra);
        for y in top..foot {
            let step = if (foot - y) % 2 == 0 { 1.0 } else { 0.8 };
            for dx in -STAIR_HALF..=STAIR_HALF {
                self.put(
                    x0 + dx,
                    y,
                    stone.scale(step * (0.9 + 0.1 * side * dx as f32 / 8.0)),
                );
            }
            for s in [-1, 1] {
                self.put(x0 + s * (STAIR_HALF + 1), y, shadow);
            }
            for k in 1..=RAIL {
                self.put(x0 - (STAIR_HALF + 1) - k, y, stone.scale(0.9));
                self.put(x0 + (STAIR_HALF + 1) + k, y, stone.scale(0.62));
            }
        }
        // The serpent of light: triangles on the right balustrade, one for each corner of the
        // terraces, apex down, each leaning the other way from the one above it.
        let amount = self.serpent(d);
        let lit = amount * TRIANGLES as f32;
        let sun = self.lit(d.sol).lerp(d.sol, 0.5);
        let (strip, y_top) = (x0 + STAIR_HALF + 2, top);
        let mut shown = 0;
        for i in 0..TRIANGLES {
            let part = (lit - i as f32).clamp(0.0, 1.0);
            if part <= 0.0 {
                continue;
            }
            // The triangle grows from its top as the light reaches it.
            let rows = (TRIANGLE_H as f32 * part).ceil() as i32;
            for k in 0..rows {
                let w = RAIL - k.min(RAIL - 1);
                let from = if i % 2 == 0 { 0 } else { RAIL - w };
                for j in 0..w {
                    let (x, y) = (strip + from + j, y_top + i * TRIANGLE_H + k);
                    self.put(x, y, sun);
                    self.blend(x + 1, y, sun, 0.25);
                }
            }
            shown += 1;
        }
        #[cfg(test)]
        {
            self.painted.castillos += 1;
            self.painted.triangles = self.painted.triangles.max(shown);
        }
        let _ = shown;
        // Kukulcán's heads at the feet of both balustrades; their eyes light when the serpent
        // arrives.
        let mut lut = [None; 128];
        for &(ch, c) in &d.serpiente() {
            lut[ch as usize] = Some(self.lit(c));
        }
        if amount >= 0.97 {
            lut[b'E' as usize] = Some(sun);
        }
        let half = (CABEZA[0].len() / 2) as i32;
        let reach = STAIR_HALF + 2 + RAIL + half;
        self.sprite(CABEZA, (x0 + reach) as f32, foot as f32, 1.0, &lut);
        self.sprite(CABEZA, (x0 - reach) as f32, foot as f32, -1.0, &lut);
    }

    /// The warm light of the serpent at the head of the stairway, over everything near it: the
    /// sun's gold, as strong as the serpent is.
    pub fn glow_castillos(&mut self, v: View) {
        let Some(d) = self.pal.equinoccio else { return };
        let amount = self.serpent(&d);
        if amount <= 0.05 {
            return;
        }
        let ruins = self.ruins(v.x, v.x + v.w);
        for r in ruins.as_slice() {
            let (cx, cy) = v.to_screen(r.x + 14.0, (RUIN_FOOT - 5) as f32);
            let rad = ((16.0 * v.z) as i32).clamp(3, 60);
            for dy in -rad..=rad {
                for dx in -rad..=rad {
                    let q = ((dx * dx + dy * dy) as f32).sqrt() / rad as f32;
                    if q < 1.0 {
                        self.blend_out(
                            cx + dx,
                            cy + dy,
                            d.sol,
                            0.3 * amount * (1.0 - q) * (1.0 - q),
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The drawing is a rectangle, and every letter in it has a color.
    #[test]
    fn the_drawing_fits_together() {
        let dress = Dress::classic(1.0);
        let pal: Vec<(u8, Rgb)> = dress.serpiente().into();
        let w = CABEZA[0].len();
        for row in CABEZA {
            assert_eq!(row.len(), w, "{row}");
            for ch in row.bytes().filter(|&c| c != b'.') {
                assert!(pal.iter().any(|&(k, _)| k == ch), "{}", ch as char);
            }
        }
        // The serpent's triangles fit down the stairway, and its strip is as wide as a rail.
        const {
            assert!(TRIANGLES * TRIANGLE_H <= TERRACE_H * TERRACES);
            assert!(TRIANGLE_H >= RAIL);
            // The Castillo fits in its cell: it is no wider than the half-width the placement
            // keeps clear.
            assert!(BASE_HALF as i32 <= CASTILLO_HALF as i32);
        }
    }
}
