//! On the Jueves de Corpus the voladores de Papantla fly: four men drop from
//! the top of a tall pole and circle down, head first, on their ropes.
//!
//! The Danza de los Voladores is a sacred ritual of the Totonac people of
//! Veracruz, and on the days of Corpus Christi Papantla holds its great
//! festival around it. A trunk of thirty meters is raised; the caporal (the
//! captain) stands on a tiny platform at its top, plays a small flute with one
//! hand and beats a drum with the other, and turns to greet the four
//! directions, while four men sit on the frame below, in red trousers and white
//! shirts, with tall headdresses hung with mirrors and ribbons of every color,
//! their ropes wound round the pole. They let themselves fall backwards and the
//! frame turns, unwinding the ropes: each of the four circles the pole thirteen
//! times, which with the caporal's makes the fifty-two years of the old
//! calendar, and lands on his feet. This draws it in original oxiper art, in
//! pixels that grow with the screen. The pole rises from the ground with its
//! frame, the ropes coiled on it and the caporal at the top; he plays, and
//! notes float up from his flute, while the four sit still. Then they drop one
//! after another, turning over until they hang head down with their arms
//! spread, the ribbons of their headdresses streaming, and circle the pole in
//! thirteen turns, wider as the ropes unwind, passing behind it and in front
//! of it, the coils on the pole growing thinner, and come down, turning upright
//! just above the ground and landing in a row with their arms raised, while the
//! caporal plays on. The pole then sinks back. All within 30 s.

use super::blit::{block, line};
use super::corpus::{Palette, fade};
use super::pen::fill;
use super::{Egg, Info};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "voladores",
    start,
    fiesta: Kind::Corpus,
    when: super::any_day,
};

/// When things happen, in seconds: the pole is risen, the first flyer lets
/// go, the others follow `DELAY` apart, each takes `FLIGHT` from letting go to
/// landing, the pole sinks and it is over.
const RISEN: f32 = 3.0;
const FALL: f32 = 7.5;
const DELAY: f32 = 0.4;
const FLIGHT: f32 = 14.0;
const SINKING: f32 = 24.6;
const END: f32 = 27.4;
/// How many times each flyer circles the pole, and how long (in seconds) he
/// takes to turn over once he lets go.
const TURNS: f32 = 13.0;
const OVER: f32 = 1.1;
/// The ribbons of the headdresses, in every bright color.
const RIBBONS: [u8; 7] = *b"PYQGVOR";
const NOTES: [&str; 3] = ["♪", "♫", "♩"];

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Voladores::new(w, h, rng))
}

struct Voladores {
    t: f32,
    w: i32,
    h: i32,
    /// Where the pole stands across the screen, as a share of its width.
    at: f32,
    /// Mixes the colors of the ribbons.
    seed: usize,
}

/// The sizes of things on this screen, in pixels (half rows): everything is
/// drawn in units of `u`, a man being about ten of them from foot to hat.
struct Geo {
    u: f32,
    /// The pole's axis (a column), the row of its top, of its frame and of the
    /// ground.
    x: f32,
    top: f32,
    frame: f32,
    ground: f32,
    /// How far everything has sunk below where it stands (while the pole
    /// rises and sinks), in pixels.
    sunk: f32,
    /// How far from the pole the flyers sit, and how wide they circle once
    /// they fly.
    seat: f32,
    reach: f32,
    /// How far apart they land.
    step: f32,
}

/// How a person holds their arms.
#[derive(Clone, Copy)]
enum Arms {
    /// Out to the sides `reach` units, `lift` units along the body toward the
    /// head at the hands (negative: toward the feet): at rest, flying, or
    /// raised in greeting.
    Spread { lift: f32, reach: f32 },
    /// The caporal: the flute at his mouth to one side (`1` right, `-1`
    /// left), the other hand on the drum, which a beat is hitting.
    Flute(f32, bool),
}

