//! Serpentinas: paper streamers thrown over the party. Each falls as a little
//! roll with its ribbon unwinding behind it, curling as it goes.

use super::{Overlay, put, solid, wave};
use crate::color::Color;
use crate::render::Canvas;
use crate::theme::Theme;

/// What is thrown.
pub(super) struct Ribbons {
    /// The paper under `classic`; any other theme gives its own colors.
    pub colors: &'static [Color],
    /// Cells per serpentina at the holiday's height.
    pub cells: i32,
}

pub(super) struct Ribbon {
    /// Column of the roll, and pixel row.
    pub x: f32,
    pub y: f32,
    /// Pixels per second down.
    pub fall: f32,
    /// How far the ribbon swings either side, and how tight its curls are
    /// (radians per pixel of ribbon).
    pub amp: f32,
    pub curl: f32,
    pub phase: f32,
    pub color: u8,
    /// Length of the ribbon in pixels.
    pub len: u8,
}

/// The most colors a serpentina can be.
const COLORS: usize = 8;

impl Overlay {
    /// Throws the serpentinas over the screen.
    pub(super) fn scatter_ribbons(&mut self, ribbons: &Ribbons, salt: u64) {
        let (w, h) = (self.w as i32, self.h as i32);
        let mut rng = fastrand::Rng::with_seed(super::SEED ^ salt ^ 0x5e57 ^ (w as u64) << 20);
        let most = (w * h / ribbons.cells.max(1)).max(1) as usize;
        self.ribbons.clear();
        for _ in 0..most {
            let mut r = ribbon(&mut rng, ribbons, w);
            r.y = rng.f32() * (h * 2) as f32;
            self.ribbons.push(r);
        }
    }

    /// Lets the serpentinas fall for `dt` seconds.
    pub(super) fn unroll(&mut self, dt: f32, ribbons: &Ribbons) {
        let (w, ph) = (self.w as i32, self.h as f32 * 2.0);
        for i in 0..self.ribbons.len() {
            let r = &mut self.ribbons[i];
            r.y += r.fall * dt;
            // Gone once its tail is off the screen.
            if r.y - r.len as f32 > ph + 1.0 {
                let mut q = ribbon(&mut self.rng, ribbons, w);
                q.y = -self.rng.f32() * 8.0;
                self.ribbons[i] = q;
            }
        }
    }

    /// Draws as many serpentinas as the holiday has reached.
    pub(super) fn draw_ribbons(
        &self,
        c: &mut Canvas,
        theme: &Theme,
        ribbons: &Ribbons,
        level: f32,
    ) {
        let level = level.clamp(0.0, 1.0);
        let n = ((self.ribbons.len() as f32 * (0.4 + 0.6 * level)).round() as usize)
            .min(self.ribbons.len());
        let mut colors = [Color::Reset; COLORS];
        let k = ribbons.colors.len().clamp(1, COLORS);
        colors[..k].copy_from_slice(&ribbons.colors[..k]);
        if !theme.native {
            for (i, slot) in colors[..k].iter_mut().enumerate() {
                *slot = solid(theme.colors.get(i % theme.colors.len().max(1)), *slot);
            }
        }
        for r in &self.ribbons[..n] {
            self.draw_ribbon(c, r, colors[r.color as usize % k]);
        }
    }

    /// The roll at the head, the ribbon fading as it trails up behind it,
    /// wriggling a little as it falls.
    fn draw_ribbon(&self, c: &mut Canvas, r: &Ribbon, color: Color) {
        let head = r.y.floor() as i32;
        let wriggle = wave(self.t, 1.7, r.phase);
        let mut prev = None;
        for i in 0..r.len as i32 {
            let curl = r.curl * i as f32 + wriggle;
            let x = (r.x + r.amp * curl.sin()).round() as i32;
            let col = if i < 2 {
                color.scale(1.15)
            } else {
                color.scale(1.0 - 0.5 * i as f32 / r.len as f32)
            };
            let y = head - i;
            put(c, x, y, col);
            // Where the curl is steep, the ribbon runs sideways.
            if let Some(p) = prev {
                let (a, b) = if p < x {
                    (p + 1, x - 1)
                } else {
                    (x + 1, p - 1)
                };
                for fx in a..=b {
                    put(c, fx, y, col);
                }
            }
            prev = Some(x);
        }
    }
}

/// A new serpentina somewhere across `w` columns.
fn ribbon(rng: &mut fastrand::Rng, ribbons: &Ribbons, w: i32) -> Ribbon {
    Ribbon {
        x: rng.f32() * w.max(1) as f32,
        y: 0.0,
        fall: 4.0 + rng.f32() * 3.0,
        amp: 1.2 + rng.f32() * 1.6,
        curl: 0.45 + rng.f32() * 0.35,
        phase: rng.f32() * std::f32::consts::TAU,
        color: rng.u8(..ribbons.colors.len().clamp(1, COLORS) as u8),
        len: 10 + rng.u8(..8),
    }
}
