//! Equinoccio on the orrery (see the module doc in `orrery.rs`).
//!
//! Both things here are drawn in pixels a unit square, two to a cell with half
//! blocks (a cell is one unit wide and two tall), so that the Earth and the
//! serpent can have two colors in a cell. Like the other holidays it draws no
//! random numbers: the continents are a value noise of a hash, the rest a clock.

use std::f32::consts::{PI, TAU};

use super::{hash01, nearest, warmest};
use crate::color::{Color, mix_oklab, rgb};
use crate::fiesta::Fiesta;
use crate::render::{Canvas, Cell};
use crate::scene::Ctx;

/// Piedra caliza, sombra, sol, jade and cielo.
pub(super) const CALIZA: u32 = 0xc9b48a;
pub(super) const SOMBRA: u32 = 0x3b3226;
pub(super) const SOL: u32 = 0xffcf4a;
pub(super) const JADE: u32 = 0x2e8b57;
pub(super) const CIELO: u32 = 0x7ec8e3;
/// The planet's own blue, and its ice.
pub(super) const MAR: u32 = 0x4f94ee;
const HIELO: u32 = 0xf4f1ea;
/// A deeper jade for the scales, and the tongue.
const JADE_OSCURO: u32 = 0x1d6b3f;
const LENGUA: u32 = 0xce1126;

/// How much bigger the sun is than the everyday sun, to give the serpent room.
pub(super) const GROW: f32 = 1.4;

/// The Earth's axis leans 23.4 degrees from the vertical, its north pole to
/// the right.
const TILT: f32 = 23.4 * PI / 180.0;
/// How far the axis reaches out beyond each pole, in units.
const AXIS_EXTRA: f32 = 1.8;
/// The coil's ring is seen squashed like the orbits, by about this much.
const SQUASH: f32 = 0.34;

/// Two pixels (the upper and the lower half of the cell) written to a cell
/// the way `Pixels::blit` does: an empty half stays transparent.
fn put_pair(c: &mut Canvas, col: i32, row: i32, top: Color, bottom: Color) {
    let bg = c.bg();
    let strict = c.strict;
    let cell = match (top, bottom) {
        (Color::Reset, Color::Reset) => return,
        (t, Color::Reset) => Cell {
            ch: '▀',
            fg: t,
            bg,
            attr: 0,
        },
        (Color::Reset, b) => Cell {
            ch: '▄',
            fg: b,
            bg,
            attr: 0,
        },
        (t, b) if t == b => Cell {
            ch: '█',
            fg: t,
            bg,
            attr: 0,
        },
        (t, b) if strict => Cell {
            ch: '█',
            fg: t.lerp(b, 0.5),
            bg,
            attr: 0,
        },
        (t, b) => Cell {
            ch: '▀',
            fg: t,
            bg: b,
            attr: 0,
        },
    };
    if let Some(slot) = c.get_mut(col, row) {
        *slot = cell;
    }
}

/// A patch of square pixels in screen units, kept between frames (it is
/// allocated when the layout is made) and blitted onto the canvas.
struct Raster {
    x0: i32,
    y0: i32,
    w: usize,
    h: usize,
    px: Vec<Color>,
}

impl Raster {
    fn new() -> Raster {
        Raster {
            x0: 0,
            y0: 0,
            w: 0,
            h: 0,
            px: Vec::new(),
        }
    }

    /// Covers the units from (`x0`, `y0`) across `w` and down `h`; the top
    /// edge is moved up to an even unit so that pixels pair up into cells.
    fn place(&mut self, x0: i32, y0: i32, w: i32, h: i32) {
        let y0 = y0 & !1;
        self.x0 = x0;
        self.y0 = y0;
        self.w = w.max(0) as usize;
        self.h = (h.max(0) as usize + 1) & !1;
        self.px.clear();
        self.px.resize(self.w * self.h, Color::Reset);
    }

    fn clear(&mut self) {
        self.px.fill(Color::Reset);
    }

