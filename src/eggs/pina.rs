//! The Flor de Piña: a line of women dances across the screen during the
//! Guelaguetza, a pineapple on each one's shoulder.
//!
//! The Flor de Piña is the dance of the women of Tuxtepec, in the Papaloapan
//! region of Oaxaca, who carry a pineapple on the shoulder, the fruit their
//! land is known for. Original oxiper pixel art: each woman in a long white
//! huipil embroidered with flowers in grana, rosa, morado, turquesa and
//! verde, a crown of flowers in her hair and her braids tied with ribbons,
//! and a wide skirt with its band and the scalloped lace of the enagua, a
//! golden piña with its crown of green leaves on her shoulder, one hand
//! holding it up and the other lifting her skirt. They go in a line, each in
//! her own colors, stepping to the beat with a sway of the hem against the
//! shoulders and the pineapple bobbing; now and then one after another turns
//! on the spot, the skirt flying out round her and its pleats and lace
//! turning, her braids and her back showing as she goes round, the pineapple
//! staying on her shoulder. The line grows and the women scale with the
//! screen, and it crosses from either side within 30 s.

use super::blit::{Xf, blit, fit, whole};
use super::guelaguetza::{BODY, LOOKS, Palette, Stance, WIDTH, arm, woman};
use super::{Egg, Info};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "pina",
    start,
    fiesta: Kind::Guelaguetza,
    when: super::any_day,
};

/// A piña: the crown of leaves, the crosshatch of its golden skin.
const PINA: &[&str] = &[
    ".....l.....",
    ".v...V...v.",
    ".VV.lVl.VV.",
    "..VVVVVVV..",
    "..vVVlVVv..",
    "...VVVVV...",
    "....vVv....",
    "...vvvvv...",
    "...AaAoA...",
    "..AaAaAaA..",
    ".AaAoAaAoA.",
    ".aAaAaAaAa.",
    ".AoAaAoAaA.",
    ".aAaAaAaAa.",
    ".AaAoAaAoA.",
    ".aAaAaAaAa.",
    "..oAaAoAa..",
    "...aAaAa...",
];

/// Her skirt's length, and how high a pineapple reaches above her: art pixels.
const SKIRT: i32 = 20;
const PINA_ABOVE: f32 = 6.0;
/// The room a woman takes, her pineapple and her skirt flying out included.
const ROOM: f32 = 30.0;

/// Steps per second, seconds a turn takes and how often she turns, and
/// seconds the line takes to cross.
const STEPS: f32 = 1.6;
const TURN: f32 = 1.7;
const EVERY: f32 = 6.0;
const CROSSING: f32 = 20.0;

/// For a screen `w` columns by `h` rows: the scale of the art, the columns
/// each woman takes, and how many women there are.
fn layout(w: u16, h: u16) -> (f32, i32, usize) {
    // At least three women have to fit across.
    let k = fit(h, BODY + SKIRT + PINA_ABOVE as i32 + 2, 0.8).min(whole(w as f32 / (3.0 * ROOM)));
    let cell = (ROOM * k).round() as i32;
    (k, cell, (w as i32 / cell.max(1)).clamp(3, 7) as usize)
}

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let mut rng = fastrand::Rng::with_seed(rng.u64(..));
    let (k, cell, n) = layout(w, h);
    let first = rng.usize(..LOOKS.len());
    let line = (0..n)
        .map(|i| Dancer {
            look: (first + i * 5) % LOOKS.len(),
            beat: rng.f32() * 0.25,
            // Each turns a little after the one in front of her.
            turns: 1.2 + 0.5 * i as f32,
        })
        .collect();
    let span = n as i32 * cell;
    Box::new(Line {
        t: 0.0,
        dist: 0.0,
        speed: ((w as i32 + span) as f32 / CROSSING).max(5.0),
        left: rng.bool(),
        w: w as i32,
        k,
        cell,
        line,
    })
}

struct Dancer {
    /// Which of the looks she has.
    look: usize,
    /// How far behind the beat she is, in seconds.
    beat: f32,
    /// When she first turns.
    turns: f32,
}

struct Line {
    t: f32,
    /// Columns walked.
    dist: f32,
    speed: f32,
    /// Going right to left?
    left: bool,
    w: i32,
    k: f32,
    cell: i32,
    line: Vec<Dancer>,
}

impl Line {
    /// The column the line's back is at.
    fn origin(&self) -> i32 {
        let span = self.cell * self.line.len() as i32;
        let d = self.dist as i32;
        if self.left { self.w - d } else { d - span }
    }

    /// The left column of woman `i`'s room (0 leads).
    fn x(&self, i: usize) -> i32 {
        let slot = if self.left {
            i
        } else {
            self.line.len() - 1 - i
        } as i32;
        self.origin() + slot * self.cell
    }
}

