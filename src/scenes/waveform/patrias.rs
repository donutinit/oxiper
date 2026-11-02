//! Fiestas Patrias on the waves (see the module doc in `waveform.rs`).
//!
//! The holiday only dresses the waves, like the others: the waves become
//! ribbons of cloth in verde, blanco y rojo, and the same `ys` the ordinary
//! scene draws from carry banderitas and the águila. It keeps nothing but a
//! clock and its colors (banderitas stand where a hash puts them and bob as the
//! swell passes under them), and draws no random numbers, so an ordinary day
//! is as it ever was.

use std::f32::consts::TAU;

use super::hues::{self, like};
use super::muertos::{hash, rnd};
use super::{Layer, Style, Waveform};
use crate::color::{Color, rgb};
use crate::render::Braille;
use crate::scene::Ctx;

/// The águila's café, the nopal, the oro of the pole's point, and the serpent.
const AGUILA: Color = rgb(0x7a4a1e);
const NOPAL: Color = rgb(0x4f8a2a);
const ORO: Color = rgb(0xd4a017);
const SERPIENTE: Color = rgb(0xb9b4a6);

/// The emblem, facing left as on the flag: the águila on its nopal with the
/// serpent in its beak, wing raised, in dots. Each letter is a color of
/// `Patrias::emblem` (`a` plumage, `l` light feathers, `o` beak and talons,
/// `e` the eye, `s` the serpent, `r` its tongue, `n` the nopal, `t` a tuna);
/// the feathers are told apart by gaps, which show at any size.
const EMBLEM: [&str; 26] = [
    ".................aaaaaaa........",
    "................aaaaaa..........",
    "..............aaaaaaa.aaaaaa....",
    "......a.....aaaaaaaa.aaaaaa.....",
    "....aaaaa...aaaaaaa.aaaaa.......",
    "..ooaeaaaa.aaaaaaa.aaaa.........",
    "..oaaaaaaa.aaaaaa.aaa..aaaaa....",
    ".o.aaaaaaaaaaaaa.aa...aaaaaaaa..",
    "ooo.aaaaaaaaaaa.aa..aaaaaa......",
    ".s....aaaaaaaa.a..aaaa..........",
    ".ss...aaaaaaaaa.aa....aaaaaa....",
    "..s....aaaaaaaaa..aaaa......a...",
    ".ss....aaaaaaaaaa......aaaaaaa..",
    ".s.....aala..aaaaaaaaaaaaaa.....",
    "ss.....lllll.....aaaaaaa........",
    "s......lllllaaaa...aaaaa........",
    "ss....lllllllaaaaaaaaaaaaaaaaa..",
    ".ss....lllllaaaaaaaa..aaaaaaaa..",
    "..ss...lllllaaaaaaaa............",
    "....s....laaaaaaaaaa.....aaaa.aa",
    ".....s....aaaaaaaaaaa..aa...aaaa",
    "......rr..a.aaaaaa.aaa..aaa....a",
    "..........nnnnnnn...aaa..aaaaa..",
    "...nntnnnnnnnnnntonnnnnnn.aaaa..",
    ".nnnnnnnnnnnnnnnnnnnnnnnntnnaa..",
    ".nnnnnnnnnnnnnnnnnnnnnnnnnnn....",
];
/// Where the emblem stands on its wave, in its own dots: the middle of the nopal.
const FOOT: (f32, f32) = (14.0, 23.0);
/// The size of the art, in dots.
const SIZE: (f32, f32) = (32.0, 26.0);
/// The letters of `EMBLEM`, in the order of `Patrias::emblem`.
const LETTERS: &[u8; 8] = b"aloesrnt";

/// A banderita's measures by size, nearest first: the pole's height, the
/// pennant's length and width, and the spacing between them at the start of
/// the mes patrio and at the Grito, in dots.
const FLAG: [([i32; 3], [f32; 2]); 4] = [
    ([12, 9, 7], [78.0, 34.0]),
    ([9, 7, 6], [64.0, 28.0]),
    ([7, 5, 4], [52.0, 24.0]),
    ([5, 4, 3], [42.0, 20.0]),
];

pub(super) struct Patrias {
    level: f32,
    /// The wind's clock, in seconds.
    t: f32,
    /// Per layer: the shades of its cloth (the fold's light to its shadow),
    /// its banderitas' three colors and the pole.
    cloth: Vec<[Color; 5]>,
    flags: Vec<[Color; 3]>,
    pole: Vec<Color>,
    /// The emblem's colors, by `LETTERS`.
    emblem: [Color; 8],
    /// The wave that carries the águila: the middle one.
    eagle: usize,
    /// The highest crest among the layers in front of the one being drawn.
    front: Vec<f32>,
    /// How many banderitas stood on the waves in the last frame.
    flagged: usize,
}

