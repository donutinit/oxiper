//! dvd: the bouncing logo, hoping to hit a corner.
//!
//! The logo moves two columns per row so it travels at 45° on screen, changes
//! color on every bounce, and celebrates when it lands exactly in a corner.
//!
//! On Día de Muertos the logo is a calavera de azúcar: a bone-white sugar
//! skull with DVD cut into the colored foil on its forehead, flowers in its
//! eye sockets, a heart for a nose, and the icing changing color on every
//! bounce (papel picado colors under `classic`). It wears a crown of
//! cempasúchil that grows with the holiday, one flower tucked in on the
//! first day and the full crown on November 2, and it sheds marigold petals
//! as it goes; they drift down and lie along the bottom of the screen, a
//! path home that thickens as the days go by. A corner hit throws a burst
//! of blossoms and petals, and the banner reads "¡YA SE LO LLEVÓ LA
//! CATRINA!".

use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "dvd",
    aliases: &["bounce"],
    about: "the bouncing logo, hoping to hit a corner",
    options: &[
        Opt {
            key: "text",
            default: Val::S(""),
            help: "Bounce this text in a block font instead of the DVD logo",
        },
        Opt {
            key: "speed",
            default: Val::F(5.0),
            help: "Rows per second (columns move twice as fast, for a 45° path)",
        },
        Opt {
            key: "count",
            default: Val::I(1),
            help: "How many logos bounce around",
        },
        Opt {
            key: "corner_assist",
            default: Val::B(false),
            help: "Quietly nudge the path now and then so corners actually happen",
        },
        Opt {
            key: "counter",
            default: Val::B(true),
            help: "Show how many corners were hit",
        },
    ],
    make: |ctx| Box::new(Dvd::new(ctx)),
};

const LOGO: [&str; 6] = [
    "██████▄  ██▄   ▄██ ██████▄ ",
    "██   ███  ██▄ ▄██  ██   ███",
    "██████▀    ▀███▀   ██████▀ ",
    "      ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄      ",
    "▄███████ V I D E O ███████▄",
    " ▀▀▀▀▀███████████████▀▀▀▀▀ ",
];

/// The colors old DVD players cycled through.
const CLASSIC: [Color; 8] = [
    rgb(0xff2b2b),
    rgb(0xff8a00),
    rgb(0xffe100),
    rgb(0x3ce03c),
    rgb(0x00d5ff),
    rgb(0x2f6bff),
    rgb(0xb04bff),
    rgb(0xff3fb4),
];

/// Día de Muertos: the icing cycles through papel picado colors, ordered so
/// that each one and the one three places on (the eye flowers) contrast.
const PAPEL: [Color; 6] = [
    rgb(0xe4007c), // rosa mexicano
    rgb(0xff8c00), // naranja
    rgb(0x7028a0), // morado
    rgb(0xffd200), // amarillo
    rgb(0x00aab8), // turquesa
    rgb(0x8cc828), // verde limón
];
const HUESO: Color = rgb(0xf4eee2);
/// Cempasúchil and terciopelo, in pairs a tumbling petal alternates between.
const MARIGOLD: [Color; 6] = [
    rgb(0xff8c00),
    rgb(0xd66200),
    rgb(0xffaa1e),
    rgb(0xffc83c),
    rgb(0xb00048),
    rgb(0xd61e64),
];
const LEAF: Color = rgb(0x4e9a28);

/// The sugar skulls, one char per pixel and two pixel rows per cell: `w`
/// bone, `i` icing (the bounce color), `j` the eye flowers' petals, `.`
/// nothing. A cell never holds two colors: where bone meets icing in one,
/// the bone half stays dark, which outlines the icing.
const CALAVERA: [&str; 24] = [
    "..........wwwwwwwww..........",
    ".......wwwwwwwwwwwwwww.......",
    ".....wwwwwwwwwwwwwwwwwww.....",
    "....wwwwwwwwwwwwwwwwwwwww....",
    "...wwwiiiiiiiiiiiiiiiiiwww...",
    "..wwwii...ii.iii.i...iiiwww..",
    "..wwwii.ii.i.iii.i.ii.iiwww..",
    ".wwwwii.ii.ii.i.ii.ii.iiwwww.",
    ".wwwwii...iiii.iii...iiiwwww.",
    ".wwwwwiiiiiiiiiiiiiiiiiwwwww.",
    ".ww.......wwwwwwwww.......ww.",
    "ww....j....wwwwwww....j....ww",
    "ww...jjj...wwwwwww...jjj...ww",
    "ww..jjijj..wwwwwww..jjijj..ww",
    "ww..jjijj..wwwwwww..jjijj..ww",
    "ww...jjj...wwwwwww...jjj...ww",
    ".ww...j...wwiiwiiww...j...ww.",
    ".www.....wwwiiiiiwww.....www.",
    ".wwjwiwjwwwwwiiiwwwwwjwiwjww.",
    "..wwwwwwwwwwwwiwwwwwwwwwwww..",
    "...www.w.w.w.w.w.w.w.w.www...",
    "...www.w.w.w.w.w.w.w.w.www...",
    "....wwwwwwwwwwwwwwwwwwwww....",
    "......wwwwwwwwwwwwwwwww......",
];
const CALAVERITA: [&str; 20] = [
    "......wwwwwww......",
    "...wwwwwwwwwwwww...",
    "..wiiiiiiiiiiiiiw..",
    ".wii..ii.i.ii..iiw.",
    ".wii.i.i.i.ii.i.iw.",
    ".wii.i.i.i.ii.i.iw.",
    ".wii..iii.iii..iiw.",
    ".wwiiiiiiiiiiiiiww.",
    ".ww....wwwww....ww.",
    "ww..jj..www..jj..ww",
    "ww.jiij.www.jiij.ww",
    "ww.jiij.www.jiij.ww",
    ".w..jj..w.w..jj..w.",
    ".ww....ww.ww....ww.",
    ".wjwwwwwwwwwwwwwjw.",
    "..wwwwwwwwwwwwwww..",
    "...w.w.w.w.w.w.w...",
    "...w.w.w.w.w.w.w...",
    "....wwwwwwwwwww....",
    "......wwwwwww......",
];
const CALAQUITA: [&str; 10] = [
    "...wwwww...",
    ".wwwwwwwww.",
    ".wiiiiiiiw.",
    "wwiiiiiiiww",
    "w...www...w",
    "w.j.www.j.w",
    "w...w.w...w",
    ".wwwwwwwww.",
    "..w.w.w.w..",
    "...wwwww...",
];

/// A sugar skull: its pixel art, the rows that replace its last ones when
/// it laughs (the mouth open), and its crown.
struct Skull {
    art: &'static [&'static str],
    laugh: &'static [&'static str],
    crown: Option<&'static Crown>,
}

