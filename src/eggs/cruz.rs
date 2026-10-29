//! On the Día de la Santa Cruz (May 3) the albañiles of a half-built house
//! hoist a cross covered in paper flowers to the top, and cohetes go up.
//!
//! On May 3 every building site in Mexico puts a cross on the highest point
//! of what it is building, decked in paper flowers and streamers, and the
//! workers celebrate with food and cohetes (rockets) that whistle up and
//! pop. This draws it in original oxiper pixel art that grows with the
//! screen: a two-story house in concrete frame, its columns and slabs in
//! cal gray with varillas (steel rods) sticking out of the top and the end
//! of the slab, tabique walls laid in courses of staggered bricks and left
//! stepped where the mason stopped, a window, a ladder, a pile of bricks. It
//! rises from the ground, and three builders in hard hats stand on the new
//! slab, hauling a rope hand over hand, in heaves, while a fourth on the
//! ground steadies the cross with another. The cross (wood, covered with
//! paper flowers in rosa, amarillo, morado and verde with their leaves, and
//! papel picado streamers that flutter) rises in jerks up the open side of
//! the house, sways, swings over the slab and is planted. Then the builders
//! raise their arms, jump and wave their hats, and the cohetes go up from
//! both sides of the site and from the roof, rising with a trail of sparks,
//! popping into bursts of colors with a "¡PUM!", more and more of them, until
//! the house sinks back into the ground. All within 30 s.

use super::pen::{fill, hash, trace};
use super::santacruz::{Palette, fade};
use super::{Egg, Info, paint, pixel};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "cruz",
    start,
    fiesta: Kind::SantaCruz,
    when: super::any_day,
};

/// When things happen, in seconds: the house is risen, the builders start
/// hauling (in `HEAVES` heaves) and the cross is up, it is planted, the last
/// cohetes go up, the house sinks back and it is over.
const RISEN: f32 = 3.0;
const HOIST: (f32, f32) = (4.2, 12.2);
const HEAVES: f32 = 8.0;
const PLANTED: f32 = 13.2;
const LAST_ROCKET: f32 = 21.0;
const SINKING: f32 = 23.2;
const END: f32 = 25.8;

/// Cohetes: how many can be in the air, how many sparks a burst has, how
/// long the burst lasts and the colors of each kind of burst.
const ROCKETS: usize = 12;
const SPARKS: usize = 20;
const BURST: f32 = 1.4;
const BURSTS: [[u8; 3]; 5] = [*b"YOW", *b"PMW", *b"GYW", *b"ROY", *b"BWP"];
const SOUNDS: [&str; 3] = ["¡PUM!", "¡PAM!", "¡PUUM!"];

/// The proportions of the house, as shares of its height: the first floor's
/// top, the slab over it, the roof slab, and how far across the roof slab
/// is laid (the rest is still open to the sky).
const FLOOR: f32 = 0.42;
const SLAB: f32 = 0.48;
const ROOF: (f32, f32) = (0.9, 0.96);
const LAID: f32 = 0.66;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Cruz {
        t: 0.0,
        w: w as i32,
        h: h as i32,
        seed: rng.i32(0..10_000),
        rockets: [Rocket::default(); ROCKETS],
        next: 0,
        launch_in: 1.5,
        rng: fastrand::Rng::with_seed(rng.u64(..)),
    })
}

#[derive(Clone, Copy, Default)]
struct Rocket {
    /// Where it lifts off, in pixels, and how it drifts.
    x: f32,
    y: f32,
    vx: f32,
    /// The height of its burst, and the second it left.
    top: f32,
    born: f32,
    /// How long it rises, in seconds, and what picks its burst.
    rise: f32,
    seed: i32,
    live: bool,
}

struct Cruz {
    t: f32,
    w: i32,
    h: i32,
    /// Shuffles the bricks so no two houses are laid alike.
    seed: i32,
    rockets: [Rocket; ROCKETS],
    next: usize,
    launch_in: f32,
    rng: fastrand::Rng,
}

/// How a person stands: the letters of the shirt and of the hat, where the
/// hands are, and how many pixels the body leans toward them.
struct Pose {
    look: (u8, u8),
    hands: [(f32, f32); 2],
    lean: f32,
}

