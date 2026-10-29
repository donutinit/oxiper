//! waveform: layered waves rolling across the screen.
//!
//! Each layer is a sum of a few sines plus slow value noise, scrolling at its
//! own pace (back layers slower, smaller and dimmer), drawn as smooth braille
//! lines, dithered fills, beads, or a mirrored oscilloscope trace.
//!
//! On Día de Muertos the waves are garlands of petals in papel picado colors
//! (cempasúchil in front, then rosa mexicano, amarillo, turquesa, morado and
//! verde limón): a thread along each wave with a petal up and a petal down
//! every few dots, and pompons of marigold and terciopelo, and now and then a
//! sugar skull, strung on the front ones. Beads turn into pompons, and the
//! filled style's dither into cut tissue paper, a diamond lattice thinning
//! out downward. Veladoras burn on the crests, on more of the waves as the
//! holiday comes (all of them by November 2): the flames flicker, light up
//! the petals around them and send embers rising, mirrored like reflections
//! on a lake in the oscilloscope, while petals blow off the waves and drift
//! away. Under other themes the garlands take the theme's colors; the
//! marigolds, the skulls and the flames keep their own. All of it lives in
//! `waveform/muertos.rs`.

mod muertos;

use std::f32::consts::{PI, TAU};

use crate::color::{Color, rgb};
use crate::fiesta::Kind;
use crate::render::{Braille, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use muertos::Muertos;

pub const INFO: SceneInfo = SceneInfo {
    name: "waveform",
    aliases: &["wave", "waves"],
    about: "layered waves rolling across the screen",
    options: &[
        Opt {
            key: "style",
            default: Val::S("lines"),
            help: "lines, filled (dithered area under each wave), dots, or mirror",
        },
        Opt {
            key: "layers",
            default: Val::I(5),
            help: "How many waves (1-12)",
        },
        Opt {
            key: "amplitude",
            default: Val::F(1.0),
            help: "Wave height, relative to the screen",
        },
        Opt {
            key: "frequency",
            default: Val::F(1.0),
            help: "How many crests fit on the screen (relative)",
        },
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast the waves roll",
        },
        Opt {
            key: "thickness",
            default: Val::I(1),
            help: "Line thickness in braille dots (1-4)",
        },
        Opt {
            key: "noise",
            default: Val::F(1.0),
            help: "Slow organic wobble on top of the sines (0 = pure sines)",
        },
    ],
    make: |ctx| Box::new(Waveform::new(ctx)),
};

/// Dusk under `classic`: deep violet through orchid and rose to peach.
const DUSK: [Color; 5] = [
    rgb(0x2d1b69),
    rgb(0x6a3fb5),
    rgb(0xc05cc4),
    rgb(0xff7fa8),
    rgb(0xffc59e),
];

/// Shades per layer: 0 is the layer's full color, higher fades toward the background.
const SHADES: usize = 16;

/// 4×4 ordered-dither thresholds (0..16).
const BAYER: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Lines,
    Filled,
    Dots,
    Mirror,
}

#[derive(Clone, Copy)]
struct Wave {
    /// Cycles per base wavelength, phase, angular speed, weight.
    k: f32,
    phase: f32,
    omega: f32,
    a: f32,
}

/// One slice of the wave surface.
struct Layer {
    /// 0 = farthest back, 1 = front.
    depth: f32,
    /// Where this slice cuts the shared swell (a sideways shift, in radians
    /// of the base wave), so neighbouring layers echo each other.
    lag: f32,
    /// A small ripple of its own.
    ripple: Wave,
    seed: f32,
}

/// Smooth 1D gradient (Perlin) noise, roughly -1..1. Unlike value noise it
/// has no flat spots at the lattice points.
fn noise1(x: f32) -> f32 {
    let i = x.floor();
    let f = x - i;
    let grad = |n: f32| {
        let n = (n as i64 as u32).wrapping_mul(0x9e37_79b9) ^ 0x85eb_ca6b;
        let n = (n ^ (n >> 15)).wrapping_mul(0x2c1b_3c6d);
        let n = n ^ (n >> 12);
        (n & 0xffff) as f32 / 32767.5 - 1.0
    };
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    let (a, b) = (grad(i) * f, grad(i + 1.0) * (f - 1.0));
    2.2 * (a + (b - a) * u)
}

