//! bonsai: a bonsai tree grows branch by branch.
//!
//! A port of cbonsai by John Allbritten <https://gitlab.com/jallbrit/cbonsai>,
//! licensed GPL-3.0. The growth engine (`branch`, `setDeltas`, `chooseString`,
//! `chooseColor`), the pots, the message box and the live and infinite modes
//! follow cbonsai.c call for call. Even `rand()` is glibc's, so `seed = N`
//! grows exactly the tree `cbonsai -s N` grows in a terminal of the same size.
//!
//! cbonsai grows the whole tree recursively and sleeps after every step in
//! live mode. Here the recursion runs at once and records every step; live
//! mode then reveals the recorded steps at `step` seconds each.
//!
//! On Día de Muertos the same tree grows as a Metepec árbol de la vida: its
//! leaves open into clumps of cempasúchil, terciopelo and nube among the green
//! (more of them as the holiday builds up), it stands in a talavera pot with
//! marigold petals strewn on the soil, and petals let go of the blossoms,
//! tumble down and lie a while on the ground. At its foot, a calavera de
//! azúcar and veladoras flickering in colored glass; by November 2 tapers and
//! a pan de muerto have joined them. Leaves the user chose keep their glyph
//! and only take the colors.
//!
//! On Navidad the tree is trimmed the Mexican way: esferas hang among the
//! leaves, a series of foquitos runs along the branches blinking in chase and
//! alternating patterns (the scene reports `next_change` so the app still
//! sleeps between steps), heno drapes from the boughs, and once the tree is
//! grown a star shines on its crown. It stands in a foil-wrapped pot with
//! nochebuenas on the soil. At its foot lies a nacimiento on musgo: a pesebre
//! with the ox and the mule, María and José, and across the pot the pastores
//! and their sheep, more of them with every posada; the Niño Dios and an
//! angel arrive on Nochebuena. In January the Reyes Magos set out from the
//! edge of the screen and come closer each day, reaching the pesebre on the
//! 6th.
//!
//! On the Fiestas Patrias the leaves dress in the flag's colors: verde, white
//! blossoms around the middle of the canopy and red flowers and tunas toward
//! the right, more of them as the month climbs, and petals drift down from
//! them. The pot is banded verde, blanco and rojo with banderitas planted in
//! the soil (more by the Grito). At its foot grows the emblem: a nopal from a
//! rock in the lake, a pad more each week, an eagle perching on it from the
//! 11th and the serpent in its beak on the 15th and 16th.

use crate::color::{self, Color, rgb};
use crate::fiesta::{self, Fiesta};
use crate::render::{BOLD, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "bonsai",
    aliases: &["cbonsai"],
    about: "a bonsai tree grows branch by branch, after cbonsai",
    options: &[
        Opt {
            key: "life",
            default: Val::I(32),
            help: "Life; higher -> more growth, 0-200 (cbonsai -L)",
        },
        Opt {
            key: "multiplier",
            default: Val::I(5),
            help: "Branch multiplier; higher -> more branching, 1-20 (-M)",
        },
        Opt {
            key: "base",
            default: Val::I(1),
            help: "Pot to grow in: 1 or 2, 0 for none (-b)",
        },
        Opt {
            key: "leaves",
            default: Val::S("&"),
            help: "Comma-separated strings chosen at random for leaves (-c)",
        },
        Opt {
            key: "message",
            default: Val::S(""),
            help: "Message shown in a box next to the tree, empty for none (-m)",
        },
        Opt {
            key: "step",
            default: Val::F(0.03),
            help: "Seconds per growth step in live mode (-t)",
        },
        Opt {
            key: "wait",
            default: Val::F(4.0),
            help: "Seconds a finished tree stays before the next one grows (-w)",
        },
        Opt {
            key: "live",
            default: Val::B(true),
            help: "Show each step of growth; false shows the grown tree at once (-l)",
        },
        Opt {
            key: "infinite",
            default: Val::B(true),
            help: "Keep growing new trees (-i)",
        },
        Opt {
            key: "colors",
            default: Val::S(""),
            help: "Dark leaves, dark wood, light leaves, light wood, e.g. \"2,3,10,11\" (-k); empty = the theme's",
        },
        Opt {
            key: "geometry",
            default: Val::S(""),
            help: "Put the pot at \"ROW,COL\" (-g); empty = centered at the bottom",
        },
        Opt {
            key: "seed",
            default: Val::I(0),
            help: "Seed of the first tree, the same tree as cbonsai -s (0 = random)",
        },
    ],
    make: |ctx| Box::new(Bonsai::new(ctx)),
};

// cbonsai's color pairs (minus one): COLOR_LEAF_DARK, COLOR_WOOD_DARK,
// COLOR_LEAF_BRIGHT, COLOR_WOOD_BRIGHT and COLOR_TEXT.
const LEAF_DARK: u8 = 0;
const WOOD_DARK: u8 = 1;
const LEAF_BRIGHT: u8 = 2;
const WOOD_BRIGHT: u8 = 3;
const TEXT: u8 = 4;
/// Style bits on top of the pair: A_BOLD, and the halves of a double-width glyph.
const BOLD_BIT: u8 = 8;
const WIDE_HEAD: u8 = 16;
const WIDE_TAIL: u8 = 32;

/// The colors released cbonsai (1.3.x) shows: its source says `-k 2,3,10,11`,
/// but the shipped binary draws the leaf shades the other way round, and that
/// is what people know. Plus its text pair (8 on 256-color terminals).
const CLASSIC: [Color; 5] = [
    Color::Ansi(10),
    Color::Ansi(3),
    Color::Ansi(2),
    Color::Ansi(11),
    Color::Ansi(8),
];

/// A safety net for absurd settings: a tree never records more steps than this.
const MAX_STEPS: usize = 100_000;

// ---------------------------------------------------------------------------
// glibc rand()

/// glibc's `srand`/`rand` (the default TYPE_3 additive feedback generator),
/// so trees match cbonsai's seed for seed.
struct CRand {
    s: [i32; 31],
    f: usize,
    r: usize,
}

impl CRand {
    fn new(seed: u32) -> CRand {
        let seed = seed.max(1);
        let mut s = [0i32; 31];
        let mut word = seed as i32;
        s[0] = word;
        for slot in s.iter_mut().skip(1) {
            // state[i] = (16807 * state[i - 1]) % 2147483647, without overflow.
            let (hi, lo) = (word as i64 / 127_773, word as i64 % 127_773);
            word = (16_807 * lo - 2_836 * hi) as i32;
            if word < 0 {
                word = word.wrapping_add(2_147_483_647);
            }
            *slot = word;
        }
        let mut g = CRand { s, f: 3, r: 0 };
        for _ in 0..310 {
            g.next();
        }
        g
    }

    /// `rand()`: 0..=RAND_MAX.
    fn next(&mut self) -> i32 {
        let v = self.s[self.f].wrapping_add(self.s[self.r]);
        self.s[self.f] = v;
        self.f = if self.f == 30 { 0 } else { self.f + 1 };
        self.r = if self.r == 30 { 0 } else { self.r + 1 };
        ((v as u32) >> 1) as i32
    }
}

// ---------------------------------------------------------------------------
// The growth engine

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Trunk,
    ShootLeft,
    ShootRight,
    Dying,
    Dead,
}

#[derive(Clone, Copy)]
enum Glyph {
    /// A step that prints nothing (a double-width leaf on an odd column).
    None,
    Wood(&'static str),
    Leaf(u16),
}

/// One growth step: what cbonsai's `mvwprintw` call would print, and where.
#[derive(Clone, Copy)]
struct Stroke {
    x: i32,
    y: i32,
    glyph: Glyph,
    style: u8,
}

struct Leaf {
    text: String,
    /// `wcwidth` of the first character, which decides on which columns it may print.
    width: i32,
}

struct Grower<'a> {
    rng: CRand,
    life_start: i32,
    multiplier: i32,
    /// `growthHeight`: the row of the pot; branches may not sink below it.
    growth_height: i32,
    leaves: &'a [Leaf],
    shoot_counter: i32,
    out: &'a mut Vec<Stroke>,
}

impl Grower<'_> {
    /// `rand() % m`
    fn roll(&mut self, m: i32) -> i32 {
        self.rng.next() % m
    }

    /// `setDeltas`
    fn set_deltas(&mut self, kind: Kind, life: i32, age: i32) -> (i32, i32) {
        let m = self.multiplier;
        match kind {
            Kind::Trunk => {
                if age <= 2 || life < 4 {
                    // new or dead trunk
                    (self.roll(3) - 1, 0)
                } else if age < m * 3 {
                    // young trunk should grow wide; `(int)(multiplier * 0.5)`,
                    // which cannot be 0 on this path.
                    let dy = if age % (m / 2).max(1) == 0 { -1 } else { 0 };
                    let dx = match self.roll(10) {
                        0 => -2,
                        1..=3 => -1,
                        4..=5 => 0,
                        6..=8 => 1,
                        _ => 2,
                    };
                    (dx, dy)
                } else {
                    // middle-aged trunk
                    let dy = if self.roll(10) > 2 { -1 } else { 0 };
                    (self.roll(3) - 1, dy)
                }
            }
            Kind::ShootLeft | Kind::ShootRight => {
                // trend left/right with little vertical movement
                let dy = match self.roll(10) {
                    0..=1 => -1,
                    2..=7 => 0,
                    _ => 1,
                };
                let dx = match self.roll(10) {
                    0..=1 => -2,
                    2..=5 => -1,
                    6..=8 => 0,
                    _ => 1,
                };
                (if kind == Kind::ShootLeft { dx } else { -dx }, dy)
            }
            Kind::Dying => {
                // discourage vertical growth, trend left/right (-3, 3)
                let dy = match self.roll(10) {
                    0..=1 => -1,
                    2..=8 => 0,
                    _ => 1,
                };
                let dx = match self.roll(15) {
                    0 => -3,
                    1..=2 => -2,
                    3..=5 => -1,
                    6..=8 => 0,
                    9..=11 => 1,
                    12..=13 => 2,
                    _ => 3,
                };
                (dx, dy)
            }
            Kind::Dead => {
                // fill in the surrounding area
                let dy = match self.roll(10) {
                    0..=2 => -1,
                    3..=6 => 0,
                    _ => 1,
                };
                (self.roll(3) - 1, dy)
            }
        }
    }

    /// `chooseColor`: a pair, sometimes bold.
    fn choose_color(&mut self, kind: Kind) -> u8 {
        match kind {
            Kind::Trunk | Kind::ShootLeft | Kind::ShootRight => {
                if self.roll(2) == 0 {
                    WOOD_BRIGHT | BOLD_BIT
                } else {
                    WOOD_DARK
                }
            }
            Kind::Dying => {
                if self.roll(10) == 0 {
                    LEAF_BRIGHT | BOLD_BIT
                } else {
                    LEAF_BRIGHT
                }
            }
            Kind::Dead => {
                if self.roll(3) == 0 {
                    LEAF_DARK | BOLD_BIT
                } else {
                    LEAF_DARK
                }
            }
        }
    }

    /// `chooseString`
    fn choose_string(&mut self, kind: Kind, life: i32, dx: i32, dy: i32) -> Glyph {
        let kind = if life < 4 { Kind::Dying } else { kind };
        Glyph::Wood(match kind {
            Kind::Trunk => match () {
                _ if dy == 0 => "/~",
                _ if dx < 0 => "\\|",
                _ if dx == 0 => "/|\\",
                _ => "|/",
            },
            Kind::ShootLeft => match () {
                _ if dy > 0 => "\\",
                _ if dy == 0 => "\\_",
                _ if dx < 0 => "\\|",
                _ if dx == 0 => "/|",
                _ => "/",
            },
            Kind::ShootRight => match () {
                _ if dy > 0 => "/",
                _ if dy == 0 => "_/",
                _ if dx < 0 => "\\|",
                _ if dx == 0 => "/|",
                _ => "/",
            },
            Kind::Dying | Kind::Dead => {
                return Glyph::Leaf(self.roll(self.leaves.len() as i32) as u16);
            }
        })
    }

    /// `branch`: grows one branch, recursing into the branches it spawns.
    fn branch(&mut self, mut y: i32, mut x: i32, kind: Kind, mut life: i32) {
        let m = self.multiplier;
        let mut shoot_cooldown = m;
        while life > 0 {
            if self.out.len() >= MAX_STEPS {
                return;
            }
            life -= 1;
            let age = self.life_start - life;
            let (dx, mut dy) = self.set_deltas(kind, life, age);

            // reduce dy if too close to the ground
            if dy > 0 && y > self.growth_height - 2 {
                dy -= 1;
            }

            let shoot = matches!(kind, Kind::ShootLeft | Kind::ShootRight);
            if life < 3 {
                // near-dead branch should branch into a lot of leaves
                self.branch(y, x, Kind::Dead, life);
            } else if kind == Kind::Trunk && life < m + 2 {
                // dying trunk should branch into a lot of leaves
                self.branch(y, x, Kind::Dying, life);
            } else if shoot && life < m + 2 {
                // dying shoot should branch into a lot of leaves
                self.branch(y, x, Kind::Dying, life);
            } else if kind == Kind::Trunk && (self.roll(3) == 0 || life % m == 0) {
                // trunks re-branch either randomly or every `multiplier` steps
                if self.roll(8) == 0 && life > 7 {
                    // not about to die: another trunk with a random life
                    shoot_cooldown = m * 2;
                    let life = life + (self.roll(5) - 2);
                    self.branch(y, x, Kind::Trunk, life);
                } else if shoot_cooldown <= 0 {
                    // otherwise a shoot, alternating sides
                    shoot_cooldown = m * 2;
                    self.shoot_counter = self.shoot_counter.wrapping_add(1);
                    let side = if self.shoot_counter.rem_euclid(2) == 0 {
                        Kind::ShootLeft
                    } else {
                        Kind::ShootRight
                    };
                    self.branch(y, x, side, life + m);
                }
            }
            shoot_cooldown -= 1;

            x += dx;
            y += dy;

            let style = self.choose_color(kind);
            let glyph = self.choose_string(kind, life, dx, dy);
            // print, but keep double-width characters from overlapping
            let width = match glyph {
                Glyph::Leaf(i) => self.leaves[i as usize].width,
                _ => 1,
            };
            let glyph = if width > 0 && x % width == 0 {
                glyph
            } else {
                Glyph::None
            };
            self.out.push(Stroke { x, y, glyph, style });
        }
    }
}

// ---------------------------------------------------------------------------
// The tree window

#[derive(Clone, Copy)]
struct GCell {
    ch: char,
    style: u8,
}

const EMPTY: GCell = GCell { ch: ' ', style: 0 };

/// cbonsai's `treeWin`: the rows above the pot, holding everything printed so far.
#[derive(Default)]
struct Grid {
    w: i32,
    /// Rows stored (the window clipped to the screen).
    h: i32,
    /// Rows of the window; printing must start inside it.
    win_h: i32,
    cells: Vec<GCell>,
    /// Bounding box of everything printed: x0, y0, x1, y1 (inclusive).
    bbox: Option<(i32, i32, i32, i32)>,
}

impl Grid {
    fn reset(&mut self, w: i32, win_h: i32, screen_h: i32) {
        self.w = w.max(0);
        self.win_h = win_h;
        self.h = win_h.clamp(0, screen_h.max(0));
        self.clear();
    }

    fn clear(&mut self) {
        self.cells.clear();
        self.cells.resize((self.w * self.h) as usize, EMPTY);
        self.bbox = None;
    }

    /// `mvwprintw(treeWin, y, x, "%s", text)`
    fn print(&mut self, x: i32, y: i32, text: &str, style: u8) {
        // curses refuses to move the cursor outside the window
        if x < 0 || y < 0 || x >= self.w || y >= self.win_h || y >= self.h {
            return;
        }
        let mut cx = x;
        for ch in text.chars() {
            let cw = char_width(ch);
            if cw <= 0 {
                continue;
            }
            // curses would wrap onto the next row here; clipping looks better
            if cx + cw > self.w {
                break;
            }
            self.put(cx, y, ch, style, cw);
            cx += cw;
        }
    }

