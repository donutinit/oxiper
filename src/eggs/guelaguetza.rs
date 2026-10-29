//! What the Guelaguetza visitors share: their colors, and a woman in her
//! huipil.
//!
//! The Guelaguetza is Oaxaca's festival of the regions, in which each of its
//! regions dances in its own costume. Under `classic` the letters of the
//! visitors' pixel art are the colors of Oaxacan embroidery: grana
//! cochinilla, amarillo, verde, rosa mexicano, morado and turquesa, the white
//! of cloth, brass, straw and wood. Under any other theme each hue becomes
//! the theme accent nearest to it, shades are taken from those, white is the
//! theme's foreground and the dark inks are mixed from its background, so the
//! visitors belong to the theme.
//!
//! Two of the visitors have a woman in them, the Flor de Piña dancers and the
//! marmotas' carriers, so she is drawn here: the art from the crown of
//! flowers to the hem of the huipil (the long embroidered blouse), her braids
//! tied with ribbons, and the skirt below it, drawn from numbers because it
//! is a cone: it looks the same from every side, its pleats, bands and lace
//! turn round it and it flies out as she spins.

use super::blit::{Xf, blit, block, line};
pub use super::palette::Palette;
use super::palette::Role;
use super::pixel;
use crate::render::Canvas;
use crate::theme::Theme;

/// Letter, color under `classic`, and role under other themes. Letters that
/// a `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // Grana cochinilla.
    (b'G', 0x9b111e, Role::Hue),
    (b'g', 0x5c0a12, Role::Shade(b'G', 0.6)),
    // Amarillo.
    (b'Y', 0xffcc00, Role::Hue),
    (b'y', 0xb88f00, Role::Shade(b'Y', 0.7)),
    (b'L', 0xffe66b, Role::Shade(b'Y', 1.25)),
    // Verde.
    (b'V', 0x1f8f4a, Role::Hue),
    (b'v', 0x105c2c, Role::Shade(b'V', 0.62)),
    (b'l', 0x5ccf86, Role::Shade(b'V', 1.4)),
    // Rosa mexicano.
    (b'R', 0xe4007c, Role::Hue),
    (b'r', 0xff6eb4, Role::Shade(b'R', 1.4)),
    (b'T', 0xa8005c, Role::Shade(b'R', 0.7)),
    // Morado.
    (b'M', 0x6a2c91, Role::Hue),
    (b'P', 0xa468d4, Role::Shade(b'M', 1.4)),
    (b'm', 0x3e1a58, Role::Shade(b'M', 0.6)),
    // Turquesa.
    (b'Q', 0x00aab8, Role::Hue),
    (b'z', 0x6fe0e8, Role::Shade(b'Q', 1.4)),
    (b'q', 0x00697d, Role::Shade(b'Q', 0.62)),
    // Naranja.
    (b'O', 0xff8c00, Role::Hue),
    (b'o', 0xffb228, Role::Shade(b'O', 1.3)),
    // The golden skin of a piña, and the brown of its crosshatch.
    (b'A', 0xe8a020, Role::Shade(b'Y', 0.9)),
    (b'a', 0xa8661c, Role::Shade(b'Y', 0.55)),
    // Skin, light, medium and dark, each with its shadow.
    (b'S', 0xf0c09a, Role::Shade(b'O', 1.35)),
    (b's', 0xc88a5e, Role::Shade(b'O', 0.95)),
    (b'H', 0xc98a58, Role::Shade(b'O', 1.05)),
    (b'h', 0x9a5f38, Role::Shade(b'O', 0.75)),
    (b'J', 0x8a5230, Role::Shade(b'O', 0.7)),
    (b'j', 0x5e3420, Role::Shade(b'O', 0.5)),
    // The white of the huipil, the lace and the eyes.
    (b'W', 0xf4eee2, Role::Bone(1.0)),
    (b'w', 0xc9bfb0, Role::Bone(0.78)),
    // Hair and the dark inks.
    (b'K', 0x2a1a14, Role::Ink(0.2)),
    (b'k', 0x1c1424, Role::Ink(0.06)),
    (b'N', 0x2c2238, Role::Ink(0.14)),
    (b'n', 0x5a4c6e, Role::Ink(0.32)),
    // Brass, and its glint.
    (b'B', 0xe0b030, Role::Shade(b'Y', 0.95)),
    (b'b', 0x8a6410, Role::Shade(b'Y', 0.55)),
    (b'F', 0xfff6c8, Role::Bone(1.05)),
    // Wood, and straw.
    (b'D', 0xa05a28, Role::Shade(b'O', 0.62)),
    (b'd', 0x64371a, Role::Shade(b'O', 0.4)),
    (b'U', 0xd92b2b, Role::Hue),
];

