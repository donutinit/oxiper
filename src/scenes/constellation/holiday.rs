//! The sky's holidays: now and then a handful of stars leave their drift,
//! glide together and draw a figure with their links, a spark traces it, the
//! icing and flowers light up, and after a while the stars let go and drift
//! on. This is the machinery every holiday shares; each one brings a `Dress`
//! with its own figures and colors (`muertos`, `navidad`, `patrias` and
//! `equinoccio`).
//!
//! Figures are drawn in design units (square, like the scene's own units)
//! and scaled to the screen. Every node is a real star; the icing (`decor`,
//! dotted lines) and the flowers (`marks`, glyphs) belong to the figure.

use std::f32::consts::{PI, TAU};

use crate::color::{Color, mix_oklab, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Braille, Canvas};
use crate::scene::Ctx;

use super::{DISSOLVE, LINK_FLOOR, Star, muertos, navidad, patrias};

/// Most stars a figure takes.
pub(super) const MAX_NODES: usize = 40;
/// Most colors a holiday has.
pub(super) const MAX_HUES: usize = 16;
/// Most figures in the sky at once.
const MAX_DRAWINGS: usize = 3;
/// A star's `fig` while its figure lets it go: it drifts on its own again,
/// but no new figure may take it yet.
pub(super) const LETTING_GO: u8 = u8::MAX;

/// Real seconds the stars take to gather, the spark to trace the figure, the
/// figure to stay, and the stars to let go.
const GATHER: f32 = 4.2;
const TRACE: f32 = 2.6;
const HOLD: f32 = 9.0;
const LET_GO: f32 = 3.6;
const LIFE: f32 = GATHER + TRACE + HOLD + LET_GO;
/// How lit an edge is while its stars are still gathering.
const SKETCH: f32 = 0.22;

/// How a figure moves while it stays. Points are in design units.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Anim {
    Still,
    /// The wings open and close.
    Flap,
    /// Everything above `y` (the flame) flickers.
    Flicker(f32),
    /// Everything below the pivot (`px`, `py`) and within `hw` of its
    /// column swings to and fro by up to `amp` radians (a bell).
    Swing {
        px: f32,
        py: f32,
        hw: f32,
        amp: f32,
    },
    /// A wave runs along `x` (a flag, a serpent), rising from nothing at
    /// `from` to `amp` design units over `span`.
    Wave {
        from: f32,
        span: f32,
        amp: f32,
    },
    /// Everything farther than `r0` from (`cx`, `cy`) breathes in and out
    /// by up to `amp` of its distance (a star's rays).
    Pulse {
        cx: f32,
        cy: f32,
        r0: f32,
        amp: f32,
    },
    /// Everything below `top` (the legs) swings back and forth, `amp` units.
    Walk {
        top: f32,
        amp: f32,
    },
}

impl Anim {
    /// How far the motion takes points out of the figure's box.
    fn pad(self) -> f32 {
        match self {
            Anim::Still | Anim::Flap | Anim::Flicker(_) => 0.0,
            Anim::Swing { amp, .. } => 22.0 * amp,
            Anim::Wave { amp, .. } | Anim::Walk { amp, .. } => amp,
            Anim::Pulse { r0, amp, .. } => 2.0 * r0 * amp,
        }
    }
}

pub(super) struct Figure {
    pub nodes: &'static [(f32, f32)],
    /// (from, to, hue), in the order the spark traces them.
    pub edges: &'static [(u8, u8, u8)],
    /// Icing: (x0, y0, x1, y1, hue, stride), dotted lines.
    pub decor: &'static [(f32, f32, f32, f32, u8, u8)],
    /// Flowers and details: (x, y, glyph, hue).
    pub marks: &'static [(f32, f32, char, u8)],
    pub anim: Anim,
    pub weight: u32,
}

