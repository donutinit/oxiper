//! Fiestas Patrias on the orrery (see the module doc in `orrery.rs`).
//!
//! Like the other holidays, it only dresses what the orrery already has, and
//! draws no random numbers: the stone's texture, the confetti and the waving of
//! the flags come from a clock and a hash. The sun is laid out like the
//! sun-calavera's face (the same four sizes, centered on a cell) and carved
//! once, when the layout is made, into the cells that draw it.

use std::f32::consts::TAU;

use super::{FACES, QUAD, RayDot, hash01, nearest, warmest};
use crate::color::{Color, Gradient, mix_oklab, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Braille, Canvas};
use crate::scene::Ctx;

/// Verde bandera, blanco, rojo, and the oro of the stone's carved gold.
pub(super) const VERDE: u32 = 0x006847;
pub(super) const BLANCO: u32 = 0xf4f1ea;
pub(super) const ROJO: u32 = 0xce1126;
pub(super) const ORO: u32 = 0xd4a017;
/// Verde and rojo lightened to show on a dark sky, for what is small or thin.
pub(super) const VERDE_CLARO: u32 = 0x1e9e62;
pub(super) const ROJO_CLARO: u32 = 0xe8404f;

/// The Grito is shouted for `SHOUT_FOR` seconds in every `SHOUT_EVERY`, from the
/// second the holiday starts, on the days that climb to it.
const SHOUT_EVERY: f32 = 14.0;
const SHOUT_FOR: f32 = 3.5;
/// A campanada ring lives this long, and they come this often at the height of
/// the holiday (less often below it).
const RING_LIFE: f32 = 7.0;
const RING_EVERY: f32 = 4.0;
/// Level from which the rings of the bell go out from the stone.
const GRITO: f32 = 0.6;

/// What a pixel of the stone is made of.
#[derive(Clone, Copy, PartialEq)]
enum Mat {
    /// Carved away, or sky.
    Air,
    Rock,
    Gold,
}

/// A cell of the stone: where, which block it is, and what it shows.
#[derive(Clone, Copy)]
pub(super) struct StoneCell {
    pub(super) dx: i32,
    pub(super) dy: i32,
    ch: char,
    gold: bool,
    /// How far from the center, in radii, and at which angle.
    rho: f32,
    ang: f32,
    /// A bit of grain for the rock, 0 to 1.
    grain: f32,
}

pub(super) struct Patrias {
    pub(super) on: bool,
    pub(super) level: f32,
    /// Seconds of holiday: the glint sweeping round the stone, the rays, the flags.
    t: f32,
    /// The face's size the sun takes (an index into `FACES`), or a glyph when tiny.
    pub(super) face: Option<usize>,
    /// The carved cells of the stone, from the layout.
    pub(super) stone: Vec<StoneCell>,
    /// Length of the long rays in units, and the dots they can fall on.
    ray: f32,
    ring: Vec<RayDot>,
    /// Verde, blanco, rojo; and the same for what is small.
    flag: [Color; 3],
    lit: [Color; 3],
    oro: Gradient,
    piedra: Gradient,
    pole: Color,
}

impl Patrias {
    pub(super) fn new() -> Patrias {
        Patrias {
            on: false,
            level: 0.0,
            t: 0.0,
            face: None,
            stone: Vec::new(),
            ray: 1.0,
            ring: Vec::new(),
            flag: [Color::Reset; 3],
            lit: [Color::Reset; 3],
            oro: Gradient::new(&[]),
            piedra: Gradient::new(&[]),
            pole: Color::Reset,
        }
    }

