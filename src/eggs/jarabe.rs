//! A charro and a china poblana dance the jarabe tapatío during the Fiestas
//! Patrias.
//!
//! Original oxiper pixel art. The sombrero falls to the floor and the dancers
//! come in from the side, stamping: the charro in black and silver, a gold
//! band on his hat, a red moño, silver botonadura down his trousers; the
//! china poblana with braids tied in green and red ribbons, a blouse
//! embroidered at the neck and the castor skirt in green, white and red with
//! gold lace. They circle the sombrero twice, one in front while the other is
//! behind it, zapateado all the way: heels stamp by turns and the spurs
//! flash. Then he stamps harder while she spins, the stripes of her skirt
//! swirling round it and her hands lifting it wide. At the end she picks up
//! the sombrero and they hide behind it, a hat of silver embroidery in front
//! of their faces, hearts rising, then it drops, they bow and dance away.
//! Notes of the jarabe float up all the while. It comes from either side and
//! is gone within 30 s.

use super::patrias::{Palette, fade};
use super::stage::{Stage, hash, unit};
use super::{Egg, Info};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "jarabe",
    start,
    fiesta: Kind::Patrias,
    when: super::any_day,
};

/// The charro from the hat to the belt, facing us.
const CHARRO: &[&str] = &[
    ".....kkkkkkk.....",
    "....kKKKKKKKk....",
    "....kKSKSKSKk....",
    "....kOOOOOOOk....",
    ".kkkkkkkkkkkkkkk.",
    "kKSKKSKKSKKSKKSKk",
    ".kkkkkkkkkkkkkkk.",
    "......FFFFF......",
    "......FkFkF......",
    "......FFFFF......",
    "......kkkkk......",
    ".......FFF.......",
    "......KWRWK......",
    "....KKKWRWKKK....",
    "...KKSKKWKKSKK...",
    "..KKKSKKWKKSKKK..",
    "..KKKSKKWKKSKKK..",
    ".FKKKSKKOKKSKKKF.",
    "..kkkkkOkkkkkkk..",
];
/// His legs: both feet down, and the left one raised mid-stamp (the right
/// one is the same mirrored).
const LEGS: [&[&str]; 2] = [
    &[
        "...KKKKKKKKKKK...",
        "...KKKKKKKKKKK...",
        "...SKKK...KKKS...",
        "...SKKK...KKKS...",
        "...SKKK...KKKS...",
        "...SKKK...KKKS...",
        "...SKKK...KKKS...",
        "...SKKK...KKKS...",
        "...kkkk...kkkk...",
        "..kkkkk...kkkkk..",
        "..kkkkk...kkkkk..",
        "..ksskk...kkssk..",
    ],
    &[
        "...KKKKKKKKKKK...",
        "...KKKKKKKKKKK...",
        "..SKKK....KKKS...",
        "..SKKK....KKKS...",
        ".SKKK.....KKKS...",
        ".kkkk.....KKKS...",
        ".kkkkk....KKKS...",
        "..ssk.....KKKS...",
        "..........kkkk...",
        ".........kkkkk...",
        ".........kkkkk...",
        ".........kkssk...",
    ],
];
const CHARRO_W: i32 = 17;
const CHARRO_H: i32 = 19 + 12;
/// The china poblana from the ribbon on her hair to her waist.
const CHINA: &[&str] = &[
    ".....RGR.....",
    "...kkkkkkk...",
    "..kkkkkkkkk..",
    "..kkFFFFFkk..",
    "..kkFkFkFkk..",
    "..kkFFFFFkk..",
    "..kkFFRFFkk..",
    "..kk.FFF.kk..",
    "..kkWRGRWkk..",
    ".kkWWWWWWWkk.",
    ".kkWWWRWWWkk.",
    ".RkWWWWWWWkG.",
    "...WWWWWWW...",
    "...RRRRRRR...",
];
const CHINA_W: i32 = 13;
const CHINA_H: i32 = 14 + SKIRT + 2;
/// Her skirt: rows, half-width at the waist and at the hem at rest.
const SKIRT: i32 = 15;
const WAIST: f32 = 3.5;
const HEM: f32 = 10.5;
/// The sombrero on the floor.
const SOMBRERO: &[&str] = &[
    "..........kkkkkkkkkk..........",
    ".........kKKKKKKKKKKk.........",
    ".........kKSKSKSKSKSk.........",
    ".........kOOOOOOOOOOk.........",
    "....kkkkkkkkkkkkkkkkkkkkkkk...",
    "..kkKKKKKKKKKKKKKKKKKKKKKKKkk.",
    ".kKSKKSKKSKKSKKSKKSKKSKKSKKSKk",
    "..kkSSSSSSSSSSSSSSSSSSSSSSSkk.",
    "....kkkkkkkkkkkkkkkkkkkkkkk...",
];
const SOMBRERO_W: i32 = 30;
const SOMBRERO_H: i32 = SOMBRERO.len() as i32;
/// A heart, rising.
const HEART: &[&str] = &[".R.R.", "RRRRR", ".RRR.", "..R.."];
/// Radius of the circle they dance on, and of the hat held up.
const ORBIT: f32 = 21.0;
const DISC: i32 = 12;

