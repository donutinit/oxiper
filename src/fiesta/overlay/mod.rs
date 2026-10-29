//! What a holiday puts over every scene, under the easter eggs.
//!
//! Each holiday that has an overlay is a module of its own, which says what
//! it hangs and what falls (a [`Look`]); the engines that do the hanging and
//! the falling are shared: papel picado strings ([`papel`]), cords with
//! things hanging from them ([`cord`]), flecks ([`drift`]) and serpentinas
//! ([`ribbon`]).
//!
//! Día de Muertos strings papel picado across the top of the screen: cut
//! tissue-paper flags in rosa mexicano, naranja, morado, turquesa, amarillo
//! and verde limón, tied along a cord that sags a little between its ties,
//! each flag half-block pixel art with a cut-out calavera, cempasúchil,
//! dove, cross, heart, candle or lattice above a fringed edge. A breeze runs
//! along the string and the flags sway with it, darker as they turn. From
//! Todos Santos on a second string hangs lower and shorter, behind the first.
//! Cempasúchil petals drift down over everything, tumbling (their two faces
//! catch the light in turn) and swaying as they fall, more of them as the
//! holiday builds up. Under any theme but `classic` the paper takes the
//! theme's colors; the petals stay marigold.
//!
//! Everything is laid pixel by pixel over what the scene drew: a cell the
//! overlay covers by half keeps the scene's other half, the cut-outs show the
//! scene through the paper, and every other cell is left alone. The flags keep
//! to the top four rows (six with two strings).
//!
//! Navidad strings foquitos (series lights) across the top: a green cord
//! that sags between its ties, bulbs in red, green, blue, gold, rosa and
//! naranja blinking in turn (a chase, a breathing wave, an alternation, a
//! twinkle), swags of heno (Spanish moss) draped along it and esferas (glass
//! balls) hanging by a thread, more of everything as the posadas climb to
//! Nochebuena. They keep to the top five rows. During the posadas a piñata
//! of siete picos hangs from the middle of the string, swaying, which takes
//! eight rows (on a screen twenty rows tall or more); on Año Nuevo confetti
//! and serpentinas fall over everything.
//! Under any theme but `classic` the bulbs and the confetti take the theme's
//! colors and the esferas the theme's closest to red, green, gold and silver.
//!
//! On an ordinary day, and on a holiday without an overlay, it draws nothing.

mod cord;
mod drift;
mod muertos;
mod navidad;
mod papel;
mod ribbon;

use super::{Fiesta, Kind};
use crate::color::Color;
use crate::render::{Canvas, Cell};
use crate::theme::Theme;
use cord::Hung;
use drift::{Fleck, Flecks};
use navidad::Lights;
use papel::{Flag, Papel};
use ribbon::{Ribbon, Ribbons};

/// The level from which a second string is up (Todos Santos).
const LOWER_LEVEL: f32 = 0.8;
/// How tall a screen has to be for the piñata to hang from it.
const PINATA_ROWS: u16 = 20;
/// The strings, as bits.
const UPPER: u8 = 1;
const LOWER: u8 = 2;
/// The overlay's own seed: it never touches a scene's random numbers.
const SEED: u64 = 0x6d75_6572_746f_7321;

/// What a holiday hangs and drops over the scene on one of its days.
#[derive(Clone, Copy)]
struct Look {
    /// Tells one look from another (the days of a holiday may be dressed
    /// differently): everything is made anew when it changes.
    key: u8,
    /// Mixed into the seeds, so each holiday's strings differ.
    salt: u64,
    papel: Option<&'static Papel>,
    flecks: Option<&'static Flecks>,
    ribbons: Option<&'static Ribbons>,
    lights: Option<&'static Lights>,
    /// A piñata hangs from the middle of the lights.
    pinata: bool,
}

impl Look {
    /// Nothing hangs and nothing falls.
    const NONE: Look = Look {
        key: 0,
        salt: 0,
        papel: None,
        flecks: None,
        ribbons: None,
        lights: None,
        pinata: false,
    };

