//! The Reyes Magos cross the screen under the Estrella de Belén, in the days
//! before Reyes (January 2 to 6).
//!
//! Original oxiper pixel art. Melchor, the old king with the white beard,
//! rides a white horse under a blue and gold caparison, a chest of gold in
//! his hands; Gaspar, in green, a censer of incense in his, sways on a camel
//! with a purple blanket, its head bobbing, trailing a thread of smoke; and
//! Baltasar, in purple, brings the myrrh on an elephant, as the Mexican
//! nacimientos have it, its ears flapping and its trunk swinging under a red
//! and gold blanket. Each mount has its own gait. The star leads the way
//! far ahead of them, twinkling, with a long tail of glitter behind it. They
//! come from either side and are gone within 30 s.

use super::navidad::{Palette, fade};
use super::{Egg, Info, pixel, sprite};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::PI;

pub const INFO: Info = Info {
    name: "reyes",
    start,
    fiesta: Kind::Navidad,
    // The Reyes are on their way from January 2.
    when: |fiesta| fiesta.reyes(),
};

type Frames = [&'static [&'static str]; 2];

/// Melchor's horse, facing right (rows 0 to 17; its legs go below).
const HORSE: &[&str] = &[
    "............................WW.WW......",
    "...........................UWWWWWW.....",
    "..........................UUWWWWWWWW...",
    ".........................UUUWWWkWWWWW..",
    "........................UUUWWWWWWWWWWW.",
    ".......................UUUWWWWWWWWWWwww",
    "......................UUUWWWWWWWWWWwwww",
    ".....................UUUWWWWWWWWW..wwwv",
    "..AAAAAAAAAAAA......UUWWWWWWWWW....www.",
    ".AAYYAAYYAAYYAA....UUWWWWWWWWW.....ww..",
    "aaAAAAAAAAAAAAAa..UUWWWWWWWWWW.........",
    "UaaYYYYYYYYYYaaWWWWWWWWWWWWWWw.........",
    "UUWWWWWWWWWWWWWWWWWWWWWWWWWWw..........",
    "UWWWWWWWWWWWWWWWWWWWWWWWWWWw...........",
    "UWWWWWWWWWWWWWWWWWWWWWWWWWw............",
    ".UWWWWWWWWWWWWWWWWWWWWWWWw.............",
    "..UWWWWwwwwwwwwwwwWWWWWWw..............",
    "...UWWwwwwwwwwwwwwwwWWww...............",
];
/// Its legs: trotting, two ways.
const HORSE_LEGS: Frames = [
    &[
        "....WW.WW............WW.WW.............",
        "....WW.WW............WW.WW.............",
        "....ww.ww............ww.ww.............",
        "....WW.WW............WW.WW.............",
        "....ww.ww............ww.ww.............",
        "....vv.vv............vv.vv.............",
    ],
    &[
        ".....WW..WW..........WW...WW...........",
        "....WW....WW........WW.....WW..........",
        "....ww....ww........ww.....ww..........",
        "...WW......WW......WW.......WW.........",
        "...ww......ww......ww.......ww.........",
        "...vv......vv......vv.......vv.........",
    ],
];

/// Gaspar's camel, facing right (rows 0 to 16; its legs go below).
const CAMEL: &[&str] = &[
    ".................................cc.........",
    "................................cccccc......",
    "................................cccckcccc...",
    "...............................ccccccccCCCC.",
    "..............................ccccccccccCCCC",
    ".............................cccccccccwCCCw.",
    "........cccc.................ccccccc.wwww...",
    ".......cccccc...............cccccccc........",
    "......cccccccc.............cccccccc.........",
    ".....ccccccccccc..........cccccccc..........",
    "....cccccccccccccMMMMMMMMccccccc............",
    "...ccccccccccccMMYMMYMMYMMcccccc............",
    "..cccccccccccccMMMMMMMMMMMMccccc............",
    ".ccccccccccccccYYYYYYYYYYYYcccc.............",
    ".cccccccccccccccccccccccccccccc.............",
    "..ccccccccccccccccccccccccccc...............",
    "...cCCCCCCCCCCCCCCCCCCCCCCcc................",
];
/// Its legs: walking, two ways.
const CAMEL_LEGS: Frames = [
    &[
        "....cc.cc.................cc.cc.............",
        "....cc.cc.................cc.cc.............",
        "....cc.cc.................cc.cc.............",
        "....cc.cc.................cc.cc.............",
        "....cc.cc.................cc.cc.............",
        "....cc.cc.................cc.cc.............",
        "....cc.cc.................cc.cc.............",
        "....CC.CC.................CC.CC.............",
        "....vv.vv.................vv.vv.............",
    ],
    &[
        ".....cc..cc..............cc...cc............",
        ".....cc..cc..............cc...cc............",
        "....cc....cc............cc.....cc...........",
        "....cc....cc............cc.....cc...........",
        "...cc......cc..........cc.......cc..........",
        "...cc......cc..........cc.......cc..........",
        "..cc........cc........cc.........cc.........",
        "..CC........CC........CC.........CC.........",
        "..vv........vv........vv.........vv.........",
    ],
];

