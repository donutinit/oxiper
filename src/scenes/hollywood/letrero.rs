#![allow(dead_code)] // wired in Task 11

//! The fitted banner: the access pane's two lines of big letters, wrapped
//! between words and drawn at the biggest of eight sizes that fits its pane
//! (spec 7.5). A pure function chooses the size, the lines and the box; one
//! routine draws every size from quarter cells.

use super::widgets;
use super::{Rect, put, put_ch};
use crate::color::Color;
use crate::render::{BOLD, Canvas};

/// The banner's font: `widgets::glyph`, and `Ñ ? ¿ - . +`.
pub fn glyph(ch: char) -> [u8; 5] {
    match ch {
        '\u{00d1}' => [0b111, 0b000, 0b110, 0b101, 0b101],
        '?' => [0b110, 0b001, 0b010, 0b000, 0b010],
        '\u{00bf}' => [0b010, 0b000, 0b010, 0b100, 0b011],
        '-' => [0, 0, 0b111, 0, 0],
        '.' => [0, 0, 0, 0, 0b010],
        '+' => [0, 0b010, 0b111, 0b010, 0],
        _ => widgets::glyph(ch),
    }
}

/// Can the banner draw `ch` in big letters? (A space is no glyph: it parts words.)
pub fn has_glyph(ch: char) -> bool {
    glyph(ch) != [0; 5]
}

/// How big a letter is drawn.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Size {
    Pocket,
    Compact,
    Narrow,
    X2,
    X3,
    X4,
    X5,
    X6,
}

impl Size {
    pub const BIG_FIRST: [Size; 8] = [
        Size::X6,
        Size::X5,
        Size::X4,
        Size::X3,
        Size::X2,
        Size::Narrow,
        Size::Compact,
        Size::Pocket,
    ];

    /// What one font pixel takes, in half columns and half rows.
    pub fn pixel(self) -> (i32, i32) {
        match self {
            Size::X6 => (12, 6),
            Size::X5 => (10, 5),
            Size::X4 => (8, 4),
            Size::X3 => (6, 3),
            Size::X2 => (4, 2),
            Size::Narrow => (2, 2),
            Size::Compact => (2, 1),
            Size::Pocket => (1, 1),
        }
    }

    /// Columns a letter takes with its gap: 4 pixels wide.
    pub fn letter(self) -> i32 {
        2 * self.pixel().0
    }

    /// The box's rows for `n` lines of letters, frame included: five pixels
    /// a line and one between lines.
    fn box_h(self, n: usize) -> i32 {
        2 + ((6 * n as i32 - 1) * self.pixel().1 + 1) / 2
    }
}

/// A banner fitted to a pane.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Fit {
    pub size: Size,
    /// two to four lines of big letters
    pub lines: Vec<String>,
    /// the box, frame included
    pub w: i32,
    pub h: i32,
}

/// `w` x `h` is the inside of the pane. None: below pocket (the caller draws one line of text, or nothing).
/// None too when `top` or `bottom` has no word.
pub fn fit(top: &str, bottom: &str, w: i32, h: i32) -> Option<Fit> {
    Size::BIG_FIRST
        .into_iter()
        .find_map(|size| fit_at(size, top, bottom, w, h))
}

/// The banner at `size`, when no word is too long for a line, there are four
/// lines at most and the box is no higher than the pane.
fn fit_at(size: Size, top: &str, bottom: &str, w: i32, h: i32) -> Option<Fit> {
    let per = (w - 2) / size.letter();
    let mut lines = Vec::new();
    for text in [top, bottom] {
        let before = lines.len();
        wrap(text, per, &mut lines)?;
        if lines.len() == before || lines.len() > 4 {
            return None;
        }
    }
    let bh = size.box_h(lines.len());
    if bh > h {
        return None;
    }
    let longest = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as i32;
    Some(Fit {
        size,
        w: 2 + size.letter() * longest,
        h: bh,
        lines,
    })
}