    /// The look of the day, if the holiday has one.
    fn of(fiesta: Fiesta) -> Option<Look> {
        match fiesta.kind {
            Kind::Muertos => Some(muertos::look(fiesta)),
            Kind::Navidad => Some(navidad::look(fiesta)),
            _ => None,
        }
    }
}

pub struct Overlay {
    w: u16,
    h: u16,
    /// Real seconds since it started (`f64`: a screensaver can run all week).
    t: f64,
    rng: fastrand::Rng,
    /// What it is made for now (nothing on an ordinary day).
    look: Option<Look>,
    /// The upper string's flags, then the lower string's.
    flags: Vec<Flag>,
    upper: usize,
    /// The holiday's level the strings are hung for.
    level: f32,
    /// Which strings have a flag over each column (`UPPER`, `LOWER`).
    cover: Vec<u8>,
    /// Columns where the upper string is tied, its two ends included.
    ties: Vec<i32>,
    /// The lower string's ends.
    lower: (i32, i32),
    /// As many flecks as the holiday's height brings; fewer are drawn before.
    petals: Vec<Fleck>,
    /// The serpentinas.
    ribbons: Vec<Ribbon>,
    /// What hangs from the cord (`ties`).
    hung: Vec<Hung>,
}

impl Overlay {
    pub fn new(w: u16, h: u16) -> Overlay {
        Overlay {
            w,
            h,
            t: 0.0,
            rng: fastrand::Rng::with_seed(SEED),
            look: None,
            flags: Vec::new(),
            upper: 0,
            level: 0.0,
            cover: Vec::new(),
            ties: Vec::new(),
            lower: (0, 0),
            petals: Vec::new(),
            ribbons: Vec::new(),
            hung: Vec::new(),
        }
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.w = w;
        self.h = h;
        if let Some(look) = self.look {
            self.hang(&look);
            self.scatter_for(&look);
        }
    }

    /// Hangs the strings across the width.
    fn hang(&mut self, look: &Look) {
        if let Some(papel) = look.papel {
            self.hang_papel(papel, look.salt);
        }
        if let Some(lights) = look.lights {
            self.hang_lights(lights, look.salt);
        }
    }

    /// Scatters what falls over the screen.
    fn scatter_for(&mut self, look: &Look) {
        if let Some(flecks) = look.flecks {
            self.scatter(flecks, look.salt);
        }
        if let Some(ribbons) = look.ribbons {
            self.scatter_ribbons(ribbons, look.salt);
        }
    }

    /// Advances by `dt` real seconds.
    pub fn update(&mut self, dt: f32, fiesta: Fiesta) {
        if !dt.is_finite() {
            return;
        }
        let dt = dt.clamp(0.0, 5.0);
        self.t += dt as f64;
        let Some(look) = Look::of(fiesta) else {
            return;
        };
        // A new look is made anew; hung again every day of the holiday.
        if self.look.map(|l| l.key) != Some(look.key) {
            self.look = Some(look);
            self.level = f32::NAN;
            self.scatter_for(&look);
        }
        if self.level != fiesta.level {
            self.level = fiesta.level;
            self.hang(&look);
        }
        if let Some(flecks) = look.flecks {
            self.fall(dt, flecks);
        }
        if let Some(ribbons) = look.ribbons {
            self.unroll(dt, ribbons);
        }
    }

    /// Draws over the scene. Nothing on an ordinary day.
    pub fn draw(&self, c: &mut Canvas, theme: &Theme, fiesta: Fiesta) {
        let Some(look) = Look::of(fiesta) else {
            return;
        };
        if self.look.map(|l| l.key) != Some(look.key) {
            return;
        }
        if let Some(flecks) = look.flecks {
            self.draw_flecks(c, theme, flecks, fiesta.level);
        }
        if let Some(ribbons) = look.ribbons {
            self.draw_ribbons(c, theme, ribbons, fiesta.level);
        }
        if let Some(papel) = look.papel {
            self.draw_papel(c, theme, papel);
        }
        if let Some(lights) = look.lights {
            self.draw_lights(c, theme, lights);
            if look.pinata && self.h >= PINATA_ROWS {
                self.draw_pinata(c, theme, lights);
            }
        }
    }
}

