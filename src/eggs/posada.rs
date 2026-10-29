//! The peregrinos of a posada walk across the bottom of the screen during
//! the Navidad, asking for lodging.
//!
//! Original oxiper pixel art: José in his straw hat, beard and green cloak,
//! leaning on a staff with a gourd hung from it; María in her blue mantle
//! and white veil under a golden halo, riding sidesaddle on a gray burro in
//! a striped sarape, its ears flicking; and four children behind them, an
//! angel in white wings and three little pilgrims, two with candles whose
//! flames flicker and two with luces de bengala (sparklers) that throw gold
//! sparks as they go. They walk at a stroll, each a little out of step, and
//! over them the verse the peregrinos sing at every door types itself out,
//! "En el nombre del cielo, os pido posada". They come from either side and
//! are gone within 30 s.

use super::navidad::{Palette, fade, type_out};
use super::{Egg, Info, pixel, sprite};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "posada",
    start,
    fiesta: Kind::Navidad,
    when: super::any_day,
};

type Frames = [&'static [&'static str]; 2];

/// José, facing right: hat, beard, cloak and staff with a gourd hung on it
/// (22 rows; his legs go below).
const JOSE: &[&str] = &[
    "................bb",
    ".......CCCC.....bB",
    "......CCCCCC..YYbB",
    ".....CCRRRRCC.YdbB",
    "..CCCCCCCCCCCC.dbB",
    "...dddddddddd...bB",
    "....bbeeeeee....bB",
    "....bbeeekeeE...bB",
    ".....bbbeeeee...bB",
    ".....bbbbbbe....bB",
    "......bbbbb.....bB",
    "....GGGeeGG.....bB",
    "...GGGGBBBBBB..eeB",
    "..GGhGGBBBBBBBeeeB",
    "..GGhGGBBBBBBBBeeB",
    "..GGhGGBBbBBBBBBeB",
    "..GGhGGRRRRRRRBB.B",
    "..GGhGGBBBBBBBB..B",
    "..GGhGGBBBbBBBB..B",
    "..GGhGGBBBBBBBB..B",
    "...GhGBBBBbBBBB..B",
    "...GGGBBBBBBBBB..B",
];
/// José's hem, legs and sandals, standing and mid-step.
const JOSE_LEGS: Frames = [
    &[
        "...hhhBBBBBBBBB..B",
        "....hhbbbbbbbbb..B",
        "......ee..ee.....B",
        "......ee..ee.....B",
        ".....bbb..bbb....B",
    ],
    &[
        "...hhhBBBBBBBBB..B",
        "....hhbbbbbbbbb..B",
        ".......eeee......B",
        ".....ee....ee....B",
        "....bbb....bbb...b",
    ],
];
const JOSE_W: i32 = 18;
const JOSE_H: i32 = JOSE.len() as i32 + 5;

/// The burro, facing right, with a sarape across its back (16 rows; its
/// legs go below).
const BURRO: &[&str] = &[
    ".....................vv..vv......",
    ".....................Uv..Uv......",
    ".....................UU..UU......",
    ".....................UUUUUUU.....",
    ".....................UUUUUkUU....",
    "....................UUUUUUUVVV...",
    "..................UUUUUUUUVVVVv..",
    ".................UUUUUUUUUVVVV...",
    "...............UUUUUUUUUUUVVVV...",
    "..UUUUUUUUUUUUUUUUUUUUUUUUUVV....",
    ".UUUURRRYYYGGGRRRYYUUUUUUUUV.....",
    "UUUURRRYYYGGGRRRYYYUUUUUUUUV.....",
    "vUUUURRRYYYGGGRRRYYUUUUUUUUV.....",
    "vUUUUUUUUUUUUUUUUUUUUUUUUUV......",
    ".vUUUUUUUUUUUUUUUUUUUUUUUUV......",
    "..UUUUUUUVVVVVVVVVVUUUUUUV.......",
];
/// The burro's legs, as it takes each step.
const BURRO_LEGS: Frames = [
    &[
        "....UU.UU.............UU.UU......",
        "....UU.UU.............UU.UU......",
        "....UU.UU.............UU.UU......",
        "....UU.UU.............UU.UU......",
        "....vv.vv.............vv.vv......",
    ],
    &[
        "....UU..UU............UU..UU.....",
        "...UU....UU..........UU....UU....",
        "...UU....UU..........UU....UU....",
        "..UU......UU........UU......UU...",
        "..vv......vv........vv......vv...",
    ],
];
const BURRO_W: i32 = 33;
const BURRO_H: i32 = BURRO.len() as i32 + 5;

