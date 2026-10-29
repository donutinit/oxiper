//! What a holiday puts over every scene, under the easter eggs.
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
//! to the top four rows (six with two strings). On an ordinary day it draws
//! nothing.

use super::{Fiesta, Kind};
use crate::color::{Color, rgb};
use crate::render::{Canvas, Cell};
use crate::theme::Theme;

/// Flag size in pixels (one pixel is half a cell).
const FLAG_W: i32 = 7;
const FLAG_H: i32 = 8;
/// Columns between two flags of a string, and of the upper string when the
/// lower one's flags hang in between.
const GAP: i32 = 2;
const GAP_TWO: i32 = 9;
/// Pixel row where the lower string's flags hang.
const LOWER_TOP: i32 = 4;
/// The level from which the second string is up (Todos Santos).
const LOWER_LEVEL: f32 = 0.8;
/// The strings, as bits.
const UPPER: u8 = 1;
const LOWER: u8 = 2;
/// About this many columns between two ties of the upper string.
const SWAG: i32 = 44;
/// Cells per petal at the holiday's height.
const CELLS_PER_PETAL: i32 = 240;
/// The overlay's own seed: it never touches a scene's random numbers.
const SEED: u64 = 0x6d75_6572_746f_7321;

/// The cut-outs, `X` for paper and `.` for a hole: a folded hem, five rows
/// of design, a border and the fringe.
const DESIGNS: &[[&str; 8]] = &[
    // calavera
    [
        "XXXXXXX", "XX...XX", "X.X.X.X", "X.....X", "XX.X.XX", "XX...XX", "XXXXXXX", "X.X.X.X",
    ],
    // calavera con quijada
    [
        "XXXXXXX", "XX...XX", "X.....X", "X.X.X.X", "XX...XX", "XX.X.XX", "XXXXXXX", "X.X.X.X",
    ],
    // cempasúchil
    [
        "XXXXXXX", "XX...XX", "X..X..X", "X.XXX.X", "X..X..X", "XX...XX", "XXXXXXX", "X.X.X.X",
    ],
    // paloma
    [
        "XXXXXXX", "XXXXXXX", "X..XXXX", "XX....X", "XXX..XX", "XXXXXXX", "XXXXXXX", "X.X.X.X",
    ],
    // cruz
    [
        "XXXXXXX", "XXX.XXX", "X.....X", "XXX.XXX", "XXX.XXX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
    // corazón
    [
        "XXXXXXX", "XXXXXXX", "XX.X.XX", "X.....X", "XX...XX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
    // veladora
    [
        "XXXXXXX", "XXX.XXX", "XX...XX", "XXX.XXX", "XX...XX", "XX...XX", "XXXXXXX", "X.X.X.X",
    ],
    // celosía
    [
        "XXXXXXX", "X.X.X.X", "XX.X.XX", "X.X.X.X", "XX.X.XX", "X.X.X.X", "XXXXXXX", "X.X.X.X",
    ],
    // rombo
    [
        "XXXXXXX", "XXX.XXX", "XX.X.XX", "X.XXX.X", "XX.X.XX", "XXX.XXX", "XXXXXXX", "X.X.X.X",
    ],
];

/// Papel picado colors under `classic`.
const PAPEL: [Color; 6] = [
    rgb(0xe4007c), // rosa mexicano
    rgb(0xff8c00), // naranja
    rgb(0x7028a0), // morado
    rgb(0x00aab8), // turquesa
    rgb(0xffd200), // amarillo
    rgb(0x8cc828), // verde limón
];
/// The cord, natural string.
const CORD: Color = rgb(0xbcae94);
/// Cempasúchil petals: the face in the light, the face in shade, and the
/// golden ones.
const PETAL_LIGHT: Color = rgb(0xffaa1e);
const PETAL_DARK: Color = rgb(0xd66200);
const GOLD_LIGHT: Color = rgb(0xffd23c);
const GOLD_DARK: Color = rgb(0xe09a00);

struct Flag {
    /// Left column.
    x: i32,
    /// Top pixel row.
    top: i32,
    design: u8,
    color: u8,
    /// A little of its own in the way it sways.
    phase: f32,
}

struct Petal {
    /// Column the sway centers on, and the pixel row.
    x: f32,
    y: f32,
    /// Pixels per second down, at the bottom of the swing.
    fall: f32,
    /// Columns either side, and radians per second.
    sway: f32,
    freq: f32,
    phase: f32,
    /// Radians per second it tumbles at.
    spin: f32,
    gold: bool,
}

pub struct Overlay {
    w: u16,
    h: u16,
    /// Real seconds since it started (`f64`: a screensaver can run all week).
    t: f64,
    rng: fastrand::Rng,
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
    /// As many petals as the holiday's height brings; fewer are drawn before.
    petals: Vec<Petal>,
}

impl Overlay {
    pub fn new(w: u16, h: u16) -> Overlay {
        let mut o = Overlay {
            w,
            h,
            t: 0.0,
            rng: fastrand::Rng::with_seed(SEED),
            flags: Vec::new(),
            upper: 0,
            level: 0.0,
            cover: Vec::new(),
            ties: Vec::new(),
            lower: (0, 0),
            petals: Vec::new(),
        };
        o.hang();
        o.scatter();
        o
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.w = w;
        self.h = h;
        self.hang();
        self.scatter();
    }

    /// Hangs the strings across the width: one, or two staggered.
    fn hang(&mut self) {
        let w = self.w as i32;
        let mut rng =
            fastrand::Rng::with_seed(SEED ^ (w as u64) << 32 ^ self.level.to_bits() as u64);
        let two = self.level >= LOWER_LEVEL;
        self.flags.clear();
        self.ties.clear();

        // The upper string, centered, its flags evenly spaced; wider apart
        // when the lower string's hang between them.
        let gap = if two { GAP_TWO } else { GAP };
        let pitch = FLAG_W + gap;
        let n = ((w + gap) / pitch).max(0);
        let x0 = (w - (n * pitch - gap)) / 2;
        // The lower string, shorter, has its flags in the upper one's gaps.
        let inset = (w / 10).max(4);
        self.lower = (inset, w - 1 - inset);
        let mut lower = Vec::new();
        // Neighbors along the zigzag never share a design or a color.
        let (mut designs, mut colors) = ([u8::MAX; 2], [u8::MAX; 2]);
        let mut add = |flags: &mut Vec<Flag>, x: i32, top: i32| {
            let design = other(&mut rng, designs, DESIGNS.len());
            let color = other(&mut rng, colors, PAPEL.len());
            designs = [designs[1], design];
            colors = [colors[1], color];
            flags.push(Flag {
                x,
                top,
                design,
                color,
                phase: rng.f32() * std::f32::consts::TAU,
            });
        };
        for i in 0..n {
            let x = x0 + i * pitch;
            add(&mut self.flags, x, 0);
            let x = x + FLAG_W + (gap - FLAG_W) / 2;
            if two && x >= self.lower.0 && x + FLAG_W <= self.lower.1 {
                add(&mut lower, x, LOWER_TOP);
            }
        }
        self.upper = self.flags.len();
        self.flags.append(&mut lower);
        self.cover.clear();
        self.cover.resize(w.max(0) as usize, 0);
        for (i, f) in self.flags.iter().enumerate() {
            let which = if i < self.upper { UPPER } else { LOWER };
            for x in f.x..f.x + FLAG_W {
                if let Some(m) = usize::try_from(x).ok().and_then(|x| self.cover.get_mut(x)) {
                    *m |= which;
                }
            }
        }

        // Tied at both ends and in gaps between flags, about a swag apart.
        self.ties.push(0);
        let swags = ((w as f32 / SWAG as f32).round() as i32).max(1);
        for k in 1..swags {
            let want = k * w / swags;
            let i = ((want - x0 + gap) as f32 / pitch as f32).round() as i32;
            let x = x0 + i * pitch - gap / 2 - 1;
            if x > self.ties[self.ties.len() - 1] + 2 && x < w - 3 {
                self.ties.push(x);
            }
        }
        self.ties.push((w - 1).max(0));
    }

    /// Scatters the petals over the screen, as many as the holiday's height
    /// brings.
    fn scatter(&mut self) {
        let (w, h) = (self.w as i32, self.h as i32);
        self.rng = fastrand::Rng::with_seed(SEED ^ ((w as u64) << 16 | h as u64));
        let most = (w * h / CELLS_PER_PETAL).max(1) as usize;
        self.petals.clear();
        for _ in 0..most {
            let mut p = petal(&mut self.rng, w);
            p.y = self.rng.f32() * (h * 2) as f32;
            self.petals.push(p);
        }
    }

    /// Advances by `dt` real seconds.
    pub fn update(&mut self, dt: f32, fiesta: Fiesta) {
        if !dt.is_finite() {
            return;
        }
        let dt = dt.clamp(0.0, 5.0);
        self.t += dt as f64;
        if !fiesta.is(Kind::Muertos) {
            return;
        }
        // Hung anew every day of the holiday.
        if self.level != fiesta.level {
            self.level = fiesta.level;
            self.hang();
        }
        let (w, ph) = (self.w as i32, self.h as f32 * 2.0);
        for i in 0..self.petals.len() {
            let p = &mut self.petals[i];
            // Like a leaf: slowest at the ends of its swing.
            let swing = wave(self.t, p.freq, p.phase).cos().abs();
            p.y += p.fall * (0.45 + 0.55 * swing) * dt;
            if p.y > ph + 1.0 {
                let mut q = petal(&mut self.rng, w);
                q.y = -1.0 - self.rng.f32() * 6.0;
                self.petals[i] = q;
            }
        }
    }

    /// Draws over the scene. Nothing on an ordinary day.
    pub fn draw(&self, c: &mut Canvas, theme: &Theme, fiesta: Fiesta) {
        if !fiesta.is(Kind::Muertos) {
            return;
        }
        let mut papel = PAPEL;
        let mut cord = CORD;
        if !theme.native {
            for (i, slot) in papel.iter_mut().enumerate() {
                *slot = solid(theme.colors.get(i % theme.colors.len().max(1)), *slot);
            }
            cord = solid(Some(&theme.fg), solid(theme.ramp.last(), CORD)).scale(0.85);
        }
        let level = fiesta.level.clamp(0.0, 1.0);

        let n = ((self.petals.len() as f32 * (0.3 + 0.7 * level)).round() as usize)
            .clamp(1, self.petals.len().max(1));
        for p in self.petals.iter().take(n) {
            self.draw_petal(c, p);
        }

        if self.upper < self.flags.len() {
            self.draw_lower_cord(c, cord);
            for f in &self.flags[self.upper..] {
                self.draw_flag(c, f, &papel);
            }
        }
        self.draw_upper_cord(c, cord);
        for f in &self.flags[..self.upper] {
            self.draw_flag(c, f, &papel);
        }
    }

    /// The upper cord's pixel row at column `x`: on the top row at the ties,
    /// one pixel lower between them.
    fn sag(&self, x: i32) -> i32 {
        let k = self.ties.partition_point(|&t| t <= x);
        if k == 0 || k >= self.ties.len() {
            return 0;
        }
        let (a, b) = (self.ties[k - 1], self.ties[k]);
        let u = (x - a) as f32 / (b - a).max(1) as f32;
        (4.0 * u * (1.0 - u) + 0.35) as i32
    }

    /// Does a flag of the upper string (`UPPER`), the lower one (`LOWER`) or
    /// either hang over column `x`?
    fn covered(&self, x: i32, which: u8) -> bool {
        usize::try_from(x)
            .ok()
            .and_then(|x| self.cover.get(x))
            .is_some_and(|m| m & which != 0)
    }

    /// The cord runs through the flags' folds, out of sight.
    fn draw_upper_cord(&self, c: &mut Canvas, cord: Color) {
        for x in 0..self.w as i32 {
            if !self.covered(x, UPPER) {
                put(c, x, self.sag(x), cord);
            }
        }
        // A knot at each tie, its ends hanging.
        for &x in &self.ties {
            put2(c, x, 0, Some(cord.scale(1.15)), Some(cord.scale(0.8)));
        }
    }

    /// The lower cord hangs from the upper one at both ends.
    fn draw_lower_cord(&self, c: &mut Canvas, cord: Color) {
        let (a, b) = self.lower;
        if b <= a {
            return;
        }
        let cord = cord.scale(0.85);
        for x in a..=b {
            if !self.covered(x, UPPER | LOWER) {
                let u = (x - a) as f32 / (b - a) as f32;
                put(c, x, LOWER_TOP + (4.0 * u * (1.0 - u) + 0.35) as i32, cord);
            }
        }
        for d in 1..LOWER_TOP {
            for x in [a - d, b + d] {
                if !self.covered(x, UPPER) {
                    put(c, x, LOWER_TOP - d, cord);
                }
            }
        }
    }

    /// How far the breeze swings the bottom of the flag at column `x`, in
    /// pixels: a slow gust travels along the string, each flag a little late.
    fn sway(&self, x: i32, phase: f32) -> f32 {
        let x = x as f32;
        let gust = (0.5 + 0.5 * (wave(self.t, 0.23, 0.0) - 0.018 * x).sin()).powi(2);
        let amp = 0.45 + 0.9 * gust;
        amp * (wave(self.t, 1.9, 0.35 * phase) - 0.11 * x).sin()
    }

    fn draw_flag(&self, c: &mut Canvas, f: &Flag, papel: &[Color; 6]) {
        let s = self.sway(f.x, f.phase);
        // Turned away from the light as it swings (in a few steps, so a
        // flag is not redrawn every frame).
        let turned = (s.abs() * 2.5).round() / 2.5;
        let base = papel[f.color as usize % papel.len()].scale(1.0 - 0.1 * turned);
        let fold = base.scale(0.78);
        let design = &DESIGNS[f.design as usize % DESIGNS.len()];
        // The bottom swings furthest.
        let mut shift = [0; FLAG_H as usize];
        for (r, dx) in shift.iter_mut().enumerate() {
            *dx = (s * r as f32 / (FLAG_H - 1) as f32).round() as i32;
        }
        let pixel = |x: i32, r: i32| -> Option<Color> {
            let r = usize::try_from(r).ok()?;
            let i = x - f.x - shift.get(r)?;
            match design.get(r)?.as_bytes().get(usize::try_from(i).ok()?)? {
                b'X' if r == 0 => Some(fold),
                b'X' => Some(base),
                _ => None,
            }
        };
        // Cell by cell (the flags hang from even pixel rows).
        for r in (0..FLAG_H).step_by(2) {
            for x in f.x - 2..f.x + FLAG_W + 2 {
                put2(c, x, (f.top + r) / 2, pixel(x, r), pixel(x, r + 1));
            }
        }
    }

    fn draw_petal(&self, c: &mut Canvas, p: &Petal) {
        let x = p.x + p.sway * wave(self.t, p.freq, p.phase).sin();
        let turn = wave(self.t, p.spin, p.phase);
        let (light, dark) = if p.gold {
            (GOLD_LIGHT, GOLD_DARK)
        } else {
            (PETAL_LIGHT, PETAL_DARK)
        };
        let (px, py) = (x.floor() as i32, p.y.floor() as i32);
        // Face up, tipping over, edge on, face down.
        match (turn.rem_euclid(std::f32::consts::TAU) * 4.0 / std::f32::consts::TAU) as u8 {
            0 => {
                put(c, px, py, light);
                put(c, px + 1, py, light.lerp(dark, 0.25));
            }
            1 => {
                put(c, px, py, light);
                put(c, px, py + 1, dark);
            }
            2 => put(c, px, py, light.lerp(dark, 0.5)),
            _ => {
                put(c, px, py, dark);
                put(c, px + 1, py, dark.lerp(light, 0.2));
            }
        }
    }
}

/// The angle at `t` seconds of something turning at `freq` radians per
/// second from `phase`.
fn wave(t: f64, freq: f32, phase: f32) -> f32 {
    ((freq as f64 * t) % std::f64::consts::TAU) as f32 + phase
}

/// A new petal somewhere across `w` columns.
fn petal(rng: &mut fastrand::Rng, w: i32) -> Petal {
    Petal {
        x: rng.f32() * w.max(1) as f32,
        y: 0.0,
        fall: 3.0 + rng.f32() * 3.5,
        sway: 1.0 + rng.f32() * 2.5,
        freq: 0.7 + rng.f32() * 0.9,
        phase: rng.f32() * std::f32::consts::TAU,
        spin: 2.5 + rng.f32() * 4.0,
        gold: rng.u8(..) < 50,
    }
}

/// A random index below `n` (at least 3) that is not one of `avoid`.
fn other(rng: &mut fastrand::Rng, avoid: [u8; 2], n: usize) -> u8 {
    let n = n.clamp(1, 255) as u8;
    let mut i = rng.u8(..n);
    for _ in 0..n {
        if !avoid.contains(&i) {
            break;
        }
        i = (i + 1) % n;
    }
    i
}

/// A theme color that can be painted as a pixel (`Reset` would be a hole).
fn solid(c: Option<&Color>, fallback: Color) -> Color {
    match c {
        Some(&c) if c != Color::Reset => c,
        _ => fallback,
    }
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

    const MUERTOS: Fiesta = Fiesta {
        kind: super::super::Kind::Muertos,
        level: 1.0,
        month: 11,
        day: 2,
    };

    /// A scene's frame: half blocks, text and empty cells.
    fn scene(w: u16, h: u16) -> Canvas {
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
    fn after(o: &mut Overlay, fiesta: Fiesta, secs: f32, theme: &Theme) -> Canvas {
        for _ in 0..(secs * 20.0) as usize {
            o.update(0.05, fiesta);
        }
        let mut c = scene(o.w, o.h);
        o.draw(&mut c, theme, fiesta);
        c
    }

    /// Rows (from, to) holding cells the overlay changed.
    fn changed(c: &Canvas, w: u16) -> Vec<(i32, i32)> {
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
}
