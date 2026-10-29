//! A piñata of siete picos is broken at a posada, during the Navidad.
//!
//! The piñata is drawn from papel de china, not from a picture, so it can
//! turn and swing: a round body with seven cones, each a different color
//! (rosa mexicano, amarillo, naranja, turquesa, morado, verde limón and
//! rojo), banded like crinkled paper and fringed at the tips, hanging from a
//! rope that someone out of sight pulls up and down. A blindfolded kid
//! walks in, is spun around until dizzy, and swings a striped stick at it,
//! "¡Dale, dale, dale, no pierdas el tino!" typing itself out above them:
//! the first swings whistle under the piñata as it is yanked out of reach,
//! then the stick lands, the piñata jolts and spins and drops a few
//! sweets, and the last blow bursts it. Colación, cacahuates, tejocotes,
//! mandarinas and cañas rain down, bounce and settle on the ground, and the
//! papel de china flutters after them while the kid jumps for joy. The kid
//! comes from either side; all of it is over within 30 s.

use super::navidad::{Palette, fade, type_out};
use super::{Egg, Info, pixel, sprite};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "pinata",
    start,
    fiesta: Kind::Navidad,
    when: super::any_day,
};

type Frames = [&'static [&'static str]; 2];

/// The kid, facing right, blindfolded in red (the colors of 'x' shirt, 'z'
/// its shade and 'j' hair are the kid's own); the arms and the stick are
/// drawn from the shoulder.
const KID: &[&str] = &[
    "...jjjjjj...",
    "..jjjjjjjj..",
    "..jjjjjjjjj.",
    ".RRRRRRRRRRR",
    "RRRRRRRRRRRR",
    "..jeeeeeeeE.",
    "..jeeeeeeee.",
    "...eeekkeee.",
    "....eeeeee..",
    "....xxxxxx..",
    "...xxxxxxxx.",
    "..xxxxxxxxxx",
    "..xxzxxxxxxx",
    "..xxzxxxxxxx",
    "..xxxxxxxxxx",
    "..xxxxxxxxxx",
    "..xxxxxxxxxx",
    "..xxxxxxxxxx",
    "..nnnnnnnnnn",
];
/// The kid's legs: standing and mid-step.
const KID_LEGS: Frames = [
    &[
        "...nn..nn...",
        "...nn..nn...",
        "...nn..nn...",
        "..bbb..bbb..",
    ],
    &[
        "....nnnn....",
        "...nn..nn...",
        "...nn..nn...",
        "..bbb..bbb..",
    ],
];
const KID_W: i32 = 12;
/// The kid's height in pixels, legs included.
const KID_H: i32 = KID.len() as i32 + 4;
/// The kid's shoulder in the art, where the arms and the stick start.
const SHOULDER: (f32, f32) = (6.0, 10.0);
/// The stick: the arm's length from the shoulder, then the stick itself.
const ARM: f32 = 4.0;
const STICK: f32 = 17.0;

/// The piñata: the body's radius and the cones' length, in pixels.
const BODY: f32 = 4.5;
const CONE: f32 = 8.0;
const RADIUS: f32 = BODY + CONE;
const PICOS: usize = 7;
/// The cones' colors, one each.
const PAPER: [u8; PICOS] = *b"ZPQOMLR";
/// How big the piñata's picture is, in pixels square.
const SIDE: usize = 2 * RADIUS as usize + 4;

/// The kid's position relative to the piñata: columns to its left, and the
/// height of the shoulder above the ground.
const STANCE: f32 = 19.0 + RADIUS - 1.0;
const SHOULDER_UP: f32 = KID_H as f32 - 10.0;
/// The piñata hangs this high above the ground when it is within reach, and
/// is pulled up this far to be out of it.
const HANG: f32 = 22.0;
const PULL: f32 = 16.0;

/// The swings: when the first comes, how long each takes and how far apart,
/// and which of them land.
const SWING: f32 = 0.7;
const PERIOD: f32 = 1.5;
/// The instant of contact into the swing.
const CONTACT: f32 = 0.36;
const LANDS: [bool; 7] = [false, false, true, false, true, true, true];
/// How stiffly the swinging rope returns, per second squared.
const SPRING: f32 = 10.0;

