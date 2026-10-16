//! Easter eggs: small surprises that show up suddenly and at random while
//! oxiper runs, drawn over whatever scene is playing.
//!
//! An egg is not a scene: it is never in a playlist, goes away by itself
//! (30 seconds at most) and can be switched off with `eggs = "none"` or
//! `egg_rate = 0`. The scene keeps running underneath.
//!
//! The scheduler has its own RNG, separate from the scenes' seeded ones, so
//! eggs never change what `record`, `snap` and `bench` produce. Those never
//! roll for an egg; only the interactive loop does (or a forced rate).

pub mod duckpond;
pub mod sl;

use crate::render::Canvas;
use crate::theme::Theme;

/// One easter egg while it plays.
pub trait Egg {
    /// Advances by `dt` real seconds (eggs ignore the speed setting).
    fn update(&mut self, dt: f32);
    /// Draws over the already drawn scene.
    fn draw(&self, c: &mut Canvas, theme: &Theme);
    /// Has it finished?
    fn done(&self) -> bool;
}

pub struct Info {
    pub name: &'static str,
    /// Starts the egg for a screen of `w` x `h` cells.
    pub start: fn(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg>,
}

pub const ALL: &[&Info] = &[&sl::INFO, &duckpond::INFO];

/// An `egg_rate` at or above this plays an egg at once, again and again, and
/// even in `snap` and `record`: the way to force one for testing.
pub const FORCE_RATE: f64 = 1e6;
/// Eggs never start in the first minute, so they stay a surprise.
const GRACE: f64 = 60.0;
/// An egg that is still running after this long is cut short.
const MAX_SECS: f64 = 30.0;

/// Parses the `eggs` setting: "all", "none" or a comma-separated list of names.
pub fn parse(s: &str, warnings: &mut Vec<String>) -> Vec<&'static Info> {
    let mut out: Vec<&'static Info> = Vec::new();
    for name in s.split(',').map(str::trim).filter(|n| !n.is_empty()) {
        match name {
            "all" => out.extend(ALL),
            "none" => {}
            _ => match ALL.iter().find(|i| i.name == name) {
                Some(i) => out.push(i),
                None => warnings.push(format!(
                    "eggs: no such egg '{name}' (known: {})",
                    ALL.iter().map(|i| i.name).collect::<Vec<_>>().join(", ")
                )),
            },
        }
    }
    out.dedup_by_key(|i| i.name);
    out
}

pub struct Eggs {
    active: Option<Box<dyn Egg>>,
    played: f64,
    rng: fastrand::Rng,
    /// Real seconds since start-up, for the first-minute grace.
    age: f64,
    w: u16,
    h: u16,
}

impl Eggs {
    pub fn new(w: u16, h: u16) -> Eggs {
        Eggs {
            active: None,
            played: 0.0,
            rng: fastrand::Rng::new(),
            age: 0.0,
            w,
            h,
        }
    }

    pub fn active(&self) -> bool {
        self.active.is_some()
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.w = w;
        self.h = h;
    }

    /// Plays one of `allowed` right now, picked at random (nothing if empty
    /// or if one is already playing).
    pub fn start(&mut self, allowed: &[&'static Info]) {
        if self.active.is_some() || allowed.is_empty() {
            return;
        }
        let info = allowed[self.rng.usize(..allowed.len())];
        self.active = Some((info.start)(self.w, self.h, &mut self.rng));
        self.played = 0.0;
    }

    /// Rolls for an egg after `dt` real seconds. The chance is a per-second
    /// probability so it does not depend on the frame rate; `rate` is the
    /// average number of eggs per hour. `allow` is false while something (the
    /// help overlay) must not be covered.
    pub fn roll(&mut self, allowed: &[&'static Info], rate: f64, dt: f64, allow: bool) {
        self.age += dt;
        if self.active.is_some() || !allow || rate <= 0.0 || allowed.is_empty() {
            return;
        }
        if rate >= FORCE_RATE {
            return self.start(allowed);
        }
        if self.age >= GRACE && self.rng.f64() < 1.0 - (-rate / 3600.0 * dt).exp() {
            self.start(allowed);
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(e) = &mut self.active {
            e.update(dt);
            self.played += dt as f64;
            if e.done() || self.played > MAX_SECS {
                self.active = None;
            }
        }
    }

    pub fn draw(&self, c: &mut Canvas, theme: &Theme) {
        if let Some(e) = &self.active {
            e.draw(c, theme);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn parses_the_setting() {
        let mut w = Vec::new();
        assert_eq!(parse("all", &mut w).len(), ALL.len());
        assert!(parse("none", &mut w).is_empty());
        assert!(parse("", &mut w).is_empty());
        let one = parse(" sl ", &mut w);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].name, "sl");
        assert!(w.is_empty());
        assert_eq!(parse("sl, bogus", &mut w).len(), 1);
        assert_eq!(w.len(), 1);
    }

    #[test]
    fn never_in_the_first_minute() {
        for _ in 0..20 {
            let mut e = Eggs::new(80, 24);
            for _ in 0..(59 * 30) {
                e.roll(ALL, 3600.0 * 100.0, 1.0 / 30.0, true);
            }
            assert!(!e.active());
        }
    }

    #[test]
    fn disabled_means_never() {
        let mut e = Eggs::new(80, 24);
        e.age = 1000.0;
        for _ in 0..100_000 {
            e.roll(&[], 1e5, 1.0, true);
            e.roll(ALL, 0.0, 1.0, true);
        }
        assert!(!e.active());
    }

    #[test]
    fn not_while_help_is_open() {
        let mut e = Eggs::new(80, 24);
        e.age = 1000.0;
        for _ in 0..1000 {
            e.roll(ALL, FORCE_RATE, 0.1, false);
        }
        assert!(!e.active());
    }

    #[test]
    fn rate_matches_the_setting() {
        // 1.2 per hour over 2000 hours of 1 s ticks: about 2400 starts.
        let mut e = Eggs::new(80, 24);
        e.age = 1000.0;
        let mut starts = 0;
        for _ in 0..(2000 * 3600) {
            e.roll(ALL, 1.2, 1.0, true);
            if e.active() {
                starts += 1;
                e.active = None;
            }
        }
        assert!((2000..2800).contains(&starts), "{starts}");
    }

    #[test]
    fn only_listed_eggs_play() {
        let only = parse("duckpond", &mut Vec::new());
        let t = Theme::builtin("nord").unwrap();
        for _ in 0..30 {
            let mut e = Eggs::new(80, 24);
            e.start(&only);
            assert!(e.active());
            let mut c = Canvas::new(80, 24, Color::Reset);
            e.update(1.0);
            e.draw(&mut c, &t);
            // The duck's water, never the locomotive's smoke or rails.
            assert!(c.to_text().contains('~'));
            assert!(!c.to_text().contains("____"));
        }
    }

    #[test]
    fn every_egg_finishes_in_time() {
        let t = Theme::builtin("nord").unwrap();
        for info in ALL {
            for (w, h) in [(80, 24), (200, 50), (20, 8)] {
                let mut e = Eggs::new(w, h);
                e.start(&[info]);
                let mut c = Canvas::new(w, h, Color::Reset);
                let mut secs = 0.0;
                while e.active() {
                    e.update(1.0 / 30.0);
                    c.clear();
                    e.draw(&mut c, &t);
                    secs += 1.0 / 30.0;
                    assert!(secs <= 31.0, "{} too long at {w}x{h}", info.name);
                }
                assert!(secs > 1.0, "{} ended at once", info.name);
            }
        }
    }
}
