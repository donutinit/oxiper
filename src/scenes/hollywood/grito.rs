//! The widgets only the Fiestas Patrias have: GRITO.EXE, the countdown to
//! 23:00 of September 15 on the scene's own clock (`Env::day`) that, when it
//! runs out, says the Grito line by line; the control panel of the campana
//! de Dolores, a pixel bell that swings and rings; the decibel meter of the
//! ¡VIVA! the crowd answers; the route of the parade of the 16th with its
//! contingents marching; and the history database of the Independence
//! (Hidalgo, Allende, Josefa Ortiz de Domínguez, Morelos, 1810...) with a
//! flag waving beside it. The month builds up: the crowd, the contingents
//! and the rehearsals grow with the level, and every widget follows the same
//! clock, so the bell rings and the meter peaks when the Grito is given.

use super::patrias::{
    CONTINGENTS, EVENTS, GRITO_LINE, GRITO_LINES, HEROES, ROUTE, since_grito, to_grito,
};
use super::posada::{BLK, bar, pixel_line, thousands};
use super::sprite::Sprite;
use super::widgets::glyph;
use super::{Env, Pal, Rect, Widget, put, put_ch, truncate};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Canvas, DIM};
use fastrand::Rng;
use std::collections::VecDeque;
use std::f32::consts::TAU;
use std::fmt::Write;

/// A word in the 3x5 pixel font at (`x`, `y`) of the pane, `sx` columns to a
/// pixel. Returns the next x.
fn big(c: &mut Canvas, r: Rect, x: i32, y: i32, text: &str, sx: i32, col: Color) -> i32 {
    let mut x = x;
    for ch in text.chars() {
        x = big_char(c, r, x, y, ch, sx, col);
    }
    x
}

/// One letter of it. Returns the next x.
fn big_char(c: &mut Canvas, r: Rect, x: i32, y: i32, ch: char, sx: i32, col: Color) -> i32 {
    for (row, bits) in glyph(ch).iter().enumerate() {
        for px in 0..3 {
            if bits >> (2 - px) & 1 == 1 {
                for k in 0..sx {
                    put_ch(c, r, x + px * sx + k, y + row as i32, '\u{2588}', col, 0);
                }
            }
        }
    }
    x + 4 * sx
}

/// Width of a word in the pixel font.
fn big_width(text: &str, sx: i32) -> i32 {
    (text.chars().count() as i32 * 4 * sx - sx).max(0)
}

/// The three colors of the flag, in order: the garland's first three.
fn tricolor(pal: &Pal) -> [Color; 3] {
    [pal.picado[0], pal.picado[1], pal.picado[2]]
}

// ---------------------------------------------------------------- GRITO.EXE

pub struct Cuenta {
    mx: Fiesta,
    rng: Rng,
    clock: f32,
    crowd: f32,
    timer: f32,
    buf: String,
}

impl Cuenta {
    pub fn new(seed: u64, mx: Fiesta) -> Cuenta {
        let mut rng = Rng::with_seed(seed ^ 0x6717e);
        // The Z\u{00f3}calo fills up as the holiday builds up.
        let crowd = 8_000.0 + mx.level * 120_000.0 + rng.f32() * 5_000.0;
        Cuenta {
            mx,
            rng,
            clock: 0.0,
            crowd,
            timer: 0.0,
            buf: String::new(),
        }
    }
}

