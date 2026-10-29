//! A train of the Revolución rolls across the screen, on the Día de la
//! Revolución.
//!
//! Original oxiper pixel art, nothing like the everyday `sl`: a wood-burning
//! American locomotive of the Porfiriato (balloon stack, brass dome and
//! bell, a lamp and cowcatcher, a cab with the engineer in his sombrero and
//! a tricolor flag on its roof), its tender heaped with firewood, and three
//! boxcars in sepia planks. Over all three cars runs a long manta banner,
//! "¡VIVA LA REVOLUCIÓN!", in red letters. Soldiers and soldaderas ride on
//! the roofs and in the open doors, sombreros and rebozos and rifles,
//! waving, and now and then one of them shouts. The drivers turn with their
//! spokes and the rods follow, the cars ripple over the joints in the rails,
//! the stack throws smoke that swells and pales as it drifts back and a few
//! sparks, and now and then the whistle blows a jet of steam. It comes from
//! either side and is gone within 30 s.

use super::revolucion::{Palette, fade};
use super::stage::{Pen, Stage, hash, text_width, unit};
use super::{Egg, Info};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "tren",
    start,
    fiesta: Kind::Revolucion,
    when: super::any_day,
};

const BANNER: &str = "¡VIVA LA REVOLUCIÓN!";
/// The three boxcars, then the tender, then the locomotive; positions are
/// pixels from the train's rear (the train faces right).
const CAR_W: i32 = 38;
const CARS: [i32; 3] = [0, 40, 80];
const TENDER: i32 = 122;
const LOCO: i32 = 150;
const TRAIN_L: i32 = LOCO + 58;
/// Height of the car bodies above the rails, and the banner on them.
const BODY: i32 = 27;
const STRIP: (i32, i32) = (17, 27);
/// The stack's top, from the locomotive's rear and the rails.
const STACK: (i32, i32) = (46, 31);
/// People waist-up: front view, X the rebozo or scarf.
const MAN: &[&str] = &[
    "..hhhhh..",
    ".hhhhhhh.",
    "hhRRRRRhh",
    "hhhhhhhhh",
    "..kFFFk..",
    "..FkFkF..",
    "..FFRFF..",
    ".MMMMMMM.",
    "MMBMMMBMM",
    "MMMBMBMMM",
    "MMMMBMMMM",
];
const WOMAN: &[&str] = &[
    "..XXXXX..",
    ".XXXXXXX.",
    ".XXFFFXX.",
    ".XXFkFXX.",
    ".kXFFFXk.",
    ".kXXFXXk.",
    "..XXXXX..",
    ".MMMMMMM.",
    "MMBMMMBMM",
    "MMMBMBMMM",
    "MMMMBMMMM",
];
const PERSON_W: i32 = 9;
const PERSON_H: i32 = 11;
/// Who rides where: (x from the train's rear, height of their feet above the
/// rails, a woman?, rebozo color, waving?, rifle?).
const RIDERS: &[(i32, i32, bool, u8, bool, bool)] = &[
    (4, BODY + 2, false, b'R', true, false),
    (15, BODY + 2, true, b'R', true, false),
    (27, BODY + 2, false, b'G', false, true),
    (44, BODY + 2, true, b'G', false, false),
    (55, BODY + 2, false, b'R', false, true),
    (67, BODY + 2, true, b'O', true, false),
    (85, BODY + 2, false, b'R', true, false),
    (97, BODY + 2, true, b'R', false, true),
    (109, BODY + 2, false, b'G', true, false),
    // The open doors of the first and last cars.
    (10, 5, true, b'O', true, false),
    (15, 5, false, b'R', true, false),
    (90, 5, false, b'R', true, false),
    (96, 5, true, b'G', false, false),
    // On the firewood.
    (TENDER + 4, 22, false, b'R', true, true),
    (TENDER + 15, 22, true, b'R', true, false),
];
/// The doors that stand open (car, x in the car).
const OPEN_DOORS: [(usize, i32); 2] = [(0, 5), (2, 6)];
const SHOUTS: [&str; 3] = ["¡Tierra y Libertad!", "¡Viva!", "¡Viva la Revolución!"];

