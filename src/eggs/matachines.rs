//! On the twelfth of December the matachines dance for the Virgen de
//! Guadalupe, a drum and a violin keeping time.
//!
//! The Danza de los Matachines is danced before the Virgen in towns across
//! Mexico, in the atrium of the church or under an arch of flowers: two rows
//! of dancers in red and white, with tall penachos of feathers in every color
//! and a sonaja, a rattle, in the hand, stamp and turn to a violin and a
//! tambor that never stop. This draws it in original oxiper pixel art that
//! grows with the screen. An arch of rosas de Castilla and leaves with gold
//! stars rises from the ground, the framed image of the Virgen on its crown,
//! veladoras and ramos at the foot of its posts. The músicos walk in first,
//! the drummer at one end with his tambor at his waist, the violinist at the
//! other, in straw hats and white manta, tapping their feet; then the
//! matachines come dancing in from both sides and take their places in a row
//! under the arch, as many as the screen has room for: skirts of carrizos in red
//! and white stripes that flare and swing, a bandolera across the chest, a
//! headband and a tall penacho of feathers in rosa, amarillo, turquesa, morado,
//! naranja, rojo, verde and azul that sways, bells at the ankles, a sonaja
//! shaking in one hand and a red paliacate waving in the other. They hop on every
//! beat of the drum, one foot and then the other, the stick of the drummer
//! falling with them, the bow sawing, notes (♪ ♫) rising from the violin. When
//! the dance is done they leave as they came and the arch sinks back into the
//! ground. All within 30 s.

use super::blit::plume;
use super::guadalupe::{Palette, Place, ROSA, fade, fit_down};
use super::{Egg, Info};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::PI;

pub const INFO: Info = Info {
    name: "matachines",
    start,
    fiesta: Kind::Navidad,
    when: |fiesta| fiesta.guadalupe(),
};

/// When things happen, in seconds: the arch is risen, the músicos at the ends
/// of the row walk in and each one after them follows `COME` later, the first
/// ones to walk off leave (the músicos last, `GO` after each other), crossing
/// the screen takes `WALK`, the arch sinks and it is over.
const RISEN: f32 = 2.8;
const ENTER: f32 = 0.5;
const COME: f32 = 0.9;
const WALK: f32 = 4.0;
const LEAVE: f32 = 20.0;
const GO: f32 = 0.5;
const SINKING: f32 = 24.4;
const END: f32 = 27.8;
/// One beat of the drum, in seconds.
const BEAT: f32 = 0.5;
const NOTES: [&str; 3] = ["♪", "♫", "♩"];

/// How tall the arch is with the image on its crown, in art pixels, and how
/// far apart the dancers, the músicos and the posts of the arch stand.
const HEIGHT: i32 = 58;
const PITCH: f32 = 17.0;
const POSTS: f32 = 6.0;
/// The feathers of the penachos, in every color.
const PLUMES: [u8; 8] = *b"PLQIOMGB";

/// A matachin's head from the front: black hair, a red and gold headband.
const HEAD: &[&str] = &[
    "..NNNNN..",
    ".NNNNNNN.",
    ".MYMYMYM.",
    ".NKKKKKN.",
    ".NKnKnKN.",
    "..KKkKK..",
    "..KKmKK..",
    "...KKK...",
];
/// A músico's head from the front, in a sombrero de palma.
const MUSICO: &[&str] = &[
    ".....hhh.....",
    "....hhhhh....",
    "....uuuuu....",
    ".hhhhhhhhhhh.",
    "..uuuuuuuuu..",
    "....kKKKk....",
    "....KnKnK....",
    "....KKkKK....",
    ".....KKK.....",
];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Who {
    Drummer,
    Violinist,
    Dancer,
}

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Matachines::new(w, h, rng))
}

