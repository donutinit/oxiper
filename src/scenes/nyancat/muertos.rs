//! nyancat's Día de Muertos: the cat comes home dressed for it.
//!
//! The frames are klange's, re-dressed pixel by pixel: the gray face becomes a
//! calavera de azúcar (bone white, flower eye sockets, a flower of icing on the
//! forehead, a stitched grin), the pop-tart a pan de muerto (golden crumb, a
//! darker crust, a cross of dough bones with the little ball on top, sugar
//! sprinkled over it) and the legs and tail are bone. The rainbow keeps its
//! six bands in papel picado colors, cut out like tissue paper; the stars
//! bloom as cempasúchil and scatter into petals. How far the holiday has come
//! decides the rest: how many petals the papel picado sheds and the sky
//! carries, monarch butterflies flying along from October 31, and from
//! November 1 a crown of flowers on the cat's head.

use super::frames::{FRAME_HEIGHT, FRAME_WIDTH};
use crate::color::{self, Color};
use crate::render::Pixels;

/// A frame, one byte per pixel.
pub(super) type Plain = [[u8; FRAME_WIDTH]; FRAME_HEIGHT];

/// The holiday's colors under `classic`, per pixel character. Icing (`R`,
/// `T`, `A`, `G`, `M`) and the papel picado take the theme's colors under
/// other themes; the bread, the bone and the flowers keep their own.
const PALETTE: &[(u8, u32)] = &[
    (b'W', 0xf4eee2), // hueso: the sugar skull, the legs and the tail
    (b'R', 0xe4007c), // rosa mexicano icing
    (b'T', 0x00aab8), // turquesa icing
    (b'A', 0xffd200), // amarillo icing
    (b'G', 0x8cc828), // verde limón
    (b'M', 0x7028a0), // morado
    (b'V', 0xb00048), // terciopelo
    (b'O', 0xff8c00), // cempasúchil
    (b'Y', 0xffd200), // its heart, gold
    (b'L', 0xffaa1e), // cempasúchil, light
    (b'D', 0xd66200), // cempasúchil, dark
    (b'B', 0x9c4a14), // the bread's crust
    (b'C', 0xd88a34), // the crumb
    (b'H', 0xf2b866), // the dough bones, lit
    (b'h', 0x8a3e10), // their shadow
    (b'S', 0xfff4dc), // sugar
    (b'K', 0x000000), // eye sockets, nose, grin
];

/// The papel picado bands, top to bottom, in place of nyancat's rainbow.
const PICADO: [u8; 6] = *b"ROAGTM";

/// The face, over the head's rows from five above the left eye's highlight to
/// four below and its columns from four left of it to ten right; `_` keeps
/// the frame's own pixel.
#[rustfmt::skip]
const FACE: [&[u8; 15]; 10] = [
    b"__WW_______WW__",
    b"__WRW_____WRW__",
    b"__WWW____WWWW__",
    b"__WWWWRAARWWW__",
    b"_WWWTTWRRWTTWW_",
    b"_WWTKKTWWTKKTW_",
    b"_WWTKKTWWTKKTW_",
    b"_WRWTTWMMWTTWR_",
    b"__WWKKKKKKKKW__",
    b"___WKWKWKWKW___",
];

/// The top of the pan de muerto, from the pop-tart's corner, over the crumb
/// the head leaves in sight: crossed dough bones (lit `H`, shaded `h`) with
/// the little ball where they meet, and sugar (`S`); `_` keeps the crumb.
#[rustfmt::skip]
const BREAD: [&[u8; 17]; 13] = [
    b"_________________",
    b"__S_HHh__S__S_S__",
    b"_____Hh_S___S____",
    b"_S__HHh_____S__S_",
    b"__S__Hh_S________",
    b"_H__HHHh_________",
    b"HHHHHSHHHh_______",
    b"hhh_HHHhhh_______",
    b"_S__hHhh_S_______",
    b"_S__HHh__________",
    b"__S__Hh_S________",
    b"____HHh__S_______",
    b"__S__h_S_________",
];

/// Where the left eye's highlight is in a (stripped) frame.
pub(super) fn eye(f: &Plain) -> Option<(i32, i32)> {
    f.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .position(|&c| c == b'.')
            .map(|x| (x as i32, y as i32))
    })
}

