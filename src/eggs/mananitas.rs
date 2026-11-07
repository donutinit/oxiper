//! At midnight on the twelfth of December mariachis sing the mañanitas to the
//! Virgen de Guadalupe in front of the Basílica.
//!
//! Since the night of the eleventh the faithful wait at the Basílica de
//! Guadalupe, at the foot of the Tepeyac, and when the clock strikes twelve
//! the mariachis begin: the mañanitas ("Estas son las mañanitas que cantaba
//! el rey David"), La Guadalupana ("Desde el cielo una hermosa mañana"), and
//! cohetes go up over the atrium. This draws it in original oxiper pixel art
//! that grows with the screen. The Basílica rises from the ground, the
//! cantera of its nave and its two bell towers with their green domes, the
//! clocks at twelve, the Virgen in her niche over the portal, and its windows
//! and door light up; veladoras burn on its steps and ramos of roses lie at
//! them. The mariachis walk in from both sides in their black charro suits,
//! silver buttons down the trousers, red bow ties and wide sombreros, and
//! stand either side of the portal facing it: trumpets, violins, a vihuela
//! and a guitarrón, as many as the screen has room for. They play and sing,
//! swaying to the beat, trumpet bells flashing and bows sawing, and notes
//! (♪ ♫ ♩) rise from them while four lines of the songs float up into the sky
//! over the Basílica, one after another. Stars come out, and every little
//! while a cohete climbs on a trail of sparks and bursts into gold, rose and
//! green sparks that fall slowly. Then the mariachis walk off and the Basílica
//! sinks back into the ground. All within 30 s.

use super::blit::hash;
use super::guadalupe::{BASILICA, Palette, Place, fade, fit_down};
use super::{Egg, Info, pixel};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas, DIM};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "mananitas",
    start,
    fiesta: Kind::Navidad,
    when: |fiesta| fiesta.guadalupe(),
};

/// When things happen, in seconds: the Basílica is risen, its lights come on
/// between the two times, the outermost mariachis walk in and each pair after
/// them follows `COME` later, crossing the screen takes `WALK`, the innermost
/// pair walks off first and the others follow `GO` apart, the Basílica sinks
/// and it is over.
const RISEN: f32 = 3.2;
const LIGHTS: (f32, f32) = (2.2, 5.0);
const ENTER: f32 = 0.6;
const COME: f32 = 0.9;
const WALK: f32 = 4.2;
const LEAVE: f32 = 21.4;
const GO: f32 = 0.7;
const SINKING: f32 = 24.8;
const END: f32 = 28.2;
/// One beat of the music, in seconds.
const BEAT: f32 = 0.5;
/// The songs, each with the second it rises at, and how long it stays up.
const SONG: [(f32, &str); 4] = [
    (6.0, "Estas son las mañanitas"),
    (10.0, "que cantaba el rey David"),
    (14.0, "Desde el cielo una hermosa mañana"),
    (18.0, "la Guadalupana bajó al Tepeyac"),
];
const SUNG: f32 = 4.2;
const NOTES: [&str; 3] = ["♪", "♫", "♩"];
/// The cohetes: how many, when the first goes up and how far apart they are,
/// how long one climbs and how long its sparks last.
const ROCKETS: usize = 9;
const FIRST_ROCKET: f32 = 7.6;
const ROCKET_GAP: f32 = 1.75;
const CLIMB: f32 = 1.05;
const SPARKS: f32 = 2.0;

/// How tall the tallest thing is (the Basílica as it is drawn here, with its
/// crosses), in art pixels, how big it is next to a mariachi, and how far
/// apart the mariachis stand and from the middle of the portal.
const HEIGHT: i32 = 50;
const SIZE: f32 = 0.8;
const PITCH: f32 = 19.0;
const GAP: f32 = 13.0;
/// How far a mariachi goes in one step, in art pixels.
const STRIDE: f32 = 12.0;

/// A mariachi's head from the front, in a sombrero de charro trimmed with
/// silver and gold, singing (`o` is the mouth).
const HEAD: &[&str] = &[
    ".....zzz.....",
    "....zzzzz....",
    "....YYYYY....",
    "...zzzzzzz...",
    "izYzzzzzzzYzi",
    "...kKKKKKk...",
    "...KnKKKnK...",
    "...KKKkKKK...",
    "...KKkokKK...",
    "....KKKKK....",
];

