//! A cartonería Judas hangs over the screen during the Quema de Judas, and
//! burns.
//!
//! On Sábado de Gloria Mexican towns burn Judases: figures of cartonería
//! (papier-mâché over a frame of reeds), usually a red devil, hung from a
//! rope across the street, stuffed with firecrackers and fireworks and set
//! alight to the crowd's delight. Original oxiper pixel art: a red and black
//! devil with horns, yellow eyes, fangs, a goatee and a spade-tipped tail,
//! in a jacket with a collar of papel de china fringe, a trinche (pitchfork)
//! in his hand and strings of firecrackers tied round his arms, legs and
//! belt; the red shows the paper strips glued over one another and their
//! seams. He is lowered on his rope and sways there; a fuse hanging from
//! his boots is lit and its spark climbs it; the firecrackers go off one
//! after the other from the legs up, each with its flash, sparks, puff of
//! smoke and scraps of colored paper, and a "¡PUM!" now and then. He begins
//! to turn on his rope, faster and faster, arms and legs flying out, and
//! with each volley a piece of him blows off and falls: the pitchfork, a
//! leg, the other, an arm, the other, the tail. At the end the rest of him
//! bursts in one great bang, torso and head in four pieces each, and falls
//! among paper, sparks and a cloud of smoke, leaving his rope swinging. The
//! whole thing is drawn from the time it has run, at any size, and takes
//! about 22 s.

use super::blit::{Xf, blit, block, fit, hash};
use super::quema::{Palette, fade};
use super::{Egg, Info, pixel};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "judas",
    start,
    fiesta: Kind::Judas,
    when: super::any_day,
};

/// The head from the front: horns, yellow slit eyes, fangs and a goatee.
const HEAD_FRONT: &[&str] = &[
    "N...................N",
    "Nk.................kN",
    "Nkk...............kkN",
    ".Nkk.............kkN.",
    ".Nkk.............kkN.",
    "..Nkkk.........kkkN..",
    "..Nkkk.........kkkN..",
    "...Nkkk.RRRRR.kkkN...",
    "...NkkRRRRDRrRRkkN...",
    ".RR.kkRRRrRRRRRkk.RR.",
    ".DRRRkkkRRRRRkkkRDRD.",
    "..RrRrYYkkRkkYYRRRR..",
    "..rRRYYkYDRRYkYYRRR..",
    "...RRRRYYDDDYYRRRR...",
    "...RDRRDRDkDRRRRDR...",
    "...RRRRRRRRRRRRrRR...",
    "....RkWkWkkkWkWkR....",
    ".....kkWkkkkkWkk.....",
    "......DRkkkkkRR......",
    "........RkkkR........",
];
/// The head from behind: horns, black hair.
const HEAD_BACK: &[&str] = &[
    "N...................N",
    "Nk.................kN",
    "Nkk...............kkN",
    ".Nkk.............kkN.",
    ".Nkk.............kkN.",
    "..Nkkk.........kkkN..",
    "..Nkkk.........kkkN..",
    "...Nkkk.RkkkR.kkkN...",
    "...NkkRkkkNkkkRkkN...",
    ".RR.kRNkkkkkkkNRk.RR.",
    ".DRRRkkkkNkNkkkkRDRD.",
    "..RrRkkkkkkkkkkkRRR..",
    "..rRkkkkNkkkNkkkkRR..",
    "...RRkkkkkkkkkkkRR...",
    "...RDkkNkkkkkNkkDR...",
    "...RRRkkkkNkkkkrRR...",
    "....RRRkkkkkkkRRR....",
    ".....RRkkkkkkkRR.....",
    "......DkkkkkkkR......",
    "........kkkkk........",
];
/// The torso from the front: the fringe of papel de china, a jacket with
/// black lapels and yellow buttons, a belt with a buckle.
const TORSO_FRONT: &[&str] = &[
    "PQYGPQYGPGYQPGYQP",
    "YGPQYGPQYQPGYQPGY",
    "RRRRrRRkkkRRRRRRR",
    "DrRRRRRkYkRRDRRRR",
    "rRRRRRkRkRkRRRRRR",
    ".RRRDRkDYRkRRRRR.",
    ".RRRRkRRkRRkRRrR.",
    ".RDRRkRRYRRkRrRR.",
    "..RRkRRRkRrRkRR..",
    "..kkkkkYyYkkkkk..",
    "..kkkkkYyYkkkkk..",
    "...kkkkkkkkkkk...",
    "...kkkkkkkkkkk...",
];
/// The torso from behind.
const TORSO_BACK: &[&str] = &[
    "PQYGPQYGPGYQPGYQP",
    "YGPQYGPQYQPGYQPGY",
    "RRrRrRRRkRRRRRRRR",
    "RrRRRRRRkRRDRRRRR",
    "RRRRRRRRkRRRRRRRR",
    ".RRDRRDRkRRRRRRD.",
    ".RRRRRRRkRRRRRrR.",
    ".DRRRRRRkRDrRRRR.",
    "..RRRRRRkRrRRRR..",
    "..kkkkkyyykkkkk..",
    "..kkkkkyyykkkkk..",
    "...kkkkkkkkkkk...",
    "...kkkkkkkkkkk...",
];
/// An arm, hanging from the shoulder, ending in claws (the right is its
/// mirror image).
const ARM_ART: &[&str] = &[
    "..RRRR.", "..RRRR.", ".RRRrRr", ".DRrRRR", ".RRRRRR", ".RRRRDR", ".RRRRRR", ".RRDRRR",
    ".RRRRRR", ".RRRRRR", ".RRRRRr", ".RDrRRR", ".YYYYY.", ".yYYYy.", ".kkkkk.", ".kkkkk.",
    ".k.k.k.", ".W.W.W.",
];
/// A leg: black trousers with a red stripe, a boot and a hoof (the right
/// is its mirror image).
const LEG_ART: &[&str] = &[
    ".dDDDDk.", ".dDDrDk.", ".dDDDDk.", ".dDrDDk.", ".dDDDDk.", ".dDDDrk.", ".dDDDDk.", ".dDrDDk.",
    ".dDDDDk.", ".dDDDDk.", ".dDDDDk.", "YYYYYYYY", ".kkkkkk.", ".kNkkkk.", "kkkkkkkk", "kkk.kkkk",
    "kkk.....",
];
/// The tail, ending in a spade.
const TAIL_ART: &[&str] = &[
    ".k...........",
    ".k...........",
    ".k...........",
    ".kk..........",
    "..k..........",
    "..kk.........",
    "...k.........",
    "....k.......R",
    ".....kk...kRR",
    "......kkkkRRR",
    "...........RR",
    "...........DR",
    ".............",
    ".............",
    ".............",
];
/// The trinche, held by its shaft at the lower end.
const FORK_ART: &[&str] = &[
    "y...y...y",
    "Y...Y...Y",
    "Y...Y...Y",
    "Y...Y...Y",
    "Y...Y...Y",
    "Y...Y...Y",
    "YYYYYYYYY",
    "YYYyYyYYY",
    "....y....",
    "....Cc...",
    "....Cc...",
    "....Cc...",
    "....c....",
    "....Cc...",
    "....Cc...",
    "....Cc...",
    "....c....",
    "....Cc...",
    "....Cc...",
    "....Cc...",
    "....c....",
    "....Cc...",
    "....Cc...",
    "....Cc...",
    "....c....",
    "....Cc...",
    "....Cc...",
    "....Cc...",
    "....c....",
    "....Cc...",
    "....Cc...",
    "....Cc...",
    "....c....",
    "....Cc...",
    "....Cc...",
    "....Cc...",
    "....c....",
    "....Cc...",
];

