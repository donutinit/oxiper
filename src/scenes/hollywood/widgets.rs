//! The graphical widgets: htop-style meters, bandwidth graph, matrix rain,
//! world map traces, radar, progress bars, satellite tracker, stock ticker,
//! access banners and key generation with randomart.
//!
//! Ideas after Dustin Kirkland's hollywood (Apache License 2.0): htop, bmon,
//! cmatrix, map, sshart, apg and friends, all simulated here.

use super::feed::{Feed, FeedKind};
use super::map::{self, Dots};
use super::{Env, Rect, Widget, babble, put, put_ch};
use crate::color::Color;
use crate::render::{BOLD, Canvas, DIM};
use fastrand::Rng;
use std::collections::VecDeque;
use std::f32::consts::{PI, TAU};
use std::fmt::Write;

/// A widget that draws nothing (placeholder while panes are moved around).
pub struct Nothing;

impl Widget for Nothing {
    fn name(&self) -> &'static str {
        ""
    }
    fn update(&mut self, _dt: f32, _env: &Env) {}
    fn draw(&mut self, _c: &mut Canvas, _r: Rect, _env: &Env) {}
}

/// Moves `cur` toward `target` exponentially.
#[inline]
fn ease(cur: &mut f32, target: f32, dt: f32, k: f32) {
    *cur += (target - *cur) * (dt * k).min(1.0);
}

/// An htop meter: `label[|||||    text]`, green to amber to red.
#[allow(clippy::too_many_arguments)]
fn meter(
    c: &mut Canvas,
    r: Rect,
    x: i32,
    y: i32,
    w: i32,
    label: &str,
    frac: f32,
    text: &str,
    env: &Env,
) {
    let pal = &env.pal;
    let mut cx = put(c, r, x, y, label, pal.cyan, BOLD);
    cx = put(c, r, cx, y, "[", pal.bright, 0);
    let inner = (x + w - cx - 1).max(0);
    let filled = (frac.clamp(0.0, 1.0) * inner as f32) as i32;
    for i in 0..inner {
        if i < filled {
            let t = i as f32 / inner.max(1) as f32;
            let col = if t < 0.55 {
                pal.green
            } else if t < 0.8 {
                pal.amber
            } else {
                pal.red
            };
            put_ch(c, r, cx + i, y, '|', col, 0);
        }
    }
    let tx = cx + inner - text.chars().count() as i32;
    put(c, r, tx.max(cx + filled), y, text, pal.dim, 0);
    put(c, r, cx + inner, y, "]", pal.bright, 0);
}

// ---------------------------------------------------------------- htop

struct Proc {
    pid: u32,
    user: &'static str,
    cpu: f32,
    mem: f32,
    cmd: String,
}

pub struct Htop {
    rng: Rng,
    cpu: [f32; 8],
    target: [f32; 8],
    mem: f32,
    swap: f32,
    procs: Vec<Proc>,
    timer: f32,
    uptime: f32,
}

impl Htop {
    pub fn new(seed: u64) -> Htop {
        let mut rng = Rng::with_seed(seed);
        let mut procs = Vec::new();
        for _ in 0..24 {
            let mut cmd = String::from(babble::pick(&mut rng, babble::DAEMONS));
            if rng.bool() {
                cmd.push_str(" --");
                cmd.push_str(babble::pick(&mut rng, babble::MODS));
                cmd.push('=');
                cmd.push_str(&rng.u32(1..9999).to_string());
            }
            procs.push(Proc {
                pid: rng.u32(1..32768),
                user: ["root", "root", "daemon", "www-data", "neo", "nobody"][rng.usize(..6)],
                cpu: rng.f32() * 30.0,
                mem: rng.f32() * 9.0,
                cmd,
            });
        }
        Htop {
            cpu: [0.3; 8],
            target: [0.3; 8],
            mem: 0.5,
            swap: 0.1,
            procs,
            timer: 0.0,
            uptime: rng.f32() * 900_000.0,
            rng,
        }
    }
}

