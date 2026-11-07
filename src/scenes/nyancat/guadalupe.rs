//! nyancat's Virgen de Guadalupe: the cat flies through the night of December
//! 12 carrying a bouquet of roses and singing las mañanitas.
//!
//! The frames are klange's, re-dressed: the pop-tart is a bouquet of rosas de
//! Castilla, the roses that Juan Diego carried in his tilma, in two pinks among
//! their leaves, tied with a ribbon in the verde azulado of her mantle with
//! stars of gold on it and wrapped in manta. The rainbow is a garland: a vine
//! swaying along the wave of the trail, with a rose on it every so often and
//! the stars of the mantle twinkling above and below. The stars of the sky are
//! gold, and rose petals and glints of gold drift through it. Her image is
//! never drawn.

use super::costume::{Painter, Plain, Sky, corner, like_each, share, spot, stamp};
use crate::color::{self, Color};

/// The holiday's colors under `classic`, per pixel character. Other themes
/// take their own accents for them (see [`palette`]).
const PALETTE: &[(u8, u32)] = &[
    (b'R', 0xd81b60), // rosa de Castilla: petals
    (b'r', 0xf06292), // its light; the heart of the other pink
    (b'Q', 0xf06292), // the other rose: petals
    (b'q', 0xd81b60), // and its heart
    (b'G', 0x2e7d32), // the leaves of the rose
    (b'g', 0x1b5e20), // in shade
    (b'C', 0xefe6cf), // manta
    (b'x', 0xc9bfa6), // its folds
    (b'T', 0x00796b), // verde azulado: the mantle's ribbon
    (b't', 0x26a69a), // its light
    (b'Y', 0xf2c14e), // the gold of its stars
    (b'Z', 0xfff0b0), // and their glint
];

/// How the vine of the garland sways, by column of its fourteen: rows up (-1)
/// or down (+1) from the middle of the trail.
const SWAY: [i32; 14] = [0, 0, -1, -1, -1, 0, 0, 0, 1, 1, 1, 0, 0, 0];

/// A rosa de Castilla seen from above, five by five: `R` its petals, `r` the
/// swirl around its heart. A rose of the other pink swaps the letters for `Q`
/// and `q`.
#[rustfmt::skip]
const ROSA: [&[u8; 5]; 5] = [
    b".RRR.",
    b"RrrrR",
    b"RrRrR",
    b"RrrrR",
    b".RRR.",
];

/// The roses of the bouquet, from ten rows above the pop-tart's corner and one
/// column left of it (over the sky): five rosas de Castilla, `R` and `r` the
/// petals and the heart of one, `Q` and `q` of another with the pinks the other
/// way round, among leaves `G` and `g`; `_` keeps the sky.
#[rustfmt::skip]
const ROSAS: [&[u8; 19]; 9] = [
    b"____G___QQQ___G____",
    b"__RRR__QqqqQ__RRR__",
    b"_RrrrRGQqQqQGRrrrR_",
    b"_RrRrRGQqqqQGRrRrR_",
    b"GRrrrQQQQQQQQQrrrRG",
    b"GGRRQqqqQGQqqqQRRGG",
    b"GGgGQqQqQGQqQqQGgGG",
    b"GGGGQqqqQgQqqqQGGGG",
    b"_GGGGQQQGGGQQQGGGG_",
];

/// The rest of the bouquet, from the pop-tart's corner (over the tart `C` and `x`):
/// leaves, the ribbon `T` and `t` of her mantle with stars of gold `Y`, and the
/// manta `C` it is wrapped in with its folds `x`.
#[rustfmt::skip]
const RAMO: [&[u8; 17]; 14] = [
    b"GGGGGGGGGGGGGGGGG",
    b"gGGGgGGGgGGGgGGGg",
    b"GGGGGGGGGGGGGGGGG",
    b"GGgGGGgGGGgGGGgGG",
    b"TYTTTYTTTYTTTYTTT",
    b"TTTtTTTtTTTtTTTtT",
    b"CCCCCCCCCCCCCCCCC",
    b"CxCCCxCCCxCCCxCCC",
    b"CCCCCCCCCCCCCCCCC",
    b"CCCCCCCCCCCCCCCCC",
    b"xCCCxCCCxCCCxCCCx",
    b"CCCCCCCCCCCCCCCCC",
    b"CCCCCCCCCCCCCCCCC",
    b"xxxxxxxxxxxxxxxxx",
];