    fn put(&mut self, x: i32, y: i32, ch: char, style: u8, cw: i32) {
        for k in 0..cw {
            self.unwide(x + k, y);
        }
        let i = (y * self.w + x) as usize;
        if cw == 2 {
            self.cells[i] = GCell {
                ch,
                style: style | WIDE_HEAD,
            };
            self.cells[i + 1] = GCell {
                ch: ' ',
                style: WIDE_TAIL,
            };
        } else {
            self.cells[i] = GCell { ch, style };
        }
        let x1 = x + cw - 1;
        self.bbox = Some(match self.bbox {
            None => (x, y, x1, y),
            Some((a, b, c, d)) => (a.min(x), b.min(y), c.max(x1), d.max(y)),
        });
    }

    /// Overwriting half of a double-width character erases the other half.
    fn unwide(&mut self, x: i32, y: i32) {
        let i = (y * self.w + x) as usize;
        let s = self.cells[i].style;
        if s & WIDE_TAIL != 0 && x > 0 {
            self.cells[i - 1] = EMPTY;
        } else if s & WIDE_HEAD != 0 && x + 1 < self.w {
            self.cells[i + 1] = EMPTY;
        }
        self.cells[i] = EMPTY;
    }
}

/// `wcwidth`: -1 for control characters, 0 for combining marks, 2 for wide
/// East Asian characters and emoji, 1 for everything else.
fn char_width(c: char) -> i32 {
    let u = c as u32;
    if u < 0x20 || (0x7f..0xa0).contains(&u) {
        return -1;
    }
    if matches!(u, 0x0300..=0x036f | 0x0483..=0x0489 | 0x0591..=0x05bd | 0x0610..=0x061a
        | 0x064b..=0x065f | 0x1ab0..=0x1aff | 0x1dc0..=0x1dff | 0x200b..=0x200f
        | 0x20d0..=0x20ff | 0xfe00..=0xfe0f | 0xfe20..=0xfe2f | 0xe0100..=0xe01ef)
    {
        return 0;
    }
    let wide = matches!(u, 0x1100..=0x115f | 0x231a..=0x231b | 0x2329..=0x232a
        | 0x23e9..=0x23ec | 0x23f0 | 0x23f3 | 0x25fd..=0x25fe | 0x2614..=0x2615
        | 0x2648..=0x2653 | 0x267f | 0x2693 | 0x26a1 | 0x26aa..=0x26ab | 0x26bd..=0x26be
        | 0x26c4..=0x26c5 | 0x26ce | 0x26d4 | 0x26ea | 0x26f2..=0x26f3 | 0x26f5 | 0x26fa
        | 0x26fd | 0x2705 | 0x270a..=0x270b | 0x2728 | 0x274c | 0x274e | 0x2753..=0x2755
        | 0x2757 | 0x2795..=0x2797 | 0x27b0 | 0x27bf | 0x2b1b..=0x2b1c | 0x2b50 | 0x2b55
        | 0x2e80..=0x303e | 0x3041..=0x33ff | 0x3400..=0x4dbf | 0x4e00..=0x9fff
        | 0xa000..=0xa4cf | 0xa960..=0xa97f | 0xac00..=0xd7a3 | 0xf900..=0xfaff
        | 0xfe10..=0xfe19 | 0xfe30..=0xfe6f | 0xff00..=0xff60 | 0xffe0..=0xffe6
        | 0x1f004 | 0x1f0cf | 0x1f18e | 0x1f191..=0x1f19a | 0x1f200..=0x1f251
        | 0x1f300..=0x1f64f | 0x1f680..=0x1f6ff | 0x1f7e0..=0x1f7eb | 0x1f90c..=0x1f9ff
        | 0x1fa70..=0x1faff | 0x20000..=0x3fffd);
    if wide { 2 } else { 1 }
}

// ---------------------------------------------------------------------------
// Pots and the message box

type Row = &'static [(&'static str, u8)];

/// `drawBase`: the art, the pair of every run, and whether it is all bold.
fn base_art(base: i64) -> (&'static [Row], bool) {
    const ONE: &[Row] = &[
        &[
            (":", TEXT),
            ("___________", LEAF_BRIGHT),
            ("./~~~\\.", WOOD_BRIGHT),
            ("___________", LEAF_BRIGHT),
            (":", TEXT),
        ],
        &[(" \\                           / ", TEXT)],
        &[("  \\_________________________/ ", TEXT)],
        &[("  (_)                     (_)", TEXT)],
    ];
    const TWO: &[Row] = &[
        &[
            ("(", TEXT),
            ("---", LEAF_BRIGHT),
            ("./~~~\\.", WOOD_BRIGHT),
            ("---", LEAF_BRIGHT),
            (")", TEXT),
        ],
        &[(" (           ) ", TEXT)],
        &[("  (_________)  ", TEXT)],
    ];
    match base {
        1 => (ONE, true),
        2 => (TWO, false),
        _ => (&[], false),
    }
}

/// `setBaseGeometry`: width and height of the pot.
fn base_size(base: i64) -> (i32, i32) {
    match base {
        1 => (31, 4),
        2 => (15, 3),
        _ => (0, 0),
    }
}

/// The message box: where the border goes and the wrapped lines inside it.
#[derive(Default)]
struct MsgBox {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    lines: Vec<Vec<(char, i32)>>,
    /// More lines than fit: the last one shows "...".
    clipped: bool,
}

/// `normalizeMessageCharacter`
fn normalize(ch: char) -> (char, i32) {
    match char_width(ch) {
        w if w < 0 => ('?', 1),
        w => (ch, w),
    }
}

/// `layoutMessage`: word-wraps `msg` to `max_w` columns. Runs of horizontal
/// whitespace become one space; newlines and blank lines are kept; words
/// longer than a line are broken. Returns the lines and the widest one.
fn layout_message(msg: &str, max_w: i32) -> (Vec<Vec<(char, i32)>>, i32) {
    struct L {
        lines: Vec<Vec<(char, i32)>>,
        width: i32,
        line: Vec<(char, i32)>,
        line_w: i32,
    }
    impl L {
        fn push_line(&mut self) {
            self.width = self.width.max(self.line_w);
            self.lines.push(std::mem::take(&mut self.line));
            self.line_w = 0;
        }
        fn append(&mut self, ch: char, w: i32) {
            self.line.push((ch, w));
            self.line_w += w;
        }
        fn flush(&mut self, word: &mut Vec<char>, add_space: &mut bool, max_w: i32) {
            if word.is_empty() {
                return;
            }
            let word_w: i32 = word
                .iter()
                .map(|&c| normalize(c).1)
                .map(|w| if w > max_w { 1 } else { w })
                .sum();
            if *add_space && !self.line.is_empty() {
                if self.line_w + 1 + word_w > max_w {
                    self.push_line();
                } else {
                    self.append(' ', 1);
                }
            }
            for &c in word.iter() {
                let (mut c, mut w) = normalize(c);
                if w > max_w {
                    (c, w) = ('?', 1);
                }
                if w > 0 && self.line_w > 0 && self.line_w + w > max_w {
                    self.push_line();
                }
                self.append(c, w);
            }
            word.clear();
            *add_space = false;
        }
    }
    let mut l = L {
        lines: Vec::new(),
        width: 0,
        line: Vec::new(),
        line_w: 0,
    };
    let mut word = Vec::new();
    let mut add_space = false;
    let mut prev_cr = false;
    for ch in msg.chars() {
        if ch == '\r' || ch == '\n' {
            if ch == '\n' && prev_cr {
                prev_cr = false;
                continue;
            }
            l.flush(&mut word, &mut add_space, max_w);
            l.push_line();
            prev_cr = ch == '\r';
            add_space = false;
            continue;
        }
        prev_cr = false;
        if ch.is_whitespace() {
            l.flush(&mut word, &mut add_space, max_w);
            add_space = !l.line.is_empty();
            continue;
        }
        word.push(ch);
    }
    l.flush(&mut word, &mut add_space, max_w);
    if !l.line.is_empty() || l.lines.is_empty() {
        l.push_line();
    }
    (l.lines, l.width)
}

/// `drawMessage` + `createMessageWindows`: the box for a `w`×`h` screen.
fn message_box(msg: &str, w: i32, h: i32) -> Option<MsgBox> {
    if msg.is_empty() || w < 5 || h < 3 {
        return None;
    }
    let max_w = ((w - 4) / 4).max(1);
    let (lines, width) = layout_message(msg, max_w);
    let content_w = if width > 0 { width } else { 1 };
    let content_h = (lines.len() as i32).min(h - 2);
    let (bw, bh) = (content_w + 4, content_h + 2);
    let mut by = ((h * 7) / 10 - 1).max(0);
    let mut bx = ((w * 7) / 10 - 2).max(0);
    if by + bh > h {
        by = h - bh;
    }
    if bx + bw > w {
        bx = w - bw;
    }
    let clipped = lines.len() as i32 > content_h;
    let mut lines = lines;
    lines.truncate(content_h as usize);
    Some(MsgBox {
        x: bx,
        y: by,
        w: bw,
        h: bh,
        lines,
        clipped,
    })
}

// ---------------------------------------------------------------------------
// Theme colors

fn hsv(c: Color) -> Option<(f32, f32, f32)> {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => color::ansi_rgb(n),
        Color::Reset => return None,
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    let h = if d <= 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    let s = if max > 0.0 { d / max } else { 0.0 };
    Some((h, s, max))
}

fn hue_gap(a: f32, b: f32) -> f32 {
    let d = (a - b).abs() % 360.0;
    d.min(360.0 - d)
}

/// The colorful accent whose hue is closest to `target`, if within `tol`
/// degrees, skipping any that would be hard to tell apart from `avoid`.
fn by_hue(colors: &[Color], target: f32, tol: f32, avoid: Option<Color>) -> Option<Color> {
    let avoid_h = avoid
        .and_then(hsv)
        .filter(|&(_, s, _)| s >= 0.2)
        .map(|(h, ..)| h);
    colors
        .iter()
        .copied()
        .filter_map(|c| {
            let (h, s, v) = hsv(c)?;
            let clash = Some(c) == avoid || avoid_h.is_some_and(|a| hue_gap(h, a) <= 25.0);
            (s >= 0.25 && v >= 0.3 && !clash).then_some((hue_gap(h, target), c))
        })
        .filter(|&(d, _)| d <= tol)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

/// The colorful accent whose hue is farthest from every color in `from`.
fn far_hue(colors: &[Color], from: [Color; 2]) -> Option<Color> {
    let hues = from.map(|c| hsv(c).filter(|&(_, s, _)| s >= 0.2).map(|(h, ..)| h));
    colors
        .iter()
        .copied()
        .filter(|c| !from.contains(c))
        .filter_map(|c| {
            let (h, s, v) = hsv(c)?;
            let gap = hues
                .iter()
                .flatten()
                .fold(180.0f32, |g, &k| g.min(hue_gap(h, k)));
            (s >= 0.2 && v >= 0.3).then_some((gap, c))
        })
        .filter(|&(gap, _)| gap >= 40.0)
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

/// The five pairs from the theme alone: leaves from its greenest accent (or
/// its first), wood from an orange/brown one (or a darker tone of its ramp),
/// the pot and the message border from an accent unlike both.
fn theme_pairs(ctx: &Ctx) -> [Color; 5] {
    let t = ctx.theme;
    let leaf = by_hue(&t.colors, 120.0, 55.0, None).unwrap_or(t.colors[0]);
    let wood = by_hue(&t.colors, 32.0, 28.0, Some(leaf)).unwrap_or_else(|| t.grad.at(0.32));
    let pot = far_hue(&t.colors, [leaf, wood]).unwrap_or_else(|| ctx.fade(t.fg, 0.4));
    // The dark pairs: faded toward the background, less so when already dark.
    let dark = |c: Color| ctx.fade(c, hsv(c).map_or(0.3, |(.., v)| (0.4 * v).clamp(0.12, 0.38)));
    [dark(leaf), dark(wood), leaf, wood, pot]
}

// ---------------------------------------------------------------------------
// Día de Muertos: the árbol de la vida

const CEMPASUCHIL: Color = rgb(0xff8c00);
const CEMPASUCHIL_LIGHT: Color = rgb(0xffaa1e);
const CEMPASUCHIL_DARK: Color = rgb(0xd66200);
const ROSA: Color = rgb(0xe4007c);
const TERCIOPELO: Color = rgb(0xb00048);
const MORADO: Color = rgb(0x7028a0);
const TURQUESA: Color = rgb(0x00aab8);
const AMARILLO: Color = rgb(0xffd200);
const LIMON: Color = rgb(0x8cc828);
const HUESO: Color = rgb(0xf4eee2);
const COBALTO: Color = rgb(0x2f5fd0);
const COBALTO_LIGHT: Color = rgb(0x7a9cf0);
const FLAMA: Color = rgb(0xffd678);
const FLAMA_EDGE: Color = rgb(0xff9628);

/// Salts the holiday's own randomness, so ordinary days draw the same numbers.
const FIESTA_SALT: u64 = 0x6d75_6572_746f_7321;

/// What a leaf becomes on the árbol de la vida.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Bloom {
    Leaf,
    /// Cempasúchil, in one of three shades.
    Marigold(u8),
    /// Terciopelo, the velvet cockscomb, in one of two.
    Velvet(u8),
    /// Nube, baby's breath.
    Nube,
}

/// A hash of a cell and a salt, 0..1.
fn hash01(x: i32, y: i32, salt: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x9e37_79b1)
        ^ (y as u32).wrapping_mul(0x85eb_ca77)
        ^ salt.wrapping_mul(0xc2b2_ae3d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297a_2d39);
    h ^= h >> 15;
    (h >> 8) as f32 / (1u32 << 24) as f32
}

/// What the leaf at a cell of tree `seed` is today. Blossoms come in clumps,
/// more of them as the holiday builds up.
fn bloom(x: i32, y: i32, seed: u32, level: f32) -> Bloom {
    // Clumps about four cells wide and two tall, staggered row to row.
    let (cx, cy) = (
        (x + 2 * (y.div_euclid(2) & 1)).div_euclid(4),
        y.div_euclid(2),
    );
    let clump = hash01(cx, cy, seed ^ 0x636c_756d);
    let amount = 0.3 + 0.6 * level.clamp(0.0, 1.0);
    let p = if clump < amount { 0.86 } else { 0.1 };
    let cell = hash01(x, y, seed);
    if cell >= p {
        return Bloom::Leaf;
    }
    let shade = (hash01(x, y, seed ^ 0x7368_6164) * 3.0) as u8;
    match hash01(cx, cy, seed ^ 0x6b69_6e64) {
        k if k < 0.6 => Bloom::Marigold(shade),
        k if k < 0.9 => Bloom::Velvet(shade & 1),
        _ => Bloom::Nube,
    }
}

/// Día de Muertos' colors.
struct Dress {
    marigold: [Color; 3],
    velvet: [Color; 2],
    nube: Color,
    /// The talavera pot: cobalt, a lighter cobalt, the white glaze, a yellow accent.
    cobalt: [Color; 2],
    glaze: Color,
    yellow: Color,
    /// The calavera: bone and its icing.
    bone: Color,
    icing: [Color; 4],
    /// The veladoras' glass, and wax.
    glass: [Color; 3],
    wax: Color,
    /// Pan de muerto: its crust and its sugared bones.
    crust: Color,
    sugar: Color,
}

impl Dress {
    const CLASSIC: Dress = Dress {
        marigold: [CEMPASUCHIL_DARK, CEMPASUCHIL, CEMPASUCHIL_LIGHT],
        velvet: [TERCIOPELO, ROSA],
        nube: HUESO,
        cobalt: [COBALTO, COBALTO_LIGHT],
        glaze: HUESO,
        yellow: AMARILLO,
        bone: HUESO,
        icing: [ROSA, TURQUESA, AMARILLO, LIMON],
        glass: [ROSA, TURQUESA, MORADO],
        wax: rgb(0xe6d8b8),
        crust: rgb(0xc47830),
        sugar: rgb(0xf0c88c),
    };

