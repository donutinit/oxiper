//! The widgets only Día de Muertos has: the gateway to the Mictlán, where
//! souls cross the nine levels of the underworld one trial at a time, and the
//! ofrenda database, a `psql` session over the altar with the altar itself
//! beside it, its candles flickering. The higher the holiday, the more souls
//! are crossing, the more candles are lit and the fuller the altar.

use super::sprite::Sprite;
use super::{Env, Pal, Rect, Widget, muertos, put, put_ch};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Canvas, DIM};
use fastrand::Rng;
use std::collections::VecDeque;
use std::fmt::Write;

/// The nine levels of the Mictlán: (name, trial, what the soul does there).
const LEVELS: [(&str, &str, &str); 9] = [
    (
        "Itzcuintl\u{00e1}n",
        "R\u{00ed}o Apanohuaya",
        "cruza el r\u{00ed}o Apanohuaya a lomo de xolo",
    ),
    (
        "Tepeme Monamictl\u{00e1}n",
        "Cerros que chocan",
        "pasa entre los cerros que chocan",
    ),
    (
        "Izt\u{00e9}petl",
        "Cerro de obsidiana",
        "sube el cerro de obsidiana",
    ),
    (
        "Itzehec\u{00e1}yan",
        "Viento de navajas",
        "atraviesa el viento de navajas",
    ),
    (
        "Paniecatacoyan",
        "Banderas",
        "cruza las banderas de papel picado",
    ),
    (
        "Timiminaloyan",
        "Lluvia de flechas",
        "esquiva la lluvia de flechas",
    ),
    (
        "Teocoyocualloyan",
        "Fieras",
        "convence a las fieras con pan de muerto",
    ),
    (
        "Izmictl\u{00e1}n Apochcalolca",
        "Niebla de obsidiana",
        "cruza la niebla de obsidiana",
    ),
    (
        "Chicunamictl\u{00e1}n",
        "Descanso",
        "llega al descanso: paz y buen pan",
    ),
];

/// Glyphs for each level's path: the river, the hills, the flags...
const LANE: [char; 9] = [
    '~', '^', '\u{25b2}', '\u{2248}', '\u{25bc}', '\u{00bb}', 'v', '\u{2591}', '\u{00b7}',
];

