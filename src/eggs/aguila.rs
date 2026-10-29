//! The águila real flies across the screen with a serpent in its beak and
//! lands on a nopal growing from a rock, the emblem of Mexico, during the
//! Fiestas Patrias.
//!
//! Original oxiper pixel art, all of it drawn from shapes so that it moves
//! smoothly. First a rock rises from the lake and the nopal sprouts its
//! pencas one by one, tunas on their edges. Then the golden eagle sweeps in
//! with big wingbeats, each wing a fan of feathers (light coverts, dark
//! primaries, fingers at the tip) swinging from high above the body to low
//! behind it, the far wing a shade darker, a golden nape, a hooked beak, the
//! tail a fan, and the serpent hanging from the beak and writhing in the
//! wind. It glides down, flares with its wings high and its talons out, and
//! lands on the highest penca: body upright, wings spread, the serpent still
//! in its beak. A sunburst of golden rays opens behind it, glints sparkle,
//! the wings stretch now and then, and at the end it all fades away. It comes
//! from either side and is gone within 30 s.

use super::patrias::{Palette, fade};
use super::stage::{Pen, Stage, hash, unit};
use super::{Egg, Info};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

pub const INFO: Info = Info {
    name: "aguila",
    start,
    fiesta: Kind::Patrias,
    when: super::any_day,
};

/// The eagle enters at `ENTER`, lands at `LANDS`, the sunburst glows until
/// the picture fades at `FADES` and it is over at `END`.
const ENTER: f32 = 0.8;
const LANDS: f32 = 10.2;
const FADES: f32 = 21.0;
const END: f32 = 24.2;
/// Wingbeats per second.
const BEATS: f32 = 1.5;

/// The pencas of the nopal: middle (pixels right of the stem, up from the
/// rock), half-width, half-height, and the second its sprouts.
const PADS: &[(f32, f32, f32, f32, f32)] = &[
    (0.0, 7.0, 5.5, 7.5, 0.5),
    (-14.0, 9.0, 3.5, 5.0, 1.0),
    (13.0, 8.0, 3.5, 5.0, 1.1),
    (-8.0, 16.0, 4.5, 6.5, 0.9),
    (8.0, 17.0, 4.5, 6.5, 1.2),
    (-1.0, 25.0, 5.0, 6.5, 1.6),
];
/// Where the eagle lands: the top of the last penca.
const PERCH: usize = 5;
const ROCK_H: i32 = 8;
const ROCK_W: f32 = 19.0;
/// Where the tunas hang: (penca, along its edge, -1 left to 1 right).
const TUNAS: &[(usize, f32)] = &[
    (1, -0.7),
    (2, 0.8),
    (3, -0.6),
    (4, 0.7),
    (5, 0.55),
    (0, -0.8),
];

fn start(_w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Aguila {
        t: 0.0,
        from_left: rng.bool(),
        seed: rng.u32(..),
    })
}

struct Aguila {
    t: f32,
    /// Flying from the left edge to the right (else the other way round).
    from_left: bool,
    seed: u32,
}

/// Where things are on a stage, in the dance's own left-to-right (the
/// eagle comes from the left); the picture is mirrored when it does not.
struct Geo {
    st: Stage,
    mirror: bool,
    /// The nopal's stem and the rock's top, the nopal's size, the perch.
    nx: i32,
    rock_top: i32,
    size: f32,
    perch: (f32, f32),
    /// How big the eagle is (1 on a tall stage), and the height it cruises at.
    sz: f32,
    cruise: f32,
}

impl Geo {
    fn new(st: Stage, from_left: bool) -> Geo {
        let g = st.h;
        // The eagle's size first, then a nopal that leaves it room above.
        let sz = (g as f32 / 70.0).clamp(0.55, 1.0);
        let size = ((g as f32 - 12.0 - 33.0 * sz) / 31.5).clamp(0.4, 1.0);
        let nx = (st.w as f32 * 0.68) as i32;
        let rock_top = g - ROCK_H;
        let (dx, dy, _, ry, _) = PADS[PERCH];
        let perch = (nx as f32 + dx * size, rock_top as f32 - (dy + ry) * size);
        Geo {
            st,
            mirror: !from_left,
            nx,
            rock_top,
            size,
            perch,
            sz,
            // High enough for the wings raised, but never below the perch.
            cruise: (34.0 * sz + 6.0).min(perch.1 - 11.0 * sz - 2.0),
        }
    }
}