    /// The same dress in the theme's colors. The marigolds keep their own hue
    /// when the theme has nothing close.
    fn themed(ctx: &Ctx, pot: Color) -> Dress {
        let t = ctx.theme;
        let mari = by_hue(&t.colors, 32.0, 30.0, None).unwrap_or(CEMPASUCHIL);
        let velvet =
            by_hue(&t.colors, 335.0, 40.0, Some(mari)).unwrap_or_else(|| ctx.pick(1, ROSA));
        let cobalt = by_hue(&t.colors, 222.0, 45.0, Some(mari)).unwrap_or(pot);
        let yellow = by_hue(&t.colors, 52.0, 22.0, Some(mari)).unwrap_or(mari);
        Dress {
            marigold: [ctx.fade(mari, 0.25), mari, mari.scale(1.18)],
            velvet: [ctx.fade(velvet, 0.25), velvet],
            nube: t.fg,
            cobalt: [cobalt, cobalt.scale(1.3)],
            glaze: t.fg,
            yellow,
            bone: t.fg,
            icing: [velvet, ctx.pick(2, TURQUESA), yellow, ctx.pick(3, LIMON)],
            glass: [velvet, ctx.pick(2, TURQUESA), ctx.pick(0, MORADO)],
            wax: ctx.fade(t.fg, 0.15),
            crust: mari.lerp(t.grad.at(0.3), 0.4),
            sugar: mari.lerp(t.fg, 0.55),
        }
    }
}

/// A petal falling from a blossom, or lying where it fell.
#[derive(Clone, Copy)]
struct Petal {
    x: f32,
    y: f32,
    /// Where it sways around, and how fast it falls.
    x0: f32,
    vy: f32,
    phase: f32,
    /// Seconds left lying on the ground; below zero while falling.
    rest: f32,
    /// How it lies once it lands.
    lie: char,
    /// Its two sides, which take turns as it tumbles.
    color: Color,
    shade: Color,
}

/// At most this many petals at once.
const PETALS: usize = 48;

/// A little picture drawn by hand: its characters, and an ink letter for
/// each (see `Bonsai::ink`; a space leaves the cell alone).
struct Sprite {
    art: &'static [&'static str],
    ink: &'static [&'static str],
}

/// The talavera pots, the same size as cbonsai's so the tree grows the same:
/// cobalt on white glaze, and the moss turned into marigold petals.
const TALAVERA_ONE: Sprite = Sprite {
    art: &[
        ":___________./~~~\\.___________:",
        " \\~@~.~@~.~@~.~@~.~@~.~@~.~@~/ ",
        "  \\_________________________/ ",
        "  (_)                     (_)",
    ],
    ink: &[
        "bpppppppppppWWWWWWWpppppppppppb",
        " bwBwywBwywBwywBwywBwywBwywBwb ",
        "  bbbbbbbbbbbbbbbbbbbbbbbbbbbb ",
        "  bbb                     bbb",
    ],
};

const TALAVERA_TWO: Sprite = Sprite {
    art: &["(---./~~~\\.---)", " (~@~.~@~.~@~) ", "  (_________)  "],
    ink: &["bpppWWWWWWWpppb", " bwBwywBwywBwb ", "  bbbbbbbbbbb  "],
};

/// A calavera de azúcar: icing on the brow, flowers for eyes.
const CALAVERA: Sprite = Sprite {
    art: &[
        "  .-\"\"\"-.  ",
        " / * @ * \\ ",
        "| (@) (@) |",
        " \\   ^   / ",
        "  |'|'|'|  ",
    ],
    ink: &[
        "  hhhhhhh  ",
        " h k i l h ",
        "h hjh hkh h",
        " h   i   h ",
        "  hhhhhhh  ",
    ],
};

/// A smaller one, for small screens.
const CALAVERITA: Sprite = Sprite {
    art: &[" .-*-. ", "(@) (@)", " \\ ^ / ", "  www  "],
    ink: &[" hhihh ", "hjhhhkh", " h i h ", "  hhh  "],
};

/// Veladoras: candles in colored glass. `t` and `f` are the tip and the
/// heart of the flame.
const VELADORA: Sprite = Sprite {
    art: &[" t ", " f ", "|~|", "|+|", "|_|"],
    ink: &[" t ", " f ", "gxg", "gyg", "ggg"],
};

const VELADORA_SHORT: Sprite = Sprite {
    art: &[" t ", " f ", "|~|", "|_|"],
    ink: &[" t ", " f ", "gxg", "ggg"],
};

/// A wax taper in a clay holder.
const CIRIO: Sprite = Sprite {
    art: &[" t ", " f ", " | ", " | ", " | ", "_|_"],
    ink: &[" t ", " f ", " x ", " x ", " x ", "dxd"],
};

/// Pan de muerto: a round loaf with its bones crossed on top.
const PAN: Sprite = Sprite {
    art: &["  _o_  ", " /=X=\\ ", "(_____)"],
    ink: &["  dsd  ", " dsssd ", "ddddddd"],
};

/// The ofrenda's pieces besides the calavera, in the order they come as the
/// holiday builds up: from what level, right of the pot or left, how many
/// columns away, and which.
const OFRENDA: [(f32, bool, i32, &Sprite); 6] = [
    (0.0, false, 2, &VELADORA),
    (0.0, true, 2, &VELADORA),
    (0.4, true, 6, &CIRIO),
    (0.55, true, 14, &PAN),
    (0.75, true, 10, &VELADORA_SHORT),
    (0.95, false, 19, &CIRIO),
];

// ---------------------------------------------------------------------------
// Navidad: the tree trimmed the Mexican way

const ROJO_NOCHEBUENA: Color = rgb(0xc8102e);
const VERDE_PINO: Color = rgb(0x1f6f3a);
const DORADO: Color = rgb(0xf2b632);
const PLATA: Color = rgb(0xc9ccd6);
const BLANCO: Color = rgb(0xf4f1ea);
const HENO: Color = rgb(0x9aa58a);
const MUSGO: Color = rgb(0x4f7a28);
const MUSGO_LIGHT: Color = rgb(0x6f9a3a);
const CANELA: Color = rgb(0x8b4a1e);
const CANA: Color = rgb(0xc9b26b);
const PIEL: Color = rgb(0xdca878);
const PIEL_OSCURA: Color = rgb(0x7a4a2a);
const AZUL_MARIA: Color = rgb(0x3a63c8);
const CAFE: Color = rgb(0x8a5a2b);
const GRIS: Color = rgb(0x9a9a9e);

/// The foquitos of the series: rojo, verde, azul, amarillo, rosa, naranja.
const FOQUITOS: [Color; 6] = [
    rgb(0xff3b3b),
    rgb(0x3bd16f),
    rgb(0x3b7bff),
    rgb(0xffd23b),
    rgb(0xff5fb4),
    rgb(0xff8c1a),
];

/// Seconds between one step of the series' blinking and the next.
const BLINK: f32 = 0.4;

/// Colors by ink letter: `a` to `z`, then `A` to `Z`.
#[derive(Clone, Copy)]
struct Inks([Color; 52]);

impl Inks {
    fn new(colors: &[(u8, Color)]) -> Inks {
        let mut inks = Inks([Color::Reset; 52]);
        for &(k, color) in colors {
            inks.0[Inks::slot(k)] = color;
        }
        inks
    }

    fn slot(k: u8) -> usize {
        match k {
            b'a'..=b'z' => (k - b'a') as usize,
            b'A'..=b'Z' => 26 + (k - b'A') as usize,
            _ => 0,
        }
    }

    fn get(&self, k: u8) -> Color {
        self.0[Inks::slot(k)]
    }
}

/// How much of the Navidad has been put up: the tree and the nacimiento fill
/// up through the posadas, and stay complete from Nochebuena on.
fn built(f: Fiesta) -> f32 {
    if f.month == 12 && f.day <= 24 {
        f.level
    } else {
        1.0
    }
}

/// The Niño Dios lies in the pesebre from Nochebuena on.
fn nino_dios(f: Fiesta) -> bool {
    f.month == 1 || f.day >= 24
}

/// How far the Reyes Magos still are from the pesebre, 1 (at the edge of the
/// screen) to 0 (arrived, on the 6th); `None` while they have not set out.
fn reyes_away(f: Fiesta) -> Option<f32> {
    f.reyes().then(|| (6 - f.day.min(6)) as f32 / 4.0)
}

/// The Navidad's colors.
struct Navidad {
    inks: Inks,
    /// Esferas: rojo, dorado, plata, blanco.
    esferas: [Color; 4],
    luces: [Color; 6],
    heno: Color,
    star: Color,
}

impl Navidad {
    fn classic(pairs: &[Color; 5]) -> Navidad {
        Navidad::make(
            pairs,
            [
                ROJO_NOCHEBUENA,
                VERDE_PINO,
                DORADO,
                PLATA,
                BLANCO,
                AZUL_MARIA,
            ],
            FOQUITOS,
            [MUSGO, MUSGO_LIGHT],
            [CANELA, CANA, HENO, PIEL, PIEL_OSCURA, CAFE, GRIS, MORADO],
        )
    }

    /// The same dress in the theme's colors.
    fn themed(ctx: &Ctx, pairs: &[Color; 5]) -> Navidad {
        let t = ctx.theme;
        let leaf = pairs[LEAF_BRIGHT as usize];
        let wood = pairs[WOOD_BRIGHT as usize];
        let by = |hue, tol, avoid, i, native| {
            by_hue(&t.colors, hue, tol, avoid).unwrap_or_else(|| ctx.pick(i, native))
        };
        let rojo = by(355.0, 35.0, None, 0, ROJO_NOCHEBUENA);
        let dorado = by(45.0, 25.0, Some(rojo), 1, DORADO);
        let azul = by(220.0, 45.0, Some(rojo), 2, AZUL_MARIA);
        let rosa = by(325.0, 40.0, Some(rojo), 4, FOQUITOS[4]);
        let morado = by(275.0, 45.0, Some(azul), 3, MORADO);
        let naranja = by_hue(&t.colors, 28.0, 22.0, Some(rojo)).unwrap_or(wood);
        let plata = ctx.fade(t.fg, 0.3);
        let heno = ctx.fade(leaf.lerp(t.fg, 0.5), 0.2);
        Navidad::make(
            pairs,
            [rojo, leaf, dorado, plata, t.fg, azul],
            [rojo, leaf, azul, dorado, rosa, naranja],
            [ctx.fade(leaf, 0.35), leaf],
            [
                wood,
                dorado.lerp(t.fg, 0.3),
                heno,
                wood.lerp(t.fg, 0.4),
                wood.lerp(rojo, 0.3),
                wood,
                plata,
                morado,
            ],
        )
    }

    /// `main`: rojo, verde, dorado, plata, blanco, azul; `luces`; the musgo's
    /// two shades; `rest`: canela, caña, heno, piel, piel oscura, café, gris,
    /// morado.
    fn make(
        pairs: &[Color; 5],
        main: [Color; 6],
        luces: [Color; 6],
        musgo: [Color; 2],
        rest: [Color; 8],
    ) -> Navidad {
        let [rojo, verde, dorado, plata, blanco, azul] = main;
        let [canela, cana, heno, piel, oscura, cafe, gris, morado] = rest;
        let inks = Inks::new(&[
            (b'r', rojo),
            (b'g', verde),
            (b'o', dorado),
            (b'p', plata),
            (b'w', blanco),
            (b'b', azul),
            (b'm', musgo[0]),
            (b'l', musgo[1]),
            (b't', canela),
            (b'c', cana),
            (b'h', heno),
            (b's', piel),
            (b'q', oscura),
            (b'n', cafe),
            (b'y', gris),
            (b'u', morado),
            (b'W', pairs[WOOD_BRIGHT as usize]),
        ]);
        Navidad {
            inks,
            esferas: [rojo, dorado, plata, blanco],
            luces,
            heno,
            star: dorado,
        }
    }
}

/// What hangs on a cell of the tree today.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Adorno {
    None,
    /// A glass ball, in one of the four colors.
    Esfera(u8),
    /// A bulb of the series, numbered along the string.
    Foquito(i32),
}

/// What the tree wears at a cell of tree `seed`. The series of foquitos runs
/// along the branches, a bulb every fifth step; the esferas hang from the
/// leaves, most where nothing below them hides them. Both fill up with
/// `built`.
fn adorno(x: i32, y: i32, seed: u32, built: f32, wood: bool, hangs: bool) -> Adorno {
    let b = built.clamp(0.0, 1.0);
    let n = x + 3 * y;
    if n.rem_euclid(5) == 0 {
        let p = if wood { 0.3 + 0.65 * b } else { 0.1 + 0.3 * b };
        if hash01(x, y, seed ^ 0x6c75_6365) < p {
            return Adorno::Foquito(n.div_euclid(5));
        }
    }
    if !wood {
        let p = (0.03 + 0.07 * b) * if hangs { 2.5 } else { 1.0 };
        if hash01(x, y, seed ^ 0x6573_6665) < p {
            let k = (hash01(x, y, seed ^ 0x636f_6c6f) * 4.0) as u8;
            return Adorno::Esfera(k.min(3));
        }
    }
    Adorno::None
}

// ---------------------------------------------------------------------------
// Fiestas Patrias: a tree in the colors of the flag

const VERDE_BANDERA: Color = rgb(0x006847);
const VERDE_CLARO: Color = rgb(0x1b9a63);
const ROJO_BANDERA: Color = rgb(0xce1126);
const ROJO_TUNA: Color = rgb(0xe8334a);
const AGUILA: Color = rgb(0x7a4a1e);
const AGUILA_CLARA: Color = rgb(0xa8743a);
const NOPAL: Color = rgb(0x4f8a2a);
const NOPAL_CLARO: Color = rgb(0x6fae3e);
const ORO: Color = rgb(0xd4a017);
const PIEDRA: Color = rgb(0x8d8a86);
const AGUA: Color = rgb(0x3a8fb7);

/// What a leaf becomes on the tree of the Fiestas Patrias.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tri {
    Leaf,
    /// A white blossom, in one of two shades.
    Blanco(u8),
    /// A red flower or tuna, in one of two.
    Rojo(u8),
}

/// What the leaf at a cell of tree `seed` is today, given the pot's center
/// column `cx`. Blossoms come in clumps, white around the middle of the tree
/// and red toward the right, so the canopy reads left to right as the flag
/// does: verde, blanco, rojo. More of them as the month climbs.
fn tricolor(x: i32, y: i32, cx: i32, seed: u32, level: f32) -> Tri {
    let (gx, gy) = (
        (x + 2 * (y.div_euclid(2) & 1)).div_euclid(4),
        y.div_euclid(2),
    );
    let clump = hash01(gx, gy, seed ^ 0x7472_6963);
    let t = ((x - cx) as f32 / 44.0 + 0.5).clamp(0.0, 1.0);
    let amount = 0.35 + 0.65 * level.clamp(0.0, 1.0);
    let white = (0.1 + 0.6 * (1.0 - ((t - 0.45).abs() / 0.28).min(1.0))) * amount;
    let red = (0.06 + 0.7 * ((t - 0.5) / 0.4).clamp(0.0, 1.0)) * amount;
    let cell = hash01(x, y, seed ^ 0x7061_7472);
    let shade = ((hash01(x, y, seed ^ 0x7368_6164) * 2.0) as u8).min(1);
    if clump < white {
        if cell < 0.8 {
            return Tri::Blanco(shade);
        }
    } else if clump < white + red {
        if cell < 0.8 {
            return Tri::Rojo(shade);
        }
    } else if cell < 0.05 {
        // a few strays among the green
        return if hash01(x, y, seed ^ 0x7374_7279) < t {
            Tri::Rojo(shade)
        } else {
            Tri::Blanco(shade)
        };
    }
    Tri::Leaf
}

/// The Fiestas Patrias' colors.
struct Patrias {
    inks: Inks,
    /// The leaves: dark and bright.
    verde: [Color; 2],
    /// White blossoms and red flowers, two shades each.
    blanco: [Color; 2],
    rojo: [Color; 2],
    /// The banderitas: verde, blanco, rojo.
    flag: [Color; 3],
}

