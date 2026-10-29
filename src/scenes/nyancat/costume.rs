//! What nyancat's holidays share: klange's frames re-dressed pixel by pixel
//! (`dress` in each holiday's module), the trail's geometry, and the sprites
//! the holidays paint over the sky, in cat pixels.

use super::frames::{FRAME_HEIGHT, FRAME_WIDTH};
use crate::color::Color;
use crate::render::Pixels;

/// A frame, one byte per pixel.
pub(super) type Plain = [[u8; FRAME_WIDTH]; FRAME_HEIGHT];

/// Where the left eye's highlight is in a (stripped) frame.
pub(super) fn eye(f: &Plain) -> Option<(i32, i32)> {
    f.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .position(|&c| c == b'.')
            .map(|x| (x as i32, y as i32))
    })
}

/// Where the pop-tart's top left corner is in a (stripped) frame, from the
/// first of its crust pixels.
pub(super) fn corner(f: &Plain) -> Option<(i32, i32)> {
    let (mut bx, mut by) = (FRAME_WIDTH as i32, FRAME_HEIGHT as i32);
    for (y, row) in f.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == b'@') {
            bx = bx.min(x as i32);
            by = by.min(y as i32);
        }
    }
    (by < FRAME_HEIGHT as i32).then_some((bx, by))
}

/// Stamps `art` with its corner at (x, y) over the pixels `over` accepts;
/// `_` keeps what is there.
pub(super) fn stamp<const W: usize>(
    out: &mut Plain,
    x: i32,
    y: i32,
    art: &[&[u8; W]],
    over: impl Fn(u8) -> bool,
) {
    for (j, line) in art.iter().enumerate() {
        for (i, &ch) in line.iter().enumerate() {
            let (px, py) = (x + i as i32, y + j as i32);
            if ch == b'_' || px < 0 || py < 0 {
                continue;
            }
            if let Some(p) = out
                .get_mut(py as usize)
                .and_then(|row| row.get_mut(px as usize))
                && over(*p)
            {
                *p = ch;
            }
        }
    }
}

/// Is this pixel one of the rainbow's bands?
#[inline]
pub(super) fn is_band(ch: u8) -> bool {
    matches!(ch, b'>' | b'&' | b'+' | b'#' | b'=' | b';')
}

/// Is this pixel part of the cat (not sky, not the trail)?
#[inline]
pub(super) fn is_solid(ch: u8) -> bool {
    ch != b',' && !is_band(ch)
}

/// Where a pixel of the trail is: nyancat's square wave steps every eight
/// columns, and each of these steps (a flag, for the papel picado) has a
/// number, counted outward from the cat.
pub(super) struct Spot {
    /// The step the pixel is in.
    pub flag: i64,
    /// Its column in the step, from the left.
    pub u: usize,
    /// Its row, from the top of the trail (0 to 17 inside it).
    pub k: i32,
}

/// Where cat pixel (x, y) is in the trail. `f` and `flip` are the frame on
/// screen, as for `sample`.
pub(super) fn spot(f: &Plain, flip: bool, x: i32, y: i32) -> Spot {
    let (flag, u, up) = if x < 0 {
        // The trail `sample` generates: 2 - x counts columns away from the frame.
        let m = 2 - x as i64;
        let up = ((m % 16) / 8 == 1) != flip;
        (m.div_euclid(8), 7 - m.rem_euclid(8), up)
    } else {
        let up = f[24][(x as usize).min(FRAME_WIDTH - 1)] == b'>';
        (
            -1 - (x as i64 - 2).div_euclid(8),
            (x as i64 - 2).rem_euclid(8),
            up,
        )
    };
    Spot {
        flag,
        u: u as usize,
        k: y - 23 + up as i32 - 2,
    }
}

/// How many of `n` things have come at `level`: none up to `from`, all of
/// them at the holiday's height.
pub(super) fn share(n: usize, level: f32, from: f32) -> usize {
    let k = ((level - from) / (1.0 - from)).clamp(0.0, 1.0);
    ((n as f32 * k).ceil() as usize).min(n)
}

/// Paints a holiday's sprites in cat pixels over the sky.
pub(super) struct Painter<'a> {
    pub px: &'a mut Pixels,
    pub pal: &'a [Color; 128],
    pub ox: i32,
    pub oy: i32,
    pub s: i32,
    /// Everything is mirrored around the frame's middle: the Santos Inocentes
    /// cat flies backwards. Sprites are placed where they would be
    /// unmirrored, and `solid` asks about that place too.
    pub mirror: bool,
}

