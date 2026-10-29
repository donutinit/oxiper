//! How a four-legged animal is put together on screen.
//!
//! Each animal's body (and the tiger's head) is drawn by hand; its legs and
//! tail are not, so that one drawing walks, trots, gallops, crouches,
//! pounces, lies down, rears up and dangles. A leg is two segments from the
//! hip to the paw, bent by inverse kinematics: given where the foot should
//! be for the gait and the moment in its stride, the middle joint falls
//! where both segments reach it, pointing forward for front legs and back
//! for hind legs. The parts are composed as palette letters on a scratch
//! [`Figure`], and the figure is then placed in the world, mirrored and
//! tilted by three shears that move whole pixels, none lost or doubled.

use std::f32::consts::PI;

use super::art::Sprite;

/// Size of the scratch canvas a figure is composed on.
const FIG_W: i32 = 112;
const FIG_H: i32 = 72;
/// Where body-space (0, 0) falls on the scratch canvas.
const ORG_X: i32 = 42;
const ORG_Y: i32 = 30;

/// One leg: where it leaves the body and how it is made.
#[derive(Clone, Copy, Debug)]
pub struct LegSpec {
    /// The joint where the leg leaves the body, in body pixels.
    pub hip: (f32, f32),
    /// Lengths of the upper and lower segments.
    pub upper: f32,
    pub lower: f32,
    /// Widths at the top and at the paw.
    pub w_up: f32,
    pub w_lo: f32,
    /// Front legs bend forward at the wrist; hind legs back at the hock.
    pub fore: bool,
}

/// A tail: a chain of one-pixel links from the root.
#[derive(Clone, Copy, Debug)]
pub struct TailSpec {
    pub root: (f32, f32),
    pub links: usize,
    /// Width at the root and at the tip.
    pub w0: f32,
    pub w1: f32,
    /// Coat, a ring every few links (if striped), and the tip's color and length.
    pub coat: u8,
    pub ring: Option<u8>,
    pub tip: u8,
    pub tip_len: usize,
}

/// How an animal is built, for everything not drawn by hand.
#[derive(Clone, Copy, Debug)]
pub struct Build {
    pub hind: LegSpec,
    pub fore: LegSpec,
    /// How far the far legs show beside the near ones.
    pub far_dx: f32,
    /// The row its paws stand on, in body pixels.
    pub ground: f32,
    /// Length of a walking stride and how high a paw lifts.
    pub stride: f32,
    pub lift: f32,
    /// Leg letters: near coat, far coat; stripes near and far, if any;
    /// paws or hooves near and far, and how wide a paw is.
    pub leg: (u8, u8),
    pub stripe: Option<(u8, u8)>,
    pub paw: (u8, u8),
    pub paw_w: i32,
    pub tail: Option<TailSpec>,
}

/// The ways of putting feet down.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gait {
    /// Feet one after another: hind, fore, other hind, other fore.
    Walk,
    /// Diagonal pairs together.
    Trot,
    /// The rotary gallop of cats and dogs, with a flight between strides.
    Gallop,
    /// The bound of a deer: hind feet together, then fore feet.
    Bound,
    /// Paws working under water.
    Paddle,
}

impl Gait {
    /// Fraction of the stride a foot is down, and each foot's place in the
    /// stride: near hind, near fore, far hind, far fore.
    fn timing(self) -> (f32, [f32; 4]) {
        match self {
            Gait::Walk => (0.62, [0.0, 0.25, 0.5, 0.75]),
            Gait::Trot => (0.5, [0.0, 0.5, 0.5, 0.0]),
            Gait::Gallop => (0.34, [0.0, 0.58, 0.1, 0.48]),
            Gait::Bound => (0.3, [0.0, 0.5, 0.06, 0.56]),
            Gait::Paddle => (0.5, [0.0, 0.5, 0.5, 0.0]),
        }
    }

    /// How long a stride is and how high a paw lifts, against a walk's.
    fn reach(self) -> (f32, f32) {
        match self {
            Gait::Walk => (1.0, 1.0),
            Gait::Trot => (1.25, 1.2),
            Gait::Gallop => (1.9, 1.6),
            Gait::Bound => (1.8, 1.8),
            Gait::Paddle => (0.8, 1.4),
        }
    }
}