/// Wraps `text` greedily between words into lines of `per` letters at most,
/// onto `lines`. None when a word is longer than that.
fn wrap(text: &str, per: i32, lines: &mut Vec<String>) -> Option<()> {
    let mut line = String::new();
    let mut len = 0;
    for word in text.split_whitespace() {
        let n = word.chars().count() as i32;
        if n > per {
            return None;
        }
        if len > 0 && len + 1 + n <= per {
            line.push(' ');
            len += 1;
        } else if len > 0 {
            lines.push(std::mem::take(&mut line));
            len = 0;
        }
        line.push_str(word);
        len += n;
    }
    if len > 0 {
        lines.push(line);
    }
    Some(())
}

/// The banner's colors.
pub struct Look {
    pub letters: Color,
    pub frame: Color,
    pub caption: Color,
    /// the box's background; None paints none
    pub ink: Option<Color>,
}

/// A cell's character for its four quarters lit: top left 1, top right 2,
/// bottom left 4, bottom right 8.
const QUADS: [char; 16] = [
    ' ', '\u{2598}', '\u{259d}', '\u{2580}', '\u{2596}', '\u{258c}', '\u{259e}', '\u{259b}',
    '\u{2597}', '\u{259a}', '\u{2590}', '\u{259c}', '\u{2584}', '\u{2599}', '\u{259f}', '\u{2588}',
];

/// The box centered in `r`: background, frame, the two captions when they fit in it, the letters.
pub fn draw(c: &mut Canvas, r: Rect, fit: &Fit, captions: (&str, &str), look: &Look) {
    let (bw, bh) = (fit.w, fit.h);
    if bw < 2 || bh < 2 {
        return;
    }
    let (bx, by) = ((r.w - bw) / 2, (r.h - bh) / 2);
    // The background: blanks in the letters' color, the ink behind them.
    for y in by.max(0)..(by + bh).min(r.h) {
        for x in bx.max(0)..(bx + bw).min(r.w) {
            put_ch(c, r, x, y, ' ', look.letters, 0);
            if let Some(ink) = look.ink {
                c.set_bg(r.x + x, r.y + y, ink);
            }
        }
    }
    for x in bx..bx + bw {
        put_ch(c, r, x, by, '\u{2550}', look.frame, BOLD);
        put_ch(c, r, x, by + bh - 1, '\u{2550}', look.frame, BOLD);
    }
    for y in by..by + bh {
        put_ch(c, r, bx, y, '\u{2551}', look.frame, BOLD);
        put_ch(c, r, bx + bw - 1, y, '\u{2551}', look.frame, BOLD);
    }
    for (y, t) in [(by, captions.0), (by + bh - 1, captions.1)] {
        let n = t.chars().count() as i32;
        if n > 0 && n + 2 <= bw {
            put(c, r, bx + (bw - n) / 2, y, t, look.caption, BOLD);
        }
    }
    letters(c, r, fit, (bx + 1, by + 1), look.letters);
}

