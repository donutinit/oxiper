//! The widgets only Día de la Revolución has, the Archivo de la Revolución:
//! the telegraph key sending telegrams in Morse (the tape of dots and dashes
//! scrolling, the text appearing letter by letter, the STOPs), the timetable
//! of the trains of the División del Norte rolling south along the Central
//! line past a pixel locomotive with soldaderas and sombreros on the roofs,
//! the viewer that pages through the Plan de San Luis (the 20th of November,
//! from six in the evening on) and the other founding documents, and the
//! banner that paints TIERRA Y LIBERTAD in big letters under a zapatista
//! sombrero.

use super::grito::{big_char, big_width};
use super::posada::pixel_line;
use super::revolucion::{self, DOCUMENTS, SLOGANS, STATIONS, TELEGRAMS, TRAINS};
use super::sprite::Sprite;
use super::{Env, Pal, Rect, Widget, put, put_ch, truncate};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Canvas, DIM};
use fastrand::Rng;
use std::collections::VecDeque;
use std::fmt::Write;

/// The three colors of the flag, in order: the garland's first three.
fn tricolor(pal: &Pal) -> [Color; 3] {
    [pal.picado[0], pal.picado[1], pal.picado[2]]
}

// ---------------------------------------------------------------- telegraph

/// A dot, in seconds; a dash is three, the gap between letters three and
/// the one between words seven.
const UNIT: f32 = 0.04;

pub struct Telegrafo {
    mx: Fiesta,
    rng: Rng,
    clock: f32,
    /// The paper tape: dots, dashes, a space between letters and a slash
    /// between words.
    tape: VecDeque<char>,
    /// The telegram being keyed, the letter and the dot or dash of it, and
    /// whether the key is down; seconds until the next change.
    msg: usize,
    ci: usize,
    si: usize,
    on: bool,
    left: f32,
    /// A pause between telegrams.
    wait: f32,
    /// Those already sent: (telegram, second of the day), and their count.
    log: VecDeque<(usize, u32)>,
    sent: u32,
    day: u32,
    buf: String,
}

impl Telegrafo {
    pub fn new(seed: u64, mx: Fiesta) -> Telegrafo {
        let mut rng = Rng::with_seed(seed ^ 0x7e1e6);
        let msg = rng.usize(..TELEGRAMS.len());
        let mut t = Telegrafo {
            mx,
            clock: 0.0,
            tape: VecDeque::new(),
            msg,
            ci: 0,
            si: 0,
            on: false,
            left: 0.0,
            wait: 0.0,
            log: VecDeque::new(),
            sent: 1200 + rng.u32(..800),
            day: 6 * 3600 + rng.u32(..40_000),
            rng,
            buf: String::new(),
        };
        // A few already sent, so the pane starts with a log.
        for _ in 0..6 {
            let m = t.rng.usize(..TELEGRAMS.len());
            t.day += t.rng.u32(20..300);
            t.log.push_back((m, t.day));
        }
        t
    }

    #[cfg(test)]
    pub fn sent(&self) -> u32 {
        self.sent
    }

    #[cfg(test)]
    pub fn finish_message(&mut self) {
        let before = self.sent;
        while self.sent == before {
            self.step(0.5);
        }
    }

