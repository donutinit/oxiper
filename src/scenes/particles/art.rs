//! What the holidays' costumes share: flowers cut into braille cells,
//! sprites, and the loose bits (petals, confetti, sweets) that ride the flow
//! or fall.
//!
//! None of it draws a random number on its own: the costume that owns a
//! thing hands in its own RNG, so an ordinary day's particles are never
//! disturbed.

use std::f32::consts::TAU;

use super::{Dots, Field};
use crate::color::Color;
use crate::render::Braille;

/// The fractional part, always in 0..1.
pub(super) fn fract(x: f32) -> f32 {
    x - x.floor()
}

/// How far around the circle an angle is, in turns (0..1).
pub(super) fn turns(a: f32) -> f32 {
    fract(a / TAU)
}

/// The angle of (x, y) in radians, to within 0.0003: all the
/// blossoms need to cut their petals, for a fraction of `f32::atan2`.
pub(super) fn atan2(y: f32, x: f32) -> f32 {
    let (ax, ay) = (x.abs(), y.abs());
    let (lo, hi) = if ax < ay { (ax, ay) } else { (ay, ax) };
    if hi == 0.0 {
        return 0.0;
    }
    let a = lo / hi;
    let s = a * a;
    let mut r = (((-0.046_496_474 * s + 0.159_314_22) * s - 0.327_622_77) * s) * a + a;
    if ay > ax {
        r = TAU / 4.0 - r;
    }
    if x < 0.0 {
        r = TAU / 2.0 - r;
    }
    if y < 0.0 { -r } else { r }
}

/// `v` rounded to a whole number of dots, for coordinates on or near the
/// screen: cheaper than `f32::round`.
pub(super) fn rnd(v: f32) -> i32 {
    (v + 8192.5) as i32 - 8192
}

/// Empties the braille cell holding dot (x, y), so what is drawn there next
/// stands in front of the trails.
pub(super) fn clear_cell(br: &mut Braille, x: i32, y: i32) {
    let (x0, y0) = (x & !1, y & !3);
    if x0 < 0 || y0 < 0 || x0 >= br.pw() || y0 >= br.ph() {
        return;
    }
    for k in 0..8 {
        br.unset(x0 + (k & 1), y0 + (k >> 1));
    }
}

