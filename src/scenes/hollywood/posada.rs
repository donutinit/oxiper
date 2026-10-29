//! The widgets only Navidad has: the tracking of the Reyes Magos on their
//! way to Mexico with an ETA to January 6, the database of cartas a los
//! Reyes (with a globo de cantoya carrying one up), the piñata impact sensor
//! ("golpes: 7/9 ¡dale!") with the piñata itself swinging beside it, and the
//! ponche's thermometer over its olla de barro. The days matter: the Reyes
//! cover a little more of the road each day from December 12, the cartas
//! pile up through the posadas, the ponche is hottest on Nochebuena.

use super::map::{self, Dots};
use super::navidad::{self, NINOS, REGALOS};
use super::sprite::Sprite;
use super::{Env, Pal, Rect, Widget, put, put_ch, truncate};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Canvas, DIM};
use fastrand::Rng;
use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::fmt::Write;

const BLK: [char; 8] = [
    '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}', '\u{2588}',
];

/// Draws a line of single pixels into a sprite.
fn pixel_line(s: &mut Sprite, x0: f32, y0: f32, x1: f32, y1: f32, col: Color) {
    let n = ((x1 - x0).abs().max((y1 - y0).abs()).ceil() as i32).max(1);
    for i in 0..=n {
        let t = i as f32 / n as f32;
        s.dot(
            (x0 + (x1 - x0) * t).round() as i32,
            (y0 + (y1 - y0) * t).round() as i32,
            col,
        );
    }
}

/// "1,204": thousands with commas, the Mexican way.
fn thousands(n: u32, out: &mut String) {
    let digits = n.to_string();
    let first = digits.len() % 3;
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (i + 3 - first).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
}

/// A bar `[█████░░░]` of `w` cells, `frac` full.
#[allow(clippy::too_many_arguments)]
fn bar(c: &mut Canvas, r: Rect, x: i32, y: i32, w: i32, frac: f32, col: Color, pal: &Pal) {
    let filled = (frac.clamp(0.0, 1.0) * w as f32) as i32;
    for i in 0..w {
        if i < filled {
            put_ch(c, r, x + i, y, '\u{2588}', col, 0);
        } else {
            put_ch(c, r, x + i, y, '\u{2591}', pal.dim, 0);
        }
    }
}

// ---------------------------------------------------------------- the Reyes

/// Where the star stops: Mexico City.
const MEXICO: (f32, f32) = (19.4, -99.1);

/// A king: name, his mount, his gift, the place he sets out from (name,
/// latitude, longitude), how far ahead of the others he rides and the garland
/// color he is drawn in.
struct King {
    name: &'static str,
    mount: &'static str,
    gift: &'static str,
    from: (&'static str, f32, f32),
    lead: f32,
    color: usize,
}

const KINGS: [King; 3] = [
    King {
        name: "MELCHOR",
        mount: "caballo",
        gift: "oro",
        from: ("SEVILLA", 37.4, -6.0),
        lead: 0.012,
        color: 0,
    },
    King {
        name: "GASPAR",
        mount: "camello",
        gift: "incienso",
        from: ("BAGDAD", 33.3, 44.4),
        lead: 0.0,
        color: 2,
    },
    King {
        name: "BALTASAR",
        mount: "elefante",
        gift: "mirra",
        from: ("ADIS ABEBA", 9.0, 38.7),
        lead: -0.015,
        color: 3,
    },
];

const NEWS: [&str; 9] = [
    "Melchor: caballo al trote, el oro viaja asegurado",
    "Gaspar: el camello cruza las dunas, el incienso se huele de lejos",
    "Baltasar: el elefante abre camino, la mirra va a salvo",
    "Los tres Reyes consultan el mapa: la estrella sigue al oeste",
    "Estrella de Bel\u{00e9}n: se\u{00f1}al estable, rumbo a M\u{00e9}xico",
    "Camello de Gaspar: sin permisos de root, sed de 40 L",
    "Elefante de Baltasar: no cabe en el t\u{00fa}nel, rodea",
    "Los Reyes piden posada en el desierto: aqu\u{00ed} no es mes\u{00f3}n",
    "Oro, incienso y mirra: sin da\u{00f1}os, firma requerida al recibir",
];

/// A day, in seconds.
const DAY: f32 = 86_400.0;

pub struct Reyes {
    mx: Fiesta,
    land: Dots,
    fx: Dots,
    land_col: Option<Color>,
    clock: f32,
    /// Seconds into the day as the scene feels it; the ETA counts down from
    /// here.
    tod: f32,
    /// How far each king is along his road, 0 to 1, and how long the road is.
    p: [f32; 3],
    km: [f32; 3],
    /// Where the lines of text are written, reused every frame.
    buf: String,
}

/// Great-circle distance in kilometers.
fn haversine(a: (f32, f32), b: (f32, f32)) -> f32 {
    let (la1, la2) = (a.0.to_radians(), b.0.to_radians());
    let dlat = la2 - la1;
    let dlon = (b.1 - a.1).to_radians();
    let h = (dlat / 2.0).sin().powi(2) + la1.cos() * la2.cos() * (dlon / 2.0).sin().powi(2);
    12_742.0 * h.sqrt().min(1.0).asin()
}

impl Reyes {
    pub fn new(seed: u64, mx: Fiesta) -> Reyes {
        let days = navidad::days_to_reyes(mx);
        // 26 days from December 12 to the 6th: the road is walked by the date.
        let base = 1.0 - days as f32 / 26.0;
        let mut p = [0.0; 3];
        let mut km = [0.0; 3];
        for (i, k) in KINGS.iter().enumerate() {
            p[i] = if days == 0 {
                1.0
            } else {
                (base + k.lead).clamp(0.0, 0.985)
            };
            km[i] = haversine((k.from.1, k.from.2), MEXICO);
        }
        Reyes {
            mx,
            land: Dots::new(1, 1),
            fx: Dots::new(1, 1),
            land_col: None,
            clock: 0.0,
            tod: (seed % 70_000) as f32 + 4_000.0,
            p,
            km,
            buf: String::new(),
        }
    }

    #[cfg(test)]
    pub fn progress(&self) -> f32 {
        self.p.iter().sum::<f32>() / 3.0
    }

    /// Seconds to January 6 at 00:00; 0 once the Reyes are here.
    fn remaining(&self) -> u32 {
        let days = navidad::days_to_reyes(self.mx);
        (days as f32 * DAY - self.tod).max(0.0) as u32
    }
}

impl Widget for Reyes {
    fn name(&self) -> &'static str {
        "rastreo-reyes --belen"
    }

