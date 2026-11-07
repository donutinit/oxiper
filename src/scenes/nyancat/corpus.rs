//! nyancat's Jueves de Corpus: the cat flies through the Thursday of Corpus
//! Christi carrying a huacal.
//!
//! The frames are klange's, re-dressed: the pop-tart is a huacal, the wooden
//! crate that children carry to the church, heaped with fruit (mangos, tunas,
//! naranjas, limones) and flowers of papel in rosa mexicano, amarillo and
//! morado, standing on a rebozo over the back of a mulita de hoja de maíz.
//! The rainbow is the rainbow. The stars are flowers of papel that open and
//! fall apart, and petals of papel and strips of hoja de maíz drift through
//! the sky.

use super::costume::{Painter, Plain, Sky, corner, like_each, share, stamp};
use crate::color::{self, Color};

/// The holiday's colors, per pixel character: the same under every theme
/// (the wood, the husk, the rebozo, the fruit and the leaves).
const PALETTE: &[(u8, u32)] = &[
    (b'W', 0xa0703a), // wood of the huacal
    (b'w', 0x6a4520), // its gaps and posts
    (b'H', 0xd9c48a), // hoja de maíz
    (b'h', 0xb08d4a), // its folds
    (b'B', 0x2a5caa), // azul of the rebozo
    (b'C', 0xefe6cf), // manta: its stripe, and the hearts of the flowers
    (b'G', 0x2f9a3a), // leaves
    (b'g', 0x1c6a28), // in shade
    (b'M', 0xffb02e), // mango
    (b'T', 0xc2185b), // tuna
    (b'O', 0xff8c1a), // naranja
    (b'L', 0x7cb342), // limón
];

/// The flowers of papel, which take the theme's colors under other themes:
/// rosa mexicano, amarillo and morado.
const FLOWERS: [(u8, u32); 3] = [(b'P', 0xe4007c), (b'Y', 0xffd200), (b'V', 0x7028a0)];

/// The flowers and fruit heaped over the huacal, from nine rows above the
/// pop-tart's corner and one column left of it (over the sky): papel flowers
/// `P`, `Y` and `V` with their hearts `C`, mangos `M`, tunas `T`, naranjas `O` and
/// limones `L` among leaves `G` and `g`; `_` keeps the sky.
#[rustfmt::skip]
const HEAP: [&[u8; 19]; 8] = [
    b"________YYY________",
    b"_______YYYYY__VVV__",
    b"__PPP__YYCYY_VVVVV_",
    b"_PPPPP_YYYYY_VVCVV_",
    b"_PPCPPGGYYYGGVVVVV_",
    b"_PPPPMMMGTTTGOOOVG_",
    b"LLPPPMMMGTTTGOOOGGg",
    b"LLgGGMMMGTTTgOOOgGg",
];

/// The huacal and what carries it, from the pop-tart's corner (over the tart `W`,
/// `H` and `h`): the crate's slats `W`, their gaps and posts `w`, the rebozo `B`
/// with its manta stripe `C`, and the mulita's hoja de maíz `H` with its folds `h`.
#[rustfmt::skip]
const HUACAL: [&[u8; 17]; 14] = [
    b"wWWWWWWWWWWWWWWWw",
    b"wwwwwwwwwwwwwwwww",
    b"wWWWWWWWWWWWWWWWw",
    b"wWWWWWWWWWWWWWWWw",
    b"wwwwwwwwwwwwwwwww",
    b"wWWWWWWWWWWWWWWWw",
    b"wWWWWWWWWWWWWWWWw",
    b"wwwwwwwwwwwwwwwww",
    b"BBBBBBBBBBBBBBBBB",
    b"CCCCCCCCCCCCCCCCC",
    b"BBBBBBBBBBBBBBBBB",
    b"hHHhHHhHHhHHhHHhH",
    b"HhHHhHHhHHhHHhHHh",
    b"HHhHHhHHhHHhHHhHH",
];

/// The six stages of a flower of papel, in place of the six of a star: a bud,
/// opening, in bloom, its petals loosening, and the last of them falling.
/// Each is 5×5 around its center; `F` is the flower's color, `C` its heart and
/// `.` is empty.
#[rustfmt::skip]
const BLOOM: [[&[u8; 5]; 5]; 6] = [
    [
        b".....",
        b".....",
        b"..F..",
        b".....",
        b".....",
    ],
    [
        b".....",
        b"..F..",
        b".FCF.",
        b"..F..",
        b".....",
    ],
    [
        b".F.F.",
        b"FFFFF",
        b".FCF.",
        b"FFFFF",
        b".F.F.",
    ],
    [
        b".FFF.",
        b"FFFFF",
        b"FFCFF",
        b"FFFFF",
        b".FFF.",
    ],
    [
        b"F...F",
        b".....",
        b"..C..",
        b".....",
        b"F...F",
    ],
    [
        b".....",
        b".F.F.",
        b".....",
        b".F.F.",
        b".....",
    ],
];