/// 0 to 1 as `x` goes from `a` to `b`, eased.
fn ease(x: f32, a: f32, b: f32) -> f32 {
    let k = ((x - a) / (b - a)).clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

fn lerp(a: f32, b: f32, k: f32) -> f32 {
    a + (b - a) * k
}

/// The eagle at a moment.
struct Bird {
    /// The middle of its body.
    x: f32,
    y: f32,
    /// Nose up (radians), and the wings' mean angle and sweep.
    pitch: f32,
    wing: f32,
    sweep: f32,
    /// How far the legs reach down, 0 tucked to 1 out for landing.
    legs: f32,
    /// Is it on the nopal?
    perched: bool,
    /// Its size (see [`Geo::sz`]).
    sz: f32,
}

impl Aguila {
    fn bird(&self, geo: &Geo) -> Bird {
        let t = self.t;
        let s = ((t - ENTER) / (LANDS - ENTER)).clamp(0.0, 1.0);
        let flare = ease(s, 0.78, 1.0);
        let settle = ease(t, LANDS, LANDS + 1.2);
        let (px, py) = geo.perch;
        let (land_x, land_y) = (px - 1.0, py - 11.0 * geo.sz);
        let x = lerp(-34.0, land_x, 1.0 - (1.0 - s) * (1.0 - s));
        let bob = (t * BEATS * TAU).sin() * 1.4 * (1.0 - flare);
        let y = lerp(geo.cruise, land_y, ease(s, 0.42, 1.0)) + bob;
        let pitch = lerp(-0.1, 0.2, ease(s, 0.5, 0.8));
        let pitch = lerp(pitch, 0.96, flare * flare);
        let wing = lerp(lerp(2.79, 2.1, flare), 2.2, settle);
        // Big beats in flight, small flutters at the perch, and now and then
        // a stretch of the wings.
        let stretch = ((t - LANDS) / 5.0 * TAU).sin().max(0.0).powi(10) * 0.55;
        let sweep = lerp(lerp(0.9, 0.5, flare), 0.1 + stretch, settle);
        Bird {
            x,
            y,
            pitch,
            wing: wing + 0.08 * settle * (t * 1.3).sin(),
            sweep,
            legs: ease(s, 0.8, 1.0),
            perched: t >= LANDS,
            sz: geo.sz,
        }
    }
}

/// Turns (x forward, y up) by `a` radians counter-clockwise.
fn rot((x, y): (f32, f32), a: f32) -> (f32, f32) {
    let (s, c) = a.sin_cos();
    (x * c - y * s, x * s + y * c)
}

impl Egg for Aguila {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let st = Stage::new(c.w(), c.h());
        let geo = Geo::new(st, self.from_left);
        let gone = ease(self.t, FADES, END - 0.2);
        if gone > 0.97 {
            return;
        }
        let mut pal = Palette::patrias(theme);
        let mut far = Palette::patrias(theme);
        far.fade_all(theme, 0.4);
        if gone > 0.0 {
            pal.fade_all(theme, gone);
            far.fade_all(theme, gone);
        }
        let mut pen = Pen {
            c,
            st,
            mirror: geo.mirror,
        };
        let bird = self.bird(&geo);
        // The glow behind the bird once it has landed.
        if self.t >= LANDS {
            self.sunburst(&mut pen, &bird, theme, gone);
        }
        self.lake_and_rock(&mut pen, &geo, &pal);
        self.nopal(&mut pen, &geo, &pal);
        if self.t >= ENTER - 0.05 {
            self.eagle(&mut pen, &bird, &pal, &far);
        }
        if self.t >= LANDS + 0.3 {
            self.glints(&mut pen, &bird, &pal, theme, gone);
        }
    }

    fn done(&self) -> bool {
        self.t > END
    }
}