    fn resize(&mut self, w: u16, h: u16) {
        let rows = (h as i32 - 1 - Self::list_rows(h as i32)).max(1) as u16;
        self.land = Dots::new(w.max(1), rows);
        self.fx = Dots::new(w.max(1), rows);
        self.land_col = None;
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        self.tod += dt;
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let arrived = navidad::days_to_reyes(self.mx) == 0;
        put(c, r, 0, 0, "RASTREO DE LOS REYES MAGOS", pal.bright, BOLD);
        let rem = self.remaining();
        let mut t = std::mem::take(&mut self.buf);
        t.clear();
        if arrived {
            t.push_str("\u{00a1}HAN LLEGADO!");
        } else {
            let _ = write!(
                t,
                "ETA 06-ENE 00:00  T-{}d {:02}:{:02}:{:02}",
                rem / 86_400,
                rem / 3600 % 24,
                rem / 60 % 60,
                rem % 60
            );
        }
        let tw = t.chars().count() as i32;
        let blink = !arrived || (self.clock * 2.0) as i32 % 2 == 0;
        if r.w > 26 + tw && blink {
            put(
                c,
                r,
                r.w - tw,
                0,
                &t,
                if arrived { pal.oro } else { pal.amber },
                BOLD,
            );
        }
        let list = Self::list_rows(r.h);
        let body = Rect {
            x: r.x,
            y: r.y + 1,
            w: r.w,
            h: (r.h - 1 - list).max(0),
        };
        if body.h > 0 && self.land_col != Some(pal.main) {
            self.land = Dots::new(body.w.max(1) as u16, body.h as u16);
            self.fx = Dots::new(body.w.max(1) as u16, body.h as u16);
            map::paint_land(&mut self.land, pal.fade(pal.main, 0.7));
            self.land_col = Some(pal.main);
        }
        if body.h > 0 {
            let (pw, ph) = (self.fx.pw(), self.fx.ph());
            self.fx.clear();
            let (x1, y1) = map::project(MEXICO.0, MEXICO.1, pw, ph);
            for (i, k) in KINGS.iter().enumerate() {
                let (x0, y0) = map::project(k.from.1, k.from.2, pw, ph);
                let lift = ((x1 - x0).abs() + (y1 - y0).abs()) * 0.25;
                let col = pal.picado[k.color];
                let steps = 48;
                let upto = (steps as f32 * self.p[i]) as i32;
                let (mut px, mut py) = (x0, y0);
                for s in 1..=steps {
                    let u = s as f32 / steps as f32;
                    let x = x0 + (x1 - x0) * u;
                    let y = y0 + (y1 - y0) * u - (u * PI).sin() * lift;
                    if s <= upto {
                        self.fx.line(px, py, x, y, col);
                    } else if s % 3 == 0 {
                        // The road still to go, in dots.
                        self.fx.set(x as i32, y as i32, pal.fade(col, 0.6));
                    }
                    px = x;
                    py = y;
                }
            }
            self.land.blit(c, body);
            self.fx.blit(c, body);
            // The Estrella de Belén over the destination, and the kings riding
            // toward it.
            let star =
                ['\u{2736}', '\u{2726}', '\u{2727}', '\u{2726}'][(self.clock * 3.0) as usize % 4];
            let (sx, sy) = ((x1 / 2.0) as i32, (y1 / 4.0) as i32);
            for (dx, dy, ch) in [(-2, 0, '\u{00b7}'), (2, 0, '\u{00b7}'), (0, -1, '\u{00b7}')] {
                put_ch(c, body, sx + dx, sy + dy, ch, pal.fade(pal.oro, 0.35), 0);
            }
            put_ch(c, body, sx, sy, star, pal.oro, BOLD);
            for (i, k) in KINGS.iter().enumerate() {
                let (x0, y0) = map::project(k.from.1, k.from.2, pw, ph);
                let lift = ((x1 - x0).abs() + (y1 - y0).abs()) * 0.25;
                let u = self.p[i];
                let x = x0 + (x1 - x0) * u;
                let y = y0 + (y1 - y0) * u - (u * PI).sin() * lift;
                let bob = if (self.clock * 2.0 + i as f32) as i32 % 2 == 0 {
                    '\u{25c9}'
                } else {
                    '\u{25ce}'
                };
                put_ch(
                    c,
                    body,
                    (x / 2.0) as i32,
                    (y / 4.0) as i32,
                    bob,
                    pal.picado[k.color],
                    BOLD,
                );
            }
        }
        // The list: one line for each king, then the news.
        for (i, k) in KINGS.iter().enumerate().take(if list >= 3 { 3 } else { 0 }) {
            let y = r.h - list + i as i32;
            let col = pal.picado[k.color];
            put_ch(c, r, 0, y, '\u{25c9}', col, BOLD);
            t.clear();
            let _ = write!(t, "{:<9}", k.name);
            let mut x = put(c, r, 2, y, &t, col, BOLD);
            if r.w >= 46 {
                t.clear();
                let _ = write!(t, "{:<9}", k.mount);
                x = put(c, r, x, y, &t, pal.main, 0);
            }
            if r.w >= 58 {
                t.clear();
                let _ = write!(t, "{:<9}", k.gift);
                x = put(c, r, x, y, &t, pal.dim, 0);
            }
            if r.w >= 70 {
                t.clear();
                let _ = write!(t, "{:<11}", k.from.0);
                x = put(c, r, x, y, &t, pal.cyan, 0);
            }
            t.clear();
            thousands(((1.0 - self.p[i]) * self.km[i]) as u32, &mut t);
            t.push_str(" km");
            let w = t.chars().count() as i32;
            put(
                c,
                r,
                (x + 7 - w).max(x),
                y,
                &t,
                if arrived { pal.oro } else { pal.amber },
                0,
            );
        }
        if list >= 5 {
            let y = r.h - list + 3;
            let avg = self.p.iter().sum::<f32>() / 3.0;
            t.clear();
            let _ = write!(
                t,
                "camino recorrido {:>3.0}%   estrella: ESTABLE   oro, incienso y mirra: SIN DA\u{00d1}OS",
                avg * 100.0
            );
            put(c, r, 0, y, &t, pal.dim, 0);
            let news = NEWS[(self.clock / 4.0) as usize % NEWS.len()];
            put_ch(c, r, 0, y + 1, '\u{25b8}', pal.oro, BOLD);
            put(
                c,
                r,
                2,
                y + 1,
                truncate(news, (r.w - 2).max(0) as usize),
                pal.bright,
                0,
            );
        }
        self.buf = t;
    }
}