#[derive(Clone, Copy, PartialEq)]
enum Role {
    Trumpet,
    Violin,
    Vihuela,
    Guitarron,
}

/// Who plays where: `p` pairs from the portal outward, the left of each pair
/// and its right.
fn role(p: usize, right: bool) -> Role {
    match (p, right) {
        (0, false) | (1, true) => Role::Trumpet,
        (0, true) | (1, false) => Role::Violin,
        (_, false) => Role::Guitarron,
        (_, true) => Role::Vihuela,
    }
}

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Mananitas::new(w, h, rng))
}

struct Mananitas {
    t: f32,
    w: i32,
    h: i32,
    /// Screen pixels to an art pixel, and the screen's width in art pixels.
    k: f32,
    wide: f32,
    /// How many pairs of mariachis the screen has room for.
    pairs: usize,
    seed: u32,
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl Mananitas {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Mananitas {
        let mut rng = fastrand::Rng::with_seed(rng.u64(..));
        let k = fit_down(h, w, HEIGHT, 100.0);
        let wide = w as f32 / k;
        Mananitas {
            t: 0.0,
            w: w as i32,
            h: h as i32,
            k,
            wide,
            pairs: (((wide * 0.5 - GAP - 8.0 + PITCH) / PITCH).floor() as usize).clamp(1, 3),
            seed: rng.u32(..),
        }
    }

    /// How far the Basílica has risen: 0 under the ground, 1 standing.
    fn rise(&self) -> f32 {
        let up = ease_out(self.t / RISEN);
        let down = ((self.t - SINKING) / (END - SINKING)).clamp(0.0, 1.0);
        (up - down * down).clamp(0.0, 1.0)
    }

    /// How lit its windows are.
    fn glow(&self) -> f32 {
        smooth(LIGHTS.0, LIGHTS.1, self.t) * (1.0 - smooth(SINKING, END, self.t))
    }

    /// The mariachis of pair `p` (counted from the portal): how far from their
    /// place they are, in art pixels (outward), and whether they are walking.
    /// They all walk at one pace, the outer pairs first.
    fn away(&self, p: usize) -> (f32, bool) {
        let target = GAP + p as f32 * PITCH;
        let travel = (self.wide * 0.5 + 20.0 - target).max(0.0);
        let pace = (self.wide * 0.5 + 20.0) / WALK;
        let order = (self.pairs - 1 - p) as f32;
        let come = pace * (self.t - (ENTER + COME * order));
        let go = pace * (self.t - (LEAVE + GO * (self.pairs - 1) as f32 - GO * order));
        let off = travel - come.clamp(0.0, travel) + go.clamp(0.0, travel);
        (off, come < travel || go > 0.0)
    }

    /// The song line rising now, if any: its text and how long it has been up.
    fn singing(&self) -> Option<(&'static str, f32)> {
        SONG.iter()
            .find(|(at, _)| (0.0..SUNG).contains(&(self.t - at)))
            .map(|&(at, text)| (text, self.t - at))
    }
}

impl Egg for Mananitas {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::guadalupe(theme);
        let ground = self.h as f32 * 2.0;
        let rise = self.rise();
        self.stars(c, theme, &pal, rise);
        for i in 0..ROCKETS {
            self.rocket(c, theme, &pal, i);
        }
        // The Basílica, sunk below the ground until it rises, with the
        // veladoras and the roses at the foot of its steps.
        let sunk = (1.0 - rise) * (HEIGHT as f32 * self.k + 14.0);
        let at = Place {
            pal: &pal,
            k: self.k,
            s: 1.0,
            ox: self.w as f32 * 0.5,
            ground,
        };
        if rise > 0.0 {
            let big = Place {
                ground: ground + sunk,
                ..at.scaled(SIZE)
            };
            big.basilica(c, 0.0, self.glow());
            big.offerings(c, self.t);
        }
        for rank in (0..self.pairs * 2).rev() {
            let (p, right) = (rank / 2, rank % 2 == 1);
            self.mariachi(c, &at, (p, right), rank);
        }
        self.notes(c, theme, &pal, ground);
        self.song(c, theme, &pal, ground);
    }