    #[inline]
    fn set(&mut self, x: i32, y: i32, c: Color) {
        let (i, j) = (x - self.x0, y - self.y0);
        if i >= 0 && j >= 0 && (i as usize) < self.w && (j as usize) < self.h {
            self.px[j as usize * self.w + i as usize] = c;
        }
    }

    fn blit(&self, c: &mut Canvas) {
        for j in (0..self.h).step_by(2) {
            let row = (self.y0 + j as i32) / 2;
            for i in 0..self.w {
                let top = self.px[j * self.w + i];
                let bottom = self.px[(j + 1) * self.w + i];
                put_pair(c, self.x0 + i as i32, row, top, bottom);
            }
        }
    }
}

/// The colors of the Earth.
#[derive(Clone, Copy)]
struct Globe {
    /// Day and night: ocean, land, ice.
    sea: [Color; 2],
    land: [Color; 2],
    ice: [Color; 2],
    /// The line where day meets night, the axis outside the globe, the thin
    /// atmosphere on the lit side.
    line: Color,
    axis: Color,
    air: Color,
}

pub(super) struct Equinoccio {
    pub(super) on: bool,
    pub(super) level: f32,
    /// Seconds of holiday: the Earth turning, the serpent crawling.
    t: f32,
    /// The Earth's radius in units, from the layout.
    pub(super) globe: f32,
    /// The sun, in units, as the serpent coils around it (from the layout);
    /// the serpent is left out when the sun is too small for it.
    sun: (f32, f32, f32),
    /// How thick the serpent is: its half-width in units.
    body: f32,
    /// The part of the coil behind the sun and the part in front of it.
    back: Raster,
    front: Raster,
    paint: Globe,
    jade: [Color; 2],
    plume: [Color; 2],
    sol: Color,
    /// The pale turquoise the light turns the serpent's scales.
    light: Color,
    caliza: Color,
    sombra: Color,
    lengua: Color,
}

impl Equinoccio {
    pub(super) fn new() -> Equinoccio {
        let none = [Color::Reset; 2];
        Equinoccio {
            on: false,
            level: 0.0,
            t: 0.0,
            globe: 2.0,
            sun: (0.0, 0.0, 0.0),
            body: 1.0,
            back: Raster::new(),
            front: Raster::new(),
            paint: Globe {
                sea: none,
                land: none,
                ice: none,
                line: Color::Reset,
                axis: Color::Reset,
                air: Color::Reset,
            },
            jade: none,
            plume: none,
            sol: Color::Reset,
            light: Color::Reset,
            caliza: Color::Reset,
            sombra: Color::Reset,
            lengua: Color::Reset,
        }
    }

    pub(super) fn set(&mut self, fiesta: Fiesta) {
        self.on = fiesta.is(crate::fiesta::Kind::Equinoccio);
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
        let sol = if native {
            rgb(SOL)
        } else {
            warmest(&ctx.theme.colors).unwrap_or(g.at(0.8))
        };
        let white = if native { rgb(HIELO) } else { g.at(1.0) };
        let caliza = if native {
            rgb(CALIZA)
        } else {
            mix_oklab(sol, white, 0.5)
        };
        let (sea, land, cielo) = (tint(MAR), tint(JADE), tint(CIELO));
        // The night side is the day's color with most of the light gone.
        let night = |c: Color| ctx.fade(c, 0.6);
        self.paint = Globe {
            sea: [sea, night(sea)],
            land: [land, night(land)],
            ice: [white, night(white)],
            line: sol,
            axis: mix_oklab(caliza, g.at(0.4), 0.4),
            air: cielo,
        };
        self.jade = [
            land,
            if native {
                rgb(JADE_OSCURO)
            } else {
                mix_oklab(land, g.at(0.2), 0.45)
            },
        ];
        self.plume = [cielo, sol];
        self.sol = sol;
        self.light = mix_oklab(cielo, white, 0.55);
        self.caliza = caliza;
        self.sombra = if native {
            rgb(SOMBRA)
        } else {
            ctx.fade(caliza, 0.7)
        };
        self.lengua = tint(LENGUA);
    }

