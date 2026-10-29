//! A winged jaguar alebrije flies across the screen during Día de Muertos.
//!
//! Original oxiper pixel art, after the Oaxacan alebrijes carved in copal
//! wood and painted in wild colors: a turquoise jaguar with rosa chevrons and
//! yellow dots, a cempasúchil head with a rosa crown pattern, turquoise
//! around the eye and fangs bared, legs stretched out as in a leap. Its
//! feathered wings (a yellow leading edge, morado coverts dotted in rosa,
//! turquoise and orange feathers) beat in four positions, the far wing a
//! shade darker behind the body, and it rises on each downstroke as it
//! swoops along. Its long striped tail curls up into a spiral that winds and
//! unwinds, and a dust of colored sparks trails behind it, twinkling as it
//! falls. It comes from either side and is gone within 30 s.

use super::muertos::{Palette, fade};
use super::{Egg, Info, pixel, sprite};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "alebrije",
    start,
    fiesta: Kind::Muertos,
    when: super::any_day,
};

/// Facing right, without wings and tail.
const BODY: &[&str] = &[
    "...............................r.....r.......",
    "..............................rrr...rrr......",
    "..............................RrroooRrR......",
    ".............................oOOOOOOOOOo.....",
    ".....QQQQrr.................oOROORROOOOOo....",
    "...QrRQQQQRrQQQQrrQQQQrrQQQQOROOROOQQQOOO....",
    "..QQQRRQQQQRRQQQQRRQQQQRRQQQOOROOOQYkQOOOo...",
    ".rQYQQRRQYQQRRQYQQRRQYQQRRQYOOOOROOQQOOOOYY..",
    "rQQQQRRQQQQRRQQQQRRQQQQRRQQQQOOROOOOOOOYYYYk.",
    "QQQQRRQQQQRRQQQQRRQQQQRRQQQQRROOOOOOOOYYYYYY.",
    "qQQRRQQQQRRQQQQRRQQQQRRQQQQRRQQOOOOOOkWkWkk..",
    ".qRRQQQYdooodooodooYOYYYQQRRQQQQddddYYYYY....",
    "..RQQQQo...........odoooqRRQQQQR.............",
    "..QQRqq..................qqqRQQQrQ...........",
    ".QQQT.......................TqqqTQQQ.........",
    "rQqq.............................qqqTQQ......",
    "YY...................................qQQrYY..",
    "......................................qqTYYY.",
];
/// The near wing at four angles, from raised high to lowered (the far one
/// is the same, a shade darker).
const WINGS: [&[&str]; 4] = [
    // At 112°.
    &[
        "...M........",
        ".OQMY.......",
        "...MY.......",
        "..QMY.......",
        ".OQRMY......",
        "..QMMY......",
        "..QMMYY.....",
        ".OQMMMY.....",
        "OOQQRRY.....",
        "..QQMMYY....",
        "..QQMMMY....",
        ".OOQMMMYY...",
        "..OQQRRYY...",
        "..OQQMMMY...",
        ".OOQQMMMYY..",
        "...QQMMRMY..",
        "..OOQQRMMYY.",
        "..OOQQMMMYY.",
        "...OQQMMMMY.",
        "...OQQQMRMYY",
        "..OOOQQMMMY.",
        "...OOQQM....",
        "...OOQ......",
    ],
    // At 140°.
    &[
        "...................",
        "...................",
        "...................",
        ".QMY...............",
        ".OMMY..............",
        "...MRY.............",
        "...QRMYY...........",
        "..OQQMMYY..........",
        "....QMMMYY.........",
        "....QQMMMYY........",
        "....OQQMMMYY.......",
        ".....OQQMMRYYY.....",
        ".....OQQQRMMMYY....",
        ".....OOQQMMMMMYY...",
        ".......OQQMMRRMYY..",
        ".......OOQQMRMMMYY.",
        ".........OQQMMMMMYY",
        ".........OOQQQMRMM.",
        ".........OOQQQQMM..",
        "...........OOQQQ...",
        "...........OOOQ....",
        "..........O.OO.....",
        "............OO.....",
        "............O......",
    ],
    // At 170°.
    &[
        "YY....................",
        "MMMMYYYY..............",
        ".QMMRMMYYYYYY.........",
        "...QMMMMRMMYYYYYYYY...",
        "...OQQQMRMMMRMMMYYYYYY",
        "...O..QQQMMMRMMMRMMMMY",
        "......OOQQQQMMMRMMMMRM",
        "........OOQQQQQQMMMRM.",
        ".........O.OQQQQQQQQM.",
        "...........OOOOQQQQQQ.",
        "..............OOOOOQQ.",
        "..............O..OOOO.",
        ".................O.OO.",
    ],
    // At 212°.
    &[
        "...............O.....",
        ".............O.OO....",
        ".............OOOOQ...",
        ".............OOQQQ...",
        "...........OOOQQQQM..",
        "...........OOQQQMRM..",
        ".........OOQQQQMMMRM.",
        ".........OQQQMRMMMMYY",
        ".......O.QQQMMRMMYYY.",
        ".......OQQQMMMMMYYY..",
        ".......QQQMRMMYYY....",
        ".....OQQMMMRMYY......",
        ".....QQMMMMYYY.......",
        "...O.QMMRMYY.........",
        "...OQMMMYYY..........",
        "...QMRMYY............",
        "...QMMY..............",
        "..QMYY...............",
        "..RY.................",
        "..Y..................",
    ],
];
const BODY_W: i32 = 45;
/// Where the wings join the body, and where the tail does.
const SHOULDER: (i32, i32) = (24, 6);
const RUMP: (f32, f32) = (0.5, 8.0);
/// The wing's joint in each of its frames.
const PIVOTS: [(i32, i32); 4] = [(12, 20), (19, 16), (22, 4), (21, 8)];
/// One wingbeat: up, down and back up.
const BEAT: [usize; 6] = [0, 1, 2, 3, 2, 1];
const BEAT_SECS: f32 = 0.75;

