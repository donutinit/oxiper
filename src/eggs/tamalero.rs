//! A tamalero pedals across the bottom of the screen on Candelaria, calling
//! his tamales through a loudspeaker.
//!
//! Original oxiper pixel art of the cargo tricycle the tamaleros ride around
//! Mexican neighborhoods in the early morning: a blue box in front with
//! TAMALES painted on a cream panel, a big aluminum tamalera on top, its lid
//! rattling and steam puffing out from under it, and a horn on a pole that
//! plays the recording everybody knows. The tamalero sits behind in a red
//! cap and a white apron, hands on the handlebar, and pedals: his legs, the
//! cranks and the wheels are drawn from their angles so they turn together
//! with the distance covered. The steam rises in puffs that swell and thin
//! out, and the call scrolls out of the horn letter by letter, in cream and
//! gold, rising a little on the air and fading: "¡Ricos tamales oaxaqueños…
//! tamales calientitos!", again every few seconds. He comes from either side
//! and is gone within 30 s.

use super::candelaria::{Palette, fade};
use super::pen::{text, trace};
use super::{Egg, Info, pixel};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "tamalero",
    start,
    fiesta: Kind::Candelaria,
    when: super::any_day,
};

/// The tricycle's size in pixels, facing right: the horn's mouth is the
/// last column and the ground is the bottom row.
const WIDTH: i32 = 61;
const HEIGHT: i32 = 46;
/// The wheels' centers and radius: the one behind, and the near one of the
/// two in front, under the box.
const REAR: (f32, f32) = (8.5, 39.5);
const FRONT: (f32, f32) = (46.5, 39.5);
const WHEEL: f32 = 6.5;
/// The bottom bracket, where the cranks turn, and their length.
const BRACKET: (f32, f32) = (20.5, 38.5);
const CRANK: f32 = 3.6;
/// His hips on the saddle, and where his shoulder and his hand are.
const HIP: (f32, f32) = (14.5, 28.5);
const SHOULDER: (f32, f32) = (18.5, 17.0);
const GRIP: (f32, f32) = (24.5, 17.5);
/// The box, the pot on it, and the pole the horn is on.
const BOX: (i32, i32, i32, i32) = (24, 21, 54, 33);
const POT_X: (i32, i32) = (27, 43);
const POLE_X: i32 = 48;
/// Where the call comes out: the horn's mouth, and the sign's top left.
const MOUTH: (i32, i32) = (60, 4);
const SIGN: (i32, i32) = (26, 25);

/// What he calls, and how fast the letters leave the horn (columns per
/// second), how long one lives, and the time from one call to the next.
const CALL: &str = "¡Ricos tamales oaxaqueños… tamales calientitos!";
const LEAVE: f32 = 9.0;
const LIFE: f32 = 5.4;
const EVERY: f32 = 7.5;
const FIRST_CALL: f32 = 1.5;

/// Seconds the crossing takes, give or take, and the slowest pace (columns
/// per second); wide screens go faster so he is across in time.
const CROSSING: f32 = 21.0;
const PACE: f32 = 5.5;
/// Pixels of road per turn of the crank (a wheel turns once in WHEEL * TAU).
const CRANK_TURN: f32 = 20.0;

/// Puffs of steam: how many at once, how often one comes, how long it lasts.
const PUFFS: usize = 40;
const PUFF_EVERY: f32 = 0.09;
const PUFF_LIFE: f32 = 2.6;

fn start(w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Tamalero {
        t: 0.0,
        speed: PACE.max((w as f32 + WIDTH as f32 + 4.0) / CROSSING),
        left: rng.bool(),
        w: w as i32,
        puffs: [Puff::default(); PUFFS],
        next: 0,
        puff_in: 0.3,
        rng: fastrand::Rng::with_seed(rng.u64(..)),
    })
}

#[derive(Clone, Copy, Default)]
struct Puff {
    /// Pixels from the left edge and from the top.
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Seconds since it came out; 0 means none.
    age: f32,
    phase: f32,
}

struct Tamalero {
    t: f32,
    /// Columns per second.
    speed: f32,
    /// Riding right to left (mirrored)?
    left: bool,
    w: i32,
    puffs: [Puff; PUFFS],
    next: usize,
    puff_in: f32,
    rng: fastrand::Rng,
}

