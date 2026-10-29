//! What the fiestas' visitors share when they draw: pixel art at any size, a
//! thick line, and a few numbers every crowd needs.
//!
//! [`sprite`](super::sprite) draws art as it is drawn, one art pixel to a
//! half-block pixel, which suits a 24-row terminal; [`blit`] draws the same
//! art scaled (whole numbers above 1, so the pixels stay square, and fourths
//! below), mirrored, rotated and shifted row by row (a hem that sways), so a
//! visitor can fill a 300-row screen as well as a 20-row one.

use super::{paint, pixel};
use crate::color::Color;
use crate::render::Canvas;
use std::f32::consts::PI;

/// Where art goes and how it is transformed, in half-block pixels.
#[derive(Clone, Copy)]
pub struct Xf {
    /// The art pixel (column, row) that stays put when it scales or turns.
    pub pivot: (f32, f32),
    /// Where that pixel is on the screen.
    pub at: (f32, f32),
    /// How many screen pixels an art pixel takes, across and down.
    pub scale: (f32, f32),
    /// Radians, clockwise.
    pub rot: f32,
    /// Mirror the art left to right (it faces right).
    pub flip: bool,
}

impl Xf {
    /// The art's top-left pixel at (`x`, `y`), `k` times its size.
    pub fn new(x: i32, y: i32, k: f32) -> Xf {
        Xf {
            pivot: (0.0, 0.0),
            at: (x as f32, y as f32),
            scale: (k, k),
            rot: 0.0,
            flip: false,
        }
    }

    /// Where a point of the art (a column and row of the art as it is
    /// before any mirroring, in art `w` pixels wide) lands on the screen.
    pub fn map(&self, w: f32, (ax, ay): (f32, f32)) -> (f32, f32) {
        let ax = if self.flip { w - ax } else { ax };
        let (dx, dy) = (
            (ax - self.pivot.0) * self.scale.0,
            (ay - self.pivot.1) * self.scale.1,
        );
        let (sin, cos) = self.rot.sin_cos();
        (
            self.at.0 + dx * cos - dy * sin,
            self.at.1 + dx * sin + dy * cos,
        )
    }
}

/// Draws pixel art (as for [`sprite`](super::sprite): `.` and spaces are see
/// through, `color` gives the other letters theirs) under a transform.
/// `shift` moves each art row sideways, in art pixels.
pub(crate) fn blit(
    c: &mut Canvas,
    rows: &[&str],
    xf: Xf,
    shift: impl Fn(i32) -> i32,
    color: impl Fn(u8) -> Option<Color>,
) {
    let w = rows.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
    let h = rows.len() as i32;
    let (sx, sy) = xf.scale;
    if w == 0 || sx.abs() < 1e-3 || sy.abs() < 1e-3 {
        return;
    }
    let (sin, cos) = xf.rot.sin_cos();
    // The screen box the art can land in.
    let reach = (0..h).map(|r| shift(r).abs()).max().unwrap_or(0) as f32;
    let (mut lo, mut hi) = ((f32::MAX, f32::MAX), (f32::MIN, f32::MIN));
    for (ax, ay) in [
        (-reach, 0.0),
        (w as f32 + reach, 0.0),
        (-reach, h as f32),
        (w as f32 + reach, h as f32),
    ] {
        let (dx, dy) = ((ax - xf.pivot.0) * sx, (ay - xf.pivot.1) * sy);
        let (x, y) = (xf.at.0 + dx * cos - dy * sin, xf.at.1 + dx * sin + dy * cos);
        lo = (lo.0.min(x), lo.1.min(y));
        hi = (hi.0.max(x), hi.1.max(y));
    }
    let cols = (lo.0.floor() as i32).max(0)..=(hi.0.ceil() as i32).min(c.w() as i32 - 1);
    let screen =
        (lo.1.div_euclid(2.0) as i32).max(0)..=(hi.1.div_euclid(2.0) as i32).min(c.h() as i32 - 1);
    let sample = |x: i32, y: i32| -> Option<Color> {
        let (dx, dy) = (x as f32 + 0.5 - xf.at.0, y as f32 + 0.5 - xf.at.1);
        let (rx, ry) = (dx * cos + dy * sin, dy * cos - dx * sin);
        let (ax, ay) = (xf.pivot.0 + rx / sx, xf.pivot.1 + ry / sy);
        let r = ay.floor() as i32;
        if !(0..h).contains(&r) {
            return None;
        }
        let mut col = ax.floor() as i32 - shift(r);
        if xf.flip {
            col = w - 1 - col;
        }
        match *rows[r as usize]
            .as_bytes()
            .get(usize::try_from(col).ok()?)?
        {
            b'.' | b' ' => None,
            ch => color(ch),
        }
    };
    for row in screen {
        for col in cols.clone() {
            paint(c, col, row, sample(col, row * 2), sample(col, row * 2 + 1));
        }
    }
}

