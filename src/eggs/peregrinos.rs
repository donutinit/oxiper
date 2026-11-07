//! On the twelfth of December pilgrims walk and cycle to the Basílica de
//! Guadalupe, carrying the Virgen with them.
//!
//! In the days before the twelfth the roads to the Tepeyac fill with
//! peregrinos: families, whole parishes and cyclists who set out from towns
//! days away, in their best clothes, walking at the side of the road with the
//! Virgen on their backs, and a procession of them goes up to the Basílica
//! with roses, veladoras and songs. This draws it in original oxiper pixel art
//! that grows with the screen. The Basílica rises from the ground at the far
//! end, its windows lit, with veladoras burning on its steps and roses
//! at its door. The pilgrims come from the other side, one after another, in
//! straw hats and rebozos, the load rocking as they step: a man with an
//! estandarte on a tall pole, the Virgen on its cloth between gold fringes
//! swaying above him; a woman and a man with a framed image of the Virgen
//! strapped to their backs; a boy with a veladora cupped in his hands, its
//! flame flickering; and, on bicycles in the lane behind them with their wheels
//! turning and their legs pedaling, a woman with her cuadro on her back, a man
//! with an estandarte tied behind the saddle and a basket of roses on the
//! handlebars. As many come as the screen has room for, in either direction;
//! they cross in front of the Basílica and the road empties, and the Basílica
//! sinks back into the ground. All within 30 s.

use super::blit::hash;
use super::guadalupe::{Palette, Place, RAMO, fit_down};
use super::{Egg, Info};
use crate::fiesta::Kind;
use crate::render::Canvas;
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "peregrinos",
    start,
    fiesta: Kind::Navidad,
    when: |fiesta| fiesta.guadalupe(),
};

/// When things happen, in seconds: the Basílica is risen, the first walker
/// comes on and the next ones follow `WALKERS_GAP` apart, the first bicycle
/// comes on and the next ones follow `BIKES_GAP` apart, the last has crossed,
/// the Basílica sinks and it is over.
const RISEN: f32 = 3.0;
const FIRST: f32 = 0.8;
const WALKERS_GAP: f32 = 3.2;
const FIRST_BIKE: f32 = 6.0;
const BIKES_GAP: f32 = 3.4;
const CROSSED: f32 = 24.8;
const SINKING: f32 = 25.0;
const END: f32 = 28.0;
/// The slowest a walker goes, in art pixels a second, and how much faster
/// a bicycle is.
const SLOWEST: f32 = 9.0;
const BIKE_PACE: f32 = 1.8;

/// How tall the tallest thing is (an estandarte's cross, on the far lane), in
/// art pixels, and how far beyond the screen a pilgrim starts and ends.
const HEIGHT: i32 = 60;
const MARGIN: f32 = 34.0;
/// How far a walker goes in one step (two strides), in art pixels, how big the
/// Basílica is next to a pilgrim and how far behind the road it stands.
const STRIDE: f32 = 15.0;
const SIZE: f32 = 0.78;
const DEPTH: f32 = 6.0;
/// How far the far lane, where the bicycles go, is behind the road.
const LANE: f32 = 3.0;

/// A man's head from the side, facing right, in a straw hat.
const MAN: &[&str] = &[
    "....hhhhh....",
    "...hhhhhhh...",
    "...uuuuuuu...",
    ".hhhhhhhhhhh.",
    "..uuuuuuuuu..",
    "...NKKKKK....",
    "...NKKKnKK...",
    "...NKKKKKkK..",
    "....KKKKKK...",
    ".....KKKKk...",
    ".....KKKK....",
];
/// A woman's head from the side, her hair gathered back.
const WOMAN: &[&str] = &[
    "....NNNNN....",
    "...NNNNNNN...",
    "...NNNNNNKK..",
    "...NNNKKKKKK.",
    "...NNNKKnKKK.",
    "...NNNKKKKkK.",
    "....NNKKKKK..",
    ".....NKKKKk..",
    ".....KKKK....",
];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Load {
    /// A banner of the Virgen on a pole.
    Estandarte,
    /// A framed image on the back.
    Cuadro,
    /// A ramo of roses in the arms.
    Ramo,
    /// A veladora cupped in the hands.
    Veladora,
}