/// Everything that tells a person from another and one moment from another.
struct Person {
    /// The waist, in pixels, and the angle of the body from there up to the
    /// head: 0 upright, `PI` head down.
    waist: (f32, f32),
    ang: f32,
    arms: Arms,
    /// Which colors of ribbons, and how the ribbons wave.
    seed: usize,
    t: f32,
}

/// How a flyer is at some moment: his waist, how far round the pole he is
/// (`z` over 0 is in front of it), the body's angle, his arms, and where his
/// rope leaves the pole until he unties it.
struct Flight {
    x: f32,
    y: f32,
    z: f32,
    ang: f32,
    arms: Arms,
    tie: Option<(f32, f32)>,
}

fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

impl Voladores {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Voladores {
        let mut rng = fastrand::Rng::with_seed(rng.u64(..));
        Voladores {
            t: 0.0,
            w: w as i32,
            h: h as i32,
            at: 0.4 + 0.2 * rng.f32(),
            seed: rng.u32(..) as usize,
        }
    }

    fn geo(&self) -> Geo {
        let px = self.h as f32 * 2.0;
        let u = (px * 0.95 / 50.0)
            .clamp(0.45, 9.0)
            .min(self.w as f32 / 18.0);
        let up = ease_out(self.t / RISEN);
        let down = ((self.t - SINKING) / (END - SINKING)).clamp(0.0, 1.0);
        let top = px - 34.0 * u;
        Geo {
            u,
            x: (self.w as f32 * self.at).round(),
            top,
            frame: top + 4.0 * u,
            ground: px,
            sunk: (1.0 - (up - down * down).clamp(0.0, 1.0)) * (px * 1.1),
            seat: 7.0 * u,
            step: (7.0 * u).min(self.w as f32 * 0.8 / 3.0),
            reach: (8.0 * u).min(self.w as f32 * 0.42),
        }
    }

    /// How many turns the frame has made after `tau` (0 to 1) of a flight:
    /// steady, but a little quicker in the middle.
    fn turns(tau: f32) -> f32 {
        TURNS * (0.8 * tau + 0.2 * smooth(0.0, 1.0, tau))
    }

    /// How long the coils of rope on the pole are, in pixels: they run down
    /// as the flyers do.
    fn coil(g: &Geo, t: f32) -> f32 {
        let mean = (0..4)
            .map(|i| ((t - (FALL + i as f32 * DELAY)) / FLIGHT).clamp(0.0, 1.0))
            .sum::<f32>()
            / 4.0;
        13.0 * g.u * (1.0 - mean)
    }

    /// Flyer `i` (0 to 3): seated until he lets go, then circling down.
    fn flight(&self, g: &Geo, i: usize) -> Flight {
        let u = g.u;
        let since = self.t - (FALL + i as f32 * DELAY);
        let tau = (since / FLIGHT).clamp(0.0, 1.0);
        // The four sit on the corners of the frame, seen from a little off.
        let theta = 0.35 + i as f32 * PI / 2.0 + TAU * Self::turns(tau);
        // He turns over, flies out wider, comes down and lands upright.
        let over = smooth(0.0, OVER, since);
        let landing = smooth(0.9, 1.0, tau);
        let wide = lerp(g.seat, g.reach, ease_out(tau * 4.0));
        let (c, s) = (theta.cos(), theta.sin());
        let row = g.x + (i as f32 - 1.5) * g.step;
        let x = lerp(g.x + wide * c, row, landing);
        let z = wide * s * (1.0 - landing);
        let flying = lerp(
            g.frame - 1.0 * u,
            g.ground - 12.0 * u,
            smooth(0.0, 0.9, tau),
        );
        let y = lerp(flying, g.ground - 5.0 * u, landing) + z * 0.1;
        let lean = 0.5 * c * over * (1.0 - landing) * (wide / g.reach.max(1.0));
        // Arms down at rest, out in flight, up in greeting.
        let (lift, reach) = (lerp(-2.2, 1.6, over), lerp(0.9, 2.7, over));
        let (lift, reach) = (lerp(lift, 3.0, landing), lerp(reach, 2.4, landing));
        Flight {
            x,
            y,
            z,
            ang: PI * over * (1.0 - landing) - lean,
            arms: Arms::Spread { lift, reach },
            tie: (tau < 0.9 && since > 0.0)
                .then(|| (g.x + 1.3 * u * c, g.frame + 1.0 * u + Self::coil(g, self.t))),
        }
    }