    /// Moves the holiday's clock along.
    pub(super) fn step(&mut self, flow: f32) {
        self.t = (self.t + flow).rem_euclid(1000.0 * TAU);
    }

    /// Sizes the Earth and the serpent's coil once the layout is known: the
    /// Earth is big, and the serpent coils round the sun at (`cx`, `cy`).
    pub(super) fn lay(&mut self, cx: f32, cy: f32, sun_r: f32, big: f32) {
        self.globe = (big * 0.07).clamp(2.5, 7.0);
        self.sun = (cx, cy, sun_r);
        self.body = (0.17 * sun_r).clamp(0.8, 1.3);
        // Room for the coil and its head, tail and plumes.
        let reach = sun_r + 7.0;
        let (x0, y0) = (
            (cx - reach).floor() as i32,
            (cy - reach - 4.0).floor() as i32,
        );
        let (w, h) = (
            (2.0 * reach).ceil() as i32 + 1,
            (2.0 * reach + 10.0).ceil() as i32,
        );
        self.back.place(x0, y0, w, h);
        self.front.place(x0, y0, w, h);
    }

    /// How far the Earth pushes its moon out, in units.
    pub(super) fn pad(&self, kind: usize) -> f32 {
        if self.on && kind == 2 {
            self.globe
        } else {
            0.0
        }
    }

    /// The serpent is drawn when the sun is big enough to coil around.
    fn coils(&self) -> bool {
        self.sun.2 >= 2.0
    }

    /// Draws the serpent into its two rasters: one for the part of the coil
    /// behind the sun, one for what is in front of it.
    pub(super) fn render(&mut self) {
        self.back.clear();
        self.front.clear();
        if !self.coils() {
            return;
        }
        let (cx, cy, r) = self.sun;
        let w_max = self.body;
        // Not quite two turns on the first days of the holiday, two and a bit at its height.
        let turns = 0.7 + 1.0 * self.level;
        let pitch = 4.0 * w_max;
        let rise = turns * pitch;
        let n = ((turns * TAU * (r + 1.0) * 1.6) as usize).clamp(40, 600);
        // The light runs down the body: four triangles, seven at the height.
        let triangles = if self.level >= 0.9 { 7.0 } else { 4.0 };
        // The whole coil turns slowly round the sun.
        let spin = self.t * 0.25;
        let mut head = (0.0, 0.0, true);
        for m in 0..=n {
            let u = m as f32 / n as f32;
            let phi = TAU * turns * u + spin;
            let (sp, cp) = phi.sin_cos();
            // A ripple runs along the body, and the tail flicks out.
            let rho = r
                + 1.5
                + w_max * 0.4 * (u * TAU * 3.0 - self.t * 1.6).sin()
                + 2.4 * (1.0 - u).powi(3);
            let x = cx + rho * cp;
            let y = cy + rho * sp * SQUASH - (u - 0.5) * rise;
            let front = sp > 0.0;
            // The tail tapers to a point.
            let w = w_max * (0.25 + 0.75 * (u * 3.0).min(1.0));
            // Each triangle of light is brightest at its head and fades toward its tail.
            let light = 1.0 - (u * triangles + self.t * 0.35).rem_euclid(1.0);
            let scale = (m / 3) % 2;
            let ras = if front {
                &mut self.front
            } else {
                &mut self.back
            };
            let (px, py) = (x.floor() as i32, y.floor() as i32);
            let reach = w.ceil() as i32 + 1;
            for dy in -reach..=reach {
                for dx in -reach..=reach {
                    let (ex, ey) = ((px + dx) as f32 + 0.5 - x, (py + dy) as f32 + 0.5 - y);
                    let d = ex.hypot(ey);
                    if d > w {
                        continue;
                    }
                    // Jade in bands, with the light falling in triangles along it.
                    let lit = light * light * (1.0 - 0.5 * (d / w));
                    let col = self.jade[0]
                        .lerp(self.jade[1], 0.55 * scale as f32)
                        .lerp(self.light, lit);
                    ras.set(px + dx, py + dy, col);
                }
            }
            // A mane of feathers along the back.
            if u > 0.12 && m % 4 == 0 {
                let col = self.plume[(m / 4) % 2];
                let top = (y - w - 0.5).floor() as i32;
                ras.set(px, top, col);
                if (m / 4) % 4 == 0 {
                    ras.set(px, top - 1, col);
                }
            }
            if m == n {
                head = (x, y, front);
            }
        }
        // The head, looking away from the sun.
        let (hx, hy, front) = head;
        let face = if hx >= cx { 1 } else { -1 };
        let ras = if front {
            &mut self.front
        } else {
            &mut self.back
        };
        let sprite: &[&str] = if r >= 5.0 { &HEAD_BIG } else { &HEAD_SMALL };
        let (ax, ay) = (hx.floor() as i32 - 1, hy.floor() as i32 - 2);
        let flick = (self.t * 3.0).rem_euclid(1.0) < 0.55;
        for (j, row) in sprite.iter().enumerate() {
            for (i, p) in row.bytes().enumerate() {
                let col = match p {
                    b'c' => self.caliza,
                    b'e' => self.sol,
                    b's' => self.sombra,
                    b'j' => self.jade[0],
                    b'F' => self.plume[0],
                    b'f' => self.plume[1],
                    b't' if flick => self.lengua,
                    _ => continue,
                };
                ras.set(ax + face * i as i32, ay + j as i32, col);
            }
        }
    }