    fn done(&self) -> bool {
        self.t > END || self.t > 29.5
    }
}

impl Mananitas {
    /// One mariachi of the pair `p` (counted from the portal), on the left or
    /// right, in his black charro suit, playing and singing.
    fn mariachi(&self, c: &mut Canvas, at: &Place, (p, right): (usize, bool), rank: usize) {
        let target = GAP + p as f32 * PITCH;
        let (off, walking) = self.away(p);
        let dist = target + off;
        if dist > self.wide * 0.5 + 12.0 {
            return;
        }
        // Local x counts toward the portal for both sides, so what one does
        // the other mirrors.
        let sign = if right { -1.0 } else { 1.0 };
        let me = Place {
            s: sign,
            ox: at.ox - sign * dist * at.k,
            ..*at
        };
        let pal = me.pal;
        let who = rank as u32 + self.seed;
        let t = self.t + hash(who, 5) * BEAT;
        let beat = (t / BEAT).fract();
        let step = if walking { off / STRIDE * TAU } else { 0.0 };
        // A little spring in the knees on the beat, and a sway.
        let bob = if walking {
            0.7 * step.sin().abs()
        } else {
            0.8 * (beat * PI).sin().abs()
        };
        let sway = if walking {
            0.0
        } else {
            0.9 * (t * PI / BEAT * 0.5).sin()
        };
        let cx = sway;
        let (hip, shoulder) = (12.0 + bob * 0.5, 20.0 + bob);
        let ink = pal.of(b'z');
        let dark = pal.of(b'n');

        // The legs: black trousers with a line of silver buttons down the
        // side, and boots.
        for (j, side) in [-1.0f32, 1.0].into_iter().enumerate() {
            let lift = if walking {
                (step + j as f32 * PI).sin().max(0.0) * 2.0
            } else {
                0.0
            };
            let foot = (cx + side * 2.6, lift);
            me.limb(
                c,
                (cx + side * 1.8, hip),
                foot,
                3.4,
                if j == 0 { dark } else { ink },
            );
            me.limb(
                c,
                (cx + side * 3.2, hip - 0.5),
                (foot.0 + side * 0.7, foot.1 + 2.0),
                0.7,
                pal.of(b'i'),
            );
            me.block(c, (foot.0 - 2.0, foot.1), (4.0, 1.6), pal.of(b'u'));
        }
        // The jacket, the shirt in a V with the red bow tie, silver buttons
        // and a gold belt.
        me.block(c, (cx - 4.5, hip - 0.5), (9.0, shoulder - hip + 0.5), ink);
        me.block(c, (cx - 5.3, shoulder - 3.0), (10.6, 3.2), ink);
        me.block(c, (cx + 3.4, hip + 0.5), (1.1, shoulder - hip - 3.0), dark);
        me.block(
            c,
            (cx - 1.2, hip + 2.5),
            (2.4, shoulder - hip - 2.5),
            pal.of(b'W'),
        );
        me.block(c, (cx - 1.5, shoulder - 0.8), (3.0, 1.4), pal.of(b'M'));
        for y in [hip + 1.5, hip + 3.5, hip + 5.5] {
            me.block(c, (cx - 3.3, y), (0.9, 0.9), pal.of(b'i'));
            me.block(c, (cx + 2.4, y), (0.9, 0.9), pal.of(b'i'));
        }
        me.block(c, (cx - 4.5, hip - 0.5), (9.0, 1.3), pal.of(b'Y'));
        me.block(c, (cx - 0.8, hip - 0.5), (1.6, 1.3), pal.of(b'Z'));
        // The head: the mouth opens and shuts as he sings.
        let open = (t * 2.0 / BEAT * 0.5).sin() > -0.2;
        me.sprite(
            c,
            HEAD,
            (cx - 6.5, shoulder + 10.5),
            (0.0, 1.0),
            |l| match l {
                b'o' => Some(if open { dark } else { pal.of(b'k') }),
                l => pal.get(l),
            },
        );
        let arms = Arms {
            cx,
            shoulder,
            beat,
            t,
            who,
        };
        match role(p, right) {
            Role::Trumpet => me.trumpet(c, &arms),
            Role::Violin => me.violin(c, &arms),
            Role::Vihuela => me.guitar(c, &arms, false),
            Role::Guitarron => me.guitar(c, &arms, true),
        }
    }

