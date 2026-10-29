//! Día de los Santos Inocentes (December 28): now and then the whole hacker
//! screen goes wrong. Every pane flashes red, a box opens over the panes that
//! shouts SISTEMA COMPROMETIDO, a red bar eats /dev/sda1 and a countdown
//! runs, until the bar sticks at 99%, the red lifts and the box turns gold:
//! "¡INOCENTE PALOMITA, QUE TE DEJASTE ENGAÑAR!", with confetti falling over
//! the panes. Then it all goes back to normal until the next time.
//!
//! The act is timed in real seconds and has its own random numbers, so the
//! rest of the screen does not notice it.

use super::widgets::glyph;
use super::{Pal, Rect, put};
use crate::color::Color;
use crate::render::{BOLD, Canvas};
use fastrand::Rng;

/// Seconds of each part of an act: the first alarm, the system being
/// compromised, the bar stuck at 99%, the reveal.
const ALARM: f32 = 1.2;
const COMPROMISED: f32 = 7.0;
const STUCK: f32 = 1.0;
const REVEAL: f32 = 5.0;
const ACT: f32 = ALARM + COMPROMISED + STUCK + REVEAL;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Part {
    Waiting,
    Alarm,
    Compromised,
    Stuck,
    Reveal,
}

pub struct Fool {
    rng: Rng,
    /// Seconds into the act; negative while waiting for the next one.
    t: f32,
    /// Where the "attacker" comes from, fixed for each act.
    ip: [u8; 4],
    /// Where the lines of text are written, reused every frame.
    buf: String,
}

#[inline]
fn hash(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9e37_79b1) ^ b.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^ (h >> 12)
}

impl Fool {
    pub fn new(seed: u64) -> Fool {
        let mut rng = Rng::with_seed(seed ^ 0x1a0c_e277);
        let ip = [
            rng.u8(11..224),
            rng.u8(..=255),
            rng.u8(..=255),
            rng.u8(1..=254),
        ];
        // The first act comes soon, so nobody has to wait for the prank.
        let t = -(5.0 + rng.f32() * 4.0);
        Fool {
            rng,
            t,
            ip,
            buf: String::new(),
        }
    }

    /// Advances by `real` seconds of wall-clock time.
    pub fn tick(&mut self, real: f32) {
        self.t += real.clamp(0.0, 30.0);
        if self.t >= ACT {
            // And again in a while.
            self.t = -(20.0 + self.rng.f32() * 30.0);
            self.ip = [
                self.rng.u8(11..224),
                self.rng.u8(..=255),
                self.rng.u8(..=255),
                self.rng.u8(1..=254),
            ];
        }
    }

    pub fn part(&self) -> Part {
        let t = self.t;
        if t < 0.0 {
            Part::Waiting
        } else if t < ALARM {
            Part::Alarm
        } else if t < ALARM + COMPROMISED {
            Part::Compromised
        } else if t < ALARM + COMPROMISED + STUCK {
            Part::Stuck
        } else {
            Part::Reveal
        }
    }

    /// Starts the act now (for the tests).
    #[cfg(test)]
    pub fn start(&mut self) {
        self.t = 0.0;
    }

    #[cfg(test)]
    pub fn seek(&mut self, t: f32) {
        self.t = t;
    }

    /// The color the panes' borders take while the act is on: flashing red
    /// through the alarm, gold when the joke is owned up to.
    pub fn edge(&self, pal: &Pal) -> Option<Color> {
        match self.part() {
            Part::Waiting => None,
            Part::Alarm | Part::Compromised | Part::Stuck => {
                Some(if ((self.t * 6.0) as u32).is_multiple_of(2) {
                    pal.red
                } else {
                    pal.fade(pal.red, 0.4)
                })
            }
            Part::Reveal => Some(pal.oro),
        }
    }

    /// The bar's fill, 0 to 1: it creeps up through the compromise and
    /// sticks at 99%.
    fn fill(&self) -> f32 {
        match self.part() {
            Part::Waiting | Part::Alarm => 0.0,
            Part::Compromised => {
                let k = (self.t - ALARM) / COMPROMISED;
                // Slow, then fast, like a real wipe.
                0.99 * (k * k * 0.6 + k * 0.4)
            }
            _ => 0.99,
        }
    }