const SKULLS: [Skull; 3] = [
    Skull {
        art: &CALAVERA,
        laugh: &[
            "...www.w.w.w.w.w.w.w.w.www...",
            "...ww...................ww...",
            "....ww.w.w.w.w.w.w.w.w.ww....",
            "......wwwwwwwwwwwwwwwww......",
        ],
        crown: Some(&CROWN),
    },
    Skull {
        art: &CALAVERITA,
        laugh: &[
            "...w.w.w.w.w.w.w...",
            "...w...........w...",
            "....w.w.w.w.w.w....",
            "......wwwwwww......",
        ],
        crown: Some(&CROWNLET),
    },
    Skull {
        art: &CALAQUITA,
        laugh: &["..w.....w..", "...wwwww..."],
        crown: None,
    },
];

/// A crown of flowers for a skull: the shape of one flower (`x` marks it),
/// where each one sits (center column, top pixel row, counting two rows
/// above the skull; and its color: `o` cempasúchil, `y` light cempasúchil,
/// `r` terciopelo) in the order they come as the holiday builds up, and
/// the leaves (`g`) of the full crown.
struct Crown {
    flower: &'static [&'static str],
    flowers: &'static [(i32, i32, u8)],
    leaves: &'static [(i32, i32)],
}

const CROWN: Crown = Crown {
    flower: &[".xxx.", "xxxxx", "xxxxx", ".xxx."],
    flowers: &[
        (8, 2, b'o'),
        (14, 0, b'y'),
        (20, 2, b'o'),
        (3, 4, b'r'),
        (25, 4, b'r'),
    ],
    leaves: &[(11, 2), (17, 2), (5, 3), (23, 3)],
};
const CROWNLET: Crown = Crown {
    flower: &[".x.", "xxx", "xxx", ".x."],
    flowers: &[
        (5, 0, b'o'),
        (9, 0, b'y'),
        (13, 0, b'o'),
        (2, 2, b'r'),
        (16, 2, b'r'),
    ],
    leaves: &[(7, 1), (11, 1)],
};

/// The corner banner on Día de Muertos.
const LLEVO: [&str; 2] = ["¡Ya se lo llevó", "la Catrina!"];

/// 5×5 block font: one byte per row, bit 4 is the leftmost pixel.
const FONT: &[(char, [u8; 5])] = &[
    ('A', [0x0e, 0x11, 0x1f, 0x11, 0x11]),
    ('B', [0x1e, 0x11, 0x1e, 0x11, 0x1e]),
    ('C', [0x0f, 0x10, 0x10, 0x10, 0x0f]),
    ('D', [0x1e, 0x11, 0x11, 0x11, 0x1e]),
    ('E', [0x1f, 0x10, 0x1e, 0x10, 0x1f]),
    ('F', [0x1f, 0x10, 0x1e, 0x10, 0x10]),
    ('G', [0x0f, 0x10, 0x13, 0x11, 0x0f]),
    ('H', [0x11, 0x11, 0x1f, 0x11, 0x11]),
    ('I', [0x1f, 0x04, 0x04, 0x04, 0x1f]),
    ('J', [0x07, 0x01, 0x01, 0x11, 0x0e]),
    ('K', [0x11, 0x12, 0x1c, 0x12, 0x11]),
    ('L', [0x10, 0x10, 0x10, 0x10, 0x1f]),
    ('M', [0x11, 0x1b, 0x15, 0x11, 0x11]),
    ('N', [0x11, 0x19, 0x15, 0x13, 0x11]),
    ('O', [0x0e, 0x11, 0x11, 0x11, 0x0e]),
    ('P', [0x1e, 0x11, 0x1e, 0x10, 0x10]),
    ('Q', [0x0e, 0x11, 0x15, 0x12, 0x0d]),
    ('R', [0x1e, 0x11, 0x1e, 0x12, 0x11]),
    ('S', [0x0f, 0x10, 0x0e, 0x01, 0x1e]),
    ('T', [0x1f, 0x04, 0x04, 0x04, 0x04]),
    ('U', [0x11, 0x11, 0x11, 0x11, 0x0e]),
    ('V', [0x11, 0x11, 0x11, 0x0a, 0x04]),
    ('W', [0x11, 0x11, 0x15, 0x1b, 0x11]),
    ('X', [0x11, 0x0a, 0x04, 0x0a, 0x11]),
    ('Y', [0x11, 0x0a, 0x04, 0x04, 0x04]),
    ('Z', [0x1f, 0x02, 0x04, 0x08, 0x1f]),
    ('0', [0x0e, 0x13, 0x15, 0x19, 0x0e]),
    ('1', [0x04, 0x0c, 0x04, 0x04, 0x0e]),
    ('2', [0x1e, 0x01, 0x0e, 0x10, 0x1f]),
    ('3', [0x1e, 0x01, 0x06, 0x01, 0x1e]),
    ('4', [0x12, 0x12, 0x1f, 0x02, 0x02]),
    ('5', [0x1f, 0x10, 0x1e, 0x01, 0x1e]),
    ('6', [0x0e, 0x10, 0x1e, 0x11, 0x0e]),
    ('7', [0x1f, 0x01, 0x02, 0x04, 0x04]),
    ('8', [0x0e, 0x11, 0x0e, 0x11, 0x0e]),
    ('9', [0x0e, 0x11, 0x0f, 0x01, 0x0e]),
    ('!', [0x04, 0x04, 0x04, 0x00, 0x04]),
    ('?', [0x0e, 0x11, 0x06, 0x00, 0x04]),
    ('.', [0x00, 0x00, 0x00, 0x00, 0x04]),
    (',', [0x00, 0x00, 0x00, 0x04, 0x08]),
    (':', [0x00, 0x04, 0x00, 0x04, 0x00]),
    (';', [0x00, 0x04, 0x00, 0x04, 0x08]),
    ('-', [0x00, 0x00, 0x0e, 0x00, 0x00]),
    ('+', [0x00, 0x04, 0x0e, 0x04, 0x00]),
    ('=', [0x00, 0x1f, 0x00, 0x1f, 0x00]),
    ('_', [0x00, 0x00, 0x00, 0x00, 0x1f]),
    ('\'', [0x04, 0x04, 0x00, 0x00, 0x00]),
    ('"', [0x0a, 0x0a, 0x00, 0x00, 0x00]),
    ('/', [0x01, 0x02, 0x04, 0x08, 0x10]),
    ('\\', [0x10, 0x08, 0x04, 0x02, 0x01]),
    ('*', [0x15, 0x0e, 0x1f, 0x0e, 0x15]),
    ('#', [0x0a, 0x1f, 0x0a, 0x1f, 0x0a]),
    ('(', [0x02, 0x04, 0x04, 0x04, 0x02]),
    (')', [0x08, 0x04, 0x04, 0x04, 0x08]),
    ('[', [0x06, 0x04, 0x04, 0x04, 0x06]),
    (']', [0x0c, 0x04, 0x04, 0x04, 0x0c]),
    ('<', [0x02, 0x04, 0x08, 0x04, 0x02]),
    ('>', [0x08, 0x04, 0x02, 0x04, 0x08]),
    ('%', [0x19, 0x1a, 0x04, 0x0b, 0x13]),
    ('&', [0x0c, 0x12, 0x0d, 0x12, 0x0d]),
    ('@', [0x0e, 0x11, 0x17, 0x10, 0x0f]),
    ('$', [0x0f, 0x14, 0x0e, 0x05, 0x1e]),
    ('^', [0x04, 0x0a, 0x11, 0x00, 0x00]),
    ('~', [0x00, 0x08, 0x15, 0x02, 0x00]),
    ('♥', [0x0a, 0x1f, 0x1f, 0x0e, 0x04]),
];

