//! Papel picado: cut tissue-paper flags strung along a cord, one string or two
//! staggered. The engine is shared by the holidays that hang it (each brings
//! its own cut-outs and colors, a [`Papel`]).

use super::cord::{Hung, Piece};
use super::{LOWER, LOWER_LEVEL, Overlay, Themed, UPPER, put, put2, solid, wave};
use crate::color::Color;
use crate::render::Canvas;
use crate::theme::Theme;

/// Flag size in pixels (one pixel is half a cell).
pub(super) const FLAG_W: i32 = 7;
pub(super) const FLAG_H: i32 = 8;
/// Columns between two flags of the upper string when the lower one's flags
/// hang in between.
const GAP_TWO: i32 = 9;
/// A banderita: a little flag in three stripes, five pixels wide and three
/// tall.
const BANDERITA_W: i32 = 5;
/// Pixel row where the lower string's flags hang.
pub(super) const LOWER_TOP: i32 = 4;
/// About this many columns between two ties of the upper string.
const SWAG: i32 = 44;

/// A flag's cut-out, `X` for paper and `.` for a hole: a folded hem, five
/// rows of design, a border and the fringe.
pub(super) type Design = [&'static str; FLAG_H as usize];

/// A holiday's papel picado.
pub(super) struct Papel {
    /// The cut-outs.
    pub designs: &'static [Design],
    /// The paper under `classic`; any other theme gives its own colors.
    pub colors: [Color; 6],
    /// The cord, natural string.
    pub cord: Color,
    /// Columns between two flags of a string.
    pub gap: i32,
    /// Do the first three colors follow each other along the strings, as in
    /// the tricolor, instead of falling at random?
    pub ordered: bool,
    /// Banderitas (little flags in the first three colors) hang in the gaps
    /// between the flags of the upper string.
    pub banderitas: bool,
    /// How the paper follows the theme.
    pub themed: Themed,
}

pub(super) struct Flag {
    /// Left column.
    pub x: i32,
    /// Top pixel row.
    pub top: i32,
    pub design: u8,
    pub color: u8,
    /// A little of its own in the way it sways.
    pub phase: f32,
}

impl Overlay {
    /// Hangs the strings across the width: one, or two staggered.
    pub(super) fn hang_papel(&mut self, papel: &Papel, salt: u64) {
        let w = self.w as i32;
        let mut rng = fastrand::Rng::with_seed(
            super::SEED ^ salt ^ (w as u64) << 32 ^ self.level.to_bits() as u64,
        );
        let two = self.level >= LOWER_LEVEL;
        self.flags.clear();
        self.ties.clear();
        self.hung.clear();

        // The upper string, centered, its flags evenly spaced; wider apart
        // when the lower string's hang between them.
        let gap = if two { GAP_TWO } else { papel.gap };
        let pitch = FLAG_W + gap;
        let n = ((w + gap) / pitch).max(0);
        let x0 = (w - (n * pitch - gap)) / 2;
        // The lower string, shorter, has its flags in the upper one's gaps.
        let inset = (w / 10).max(4);
        self.lower = (inset, w - 1 - inset);
        let mut lower = Vec::new();
        // Neighbors along the zigzag never share a design or a color.
        let (mut designs, mut colors) = ([u8::MAX; 2], [u8::MAX; 2]);
        let mut add = |flags: &mut Vec<Flag>, x: i32, top: i32, nth: usize| {
            let design = other(&mut rng, designs, papel.designs.len());
            let color = if papel.ordered {
                (nth % 3) as u8
            } else {
                other(&mut rng, colors, papel.colors.len())
            };
            designs = [designs[1], design];
            colors = [colors[1], color];
            flags.push(Flag {
                x,
                top,
                design,
                color,
                phase: rng.f32() * std::f32::consts::TAU,
            });
        };
        for i in 0..n {
            let x = x0 + i * pitch;
            add(&mut self.flags, x, 0, i as usize);
            if papel.banderitas && i + 1 < n {
                self.hung.push(Hung {
                    x: x + FLAG_W + (gap - BANDERITA_W) / 2,
                    piece: Piece::Banderita,
                    color: 0,
                    len: 0,
                    phase: 1.7 * i as f32,
                });
            }
            let x = x + FLAG_W + (gap - FLAG_W) / 2;
            if two && x >= self.lower.0 && x + FLAG_W <= self.lower.1 {
                // Neither of its neighbors' colors, in the tricolor.
                add(&mut lower, x, LOWER_TOP, i as usize + 2);
            }
        }
        self.upper = self.flags.len();
        self.flags.append(&mut lower);
        self.cover.clear();
        self.cover.resize(w.max(0) as usize, 0);
        for (i, f) in self.flags.iter().enumerate() {
            let which = if i < self.upper { UPPER } else { LOWER };
            for x in f.x..f.x + FLAG_W {
                if let Some(m) = usize::try_from(x).ok().and_then(|x| self.cover.get_mut(x)) {
                    *m |= which;
                }
            }
        }

        // Tied at both ends and in gaps between flags, about a swag apart.
        self.ties.push(0);
        let swags = ((w as f32 / SWAG as f32).round() as i32).max(1);
        for k in 1..swags {
            let want = k * w / swags;
            let i = ((want - x0 + gap) as f32 / pitch as f32).round() as i32;
            let x = x0 + i * pitch - gap / 2 - 1;
            if x > self.ties[self.ties.len() - 1] + 2 && x < w - 3 {
                self.ties.push(x);
            }
        }
        self.ties.push((w - 1).max(0));
    }

    /// Draws the strings, the lower one behind the upper.
    pub(super) fn draw_papel(&self, c: &mut Canvas, theme: &Theme, papel: &Papel) {
        let mut colors = papel.colors;
        let mut cord = papel.cord;
        if !theme.native {
            for (i, slot) in colors.iter_mut().enumerate() {
                *slot = papel.themed.color(theme, i, *slot);
            }
            cord = solid(Some(&theme.fg), solid(theme.ramp.last(), papel.cord)).scale(0.85);
        }
        if self.upper < self.flags.len() {
            self.draw_lower_cord(c, cord);
            for f in &self.flags[self.upper..] {
                self.draw_flag(c, f, papel, &colors);
            }
        }
        self.draw_upper_cord(c, cord);
        for h in &self.hung {
            if h.piece == Piece::Banderita {
                self.draw_banderita(c, h, &colors);
            }
        }
        for f in &self.flags[..self.upper] {
            self.draw_flag(c, f, papel, &colors);
        }
    }

    /// The upper cord's pixel row at column `x`: on the top row at the ties,
    /// one pixel lower between them.
    pub(super) fn sag(&self, x: i32) -> i32 {
        self.sag_by(x, 1.0)
    }

    /// Does a flag of the upper string (`UPPER`), the lower one (`LOWER`) or
    /// either hang over column `x`?
    fn covered(&self, x: i32, which: u8) -> bool {
        usize::try_from(x)
            .ok()
            .and_then(|x| self.cover.get(x))
            .is_some_and(|m| m & which != 0)
    }

    /// The cord runs through the flags' folds, out of sight.
    fn draw_upper_cord(&self, c: &mut Canvas, cord: Color) {
        for x in 0..self.w as i32 {
            if !self.covered(x, UPPER) {
                super::put(c, x, self.sag(x), cord);
            }
        }
        // A knot at each tie, its ends hanging.
        for &x in &self.ties {
            put2(c, x, 0, Some(cord.scale(1.15)), Some(cord.scale(0.8)));
        }
    }

    /// The lower cord hangs from the upper one at both ends.
    fn draw_lower_cord(&self, c: &mut Canvas, cord: Color) {
        let (a, b) = self.lower;
        if b <= a {
            return;
        }
        let cord = cord.scale(0.85);
        for x in a..=b {
            if !self.covered(x, UPPER | LOWER) {
                let u = (x - a) as f32 / (b - a) as f32;
                super::put(c, x, LOWER_TOP + (4.0 * u * (1.0 - u) + 0.35) as i32, cord);
            }
        }
        for d in 1..LOWER_TOP {
            for x in [a - d, b + d] {
                if !self.covered(x, UPPER) {
                    super::put(c, x, LOWER_TOP - d, cord);
                }
            }
        }
    }

    /// How far the breeze swings the bottom of the flag at column `x`, in
    /// pixels: a slow gust travels along the string, each flag a little late.
    pub(super) fn sway(&self, x: i32, phase: f32) -> f32 {
        let x = x as f32;
        let gust = (0.5 + 0.5 * (wave(self.t, 0.23, 0.0) - 0.018 * x).sin()).powi(2);
        let amp = 0.45 + 0.9 * gust;
        amp * (wave(self.t, 1.9, 0.35 * phase) - 0.11 * x).sin()
    }

    /// A banderita on the cord between two flags: stripes of the first,
    /// second and third color, the bottom swaying in the breeze.
    fn draw_banderita(&self, c: &mut Canvas, h: &Hung, colors: &[Color; 6]) {
        let s = self.sway(h.x, h.phase) * 0.8;
        let top = self.sag(h.x + BANDERITA_W / 2);
        for r in 0..3 {
            let dx = (s * r as f32 / 2.0).round() as i32;
            for i in 0..BANDERITA_W {
                let col = colors[[0, 0, 1, 2, 2][i as usize % 5]];
                put(
                    c,
                    h.x + i + dx,
                    top + r,
                    col.scale(if r == 0 { 0.8 } else { 1.0 }),
                );
            }
        }
    }

    fn draw_flag(&self, c: &mut Canvas, f: &Flag, papel: &Papel, colors: &[Color; 6]) {
        let s = self.sway(f.x, f.phase);
        // Turned away from the light as it swings (in a few steps, so a
        // flag is not redrawn every frame).
        let turned = (s.abs() * 2.5).round() / 2.5;
        let base = colors[f.color as usize % colors.len()].scale(1.0 - 0.1 * turned);
        let fold = base.scale(0.78);
        let design = &papel.designs[f.design as usize % papel.designs.len()];
        // The bottom swings furthest.
        let mut shift = [0; FLAG_H as usize];
        for (r, dx) in shift.iter_mut().enumerate() {
            *dx = (s * r as f32 / (FLAG_H - 1) as f32).round() as i32;
        }
        let pixel = |x: i32, r: i32| -> Option<Color> {
            let r = usize::try_from(r).ok()?;
            let i = x - f.x - shift.get(r)?;
            match design.get(r)?.as_bytes().get(usize::try_from(i).ok()?)? {
                b'X' if r == 0 => Some(fold),
                b'X' => Some(base),
                _ => None,
            }
        };
        // Cell by cell (the flags hang from even pixel rows).
        for r in (0..FLAG_H).step_by(2) {
            for x in f.x - 2..f.x + FLAG_W + 2 {
                put2(c, x, (f.top + r) / 2, pixel(x, r), pixel(x, r + 1));
            }
        }
    }
}

/// A random index below `n` (at least 3) that is not one of `avoid`.
pub(super) fn other(rng: &mut fastrand::Rng, avoid: [u8; 2], n: usize) -> u8 {
    let n = n.clamp(1, 255) as u8;
    let mut i = rng.u8(..n);
    for _ in 0..n {
        if !avoid.contains(&i) {
            break;
        }
        i = (i + 1) % n;
    }
    i
}
