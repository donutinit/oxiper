//! The cell grid scenes draw into, and the diffing renderer that turns it into
//! the fewest escape bytes possible.

use crate::color::{self, Color, ColorMode};

pub const BOLD: u8 = 1;
pub const DIM: u8 = 2;
pub const ITALIC: u8 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub attr: u8,
}

impl Cell {
    pub const BLANK: Cell = Cell {
        ch: ' ',
        fg: Color::Reset,
        bg: Color::Reset,
        attr: 0,
    };
}

/// Does this character take two terminal columns (CJK, fullwidth forms, most emoji)?
#[inline]
pub fn is_wide(c: char) -> bool {
    let c = c as u32;
    c >= 0x1100
        && matches!(c,
            0x1100..=0x115f | 0x231a..=0x231b | 0x2329..=0x232a | 0x23e9..=0x23ec | 0x23f0 | 0x23f3
            | 0x25fd..=0x25fe | 0x2614..=0x2615 | 0x2648..=0x2653 | 0x267f | 0x2693 | 0x26a1
            | 0x26aa..=0x26ab | 0x26bd..=0x26be | 0x26c4..=0x26c5 | 0x26ce | 0x26d4 | 0x26ea
            | 0x26f2..=0x26f3 | 0x26f5 | 0x26fa | 0x26fd | 0x2705 | 0x270a..=0x270b | 0x2728 | 0x274c
            | 0x274e | 0x2753..=0x2755 | 0x2757 | 0x2795..=0x2797 | 0x27b0 | 0x27bf | 0x2b1b..=0x2b1c
            | 0x2b50 | 0x2b55 | 0x2e80..=0x303e | 0x3041..=0x33ff | 0x3400..=0x4dbf | 0x4e00..=0x9fff
            | 0xa000..=0xa4cf | 0xa960..=0xa97f | 0xac00..=0xd7a3 | 0xf900..=0xfaff | 0xfe10..=0xfe19
            | 0xfe30..=0xfe6f | 0xff00..=0xff60 | 0xffe0..=0xffe6 | 0x1f004 | 0x1f0cf | 0x1f18e
            | 0x1f191..=0x1f19a | 0x1f200..=0x1f2ff | 0x1f300..=0x1f320 | 0x1f32d..=0x1f335
            | 0x1f337..=0x1f37c | 0x1f37e..=0x1f393 | 0x1f3a0..=0x1f3ca | 0x1f3cf..=0x1f3d3
            | 0x1f3e0..=0x1f3f0 | 0x1f3f4 | 0x1f3f8..=0x1f43e | 0x1f440 | 0x1f442..=0x1f4fc
            | 0x1f4ff..=0x1f53d | 0x1f54b..=0x1f54e | 0x1f550..=0x1f567 | 0x1f57a | 0x1f595..=0x1f596
            | 0x1f5a4 | 0x1f5fb..=0x1f64f | 0x1f680..=0x1f6c5 | 0x1f6cc | 0x1f6d0..=0x1f6d2
            | 0x1f6d5..=0x1f6d7 | 0x1f6eb..=0x1f6ec | 0x1f6f4..=0x1f6fc | 0x1f7e0..=0x1f7eb
            | 0x1f90c..=0x1f93a | 0x1f93c..=0x1f945 | 0x1f947..=0x1f9ff | 0x1fa70..=0x1faff
            | 0x20000..=0x3fffd)
}

/// What scenes draw on. Coordinates are `i32` and silently clipped, so scenes
/// can draw sprites that hang off the edges without bounds checks.
pub struct Canvas {
    w: u16,
    h: u16,
    cells: Vec<Cell>,
    blank: Cell,
    /// Never paint a background color, not even for two-colored half blocks.
    pub strict: bool,
}

impl Canvas {
    pub fn new(w: u16, h: u16, bg: Color) -> Canvas {
        let blank = Cell { bg, ..Cell::BLANK };
        Canvas {
            w,
            h,
            cells: vec![blank; w as usize * h as usize],
            blank,
            strict: false,
        }
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.w = w;
        self.h = h;
        self.cells.clear();
        self.cells.resize(w as usize * h as usize, self.blank);
    }

    pub fn set_background(&mut self, bg: Color) {
        self.blank.bg = bg;
    }

    #[inline]
    pub fn w(&self) -> u16 {
        self.w
    }

    #[inline]
    pub fn h(&self) -> u16 {
        self.h
    }