impl Reyes {
    /// Rows under the map: a line per king and, when there is room, the
    /// summary and the news.
    fn list_rows(h: i32) -> i32 {
        if h >= 15 {
            5
        } else if h >= 9 {
            3
        } else {
            0
        }
    }
}

// ---------------------------------------------------------------- cartas

/// A globo de cantoya, the paper balloon that carries a letter up: this wide
/// and this tall, in pixels.
const BW: usize = 15;
const BH: usize = 26;

/// How wide a row of the balloon's dome is, top to bottom.
const DOME: [i32; 14] = [3, 5, 7, 9, 10, 11, 11, 11, 11, 10, 9, 8, 7, 6];

struct Carta {
    id: u16,
    nino: &'static str,
    edad: u8,
    regalo: &'static str,
    /// Stars of good behavior, 1 to 5.
    bien: u8,
    /// 0 escrita, 1 en camino, 2 entregada, 3 carbón dulce.
    estado: u8,
    flash: f32,
}

pub struct Cartas {
    mx: Fiesta,
    rng: Rng,
    rows: VecDeque<Carta>,
    next_id: u16,
    timer: f32,
    clock: f32,
    /// The balloon, repainted every frame.
    globo: Sprite,
    /// Totals for the footer.
    cartas: u32,
    juguetes: u32,
    carbon: u32,
    buf: String,
}

impl Cartas {
    pub fn new(seed: u64, mx: Fiesta) -> Cartas {
        let mut c = Cartas {
            mx,
            rng: Rng::with_seed(seed ^ 0xca27a5),
            rows: VecDeque::new(),
            next_id: 4000,
            timer: 0.5,
            clock: 0.0,
            globo: Sprite::new(BW, BH),
            // The cartas pile up as the holiday goes.
            cartas: 800 + (mx.level * 3000.0) as u32,
            juguetes: 0,
            carbon: 0,
            buf: String::new(),
        };
        c.next_id += c.rng.u16(..900);
        c.juguetes = c.cartas * 3 / 4;
        c.carbon = c.cartas / 40;
        for _ in 0..40 {
            c.add_row();
            if let Some(r) = c.rows.back_mut() {
                r.flash = 0.0;
                r.estado = c.rng.u8(..3);
                if r.estado == 2 && r.bien <= 1 {
                    r.estado = 3;
                }
            }
        }
        c
    }

    fn add_row(&mut self) {
        let mut nino = NINOS[self.rng.usize(..NINOS.len())];
        for _ in 0..8 {
            if !self.rows.iter().rev().take(24).any(|r| r.nino == nino) {
                break;
            }
            nino = NINOS[self.rng.usize(..NINOS.len())];
        }
        let row = Carta {
            id: self.next_id,
            nino,
            edad: self.rng.u8(3..13),
            regalo: REGALOS[self.rng.usize(..REGALOS.len())],
            // Most are good; the occasional one gets carbón dulce.
            bien: if self.rng.u8(..12) == 0 {
                1
            } else {
                self.rng.u8(3..6)
            },
            estado: 0,
            flash: 1.0,
        };
        self.next_id = self.next_id.wrapping_add(1);
        self.rows.push_back(row);
        while self.rows.len() > 40 {
            self.rows.pop_front();
        }
    }

    /// The query, following the days: the posadas one by one, then the days
    /// of waiting, then the delivery.
    fn query(&self, narrow: bool, out: &mut String) {
        let n = navidad::posada(self.mx);
        out.push_str(if narrow {
            "reyes=# SELECT * FROM cartas;"
        } else if n > 0 {
            ""
        } else if self.mx.month == 1 && self.mx.day >= 6 {
            "reyes=# SELECT * FROM cartas WHERE estado = 'ENTREGADA';"
        } else if self.mx.month == 1 {
            "reyes=# SELECT * FROM cartas WHERE estado <> 'ENTREGADA';"
        } else if self.mx.day >= 25 {
            "reyes=# SELECT * FROM cartas ORDER BY edad;"
        } else {
            "reyes=# SELECT * FROM cartas ORDER BY id;"
        });
        if n > 0 && !narrow {
            let _ = write!(out, "reyes=# SELECT * FROM cartas WHERE posada <= {n};");
        }
    }

    fn paint_globo(&mut self, pal: &Pal) {
        let tick = (self.clock * 10.0) as u32;
        let s = &mut self.globo;
        s.clear();
        let mid = BW as i32 / 2;
        for (y, &w) in DOME.iter().enumerate() {
            for x in 0..w {
                let px = mid - w / 2 + x;
                // Gores of papel de china, a color every three pixels.
                let col = pal.papel[((px + 40) / 3) as usize % 6];
                s.dot(px, y as i32, if y < 2 { col.scale(1.15) } else { col });
            }
        }
        // The mouth, the little flame and the string.
        s.rect(mid - 2, 14, 5, 1, rgb(0x8c6a3c));
        let flick = (tick * 7 % 5) as i32;
        s.rect(mid - 1, 15, 3, 2, rgb(0xff8c1a));
        s.dot(mid, 15, rgb(0xffe27a));
        if flick > 1 {
            s.dot(mid, 17, rgb(0xff8c1a));
        }
        s.dot(mid, 18, pal.dim);
        s.dot(mid, 19, pal.dim);
        // The carta, with a heart for a seal.
        s.rect(mid - 3, 20, 7, 5, rgb(0xf4eee2));
        s.rect(mid - 3, 20, 7, 1, rgb(0xd9d0bc));
        s.dot(mid, 22, pal.red);
        s.dot(mid - 1, 21, pal.red);
        s.dot(mid + 1, 21, pal.red);
    }
}

