//! The backdrop, painted in one pass: the sky through the surface, the
//! water with its shafts of light, faraway ridges and nearer reefs fading
//! into the haze, and the sand under the shark with its ripples and caustics.
//!
//! Each column first works out where the waterline, the ridges, the reef and
//! the sand begin; each row has its colors worked out once; then every pixel
//! is painted exactly once.

use super::sea::{BED, SURF, Sea, View};
use super::{KEY, Rgb, Rng, WORLD_H, noise, unit};

/// Levels of brightness a light shaft comes in: few, so the water stays calm
/// for the terminal instead of every cell changing every frame.
pub const RAY_LEVELS: usize = 6;
/// How fast the layers scroll, the shark's plane being 1.
const FAR_P: f64 = 0.3;
const MID_P: f64 = 0.6;
const RAY_P: f64 = 0.5;
/// Size of the caustics texture.
const CAU_W: usize = 64;
const CAU_H: usize = 32;

pub const ROCK: Rgb = Rgb(158, 148, 132);
const SAND: Rgb = Rgb(214, 196, 150);
const CORAL_SAND: Rgb = Rgb(230, 220, 186);

/// Where things begin in one painted column.
#[derive(Clone, Copy, Default)]
pub struct Col {
    /// Rows of the waterline, the top of the far ridges, of the reef, and of
    /// the sand.
    surf: i32,
    far: i32,
    mid: i32,
    pub bed: i32,
    /// Whether the sand's edge here faces the light.
    lit: bool,
    /// The shark plane's coordinate here, and how the ripples, the
    /// caustics and the sheen under the surface wobble here.
    u: f32,
    ripple: f32,
    wobble: f32,
    sheen: f32,
}

/// What every row below the sand's edge shares, by depth into the sand.
#[derive(Clone, Copy, Default)]
pub struct SandRow {
    /// How near the viewer: the far edge of the floor is the haziest.
    near: f32,
    /// Ripples: where on a ridge this depth falls.
    ridge: f32,
    /// The caustics texture's scale and offsets.
    scale: f32,
    cv: f32,
    cu: f32,
}

impl Sea {
    /// The rows where the far ridges and the near reef begin at the layers'
    /// own coordinate `u`.
    fn far_top(&self, u: f64) -> f32 {
        let s = self.seed;
        100.0 - 26.0 * noise(u, 1.0 / 90.0, 21, s) - 9.0 * noise(u, 1.0 / 29.0, 22, s)
    }

    fn mid_top(&self, u: i64) -> f32 {
        const OUTCROP: i64 = 70;
        let s = self.seed;
        let mut top = 120.0 - 4.0 * noise(u as f64, 1.0 / 60.0, 31, s);
        let cell = u.div_euclid(OUTCROP) as i32;
        for c in cell - 1..=cell + 1 {
            if unit(c, 32, s) < 0.6 {
                let center = c as f32 * OUTCROP as f32 + unit(c, 33, s) * OUTCROP as f32;
                let hw = 12.0 + 22.0 * unit(c, 34, s);
                let hh = 8.0 + 16.0 * unit(c, 35, s);
                let d = (u as f32 - center) / hw;
                if d.abs() < 1.0 {
                    top = top.min(120.0 - hh * (1.0 - d * d).powf(0.6));
                }
            }
        }
        top
    }

    /// How far a layer that scrolls at `p` rises when the camera rises from
    /// the floor: far layers lag behind it a little.
    pub fn lift(&self, v: View, p: f64) -> i32 {
        let up = (WORLD_H as f32 - v.h - v.y).max(0.0);
        (up * (1.0 - p as f32) * 0.35).round() as i32
    }