/// When things happen, in seconds: they are in by `ENTER`, circle until
/// `ORBIT_END`, dance on the spot until `FINALE_END`, she fetches the
/// sombrero until `LIFTED` and they hide until `DROP`, then they bow at
/// `BOW` and are gone by `END`.
const ENTER: f32 = 3.2;
const ORBIT_END: f32 = 13.2;
const FINALE_END: f32 = 17.0;
const LIFTED: f32 = 19.4;
const DROP: f32 = 23.0;
const BOW: f32 = 24.0;
const END: f32 = 27.6;
/// Stamps per second, and turns around the sombrero per second.
const STAMPS: f32 = 3.6;
const TURNS: f32 = 0.2;

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let st = Stage::new(w, h);
    Box::new(Jarabe {
        t: 0.0,
        left: rng.bool(),
        // The dance is at most this far in from the edge they come from, so
        // even a very wide screen is crossed in time.
        in_from_edge: (st.w / 2).min(100) as f32,
        w: st.w,
        seed: rng.u32(..),
    })
}

struct Jarabe {
    t: f32,
    /// Coming from the left (else from the right, mirrored)?
    left: bool,
    in_from_edge: f32,
    w: i32,
    seed: u32,
}

/// Where a dancer is and what she or he is doing.
struct Pose {
    /// Pixels to the right of the sombrero (in the dance's own left-to-right).
    dx: f32,
    /// In front of (above 0) or behind (below 0) the sombrero.
    z: f32,
    /// Crouching, in pixels.
    crouch: i32,
    /// Turns of her skirt so far, and how wide it flares.
    spin: f32,
    flare: f32,
    /// Stamping (else standing or walking).
    stamping: f32,
}