struct Waveform {
    style: Style,
    amplitude: f32,
    frequency: f32,
    speed: f32,
    thickness: i32,
    noise: f32,
    /// The swell all layers share: a few sines with deep-water dispersion
    /// (longer waves travel faster), so the shape slowly morphs as it rolls.
    swell: [Wave; 3],
    drift: f32,
    layers: Vec<Layer>,
    w: usize,
    h: usize,
    pw: usize,
    ph: usize,
    t: f32,
    braille: Braille,
    /// Crest height per layer and dot column.
    ys: Vec<f32>,
    /// Mirror style's lens-shaped envelope per dot column.
    env: Vec<f32>,
    /// Per layer, `SHADES` colors from full to faint.
    shades: Vec<Color>,
    /// Día de Muertos, while it lasts.
    muertos: Option<Muertos>,
}

impl Waveform {
    fn new(ctx: &Ctx) -> Waveform {
        let o = ctx.opts;
        let style = match o.s("style").to_ascii_lowercase().as_str() {
            "filled" | "fill" | "area" => Style::Filled,
            "dots" | "dotted" | "beads" => Style::Dots,
            "mirror" | "mirrored" | "scope" | "oscilloscope" => Style::Mirror,
            _ => Style::Lines,
        };
        let n = o.i("layers").clamp(1, 12) as usize;
        let mut rng = ctx.rng();
        // Deep water: ω ∝ √k.
        fn wave(rng: &mut fastrand::Rng, k: f32, a: f32, dir: f32) -> Wave {
            Wave {
                k,
                phase: rng.f32() * TAU,
                omega: dir * 0.75 * k.sqrt(),
                a,
            }
        }
        let k = [
            1.0 + 0.25 * rng.f32(),
            2.0 + 0.5 * rng.f32(),
            3.4 + 0.8 * rng.f32(),
        ];
        let swell = [
            wave(&mut rng, k[0], 0.7, 1.0),
            wave(&mut rng, k[1], 0.2, 1.0),
            wave(&mut rng, k[2], 0.05, 1.0),
        ];
        let layers = (0..n)
            .map(|i| {
                let depth = if n == 1 {
                    1.0
                } else {
                    i as f32 / (n - 1) as f32
                };
                let dir = if rng.bool() { 1.0 } else { -0.7 };
                Layer {
                    depth,
                    lag: (n - 1 - i) as f32 * 0.55,
                    ripple: {
                        let k = 5.0 + 2.0 * rng.f32();
                        wave(&mut rng, k, 0.03, dir)
                    },
                    seed: rng.f32() * 1000.0,
                }
            })
            .collect();
        // Non-finite numbers fall back to the defaults.
        let num = |k: &str, d: f32, lo: f32, hi: f32| {
            let v = o.f(k);
            if v.is_finite() { v.clamp(lo, hi) } else { d }
        };
        Waveform {
            style,
            amplitude: num("amplitude", 1.0, 0.0, 5.0),
            frequency: num("frequency", 1.0, 0.05, 20.0),
            speed: num("speed", 1.0, 0.0, 20.0),
            thickness: o.i("thickness").clamp(1, 4) as i32,
            noise: num("noise", 1.0, 0.0, 5.0),
            swell,
            drift: 0.18 + 0.08 * rng.f32(),
            layers,
            w: 0,
            h: 0,
            pw: 0,
            ph: 0,
            t: 0.0,
            braille: Braille::new(1, 1),
            ys: Vec::new(),
            env: Vec::new(),
            shades: Vec::new(),
            muertos: None,
        }
    }

    /// The base wavelength in dots: about three quarters of the screen, but
    /// between 40 and 90 columns, so waves look alike at any size.
    fn lambda(&self) -> f32 {
        (self.w as f32 * 0.75).clamp(40.0, 90.0) * 2.0 / self.frequency
    }

