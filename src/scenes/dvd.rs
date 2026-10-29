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
//!
//! Each holiday that dresses the logo is a [`Look`] in a file of its own:
//! pixel art at a few sizes, how its parts are colored (the holiday's colors
//! under `classic`, the theme's otherwise), what it does and says at a corner
//! and what it sheds and throws.
//!
//! In the Navidad the logo is a piñata de siete picos, each pico in a papel
//! de china color that changes on every bounce, hanging from its rope. It
//! sheds colación as it goes, more as the posadas climb to Nochebuena, and
//! the candy lies along the bottom row for a while. At a corner somebody
//! swings the stick: the piñata lurches three times ("¡DALE, DALE, DALE!"),
//! bursts into candy and fruit (tejocotes, mandarinas, cañas, cacahuates,
//! caramelos), and a new piñata comes. On the Santos Inocentes it slows down
//! as it nears a corner and misses it by a cell: "¡INOCENTE PALOMITA!". On
//! Año Nuevo the banner is "¡FELIZ AÑO NUEVO!" and grapes, confetti and
//! sparks of gold come out of it. From January 2 to 6 the logo is a rosca de
//! reyes, with more candied fruit on it as the Reyes come closer; at a
//! corner the muñeco shows in its hole: "¡TE SALIÓ EL MUÑECO!".
//!
//! In the Fiestas Patrias the logo is a sombrero de charro with gold
//! embroidery and a band in the colors of the flag, which a bounce turns
//! around the crown a stripe. It sheds papelitos verde, blanco y rojo that
//! pile up along the bottom row, more as the month goes on, and banderitas
//! go up on its crown (one in the first week, three after that). At a corner
//! the sombrero is tipped, its brim curling up in a smile, a burst of
//! papelitos and sparks of gold flies off and the banner reads "¡VIVA
//! MÉXICO!".
//!
//! On Candelaria the logo is a tamal in its hoja de maíz, tied at both
//! ends with a cinta that takes the colors of the Niño Dios' gown (azul,
//! dorado, blanco) in turn, steam curling up from it. At a corner the hoja
//! opens on the masa and a stripe of salsa roja, hojas, maíz, steam and
//! sparks of a vela fly off, and the banner reads "¡TE TOCAN LOS TAMALES!".
//!
//! On Día del Niño the logo is a trompo, its painted stripes (in toy colors,
//! changing on every bounce) sliding around it as it spins on its púa. At a
//! corner it spins three times as fast for a moment while canicas, estrellas
//! and caramelos shoot out of it and bounce down to lie along the bottom
//! row, and the banner reads "¡FELIZ DÍA DEL NIÑO!".
//!
//! On the Día de la Revolución the logo is a wide sombrero of manta over two
//! crossed cananas, brass cartuchos in their straps, with a band that is
//! rojo or verde, whichever way the last bounce went, making the tricolor
//! with the hat. At a corner the sombrero is raised, its brim curling up
//! and the brass gleaming, confeti, sparks of brass and the dust of the road
//! fly off, and the banner reads "¡TIERRA Y LIBERTAD!".

mod candelaria;
mod muertos;
mod navidad;
mod nino;
mod patrias;
mod revolucion;

use crate::color::{Color, rgb};
use crate::fiesta::{Fiesta, Kind};
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

// What the logo becomes on a holiday. Each holiday has a `Look` (in its own
// file): pixel art at a few sizes, how its parts are colored, the banner it
// shows at a corner and what it sheds and throws.

/// How a part of a logo is colored; the pictures give each of their pixel
/// letters one.
#[derive(Clone, Copy)]
enum Tint {
    /// Keeps its own color under every theme (the marigolds).
    Own(Color),
    /// Its own color under `classic`, the theme's text color otherwise.
    Ink(Color),
    /// Its own color under `classic`, the theme's text color faded otherwise.
    Dim(Color),
    /// Its own color under `classic`, accent `i` of the theme otherwise.
    Accent(usize, Color),
    /// The logo's bounce color, that many places on in the palette.
    Cycle(u8),
    /// The bounce color from the middle of the palette: the one that contrasts
    /// with `Cycle(0)`.
    Across,
}

/// What a logo does when it hits a corner.
#[derive(Clone, Copy, PartialEq)]
enum React {
    /// Its reaction picture and the plain one alternate, a laugh.
    Chatter,
    /// Its reaction picture shows for a moment.
    Hold,
    /// Its frames play faster for a moment.
    Spin,
    /// It shakes at each "¡dale!", bursts, and comes back whole: its
    /// reaction picture is what is left of it.
    Pop,
}

/// A logo's picture at one size, one char per pixel and two pixel rows per
/// cell, `.` for nothing, every other letter but `k` a part (see
/// `Look::parts`; `k` marks the holes of a solid logo). A
/// cell holds one color: where two share it, the one that is not `Look::back`
/// wins, or else the lower pixel.
struct Size {
    /// The frames, as many as the logo has (a spinning top, steam rising);
    /// they are as big as each other.
    frames: &'static [&'static [&'static str]],
    /// The rows that replace the last rows of the first frame while it reacts
    /// to a corner; none for a logo that does not change.
    react: &'static [&'static str],
    trim: Option<&'static Trim>,
}

/// What dresses a logo more as the holiday builds up: pieces of the same
/// shape in the places they take, in the order they come, one, then three,
/// then all of them.
struct Trim {
    /// One piece, where `x` is the piece's own letter and any other letter
    /// itself.
    piece: &'static [&'static str],
    /// Where each sits (center column, top pixel row) and its letter.
    pieces: &'static [(i32, i32, u8)],
    /// Single pixels (column, row) only the full trim has, of letter `leaf`.
    leaves: &'static [(i32, i32)],
    leaf: u8,
    /// Pixel rows the trim adds above the picture, once there are pieces.
    above: i32,
}

/// What a bit is made of: the two faces it shows as it tumbles, and how fast
/// it falls.
struct BitKind {
    glyphs: [char; 2],
    tints: [Tint; 2],
    /// Rows per second it ends up falling at.
    fall: f32,
}

impl BitKind {
    const fn new(glyphs: [char; 2], tints: [Tint; 2], fall: f32) -> BitKind {
        BitKind {
            glyphs,
            tints,
            fall,
        }
    }
}

/// How the air treats a bit.
struct Air {
    /// How fast what a burst gave it is spent, per second.
    drag: f32,
    /// How fast it comes to its falling speed, per second.
    settle: f32,
    /// A pull down on top of that, rows per second squared.
    gravity: f32,
    /// How far it sways, columns per second.
    sway: f32,
}

