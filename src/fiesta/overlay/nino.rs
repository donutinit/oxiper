//! Día del Niño: globos of every color rising over every scene, with their
//! strings trailing below, and now and then one of them pops.

use super::{Fiesta, Look, Overlay, Themed, put, wave};
use crate::color::{Color, rgb};
use crate::render::Canvas;
use crate::theme::Theme;

/// A small balloon and a big one, `X` for rubber: the knot at the bottom.
const SMALL: [&str; 6] = [".XXX.", "XXXXX", "XXXXX", "XXXXX", ".XXX.", "..X.."];
const BIG: [&str; 8] = [
    "..XXX..", ".XXXXX.", "XXXXXXX", "XXXXXXX", "XXXXXXX", ".XXXXX.", "..XXX..", "...X...",
];
/// How long a pop lasts, in seconds: a flash, then the scraps flying.
const POP: f32 = 0.9;
const FLASH: f32 = 0.08;
/// Seconds between one pop and the next, at least and at most.
const POP_EVERY: (f32, f32) = (5.0, 15.0);
/// The most balloons on a screen.
const MOST: i32 = 12;

/// The balloons of a holiday.
pub(super) struct Balloons {
    /// The rubber under `classic`; any other theme gives its own colors.
    pub colors: [Color; 6],
    /// Cells per balloon.
    pub cells: i32,
}

const BALLOONS: Balloons = Balloons {
    colors: [
        rgb(0xe8322a), // rojo
        rgb(0xffd200), // amarillo
        rgb(0x1f7ae0), // azul
        rgb(0x2fb84a), // verde
        rgb(0xff8c00), // naranja
        rgb(0xe4007c), // rosa mexicano
    ],
    cells: 700,
};

pub(super) struct Balloon {
    /// Column the sway centers on, and the pixel row of the top (it rises, so
    /// the row falls).
    pub x: f32,
    pub y: f32,
    /// Pixels per second up.
    pub rise: f32,
    /// Columns either side, and radians per second.
    pub sway: f32,
    pub freq: f32,
    pub phase: f32,
    pub color: u8,
    pub big: bool,
    /// Length of the string in pixels.
    pub string: u8,
    /// Seconds since it popped, if it did.
    pub popped: Option<f32>,
}

/// What Día del Niño hangs: balloons, which need no day of their own.
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 5,
        salt: 0x6e69_6e6f_7321_2121,
        balloons: Some(&BALLOONS),
        ..Look::NONE
    }
}

impl Overlay {
    /// Lets the balloons go: spread over the screen, as many as it holds
    /// without being buried.
    pub(super) fn scatter_balloons(&mut self, balloons: &Balloons, salt: u64) {
        let (w, h) = (self.w as i32, self.h as i32);
        let mut rng = fastrand::Rng::with_seed(super::SEED ^ salt ^ ((w as u64) << 16 | h as u64));
        let most = (w * h / balloons.cells.max(1)).clamp(1, MOST);
        self.balloons.clear();
        for i in 0..most {
            let mut b = balloon(&mut rng, w);
            // Evenly across the width, each a little off its place.
            b.x = (i as f32 + 0.25 + 0.5 * rng.f32()) * w as f32 / most as f32;
            b.y = rng.f32() * (h * 2) as f32 - 6.0;
            self.balloons.push(b);
        }
        self.pop_in = rng.f32() * POP_EVERY.1;
        self.rng = rng;
    }