    /// Draws the act over whatever is on the canvas.
    pub fn draw(&mut self, c: &mut Canvas, pal: &Pal) {
        let part = self.part();
        if part == Part::Waiting {
            return;
        }
        let (w, h) = (c.w() as i32, c.h() as i32);
        // The wash: everything on screen turns red, flickering.
        let tick = (self.t * 12.0) as u32;
        let wash = match part {
            Part::Alarm => {
                if tick.is_multiple_of(2) {
                    0.8
                } else {
                    0.35
                }
            }
            Part::Compromised => 0.5 + 0.2 * ((self.t * 7.0).sin() * 0.5 + 0.5),
            Part::Stuck => {
                if tick.is_multiple_of(3) {
                    0.85
                } else {
                    0.4
                }
            }
            _ => 0.0,
        };
        if wash > 0.0 {
            for y in 0..h {
                for x in 0..w {
                    if let Some(cell) = c.get_mut(x, y)
                        && cell.ch != ' '
                    {
                        cell.fg = cell.fg.lerp(pal.red, wash);
                    }
                }
            }
        }
        if part == Part::Alarm {
            let flash = tick.is_multiple_of(2);
            for x in 0..w {
                c.set_attr(
                    x,
                    0,
                    if flash { '\u{2593}' } else { '\u{2591}' },
                    pal.red,
                    BOLD,
                );
            }
            let msg = " \u{26a0} \u{00a1}\u{00a1}ALERTA!! \u{26a0} ";
            let n = msg.chars().count() as i32;
            if w > n + 2 {
                put(
                    c,
                    Rect { x: 0, y: 0, w, h },
                    (w - n) / 2,
                    0,
                    msg,
                    pal.bright,
                    BOLD,
                );
            }
            return;
        }
        if part == Part::Reveal {
            self.confetti(c, pal);
        }
        self.banner(c, pal, part);
    }