/// Baltasar's elephant, facing right (rows 0 to 19; its legs go below).
const ELEPHANT: &[&str] = &[
    "..................................UUUUUUU...........",
    "................................UUUUUUUUUUU.........",
    ".............................UUUUUUUUUUUUUUUU.......",
    ".........RRRRRRRRRRRRRRRRR..UUUUUUUUUUUUUUUUUU......",
    "......RRRRRRRRRRRRRRRRRRRRRUUUUUUUUUUUkUUUUUUUU.....",
    ".....RRYRRRYRRRYRRRYRRRYRRRUUUUUUUUUUUUUUUUUUUUU....",
    "...UYYYYYYYYYYYYYYYYYYYYYYYUvvvvvvUUUUUUUUUUUUUUU...",
    "...URRRRRRRRRRRRRRRRRRRRRRRvUUUUUUvUUUUUUUUUUUUUU...",
    ".UUUDDDDDDDDDDDDDDDDDDDDDDvUUUUUUUUvUUUUUUUUUUUUU...",
    ".UUUUUUUUUUUUUUUUUUUUUUUUUvUUVVVVUUvUUUUUUUUUUUUW...",
    ".UUUUUUUUUUUUUUUUUUUUUUUUUvUUVVVVUUvUUUUUUUUUUUWW...",
    ".UUUUUUUUUUUUUUUUUUUUUUUUUvUUVVVVUUvUUUUUUUUWWWWU...",
    "U.UUUUUUUUUUUUUUUUUUUUUUUUUvUUUUUUUvUUUWWWWWUUUUU...",
    "U.UUUUUUUUUUUUUUUUUUUUUUUUUUvUUUUUvUUU.....UUUUU....",
    "U.UUUUUUUUUUUUUUUUUUUUUUUUUUUvUUUvUUUU.....UUUUU....",
    "U.UUUUUUUUUUUUUUUUUUUUUUUUUUUUvvvUUUUU.....UUUU.....",
    "v.UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU.....UUUU.....",
    "...UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU.....UUUU.....",
    "...UvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvUU......UUUUUU..",
    "....UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU........UUUVV.",
];
/// Its legs: lumbering, two ways.
const ELEPHANT_LEGS: Frames = [
    &[
        ".....UUUUU..UUUUU.........UUUUU..UUUUU..............",
        ".....UUUUU..UUUUU.........UUUUU..UUUUU..............",
        ".....UUUUU..UUUUU.........UUUUU..UUUUU..............",
        ".....UUUUU..UUUUU.........UUUUU..UUUUU..............",
        ".....UUUUU..UUUUU.........UUUUU..UUUUU..............",
        ".....UUUUU..UUUUU.........UUUUU..UUUUU..............",
        ".....WWWWW..WWWWW.........WWWWW..WWWWW..............",
    ],
    &[
        ".....UUUUU....UUUUU.......UUUUU....UUUUU............",
        ".....UUUUU....UUUUU.......UUUUU....UUUUU............",
        ".....UUUUU....UUUUU.......UUUUU....UUUUU............",
        ".....UUUUU....UUUUU.......UUUUU....UUUUU............",
        ".....UUUUU....UUUUU.......UUUUU....UUUUU............",
        ".....UUUUU....UUUUU.......UUUUU....UUUUU............",
        ".....WWWWW....WWWWW.......WWWWW....WWWWW............",
    ],
];

