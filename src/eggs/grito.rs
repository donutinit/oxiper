//! El Grito de Dolores, from the balcony, during the Fiestas Patrias.
//!
//! Original oxiper pixel art: the front of a palace in tezontle-gray stone
//! with a balcony, and above it, in its stone frame, the Dolores bell. A
//! president in the tricolor sash steps out on the balcony, and the bell
//! begins to swing and ring, its clapper striking, arcs of sound leaving it
//! at each stroke. Three times he raises the banderita and calls
//! "¡VIVA MÉXICO!", the letters popping up one by one in verde, blanco and
//! rojo as big as the screen allows, then fading; each time the plaza
//! answers: a crowd (seen from behind, in sombreros, rebozos and braids)
//! throws up its arms and its banderitas, "¡VIVA!" rising over their
//! heads, and tricolor confetti bursts from the balcony, more with every
//! call. The bell rings loudest after the third, then calms, and the plaza
//! empties. Everything is drawn to the screen's size and is gone within 30 s.

use super::patrias::{Palette, fade};
use super::stage::{Stage, advance, hash, text_width, unit};
use super::{Egg, Info};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "grito",
    start,
    fiesta: Kind::Patrias,
    when: super::any_day,
};

/// What he calls, and the same on two lines for a narrow screen.
const CALL: &str = "¡VIVA MÉXICO!";
const CALL_LINES: [&str; 2] = ["¡VIVA", "MÉXICO!"];
/// Seconds at which each call starts; the letters pop in over `POP`, hold
/// for `HOLD` and fade over `FADE`.
const CALLS: [f32; 3] = [4.5, 10.5, 16.5];
const POP: f32 = 0.6;
const HOLD: f32 = 1.5;
const FADE: f32 = 0.6;
/// The crowd answers this long after a call starts, for this long.
const ANSWER_AT: f32 = 1.4;
const ANSWER_FOR: f32 = 2.6;
/// The bell calms down and the plaza empties here, and the egg is over.
const CALM: f32 = 21.0;
const END: f32 = 24.5;
/// Bell strokes per second.
const STROKES: f32 = 1.15;

/// The president, facing us: tricolor sash from shoulder to hip.
const MAN: &[&str] = &[
    "..kkk..", ".kkkkk.", ".kFkFk.", ".kFFFk.", "..FFF..", ".sWFWs.", "ssWGWss", "ssWWGss",
    "ssGWRss", "ssWRsss", "ssRssss", "sssssss",
];
const MAN_W: i32 = 7;
/// The Dolores bell, hanging by its crown.
const BELL: &[&str] = &[
    "...aaa...",
    "..aOOOa..",
    "..aOoOa..",
    ".aOoOOOa.",
    ".aOoOOOa.",
    ".aOoOOOa.",
    "aOoOOOOOa",
    "aOoOOOOOa",
    "aOOOOOOOa",
    "aooooooOa",
];
const BELL_W: i32 = 9;
/// A window of the palace.
const WINDOW: &[&str] = &[
    ".WWW.", "WkkkW", "WkkkW", "WkkkW", "WkkkW", "WkkkW", "WkkkW", "wwwww",
];
/// What a person in the plaza wears on the head (three letters stand for
/// colors picked per person: X the clothes, H a hatband), seen from behind.
const HEADS: [&[&str]; 4] = [
    &["...kkk...", "..kkkkk..", "..kkkkk..", "...FFF..."],
    &["...eee...", "...HHH...", "eeeeeeeee", "...FFF..."],
    &["...kkk...", "..kkRkk..", "..kkkkk..", "...FFF..."],
    &["...XXX...", "..XXXXX..", "..XXXXX..", "...FFF..."],
];
/// The body with its arms down: shirt, then trousers or a skirt (P).
const BODY: &[&str] = &[
    ".XXXXXXX.",
    ".XXXXXXX.",
    ".XXXXXXX.",
    ".XXXXXXX.",
    ".FXXXXXF.",
    "..PPPPP..",
    "..PPPPP..",
    "..PP.PP..",
    "..kk.kk..",
];
/// The same with the arms thrown up (drawn from the head down to the shoulder).
const ARMS_UP: &[&str] = &[
    "F.......F",
    "F.......F",
    "X.......X",
    "X.......X",
    "X.......X",
];
const BODY_UP: &[&str] = &[
    ".XXXXXXX.",
    ".XXXXXXX.",
    ".XXXXXXX.",
    ".XXXXXXX.",
    ".XXXXXXX.",
    "..PPPPP..",
    "..PPPPP..",
    "..PP.PP..",
    "..kk.kk..",
];
const PERSON_W: i32 = 9;
const PERSON_H: i32 = 13;
/// Shirts, hatbands and trousers, picked per person.
const SHIRTS: &[u8] = b"WlrWGRWOeNw";
const BANDS: &[u8] = b"RGOR";
const PANTS: &[u8] = b"KdkzKtg";
/// The crowd stands this many pixels apart.
const PITCH: i32 = 6;