/// What a holiday brings to the sky: its figures, its colors and its pace.
pub(super) struct Dress {
    pub kind: Kind,
    /// Mixed into the scene's seed for the holiday's own dice.
    pub salt: u64,
    pub figures: &'static [&'static Figure],
    /// How often a figure comes, given its `weight`, its index and the day.
    pub weigh: fn(u32, usize, Fiesta) -> u32,
    /// Whether two of the same figure may be in the sky at once.
    pub distinct: bool,
    /// The holiday's colors under `classic`, which figures name by index.
    pub native: &'static [u32],
    /// Colors that become the theme's brightest under any other theme, and
    /// colors (a flame) that keep their own whatever the theme.
    pub bright: &'static [u8],
    pub keep: &'static [u8],
    /// Star tints during the holiday (the same weights as the ordinary
    /// ones), which of them lean on the theme's brightest, and what the
    /// ordinary links lean toward.
    pub stars: [u32; 7],
    pub pale: &'static [usize],
    pub link: u32,
    /// The meteors' head and the color (an index) their tails fade to, and
    /// the spark that traces the figures (an index too).
    pub meteor: u32,
    pub tail: u8,
    pub pen: u8,
    /// Real seconds between figures at the start of the holiday and at its
    /// height, and until the first one.
    pub gap: (f32, f32),
    pub first: f32,
}

/// Every holiday the sky dresses for.
const DRESSES: [&Dress; 3] = [&muertos::DRESS, &navidad::DRESS, &patrias::DRESS];

/// One figure in the sky.
#[derive(Clone, Copy)]
struct Drawing {
    on: bool,
    fig: usize,
    /// Real seconds since the stars were called.
    age: f32,
    /// Where the figure's center is, in field units, and how it drifts.
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Field units per design unit.
    k: f32,
    /// The figure's design-space center.
    cx: f32,
    cy: f32,
    /// Half its size in field units, for keeping figures apart.
    hw: f32,
    hh: f32,
    /// The star on every node, where it was called from, when it sets off
    /// and how its path bends.
    stars: [u32; MAX_NODES],
    from: [(f32, f32); MAX_NODES],
    delay: [f32; MAX_NODES],
    bend: [f32; MAX_NODES],
}

impl Drawing {
    const OFF: Drawing = Drawing {
        on: false,
        fig: 0,
        age: 0.0,
        x: 0.0,
        y: 0.0,
        vx: 0.0,
        vy: 0.0,
        k: 1.0,
        cx: 0.0,
        cy: 0.0,
        hw: 0.0,
        hh: 0.0,
        stars: [0; MAX_NODES],
        from: [(0.0, 0.0); MAX_NODES],
        delay: [0.0; MAX_NODES],
        bend: [0.0; MAX_NODES],
    };

    /// Design units to field units, with the figure's own motion.
    fn place(&self, f: &Figure, x: f32, y: f32) -> (f32, f32) {
        let (mut dx, mut dy) = (x - self.cx, y - self.cy);
        match f.anim {
            Anim::Still => {}
            Anim::Flap => dx *= 0.78 + 0.22 * (self.age * 1.7).cos(),
            Anim::Flicker(top) if y < top => {
                let up = (top - y) / top.max(1.0);
                dx += up * up * 0.9 * ((self.age * 5.3).sin() + 0.5 * (self.age * 8.9).sin());
                dy -= up * 0.5 * (self.age * 6.1).sin();
            }
            Anim::Flicker(_) => {}
            Anim::Swing { px, py, hw, amp } if y >= py && (x - px).abs() <= hw => {
                let (sin, cos) = (amp * (self.age * 2.1).sin()).sin_cos();
                let (ux, uy) = (x - px, y - py);
                dx = px + ux * cos - uy * sin - self.cx;
                dy = py + ux * sin + uy * cos - self.cy;
            }
            Anim::Swing { .. } => {}
            Anim::Wave { from, span, amp } => {
                let ramp = ((x - from) / span).clamp(0.0, 1.0);
                dy += amp * ramp * (x * 0.5 - self.age * 2.6).sin();
            }
            Anim::Pulse { cx, cy, r0, amp } => {
                let (ux, uy) = (x - cx, y - cy);
                let r = ux.hypot(uy);
                if r > r0 {
                    let g = amp * ((r - r0) / r0).min(1.0) * (self.age * 2.4).sin();
                    dx += ux * g;
                    dy += uy * g;
                }
            }
            Anim::Walk { top, amp } if y > top => {
                let down = ((y - top) / 8.0).min(1.0);
                dx += amp * down * (self.age * 3.2 + x * 0.55).sin();
            }
            Anim::Walk { .. } => {}
        }
        (self.x + dx * self.k, self.y + dy * self.k)
    }