/// A king, facing right, seated (the colors of 'x' robe, 'z' its shade, 'j'
/// beard, 'e' skin and 'E' its shade are the king's own): crown, beard, cape
/// and an arm held out with a gift in the hand.
const KING: &[&str] = &[
    "...Y.Y.Y.Y......",
    "...YYYYYYYY.....",
    "..jYdYdYdYY.....",
    "..jjeeeeeee.....",
    "..jjeekeeeE.....",
    "..jjjeeeeee.....",
    "...jjjjjeee.....",
    "....jjjjjj......",
    "...xxxxxxx......",
    "..xxxxxxxxxee...",
    ".xxxxxxxxxxxxee.",
    ".xxzxxxxxxxxxe..",
    ".xxzxxxxxxxxx...",
    "xxxzxxxxxxxxxx..",
    "xxzxxxxxxxxxxxx.",
    "xzzxxxxxxxxxxxxz",
    "zzxxxxxxxxxxxxzz",
];
const KING_W: i32 = 16;
/// Where the gift is, in the king's hands.
const HANDS: (i32, i32) = (12, 8);
/// The gifts: a chest of gold, a censer and a jar of myrrh.
const ORO: &[&str] = &["YYYYY", "YyyyY", "YYdYY", "dYYYd"];
const INCIENSO: &[&str] = &[".YYY.", "YYYYY", "YdddY", ".YYY."];
const MIRRA: &[&str] = &["..d..", ".YYY.", "PPPPP", "PpPPP", ".PPP."];

/// A king on his mount: what he rides and where in its art he sits, and his
/// colors and gift.
struct Rider {
    body: &'static [&'static str],
    legs: Frames,
    width: i32,
    /// Columns walked per step.
    stride: f32,
    /// The king's left column and top row in the mount's art (negative:
    /// above it).
    seat: (i32, i32),
    robe: u8,
    beard: u8,
    skin: (u8, u8),
    gift: &'static [&'static str],
}

/// From the back of the procession: Baltasar, Gaspar and Melchor, who leads.
const RIDERS: [Rider; 3] = [
    Rider {
        body: ELEPHANT,
        legs: ELEPHANT_LEGS,
        width: 52,
        stride: 4.4,
        seat: (11, -9),
        robe: b'M',
        beard: b'k',
        skin: (b'B', b'b'),
        gift: MIRRA,
    },
    Rider {
        body: CAMEL,
        legs: CAMEL_LEGS,
        width: 44,
        stride: 3.6,
        seat: (14, -3),
        robe: b'G',
        beard: b'T',
        skin: (b'e', b'E'),
        gift: INCIENSO,
    },
    Rider {
        body: HORSE,
        legs: HORSE_LEGS,
        width: 39,
        stride: 3.2,
        seat: (2, -4),
        robe: b'R',
        beard: b'W',
        skin: (b'e', b'E'),
        gift: ORO,
    },
];
/// The gap between mounts, and from the first king to the star.
const GAP: i32 = 6;
const AHEAD: i32 = 24;
/// The star's size: its rays and its tail, in pixels.
const RAYS: f32 = 8.0;
const TAIL: i32 = 26;

/// Where each mount is in the procession, and its width, height and length.
const fn mount_at(i: usize) -> i32 {
    let mut x = 0;
    let mut k = 0;
    while k < i {
        x += RIDERS[k].width + GAP;
        k += 1;
    }
    x
}
const WIDTH: i32 = mount_at(2) + RIDERS[2].width + AHEAD + 12;
/// The tallest of them, the elephant with its king, in pixels.
const HEIGHT: i32 = 20 + 7 + 9;

/// Columns per second at a stately pace; faster on wide screens.
const PACE: f32 = 6.5;
const CROSSING: f32 = 26.0;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Reyes {
        t: 0.0,
        dist: 0.0,
        speed: PACE.max((w as f32 + WIDTH as f32 + 2.0) / CROSSING),
        left: rng.bool(),
        w: w as i32,
        h: h as i32 * 2,
    })
}

struct Reyes {
    t: f32,
    /// Columns walked.
    dist: f32,
    speed: f32,
    /// Going right to left (mirrored)?
    left: bool,
    w: i32,
    /// Screen height in pixels: the row under their feet.
    h: i32,
}