/// The angle at `t` seconds of something turning at `freq` radians per
/// second from `phase`.
fn wave(t: f64, freq: f32, phase: f32) -> f32 {
    ((freq as f64 * t) % std::f64::consts::TAU) as f32 + phase
}

/// A theme color that can be painted as a pixel (`Reset` would be a hole).
fn solid(c: Option<&Color>, fallback: Color) -> Color {
    match c {
        Some(&c) if c != Color::Reset => c,
        _ => fallback,
    }
}

/// The theme color closest to a classic one (the classic color itself under
/// `classic`): a Christmas ball is red under any theme.
fn like(theme: &Theme, native: Color) -> Color {
    let Color::Rgb(r, g, b) = native else {
        return native;
    };
    let dist = |c: &&Color| match **c {
        Color::Rgb(r2, g2, b2) => {
            let d = |a: u8, b: u8| (a.abs_diff(b) as u32).pow(2);
            d(r, r2) + d(g, g2) + d(b, b2)
        }
        _ => u32::MAX,
    };
    if theme.native {
        return native;
    }
    solid(theme.colors.iter().min_by_key(dist), native)
}

/// The colors a cell shows in its upper and lower half.
fn halves(cell: &Cell) -> (Color, Color) {
    match cell.ch {
        '▀' => (cell.fg, cell.bg),
        '▄' => (cell.bg, cell.fg),
        '█' => (cell.fg, cell.fg),
        _ => (cell.bg, cell.bg),
    }
}

/// Paints the pixel at column `x`, pixel row `y`, keeping the other half of
/// its cell as the scene left it. Under strict transparency, where a cell
/// cannot show two colors, it takes the whole cell instead.
fn put(c: &mut Canvas, x: i32, y: i32, col: Color) {
    if y < 0 {
        return;
    }
    let (up, down) = if y % 2 == 0 {
        (Some(col), None)
    } else {
        (None, Some(col))
    };
    let whole = c.strict
        && c.get(x, y / 2).is_some_and(|cell| {
            let (t, b) = halves(cell);
            let other = if y % 2 == 0 { b } else { t };
            other != Color::Reset && other != col
        });
    if whole {
        put2(c, x, y / 2, Some(col), Some(col));
    } else {
        put2(c, x, y / 2, up, down);
    }
}

/// Tints the pixel at column `x`, pixel row `y` toward `col` by `k`: the glow
/// of a light. It blends into the scene's own color, or into `bg` where the
/// scene left it bare, and glows nowhere when neither is a color it can mix.
fn glow(c: &mut Canvas, x: i32, y: i32, col: Color, k: f32, bg: Color) {
    let Some(cell) = c.get(x, y.div_euclid(2)) else {
        return;
    };
    let (up, down) = halves(cell);
    let old = if y.rem_euclid(2) == 0 { up } else { down };
    let old = if old == Color::Reset { bg } else { old };
    if old.is_rgb() {
        put(c, x, y, old.lerp(col, k));
    }
}