/// 0 to 1 as `x` goes from `a` to `b`, eased.
fn ease(x: f32, a: f32, b: f32) -> f32 {
    let k = ((x - a) / (b - a)).clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

fn lerp(a: f32, b: f32, k: f32) -> f32 {
    a + (b - a) * k
}

impl Jarabe {
    /// The charro's and the china's poses.
    fn poses(&self) -> [Pose; 2] {
        let t = self.t;
        let edge = -(self.in_from_edge + 26.0);
        let spin_up =
            ease(t, ORBIT_END, ORBIT_END + 1.2) * (1.0 - ease(t, FINALE_END - 0.6, FINALE_END));
        // The charro comes first, to the far side; she follows to the near side.
        let walk_c = ((t) / ENTER).clamp(0.0, 1.0);
        let walk_p = ((t - 1.0) / (ENTER - 1.0)).clamp(0.0, 1.0);
        let (mut cx, mut cz) = (lerp(edge, ORBIT, walk_c), 0.0);
        let (mut px, mut pz) = (lerp(edge, -ORBIT, walk_p), 0.0);
        let mut spin = t * 0.6;
        let mut crouch = [0, 0];
        if t > ENTER {
            // Circling: he goes in front, she behind, and the other way round.
            let a = (t - ENTER).min(ORBIT_END - ENTER) * TURNS * TAU;
            (cx, cz) = (ORBIT * a.cos(), a.sin());
            (px, pz) = (-ORBIT * a.cos(), -a.sin());
            spin = t * 0.6 + spin_up * (t - ORBIT_END) * 5.0;
        }
        if t > FINALE_END {
            // She goes to the sombrero, he to her side; she bends for it.
            let near = ease(t, FINALE_END, FINALE_END + 1.4);
            (px, pz) = (lerp(-ORBIT, -9.0, near), 0.0);
            (cx, cz) = (lerp(ORBIT, 9.0, near), 0.0);
            let bend = ease(t, FINALE_END + 1.2, FINALE_END + 1.6)
                * (1.0 - ease(t, FINALE_END + 1.8, LIFTED));
            crouch[1] = (bend * 5.0) as i32;
        }
        if t > BOW {
            let go = ease(t, BOW + 0.8, END - 0.2);
            let (c0, p0) = (cx, px);
            cx = lerp(c0, edge * 0.9, go);
            px = lerp(p0, edge * 0.9 - 14.0, go);
            let bow = ease(t, BOW, BOW + 0.4) * (1.0 - ease(t, BOW + 0.4, BOW + 0.8));
            crouch = [(bow * 4.0) as i32; 2];
        }
        let finale =
            ease(t, ORBIT_END, ORBIT_END + 0.4) * (1.0 - ease(t, FINALE_END, FINALE_END + 0.3));
        let dancing = ease(t, 0.0, 0.5) * (1.0 - ease(t, FINALE_END + 1.4, FINALE_END + 1.8));
        let flare = 1.0 + 0.1 * (t * 5.0).sin() + 0.45 * spin_up;
        let steps = |walking: bool| if walking { 1.0 } else { dancing };
        [
            Pose {
                dx: cx,
                z: cz,
                crouch: crouch[0],
                spin: 0.0,
                flare: 1.0,
                stamping: steps(true) * (1.0 + finale),
            },
            Pose {
                dx: px,
                z: pz,
                crouch: crouch[1],
                spin,
                flare,
                stamping: steps(true),
            },
        ]
    }

    /// Is the sombrero in her hands (lifted), and how high (0 floor, 1 head)?
    fn lift(&self) -> f32 {
        ease(self.t, FINALE_END + 1.8, LIFTED) * (1.0 - ease(self.t, DROP, DROP + 0.6))
    }

    /// Screen x of the sombrero's middle and the ground row.
    fn origin(&self) -> f32 {
        if self.left {
            self.in_from_edge
        } else {
            (self.w - 1) as f32 - self.in_from_edge
        }
    }
}

impl Egg for Jarabe {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::patrias(theme);
        let st = Stage::new(c.w(), c.h());
        let g = st.h;
        let o = self.origin();
        // Mirrored, the dance runs the other way along the screen.
        let x_of = |dx: f32| (o + if self.left { dx } else { -dx }).round() as i32;
        let poses = self.poses();
        let [charro, china] = &poses;
        // The sombrero falls to the floor first, and bounces.
        let fall = ease(self.t, 0.0, 0.7);
        let bounce = if self.t < 1.1 {
            (1.0 - fall) * 40.0
        } else {
            ((self.t - 0.7) * 9.0).sin().max(0.0) * 2.0 * (-(self.t - 0.7) * 3.0).exp()
        };
        let lifted = self.lift();
        // The floor sombrero, unless she has it in her hands.
        let floor_hat = |c: &mut Canvas| {
            if self.t < FINALE_END + 1.8 || lifted == 0.0 && self.t > DROP {
                let y = g - SOMBRERO_H - bounce.round() as i32;
                st.sprite(
                    c,
                    (x_of(0.0) - SOMBRERO_W / 2, y),
                    SOMBRERO,
                    false,
                    |_| 0,
                    |ch| pal.get(ch),
                );
            }
        };
        // The one behind is drawn first, then the hat, then the one in front.
        let order = if charro.z < china.z { [0, 1] } else { [1, 0] };
        let mut hat_done = false;
        for who in order {
            let front = if who == 0 { charro.z } else { china.z } >= 0.0;
            if front && !hat_done {
                floor_hat(c);
                hat_done = true;
            }
            if who == 0 {
                self.charro(c, &st, &pal, x_of(charro.dx), charro, g);
            } else {
                self.china(c, &st, &pal, x_of(china.dx), china, g);
            }
        }
        if !hat_done {
            floor_hat(c);
        }
        if lifted > 0.0 || (self.t > FINALE_END + 1.8 && self.t < DROP + 0.6) {
            self.held_hat(c, (&st, &pal, theme), (x_of(0.0), g), lifted);
        }
        self.music(c, &st, &pal, theme, x_of(0.0), g);
    }

    fn done(&self) -> bool {
        self.t > END
    }
}