impl Widget for Cartas {
    fn name(&self) -> &'static str {
        "psql cartas_reyes"
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
            0..=4 if n > 0 => {
                // A carta moves on: written, on its balloon, delivered (or
                // paid in carbón dulce, the sweet coal).
                let i = self.rng.usize(..n);
                let row = &mut self.rows[i];
                if row.estado < 2 {
                    row.estado += 1;
                    row.flash = 1.0;
                    if row.estado == 2 {
                        if row.bien <= 1 {
                            row.estado = 3;
                            self.carbon += 1;
                        } else {
                            self.juguetes += 1;
                        }
                    }
                }
            }
            _ => {
                self.add_row();
                self.cartas += 1;
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let globo = r.w >= 74 && r.h >= 14;
        let tw = if globo { r.w - BW as i32 - 2 } else { r.w };
        let table = Rect { w: tw, ..r };
        let cursor = if (self.clock * 2.0) as i32 % 2 == 0 {
            "\u{258c}"
        } else {
            " "
        };
        let mut line = std::mem::take(&mut self.buf);
        line.clear();
        self.query(tw < 52, &mut line);
        let x = put(c, table, 0, 0, &line, pal.bright, BOLD);
        put(c, table, x + 1, 0, cursor, pal.oro, 0);
        // Columns by the room there is: id and name always, the gift when
        // there are 8 cells for it, the age and the stars when wide.
        let wide = tw >= 70;
        let item_w = if wide {
            24
        } else {
            (tw - 19 - 13).clamp(0, 24) as usize
        };
        line.clear();
        if wide {
            let _ = write!(
                line,
                "{:>5}  {:<11} {:>4}  {:<24} {:<8} estado",
                "id", "nombre", "edad", "regalo", "conducta"
            );
        } else if item_w >= 8 {
            let _ = write!(
                line,
                "{:>5}  {:<11} {:<item_w$} estado",
                "id", "nombre", "regalo"
            );
        } else {
            let _ = write!(line, "{:>5}  {:<11} estado", "id", "nombre");
        }
        put(c, table, 0, 1, &line, pal.cyan, BOLD);
        for x in 0..tw {
            put_ch(c, table, x, 2, '\u{2500}', pal.dim, 0);
        }
        let rows = (r.h - 4).max(0) as usize;
        for (i, row) in self.rows.iter().rev().take(rows).enumerate() {
            let y = r.h - 2 - i as i32;
            line.clear();
            let _ = write!(line, "{:>5}  {:<11} ", row.id, truncate(row.nino, 11));
            if wide {
                let _ = write!(line, "{:>4}  {:<24} ", row.edad, truncate(row.regalo, 24));
            } else if item_w >= 8 {
                let _ = write!(line, "{:<item_w$} ", truncate(row.regalo, item_w));
            }
            let hot = row.flash > 0.05;
            let base = if hot { pal.bright } else { pal.main };
            let mut x = put(c, table, 0, y, &line, base, if hot { BOLD } else { 0 });
            if wide {
                // Five stars of conducta, as many filled as the child earned.
                for s in 0..5u8 {
                    put_ch(
                        c,
                        table,
                        x + s as i32,
                        y,
                        if s < row.bien { '\u{2605}' } else { '\u{2606}' },
                        if s < row.bien { pal.oro } else { pal.dim },
                        0,
                    );
                }
                x += 8;
            }
            let (text, col) = match row.estado {
                0 => ("ESCRITA", pal.amber),
                1 => ("EN GLOBO", pal.cyan),
                2 => ("ENTREGADA \u{2713}", pal.green),
                _ => ("CARB\u{00d3}N DULCE", pal.red),
            };
            put(
                c,
                table,
                x,
                y,
                text,
                col,
                if row.estado >= 2 { BOLD } else { 0 },
            );
        }
        line.clear();
        let _ = write!(
            line,
            "({} filas)  cartas {}  juguetes {}  carb\u{00f3}n dulce {}",
            self.rows.len(),
            self.cartas,
            self.juguetes,
            self.carbon
        );
        put(c, table, 0, r.h - 1, &line, pal.dim, 0);
        self.buf = line;
        if globo {
            self.paint_globo(pal);
            // It rises from the bottom and out at the top, then another one
            // is let go.
            let rows = BH as i32 / 2;
            let t = (self.clock % 14.0) / 14.0;
            let y0 = r.y + r.h - (t * (r.h + rows) as f32) as i32;
            let sway = ((self.clock * 0.9).sin() * 1.5) as i32;
            self.globo.blit(c, r, r.x + r.w - BW as i32 - 1 + sway, y0);
        }
    }
}

// ---------------------------------------------------------------- piñata

/// The piñata is a pixel sprite, this wide and this tall.
const PW: usize = 23;
const PH: usize = 28;

/// One falling bit: a piece of the piñata or a sweet.
#[derive(Clone, Copy)]
struct Bit {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    col: Color,
}

const SHOUTS: [&str; 6] = [
    "\u{00a1}dale!",
    "\u{00a1}dale, dale!",
    "\u{00a1}otro!",
    "\u{00a1}casi!",
    "\u{00a1}no pierdas el tino!",
    "\u{00a1}le diste!",
];

pub struct Pinata {
    mx: Fiesta,
    rng: Rng,
    pic: Sprite,
    clock: f32,
    golpes: u8,
    integrity: f32,
    /// Seconds until the next swing of the stick.
    wait: f32,
    /// How far the swing has come, 0 to 1; negative when idle.
    swing: f32,
    /// The pendulum: angle and angular speed.
    ang: f32,
    vel: f32,
    flash: f32,
    /// Seconds since the piñata burst; negative while it hangs.
    burst: f32,
    /// Force of the last impacts, in newtons.
    force: VecDeque<f32>,
    rotas: u32,
    dulces: u32,
    /// Which set of colors this piñata wears.
    turno: usize,
    shout: &'static str,
    bateador: &'static str,
    angulo: u8,
    bits: Vec<Bit>,
    buf: String,
}

impl Pinata {
    pub fn new(seed: u64, mx: Fiesta) -> Pinata {
        let mut rng = Rng::with_seed(seed ^ 0x91a7a);
        let golpes = rng.u8(..8);
        let n = navidad::posada(mx) as u32;
        let mut p = Pinata {
            mx,
            pic: Sprite::new(PW, PH),
            clock: 0.0,
            golpes,
            integrity: 100.0 - golpes as f32 * 100.0 / 9.0,
            wait: 0.8,
            swing: -1.0,
            ang: 0.0,
            vel: 0.0,
            flash: 0.0,
            burst: -1.0,
            force: VecDeque::new(),
            // One piñata for each posada so far, and the ones of the year.
            rotas: 2 + n * 3 + (mx.level * 10.0) as u32,
            dulces: 0,
            turno: rng.usize(..6),
            shout: SHOUTS[0],
            bateador: NINOS[rng.usize(..NINOS.len())],
            angulo: 30,
            bits: Vec::with_capacity(48),
            buf: String::new(),
            rng,
        };
        for _ in 0..golpes {
            let f = 40.0 + p.rng.f32() * 60.0;
            p.force.push_back(f);
        }
        p.dulces = 20 * golpes as u32;
        p
    }

    #[cfg(test)]
    pub fn golpes(&self) -> u8 {
        self.golpes
    }

    #[cfg(test)]
    pub fn broken(&self) -> bool {
        self.burst >= 0.0
    }