impl Widget for Htop {
    fn name(&self) -> &'static str {
        "htop"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.uptime += dt;
        self.timer -= dt;
        if self.timer <= 0.0 {
            self.timer = 0.5 + self.rng.f32() * 0.8;
            for t in self.target.iter_mut() {
                *t = if self.rng.u8(..4) == 0 {
                    0.7 + self.rng.f32() * 0.3
                } else {
                    self.rng.f32() * 0.5
                };
            }
            for p in self.procs.iter_mut() {
                p.cpu = (p.cpu + (self.rng.f32() - 0.45) * 14.0).clamp(0.0, 99.0);
            }
            self.procs.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
            self.mem = (self.mem + (self.rng.f32() - 0.5) * 0.08).clamp(0.2, 0.95);
            self.swap = (self.swap + (self.rng.f32() - 0.5) * 0.04).clamp(0.0, 0.8);
        }
        for i in 0..8 {
            let t = self.target[i];
            ease(&mut self.cpu[i], t, dt, 5.0);
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let two = r.w >= 56;
        let cores = (((r.h - 6) / 3).clamp(1, 8)) as usize;
        let colw = if two { r.w / 2 } else { r.w };
        let mut y = 0;
        let per_col = if two { cores.div_ceil(2) } else { cores };
        for i in 0..cores {
            let (col, row) = if two {
                (i / per_col, i % per_col)
            } else {
                (0, i)
            };
            let mut t = String::new();
            let _ = write!(t, "{:.1}%", self.cpu[i] * 100.0);
            meter(
                c,
                r,
                col as i32 * colw,
                row as i32,
                colw - 1,
                &format!("{:>2}", i + 1),
                self.cpu[i],
                &t,
                env,
            );
            y = y.max(row as i32 + 1);
        }
        let mut t = String::new();
        let _ = write!(t, "{:.2}G/15.6G", self.mem * 15.6);
        meter(c, r, 0, y, colw - 1, "Mem", self.mem, &t, env);
        t.clear();
        let _ = write!(t, "{:.2}G/8.00G", self.swap * 8.0);
        if two {
            meter(c, r, colw, y, colw - 1, "Swp", self.swap, &t, env);
        } else {
            y += 1;
            meter(c, r, 0, y, colw - 1, "Swp", self.swap, &t, env);
        }
        y += 1;
        t.clear();
        let up = self.uptime as u32;
        let _ = write!(
            t,
            "Tasks: {}, {} thr; Load: {:.2}  Up {}d {:02}:{:02}",
            200 + (up % 37),
            900 + (up % 211),
            self.cpu.iter().sum::<f32>() / 4.0,
            up / 86400,
            up / 3600 % 24,
            up / 60 % 60
        );
        put(c, r, 0, y, &t, pal.main, 0);
        y += 1;
        if r.h - y >= 3 {
            put(
                c,
                r,
                0,
                y,
                "  PID USER      CPU%  MEM%  COMMAND",
                pal.cyan,
                BOLD,
            );
            y += 1;
            for p in &self.procs {
                if y >= r.h {
                    break;
                }
                let tone = if p.cpu > 60.0 {
                    pal.red
                } else if p.cpu > 25.0 {
                    pal.amber
                } else {
                    pal.main
                };
                t.clear();
                let _ = write!(
                    t,
                    "{:>5} {:<8} {:>5.1} {:>5.1}  {}",
                    p.pid, p.user, p.cpu, p.mem, p.cmd
                );
                put(c, r, 0, y, &t, tone, 0);
                y += 1;
            }
        }
    }
}

// ---------------------------------------------------------------- bandwidth

pub struct Bandwidth {
    rng: Rng,
    hist: VecDeque<(f32, f32)>,
    rx: f32,
    tx: f32,
    rx_t: f32,
    tx_t: f32,
    timer: f32,
    acc: f32,
    dots: Dots,
    peak: f32,
}

impl Bandwidth {
    pub fn new(seed: u64) -> Bandwidth {
        Bandwidth {
            rng: Rng::with_seed(seed),
            hist: VecDeque::new(),
            rx: 0.3,
            tx: 0.1,
            rx_t: 0.3,
            tx_t: 0.1,
            timer: 0.0,
            acc: 0.0,
            dots: Dots::new(1, 1),
            peak: 0.0,
        }
    }
}

impl Widget for Bandwidth {
    fn name(&self) -> &'static str {
        "bmon -p eth0"
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.dots = Dots::new(w.max(1), h.saturating_sub(2).max(1));
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.timer -= dt;
        if self.timer <= 0.0 {
            self.timer = 0.3 + self.rng.f32() * 1.2;
            self.rx_t = if self.rng.u8(..5) == 0 {
                0.7 + self.rng.f32() * 0.3
            } else {
                0.1 + self.rng.f32() * 0.5
            };
            self.tx_t = 0.05 + self.rng.f32() * 0.3;
        }
        ease(&mut self.rx, self.rx_t, dt, 4.0);
        ease(&mut self.tx, self.tx_t, dt, 4.0);
        self.acc += dt;
        let mut n = 0;
        while self.acc >= 0.1 && n < 20 {
            self.acc -= 0.1;
            n += 1;
            let j = (self.rng.f32() - 0.5) * 0.12;
            self.hist.push_back((
                (self.rx + j).clamp(0.02, 1.0),
                (self.tx + j * 0.5).clamp(0.01, 1.0),
            ));
            self.peak = self.peak.max(self.rx);
        }
        self.acc = self.acc.min(0.1);
        while self.hist.len() > 600 {
            self.hist.pop_front();
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let (last_rx, last_tx) = self.hist.back().copied().unwrap_or((0.0, 0.0));
        let mut t = String::new();
        let _ = write!(
            t,
            "eth0  RX {:7.1} Mbit/s  TX {:7.1} Mbit/s",
            last_rx * 940.0,
            last_tx * 940.0
        );
        put(c, r, 0, 0, &t, pal.bright, BOLD);
        let (pw, ph) = (self.dots.pw(), self.dots.ph());
        self.dots.clear();
        let n = self.hist.len() as i32;
        for x in 0..pw {
            let i = n - pw + x;
            if i < 0 {
                continue;
            }
            let (rx, tx) = self.hist[i as usize];
            let top = ph - (rx * ph as f32) as i32;
            for y in top.max(0)..ph {
                let k = 1.0 - (y as f32 / ph as f32);
                self.dots.set(x, y, pal.ramp.at(0.3 + 0.7 * k));
            }
            self.dots.set(x, ph - (tx * ph as f32) as i32, pal.cyan);
        }
        let body = Rect {
            x: r.x,
            y: r.y + 1,
            w: r.w,
            h: r.h - 2,
        };
        self.dots.blit(c, body);
        t.clear();
        let _ = write!(
            t,
            "peak {:.0} Mbit/s   avg {:.0} Mbit/s",
            self.peak * 940.0,
            self.hist.iter().map(|h| h.0).sum::<f32>() / n.max(1) as f32 * 940.0
        );
        put(c, r, 0, r.h - 1, &t, pal.dim, 0);
    }
}

