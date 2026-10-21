//! nyancat: the pop-tart cat flies through space, a port of nyancat.
//!
//! The frames, the palettes and the rainbow-tail logic come from K. Lange's
//! nyancat, <https://github.com/klange/nyancat>, under the NCSA license; its
//! copyright notice, conditions and disclaimer are kept in `nyancat/frames.rs`
//! and cover those parts of this file too. Like nyancat, the animation is a 64×64 frame centered on
//! the screen with the rainbow continued to the left edge, advancing every
//! 90 ms. On top of that, the stars are not baked into the 64×64 frame but
//! scattered over the whole screen, using the frames' six star stages and
//! moving left at the same pace, so big terminals get a sky instead of a box.

pub(crate) mod frames;

use frames::{FRAME_HEIGHT, FRAME_WIDTH, FRAMES, Frame};

use crate::color::{self, Color};
use crate::render::{BOLD, Canvas, Pixels};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "nyancat",
    aliases: &["nyan"],
    about: "the poptart cat flies through space, after nyancat",
    options: &[
        Opt {
            key: "background",
            default: Val::B(false),
            help: "Paint nyancat's blue sky (the theme background under other themes) instead of keeping the terminal's",
        },
        Opt {
            key: "counter",
            default: Val::B(true),
            help: "Show \"You have nyaned for N seconds!\" on the bottom line",
        },
        Opt {
            key: "scale",
            default: Val::I(0),
            help: "Half-block pixels per cat pixel (0 = fit the terminal; 2 is nyancat's own size)",
        },
        Opt {
            key: "delay_ms",
            default: Val::I(90),
            help: "Milliseconds per animation frame, as nyancat -d",
        },
        Opt {
            key: "palette",
            default: Val::S("rgb"),
            help: "nyancat's colors under classic: rgb (its truecolor table) or 256 (its xterm-256 table)",
        },
    ],
    make: |ctx| Box::new(Nyancat::new(ctx)),
};

/// The rainbow nyancat extends to the left of the frame, rows 23-42 (with padding).
pub(crate) const RAINBOW: &[u8; 20] = b",,>>&&&+++###==;;;,,";

/// nyancat's truecolor palette (`TERM=truecolor`) and xterm-256 palette, per frame character.
pub(crate) const PALETTE: &[(u8, u32, u8)] = &[
    (b',', 0x003169, 17),  // blue background
    (b'.', 0xffffff, 231), // white stars
    (b'\'', 0x000000, 16), // black border
    (b'@', 0xffcd98, 230), // tan poptart
    (b'$', 0xffa9ff, 175), // pink poptart
    (b'-', 0xff4c98, 162), // red poptart
    (b'>', 0xff1900, 196), // red rainbow
    (b'&', 0xff9a00, 214), // orange rainbow
    (b'+', 0xfff000, 226), // yellow rainbow
    (b'#', 0x28dc00, 118), // green rainbow
    (b'=', 0x0090ff, 33),  // light blue rainbow
    (b';', 0x6844ff, 19),  // dark blue rainbow
    (b'*', 0x999999, 240), // gray cat face
    (b'%', 0xffa398, 175), // pink cheeks
];

pub(crate) const RAINBOW_CHARS: [u8; 6] = *b">&+#=;";

/// The six stages a star goes through in nyancat's frames, as offsets from its center.
const STAR_STAGES: [&[(i32, i32)]; 6] = [
    &[(0, 0)],
    &[(0, -1), (-1, 0), (1, 0), (0, 1)],
    &[
        (0, -2),
        (0, -1),
        (-2, 0),
        (-1, 0),
        (1, 0),
        (2, 0),
        (0, 1),
        (0, 2),
    ],
    &[
        (0, -3),
        (0, -2),
        (-3, 0),
        (-2, 0),
        (0, 0),
        (2, 0),
        (3, 0),
        (0, 2),
        (0, 3),
    ],
    &[
        (0, -3),
        (-2, -2),
        (2, -2),
        (-3, 0),
        (3, 0),
        (-2, 2),
        (2, 2),
        (0, 3),
    ],
    &[(0, -3), (-3, 0), (3, 0), (0, 3)],
];
/// How far stars travel per frame, in cat pixels (13/2: they move 6 or 7).
const STAR_STEP_X2: i64 = 13;
/// Stars sit on a jittered grid of this many cat pixels (nyancat has about six per frame).
const STAR_CELL: i32 = 21;
/// Keep star centers this far outside the visible area so they never pop in.
const STAR_MARGIN: i32 = 4;
/// Neighboring star centers stay at least this far apart (the largest stage has radius 3).
const STAR_GAP: i32 = 8;