impl Aguila {
    /// The rock rising out of the lake.
    fn lake_and_rock(&self, pen: &mut Pen, geo: &Geo, pal: &Palette) {
        let g = geo.st.h;
        let rise = ease(self.t, 0.0, 0.8);
        let (water, ripple) = (pal.of(b'q'), pal.of(b'Q'));
        // The lake: two rows of water with ripples that drift.
        let reach = 38;
        for dx in -reach..=reach {
            let x = geo.nx + dx;
            let k = 1.0 - (dx.abs() as f32 / reach as f32).powi(3);
            if k * rise < 0.15 {
                continue;
            }
            pen.px(x, g - 1, water);
            if (dx * 3 + (self.t * 3.0) as i32).rem_euclid(7) < 2 && k > 0.4 {
                pen.px(x, g - 2, ripple);
            }
        }
        // The rock: a mound of stone with lit edges and a little moss.
        let (stone, shade, lit, moss) = (pal.of(b'Z'), pal.of(b'z'), pal.of(b'W'), pal.of(b'n'));
        let lift = ((1.0 - rise) * (ROCK_H + 2) as f32) as i32;
        for r in 0..ROCK_H {
            let half = (ROCK_W * (0.45 + 0.55 * r as f32 / (ROCK_H - 1) as f32)) as i32;
            let y = geo.rock_top + r + lift;
            for dx in -half..=half {
                let d = hash((dx + 100) as u32, r as u32 + self.seed % 3);
                let col = if dx < -half + 2 || r == 0 {
                    lit.lerp(stone, 0.45)
                } else if dx > half - 3 || d.is_multiple_of(6) {
                    shade
                } else if r == 1 && d.is_multiple_of(4) {
                    moss
                } else {
                    stone
                };
                pen.px(geo.nx + dx, y, col);
            }
        }
    }