/// Paints the upper and lower pixels of a cell; `None` keeps what the scene
/// has in that half. Under strict transparency, where a cell cannot show two
/// colors, the overlay's pixel wins and the scene's half goes (the holes in a
/// flag stay holes).
fn put2(c: &mut Canvas, x: i32, row: i32, up: Option<Color>, down: Option<Color>) {
    if up.is_none() && down.is_none() {
        return;
    }
    let (bg, strict) = (c.bg(), c.strict);
    let Some(slot) = c.get_mut(x, row) else {
        return;
    };
    let (old_up, old_down) = halves(slot);
    let (mut t, mut b) = (up.unwrap_or(old_up), down.unwrap_or(old_down));
    if strict && t != b {
        match (up, down) {
            (Some(_), None) => b = Color::Reset,
            (None, Some(_)) => t = Color::Reset,
            _ => {}
        }
    }
    let (ch, fg, cbg) = match (t, b) {
        (t, Color::Reset) => ('▀', t, bg),
        (Color::Reset, b) => ('▄', b, bg),
        (t, b) if t == b => ('█', t, bg),
        (t, b) if strict => ('█', t.lerp(b, 0.5), bg),
        (t, b) => ('▀', t, b),
    };
    *slot = Cell {
        ch,
        fg,
        bg: cbg,
        attr: 0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;

    const MUERTOS: Fiesta = Fiesta {
        kind: super::super::Kind::Muertos,
        level: 1.0,
        month: 11,
        day: 2,
    };

    /// A scene's frame: half blocks, text and empty cells.
    pub(super) fn scene(w: u16, h: u16) -> Canvas {
        let mut c = Canvas::new(w, h, Color::Reset);
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let cell = match (x + 2 * y) % 3 {
                    0 => Cell {
                        ch: '▀',
                        fg: rgb(0x123456),
                        bg: rgb(0x654321),
                        attr: 0,
                    },
                    1 => Cell {
                        ch: '#',
                        fg: rgb(0x00ff00),
                        ..Cell::BLANK
                    },
                    _ => Cell::BLANK,
                };
                c.set_cell(x, y, cell);
            }
        }
        c
    }

    /// The frame after `secs` real seconds of the holiday.
    pub(super) fn after(o: &mut Overlay, fiesta: Fiesta, secs: f32, theme: &Theme) -> Canvas {
        for _ in 0..(secs * 20.0) as usize {
            o.update(0.05, fiesta);
        }
        let mut c = scene(o.w, o.h);
        o.draw(&mut c, theme, fiesta);
        c
    }

    /// Rows (from, to) holding cells the overlay changed.
    pub(super) fn changed(c: &Canvas, w: u16) -> Vec<(i32, i32)> {
        let base = scene(w, c.h());
        c.cells()
            .iter()
            .zip(base.cells())
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(i, _)| ((i % w as usize) as i32, (i / w as usize) as i32))
            .collect()
    }

    #[test]
    fn dresses_every_day_of_muertos() {
        let theme = Theme::classic();
        for (month, day) in [(10, 28), (10, 31), (11, 2), (11, 3)] {
            let fiesta = Fiesta::of_day(month, day);
            let mut o = Overlay::new(120, 36);
            let c = after(&mut o, fiesta, 4.0, &theme);
            let cells = changed(&c, 120);
            let top = cells.iter().filter(|&&(_, y)| y < 4).count();
            let lower = cells.iter().filter(|&&(_, y)| (4..6).contains(&y)).count();
            let below = cells.iter().filter(|&&(_, y)| y >= 6).count();
            assert!(top > 250, "{month}-{day}: {top} cells of papel picado");
            assert!(below > 0, "{month}-{day}: no petals");
            // The second string from Todos Santos on.
            if fiesta.level >= LOWER_LEVEL {
                assert!(lower > 60, "{month}-{day}: {lower} cells");
            } else {
                assert!(lower < 15, "{month}-{day}: {lower} cells");
            }
        }
        // More petals as the holiday builds up.
        let count = |level| {
            let mut o = Overlay::new(200, 60);
            let c = after(&mut o, Fiesta { level, ..MUERTOS }, 2.0, &theme);
            changed(&c, 200).iter().filter(|&&(_, y)| y >= 6).count()
        };
        assert!(count(1.0) > count(0.25) + 10);
    }

    #[test]
    fn nothing_on_an_ordinary_day() {
        let mut o = Overlay::new(80, 24);
        let c = after(&mut o, Fiesta::NONE, 3.0, &Theme::classic());
        assert!(changed(&c, 80).is_empty());
    }

    #[test]
    fn comes_and_goes_with_the_holiday() {
        let theme = Theme::builtin("dracula").unwrap();
        let mut o = Overlay::new(100, 30);
        let on = |o: &mut Overlay, f| !changed(&after(o, f, 1.0, &theme), 100).is_empty();
        assert!(on(&mut o, Fiesta::of_day(10, 28)));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, Fiesta::of_day(10, 29)));
        // A new day hangs the strings anew, the second one from November 1.
        o.update(0.0, Fiesta::of_day(11, 1));
        assert!(o.upper < o.flags.len());
        o.update(0.0, Fiesta::of_day(11, 3));
        assert_eq!(o.upper, o.flags.len());
    }

    #[test]
    fn papel_picado_keeps_to_its_rows() {
        let theme = Theme::classic();
        for (fiesta, rows) in [(Fiesta::of_day(10, 30), 4), (Fiesta::of_day(11, 2), 6)] {
            for w in [9, 40, 80, 121, 200] {
                let mut o = Overlay::new(w, 30);
                o.update(0.0, fiesta);
                o.petals.clear();
                for step in 0..60 {
                    o.update(0.17, fiesta);
                    let mut c = scene(w, 30);
                    o.draw(&mut c, &theme, fiesta);
                    let cells = changed(&c, w);
                    assert!(
                        cells.iter().all(|&(_, y)| y < rows),
                        "{w} wide, step {step}"
                    );
                    if w >= 40 {
                        assert!(cells.len() > w as usize, "{w} wide: {}", cells.len());
                    }
                }
            }
        }
    }

    #[test]
    fn petals_touch_only_their_cells() {
        let fiesta = Fiesta::of_day(11, 2);
        let mut o = Overlay::new(120, 40);
        o.update(0.0, fiesta);
        o.flags.clear();
        o.upper = 0;
        o.cover.fill(UPPER);
        o.ties.clear();
        for _ in 0..50 {
            o.update(0.1, fiesta);
            let mut c = scene(120, 40);
            o.draw(&mut c, &Theme::classic(), fiesta);
            // Each petal covers two pixels at most.
            let n = changed(&c, 120).len();
            assert!(n > 0 && n <= 2 * o.petals.len(), "{n} cells");
        }
    }

    #[test]
    fn keeps_the_other_half_of_a_cell() {
        let (a, b, ours) = (rgb(0x112233), rgb(0x445566), rgb(0xff8c00));
        let mut c = Canvas::new(3, 2, Color::Reset);
        c.set_cell(
            0,
            0,
            Cell {
                ch: '▀',
                fg: a,
                bg: b,
                attr: 0,
            },
        );
        put(&mut c, 0, 1, ours);
        assert_eq!(halves(c.get(0, 0).unwrap()), (a, ours));
        put(&mut c, 0, 0, ours);
        assert_eq!(halves(c.get(0, 0).unwrap()), (ours, ours));
        // Over nothing, the other half stays see-through.
        put(&mut c, 1, 2, ours);
        let cell = c.get(1, 1).unwrap();
        assert_eq!((cell.ch, cell.fg, cell.bg), ('▀', ours, Color::Reset));
        // Under strict transparency the overlay wins the cell.
        c.strict = true;
        put2(&mut c, 0, 0, None, Some(a));
        assert_eq!(halves(c.get(0, 0).unwrap()), (Color::Reset, a));
        put(&mut c, 0, 0, b);
        assert_eq!(halves(c.get(0, 0).unwrap()), (b, b));
        put2(&mut c, 0, 0, Some(ours), Some(a));
        assert_eq!(c.get(0, 0).unwrap().bg, Color::Reset);
        // Off the canvas: nothing.
        put(&mut c, -1, 0, ours);
        put(&mut c, 0, -1, ours);
        put(&mut c, 3, 0, ours);
        put(&mut c, 0, 4, ours);
        assert_eq!(c.get(2, 0), Some(&Cell::BLANK));
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
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
                for fiesta in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
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
                    o.update(5.0, fiesta);
                    o.draw(&mut c, &theme, fiesta);
                }
            }
        }
    }

    #[test]
    fn same_frames_every_time() {
        let theme = Theme::classic();
        let frame = || after(&mut Overlay::new(90, 28), MUERTOS, 6.0, &theme).to_text();
        assert_eq!(frame(), frame());
    }

    #[test]
    fn holidays_without_an_overlay_draw_nothing() {
        for kind in [
            Kind::Candelaria,
            Kind::Equinoccio,
            Kind::SantaCruz,
            Kind::Judas,
        ] {
            let fiesta = Fiesta::height(kind);
            assert!(fiesta.is(kind));
            let mut o = Overlay::new(80, 24);
            let c = after(&mut o, fiesta, 3.0, &Theme::classic());
            assert!(changed(&c, 80).is_empty(), "{kind:?}");
        }
    }
}