/// Where the house stands on this screen.
struct Geo {
    /// Its left edge, and its width and height (to the top of the roof
    /// slab), in pixels, and a unit for the people and the cross.
    x: f32,
    w: f32,
    h: f32,
    u: f32,
    /// The pixel row under the ground, which is below the screen while the
    /// house is rising or sinking.
    base: i32,
}

/// The colors a frame needs, found once.
struct Cols {
    cal: Color,
    cal_dark: Color,
    steel: Color,
    tie: Color,
    brick: Color,
    brick_dark: Color,
    wood: Color,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

impl Cruz {
    /// How far the house has risen: 0 under the ground, 1 standing.
    fn rise(&self) -> f32 {
        let up = ease_out(self.t / RISEN);
        let down = ((self.t - SINKING) / (END - SINKING)).clamp(0.0, 1.0);
        (up - down * down).clamp(0.0, 1.0)
    }

    fn geo(&self, w: i32, rows: i32) -> Geo {
        let screen = rows * 2;
        let h = (screen as f32 * 0.5)
            .min(w as f32 * 0.58)
            .clamp(18.0, 100.0);
        Geo {
            x: ((w as f32 - h * 1.35) / 2.0).round(),
            w: h * 1.35,
            h,
            u: (h / 30.0).max(0.8),
            base: screen + ((1.0 - self.rise()) * (1.5 * h + 12.0)) as i32,
        }
    }

    /// The cross's height, and where the roof is (pixel row of the slab's
    /// top).
    fn cross_size(g: &Geo) -> f32 {
        (g.h * 0.36).max(12.0)
    }

    fn roof_y(g: &Geo) -> f32 {
        g.base as f32 - ROOF.1 * g.h
    }

    /// How far up the hauling has got, 0 to 1: the cross rises in jerks, one
    /// heave after another, each easing out.
    fn hauled(&self) -> f32 {
        let s = ((self.t - HOIST.0) / (HOIST.1 - HOIST.0) * HEAVES).clamp(0.0, HEAVES);
        ((s.floor() + ease_out(s.fract())) / HEAVES).min(1.0)
    }

    /// Where the cross is: the middle of its foot, and the pixel row of it.
    fn cross_at(&self, g: &Geo) -> (f32, f32) {
        let p = self.hauled();
        let roof = Self::roof_y(g);
        let hc = Self::cross_size(g);
        let start_x = g.x + 0.8 * g.w;
        let plant_x = g.x + 0.62 * g.w;
        // It sways on the rope, less and less as it nears the builders.
        let sway =
            ((self.t * 2.1).sin() * 1.2 + (self.t * 3.7).sin() * 0.5) * g.u * (1.0 - 0.8 * p);
        // At the top it is swung over the slab, and lowered into its place.
        let over = ((p - 0.86) / 0.14).clamp(0.0, 1.0);
        let over = over * over * (3.0 - 2.0 * over);
        let x = lerp(start_x + sway, plant_x, over);
        let above = roof - 0.06 * hc;
        let y = lerp(g.base as f32, above, p);
        let lower = ((self.t - HOIST.1) / (PLANTED - HOIST.1 - 0.2)).clamp(0.0, 1.0);
        (x, lerp(y, roof, lower * lower))
    }