struct Matachines {
    t: f32,
    w: i32,
    h: i32,
    /// Screen pixels to an art pixel, and the screen's width in art pixels.
    k: f32,
    wide: f32,
    /// How many dancers there are room for.
    dancers: usize,
    seed: u32,
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

impl Matachines {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Matachines {
        let mut rng = fastrand::Rng::with_seed(rng.u64(..));
        let k = fit_down(h, w, HEIGHT, 100.0);
        let wide = w as f32 / k;
        Matachines {
            t: 0.0,
            w: w as i32,
            h: h as i32,
            k,
            wide,
            dancers: (((wide - 2.0 * POSTS) / PITCH).floor() as usize)
                .saturating_sub(2)
                .clamp(3, 7),
            seed: rng.u32(..),
        }
    }

    /// How far the arch has risen: 0 under the ground, 1 standing.
    fn rise(&self) -> f32 {
        let up = ease_out(self.t / RISEN);
        let down = ((self.t - SINKING) / (END - SINKING)).clamp(0.0, 1.0);
        (up - down * down).clamp(0.0, 1.0)
    }

    /// How many are in the row: the drummer, the dancers, the violinist.
    fn slots(&self) -> usize {
        self.dancers + 2
    }

    /// Who stands in slot `i`, and where, in art pixels from the middle.
    fn slot(&self, i: usize) -> (Who, f32) {
        let who = if i == 0 {
            Who::Drummer
        } else if i == self.slots() - 1 {
            Who::Violinist
        } else {
            Who::Dancer
        };
        (who, (i as f32 - (self.slots() - 1) as f32 * 0.5) * PITCH)
    }

    /// How many steps in from the outer músicos slot `i` is: 0 for them, one
    /// more for each dancer inward.
    fn order(&self, i: usize) -> usize {
        i.min(self.slots() - 1 - i)
    }

    /// Slot `i`'s distance from its place, in art pixels, toward the edge
    /// of the screen on its side, and whether it is on the move. They all
    /// walk at one pace, the outer ones first, so they come in as a line
    /// and leave as one.
    fn away(&self, i: usize) -> (f32, bool) {
        let (_, x) = self.slot(i);
        let travel = (self.wide * 0.5 + 24.0 - x.abs()).max(0.0);
        let pace = (self.wide * 0.5 + 24.0) / WALK;
        let (order, inner) = (self.order(i) as f32, ((self.slots() - 1) / 2) as f32);
        let come = pace * (self.t - (ENTER + COME * order));
        let go = pace * (self.t - (LEAVE + GO * (inner - order)));
        let off = travel - come.clamp(0.0, travel) + go.clamp(0.0, travel);
        (off, come < travel || go > 0.0)
    }
}

impl Egg for Matachines {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::guadalupe(theme);
        let ground = self.h as f32 * 2.0;
        let rise = self.rise();
        let at = Place {
            pal: &pal,
            k: self.k,
            s: 1.0,
            ox: self.w as f32 * 0.5,
            ground,
        };
        if rise > 0.0 {
            let sunk = (1.0 - rise) * (HEIGHT as f32 * self.k + 14.0);
            let arch = Place {
                ground: ground + sunk,
                ..at
            };
            arch.arch(c, self.slots() as f32 * PITCH * 0.5 + POSTS, self.t);
        }
        for i in 0..self.slots() {
            let (who, x) = self.slot(i);
            let (off, moving) = self.away(i);
            let side = if x < 0.0 || (x == 0.0 && i % 2 == 0) {
                -1.0
            } else {
                1.0
            };
            let me = Place {
                ox: at.ox + (x + side * off) * at.k,
                ..at
            };
            // The beat is the same for everybody; a dancer's foot is the
            // other one from his neighbor's.
            let beat = self.t / BEAT;
            match who {
                Who::Dancer => me.matachin(c, i as u32 + self.seed, (self.t, beat), moving),
                Who::Drummer => me.drummer(c, beat),
                Who::Violinist => me.violinist(c, (self.t, beat)),
            }
        }
        self.notes(c, theme, &pal, ground);
    }

    fn done(&self) -> bool {
        self.t > END || self.t > 29.5
    }
}