impl Widget for Cuenta {
    fn name(&self) -> &'static str {
        "grito.exe --palacio-nacional"
    }

    fn update(&mut self, dt: f32, env: &Env) {
        self.clock += dt;
        self.timer -= dt;
        if self.timer <= 0.0 {
            self.timer = 0.4 + self.rng.f32() * 0.8;
            // People arrive all evening, and a lot of them for the Grito.
            let pull = if since_grito(self.mx, env.day).is_some() {
                1.2
            } else {
                0.4
            };
            self.crowd =
                (self.crowd + (self.rng.f32() - 0.35) * 900.0 * pull).clamp(1_000.0, 250_000.0);
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let rem = to_grito(self.mx, env.day);
        let event = since_grito(self.mx, env.day);
        let mut t = std::mem::take(&mut self.buf);
        put(c, r, 0, 0, "GRITO.EXE --palacio-nacional", pal.bright, BOLD);
        let blink = (self.clock * 2.5) as i32 % 2 == 0;
        let (sub, subcol) = if event.is_some() {
            (
                "\u{00a1}EL GRITO EST\u{00c1} EN CURSO!",
                if blink { pal.red } else { pal.oro },
            )
        } else if rem > 0.0 {
            ("cuenta regresiva: 23:00 del 15 de septiembre", pal.cyan)
        } else {
            (
                "GRITO.EXE termin\u{00f3} (exit 0): \u{00a1}VIVA M\u{00c9}XICO!",
                pal.oro,
            )
        };
        put(
            c,
            r,
            0,
            1,
            sub,
            subcol,
            if event.is_some() { BOLD } else { 0 },
        );
        // The flag, as a stripe along the bottom row.
        if r.h >= 8 {
            let [g, w, red] = tricolor(pal);
            for x in 0..r.w {
                let col = [g, w, red][(x * 3 / r.w.max(1)) as usize % 3];
                put_ch(c, r, x, r.h - 1, '\u{2584}', col, 0);
            }
        }
        let avail = r.h - if r.h >= 8 { 1 } else { 0 };
        if let Some(e) = event {
            // The Grito itself, line by line; the last three, big.
            let n = ((e / GRITO_LINE) as usize).min(GRITO_LINES.len() - 1);
            let line = GRITO_LINES[n];
            let viva = n >= 8;
            let sx = if r.w >= 40 { 2 } else { 1 };
            if viva && avail >= 14 && r.w >= big_width("MEXICO", sx) {
                let cols = tricolor(pal);
                let y = 3;
                for (i, word) in ["VIVA", "MEXICO"].iter().enumerate() {
                    let x = (r.w - big_width(word, sx)) / 2;
                    big(c, r, x, y + i as i32 * 6, word, sx, cols[i * 2]);
                }
                // Sparks round the words.
                for k in 0..10 {
                    let h = super::widgets::hash(k, (self.clock * 6.0) as u32);
                    let x = (h % r.w.max(1) as u32) as i32;
                    let yy = 3 + (h / 7 % 11) as i32;
                    put_ch(c, r, x, yy, '\u{2726}', pal.oro, BOLD);
                }
            } else {
                let y = (avail / 2 - 1).max(3);
                let w = line.chars().count() as i32;
                put(c, r, (r.w - w) / 2, y, line, pal.bright, BOLD);
            }
            t.clear();
            let _ = write!(t, "l\u{00ed}nea {}/{}", n + 1, GRITO_LINES.len());
            put(c, r, 0, avail - 1, &t, pal.dim, 0);
            self.buf = t;
            return;
        }
        // The countdown, in big digits when there is room.
        let shown = rem.abs();
        let days = (shown / 86_400.0) as u32;
        let left = shown as u32 % 86_400;
        let digit = |n: u32| char::from(b'0' + (n % 10) as u8);
        let (hh, mm, ss) = (left / 3600, left / 60 % 60, left % 60);
        let clock = [
            digit(hh / 10),
            digit(hh),
            ':',
            digit(mm / 10),
            digit(mm),
            ':',
            digit(ss / 10),
            digit(ss),
        ];
        let cols = tricolor(pal);
        let sx = if r.w >= 66 { 2 } else { 1 };
        let dig = avail >= 15 && r.w >= big_width("00:00:00", 1);
        let mut y = 3;
        if dig {
            let sx = if r.w >= big_width("00:00:00", 2) {
                sx
            } else {
                1
            };
            let mut x = (r.w - big_width("00:00:00", sx)) / 2;
            let col = if rem > 0.0 { None } else { Some(pal.oro) };
            for (i, &ch) in clock.iter().enumerate() {
                let k = col.unwrap_or(cols[i / 3 % 3]);
                let k = if ch == ':' { pal.dim } else { k };
                x = big_char(c, r, x, y, ch, sx, k);
            }
            y += 6;
        } else {
            for (i, &ch) in clock.iter().enumerate() {
                put_ch(
                    c,
                    r,
                    i as i32,
                    3,
                    ch,
                    if rem > 0.0 { pal.bright } else { pal.oro },
                    BOLD,
                );
            }
            y += 2;
        }
        t.clear();
        if rem > 0.0 && days > 0 {
            let _ = write!(t, "T-{days}d");
        } else if rem > 0.0 {
            t.push_str("esta noche");
        } else {
            t.push_str("desde el Grito");
        }
        let tx = if dig {
            (r.w - t.chars().count() as i32) / 2
        } else {
            9
        };
        let ty = if dig { y - 1 } else { 3 };
        put(c, r, tx.max(0), ty, &t, pal.amber, BOLD);
        y += 1;
        // What GRITO.EXE is checking.
        let rows: [(&str, &str); 4] = [
            ("balc\u{00f3}n central", "LISTO"),
            ("campana de Dolores", "LISTA"),
            ("bandera", "ONDEANDO"),
            ("micr\u{00f3}fono", "OK"),
        ];
        let dots = (r.w - 2).clamp(0, 34);
        for (name, state) in rows {
            if y >= avail - 2 {
                break;
            }
            let n = name.chars().count() as i32;
            let x = put(c, r, 0, y, name, pal.main, 0);
            let lead = (dots - n - state.len() as i32 - 2).max(1);
            for k in 0..lead {
                put_ch(c, r, x + 1 + k, y, '.', pal.dim, 0);
            }
            put(c, r, x + lead + 2, y, state, pal.green, BOLD);
            y += 1;
        }
        if y < avail - 1 {
            t.clear();
            thousands(self.crowd as u32, &mut t);
            let x = put(c, r, 0, y, "multitud en el Z\u{00f3}calo ", pal.main, 0);
            put(c, r, x, y, &t, pal.bright, BOLD);
            y += 1;
        }
        if y < avail {
            // The program loading, as the month goes by.
            let frac = 1.0 - (rem / (15.0 * 86_400.0)).clamp(0.0, 1.0);
            let x = put(c, r, 0, y, "GRITO.EXE ", pal.dim, 0);
            let bw = (r.w - x - 6).clamp(0, 30);
            bar(c, r, x, y, bw, frac, pal.ramp.at(0.5 + 0.4 * frac), pal);
            t.clear();
            let _ = write!(t, " {:>3.0}%", frac * 100.0);
            put(c, r, x + bw, y, &t, pal.dim, 0);
        }
        self.buf = t;
    }
}

// ---------------------------------------------------------------- the bell

/// The campana is a pixel sprite, this wide and this tall.
const CW: usize = 32;
const CH: usize = 28;
/// The bell is drawn at this scale of the profile below.
const CK: f32 = 0.8;

pub struct Campana {
    mx: Fiesta,
    rng: Rng,
    pic: Sprite,
    clock: f32,
    /// The swing: phase and amplitude in radians, and the target amplitude.
    phase: f32,
    amp: f32,
    ang: f32,
    /// Strokes of the clapper so far, and which side the last one was on.
    golpes: u32,
    side: bool,
    /// Seconds since the last stroke (the word of it, DING or DONG, is shown).
    since: f32,
    /// Seconds until the next rehearsal, and how long it lasts.
    rehearsal: f32,
    ensayo: f32,
    buf: String,
}

impl Campana {
    pub fn new(seed: u64, mx: Fiesta) -> Campana {
        let mut rng = Rng::with_seed(seed ^ 0xca4a4a);
        Campana {
            mx,
            pic: Sprite::new(CW, CH),
            clock: 0.0,
            phase: 0.0,
            amp: 0.0,
            ang: 0.0,
            golpes: 1000 + rng.u32(..80),
            side: false,
            since: 9.0,
            rehearsal: 2.0 + rng.f32() * 6.0,
            ensayo: 0.0,
            rng,
            buf: String::new(),
        }
    }