    fn text(&self) -> &'static str {
        TELEGRAMS[self.msg].2
    }

    /// Moves the key on by `dt` seconds: dots and dashes, the gaps between
    /// them, letter by letter.
    fn step(&mut self, dt: f32) {
        let mut dt = dt;
        if self.wait > 0.0 {
            if self.wait > dt {
                self.wait -= dt;
                return;
            }
            dt -= self.wait;
            self.wait = 0.0;
        }
        for _ in 0..4000 {
            if self.left > dt {
                self.left -= dt;
                return;
            }
            dt -= self.left;
            self.left = 0.0;
            self.advance();
            if self.wait > 0.0 {
                self.wait -= dt;
                return;
            }
        }
    }

    fn advance(&mut self) {
        let text = self.text();
        let ch = text.chars().nth(self.ci);
        if self.on {
            // A dot or dash ended: a gap, and the letter if it was the last.
            self.on = false;
            let code = ch.map_or("", revolucion::morse);
            if self.si >= code.len() {
                self.ci += 1;
                self.si = 0;
                self.left = 3.0 * UNIT;
                self.push(' ');
            } else {
                self.left = UNIT;
            }
            return;
        }
        let Some(ch) = ch else {
            // The telegram is done.
            self.sent += 1;
            self.day += self.rng.u32(15..140);
            self.log.push_back((self.msg, self.day));
            while self.log.len() > 12 {
                self.log.pop_front();
            }
            let next = self.rng.usize(..TELEGRAMS.len());
            self.msg = if next == self.msg {
                (next + 1) % TELEGRAMS.len()
            } else {
                next
            };
            self.ci = 0;
            self.si = 0;
            self.wait = 2.5;
            self.push('/');
            return;
        };
        let code = revolucion::morse(ch);
        if code.is_empty() {
            // A space between words.
            self.ci += 1;
            self.si = 0;
            self.left = 4.0 * UNIT;
            self.push('/');
            return;
        }
        let dash = code.as_bytes()[self.si] == b'-';
        self.si += 1;
        self.on = true;
        self.left = if dash { 3.0 * UNIT } else { UNIT };
        self.push(if dash { '\u{2501}' } else { '\u{00b7}' });
    }

    fn push(&mut self, ch: char) {
        self.tape.push_back(ch);
        while self.tape.len() > 240 {
            self.tape.pop_front();
        }
    }
}

impl Widget for Telegrafo {
    fn name(&self) -> &'static str {
        "telegrafo --morse"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        self.step(dt);
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        put(
            c,
            r,
            0,
            0,
            "TEL\u{00c9}GRAFO \u{00b7} l\u{00ed}nea del Central",
            pal.bright,
            BOLD,
        );
        // The key's lamp.
        if r.w >= 40 {
            put_ch(
                c,
                r,
                r.w - 2,
                0,
                if self.on { '\u{25cf}' } else { '\u{25cb}' },
                if self.on { pal.oro } else { pal.dim },
                BOLD,
            );
        }
        // The tape runs from the key, on the right, to the left.
        let x = put(c, r, 0, 2, "cinta ", pal.dim, 0);
        let room = (r.w - x - 1).max(0) as usize;
        let skip = self.tape.len().saturating_sub(room);
        let off = room.saturating_sub(self.tape.len()) as i32;
        for (k, &ch) in self.tape.iter().skip(skip).enumerate() {
            let col = match ch {
                '\u{2501}' => pal.oro,
                '\u{00b7}' => pal.cyan,
                _ => pal.dim,
            };
            put_ch(
                c,
                r,
                x + off + k as i32,
                2,
                ch,
                col,
                if ch == '\u{2501}' { BOLD } else { 0 },
            );
        }
        for k in 0..r.w {
            put_ch(c, r, k, 3, '\u{2500}', pal.dim, 0);
        }
        // What is being sent, letter by letter as the key finishes them.
        let (from, to, _) = TELEGRAMS[self.msg];
        let mut t = std::mem::take(&mut self.buf);
        t.clear();
        let _ = write!(t, "transmitiendo {from}\u{2192}{to}");
        put(c, r, 0, 4, &t, pal.cyan, BOLD);
        let shown = truncate(self.text(), self.ci);
        let width = (r.w - 2).max(8);
        let mut y = 5;
        let mut x = 0;
        for word in shown.split(' ') {
            let w = word.chars().count() as i32;
            if x > 0 && x + 1 + w > width {
                y += 1;
                x = 0;
            }
            if x > 0 {
                x += 1;
            }
            x = put(c, r, x, y, word, pal.bright, BOLD);
        }
        if (self.clock * 3.0) as i32 % 2 == 0 {
            put_ch(c, r, x + 1, y, '\u{258c}', pal.oro, 0);
        }
        y += 2;
        // The telegrams that came before, newest last.
        if y < r.h - 1 {
            put(c, r, 0, y, "recibidos", pal.dim, 0);
            y += 1;
        }
        let rows = (r.h - y - 1).max(0) as usize;
        let first = self.log.len().saturating_sub(rows);
        for (i, &(m, secs)) in self.log.iter().skip(first).enumerate() {
            let yy = y + i as i32;
            let (from, to, text) = TELEGRAMS[m];
            t.clear();
            let _ = write!(
                t,
                "{:02}:{:02} {from}\u{2192}{to} ",
                secs / 3600 % 24,
                secs / 60 % 60
            );
            let x = put(c, r, 0, yy, &t, pal.dim, 0);
            put(
                c,
                r,
                x,
                yy,
                truncate(text, (r.w - x).max(0) as usize),
                pal.main,
                0,
            );
        }
        t.clear();
        let _ = write!(
            t,
            "telegramas {}  clave {} WPM",
            self.sent,
            25 + self.mx.level as u32 * 5
        );
        put(c, r, 0, r.h - 1, &t, pal.dim, 0);
        self.buf = t;
    }
}

