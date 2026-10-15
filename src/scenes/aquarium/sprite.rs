//! Sprites the way Term::Animation builds them, so the art from asciiquarium
//! draws exactly as it does there:
//!
//! - a shape is a string (or one string per frame); one leading newline is
//!   dropped and lines are split like Perl's `split("\n")` (trailing empty
//!   lines vanish, leading ones stay);
//! - the transparent character (`?` unless the entity says otherwise) is never
//!   drawn, and neither is anything past the end of a line, but every other
//!   character is, spaces included, so a fish hides the seaweed behind it;
//! - with `auto_trans`, the whitespace that starts each line becomes transparent;
//! - the color mask is laid over the shape cell by cell: a letter picks a color
//!   (uppercase = bold), a space or a missing cell means the entity's default
//!   color, and a digit is a slot the spawner fills with a random color.

/// Mask code for "the entity's default color". Codes below it are pens.
pub const DEFAULT: u8 = 16;
/// Mask code of digit `d` is `DIGIT + d`.
pub const DIGIT: u8 = 16;

/// One drawn character of a frame, relative to the entity's position.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub dx: i16,
    pub dy: i16,
    pub ch: char,
    /// A pen (0..16: ANSI color index, +8 for bold), `DEFAULT`, or `DIGIT + d`.
    pub code: u8,
}

pub struct Sprite {
    /// Width and height as Term::Animation computes them: the longest line and
    /// the most lines over all frames. Used for collisions and for "offscreen".
    pub w: i32,
    pub h: i32,
    cells: Vec<Cell>,
    frames: Vec<(u32, u32)>,
}

/// The pen for a Term::Animation color letter.
pub fn pen(letter: char) -> Option<u8> {
    let base = match letter.to_ascii_lowercase() {
        'k' => 0,
        'r' => 1,
        'g' => 2,
        'y' => 3,
        'b' => 4,
        'm' => 5,
        'c' => 6,
        'w' => 7,
        _ => return None,
    };
    Some(if letter.is_ascii_uppercase() {
        base + 8
    } else {
        base
    })
}

/// Perl's `split("\n", $s)`: leading empty fields are kept, trailing ones dropped.
fn perl_lines(s: &str) -> Vec<&str> {
    let mut v: Vec<&str> = s.split('\n').collect();
    while v.last().is_some_and(|l| l.is_empty()) {
        v.pop();
    }
    v
}

/// Term::Animation's `_trans_fill_string`: leading whitespace becomes `trans`.
fn trans_fill(s: &str, trans: char) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for line in perl_lines(s) {
        let lead = line.len() - line.trim_start().len();
        out.extend(std::iter::repeat_n(trans, lead));
        out.push_str(&line[lead..]);
        out.push('\n');
    }
    out
}

/// Term::Animation's `_build_shape` for one frame: rows of characters.
fn rows(s: &str) -> Vec<Vec<char>> {
    let s = s.strip_prefix('\n').unwrap_or(s);
    perl_lines(s).iter().map(|l| l.chars().collect()).collect()
}

impl Sprite {
    /// Builds a sprite from its frames and color masks (fewer masks than
    /// frames are repeated, as Term::Animation does).
    pub fn new<S: AsRef<str>>(
        frames: &[S],
        masks: &[&str],
        auto_trans: bool,
        trans: char,
    ) -> Sprite {
        let shapes: Vec<Vec<Vec<char>>> = frames
            .iter()
            .map(|f| {
                let f = f.as_ref();
                if auto_trans {
                    rows(&trans_fill(f, trans))
                } else {
                    rows(f)
                }
            })
            .collect();
        let masks: Vec<Vec<Vec<char>>> = masks.iter().map(|m| rows(m)).collect();
        let w = shapes.iter().flatten().map(|r| r.len()).max().unwrap_or(0) as i32;
        let h = shapes.iter().map(|f| f.len()).max().unwrap_or(0) as i32;
        let mut cells = Vec::new();
        let mut ranges = Vec::with_capacity(shapes.len());
        for (fi, shape) in shapes.iter().enumerate() {
            let start = cells.len() as u32;
            let mask = (!masks.is_empty()).then(|| &masks[fi % masks.len()]);
            for (y, row) in shape.iter().enumerate() {
                for (x, &ch) in row.iter().enumerate() {
                    if ch == trans {
                        continue;
                    }
                    let m = mask.and_then(|m| m.get(y)).and_then(|r| r.get(x)).copied();
                    let code = match m {
                        None | Some(' ') => DEFAULT,
                        Some(d @ '1'..='9') => DIGIT + (d as u8 - b'0'),
                        // Term::Animation rejects other characters; they end up uncolored.
                        Some(c) => pen(c).unwrap_or(7),
                    };
                    cells.push(Cell {
                        dx: x as i16,
                        dy: y as i16,
                        ch,
                        code,
                    });
                }
            }
            ranges.push((start, cells.len() as u32));
        }
        Sprite {
            w,
            h,
            cells,
            frames: ranges,
        }
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// The cells of frame `f` (clamped to the last frame).
    #[inline]
    pub fn frame(&self, f: usize) -> &[Cell] {
        match self.frames.get(f).or(self.frames.last()) {
            Some(&(a, b)) => &self.cells[a as usize..b as usize],
            None => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perl_split_semantics() {
        assert_eq!(perl_lines("\n\na\n\n"), vec!["", "", "a"]);
        assert_eq!(rows("\nab\n c\n").len(), 2);
    }

    #[test]
    fn auto_trans_only_hides_leading_space() {
        let s = Sprite::new(&["\n  a b\n"], &[], true, '?');
        let xs: Vec<(i16, char)> = s.frame(0).iter().map(|c| (c.dx, c.ch)).collect();
        assert_eq!(xs, vec![(2, 'a'), (3, ' '), (4, 'b')]);
        assert_eq!((s.w, s.h), (5, 1));
    }

    #[test]
    fn masks_letters_digits_default() {
        let s = Sprite::new(&["\nabcd\n"], &["\nR1 \n"], false, '?');
        let codes: Vec<u8> = s.frame(0).iter().map(|c| c.code).collect();
        assert_eq!(codes, vec![9, DIGIT + 1, DEFAULT, DEFAULT]);
    }
}