impl Matachines {
    /// The notes of the violin, floating up from it and swaying, fading.
    fn notes(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, ground: f32) {
        let i = self.slots() - 1;
        let (_, x) = self.slot(i);
        let (off, _) = self.away(i);
        if off > 1.0 || !(RISEN..LEAVE + 2.0).contains(&self.t) {
            return;
        }
        let x0 = self.w as f32 * 0.5 + (x - 7.0) * self.k;
        let top = ground - 28.0 * self.k;
        for j in 0..3 {
            let age = ((self.t * 0.6 + j as f32 * 0.33) % 1.0).abs();
            let drift = (age * 6.0 + j as f32 * 2.0).sin() * 2.0;
            let color = fade(theme, pal.of(if j == 1 { b'r' } else { b'Y' }), age * age);
            let note = NOTES[(j + (self.t * 0.6) as usize) % NOTES.len()];
            let y = top - age * 18.0 * self.k.max(0.5);
            c.text_attr(
                (x0 + drift - age * 4.0).round() as i32,
                (y / 2.0).floor() as i32,
                note,
                color,
                BOLD,
            );
        }
    }
}

/// How the dance is at some moment: how high the dancer is off the ground
/// (0 to 1 between two beats) and which foot is up.
fn hop(beat: f32, who: u32) -> (f32, f32) {
    let up = (beat * PI).sin().abs();
    let left = (beat.floor() as i64 + i64::from(who)) % 2 == 0;
    (up, if left { 1.0 } else { -1.0 })
}