    /// The color of the house at pixel (`x`, `y`), or `None` for the sky and
    /// the openings.
    fn house(&self, g: &Geo, k: &Cols, x: i32, y: i32) -> Option<Color> {
        let dx = x as f32 + 0.5 - g.x;
        let dy = (g.base - 1 - y) as f32 + 0.5;
        let (w, h) = (g.w, g.h);
        if dx < 0.0 || dx >= w || dy < 0.0 || dy > h * 1.14 {
            return None;
        }
        let cw = (w * 0.07).max(2.0);
        let columns = [(0.0, cw), ((w - cw) / 2.0, (w + cw) / 2.0), (w - cw, w)];
        let (floor, slab, roof0, roof1) = (FLOOR * h, SLAB * h, ROOF.0 * h, ROOF.1 * h);
        // The columns, with the varillas sticking out of their tops.
        for (i, &(a, b)) in columns.iter().enumerate() {
            if dx < a || dx >= b {
                continue;
            }
            if dy <= roof1 {
                let edge = dx >= b - 1.0;
                let tie = (dy / (h * 0.06).max(2.0)) as i32 % 2 == 0 && dx < a + 1.0;
                return Some(if edge || tie { k.cal_dark } else { k.cal });
            }
            let top = h * (1.1 + 0.02 * hash(i as i32 + self.seed, 3));
            let rel = (dx - a) / (b - a);
            let rod = (0.1..0.38).contains(&rel) || (0.62..0.9).contains(&rel);
            return (dy <= top && rod).then_some(if dy > top - 1.5 { k.tie } else { k.steel });
        }
        // The slab over the first floor, and the roof slab, laid across part
        // of the house, on wooden formwork, its rods sticking out of its end.
        if (floor..slab).contains(&dy) {
            return Some(if dy < floor + 1.0 { k.cal_dark } else { k.cal });
        }
        if (roof0..roof1).contains(&dy) && dx < LAID * w {
            return Some(if dy < roof0 + 1.2 { k.wood } else { k.cal });
        }
        if dx >= LAID * w && dx < (LAID + 0.07) * w {
            for rod in [0.915, 0.94] {
                if (dy - rod * h).abs() < 0.6 {
                    return Some(k.steel);
                }
            }
        }
        // The walls: tabique in courses, the bays between the columns.
        let (bay_a, bay_b) = ((columns[0].1, columns[1].0), (columns[1].1, columns[2].0));
        let rel = |bay: (f32, f32)| ((dx - bay.0) / (bay.1 - bay.0)).clamp(0.0, 1.0);
        let course = (h * 0.04).max(1.6);
        let stepped = |top: f32| (top / course).floor() * course;
        let in_bay = |bay: (f32, f32)| dx >= bay.0 && dx < bay.1;
        let laid = if in_bay(bay_a) {
            // The first floor is up to the slab, with a window; the second is
            // stepped down where the mason stopped.
            let window = (dx - (bay_a.0 + bay_a.1) / 2.0).abs() < 0.1 * w
                && (0.17 * h..0.33 * h).contains(&dy);
            if dy < floor {
                if window {
                    return None;
                }
                true
            } else {
                dy >= slab && dy < stepped(slab + (roof0 - slab) * lerp(0.7, 0.25, rel(bay_a)))
            }
        } else if in_bay(bay_b) {
            dy < stepped(floor * lerp(0.95, 0.1, rel(bay_b)))
        } else {
            false
        };
        if !laid {
            return None;
        }
        // One brick: a mortar line under each course and between the bricks,
        // every other course shifted half a brick.
        let width = (h * 0.1).max(3.2);
        let c = (dy / course).floor();
        let shift = if (c as i32) % 2 == 0 {
            0.0
        } else {
            width / 2.0
        };
        let (mortar_y, mortar_x) = (dy - c * course, (dx + shift) % width);
        if mortar_y < 1.0 || mortar_x < 1.0 {
            return Some(k.cal_dark);
        }
        let n = hash(((dx + shift) / width) as i32 + self.seed, c as i32);
        Some(if n > 0.85 {
            k.brick_dark
        } else {
            k.brick.scale(0.9 + 0.2 * n)
        })
    }

    /// A person: feet at (`x`, `feet`), shirt and hat colored, the hands
    /// where they are told, the body leaning `lean` pixels toward them.
    fn person(c: &mut Canvas, pal: &Palette, u: f32, at: (f32, f32), pose: Pose) {
        let (x, feet) = at;
        let Pose { look, hands, lean } = pose;
        let r = |v: f32| v.round() as i32;
        // Legs, shirt, head, and a hat with a brim.
        let hip = feet - 4.0 * u;
        let shoulder = (x + lean * 0.5, feet - 9.0 * u);
        fill(
            c,
            (r(x - 1.7 * u), r(hip)),
            (r(1.5 * u).max(1), r(4.0 * u)),
            pal.of(b'n'),
        );
        fill(
            c,
            (r(x + 0.2 * u), r(hip)),
            (r(1.5 * u).max(1), r(4.0 * u)),
            pal.of(b'n'),
        );
        fill(
            c,
            (r(x - 2.0 * u + lean * 0.4), r(shoulder.1)),
            (r(4.0 * u), r(5.2 * u)),
            pal.of(look.0),
        );
        let head = (x + lean, feet - 11.4 * u);
        fill(
            c,
            (r(head.0 - 1.4 * u), r(head.1 - 1.4 * u)),
            (r(2.8 * u), r(2.8 * u)),
            pal.of(b'K'),
        );
        fill(
            c,
            (r(head.0 - 1.9 * u), r(head.1 - 2.6 * u)),
            (r(3.8 * u), r(1.4 * u).max(1)),
            pal.of(look.1),
        );
        fill(
            c,
            (r(head.0 - 2.4 * u), r(head.1 - 1.4 * u)),
            (r(4.8 * u), r(0.7 * u).max(1)),
            pal.of(look.1),
        );
        // The arms, from the shoulders to the hands.
        for hand in hands {
            let thick = r(u).max(1);
            trace(
                (r(shoulder.0), r(shoulder.1)),
                (r(hand.0), r(hand.1)),
                |px, py| {
                    fill(c, (px, py), (thick, thick), pal.of(look.0));
                },
            );
            fill(c, (r(hand.0), r(hand.1)), (thick, thick), pal.of(b'K'));
        }
    }