const CROSSING: f32 = 21.0;
/// Puffs of smoke per second, and how long one lasts.
const PUFFS: f32 = 5.0;
const SMOKE_LIFE: f32 = 3.2;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let st = Stage::new(w, h);
    Box::new(Tren {
        t: 0.0,
        speed: ((st.w + TRAIN_L + 24) as f32 / CROSSING).max(14.0),
        left: rng.bool(),
        w: st.w,
        seed: rng.u32(..),
        whistle: 5.0 + 6.0 * rng.f32(),
    })
}

struct Tren {
    t: f32,
    /// Pixels per second.
    speed: f32,
    /// Rolling to the left (the picture mirrored)?
    left: bool,
    w: i32,
    seed: u32,
    /// When the whistle blows.
    whistle: f32,
}

impl Tren {
    /// Where the train's rear is, in pixels from the left edge (unmirrored).
    fn rear(&self) -> f32 {
        self.t * self.speed - TRAIN_L as f32
    }

    /// Pixels the wheels have turned through (radians for a wheel of radius 1).
    fn rolled(&self) -> f32 {
        self.t * self.speed
    }
}

impl Egg for Tren {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::revolucion(theme);
        let st = Stage::new(c.w(), c.h());
        let mut pen = Pen {
            c,
            st,
            mirror: self.left,
        };
        let rail = st.h - 3;
        let rear = self.rear().round() as i32;
        if rear > self.w || rear + TRAIN_L < 0 {
            // Off the screen, but the smoke may still hang about.
            self.smoke(&mut pen, &pal, rail);
            return;
        }
        self.track(&mut pen, &pal, rear, rail);
        for (i, &x) in CARS.iter().enumerate() {
            self.boxcar(&mut pen, &pal, (rear + x, rail), i);
        }
        self.banner(&mut pen, &pal, rear, rail);
        self.tender(&mut pen, &pal, (rear + TENDER, rail));
        self.locomotive(&mut pen, &pal, (rear + LOCO, rail));
        self.riders(&mut pen, &pal, rear, rail);
        self.smoke(&mut pen, &pal, rail);
        self.shouts(&mut pen, &pal, theme, (rear, rail));
    }

    fn done(&self) -> bool {
        self.t > (self.w + 6 + TRAIN_L) as f32 / self.speed + SMOKE_LIFE || self.t > 29.5
    }
}

/// A rectangle from its bottom-left corner, `y` rows above the rails.
fn rup(
    pen: &mut Pen,
    rail: i32,
    (x, y): (i32, i32),
    (w, h): (i32, i32),
    color: crate::color::Color,
) {
    pen.rect(x, rail - (y + h - 1), w, h, color);
}

impl Tren {
    /// The rails and sleepers under and around the train.
    fn track(&self, pen: &mut Pen, pal: &Palette, rear: i32, rail: i32) {
        let (lo, hi) = (rear - 8, rear + TRAIN_L + 8);
        pen.rect(lo, rail + 1, hi - lo, 1, pal.of(b'S'));
        let first = lo.div_euclid(6) * 6;
        for x in (first..hi).step_by(6) {
            pen.rect(x, rail + 2, 3, 1, pal.of(b'b'));
        }
    }

    /// How much a car (0 is the first, 3 the tender, 4 the locomotive) is
    /// lifted at the joints of the rails, rippling along the train.
    fn jolt(&self, i: usize) -> i32 {
        i32::from((self.rolled() / 5.0 - i as f32 * 0.9).sin() > 0.75)
    }

    /// A wheel of `r` pixels with `spokes` spokes, turned `turn` radians.
    fn wheel(&self, pen: &mut Pen, pal: &Palette, (cx, cy): (i32, i32), r: f32, spokes: u32) {
        let (rail_y, ang) = (pen.st.h - 3, self.rolled() / r);
        let c = (cx as f32, (rail_y - cy) as f32);
        pen.blob(c, (r, r), pal.of(b'K'));
        pen.blob(c, (r - 1.0, r - 1.0), pal.of(b'k'));
        for s in 0..spokes {
            let a = ang + s as f32 * TAU / (spokes * 2) as f32;
            let (dx, dy) = (
                (a.cos() * (r - 0.5)).round() as i32,
                (a.sin() * (r - 0.5)).round() as i32,
            );
            pen.line(
                (cx - dx, rail_y - cy + dy),
                (cx + dx, rail_y - cy - dy),
                1,
                pal.of(b'S'),
            );
        }
        pen.blob(c, (1.0, 1.0), pal.of(b'o'));
    }