    /// The caporal's waist, on his platform at the top of the pole.
    fn caporal(&self, g: &Geo) -> (f32, f32) {
        let sway = 0.3 * g.u * (self.t * 2.3).sin();
        let bob = 0.25 * g.u * (self.t * 4.6).sin().abs();
        (g.x + sway, g.top - 4.0 * g.u - bob + g.sunk)
    }
}

impl Egg for Voladores {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::corpus(theme);
        let g = self.geo();
        if g.sunk >= self.h as f32 * 2.0 + 4.0 * g.u {
            return;
        }
        let flights: [Flight; 4] = std::array::from_fn(|i| self.flight(&g, i));
        // The ones behind the pole first, then the pole, then the caporal and
        // the ones in front of it.
        for (i, f) in flights.iter().enumerate().filter(|(_, f)| f.z < 0.0) {
            self.flyer(c, &pal, &g, i, f);
        }
        self.pole(c, &pal, &g);
        self.coils(c, &pal, &g);
        let waist = self.caporal(&g);
        // He turns to each of the four directions in turn: his flute to one
        // side, then the other.
        let side = if (self.t / 2.4) as i32 % 2 == 0 {
            1.0
        } else {
            -1.0
        };
        let beat = (self.t * 4.4).fract() < 0.25;
        let caporal = Person {
            waist,
            ang: 0.05 * (self.t * 2.3).sin(),
            arms: Arms::Flute(side, beat),
            seed: self.seed,
            t: self.t,
        };
        self.person(c, &pal, &g, &caporal);
        for (i, f) in flights.iter().enumerate().filter(|(_, f)| f.z >= 0.0) {
            self.flyer(c, &pal, &g, i, f);
        }
        self.notes(c, theme, &pal, &g, (waist, side));
    }

    fn done(&self) -> bool {
        self.t > END || self.t > 29.5
    }
}

impl Voladores {
    /// The trunk, tapered, of bare wood lit on one side, with a few knots; the
    /// platform on its top and the frame under it, with its struts; a few
    /// stones at its foot.
    fn pole(&self, c: &mut Canvas, pal: &Palette, g: &Geo) {
        let u = g.u;
        let (pale, shade, dark) = (pal.of(b'U'), pal.of(b'u'), pal.of(b'd'));
        for y in (g.top + g.sunk).floor() as i32..=(g.ground + g.sunk) as i32 {
            let along = ((y as f32 - g.sunk - g.top) / (g.ground - g.top)).clamp(0.0, 1.0);
            let half = lerp(1.15, 1.7, along) * u;
            let (x0, x1) = ((g.x - half).round() as i32, (g.x + half).round() as i32);
            let width = (x1 - x0).max(2);
            let knot = (y * 7 + 3) % 17 == 0 && y % 3 != 0;
            for x in x0..x0 + width {
                let k = (x - x0) as f32 / width as f32;
                block(
                    c,
                    x,
                    y,
                    1,
                    if k > 0.68 || knot && k < 0.6 {
                        shade
                    } else {
                        pale
                    },
                );
            }
        }
        let (top, frame) = (g.top + g.sunk, g.frame + g.sunk);
        rect(
            c,
            (g.x - 2.0 * u, top - 0.5 * u),
            (4.0 * u, 0.8 * u),
            pal.of(b'D'),
        );
        let thick = u.max(1.0);
        rect(c, (g.x - 7.5 * u, frame), (15.0 * u, thick), pal.of(b'D'));
        rect(
            c,
            (g.x - 6.5 * u, frame + thick * 0.6),
            (13.0 * u, thick * 0.4),
            dark,
        );
        for side in [-1.0f32, 1.0] {
            let strut = ((0.7 * u).round() as i32).max(1);
            let (from, to) = ((7.0 * u, thick), (1.4 * u, 3.6 * u));
            let (from, to) = (
                (g.x + side * from.0, frame + from.1),
                (g.x + side * to.0, frame + to.1),
            );
            line(c, from, to, strut, dark);
        }
        for (dx, w) in [(-3.0, 1.6), (2.6, 2.0), (-1.2, 1.0)] {
            rect(
                c,
                (g.x + dx * u, g.ground + g.sunk - 0.9 * u),
                (w * u, 0.9 * u),
                shade,
            );
        }
    }