// ---------------------------------------------------------------- rain

#[derive(Clone, Copy)]
struct Drop {
    y: f32,
    speed: f32,
    len: i32,
}

pub struct Rain {
    rng: Rng,
    drops: Vec<Drop>,
    h: i32,
    tick: u32,
    acc: f32,
}

#[inline]
fn hash(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9e37_79b1) ^ b.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^ (h >> 12)
}

impl Rain {
    pub fn new(seed: u64) -> Rain {
        Rain {
            rng: Rng::with_seed(seed),
            drops: Vec::new(),
            h: 0,
            tick: 0,
            acc: 0.0,
        }
    }

    fn respawn(rng: &mut Rng, h: i32, first: bool) -> Drop {
        let len = rng.i32(5..(h.max(8) * 2 / 3).max(7));
        Drop {
            y: if first {
                rng.f32() * (h + len) as f32 - len as f32
            } else {
                -(rng.f32() * h as f32 * 0.7) - 1.0
            },
            speed: 6.0 + rng.f32() * 16.0,
            len,
        }
    }
}

impl Widget for Rain {
    fn name(&self) -> &'static str {
        "cmatrix -ab"
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.h = h as i32;
        self.drops = (0..w)
            .map(|_| Self::respawn(&mut self.rng, h as i32, true))
            .collect();
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.acc += dt;
        if self.acc >= 0.12 {
            self.acc = 0.0;
            self.tick = self.tick.wrapping_add(1);
        }
        for d in self.drops.iter_mut() {
            d.y += d.speed * dt;
            if d.y - d.len as f32 > self.h as f32 {
                *d = Self::respawn(&mut self.rng, self.h, false);
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        for (x, d) in self.drops.iter().enumerate() {
            let head = d.y.floor() as i32;
            for k in 0..d.len {
                let row = head - k;
                if row < 0 || row >= r.h {
                    continue;
                }
                let h = hash(x as u32 * 131 + row as u32, 7);
                // A few glyphs flicker with the tick, most stay put.
                let h2 = if h & 7 == 0 { hash(h, self.tick) } else { h };
                let ch = if h2 % 5 == 0 {
                    char::from_digit(h2 / 5 % 10, 10).unwrap_or('0')
                } else {
                    char::from_u32(0xff66 + h2 / 5 % 56).unwrap_or('?')
                };
                let (col, at) = if k == 0 {
                    (pal.bright, BOLD)
                } else {
                    (pal.ramp.at(1.0 - k as f32 / d.len as f32 * 0.85), 0)
                };
                put_ch(c, r, x as i32, row, ch, col, at);
            }
        }
    }
}

// ---------------------------------------------------------------- world map

struct Arc {
    from: usize,
    to: usize,
    t: f32,
    glow: f32,
}

pub struct WorldMap {
    rng: Rng,
    land: Dots,
    fx: Dots,
    arcs: Vec<Arc>,
    spawn: f32,
    painted: bool,
}

impl WorldMap {
    pub fn new(seed: u64) -> WorldMap {
        WorldMap {
            rng: Rng::with_seed(seed),
            land: Dots::new(1, 1),
            fx: Dots::new(1, 1),
            arcs: Vec::new(),
            spawn: 0.0,
            painted: false,
        }
    }
}

impl Widget for WorldMap {
    fn name(&self) -> &'static str {
        "tracemap --live"
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.land = Dots::new(w.max(1), h.saturating_sub(1).max(1));
        self.fx = Dots::new(w.max(1), h.saturating_sub(1).max(1));
        self.painted = false;
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.spawn -= dt;
        if self.spawn <= 0.0 && self.arcs.len() < 5 {
            self.spawn = 0.8 + self.rng.f32() * 2.0;
            let from = self.rng.usize(..map::CITIES.len());
            let to = (from + 1 + self.rng.usize(..map::CITIES.len() - 1)) % map::CITIES.len();
            self.arcs.push(Arc {
                from,
                to,
                t: 0.0,
                glow: 0.0,
            });
        }
        for a in self.arcs.iter_mut() {
            if a.t < 1.0 {
                a.t = (a.t + dt * 0.5).min(1.0);
            } else {
                a.glow += dt * 0.7;
            }
        }
        self.arcs.retain(|a| a.glow < 1.0);
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        if !self.painted {
            self.land = Dots::new((r.w.max(1)) as u16, (r.h - 1).max(1) as u16);
            map::paint_land(&mut self.land, pal.fade(pal.main, 0.55));
            self.painted = true;
        }
        let body = Rect {
            x: r.x,
            y: r.y + 1,
            w: r.w,
            h: r.h - 1,
        };
        self.land.blit(c, body);
        let (pw, ph) = (self.fx.pw(), self.fx.ph());
        self.fx.clear();
        let mut status = String::new();
        for a in &self.arcs {
            let (_, la, lo) = map::CITIES[a.from];
            let (_, lb, lob) = map::CITIES[a.to];
            let (x0, y0) = map::project(la, lo, pw, ph);
            let (x1, y1) = map::project(lb, lob, pw, ph);
            let lift = ((x1 - x0).abs() + (y1 - y0).abs()) * 0.25;
            let steps = 40;
            let col = if a.glow > 0.0 { pal.red } else { pal.amber };
            let upto = (steps as f32 * a.t) as i32;
            let (mut px, mut py) = (x0, y0);
            for i in 1..=upto {
                let t = i as f32 / steps as f32;
                let x = x0 + (x1 - x0) * t;
                let y = y0 + (y1 - y0) * t - (t * PI).sin() * lift;
                self.fx.line(px, py, x, y, col);
                px = x;
                py = y;
            }
            status = format!("TRACE {} -> {}", map::CITIES[a.from].0, map::CITIES[a.to].0);
        }
        self.fx.blit(c, body);
        for a in &self.arcs {
            let (_, lb, lob) = map::CITIES[a.to];
            let (x1, y1) = map::project(lb, lob, pw, ph);
            let ch = if a.t >= 1.0 && (a.glow * 6.0) as i32 % 2 == 0 {
                '\u{25c9}'
            } else {
                '\u{25cf}'
            };
            let col = if a.t >= 1.0 { pal.red } else { pal.amber };
            put_ch(c, body, (x1 / 2.0) as i32, (y1 / 4.0) as i32, ch, col, BOLD);
        }
        put(c, r, 0, 0, &status, pal.bright, BOLD);
    }
}

