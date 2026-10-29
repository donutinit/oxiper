//! Four soldaderas march across the screen with a horseman riding alongside,
//! on the Día de la Revolución.
//!
//! Original oxiper pixel art, after the women who followed the armies of the
//! Revolution, the Adelitas. Each wears a straw sombrero with a red band, a
//! long braid tied with a ribbon, a rebozo over the shoulders, a blouse of
//! manta, two cananas crossed over the chest (leather, with brass cartridges)
//! and a pleated skirt to the huaraches, and each carries something
//! different: the first a tricolor flag rippling on its pole, the second a
//! rifle on her shoulder, the third a rifle low at her side, the fourth her
//! rifle held high. They march in step, skirts swaying, a puff of dust at
//! every footfall. Behind them, in the far lane, a horseman in a great
//! sombrero walks his horse (the four legs in a true walking gait, the tail
//! swishing, the mane stirring) and now and then raises his rifle with a
//! shout. They come from either side and are gone within 30 s.

use super::revolucion::{Palette, fade};
use super::stage::{Pen, Stage, hash, unit};
use super::{Egg, Info};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::PI;

pub const INFO: Info = Info {
    name: "adelitas",
    start,
    fiesta: Kind::Revolucion,
    when: super::any_day,
};

/// A soldadera facing right, from the hat to the waist: X is her rebozo.
/// (Her cananas, arm and rifle are drawn over it.)
const TOP: &[&str] = &[
    "....hhhhhh....",
    "...hhhhhhhh...",
    "...hRRRRRRh...",
    ".hhhhhhhhhhhh.",
    "hChCChCChCChCh",
    "...kkFFFFF....",
    "...kkFkFFF....",
    "...kkFFFFFf...",
    "...kkFFRFF....",
    "..bkk.FFF.....",
    "..bkXXXXXXX...",
    "..bXXXXXXXXX..",
    "..bXXMMMMMXX..",
    "..kXMMMMMMMX..",
    "..kXMMMMMMMX..",
    "..kXXMMMMMXX..",
    "..kXXXMMMXXX..",
    "..k.XXXMXXX...",
    "..k..XXXXX....",
    "..R..MMMMM....",
];
/// Her skirt: P its color, p its pleats.
const SKIRT: &[&str] = &[
    "...PPPPPPPP...",
    "..PPpPPPPpPP..",
    "..PPpPPPPpPP..",
    ".PPPpPPPPpPPP.",
    ".PPPpPPPPpPPP.",
    ".PPPpPPPPpPPP.",
    "PPPPpPPPPpPPPP",
    "PPPPpPPPPpPPPP",
    "PPPPpPPPPpPPPP",
    "PPPPpPPPPpPPPP",
    "PPPPpPPPPpPPPP",
    "PPPPpPPPPpPPPP",
    "bbbbbbbbbbbbbb",
];
/// Her huaraches, mid-stride and together.
const FEET: [&[&str]; 2] = [
    &["..uu......uu..", ".uuu......uuu."],
    &["....uu..uu....", "...uuuuuuuuu.."],
];
const WALKER_W: i32 = 14;
const WALKER_H: i32 = TOP.len() as i32 + SKIRT.len() as i32 + 2;
/// The horseman, from his hat to the waist, facing right.
const RIDER: &[&str] = &[
    ".....hhhhhh.....",
    "....hhhhhhhh....",
    "....hRRRRRRh....",
    "..hhhhhhhhhhhh..",
    "hhCCCCCCCCCCCChh",
    ".....kFFFFF.....",
    ".....kFkFFF.....",
    ".....kFFFFFf....",
    ".....kkkkFF.....",
    "......KFFF......",
    ".....MMMMMM.....",
    "....MMMMMMMM....",
    "...MMBMMMMBMM...",
    "...MMMBMMBMMM...",
    "...MMMMBBMMMM...",
    "...MMMBMMBMMM...",
    "...MMBMMMMBMM...",
    "....MMMMMMMM....",
    "....cccccccc....",
];
const RIDER_W: i32 = 16;

