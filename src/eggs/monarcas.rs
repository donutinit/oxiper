//! A swarm of monarch butterflies flows across the screen during Día de
//! Muertos, when they come back to Michoacán and are said to carry the souls.
//!
//! Original oxiper pixel art: each monarch is drawn from above, orange wings
//! veined and edged in black with white spots, at one of three sizes (the
//! small ones are farther away and paler). Each flaps at its own pace, a few
//! beats and then a glide, and rises a little on every downstroke; together
//! they drift across as a loose cloud that swells and wanders, the stragglers
//! weaving in and out. The swarm scales with the screen and crosses within
//! 30 s, from either side.

use super::muertos::Palette;
use super::{Egg, Info, sprite};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "monarcas",
    start,
    fiesta: Kind::Muertos,
    when: super::any_day,
};

type Frames = [&'static [&'static str]; 3];

/// Open, raised and closed, for each size from far to near.
const SMALL: Frames = [
    &["kO.Ok", "kOkOk", ".dkd."],
    &[".O.O.", ".kOk.", "..k.."],
    &["..o..", "..o..", "..k.."],
];
const MEDIUM: Frames = [
    &["kWk.kWk", "kOOkOOk", ".kOkOk.", "..dkd.."],
    &[".k...k.", ".kOkOk.", "..OkO..", "...k..."],
    &["...o...", "...o...", "...k...", "......."],
];
const BIG: Frames = [
    &[
        "kWk...kWk",
        "kOOk.kOOk",
        "WOoOkOoOW",
        ".kOOkOOk.",
        ".kdOkOdk.",
        "..kW.Wk..",
    ],
    &[
        ".W.....W.",
        ".kOk.kOk.",
        "..kOkOk..",
        "..kdkdk..",
        "...kWk...",
        "....k....",
    ],
    &[
        "....W....",
        "...kok...",
        "...kok...",
        "...kOk...",
        "....k....",
        "....k....",
    ],
];
const SIZES: [&Frames; 3] = [&SMALL, &MEDIUM, &BIG];
/// One wingbeat: open, raised, closed, raised.
const BEAT: [usize; 4] = [0, 1, 2, 1];

/// Seconds the swarm takes to cross, give or take.
const CROSSING: f32 = 19.0;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let (wf, hf) = (w as f32, h as f32 * 2.0);
    let n = ((w as usize * h as usize) / 110).clamp(8, 48);
    // The cloud, in pixels: wider than tall, at most a third of the screen across.
    let (rx, ry) = ((wf * 0.24).clamp(6.0, 56.0), (hf * 0.28).clamp(3.0, 30.0));
    let mut flock: Vec<Monarch> = (0..n)
        .map(|_| {
            // Mostly near the middle of the cloud, some strays at its edges.
            let (a, r) = (rng.f32() * TAU, rng.f32().sqrt() * (0.6 + 0.6 * rng.f32()));
            Monarch {
                ox: a.cos() * r * rx,
                oy: a.sin() * r * ry,
                pace: 0.85 + 0.3 * rng.f32(),
                size: match rng.u8(0..10) {
                    0..4 => 0,
                    4..8 => 1,
                    _ => 2,
                },
                flap: 4.0 + 3.0 * rng.f32(),
                phase: rng.f32() * 8.0,
                wander: [rng.f32() * TAU, rng.f32() * TAU, 0.4 + rng.f32() * 0.5],
            }
        })
        .collect();
    // Far ones first, so the near ones cross over them.
    flock.sort_by_key(|m| m.size);
    // Even the slowest is across in time.
    let span = wf + 4.0 * rx + 40.0;
    Box::new(Swarm {
        t: 0.0,
        flock,
        speed: span / (0.85 * CROSSING * (0.9 + 0.2 * rng.f32())),
        left: rng.bool(),
        rx,
        y0: hf * (0.3 + 0.3 * rng.f32()),
        lilt: rng.f32() * TAU,
        w: wf,
        h: hf,
    })
}

