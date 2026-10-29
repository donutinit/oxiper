//! Mexican holidays: which one is on today, and how far it has come.
//!
//! Scenes read `ctx.fiesta` and dress up for it, caching what they need in
//! `retheme` (which runs again whenever the holiday changes) or reading it
//! while they draw. The app adds what every scene shares: the holiday's
//! overlay ([`Overlay`]) and its visitors (the easter eggs whose `fiesta` is
//! set). On any other day, or with `fiestas = "none"`, every scene looks
//! exactly as it always has.
//!
//! The calendar, in the order the year brings it:
//!
//! | holiday | days | at its height |
//! |---|---|---|
//! | Navidad (the Guadalupe–Reyes) | December 12 to January 6 | Nochebuena |
//! | Candelaria | February 2 | |
//! | Carnaval | Thursday to Martes de Carnaval, before Ash Wednesday | Martes de Carnaval |
//! | Equinoccio | March 19 to 22 | March 20 and 21 |
//! | Quema de Judas | Viernes Santo and Sábado de Gloria | Sábado de Gloria |
//! | Día del Niño | April 30 | |
//! | Día de la Santa Cruz | May 3 | |
//! | Guelaguetza | the Saturday before the first Lunes del Cerro to the second | each Monday |
//! | Fiestas Patrias | September 1 to 16 | the Grito, the night of the 15th |
//! | Día de Muertos | October 28 to November 3 | November 2 |
//! | Día de la Revolución | November 20 | |
//!
//! Carnaval and the quema de Judas follow Easter, and the Guelaguetza the
//! Mondays after July 16. On the rare day one of them falls on a fixed
//! holiday, the movable one wins, unless the fixed one is at its height and
//! the movable one is not (Candelaria on a Friday of Carnaval).

mod overlay;

pub use overlay::Overlay;

/// A holiday.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Kind {
    #[default]
    None,
    /// Navidad, the Guadalupe–Reyes: from the Virgen de Guadalupe on December
    /// 12 through the posadas (16 to 24), Nochebuena, the Santos Inocentes
    /// (28), Año Nuevo and the Reyes Magos on January 6.
    Navidad,
    /// Candelaria, February 2: the Niño Dios is dressed and taken to be
    /// blessed, and whoever found the muñeco in the rosca pays the tamales.
    Candelaria,
    /// Carnaval, as in Veracruz and Mazatlán: from Thursday's quema del mal
    /// humor to Martes de Carnaval, the day before Ash Wednesday.
    Carnaval,
    /// The spring equinox, March 19 to 22, when the serpent of light comes
    /// down El Castillo at Chichén Itzá.
    Equinoccio,
    /// The quema de Judas: cartonería Judases hang on Viernes Santo and burst
    /// into rockets on Sábado de Gloria.
    Judas,
    /// Día del Niño, April 30.
    Nino,
    /// Día de la Santa Cruz, May 3: builders raise a flowered cross over
    /// every building site and set off rockets.
    SantaCruz,
    /// The Guelaguetza in Oaxaca: the two Lunes del Cerro after July 16 and
    /// the days between, from the Saturday before.
    Guelaguetza,
    /// Fiestas Patrias: September, the mes patrio, up to the Grito on the
    /// night of the 15th and the parade on the 16th.
    Patrias,
    /// Día de Muertos, October 28 to November 3.
    Muertos,
    /// Día de la Revolución, November 20.
    Revolucion,
}

impl Kind {
    /// Every holiday, in the order the year brings them.
    pub const ALL: [Kind; 11] = [
        Kind::Navidad,
        Kind::Candelaria,
        Kind::Carnaval,
        Kind::Equinoccio,
        Kind::Judas,
        Kind::Nino,
        Kind::SantaCruz,
        Kind::Guelaguetza,
        Kind::Patrias,
        Kind::Muertos,
        Kind::Revolucion,
    ];