    pub(super) fn set(&mut self, fiesta: Fiesta) {
        self.on = fiesta.is(crate::fiesta::Kind::Patrias);
        self.level = if self.on {
            fiesta.level.clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    pub(super) fn retheme(&mut self, ctx: &Ctx) {
        let native = ctx.theme.native;
        let g = ctx.ramp(&[]);
        // Under other themes each color is the theme's nearest accent.
        let tint = |hex: u32| {
            if native {
                rgb(hex)
            } else {
                nearest(&ctx.theme.colors, hex).unwrap_or(g.at(0.8))
            }
        };
        let gold = if native {
            rgb(ORO)
        } else {
            warmest(&ctx.theme.colors).unwrap_or(g.at(0.8))
        };
        let white = if native { rgb(BLANCO) } else { g.at(1.0) };
        self.flag = [tint(VERDE), white, tint(ROJO)];
        self.lit = [
            if native {
                rgb(VERDE_CLARO)
            } else {
                self.flag[0]
            },
            white,
            if native {
                rgb(ROJO_CLARO)
            } else {
                self.flag[2]
            },
        ];
        self.oro = Gradient::new(&if native {
            [0x7a5a0c, ORO, 0xf0cf63, 0xfff1b8].map(rgb)
        } else {
            [
                mix_oklab(gold, g.at(0.3), 0.55),
                gold,
                mix_oklab(gold, g.at(1.0), 0.45),
                g.at(1.0),
            ]
        });
        self.piedra = Gradient::new(&if native {
            [0x3b362d, 0x6a6356, 0x978d7b, 0xc0b6a0].map(rgb)
        } else {
            [
                ctx.fade(g.at(0.5), 0.45),
                g.at(0.5),
                mix_oklab(g.at(0.5), g.at(1.0), 0.45),
                mix_oklab(g.at(0.5), g.at(1.0), 0.8),
            ]
        });
        self.pole = self.oro.at(0.5);
    }

    /// Moves the holiday's clock along.
    pub(super) fn step(&mut self, flow: f32) {
        self.t = (self.t + flow).rem_euclid(1000.0 * TAU);
    }

    /// The color of the `i`th planet from the sun: verde, blanco, rojo in turn.
    pub(super) fn planet(&self, i: usize) -> Color {
        self.lit[i % 3]
    }

    /// The color of the `i`th rock of the belt when it is a scrap of tricolor
    /// papel: more of them as the month goes by. `None` when it stays a rock.
    pub(super) fn belt(&self, i: usize) -> Option<Color> {
        let share = 0.15 + 0.55 * self.level;
        let p = hash01(i as u32);
        (p < share).then(|| self.lit[((p / share * 3.0) as usize).min(2)])
    }

    /// How far the rays reach from the sun's center, in units.
    pub(super) fn reach(&self, r: f32) -> f32 {
        r + 0.3 + self.ray * (0.8 + 0.2 * self.level)
    }

    /// Carves the stone and finds the dots of the ring around it where the
    /// rays can fall, once the layout is known, so that drawing takes no
    /// trigonometry.
    pub(super) fn lay(&mut self, br: &Braille, cx: f32, cy: f32, r: f32, inner: f32) {
        self.stone.clear();
        if let Some(f) = self.face {
            let face = &FACES[f];
            let (pw, ph) = (face.rows[0].len(), face.rows.len());
            let (sw, sh) = (pw / 2, ph / 2);
            for j in 0..sh {
                for i in 0..sw {
                    let (mut bits, mut gold, mut rock) = (0, 0, 0);
                    for q in 0..4 {
                        // Each pixel is half a unit wide and a unit tall.
                        let (px, py) = (2 * i + (q & 1), 2 * j + (q >> 1));
                        let ux = (px as f32 + 0.5 - pw as f32 / 2.0) * 0.5;
                        let uy = py as f32 + 0.5 - ph as f32 / 2.0;
                        match carve(ux.hypot(uy) / face.r, uy.atan2(ux), f) {
                            Mat::Air => continue,
                            Mat::Rock => rock += 1,
                            Mat::Gold => gold += 1,
                        }
                        bits |= 1 << q;
                    }
                    if bits == 0 {
                        continue;
                    }
                    let ux = i as f32 + 0.5 - sw as f32 / 2.0;
                    let uy = (j as f32 + 0.5 - sh as f32 / 2.0) * 2.0;
                    self.stone.push(StoneCell {
                        dx: i as i32 - (sw / 2) as i32,
                        dy: j as i32 - (sh / 2) as i32,
                        ch: QUAD[bits],
                        gold: gold >= rock,
                        rho: ux.hypot(uy) / face.r,
                        ang: uy.atan2(ux),
                        grain: hash01((j * 64 + i) as u32),
                    });
                }
            }
        }
        // Rays reach most of the way to the first orbit.
        self.ray = (0.75 * r + 0.8).min((inner - r) * 0.8).max(0.8);
        self.ring.clear();
        let (r0, r1) = (r + 0.3, r + 0.3 + self.ray);
        let (pw, ph) = (br.pw(), br.ph());
        let x0 = (((cx - r1) * 2.0).floor() as i32).max(0);
        let x1 = (((cx + r1) * 2.0).ceil() as i32).min(pw - 1);
        let y0 = (((cy - r1) * 2.0).floor() as i32).max(0);
        let y1 = (((cy + r1) * 2.0).ceil() as i32).min(ph - 1);
        for y in y0..=y1 {
            let uy = (y as f32 + 0.5) * 0.5 - cy;
            for x in x0..=x1 {
                let ux = (x as f32 + 0.5) * 0.5 - cx;
                let d2 = ux * ux + uy * uy;
                if d2 >= r0 * r0 && d2 <= r1 * r1 {
                    self.ring.push(RayDot {
                        x,
                        y,
                        d: d2.sqrt(),
                        angle: uy.atan2(ux),
                    });
                }
            }
        }
    }

    /// The Piedra del Sol: the carved stone, its gold catching a glint that
    /// sweeps slowly round it. When the sun is no bigger than a cell, a gold
    /// sun glyph.
    pub(super) fn draw_stone(&self, c: &mut Canvas, cx: f32, cy: f32) {
        let (col, row) = (cx.floor() as i32, (cy * 0.5).floor() as i32);
        if self.face.is_none() {
            c.set_attr(col, row, '☉', self.oro.at(0.85), BOLD);
            return;
        }
        let sweep = self.t * 0.6;
        // At the height of the holiday the gold blazes.
        let blaze = if self.level >= 0.9 {
            0.12 + 0.1 * (self.t * 2.4).sin()
        } else {
            0.0
        };
        for cell in &self.stone {
            let fg = if cell.gold {
                let glint = (cell.ang - sweep).cos().max(0.0).powi(6);
                let v = 0.45 + 0.3 * (1.0 - cell.rho) + 0.4 * glint + blaze;
                self.oro.at(v.min(1.0))
            } else {
                let glint = (cell.ang - sweep).cos().max(0.0).powi(8);
                let v = 0.22 + 0.4 * cell.rho + 0.25 * cell.grain + 0.3 * glint;
                self.piedra.at(v.min(1.0))
            };
            c.set(col + cell.dx, row + cell.dy, cell.ch, fg);
        }
    }

    /// The gold rays around the stone, turning slowly: eight long ones, and
    /// from the middle of the month eight short ones between them, a ripple
    /// of light running round.
    pub(super) fn draw_rays(&self, br: &mut Braille, r: f32) {
        const N: usize = 16;
        let turn = self.t * 0.05;
        let long = self.reach(r) - r - 0.3;
        let short = if self.level >= 0.5 { long * 0.6 } else { 0.0 };
        let mut cols = [Color::Reset; N];
        for (k, col) in cols.iter_mut().enumerate() {
            let lit = 0.5 + 0.5 * (self.t * 1.7 - k as f32 * 0.8).sin();
            *col = self.oro.at(0.5 + 0.3 * lit);
        }
        let (r0, per) = (r + 0.3, N as f32 / TAU);
        for dot in &self.ring {
            let into = dot.d - r0;
            // Which ray, and how far off its middle (in ray spacings).
            let rel = (dot.angle - turn) * per;
            let k = rel.round();
            let off = rel - k;
            let k = (k as i32).rem_euclid(N as i32) as usize;
            let (len, wide) = if k.is_multiple_of(2) {
                (long, 0.4)
            } else {
                (short, 0.3)
            };
            if into > len || off.abs() >= wide * (1.0 - into / len) {
                continue;
            }
            br.set(dot.x, dot.y, cols[k]);
        }
    }

    /// The campanada: rings of the bell's sound going out from the stone across
    /// the whole sky in verde, blanco and rojo, fading as they go. Only at the
    /// Grito, the more often the nearer its height.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn draw_rings(
        &self,
        br: &mut Braille,
        ctx: &Ctx,
        center: (f32, f32),
        sun_r: f32,
        big: f32,
        squash: f32,
    ) {
        if self.level < GRITO {
            return;
        }
        let every = RING_EVERY + (1.0 - self.level) * 12.0;
        let first = self.t.rem_euclid(every);
        for m in 0..3 {
            let age = first + m as f32 * every;
            if age >= RING_LIFE {
                break;
            }
            let f = age / RING_LIFE;
            let rad = sun_r + 1.0 + f * (big * 1.1 - sun_r);
            let n = ((rad * 12.0) as usize).clamp(48, 2400);
            let spin = self.t * 0.2;
            let cols = self.lit.map(|c| ctx.fade(c, 0.9 * f));
            for k in 0..n {
                let a = k as f32 / n as f32 * TAU + spin;
                let (s, co) = a.sin_cos();
                let col = cols[(k * 6 / n) % 3];
                let x = (center.0 + co * rad) * 2.0;
                let y = (center.1 + s * rad * squash) * 2.0;
                // Two dots thick.
                br.setf(x, y, col);
                br.setf(x, y + 1.0, col);
            }
        }
    }

    /// A banderita on a pole, planted on the planet at (`x`, `y`) in units: a
    /// little flag of three stripes that waves.
    pub(super) fn draw_banderita(&self, c: &mut Canvas, x: f32, y: f32, i: usize) {
        let (col, row) = (x.floor() as i32, (y * 0.5).floor() as i32);
        c.set(col, row - 1, '│', self.pole);
        c.set(col, row - 2, '│', self.pole);
        for k in 0..3 {
            let wave = (self.t * 5.0 - k as f32 * 1.4 + i as f32 * 2.0).sin();
            let ch = if wave > 0.45 {
                '▀'
            } else if wave < -0.45 {
                '▄'
            } else {
                '█'
            };
            // The stripes in their clear colors, so that they show on the dark.
            c.set(col + 1 + k as i32, row - 2, ch, self.lit[k]);
        }
    }

    /// What is shouted on the night of the Grito: "¡VIVA MÉXICO!" for a few
    /// seconds every so often, from the day it climbs to its height.
    pub(super) fn shout(&self) -> Option<&'static str> {
        let phase = (self.t + SHOUT_EVERY - 2.0).rem_euclid(SHOUT_EVERY);
        (self.level >= 0.75 && phase < SHOUT_FOR).then_some("¡VIVA MÉXICO!")
    }