    /// Fills `ys` with every layer's crest for this frame.
    fn compute(&mut self) {
        let (pw, ph) = (self.pw, self.ph as f32);
        let lambda = self.lambda();
        let t = self.t;
        let n = self.layers.len();
        for (li, l) in self.layers.iter().enumerate() {
            let d = l.depth;
            let (base, amp) = match self.style {
                // Mirror nests the layers like harmonics on a scope: same phase,
                // growing amplitude.
                Style::Mirror => (0.5, 0.07 + 0.29 * d),
                Style::Filled => (0.3 + 0.44 * d, 0.06 + 0.06 * d),
                _ if n == 1 => (0.5, 0.2),
                _ => (0.24 + 0.52 * d, 0.08 + 0.07 * d),
            };
            let amp = amp * ph * self.amplitude;
            let base = base * ph;
            // Perspective: farther slices look compressed, so they also seem
            // to roll by more slowly.
            let mirror = self.style == Style::Mirror;
            let kx = TAU / lambda * if mirror { 1.0 } else { 1.0 + 0.45 * (1.0 - d) };
            let lag = if mirror { 0.0 } else { l.lag };
            // Each sine is a phasor turned by a fixed step per column: four
            // multiplies instead of a `sin` call.
            let mut ph4 = [(0.0f32, 0.0f32); 4];
            let mut rot = [(0.0f32, 0.0f32); 4];
            let mut amps = [0.0f32; 4];
            for (j, wv) in self
                .swell
                .iter()
                .chain(std::iter::once(&l.ripple))
                .enumerate()
            {
                let a0 = wv.k * lag - wv.omega * t + wv.phase;
                ph4[j] = a0.sin_cos();
                rot[j] = (wv.k * kx).sin_cos();
                amps[j] = wv.a;
            }
            let row = &mut self.ys[li * pw..(li + 1) * pw];
            for (x, y) in row.iter_mut().enumerate() {
                let xf = x as f32 * kx + lag;
                let mut s = 0.0;
                for j in 0..4 {
                    let (sn, cs) = ph4[j];
                    s += amps[j] * sn;
                    let (rs, rc) = rot[j];
                    ph4[j] = (sn * rc + cs * rs, cs * rc - sn * rs);
                }
                if self.noise > 0.0 {
                    s += self.noise
                        * (0.3 * noise1(xf * 0.55 - self.drift * t)
                            + 0.04 * noise1(xf * 1.7 - 0.4 * t + l.seed));
                }
                *y = match self.style {
                    // Mirror keeps the signed offset; an envelope pinches the
                    // trace toward the edges like a lens.
                    Style::Mirror => amp * s * self.env[x],
                    _ => base + amp * s,
                };
            }
        }
    }

    /// Lights dots `y0..=y1` in column `x` with one color.
    #[inline]
    fn span(b: &mut Braille, x: i32, y0: i32, y1: i32, c: Color) {
        let (lo, hi) = (y0.min(y1).max(0), y0.max(y1).min(b.ph() - 1));
        for y in lo..=hi {
            b.set(x, y, c);
        }
    }

    /// A smooth line through `ys`: each column covers the span to its
    /// neighbours' midpoints, so steep slopes stay connected.
    fn stroke(&mut self, li: usize, sign: f32, center: f32) {
        let pw = self.pw;
        let th = self.thickness;
        let t = self.t;
        let depth = self.layers[li].depth;
        // Back layers sit dimmer; a slow glint travels along each line.
        let base = ((1.0 - depth) * 7.0) as usize;
        for x in 0..pw {
            let y = |i: usize| center + sign * self.ys[li * pw + i];
            let yc = y(x);
            let yl = if x > 0 { (y(x - 1) + yc) * 0.5 } else { yc };
            let yr = if x + 1 < pw {
                (y(x + 1) + yc) * 0.5
            } else {
                yc
            };
            let lo = yc.min(yl).min(yr).round() as i32;
            let hi = yc.max(yl).max(yr).round() as i32 + th - 1;
            let glint =
                0.5 + 0.5 * (x as f32 * 0.045 - t * (0.6 + depth * 0.5) + li as f32 * 1.7).sin();
            let s = (base + (glint * 5.0) as usize).min(SHADES - 1);
            let c = self.shades[li * SHADES + s];
            Waveform::span(&mut self.braille, x as i32, lo, hi, c);
        }
    }

