//! nyancat's Candelaria: the cat flies through the night of the Candelaria
//! with a tamal.
//!
//! The frames are klange's, re-dressed: the pop-tart is a tamal in its hoja de
//! maíz (the husk opened around the masa, a bit of salsa roja in it, tied
//! with a strip of husk) and it steams. The stars are the little flames of the
//! velas that are blessed that day, flickering as they fly. The rainbow is
//! the rainbow.

use super::costume::{Painter, Plain, corner, stamp};
use crate::color::{self, Color};

/// The holiday's colors, per pixel character. They are the same under every
/// theme (the tamal, the vela's flame), but for the steam.
const PALETTE: &[(u8, u32)] = &[
    (b'H', 0xd9c48a), // hoja de maíz
    (b'h', 0xbfa468), // its fibers
    (b'T', 0xa88a50), // the strip of husk it is tied with
    (b'M', 0xf2e2b0), // masa
    (b'm', 0xe3cc92), // its grain
    (b'S', 0xb5311f), // salsa roja
    (b's', 0xd8553a), // its shine
    (b'e', 0xff6a1a), // a flame's edge
    (b'f', 0xffb000), // its body
    (b'y', 0xffd23b), // its light
    (b'c', 0xfff4c8), // its core
    (b'u', 0x4a78e0), // the blue at its base
];

/// The steam of the tamal: a pale blue-white under `classic`, the theme's
/// text under other themes.
const STEAM: u8 = b'v';
const STEAM_CLASSIC: u32 = 0xdce4f0;

/// The top of the tamal, from the pop-tart's corner: fibers in the husk, a
/// strip of husk tying it with a knot, grain in the masa and a patch of salsa
/// (over the husk `H`, the masa `M` and the salsa `S`); `_` keeps what is
/// there.
#[rustfmt::skip]
const TAMAL: [&[u8; 17]; 13] = [
    b"_hh_hhTTh_hhh_hh_",
    b"___m__TT__m___m__",
    b"_m____TT_____m___",
    b"___m__TT__m______",
    b"_m____TT_________",
    b"___m_TTTT_m______",
    b"____TTTTTT_______",
    b"_m___TTTT___m____",
    b"___m__TT_________",
    b"_SSs__TT_m_______",
    b"_SSS__TT__m______",
    b"_m____TT_________",
    b"_hh_hhTTh_hhh_hh_",
];

/// The six stages of a flame, in place of the six of a star: an ember, a
/// spark of flame, growing, in full, guttering and a last ember. Each is 5×7
/// around its center; `.` is empty.
#[rustfmt::skip]
const FLAME: [[&[u8; 5]; 7]; 6] = [
    [
        b".....",
        b".....",
        b".....",
        b"..e..",
        b".....",
        b".....",
        b".....",
    ],
    [
        b".....",
        b".....",
        b"..e..",
        b"..f..",
        b"..u..",
        b".....",
        b".....",
    ],
    [
        b".....",
        b"..e..",
        b".efe.",
        b".fyf.",
        b"..u..",
        b".....",
        b".....",
    ],
    [
        b"..e..",
        b".efe.",
        b".efe.",
        b"efycf",
        b"efccf",
        b".fyf.",
        b"..u..",
    ],
    [
        b".....",
        b"...e.",
        b"..ef.",
        b"..fy.",
        b"..fu.",
        b".....",
        b".....",
    ],
    [
        b".....",
        b".....",
        b".....",
        b"..f..",
        b".....",
        b".....",
        b".....",
    ],
];

/// A wisp of steam: a few pixels of it, rising and drifting.
const WISP: [(i32, i32); 3] = [(0, 0), (1, -1), (0, -2)];

/// A stripped frame dressed for the holiday: the pop-tart a tamal.
pub(super) fn dress(f: &Plain) -> Plain {
    let mut out = *f;
    for row in out.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                b'@' => b'H',
                b'$' => b'M',
                b'-' => b'S',
                c => c,
            };
        }
    }
    if let Some((bx, by)) = corner(f) {
        stamp(&mut out, bx, by, &TAMAL, |c| {
            matches!(c, b'H' | b'M' | b'S')
        });
    }
    out
}

/// The holiday's colors: [`PALETTE`]'s, and the steam's, `white` under other
/// themes (the theme's text).
pub(super) fn palette(pal: &mut [Color; 128], white: Option<Color>) {
    for &(ch, hex) in PALETTE {
        pal[ch as usize] = color::rgb(hex);
    }
    pal[STEAM as usize] = white.unwrap_or(color::rgb(STEAM_CLASSIC));
}

/// Paints a flame at stage `stage`, centered on (x, y).
pub(super) fn star(
    p: &mut Painter,
    solid: &impl Fn(i32, i32) -> bool,
    x: i32,
    y: i32,
    stage: usize,
) {
    p.sprite(solid, x, y, &FLAME[stage % 6]);
}

/// The steam rising from the tamal at animation tick `k`: three wisps, each
/// further along its way up, drifting back as they go; `eye` is the frame's
/// left eye.
pub(super) fn steam(p: &mut Painter, k: i64, eye: (i32, i32)) {
    let (ex, ey) = eye;
    for j in 0..3i64 {
        // Each wisp lives twelve ticks, four ticks apart from the next.
        let age = (k + 4 * j).rem_euclid(12);
        let x = ex - 10 + 4 * j as i32 - (age / 3) as i32 + [0, 1, 1, 0][(age % 4) as usize];
        let y = ey - 11 - age as i32 / 2;
        // They thin out as they rise.
        let n = if age < 8 { 3 } else { 1 };
        for &(dx, dy) in &WISP[..n] {
            p.put(&|_, _| false, x + dx, y + dy, STEAM);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::costume::is_band;
    use super::super::frames::FRAMES;
    use super::super::strip_stars;
    use super::*;

    #[test]
    fn the_pop_tart_is_a_tamal() {
        for f in &FRAMES {
            let plain = strip_stars(f);
            let d = dress(&plain);
            let count = |ch: u8| d.iter().flatten().filter(|&&c| c == ch).count();
            for ch in *b"@$-" {
                assert_eq!(count(ch), 0, "{}", ch as char);
            }
            assert!(
                count(b'H') > 30 && count(b'M') > 30 && count(b'T') > 10 && count(b'S') > 3,
                "{} {} {} {}",
                count(b'H'),
                count(b'M'),
                count(b'T'),
                count(b'S')
            );
            // The cat and its rainbow are as they were.
            for (a, b) in plain.iter().flatten().zip(d.iter().flatten()) {
                assert_eq!(*a == b',', *b == b',');
                assert_eq!(is_band(*a), is_band(*b));
                if matches!(*a, b'*' | b'.' | b'%' | b'\'') || is_band(*a) {
                    assert_eq!(a, b);
                }
            }
        }
    }

    #[test]
    fn the_flames_flicker() {
        // Each stage is a different flame.
        for (i, a) in FLAME.iter().enumerate() {
            for (j, b) in FLAME.iter().enumerate().skip(i + 1) {
                assert_ne!(a, b, "{i} {j}");
            }
        }
    }
}