/// How far a rearing animal is turned, nose up, about its hind hip.
pub const REAR: f32 = -0.72;

/// How far a sitting animal is turned, nose up, about its hind hip.
pub const SIT: f32 = -0.5;

/// What the legs are doing.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Legs {
    /// Standing square.
    Stand,
    /// Moving in a gait, `phase` in `[0, 1)` through the stride.
    Gait(Gait, f32),
    /// In the air: fore legs reaching forward, hind legs thrown back, `k`
    /// from tucked (0) to fully stretched (1).
    Reach(f32),
    /// Lying down: fore legs out in front, hind legs folded under.
    Fold,
    /// Hanging in the air, kicking, `phase` for the kicks.
    Dangle(f32),
    /// Reared up against something, the body turned by [`REAR`]: fore paws
    /// up ahead of the shoulders, raking up and down with `phase`.
    Rear(f32),
    /// Sitting on its haunches, the body turned by [`SIT`] times `k` (0 standing, 1 sat): the
    /// fore paws straight down from the shoulders, the hind paws folded forward under it.
    Sit(f32),
}

/// How the tail is carried.
#[derive(Clone, Copy, Debug)]
pub struct TailPose {
    /// Angle where it leaves the body, radians, 0 pointing forward and
    /// `PI` straight back; more than `PI` droops.
    pub base: f32,
    /// Bend added at each link.
    pub curl: f32,
    /// A wave travelling down it: how strong and where it is.
    pub sway: f32,
    pub phase: f32,
}

/// A figure being composed: a palette letter per pixel, 0 for nothing.
pub struct Figure {
    px: Vec<u8>,
    /// What has been drawn: the box, in canvas pixels, inclusive.
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Figure {
    pub fn new() -> Figure {
        Figure {
            px: vec![0; (FIG_W * FIG_H) as usize],
            x0: FIG_W,
            y0: FIG_H,
            x1: -1,
            y1: -1,
        }
    }

    /// Forgets the last figure, touching only what it covered.
    pub fn clear(&mut self) {
        if self.x1 >= self.x0 {
            for y in self.y0..=self.y1 {
                let row = (y * FIG_W) as usize;
                self.px[row + self.x0 as usize..=row + self.x1 as usize].fill(0);
            }
        }
        (self.x0, self.y0, self.x1, self.y1) = (FIG_W, FIG_H, -1, -1);
    }

    /// Sets the body pixel `(x, y)` to letter `ch`.
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, ch: u8) {
        let (cx, cy) = (x + ORG_X, y + ORG_Y);
        if cx >= 0 && cy >= 0 && cx < FIG_W && cy < FIG_H {
            self.px[(cy * FIG_W + cx) as usize] = ch;
            self.x0 = self.x0.min(cx);
            self.x1 = self.x1.max(cx);
            self.y0 = self.y0.min(cy);
            self.y1 = self.y1.max(cy);
        }
    }

    /// The letter at body pixel `(x, y)`, 0 if nothing.
    #[inline]
    pub fn get(&self, x: i32, y: i32) -> u8 {
        let (cx, cy) = (x + ORG_X, y + ORG_Y);
        if cx >= 0 && cy >= 0 && cx < FIG_W && cy < FIG_H {
            self.px[(cy * FIG_W + cx) as usize]
        } else {
            0
        }
    }

