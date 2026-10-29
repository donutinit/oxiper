//! The graphical widgets: htop-style meters, bandwidth graph, matrix rain,
//! world map traces, radar, progress bars, satellite tracker, stock ticker,
//! access banners and key generation with randomart.
//!
//! Ideas after Dustin Kirkland's hollywood (Apache License 2.0): htop, bmon,
//! cmatrix, map, sshart, apg and friends, all simulated here.

use super::feed::{Feed, FeedKind};
use super::lexico::{self, Dress};
use super::map::{self, Dots};
use super::revolucion;
use super::{Env, Pal, Rect, Widget, babble, muertos, put, put_ch};
use crate::color::Color;
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Canvas, DIM};
use fastrand::Rng;
use std::collections::VecDeque;
use std::f32::consts::{PI, TAU};
use std::fmt::Write;

/// How the day's holiday after Día de Muertos renames the ordinary widgets.
fn dress(mx: Fiesta) -> Option<&'static Dress> {
    lexico::of(mx).map(|v| &v.dress)
}

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
    mx: Fiesta,
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
    pub fn new(seed: u64, mx: Fiesta) -> Htop {
        let mut rng = Rng::with_seed(seed);
        let mut procs = Vec::new();
        let users = if mx.is(Kind::Muertos) {
            ["root", "mictlan", "xolo", "catrina", "abuela", "nobody"]
        } else if let Some(d) = dress(mx) {
            d.users
        } else {
            ["root", "root", "daemon", "www-data", "neo", "nobody"]
        };
        for _ in 0..24 {
            let mut cmd = String::from(if mx.is(Kind::Muertos) && rng.bool() {
                babble::pick(&mut rng, muertos::DAEMONS)
            } else if let Some(v) = lexico::of(mx)
                && rng.bool()
            {
                babble::pick(&mut rng, v.daemons)
            } else {
                babble::pick(&mut rng, babble::DAEMONS)
            });
            if rng.bool() {
                cmd.push_str(" --");
                cmd.push_str(babble::pick(&mut rng, babble::MODS));
                cmd.push('=');
                cmd.push_str(&rng.u32(1..9999).to_string());
            }
            procs.push(Proc {
                pid: rng.u32(1..32768),
                user: users[rng.usize(..6)],
                cpu: rng.f32() * 30.0,
                mem: rng.f32() * 9.0,
                cmd,
            });
        }
        Htop {
            mx,
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
        if self.mx.is(Kind::Muertos) {
            "htop --almas"
        } else if let Some(d) = dress(self.mx) {
            d.htop
        } else {
            "htop"
        }
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
        let mx = self.mx.is(Kind::Muertos);
        let dr = dress(self.mx);
        let es = mx || dr.is_some();
        let (mem_label, swap_label) = match dr {
            Some(d) => (d.mem, d.swap),
            None if mx => ("Ofr", "Vel"),
            None => ("Mem", "Swp"),
        };
        let mut t = String::new();
        let _ = write!(t, "{:.2}G/15.6G", self.mem * 15.6);
        meter(c, r, 0, y, colw - 1, mem_label, self.mem, &t, env);
        t.clear();
        let _ = write!(t, "{:.2}G/8.00G", self.swap * 8.0);
        if two {
            meter(c, r, colw, y, colw - 1, swap_label, self.swap, &t, env);
        } else {
            y += 1;
            meter(c, r, 0, y, colw - 1, swap_label, self.swap, &t, env);
        }
        y += 1;
        t.clear();
        let up = self.uptime as u32;
        if mx {
            let _ = write!(
                t,
                "Almas: {}, {} en tr\u{00e1}nsito; Carga: {:.2}  Arriba {}d {:02}:{:02}",
                200 + (up % 37),
                900 + (up % 211),
                self.cpu.iter().sum::<f32>() / 4.0,
                up / 86400,
                up / 3600 % 24,
                up / 60 % 60
            );
        } else if let Some(d) = dr {
            let _ = write!(
                t,
                "{}: {}, {} en tr\u{00e1}nsito; Carga: {:.2}  Arriba {}d {:02}:{:02}",
                d.tasks,
                200 + (up % 37),
                900 + (up % 211),
                self.cpu.iter().sum::<f32>() / 4.0,
                up / 86400,
                up / 3600 % 24,
                up / 60 % 60
            );
        } else {
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
        }
        put(c, r, 0, y, &t, pal.main, 0);
        y += 1;
        if r.h - y >= 3 {
            put(
                c,
                r,
                0,
                y,
                if es {
                    "  PID USUARIO   CPU%  MEM%  COMANDO"
                } else {
                    "  PID USER      CPU%  MEM%  COMMAND"
                },
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
    mx: Fiesta,
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
    pub fn new(seed: u64, mx: Fiesta) -> Bandwidth {
        Bandwidth {
            mx,
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
        if self.mx.is(Kind::Muertos) {
            "bmon -p mictlan0"
        } else if let Some(d) = dress(self.mx) {
            d.bmon
        } else {
            "bmon -p eth0"
        }
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
            if self.mx.is(Kind::Muertos) || dress(self.mx).is_some() {
                // More souls come home the higher the holiday.
                self.rx_t *= 0.55 + 0.45 * self.mx.level;
            }
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
        if self.mx.is(Kind::Muertos) {
            let _ = write!(
                t,
                "mictlan0  \u{2193} llegan {:7.1} almas/s  \u{2191} vuelven {:6.1} almas/s",
                last_rx * 1840.0,
                last_tx * 310.0
            );
        } else if let Some(d) = dress(self.mx) {
            let _ = write!(
                t,
                "{}  \u{2193} {} {:7.1} {}  \u{2191} {} {:6.1} {}",
                d.iface,
                d.down,
                last_rx * 1840.0,
                d.down_unit,
                d.up,
                last_tx * 310.0,
                d.up_unit
            );
        } else {
            let _ = write!(
                t,
                "eth0  RX {:7.1} Mbit/s  TX {:7.1} Mbit/s",
                last_rx * 940.0,
                last_tx * 940.0
            );
        }
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
        if self.mx.is(Kind::Muertos) {
            let _ = write!(
                t,
                "pico {:.0} almas/s   prom {:.0} almas/s",
                self.peak * 1840.0,
                self.hist.iter().map(|h| h.0).sum::<f32>() / n.max(1) as f32 * 1840.0
            );
        } else if let Some(d) = dress(self.mx) {
            let _ = write!(
                t,
                "pico {:.0} {}   prom {:.0} {}",
                self.peak * 1840.0,
                d.down_unit,
                self.hist.iter().map(|h| h.0).sum::<f32>() / n.max(1) as f32 * 1840.0,
                d.down_unit
            );
        } else {
            let _ = write!(
                t,
                "peak {:.0} Mbit/s   avg {:.0} Mbit/s",
                self.peak * 940.0,
                self.hist.iter().map(|h| h.0).sum::<f32>() / n.max(1) as f32 * 940.0
            );
        }
        put(c, r, 0, r.h - 1, &t, pal.dim, 0);
    }
}

// ---------------------------------------------------------------- rain

#[derive(Clone, Copy)]
struct Drop {
    y: f32,
    speed: f32,
    len: i32,
    /// Día de Muertos: a word the drop spells top to bottom (or "").
    word: &'static str,
}

/// Words that fall in the holiday's rain.
const RAIN_WORDS: &[&str] = &[
    "MICTLAN",
    "CEMPASUCHIL",
    "ABUELA",
    "OFRENDA",
    "CALAVERA",
    "VELADORA",
    "XOLO",
    "CATRINA",
    "COPAL",
    "ALMAS",
    "RECUERDO",
    "MARIPOSA",
    "ALEBRIJE",
    "PANTEON",
    "FLORES",
    "NOVIEMBRE",
    "PAN",
    "SAL",
];

pub struct Rain {
    mx: Fiesta,
    rng: Rng,
    drops: Vec<Drop>,
    h: i32,
    tick: u32,
    acc: f32,
}

#[inline]
pub(super) fn hash(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9e37_79b1) ^ b.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^ (h >> 12)
}

/// The head of a drop in a holiday's rain: its star.
fn rain_head(kind: Kind) -> char {
    match kind {
        Kind::Navidad => '\u{2736}',
        Kind::Revolucion => '\u{25ae}',
        _ => '\u{2605}',
    }
}

/// The color a column's trail takes in a holiday's rain: green and red
/// turn by turn for Navidad.
fn rain_trail(kind: Kind, x: usize, pal: &Pal) -> Color {
    match kind {
        Kind::Navidad => [pal.green, pal.red][x % 2],
        // The flag: verde, blanco y rojo, column by column.
        Kind::Patrias => [pal.green, pal.bright, pal.red][x % 3],
        // Brass among the sepia.
        Kind::Revolucion if x.is_multiple_of(5) => pal.oro,
        _ => pal.main,
    }
}

/// What the rain is made of: the kana of the matrix, or in the Revolución the
/// dots and dashes of the telegraph.
fn rain_glyph(kind: Kind, h2: u32, plain: char) -> char {
    match kind {
        Kind::Revolucion => [
            '\u{00b7}', '\u{2501}', '\u{00b7}', '\u{2501}', '\u{00b7}', ' ',
        ][(h2 / 5 % 6) as usize],
        _ => plain,
    }
}

/// The glints among a holiday's rain.
fn rain_sparkle(kind: Kind, n: u32) -> char {
    let _ = kind;
    ['\u{2726}', '\u{2727}', '\u{2736}', '\u{2022}'][n as usize % 4]
}

impl Rain {
    pub fn new(seed: u64, mx: Fiesta) -> Rain {
        Rain {
            mx,
            rng: Rng::with_seed(seed),
            drops: Vec::new(),
            h: 0,
            tick: 0,
            acc: 0.0,
        }
    }

    fn respawn(rng: &mut Rng, h: i32, first: bool, mx: Fiesta) -> Drop {
        let mut len = rng.i32(5..(h.max(8) * 2 / 3).max(7));
        // Día de Muertos: some drops spell a word, more of them the higher
        // the holiday (drawn after the ordinary numbers).
        let mut word = "";
        if mx.is(Kind::Muertos) && rng.f32() < 0.1 + 0.2 * mx.level {
            word = RAIN_WORDS[rng.usize(..RAIN_WORDS.len())];
            len = len.max(word.len() as i32 + 3);
        } else if let Some(d) = dress(mx)
            && rng.f32() < 0.1 + 0.2 * mx.level
        {
            word = d.rain_words[rng.usize(..d.rain_words.len())];
            len = len.max(word.len() as i32 + 3);
        }
        Drop {
            y: if first {
                rng.f32() * (h + len) as f32 - len as f32
            } else {
                -(rng.f32() * h as f32 * 0.7) - 1.0
            },
            speed: 6.0 + rng.f32() * 16.0,
            len,
            word,
        }
    }
}

impl Widget for Rain {
    fn name(&self) -> &'static str {
        if self.mx.is(Kind::Muertos) {
            "cmatrix -ab --petalos"
        } else if let Some(d) = dress(self.mx) {
            d.rain
        } else {
            "cmatrix -ab"
        }
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.h = h as i32;
        self.drops = (0..w)
            .map(|_| Self::respawn(&mut self.rng, h as i32, true, self.mx))
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
                *d = Self::respawn(&mut self.rng, self.h, false, self.mx);
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let dr = dress(self.mx);
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
                let mut ch = if h2 % 5 == 0 {
                    char::from_digit(h2 / 5 % 10, 10).unwrap_or('0')
                } else {
                    char::from_u32(0xff66 + h2 / 5 % 56).unwrap_or('?')
                };
                let (mut col, mut at) = if k == 0 {
                    (pal.bright, BOLD)
                } else {
                    (pal.ramp.at(1.0 - k as f32 / d.len as f32 * 0.85), 0)
                };
                if self.mx.is(Kind::Muertos) {
                    // Petals fall among the kana, flowers head the drops and
                    // the words read from top to bottom.
                    let wl = d.word.len() as i32;
                    if k < wl {
                        ch = d.word.as_bytes()[(wl - 1 - k) as usize] as char;
                        col = pal.fade(pal.bright, k as f32 / wl as f32 * 0.35);
                        at = BOLD;
                    } else if k == 0 {
                        ch = '\u{273f}';
                        col = pal.flor.scale(1.25);
                    } else if h2 % 9 == 0 {
                        ch =
                            ['\u{273f}', '\u{2740}', '\u{273e}', '\u{2741}'][(h2 / 9 % 4) as usize];
                        col = pal.flor.lerp(col, k as f32 / d.len as f32);
                    }
                } else if dr.is_some() {
                    // The holiday's rain: its words read from top to bottom,
                    // its star heads the drops and the trails take its colors.
                    let wl = d.word.len() as i32;
                    let fall = k as f32 / d.len as f32;
                    if k < wl {
                        ch = d.word.as_bytes()[(wl - 1 - k) as usize] as char;
                        col = pal.fade(pal.bright, k as f32 / wl as f32 * 0.35);
                        at = BOLD;
                    } else if k == 0 {
                        ch = rain_head(self.mx.kind);
                        col = pal.oro.scale(1.2);
                    } else {
                        ch = rain_glyph(self.mx.kind, h2, ch);
                        col = pal.fade(rain_trail(self.mx.kind, x, pal), fall * 0.85);
                        if h2 % 9 == 0 {
                            ch = rain_sparkle(self.mx.kind, h2 / 9);
                            col = pal.oro.lerp(col, fall);
                        }
                    }
                }
                put_ch(c, r, x as i32, row, ch, col, at);
            }
        }
    }
}