impl Reyes {
    /// The left column of the procession.
    fn edge(&self) -> i32 {
        let d = self.dist as i32;
        if self.left { self.w - d } else { d - WIDTH }
    }

    /// The left column of something `w` wide at `x` in the procession (drawn
    /// facing right).
    fn col(&self, x: i32, w: i32) -> i32 {
        if self.left {
            self.edge() + WIDTH - x - w
        } else {
            self.edge() + x
        }
    }

    /// Mirrors a column when going left.
    fn mx(&self, x: i32, w: i32) -> i32 {
        if self.left { w - 1 - x } else { x }
    }

    /// Is rider `i` mid-step? The mounts keep their own gait, a little out
    /// of step with each other.
    fn step(&self, i: usize) -> bool {
        ((self.dist + i as f32 * 2.1) / RIDERS[i].stride) as i32 % 2 == 1
    }

    /// The star's center in pixels.
    fn star(&self) -> (i32, i32) {
        let front = mount_at(2) + RIDERS[2].width + AHEAD;
        let x = self.col(front, 1);
        let y = (self.h - HEIGHT - 16).max(6) as f32 + (self.t * 1.3).sin() * 2.0;
        (x, y.round() as i32)
    }
}

impl Egg for Reyes {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::navidad(theme);
        let back = if self.left { 1 } else { -1 };
        for (i, rider) in RIDERS.iter().enumerate() {
            let x = self.col(mount_at(i), rider.width);
            if x + rider.width + 4 < 0 || x >= self.w + 4 {
                continue;
            }
            let step = self.step(i);
            let rows = rider.body.len() as i32;
            let top = self.h - rows - rider.legs[0].len() as i32 - i32::from(!step);
            let color = |ch: u8| pal.get(ch);
            // How the mount moves besides its legs: the elephant's ears
            // flap and its trunk swings, the camel's head bobs, the horse
            // nods.
            let beat = (self.dist / rider.stride * PI).sin();
            let shift = |r: i32| match (i, r) {
                (0, 6..=15) => (beat * 0.6).round() as i32 * back,
                (0, 11..) => ((r - 10) as f32 * beat * 0.25).round() as i32 * -back,
                (1, 0..=6) => i32::from(step) * -back,
                (2, 0..=5) => i32::from(step) * -back,
                _ => 0,
            };
            let legs = rider.legs[usize::from(step)];
            sprite(c, (x, top + rows), legs, self.left, |_| 0, color);
            sprite(c, (x, top), rider.body, self.left, shift, color);
            // The king, swaying a little in his seat.
            let kx = self.col(mount_at(i) + rider.seat.0, KING_W);
            let ky = top + rider.seat.1;
            let (robe, skin) = (pal.of(rider.robe), pal.of(rider.skin.0));
            let (shade, skin_shade) = (robe.scale(0.7), pal.of(rider.skin.1));
            let beard = pal.of(rider.beard);
            let king = |ch: u8| match ch {
                b'x' => Some(robe),
                b'z' => Some(shade),
                b'j' => Some(beard),
                b'e' => Some(skin),
                b'E' => Some(skin_shade),
                _ => pal.get(ch),
            };
            let sway = (beat * 0.5).round() as i32;
            let lean = |r: i32| if r < 10 { sway } else { 0 };
            sprite(c, (kx, ky), KING, self.left, lean, king);
            let (gx, gy) = (HANDS.0 + 1, HANDS.1 - rider.gift.len() as i32 / 2);
            let gift_x = kx + self.mx(gx, KING_W) - if self.left { 4 } else { 0 };
            sprite(c, (gift_x, ky + gy), rider.gift, self.left, lean, color);
            // The gold glints now and then; the censer smokes.
            let spark = ((self.t * 3.0 + i as f32 * 1.7) as i32) % 4;
            if spark == 0 && rider.gift != MIRRA {
                let at = (kx + self.mx(HANDS.0 + 2 + spark, KING_W), ky + gy);
                pixel(c, at.0, at.1 - 1, pal.of(b'X'));
            }
            if rider.gift == INCIENSO {
                let smoke = (kx + self.mx(HANDS.0 + 2, KING_W), ky + gy);
                for k in 0..7 {
                    let age = (self.t * 0.8 + k as f32 / 7.0).fract();
                    let sx = smoke.0 as f32 + (age * 6.0 + k as f32).sin() * 2.0 * age;
                    let sy = smoke.1 as f32 - age * 16.0;
                    let col = fade(theme, pal.of(b'w'), age * 0.9);
                    pixel(c, sx.round() as i32, sy.round() as i32, col);
                }
            }
        }
        self.draw_star(c, theme, &pal);
    }

    fn done(&self) -> bool {
        let gone = if self.left {
            self.edge() + WIDTH < 0
        } else {
            self.edge() > self.w
        };
        gone || self.t > 29.5
    }
}

