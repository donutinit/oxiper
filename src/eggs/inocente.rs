//! A fake system failure takes over the screen on the Día de los Santos
//! Inocentes (December 28), and turns out to be a prank.
//!
//! On the 28th Mexicans play pranks on each other, and whoever falls for one
//! is told "¡Inocente palomita, que te dejaste engañar!" (innocent little
//! dove, you let yourself be fooled). So the screen glitches: rows tear
//! sideways, noise crackles, everything turns red and a frightening error
//! box opens, a bar running down and a countdown to a forced restart. For a
//! couple of seconds it could be real, then the log turns silly (the cause
//! is too many buñuelos, the culprit a virus called SANTOS-INOCENTES v28.12,
//! the backup of ponche not found), the countdown reaches zero, the screen
//! flashes and the truth comes out on a banner of papel de china colors
//! while confetti falls and a little dove flaps in to say it again. Then it
//! all goes away, the dove last. It is over within 20 s, and the glitches
//! never last long enough to be taken for a real crash.

use super::navidad::{Palette, fade, split_in_two};
use super::{Egg, Info, pixel, sprite};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas, Cell};
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "inocente",
    start,
    fiesta: Kind::Navidad,
    // Only the prank day.
    when: |fiesta| fiesta.inocentes(),
};

type Frames = [&'static [&'static str]; 3];

/// The little dove (la palomita), facing right, in three beats of its
/// wings: up, level and down.
const DOVE: Frames = [
    &[
        "....WW.............",
        "...WWWW............",
        "..WWWwWW...........",
        "..WWWwWWW....WWW...",
        ".WWW.wWWWWWWWWWkW..",
        "WW...WWWWWWWWWWWWOO",
        ".....wWWWWWWWWWW...",
        "......wwWWWWWww....",
        ".......wWWwwww.....",
        "...................",
    ],
    &[
        "...................",
        "...................",
        "......WWWWW.....WW.",
        "...WWWWWwwWWWWWWWkW",
        "WWWWWWWWWwwWWWWWWOO",
        "WWW...wwWWWWWWWWW..",
        ".....wWWWWWWWwww...",
        "......wwWWwww......",
        "...................",
        "...................",
    ],
    &[
        "...................",
        "...................",
        ".............WWW...",
        "......WWWW.WWWWWkW.",
        "WW...WWWWWWWWWWWWOO",
        "WWWWWWWWwwWWWWWWW..",
        "..WWWWWWWwWWWWww...",
        "...WWwwwWWWWww.....",
        "....WWWwwwww.......",
        ".....WWww..........",
    ],
];
const DOVE_W: i32 = 19;

/// The log of the failure: when each line shows up (the first two are
/// frightening, the rest give the joke away) and what it says.
const LOG: [(f32, &str); 5] = [
    (1.4, "¡ALERTA! Intrusión detectada en el sistema"),
    (1.9, "Error 0x1C0DEA28: borrando todos los archivos"),
    (3.3, "Causa probable: exceso de buñuelos"),
    (4.4, "Culpable: virus SANTOS-INOCENTES v28.12"),
    (5.6, "Respaldo de ponche: no se encontró"),
];
/// The truth, in one line or two, and the greeting under it.
const TRUTH: &str = "¡INOCENTE PALOMITA, QUE TE DEJASTE ENGAÑAR!";
const GREETING: &str = "¡Feliz día de los Santos Inocentes!";
/// The truth in two lines, for the big letters.
const BIG: [&str; 2] = ["¡INOCENTE PALOMITA,", "QUE TE DEJASTE ENGAÑAR!"];
/// The big letters' font, 3 by 5 pixels: each glyph's five rows, three bits
/// each.
const FONT: &[(char, [u8; 5])] = &[
    ('A', [0b010, 0b101, 0b111, 0b101, 0b101]),
    ('C', [0b011, 0b100, 0b100, 0b100, 0b011]),
    ('D', [0b110, 0b101, 0b101, 0b101, 0b110]),
    ('E', [0b111, 0b100, 0b110, 0b100, 0b111]),
    ('G', [0b011, 0b100, 0b101, 0b101, 0b011]),
    ('I', [0b111, 0b010, 0b010, 0b010, 0b111]),
    ('J', [0b001, 0b001, 0b001, 0b101, 0b010]),
    ('L', [0b100, 0b100, 0b100, 0b100, 0b111]),
    ('M', [0b101, 0b111, 0b111, 0b101, 0b101]),
    ('N', [0b111, 0b101, 0b101, 0b101, 0b101]),
    ('Ñ', [0b111, 0b101, 0b101, 0b101, 0b101]),
    ('O', [0b010, 0b101, 0b101, 0b101, 0b010]),
    ('P', [0b110, 0b101, 0b110, 0b100, 0b100]),
    ('Q', [0b010, 0b101, 0b101, 0b110, 0b011]),
    ('R', [0b110, 0b101, 0b110, 0b101, 0b101]),
    ('S', [0b011, 0b100, 0b010, 0b001, 0b110]),
    ('T', [0b111, 0b010, 0b010, 0b010, 0b010]),
    ('U', [0b101, 0b101, 0b101, 0b101, 0b111]),
    ('!', [0b010, 0b010, 0b010, 0b000, 0b010]),
    ('¡', [0b010, 0b000, 0b010, 0b010, 0b010]),
    (',', [0b000, 0b000, 0b000, 0b010, 0b100]),
];