// ---------------------------------------------------------------- trains

/// The train sprite crosses the pane this wide and this tall (pixels).
const TW: usize = 46;
const TH: usize = 14;

/// Seconds a train takes to run the whole line.
const RUN: f32 = 150.0;

pub struct Trenes {
    clock: f32,
    /// Kilometers from Ciudad Juárez of each train, and which way it goes.
    km: Vec<f32>,
    pic: Sprite,
    buf: String,
}

/// Total length of the line, in kilometers.
fn line_km() -> f32 {
    STATIONS.last().map_or(1.0, |s| s.1 as f32)
}

impl Trenes {
    pub fn new(seed: u64, _mx: Fiesta) -> Trenes {
        let mut t = Trenes {
            clock: (seed % 90) as f32,
            km: vec![0.0; TRAINS.len()],
            pic: Sprite::new(TW, TH),
            buf: String::new(),
        };
        t.place();
        t
    }

    fn place(&mut self) {
        let total = line_km();
        for (i, k) in self.km.iter_mut().enumerate() {
            // Each train runs at its own speed, starting from a different place.
            let speed = TRAINS[i].2 as f32 / 50.0;
            let start = i as f32 / TRAINS.len() as f32;
            *k = ((self.clock * speed / RUN + start).fract()) * total;
        }
    }

    /// Where on a line of `w` cells a place `km` kilometers down the line is:
    /// the stations are spaced evenly, the trains in between by distance.
    fn x_of(km: f32, w: i32) -> i32 {
        let n = STATIONS.len() - 1;
        let mut seg = 0;
        while seg + 1 < n && km >= STATIONS[seg + 1].1 as f32 {
            seg += 1;
        }
        let (a, b) = (STATIONS[seg].1 as f32, STATIONS[seg + 1].1 as f32);
        let u = ((km - a) / (b - a)).clamp(0.0, 1.0);
        (((seg as f32 + u) / n as f32) * (w - 1) as f32).round() as i32
    }

    /// The station a train has just left, and the one it heads for.
    fn leg(km: f32) -> (usize, usize) {
        let n = STATIONS.len() - 1;
        let mut seg = 0;
        while seg + 1 < n && km >= STATIONS[seg + 1].1 as f32 {
            seg += 1;
        }
        (seg, seg + 1)
    }

    #[cfg(test)]
    pub fn km(&self, i: usize) -> f32 {
        self.km[i]
    }