    /// The amplitude the bell is driven to: rung by the Grito, at a
    /// rehearsal, or at rest.
    fn target(&self, env: &Env) -> (f32, &'static str) {
        if let Some(e) = since_grito(self.mx, env.day) {
            // Rung as the Grito begins, and once more as it ends.
            if !(8.0..40.0).contains(&e) {
                return (0.32, "REPICANDO (modo Grito)");
            }
            return (0.0, "EN ESPERA (el Grito sigue)");
        }
        if self.ensayo > 0.0 {
            return (0.12 + 0.12 * self.mx.level, "ENSAYO DE REPIQUE");
        }
        (0.0, "EN REPOSO")
    }

    #[cfg(test)]
    pub fn golpes(&self) -> u32 {
        self.golpes
    }

    /// Paints the bell on its yoke: the profile is tested pixel by pixel
    /// after rotating back about the pivot, so it fills at any angle.
    fn paint(&mut self, pal: &Pal) {
        let s = &mut self.pic;
        s.clear();
        let madera = rgb(0x8b5a2b);
        let madera_o = rgb(0x5c3a1c);
        let (cx, piv) = (CW as f32 / 2.0, 1.5);
        s.rect(0, 0, CW as i32, 2, madera);
        s.rect(0, 0, 2, 5, madera_o);
        s.rect(CW as i32 - 2, 0, 2, 5, madera_o);
        let (sin, cos) = self.ang.sin_cos();
        let (oro, claro, oscuro, banda) = (
            pal.oro,
            pal.oro.scale(1.3),
            pal.oro.scale(0.62),
            pal.oro.scale(0.8),
        );
        for y in 2..CH as i32 {
            for x in 0..CW as i32 {
                let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - piv);
                let lx = (dx * cos + dy * sin) / CK;
                let ly = (-dx * sin + dy * cos) / CK;
                let hw = match ly {
                    l if l < 2.0 => continue,
                    l if l < 5.0 => 1.8,
                    l if l < 8.0 => 1.8 + (l - 5.0) * 0.9,
                    l if l < 17.0 => 4.5 + (l - 8.0) * 0.35,
                    l if l < 22.0 => 7.65 + (l - 17.0) * 0.75,
                    l if l < 23.6 => 11.5,
                    _ => continue,
                };
                if lx.abs() > hw {
                    continue;
                }
                let col = if ly >= 22.0 || (18.0..19.2).contains(&ly) {
                    banda
                } else if lx < -hw * 0.5 {
                    claro
                } else if lx > hw * 0.55 {
                    oscuro
                } else {
                    oro
                };
                s.dot(x, y, col);
            }
        }
        // The clapper hangs inside it, lagging behind the swing, and a rope
        // trails from it.
        let lag = self.ang * 0.5;
        let (kx, ky) = (cx - 21.0 * lag.sin(), piv + 21.0 * lag.cos());
        s.rect(kx.round() as i32 - 1, ky.round() as i32, 3, 2, madera_o);
        pixel_line(
            s,
            kx,
            ky + 2.0,
            kx - 0.5 * lag.sin(),
            CH as f32 - 1.0,
            rgb(0xc9b26b),
        );
    }
}

impl Widget for Campana {
    fn name(&self) -> &'static str {
        "campana-dolores --panel"
    }

    fn update(&mut self, dt: f32, env: &Env) {
        self.clock += dt;
        self.since += dt;
        // Rehearsals every so often, the bell swinging a few seconds.
        if self.ensayo > 0.0 {
            self.ensayo -= dt;
        } else {
            self.rehearsal -= dt;
            if self.rehearsal <= 0.0 {
                self.rehearsal = 14.0 + self.rng.f32() * 14.0 - 8.0 * self.mx.level;
                self.ensayo = 4.0 + self.rng.f32() * 2.0;
            }
        }
        let (target, _) = self.target(env);
        self.amp += (target - self.amp) * (dt * 1.8).min(1.0);
        let before = self.phase.sin();
        self.phase = (self.phase + dt * TAU * 0.55) % TAU;
        let now = self.phase.sin();
        self.ang = self.amp * now;
        // The clapper strikes at the end of each swing.
        if self.amp > 0.05 && before.signum() != now.signum() {
            self.golpes += 1;
            self.side = !self.side;
            self.since = 0.0;
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let pic = r.w >= 60 && r.h >= 12;
        let x0 = if pic { CW as i32 + 3 } else { 0 };
        if pic {
            self.paint(pal);
            let rows = CH as i32 / 2;
            let y0 = r.y + ((r.h - rows) / 2).max(0);
            self.pic.blit(c, r, r.x + 1, y0);
        }
        put(c, r, x0, 0, "CAMPANA DE DOLORES", pal.bright, BOLD);
        let (_, state) = self.target(env);
        let ringing = self.amp > 0.05;
        let mut t = std::mem::take(&mut self.buf);
        let mut y = 2;
        put(c, r, x0, y, "estado       ", pal.dim, 0);
        put(
            c,
            r,
            x0 + 13,
            y,
            state,
            if ringing { pal.oro } else { pal.green },
            BOLD,
        );
        y += 1;
        t.clear();
        let _ = write!(t, "{:+5.1}\u{00b0}", self.ang.to_degrees());
        put(c, r, x0, y, "\u{00e1}ngulo       ", pal.dim, 0);
        let x = put(c, r, x0 + 13, y, &t, pal.main, 0);
        // A pendulum: the marker travels along the dial with the swing.
        let dial = (r.w - x - 3).clamp(0, 21);
        if dial >= 7 {
            put_ch(c, r, x + 1, y, '[', pal.dim, 0);
            for k in 0..dial {
                put_ch(c, r, x + 2 + k, y, '\u{2500}', pal.dim, 0);
            }
            let mid = x + 2 + dial / 2;
            put_ch(c, r, mid, y, '\u{253c}', pal.dim, 0);
            let at = mid + (self.ang / 0.35 * (dial / 2) as f32).round() as i32;
            put_ch(c, r, at, y, '\u{25c6}', pal.oro, BOLD);
            put_ch(c, r, x + 2 + dial, y, ']', pal.dim, 0);
        }
        y += 1;
        t.clear();
        let _ = write!(t, "{:.2} Hz", 0.55);
        put(c, r, x0, y, "frecuencia   ", pal.dim, 0);
        put(c, r, x0 + 13, y, &t, pal.main, 0);
        y += 1;
        t.clear();
        thousands(self.golpes, &mut t);
        put(c, r, x0, y, "badajo       ", pal.dim, 0);
        let x = put(c, r, x0 + 13, y, "golpes ", pal.main, 0);
        put(c, r, x, y, &t, pal.bright, BOLD);
        y += 1;
        t.clear();
        let _ = write!(t, "{:.0} N", 36.0 + self.amp * 90.0);
        put(c, r, x0, y, "cuerda       ", pal.dim, 0);
        put(c, r, x0 + 13, y, "tensi\u{00f3}n ", pal.main, 0);
        put(c, r, x0 + 22, y, &t, pal.main, 0);
        y += 1;
        put(c, r, x0, y, "origen       ", pal.dim, 0);
        put(c, r, x0 + 13, y, "Dolores, Guanajuato, 1810", pal.main, 0);
        y += 2;
        // What it says at each stroke.
        if self.since < 0.9 {
            let word = if self.side { "D O N G" } else { "D I N G" };
            let k = 1.0 - self.since / 0.9;
            put(c, r, x0, y, word, pal.fade(pal.oro, 1.0 - k), BOLD);
        } else if ringing {
            put(c, r, x0, y, "\u{266a}", pal.dim, 0);
        }
        y += 2;
        if y < r.h {
            put(
                c,
                r,
                x0,
                y,
                "En la tradici\u{00f3}n, Hidalgo la toc\u{00f3} para convocar al pueblo.",
                pal.dim,
                DIM,
            );
        }
        self.buf = t;
    }
}