/// Which of verde (0), blanco (1) and rojo (2) the wave `li` of `n` is: the
/// back ones are the green stripe, the middle ones the white, the front ones
/// the red.
fn stripe(li: usize, n: usize) -> usize {
    ((3 * (2 * li + 1)) / (2 * n)).min(2)
}

/// A stripe's color: the flag's own under `classic`, else the theme's green,
/// its foreground and its red.
fn tricolor(ctx: &Ctx, s: usize) -> Color {
    hues::tricolor(ctx.theme)[s.min(2)]
}

/// A layer's color under the holiday: its stripe's.
pub(super) fn color(ctx: &Ctx, li: usize, n: usize) -> Color {
    tricolor(ctx, stripe(li, n))
}

impl Patrias {
    /// Dresses the waves for the holiday, or retouches the colors after a
    /// theme change.
    pub(super) fn dress(slot: &mut Option<Patrias>, ctx: &Ctx, layers: &[Layer], pw: usize) {
        let n = layers.len();
        let m = slot.get_or_insert_with(|| Patrias {
            level: 0.0,
            t: 0.0,
            cloth: Vec::with_capacity(n),
            flags: Vec::with_capacity(n),
            pole: Vec::with_capacity(n),
            emblem: [Color::Reset; 8],
            eagle: 0,
            front: vec![0.0; pw],
            flagged: 0,
        });
        m.level = ctx.fiesta.level.clamp(0.0, 1.0);
        m.eagle = n / 2;
        m.cloth.clear();
        m.flags.clear();
        m.pole.clear();
        for (li, l) in layers.iter().enumerate() {
            // The farther back, the dimmer.
            let k = (1.0 - l.depth) * 0.16;
            let c = color(ctx, li, n);
            m.cloth.push([
                ctx.fade(c.lerp(rgb(0xffffff), 0.3), k),
                ctx.fade(c.lerp(rgb(0xffffff), 0.06), k),
                ctx.fade(c, k + 0.08),
                ctx.fade(c, k + 0.28),
                ctx.fade(c, k + 0.45),
            ]);
            m.flags
                .push(std::array::from_fn(|s| ctx.fade(tricolor(ctx, s), k)));
            m.pole.push(ctx.fade(like(ctx.theme, ORO), k));
        }
        let plumage = like(ctx.theme, AGUILA);
        let (white, red) = (tricolor(ctx, 1), tricolor(ctx, 2));
        m.emblem = [
            plumage,
            plumage.lerp(rgb(0xffffff), 0.32),
            like(ctx.theme, ORO),
            white,
            like(ctx.theme, SERPIENTE),
            red,
            like(ctx.theme, NOPAL),
            red,
        ];
    }

    /// The screen changed size.
    pub(super) fn resized(&mut self, pw: usize) {
        self.front.clear();
        self.front.resize(pw, 0.0);
    }

    /// How many águilas ride the middle wave on a screen `pw` dots wide: one
    /// at first, three at the Grito, never crowding.
    fn eagles(&self, pw: usize) -> usize {
        let wanted = 1 + (self.level > 0.5) as usize + (self.level > 0.9) as usize;
        wanted.min((pw / 90).max(1))
    }
}

impl Waveform {
    /// Moves the holiday on by `dt` seconds: only the wind blows.
    pub(super) fn step_patrias(&mut self, dt: f32) {
        if let Some(m) = self.patrias.as_mut() {
            m.t = (m.t + dt) % 3600.0;
        }
    }

    pub(super) fn draw_patrias(&mut self) {
        let Some(mut m) = self.patrias.take() else {
            return;
        };
        let n = self.layers.len();
        let center = self.ph as f32 * 0.5 - 0.5;
        m.flagged = 0;
        // Filled and dotted waves are drawn as ever, in the flag's colors.
        match self.style {
            Style::Filled => self.draw_filled(),
            Style::Dots => self.draw_dots(),
            _ => {}
        }
        for li in 0..n {
            if self.style == Style::Filled {
                self.crests_in_front(&mut m.front, li);
            }
            match self.style {
                Style::Lines => {
                    self.ribbon(&m, li, 1.0, 0.0);
                    self.banderitas(&mut m, li, (1.0, 0.0));
                }
                Style::Mirror => {
                    for sign in [1.0, -1.0] {
                        self.ribbon(&m, li, sign, center);
                        self.banderitas(&mut m, li, (sign, center));
                    }
                }
                Style::Filled | Style::Dots => self.banderitas(&mut m, li, (1.0, 0.0)),
            }
        }
        // The águila last, so no wave hides her.
        self.aguilas(&m);
        self.patrias = Some(m);
    }

