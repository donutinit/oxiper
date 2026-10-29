//! A pastorela plays out on the bottom of the screen during the Navidad:
//! the shepherds are tempted by the devil on their way to Belén, and San
//! Miguel arcángel drives him off.
//!
//! The pastorela is the Mexican Christmas play, centuries old, in which a
//! band of shepherds travels to the nacimiento and Luzbel, the devil,
//! tries every trick to keep them from arriving, until the arcángel Miguel
//! ("¿Quién como Dios?") sends him packing. Here Bato and Gila walk in, a
//! sarape and a basket of tejocotes between them, chatting; Luzbel springs
//! out of a burst of flame, red, horned and tailed, and tempts them to a
//! nap; poor Bato nods off with his zzz while Gila frets; San Miguel drops
//! out of the sky in a flash, wings beating, sword raised, and strikes; the
//! devil flees howling, trailing smoke, his tail between his legs; and the
//! shepherds wake, give thanks and walk on to Belén while the arcángel
//! rises back into the sky. Original oxiper pixel art and dialogue, typed
//! out above them as the play goes. They come from either side and are all
//! gone within 30 s.

use super::navidad::{Palette, fade, split_in_two, type_out};
use super::{Egg, Info, pixel, sprite};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "pastorela",
    start,
    fiesta: Kind::Navidad,
    when: super::any_day,
};

type Frames = [&'static [&'static str]; 2];

/// Bato, the shepherd, facing right: straw hat, moustache, a striped poncho,
/// a staff in his hand (18 rows; his legs go below).
const BATO: &[&str] = &[
    ".............bB",
    ".....CCCC....bB",
    "....CCCCCC...bB",
    "...CCRRRRCC..bB",
    ".CCCCCCCCCCC.bB",
    "..dddddddddd.bB",
    "...eeeeeeee..bB",
    "...eeeeekeE..bB",
    "...eeeebbbbe.bB",
    "....eeeeeee..bB",
    "...xxxRxxxx..bB",
    "..xxxxRxxxxxeeB",
    "..xxxxRxxxxxxeB",
    "..xxxxRxxxxx.bB",
    "..xxxxRxxxxx.bB",
    "..xxxxRxxxxx.bB",
    "..RRRRRRRRRR.bB",
    "..Y.Y.Y.Y.Y..bB",
];
/// His legs, huaraches and the foot of his staff.
const BATO_LEGS: Frames = [
    &[
        "....WW..WW...bB",
        "....WW..WW...bB",
        "....WW..WW...bB",
        "...bbb.bbb...bB",
    ],
    &[
        ".....WWWW....bB",
        "....WW..WW...bB",
        "...WW....WW..bB",
        "..bbb....bbb.bB",
    ],
];
const BATO_W: i32 = 15;

/// Gila, the shepherdess, facing right: braids, a rebozo, a long skirt and a
/// basket of tejocotes on her arm (18 rows; her feet go below).
const GILA: &[&str] = &[
    "....kkkkk......",
    "...kkkkkkkk....",
    "..kkkeeeeee....",
    "..kkkeeekeE....",
    "..kkkeeeeee....",
    "...kkeeeeee....",
    "..kk..eeee.....",
    "..kk.xxxxxxx...",
    "..kkxxxxxxxxx..",
    "..PkxxxxxxxxxTT",
    "..kkxxxxxxxxeCC",
    "...xxxxxxxxxxCC",
    "..WWxxxxxxxxCCC",
    "..WWzzzzzzzzzzz",
    ".PPPPPPPPPPPPP.",
    ".PPzPPPPzPPPPP.",
    "PPPPPPzPPPPzPP.",
    "PPzPPPPPPPPPPP.",
];
const GILA_FEET: Frames = [
    &["...bb..bb......", "..bbb..bbb....."],
    &["....bbbb.......", "...bbb.bbb....."],
];
const GILA_W: i32 = 15;