    /// Where the notes of the music come up, for every mariachi, over his
    /// head: they float up and sway, and fade.
    fn notes(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, ground: f32) {
        if !(RISEN..LEAVE + 2.0).contains(&self.t) {
            return;
        }
        let mid = self.w as f32 * 0.5;
        for rank in 0..self.pairs * 2 {
            let (p, right) = (rank / 2, rank % 2 == 1);
            let target = GAP + p as f32 * PITCH;
            let (off, _) = self.away(p);
            if off > 1.0 {
                continue;
            }
            let sign = if right { 1.0 } else { -1.0 };
            let x0 = mid + sign * target * self.k;
            let top = ground - 33.0 * self.k;
            for j in 0..2 {
                let age = ((self.t * 0.6 + j as f32 * 0.5 + hash(rank as u32, 9)) % 1.0).abs();
                let drift = (age * 7.0 + j as f32 * 2.0 + rank as f32).sin() * 2.0;
                let x = x0 + drift + sign * age * 5.0;
                let y = top - age * (14.0 + 6.0 * j as f32) * self.k.max(0.5);
                let color = fade(theme, pal.of(if j == 0 { b'Y' } else { b'r' }), age * age);
                let note = NOTES[(rank + j + (self.t * 0.6) as usize) % NOTES.len()];
                c.text_attr(
                    x.round() as i32,
                    (y / 2.0).floor() as i32,
                    note,
                    color,
                    BOLD,
                );
            }
        }
    }

    /// The line of the song that is up, written across the sky over the
    /// Basílica, rising and fading.
    fn song(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, ground: f32) {
        let Some((text, age)) = self.singing() else {
            return;
        };
        let len = text.chars().count() as i32;
        let sky = ((ground - SIZE * BASILICA.1 * self.k) / 2.0).floor() as i32;
        if self.w < len + 2 || sky < 3 {
            return;
        }
        let come = (age * 2.5).min(1.0);
        let gone = ((age - (SUNG - 0.9)) / 0.9).clamp(0.0, 1.0);
        let row = sky - 2 - (age / SUNG * 2.0) as i32;
        let left = (self.w - len) / 2;
        let color = fade(theme, pal.of(b'Y'), gone.max(1.0 - come));
        c.text_attr(left, row.max(0), text, color, BOLD);
        let note = NOTES[(self.t * 1.3) as usize % NOTES.len()];
        c.text_attr(left - 2, row.max(0), note, color, BOLD);
        c.text_attr(left + len + 1, row.max(0), note, color, BOLD);
    }

    /// A few stars, twinkling, that come out as the night settles.
    fn stars(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, rise: f32) {
        let n = ((self.w * self.h) / 50).clamp(4, 36);
        let sky = (self.h * 2) as f32 * 0.45;
        let appear = smooth(0.4, 2.5, self.t) * (1.0 - smooth(SINKING, END, self.t));
        if appear <= 0.0 || rise <= 0.0 {
            return;
        }
        for i in 0..n as u32 {
            let x = (hash(self.seed, i * 2 + 1) * self.w as f32) as i32;
            let y = (hash(self.seed, i * 2 + 2) * sky / 2.0) as i32;
            let tw = (self.t * (1.1 + hash(i, 3)) + i as f32 * 2.1).sin() * 0.5 + 0.5;
            let color = fade(theme, pal.of(b'Z'), 1.0 - appear * (0.35 + 0.65 * tw));
            if tw > 0.7 {
                c.set_attr(x, y, '✦', color, BOLD);
            } else {
                c.set_attr(x, y, '·', color, DIM);
            }
        }
    }