    /// Layer `li` as a ribbon of cloth: a band along the wave, rippling in
    /// the wind, its folds lighter and darker as they roll along.
    fn ribbon(&mut self, m: &Patrias, li: usize, sign: f32, center: f32) {
        let pw = self.pw;
        let (t, depth) = (m.t, self.layers[li].depth);
        let lf = li as f32;
        // The wind rises toward the Grito, and the cloth with it.
        let wind = 0.6 + 0.8 * m.level;
        let width = (2.6 + 3.4 * depth) * self.scale() * (0.9 + 0.25 * m.level)
            + (self.thickness - 1) as f32;
        let row = &self.ys[li * pw..(li + 1) * pw];
        let cloth = &m.cloth[li];
        // The two edges of the band at column `x`.
        let edge = |x: usize| {
            let xf = x as f32;
            let flutter = wind
                * 1.2
                * (xf * 0.17 - t * 4.2 + lf * 1.9).sin()
                * (0.6 + 0.4 * (xf * 0.021 + t * 0.7).sin());
            let top = center + sign * row[x] + flutter;
            let w = width * (0.85 + 0.3 * (xf * 0.085 - t * 2.4 + lf).sin());
            (top, top + w)
        };
        let (mut a, mut b) = (edge(0), edge(0));
        for x in 0..pw {
            let c = edge((x + 1).min(pw - 1));
            // Like the ordinary line, each column reaches its neighbours'
            // midpoints, so steep slopes stay connected.
            let lo = b.0.min((a.0 + b.0) * 0.5).min((c.0 + b.0) * 0.5);
            let hi = b.1.max((a.1 + b.1) * 0.5).max((c.1 + b.1) * 0.5);
            // A fold of light rolling along the cloth, a cell at a time; the
            // slope turns it toward the light or away.
            let slope = (c.0 - a.0) * 0.5 * sign;
            let fold =
                0.5 + 0.5 * (((x & !1) as f32) * 0.16 - t * 2.9 + lf * 1.7).sin() + 0.12 * slope;
            let shade = cloth[(fold * 3.4).clamp(0.0, 4.0) as usize];
            let (y0, y1) = (rnd(lo), rnd(hi));
            for y in y0.max(0)..=y1.min(self.braille.ph() - 1) {
                self.braille.set(x as i32, y, shade);
            }
            (a, b) = (b, c);
        }
    }

    /// The águilas riding the middle wave: the nopal on its crest, tilted
    /// with the slope, one at first and three at the Grito. On the
    /// oscilloscope they stand on the upper trace.
    fn aguilas(&mut self, m: &Patrias) {
        let pw = self.pw;
        let center = self.ph as f32 * 0.5 - 0.5;
        let mirror = self.style == Style::Mirror;
        let row = &self.ys[m.eagle * pw..(m.eagle + 1) * pw];
        // The top of the wave at dot column `x`.
        let top = |x: f32| {
            let xf = x.clamp(0.0, pw as f32 - 1.001);
            let i = xf as usize;
            let (a, b) = (row[i], row[(i + 1).min(pw - 1)]);
            let y = a + (b - a) * (xf - i as f32);
            if mirror { center - y.abs() } else { y }
        };
        let scale = (self.scale() * 1.1).clamp(0.7, 1.6);
        let spread: &[f32] = match m.eagles(pw) {
            1 => &[0.5],
            2 => &[0.3, 0.7],
            _ => &[0.18, 0.5, 0.82],
        };
        for &f in spread {
            let x = f * pw as f32;
            let slope = (top(x + 9.0) - top(x - 9.0)) / 18.0;
            let tilt = (slope.atan() * 0.7).clamp(-0.45, 0.45);
            emblem_sprite(&mut self.braille, &m.emblem, (x, top(x) + 1.0), tilt, scale);
        }
    }