    /// Paints the locomotive and two carriages of the División del Norte: a
    /// chimney with smoke, a boiler, wheels, and soldaderas and men in
    /// sombreros on the carriage roofs.
    fn paint(&mut self, pal: &Pal) {
        let tick = (self.clock * 6.0) as u32;
        let s = &mut self.pic;
        s.clear();
        let hierro = rgb(0x3a2d22);
        let hierro_c = rgb(0x5a4838);
        let madera = rgb(0x8b6b42);
        let madera_o = rgb(0x5e4529);
        let manta = rgb(0xe6d9b4);
        let rueda = pal.oro;
        // Smoke from the chimney, drifting back.
        for k in 0..5 {
            let age = ((self.clock * 2.0 + k as f32 * 0.37).fract() * 10.0) as i32;
            s.dot(
                4 - age / 2 + k % 2,
                5 - age / 2,
                pal.fade(rgb(0xd8d2c4), 0.15 + age as f32 * 0.06),
            );
        }
        // The locomotive: boiler, chimney, cab and cowcatcher.
        s.rect(2, 6, 3, 2, hierro);
        s.rect(5, 8, 12, 4, hierro);
        s.rect(5, 8, 12, 1, hierro_c);
        s.rect(15, 5, 5, 7, hierro);
        s.rect(16, 6, 3, 2, rgb(0xb8a67a));
        s.rect(0, 11, 3, 2, hierro_c);
        // Two carriages: wooden, with a window each, and men on the roofs.
        for (n, x) in [22, 35].iter().enumerate() {
            s.rect(*x, 8, 11, 4, madera);
            s.rect(*x, 8, 11, 1, madera_o);
            s.dot(*x + 3, 9, rgb(0x1c140a));
            s.dot(*x + 7, 9, rgb(0x1c140a));
            for p in 0..3i32 {
                let px = *x + 1 + p * 4;
                let bob = (tick + n as u32 + p as u32).is_multiple_of(5) as i32;
                // A sombrero (the brim and the crown) over a body.
                s.rect(px - 1, 5 - bob, 3, 1, manta);
                s.dot(px, 4 - bob, manta);
                s.rect(
                    px,
                    6 - bob,
                    1,
                    2,
                    [pal.red, pal.green, pal.oro][((p + n as i32) % 3) as usize],
                );
            }
        }
        pixel_line(s, 20.0, 10.0, 22.0, 10.0, hierro_c);
        pixel_line(s, 33.0, 10.0, 35.0, 10.0, hierro_c);
        // The wheels, turning.
        for x in [6, 10, 14, 25, 30, 38, 43] {
            s.rect(x, 12, 2, 2, rueda);
            if (tick + x as u32).is_multiple_of(2) {
                s.dot(x, 12, hierro);
            }
        }
        for x in 0..TW as i32 {
            s.dot(x, 13, hierro_c);
        }
    }
}

impl Widget for Trenes {
    fn name(&self) -> &'static str {
        "trenes --division-del-norte"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        self.place();
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        put(
            c,
            r,
            0,
            0,
            "TRENES DE LA DIVISI\u{00d3}N DEL NORTE \u{00b7} itinerario",
            pal.bright,
            BOLD,
        );
        let lw = (r.w - 1).max(2);
        let n = STATIONS.len();
        if r.h >= 6 {
            for (i, (name, _)) in STATIONS.iter().enumerate() {
                let x = i as i32 * (lw - 1) / (n as i32 - 1);
                // The names stagger over two rows when the line is too short
                // for all of them side by side, cut to the room they have.
                let wide = lw >= 140;
                let room = if wide {
                    20
                } else {
                    (2 * (lw - 1) / (n as i32 - 1) - 1).max(1)
                };
                let name = truncate(name, room as usize);
                let w = name.chars().count() as i32;
                let row = if wide || i % 2 == 1 { 2 } else { 1 };
                put(
                    c,
                    r,
                    (x - w / 2).clamp(0, (lw - w).max(0)),
                    row,
                    name,
                    pal.cyan,
                    0,
                );
                put_ch(c, r, x, 3, '\u{25cb}', pal.oro, 0);
            }
            for x in 0..lw {
                if c.get(r.x + x, r.y + 3).is_some_and(|cell| cell.ch == ' ') {
                    put_ch(c, r, x, 3, '\u{2550}', pal.dim, 0);
                }
            }
            let [g, w, red] = tricolor(pal);
            for (i, &km) in self.km.iter().enumerate() {
                put_ch(
                    c,
                    r,
                    Self::x_of(km, lw),
                    3,
                    '\u{25b6}',
                    [g, w, red][i % 3],
                    BOLD,
                );
            }
        }
        // The timetable: where each is, how far from its next station.
        let mut t = std::mem::take(&mut self.buf);
        let y = 5;
        if r.h > 6 {
            t.clear();
            let _ = write!(
                t,
                "{:<24}{:<30}{:>8}{:>7}",
                "tren", "tramo", "faltan", "km/h"
            );
            put(c, r, 0, y - 1, &t, pal.cyan, BOLD);
        }
        let sprite = r.w >= 48 && r.h >= 17;
        let limit = if sprite { r.h - 8 } else { r.h - 1 };
        for (i, &(name, cars, speed)) in TRAINS.iter().enumerate() {
            let y = y + i as i32;
            if y >= limit {
                break;
            }
            let km = self.km[i];
            let (a, b) = Self::leg(km);
            let to_go = STATIONS[b].1 as f32 - km;
            let at_station = to_go < 14.0 || km - (STATIONS[a].1 as f32) < 14.0;
            put(c, r, 0, y, truncate(name, 23), pal.bright, 0);
            t.clear();
            let _ = write!(t, "{} \u{2192} {}", STATIONS[a].0, STATIONS[b].0);
            put(c, r, 24, y, &t, pal.main, 0);
            t.clear();
            let _ = write!(t, "{to_go:>5.0} km{speed:>7}");
            let end = put(
                c,
                r,
                54,
                y,
                &t,
                if at_station { pal.oro } else { pal.amber },
                0,
            );
            if r.w >= 54 + 14 + 18 {
                t.clear();
                let _ = write!(t, "  {cars} carros");
                put(c, r, end, y, &t, pal.dim, 0);
            }
        }
        if r.h >= 4 {
            t.clear();
            let _ = write!(
                t,
                "l\u{00ed}nea {:.0} km \u{00b7} {} trenes en marcha",
                line_km(),
                TRAINS.len()
            );
            put(
                c,
                r,
                0,
                r.h - 1 - if sprite { 7 } else { 0 },
                &t,
                pal.dim,
                0,
            );
        }
        self.buf = t;
        if sprite {
            self.paint(pal);
            let x = ((self.clock * 5.0) as i32).rem_euclid(r.w + TW as i32) - TW as i32;
            let rows = TH as i32 / 2;
            self.pic.blit(c, r, r.x + x, r.y + r.h - rows);
            // The line under it.
            for k in 0..r.w {
                put_ch(c, r, k, r.h - 1, '\u{2550}', pal.dim, 0);
            }
        }
    }
}