/// The parts of the Judas, as the indexes of [`PARTS`].
const FORK: usize = 0;
const TAIL: usize = 1;
const LEG_R: usize = 2;
const LEG_L: usize = 3;
const TORSO: usize = 4;
const ARM_R: usize = 5;
const ARM_L: usize = 6;
const HEAD: usize = 7;

/// One part of him: its art seen from the front and from behind, the point
/// of that art that hangs from the figure, where in the figure that is (in
/// art pixels from the middle of the top of his horns), whether it is the
/// mirror image of its art, and the second at which it blows off.
struct Part {
    front: &'static [&'static str],
    back: &'static [&'static str],
    pivot: (f32, f32),
    hang: (f32, f32),
    mirror: bool,
    breaks: f32,
}

/// The seconds of the show: he is lowered, the fuse is lit, its spark
/// reaches him, he starts to turn, the turning is at its fastest and the
/// last of him bursts; then smoke and paper settle and it is over.
const LOWERED: f32 = 1.4;
const LIT: f32 = 2.4;
const REACHED: f32 = 4.8;
const TURNING: f32 = 5.0;
const FASTEST: f32 = 12.6;
const BURST: f32 = 13.2;
const OVER: f32 = 22.0;

const PARTS: [Part; 8] = [
    Part {
        front: FORK_ART,
        back: FORK_ART,
        pivot: (4.0, 31.0),
        hang: (0.0, 0.0),
        mirror: false,
        breaks: 7.0,
    },
    Part {
        front: TAIL_ART,
        back: TAIL_ART,
        pivot: (1.0, 1.0),
        hang: (0.0, 29.0),
        mirror: false,
        breaks: 11.7,
    },
    Part {
        front: LEG_ART,
        back: LEG_ART,
        pivot: (4.0, 1.0),
        hang: (4.0, 31.0),
        mirror: true,
        breaks: 8.4,
    },
    Part {
        front: LEG_ART,
        back: LEG_ART,
        pivot: (4.0, 1.0),
        hang: (-4.0, 31.0),
        mirror: false,
        breaks: 9.2,
    },
    Part {
        front: TORSO_FRONT,
        back: TORSO_BACK,
        pivot: (8.5, 6.5),
        hang: (0.0, 25.5),
        mirror: false,
        breaks: BURST,
    },
    Part {
        front: ARM_ART,
        back: ARM_ART,
        pivot: (3.5, 1.5),
        hang: (8.0, 21.5),
        mirror: true,
        breaks: 10.2,
    },
    Part {
        front: ARM_ART,
        back: ARM_ART,
        pivot: (3.5, 1.5),
        hang: (-8.0, 21.5),
        mirror: false,
        breaks: 11.0,
    },
    Part {
        front: HEAD_FRONT,
        back: HEAD_BACK,
        pivot: (10.5, 10.0),
        hang: (0.0, 10.0),
        mirror: false,
        breaks: BURST,
    },
];
/// How tall he is from the knot to his boots, and how long his fuse hangs
/// below them, in art pixels.
const HEIGHT: f32 = 47.0;
const FUSE: f32 = 12.0;