/// A stripped frame dressed for the holiday: the pop-tart a huacal on a mulita,
/// with its fruit and flowers heaped over the rim.
pub(super) fn dress(f: &Plain) -> Plain {
    let mut out = *f;
    for row in out.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                b'@' => b'W',
                b'$' => b'H',
                b'-' => b'h',
                c => c,
            };
        }
    }
    if let Some((bx, by)) = corner(f) {
        stamp(&mut out, bx - 1, by - 9, &HEAP, |c| c == b',');
        stamp(&mut out, bx, by, &HUACAL, |c| {
            matches!(c, b'W' | b'H' | b'h')
        });
    }
    out
}

/// The holiday's colors for the theme: [`PALETTE`]'s and [`FLOWERS`]' under
/// `classic`, the flowers in the colors of `rainbow` (the theme's accents, as
/// for the ordinary rainbow) nearest in hue to theirs otherwise.
pub(super) fn palette(pal: &mut [Color; 128], rainbow: Option<[Color; 6]>) {
    for &(ch, hex) in PALETTE {
        pal[ch as usize] = color::rgb(hex);
    }
    let hexes = FLOWERS.map(|(_, hex)| hex);
    let colors = match rainbow {
        None => hexes.map(color::rgb),
        Some(rb) => like_each(&rb, hexes),
    };
    for (&(ch, _), c) in FLOWERS.iter().zip(colors) {
        pal[ch as usize] = c;
    }
}

/// Paints star number `n`, at stage `stage`, centered on (x, y): a flower of
/// papel in rosa mexicano, amarillo or morado in turn.
pub(super) fn star(
    p: &mut Painter,
    solid: &impl Fn(i32, i32) -> bool,
    n: usize,
    x: i32,
    y: i32,
    stage: usize,
) {
    let color = FLOWERS[n % 3].0;
    let pal = p.pal;
    p.sprite_with(solid, x, y, &BLOOM[stage % 6], |ch| match ch {
        b'F' => pal[color as usize],
        ch => pal[ch as usize & 127],
    });
}

/// The petals of papel and the strips of hoja de maíz drifting through the
/// sky, behind whatever is `filled`.
pub(super) fn sky(
    p: &mut Painter,
    sky: &Sky,
    level: f32,
    k: i64,
    filled: &impl Fn(i32, i32) -> bool,
) {
    let petals = &sky.motes[0];
    for m in &petals[..share(petals.len(), level, 0.0)] {
        let (x, y) = sky.at(m, k, 3, 4);
        p.put(filled, x, y, FLOWERS[m.phase as usize % 3].0);
    }
    let husks = &sky.motes[1];
    for m in &husks[..share(husks.len(), level, 0.0)] {
        let (x, y) = sky.at(m, k, 2, 5);
        p.put(filled, x, y, b'H');
        // A strip is two pixels long, lying flat or leaning.
        p.put(filled, x + 1, y + (m.phase as i32 % 2), b'h');
    }
}

#[cfg(test)]
mod tests {
    use super::super::costume::is_band;
    use super::super::frames::FRAMES;
    use super::super::strip_stars;
    use super::*;

    #[test]
    fn the_pop_tart_is_a_huacal() {
        for f in &FRAMES {
            let plain = strip_stars(f);
            let d = dress(&plain);
            let count = |ch: u8| d.iter().flatten().filter(|&&c| c == ch).count();
            for ch in *b"@$-" {
                assert_eq!(count(ch), 0, "{}", ch as char);
            }
            // Wood, rebozo and husk; flowers of all three colors, fruit and leaves.
            for ch in *b"WwHhBCGgPYVMTOL" {
                assert!(count(ch) > 3, "{}", ch as char);
            }
            // The cat and its rainbow are as they were, and nothing is added
            // but what is heaped over the crate.
            let (_, by) = corner(&plain).unwrap();
            for (y, (ra, rb)) in plain.iter().zip(d.iter()).enumerate() {
                for (a, b) in ra.iter().zip(rb.iter()) {
                    let heap = (y as i32) < by - 1;
                    assert!(heap || (*a == b',') == (*b == b','));
                    assert_eq!(is_band(*a), is_band(*b));
                    if matches!(*a, b'*' | b'.' | b'%' | b'\'') || is_band(*a) {
                        assert_eq!(a, b);
                    }
                }
            }
        }
    }

    #[test]
    fn the_flowers_take_the_themes_colors() {
        use crate::theme::Theme;
        for name in ["nord", "dracula", "gruvbox", "mono"] {
            let theme = Theme::builtin(name).unwrap();
            let rainbow = super::super::theme_rainbow(&theme);
            let mut pal = [Color::Reset; 128];
            palette(&mut pal, Some(rainbow));
            let flowers = FLOWERS.map(|(ch, _)| pal[ch as usize]);
            assert!(flowers.iter().all(|c| *c != Color::Reset), "{name}");
            // Each takes a color of its own, while the rainbow has any left.
            if rainbow
                .iter()
                .enumerate()
                .all(|(i, c)| !rainbow[..i].contains(c))
            {
                assert!(
                    flowers[0] != flowers[1] && flowers[1] != flowers[2],
                    "{name}"
                );
                assert!(flowers[0] != flowers[2], "{name}");
            }
        }
    }

    #[test]
    fn the_flowers_bloom_and_fall() {
        for (i, a) in BLOOM.iter().enumerate() {
            for b in &BLOOM[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }
}