impl Patrias {
    /// The flag's own greens, unless the user chose the leaves' colors.
    fn classic(pairs: &[Color; 5], own_leaves: bool) -> Patrias {
        let verde = if own_leaves {
            [pairs[LEAF_DARK as usize], pairs[LEAF_BRIGHT as usize]]
        } else {
            [VERDE_BANDERA, VERDE_CLARO]
        };
        Patrias::make(
            pairs,
            verde,
            [BLANCO, BLANCO.lerp(CANA, 0.35)],
            [ROJO_BANDERA, ROJO_TUNA],
            [VERDE_BANDERA, BLANCO, ROJO_BANDERA],
            [
                AGUILA,
                AGUILA_CLARA,
                NOPAL,
                NOPAL_CLARO,
                ORO,
                PIEDRA,
                AGUA,
                PLATA,
            ],
        )
    }

    /// The same dress in the theme's colors.
    fn themed(ctx: &Ctx, pairs: &[Color; 5]) -> Patrias {
        let t = ctx.theme;
        let leaf = pairs[LEAF_BRIGHT as usize];
        let wood = pairs[WOOD_BRIGHT as usize];
        let rojo =
            by_hue(&t.colors, 355.0, 35.0, None).unwrap_or_else(|| ctx.pick(0, ROJO_BANDERA));
        let oro = by_hue(&t.colors, 45.0, 25.0, Some(rojo)).unwrap_or_else(|| ctx.pick(1, ORO));
        let agua = by_hue(&t.colors, 200.0, 45.0, Some(rojo)).unwrap_or_else(|| ctx.pick(2, AGUA));
        let nopal = by_hue(&t.colors, 100.0, 40.0, None).unwrap_or(leaf);
        Patrias::make(
            pairs,
            [pairs[LEAF_DARK as usize], leaf],
            [t.fg, ctx.fade(t.fg, 0.2)],
            [rojo, rojo.scale(1.15)],
            [leaf, t.fg, rojo],
            [
                wood,
                wood.lerp(t.fg, 0.3),
                nopal,
                nopal.scale(1.2),
                oro,
                ctx.fade(t.fg, 0.4),
                agua,
                ctx.fade(t.fg, 0.2),
            ],
        )
    }

    /// `rest`: águila, águila clara, nopal, nopal claro, oro, piedra, agua,
    /// serpiente.
    fn make(
        pairs: &[Color; 5],
        verde: [Color; 2],
        blanco: [Color; 2],
        rojo: [Color; 2],
        flag: [Color; 3],
        rest: [Color; 8],
    ) -> Patrias {
        let [aguila, clara, nopal, claro, oro, piedra, agua, serpiente] = rest;
        let inks = Inks::new(&[
            (b'g', flag[0]),
            (b'w', flag[1]),
            (b'r', flag[2]),
            (b'e', aguila),
            (b'f', clara),
            (b'n', nopal),
            (b'N', claro),
            (b'o', oro),
            (b'k', piedra),
            (b'a', agua),
            (b's', serpiente),
            (b'x', rojo[1]),
            (b'W', pairs[WOOD_BRIGHT as usize]),
        ]);
        Patrias {
            inks,
            verde,
            blanco,
            rojo,
            flag,
        }
    }
}

// ---------------------------------------------------------------------------
// The sprites of both: ink letters are per holiday (see `Navidad::make` and
// `Patrias::make`); a space leaves the cell alone and a `.` clears it.

/// Cbonsai's pot one in foil, with nochebuenas on the soil: red bracts, a
/// golden heart, green leaves between them.
const NOCHEBUENA_ONE: Sprite = Sprite {
    art: &[
        r":___________./~~~\.___________:",
        r" \(*)v(*)v(*)v(*)v(*)v(*)v(*)/ ",
        r"  \_________________________/ ",
        r"  (_)                     (_)",
    ],
    ink: &[
        "ooooooooooooWWWWWWWoooooooooooo",
        " rrorgrorgrorgrorgrorgrorgrorr ",
        "  rrrrrrrrrrrrrrrrrrrrrrrrrrr ",
        "  rrr                     rrr",
    ],
};

const NOCHEBUENA_TWO: Sprite = Sprite {
    art: &[r"(---./~~~\.---)", r" ((*)v(*)v(*)) ", r"  (_________)  "],
    ink: &["ooooWWWWWWWoooo", " rrorgrorgrorr ", "  rrrrrrrrrrr  "],
};

/// The same pots banded verde, blanco and rojo like the flag, a banderita
/// planted in each (see `Bonsai::draw_banderitas`).
const PATRIA_ONE: Sprite = Sprite {
    art: &[
        r":___________./~~~\.___________:",
        r" \~.~.~.~.~.~.~.~.~.~.~.~.~.~/ ",
        r"  \_________________________/ ",
        r"  (_)                     (_)",
    ],
    ink: &[
        "ooooooooooooWWWWWWWoooooooooooo",
        " gggggggggwwwwwwwwwwwrrrrrrrrr ",
        "  ggggggggwwwwwwwwwwwrrrrrrrr ",
        "  ggg                     rrr",
    ],
};

const PATRIA_TWO: Sprite = Sprite {
    art: &[r"(---./~~~\.---)", r" (~.~.~.~.~.~) ", r"  (_________)  "],
    ink: &["ooooWWWWWWWoooo", " ggggwwwwwrrrr ", "  gggwwwwwrrr  "],
};

/// The pesebre: a thatched roof on two posts, 26 wide. The holy family and
/// the animals stand between the posts (see `NACIMIENTO`).
const PORTAL: Sprite = Sprite {
    art: &[
        r"        .-~~~~~~-.        ",
        r"    .-'~~~~~~~~~~~~'-.    ",
        r" /~~~~~~~~~~~~~~~~~~~~~~\ ",
        r"  |                    |  ",
        r"  |                    |  ",
        r"  |                    |  ",
    ],
    ink: &[
        "        cccccccccc        ",
        "    cccccccccccccccccc    ",
        " tcccccccccccccccccccccct ",
        "  t                    t  ",
        "  t                    t  ",
        "  t                    t  ",
    ],
};
const PORTAL_W: i32 = 26;

/// A narrower pesebre for terminals without room for the full one: the holy
/// family without the animals, 18 wide.
const PORTAL_COMPACT: Sprite = Sprite {
    art: &[
        r"      .-~~-.      ",
        r"   .-'~~~~~~'-.   ",
        r" /~~~~~~~~~~~~~~\ ",
        r"  |            |  ",
        r"  |            |  ",
        r"  |            |  ",
    ],
    ink: &[
        "      cccccc      ",
        "   cccccccccccc   ",
        " tcccccccccccccct ",
        "  t            t  ",
        "  t            t  ",
        "  t            t  ",
    ],
};
const COMPACT_W: i32 = 18;

/// The manger, straw and all; the Niño Dios lies in it from Nochebuena.
const CUNA: Sprite = Sprite {
    art: &["~~~~~", r"\___/"],
    ink: &["ccccc", "ttttt"],
};

/// The Niño Dios, rays around him.
const NINO: Sprite = Sprite {
    art: &[r"\|/", "(o)"],
    ink: &["ooo", "oso"],
};

const OX: Sprite = Sprite {
    art: &["u--u", "(oo)"],
    ink: &["wnnw", "nwwn"],
};