impl Tamalero {
    /// The tricycle's left column on the screen, mirrored or not.
    fn x(&self) -> i32 {
        let d = (self.t * self.speed) as i32;
        if self.left {
            self.w + 2 - d
        } else {
            d - WIDTH - 2
        }
    }

    /// The road he has covered, in pixels: it turns the cranks and the
    /// wheels alike.
    fn road(&self) -> f32 {
        self.t * self.speed
    }

    /// The screen column of a column of the art, which is mirrored when he
    /// rides left.
    fn col(&self, lx: i32) -> i32 {
        if self.left {
            self.x() + WIDTH - 1 - lx
        } else {
            self.x() + lx
        }
    }

    /// How far the lid is lifted by the steam, in pixels: it rattles.
    fn lid(&self) -> i32 {
        i32::from((self.t * 7.0).sin() + (self.t * 11.3).sin() > 0.9)
    }

    /// The top of the art, in pixels from the top of the screen.
    fn top(&self, h: i32) -> i32 {
        h * 2 - HEIGHT
    }
}

/// The tricycle in its own pixels: x to the right from its tail, y down,
/// facing right, mirrored when it rides left.
struct Rig<'a> {
    c: &'a mut Canvas,
    pal: &'a Palette,
    /// The screen's column and pixel row of the art's top-left corner.
    x: i32,
    y: i32,
    left: bool,
}

impl Rig<'_> {
    fn put(&mut self, lx: i32, ly: i32, ch: u8) {
        let x = if self.left {
            self.x + WIDTH - 1 - lx
        } else {
            self.x + lx
        };
        if let Some(color) = self.pal.get(ch) {
            pixel(self.c, x, self.y + ly, color);
        }
    }

    fn rect(&mut self, (x0, y0, x1, y1): (i32, i32, i32, i32), ch: u8) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.put(x, y, ch);
            }
        }
    }

    /// A line between two points (the pixels they are in).
    fn line(&mut self, a: (f32, f32), b: (f32, f32), ch: u8) {
        let p = |v: (f32, f32)| (v.0.floor() as i32, v.1.floor() as i32);
        let mut pts = [(0, 0); 64];
        let mut n = 0;
        trace(p(a), p(b), |x, y| {
            if n < pts.len() {
                pts[n] = (x, y);
                n += 1;
            }
        });
        for &(x, y) in &pts[..n] {
            self.put(x, y, ch);
        }
    }

    /// A line two pixels thick.
    fn thick(&mut self, a: (f32, f32), b: (f32, f32), ch: u8) {
        let flat = (b.0 - a.0).abs() > (b.1 - a.1).abs();
        let (dx, dy) = if flat { (0.0, 1.0) } else { (1.0, 0.0) };
        self.line(a, b, ch);
        self.line((a.0 + dx, a.1 + dy), (b.0 + dx, b.1 + dy), ch);
    }

    fn disc(&mut self, (cx, cy): (f32, f32), r: f32, ch: u8) {
        for y in (cy - r).floor() as i32..=(cy + r).ceil() as i32 {
            for x in (cx - r).floor() as i32..=(cx + r).ceil() as i32 {
                if (x as f32 + 0.5 - cx).hypot(y as f32 + 0.5 - cy) <= r {
                    self.put(x, y, ch);
                }
            }
        }
    }

    /// A leg from the hip to a pedal: thigh and shin, the knee bent forward.
    fn leg(&mut self, hip: (f32, f32), pedal: (f32, f32), cloth: u8) {
        let ankle = (pedal.0 - 0.5, pedal.1 - 1.5);
        let knee = joint(hip, ankle, 10.0, 10.0, -1.0);
        self.thick(hip, knee, cloth);
        self.thick((hip.0 - 1.0, hip.1), (knee.0 - 1.0, knee.1), cloth);
        self.thick(knee, ankle, cloth);
        // A white shoe.
        self.rect(
            (
                (pedal.0 - 1.0).floor() as i32,
                (pedal.1 - 0.5).floor() as i32,
                (pedal.0 + 2.0).floor() as i32,
                (pedal.1 + 0.5).floor() as i32,
            ),
            b'W',
        );
    }

    /// A wheel: tire, bright rim, eight spokes turning with `angle` and a hub.
    fn wheel(&mut self, (cx, cy): (f32, f32), angle: f32) {
        for y in (cy - 7.0) as i32..=(cy + 7.0) as i32 {
            for x in (cx - 7.0) as i32..=(cx + 7.0) as i32 {
                let d = (x as f32 + 0.5 - cx).hypot(y as f32 + 0.5 - cy);
                if (5.0..6.6).contains(&d) {
                    self.put(x, y, b'N');
                } else if (4.1..5.0).contains(&d) {
                    self.put(x, y, b'S');
                }
            }
        }
        for k in 0..8 {
            let a = angle + k as f32 * PI / 4.0;
            self.line((cx, cy), (cx + 4.4 * a.cos(), cy + 4.4 * a.sin()), b's');
        }
        self.disc((cx, cy), 1.5, b'z');
        self.put(cx.floor() as i32, cy.floor() as i32, b'S');
    }
}