    /// Seconds between swings: the posadas get faster as they go.
    fn pace(&mut self) -> f32 {
        (0.8 + self.rng.f32() * 0.9) * (1.35 - 0.5 * self.mx.level)
    }

    fn hit(&mut self) {
        self.golpes += 1;
        self.integrity = (100.0 - self.golpes as f32 * 100.0 / 9.0).max(0.0);
        self.vel += 2.4;
        self.flash = 1.0;
        let f = 38.0 + self.rng.f32() * 62.0;
        self.force.push_back(f);
        while self.force.len() > 24 {
            self.force.pop_front();
        }
        self.dulces += self.rng.u32(0..6);
        self.shout = SHOUTS[self.rng.usize(..SHOUTS.len())];
        self.angulo = self.rng.u8(12..65);
        if self.golpes >= 9 {
            self.shout = "\u{00a1}\u{00a1}SE ROMPI\u{00d3}!!";
            self.burst = 0.0;
            self.rotas += 1;
            self.dulces += 150 + (self.mx.level * 120.0) as u32;
            self.spawn_bits();
        }
    }

    /// The pieces of the piñata and a rain of sweets, falling from where it hung.
    fn spawn_bits(&mut self) {
        self.bits.clear();
        let (cx, cy) = (PW as f32 / 2.0, 15.0);
        let sweets = [
            rgb(0xf4f1ea),
            rgb(0xff9ec7),
            rgb(0xffe28a),
            rgb(0x9ee6ff),
            rgb(0xb5f0a0),
            rgb(0xe8891c),
            rgb(0xc9b26b),
            rgb(0xff8c00),
        ];
        for i in 0..40 {
            let a = self.rng.f32() * TAU;
            let sp = 4.0 + self.rng.f32() * 16.0;
            self.bits.push(Bit {
                x: cx,
                y: cy,
                vx: a.cos() * sp,
                vy: a.sin() * sp - 8.0,
                col: sweets[i % sweets.len()],
            });
        }
    }

    /// Paints the piñata of seven points on its rope, the stick and whatever
    /// is falling.
    fn paint(&mut self, pal: &Pal) {
        let s = &mut self.pic;
        s.clear();
        let pivot = (PW as f32 / 2.0, 0.0);
        let len = 15.0;
        let (cx, cy) = (
            pivot.0 + len * self.ang.sin(),
            pivot.1 + len * self.ang.cos(),
        );
        let bursting = self.burst >= 0.0;
        if !bursting {
            pixel_line(s, pivot.0, pivot.1, cx, cy - 5.0, rgb(0xc9b26b));
            let lit = self.flash;
            // Seven cones, one per color of papel de china, turning with the swing.
            for k in 0..7 {
                let a = self.ang * 1.5 + k as f32 * TAU / 7.0 - FRAC_PI_2;
                let col = pal.papel[(k + self.turno) % 6].lerp(rgb(0xffffff), lit * 0.55);
                for step in 0..=15 {
                    let rr = 2.0 + step as f32 * 0.6;
                    let hw = (1.0 - step as f32 / 15.0) * 1.9 + 0.1;
                    let mut w = -hw;
                    while w <= hw {
                        let px = cx + a.cos() * rr - a.sin() * w;
                        let py = cy + a.sin() * rr + a.cos() * w;
                        s.dot(px.round() as i32, py.round() as i32, col);
                        w += 0.5;
                    }
                }
            }
            // The body, in bands, with a highlight.
            for dy in -3..=3i32 {
                let half = ((11 - dy * dy) as f32).sqrt().round() as i32;
                let col = pal.papel[((dy + 3) as usize / 2 + self.turno) % 6]
                    .lerp(rgb(0xffffff), lit * 0.55);
                for dx in -half..=half {
                    s.dot(
                        (cx + dx as f32).round() as i32,
                        (cy + dy as f32).round() as i32,
                        col,
                    );
                }
            }
            s.dot(
                (cx - 2.0).round() as i32,
                (cy - 2.0).round() as i32,
                rgb(0xffffff),
            );
        }
        // The stick: raised, then swung down to meet the piñata.
        if self.swing >= 0.0 {
            let a = (1.0 - (self.swing / 0.6).min(1.0)) * 1.2 - 0.05;
            let back = if self.swing > 0.6 {
                (self.swing - 0.6) / 0.4 * 0.8
            } else {
                0.0
            };
            let a = a + back;
            let base = (PW as f32 - 1.0, 17.0);
            let tip = (base.0 - 13.0 * a.cos(), base.1 - 13.0 * a.sin());
            pixel_line(s, base.0, base.1, tip.0, tip.1, rgb(0xa0703c));
            pixel_line(s, base.0, base.1 - 1.0, tip.0, tip.1 - 1.0, rgb(0x7a5230));
        }
        for b in &self.bits {
            s.dot(b.x.round() as i32, b.y.round() as i32, b.col);
        }
    }
}