    /// Its name in the `fiestas` setting.
    pub fn name(self) -> &'static str {
        match self {
            Kind::None => "none",
            Kind::Navidad => "navidad",
            Kind::Candelaria => "candelaria",
            Kind::Carnaval => "carnaval",
            Kind::Equinoccio => "equinoccio",
            Kind::Judas => "judas",
            Kind::Nino => "nino",
            Kind::SantaCruz => "santa-cruz",
            Kind::Guelaguetza => "guelaguetza",
            Kind::Patrias => "patrias",
            Kind::Muertos => "muertos",
            Kind::Revolucion => "revolucion",
        }
    }

    /// The holiday a name in the `fiestas` setting stands for.
    fn named(name: &str) -> Option<Kind> {
        let kind = match name {
            "navidad" | "guadalupe-reyes" | "posadas" => Kind::Navidad,
            "candelaria" => Kind::Candelaria,
            "carnaval" => Kind::Carnaval,
            "equinoccio" => Kind::Equinoccio,
            "judas" | "quema-de-judas" | "sabado-de-gloria" | "sábado-de-gloria" => Kind::Judas,
            "nino" | "niño" | "dia-del-nino" | "día-del-niño" => Kind::Nino,
            "santa-cruz" => Kind::SantaCruz,
            "guelaguetza" => Kind::Guelaguetza,
            "patrias" | "fiestas-patrias" | "independencia" | "grito" => Kind::Patrias,
            "muertos" | "dia-de-muertos" | "día-de-muertos" => Kind::Muertos,
            "revolucion" | "revolución" => Kind::Revolucion,
            _ => return None,
        };
        Some(kind)
    }

    /// Does it move with Easter or the Lunes del Cerro?
    fn movable(self) -> bool {
        matches!(self, Kind::Carnaval | Kind::Judas | Kind::Guelaguetza)
    }

    /// The day it is at its height in `year`.
    fn height(self, year: i32) -> (u32, u32) {
        match self {
            Kind::None => (1, 1),
            Kind::Navidad => (12, 24),
            Kind::Candelaria => (2, 2),
            Kind::Carnaval => civil(easter(year) - 47),
            Kind::Equinoccio => (3, 20),
            Kind::Judas => civil(easter(year) - 1),
            Kind::Nino => (4, 30),
            Kind::SantaCruz => (5, 3),
            Kind::Guelaguetza => civil(lunes_del_cerro(year).0),
            Kind::Patrias => (9, 15),
            Kind::Muertos => (11, 2),
            Kind::Revolucion => (11, 20),
        }
    }
}

/// The holiday on screen and how far it has come.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Fiesta {
    pub kind: Kind,
    /// How much of the holiday has arrived, 0 to 1. Día de Muertos builds up
    /// the way an ofrenda does: the first petals on October 28 for those who
    /// died by accident, the forgotten souls on the 30th, the little ones
    /// (the angelitos) on the 31st and November 1, everyone and everything
    /// on November 2, and on the 3rd the souls go back. Navidad builds up
    /// through the posadas to Nochebuena, comes back up for Año Nuevo and
    /// for Reyes; the Fiestas Patrias through September to the Grito.
    pub level: f32,
    /// The day it is, for holidays whose days differ (the posadas,
    /// Nochebuena, the Santos Inocentes, Año Nuevo and Reyes all fall in the
    /// Navidad): see [`Fiesta::inocentes`] and the others. 0 when no holiday
    /// is on.
    pub month: u8,
    pub day: u8,
}

/// Days of the year on the proleptic Gregorian calendar, as days since
/// 1970-01-01.
type Serial = i64;

impl Fiesta {
    pub const NONE: Fiesta = Fiesta {
        kind: Kind::None,
        level: 0.0,
        month: 0,
        day: 0,
    };

    /// Is this holiday on?
    pub fn is(self, kind: Kind) -> bool {
        self.kind == kind && kind != Kind::None
    }

    /// Is any holiday on?
    pub fn on(self) -> bool {
        self.kind != Kind::None
    }

    /// The day it is, (month, day); (0, 0) when no holiday is on.
    #[allow(dead_code)] // for the scenes as they dress up
    pub fn date(self) -> (u8, u8) {
        (self.month, self.day)
    }

    /// The posadas, December 16 to 24.
    #[allow(dead_code)] // for the scenes as they dress up
    pub fn posadas(self) -> bool {
        self.is(Kind::Navidad) && self.month == 12 && (16..=24).contains(&self.day)
    }

    /// Día de los Santos Inocentes, December 28: the day for pranks, each
    /// one owned up to with "¡Inocente palomita, que te dejaste engañar!".
    #[allow(dead_code)] // for the scenes as they dress up
    pub fn inocentes(self) -> bool {
        self.is(Kind::Navidad) && self.date() == (12, 28)
    }