    /// The cross: a wooden one covered in paper flowers, with papel picado
    /// streamers from its arms that flutter.
    fn cross(&self, c: &mut Canvas, pal: &Palette, g: &Geo, (cx, bottom): (f32, f32)) {
        let hc = Self::cross_size(g);
        let bar = (hc * 0.22).max(3.0);
        let (arm_y, arm_half) = (bottom - hc * 0.8, hc * 0.38);
        let cell = (bar / 1.5).max(2.0) as i32;
        let flower = |px: i32, py: i32| {
            let (fx, fy) = (px.div_euclid(cell), py.div_euclid(cell));
            let n = hash(fx + self.seed, fy);
            // Each cell is a flower, its center bright, some a leaf.
            let center = px.rem_euclid(cell) == cell / 2 && py.rem_euclid(cell) == cell / 2;
            if center {
                return pal.of(if n > 0.5 { b'Y' } else { b'W' });
            }
            pal.of(match (n * 8.0) as i32 {
                0 | 1 => b'P',
                2 | 3 => b'Y',
                4 | 5 => b'M',
                6 => b'P',
                _ => b'G',
            })
        };
        let (top, left) = ((bottom - hc) as i32, (cx - bar / 2.0) as i32);
        for py in top..bottom as i32 {
            for px in left..=(cx + bar / 2.0) as i32 {
                pixel(c, px, py, flower(px, py));
            }
        }
        for py in arm_y as i32..=(arm_y + bar) as i32 {
            for px in (cx - arm_half) as i32..=(cx + arm_half) as i32 {
                pixel(c, px, py, flower(px, py));
            }
        }
        // The foot, plain wood.
        fill(
            c,
            (left, bottom as i32 - 1),
            ((bar as i32).max(2), 1),
            pal.of(b'D'),
        );
        // Streamers hanging from the arms' ends, in the colors of the flowers.
        for (side, end) in [(-1.0, cx - arm_half), (1.0, cx + arm_half)] {
            for step in 0..(hc * 0.5) as i32 {
                let wave =
                    (self.t * 5.0 + step as f32 * 0.5 + side).sin() * (0.6 + step as f32 * 0.12);
                let color = pal.of((*b"PYGM")[(step / 2) as usize % 4]);
                pixel(c, (end + wave) as i32, (arm_y + bar) as i32 + step, color);
            }
        }
    }