    /// How many pixels of the serpent are drawn, behind the sun and in front of
    /// it (for the tests).
    #[cfg(test)]
    pub(super) fn counts(&self) -> (usize, usize) {
        let n = |r: &Raster| r.px.iter().filter(|&&c| c != Color::Reset).count();
        (n(&self.back), n(&self.front))
    }

    /// How many pixels the serpent has in all (for the tests).
    #[cfg(test)]
    pub(super) fn pixels(&self) -> usize {
        let (a, b) = self.counts();
        a + b
    }

    /// The part of the serpent behind the sun.
    pub(super) fn blit_back(&self, c: &mut Canvas) {
        self.back.blit(c);
    }

    /// The part of the serpent in front of the sun.
    pub(super) fn blit_front(&self, c: &mut Canvas) {
        self.front.blit(c);
    }

    /// The Earth, big: a globe with its axis tilted, lit on the side facing
    /// the sun at (`sun.0`, `sun.1`), with the line between day and night
    /// running from pole to pole, so that day and night are split equally.
    /// Seen from the side, the terminator is the axis. The land turns slowly.
    pub(super) fn draw_earth(&self, c: &mut Canvas, x: f32, y: f32, sun: (f32, f32)) {
        let rad = self.globe;
        let reach = rad + AXIS_EXTRA + 1.0;
        let (sin_t, cos_t) = TILT.sin_cos();
        // Along the axis (north, up and a little to the right) and across it.
        let (ax, ay) = (sin_t, -cos_t);
        let (bx, by) = (cos_t, sin_t);
        // Which side of the axis the sun is on, and so which half is day: the
        // halves trade places smoothly as the Earth passes between the sun and
        // the line of its axis.
        let toward = ((sun.0 - x) * bx + (sun.1 - y) * by) / 3.0;
        let day_right = 0.5 + 0.5 * toward.clamp(-1.0, 1.0);
        let spin = self.t * 0.22;
        let g = &self.paint;
        let (x0, x1) = ((x - reach).floor() as i32, (x + reach).ceil() as i32);
        let (y0, y1) = ((y - reach).floor() as i32, (y + reach).ceil() as i32);
        for row in (y0.div_euclid(2))..=(y1.div_euclid(2)) {
            for col in x0..=x1 {
                let mut half = [Color::Reset; 2];
                for (k, px) in half.iter_mut().enumerate() {
                    let (dx, dy) = (col as f32 + 0.5 - x, (2 * row + k as i32) as f32 + 0.5 - y);
                    let (s, u) = (dx * ax + dy * ay, dx * bx + dy * by);
                    let d2 = dx * dx + dy * dy;
                    *px = if d2 < rad * rad {
                        if u.abs() < 0.5 {
                            // The line between day and night, pole to pole.
                            g.line
                        } else {
                            let day = if u > 0.0 { day_right } else { 1.0 - day_right };
                            self.globe_px(s, u, rad, spin, day)
                        }
                    } else if u.abs() < 0.5 && s.abs() < rad + AXIS_EXTRA {
                        // The axis reaching out of both poles; the north one is brighter.
                        if s > 0.0 { g.line } else { g.axis }
                    } else {
                        Color::Reset
                    };
                }
                put_pair(c, col, row, half[0], half[1]);
            }
        }
    }