// ---------------------------------------------------------------- the vivas

pub struct Vivas {
    mx: Fiesta,
    rng: Rng,
    clock: f32,
    db: f32,
    peak: f32,
    peak_age: f32,
    hist: VecDeque<f32>,
    acc: f32,
    /// Seconds until the crowd's next ¡VIVA!, and the one in progress.
    next: f32,
    viva: f32,
    boost: f32,
    vivas: u32,
    /// The line of the Grito whose answer was already given.
    answered: Option<usize>,
    buf: String,
}

impl Vivas {
    pub fn new(seed: u64, mx: Fiesta) -> Vivas {
        let mut rng = Rng::with_seed(seed ^ 0x71744);
        let base = Self::ambient(mx);
        Vivas {
            mx,
            clock: 0.0,
            db: base,
            peak: base,
            peak_age: 0.0,
            hist: VecDeque::from(vec![base; 40]),
            acc: 0.0,
            next: 1.0 + rng.f32() * 3.0,
            viva: 0.0,
            boost: 0.0,
            vivas: 300 + (mx.level * 4000.0) as u32,
            answered: None,
            buf: String::new(),
            rng,
        }
    }

    /// The noise of the crowd without a viva, louder the higher the holiday.
    fn ambient(mx: Fiesta) -> f32 {
        62.0 + 22.0 * mx.level
    }

    #[cfg(test)]
    pub fn peak(&self) -> f32 {
        self.peak
    }

    #[cfg(test)]
    pub fn vivas(&self) -> u32 {
        self.vivas
    }
}