    /// Draws a sprite with its top left corner at body pixel `(x, y)`.
    pub fn stamp(&mut self, s: Sprite, x: i32, y: i32) {
        for (r, row) in s.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                if ch != b'.' {
                    self.set(x + c as i32, y + r as i32, ch);
                }
            }
        }
    }

    /// Draws a sprite with its top left corner at body pixel `(x, y)`,
    /// turned by `angle` (clockwise on screen) about its own pixel `pivot`.
    pub fn stamp_turned(&mut self, s: Sprite, x: i32, y: i32, pivot: (i32, i32), angle: f32) {
        if angle.abs() < 0.02 {
            self.stamp(s, x, y);
            return;
        }
        let sh = Shears::new(angle);
        for (r, row) in s.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                if ch != b'.' {
                    let (dx, dy) = sh.turn(c as i32 - pivot.0, r as i32 - pivot.1);
                    self.set(x + pivot.0 + dx, y + pivot.1 + dy, ch);
                }
            }
        }
    }

    /// Fills the pixels within `w / 2` of the segment from `a` to `b` (the
    /// width easing from `wa` to `wb`), with `stripe` across it where `bands`
    /// say, `ch` elsewhere. Only pixels still empty are painted when `under`.
    #[allow(clippy::too_many_arguments)]
    fn limb(
        &mut self,
        a: (f32, f32),
        b: (f32, f32),
        wa: f32,
        wb: f32,
        ch: u8,
        stripe: Option<(u8, &[(f32, f32)])>,
        under: bool,
    ) {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let len2 = (dx * dx + dy * dy).max(1e-4);
        let r = wa.max(wb) / 2.0 + 1.0;
        let (lx, hx) = (a.0.min(b.0) - r, a.0.max(b.0) + r);
        let (ly, hy) = (a.1.min(b.1) - r, a.1.max(b.1) + r);
        for y in ly.floor() as i32..=hy.ceil() as i32 {
            for x in lx.floor() as i32..=hx.ceil() as i32 {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let t = (((px - a.0) * dx + (py - a.1) * dy) / len2).clamp(0.0, 1.0);
                let (qx, qy) = (a.0 + dx * t - px, a.1 + dy * t - py);
                let half = (wa + (wb - wa) * t) / 2.0;
                if qx * qx + qy * qy <= half * half + 0.05 {
                    if under && self.get(x, y) != 0 {
                        continue;
                    }
                    let c = match stripe {
                        Some((s, bands)) if bands.iter().any(|&(lo, hi)| t >= lo && t < hi) => s,
                        _ => ch,
                    };
                    self.set(x, y, c);
                }
            }
        }
    }

    /// The letters that have been drawn, with their body pixel positions.
    pub fn each(&self, mut f: impl FnMut(i32, i32, u8)) {
        if self.x1 < self.x0 {
            return;
        }
        for cy in self.y0..=self.y1 {
            let row = (cy * FIG_W) as usize;
            for cx in self.x0..=self.x1 {
                let ch = self.px[row + cx as usize];
                if ch != 0 {
                    f(cx - ORG_X, cy - ORG_Y, ch);
                }
            }
        }
    }

    /// Draws the far legs: the ones on the other side of the body, which
    /// go down first and show a little beside the near ones.
    pub fn far_legs(&mut self, b: &Build, legs: Legs, drop: f32) {
        self.legs(b, legs, drop, true);
    }

    /// Draws the near legs, over the body.
    pub fn near_legs(&mut self, b: &Build, legs: Legs, drop: f32) {
        self.legs(b, legs, drop, false);
    }

    /// Draws one side's legs. `drop` is how far the body sits below its
    /// standing height (negative: above it), in body pixels: the figure is
    /// placed that much lower, and standing feet stay on the ground.
    fn legs(&mut self, b: &Build, legs: Legs, drop: f32, far: bool) {
        let side = if far { 1 } else { 0 };
        for (i, spec) in [b.hind, b.fore].iter().enumerate() {
            let slot = i + 2 * side;
            let mut hip = spec.hip;
            if far {
                hip.0 += b.far_dx;
                hip.1 -= 0.5;
            }
            let foot = foot(b, spec, hip, legs, slot, drop);
            let knee = bend(hip, foot, spec.upper, spec.lower, spec.fore);
            let coat = if far { b.leg.1 } else { b.leg.0 };
            let stripe = b.stripe.map(|s| if far { s.1 } else { s.0 });
            let bands: &[(f32, f32)] = if spec.fore {
                &[(0.42, 0.56), (0.78, 0.9)]
            } else {
                &[(0.3, 0.42), (0.62, 0.74)]
            };
            // A far leg only shows where the body does not already cover it.
            self.limb(
                hip,
                knee,
                spec.w_up,
                (spec.w_up + spec.w_lo) / 2.0,
                coat,
                stripe.map(|s| (s, bands)),
                far,
            );
            self.limb(
                knee,
                foot,
                (spec.w_up + spec.w_lo) / 2.0,
                spec.w_lo,
                coat,
                stripe.map(|s| (s, &[(0.5, 0.62)][..])),
                far,
            );
            let paw = if far { b.paw.1 } else { b.paw.0 };
            let (fx, fy) = (foot.0.round() as i32, foot.1.round() as i32);
            if b.paw_w <= 1 {
                // A hoof.
                self.set(fx, fy, paw);
                self.set(fx, fy - 1, paw);
            } else {
                // Paws reach forward, toes on the ground.
                let back = -(b.paw_w / 3);
                for dx in back..back + b.paw_w {
                    self.set(fx + dx, fy, paw);
                    if dx > back && dx < back + b.paw_w - 1 {
                        self.set(fx + dx, fy - 1, paw);
                    }
                }
            }
        }
    }

    /// Draws the tail, behind whatever is drawn after it.
    pub fn tail(&mut self, t: &TailSpec, pose: TailPose) {
        let (mut x, mut y) = t.root;
        let mut a = pose.base;
        let n = t.links;
        for i in 0..n {
            let k = i as f32 / n.max(1) as f32;
            let w = t.w0 + (t.w1 - t.w0) * k;
            let ch = if i + t.tip_len >= n {
                t.tip
            } else {
                match t.ring {
                    Some(r) if i >= 3 && (i % 4 == 1) => r,
                    _ => t.coat,
                }
            };
            let wave = pose.sway * (pose.phase - i as f32 * 0.32).sin();
            a += pose.curl + wave * 0.12;
            let (nx, ny) = (x + a.cos(), y + a.sin());
            self.limb((x, y), (nx, ny), w, w, ch, None, false);
            (x, y) = (nx, ny);
        }
    }
}