    /// Works out the columns' outlines and the rows' colors for this frame.
    fn outline(&mut self, v: View) {
        let time = self.time();
        // The caustics move fifteen times a second: smooth enough for light
        // on sand, and half the work for the terminal.
        let shimmer = (time * 15.0).floor() / 15.0;
        let (bx, bw) = (self.bx, self.bw);
        let main = self.shift(v, 1.0);
        let far = self.shift(v, FAR_P);
        let mid = self.shift(v, MID_P);
        let (far_lift, mid_lift) = (self.lift(v, FAR_P), self.lift(v, MID_P));
        // Phases along the endless floor are worked out in double precision,
        // so they stay smooth however far the shark has swum.
        let (t, sh) = (time as f64, shimmer as f64);
        for i in 0..bw {
            let x = (bx + i) as i64;
            let u = x + main;
            let ud = u as f64;
            let bed = self.bed_at(ud);
            let col = Col {
                surf: SURF
                    + (0.9 * (ud * 0.11 + t * 0.9).sin() + 0.6 * (ud * 0.29 - t * 1.6).sin())
                        .round() as i32,
                // Without water there is no haze for the far layers to stand in.
                far: if self.set.water {
                    self.far_top((x + far) as f64) as i32 - far_lift
                } else {
                    i32::MAX
                },
                mid: if self.set.water {
                    self.mid_top(x + mid) as i32 - mid_lift
                } else {
                    i32::MAX
                },
                bed,
                lit: self.bed_at(ud - 1.0) >= bed,
                u: ud.rem_euclid(CAU_W as f64 * 4096.0) as f32,
                ripple: (0.35 * (ud * 0.13).sin() + 0.2 * (ud * 0.051).sin()) as f32,
                wobble: (1.2 * (ud * 0.13 - sh * 0.7).sin() + sh * 0.35) as f32,
                sheen: ((ud * 0.37 - t * 1.4).sin() + (ud * 0.11 + t * 0.6).sin()) as f32,
            };
            self.cols[i as usize] = col;
        }
        for d in 0..self.sand.len() {
            let near = (d as f32 / (WORLD_H - BED + 6) as f32).min(1.0);
            let scale = 0.5 + 0.5 * near;
            let cv = d as f32 * 1.1 / scale;
            self.sand[d] = SandRow {
                near,
                ridge: (d as f32 + 1.0).sqrt() * 2.6,
                scale: 0.42 / scale,
                cv,
                cu: 1.6 * (cv * 0.45 + shimmer * 0.9).sin() + shimmer * 1.1,
            };
        }
    }

    /// Paints the backdrop: everything behind what lives and grows in the sea.
    pub fn paint_backdrop(&mut self, v: View) {
        self.outline(v);
        let (bw, bh, bx, by) = (self.bw, self.bh, self.bx, self.by);
        let time = self.time();
        let water = self.set.water;
        // Shafts: soft beams dealt out along the slanted coordinate (where a
        // beam meets the surface), each drifting and pulsing on its own. The
        // sun crosses from left to right through the day, and so does the
        // shafts' slant.
        let sun = ((self.t - 0.5) * 2.4).clamp(-1.0, 1.0);
        let slant = 0.45 * sun;
        let ray_shift = self.shift(v, RAY_P) + ((by - SURF) as f32 * slant).round() as i64;
        let span = bw + (bh as f32 * slant.abs()).ceil() as i32 + 2;
        let n = (span as usize).min(self.rays.len());
        let lo = if slant < 0.0 {
            -((bh as f32 * slant.abs()).ceil() as i32) - 1
        } else {
            -1
        };
        if water {
            const BEAM: i64 = 34;
            let s = self.seed;
            // Shafts drift and pulse slowly, and in steps, to keep the
            // terminal's work down.
            let slow = (time * 4.0).floor() / 4.0;
            for k in 0..n {
                let u = (lo + k as i32 + bx) as i64 + ray_shift;
                let cell = u.div_euclid(BEAM) as i32;
                let mut r = 0.0;
                for c in cell - 1..=cell + 1 {
                    if unit(c, 61, s) > 0.6 {
                        continue;
                    }
                    let center = c as f32 * BEAM as f32
                        + unit(c, 62, s) * BEAM as f32
                        + 5.0 * (slow * 0.045 + c as f32 * 2.1).sin();
                    let width = 2.0 + 6.5 * unit(c, 63, s);
                    let pulse = 0.5 + 0.5 * (slow * (0.1 + 0.15 * unit(c, 64, s)) + c as f32).sin();
                    let d = (u as f32 - center) / width;
                    if d.abs() < 1.0 {
                        r += (1.0 - d * d) * (0.35 + 0.65 * pulse) * (0.45 + 0.55 * unit(c, 65, s));
                    }
                }
                self.rays[k] = ((r * RAY_LEVELS as f32) as usize).min(RAY_LEVELS - 1) as u8;
            }
        }
        // The sand's tones and the caustics' strength.
        // Coral sand is whiter than the sand of cool seas.
        let sand = if self.tropical { CORAL_SAND } else { SAND };
        let tones = [sand.scale(1.1), sand.scale(0.9), sand.scale(1.04), sand];
        let caustic = self.light.sky.lerp(Rgb::WHITE, 0.6);
        let glow = if self.set.rays {
            0.5 * self.daylight.powf(1.5) + 0.05
        } else {
            0.0
        };
        // Snell's window: the sky shows through right above the viewer.
        let center = v.x + v.w / 2.0;
        let sky_low = self.light.sky.lerp(Rgb::WHITE, 0.25);
        let sky_high = self.light.sky.lerp(self.light.shallow, 0.35);
        let line_c = self.light.sky.lerp(Rgb::WHITE, 0.3);
        for j in 0..bh {
            let y = by + j;
            let row = self.rows[y.clamp(0, WORLD_H - 1) as usize];
            let off = (j as f32 * slant).round() as i32 - lo;
            let line = (j * bw) as usize;
            for i in 0..bw {
                let col = self.cols[i as usize];
                let c = if y >= col.bed {
                    let d = ((y - col.bed) as usize).min(self.sand.len() - 1);
                    let sr = self.sand[d];
                    let fr = (sr.ridge + col.ripple).fract();
                    let tone = if d == 0 && col.lit {
                        0
                    } else if fr < 0.2 {
                        1
                    } else if fr > 0.55 && fr < 0.68 {
                        2
                    } else {
                        3
                    };
                    let mut c = tones[tone]
                        .mul(row.lit)
                        .lerp(row.water, 0.26 * (1.0 - sr.near));
                    if glow > 0.0 {
                        // The web of light wobbles and drifts; nearer cells look bigger.
                        let k = self.caustic(col.u * sr.scale + sr.cu, sr.cv + col.wobble);
                        if k > 0.3 {
                            let level = if k > 0.62 { 0.7 } else { 0.35 };
                            c = c.lerp(caustic, glow * level * (0.45 + 0.55 * sr.near));
                        }
                    }
                    c
                } else if y >= col.mid {
                    if y <= col.mid + 1 {
                        row.mid_top
                    } else {
                        row.mid
                    }
                } else if y >= col.far {
                    if y == col.far { row.far_top } else { row.far }
                } else if !water {
                    if y == col.surf { line_c } else { KEY }
                } else if y > col.surf + 2 {
                    let k = (i + off).clamp(0, n as i32 - 1) as usize;
                    row.ray[self.rays[k] as usize]
                } else {
                    let x = (bx + i) as f32;
                    let dx = (x - center) / (v.w * 0.32);
                    let g = (-dx * dx).exp();
                    let ripple = col.sheen;
                    let below = y - col.surf;
                    if below > 0 {
                        // The underside of the surface, a moving sheen.
                        let k = (i + off).clamp(0, n as i32 - 1) as usize;
                        let sheen = if ripple > 0.6 { 0.32 } else { 0.16 };
                        row.ray[self.rays[k] as usize].lerp(Rgb::WHITE, sheen / below as f32)
                    } else if below == 0 {
                        // The waterline itself catches the light.
                        let crest = if ripple > -0.3 { 0.62 } else { 0.4 };
                        sky_low.lerp(Rgb::WHITE, crest + 0.2 * g)
                    } else {
                        // The sky above, brightest where Snell's window is.
                        let up = (col.surf - y) as f32 / SURF as f32;
                        sky_low.lerp(sky_high, up).lerp(Rgb::WHITE, 0.3 * g)
                    }
                };
                self.buf[line + i as usize] = c;
            }
        }
    }