impl Place<'_> {
    /// The arch: posts wound with leaves and rosas de Castilla, a garland over
    /// them rising to a crown, with gold stars in it, the image of the Virgen
    /// on top, and at the foot of each post veladoras and a ramo. `half` is how
    /// far the posts stand from the middle.
    fn arch(&self, c: &mut Canvas, half: f32, t: f32) {
        let pal = self.pal;
        let post = 33.0;
        let rose = |c: &mut Canvas, x: f32, y: f32| {
            self.sprite(c, ROSA, (x - 2.5, y + 2.5), (0.0, 1.0), |l| pal.get(l));
        };
        // The posts.
        for side in [-1.0f32, 1.0] {
            let x = side * half;
            self.block(c, (x - 1.0, 0.0), (2.0, post), pal.of(b'u'));
            for i in 0..6 {
                let y = 2.0 + i as f32 * 5.0;
                let wiggle = if i % 2 == 0 { -1.2 } else { 1.2 };
                self.block(c, (x - 2.5 + wiggle, y), (5.0, 3.0), pal.of(b'G'));
                self.block(c, (x - 1.5 - wiggle, y + 2.0), (3.0, 1.0), pal.of(b'g'));
                if i % 2 == 1 {
                    rose(c, x - side * 2.0, y + 1.0);
                }
            }
            for (j, glass) in b"VWv".iter().copied().enumerate() {
                let vx = x - side * (5.0 + j as f32 * 4.0) - 1.5;
                self.veladora(c, (vx, 0.0), glass, t + j as f32 + side);
            }
        }
        // The garland over them, a low arc, in two greens.
        let apex = 40.0;
        let steps = 28;
        let at = |u: f32| {
            (
                -half + 2.0 * half * u,
                post + (apex - post) * 4.0 * u * (1.0 - u),
            )
        };
        let mut prev = at(0.0);
        for i in 1..=steps {
            let next = at(i as f32 / steps as f32);
            self.limb(
                c,
                (prev.0, prev.1 - 0.8),
                (next.0, next.1 - 0.8),
                3.4,
                pal.of(b'g'),
            );
            self.limb(c, prev, next, 3.0, pal.of(b'G'));
            prev = next;
        }
        for i in 0..=(half * 2.0 / 8.0) as i32 {
            let u = (i as f32 * 8.0 / (half * 2.0)).min(1.0);
            let (x, y) = at(u);
            rose(c, x, y - 0.5);
            let (sx, sy) = at(((i as f32 + 0.5) * 8.0 / (half * 2.0)).min(1.0));
            self.block(c, (sx - 0.5, sy + 0.5), (1.4, 1.4), pal.of(b'Y'));
        }
        // The image on the crown, with a rose at each side of it.
        self.framed(c, (-6.0, apex + 2.0 + 16.0), 0.55);
        rose(c, -11.5, apex + 1.0);
        rose(c, 11.5, apex + 1.0);
    }

    /// A matachin from the front, at some moment of the dance: `t` the time
    /// and `beat` the number of beats since the start; `moving` if he is on
    /// his way in or out, when the dance is quicker.
    fn matachin(&self, c: &mut Canvas, who: u32, (t, beat): (f32, f32), moving: bool) {
        let pal = self.pal;
        let beat = if moving { beat * 2.0 } else { beat };
        let (up, foot) = hop(beat, who);
        let jump = 1.8 * up;
        let flare = 0.7 * up;
        let sway = 0.8 * (beat * PI * 0.5).sin();
        let (waist, shoulder) = (14.0 + jump, 23.0 + jump);

        // The legs in white manta, the ankle bells and the huaraches; the
        // foot that is up kicks to the side.
        for side in [-1.0f32, 1.0] {
            let lifted = side == foot;
            let lift = if lifted { 2.2 * up } else { 0.0 };
            let ft = (side * (2.8 + if lifted { 1.2 * up } else { 0.0 }), lift);
            self.limb(
                c,
                (side * 1.8, waist - 6.0),
                (ft.0, ft.1 + 2.5),
                2.4,
                pal.of(b'A'),
            );
            self.block(c, (ft.0 - 1.4, ft.1 + 2.0), (2.8, 1.0), pal.of(b'u'));
            self.block(c, (ft.0 - 1.6, ft.1), (3.4, 1.4), pal.of(b'u'));
        }
        // The skirt of carrizos: stripes of red and white that flare and swing.
        for r in 0..8 {
            let y = waist - 1.0 - r as f32;
            let half = 4.6 + r as f32 * 0.55 + flare * r as f32 * 0.4;
            let swing = sway * r as f32 / 8.0 * 1.4;
            let n = (half * 2.0 / 1.5).ceil() as i32;
            for s in 0..n {
                let x = -half + s as f32 * 1.5 + swing;
                let color = if r == 7 {
                    pal.of(b'Y')
                } else if s % 2 == 0 {
                    pal.of(b'M')
                } else {
                    pal.of(b'W')
                };
                self.block(
                    c,
                    (x, y),
                    (1.5f32.min(half - (x - swing) - 0.0).max(0.5), 1.0),
                    color,
                );
            }
            if r % 3 == 1 {
                self.block(c, (-half + swing, y), (2.0 * half, 0.5), pal.of(b'm'));
            }
        }
        // The blouse, the bandolera of red across it and the belt.
        self.block(
            c,
            (-4.5 + sway * 0.3, waist - 0.5),
            (9.0, shoulder - waist + 0.5),
            pal.of(b'W'),
        );
        self.block(
            c,
            (-5.2 + sway * 0.3, shoulder - 3.0),
            (10.4, 3.2),
            pal.of(b'W'),
        );
        self.limb(
            c,
            (-4.2, shoulder - 0.8),
            (4.0, waist + 0.8),
            1.4,
            pal.of(b'M'),
        );
        self.limb(
            c,
            (4.2, shoulder - 0.8),
            (-4.0, waist + 0.8),
            1.4,
            pal.of(b'L'),
        );
        self.block(c, (-4.8, waist - 0.8), (9.6, 1.6), pal.of(b'M'));
        // The arms: the sonaja shaking at one side, the paliacate waving at the
        // other.
        let shake = (t * 26.0).sin() * 0.7;
        let hand_r = (8.0 + shake * 0.6, shoulder - 0.5 + 2.0 * up + shake);
        self.limb(c, (4.8, shoulder - 1.0), hand_r, 2.4, pal.of(b'W'));
        self.disc(c, (hand_r.0 + 0.5, hand_r.1 + 2.2), 1.9, pal.of(b'D'));
        self.block(
            c,
            (hand_r.0 - 0.2, hand_r.1 + 0.4),
            (1.4, 1.2),
            pal.of(b'u'),
        );
        self.block(
            c,
            (hand_r.0 + 2.8 * shake.signum(), hand_r.1 + 3.8),
            (0.8, 0.8),
            pal.of(b'Z'),
        );
        let wave = (beat * PI).sin();
        let hand_l = (
            -8.0 - 0.6 * up,
            shoulder - 3.0 + 3.5 * up * foot.max(0.0) + 1.0 * wave,
        );
        self.limb(c, (-4.8, shoulder - 1.0), hand_l, 2.4, pal.of(b'W'));
        self.limb(
            c,
            hand_l,
            (hand_l.0 - 2.0, hand_l.1 - 2.5 - wave),
            1.6,
            pal.of(b'M'),
        );
        for h in [hand_r, hand_l] {
            self.block(c, (h.0 - 1.0, h.1 - 1.0), (2.0, 2.0), pal.of(b'K'));
        }
        // The head, and the penacho.
        let head = shoulder + HEAD.len() as f32 - 0.5;
        self.sprite(
            c,
            HEAD,
            (-4.5 + sway * 0.4, head),
            (0.0, 1.0),
            |l| match l {
                b'm' => pal.get(b'q'),
                l => pal.get(l),
            },
        );
        self.penacho(c, (sway * 0.4, head), who, t);
    }

    /// A penacho on a head whose top is at `top`: a band of red and gold with
    /// mirrors and a fan of nine feathers in every color that sways.
    fn penacho(&self, c: &mut Canvas, (x, top): (f32, f32), who: u32, t: f32) {
        let pal = self.pal;
        let crown = 2.8;
        for i in 0..6 {
            let color = if i % 2 == 0 { b'M' } else { b'Y' };
            self.block(
                c,
                (x - 4.5 + i as f32 * 1.5, top),
                (1.5, crown),
                pal.of(color),
            );
        }
        for i in [1usize, 3, 5] {
            self.block(
                c,
                (x - 4.5 + i as f32 * 1.5 + 0.3, top + 1.0),
                (0.9, 0.9),
                pal.of(b'i'),
            );
        }
        let (bx, by) = (x, top + crown - 0.4);
        for j in 0..9i32 {
            let spread = (j - 4) as f32 / 4.0;
            let a = spread * 1.15 + 0.12 * (t * PI / BEAT * 0.5 + j as f32 * 0.5).sin();
            let len = 11.0 - 3.0 * spread.abs();
            let tip = (bx + a.sin() * len, by + a.cos() * len);
            let color = PLUMES[(who as usize + j as usize) % PLUMES.len()];
            plume(
                c,
                self.at(bx + spread * 1.5, by),
                self.at(tip.0, tip.1),
                1.4 * self.k * spread.signum(),
                (2.6 * self.k).max(1.5),
                (pal.of(color), pal.of(b'Z')),
            );
        }
    }

    /// The drummer, from the front, in a straw hat and white manta, with his
    /// tambor at his belly, the sticks falling on the beat, a foot tapping.
    fn drummer(&self, c: &mut Canvas, beat: f32) {
        let pal = self.pal;
        let up = (beat * PI).sin().abs();
        let (waist, shoulder) = (12.0, 21.0);
        let sway = 0.7 * (beat * PI * 0.5).sin();
        let tap = beat.floor() as i64 % 2 == 0;
        for side in [-1.0f32, 1.0] {
            let lift = if (side > 0.0) == tap { 1.2 * up } else { 0.0 };
            self.limb(
                c,
                (side * 1.8, waist),
                (side * 2.4, lift + 1.6),
                3.0,
                pal.of(b'A'),
            );
            self.block(c, (side * 2.4 - 1.8, lift), (3.6, 1.6), pal.of(b'u'));
        }
        self.block(
            c,
            (-4.4 + sway, waist - 0.5),
            (8.8, shoulder - waist + 0.5),
            pal.of(b'A'),
        );
        self.block(c, (-5.2 + sway, shoulder - 3.0), (10.4, 3.2), pal.of(b'A'));
        self.block(c, (-4.4 + sway, waist - 0.5), (8.8, 1.4), pal.of(b'M'));
        // The tambor, across his belly: its body of wood, the cords and the
        // skins at both ends.
        let drum = (0.0 + sway, 13.0);
        self.block(c, (drum.0 - 7.0, drum.1 - 3.6), (14.0, 7.2), pal.of(b'D'));
        self.block(c, (drum.0 - 7.0, drum.1 - 3.6), (14.0, 1.0), pal.of(b'u'));
        self.block(c, (drum.0 - 7.0, drum.1 + 2.6), (14.0, 1.0), pal.of(b'u'));
        for i in 0..6 {
            let x = drum.0 - 5.5 + i as f32 * 2.2;
            self.limb(
                c,
                (x, drum.1 - 2.6),
                (x + 1.1, drum.1 + 2.6),
                0.5,
                pal.of(b'M'),
            );
            self.limb(
                c,
                (x + 1.1, drum.1 - 2.6),
                (x + 2.2, drum.1 + 2.6),
                0.5,
                pal.of(b'W'),
            );
        }
        self.disc(c, (drum.0 - 7.0, drum.1), 3.8, pal.of(b'u'));
        self.disc(c, (drum.0 - 7.0, drum.1), 3.0, pal.of(b'A'));
        self.disc(c, (drum.0 + 7.0, drum.1), 3.8, pal.of(b'u'));
        self.disc(c, (drum.0 + 7.0, drum.1), 3.0, pal.of(b'A'));
        // The sticks, one hand and then the other, each one falling on the
        // skin on its beat.
        for (j, side) in [-1.0f32, 1.0].into_iter().enumerate() {
            let phase = (beat * 2.0 + j as f32 * 0.5).fract();
            let lift = 5.0 * (1.0 - phase).powi(2);
            let hand = (side * 5.0 + sway, drum.1 + 4.5 + lift);
            self.limb(
                c,
                (side * 4.8 + sway, shoulder - 1.0),
                hand,
                2.4,
                pal.of(b'A'),
            );
            self.limb(
                c,
                hand,
                (side * 2.4 + sway, drum.1 + 3.6 + lift * 0.3),
                0.8,
                pal.of(b'Z'),
            );
            self.block(c, (hand.0 - 1.0, hand.1 - 1.0), (2.0, 2.0), pal.of(b'K'));
        }
        let head = shoulder + MUSICO.len() as f32 - 0.5;
        self.sprite(c, MUSICO, (-6.5 + sway, head), (0.0, 1.0), |l| pal.get(l));
    }

    /// The violinist, from the front, his violin at his shoulder, the bow
    /// sawing, a foot tapping.
    fn violinist(&self, c: &mut Canvas, (t, beat): (f32, f32)) {
        let pal = self.pal;
        let up = (beat * PI).sin().abs();
        let (waist, shoulder) = (12.0, 21.0);
        let sway = 0.8 * (beat * PI * 0.5).sin();
        let tap = beat.floor() as i64 % 2 == 1;
        for side in [-1.0f32, 1.0] {
            let lift = if (side > 0.0) == tap { 1.2 * up } else { 0.0 };
            self.limb(
                c,
                (side * 1.8, waist),
                (side * 2.4, lift + 1.6),
                3.0,
                pal.of(b'A'),
            );
            self.block(c, (side * 2.4 - 1.8, lift), (3.6, 1.6), pal.of(b'u'));
        }
        self.block(
            c,
            (-4.4 + sway, waist - 0.5),
            (8.8, shoulder - waist + 0.5),
            pal.of(b'A'),
        );
        self.block(c, (-5.2 + sway, shoulder - 3.0), (10.4, 3.2), pal.of(b'A'));
        self.block(c, (-4.4 + sway, waist - 0.5), (8.8, 1.4), pal.of(b'M'));
        let head = shoulder + MUSICO.len() as f32 - 0.5;
        self.sprite(c, MUSICO, (-6.5 + sway, head), (0.0, 1.0), |l| pal.get(l));
        // The violin at the left shoulder, its neck out to the side, and the
        // bow across it.
        let saw = (t * 5.5).sin();
        let body = (-4.2 + sway, shoulder + 0.8);
        self.limb(c, body, (-11.0 + sway, shoulder + 4.2), 0.9, pal.of(b'u'));
        self.disc(c, body, 2.5, pal.of(b'D'));
        self.disc(c, body, 1.0, pal.of(b'u'));
        let bow = (3.0 + saw * 2.0 + sway, shoulder - 1.5);
        self.limb(
            c,
            (bow.0 - 8.0, bow.1 + 2.2),
            (bow.0 + 1.0, bow.1 - 0.4),
            0.6,
            pal.of(b'Z'),
        );
        self.limb(c, (4.8 + sway, shoulder - 1.0), bow, 2.4, pal.of(b'A'));
        self.limb(
            c,
            (-4.8 + sway, shoulder - 1.0),
            (-9.0 + sway, shoulder + 3.2),
            2.4,
            pal.of(b'A'),
        );
        for h in [bow, (-9.0 + sway, shoulder + 3.2)] {
            self.block(c, (h.0 - 1.0, h.1 - 1.0), (2.0, 2.0), pal.of(b'K'));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, rgb};

    fn egg(w: u16, h: u16) -> Matachines {
        Matachines::new(w, h, &mut fastrand::Rng::with_seed(4))
    }

    fn shows(c: &Canvas, hex: u32) -> bool {
        c.cells()
            .iter()
            .any(|cell| cell.fg == rgb(hex) || cell.bg == rgb(hex))
    }

    #[test]
    fn the_art_is_whole() {
        let pal = Palette::guadalupe(&Theme::classic());
        for art in [HEAD, MUSICO] {
            assert!(art.iter().all(|r| r.len() == art[0].len()));
            for l in art.iter().flat_map(|r| r.bytes()) {
                assert!(l == b'.' || pal.get(l).is_some(), "{}", l as char);
            }
        }
        for l in PLUMES {
            assert!(pal.get(l).is_some(), "{}", l as char);
        }
    }

    /// The arch rises with its image, stands, and sinks.
    #[test]
    fn the_arch_rises_and_sinks() {
        let mut e = egg(100, 30);
        let mut rise = |t: f32| {
            e.t = t;
            e.rise()
        };
        assert_eq!(rise(0.0), 0.0);
        assert!(rise(RISEN) > 0.99);
        assert!(rise(SINKING + 1.5) < rise(SINKING));
        assert_eq!(rise(END), 0.0);
    }

    /// The row has the drummer, the violinist and at least three dancers,
    /// centered, and fits the screen.
    #[test]
    fn the_row_has_room_for_everyone() {
        for (w, h) in [(80, 24), (100, 30), (160, 50), (300, 60), (40, 14)] {
            let e = egg(w, h);
            assert!(e.dancers >= 3 && e.dancers <= 7, "{w}x{h}");
            assert_eq!(e.slot(0).0, Who::Drummer);
            assert_eq!(e.slot(e.slots() - 1).0, Who::Violinist);
            assert!((1..e.slots() - 1).all(|i| e.slot(i).0 == Who::Dancer));
            let (_, left) = e.slot(0);
            let (_, right) = e.slot(e.slots() - 1);
            assert!((left + right).abs() < 1e-3);
            if e.dancers > 3 {
                assert!(
                    right + PITCH * 0.5 + POSTS <= e.wide * 0.5 + 0.01,
                    "{w}x{h}"
                );
            }
        }
    }

    /// Everyone walks in from the edge of the screen, to their place, the outer
    /// ones first and at one pace, stays there while the dance lasts, and walks
    /// off again.
    #[test]
    fn they_come_dancing_in_and_leave() {
        for (w, h) in [(100, 30), (120, 36), (300, 60)] {
            let mut e = egg(w, h);
            let inner = ((e.slots() - 1) / 2) as f32;
            for i in 0..e.slots() {
                let travel = e.wide * 0.5 + 24.0 - e.slot(i).1.abs();
                e.t = 0.0;
                assert_eq!(e.away(i).0, travel, "{w}x{h} {i}");
                e.t = ENTER + COME * inner + WALK + 0.01;
                assert_eq!(e.away(i), (0.0, false), "{w}x{h} {i}");
                e.t = LEAVE - 0.01;
                assert_eq!(e.away(i), (0.0, false), "{w}x{h} {i}");
                e.t = END;
                let (off, moving) = e.away(i);
                assert!(off >= travel - 0.01 && moving, "{w}x{h} {i}");
            }
            // The músicos are first in and last out; the dance has begun
            // before the arch sinks.
            e.t = ENTER + WALK + 0.01;
            assert_eq!(e.away(0), (0.0, false));
            assert_eq!(e.away(e.slots() - 1), (0.0, false));
            assert!(e.away(e.slots() / 2).1);
            assert!(ENTER + COME * inner + WALK < LEAVE - 6.0);
            e.t = LEAVE + 0.1;
            if e.slots() > 3 {
                assert!(e.away(e.slots() / 2).1 && !e.away(0).1);
            }
            assert!(END - LEAVE - GO * inner >= WALK);
        }
    }

    /// A foot is up for a beat, and the other one the next.
    #[test]
    fn the_feet_alternate_on_the_beat() {
        for who in 0..4u32 {
            let (up0, foot0) = hop(0.25, who);
            let (up1, foot1) = hop(1.25, who);
            assert!(up0 > 0.0 && up1 > 0.0);
            assert_eq!(foot0, -foot1);
            // Neighbors have their feet the other way round.
            assert_eq!(hop(0.25, who).1, -hop(0.25, who + 1).1);
        }
        assert!(hop(1.0, 0).0 < 1e-5 && hop(1.5, 0).0 > 0.99);
    }

    /// Red and white skirts, penachos in every color, the drum and the
    /// violin, the arch of roses with its image and the veladoras: all on
    /// show at some time, the notes rise, and at the end nothing is left.
    #[test]
    fn the_dance_comes_dressed_and_goes() {
        let theme = Theme::classic();
        let mut e = egg(120, 40);
        let mut c = Canvas::new(120, 40, Color::Reset);
        let wanted = [
            0xc8102e, 0xf4f1ea, 0xe4007c, 0xffd200, 0x00aab8, 0x7028a0, 0xff8c1a, 0x2e7d32,
            0x2a5caa, 0xd81b60, 0x00796b, 0xa0703a, 0xffcc4d, 0xefe6cf, 0xd7a066,
        ];
        let mut seen = [false; 15];
        let mut notes = false;
        while !e.done() {
            e.update(0.1);
            c.clear();
            e.draw(&mut c, &theme);
            for (i, hex) in wanted.iter().enumerate() {
                seen[i] |= shows(&c, *hex);
            }
            notes |= NOTES.iter().any(|n| c.to_text().contains(n));
        }
        assert!(seen.iter().all(|&s| s), "{seen:?}");
        assert!(notes);
        assert!(c.cells().iter().all(|cell| cell.ch == ' '));
    }

    /// A theme other than `classic` paints the dance with its own colors.
    #[test]
    fn a_theme_gives_its_own_colors() {
        let nord = Theme::builtin("nord").unwrap();
        let mut e = egg(100, 40);
        e.t = 12.0;
        let mut c = Canvas::new(100, 40, Color::Reset);
        e.draw(&mut c, &nord);
        assert!(!shows(&c, 0xd81b60) && !shows(&c, 0xc8102e));
        assert!(c.cells().iter().any(|cell| cell.ch != ' '));
    }
}