impl Egg for Line {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::guelaguetza(theme);
        let k = self.k;
        let ground = c.h() as i32 * 2;
        let feet = ((BODY + SKIRT) as f32 * k).round() as i32;
        for (i, wm) in self.line.iter().enumerate() {
            let room = self.x(i);
            if room + self.cell + ((4.0 * k) as i32) < 0 || room - (4.0 * k) as i32 > self.w {
                continue;
            }
            let look = &LOOKS[wm.look];
            let beat = (self.t - wm.beat) * STEPS * TAU;
            // The hem swings one way as the shoulders lean the other, and
            // she rises a little with each step.
            let sway = beat.sin() * 3.0;
            let bob = (0.5 - 0.5 * (beat * 2.0).cos()).round();
            // Her turn: a whole one, every few seconds, in one smooth swing.
            let u = (self.t - wm.turns).rem_euclid(EVERY) / TURN;
            let (turn, flare) = if self.t >= wm.turns && u < 1.0 {
                let s = u * u * (3.0 - 2.0 * u);
                (TAU * s, (PI * u).sin())
            } else {
                (0.0, 0.0)
            };
            let st = Stance {
                at: (
                    room + ((ROOM - WIDTH as f32) * 0.5 * k).round() as i32,
                    ground - feet - (bob * k) as i32,
                ),
                k,
                turn,
                flare,
                sway: sway * (1.0 - 0.5 * flare),
                skirt: SKIRT,
                feet: [(beat.sin()).max(0.0) * 1.5, (-beat.sin()).max(0.0) * 1.5],
            };
            let cos = turn.cos();
            // The piña on her right shoulder (the left of the screen, as she
            // faces us), tilting with the step and riding round with her.
            let hold = st.point(-8.5, 11.0);
            let tilt = 0.1 * beat.sin() + 0.1 * (sway / 3.0);
            let pina = Xf {
                pivot: (5.5, 17.0),
                at: (hold.0, hold.1 - bob.min(1.0) * k * 0.5),
                scale: (k, k),
                rot: tilt,
                flip: false,
            };
            let draw_pina = |c: &mut Canvas| blit(c, PINA, pina, |_| 0, |l| pal.get(l));
            if cos < 0.0 {
                draw_pina(c);
            }
            woman(c, &pal, look, &st);
            // Her right hand holds the pineapple up; her left lifts her skirt.
            arm(
                c,
                &pal,
                look,
                st.point(-7.5, 12.5),
                pina.map(11.0, (5.5, 16.0)),
                k,
            );
            let hem = st.point(11.5, 30.0);
            arm(
                c,
                &pal,
                look,
                st.point(7.5, 12.5),
                (hem.0 + st.sway * 0.4 * k, hem.1 - 2.0 * k * (1.0 + flare)),
                k,
            );
            if cos >= 0.0 {
                draw_pina(c);
            }
        }
    }

    fn done(&self) -> bool {
        let span = self.cell * self.line.len() as i32;
        let gone = if self.left {
            self.origin() + span < 0
        } else {
            self.origin() > self.w
        };
        gone || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        assert!(PINA.iter().all(|r| r.len() == PINA[0].len()));
        let pal = Palette::guelaguetza(&Theme::classic());
        for &b in PINA.iter().flat_map(|r| r.as_bytes()) {
            assert!(b == b'.' || pal.get(b).is_some(), "{}", b as char);
        }
    }

    /// The line grows with the width of the screen and the women with its
    /// height (but never so big that three do not fit across).
    #[test]
    fn the_line_scales_with_the_screen() {
        let (small, big) = (layout(80, 12), layout(400, 120));
        assert!(small.0 < big.0, "{small:?} {big:?}");
        assert!(layout(80, 40).2 < layout(240, 40).2);
        for (w, h) in [
            (1, 1),
            (40, 12),
            (80, 24),
            (100, 150),
            (400, 120),
            (500, 150),
        ] {
            let (k, cell, n) = layout(w, h);
            assert!((3..=7).contains(&n), "{w}x{h}: {n}");
            assert!(cell as f32 <= ROOM * k + 1.0);
        }
        assert!(layout(100, 150).0 <= 1.0);
    }

    /// They cross from one side to the other, standing on the bottom row,
    /// and each turns: the skirts fly out, which shows as a wider line.
    #[test]
    fn dance_across_and_turn() {
        let theme = Theme::classic();
        for seed in 0..4 {
            let mut e = start(120, 40, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(120, 40, Color::Reset);
            let (mut secs, mut widths, mut seen) = (0.0, Vec::new(), 0);
            while !e.done() {
                e.update(0.05);
                secs += 0.05;
                c.clear();
                e.draw(&mut c, &theme);
                // The width of the screen the bottom rows paint (the skirts).
                let row: String = c.to_text().lines().nth(34).unwrap_or("").to_string();
                let n = row.chars().filter(|ch| *ch != ' ').count();
                if n > 20 {
                    seen += 1;
                    widths.push(n);
                }
            }
            assert!(secs < 29.0, "{secs}");
            assert!(seen > 100, "{seen}");
            assert!(widths.iter().min() != widths.iter().max(), "nothing moved");
        }
    }
}