    /// How lit the finished figure is: up as the spark traces it, down as
    /// the stars let go.
    fn lit(&self) -> f32 {
        let a = self.age;
        if a < GATHER {
            0.0
        } else if a < GATHER + TRACE + HOLD {
            1.0
        } else {
            1.0 - smooth((a - GATHER - TRACE - HOLD) / LET_GO)
        }
    }

    /// How much the figure is there, from the stars' call to their release.
    fn here(&self) -> f32 {
        smooth(self.age / GATHER) * (1.0 - smooth((self.age - GATHER - TRACE - HOLD) / LET_GO))
    }

    /// How much of the icing and flowers shows.
    fn trim(&self) -> f32 {
        let t = self.age - GATHER - TRACE;
        let gone = self.age - GATHER - TRACE - HOLD;
        (smooth(t / 1.4) * (1.0 - smooth(gone / (LET_GO * 0.5)))).clamp(0.0, 1.0)
    }
}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// The holiday's part of the scene.
pub(super) struct Holiday {
    pub on: bool,
    /// What the holiday brings (Día de Muertos's until one is on).
    dress: &'static Dress,
    fiesta: Fiesta,
    level: f32,
    seed: u64,
    /// The holiday's own dice, so ordinary days roll exactly as before, and
    /// the holiday they were seeded for.
    rng: fastrand::Rng,
    seeded: Kind,
    drawings: [Drawing; MAX_DRAWINGS],
    /// Real seconds until the next figure may begin.
    next: f32,
    last: usize,
    pal: [Color; MAX_HUES],
    /// The spark that traces figures.
    pen: Color,
}

impl Holiday {
    pub fn new(seed: u64) -> Holiday {
        Holiday {
            on: false,
            dress: &muertos::DRESS,
            fiesta: Fiesta::NONE,
            level: 0.0,
            seed,
            rng: fastrand::Rng::with_seed(seed ^ muertos::DRESS.salt),
            seeded: Kind::Muertos,
            drawings: [Drawing::OFF; MAX_DRAWINGS],
            next: 0.0,
            last: usize::MAX,
            pal: [Color::Reset; MAX_HUES],
            pen: Color::Reset,
        }
    }