/// María, facing right, in her blue mantle and white veil under a halo, her
/// hands folded; she rides sidesaddle with her skirt over the burro's flank.
const MARIA: &[&str] = &[
    "....yyyyyy......",
    "...yd....dy.....",
    "..yd.WWWWW.dy...",
    "..y.WWWWWWW.y...",
    "..y.WWWWeeW.y...",
    "...yWWWWeeeWy...",
    "...yWWWWekeEy...",
    "....WWWWeeeE....",
    "....AWWWWeeW....",
    "...AAAWWWWWAA...",
    "..AAAIAAAAAAAA..",
    "..AAIAAAAAAAAAA.",
    ".AAAIAAAAAeeAAA.",
    ".AAAIAAAAeeAAAA.",
    ".AAAIAAAAAAAAAA.",
    "AAAAIAAAAAAAAAAA",
    "AAAAIAAAAAAAAAAA",
    "AAAIAAAAAAAAAAAA",
    "AAAIAAAAAAAAAAAA",
    "AAAIAAAAAAAAAAAA",
    "WAWIAWAWAWAWAWAW",
    "..........ee....",
    "..........bb....",
];
const MARIA_W: i32 = 16;
/// Where she sits on the burro: columns from its tail, and the burro's row
/// where her hem (her row `HEM`) falls.
const SEAT: i32 = 5;
const SADDLE: i32 = 12;
const HEM: i32 = 20;

/// A child with a candle (the colors of 'x' shirt, 'z' its shade and 'j'
/// hair are the child's own); the flame goes above the candle.
const CHILD: &[&str] = &[
    "...jjjjj....",
    "..jjjjjjj...",
    "..jjeeeeej..",
    "..jeekeeeE..",
    "..jeeeeeee.W",
    "...eeEeeee.W",
    "....xxxxxeeW",
    "...xxxxxxxee",
    "..xxxxxxxxx.",
    "..xxzxxxxxx.",
    "..xxzxxxxxx.",
    ".xxxzxxxxxx.",
    ".xxxxzxxxxx.",
    "xxxxxxxxxxxx",
];
/// A child holding a luz de bengala up high; the sparks fly from its tip.
const SPARKLER: &[&str] = &[
    "...........S",
    "...........S",
    "...........S",
    "...jjjjj...S",
    "..jjjjjjj..S",
    "..jjeeeeej.S",
    "..jeekeeeE.S",
    "..jeeeeeee.S",
    "...eeEeeee.S",
    "....xxxxxeeS",
    "...xxxxxxxee",
    "..xxxxxxxxx.",
    "..xxzxxxxxx.",
    "..xxzxxxxxx.",
    ".xxxzxxxxxx.",
    "xxxxxxxxxxxx",
];
/// Their legs: standing and mid-step.
const CHILD_LEGS: Frames = [
    &["...ee..ee...", "...ee..ee...", "..bbb..bbb.."],
    &["....eeee....", "...ee..ee...", "..bbb..bbb.."],
];
/// An angel's wings, behind the child.
const WINGS: &[&str] = &[
    "WW........",
    "WWW.......",
    ".WWW......",
    ".WwWW.....",
    "..WwWW....",
    "..wWWWW...",
    "...wWWWW..",
    "....wWWW..",
    "....wwWW..",
];
const KID_W: i32 = 12;
/// Where a candle's flame and a sparkler's tip are in the art.
const FLAME: (i32, i32) = (11, 3);
const TIP: (i32, i32) = (11, 0);

/// A child in the procession: its shirt and hair, and what it carries.
struct Kid {
    shirt: u8,
    hair: u8,
    angel: bool,
    sparkler: bool,
}

impl Kid {
    fn art(&self) -> &'static [&'static str] {
        if self.sparkler { SPARKLER } else { CHILD }
    }
}