const MULA: Sprite = Sprite {
    art: &[r"/  \", "(..)"],
    ink: &["y  y", "yyyy"],
};

const MARIA: Sprite = Sprite {
    art: &["(o)", r"/#\", r"/_\"],
    ink: &["oso", "bwb", "bbb"],
};

const JOSE: Sprite = Sprite {
    art: &["(o)*", r"/#\|", r"/ \|"],
    ink: &["osow", "nnnt", "n nt"],
};

const PASTOR_A: Sprite = Sprite {
    art: &["_o_)", r"/#\|", r"/ \|"],
    ink: &["nsnt", "wwwt", "w wt"],
};

const PASTOR_B: Sprite = Sprite {
    art: &["_o_)", r"/#\|", r"/ \|"],
    ink: &["nsnt", "uuut", "u ut"],
};

const OVEJA: Sprite = Sprite {
    art: &["(@@@)o", " || ||"],
    ink: &["wwwwwy", " yy yy"],
};

const ANGEL: Sprite = Sprite {
    art: &[r"\(o)/", r" /#\ ", "  V  "],
    ink: &["wosow", " www ", "  w  "],
};

/// The Reyes Magos, three wide and a column of room, solid so that they stand
/// cleanly in front of whatever they walk past: Melchor with oro, Gaspar with
/// incienso, Baltasar with mirra.
const MELCHOR: Sprite = Sprite {
    art: &[" ^  ", "(_) ", r"/$\ ", r"/ \ "],
    ink: &[".o..", "sss.", "ror.", "r.r."],
};

const GASPAR: Sprite = Sprite {
    art: &[" ^  ", "(_) ", r"/*\ ", r"/ \ "],
    ink: &[".o..", "sss.", "uwu.", "u.u."],
};

const BALTASAR: Sprite = Sprite {
    art: &[" ^  ", "(_) ", r"/&\ ", r"/ \ "],
    ink: &[".o..", "qqq.", "bnb.", "b.b."],
};

/// The nacimiento's figures inside the pesebre, in the order they join as the
/// posadas go: from what level, how many columns from the portal's left edge,
/// which.
const NACIMIENTO: [(f32, i32, &Sprite); 4] = [
    (0.4, 3, &OX),
    (0.5, 16, &MARIA),
    (0.6, 7, &JOSE),
    (0.47, 19, &MULA),
];

/// The same for the narrower pesebre.
const NACIMIENTO_COMPACT: [(f32, i32, &Sprite); 2] = [(0.5, 12, &MARIA), (0.6, 3, &JOSE)];

/// The pastores and their sheep, in the order they gather: from what level,
/// how many columns from the pot, which. They stand on the side of the pot
/// opposite the pesebre.
const PASTORES: [(f32, i32, &Sprite); 6] = [
    (0.0, 1, &OVEJA),
    (0.65, 8, &PASTOR_A),
    (0.75, 13, &OVEJA),
    (0.82, 20, &PASTOR_B),
    (0.89, 25, &OVEJA),
    (1.0, 32, &OVEJA),
];
const PASTORES_W: i32 = 40;

// ---------------------------------------------------------------------------
// The emblem at the foot of the Fiestas Patrias' tree: an eagle on a nopal
// that grows from a rock in the lake.

/// The rock in the lake, ripples around it.
const ROCA: Sprite = Sprite {
    art: &[r"    ,-^^^^^^^-,    ", r"~ ~ /#########\ ~ ~"],
    ink: &["    kkkkkkkkkkk    ", "a a kkkkkkkkkkk a a"],
};
const EMBLEMA_W: i32 = 19;

/// A pad of the nopal.
const PENCA: Sprite = Sprite {
    art: &["(:)"],
    ink: &["nNn"],
};

/// The eagle in profile, its wings raised, perched on the nopal; the
/// serpent's place is the two columns before the beak.
const AGUILA_SPRITE: Sprite = Sprite {
    art: &[
        r"    ,\\  |  //,",
        r"  <(o)\\\|///-'",
        r"     `-\\||//-'",
        r"        /||\   ",
    ],
    ink: &[
        "    eff  e  ffe",
        "  oewefffefffee",
        "     ffeeeeeeff",
        "        oooo   ",
    ],
};

/// The serpent in the eagle's beak, hanging from it.
const SERPIENTE: Sprite = Sprite {
    art: &["~s", ".~", "~'"],
    ink: &["ss", "ss", "ss"],
};

// ---------------------------------------------------------------------------
// The scene

/// The holiday the tree is dressed for. Any other holiday leaves it as it is
/// on an ordinary day.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Look {
    Plain,
    Muertos,
    Navidad,
    Patrias,
}

impl Look {
    fn of(f: Fiesta) -> Look {
        match f.kind {
            fiesta::Kind::Muertos => Look::Muertos,
            fiesta::Kind::Navidad => Look::Navidad,
            fiesta::Kind::Patrias => Look::Patrias,
            _ => Look::Plain,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Growing,
    /// The tree is complete; `hold` counts toward `wait`, in real seconds.
    Done,
}

struct Bonsai {
    // options
    life: i32,
    multiplier: i32,
    base: i64,
    leaves: Vec<Leaf>,
    message: String,
    step: f64,
    wait: f64,
    live: bool,
    infinite: bool,
    user_colors: Option<[Color; 4]>,
    geometry: Option<(i32, i32)>,

    /// Seeds the trees after the first, like cbonsai's `srand(time(NULL))`.
    seeder: fastrand::Rng,
    seed: u32,

    // layout
    w: i32,
    h: i32,
    base_x: i32,
    base_y: i32,

    // growth
    strokes: Vec<Stroke>,
    shown: usize,
    acc: f64,
    hold: f64,
    /// Scene seconds per real second, from the last tick: growing follows
    /// scene time, the wait between trees the wall clock.
    pace: f64,
    phase: Phase,
    grown: bool,
    grid: Grid,
    msg: Option<MsgBox>,

    pairs: [Color; 5],
    msg_fg: Color,

    // The holidays
    fiesta: Fiesta,
    look: Look,
    dress: Dress,
    navidad: Navidad,
    patrias: Patrias,
    /// Leaves may turn into blossom glyphs (only with the default `&` leaves;
    /// leaves the user chose keep their glyph and only change color).
    bloom_glyphs: bool,
    /// The holiday's own randomness.
    frng: fastrand::Rng,
    petals: Vec<Petal>,
    /// Petals owed, and seconds for the candles' flicker and the banderitas'
    /// waving (scene time), and for the series' blinking (wall clock).
    spawn: f32,
    clock: f32,
}

impl Bonsai {
    fn new(ctx: &Ctx) -> Bonsai {
        let o = ctx.opts;
        let leaves: Vec<Leaf> = o
            .s("leaves")
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| {
                // cbonsai copies at most 31 bytes of a leaf
                let mut end = s.len().min(31);
                while !s.is_char_boundary(end) {
                    end -= 1;
                }
                let text = s[..end].to_string();
                let width = text.chars().next().map_or(0, char_width);
                Leaf { text, width }
            })
            .collect();
        let leaves = if leaves.is_empty() {
            vec![Leaf {
                text: "&".into(),
                width: 1,
            }]
        } else {
            leaves
        };
        let user_colors = {
            let v: Vec<Color> = o
                .s("colors")
                .split(',')
                .filter_map(|s| Color::parse(s.trim()))
                .collect();
            <[Color; 4]>::try_from(v).ok()
        };
        let geometry = o.s("geometry").split_once(',').and_then(|(r, c)| {
            let (r, c) = (r.trim().parse::<u16>().ok()?, c.trim().parse::<u16>().ok()?);
            Some((r as i32, c as i32))
        });
        let mut seeder = ctx.rng();
        let seed = match o.i("seed") {
            s if s > 0 => s as u32,
            _ => seeder.u32(..),
        };
        let bloom_glyphs = leaves.len() == 1 && leaves[0].text == "&";
        Bonsai {
            life: o.i("life").clamp(0, 200) as i32,
            multiplier: o.i("multiplier").clamp(1, 20) as i32,
            base: o.i("base"),
            leaves,
            message: o.s("message").to_string(),
            step: o.f64("step").clamp(1e-4, 60.0),
            wait: o.f64("wait").clamp(0.0, 86_400.0),
            live: o.b("live"),
            infinite: o.b("infinite"),
            user_colors,
            geometry,
            seeder,
            seed,
            w: 0,
            h: 0,
            base_x: 0,
            base_y: 0,
            strokes: Vec::new(),
            shown: 0,
            acc: 0.0,
            hold: 0.0,
            pace: 1.0,
            phase: Phase::Growing,
            grown: false,
            grid: Grid::default(),
            msg: None,
            pairs: CLASSIC,
            msg_fg: Color::Reset,
            fiesta: ctx.fiesta,
            look: Look::of(ctx.fiesta),
            dress: Dress::CLASSIC,
            navidad: Navidad::classic(&CLASSIC),
            patrias: Patrias::classic(&CLASSIC, false),
            bloom_glyphs,
            frng: fastrand::Rng::with_seed(ctx.seed ^ FIESTA_SALT),
            petals: Vec::with_capacity(PETALS),
            spawn: 0.0,
            clock: 0.0,
        }
    }

    /// `growTree`: records every step of the tree for the current seed and size.
    fn grow(&mut self) {
        self.strokes.clear();
        let (bw, _) = base_size(self.base);
        let mut g = Grower {
            rng: CRand::new(self.seed),
            life_start: self.life,
            multiplier: self.multiplier,
            growth_height: self.base_y,
            leaves: &self.leaves,
            shoot_counter: 0,
            out: &mut self.strokes,
        };
        g.shoot_counter = g.rng.next();
        g.branch(
            self.base_y - 1,
            self.base_x + bw / 2,
            Kind::Trunk,
            self.life,
        );
        self.grown = true;
    }

    /// A new tree from the current seed.
    fn start_tree(&mut self) {
        self.grow();
        self.grid.clear();
        self.shown = 0;
        self.acc = 0.0;
        self.hold = 0.0;
        self.phase = Phase::Growing;
        if !self.live {
            self.reveal(usize::MAX);
            self.phase = Phase::Done;
        }
    }

    /// Prints the next `n` recorded steps into the tree window.
    fn reveal(&mut self, n: usize) {
        let end = self.shown.saturating_add(n).min(self.strokes.len());
        for s in &self.strokes[self.shown..end] {
            let text = match s.glyph {
                Glyph::None => continue,
                Glyph::Wood(t) => t,
                Glyph::Leaf(i) => &self.leaves[i as usize].text,
            };
            self.grid.print(s.x, s.y, text, s.style);
        }
        self.shown = end;
    }

    fn draw_base(&self, c: &mut Canvas) {
        let (rows, bold) = base_art(self.base);
        let attr = if bold { BOLD } else { 0 };
        for (dy, row) in rows.iter().enumerate() {
            let mut x = self.base_x;
            for &(run, pair) in row.iter() {
                let fg = self.pairs[pair as usize];
                for ch in run.chars() {
                    if ch != ' ' {
                        c.set_attr(x, self.base_y + dy as i32, ch, fg, attr);
                    }
                    x += 1;
                }
            }
        }
    }

    fn draw_message(&self, c: &mut Canvas) {
        let Some(m) = &self.msg else { return };
        let border = self.pairs[TEXT as usize];
        let (x1, y1) = (m.x + m.w - 1, m.y + m.h - 1);
        for y in m.y..=y1 {
            for x in m.x..=x1 {
                let ch = match (x == m.x || x == x1, y == m.y || y == y1) {
                    (true, true) => '+',
                    (false, true) => '-',
                    (true, false) => '|',
                    // the box is opaque: it hides the tree behind it
                    (false, false) => {
                        c.set(x, y, ' ', Color::Reset);
                        continue;
                    }
                };
                c.set_attr(x, y, ch, border, BOLD);
            }
        }
        let cw = m.w - 4;
        for (dy, line) in m.lines.iter().enumerate() {
            let mut x = m.x + 2;
            for &(ch, w) in line {
                if w > 0 {
                    c.set(x, m.y + 1 + dy as i32, ch, self.msg_fg);
                }
                x += w;
            }
        }
        if m.clipped {
            let marker = if cw >= 3 { "..." } else { "." };
            let y = m.y + m.lines.len() as i32;
            c.text(m.x + 2 + cw - marker.len() as i32, y, marker, self.msg_fg);
        }
    }

    // -- Día de Muertos --

    /// The pot's left and right columns and the ground row, where the ofrenda
    /// stands (without a pot, where the first pot would be, so the lowest
    /// branches have room).
    fn foot(&self) -> (i32, i32, i32) {
        match base_size(self.base) {
            (0, _) => {
                let half = base_size(1).0 / 2;
                (self.base_x - half, self.base_x + half, self.base_y - 1)
            }
            (bw, bh) => (self.base_x, self.base_x + bw - 1, self.base_y + bh - 1),
        }
    }

    /// What a leaf is today: its glyph and color.
    fn blossom(&self, x: i32, y: i32, cell: GCell) -> (char, Color) {
        let glyph = |g: char| if self.bloom_glyphs { g } else { cell.ch };
        let d = &self.dress;
        match bloom(x, y, self.seed, self.fiesta.level) {
            Bloom::Leaf => (cell.ch, self.pairs[(cell.style & 7) as usize]),
            Bloom::Marigold(s) => (glyph('@'), d.marigold[s.min(2) as usize]),
            Bloom::Velvet(s) => (cell.ch, d.velvet[s.min(1) as usize]),
            Bloom::Nube => (glyph('*'), d.nube),
        }
    }

    /// The color of a sprite's ink letter.
    fn ink(&self, k: u8, x: i32, glass: Color) -> Color {
        let d = &self.dress;
        match k {
            b'b' => d.cobalt[0],
            b'B' => d.cobalt[1],
            b'w' => d.glaze,
            b'y' => d.yellow,
            // the petals strewn on the soil
            b'p' => d.marigold[((hash01(x, 0, self.seed) * 3.0) as usize).min(2)],
            b'W' => self.pairs[WOOD_BRIGHT as usize],
            b'i' => d.icing[0],
            b'j' => d.icing[1],
            b'k' => d.icing[2],
            b'l' => d.icing[3],
            b'g' => glass,
            b'x' => d.wax,
            b'd' => d.crust,
            b's' => d.sugar,
            _ => d.bone,
        }
    }

    /// Candle `i`'s flame as it flickers: its tip, or its heart.
    fn flame(&self, i: i32, tip: bool) -> (char, Color) {
        let f = (self.clock * 9.0) as i32;
        let h = hash01(f, i, 0x666c_616d);
        if tip {
            let ch = ['(', ')', '\'', '`', ')', '.'][((h * 6.0) as usize).min(5)];
            (ch, FLAMA_EDGE)
        } else {
            let ch = if h < 0.5 { ')' } else { '(' };
            (ch, FLAMA.lerp(FLAMA_EDGE, 0.4 * hash01(f, i, 0x676c_6f77)))
        }
    }

    /// Draws a sprite at column `x` standing on row `ground`; its flame cells
    /// are candle `candle`'s.
    #[allow(clippy::too_many_arguments)]
    fn sprite(
        &self,
        c: &mut Canvas,
        x: i32,
        ground: i32,
        s: &Sprite,
        glass: Color,
        candle: i32,
        attr: u8,
    ) {
        let top = ground + 1 - s.art.len() as i32;
        for (dy, (art, ink)) in s.art.iter().zip(s.ink).enumerate() {
            let y = top + dy as i32;
            for (dx, (ch, k)) in art.chars().zip(ink.bytes()).enumerate() {
                let x = x + dx as i32;
                if ch == ' ' || k == b' ' {
                    continue;
                }
                if k == b'f' || k == b't' {
                    let (ch, fg) = self.flame(candle, k == b't');
                    c.set_attr(x, y, ch, fg, BOLD);
                } else {
                    let a = if k == b'B' { BOLD } else { attr };
                    c.set_attr(x, y, ch, self.ink(k, x, glass), a);
                }
            }
        }
    }

    /// The talavera pot in place of cbonsai's.
    fn draw_talavera(&self, c: &mut Canvas) {
        let (pot, bold) = match self.base {
            1 => (&TALAVERA_ONE, true),
            2 => (&TALAVERA_TWO, false),
            _ => return,
        };
        let ground = self.base_y + pot.art.len() as i32 - 1;
        let attr = if bold { BOLD } else { 0 };
        self.sprite(c, self.base_x, ground, pot, Color::Reset, 0, attr);
    }

    /// The ofrenda at the foot of the tree, and the petals.
    fn draw_ofrenda(&self, ctx: &Ctx, c: &mut Canvas) {
        let (l, r, ground) = self.foot();
        self.lying_petals(ctx, c);
        let skull = if l - 6 - 10 >= 0 || self.w >= 100 {
            &CALAVERA
        } else {
            &CALAVERITA
        };
        let sw = skull.art[0].len() as i32;
        self.sprite(c, l - 6 - sw + 1, ground, skull, Color::Reset, 0, 0);
        let level = self.fiesta.level;
        for (i, &(from, right, off, s)) in OFRENDA.iter().enumerate() {
            if from > level {
                continue;
            }
            let w = s.art[0].len() as i32;
            let x = if right { r + off } else { l - off - w + 1 };
            let glass = self.dress.glass[i % 3];
            self.sprite(c, x, ground, s, glass, i as i32, 0);
        }
        self.flying_petals(c);
    }

    /// Petals lying on the ground, under everything else.
    fn lying_petals(&self, ctx: &Ctx, c: &mut Canvas) {
        for p in self.petals.iter().filter(|p| p.rest >= 0.0) {
            let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
            if c.get(x, y).is_some_and(|cell| cell.ch == ' ') {
                let fg = ctx.fade(p.color, (1.0 - p.rest / 1.5).clamp(0.0, 1.0));
                c.set(x, y, p.lie, fg);
            }
        }
    }

    /// Petals in the air, in front of it all.
    fn flying_petals(&self, c: &mut Canvas) {
        for p in self.petals.iter().filter(|p| p.rest < 0.0) {
            let k = (p.phase * 1.6) as i32;
            let ch = ['\'', '`', '.', ','][k.rem_euclid(4) as usize];
            let fg = if k & 1 == 0 { p.color } else { p.shade };
            c.set_attr(p.x.floor() as i32, p.y.floor() as i32, ch, fg, BOLD);
        }
    }

    /// Petals fall from the blossoms, sway, and lie a while where they land.
    fn tick_fiesta(&mut self, dt: f32, real: f32) {
        let dt = dt.clamp(0.0, 1.0);
        let real = real.clamp(0.0, 60.0);
        self.clock = (self.clock + dt).rem_euclid(3600.0);
        let (l, r, ground) = self.foot();
        let pot = base_size(self.base).0 > 0;
        let mut i = 0;
        while i < self.petals.len() {
            let p = &mut self.petals[i];
            let alive = if p.rest >= 0.0 {
                p.rest -= real;
                p.rest > 0.0
            } else {
                p.phase += dt * 2.4;
                // a breath of wind
                p.x0 += dt * 0.3;
                p.y += p.vy * dt;
                p.x = p.x0 + p.phase.sin() * 1.2;
                let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
                if pot && (l..=r).contains(&x) && y >= self.base_y {
                    // into the petals strewn on the soil
                    false
                } else if y >= ground {
                    p.y = ground as f32;
                    p.rest = 5.0 + 9.0 * self.frng.f32();
                    p.lie = ['.', ',', '_'][self.frng.usize(..3)];
                    true
                } else {
                    x >= 0 && x < self.w
                }
            };
            if alive {
                i += 1;
            } else {
                self.petals.swap_remove(i);
            }
        }
        self.spawn += dt
            * match self.look {
                Look::Patrias => 0.5 + 2.0 * self.fiesta.level,
                _ => 0.8 + 3.2 * self.fiesta.level,
            };
        while self.spawn >= 1.0 {
            self.spawn -= 1.0;
            if self.petals.len() < PETALS {
                self.drop_petal();
            }
        }
    }

    /// A petal lets go of a blossom somewhere on the tree, if there is one.
    fn drop_petal(&mut self) {
        let Some((x0, y0, x1, y1)) = self.grid.bbox else {
            return;
        };
        for _ in 0..24 {
            let (x, y) = (self.frng.i32(x0..=x1), self.frng.i32(y0..=y1));
            let cell = self.grid.cells[(y * self.grid.w + x) as usize];
            let leaf = matches!(cell.style & 7, LEAF_DARK | LEAF_BRIGHT);
            if cell.ch <= ' ' || cell.style & WIDE_TAIL != 0 || !leaf {
                continue;
            }
            let d = &self.dress;
            let (color, shade) = match self.look {
                Look::Patrias => match tricolor(x, y, self.mid(), self.seed, self.fiesta.level) {
                    Tri::Blanco(_) => (self.patrias.blanco[0], self.patrias.blanco[1]),
                    Tri::Rojo(_) => (self.patrias.rojo[0], self.patrias.rojo[1]),
                    Tri::Leaf => continue,
                },
                _ => match bloom(x, y, self.seed, self.fiesta.level) {
                    Bloom::Marigold(_) => (d.marigold[2], d.marigold[1]),
                    Bloom::Velvet(_) => (d.velvet[1], d.velvet[0]),
                    _ => continue,
                },
            };
            let phase = self.frng.f32() * std::f32::consts::TAU;
            let px = x as f32 + 0.5;
            self.petals.push(Petal {
                x: px,
                y: y as f32 + 0.5,
                x0: px - phase.sin() * 1.2,
                vy: 1.3 + 1.5 * self.frng.f32(),
                phase,
                rest: -1.0,
                lie: '.',
                color,
                shade,
            });
            return;
        }
    }
}

// -- Navidad and the Fiestas Patrias --

/// Where the pesebre stands: its left column, the side of the pot (1 right,
/// -1 left), and whether it is the full one or the narrow one.
struct Belen {
    x: i32,
    side: i32,
    full: bool,
}

impl Belen {
    fn width(&self) -> i32 {
        if self.full { PORTAL_W } else { COMPACT_W }
    }
}

impl Bonsai {
    /// The pot's middle column.
    fn mid(&self) -> i32 {
        let (l, r, _) = self.foot();
        (l + r) / 2
    }

    /// The color of a sprite's ink letter for the holiday on.
    fn ink_of(&self, k: u8) -> Color {
        match self.look {
            Look::Navidad => self.navidad.inks.get(k),
            Look::Patrias => self.patrias.inks.get(k),
            _ => Color::Reset,
        }
    }

    /// Draws a sprite with its left column at `at.0`, standing on row `at.1`,
    /// faded `fade` toward the background.
    fn plot(&self, ctx: &Ctx, c: &mut Canvas, at: (i32, i32), s: &Sprite, attr: u8, fade: f32) {
        let top = at.1 + 1 - s.art.len() as i32;
        for (dy, (art, ink)) in s.art.iter().zip(s.ink).enumerate() {
            let y = top + dy as i32;
            for (dx, (ch, k)) in art.chars().zip(ink.bytes()).enumerate() {
                let x = at.0 + dx as i32;
                match k {
                    b' ' => {}
                    b'.' => c.set(x, y, ' ', Color::Reset),
                    _ if ch == ' ' => {}
                    _ => {
                        let fg = self.ink_of(k);
                        let fg = if fade > 0.0 { ctx.fade(fg, fade) } else { fg };
                        c.set_attr(x, y, ch, fg, attr);
                    }
                }
            }
        }
    }

    /// Room for a piece `width` wide beside the pot, on the side `prefer`
    /// (1 right, -1 left) if it fits, else on the other: its left column and
    /// the side.
    fn room(&self, width: i32, prefer: i32) -> Option<(i32, i32)> {
        let (l, r, _) = self.foot();
        let fits = |side: i32| {
            let x = if side > 0 { r + 3 } else { l - 3 - width + 1 };
            (x >= 0 && x + width <= self.w).then_some((x, side))
        };
        fits(prefer).or_else(|| fits(-prefer))
    }

    /// The pot of the holiday in place of cbonsai's.
    fn draw_pot(&self, ctx: &Ctx, c: &mut Canvas) {
        let (one, two) = match self.look {
            Look::Navidad => (&NOCHEBUENA_ONE, &NOCHEBUENA_TWO),
            Look::Patrias => (&PATRIA_ONE, &PATRIA_TWO),
            _ => return,
        };
        let (pot, bold) = match self.base {
            1 => (one, true),
            2 => (two, false),
            _ => return,
        };
        let ground = self.base_y + pot.art.len() as i32 - 1;
        let attr = if bold { BOLD } else { 0 };
        self.plot(ctx, c, (self.base_x, ground), pot, attr, 0.0);
    }

    /// What a leaf or a bough wears on Navidad: its glyph, color and weight.
    fn trim(&self, ctx: &Ctx, x: i32, y: i32, cell: GCell, attr: u8) -> (char, Color, u8) {
        let pair = cell.style & 7;
        let wood = matches!(pair, WOOD_DARK | WOOD_BRIGHT);
        let g = &self.grid;
        let hangs = !wood && y + 1 < g.h && g.cells[((y + 1) * g.w + x) as usize].ch <= ' ';
        let n = &self.navidad;
        match adorno(x, y, self.seed, built(self.fiesta), wood, hangs) {
            Adorno::None => (cell.ch, self.pairs[pair as usize], attr),
            Adorno::Esfera(k) => {
                let ch = if self.bloom_glyphs { '@' } else { cell.ch };
                (ch, n.esferas[k as usize], BOLD)
            }
            Adorno::Foquito(i) => {
                let color = n.luces[i.rem_euclid(6) as usize];
                let glyph = |g| {
                    if wood || self.bloom_glyphs {
                        g
                    } else {
                        cell.ch
                    }
                };
                if self.lit(i) {
                    (glyph('O'), color, BOLD)
                } else {
                    (glyph('o'), ctx.fade(color, 0.7), 0)
                }
            }
        }
    }

    /// What a leaf is on the Fiestas Patrias: its glyph and color.
    fn patriotic(&self, x: i32, y: i32, mid: i32, cell: GCell) -> (char, Color) {
        let p = &self.patrias;
        let glyph = |g: char| if self.bloom_glyphs { g } else { cell.ch };
        match tricolor(x, y, mid, self.seed, self.fiesta.level) {
            Tri::Leaf => (cell.ch, p.verde[(cell.style & 7 == LEAF_BRIGHT) as usize]),
            Tri::Blanco(s) => (glyph('*'), p.blanco[s as usize]),
            Tri::Rojo(s) => (glyph('@'), p.rojo[s as usize]),
        }
    }

    /// The step of the series' blinking the scene is at.
    fn step(&self) -> i32 {
        (self.clock / BLINK) as i32
    }

    /// Is bulb `i` of the series lit? Every few seconds the series changes
    /// its pattern: steady with a few twinkling, a chase, bulbs taking turns,
    /// all of them blinking together.
    fn lit(&self, i: i32) -> bool {
        let step = self.step();
        match (step / 16) % 4 {
            0 => hash01(i, step, 0x7477_696e) > 0.12,
            1 => (i + step).rem_euclid(3) != 0,
            2 => (i + step / 2).rem_euclid(2) == 0,
            _ => (step / 3) % 2 == 0,
        }
    }

    /// The musgo the nacimiento stands on: moss along the ground, in tufts a
    /// row above it, thinning out at both ends.
    fn draw_musgo(&self, c: &mut Canvas) {
        let (l, r, ground) = self.foot();
        let belen = self.belen();
        let (side, w) = belen
            .as_ref()
            .map_or((1, PORTAL_W), |b| (b.side, b.width()));
        let (a, b) = if side > 0 {
            (l - 3 - PASTORES_W, r + 3 + w)
        } else {
            (l - 4 - w, r + 3 + PASTORES_W)
        };
        let inks = &self.navidad.inks;
        let portal = belen.map(|b| b.x..b.x + w);
        for x in a.max(0)..=b.min(self.w - 1) {
            let edge = (((x - a).min(b - x) + 1) as f32 / 4.0).min(1.0);
            if hash01(x, ground, self.seed ^ 0x6d75_7367) >= edge {
                continue;
            }
            let ink = if hash01(x, 1, self.seed ^ 0x6d6f_7373) < 0.5 {
                b'm'
            } else {
                b'l'
            };
            let h = hash01(x, 2, self.seed ^ 0x6d6f_7374);
            let ch = ['"', '\'', ',', ';', '.', '"'][((h * 6.0) as usize).min(5)];
            c.set(x, ground, ch, inks.get(ink));
            let inside = portal.as_ref().is_some_and(|r| r.contains(&x));
            if !inside && hash01(x, ground - 1, self.seed ^ 0x7475_6674) < 0.1 * edge {
                c.set(x, ground - 1, '\'', inks.get(b'l'));
            }
        }
    }

    /// Where the pesebre stands, the right of the pot when it fits, and
    /// which of the two sizes.
    fn belen(&self) -> Option<Belen> {
        [true, false].into_iter().find_map(|full| {
            let w = if full { PORTAL_W } else { COMPACT_W };
            let (x, side) = self.room(w, 1)?;
            Some(Belen { x, side, full })
        })
    }

    /// Heno draped from the boughs, in strands one to three cells long.
    fn draw_heno(&self, c: &mut Canvas) {
        let g = &self.grid;
        let Some((x0, y0, x1, y1)) = g.bbox else {
            return;
        };
        let p = 0.1 + 0.2 * built(self.fiesta);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let cell = g.cells[(y * g.w + x) as usize];
                let wood = matches!(cell.style & 7, WOOD_DARK | WOOD_BRIGHT);
                if !wood || cell.ch <= ' ' || hash01(x, y, self.seed ^ 0x6865_6e6f) >= p {
                    continue;
                }
                let len = 1 + (hash01(x, y, self.seed ^ 0x6c65_6e67) * 3.0) as i32;
                let sway = hash01(x, y, self.seed ^ 0x7377_6179);
                for k in 1..=len.min(3) {
                    let ch = [';', ':', '\''][(k - 1) as usize];
                    let hx = x + (sway < 0.5 && k == 2) as i32;
                    if c.get(hx, y + k).is_none_or(|cell| cell.ch != ' ') {
                        break;
                    }
                    c.set(hx, y + k, ch, self.navidad.heno);
                }
            }
        }
    }

    /// The star on the crown, once the tree has grown.
    fn draw_star(&self, c: &mut Canvas) {
        let g = &self.grid;
        let (Phase::Done, Some((x0, y0, x1, _))) = (self.phase, g.bbox) else {
            return;
        };
        // the crown: the middle of the topmost row
        let top = &g.cells[(y0 * g.w) as usize..((y0 + 1) * g.w) as usize];
        let mut xs = (x0..=x1).filter(|&x| top[x as usize].ch > ' ');
        let (first, last) = match xs.next() {
            Some(first) => (first, xs.next_back().unwrap_or(first)),
            None => (x0, x1),
        };
        let (x, y) = ((first + last) / 2, y0 - 2);
        let k = self.navidad.star;
        let (ray, diag) = (self.step() & 1 == 0, self.step() & 1 == 1);
        for (dx, dy, ch, on) in [
            (0, -1, '|', ray),
            (0, 1, '|', ray),
            (-1, 0, '-', ray),
            (1, 0, '-', ray),
            (-1, -1, '\\', diag),
            (1, 1, '\\', diag),
            (1, -1, '/', diag),
            (-1, 1, '/', diag),
        ] {
            if on {
                c.set_attr(x + dx, y + dy, ch, k, 0);
            }
        }
        c.set_attr(x, y, '*', k, BOLD);
    }

    /// The nacimiento: the pesebre with its figures, the pastores and their
    /// sheep across the pot, the angel and the Niño Dios from Nochebuena, the
    /// Reyes Magos on their way in January.
    fn draw_belen(&self, ctx: &Ctx, c: &mut Canvas) {
        self.draw_heno(c);
        self.draw_star(c);
        let (l, _, ground) = self.foot();
        let built = built(self.fiesta);
        let nino = nino_dios(self.fiesta);
        let portal = self.belen();
        // the pastores, on the other side of the pot
        let side = -portal.as_ref().map_or(1, |b| b.side);
        for (i, &(from, off, s)) in PASTORES.iter().enumerate() {
            let w = s.art[0].chars().count() as i32;
            let x = if side < 0 {
                l - 3 - off - w + 1
            } else {
                self.foot().1 + 2 + off
            };
            if from <= built && x >= 0 && x + w <= self.w {
                self.plot(ctx, c, (x, ground), s, 0, 0.0);
                if nino && i == 2 {
                    // the angel announces it over the flock
                    self.plot(ctx, c, (x + 1, ground - 3), &ANGEL, 0, 0.0);
                }
            }
        }
        let Some(belen) = portal else {
            return;
        };
        let px = belen.x;
        let (roof, figures, cuna): (_, &[(f32, i32, &Sprite)], _) = if belen.full {
            (&PORTAL, &NACIMIENTO, 11)
        } else {
            (&PORTAL_COMPACT, &NACIMIENTO_COMPACT, 7)
        };
        self.plot(ctx, c, (px, ground), roof, 0, 0.0);
        self.plot(ctx, c, (px + cuna, ground), &CUNA, 0, 0.0);
        for &(from, dx, s) in figures {
            if from <= built {
                self.plot(ctx, c, (px + dx, ground), s, 0, 0.0);
            }
        }
        if nino {
            // rays around the Niño, twinkling
            self.plot(ctx, c, (px + cuna + 1, ground - 1), &NINO, 0, 0.0);
            if self.step() & 1 == 1 {
                c.set_attr(px + cuna + 1, ground - 2, '.', self.navidad.star, 0);
                c.set_attr(px + cuna + 3, ground - 2, '.', self.navidad.star, 0);
            }
        }
        self.draw_reyes(ctx, c, &belen);
    }

    /// The Reyes Magos, in single file from the edge of the screen toward the
    /// pesebre: dim and far on January 2, in front of it on the 6th.
    fn draw_reyes(&self, ctx: &Ctx, c: &mut Canvas, belen: &Belen) {
        let Some(away) = reyes_away(self.fiesta) else {
            return;
        };
        let (_, _, ground) = self.foot();
        let side = belen.side;
        let (here, far) = if side > 0 {
            ((belen.x + belen.width() + 1).min(self.w - 12), self.w - 4)
        } else {
            ((belen.x - 12).max(1), -7)
        };
        let x0 = here + ((far - here) as f32 * away).round() as i32;
        let fade = 0.5 * away;
        for (i, king) in [&MELCHOR, &GASPAR, &BALTASAR].into_iter().enumerate() {
            let slot = if side > 0 { i } else { 2 - i } as i32;
            self.plot(ctx, c, (x0 + 4 * slot, ground), king, 0, fade);
        }
        // the star they follow
        let k = ctx.fade(self.navidad.star, fade);
        let star = if self.step() & 1 == 0 { '*' } else { '+' };
        c.set_attr(x0 + 5, ground - 5, star, k, BOLD);
    }

    /// The emblem at the foot of the tree: an eagle on a nopal growing from a
    /// rock in the lake. The nopal gains a pad as the month climbs and ripens
    /// its tunas, the eagle perches on it from September 11, and on the 15th
    /// and 16th it holds the serpent in its beak.
    fn draw_emblema(&self, ctx: &Ctx, c: &mut Canvas) {
        let Some((ex, _)) = self.room(EMBLEMA_W, -1) else {
            return;
        };
        let (_, _, ground) = self.foot();
        let level = self.fiesta.level;
        self.plot(ctx, c, (ex, ground), &ROCA, 0, 0.0);
        let pads = if level < 0.3 {
            1
        } else if level < 0.45 {
            2
        } else {
            3
        };
        let mut top = ex + 8;
        for (i, dx) in [8, 10, 8].into_iter().take(pads).enumerate() {
            let (x, y) = (ex + dx, ground - 2 - i as i32);
            self.plot(ctx, c, (x, y), &PENCA, 0, 0.0);
            if level >= 0.4 {
                // a tuna ripening on the side
                let tx = if i % 2 == 0 { x + 3 } else { x - 1 };
                c.set_attr(tx, y, 'o', self.patrias.inks.get(b'x'), BOLD);
            }
            top = x;
        }
        if level >= 0.5 {
            let feet = ground - 2 - pads as i32;
            self.plot(ctx, c, (top - 8, feet), &AGUILA_SPRITE, 0, 0.0);
            if self.fiesta.date() >= (9, 15) {
                self.plot(ctx, c, (top - 8, feet), &SERPIENTE, 0, 0.0);
            }
        }
    }

    /// Banderitas planted in the soil of the pot, waving; one at first, and
    /// more as the month climbs.
    fn draw_banderitas(&self, c: &mut Canvas) {
        let bw = base_size(self.base).0;
        if bw == 0 {
            return;
        }
        let level = self.fiesta.level;
        let step = (self.clock * 2.0) as i32;
        let pole = self.pairs[WOOD_BRIGHT as usize];
        // from what level, the pot's column, which way it flies, how tall
        for (from, col, dir, tall) in [
            (0.0, 3, 1, 4),
            (0.5, 27, -1, 4),
            (0.8, 8, 1, 3),
            (0.95, 22, -1, 3),
        ] {
            if level < from {
                continue;
            }
            let x = self.base_x + col * (bw - 1) / 30;
            for dy in 1..=tall {
                c.set(x, self.base_y - dy, '|', pole);
            }
            for k in 0..3 {
                // the green at the pole, the red at the fly
                let flag = self.patrias.flag[if dir > 0 { k } else { 2 - k } as usize];
                let (top, bottom) = if (step + k).rem_euclid(2) == 0 {
                    ('█', '▀')
                } else {
                    ('▄', '█')
                };
                let fx = x + dir * (k + 1);
                c.set(fx, self.base_y - tall, top, flag);
                c.set(fx, self.base_y - tall + 1, bottom, flag);
            }
        }
    }
}