/// Who walks, in turn: a woman, a child, and what they carry.
const WALKERS: [(bool, bool, Load); 5] = [
    (false, false, Load::Estandarte),
    (true, false, Load::Ramo),
    (false, true, Load::Veladora),
    (false, false, Load::Cuadro),
    (true, false, Load::Cuadro),
];
/// Who rides, in turn: a woman and what they carry.
const RIDERS: [(bool, Load); 4] = [
    (true, Load::Cuadro),
    (false, Load::Estandarte),
    (true, Load::Ramo),
    (false, Load::Ramo),
];
/// Shirts and their shade, rebozos, skirts, trousers, banners and bicycles.
const SHIRTS: [(u8, u8); 5] = [
    (b'A', b'a'),
    (b'B', b'b'),
    (b'M', b'm'),
    (b'G', b'g'),
    (b'T', b't'),
];
const REBOZOS: [u8; 4] = *b"BMTI";
const SKIRTS: [u8; 5] = *b"GMIBP";
const TROUSERS: [u8; 4] = *b"bzau";
const CLOTHS: [u8; 4] = *b"MBTG";
const BIKES: [u8; 5] = *b"MBLGP";

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Peregrinos::new(w, h, rng))
}

struct Peregrinos {
    t: f32,
    /// Going right to left?
    left: bool,
    h: i32,
    /// Screen pixels to an art pixel, and the screen's width in art pixels.
    k: f32,
    wide: f32,
    /// How many walk and how many ride.
    walkers: usize,
    riders: usize,
    seed: u32,
}

/// One pilgrim: who, what they carry, how they look and when they come.
#[derive(Clone, Copy)]
struct Pilgrim {
    bike: bool,
    woman: bool,
    child: bool,
    load: Load,
    look: u32,
    enter: f32,
    speed: f32,
}

impl Pilgrim {
    /// The color of the shirt (a woman's, her rebozo) and its shade.
    fn shirt(&self) -> (u8, u8) {
        let i = (self.look >> 3) as usize;
        if self.woman {
            (REBOZOS[i % REBOZOS.len()], b'q')
        } else {
            SHIRTS[i % SHIRTS.len()]
        }
    }

    /// The color of the estandarte's cloth.
    fn cloth(&self) -> u8 {
        CLOTHS[(self.look >> 6) as usize % CLOTHS.len()]
    }

    /// The color of the trousers.
    fn trousers(&self) -> u8 {
        TROUSERS[(self.look >> 9) as usize % TROUSERS.len()]
    }
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

impl Peregrinos {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Peregrinos {
        let mut rng = fastrand::Rng::with_seed(rng.u64(..));
        let k = fit_down(h, w, HEIGHT, 100.0);
        let wide = w as f32 / k;
        Peregrinos {
            t: 0.0,
            left: rng.bool(),
            h: h as i32,
            k,
            wide,
            walkers: ((wide / 50.0).round() as usize).clamp(3, WALKERS.len()),
            riders: ((wide / 50.0).round() as usize).clamp(2, RIDERS.len()),
            seed: rng.u32(..),
        }
    }

    /// How far the Basílica has risen: 0 under the ground, 1 standing.
    fn rise(&self) -> f32 {
        let up = ease_out(self.t / RISEN);
        let down = ((self.t - SINKING) / (END - SINKING)).clamp(0.0, 1.0);
        (up - down * down).clamp(0.0, 1.0)
    }

    /// How many pilgrims come.
    fn n(&self) -> usize {
        self.walkers + self.riders
    }

    /// Pilgrim `i`: the walkers first, then the cyclists. All the walkers
    /// keep one pace and all the cyclists another, so that nobody catches up
    /// with anybody in their lane, and the last of each has crossed by
    /// `CROSSED`.
    fn pilgrim(&self, i: usize) -> Pilgrim {
        let road = self.wide + 2.0 * MARGIN;
        let look = (hash(self.seed, i as u32) * 65536.0) as u32;
        if i < self.walkers {
            let (woman, child, load) = WALKERS[i];
            let last = FIRST + (self.walkers - 1) as f32 * WALKERS_GAP;
            Pilgrim {
                bike: false,
                woman,
                child,
                load,
                look,
                enter: FIRST + i as f32 * WALKERS_GAP,
                speed: (road / (CROSSED - last)).max(SLOWEST),
            }
        } else {
            let k = i - self.walkers;
            let (woman, load) = RIDERS[k];
            let last = FIRST_BIKE + (self.riders - 1) as f32 * BIKES_GAP;
            Pilgrim {
                bike: true,
                woman,
                child: false,
                load,
                look,
                enter: FIRST_BIKE + k as f32 * BIKES_GAP,
                speed: (road / (CROSSED - last)).max(SLOWEST * BIKE_PACE),
            }
        }
    }

