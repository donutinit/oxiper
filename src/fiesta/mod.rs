//! Mexican holidays: which one is on today, and how far it has come.
//!
//! Scenes read `ctx.fiesta` and dress up for it, caching what they need in
//! `retheme` (which runs again whenever the holiday changes) or reading it
//! while they draw. The app adds what every scene shares: the holiday's
//! overlay ([`Overlay`]) and its visitors (the easter eggs whose `fiesta` is
//! set). On any other day, or with `fiestas = "none"`, every scene looks
//! exactly as it always has.

mod overlay;

pub use overlay::Overlay;

/// A holiday.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Kind {
    #[default]
    None,
    /// Día de Muertos, October 28 to November 3.
    Muertos,
}

/// The holiday on screen and how far it has come.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Fiesta {
    pub kind: Kind,
    /// How much of the holiday has arrived, 0 to 1. Día de Muertos builds up
    /// the way an ofrenda does: the first petals on October 28 for those who
    /// died by accident, the forgotten souls on the 30th, the little ones
    /// (the angelitos) on the 31st and November 1, everyone and everything
    /// on November 2, and on the 3rd the souls go back.
    pub level: f32,
}

impl Fiesta {
    pub const NONE: Fiesta = Fiesta {
        kind: Kind::None,
        level: 0.0,
    };

    /// Is this holiday on?
    #[allow(dead_code)] // scene API (docs/SCENES.md)
    pub fn is(self, kind: Kind) -> bool {
        self.kind == kind && kind != Kind::None
    }

    /// Is any holiday on?
    pub fn on(self) -> bool {
        self.kind != Kind::None
    }

    /// The holiday on a (month, day).
    pub fn of_day(month: u32, day: u32) -> Fiesta {
        let muertos = |level| Fiesta {
            kind: Kind::Muertos,
            level,
        };
        match (month, day) {
            (10, 28) => muertos(0.25),
            (10, 29) => muertos(0.35),
            (10, 30) => muertos(0.45),
            (10, 31) => muertos(0.6),
            (11, 1) => muertos(0.8),
            (11, 2) => muertos(1.0),
            (11, 3) => muertos(0.5),
            _ => Fiesta::NONE,
        }
    }
}

/// The `fiestas` setting.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Setting {
    /// Follow the local date.
    Auto,
    Off,
    /// Always this holiday, as on one of its days.
    Fixed(Fiesta),
}

impl Setting {
    /// Parses `auto`, `none`, a holiday at its height (`muertos`, as on
    /// November 2) or a holiday on one of its days (`muertos@10-31`).
    pub fn parse(s: &str) -> Result<Setting, String> {
        let s = s.trim().to_lowercase();
        let (name, date) = match s.split_once('@') {
            Some((n, d)) => (n.trim(), Some(d.trim())),
            None => (s.as_str(), None),
        };
        let bad = || format!("fiestas: unknown '{s}' (auto, none, muertos or muertos@MM-DD)");
        let (kind, height) = match name {
            "auto" | "" if date.is_none() => return Ok(Setting::Auto),
            "none" | "off" if date.is_none() => return Ok(Setting::Off),
            "muertos" | "dia-de-muertos" | "día-de-muertos" => (Kind::Muertos, (11, 2)),
            _ => return Err(bad()),
        };
        let (month, day) = match date {
            None => height,
            Some(d) => {
                let (m, d) = d.split_once('-').ok_or_else(bad)?;
                (m.parse().map_err(|_| bad())?, d.parse().map_err(|_| bad())?)
            }
        };
        let f = Fiesta::of_day(month, day);
        if f.kind != kind {
            return Err(format!(
                "fiestas: {month:02}-{day:02} is not one of its days"
            ));
        }
        Ok(Setting::Fixed(f))
    }

    /// The holiday now.
    pub fn resolve(self) -> Fiesta {
        match self {
            Setting::Auto => {
                let (month, day) = today();
                Fiesta::of_day(month, day)
            }
            Setting::Off => Fiesta::NONE,
            Setting::Fixed(f) => f,
        }
    }
}

/// Today's (month, day) by the local clock.
fn today() -> (u32, u32) {
    // SAFETY: `time` with a null pointer only returns the time; `localtime_r`
    // writes into the `tm` we own and returns null on failure.
    unsafe {
        let t = libc::time(std::ptr::null_mut());
        let mut tm: libc::tm = std::mem::zeroed();
        if libc::localtime_r(&t, &mut tm).is_null() {
            return (1, 1);
        }
        ((tm.tm_mon + 1) as u32, tm.tm_mday as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn muertos_builds_up_and_goes() {
        assert_eq!(Fiesta::of_day(10, 27), Fiesta::NONE);
        assert_eq!(Fiesta::of_day(11, 4), Fiesta::NONE);
        let days = [(10, 28), (10, 29), (10, 30), (10, 31), (11, 1), (11, 2)];
        let levels: Vec<f32> = days
            .iter()
            .map(|&(m, d)| Fiesta::of_day(m, d).level)
            .collect();
        assert!(levels.windows(2).all(|p| p[0] < p[1]), "{levels:?}");
        assert_eq!(Fiesta::of_day(11, 2).level, 1.0);
        assert!(Fiesta::of_day(11, 3).is(Kind::Muertos));
        assert!(!Fiesta::NONE.on() && !Fiesta::NONE.is(Kind::None));
    }

    #[test]
    fn parses_the_setting() {
        assert_eq!(Setting::parse("auto"), Ok(Setting::Auto));
        assert_eq!(Setting::parse(" None "), Ok(Setting::Off));
        assert_eq!(
            Setting::parse("muertos"),
            Ok(Setting::Fixed(Fiesta::of_day(11, 2)))
        );
        assert_eq!(
            Setting::parse("muertos@10-31"),
            Ok(Setting::Fixed(Fiesta::of_day(10, 31)))
        );
        assert!(Setting::parse("muertos@12-25").is_err());
        assert!(Setting::parse("navidad").is_err());
        assert!(Setting::parse("muertos@11").is_err());
        assert_eq!(Setting::Off.resolve(), Fiesta::NONE);
    }
}