    /// Año Nuevo: December 31 and January 1.
    #[allow(dead_code)] // for the scenes as they dress up
    pub fn ano_nuevo(self) -> bool {
        self.is(Kind::Navidad) && matches!(self.date(), (12, 31) | (1, 1))
    }

    /// Reyes: the Reyes Magos on their way, January 2 to 5, and arrived on
    /// the 6th, the day of the rosca.
    #[allow(dead_code)] // for the scenes as they dress up
    pub fn reyes(self) -> bool {
        self.is(Kind::Navidad) && self.month == 1 && (2..=6).contains(&self.day)
    }

    /// The holiday at its height, as on its biggest day (for a movable
    /// holiday, its day in 2027).
    #[allow(dead_code)] // for the scenes as they dress up
    pub fn height(kind: Kind) -> Fiesta {
        Fiesta::height_in(kind, 2027)
    }

    fn height_in(kind: Kind, year: i32) -> Fiesta {
        let (month, day) = kind.height(year);
        let f = Fiesta::of_date(year, month, day);
        if f.kind == kind { f } else { Fiesta::NONE }
    }

    /// The holiday with a fixed date on a (month, day): every one but
    /// Carnaval, the quema de Judas and the Guelaguetza, which move from
    /// year to year (see [`Fiesta::of_date`]).
    pub fn of_day(month: u32, day: u32) -> Fiesta {
        let (kind, level) = match (month, day) {
            // The Guadalupe–Reyes: the Virgen, nine posadas climbing to
            // Nochebuena, the days between, Año Nuevo and the Reyes.
            (12, 12..=15) => (Kind::Navidad, 0.3),
            (12, d @ 16..=23) => (Kind::Navidad, 0.4 + 0.07 * (d - 16) as f32),
            (12, 24) => (Kind::Navidad, 1.0),
            (12, 25) => (Kind::Navidad, 0.9),
            (12, 26..=30) => (Kind::Navidad, 0.6),
            (12, 31) => (Kind::Navidad, 0.9),
            (1, 1) => (Kind::Navidad, 0.8),
            (1, 2..=4) => (Kind::Navidad, 0.5),
            (1, 5) => (Kind::Navidad, 0.7),
            (1, 6) => (Kind::Navidad, 0.9),
            (2, 2) => (Kind::Candelaria, 1.0),
            (3, 19) | (3, 22) => (Kind::Equinoccio, 0.6),
            (3, 20 | 21) => (Kind::Equinoccio, 1.0),
            (4, 30) => (Kind::Nino, 1.0),
            (5, 3) => (Kind::SantaCruz, 1.0),
            // The mes patrio, flags going up all month, then the Grito and
            // the parade.
            (9, d @ 1..=14) => (Kind::Patrias, 0.2 + 0.03 * (d - 1) as f32),
            (9, 15) => (Kind::Patrias, 1.0),
            (9, 16) => (Kind::Patrias, 0.8),
            (10, 28) => (Kind::Muertos, 0.25),
            (10, 29) => (Kind::Muertos, 0.35),
            (10, 30) => (Kind::Muertos, 0.45),
            (10, 31) => (Kind::Muertos, 0.6),
            (11, 1) => (Kind::Muertos, 0.8),
            (11, 2) => (Kind::Muertos, 1.0),
            (11, 3) => (Kind::Muertos, 0.5),
            (11, 20) => (Kind::Revolucion, 1.0),
            _ => return Fiesta::NONE,
        };
        Fiesta {
            kind,
            level,
            month: month as u8,
            day: day as u8,
        }
    }

    /// The holiday on a date, movable holidays included.
    pub fn of_date(year: i32, month: u32, day: u32) -> Fiesta {
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return Fiesta::NONE;
        }
        let n = serial(year, month, day);
        if civil(n) != (month, day) {
            // February 30 and the like.
            return Fiesta::NONE;
        }
        let fixed = Fiesta::of_day(month, day);
        let Some((kind, level)) = movable(year, n) else {
            return fixed;
        };
        if fixed.level >= 1.0 && level < 1.0 {
            return fixed;
        }
        Fiesta {
            kind,
            level,
            month: month as u8,
            day: day as u8,
        }
    }
}