/// The firecrackers tied to a part: where in its art, and the seconds within
/// which they all go off.
struct Tied {
    part: usize,
    spots: &'static [(f32, f32)],
    within: (f32, f32),
}

const CRACKERS: [Tied; 6] = [
    Tied {
        part: LEG_R,
        spots: &[(3.0, 3.0), (3.0, 7.0), (3.0, 11.0)],
        within: (4.9, 8.0),
    },
    Tied {
        part: LEG_L,
        spots: &[(4.0, 3.0), (4.0, 7.0), (4.0, 11.0)],
        within: (5.4, 8.8),
    },
    Tied {
        part: ARM_R,
        spots: &[(3.0, 4.0), (3.0, 9.0)],
        within: (7.6, 9.8),
    },
    Tied {
        part: ARM_L,
        spots: &[(3.0, 4.0), (3.0, 9.0)],
        within: (8.0, 10.6),
    },
    Tied {
        part: TORSO,
        spots: &[(3.0, 5.5), (13.0, 5.5), (5.0, 8.5), (11.5, 8.5)],
        within: (6.0, 12.8),
    },
    Tied {
        part: HEAD,
        spots: &[(2.0, 11.5), (18.0, 11.5)],
        within: (10.8, 12.9),
    },
];

/// What shouts from the bigger bangs.
const WORDS: [&str; 6] = ["¡PUM!", "¡PAM!", "¡TRAS!", "¡PIN PAN!", "¡CRAC!", "¡BUM!"];
/// The colors of the scraps of paper, and of the sparks.
const SCRAPS: [u8; 7] = *b"RYPQGWM";
const SPARKS: [u8; 4] = *b"FfFH";

/// A firecracker: when it goes off, on which part, where in its art, and
/// how big a bang.
#[derive(Clone, Copy, Debug)]
struct Pop {
    t: f32,
    part: usize,
    at: (f32, f32),
    size: f32,
}

/// A piece that blows off and falls: one part, or a quarter of the torso
/// or the head (`center` is where its middle was in the art of the part).
struct Piece {
    part: usize,
    art: Vec<&'static str>,
    center: Option<(f32, f32)>,
    t0: f32,
    seed: u32,
}

/// Cuts art into four at row `r` and column `col`; each quarter with the
/// column and row of its top-left corner.
fn quarters(
    art: &'static [&'static str],
    r: usize,
    col: usize,
) -> Vec<(Vec<&'static str>, (usize, usize))> {
    let w = art.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut out = Vec::new();
    for (r0, r1) in [(0, r), (r, art.len())] {
        for (c0, c1) in [(0, col), (col, w)] {
            let rows = art[r0..r1]
                .iter()
                .map(|l| &l[c0.min(l.len())..c1.min(l.len())])
                .collect();
            out.push((rows, (c0, r0)));
        }
    }
    out
}

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(build(w, h, rng))
}

fn build(w: u16, h: u16, rng: &mut fastrand::Rng) -> Judas {
    let mut rng = fastrand::Rng::with_seed(rng.u64(..));
    let k = fit(h, (HEIGHT + FUSE) as i32 + 4, 0.92);
    let hpx = h as f32 * 2.0;
    // The rope: the room that is left over, a third of it above him.
    let rope = ((hpx - (HEIGHT + FUSE) * k) * 0.3).max(3.0 * k);
    // The firecrackers go off in the order of the schedule.
    let mut pops = Vec::new();
    for Tied {
        part,
        spots,
        within: (from, to),
    } in CRACKERS
    {
        for (j, &at) in spots.iter().enumerate() {
            let t = from + (to - from) * (j as f32 + 0.2 + 0.6 * rng.f32()) / spots.len() as f32;
            pops.push(Pop {
                t,
                part,
                at,
                size: 1.0,
            });
        }
    }
    // A bigger bang where each part blows off, and the great one at the end.
    for (part, p) in PARTS.iter().enumerate() {
        if part != TORSO && part != HEAD {
            pops.push(Pop {
                t: p.breaks,
                part,
                at: p.pivot,
                size: 2.2,
            });
        }
    }
    for (part, at) in [
        (TORSO, (8.5, 6.5)),
        (TORSO, (3.0, 9.0)),
        (TORSO, (14.0, 3.0)),
        (HEAD, (10.5, 12.0)),
        (HEAD, (5.0, 6.0)),
        (HEAD, (16.0, 6.0)),
    ] {
        pops.push(Pop {
            t: BURST,
            part,
            at,
            size: 4.5,
        });
    }
    // The pieces: whole parts, and quarters of the torso and the head.
    let mut pieces = Vec::new();
    for (i, p) in PARTS.iter().enumerate() {
        if i != TORSO && i != HEAD {
            pieces.push(Piece {
                part: i,
                art: p.front.to_vec(),
                center: None,
                t0: p.breaks,
                seed: rng.u32(..),
            });
        }
    }
    for (part, r, col) in [(TORSO, 7, 9), (HEAD, 10, 11)] {
        for (art, (c0, r0)) in quarters(PARTS[part].front, r, col) {
            let (cw, ch) = (
                art.iter().map(|l| l.len()).max().unwrap_or(0) as f32,
                art.len() as f32,
            );
            pieces.push(Piece {
                part,
                art,
                center: Some((c0 as f32 + cw / 2.0, r0 as f32 + ch / 2.0)),
                t0: BURST,
                seed: rng.u32(..),
            });
        }
    }
    Judas {
        t: 0.0,
        k,
        w: w as f32,
        h: hpx,
        rope,
        left: rng.bool(),
        pops,
        pieces,
        seed: rng.u32(..),
    }
}