    /// The shout, letter by letter in verde, blanco and rojo, along the
    /// bottom of the screen.
    pub(super) fn draw_shout(&self, c: &mut Canvas) {
        let Some(text) = self.shout() else {
            return;
        };
        let gap = 2;
        let len = (text.chars().count() as i32 - 1) * gap + 1;
        let (w, h) = (c.w() as i32, c.h() as i32);
        let (x0, y) = ((w - len) / 2, h - 2);
        // The letters have the row to themselves.
        for x in x0..x0 + len {
            c.set(x, y, ' ', Color::Reset);
        }
        for (i, ch) in text.chars().enumerate() {
            if ch == ' ' {
                continue;
            }
            let bounce = ((self.t * 6.0 - i as f32 * 0.7).sin() > 0.85) as i32;
            c.set_attr(x0 + i as i32 * gap, y - bounce, ch, self.lit[i % 3], BOLD);
        }
    }
}

/// What the Piedra del Sol is made of at a point `rho` radii from its center,
/// at angle `ang` (radians, from the right, going down). `size` is the face
/// (0 to 3): the smaller the stone the fewer rings it can show.
fn carve(rho: f32, ang: f32, size: usize) -> Mat {
    if rho >= 1.0 {
        return Mat::Air;
    }
    // Turns clockwise from straight up.
    let up = ang / TAU + 0.25;
    // How far, in fractions of a sector, from the nearest of `n` directions
    // (the first straight up, shifted by `shift` sectors).
    let off = |n: f32, shift: f32| ((up * n - shift + 0.5).rem_euclid(1.0) - 0.5).abs();
    let big = size >= 2;
    let some = size >= 1;
    // The face of Tonatiuh, the sun.
    if rho < 0.2 {
        return Mat::Gold;
    }
    // The four squares of the earlier suns, at the diagonals.
    if rho < 0.38 {
        return if big && off(4.0, 0.5) < 0.3 {
            Mat::Gold
        } else {
            Mat::Rock
        };
    }
    // A carved line between the rings.
    if some && rho < 0.43 {
        return Mat::Air;
    }
    // The ring of the day signs, twenty to a turn when there is room.
    if rho < 0.68 {
        let n = match size {
            3 => 20.0,
            2 => 14.0,
            _ => 0.0,
        };
        return if n > 0.0 && off(n, 0.5) < 0.12 {
            Mat::Air
        } else {
            Mat::Rock
        };
    }
    if some && rho < 0.74 {
        return Mat::Air;
    }
    // Eight pointers of gold, the rays of the sun, on a band of stone.
    if rho < 0.9 {
        let wide = 0.24 * (1.0 - (rho - 0.74) / 0.16 * 0.8);
        return if off(8.0, 0.0) < wide {
            Mat::Gold
        } else {
            Mat::Rock
        };
    }
    // The rim, notched like the bodies of the two fire serpents.
    if size >= 3 && rho > 0.95 && off(32.0, 0.0) < 0.2 {
        return Mat::Air;
    }
    Mat::Rock
}