/// The bend of a limb of two bones, `a` and `b` long, reaching from `from`
/// toward `to` (as far as it can): which way it bends is `side`, 1 or -1.
fn joint(from: (f32, f32), to: (f32, f32), a: f32, b: f32, side: f32) -> (f32, f32) {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let dist = dx.hypot(dy).max(1e-3);
    let reach = dist.min(a + b - 0.01);
    let along = (reach * reach + a * a - b * b) / (2.0 * reach);
    let off = (a * a - along * along).max(0.0).sqrt() * side;
    let (ux, uy) = (dx / dist, dy / dist);
    (
        from.0 + ux * along - uy * off,
        from.1 + uy * along + ux * off,
    )
}

/// A letter of aluminum for a point across a cylinder, from -1 (the lit
/// edge) to 1: a bright stripe where the light hits, darker to the far side.
fn metal(u: f32) -> u8 {
    match u {
        u if u < -0.82 => b's',
        u if u < -0.62 => b'S',
        u if u < -0.4 => b'W',
        u if u < 0.05 => b'S',
        u if u < 0.5 => b's',
        _ => b'z',
    }
}

impl Rig<'_> {
    /// The whole tricycle with its rider, the cranks at `crank` and the
    /// wheels turned `spin`, the lid lifted `lid` pixels.
    fn vehicle(&mut self, crank: f32, spin: f32, lid: i32) {
        let pedal = |a: f32| (BRACKET.0 + CRANK * a.cos(), BRACKET.1 + CRANK * a.sin());
        // Riding is work: he rises a little on each downstroke.
        let hip = (HIP.0, HIP.1 + f32::from((crank * 2.0).sin() > 0.4));
        let far = pedal(crank + PI);
        let near = pedal(crank);

        // The far leg and crank, in the shade.
        self.line(BRACKET, far, b'z');
        self.leg(hip, far, b'j');

        // The frame: chain stay, seat stay, seat tube and down tube.
        self.line(REAR, BRACKET, b'B');
        self.line(REAR, (14.0, 29.5), b'B');
        self.line(BRACKET, (15.0, 29.5), b'B');
        self.line(
            (BRACKET.0 + 1.0, BRACKET.1),
            (BOX.0 as f32 + 1.0, 33.5),
            b'B',
        );
        self.line((FRONT.0, FRONT.1), (FRONT.0, 34.0), b's');
        // The saddle and its post.
        self.rect((11, 29, 17, 29), b'z');
        self.rect((14, 30, 14, 30), b'z');

        // The horn's pole and the horn: its driver, then the bell opening
        // toward the mouth, lit on top and dark below, and a rolled lip.
        self.rect((POLE_X, 9, POLE_X, BOX.1), b's');
        self.rect((POLE_X - 1, 9, POLE_X + 1, 9), b'z');
        self.rect((POLE_X - 2, 2, POLE_X + 1, 6), b'z');
        self.rect((POLE_X - 2, 3, POLE_X - 1, 5), b's');
        for x in POLE_X + 2..=MOUTH.0 {
            let k = (x - POLE_X - 2) as f32 / (MOUTH.0 - POLE_X - 2) as f32;
            let half = 1.6 + 3.2 * k * k.sqrt();
            let (top, bottom) = ((4.5 - half).round() as i32, (4.5 + half).round() as i32);
            for y in top..=bottom {
                let u = (y as f32 + 0.5 - 4.5) / half;
                let ch = if x >= MOUTH.0 - 1 {
                    b'W'
                } else if u.abs() < 0.5 && x > MOUTH.0 - 5 {
                    b'k'
                } else {
                    metal(u + 0.3)
                };
                self.put(x, y, ch);
            }
        }

        // The box: a rim of wood, blue sides, a cream panel with the name.
        let (bx0, by0, bx1, by1) = BOX;
        self.rect((bx0, by0, bx1, by0 + 1), b'D');
        self.rect((bx0, by0 + 2, bx1, by1), b'B');
        self.rect((bx0, by1 - 1, bx1, by1), b'b');
        self.rect((bx0 + 1, by0 + 2, bx1 - 1, by0 + 10), b'M');
        self.rect((bx0 + 1, by0 + 10, bx1 - 1, by0 + 10), b'm');

        // A few tamales in their husks, stacked beside the pole.
        for (x, y) in [(49, 19), (52, 19), (50, 17)] {
            self.rect((x, y, x + 2, y + 1), b'Y');
            self.put(x, y + 1, b'y');
            self.put(x + 2, y, b'M');
        }

        // The tamalera: a cylinder of aluminum with bands and rivets.
        let (px0, px1) = POT_X;
        let cx = (px0 + px1 + 1) as f32 / 2.0;
        let half = (px1 - px0 + 1) as f32 / 2.0;
        for y in 7..=20 {
            for x in px0..=px1 {
                let u = (x as f32 + 0.5 - cx) / half;
                let ch = match y {
                    8 | 18 if u < 0.2 => b's',
                    8 | 18 => b'z',
                    14 if (x - px0) % 4 == 2 => b'z',
                    20 => b'z',
                    _ => metal(u),
                };
                self.put(x, y, ch);
            }
        }
        // Handles on either side, and a cloth tied to one.
        for (x, ch) in [
            (px0 - 2, b'z'),
            (px0 - 1, b'z'),
            (px1 + 1, b'z'),
            (px1 + 2, b'z'),
        ] {
            self.put(x, 11, ch);
            self.put(x, 12, ch);
        }
        self.rect((px0 - 2, 13, px0 - 1, 17), b'W');
        self.rect((px0 - 2, 15, px0 - 1, 15), b'R');
        // The lid, domed, and its knob; the steam lifts it now and then.
        for (row, half_lid) in [(6, 7.4f32), (5, 6.0), (4, 4.0)] {
            for x in px0..=px1 {
                let u = (x as f32 + 0.5 - cx) / half_lid;
                if u.abs() <= 1.0 {
                    self.put(x, row - lid, metal(u));
                }
            }
        }
        self.rect((px0 + 1, 7 - lid, px1 - 1, 7 - lid), b'z');
        self.rect((cx as i32 - 1, 2 - lid, cx as i32 + 1, 3 - lid), b'z');
        self.rect((cx as i32 - 1, 2 - lid, cx as i32, 2 - lid), b'S');

        // The tamalero: handlebar, torso in a blue jacket and white apron.
        self.rect((GRIP.0 as i32 - 1, 21, GRIP.0 as i32 - 1, 18), b'z');
        self.rect((GRIP.0 as i32 - 3, 17, GRIP.0 as i32, 17), b'k');
        let top = SHOULDER.1 as i32 - 1;
        for y in top..=hip.1 as i32 {
            let f = (HIP.1 - y as f32) / (HIP.1 - SHOULDER.1);
            let mid = (HIP.0 + (SHOULDER.0 - HIP.0) * f).round() as i32;
            for x in mid - 3..=mid + 3 {
                let ch = if y >= top + 4 && x > mid { b'W' } else { b'b' };
                self.put(x, y, ch);
            }
        }
        // The head: skin, red cap with its visor, eye, nose and mustache.
        let head = (SHOULDER.0 + 1.5, SHOULDER.1 - 6.0);
        self.rect((head.0 as i32 - 1, top - 1, head.0 as i32 + 1, top), b'K');
        self.disc(head, 4.2, b'K');
        for y in (head.1 - 5.0) as i32..=(head.1 - 0.8) as i32 {
            for x in (head.0 - 5.0) as i32..=(head.0 + 5.0) as i32 {
                if (x as f32 + 0.5 - head.0).hypot(y as f32 + 0.5 - head.1) <= 4.7 {
                    self.put(x, y, b'R');
                }
            }
        }
        let (hx, hy) = (head.0.floor() as i32, head.1.floor() as i32);
        self.rect((hx + 1, hy - 1, hx + 6, hy - 1), b'r');
        self.rect((hx - 4, hy - 1, hx - 4, hy), b'r');
        self.put(hx + 2, hy + 1, b'k');
        self.put(hx + 4, hy + 2, b'K');
        self.rect((hx + 1, hy + 3, hx + 4, hy + 3), b'k');
        // His arm, from the shoulder to the grip, the elbow bent down.
        let elbow = joint(SHOULDER, GRIP, 5.0, 5.0, 1.0);
        self.thick(SHOULDER, elbow, b'B');
        self.thick(elbow, GRIP, b'B');
        self.rect(
            (
                GRIP.0 as i32 - 1,
                GRIP.1 as i32 - 1,
                GRIP.0 as i32,
                GRIP.1 as i32,
            ),
            b'K',
        );

        // The near crank and leg, in front of the frame.
        self.line(BRACKET, near, b's');
        self.leg(hip, near, b'J');
        self.disc(BRACKET, 1.6, b'z');

        // The wheels turn with the road.
        self.wheel(REAR, spin);
        self.wheel(FRONT, spin);
    }
}