/// How a soldadera carries herself: what is in her hands.
#[derive(Clone, Copy, PartialEq)]
enum Carry {
    Flag,
    Shoulder,
    Low,
    High,
}

/// Each of the four: what she carries, her rebozo, her skirt, how far
/// behind the leader she walks.
const MARCHERS: [(Carry, u8, u8, i32); 4] = [
    (Carry::Flag, b'R', b'G', 0),
    (Carry::Shoulder, b'G', b'R', 22),
    (Carry::Low, b'O', b'c', 92),
    (Carry::High, b'R', b'O', 114),
];
/// The distance the horse's middle is behind the leader: he rides in the
/// column, between the second and third.
const HORSE_AT: i32 = 64;
/// How far behind the leader the column reaches, from the leader's front.
const COLUMN: i32 = 114 + WALKER_W;
/// Pixels per footfall, pixels per stride of the horse.
const STEP: f32 = 5.0;
const STRIDE: f32 = 24.0;
const CROSSING: f32 = 22.0;
const SHOUTS: [&str; 3] = ["¡Tierra y Libertad!", "¡Viva!", "¡Arriba!"];

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let st = Stage::new(w, h);
    Box::new(March {
        t: 0.0,
        speed: ((st.w + COLUMN + 30) as f32 / CROSSING).max(9.0),
        left: rng.bool(),
        w: st.w,
        seed: rng.u32(..),
    })
}

struct March {
    t: f32,
    /// Pixels per second.
    speed: f32,
    /// Marching to the left (the picture mirrored)?
    left: bool,
    w: i32,
    seed: u32,
}

impl March {
    /// How far the leader has marched.
    fn u(&self) -> f32 {
        self.t * self.speed
    }

    /// Is the horseman holding his rifle up, and is he shouting?
    fn raised(&self) -> bool {
        (self.t * 0.9 + 1.0).sin() > 0.35
    }

    /// Who shouts what, if anyone: (the marcher, 4 for the horseman, and the words).
    fn shout(&self) -> Option<(usize, &'static str, f32)> {
        let slot = self.t / 4.5;
        let age = (slot.fract() - 0.25) * 4.5;
        if !(0.0..1.8).contains(&age) {
            return None;
        }
        let n = slot as u32;
        let who = hash(n, self.seed) as usize % 5;
        let words = if who == 4 {
            0
        } else {
            1 + hash(n, 9) as usize % 2
        };
        Some((who, SHOUTS[words], age))
    }
}

impl Egg for March {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::revolucion(theme);
        let mut far = Palette::revolucion(theme);
        far.fade_all(theme, 0.12);
        let st = Stage::new(c.w(), c.h());
        let mut pen = Pen {
            c,
            st,
            mirror: self.left,
        };
        let g = st.h;
        let u = self.u();
        // The horseman a little farther back, the marchers over him.
        self.horseman(&mut pen, &far, (u - HORSE_AT as f32, g - 3));
        for (k, &(carry, rebozo, skirt, off)) in MARCHERS.iter().enumerate() {
            let x = (u - off as f32 - WALKER_W as f32).round() as i32;
            self.dust(&mut pen, &pal, off, k as u32);
            self.soldadera(&mut pen, &pal, (x, g), (carry, rebozo, skirt), off);
        }
        self.shouts(&mut pen, &pal, theme, g);
    }

    fn done(&self) -> bool {
        self.u() > (self.w + COLUMN + 24) as f32 || self.t > 29.5
    }
}

