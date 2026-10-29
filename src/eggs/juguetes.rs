//! Traditional toys go by on Día del Niño: a trompo, a balero, a yoyo and a
//! handful of canicas, one after another, each doing what it is for.
//!
//! Original oxiper pixel art, all drawn from angles and positions so it
//! moves smoothly and scales to the screen. The **trompo** (spinning top)
//! is a wooden pear with a steel point and painted stripes that run around
//! it as it spins, leaning over and wobbling in a wide circle as it travels
//! along the ground, with the odd spark where its point grinds. The
//! **balero** (cup-and-ball) is held by a hand in a blue sleeve: the ball
//! hangs and sways on its string, swings up in an arc, drops into the cup
//! with a "¡clac!", rests there and is flicked out to hang again. The
//! **yoyo** comes down from the top of the screen on its string (the hand
//! holding it is above the screen): it falls, sleeps spinning at the bottom
//! and climbs back. And the **canicas** (glass marbles with a swirl inside)
//! roll in a row at different speeds; the fast ones catch the slow ones and
//! click, trading speeds, with a spark and a "¡clic!". The toys go by in a
//! line, overlapping, from either side, and it is all over within 30 s.

use super::nino::{Palette, fade};
use super::pen::{fill as rect, line};
use super::{Egg, Info, pixel};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::PI;

pub const INFO: Info = Info {
    name: "juguetes",
    start,
    fiesta: Kind::Nino,
    when: super::any_day,
};

/// When each toy is on the screen: it starts, and how long it takes to cross.
const TROMPO: (f32, f32) = (0.0, 9.0);
const BALERO: (f32, f32) = (5.5, 9.5);
const YOYO: (f32, f32) = (10.5, 9.5);
const CANICAS: (f32, f32) = (14.5, 9.5);
const END: f32 = 25.0;

/// How long the balero takes to catch its ball once, and the yoyo to go
/// down and up.
const BALERO_CYCLE: f32 = 2.8;
const YOYO_CYCLE: f32 = 3.0;

/// The marbles: how many, and how long a "¡clic!" shows.
const MARBLES: usize = 6;
const CLICKS: usize = 8;
const CLICK_LIFE: f32 = 0.7;
/// The marbles' colors, and what the pop-ups say.
const MARBLE_COLORS: [u8; MARBLES] = *b"BRGYOM";
/// A marble's radius, in the toys' unit.
const MARBLE: f32 = 3.6;
const SOUNDS: [&str; 3] = ["¡clic!", "¡tac!", "¡clac!"];

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Juguetes::new(w, h, rng))
}