/// From the back of the procession.
const KIDS: [Kid; 4] = [
    Kid {
        shirt: b'W',
        hair: b'b',
        angel: true,
        sparkler: false,
    },
    Kid {
        shirt: b'Q',
        hair: b'k',
        angel: false,
        sparkler: true,
    },
    Kid {
        shirt: b'P',
        hair: b'k',
        angel: false,
        sparkler: false,
    },
    Kid {
        shirt: b'O',
        hair: b'b',
        angel: false,
        sparkler: true,
    },
];
/// Columns from one child to the next, from the children to the burro, and
/// from the burro to José.
const KID_PITCH: i32 = KID_W + 3;
const BURRO_AT: i32 = KIDS.len() as i32 * KID_PITCH + 3;
const JOSE_AT: i32 = BURRO_AT + BURRO_W + 4;
/// The procession's width and height in pixels (María rides above the burro).
const WIDTH: i32 = JOSE_AT + JOSE_W;
const HEIGHT: i32 = BURRO_H + HEM - SADDLE;

/// Columns per second at a stroll; faster on wide screens so they cross in time.
const STROLL: f32 = 6.0;
const CROSSING: f32 = 26.0;
/// Columns walked per step.
const STEP: f32 = 2.6;

/// What the peregrinos sing at every door, in one line or two for a
/// narrow screen; how fast it types (letters per second) and when it starts.
const SONG: &str = "En el nombre del cielo, os pido posada";
const SONG_LINES: [&str; 2] = ["En el nombre del cielo,", "os pido posada"];
const TYPING: f32 = 12.0;
const SONG_AT: f32 = 0.8;

/// Sparks in the air at most, how many each sparkler throws a second, and
/// how long one lasts.
const SPARKS: usize = 160;
const SPARK_RATE: f32 = 70.0;
const SPARK_LIFE: f32 = 0.8;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let span = w as f32 + WIDTH as f32 + 2.0;
    let speed = STROLL.max(span / CROSSING);
    Box::new(Posada {
        t: 0.0,
        dist: 0.0,
        speed,
        total: span / speed,
        left: rng.bool(),
        w: w as i32,
        h: h as i32 * 2,
        sparks: [Spark::default(); SPARKS],
        next: 0,
        spark_in: 0.0,
        rng: fastrand::Rng::with_seed(rng.u64(..)),
    })
}

#[derive(Clone, Copy, Default)]
struct Spark {
    /// Pixels from the left edge and from the top.
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Seconds left, and how many it had; 0 means none.
    life: f32,
    span: f32,
}

struct Posada {
    t: f32,
    /// Columns walked.
    dist: f32,
    speed: f32,
    /// Seconds the crossing takes.
    total: f32,
    /// Walking right to left (mirrored)?
    left: bool,
    w: i32,
    /// Screen height in pixels: the row under their feet.
    h: i32,
    sparks: [Spark; SPARKS],
    next: usize,
    spark_in: f32,
    rng: fastrand::Rng,
}

impl Posada {
    /// The left column of the procession.
    fn edge(&self) -> i32 {
        let d = self.dist as i32;
        if self.left { self.w - d } else { d - WIDTH }
    }

    /// The left column of something `w` columns wide at `x` in the
    /// procession (which is drawn facing right).
    fn col(&self, x: i32, w: i32) -> i32 {
        if self.left {
            self.edge() + WIDTH - x - w
        } else {
            self.edge() + x
        }
    }

    /// Mirrors a column of a sprite of width `w` when walking left.
    fn mx(&self, x: i32, w: i32) -> i32 {
        if self.left { w - 1 - x } else { x }
    }

    /// Is walker `i` mid-step? Each is a little behind the one in front.
    fn step(&self, i: usize) -> bool {
        ((self.dist + i as f32 * 1.4) / STEP) as i32 % 2 == 1
    }

    /// Where child `i` is: its left column and top pixel row (the body
    /// drops a little as the legs part).
    fn kid(&self, i: usize) -> (i32, i32) {
        let lift = i32::from(!self.step(i));
        (
            self.col(i as i32 * KID_PITCH, KID_W),
            self.h - KIDS[i].art().len() as i32 - 3 - lift,
        )
    }
}

