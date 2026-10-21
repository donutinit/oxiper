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
pub mod nyan;
pub mod sl;

use crate::fiesta::{Fiesta, Kind};
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
    /// The holiday it belongs to: it only comes then, and then more often
    /// than the others. `Kind::None` comes on any day.
    pub fiesta: Kind,
}

pub const ALL: &[&Info] = &[&sl::INFO, &duckpond::INFO, &nyan::INFO];

/// An `egg_rate` at or above this plays an egg at once, again and again, and
/// even in `snap` and `record`: the way to force one for testing.
pub const FORCE_RATE: f64 = 1e6;
/// During a holiday at its height, eggs come this many times as often.
const FIESTA_BOOST: f64 = 5.0;
/// How often a holiday's own eggs are picked over the everyday ones.
const FIESTA_SHARE: f64 = 0.8;
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
    /// or if one is already playing). During a holiday its own eggs come
    /// first; another holiday's never do.
    pub fn start(&mut self, allowed: &[&'static Info], fiesta: Fiesta) {
        if self.active.is_some() {
            return;
        }
        let ours = |i: &&&Info| fiesta.on() && i.fiesta == fiesta.kind;
        let any_day = |i: &&&Info| i.fiesta == Kind::None;
        let seasonal: Vec<&'static Info> = allowed.iter().filter(ours).copied().collect();
        let everyday: Vec<&'static Info> = allowed.iter().filter(any_day).copied().collect();
        let pool = if !seasonal.is_empty() && (everyday.is_empty() || self.rng.f64() < FIESTA_SHARE)
        {
            seasonal
        } else {
            everyday
        };
        if pool.is_empty() {
            return;
        }
        let info = pool[self.rng.usize(..pool.len())];
        self.active = Some((info.start)(self.w, self.h, &mut self.rng));
        self.played = 0.0;
    }

    /// Rolls for an egg after `dt` real seconds. The chance is a per-second
    /// probability so it does not depend on the frame rate; `rate` is the
    /// average number of eggs per hour. `allow` is false while something (the
    /// help overlay) must not be covered.
    pub fn roll(
        &mut self,
        allowed: &[&'static Info],
        rate: f64,
        dt: f64,
        allow: bool,
        fiesta: Fiesta,
    ) {
        self.age += dt;
        if self.active.is_some() || !allow || rate <= 0.0 || allowed.is_empty() {
            return;
        }
        if rate >= FORCE_RATE {
            return self.start(allowed, fiesta);
        }
        let rate = if fiesta.on() {
            rate * (1.0 + FIESTA_BOOST * fiesta.level as f64)
        } else {
            rate
        };
        if self.age >= GRACE && self.rng.f64() < 1.0 - (-rate / 3600.0 * dt).exp() {
            self.start(allowed, fiesta);
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
                e.roll(ALL, 3600.0 * 100.0, 1.0 / 30.0, true, Fiesta::NONE);
            }
            assert!(!e.active());
        }
    }

    #[test]
    fn disabled_means_never() {
        let mut e = Eggs::new(80, 24);
        e.age = 1000.0;
        for _ in 0..100_000 {
            e.roll(&[], 1e5, 1.0, true, Fiesta::NONE);
            e.roll(ALL, 0.0, 1.0, true, Fiesta::NONE);
        }
        assert!(!e.active());
    }

    #[test]
    fn not_while_help_is_open() {
        let mut e = Eggs::new(80, 24);
        e.age = 1000.0;
        for _ in 0..1000 {
            e.roll(ALL, FORCE_RATE, 0.1, false, Fiesta::NONE);
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
            e.roll(ALL, 1.2, 1.0, true, Fiesta::NONE);
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
            e.start(&only, Fiesta::NONE);
            assert!(e.active());
            let mut c = Canvas::new(80, 24, Color::Reset);
            e.update(1.0);
            e.draw(&mut c, &t);
            // The duck's water, never the locomotive's smoke or rails.
            assert!(c.to_text().contains('~'));
            assert!(!c.to_text().contains("____"));
        }
    }

    /// An egg that only marks the top-left cell with its letter.
    struct Mark(char);

    impl Egg for Mark {
        fn update(&mut self, _dt: f32) {}
        fn draw(&self, c: &mut Canvas, theme: &Theme) {
            c.text(0, 0, &self.0.to_string(), theme.fg);
        }
        fn done(&self) -> bool {
            false
        }
    }

    const ORDINARY: Info = Info {
        name: "ordinary",
        start: |_, _, _| Box::new(Mark('o')),
        fiesta: Kind::None,
    };
    const SEASONAL: Info = Info {
        name: "seasonal",
        start: |_, _, _| Box::new(Mark('s')),
        fiesta: Kind::Muertos,
    };

    /// Which egg `e` is playing: its mark.
    fn mark(e: &Eggs) -> Option<char> {
        let mut c = Canvas::new(4, 1, Color::Reset);
        e.draw(&mut c, &Theme::classic());
        e.active()
            .then(|| c.to_text().chars().next().unwrap_or(' '))
    }

    /// A holiday's eggs never come on other days, come most of the time on
    /// theirs, and the holiday brings eggs more often.
    #[test]
    fn holiday_eggs_keep_to_their_days() {
        let both: &[&'static Info] = &[&ORDINARY, &SEASONAL];
        let muertos = Fiesta::of_day(11, 2);
        let mut seasonal = 0;
        for _ in 0..1000 {
            let mut e = Eggs::new(80, 24);
            e.start(both, Fiesta::NONE);
            assert_eq!(mark(&e), Some('o'));
            let mut e = Eggs::new(80, 24);
            e.start(&[&SEASONAL], Fiesta::NONE);
            assert_eq!(mark(&e), None, "a holiday egg came on an ordinary day");
            let mut e = Eggs::new(80, 24);
            e.start(both, muertos);
            seasonal += usize::from(mark(&e) == Some('s'));
        }
        assert!((700..900).contains(&seasonal), "{seasonal} of 1000");
        let starts = |fiesta: Fiesta| {
            let mut e = Eggs::new(80, 24);
            e.age = 1000.0;
            let mut n = 0;
            for _ in 0..(200 * 3600) {
                e.roll(both, 1.2, 1.0, true, fiesta);
                if e.active() {
                    n += 1;
                    e.active = None;
                }
            }
            n
        };
        let (ordinary, holiday) = (starts(Fiesta::NONE), starts(muertos));
        assert!(holiday > ordinary * 4, "{ordinary} vs {holiday}");
    }

    #[test]
    fn every_egg_finishes_in_time() {
        let t = Theme::builtin("nord").unwrap();
        for info in ALL {
            for (w, h) in [(80, 24), (200, 50), (20, 8)] {
                let mut e = Eggs::new(w, h);
                // A holiday's egg starts on its holiday.
                let fiesta = Fiesta {
                    kind: info.fiesta,
                    level: 1.0,
                };
                e.start(&[info], fiesta);
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