impl Juguetes {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Juguetes {
        let k = (h as i32 / 22).clamp(1, 3) as f32;
        // A pack of five rolls in at a walking pace, a little different each,
        // and the sixth, behind them, comes fast: it hits the last one of the
        // pack, which passes the blow on, and so on down the row. The slowest
        // marble crosses in time.
        let slow = (w as f32 + 100.0 * k) / CANICAS.1;
        let mut marbles = [Marble::default(); MARBLES];
        for (i, m) in marbles.iter_mut().enumerate() {
            let shooter = i == MARBLES - 1;
            m.x = -MARBLE * k - i as f32 * 10.0 * k - if shooter { 30.0 * k } else { 0.0 };
            m.v = if shooter {
                slow * 3.0
            } else {
                slow * (1.0 + 0.22 * rng.f32())
            };
        }
        Juguetes {
            t: 0.0,
            w: w as i32,
            k,
            left: rng.bool(),
            marbles,
            clicks: [Click::default(); CLICKS],
            next: 0,
            rng: fastrand::Rng::with_seed(rng.u64(..)),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Marble {
    /// Pixels traveled from where the row began, and speed.
    x: f32,
    v: f32,
}

#[derive(Clone, Copy, Default)]
struct Click {
    /// Where, in the marbles' own way of counting, and for how long.
    x: f32,
    /// Seconds since; 0 means none.
    age: f32,
    sound: u8,
}

struct Juguetes {
    t: f32,
    w: i32,
    /// The scale of the toys: 1 to 3, with the screen.
    k: f32,
    /// Going by from right to left?
    left: bool,
    marbles: [Marble; MARBLES],
    clicks: [Click; CLICKS],
    next: usize,
    rng: fastrand::Rng,
}

/// Paints a disc, asking `shade` for the color of each pixel in it, given
/// as the pixel's offset from the center in radii.
fn disc(c: &mut Canvas, (cx, cy): (f32, f32), r: f32, shade: impl Fn(f32, f32) -> Option<Color>) {
    let r = r.min(200.0);
    for py in (cy - r).floor() as i32..=(cy + r).ceil() as i32 {
        for px in (cx - r).floor() as i32..=(cx + r).ceil() as i32 {
            let (dx, dy) = ((px as f32 + 0.5 - cx) / r, (py as f32 + 0.5 - cy) / r);
            if dx * dx + dy * dy <= 1.0
                && let Some(color) = shade(dx, dy)
            {
                pixel(c, px, py, color);
            }
        }
    }
}

fn ease_in(t: f32) -> f32 {
    t.clamp(0.0, 1.0).powi(2)
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(2)
}

impl Juguetes {
    /// Where a toy is on its way across, in columns, `margin` off the
    /// screen at either end; `None` while it is not on the screen.
    fn along(&self, (t0, dur): (f32, f32), margin: f32) -> Option<(f32, f32)> {
        let s = (self.t - t0) / dur;
        if !(0.0..=1.0).contains(&s) {
            return None;
        }
        let x = -margin + (self.w as f32 + 2.0 * margin) * s;
        Some((if self.left { self.w as f32 - x } else { x }, self.t - t0))
    }

    /// Which side of the toy it is moving toward: 1 or -1.
    fn ahead(&self) -> f32 {
        if self.left { -1.0 } else { 1.0 }
    }

    /// The trompo's color at a longitude (in radians) and a height up its
    /// body, 0 at the point to 23 at the knob, shaded by `side` (-1 lit, 1 dark).
    fn trompo_color(&self, pal: &Palette, lon: f32, h: f32, side: f32) -> Color {
        let ch = if h < 2.6 {
            b'S'
        } else if h > 20.4 {
            b'D'
        } else if (h - 7.4).abs() < 0.45 || (h - 12.6).abs() < 0.5 {
            // The rings painted around it.
            b'k'
        } else {
            (*b"RYBG")[((lon / (PI / 2.5)).floor() as i32).rem_euclid(4) as usize]
        };
        let color = pal.of(ch);
        if side < -0.5 && ch != b'k' {
            color.scale(1.25)
        } else if side > 0.55 {
            color.scale(0.75)
        } else {
            color
        }
    }

    /// The trompo: spinning, leaning and wobbling around its point.
    fn trompo(&self, c: &mut Canvas, pal: &Palette, x: f32, t: f32) {
        let u = self.k;
        let spin = t * 11.0;
        let tip = c.h() as f32 * 2.0;
        // It travels in a wide wobbling curve, its axis leaning one way and
        // the other (a top that is slowing down wobbles more).
        let x = x + (t * 1.3).sin() * 3.0 * u;
        let lean = (t * 2.3).sin() * 0.16 + (t * 0.7).sin() * 0.05;
        let top = (tip - 23.0 * u) as i32;
        for py in top..=tip as i32 {
            let h = (tip - py as f32 - 0.5) / u;
            let r = match h {
                h if h < 0.0 => continue,
                h if h < 2.5 => 0.6,
                h if h < 11.0 => 8.0 * ((h - 2.5) / 8.5).powf(0.75),
                h if h < 20.0 => 8.0 * (1.0 - ((h - 11.0) / 9.0).powi(2)).max(0.0).sqrt(),
                h if h <= 23.0 => 1.4,
                _ => continue,
            } * u;
            let axis = x + lean * h * u;
            for px in (axis - r).floor() as i32..=(axis + r).ceil() as i32 {
                let side = ((px as f32 + 0.5 - axis) / r.max(0.5)).clamp(-1.0, 1.0);
                let lon = side.asin() + spin;
                pixel(c, px, py, self.trompo_color(pal, lon, h, side));
            }
        }
        // Where the steel point grinds the ground, a spark now and then.
        let n = (t * 24.0) as i32;
        if n % 3 == 0 {
            let dx = ((n * 7) % 5 - 2) as f32 * u;
            pixel(c, (x + dx) as i32, tip as i32 - 1 - (n % 2), pal.of(b'Y'));
        }
    }

    /// The balero held in a hand: where the ball is at time `s` of its
    /// cycle, relative to where the string is tied, as (x ahead, y down) in
    /// units of the toy's size, and whether it is in the cup.
    fn ball(s: f32, string: f32, cup: f32) -> ((f32, f32), bool) {
        let hang = |sway: f32| (string * sway.sin(), string * sway.cos());
        match s {
            // Hanging and swaying.
            s if s < 0.35 => (hang(0.35 * (s * 9.0).sin() * (1.0 - s / 0.35)), false),
            // Swung up in an arc over the cup.
            s if s < 1.15 => {
                let a = PI * ease_out((s - 0.35) / 0.8);
                ((string * a.sin(), string * a.cos()), false)
            }
            // A breath at the top, then it drops into the cup.
            s if s < 1.45 => {
                let f = ease_in((s - 1.2) / 0.25);
                ((0.0, -string + (string - cup) * f), f >= 1.0)
            }
            // Resting in the cup.
            s if s < 2.1 => ((0.0, -cup), true),
            // Flicked out and dropping back to hang.
            s => {
                let f = ((s - 2.1) / 0.7).clamp(0.0, 1.0);
                ((0.0, -cup + (string + cup) * ease_in(f)), false)
            }
        }
    }

    /// The balero: a hand in a blue sleeve holding a wooden stick with a cup,
    /// and the ball on its string.
    fn balero(&self, c: &mut Canvas, pal: &Palette, x: f32, s: f32) {
        let u = self.k;
        let ground = c.h() as f32 * 2.0;
        let bob = (s * 3.0).sin() * 0.6 * u;
        let (cx, cy) = (x, ground - bob);
        let col = |ch: u8| pal.of(ch);
        let (x0, ground) = (cx.round() as i32, cy.round() as i32);
        let ui = u as i32;
        let up = |n: f32| ground - (n * u) as i32;
        // The stick, its painted bands, the bulb under the cup and the cup.
        rect(c, (x0 - ui, up(27.0)), (2 * ui, 27 * ui + 1), col(b'D'));
        rect(c, (x0 - ui, up(14.0)), (2 * ui, 2 * ui), col(b'R'));
        rect(c, (x0 - ui, up(18.0)), (2 * ui, ui), col(b'Y'));
        rect(c, (x0 + ui - 1, up(27.0)), (ui, 27 * ui), col(b'd'));
        let tie = up(22.0);
        rect(c, (x0 - 2 * ui, up(28.0)), (4 * ui, 3 * ui), col(b'D'));
        // The cup: a wooden bowl, wider at its mouth, dark inside.
        for row in 0..6 * ui {
            let half = (3.6 * u - row as f32 * 0.35 * u / ui as f32).max(2.0 * u);
            let y = up(34.0) + row;
            let (lo, hi) = (
                (x0 as f32 - half).round() as i32,
                (x0 as f32 + half).round() as i32,
            );
            rect(
                c,
                (lo, y),
                (hi - lo, 1),
                col(if row < ui { b'k' } else { b'D' }),
            );
            rect(
                c,
                (hi - ui, y),
                (ui, 1),
                col(if row < ui { b'k' } else { b'd' }),
            );
        }
        // The hand: a sleeve from the bottom of the screen, a white cuff,
        // and the fingers around the stick.
        rect(c, (x0 - 3 * ui, up(8.0)), (6 * ui, 8 * ui + 1), col(b'B'));
        rect(c, (x0 - 3 * ui, up(9.0)), (6 * ui, ui), col(b'W'));
        rect(c, (x0 - 2 * ui, up(14.0)), (4 * ui, 5 * ui), col(b'K'));
        rect(c, (x0 - ui, up(12.0)), (2 * ui, ui), col(b'd'));

        // The ball, and its string: tied under the cup, a straight line when
        // taut and a sag when the ball is dropping.
        let string = 16.0 * u;
        let (cup_depth, tie_x) = (7.0 * u, x0 as f32 + 0.5);
        let ((bx, by), _) = Self::ball(s % BALERO_CYCLE, string, cup_depth);
        let (bx, by) = (tie_x + bx * self.ahead(), tie as f32 + by);
        let slack = (string - (bx - tie_x).hypot(by - tie as f32)).max(0.0);
        let mid = (
            (tie_x + bx) / 2.0 + slack * 0.7 * self.ahead(),
            (tie as f32 + by) / 2.0,
        );
        let to = |p: (f32, f32)| (p.0.floor() as i32, p.1.floor() as i32);
        for (a, b) in [((tie_x, tie as f32), mid), (mid, (bx, by))] {
            line(c, to(a), to(b), col(b'W'));
        }
        disc(c, (bx, by), 2.8 * u, |dx, dy| {
            Some(if dx * dx + dy * dy < 0.12 && dx < 0.0 && dy < 0.0 {
                col(b'W')
            } else if dx + dy > 0.7 {
                col(b'r')
            } else {
                col(b'R')
            })
        });
    }

    /// The yoyo, down on its string from above the screen.
    fn yoyo(&self, c: &mut Canvas, pal: &Palette, x: f32, s: f32) {
        let u = self.k;
        let screen = c.h() as f32 * 2.0;
        let (high, low) = (7.0 * u, (screen * 0.55).max(14.0 * u));
        // Down, asleep at the bottom, back up, a breath at the top.
        let phase = s % YOYO_CYCLE;
        let (y, spin, sway) = match phase {
            p if p < 0.8 => (high + (low - high) * ease_in(p / 0.8), p * 14.0, 0.0),
            p if p < 1.6 => (low, p * 30.0, ((p - 0.8) * 7.0).sin() * 0.8 * u),
            p if p < 2.5 => (
                low - (low - high) * ease_out((p - 1.6) / 0.9),
                -p * 16.0,
                0.0,
            ),
            p => (high, p * 3.0, 0.0),
        };
        let x = x + sway;
        line(c, (x as i32, 0), (x as i32, y as i32), pal.of(b'W'));
        let r = 5.0 * u;
        disc(c, (x, y), r, |dx, dy| {
            let d = dx.hypot(dy);
            let (sa, ca) = spin.sin_cos();
            // Across the face: a stripe that turns with the yoyo.
            let across = (dx * ca + dy * sa).abs() < 0.22;
            let ch = if d > 0.76 {
                if dx + dy > 0.6 { b'r' } else { b'R' }
            } else if d < 0.22 {
                b'S'
            } else if across {
                b'B'
            } else {
                b'Y'
            };
            Some(if dx < -0.3 && dy < -0.3 && d > 0.7 {
                pal.of(b'W')
            } else {
                pal.of(ch)
            })
        });
    }

    /// The marbles roll and click.
    fn marbles(&self, c: &mut Canvas, pal: &Palette) {
        let u = self.k;
        let r = MARBLE * u;
        let ground = c.h() as f32 * 2.0;
        for (i, m) in self.marbles.iter().enumerate() {
            let x = if self.left { self.w as f32 - m.x } else { m.x };
            if x < -r || x > self.w as f32 + r {
                continue;
            }
            // Rolling: what is inside turns with the way it has gone.
            let roll = m.x / r * if self.left { -1.0 } else { 1.0 };
            let (sr, cr) = roll.sin_cos();
            let (base, dark) = (
                pal.of(MARBLE_COLORS[i]),
                pal.of(MARBLE_COLORS[i]).scale(0.62),
            );
            disc(c, (x, ground - r), r, |dx, dy| {
                let (rx, ry) = (dx * cr - dy * sr, dx * sr + dy * cr);
                // A ribbon of white across it, and a second, thinner one.
                let ribbon = (ry + 0.25 * rx).abs() < 0.26 || (rx - 0.55).abs() < 0.12;
                let mut color = if ribbon { pal.of(b'W') } else { base };
                if dx + dy > 0.75 {
                    color = if ribbon { pal.of(b'w') } else { dark };
                }
                // A glint on the glass, which stays where the light is.
                if (dx + 0.45).powi(2) + (dy + 0.5).powi(2) < 0.04 {
                    color = pal.of(b'W');
                }
                Some(color)
            });
        }
    }
}

impl Egg for Juguetes {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        let before = self.t;
        self.t += dt;
        for click in self.clicks.iter_mut().filter(|c| c.age > 0.0) {
            click.age += dt;
            if click.age > CLICK_LIFE {
                click.age = 0.0;
            }
        }
        if self.t < CANICAS.0 {
            return;
        }
        // The marbles roll in small steps, so a fast one never goes through
        // a slow one; when they touch they trade speeds, like billiard balls.
        let mut left = (self.t - before.max(CANICAS.0)).max(0.0);
        let r = MARBLE * self.k;
        while left > 0.0 {
            let step = left.min(0.01);
            left -= step;
            for m in self.marbles.iter_mut() {
                m.x += m.v * step;
            }
            for i in 0..MARBLES - 1 {
                let (front, back) = (self.marbles[i], self.marbles[i + 1]);
                if front.x - back.x < 2.0 * r && back.v > front.v {
                    let mid = (front.x + back.x) / 2.0;
                    self.marbles[i].v = back.v;
                    self.marbles[i + 1].v = front.v;
                    self.marbles[i].x = mid + r;
                    self.marbles[i + 1].x = mid - r;
                    if mid > 0.0 && mid < self.w as f32 {
                        self.clicks[self.next] = Click {
                            x: mid,
                            age: 1e-3,
                            sound: self.rng.u8(..SOUNDS.len() as u8),
                        };
                        self.next = (self.next + 1) % CLICKS;
                    }
                }
            }
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::nino(theme);
        let u = self.k;
        if let Some((x, t)) = self.along(TROMPO, 12.0 * u) {
            self.trompo(c, &pal, x, t);
        }
        if let Some((x, t)) = self.along(BALERO, 7.0 * u) {
            self.balero(c, &pal, x, t);
        }
        if let Some((x, t)) = self.along(YOYO, 7.0 * u) {
            self.yoyo(c, &pal, x, t);
        }
        if self.t >= CANICAS.0 {
            self.marbles(c, &pal);
        }
        // The clicks: a spark where two marbles meet, and the sound rising.
        let ground = c.h() as i32 * 2;
        for click in self.clicks.iter().filter(|k| k.age > 0.0) {
            let x = if self.left {
                self.w as f32 - click.x
            } else {
                click.x
            };
            let (x, y) = (x as i32, ground - (MARBLE * u) as i32);
            if click.age < 0.18 {
                for d in 1..=(2.0 * u) as i32 {
                    for (dx, dy) in [
                        (d, 0),
                        (-d, 0),
                        (0, d),
                        (0, -d),
                        (d, d),
                        (-d, -d),
                        (d, -d),
                        (-d, d),
                    ] {
                        pixel(c, x + dx, y + dy, pal.of(b'Y'));
                    }
                }
            }
            let rise = click.age / CLICK_LIFE;
            let sound = SOUNDS[click.sound as usize % SOUNDS.len()];
            let col = fade(theme, pal.of(b'Y'), rise * rise);
            let row = (ground - (9.0 * u) as i32) / 2 - (rise * 3.0) as i32;
            c.text_attr(x - sound.chars().count() as i32 / 2, row, sound, col, BOLD);
        }
        // The balero says what it did when the ball lands in the cup.
        if let Some((x, s)) = self.along(BALERO, 7.0 * u) {
            let since = s % BALERO_CYCLE - 1.4;
            if (0.0..0.7).contains(&since) {
                let row = (ground - (36.0 * u) as i32) / 2 - (since * 3.0) as i32;
                let col = fade(theme, pal.of(b'Y'), since / 0.7 * 0.8);
                c.text_attr(x as i32 - 3, row, "¡clac!", col, BOLD);
            }
        }
    }

    fn done(&self) -> bool {
        // Over once the last marble has rolled off and its click has faded.
        let off = |m: &Marble| m.x > self.w as f32 + MARBLE * self.k;
        let rolled = self.t > CANICAS.0 && self.marbles.iter().all(off);
        rolled && self.clicks.iter().all(|c| c.age == 0.0) || self.t > END
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The toys come one after another, each is on the screen when it
    /// should be, and the marbles click.
    #[test]
    fn the_toys_go_by() {
        let theme = Theme::classic();
        for (seed, w, h) in [(1, 100, 30), (2, 160, 48), (3, 60, 20)] {
            let mut e = start(w, h, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(w, h, Color::Reset);
            let mut painted = Vec::new();
            let mut sounds = 0;
            let mut secs = 0.0;
            while !e.done() && secs < 40.0 {
                e.update(0.1);
                secs += 0.1;
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                sounds += text.matches("¡clic!").count()
                    + text.matches("¡tac!").count()
                    + text.matches("¡clac!").count();
                painted.push(c.cells().iter().filter(|cell| cell.ch != ' ').count());
            }
            assert!(secs <= 26.0, "{secs} s at {w}x{h}");
            assert!(sounds > 3, "{sounds} sounds at {w}x{h}");
            // Something is on the screen all the time, from the trompo's
            // first steps to the last marbles.
            assert!(
                painted[20..painted.len() - 10].iter().all(|&n| n > 20),
                "{painted:?}"
            );
        }
    }

    /// The marbles trade speeds when they touch, and never go through one
    /// another: they stay in order, a marble's width apart at least.
    #[test]
    fn marbles_trade_speeds_without_passing_through() {
        let mut e = Juguetes::new(100, 30, &mut fastrand::Rng::with_seed(5));
        let speeds = |e: &Juguetes| {
            let mut v: Vec<f32> = e.marbles.iter().map(|m| m.v).collect();
            v.sort_by(f32::total_cmp);
            v
        };
        let before = speeds(&e);
        let mut clicked = false;
        while !e.done() {
            e.update(0.1);
            for pair in e.marbles.windows(2) {
                assert!(pair[0].x - pair[1].x >= 2.0 * MARBLE - 0.5, "{pair:?}");
            }
            clicked |= e.clicks.iter().any(|c| c.age > 0.0);
        }
        // The same speeds, only traded.
        assert_eq!(before, speeds(&e));
        assert!(clicked);
    }
}