#[derive(Clone, Copy)]
struct Star {
    x: i32,
    y: i32,
    phase: u8,
    /// +1: the star grows through the stages, -1: it shrinks (both happen in nyancat).
    dir: i8,
}

struct Nyancat {
    /// The frames with their baked-in stars removed (eyes kept), one byte per pixel.
    frames: Vec<[[u8; FRAME_WIDTH]; FRAME_HEIGHT]>,
    pal: [Color; 128],
    star_color: Color,
    sky: Color,
    text_fg: Color,
    background: bool,
    counter: bool,
    scale_opt: i32,
    delay: f64,
    use_256: bool,

    px: Pixels,
    /// Cat-pixel coordinate of each pixel column/row.
    col_fx: Vec<i32>,
    row_fy: Vec<i32>,
    /// Pixel position of the frame's origin and the size of one cat pixel.
    ox: i32,
    oy: i32,
    scale: i32,
    stars: Vec<Star>,
    /// Horizontal star field span (cat pixels) and its left edge.
    star_lo: i32,
    star_span: i32,
    w: u16,
    h: u16,

    acc: f64,
    ticks: u64,
    /// Real seconds nyaned, for the counter.
    seconds: f64,
    /// Scene seconds per real second, from the last tick.
    pace: f64,
    rng: fastrand::Rng,
}

impl Nyancat {
    fn new(ctx: &Ctx) -> Nyancat {
        let o = ctx.opts;
        let mut n = Nyancat {
            frames: FRAMES.iter().map(strip_stars).collect(),
            pal: [Color::Reset; 128],
            star_color: Color::Rgb(255, 255, 255),
            sky: Color::Reset,
            text_fg: Color::Reset,
            background: o.b("background"),
            counter: o.b("counter"),
            scale_opt: o.i("scale").clamp(0, 64) as i32,
            delay: o.i("delay_ms").clamp(10, 1000) as f64 / 1000.0,
            use_256: o.s("palette").trim() == "256",
            px: Pixels::new(1, 1),
            col_fx: Vec::new(),
            row_fy: Vec::new(),
            ox: 0,
            oy: 0,
            scale: 1,
            stars: Vec::new(),
            star_lo: 0,
            star_span: STAR_CELL,
            w: 1,
            h: 1,
            acc: 0.0,
            ticks: 0,
            seconds: 0.0,
            pace: 1.0,
            rng: ctx.rng(),
        };
        n.retheme(ctx);
        n
    }

    /// Rows of the screen used by the animation (the last one holds the counter).
    fn anim_rows(&self) -> u16 {
        if self.counter && self.h >= 3 {
            self.h - 1
        } else {
            self.h
        }
    }
}

/// A copy of a frame without its stars: every white pixel except the eyes (the
/// only ones with face on their left and outline on their right) becomes
/// whatever is behind it, the sky or the rainbow.
pub(crate) fn strip_stars(f: &Frame) -> [[u8; FRAME_WIDTH]; FRAME_HEIGHT] {
    let mut out = [[b','; FRAME_WIDTH]; FRAME_HEIGHT];
    for (y, row) in f.iter().enumerate() {
        for x in 0..FRAME_WIDTH {
            let ch = row[x];
            let eye = x > 0 && x + 1 < FRAME_WIDTH && row[x - 1] == b'*' && row[x + 1] == b'\'';
            out[y][x] = if ch != b'.' || eye {
                ch
            } else if (24..=42).contains(&y) {
                // The rainbow in this column starts on row 24 (up) or 25 (down).
                match (f[24][x], f[25][x]) {
                    (b'>', _) => *RAINBOW.get(1 + y - 23).unwrap_or(&b','),
                    (_, b'>') => *RAINBOW.get(y - 23).unwrap_or(&b','),
                    _ => b',',
                }
            } else {
                b','
            };
        }
    }
    out
}

#[inline]
pub(crate) fn is_cat(ch: u8) -> bool {
    matches!(ch, b'\'' | b'@' | b'$' | b'-' | b'*' | b'%' | b'.')
}