    /// The builders on the roof, the one on the ground, the ropes and the
    /// ladder, the bricks and the sack: what is around the house.
    fn site(&self, c: &mut Canvas, pal: &Palette, g: &Geo, cross: (f32, f32)) {
        let u = g.u;
        let (wood, roof) = (pal.of(b'D'), Self::roof_y(g));
        let r = |v: f32| v.round() as i32;
        // A ladder against the first floor, with its rungs.
        let (lx, top) = (g.x - 0.1 * g.w, g.base as f32 - FLOOR * g.h);
        for dx in [0.0, 2.2 * u] {
            trace(
                (r(lx + dx), g.base),
                (r(g.x - 0.02 * g.w + dx), r(top)),
                |px, py| pixel(c, px, py, wood),
            );
        }
        for k in 1..(FLOOR * g.h / (3.0 * u)) as i32 {
            let y = g.base as f32 - k as f32 * 3.0 * u;
            let f = (g.base as f32 - y) / (g.base as f32 - top);
            let x0 = lx + (g.x - 0.02 * g.w - lx) * f;
            trace((r(x0), r(y)), (r(x0 + 2.2 * u), r(y)), |px, py| {
                pixel(c, px, py, wood)
            });
        }
        // A pile of bricks, three courses high, and a sack of cement.
        let pile = g.x + 1.06 * g.w;
        for course in 0..3 {
            let (bw, bh) = (r(2.6 * u).max(2), r(1.1 * u).max(1));
            for k in 0..3 - course {
                let x = pile + (k as f32 + course as f32 * 0.5) * (bw as f32 + 1.0);
                let y = g.base - (course + 1) * (bh + 1);
                fill(c, (r(x), y), (bw, bh), pal.of(b'T'));
                fill(c, (r(x), y + bh - 1), (bw, 1), pal.of(b't'));
            }
        }
        fill(
            c,
            (r(g.x - 0.2 * g.w), g.base - r(2.4 * u)),
            (r(3.6 * u), r(2.4 * u)),
            pal.of(b'L'),
        );
        fill(
            c,
            (r(g.x - 0.2 * g.w), g.base - r(1.0 * u)),
            (r(3.6 * u), r(1.0 * u)),
            pal.of(b'l'),
        );

        // The builders on the new slab; they haul while the cross goes up
        // and cheer once it is planted.
        let hauling = self.t < PLANTED;
        let s = ((self.t - HOIST.0) / (HOIST.1 - HOIST.0) * HEAVES).clamp(0.0, HEAVES);
        let heave = (s.fract() * std::f32::consts::PI).sin();
        let cheer = ((self.t - PLANTED) * 4.0).sin();
        let shirts = [(b'R', b'Y'), (b'B', b'O'), (b'G', b'Y')];
        let post = [0.5, 0.36, 0.22];
        let mut hand_first = (0.0, 0.0);
        for (i, &fx) in post.iter().enumerate() {
            let x = g.x + fx * g.w;
            if hauling {
                // Leaning back on each heave, hands on the rope.
                let back = if i == 0 { 1.0 } else { 0.6 };
                let hands = [
                    (x + 3.2 * u - heave * 1.5 * u * back, roof - 8.2 * u),
                    (x + 4.2 * u - heave * 1.5 * u * back, roof - 7.6 * u),
                ];
                if i == 0 {
                    hand_first = hands[0];
                }
                let pose = Pose {
                    look: shirts[i],
                    hands,
                    lean: -heave * 1.4 * u,
                };
                Self::person(c, pal, u, (x, roof), pose);
            } else {
                // Arms up, and a little jump of joy.
                let hop = (cheer * (1.0 + i as f32 * 0.3)).abs() * 1.2 * u;
                let wave = cheer * 1.0 * u;
                let hands = [
                    (x - 3.0 * u - wave, roof - hop - 15.5 * u),
                    (x + 3.0 * u + wave, roof - hop - 15.0 * u),
                ];
                let pose = Pose {
                    look: shirts[i],
                    hands,
                    lean: 0.0,
                };
                Self::person(c, pal, u, (x, roof - hop), pose);
            }
        }
        // The one on the ground, who steadies the cross, and then cheers too.
        let gx = g.x + 0.93 * g.w;
        let (gdx, gy) = (gx - 2.0 * u, g.base as f32);
        if hauling {
            let hands = [(gdx - 1.0 * u, gy - 7.0 * u), (gdx, gy - 6.4 * u)];
            let pose = Pose {
                look: (b'O', b'W'),
                hands,
                lean: -u,
            };
            Self::person(c, pal, u, (gx, gy), pose);
            // His rope runs up to the cross, slack and taut as it sways.
            let to = (cross.0, cross.1 - Self::cross_size(g) * 0.3);
            trace(
                (r(gdx - 1.0 * u), r(gy - 7.0 * u)),
                (r(to.0), r(to.1)),
                |px, py| pixel(c, px, py, pal.of(b'l')),
            );
            // And the haulers' rope, from their hands to the cross's top.
            let top = (cross.0, cross.1 - Self::cross_size(g));
            trace(
                (r(top.0), r(top.1)),
                (r(hand_first.0), r(hand_first.1)),
                |px, py| pixel(c, px, py, pal.of(b'M')),
            );
            for (a, b) in post.windows(2).map(|p| (p[0], p[1])) {
                let (xa, xb) = (g.x + a * g.w + 3.2 * u, g.x + b * g.w + 3.2 * u);
                trace(
                    (r(xa), r(roof - 8.0 * u)),
                    (r(xb), r(roof - 8.0 * u)),
                    |px, py| pixel(c, px, py, pal.of(b'M')),
                );
            }
        } else {
            let hop = (cheer * 1.3).abs() * 1.2 * u;
            let hands = [
                (gx - 3.0 * u, gy - hop - 14.0 * u),
                (gx + 3.0 * u, gy - hop - 14.5 * u),
            ];
            let pose = Pose {
                look: (b'O', b'W'),
                hands,
                lean: 0.0,
            };
            Self::person(c, pal, u, (gx, gy - hop), pose);
        }
    }

