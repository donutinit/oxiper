//! Flecks: little things that fall over the whole screen, tumbling and
//! swaying (cempasúchil petals, confetti). Each lays only its own pixels, and
//! the few of them keep the scene visible.

use super::{Overlay, put, solid, wave};
use crate::color::{Color, rgb};
use crate::render::Canvas;
use crate::theme::Theme;

/// What falls.
pub(super) struct Flecks {
    /// The two faces of each color, the one in the light and the one in
    /// shade.
    pub tones: &'static [(Color, Color)],
    /// How many in 256 are of the last tone (the others share the first), or
    /// all of them evenly when 0.
    pub share: u8,
    /// Cells per fleck at the holiday's height.
    pub cells: i32,
    /// Do the tones follow the theme's colors (in order) under any theme but
    /// `classic`?
    pub themed: bool,
}

/// A color and the shade of its other face.
pub(super) const fn tone(hex: u32) -> (Color, Color) {
    const fn shade(hex: u32, shift: u32) -> u32 {
        (((hex >> shift) & 0xff) * 6 / 10) << shift
    }
    (
        rgb(hex),
        rgb(shade(hex, 16) | shade(hex, 8) | shade(hex, 0)),
    )
}

/// The most tones a holiday's flecks can have.
const TONES: usize = 8;

pub(super) struct Fleck {
    /// Column the sway centers on, and the pixel row.
    pub x: f32,
    pub y: f32,
    /// Pixels per second down, at the bottom of the swing.
    pub fall: f32,
    /// Columns either side, and radians per second.
    pub sway: f32,
    pub freq: f32,
    pub phase: f32,
    /// Radians per second it tumbles at.
    pub spin: f32,
    pub tone: u8,
}

impl Overlay {
    /// Scatters the flecks over the screen, as many as the holiday's height
    /// brings.
    pub(super) fn scatter(&mut self, flecks: &Flecks, salt: u64) {
        let (w, h) = (self.w as i32, self.h as i32);
        self.rng = fastrand::Rng::with_seed(super::SEED ^ salt ^ ((w as u64) << 16 | h as u64));
        let most = (w * h / flecks.cells).max(1) as usize;
        self.petals.clear();
        for _ in 0..most {
            let mut p = fleck(&mut self.rng, flecks, w);
            p.y = self.rng.f32() * (h * 2) as f32;
            self.petals.push(p);
        }
    }

    /// Lets the flecks fall for `dt` seconds.
    pub(super) fn fall(&mut self, dt: f32, flecks: &Flecks) {
        let (w, ph) = (self.w as i32, self.h as f32 * 2.0);
        for i in 0..self.petals.len() {
            let p = &mut self.petals[i];
            // Like a leaf: slowest at the ends of its swing.
            let swing = wave(self.t, p.freq, p.phase).cos().abs();
            p.y += p.fall * (0.45 + 0.55 * swing) * dt;
            if p.y > ph + 1.0 {
                let mut q = fleck(&mut self.rng, flecks, w);
                q.y = -1.0 - self.rng.f32() * 6.0;
                self.petals[i] = q;
            }
        }
    }

    /// Draws as many flecks as the holiday has reached.
    pub(super) fn draw_flecks(&self, c: &mut Canvas, theme: &Theme, flecks: &Flecks, level: f32) {
        let level = level.clamp(0.0, 1.0);
        let n = ((self.petals.len() as f32 * (0.3 + 0.7 * level)).round() as usize)
            .clamp(1, self.petals.len().max(1));
        let mut tones = [(Color::Reset, Color::Reset); TONES];
        let k = flecks.tones.len().clamp(1, TONES);
        tones[..k].copy_from_slice(&flecks.tones[..k]);
        if flecks.themed && !theme.native {
            for (i, t) in tones[..k].iter_mut().enumerate() {
                let c = solid(theme.colors.get(i % theme.colors.len().max(1)), t.0);
                *t = (c, c.scale(0.6));
            }
        }
        for p in self.petals.iter().take(n) {
            self.draw_fleck(c, &tones[..k], p);
        }
    }

    fn draw_fleck(&self, c: &mut Canvas, tones: &[(Color, Color)], p: &Fleck) {
        let x = p.x + p.sway * wave(self.t, p.freq, p.phase).sin();
        let turn = wave(self.t, p.spin, p.phase);
        let (light, dark) = tones[p.tone as usize % tones.len()];
        let (px, py) = (x.floor() as i32, p.y.floor() as i32);
        // Face up, tipping over, edge on, face down.
        match (turn.rem_euclid(std::f32::consts::TAU) * 4.0 / std::f32::consts::TAU) as u8 {
            0 => {
                put(c, px, py, light);
                put(c, px + 1, py, light.lerp(dark, 0.25));
            }
            1 => {
                put(c, px, py, light);
                put(c, px, py + 1, dark);
            }
            2 => put(c, px, py, light.lerp(dark, 0.5)),
            _ => {
                put(c, px, py, dark);
                put(c, px + 1, py, dark.lerp(light, 0.2));
            }
        }
    }
}

/// A new fleck somewhere across `w` columns.
fn fleck(rng: &mut fastrand::Rng, flecks: &Flecks, w: i32) -> Fleck {
    Fleck {
        x: rng.f32() * w.max(1) as f32,
        y: 0.0,
        fall: 3.0 + rng.f32() * 3.5,
        sway: 1.0 + rng.f32() * 2.5,
        freq: 0.7 + rng.f32() * 0.9,
        phase: rng.f32() * std::f32::consts::TAU,
        spin: 2.5 + rng.f32() * 4.0,
        tone: if flecks.share > 0 {
            u8::from(rng.u8(..) < flecks.share)
        } else {
            rng.u8(..flecks.tones.len().clamp(1, TONES) as u8)
        },
    }
}