    /// The nopal: pencas that sprout one by one, with their spines and tunas.
    fn nopal(&self, pen: &mut Pen, geo: &Geo, pal: &Palette) {
        let (green, dark, spine) = (pal.of(b'N'), pal.of(b'n'), pal.of(b'm'));
        let base = (geo.nx as f32, geo.rock_top as f32 + 1.0);
        for (i, &(dx, dy, rx, ry, born)) in PADS.iter().enumerate() {
            let grow = ease(self.t, born, born + 0.7);
            if grow <= 0.0 {
                continue;
            }
            let (rx, ry) = (rx * geo.size * grow, ry * geo.size * grow);
            let (cx, cy) = (
                base.0 + dx * geo.size,
                base.1 - dy * geo.size * grow.max(0.3),
            );
            let (x0, x1) = ((cx - rx).floor() as i32, (cx + rx).ceil() as i32);
            let (y0, y1) = ((cy - ry).floor() as i32, (cy + ry).ceil() as i32);
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let (u, v) = ((x as f32 + 0.5 - cx) / rx, (y as f32 + 0.5 - cy) / ry);
                    let d = u * u + v * v;
                    if d > 1.0 {
                        continue;
                    }
                    let h = hash((x + 50) as u32, (y + 50) as u32 + i as u32);
                    let col = if d > 0.62 {
                        dark
                    } else if h.is_multiple_of(9) {
                        spine
                    } else {
                        green
                    };
                    pen.px(x, y, col);
                }
            }
        }
        // Tunas on the edges of the pencas, once grown.
        for &(i, side) in TUNAS {
            let (dx, dy, rx, ry, born) = PADS[i];
            let grow = ease(self.t, born + 0.6, born + 1.2);
            if grow <= 0.0 {
                continue;
            }
            let (tx, ty) = (
                base.0 + (dx + side * rx * 0.8) * geo.size,
                base.1 - (dy + ry * 0.75) * geo.size,
            );
            let (x, y) = (tx.round() as i32, ty.round() as i32);
            pen.px(x, y, pal.of(b'P'));
            pen.px(x, y - 1, pal.of(b'P'));
            pen.px(x + 1, y - 1, pal.of(b'o'));
        }
    }

    /// The golden rays opening behind the bird.
    fn sunburst(&self, pen: &mut Pen, bird: &Bird, theme: &Theme, gone: f32) {
        let open = ease(self.t, LANDS, LANDS + 1.6);
        let glow = Palette::patrias(theme).of(b'Y');
        let (cx, cy) = (bird.x, bird.y - 1.0);
        let spin = self.t * 0.18;
        for i in 0..24 {
            let a = spin + i as f32 * TAU / 24.0;
            let long = if i % 2 == 0 { 36.0 } else { 26.0 } * bird.sz;
            let (r0, r1) = (
                14.0 * bird.sz,
                14.0 * bird.sz + (long - 14.0 * bird.sz) * open,
            );
            let pulse = 0.5 + 0.2 * (self.t * 2.0 + i as f32).sin();
            let k = (1.0 - pulse * open).max(gone);
            if k > 0.9 {
                continue;
            }
            let col = fade(theme, glow, k);
            let from = (
                (cx + a.cos() * r0) as i32,
                (cy - a.sin() * r0 * 0.85) as i32,
            );
            let to = (
                (cx + a.cos() * r1) as i32,
                (cy - a.sin() * r1 * 0.85) as i32,
            );
            pen.line(from, to, 1, col);
        }
    }

    /// Glints of light around the perched bird.
    fn glints(&self, pen: &mut Pen, bird: &Bird, pal: &Palette, theme: &Theme, gone: f32) {
        for i in 0..10u32 {
            let period = 1.2 + unit(i, self.seed) * 1.0;
            let phase = ((self.t + unit(i, 9) * 4.0) / period).fract();
            if phase > 0.3 {
                continue;
            }
            let round = ((self.t + unit(i, 9) * 4.0) / period) as u32;
            let (a, r) = (
                unit(i, round ^ self.seed) * TAU,
                14.0 + unit(i, round) * 20.0,
            );
            let (x, y) = (
                (bird.x + a.cos() * r) as i32,
                (bird.y - a.sin() * r * 0.8) as i32,
            );
            let k = ((phase / 0.3 - 0.5).abs() * 2.0).max(gone);
            if k > 0.9 {
                continue;
            }
            let col = fade(theme, pal.of(b'Y'), k);
            for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                pen.px(x + dx, y + dy, col);
            }
        }
    }

    /// The eagle: far wing, tail, body, legs, head, near wing and serpent.
    fn eagle(&self, pen: &mut Pen, b: &Bird, pal: &Palette, far: &Palette) {
        let (cx, cy, sz) = (b.x, b.y, b.sz);
        // Body frame: forward along the pitch, up across it.
        let to_screen = |p: (f32, f32)| {
            let (x, y) = rot((p.0 * sz, p.1 * sz), b.pitch);
            (cx + x, cy - y)
        };
        let ipt = |p: (f32, f32)| (p.0.round() as i32, p.1.round() as i32);
        let shoulder = to_screen((3.5, 2.4));
        // The far wing swings a little behind the near one, and darker.
        self.wing(pen, shoulder, (b.wing + 0.4, b.sweep), sz, far, (-2, -1));
        // The tail: a fan of feathers.
        let rear = (-10.5, -0.4);
        for i in 0..6 {
            let a = PI + (i as f32 - 2.5) * 0.17;
            let tip = (rear.0 + a.cos() * 11.0, rear.1 + a.sin() * 11.0 - 1.0);
            let col = if i % 2 == 0 {
                pal.of(b'd')
            } else {
                pal.of(b'E')
            };
            pen.line(ipt(to_screen(rear)), ipt(to_screen(tip)), 1, col);
        }
        // The body: an ellipse, lighter below, darker scallops of feathers.
        let (ea, eb) = (11.5 * sz, 4.6 * sz);
        let r = (12.0 * sz).ceil() as i32;
        for iy in -r..=r {
            for ix in -r..=r {
                let local = rot((ix as f32, iy as f32), -b.pitch);
                let (u, v) = (local.0 / ea, local.1 / eb);
                if u * u + v * v > 1.0 {
                    continue;
                }
                let h = hash((ix + 40) as u32, (iy + 40) as u32 + (b.pitch * 9.0) as u32);
                let col = if v < -0.35 {
                    pal.of(b'e')
                } else if h.is_multiple_of(7) {
                    pal.of(b'd')
                } else {
                    pal.of(b'E')
                };
                pen.px(cx.round() as i32 + ix, cy.round() as i32 - iy, col);
            }
        }
        // The legs and talons: tucked in flight, reaching down to land.
        let reach = lerp(3.0, 9.0, b.legs) * sz;
        let leg = pal.of(b'O');
        for off in [-1.5f32, 2.0] {
            let hip = to_screen((off, -3.5));
            let foot = if b.perched {
                (hip.0 + 0.5 * (1.0 + off), hip.1 + reach)
            } else {
                (hip.0 - 1.0 - 0.3 * reach, hip.1 + reach)
            };
            pen.line(ipt(hip), ipt(foot), 1, leg);
            pen.px(foot.0 as i32 + 1, foot.1 as i32, leg);
            pen.px(foot.0 as i32 - 1, foot.1 as i32 + 1, leg);
        }
        // The head, with a golden nape, an eye and a hooked beak.
        let (hx, hy) = ipt(to_screen((11.0, 2.0)));
        let hr = if sz > 0.8 { 10 } else { 7 };
        for dy in -3..=3i32 {
            for dx in -3..=3i32 {
                if dx * dx + dy * dy > hr {
                    continue;
                }
                let col = if dx < 0 && dy <= 0 {
                    pal.of(b'O')
                } else {
                    pal.of(b'e')
                };
                pen.px(hx + dx, hy + dy, col);
            }
        }
        pen.px(hx + 1, hy - 1, pal.of(b'W'));
        pen.px(hx + 2, hy - 1, pal.of(b'k'));
        let (bx, by) = (hx + 3, hy + 1);
        pen.line((hx + 2, hy), (bx, by), 2, pal.of(b'O'));
        pen.px(bx + 1, by + 1, pal.of(b'a'));
        pen.px(bx + 1, by + 2, pal.of(b'a'));
        // The near wing, over the body.
        self.wing(pen, shoulder, (b.wing, b.sweep), sz, pal, (0, 0));
        // The serpent hangs from the beak, writhing.
        self.serpent(pen, (bx + 1, by + 2), b, pal);
    }

    /// A wing: a fan of feathers along an arm and a hand. `angle` is the
    /// arm's direction (radians from forward, counter-clockwise: 1.7 is up
    /// and a little back, 3.14 straight back, 4.4 down and back); the
    /// feathers trail from it toward the back of the bird.
    fn wing(
        &self,
        pen: &mut Pen,
        at: (f32, f32),
        (angle, sweep): (f32, f32),
        sz: f32,
        pal: &Palette,
        off: (i32, i32),
    ) {
        let (arm, hand) = (11.5 * sz, 13.5 * sz);
        // The hand trails the arm on the downstroke and leads on the upstroke.
        let lag = sweep * (self.t * BEATS * TAU + 1.2).sin() * 0.5;
        let (a1, a2) = (angle, angle + lag);
        let wrist = (at.0 + a1.cos() * arm, at.1 - a1.sin() * arm);
        let tip = (wrist.0 + a2.cos() * hand, wrist.1 - a2.sin() * hand);
        const N: i32 = 44;
        for i in 0..=N {
            let s = i as f32 / N as f32;
            let (root, dir) = if s < 0.45 {
                let k = s / 0.45;
                ((lerp(at.0, wrist.0, k), lerp(at.1, wrist.1, k)), a1)
            } else {
                let k = (s - 0.45) / 0.55;
                ((lerp(wrist.0, tip.0, k), lerp(wrist.1, tip.1, k)), a2)
            };
            // Gaps between the fingers at the tip.
            if s > 0.55 && i % 3 == 0 {
                continue;
            }
            // Feathers leave the arm at a right angle toward the back.
            let trail = dir + FRAC_PI_2 * (3.0 * dir.sin()).tanh();
            let len =
                (5.0 + 8.0 * (PI * s.powf(0.8)).sin() + if s > 0.45 { 2.0 } else { 0.0 }) * sz;
            let end = (root.0 + trail.cos() * len, root.1 - trail.sin() * len);
            let steps = len.ceil() as i32;
            for j in 0..=steps {
                let f = j as f32 / steps.max(1) as f32;
                // Coverts near the arm, a dark shaft now and then, dark tips.
                let col = if f < 0.28 {
                    pal.of(b'e')
                } else if i % 4 == 0 || (f > 0.72 && s > 0.3) {
                    pal.of(b'd')
                } else {
                    pal.of(b'E')
                };
                let x = lerp(root.0, end.0, f).round() as i32 + off.0;
                let y = lerp(root.1, end.1, f).round() as i32 + off.1;
                pen.px(x, y, col);
            }
        }
    }

    /// The serpent: green with a pale belly, flicking a red tongue.
    fn serpent(&self, pen: &mut Pen, from: (i32, i32), b: &Bird, pal: &Palette) {
        let len = (22.0 * b.sz) as i32;
        let t = self.t;
        let (mut px, mut py) = (from.0 as f32, from.1 as f32);
        for s in 0..len {
            let f = s as f32 / len as f32;
            // In flight it streams back and down; perched it hangs and coils.
            let (sx, sy) = if b.perched {
                (0.15, 0.95)
            } else {
                (-0.7, 0.55)
            };
            let wiggle = (t * 7.0 - s as f32 * 0.7).sin() * (0.4 + 0.9 * f);
            px += sx + wiggle * -sy * 0.8;
            py += sy + wiggle * sx * 0.8;
            let col = match s % 6 {
                0..=2 => pal.of(b'N'),
                3 => pal.of(b'm'),
                _ => pal.of(b'n'),
            };
            let (x, y) = (px.round() as i32, py.round() as i32);
            pen.px(x, y, col);
            if f < 0.6 {
                pen.px(x + 1, y, pal.of(b'N'));
            }
            if s == len - 1 {
                pen.px(x, y + 1, pal.of(b'R'));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    /// The nopal grows, the eagle comes in and lands on its top penca and
    /// sits there until the end.
    #[test]
    fn flies_in_and_lands_on_the_nopal() {
        for (cols, rows) in [(80, 24), (100, 30), (200, 60)] {
            for left in [false, true] {
                let theme = Theme::classic();
                let st = Stage::new(cols, rows);
                let geo = Geo::new(st, left);
                let mut e = Aguila {
                    t: 0.0,
                    from_left: left,
                    seed: 1,
                };
                let mut c = Canvas::new(cols, rows, Color::Reset);
                let mut at_perch = None;
                while !e.done() {
                    e.update(0.1);
                    c.clear();
                    e.draw(&mut c, &theme);
                    if (e.t - 0.1 - (LANDS + 2.0)).abs() < 0.05 {
                        at_perch = Some(e.bird(&geo));
                    }
                }
                let b = at_perch.unwrap();
                assert!(b.perched);
                // On the nopal, over the rock, up on its highest penca.
                assert!((b.x - geo.perch.0).abs() < 2.5);
                assert!(
                    b.y < geo.perch.1 && b.y > geo.perch.1 - 14.0,
                    "{} vs {}",
                    b.y,
                    geo.perch.1
                );
                // Wide wings: plenty drawn at the perch.
                e.t = LANDS + 2.0;
                c.clear();
                e.draw(&mut c, &theme);
                assert!(c.cells().iter().filter(|x| x.ch != ' ').count() > 150);
            }
        }
    }
}