/// A stripped frame dressed for the holiday.
pub(super) fn dress(f: &Plain) -> Plain {
    let mut out = *f;
    for row in out.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                b'*' | b'.' => b'W',
                b'%' => b'R',
                b'@' => b'B',
                b'$' => b'C',
                b'-' => b'S',
                c => c,
            };
        }
    }

    // The bread: a cross of bones with the ball where they meet, on the part
    // of the pop-tart the head leaves in sight.
    let (mut bx, mut by) = (FRAME_WIDTH as i32, FRAME_HEIGHT as i32);
    for (y, row) in f.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == b'@') {
            bx = bx.min(x as i32);
            by = by.min(y as i32);
        }
    }
    if by < FRAME_HEIGHT as i32 {
        stamp(&mut out, bx, by, &BREAD, |c| matches!(c, b'C' | b'S'));
    }

    if let Some((ex, ey)) = eye(f) {
        stamp(&mut out, ex - 4, ey - 5, &FACE, |c| c != b',');
    }
    out
}

/// Stamps `art` with its corner at (x, y) over the pixels `over` accepts;
/// `_` keeps what is there.
fn stamp<const W: usize>(
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

/// Is this pixel one of the papel picado's (the rainbow's) bands?
#[inline]
pub(super) fn is_band(ch: u8) -> bool {
    matches!(ch, b'>' | b'&' | b'+' | b'#' | b'=' | b';')
}

/// Is this pixel part of the cat (not sky, not the trail)?
#[inline]
pub(super) fn is_solid(ch: u8) -> bool {
    ch != b',' && !is_band(ch)
}

/// The cut-outs of one flag of papel picado: eight columns (one step of
/// nyancat's square wave) by the trail's rows; `#` is cut. Flags alternate
/// between a flower and a calavera.
#[rustfmt::skip]
const CUTS: [[&[u8; 8]; 16]; 2] = [
    [
        b"........",
        b"........",
        b"...##...",
        b"..#..#..",
        b".#....#.",
        b"#..##..#",
        b".#....#.",
        b"..#..#..",
        b"...##...",
        b"........",
        b"........",
        b"..#..#..",
        b"........",
        b"........",
        b"........",
        b"........",
    ],
    [
        b"........",
        b"........",
        b"........",
        b"........",
        b".##..##.",
        b".##..##.",
        b"...##...",
        b"........",
        b".#.##.#.",
        b"........",
        b"........",
        b"..#..#..",
        b"........",
        b"........",
        b"........",
        b"........",
    ],
];

/// Is the papel picado cut out at cat pixel (x, y)? `f` and `flip` are the
/// frame on screen, as for `sample`.
pub(super) fn cut(f: &Plain, flip: bool, x: i32, y: i32) -> bool {
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
    let k = y - 23 + up as i32 - 2;
    if k < 0 {
        return false;
    }
    CUTS[flag.rem_euclid(2) as usize]
        .get(k as usize)
        .is_some_and(|line| line[u as usize] == b'#')
}

/// The six stages of a cempasúchil, in place of the six of a star: a bud,
/// opening, in full bloom, then its petals scattering. Each is 7×7 around
/// its center; `.` is empty.
#[rustfmt::skip]
pub(super) const MARIGOLD: [[&[u8; 7]; 7]; 6] = [
    [
        b".......",
        b".......",
        b".......",
        b"...L...",
        b".......",
        b".......",
        b".......",
    ],
    [
        b".......",
        b".......",
        b"...O...",
        b"..OYO..",
        b"...O...",
        b".......",
        b".......",
    ],
    [
        b".......",
        b"...D...",
        b"..OLO..",
        b".DLYLD.",
        b"..OLO..",
        b"...D...",
        b".......",
    ],
    [
        b"..D.D..",
        b".DOLOD.",
        b"DOLYLOD",
        b".LYYYL.",
        b"DOLYLOD",
        b".DOLOD.",
        b"..D.D..",
    ],
    [
        b"...O...",
        b".L...L.",
        b".......",
        b"O..Y..O",
        b".......",
        b".L...L.",
        b"...O...",
    ],
    [
        b"...D...",
        b".......",
        b".......",
        b"D.....D",
        b".......",
        b".......",
        b"...D...",
    ],
];

/// A monarch, wings open and closed; `.` is empty.
#[rustfmt::skip]
const MONARCH: [[&[u8; 7]; 4]; 2] = [
    [
        b"LOO.OOL",
        b"OODKDOO",
        b".OD.DO.",
        b"..O.O..",
    ],
    [
        b".......",
        b"..L.L..",
        b"..ODO..",
        b"...K...",
    ],
];

/// A crown of flowers on the cat's head: cempasúchil on the ears and
/// terciopelo between them, from three left of the eye and seven above.
#[rustfmt::skip]
const CROWN: [&[u8; 13]; 4] = [
    b"..L........L.",
    b".OYO..V...OYO",
    b"..D..VYV...D.",
    b"......V......",
];

/// The holiday's colors for the theme: `classic`'s from [`PALETTE`], the
/// icing and the papel picado from `rainbow` (the theme's accents, as for
/// the ordinary rainbow) otherwise.
pub(super) fn palette(pal: &mut [Color; 128], rainbow: Option<[Color; 6]>) {
    for &(ch, hex) in PALETTE {
        pal[ch as usize] = color::rgb(hex);
    }
    let bands = super::RAINBOW_CHARS;
    match rainbow {
        None => {
            for (b, p) in bands.iter().zip(PICADO) {
                pal[*b as usize] = pal[p as usize];
            }
        }
        Some(rb) => {
            for (b, c) in bands.iter().zip(rb) {
                pal[*b as usize] = c;
            }
            for (ch, i) in [(b'R', 0), (b'A', 2), (b'G', 3), (b'T', 4), (b'M', 5)] {
                pal[ch as usize] = rb[i];
            }
        }
    }
}

/// A petal in the sky, drifting left more slowly than the stars and falling.
#[derive(Clone, Copy)]
struct Petal {
    x: i32,
    y: i32,
    phase: u8,
}

/// A petal the papel picado sheds: it lets go of the trail, falls behind
/// and starts over.
#[derive(Clone, Copy)]
struct Shed {
    x: i32,
    y: i32,
    /// Ticks per fall and how far into it the petal starts.
    period: u8,
    phase: u8,
}

/// A monarch flying along with the cat.
#[derive(Clone, Copy)]
struct Monarch {
    x: i32,
    y: i32,
    phase: u8,
}

/// Where things go, in cat pixels, and how many of them there can be.
pub(super) struct Field {
    petals: Vec<Petal>,
    shed: Vec<Shed>,
    monarchs: Vec<Monarch>,
    /// Cat-pixel bounds of the sky (with a margin) and its width.
    lo: i32,
    top: i32,
    span: i32,
    rows: i32,
}

impl Field {
    pub fn new() -> Field {
        Field {
            petals: Vec::new(),
            shed: Vec::new(),
            monarchs: Vec::new(),
            lo: 0,
            top: 0,
            span: 1,
            rows: 1,
        }
    }

    /// Scatters everything over the sky from `x0`..`x1` × `y0`..`y1`, in cat
    /// pixels (the star field's, which wraps around seamlessly).
    pub fn layout(&mut self, rng: &mut fastrand::Rng, x0: i32, x1: i32, y0: i32, y1: i32) {
        self.lo = x0;
        self.top = y0;
        self.span = (x1 - x0).max(1);
        self.rows = (y1 - y0).max(1);
        let area = self.span as usize * self.rows as usize;
        self.petals.clear();
        for _ in 0..(area / 140).max(2) {
            self.petals.push(Petal {
                x: x0 + rng.i32(0..self.span),
                y: y0 + rng.i32(0..self.rows),
                phase: rng.u8(..),
            });
        }
        // Along the visible trail, more of them near the cat: they fall
        // behind it as they go.
        self.shed.clear();
        let trail = (22 - (x0 + 4)).max(1);
        for _ in 0..(trail as usize).clamp(6, 48) {
            let x = 22 - rng.i32(0..trail).min(rng.i32(0..trail));
            self.shed.push(Shed {
                x,
                y: 36 + rng.i32(0..5),
                period: rng.u8(30..56),
                phase: rng.u8(..),
            });
        }
        self.monarchs.clear();
        for _ in 0..(area / 900).clamp(2, 12) {
            self.monarchs.push(Monarch {
                x: x0 + rng.i32(0..self.span),
                y: y0 + rng.i32(0..self.rows),
                phase: rng.u8(..),
            });
        }
    }
}

/// How many of `n` things have come at `level`: none up to `from`, all of
/// them at the holiday's height.
fn share(n: usize, level: f32, from: f32) -> usize {
    let k = ((level - from) / (1.0 - from)).clamp(0.0, 1.0);
    ((n as f32 * k).ceil() as usize).min(n)
}

/// Paints the holiday's sprites in cat pixels: the marigolds where the stars
/// are, petals and monarchs behind the cat, its crown.
pub(super) struct Painter<'a> {
    pub px: &'a mut Pixels,
    pub pal: &'a [Color; 128],
    pub ox: i32,
    pub oy: i32,
    pub s: i32,
}