/// Confetti per call (more each time), how long it flies, how hard it falls.
const CONFETTI: [u32; 3] = [26, 40, 60];
const CONFETTI_LIFE: f32 = 3.0;
const GRAVITY: f32 = 20.0;

fn start(_w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Grito {
        t: 0.0,
        seed: rng.u32(..),
    })
}

struct Grito {
    t: f32,
    /// Who is in the crowd, and what they do, comes from this.
    seed: u32,
}

/// Where things stand on a stage, from the ground up.
struct Geo {
    /// The palace's middle column, and the ground (one past the last row).
    cx: i32,
    g: i32,
    /// Two rows of people when there is room, else one.
    two: bool,
    /// The balcony's floor, the roof line and the top of the bell's frame.
    y_p: i32,
    y_f: i32,
    y_top: i32,
    /// Does a flag fly over the frame?
    flag: bool,
    /// How far the crowd stands to each side of the middle.
    half: i32,
}

impl Geo {
    fn new(st: &Stage) -> Geo {
        let g = st.h;
        let two = g >= 62;
        let y_p = g - if two { 21 } else { 16 };
        let y_top = y_p - 14 - 15;
        Geo {
            cx: st.w / 2,
            g,
            two,
            y_p,
            y_f: y_p - 14,
            y_top,
            flag: y_top >= 11,
            half: (st.w / 2 - 4).clamp(8, 84),
        }
    }

    /// The top of everything standing, where the letters may come above.
    fn skyline(&self) -> i32 {
        if self.flag {
            self.y_top - 10
        } else {
            self.y_top
        }
    }
}