    /// Cohete `i`: it climbs on a trail of sparks, bursts, and the sparks
    /// open out and fall, fading.
    fn rocket(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, i: usize) {
        let age = self.t - (FIRST_ROCKET + i as f32 * ROCKET_GAP);
        if !(0.0..CLIMB + SPARKS).contains(&age) {
            return;
        }
        let i = i as u32;
        let (px, ph) = (self.w as f32, self.h as f32 * 2.0);
        let x = px * (0.06 + 0.88 * hash(self.seed, 50 + i));
        let top = ph * (0.1 + 0.3 * hash(self.seed, 70 + i));
        let from = ph - 4.0 * self.k;
        let set = *b"YrZHR";
        let hue = set[(i as usize + (self.seed & 3) as usize) % set.len()];
        if age < CLIMB {
            let u = age / CLIMB;
            let head = from + (top - from) * ease_out(u);
            for j in 0..7 {
                let back = j as f32 * 1.6;
                let wob = (self.t * 20.0 + j as f32 * 1.7).sin() * 0.5;
                let color = fade(
                    theme,
                    pal.of(if j < 2 { b'X' } else { b'F' }),
                    j as f32 / 8.0,
                );
                pixel(
                    c,
                    (x + wob).round() as i32,
                    (head + back).round() as i32,
                    color,
                );
            }
        } else {
            let u = (age - CLIMB) / SPARKS;
            let reach = ph * (0.17 + 0.07 * hash(self.seed, 90 + i)) * ease_out(u * 2.2);
            let sparks = 18;
            let turn = hash(self.seed, 110 + i) * TAU;
            for j in 0..sparks {
                let a = turn + j as f32 / sparks as f32 * TAU;
                let r = reach * (1.0 - 0.18 * (j % 2) as f32);
                let fall = 36.0 * u * u * self.k.max(0.5);
                let (sx, sy) = (x + a.cos() * r * 1.6, top + a.sin() * r + fall);
                let shade = if j % 3 == 0 { b'Z' } else { hue };
                let color = fade(theme, pal.of(shade), (u * u).min(1.0));
                pixel(c, sx.round() as i32, sy.round() as i32, color);
                let (tx, ty) = (x + a.cos() * r * 1.35, top + a.sin() * r * 0.85 + fall);
                pixel(
                    c,
                    tx.round() as i32,
                    ty.round() as i32,
                    fade(theme, pal.of(shade), (u * u * 0.6 + 0.35).min(1.0)),
                );
            }
        }
    }
}

/// Where a mariachi's arms are: the middle of his body, the height of his
/// shoulders, where in the beat he is, the time and who he is.
struct Arms {
    cx: f32,
    shoulder: f32,
    beat: f32,
    t: f32,
    who: u32,
}