    /// How far along the road a pilgrim is, from where he came on, in art
    /// pixels (negative before the screen, over `wide` after it).
    fn along(&self, p: &Pilgrim) -> f32 {
        p.speed * (self.t - p.enter) - MARGIN
    }
}

impl Egg for Peregrinos {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::guadalupe(theme);
        let (k, ground) = (self.k, self.h as f32 * 2.0);
        let rise = self.rise();
        // The Basílica at the far end of the road, sunk until it rises.
        if rise > 0.0 {
            let x = if self.left { 40.0 } else { self.wide - 40.0 };
            let sunk = (1.0 - rise) * (HEIGHT as f32 * k + 14.0);
            let far = Place {
                pal: &pal,
                k,
                s: 1.0,
                ox: x * k,
                ground: ground - DEPTH * k + sunk,
            }
            .scaled(SIZE);
            far.basilica(c, 0.0, 0.55 * rise);
            far.offerings(c, self.t);
        }
        // The bicycles on the far lane first, then the walkers on the road.
        for far in [true, false] {
            for i in (0..self.n()).rev() {
                let p = self.pilgrim(i);
                let u = self.along(&p);
                if p.bike != far || self.t < p.enter || !(-MARGIN..self.wide + MARGIN).contains(&u)
                {
                    continue;
                }
                let (ox, s) = if self.left {
                    ((self.wide - u) * k, -1.0)
                } else {
                    (u * k, 1.0)
                };
                let lane = if far { LANE * k } else { 0.0 };
                let at = Place {
                    pal: &pal,
                    k,
                    s,
                    ox,
                    ground: ground - lane,
                };
                let at = if p.child { at.scaled(0.72) } else { at };
                let spin = u / STRIDE * TAU;
                if p.bike {
                    at.cyclist(c, &p, u / 7.0, self.t);
                } else {
                    at.walker(c, &p, spin, self.t);
                }
            }
        }
    }

    fn done(&self) -> bool {
        self.t > END || self.t > 29.5
    }
}