fn truncate(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

// ---------------------------------------------------------------- gateway

struct Soul {
    name: &'static str,
    /// Level the soul is crossing (0 to 8) and how far along it is (0 to 1).
    level: u8,
    t: f32,
    /// Levels per second.
    speed: f32,
}

pub struct Gateway {
    mx: Fiesta,
    rng: Rng,
    souls: Vec<Soul>,
    log: VecDeque<String>,
    spawn: f32,
    clock: f32,
    /// Souls that reached the last level.
    home: u32,
    /// Where the lines of text are written, reused every frame.
    buf: String,
}

impl Gateway {
    pub fn new(seed: u64, mx: Fiesta) -> Gateway {
        let mut g = Gateway {
            mx,
            rng: Rng::with_seed(seed ^ 0x9a7e),
            souls: Vec::new(),
            log: VecDeque::new(),
            spawn: 1.0,
            clock: 0.0,
            home: 100 + (seed % 900) as u32,
            buf: String::new(),
        };
        for _ in 0..g.want() {
            let mut s = g.soul();
            s.level = g.rng.u8(..9);
            s.t = g.rng.f32();
            // Where each one has been already.
            if s.level > 0 {
                g.say(s.name, s.level as usize - 1);
            }
            g.souls.push(s);
        }
        g
    }

    /// How many souls cross at once: more the higher the holiday.
    fn want(&self) -> usize {
        3 + (7.0 * self.mx.level.clamp(0.0, 1.0)) as usize
    }

    fn soul(&mut self) -> Soul {
        Soul {
            name: muertos::soul(&mut self.rng, self.mx.level),
            level: 0,
            t: 0.0,
            speed: 0.12 + self.rng.f32() * 0.16,
        }
    }

    fn say(&mut self, name: &str, level: usize) {
        let mut line = String::new();
        let _ = write!(line, "{name} {}", LEVELS[level].2);
        self.log.push_back(line);
        while self.log.len() > 8 {
            self.log.pop_front();
        }
    }

    #[cfg(test)]
    pub fn crossing(&self) -> usize {
        self.souls.len()
    }
}

impl Widget for Gateway {
    fn name(&self) -> &'static str {
        "mictlan-gw --online"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        let mut done: Vec<(&'static str, usize)> = Vec::new();
        for s in self.souls.iter_mut() {
            s.t += s.speed * dt;
            while s.t >= 1.0 && s.level < 9 {
                s.t -= 1.0;
                done.push((s.name, s.level as usize));
                s.level += 1;
            }
        }
        for (name, level) in done {
            if level == 8 {
                self.home += 1;
            }
            self.say(name, level);
        }
        self.souls.retain(|s| s.level < 9);
        self.spawn -= dt;
        if self.spawn <= 0.0 {
            self.spawn = 0.5 + self.rng.f32() * 1.8;
            if self.souls.len() < self.want() {
                let s = self.soul();
                self.souls.push(s);
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        // Header: the pilot light flickers like a candle.
        let flick = ((self.clock * 9.0).sin() * 0.5 + 0.5) * ((self.clock * 5.3).sin() * 0.5 + 0.5);
        let lamp = pal.flor.lerp(pal.vela, flick);
        put(c, r, 0, 0, "\u{25cf}", lamp, BOLD);
        put(
            c,
            r,
            2,
            0,
            "MICTL\u{00c1}N GATEWAY ONLINE",
            pal.bright,
            BOLD,
        );
        let mut t = std::mem::take(&mut self.buf);
        t.clear();
        let _ = write!(t, "{} en camino", self.souls.len());
        let tw = t.chars().count() as i32;
        if r.w >= 24 + tw + 2 {
            put(c, r, r.w - tw, 0, &t, pal.cyan, 0);
        }
        let xolos = 9 - (self.souls.len() / 4).min(5);
        t.clear();
        let _ = write!(
            t,
            "descansan {}   xolos {}/9   nivel 9: abierto",
            self.home, xolos
        );
        put(c, r, 0, 1, &t, pal.dim, 0);
        self.buf = t;
        // The nine levels, one lane each, as many as fit.
        let avail = (r.h - 2).max(0);
        let log_rows = if avail >= 11 { (avail - 10).min(8) } else { 0 };
        let shown = (avail - log_rows).clamp(0, 9);
        let nw = if r.w >= 54 {
            22
        } else if r.w >= 36 {
            16
        } else {
            0
        };
        let tail = if r.w >= 78 { 22 } else { 0 };
        let lx = 2 + nw + if nw > 0 { 1 } else { 0 };
        let lw = (r.w - lx - tail - 1).max(0);
        for i in 0..shown {
            let y = 2 + i;
            let here = self.souls.iter().any(|s| s.level as i32 == i);
            let (num, name) = if here {
                (pal.bright, pal.main)
            } else {
                (pal.dim, pal.dim)
            };
            put_ch(c, r, 0, y, char::from(b'1' + i as u8), num, BOLD);
            if nw > 0 {
                put(
                    c,
                    r,
                    2,
                    y,
                    truncate(LEVELS[i as usize].0, nw as usize),
                    name,
                    0,
                );
            }
            if lw > 2 {
                // The path: left to right on one level, back on the next.
                let glyph = LANE[i as usize];
                let rtl = i % 2 == 1;
                for k in 0..lw {
                    let (ch, col) = if i == 7 {
                        (glyph, pal.dim)
                    } else if k % 2 == 0 {
                        let col = match i {
                            0 => pal.cyan,
                            2 => pal.picado[2],
                            4 => pal.picado[(k as usize / 2) % 6],
                            5 => pal.amber,
                            6 => pal.red,
                            8 => pal.flor,
                            _ => pal.dim,
                        };
                        (glyph, if here { col } else { pal.fade(col, 0.55) })
                    } else {
                        continue;
                    };
                    put_ch(c, r, lx + k, y, ch, col, 0);
                }
                put_ch(
                    c,
                    r,
                    if rtl { lx - 1 } else { lx + lw },
                    y,
                    if rtl { '\u{258f}' } else { '\u{2595}' },
                    pal.dim,
                    0,
                );
                for s in self.souls.iter().filter(|s| s.level as i32 == i) {
                    let f = if rtl { 1.0 - s.t } else { s.t };
                    let x = lx + (f * (lw - 1) as f32) as i32;
                    put_ch(c, r, x, y, '\u{273f}', pal.flor.scale(1.2), BOLD);
                    // A faint trail behind it.
                    let dx = if rtl { 1 } else { -1 };
                    put_ch(c, r, x + dx, y, '\u{2022}', pal.fade(pal.flor, 0.45), 0);
                }
            }
            if tail > 0 {
                put(c, r, r.w - tail + 1, y, LEVELS[i as usize].1, pal.dim, 0);
            }
        }
        for (k, line) in self.log.iter().rev().take(log_rows as usize).enumerate() {
            let y = r.h - 1 - k as i32;
            put_ch(c, r, 0, y, '\u{25b8}', pal.flor, BOLD);
            put(
                c,
                r,
                2,
                y,
                line,
                if k == 0 { pal.bright } else { pal.main },
                if k == 0 { 0 } else { DIM },
            );
        }
    }
}

// ---------------------------------------------------------------- ofrenda

/// The altar beside the table is a pixel sprite (half blocks): this wide and
/// this tall, in pixels.
const AW: usize = 25;
const AH: usize = 24;
/// Cells it takes, with a column of air.
const ALTAR_W: i32 = AW as i32 + 1;

struct Row {
    id: u16,
    soul: &'static str,
    nivel: u8,
    item: &'static str,
    velas: u8,
    /// 0 pendiente, 1 en camino, 2 recibida.
    estado: u8,
    flash: f32,
}

pub struct Ofrenda {
    mx: Fiesta,
    rng: Rng,
    rows: VecDeque<Row>,
    next_id: u16,
    timer: f32,
    clock: f32,
    /// The altar, repainted every frame.
    altar: Sprite,
    /// Where the lines of text are written, reused every frame.
    buf: String,
    /// Totals for the footer.
    panes: u32,
    velas: u32,
    kilos: f32,
}

impl Ofrenda {
    pub fn new(seed: u64, mx: Fiesta) -> Ofrenda {
        let mut o = Ofrenda {
            mx,
            rng: Rng::with_seed(seed ^ 0x0f7e),
            rows: VecDeque::new(),
            next_id: 1000,
            timer: 0.5,
            clock: 0.0,
            altar: Sprite::new(AW, AH),
            buf: String::new(),
            panes: 1200,
            velas: 3500,
            kilos: 410.0,
        };
        o.next_id += o.rng.u16(..500);
        for _ in 0..40 {
            o.add_row();
            if let Some(r) = o.rows.back_mut() {
                r.flash = 0.0;
                r.estado = o.rng.u8(..3);
            }
        }
        o
    }

    fn add_row(&mut self) {
        // Not a name that is already on the screen, if it can be helped.
        let mut soul = muertos::soul(&mut self.rng, self.mx.level);
        for _ in 0..8 {
            if !self.rows.iter().rev().take(24).any(|r| r.soul == soul) {
                break;
            }
            soul = muertos::soul(&mut self.rng, self.mx.level);
        }
        let row = Row {
            id: self.next_id,
            soul,
            nivel: self.rng.u8(1..10),
            item: muertos::offering(&mut self.rng, soul),
            velas: self.rng.u8(1..9),
            estado: 0,
            flash: 1.0,
        };
        self.next_id = self.next_id.wrapping_add(1);
        self.rows.push_back(row);
        while self.rows.len() > 40 {
            self.rows.pop_front();
        }
    }
}

impl Widget for Ofrenda {
    fn name(&self) -> &'static str {
        "psql ofrenda"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        for r in self.rows.iter_mut() {
            r.flash = (r.flash - dt * 0.8).max(0.0);
        }
        self.timer -= dt;
        if self.timer > 0.0 {
            return;
        }
        // Faster the higher the holiday.
        self.timer = (0.4 + self.rng.f32() * 1.4) * (1.5 - 0.7 * self.mx.level);
        let n = self.rows.len();
        match self.rng.u8(..10) {
            0..=3 if n > 0 => {
                // An offering moves on: pending, on its way, received.
                let i = self.rng.usize(..n);
                let row = &mut self.rows[i];
                if row.estado < 2 {
                    row.estado += 1;
                    row.flash = 1.0;
                    if row.estado == 2 {
                        self.panes += self.rng.u32(1..4);
                    }
                }
            }
            4..=6 if n > 0 => {
                let i = self.rng.usize(..n);
                let row = &mut self.rows[i];
                row.velas = row.velas.saturating_add(1).min(99);
                row.flash = 1.0;
                self.velas += 1;
            }
            _ => {
                self.add_row();
                self.kilos += self.rng.f32() * 3.0;
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let altar = r.w >= 68 && r.h >= 16;
        let tw = if altar { r.w - ALTAR_W - 1 } else { r.w };
        let table = Rect { w: tw, ..r };
        // The query, with its cursor.
        let cursor = if (self.clock * 2.0) as i32 % 2 == 0 {
            "\u{258c}"
        } else {
            " "
        };
        // The query follows the days: the first petals, the forgotten souls
        // nobody visits, the angelitos, everyone remembered.
        let level = self.mx.level;
        let q = if tw < 50 {
            "ofrenda=# SELECT * FROM altar;"
        } else if level < 0.4 {
            "ofrenda=# SELECT * FROM altar ORDER BY nivel;"
        } else if level < 0.55 {
            "ofrenda=# SELECT * FROM animas ORDER BY visitas;"
        } else if level < 0.9 {
            "ofrenda=# SELECT * FROM altar ORDER BY edad;"
        } else {
            "ofrenda=# SELECT * FROM altar WHERE recordado;"
        };
        let x = put(c, table, 0, 0, q, pal.bright, BOLD);
        put(c, table, x + 1, 0, cursor, pal.flor, 0);
        // Columns by the room there is: id and name always, the offering when
        // there are 8 cells for it, the level and the candles when wide.
        let wide = tw >= 64;
        let item_w = if wide {
            20
        } else {
            (tw - 21 - 1 - 10).clamp(0, 20) as usize
        };
        let mut line = std::mem::take(&mut self.buf);
        line.clear();
        if wide {
            let _ = write!(
                line,
                "{:>5}  {:<13} {:>3}  {:<20} {:>5}  estado",
                "id", "nombre", "niv", "ofrenda", "velas"
            );
        } else if item_w >= 8 {
            let _ = write!(
                line,
                "{:>5}  {:<13} {:<item_w$} estado",
                "id", "nombre", "ofrenda"
            );
        } else {
            let _ = write!(line, "{:>5}  {:<13} estado", "id", "nombre");
        }
        put(c, table, 0, 1, &line, pal.cyan, BOLD);
        for x in 0..tw {
            put_ch(c, table, x, 2, '\u{2500}', pal.dim, 0);
        }
        let rows = (r.h - 4).max(0) as usize;
        for (i, row) in self.rows.iter().rev().take(rows).enumerate() {
            let y = r.h - 2 - i as i32;
            line.clear();
            let name = truncate(row.soul, 13);
            let item = row.item;
            let _ = write!(line, "{:>5}  {name:<13} ", row.id);
            if wide {
                let _ = write!(
                    line,
                    "{:>3}  {:<20} {:>5}  ",
                    row.nivel,
                    truncate(item, 20),
                    row.velas
                );
            } else if item_w >= 8 {
                let _ = write!(line, "{:<item_w$} ", truncate(item, item_w));
            }
            let hot = row.flash > 0.05;
            let base = if hot { pal.bright } else { pal.main };
            let x = put(c, table, 0, y, &line, base, if hot { BOLD } else { 0 });
            let (text, col) = match row.estado {
                0 => ("PENDIENTE", pal.amber),
                1 => ("EN CAMINO", pal.cyan),
                _ => ("RECIBIDA \u{2713}", pal.green),
            };
            put(
                c,
                table,
                x,
                y,
                text,
                col,
                if row.estado == 2 { BOLD } else { 0 },
            );
        }
        line.clear();
        let _ = write!(
            line,
            "({} filas)  pan {}  veladoras {}  cempas\u{00fa}chil {:.0} kg",
            self.rows.len(),
            self.panes,
            self.velas,
            self.kilos
        );
        put(c, table, 0, r.h - 1, &line, pal.dim, 0);
        self.buf = line;
        if altar {
            self.draw_altar(c, r, env);
        }
    }
}

impl Ofrenda {
    /// Paints the three-tiered altar: papel picado across the top, the
    /// photograph and candles on the highest tier, marigolds, pan de muerto
    /// and candles on the next, sugar skulls on the lowest. More candles are
    /// lit the higher the holiday.
    fn paint_altar(&mut self, pal: &Pal) {
        let tick = (self.clock * 8.0) as u32;
        let lit = 2 + (6.0 * self.mx.level).round() as usize;
        let s = &mut self.altar;
        s.clear();
        let hueso = rgb(0xf4eee2);
        let sepia = rgb(0xb58a5a);
        let skin = rgb(0xe8c9a0);
        let pelo = rgb(0x2a1a14);
        let pan = rgb(0xd9a05b);
        let pan_d = rgb(0xa8702f);
        let tallo = rgb(0x78b42a);
        let (naranja, oscuro, amarillo) = (rgb(0xff8c00), rgb(0xd66200), rgb(0xffd200));
        // Papel picado: a cord and six flags, each cut with a hole.
        s.rect(0, 0, AW as i32, 1, pal.fade(pal.main, 0.4));
        for k in 0..6 {
            let x = 1 + 4 * k;
            let col = pal.picado[(k as usize + tick as usize / 24) % 6];
            s.rect(x, 1, 3, 2, col);
            s.dot(x + 1, 2, Color::Reset);
            s.dot(x, 3, col);
            s.dot(x + 2, 3, col);
        }
        // The highest tier: the photograph.
        s.rect(9, 4, 7, 6, hueso);
        s.rect(10, 5, 5, 4, sepia);
        s.rect(11, 5, 3, 1, pelo);
        s.rect(11, 6, 3, 2, skin);
        s.dot(12, 7, pelo);
        s.rect(10, 8, 5, 1, pal.picado[2]);
        s.rect(5, 10, 15, 1, pal.picado[0]);
        s.rect(5, 11, 15, 1, pal.picado[2]);
        // The middle tier: marigolds and bread.
        let flower = |s: &mut Sprite, x: i32, y: i32| {
            s.rect(x, y, 4, 3, naranja);
            s.dot(x, y, Color::Reset);
            s.dot(x + 3, y, Color::Reset);
            s.dot(x + 1, y + 1, amarillo);
            s.dot(x + 2, y + 1, oscuro);
            s.dot(x + 1, y + 3, tallo);
        };
        flower(s, 5, 12);
        flower(s, 15, 12);
        for x in [10, 14] {
            s.rect(x, 14, 4, 2, pan);
            s.rect(x + 1, 13, 2, 1, pan);
            s.dot(x + 1, 14, pan_d);
            s.dot(x + 2, 14, pan_d);
            s.dot(x + 1, 15, pan_d);
        }
        s.rect(1, 16, 23, 1, pal.picado[4]);
        s.rect(1, 17, 23, 1, pal.picado[3]);
        // The lowest tier: sugar skulls with marigold eyes and the garland.
        for x in [5, 15] {
            // ".WRW." / "WOWOW" / ".WkW." / ".kWk.": icing, flower sockets, nose, teeth.
            s.rect(x, 18, 5, 4, hueso);
            s.dot(x, 18, Color::Reset);
            s.dot(x + 4, 18, Color::Reset);
            s.dot(x + 2, 18, pal.picado[0]);
            s.dot(x + 1, 19, naranja);
            s.dot(x + 3, 19, naranja);
            s.dot(x + 2, 20, pelo);
            s.dot(x, 20, Color::Reset);
            s.dot(x + 4, 20, Color::Reset);
            s.dot(x, 21, Color::Reset);
            s.dot(x + 4, 21, Color::Reset);
            s.dot(x + 1, 21, pelo);
            s.dot(x + 3, 21, pelo);
        }
        for x in 0..AW as i32 {
            let col = [naranja, amarillo, oscuro, pal.picado[0]][(x as usize / 2) % 4];
            s.dot(x, 22, col);
        }
        s.rect(0, 23, AW as i32, 1, pal.picado[1]);
        // Candles, in the order they are lit: (x, top of the body).
        const CANDLES: [(i32, i32); 8] = [
            (6, 8),
            (17, 8),
            (10, 20),
            (13, 20),
            (2, 14),
            (21, 14),
            (1, 20),
            (22, 20),
        ];
        for (i, &(x, y)) in CANDLES.iter().enumerate() {
            // A veladora in colored glass, its flame flickering above it.
            s.rect(x, y, 2, 2, pal.picado[(i + 3) % 6]);
            s.dot(x, y, pal.picado[(i + 3) % 6].scale(1.3));
            if i < lit {
                let h = (i as u32 + 1).wrapping_mul(0x9e37_79b1) ^ tick.wrapping_mul(0x85eb_ca6b);
                let k = ((h >> 9) % 100) as f32 / 100.0;
                s.rect(x, y - 2, 1, 2, pal.vela);
                // (The lowest tier's flames would touch the shelf above.)
                if k > 0.35 && y != 20 {
                    s.dot(x, y - 3, pal.flor.lerp(pal.vela, k));
                }
            } else {
                s.dot(x, y - 1, pal.dim);
            }
        }
    }

    fn draw_altar(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        self.paint_altar(&env.pal);
        let (cols, rows) = self.altar.cells();
        let y0 = r.y + ((r.h - rows) / 2).max(0);
        self.altar.blit(c, r, r.x + r.w - cols, y0);
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
    fn the_gateway_shows_the_nine_levels_and_where_souls_have_been() {
        let fiesta = Fiesta::of_day(11, 2);
        let env = env(fiesta);
        let mut g = Gateway::new(3, fiesta);
        let text = draw(&mut g, &env, 90, 30).to_text();
        assert!(text.contains("MICTL\u{00c1}N GATEWAY ONLINE"));
        for (name, trial, _) in LEVELS {
            assert!(text.contains(name), "{name}");
            assert!(text.contains(trial), "{trial}");
        }
        assert!(text.contains('\u{273f}') && text.contains('\u{25b8}'));
        // Souls cross the levels for as long as it runs: nobody gets stuck.
        for _ in 0..4000 {
            g.update(0.1, &env);
        }
        assert!(g.home > 150 && !g.souls.is_empty());
        assert!(g.log.len() > 3);
    }

    #[test]
    fn the_ofrenda_has_its_table_its_altar_and_its_candles() {
        let env = env(Fiesta::of_day(11, 2));
        let lit = |level_day: Fiesta| {
            let mut o = Ofrenda::new(4, level_day);
            let c = draw(&mut o, &env, 100, 30);
            let flames = c
                .cells()
                .iter()
                .filter(|x| x.fg == env.pal.vela && x.ch != ' ')
                .count();
            (c.to_text(), flames)
        };
        let (text, high) = lit(Fiesta::of_day(11, 2));
        assert!(text.contains("SELECT * FROM altar"));
        assert!(text.contains("RECIBIDA") && text.contains("PENDIENTE"));
        assert!(text.contains("cempas\u{00fa}chil"));
        let (_, low) = lit(Fiesta::of_day(10, 28));
        assert!(high > low && low > 0, "{low} {high}");
        // Too narrow or too short for the altar: the table alone.
        let mut o = Ofrenda::new(4, Fiesta::of_day(11, 2));
        let c = draw(&mut o, &env, 60, 30);
        assert!(
            c.cells()
                .iter()
                .all(|x| x.ch != '\u{2580}' && x.ch != '\u{2584}')
        );
    }

    #[test]
    fn the_children_come_on_the_last_days() {
        let mut rng = Rng::with_seed(8);
        let angelitos = |level: f32, rng: &mut Rng| {
            (0..2000)
                .filter(|_| muertos::ANGELITOS.contains(&muertos::soul(rng, level)))
                .count()
        };
        assert_eq!(angelitos(0.25, &mut rng), 0);
        assert_eq!(angelitos(0.45, &mut rng), 0);
        let (a, b, c) = (
            angelitos(0.6, &mut rng),
            angelitos(0.8, &mut rng),
            angelitos(1.0, &mut rng),
        );
        assert!(a > 200 && b > a && c > 200 && c < b, "{a} {b} {c}");
        // They are given toys and sweets.
        let mut rng = Rng::with_seed(9);
        assert!(muertos::TOYS.contains(&muertos::offering(&mut rng, "Lupita")));
    }
}