struct Monarch {
    /// Place in the cloud, in pixels from its center.
    ox: f32,
    oy: f32,
    /// How much faster or slower than the cloud it flies.
    pace: f32,
    /// 0 small (far), 1 medium, 2 big (near).
    size: u8,
    /// Wingbeats per second, and where in its flapping it starts.
    flap: f32,
    phase: f32,
    /// Phases and reach of its weaving about.
    wander: [f32; 3],
}

struct Swarm {
    t: f32,
    flock: Vec<Monarch>,
    /// Pixels per second.
    speed: f32,
    /// Flying right to left?
    left: bool,
    rx: f32,
    /// The height the cloud flies at, in pixels, and the phase of its rise and fall.
    y0: f32,
    lilt: f32,
    w: f32,
    h: f32,
}

impl Swarm {
    /// How far the cloud's center has come from where it started, off screen.
    fn gone(&self, m: &Monarch) -> f32 {
        self.t * self.speed * m.pace - self.rx - 12.0
    }

    /// Where a monarch is now (top-left pixel of its sprite) and which frame it shows.
    fn place(&self, m: &Monarch) -> (i32, i32, usize) {
        let t = self.t;
        let [p, q, reach] = m.wander;
        let along = self.gone(m) + m.ox + (t * 0.9 + p).sin() * 4.0 * reach;
        let x = if self.left { self.w - along } else { along };
        // The cloud rises and dips; each monarch weaves within it.
        let y = self.y0
            + (t * 0.35 + self.lilt).sin() * self.h * 0.12
            + m.oy * (1.0 + 0.25 * (t * 0.5 + q).sin())
            + (t * 1.3 + q).sin() * 3.0 * reach;
        // A few beats, then a glide with the wings open.
        let s = t * m.flap + m.phase;
        let gliding = (s / 5.0).fract() > 0.55;
        let frame = if gliding {
            0
        } else {
            BEAT[(s.fract() * 4.0) as usize % 4]
        };
        // Lifted a little on the downstroke.
        let lift = if gliding { 0.0 } else { (s * TAU).sin() * 0.8 };
        let f = SIZES[m.size as usize][frame];
        let (fw, fh) = (f[0].len() as f32, f.len() as f32);
        (
            (x - fw / 2.0).round() as i32,
            (y - lift - fh / 2.0).round() as i32,
            frame,
        )
    }
}

impl Egg for Swarm {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::new(theme);
        // The far ones paler, as through the air.
        let paler: [Palette; 2] = [0.35f32, 0.15].map(|k| {
            let mut p = Palette::new(theme);
            p.fade_all(theme, k);
            p
        });
        for m in &self.flock {
            let (x, y, frame) = self.place(m);
            let p = match m.size {
                0 => &paler[0],
                1 => &paler[1],
                _ => &pal,
            };
            let art = SIZES[m.size as usize][frame];
            sprite(c, (x, y), art, false, |_| 0, |ch| p.get(ch));
        }
    }

    fn done(&self) -> bool {
        let w = self.w + 2.0 * self.rx + 24.0;
        self.flock.iter().all(|m| self.gone(m) - self.rx > w) || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn frames_are_rectangles() {
        for frames in SIZES {
            for f in frames {
                assert!(f.iter().all(|r| r.len() == f[0].len()), "{f:?}");
            }
        }
    }

    /// The swarm comes in, fills a good part of the screen, and leaves.
    #[test]
    fn flows_across() {
        let theme = Theme::builtin("nord").unwrap();
        let mut e = start(120, 36, &mut fastrand::Rng::with_seed(3));
        let mut c = Canvas::new(120, 36, Color::Reset);
        let mut most = 0;
        let mut secs = 0.0;
        while !e.done() {
            e.update(0.25);
            secs += 0.25;
            c.clear();
            e.draw(&mut c, &theme);
            most = most.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
        }
        assert!(most > 80, "{most} cells");
        assert!(secs < 29.0, "{secs}");
    }
}