impl Widget for Vivas {
    fn name(&self) -> &'static str {
        "vivometro --zocalo"
    }

    fn update(&mut self, dt: f32, env: &Env) {
        self.clock += dt;
        self.next -= dt;
        self.viva = (self.viva - dt).max(0.0);
        // The crowd answers a viva of the Grito when the line is nearly said.
        if let Some(e) = since_grito(self.mx, env.day) {
            let n = ((e / GRITO_LINE) as usize).min(GRITO_LINES.len() - 1);
            let into = e - n as f32 * GRITO_LINE;
            if n >= 1 && into > GRITO_LINE * 0.62 && self.answered != Some(n) {
                self.answered = Some(n);
                self.viva = 1.1;
                self.boost = if n >= 8 { 60.0 } else { 42.0 };
                self.vivas += self.rng.u32(900..4000);
            }
        } else if self.next <= 0.0 {
            // On any other night, the odd viva from somebody in the crowd.
            self.next = 4.0 + self.rng.f32() * 7.0 - 3.0 * self.mx.level;
            self.viva = 0.8;
            self.boost = 14.0 + self.rng.f32() * 14.0;
            self.vivas += self.rng.u32(2..40);
        }
        let target = Self::ambient(self.mx)
            + (self.clock * 1.7).sin() * 3.0
            + (self.clock * 5.3).sin() * 1.5
            + if self.viva > 0.0 { self.boost } else { 0.0 };
        self.db += (target - self.db) * (dt * 7.0).min(1.0);
        if self.db > self.peak {
            self.peak = self.db;
            self.peak_age = 0.0;
        } else {
            self.peak_age += dt;
            if self.peak_age > 3.5 {
                self.peak = (self.peak - dt * 8.0).max(self.db);
            }
        }
        self.acc += dt;
        while self.acc >= 0.15 {
            self.acc -= 0.15;
            self.hist.push_back(self.db);
            while self.hist.len() > 60 {
                self.hist.pop_front();
            }
        }
        self.acc = self.acc.min(0.15);
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        put(
            c,
            r,
            0,
            0,
            "VIV\u{00d3}METRO DEL Z\u{00d3}CALO",
            pal.bright,
            BOLD,
        );
        let zone = |db: f32| {
            if db < 85.0 {
                pal.green
            } else if db < 110.0 {
                pal.amber
            } else {
                pal.red
            }
        };
        let mut t = std::mem::take(&mut self.buf);
        let mut y = 2;
        // The level in big digits when there is room for them.
        let big_ok = r.h >= 15 && r.w >= 34;
        if big_ok {
            t.clear();
            let _ = write!(t, "{:03}", self.db.round() as u32);
            let x = big(c, r, 0, y, &t, 1, zone(self.db));
            put(c, r, x + 1, y + 4, "dB", pal.dim, 0);
            if self.viva > 0.0 {
                let blink = (self.clock * 8.0) as i32 % 2 == 0;
                put(
                    c,
                    r,
                    x + 6,
                    y + 2,
                    "\u{00a1}VIVA!",
                    if blink { pal.oro } else { pal.bright },
                    BOLD,
                );
            }
            y += 7;
        } else {
            t.clear();
            let _ = write!(t, "nivel {:>6.1} dB", self.db);
            put(c, r, 0, y, &t, zone(self.db), BOLD);
            if self.viva > 0.0 {
                put(c, r, 20, y, "\u{00a1}VIVA!", pal.oro, BOLD);
            }
            y += 1;
        }
        // The bar from 40 to 140 dB in three zones, with the peak held.
        let bw = (r.w - 2).clamp(4, 60);
        let frac = |db: f32| ((db - 40.0) / 100.0).clamp(0.0, 1.0);
        let filled = (frac(self.db) * bw as f32) as i32;
        for i in 0..bw {
            let db = 40.0 + i as f32 / bw as f32 * 100.0;
            if i < filled {
                put_ch(c, r, i, y, '\u{2588}', zone(db), 0);
            } else {
                put_ch(c, r, i, y, '\u{2591}', pal.dim, 0);
            }
        }
        let pk = (frac(self.peak) * (bw - 1) as f32) as i32;
        put_ch(c, r, pk, y, '\u{2502}', pal.bright, BOLD);
        y += 1;
        t.clear();
        let _ = write!(t, "pico {:>6.1} dB", self.peak);
        put(c, r, 0, y, &t, pal.dim, 0);
        y += 2;
        // The last few seconds, as bars.
        let room = (r.w as usize).min(60);
        for (k, &v) in self
            .hist
            .iter()
            .skip(self.hist.len().saturating_sub(room))
            .enumerate()
        {
            let n = ((v - 55.0) / 85.0 * 7.0).clamp(0.0, 7.0) as usize;
            put_ch(c, r, k as i32, y, BLK[n], zone(v), 0);
        }
        y += 2;
        t.clear();
        t.push_str("vivas detectadas ");
        thousands(self.vivas, &mut t);
        put(c, r, 0, y, &t, pal.main, 0);
        y += 1;
        put(
            c,
            r,
            0,
            y,
            if self.db > 120.0 {
                "protecci\u{00f3}n auditiva: OBLIGATORIA"
            } else if self.db > 95.0 {
                "protecci\u{00f3}n auditiva: RECOMENDADA"
            } else {
                "protecci\u{00f3}n auditiva: opcional"
            },
            if self.db > 120.0 { pal.red } else { pal.dim },
            0,
        );
        self.buf = t;
    }
}

// ---------------------------------------------------------------- the parade

/// How long the route is, in kilometers.
const ROUTE_KM: f32 = 4.2;
/// Seconds between contingents, and each one's walk along the route.
const GAP: f32 = 6.0;
const WALK: f32 = 40.0;

pub struct Desfile {
    mx: Fiesta,
    clock: f32,
    /// Where each contingent is, 0 (the Zócalo) to 1 (the end of the route).
    at: Vec<f32>,
    buf: String,
}

impl Desfile {
    pub fn new(seed: u64, mx: Fiesta) -> Desfile {
        let mut d = Desfile {
            mx,
            // Start the parade at a different point for each one.
            clock: (seed % 60) as f32,
            at: vec![0.0; CONTINGENTS.len()],
            buf: String::new(),
        };
        d.place();
        d
    }

    /// How many contingents take part: more the higher the holiday, all of
    /// them on the 16th.
    pub fn marching(&self) -> usize {
        if self.mx.date() == (9, 16) {
            CONTINGENTS.len()
        } else {
            6 + (self.mx.level * 6.0) as usize
        }
    }

    fn cycle(&self) -> f32 {
        (self.marching() as f32 - 1.0) * GAP + WALK + 10.0
    }

    #[cfg(test)]
    pub fn on_route(&self) -> usize {
        self.at.iter().filter(|&&p| p > 0.0 && p < 1.0).count()
    }

    fn place(&mut self) {
        let t = self.clock % self.cycle();
        let n = self.marching();
        for (i, p) in self.at.iter_mut().enumerate() {
            *p = if i < n {
                ((t - i as f32 * GAP) / WALK).clamp(-0.01, 1.0)
            } else {
                -1.0
            };
        }
    }
}