/// The moments, in seconds: the glitches start, the alarm opens (and the bar
/// and countdown start), the countdown reaches zero and the screen flashes,
/// the banner starts to leave, and it is all over.
const ALARM: f32 = 1.4;
const COUNT_FROM: f32 = 2.4;
const REVEAL: f32 = 8.6;
const FLASH: f32 = 0.12;
const LEAVE: f32 = 15.6;
const END: f32 = 18.4;

/// Confetti in the air at most, and how many fall per second.
const CONFETTI: usize = 150;
const RAIN: f32 = 55.0;
/// The papel de china colors of the banner and the confetti.
const PAPER: [u8; 6] = *b"PZQOML";

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Inocente {
        t: 0.0,
        w: w as i32,
        h: h as i32,
        left: rng.bool(),
        seed: rng.u32(..),
        bits: [Bit::default(); CONFETTI],
        next: 0,
        rain_in: 0.0,
        rng: fastrand::Rng::with_seed(rng.u64(..)),
    })
}

#[derive(Clone, Copy, Default)]
struct Bit {
    /// Position in pixels, speed down, and sway.
    x: f32,
    y: f32,
    fall: f32,
    sway: f32,
    /// Its color, in the palette's letters; 0 when unused.
    color: u8,
}

struct Inocente {
    t: f32,
    w: i32,
    /// Screen height in cells.
    h: i32,
    /// The dove comes in from the left?
    left: bool,
    seed: u32,
    bits: [Bit; CONFETTI],
    next: usize,
    rain_in: f32,
    rng: fastrand::Rng,
}