impl March {
    /// One soldadera: `at` is the left edge of her art and the ground row.
    fn soldadera(
        &self,
        pen: &mut Pen,
        pal: &Palette,
        (x, ground): (i32, i32),
        (carry, rebozo, skirt): (Carry, u8, u8),
        off: i32,
    ) {
        if x + WALKER_W < 0 || x > self.w {
            return;
        }
        let stride = (self.u() + off as f32 * 0.37) / STEP;
        let step = stride as i32 % 2;
        let bob = step;
        let top = ground - WALKER_H - bob;
        let sway = (stride * PI).sin();
        let (shawl, skirt_col) = (pal.of(rebozo), pal.of(skirt));
        let pleat = skirt_col.scale(0.7);
        let colors = |ch: u8| match ch {
            b'X' => Some(shawl),
            b'P' => Some(skirt_col),
            b'p' => Some(pleat),
            ch => pal.get(ch),
        };
        pen.sprite((x, top), TOP, false, |_| 0, colors);
        let rows = SKIRT.len() as f32;
        pen.sprite(
            (x, top + TOP.len() as i32),
            SKIRT,
            false,
            |r| (sway * 1.6 * r as f32 / rows).round() as i32,
            colors,
        );
        pen.sprite(
            (x, top + (TOP.len() + SKIRT.len()) as i32),
            FEET[step as usize],
            false,
            |_| 0,
            colors,
        );
        // Her cananas, crossed over the chest: leather with brass cartridges.
        for (a, b) in [
            ((x + 4, top + 11), (x + 9, top + 17)),
            ((x + 10, top + 11), (x + 5, top + 17)),
        ] {
            pen.line(a, b, 2, pal.of(b'b'));
            for i in 0..3 {
                let (px, py) = (a.0 + (b.0 - a.0) * i / 2, a.1 + (b.1 - a.1) * i / 2);
                pen.px(px, py, pal.of(b'A'));
            }
        }
        // Her arm and what it holds.
        let shoulder = (x + 9, top + 11);
        let (muzzle, hand) = match carry {
            Carry::Flag => ((x + 11, top - 18), (x + 11, top + 14)),
            Carry::Shoulder => ((x + 13, top + 6), (x + 9, top + 14)),
            Carry::Low => ((x, top + 21), (x + 11, top + 17)),
            Carry::High => ((x + 12, top - 12), (x + 12, top + 6)),
        };
        let butt = match carry {
            Carry::Flag => (x + 11, top + 20),
            Carry::Shoulder => (x + 2, top + 21),
            Carry::Low => (x + 13, top + 16),
            Carry::High => (x + 12, top + 10),
        };
        if carry == Carry::Flag {
            pen.line(butt, muzzle, 1, pal.of(b'u'));
            self.flag(pen, pal, (x + 12, top - 18));
        } else {
            pen.line(butt, muzzle, 1, pal.of(b'b'));
            let mid = (
                butt.0 + (muzzle.0 - butt.0) * 2 / 3,
                butt.1 + (muzzle.1 - butt.1) * 2 / 3,
            );
            pen.line(mid, muzzle, 1, pal.of(b'S'));
        }
        pen.line(shoulder, hand, 1, pal.of(b'M'));
        pen.px(hand.0, hand.1, pal.of(b'F'));
    }

    /// The flag on its pole: verde, blanco, rojo, rippling.
    fn flag(&self, pen: &mut Pen, pal: &Palette, (x, y): (i32, i32)) {
        for fx in 0..12 {
            let wave =
                ((self.t * 7.0 - fx as f32 * 0.6).sin() * 1.3 * fx as f32 / 12.0).round() as i32;
            let col = pal.of(b"GWR"[(fx / 4) as usize]);
            let col = if (fx / 4) == 1 { pal.of(b'M') } else { col };
            pen.rect(x + fx, y + wave, 1, 7, col);
            if fx / 4 == 1 && (5..7).contains(&fx) {
                pen.rect(x + fx, y + 2 + wave, 1, 3, pal.of(b'B'));
            }
        }
    }

