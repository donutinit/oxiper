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

pub mod adelitas;
pub mod aguila;
pub mod alebrije;
pub mod calaverita;
pub mod catrina;
pub mod duckpond;
pub mod grito;
pub mod jarabe;
pub mod mariachis;
pub mod monarcas;
mod muertos;
pub mod nyan;
pub(crate) mod palette;
mod patrias;
mod revolucion;
pub mod sl;
mod stage;
pub mod tren;

use crate::color::Color;
use crate::fiesta::{Fiesta, Kind};
use crate::render::{Canvas, Cell};
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
    /// Which of the holiday's days it comes on ([`any_day`] for all of
    /// them): the Reyes Magos only from January 2, a prank only on the
    /// Santos Inocentes.
    pub when: fn(Fiesta) -> bool,
}

/// For [`Info::when`]: every day of its holiday.
pub fn any_day(_: Fiesta) -> bool {
    true
}

pub const ALL: &[&Info] = &[
    &sl::INFO,
    &duckpond::INFO,
    &nyan::INFO,
    &catrina::INFO,
    &monarcas::INFO,
    &alebrije::INFO,
    &mariachis::INFO,
    &calaverita::INFO,
    &grito::INFO,
    &jarabe::INFO,
    &aguila::INFO,
    &adelitas::INFO,
    &tren::INFO,
];

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

/// The colors a cell shows in its upper and lower half.
pub(crate) fn halves(cell: &Cell) -> (Color, Color) {
    match cell.ch {
        '▀' => (cell.fg, cell.bg),
        '▄' => (cell.bg, cell.fg),
        '█' => (cell.fg, cell.fg),
        _ => (cell.bg, cell.bg),
    }
}

/// Paints the upper and lower half of a cell as half-block pixels; a `None`
/// half keeps what the cell showed there, so an egg covering a cell by half
/// leaves the scene's other half alone.
pub(crate) fn paint(c: &mut Canvas, col: i32, row: i32, up: Option<Color>, down: Option<Color>) {
    if up.is_none() && down.is_none() {
        return;
    }
    let (bg, strict) = (c.bg(), c.strict);
    let Some(slot) = c.get_mut(col, row) else {
        return;
    };
    let (old_up, old_down) = halves(slot);
    let (t, b) = (up.unwrap_or(old_up), down.unwrap_or(old_down));
    let cell = |ch, fg, bg| Cell {
        ch,
        fg,
        bg,
        attr: 0,
    };
    *slot = match (t, b) {
        (t, Color::Reset) => cell('▀', t, bg),
        (Color::Reset, b) => cell('▄', b, bg),
        (t, b) if t == b => cell('█', t, bg),
        (t, b) if strict => cell('█', t.lerp(b, 0.5), bg),
        (t, b) => cell('▀', t, b),
    };
}

/// Paints one half-block pixel (two to a cell, so `y` counts half rows).
pub(crate) fn pixel(c: &mut Canvas, x: i32, y: i32, color: Color) {
    let row = y.div_euclid(2);
    if y.rem_euclid(2) == 0 {
        paint(c, x, row, Some(color), None);
    } else {
        paint(c, x, row, None, Some(color));
    }
}