    /// The coils of the ropes round the pole under the frame: bands of rope
    /// slanting across it, running down as the ropes unwind.
    fn coils(&self, c: &mut Canvas, pal: &Palette, g: &Geo) {
        let len = Self::coil(g, self.t);
        if len < 1.0 {
            return;
        }
        let u = g.u;
        let (rope, dark) = (pal.of(b'E'), pal.of(b'D'));
        let y0 = g.frame + 1.0 * u + g.sunk;
        let spin = self.t * 6.0;
        for y in y0.round() as i32..(y0 + len).round() as i32 {
            let half = (lerp(1.2, 1.4, (y as f32 - y0) / (13.0 * u)) * u + 0.5).round();
            for x in (g.x - half) as i32..=(g.x + half) as i32 {
                let band = (y as f32 * 0.9 + (x as f32 - g.x) * 0.7 / u.max(0.5) - spin) % 3.0;
                block(c, x, y, 1, if band.abs() < 1.6 { rope } else { dark });
            }
        }
    }

    /// A flyer with his rope: the rope first, then the man.
    fn flyer(&self, c: &mut Canvas, pal: &Palette, g: &Geo, i: usize, f: &Flight) {
        if let Some((tx, ty)) = f.tie {
            let thick = ((0.4 * g.u).round() as i32).max(1);
            line(
                c,
                (tx, ty + g.sunk),
                (f.x, f.y + g.sunk),
                thick,
                pal.of(b'E'),
            );
        }
        let flyer = Person {
            waist: (f.x, f.y + g.sunk),
            ang: f.ang,
            arms: f.arms,
            seed: self.seed + i * 3,
            t: self.t + i as f32 * 0.7,
        };
        self.person(c, pal, g, &flyer);
    }

