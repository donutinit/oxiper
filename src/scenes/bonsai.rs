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

use crate::color::{self, Color};
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
// The scene

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Growing,
    /// The tree is complete; `hold` counts toward `wait`.
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
    phase: Phase,
    grown: bool,
    grid: Grid,
    msg: Option<MsgBox>,

    pairs: [Color; 5],
    msg_fg: Color,
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
            phase: Phase::Growing,
            grown: false,
            grid: Grid::default(),
            msg: None,
            pairs: CLASSIC,
            msg_fg: Color::Reset,
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
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let mut t = (dt.max(0.0) as f64).min(86_400.0);
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
                    t = self.acc.max(0.0);
                    self.acc = 0.0;
                    self.hold = 0.0;
                    self.phase = Phase::Done;
                }
                Phase::Done => {
                    if !self.infinite {
                        return;
                    }
                    self.hold += t;
                    if self.hold < self.wait {
                        return;
                    }
                    t = self.hold - self.wait;
                    self.seed = self.seeder.u32(..);
                    self.start_tree();
                }
            }
        }
    }

    fn next_change(&self) -> f32 {
        match self.phase {
            Phase::Growing => (self.step - self.acc).max(0.0) as f32,
            Phase::Done if self.infinite => (self.wait - self.hold).max(0.0) as f32,
            Phase::Done => f32::INFINITY,
        }
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        self.draw_base(c);
        let g = &self.grid;
        if let Some((x0, y0, x1, y1)) = g.bbox {
            for y in y0..=y1 {
                let row = &g.cells[(y * g.w) as usize..((y + 1) * g.w) as usize];
                for x in x0..=x1 {
                    let cell = row[x as usize];
                    if cell.ch > ' ' && cell.style & WIDE_TAIL == 0 {
                        let attr = if cell.style & BOLD_BIT != 0 { BOLD } else { 0 };
                        c.set_attr(x, y, cell.ch, self.pairs[(cell.style & 7) as usize], attr);
                    }
                }
            }
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
    fn resize_keeps_the_tree() {
        let (mut b, c1) = scene("seed = 42\nlive = false\ninfinite = false", 60, 24);
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
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