impl Egg for Tamalero {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        for p in self.puffs.iter_mut().filter(|p| p.age > 0.0) {
            p.age += dt;
            p.x += (p.vx + (p.age * 3.0 + p.phase).sin() * 1.4) * dt;
            p.y -= p.vy * dt;
            if p.age > PUFF_LIFE {
                p.age = 0.0;
            }
        }
        self.puff_in -= dt;
        while self.puff_in <= 0.0 {
            self.puff_in += PUFF_EVERY * (0.6 + 0.8 * self.rng.f32());
            // From under the lid, mostly at its middle, now and then at its sides.
            let lx = [35.0, 35.0, 30.0, 40.0][self.rng.usize(..4)] + self.rng.f32() * 2.0 - 1.0;
            let x = self.col(lx as i32) as f32 + 0.5;
            if x < -4.0 || x >= self.w as f32 + 4.0 {
                continue;
            }
            self.puffs[self.next] = Puff {
                x,
                y: 3.0,
                vx: (self.rng.f32() - 0.5) * 2.0,
                vy: 5.0 + self.rng.f32() * 4.0,
                age: 1e-3,
                phase: self.rng.f32() * TAU,
            };
            self.next = (self.next + 1) % PUFFS;
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::candelaria(theme);
        let (w, h) = (c.w() as i32, c.h() as i32);
        let top = self.top(h);
        let (x, y) = (self.x(), top);
        // Steam first, so the tricycle stands in front of what it let out.
        let steam = pal.of(b'W');
        for p in self.puffs.iter().filter(|p| p.age > 0.0) {
            let k = p.age / PUFF_LIFE;
            let r = 0.9 + 2.4 * k.sqrt();
            let col = fade(theme, steam, 0.3 + 0.65 * k * k);
            let (cx, cy) = (p.x, p.y + top as f32);
            for py in (cy - r).floor() as i32..=(cy + r).ceil() as i32 {
                for px in (cx - r).floor() as i32..=(cx + r).ceil() as i32 {
                    if (px as f32 + 0.5 - cx).hypot(py as f32 + 0.5 - cy) <= r {
                        pixel(c, px, py, col);
                    }
                }
            }
        }
        if x + WIDTH >= 0 && x < w {
            let mut rig = Rig {
                c,
                pal: &pal,
                x,
                y,
                left: self.left,
            };
            let road = self.road();
            rig.vehicle(road / CRANK_TURN * TAU, road / WHEEL, self.lid());
            // TAMALES, painted on the box (never backwards, on either side).
            let at = if self.left {
                self.col(SIGN.0 + 26)
            } else {
                self.col(SIGN.0)
            };
            text(c, (at, y + SIGN.1), "TAMALES", 1, pal.of(b'R'));
        }
        // The call scrolls out of the horn, ahead of the tricycle.
        self.speak(c, theme, &pal);
    }