    /// The box over the panes.
    fn banner(&mut self, c: &mut Canvas, pal: &Pal, part: Part) {
        let (w, h) = (c.w() as i32, c.h() as i32);
        let reveal = part == Part::Reveal;
        let col = if reveal { pal.oro } else { pal.red };
        let (l1, l2, up, down) = if reveal {
            (
                "INOCENTE",
                "PALOMITA!",
                " SANTOS INOCENTES ",
                " \u{00a1}INOCENTE PALOMITA, QUE TE DEJASTE ENGA\u{00d1}AR! ",
            )
        } else {
            (
                "SISTEMA",
                "COMPROMETIDO",
                " SISTEMA COMPROMETIDO ",
                " ACCESO NO AUTORIZADO ",
            )
        };
        let sx = if w >= 2 * l2.len() as i32 * 4 + 6 {
            2
        } else {
            1
        };
        let big = h >= 22 && w >= l2.len() as i32 * 4 + 6;
        let bw = if big {
            (l2.len() as i32 * 4 * sx + 6).min(w)
        } else {
            (down.chars().count() as i32 + 4).max(24).min(w)
        };
        let lines = if big { 5 + 1 + 5 + 4 } else { 5 };
        let bh = (lines + 2).min(h);
        if bw < 12 || bh < 3 {
            return;
        }
        let (bx, by) = ((w - bw) / 2, ((h - bh) / 2).max(0));
        let full = Rect { x: 0, y: 0, w, h };
        for y in by..by + bh {
            for x in bx..bx + bw {
                c.set(x, y, ' ', col);
                c.set_bg(x, y, pal.ink);
            }
        }
        for x in bx..bx + bw {
            c.set_attr(x, by, '\u{2550}', col, BOLD);
            c.set_attr(x, by + bh - 1, '\u{2550}', col, BOLD);
        }
        for y in by..by + bh {
            c.set_attr(bx, y, '\u{2551}', col, BOLD);
            c.set_attr(bx + bw - 1, y, '\u{2551}', col, BOLD);
        }
        c.set_attr(bx, by, '\u{2554}', col, BOLD);
        c.set_attr(bx + bw - 1, by, '\u{2557}', col, BOLD);
        c.set_attr(bx, by + bh - 1, '\u{255a}', col, BOLD);
        c.set_attr(bx + bw - 1, by + bh - 1, '\u{255d}', col, BOLD);
        // Captions on the frame, as the access banners have them.
        for (y, t) in [(by, up), (by + bh - 1, down)] {
            let n = t.chars().count() as i32;
            if n + 2 <= bw {
                put(c, full, bx + (bw - n) / 2, y, t, pal.bright, BOLD);
            }
        }
        let blink = reveal || (self.t * 3.0) as i32 % 2 == 0;
        let mut y = by + 1;
        if big {
            if blink {
                word(c, bx, y, bw, l1, sx, col);
                word(c, bx, y + 6, bw, l2, sx, col);
            }
            y += 12;
        } else {
            let n = (l1.len() + 1 + l2.len()) as i32;
            if blink {
                let x = put(c, full, bx + (bw - n) / 2, y + 1, l1, col, BOLD);
                put(c, full, x + 1, y + 1, l2, col, BOLD);
            }
            y += 3;
        }
        // Under the words: what the "attack" is doing, or the joke owned up to.
        if reveal {
            if big && y < by + bh - 1 {
                let t = "que te dejaste enga\u{00f1}ar \u{2014} \u{00a1}feliz d\u{00ed}a de los Santos Inocentes!";
                put_centered(c, full, bx, bw, y, t, pal.bright);
            }
            return;
        }
        if big && y + 2 < by + bh {
            // The bar, and what it is erasing.
            let fill = self.fill();
            let mut t = std::mem::take(&mut self.buf);
            t.clear();
            use std::fmt::Write;
            let _ = write!(
                t,
                "{}.{}.{}.{} \u{2192} /dev/sda1: borrando {:>2.0}%",
                self.ip[0],
                self.ip[1],
                self.ip[2],
                self.ip[3],
                fill * 100.0
            );
            put_centered(c, full, bx, bw, y, &t, pal.red);
            self.buf = t;
            let barw = (bw - 8).max(4);
            let filled = (fill * barw as f32) as i32;
            for i in 0..barw {
                let (ch, k) = if i < filled {
                    ('\u{2588}', pal.red)
                } else {
                    ('\u{2591}', pal.dim)
                };
                c.set(bx + 4 + i, y + 1, ch, k);
            }
        }
    }

    /// Confetti and serpentinas over everything: a fixed number of bits, each
    /// falling at its own speed.
    fn confetti(&self, c: &mut Canvas, pal: &Pal) {
        let (w, h) = (c.w() as i32, c.h() as i32);
        if w < 4 || h < 4 {
            return;
        }
        let t = self.t - (ALARM + COMPROMISED + STUCK);
        let n = ((w * h) / 45).clamp(8, 160) as u32;
        for i in 0..n {
            let hx = hash(i, 17);
            let x = (hx % w as u32) as i32;
            let speed = 3.0 + (hash(i, 5) % 100) as f32 / 12.0;
            let y = ((hash(i, 9) % h as u32) as f32 + t * speed).rem_euclid(h as f32) as i32;
            let ch = [
                '\u{2022}', '\u{25aa}', '*', '\u{2726}', '\u{2736}', '\u{00b7}',
            ][(hx / 7 % 6) as usize];
            c.set_attr(x, y, ch, pal.picado[(i % 6) as usize], BOLD);
        }
    }
}

fn put_centered(c: &mut Canvas, full: Rect, bx: i32, bw: i32, y: i32, t: &str, col: Color) {
    let n = t.chars().count() as i32;
    if n <= bw {
        put(c, full, bx + (bw - n) / 2, y, t, col, 0);
    }
}