/// Where a foot goes, in body pixels.
fn foot(
    b: &Build,
    spec: &LegSpec,
    hip: (f32, f32),
    legs: Legs,
    slot: usize,
    drop: f32,
) -> (f32, f32) {
    let ground = b.ground - 1.0;
    let rest_x = hip.0 + if spec.fore { 0.5 } else { 0.0 };
    let reach = spec.upper + spec.lower;
    match legs {
        Legs::Stand => (rest_x, ground - drop),
        Legs::Gait(g, phase) => {
            let (duty, offs) = g.timing();
            let (sk, lk) = g.reach();
            let stride = b.stride * sk;
            let u = (phase + offs[slot]).rem_euclid(1.0);
            let (dx, up) = if u < duty {
                (stride * (0.5 - u / duty), 0.0)
            } else {
                let s = (u - duty) / (1.0 - duty);
                (stride * (s - 0.5), b.lift * lk * (PI * s).sin())
            };
            if g == Gait::Paddle {
                // Paws circle under the body instead of meeting the ground.
                let a = 2.0 * PI * u;
                let r = reach * 0.3;
                (
                    rest_x + r * a.cos() * 1.4,
                    hip.1 + reach * 0.7 + r * a.sin(),
                )
            } else {
                (rest_x + dx, ground - drop - up)
            }
        }
        Legs::Reach(k) => {
            let a = if spec.fore {
                // Forward and a little down, claws out.
                -0.35 + 0.9 * (1.0 - k)
            } else {
                // Back, pushed off.
                PI - 0.5 + 1.3 * (1.0 - k)
            };
            let r = reach * (0.75 + 0.2 * k);
            (hip.0 + r * a.cos(), hip.1 + r * a.sin())
        }
        Legs::Fold => {
            if spec.fore {
                (hip.0 + reach * 0.82, ground - drop)
            } else {
                (hip.0 + reach * 0.42, ground - drop)
            }
        }
        Legs::Dangle(phase) => {
            let kick = (phase * 2.0 * PI + slot as f32 * 1.7).sin();
            (hip.0 + kick * 2.5, hip.1 + reach * (0.85 + 0.1 * kick))
        }
        Legs::Sit(k) => {
            // Planted on the ground in the world's frame, which is turned the other way in the
            // body's frame while the body is turned by SIT.
            let pivot = b.hind.hip;
            let (c, s) = ((SIT * k).cos(), (SIT * k).sin());
            let world = if spec.fore {
                (rest_x, ground - drop)
            } else {
                (hip.0 + reach * 0.5, ground - drop)
            };
            let (px, py) = (world.0 - pivot.0, world.1 - pivot.1);
            (pivot.0 + px * c + py * s, pivot.1 - px * s + py * c)
        }
        Legs::Rear(phase) => {
            if spec.fore {
                // Paws on the trunk ahead, raking down and up.
                let rake = (phase * 2.0 * PI + slot as f32 * 1.4).sin();
                (hip.0 + reach * 0.55, hip.1 - reach * (0.55 + 0.2 * rake))
            } else {
                // Planted on the ground, which is turned the other way in
                // the body's frame while the body is turned by REAR.
                let pivot = b.hind.hip;
                let (px, py) = (rest_x - pivot.0, ground - drop - pivot.1);
                let (c, s) = (REAR.cos(), REAR.sin());
                (pivot.0 + px * c + py * s, pivot.1 - px * s + py * c)
            }
        }
    }
}