impl Jarabe {
    /// The charro: stamping by turns, hopping, spurs flashing.
    fn charro(&self, c: &mut Canvas, st: &Stage, pal: &Palette, x: i32, pose: &Pose, g: i32) {
        let beat = (self.t * STAMPS * pose.stamping.max(0.3)) as i32;
        let up_foot = beat % 4;
        let frame = usize::from(up_foot % 2 == 0);
        let hop = i32::from(frame == 1);
        let front = i32::from(pose.z > 0.0) * 2;
        let (top, feet) = (g - CHARRO_H - hop + pose.crouch - 1 + front, g - 12 + front);
        // Mirrored for the right foot.
        let flip_legs = up_foot == 2;
        let (lx, ly) = (x - CHARRO_W / 2, feet);
        st.sprite(
            c,
            (lx, ly - pose.crouch),
            LEGS[frame],
            flip_legs,
            |_| 0,
            |ch| pal.get(ch),
        );
        st.sprite(c, (lx, top + 1), CHARRO, false, |_| 0, |ch| pal.get(ch));
        // The spurs flash as a heel lands.
        let land = (self.t * STAMPS * pose.stamping.max(0.3)).fract();
        if frame == 0 && land < 0.25 {
            let side = if beat % 4 == 1 { -5 } else { 5 };
            st.px(c, lx + 8 + side, g - 2 + front, pal.of(b'S'));
            st.px(c, lx + 8 + side + 1, g - 3 + front, pal.of(b'W'));
        }
    }

    /// The china poblana: stripes swirling round her skirt, hands holding it.
    fn china(&self, c: &mut Canvas, st: &Stage, pal: &Palette, x: i32, pose: &Pose, g: i32) {
        let beat = self.t * STAMPS * pose.stamping.max(0.3);
        let hop = i32::from((beat as i32) % 2 == 0);
        let front = i32::from(pose.z > 0.0) * 2;
        let top = g - CHINA_H - hop + pose.crouch + front;
        let (lx, ly) = (x - CHINA_W / 2, top);
        st.sprite(c, (lx, ly), CHINA, false, |_| 0, |ch| pal.get(ch));
        // Her skirt, a cylinder of stripes turning round, swaying at the hem.
        let y0 = ly + 14;
        let sway = (self.t * 4.0).sin();
        let hem_hw = HEM * pose.flare;
        let mut hand = (0, 0);
        for r in 0..SKIRT {
            let f = r as f32 / (SKIRT - 1) as f32;
            let hw = WAIST + (hem_hw - WAIST) * f.powf(0.85);
            let off = (sway * 1.6 * f * f
                + (self.t * 7.0 + f * 4.0).sin() * pose.flare.max(1.0).powi(2) * 0.5 * f)
                .round() as i32;
            let half = hw.round() as i32;
            for dxp in -half..=half {
                let u = dxp as f32 / hw.max(1.0);
                // Around the cylinder: three colors in two stripes each.
                let around = (u.clamp(-1.0, 1.0).asin() / PI + 0.5 + pose.spin) * 6.0;
                let idx = (around.floor() as i32).rem_euclid(3);
                let shade = u.abs() > 0.78;
                let ch = match (idx, shade) {
                    (0, false) => b'G',
                    (0, true) => b'g',
                    (1, false) => b'W',
                    (1, true) => b'w',
                    (_, false) => b'R',
                    (_, true) => b't',
                };
                let lace = r >= SKIRT - 2;
                let col = if lace {
                    // Gold lace with sequins catching the light.
                    if hash((dxp + 99) as u32, (self.t * 6.0) as u32 + r as u32).is_multiple_of(5) {
                        pal.of(b'W')
                    } else {
                        pal.of(b'O')
                    }
                } else {
                    pal.of(ch)
                };
                st.px(c, x + dxp + off, y0 + r, col);
            }
            if r == 3 {
                hand = (half + off, y0 + r);
            }
        }
        // Her feet peek out, stepping.
        let step = (beat as i32) % 2;
        for (i, dxp) in [-3, 2].into_iter().enumerate() {
            let lift = i32::from(i as i32 == step);
            st.rect(c, x + dxp, y0 + SKIRT - lift, 2, 2 - lift, pal.of(b'k'));
        }
        // Her arms: from the shoulders out to the skirt she holds up.
        let sleeve = pal.of(b'W');
        for side in [-1, 1] {
            let (sx, sy) = (x + side * 4, ly + 10);
            let (hx, hy) = (x + side * (hand.0 + 1), hand.1);
            st.line(c, (sx, sy), (hx, hy), 1, sleeve);
            st.px(c, hx, hy, pal.of(b'F'));
        }
    }