    /// A man in red trousers and a white shirt under a cape, with a headdress of
    /// mirrors and ribbons, drawn from the waist: the legs along the body away
    /// from the head, the shirt, the arms, the head and what it wears.
    fn person(&self, c: &mut Canvas, pal: &Palette, g: &Geo, p: &Person) {
        let u = g.u;
        let (sin, cos) = p.ang.sin_cos();
        // The point `along` the body from the waist toward the head and `side`
        // across it, in units.
        let at = |along: f32, side: f32| {
            (
                p.waist.0 + (sin * along + cos * side) * u,
                p.waist.1 + (-cos * along + sin * side) * u,
            )
        };
        // A limb of `width` units from one point to another, rounded at its
        // ends; a band is cut square.
        let limb = |c: &mut Canvas, a: (f32, f32), b: (f32, f32), width: f32, color: u8| {
            bar(
                c,
                at(a.0, a.1),
                at(b.0, b.1),
                width * u,
                true,
                pal.of(color),
            );
        };
        let band = |c: &mut Canvas, a: (f32, f32), b: (f32, f32), width: f32, color: u8| {
            bar(
                c,
                at(a.0, a.1),
                at(b.0, b.1),
                width * u,
                false,
                pal.of(color),
            );
        };

        // The legs, in red with a white wrap at the ankle, and the bare feet.
        for side in [-0.8, 0.8] {
            limb(c, (0.0, side * 0.6), (-3.6, side * 1.1), 1.5, b'R');
            band(c, (-3.4, side * 1.1), (-3.9, side * 1.15), 1.2, b'W');
            limb(c, (-4.0, side * 1.15), (-4.6, side * 1.2), 1.0, b'K');
        }
        // The shirt with its shade on one side, a red sash at the waist and
        // the cape over the shoulders, embroidered along its edge.
        band(c, (0.2, 0.0), (3.2, 0.0), 2.4, b'W');
        band(c, (0.2, 0.9), (3.0, 0.9), 0.6, b'w');
        band(c, (-0.1, 0.0), (0.8, 0.0), 2.6, b'r');
        band(c, (3.0, -2.0), (3.0, 2.0), 1.0, b'R');
        band(c, (2.45, -2.0), (2.45, 2.0), 0.45, b'Y');
        // The arms.
        match p.arms {
            Arms::Spread { lift, reach } => {
                for side in [-1.0f32, 1.0] {
                    let hand = (2.8 + lift, side * (1.1 + reach));
                    limb(c, (2.8, side * 1.1), hand, 1.2, b'W');
                    limb(c, (hand.0, hand.1), (hand.0, hand.1), 1.0, b'K');
                }
            }
            Arms::Flute(side, beat) => {
                // The flute at one side of his mouth, his hand on it; the
                // other hand holds a stick over the drum at his hip.
                limb(c, (2.8, side), (4.5, 0.4 * side), 1.2, b'W');
                limb(c, (4.5, 0.4 * side), (4.1, 4.8 * side), 0.7, b'Y');
                limb(c, (4.1, 4.8 * side), (4.1, 4.8 * side), 0.9, b'F');
                limb(c, (2.8, -side), (1.9, -2.1 * side), 1.2, b'W');
                let drum = at(1.2, -2.7 * side);
                disc(c, drum, 1.5 * u, pal.of(b'R'));
                disc(c, drum, 1.05 * u, pal.of(if beat { b'F' } else { b'D' }));
            }
        }
        // The neck and head, and the headdress: a tall cone in bands of red
        // and yellow with mirrors, and ribbons streaming from its top.
        limb(c, (3.0, 0.0), (4.0, 0.0), 1.0, b'K');
        disc(c, at(4.6, 0.0), 1.2 * u, pal.of(b'K'));
        for (from, to, width, color) in [
            (5.4, 6.4, 2.8, b'R'),
            (6.4, 7.2, 2.3, b'Y'),
            (7.2, 8.2, 1.6, b'R'),
            (8.2, 9.0, 0.9, b'Y'),
        ] {
            band(c, (from, 0.0), (to, 0.0), width, color);
        }
        for (along, side) in [(5.9, -0.7), (5.9, 0.7), (6.8, 0.0)] {
            limb(c, (along, side), (along, side), 0.6, b'S');
        }
        for j in 0..5usize {
            let color = RIBBONS[(p.seed + j * 3) % RIBBONS.len()];
            let fan = (j as f32 - 2.0) * 0.55;
            let wave = |s: f32| (p.t * 5.0 + j as f32 * 1.7 + s * 3.0).sin() * 0.5 * s;
            let mut prev = (8.4, fan * 0.3);
            for step in 1..=5 {
                let s = step as f32 / 5.0;
                let next = (8.4 + 2.2 * s, fan * (0.3 + 1.8 * s) + wave(s));
                limb(c, prev, next, 0.6, color);
                prev = next;
            }
        }
    }