    /// The caustics texture at `(u, v)`, wrapping, in `[0, 1]`.
    #[inline]
    fn caustic(&self, u: f32, v: f32) -> f32 {
        let x = (u.floor() as i32).rem_euclid(CAU_W as i32) as usize;
        let y = (v.floor() as i32).rem_euclid(CAU_H as i32) as usize;
        self.caustics[y * CAU_W + x] as f32 / 255.0
    }

    /// The colors of the far layers at row `y`, for [`Sea::light_rows`].
    pub fn layer_colors(&self, y: i32) -> [Rgb; 4] {
        [
            self.shade(ROCK, y, 0.7),
            self.shade(ROCK, y, 0.78),
            self.shade(ROCK, y, 0.46),
            self.shade(ROCK, y, 0.54),
        ]
    }
}

/// A seamless texture of caustics: bright where cells of a random
/// Voronoi pattern meet, like light focused by the waves.
pub fn caustics(rng: &mut Rng) -> Vec<u8> {
    use super::Dice;
    let n = 14;
    let pts: Vec<(f32, f32)> = (0..n)
        .map(|_| (rng.rangef(0.0, CAU_W as f32), rng.rangef(0.0, CAU_H as f32)))
        .collect();
    let mut tex = vec![0u8; CAU_W * CAU_H];
    for y in 0..CAU_H {
        for x in 0..CAU_W {
            let (mut d1, mut d2) = (f32::MAX, f32::MAX);
            for &(px, py) in &pts {
                for (ox, oy) in [
                    (0.0, 0.0),
                    (CAU_W as f32, 0.0),
                    (-(CAU_W as f32), 0.0),
                    (0.0, CAU_H as f32),
                    (0.0, -(CAU_H as f32)),
                ] {
                    let (dx, dy) = (x as f32 + 0.5 - px - ox, y as f32 + 0.5 - py - oy);
                    let d = (dx * dx + dy * dy).sqrt();
                    if d < d1 {
                        d2 = d1;
                        d1 = d;
                    } else if d < d2 {
                        d2 = d;
                    }
                }
            }
            let edge = (1.0 - (d2 - d1) / 3.0).max(0.0);
            tex[y * CAU_W + x] = (edge * edge * 255.0) as u8;
        }
    }
    tex
}
