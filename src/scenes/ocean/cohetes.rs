//! Fireworks over the water: cohetes climb from the water's edge, burst
//! into sparks that drift down and fade, and the water gives every spark
//! back, upside down and broken by the ripples.
//!
//! A burst is a round peony of one color with some white among it, a ring,
//! or a split of three colors, a third of the sparks each. The holiday
//! chooses the colors; how many bursts there are follows its level, and
//! when it is high enough now and then a volley goes off, six in a row.
//! They go off only at night, so the day goes by without.

use std::f32::consts::TAU;

use super::Dice;
use super::gala::Paint;
use super::sea::Sea;
use crate::color::Color;
use crate::fiesta::Kind;

/// How much the sparks slow down each tick, and how hard they fall.
const DRAG: f32 = 0.93;
const FALL: f32 = 0.012;
/// The most sparks in the air at once.
const MOST: usize = 480;

/// How a burst is shaped and colored.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Style {
    /// A round burst of one color with white among it.
    Peony,
    /// A single ring of one color.
    Ring,
    /// Three colors, a third of the sparks each.
    Split,
}

struct Rocket {
    x: f32,
    y: f32,
    vy: f32,
    /// The height it bursts at.
    top: f32,
    style: Style,
    hue: u8,
}

struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: u16,
    max: u16,
    hue: u8,
}

/// The fireworks of a night.
#[derive(Default)]
pub struct Cohetes {
    rockets: Vec<Rocket>,
    sparks: Vec<Spark>,
    /// Ticks until the next rocket and the next volley, and how many
    /// rockets of a volley are still to go.
    wait: f32,
    volley_in: f32,
    volley: u8,
}

/// The four colors of a holiday's fireworks, the second always white, and
/// which three make up a split burst.
fn hues(kind: Kind, p: &Paint) -> ([Color; 4], [u8; 3]) {
    // Año Nuevo: dorado and plata, rojo and verde.
    let _ = kind;
    ([p.gold, p.plata, p.rojo, p.verde], [2, 1, 3])
}

impl Cohetes {
    /// How many rockets and sparks are in the air.
    #[cfg(test)]
    pub fn counts(&self) -> (usize, usize) {
        (self.rockets.len(), self.sparks.len())
    }
}

impl Sea {
    /// Makes room for the sparks and sets when the first volley goes off.
    pub fn dress_cohetes(&mut self) {
        let g = &mut self.gala;
        g.works.sparks.reserve(MOST + 64);
        g.works.rockets.reserve(16);
        g.works.volley_in = g.rng.rangef(20.0, 35.0) * 20.0;
    }

    /// How far a burst reaches, in pixels: what the sky has room for.
    fn reach(&self) -> f32 {
        (self.surf as f32 * 0.38).clamp(2.0, 9.0)
    }

    /// Sends up a rocket from the water's edge, where the boats leave the
    /// sky free.
    fn launch(&mut self) {
        let (w, surf) = (self.w as f32, self.surf as f32);
        let reach = self.reach();
        let g = &mut self.gala;
        // Of a few tries, the column least covered by boats.
        let mut x = w * 0.5;
        let mut covered = f32::MAX;
        for _ in 0..8 {
            let try_x = g.rng.rangef(w * 0.04, w * 0.96);
            let cover: f32 = g
                .boats
                .iter()
                .map(|b| (b.w as f32 * 0.5 + reach - (b.x - try_x).abs()).max(0.0))
                .sum();
            if cover < covered {
                (x, covered) = (try_x, cover);
            }
        }
        let rng = &mut g.rng;
        // High enough that the burst fits under the top of the screen.
        let lo = reach + 0.5;
        let top = rng.rangef(lo, (surf - reach * 0.5 - 1.0).max(lo + 0.5));
        let ticks = rng.rangef(18.0, 28.0);
        let style = match rng.usize(..5) {
            0 => Style::Ring,
            1 | 2 => Style::Split,
            _ => Style::Peony,
        };
        let y = surf - 1.0;
        g.works.rockets.push(Rocket {
            x,
            y,
            vy: (top - y) / ticks,
            top,
            style,
            hue: [0, 2, 3][rng.usize(..3)],
        });
    }