    /// The notes of the flute, floating up and away to the side it is played to.
    fn notes(
        &self,
        c: &mut Canvas,
        theme: &Theme,
        pal: &Palette,
        g: &Geo,
        (waist, side): ((f32, f32), f32),
    ) {
        if !(RISEN..FALL + FLIGHT + 1.0).contains(&self.t) {
            return;
        }
        let u = g.u;
        let mouth = (waist.0 + 3.0 * u * side, waist.1 - 4.0 * u);
        for j in 0..4 {
            let age = ((self.t * 0.7 + j as f32 * 0.7) % 2.8) / 2.8;
            let drift = (age * 9.0 + j as f32).sin() * 1.5 * u;
            let x = mouth.0 + side * (2.0 + age * 9.0) * u + drift;
            let y = mouth.1 - age * 8.0 * u;
            let color = fade(theme, pal.of(b'Y'), age * age);
            let note = NOTES[(j + (self.t * 0.7) as usize) % NOTES.len()];
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

/// A filled rectangle of pixels, its top-left corner at `at`.
fn rect(c: &mut Canvas, at: (f32, f32), (w, h): (f32, f32), color: Color) {
    let (x0, y0) = (at.0.round() as i32, at.1.round() as i32);
    let x1 = ((at.0 + w).round() as i32).max(x0 + 1);
    let y1 = ((at.1 + h).round() as i32).max(y0 + 1);
    fill(c, (x0, y0), (x1 - x0, y1 - y0), color);
}

/// A filled disc of pixels, about `at`.
fn disc(c: &mut Canvas, at: (f32, f32), r: f32, color: Color) {
    let r = r.max(0.8);
    for y in (at.1 - r).floor() as i32..=(at.1 + r).ceil() as i32 {
        for x in (at.0 - r).floor() as i32..=(at.0 + r).ceil() as i32 {
            let (dx, dy) = (x as f32 + 0.5 - at.0, y as f32 + 0.5 - at.1);
            if dx * dx + dy * dy <= r * r {
                block(c, x, y, 1, color);
            }
        }
    }
}

/// A bar of pixels `width` across from `a` to `b`, rounded at its ends or cut
/// square (the pixels of a rotated rectangle, not of a square brush swept
/// along a line, which fattens any slant).
fn bar(c: &mut Canvas, a: (f32, f32), b: (f32, f32), width: f32, round: bool, color: Color) {
    let half = (width * 0.5).max(0.5);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    let pad = half + 1.0;
    let (w, h) = (c.w() as f32, c.h() as f32 * 2.0);
    // Only the part of its box that is on the screen is walked.
    let x0 = (a.0.min(b.0) - pad).floor().max(0.0) as i32;
    let x1 = (a.0.max(b.0) + pad).ceil().min(w - 1.0) as i32;
    let y0 = (a.1.min(b.1) - pad).floor().max(0.0) as i32;
    let y1 = (a.1.max(b.1) + pad).ceil().min(h - 1.0) as i32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            let (px, py) = (x as f32 + 0.5 - a.0, y as f32 + 0.5 - a.1);
            let along = if len2 > 1e-6 {
                (px * dx + py * dy) / len2
            } else {
                0.0
            };
            if !round && !(0.0..=1.0).contains(&along) && len2 > 1e-6 {
                continue;
            }
            let t = along.clamp(0.0, 1.0);
            let (qx, qy) = (px - dx * t, py - dy * t);
            if qx * qx + qy * qy <= half * half {
                block(c, x, y, 1, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgb;

    fn egg(w: u16, h: u16) -> Voladores {
        Voladores::new(w, h, &mut fastrand::Rng::with_seed(3))
    }

    /// Does the canvas show a color (of the classic palette) in some cell?
    fn shows(c: &Canvas, hex: u32) -> bool {
        c.cells()
            .iter()
            .any(|cell| cell.fg == rgb(hex) || cell.bg == rgb(hex))
    }

    /// Thirteen turns, never backwards.
    #[test]
    fn the_frame_turns_thirteen_times() {
        assert_eq!(Voladores::turns(0.0), 0.0);
        assert!((Voladores::turns(1.0) - 13.0).abs() < 1e-4);
        let mut last = 0.0;
        for i in 1..=100 {
            let n = Voladores::turns(i as f32 / 100.0);
            assert!(n > last, "{i}");
            last = n;
        }
    }

    /// Each flyer sits until his turn, flies down (never under the ground,
    /// whatever the screen), passes both behind the pole and in front of it,
    /// and lands upright on the ground, untied.
    #[test]
    fn the_flyers_circle_down_and_land_on_their_feet() {
        for (w, h) in [(40, 14), (80, 24), (200, 60), (60, 90)] {
            let mut e = egg(w, h);
            for i in 0..4 {
                let letting_go = FALL + i as f32 * DELAY;
                let (mut front, mut behind) = (false, false);
                let mut t = 0.0;
                while t <= END {
                    e.t = t;
                    let g = e.geo();
                    let f = e.flight(&g, i);
                    // The lowest the headdress or the feet reach.
                    let low = f.y + (-f.ang.cos() * 10.6).max(f.ang.cos() * 5.0) * g.u;
                    assert!(low <= g.ground + 0.1 * g.u, "{w}x{h} {i} at {t}: {low}");
                    if t < letting_go {
                        assert_eq!(f.ang, 0.0, "seated at {t}");
                        assert!(f.tie.is_none());
                    } else if t < letting_go + FLIGHT * 0.9 {
                        front |= f.z > 1.0;
                        behind |= f.z < -1.0;
                    }
                    t += 0.05;
                }
                assert!(front && behind, "{w}x{h} {i}");
                e.t = letting_go + FLIGHT + 0.01;
                let g = e.geo();
                let f = e.flight(&g, i);
                assert!(f.ang.abs() < 1e-3 && f.tie.is_none());
                assert!((f.y - (g.ground - 5.0 * g.u)).abs() < 0.01);
            }
        }
    }

    /// The coils on the pole are all there until the first flyer lets go, and
    /// run down to nothing as they land.
    #[test]
    fn the_coils_run_down() {
        let e = egg(100, 30);
        let g = e.geo();
        assert_eq!(Voladores::coil(&g, FALL - 0.1), 13.0 * g.u);
        let mut last = f32::MAX;
        let mut t = FALL;
        while t < FALL + 3.0 * DELAY + FLIGHT + 0.2 {
            let len = Voladores::coil(&g, t);
            assert!(len <= last, "{t}");
            last = len;
            t += 0.1;
        }
        assert_eq!(last, 0.0);
    }

    /// The pole rises with everyone on it, and sinks back.
    #[test]
    fn the_pole_rises_and_sinks() {
        let mut e = egg(100, 30);
        let mut sunk = |t: f32| {
            e.t = t;
            e.geo().sunk
        };
        let px = 60.0;
        assert!(sunk(0.0) > px);
        assert!(sunk(1.0) < sunk(0.5));
        assert_eq!(sunk(RISEN), 0.0);
        assert_eq!(sunk(FALL), 0.0);
        assert_eq!(sunk(SINKING), 0.0);
        assert!(sunk(SINKING + 1.0) > 0.0);
        assert!(sunk(END) > px);
    }

    /// Red trousers and white shirts, mirrors and ribbons of every color,
    /// the flute's notes, the rope: everything is on show, and at the end
    /// nothing is left.
    #[test]
    fn the_voladores_come_dressed_and_go() {
        let theme = Theme::classic();
        let mut e = egg(120, 40);
        let mut c = Canvas::new(120, 40, Color::Reset);
        let wanted = [
            0xc8102e, 0xf4f1ea, 0xdadde6, 0xe4007c, 0xffd200, 0x00aab8, 0x2f9a3a, 0x7028a0,
            0xff8c1a, 0xb89868, 0xc9a96a,
        ];
        let mut seen = [false; 11];
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

    /// A theme other than `classic` paints the men with its own colors.
    #[test]
    fn a_theme_gives_its_own_colors() {
        let nord = Theme::builtin("nord").unwrap();
        let mut e = egg(100, 40);
        e.t = 16.0;
        let mut c = Canvas::new(100, 40, Color::Reset);
        e.draw(&mut c, &nord);
        assert!(!shows(&c, 0xc8102e));
        assert!(c.cells().iter().any(|cell| cell.ch != ' '));
    }
}