impl Egg for Posada {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
        for s in self.sparks.iter_mut().filter(|s| s.life > 0.0) {
            s.life = (s.life - dt).max(0.0);
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            s.vy += 30.0 * dt;
        }
        self.spark_in -= dt;
        while self.spark_in <= 0.0 {
            self.spark_in += 1.0 / SPARK_RATE;
            for i in (0..KIDS.len()).filter(|&i| KIDS[i].sparkler) {
                let (x, y) = self.kid(i);
                let tx = x + self.mx(TIP.0, KID_W);
                if tx < -8 || tx >= self.w + 8 {
                    continue;
                }
                let ang = self.rng.f32() * TAU;
                let speed = 2.0 + self.rng.f32() * 13.0;
                let life = SPARK_LIFE * (0.4 + 0.6 * self.rng.f32());
                self.sparks[self.next] = Spark {
                    x: tx as f32 + 0.5,
                    y: (y + TIP.1) as f32,
                    vx: ang.cos() * speed,
                    vy: ang.sin() * speed * 0.7 - 6.0,
                    life,
                    span: life,
                };
                self.next = (self.next + 1) % SPARKS;
            }
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::navidad(theme);
        // The children, from the back of the procession.
        for (i, kid) in KIDS.iter().enumerate() {
            let (x, y) = self.kid(i);
            if x + KID_W < 0 || x >= self.w {
                continue;
            }
            let shirt = pal.of(kid.shirt);
            let (shade, hair) = (shirt.scale(0.7), pal.of(kid.hair));
            let color = |ch: u8| match ch {
                b'x' => Some(shirt),
                b'z' => Some(shade),
                b'j' => Some(hair),
                _ => pal.get(ch),
            };
            let art = kid.art();
            let legs = CHILD_LEGS[usize::from(self.step(i))];
            sprite(c, (x, y + art.len() as i32), legs, self.left, |_| 0, color);
            if kid.angel {
                let wx = x + if self.left { KID_W - 5 } else { -5 };
                sprite(c, (wx, y + 1), WINGS, self.left, |_| 0, color);
                // A halo of gold above the head.
                for dx in 3..8 {
                    let hx = x + self.mx(dx, KID_W);
                    pixel(c, hx, y - 2 + i32::from(dx % 4 == 3), pal.of(b'Y'));
                }
            }
            sprite(c, (x, y), art, self.left, |_| 0, color);
        }
        // The burro with María on its back, and José in front.
        let mid = self.step(KIDS.len());
        let bx = self.col(BURRO_AT, BURRO_W);
        let by = self.h - BURRO_H - i32::from(!mid);
        if bx + BURRO_W >= 0 && bx < self.w {
            // Its ears flick now and then.
            let flick = (self.t * 1.7).sin() > 0.93;
            let back = if self.left { 1 } else { -1 };
            let ears = |r: i32| if r < 3 && flick { back } else { 0 };
            let legs = BURRO_LEGS[usize::from(mid)];
            let rows = BURRO.len() as i32;
            sprite(c, (bx, by + rows), legs, self.left, |_| 0, |ch| pal.get(ch));
            sprite(c, (bx, by), BURRO, self.left, ears, |ch| pal.get(ch));
            // María sways a little in the saddle.
            let sway = ((self.dist / STEP * PI).sin() * 0.6).round() as i32;
            let mx = self.col(BURRO_AT + SEAT, MARIA_W);
            let top = (mx, by + SADDLE - HEM);
            let lean = |r: i32| if r < 10 { sway } else { 0 };
            sprite(c, top, MARIA, self.left, lean, |ch| pal.get(ch));
        }
        let jx = self.col(JOSE_AT, JOSE_W);
        if jx + JOSE_W >= 0 && jx < self.w {
            let step = self.step(KIDS.len() + 1);
            let jy = self.h - JOSE_H - i32::from(!step);
            let legs = JOSE_LEGS[usize::from(step)];
            let rows = JOSE.len() as i32;
            sprite(c, (jx, jy + rows), legs, self.left, |_| 0, |ch| pal.get(ch));
            sprite(c, (jx, jy), JOSE, self.left, |_| 0, |ch| pal.get(ch));
        }
        // The candles' flames flicker; the sparks fly.
        for i in (0..KIDS.len()).filter(|&i| !KIDS[i].sparkler) {
            let (x, y) = self.kid(i);
            let beat = (self.t * 9.0 + i as f32 * 2.3) as i32;
            let (fx, fy) = (x + self.mx(FLAME.0, KID_W), y + FLAME.1);
            pixel(c, fx, fy, pal.of(b'F'));
            let sway = (beat % 3 - 1) * i32::from(beat % 3 != 1);
            pixel(c, fx + sway, fy - 1, pal.of(b'f'));
        }
        for s in self.sparks.iter().filter(|s| s.life > 0.0) {
            // Twinkling, from white-gold to orange, then gone.
            if (s.life * 25.0) as i32 % 4 == 0 {
                continue;
            }
            let k = 1.0 - s.life / s.span;
            let letter = match k {
                k if k < 0.25 => b'X',
                k if k < 0.6 => b'F',
                _ => b'f',
            };
            let col = fade(theme, pal.of(letter), ((k - 0.5) * 1.4).max(0.0));
            pixel(c, s.x.floor() as i32, s.y.floor() as i32, col);
        }
        // The verse they sing, typed above them while they cross.
        let shown = ((self.t - SONG_AT) * TYPING).max(0.0) as usize;
        let gone = ((self.t - (self.total - 2.5)) / 2.0).clamp(0.0, 1.0);
        if shown > 0 && gone < 1.0 {
            let one = c.w() as usize >= SONG.len() + 4;
            let lines: &[&str] = if one { &[SONG] } else { &SONG_LINES };
            let row = (self.h - HEIGHT).div_euclid(2) - 2 - lines.len() as i32;
            let ink = fade(theme, pal.of(b'y'), gone);
            let cursor = (shown < SONG.len()).then(|| pal.of(b'Y'));
            type_out(c, row.max(0), lines, shown, (ink, cursor), BOLD);
        }
    }