    /// A boxcar: sepia planks, a roof, a door, the banner goes over it.
    fn boxcar(&self, pen: &mut Pen, pal: &Palette, (x, rail): (i32, i32), i: usize) {
        let lift = self.jolt(i);
        let (wood, dark, light) = (pal.of(b'B'), pal.of(b'b'), pal.of(b'u'));
        rup(pen, rail, (x, 3), (CAR_W, 2), pal.of(b'k'));
        rup(pen, rail, (x, 5 + lift), (CAR_W, BODY - 4), wood);
        for px in (2..CAR_W).step_by(4) {
            rup(pen, rail, (x + px, 5 + lift), (1, BODY - 4), dark);
        }
        rup(
            pen,
            rail,
            (x - 1, BODY + 1 + lift),
            (CAR_W + 2, 2),
            pal.of(b'K'),
        );
        rup(pen, rail, (x + 1, BODY + 3 + lift), (CAR_W - 2, 1), dark);
        // The door: open on two cars, showing the dark inside, else shut.
        let door = OPEN_DOORS.iter().find(|d| d.0 == i);
        let dx = x + door.map_or(13, |d| d.1 + 4);
        if door.is_some() {
            rup(pen, rail, (dx, 5 + lift), (14, 11), pal.of(b'k'));
        } else {
            rup(pen, rail, (dx, 5 + lift), (12, 11), light);
            rup(pen, rail, (dx, 5 + lift), (12, 1), dark);
            rup(pen, rail, (dx, 15 + lift), (12, 1), dark);
            pen.line((dx, rail - 5 - lift), (dx + 11, rail - 15 - lift), 1, dark);
            pen.line((dx, rail - 15 - lift), (dx + 11, rail - 5 - lift), 1, dark);
        }
        for wx in [x + 7, x + CAR_W - 8] {
            self.wheel(pen, pal, (wx, 3), 3.0, 2);
        }
    }

    /// The long manta banner over the three cars, with its red letters.
    fn banner(&self, pen: &mut Pen, pal: &Palette, rear: i32, rail: i32) {
        let width = CARS[2] + CAR_W;
        let lift = self.jolt(1);
        let (y0, y1) = (STRIP.0 + lift, STRIP.1 + lift);
        rup(pen, rail, (rear, y0), (width, y1 - y0 + 1), pal.of(b'M'));
        rup(pen, rail, (rear, y1), (width, 1), pal.of(b'G'));
        rup(pen, rail, (rear, y0), (width, 1), pal.of(b'R'));
        // The letters read left to right whichever way the train faces.
        let text = text_width(BANNER, 1);
        let left = pen.left_of(rear + (width - text) / 2, text);
        let mut x = left;
        let top = rail - (y1 - 3);
        for ch in BANNER.chars() {
            pen.st.glyph(pen.c, (x, top), ch, 1, pal.of(b'R'), None);
            x += super::stage::advance(ch);
        }
    }

    /// The tender: a coal-black box heaped with firewood.
    fn tender(&self, pen: &mut Pen, pal: &Palette, (x, rail): (i32, i32)) {
        let lift = self.jolt(3);
        rup(pen, rail, (x, 3), (26, 2), pal.of(b'k'));
        rup(pen, rail, (x, 5 + lift), (26, 8), pal.of(b'K'));
        rup(pen, rail, (x, 11 + lift), (26, 1), pal.of(b'b'));
        // The logs, ends toward us.
        for row in 0..3 {
            for col in 0..8 {
                let (lx, ly) = (x + 1 + col * 3 + row % 2, 13 + row * 3 + lift);
                if lx + 3 > x + 26 {
                    continue;
                }
                let d = hash(col as u32, row as u32 + self.seed % 5);
                rup(pen, rail, (lx, ly), (3, 3), pal.of(b'u'));
                rup(
                    pen,
                    rail,
                    (lx + 1, ly + 1),
                    (1, 1),
                    if d.is_multiple_of(3) {
                        pal.of(b'b')
                    } else {
                        pal.of(b'B')
                    },
                );
            }
        }
        for wx in [x + 5, x + 20] {
            self.wheel(pen, pal, (wx, 3), 3.0, 2);
        }
        // The hook to the first car.
        rup(pen, rail, (x - 2, 6), (2, 1), pal.of(b'k'));
    }