/// A block font: the glyph for a char, if it has one.
type Font = fn(char) -> Option<[u8; 5]>;

fn glyph(ch: char) -> Option<[u8; 5]> {
    let up = ch.to_ascii_uppercase();
    FONT.iter().find(|(c, _)| *c == up).map(|(_, g)| *g)
}

/// The block font and what Spanish needs on top of it. Only the holiday's
/// banner uses it, so a `text` with `¡` or `ó` bounces as it always has.
fn glyph_es(ch: char) -> Option<[u8; 5]> {
    match ch {
        '¡' => Some([0x04, 0x00, 0x04, 0x04, 0x04]),
        'ó' | 'Ó' => Some([0x02, 0x0e, 0x11, 0x11, 0x0e]),
        _ => glyph(ch),
    }
}

/// Columns a glyph occupies (empty columns trimmed), as (first bit, width).
fn glyph_span(g: &[u8; 5]) -> (u32, u32) {
    let bits = g.iter().fold(0u8, |a, b| a | b);
    if bits == 0 {
        return (0, 0);
    }
    let hi = 7 - bits.leading_zeros(); // leftmost set bit (4 = column 0)
    let lo = bits.trailing_zeros();
    (hi, hi - lo + 1)
}

/// A small picture of characters; spaces are transparent.
#[derive(Default)]
struct Sprite {
    w: i32,
    h: i32,
    cells: Vec<char>,
    /// What each cell is a part of (a pixel letter of the skulls), each part
    /// in its own color; empty for a sprite in one color.
    parts: Vec<u8>,
}

impl Sprite {
    fn from_lines(lines: &[&str]) -> Sprite {
        let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let mut s = Sprite {
            w: w as i32,
            h: lines.len() as i32,
            cells: vec![' '; w * lines.len()],
            parts: Vec::new(),
        };
        for (y, l) in lines.iter().enumerate() {
            for (x, ch) in l.chars().enumerate() {
                s.cells[y * w + x] = ch;
            }
        }
        s
    }

    /// Plain one-line text.
    fn plain(text: &str) -> Sprite {
        Sprite::from_lines(&[text])
    }

    /// Width in font pixels of `text` set in the block font.
    fn text_px(text: &str, font: Font) -> i32 {
        let mut w = 0;
        for ch in text.chars() {
            w += match font(ch).or(font('?')) {
                _ if ch == ' ' => 3,
                Some(g) => glyph_span(&g).1 as i32 + 1,
                None => 0,
            };
        }
        (w - 1).max(0)
    }

    /// Renders `text` in the 5×5 block font. Scale 1 packs two pixel rows per
    /// cell with half blocks (square pixels, 3 rows tall); scale 2 uses a full
    /// block two columns wide per pixel (5 rows tall).
    fn from_text(text: &str, scale: i32, font: Font) -> Sprite {
        let pw = Sprite::text_px(text, font).max(1);
        let mut px = vec![false; pw as usize * 5];
        let mut x0 = 0;
        for ch in text.chars() {
            if ch == ' ' {
                x0 += 3;
                continue;
            }
            let Some(g) = font(ch).or(font('?')) else {
                continue;
            };
            let (hi, w) = glyph_span(&g);
            for (row, bits) in g.iter().enumerate() {
                for i in 0..w {
                    if bits >> (hi - i) & 1 == 1 && x0 + (i as i32) < pw {
                        px[row * pw as usize + (x0 as usize + i as usize)] = true;
                    }
                }
            }
            x0 += w as i32 + 1;
        }
        let at = |x: i32, y: i32| y < 5 && px[(y * pw + x) as usize];
        if scale >= 2 {
            let (w, h) = (pw * 2, 5);
            let mut cells = vec![' '; (w * h) as usize];
            for y in 0..h {
                for x in 0..w {
                    if at(x / 2, y) {
                        cells[(y * w + x) as usize] = '█';
                    }
                }
            }
            Sprite {
                w,
                h,
                cells,
                parts: Vec::new(),
            }
        } else {
            let (w, h) = (pw, 3);
            let mut cells = vec![' '; (w * h) as usize];
            for y in 0..h {
                for x in 0..w {
                    cells[(y * w + x) as usize] = match (at(x, y * 2), at(x, y * 2 + 1)) {
                        (true, true) => '█',
                        (true, false) => '▀',
                        (false, true) => '▄',
                        _ => ' ',
                    };
                }
            }
            Sprite {
                w,
                h,
                cells,
                parts: Vec::new(),
            }
        }
    }

    /// The biggest rendering of `text` that leaves room to move on a `sw`×`sh`
    /// screen; the big font is used when it takes at most `big`% of the width.
    fn fit_text(text: &str, sw: i32, sh: i32, big: i32) -> Sprite {
        let px = Sprite::text_px(text, glyph);
        // Keep at least twice the sprite's height free to bounce in.
        if px * 2 * 100 <= sw * big && sh >= 10 {
            Sprite::from_text(text, 2, glyph)
        } else if px + 2 <= sw && sh >= 4 {
            Sprite::from_text(text, 1, glyph)
        } else {
            Sprite::plain(text)
        }
    }

    fn draw(&self, c: &mut Canvas, x: i32, y: i32, fg: Color, attr: u8) {
        for row in 0..self.h {
            for col in 0..self.w {
                let ch = self.cells[(row * self.w + col) as usize];
                if ch != ' ' {
                    c.set_attr(x + col, y + row, ch, fg, attr);
                }
            }
        }
    }

    /// Draws a sprite with parts, each in the color `paint` gives it. Holes
    /// (`k`) are blanked, without a background, so nothing shows through.
    fn draw_parts(&self, c: &mut Canvas, x: i32, y: i32, paint: impl Fn(u8) -> Color) {
        for row in 0..self.h {
            for col in 0..self.w {
                let k = (row * self.w + col) as usize;
                match (self.cells[k], self.parts[k]) {
                    (' ', b'k') => c.set(x + col, y + row, ' ', Color::Reset),
                    (' ', _) => {}
                    (ch, part) => c.set(x + col, y + row, ch, paint(part)),
                }
            }
        }
    }

