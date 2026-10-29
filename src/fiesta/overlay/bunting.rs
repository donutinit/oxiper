//! Banderines: bunting, a string of little triangular flags hanging point
//! down from a cord that sags between its ties, one string or two staggered.

use super::cord::{Hung, Piece};
use super::papel::other;
use super::{LOWER_LEVEL, Overlay, Themed, put, solid};
use crate::color::Color;
use crate::render::Canvas;
use crate::theme::Theme;

/// A pennant: seven pixels wide and four tall, a column between two of them.
const W: i32 = 7;
const H: i32 = 4;
const PITCH: i32 = W + 1;
/// How deep the cord sags in the middle of a swag, and about how many columns
/// the ties are apart.
const DEPTH: f32 = 2.0;
const SWAG: i32 = 36;
/// Pixel row where the lower string's cord hangs.
const LOWER_TOP: i32 = 5;

/// A holiday's bunting.
pub(super) struct Bunting {
    /// The pennants under `classic`.
    pub colors: [Color; 6],
    /// Do the first three colors follow each other along the string (the
    /// tricolor) instead of falling at random?
    pub ordered: bool,
    /// The cord, natural string.
    pub cord: Color,
    /// A second string, lower and shorter, once the holiday has built up.
    pub two: bool,
    /// How the pennants follow the theme.
    pub themed: Themed,
}

impl Overlay {
    /// Hangs the pennants from the cord: one string, or two staggered.
    pub(super) fn hang_bunting(&mut self, bunting: &Bunting, salt: u64) {
        let w = self.w as i32;
        let mut rng = fastrand::Rng::with_seed(
            super::SEED ^ salt ^ (w as u64) << 32 ^ self.level.to_bits() as u64,
        );
        self.tie(SWAG);
        self.hung.clear();
        let two = bunting.two && self.level >= LOWER_LEVEL;
        let inset = (w / 10).max(4);
        self.lower = (inset, w - 1 - inset);

        let n = ((w + 1) / PITCH).max(0);
        let x0 = (w - (n * PITCH - 1)) / 2;
        for string in 0..1 + usize::from(two) {
            // Neighbors never share a color (but in order, they follow it).
            let mut avoid = [u8::MAX; 2];
            for i in 0..n {
                // The lower string hangs in the upper one's gaps.
                let x = x0 + i * PITCH + string as i32 * PITCH / 2;
                if string == 1 && (x < self.lower.0 || x + W > self.lower.1) {
                    continue;
                }
                let color = if bunting.ordered {
                    ((i as usize + string) % 3) as u8
                } else {
                    other(&mut rng, avoid, bunting.colors.len())
                };
                avoid = [avoid[1], color];
                self.hung.push(Hung {
                    x,
                    piece: Piece::Pennant,
                    color,
                    len: string as u8,
                    phase: rng.f32() * std::f32::consts::TAU,
                });
            }
        }
    }

    /// Draws the strings, the lower one behind the upper.
    pub(super) fn draw_bunting(&self, c: &mut Canvas, theme: &Theme, bunting: &Bunting) {
        let mut colors = bunting.colors;
        let mut cord = bunting.cord;
        if !theme.native {
            for (i, slot) in colors.iter_mut().enumerate() {
                *slot = bunting.themed.color(theme, i, *slot);
            }
            cord = solid(Some(&theme.fg), bunting.cord).scale(0.85);
        }
        let lower = self.hung.iter().any(|h| h.len == 1);
        if lower {
            self.draw_lower_string(c, cord);
            self.draw_pennants(c, &colors, 1);
        }
        self.draw_cord(c, cord, DEPTH);
        self.draw_pennants(c, &colors, 0);
    }

    fn draw_pennants(&self, c: &mut Canvas, colors: &[Color; 6], string: u8) {
        for h in self.hung.iter().filter(|h| h.piece == Piece::Pennant) {
            if h.len == string {
                self.draw_pennant(c, h, colors[h.color as usize % colors.len()]);
            }
        }
    }

    /// The lower cord, sagging between its ends, which hang from the upper
    /// one.
    fn draw_lower_string(&self, c: &mut Canvas, cord: Color) {
        let (a, b) = self.lower;
        if b <= a {
            return;
        }
        let cord = cord.scale(0.85);
        for x in a..=b {
            put(c, x, self.lower_sag(x), cord);
        }
        for d in 1..LOWER_TOP {
            for x in [a - d, b + d] {
                put(c, x, LOWER_TOP - d, cord);
            }
        }
    }

    /// The lower cord's pixel row at column `x`.
    fn lower_sag(&self, x: i32) -> i32 {
        let (a, b) = self.lower;
        let u = (x - a) as f32 / (b - a).max(1) as f32;
        LOWER_TOP + (DEPTH * 4.0 * u * (1.0 - u) + 0.35) as i32
    }

    /// A pennant hanging from its cord, its point swinging in the breeze.
    fn draw_pennant(&self, c: &mut Canvas, h: &Hung, color: Color) {
        let s = self.sway(h.x, h.phase) * 0.7;
        let middle = h.x + W / 2;
        let top = if h.len == 0 {
            self.sag_by(middle, DEPTH)
        } else {
            self.lower_sag(middle)
        };
        // Darker as it turns away from the light.
        let turned = (s.abs() * 2.5).round() / 2.5;
        let base = color.scale(1.0 - 0.1 * turned);
        for r in 0..H {
            let dx = (s * r as f32 / (H - 1) as f32).round() as i32;
            // The top edge is the fold over the cord.
            let shade = if r == 0 { base.scale(0.8) } else { base };
            for i in r..W - r {
                put(c, h.x + i + dx, top + r, shade);
            }
        }
    }
}