/// Where the figure is at one moment: where its knot is, how far the rope
/// swings, and how it is turning (the cosine of the turn, so 1 is facing
/// front and -1 is facing the back; and how hard that flings his limbs).
struct Pose {
    knot: (f32, f32),
    swing: f32,
    c: f32,
    fling: f32,
}

struct Judas {
    t: f32,
    /// Screen pixels to an art pixel.
    k: f32,
    /// The screen's size in pixels.
    w: f32,
    h: f32,
    /// How long the rope is once he is lowered.
    rope: f32,
    /// Hung off the left of the middle (else the right).
    left: bool,
    pops: Vec<Pop>,
    pieces: Vec<Piece>,
    seed: u32,
}

/// 0 to 1, smoothly.
fn smooth(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

impl Judas {
    /// Where the rope is tied up, on the top edge.
    fn top(&self) -> (f32, f32) {
        let side = if self.left { -0.12 } else { 0.12 };
        (self.w * (0.5 + side), 0.0)
    }

    /// How fast he turns at `t`, in radians per second, and the angle.
    fn turn(&self, t: f32) -> (f32, f32) {
        let (w0, w1) = (1.2, 21.0);
        let d = FASTEST - TURNING;
        let before = 0.6 * (TURNING * 1.3).sin();
        if t < TURNING {
            return (0.0, 0.6 * (t * 1.3).sin());
        }
        let tau = t - TURNING;
        if tau < d {
            (
                w0 + (w1 - w0) * (tau / d).powi(2),
                before + w0 * tau + (w1 - w0) * tau.powi(3) / (3.0 * d * d),
            )
        } else {
            (w1, before + w0 * d + (w1 - w0) * d / 3.0 + w1 * (tau - d))
        }
    }

    fn pose(&self, t: f32) -> Pose {
        // Lowered from above the screen, easing to a stop.
        let u = (t / LOWERED).clamp(0.0, 1.0);
        let ease = 1.0 - (1.0 - u).powi(3);
        let start = -(HEIGHT + 8.0) * self.k;
        let rope = start + (self.rope - start) * ease;
        // The rope swings, wider as it burns.
        let amp = 0.05 + 0.23 * smooth((t - REACHED) / (BURST - REACHED));
        let amp = amp * smooth(t / LOWERED);
        let swing = amp * (t * TAU / 1.9).sin() + 0.3 * amp * (t * TAU / 0.83 + 1.0).sin();
        let (omega, angle) = self.turn(t);
        let top = self.top();
        Pose {
            knot: (top.0 - rope * swing.sin(), top.1 + rope * swing.cos()),
            swing,
            c: angle.cos(),
            fling: smooth(omega / 21.0),
        }
    }

    /// A point of the figure (in art pixels from his knot, facing front)
    /// on the screen.
    fn fig(&self, p: &Pose, (x, y): (f32, f32)) -> (f32, f32) {
        let (s, c) = p.swing.sin_cos();
        let x = x * p.c;
        (
            p.knot.0 + self.k * (x * c - y * s),
            p.knot.1 + self.k * (x * s + y * c),
        )
    }

    /// How far an arm or a leg is flung out at `t`, for `part`.
    fn flung(&self, p: &Pose, part: usize, t: f32) -> f32 {
        let (base, reach, flail) = match part {
            ARM_R | ARM_L => (0.2, 0.95, 0.1 * (t * 6.0).sin()),
            _ => (0.06, 0.5, 0.08 * (t * 4.0 + 1.0).sin()),
        };
        let out = base + reach * p.fling + flail * (0.3 + p.fling);
        // Outwards is clockwise for what hangs to the left of him.
        if PARTS[part].hang.0 < 0.0 { out } else { -out }
    }

    /// How a part is placed and turned at time `t`, given his `pose`.
    fn place(&self, p: &Pose, part: usize, t: f32) -> Xf {
        let def = &PARTS[part];
        let mut hang = def.hang;
        let mut turn = 0.0;
        match part {
            FORK => {
                // In the right hand, at the end of that arm.
                let a = -self.flung(p, ARM_R, t);
                hang = (
                    PARTS[ARM_R].hang.0 + a.sin() * 15.0,
                    PARTS[ARM_R].hang.1 + a.cos() * 15.0,
                );
                turn = -a * 0.5;
            }
            TAIL => turn = 0.25 * (t * 3.0).sin(),
            LEG_R | LEG_L | ARM_R | ARM_L => turn = self.flung(p, part, t),
            _ => {}
        }
        let back = p.c < 0.0;
        let art = if back { def.back } else { def.front };
        let w = art.iter().map(|r| r.len()).max().unwrap_or(0) as f32;
        let flip = def.mirror != back;
        Xf {
            pivot: (
                if flip { w - def.pivot.0 } else { def.pivot.0 },
                def.pivot.1,
            ),
            at: self.fig(p, hang),
            scale: (self.k * p.c.abs().max(0.12), self.k),
            rot: p.swing + turn,
            flip,
        }
    }

    fn piece_xf(&self, pc: &Piece, t: f32) -> (Xf, f32, (f32, f32)) {
        let pose = self.pose(pc.t0);
        let base = self.place(&pose, pc.part, pc.t0);
        let def = &PARTS[pc.part];
        let w = def.front.iter().map(|r| r.len()).max().unwrap_or(0) as f32;
        let (cw, ch) = pc
            .art
            .iter()
            .fold((0.0f32, pc.art.len() as f32), |(a, _), r| {
                (a.max(r.len() as f32), pc.art.len() as f32)
            });
        let xf = match pc.center {
            Some(center) => Xf {
                pivot: (cw / 2.0, ch / 2.0),
                at: base.map(w, center),
                ..base
            },
            None => base,
        };
        // How fast it was going, from where it was a moment before.
        let before = {
            let pose = self.pose(pc.t0 - 0.05);
            let b = self.place(&pose, pc.part, pc.t0 - 0.05);
            match pc.center {
                Some(center) => b.map(w, center),
                None => b.at,
            }
        };
        let v0 = ((xf.at.0 - before.0) / 0.05, (xf.at.1 - before.1) / 0.05);
        // Out from the knot, and a little at random.
        let (dx, dy) = (xf.at.0 - pose.knot.0, xf.at.1 - pose.knot.1);
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let speed = (28.0 + 40.0 * hash(pc.seed, 1)) * self.k.max(1.0);
        let ang = (hash(pc.seed, 2) - 0.5) * 1.6;
        let (sn, cs) = ang.sin_cos();
        let dir = ((dx * cs - dy * sn) / len, (dx * sn + dy * cs) / len);
        let vel = (
            v0.0 * 0.5 + dir.0 * speed,
            v0.1 * 0.5 + dir.1 * speed - 18.0 * self.k.max(1.0),
        );
        let g = 0.6 * self.h;
        let floor = self.h - 1.0 - ch.max(cw) * 0.5 * self.k;
        // Falls to the ground and lies there.
        let drop = (floor - xf.at.1).max(0.0);
        let land = (-vel.1 + (vel.1 * vel.1 + 2.0 * g * drop).sqrt()) / g;
        let tau = (t - pc.t0).max(0.0);
        let fly = tau.min(land);
        let at = (
            xf.at.0 + vel.0 * fly,
            (xf.at.1 + vel.1 * fly + 0.5 * g * fly * fly).min(floor),
        );
        let spin = (hash(pc.seed, 3) - 0.5) * 14.0;
        let relax = smooth(tau / 0.3);
        let sx = xf.scale.0 + (self.k - xf.scale.0) * relax;
        (
            Xf {
                at,
                scale: (sx, xf.scale.1),
                rot: xf.rot + spin * fly,
                ..xf
            },
            tau - land,
            at,
        )
    }
}

impl Egg for Judas {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::quema(theme);
        let (t, k) = (self.t, self.k);
        let pose = self.pose(t);
        // Everything fades at the very end.
        let gone = smooth((t - (OVER - 1.5)) / 1.5);
        let faded = |col: Color| fade(theme, col, gone);
        let top = self.top();
        // The cable he hangs from, across the top of the screen, and the rope.
        for x in 0..self.w as i32 {
            pixel(c, x, 0, faded(pal.of(b'c')));
        }
        let (rope_a, rope_b) = (top, pose.knot);
        let steps = ((rope_b.1 - rope_a.1).abs().max(1.0)) as i32;
        for s in 0..=steps {
            let f = s as f32 / steps as f32;
            let (x, y) = (
                rope_a.0 + (rope_b.0 - rope_a.0) * f,
                rope_a.1 + (rope_b.1 - rope_a.1) * f,
            );
            // Twisted: the strands alternate along it.
            let twist = ((y as i32 + (t * 6.0) as i32) / 2) % 2 == 0;
            let col = pal.of(if twist { b'C' } else { b'c' });
            pixel(c, x.round() as i32, y as i32, faded(col));
            if k >= 2.0 {
                pixel(c, x.round() as i32 + 1, y as i32, faded(col));
            }
        }
        let color = |l: u8| pal.get(l).map(faded);
        // The fuse, hanging from his boots: the spark climbs it and burns it.
        if t < REACHED {
            let spark = ((t - LIT) / (REACHED - LIT)).clamp(0.0, 1.0);
            let lit = t >= LIT;
            let at = |y: f32| self.fig(&pose, (0.0, y));
            let end = HEIGHT + FUSE * (1.0 - spark);
            let (a, b) = (at(HEIGHT), at(end));
            // The unburnt cord, from his boots down to the spark.
            let n = ((b.1 - a.1).abs().max(1.0)) as i32;
            for s in 0..=n {
                let f = s as f32 / n as f32;
                let (x, y) = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
                let col = pal.of(if s % 3 == 0 { b'C' } else { b'c' });
                pixel(c, x.round() as i32, y.round() as i32, faded(col));
            }
            if lit {
                // The spark, and the sparks it throws off, falling.
                let flick = if (t * 30.0) as i32 % 2 == 0 {
                    b'F'
                } else {
                    b'f'
                };
                let sz = (k.round() as i32).max(1);
                block(
                    c,
                    b.0.round() as i32 - sz / 2,
                    b.1.round() as i32 - sz / 2,
                    sz,
                    pal.of(flick),
                );
                for j in 0..7u32 {
                    let age = hash(self.seed.wrapping_add(j), (t * 12.0) as u32) * 0.4;
                    let pos =
                        at(HEIGHT
                            + FUSE * (1.0 - ((t - age - LIT) / (REACHED - LIT)).clamp(0.0, 1.0)));
                    let side = (hash(j, 9) - 0.5) * 12.0 * k;
                    let (x, y) = (
                        pos.0 + side * age,
                        pos.1 + (40.0 * age * age + 6.0 * age) * k.max(1.0),
                    );
                    let letter = SPARKS[j as usize % SPARKS.len()];
                    let col = fade(theme, pal.of(letter), age / 0.4 * 0.7);
                    pixel(c, x.round() as i32, y.round() as i32, col);
                }
            }
        }
        // The figure, back to front: the tail behind the legs from the
        // front, in front of them from the back.
        let order: [usize; 8] = if pose.c >= 0.0 {
            [TAIL, FORK, LEG_R, LEG_L, TORSO, ARM_R, ARM_L, HEAD]
        } else {
            [FORK, LEG_R, LEG_L, TAIL, TORSO, ARM_R, ARM_L, HEAD]
        };
        for part in order {
            if t >= PARTS[part].breaks {
                continue;
            }
            let def = &PARTS[part];
            let art = if pose.c < 0.0 { def.back } else { def.front };
            blit(c, art, self.place(&pose, part, t), |_| 0, color);
        }
        // The firecrackers still tied on, lit as the fire comes to them.
        for pop in &self.pops {
            if pop.size > 1.0 || t >= pop.t || t >= PARTS[pop.part].breaks {
                continue;
            }
            let xf = self.place(&pose, pop.part, t);
            let w = PARTS[pop.part]
                .front
                .iter()
                .map(|r| r.len())
                .max()
                .unwrap_or(0) as f32;
            let (x, y) = xf.map(w, pop.at);
            let (sx, sy) = (
                (k * 1.5).round().max(1.0) as i32,
                (k * 2.0).round().max(1.0) as i32,
            );
            let (x, y) = (x.round() as i32 - sx / 2, y.round() as i32 - sy / 2);
            let burning = t >= pop.t - 0.5;
            for dy in 0..sy {
                for dx in 0..sx {
                    let letter = if dy == 0 {
                        if burning && (t * 24.0) as i32 % 2 == 0 {
                            b'F'
                        } else {
                            b'W'
                        }
                    } else if dy == sy - 1 {
                        b'D'
                    } else {
                        b'Y'
                    };
                    pixel(c, x + dx, y + dy, faded(pal.of(letter)));
                }
            }
            if burning {
                // The little sparks of the fuse burning down.
                let tick = (t * 20.0) as u32;
                for j in 0..2u32 {
                    let (a, b) = (hash(tick, j + pop.part as u32), hash(j, tick));
                    pixel(
                        c,
                        x + (a * 6.0 * k) as i32 - 2,
                        y - 1 - (b * 5.0 * k) as i32,
                        pal.of(if j == 0 { b'f' } else { b'F' }),
                    );
                }
            }
        }
        // The pieces blown off, falling and lying about.
        for pc in &self.pieces {
            if t < pc.t0 {
                continue;
            }
            let (xf, since, _) = self.piece_xf(pc, t);
            // After a while on the ground each flickers and is gone.
            let rest = since.max(0.0);
            if rest > 3.4 || (rest > 2.4 && (rest * 12.0) as i32 % 2 == 0) {
                continue;
            }
            blit(c, &pc.art, xf, |_| 0, color);
        }
        // The bangs: flash, sparks, smoke, scraps of paper and shouts.
        self.bangs(c, theme, &pal, &pose);
    }

    fn done(&self) -> bool {
        self.t >= OVER
    }
}