    fn draw_lines(&mut self) {
        for li in 0..self.layers.len() {
            self.stroke(li, 1.0, 0.0);
        }
    }

    fn draw_mirror(&mut self) {
        let center = self.ph as f32 * 0.5 - 0.5;
        for li in 0..self.layers.len() {
            self.stroke(li, 1.0, center);
            self.stroke(li, -1.0, center);
        }
    }

    fn draw_dots(&mut self) {
        let pw = self.pw;
        let th = self.thickness;
        for li in 0..self.layers.len() {
            let l = &self.layers[li];
            let gap = 3 + ((1.0 - l.depth) * 1.5) as usize;
            // Beads ride along with the main swell.
            let wv = self.swell[0];
            let kx = TAU / self.lambda() * (1.0 + 0.45 * (1.0 - l.depth));
            let shift = (wv.omega * self.t / (wv.k * kx)).rem_euclid(gap as f32) as usize;
            let base = ((1.0 - l.depth) * 6.0) as usize;
            let c = self.shades[li * SHADES + base];
            let mut x = shift % gap;
            while x < pw {
                let y = self.ys[li * pw + x].round() as i32;
                Waveform::span(&mut self.braille, x as i32, y, y + th - 1, c);
                if th > 1 {
                    Waveform::span(&mut self.braille, x as i32 + 1, y, y + th - 1, c);
                }
                x += gap;
            }
        }
    }

    fn draw_filled(&mut self) {
        let (pw, ph) = (self.pw, self.ph as i32);
        let n = self.layers.len();
        let th = self.thickness;
        // How far below its crest a layer's fill fades out.
        let reach = (self.ph as f32 * 0.26).max(4.0);
        for li in 0..n {
            let base = ((1.0 - self.layers[li].depth) * 5.0) as usize;
            for x in 0..pw {
                let top = self.ys[li * pw + x];
                // Hidden behind the crests of the layers in front.
                let mut bottom = ph as f32;
                for lj in li + 1..n {
                    bottom = bottom.min(self.ys[lj * pw + x]);
                }
                let (t0, b0) = (top.round() as i32, bottom.round() as i32);
                if b0 <= t0 {
                    continue;
                }
                // Bottom up, so each cell ends with the color of its highest dot.
                let end = b0.min(ph).min(t0 + reach as i32 + 1);
                for y in (t0.max(0)..end).rev() {
                    let depth = (y - t0) as f32 / reach;
                    // A solid crest, then a dithered fill thinning out below.
                    let crest = y - t0 < th;
                    let density = (1.0 - depth).powf(2.0) * 0.55;
                    if crest || (BAYER[(y & 3) as usize][x & 3] as f32) < density * 16.0 {
                        let s = if crest {
                            base
                        } else {
                            (base + (2.0 + depth * 8.0) as usize).min(SHADES - 1)
                        };
                        self.braille.set(x as i32, y, self.shades[li * SHADES + s]);
                    }
                }
            }
        }
    }
}