impl Painter<'_> {
    /// Paints cat pixel (x, y) with `ch` unless the cat is there.
    #[inline]
    pub fn put(&mut self, solid: &impl Fn(i32, i32) -> bool, x: i32, y: i32, ch: u8) {
        if solid(x, y) {
            return;
        }
        let c = self.pal[ch as usize & 127];
        let s = self.s;
        self.px.fill_rect(self.ox + x * s, self.oy + y * s, s, s, c);
    }

    /// Paints `art` centered on cat pixel (x, y), where `solid` allows; `.`
    /// is empty.
    pub fn sprite<const W: usize>(
        &mut self,
        solid: &impl Fn(i32, i32) -> bool,
        cx: i32,
        cy: i32,
        art: &[&[u8; W]],
    ) {
        let (hx, hy) = (W as i32 / 2, art.len() as i32 / 2);
        for (j, line) in art.iter().enumerate() {
            for (i, &ch) in line.iter().enumerate() {
                if ch != b'.' {
                    self.put(solid, cx + i as i32 - hx, cy + j as i32 - hy, ch);
                }
            }
        }
    }

    /// Everything but the marigolds, at animation tick `k` (kept well away
    /// from overflowing by the caller): petals behind
    /// whatever is `filled` (the cat, the papel picado but its cut-outs),
    /// monarchs behind the `solid` cat only.
    pub fn sky(
        &mut self,
        field: &Field,
        level: f32,
        k: i64,
        solid: &impl Fn(i32, i32) -> bool,
        filled: &impl Fn(i32, i32) -> bool,
    ) {
        // Petals drifting through the sky, tumbling as they go.
        for p in &field.petals[..share(field.petals.len(), level, 0.0)] {
            let ph = p.phase as i64;
            let x = field.lo
                + (p.x as i64 - field.lo as i64 - k * 4).rem_euclid(field.span as i64) as i32;
            let y = field.top
                + (p.y as i64 - field.top as i64 + (k + ph) / 3).rem_euclid(field.rows as i64)
                    as i32;
            match (k / 2 + ph) % 4 {
                0 => self.put(filled, x, y, b'L'),
                1 => {
                    self.put(filled, x, y, b'O');
                    self.put(filled, x + 1, y, b'D');
                }
                2 => self.put(filled, x, y, b'D'),
                _ => {
                    self.put(filled, x, y, b'O');
                    self.put(filled, x, y + 1, b'D');
                }
            }
        }
        // Petals letting go of the papel picado.
        for p in &field.shed[..share(field.shed.len(), level, 0.0)] {
            let period = p.period.max(1) as i64;
            let a = (k + p.phase as i64).rem_euclid(period);
            let x = p.x - (a * a / 40) as i32;
            let y = p.y + (a / 2) as i32 + [0, 1, 1, 0][(a / 3 % 4) as usize];
            let ch = if (a / 2) % 2 == 0 { b'L' } else { b'D' };
            self.put(filled, x, y, ch);
        }
        // Monarchs from the 31st, when the little ones come.
        for m in &field.monarchs[..share(field.monarchs.len(), level, 0.5)] {
            let ph = m.phase as i64;
            let x =
                field.lo + (m.x as i64 - field.lo as i64 - k).rem_euclid(field.span as i64) as i32;
            let bob = [0, 0, 1, 1, 1, 0, -1, -1][((k + ph) / 2 % 8) as usize];
            let open = ((k + ph) / 2) % 2 == 0;
            self.sprite(solid, x, m.y + bob, &MONARCH[!open as usize]);
        }
    }

    /// The crown of flowers on the cat's head, from November 1; `eye` is the
    /// frame's left eye.
    pub fn crown(&mut self, level: f32, eye: (i32, i32)) {
        if level < 0.8 {
            return;
        }
        let (ex, ey) = eye;
        self.sprite(&|_, _| false, ex + 3, ey - 5, &CROWN);
    }
}