impl Widget for Desfile {
    fn name(&self) -> &'static str {
        "desfile --16-sep"
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
            "DESFILE MILITAR DEL 16 DE SEPTIEMBRE",
            pal.bright,
            BOLD,
        );
        let live = self.mx.date() == (9, 16);
        let mut t = std::mem::take(&mut self.buf);
        t.clear();
        if live {
            let _ = write!(t, "EN CURSO \u{00b7} ruta {ROUTE_KM:.1} km");
        } else {
            let _ = write!(t, "ensayo general \u{00b7} ruta {ROUTE_KM:.1} km");
        }
        let tw = t.chars().count() as i32;
        if r.w > 38 + tw {
            put(
                c,
                r,
                r.w - tw,
                0,
                &t,
                if live { pal.oro } else { pal.cyan },
                BOLD,
            );
        }
        // The route, with its stretches named and the contingents on it.
        let lw = (r.w - 1).max(2);
        if r.h >= 6 {
            let n = ROUTE.len() as i32;
            for (i, name) in ROUTE.iter().enumerate() {
                let x = i as i32 * (lw - 1) / (n - 1);
                let w = name.chars().count() as i32;
                let nx = (x - w / 2).clamp(0, (lw - w).max(0));
                if r.w >= 60 || i % 2 == 0 {
                    put(c, r, nx, 2, name, pal.cyan, 0);
                }
                put_ch(c, r, x, 3, '\u{253c}', pal.dim, 0);
            }
            for x in 0..lw {
                if c.get(r.x + x, r.y + 3).is_some_and(|cell| cell.ch == ' ') {
                    put_ch(c, r, x, 3, '\u{2500}', pal.dim, 0);
                }
            }
            let [g, w, red] = tricolor(pal);
            for (i, &p) in self.at.iter().enumerate() {
                if p > 0.0 && p < 1.0 {
                    let x = (p * (lw - 1) as f32) as i32;
                    put_ch(c, r, x, 3, '\u{25b6}', [g, w, red][i % 3], BOLD);
                }
            }
            put_ch(c, r, 0, 3, '\u{25cf}', pal.oro, BOLD);
            put_ch(c, r, lw - 1, 3, '\u{25cf}', pal.oro, BOLD);
        }
        // The contingents, in order, as many as fit.
        let mut y = 5;
        if r.h > 6 {
            t.clear();
            let _ = write!(
                t,
                "{:<26}{:>10}{:>7}   estado",
                "contingente", "marchistas", "km"
            );
            put(c, r, 0, y - 1, &t, pal.cyan, BOLD);
        }
        let mut total = 0u32;
        for (i, (&(name, people), &p)) in CONTINGENTS.iter().zip(&self.at).enumerate() {
            let on = p > 0.0 && p < 1.0;
            if on {
                total += people as u32;
            }
            if y >= r.h - 1 {
                continue;
            }
            if p < 0.0 && i >= self.marching() {
                continue;
            }
            t.clear();
            let _ = write!(t, "{:<26}", truncate(name, 25));
            let _ = write!(t, "{people:>10}{:>7.1}   ", p.clamp(0.0, 1.0) * ROUTE_KM);
            let (state, col) = if p >= 1.0 {
                ("LLEG\u{00d3} \u{2713}", pal.oro)
            } else if on {
                ("EN MARCHA \u{25b6}", pal.green)
            } else {
                ("EN ESPERA", pal.dim)
            };
            let x = put(c, r, 0, y, &t, if on { pal.bright } else { pal.main }, 0);
            put(c, r, x, y, state, col, if on { BOLD } else { 0 });
            y += 1;
        }
        if r.h >= 4 {
            t.clear();
            let _ = write!(t, "marchistas en la ruta {total}");
            put(c, r, 0, r.h - 1, &t, pal.dim, 0);
        }
        self.buf = t;
    }
}

// ---------------------------------------------------------------- history

/// The waving flag is a pixel sprite, this wide and this tall.
const FW: usize = 31;
const FH: usize = 27;

#[derive(Clone, Copy)]
enum Item {
    Hero(usize),
    Event(usize),
}

struct Row {
    item: Item,
    flash: f32,
}

pub struct Historia {
    mx: Fiesta,
    rng: Rng,
    order: Vec<Item>,
    next: usize,
    rows: VecDeque<Row>,
    timer: f32,
    clock: f32,
    flag: Sprite,
    consultas: u32,
    buf: String,
}

impl Historia {
    pub fn new(seed: u64, mx: Fiesta) -> Historia {
        let mut rng = Rng::with_seed(seed ^ 0x4157);
        // On the night of the Grito, the dates that lead up to it (1808 to
        // 1821), not the later ones.
        let night = mx.date() == (9, 15);
        let mut order: Vec<Item> = HEROES
            .iter()
            .enumerate()
            .map(|(i, _)| Item::Hero(i))
            .chain(
                EVENTS
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !night || e.0 <= 1821)
                    .map(|(i, _)| Item::Event(i)),
            )
            .collect();
        rng.shuffle(&mut order);
        let mut h = Historia {
            mx,
            rng,
            order,
            next: 0,
            rows: VecDeque::new(),
            timer: 0.4,
            clock: 0.0,
            flag: Sprite::new(FW, FH),
            consultas: 1810 + (mx.level * 900.0) as u32,
            buf: String::new(),
        };
        for _ in 0..30 {
            h.add();
            if let Some(r) = h.rows.back_mut() {
                r.flash = 0.0;
            }
        }
        h
    }

    fn add(&mut self) {
        let item = self.order[self.next % self.order.len()];
        self.next += 1;
        if self.next.is_multiple_of(self.order.len()) {
            self.rng.shuffle(&mut self.order);
        }
        self.rows.push_back(Row { item, flash: 1.0 });
        // (No more rows than there are things to show, so none repeats on screen.)
        while self.rows.len() > self.order.len().min(40) {
            self.rows.pop_front();
        }
    }

    fn query(&self, narrow: bool, out: &mut String) {
        out.push_str(if narrow {
            "independencia=# SELECT * FROM historia;"
        } else if self.mx.date() == (9, 15) {
            "independencia=# SELECT * FROM historia WHERE a\u{00f1}o BETWEEN 1808 AND 1821;"
        } else if self.mx.date() == (9, 16) {
            "independencia=# SELECT * FROM historia WHERE evento LIKE '%Grito%';"
        } else {
            "independencia=# SELECT * FROM historia ORDER BY a\u{00f1}o;"
        });
    }

    /// The flag of Mexico, waving on its pole: three bands, a gold ring and a
    /// brown eagle in the middle, the cloth rippling more the farther from
    /// the pole.
    fn paint_flag(&mut self, pal: &Pal) {
        let s = &mut self.flag;
        s.clear();
        let [g, w, red] = tricolor(pal);
        let (fw, fh) = (FW as i32 - 3, FH as i32 - 8);
        let t = self.clock * 4.0;
        for x in 0..fw {
            let k = x as f32 / fw as f32;
            let off = ((x as f32 * 0.42 - t).sin() * 1.6 * k).round() as i32;
            // The folds catch the light.
            let shade = 1.0 + ((x as f32 * 0.42 - t).cos() * 0.16 * k);
            for y in 0..fh {
                let band = (x * 3 / fw) as usize;
                let col = [g, w, red][band.min(2)].scale(shade);
                s.dot(3 + x, 3 + y + off, col);
            }
            // The emblem: a ring of oro round the eagle.
            if (x - fw / 2).abs() <= 4 {
                let cy = 3 + fh / 2 + off;
                for dy in -4..=4i32 {
                    let d = (((x - fw / 2).pow(2) + dy * dy) as f32).sqrt();
                    if (3.0..=4.2).contains(&d) {
                        s.dot(3 + x, cy + dy, pal.oro);
                    } else if d < 3.0 {
                        s.dot(3 + x, cy + dy, rgb(0x7a4a1e));
                    }
                }
                s.dot(3 + x, cy + 5, rgb(0x4f8a2a));
            }
        }
        s.rect(1, 1, 2, FH as i32 - 1, rgb(0x8b5a2b));
        s.rect(0, 0, 4, 2, pal.oro);
    }
}