    /// Lets the balloons rise for `dt` seconds, and pops one now and then.
    pub(super) fn rise(&mut self, dt: f32) {
        let (w, ph) = (self.w as i32, self.h as f32 * 2.0);
        for i in 0..self.balloons.len() {
            let b = &mut self.balloons[i];
            let height = if b.big { BIG.len() } else { SMALL.len() };
            match &mut b.popped {
                None => b.y -= b.rise * dt,
                Some(t) => *t += dt,
            }
            // Gone off the top, or popped and done: a new one rises from
            // below.
            let gone = b.y + (height + b.string as usize) as f32 + 2.0 < 0.0;
            if gone || b.popped.is_some_and(|t| t > POP) {
                let mut q = balloon(&mut self.rng, w);
                q.y = ph + self.rng.f32() * 10.0;
                self.balloons[i] = q;
            }
        }
        self.pop_in -= dt;
        if self.pop_in <= 0.0 && !self.balloons.is_empty() {
            let i = self.rng.usize(..self.balloons.len());
            // Only one that is up in the screen and still whole.
            let b = &mut self.balloons[i];
            if b.popped.is_none() && b.y > 0.0 && b.y < ph - 8.0 {
                b.popped = Some(0.0);
            }
            self.pop_in = POP_EVERY.0 + self.rng.f32() * (POP_EVERY.1 - POP_EVERY.0);
        }
    }

    /// Draws the balloons: each one's string behind it, and the pop.
    pub(super) fn draw_balloons(
        &self,
        c: &mut Canvas,
        theme: &Theme,
        balloons: &Balloons,
        level: f32,
    ) {
        let mut colors = balloons.colors;
        let mut string = rgb(0xd8d0c0);
        if !theme.native {
            for (i, slot) in colors.iter_mut().enumerate() {
                *slot = Themed::Accents.color(theme, i, *slot);
            }
            string = theme.fg;
        }
        let level = level.clamp(0.0, 1.0);
        let n = ((self.balloons.len() as f32 * (0.3 + 0.7 * level)).round() as usize)
            .min(self.balloons.len());
        for b in &self.balloons[..n] {
            self.draw_balloon(c, b, colors[b.color as usize % colors.len()], string);
        }
    }

    fn draw_balloon(&self, c: &mut Canvas, b: &Balloon, color: Color, string: Color) {
        let sprite: &[&str] = if b.big { &BIG } else { &SMALL };
        let (w, h) = (sprite[0].len() as i32, sprite.len() as i32);
        let sway = b.sway * wave(self.t, b.freq, b.phase).sin();
        let cx = (b.x + sway).round() as i32;
        let top = b.y.floor() as i32;
        let white = Color::Rgb(255, 255, 255);
        let age = b.popped.map(|t| (t - FLASH).max(0.0));
        // The string wanders a little below the knot (and falls once the
        // balloon has popped).
        let fall = age.map_or(0.0, |e| 14.0 * e * e);
        for r in 0..b.string as i32 {
            let wiggle = (0.9 * (wave(self.t, 1.8, b.phase) + 0.45 * r as f32).sin()).round();
            let fade = 1.0 - 0.55 * r as f32 / b.string.max(1) as f32;
            let fade = fade * age.map_or(1.0, |e| (1.0 - e * 1.1).max(0.0));
            put(
                c,
                cx + wiggle as i32,
                top + h + r + fall as i32,
                string.scale(0.75 * fade),
            );
        }
        match b.popped {
            // The rubber: lit from the upper left, darker toward the lower right.
            None => {
                for (r, row) in sprite.iter().enumerate() {
                    for (i, ch) in row.bytes().enumerate() {
                        if ch != b'X' {
                            continue;
                        }
                        let u = (i as f32 + 0.5) / w as f32 + (r as f32 + 0.5) / h as f32;
                        let shade = if r == 1 && i == 1 + b.big as usize {
                            color.lerp(white, 0.6)
                        } else if r == (h as usize) - 2 && i == 0 {
                            color
                        } else if u > 1.15 {
                            color.scale(0.8)
                        } else {
                            color
                        };
                        let shade = if r + 1 == sprite.len() {
                            color.scale(0.75)
                        } else {
                            shade
                        };
                        put(c, cx - w / 2 + i as i32, top + r as i32, shade);
                    }
                }
            }
            // A flash where it was, and then the scraps flying out and
            // falling.
            Some(t) if t < FLASH => {
                for (r, row) in sprite.iter().take(sprite.len() - 1).enumerate() {
                    for (i, ch) in row.bytes().enumerate() {
                        if ch == b'X' {
                            put(
                                c,
                                cx - w / 2 + i as i32,
                                top + r as i32,
                                color.lerp(white, 0.85),
                            );
                        }
                    }
                }
            }
            Some(_) => {
                let e = age.unwrap_or(0.0);
                let (mx, my) = (cx as f32, top as f32 + h as f32 * 0.5);
                for k in 0..8 {
                    let a = k as f32 * std::f32::consts::TAU / 8.0 + b.phase;
                    let r = w as f32 * 0.5 + 22.0 * e * (1.0 - 0.6 * e);
                    let (x, y) = (mx + r * a.cos(), my + r * a.sin() + 9.0 * e * e);
                    let fade = (1.0 - e * 1.25).max(0.15);
                    put(c, x.round() as i32, y.round() as i32, color.scale(fade));
                }
            }
        }
    }
}