/// A square of `k` by `k` pixels, its top-left pixel at (`x`, `y`).
pub(crate) fn block(c: &mut Canvas, x: i32, y: i32, k: i32, color: Color) {
    for dy in 0..k {
        for dx in 0..k {
            pixel(c, x + dx, y + dy, color);
        }
    }
}

/// A line `thick` pixels across from (`x0`, `y0`) to (`x1`, `y1`).
pub(crate) fn line(
    c: &mut Canvas,
    (x0, y0): (f32, f32),
    (x1, y1): (f32, f32),
    thick: i32,
    color: Color,
) {
    let steps = (x1 - x0).abs().max((y1 - y0).abs()).ceil().max(1.0) as i32;
    let back = thick / 2;
    for s in 0..=steps {
        let t = s as f32 / steps as f32;
        let (x, y) = (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t);
        block(
            c,
            x.round() as i32 - back,
            y.round() as i32 - back,
            thick,
            color,
        );
    }
}

/// A feather from `base` to `tip`, bowed `bend` pixels sideways and `width`
/// pixels across at its fullest, in `color` with a lighter `spine`.
pub(crate) fn plume(
    c: &mut Canvas,
    base: (f32, f32),
    tip: (f32, f32),
    bend: f32,
    width: f32,
    (color, spine): (Color, Color),
) {
    let (dx, dy) = (tip.0 - base.0, tip.1 - base.1);
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let (nx, ny) = (-dy / len, dx / len);
    let at = |t: f32| {
        let off = bend * (PI * t).sin();
        (base.0 + dx * t + nx * off, base.1 + dy * t + ny * off)
    };
    // Discs of its width along it, close enough to leave no gaps: further
    // apart the wider it is, so a big feather costs no more than a small one.
    let steps = (len / (width * 0.2).max(0.67)).ceil() as i32;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let (x, y) = at(t);
        // Narrow at the quill, fullest a little before the tip.
        let r = width * 0.5 * (PI * t.powf(0.7)).sin();
        for py in (y - r).floor() as i32..=(y + r).ceil() as i32 {
            for px in (x - r).floor() as i32..=(x + r).ceil() as i32 {
                let (ex, ey) = (px as f32 + 0.5 - x, py as f32 + 0.5 - y);
                if ex * ex + ey * ey <= r * r + 0.3 {
                    pixel(c, px, py, color);
                }
            }
        }
    }
    let steps = (len * 1.5).ceil() as i32;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        if (0.1..0.92).contains(&t) {
            let (x, y) = at(t);
            pixel(c, x.floor() as i32, y.floor() as i32, spine);
        }
    }
}

/// The scale to draw art `art_h` pixels tall at on a screen `rows` rows
/// tall, so it fills at most `share` of it: see [`whole`].
pub fn fit(rows: u16, art_h: i32, share: f32) -> f32 {
    whole(rows as f32 * 2.0 * share / art_h.max(1) as f32)
}

/// The scale nearest to `k` that keeps the pixels square: whole numbers
/// from 1 to 6 (rounded down), fourths below 1 (never under a fourth).
pub fn whole(k: f32) -> f32 {
    if k >= 1.0 {
        k.floor().min(6.0)
    } else {
        ((k * 4.0).round() / 4.0).max(0.25)
    }
}