/// Rasterizes the letters into quarter cells (half columns, half rows) inside
/// the frame, whose top left cell is `inner`. A pixel a whole number of
/// columns wide starts on a column, so those sizes draw full blocks across.
fn letters(c: &mut Canvas, r: Rect, fit: &Fit, inner: (i32, i32), col: Color) {
    let (pw, ph) = fit.size.pixel();
    let (iw, ih) = (2 * (fit.w - 2), 2 * (fit.h - 2));
    let n = fit.lines.len() as i32;
    let snap = |off: i32, unit: i32| {
        if unit % 2 == 0 {
            off - off.rem_euclid(2)
        } else {
            off
        }
    };
    let top = 2 * inner.1 + snap((ih - (6 * n - 1) * ph) / 2, ph);
    for (i, line) in fit.lines.iter().enumerate() {
        let len = line.chars().count() as i32;
        let left = 2 * inner.0 + snap((iw - (4 * len - 1) * pw) / 2, pw);
        let y0 = top + 6 * i as i32 * ph;
        for (j, ch) in line.chars().enumerate() {
            let x0 = left + 4 * j as i32 * pw;
            for (row, bits) in glyph(ch).iter().enumerate() {
                for px in 0..3 {
                    if bits >> (2 - px) & 1 == 1 {
                        let (qx, qy) = (x0 + px * pw, y0 + row as i32 * ph);
                        for y in qy..qy + ph {
                            for x in qx..qx + pw {
                                light(c, r, x, y, col);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Lights one quarter cell, relative to `r` and clipped to it, keeping the
/// quarters of the cell already lit.
fn light(c: &mut Canvas, r: Rect, qx: i32, qy: i32, col: Color) {
    let (x, y) = (qx.div_euclid(2), qy.div_euclid(2));
    if x < 0 || y < 0 || x >= r.w || y >= r.h {
        return;
    }
    let Some(cell) = c.get(r.x + x, r.y + y) else {
        return;
    };
    let had = QUADS.iter().position(|&q| q == cell.ch).unwrap_or(0);
    let bit = 1 << (qx.rem_euclid(2) + 2 * qy.rem_euclid(2));
    put_ch(c, r, x, y, QUADS[had | bit], col, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The banners of the example preset.
    const PAIRS: [(&str, &str); 4] = [
        ("SOFTWARE", "LIBRE"),
        ("GNU", "NO ES UNIX"),
        ("CUATRO", "LIBERTADES"),
        ("CODIGO", "CERRADO"),
    ];

    const QUADS: &str = " \u{2598}\u{259d}\u{2580}\u{2596}\u{258c}\u{259e}\u{259b}\u{2597}\u{259a}\u{2590}\u{259c}\u{2584}\u{2599}\u{259f}\u{2588}";

    fn look() -> Look {
        Look {
            letters: Color::Rgb(0, 255, 0),
            frame: Color::Rgb(0, 128, 0),
            caption: Color::Rgb(255, 255, 255),
            ink: Some(Color::Rgb(0, 20, 0)),
        }
    }

    /// The quarters a block character lights: top left 1, top right 2,
    /// bottom left 4, bottom right 8.
    fn quarters(ch: char) -> Option<u8> {
        QUADS.chars().position(|q| q == ch).map(|i| i as u8)
    }

    /// The box's height for `n` lines of letters (spec 7.5).
    fn box_h(size: Size, n: usize) -> i32 {
        2 + ((6 * n as i32 - 1) * size.pixel().1 + 1) / 2
    }

    /// The fewest lines `text` wraps to at `per` letters a line; None when a
    /// word is longer than that.
    fn least_lines(text: &str, per: i32) -> Option<usize> {
        let mut lines = 0;
        let mut len = 0;
        for word in text.split_whitespace() {
            let n = word.chars().count() as i32;
            if n > per {
                return None;
            }
            if lines > 0 && len + 1 + n <= per {
                len += 1 + n;
            } else {
                lines += 1;
                len = n;
            }
        }
        Some(lines)
    }

    /// The three checks of a size: no word too long, four lines at most, a
    /// box no higher than the pane.
    fn passes(size: Size, top: &str, bottom: &str, w: i32, h: i32) -> bool {
        let per = (w - 2) / size.letter();
        let (Some(a), Some(b)) = (least_lines(top, per), least_lines(bottom, per)) else {
            return false;
        };
        a + b <= 4 && box_h(size, a + b) <= h
    }

    #[test]
    fn every_glyph_is_its_own() {
        let chars: Vec<char> = ('A'..='Z')
            .chain(['\u{00d1}'])
            .chain('0'..='9')
            .chain(['!', '\u{00a1}', '?', '\u{00bf}', ':', '-', '.', '+'])
            .collect();
        for (i, &a) in chars.iter().enumerate() {
            assert!(has_glyph(a), "{a:?} has no glyph");
            for &b in &chars[i + 1..] {
                assert_ne!(glyph(a), glyph(b), "{a:?} and {b:?} look the same");
            }
        }
        assert_eq!(glyph('\u{00d1}'), [0b111, 0b000, 0b110, 0b101, 0b101]);
        assert_ne!(glyph('\u{00d1}'), glyph('N'));
        assert!(!has_glyph(' '));
        for ch in (0..=0x10ffff).filter_map(char::from_u32) {
            let old = widgets::glyph(ch);
            if old != [0; 5] {
                assert_eq!(glyph(ch), old, "{ch:?} changed");
            }
        }
    }

    #[test]
    fn the_table_of_cases() {
        use Size::*;
        type Case = (Size, usize, i32, i32);
        let table: [((i32, i32), [Case; 3]); 6] = [
            (
                (66, 19),
                [(X2, 2, 66, 13), (X2, 3, 42, 19), (Narrow, 2, 42, 13)],
            ),
            (
                (119, 28),
                [(X3, 2, 98, 19), (X3, 3, 62, 28), (X2, 2, 82, 13)],
            ),
            (
                (131, 44),
                [(X4, 2, 130, 24), (X4, 3, 82, 36), (X3, 2, 122, 19)],
            ),
            (
                (73, 21),
                [(X2, 2, 66, 13), (X2, 3, 42, 19), (Narrow, 2, 42, 13)],
            ),
            (
                (49, 15),
                [
                    (Narrow, 2, 34, 13),
                    (Narrow, 2, 42, 13),
                    (Narrow, 2, 42, 13),
                ],
            ),
            (
                (33, 9),
                [(Pocket, 2, 18, 8), (Pocket, 2, 22, 8), (Pocket, 2, 22, 8)],
            ),
        ];
        for ((w, h), cases) in table {
            for ((top, bottom), (size, n, bw, bh)) in PAIRS.iter().zip(cases) {
                let f = fit(top, bottom, w, h)
                    .unwrap_or_else(|| panic!("{top} / {bottom} at {w}x{h}: no fit"));
                assert_eq!(
                    (f.size, f.lines.len(), f.w, f.h),
                    (size, n, bw, bh),
                    "{top} / {bottom} at {w}x{h}: {:?}",
                    f.lines
                );
            }
        }
        let f = fit("GNU", "NO ES UNIX", 66, 19).unwrap();
        assert_eq!(f.lines, ["GNU", "NO ES", "UNIX"]);
    }

    #[test]
    fn the_biggest_that_fits() {
        for (top, bottom) in PAIRS {
            let words = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
            let (top_words, bottom_words) = (words(top), words(bottom));
            for w in 1..=150 {
                for h in 1..=60 {
                    let at = format!("{top} / {bottom} at {w}x{h}");
                    let Some(f) = fit(top, bottom, w, h) else {
                        for size in Size::BIG_FIRST {
                            assert!(!passes(size, top, bottom, w, h), "{at}: {size:?} fits");
                        }
                        continue;
                    };
                    assert!(f.w <= w && f.h <= h, "{at}: box {}x{}", f.w, f.h);
                    assert!((2..=4).contains(&f.lines.len()), "{at}: {:?}", f.lines);
                    let longest = f.lines.iter().map(|l| l.chars().count()).max();
                    assert_eq!(f.w, 2 + f.size.letter() * longest.unwrap() as i32, "{at}");
                    assert_eq!(f.h, box_h(f.size, f.lines.len()), "{at}");
                    // The lines, joined, are the words of `top` then of `bottom`.
                    let split = (1..f.lines.len())
                        .find(|&k| f.lines[..k].join(" ") == top_words)
                        .unwrap_or_else(|| panic!("{at}: {:?}", f.lines));
                    assert_eq!(f.lines[split..].join(" "), bottom_words, "{at}");
                    for line in &f.lines {
                        assert_eq!(line, &words(line), "{at}: {line:?}");
                    }
                    assert!(passes(f.size, top, bottom, w, h), "{at}");
                    for size in Size::BIG_FIRST.into_iter().filter(|&s| s > f.size) {
                        assert!(!passes(size, top, bottom, w, h), "{at}: {size:?} fits");
                    }
                }
            }
        }
    }

    #[test]
    fn below_pocket_there_is_no_fit() {
        assert_eq!(fit("SOFTWARE", "LIBRE", 33, 7), None);
        assert_eq!(fit("SOFTWARE", "LIBRE", 17, 9), None);
        assert_eq!(
            fit("SOFTWARE", "LIBRE", 18, 8).map(|f| f.size),
            Some(Size::Pocket)
        );
        assert_eq!(fit("", "LIBRE", 80, 40), None);
        assert_eq!(fit("SOFTWARE", " ", 80, 40), None);
    }

    #[test]
    fn the_letters_are_the_fonts_pixels() {
        let word = "A\u{00d1}O";
        for size in Size::BIG_FIRST {
            let (pw, ph) = size.pixel();
            let f = Fit {
                size,
                lines: vec![word.to_string(), word.to_string()],
                w: 2 + size.letter() * 3,
                h: box_h(size, 2),
            };
            let (cw, ch) = (f.w + 6, f.h + 6);
            let mut c = Canvas::new(cw as u16, ch as u16, Color::Reset);
            let r = Rect {
                x: 0,
                y: 0,
                w: cw,
                h: ch,
            };
            draw(&mut c, r, &f, ("", ""), &look());
            let (bx, by) = ((cw - f.w) / 2, (ch - f.h) / 2);
            // The quarter cells inside the frame, as drawn.
            let (qw, qh) = (2 * (f.w - 2), 2 * (f.h - 2));
            let mut lit = vec![vec![false; qw as usize]; qh as usize];
            for y in 0..f.h - 2 {
                for x in 0..f.w - 2 {
                    let cell = c.get(bx + 1 + x, by + 1 + y).unwrap();
                    let q = quarters(cell.ch)
                        .unwrap_or_else(|| panic!("{size:?}: {:?} in the letters", cell.ch));
                    for (bit, dx, dy) in [(1, 0, 0), (2, 1, 0), (4, 0, 1), (8, 1, 1)] {
                        lit[(2 * y + dy) as usize][(2 * x + dx) as usize] = q & bit != 0;
                    }
                }
            }
            // The top left pixel of the first letter is where `A` and `Ñ` begin.
            let ax = (0..qw)
                .find(|&x| (0..qh).any(|y| lit[y as usize][x as usize]))
                .unwrap();
            let ay = (0..qh)
                .find(|&y| (0..qw).any(|x| lit[y as usize][x as usize]))
                .unwrap();
            if size != Size::Pocket {
                assert_eq!(ax % 2, 0, "{size:?}: letters begin inside a column");
            }
            assert_eq!(ay, 0, "{size:?}: letters do not begin under the frame");
            let mut want = vec![vec![false; qw as usize]; qh as usize];
            for i in 0..2 {
                for (j, chr) in word.chars().enumerate() {
                    for (row, bits) in glyph(chr).iter().enumerate() {
                        for px in 0..3 {
                            if bits >> (2 - px) & 1 == 0 {
                                continue;
                            }
                            let x0 = ax + (4 * j as i32 + px) * pw;
                            let y0 = ay + (6 * i + row as i32) * ph;
                            for y in y0..y0 + ph {
                                for x in x0..x0 + pw {
                                    want[y as usize][x as usize] = true;
                                }
                            }
                        }
                    }
                }
            }
            assert!(lit == want, "{size:?}: the letters are not the font's");
        }
    }

    #[test]
    fn nothing_outside_the_pane() {
        let mut c = Canvas::new(90, 50, Color::Ansi(1));
        for (top, bottom) in PAIRS {
            for w in 1..=80 {
                for h in 1..=40 {
                    let Some(f) = fit(top, bottom, w, h) else {
                        continue;
                    };
                    c.clear();
                    let r = Rect { x: 3, y: 4, w, h };
                    draw(&mut c, r, &f, (" Software libre ", " caption "), &look());
                    for y in 0..c.h() as i32 {
                        for x in 0..c.w() as i32 {
                            let inside = x >= r.x && y >= r.y && x < r.x + w && y < r.y + h;
                            if !inside {
                                assert_eq!(
                                    c.get(x, y),
                                    Some(&crate::render::Cell {
                                        ch: ' ',
                                        fg: Color::Reset,
                                        bg: Color::Ansi(1),
                                        attr: 0
                                    }),
                                    "{top} / {bottom} at {w}x{h}: ({x}, {y})"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn letters_are_one_color() {
        let l = look();
        for strict in [false, true] {
            for size in Size::BIG_FIRST {
                let f = Fit {
                    size,
                    lines: vec!["\u{00bf}A\u{00d1}O?".into(), "C++ 3.0".into()],
                    w: 2 + size.letter() * 7,
                    h: box_h(size, 2),
                };
                let mut c = Canvas::new(f.w as u16 + 2, f.h as u16 + 2, Color::Reset);
                c.strict = strict;
                let r = Rect {
                    x: 0,
                    y: 0,
                    w: f.w + 2,
                    h: f.h + 2,
                };
                draw(&mut c, r, &f, (" tema ", " pie "), &l);
                let mut letters = 0;
                for y in 2..f.h {
                    for x in 2..f.w {
                        let cell = c.get(x, y).unwrap();
                        assert_eq!(cell.fg, l.letters, "{size:?} strict {strict}: ({x}, {y})");
                        letters += (cell.ch != ' ') as i32;
                    }
                }
                assert!(letters > 0, "{size:?}: no letters");
                for cell in c.cells() {
                    if strict {
                        assert_eq!(cell.bg, Color::Reset, "{size:?}: a background");
                    }
                }
                if !strict {
                    assert_eq!(c.get(1, 1).unwrap().bg, l.ink.unwrap());
                }
            }
        }
    }

    #[test]
    fn captions_fit_or_are_left_out() {
        let f = fit("SOFTWARE", "LIBRE", 33, 9).unwrap();
        assert_eq!(f.w, 18);
        let r = Rect {
            x: 0,
            y: 0,
            w: 33,
            h: 9,
        };
        let (bx, by) = ((33 - 18) / 2, (9 - 8) / 2);
        let row = |c: &Canvas, y: i32| -> String {
            (bx..bx + 18).map(|x| c.get(x, y).unwrap().ch).collect()
        };
        let l = look();
        // Sixteen characters, two of them not ASCII: on the frame, centered.
        let mut c = Canvas::new(33, 9, Color::Reset);
        draw(&mut c, r, &f, ("\u{00bf}SIXTEEN LETTERS", "A\u{00d1}O"), &l);
        assert_eq!(row(&c, by), "\u{2551}\u{00bf}SIXTEEN LETTERS\u{2551}");
        assert_eq!(
            row(&c, by + 7),
            "\u{2551}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}A\u{00d1}O\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2551}"
        );
        assert_eq!(c.get(bx + 1, by).unwrap().fg, l.caption);
        // Seventeen do not fit.
        let mut c = Canvas::new(33, 9, Color::Reset);
        draw(
            &mut c,
            r,
            &f,
            ("SEVENTEEN LETTERS", "SEVENTEEN LETTERZ"),
            &l,
        );
        let bare = format!("\u{2551}{}\u{2551}", "\u{2550}".repeat(16));
        assert_eq!(row(&c, by), bare);
        assert_eq!(row(&c, by + 7), bare);
        assert_eq!(c.get(bx + 1, by).unwrap().fg, l.frame);
    }
}