impl Judas {
    /// The effects of every firecracker that has gone off.
    fn bangs(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, _pose: &Pose) {
        let (t, k) = (self.t, self.k);
        let kk = k.max(1.0);
        for (i, pop) in self.pops.iter().enumerate() {
            let tau = t - pop.t;
            if tau < 0.0 || tau > 5.0 {
                continue;
            }
            let pose = self.pose(pop.t);
            let xf = self.place(&pose, pop.part, pop.t);
            let w = PARTS[pop.part]
                .front
                .iter()
                .map(|r| r.len())
                .max()
                .unwrap_or(0) as f32;
            let (x, y) = xf.map(w, pop.at);
            let seed = self.seed.wrapping_add(i as u32 * 7919);
            let size = pop.size;
            // Smoke first, under the rest: a ring of stippled puffs, rising,
            // growing, thinning out.
            let life = 1.2 + 0.5 * size;
            if tau < life {
                let f = tau / life;
                let puffs = 1 + (size as u32);
                for j in 0..puffs {
                    let spread = (hash(seed, 20 + j) - 0.5) * 8.0 * size * kk;
                    let rise = (6.0 + 5.0 * hash(seed, 30 + j)) * kk * tau;
                    let r = (1.5 + 3.5 * size.sqrt() * f.sqrt())
                        * kk
                        * (0.7 + 0.5 * hash(seed, 40 + j));
                    let density = 0.6 * (1.0 - f) * (1.0 - f * 0.3);
                    puff(
                        c,
                        (x + spread + 3.0 * kk * tau, y - rise - spread.abs() * 0.3),
                        r,
                        density,
                        seed.wrapping_add(j),
                        [pal.of(b'S'), pal.of(b's')],
                    );
                }
            }
            // The scraps of colored paper, drifting down.
            let scraps = (4.0 * size) as u32;
            for j in 0..scraps {
                let life = 2.6 + 2.0 * hash(seed, 50 + j);
                if tau > life {
                    continue;
                }
                let ang = hash(seed, 60 + j) * TAU;
                let sp = (14.0 + 40.0 * hash(seed, 70 + j)) * kk * size.sqrt().min(2.0);
                let (vx, vy) = (ang.cos() * sp, ang.sin() * sp - 10.0 * kk);
                // They lose their speed to the air and sink slowly, swaying.
                let e = 0.4 * (1.0 - (-tau / 0.4).exp());
                let px = x + vx * e + (tau * 5.0 + j as f32).sin() * 2.0 * kk;
                let py =
                    (y + vy * e + (10.0 + 4.0 * hash(seed, 80 + j)) * kk * tau).min(self.h - 1.0);
                let letter =
                    SCRAPS[(hash(seed, 90 + j) * SCRAPS.len() as f32) as usize % SCRAPS.len()];
                let col = fade(theme, pal.of(letter), smooth((tau / life - 0.7) / 0.3));
                pixel(c, px.round() as i32, py.round() as i32, col);
                if (tau * 7.0 + j as f32) as i32 % 3 == 0 {
                    pixel(c, px.round() as i32 + 1, py.round() as i32, col);
                }
            }
            // The sparks: out and down, each in its own time, cooling from
            // white-gold to orange to red.
            let sparks = (7.0 * size) as u32;
            for j in 0..sparks {
                let life = 0.5 + 0.35 * hash(seed, 100 + j);
                if tau > life {
                    continue;
                }
                let ang = hash(seed, 110 + j) * TAU;
                let sp = (22.0 + 60.0 * hash(seed, 120 + j)) * kk * size.sqrt();
                let px = x + ang.cos() * sp * tau;
                let py = y + ang.sin() * sp * tau + 0.5 * 60.0 * kk * tau * tau;
                let f = tau / life;
                let letter = if f < 0.35 {
                    b'F'
                } else if f < 0.7 {
                    b'f'
                } else {
                    b'H'
                };
                if (tau * 40.0 + j as f32) as i32 % 5 != 0 {
                    pixel(
                        c,
                        px.round() as i32,
                        py.round() as i32,
                        fade(theme, pal.of(letter), f * f * 0.6),
                    );
                }
            }
            // The flash.
            if tau < 0.16 {
                let r = ((1.0 - tau / 0.16) * (3.0 + 2.5 * size) * kk).round() as i32;
                let (fx, fy) = (x.round() as i32, y.round() as i32);
                for d in -r..=r {
                    let col = if d.abs() * 2 < r { b'F' } else { b'f' };
                    pixel(c, fx + d, fy, pal.of(col));
                    pixel(c, fx, fy + d, pal.of(col));
                    if d.abs() * 2 < r {
                        pixel(c, fx + d, fy + 1, pal.of(b'F'));
                        pixel(c, fx + 1, fy + d, pal.of(b'F'));
                    }
                }
                if r > 4 {
                    for d in 1..=r / 2 {
                        for (sx, sy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                            pixel(c, fx + sx * d, fy + sy * d, pal.of(b'f'));
                        }
                    }
                }
            }
            // The shout of the bigger bangs.
            if size >= 2.0 && tau < 0.9 {
                let word = if size > 4.0 {
                    "¡BUUUM!"
                } else {
                    WORDS[(hash(seed, 130) * 6.0) as usize % WORDS.len()]
                };
                let len = word.chars().count() as i32;
                let col = fade(theme, pal.of(b'F'), smooth((tau - 0.5) / 0.4));
                let (cx, cy) = (
                    (x as i32) - len / 2,
                    (y as i32).div_euclid(2) - 2 - (tau * 5.0) as i32,
                );
                c.text_attr(cx, cy, word, col, BOLD);
            }
        }
        // The spark showers while he turns: from the tips of his limbs, flung
        // off along the turn.
        if t > TURNING && t < BURST {
            let level = smooth((t - TURNING) / (FASTEST - TURNING));
            for slot in 0..20u32 {
                let period = 0.5;
                let phase = hash(self.seed, slot) * period;
                let n = ((t - phase) / period).floor();
                let born = n * period + phase;
                let age = t - born;
                if age > 0.45
                    || born < TURNING
                    || hash(slot, (n as u32).wrapping_add(self.seed)) > 0.25 + 0.75 * level
                {
                    continue;
                }
                let pose = self.pose(born);
                let spots = [
                    (0.0, 0.0),
                    (-9.0, 36.0),
                    (9.0, 36.0),
                    (-5.0, 46.0),
                    (5.0, 46.0),
                    (0.0, 22.0),
                ];
                let spot =
                    spots[(hash(slot, n as u32 + 5) * spots.len() as f32) as usize % spots.len()];
                let fling = 1.0 + 2.0 * pose.fling;
                let (px, py) = self.fig(&pose, (spot.0 * fling, spot.1));
                let ang = hash(slot, n as u32) * TAU;
                let sp = (20.0 + 30.0 * hash(n as u32, slot)) * kk * (0.5 + level);
                let x = px + ang.cos() * sp * age;
                let y = py + ang.sin() * sp * age + 30.0 * kk * age * age;
                let f = age / 0.45;
                let letter = if f < 0.4 { b'F' } else { b'f' };
                pixel(
                    c,
                    x.round() as i32,
                    y.round() as i32,
                    fade(theme, pal.of(letter), f * 0.6),
                );
            }
        }
    }
}