    /// The background empty cells use: `Color::Reset` when the terminal stays transparent.
    #[inline]
    pub fn bg(&self) -> Color {
        self.blank.bg
    }

    pub fn clear(&mut self) {
        self.cells.fill(self.blank);
    }

    #[inline]
    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        (x >= 0 && y >= 0 && x < self.w as i32 && y < self.h as i32)
            .then(|| y as usize * self.w as usize + x as usize)
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Option<&Cell> {
        self.idx(x, y).map(|i| &self.cells[i])
    }

    #[inline]
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        self.idx(x, y).map(|i| &mut self.cells[i])
    }

    /// Draws a character, keeping whatever background the cell already has.
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, ch: char, fg: Color) {
        if let Some(i) = self.idx(x, y) {
            let c = &mut self.cells[i];
            c.ch = ch;
            c.fg = fg;
            c.attr = 0;
        }
    }

    #[inline]
    pub fn set_attr(&mut self, x: i32, y: i32, ch: char, fg: Color, attr: u8) {
        if let Some(i) = self.idx(x, y) {
            let c = &mut self.cells[i];
            c.ch = ch;
            c.fg = fg;
            c.attr = attr;
        }
    }

    /// Paints a cell including its background. In strict-transparency mode the
    /// background is dropped and only the glyph is kept.
    #[inline]
    #[allow(dead_code)] // scene API (docs/SCENES.md)
    pub fn set_cell(&mut self, x: i32, y: i32, mut cell: Cell) {
        if self.strict {
            cell.bg = self.blank.bg;
        }
        if let Some(i) = self.idx(x, y) {
            self.cells[i] = cell;
        }
    }

    /// Paints only the background of a cell (e.g. a lightning flash).
    #[inline]
    pub fn set_bg(&mut self, x: i32, y: i32, bg: Color) {
        if self.strict {
            return;
        }
        if let Some(i) = self.idx(x, y) {
            self.cells[i].bg = bg;
        }
    }

    /// Writes a string left to right. Spaces are drawn (they erase); use `art` for
    /// sprites where spaces should be see-through.
    pub fn text(&mut self, x: i32, y: i32, s: &str, fg: Color) -> i32 {
        self.text_attr(x, y, s, fg, 0)
    }

    /// Like `text` with attributes. Double-width characters take two cells.
    pub fn text_attr(&mut self, x: i32, y: i32, s: &str, fg: Color, attr: u8) -> i32 {
        let mut cx = x;
        for ch in s.chars() {
            self.set_attr(cx, y, ch, fg, attr);
            cx += if is_wide(ch) { 2 } else { 1 };
        }
        cx
    }

    /// Draws multi-line ASCII art where spaces are transparent.
    #[allow(dead_code)] // scene API (docs/SCENES.md)
    pub fn art(&mut self, x: i32, y: i32, lines: &[&str], fg: Color) {
        for (dy, line) in lines.iter().enumerate() {
            for (dx, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    self.set(x + dx as i32, y + dy as i32, ch, fg);
                }
            }
        }
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// The frame as plain text, one line per row (for snapshots and tests).
    pub fn to_text(&self) -> String {
        let mut s = String::with_capacity(self.cells.len() + self.h as usize);
        for row in self.cells.chunks(self.w.max(1) as usize) {
            let line: String = row.iter().map(|c| c.ch).collect();
            s.push_str(line.trim_end());
            s.push('\n');
        }
        s
    }
}

/// Sub-cell canvas made of braille dots: 2×4 dots per cell, one color per cell.
pub struct Braille {
    w: usize,
    h: usize,
    dots: Vec<u8>,
    color: Vec<Color>,
}