    /// Dust kicked up at each of the last footfalls.
    fn dust(&self, pen: &mut Pen, pal: &Palette, off: i32, who: u32) {
        let g = pen.st.h;
        let phase = (self.u() + off as f32 * 0.37) / STEP;
        let n = phase.floor() as i32;
        for s in n - 2..=n {
            let age = (phase - s as f32) * STEP / self.speed;
            if !(0.0..0.6).contains(&age) {
                continue;
            }
            // Where the foot came down, as the column was then.
            let x = (s as f32 * STEP - off as f32 * 1.37 - 7.0).round() as i32;
            let d = hash(s as u32, who);
            for i in 0..3u32 {
                let dx = (unit(d, i) * 5.0) as i32 - 2 - (age * 6.0) as i32;
                let dy = 1 + (age * 5.0 * (0.5 + unit(d, i + 7))) as i32;
                pen.px(x + dx, g - dy, pal.of(b'h'));
            }
        }
    }

    /// The horseman: `at` is the middle of the horse's body and the ground
    /// row of his lane.
    fn horseman(&self, pen: &mut Pen, pal: &Palette, (mx, ground): (f32, i32)) {
        let mx = mx.round() as i32;
        if mx + 40 < 0 || mx - 40 > self.w {
            return;
        }
        let up = |y: i32| ground - y;
        let phase = self.u() / STRIDE;
        // Legs first: the far ones darker.
        let (dark, light) = (pal.of(b'b'), pal.of(b'B'));
        for (hip, offset, col) in [
            (-8, 0.5, dark),
            (8, 0.75, dark),
            (-11, 0.0, light),
            (11, 0.25, light),
        ] {
            let p = (phase + offset).fract();
            let (fx, fy) = if p < 0.6 {
                (5.0 - 10.0 * p / 0.6, 0.0)
            } else {
                let s = (p - 0.6) / 0.4;
                (-5.0 + 10.0 * s, 4.0 * (PI * s).sin())
            };
            let (hx, hy) = (mx + hip, up(10));
            let (foot_x, foot_y) = (hx + fx.round() as i32, up(0) - fy.round() as i32);
            let knee = (
                (hx + foot_x) / 2 + if hip > 0 { 1 } else { -1 },
                (hy + foot_y) / 2,
            );
            pen.line((hx, hy), knee, 2, col);
            pen.line(knee, (foot_x, foot_y - 1), 1, col);
            pen.rect(foot_x - 1, foot_y - 1, 2, 2, pal.of(b'k'));
        }
        // The body, chest and rump, neck and head.
        for (cx, cy, rx, ry, col) in [
            (0.0, 15.0, 13.0, 6.0, light),
            (9.0, 14.0, 6.0, 7.0, light),
            (-9.0, 15.0, 6.0, 6.5, light),
        ] {
            pen.blob((mx as f32 + cx, up(0) as f32 - cy), (rx, ry), col);
        }
        // The belly in shade, the back lit.
        pen.rect(mx - 9, up(10), 19, 1, dark);
        pen.rect(mx - 8, up(21), 16, 1, pal.of(b'u'));
        pen.line((mx + 11, up(19)), (mx + 17, up(27)), 3, light);
        pen.line((mx + 16, up(27)), (mx + 22, up(23)), 3, light);
        pen.rect(mx + 22, up(23), 2, 2, dark);
        pen.px(mx + 18, up(27), pal.of(b'k'));
        pen.rect(mx + 15, up(31), 1, 2, dark);
        // The mane stirs, the tail swishes.
        for i in 0..5 {
            let wob = ((self.t * 6.0 + i as f32).sin() * 0.8).round() as i32;
            pen.px(mx + 10 + i * 2 - 1 + wob, up(21 + i * 2), dark);
        }
        let mut tail = (mx - 14, up(19));
        for i in 0..8 {
            let wave =
                ((self.t * 4.0 - i as f32 * 0.6).sin() * (1.0 + i as f32 * 0.3)).round() as i32;
            tail = (mx - 15 - i / 2 + wave / 2, up(19 - i * 2));
            pen.rect(tail.0, tail.1, 2, 2, dark);
        }
        let _ = tail;
        // The rider: sombrero, cananas, a boot in the stirrup, the arm raised.
        let rider_x = mx - RIDER_W / 2 + 1;
        let top = up(20) - RIDER.len() as i32;
        let (cc, boot) = (pal.of(b'c'), pal.of(b'k'));
        pen.line((mx - 1, up(21)), (mx + 2, up(11)), 2, cc);
        pen.rect(mx + 1, up(11), 3, 2, boot);
        pen.sprite((rider_x, top), RIDER, false, |_| 0, |ch| pal.get(ch));
        let shoulder = (mx + 3, top + 11);
        if self.raised() {
            let hand = (mx + 7, top - 3);
            pen.line(shoulder, hand, 1, pal.of(b'M'));
            pen.px(hand.0, hand.1, pal.of(b'F'));
            pen.line(
                (hand.0, hand.1 + 4),
                (hand.0 + 1, hand.1 - 9),
                1,
                pal.of(b'b'),
            );
            pen.line(
                (hand.0, hand.1 - 3),
                (hand.0 + 1, hand.1 - 9),
                1,
                pal.of(b'S'),
            );
        } else {
            let hand = (mx + 9, top + 15);
            pen.line(shoulder, hand, 1, pal.of(b'M'));
            pen.px(hand.0, hand.1, pal.of(b'F'));
            pen.line(
                (hand.0 - 8, hand.1 + 3),
                (hand.0 + 5, hand.1 - 6),
                1,
                pal.of(b'b'),
            );
        }
    }