/// What a logo sheds as it goes and throws at a corner: bits that fall and
/// lie along the bottom row for a while.
struct Fall {
    kinds: &'static [BitKind],
    /// Picks the kind of a new bit, thrown at a corner (true) or shed on the
    /// way (false); the random numbers are the holiday's own.
    pick: fn(&mut fastrand::Rng, bool) -> u8,
    /// How many bits can be about, per logo (up to three).
    cap: usize,
    /// Seconds a fallen bit lies at the start of the holiday and at its
    /// height.
    lie: (f32, f32),
    /// Bits shed per second by each logo: at the start of the holiday, and
    /// what its height adds.
    shed: (f32, f32),
    /// The share of the room for bits that a corner throws: likewise.
    burst: (f32, f32),
    /// How fast a thrown bit flies: the least, and what chance adds.
    speed: (f32, f32),
    /// Seconds a bit stays in the air, thrown and shed: the least, and what
    /// chance adds.
    ttl: ((f32, f32), (f32, f32)),
    air: Air,
}

impl Fall {
    fn kind(&self, k: u8) -> &BitKind {
        &self.kinds[k as usize % self.kinds.len()]
    }
}

/// What the logo is on a holiday.
struct Look {
    /// Its pictures, the biggest first.
    sizes: &'static [Size],
    /// How each letter of the pictures is colored.
    parts: &'static [(u8, Tint)],
    /// The letter that gives way where two parts share a cell.
    back: u8,
    /// Do the empty pixels inside the picture hide what passes behind?
    solid: bool,
    /// What the bounce color cycles through under `classic`.
    classic: &'static [Color],
    /// The corner banner, a line to a row.
    banner: &'static [&'static str],
    react: React,
    /// Frames per second, when it has more than one.
    beat: f32,
    fall: Fall,
    /// The banner of a near miss: if set, the logo now and then slows near a
    /// corner and misses it by a cell.
    joke: Option<&'static [&'static str]>,
}