impl Widget for Pinata {
    fn name(&self) -> &'static str {
        "pinata-sensor --golpes"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        self.flash = (self.flash - dt * 4.0).max(0.0);
        // The pendulum, with a little friction.
        let acc = -(self.ang) * 30.0 - self.vel * 1.4;
        self.vel += acc * dt;
        self.ang = (self.ang + self.vel * dt).clamp(-1.2, 1.2);
        if self.swing >= 0.0 {
            let before = self.swing;
            self.swing += dt * 2.6;
            if self.burst < 0.0 && before < 0.6 && self.swing >= 0.6 {
                self.hit();
            }
            if self.swing >= 1.0 {
                self.swing = -1.0;
                self.wait = self.pace();
            }
        } else if self.burst < 0.0 {
            self.wait -= dt;
            if self.wait <= 0.0 {
                self.swing = 0.0;
            }
        }
        if self.burst >= 0.0 {
            self.burst += dt;
            for b in self.bits.iter_mut() {
                b.vy += 46.0 * dt;
                b.x += b.vx * dt;
                b.y += b.vy * dt;
                if b.y > PH as f32 - 1.0 {
                    b.y = PH as f32 - 1.0;
                    b.vy *= -0.35;
                    b.vx *= 0.7;
                }
                b.x = b.x.clamp(0.0, PW as f32 - 1.0);
            }
            if self.burst > 3.4 {
                // Another piñata is hung.
                self.burst = -1.0;
                self.golpes = 0;
                self.integrity = 100.0;
                self.turno = (self.turno + 1) % 6;
                self.bateador = NINOS[self.rng.usize(..NINOS.len())];
                self.wait = 1.4;
                self.swing = -1.0;
                self.bits.clear();
                self.shout = SHOUTS[0];
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let pic = r.w >= 54 && r.h >= 11;
        let x0 = if pic { PW as i32 + 3 } else { 0 };
        if pic {
            self.paint(pal);
            let rows = PH as i32 / 2;
            let y0 = r.y + ((r.h - rows) / 2).max(0);
            self.pic.blit(c, r, r.x + 1, y0);
        }
        put(
            c,
            r,
            x0,
            0,
            "SENSOR DE IMPACTOS: PI\u{00d1}ATA DE 7 PICOS",
            pal.bright,
            BOLD,
        );
        let mut t = std::mem::take(&mut self.buf);
        let mut y = 2;
        let n = navidad::posada(self.mx);
        t.clear();
        if n > 0 {
            let _ = write!(t, "posada {n} de 9   pi\u{00f1}atas rotas {}", self.rotas);
        } else {
            let _ = write!(t, "pi\u{00f1}atas rotas {}", self.rotas);
        }
        put(c, r, x0, y, &t, pal.cyan, 0);
        y += 1;
        let hot = self.flash > 0.2 || self.burst >= 0.0;
        t.clear();
        let _ = write!(t, "golpes: {}/9 {}", self.golpes, self.shout);
        put(c, r, x0, y, &t, if hot { pal.oro } else { pal.amber }, BOLD);
        y += 1;
        // The force of the last impacts, as bars.
        let x = put(c, r, x0, y, "fuerza     ", pal.dim, 0);
        let room = (r.w - x - 8).clamp(0, 24) as usize;
        let hist = self
            .force
            .iter()
            .skip(self.force.len().saturating_sub(room));
        let mut xx = x;
        for f in hist {
            let k = ((f - 30.0) / 70.0 * 7.0).clamp(0.0, 7.0) as usize;
            put_ch(
                c,
                r,
                xx,
                y,
                BLK[k],
                if *f > 80.0 { pal.red } else { pal.main },
                0,
            );
            xx += 1;
        }
        t.clear();
        let last = self.force.back().copied().unwrap_or(0.0);
        let _ = write!(t, " {last:.0} N");
        put(c, r, xx, y, &t, pal.dim, 0);
        y += 1;
        let x = put(c, r, x0, y, "integridad ", pal.dim, 0);
        let bw = (r.w - x - 6).clamp(0, 20);
        bar(
            c,
            r,
            x,
            y,
            bw,
            self.integrity / 100.0,
            pal.ramp.at(0.45 + 0.5 * self.integrity / 100.0),
            pal,
        );
        t.clear();
        let _ = write!(t, " {:>3.0}%", self.integrity);
        put(c, r, x + bw, y, &t, pal.dim, 0);
        y += 1;
        t.clear();
        let _ = write!(
            t,
            "bateador   {} (vendado)  \u{00e1}ngulo {}\u{00b0}",
            self.bateador, self.angulo
        );
        put(c, r, x0, y, &t, pal.main, 0);
        y += 1;
        t.clear();
        let _ = write!(
            t,
            "dulces     {}  (colaci\u{00f3}n, cacahuate, tejocote, ca\u{00f1}a, mandarina)",
            self.dulces
        );
        put(c, r, x0, y, &t, pal.main, 0);
        y += 2;
        let state = if self.burst >= 0.0 {
            "ESTADO: \u{00a1}ROTA! LLUEVEN DULCES"
        } else if self.swing >= 0.0 {
            "ESTADO: GOLPEADA"
        } else {
            "ESTADO: COLGADA"
        };
        put(
            c,
            r,
            x0,
            y,
            state,
            if self.burst >= 0.0 {
                pal.green
            } else {
                pal.amber
            },
            BOLD,
        );
        y += 2;
        // The chant the sensor listens for.
        for (i, line) in [
            "Dale, dale, dale, no pierdas el tino,",
            "porque si lo pierdes, pierdes el camino.",
        ]
        .iter()
        .enumerate()
        {
            let lit = (self.clock * 0.5) as usize % 2 == i;
            put(
                c,
                r,
                x0,
                y + i as i32,
                line,
                if lit { pal.bright } else { pal.dim },
                if lit { 0 } else { DIM },
            );
        }
        self.buf = t;
    }
}

// ---------------------------------------------------------------- ponche

const OW: usize = 25;
const OH: usize = 22;

/// What goes into the ponche, as the log says it.
const INGREDIENTS: [(&str, &str); 8] = [
    ("tejocotes", "tejocote"),
    ("guayabas", "guayaba"),
    ("ca\u{00f1}a", "ca\u{00f1}a"),
    ("canela", "canela"),
    ("ciruela pasa", "ciruela"),
    ("tamarindo", "tamarindo"),
    ("jamaica", "jamaica"),
    ("piloncillo", "piloncillo"),
];

pub struct Ponche {
    mx: Fiesta,
    rng: Rng,
    olla: Sprite,
    clock: f32,
    temp: f32,
    target: f32,
    hist: VecDeque<f32>,
    acc: f32,
    nivel: f32,
    items: [u16; 8],
    add: f32,
    log: VecDeque<String>,
    servidas: u32,
    buf: String,
}

impl Ponche {
    pub fn new(seed: u64, mx: Fiesta) -> Ponche {
        let mut rng = Rng::with_seed(seed ^ 0x70c4e);
        let mut items = [0u16; 8];
        for it in items.iter_mut() {
            *it = rng.u16(4..30);
        }
        let target = Self::hot(mx);
        Ponche {
            mx,
            olla: Sprite::new(OW, OH),
            clock: 0.0,
            temp: target - 6.0,
            target,
            hist: VecDeque::from(vec![target - 6.0; 30]),
            acc: 0.0,
            nivel: 0.7 + rng.f32() * 0.2,
            items,
            add: 1.0,
            log: VecDeque::new(),
            servidas: 120 + (mx.level * 400.0) as u32,
            buf: String::new(),
            rng,
        }
    }

    /// How hot it runs: the ponche is hottest on Nochebuena.
    fn hot(mx: Fiesta) -> f32 {
        72.0 + 20.0 * mx.level
    }

    #[cfg(test)]
    pub fn temperature(&self) -> f32 {
        self.temp
    }

    fn say(&mut self, line: String) {
        self.log.push_back(line);
        while self.log.len() > 3 {
            self.log.pop_front();
        }
    }

