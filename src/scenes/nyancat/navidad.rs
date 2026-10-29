//! nyancat's Navidad: the cat flies through the posadas with a buñuelo.
//!
//! The frames are klange's, re-dressed: the pop-tart is a buñuelo (a crisp
//! golden fritter dusted with sugar, a squiggle of miel de piloncillo over
//! it) and, from January 2 to 6, a rosca de reyes (the same golden dough
//! studded with ate and acitrón). The rainbow becomes a string of foquitos:
//! six green cords with bulbs of every color that blink in chase patterns,
//! more of them lit as the posadas go. The stars are piñatas of siete picos
//! in papel de china that glue together and burst into colación, which keeps
//! drifting through the sky. From Nochebuena the cat wears a flor de
//! nochebuena at its ear. On the Santos Inocentes the cat flies backwards.

use super::costume::{Painter, Plain, Sky, corner, share, spot, stamp};
use crate::color::{self, Color};

/// The foquitos of a series, in rojo, verde, azul, amarillo, rosa and naranja.
const FOQUITOS: [u32; 6] = [0xff3b3b, 0x3bd16f, 0x3b7bff, 0xffd23b, 0xff5fb4, 0xff8c1a];

/// The piñata's papel de china, in rosa mexicano, amarillo, naranja,
/// turquesa, morado and verde limón.
const PAPEL: [u32; 6] = [0xe4007c, 0xffd200, 0xff8c00, 0x00aab8, 0x7028a0, 0x8cc828];

/// The cord the foquitos hang from, in verde pino.
const CORD: u8 = b'~';

/// The holiday's colors under `classic`, per pixel character. The dough, the
/// jewels and the flor de nochebuena keep their own colors under other
/// themes; the foquitos and the papel de china take the theme's.
const PALETTE: &[(u8, u32)] = &[
    (b'B', 0xb9772a), // the fried edge
    (b'F', 0xe2aa55), // golden dough
    (b'L', 0xf2c97e), // a blister
    (b'S', 0xf7ebcf), // sugar
    (b'P', 0x7a3a12), // miel de piloncillo
    (b'p', 0xb5651d), // the miel where it catches the light
    (b'W', 0xf4f1ea), // the rosca's pasta, blanco
    (b'w', 0xdcd5c2), // its shadow
    (b'R', 0xc8102e), // ate, rojo nochebuena
    (b'r', 0xee4a5c), // its shine
    (b'q', 0x8f0a20), // the flor de nochebuena's shade
    (b'N', 0xcfe08a), // acitrón
    (b'n', 0xeaf3b4), // its shine
    (b'M', 0xa8205a), // ate of membrillo
    (b'm', 0xd8548a), // its shine
    (b'Y', 0xf2b632), // dorado: the piñata's heart, a gold jewel
    (b'y', 0xffdd80), // its shine
    (b'V', 0x1f6f3a), // verde pino
    (b'v', 0x2f9a52), // verde pino, light
    (CORD, 0x1f6f3a),
];

/// The top of the buñuelo, from the pop-tart's corner: blisters in the
/// dough and three squiggles of miel de piloncillo with a drip each (`P`
/// over the dough `F` and the sugar `S`); `_` keeps the dough.
#[rustfmt::skip]
const BUNUELO: [&[u8; 17]; 13] = [
    b"_________________",
    b"____LL_____LL____",
    b"___PP__PP__PP__PP",
    b"___P_PP__PP__PP__",
    b"___p__LL_________",
    b"__LL_____________",
    b"_PP__PP__PP__PP__",
    b"_P_PP__PP__PP__PP",
    b"_p__LL___________",
    b"________LL_______",
    b"___PP__PP__PP__PP",
    b"___P_PP__PP__PP__",
    b"___p_LL__________",
];

/// A jewel of the rosca, three by two: ate rojo, acitrón, dorado, membrillo.
#[rustfmt::skip]
const JEWELS: [[&[u8; 3]; 2]; 4] = [
    [b"rRR", b"RRR"],
    [b"nNN", b"NNN"],
    [b"yYY", b"YYY"],
    [b"mMM", b"MMM"],
];

/// A flor de nochebuena: red bracts around a yellow heart, two leaves.
#[rustfmt::skip]
const NOCHEBUENA: [&[u8; 7]; 6] = [
    b"..R.R..",
    b".RRrRR.",
    b"RRqYqRR",
    b".RRrRR.",
    b"vV.R.Vv",
    b".v...v.",
];

