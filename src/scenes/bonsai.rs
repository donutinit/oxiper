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

use crate::color::{self, Color, rgb};
use crate::fiesta::Fiesta;
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

/// The holiday's colors.
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
// The scene

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

    // Día de Muertos
    fiesta: Fiesta,
    dress: Dress,
    /// Leaves may turn into blossom glyphs (only with the default `&` leaves;
    /// leaves the user chose keep their glyph and only change color).
    bloom_glyphs: bool,
    /// The holiday's own randomness.
    frng: fastrand::Rng,
    petals: Vec<Petal>,
    /// Petals owed, and scene seconds for the candles' flicker.
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
            dress: Dress::CLASSIC,
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
        // Petals lying on the ground, under everything else.
        for p in self.petals.iter().filter(|p| p.rest >= 0.0) {
            let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
            if c.get(x, y).is_some_and(|cell| cell.ch == ' ') {
                let fg = ctx.fade(p.color, (1.0 - p.rest / 1.5).clamp(0.0, 1.0));
                c.set(x, y, p.lie, fg);
            }
        }
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
        // Petals in the air, in front of it all.
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
        self.spawn += dt * (0.8 + 3.2 * self.fiesta.level);
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
            let (color, shade) = match bloom(x, y, self.seed, self.fiesta.level) {
                Bloom::Marigold(_) => (d.marigold[2], d.marigold[1]),
                Bloom::Velvet(_) => (d.velvet[1], d.velvet[0]),
                _ => continue,
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
        if !self.fiesta.on() {
            self.petals.clear();
        }
    }

    fn update(&mut self, ctx: &Ctx, dt: f32) {
        self.tick(ctx, dt, dt);
    }

    fn tick(&mut self, _ctx: &Ctx, dt: f32, real: f32) {
        if self.fiesta.on() {
            self.tick_fiesta(dt, real);
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
        if self.fiesta.on() {
            // candles flicker and petals fall
            return 0.0;
        }
        match self.phase {
            Phase::Growing => (self.step - self.acc).max(0.0) as f32,
            // The wait is in real seconds; this is asked in scene seconds.
            Phase::Done if self.infinite => ((self.wait - self.hold).max(0.0) * self.pace) as f32,
            Phase::Done => f32::INFINITY,
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let dressed = self.fiesta.on();
        if dressed {
            self.draw_talavera(c);
        } else {
            self.draw_base(c);
        }
        let g = &self.grid;
        if let Some((x0, y0, x1, y1)) = g.bbox {
            for y in y0..=y1 {
                let row = &g.cells[(y * g.w) as usize..((y + 1) * g.w) as usize];
                for x in x0..=x1 {
                    let cell = row[x as usize];
                    if cell.ch > ' ' && cell.style & WIDE_TAIL == 0 {
                        let attr = if cell.style & BOLD_BIT != 0 { BOLD } else { 0 };
                        let (ch, fg) = match cell.style & 7 {
                            LEAF_DARK | LEAF_BRIGHT if dressed => self.blossom(x, y, cell),
                            pair => (cell.ch, self.pairs[pair as usize]),
                        };
                        c.set_attr(x, y, ch, fg, attr);
                    }
                }
            }
        }
        if dressed {
            self.draw_ofrenda(ctx, c);
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