impl Palette {
    /// The Guelaguetza visitors' colors under a theme.
    pub fn guelaguetza(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

/// A woman from the crown of flowers to the hem of her huipil, facing front
/// (the skin is `S` and `s`: each woman has her own tone).
pub const HUIPIL: &[&str] = &[
    ".....Y...R...Y.....",
    "....VRVYVMVYVRV....",
    ".....KKKKKKKKK.....",
    ".....KKKKKKKKK.....",
    "....KKSSSSSSSKK....",
    "....KKKKKSKKKKK....",
    "....KKSkWSWkSKK....",
    "....KKrSSsSSrKK....",
    "..KNKYSSRTRSSYKNK..",
    "..RRKKSSSSSSSKKRR..",
    "..KN....sss....NK..",
    ".wKKYRGsssssGRYKKw.",
    ".wQQQWVWRWRWVWQQQw.",
    ".wKKkWWkWWWkWWkKKw.",
    "..KNWWWWWWWWWWWNK..",
    "..YYVRWWWWWWWRVYY..",
    "..KNRYRWVMVWRYRNK..",
    "..KKWRVWMYMWVRWKK..",
    "..KNWWWWWMWWWWWNK..",
    "..KKVQWWWWWWWQVKK..",
    "..RRQYQWVGVWQYQRR..",
    ".RRWWQVWGYGWVQWWRR.",
    "..wWWWWWWGWWWWWWw..",
    "..GYGYGYGYGYGYGYG..",
];
/// The same from behind: the back of her head, her braids, the embroidery.
pub const HUIPIL_BACK: &[&str] = &[
    ".....Y...R...Y.....",
    "....VRVYVMVYVRV....",
    ".....KKKKKKKKK.....",
    ".....KKKKKKKKK.....",
    "....KKKKKKKKKKK....",
    "....KKKNKKKNKKK....",
    "....KKKKKKKKKKK....",
    "....KKKKKNKKKKK....",
    "..KNKKKKNKNKKKKNK..",
    "..RRKKKKRRRKKKKRR..",
    "..KN....sss....NK..",
    ".wKKYRGQMVMQGRYKKw.",
    ".wQQQWVWRWRWVWQQQw.",
    ".wKKkWWkWWWkWWkKKw.",
    "..KNWWWWWWWWWWWNK..",
    "..YYVRWWWWWWWRVYY..",
    "..KNRYRWVMVWRYRNK..",
    "..KKWRVWMYMWVRWKK..",
    "..KNWWWWWMWWWWWNK..",
    "..KKVQWWWWWWWQVKK..",
    "..RRQYQWVGVWQYQRR..",
    ".RRWWQVWGYGWVQWWRR.",
    "..wWWWWWWGWWWWWWw..",
    "..GYGYGYGYGYGYGYG..",
];

/// The width and height of the huipil's art, and the column of her axis.
pub const WIDTH: i32 = 19;
pub const BODY: i32 = 24;

/// What sets one woman apart: the colors of her skirt (the cloth, its
/// shadow, its band) and her skin (the light and the shade).
pub struct Look {
    pub skirt: [u8; 3],
    pub skin: [u8; 2],
}

pub const LOOKS: [Look; 6] = [
    Look {
        skirt: *b"GgY",
        skin: *b"Hh",
    },
    Look {
        skirt: *b"VvY",
        skin: *b"Ss",
    },
    Look {
        skirt: *b"MmR",
        skin: *b"Jj",
    },
    Look {
        skirt: *b"QqY",
        skin: *b"Hh",
    },
    Look {
        skirt: *b"RTY",
        skin: *b"Ss",
    },
    Look {
        skirt: *b"GgQ",
        skin: *b"Jj",
    },
];

/// How a woman stands: where the top-left pixel of her huipil's art is, how
/// many pixels an art pixel takes, how far she has turned (radians, 0 facing
/// front), how far her skirt flies out (0 to 1), how far its hem has swung
/// sideways and how long it is (in art pixels), and her feet.
#[derive(Clone, Copy)]
pub struct Stance {
    pub at: (i32, i32),
    pub k: f32,
    pub turn: f32,
    pub flare: f32,
    pub sway: f32,
    pub skirt: i32,
    /// How high each foot is lifted (left, right), in art pixels.
    pub feet: [f32; 2],
}

impl Stance {
    /// The column of the line down the middle of her.
    pub fn axis(&self) -> f32 {
        self.at.0 as f32 + WIDTH as f32 * 0.5 * self.k
    }

    /// Where a point of her is on the screen, given in art pixels: `dx` to
    /// the right of her axis (as she is seen from the front, and it turns
    /// with her) and `y` down from the top of her huipil.
    pub fn point(&self, dx: f32, y: f32) -> (f32, f32) {
        (
            self.axis() + dx * self.turn.cos() * self.k,
            self.at.1 as f32 + y * self.k,
        )
    }

    /// Where her skirt's hem is, in art pixels down from the top of her
    /// huipil.
    pub fn hem(&self) -> f32 {
        (BODY - 1 + self.skirt) as f32
    }
}

/// The half width of the skirt at row `r` of `len`, in art pixels.
fn skirt_half(r: i32, len: i32, flare: f32) -> f32 {
    let f = (r + 1) as f32 / len as f32;
    5.5 + (6.5 + 3.0 * flare) * f.powf(0.9)
}

/// Draws a woman: her skirt, her feet, and her huipil over them; her arms
/// and what she carries are the caller's, who knows what she is doing.
pub fn woman(c: &mut Canvas, pal: &Palette, look: &Look, s: &Stance) {
    let k = s.k;
    let cos = s.turn.cos();
    let [cloth, shade, band] = look.skirt;
    let axis = s.axis();
    let top = s.at.1 as f32 + (BODY - 2) as f32 * k;
    // The skirt, row by row: a cone, so its edges do not turn, but its pleats
    // do; they are the lines where it would be creased, seen round it.
    for r in 0..s.skirt {
        let half = skirt_half(r, s.skirt, s.flare) * k;
        let lean = s.sway * (r as f32 / s.skirt as f32).powf(1.5) * k;
        let (y0, y1) = (top + r as f32 * k, top + (r + 1) as f32 * k);
        for y in y0.round() as i32..y1.round() as i32 {
            for x in (axis + lean - half).round() as i32..=(axis + lean + half).round() as i32 {
                let u = ((x as f32 + 0.5 - axis - lean) / half.max(0.5)).clamp(-1.0, 1.0);
                let lon = u.asin() + s.turn;
                let pleat = (lon * 5.0 / std::f32::consts::PI).floor() as i32;
                let letter = if r < 1 {
                    shade
                } else if r >= s.skirt - 3 {
                    // The white lace of the enagua, scalloped at the hem.
                    let scallop = (lon * 7.0 / std::f32::consts::PI).rem_euclid(1.0) < 0.5;
                    if r == s.skirt - 1 && scallop {
                        b'w'
                    } else {
                        b'W'
                    }
                } else if r >= s.skirt - 6 && r < s.skirt - 4 {
                    band
                } else if pleat.rem_euclid(2) == 0 {
                    cloth
                } else {
                    shade
                };
                pixel(c, x, y, pal.of(letter));
            }
        }
    }
    // Her feet, in dark shoes, lifted by turns.
    let hem = s.at.1 as f32 + s.hem() * k;
    for (side, lift) in s.feet.into_iter().enumerate() {
        let off = if side == 0 { -3.5 } else { 3.5 } * cos * k;
        let (x, y) = (axis + off + s.sway * k * 0.9, hem - lift * k);
        let w = ((2.5 * k).round() as i32).max(1);
        block(
            c,
            x.round() as i32 - w / 2,
            y.round() as i32,
            w,
            pal.of(b'd'),
        );
        block(
            c,
            x.round() as i32 + w / 2 - 1,
            y.round() as i32,
            w / 2 + 1,
            pal.of(b'd'),
        );
    }
    // The huipil, seen from the front or the back, narrower as she turns, her
    // shoulders swinging against her hem.
    let back = cos < 0.0;
    let art = if back { HUIPIL_BACK } else { HUIPIL };
    let xf = Xf {
        pivot: (WIDTH as f32 * 0.5, 0.0),
        at: (axis, s.at.1 as f32),
        scale: (k * cos.abs().max(0.1), k),
        rot: 0.0,
        flip: false,
    };
    let lean = -0.35 * s.sway;
    blit(
        c,
        art,
        xf,
        |r| (lean * (1.0 - r as f32 / BODY as f32)).round() as i32,
        |l| match l {
            b'S' => pal.get(look.skin[0]),
            b's' => pal.get(look.skin[1]),
            _ => pal.get(l),
        },
    );
}

/// A bare arm from `from` to `to`, in a short sleeve of the huipil, a gold
/// bracelet at the wrist and a hand.
pub fn arm(c: &mut Canvas, pal: &Palette, look: &Look, from: (f32, f32), to: (f32, f32), k: f32) {
    let thick = ((2.0 * k).round() as i32).max(1);
    let at = |f: f32| (from.0 + (to.0 - from.0) * f, from.1 + (to.1 - from.1) * f);
    line(c, from, at(0.3), thick + 1, pal.of(b'W'));
    line(c, at(0.3), to, thick, pal.of(look.skin[0]));
    line(c, at(0.8), at(0.88), thick, pal.of(b'Y'));
    let hand = at(1.0);
    block(
        c,
        hand.0.round() as i32 - thick / 2,
        hand.1.round() as i32 - thick / 2,
        thick,
        pal.of(look.skin[0]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, rgb};
    use crate::eggs::{ALL, Eggs};
    use crate::fiesta::{Fiesta, Kind};

    /// The Guelaguetza visitors come from the Saturday before the first Lunes
    /// del Cerro to the second (which move with the calendar), and never on
    /// an ordinary day or during another holiday.
    #[test]
    fn the_visitors_keep_to_the_guelaguetza() {
        let days = [
            (2027, 7, 17),
            (2027, 7, 19),
            (2027, 7, 22),
            (2027, 7, 26),
            (2026, 7, 18),
            (2026, 7, 20),
            (2026, 7, 27),
        ];
        let others = [
            Fiesta::NONE,
            Fiesta::of_date(2027, 6, 15),
            Fiesta::of_date(2027, 7, 15),
            Fiesta::of_date(2027, 7, 27),
            Fiesta::of_date(2027, 2, 9),
            Fiesta::of_date(2027, 3, 27),
            Fiesta::of_date(2027, 9, 15),
            Fiesta::of_date(2027, 11, 2),
            Fiesta::of_date(2027, 12, 24),
        ];
        for name in ["pina", "calenda"] {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Guelaguetza, "{name}");
            for fiesta in others {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                assert!(!e.active(), "{name} came on {:?}", fiesta.kind);
            }
            for (y, m, d) in days {
                let fiesta = Fiesta::of_date(y, m, d);
                assert!(fiesta.is(Kind::Guelaguetza), "{y}-{m}-{d}");
                let mut e = Eggs::new(80, 24);
                e.start(&[info], fiesta);
                assert!(e.active(), "{name} on {y}-{m}-{d}");
            }
        }
    }

    #[test]
    fn every_letter_has_a_color_under_every_theme() {
        for name in ["classic", "dracula", "nord", "gruvbox", "mono"] {
            let Some(t) = Theme::builtin(name) else {
                continue;
            };
            let pal = Palette::guelaguetza(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::guelaguetza(&Theme::classic());
        assert_eq!(pal.of(b'G'), rgb(0x9b111e));
        assert_eq!(pal.of(b'Q'), rgb(0x00aab8));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::guelaguetza(&t);
        assert!(t.colors.contains(&pal.of(b'R')));
        assert!(t.colors.contains(&pal.of(b'V')));
    }

    #[test]
    fn her_art_is_whole() {
        for art in [HUIPIL, HUIPIL_BACK] {
            assert_eq!(art.len() as i32, BODY);
            assert!(art.iter().all(|r| r.len() as i32 == WIDTH));
        }
        let pal = Palette::guelaguetza(&Theme::classic());
        for l in LOOKS.iter().flat_map(|l| l.skirt.iter().chain(&l.skin)) {
            assert!(pal.get(*l).is_some(), "{}", *l as char);
        }
    }

    /// She stands up whole at any size, facing front, turned and from behind.
    #[test]
    fn she_is_drawn_at_any_size_and_turn() {
        let pal = Palette::guelaguetza(&Theme::classic());
        for (w, h) in [(1, 1), (30, 30), (80, 60)] {
            for k in [0.25, 1.0, 3.0] {
                let mut c = Canvas::new(w, h, Color::Reset);
                for turn in [0.0, 1.0, 1.6, 3.2, 4.5, 6.2] {
                    let s = Stance {
                        at: (-3, h as i32 / 2),
                        k,
                        turn,
                        flare: turn / 6.3,
                        sway: 3.0,
                        skirt: 18,
                        feet: [1.0, 0.0],
                    };
                    woman(&mut c, &pal, &LOOKS[1], &s);
                    arm(
                        &mut c,
                        &pal,
                        &LOOKS[1],
                        s.point(-8.0, 12.0),
                        s.point(-12.0, 4.0),
                        k,
                    );
                }
            }
        }
    }
}