    /// What they shout, in the terminal's own letters over their heads.
    fn shouts(&self, pen: &mut Pen, pal: &Palette, theme: &Theme, g: i32) {
        let Some((who, words, age)) = self.shout() else {
            return;
        };
        let u = self.u();
        let (cx, top) = if who == 4 {
            ((u - HORSE_AT as f32) as i32 + 4, g - 5 - 46)
        } else {
            let off = MARCHERS[who].3;
            ((u - off as f32 - 7.0) as i32, g - WALKER_H - 4)
        };
        let len = words.chars().count() as i32;
        let (col, row) = pen.st.cell(pen.x(cx), top);
        let rise = (age * 1.2) as i32;
        let k = ((age - 1.2) / 0.6).clamp(0.0, 1.0);
        let color: Color = fade(theme, pal.of(b'o'), k);
        pen.c
            .text_attr(col - len / 2, row - rise, words, color, BOLD);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_art_is_whole() {
        assert!(TOP.iter().all(|r| r.len() as i32 == WALKER_W));
        assert!(SKIRT.iter().all(|r| r.len() as i32 == WALKER_W));
        assert!(
            FEET.iter()
                .all(|f| f.len() == 2 && f.iter().all(|r| r.len() as i32 == WALKER_W))
        );
        assert!(RIDER.iter().all(|r| r.len() as i32 == RIDER_W));
        assert_eq!(MARCHERS.len(), 4);
        assert!(MARCHERS.iter().all(|m| m.3 + WALKER_W <= COLUMN));
    }

    /// The column crosses the screen in both directions within the time, the
    /// four soldaderas and the horseman on the way.
    #[test]
    fn marches_across_in_both_directions() {
        let theme = Theme::classic();
        for seed in 0..6 {
            let mut e = start(120, 36, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(120, 36, Color::Reset);
            let mut most = 0;
            let mut secs = 0.0;
            while !e.done() {
                e.update(0.1);
                secs += 0.1;
                c.clear();
                e.draw(&mut c, &theme);
                most = most.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
            }
            assert!(most > 600, "{most} cells");
            assert!(secs < 29.6, "{secs}");
        }
    }
}