// ---------------------------------------------------------------- radar

struct Blip {
    ang: f32,
    dist: f32,
    glow: f32,
    id: u16,
}

pub struct Radar {
    rng: Rng,
    dots: Dots,
    sweep: f32,
    blips: Vec<Blip>,
}

impl Radar {
    pub fn new(seed: u64) -> Radar {
        let mut rng = Rng::with_seed(seed);
        let blips = (0..9)
            .map(|_| Blip {
                ang: rng.f32() * TAU,
                dist: 0.15 + rng.f32() * 0.8,
                glow: 0.0,
                id: rng.u16(100..999),
            })
            .collect();
        Radar {
            rng,
            dots: Dots::new(1, 1),
            sweep: 0.0,
            blips,
        }
    }
}

impl Widget for Radar {
    fn name(&self) -> &'static str {
        "radar --sweep"
    }

    fn resize(&mut self, w: u16, h: u16) {
        let rw = if w >= 50 { w - 24 } else { w };
        self.dots = Dots::new(rw.max(1), h.max(1));
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        let prev = self.sweep;
        self.sweep = (self.sweep + dt * 1.6) % TAU;
        for b in self.blips.iter_mut() {
            b.glow = (b.glow - dt * 0.35).max(0.0);
            let crossed = if self.sweep >= prev {
                b.ang >= prev && b.ang < self.sweep
            } else {
                b.ang >= prev || b.ang < self.sweep
            };
            if crossed {
                b.glow = 1.0;
            }
            // Contacts drift slowly.
            b.ang = (b.ang + (self.rng.f32() - 0.5) * 0.01).rem_euclid(TAU);
            b.dist = (b.dist + (self.rng.f32() - 0.5) * 0.004).clamp(0.1, 0.95);
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let (pw, ph) = (self.dots.pw(), self.dots.ph());
        let (cx, cy) = (pw as f32 / 2.0, ph as f32 / 2.0);
        let rad = (cx.min(cy) - 1.0).max(2.0);
        self.dots.clear();
        let ring = pal.fade(pal.main, 0.6);
        for k in 1..=3 {
            let rr = rad * k as f32 / 3.0;
            let n = (rr * 4.0) as i32 + 8;
            for i in 0..n {
                let a = i as f32 / n as f32 * TAU;
                self.dots
                    .set((cx + a.cos() * rr) as i32, (cy + a.sin() * rr) as i32, ring);
            }
        }
        self.dots.line(cx - rad, cy, cx + rad, cy, ring);
        self.dots.line(cx, cy - rad, cx, cy + rad, ring);
        for i in (0..8).rev() {
            let a = self.sweep - i as f32 * 0.07;
            let col = if i == 0 {
                pal.bright
            } else {
                pal.fade(pal.green, 0.15 + i as f32 * 0.1)
            };
            self.dots
                .line(cx, cy, cx + a.cos() * rad, cy + a.sin() * rad, col);
        }
        let rw = self.dots.pw() / 2;
        let area = Rect {
            x: r.x,
            y: r.y,
            w: rw.min(r.w),
            h: r.h,
        };
        self.dots.blit(c, area);
        for b in &self.blips {
            if b.glow > 0.02 {
                let x = cx + b.ang.cos() * b.dist * rad;
                let y = cy + b.ang.sin() * b.dist * rad;
                let col = pal.fade(if b.glow > 0.6 { pal.red } else { pal.amber }, 1.0 - b.glow);
                put_ch(
                    c,
                    area,
                    (x / 2.0) as i32,
                    (y / 4.0) as i32,
                    '\u{25c6}',
                    col,
                    BOLD,
                );
            }
        }
        if r.w >= 50 {
            let x0 = rw + 1;
            put(c, r, x0, 0, "CONTACTS", pal.cyan, BOLD);
            for (i, b) in self.blips.iter().enumerate() {
                let y = 2 + i as i32;
                let col = if b.glow > 0.5 { pal.red } else { pal.main };
                let t = format!(
                    "TGT-{} BRG {:03} RNG {:4.1}",
                    b.id,
                    (b.ang.to_degrees() as i32 + 90).rem_euclid(360),
                    b.dist * 40.0
                );
                put(c, r, x0, y, &t, col, 0);
            }
        }
    }
}

