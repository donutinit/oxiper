//! A cord strung across the top of the screen, tied here and there and
//! sagging between its ties, with things hanging from it.

use super::{Overlay, put, put2};
use crate::color::Color;
use crate::render::Canvas;

/// What hangs from a cord.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Piece {
    /// A foquito of a string of series lights.
    Bulb,
    /// An esfera: a glass ball with its cap, hanging by a thread.
    Esfera,
    /// A swag of heno (Spanish moss) draped along the cord.
    Heno,
}

pub(super) struct Hung {
    /// Column: the bulb's, the esfera's left edge, the swag's first.
    pub x: i32,
    pub piece: Piece,
    /// Which color of the piece's palette.
    pub color: u8,
    /// The thread of an esfera in pixels, the width of a swag, the place of
    /// a bulb along the string.
    pub len: u8,
    /// A little of its own in the way it moves.
    pub phase: f32,
}

impl Overlay {
    /// Ties the cord at both ends and evenly between them, a swag apart (or
    /// a little more).
    pub(super) fn tie(&mut self, swag: i32) {
        let w = self.w as i32;
        self.ties.clear();
        self.ties.push(0);
        let swags = ((w as f32 / swag.max(1) as f32).round() as i32).max(1);
        for k in 1..swags {
            self.ties.push(k * w / swags);
        }
        self.ties.push((w - 1).max(0));
    }

    /// The cord's pixel row at column `x`: on the top row at the ties,
    /// `depth` pixels lower in the middle of a swag.
    pub(super) fn sag_by(&self, x: i32, depth: f32) -> i32 {
        let k = self.ties.partition_point(|&t| t <= x);
        if k == 0 || k >= self.ties.len() {
            return 0;
        }
        let (a, b) = (self.ties[k - 1], self.ties[k]);
        let u = (x - a) as f32 / (b - a).max(1) as f32;
        (depth * 4.0 * u * (1.0 - u) + 0.35) as i32
    }

    /// The cord with a knot at each tie, its ends hanging.
    pub(super) fn draw_cord(&self, c: &mut Canvas, cord: Color, depth: f32) {
        for x in 0..self.w as i32 {
            put(c, x, self.sag_by(x, depth), cord);
        }
        for &x in &self.ties {
            put2(c, x, 0, Some(cord.scale(1.15)), Some(cord.scale(0.8)));
        }
    }
}