    /// A cohete: while it rises, a trail of sparks behind it; at the top, a
    /// flash and a burst that spreads and falls, and its sound.
    fn rocket(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, r: &Rocket) {
        let age = self.t - r.born;
        if age < 0.0 {
            return;
        }
        let at = |a: f32| {
            let f = (a / r.rise).clamp(0.0, 1.0).powf(1.25);
            (
                r.x + r.vx * a + (a * 11.0).sin() * 0.7,
                r.y - (r.y - r.top) * f,
            )
        };
        let screen = c.h() as f32 * 2.0;
        if age < r.rise {
            // The trail: white-hot at the head, then gold, then ember, thinning.
            for k in 0..9 {
                let (x, y) = at((age - k as f32 * 0.03).max(0.0));
                let ch = (*b"WWYYOOOOO")[k];
                let color = fade(theme, pal.of(ch), k as f32 / 10.0);
                pixel(c, x as i32, y as i32 + k as i32 / 2, color);
            }
            return;
        }
        let ba = age - r.rise;
        if ba > BURST {
            return;
        }
        let (bx, by) = at(r.rise);
        let kind = BURSTS[r.seed.unsigned_abs() as usize % BURSTS.len()];
        let reach = (screen * 0.2).max(8.0);
        // A flash, then the sparks flying out and falling.
        if ba < 0.12 {
            for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                pixel(c, bx as i32 + dx, by as i32 + dy, pal.of(b'W'));
            }
        }
        let spread = (1.0 - (-3.2 * ba).exp()) / 3.2;
        for j in 0..SPARKS {
            let ang = TAU * j as f32 / SPARKS as f32 + hash(r.seed, j as i32) * 0.3;
            let speed = reach * (0.55 + 0.45 * hash(r.seed, j as i32 + 40));
            let (x, y) = (
                bx + ang.cos() * speed * spread * 2.0,
                by + ang.sin() * speed * spread * 1.6 + 14.0 * ba * ba,
            );
            let color = fade(theme, pal.of(kind[j % 3]), (ba / BURST).powf(1.6));
            pixel(c, x as i32, y as i32, color);
        }
        // The sound, rising a little, for some of them.
        if ba < 0.5 && hash(r.seed, 99) > 0.4 {
            let sound = SOUNDS[r.seed.unsigned_abs() as usize % SOUNDS.len()];
            let col = fade(theme, pal.of(b'Y'), ba / 0.5 * 0.8);
            let row = by as i32 / 2 - 2 - (ba * 3.0) as i32;
            c.text_attr(
                bx as i32 - sound.chars().count() as i32 / 2,
                row,
                sound,
                col,
                BOLD,
            );
        }
    }
}