/// Pixels of tail, and how far behind and around the creature reaches.
const TAIL: usize = 28;
const REACH: i32 = 30;

/// Sparks: how many at once, how often one falls, how long one lives.
const SPARKS: usize = 48;
const SPARK_EVERY: f32 = 0.04;
const SPARK_LIFE: f32 = 1.4;
const SPARK_COLORS: [u8; 6] = *b"YrQoPW";

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let hf = h as f32 * 2.0;
    let span = w as f32 + (BODY_W + 2 * REACH) as f32;
    Box::new(Alebrije {
        t: 0.0,
        speed: span / (11.0 + 4.0 * rng.f32()),
        left: rng.bool(),
        y0: hf * (0.25 + 0.3 * rng.f32()),
        swoop: (hf * 0.12).min(10.0),
        w: w as i32,
        sparks: [Spark::default(); SPARKS],
        next: 0,
        spark_in: 0.0,
        rng: fastrand::Rng::with_seed(rng.u64(..)),
    })
}

#[derive(Clone, Copy, Default)]
struct Spark {
    x: f32,
    y: f32,
    /// Seconds left; 0 means none.
    life: f32,
    color: u8,
}

struct Alebrije {
    t: f32,
    /// Columns per second.
    speed: f32,
    /// Flying right to left (mirrored)?
    left: bool,
    /// The height it flies at, in pixels, and how deep it swoops.
    y0: f32,
    swoop: f32,
    w: i32,
    sparks: [Spark; SPARKS],
    next: usize,
    spark_in: f32,
    rng: fastrand::Rng,
}

impl Alebrije {
    /// Where in its wingbeat it is, 0 to 1.
    fn beat(&self) -> f32 {
        (self.t / BEAT_SECS).fract()
    }

    /// The body's top-left pixel.
    fn at(&self) -> (i32, i32) {
        let d = (self.t * self.speed) as i32;
        let x = if self.left {
            self.w + REACH - d
        } else {
            d - BODY_W - REACH
        };
        // It swoops along, and every downstroke lifts it.
        let y = self.y0 + (self.t * 0.6).sin() * self.swoop - (self.beat() * TAU).sin() * 1.5;
        (x, y.round() as i32)
    }

    /// Mirrors a column of the art when flying left.
    fn mx(&self, x: f32) -> f32 {
        if self.left { BODY_W as f32 - x } else { x }
    }