impl Scene for Bonsai {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        let (w, h) = (w as i32, h as i32);
        if self.grown && (w, h) == (self.w, self.h) {
            return;
        }
        self.w = w;
        self.h = h;
        let (bw, bh) = base_size(self.base);
        (self.base_y, self.base_x) = self.geometry.unwrap_or((h - bh, w / 2 - bw / 2));
        self.grid.reset(w, self.base_y, h);
        self.msg = message_box(&self.message, w, h);
        if !self.grown {
            self.start_tree();
            return;
        }
        // Like cbonsai's redrawCurrentTree: the same tree, regrown for the new
        // size and replayed up to where it was. A finished tree restarts its wait.
        let shown = self.shown;
        self.grow();
        self.shown = 0;
        self.reveal(shown);
        self.hold = 0.0;
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let mut pairs = if ctx.theme.native {
            CLASSIC
        } else {
            theme_pairs(ctx)
        };
        if let Some(u) = self.user_colors {
            pairs[..4].copy_from_slice(&u);
        }
        self.pairs = pairs;
        // cbonsai prints the message in the terminal's default color
        self.msg_fg = if ctx.theme.native {
            Color::Reset
        } else {
            ctx.theme.fg
        };
        self.fiesta = ctx.fiesta;
        self.dress = if ctx.theme.native {
            Dress::CLASSIC
        } else {
            Dress::themed(ctx, self.pairs[TEXT as usize])
        };
        let look = Look::of(ctx.fiesta);
        match look {
            Look::Navidad => {
                self.navidad = if ctx.theme.native {
                    Navidad::classic(&self.pairs)
                } else {
                    Navidad::themed(ctx, &self.pairs)
                };
            }
            Look::Patrias => {
                self.patrias = if ctx.theme.native {
                    Patrias::classic(&self.pairs, self.user_colors.is_some())
                } else {
                    Patrias::themed(ctx, &self.pairs)
                };
            }
            _ => {}
        }
        // petals belong to the holiday that dropped them
        if look != self.look || !matches!(look, Look::Muertos | Look::Patrias) {
            self.petals.clear();
        }
        self.look = look;
    }

    fn update(&mut self, ctx: &Ctx, dt: f32) {
        self.tick(ctx, dt, dt);
    }

    fn tick(&mut self, _ctx: &Ctx, dt: f32, real: f32) {
        match self.look {
            Look::Muertos | Look::Patrias => self.tick_fiesta(dt, real),
            // the series blinks on the wall clock, however fast the scene runs
            Look::Navidad => self.clock = (self.clock + real.clamp(0.0, 1.0)).rem_euclid(3600.0),
            Look::Plain => {}
        }
        let mut t = (dt.max(0.0) as f64).min(86_400.0);
        let mut r = (real.max(0.0) as f64).min(86_400.0);
        if t > 0.0 && r > 0.0 {
            self.pace = t / r;
        }
        // A huge dt may finish a tree and start the next; a few phases per call is plenty.
        for _ in 0..6 {
            match self.phase {
                Phase::Growing => {
                    self.acc += t;
                    let left = self.strokes.len() - self.shown;
                    let n = ((self.acc / self.step) as usize).min(left);
                    self.reveal(n);
                    self.acc -= n as f64 * self.step;
                    if self.shown < self.strokes.len() {
                        return;
                    }
                    // What is left of this tick, on the wall clock, starts the wait.
                    r = self.acc.max(0.0) / self.pace;
                    self.acc = 0.0;
                    self.hold = 0.0;
                    self.phase = Phase::Done;
                }
                Phase::Done => {
                    if !self.infinite {
                        return;
                    }
                    self.hold += r;
                    if self.hold < self.wait {
                        return;
                    }
                    t = (self.hold - self.wait) * self.pace;
                    self.seed = self.seeder.u32(..);
                    self.start_tree();
                }
            }
        }
    }

    fn next_change(&self) -> f32 {
        if matches!(self.look, Look::Muertos | Look::Patrias) {
            // candles flicker and petals fall
            return 0.0;
        }
        let own = match self.phase {
            Phase::Growing => (self.step - self.acc).max(0.0) as f32,
            // The wait is in real seconds; this is asked in scene seconds.
            Phase::Done if self.infinite => ((self.wait - self.hold).max(0.0) * self.pace) as f32,
            Phase::Done => f32::INFINITY,
        };
        if self.look == Look::Navidad {
            // the series blinks a step at a time, in real seconds; this is
            // asked in scene seconds
            let step = (BLINK - self.clock.rem_euclid(BLINK)) as f64 * self.pace;
            return own.min(step as f32);
        }
        own
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        match self.look {
            Look::Muertos => self.draw_talavera(c),
            Look::Navidad => {
                self.draw_musgo(c);
                self.draw_pot(ctx, c);
            }
            Look::Patrias => self.draw_pot(ctx, c),
            Look::Plain => self.draw_base(c),
        }
        let mid = self.mid();
        let g = &self.grid;
        if let Some((x0, y0, x1, y1)) = g.bbox {
            for y in y0..=y1 {
                let row = &g.cells[(y * g.w) as usize..((y + 1) * g.w) as usize];
                for x in x0..=x1 {
                    let cell = row[x as usize];
                    if cell.ch > ' ' && cell.style & WIDE_TAIL == 0 {
                        let attr = if cell.style & BOLD_BIT != 0 { BOLD } else { 0 };
                        let pair = cell.style & 7;
                        let leaf = matches!(pair, LEAF_DARK | LEAF_BRIGHT);
                        let (ch, fg, attr) = match self.look {
                            Look::Muertos if leaf => {
                                let (ch, fg) = self.blossom(x, y, cell);
                                (ch, fg, attr)
                            }
                            Look::Navidad if leaf || matches!(pair, WOOD_DARK | WOOD_BRIGHT) => {
                                self.trim(ctx, x, y, cell, attr)
                            }
                            Look::Patrias if leaf => {
                                let (ch, fg) = self.patriotic(x, y, mid, cell);
                                (ch, fg, attr)
                            }
                            _ => (cell.ch, self.pairs[pair as usize], attr),
                        };
                        c.set_attr(x, y, ch, fg, attr);
                    }
                }
            }
        }
        match self.look {
            Look::Muertos => self.draw_ofrenda(ctx, c),
            Look::Navidad => self.draw_belen(ctx, c),
            Look::Patrias => {
                self.lying_petals(ctx, c);
                self.draw_emblema(ctx, c);
                self.draw_banderitas(c);
                self.flying_petals(c);
            }
            Look::Plain => {}
        }
        self.draw_message(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::Theme;

    fn scene(opts: &str, w: u16, h: u16) -> (Bonsai, Canvas) {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, opts.parse().unwrap());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut b = Bonsai::new(&ctx);
        b.resize(&ctx, w, h);
        b.retheme(&ctx);
        b.update(&ctx, 1e6);
        let mut c = Canvas::new(w, h, Color::Reset);
        b.draw(&ctx, &mut c);
        (b, c)
    }

    #[test]
    fn glibc_rand() {
        let mut r = CRand::new(42);
        let got: Vec<i32> = (0..6).map(|_| r.next()).collect();
        assert_eq!(
            got,
            [
                71876166, 708592740, 1483128881, 907283241, 442951012, 537146758
            ]
        );
        let mut r = CRand::new(0);
        assert_eq!(r.next(), 1804289383);
        let mut r = CRand::new(3_000_000_000);
        assert_eq!(r.next(), 2058147116);
    }

    /// `cbonsai -s 42` in a 60x24 terminal, captured from cbonsai 1.3.1.
    const CBONSAI_42: &str = "
                           &&&&&&&&
                        &&&&&&&&&&&&
                      &&&&&&&&&&&&&&&
                     &&&&&&&&&&&&&&&&
                    &&&&&&&&&&&&\\&&& &
                    &&&&&&|&&&&||/&&
                   &&&&&&&&&&&&_/&&
             &     &&& &\\&/~&&&&/~
            &&&  &  &   //&\\  &/|\\~/
           &&&&&&\\&&_\\_ \\|/    \\//~                  &
             &&&_\\ &  \\_/~~\\    /~|               &&&&&&
           &&&&        ///~       /|\\           &  &&&&&&&
             &          /~|       \\|    /___/     /_&_&&
                        /|/~     \\|/~/_/     / _/_/  &&&&
                        /~        |/~                  &
                         /~|   |//~
                           /~|~
                             /~
               :___________./~~~\\.___________:
                \\                           /
                 \\_________________________/
                 (_)                     (_)
";

    #[test]
    fn same_tree_as_cbonsai() {
        let (_, c) = scene("seed = 42\nlive = false\ninfinite = false", 60, 24);
        let text = c.to_text();
        assert_eq!(text.trim_matches('\n'), CBONSAI_42.trim_matches('\n'));
    }

    #[test]
    fn live_mode_reveals_and_infinite_mode_regrows() {
        let (mut b, _) = scene("seed = 42\ninfinite = false", 60, 24);
        let n = b.strokes.len();
        assert!(n > 100 && b.shown == n && b.phase == Phase::Done);
        let theme = Theme::classic();
        let opts = Opts::new(
            INFO.options,
            "seed = 42\nstep = 0.5\nwait = 1.0".parse().unwrap(),
        );
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        b = Bonsai::new(&ctx);
        b.resize(&ctx, 60, 24);
        b.update(&ctx, 1.1);
        assert_eq!(b.shown, 2);
        b.update(&ctx, 0.5 * (n - 2) as f32);
        assert_eq!((b.phase, b.shown), (Phase::Done, n));
        b.update(&ctx, 0.95);
        assert_eq!((b.phase, b.shown), (Phase::Growing, 0));
        assert_ne!(b.seed, 42);
    }

    #[test]
    fn the_wait_between_trees_is_real_time() {
        let theme = Theme::classic();
        let opts = Opts::new(
            INFO.options,
            "seed = 42\nstep = 0.5\nwait = 2.0".parse().unwrap(),
        );
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        let mut b = Bonsai::new(&ctx);
        b.resize(&ctx, 60, 24);
        let n = b.strokes.len();
        // At a tenth of the speed the tree takes ten times as long to grow...
        let grow = 0.5 * n as f32 + 0.001;
        b.tick(&ctx, grow, grow * 10.0);
        assert_eq!((b.phase, b.shown), (Phase::Done, n));
        // ...but the finished tree waits its two real seconds, not twenty.
        b.tick(&ctx, 0.19, 1.9);
        assert_eq!(b.phase, Phase::Done);
        assert!(b.next_change() < 0.02, "{}", b.next_change());
        b.tick(&ctx, 0.02, 0.2);
        assert_eq!(b.phase, Phase::Growing);
    }

    #[test]
    fn resize_keeps_the_tree() {
        let (mut b, c1) = scene("seed = 42\nlive = false\ninfinite = false", 60, 24);
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        b.resize(&ctx, 80, 30);
        b.resize(&ctx, 60, 24);
        let mut c2 = Canvas::new(60, 24, Color::Reset);
        b.draw(&ctx, &mut c2);
        assert_eq!(c1.to_text(), c2.to_text());
    }

    #[test]
    fn message_wraps_like_cbonsai() {
        let (lines, w) = layout_message("hello   world, this is\n\nlong-wordy-word", 8);
        let s: Vec<String> = lines
            .iter()
            .map(|l| l.iter().map(|c| c.0).collect())
            .collect();
        assert_eq!(s, ["hello", "world,", "this is", "", "long-wor", "dy-word"]);
        assert_eq!(w, 8);
        let (_, c) = scene("seed = 3\nmessage = 'hi there'\nlive = false", 60, 24);
        let t = c.to_text();
        assert!(t.contains("| hi there |"), "{t}");
    }

    /// A grown tree on a holiday (or on none), and what it draws.
    fn dressed(opts: &str, theme: &Theme, fiesta: Fiesta, w: u16, h: u16) -> (Bonsai, Canvas) {
        let opts = Opts::new(INFO.options, opts.parse().unwrap());
        let ctx = Ctx {
            theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta,
        };
        let mut b = Bonsai::new(&ctx);
        b.resize(&ctx, w, h);
        b.retheme(&ctx);
        for _ in 0..40 {
            b.tick(&ctx, 0.25, 0.25);
        }
        let mut c = Canvas::new(w, h, Color::Reset);
        b.draw(&ctx, &mut c);
        (b, c)
    }

    fn count(c: &Canvas, f: impl Fn(&crate::render::Cell) -> bool) -> usize {
        c.cells().iter().filter(|cell| f(cell)).count()
    }

    #[test]
    fn muertos_dresses_the_tree() {
        let theme = Theme::classic();
        let opts = "seed = 42\nlive = false\ninfinite = false";
        for (day, pieces) in [((10, 28), 2), ((11, 2), 6)] {
            let fiesta = Fiesta::of_day(day.0, day.1);
            let (b, c) = dressed(opts, &theme, fiesta, 100, 30);
            let text = c.to_text();
            // marigolds and terciopelo on the branches, among the leaves
            let marigolds = count(&c, |cell| {
                cell.ch == '@' && Dress::CLASSIC.marigold.contains(&cell.fg)
            });
            let velvet = count(&c, |cell| Dress::CLASSIC.velvet.contains(&cell.fg));
            let leaves = count(&c, |cell| cell.ch == '&' && CLASSIC[..3].contains(&cell.fg));
            assert!(
                marigolds > 10 && velvet > 5 && leaves > 5,
                "{day:?}\n{text}"
            );
            // the talavera pot, the calavera, the candles and the petals
            assert!(text.contains("~@~.~@~.~@~"), "{text}");
            assert!(text.contains("| (@) (@) |"), "{text}");
            // below the tree, which grows above the pot
            let ground: String = text.lines().rev().take(3).collect();
            let candles = ground.matches("|~|").count() + ground.matches("_|_").count();
            let pan = ground.contains("/=X=\\") as usize;
            assert_eq!(candles + pan, pieces, "{day:?}\n{text}");
            let flames = count(&c, |cell| cell.fg == FLAMA_EDGE);
            assert!(flames >= 2, "{text}");
            assert!(!b.petals.is_empty());
        }
    }

    #[test]
    fn ordinary_days_are_untouched() {
        let theme = Theme::classic();
        let opts = "seed = 42\nlive = false\ninfinite = false";
        let (b, c) = dressed(opts, &theme, Fiesta::NONE, 60, 24);
        assert_eq!(
            c.to_text().trim_matches('\n'),
            CBONSAI_42.trim_matches('\n')
        );
        assert!(b.petals.is_empty());
        assert_eq!(b.next_change(), f32::INFINITY);
        let rgb = count(&c, |cell| cell.fg.is_rgb());
        assert_eq!(rgb, 0);
    }

    #[test]
    fn the_holiday_comes_and_goes_through_retheme() {
        let theme = Theme::classic();
        let opts = Opts::new(
            INFO.options,
            "seed = 42\nlive = false\ninfinite = false".parse().unwrap(),
        );
        let ctx = |fiesta| Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta,
        };
        let plain = ctx(Fiesta::NONE);
        let mut b = Bonsai::new(&plain);
        b.resize(&plain, 60, 24);
        b.retheme(&plain);
        let draw = |b: &mut Bonsai, ctx: &Ctx| {
            let mut c = Canvas::new(60, 24, Color::Reset);
            b.draw(ctx, &mut c);
            c
        };
        let before = draw(&mut b, &plain).to_text();
        let muertos = ctx(Fiesta::of_day(11, 2));
        b.retheme(&muertos);
        for _ in 0..20 {
            b.tick(&muertos, 0.2, 0.2);
        }
        assert!(!b.petals.is_empty() && b.next_change() == 0.0);
        assert!(draw(&mut b, &muertos).to_text().contains("~@~."));
        b.retheme(&plain);
        assert!(b.petals.is_empty());
        assert_eq!(draw(&mut b, &plain).to_text(), before);
    }

    #[test]
    fn muertos_survives_odd_sizes_and_huge_steps() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        for theme in &themes {
            for opts in [
                "",
                "base = 2",
                "base = 0\nmessage = 'hola'",
                "geometry = '3,70'",
            ] {
                for (w, h) in [(1, 1), (2, 3), (7, 2), (40, 6), (300, 90)] {
                    let opts = Opts::new(INFO.options, opts.parse().unwrap());
                    let ctx = Ctx {
                        theme,
                        opts: &opts,
                        bg: rgb(0x0d1117),
                        seed: 5,
                        fiesta: Fiesta::of_day(11, 2),
                    };
                    let mut b = Bonsai::new(&ctx);
                    b.resize(&ctx, w, h);
                    b.retheme(&ctx);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for dt in [5.0, 0.01, 5.0, 1e6, 0.0, 5.0] {
                        b.tick(&ctx, dt, dt);
                        b.draw(&ctx, &mut c);
                    }
                    assert!(b.petals.len() <= PETALS);
                }
            }
        }
    }

    #[test]
    fn muertos_under_a_theme_uses_its_colors() {
        let theme = Theme::builtin("dracula").unwrap();
        let opts = "seed = 42\nlive = false\ninfinite = false";
        let (_, c) = dressed(opts, &theme, Fiesta::of_day(11, 2), 100, 30);
        // dracula's orange and pink for the marigolds and the terciopelo
        assert!(count(&c, |cell| cell.ch == '@' && cell.fg == rgb(0xffb86c)) > 10);
        assert!(count(&c, |cell| cell.fg == rgb(0xff79c6)) > 5);
    }

    const GROWN: &str = "seed = 42\nlive = false\ninfinite = false";

    #[test]
    fn the_new_sprites_are_well_formed() {
        for s in [
            &NOCHEBUENA_ONE,
            &NOCHEBUENA_TWO,
            &PATRIA_ONE,
            &PATRIA_TWO,
            &PORTAL,
            &PORTAL_COMPACT,
            &CUNA,
            &NINO,
            &OX,
            &MULA,
            &MARIA,
            &JOSE,
            &PASTOR_A,
            &PASTOR_B,
            &OVEJA,
            &ANGEL,
            &MELCHOR,
            &GASPAR,
            &BALTASAR,
            &ROCA,
            &PENCA,
            &AGUILA_SPRITE,
            &SERPIENTE,
        ] {
            assert_eq!(s.art.len(), s.ink.len(), "{:?}", s.art);
            for (art, ink) in s.art.iter().zip(s.ink) {
                assert_eq!(art.chars().count(), ink.len(), "{art:?} / {ink:?}");
            }
        }
        assert_eq!(PORTAL.art[0].len() as i32, PORTAL_W);
        assert_eq!(PORTAL_COMPACT.art[0].len() as i32, COMPACT_W);
        assert_eq!(ROCA.art[0].len() as i32, EMBLEMA_W);
    }

    /// The nacimiento's side of the screen: the bottom rows, without the
    /// columns of the pot and its tree.
    fn belen_text(c: &Canvas) -> String {
        let (w, h) = (c.w() as i32, c.h() as i32);
        let mid = w / 2;
        let mut text = String::new();
        for y in h - 6..h {
            for x in (0..w).filter(|&x| (x - mid).abs() > 17) {
                text.push(c.get(x, y).unwrap().ch);
            }
            text.push('\n');
        }
        text
    }

    #[test]
    fn navidad_dresses_the_tree() {
        let theme = Theme::classic();
        let n = Navidad::classic(&CLASSIC);
        let (b, c) = dressed(GROWN, &theme, Fiesta::of_day(12, 24), 100, 30);
        let text = c.to_text();
        // the foquitos along the branches, esferas and heno among the leaves
        let bulbs = count(&c, |cell| cell.ch == 'O' && FOQUITOS.contains(&cell.fg));
        let esferas = count(&c, |cell| cell.ch == '@' && n.esferas.contains(&cell.fg));
        let heno = count(&c, |cell| cell.fg == HENO && ";:'".contains(cell.ch));
        let leaves = count(&c, |cell| cell.ch == '&' && CLASSIC[..3].contains(&cell.fg));
        assert!(
            bulbs > 5 && esferas > 3 && heno > 3 && leaves > 20,
            "{bulbs} {esferas} {heno} {leaves}\n{text}"
        );
        // the star on the crown, the nochebuenas in the pot
        assert!(
            count(&c, |cell| cell.ch == '*' && cell.fg == DORADO) >= 1,
            "{text}"
        );
        assert!(text.contains("(*)v(*)v(*)"), "{text}");
        // the pesebre with everyone in it, the Niño Dios and the angel
        let belen = belen_text(&c);
        assert!(belen.contains("(..)"), "{belen}");
        for piece in ["u--u", "(o)*", "~(o)~", r"\(o)/", "(@@@)o", "_o_)"] {
            assert!(belen.contains(piece), "{piece}\n{belen}");
        }
        assert!(text.contains(".-'~~~~"), "{text}");
        assert!(count(&c, |cell| cell.fg == MUSGO || cell.fg == MUSGO_LIGHT) > 20);
        // no Reyes yet, and no petals
        assert_eq!(
            count(&c, |cell| cell.ch == '^' && cell.fg == DORADO),
            0,
            "{text}"
        );
        assert!(b.petals.is_empty());
    }

    #[test]
    fn the_posadas_add_to_the_nacimiento() {
        let theme = Theme::classic();
        let pieces = |m, d| {
            let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(m, d), 100, 30);
            let t = belen_text(&c);
            ["u--u", "(..)", "(o)*", "_o_)", "~(o)~", r"\(o)/"].map(|p| t.matches(p).count())
        };
        let (early, middle, nochebuena, later) = (
            pieces(12, 13),
            pieces(12, 20),
            pieces(12, 24),
            pieces(12, 26),
        );
        // the ox and the mule, then José and the pastores; the Niño Dios and
        // the angel only from Nochebuena on
        assert_eq!(early, [0, 0, 0, 0, 0, 0]);
        assert_eq!(middle, [1, 1, 1, 1, 0, 0]);
        assert_eq!(pieces(12, 19), [1, 1, 1, 0, 0, 0]);
        assert_eq!(nochebuena[..4], [1, 1, 1, 2]);
        assert_eq!(nochebuena[4..], [1, 1]);
        assert_eq!(later, nochebuena);
        assert_eq!(pieces(1, 1), nochebuena);
        // the tree itself is trimmed more as the posadas go
        let bulbs = |m, d| {
            let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(m, d), 100, 30);
            count(&c, |cell| cell.ch == 'O' && FOQUITOS.contains(&cell.fg))
        };
        assert!(bulbs(12, 12) < bulbs(12, 24));
    }

    #[test]
    fn a_narrow_screen_gets_the_narrow_pesebre() {
        let theme = Theme::classic();
        let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(12, 24), 80, 24);
        let t = c.to_text();
        assert!(t.contains(".-'~~~~~~'-."), "{t}");
        for piece in ["(o)*", "~(o)~"] {
            assert!(t.contains(piece), "{piece}\n{t}");
        }
        // too narrow for either: the tree and the pot, the sheep that fit
        let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(12, 24), 50, 20);
        assert!(!c.to_text().contains(".-'"), "{}", c.to_text());
    }

    #[test]
    fn the_reyes_come_closer_each_day() {
        let theme = Theme::classic();
        let kings = |d| {
            let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(1, d), 200, 40);
            let (w, h) = (c.w() as i32, c.h() as i32);
            // the crowns, and Melchor's gift, on the bottom rows
            let crowns = count(&c, |cell| cell.ch == '^' && cell.fg != Color::Reset);
            let mut gift = None;
            for y in h - 4..h {
                for x in 0..w {
                    if c.get(x, y).unwrap().ch == '$' && gift.is_none() {
                        gift = Some(x);
                    }
                }
            }
            (crowns, gift)
        };
        let days: Vec<_> = (1..=6).map(kings).collect();
        assert_eq!(days[0], (0, None));
        let mut last = i32::MAX;
        for (n, x) in &days[1..] {
            let x = x.expect("Melchor's gift");
            assert!(x < last, "{days:?}");
            last = x;
            // the others follow him in from the edge
            assert!(*n >= 1, "{days:?}");
        }
        assert_eq!(days[5].0, 3);
        // and on the 6th they stand at the pesebre
        let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(1, 6), 200, 40);
        let t = c.to_text();
        assert!(
            t.contains(r"/$\") && t.contains(r"/*\") && t.contains(r"/&\"),
            "{t}"
        );
        // the farther away, the dimmer
        let dim = |d| {
            let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(1, d), 200, 40);
            let fg = c
                .cells()
                .iter()
                .find(|cell| cell.ch == '$')
                .map(|cell| cell.fg);
            fg.unwrap_or(Color::Reset)
        };
        assert_ne!(dim(2), dim(6));
        assert_eq!(dim(6), DORADO);
    }

    #[test]
    fn the_series_blinks_a_step_at_a_time() {
        let theme = Theme::classic();
        let (mut b, c) = dressed(GROWN, &theme, Fiesta::of_day(12, 24), 100, 30);
        let opts = Opts::new(INFO.options, GROWN.parse().unwrap());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta: Fiesta::of_day(12, 24),
        };
        // a grown tree still asks for a redraw each step, and no oftener
        assert!(b.next_change() > 0.0 && b.next_change() <= BLINK);
        let mut frames = vec![c.to_text()];
        let mut seen = std::collections::HashSet::new();
        let mut canvas = Canvas::new(100, 30, Color::Reset);
        for _ in 0..40 {
            b.tick(&ctx, BLINK, BLINK);
            canvas.clear();
            b.draw(&ctx, &mut canvas);
            seen.insert(canvas.to_text());
            assert!(b.next_change() <= BLINK);
        }
        frames.extend(seen);
        assert!(frames.len() > 4, "the lights never change");
    }

    #[test]
    fn ordinary_days_and_other_holidays_are_untouched() {
        use crate::fiesta::Kind;
        let theme = Theme::classic();
        let (_, plain) = dressed(GROWN, &theme, Fiesta::NONE, 100, 30);
        for fiesta in [
            Fiesta::of_day(6, 15),
            Fiesta::height(Kind::Candelaria),
            Fiesta::height(Kind::Equinoccio),
            Fiesta::height(Kind::Revolucion),
            Fiesta::height(Kind::Nino),
        ] {
            let (b, c) = dressed(GROWN, &theme, fiesta, 100, 30);
            assert_eq!(c.cells(), plain.cells(), "{fiesta:?}");
            assert_eq!(b.next_change(), f32::INFINITY);
        }
    }

    #[test]
    fn navidad_and_the_patrias_come_and_go_through_retheme() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, GROWN.parse().unwrap());
        let ctx = |fiesta| Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
            fiesta,
        };
        let plain = ctx(Fiesta::NONE);
        let mut b = Bonsai::new(&plain);
        b.resize(&plain, 100, 30);
        b.retheme(&plain);
        let draw = |b: &mut Bonsai, ctx: &Ctx| {
            let mut c = Canvas::new(100, 30, Color::Reset);
            b.draw(ctx, &mut c);
            c
        };
        let before = draw(&mut b, &plain);
        for (fiesta, mark) in [
            (Fiesta::of_day(12, 24), "(*)v(*)"),
            (Fiesta::of_day(9, 15), "(:)"),
            (Fiesta::of_day(11, 2), "~@~."),
        ] {
            let on = ctx(fiesta);
            b.retheme(&on);
            for _ in 0..20 {
                b.tick(&on, 0.2, 0.2);
            }
            assert!(draw(&mut b, &on).to_text().contains(mark), "{mark}");
            assert!(b.next_change() < 1.0);
            b.retheme(&plain);
            assert!(b.petals.is_empty());
            assert_eq!(b.next_change(), f32::INFINITY);
            assert_eq!(draw(&mut b, &plain).cells(), before.cells());
        }
    }

    #[test]
    fn navidad_and_the_patrias_survive_odd_sizes_and_huge_steps() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        let days = [
            (12, 12),
            (12, 20),
            (12, 24),
            (1, 3),
            (1, 6),
            (9, 1),
            (9, 12),
            (9, 15),
        ];
        for theme in &themes {
            for opts in [
                "",
                "base = 2",
                "base = 0\nmessage = 'hola'",
                "geometry = '3,70'",
                "leaves = '木,葉'",
            ] {
                for (w, h) in [(1, 1), (2, 3), (7, 2), (40, 6), (61, 20), (300, 90)] {
                    for (m, d) in days {
                        let opts = Opts::new(INFO.options, opts.parse().unwrap());
                        let ctx = Ctx {
                            theme,
                            opts: &opts,
                            bg: rgb(0x0d1117),
                            seed: 5,
                            fiesta: Fiesta::of_day(m, d),
                        };
                        let mut b = Bonsai::new(&ctx);
                        b.resize(&ctx, w, h);
                        b.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for dt in [5.0, 0.01, 5.0, 1e6, 0.0, 5.0] {
                            b.tick(&ctx, dt, dt);
                            b.draw(&ctx, &mut c);
                            assert!(b.next_change() >= 0.0);
                        }
                        assert!(b.petals.len() <= PETALS);
                    }
                }
            }
        }
    }

    #[test]
    fn navidad_under_a_theme_uses_its_colors() {
        let theme = Theme::builtin("dracula").unwrap();
        let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(12, 24), 100, 30);
        // none of the classic hexes: dracula's red and its green, its yellow
        // and its pink for the series and the esferas
        for classic in [ROJO_NOCHEBUENA, VERDE_PINO, FOQUITOS[0], FOQUITOS[2]] {
            assert_eq!(count(&c, |cell| cell.fg == classic), 0, "{classic:?}");
        }
        assert!(count(&c, |cell| cell.fg == rgb(0xff5555)) > 5);
        assert!(count(&c, |cell| cell.fg == rgb(0x50fa7b)) > 5);
    }

    #[test]
    fn the_patrias_dress_the_tree_in_the_flags_colors() {
        let theme = Theme::classic();
        let p = Patrias::classic(&CLASSIC, false);
        let (b, c) = dressed(GROWN, &theme, Fiesta::of_day(9, 15), 100, 30);
        let text = c.to_text();
        let verde = count(&c, |cell| cell.ch == '&' && p.verde.contains(&cell.fg));
        let blanco = count(&c, |cell| cell.ch == '*' && p.blanco.contains(&cell.fg));
        let rojo = count(&c, |cell| cell.ch == '@' && p.rojo.contains(&cell.fg));
        assert!(
            verde > 20 && blanco > 5 && rojo > 5,
            "{verde} {blanco} {rojo}\n{text}"
        );
        // white toward the middle, red toward the right
        let mean = |color: &[Color], ch| {
            let (mut sum, mut n) = (0, 0);
            for y in 0..30 {
                for x in 0..100 {
                    let cell = c.get(x, y).unwrap();
                    if cell.ch == ch && color.contains(&cell.fg) {
                        (sum, n) = (sum + x, n + 1);
                    }
                }
            }
            sum as f32 / n as f32
        };
        assert!(mean(&p.blanco, '*') < mean(&p.rojo, '@'), "{text}");
        // the pot banded like the flag, banderitas in the soil
        assert!(count(&c, |cell| cell.ch == '~' && cell.fg == VERDE_BANDERA) > 3);
        assert!(count(&c, |cell| cell.ch == '~' && cell.fg == ROJO_BANDERA) > 3);
        assert!(
            count(&c, |cell| "█▀▄".contains(cell.ch)
                && cell.fg == VERDE_BANDERA)
                >= 2
        );
        assert!(
            count(&c, |cell| "█▀▄".contains(cell.ch)
                && cell.fg == ROJO_BANDERA)
                >= 2
        );
        // the emblem: the eagle with the serpent in its beak on a nopal on a rock
        for piece in [",-^^^^^^^-,", "(:)", r"<(o)\\\|///-'", "~s<"] {
            assert!(text.contains(piece), "{piece}\n{text}");
        }
        assert!(count(&c, |cell| cell.ch == 'o' && cell.fg == ROJO_TUNA) >= 1);
        assert!(count(&c, |cell| cell.fg == NOPAL || cell.fg == NOPAL_CLARO) >= 3);
        // petals fall from the blossoms, white and red
        assert!(!b.petals.is_empty());
    }

    #[test]
    fn the_emblem_grows_as_the_month_climbs() {
        let theme = Theme::classic();
        let look = |d| {
            let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(9, d), 100, 30);
            let t = c.to_text();
            (
                t.matches("(:)").count(),
                t.contains(r"<(o)\\\|///"),
                t.contains("~s<"),
                count(&c, |cell| {
                    "█▀▄".contains(cell.ch) && cell.fg == VERDE_BANDERA
                }),
            )
        };
        assert_eq!(look(1), (1, false, false, 2));
        assert_eq!(look(6), (2, false, false, 2));
        let (pads, eagle, serpent, flags) = look(10);
        assert_eq!((pads, eagle, serpent), (3, false, false));
        assert!(look(12).1);
        assert!(!look(14).2);
        // the serpent only at the Grito and the parade
        assert!(look(15).2 && look(16).2);
        assert!(flags >= 2 && look(15).3 > flags);
    }

    #[test]
    fn the_patrias_under_a_theme_use_its_colors() {
        let theme = Theme::builtin("dracula").unwrap();
        let (_, c) = dressed(GROWN, &theme, Fiesta::of_day(9, 15), 100, 30);
        for classic in [VERDE_BANDERA, ROJO_BANDERA, ROJO_TUNA, NOPAL] {
            assert_eq!(count(&c, |cell| cell.fg == classic), 0, "{classic:?}");
        }
        // dracula's red for the flowers, its foreground for the blossoms
        assert!(count(&c, |cell| cell.ch == '@' && cell.fg == rgb(0xff5555)) > 3);
        assert!(count(&c, |cell| cell.ch == '*' && cell.fg == theme.fg) > 3);
    }

    #[test]
    fn user_leaves_keep_their_glyph_on_every_holiday() {
        let theme = Theme::classic();
        let opts = "seed = 42\nlive = false\ninfinite = false\nleaves = '#'";
        for day in [(12, 24), (9, 15)] {
            let (_, c) = dressed(opts, &theme, Fiesta::of_day(day.0, day.1), 100, 30);
            assert_eq!(count(&c, |cell| cell.ch == '&'), 0);
            assert!(count(&c, |cell| cell.ch == '#' && cell.fg.is_rgb()) > 20);
        }
    }

    #[test]
    fn wide_leaves_print_on_even_columns() {
        let (b, c) = scene("seed = 9\nleaves = '木,葉'\nlive = false", 70, 24);
        assert!(b.leaves.iter().all(|l| l.width == 2));
        for y in 0..24 {
            for x in 0..70 {
                let ch = c.get(x, y).unwrap().ch;
                if ch == '木' || ch == '葉' {
                    assert_eq!(x % 2, 0);
                }
            }
        }
    }
}