impl Place<'_> {
    /// An arm in the jacket's sleeve from the shoulder to the hand.
    fn arm(&self, c: &mut Canvas, from: (f32, f32), hand: (f32, f32)) {
        self.limb(c, from, hand, 2.6, self.pal.of(b'z'));
        self.block(
            c,
            (hand.0 - 1.0, hand.1 - 1.0),
            (2.0, 2.0),
            self.pal.of(b'K'),
        );
    }

    /// The trumpet, raised toward the portal, its bell flashing on the beat.
    fn trumpet(&self, c: &mut Canvas, a: &Arms) {
        let pal = self.pal;
        let mouth = (a.cx + 1.6, a.shoulder + 3.0);
        let bell = (a.cx + 11.5, a.shoulder + 5.0 + 0.5 * (a.t * 3.0).sin());
        self.limb(c, mouth, bell, 1.3, pal.of(b'y'));
        self.disc(c, bell, 2.1, pal.of(b'Y'));
        let flash = (a.beat * 2.0).fract() < 0.3;
        self.disc(
            c,
            (bell.0 + 0.4, bell.1),
            1.2,
            pal.of(if flash { b'Z' } else { b'd' }),
        );
        self.arm(
            c,
            (a.cx + 4.5, a.shoulder - 1.0),
            (a.cx + 5.0, a.shoulder + 2.6),
        );
        self.arm(
            c,
            (a.cx - 4.5, a.shoulder - 1.0),
            (a.cx + 8.0, a.shoulder + 3.6),
        );
    }

    /// The violin at his shoulder toward the portal, the bow sawing.
    fn violin(&self, c: &mut Canvas, a: &Arms) {
        let pal = self.pal;
        let saw = (a.t * 5.0 + a.who as f32).sin();
        let body = (a.cx + 4.2, a.shoulder + 1.6);
        self.limb(c, body, (a.cx + 10.5, a.shoulder + 5.5), 0.9, pal.of(b'u'));
        self.disc(c, body, 2.3, pal.of(b'D'));
        self.disc(c, body, 1.0, pal.of(b'u'));
        let bow_hand = (a.cx + 3.0 + saw * 1.8, a.shoulder - 1.5);
        self.limb(
            c,
            (bow_hand.0 - 3.5, bow_hand.1 + 1.5),
            (bow_hand.0 + 5.5, bow_hand.1 - 0.2),
            0.6,
            pal.of(b'Z'),
        );
        self.arm(
            c,
            (a.cx + 4.5, a.shoulder - 1.0),
            (a.cx + 9.2, a.shoulder + 4.6),
        );
        self.arm(c, (a.cx - 4.5, a.shoulder - 1.0), bow_hand);
    }

    /// The vihuela or the great guitarrón, held across the chest, one hand
    /// strumming and the other up the neck.
    fn guitar(&self, c: &mut Canvas, a: &Arms, big: bool) {
        let pal = self.pal;
        let (r, y) = if big {
            (4.8, 13.5)
        } else {
            (3.2, a.shoulder - 4.5)
        };
        let body = (a.cx + 0.5, y);
        let top = (a.cx + 10.5, a.shoulder + if big { 5.5 } else { 3.0 });
        self.limb(
            c,
            (body.0 + 2.0, body.1 + 1.5),
            top,
            if big { 1.6 } else { 1.1 },
            pal.of(b'u'),
        );
        self.disc(c, body, r, pal.of(b'D'));
        self.disc(c, body, r * 0.35, pal.of(b'n'));
        self.block(
            c,
            (body.0 - r * 0.6, body.1 - r - 0.6),
            (r * 1.2, 0.9),
            pal.of(b'u'),
        );
        let strum = (a.t * 7.0).sin() * 1.2;
        self.arm(
            c,
            (a.cx - 4.5, a.shoulder - 1.0),
            (body.0 - 0.5, body.1 + strum),
        );
        self.arm(
            c,
            (a.cx + 4.5, a.shoulder - 1.0),
            (top.0 - 3.0, top.1 - 1.0),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, rgb};

    fn egg(w: u16, h: u16) -> Mananitas {
        Mananitas::new(w, h, &mut fastrand::Rng::with_seed(7))
    }

    fn shows(c: &Canvas, hex: u32) -> bool {
        c.cells()
            .iter()
            .any(|cell| cell.fg == rgb(hex) || cell.bg == rgb(hex))
    }

    #[test]
    fn the_head_is_whole() {
        let w = HEAD[0].len();
        assert!(HEAD.iter().all(|r| r.len() == w));
        let pal = Palette::guadalupe(&Theme::classic());
        for l in HEAD.iter().flat_map(|r| r.bytes()) {
            assert!(
                matches!(l, b'.' | b'o') || pal.get(l).is_some(),
                "{}",
                l as char
            );
        }
    }

    /// The Basílica rises with its lights, stands, and sinks again.
    #[test]
    fn the_basilica_rises_and_sinks() {
        let mut e = egg(100, 30);
        let mut rise = |t: f32| {
            e.t = t;
            (e.rise(), e.glow())
        };
        assert_eq!(rise(0.0), (0.0, 0.0));
        assert!(rise(RISEN).0 > 0.99);
        assert!(rise(1.0).0 < rise(2.0).0);
        assert!(rise(SINKING - 1.0).0 > 0.99 && rise(SINKING - 1.0).1 > 0.99);
        assert!(rise(SINKING + 1.5).0 < rise(SINKING).0);
        assert_eq!(rise(END), (0.0, 0.0));
    }

    /// The mariachis come in from their sides to their places, the outer pairs
    /// first, stay while the song lasts and walk off again.
    #[test]
    fn the_mariachis_walk_in_and_out() {
        for (w, h) in [(100, 30), (120, 36), (200, 60)] {
            let mut e = egg(w, h);
            assert!(e.pairs >= 1);
            for p in 0..e.pairs {
                let travel = e.wide * 0.5 + 20.0 - (GAP + p as f32 * PITCH);
                e.t = 0.0;
                assert_eq!(e.away(p).0, travel, "{w}x{h} {p}");
                e.t = ENTER + COME * (e.pairs - 1) as f32 + WALK + 0.01;
                assert_eq!(e.away(p), (0.0, false), "{w}x{h} {p}");
                e.t = LEAVE - 0.01;
                assert_eq!(e.away(p), (0.0, false), "{w}x{h} {p}");
                e.t = END;
                assert!(e.away(p).0 >= travel - 0.01 && e.away(p).1, "{w}x{h} {p}");
            }
            assert!(ENTER + COME * (e.pairs - 1) as f32 + WALK < LEAVE - 8.0);
            assert!(LEAVE + GO * (e.pairs - 1) as f32 + WALK < END);
        }
    }

    /// The songs follow one another and are written across the sky.
    #[test]
    fn the_songs_are_sung_one_after_another() {
        let theme = Theme::classic();
        let mut e = egg(100, 30);
        let mut c = Canvas::new(100, 30, Color::Reset);
        for (at, text) in SONG {
            e.t = at + 1.0;
            c.clear();
            e.draw(&mut c, &theme);
            assert!(c.to_text().contains(text), "{text}");
        }
        for pair in SONG.windows(2) {
            assert!(pair[0].0 + SUNG <= pair[1].0 + 0.5);
        }
        assert!(SONG[3].0 + SUNG < LEAVE + 2.0);
        e.t = 1.0;
        assert!(e.singing().is_none());
        // A screen too narrow for a line leaves it out, and nothing breaks.
        let mut narrow = egg(24, 20);
        narrow.t = SONG[3].0 + 1.0;
        let mut c = Canvas::new(24, 20, Color::Reset);
        narrow.draw(&mut c, &theme);
        assert!(!c.to_text().contains("Guadalupana"));
    }

    /// Everything on show: the Basílica's stone, domes and gold, the
    /// mariachis' suits, silver, bow ties and instruments, the cohetes' sparks,
    /// the notes; and at the end nothing is left.
    #[test]
    fn the_serenade_comes_dressed_and_goes() {
        let theme = Theme::classic();
        let mut e = egg(120, 40);
        let mut c = Canvas::new(120, 40, Color::Reset);
        let wanted = [
            0xbfa98a, 0x4f8f7a, 0xf2c14e, 0x1c1424, 0xdadde6, 0xc8102e, 0xa0703a, 0xffcc4d,
            0xd7a066,
        ];
        let mut seen = [false; 9];
        let (mut notes, mut stars) = (false, false);
        while !e.done() {
            e.update(0.1);
            c.clear();
            e.draw(&mut c, &theme);
            for (i, hex) in wanted.iter().enumerate() {
                seen[i] |= shows(&c, *hex);
            }
            let text = c.to_text();
            notes |= NOTES.iter().any(|n| text.contains(n));
            stars |= text.contains('✦');
        }
        assert!(seen.iter().all(|&s| s), "{seen:?}");
        assert!(notes && stars);
        assert!(c.cells().iter().all(|cell| cell.ch == ' '));
    }

    /// The cohetes climb and burst: before the first one the sky is only
    /// stars, and while they go up there are sparks high on the screen on
    /// many frames.
    #[test]
    fn the_cohetes_burst() {
        let theme = Theme::classic();
        let mut e = egg(120, 40);
        let mut c = Canvas::new(120, 40, Color::Reset);
        let sparks = |c: &Canvas| {
            c.cells()[..120 * 12]
                .iter()
                .any(|cell| matches!(cell.ch, '▀' | '▄' | '█'))
        };
        e.t = FIRST_ROCKET - 0.5;
        e.draw(&mut c, &theme);
        assert!(!sparks(&c));
        let mut high = 0;
        let mut t = FIRST_ROCKET;
        while t < FIRST_ROCKET + 3.0 * ROCKET_GAP {
            e.t = t;
            c.clear();
            e.draw(&mut c, &theme);
            high += usize::from(sparks(&c));
            t += 0.1;
        }
        assert!(high > 20, "{high}");
    }

    /// A theme other than `classic` paints the serenade with its own colors.
    #[test]
    fn a_theme_gives_its_own_colors() {
        let nord = Theme::builtin("nord").unwrap();
        let mut e = egg(100, 40);
        e.t = 12.0;
        let mut c = Canvas::new(100, 40, Color::Reset);
        e.draw(&mut c, &nord);
        assert!(!shows(&c, 0xbfa98a) && !shows(&c, 0x4f8f7a));
        assert!(c.cells().iter().any(|cell| cell.ch != ' '));
    }
}