    /// The tail's pixels, from the rump to the tip of its spiral, relative to
    /// the body, facing right.
    fn tail(&self) -> impl Iterator<Item = (f32, f32, usize)> + '_ {
        // How tightly it curls breathes in and out.
        let curl = 0.27 + 0.07 * (self.t * 2.1).sin();
        let mut ang = PI * 0.93 + 0.1 * (self.t * 2.1 + 1.0).sin();
        let (mut x, mut y) = RUMP;
        (0..TAIL).map(move |s| {
            let f = s as f32 / TAIL as f32;
            ang -= 0.02 + curl * f.powf(1.5);
            x += ang.cos();
            y -= ang.sin();
            (x, y, s)
        })
    }
}

impl Egg for Alebrije {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        for s in self.sparks.iter_mut().filter(|s| s.life > 0.0) {
            s.life = (s.life - dt).max(0.0);
            s.y += 4.0 * dt;
        }
        self.spark_in -= dt;
        while self.spark_in <= 0.0 {
            self.spark_in += SPARK_EVERY;
            // From the wings and the tail, behind the body.
            let (x, y) = self.at();
            let (along, down) = (self.rng.f32() * 26.0, self.rng.f32() * 14.0);
            let (sx, sy) = (self.mx(along) + x as f32, y as f32 + down);
            if sx < 0.0 || sx >= self.w as f32 {
                continue;
            }
            self.sparks[self.next] = Spark {
                x: sx,
                y: sy,
                life: SPARK_LIFE * (0.5 + 0.5 * self.rng.f32()),
                color: SPARK_COLORS[self.rng.usize(..SPARK_COLORS.len())],
            };
            self.next = (self.next + 1) % SPARKS;
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::new(theme);
        let mut far = Palette::new(theme);
        far.fade_all(theme, 0.45);
        for s in self.sparks.iter().filter(|s| s.life > 0.0) {
            // Twinkling: on and off as it falls, dimmer as it dies.
            if (s.life * 9.0) as i32 % 3 == 0 {
                continue;
            }
            let k = 1.0 - s.life / SPARK_LIFE;
            pixel(
                c,
                s.x as i32,
                s.y as i32,
                fade(theme, pal.of(s.color), k * 0.8),
            );
        }
        let (x, y) = self.at();
        let frame = BEAT[(self.beat() * BEAT.len() as f32) as usize % BEAT.len()];
        let (art, (px, py)) = (WINGS[frame], PIVOTS[frame]);
        let wing_w = art.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
        // A wing's top-left pixel, its joint `dx` columns behind the shoulder.
        let wing_at = |dx: i32, dy: i32| {
            let jx = SHOULDER.0 + dx;
            let left = if self.left {
                BODY_W - 1 - jx - (wing_w - 1 - px)
            } else {
                jx - px
            };
            (x + left, y + SHOULDER.1 + dy - py)
        };
        sprite(c, wing_at(4, -1), art, self.left, |_| 0, |ch| far.get(ch));
        for (tx, ty, s) in self.tail() {
            let col = pal.of(b"RYQ"[s / 2 % 3]);
            let (tx, ty) = (x + self.mx(tx).floor() as i32, y + ty.floor() as i32);
            pixel(c, tx, ty, col);
            if s < TAIL * 3 / 5 {
                pixel(c, tx, ty + 1, col);
            }
        }
        sprite(c, (x, y), BODY, self.left, |_| 0, |ch| pal.get(ch));
        sprite(c, wing_at(0, 0), art, self.left, |_| 0, |ch| pal.get(ch));
    }

    fn done(&self) -> bool {
        let (x, _) = self.at();
        let gone = if self.left {
            x + BODY_W + REACH < 0
        } else {
            x - REACH > self.w
        };
        gone && self.sparks.iter().all(|s| s.life == 0.0) || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_fits_together() {
        assert!(BODY.iter().all(|r| r.len() as i32 <= BODY_W));
        for (art, (px, py)) in WINGS.iter().zip(PIVOTS) {
            let w = art.iter().map(|r| r.len()).max().unwrap() as i32;
            assert!(px <= w && (py as usize) <= art.len());
        }
    }

    /// It flies across in both directions and paints plenty on the way.
    #[test]
    fn flies_across() {
        let theme = Theme::classic();
        for seed in 0..4 {
            let mut e = start(120, 36, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(120, 36, Color::Reset);
            let mut most = 0;
            while !e.done() {
                e.update(0.1);
                c.clear();
                e.draw(&mut c, &theme);
                most = most.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
            }
            assert!(most > 250, "{most} cells");
        }
    }
}