    /// The locomotive: boiler and stack, dome, bell, lamp, cab, cylinders,
    /// drivers and rods, and the cowcatcher.
    fn locomotive(&self, pen: &mut Pen, pal: &Palette, (x, rail): (i32, i32)) {
        let lift = self.jolt(4);
        let (black, steel, brass, bright) =
            (pal.of(b'K'), pal.of(b'S'), pal.of(b'A'), pal.of(b'o'));
        // Chassis, boiler, smokebox.
        rup(pen, rail, (x, 4), (54, 3), pal.of(b'k'));
        let hide = pal.of(b'c');
        rup(pen, rail, (x + 14, 8 + lift), (30, 12), hide);
        rup(pen, rail, (x + 14, 8 + lift), (30, 3), black);
        rup(pen, rail, (x + 14, 19 + lift), (30, 1), pal.of(b'h'));
        rup(pen, rail, (x + 15, 17 + lift), (28, 1), pal.of(b'C'));
        for bx in [18, 28, 38] {
            rup(pen, rail, (x + bx, 8 + lift), (2, 12), brass);
        }
        rup(pen, rail, (x + 44, 8 + lift), (6, 12), black);
        pen.blob(
            (x as f32 + 47.0, (rail - 14 - lift) as f32),
            (2.5, 2.5),
            black,
        );
        pen.px(x + 47, rail - 14 - lift, steel);
        // The balloon stack, flaring at the top, with a rim.
        let sx = x + STACK.0;
        for (i, half) in [1, 1, 1, 2, 3, 4, 4, 4].into_iter().enumerate() {
            rup(
                pen,
                rail,
                (sx - half, 20 + lift + i as i32),
                (2 * half + 1, 1),
                black,
            );
            pen.px(sx - half, rail - 20 - lift - i as i32, steel);
        }
        rup(pen, rail, (sx - 5, 28 + lift), (11, 1), brass);
        rup(pen, rail, (sx - 5, 29 + lift), (11, 1), bright);
        // The steam dome, sand dome and bell: brass.
        pen.blob(
            (x as f32 + 27.0, (rail - 20 - lift) as f32),
            (3.5, 2.5),
            brass,
        );
        pen.px(x + 26, rail - 21 - lift, bright);
        pen.blob(
            (x as f32 + 36.0, (rail - 20 - lift) as f32),
            (2.0, 1.6),
            brass,
        );
        rup(pen, rail, (x + 20, 20 + lift), (3, 2), brass);
        pen.px(x + 21, rail - 22 - lift, bright);
        // The headlamp, with its glass lit.
        rup(pen, rail, (x + 48, 20 + lift), (5, 5), brass);
        rup(pen, rail, (x + 52, 21 + lift), (1, 3), pal.of(b'Y'));
        // The cab: wood, a window with the engineer in his sombrero, a roof.
        rup(pen, rail, (x, 6 + lift), (14, 20), pal.of(b'B'));
        for px in [4, 8, 12] {
            rup(pen, rail, (x + px, 6 + lift), (1, 20), pal.of(b'b'));
        }
        rup(pen, rail, (x + 3, 15 + lift), (7, 7), pal.of(b'k'));
        rup(pen, rail, (x + 5, 15 + lift), (3, 3), pal.of(b'F'));
        rup(pen, rail, (x + 4, 19 + lift), (5, 1), pal.of(b'h'));
        rup(pen, rail, (x + 3, 18 + lift), (7, 1), pal.of(b'h'));
        rup(pen, rail, (x - 1, 26 + lift), (17, 2), black);
        // The tricolor flag on the cab roof.
        let top = 37;
        rup(pen, rail, (x + 7, 28 + lift), (1, 9), pal.of(b'u'));
        for fx in 0..10 {
            let wave =
                ((self.t * 6.0 - fx as f32 * 0.7).sin() * 1.2 * fx as f32 / 10.0).round() as i32;
            let col = if fx < 3 {
                pal.of(b'G')
            } else if fx < 7 {
                pal.of(b'M')
            } else {
                pal.of(b'R')
            };
            rup(pen, rail, (x + 8 + fx, top - 5 + lift + wave), (1, 6), col);
        }
        // Cylinders, drivers, the rods.
        rup(pen, rail, (x + 40, 4), (8, 4), black);
        let drivers = [x + 20, x + 33];
        let turn = self.rolled() / 5.0;
        for &dx in &drivers {
            self.wheel(pen, pal, (dx, 5), 5.0, 4);
        }
        let pins = drivers.map(|dx| {
            (
                dx + (turn.cos() * 3.2).round() as i32,
                rail - 5 - (turn.sin() * 3.2).round() as i32,
            )
        });
        pen.line(pins[0], pins[1], 1, steel);
        pen.line(pins[1], (x + 42, rail - 6), 1, steel);
        pen.rect(pins[0].0, pins[0].1, 1, 1, bright);
        pen.rect(pins[1].0, pins[1].1, 1, 1, bright);
        self.wheel(pen, pal, (x + 46, 3), 3.0, 2);
        // The cowcatcher: slats sloping to the front.
        for r in 0..6 {
            rup(
                pen,
                rail,
                (x + 50, 1 + r),
                (3 + (5 - r), 1),
                if r % 2 == 0 { steel } else { pal.of(b'k') },
            );
        }
        // The whistle's steam, when it blows.
        let blow = self.t - self.whistle;
        if (0.0..1.1).contains(&blow) {
            for i in 0..6 {
                let age = blow - i as f32 * 0.06;
                if age <= 0.0 {
                    continue;
                }
                let (jx, jy) = (
                    x as f32 + 27.0 - age * 3.0,
                    (rail - 22 - lift) as f32 - age * 11.0,
                );
                pen.blob((jx, jy), (1.0 + age * 1.5, 1.0 + age * 1.5), pal.of(b'X'));
            }
        }
    }