/// What the kid sings, and what comes after the piñata breaks.
const CHANT: &str = "¡Dale, dale, dale, no pierdas el tino!";
const CHANT_LINES: [&str; 2] = ["¡Dale, dale, dale,", "no pierdas el tino!"];
const AFTER: &str = "¡Se rompió! ¡A juntar la colación!";
const AFTER_LINES: [&str; 2] = ["¡Se rompió!", "¡A juntar la colación!"];
const TYPING: f32 = 11.0;

/// Pieces flying or lying about at most.
const PIECES: usize = 150;
/// How long a piece rests on the ground before it fades.
const REST: f32 = 2.2;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Pinata::new(w, h, rng))
}

impl Pinata {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Pinata {
        let (wf, hf) = (w as f32, h as f32 * 2.0);
        let mut rng = fastrand::Rng::with_seed(rng.u64(..));
        // The piñata hangs somewhere in the middle of the screen; the kid stands
        // to one side of it and walks in from that edge.
        let lo = (STANCE + 14.0).min(wf / 2.0);
        let x = (wf * (0.4 + 0.2 * rng.f32())).clamp(lo, (wf - 14.0).max(lo));
        let left = rng.bool();
        let x = if left { wf - x } else { x };
        let kid_x = if left { x + STANCE } else { x - STANCE };
        let from = if left { wf + 14.0 } else { -14.0 };
        let enter = ((from - kid_x).abs() / 14.0).clamp(1.0, 4.0);
        let first = enter + 1.3;
        Pinata {
            t: 0.0,
            ground: hf,
            x,
            left,
            kid_x,
            from,
            enter,
            first,
            angle: 0.0,
            swing: 0.0,
            turn: 0.0,
            spin: 0.0,
            hits: 0,
            burst: None,
            pieces: [Piece::default(); PIECES],
            next: 0,
            rain: 0.0,
            rng,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Piece {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// What it is: 0 colación, 1 cacahuate, 2 tejocote, 3 mandarina, 4 caña,
    /// 5 a bit of papel de china, 6 a spark.
    kind: u8,
    color: u8,
    /// Seconds in the air; seconds resting on the ground.
    age: f32,
    rest: f32,
    spin: f32,
}

struct Pinata {
    t: f32,
    /// The row under the kid's feet, in pixels.
    ground: f32,
    /// Where the piñata hangs, and the kid stands (columns).
    x: f32,
    left: bool,
    kid_x: f32,
    /// Where the kid comes in from, and the seconds it takes.
    from: f32,
    enter: f32,
    /// When the first swing starts.
    first: f32,
    /// The rope's angle from the vertical and its angular speed, and the
    /// piñata's own turning (its angle and speed).
    angle: f32,
    swing: f32,
    turn: f32,
    spin: f32,
    /// Blows landed so far.
    hits: usize,
    /// The moment it burst (and where, in pixels), once it has.
    burst: Option<(f32, f32)>,
    pieces: [Piece; PIECES],
    next: usize,
    /// Sweets still to pour out, once it has burst.
    rain: f32,
    rng: fastrand::Rng,
}

/// Smooth from 0 to 1.
fn smooth(k: f32) -> f32 {
    let k = k.clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

impl Pinata {
    /// When swing `k` starts, and when it makes contact.
    fn at(&self, k: usize) -> f32 {
        self.first + k as f32 * PERIOD
    }

    /// How far the piñata has been pulled up by the rope at time `t`: out of
    /// reach for the swings that miss, down for those that land.
    fn pull(&self, t: f32) -> f32 {
        let (mut t0, mut v0) = (0.0, 0.5 * PULL);
        for (k, &lands) in LANDS.iter().enumerate() {
            let (t1, v1) = (self.at(k) + CONTACT, if lands { 0.0 } else { PULL });
            if t < t1 {
                return v0 + (v1 - v0) * smooth((t - t0) / (t1 - t0));
            }
            (t0, v0) = (t1, v1);
        }
        v0
    }

    /// Where the piñata's center is, in pixels.
    fn center(&self) -> (f32, f32) {
        if let Some(b) = self.burst {
            return b;
        }
        // It is let down from above the screen at the start.
        let drop = (1.0 - self.t / 1.4).max(0.0).powi(2) * (self.ground + RADIUS);
        let depth = (self.ground - HANG - self.pull(self.t) - drop).max(0.0);
        (self.x + depth * self.angle.sin(), depth * self.angle.cos())
    }

    /// The stick's angle above the horizontal, facing the piñata, for the
    /// swing at time `t` (0 when it rests).
    fn stick(&self, t: f32) -> f32 {
        let hit = 25f32.to_radians();
        let (back, rest) = (140f32.to_radians(), 55f32.to_radians());
        let s = t - self.first;
        if s < 0.0 {
            // Held ready, wagging a little.
            return rest + 0.08 * (self.t * 3.0).sin();
        }
        let k = (s / PERIOD) as usize;
        if k >= LANDS.len() {
            // Held up high in triumph.
            return 85f32.to_radians();
        }
        let p = s - k as f32 * PERIOD;
        let through = if LANDS[k] {
            12f32.to_radians()
        } else {
            -35f32.to_radians()
        };
        if p < 0.55 * CONTACT {
            // Wind up.
            rest + (back - rest) * smooth(p / (0.55 * CONTACT))
        } else if p < CONTACT {
            // Swing through: back over the shoulder, down to the piñata.
            back + (hit - back) * smooth((p - 0.55 * CONTACT) / (0.45 * CONTACT))
        } else if p < SWING {
            hit + (through - hit) * smooth((p - CONTACT) / (SWING - CONTACT)).sqrt()
        } else {
            // Back to ready.
            through + (rest - through) * smooth((p - SWING) / (PERIOD - SWING))
        }
    }

    /// Throws out a piece from (`x`, `y`).
    fn throw(&mut self, (x, y): (f32, f32), kind: u8, speed: f32) {
        let ang = self.rng.f32() * TAU;
        let r = self.rng.f32().sqrt() * speed;
        let color = match kind {
            0 => b"PZLQMW"[self.rng.usize(..6)],
            1 => b'c',
            2 => b'T',
            3 => b'O',
            4 => b'C',
            5 => PAPER[self.rng.usize(..PICOS)],
            _ => b'X',
        };
        self.pieces[self.next] = Piece {
            x,
            y,
            vx: ang.cos() * r,
            vy: ang.sin() * r * 0.8 - 12.0,
            kind,
            color,
            age: 1e-3,
            rest: 0.0,
            spin: self.rng.f32() * TAU,
        };
        self.next = (self.next + 1) % PIECES;
    }

    /// The kid's shoulder, in pixels.
    fn shoulder(&self) -> (f32, f32) {
        let dx = SHOULDER.0 - KID_W as f32 / 2.0;
        let side = if self.left { -1.0 } else { 1.0 };
        (self.kid_x + dx * side, self.ground - SHOULDER_UP)
    }

    /// The kid's tip of the stick.
    fn tip(&self, angle: f32) -> (f32, f32) {
        let (sx, sy) = self.shoulder();
        let side = if self.left { -1.0 } else { 1.0 };
        let r = ARM + STICK;
        (sx + side * r * angle.cos(), sy - r * angle.sin())
    }
}

impl Egg for Pinata {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        let before = self.t;
        self.t += dt;
        // The rope swings and the piñata turns, a step at a time.
        let steps = (dt * 60.0).ceil().max(1.0) as usize;
        let h = dt / steps as f32;
        for _ in 0..steps {
            // Critically damped: it jolts away and is back by the next blow.
            let damping = 2.0 * SPRING.sqrt();
            self.swing += (-SPRING * self.angle - damping * self.swing) * h;
            self.angle += self.swing * h;
            self.spin -= 1.4 * self.spin * h;
            self.turn += self.spin * h;
        }
        // The blows: each lands (or whistles by) at its moment of contact.
        if self.burst.is_none() {
            for (k, &lands) in LANDS.iter().enumerate() {
                let contact = self.at(k) + CONTACT;
                if !(before < contact && contact <= self.t) {
                    continue;
                }
                if !lands {
                    continue;
                }
                self.hits += 1;
                let from = self.center();
                let push = if self.left { -1.0 } else { 1.0 };
                // The rope's swing for a push of 40 pixels a second.
                self.swing += push * 40.0 / self.center().1.max(20.0);
                self.spin += push * (3.0 + self.rng.f32() * 3.0);
                // Sparks fly from where the stick met the paper.
                let at = self.tip(self.stick(contact));
                for _ in 0..6 {
                    self.throw(at, 6, 30.0);
                }
                if self.hits == 2 {
                    self.throw(from, 0, 12.0);
                    self.throw(from, 0, 12.0);
                }
                if self.hits == 3 {
                    for kind in [0, 0, 0, 1, 2] {
                        self.throw(from, kind, 14.0);
                    }
                }
                if self.hits == LANDS.iter().filter(|&&l| l).count() {
                    self.burst = Some(from);
                    self.rain = 1.0;
                    for _ in 0..22 {
                        self.throw(from, 5, 38.0);
                    }
                    for i in 0..44 {
                        let kind = [0, 0, 1, 2, 3, 4, 0, 0][i % 8];
                        self.throw(from, kind, 46.0);
                    }
                }
            }
        }
        // The rest of the sweets pour out of the broken piñata.
        if let Some(from) = self.burst {
            let wanted = self.rain;
            self.rain = (self.rain - dt * 1.4).max(0.0);
            let n = ((wanted - self.rain) * 40.0) as usize;
            for i in 0..n {
                let kind = [0, 1, 0, 2, 4, 0, 3][i % 7];
                self.throw(from, kind, 22.0);
            }
        }
        let ground = self.ground;
        for p in self.pieces.iter_mut().filter(|p| p.age > 0.0) {
            if p.rest > 0.0 {
                p.rest += dt;
                if p.rest > REST {
                    p.age = 0.0;
                }
                continue;
            }
            p.age += dt;
            match p.kind {
                5 => {
                    // Paper flutters down.
                    p.vy += (14.0 - p.vy) * (1.0 - (-2.0 * dt).exp());
                    p.vx *= (-1.2 * dt).exp();
                    p.x += (p.vx + (p.age * 5.0 + p.spin).sin() * 6.0) * dt;
                }
                6 => {
                    // Sparks from a blow die quickly.
                    p.vy += 40.0 * dt;
                    p.x += p.vx * dt;
                    if p.age > 0.35 {
                        p.age = 0.0;
                    }
                }
                _ => {
                    p.vy += 85.0 * dt;
                    p.x += p.vx * dt;
                }
            }
            p.y += p.vy * dt;
            if p.y >= ground - 1.0 && p.kind != 6 {
                p.y = ground - 1.0;
                if p.kind == 5 || p.vy < 12.0 {
                    p.rest = 1e-3;
                    p.vx = 0.0;
                } else {
                    // A bounce, losing most of its speed.
                    p.vy *= -0.38;
                    p.vx *= 0.6;
                }
            }
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::navidad(theme);
        // The kid, jumping for joy once the piñata has burst, and the piñata
        // until then; the sweets fall in front of both.
        self.draw_kid(c, &pal, self.burst.is_some());
        if self.burst.is_none() {
            self.draw_pinata(c, &pal);
        }
        for p in self.pieces.iter().filter(|p| p.age > 0.0) {
            let fading = (p.rest / REST).powi(2);
            let (x, y) = (p.x.floor() as i32, p.y.floor() as i32);
            let col = |letter: u8| fade(theme, pal.of(letter), fading);
            match p.kind {
                // Colación: a pinch of sugar that twinkles.
                0 => pixel(c, x, y, col(p.color)),
                // A cacahuate, a pair of pixels.
                1 => {
                    pixel(c, x, y, col(b'c'));
                    pixel(c, x + 1, y, col(b'C'));
                }
                // A tejocote or a mandarina, a round 2 by 2.
                2 | 3 => {
                    let (a, b) = if p.kind == 2 {
                        (b'T', b'O')
                    } else {
                        (b'O', b'o')
                    };
                    pixel(c, x, y, col(a));
                    pixel(c, x + 1, y, col(a));
                    pixel(c, x, y + 1, col(a));
                    pixel(c, x + 1, y + 1, col(b));
                }
                // A caña, a stick that tumbles in the air.
                4 => {
                    let upright = (p.age * 5.0 + p.spin) as i32 % 2 == 0 && p.rest == 0.0;
                    for i in 0..3 {
                        let (dx, dy) = if upright { (0, i) } else { (i, 0) };
                        pixel(c, x + dx, y + dy, col(if i == 1 { b'y' } else { b'C' }));
                    }
                }
                // Papel de china, turning over as it falls.
                5 => {
                    let wide = (p.age * 4.0 + p.spin) as i32 % 2 == 0;
                    pixel(c, x, y, col(p.color));
                    if wide {
                        pixel(c, x + 1, y, col(p.color));
                    }
                }
                // A spark.
                _ => {
                    let k = p.age / 0.35;
                    pixel(
                        c,
                        x,
                        y,
                        fade(theme, pal.of(if k < 0.5 { b'X' } else { b'F' }), k),
                    );
                }
            }
        }
        // The song, typed above them.
        let first = self.first - 1.0;
        let (text, lines, from): (&str, &[&str; 2], f32) = match self.burst {
            None => (CHANT, &CHANT_LINES, first.max(0.3)),
            Some(_) => (AFTER, &AFTER_LINES, self.burst_at()),
        };
        let shown = ((self.t - from) * TYPING).max(0.0) as usize;
        let gone = if self.burst.is_some() {
            ((self.t - self.burst_at() - 7.0) / 2.0).clamp(0.0, 1.0)
        } else {
            0.0
        };
        if shown > 0 && gone < 1.0 {
            let one = c.w() as usize >= text.chars().count() + 4;
            let lines: &[&str] = if one { &[text] } else { lines };
            let top = self.ground - HANG - PULL - RADIUS - 8.0;
            let row = ((top / 2.0) as i32 - lines.len() as i32).max(0);
            let cursor = (shown < text.chars().count()).then(|| pal.of(b'Y'));
            let ink = fade(theme, pal.of(b'y'), gone);
            type_out(c, row, lines, shown, (ink, cursor), BOLD);
        }
    }

    fn done(&self) -> bool {
        let over =
            self.burst.is_some() && self.rain == 0.0 && self.pieces.iter().all(|p| p.age == 0.0);
        over || self.t > 29.5
    }
}

impl Pinata {
    /// When it burst: the contact of the last blow.
    fn burst_at(&self) -> f32 {
        self.at(LANDS.len() - 1) + CONTACT
    }

    /// Draws the kid: walking in, dizzy, swinging, then jumping for joy.
    fn draw_kid(&self, c: &mut Canvas, pal: &Palette, joy: bool) {
        let shirt = pal.of(b'G');
        let (shade, hair) = (shirt.scale(0.7), pal.of(b'k'));
        let color = |ch: u8| match ch {
            b'x' => Some(shirt),
            b'z' => Some(shade),
            b'j' => Some(hair),
            _ => pal.get(ch),
        };
        // Walking in from the edge, then standing in place.
        let k = (self.t / self.enter).min(1.0);
        let walking = k < 1.0;
        let x = self.from + (self.kid_x - self.from) * k;
        let step = walking && (x / 2.5) as i32 % 2 == 0;
        // Spun around and around, dizzy, before the first swing.
        let dizzy = self.t > self.enter && self.t < self.first - 0.2;
        let turning = dizzy && ((self.t * 6.0) as i32 % 2 == 1);
        let flip = self.left != turning;
        // The jump for joy.
        let hop = if joy {
            ((self.t * 7.0).sin().abs() * 3.0).round() as i32
        } else {
            0
        };
        let feet = (self.ground as i32) - hop;
        let top = feet - KID_H;
        let col0 = x.round() as i32 - KID_W / 2;
        let legs = KID_LEGS[usize::from(step)];
        sprite(c, (col0, top + KID.len() as i32), legs, flip, |_| 0, color);
        sprite(c, (col0, top), KID, flip, |_| 0, color);
        if dizzy {
            // Little stars circle the head.
            for i in 0..3 {
                let a = self.t * 7.0 + i as f32 * TAU / 3.0;
                let (sx, sy) = (a.cos() * 7.0, a.sin() * 1.6 - 3.0);
                let star = if i == 1 { b'Y' } else { b'X' };
                pixel(
                    c,
                    x.round() as i32 + sx.round() as i32,
                    top + sy.round() as i32,
                    pal.of(star),
                );
            }
            return;
        }
        if walking {
            return;
        }
        // The arms and the stick, from the shoulder.
        let angle = if joy {
            // Held up in triumph after the last blow.
            let k = smooth((self.t - self.burst_at()) / 0.5);
            let hit = 25f32.to_radians();
            hit + (85f32.to_radians() - hit) * k + 0.12 * k * (self.t * 9.0).sin()
        } else {
            self.stick(self.t)
        };
        let sign = if self.left { -1.0 } else { 1.0 };
        let (sx, sy) = (self.shoulder().0, (top as f32) + SHOULDER.1);
        let at = |r: f32| (sx + sign * r * angle.cos(), sy - r * angle.sin());
        let wood = pal.of(b'B');
        let stripes = [pal.of(b'P'), pal.of(b'Z'), pal.of(b'Q')];
        let n = ((ARM + STICK) * 2.0) as i32;
        for i in 0..=n {
            let r = i as f32 * 0.5;
            let (px, py) = at(r);
            let col = if r < ARM * 0.6 {
                shirt
            } else if r < ARM {
                pal.of(b'e')
            } else {
                match ((r - ARM) / 2.0) as usize {
                    j if j % 3 == 0 && j > 0 => stripes[(j / 3) % 3],
                    _ => wood,
                }
            };
            pixel(c, px.round() as i32, py.round() as i32, col);
        }
    }

    /// Draws the piñata and its rope.
    fn draw_pinata(&self, c: &mut Canvas, pal: &Palette) {
        let (cx, cy) = self.center();
        // The rope from the top of the screen to the top of the piñata.
        let (ux, uy) = (self.angle.sin(), self.angle.cos());
        let (ax, ay) = (cx - ux * (RADIUS - 1.0), cy - uy * (RADIUS - 1.0));
        let rope = (ay / 0.5).max(1.0) as i32;
        for i in 0..=rope {
            let k = i as f32 / rope as f32;
            let (px, py) = (self.x + (ax - self.x) * k, ay * k);
            let twist = if (py as i32 / 2) % 2 == 0 { b'C' } else { b'c' };
            pixel(c, px.round() as i32, py.round() as i32, pal.of(twist));
        }
        // Rasterize the star into a small picture first (the letter of the
        // paper on each pixel, and whether it is a lighter band), so its edges
        // can be shaded.
        let mut grid = [[(0u8, false); SIDE]; SIDE];
        let half = SIDE as f32 / 2.0;
        let rot = self.turn + self.angle * 0.8 + 0.3 * (self.t * 1.3).sin();
        let root = BODY * 0.5;
        for (gy, row) in grid.iter_mut().enumerate() {
            for (gx, cell) in row.iter_mut().enumerate() {
                let (dx, dy) = (gx as f32 + 0.5 - half, gy as f32 + 0.5 - half);
                let r = (dx * dx + dy * dy).sqrt();
                if r <= BODY {
                    // The round body, ringed like a target.
                    let ring = if r < 1.7 {
                        b'Z'
                    } else if r < 3.2 {
                        b'O'
                    } else {
                        b'P'
                    };
                    *cell = (ring, false);
                    continue;
                }
                for (i, &paper) in PAPER.iter().enumerate() {
                    let ang = rot + i as f32 * TAU / PICOS as f32;
                    // Along the cone's axis and across it.
                    let (a, b) = (
                        dx * ang.sin() - dy * ang.cos(),
                        dx * ang.cos() + dy * ang.sin(),
                    );
                    if a < root || a > RADIUS {
                        continue;
                    }
                    let half_w = 3.1 * (1.0 - (a - root) / (RADIUS - root)) + 0.5;
                    if b.abs() > half_w {
                        continue;
                    }
                    // Fringed at the tip: every other bit is missing.
                    if a > RADIUS - 2.5 && ((b * 1.4).round() as i32 + i as i32) % 2 != 0 {
                        continue;
                    }
                    // Crinkled paper: lighter bands across the cone.
                    *cell = (paper, ((a - root) / 2.6) as i32 % 2 == 1);
                    break;
                }
            }
        }
        let (x0, y0) = (
            cx.round() as i32 - SIDE as i32 / 2,
            cy.round() as i32 - SIDE as i32 / 2,
        );
        for gy in 0..SIDE {
            for gx in 0..SIDE {
                let (letter, light) = grid[gy][gx];
                if letter == 0 {
                    continue;
                }
                let paper = pal.of(letter);
                let base = if light { paper.scale(1.22) } else { paper };
                // Edges are a darker shade of the paper.
                let empty = |dx: i32, dy: i32| {
                    let (nx, ny) = (gx as i32 + dx, gy as i32 + dy);
                    nx < 0
                        || ny < 0
                        || nx >= SIDE as i32
                        || ny >= SIDE as i32
                        || grid[ny as usize][nx as usize].0 == 0
                };
                let edge = empty(-1, 0) || empty(1, 0) || empty(0, -1) || empty(0, 1);
                let col = if edge { base.scale(0.72) } else { base };
                pixel(c, x0 + gx as i32, y0 + gy as i32, col);
            }
        }
        // The ring the rope is tied to.
        pixel(c, ax.round() as i32, ay.round() as i32, pal.of(b'Y'));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        assert!(KID.iter().all(|r| r.len() as i32 == KID_W));
        assert!(
            KID_LEGS
                .iter()
                .all(|l| l.len() == 4 && l.iter().all(|r| r.len() as i32 == KID_W))
        );
        assert_eq!(CHANT_LINES.join(" "), CHANT);
        assert_eq!(AFTER_LINES.join(" "), AFTER);
        assert!(LANDS[LANDS.len() - 1], "the last blow must land");
    }

    /// Steps `p` to time `t`.
    fn run_to(p: &mut Pinata, t: f32) {
        while p.t < t {
            let dt = (t - p.t).min(1.0 / 60.0);
            p.update(dt);
        }
    }

    /// The stick reaches the piñata on the blows that land and whistles by
    /// under it on the others.
    #[test]
    fn blows_land_or_miss_as_they_should() {
        for (w, h) in [(80, 24), (200, 60), (60, 20), (500, 150)] {
            for seed in 0..6 {
                let mut p = Pinata::new(w, h, &mut fastrand::Rng::with_seed(seed));
                for (k, &lands) in LANDS.iter().enumerate() {
                    let contact = p.at(k) + CONTACT - 0.01;
                    run_to(&mut p, contact);
                    if p.burst.is_some() {
                        break;
                    }
                    let tip = p.tip(p.stick(p.t));
                    let (cx, cy) = p.center();
                    let d = ((tip.0 - cx).powi(2) + (tip.1 - cy).powi(2)).sqrt();
                    if lands {
                        assert!(d <= RADIUS + 2.0, "blow {k} falls short by {d} at {w}x{h}");
                    } else {
                        assert!(d >= RADIUS + 1.0, "blow {k} hits at {d} at {w}x{h}");
                    }
                }
            }
        }
    }

    /// It bursts at the last blow, the sweets rain down and settle, and the
    /// whole thing is over in good time.
    #[test]
    fn bursts_rains_and_ends() {
        let theme = Theme::classic();
        let mut p = Pinata::new(100, 30, &mut fastrand::Rng::with_seed(5));
        let mut c = Canvas::new(100, 30, Color::Reset);
        let mut shown = false;
        while !p.done() {
            p.update(0.1);
            c.clear();
            p.draw(&mut c, &theme);
            shown |= c.to_text().contains(CHANT);
        }
        assert!(shown, "the chant never came");
        assert!(p.burst.is_some() && p.hits == 4, "{} hits", p.hits);
        assert!(p.t < 29.0, "{}", p.t);
    }
}