/// A puff of smoke: a disc of `radius` pixels around `at` in which each
/// pixel of a checkerboard shows with the chance `density`.
fn puff(c: &mut Canvas, at: (f32, f32), radius: f32, density: f32, seed: u32, colors: [Color; 2]) {
    if density <= 0.0 || radius < 0.5 {
        return;
    }
    let r2 = radius * radius;
    for y in (at.1 - radius).floor() as i32..=(at.1 + radius).ceil() as i32 {
        for x in (at.0 - radius).floor() as i32..=(at.0 + radius).ceil() as i32 {
            let (dx, dy) = (x as f32 + 0.5 - at.0, y as f32 + 0.5 - at.1);
            if dx * dx + dy * dy > r2 || (x + y) & 1 != 0 {
                continue;
            }
            // Thinner toward the edge.
            let h = hash(x as u32 ^ seed, y as u32);
            if h < density * (1.0 - 0.7 * (dx * dx + dy * dy) / r2) {
                pixel(c, x, y, colors[usize::from(h < density * 0.35)]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        let pal = Palette::quema(&Theme::classic());
        for part in &PARTS {
            for art in [part.front, part.back] {
                assert!(art.iter().all(|r| r.len() == art[0].len()), "{art:?}");
                for &b in art.iter().flat_map(|r| r.as_bytes()) {
                    assert!(b == b'.' || pal.get(b).is_some(), "{}", b as char);
                }
            }
        }
    }

    /// Plays an egg up to `secs` and draws it there.
    fn at(secs: f32, w: u16, h: u16, theme: &Theme) -> Canvas {
        let mut e = start(w, h, &mut fastrand::Rng::with_seed(3));
        let mut c = Canvas::new(w, h, Color::Reset);
        let mut t = 0.0;
        while t < secs {
            e.update(0.25_f32.min(secs - t));
            t += 0.25;
        }
        c.clear();
        e.draw(&mut c, theme);
        c
    }

    fn painted(c: &Canvas) -> usize {
        c.cells().iter().filter(|cell| cell.ch != ' ').count()
    }

    /// The show goes in its order: every part blows off after its own
    /// firecrackers have gone off, and the pitchfork, legs, arms and tail
    /// before the rest of him.
    #[test]
    fn the_parts_go_in_order() {
        let order = [FORK, LEG_R, LEG_L, ARM_R, ARM_L, TAIL, TORSO];
        let times = order.map(|p| PARTS[p].breaks);
        assert!(times.windows(2).all(|w| w[0] < w[1]), "{times:?}");
        assert_eq!(PARTS[TORSO].breaks, BURST);
        assert_eq!(PARTS[HEAD].breaks, BURST);
        const { assert!(REACHED > LIT && TURNING >= REACHED && FASTEST < BURST && BURST < OVER) };
        let mut pops = 0;
        for seed in 0..8 {
            let Judas { pops: all, .. } = judas(seed);
            for pop in &all {
                assert!(
                    pop.t >= REACHED && pop.t <= PARTS[pop.part].breaks,
                    "{pop:?}"
                );
                if pop.size <= 1.0 {
                    pops += 1;
                }
            }
        }
        assert!(pops >= 8 * 15, "{pops}");
    }

    fn judas(seed: u64) -> Judas {
        build(100, 40, &mut fastrand::Rng::with_seed(seed))
    }

    /// He turns faster and faster, and never back.
    #[test]
    fn spins_faster_and_faster() {
        let j = judas(1);
        let (mut last_w, mut last_a) = (0.0, f32::MIN);
        let mut t = TURNING;
        while t < OVER {
            let (w, a) = j.turn(t);
            assert!(w >= last_w - 1e-4, "{t}: {w} < {last_w}");
            assert!(a > last_a, "{t}: it turned back");
            (last_w, last_a) = (w, a);
            t += 0.05;
        }
        assert!(last_w > 15.0);
    }

    /// He hangs whole, burns, and is only pieces and smoke at the end: at
    /// the start he is drawn up to the rope, at the end what is left lies
    /// on the ground, and by the last second the screen is clear again.
    #[test]
    fn hangs_burns_and_falls_apart() {
        let theme = Theme::classic();
        let hanging = at(3.0, 100, 40, &theme);
        assert!(painted(&hanging) > 250, "{}", painted(&hanging));
        // Lit and sparking.
        assert!(painted(&at(9.0, 100, 40, &theme)) > 250);
        // After the bang pieces lie about at the bottom of the screen.
        let after = at(BURST + 3.0, 100, 40, &theme);
        let bottom: usize = after
            .cells()
            .iter()
            .skip(100 * 36)
            .filter(|c| c.ch != ' ')
            .count();
        assert!(bottom > 5, "{bottom} cells on the ground");
        // The rope and the cable are all that stay to the end.
        let end = at(OVER - 0.2, 100, 40, &theme);
        assert!(painted(&end) < 200, "{}", painted(&end));
    }

    /// He fills what room the screen has, whatever its size.
    #[test]
    fn scales_with_the_screen() {
        let theme = Theme::classic();
        for (w, h) in [(20, 8), (40, 12), (80, 24), (200, 60), (500, 150)] {
            let c = at(3.0, w, h, &theme);
            assert!(
                painted(&c) > (w as usize * h as usize) / 60,
                "{w}x{h}: {}",
                painted(&c)
            );
        }
        let (small, big) = (judas(1), build(400, 120, &mut fastrand::Rng::with_seed(1)));
        assert!(small.k < big.k, "{} {}", small.k, big.k);
    }
}