    /// The people on the roofs, in the doors and on the firewood.
    fn riders(&self, pen: &mut Pen, pal: &Palette, rear: i32, rail: i32) {
        for (n, &(x, y, woman, rebozo, wave, rifle)) in RIDERS.iter().enumerate() {
            let n = n as u32;
            let car = if x >= TENDER { 3 } else { (x / 40) as usize };
            let lift = self.jolt(car);
            let top = rail - (y + lift + PERSON_H - 1);
            let at = rear + x;
            let shawl = pal.of(rebozo);
            let art = if woman { WOMAN } else { MAN };
            pen.sprite(
                (at, top),
                art,
                false,
                |_| 0,
                |ch| if ch == b'X' { Some(shawl) } else { pal.get(ch) },
            );
            let shoulder = (at + PERSON_W - 1, top + 7);
            if wave {
                // An arm up, the hand swinging from side to side.
                let sway = ((self.t * 5.0 + unit(n, self.seed) * TAU).sin() * 1.6).round() as i32;
                let hand = (at + 10 + sway, top - 3);
                pen.line(shoulder, hand, 1, pal.of(b'M'));
                pen.px(hand.0, hand.1 - 1, pal.of(b'F'));
            } else {
                pen.line((at, top + 7), (at - 1, top + 10), 1, pal.of(b'M'));
            }
            if rifle {
                // Held upright at the side, the barrel above the shoulder.
                pen.line((at - 2, top + 10), (at - 2, top - 5), 1, pal.of(b'b'));
                pen.line((at - 2, top - 2), (at - 2, top - 7), 1, pal.of(b'S'));
            }
        }
    }