/// What nyancat shows at cat-pixel (x, y): the frame inside it, the generated
/// rainbow to its left, sky everywhere else.
#[inline]
pub(crate) fn sample(f: &[[u8; FRAME_WIDTH]; FRAME_HEIGHT], flip: bool, x: i32, y: i32) -> u8 {
    if y > 23 && y < 43 && x < 0 {
        // nyancat's square wave: eight columns up, eight down, flipping every two frames.
        let mut mod_x = ((2 - x as i64) % 16) / 8;
        if flip {
            mod_x = 1 - mod_x;
        }
        *RAINBOW
            .get(mod_x as usize + y as usize - 23)
            .unwrap_or(&b',')
    } else if x < 0 || y < 0 || x >= FRAME_WIDTH as i32 || y >= FRAME_HEIGHT as i32 {
        b','
    } else {
        f[y as usize][x as usize]
    }
}

/// The scale nyancat itself uses (one cat pixel = two columns × one row), grown
/// on very tall terminals and shrunk when the cat would not fit.
fn auto_scale(cols: i32, rows: i32) -> i32 {
    let want = (rows / 22).max(2);
    want.min(rows / 11).min(cols / 36).max(1)
}

impl Scene for Nyancat {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.w = w.max(1);
        self.h = h.max(1);
        let rows = self.anim_rows();
        self.px.resize(self.w, rows);
        let (pw, ph) = (self.px.pw(), self.px.ph());
        let s = if self.scale_opt > 0 {
            self.scale_opt
        } else {
            auto_scale(pw, rows as i32)
        };
        self.scale = s;
        // Center the frame like nyancat does; the cat sits at its middle.
        self.ox = (pw - FRAME_WIDTH as i32 * s).div_euclid(2);
        // Start on a cell boundary so even scales give whole blocks, not half-block seams.
        let oy = (ph - FRAME_HEIGHT as i32 * s).div_euclid(2);
        self.oy = oy - oy.rem_euclid(2);
        self.col_fx.clear();
        self.col_fx
            .extend((0..pw).map(|px| (px - self.ox).div_euclid(s)));
        self.row_fy.clear();
        self.row_fy
            .extend((0..ph).map(|py| (py - self.oy).div_euclid(s)));