/// Smooth from 0 to 1.
fn smooth(k: f32) -> f32 {
    let k = k.clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

impl Inocente {
    /// A number in 0..1 for (`a`, `b`, `c`), the same every time.
    fn hash(&self, a: u32, b: u32, c: u32) -> f32 {
        let mut v = a
            .wrapping_mul(0x9e37_79b9)
            .wrapping_add(b.wrapping_mul(0x85eb_ca6b))
            .wrapping_add(c.wrapping_mul(0xc2b2_ae35))
            ^ self.seed;
        v ^= v >> 15;
        v = v.wrapping_mul(0x2c1b_3c6d);
        v ^= v >> 12;
        (v & 0xffff) as f32 / 65536.0
    }

    /// How hard the screen glitches: growing, at its worst as the alarm
    /// opens, then fading to the odd twitch.
    fn glitch(&self) -> f32 {
        let t = self.t;
        if t < ALARM {
            t / ALARM
        } else if t < ALARM + 1.9 {
            1.0 - 0.75 * (t - ALARM) / 1.9
        } else if t < REVEAL {
            0.2 * (1.0 - smooth((t - 3.3) / 3.0))
        } else {
            0.0
        }
    }

    /// How red the screen is, from the alarm to the flash.
    fn red(&self) -> f32 {
        if (ALARM..REVEAL).contains(&self.t) {
            0.5 * smooth((self.t - ALARM) / 0.6)
        } else {
            0.0
        }
    }

    /// The banner's lines for a screen this wide, and its inner width.
    fn truth(&self, wide: i32) -> ([&'static str; 2], usize) {
        if wide >= TRUTH.chars().count() as i32 + 10 {
            ([TRUTH, ""], 1)
        } else {
            (split_in_two(TRUTH), 2)
        }
    }

    /// Shifts the row `y` sideways by `dx`, smearing the edge.
    fn tear(c: &mut Canvas, y: i32, dx: i32) {
        let w = c.w() as i32;
        if dx > 0 {
            for x in (dx..w).rev() {
                if let Some(&cell) = c.get(x - dx, y) {
                    c.set_cell(x, y, cell);
                }
            }
        } else {
            for x in 0..(w + dx).max(0) {
                if let Some(&cell) = c.get(x - dx, y) {
                    c.set_cell(x, y, cell);
                }
            }
        }
    }

    /// Glitches the screen: rows torn sideways, blocks of noise.
    fn glitch_screen(&self, c: &mut Canvas, pal: &Palette) {
        let level = self.glitch();
        if level <= 0.0 {
            return;
        }
        let (w, h) = (c.w() as i32, c.h() as i32);
        // It changes 14 times a second.
        let q = (self.t * 14.0) as u32;
        for y in 0..h {
            if self.hash(q, y as u32, 1) < level * 0.28 {
                let dx = ((self.hash(q, y as u32, 2) - 0.5) * 2.0 * (4.0 + 0.2 * w as f32 * level))
                    as i32;
                Self::tear(c, y, dx);
            }
        }
        let noise = (level * (w * h) as f32 * 0.03) as u32;
        for n in 0..noise {
            let x = (self.hash(q, n, 3) * w as f32) as i32;
            let y = (self.hash(q, n, 4) * h as f32) as i32;
            let ch = ['░', '▒', '▓', '█', '╳', '#'][(self.hash(q, n, 5) * 6.0) as usize % 6];
            let ink = (*b"RrQWR")[(self.hash(q, n, 6) * 5.0) as usize % 5];
            if let Some(cell) = c.get_mut(x, y) {
                cell.ch = ch;
                cell.fg = pal.of(ink);
            }
        }
    }

    /// Washes the screen red, dimming it, while the alarm lasts.
    fn redden(&self, c: &mut Canvas, theme: &Theme, pal: &Palette) {
        let k = self.red();
        if k <= 0.0 {
            return;
        }
        let red = pal.of(b'R');
        let (w, h) = (c.w() as i32, c.h() as i32);
        for y in 0..h {
            for x in 0..w {
                if let Some(cell) = c.get_mut(x, y) {
                    cell.fg = fade(theme, cell.fg, 0.35 * k).lerp(red, 0.4 * k);
                    if matches!(cell.bg, Color::Rgb(..)) {
                        cell.bg = cell.bg.lerp(red.scale(0.4), 0.3 * k);
                    }
                }
            }
        }
    }

    /// Draws a box on its own background, its border colored by `border`.
    fn draw_box(
        c: &mut Canvas,
        (x0, y0, bw, bh): (i32, i32, i32, i32),
        border: impl Fn(i32) -> Color,
        double: bool,
    ) {
        let bg = c.bg();
        let (x1, y1) = (x0 + bw - 1, y0 + bh - 1);
        let ch = if double {
            ['╔', '╗', '╚', '╝', '═', '║']
        } else {
            ['╭', '╮', '╰', '╯', '─', '│']
        };
        let put = |c: &mut Canvas, x: i32, y: i32, chr: char, fg: Color| {
            c.set_cell(
                x,
                y,
                Cell {
                    ch: chr,
                    fg,
                    bg,
                    attr: BOLD,
                },
            );
        };
        for y in y0..=y1 {
            for x in x0..=x1 {
                let i = (x - x0) + (y - y0) * 3;
                let (top, bottom, left, right) = (y == y0, y == y1, x == x0, x == x1);
                let chr = match (top, bottom, left, right) {
                    (true, _, true, _) => ch[0],
                    (true, _, _, true) => ch[1],
                    (_, true, true, _) => ch[2],
                    (_, true, _, true) => ch[3],
                    (true, ..) | (_, true, ..) => ch[4],
                    (_, _, true, _) | (_, _, _, true) => ch[5],
                    _ => ' ',
                };
                put(c, x, y, chr, border(i));
            }
        }
    }

    /// Writes `s` at (`x`, `y`) on the screen's own background.
    fn write(c: &mut Canvas, x: i32, y: i32, s: &str, fg: Color, attr: u8) {
        let bg = c.bg();
        for (i, chr) in s.chars().enumerate() {
            c.set_cell(
                x + i as i32,
                y,
                Cell {
                    ch: chr,
                    fg,
                    bg,
                    attr,
                },
            );
        }
    }

    /// The frightening box: the log so far, a bar running down and the
    /// countdown to the restart.
    fn draw_alarm(&self, c: &mut Canvas, pal: &Palette) {
        let (w, h) = (c.w() as i32, c.h() as i32);
        let bw = (LOG[1].1.chars().count() as i32 + 6).min(w - 2);
        let bh = 5 + LOG.len() as i32 + 1;
        // It trembles while it is still frightening.
        let shake = if self.t < 3.3 {
            (self.hash((self.t * 20.0) as u32, 0, 9) * 3.0) as i32 - 1
        } else {
            0
        };
        let (x0, y0) = ((w - bw) / 2 + shake, ((h - bh) / 2).max(0));
        let blink = (self.t * 3.0) as i32 % 2 == 0;
        let (red, hot) = (pal.of(b'R'), pal.of(b'r'));
        Self::draw_box(c, (x0, y0, bw, bh), |_| if blink { hot } else { red }, true);
        let title = " ▲ FALLA CRÍTICA DEL SISTEMA ▲ ";
        let tx = x0 + (bw - title.chars().count() as i32) / 2;
        Self::write(
            c,
            tx,
            y0,
            title,
            if blink { pal.of(b'W') } else { hot },
            BOLD,
        );
        for (i, (at, text)) in LOG.iter().enumerate() {
            if *at > self.t {
                continue;
            }
            // The first two lines are red, the rest give it away in yellow.
            let ink = if i < 2 { hot } else { pal.of(b'Y') };
            Self::write(c, x0 + 3, y0 + 2 + i as i32, text, ink, 0);
        }
        if self.t >= COUNT_FROM {
            // The bar runs from 0 to 100 as the countdown does.
            let k = ((self.t - COUNT_FROM) / (REVEAL - COUNT_FROM)).clamp(0.0, 1.0);
            let room = (bw - 6 - 18).max(4);
            let full = (room as f32 * k) as i32;
            let row = y0 + 2 + LOG.len() as i32;
            let ink = pal.of(b'W');
            Self::write(c, x0 + 3, row, "Borrando [", ink, 0);
            let bar = x0 + 13;
            for i in 0..room {
                let chr = if i < full { "█" } else { "░" };
                Self::write(c, bar + i, row, chr, ink, 0);
            }
            Self::write(c, bar + room, row, "]", ink, 0);
            // The percentage, in three places, and the seconds left.
            let percent = (k * 100.0) as i32;
            for (place, div) in [100, 10, 1].into_iter().enumerate() {
                let digit = (percent / div % 10) as u8;
                let shown = div == 1 || percent >= div;
                let chr = if shown { (b'0' + digit) as char } else { ' ' };
                let at = bar + room + 2 + place as i32;
                c.set_cell(
                    at,
                    row,
                    Cell {
                        ch: chr,
                        fg: ink,
                        bg: c.bg(),
                        attr: 0,
                    },
                );
            }
            Self::write(c, bar + room + 5, row, "%", ink, 0);
            let left = (REVEAL - self.t).ceil().clamp(0.0, 9.0) as u8;
            let ink = if blink { hot } else { ink };
            Self::write(c, x0 + 3, row + 1, "Reinicio forzado en  s", ink, BOLD);
            let digit = Cell {
                ch: (b'0' + left) as char,
                fg: ink,
                bg: c.bg(),
                attr: BOLD,
            };
            c.set_cell(x0 + 3 + 20, row + 1, digit);
        }
    }

    /// Draws `text` in the big pixel font from the pixel (`x`, `y`), each
    /// word in its own color.
    fn big(c: &mut Canvas, (x, y): (i32, i32), text: &str, color: impl Fn(usize) -> Color) {
        let mut word = 0;
        for (i, ch) in text.chars().enumerate() {
            if ch == ' ' {
                word += 1;
                continue;
            }
            let Some((_, rows)) = FONT.iter().find(|(g, _)| *g == ch) else {
                continue;
            };
            let gx = x + i as i32 * 4;
            for (gy, bits) in rows.iter().enumerate() {
                for bit in 0..3 {
                    if bits >> (2 - bit) & 1 == 1 {
                        pixel(c, gx + bit, y + gy as i32, color(word));
                    }
                }
            }
            // The tilde of the Ñ.
            if ch == 'Ñ' {
                pixel(c, gx + 1, y - 1, color(word));
            }
        }
    }

    /// The truth: a banner of papel de china colors, in big letters when the
    /// screen has room for them, else in plain text.
    fn draw_banner(&self, c: &mut Canvas, theme: &Theme, pal: &Palette) {
        let (w, h) = (c.w() as i32, c.h() as i32);
        let big = w >= BIG[1].chars().count() as i32 * 4 + 8 && h >= 14;
        let (lines, n) = if big { (BIG, 2) } else { self.truth(w) };
        let letter = if big { 4 } else { 1 };
        let widest = lines[..n]
            .iter()
            .map(|l| l.chars().count() as i32 * letter)
            .max()
            .unwrap_or(0);
        let greet = w >= GREETING.chars().count() as i32 + 6;
        let bw = (widest.max(if greet {
            GREETING.chars().count() as i32
        } else {
            0
        }) + 6)
            .min(w);
        // Big letters are five pixels tall and two apart.
        let text_rows = if big { 7 } else { n as i32 };
        let bh = text_rows + 4 + i32::from(greet);
        // It pops in with a little bounce and goes away upward.
        let pop = smooth((self.t - REVEAL) / 0.35);
        let away = smooth((self.t - LEAVE) / 1.2);
        let (x0, y0) = ((w - bw) / 2, ((h - bh) / 2) as f32);
        let y0 = (y0 - away * (h as f32)).round() as i32;
        if pop <= 0.0 || y0 + bh < 0 {
            return;
        }
        let spin = (self.t * 9.0) as i32;
        let colors = PAPER.map(|l| pal.of(l));
        Self::draw_box(
            c,
            (x0, y0, bw, bh),
            |i| colors[((i + spin) as usize) % colors.len()],
            true,
        );
        let tint = |word: usize| colors[(word + spin as usize / 4) % colors.len()];
        for (i, line) in lines[..n].iter().enumerate() {
            let len = line.chars().count() as i32 * letter;
            let x = x0 + (bw - len) / 2;
            if big {
                // Each line is five pixels tall; the Ñ's tilde goes in the
                // gap above.
                let y = (y0 + 2) * 2 + i as i32 * 7 + 1;
                Self::big(c, (x, y), line, tint);
                continue;
            }
            // Each word in its own color.
            let mut cx = x;
            for (k, word) in line.split(' ').enumerate() {
                Self::write(c, cx, y0 + 2 + i as i32, word, tint(k), BOLD);
                cx += word.chars().count() as i32 + 1;
            }
        }
        if greet {
            let x = x0 + (bw - GREETING.chars().count() as i32) / 2;
            let ink = fade(theme, pal.of(b'w'), 0.1);
            Self::write(c, x, y0 + 2 + text_rows, GREETING, ink, 0);
        }
    }

    /// Where the dove is: in from one side, hovering over the banner, then
    /// away the other way.
    fn dove(&self, w: i32, h: i32) -> (i32, i32) {
        let (wf, pause) = (w as f32, self.t - REVEAL);
        let rest = (
            wf / 2.0 - DOVE_W as f32 / 2.0,
            (h as f32 * 2.0) * 0.5 - 28.0 - (self.t * 3.0).sin() * 2.0,
        );
        let (x, y) = if self.t < LEAVE {
            let k = smooth(pause / 2.2);
            let from = -(DOVE_W as f32) - 4.0;
            (
                from + (rest.0 - from) * k + (1.0 - k) * 0.0 + (self.t * 1.7).sin() * 3.0 * k,
                rest.1 - (1.0 - k) * 14.0,
            )
        } else {
            let k = smooth((self.t - LEAVE) / 2.6);
            let to = wf + 6.0;
            (rest.0 + (to - rest.0) * k, rest.1 - 12.0 * k)
        };
        (
            if self.left { x } else { wf - x - DOVE_W as f32 }.round() as i32,
            y.round() as i32,
        )
    }
}

impl Egg for Inocente {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        for b in self.bits.iter_mut().filter(|b| b.color != 0) {
            b.y += b.fall * dt;
            b.x += (self.t * 3.0 + b.sway).sin() * 6.0 * dt;
            if b.y > self.h as f32 * 2.0 {
                b.color = 0;
            }
        }
        if (REVEAL..LEAVE).contains(&self.t) {
            self.rain_in -= dt;
            while self.rain_in <= 0.0 {
                self.rain_in += 1.0 / RAIN;
                self.bits[self.next] = Bit {
                    x: self.rng.f32() * self.w as f32,
                    y: -1.0,
                    fall: 12.0 + self.rng.f32() * 16.0,
                    sway: self.rng.f32() * TAU,
                    color: PAPER[self.rng.usize(..PAPER.len())],
                };
                self.next = (self.next + 1) % CONFETTI;
            }
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::navidad(theme);
        let t = self.t;
        if t < REVEAL {
            self.redden(c, theme, &pal);
            self.glitch_screen(c, &pal);
            if t >= ALARM {
                self.draw_alarm(c, &pal);
            }
        } else if t < REVEAL + FLASH {
            // The flash: the whole screen white for an instant.
            let col = pal.of(b'W');
            let (w, h) = (c.w() as i32, c.h() as i32);
            for y in 0..h {
                for x in 0..w {
                    c.set_cell(
                        x,
                        y,
                        Cell {
                            ch: '█',
                            fg: col,
                            bg: col,
                            attr: 0,
                        },
                    );
                }
            }
            return;
        }
        if t < REVEAL {
            return;
        }
        for b in self.bits.iter().filter(|b| b.color != 0) {
            pixel(c, b.x.round() as i32, b.y.round() as i32, pal.of(b.color));
        }
        self.draw_banner(c, theme, &pal);
        // The dove, flapping.
        let (dx, dy) = self.dove(c.w() as i32, c.h() as i32);
        let beat = [0, 1, 2, 1][(self.t * 9.0) as usize % 4];
        let flip = !self.left;
        sprite(c, (dx, dy), DOVE[beat], flip, |_| 0, |ch| pal.get(ch));
    }

    fn done(&self) -> bool {
        self.t > END
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_art_is_whole() {
        assert!(
            DOVE.iter().all(|f| {
                f.len() == DOVE[0].len() && f.iter().all(|r| r.len() as i32 == DOVE_W)
            })
        );
        // The frightening part is short, and the joke is out by the middle.
        assert!(LOG[2].0 < 4.0 && LOG[1].0 < LOG[2].0 && LOG[LOG.len() - 1].0 < COUNT_FROM + 4.0);
        const { assert!(REVEAL < LEAVE && LEAVE < END && END < 20.0) };
    }
}