/// Luzbel, the devil, facing right: horns, pointed ears, a wicked grin, a
/// black cape and red tights, a trident in his hand (his legs go below).
const LUZBEL: &[&str] = &[
    "..R.......R.....",
    "..RR......RR....",
    "...RRRRRRRRR....",
    "..RRRRRRRRRRR...",
    "..RRRYkRRYkRR...",
    "...RRRRRRRRRRR..",
    "...RRRkkkkkRRR..",
    "....RWkWkWkWR...",
    ".....RRDDDRR....",
    "....nnnRRRnn....",
    "...nnnRRRRRRn...",
    "..nnnRRRRRRRRRR.",
    "..nnnRRRRRRRRRRR",
    "..nnnnRRRRRRRRRR",
    "..nnnnnRRRRRRRR.",
    "...nnnnkkkkkRR..",
    "...nnnnRRRRRR...",
    "....nnnRRRRRR...",
    ".....nnRRRRRR...",
];
const LUZBEL_LEGS: Frames = [
    &[
        ".....RR..RR.....",
        ".....RR..RR.....",
        ".....RR..RR.....",
        "....kkk..kkk....",
    ],
    &[
        "......RRRR......",
        ".....RR..RR.....",
        "....RR....RR....",
        "...kkk....kkk...",
    ],
];
const LUZBEL_W: i32 = 16;
/// Where the trident's handle is, in his hand.
const TRIDENT: (i32, i32) = (14, 12);

/// San Miguel arcángel, facing right: golden hair under a halo, a white tunic
/// under a golden cuirass, a blue mantle, sandals; the sword is his own.
const MIGUEL: &[&str] = &[
    "....yyyyyy......",
    "...yd.....dy....",
    "....YYYYYY......",
    "...YYYYYYYYY....",
    "...YYeeeeeY.....",
    "...YYeeekeE.....",
    "...YYeeeeee.....",
    "....YYeeEe......",
    "....AAWWWWA.....",
    "...AAAYYYYAA....",
    "..AAAYyYYyYAAee.",
    "..AAAYYYYYYAAeee",
    "..AAAYdYYdYAAee.",
    "..AAAYYYYYYAA...",
    "..AAAWWddWWAA...",
    "..AAWWWWWWWWA...",
    ".AAAWWWWWWWWAA..",
    ".AAAWWwWWwWWAA..",
    ".AAAWWWWWWWWAA..",
    ".AAIWWwWWwWWAA..",
    ".AAIWWWWWWWWAA..",
    "AAAIWWWWWWWWAAA.",
];
const MIGUEL_LEGS: Frames = [
    &["....WW....WW....", "....WW....WW....", "...dYY....dYY..."],
    &[".....WW..WW.....", "....WW....WW....", "...dYY....dYY..."],
];
const MIGUEL_W: i32 = 16;
/// Where his hand is, to hold the sword.
const SWORD_AT: (i32, i32) = (14, 11);

/// Who speaks, for the color of the line.
#[derive(Clone, Copy)]
enum Who {
    Bato,
    Gila,
    Luzbel,
    Miguel,
}

/// The play, line by line: when each is said, and by whom.
const SCRIPT: [(f32, Who, &str); 7] = [
    (
        0.5,
        Who::Bato,
        "Bato: ¡Ándale, Gila, que nos vamos a Belén!",
    ),
    (4.2, Who::Gila, "Gila: ¡Voy, Bato! ¡Qué frío hace!"),
    (
        7.5,
        Who::Luzbel,
        "Luzbel: ¡Alto, pastores! ¡Duérmanse, que es tarde!",
    ),
    (10.4, Who::Gila, "Gila: ¡Bato, no te duermas!"),
    (
        13.3,
        Who::Miguel,
        "San Miguel: ¡Atrás, Luzbel! ¿Quién como Dios?",
    ),
    (16.0, Who::Luzbel, "Luzbel: ¡Ay, ay, ay, me quemo!"),
    (18.9, Who::Gila, "Gila: ¡Gracias, arcángel! ¡A Belén!"),
];
/// Letters typed per second.
const TYPING: f32 = 18.0;

/// The moments of the play, in seconds: the shepherds are in by `ENTER`,
/// Luzbel springs out at `POP` and has landed by `LAND`, Bato nods off at
/// `SLEEP`, San Miguel falls from the sky at `ARRIVE`, strikes at `STRIKE`,
/// the devil flees from `FLEE` for `RUN` seconds, Bato wakes at `WAKE`, and
/// the shepherds walk on (and the arcángel rises) from `LEAVE`, for `EXIT`.
const ENTER: f32 = 4.0;
const POP: f32 = 6.8;
const LAND: f32 = 7.6;
const SLEEP: f32 = 10.4;
const ARRIVE: f32 = 13.0;
const DESCENT: f32 = 1.3;
const STRIKE: f32 = 15.3;
const FLEE: f32 = 16.0;
const RUN: f32 = 2.6;
const WAKE: f32 = 18.9;
const LEAVE: f32 = 21.4;
const EXIT: f32 = 4.0;
const END: f32 = 26.5;