// ---------------------------------------------------------------- progress bars

struct Bar {
    label: String,
    pct: f32,
    rate: f32,
    done: f32,
}

pub struct Progress {
    rng: Rng,
    bars: Vec<Bar>,
}

impl Progress {
    pub fn new(seed: u64) -> Progress {
        Progress {
            rng: Rng::with_seed(seed),
            bars: Vec::new(),
        }
    }

    fn fresh(rng: &mut Rng) -> Bar {
        Bar {
            label: String::new(),
            pct: rng.f32() * 30.0,
            rate: 2.0 + rng.f32() * 12.0,
            done: 0.0,
        }
    }
}

impl Widget for Progress {
    fn name(&self) -> &'static str {
        "pv --force"
    }

    fn resize(&mut self, _w: u16, h: u16) {
        let n = (h as usize / 2).clamp(1, 9);
        self.bars.clear();
        for _ in 0..n {
            self.bars.push(Self::fresh(&mut self.rng));
        }
    }

    fn update(&mut self, dt: f32, env: &Env) {
        for b in self.bars.iter_mut() {
            if b.label.is_empty() {
                babble::progress_label(&mut self.rng, env.level.max(1), &mut b.label);
            }
            if b.done > 0.0 {
                b.done -= dt;
                if b.done <= 0.0 {
                    *b = Self::fresh(&mut self.rng);
                }
            } else {
                b.pct += b.rate * dt * (0.6 + self.rng.f32() * 0.8);
                if b.pct >= 100.0 {
                    b.pct = 100.0;
                    b.done = 1.4;
                }
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let mut t = String::new();
        for (i, b) in self.bars.iter().enumerate() {
            let y = i as i32 * 2;
            if y + 1 >= r.h {
                break;
            }
            let done = b.done > 0.0;
            if done {
                put(c, r, 0, y, "COMPLETE", pal.green, BOLD);
                put(c, r, 9, y, &b.label, pal.dim, 0);
            } else {
                put(c, r, 0, y, &b.label, pal.bright, BOLD);
            }
            let bw = (r.w - 20).max(4);
            let filled = (b.pct / 100.0 * bw as f32) as i32;
            put(c, r, 0, y + 1, "[", pal.dim, 0);
            for x in 0..bw {
                let (ch, col) = if x < filled {
                    (
                        '\u{2588}',
                        if done {
                            pal.green
                        } else {
                            pal.ramp.at(0.45 + 0.5 * x as f32 / bw as f32)
                        },
                    )
                } else {
                    ('\u{2591}', pal.dim)
                };
                put_ch(c, r, 1 + x, y + 1, ch, col, 0);
            }
            t.clear();
            let eta = ((100.0 - b.pct) / b.rate.max(0.1)) as u32;
            let _ = write!(t, "] {:>3.0}%  ETA {:02}:{:02}", b.pct, eta / 60, eta % 60);
            put(
                c,
                r,
                1 + bw,
                y + 1,
                &t,
                if done { pal.green } else { pal.amber },
                0,
            );
        }
    }
}

// ---------------------------------------------------------------- satellites

struct Sat {
    name: &'static str,
    lon: f32,
    speed: f32,
    amp: f32,
    phase: f32,
    alt: u16,
}

const SAT_NAMES: &[&str] = &[
    "ISS (ZARYA)",
    "STARLINK-4417",
    "NOAA-19",
    "GPS BIIR-2",
    "COSMOS 2251",
    "SKYNET-5C",
    "TIANGONG",
    "HUBBLE",
    "KH-11 KENNEN",
    "USA-224",
    "IRIDIUM 33",
    "MOLNIYA 1-91",
];

pub struct Satellites {
    rng: Rng,
    land: Dots,
    fx: Dots,
    sats: Vec<Sat>,
    painted: bool,
    list: i32,
}

impl Satellites {
    pub fn new(seed: u64) -> Satellites {
        let mut rng = Rng::with_seed(seed);
        let mut names: Vec<&str> = SAT_NAMES.to_vec();
        rng.shuffle(&mut names);
        let sats = names
            .iter()
            .take(5)
            .map(|n| Sat {
                name: n,
                lon: rng.f32() * 360.0 - 180.0,
                speed: 6.0 + rng.f32() * 14.0,
                amp: 25.0 + rng.f32() * 55.0,
                phase: rng.f32() * 360.0,
                alt: rng.u16(350..36000),
            })
            .collect();
        Satellites {
            rng,
            land: Dots::new(1, 1),
            fx: Dots::new(1, 1),
            sats,
            painted: false,
            list: 0,
        }
    }

    fn lat(s: &Sat, lon: f32) -> f32 {
        s.amp * ((lon + s.phase).to_radians()).sin()
    }
}

impl Widget for Satellites {
    fn name(&self) -> &'static str {
        "satellite-tracker"
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.list = if h >= 16 {
            5
        } else if h >= 11 {
            3
        } else {
            0
        };
        let rows = (h as i32 - self.list - 1).max(1) as u16;
        self.land = Dots::new(w.max(1), rows);
        self.fx = Dots::new(w.max(1), rows);
        self.painted = false;
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        for s in self.sats.iter_mut() {
            s.lon += s.speed * dt;
            if s.lon > 180.0 {
                s.lon -= 360.0;
                s.alt = (s.alt as i32 + self.rng.i32(-3..4)).clamp(300, 36000) as u16;
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        if !self.painted {
            map::paint_land(&mut self.land, pal.fade(pal.main, 0.65));
            self.painted = true;
        }
        let body = Rect {
            x: r.x,
            y: r.y + 1,
            w: r.w,
            h: r.h - 1 - self.list,
        };
        self.land.blit(c, body);
        let (pw, ph) = (self.fx.pw(), self.fx.ph());
        self.fx.clear();
        for (i, s) in self.sats.iter().enumerate() {
            let col = if i % 2 == 0 { pal.cyan } else { pal.amber };
            let track = pal.fade(col, 0.45);
            for x in (0..pw).step_by(2) {
                let lon = x as f32 / (pw - 1).max(1) as f32 * 360.0 - 180.0;
                let (_, y) = map::project(Self::lat(s, lon), lon, pw, ph);
                self.fx.set(x, y as i32, track);
            }
        }
        self.fx.blit(c, body);
        for (i, s) in self.sats.iter().enumerate() {
            let (x, y) = map::project(Self::lat(s, s.lon), s.lon, pw, ph);
            let col = if i % 2 == 0 { pal.cyan } else { pal.amber };
            put_ch(
                c,
                body,
                (x / 2.0) as i32,
                (y / 4.0) as i32,
                '\u{25c9}',
                col,
                BOLD,
            );
        }
        put(
            c,
            r,
            0,
            0,
            "ORBITAL TRACKING  UPLINK LOCKED",
            pal.bright,
            BOLD,
        );
        let mut t = String::new();
        for (i, s) in self.sats.iter().take(self.list as usize).enumerate() {
            t.clear();
            let _ = write!(
                t,
                "{:<14} ALT {:>5} km  LAT {:+6.2}  LON {:+7.2}",
                s.name,
                s.alt,
                Self::lat(s, s.lon),
                s.lon
            );
            put(
                c,
                r,
                0,
                r.h - self.list + i as i32,
                &t,
                if i % 2 == 0 { pal.cyan } else { pal.amber },
                0,
            );
        }
    }
}

// ---------------------------------------------------------------- ticker

const SYMBOLS: &[&str] = &[
    "FLUX", "WARP", "DLTM", "TACH", "QBIT", "NEUR", "ROOT", "HAXX", "MAIN", "CYBR", "BYTE", "0DAY",
    "BTC", "ETH", "DOGE", "SOL", "NULL", "SUDO", "CRON", "KRNL", "PING", "ZERO", "ECHO", "HEXA",
];

struct Quote {
    sym: &'static str,
    price: f32,
    open: f32,
    hist: VecDeque<f32>,
}

pub struct Ticker {
    rng: Rng,
    quotes: Vec<Quote>,
    timer: f32,
}

impl Ticker {
    pub fn new(seed: u64) -> Ticker {
        Ticker {
            rng: Rng::with_seed(seed),
            quotes: Vec::new(),
            timer: 0.0,
        }
    }
}

impl Widget for Ticker {
    fn name(&self) -> &'static str {
        "ticker --crypto"
    }

    fn resize(&mut self, _w: u16, h: u16) {
        let n = (h as usize).saturating_sub(2).clamp(1, SYMBOLS.len());
        let mut syms: Vec<&'static str> = SYMBOLS.to_vec();
        self.rng.shuffle(&mut syms);
        self.quotes = syms
            .into_iter()
            .take(n)
            .map(|sym| {
                let p = 5.0 + self.rng.f32().powi(3) * 60000.0;
                Quote {
                    sym,
                    price: p,
                    open: p,
                    hist: VecDeque::from(vec![p; 12]),
                }
            })
            .collect();
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.timer -= dt;
        if self.timer > 0.0 {
            return;
        }
        self.timer = 0.4 + self.rng.f32() * 0.4;
        for q in self.quotes.iter_mut() {
            q.price *= 1.0 + (self.rng.f32() - 0.5) * 0.03;
            q.hist.push_back(q.price);
            if q.hist.len() > 12 {
                q.hist.pop_front();
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        put(
            c,
            r,
            0,
            0,
            "SYMBOL      LAST     CHG%  TREND",
            pal.cyan,
            BOLD,
        );
        const BLK: [char; 8] = [
            '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}',
            '\u{2588}',
        ];
        let mut t = String::new();
        for (i, q) in self.quotes.iter().enumerate() {
            let y = 2 + i as i32;
            if y >= r.h {
                break;
            }
            let chg = (q.price / q.open - 1.0) * 100.0;
            let col = if chg >= 0.0 { pal.green } else { pal.red };
            t.clear();
            let _ = write!(
                t,
                "{:<6} {:>10.2} {}{:>6.2}  ",
                q.sym,
                q.price,
                if chg >= 0.0 { '\u{25b2}' } else { '\u{25bc}' },
                chg.abs()
            );
            let x = put(c, r, 0, y, &t, col, 0);
            let (lo, hi) = q
                .hist
                .iter()
                .fold((f32::MAX, f32::MIN), |a, &v| (a.0.min(v), a.1.max(v)));
            for (k, &v) in q.hist.iter().enumerate() {
                let n = if hi > lo {
                    ((v - lo) / (hi - lo) * 7.0) as usize
                } else {
                    3
                };
                put_ch(c, r, x + k as i32, y, BLK[n.min(7)], col, 0);
            }
        }
    }
}

// ---------------------------------------------------------------- access banners

/// A 3x5 pixel font for the banner letters.
fn glyph(ch: char) -> [u8; 5] {
    match ch {
        'A' => [0b010, 0b101, 0b111, 0b101, 0b101],
        'C' => [0b011, 0b100, 0b100, 0b100, 0b011],
        'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
        'E' => [0b111, 0b100, 0b110, 0b100, 0b111],
        'G' => [0b011, 0b100, 0b101, 0b101, 0b011],
        'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
        'N' => [0b101, 0b111, 0b111, 0b101, 0b101],
        'R' => [0b110, 0b101, 0b110, 0b101, 0b101],
        'S' => [0b011, 0b100, 0b010, 0b001, 0b110],
        'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
        _ => [0; 5],
    }
}

pub struct Access {
    feed: Feed,
    rng: Rng,
    /// Some(granted, seconds left) while a banner is up.
    banner: Option<(bool, f32)>,
    wait: f32,
}

impl Access {
    pub fn new(seed: u64, level: u8) -> Access {
        let mut rng = Rng::with_seed(seed ^ 0xacce55);
        Access {
            feed: Feed::new(FeedKind::Auth, seed, level),
            wait: 2.0 + rng.f32() * 4.0,
            rng,
            banner: None,
        }
    }

    fn word(c: &mut Canvas, r: Rect, y: i32, word: &str, sx: i32, col: Color) {
        let gw = 4 * sx;
        let total = word.len() as i32 * gw - sx;
        let mut x = (r.w - total) / 2;
        for ch in word.chars() {
            let g = glyph(ch);
            for (row, bits) in g.iter().enumerate() {
                for px in 0..3 {
                    if bits >> (2 - px) & 1 == 1 {
                        for k in 0..sx {
                            put_ch(c, r, x + px * sx + k, y + row as i32, '\u{2588}', col, 0);
                        }
                    }
                }
            }
            x += gw;
        }
    }
}

impl Widget for Access {
    fn name(&self) -> &'static str {
        "authd"
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.feed.resize(w, h);
    }

    fn update(&mut self, dt: f32, env: &Env) {
        self.feed.update(dt, env);
        match &mut self.banner {
            Some((_, left)) => {
                *left -= dt;
                if *left <= 0.0 {
                    self.banner = None;
                    self.wait = 3.0 + self.rng.f32() * 7.0;
                }
            }
            None => {
                self.wait -= dt;
                if self.wait <= 0.0 {
                    self.banner = Some((self.rng.u8(..10) < 4, 1.8));
                }
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        self.feed.draw(c, r, env);
        let Some((granted, left)) = self.banner else {
            return;
        };
        let pal = &env.pal;
        let col = if granted { pal.green } else { pal.red };
        let (w1, w2) = if granted {
            ("ACCESS", "GRANTED")
        } else {
            ("ACCESS", "DENIED")
        };
        // Blink in the last moments.
        let on = left > 0.5 || (left * 10.0) as i32 % 2 == 0;
        if !on {
            return;
        }
        let sx = if r.w >= 58 { 2 } else { 1 };
        let bw = (w2.len() as i32 * 4 * sx + 2).min(r.w);
        let (bh, big) = if r.h >= 13 && r.w >= w2.len() as i32 * 4 + 2 {
            (13, true)
        } else {
            (3, false)
        };
        if r.h < bh || r.w < 10 {
            return;
        }
        let bw = if big {
            bw
        } else {
            (w2.len() as i32 + w1.len() as i32 + 5).min(r.w)
        };
        let bx = (r.w - bw) / 2;
        let by = (r.h - bh) / 2;
        for y in by..by + bh {
            for x in bx..bx + bw {
                put_ch(c, r, x, y, ' ', col, 0);
                c.set_bg(r.x + x, r.y + y, pal.ink);
            }
        }
        // A frame around the banner.
        for x in bx..bx + bw {
            put_ch(c, r, x, by, '\u{2550}', col, BOLD);
            put_ch(c, r, x, by + bh - 1, '\u{2550}', col, BOLD);
        }
        for y in by..by + bh {
            put_ch(c, r, bx, y, '\u{2551}', col, BOLD);
            put_ch(c, r, bx + bw - 1, y, '\u{2551}', col, BOLD);
        }
        if big {
            let sub = Rect {
                x: r.x + bx,
                y: r.y,
                w: bw,
                h: r.h,
            };
            Self::word(c, sub, by + 1, w1, sx, col);
            Self::word(c, sub, by + 7, w2, sx, col);
        } else {
            let t = format!("{w1} {w2}");
            put(c, r, bx + (bw - t.len() as i32) / 2, by + 1, &t, col, BOLD);
        }
    }
}

// ---------------------------------------------------------------- keygen

const ART: &[u8; 17] = b" .o+=*BOX@%&#/^SE";

pub struct Keygen {
    feed: Feed,
    rng: Rng,
    field: [[u8; 17]; 9],
    pos: (usize, usize),
    bytes: [u8; 16],
    step: usize,
    hold: f32,
    acc: f32,
    start_flash: f32,
}

impl Keygen {
    pub fn new(seed: u64, level: u8) -> Keygen {
        let mut k = Keygen {
            feed: Feed::new(FeedKind::Passwords, seed, level),
            rng: Rng::with_seed(seed ^ 0x6b657967),
            field: [[0; 17]; 9],
            pos: (8, 4),
            bytes: [0; 16],
            step: 0,
            hold: 0.0,
            acc: 0.0,
            start_flash: 0.0,
        };
        k.restart();
        k
    }

    fn restart(&mut self) {
        for b in self.bytes.iter_mut() {
            *b = self.rng.u8(..);
        }
        self.field = [[0; 17]; 9];
        self.pos = (8, 4);
        self.step = 0;
    }

    /// One bishop move: the drunken-bishop walk used by OpenSSH's randomart.
    fn walk(&mut self) {
        let byte = self.bytes[self.step / 4];
        let m = (byte >> ((self.step % 4) * 2)) & 3;
        let (mut x, mut y) = self.pos;
        x = if m & 1 == 1 {
            (x + 1).min(16)
        } else {
            x.saturating_sub(1)
        };
        y = if m & 2 == 2 {
            (y + 1).min(8)
        } else {
            y.saturating_sub(1)
        };
        self.pos = (x, y);
        self.field[y][x] = self.field[y][x].saturating_add(1);
        self.step += 1;
    }
}

impl Widget for Keygen {
    fn name(&self) -> &'static str {
        "ssh-keygen -lv"
    }

    fn resize(&mut self, w: u16, h: u16) {
        let art = h >= 16 && w >= 22;
        let rest = if art { h - 11 } else { h };
        self.feed.resize(w, rest);
    }

    fn update(&mut self, dt: f32, env: &Env) {
        self.feed.update(dt, env);
        self.start_flash += dt;
        if self.step < 64 {
            self.acc += dt;
            while self.acc >= 0.05 && self.step < 64 {
                self.acc -= 0.05;
                self.walk();
            }
        } else {
            self.hold += dt;
            if self.hold > 3.5 {
                self.hold = 0.0;
                self.restart();
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let art = r.h >= 16 && r.w >= 22;
        if !art {
            self.feed.draw(c, r, env);
            return;
        }
        let title = "[RSA 4096]";
        let mut top = String::from("+");
        top.push_str(&"-".repeat(3));
        top.push_str(title);
        top.push_str(&"-".repeat(17 - 3 - title.len()));
        top.push('+');
        put(c, r, 0, 0, &top, pal.dim, 0);
        for y in 0..9 {
            put_ch(c, r, 0, 1 + y as i32, '|', pal.dim, 0);
            put_ch(c, r, 18, 1 + y as i32, '|', pal.dim, 0);
            for x in 0..17 {
                let n = self.field[y][x] as usize;
                let (ch, col, at) = if (x, y) == self.pos && self.step < 64 {
                    ('^', pal.bright, BOLD)
                } else if (x, y) == (8, 4) {
                    ('S', pal.amber, BOLD)
                } else if (x, y) == self.pos && n > 0 {
                    ('E', pal.amber, BOLD)
                } else {
                    (
                        ART[n.min(14)] as char,
                        pal.cyan,
                        if n > 4 { BOLD } else { DIM },
                    )
                };
                put_ch(c, r, 1 + x as i32, 1 + y as i32, ch, col, at);
            }
        }
        put(c, r, 0, 10, "+----[SHA256]-----+", pal.dim, 0);
        if r.w >= 44 {
            put(
                c,
                r,
                22,
                1,
                "Generating public/private rsa key pair",
                pal.main,
                0,
            );
            let mut fp = String::new();
            for (i, b) in self.bytes.iter().enumerate() {
                let _ = write!(fp, "{b:02x}{}", if i < 15 { ":" } else { "" });
            }
            put(c, r, 22, 3, "Fingerprint:", pal.dim, 0);
            put(c, r, 22, 4, &fp, pal.bright, BOLD);
            let steps = format!("Walk {:>2}/64", self.step);
            put(c, r, 22, 6, &steps, pal.amber, 0);
        }
        let sub = Rect {
            x: r.x,
            y: r.y + 11,
            w: r.w,
            h: r.h - 11,
        };
        self.feed.draw(c, sub, env);
    }
}