impl Scene for Waveform {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        let (w, h) = (w.max(1) as usize, h.max(1) as usize);
        self.w = w;
        self.h = h;
        self.pw = w * 2;
        self.ph = h * 4;
        self.braille.resize(w as u16, h as u16);
        self.ys.clear();
        self.ys.resize(self.layers.len() * self.pw, 0.0);
        let pw = self.pw as f32;
        self.env.clear();
        self.env
            .extend((0..self.pw).map(|x| 0.25 + 0.75 * (PI * (x as f32 + 0.5) / pw).sin()));
        if let Some(m) = self.muertos.as_mut() {
            m.resized(self.pw);
        }
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let g = ctx.ramp(&DUSK);
        let n = self.layers.len();
        // Without a known background fading only darkens; keep it gentle so
        // back layers never turn into dark shadows on a lighter terminal.
        let reach = if matches!(ctx.bg, Color::Rgb(..)) {
            0.85
        } else {
            0.55
        };
        let muertos = ctx.fiesta.is(Kind::Muertos);
        self.shades.clear();
        for (li, l) in self.layers.iter().enumerate() {
            // Front layers take the bright end of the ramp, back layers the
            // middle; ramps start near the background, so skip that end. On
            // Día de Muertos each wave is a papel picado color instead.
            let c = if muertos {
                muertos::picado(ctx, n - 1 - li)
            } else {
                g.at(0.38 + 0.62 * l.depth)
            };
            for s in 0..SHADES {
                let k = s as f32 / (SHADES - 1) as f32;
                self.shades.push(ctx.fade(c, k * reach));
            }
        }
        debug_assert_eq!(self.shades.len(), n * SHADES);
        if muertos {
            Muertos::dress(
                &mut self.muertos,
                ctx,
                &self.layers,
                &self.shades,
                reach,
                self.pw,
            );
        } else {
            self.muertos = None;
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 1.0)
        } else {
            0.0
        };
        // Keep the clock small so `sin` stays precise over long runs.
        self.t = (self.t + dt * self.speed) % 36_000.0;
        self.step_muertos(dt);
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        if self.shades.is_empty() {
            return;
        }
        // The holiday finds its candles from this frame's waves in `update`.
        let found = self
            .muertos
            .as_mut()
            .is_some_and(|m| std::mem::take(&mut m.fresh));
        if !found {
            self.compute();
        }
        self.braille.clear();
        if self.muertos.is_some() {
            self.draw_muertos();
        } else {
            match self.style {
                Style::Lines => self.draw_lines(),
                Style::Filled => self.draw_filled(),
                Style::Dots => self.draw_dots(),
                Style::Mirror => self.draw_mirror(),
            }
        }
        self.braille.blit(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::scene::Opts;
    use crate::theme::Theme;

    /// Every style and odd option survives tiny and huge screens and huge steps.
    #[test]
    fn variants_survive() {
        let variants = [
            "style = 'lines'",
            "style = 'filled'\nthickness = 4",
            "style = 'dots'\nthickness = 3",
            "style = 'mirror'\nlayers = 12",
            "layers = 1\nstyle = 'filled'",
            "amplitude = 5.0\nfrequency = 20.0\nspeed = 20.0\nnoise = 5.0",
            "amplitude = nan\nfrequency = inf\nspeed = -1.0\nlayers = -3",
        ];
        // Ordinary days, then Día de Muertos at its first day and at its height.
        let days = [Fiesta::NONE, Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)];
        for (v, fiesta) in variants
            .iter()
            .flat_map(|v| days.iter().map(move |d| (v, *d)))
        {
            for theme in [Theme::classic(), Theme::builtin("nord").unwrap()] {
                let opts = Opts::new(INFO.options, v.parse().unwrap());
                let ctx = Ctx {
                    theme: &theme,
                    opts: &opts,
                    bg: theme.bg,
                    seed: 11,
                    fiesta,
                };
                let mut s = (INFO.make)(&ctx);
                let sizes = [(80, 24), (1, 1), (2, 3), (7, 2), (500, 150), (3, 40)];
                for (i, &(w, h)) in sizes.iter().enumerate() {
                    s.resize(&ctx, w, h);
                    if i == 0 {
                        s.retheme(&ctx);
                    }
                    let mut c = Canvas::new(w, h, Color::Reset);
                    // The holiday on a huge screen is slow to run unoptimized: fewer frames.
                    let (frames, every) =
                        if fiesta.is(Kind::Muertos) && w as u32 * h as u32 > 10_000 {
                            (10, 4)
                        } else {
                            (40, 13)
                        };
                    for f in 0..frames {
                        s.update(
                            &ctx,
                            if f % every == every - 1 {
                                5.0
                            } else {
                                1.0 / 30.0
                            },
                        );
                        c.clear();
                        s.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }
}