impl Painter<'_> {
    /// Paints cat pixel (x, y) with `c` unless the cat is there.
    #[inline]
    pub fn dot(&mut self, solid: &impl Fn(i32, i32) -> bool, x: i32, y: i32, c: Color) {
        if solid(x, y) {
            return;
        }
        let s = self.s;
        let x = if self.mirror {
            FRAME_WIDTH as i32 - 1 - x
        } else {
            x
        };
        self.px.fill_rect(self.ox + x * s, self.oy + y * s, s, s, c);
    }

    /// Paints cat pixel (x, y) with palette character `ch` unless the cat is
    /// there.
    #[inline]
    pub fn put(&mut self, solid: &impl Fn(i32, i32) -> bool, x: i32, y: i32, ch: u8) {
        let c = self.pal[ch as usize & 127];
        self.dot(solid, x, y, c);
    }

    /// Paints `art` centered on cat pixel (x, y), where `solid` allows, with
    /// the colors `color` gives its characters; `.` is empty.
    pub fn sprite_with<const W: usize>(
        &mut self,
        solid: &impl Fn(i32, i32) -> bool,
        cx: i32,
        cy: i32,
        art: &[&[u8; W]],
        color: impl Fn(u8) -> Color,
    ) {
        let (hx, hy) = (W as i32 / 2, art.len() as i32 / 2);
        for (j, line) in art.iter().enumerate() {
            for (i, &ch) in line.iter().enumerate() {
                if ch != b'.' {
                    self.dot(solid, cx + i as i32 - hx, cy + j as i32 - hy, color(ch));
                }
            }
        }
    }

    /// Paints `art` centered on cat pixel (x, y), where `solid` allows, in
    /// the palette's colors; `.` is empty.
    pub fn sprite<const W: usize>(
        &mut self,
        solid: &impl Fn(i32, i32) -> bool,
        cx: i32,
        cy: i32,
        art: &[&[u8; W]],
    ) {
        let pal = self.pal;
        self.sprite_with(solid, cx, cy, art, |ch| pal[ch as usize & 127]);
    }
}

/// One thing drifting through the sky.
#[derive(Clone, Copy)]
pub(super) struct Mote {
    pub x: i32,
    pub y: i32,
    pub phase: u8,
}

/// Things a holiday scatters over the sky, in two kinds, drifting left more
/// slowly than the stars as they fall.
pub(super) struct Sky {
    pub motes: [Vec<Mote>; 2],
    /// Cat-pixel bounds of the sky (with a margin) and its width.
    lo: i32,
    top: i32,
    span: i32,
    rows: i32,
}

impl Sky {
    pub fn new() -> Sky {
        Sky {
            motes: [Vec::new(), Vec::new()],
            lo: 0,
            top: 0,
            span: 1,
            rows: 1,
        }
    }

    /// Scatters the two kinds over the sky from `x0`..`x1` × `y0`..`y1`, in
    /// cat pixels (the star field's, which wraps around seamlessly): one of
    /// each for every `per[0]` and `per[1]` cat pixels of sky, at least two.
    pub fn layout(
        &mut self,
        rng: &mut fastrand::Rng,
        x0: i32,
        x1: i32,
        y0: i32,
        y1: i32,
        per: [usize; 2],
    ) {
        self.lo = x0;
        self.top = y0;
        self.span = (x1 - x0).max(1);
        self.rows = (y1 - y0).max(1);
        let area = self.span as usize * self.rows as usize;
        for (motes, per) in self.motes.iter_mut().zip(per) {
            motes.clear();
            for _ in 0..(area / per.max(1)).max(2) {
                motes.push(Mote {
                    x: x0 + rng.i32(0..self.span),
                    y: y0 + rng.i32(0..self.rows),
                    phase: rng.u8(..),
                });
            }
        }
    }

    /// Where a mote is at animation tick `k`: it drifts left `vx` cat pixels a
    /// tick and falls one every `fall` ticks, wrapping around the sky.
    #[inline]
    pub fn at(&self, m: &Mote, k: i64, vx: i64, fall: i64) -> (i32, i32) {
        let x =
            self.lo + (m.x as i64 - self.lo as i64 - k * vx).rem_euclid(self.span as i64) as i32;
        let y = self.top
            + (m.y as i64 - self.top as i64 + (k + m.phase as i64) / fall.max(1))
                .rem_euclid(self.rows as i64) as i32;
        (x, y)
    }
}