    /// Lines of text in the Spanish block font, centered under each other,
    /// all at the biggest scale that leaves them room on a `sw`×`sh` screen.
    fn fit_lines(lines: &[&str], sw: i32, sh: i32) -> Sprite {
        let px = lines
            .iter()
            .map(|l| Sprite::text_px(l, glyph_es))
            .max()
            .unwrap_or(0);
        let n = lines.len() as i32;
        let (rows, gap): (Vec<Sprite>, i32) = if px * 2 + 4 <= sw && sh >= 6 * n + 8 {
            let rows = lines.iter().map(|l| Sprite::from_text(l, 2, glyph_es));
            (rows.collect(), 1)
        } else if px + 2 <= sw && sh >= 4 * n + 4 {
            let rows = lines.iter().map(|l| Sprite::from_text(l, 1, glyph_es));
            (rows.collect(), 1)
        } else {
            (lines.iter().map(|l| Sprite::plain(l)).collect(), 0)
        };
        let w = rows.iter().map(|r| r.w).max().unwrap_or(0);
        let h = rows.iter().map(|r| r.h + gap).sum::<i32>() - gap;
        let mut s = Sprite {
            w,
            h: h.max(0),
            cells: vec![' '; (w * h.max(0)) as usize],
            parts: Vec::new(),
        };
        let mut y0 = 0;
        for r in &rows {
            let x0 = (w - r.w) / 2;
            for y in 0..r.h {
                for x in 0..r.w {
                    s.cells[((y0 + y) * w + x0 + x) as usize] = r.cells[(y * r.w + x) as usize];
                }
            }
            y0 += r.h + gap;
        }
        s
    }

    /// A sugar skull from its pixel art, laughing or not, crowned with as
    /// many flowers as the holiday has brought by `level`.
    fn calavera(skull: &Skull, laughing: bool, level: f32) -> Sprite {
        let (art, crown) = (skull.art, skull.crown);
        let w = art.iter().map(|l| l.len()).max().unwrap_or(0);
        let open = art.len().saturating_sub(skull.laugh.len());
        let blooms = match crown {
            None => 0,
            Some(c) if level < 0.4 => c.flowers.len().min(1),
            Some(c) if level < 0.75 => c.flowers.len().min(3),
            Some(c) => c.flowers.len(),
        };
        // Two pixel rows above the skull for the crown.
        let top = if blooms > 0 { 2 } else { 0 };
        let ph = art.len() + top;
        let mut px = vec![b'.'; w * ph];
        for (y, &l) in art.iter().enumerate() {
            let l = if laughing && y >= open {
                skull.laugh[y - open]
            } else {
                l
            };
            // Holes inside the skull (`k`) hide what passes behind it.
            let first = l.bytes().position(|b| b != b'.').unwrap_or(0);
            let last = l.bytes().rposition(|b| b != b'.').unwrap_or(0);
            for (x, b) in l.bytes().enumerate() {
                let inside = b == b'.' && (first..=last).contains(&x);
                px[(y + top) * w + x] = if inside { b'k' } else { b };
            }
        }
        let mut put = |x: i32, y: i32, b: u8| {
            if (0..w as i32).contains(&x) && (0..ph as i32).contains(&y) {
                px[y as usize * w + x as usize] = b;
            }
        };
        if let Some(c) = crown {
            if blooms == c.flowers.len() {
                for &(x, y) in c.leaves {
                    put(x, y, b'g');
                }
            }
            let fw = c.flower.first().map_or(0, |l| l.len()) as i32;
            for &(cx, y0, b) in &c.flowers[..blooms] {
                for (dy, l) in c.flower.iter().enumerate() {
                    for (dx, ch) in l.bytes().enumerate() {
                        if ch != b'.' {
                            put(cx - fw / 2 + dx as i32, y0 + dy as i32, b);
                        }
                    }
                }
            }
        }
        // Two pixels per cell; where two colors share one, the bone gives
        // way, or else the lower pixel.
        let h = ph.div_ceil(2);
        let mut s = Sprite {
            w: w as i32,
            h: h as i32,
            cells: vec![' '; w * h],
            parts: vec![0; w * h],
        };
        for y in 0..h {
            for x in 0..w {
                let mut t = px[2 * y * w + x];
                let mut b = px.get((2 * y + 1) * w + x).copied().unwrap_or(b'.');
                let hole = t == b'k' || b == b'k';
                if t == b'k' {
                    t = b'.';
                }
                if b == b'k' {
                    b = b'.';
                }
                if t != b'.' && b != b'.' && t != b {
                    if t == b'w' {
                        t = b'.';
                    } else {
                        b = b'.';
                    }
                }
                let (ch, part) = match (t, b) {
                    (b'.', b'.') if hole => (' ', b'k'),
                    (b'.', b'.') => (' ', 0),
                    (t, b'.') => ('▀', t),
                    (b'.', b) => ('▄', b),
                    (t, _) => ('█', t),
                };
                s.cells[y * w + x] = ch;
                s.parts[y * w + x] = part;
            }
        }
        s
    }
}

/// How a sugar skull is painted: its bone, its crown and the petals it
/// sheds. The icing and the eye flowers come from the logo's color.
#[derive(Clone, Copy)]
struct Dress {
    bone: Color,
    marigold: [Color; 6],
    leaf: Color,
}

/// A marigold petal, or a whole blossom, drifting down.
#[derive(Clone, Copy)]
struct Petal {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    ttl: f32,
    /// Where it is in its sway and tumble.
    phase: f32,
    /// Which pair of `Dress::marigold` it tumbles between.
    shade: u8,
    bloom: bool,
}

/// A petal lying on the bottom row, part of the path.
#[derive(Clone, Copy, Default)]
struct Fallen {
    /// Seconds left; 0 when nothing lies here.
    life: f32,
    shade: u8,
    bloom: bool,
}

/// Seconds a fallen petal stays at the start of the holiday, and at its
/// height.
const LIE: (f32, f32) = (14.0, 45.0);
const PETAL_GLYPHS: [char; 2] = ['◖', '◗'];
const BLOOM_GLYPHS: [char; 2] = ['✿', '❀'];

/// A petal's pair of shades: mostly cempasúchil, some terciopelo.
fn petal_shade(r: &mut fastrand::Rng) -> u8 {
    match r.u8(..20) {
        0..=8 => 0,
        9..=16 => 1,
        _ => 2,
    }
}

struct Logo {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: usize,
    since_bounce: f32,
    since_corner: f32,
}

#[derive(Clone, Copy)]
struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    ttl: f32,
    color: u8,
}

const SPARK_GLYPHS: [char; 5] = ['✦', '✧', '*', '+', '·'];
/// How long the "CORNER!" celebration lasts.
const PARTY: f32 = 2.4;

struct Dvd {
    rng: fastrand::Rng,
    text: String,
    speed: f32,
    count: usize,
    assist: bool,
    show_counter: bool,

    w: i32,
    h: i32,
    sprite: Sprite,
    /// The sugar skull laughing at a corner (empty when there is none).
    laugh: Sprite,
    banner: Sprite,
    logos: Vec<Logo>,
    sparks: Vec<Spark>,
    spark_cap: usize,
    corners: u32,
    /// Seconds since the last corner hit.
    party_t: f32,
    clock: f32,