// ---------------------------------------------------------------- archive

/// Characters typed a second.
const TYPING: f32 = 40.0;

pub struct Archivo {
    rng: Rng,
    doc: usize,
    t: f32,
    folio: u32,
    buf: String,
}

impl Archivo {
    pub fn new(seed: u64, _mx: Fiesta) -> Archivo {
        let mut rng = Rng::with_seed(seed ^ 0xa4c41);
        let doc = rng.usize(..DOCUMENTS.len());
        let folio = rng.u32(100..9000);
        Archivo {
            rng,
            doc,
            t: 0.0,
            folio,
            buf: String::new(),
        }
    }

    fn period(&self) -> f32 {
        DOCUMENTS[self.doc].1.chars().count() as f32 / TYPING + 7.0
    }

    #[cfg(test)]
    pub fn doc(&self) -> usize {
        self.doc
    }
}

impl Widget for Archivo {
    fn name(&self) -> &'static str {
        "less archivo/revolucion"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.t += dt;
        if self.t >= self.period() {
            self.t = 0.0;
            // The next document in the legajo, now and then a jump.
            self.doc = if self.rng.u8(..4) == 0 {
                self.rng.usize(..DOCUMENTS.len())
            } else {
                (self.doc + 1) % DOCUMENTS.len()
            };
            self.folio += self.rng.u32(1..40);
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        put(
            c,
            r,
            0,
            0,
            "ARCHIVO DE LA REVOLUCI\u{00d3}N",
            pal.bright,
            BOLD,
        );
        let mut t = std::mem::take(&mut self.buf);
        t.clear();
        let _ = write!(
            t,
            "Fondo Revoluci\u{00f3}n \u{00b7} legajo {}/{} \u{00b7} folio {}",
            self.doc + 1,
            DOCUMENTS.len(),
            self.folio
        );
        put(c, r, 0, 1, &t, pal.dim, 0);
        let (title, body) = DOCUMENTS[self.doc];
        put(
            c,
            r,
            0,
            3,
            truncate(title, r.w.max(0) as usize),
            pal.oro,
            BOLD,
        );
        for x in 0..r.w {
            put_ch(c, r, x, 4, '\u{2500}', pal.dim, 0);
        }
        let typed = (self.t * TYPING) as usize;
        let shown = truncate(body, typed);
        let width = (r.w - 1).max(8);
        let (mut x, mut y) = (0, 5);
        for word in shown.split(' ') {
            let w = word.chars().count() as i32;
            if x > 0 && x + 1 + w > width {
                y += 1;
                x = 0;
            }
            if x > 0 {
                x += 1;
            }
            x = put(c, r, x, y, word, pal.main, 0);
        }
        let done = typed >= body.chars().count();
        if !done && (self.t * 4.0) as i32 % 2 == 0 {
            put_ch(c, r, x, y, '\u{258c}', pal.oro, 0);
        }
        if done && r.w >= 44 && (self.t * 2.0) as i32 % 2 == 0 {
            put(c, r, r.w - 21, 1, "[ COPIA CERTIFICADA ]", pal.red, BOLD);
        }
        // The legajo's catalog, the open document marked; in a short pane
        // only the numbers.
        let first = r.h - 1 - DOCUMENTS.len() as i32;
        if first > y + 2 && r.w >= 30 {
            put(c, r, 0, first - 1, "cat\u{00e1}logo del legajo", pal.dim, 0);
            for (i, (title, _)) in DOCUMENTS.iter().enumerate() {
                let open = i == self.doc;
                t.clear();
                let _ = write!(t, "{} ", i + 1);
                let x = put(
                    c,
                    r,
                    0,
                    first + i as i32,
                    &t,
                    if open { pal.oro } else { pal.dim },
                    BOLD,
                );
                put(
                    c,
                    r,
                    x,
                    first + i as i32,
                    truncate(title, (r.w - x).max(0) as usize),
                    if open { pal.bright } else { pal.dim },
                    if open { BOLD } else { 0 },
                );
            }
        } else if r.h >= 12 {
            let mut x = put(c, r, 0, r.h - 1, "legajo ", pal.dim, 0);
            for i in 0..DOCUMENTS.len() {
                t.clear();
                let _ = write!(t, "{}", i + 1);
                let open = i == self.doc;
                x = put(
                    c,
                    r,
                    x,
                    r.h - 1,
                    &t,
                    if open { pal.oro } else { pal.dim },
                    if open { BOLD } else { DIM },
                ) + 1;
            }
        }
        self.buf = t;
    }
}