#[cfg(test)]
mod tests {
    use super::super::frames::FRAMES;
    use super::super::strip_stars;
    use super::*;

    #[test]
    fn the_cat_is_dressed() {
        for f in &FRAMES {
            let plain = strip_stars(f);
            let d = dress(&plain);
            let count = |ch: u8| d.iter().flatten().filter(|&&c| c == ch).count();
            // No gray fur, no pop-tart left.
            for ch in *b"*%@$-." {
                assert_eq!(count(ch), 0, "{}", ch as char);
            }
            assert!(count(b'W') > 60 && count(b'T') == 16 && count(b'H') > 10);
            // The silhouette is the cat's.
            for (a, b) in plain.iter().flatten().zip(d.iter().flatten()) {
                assert_eq!(*a == b',', *b == b',');
                assert_eq!(is_band(*a), is_band(*b));
            }
        }
    }

    #[test]
    fn the_papel_picado_is_cut() {
        let f = strip_stars(&FRAMES[0]);
        let cuts = (-40..20)
            .flat_map(|x| (24..43).map(move |y| (x, y)))
            .filter(|&(x, y)| cut(&f, false, x, y))
            .count();
        assert!(cuts > 30, "{cuts}");
        assert!(!cut(&f, false, -3, 0) && !cut(&f, true, 5, 80));
    }
}