    palette: Vec<Color>,
    counter_dim: Color,

    /// The holiday the logo is dressed for.
    fiesta: Fiesta,
    /// The holiday's own randomness, so ordinary days draw the same numbers.
    frng: fastrand::Rng,
    dress: Dress,
    petals: Vec<Petal>,
    petal_cap: usize,
    /// Petals owed to the trail, carried over between steps.
    shed: f32,
    /// What lies on the bottom row, per column.
    floor: Vec<Fallen>,
}

impl Dvd {
    fn new(ctx: &Ctx) -> Dvd {
        let o = ctx.opts;
        Dvd {
            rng: ctx.rng(),
            text: o.s("text").replace(['\n', '\t'], " ").trim().to_string(),
            speed: o.f("speed").clamp(0.0, 500.0),
            count: o.i("count").clamp(1, 64) as usize,
            assist: o.b("corner_assist"),
            show_counter: o.b("counter"),
            w: 1,
            h: 1,
            sprite: Sprite::default(),
            laugh: Sprite::default(),
            banner: Sprite::default(),
            logos: Vec::new(),
            sparks: Vec::new(),
            spark_cap: 0,
            corners: 0,
            party_t: PARTY,
            clock: 0.0,
            palette: CLASSIC.to_vec(),
            counter_dim: Color::Reset,
            fiesta: Fiesta::NONE,
            frng: fastrand::Rng::with_seed(ctx.seed ^ 0x6361_6c61_7665_7261),
            dress: Dress {
                bone: HUESO,
                marigold: MARIGOLD,
                leaf: LEAF,
            },
            petals: Vec::new(),
            petal_cap: 0,
            shed: 0.0,
            floor: Vec::new(),
        }
    }

    /// Picks the logo and the banner for the screen, the text and the holiday.
    fn fit(&mut self) {
        let (lw, lh) = (LOGO[4].chars().count() as i32, LOGO.len() as i32);
        let on = self.fiesta.on();
        self.laugh = Sprite::default();
        self.sprite = if !self.text.is_empty() {
            Sprite::fit_text(&self.text, self.w, self.h, 40)
        } else if on {
            let (s, laugh) = self.calavera();
            self.laugh = laugh;
            s
        } else if lw + 6 <= self.w && lh + 3 <= self.h {
            Sprite::from_lines(&LOGO)
        } else {
            Sprite::fit_text("DVD", self.w, self.h, 40)
        };
        self.banner = if on {
            Sprite::fit_lines(&LLEVO, self.w, self.h)
        } else {
            Sprite::fit_text("CORNER!", self.w, self.h, 50)
        };
        self.petals.clear();
        self.floor.clear();
        if on {
            self.petal_cap = 300 * self.count.min(3);
            self.petals.reserve(self.petal_cap);
            self.floor.resize(self.w as usize, Fallen::default());
        }
    }

    /// The biggest sugar skull that leaves room to bounce, and the same
    /// skull laughing.
    fn calavera(&self) -> (Sprite, Sprite) {
        let level = self.fiesta.level;
        for (i, skull) in SKULLS.iter().enumerate() {
            let s = Sprite::calavera(skull, false, level);
            let room = match i {
                0 => s.w + 6 <= self.w && s.h * 2 <= self.h,
                1 => s.w + 4 <= self.w && s.h + 4 <= self.h,
                _ => s.w + 2 <= self.w && s.h + 2 <= self.h,
            };
            if room {
                return (s, Sprite::calavera(skull, true, level));
            }
        }
        let dvd = Sprite::fit_text("DVD", self.w, self.h, 40);
        (dvd, Sprite::default())
    }

    fn max_x(&self) -> f32 {
        (self.w - self.sprite.w).max(0) as f32
    }

    fn max_y(&self) -> f32 {
        (self.h - self.sprite.h).max(0) as f32
    }

    fn other_color(&mut self, cur: usize) -> usize {
        let n = self.palette.len().max(1);
        if n == 1 {
            return 0;
        }
        (cur + 1 + self.rng.usize(..n - 1)) % n
    }

    /// With corner assist, bend the vertical speed a little (right at an x
    /// bounce, where nobody notices) so the next x wall arrives together with a
    /// y wall.
    fn plan(&mut self, i: usize) {
        let (mx, my) = (self.max_x(), self.max_y());
        let base = self.speed;
        let l = &mut self.logos[i];
        let sy = if l.vy < 0.0 { -1.0 } else { 1.0 };
        l.vy = sy * base;
        if !self.assist || mx <= 0.0 || my <= 0.0 || base <= 0.0 || self.rng.f32() > 0.5 {
            return;
        }
        let dist_x = if l.vx > 0.0 { mx - l.x } else { l.x };
        let travel = base * dist_x / l.vx.abs().max(1e-3);
        if travel < 1.0 {
            return;
        }
        let d0 = if l.vy > 0.0 { my - l.y } else { l.y };
        let k = ((travel - d0) / my).round().max(0.0);
        let ratio = (d0 + k * my) / travel;
        if (0.86..=1.14).contains(&ratio) {
            l.vy = sy * base * ratio;
        }
    }

    fn celebrate(&mut self, i: usize) {
        self.corners = self.corners.saturating_add(1);
        self.party_t = 0.0;
        let (mx, my) = (self.max_x(), self.max_y());
        let l = &self.logos[i];
        let left = l.x < mx * 0.5 || mx <= 0.0;
        let top = l.y < my * 0.5 || my <= 0.0;
        // Sparks leave from the logo's edges that face into the screen.
        let (lx, ly) = (l.x.round(), l.y.round());
        let (sw, sh) = (self.sprite.w as f32, self.sprite.h as f32);
        let edge_x = if left { lx + sw } else { lx - 1.0 };
        let edge_y = if top { ly + sh } else { ly - 1.0 };
        let (dx, dy) = (if left { 1.0 } else { -1.0 }, if top { 1.0 } else { -1.0 });
        if self.fiesta.on() {
            // Cempasúchil instead of sparks, and more of it as the days go by.
            let burst = self.petal_cap as f32 * (0.3 + 0.3 * self.fiesta.level);
            for _ in 0..burst as usize {
                if self.petals.len() >= self.petal_cap {
                    break;
                }
                let r = &mut self.frng;
                let (x, y) = if r.f32() * (sw + sh * 2.0) < sw {
                    (lx + r.f32() * sw, edge_y)
                } else {
                    (edge_x, ly + r.f32() * sh)
                };
                let a = r.f32() * std::f32::consts::FRAC_PI_2;
                let v = 3.0 + r.f32() * r.f32() * 24.0;
                self.petals.push(Petal {
                    x,
                    y,
                    vx: dx * a.cos() * v * 2.0,
                    vy: dy * a.sin() * v - 2.0,
                    age: 0.0,
                    ttl: 5.0 + r.f32() * 6.0,
                    phase: r.f32() * std::f32::consts::TAU,
                    shade: petal_shade(r),
                    bloom: r.f32() < 0.4,
                });
            }
            return;
        }
        let n = self.palette.len().max(1);
        let burst = (self.spark_cap * 2 / 3).max(1);
        for _ in 0..burst {
            if self.sparks.len() >= self.spark_cap {
                break;
            }
            // Pick the vertical or horizontal edge by its on-screen length.
            let (x, y) = if self.rng.f32() * (sw + sh * 2.0) < sw {
                (lx + self.rng.f32() * sw, edge_y)
            } else {
                (edge_x, ly + self.rng.f32() * sh)
            };
            // Spread over the quarter circle that points into the screen.
            let a = self.rng.f32() * std::f32::consts::FRAC_PI_2;
            let v = 4.0 + self.rng.f32() * self.rng.f32() * 30.0;
            self.sparks.push(Spark {
                x,
                y,
                vx: dx * a.cos() * v * 2.0,
                vy: dy * a.sin() * v,
                age: 0.0,
                ttl: 1.0 + self.rng.f32() * 1.4,
                color: self.rng.usize(..n) as u8,
            });
        }
    }