    /// Smoke from the stack: puffs that swell and pale, and a few sparks.
    fn smoke(&self, pen: &mut Pen, pal: &Palette, rail: i32) {
        let n = (self.t * PUFFS).floor() as i32;
        let lift = self.jolt(4);
        for s in (n - (SMOKE_LIFE * PUFFS) as i32).max(0)..=n {
            let born = s as f32 / PUFFS;
            let age = self.t - born;
            if !(0.0..SMOKE_LIFE).contains(&age) {
                continue;
            }
            // Where the stack was when it left: the train's place then.
            let stack_x = born * self.speed - TRAIN_L as f32 + (LOCO + STACK.0) as f32;
            let d = hash(s as u32, self.seed);
            let x = stack_x - 2.0 * age + (unit(d, 1) - 0.5) * 2.0;
            let y = (STACK.1 + 2 + lift) as f32 + 5.0 * age.powf(0.6) + unit(d, 2) * 1.5;
            let k = age / SMOKE_LIFE;
            // It swells, then thins out to nothing.
            let thin = ((k - 0.6) / 0.4).clamp(0.0, 1.0);
            let r = (1.5 + 1.7 * age.powf(0.9)) * (1.0 - thin * thin);
            if r < 0.6 {
                continue;
            }
            // Mottled: some puffs darker, all paling as they thin out.
            let base = if (d >> 5).is_multiple_of(2) {
                pal.of(b'x')
            } else {
                pal.of(b'X')
            };
            let col = base.lerp(pal.of(b'X'), (k * 1.4).min(1.0));
            pen.blob((x, rail as f32 - y), (r, r * 0.85), col);
            // Some puffs carry a spark that arcs up and falls.
            if d.is_multiple_of(3) && age < 0.9 {
                let sx = x - 3.0 * age;
                let sy = rail as f32 - (STACK.1 + 3 + lift) as f32 - 9.0 * age + 14.0 * age * age;
                pen.px(
                    sx.round() as i32,
                    sy.round() as i32,
                    pal.of(if d.is_multiple_of(2) { b'Y' } else { b'y' }),
                );
            }
        }
    }

    /// Now and then somebody on the train shouts.
    fn shouts(&self, pen: &mut Pen, pal: &Palette, theme: &Theme, (rear, rail): (i32, i32)) {
        let slot = self.t / 4.0;
        let age = (slot.fract() - 0.2) * 4.0;
        if !(0.0..1.8).contains(&age) {
            return;
        }
        let n = slot as u32;
        let who = hash(n, self.seed) as usize % RIDERS.len();
        let words = SHOUTS[hash(n, 3) as usize % SHOUTS.len()];
        let (x, y, ..) = RIDERS[who];
        let len = words.chars().count() as i32;
        let (col, row) = pen.st.cell(pen.x(rear + x + 4), rail - y - PERSON_H - 7);
        let color = fade(theme, pal.of(b'o'), ((age - 1.2) / 0.6).clamp(0.0, 1.0));
        pen.c
            .text_attr(col - len / 2, row - (age * 1.2) as i32, words, color, BOLD);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        for art in [MAN, WOMAN] {
            assert!(art.iter().all(|r| r.len() as i32 == PERSON_W));
            assert_eq!(art.len() as i32, PERSON_H);
        }
        assert!(CARS.iter().all(|&x| x + CAR_W <= TENDER));
        const { assert!(TENDER + 26 < LOCO) };
        // The banner fits over the three cars.
        assert!(text_width(BANNER, 1) <= CARS[2] + CAR_W);
        assert!(RIDERS.iter().all(|r| r.0 + PERSON_W <= TRAIN_L));
    }

    /// The train crosses the screen in either direction, the banner readable
    /// whichever way it faces, and leaves its smoke behind it.
    #[test]
    fn rolls_across_with_its_banner() {
        let theme = Theme::classic();
        for seed in 0..6 {
            let mut e = start(160, 36, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(160, 36, Color::Reset);
            let mut most = 0;
            let mut secs = 0.0;
            while !e.done() {
                e.update(0.1);
                secs += 0.1;
                c.clear();
                e.draw(&mut c, &theme);
                most = most.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
            }
            assert!(most > 900, "{most} cells");
            assert!(secs < 29.6, "{secs}");
        }
    }
}