/// A new balloon somewhere across `w` columns.
fn balloon(rng: &mut fastrand::Rng, w: i32) -> Balloon {
    Balloon {
        x: rng.f32() * w.max(1) as f32,
        y: 0.0,
        rise: 3.0 + rng.f32() * 3.0,
        sway: 0.8 + rng.f32() * 1.4,
        freq: 0.5 + rng.f32() * 0.5,
        phase: rng.f32() * std::f32::consts::TAU,
        color: rng.u8(..6),
        big: rng.bool(),
        string: 7 + rng.u8(..6),
        popped: None,
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{after, changed, scene};
    use super::*;
    use crate::fiesta::Kind;

    #[test]
    fn dresses_dia_del_nino() {
        let theme = Theme::classic();
        let fiesta = Fiesta::height(Kind::Nino);
        assert!(fiesta.is(Kind::Nino));
        for (w, h) in [(40, 12), (80, 24), (120, 36), (200, 60), (500, 150)] {
            let mut o = Overlay::new(w, h);
            let mut most = 0;
            for _ in 0..40 {
                let n = changed(&after(&mut o, fiesta, 0.5, &theme), w).len();
                // Few enough not to bury the scene.
                assert!(n * 12 < w as usize * h as usize, "{w}x{h}: {n} cells");
                most = most.max(n);
            }
            assert!(most > 10, "{w}x{h}: {most} cells of balloons");
            assert!((1..=MOST as usize).contains(&o.balloons.len()));
        }
    }

    #[test]
    fn balloons_rise_from_the_bottom() {
        let fiesta = Fiesta::height(Kind::Nino);
        let (w, h) = (120, 36);
        let mut o = Overlay::new(w, h);
        o.update(0.0, fiesta);
        let ph = h as f32 * 2.0;
        let mut went_round = 0;
        for _ in 0..3000 {
            let before: Vec<f32> = o.balloons.iter().map(|b| b.y).collect();
            o.update(0.1, fiesta);
            for (b, y) in o.balloons.iter().zip(before) {
                if b.y > y {
                    // A new one, below the bottom of the screen.
                    assert!(b.y >= ph, "{} after {y}", b.y);
                    went_round += 1;
                } else {
                    assert!(b.y < y + 1e-3);
                }
            }
        }
        assert!(went_round > 10, "{went_round}");
    }

    #[test]
    fn one_pops_now_and_then() {
        let fiesta = Fiesta::height(Kind::Nino);
        let theme = Theme::classic();
        let mut o = Overlay::new(120, 36);
        o.update(0.0, fiesta);
        let (mut pops, mut popping) = (0, false);
        for _ in 0..3000 {
            o.update(0.1, fiesta);
            let now = o.balloons.iter().any(|b| b.popped.is_some());
            pops += usize::from(now && !popping);
            popping = now;
            // It leaves the screen drawn: flash, scraps and string.
            let mut c = Canvas::new(120, 36, Color::Reset);
            o.draw(&mut c, &theme, fiesta);
        }
        // About every ten seconds in five minutes.
        assert!((8..=80).contains(&pops), "{pops} pops");
    }

    #[test]
    fn a_pop_flashes_then_scatters_then_is_gone() {
        let fiesta = Fiesta::height(Kind::Nino);
        let theme = Theme::classic();
        let mut o = Overlay::new(60, 30);
        o.update(0.0, fiesta);
        o.balloons.truncate(1);
        o.balloons[0].y = 20.0;
        o.balloons[0].x = 30.0;
        o.balloons[0].sway = 0.0;
        let draw = |o: &Overlay| {
            let mut c = Canvas::new(60, 30, Color::Reset);
            o.draw(&mut c, &theme, fiesta);
            c.to_text()
        };
        let whole = draw(&o);
        let mut frames = vec![whole];
        for t in [0.03, 0.2, 0.5, 0.85] {
            o.balloons[0].popped = Some(t);
            frames.push(draw(&o));
        }
        frames.dedup();
        assert_eq!(frames.len(), 5, "{}", frames.join("\n--\n"));
        // And a new one rises from below once it is over.
        o.balloons[0].popped = Some(POP);
        o.update(0.1, fiesta);
        assert!(o.balloons[0].popped.is_none() && o.balloons[0].y >= 60.0);
    }

    #[test]
    fn balloons_touch_only_their_cells() {
        let fiesta = Fiesta::height(Kind::Nino);
        let mut o = Overlay::new(120, 40);
        o.update(0.0, fiesta);
        for _ in 0..80 {
            o.update(0.4, fiesta);
            let mut c = scene(120, 40);
            o.draw(&mut c, &Theme::classic(), fiesta);
            // The biggest balloon's pixels, its string and its scraps.
            let most = o.balloons.len() * (BIG.len() * 7 + 13 + 8);
            let n = changed(&c, 120).len();
            assert!(n > 0 && n <= most, "{n} cells");
        }
    }

    #[test]
    fn nothing_on_an_ordinary_day() {
        for fiesta in [Fiesta::NONE, Fiesta::of_day(6, 15), Fiesta::of_day(8, 20)] {
            let mut o = Overlay::new(80, 24);
            let c = after(&mut o, fiesta, 3.0, &Theme::classic());
            assert!(changed(&c, 80).is_empty());
        }
    }

    #[test]
    fn comes_and_goes_with_the_holiday() {
        let theme = Theme::builtin("dracula").unwrap();
        let mut o = Overlay::new(100, 30);
        let on = |o: &mut Overlay, f| !changed(&after(o, f, 1.0, &theme), 100).is_empty();
        assert!(on(&mut o, Fiesta::height(Kind::Nino)));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, Fiesta::of_day(4, 30)));
        assert!(!on(&mut o, Fiesta::of_day(5, 3)));
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert!(on(&mut o, Fiesta::of_day(4, 30)));
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        let fiesta = Fiesta::height(Kind::Nino);
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (w, h) in [
                (1, 1),
                (2, 3),
                (7, 4),
                (8, 1),
                (16, 2),
                (300, 90),
                (500, 150),
            ] {
                let mut o = Overlay::new(w, h);
                let mut c = Canvas::new(w, h, Color::Reset);
                for dt in [5.0, f32::NAN, -3.0, f32::INFINITY, 0.0, 5.0, 1e9] {
                    o.update(dt, fiesta);
                    c.clear();
                    o.draw(&mut c, &theme, fiesta);
                }
                o.resize(h, w);
                c.resize(h, w);
                c.strict = true;
                for _ in 0..200 {
                    o.update(0.5, fiesta);
                    c.clear();
                    o.draw(&mut c, &theme, fiesta);
                }
            }
        }
    }

    #[test]
    fn same_frames_every_time() {
        let theme = Theme::classic();
        let fiesta = Fiesta::height(Kind::Nino);
        let frame = || after(&mut Overlay::new(90, 28), fiesta, 20.0, &theme).to_text();
        assert_eq!(frame(), frame());
    }
}