/// 0 to 1 as `x` goes from `a` to `b`, eased.
fn ease(x: f32, a: f32, b: f32) -> f32 {
    let k = ((x - a) / (b - a)).clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

impl Grito {
    /// How hard the bell swings, 0 to 1: it starts gently, rings out after
    /// each call, loudest after the last, and calms at the end.
    fn ring(&self) -> f32 {
        let t = self.t;
        let mut loud = 0.0f32;
        for (n, &at) in CALLS.iter().enumerate() {
            let a = at + ANSWER_AT;
            let up = ease(t, a - 0.2, a + 0.5);
            let down = if n + 1 == CALLS.len() {
                1.0
            } else {
                1.0 - ease(t, a + 2.2, a + 4.0)
            };
            loud = loud.max(up * down);
        }
        let amp = 0.5 * ease(t, 0.5, 2.0) + 0.5 * loud;
        amp * (1.0 - ease(t, CALM, END - 0.5))
    }

    /// The bell's swing, in pixels at its lip, and the stroke's phase.
    fn swing(&self) -> f32 {
        (self.t * STROKES * TAU).sin() * 3.0 * self.ring()
    }

    /// The call under way, if any, and the seconds since it started.
    fn calling(&self) -> Option<(usize, f32)> {
        CALLS.iter().enumerate().find_map(|(n, &at)| {
            let age = self.t - at;
            (0.0..POP + HOLD + FADE + 0.2)
                .contains(&age)
                .then_some((n, age))
        })
    }

    /// How many seconds into the crowd's answer it is, if it is answering
    /// (with the number of the call).
    fn answering(&self) -> Option<(usize, f32)> {
        CALLS.iter().enumerate().find_map(|(n, &at)| {
            let age = self.t - at - ANSWER_AT;
            (-0.6..ANSWER_FOR).contains(&age).then_some((n, age))
        })
    }

    /// Does the president have his banderita up?
    fn flag_up(&self) -> bool {
        CALLS
            .iter()
            .any(|&at| (-0.4..POP + HOLD + 0.8).contains(&(self.t - at)))
    }

    /// Are arms up in the plaza for person `i`? They answer together, a
    /// beat apart, and now and then someone waves in between.
    fn raised(&self, i: u32) -> bool {
        if let Some((_, age)) = self.answering() {
            return age > unit(i, self.seed) * 0.5 - 0.3;
        }
        hash(i, (self.t * 0.7) as u32 ^ self.seed) % 100 < 6
    }

    /// The crowd's rise (and fall at the end): 0 is gone, 1 is there.
    fn presence(&self) -> f32 {
        ease(self.t, 0.0, 1.4) * (1.0 - ease(self.t, CALM, END - 0.5))
    }

    /// How the call is written: art pixels to a font pixel, the top of its
    /// first line, its height, and whether it fits on one line.
    fn letters(&self, geo: &Geo, st: &Stage) -> (i32, i32, i32, bool) {
        let s = if st.w >= 170 { 2 } else { 1 };
        let one_line = text_width(CALL, s) + 4 <= st.w;
        let lines = if one_line { 1 } else { 2 };
        let height = (7 * lines + (lines - 1) * 2) * s;
        // In the sky above the palace when there is room, else across its
        // wall, so the bell stays in sight.
        let sky = geo.skyline() - height - 3 * s;
        let y = if sky >= 3 * s { sky } else { geo.y_f + 3 };
        (s, y, height, one_line)
    }
}

impl Egg for Grito {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::patrias(theme);
        let st = Stage::new(c.w(), c.h());
        let geo = Geo::new(&st);
        self.palace(c, &st, &geo, &pal, theme);
        self.crowd(c, &st, &geo, theme);
        self.confetti(c, &st, &geo, &pal);
        self.call(c, &st, &geo, &pal, theme);
    }

    fn done(&self) -> bool {
        self.t > END
    }
}