        // A jittered grid of stars over the visible area plus a margin, in cat
        // pixels. The width is a whole number of cells so the field wraps seamlessly.
        let x0 = (-self.ox).div_euclid(s) - STAR_MARGIN;
        let x1 = (pw - 1 - self.ox).div_euclid(s) + STAR_MARGIN + 1;
        let y0 = (-self.oy).div_euclid(s) - STAR_MARGIN;
        let y1 = (ph - 1 - self.oy).div_euclid(s) + STAR_MARGIN + 1;
        let nx = ((x1 - x0) + STAR_CELL - 1) / STAR_CELL;
        let ny = ((y1 - y0) + STAR_CELL - 1) / STAR_CELL;
        self.star_lo = x0;
        self.star_span = nx.max(1) * STAR_CELL;
        self.stars.clear();
        for j in 0..ny {
            for i in 0..nx {
                self.stars.push(Star {
                    x: x0 + i * STAR_CELL + self.rng.i32(0..STAR_CELL - STAR_GAP),
                    y: y0 + j * STAR_CELL + self.rng.i32(0..STAR_CELL - STAR_GAP),
                    phase: self.rng.u8(0..6),
                    dir: if self.rng.bool() { 1 } else { -1 },
                });
            }
        }
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let native = ctx.theme.native;
        let mut pal = [Color::Reset; 128];
        for &(ch, hex, idx) in PALETTE {
            pal[ch as usize] = if self.use_256 && native {
                let (r, g, b) = color::ansi_rgb(idx);
                Color::Rgb(r, g, b)
            } else {
                color::rgb(hex)
            };
        }
        let blue = pal[b',' as usize];
        if native {
            self.star_color = pal[b'.' as usize];
            self.sky = blue;
        } else {
            for (ch, c) in RAINBOW_CHARS.iter().zip(theme_rainbow(ctx.theme)) {
                pal[*ch as usize] = c;
            }
            self.star_color = match ctx.theme.fg {
                Color::Reset => Color::Ansi(15),
                c => c,
            };
            self.sky = match ctx.theme.bg {
                Color::Reset => blue,
                c => c,
            };
        }
        // The sky stays see-through unless asked for.
        pal[b',' as usize] = if self.background {
            self.sky
        } else {
            Color::Reset
        };
        self.pal = pal;
        self.text_fg = match (native, self.background) {
            (true, true) => Color::Rgb(255, 255, 255),
            (true, false) => Color::Reset,
            (false, _) => ctx.theme.fg,
        };
    }

    fn update(&mut self, ctx: &Ctx, dt: f32) {
        self.tick(ctx, dt, dt);
    }

    fn tick(&mut self, _ctx: &Ctx, dt: f32, real: f32) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        // The counter counts real seconds, whatever the speed.
        if real.is_finite() && real > 0.0 {
            self.seconds += real as f64;
            self.pace = dt as f64 / real as f64;
        }
        let dt = dt as f64;
        self.acc += dt;
        let n = (self.acc / self.delay).floor();
        self.acc -= n * self.delay;
        self.ticks = self.ticks.wrapping_add(n as u64);
    }

    fn next_change(&self) -> f32 {
        let frame = (self.delay - self.acc).max(0.0);
        // The counter ticks over on whole seconds.
        let counter = if self.counter {
            // Real seconds to the next whole one, in scene seconds.
            (1.0 - self.seconds.fract()) * self.pace
        } else {
            f64::INFINITY
        };
        frame.min(counter) as f32
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        let i = (self.ticks % FRAMES.len() as u64) as usize;
        let flip = (i / 2) % 2 == 1;
        let f = &self.frames[i];
        self.px.clear();
        let empty_sky = self.pal[b',' as usize] == Color::Reset;
        for (py, &fy) in self.row_fy.iter().enumerate() {
            // Rows that are all sky have nothing to paint when the sky is transparent.
            if empty_sky && !(0..43).contains(&fy) {
                continue;
            }
            for (px, &fx) in self.col_fx.iter().enumerate() {
                let col = self.pal[sample(f, flip, fx, fy) as usize & 127];
                if col != Color::Reset {
                    self.px.set(px as i32, py as i32, col);
                }
            }
        }

        // Stars: behind the cat, in front of the sky and the rainbow.
        let k = self.ticks as i64;
        let shift = (k.wrapping_mul(STAR_STEP_X2) / 2).rem_euclid(self.star_span as i64) as i32;
        let s = self.scale;
        for st in &self.stars {
            let cx = self.star_lo + (st.x - self.star_lo - shift).rem_euclid(self.star_span);
            let stage = (st.phase as i64 + st.dir as i64 * (k % 6)).rem_euclid(6) as usize;
            for &(dx, dy) in STAR_STAGES[stage] {
                let (fx, fy) = (cx + dx, st.y + dy);
                if is_cat(sample(f, flip, fx, fy)) {
                    continue;
                }
                self.px
                    .fill_rect(self.ox + fx * s, self.oy + fy * s, s, s, self.star_color);
            }
        }
        self.px.blit(c);

        if self.anim_rows() < self.h {
            let y = self.h as i32 - 1;
            if self.background {
                for x in 0..self.w as i32 {
                    c.set_bg(x, y, self.sky);
                }
            }
            // "You have nyaned for N seconds!", centered like nyancat does.
            let mut buf = [0u8; 20];
            let digits = fmt_u64(self.seconds.floor() as u64, &mut buf);
            let x = ((self.w as i32 - 29 - digits.len() as i32) / 2).max(0);
            let fg = self.text_fg;
            let x = c.text_attr(x, y, "You have nyaned for ", fg, BOLD);
            let x = c.text_attr(x, y, digits, fg, BOLD);
            c.text_attr(x, y, " seconds!", fg, BOLD);
        }
    }
}

/// Formats `n` into `buf` without allocating.
fn fmt_u64(mut n: u64, buf: &mut [u8; 20]) -> &str {
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 || i == 0 {
            break;
        }
    }
    std::str::from_utf8(&buf[i..]).unwrap_or("0")
}

