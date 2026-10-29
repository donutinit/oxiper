//! nyancat's Fiestas Patrias: the cat flies through the mes patrio with a
//! chile en nogada and a sombrero.
//!
//! The frames are klange's, re-dressed: the pop-tart is a chile en nogada (the
//! green poblano showing at its edges under the creamy white nogada, red
//! seeds of granada and flecks of perejil scattered over it) and the cat
//! wears a tiny sombrero de charro, café with a golden band and embroidery.
//! The rainbow is the flag: three bands in verde, blanco and rojo, with the
//! escudo (the eagle on its nopal) on every other flag of the wave. The stars
//! are luces de bengala, sparklers in the flag's three colors with a spark of
//! oro in the middle. As the month goes, more banderitas wave through the sky
//! and more tricolor papelitos drift, the most at the Grito.

use super::STAR_STAGES;
use super::costume::{Painter, Plain, Sky, corner, share, spot, stamp};
use crate::color::{self, Color};

/// The holiday's colors under `classic`, per pixel character. The chile, the
/// sombrero and the escudo keep their own colors under other themes; the
/// flag and the sparks take the theme's.
const PALETTE: &[(u8, u32)] = &[
    (b'G', 0x006847), // verde bandera: the poblano
    (b'g', 0x1c9a63), // its shine
    (b'N', 0xf4f1ea), // blanco: the nogada
    (b'n', 0xdfd8c4), // its folds
    (b'R', 0xce1126), // rojo: the granada
    (b'r', 0xff6b78), // its glint
    (b'p', 0x4f8a2a), // perejil; the nopal
    (b'b', 0x7a4a1e), // café: the águila, the sombrero, a flagpole
    (b'H', 0xa26a30), // the sombrero, lit
    (b'O', 0xd4a017), // oro: the embroidery
    (SPARK_GREEN, 0x1f9d5b),
    (SPARK_WHITE, 0xf4f1ea),
    (SPARK_RED, 0xe0263a),
];

/// The flag's three colors for what is small, the sparks and papelitos: a
/// verde bright enough to shine in the dark.
const SPARK_GREEN: u8 = b'l';
const SPARK_WHITE: u8 = b'w';
const SPARK_RED: u8 = b'k';
const TRICOLOR: [u8; 3] = [SPARK_GREEN, SPARK_WHITE, SPARK_RED];

/// The top of the chile, from the pop-tart's corner: nogada folds, clusters
/// of granada seeds with a glint, flecks of perejil (over the nogada `N`
/// and the seeds `R`); `_` keeps what is there.
#[rustfmt::skip]
const NOGADA: [&[u8; 17]; 13] = [
    b"_________________",
    b"___n___n___n___n_",
    b"__Rr___p__Rr___p_",
    b"__RR_n___rR______",
    b"____n_p___n______",
    b"_p______n___p____",
    b"____Rr___________",
    b"____RR__p_n______",
    b"_n______n________",
    b"__p_Rr___________",
    b"___RR___n________",
    b"_n____p__________",
    b"____n____________",
];

/// The shine on the poblano's skin, over the edge (`G`).
#[rustfmt::skip]
const SKIN: [&[u8; 17]; 3] = [
    b"_ggg__ggg__ggg___",
    b"_g_______________",
    b"_________________",
];

/// The sombrero de charro, 15×5: the crown with a golden band, the brim turned
/// up at its ends and embroidered with oro.
#[rustfmt::skip]
const SOMBRERO: [&[u8; 15]; 5] = [
    b".....bbbbb.....",
    b"....bHbbbbb....",
    b"....OOOOOOO....",
    b"b.bbbbbbbbbbb.b",
    b".bObObObObObOb.",
];

/// The escudo on the white band: the águila (café) over the nopal, with a
/// tuna, eight columns by five rows (`.` is the band).
#[rustfmt::skip]
const ESCUDO: [&[u8; 8]; 5] = [
    b"..b..b..",
    b".bbbbbb.",
    b"..bbbb..",
    b"...pp...",
    b"..pRpp..",
];

/// A banderita on its pole, waving between two shapes; `.` is empty.
#[rustfmt::skip]
const BANDERITA: [[&[u8; 7]; 4]; 2] = [
    [
        b"bllwwkk",
        b"bllwwkk",
        b"bllwwkk",
        b"b......",
    ],
    [
        b"bllww..",
        b"bllwwkk",
        b"bllwwkk",
        b"b....kk",
    ],
];

/// A stripped frame dressed for the holiday: the pop-tart a chile en nogada.
pub(super) fn dress(f: &Plain) -> Plain {
    let mut out = *f;
    for row in out.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                b'@' => b'G',
                b'$' => b'N',
                b'-' => b'R',
                c => c,
            };
        }
    }
    if let Some((bx, by)) = corner(f) {
        stamp(&mut out, bx, by, &NOGADA, |c| matches!(c, b'N' | b'R'));
        stamp(&mut out, bx, by, &SKIN, |c| c == b'G');
    }
    out
}