impl Grito {
    /// The palace, its balcony, the bell and the man.
    fn palace(&self, c: &mut Canvas, st: &Stage, geo: &Geo, pal: &Palette, theme: &Theme) {
        let Geo {
            cx,
            g,
            y_p,
            y_f,
            y_top,
            ..
        } = *geo;
        let fade_in = ease(self.t, 0.0, 1.0) * (1.0 - ease(self.t, CALM, END - 0.5));
        if fade_in < 0.03 {
            return;
        }
        let mut dim;
        let pal = if fade_in < 1.0 {
            dim = Palette::patrias(theme);
            dim.fade_all(theme, 1.0 - fade_in);
            &dim
        } else {
            pal
        };
        let (stone, joint) = (pal.of(b'Z'), pal.of(b'z'));
        // The wall, in courses of stone, with a cornice and its dentils.
        let (fw, x0) = (54, cx - 27);
        st.rect(c, x0, y_f, fw, g - y_f, stone);
        for (n, y) in (y_f + 4..g).step_by(4).enumerate() {
            st.rect(c, x0, y, fw, 1, joint);
            for jx in (4 * (n as i32 % 2)..fw).step_by(8) {
                st.rect(c, x0 + jx, y + 1, 1, 3, joint);
            }
        }
        st.rect(c, x0 - 1, y_f, fw + 2, 2, pal.of(b'W'));
        for i in (0..fw).step_by(2) {
            st.px(c, x0 + i, y_f + 2, pal.of(b'w'));
        }
        // Banderines hung from a cord under the cornice, rippling.
        st.rect(c, x0, y_f + 3, fw, 1, pal.of(b'w'));
        for i in 0..fw / 6 {
            let col = pal.of(b"GWR"[i as usize % 3]);
            let ripple = i32::from(((self.t * 4.0 + i as f32 * 0.9).sin()) > 0.4);
            st.rect(c, x0 + i * 6 + 1, y_f + 4, 4, 1, col);
            st.rect(c, x0 + i * 6 + 2, y_f + 5, 2, 1, col);
            st.px(c, x0 + i * 6 + 2 + ripple, y_f + 6, col);
        }
        // Windows on both sides of the balcony's door.
        for wx in [5, 14, 35, 44] {
            st.sprite(
                c,
                (x0 + wx, y_p - 12),
                WINDOW,
                false,
                |_| 0,
                |ch| pal.get(ch),
            );
        }
        // The door, with its arch.
        st.rect(c, cx - 6, y_p - 15, 13, 15, pal.of(b'W'));
        st.rect(c, cx - 5, y_p - 14, 11, 14, pal.of(b'k'));
        st.px(c, cx - 5, y_p - 14, pal.of(b'W'));
        st.px(c, cx + 5, y_p - 14, pal.of(b'W'));
        self.man(c, st, geo, pal);
        // The balcony: floor, corbels and rail.
        st.rect(c, cx - 12, y_p, 25, 2, pal.of(b'W'));
        st.rect(c, cx - 11, y_p + 2, 23, 1, pal.of(b'w'));
        st.rect(c, cx - 12, y_p - 4, 25, 1, pal.of(b'W'));
        for i in 0..12 {
            st.rect(c, cx - 11 + i * 2, y_p - 3, 1, 3, pal.of(b'w'));
        }
        self.bell(c, st, geo, pal);
        // Over the frame, the flag of the Grito.
        if geo.flag {
            let top = y_top - 10;
            st.rect(c, cx, top, 1, 10, pal.of(b'E'));
            for fx in 0..12 {
                let wave = ((self.t * 5.0 - fx as f32 * 0.7).sin() * 1.3 * fx as f32 / 12.0).round()
                    as i32;
                let col = pal.of(b"GWR"[(fx / 4) as usize]);
                st.rect(c, cx + 1 + fx, top + wave, 1, 7, col);
                if fx / 4 == 1 && (3..9).contains(&fx) {
                    st.rect(c, cx + 1 + fx, top + 2 + wave, 1, 3, pal.of(b'E'));
                }
            }
        }
    }

    /// The man on the balcony, calling.
    fn man(&self, c: &mut Canvas, st: &Stage, geo: &Geo, pal: &Palette) {
        if self.t < 1.2 {
            return;
        }
        let Geo { cx, y_p, .. } = *geo;
        let top = y_p - 12;
        let colors = |ch| pal.get(ch);
        st.sprite(c, (cx - MAN_W / 2, top), MAN, false, |_| 0, colors);
        // His arm: down at his side, or up with the banderita.
        let (sx, sy) = (cx + 3, top + 6);
        if self.flag_up() {
            let (hx, hy) = (cx + 6, top - 3);
            st.line(c, (sx, sy), (hx, hy + 1), 1, pal.of(b's'));
            st.px(c, hx, hy, pal.of(b'F'));
            st.rect(c, hx, hy - 6, 1, 6, pal.of(b'E'));
            for fx in 0..5 {
                let wave = ((self.t * 9.0 - fx as f32).sin() * 0.9).round() as i32;
                let col = pal.of(b"GGWRR"[fx as usize]);
                st.rect(c, hx + 1 + fx, hy - 6 + wave, 1, 3, col);
            }
        } else {
            st.rect(c, sx, sy, 1, 4, pal.of(b's'));
            st.px(c, sx, sy + 4, pal.of(b'F'));
        }
    }