    /// The banderitas on layer `li` (as `sign * ys + center`), on every wave
    /// but the middle one, which carries the águila.
    fn banderitas(&mut self, m: &mut Patrias, li: usize, (sign, center): (f32, f32)) {
        if li == m.eagle {
            return;
        }
        let pw = self.pw;
        let (tier, level, t) = (self.tier(li), m.level, m.t);
        let filled = self.style == Style::Filled;
        let row = &self.ys[li * pw..(li + 1) * pw];
        // The wave's top at dot column `x`.
        let at = |x: f32| {
            let xf = x.clamp(0.0, pw as f32 - 1.001);
            let i = xf as usize;
            let (a, b) = (row[i], row[(i + 1).min(pw - 1)]);
            center + sign * (a + (b - a) * (xf - i as f32))
        };
        let Patrias {
            flags,
            pole,
            front,
            flagged,
            ..
        } = m;
        let skip =
            |x: i32, y: i32| filled && x >= 0 && x < pw as i32 && front[x as usize] <= y as f32;
        // Banderitas: a pole planted on the wave and a pennant streaming in
        // the wind, bobbing as the swell passes under them.
        let ([pole_h, flag_w, flag_h], [far, near]) = FLAG[tier];
        let gap = far - (far - near) * level;
        let wind = 0.8 + 0.8 * level;
        let first = hash(li as u32 * 7919 + 11) * gap;
        let mut i = 0;
        loop {
            let x = first + i as f32 * gap;
            if x >= pw as f32 + flag_w as f32 {
                break;
            }
            i += 1;
            let base = at(x) + 1.0;
            let slope = (at(x + 3.0) - at(x - 3.0)) / 6.0;
            let tilt = (slope.atan() * 0.5).clamp(-0.5, 0.5);
            let (sin, cos) = tilt.sin_cos();
            let up = (sin, -cos);
            // The pole.
            for k in 0..pole_h {
                let (px, py) = (rnd(x + up.0 * k as f32), rnd(base + up.1 * k as f32));
                if !skip(px, py) {
                    self.braille.set(px, py, pole[li]);
                }
            }
            // The pennant: tapering to a point, its far end fluttering.
            let col = flags[li][(i + li) % 3];
            let phase = hash(i as u32 * 131 + li as u32) * TAU;
            let tip = (x + up.0 * pole_h as f32, base + up.1 * pole_h as f32);
            for u in 0..flag_w {
                let f = u as f32 / flag_w as f32;
                let rows = (flag_h as f32 * (1.0 - f)).ceil() as i32;
                let flutter = (t * 7.0 - u as f32 * 0.9 + phase).sin() * wind * f;
                for j in 0..rows {
                    let d = j as f32 + flutter;
                    let (px, py) = (
                        rnd(tip.0 + cos * u as f32 - up.0 * d),
                        rnd(tip.1 + sin * u as f32 - up.1 * d),
                    );
                    if !skip(px, py) {
                        self.braille.set(px, py, col);
                    }
                }
            }
            *flagged += 1;
        }
    }
}