const BRAILLE_BITS: [[u8; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

impl Braille {
    pub fn new(cols: u16, rows: u16) -> Braille {
        let n = cols as usize * rows as usize;
        Braille {
            w: cols as usize,
            h: rows as usize,
            dots: vec![0; n],
            color: vec![Color::Reset; n],
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        *self = Braille::new(cols, rows);
    }

    /// Width in dots.
    pub fn pw(&self) -> i32 {
        self.w as i32 * 2
    }

    /// Height in dots.
    pub fn ph(&self) -> i32 {
        self.h as i32 * 4
    }

    pub fn clear(&mut self) {
        self.dots.fill(0);
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, c: Color) {
        if x < 0 || y < 0 || x >= self.pw() || y >= self.ph() {
            return;
        }
        let i = (y as usize / 4) * self.w + x as usize / 2;
        self.dots[i] |= BRAILLE_BITS[y as usize & 3][x as usize & 1];
        self.color[i] = c;
    }

    #[inline]
    pub fn unset(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.pw() || y >= self.ph() {
            return;
        }
        let i = (y as usize / 4) * self.w + x as usize / 2;
        self.dots[i] &= !BRAILLE_BITS[y as usize & 3][x as usize & 1];
    }

    #[inline]
    pub fn setf(&mut self, x: f32, y: f32, c: Color) {
        self.set(x.floor() as i32, y.floor() as i32, c);
    }

    /// Bresenham line in dot coordinates.
    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, c: Color) {
        let (mut x, mut y) = (x0.round() as i32, y0.round() as i32);
        let (x1, y1) = (x1.round() as i32, y1.round() as i32);
        let (dx, dy) = ((x1 - x).abs(), -(y1 - y).abs());
        let (sx, sy) = (if x < x1 { 1 } else { -1 }, if y < y1 { 1 } else { -1 });
        let mut err = dx + dy;
        // Guard against absurdly long lines from bad input.
        for _ in 0..4096 {
            self.set(x, y, c);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Like `line`, but the color is sampled along the way (t goes 0→1).
    #[allow(dead_code)] // scene API (docs/SCENES.md)
    pub fn line_with(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        mut f: impl FnMut(f32) -> Color,
    ) {
        let n = ((x1 - x0).abs().max((y1 - y0).abs()).ceil() as i32).clamp(1, 4096);
        for i in 0..=n {
            let t = i as f32 / n as f32;
            self.setf(x0 + (x1 - x0) * t + 0.5, y0 + (y1 - y0) * t + 0.5, f(t));
        }
    }

    /// Copies every non-empty braille cell onto the canvas.
    pub fn blit(&self, c: &mut Canvas) {
        for row in 0..self.h.min(c.h() as usize) {
            for col in 0..self.w.min(c.w() as usize) {
                let i = row * self.w + col;
                let d = self.dots[i];
                if d != 0 {
                    let ch = char::from_u32(0x2800 + d as u32).unwrap_or(' ');
                    c.set(col as i32, row as i32, ch, self.color[i]);
                }
            }
        }
    }
}

/// Sub-cell canvas of square-ish pixels: each cell holds two (upper/lower half block).
/// `Color::Reset` pixels are empty, so the terminal background shows through.
pub struct Pixels {
    w: usize,
    h: usize,
    px: Vec<Color>,
}

impl Pixels {
    pub fn new(cols: u16, rows: u16) -> Pixels {
        Pixels {
            w: cols as usize,
            h: rows as usize * 2,
            px: vec![Color::Reset; cols as usize * rows as usize * 2],
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        *self = Pixels::new(cols, rows);
    }

    pub fn pw(&self) -> i32 {
        self.w as i32
    }

    pub fn ph(&self) -> i32 {
        self.h as i32
    }

    pub fn clear(&mut self) {
        self.px.fill(Color::Reset);
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, c: Color) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            self.px[y as usize * self.w + x as usize] = c;
        }
    }

    #[inline]
    #[allow(dead_code)] // scene API (docs/SCENES.md)
    pub fn get(&self, x: i32, y: i32) -> Color {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            self.px[y as usize * self.w + x as usize]
        } else {
            Color::Reset
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: Color) {
        for yy in y.max(0)..(y + h).min(self.h as i32) {
            for xx in x.max(0)..(x + w).min(self.w as i32) {
                self.px[yy as usize * self.w + xx as usize] = c;
            }
        }
    }

    /// Composites onto the canvas with half blocks. Cells whose two pixels are
    /// empty are left untouched; a cell with one empty half only sets a
    /// foreground color, so the empty half stays transparent.
    pub fn blit(&self, c: &mut Canvas) {
        let bg = c.bg();
        let strict = c.strict;
        for row in 0..(self.h / 2).min(c.h() as usize) {
            for col in 0..self.w.min(c.w() as usize) {
                let top = self.px[row * 2 * self.w + col];
                let bot = self.px[(row * 2 + 1) * self.w + col];
                let cell = match (top, bot) {
                    (Color::Reset, Color::Reset) => continue,
                    (t, Color::Reset) => Cell {
                        ch: '▀',
                        fg: t,
                        bg,
                        attr: 0,
                    },
                    (Color::Reset, b) => Cell {
                        ch: '▄',
                        fg: b,
                        bg,
                        attr: 0,
                    },
                    (t, b) if t == b => Cell {
                        ch: '█',
                        fg: t,
                        bg,
                        attr: 0,
                    },
                    (t, b) if strict => Cell {
                        ch: '█',
                        fg: t.lerp(b, 0.5),
                        bg,
                        attr: 0,
                    },
                    (t, b) => Cell {
                        ch: '▀',
                        fg: t,
                        bg: b,
                        attr: 0,
                    },
                };
                if let Some(slot) = c.get_mut(col as i32, row as i32) {
                    *slot = cell;
                }
            }
        }
    }
}

/// Turns canvases into escape sequences, emitting only the cells that changed
/// since the previous frame and only the SGR parameters that differ.
pub struct Renderer {
    front: Vec<Cell>,
    w: u16,
    h: u16,
    pub mode: ColorMode,
    out: Vec<u8>,
    full: bool,
    fg: Color,
    bg: Color,
    attr: u8,
    /// Wrap frames in synchronized-update markers (DEC mode 2026) so the
    /// terminal never shows a half-drawn frame.
    pub sync: bool,
}

impl Renderer {
    pub fn new(mode: ColorMode) -> Renderer {
        Renderer {
            front: Vec::new(),
            w: 0,
            h: 0,
            mode,
            out: Vec::with_capacity(64 * 1024),
            full: true,
            fg: Color::Reset,
            bg: Color::Reset,
            attr: 0,
            sync: true,
        }
    }

    /// Forces the next frame to repaint everything (after a resize or when the
    /// screen may have been disturbed).
    pub fn invalidate(&mut self) {
        self.full = true;
    }

    /// Diffs `canvas` against the last frame and returns the bytes to write
    /// (empty when nothing changed).
    pub fn frame(&mut self, canvas: &Canvas) -> &[u8] {
        self.out.clear();
        let (w, h) = (canvas.w(), canvas.h());
        if w != self.w || h != self.h || self.front.len() != canvas.cells().len() {
            self.w = w;
            self.h = h;
            self.front = vec![Cell::BLANK; canvas.cells().len()];
            self.full = true;
        }
        let full = std::mem::take(&mut self.full);
        let cells = canvas.cells();
        if self.sync {
            self.out.extend_from_slice(b"\x1b[?2026h");
        }
        if full {
            self.out.extend_from_slice(b"\x1b[0m");
            self.fg = Color::Reset;
            self.bg = Color::Reset;
            self.attr = 0;
            // With a solid background, clearing with it avoids a flash of the default color.
            self.emit_sgr(Color::Reset, canvas.bg(), 0);
            self.out.extend_from_slice(b"\x1b[2J");
        }
        let start = self.out.len();
        let mut cur: Option<(u16, u16)> = None;
        for y in 0..h {
            let row = y as usize * w as usize;
            // The previous cell holds a double-width glyph covering this one.
            let mut covered = false;
            for x in 0..w {
                let i = row + x as usize;
                let c = cells[i];
                let wide = is_wide(c.ch) && x + 1 < w;
                if std::mem::replace(&mut covered, wide) {
                    covered = false;
                    continue;
                }
                // Overwriting the left half of a wide glyph blanks its right half
                // on screen, so that cell must be repainted even if "unchanged".
                let uncovered = x > 0 && is_wide(self.front[i - 1].ch) && !is_wide(cells[i - 1].ch);
                if !full && c == self.front[i] && !uncovered {
                    continue;
                }
                if full && c.ch == ' ' && c.bg == canvas.bg() {
                    // Already cleared by ED 2.
                    continue;
                }
                self.move_to(cur, x, y);
                self.emit_sgr(c.fg, c.bg, c.attr);
                let mut buf = [0u8; 4];
                let ch = if c.ch < ' ' || c.ch == '\x7f' || (is_wide(c.ch) && !wide) {
                    ' '
                } else {
                    c.ch
                };
                self.out
                    .extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                cur = Some((x + if wide { 2 } else { 1 }, y));
            }
        }
        self.front.copy_from_slice(cells);
        if self.out.len() == start && !full {
            self.out.clear();
            return &self.out;
        }
        if self.sync {
            self.out.extend_from_slice(b"\x1b[?2026l");
        }
        &self.out
    }

    fn move_to(&mut self, cur: Option<(u16, u16)>, x: u16, y: u16) {
        match cur {
            Some((cx, cy)) if cy == y && cx == x => {}
            Some((cx, cy)) if cy == y && x > cx => {
                // Cursor forward is shorter than an absolute position.
                let n = x - cx;
                self.out.extend_from_slice(b"\x1b[");
                if n > 1 {
                    push_num(&mut self.out, n as u32);
                }
                self.out.push(b'C');
            }
            _ => {
                self.out.extend_from_slice(b"\x1b[");
                push_num(&mut self.out, y as u32 + 1);
                self.out.push(b';');
                push_num(&mut self.out, x as u32 + 1);
                self.out.push(b'H');
            }
        }
    }

    fn emit_sgr(&mut self, fg: Color, bg: Color, attr: u8) {
        let mut attr = attr;
        let (mut fg, mut bg) = (fg, bg);
        if self.mode == ColorMode::Mono {
            // Keep some depth without color: dim the dark, embolden the bright.
            if fg != Color::Reset && attr & (BOLD | DIM) == 0 {
                let l = fg.luma();
                attr |= if l < 0.3 {
                    DIM
                } else if l > 0.78 {
                    BOLD
                } else {
                    0
                };
            }
            fg = Color::Reset;
            bg = Color::Reset;
        }
        if fg == self.fg && bg == self.bg && attr == self.attr {
            return;
        }
        let start = self.out.len();
        self.out.extend_from_slice(b"\x1b[");
        let mut first = true;
        let mut param = |out: &mut Vec<u8>, f: &dyn Fn(&mut Vec<u8>)| {
            if !first {
                out.push(b';');
            }
            first = false;
            f(out);
        };
        if attr != self.attr {
            if self.attr & !attr & (BOLD | DIM) != 0 {
                param(&mut self.out, &|o| o.extend_from_slice(b"22"));
            }
            if self.attr & !attr & ITALIC != 0 {
                param(&mut self.out, &|o| o.extend_from_slice(b"23"));
            }
            let readd = if self.attr & !attr & (BOLD | DIM) != 0 {
                attr
            } else {
                attr & !self.attr
            };
            if readd & BOLD != 0 {
                param(&mut self.out, &|o| o.push(b'1'));
            }
            if readd & DIM != 0 {
                param(&mut self.out, &|o| o.push(b'2'));
            }
            if attr & !self.attr & ITALIC != 0 {
                param(&mut self.out, &|o| o.push(b'3'));
            }
        }
        let mode = self.mode;
        if fg != self.fg {
            param(&mut self.out, &|o| push_color(o, fg, false, mode));
        }
        if bg != self.bg {
            param(&mut self.out, &|o| push_color(o, bg, true, mode));
        }
        self.out.push(b'm');
        if first {
            self.out.truncate(start);
        }
        self.fg = fg;
        self.bg = bg;
        self.attr = attr;
    }
}

fn push_color(out: &mut Vec<u8>, c: Color, bg: bool, mode: ColorMode) {
    let ansi = |out: &mut Vec<u8>, n: u8| {
        let base = match (n < 8, bg) {
            (true, false) => 30,
            (true, true) => 40,
            (false, false) => 90 - 8,
            (false, true) => 100 - 8,
        };
        push_num(out, base + n as u32);
    };
    match c {
        Color::Reset => out.extend_from_slice(if bg { b"49" } else { b"39" }),
        Color::Ansi(n) if n < 16 => ansi(out, n),
        Color::Ansi(n) => {
            if mode == ColorMode::Ansi16 {
                let (r, g, b) = color::ansi_rgb(n);
                ansi(out, color::to_16(r, g, b));
            } else {
                out.extend_from_slice(if bg { b"48;5;" } else { b"38;5;" });
                push_num(out, n as u32);
            }
        }
        Color::Rgb(r, g, b) => match mode {
            ColorMode::TrueColor => {
                out.extend_from_slice(if bg { b"48;2;" } else { b"38;2;" });
                push_num(out, r as u32);
                out.push(b';');
                push_num(out, g as u32);
                out.push(b';');
                push_num(out, b as u32);
            }
            ColorMode::Ansi256 => {
                out.extend_from_slice(if bg { b"48;5;" } else { b"38;5;" });
                push_num(out, color::to_256(r, g, b) as u32);
            }
            ColorMode::Ansi16 | ColorMode::Mono => ansi(out, color::to_16(r, g, b)),
        },
    }
}

#[inline]
fn push_num(out: &mut Vec<u8>, mut n: u32) {
    let mut buf = [0u8; 10];
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    out.extend_from_slice(&buf[i..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(r: &mut Renderer, c: &Canvas) -> String {
        String::from_utf8(r.frame(c).to_vec()).unwrap()
    }

    #[test]
    fn unchanged_frame_emits_nothing() {
        let mut c = Canvas::new(10, 3, Color::Reset);
        let mut r = Renderer::new(ColorMode::TrueColor);
        c.set(1, 1, 'x', Color::Rgb(1, 2, 3));
        assert!(!frame(&mut r, &c).is_empty());
        assert!(frame(&mut r, &c).is_empty());
    }

    #[test]
    fn diff_emits_only_changes() {
        let mut c = Canvas::new(10, 3, Color::Reset);
        let mut r = Renderer::new(ColorMode::TrueColor);
        r.sync = false;
        frame(&mut r, &c);
        c.set(4, 2, 'a', Color::Rgb(255, 0, 0));
        c.set(5, 2, 'b', Color::Rgb(255, 0, 0));
        assert_eq!(frame(&mut r, &c), "\x1b[3;5H\x1b[38;2;255;0;0mab");
        c.set(5, 2, 'c', Color::Rgb(255, 0, 0));
        assert_eq!(frame(&mut r, &c), "\x1b[3;6Hc");
    }

    #[test]
    fn transparent_background_never_sets_bg() {
        let mut c = Canvas::new(4, 2, Color::Reset);
        let mut r = Renderer::new(ColorMode::TrueColor);
        let mut p = Pixels::new(4, 2);
        p.set(0, 0, Color::Rgb(9, 9, 9));
        p.set(1, 1, Color::Rgb(9, 9, 9));
        p.blit(&mut c);
        let s = frame(&mut r, &c);
        assert!(!s.contains("48;"), "{s:?}");
        assert!(s.contains('▀') && s.contains('▄'));
    }

    #[test]
    fn strict_mode_merges_half_blocks() {
        let mut c = Canvas::new(1, 1, Color::Reset);
        c.strict = true;
        let mut p = Pixels::new(1, 1);
        p.set(0, 0, Color::Rgb(200, 0, 0));
        p.set(0, 1, Color::Rgb(0, 0, 200));
        p.blit(&mut c);
        assert_eq!(c.get(0, 0).unwrap().bg, Color::Reset);
    }

    #[test]
    fn attrs_reset_cleanly() {
        let mut c = Canvas::new(3, 1, Color::Reset);
        let mut r = Renderer::new(ColorMode::Ansi16);
        r.sync = false;
        frame(&mut r, &c);
        c.set_attr(0, 0, 'a', Color::Ansi(2), BOLD);
        c.set_attr(1, 0, 'b', Color::Ansi(2), 0);
        assert_eq!(frame(&mut r, &c), "\x1b[1;1H\x1b[1;32ma\x1b[22mb");
    }

    #[test]
    fn wide_glyphs_cover_the_next_cell() {
        let mut c = Canvas::new(4, 1, Color::Reset);
        let mut r = Renderer::new(ColorMode::TrueColor);
        r.sync = false;
        frame(&mut r, &c);
        c.set(0, 0, '🌸', Color::Reset);
        c.set(2, 0, 'x', Color::Reset);
        // The blank under the flower's right half is not painted over it.
        assert_eq!(frame(&mut r, &c), "\x1b[1;1H🌸x");
        c.set(0, 0, 'a', Color::Reset);
        // Replacing it repaints the cell it used to cover.
        assert_eq!(frame(&mut r, &c), "\x1b[1;1Ha ");
        assert!(!is_wide('a') && !is_wide('▀') && is_wide('漢') && !is_wide('ｱ'));
    }

    #[test]
    fn braille_dots() {
        let mut b = Braille::new(1, 1);
        let mut c = Canvas::new(1, 1, Color::Reset);
        b.set(0, 0, Color::Ansi(1));
        b.set(1, 3, Color::Ansi(1));
        b.blit(&mut c);
        assert_eq!(c.get(0, 0).unwrap().ch, '\u{2881}');
    }
}