impl Place<'_> {
    /// An estandarte: a banner of cloth in the color of the letter `cloth`
    /// with gold edges and fringe, hanging from a crossbar at the top of a pole
    /// that is topped by a cross, the Virgen on it; `pole` is the local `x` of
    /// the pole and `t` makes the cloth sway (the pole itself is [`Self::pole`]).
    fn banner(&self, c: &mut Canvas, (pole, t, cloth): (f32, f32, u8)) {
        let pal = self.pal;
        let sway = 0.9 * (t * 2.2 + pole).sin();
        self.block(c, (pole - 11.5, 50.0), (23.0, 1.4), pal.of(b'u'));
        self.block(c, (pole - 12.5, 49.6), (1.6, 2.0), pal.of(b'Y'));
        self.block(c, (pole + 10.9, 49.6), (1.6, 2.0), pal.of(b'Y'));
        for r in 0..12 {
            let y = 48.0 - r as f32 * 2.0;
            let off = sway * (r as f32 / 12.0);
            self.block(c, (pole - 9.5 + off, y), (19.0, 2.0), pal.of(cloth));
            self.block(c, (pole - 9.5 + off, y), (1.0, 2.0), pal.of(b'Y'));
            self.block(c, (pole + 8.5 + off, y), (1.0, 2.0), pal.of(b'Y'));
        }
        self.block(c, (pole - 9.5, 49.0), (19.0, 1.0), pal.of(b'Y'));
        for i in 0..10 {
            let off = sway;
            self.block(
                c,
                (pole - 9.5 + off + i as f32 * 2.0, 24.0),
                (1.0, 2.0),
                pal.of(b'Y'),
            );
        }
        self.block(c, (pole - 9.5 + sway, 26.0), (19.0, 1.0), pal.of(b'Y'));
        self.virgen(c, (pole - 5.3 + sway * 0.5, 46.0), 0.62);
        // The cross at the top.
        self.block(c, (pole - 0.5, 51.0), (1.0, 6.0), pal.of(b'Y'));
        self.block(c, (pole - 1.8, 54.5), (3.6, 1.0), pal.of(b'Y'));
    }

    /// The pole of an estandarte held at the hands, from `x0` at the hands up
    /// to `x1` at the crossbar.
    fn pole(&self, c: &mut Canvas, x0: f32, x1: f32) {
        self.limb(c, (x0, 8.0), (x1, 50.0), 1.4, self.pal.of(b'D'));
    }

    /// A walker in profile, facing the way the pilgrims go, `phase` into his
    /// step, carrying his load.
    fn walker(&self, c: &mut Canvas, p: &Pilgrim, phase: f32, t: f32) {
        let pal = self.pal;
        let bob = 0.9 * (0.5 - 0.5 * (phase * 2.0).cos());
        let (shirt, shade) = p.shirt();
        let hip = 13.0 + bob;
        let lean = match p.load {
            Load::Cuadro => 1.6,
            Load::Veladora => 0.2,
            _ => 0.8,
        };
        let shoulder = (0.8 + lean, 24.0 + bob);
        let (cloth, trousers) = (p.cloth(), p.trousers());

        // What hangs behind: the estandarte's banner.
        if p.load == Load::Estandarte {
            self.banner(c, (11.5 + 0.4 * bob, t, cloth));
        }
        // The legs: trousers, or the calves under a skirt; huaraches.
        for j in [1.0f32, 0.0] {
            let psi = phase + j * PI;
            let foot = (
                psi.sin() * if p.woman { 3.0 } else { 3.8 },
                psi.cos().max(0.0) * 2.0,
            );
            if p.woman {
                self.limb(c, (0.2, 6.0), (foot.0, foot.1 + 1.0), 1.6, pal.of(b'K'));
            } else {
                self.limb(c, (0.2, hip), foot, 3.0, pal.of(trousers));
            }
            self.block(c, (foot.0 - 1.0, foot.1), (4.2, 1.4), pal.of(b'u'));
        }
        // The far arm, behind the body.
        let hands = self.hands(p.load, shoulder.1);
        self.limb(
            c,
            (shoulder.0 + 0.5, shoulder.1 - 1.5),
            hands.1,
            2.2,
            pal.of(shade),
        );
        // The body: shirt and belt, or rebozo and skirt.
        let w = if p.woman { 6.8 } else { 7.6 };
        self.limb(
            c,
            (0.0, hip + 0.5),
            (shoulder.0 - 0.3, shoulder.1 - 0.5),
            w,
            pal.of(shirt),
        );
        self.limb(
            c,
            (-2.6, hip + 0.5),
            (shoulder.0 - 2.8, shoulder.1 - 1.0),
            1.4,
            pal.of(shade),
        );
        if p.woman {
            let skirt = SKIRTS[(p.look >> 12) as usize % SKIRTS.len()];
            for r in 0..9 {
                let y = hip - r as f32;
                let half = 3.4 + r as f32 * 0.4;
                let sway = (phase.sin() * 0.7) * (r as f32 / 9.0);
                let band = if r >= 7 { b'Y' } else { skirt };
                self.block(c, (-half + sway, y), (2.0 * half, 1.0), pal.of(band));
            }
            self.limb(
                c,
                (shoulder.0 + 1.5, shoulder.1 - 0.8),
                (shoulder.0 + 1.3, hip + 1.0),
                0.8,
                pal.of(b'Y'),
            );
        } else {
            self.block(c, (-3.8, hip - 0.5), (7.6, 1.3), pal.of(b'u'));
        }
        // The head, and a woman's braid.
        let art = if p.woman { WOMAN } else { MAN };
        let top = shoulder.1 + art.len() as f32 - 1.0;
        self.sprite(c, art, (shoulder.0 - 5.8, top), (0.0, 1.0), |l| pal.get(l));
        if p.woman {
            let sway = 0.8 * phase.sin();
            self.limb(
                c,
                (shoulder.0 - 3.6, shoulder.1 + 2.0),
                (-5.2 + sway, hip + 2.0),
                1.7,
                pal.of(b'N'),
            );
            self.block(c, (-6.0 + sway, hip + 0.8), (1.8, 1.8), pal.of(b'P'));
        }
        // What is in front: the pole, the image, the roses, the veladora.
        match p.load {
            Load::Estandarte => self.pole(c, 10.4, 11.6),
            Load::Cuadro => {
                self.limb(
                    c,
                    (-8.0, 28.0),
                    (shoulder.0 + 1.0, shoulder.1 - 0.5),
                    0.9,
                    pal.of(b'a'),
                );
                self.framed(c, (-11.0 + 0.3 * bob, 30.0), 0.62);
            }
            Load::Ramo => {
                self.sprite(c, RAMO, (1.8, 23.0 + bob), (0.0, 1.2), |l| pal.get(l));
            }
            Load::Veladora => {
                let big = self.scaled(1.7);
                big.veladora(
                    c,
                    (3.4 / 1.7, 15.0 / 1.7 + bob / 1.7),
                    if p.look & 1 == 0 { b'V' } else { b'W' },
                    t + p.look as f32,
                );
            }
        }
        // The near arm and the hands.
        self.limb(
            c,
            (shoulder.0 + 0.5, shoulder.1 - 1.5),
            hands.0,
            2.4,
            pal.of(shirt),
        );
        for h in [hands.1, hands.0] {
            self.block(c, (h.0 - 1.0, h.1 - 1.0), (2.2, 2.2), pal.of(b'K'));
        }
    }

    /// Where the near and the far hand are, for a load.
    fn hands(&self, load: Load, shoulder: f32) -> ((f32, f32), (f32, f32)) {
        match load {
            Load::Estandarte => ((10.6, 14.0), (10.9, 19.5)),
            Load::Cuadro => ((2.6, shoulder - 6.0), (1.6, shoulder - 7.5)),
            Load::Ramo => ((6.4, 15.0), (3.8, 14.6)),
            Load::Veladora => ((6.0, 17.5), (4.4, 16.6)),
        }
    }

    /// A cyclist and her bicycle in profile, the wheels `spin` radians round,
    /// with her load.
    fn cyclist(&self, c: &mut Canvas, p: &Pilgrim, spin: f32, t: f32) {
        let pal = self.pal;
        let frame = pal.of(BIKES[(p.look >> 4) as usize % BIKES.len()]);
        let (shirt, shade) = p.shirt();
        let (cloth, trousers) = (p.cloth(), pal.of(p.trousers()));
        let (rear, front, bb) = ((-10.0, 7.5), (10.0, 7.5), (-0.5, 8.0));
        let (seat, head) = ((-3.5, 18.5), (7.5, 18.5));
        let crank = spin * 1.3;
        let hip = (-3.0, 20.5 + 0.3 * (crank * 2.0).sin());
        let shoulder = (3.0, 30.0);

        // What is behind: the estandarte tied behind the saddle, the image.
        if p.load == Load::Estandarte {
            self.limb(c, (-9.0, 11.0), (-12.0, 50.0), 1.4, pal.of(b'D'));
            self.banner(c, (-12.0, t, cloth));
        }
        if p.load == Load::Cuadro {
            self.framed(c, (-17.0, 40.0), 0.62);
        }
        // The wheels and the frame.
        for center in [rear, front] {
            let mut prev = (center.0 + 7.0, center.1);
            for j in 1..=26 {
                let a = j as f32 / 26.0 * TAU;
                let at = (center.0 + 7.0 * a.cos(), center.1 + 7.0 * a.sin());
                self.limb(c, prev, at, 1.3, pal.of(b'z'));
                prev = at;
            }
            for m in 0..3 {
                let a = spin + m as f32 * PI / 3.0;
                let (dx, dy) = (6.0 * a.cos(), 6.0 * a.sin());
                self.limb(
                    c,
                    (center.0 - dx, center.1 - dy),
                    (center.0 + dx, center.1 + dy),
                    0.5,
                    pal.of(b's'),
                );
            }
            self.block(
                c,
                (center.0 - 0.6, center.1 - 0.6),
                (1.2, 1.2),
                pal.of(b'i'),
            );
        }
        for (a, b) in [
            (rear, bb),
            (bb, seat),
            (seat, head),
            (bb, (7.0, 15.0)),
            (head, front),
            (rear, seat),
        ] {
            self.limb(c, a, b, 1.3, frame);
        }
        self.block(c, (-6.0, 18.5), (5.0, 1.2), pal.of(b'u'));
        self.limb(c, (head.0, head.1), (9.0, 21.5), 1.0, pal.of(b's'));
        self.limb(c, (8.0, 21.5), (11.0, 21.0), 1.0, pal.of(b'u'));
        // A basket of roses on the handlebars.
        if p.load == Load::Ramo {
            self.block(c, (10.0, 22.0), (7.0, 3.5), pal.of(b'D'));
            self.block(c, (10.0, 24.0), (7.0, 0.8), pal.of(b'u'));
            self.sprite(c, RAMO, (9.4, 31.0), (0.0, 0.8), |l| pal.get(l));
        }
        // The far leg, the body, the near leg: she pedals.
        let knee_of = |pedal: (f32, f32)| {
            let (dx, dy) = (pedal.0 - hip.0, pedal.1 - hip.1);
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            let sign = if -dy >= 0.0 { 1.0 } else { -1.0 };
            let off = 3.0;
            (
                hip.0 + dx / 2.0 - dy / len * off * sign,
                hip.1 + dy / 2.0 + dx / len * off * sign,
            )
        };
        let pedals = [0.0f32, PI].map(|j| {
            (
                bb.0 + 3.6 * (crank + j).cos(),
                bb.1 + 3.6 * (crank + j).sin(),
            )
        });
        let leg = |c: &mut Canvas, pedal: (f32, f32)| {
            let knee = knee_of(pedal);
            self.limb(c, hip, knee, 2.8, trousers);
            self.limb(c, knee, pedal, 2.6, trousers);
            self.block(c, (pedal.0 - 1.4, pedal.1 - 0.8), (3.8, 1.4), pal.of(b'u'));
        };
        leg(c, pedals[1]);
        self.limb(
            c,
            (shoulder.0 + 0.5, shoulder.1 - 1.5),
            (9.5, 21.5),
            2.2,
            pal.of(shade),
        );
        self.limb(
            c,
            (hip.0 + 0.2, hip.1 + 0.5),
            (shoulder.0 - 0.3, shoulder.1 - 0.5),
            7.0,
            pal.of(shirt),
        );
        if p.woman {
            self.limb(
                c,
                (-3.8, shoulder.1 - 3.0),
                (-6.0, hip.1 + 2.0),
                1.6,
                pal.of(b'N'),
            );
        }
        leg(c, pedals[0]);
        let art = if p.woman { WOMAN } else { MAN };
        let top = shoulder.1 + art.len() as f32 - 1.0;
        self.sprite(c, art, (shoulder.0 - 5.0, top), (0.0, 1.0), |l| pal.get(l));
        self.limb(
            c,
            (shoulder.0 + 0.5, shoulder.1 - 1.5),
            (10.0, 21.8),
            2.4,
            pal.of(shirt),
        );
        self.block(c, (9.0, 20.7), (2.2, 2.2), pal.of(b'K'));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, rgb};

    fn egg(w: u16, h: u16, left: bool) -> Peregrinos {
        let mut e = Peregrinos::new(w, h, &mut fastrand::Rng::with_seed(5));
        e.left = left;
        e
    }

    fn shows(c: &Canvas, hex: u32) -> bool {
        c.cells()
            .iter()
            .any(|cell| cell.fg == rgb(hex) || cell.bg == rgb(hex))
    }

    #[test]
    fn the_art_is_whole() {
        let pal = Palette::guadalupe(&Theme::classic());
        for art in [MAN, WOMAN] {
            assert!(art.iter().all(|r| r.len() == art[0].len()));
            for l in art.iter().flat_map(|r| r.bytes()) {
                assert!(l == b'.' || pal.get(l).is_some(), "{}", l as char);
            }
        }
        for l in REBOZOS
            .iter()
            .chain(&SKIRTS)
            .chain(&TROUSERS)
            .chain(&CLOTHS)
            .chain(&BIKES)
        {
            assert!(pal.get(*l).is_some(), "{}", *l as char);
        }
        for (a, b) in SHIRTS {
            assert!(pal.get(a).is_some() && pal.get(b).is_some());
        }
    }

    /// The Basílica rises and sinks, and every pilgrim has come on and
    /// crossed before it does.
    #[test]
    fn the_basilica_rises_and_the_road_empties() {
        let mut e = egg(100, 30, false);
        let mut rise = |t: f32| {
            e.t = t;
            e.rise()
        };
        assert_eq!(rise(0.0), 0.0);
        assert!(rise(RISEN) > 0.99);
        assert!(rise(SINKING + 1.0) < rise(SINKING));
        assert_eq!(rise(END), 0.0);
        for (w, h) in [(40, 14), (100, 30), (200, 60), (400, 40), (60, 90)] {
            let e = egg(w, h, false);
            for i in 0..e.n() {
                let p = e.pilgrim(i);
                let leaves = p.enter + (e.wide + 2.0 * MARGIN) / p.speed;
                assert!(leaves <= CROSSED + 0.01, "{w}x{h} {i}: {leaves}");
            }
            // Nobody catches up with the pilgrim ahead in the same lane.
            for i in 1..e.walkers {
                assert!(e.pilgrim(i).speed <= e.pilgrim(i - 1).speed + 1e-3);
            }
        }
    }

    /// Everyone is on show in either direction: the pilgrims' skin and
    /// clothes, the estandarte's gold and the Virgen's mantle, a framed image,
    /// roses, veladora flames, the bicycles' tires; the Basílica; and at the
    /// end nothing is left.
    #[test]
    fn the_pilgrims_cross_and_nothing_is_left() {
        let theme = Theme::classic();
        let wanted = [
            0xd7a066, 0xefe6cf, 0xf2c14e, 0x00796b, 0xa0703a, 0xd81b60, 0xffcc4d, 0x1c1424,
            0xbfa98a, 0x4f8f7a, 0xe57373,
        ];
        for left in [false, true] {
            let mut e = egg(120, 36, left);
            let mut c = Canvas::new(120, 36, Color::Reset);
            let mut seen = [false; 11];
            let (mut least, mut most) = (i32::MAX, 0);
            while !e.done() {
                e.update(0.1);
                c.clear();
                e.draw(&mut c, &theme);
                for (i, hex) in wanted.iter().enumerate() {
                    seen[i] |= shows(&c, *hex);
                }
                // The skin of a pilgrim: where the walkers are.
                for (i, cell) in c.cells().iter().enumerate() {
                    if cell.fg == rgb(0xd7a066) {
                        least = least.min(i as i32 % 120);
                        most = most.max(i as i32 % 120);
                    }
                }
            }
            assert!(seen.iter().all(|&s| s), "{left}: {seen:?}");
            assert!(least < 20 && most > 100, "{left}: {least} to {most}");
            assert!(c.cells().iter().all(|cell| cell.ch == ' '), "{left}");
        }
    }

    /// Every kind of load is carried by someone on a screen the size of a
    /// common terminal.
    #[test]
    fn every_load_is_carried() {
        let e = egg(100, 30, false);
        for load in [Load::Estandarte, Load::Cuadro, Load::Ramo, Load::Veladora] {
            assert!((0..e.n()).any(|i| e.pilgrim(i).load == load), "{load:?}");
        }
        assert!((0..e.n()).any(|i| e.pilgrim(i).bike));
        assert!((0..e.n()).any(|i| e.pilgrim(i).child));
        let wide = egg(300, 40, false);
        assert!(wide.n() >= 8);
    }

    /// A theme other than `classic` paints them with its own colors.
    #[test]
    fn a_theme_gives_its_own_colors() {
        let nord = Theme::builtin("nord").unwrap();
        let mut e = egg(100, 40, false);
        e.t = 9.0;
        let mut c = Canvas::new(100, 40, Color::Reset);
        e.draw(&mut c, &nord);
        assert!(!shows(&c, 0xd81b60) && !shows(&c, 0xbfa98a));
        assert!(c.cells().iter().any(|cell| cell.ch != ' '));
    }
}