    fn done(&self) -> bool {
        let gone = if self.left {
            self.x() + WIDTH < 0
        } else {
            self.x() > self.w
        };
        gone && self.puffs.iter().all(|p| p.age == 0.0) || self.t > 29.5
    }
}

impl Tamalero {
    /// The letters of the call that are out of the horn, and the arcs of sound.
    fn speak(&self, c: &mut Canvas, theme: &Theme, pal: &Palette) {
        let (mouth, row) = (self.col(MOUTH.0), (self.top(c.h() as i32) + MOUTH.1) / 2);
        let dir = if self.left { -1 } else { 1 };
        let n = CALL.chars().count();
        let mut word = 0;
        let mut calling = false;
        for (j, ch) in CALL.chars().enumerate() {
            if ch == ' ' {
                word += 1;
                continue;
            }
            // Riding left, the call leaves in order; riding right, its end
            // leaves first, so it still reads from left to right.
            let order = if self.left { j } else { n - 1 - j } as i32;
            let color = pal.of(if word % 2 == 0 { b'M' } else { b'G' });
            for call in 0..4 {
                // Columns out of the horn, a whole number for every letter.
                let out = (LEAVE * (self.t - FIRST_CALL - call as f32 * EVERY)).floor() as i32;
                let off = out - order;
                let age = off as f32 / LEAVE;
                if !(0.0..=LIFE).contains(&age) {
                    continue;
                }
                calling |= age < 0.4;
                // The older it is, the higher it floats.
                let rise = (age * 0.55) as i32;
                let fadeout = ((age - (LIFE - 1.5)) / 1.5).clamp(0.0, 1.0);
                let color = fade(theme, color, fadeout * 0.85);
                c.set_attr(mouth + dir * (2 + off), row - rise, ch, color, BOLD);
            }
        }
        // Sound leaves the horn in arcs while it is calling.
        if calling {
            let arc = if self.left { '(' } else { ')' };
            for k in 0..2 {
                if (self.t * 9.0) as i32 % 3 != k {
                    c.set_attr(mouth + dir * (1 + k), row, arc, pal.of(b'G'), BOLD);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    /// The call is on the screen: every letter of it, in reading order.
    #[test]
    fn calls_out_its_tamales() {
        let theme = Theme::classic();
        for (seed, w) in [(1, 120), (2, 120), (3, 90), (4, 200)] {
            let mut e = start(w, 40, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(w, 40, Color::Reset);
            let mut best = 0;
            for _ in 0..(22 * 10) {
                e.update(0.1);
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                let letters = text
                    .chars()
                    .filter(|ch| CALL.contains(*ch) && *ch != ' ')
                    .count();
                best = best.max(letters);
            }
            assert!(
                best >= 30,
                "seed {seed}: {best} letters of the call at once"
            );
        }
    }

    /// He crosses the screen standing on its bottom row, from either side,
    /// and the box says TAMALES.
    #[test]
    fn rides_across_on_the_ground() {
        let theme = Theme::classic();
        let mut seen = [false; 2];
        for seed in 0..8 {
            let mut e = start(100, 30, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(100, 30, Color::Reset);
            let mut centers = Vec::new();
            for _ in 0..40 {
                e.update(0.5);
                c.clear();
                e.draw(&mut c, &theme);
                let bottom: Vec<usize> = (0..100)
                    .filter(|&x| c.get(x, 29).unwrap().ch != ' ')
                    .map(|x| x as usize)
                    .collect();
                if bottom.len() > 8 {
                    centers.push(bottom.iter().sum::<usize>() / bottom.len());
                }
            }
            assert!(centers.len() > 10, "{centers:?}");
            let moved = *centers.last().unwrap() as i32 - centers[0] as i32;
            assert!(moved.abs() > 30, "{centers:?}");
            seen[usize::from(moved < 0)] = true;
        }
        assert!(seen[0] && seen[1], "he came from one side only");
    }

    #[test]
    fn the_legs_reach_the_pedals() {
        // The knee is between the hip and the pedal, whichever way the crank is.
        for k in 0..16 {
            let a = k as f32 * TAU / 16.0;
            let pedal = (BRACKET.0 + CRANK * a.cos(), BRACKET.1 + CRANK * a.sin());
            let ankle = (pedal.0 - 0.5, pedal.1 - 1.5);
            let knee = joint(HIP, ankle, 10.0, 10.0, -1.0);
            let (a1, a2) = (
                (knee.0 - HIP.0).hypot(knee.1 - HIP.1),
                (knee.0 - ankle.0).hypot(knee.1 - ankle.1),
            );
            assert!(
                (a1 - 10.0).abs() < 0.1 && (a2 - 10.0).abs() < 0.1,
                "{a1} {a2}"
            );
            assert!(knee.0 > HIP.0.min(ankle.0), "the knee bends forward");
        }
        // Out of reach it stretches without breaking.
        let far = joint((0.0, 0.0), (100.0, 0.0), 3.0, 3.0, 1.0);
        assert!(far.0.is_finite() && far.1.is_finite());
    }
}