    fn step(&mut self, dt: f32) {
        let (mx, my) = (self.max_x(), self.max_y());
        for i in 0..self.logos.len() {
            let (mut bx, mut by) = (false, false);
            {
                let l = &mut self.logos[i];
                l.since_bounce += dt;
                l.since_corner += dt;
                if mx > 0.0 {
                    l.x += l.vx * dt;
                    if l.x < 0.0 {
                        l.x = -l.x;
                        l.vx = l.vx.abs();
                        bx = true;
                    } else if l.x > mx {
                        l.x = 2.0 * mx - l.x;
                        l.vx = -l.vx.abs();
                        bx = true;
                    }
                    l.x = l.x.clamp(0.0, mx);
                }
                if my > 0.0 {
                    l.y += l.vy * dt;
                    if l.y < 0.0 {
                        l.y = -l.y;
                        l.vy = l.vy.abs();
                        by = true;
                    } else if l.y > my {
                        l.y = 2.0 * my - l.y;
                        l.vy = -l.vy.abs();
                        by = true;
                    }
                    l.y = l.y.clamp(0.0, my);
                }
            }
            if !(bx || by) {
                continue;
            }
            let (x, y, since_bounce, since_corner) = {
                let l = &self.logos[i];
                (l.x, l.y, l.since_bounce, l.since_corner)
            };
            // A corner: both walls at once, or the other wall visually touched.
            let near_x = mx <= 0.0 || x < 1.0 || x > mx - 1.0;
            let near_y = my <= 0.0 || y < 0.5 || y > my - 0.5;
            let corner = (bx || near_x) && (by || near_y) && mx > 0.0 && my > 0.0;
            if since_bounce > 0.15 {
                let c = self.other_color(self.logos[i].color);
                self.logos[i].color = c;
            }
            self.logos[i].since_bounce = 0.0;
            if corner && since_corner > 1.0 {
                self.logos[i].since_corner = 0.0;
                self.celebrate(i);
            }
            if bx {
                self.plan(i);
            }
        }

        for s in self.sparks.iter_mut() {
            s.age += dt;
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            s.vy += 9.0 * dt;
            s.vx *= 1.0 - 0.9 * dt;
        }
        self.sparks.retain(|s| s.age < s.ttl);
        if self.fiesta.on() {
            self.drift(dt);
        }
        self.party_t = (self.party_t + dt).min(PARTY);
        self.clock = (self.clock + dt) % 3600.0;
    }

    /// Día de Muertos: the skulls shed petals from their trailing side, and
    /// the petals drift down, swaying, to lie on the bottom row.
    fn drift(&mut self, dt: f32) {
        let level = self.fiesta.level;
        let (sw, sh) = (self.sprite.w as f32, self.sprite.h as f32);
        self.shed += dt * (0.8 + 3.6 * level) * self.logos.len().min(3) as f32;
        while self.shed >= 1.0 {
            self.shed -= 1.0;
            if self.petals.len() >= self.petal_cap || self.logos.is_empty() {
                continue;
            }
            let r = &mut self.frng;
            let l = &self.logos[r.usize(..self.logos.len())];
            let back = if l.vx > 0.0 { l.x } else { l.x + sw - 1.0 };
            self.petals.push(Petal {
                x: back + (r.f32() - 0.5) * 2.0,
                y: l.y + r.f32() * sh,
                vx: -l.vx.signum() * (0.5 + r.f32() * 2.0),
                vy: r.f32() * 1.5,
                age: 0.0,
                ttl: 10.0 + r.f32() * 8.0,
                phase: r.f32() * std::f32::consts::TAU,
                shade: petal_shade(r),
                bloom: r.f32() < 0.12,
            });
        }
        let (w, floor_y) = (self.w as f32, (self.h - 1) as f32);
        let lie = LIE.0 + (LIE.1 - LIE.0) * level;
        let floor = &mut self.floor;
        let drag = (1.0 - 1.6 * dt).max(0.0);
        self.petals.retain_mut(|p| {
            p.age += dt;
            // The burst spends itself, then the petals fall like petals do:
            // slowly, swaying, blossoms a little faster.
            p.vx *= drag;
            let fall = if p.bloom { 3.2 } else { 2.2 };
            p.vy += (fall - p.vy) * (1.2 * dt).min(1.0);
            let sway = (p.age * 1.9 + p.phase).sin() * 2.4;
            p.x += (p.vx + sway) * dt;
            p.y += p.vy * dt;
            if p.y >= floor_y {
                let x = p.x.round();
                if (0.0..w).contains(&x) {
                    floor[x as usize] = Fallen {
                        life: lie * (0.7 + 0.3 * (p.phase / std::f32::consts::TAU)),
                        shade: p.shade,
                        bloom: p.bloom,
                    };
                }
                return false;
            }
            p.age < p.ttl && p.x > -2.0 && p.x < w + 2.0
        });
    }

    /// The petals in the air and the path they make on the bottom row.
    fn draw_petals(&self, ctx: &Ctx, c: &mut Canvas) {
        let m = &self.dress.marigold;
        for (x, f) in self.floor.iter().enumerate() {
            if f.life <= 0.0 {
                continue;
            }
            // Lying petals face up or down by where they fell.
            let side = (x * 7 + f.shade as usize).is_multiple_of(3);
            let g = if f.bloom {
                BLOOM_GLYPHS[side as usize]
            } else {
                PETAL_GLYPHS[side as usize]
            };
            let col = m[f.shade as usize * 2 + side as usize];
            let col = ctx.fade(col, (1.0 - f.life / 4.0).clamp(0.0, 1.0));
            c.set(x as i32, self.h - 1, g, col);
        }
        for p in &self.petals {
            // A petal tumbles: its two faces, in two shades, take turns.
            let face = ((p.age * 2.6 + p.phase) as i32 & 1) as usize;
            let g = if p.bloom {
                BLOOM_GLYPHS[face]
            } else {
                PETAL_GLYPHS[face]
            };
            let col = m[p.shade as usize * 2 + face];
            let k = ((p.age / p.ttl - 0.75) * 4.0).clamp(0.0, 1.0);
            c.set(p.x.round() as i32, p.y.round() as i32, g, ctx.fade(col, k));
        }
    }