/// The holiday's colors for the theme: `classic`'s from [`PALETTE`], the
/// flag (the trail) and the sparks from `rainbow` (the theme's accents, as
/// for the ordinary rainbow) and `white` (its text) otherwise.
pub(super) fn palette(pal: &mut [Color; 128], rainbow: Option<[Color; 6]>, white: Color) {
    for &(ch, hex) in PALETTE {
        pal[ch as usize] = color::rgb(hex);
    }
    // Two bands each, from the top: verde, blanco, rojo.
    let (verde, blanco, rojo) = match rainbow {
        None => (pal[b'G' as usize], pal[b'N' as usize], pal[b'R' as usize]),
        Some(rb) => {
            for (ch, c) in [
                (SPARK_GREEN, rb[3]),
                (SPARK_WHITE, white),
                (SPARK_RED, rb[0]),
            ] {
                pal[ch as usize] = c;
            }
            (rb[3], white, rb[0])
        }
    };
    for (bands, c) in [(b">&", verde), (b"+#", blanco), (b"=;", rojo)] {
        for &b in bands {
            pal[b as usize] = c;
        }
    }
}

/// The flag at cat pixel (x, y) of the trail, whose character in the frame
/// is `ch`: the escudo on every other flag's white band. `f` and `flip` are
/// the frame on screen, as for `sample`.
pub(super) fn flag(f: &Plain, flip: bool, x: i32, y: i32, ch: u8) -> u8 {
    let s = spot(f, flip, x, y);
    if s.flag.rem_euclid(2) != 0 || !(6..11).contains(&s.k) {
        return ch;
    }
    match ESCUDO[(s.k - 6) as usize][s.u] {
        b'.' => ch,
        c => c,
    }
}

/// Paints star number `n`, at stage `stage`, centered on (x, y): a spark of
/// the flag's colors, gold in the middle once it is lit.
pub(super) fn star(
    p: &mut Painter,
    solid: &impl Fn(i32, i32) -> bool,
    n: usize,
    x: i32,
    y: i32,
    stage: usize,
) {
    let c = TRICOLOR[n % 3];
    for &(dx, dy) in STAR_STAGES[stage % 6] {
        p.put(solid, x + dx, y + dy, c);
    }
    if (2..=4).contains(&(stage % 6)) {
        p.put(solid, x, y, b'O');
    }
}

/// The sombrero on the cat's head, resting on its ears; `eye` is the frame's
/// left eye.
pub(super) fn sombrero(p: &mut Painter, eye: (i32, i32)) {
    let (ex, ey) = eye;
    p.sprite(&|_, _| false, ex + 3, ey - 9, &SOMBRERO);
}

/// The papelitos and banderitas of the mes patrio, drifting through the sky,
/// more of them as it goes, behind whatever is `filled`.
pub(super) fn sky(
    p: &mut Painter,
    sky: &Sky,
    level: f32,
    k: i64,
    filled: &impl Fn(i32, i32) -> bool,
) {
    let confetti = &sky.motes[0];
    for m in &confetti[..share(confetti.len(), level, 0.0)] {
        let (x, y) = sky.at(m, k, 4, 3);
        p.put(filled, x, y, TRICOLOR[m.phase as usize % 3]);
    }
    let flags = &sky.motes[1];
    for m in &flags[..share(flags.len(), level, 0.0)] {
        let (x, y) = sky.at(m, k, 2, 6);
        let wave = ((k + m.phase as i64) / 2 % 2) as usize;
        p.sprite(filled, x, y, &BANDERITA[wave]);
    }
}

#[cfg(test)]
mod tests {
    use super::super::costume::is_band;
    use super::super::frames::FRAMES;
    use super::super::strip_stars;
    use super::*;

    #[test]
    fn the_pop_tart_is_a_chile_en_nogada() {
        for f in &FRAMES {
            let plain = strip_stars(f);
            let d = dress(&plain);
            let count = |ch: u8| d.iter().flatten().filter(|&&c| c == ch).count();
            for ch in *b"@$-" {
                assert_eq!(count(ch), 0, "{}", ch as char);
            }
            assert!(count(b'N') > 40 && count(b'G') > 30 && count(b'R') > 8);
            for (a, b) in plain.iter().flatten().zip(d.iter().flatten()) {
                assert_eq!(*a == b',', *b == b',');
                assert_eq!(is_band(*a), is_band(*b));
                if matches!(*a, b'*' | b'.' | b'%' | b'\'') {
                    assert_eq!(a, b);
                }
            }
        }
    }

    #[test]
    fn the_flag_has_its_escudo() {
        let f = dress(&strip_stars(&FRAMES[0]));
        let mut escudos = 0;
        for x in -60..0 {
            for y in 24..43 {
                // The band under the escudo is white (and so it is on both sides of it).
                if flag(&f, false, x, y, b'+') != b'+' {
                    escudos += 1;
                }
            }
        }
        // Three or four flags with escudos, about twenty pixels each.
        assert!((50..=100).contains(&escudos), "{escudos}");
        assert_eq!(flag(&f, false, -10, 3, b'+'), b'+');
    }
}