    /// The bell in its frame, swinging, with sound at each stroke.
    fn bell(&self, c: &mut Canvas, st: &Stage, geo: &Geo, pal: &Palette) {
        let Geo { cx, y_f, y_top, .. } = *geo;
        // The frame: pillars, lintel, a dark niche and the beam it hangs from.
        st.rect(c, cx - 8, y_top + 2, 17, y_f - y_top - 2, pal.of(b'k'));
        for px in [cx - 11, cx + 9] {
            st.rect(c, px, y_top + 2, 3, y_f - y_top - 2, pal.of(b'Z'));
            st.rect(c, px, y_top + 2, 1, y_f - y_top - 2, pal.of(b'W'));
        }
        st.rect(c, cx - 12, y_top, 25, 2, pal.of(b'W'));
        st.rect(c, cx - 8, y_top + 2, 17, 2, pal.of(b'E'));
        st.rect(c, cx - 8, y_top + 3, 17, 1, pal.of(b'd'));
        // The bell hangs by its crown: the lower it is, the more it swings.
        let swing = self.swing();
        let rows = BELL.len() as f32;
        let (bx, by) = (cx - BELL_W / 2, y_top + 4);
        st.sprite(
            c,
            (bx, by),
            BELL,
            false,
            |r| (swing * r as f32 / rows).round() as i32,
            |ch| pal.get(ch),
        );
        // The clapper swings against it and strikes at each end.
        let clap = (-swing * 1.5).round() as i32;
        st.rect(c, cx - 1 + clap, by + BELL.len() as i32, 3, 2, pal.of(b'a'));
        st.px(c, cx + clap, by + BELL.len() as i32 + 1, pal.of(b'O'));
        // Sound leaves the bell at the top of each stroke.
        let amp = self.ring();
        if amp > 0.35 && swing.abs() > 2.6 * amp {
            let side = if swing > 0.0 { 1 } else { -1 };
            let sound = pal.of(b'o');
            for arc in 0..3 {
                let x = cx + side * (8 + arc * 3 + 2);
                for (dx, dy) in [(0, 0), (-1, 1), (-1, 2), (0, 3)] {
                    st.px(c, x - side * dx, by + 3 + dy + arc, sound);
                }
            }
        }
    }

    /// The people in the plaza: a back row when there is room, and a front one.
    fn crowd(&self, c: &mut Canvas, st: &Stage, geo: &Geo, theme: &Theme) {
        let here = self.presence();
        if here <= 0.0 {
            return;
        }
        let near = Palette::patrias(theme);
        let mut far = Palette::patrias(theme);
        far.fade_all(theme, 0.28);
        let n = geo.half * 2 / PITCH;
        let rise = ((1.0 - here) * 18.0) as i32;
        for row in (0..if geo.two { 2 } else { 1 }).rev() {
            let (pal, feet) = if row == 1 {
                (&far, geo.g - 5)
            } else {
                (&near, geo.g - 1)
            };
            for k in 0..n {
                let i = k as u32 + 100 * row as u32;
                let jitter = hash(i, self.seed) as i32 % 3 - 1;
                let x =
                    geo.cx - geo.half + k * PITCH + jitter + if row == 1 { PITCH / 2 } else { 0 };
                self.person(c, st, pal, i, (x, feet + 1 - PERSON_H + rise));
            }
        }
    }