/// Where the middle joint of a leg from `hip` to `foot` falls, its segments
/// `l1` and `l2` long: forward of the line for front legs, behind it for
/// hind legs.
fn bend(hip: (f32, f32), foot: (f32, f32), l1: f32, l2: f32, fore: bool) -> (f32, f32) {
    let (dx, dy) = (foot.0 - hip.0, foot.1 - hip.1);
    let d = (dx * dx + dy * dy)
        .sqrt()
        .clamp((l1 - l2).abs() + 0.01, l1 + l2 - 0.01);
    let len = (dx * dx + dy * dy).sqrt().max(1e-3);
    let (ux, uy) = (dx / len, dy / len);
    let a = (l1 * l1 - l2 * l2 + d * d) / (2.0 * d);
    let h = (l1 * l1 - a * a).max(0.0).sqrt();
    let (px, py) = (hip.0 + ux * a, hip.1 + uy * a);
    // The two candidates sit either side of the line; pick by the joint's side.
    let (k1, k2) = ((px - uy * h, py + ux * h), (px + uy * h, py - ux * h));
    let forward = |k: (f32, f32)| k.0 - (hip.0 + (foot.0 - hip.0) * 0.5);
    let pick_first = if fore {
        forward(k1) >= forward(k2)
    } else {
        forward(k1) <= forward(k2)
    };
    if pick_first { k1 } else { k2 }
}

/// A rotation by `angle` done as three shears, each moving whole rows or
/// columns by whole pixels, so that no pixel is lost or doubled.
#[derive(Clone, Copy)]
pub struct Shears {
    a: f32,
    b: f32,
}

impl Shears {
    pub fn new(angle: f32) -> Shears {
        Shears {
            a: -(angle / 2.0).tan(),
            b: angle.sin(),
        }
    }

    /// Turns the offset `(x, y)` (y down) clockwise on screen.
    #[inline]
    pub fn turn(self, x: i32, y: i32) -> (i32, i32) {
        let x1 = x + (self.a * y as f32).round() as i32;
        let y2 = y + (self.b * x1 as f32).round() as i32;
        let x3 = x1 + (self.a * y2 as f32).round() as i32;
        (x3, y2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shears turn a quarter of the way round like a rotation does.
    #[test]
    fn shears_rotate() {
        let s = Shears::new(PI / 2.0);
        assert_eq!(s.turn(10, 0), (0, 10));
        assert_eq!(s.turn(0, 10), (-10, 0));
        let s = Shears::new(0.0);
        assert_eq!(s.turn(3, -4), (3, -4));
    }

    /// A bent leg's joint is where both segments reach, on its own side.
    #[test]
    fn legs_bend_the_right_way() {
        let hip = (0.0, 0.0);
        let foot = (0.0, 9.0);
        for fore in [true, false] {
            let k = bend(hip, foot, 6.0, 6.0, fore);
            let d1 = (k.0 * k.0 + k.1 * k.1).sqrt();
            let d2 = ((k.0 - foot.0).powi(2) + (k.1 - foot.1).powi(2)).sqrt();
            assert!((d1 - 6.0).abs() < 0.01 && (d2 - 6.0).abs() < 0.01);
            assert_eq!(k.0 > 0.0, fore, "fore {fore}: {k:?}");
        }
    }
}