    /// The sombrero held up: a disc of silver and gold embroidery seen from
    /// above, in front of their faces when it is all the way up.
    fn held_hat(
        &self,
        c: &mut Canvas,
        (st, pal, theme): (&Stage, &Palette, &Theme),
        (x, g): (i32, i32),
        lifted: f32,
    ) {
        let head = g - CHARRO_H + 6;
        let floor = g - SOMBRERO_H / 2 - 4;
        let wobble = if lifted >= 1.0 {
            ((self.t * 6.0).sin() * 1.2).round() as i32
        } else {
            0
        };
        let (cx, cy) = (
            x + wobble,
            lerp(floor as f32, head as f32, lifted).round() as i32,
        );
        // Rings of embroidery, with spokes of silver stitching.
        let (black, charcoal, silver, gold) =
            (pal.of(b'k'), pal.of(b'K'), pal.of(b'S'), pal.of(b'O'));
        for dy in -DISC..=DISC {
            for dxp in -DISC..=DISC {
                let d = ((dxp * dxp + dy * dy) as f32).sqrt();
                if d > DISC as f32 + 0.3 {
                    continue;
                }
                let spoke =
                    (((dy as f32).atan2(dxp as f32) / TAU + 0.5) * 8.0 + 0.5).floor() as i32 % 2
                        == 0;
                let col = match d {
                    d if d < 2.5 => gold,
                    d if d < 4.5 => black,
                    d if d < 6.0 => silver,
                    d if d < 9.0 => {
                        if spoke {
                            charcoal
                        } else {
                            silver
                        }
                    }
                    d if d < 10.6 => charcoal,
                    d if d < 11.6 => gold,
                    _ => black,
                };
                st.px(c, cx + dxp, cy + dy, col);
            }
        }
        // Hearts rise from behind it while they are hidden.
        if self.t > LIFTED && self.t < DROP + 1.0 {
            for i in 0..8u32 {
                let born = LIFTED + 0.2 + i as f32 * 0.45;
                let age = self.t - born;
                if !(0.0..2.6).contains(&age) {
                    continue;
                }
                let sway = (age * 3.0 + i as f32).sin() * 3.0;
                let hx = cx + ((unit(i, self.seed) - 0.5) * 14.0) as i32 + sway.round() as i32;
                let hy = cy - DISC - 2 - (age * 8.0) as i32;
                st.sprite(
                    c,
                    (hx - 2, hy),
                    HEART,
                    false,
                    |_| 0,
                    |ch| {
                        (ch == b'R')
                            .then(|| fade(theme, pal.of(b'r'), ((age - 1.6) / 1.0).max(0.0)))
                    },
                );
            }
        }
    }

    /// Notes of the jarabe rising round them.
    fn music(&self, c: &mut Canvas, st: &Stage, pal: &Palette, theme: &Theme, x: i32, g: i32) {
        let on = ease(self.t, 1.0, 2.0) * (1.0 - ease(self.t, DROP, DROP + 2.0));
        if on <= 0.0 {
            return;
        }
        for i in 0..14u32 {
            let born = 1.5 + i as f32 * 1.7;
            let age = self.t - born;
            if !(0.0..3.0).contains(&age) {
                continue;
            }
            let d = hash(i, self.seed ^ 0x77);
            let nx = x
                + ((unit(d, 1) - 0.5) * 60.0) as i32
                + ((age * 2.5 + i as f32).sin() * 2.0) as i32;
            let ny = g - CHARRO_H - 4 - (age * 4.5) as i32 - (d % 5) as i32;
            let (col, row) = st.cell(nx, ny);
            let glyph = ['♪', '♫', '♩', '♬'][d as usize % 4];
            let tone = pal.of(b"lOrW"[(d >> 4) as usize % 4]);
            let k = (age / 3.0 - 0.6).max(0.0) / 0.4;
            c.set_attr(col, row, glyph, fade(theme, tone, k.min(1.0)), BOLD);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        assert!(CHARRO.iter().all(|r| r.len() as i32 == CHARRO_W));
        for l in LEGS {
            assert!(l.iter().all(|r| r.len() as i32 == CHARRO_W));
            assert_eq!(l.len(), 12);
        }
        assert_eq!(CHARRO.len() + LEGS[0].len(), CHARRO_H as usize);
        assert!(CHINA.iter().all(|r| r.len() as i32 == CHINA_W));
        assert!(SOMBRERO.iter().all(|r| r.len() as i32 == SOMBRERO_W));
        assert!(HEART.iter().all(|r| r.len() == 5));
        const { assert!(ENTER < ORBIT_END && ORBIT_END < FINALE_END && FINALE_END + 1.8 < LIFTED) };
        const { assert!(LIFTED < DROP && DROP < BOW && BOW < END && END < 29.5) };
    }

    /// The pair dances in, circles, hides behind the hat and leaves, from
    /// either side, and the hat is on the floor again at the end.
    #[test]
    fn dances_around_the_sombrero() {
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
            assert!(most > 300, "{most} cells");
        }
    }
}