/// The six stages of a piñata, in place of the six of a star: a speck of
/// papel, glued up, the cones coming, all seven picos, then bursting into
/// colación and the last of it. `P` and `Q` are the star's papeles, `Y` is
/// gold, and each is 7×7 around its center; `.` is empty.
#[rustfmt::skip]
const PINATA: [[&[u8; 7]; 7]; 6] = [
    [
        b".......",
        b".......",
        b".......",
        b"...P...",
        b".......",
        b".......",
        b".......",
    ],
    [
        b".......",
        b".......",
        b"...P...",
        b"..PYP..",
        b"...P...",
        b".......",
        b".......",
    ],
    [
        b".......",
        b"...Q...",
        b"..PPP..",
        b".QPYPQ.",
        b"..PPP..",
        b"...Q...",
        b".......",
    ],
    [
        b"...Q...",
        b".Q.P.Q.",
        b"..PPP..",
        b"..PYP..",
        b"QPPPPPQ",
        b"..P.P..",
        b"..Q.Q..",
    ],
    [
        b"...Q...",
        b".P...P.",
        b".......",
        b"Q..Y..Q",
        b".......",
        b".P...P.",
        b"...Q...",
    ],
    [
        b"...P...",
        b".......",
        b".......",
        b"P.....P",
        b".......",
        b".......",
        b"...P...",
    ],
];

/// The palette characters of the six foquitos lit (`0` to `5`), dimmed (`a`
/// to `f`), with a shine (`g` to `l`), and of the six papeles de china (`[`
/// to `` ` ``), in the order of [`FOQUITOS`] and [`PAPEL`].
const LIT: u8 = b'0';
const DIM: u8 = b'a';
const SHINE: u8 = b'g';
const PAPER: u8 = b'[';

/// A stripped frame dressed for the holiday: the pop-tart a buñuelo, or a
/// rosca de reyes when `reyes`.
pub(super) fn dress(f: &Plain, reyes: bool) -> Plain {
    let mut out = *f;
    for row in out.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                b'@' => b'B',
                b'$' => b'F',
                b'-' => b'S',
                c => c,
            };
        }
    }
    let Some((bx, by)) = corner(f) else {
        return out;
    };
    let dough = |c| matches!(c, b'F' | b'S');
    if reyes {
        // Jewels in staggered rows, a thin strip of pasta between them.
        for row in 0..4 {
            for col in 0..5 {
                let x = bx + 1 + col * 4 + (row % 2) * 2;
                let y = by + 1 + row * 3;
                stamp(&mut out, x, y, &JEWELS[(row + col * 3) as usize % 4], dough);
            }
            let y = by + 3 + row * 3;
            for x in bx + 1..bx + 18 {
                stamp(&mut out, x, y, &[b"W"], dough);
            }
        }
    } else {
        stamp(&mut out, bx, by, &BUNUELO, dough);
    }
    out
}

/// The holiday's colors for the theme: `classic`'s from [`FOQUITOS`],
/// [`PAPEL`] and [`PALETTE`], the foquitos and the papel de china from
/// `rainbow` (the theme's accents, as for the ordinary rainbow) otherwise.
pub(super) fn palette(pal: &mut [Color; 128], rainbow: Option<[Color; 6]>) {
    for &(ch, hex) in PALETTE {
        pal[ch as usize] = color::rgb(hex);
    }
    let (bulbs, papel) = match rainbow {
        None => (FOQUITOS.map(color::rgb), PAPEL.map(color::rgb)),
        Some(rb) => {
            pal[CORD as usize] = rb[3].scale(0.7);
            (rb, rb)
        }
    };
    let cord = pal[CORD as usize];
    for i in 0..6 {
        pal[(LIT + i as u8) as usize] = bulbs[i];
        // Lights out: the bulb's glass dark against the cord, so they blink
        // even in a theme whose colors cannot be dimmed.
        pal[(DIM + i as u8) as usize] = bulbs[i].lerp(cord, 0.7);
        pal[(SHINE + i as u8) as usize] = bulbs[i].lerp(Color::Rgb(255, 255, 255), 0.45);
        pal[(PAPER + i as u8) as usize] = papel[i];
    }
}

/// What the string of foquitos shows at cat pixel (x, y) of the trail, as a
/// palette character (`,` for nothing), at light tick `t` and `level`. `f`
/// and `flip` are the frame on screen, as for `sample`.
///
/// Each of the six bands is a strand: its top row a cord, and below it
/// bulbs two by two, four columns apart, a different color each. The bulbs
/// light in a chase that runs the other way on every strand, more of them
/// lit as the posadas go.
pub(super) fn light(f: &Plain, flip: bool, x: i32, y: i32, t: i64, level: f32) -> u8 {
    let k = spot(f, flip, x, y).k;
    if !(0..18).contains(&k) {
        return b',';
    }
    let (strand, row) = (k / 3, k % 3);
    if row == 0 {
        return CORD;
    }
    // Strands interleave their bulbs.
    let col = x + 2 * strand;
    let u = col.rem_euclid(4);
    if u != 1 && u != 2 {
        return b',';
    }
    let n = col.div_euclid(4) as i64;
    let color = (n + strand as i64).rem_euclid(6) as u8;
    let dir = if strand % 2 == 0 { 1 } else { -1 };
    let lit_of_five = 2 + (level.clamp(0.0, 1.0) * 2.0).round() as i64;
    if (n * dir + t).rem_euclid(5) >= lit_of_five {
        DIM + color
    } else if row == 1 && u == 1 {
        SHINE + color
    } else {
        LIT + color
    }
}