/// A repeatable number in 0..1 from two others: the same ones always give
/// the same, and different ones look unrelated.
pub fn hash(a: u32, b: u32) -> f32 {
    let mut v = a.wrapping_mul(0x9e37_79b9) ^ b.wrapping_mul(0x85eb_ca6b) ^ 0x27d4_eb2f;
    v ^= v >> 15;
    v = v.wrapping_mul(0x2c1b_3c6d);
    v ^= v >> 12;
    v = v.wrapping_mul(0x297a_2d39);
    v ^= v >> 15;
    (v & 0xffff) as f32 / 65536.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;
    use crate::eggs::halves;

    const RED: Color = Color::Rgb(255, 0, 0);

    /// With no transform it draws what `sprite` draws; scaled, each pixel
    /// is a square; mirrored, the art faces the other way.
    #[test]
    fn blit_scales_flips_and_clips() {
        let art = ["R.", "RR"];
        let mut c = Canvas::new(8, 4, Color::Reset);
        blit(&mut c, &art, Xf::new(1, 0, 1.0), |_| 0, |_| Some(RED));
        assert_eq!(halves(c.get(1, 0).unwrap()), (RED, RED));
        assert_eq!(halves(c.get(2, 0).unwrap()), (Color::Reset, RED));
        c.clear();
        blit(&mut c, &art, Xf::new(0, 0, 2.0), |_| 0, |_| Some(RED));
        // The top-left art pixel is a 2x2 square: two columns, one cell row.
        assert_eq!(halves(c.get(1, 0).unwrap()), (RED, RED));
        assert_eq!(halves(c.get(2, 0).unwrap()), (Color::Reset, Color::Reset));
        assert_eq!(halves(c.get(3, 1).unwrap()), (RED, RED));
        c.clear();
        let mirrored = Xf {
            flip: true,
            ..Xf::new(0, 0, 1.0)
        };
        blit(&mut c, &art, mirrored, |_| 0, |_| Some(RED));
        assert_eq!(halves(c.get(0, 0).unwrap()), (Color::Reset, RED));
        assert_eq!(halves(c.get(1, 0).unwrap()), (RED, RED));
        // A turn, a squeeze to nothing and the far edges are all fine.
        let turned = Xf {
            pivot: (1.0, 1.0),
            at: (3.0, 3.0),
            scale: (3.0, 3.0),
            rot: 1.0,
            flip: false,
        };
        for xf in [turned, Xf::new(-40, -9, 3.0), Xf::new(7, 3, 0.25)] {
            blit(&mut c, &art, xf, |r| r * 9, |_| Some(rgb(0)));
        }
    }

    /// A point of the art lands where the art is drawn, however it is
    /// scaled, turned and mirrored.
    #[test]
    fn map_follows_the_art() {
        let xf = Xf {
            pivot: (1.0, 1.0),
            at: (10.0, 10.0),
            scale: (2.0, 2.0),
            rot: 0.0,
            flip: false,
        };
        assert_eq!(xf.map(4.0, (1.0, 1.0)), (10.0, 10.0));
        assert_eq!(xf.map(4.0, (3.0, 2.0)), (14.0, 12.0));
        // A quarter turn clockwise takes what is to the right below.
        let turned = Xf {
            rot: std::f32::consts::FRAC_PI_2,
            ..xf
        };
        let (x, y) = turned.map(4.0, (3.0, 1.0));
        assert!(
            (x - 10.0).abs() < 1e-4 && (y - 14.0).abs() < 1e-4,
            "{x} {y}"
        );
        // Mirrored art has its left edge at the right.
        let mirrored = Xf {
            pivot: (3.0, 1.0),
            flip: true,
            ..xf
        };
        assert_eq!(mirrored.map(4.0, (1.0, 1.0)), (10.0, 10.0));
    }

    #[test]
    fn fit_picks_whole_scales_and_fourths() {
        assert_eq!(fit(24, 44, 0.9), 1.0);
        assert_eq!(fit(80, 40, 0.5), 2.0);
        assert_eq!(fit(500, 40, 0.5), 6.0);
        assert_eq!(fit(10, 44, 0.9), 0.5);
        assert_eq!(fit(1, 44, 0.9), 0.25);
        assert_eq!(fit(0, 44, 0.9), 0.25);
        assert_eq!(
            (whole(2.9), whole(1.0), whole(0.6), whole(0.01), whole(40.0)),
            (2.0, 1.0, 0.5, 0.25, 6.0)
        );
    }

    #[test]
    fn hashes_repeat_and_spread() {
        assert_eq!(hash(3, 4), hash(3, 4));
        let n = (0..1000).filter(|&i| hash(i, 7) < 0.5).count();
        assert!((400..600).contains(&n), "{n}");
    }
}