/// Columns walked per step.
const STEP: f32 = 2.6;
/// How far Bato walks ahead of Gila, and Luzbel stands ahead of Bato, and
/// San Miguel lands between them, facing the devil.
const BATO_AT: f32 = 20.0;
const LUZBEL_AT: f32 = 79.0;
const MIGUEL_AT: f32 = 39.0;
/// The columns from Gila's left edge to Luzbel's right one.
const SPAN: f32 = LUZBEL_AT + LUZBEL_W as f32;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let w = w as f32;
    // The whole play spans `SPAN` columns; on a narrower screen the actors
    // stand closer together (down to a point) instead of out of sight.
    let room = w - 6.0;
    let squeeze = (room / SPAN).clamp(0.45, 1.0);
    let stop = if room >= SPAN {
        (w * 0.36).clamp(3.0, w - SPAN - 3.0)
    } else {
        3.0
    };
    Box::new(Pastorela {
        t: 0.0,
        left: rng.bool(),
        w,
        h: h as f32 * 2.0,
        stop,
        squeeze,
        seed: rng.u32(..),
    })
}

#[derive(Clone, Copy)]
struct Pastorela {
    t: f32,
    /// Walking right to left (mirrored)?
    left: bool,
    w: f32,
    /// The screen's height in pixels: the ground under their feet.
    h: f32,
    /// Where Gila stands when they stop, in columns from where they come in.
    stop: f32,
    /// How much the actors' distances are shortened for a narrow screen.
    squeeze: f32,
    seed: u32,
}