/// The look of the day, if the holiday dresses the logo.
fn look_for(f: Fiesta) -> Option<&'static Look> {
    match f.kind {
        Kind::Candelaria => Some(&candelaria::LOOK),
        Kind::Muertos => Some(&muertos::LOOK),
        Kind::Navidad => Some(navidad::look(f)),
        Kind::Nino => Some(&nino::LOOK),
        Kind::Patrias => Some(&patrias::LOOK),
        Kind::Revolucion => Some(&revolucion::LOOK),
        _ => None,
    }
}

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
        'é' | 'É' => Some([0x02, 0x1f, 0x10, 0x1c, 0x1f]),
        'í' | 'Í' => Some([0x02, 0x0e, 0x04, 0x04, 0x0e]),
        'ñ' | 'Ñ' => Some([0x0a, 0x19, 0x15, 0x13, 0x11]),
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

    /// A logo from the pixel art of a look at one size: its frame number
    /// `frame`, or its reaction to a corner, with as much of the trim as the
    /// holiday has brought by `level`.
    fn pixels(look: &Look, size: &Size, frame: usize, reacting: bool, level: f32) -> Sprite {
        let art = size.frames[frame];
        let trim = size.trim;
        let w = art.iter().map(|l| l.len()).max().unwrap_or(0);
        let open = art.len().saturating_sub(size.react.len());
        let pieces = match trim {
            None => 0,
            Some(t) if level < 0.4 => t.pieces.len().min(1),
            Some(t) if level < 0.75 => t.pieces.len().min(3),
            Some(t) => t.pieces.len(),
        };
        // Pixel rows above the picture for the trim.
        let top = match trim {
            Some(t) if pieces > 0 => t.above.max(0) as usize,
            _ => 0,
        };
        let ph = art.len() + top;
        let mut px = vec![b'.'; w * ph];
        for (y, &l) in art.iter().enumerate() {
            let l = if reacting && y >= open {
                size.react[y - open]
            } else {
                l
            };
            // Holes inside a solid logo (`k`) hide what passes behind it.
            let first = l.bytes().position(|b| b != b'.').unwrap_or(0);
            let last = l.bytes().rposition(|b| b != b'.').unwrap_or(0);
            for (x, b) in l.bytes().enumerate() {
                let inside = look.solid && b == b'.' && (first..=last).contains(&x);
                px[(y + top) * w + x] = if inside { b'k' } else { b };
            }
        }
        let mut put = |x: i32, y: i32, b: u8| {
            if (0..w as i32).contains(&x) && (0..ph as i32).contains(&y) {
                px[y as usize * w + x as usize] = b;
            }
        };
        if let Some(t) = trim {
            if pieces == t.pieces.len() {
                for &(x, y) in t.leaves {
                    put(x, y, t.leaf);
                }
            }
            let pw = t.piece.first().map_or(0, |l| l.len()) as i32;
            for &(cx, y0, b) in &t.pieces[..pieces] {
                for (dy, l) in t.piece.iter().enumerate() {
                    for (dx, ch) in l.bytes().enumerate() {
                        if ch != b'.' {
                            let ch = if ch == b'x' { b } else { ch };
                            put(cx - pw / 2 + dx as i32, y0 + dy as i32, ch);
                        }
                    }
                }
            }
        }
        // Two pixels per cell; where two colors share one, the base gives
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
                    if t == look.back {
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

/// A bit of the holiday falling: a marigold petal, a piece of candy,
/// confetti.
#[derive(Clone, Copy)]
struct Bit {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    ttl: f32,
    /// Where it is in its sway and tumble.
    phase: f32,
    /// Which of `Fall::kinds` it is.
    kind: u8,
}

/// A bit lying on the bottom row, part of the path.
#[derive(Clone, Copy, Default)]
struct Fallen {
    /// Seconds left; 0 when nothing lies here.
    life: f32,
    kind: u8,
}

struct Logo {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: usize,
    since_bounce: f32,
    since_corner: f32,
    /// It is on its way to miss a corner by a cell, for a joke.
    jest: bool,
    since_jest: f32,
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
/// Seconds after a corner that a logo that bursts open does, three hits of
/// the stick in, and how long what is left of it hangs there.
const POP: f32 = 1.1;
const BROKEN: f32 = 0.8;
/// Seconds between a logo's jokes.
const JEST_EVERY: f32 = 12.0;

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
    /// The rest of the frames of a logo that has several (`sprite` is the
    /// first).
    frames: Vec<Sprite>,
    /// The logo reacting to a corner (empty when it has no reaction).
    laugh: Sprite,
    banner: Sprite,
    /// The banner of a near miss (empty when there are none).
    joke: Sprite,
    logos: Vec<Logo>,
    sparks: Vec<Spark>,
    spark_cap: usize,
    corners: u32,
    /// Seconds since the last corner hit, and since the last near miss.
    party_t: f32,
    joke_t: f32,
    clock: f32,

    palette: Vec<Color>,
    counter_dim: Color,

    /// The holiday the logo is dressed for, and what it dresses it as.
    fiesta: Fiesta,
    look: Option<&'static Look>,
    /// The holiday's own randomness, so ordinary days draw the same numbers.
    frng: fastrand::Rng,
    bits: Vec<Bit>,
    bit_cap: usize,
    /// Bits owed to the trail, carried over between steps.
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
            frames: Vec::new(),
            laugh: Sprite::default(),
            banner: Sprite::default(),
            joke: Sprite::default(),
            logos: Vec::new(),
            sparks: Vec::new(),
            spark_cap: 0,
            corners: 0,
            party_t: PARTY,
            joke_t: PARTY,
            clock: 0.0,
            palette: CLASSIC.to_vec(),
            counter_dim: Color::Reset,
            fiesta: Fiesta::NONE,
            look: None,
            frng: fastrand::Rng::with_seed(ctx.seed ^ 0x6361_6c61_7665_7261),
            bits: Vec::new(),
            bit_cap: 0,
            shed: 0.0,
            floor: Vec::new(),
        }
    }

    /// Picks the logo and the banner for the screen, the text and the holiday.
    fn fit(&mut self) {
        let (lw, lh) = (LOGO[4].chars().count() as i32, LOGO.len() as i32);
        self.look = look_for(self.fiesta);
        self.frames.clear();
        self.laugh = Sprite::default();
        self.joke = Sprite::default();
        self.sprite = if !self.text.is_empty() {
            Sprite::fit_text(&self.text, self.w, self.h, 40)
        } else if let Some(look) = self.look {
            let (s, frames, laugh) = self.costume(look);
            self.frames = frames;
            self.laugh = laugh;
            s
        } else if lw + 6 <= self.w && lh + 3 <= self.h {
            Sprite::from_lines(&LOGO)
        } else {
            Sprite::fit_text("DVD", self.w, self.h, 40)
        };
        self.banner = match self.look {
            Some(look) => Sprite::fit_lines(look.banner, self.w, self.h),
            None => Sprite::fit_text("CORNER!", self.w, self.h, 50),
        };
        if let Some(lines) = self.look.and_then(|look| look.joke) {
            self.joke = Sprite::fit_lines(lines, self.w, self.h);
        }
        for l in self.logos.iter_mut() {
            l.jest = false;
        }
        self.bits.clear();
        self.floor.clear();
        if let Some(look) = self.look {
            self.bit_cap = look.fall.cap * self.count.min(3);
            self.bits.reserve(self.bit_cap);
            self.floor.resize(self.w as usize, Fallen::default());
        }
    }

    /// The biggest picture of the look that leaves room to bounce, its other
    /// frames, and the picture reacting to a corner (none when it does not).
    fn costume(&self, look: &Look) -> (Sprite, Vec<Sprite>, Sprite) {
        let level = self.fiesta.level;
        for (i, size) in look.sizes.iter().enumerate() {
            let s = Sprite::pixels(look, size, 0, false, level);
            let room = if i + 1 == look.sizes.len() {
                s.w + 2 <= self.w && s.h + 2 <= self.h
            } else if i == 0 {
                s.w + 6 <= self.w && s.h * 2 <= self.h
            } else {
                s.w + 4 <= self.w && s.h + 4 <= self.h
            };
            if room {
                let frames = (1..size.frames.len())
                    .map(|k| Sprite::pixels(look, size, k, false, level))
                    .collect();
                let laugh = if size.react.is_empty() {
                    Sprite::default()
                } else {
                    Sprite::pixels(look, size, 0, true, level)
                };
                return (s, frames, laugh);
            }
        }
        let dvd = Sprite::fit_text("DVD", self.w, self.h, 40);
        (dvd, Vec::new(), Sprite::default())
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
        let joker = self.look.is_some_and(|look| look.joke.is_some()) && !self.assist;
        let l = &mut self.logos[i];
        let sy = if l.vy < 0.0 { -1.0 } else { 1.0 };
        l.vy = sy * base;
        if joker && l.since_jest > JEST_EVERY && mx > 0.0 && my >= 3.0 && base > 0.0 {
            // Aim at a corner, but stop a row short of it: the same bend as
            // the assist, for a joke. Not every leg has the room for it.
            let dist_x = if l.vx > 0.0 { mx - l.x } else { l.x };
            let travel = base * dist_x / l.vx.abs().max(1e-3);
            let d0 = if l.vy > 0.0 { my - l.y } else { l.y };
            let k = ((travel - d0 + 1.0) / my).round().max(0.0);
            let run = d0 + k * my - 1.0;
            if travel >= 1.0 && run > 1.0 && (0.86..=1.14).contains(&(run / travel)) {
                l.vy = sy * base * run / travel;
                l.jest = true;
            }
            return;
        }
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
        if let Some(look) = self.look {
            // The holiday's bits instead of sparks, and more of them as the
            // days go by; a logo that bursts open does it a moment later.
            if look.react == React::Pop {
                return;
            }
            let f = &look.fall;
            let burst = self.bit_cap as f32 * (f.burst.0 + f.burst.1 * self.fiesta.level);
            for _ in 0..burst as usize {
                if self.bits.len() >= self.bit_cap {
                    break;
                }
                let r = &mut self.frng;
                let (x, y) = if r.f32() * (sw + sh * 2.0) < sw {
                    (lx + r.f32() * sw, edge_y)
                } else {
                    (edge_x, ly + r.f32() * sh)
                };
                let a = r.f32() * std::f32::consts::FRAC_PI_2;
                let v = f.speed.0 + r.f32() * r.f32() * f.speed.1;
                self.bits.push(Bit {
                    x,
                    y,
                    vx: dx * a.cos() * v * 2.0,
                    vy: dy * a.sin() * v - 2.0,
                    age: 0.0,
                    ttl: f.ttl.0.0 + r.f32() * f.ttl.0.1,
                    phase: r.f32() * std::f32::consts::TAU,
                    kind: (f.pick)(r, true),
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

    /// A logo bursting open: its bits fly out of its middle in every
    /// direction, `share` of what a corner throws.
    fn burst(&mut self, i: usize, share: f32) {
        let Some(look) = self.look else {
            return;
        };
        let f = &look.fall;
        let l = &self.logos[i];
        let (sw, sh) = (self.sprite.w as f32, self.sprite.h as f32);
        let (cx, cy) = (l.x + sw * 0.5, l.y + sh * 0.5);
        let n = self.bit_cap as f32 * share * (f.burst.0 + f.burst.1 * self.fiesta.level);
        for _ in 0..n as usize {
            if self.bits.len() >= self.bit_cap {
                break;
            }
            let r = &mut self.frng;
            let a = r.f32() * std::f32::consts::TAU;
            let v = f.speed.0 + r.f32() * r.f32() * f.speed.1;
            self.bits.push(Bit {
                x: cx + (r.f32() - 0.5) * sw * 0.4,
                y: cy + (r.f32() - 0.5) * sh * 0.4,
                vx: a.cos() * v * 2.0,
                vy: a.sin() * v - 5.0,
                age: 0.0,
                ttl: f.ttl.0.0 + r.f32() * f.ttl.0.1,
                phase: r.f32() * std::f32::consts::TAU,
                kind: (f.pick)(r, true),
            });
        }
    }

    fn step(&mut self, dt: f32) {
        let (mx, my) = (self.max_x(), self.max_y());
        let pops = self.look.is_some_and(|look| look.react == React::Pop);
        for i in 0..self.logos.len() {
            let (mut bx, mut by) = (false, false);
            let bursts;
            {
                let l = &mut self.logos[i];
                l.since_bounce += dt;
                let before = l.since_corner;
                l.since_corner += dt;
                l.since_jest = (l.since_jest + dt).min(1e6);
                bursts = pops && before < POP && l.since_corner >= POP;
                // Nearing the corner it is about to miss, a logo slows down.
                let mv = if l.jest && mx > 0.0 {
                    let dist_x = if l.vx > 0.0 { mx - l.x } else { l.x };
                    let s = (dist_x / l.vx.abs().max(1e-3) / 1.6).clamp(0.0, 1.0);
                    dt * (0.25 + 0.75 * s * s * (3.0 - 2.0 * s))
                } else {
                    dt
                };
                if mx > 0.0 {
                    l.x += l.vx * mv;
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
                    l.y += l.vy * mv;
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
            if bursts {
                self.burst(i, 1.0);
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
            if bx && self.logos[i].jest {
                // The corner it missed by a row: "¡Inocente palomita!".
                self.logos[i].jest = false;
                self.logos[i].since_jest = 0.0;
                if !corner {
                    self.joke_t = 0.0;
                    self.burst(i, 0.3);
                }
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
        if let Some(look) = self.look {
            self.drift(&look.fall, dt);
        }
        self.party_t = (self.party_t + dt).min(PARTY);
        self.joke_t = (self.joke_t + dt).min(PARTY);
        self.clock = (self.clock + dt) % 3600.0;
    }

    /// The logos shed bits from their trailing side, and the bits drift down
    /// (or fall, if they are heavy) to lie on the bottom row.
    fn drift(&mut self, f: &Fall, dt: f32) {
        let level = self.fiesta.level;
        let (sw, sh) = (self.sprite.w as f32, self.sprite.h as f32);
        self.shed += dt * (f.shed.0 + f.shed.1 * level) * self.logos.len().min(3) as f32;
        while self.shed >= 1.0 {
            self.shed -= 1.0;
            if self.bits.len() >= self.bit_cap || self.logos.is_empty() {
                continue;
            }
            let r = &mut self.frng;
            let l = &self.logos[r.usize(..self.logos.len())];
            let back = if l.vx > 0.0 { l.x } else { l.x + sw - 1.0 };
            self.bits.push(Bit {
                x: back + (r.f32() - 0.5) * 2.0,
                y: l.y + r.f32() * sh,
                vx: -l.vx.signum() * (0.5 + r.f32() * 2.0),
                vy: r.f32() * 1.5,
                age: 0.0,
                ttl: f.ttl.1.0 + r.f32() * f.ttl.1.1,
                phase: r.f32() * std::f32::consts::TAU,
                kind: (f.pick)(r, false),
            });
        }
        let (w, floor_y) = (self.w as f32, (self.h - 1) as f32);
        let lie = f.lie.0 + (f.lie.1 - f.lie.0) * level;
        let floor = &mut self.floor;
        let drag = (1.0 - f.air.drag * dt).max(0.0);
        self.bits.retain_mut(|p| {
            p.age += dt;
            // The burst spends itself, then the bits fall the way their kind
            // does: petals slowly and swaying, candy with some weight.
            p.vx *= drag;
            let fall = f.kind(p.kind).fall;
            p.vy += (fall - p.vy) * (f.air.settle * dt).min(1.0) + f.air.gravity * dt;
            let sway = (p.age * 1.9 + p.phase).sin() * f.air.sway;
            p.x += (p.vx + sway) * dt;
            p.y += p.vy * dt;
            if p.y >= floor_y {
                let x = p.x.round();
                if (0.0..w).contains(&x) {
                    floor[x as usize] = Fallen {
                        life: lie * (0.7 + 0.3 * (p.phase / std::f32::consts::TAU)),
                        kind: p.kind,
                    };
                }
                return false;
            }
            p.age < p.ttl && p.x > -2.0 && p.x < w + 2.0
        });
    }

    /// The color of a tint for a logo whose bounce color is `base`.
    fn tint(&self, ctx: &Ctx, tint: Tint, base: usize) -> Color {
        let n = self.palette.len().max(1);
        match tint {
            Tint::Own(c) => c,
            Tint::Ink(c) if ctx.theme.native => c,
            Tint::Ink(_) => ctx.theme.fg,
            Tint::Dim(c) if ctx.theme.native => c,
            Tint::Dim(_) => ctx.fade(ctx.theme.fg, 0.45),
            Tint::Accent(i, c) => ctx.pick(i, c),
            Tint::Cycle(k) => self.palette[(base + k as usize) % n],
            Tint::Across => self.palette[(base + n / 2) % n],
        }
    }

    /// The bits in the air and the path they make on the bottom row.
    fn draw_bits(&self, ctx: &Ctx, c: &mut Canvas, f: &Fall) {
        for (x, fl) in self.floor.iter().enumerate() {
            if fl.life <= 0.0 {
                continue;
            }
            // Lying bits face up or down by where they fell.
            let side = (x * 7 + fl.kind as usize % 3).is_multiple_of(3) as usize;
            let kind = f.kind(fl.kind);
            let col = self.tint(ctx, kind.tints[side], 0);
            let col = ctx.fade(col, (1.0 - fl.life / 4.0).clamp(0.0, 1.0));
            c.set(x as i32, self.h - 1, kind.glyphs[side], col);
        }
        for p in &self.bits {
            // A bit tumbles: its two faces take turns.
            let face = ((p.age * 2.6 + p.phase) as i32 & 1) as usize;
            let kind = f.kind(p.kind);
            let col = self.tint(ctx, kind.tints[face], 0);
            let k = ((p.age / p.ttl - 0.75) * 4.0).clamp(0.0, 1.0);
            c.set(
                p.x.round() as i32,
                p.y.round() as i32,
                kind.glyphs[face],
                ctx.fade(col, k),
            );
        }
    }

    /// The picture logo `i` shows now, and how many columns it is shaken
    /// aside.
    fn pose(&self, i: usize, l: &Logo) -> (&Sprite, i32) {
        let Some(look) = self.look else {
            return (&self.sprite, 0);
        };
        let t = self.party_t;
        let reacts = !self.laugh.parts.is_empty();
        match look.react {
            // At a corner the skulls laugh, their jaws chattering.
            React::Chatter if reacts && t < 1.8 && (t * 9.0) as i32 % 2 == 1 => {
                return (&self.laugh, 0);
            }
            React::Hold if reacts && t < 1.8 => return (&self.laugh, 0),
            React::Pop if reacts => {
                let s = l.since_corner;
                if (POP..POP + BROKEN).contains(&s) {
                    return (&self.laugh, 0);
                }
                // Three hits of the stick, each one a lurch to the side.
                let hit = s * 3.0 / POP;
                if s < POP && hit.fract() < 0.5 {
                    return (&self.sprite, if hit as i32 % 2 == 0 { 1 } else { -1 });
                }
            }
            _ => {}
        }
        if !self.frames.is_empty() && look.beat > 0.0 {
            let spin = if look.react == React::Spin && t < 1.8 {
                3.0
            } else {
                1.0
            };
            let k = ((self.clock * look.beat * spin) as usize + i) % (self.frames.len() + 1);
            if k > 0 {
                return (&self.frames[k - 1], 0);
            }
        }
        (&self.sprite, 0)
    }

    fn draw_counter(&self, c: &mut Canvas) {
        let mut buf = [0u8; 24];
        let prefix: &[u8] = if self.look.is_some() {
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
                jest: false,
                since_jest: 100.0,
            };
            self.logos.push(l);
        }
        self.spark_cap = 180 * self.count.min(3);
        self.sparks = Vec::with_capacity(self.spark_cap);
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.palette.clear();
        if ctx.theme.native {
            let classic = look_for(ctx.fiesta).map_or(&CLASSIC[..], |look| look.classic);
            self.palette.extend_from_slice(classic);
            self.counter_dim = ctx.fade(rgb(0xb0b0b0), 0.45);
        } else {
            let n = ctx.theme.colors.len().max(1);
            self.palette
                .extend((0..n).map(|i| ctx.pick(i, Color::Reset)));
            self.counter_dim = ctx.fade(ctx.theme.fg, 0.45);
        }
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
        // The banner of a corner, or of a near miss.
        let (t, b) = if self.party_t < PARTY {
            (self.party_t, &self.banner)
        } else {
            (self.joke_t, &self.joke)
        };
        if t < PARTY {
            // Blink fast at first, then hold and fade out.
            let on = t > 1.2 || (t * 7.0) as i32 % 2 == 0;
            if on {
                let col = self.palette[(t * 9.0) as usize % n];
                let col = if t > PARTY - 0.6 {
                    ctx.fade(col, (t - (PARTY - 0.6)) / 0.6)
                } else {
                    col
                };
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
        if let Some(look) = self.look {
            self.draw_bits(ctx, c, &look.fall);
        }
        for (i, l) in self.logos.iter().enumerate() {
            let (pose, shake) = self.pose(i, l);
            let (x, y) = (l.x.round() as i32 + shake, l.y.round() as i32);
            let base = l.color % n;
            let Some(look) = self.look.filter(|_| !pose.parts.is_empty()) else {
                self.sprite.draw(c, x, y, self.palette[base], 0);
                continue;
            };
            pose.draw_parts(c, x, y, |part| {
                let tint = look.parts.iter().find(|(p, _)| *p == part);
                self.tint(ctx, tint.map_or(Tint::Ink(Color::Reset), |t| t.1), base)
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
    use super::muertos::HUESO;
    use super::*;

    /// A day of every look the logo has, as the holidays are dressed so far.
    fn days() -> Vec<(&'static str, Fiesta)> {
        vec![
            ("muertos", Fiesta::of_day(11, 2)),
            ("muertos, early", Fiesta::of_day(10, 28)),
            ("navidad", Fiesta::height(Kind::Navidad)),
            ("posadas", Fiesta::of_day(12, 16)),
            ("inocentes", Fiesta::of_day(12, 28)),
            ("año nuevo", Fiesta::of_day(12, 31)),
            ("reyes", Fiesta::of_day(1, 3)),
            ("rosca", Fiesta::of_day(1, 6)),
            ("mes patrio", Fiesta::of_day(9, 2)),
            ("grito", Fiesta::of_day(9, 15)),
            ("candelaria", Fiesta::of_day(2, 2)),
            ("niño", Fiesta::of_day(4, 30)),
            ("revolución", Fiesta::of_day(11, 20)),
        ]
    }

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
        let mut every = vec![Fiesta::NONE];
        every.extend(days().iter().map(|d| d.1));
        for fiesta in every {
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
    fn banners_are_all_in_the_font() {
        for (_, f) in days() {
            let look = look_for(f).unwrap();
            for line in look.banner.iter().chain(look.joke.into_iter().flatten()) {
                for ch in line.chars().filter(|c| *c != ' ') {
                    assert!(glyph_es(ch).is_some(), "{ch} in {line}");
                }
            }
        }
    }

    #[test]
    fn pictures_line_up() {
        for (day, f) in days() {
            let look = look_for(f).unwrap();
            assert!(!look.classic.is_empty() && !look.fall.kinds.is_empty());
            assert!(
                look.parts.iter().all(|p| p.0 != b'k' && p.0 != b'.'),
                "{day}"
            );
            for size in look.sizes {
                let w = size.frames[0][0].len();
                for l in size.frames.iter().flat_map(|f| f.iter()).chain(size.react) {
                    assert_eq!(l.len(), w, "{day}: {l}");
                }
                for f in size.frames {
                    assert_eq!(f.len(), size.frames[0].len(), "{day}");
                }
                assert!(size.react.len() <= size.frames[0].len(), "{day}");
                for level in [0.25, 0.6, 1.0] {
                    let s = Sprite::pixels(look, size, 0, false, level);
                    assert_eq!(s.cells.len(), s.parts.len());
                    if !size.react.is_empty() {
                        let laugh = Sprite::pixels(look, size, 0, true, level);
                        assert_eq!((s.w, s.h), (laugh.w, laugh.h), "{day}");
                        assert_ne!(s.cells, laugh.cells, "{day}");
                    }
                    // Every letter of the art is a part with a color.
                    for &p in s.parts.iter().filter(|p| **p != 0 && **p != b'k') {
                        assert!(look.parts.iter().any(|q| q.0 == p), "{day}: {}", p as char);
                    }
                }
            }
            // The sizes get smaller, and each fits a screen the one before
            // does not.
            let w: Vec<i32> = look
                .sizes
                .iter()
                .map(|s| Sprite::pixels(look, s, 0, false, 1.0).w)
                .collect();
            assert!(w.windows(2).all(|p| p[0] > p[1]), "{day}: {w:?}");
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

    /// A scene dressed for `f`, on a `w`×`h` screen.
    fn dressed(ctx: &Ctx, w: u16, h: u16) -> Dvd {
        let mut d = Dvd::new(ctx);
        d.resize(ctx, w, h);
        d.retheme(ctx);
        d
    }

    fn bones(c: &Canvas, bone: Color) -> usize {
        c.cells().iter().filter(|cell| cell.fg == bone).count()
    }

    fn flowers(d: &Dvd) -> usize {
        let crown = *b"oyr";
        d.sprite.parts.iter().filter(|p| crown.contains(p)).count()
    }

    /// How many pixels of a logo are of these parts.
    fn parts(d: &Dvd, of: &[u8]) -> usize {
        d.sprite.parts.iter().filter(|p| of.contains(p)).count()
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
                let mut d = dressed(&ctx, 120, 36);
                assert!(!d.sprite.parts.is_empty(), "a sugar skull");
                crowns.push(flowers(&d));
                let c = play(&mut d, &ctx, 20.0);
                assert!(bones(&c, bone) > 100, "{theme}");
                assert!(!d.bits.is_empty(), "petals fall from it");
                assert!(d.floor.iter().any(|f| f.life > 0.0), "and lie on the floor");
                // A corner: a burst of petals, the skull laughs, the banner.
                let before = d.bits.len();
                d.celebrate(0);
                assert!(d.bits.len() > before + 20);
                let banner = Sprite::fit_lines(muertos::LOOK.banner, 120, 36);
                assert_eq!((d.banner.h, &d.banner.cells), (7, &banner.cells));
                let c = play(&mut d, &ctx, 0.5);
                assert!(c.to_text().contains("esquinas: "));
            }
            // The crown grows as the holiday builds up.
            assert!(crowns[0] > 0 && crowns[1] > crowns[0], "{crowns:?}");
        }
    }

    /// Dates that were ordinary before the other holidays came.
    const ORDINARY: [(u32, u32); 3] = [(6, 15), (8, 20), (3, 5)];

    #[test]
    fn ordinary_days_stay_plain() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let mut plain = vec![Fiesta::NONE];
        plain.extend(ORDINARY.iter().map(|&(m, d)| Fiesta::of_day(m, d)));
        // And a holiday that has no costume yet is as plain as any day.
        plain.extend([
            Fiesta::height(Kind::Equinoccio),
            Fiesta::height(Kind::SantaCruz),
        ]);
        for f in plain {
            let ctx = ctx(&th, &o, f);
            let mut d = dressed(&ctx, 120, 36);
            assert!(d.sprite.parts.is_empty() && d.laugh.cells.is_empty());
            assert_eq!(d.sprite.w, LOGO[4].chars().count() as i32);
            assert_eq!(d.palette, CLASSIC.to_vec());
            d.celebrate(0);
            let c = play(&mut d, &ctx, 1.0);
            assert_eq!(bones(&c, HUESO), 0);
            assert!(d.bits.is_empty() && d.floor.is_empty() && !d.sparks.is_empty());
            assert!(c.to_text().contains("corners: 1"));
        }
    }

    #[test]
    fn the_holiday_comes_and_goes_through_retheme() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let plain = ctx(&th, &o, Fiesta::NONE);
        for (day, f) in days() {
            let holiday = ctx(&th, &o, f);
            let look = look_for(f).unwrap();
            let mut d = dressed(&plain, 100, 30);
            play(&mut d, &plain, 2.0);
            d.retheme(&holiday);
            assert!(!d.sprite.parts.is_empty(), "{day}");
            assert_eq!(d.palette, look.classic.to_vec(), "{day}");
            let c = play(&mut d, &holiday, 3.0);
            assert!(c.cells().iter().any(|cell| cell.ch != ' '), "{day}");
            for l in &d.logos {
                assert!(l.x <= d.max_x() && l.y <= d.max_y());
            }
            d.retheme(&plain);
            assert!(d.sprite.parts.is_empty() && d.bits.is_empty() && d.floor.is_empty());
            assert_eq!(d.palette, CLASSIC.to_vec(), "{day}");
            let c = play(&mut d, &plain, 1.0);
            assert_eq!(bones(&c, HUESO), 0);
        }
    }

    /// The days the holiday moves from one look to another: the logo is
    /// dressed again when the date changes.
    #[test]
    fn the_navidad_changes_its_look_by_the_day() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let eve = ctx(&th, &o, Fiesta::of_day(12, 24));
        let reyes = ctx(&th, &o, Fiesta::of_day(1, 3));
        let mut d = dressed(&eve, 120, 36);
        let pinata = d.sprite.cells.clone();
        d.retheme(&reyes);
        assert_ne!(d.sprite.cells, pinata, "a rosca now");
        assert_eq!(d.palette, navidad::look(reyes.fiesta).classic.to_vec());
        d.retheme(&eve);
        assert_eq!(d.sprite.cells, pinata, "a piñata again");
    }

    #[test]
    fn navidad_dresses_the_logo() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = ctx(&th, &o, Fiesta::height(Kind::Navidad));
            let mut d = dressed(&ctx, 120, 40);
            // Seven picos, a ball and a rope.
            for p in b"abcdefghr" {
                assert!(parts(&d, &[*p]) > 0, "{}", *p as char);
            }
            let c = play(&mut d, &ctx, 20.0);
            let col = |cell: &crate::render::Cell| cell.fg != Color::Reset && cell.ch != ' ';
            assert!(c.cells().iter().filter(|cell| col(cell)).count() > 100);
            assert!(!d.bits.is_empty() || d.floor.iter().any(|f| f.life > 0.0));
            // Under classic, the picos wear papel de china.
            if th.native {
                assert!(c.cells().iter().any(|cell| cell.fg == rgb(0xe4007c)));
            }
            let banner = Sprite::fit_lines(&["¡Dale, dale,", "dale!"], 120, 40);
            assert_eq!(d.banner.cells, banner.cells);
            assert_eq!(d.banner.h, 11);
            assert!(play(&mut d, &ctx, 0.5).to_text().contains("esquinas: "));
        }
    }

    #[test]
    fn the_pinata_bursts_at_the_third_dale() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let ctx = ctx(&th, &o, Fiesta::height(Kind::Navidad));
        let mut d = dressed(&ctx, 120, 40);
        d.bits.clear();
        d.shed = 0.0;
        d.celebrate(0);
        d.logos[0].since_corner = 0.0;
        assert!(d.bits.is_empty(), "nothing flies out yet");
        let whole = d.sprite.cells.clone();
        // The stick lands three times, each a lurch to the side...
        let mut lurches = 0;
        let mut was = false;
        for _ in 0..30 {
            d.step(1.0 / 30.0);
            let (pose, shake) = d.pose(0, &d.logos[0]);
            assert_eq!(pose.cells, whole, "still whole");
            lurches += (shake != 0 && !was) as i32;
            was = shake != 0;
        }
        assert_eq!(lurches, 3);
        assert!(d.bits.len() < 10, "{}", d.bits.len());
        // ...and then it bursts into candy and fruit, and what is left of
        // it hangs there for a moment before a new one comes.
        for _ in 0..8 {
            d.step(1.0 / 30.0);
        }
        assert!(d.bits.len() > 30, "{}", d.bits.len());
        assert_eq!(d.pose(0, &d.logos[0]).0.cells, d.laugh.cells);
        assert_ne!(d.laugh.cells, whole);
        for _ in 0..40 {
            d.step(1.0 / 30.0);
        }
        assert_eq!(d.pose(0, &d.logos[0]).0.cells, whole, "a fresh piñata");
    }

    #[test]
    fn candy_falls_and_lies_on_the_floor() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let ctx = ctx(&th, &o, Fiesta::height(Kind::Navidad));
        let mut d = dressed(&ctx, 100, 30);
        d.burst(0, 1.0);
        let thrown = d.bits.len();
        assert!(thrown > 20);
        let kinds: std::collections::HashSet<u8> = d.bits.iter().map(|b| b.kind).collect();
        assert!(kinds.len() >= 4, "{kinds:?}: candy, fruit and more");
        for _ in 0..(6.0 * 30.0) as usize {
            d.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert!(d.floor.iter().filter(|f| f.life > 0.0).count() > 10);
        // The candy lying there goes after a while.
        for _ in 0..(40.0 * 30.0) as usize {
            d.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
            d.bits.clear();
        }
        assert!(d.floor.iter().all(|f| f.life <= 0.0));
    }

    #[test]
    fn the_posadas_bring_more_candy() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let mut seen = Vec::new();
        for day in [16, 24] {
            let ctx = ctx(&th, &o, Fiesta::of_day(12, day));
            let mut d = dressed(&ctx, 100, 30);
            let mut shed = 0;
            for _ in 0..(60.0 * 30.0) as usize {
                d.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                shed += d.bits.len() + d.floor.iter().filter(|f| f.life > 0.0).count();
            }
            seen.push(shed);
        }
        assert!(seen[1] > seen[0] * 3 / 2, "{seen:?}");
    }

    #[test]
    fn reyes_brings_the_rosca() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let mut jewels = Vec::new();
        for day in [2, 6] {
            let ctx = ctx(&th, &o, Fiesta::of_day(1, day));
            let mut d = dressed(&ctx, 120, 36);
            assert!(
                parts(&d, b"b") > 100 && parts(&d, b"s") > 20,
                "bread and sugar"
            );
            jewels.push(parts(&d, b"jlm"));
            let banner = Sprite::fit_lines(&["¡Te salió", "el muñeco!"], 120, 36);
            assert_eq!(d.banner.cells, banner.cells);
            // At a corner the muñeco shows in the hole.
            assert_eq!(parts(&d, b"p"), 0);
            d.celebrate(0);
            let mut c = Canvas::new(120, 36, Color::Reset);
            d.draw(&ctx, &mut c);
            assert!(!d.laugh.parts.is_empty());
            assert!(d.laugh.parts.contains(&b'p') && d.laugh.parts.contains(&b'q'));
            assert!(c.cells().iter().any(|cell| cell.fg == rgb(0xf2c4a0)));
        }
        // More of the candied fruit as the days go by.
        assert!(jewels[0] > 0 && jewels[1] > jewels[0], "{jewels:?}");
    }

    #[test]
    fn ano_nuevo_says_so() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        for (m, day) in [(12, 31), (1, 1)] {
            let ctx = ctx(&th, &o, Fiesta::of_day(m, day));
            let mut d = dressed(&ctx, 120, 36);
            let banner = Sprite::fit_lines(&["¡Feliz Año", "Nuevo!"], 120, 36);
            assert_eq!(d.banner.cells, banner.cells);
            // Grapes, confetti and sparks of gold, not only candy.
            d.burst(0, 1.0);
            assert!(d.bits.iter().any(|b| b.kind == 0 || b.kind == 2));
            assert_eq!(d.look.map(|l| l.fall.kinds.len()), Some(4));
        }
    }

    #[test]
    fn on_the_santos_inocentes_it_misses_a_corner_by_a_cell() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let ctx = ctx(&th, &o, Fiesta::of_day(12, 28));
        let mut d = dressed(&ctx, 100, 30);
        assert!(!d.joke.cells.is_empty());
        let joke = Sprite::fit_lines(&["¡Inocente", "palomita!"], 100, 30);
        assert_eq!(d.joke.cells, joke.cells);
        let (mx, my) = (d.max_x(), d.max_y());
        let (mut jokes, mut slowed) = (0, false);
        let mut was = d.joke_t;
        for _ in 0..(300.0 * 30.0) as usize {
            let before = d.logos[0].x;
            d.step(1.0 / 30.0);
            if d.logos[0].jest {
                let l = &d.logos[0];
                // Near the wall it crawls.
                let to_wall = if l.vx > 0.0 { mx - l.x } else { l.x };
                slowed |= to_wall < 3.0 && (l.x - before).abs() < 0.5 * l.vx.abs() / 30.0;
            }
            if d.joke_t < was {
                // The wall it just bounced off, and a row from a corner.
                let l = &d.logos[0];
                assert!(l.x < 1.0 || l.x > mx - 1.0, "{}", l.x);
                let gap = l.y.min(my - l.y);
                assert!((0.7..1.4).contains(&gap), "a cell from the corner: {gap}");
                jokes += 1;
            }
            was = d.joke_t;
        }
        assert!(jokes >= 5, "{jokes} jokes");
        assert!(slowed, "it slows down as it nears the corner");
        assert_eq!(d.corners, 0, "and never gets there");
        // The banner shows for a moment.
        d.joke_t = 0.1;
        d.party_t = PARTY;
        let mut c = Canvas::new(100, 30, Color::Reset);
        d.draw(&ctx, &mut c);
        assert!(c.cells().iter().filter(|cell| cell.ch == '█').count() > 100);
    }

    #[test]
    fn other_days_never_play_the_joke() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        for f in [
            Fiesta::of_day(12, 27),
            Fiesta::of_day(12, 24),
            Fiesta::of_day(1, 3),
            Fiesta::NONE,
        ] {
            let ctx = ctx(&th, &o, f);
            let mut d = dressed(&ctx, 100, 30);
            assert!(d.joke.cells.is_empty());
            for _ in 0..(120.0 * 30.0) as usize {
                d.step(1.0 / 30.0);
                assert!(d.joke_t >= PARTY && !d.logos[0].jest);
            }
        }
    }

    #[test]
    fn patrias_dresses_the_logo() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let flag = [rgb(0x006847), rgb(0xf4f1ea), rgb(0xce1126)];
        for theme in ["classic", "gruvbox"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let mut banderas = Vec::new();
            for day in [2, 15] {
                let ctx = ctx(&th, &o, Fiesta::of_day(9, day));
                let mut d = dressed(&ctx, 120, 40);
                // A sombrero: felt, gold and the three stripes of the band.
                for p in b"fdgvwr" {
                    assert!(parts(&d, &[*p]) > 0, "{}", *p as char);
                }
                banderas.push(parts(&d, b"vwr"));
                let c = play(&mut d, &ctx, 20.0);
                if th.native {
                    // The band is the flag, whichever way it has turned.
                    for col in flag {
                        assert!(c.cells().iter().any(|cell| cell.fg == col), "{col:?}");
                    }
                    assert_eq!(d.palette, flag.to_vec());
                }
                assert!(!d.bits.is_empty() || d.floor.iter().any(|f| f.life > 0.0));
                let banner = Sprite::fit_lines(&["¡VIVA", "MÉXICO!"], 120, 40);
                assert_eq!(d.banner.cells, banner.cells);
                // At a corner the sombrero is tipped, its brim curling up.
                d.celebrate(0);
                let mut c = Canvas::new(120, 40, Color::Reset);
                d.draw(&ctx, &mut c);
                assert_ne!(d.laugh.cells, d.sprite.cells);
                assert!(c.to_text().contains("esquinas: 1"));
            }
            // More banderitas as the mes patrio goes on.
            assert!(banderas[1] > banderas[0], "{banderas:?}");
        }
    }

    #[test]
    fn the_band_turns_with_each_bounce() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let ctx = ctx(&th, &o, Fiesta::of_day(9, 15));
        let mut d = dressed(&ctx, 120, 40);
        let mut left = Vec::new();
        for color in 0..3 {
            d.logos[0].color = color;
            d.logos[0].x = 0.0;
            d.logos[0].y = 0.0;
            let mut c = Canvas::new(120, 40, Color::Reset);
            d.draw(&ctx, &mut c);
            // The stripe at the left end of the band.
            let x = d.sprite.parts.chunks(d.sprite.w as usize).find_map(|row| {
                let first = row.iter().position(|p| *p == b'v')?;
                Some(first)
            });
            let y = d
                .sprite
                .parts
                .chunks(d.sprite.w as usize)
                .position(|r| r.contains(&b'v'));
            left.push(c.get(x.unwrap() as i32, y.unwrap() as i32).unwrap().fg);
        }
        left.sort_by_key(|c| format!("{c:?}"));
        left.dedup();
        assert_eq!(left.len(), 3, "each stripe in turn");
    }

    #[test]
    fn candelaria_dresses_the_logo() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let flame = rgb(0xffb347);
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = ctx(&th, &o, Fiesta::of_day(2, 2));
            let mut d = dressed(&ctx, 120, 40);
            // A tamal: hoja de maíz, tied at both ends, and steam above it.
            for p in b"hHcs" {
                assert!(parts(&d, &[*p]) > 0, "{}", *p as char);
            }
            if th.native {
                assert_eq!(d.palette, vec![rgb(0x2a5caa), rgb(0xf2b632), rgb(0xf4f1ea)]);
            }
            // The steam curls: the frames take turns.
            assert!(!d.frames.is_empty());
            let mut now = Canvas::new(120, 40, Color::Reset);
            d.draw(&ctx, &mut now);
            d.clock += 0.4;
            let mut later = Canvas::new(120, 40, Color::Reset);
            d.draw(&ctx, &mut later);
            assert_ne!(now.to_text(), later.to_text(), "steam rises");
            let banner = Sprite::fit_lines(&["¡Te tocan", "los tamales!"], 120, 40);
            assert_eq!(d.banner.cells, banner.cells);
            // At a corner it opens: masa and salsa show.
            assert_eq!(parts(&d, b"mr"), 0);
            d.celebrate(0);
            assert!(d.laugh.parts.contains(&b'm') && d.laugh.parts.contains(&b'r'));
            let (pose, _) = d.pose(0, &d.logos[0]);
            assert_eq!(pose.cells, d.laugh.cells);
            // The spark of a vela keeps its flame under every theme.
            d.bits.push(Bit {
                x: 5.0,
                y: 5.0,
                vx: 0.0,
                vy: 0.0,
                age: 0.0,
                ttl: 100.0,
                phase: 0.0,
                kind: 3,
            });
            let mut c = Canvas::new(120, 40, Color::Reset);
            d.draw(&ctx, &mut c);
            assert!(c.cells().iter().any(|cell| cell.fg == flame), "{theme}");
        }
    }

    #[test]
    fn the_nino_spins_a_trompo() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "nord"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = ctx(&th, &o, Fiesta::of_day(4, 30));
            let mut d = dressed(&ctx, 120, 40);
            // Stripes, a belt, the cuerda and the púa.
            for p in b"abwzm" {
                assert!(parts(&d, &[*p]) > 0, "{}", *p as char);
            }
            if th.native {
                assert_eq!(d.palette.len(), 5);
            }
            // Four frames, the stripes in a different place in each.
            assert_eq!(d.frames.len(), 3);
            let mut all = vec![d.sprite.parts.clone()];
            all.extend(d.frames.iter().map(|f| f.parts.clone()));
            for (i, a) in all.iter().enumerate() {
                for b in &all[i + 1..] {
                    assert_ne!(a, b);
                }
            }
            let banner = Sprite::fit_lines(&["¡Feliz Día", "del Niño!"], 120, 40);
            assert_eq!(d.banner.cells, banner.cells);
            // It turns on its own, and three times as fast at a corner.
            d.party_t = PARTY;
            d.clock = 0.15;
            let calm = d.pose(0, &d.logos[0]).0.parts.clone();
            assert_eq!(calm, d.frames[0].parts, "frame 2 of 4 at 8 fps");
            d.celebrate(0);
            d.clock = 0.15;
            let fast = d.pose(0, &d.logos[0]).0.parts.clone();
            assert_eq!(fast, d.frames[2].parts, "frame 4 of 4");
            // And it throws canicas and stars.
            assert!(d.bits.len() > 20);
            d.clock = 0.0;
            assert!(play(&mut d, &ctx, 0.5).to_text().contains("esquinas: 1"));
        }
    }

    #[test]
    fn the_revolucion_raises_a_sombrero_over_cananas() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "nord"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = ctx(&th, &o, Fiesta::of_day(11, 20));
            let mut d = dressed(&ctx, 120, 44);
            // A hat with its band, and the straps with brass in them.
            for p in b"fdvsb" {
                assert!(parts(&d, &[*p]) > 0, "{}", *p as char);
            }
            assert_eq!(parts(&d, b"B"), 0);
            let banner = Sprite::fit_lines(&["¡Tierra y", "Libertad!"], 120, 44);
            assert_eq!(d.banner.cells, banner.cells);
            if th.native {
                // The band is rojo or verde, a bounce after a bounce, on the
                // manta of the hat.
                assert_eq!(d.palette, vec![rgb(0xce1126), rgb(0x006847)]);
                let mut seen = Vec::new();
                for color in 0..2 {
                    d.logos[0].color = color;
                    let mut c = Canvas::new(120, 44, Color::Reset);
                    d.draw(&ctx, &mut c);
                    assert!(c.cells().iter().any(|cell| cell.fg == rgb(0xefe6cf)));
                    assert!(c.cells().iter().any(|cell| cell.fg == rgb(0x8b7d5b)));
                    seen.push(c.cells().iter().any(|cell| cell.fg == d.palette[color]));
                }
                assert_eq!(seen, [true, true]);
            }
            // At a corner the sombrero is raised and the brass gleams.
            d.celebrate(0);
            assert!(d.laugh.parts.contains(&b'B'));
            assert_ne!(d.laugh.cells, d.sprite.cells);
            assert!(d.bits.len() > 20);
        }
    }
}