    /// One person of the plaza, with the top-left pixel of the art at `at`.
    fn person(&self, c: &mut Canvas, st: &Stage, pal: &Palette, i: u32, (x, y): (i32, i32)) {
        let dice = hash(i, self.seed ^ 0x51ed);
        let shirt = SHIRTS[dice as usize % SHIRTS.len()];
        let band = BANDS[(dice >> 8) as usize % BANDS.len()];
        let pants = PANTS[(dice >> 12) as usize % PANTS.len()];
        let head = HEADS[(dice >> 16) as usize % HEADS.len()];
        let colors = |ch| match ch {
            b'X' => pal.get(shirt),
            b'H' => pal.get(band),
            b'P' => pal.get(pants),
            ch => pal.get(ch),
        };
        let up = self.raised(i);
        // Jumping a little when they shout.
        let hop = if up && (self.t * 7.0 + unit(i, 7) * TAU).sin() > 0.55 {
            1
        } else {
            0
        };
        let y = y - hop;
        st.sprite(c, (x, y), head, false, |_| 0, colors);
        if up {
            st.sprite(c, (x, y), ARMS_UP, false, |_| 0, colors);
            st.sprite(c, (x, y + 4), BODY_UP, false, |_| 0, colors);
            // Some hold a banderita high.
            if dice >> 20 & 3 < 2 {
                let left = dice >> 22 & 1 == 0;
                let hx = if left { x } else { x + PERSON_W - 1 };
                st.rect(c, hx, y - 5, 1, 5, pal.of(b'E'));
                for fx in 0..3 {
                    let wave = ((self.t * 10.0 + i as f32 - fx as f32).sin() * 0.8).round() as i32;
                    let col = pal.of(b"GWR"[fx as usize]);
                    let at = if left { hx - 1 - fx } else { hx + 1 + fx };
                    st.rect(c, at, y - 5 + wave, 1, 3, col);
                }
            }
        } else {
            st.sprite(c, (x, y + 4), BODY, false, |_| 0, colors);
        }
    }

    /// Confetti thrown from the balcony after each call.
    fn confetti(&self, c: &mut Canvas, st: &Stage, geo: &Geo, pal: &Palette) {
        for (n, &at) in CALLS.iter().enumerate() {
            let age = self.t - at - ANSWER_AT;
            if !(0.0..CONFETTI_LIFE).contains(&age) {
                continue;
            }
            for i in 0..CONFETTI[n] {
                let d = hash(i, n as u32 + self.seed);
                let a = unit(i, d) * TAU;
                let speed = 8.0 + unit(d, 3) * 22.0;
                let (vx, vy) = (a.cos() * speed, -6.0 - a.sin().abs() * speed);
                let x = geo.cx as f32 + vx * age + (age * 4.0 + a).sin() * 1.5;
                let y = (geo.y_p - 14) as f32 + vy * age + 0.5 * GRAVITY * age * age;
                if y >= geo.g as f32 {
                    continue;
                }
                let col = pal.of(b"lWr"[(d >> 8) as usize % 3]);
                // They flutter: now wide, now thin.
                let wide = ((age * 8.0 + a) as i32) % 2 == 0;
                st.rect(
                    c,
                    x.round() as i32,
                    y.round() as i32,
                    if wide { 2 } else { 1 },
                    1,
                    col,
                );
            }
        }
    }