/// The six stages of a star of the mantle, in place of the six of a star: a
/// spark, a small cross, a cross growing, a full star with its glint, and two
/// stages of it going out. `Y` is gold and `Z` the glint; `.` is empty.
#[rustfmt::skip]
const STAR: [[&[u8; 5]; 5]; 6] = [
    [
        b".....",
        b".....",
        b"..Y..",
        b".....",
        b".....",
    ],
    [
        b".....",
        b"..Y..",
        b".YZY.",
        b"..Y..",
        b".....",
    ],
    [
        b"..Y..",
        b"..Y..",
        b"YYZYY",
        b"..Y..",
        b"..Y..",
    ],
    [
        b"..Y..",
        b".YYY.",
        b"YYZYY",
        b".YYY.",
        b"..Y..",
    ],
    [
        b"..Y..",
        b".....",
        b"Y.Z.Y",
        b".....",
        b"..Y..",
    ],
    [
        b".....",
        b"..Y..",
        b".Y.Y.",
        b"..Y..",
        b".....",
    ],
];

/// A rose of the other pink: `R` and `r` become `Q` and `q`.
fn other(ch: u8) -> u8 {
    match ch {
        b'R' => b'Q',
        b'r' => b'q',
        c => c,
    }
}

/// A stripped frame dressed for the holiday: the pop-tart a bouquet.
pub(super) fn dress(f: &Plain) -> Plain {
    let mut out = *f;
    for row in out.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                b'@' | b'$' => b'C',
                b'-' => b'x',
                c => c,
            };
        }
    }
    if let Some((bx, by)) = corner(f) {
        stamp(&mut out, bx - 1, by - 10, &ROSAS, |c| c == b',');
        stamp(&mut out, bx, by, &RAMO, |c| matches!(c, b'C' | b'x'));
    }
    out
}

/// The holiday's colors for the theme: [`PALETTE`]'s under `classic`; under
/// other themes the roses, the leaves, the gold and the ribbon are the colors
/// of `rainbow` (the theme's accents, as for the ordinary rainbow) nearest in
/// hue to theirs, and the manta is its text (`white`).
pub(super) fn palette(pal: &mut [Color; 128], rainbow: Option<[Color; 6]>, white: Color) {
    for &(ch, hex) in PALETTE {
        pal[ch as usize] = color::rgb(hex);
    }
    let Some(rb) = rainbow else {
        return;
    };
    let light = Color::Rgb(255, 255, 255);
    let [rose, leaf, gold, ribbon] = like_each(&rb, [0xd81b60, 0x2e7d32, 0xf2c14e, 0x00796b]);
    let pink = rose.lerp(light, 0.4);
    for (ch, c) in [
        (b'R', rose),
        (b'r', pink),
        (b'Q', pink),
        (b'q', rose),
        (b'G', leaf.scale(0.8)),
        (b'g', leaf.scale(0.5)),
        (b'C', white),
        (b'x', white.scale(0.75)),
        (b'T', ribbon),
        (b't', ribbon.lerp(light, 0.3)),
        (b'Y', gold),
        (b'Z', gold.lerp(light, 0.6)),
    ] {
        pal[ch as usize] = c;
    }
}

/// What the garland shows at cat pixel (x, y) of the trail, as a palette
/// character (`,` for nothing), at tick `t`. `f` and `flip` are the frame on
/// screen, as for `sample`.
///
/// A vine of leaves three rows thick sways along the trail; every fourteen
/// columns a rosa de Castilla sits on it (in the two pinks in turn), and
/// between them a star of the mantle twinkles, above the vine and below it by
/// turns.
pub(super) fn garland(f: &Plain, flip: bool, x: i32, y: i32, t: i64) -> u8 {
    let k = spot(f, flip, x, y).k;
    if !(0..16).contains(&k) {
        return b',';
    }
    let (n, u) = (x.div_euclid(14), x.rem_euclid(14));
    let mid = 8 + SWAY[u as usize];
    // A rose on the vine.
    if (1..6).contains(&u) {
        let ry = k - (mid - 2);
        if (0..5).contains(&ry) {
            let ch = ROSA[ry as usize][(u - 1) as usize];
            if ch != b'.' {
                return if n % 2 == 0 { ch } else { other(ch) };
            }
        }
    }
    // A star of the mantle, twinkling.
    if (8..13).contains(&u) {
        let (dx, dy) = (u - 10, k - if n % 2 == 0 { 2 } else { 13 });
        let reach = if (n as i64 + t).rem_euclid(3) == 0 {
            1
        } else {
            2
        };
        if (dx == 0 && dy.abs() <= reach) || (dy == 0 && dx.abs() <= reach) {
            return if dx == 0 && dy == 0 { b'Z' } else { b'Y' };
        }
    }
    // A leaf here and there, and the vine itself.
    if (u == 6 && k == mid - 3) || (u == 7 && k == mid + 3) {
        return b'G';
    }
    if (mid - 1..=mid + 1).contains(&k) {
        return if k != mid && (x + k).rem_euclid(3) == 0 {
            b'g'
        } else {
            b'G'
        };
    }
    b','
}