    /// Bursts a rocket into sparks.
    fn burst(&mut self, r: &Rocket) {
        let radius = self.reach();
        let g = &mut self.gala;
        let rng = &mut g.rng;
        let radius = radius * rng.rangef(0.85, 1.1);
        let v0 = radius * (1.0 - DRAG) / DRAG;
        let n = (radius * 4.0 + 10.0) as usize;
        let split = hues(g.kind, &self.pal.paint).1;
        for i in 0..n {
            let a = TAU * i as f32 / n as f32 + rng.rangef(-0.1, 0.1);
            let speed = match r.style {
                Style::Ring => v0,
                Style::Split => v0 * rng.rangef(0.75, 1.0),
                Style::Peony => v0 * rng.rangef(0.35, 1.0),
            };
            let hue = match r.style {
                Style::Split => split[i * 3 / n],
                Style::Peony if i % 5 == 0 => 1,
                _ => r.hue,
            };
            if g.works.sparks.len() >= MOST {
                break;
            }
            let life = rng.range(30, 46) as u16;
            g.works.sparks.push(Spark {
                x: r.x,
                y: r.top,
                vx: a.cos() * speed,
                vy: a.sin() * speed - 0.02,
                life,
                max: life,
                hue,
            });
        }
    }

    /// A tick of the fireworks: rockets climb and burst, sparks fall and
    /// fade, and at night more go up.
    pub fn cohetes_tick(&mut self) {
        let surf = self.surf as f32;
        let works = &mut self.gala.works;
        for r in &mut works.rockets {
            r.y += r.vy;
        }
        works.sparks.retain_mut(|s| {
            s.life = s.life.saturating_sub(1);
            s.vx *= DRAG;
            s.vy = s.vy * DRAG + FALL;
            s.x += s.vx;
            s.y += s.vy;
            s.life > 0 && s.y < surf - 0.5
        });
        for i in (0..self.gala.works.rockets.len()).rev() {
            let r = &self.gala.works.rockets[i];
            if r.y <= r.top {
                let r = self.gala.works.rockets.swap_remove(i);
                self.burst(&r);
            }
        }
        let (w, dusk, level) = (self.w as f32, self.gala.dusk, self.gala.level);
        if dusk < 0.7 || self.surf < 6 {
            return;
        }
        // Bursts a second, more of them in a wide sea and as the holiday comes.
        let per_second = (0.1 + 0.35 * level) * (w / 100.0).clamp(0.6, 2.0);
        let g = &mut self.gala;
        if g.works.rockets.len() < 12 && g.rng.chance(per_second / 20.0) {
            self.launch();
        }
        if level >= 0.85 {
            let g = &mut self.gala;
            g.works.volley_in -= 1.0;
            g.works.wait -= 1.0;
            if g.works.volley_in <= 0.0 {
                g.works.volley = 6;
                g.works.volley_in = g.rng.rangef(25.0, 40.0) * 20.0;
            }
            if g.works.volley > 0 && g.works.wait <= 0.0 {
                g.works.volley -= 1;
                g.works.wait = 6.0;
                self.launch();
            }
        }
    }

    /// The rockets and sparks in the sky, and in the water.
    pub fn paint_cohetes(&mut self, t: f32) {
        let paint = self.pal.paint;
        let (palette, _) = hues(self.gala.kind, &paint);
        for i in 0..self.gala.works.rockets.len() {
            let r = &self.gala.works.rockets[i];
            let (x, y) = (r.x.round() as i32, r.y.round() as i32);
            let c = palette[r.hue as usize % 4];
            self.blend(x, y, paint.blanco, 0.95);
            self.blend(x, y + 1, c, 0.6);
            self.blend(x, y + 2, c, 0.3);
            self.mirror(x, y, c, 0.8, t);
        }
        for i in 0..self.gala.works.sparks.len() {
            let s = &self.gala.works.sparks[i];
            let (x, y) = (s.x.round() as i32, s.y.round() as i32);
            let fade = s.life as f32 / s.max as f32;
            let c = palette[s.hue as usize % 4];
            // A spark burns white at first, then its color, then dies down.
            let c = c.lerp(paint.blanco, ((fade - 0.85) * 5.0).max(0.0));
            self.blend(x, y, c, (fade * 2.4).min(1.0));
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                self.glow(x + dx, y + dy, c, 0.16 * fade);
            }
            self.mirror(x, y, c, fade, t);
        }
    }
}