/// A word in the 3x5 pixel font, centered in the box at `bx` of width `bw`.
fn word(c: &mut Canvas, bx: i32, y: i32, bw: i32, text: &str, sx: i32, col: Color) {
    let gw = 4 * sx;
    let total = text.chars().count() as i32 * gw - sx;
    let mut x = bx + (bw - total) / 2;
    for ch in text.chars() {
        let g = glyph(ch);
        for (row, bits) in g.iter().enumerate() {
            for px in 0..3 {
                if bits >> (2 - px) & 1 == 1 {
                    for k in 0..sx {
                        c.set(x + px * sx + k, y + row as i32, '\u{2588}', col);
                    }
                }
            }
        }
        x += gw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::scene::Ctx;
    use crate::theme::Theme;

    fn pal() -> Pal {
        let theme = Theme::classic();
        let o = crate::scene::Opts::new(&[], toml::Table::new());
        Pal::new(&Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 1,
            fiesta: Fiesta::of_day(12, 28),
        })
    }

    #[test]
    fn the_act_runs_through_its_parts_and_comes_back() {
        let mut f = Fool::new(3);
        assert_eq!(f.part(), Part::Waiting);
        let mut seen = Vec::new();
        for _ in 0..4000 {
            f.tick(0.05);
            let p = f.part();
            if seen.last() != Some(&p) {
                seen.push(p);
            }
        }
        // Waiting, then each part in order, then waiting again, again and again.
        assert!(seen.len() >= 11, "{seen:?}");
        assert_eq!(
            &seen[..6],
            &[
                Part::Waiting,
                Part::Alarm,
                Part::Compromised,
                Part::Stuck,
                Part::Reveal,
                Part::Waiting
            ]
        );
        // The bar creeps up and sticks at 99%.
        f.seek(ALARM + 0.1);
        let early = f.fill();
        f.seek(ALARM + COMPROMISED - 0.1);
        let late = f.fill();
        f.seek(ALARM + COMPROMISED + 0.3);
        assert!(early < 0.05 && late > 0.8 && f.fill() == 0.99);
    }

    #[test]
    fn it_ends_in_inocente_palomita() {
        let pal = pal();
        let mut f = Fool::new(3);
        for &(w, h) in &[(140u16, 40u16), (80, 24), (40, 12), (13, 5), (3, 3), (1, 1)] {
            f.start();
            let mut texts = Vec::new();
            for step in 0..300 {
                let mut c = Canvas::new(w, h, Color::Reset);
                // Some text underneath to turn red.
                c.text(0, 0, "hello hacker", pal.main);
                f.seek(step as f32 * ACT / 300.0);
                f.draw(&mut c, &pal);
                texts.push(c.to_text());
            }
            if w >= 80 {
                assert!(texts.iter().any(|t| t.contains("ALERTA")), "{w}x{h}");
                assert!(
                    texts.iter().any(|t| t.contains("SISTEMA COMPROMETIDO")),
                    "{w}x{h}"
                );
                assert!(texts.iter().any(|t| t.contains("borrando")), "{w}x{h}");
                assert!(
                    texts
                        .iter()
                        .any(|t| t.contains("INOCENTE PALOMITA, QUE TE DEJASTE")),
                    "{w}x{h}"
                );
            }
        }
        // Waiting draws nothing.
        let mut c = Canvas::new(80, 24, Color::Reset);
        let mut f = Fool::new(3);
        f.draw(&mut c, &pal);
        assert!(c.cells().iter().all(|x| x.ch == ' '));
    }

    #[test]
    fn the_wash_turns_the_screen_red_and_the_reveal_lifts_it() {
        let pal = pal();
        let mut f = Fool::new(4);
        let text_color = |f: &mut Fool| {
            let mut c = Canvas::new(100, 30, Color::Reset);
            c.text(2, 10, "plain green text", pal.main);
            f.draw(&mut c, &pal);
            c.get(2, 10).unwrap().fg
        };
        f.seek(ALARM + 2.0);
        assert_ne!(text_color(&mut f), pal.main);
        f.seek(ALARM + COMPROMISED + STUCK + 2.0);
        assert_eq!(text_color(&mut f), pal.main);
    }
}