    fn done(&self) -> bool {
        let gone = if self.left {
            self.edge() + WIDTH < 0
        } else {
            self.edge() > self.w
        };
        gone && self.sparks.iter().all(|s| s.life == 0.0) || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        let whole = |rows: &[&str], w: i32| rows.iter().all(|r| r.len() as i32 == w);
        assert!(whole(JOSE, JOSE_W));
        assert!(JOSE_LEGS.iter().all(|l| whole(l, JOSE_W) && l.len() == 5));
        assert!(whole(BURRO, BURRO_W));
        assert!(BURRO_LEGS.iter().all(|l| whole(l, BURRO_W) && l.len() == 5));
        assert!(whole(MARIA, MARIA_W));
        assert!(whole(CHILD, KID_W) && whole(SPARKLER, KID_W));
        assert!(CHILD_LEGS.iter().all(|l| whole(l, KID_W) && l.len() == 3));
        // The candle and the sparkler are where the lights come from.
        assert_eq!(
            CHILD[FLAME.1 as usize + 1].as_bytes()[FLAME.0 as usize],
            b'W'
        );
        assert_eq!(SPARKLER[TIP.1 as usize].as_bytes()[TIP.0 as usize], b'S');
        assert_eq!(SONG_LINES.join(" "), SONG);
    }

    /// They walk across on the ground, from either side, singing.
    #[test]
    fn walk_across_singing() {
        for theme in [Theme::classic(), Theme::builtin("nord").unwrap()] {
            for seed in 0..4 {
                let mut e = start(140, 36, &mut fastrand::Rng::with_seed(seed));
                let mut c = Canvas::new(140, 36, Color::Reset);
                let (mut sang, mut most) = (false, 0);
                let mut mid = Vec::new();
                for _ in 0..120 {
                    e.update(0.25);
                    c.clear();
                    e.draw(&mut c, &theme);
                    sang |= c.to_text().contains(SONG);
                    let on = |x| (20..36).any(|y| c.get(x, y).unwrap().ch != ' ');
                    let cols: Vec<i32> = (0..140).filter(|&x| on(x)).collect();
                    most = most.max(cols.len());
                    if cols.len() > 20 {
                        mid.push(cols.iter().sum::<i32>() / cols.len() as i32);
                    }
                    if e.done() {
                        break;
                    }
                }
                assert!(sang, "the verse never came");
                assert!(most > 60, "{most} columns");
                let moved = mid.last().unwrap() - mid[0];
                assert!(moved.abs() > 40, "{mid:?}");
                assert!(e.done());
            }
        }
    }

    /// On a narrow screen the verse breaks in two lines and still reads.
    #[test]
    fn the_verse_fits_a_narrow_screen() {
        let mut e = start(30, 20, &mut fastrand::Rng::with_seed(3));
        let mut c = Canvas::new(30, 20, Color::Reset);
        for _ in 0..40 {
            e.update(0.25);
        }
        e.draw(&mut c, &Theme::classic());
        let text = c.to_text();
        assert!(text.contains("os pido posada"), "{text}");
    }
}