/// Smooth from 0 to 1.
fn smooth(k: f32) -> f32 {
    let k = k.clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

impl Pastorela {
    /// Where Bato, Luzbel and San Miguel stand, from Gila's left edge.
    fn bato_at(&self) -> f32 {
        BATO_AT * self.squeeze
    }

    fn luzbel_at(&self) -> f32 {
        LUZBEL_AT * self.squeeze
    }

    fn miguel_at(&self) -> f32 {
        MIGUEL_AT * self.squeeze
    }

    /// Where Gila's left column is, in the play's own columns (the shepherds
    /// walk toward increasing ones).
    fn gila_x(&self) -> f32 {
        let from = -(self.bato_at() + BATO_W as f32 + 30.0);
        if self.t < LEAVE {
            from + (self.stop - from) * (self.t / ENTER).min(1.0)
        } else {
            let end = self.w + 8.0;
            self.stop + (end - self.stop) * ((self.t - LEAVE) / EXIT).min(1.0)
        }
    }

    /// Is the walker at `x` mid-step?
    fn step(x: f32) -> bool {
        (x / STEP).floor() as i32 & 1 == 1
    }

    /// Are the shepherds walking?
    fn walking(&self) -> bool {
        self.t < ENTER || self.t >= LEAVE
    }

    /// Luzbel's left column, and how high he is above the ground (his
    /// spring out of the fire).
    fn luzbel(&self) -> (f32, f32) {
        let x = self.stop + self.luzbel_at();
        if self.t < FLEE {
            // The blow throws him back a little.
            let knock = if self.t > STRIKE {
                7.0 * (1.0 - smooth((self.t - STRIKE) / 0.5))
            } else {
                0.0
            };
            let up = (self.t - POP) / (LAND - POP);
            let jump = if (0.0..1.0).contains(&up) {
                26.0 * (up * PI).sin()
            } else {
                0.0
            };
            (x + knock, jump)
        } else {
            let k = ((self.t - FLEE) / RUN).clamp(0.0, 1.0);
            let far = self.w + 50.0;
            (x + 7.0 * 0.0 + (far - x) * k.powf(1.5), 0.0)
        }
    }

    /// San Miguel's column, and how high above the ground his feet are.
    fn miguel(&self) -> (f32, f32) {
        let x = self.stop + self.miguel_at();
        let sky = self.h + 40.0;
        let up = if self.t < ARRIVE + DESCENT {
            sky * (1.0 - smooth((self.t - ARRIVE) / DESCENT)).powi(2)
        } else if self.t > LEAVE {
            sky * smooth((self.t - LEAVE) / 3.0)
        } else {
            0.0
        };
        (x, up)
    }

    /// His wings beat while he flies, and slowly stir on the ground.
    fn flap(&self) -> f32 {
        let flying = self.t < ARRIVE + DESCENT || self.t > LEAVE;
        (self.t * if flying { 9.0 } else { 2.5 }).sin()
    }

    /// The column on screen of a sprite `wd` wide at `x` in the play.
    fn at(&self, x: f32, wd: i32) -> i32 {
        if self.left {
            (self.w - x).round() as i32 - wd
        } else {
            x.round() as i32
        }
    }

    /// +1 where the shepherds walk to, in screen columns.
    fn fwd(&self) -> i32 {
        if self.left { -1 } else { 1 }
    }

    /// A number in 0..1 for `i`, the same every time.
    fn hash(&self, i: u32) -> f32 {
        let mut v = i.wrapping_mul(0x9e37_79b9) ^ self.seed;
        v ^= v >> 15;
        v = v.wrapping_mul(0x2c1b_3c6d);
        v ^= v >> 12;
        (v & 0xffff) as f32 / 65536.0
    }

    /// A burst of little bits flying out of (`x`, `y`) pixels: `age`
    /// seconds in, each of `letters` colors, for `life` seconds. `fall` is
    /// how much they sink (negative: they rise, like flames).
    #[allow(clippy::too_many_arguments)]
    fn burst(
        &self,
        c: &mut Canvas,
        (theme, pal): (&Theme, &Palette),
        (x, y): (f32, f32),
        age: f32,
        life: f32,
        n: u32,
        (speed, fall): (f32, f32),
        letters: &[u8],
        salt: u32,
    ) {
        if !(0.0..life).contains(&age) {
            return;
        }
        for i in 0..n {
            let a = self.hash(salt + i) * TAU;
            let sp = (0.3 + 0.7 * self.hash(salt + i + 99)) * speed;
            let (px, py) = (
                x + a.cos() * sp * age,
                y + a.sin() * sp * 0.7 * age + fall * age * age,
            );
            let letter = letters[i as usize % letters.len()];
            let col = fade(theme, pal.of(letter), (age / life).powi(2));
            pixel(c, px.round() as i32, py.round() as i32, col);
        }
    }

    fn draw_wings(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, (ox, oy): (i32, i32)) {
        // A fan of nine feathers from the shoulder, up and back, sweeping
        // with the beat.
        let (sign, flap) = (self.fwd() as f32, self.flap());
        for k in 0..9 {
            let spread = (k as f32 - 4.0) * 0.26;
            let ang = 2.5 + 0.55 * flap + spread;
            let len = 11.0 + 9.0 * (1.0 - (k as f32 - 4.0).abs() / 5.0);
            let mut r = 2.0;
            while r <= len {
                let (px, py) = (ox as f32 + sign * ang.cos() * r, oy as f32 - ang.sin() * r);
                let letter = if r > len - 2.5 {
                    b'Y'
                } else if k % 2 == 0 {
                    b'W'
                } else {
                    b'w'
                };
                let col = pal.of(letter);
                pixel(c, px.round() as i32, py.round() as i32, col);
                pixel(
                    c,
                    px.round() as i32,
                    py.round() as i32 + 1,
                    fade(theme, col, 0.15),
                );
                r += 0.5;
            }
        }
    }
}

impl Egg for Pastorela {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::navidad(theme);
        let (ground, fwd) = (self.h as i32, self.fwd());
        let tp = (theme, &pal);
        let t = self.t;
        let (gx, bx) = (self.gila_x(), self.gila_x() + self.bato_at());

        // Gila, a little afraid while the devil is about.
        let scared = t > LAND && t < FLEE + 0.6;
        let step = self.walking() && Self::step(gx);
        let (gc, gy) = (
            self.at(gx, GILA_W),
            ground - GILA.len() as i32 - 2 - i32::from(self.walking() && !step),
        );
        let lean = |r: i32| {
            if scared && r < 12 {
                -fwd * (1 - r / 6).max(0)
            } else {
                0
            }
        };
        let skirt = |ch: u8| match ch {
            b'x' => Some(pal.of(b'Q')),
            b'z' => Some(pal.of(b'Q').scale(0.7)),
            b'k' => Some(pal.of(b'k')),
            _ => pal.get(ch),
        };
        sprite(
            c,
            (gc, gy + GILA.len() as i32),
            GILA_FEET[usize::from(step)],
            self.left,
            |_| 0,
            skirt,
        );
        sprite(c, (gc, gy), GILA, self.left, lean, skirt);
        if scared && (t * 4.0) as i32 % 2 == 0 {
            c.text_attr(gc + GILA_W / 2, gy / 2 - 1, "!", pal.of(b'Y'), BOLD);
        }

        // Bato, who nods off while the devil talks and wakes at the end.
        let asleep = t > SLEEP + 0.4 && t < WAKE;
        let step = self.walking() && Self::step(bx);
        let (bc, by) = (
            self.at(bx, BATO_W),
            ground - BATO.len() as i32 - 4 - i32::from(self.walking() && !step) + i32::from(asleep),
        );
        let nod = |r: i32| {
            if asleep && r < 12 {
                fwd * ((12 - r) / 4 + 1)
            } else {
                0
            }
        };
        let poncho = |ch: u8| match ch {
            b'x' => Some(pal.of(b'O')),
            b'R' => Some(pal.of(b'R')),
            _ => pal.get(ch),
        };
        sprite(
            c,
            (bc, by + BATO.len() as i32),
            BATO_LEGS[usize::from(step)],
            self.left,
            |_| 0,
            poncho,
        );
        sprite(c, (bc, by), BATO, self.left, nod, poncho);
        if asleep {
            // zzz, floating up from his hat.
            for (i, z) in ['z', 'Z', 'Z'].into_iter().enumerate() {
                let age = ((t - SLEEP) * 0.8 + i as f32 / 3.0).fract();
                let (zx, zy) = (
                    bc + BATO_W / 2 + fwd * (3 + (age * 7.0) as i32),
                    by / 2 - 1 - (age * 5.0) as i32,
                );
                c.set_attr(zx, zy, z, fade(theme, pal.of(b'w'), age * 0.8), BOLD);
            }
        } else if (WAKE..WAKE + 1.0).contains(&t) {
            c.text_attr(bc + BATO_W / 2, by / 2 - 1, "!", pal.of(b'Y'), BOLD);
        }

        // Luzbel, from his burst of flame until he is gone.
        if t >= POP {
            let (lx, up) = self.luzbel();
            let fleeing = t >= FLEE;
            let facing_left = !self.left != fleeing;
            let lc = if self.left {
                self.at(lx, LUZBEL_W)
            } else {
                lx.round() as i32
            };
            let (lw, lh) = (LUZBEL_W, LUZBEL.len() as i32 + 4);
            let hit = (STRIKE..STRIKE + 0.12).contains(&t);
            let cower = t > ARRIVE + 0.8 && !fleeing;
            let running = fleeing || (t > POP && t < LAND);
            let step = running && Self::step(lx * 1.4);
            let ly = ground - lh - up.round() as i32 - i32::from(running && !step);
            // He trembles with glee while he tempts, and with fright after.
            let shake = if cower {
                (t * 22.0).sin().round() as i32
            } else if t > LAND && t < SLEEP + 2.0 {
                ((t * 8.0).sin() * 1.1).round() as i32
            } else {
                0
            };
            let facing = if facing_left { -1 } else { 1 };
            let flip = facing < 0;
            let ink = |ch: u8| {
                if hit {
                    pal.get(ch).map(|_| pal.of(b'W'))
                } else {
                    pal.get(ch)
                }
            };
            let afraid = |r: i32| {
                if cower && r < 12 {
                    -facing * (1 + (12 - r) / 5) + shake
                } else {
                    shake
                }
            };
            sprite(
                c,
                (lc, ly + LUZBEL.len() as i32),
                LUZBEL_LEGS[usize::from(step)],
                flip,
                |_| 0,
                ink,
            );
            // His tail, wagging, with a spade at the tip.
            let rump = (lc + if flip { lw - 3 } else { 2 }, ly + 16);
            for i in 0..36 {
                let (s, f) = (i as f32 * 0.5, i as f32 / 36.0);
                let wag = (t * 7.0 - f * 3.0).sin() * 3.0 * f;
                let tx = rump.0 as f32 - facing as f32 * s * 0.9;
                let ty = rump.1 as f32 - (f * 6.0).sin() * 5.0 + wag;
                let spade = s > 15.0 && s < 17.5 || s > 16.0 && i % 2 == 0;
                let letter = if spade { b'D' } else { b'R' };
                pixel(c, tx.round() as i32, ty.round() as i32, pal.of(letter));
            }
            sprite(c, (lc, ly), LUZBEL, flip, afraid, ink);
            // His trident: a silver handle with three prongs.
            let hand = (
                lc + if flip { lw - 1 - TRIDENT.0 } else { TRIDENT.0 },
                ly + TRIDENT.1,
            );
            let lean = (t * 6.0).sin().round() as i32 * i32::from(!cower);
            for k in 0..22 {
                let col = pal.of(if k > 18 { b'X' } else { b'S' });
                pixel(c, hand.0 + lean * k / 14, hand.1 + 4 - k, col);
            }
            let tip = (hand.0 + lean * 22 / 14, hand.1 - 18);
            for dx in [-2, 0, 2] {
                for dy in 0..3 {
                    pixel(c, tip.0 + dx, tip.1 - dy - i32::from(dx == 0), pal.of(b'X'));
                }
            }
        }

        // San Miguel, falling from the sky in a beam of light.
        if (ARRIVE..LEAVE + 3.0).contains(&t) {
            let (mx, up) = self.miguel();
            let mc = self.at(mx, MIGUEL_W);
            let my = ground - MIGUEL.len() as i32 - 3 - up.round() as i32;
            // The beam, while he comes down and a moment after.
            let beam = 1.0 - ((t - ARRIVE - DESCENT) / 0.6).clamp(0.0, 1.0);
            if t < ARRIVE + DESCENT + 0.6 {
                let bx = mc + MIGUEL_W / 2;
                for y in 0..my + 20 {
                    if (y + (t * 20.0) as i32) % 2 == 0 {
                        let col = fade(theme, pal.of(b'y'), 0.55 + 0.35 * (1.0 - beam));
                        pixel(c, bx - 1, y, col);
                        pixel(c, bx + 2, y, col);
                    }
                }
            }
            let flip = self.left;
            self.draw_wings(
                c,
                theme,
                &pal,
                (mc + if flip { MIGUEL_W - 1 - 5 } else { 5 }, my + 9),
            );
            let cloak = |ch: u8| pal.get(ch);
            sprite(
                c,
                (mc, my + MIGUEL.len() as i32),
                MIGUEL_LEGS[usize::from(self.flap() > 0.0)],
                flip,
                |_| 0,
                cloak,
            );
            sprite(c, (mc, my), MIGUEL, flip, |_| 0, cloak);
            // The sword: raised, then down in a blow at the devil.
            let s = t - STRIKE;
            let ang = if (0.0..0.9).contains(&s) {
                let k = if s < 0.25 {
                    smooth(s / 0.25)
                } else {
                    1.0 - smooth((s - 0.25) / 0.65)
                };
                70f32.to_radians() - 100f32.to_radians() * k
            } else {
                70f32.to_radians() + 0.05 * (t * 3.0).sin()
            };
            let hand = (
                mc + if flip {
                    MIGUEL_W - 1 - SWORD_AT.0
                } else {
                    SWORD_AT.0
                },
                my + SWORD_AT.1,
            );
            let mut r = -2.0;
            while r <= 26.0 {
                let (px, py) = (
                    hand.0 as f32 + fwd as f32 * ang.cos() * r,
                    hand.1 as f32 - ang.sin() * r,
                );
                let letter = if r < 0.0 {
                    b'Y'
                } else if r < 3.0 {
                    b'd'
                } else {
                    b'S'
                };
                pixel(c, px.round() as i32, py.round() as i32, pal.of(letter));
                if r > 3.0 && (r * 2.0) as i32 % 3 == 0 {
                    pixel(c, px.round() as i32, py.round() as i32 - 1, pal.of(b'X'));
                }
                r += 0.5;
            }
            // The crossguard.
            let (gx0, gy0) = (
                hand.0 as f32 + fwd as f32 * ang.cos() * 3.0,
                hand.1 as f32 - ang.sin() * 3.0,
            );
            for d in [-2.0f32, -1.0, 1.0, 2.0] {
                pixel(
                    c,
                    (gx0 - ang.sin() * d * fwd as f32).round() as i32,
                    (gy0 - ang.cos() * d).round() as i32,
                    pal.of(b'Y'),
                );
            }
        }

        // Effects: the flames Luzbel springs out of, the flash of the blow,
        // the arrival of the arcángel, and the smoke of the devil's flight.
        let lspot = {
            let x = self.stop + self.luzbel_at() + LUZBEL_W as f32 / 2.0;
            let sx = if self.left { self.w - x } else { x };
            (sx, self.h - 2.0)
        };
        self.burst(c, tp, lspot, t - POP, 1.2, 36, (46.0, -30.0), b"fFRfFXD", 7);
        self.burst(
            c,
            tp,
            lspot,
            t - POP - 0.2,
            1.4,
            18,
            (22.0, -14.0),
            b"nnwn",
            77,
        );
        let mx = {
            let x = self.stop + self.luzbel_at() + LUZBEL_W as f32 / 2.0;
            let sx = if self.left { self.w - x } else { x };
            (sx, self.h - 16.0)
        };
        self.burst(
            c,
            tp,
            mx,
            t - STRIKE - 0.1,
            0.6,
            22,
            (60.0, 20.0),
            b"XWyXY",
            31,
        );
        let landing = {
            let x = self.stop + self.miguel_at() + MIGUEL_W as f32 / 2.0;
            ((if self.left { self.w - x } else { x }), self.h - 2.0)
        };
        self.burst(
            c,
            tp,
            landing,
            t - ARRIVE - DESCENT,
            0.9,
            26,
            (40.0, 10.0),
            b"XyYW",
            53,
        );
        if (FLEE..FLEE + RUN + 1.0).contains(&t) {
            for j in 1..12 {
                let past = t - j as f32 * 0.07;
                if past < FLEE {
                    continue;
                }
                let save = Pastorela { t: past, ..*self };
                let (px, _) = save.luzbel();
                let x = if self.left {
                    self.w - px - LUZBEL_W as f32 / 2.0
                } else {
                    px + LUZBEL_W as f32 / 2.0
                };
                let col = fade(
                    theme,
                    pal.of(if j % 2 == 0 { b'n' } else { b'w' }),
                    j as f32 / 12.0,
                );
                pixel(
                    c,
                    x.round() as i32,
                    ground - 6 - j / 2 - (self.hash(j as u32) * 5.0) as i32,
                    col,
                );
            }
        }

        // The lines of the play, typed above them.
        let Some(k) = SCRIPT.iter().rposition(|&(from, ..)| from <= t) else {
            return;
        };
        let (from, who, text) = SCRIPT[k];
        let shown = ((t - from) * TYPING) as usize;
        let gone = ((t - (END - 2.0)) / 1.5).clamp(0.0, 1.0);
        if gone >= 1.0 {
            return;
        }
        let letter = match who {
            Who::Bato => b'Y',
            Who::Gila => b'p',
            Who::Luzbel => b'r',
            Who::Miguel => b'I',
        };
        let ink = fade(theme, pal.of(letter), gone);
        let one = c.w() as usize >= text.chars().count() + 4;
        let two = split_in_two(text);
        let lines: &[&str] = if one { &[text] } else { &two };
        let row = (((self.h - 50.0) / 2.0) as i32 - 1).max(0);
        let cursor = (shown < text.chars().count()).then(|| pal.of(b'F'));
        type_out(c, row, lines, shown, (ink, cursor), BOLD);
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
        let whole = |rows: &[&str], w: i32| rows.iter().all(|r| r.len() as i32 == w);
        assert!(whole(BATO, BATO_W));
        assert!(BATO_LEGS.iter().all(|l| whole(l, BATO_W) && l.len() == 4));
        assert!(whole(GILA, GILA_W));
        assert!(GILA_FEET.iter().all(|l| whole(l, GILA_W) && l.len() == 2));
        assert!(whole(LUZBEL, LUZBEL_W));
        assert!(
            LUZBEL_LEGS
                .iter()
                .all(|l| whole(l, LUZBEL_W) && l.len() == 4)
        );
        assert!(whole(MIGUEL, MIGUEL_W));
        assert!(
            MIGUEL_LEGS
                .iter()
                .all(|l| whole(l, MIGUEL_W) && l.len() == 3)
        );
        // The lines come in order, each with room to type before the next.
        for pair in SCRIPT.windows(2) {
            let letters = pair[0].2.chars().count() as f32;
            assert!(pair[0].0 + letters / TYPING < pair[1].0, "{}", pair[0].2);
        }
        const { assert!(LEAVE + EXIT < END && FLEE + RUN < WAKE && STRIKE < FLEE) };
    }
}