// ---------------------------------------------------------------- world map

struct Arc {
    from: (&'static str, f32, f32),
    to: (&'static str, f32, f32),
    t: f32,
    glow: f32,
}

pub struct WorldMap {
    mx: Fiesta,
    /// Souls that got home (Día de Muertos).
    home: u32,
    rng: Rng,
    land: Dots,
    fx: Dots,
    arcs: Vec<Arc>,
    spawn: f32,
    painted: bool,
}

impl WorldMap {
    pub fn new(seed: u64, mx: Fiesta) -> WorldMap {
        WorldMap {
            mx,
            home: 100 + (seed % 900) as u32,
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
        if self.mx.is(Kind::Muertos) {
            "tracemap --regreso"
        } else if let Some(d) = dress(self.mx) {
            d.map
        } else {
            "tracemap --live"
        }
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
            let (from, to) = if self.mx.is(Kind::Revolucion) {
                // The telegrams of the Revolución, along the historic routes.
                revolucion::ROUTES[self.rng.usize(..revolucion::ROUTES.len())]
            } else if self.mx.is(Kind::Muertos) || dress(self.mx).is_some() {
                // Souls (or greetings, or aguinaldos, or echoes of the Grito)
                // of the whole world on their way back to Mexico.
                let homes = if self.mx.is(Kind::Patrias) {
                    map::SITES_PATRIAS
                } else {
                    map::HOMES
                };
                (
                    map::ORIGINS[self.rng.usize(..map::ORIGINS.len())],
                    homes[self.rng.usize(..homes.len())],
                )
            } else {
                let from = self.rng.usize(..map::CITIES.len());
                let to = (from + 1 + self.rng.usize(..map::CITIES.len() - 1)) % map::CITIES.len();
                (map::CITIES[from], map::CITIES[to])
            };
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
                if a.t >= 1.0 {
                    self.home += 1;
                }
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
        let mx = self.mx.is(Kind::Muertos);
        let dr = dress(self.mx);
        for a in &self.arcs {
            let (_, la, lo) = a.from;
            let (_, lb, lob) = a.to;
            let (x0, y0) = map::project(la, lo, pw, ph);
            let (x1, y1) = map::project(lb, lob, pw, ph);
            let lift = ((x1 - x0).abs() + (y1 - y0).abs()) * 0.25;
            let steps = 40;
            let col = match (a.glow > 0.0, mx) {
                (true, _) => pal.red,
                (false, true) => pal.main,
                (false, false) if dr.is_some() => pal.oro,
                (false, false) => pal.amber,
            };
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
            status = if mx {
                format!("ALMA EN CAMINO: {} -> {}", a.from.0, a.to.0)
            } else if let Some(d) = dr {
                format!("{}: {} -> {}", d.trace, a.from.0, a.to.0)
            } else {
                format!("TRACE {} -> {}", a.from.0, a.to.0)
            };
        }
        self.fx.blit(c, body);
        for a in &self.arcs {
            let (_, lb, lob) = a.to;
            let (x1, y1) = map::project(lb, lob, pw, ph);
            let blink = a.t >= 1.0 && (a.glow * 6.0) as i32 % 2 == 0;
            let ch = match (mx, blink) {
                (true, true) => '\u{2740}',
                (true, false) => '\u{273f}',
                (false, true) if dr.is_some() => '\u{2736}',
                (false, false) if dr.is_some() => '\u{2726}',
                (false, true) => '\u{25c9}',
                (false, false) => '\u{25cf}',
            };
            let col = if a.t >= 1.0 {
                pal.red
            } else if mx {
                pal.flor
            } else if dr.is_some() {
                pal.oro
            } else {
                pal.amber
            };
            put_ch(c, body, (x1 / 2.0) as i32, (y1 / 4.0) as i32, ch, col, BOLD);
        }
        put(c, r, 0, 0, &status, pal.bright, BOLD);
        if mx {
            let t = format!("EN CASA: {}", self.home);
            let tw = t.chars().count() as i32;
            if r.w >= status.chars().count() as i32 + tw + 2 {
                put(c, r, r.w - tw, 0, &t, pal.flor, BOLD);
            }
        } else if let Some(d) = dr {
            let t = format!("{}: {}", d.arrived, self.home);
            let tw = t.chars().count() as i32;
            if r.w >= status.chars().count() as i32 + tw + 2 {
                put(c, r, r.w - tw, 0, &t, pal.oro, BOLD);
            }
        }
    }
}

// ---------------------------------------------------------------- radar

struct Blip {
    ang: f32,
    dist: f32,
    glow: f32,
    id: u16,
    /// Día de Muertos: the soul's name and how long ago it got home.
    soul: &'static str,
    home: f32,
}

pub struct Radar {
    mx: Fiesta,
    rng: Rng,
    dots: Dots,
    sweep: f32,
    blips: Vec<Blip>,
}

impl Radar {
    pub fn new(seed: u64, mx: Fiesta) -> Radar {
        let mut rng = Rng::with_seed(seed);
        // The holiday tracks more souls the higher it is.
        let n = if mx.is(Kind::Muertos) || dress(mx).is_some() {
            5 + (6.0 * mx.level).round() as usize
        } else {
            9
        };
        let blips = (0..n)
            .map(|_| Blip {
                ang: rng.f32() * TAU,
                dist: 0.15 + rng.f32() * 0.8,
                glow: 0.0,
                id: rng.u16(100..999),
                soul: if mx.is(Kind::Muertos) {
                    muertos::soul(&mut rng, mx.level)
                } else if let Some(v) = lexico::of(mx) {
                    babble::pick(&mut rng, v.people)
                } else {
                    ""
                },
                home: 9.0,
            })
            .collect();
        Radar {
            mx,
            rng,
            dots: Dots::new(1, 1),
            sweep: 0.0,
            blips,
        }
    }

    #[cfg(test)]
    pub fn blips_len(&self) -> usize {
        self.blips.len()
    }
}

impl Widget for Radar {
    fn name(&self) -> &'static str {
        if self.mx.is(Kind::Muertos) {
            "radar-almas --sweep"
        } else if let Some(d) = dress(self.mx) {
            d.radar
        } else {
            "radar --sweep"
        }
    }

    fn resize(&mut self, w: u16, h: u16) {
        let rw = if w >= 50 { w - 24 } else { w };
        self.dots = Dots::new(rw.max(1), h.max(1));
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        let prev = self.sweep;
        self.sweep = (self.sweep + dt * 1.6) % TAU;
        let mx = self.mx.is(Kind::Muertos) || dress(self.mx).is_some();
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
            if mx {
                // Souls walk toward the middle, the house with the ofrenda,
                // and a new one sets out from the edge when one gets there.
                b.home += dt;
                b.dist -= dt * (0.012 + 0.0004 * (b.id % 17) as f32);
                b.ang = (b.ang + (self.rng.f32() - 0.5) * 0.02).rem_euclid(TAU);
                if b.dist < 0.07 {
                    b.home = 0.0;
                    b.dist = 0.9 + self.rng.f32() * 0.08;
                    b.ang = self.rng.f32() * TAU;
                    b.soul = match lexico::of(self.mx) {
                        Some(v) => babble::pick(&mut self.rng, v.people),
                        None => muertos::soul(&mut self.rng, self.mx.level),
                    };
                    b.id = self.rng.u16(100..999);
                }
                continue;
            }
            // Contacts drift slowly.
            b.ang = (b.ang + (self.rng.f32() - 0.5) * 0.01).rem_euclid(TAU);
            b.dist = (b.dist + (self.rng.f32() - 0.5) * 0.004).clamp(0.1, 0.95);
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let dr = dress(self.mx);
        let mx = self.mx.is(Kind::Muertos) || dr.is_some();
        // The light that sweeps and burns in the middle: a marigold's or a
        // holiday's gold.
        let glow = if dr.is_some() { pal.oro } else { pal.flor };
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
            } else if mx {
                pal.fade(glow, 0.15 + i as f32 * 0.1)
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
        if mx {
            // The ofrenda (or the star of Belén) in the middle, its candle lit.
            let lit = (self.sweep * 2.0) as i32 % 2 == 0;
            let (ch, col) = match dr {
                Some(_) => (if lit { '\u{2726}' } else { '\u{2736}' }, pal.oro),
                None => (if lit { '\u{2740}' } else { '\u{273f}' }, pal.vela),
            };
            put_ch(c, area, (cx / 2.0) as i32, (cy / 4.0) as i32, ch, col, BOLD);
        }
        for b in &self.blips {
            if b.glow > 0.02 || mx {
                let x = cx + b.ang.cos() * b.dist * rad;
                let y = cy + b.ang.sin() * b.dist * rad;
                // The souls of the holiday stay faintly in sight between sweeps.
                let glow = if mx { b.glow.max(0.3) } else { b.glow };
                let col = pal.fade(if glow > 0.6 { pal.red } else { pal.amber }, 1.0 - glow);
                put_ch(
                    c,
                    area,
                    (x / 2.0) as i32,
                    (y / 4.0) as i32,
                    if mx { '\u{2726}' } else { '\u{25c6}' },
                    col,
                    BOLD,
                );
            }
        }
        if r.w >= 50 {
            let x0 = rw + 1;
            if let Some(d) = dr {
                put(c, r, x0, 0, d.contacts, pal.cyan, BOLD);
            } else if mx {
                put(c, r, x0, 0, "ALMAS EN CAMINO", pal.cyan, BOLD);
            } else {
                put(c, r, x0, 0, "CONTACTS", pal.cyan, BOLD);
            }
            for (i, b) in self.blips.iter().enumerate() {
                let y = 2 + i as i32;
                let col = if b.glow > 0.5 { pal.red } else { pal.main };
                if mx {
                    let name = b.soul;
                    let t = if b.home < 4.0 {
                        format!("{name:<13} {}", dr.map_or("EN CASA", |d| d.home))
                    } else {
                        format!(
                            "{name:<13} {:03}\u{00b0} {:4.1}",
                            (b.ang.to_degrees() as i32 + 90).rem_euclid(360),
                            b.dist * 40.0
                        )
                    };
                    let col = if b.home < 4.0 { glow } else { col };
                    put(c, r, x0, y, &t, col, if b.home < 4.0 { BOLD } else { 0 });
                    continue;
                }
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
    mx: Fiesta,
    rng: Rng,
    bars: Vec<Bar>,
}

impl Progress {
    pub fn new(seed: u64, mx: Fiesta) -> Progress {
        Progress {
            mx,
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
        if self.mx.is(Kind::Muertos) {
            "pv --ofrenda"
        } else if let Some(d) = dress(self.mx) {
            d.progress
        } else {
            "pv --force"
        }
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
                if self.mx.is(Kind::Muertos) && self.rng.f32() < muertos::share(self.mx.level) {
                    muertos::progress_label(&mut self.rng, env.level.max(1), &mut b.label);
                } else if let Some(v) = lexico::of(self.mx)
                    && self.rng.f32() < lexico::share(self.mx.level)
                {
                    v.progress_label(&mut self.rng, env.level.max(1), &mut b.label);
                } else {
                    babble::progress_label(&mut self.rng, env.level.max(1), &mut b.label);
                }
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
                put(
                    c,
                    r,
                    0,
                    y,
                    if self.mx.is(Kind::Muertos) || dress(self.mx).is_some() {
                        "COMPLETO"
                    } else {
                        "COMPLETE"
                    },
                    pal.green,
                    BOLD,
                );
                put(c, r, 9, y, &b.label, pal.dim, 0);
            } else {
                put(c, r, 0, y, &b.label, pal.bright, BOLD);
            }
            let bw = (r.w - 20).max(4);
            let filled = (b.pct / 100.0 * bw as f32) as i32;
            put(c, r, 0, y + 1, "[", pal.dim, 0);
            for x in 0..bw {
                let (ch, col) = if self.mx.is(Kind::Muertos) && !done && x + 1 == filled {
                    // A marigold at the head of the bar.
                    ('\u{273f}', pal.flor)
                } else if dress(self.mx).is_some() && !done && x + 1 == filled {
                    // A star at the head of it.
                    ('\u{2736}', pal.oro)
                } else if x < filled {
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

/// Día de Muertos: Mexico's satellites, some real and some on their way home.
const SAT_NAMES_MX: &[&str] = &[
    "MORELOS-3",
    "SATMEX-8",
    "QUETZSAT-1",
    "BICENTENARIO",
    "CENTENARIO",
    "AZTECHSAT-1",
    "XOLO-1",
    "MONARCA-9",
    "CATRINA-2",
    "ALEBRIJE-7",
    "COPAL-4",
    "TONATIUH-1",
    "COATLICUE-3",
    "ISS (ZARYA)",
    "VELADORA-12",
    "MICTLAN-9",
];

pub struct Satellites {
    mx: Fiesta,
    rng: Rng,
    land: Dots,
    fx: Dots,
    sats: Vec<Sat>,
    painted: bool,
    list: i32,
}

impl Satellites {
    pub fn new(seed: u64, mx: Fiesta) -> Satellites {
        let mut rng = Rng::with_seed(seed);
        let mut names: Vec<&str> = if mx.is(Kind::Muertos) {
            SAT_NAMES_MX
        } else if let Some(d) = dress(mx) {
            d.sat_names
        } else {
            SAT_NAMES
        }
        .to_vec();
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
            mx,
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
        if self.mx.is(Kind::Muertos) {
            "rastreador-satelital"
        } else if let Some(d) = dress(self.mx) {
            d.sats
        } else {
            "satellite-tracker"
        }
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
            if self.mx.is(Kind::Muertos) {
                "RASTREO ORBITAL  ENLACE AL MICTL\u{00c1}N"
            } else if let Some(d) = dress(self.mx) {
                d.sats_head
            } else {
                "ORBITAL TRACKING  UPLINK LOCKED"
            },
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

/// Día de Muertos: the stock market of the Mictlán, (symbol, bullish): the
/// marigold, the candle and the bread only go up as the holiday builds up.
const SYMBOLS_MX: &[(&str, bool)] = &[
    ("CEMPAS", true),
    ("COPAL", false),
    ("VELA", true),
    ("PAN", true),
    ("MOLE", false),
    ("TAMAL", false),
    ("ATOLE", false),
    ("MEZCAL", false),
    ("PAPEL", false),
    ("CALAV", true),
    ("TEJOC", false),
    ("CA\u{00d1}A", false),
    ("CHOCO", false),
    ("XOLO", true),
    ("ALEBR", false),
    ("CATRIN", true),
];

struct Quote {
    sym: &'static str,
    price: f32,
    open: f32,
    bull: bool,
    hist: VecDeque<f32>,
}

pub struct Ticker {
    mx: Fiesta,
    rng: Rng,
    quotes: Vec<Quote>,
    timer: f32,
}

impl Ticker {
    pub fn new(seed: u64, mx: Fiesta) -> Ticker {
        Ticker {
            mx,
            rng: Rng::with_seed(seed),
            quotes: Vec::new(),
            timer: 0.0,
        }
    }
}

impl Widget for Ticker {
    fn name(&self) -> &'static str {
        if self.mx.is(Kind::Muertos) {
            "bolsa --mictlan"
        } else if let Some(d) = dress(self.mx) {
            d.ticker
        } else {
            "ticker --crypto"
        }
    }

    fn resize(&mut self, _w: u16, h: u16) {
        let holiday: Option<&'static [(&'static str, bool)]> = if self.mx.is(Kind::Muertos) {
            Some(SYMBOLS_MX)
        } else {
            dress(self.mx).map(|d| d.symbols)
        };
        if let Some(list) = holiday {
            let n = (h as usize).saturating_sub(2).clamp(1, list.len());
            let mut syms: Vec<(&'static str, bool)> = list.to_vec();
            self.rng.shuffle(&mut syms);
            self.quotes = syms
                .into_iter()
                .take(n)
                .map(|(sym, bull)| {
                    let p = 8.0 + self.rng.f32().powi(2) * 400.0;
                    Quote {
                        sym,
                        price: p,
                        open: p,
                        bull,
                        hist: VecDeque::from(vec![p; 12]),
                    }
                })
                .collect();
            return;
        }
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
                    bull: false,
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
        // Marigolds, candles and bread climb with the holiday.
        let lift = if self.mx.is(Kind::Muertos) || dress(self.mx).is_some() {
            0.012 * self.mx.level
        } else {
            0.0
        };
        for q in self.quotes.iter_mut() {
            q.price *= 1.0 + (self.rng.f32() - 0.5) * 0.03 + if q.bull { lift } else { 0.0 };
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
            if self.mx.is(Kind::Muertos) {
                "CLAVE       \u{00da}LTIMO   CAMB%  TENDENCIA"
            } else if let Some(d) = dress(self.mx) {
                d.ticker_head
            } else {
                "SYMBOL      LAST     CHG%  TREND"
            },
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

/// A 3x5 pixel font for the banner letters. The holidays after Día de
/// Muertos need more of the alphabet than "ACCESS DENIED" does.
pub(super) fn glyph(ch: char) -> [u8; 5] {
    match ch {
        'A' => [0b010, 0b101, 0b111, 0b101, 0b101],
        'B' => [0b110, 0b101, 0b110, 0b101, 0b110],
        'C' => [0b011, 0b100, 0b100, 0b100, 0b011],
        'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
        'E' => [0b111, 0b100, 0b110, 0b100, 0b111],
        'F' => [0b111, 0b100, 0b110, 0b100, 0b100],
        'G' => [0b011, 0b100, 0b101, 0b101, 0b011],
        'H' => [0b101, 0b101, 0b111, 0b101, 0b101],
        'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
        'J' => [0b001, 0b001, 0b001, 0b101, 0b010],
        'K' => [0b101, 0b101, 0b110, 0b101, 0b101],
        'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
        'M' => [0b111, 0b111, 0b101, 0b101, 0b101],
        'N' => [0b101, 0b111, 0b111, 0b101, 0b101],
        'P' => [0b110, 0b101, 0b110, 0b100, 0b100],
        'Q' => [0b010, 0b101, 0b101, 0b111, 0b011],
        'R' => [0b110, 0b101, 0b110, 0b101, 0b101],
        'S' => [0b011, 0b100, 0b010, 0b001, 0b110],
        'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
        'O' => [0b010, 0b101, 0b101, 0b101, 0b010],
        'U' => [0b101, 0b101, 0b101, 0b101, 0b111],
        'V' => [0b101, 0b101, 0b101, 0b101, 0b010],
        'W' => [0b101, 0b101, 0b111, 0b111, 0b101],
        'X' => [0b101, 0b101, 0b010, 0b101, 0b101],
        'Y' => [0b101, 0b101, 0b010, 0b010, 0b010],
        'Z' => [0b111, 0b001, 0b010, 0b100, 0b111],
        '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
        '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
        '2' => [0b110, 0b001, 0b010, 0b100, 0b111],
        '3' => [0b110, 0b001, 0b010, 0b001, 0b110],
        '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
        '5' => [0b111, 0b100, 0b110, 0b001, 0b110],
        '6' => [0b011, 0b100, 0b111, 0b101, 0b111],
        '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
        '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
        '9' => [0b111, 0b101, 0b111, 0b001, 0b110],
        '!' => [0b010, 0b010, 0b010, 0b000, 0b010],
        ':' => [0b000, 0b010, 0b000, 0b010, 0b000],
        '\u{00a1}' => [0b010, 0b000, 0b010, 0b010, 0b010],
        _ => [0; 5],
    }
}

/// Can the banner font draw `ch`?
#[cfg(test)]
pub(super) fn has_glyph(ch: char) -> bool {
    glyph(ch) != [0; 5]
}

pub struct Access {
    mx: Fiesta,
    feed: Feed,
    rng: Rng,
    /// Some(granted, seconds left) while a banner is up.
    banner: Option<(bool, f32)>,
    wait: f32,
}

impl Access {
    pub fn new(seed: u64, level: u8, mx: Fiesta) -> Access {
        let mut rng = Rng::with_seed(seed ^ 0xacce55);
        Access {
            mx,
            feed: Feed::new(FeedKind::Auth, seed, level, mx),
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
        if self.mx.is(Kind::Muertos) {
            "authd --mictlan"
        } else if let Some(d) = dress(self.mx) {
            d.authd
        } else {
            "authd"
        }
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
                    // The souls are let in more often the higher the holiday.
                    let granted = if self.mx.is(Kind::Muertos) || dress(self.mx).is_some() {
                        self.rng.f32() < 0.25 + 0.4 * self.mx.level
                    } else {
                        self.rng.u8(..10) < 4
                    };
                    self.banner = Some((granted, 1.8));
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
        let dr = dress(self.mx);
        let words = dr.map(|d| if granted { &d.granted } else { &d.denied });
        let mx = self.mx.is(Kind::Muertos) || dr.is_some();
        let (w1, w2) = match (words, mx, granted) {
            (Some(w), _, _) => (w[0], w[1]),
            (None, true, true) => ("ACCESO", "CONCEDIDO"),
            (None, true, false) => ("ACCESO", "DENEGADO"),
            (None, false, true) => ("ACCESS", "GRANTED"),
            (None, false, false) => ("ACCESS", "DENIED"),
        };
        // Blink in the last moments.
        let on = left > 0.5 || (left * 10.0) as i32 % 2 == 0;
        if !on {
            return;
        }
        // (The ordinary banners keep their breakpoint: DENIED is narrower than GRANTED.)
        let wide = if mx { w2.len() as i32 * 8 + 2 } else { 58 };
        let sx = if r.w >= wide { 2 } else { 1 };
        let bw = (w2.len() as i32 * 4 * sx + 2).min(r.w);
        let (bh, big) = if r.h >= 13 && r.w >= w2.len() as i32 * 4 + 2 {
            (13, true)
        } else {
            (3, false)
        };
        if r.h < bh || r.w < 10 {
            return;
        }
        // The little banner says it in one line; at the holiday it says as
        // much of "ACCESO AL MICTLAN DENEGADO" as fits.
        let small = if !mx || dr.is_some() {
            format!("{w1} {w2}")
        } else if r.w >= 32 {
            format!("{w1} AL MICTL\u{00c1}N {w2}")
        } else if r.w >= 22 {
            format!("MICTL\u{00c1}N {w2}")
        } else {
            w2.to_string()
        };
        let bw = if big {
            bw
        } else {
            (small.chars().count() as i32 + 4).min(r.w)
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
            if mx {
                // Captions on the frame: where to, and what the guard says.
                let (up, down) = match words {
                    Some(w) => (w[2], w[3]),
                    None if granted => (" MICTL\u{00c1}N ", " \u{00a1}PASA, TE ESPERAN! "),
                    None => (" MICTL\u{00c1}N ", " TODAV\u{00cd}A NO TE TOCA "),
                };
                for (y, t) in [(by, up), (by + bh - 1, down)] {
                    let n = t.chars().count() as i32;
                    if n + 2 <= bw {
                        put(c, r, bx + (bw - n) / 2, y, t, pal.bright, BOLD);
                    }
                }
            }
        } else {
            put(
                c,
                r,
                bx + (bw - small.chars().count() as i32) / 2,
                by + 1,
                &small,
                col,
                BOLD,
            );
        }
    }
}

// ---------------------------------------------------------------- keygen

const ART: &[u8; 17] = b" .o+=*BOX@%&#/^SE";
/// Día de Muertos: the walk leaves flowers, one more petal each visit.
const ART_MX: [char; 15] = [
    ' ', '.', '\u{00b7}', ':', '+', 'x', '\u{2022}', '*', '\u{273e}', '\u{2740}', '\u{273f}',
    '\u{2741}', '\u{2742}', '\u{273b}', '\u{2735}',
];

pub struct Keygen {
    mx: Fiesta,
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
    pub fn new(seed: u64, level: u8, mx: Fiesta) -> Keygen {
        let mut k = Keygen {
            mx,
            feed: Feed::new(FeedKind::Passwords, seed, level, mx),
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
        if self.mx.is(Kind::Muertos) {
            "ssh-keygen -lv --altar"
        } else if let Some(d) = dress(self.mx) {
            d.keygen
        } else {
            "ssh-keygen -lv"
        }
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
        let dr = dress(self.mx);
        let mx = self.mx.is(Kind::Muertos) || dr.is_some();
        let title = match dr {
            Some(d) => d.key_title,
            None if mx => "[ALMA 4096]",
            None => "[RSA 4096]",
        };
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
                    (
                        if !mx {
                            '^'
                        } else {
                            dr.map_or('\u{2726}', |d| d.art_walker)
                        },
                        pal.bright,
                        BOLD,
                    )
                } else if (x, y) == (8, 4) {
                    // Start and end: the house and the ofrenda.
                    (
                        if !mx {
                            'S'
                        } else {
                            dr.map_or('\u{2302}', |d| d.art_start)
                        },
                        pal.amber,
                        BOLD,
                    )
                } else if (x, y) == self.pos && n > 0 {
                    (
                        if !mx {
                            'E'
                        } else {
                            dr.map_or('\u{2740}', |d| d.art_end)
                        },
                        pal.amber,
                        BOLD,
                    )
                } else if let Some(d) = dr {
                    // Ornaments, from dim dots to green stars to gold as they pile up.
                    let k = n.min(14);
                    let col = if k < 4 {
                        pal.dim
                    } else if k < 9 {
                        pal.main
                    } else {
                        pal.oro
                    };
                    (d.art[k], col, if n > 4 { BOLD } else { 0 })
                } else if mx {
                    // Flowers, from white to orange to rosa as they pile up.
                    let k = n.min(14);
                    let col = if k < 4 {
                        pal.dim
                    } else if k < 9 {
                        pal.flor
                    } else {
                        pal.red
                    };
                    (ART_MX[k], col, if n > 4 { BOLD } else { 0 })
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
        put(
            c,
            r,
            0,
            10,
            if let Some(d) = dr {
                d.key_end
            } else if mx {
                "+---[PANTE\u{00d3}N]-----+"
            } else {
                "+----[SHA256]-----+"
            },
            pal.dim,
            0,
        );
        if r.w >= 44 {
            put(
                c,
                r,
                22,
                1,
                if let Some(d) = dr {
                    d.key_lines[0]
                } else if mx {
                    "Generando la llave de la ofrenda"
                } else {
                    "Generating public/private rsa key pair"
                },
                pal.main,
                0,
            );
            let mut fp = String::new();
            for (i, b) in self.bytes.iter().enumerate() {
                let _ = write!(fp, "{b:02x}{}", if i < 15 { ":" } else { "" });
            }
            put(
                c,
                r,
                22,
                3,
                match dr {
                    Some(d) => d.key_lines[1],
                    None if mx => "Huella:",
                    None => "Fingerprint:",
                },
                pal.dim,
                0,
            );
            put(c, r, 22, 4, &fp, pal.bright, BOLD);
            let walk = match dr {
                Some(d) => d.key_lines[2],
                None if mx => "Paso",
                None => "Walk",
            };
            let steps = format!("{walk} {:>2}/64", self.step);
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