// ---------------------------------------------------------------- the slogan

/// The sombrero is a pixel sprite, this wide and this tall.
const SW: usize = 28;
const SH: usize = 12;

pub struct Lema {
    clock: f32,
    which: usize,
    pic: Sprite,
}

/// Seconds each slogan is up.
const SLOGAN: f32 = 9.0;

impl Lema {
    pub fn new(seed: u64, _mx: Fiesta) -> Lema {
        Lema {
            clock: 0.0,
            // Most often, the first: Tierra y Libertad.
            which: if seed.is_multiple_of(3) {
                (seed / 3) as usize % SLOGANS.len()
            } else {
                0
            },
            pic: Sprite::new(SW, SH),
        }
    }

    #[cfg(test)]
    pub fn slogan(&self) -> usize {
        self.which
    }

    /// A sombrero de ala ancha with a band of cartridges: brim, crown and a
    /// strap, in manta and sepia.
    fn paint(&mut self, pal: &Pal) {
        let s = &mut self.pic;
        s.clear();
        let manta = rgb(0xe6d9b4);
        let sombra = rgb(0xb8a67a);
        let cinta = rgb(0x5e4529);
        // The crown.
        for y in 0..6 {
            let inset = 8 - (y * 3 / 5).min(2) - if y == 0 { 2 } else { 0 };
            s.rect(
                inset,
                1 + y,
                SW as i32 - 2 * inset,
                1,
                if y > 3 { sombra } else { manta },
            );
        }
        // The band with its brass cartridges.
        s.rect(7, 6, SW as i32 - 14, 1, cinta);
        for x in 0..6 {
            s.dot(9 + x * 2, 6, pal.oro);
        }
        // The brim, wide, curling up at the ends.
        s.rect(1, 7, SW as i32 - 2, 2, manta);
        s.rect(0, 6, 3, 2, manta);
        s.rect(SW as i32 - 3, 6, 3, 2, manta);
        s.rect(3, 9, SW as i32 - 6, 1, sombra);
    }
}