/// The águila on its nopal, standing with the foot of its sprite at `(x, y)`,
/// tilted `tilt` radians (clockwise on screen) and `scale` times its size.
/// Details (eye, beak, talons, tongue, tunas) go on last, so a braille cell
/// shared with the plumage keeps them.
fn emblem_sprite(b: &mut Braille, colors: &[Color; 8], (x, y): (f32, f32), tilt: f32, scale: f32) {
    let (sin, cos) = tilt.sin_cos();
    let (cx, cy) = (x.round() as i32, y.round() as i32);
    // The box the turned art fits in (round its corners), kept on the screen.
    let (mut lo, mut hi) = ((f32::MAX, f32::MAX), (f32::MIN, f32::MIN));
    for (sx, sy) in [(0.0, 0.0), (SIZE.0, 0.0), (0.0, SIZE.1), SIZE] {
        let (ux, uy) = ((sx - FOOT.0) * scale, (sy - FOOT.1) * scale);
        let (dx, dy) = (cos * ux - sin * uy, sin * ux + cos * uy);
        lo = (lo.0.min(dx), lo.1.min(dy));
        hi = (hi.0.max(dx), hi.1.max(dy));
    }
    let (x0, x1) = (
        (lo.0.floor() as i32).max(-cx),
        (hi.0.ceil() as i32).min(b.pw() - cx),
    );
    let (y0, y1) = (
        (lo.1.floor() as i32).max(-cy),
        (hi.1.ceil() as i32).min(b.ph() - cy),
    );
    for details in [false, true] {
        for dy in y0..=y1 {
            for dx in x0..=x1 {
                let (fx, fy) = (dx as f32, dy as f32);
                // Which dot of the art lands here.
                let sx = ((cos * fx + sin * fy) / scale + FOOT.0).round();
                let sy = ((-sin * fx + cos * fy) / scale + FOOT.1).round();
                if sx < 0.0 || sy < 0.0 || sy >= EMBLEM.len() as f32 {
                    continue;
                }
                let Some(&ch) = EMBLEM[sy as usize].as_bytes().get(sx as usize) else {
                    continue;
                };
                let Some(i) = LETTERS.iter().position(|&l| l == ch) else {
                    continue;
                };
                if details == matches!(ch, b'e' | b'o' | b'r' | b't') {
                    b.set(cx + dx, cy + dy, colors[i]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::hues::{BLANCO, ROJO, VERDE};
    use super::super::{INFO, SHADES};
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::render::Canvas;
    use crate::scene::{Opts, Scene};
    use crate::theme::Theme;

    /// A scene of `style` at 120×36 under `theme`, `secs` seconds in, and what it drew.
    fn run(theme: &Theme, style: &str, fiesta: Fiesta, secs: usize) -> (Waveform, Canvas) {
        let opts = Opts::new(INFO.options, format!("style = '{style}'").parse().unwrap());
        let ctx = Ctx {
            theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta,
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        for _ in 0..secs * 30 {
            s.update(&ctx, 1.0 / 30.0);
        }
        let mut c = Canvas::new(120, 36, Color::Reset);
        s.draw(&ctx, &mut c);
        (s, c)
    }

    fn colors(c: &Canvas) -> Vec<Color> {
        c.cells()
            .iter()
            .filter(|c| c.ch != ' ')
            .map(|c| c.fg)
            .collect()
    }

    /// The waves are verde, blanco y rojo, the middle one carries the águila
    /// and the others bob with banderitas; an ordinary day none of it.
    #[test]
    fn patrias_dresses_the_waves() {
        let classic = Theme::classic();
        for style in ["lines", "filled", "dots", "mirror"] {
            for day in [(9, 1), (9, 15), (9, 16)] {
                let (s, c) = run(&classic, style, Fiesta::of_day(day.0, day.1), 12);
                assert!(s.patrias.is_some() && s.muertos.is_none() && s.navidad.is_none());
                let m = s.patrias.as_ref().unwrap();
                let on = colors(&c);
                // Every stripe of the flag shows, banderitas stand on the
                // waves, and the águila is there in her plumage.
                let n = s.layers.len();
                for st in 0..3 {
                    let shown = (0..n).filter(|&li| stripe(li, n) == st).any(|li| {
                        m.cloth[li]
                            .iter()
                            .chain(&s.shades[li * SHADES..(li + 1) * SHADES])
                            .any(|c| on.contains(c))
                    });
                    assert!(shown, "{style} {day:?}: no stripe {st}");
                }
                assert!(m.flagged > 0, "{style} {day:?}: no banderitas");
                assert!(
                    m.emblem[..2].iter().any(|c| on.contains(c)),
                    "{style} {day:?}: no águila"
                );
            }
            let (s, c) = run(&classic, style, Fiesta::NONE, 12);
            assert!(s.patrias.is_none());
            let plain = colors(&c);
            for c in [VERDE, BLANCO, ROJO, AGUILA, NOPAL, ORO, SERPIENTE] {
                assert!(
                    !plain.contains(&c),
                    "{style}: holiday color on an ordinary day"
                );
            }
        }
    }

    /// The stripes go verde, blanco, rojo from the back to the front, however
    /// many waves there are.
    #[test]
    fn the_waves_make_a_tricolor() {
        for n in 1..=12 {
            let s: Vec<_> = (0..n).map(|li| stripe(li, n)).collect();
            assert!(s.windows(2).all(|w| w[0] <= w[1]), "{n}: {s:?}");
            assert_eq!(s.last(), Some(&if n == 1 { 1 } else { 2 }), "{n}: {s:?}");
            if n >= 3 {
                assert_eq!(s[0], 0, "{n}: {s:?}");
                assert!(s.contains(&1), "{n}: {s:?}");
            }
        }
    }

    /// More of it as the mes patrio goes on: banderitas close together, more
    /// águilas, a stronger wind.
    #[test]
    fn more_of_it_toward_the_grito() {
        let classic = Theme::classic();
        for style in ["lines", "mirror"] {
            let early = run(&classic, style, Fiesta::of_day(9, 1), 10).0;
            let grito = run(&classic, style, Fiesta::of_day(9, 15), 10).0;
            let (e, g) = (early.patrias.unwrap(), grito.patrias.unwrap());
            assert!(
                e.flagged < g.flagged,
                "{style}: {} {}",
                e.flagged,
                g.flagged
            );
            assert!(e.eagles(240) < g.eagles(240));
            assert!(e.level < g.level);
        }
        // Never crowding a narrow screen.
        let m = run(&classic, "lines", Fiesta::of_day(9, 15), 1)
            .0
            .patrias
            .unwrap();
        assert_eq!(m.eagles(100), 1);
        assert_eq!(m.eagles(240), 2);
        assert_eq!(m.eagles(400), 3);
    }

    /// The holiday comes and goes through `retheme`, and leaves the ordinary
    /// picture exactly as it was.
    #[test]
    fn patrias_comes_and_goes() {
        let theme = Theme::builtin("dracula").unwrap();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let mut ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: theme.bg,
            seed: 9,
            fiesta: Fiesta::NONE,
        };
        let frame = |s: &mut Waveform, ctx: &Ctx| {
            let mut c = Canvas::new(90, 30, Color::Reset);
            s.draw(ctx, &mut c);
            c.cells().to_vec()
        };
        let mut plain = Waveform::new(&ctx);
        let mut s = Waveform::new(&ctx);
        for w in [&mut plain, &mut s] {
            w.resize(&ctx, 90, 30);
            w.retheme(&ctx);
        }
        let before = frame(&mut s, &ctx);
        ctx.fiesta = Fiesta::of_day(9, 10);
        s.retheme(&ctx);
        for _ in 0..90 {
            s.update(&ctx, 1.0 / 30.0);
        }
        assert!(s.patrias.is_some());
        assert_ne!(frame(&mut s, &ctx), before);
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert!(s.patrias.is_none());
        for _ in 0..90 {
            plain.update(&ctx, 1.0 / 30.0);
        }
        assert!(frame(&mut s, &ctx) == frame(&mut plain, &ctx));
        // And back again, with a different day: the same scene, redressed.
        ctx.fiesta = Fiesta::of_day(9, 1);
        s.retheme(&ctx);
        assert_eq!(s.patrias.as_ref().map(|m| m.level), Some(0.2));
        ctx.fiesta = Fiesta::of_day(9, 15);
        s.retheme(&ctx);
        assert_eq!(s.patrias.as_ref().map(|m| m.level), Some(1.0));
        // Another holiday leaves it ordinary.
        ctx.fiesta = Fiesta::of_day(11, 20);
        s.retheme(&ctx);
        assert!(s.patrias.is_none() && s.navidad.is_none() && s.muertos.is_none());
    }

    /// The same seed paints the same holiday, and it follows the theme.
    #[test]
    fn patrias_is_deterministic_and_themed() {
        let day = Fiesta::of_day(9, 15);
        let nord = Theme::builtin("nord").unwrap();
        let (_, a) = run(&nord, "lines", day, 15);
        let (_, b) = run(&nord, "lines", day, 15);
        assert!(a.cells() == b.cells());
        let (_, classic) = run(&Theme::classic(), "lines", day, 15);
        assert!(a.cells() != classic.cells());
        // Under another theme nothing is the flag's own color.
        let on = colors(&a);
        for c in [VERDE, BLANCO, ROJO, AGUILA, NOPAL, ORO, SERPIENTE] {
            assert!(!on.contains(&c), "a classic color under nord");
        }
    }

    /// The águila stands on the middle wave and follows it up and down.
    #[test]
    fn the_aguila_rides_the_middle_wave() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 5,
            fiesta: Fiesta::of_day(9, 1),
        };
        let mut s = Waveform::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let plumage = s.patrias.as_ref().unwrap().emblem;
        let mut c = Canvas::new(120, 36, Color::Reset);
        // The rows her plumage covers, over a minute: she bobs up and down.
        let mut rows = std::collections::BTreeSet::new();
        for f in 0..1800 {
            s.update(&ctx, 1.0 / 30.0);
            if f % 15 == 0 {
                c.clear();
                s.draw(&ctx, &mut c);
                let (top, _) = (0..36).fold((36, 0), |(t, b), y| {
                    let any = (0..120).any(|x| {
                        c.get(x, y)
                            .is_some_and(|cell| cell.ch != ' ' && plumage[..2].contains(&cell.fg))
                    });
                    if any { (t.min(y), b.max(y)) } else { (t, b) }
                });
                rows.insert(top);
            }
        }
        assert!(rows.len() >= 3, "the águila never moves: {rows:?}");
    }
}