/// Paints a star of the sky at stage `stage`, centered on (x, y): gold, with
/// its glint, in place of the white star.
pub(super) fn star(
    p: &mut Painter,
    solid: &impl Fn(i32, i32) -> bool,
    x: i32,
    y: i32,
    stage: usize,
) {
    p.sprite(solid, x, y, &STAR[stage % 6]);
}

/// The petals of the rosas de Castilla and the glints of gold drifting through
/// the sky, behind whatever is `filled`.
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
        p.put(filled, x, y, if m.phase % 2 == 0 { b'R' } else { b'r' });
        // Some are a pair, the second one a pink lighter.
        if m.phase % 3 == 0 {
            p.put(filled, x + 1, y, b'Q');
        }
    }
    let glints = &sky.motes[1];
    for m in &glints[..share(glints.len(), level, 0.0)] {
        let (x, y) = sky.at(m, k, 2, 7);
        let lit = (k / 2 + m.phase as i64) % 4 == 0;
        p.put(filled, x, y, if lit { b'Z' } else { b'Y' });
    }
}

#[cfg(test)]
mod tests {
    use super::super::costume::is_band;
    use super::super::frames::FRAMES;
    use super::super::strip_stars;
    use super::*;

    #[test]
    fn the_pop_tart_is_a_bouquet() {
        for f in &FRAMES {
            let plain = strip_stars(f);
            let d = dress(&plain);
            let count = |ch: u8| d.iter().flatten().filter(|&&c| c == ch).count();
            for ch in *b"@$-" {
                assert_eq!(count(ch), 0, "{}", ch as char);
            }
            // Roses in both pinks, leaves, the ribbon with its stars, the manta.
            for ch in *b"RrQqGgCxT" {
                assert!(count(ch) > 3, "{}", ch as char);
            }
            // (Most of the ribbon is under the cat's face: its stars and its
            // light are a pixel or two each.)
            assert!(count(b'Y') > 0 && count(b't') > 0);
            // The cat and its rainbow are as they were; nothing is added but
            // what is heaped over the pop-tart.
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
    fn the_trail_is_a_garland_of_roses_and_stars() {
        let f = dress(&strip_stars(&FRAMES[0]));
        let at = |t: i64| {
            let mut n = [0usize; 4];
            for x in -140..0 {
                for y in 24..43 {
                    match garland(&f, false, x, y, t) {
                        b',' => {}
                        b'G' | b'g' => n[0] += 1,
                        b'R' | b'r' | b'Q' | b'q' => n[1] += 1,
                        b'Y' | b'Z' => n[2] += 1,
                        c => panic!("{}", c as char),
                    }
                    n[3] += 1;
                }
            }
            n
        };
        // A vine, roses of both pinks and stars, with room between them, and
        // the stars twinkle.
        let n = at(0);
        assert!(n[0] > 200 && n[1] > 100 && n[2] > 30, "{n:?}");
        assert!(n[0] + n[1] + n[2] < n[3] / 2, "{n:?}");
        assert_ne!(at(0), at(1));
        let pinks = |ch: u8| {
            (-140..0)
                .flat_map(|x| (24..43).map(move |y| (x, y)))
                .filter(|&(x, y)| garland(&f, false, x, y, 0) == ch)
                .count()
        };
        assert!(pinks(b'R') > 20 && pinks(b'Q') > 20);
        // Nothing outside the trail's rows.
        assert_eq!(garland(&f, false, -10, 5, 0,), b',');
        assert_eq!(garland(&f, false, -10, 60, 0), b',');
    }

    #[test]
    fn the_roses_take_the_themes_colors() {
        use crate::theme::Theme;
        for name in ["nord", "dracula", "gruvbox"] {
            let theme = Theme::builtin(name).unwrap();
            let mut pal = [Color::Reset; 128];
            palette(
                &mut pal,
                Some(super::super::theme_rainbow(&theme)),
                theme.fg,
            );
            // Roses, leaves, gold and the ribbon each have a color of their own.
            let main = (*b"RGYT").map(|ch| pal[ch as usize]);
            for (i, c) in main.iter().enumerate() {
                assert!(*c != Color::Reset && !main[..i].contains(c), "{name}");
            }
            // The two pinks of a rose are the other way round on the other.
            assert_eq!(pal[b'R' as usize], pal[b'q' as usize]);
            assert_eq!(pal[b'r' as usize], pal[b'Q' as usize]);
        }
    }

    #[test]
    fn the_stars_of_the_mantle_twinkle() {
        for (i, a) in STAR.iter().enumerate() {
            for b in &STAR[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }
}