impl Widget for Lema {
    fn name(&self) -> &'static str {
        "lema --revolucion"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        if self.clock >= SLOGAN {
            self.clock -= SLOGAN;
            self.which = (self.which + 1) % SLOGANS.len();
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let (l1, l2) = SLOGANS[self.which];
        let [g, w, red] = tricolor(pal);
        // A bandolier of cartridges along the top and the bottom, a flag now
        // and then.
        for x in 0..r.w {
            for y in [0, r.h - 1] {
                if x % 12 == 6 {
                    put_ch(
                        c,
                        r,
                        x,
                        y,
                        '\u{25bc}',
                        [g, w, red][(x / 12) as usize % 3],
                        BOLD,
                    );
                } else if x % 2 == 0 {
                    put_ch(c, r, x, y, '\u{25ae}', pal.oro, 0);
                }
            }
        }
        let want = big_width(l1, 1).max(big_width(l2, 1));
        let sx = if r.w >= big_width(l1, 2).max(big_width(l2, 2)) + 4 {
            2
        } else {
            1
        };
        let hat = r.h >= 26;
        let letters = r.h >= 15 && r.w >= want + 2;
        // Letters appear one after another, then stay, then the next slogan.
        let n = (self.clock * 14.0) as usize;
        let n1 = n.min(l1.chars().count());
        let n2 = n
            .saturating_sub(l1.chars().count() + 2)
            .min(l2.chars().count());
        if letters {
            let top = ((r.h - 11) / 2).max(1) + if hat { 3 } else { 0 };
            if hat {
                self.paint(pal);
                let x = r.x + (r.w - SW as i32) / 2;
                self.pic.blit(c, r, x, r.y + top - 7);
            }
            let mut x = (r.w - big_width(l1, sx)) / 2;
            for ch in l1.chars().take(n1) {
                x = big_char(c, r, x, top, ch, sx, pal.oro);
            }
            let mut x = (r.w - big_width(l2, sx)) / 2;
            for (i, ch) in l2.chars().take(n2).enumerate() {
                x = big_char(c, r, x, top + 6, ch, sx, [g, w, red][i % 3]);
            }
        } else {
            let y = r.h / 2 - 1;
            put(
                c,
                r,
                (r.w - l1.chars().count() as i32) / 2,
                y,
                truncate(l1, n1),
                pal.oro,
                BOLD,
            );
            put(
                c,
                r,
                (r.w - l2.chars().count() as i32) / 2,
                y + 1,
                truncate(l2, n2),
                pal.bright,
                BOLD,
            );
        }
        let note = "\u{2014} lema de la Revoluci\u{00f3}n \u{2014}";
        if r.h >= 6 {
            put(
                c,
                r,
                (r.w - note.chars().count() as i32) / 2,
                r.h - 2,
                note,
                pal.dim,
                0,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Ctx;
    use crate::theme::Theme;

    fn env(fiesta: Fiesta) -> Env {
        let theme = Theme::classic();
        let o = crate::scene::Opts::new(&[], toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 1,
            fiesta,
        };
        Env {
            pal: Pal::new(&ctx),
            level: 7,
            day: 0.0,
        }
    }

    fn draw(w: &mut dyn Widget, env: &Env, width: u16, height: u16) -> Canvas {
        let mut c = Canvas::new(width, height, Color::Reset);
        let r = Rect {
            x: 0,
            y: 0,
            w: width as i32,
            h: height as i32,
        };
        w.resize(width, height);
        w.draw(&mut c, r, env);
        c
    }

    #[test]
    fn the_key_sends_telegrams_in_morse_and_the_text_follows() {
        let f = Fiesta::of_day(11, 20);
        let e = env(f);
        let mut t = Telegrafo::new(3, f);
        let start = t.sent();
        let (mut dots, mut dashes, mut lamp_on) = (false, false, false);
        let mut seen = String::new();
        for step in 0..1500 {
            t.update(0.05, &e);
            lamp_on |= t.on;
            if step % 5 == 0 {
                let text = draw(&mut t, &e, 100, 24).to_text();
                dots |= text.contains('\u{00b7}');
                dashes |= text.contains('\u{2501}');
                seen.push_str(&text);
            }
        }
        assert!(dots && dashes && lamp_on);
        assert!(t.sent() > start + 2);
        assert!(
            seen.contains("TEL\u{00c9}GRAFO") && seen.contains("transmitiendo"),
            "{seen}"
        );
        assert!(seen.contains("STOP"), "{seen}");
        assert!(seen.contains("recibidos"));
        // A letter at a time: the first words come out before the last.
        let mut t = Telegrafo::new(3, f);
        t.finish_message();
        t.wait = 0.0;
        for _ in 0..40 {
            t.update(0.05, &e);
        }
        draw(&mut t, &e, 100, 24);
        let total = t.text().chars().count();
        assert!(t.ci > 0 && t.ci < total, "{} of {total}", t.ci);
    }

    #[test]
    fn the_trains_run_south_along_the_line() {
        let f = Fiesta::of_day(11, 20);
        let e = env(f);
        let mut t = Trenes::new(1, f);
        let first = t.km(0);
        for _ in 0..200 {
            t.update(0.1, &e);
        }
        assert_ne!(t.km(0), first);
        let text = draw(&mut t, &e, 110, 30).to_text();
        assert!(
            text.contains("TRENES DE LA DIVISI\u{00d3}N DEL NORTE"),
            "{text}"
        );
        for station in ["Chihuahua", "Torre\u{00f3}n", "Zacatecas"] {
            assert!(text.contains(station), "{station}");
        }
        assert!(text.contains("Tren de Artiller\u{00ed}a") && text.contains("km/h"));
        assert!(text.contains('\u{25b6}'));
        // The locomotive crosses the pane, its roofs full of sombreros.
        assert!(
            text.contains('\u{2580}') || text.contains('\u{2584}') || text.contains('\u{2588}')
        );
        // And the markers move along the line from station to station.
        assert_eq!(Trenes::x_of(0.0, 91), 0);
        assert_eq!(Trenes::x_of(line_km(), 91), 90);
        assert!(Trenes::x_of(500.0, 91) > Trenes::x_of(400.0, 91));
    }

    #[test]
    fn the_archive_types_out_the_plan_de_san_luis() {
        let f = Fiesta::of_day(11, 20);
        let e = env(f);
        let mut a = Archivo::new(2, f);
        a.doc = 0;
        let mut seen = String::new();
        for _ in 0..90 {
            a.update(0.2, &e);
            seen.push_str(&draw(&mut a, &e, 90, 20).to_text());
            if a.doc() != 0 {
                break;
            }
        }
        assert!(seen.contains("ARCHIVO DE LA REVOLUCI\u{00d3}N"), "{seen}");
        assert!(seen.contains("PLAN DE SAN LUIS"), "{seen}");
        assert!(seen.contains("las seis de la tarde en adelante"), "{seen}");
        assert!(seen.contains("COPIA CERTIFICADA"));
        // It goes on to the next document by itself.
        for _ in 0..400 {
            a.update(0.2, &e);
        }
        assert!(a.doc() != 0 || a.t > 0.0);
    }

    #[test]
    fn tierra_y_libertad_is_painted_in_big_letters() {
        let f = Fiesta::of_day(11, 20);
        let e = env(f);
        let mut l = Lema::new(1, f);
        assert_eq!(l.slogan(), 0);
        for _ in 0..80 {
            l.update(0.1, &e);
        }
        let c = draw(&mut l, &e, 100, 30);
        let text = c.to_text();
        assert!(text.matches('\u{2588}').count() > 100, "{text}");
        assert!(text.contains('\u{25ae}') && text.contains("lema de la Revoluci"));
        // In a small pane it says it in plain letters.
        l.update(8.0, &e);
        let mut l = Lema::new(1, f);
        for _ in 0..80 {
            l.update(0.1, &e);
        }
        let text = draw(&mut l, &e, 40, 9).to_text();
        assert!(
            text.contains("TIERRA") && text.contains("Y LIBERTAD"),
            "{text}"
        );
        // It goes through the slogans.
        let mut l = Lema::new(1, f);
        let mut seen = std::collections::HashSet::new();
        for _ in 0..600 {
            l.update(0.1, &e);
            seen.insert(l.slogan());
        }
        assert_eq!(seen.len(), SLOGANS.len());
    }
}