    /// The big letters of the call, and the plaza's "¡VIVA!" in answer.
    fn call(&self, c: &mut Canvas, st: &Stage, geo: &Geo, pal: &Palette, theme: &Theme) {
        if let Some((n, age)) = self.calling() {
            let (s, y0, _, one_line) = self.letters(geo, st);
            let ink = pal.of(b'k');
            let lines: &[&str] = if one_line { &[CALL] } else { &CALL_LINES };
            let mut letter = 0;
            for (ln, line) in lines.iter().enumerate() {
                let mut x = geo.cx - text_width(line, s) / 2;
                let y = y0 + ln as i32 * 9 * s;
                for ch in line.chars() {
                    if ch != ' ' {
                        // Each letter pops up a moment after the one before,
                        // and fades the same way.
                        let born = letter as f32 * POP / 13.0;
                        let into = ease(age, born, born + 0.2);
                        let gone = ease(
                            age,
                            POP + HOLD + letter as f32 * FADE / 13.0,
                            POP + HOLD + FADE,
                        );
                        letter += 1;
                        if into > 0.0 && gone < 1.0 {
                            let pop = ((1.0 - into) * 6.0 * s as f32) as i32;
                            let shake = if n == 2 {
                                (hash(letter, (age * 14.0) as u32) % 3) as i32 - 1
                            } else {
                                0
                            };
                            let tri = b"lWr"[(letter as usize + n) % 3];
                            let col: Color = fade(theme, pal.of(tri), gone);
                            let edge = fade(theme, ink, gone);
                            st.glyph(c, (x, y + pop + shake), ch, s, col, Some(edge));
                        }
                    }
                    x += advance(ch) * s;
                }
            }
        }
        // The plaza's answer, in the terminal's own small letters.
        if let Some((n, age)) = self.answering() {
            let spread = geo.half as f32 * 1.8;
            for e in 0..7u32 {
                let at = e as f32 * 0.2;
                let life = age - at;
                if !(0.0..1.6).contains(&life) {
                    continue;
                }
                // Spread along the plaza, a little apart, two heights.
                let d = hash(e + 7 * n as u32, self.seed ^ 0xa11);
                let word = if d.is_multiple_of(3) {
                    "¡VIVA MÉXICO!"
                } else {
                    "¡VIVA!"
                };
                let len = word.chars().count() as i32;
                let slot = (e * 3 % 7) as f32 + unit(d, 5) * 0.4;
                let x = geo.cx + ((slot / 7.0 - 0.5) * spread) as i32;
                let (col, row) = st.cell(x, geo.g - PERSON_H - 3 - (geo.two as i32) * 5);
                let rise = (life * 2.2) as i32 + (e % 2) as i32 * 2;
                let tri = pal.of(b"lWr"[d as usize / 3 % 3]);
                let col_now = fade(theme, tri, ease(life, 1.1, 1.6));
                c.text_attr(col - len / 2, row - rise, word, col_now, BOLD);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_art_is_whole() {
        for (art, w) in [
            (MAN, MAN_W),
            (BELL, BELL_W),
            (WINDOW, 5),
            (BODY, PERSON_W),
            (BODY_UP, PERSON_W),
            (ARMS_UP, PERSON_W),
        ] {
            assert!(art.iter().all(|r| r.len() as i32 == w), "{art:?}");
        }
        assert!(
            HEADS
                .iter()
                .all(|h| h.len() == 4 && h.iter().all(|r| r.len() as i32 == PERSON_W))
        );
        assert_eq!((HEADS[0].len() + BODY.len()) as i32, PERSON_H);
        assert!(
            CALLS
                .windows(2)
                .all(|w| w[1] - w[0] > POP + HOLD + FADE + ANSWER_FOR - 2.0)
        );
        const { assert!(CALLS[2] + ANSWER_AT + ANSWER_FOR < CALM) };
    }

    /// The three calls show their letters, one after another, and are
    /// answered; at the start and the end the screen is bare.
    #[test]
    fn three_calls_and_three_answers() {
        for (cols, rows) in [(80, 24), (200, 60), (120, 40)] {
            let theme = Theme::classic();
            let mut e = Grito { t: 0.0, seed: 3 };
            let mut c = Canvas::new(cols, rows, Color::Reset);
            let (mut calls, mut shouts, mut was) = (0, 0, false);
            for _ in 0..(END * 20.0) as usize {
                e.update(0.05);
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                shouts = shouts.max(text.matches("¡VIVA").count());
                let now = e
                    .calling()
                    .is_some_and(|(_, age)| age > POP && age < POP + HOLD);
                if now && !was {
                    calls += 1;
                    // The letters alone fill plenty of cells.
                    let (st, mut bare) = (
                        Stage::new(cols, rows),
                        Canvas::new(cols, rows, Color::Reset),
                    );
                    let (geo, pal) = (Geo::new(&st), Palette::patrias(&theme));
                    e.call(&mut bare, &st, &geo, &pal, &theme);
                    let cells = bare.cells().iter().filter(|cell| cell.ch != ' ').count();
                    assert!(cells > 40, "{cells} cells of letters at {cols}x{rows}");
                }
                was = now;
            }
            assert_eq!(calls, 3);
            assert!(shouts >= 3, "{shouts}");
            // Over, and nothing left on the screen.
            e.update(0.5);
            c.clear();
            e.draw(&mut c, &theme);
            assert!(e.done() && c.cells().iter().all(|cell| cell.ch == ' '));
        }
    }
}
