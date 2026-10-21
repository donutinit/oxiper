//! What a holiday puts over every scene, under the easter eggs.

use super::Fiesta;
use crate::render::Canvas;
use crate::theme::Theme;

pub struct Overlay {
    w: u16,
    h: u16,
    /// Real seconds since it started.
    t: f32,
}

impl Overlay {
    pub fn new(w: u16, h: u16) -> Overlay {
        Overlay { w, h, t: 0.0 }
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.w = w;
        self.h = h;
    }

    /// Advances by `dt` real seconds.
    pub fn update(&mut self, dt: f32, _fiesta: Fiesta) {
        if dt.is_finite() {
            self.t += dt.clamp(0.0, 5.0);
        }
    }

    /// Draws over the scene. Nothing on an ordinary day.
    pub fn draw(&self, _c: &mut Canvas, _theme: &Theme, fiesta: Fiesta) {
        if !fiesta.on() {
            return;
        }
        let _ = (self.w, self.h, self.t);
    }
}