    /// One pixel of the globe: `s` along the axis and `u` across it, in
    /// units from the center, `day` how much light it gets (0 to 1).
    fn globe_px(&self, s: f32, u: f32, rad: f32, spin: f32, day: f32) -> Color {
        let g = &self.paint;
        // Latitude and longitude on the sphere, seen from the side.
        let lat = (s / rad).clamp(-1.0, 1.0).asin();
        let z = (rad * rad - s * s - u * u).max(0.0).sqrt();
        let lon = u.atan2(z) + spin;
        let kind = if lat.abs() > 1.2 {
            2
        } else if land(lon, lat) {
            1
        } else {
            0
        };
        let (lit, dark) = match kind {
            2 => (g.ice[0], g.ice[1]),
            1 => (g.land[0], g.land[1]),
            _ => (g.sea[0], g.sea[1]),
        };
        // Brighter away from the terminator, a thin atmosphere at the edge.
        let shade = 0.7 + 0.3 * (u.abs() / rad).min(1.0);
        let mut col = dark.lerp(lit.scale(shade), day);
        if day > 0.5 && u * u + s * s > (rad - 0.9) * (rad - 0.9) {
            col = col.lerp(g.air, 0.45 * day);
        }
        col
    }
}

/// Is there land at this longitude and latitude (radians)? A value noise of
/// two octaves on a lattice that wraps around the globe.
fn land(lon: f32, lat: f32) -> bool {
    let x = (lon / TAU).rem_euclid(1.0);
    let y = lat / PI + 0.5;
    let n = 0.65 * vnoise(x * 6.0, y * 4.0, 6) + 0.35 * vnoise(x * 12.0 + 3.0, y * 8.0, 12);
    n > 0.53
}

/// Smooth value noise at (`x`, `y`), 0 to 1, repeating every `period` in `x`.
fn vnoise(x: f32, y: f32, period: i32) -> f32 {
    let (ix, iy) = (x.floor(), y.floor());
    let (fx, fy) = (x - ix, y - iy);
    let at = |i: i32, j: i32| hash01((i.rem_euclid(period) * 131 + j * 977 + 7) as u32);
    let (i, j) = (ix as i32, iy as i32);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let top = at(i, j) + (at(i + 1, j) - at(i, j)) * sx;
    let bottom = at(i, j + 1) + (at(i + 1, j + 1) - at(i, j + 1)) * sx;
    top + (bottom - top) * sy
}

/// The serpent's head in pixels, looking to the right: `c` limestone, `e` the
/// eye, `s` shadow in the open mouth, `j` the neck, `F` and `f` plumes, `t`
/// the tongue that flicks.
const HEAD_BIG: [&str; 6] = [
    "Ff.cccc...",
    ".Fcccccc..",
    "jjccecccc.",
    "jjcccccccc",
    ".jsssssss.",
    ".......tt.",
];
const HEAD_SMALL: [&str; 4] = ["F.ccc..", "jccecc.", "jcccccc", "..sss.t"];