/// Draws a sprite centered on (x, y) in front of whatever is there.
pub(super) fn sprite(br: &mut Braille, rows: &[&str], x: f32, y: f32, c: Color) {
    let w = rows.first().map_or(0, |r| r.len()) as i32;
    let (x0, y0) = (
        (x - w as f32 / 2.0).round() as i32,
        (y - rows.len() as f32 / 2.0).round() as i32,
    );
    // Each cell it touches is emptied once (the biggest takes 9×4 of them).
    let mut done = 0u64;
    for (j, row) in rows.iter().enumerate() {
        for (i, ch) in row.bytes().enumerate() {
            if ch != b'#' {
                continue;
            }
            let (px, py) = (x0 + i as i32, y0 + j as i32);
            let bit = ((py >> 2) - (y0 >> 2)) * 10 + (px >> 1) - (x0 >> 1);
            if done & (1 << bit) == 0 {
                done |= 1 << bit;
                clear_cell(br, px, py);
            }
        }
    }
    for (j, row) in rows.iter().enumerate() {
        for (i, ch) in row.bytes().enumerate() {
            if ch == b'#' {
                br.set(x0 + i as i32, y0 + j as i32, c);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Flowers

#[derive(Clone, Copy)]
pub(super) struct Blossom {
    pub x: f32,
    pub y: f32,
    /// Radius when open, in dots.
    pub size: f32,
    /// Negative while waiting to bloom.
    pub age: f32,
    pub life: f32,
    pub rot: f32,
    pub kind: u8,
    /// Has let go of its petals.
    pub scattered: bool,
}

/// How a blossom's life went this frame.
pub(super) enum Life {
    Open,
    /// It lets go of its petals now.
    Scatter,
    /// It is over; another blooms elsewhere.
    Over,
}

impl Blossom {
    /// A new blossom somewhere on a screen `pw`×`ph` dots big, with a
    /// chance `odd` of being of the second kind.
    pub fn new(rng: &mut fastrand::Rng, pw: f32, ph: f32, odd: f32) -> Blossom {
        let big = (ph * 0.075).clamp(3.0, 14.0);
        let size = big * (0.75 + rng.f32() * 0.45);
        let m = size + 2.0;
        Blossom {
            x: m + rng.f32() * (pw - 2.0 * m).max(0.0),
            y: m + rng.f32() * (ph - 2.0 * m).max(0.0),
            size,
            age: 0.0,
            life: 12.0 + rng.f32() * 10.0,
            rot: rng.f32() * TAU,
            kind: (rng.f32() < odd) as u8,
            scattered: false,
        }
    }

    /// Ages it by `dt`, turns it and lets it drift with the current, much
    /// slower than the petals.
    pub fn grow(&mut self, dt: f32, field: &Field, v: f32, (pw, ph): (f32, f32)) -> Life {
        self.age += dt;
        if self.age >= self.life {
            return Life::Over;
        }
        self.rot += dt * 0.25;
        if self.age > 0.0 {
            let (fx, fy) = field.sample(self.x, self.y);
            let m = self.size + 1.0;
            self.x = (self.x + fx * v * 0.12 * dt).clamp(m, (pw - m).max(m));
            self.y = (self.y + fy * v * 0.12 * dt).clamp(m, (ph - m).max(m));
        }
        // Near its end it lets go of its petals all at once.
        if !self.scattered && self.age > self.life - 1.6 {
            self.scattered = true;
            return Life::Scatter;
        }
        Life::Open
    }
}

/// A blossom cut into braille cells (column, row, dots lit, tone), kept
/// until it has moved, opened or turned enough to look different.
#[derive(Default)]
pub(super) struct Cut {
    pub key: Option<(i32, i32, u32, i32)>,
    pub cells: Vec<(i32, i32, u8, u8)>,
}

impl Cut {
    /// A cut with room for the cells of the biggest blossom, so cutting
    /// never allocates.
    pub fn roomy() -> Cut {
        Cut {
            key: None,
            cells: Vec::with_capacity(224),
        }
    }

    /// Cuts a flower of radius `r` dots centered on `(cx, cy)` into cells of
    /// a screen `(pw, ph)` dots big. `lit` tells for a dot whether it is
    /// outside the flower (`None`), part of it but dark (`Some(false)`) or
    /// lit. A cell takes its tone (0 at the heart, 1, 2 at the rim) from
    /// where its middle lies, a fraction of the radius below `tones.0`,
    /// `tones.1` or beyond, so cells stay whole. Cells whose middle is within
    /// `halo` dots of the rim are cut too, empty, to clear the trails
    /// around the flower.
    pub fn fill(
        &mut self,
        r: f32,
        (cx, cy): (i32, i32),
        (pw, ph): (i32, i32),
        (tones, halo): ((f32, f32), f32),
        lit: impl Fn(i32, i32) -> Option<bool>,
    ) {
        self.cells.clear();
        let reach = r.ceil() as i32 + 1;
        let (x0, x1) = ((cx - reach).max(0), (cx + reach).min(pw - 1));
        let (y0, y1) = ((cy - reach).max(0), (cy + reach).min(ph - 1));
        let far = (r + 2.2) * (r + 2.2);
        for gy in (y0 >> 2)..=(y1 >> 2) {
            for gx in (x0 >> 1)..=(x1 >> 1) {
                // Cells in the corners of the box are nowhere near the rim.
                let (fx, fy) = ((gx * 2 - cx) as f32 + 0.5, (gy * 4 - cy) as f32 + 1.5);
                if fx * fx + fy * fy > far {
                    continue;
                }
                let (mut covered, mut mask) = (false, 0u8);
                for k in 0..8 {
                    if let Some(on) = lit(gx * 2 + (k & 1), gy * 4 + (k >> 1)) {
                        covered = true;
                        mask |= (on as u8) << k;
                    }
                }
                // Colored by where the cell's middle is, so cells stay whole.
                let (mx, my) = (gx * 2 + 1 - cx, gy * 4 + 2 - cy);
                let d = ((mx * mx + my * my) as f32).sqrt() / r;
                if !covered && (halo <= 0.0 || d * r > r + halo) {
                    continue;
                }
                let tone = if d < tones.0 {
                    0
                } else if d < tones.1 {
                    1
                } else {
                    2
                };
                self.cells.push((gx, gy, mask, tone));
            }
        }
    }
}

/// How a flower is cut: its radius, center, turn and the screen's size.
pub(super) type Shape = fn(&mut Cut, f32, (i32, i32), f32, (i32, i32));

/// Draws an opening, turning flower in front of the trails, cut with `shape`
/// again every `1 / steps` radians it turns.
pub(super) fn draw_flower(
    b: &Blossom,
    cut: &mut Cut,
    cols: &[Color; 3],
    br: &mut Braille,
    (shape, steps): (Shape, f32),
) {
    if b.age <= 0.0 {
        return;
    }
    let open = (b.age / 1.8).min(1.0);
    let close = ((b.life - b.age) / 1.6).clamp(0.0, 1.0);
    let r = b.size * (1.0 - (1.0 - open).powi(3)) * close.sqrt();
    if r < 0.8 {
        return;
    }
    let (cx, cy) = (b.x.round() as i32, b.y.round() as i32);
    // It turns slowly: cutting it again every few frames looks the same.
    let turn = (b.rot * steps) as i32;
    let key = (cx, cy, r.to_bits(), turn);
    if cut.key != Some(key) {
        cut.key = Some(key);
        shape(cut, r, (cx, cy), turn as f32 / steps, (br.pw(), br.ph()));
    }
    for &(gx, gy, mask, tone) in &cut.cells {
        clear_cell(br, gx * 2, gy * 4);
        let c = cols[tone as usize];
        for k in 0..8 {
            if mask & (1 << k) != 0 {
                br.set(gx * 2 + (k & 1), gy * 4 + (k >> 1), c);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Bits

/// A loose thing riding the flow (a petal, a piece of confetti, a sweet), or
/// thrown up and falling.
#[derive(Clone, Copy)]
pub(super) struct Bit {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    /// The angle it tumbles through (radians) and how fast.
    pub spin: f32,
    pub spin_rate: f32,
    pub age: f32,
    pub life: f32,
    /// What sort of thing it is: its shape, how it moves, what it leaves.
    pub kind: u8,
    /// Its color among the kind's.
    pub tone: u8,
    /// Falls (fireworks, fountain) instead of riding the flow.
    pub falls: bool,
}

/// Seconds a loose bit takes to fade in.
pub(super) const FADE_IN: f32 = 0.5;

impl Bit {
    /// A bit cast off from (x, y) at (vx, vy): bright from the start (only
    /// the loose ones fade in, see [`Bit::loose`]).
    #[allow(clippy::too_many_arguments)]
    pub fn cast(
        rng: &mut fastrand::Rng,
        (x, y): (f32, f32),
        (vx, vy): (f32, f32),
        kind: u8,
        tone: u8,
        falls: bool,
    ) -> Bit {
        Bit {
            x,
            y,
            vx,
            vy,
            spin: rng.f32() * TAU,
            spin_rate: (3.0 + rng.f32() * 6.0) * if rng.bool() { 1.0 } else { -1.0 },
            age: FADE_IN,
            life: if falls {
                9.0 + rng.f32() * 6.0
            } else {
                6.0 + rng.f32() * 9.0
            },
            kind,
            tone,
            falls,
        }
    }

    /// A bit somewhere on the screen, fading in.
    pub fn loose(rng: &mut fastrand::Rng, (pw, ph): (f32, f32), kind: u8, tone: u8) -> Bit {
        let at = (rng.f32() * pw, rng.f32() * ph);
        let mut b = Bit::cast(rng, at, (0.0, 0.0), kind, tone, false);
        b.age = 0.0;
        b
    }

    /// Moves it by `dt`. A falling bit drops like the fountain's water at
    /// first, then the air holds it and it sways down (`g` and `fall` are
    /// the pull and the speed it settles at); one riding the flow follows it
    /// as `phys` says, `follow` being how much of the way it catches up.
    pub fn drift(
        &mut self,
        dt: f32,
        wind: &Wind,
        phys: Option<&Physics>,
        follow: f32,
        (g, fall): (f32, f32),
    ) {
        self.age += dt;
        self.spin += self.spin_rate * dt;
        if self.falls {
            let air = (self.age * 1.4).min(1.0);
            self.vy += g * dt * (1.0 - air);
            let drag = 1.0 - (-(0.6 + 2.5 * air) * dt).exp();
            self.vy += (fall - self.vy) * drag * air;
            self.vx -= self.vx * drag;
            self.vx += (self.spin * 0.35).cos() * fall * 1.6 * air * dt;
        } else if let Some(ph) = phys {
            let (fx, fy) = wind.field.sample(self.x, self.y);
            self.vx += (fx * wind.v * ph.gain - self.vx) * follow;
            self.vy += (fy * wind.v * ph.gain - self.vy) * follow;
            if ph.flutter > 0.0 {
                self.vx += (self.spin * 0.5).cos() * ph.flutter * dt;
                self.vy += (self.spin * 0.5 + 1.3).sin() * ph.flutter * dt;
            }
        }
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    /// Is it over, or out of the screen `pw`×`ph` dots big?
    pub fn gone(&self, pw: f32, ph: f32) -> bool {
        self.age >= self.life
            || self.x < -4.0
            || self.x > pw + 4.0
            || self.y > ph + 4.0
            || (!self.falls && self.y < -4.0)
    }

    /// How much of it shows: it fades in when loose and out at the end.
    pub fn show(&self) -> f32 {
        (self.age / FADE_IN)
            .min((self.life - self.age) / 1.2)
            .clamp(0.0, 1.0)
    }
}

/// How a kind of bit moves in the flow: how much of the flow's speed it takes
/// up, how quickly (per second) and how much it flutters about in it.
#[derive(Clone, Copy)]
pub(super) struct Physics {
    pub gain: f32,
    pub follow: f32,
    pub flutter: f32,
}

/// The flow bits ride: its field, its speed and the trail buffer's clock.
pub(super) struct Wind<'a> {
    pub field: &'a Field,
    pub v: f32,
    pub now: u32,
    pub span: u32,
}

/// The bits a costume carries: as many as `target` ride the flow, and the
/// ones cast off by something (fireworks, a blossom, a jet) come and go.
#[derive(Default)]
pub(super) struct Bits {
    pub list: Vec<Bit>,
    /// How many ride the flow.
    pub target: usize,
}

impl Bits {
    /// Empties the list and sets aside room for `room` bits.
    pub fn reset(&mut self, target: usize, room: usize) {
        self.list.clear();
        self.list.reserve_exact(room);
        self.target = target;
    }

    /// Adds a bit if there is room for it.
    pub fn add(&mut self, b: Bit) {
        if self.list.len() < self.list.capacity() {
            self.list.push(b);
        }
    }

    pub fn full(&self) -> bool {
        self.list.len() >= self.list.capacity()
    }

    /// Moves every bit: the ones riding the flow follow it (as `phys` says),
    /// the ones falling drop and sway down. Each leaves a wake in the trail
    /// buffer if `wake` gives it a palette slot. A bit that is gone is
    /// replaced by `renew`, unless the flow has more than its share.
    pub fn update(
        &mut self,
        dt: f32,
        wind: &Wind,
        dots: &mut Dots,
        phys: &[Physics],
        wake: impl Fn(&Bit) -> Option<u8>,
        mut renew: impl FnMut() -> Bit,
    ) {
        let (pw, ph) = (dots.pw as f32, dots.ph as f32);
        let mut follow = [0.0f32; 8];
        for (f, p) in follow.iter_mut().zip(phys) {
            *f = 1.0 - (-p.follow * dt).exp();
        }
        let tail = wind.now.wrapping_sub((wind.span as f32 * 0.05) as u32);
        let (g, fall) = (ph * 1.1, (ph * 0.06).max(2.0));
        let mut i = 0;
        while i < self.list.len() {
            let mut p = self.list[i];
            let k = (p.kind as usize).min(phys.len().saturating_sub(1)).min(7);
            let (ox, oy) = (p.x, p.y);
            p.drift(dt, wind, phys.get(k), follow[k], (g, fall));
            if p.gone(pw, ph) {
                if p.falls || self.list.len() > self.target {
                    self.list.swap_remove(i);
                } else {
                    self.list[i] = renew();
                    i += 1;
                }
                continue;
            }
            if let Some(slot) = wake(&p) {
                dots.line(ox, oy, p.x, p.y, tail, slot);
            }
            self.list[i] = p;
            i += 1;
        }
    }
}

/// A spark of a burst going off in the stream.
#[derive(Clone, Copy)]
struct Star {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
    col: u8,
}

/// What the bursts in the stream are like.
pub(super) struct Burst<'a> {
    /// How many go off a second on a screen 100 columns wide.
    pub rate: f32,
    /// The trail palette slots each burst picks two colors from.
    pub cols: &'a [u8],
    /// The white-hot slot every burst starts with.
    pub hot: u8,
}

/// Bursts that go off now and then in the stream: a ring of sparks opens,
/// carried along by the current, each leaving a fading trail.
#[derive(Default)]
pub(super) struct Stars {
    list: Vec<Star>,
    /// Seconds until the next burst, once the first has been set.
    next: f32,
    started: bool,
}

impl Stars {
    /// Sets room aside for the bursts, or takes it back (`false`).
    pub fn reset(&mut self, on: bool) {
        *self = Stars::default();
        if on {
            self.list.reserve_exact(320);
        }
    }

    pub fn on(&self) -> bool {
        self.list.capacity() > 0
    }

    /// How many sparks are flying.
    #[cfg(test)]
    pub fn sparks(&self) -> usize {
        self.list.len()
    }

    pub fn update(
        &mut self,
        dt: f32,
        rng: &mut fastrand::Rng,
        wind: &Wind,
        dots: &mut Dots,
        burst: &Burst,
    ) {
        let (pw, ph) = (dots.pw as f32, dots.ph as f32);
        if burst.rate > 0.0 && !burst.cols.is_empty() {
            // Poisson-ish gaps, never too regular.
            let per_s = (burst.rate * (pw / 200.0).max(0.3)).max(1e-3);
            let gap = |rng: &mut fastrand::Rng| {
                let g = -(1.0 - rng.f32()).max(1e-3).ln() / per_s;
                g.clamp(0.4, (3.0 / per_s).max(0.8))
            };
            if !self.started {
                self.started = true;
                self.next = gap(rng);
            }
            self.next -= dt;
            if self.next <= 0.0 {
                self.next = gap(rng);
                self.pop(rng, (pw, ph), burst.cols);
            }
        }
        let k = (-2.2 * dt).exp();
        let mut i = 0;
        while i < self.list.len() {
            let mut s = self.list[i];
            s.age += dt;
            if s.age >= s.life {
                self.list.swap_remove(i);
                continue;
            }
            let (ox, oy) = (s.x, s.y);
            // The stream carries the burst along while it opens.
            let (fx, fy) = wind.field.sample(s.x, s.y);
            s.vx *= k;
            s.vy *= k;
            s.x += (s.vx + fx * wind.v * 0.6) * dt;
            s.y += (s.vy + fy * wind.v * 0.6) * dt;
            let f = s.age / s.life;
            let dim = (f * f * wind.span as f32 * 0.85) as u32;
            let col = if s.age < 0.12 { burst.hot } else { s.col };
            dots.line(ox, oy, s.x, s.y, wind.now.wrapping_sub(dim), col);
            self.list[i] = s;
            i += 1;
        }
    }

    /// A burst goes off somewhere on a screen `pw`×`ph` dots big.
    fn pop(&mut self, rng: &mut fastrand::Rng, (pw, ph): (f32, f32), cols: &[u8]) {
        let area = pw * ph / 24_000.0;
        let n = ((36.0 + rng.f32() * 24.0) * area.sqrt().clamp(0.5, 2.5)) as usize;
        let n = n.min(self.list.capacity() - self.list.len());
        let x = pw * (0.12 + 0.76 * rng.f32());
        let y = ph * (0.15 + 0.6 * rng.f32());
        let radius = ph.min(pw * 0.6) * (0.2 + rng.f32() * 0.12);
        let pair = [
            cols[rng.usize(0..cols.len())],
            cols[rng.usize(0..cols.len())],
        ];
        let drag = 2.2;
        for k in 0..n {
            let a = rng.f32() * TAU;
            let v = radius * drag * (0.45 + 0.55 * rng.f32().sqrt());
            self.list.push(Star {
                x,
                y,
                vx: a.cos() * v,
                vy: a.sin() * v,
                age: 0.0,
                life: 1.1 + rng.f32() * 0.6,
                col: pair[k % 2],
            });
        }
    }
}

/// The dots of a small shape, kept in a fixed array so drawing never
/// allocates.
pub(super) struct Pts {
    n: usize,
    a: [(i32, i32); 40],
}

impl Pts {
    pub fn new() -> Pts {
        Pts {
            n: 0,
            a: [(0, 0); 40],
        }
    }

    pub fn add(&mut self, x: i32, y: i32) {
        if self.n < self.a.len() {
            self.a[self.n] = (x, y);
            self.n += 1;
        }
    }

    /// The dots of a shape given as offsets from (x, y).
    pub fn shape(&mut self, x: i32, y: i32, offsets: &[(i32, i32)]) {
        for &(dx, dy) in offsets {
            self.add(x + dx, y + dy);
        }
    }

    /// A lens (a leaf, a feather, a strand of tinsel) of `len` dots long and
    /// `width` dots across at its fullest, centered on (x, y) and leaning at
    /// `angle`. `lean` slides its fullest point toward the tail (positive)
    /// or the tip.
    pub fn lens(&mut self, x: f32, y: f32, angle: f32, len: f32, width: f32, lean: f32) {
        let (s, c) = angle.sin_cos();
        let n = len.ceil().max(1.0) as i32;
        for i in 0..=n {
            let u = i as f32 / n as f32;
            // Fullest at `0.5 - lean`, pointed at both ends.
            let peak = (0.5 - lean).clamp(0.2, 0.8);
            let swell = if u < peak {
                u / peak
            } else {
                (1.0 - u) / (1.0 - peak)
            };
            let half = width * 0.5 * swell;
            let t = (u - 0.5) * len;
            let rows = half.round() as i32;
            for j in -rows..=rows {
                let across = j as f32;
                self.add(rnd(x + c * t - s * across), rnd(y + s * t + c * across));
            }
        }
    }

    /// Draws the dots in color `c`, in front of the trails if `front`: each
    /// cell they touch is emptied once.
    pub fn draw(&self, br: &mut Braille, c: Color, front: bool) {
        let pts = &self.a[..self.n];
        if front {
            let (bx, by) = pts
                .iter()
                .fold((i32::MAX, i32::MAX), |m, p| (m.0.min(p.0), m.1.min(p.1)));
            let mut done = 0u64;
            for &(px, py) in pts {
                let (cx, cy) = ((px >> 1) - (bx >> 1), (py >> 2) - (by >> 2));
                if !(0..8).contains(&cx) || !(0..8).contains(&cy) {
                    clear_cell(br, px, py);
                    continue;
                }
                let bit = 1u64 << (cy * 8 + cx);
                if done & bit == 0 {
                    done |= bit;
                    clear_cell(br, px, py);
                }
            }
        }
        for &(px, py) in pts {
            br.set(px, py, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cheap_arctangent_is_close_enough() {
        for i in 0..720 {
            let a = i as f32 / 720.0 * TAU - TAU / 2.0;
            let (y, x) = (a.sin() * 7.5, a.cos() * 7.5);
            assert!((atan2(y, x) - y.atan2(x)).abs() < 3e-4, "{a}");
        }
        assert_eq!(atan2(0.0, 0.0), 0.0);
    }
}