/// The movable holiday on serial day `n` of `year`, and its level.
fn movable(year: i32, n: Serial) -> Option<(Kind, f32)> {
    // Carnaval, from the quema del mal humor to Martes de Carnaval, and the
    // quema de Judas.
    const CARNAVAL: [f32; 6] = [0.4, 0.55, 0.7, 0.8, 0.9, 1.0];
    match n - easter(year) {
        d @ -52..=-47 => return Some((Kind::Carnaval, CARNAVAL[(d + 52) as usize])),
        -2 => return Some((Kind::Judas, 0.4)),
        -1 => return Some((Kind::Judas, 1.0)),
        _ => {}
    }
    // The Guelaguetza: each Monday at the height, the Saturday parade and the
    // Sunday building up to it, and the days between.
    let (first, second) = lunes_del_cerro(year);
    if !(first - 2..=second).contains(&n) {
        return None;
    }
    let level = match (n - first).rem_euclid(7) {
        0 => 1.0,
        5 => 0.6,
        6 => 0.7,
        _ => 0.5,
    };
    Some((Kind::Guelaguetza, level))
}

/// Days since 1970-01-01 of a date on the proleptic Gregorian calendar
/// (Howard Hinnant's `days_from_civil`).
fn serial(year: i32, month: u32, day: u32) -> Serial {
    let (m, d) = (month as i64, day as i64);
    let y = year as i64 - i64::from(m <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The (month, day) of a serial day (`civil_from_days`).
fn civil(n: Serial) -> (u32, u32) {
    let z = n + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (month, day)
}

/// Day of the week, 0 for Sunday.
fn weekday(n: Serial) -> i64 {
    (n + 4).rem_euclid(7)
}

/// Easter Sunday (the anonymous Gregorian computus).
fn easter(year: i32) -> Serial {
    let y = year as i64;
    let a = y.rem_euclid(19);
    let (b, c) = (y.div_euclid(100), y.rem_euclid(100));
    let (d, e) = (b / 4, b % 4);
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let (i, k) = (c / 4, c % 4);
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;
    serial(year, month as u32, day as u32)
}

/// The two Lunes del Cerro: the Mondays after July 16 (the Virgen del
/// Carmen), unless the first falls on July 18, the day Juárez died, and
/// they move a week later.
fn lunes_del_cerro(year: i32) -> (Serial, Serial) {
    let carmen = serial(year, 7, 16);
    let mut first = carmen + (8 - weekday(carmen)) % 7;
    if first == carmen {
        first += 7;
    }
    if first == serial(year, 7, 18) {
        first += 7;
    }
    (first, first + 7)
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
    /// November 2) or a holiday on one of its days (`muertos@10-31`, or
    /// `carnaval@2027-02-06` for a movable one in a given year; without the
    /// year it is this year's).
    pub fn parse(s: &str) -> Result<Setting, String> {
        let s = s.trim().to_lowercase();
        let (name, date) = match s.split_once('@') {
            Some((n, d)) => (n.trim(), Some(d.trim())),
            None => (s.as_str(), None),
        };
        let bad = || {
            let names: Vec<&str> = Kind::ALL.iter().map(|k| k.name()).collect();
            format!(
                "fiestas: unknown '{s}' (auto, none, {}, or one of them @MM-DD)",
                names.join(", ")
            )
        };
        match name {
            "auto" | "" if date.is_none() => return Ok(Setting::Auto),
            "none" | "off" if date.is_none() => return Ok(Setting::Off),
            _ => {}
        }
        let kind = Kind::named(name).ok_or_else(bad)?;
        let year = today().0;
        let Some(date) = date else {
            return Ok(Setting::Fixed(Fiesta::height_in(kind, year)));
        };
        let num = |p: &str| p.parse::<u32>().map_err(|_| bad());
        let f = match date.split('-').collect::<Vec<_>>()[..] {
            [y, m, d] => {
                let y = y.parse::<i32>().map_err(|_| bad())?;
                Fiesta::of_date(y, num(m)?, num(d)?)
            }
            [m, d] if kind.movable() => Fiesta::of_date(year, num(m)?, num(d)?),
            [m, d] => Fiesta::of_day(num(m)?, num(d)?),
            _ => return Err(bad()),
        };
        if f.kind != kind {
            return Err(format!(
                "fiestas: {date} is not one of {}'s days",
                kind.name()
            ));
        }
        Ok(Setting::Fixed(f))
    }

    /// The holiday now.
    pub fn resolve(self) -> Fiesta {
        match self {
            Setting::Auto => {
                let (year, month, day) = today();
                Fiesta::of_date(year, month, day)
            }
            Setting::Off => Fiesta::NONE,
            Setting::Fixed(f) => f,
        }
    }
}

/// Today's (year, month, day) by the local clock.
fn today() -> (i32, u32, u32) {
    // SAFETY: `time` with a null pointer only returns the time; `localtime_r`
    // writes into the `tm` we own and returns null on failure.
    unsafe {
        let t = libc::time(std::ptr::null_mut());
        let mut tm: libc::tm = std::mem::zeroed();
        if libc::localtime_r(&t, &mut tm).is_null() {
            return (1970, 1, 1);
        }
        (tm.tm_year + 1900, (tm.tm_mon + 1) as u32, tm.tm_mday as u32)
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
    fn the_posadas_climb_to_nochebuena() {
        let posadas: Vec<f32> = (16..=24).map(|d| Fiesta::of_day(12, d).level).collect();
        assert!(posadas.windows(2).all(|p| p[0] < p[1]), "{posadas:?}");
        assert_eq!(Fiesta::of_day(12, 24).level, 1.0);
        assert_eq!(Fiesta::of_day(12, 11), Fiesta::NONE);
        assert_eq!(Fiesta::of_day(1, 7), Fiesta::NONE);
        for (m, d) in [(12, 12), (12, 28), (1, 1), (1, 6)] {
            assert!(Fiesta::of_day(m, d).is(Kind::Navidad), "{m}-{d}");
        }
        let f = |m, d| Fiesta::of_day(m, d);
        assert!(f(12, 28).inocentes() && !f(12, 27).inocentes());
        assert!(f(12, 31).ano_nuevo() && f(1, 1).ano_nuevo() && !f(1, 2).ano_nuevo());
        assert!(f(1, 6).reyes() && f(1, 2).reyes() && !f(1, 1).reyes());
        assert!(f(12, 16).posadas() && f(12, 24).posadas() && !f(12, 25).posadas());
        assert!(!Fiesta::height(Kind::Muertos).posadas());
    }

    #[test]
    fn the_mes_patrio_climbs_to_the_grito() {
        let month: Vec<f32> = (1..=15).map(|d| Fiesta::of_day(9, d).level).collect();
        assert!(month.windows(2).all(|p| p[0] < p[1]), "{month:?}");
        assert_eq!(Fiesta::of_day(9, 15).level, 1.0);
        assert!(Fiesta::of_day(9, 16).is(Kind::Patrias));
        assert_eq!(Fiesta::of_day(9, 17), Fiesta::NONE);
        assert_eq!(Fiesta::of_day(8, 31), Fiesta::NONE);
    }

    #[test]
    fn easter_and_what_follows_it() {
        for (y, m, d) in [
            (2000, 4, 23),
            (2024, 3, 31),
            (2025, 4, 20),
            (2026, 4, 5),
            (2027, 3, 28),
            (2038, 4, 25),
            (2285, 3, 22),
        ] {
            assert_eq!(easter(y), serial(y, m, d), "{y}");
        }
        // 2026: Ash Wednesday is February 18.
        assert!(Fiesta::of_date(2026, 2, 12).is(Kind::Carnaval));
        assert_eq!(Fiesta::of_date(2026, 2, 17).level, 1.0);
        assert!(Fiesta::of_date(2026, 2, 17).is(Kind::Carnaval));
        assert!(!Fiesta::of_date(2026, 2, 18).on());
        assert!(!Fiesta::of_date(2026, 2, 11).on());
        // Sábado de Gloria, April 4.
        assert_eq!(Fiesta::of_date(2026, 4, 4).kind, Kind::Judas);
        assert_eq!(Fiesta::of_date(2026, 4, 4).level, 1.0);
        assert_eq!(Fiesta::of_date(2026, 4, 3).kind, Kind::Judas);
        assert!(!Fiesta::of_date(2026, 4, 5).on());
        // The fixed holidays don't move.
        assert_eq!(Fiesta::of_date(2026, 11, 2), Fiesta::of_day(11, 2));
        assert_eq!(Fiesta::of_date(2031, 12, 24), Fiesta::of_day(12, 24));
    }

    #[test]
    fn the_lunes_del_cerro() {
        let mondays = |y| {
            let (a, b) = lunes_del_cerro(y);
            (civil(a), civil(b))
        };
        assert_eq!(mondays(2024), ((7, 22), (7, 29)));
        assert_eq!(mondays(2025), ((7, 21), (7, 28)));
        assert_eq!(mondays(2026), ((7, 20), (7, 27)));
        // 2022: the first would have been July 18.
        assert_eq!(mondays(2022), ((7, 25), (8, 1)));
        let g = |m, d| Fiesta::of_date(2026, m, d);
        assert_eq!((g(7, 20).kind, g(7, 20).level), (Kind::Guelaguetza, 1.0));
        assert_eq!(g(7, 27).level, 1.0);
        assert_eq!(g(7, 18).level, 0.6);
        assert_eq!(g(7, 22).level, 0.5);
        assert!(!g(7, 17).on() && !g(7, 28).on());
    }

    #[test]
    fn every_holiday_has_its_height() {
        for kind in Kind::ALL {
            let f = Fiesta::height(kind);
            assert_eq!((f.kind, f.level), (kind, 1.0), "{kind:?}");
            assert_eq!(Kind::named(kind.name()), Some(kind));
            for y in 1990..2100 {
                assert_eq!(Fiesta::height_in(kind, y).level, 1.0, "{kind:?} {y}");
            }
        }
        // 2035: Carnaval's Friday is February 2, and Candelaria keeps it.
        assert_eq!(Fiesta::of_date(2035, 2, 2).kind, Kind::Candelaria);
        assert_eq!(Fiesta::of_date(2035, 2, 1).kind, Kind::Carnaval);
        assert_eq!(Fiesta::of_date(2027, 2, 30), Fiesta::NONE);
        // Every day of a few years is at most one holiday, with a level.
        for y in [2026, 2027, 2038] {
            let (mut n, end) = (serial(y, 1, 1), serial(y + 1, 1, 1));
            while n < end {
                let (m, d) = civil(n);
                let f = Fiesta::of_date(y, m, d);
                assert!(f.level > 0.0 && f.level <= 1.0 || !f.on(), "{y}-{m}-{d}");
                assert!(!f.on() || f.date() == (m as u8, d as u8));
                n += 1;
            }
        }
    }

    #[test]
    fn dates_go_there_and_back() {
        let mut last = serial(1999, 12, 31);
        for y in 2000..2031 {
            for m in 1..=12 {
                for d in 1..=31 {
                    let n = serial(y, m, d);
                    if civil(n) != (m, d) {
                        continue; // no such day
                    }
                    assert_eq!(n, last + 1, "{y}-{m}-{d}");
                    last = n;
                }
            }
        }
        assert_eq!(weekday(serial(2026, 10, 29)), 4);
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
        assert_eq!(
            Setting::parse("Navidad@12-28"),
            Ok(Setting::Fixed(Fiesta::of_day(12, 28)))
        );
        assert_eq!(
            Setting::parse("posadas"),
            Ok(Setting::Fixed(Fiesta::of_day(12, 24)))
        );
        assert_eq!(
            Setting::parse("grito"),
            Ok(Setting::Fixed(Fiesta::of_day(9, 15)))
        );
        assert_eq!(
            Setting::parse("carnaval@2026-02-14"),
            Ok(Setting::Fixed(Fiesta::of_date(2026, 2, 14)))
        );
        for kind in Kind::ALL {
            let Ok(Setting::Fixed(f)) = Setting::parse(kind.name()) else {
                panic!("{kind:?}");
            };
            assert_eq!((f.kind, f.level), (kind, 1.0));
        }
        assert!(Setting::parse("muertos@12-25").is_err());
        assert!(Setting::parse("carnaval@2026-12-25").is_err());
        assert!(Setting::parse("halloween").is_err());
        assert!(Setting::parse("muertos@11").is_err());
        assert!(Setting::parse("muertos@aa-bb").is_err());
        assert_eq!(Setting::Off.resolve(), Fiesta::NONE);
    }
}