/// Draws pixel art, one character per half-block pixel, with its top-left
/// pixel at (`x`, `y`); `.` and spaces are see-through and `color` gives the
/// rest their colors. `flip` mirrors it (the art faces right) and `shift`
/// moves each of its rows sideways, for a skirt or a feather that sways.
pub(crate) fn sprite(
    c: &mut Canvas,
    (x, y): (i32, i32),
    rows: &[&str],
    flip: bool,
    shift: impl Fn(i32) -> i32,
    color: impl Fn(u8) -> Option<Color>,
) {
    let w = rows.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
    let h = rows.len() as i32;
    let at = |r: i32, col: i32| -> Option<Color> {
        let line = rows.get(usize::try_from(r).ok()?)?.as_bytes();
        let sx = col - x - shift(r);
        if !(0..w).contains(&sx) {
            return None;
        }
        match *line.get((if flip { w - 1 - sx } else { sx }) as usize)? {
            b'.' | b' ' => None,
            ch => color(ch),
        }
    };
    let rows_on_screen = y.div_euclid(2).max(0)..=(y + h - 1).div_euclid(2).min(c.h() as i32 - 1);
    for row in rows_on_screen {
        let (r0, r1) = (row * 2 - y, row * 2 + 1 - y);
        let span = |r: i32| (0..h).contains(&r).then(|| x + shift(r));
        let (lo, hi) = match (span(r0), span(r1)) {
            (Some(a), Some(b)) => (a.min(b), a.max(b) + w),
            (Some(a), None) | (None, Some(a)) => (a, a + w),
            (None, None) => continue,
        };
        for col in lo.max(0)..hi.min(c.w() as i32) {
            paint(c, col, row, at(r0, col), at(r1, col));
        }
    }
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
        let ours = |i: &&&Info| fiesta.on() && i.fiesta == fiesta.kind && (i.when)(fiesta);
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
    use crate::color::{Color, rgb};

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
        when: any_day,
    };
    const SEASONAL: Info = Info {
        name: "seasonal",
        start: |_, _, _| Box::new(Mark('s')),
        fiesta: Kind::Muertos,
        when: any_day,
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

    /// Half-block pixels keep what the other half of their cell showed, and
    /// nothing outside the screen is touched.
    #[test]
    fn pixels_keep_the_other_half() {
        let (a, b) = (rgb(0xff8c00), rgb(0x0000ff));
        let mut c = Canvas::new(3, 2, Color::Reset);
        pixel(&mut c, 1, 0, a);
        assert_eq!(halves(c.get(1, 0).unwrap()), (a, Color::Reset));
        pixel(&mut c, 1, 1, b);
        assert_eq!(halves(c.get(1, 0).unwrap()), (a, b));
        pixel(&mut c, 1, 0, b);
        assert_eq!(halves(c.get(1, 0).unwrap()), (b, b));
        // Off the screen, or nothing to paint: no change, no panic.
        pixel(&mut c, -1, 0, a);
        pixel(&mut c, 3, 1, a);
        pixel(&mut c, 0, 4, a);
        paint(&mut c, 0, 0, None, None);
        assert!(c.get(0, 0).unwrap().ch == ' ' && c.get(2, 1).unwrap().ch == ' ');
        // A strict canvas has one color per cell.
        let mut c = Canvas::new(1, 1, Color::Reset);
        c.strict = true;
        paint(&mut c, 0, 0, Some(a), Some(b));
        assert_eq!(c.get(0, 0).unwrap().ch, '█');
    }

    /// Sprites paint only their own pixels, mirrored when asked, each row
    /// shifted as told.
    #[test]
    fn sprites_flip_shift_and_clip() {
        let red = rgb(0xff0000);
        let mut c = Canvas::new(6, 2, Color::Reset);
        let art = ["R.", "RR"];
        sprite(&mut c, (1, 0), &art, false, |_| 0, |_| Some(red));
        assert_eq!(halves(c.get(1, 0).unwrap()), (red, red));
        assert_eq!(halves(c.get(2, 0).unwrap()), (Color::Reset, red));
        c.clear();
        // Mirrored, the bare pixel is on the left; shifted, row 1 moves right.
        sprite(&mut c, (1, 0), &art, true, |r| r, |_| Some(red));
        assert_eq!(halves(c.get(1, 0).unwrap()), (Color::Reset, Color::Reset));
        assert_eq!(halves(c.get(2, 0).unwrap()), (red, red));
        assert_eq!(halves(c.get(3, 0).unwrap()), (Color::Reset, red));
        // Partly or wholly off the screen is fine.
        for (x, y) in [(-5, -5), (-1, 0), (5, 1), (9, 9), (0, -1)] {
            sprite(&mut c, (x, y), &art, true, |r| r, |_| Some(red));
        }
    }

    /// The Día de Muertos visitors come during the holiday, from its first
    /// day, and never on an ordinary day.
    #[test]
    fn the_visitors_keep_to_muertos() {
        let names = ["catrina", "monarcas", "alebrije", "mariachis", "calaverita"];
        for name in names {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            assert_eq!(info.fiesta, Kind::Muertos, "{name}");
            let mut e = Eggs::new(80, 24);
            e.start(&[info], Fiesta::NONE);
            assert!(!e.active(), "{name} came on an ordinary day");
            for (m, d) in [(10, 28), (11, 2), (11, 3)] {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], Fiesta::of_day(m, d));
                assert!(e.active(), "{name} on {m}-{d}");
            }
        }
    }

    /// A holiday's visitors come on its days (here: from its first to its
    /// last), and never on an ordinary day or another holiday.
    fn keep_to(names: &[&str], kind: Kind, days: &[(u32, u32)]) {
        let elsewhere = [(6, 15), (8, 20), (11, 2), (12, 24), (11, 20), (9, 15)];
        for name in names {
            let info = ALL.iter().find(|i| i.name == *name).unwrap();
            assert_eq!(info.fiesta, kind, "{name}");
            let mut e = Eggs::new(80, 24);
            e.start(&[info], Fiesta::NONE);
            assert!(!e.active(), "{name} came on an ordinary day");
            for (m, d) in elsewhere {
                let day = Fiesta::of_day(m, d);
                if day.kind == kind {
                    continue;
                }
                let mut e = Eggs::new(80, 24);
                e.start(&[info], day);
                assert!(!e.active(), "{name} came on {m}-{d}");
            }
            for &(m, d) in days {
                let mut e = Eggs::new(80, 24);
                e.start(&[info], Fiesta::of_day(m, d));
                assert!(e.active(), "{name} on {m}-{d}");
            }
        }
    }

    /// The Fiestas Patrias visitors come during the mes patrio, from the
    /// first of September to the night of the Grito and the parade after it.
    #[test]
    fn the_visitors_keep_to_patrias() {
        keep_to(
            &["grito", "jarabe", "aguila"],
            Kind::Patrias,
            &[(9, 1), (9, 15), (9, 16)],
        );
    }

    /// The Día de la Revolución visitors come on the 20th of November, and
    /// never on an ordinary day or another holiday.
    #[test]
    fn the_visitors_keep_to_the_revolucion() {
        keep_to(&["adelitas", "tren"], Kind::Revolucion, &[(11, 20)]);
    }

    /// Any egg survives tiny and huge screens and long steps, under any theme.
    #[test]
    fn eggs_survive_odd_sizes_and_huge_steps() {
        let themes = [Theme::classic(), Theme::builtin("gruvbox").unwrap()];
        let mut rng = fastrand::Rng::with_seed(9);
        for info in ALL {
            for t in &themes {
                for (w, h) in [(1, 1), (2, 40), (7, 3), (40, 12), (300, 90), (500, 150)] {
                    let mut e = (info.start)(w, h, &mut rng);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for dt in [0.0, 5.0, 0.01, 5.0, 3.0, 5.0, 5.0, 5.0, 5.0] {
                        e.update(dt);
                        c.clear();
                        e.draw(&mut c, t);
                    }
                    // The visitors are done within 30 s at any size (the
                    // scheduler cuts the others short).
                    if info.fiesta != Kind::None {
                        let mut secs = 0.0;
                        while !e.done() && secs <= 30.0 {
                            e.update(0.25);
                            secs += 0.25;
                            c.clear();
                            e.draw(&mut c, t);
                        }
                        assert!(secs <= 30.0, "{} still going at {w}x{h}", info.name);
                    }
                }
            }
        }
    }

    /// A day an egg comes on: any day for the everyday ones, its holiday's
    /// height, or else the first of its days that it comes on.
    pub(crate) fn its_day(info: &Info) -> Fiesta {
        if info.fiesta == Kind::None {
            return Fiesta::NONE;
        }
        let height = Fiesta::height(info.fiesta);
        if (info.when)(height) {
            return height;
        }
        (1..=12)
            .flat_map(|m| (1..=31).map(move |d| Fiesta::of_date(2027, m, d)))
            .find(|f| f.kind == info.fiesta && (info.when)(*f))
            .unwrap_or_else(|| panic!("{} never comes", info.name))
    }

    #[test]
    fn every_egg_finishes_in_time() {
        let t = Theme::builtin("nord").unwrap();
        for info in ALL {
            for (w, h) in [(80, 24), (200, 50), (20, 8)] {
                let mut e = Eggs::new(w, h);
                e.start(&[info], its_day(info));
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