impl Egg for Cruz {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        for r in self.rockets.iter_mut().filter(|r| r.live) {
            if self.t - r.born > r.rise + BURST + 0.1 {
                r.live = false;
            }
        }
        self.launch_in -= dt;
        while self.launch_in <= 0.0 && self.t < LAST_ROCKET {
            // A few before the cross is up, many once it is planted.
            let pace = if self.t < PLANTED { 2.2 } else { 0.6 };
            self.launch_in += pace * (0.6 + 0.8 * self.rng.f32());
            let g = self.geo(self.w, self.h);
            let (screen, ground) = (self.h as f32 * 2.0, self.h as f32 * 2.0);
            // From the ground at either side of the site, or off the roof.
            let (x, y) = if self.rng.u8(0..4) == 0 {
                (g.x + g.w * (0.1 + 0.3 * self.rng.f32()), Self::roof_y(&g))
            } else if self.rng.bool() {
                (self.rng.f32() * (g.x - 3.0).max(2.0), ground)
            } else {
                let right = g.x + g.w * 1.3;
                (
                    right + self.rng.f32() * (self.w as f32 - right).max(2.0),
                    ground,
                )
            };
            let height = screen * (0.4 + 0.45 * self.rng.f32());
            self.rockets[self.next] = Rocket {
                x: x.clamp(0.0, (self.w as f32 - 1.0).max(0.0)),
                y,
                vx: (self.rng.f32() - 0.5) * 3.0,
                top: (y - height).max(6.0),
                born: self.t,
                rise: 1.1 + 0.6 * self.rng.f32(),
                seed: self.rng.i32(0..100_000),
                live: true,
            };
            self.next = (self.next + 1) % ROCKETS;
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::santa_cruz(theme);
        let (w, rows) = (c.w() as i32, c.h() as i32);
        let g = self.geo(w, rows);
        let k = Cols {
            cal: pal.of(b'L'),
            cal_dark: pal.of(b'l'),
            steel: pal.of(b'V'),
            tie: pal.of(b'v'),
            brick: pal.of(b'T'),
            brick_dark: pal.of(b't'),
            wood: pal.of(b'D'),
        };
        // The house, cell by cell.
        let (x0, x1) = ((g.x as i32 - 1).max(0), ((g.x + g.w) as i32 + 1).min(w - 1));
        let top_row = ((g.base as f32 - g.h * 1.14) / 2.0).floor() as i32;
        for row in top_row.max(0)..rows {
            for col in x0..=x1 {
                let up = self.house(&g, &k, col, row * 2);
                let down = self.house(&g, &k, col, row * 2 + 1);
                paint(c, col, row, up, down);
            }
        }
        let cross = self.cross_at(&g);
        self.site(c, &pal, &g, cross);
        if self.rise() > 0.0 {
            self.cross(c, &pal, &g, cross);
        }
        for r in self.rockets.iter().filter(|r| r.live) {
            self.rocket(c, theme, &pal, r);
        }
    }

    fn done(&self) -> bool {
        self.t > END || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cross, builders and house take their places: the cross is hauled up
    /// in jerks (never down), ends planted on the roof, and the house is
    /// risen by then.
    #[test]
    fn the_cross_goes_up_and_is_planted() {
        let mut e = Cruz {
            t: 0.0,
            w: 100,
            h: 30,
            seed: 1,
            rockets: [Rocket::default(); ROCKETS],
            next: 0,
            launch_in: 1.5,
            rng: fastrand::Rng::with_seed(2),
        };
        let g = e.geo(100, 30);
        let mut last = f32::MAX;
        let mut moved = 0;
        while e.t < PLANTED + 0.5 {
            e.t += 0.05;
            let (_, y) = e.cross_at(&g);
            // Up all the way to the top, then lowered into its place.
            if e.t < HOIST.1 {
                assert!(
                    y <= last + 0.001,
                    "the cross fell at {}: {y} after {last}",
                    e.t
                );
            }
            moved += usize::from(y < last - 0.5);
            last = y;
        }
        assert!(moved > 5, "{moved} moves");
        let (x, y) = e.cross_at(&g);
        assert!((y - Cruz::roof_y(&g)).abs() < 0.01, "planted at {y}");
        assert!(x > g.x && x < g.x + g.w, "{x}");
        assert!(e.rise() > 0.99);
    }

    /// The house stands, the cross is covered in flowers of its colors,
    /// the builders have hard hats, and the cohetes burst.
    #[test]
    fn a_site_with_flowers_and_cohetes() {
        let theme = Theme::classic();
        for seed in 0..3 {
            let mut e = start(120, 36, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(120, 36, Color::Reset);
            let mut sounds = 0;
            let mut seen = [false; 4];
            for _ in 0..270 {
                e.update(0.1);
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                sounds += SOUNDS.iter().filter(|s| text.contains(*s)).count();
                let colors: Vec<Color> = c.cells().iter().map(|cell| cell.fg).collect();
                for (i, hex) in [0xe4007c, 0xffd200, 0x7028a0, 0x2fb84a]
                    .into_iter()
                    .enumerate()
                {
                    seen[i] |= colors.contains(&crate::color::rgb(hex));
                }
            }
            assert!(seen.iter().all(|&s| s), "{seen:?}");
            assert!(sounds > 0, "no ¡PUM! on seed {seed}");
            assert!(e.done());
        }
    }
}