/// Six theme accents for the rainbow bands, in rainbow order: by hue when the
/// accents differ in hue, else bright to dark. Themes with fewer than six
/// accents get bands blended between them, so no two bands repeat.
pub(crate) fn theme_rainbow(theme: &crate::theme::Theme) -> [Color; 6] {
    let mut v: Vec<Color> = theme.colors.iter().take(6).copied().collect();
    if v.is_empty() {
        v.push(theme.fg);
    }
    let hues: Vec<Option<f32>> = v.iter().map(|&c| hue(c)).collect();
    let all_rgb = v.iter().all(|c| c.is_rgb());
    if hues.iter().all(Option::is_some) {
        let (lo, hi) = hues
            .iter()
            .flatten()
            .fold((f32::MAX, f32::MIN), |(a, b), &h| (a.min(h), b.max(h)));
        if hi - lo >= 60.0 {
            v.sort_by(|a, b| hue(*a).unwrap_or(0.0).total_cmp(&hue(*b).unwrap_or(0.0)));
        } else {
            v.sort_by(|a, b| b.luma().total_cmp(&a.luma()));
        }
    } else if all_rgb {
        v.sort_by(|a, b| b.luma().total_cmp(&a.luma()));
    }
    if v.len() < 6 && all_rgb {
        let g = color::Gradient::new(&v);
        return std::array::from_fn(|i| g.at(i as f32 / 5.0));
    }
    std::array::from_fn(|i| v[i % v.len()])
}

/// Hue in degrees, rotated so reds come first and pinks last; `None` for grays
/// and palette colors.
fn hue(c: Color) -> Option<f32> {
    let Color::Rgb(r, g, b) = c else { return None };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    if d < 0.08 {
        return None;
    }
    let h = if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    Some((h + 20.0).rem_euclid(360.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripping_keeps_only_the_eyes() {
        for f in &FRAMES {
            let s = strip_stars(f);
            let whites = s.iter().flatten().filter(|&&c| c == b'.').count();
            assert_eq!(whites, 2);
            // `draw` skips rows from 43 down when the sky is transparent.
            assert!(s[43..].iter().flatten().all(|&c| c == b','));
        }
        // Frame 0 has a star over the rainbow at (10, 37): it becomes light blue.
        assert_eq!(strip_stars(&FRAMES[0])[37][10], b'=');
    }

    #[test]
    fn rainbow_continues_the_frame() {
        let f = strip_stars(&FRAMES[0]);
        // Columns -5..=1 are down, -13..=-6 up, matching the frame's own columns 0 and 1.
        assert_eq!(sample(&f, false, -1, 24), b',');
        assert_eq!(sample(&f, false, -6, 24), b'>');
        assert_eq!(sample(&f, false, 0, 24), b',');
        assert_eq!(sample(&f, true, -1, 24), b'>');
        assert_eq!(sample(&f, false, -400, 50), b',');
    }

    #[test]
    fn the_counter_counts_real_seconds() {
        use crate::scene::Opts;
        use crate::theme::Theme;
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Reset,
            seed: 1,
        };
        let mut n = Nyancat::new(&ctx);
        // Five real seconds at a tenth of the speed.
        for _ in 0..50 {
            n.tick(&ctx, 0.01, 0.1);
        }
        assert!((n.seconds - 5.0).abs() < 1e-6, "{}", n.seconds);
        assert!(n.next_change() <= 0.1 + 1e-6);
    }

    /// Odd option values and sizes must not panic (the registry test covers the defaults).
    #[test]
    fn extreme_options_survive() {
        use crate::scene::Opts;
        use crate::theme::{TermColors, Theme};
        let cases = [
            "scale = 64\ndelay_ms = 1\nbackground = true",
            "counter = false\npalette = '256'\nscale = -3\ndelay_ms = 100000",
        ];
        let themes = [
            Theme::classic(),
            Theme::builtin("mono").unwrap(),
            Theme::terminal(&TermColors::default()),
        ];
        for (i, case) in cases.iter().enumerate() {
            let opts = Opts::new(INFO.options, case.parse().unwrap());
            for theme in &themes {
                let ctx = Ctx {
                    theme,
                    opts: &opts,
                    bg: Color::Reset,
                    seed: 5 + i as u64,
                };
                let mut s = (INFO.make)(&ctx);
                for &(w, h) in &[(1u16, 1u16), (2, 3), (3, 1), (1, 40), (500, 150), (80, 24)] {
                    s.resize(&ctx, w, h);
                    s.retheme(&ctx);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for dt in [0.0, f32::NAN, -1.0, f32::INFINITY, 0.09, 5.0, 1e9] {
                        s.update(&ctx, dt);
                        c.clear();
                        s.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }

    #[test]
    fn formats_numbers() {
        let mut b = [0u8; 20];
        assert_eq!(fmt_u64(0, &mut b), "0");
        assert_eq!(fmt_u64(1234, &mut b), "1234");
        assert_eq!(fmt_u64(u64::MAX, &mut b), "18446744073709551615");
    }
}