    fn draw_counter(&self, c: &mut Canvas) {
        let mut buf = [0u8; 24];
        let prefix: &[u8] = if self.fiesta.on() {
            b"esquinas: "
        } else {
            b"corners: "
        };
        buf[..prefix.len()].copy_from_slice(prefix);
        let mut n = self.corners;
        let mut digits = [0u8; 10];
        let mut nd = 0;
        loop {
            digits[nd] = b'0' + (n % 10) as u8;
            nd += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        for k in 0..nd {
            buf[prefix.len() + k] = digits[nd - 1 - k];
        }
        let len = prefix.len() + nd;
        let s = std::str::from_utf8(&buf[..len]).unwrap_or("");
        let x = self.w - len as i32 - 1;
        let y = self.h - 1;
        let fresh = self.party_t < PARTY;
        let col = if fresh {
            self.palette[(self.clock * 10.0) as usize % self.palette.len()]
        } else {
            self.counter_dim
        };
        c.text_attr(x.max(0), y, s, col, if fresh { BOLD } else { 0 });
    }
}

impl Scene for Dvd {
    fn resize(&mut self, ctx: &Ctx, w: u16, h: u16) {
        self.w = w.max(1) as i32;
        self.h = h.max(1) as i32;
        self.fiesta = ctx.fiesta;
        self.fit();
        let (mx, my) = (self.max_x(), self.max_y());
        let n = self.palette.len().max(1);
        if self.logos.len() == self.count {
            // A resize keeps the logos where they were, as far as they fit.
            for l in self.logos.iter_mut() {
                l.x = l.x.clamp(0.0, mx);
                l.y = l.y.clamp(0.0, my);
            }
            self.sparks.clear();
            return;
        }
        self.logos.clear();
        for _ in 0..self.count {
            let l = Logo {
                x: self.rng.f32() * mx,
                y: self.rng.f32() * my,
                vx: if self.rng.bool() { 2.0 } else { -2.0 } * self.speed,
                vy: if self.rng.bool() { 1.0 } else { -1.0 } * self.speed,
                color: self.rng.usize(..n),
                since_bounce: 1.0,
                since_corner: 10.0,
            };
            self.logos.push(l);
        }
        self.spark_cap = 180 * self.count.min(3);
        self.sparks = Vec::with_capacity(self.spark_cap);
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.palette.clear();
        let on = ctx.fiesta.on();
        if ctx.theme.native {
            self.palette
                .extend_from_slice(if on { &PAPEL[..] } else { &CLASSIC[..] });
            self.counter_dim = ctx.fade(rgb(0xb0b0b0), 0.45);
        } else {
            let n = ctx.theme.colors.len().max(1);
            self.palette
                .extend((0..n).map(|i| ctx.pick(i, Color::Reset)));
            self.counter_dim = ctx.fade(ctx.theme.fg, 0.45);
        }
        // The bone is the theme's own ink; the marigolds keep their hue.
        self.dress.bone = if ctx.theme.native {
            HUESO
        } else {
            ctx.theme.fg
        };
        if ctx.fiesta != self.fiesta {
            // The holiday came or went (or moved on a day): dress again.
            self.fiesta = ctx.fiesta;
            self.fit();
            let (mx, my) = (self.max_x(), self.max_y());
            for l in self.logos.iter_mut() {
                l.x = l.x.clamp(0.0, mx);
                l.y = l.y.clamp(0.0, my);
            }
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let mut left = dt.clamp(0.0, 2.0);
        while left > 0.0 {
            let h = left.min(1.0 / 60.0);
            self.step(h);
            left -= h;
        }
    }

    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        // The path of petals fades on the wall clock.
        let real = real.clamp(0.0, 5.0);
        for f in self.floor.iter_mut() {
            f.life = (f.life - real).max(0.0);
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let n = self.palette.len().max(1);
        if self.party_t < PARTY {
            // Blink fast at first, then hold and fade out.
            let t = self.party_t;
            let on = t > 1.2 || (t * 7.0) as i32 % 2 == 0;
            if on {
                let col = self.palette[(t * 9.0) as usize % n];
                let col = if t > PARTY - 0.6 {
                    ctx.fade(col, (t - (PARTY - 0.6)) / 0.6)
                } else {
                    col
                };
                let b = &self.banner;
                b.draw(c, (self.w - b.w) / 2, (self.h - b.h) / 2, col, BOLD);
            }
        }
        for s in &self.sparks {
            let k = s.age / s.ttl;
            let mut gi = (k * SPARK_GLYPHS.len() as f32) as usize;
            // Twinkle: now and then a spark flares back up for a frame.
            if ((s.ttl * 97.0 + s.age * 23.0) as u32).is_multiple_of(7) {
                gi = gi.saturating_sub(2);
            }
            let g = SPARK_GLYPHS[gi.min(SPARK_GLYPHS.len() - 1)];
            let col = ctx.fade(self.palette[s.color as usize % n], k * 0.7);
            c.set(s.x.round() as i32, s.y.round() as i32, g, col);
        }
        if self.fiesta.on() {
            self.draw_petals(ctx, c);
        }
        // At a corner the skulls laugh, their jaws chattering.
        let t = self.party_t;
        let laughing = t < 1.8 && (t * 9.0) as i32 % 2 == 1 && !self.laugh.parts.is_empty();
        let skull = if laughing { &self.laugh } else { &self.sprite };
        for l in &self.logos {
            let (x, y) = (l.x.round() as i32, l.y.round() as i32);
            let icing = self.palette[l.color % n];
            if skull.parts.is_empty() {
                self.sprite.draw(c, x, y, icing, 0);
                continue;
            }
            // The eye flowers take the color across the palette.
            let petal = self.palette[(l.color + n / 2) % n];
            let d = &self.dress;
            skull.draw_parts(c, x, y, |part| match part {
                b'i' => icing,
                b'j' => petal,
                b'o' => d.marigold[0],
                b'y' => d.marigold[3],
                b'r' => d.marigold[4],
                b'g' => d.leaf,
                _ => d.bone,
            });
        }
        // A HUD on top of everything, so a corner hit never hides it.
        if self.show_counter {
            self.draw_counter(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the scene with `opts` through odd sizes, huge steps and themes.
    fn torture(opts: &str, fiesta: Fiesta) {
        let table: toml::Table = opts.parse().unwrap();
        let o = crate::scene::Opts::new(INFO.options, table);
        for (theme, bg) in [("classic", Color::Reset), ("nord", Color::Rgb(20, 20, 30))] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = Ctx {
                theme: &th,
                opts: &o,
                bg,
                seed: 11,
                fiesta,
            };
            let mut s = (INFO.make)(&ctx);
            for &(w, h) in &[(1u16, 1u16), (2, 3), (500, 150), (3, 1), (1, 40), (37, 11)] {
                s.resize(&ctx, w, h);
                s.retheme(&ctx);
                let mut c = Canvas::new(w, h, Color::Reset);
                for i in 0..60 {
                    let dt = if i % 20 == 19 { 5.0 } else { 1.0 / 30.0 };
                    s.tick(&ctx, dt, dt);
                    c.clear();
                    s.draw(&ctx, &mut c);
                    let clean = c.cells().iter().all(|cell| cell.bg == Color::Reset);
                    assert!(clean || opts.contains("_bg = true"), "{opts}");
                }
            }
        }
    }

    #[test]
    fn survives_options() {
        for fiesta in [Fiesta::NONE, Fiesta::of_day(11, 2), Fiesta::of_day(10, 28)] {
            torture("", fiesta);
            torture("count = 64\nspeed = 500.0\ncorner_assist = true", fiesta);
            torture(
                "text = 'Hello, wörld ♥ 123 !?'\nspeed = 0.0\ncounter = false",
                fiesta,
            );
            torture(
                "text = '   '\ncount = 0\ncorner_assist = true\nspeed = 40.0",
                fiesta,
            );
            torture(
                "text = 'a very long line of text that will never fit anywhere at all on small screens'",
                fiesta,
            );
        }
    }

    #[test]
    fn logo_rows_line_up() {
        let w = LOGO[4].chars().count();
        for l in LOGO {
            assert!(l.chars().count() <= w, "{l}");
        }
    }

    #[test]
    fn font_renders() {
        let s = Sprite::from_text("Hi!", 1, glyph);
        assert_eq!(s.h, 3);
        assert_eq!(s.w, Sprite::text_px("Hi!", glyph));
        let s = Sprite::from_text("A", 2, glyph);
        assert_eq!((s.w, s.h), (10, 5));
        // Spanish is only for the holiday's banner.
        assert_eq!(glyph('¡'), None);
        assert!(glyph_es('¡').is_some() && glyph_es('ó').is_some());
    }

    #[test]
    fn skulls_line_up() {
        for skull in &SKULLS {
            let w = skull.art[0].len();
            for l in skull.art.iter().chain(skull.laugh) {
                assert_eq!(l.len(), w, "{l}");
            }
            assert!(skull.laugh.len() < skull.art.len());
            for level in [0.25, 0.6, 1.0] {
                let s = Sprite::calavera(skull, false, level);
                let laugh = Sprite::calavera(skull, true, level);
                assert_eq!((s.w, s.h), (laugh.w, laugh.h));
                assert_eq!(s.cells.len(), s.parts.len());
                assert_ne!(s.cells, laugh.cells);
            }
        }
    }

    fn ctx<'a>(th: &'a crate::theme::Theme, o: &'a crate::scene::Opts, f: Fiesta) -> Ctx<'a> {
        Ctx {
            theme: th,
            opts: o,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta: f,
        }
    }

    /// Runs the scene for `secs` and draws it.
    fn play(d: &mut Dvd, ctx: &Ctx, secs: f32) -> Canvas {
        for _ in 0..(secs * 30.0) as usize {
            d.tick(ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
        let mut c = Canvas::new(d.w as u16, d.h as u16, Color::Reset);
        d.draw(ctx, &mut c);
        c
    }

    fn bones(c: &Canvas, bone: Color) -> usize {
        c.cells().iter().filter(|cell| cell.fg == bone).count()
    }

    fn flowers(d: &Dvd) -> usize {
        let crown = *b"oyr";
        d.sprite.parts.iter().filter(|p| crown.contains(p)).count()
    }

    #[test]
    fn muertos_dresses_the_logo() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let bone = if th.native { HUESO } else { th.fg };
            let mut crowns = Vec::new();
            for f in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
                let ctx = ctx(&th, &o, f);
                let mut d = Dvd::new(&ctx);
                d.resize(&ctx, 120, 36);
                d.retheme(&ctx);
                assert!(!d.sprite.parts.is_empty(), "a sugar skull");
                crowns.push(flowers(&d));
                let c = play(&mut d, &ctx, 20.0);
                assert!(bones(&c, bone) > 100, "{theme}");
                assert!(!d.petals.is_empty(), "petals fall from it");
                assert!(d.floor.iter().any(|f| f.life > 0.0), "and lie on the floor");
                // A corner: a burst of petals, the skull laughs, the banner.
                let before = d.petals.len();
                d.celebrate(0);
                assert!(d.petals.len() > before + 20);
                let banner = Sprite::fit_lines(&LLEVO, 120, 36);
                assert_eq!((d.banner.h, &d.banner.cells), (7, &banner.cells));
                let c = play(&mut d, &ctx, 0.5);
                assert!(c.to_text().contains("esquinas: "));
            }
            // The crown grows as the holiday builds up.
            assert!(crowns[0] > 0 && crowns[1] > crowns[0], "{crowns:?}");
        }
    }

    #[test]
    fn ordinary_days_stay_plain() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let ctx = ctx(&th, &o, Fiesta::NONE);
        let mut d = Dvd::new(&ctx);
        d.resize(&ctx, 120, 36);
        d.retheme(&ctx);
        assert!(d.sprite.parts.is_empty() && d.laugh.cells.is_empty());
        assert_eq!(d.sprite.w, LOGO[4].chars().count() as i32);
        d.celebrate(0);
        let c = play(&mut d, &ctx, 1.0);
        assert_eq!(bones(&c, HUESO), 0);
        assert!(d.petals.is_empty() && d.floor.is_empty() && !d.sparks.is_empty());
        assert!(c.to_text().contains("corners: 1"));
    }

    #[test]
    fn the_holiday_comes_and_goes_through_retheme() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let plain = ctx(&th, &o, Fiesta::NONE);
        let muertos = ctx(&th, &o, Fiesta::of_day(11, 1));
        let mut d = Dvd::new(&plain);
        d.resize(&plain, 100, 30);
        d.retheme(&plain);
        play(&mut d, &plain, 2.0);
        d.retheme(&muertos);
        assert!(!d.sprite.parts.is_empty());
        assert_eq!(d.palette, PAPEL.to_vec());
        let c = play(&mut d, &muertos, 3.0);
        assert!(bones(&c, HUESO) > 50 && !d.petals.is_empty());
        for l in &d.logos {
            assert!(l.x <= d.max_x() && l.y <= d.max_y());
        }
        d.retheme(&plain);
        assert!(d.sprite.parts.is_empty() && d.petals.is_empty() && d.floor.is_empty());
        assert_eq!(d.palette, CLASSIC.to_vec());
        let c = play(&mut d, &plain, 1.0);
        assert_eq!(bones(&c, HUESO), 0);
    }
}