impl Reyes {
    /// The Estrella de Belén: a bright core with long rays that pulse, short
    /// diagonal ones, and a tail of glitter streaming back over the kings.
    fn draw_star(&self, c: &mut Canvas, theme: &Theme, pal: &Palette) {
        let (sx, sy) = self.star();
        let dir = if self.left { -1 } else { 1 };
        // The tail first, so the star sits over it.
        for k in 1..=TAIL {
            let along = k as f32;
            let (tx, ty) = (sx - dir * k, sy as f32 - along * 0.32);
            // Glitter: each bit of the tail flickers on its own.
            let flick = (k * 7 + (self.t * 9.0) as i32 * 3) % 5;
            if flick == 0 || flick == 2 {
                continue;
            }
            let fall = along / TAIL as f32;
            let letter = if fall < 0.3 { b'X' } else { b'F' };
            let col = fade(theme, pal.of(letter), fall * 0.9);
            pixel(c, tx, ty.round() as i32, col);
            // The tail widens a little as it goes.
            if k % 3 == 0 {
                pixel(
                    c,
                    tx,
                    ty.round() as i32 + 1 + k / 12,
                    fade(theme, pal.of(b'f'), fall),
                );
            }
        }
        let pulse = RAYS + (self.t * 4.0).sin() * 1.5;
        for k in 1..=pulse as i32 {
            let fall = k as f32 / pulse;
            let col = fade(
                theme,
                pal.of(if fall < 0.5 { b'X' } else { b'Y' }),
                fall * 0.7,
            );
            for (dx, dy) in [(k, 0), (-k, 0), (0, k), (0, -k)] {
                // The rays are longer up and down than across, as drawn on
                // half-block pixels that are twice as tall as they are wide.
                pixel(c, sx + dx, sy + dy * 2 / 2, col);
            }
        }
        for k in 1..=3 {
            let col = fade(theme, pal.of(b'Y'), k as f32 * 0.25);
            for (dx, dy) in [(k, k), (-k, k), (k, -k), (-k, -k)] {
                pixel(c, sx + dx, sy + dy, col);
            }
        }
        for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            pixel(c, sx + dx, sy + dy, pal.of(b'X'));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        let whole = |rows: &[&str], w: i32| rows.iter().all(|r| r.len() as i32 == w);
        for r in &RIDERS {
            assert!(whole(r.body, r.width), "{}", r.width);
            assert!(r.legs.iter().all(|l| whole(l, r.width)));
            assert_eq!(r.legs[0].len(), r.legs[1].len());
        }
        assert!(whole(KING, KING_W));
        for gift in [ORO, INCIENSO, MIRRA] {
            assert!(gift.iter().all(|r| r.len() == gift[0].len()));
        }
    }

    /// They cross in procession, the star ahead of the first king, from
    /// either side.
    #[test]
    fn cross_behind_the_star() {
        let theme = Theme::classic();
        for seed in 0..4 {
            let mut e = start(160, 40, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(160, 40, Color::Reset);
            let mut most = 0;
            let mut cols = Vec::new();
            for _ in 0..120 {
                e.update(0.25);
                c.clear();
                e.draw(&mut c, &theme);
                let on = |x| (10..40).any(|y| c.get(x, y).unwrap().ch != ' ');
                let seen: Vec<i32> = (0..160).filter(|&x| on(x)).collect();
                most = most.max(seen.len());
                if seen.len() > 30 {
                    cols.push(seen.iter().sum::<i32>() / seen.len() as i32);
                }
                if e.done() {
                    break;
                }
            }
            assert!(most > 100, "{most} columns");
            let moved = cols.last().unwrap() - cols[0];
            assert!(moved.abs() > 40, "{cols:?}");
            assert!(e.done());
        }
    }
}