    /// Follows the holiday; lets go of every figure when it ends.
    pub fn set(&mut self, fiesta: Fiesta, stars: &mut [Star]) {
        let dress = DRESSES.into_iter().find(|d| fiesta.is(d.kind));
        let on = dress.is_some();
        if let Some(dress) = dress {
            if !std::ptr::eq(dress, self.dress) {
                // Another holiday's figures: let go of the ones in the sky.
                self.cancel(stars);
                self.dress = dress;
            }
            if self.seeded != dress.kind {
                self.rng = fastrand::Rng::with_seed(self.seed ^ dress.salt);
                self.seeded = dress.kind;
            }
            if !self.on {
                // The first figure soon after the holiday arrives.
                self.next = dress.first;
            }
        }
        if !on {
            self.cancel(stars);
        }
        self.on = on;
        self.fiesta = fiesta;
        self.level = if on {
            fiesta.level.clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    /// Releases every star at once (the screen changed size, or the holiday
    /// ended).
    pub fn cancel(&mut self, stars: &mut [Star]) {
        for s in stars.iter_mut() {
            s.fig = 0;
            s.glow = 0.0;
            s.gain = 1.0;
        }
        for d in &mut self.drawings {
            d.on = false;
        }
    }

    /// Dresses the sky: the holiday's star tints, the ordinary links leaning
    /// toward its hue, its meteors.
    pub fn retheme(
        &mut self,
        ctx: &Ctx,
        tints: &mut [Color; 7],
        link_tints: &mut [Color; 7],
        meteor: &mut Color,
        meteor_tail: &mut Color,
    ) {
        let dress = self.dress;
        let top = ctx.ramp(&[]).at(1.0);
        let tone = |hex: u32| -> Color {
            if ctx.theme.native {
                rgb(hex)
            } else {
                nearest(ctx, rgb(hex))
            }
        };
        for (i, &hex) in dress.native.iter().enumerate() {
            self.pal[i] = tone(hex);
        }
        if !ctx.theme.native {
            for &h in dress.bright {
                self.pal[h as usize] = top;
            }
            for &h in dress.keep {
                self.pal[h as usize] = rgb(dress.native[h as usize]);
            }
        }
        let link = tone(dress.link);
        for (i, &hex) in dress.stars.iter().enumerate() {
            tints[i] = if ctx.theme.native {
                rgb(hex)
            } else if dress.pale.contains(&i) {
                // The pale ones lean on the theme's brightest.
                mix_oklab(top, tone(hex), 0.3)
            } else {
                tone(hex)
            };
            link_tints[i] = mix_oklab(tints[i], link, 0.55);
        }
        *meteor = if ctx.theme.native {
            rgb(dress.meteor)
        } else {
            top
        };
        *meteor_tail = self.pal[dress.tail as usize];
        self.pen = self.pal[dress.pen as usize];
    }

    /// Runs the figures' clocks (`real` seconds) and moves their stars
    /// (`dt`, scene time).
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        stars: &mut [Star],
        sw: f32,
        sh: f32,
        margin: f32,
        dt: f32,
        real: f32,
        lines: bool,
    ) {
        if !self.on {
            return;
        }
        let figs = self.dress.figures;
        let real = if real.is_finite() {
            real.clamp(0.0, 1.0)
        } else {
            0.0
        };
        // Stars a figure called fade their ordinary links out; stars let go
        // fade them back in.
        for s in stars.iter_mut() {
            if s.fig == 0 || s.fig == LETTING_GO {
                if s.gain < 1.0 {
                    s.gain = (s.gain + real / 2.5).min(1.0);
                }
            } else {
                s.gain = (s.gain - real / 0.6).max(0.0);
            }
        }
        for slot in 0..MAX_DRAWINGS {
            let d = &mut self.drawings[slot];
            if !d.on {
                continue;
            }
            let was = d.age;
            d.age += real;
            // A slow drift with the sky, never off the screen.
            d.x = (d.x + d.vx * dt).clamp(
                margin + d.hw + 1.0,
                margin + (sw - d.hw - 1.0).max(d.hw + 1.0),
            );
            d.y = (d.y + d.vy * dt).clamp(
                margin + d.hh + 1.0,
                margin + (sh - d.hh - 1.0).max(d.hh + 1.0),
            );
            let f = figs[d.fig];
            let n = f.nodes.len();
            if d.age >= LIFE {
                for &i in &d.stars[..n] {
                    let s = &mut stars[i as usize];
                    s.fig = 0;
                    s.glow = 0.0;
                }
                d.on = false;
                continue;
            }
            let letting = d.age >= GATHER + TRACE + HOLD;
            if letting && was < GATHER + TRACE + HOLD {
                for &i in &d.stars[..n] {
                    stars[i as usize].fig = LETTING_GO;
                }
            }
            let lit = d.lit();
            for (j, &(nx, ny)) in f.nodes.iter().enumerate() {
                let s = &mut stars[d.stars[j] as usize];
                if letting {
                    s.glow = lit;
                    continue;
                }
                let (tx, ty) = d.place(f, nx, ny);
                let (fx, fy) = d.from[j];
                let go = smooth((d.age - d.delay[j]) / (GATHER - d.delay[j]));
                // A gentle arc on the way in.
                let (ox, oy) = (tx - fx, ty - fy);
                let arc = d.bend[j] * (go * PI).sin();
                s.x = fx + ox * go - oy * arc;
                s.y = fy + oy * go + ox * arc;
                s.glow = if d.age < GATHER { 0.55 * go } else { 1.0 };
            }
        }

        self.next -= real;
        let busy = self.drawings.iter().filter(|d| d.on).count();
        let most = if self.level >= 0.95 {
            3
        } else if self.level >= 0.6 {
            2
        } else {
            1
        };
        if self.next <= 0.0 && busy < most && lines {
            self.next = if self.spawn(stars, sw, sh, margin) {
                let (far, near) = self.dress.gap;
                let gap = far + (near - far) * self.level;
                gap * (0.8 + 0.4 * self.rng.f32())
            } else {
                1.5
            };
        }
    }

    /// Calls the stars for a new figure, if there is room for one.
    fn spawn(&mut self, stars: &mut [Star], sw: f32, sh: f32, margin: f32) -> bool {
        let Some(slot) = self.drawings.iter().position(|d| !d.on) else {
            return false;
        };
        let (dress, fiesta) = (self.dress, self.fiesta);
        let figs = dress.figures;
        let weigh = |i: usize| (dress.weigh)(figs[i].weight, i, fiesta);
        // Some holidays never draw the same figure twice at once.
        let showing = |i: usize| dress.distinct && self.drawings.iter().any(|d| d.on && d.fig == i);
        let total: u32 = (0..figs.len()).map(weigh).sum();
        if total == 0 {
            return false;
        }
        let r = &mut self.rng;
        let mut fig = 0;
        for _ in 0..4 {
            let mut pick = r.u32(0..total);
            for i in 0..figs.len() {
                if pick < weigh(i) {
                    fig = i;
                    break;
                }
                pick -= weigh(i);
            }
            if fig != self.last && !showing(fig) {
                break;
            }
        }
        if showing(fig) {
            return false;
        }
        let f = figs[fig];
        let n = f.nodes.len();
        if stars.iter().filter(|s| s.fig == 0).count() < n {
            return false;
        }
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        let ends = f.decor.iter().flat_map(|d| [(d.0, d.1), (d.2, d.3)]);
        let marks = f.marks.iter().map(|m| (m.0, m.1));
        for (x, y) in f.nodes.iter().copied().chain(ends).chain(marks) {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
        let pad = 2.0 * f.anim.pad();
        let (dw, dh) = (x1 - x0 + pad, y1 - y0 + pad);
        // Tall enough to read, never wider than most of the screen.
        let want = (sh * 0.62).clamp(14.0, 58.0) * (0.88 + 0.24 * r.f32());
        let k = (want / dh).min(sw * 0.8 / dw);
        let (hw, hh) = (dw * k * 0.5, dh * k * 0.5);
        if hh * 2.0 < 12.0 || hw * 2.0 + 4.0 > sw || hh * 2.0 + 2.0 > sh {
            return false;
        }
        let mut spot = None;
        for _ in 0..12 {
            let x = hw + 2.0 + r.f32() * (sw - 2.0 * hw - 4.0).max(0.0);
            let y = hh + 1.0 + r.f32() * (sh - 2.0 * hh - 2.0).max(0.0);
            let clear = self.drawings.iter().all(|o| {
                !o.on || (x - o.x).abs() > hw + o.hw + 3.0 || (y - o.y).abs() > hh + o.hh + 2.0
            });
            if clear {
                spot = Some((x + margin, y + margin));
                break;
            }
        }
        let Some((x, y)) = spot else {
            return false;
        };
        let mut d = Drawing {
            on: true,
            fig,
            x,
            y,
            vx: (r.f32() - 0.5) * 0.35,
            vy: (r.f32() - 0.5) * 0.2,
            k,
            cx: (x0 + x1) * 0.5,
            cy: (y0 + y1) * 0.5,
            hw,
            hh,
            ..Drawing::OFF
        };
        // Nearest free star to every node, the nodes in a random order so no
        // part of the figure always gets the far ones.
        let mut order = [0u8; MAX_NODES];
        for (i, o) in order.iter_mut().enumerate().take(n) {
            *o = i as u8;
        }
        r.shuffle(&mut order[..n]);
        for &j in &order[..n] {
            let j = j as usize;
            let (tx, ty) = d.place(f, f.nodes[j].0, f.nodes[j].1);
            let mut best = (f32::MAX, 0);
            for (i, s) in stars.iter().enumerate() {
                if s.fig != 0 {
                    continue;
                }
                let d2 = (s.x - tx).powi(2) + (s.y - ty).powi(2);
                if d2 < best.0 {
                    best = (d2, i);
                }
            }
            let s = &mut stars[best.1];
            s.fig = slot as u8 + 1;
            d.stars[j] = best.1 as u32;
            d.from[j] = (s.x, s.y);
            d.delay[j] = r.f32() * 1.4;
            d.bend[j] = (r.f32() - 0.5) * 0.5;
        }
        self.drawings[slot] = d;
        self.last = fig;
        true
    }

    /// The figures' links and icing, in braille. Returns how many dots of
    /// finished (traced) links it drew.
    pub fn draw_lines(&self, ctx: &Ctx, stars: &[Star], br: &mut Braille, margin: f32) -> usize {
        let mut traced = 0;
        let pos = |i: u32| {
            let s = &stars[i as usize];
            ((s.x - margin) * 2.0, (s.y - margin) * 2.0)
        };
        for d in self.drawings.iter().filter(|d| d.on) {
            let f = self.dress.figures[d.fig];
            let lit = d.lit();
            // How far the spark has gone, in dots along the edges.
            let mut total = 0.0;
            for &(a, b, _) in f.edges {
                let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                total += (x1 - x0).hypot(y1 - y0);
            }
            let pen = if d.age < GATHER {
                0.0
            } else {
                smooth((d.age - GATHER) / TRACE) * total
            };
            let sketch = SKETCH * smooth(d.age / GATHER * 1.6 - 0.3);
            let mut run = 0.0;
            for (e, &(a, b, hue)) in f.edges.iter().enumerate() {
                let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                let len = (x1 - x0).hypot(y1 - y0);
                // The part the spark has traced is lit, the rest sketched.
                let done = if d.age >= GATHER + TRACE {
                    1.0
                } else {
                    ((pen - run) / len.max(1e-3)).clamp(0.0, 1.0)
                };
                run += len;
                let col = self.pal[hue as usize];
                let phase = d.age * 2.2 + e as f32 * 0.9;
                let n = (len.ceil() as usize).clamp(1, 4096);
                let inv = 1.0 / n as f32;
                for s in 0..=n {
                    let t = s as f32 * inv;
                    let alpha = if t <= done { lit.max(sketch) } else { sketch };
                    if alpha <= 0.0 {
                        continue;
                    }
                    if t <= done && lit >= 1.0 {
                        traced += 1;
                    }
                    if alpha < DISSOLVE {
                        let stride = (DISSOLVE / alpha.max(1e-3)).round() as usize;
                        if stride > 6 || s % stride != 0 {
                            continue;
                        }
                    }
                    // A glimmer runs along the finished figure.
                    let shine = 0.86 + 0.14 * (phase - t * len * 0.15).sin();
                    let v = LINK_FLOOR
                        + (1.0 - LINK_FLOOR)
                            * ((alpha - DISSOLVE) / (1.0 - DISSOLVE)).clamp(0.0, 1.0);
                    br.setf(
                        x0 + (x1 - x0) * t + 0.5,
                        y0 + (y1 - y0) * t + 0.5,
                        ctx.fade(col, 1.0 - v * shine),
                    );
                }
            }
            let trim = d.trim();
            if trim > 0.05 {
                for &(x0, y0, x1, y1, hue, stride) in f.decor {
                    let (a, b) = (d.place(f, x0, y0), d.place(f, x1, y1));
                    let (a, b) = (
                        ((a.0 - margin) * 2.0, (a.1 - margin) * 2.0),
                        ((b.0 - margin) * 2.0, (b.1 - margin) * 2.0),
                    );
                    let stride = stride.max(1) as usize * if trim < 0.5 { 2 } else { 1 };
                    let col = ctx.fade(self.pal[hue as usize], 1.0 - trim.max(LINK_FLOOR) * 0.92);
                    let n =
                        ((b.0 - a.0).abs().max((b.1 - a.1).abs()).ceil() as usize).clamp(1, 4096);
                    for s in (0..=n).step_by(stride) {
                        let t = s as f32 / n as f32;
                        br.setf(
                            a.0 + (b.0 - a.0) * t + 0.5,
                            a.1 + (b.1 - a.1) * t + 0.5,
                            col,
                        );
                    }
                }
            }
        }
        traced
    }

    /// The figures' flowers and the tracing spark's head, over everything.
    pub fn draw_marks(&self, ctx: &Ctx, stars: &[Star], c: &mut Canvas, margin: f32) {
        for d in self.drawings.iter().filter(|d| d.on) {
            let f = self.dress.figures[d.fig];
            let trim = d.trim();
            if trim > 0.3 {
                for &(x, y, ch, hue) in f.marks {
                    let (px, py) = d.place(f, x, y);
                    let col = ctx.fade(self.pal[hue as usize], 1.0 - trim);
                    let attr = if trim > 0.8 && ch != '·' { BOLD } else { 0 };
                    c.set_attr(
                        (px - margin).floor() as i32,
                        ((py - margin) * 0.5).floor() as i32,
                        ch,
                        col,
                        attr,
                    );
                }
            }
            if d.age >= GATHER && d.age < GATHER + TRACE {
                // Where the spark is along the edges.
                let pos = |i: u32| {
                    let s = &stars[i as usize];
                    (s.x - margin, s.y - margin)
                };
                let mut total = 0.0;
                for &(a, b, _) in f.edges {
                    let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                    total += (x1 - x0).hypot(y1 - y0);
                }
                let mut left = smooth((d.age - GATHER) / TRACE) * total;
                for &(a, b, _) in f.edges {
                    let ((x0, y0), (x1, y1)) = (pos(d.stars[a as usize]), pos(d.stars[b as usize]));
                    let len = (x1 - x0).hypot(y1 - y0);
                    if left <= len {
                        let t = left / len.max(1e-3);
                        let (x, y) = (x0 + (x1 - x0) * t, y0 + (y1 - y0) * t);
                        c.set_attr(
                            x.floor() as i32,
                            (y * 0.5).floor() as i32,
                            '✦',
                            self.pen,
                            BOLD,
                        );
                        break;
                    }
                    left -= len;
                }
            }
        }
    }

    /// How much an ordinary link from (ax, ay) to (bx, by) shows: the sky
    /// clears around the figures.
    pub fn clear(&self, ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
        let mut k = 1.0;
        for d in self.drawings.iter().filter(|d| d.on) {
            let here = d.here();
            if here <= 0.0 {
                continue;
            }
            let (mx, my) = ((ax + bx) * 0.5, (ay + by) * 0.5);
            let mut inside: f32 = 0.0;
            for (x, y) in [(ax, ay), (mx, my), (bx, by)] {
                let ox = ((x - d.x).abs() - d.hw - 1.0).max(0.0);
                let oy = ((y - d.y).abs() - d.hh - 1.0).max(0.0);
                inside = inside.max(1.0 - (ox.max(oy) / 4.0).min(1.0));
            }
            k *= 1.0 - here * inside;
        }
        k
    }

    /// How a star of a figure shows: its color leaning to its edges' hue.
    pub fn node_color(&self, s: &Star) -> Color {
        self.pal[s.hue as usize]
    }

    /// Gives each called star the hue of its first edge, for `node_color`.
    pub fn paint_nodes(&self, stars: &mut [Star]) {
        for d in self.drawings.iter().filter(|d| d.on) {
            let f = self.dress.figures[d.fig];
            for &(a, b, hue) in f.edges.iter().rev() {
                stars[d.stars[a as usize] as usize].hue = hue;
                stars[d.stars[b as usize] as usize].hue = hue;
            }
        }
    }

    #[cfg(test)]
    pub fn figures_on(&self) -> usize {
        self.drawings.iter().filter(|d| d.on).count()
    }

    /// Which figures are in the sky now, one bit each by index.
    #[cfg(test)]
    pub fn showing(&self) -> u32 {
        self.drawings
            .iter()
            .filter(|d| d.on)
            .fold(0, |m, d| m | 1 << d.fig)
    }
}

/// Does the sky dress for this holiday?
#[cfg(test)]
pub(super) fn dresses(kind: Kind) -> bool {
    DRESSES.iter().any(|d| d.kind == kind)
}

/// Every holiday's figures fit the node budget, link only their own nodes
/// and use only their holiday's colors.
#[cfg(test)]
pub(super) fn check_figures() {
    for dress in DRESSES {
        let hues = dress.native.len();
        assert!((1..=MAX_HUES).contains(&hues));
        let colors = [dress.tail, dress.pen]
            .into_iter()
            .chain(dress.bright.iter().copied())
            .chain(dress.keep.iter().copied());
        assert!(colors.into_iter().all(|h| (h as usize) < hues));
        assert!(dress.pale.iter().all(|&i| i < 7));
        // Something can always be drawn, on any day of the holiday.
        assert!(!dress.figures.is_empty());
        for (i, f) in dress.figures.iter().enumerate() {
            let what = format!("{:?} figure {i}", dress.kind);
            let n = f.nodes.len();
            assert!((3..=MAX_NODES).contains(&n), "{what}: {n} nodes");
            for &(a, b, hue) in f.edges {
                assert!((a as usize) < n && (b as usize) < n && a != b, "{what}");
                assert!((hue as usize) < hues, "{what}");
            }
            // Every node is on some edge, or its star would just sit there.
            for j in 0..n as u8 {
                assert!(
                    f.edges.iter().any(|e| e.0 == j || e.1 == j),
                    "{what} node {j}"
                );
            }
            assert!(
                f.decor.iter().all(|d| (d.4 as usize) < hues && d.5 >= 1),
                "{what}"
            );
            assert!(
                f.marks
                    .iter()
                    .all(|m| (m.3 as usize) < hues && !crate::render::is_wide(m.2)),
                "{what}"
            );
        }
    }
}

/// The theme accent closest in hue to a holiday color (theme colors that
/// are not RGB leave the holiday color as it is).
fn nearest(ctx: &Ctx, want: Color) -> Color {
    let Some((h0, s0, v0)) = hsv(want) else {
        return want;
    };
    let mut best = (f32::MAX, want);
    for &c in &ctx.theme.colors {
        let Some((h, s, v)) = hsv(c) else {
            continue;
        };
        let dh = (h - h0).abs().min(TAU - (h - h0).abs());
        let score = dh + (s - s0).abs() * 0.8 + (v - v0).abs() * 0.4;
        if score < best.0 {
            best = (score, c);
        }
    }
    best.1
}

/// Hue (radians), saturation and value of an RGB color.
fn hsv(c: Color) -> Option<(f32, f32, f32)> {
    let Color::Rgb(r, g, b) = c else {
        return None;
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let (mx, mn) = (r.max(g).max(b), r.min(g).min(b));
    let d = mx - mn;
    let h = if d <= 1e-6 {
        0.0
    } else if mx == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if mx == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    Some((h / 6.0 * TAU, if mx > 0.0 { d / mx } else { 0.0 }, mx))
}