    fn paint(&mut self, pal: &Pal) {
        let tick = (self.clock * 9.0) as u32;
        let hirviendo = self.temp > 88.0;
        let s = &mut self.olla;
        s.clear();
        let barro = rgb(0xb5532f);
        let sombra = rgb(0x8c3f22);
        let borde = rgb(0xd9763f);
        let ponche = rgb(0xa3182a);
        // Steam: wisps that climb and sway, more of them the hotter it is.
        let wisps = 2 + (self.temp - 60.0).max(0.0) as usize / 9;
        for k in 0..wisps.min(5) {
            let phase = self.clock * (0.9 + k as f32 * 0.17) + k as f32 * 2.1;
            for j in 0..7 {
                let up = (phase * 3.0 + j as f32 * 1.3).fract();
                let y = 8.0 - up * 8.0 - j as f32 * 0.6;
                let x = 6.0 + k as f32 * 3.4 + (phase + j as f32 * 0.8).sin() * 1.4;
                if y >= 0.0 {
                    s.dot(
                        x.round() as i32,
                        y.round() as i32,
                        pal.fade(rgb(0xdfe6e2), 0.2 + up * 0.55),
                    );
                }
            }
        }
        // The olla: a wide belly under a rim, with two ears.
        s.rect(3, 9, 19, 1, borde);
        s.rect(2, 10, 21, 1, borde);
        for (i, &w) in [21, 23, 25, 25, 25, 23, 21, 17].iter().enumerate() {
            let y = 11 + i as i32;
            let x = (OW as i32 - w) / 2;
            s.rect(x, y, w, 1, barro);
            s.dot(x, y, sombra);
            s.dot(x + w - 1, y, sombra);
            s.dot(x + w - 2, y, sombra);
        }
        s.rect(5, 19, 15, 1, sombra);
        // The ponche at the brim, with fruit floating in it.
        s.rect(4, 10, 17, 1, ponche);
        let fruit = [
            rgb(0xe8891c),
            rgb(0xc9b26b),
            rgb(0xd24a2a),
            rgb(0x8b4a1e),
            rgb(0xe8c25a),
        ];
        for i in 0..6i32 {
            let bob = ((self.clock * 1.6 + i as f32 * 1.7).sin() > 0.4) as i32;
            s.dot(5 + i * 3, 10 - bob, fruit[i as usize % fruit.len()]);
        }
        // Bubbles at the surface when it boils.
        if hirviendo {
            for i in 0..4u32 {
                let h = (i + 1).wrapping_mul(0x9e37_79b1) ^ tick.wrapping_mul(0x85eb_ca6b);
                let x = 5 + ((h >> 8) % 15) as i32;
                s.dot(x, 9, rgb(0xff9aa5));
            }
        }
        // The fire under it, between two stones.
        s.rect(2, 20, 3, 2, rgb(0x6b5a4a));
        s.rect(20, 20, 3, 2, rgb(0x6b5a4a));
        for i in 0..4i32 {
            let h = (i as u32 + 7).wrapping_mul(0x9e37_79b1) ^ tick.wrapping_mul(0x27d4_eb2f);
            let top = 20 - ((h >> 11) % 3) as i32;
            let x = 6 + i * 4;
            s.rect(x, top, 3, 22 - top, rgb(0xff8c1a));
            s.dot(x + 1, top.max(20), rgb(0xffd23b));
            s.dot(x + 1, 21, rgb(0xffd23b));
        }
    }
}

impl Widget for Ponche {
    fn name(&self) -> &'static str {
        "ponche --olla"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        // The temperature follows its target with a little wobble; a pinch of
        // fresh fruit cools it for a moment.
        self.target = Self::hot(self.mx) + (self.clock * 0.35).sin() * 4.0;
        self.temp += (self.target - self.temp) * (dt * 0.5).min(1.0);
        self.acc += dt;
        while self.acc >= 0.5 {
            self.acc -= 0.5;
            self.hist.push_back(self.temp);
            while self.hist.len() > 30 {
                self.hist.pop_front();
            }
        }
        self.add -= dt;
        if self.add <= 0.0 {
            self.add = 1.5 + self.rng.f32() * 3.0;
            match self.rng.u8(..8) {
                0..=4 => {
                    let i = self.rng.usize(..8);
                    let n = self.rng.u16(1..5);
                    self.items[i] = self.items[i].saturating_add(n);
                    self.temp -= 0.8 * n as f32;
                    let line = format!("+ {n} {}", INGREDIENTS[i].1);
                    self.say(line);
                }
                5 | 6 => {
                    let who = NINOS[self.rng.usize(..NINOS.len())];
                    let tazas = self.rng.u8(1..4);
                    self.servidas += tazas as u32;
                    self.nivel = (self.nivel - 0.01 * tazas as f32).max(0.2);
                    let line = format!("cucharon: {tazas} tazas servidas a {who}");
                    self.say(line);
                }
                _ => {
                    self.nivel = (self.nivel + 0.08).min(0.95);
                    self.say("+ agua para que no se seque".to_string());
                }
            }
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let pic = r.w >= 52 && r.h >= 13;
        let x0 = if pic { OW as i32 + 3 } else { 0 };
        if pic {
            self.paint(pal);
            let rows = OH as i32 / 2;
            let y0 = r.y + ((r.h - rows) / 2).max(0);
            self.olla.blit(c, r, r.x + 1, y0);
        }
        put(
            c,
            r,
            x0,
            0,
            "PONCHE NAVIDE\u{00d1}O \u{00b7} olla de barro 40 L",
            pal.bright,
            BOLD,
        );
        let mut t = std::mem::take(&mut self.buf);
        let mut y = 2;
        let boil = self.temp > 88.0;
        let col = if boil {
            pal.red
        } else if self.temp > 75.0 {
            pal.amber
        } else {
            pal.green
        };
        t.clear();
        let _ = write!(t, "temperatura {:>5.1} \u{00b0}C ", self.temp);
        let x = put(c, r, x0, y, &t, col, BOLD);
        let bw = (r.w - x - 2).clamp(0, 18);
        bar(c, r, x, y, bw, (self.temp - 20.0) / 80.0, col, pal);
        y += 1;
        let x = put(c, r, x0, y, "tendencia   ", pal.dim, 0);
        let (lo, hi) = self
            .hist
            .iter()
            .fold((f32::MAX, f32::MIN), |a, &v| (a.0.min(v), a.1.max(v)));
        let room = (r.w - x).clamp(0, 30) as usize;
        for (k, &v) in self
            .hist
            .iter()
            .skip(self.hist.len().saturating_sub(room))
            .enumerate()
        {
            let n = if hi > lo + 0.01 {
                ((v - lo) / (hi - lo) * 7.0) as usize
            } else {
                3
            };
            put_ch(c, r, x + k as i32, y, BLK[n.min(7)], col, 0);
        }
        y += 1;
        t.clear();
        let _ = write!(
            t,
            "nivel       {:>3.0}%   servidas {}   {}",
            self.nivel * 100.0,
            self.servidas,
            if boil {
                "HIRVIENDO"
            } else if self.temp > 75.0 {
                "CALIENTITO"
            } else {
                "TIBIO"
            }
        );
        put(c, r, x0, y, &t, pal.main, 0);
        y += 2;
        // What is in it, as many as fit in a line.
        let mut x = x0;
        for (i, (name, _)) in INGREDIENTS.iter().enumerate() {
            t.clear();
            let _ = write!(t, "{name} {}", self.items[i]);
            let w = t.chars().count() as i32;
            if x + w > r.w {
                y += 1;
                x = x0;
            }
            x = put(
                c,
                r,
                x,
                y,
                &t,
                if i % 2 == 0 { pal.main } else { pal.cyan },
                0,
            ) + 2;
        }
        y += 2;
        put(
            c,
            r,
            x0,
            y,
            "piquete     0 ml (hay ni\u{00f1}os presentes)",
            pal.dim,
            0,
        );
        y += 2;
        for line in self.log.iter().rev() {
            if y >= r.h {
                break;
            }
            put_ch(c, r, x0, y, '\u{25b8}', pal.oro, BOLD);
            put(c, r, x0 + 2, y, line, pal.bright, 0);
            y += 1;
        }
        self.buf = t;
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
    fn thousands_get_their_commas() {
        let f = |n| {
            let mut s = String::new();
            thousands(n, &mut s);
            s
        };
        assert_eq!(f(0), "0");
        assert_eq!(f(999), "999");
        assert_eq!(f(1204), "1,204");
        assert_eq!(f(12_345), "12,345");
        assert_eq!(f(1_234_567), "1,234,567");
    }

    #[test]
    fn the_reyes_are_tracked_and_close_in_through_january() {
        let env = env(Fiesta::of_day(1, 2));
        let mut r = Reyes::new(3, Fiesta::of_day(1, 2));
        let text = draw(&mut r, &env, 100, 30).to_text();
        assert!(text.contains("RASTREO DE LOS REYES MAGOS"), "{text}");
        for name in ["MELCHOR", "GASPAR", "BALTASAR"] {
            assert!(text.contains(name), "{name}");
        }
        assert!(text.contains("ETA 06-ENE") && text.contains("T-"), "{text}");
        assert!(text.contains('\u{25c9}') || text.contains('\u{25ce}'));
        // Farther along each day, and there on the 6th.
        let at = |m: u32, d: u32| Reyes::new(3, Fiesta::of_day(m, d)).progress();
        let days = [(12, 12), (12, 24), (1, 1), (1, 2), (1, 4), (1, 5), (1, 6)];
        let along: Vec<f32> = days.iter().map(|&(m, d)| at(m, d)).collect();
        assert!(along.windows(2).all(|w| w[0] < w[1]), "{along:?}");
        assert_eq!(at(1, 6), 1.0);
        let text = draw(&mut Reyes::new(3, Fiesta::of_day(1, 6)), &env, 100, 30).to_text();
        assert!(
            text.contains("HAN LLEGADO") || text.contains("RASTREO"),
            "{text}"
        );
    }

    #[test]
    fn the_cartas_follow_the_posadas() {
        let env = env(Fiesta::of_day(12, 20));
        let mut c = Cartas::new(4, Fiesta::of_day(12, 20));
        let text = draw(&mut c, &env, 110, 30).to_text();
        assert!(text.contains("posada <= 5"), "{text}");
        assert!(
            text.contains("ENTREGADA") && text.contains("ESCRITA"),
            "{text}"
        );
        assert!(text.contains("juguetes") && text.contains("carb\u{00f3}n dulce"));
        // The balloon with the carta climbs beside the table.
        assert!(c.globo.is_set(7, 0) || c.globo.is_set(7, 5));
        let text = draw(&mut Cartas::new(4, Fiesta::of_day(1, 6)), &env, 110, 30).to_text();
        assert!(text.contains("estado = 'ENTREGADA'"), "{text}");
        // More cartas the higher the holiday.
        assert!(
            Cartas::new(1, Fiesta::of_day(12, 12)).cartas
                < Cartas::new(1, Fiesta::of_day(12, 24)).cartas
        );
    }

    #[test]
    fn the_pinata_is_hit_nine_times_and_bursts() {
        let env = env(Fiesta::of_day(12, 24));
        let mut p = Pinata::new(5, Fiesta::of_day(12, 24));
        let text = draw(&mut p, &env, 100, 30).to_text();
        assert!(text.contains("SENSOR DE IMPACTOS"), "{text}");
        assert!(text.contains("golpes:") && text.contains("/9"), "{text}");
        let (mut broke, mut max) = (0, 0);
        let mut was = false;
        for _ in 0..4000 {
            p.update(0.05, &env);
            max = max.max(p.golpes());
            if p.broken() && !was {
                broke += 1;
            }
            was = p.broken();
        }
        assert_eq!(max, 9);
        assert!(broke >= 3, "{broke}");
        // While it bursts the sweets rain.
        let mut p = Pinata::new(5, Fiesta::of_day(12, 24));
        for _ in 0..4000 {
            p.update(0.05, &env);
            if p.broken() {
                break;
            }
        }
        assert!(p.broken());
        let text = draw(&mut p, &env, 100, 30).to_text();
        assert!(text.contains("SE ROMPI"), "{text}");
    }

    #[test]
    fn the_ponche_is_hottest_on_nochebuena() {
        let env = env(Fiesta::of_day(12, 24));
        let temp = |m, d| {
            let mut p = Ponche::new(2, Fiesta::of_day(m, d));
            for _ in 0..400 {
                p.update(0.1, &env);
            }
            p.temperature()
        };
        assert!(temp(12, 12) < temp(12, 24) - 5.0);
        let mut p = Ponche::new(2, Fiesta::of_day(12, 24));
        for _ in 0..100 {
            p.update(0.1, &env);
        }
        let text = draw(&mut p, &env, 100, 30).to_text();
        assert!(text.contains("PONCHE NAVIDE"), "{text}");
        assert!(
            text.contains("temperatura") && text.contains("\u{00b0}C"),
            "{text}"
        );
        assert!(text.contains("tejocotes"), "{text}");
    }
}