/// The piñata's two papeles, from the star's number.
#[inline]
fn papeles(n: usize) -> (u8, u8) {
    let a = (n * 7 + n / 3) % 6;
    (PAPER + a as u8, PAPER + ((a + 3) % 6) as u8)
}

/// Paints star number `n`, at stage `stage`, centered on (x, y).
pub(super) fn star(
    p: &mut Painter,
    solid: &impl Fn(i32, i32) -> bool,
    n: usize,
    x: i32,
    y: i32,
    stage: usize,
) {
    let (a, b) = papeles(n);
    let pal = p.pal;
    p.sprite_with(solid, x, y, &PINATA[stage % 6], |ch| match ch {
        b'P' => pal[a as usize],
        b'Q' => pal[b as usize],
        ch => pal[ch as usize & 127],
    });
}

/// The colación the burst piñatas leave, drifting through the sky, more of it
/// as the posadas go, behind whatever is `filled`.
pub(super) fn sky(
    p: &mut Painter,
    sky: &Sky,
    level: f32,
    k: i64,
    filled: &impl Fn(i32, i32) -> bool,
) {
    let candy = &sky.motes[0];
    for m in &candy[..share(candy.len(), level, 0.0)] {
        let (x, y) = sky.at(m, k, 4, 3);
        p.put(filled, x, y, PAPER + m.phase % 6);
        // Some pieces are a pair.
        if m.phase % 3 == 0 {
            p.put(filled, x + 1, y, PAPER + (m.phase / 6) % 6);
        }
    }
}

/// The flor de nochebuena at the cat's ear, from Nochebuena; `eye` is the
/// frame's left eye.
pub(super) fn flower(p: &mut Painter, level: f32, eye: (i32, i32)) {
    if level < 0.85 {
        return;
    }
    let (ex, ey) = eye;
    p.sprite(&|_, _| false, ex - 3, ey - 7, &NOCHEBUENA);
}

#[cfg(test)]
mod tests {
    use super::super::costume::is_band;
    use super::super::frames::FRAMES;
    use super::super::strip_stars;
    use super::*;

    #[test]
    fn the_pop_tart_is_a_bunuelo() {
        for f in &FRAMES {
            let plain = strip_stars(f);
            let d = dress(&plain, false);
            let count = |ch: u8| d.iter().flatten().filter(|&&c| c == ch).count();
            // No pop-tart left, no jewels either.
            for ch in *b"@$-RNMW" {
                assert_eq!(count(ch), 0, "{}", ch as char);
            }
            assert!(
                count(b'F') > 40 && count(b'B') > 30 && count(b'P') > 10,
                "{} {} {}",
                count(b'F'),
                count(b'B'),
                count(b'P')
            );
            // The cat and its trail are as they were.
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
    fn the_rosca_has_its_jewels() {
        for f in &FRAMES {
            let d = dress(&strip_stars(f), true);
            let count = |ch: u8| d.iter().flatten().filter(|&&c| c == ch).count();
            for ch in *b"RNYMW" {
                assert!(count(ch) > 2, "{}", ch as char);
            }
            // No miel on the rosca.
            assert_eq!(count(b'P'), 0);
        }
    }

    #[test]
    fn the_trail_is_a_string_of_foquitos() {
        let f = dress(&strip_stars(&FRAMES[0]), false);
        let lights = |level: f32, t: i64| {
            let mut n = [0usize; 3];
            for x in -60..0 {
                for y in 24..43 {
                    match light(&f, false, x, y, t, level) {
                        b',' => {}
                        CORD => n[0] += 1,
                        c if (LIT..LIT + 6).contains(&c) || (SHINE..SHINE + 6).contains(&c) => {
                            n[1] += 1
                        }
                        c if (DIM..DIM + 6).contains(&c) => n[2] += 1,
                        c => panic!("{}", c as char),
                    }
                }
            }
            n
        };
        // Six cords, bulbs of every color, some dark, and they blink.
        let n = lights(0.5, 0);
        assert!(n[0] >= 6 * 60 - 12 && n[1] > 100 && n[2] > 40, "{n:?}");
        let at = |t: i64| {
            (-60..0)
                .flat_map(|x| (24..43).map(move |y| (x, y)))
                .map(|(x, y)| light(&f, false, x, y, t, 0.5))
                .collect::<Vec<_>>()
        };
        assert_ne!(at(0), at(1));
        // More lit as the posadas go.
        assert!(lights(0.3, 0)[1] < lights(1.0, 0)[1]);
        // Nothing outside the trail's rows.
        assert_eq!(light(&f, false, -10, 5, 0, 1.0), b',');
        assert_eq!(light(&f, false, -10, 60, 0, 1.0), b',');
    }

    #[test]
    fn the_papeles_differ() {
        for n in 0..40 {
            let (a, b) = papeles(n);
            assert_ne!(a, b);
            assert!((PAPER..PAPER + 6).contains(&a) && (PAPER..PAPER + 6).contains(&b));
        }
    }
}