impl Widget for Historia {
    fn name(&self) -> &'static str {
        "psql independencia"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.clock += dt;
        for r in self.rows.iter_mut() {
            r.flash = (r.flash - dt * 0.8).max(0.0);
        }
        self.timer -= dt;
        if self.timer <= 0.0 {
            // Faster the higher the holiday.
            self.timer = (0.5 + self.rng.f32() * 1.3) * (1.5 - 0.7 * self.mx.level);
            self.add();
            self.consultas += 1;
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let flag = r.w >= 84 && r.h >= 14;
        let tw = if flag { r.w - FW as i32 - 2 } else { r.w };
        let table = Rect { w: tw, ..r };
        let mut line = std::mem::take(&mut self.buf);
        line.clear();
        self.query(tw < 56, &mut line);
        let x = put(c, table, 0, 0, &line, pal.bright, BOLD);
        let cursor = if (self.clock * 2.0) as i32 % 2 == 0 {
            "\u{258c}"
        } else {
            " "
        };
        put(c, table, x + 1, 0, cursor, pal.oro, 0);
        let name_w = if tw >= 70 { 26 } else { 22 };
        put(
            c,
            table,
            0,
            1,
            if tw >= 60 {
                "a\u{00f1}o        qui\u{00e9}n o qu\u{00e9}"
            } else {
                "a\u{00f1}o        qui\u{00e9}n"
            },
            pal.cyan,
            BOLD,
        );
        for x in 0..tw {
            put_ch(c, table, x, 2, '\u{2500}', pal.dim, 0);
        }
        let rows = (r.h - 4).max(0) as usize;
        for (i, row) in self.rows.iter().rev().take(rows).enumerate() {
            let y = r.h - 2 - i as i32;
            let hot = row.flash > 0.05;
            let base = if hot { pal.bright } else { pal.main };
            let bold = if hot { BOLD } else { 0 };
            match row.item {
                Item::Hero(h) => {
                    let hero = &HEROES[h];
                    put(c, table, 0, y, hero.years, pal.cyan, 0);
                    let x = put(c, table, 10, y, truncate(hero.name, name_w), base, bold);
                    if tw >= 60 {
                        line.clear();
                        let _ = write!(line, "  {} ({})", hero.role, hero.place);
                        put(
                            c,
                            table,
                            x,
                            y,
                            truncate(&line, (tw - x).max(0) as usize),
                            pal.dim,
                            0,
                        );
                    }
                }
                Item::Event(e) => {
                    let (year, what, place) = EVENTS[e];
                    line.clear();
                    let _ = write!(line, "{year}");
                    put(c, table, 0, y, &line, pal.amber, 0);
                    let x = put(
                        c,
                        table,
                        10,
                        y,
                        truncate(what, (tw - 10).max(0) as usize),
                        base,
                        bold,
                    );
                    if tw >= 76 {
                        line.clear();
                        let _ = write!(line, "  ({place})");
                        put(
                            c,
                            table,
                            x,
                            y,
                            truncate(&line, (tw - x).max(0) as usize),
                            pal.dim,
                            0,
                        );
                    }
                }
            }
        }
        line.clear();
        let _ = write!(
            line,
            "({} filas)  h\u{00e9}roes {}  fechas {}  consultas {}",
            self.rows.len(),
            HEROES.len(),
            EVENTS.len(),
            self.consultas
        );
        put(c, table, 0, r.h - 1, &line, pal.dim, 0);
        self.buf = line;
        if flag {
            self.paint_flag(pal);
            let rows = FH as i32 / 2;
            let y0 = r.y + ((r.h - rows) / 2).max(0);
            self.flag.blit(c, r, r.x + r.w - FW as i32, y0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::patrias;
    use super::*;
    use crate::scene::Ctx;
    use crate::theme::Theme;

    fn env(fiesta: Fiesta, day: f32) -> Env {
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
            day,
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
    fn the_countdown_counts_days_and_the_grito_is_said_line_by_line() {
        let night = Fiesta::of_day(9, 15);
        let mut e = env(night, patrias::GRITO_AT - 3.0 * 3600.0 - 5.0);
        let mut g = Cuenta::new(3, night);
        let text = draw(&mut g, &e, 100, 30).to_text();
        assert!(text.contains("GRITO.EXE"), "{text}");
        assert!(text.contains("23:00 del 15 de septiembre"), "{text}");
        assert!(text.contains("esta noche") && text.contains("campana de Dolores"));
        // Days before, the days are counted.
        let e0 = env(Fiesta::of_day(9, 10), 8.0 * 3600.0);
        let text = draw(&mut Cuenta::new(3, Fiesta::of_day(9, 10)), &e0, 100, 30).to_text();
        assert!(text.contains("T-5d"), "{text}");
        // At 23:00 the Grito: every line in order, the last three big.
        let mut seen = String::new();
        for step in 0..120 {
            e.day = patrias::GRITO_AT + step as f32 * 0.5;
            g.update(0.5, &e);
            seen.push_str(&draw(&mut g, &e, 100, 30).to_text());
        }
        assert!(seen.contains("EL GRITO EST"), "{seen}");
        for line in &GRITO_LINES[..8] {
            assert!(seen.contains(line), "{line}");
        }
        assert!(seen.contains("\u{2588}"), "the big VIVA MEXICO");
        // After it, the program has finished.
        e.day = patrias::GRITO_AT + 120.0;
        let text = draw(&mut g, &e, 100, 30).to_text();
        assert!(text.contains("termin"), "{text}");
    }

    #[test]
    fn the_bell_rings_when_the_grito_is_given_and_rehearses_otherwise() {
        let night = Fiesta::of_day(9, 15);
        let mut e = env(night, patrias::GRITO_AT - 60.0);
        let mut b = Campana::new(2, night);
        let at_rest = b.golpes();
        for _ in 0..200 {
            e.day += 0.1;
            b.update(0.1, &e);
        }
        // (Only the odd rehearsal until the Grito.)
        let before = b.golpes();
        assert!(before - at_rest < 40);
        let mut max_amp = 0.0f32;
        for _ in 0..80 {
            e.day = patrias::GRITO_AT + 1.0 + (b.clock % 6.0);
            b.update(0.1, &e);
            max_amp = max_amp.max(b.amp);
        }
        assert!(max_amp > 0.2, "{max_amp}");
        assert!(b.golpes() - before > 5);
        let text = draw(&mut b, &e, 100, 24).to_text();
        assert!(
            text.contains("CAMPANA DE DOLORES") && text.contains("badajo"),
            "{text}"
        );
        // The bell is on screen (a sprite of half blocks) in a wide pane.
        assert!(
            text.contains('\u{2580}') || text.contains('\u{2584}') || text.contains('\u{2588}')
        );
        // A quiet day still has its rehearsals.
        let e2 = env(Fiesta::of_day(9, 3), 9.0 * 3600.0);
        let mut b = Campana::new(2, Fiesta::of_day(9, 3));
        let start = b.golpes();
        for _ in 0..1200 {
            b.update(0.1, &e2);
        }
        assert!(b.golpes() > start);
    }

    #[test]
    fn the_crowd_answers_each_viva_of_the_grito() {
        let night = Fiesta::of_day(9, 15);
        let mut e = env(night, patrias::GRITO_AT - 30.0);
        let mut v = Vivas::new(4, night);
        let ambient = v.peak();
        let before = v.vivas();
        for step in 0..600 {
            e.day = patrias::GRITO_AT - 30.0 + step as f32 * 0.1;
            v.update(0.1, &e);
        }
        // Before the Grito, only the odd viva; during it, the peaks.
        assert!(v.peak() > ambient);
        let mut loudest = 0.0f32;
        for step in 0..560 {
            e.day = patrias::GRITO_AT + step as f32 * 0.1;
            v.update(0.1, &e);
            loudest = loudest.max(v.peak());
        }
        assert!(v.vivas() - before > 6_000, "{}", v.vivas() - before);
        assert!(loudest > 120.0, "{loudest}");
        let text = draw(&mut v, &e, 90, 24).to_text();
        assert!(text.contains("VIV") && text.contains("dB") && text.contains("vivas detectadas"));
    }

    #[test]
    fn the_parade_marches_along_its_route() {
        let e = env(Fiesta::of_day(9, 16), 0.0);
        let mut d = Desfile::new(1, Fiesta::of_day(9, 16));
        let mut max = 0;
        for _ in 0..1200 {
            d.update(0.1, &e);
            max = max.max(d.on_route());
        }
        assert!(max >= 4, "{max}");
        let text = draw(&mut d, &e, 100, 30).to_text();
        assert!(
            text.contains("DESFILE MILITAR DEL 16 DE SEPTIEMBRE"),
            "{text}"
        );
        assert!(
            text.contains("EN CURSO") && text.contains("Z\u{00f3}calo"),
            "{text}"
        );
        assert!(text.contains("Heroico Colegio Militar"));
        // Before the 16th it is a rehearsal, with fewer contingents.
        let rehearsal = Desfile::new(1, Fiesta::of_day(9, 3));
        assert!(rehearsal.marching() < d.marching());
        let text = draw(&mut Desfile::new(1, Fiesta::of_day(9, 3)), &e, 100, 30).to_text();
        assert!(text.contains("ensayo general"));
    }

    #[test]
    fn the_history_knows_the_heroes_and_follows_the_date() {
        let e = env(Fiesta::of_day(9, 15), 0.0);
        let mut h = Historia::new(2, Fiesta::of_day(9, 15));
        for _ in 0..200 {
            h.update(0.5, &e);
        }
        let text = draw(&mut h, &e, 110, 30).to_text();
        assert!(text.contains("BETWEEN 1808 AND 1821"), "{text}");
        assert!(text.contains("1810"), "{text}");
        // The waving flag: green, white and red.
        assert!(text.contains('\u{2580}') || text.contains('\u{2588}'));
        let mut seen = String::new();
        let mut h = Historia::new(2, Fiesta::of_day(9, 3));
        for _ in 0..400 {
            h.update(0.5, &e);
            seen.push_str(&draw(&mut h, &e, 110, 30).to_text());
        }
        for name in ["Hidalgo", "Allende", "Josefa Ortiz", "Morelos"] {
            assert!(seen.contains(name), "{name}");
        }
        assert!(seen.contains("ORDER BY a\u{00f1}o"));
    }
}
