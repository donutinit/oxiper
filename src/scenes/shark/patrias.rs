//! The Fiestas Patrias under the sea. The shark wears tricolor face paint,
//! the flag on its cheeks (verde, blanco, rojo, the eagle on the white), the
//! gill slits in the three colors too; the whale shark's spots turn to
//! confetti in them. The plankton glows in verde, blanco y rojo, flagpoles
//! stand sunk in the sand with their flags rippling in the current and
//! banderitas strung from them, and once in a while a vaquita marina, the
//! tiny porpoise of the Gulf of California that is Mexico's own, passes by.
//!
//! The mes patrio builds up to the Grito on the night of September 15: at
//! first a flag or two and the paint on the cheeks; then the gill slits
//! painted, banderitas strung from the poles, more flagpoles and the vaquita
//! more often; at the Grito a second string of banderitas on each pole, the
//! poles tallest, and the vaquita with her calf.
//!
//! Like the ofrendas, the flagpoles are dealt out by hashing their place (on
//! the same spots), and what moves on its own (the vaquitas) draws from its
//! own generator, so an ordinary day's sea is the same as ever.

use super::art::{Kind, Sprite};
use super::muertos::{Dab, Face, OFR_W, Paint};
use super::sea::Sea;
use super::{Dice, Rgb, Rng, col, hash, rgb_of};
use crate::fiesta;
use crate::scene::Ctx;

/// The flag: verde, blanco, rojo; the eagle's brown, the nopal and the gold.
const VERDE: Rgb = Rgb(0, 104, 71);
const BLANCO: Rgb = Rgb(244, 241, 234);
const ROJO: Rgb = Rgb(206, 17, 38);
const AGUILA: Rgb = Rgb(122, 74, 30);
const NOPAL: Rgb = Rgb(79, 138, 42);
const ORO: Rgb = Rgb(212, 160, 23);

/// The holiday's colors under the theme, and how far it has come.
#[derive(Clone, Copy)]
pub struct Dress {
    pub level: f32,
    /// Verde, blanco, rojo: the theme's accents, or the flag's own under
    /// `classic`. The eagle, the nopal and the gold keep theirs.
    pub tri: [Rgb; 3],
}

impl Dress {
    /// The dress for today, if today is part of the Fiestas Patrias.
    pub fn of(ctx: &Ctx) -> Option<Dress> {
        if !ctx.fiesta.is(fiesta::Kind::Patrias) {
            return None;
        }
        let mut tri = [VERDE, BLANCO, ROJO];
        for (i, c) in tri.iter_mut().enumerate() {
            *c = rgb_of(ctx.pick(i, col(*c))).unwrap_or(*c);
        }
        Some(Dress {
            level: ctx.fiesta.level.clamp(0.0, 1.0),
            tri,
        })
    }

    /// The color of a letter of the face paint or the drawings that has a
    /// fixed one: `g` verde, `w` blanco, `r` rojo, `a` the eagle's brown,
    /// `d` gold, `l` the nopal.
    fn color(&self, ch: u8) -> Option<Rgb> {
        Some(match ch {
            b'g' => self.tri[0],
            b'w' => self.tri[1],
            b'r' => self.tri[2],
            b'a' => AGUILA,
            b'd' => ORO,
            b'l' => NOPAL,
            _ => return None,
        })
    }

    /// One of the three colors as light: lifted towards white, since verde
    /// and rojo are dark against the sea.
    fn glow(&self, i: usize) -> Rgb {
        self.tri[i % 3].lerp(Rgb::WHITE, 0.25)
    }
}

// ----- the face paint -----

/// A shark's tricolor face paint, side on and through a turn: patches of
/// letters like Día de Muertos's, drawn by hand over the drawings. `g` is
/// verde, `w` blanco, `r` rojo, `a` the eagle's brown and `d` its gold; the
/// capitals are the gill slits, painted from the middle of the mes patrio.
pub fn face(kind: Kind) -> Face {
    match kind {
        Kind::GreatWhite => Face {
            side: &GW_SIDE,
            quarter: &GW_QUARTER,
            front: &GW_FRONT,
        },
        // A tiger shark's head is a great white's, give or take the mouth.
        Kind::Tiger => Face {
            side: &TG_SIDE,
            quarter: &GW_QUARTER,
            front: &GW_FRONT,
        },
        Kind::Hammerhead => Face {
            side: &HH_SIDE,
            quarter: &HH_QUARTER,
            front: &HH_FRONT,
        },
        Kind::WhaleShark => Face {
            side: &WS_SIDE,
            quarter: &WS_QUARTER,
            front: &WS_FRONT,
        },
    }
}

/// From this level the capital letters of the face paint (the gill slits) are
/// painted.
const LATER: f32 = 0.5;

// ----- the flagpoles -----

/// Height of a pole above the sand at the start of the mes patrio, and how
/// much taller it gets by the Grito.
const POLE_H: f32 = 54.0;
const POLE_GROWS: f32 = 18.0;
/// The flag's width and height, in the proportions of the real one.
const FLAG_W: i32 = 21;
const FLAG_H: i32 = 12;

/// The escudo on the white: an eagle on a nopal, its beak gold.
#[rustfmt::skip]
const ESCUDO: Sprite = &[
    "..d..",
    ".aaa.",
    "aaaaa",
    ".aaa.",
    "..l..",
    ".lll.",
];

/// How far the flag's column `i` of `FLAG_W` is lifted or dropped at time
/// `t`: a wave running down it, small at the pole and growing towards the
/// free end, the whole flag sagging a little in the water, and the current's
/// gusts (`gust`, around 1) making it more or less.
pub fn ripple(i: i32, t: f32, gust: f32) -> f32 {
    let f = i as f32 / (FLAG_W - 1) as f32;
    gust * (0.3 + 2.2 * f) * (i as f32 * 0.55 - t * 3.0).sin() + 1.6 * f
}

// ----- the vaquita marina -----

/// A vaquita swimming by, with her calf at the height of the holiday.
#[derive(Clone, Copy)]
struct Vaquita {
    x: f32,
    y: f32,
    vx: f32,
    phase: f32,
    calf: bool,
}

/// A vaquita marina, side on, tail up and down: grey back, white belly,
/// the dark rings round the eyes and dark lips that give her away, and the
/// tall dorsal fin.
#[rustfmt::skip]
const VAQUITA: [Sprite; 2] = [
    &[
        ".......dd........",
        "......dBBd.......",
        ".d...dBBBBd......",
        ".dd.BBBBBBBBBBB..",
        "..dBBBBBBBBdddBB.",
        "..dGGGGGGGGdeddGG",
        "..GGGGGGGGGGdddGk",
        "...WWWWWWWWWWWWWk",
        "....WWWWfffWWWW..",
        "..........ff.....",
    ],
    &[
        ".......dd........",
        "......dBBd.......",
        ".....dBBBBd......",
        "....BBBBBBBBBBB..",
        "..dBBBBBBBBdddBB.",
        "..dGGGGGGGGdeddGG",
        ".dGGGGGGGGGGdddGk",
        ".ddWWWWWWWWWWWWWk",
        ".d..WWWWfffWWWW..",
        "..........ff.....",
    ],
];

/// Her calf: smaller, the same markings.
#[rustfmt::skip]
const CALF: [Sprite; 2] = [
    &[
        "....dd.....",
        "...dBBd....",
        ".dBBBBBBBB.",
        "dBBBBBdddB.",
        ".GGGGGdeddG",
        "..WWWWWdddk",
        "...WWfWWWW.",
    ],
    &[
        "....dd.....",
        "...dBBd....",
        "..BBBBBBBB.",
        ".dBBBBBdddB",
        ".dGGGGGdedd",
        ".dWWWWWdddk",
        "..d.WWfWWWW",
    ],
];

const VAQUITA_COLORS: [(u8, Rgb); 7] = [
    (b'B', Rgb(96, 106, 118)),
    (b'G', Rgb(140, 150, 160)),
    (b'W', Rgb(214, 220, 224)),
    (b'd', Rgb(46, 52, 62)),
    (b'e', Rgb(10, 12, 16)),
    (b'f', Rgb(84, 94, 106)),
    (b'k', Rgb(40, 44, 54)),
];

/// The holiday's own state in the sea.
pub struct Patrias {
    pub dress: Dress,
    rng: Rng,
    /// Where the first flagpole stands, just ahead of where the shark was
    /// when the holiday came.
    first: i64,
    vaquitas: Vec<Vaquita>,
    /// Steps until the next vaquita swims by.
    wait: u32,
}

impl Patrias {
    pub fn new(dress: Dress, seed: u32, first: i64) -> Patrias {
        let mut rng = Rng::with_seed(seed as u64 ^ 0x7061_7472_6961_7321);
        let wait = 30 * 12 + rng.range(0, 30 * 12) as u32;
        Patrias {
            dress,
            first,
            rng,
            vaquitas: Vec::with_capacity(2),
            wait,
        }
    }

    /// Everything here is in local coordinates, which shift when the sea is
    /// rebased.
    pub fn shift(&mut self, dx: f32) {
        for v in &mut self.vaquitas {
            v.x -= dx;
        }
    }
}

impl Sea {
    /// Puts on (or takes off) the holiday's dress, keeping what swims in the
    /// sea if it was already on.
    pub fn dress_patrias(&mut self, dress: Option<Dress>) {
        match (dress, self.patrias.as_mut()) {
            (None, _) => self.patrias = None,
            (Some(d), Some(p)) => p.dress = d,
            (Some(d), None) => {
                let first = self.base + (self.shark.x + self.shark.dir * 70.0) as i64;
                self.patrias = Some(Patrias::new(d, self.seed, first));
            }
        }
    }

    /// The middle (local column) of every flagpole within `reach` of local
    /// columns `lo..hi`, with its number: the same spots as the ofrendas, but
    /// fewer of them.
    fn flagpoles(
        &self,
        p: &Patrias,
        range: (f32, f32),
        reach: f32,
        out: &mut [(i64, i32); 4],
    ) -> usize {
        self.spots(0.3 + 0.4 * p.dress.level, p.first, range, reach, out)
    }

    /// Whether a flagpole stands on the floor's cell `x0..x0 + w`, which then
    /// stays bare.
    pub fn flagpole_on(&self, x0: i32, w: i32) -> bool {
        let Some(p) = &self.patrias else {
            return false;
        };
        let mut found = [(0, 0); 4];
        let (lo, hi) = (x0 as f32 - OFR_W as f32, (x0 + w) as f32 + OFR_W as f32);
        self.flagpoles(p, (lo, hi), 0.0, &mut found) > 0
    }

    // ----- the colors of the water -----

    /// How the holiday colors a speck of marine snow: plankton glowing in
    /// verde, blanco and rojo, twinkling, brightest at night. Its color and
    /// strength.
    pub fn patrias_mote(&self, i: usize, near: f32, phase: f32) -> Option<(Rgb, f32)> {
        let p = self.patrias.as_ref()?;
        let roll = (hash(i as i32, 7, 0x7a1d) % 100) as f32;
        if roll >= 30.0 + 50.0 * p.dress.level {
            return None;
        }
        let c = p.dress.glow((hash(i as i32, 8, 0x7a1d) % 3) as usize);
        let dark = 1.0 - self.daylight;
        let twinkle = 0.65 + 0.35 * (self.time() * 1.7 + phase * 3.0).sin();
        Some((c, (0.25 + 0.45 * near) * (0.45 + 0.55 * dark) * twinkle))
    }

    /// The light of lanternfish (and jellyfish) `i`: one of the three colors.
    pub fn patrias_lamp(&self, i: usize) -> Option<Rgb> {
        Some(self.patrias.as_ref()?.dress.glow(i))
    }

    /// The color of a glowing fleck, told apart by the steps it was born with.
    pub fn patrias_fleck(&self, span: u16) -> Option<Rgb> {
        Some(self.patrias.as_ref()?.dress.glow(span as usize))
    }

    /// The colors of the face paint, lit for the shark's depth, for one of
    /// its drawings.
    pub fn patrias_dab(&self, paint: &'static Paint) -> Dab {
        let mut lut = [None; 128];
        let mut later = false;
        if let Some(p) = &self.patrias {
            let y = self.shark.y as i32;
            for ch in *b"gwrad" {
                lut[ch as usize] = p.dress.color(ch).map(|c| self.shade(c, y, 0.08));
            }
            later = p.dress.level >= LATER;
        }
        let spots = (self.shark.kind == Kind::WhaleShark).then_some(&b"gwr"[..]);
        Dab::new(paint, lut, spots, b"ehmMtn", Some(later))
    }

    // ----- the vaquitas -----

    /// One step of what swims by on its own: a vaquita now and then, rolling
    /// gently through the water as porpoises do.
    pub fn update_patrias(&mut self) {
        let Some(mut p) = self.patrias.take() else {
            return;
        };
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        for v in &mut p.vaquitas {
            v.x += v.vx;
            v.phase += 0.07;
            v.y = 62.0 + 14.0 * (v.phase * 0.15).sin();
        }
        // Left far behind, or far ahead when the shark turned back, she is
        // gone.
        p.vaquitas.retain(|v| (v.x - cam).abs() < vw * 1.0 + 80.0);
        p.wait = p.wait.saturating_sub(1);
        if p.wait == 0 {
            let level = p.dress.level;
            if self.daylight < 0.2 || !p.vaquitas.is_empty() {
                // Not at night, and one at a time: try again soon.
                p.wait = 150;
            } else {
                // From just beyond the edge of the picture, ahead of the
                // shark, swimming the way it does but slower, so that it
                // catches her up and passes her.
                let way = self.shark.dir.signum();
                p.vaquitas.push(Vaquita {
                    x: cam + way * (vw * 0.5 + 20.0),
                    y: 62.0,
                    vx: way * p.rng.rangef(0.35, 0.5),
                    phase: p.rng.rangef(0.0, 6.3),
                    // Mother and calf, when the holiday is at its height.
                    calf: level >= 0.9,
                });
                // The less of the holiday, the longer until the next.
                p.wait = (30.0 * (60.0 - 35.0 * level)) as u32 + p.rng.range(0, 600) as u32;
            }
        }
        self.patrias = Some(p);
    }

    /// The vaquitas, on the near side of the shark's plane.
    pub fn draw_vaquitas(&mut self) {
        let Some(p) = self.patrias.take() else {
            return;
        };
        for v in &p.vaquitas {
            let face = v.vx.signum();
            let frame = (v.phase.sin() > 0.0) as usize;
            self.draw_small(VAQUITA[frame], &VAQUITA_COLORS, v.x, v.y, face, 0.12);
            if v.calf {
                // Close behind and below her, a beat out of step.
                let frame = (v.phase.cos() > 0.0) as usize;
                let (cx, cy) = (v.x - face * 13.0, v.y + 5.0);
                self.draw_small(CALF[frame], &VAQUITA_COLORS, cx, cy, face, 0.12);
            }
        }
        self.patrias = Some(p);
    }

    // ----- painting -----

    /// Paints the flagpoles in view onto the floor.
    pub fn paint_patrias(&mut self) {
        let Some(p) = self.patrias.take() else {
            return;
        };
        let mut found = [(0, 0); 4];
        let (lo, hi) = (self.bx as f32, (self.bx + self.bw) as f32);
        let n = self.flagpoles(&p, (lo, hi), OFR_W as f32, &mut found);
        for &(k, cx) in &found[..n] {
            self.flagpole(&p.dress, k, cx);
        }
        self.patrias = Some(p);
    }

    /// One flagpole: stones heaped round its foot, the pole with a gold ball
    /// on top, the flag rippling from it and, as the mes patrio goes on, one
    /// string of banderitas and then two.
    fn flagpole(&mut self, d: &Dress, k: i64, cx: i32) {
        let level = d.level;
        let t = self.time();
        let foot = self.bed(cx as f32) + 1;
        let seed = self.seed ^ k as u32 ^ 0x7061_7472;
        let top = foot - (POLE_H + POLE_GROWS * level) as i32;
        // The flag flies this way.
        let dir = if hash(k as i32, 403, self.seed) & 1 == 0 {
            1
        } else {
            -1
        };

        // The stones that hold it up.
        for dx in -5..=6 {
            let x = cx + dx;
            let f = (dx as f32 - 0.5) / 6.0;
            let height = (4.0 * (1.0 - f * f)).round() as i32;
            for y in foot - height..=self.bed(x as f32) + 1 {
                let h = hash(x, y, seed);
                let stone = if h.is_multiple_of(7) {
                    Rgb(84, 122, 70)
                } else if y == foot - height {
                    Rgb(140, 134, 142)
                } else {
                    Rgb(104, 98, 108).scale(0.85 + 0.2 * (h % 4) as f32 / 3.0)
                };
                self.dot(x, y, stone);
            }
        }

        // Banderitas strung from the pole to the sand: one from the middle
        // of the mes patrio, two at the Grito, the second on the side of the
        // flag, below it.
        if level >= 0.4 {
            self.bunting(
                d,
                (cx + (dir < 0) as i32, top + 8),
                (cx - dir * 22, foot - 6),
                5.0,
            );
        }
        if level >= 0.8 {
            self.bunting(
                d,
                (cx + (dir > 0) as i32, top + 26),
                (cx + dir * 20, foot - 5),
                4.0,
            );
        }

        // The pole, and its gold ball.
        for y in top..foot {
            self.dot(cx, y, Rgb(206, 206, 216));
            self.dot(cx + 1, y, Rgb(150, 150, 162));
        }
        let ball = [(top - 3, 0, 1), (top - 2, -1, 2), (top - 1, 0, 1)];
        for (y, a, b) in ball {
            for x in cx + a..=cx + b {
                let c = if y == top - 3 {
                    ORO.lerp(Rgb::WHITE, 0.4)
                } else {
                    ORO
                };
                self.dot(x, y, c);
            }
        }

        // The flag, ripples running along it, the folds catching the light,
        // the escudo on its white.
        let gust = 0.85 + 0.25 * (t * 0.5 + k as f32).sin();
        let y0 = top + 3;
        for i in 0..FLAG_W {
            let dy = ripple(i, t, gust).round() as i32;
            let fold = 0.84 + 0.16 * (i as f32 * 0.55 - t * 3.0).cos();
            let x = if dir > 0 { cx + 2 + i } else { cx - 1 - i };
            let band = (i * 3 / FLAG_W) as usize;
            for j in 0..FLAG_H {
                self.dot(x, y0 + j + dy, d.tri[band].scale(fold));
            }
            // The escudo is five columns wide, in the middle of the white.
            let e = i - (FLAG_W - 5) / 2;
            if (0..5).contains(&e) {
                for (j, row) in ESCUDO.iter().enumerate() {
                    if let Some(c) = d.color(row.as_bytes()[e as usize]) {
                        self.dot(x, y0 + 3 + j as i32 + dy, c.scale(fold));
                    }
                }
            }
        }
    }

    /// A string from `(x0, y0)` to `(x1, y1)`, sagging by `sag` in the middle,
    /// with banderitas hanging from it, verde, blanco, rojo, fluttering.
    fn bunting(&mut self, d: &Dress, (x0, y0): (i32, i32), (x1, y1): (i32, i32), sag: f32) {
        let t = self.time();
        // One step along the longer way, so the string has no gaps.
        let n = (x1 - x0).abs().max((y1 - y0).abs()).max(1);
        let cord = Rgb(226, 216, 186);
        for s in 0..=n {
            let u = s as f32 / n as f32;
            let x = x0 + ((x1 - x0) as f32 * u).round() as i32;
            let y = y0 + ((y1 - y0) as f32 * u + sag * 4.0 * u * (1.0 - u)).round() as i32;
            self.dot(x, y, cord);
            if s % 8 == 4 {
                let c = d.tri[(s / 8) as usize % 3];
                let sway = ((t * 3.0 + s as f32).sin() * 0.7).round() as i32;
                let bed = self.bed(x as f32);
                // A pennant, five columns narrowing to a point.
                for (j, (a, b)) in [(-2, 2), (-1, 1), (0, 0)].into_iter().enumerate() {
                    if y + 1 + (j as i32) < bed {
                        for px in x + a..=x + b {
                            self.dot(px + sway * j as i32, y + 1 + j as i32, c);
                        }
                    }
                }
            }
        }
    }
}

// ----- the face paint, kind by kind -----

const GW_SIDE: Paint = Paint {
    x: 41,
    y: 8,
    rows: &[
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "G.W.R.G.W.gggwwwrrr",
        "G.W.R.G.W.gggwawrrr",
        "G.W.R.G.W.gggadarrr",
        "G.W.R.G.W.gggwwwrrr",
        "G.W.R.G.W.gggwwwrrr",
        "..........gggwwwrrr",
    ],
};

const TG_SIDE: Paint = Paint {
    x: 41,
    y: 8,
    rows: &[
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "..........gggwwwrrr",
        "G.W.R.G.W.gggwwwrrr",
        "G.W.R.G.W.gggwawrrr",
        "G.W.R.G.W.gggadarrr",
        "G.W.R.G.W.gggwwwrrr",
        "G.W.R.G.W.gggwwwrrr",
        "..........gggwwwrrr",
    ],
};

const HH_SIDE: Paint = Paint {
    x: 41,
    y: 7,
    rows: &[
        "................ggggggg",
        "................ggggggg",
        "................ggggggg",
        "................ggggggg",
        "................ggggggg",
        "................ggggggg",
        "................wwwwwww",
        "G.W.R.G.........wwwawww",
        "G.W.R.G.........wwadaww",
        "G.W.R.G.........wwwwwww",
        "G.W.R.G.........wwwwwww",
        "................rrrrrrr",
        "................rrrrrrr",
        "................rrrrrrr",
        "................rrrrrrr",
        "................rrrrrrr",
        "................rrrrrrr",
    ],
};

const WS_SIDE: Paint = Paint {
    x: 44,
    y: 9,
    rows: &[
        ".........gggwwwrrr",
        ".........gggwwwrrr",
        "G.W.R.G.Wgggwwwrrr",
        "G.W.R.G.Wgggwwwrrr",
        "G.W.R.G.Wgggwawrrr",
        "G.W.R.G.Wgggadarrr",
        "G.W.R.G.Wgggwwwrrr",
        "G.W.R.G.Wgggwwwrrr",
        ".........gggwwwrrr",
        ".........gggwwwrrr",
    ],
};

const GW_QUARTER: Paint = Paint {
    x: 29,
    y: 9,
    rows: &[
        ".......ggwwrrr",
        ".......ggwwrrr",
        ".......ggwwrrr",
        ".......ggwwrrr",
        "G.W.R.Gggwwrrr",
        "G.W.R.Gggwwrrr",
        "G.W.R.Gggwwrrr",
        "G.W.R.Gggwwrrr",
        "G.W.R.Gggwwrrr",
        ".......ggwwrrr",
    ],
};

const HH_QUARTER: Paint = Paint {
    x: 31,
    y: 7,
    rows: &[
        "ggggggggggggg",
        "ggggggggggggg",
        "ggggggggggggg",
        "ggggggggggggg",
        "ggggggggggggg",
        "ggggggggggggg",
        "wwwwwwwwwwwww",
        "wwwwwwwwwwwww",
        "wwwwwwwwwwwww",
        "wwwwwwwwwwwww",
        "wwwwwwwwwwwww",
        "rrrrrrrrrrrrr",
        "rrrrrrrrrrrrr",
        "rrrrrrrrrrrrr",
        "rrrrrrrrrrrrr",
        "rrrrrrrrrrrrr",
        "rrrrrrrrrrrrr",
    ],
};

const WS_QUARTER: Paint = Paint {
    x: 31,
    y: 9,
    rows: &[
        ".....ggwwwrrr",
        ".....ggwwwrrr",
        "G.W.Rggwwwrrr",
        "G.W.Rggwwwrrr",
        "G.W.Rggwwwrrr",
        "G.W.Rggwwwrrr",
        "G.W.Rggwwwrrr",
        "G.W.Rggwwwrrr",
        ".....ggwwwrrr",
        ".....ggwwwrrr",
    ],
};

const GW_FRONT: Paint = Paint {
    x: 9,
    y: 8,
    rows: &[
        "ggggggwwwwwrrrrrr",
        "ggggggwwwwwrrrrrr",
        "ggggggwwwwwrrrrrr",
        "ggggggwwwwwrrrrrr",
        "ggggggwwawwrrrrrr",
        "ggggggwadawrrrrrr",
        "ggggggwwwwwrrrrrr",
        "ggggggwwwwwrrrrrr",
        "ggggggwwwwwrrrrrr",
        "ggggggwwwwwrrrrrr",
    ],
};

const HH_FRONT: Paint = Paint {
    x: 2,
    y: 10,
    rows: &[
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwawwwwwrrrrrrrrrr",
        "ggggggggggwwwwadawwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
    ],
};

const WS_FRONT: Paint = Paint {
    x: 2,
    y: 9,
    rows: &[
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwawwwwwrrrrrrrrrr",
        "ggggggggggwwwwadawwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
        "ggggggggggwwwwwwwwwwwrrrrrrrrrr",
    ],
};

#[cfg(test)]
mod tests {
    use super::super::{INFO, SharkScene};
    use super::*;
    use crate::color::Color;
    use crate::fiesta::Fiesta;
    use crate::render::Canvas;
    use crate::scene::{Opts, Scene};
    use crate::theme::Theme;

    /// The letters the face paint is made of: the lowercase ones are always
    /// there, the capital ones (the gill slits) only from the middle of the
    /// mes patrio.
    const PAINT_LETTERS: &[u8] = b"gwradGWR";

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: Color::Reset,
            seed: 5,
            fiesta,
        }
    }

    fn opts(table: &str) -> Opts {
        Opts::new(INFO.options, table.parse().unwrap())
    }

    /// A fresh sea and its first picture.
    fn first(c: &Ctx) -> (SharkScene, Canvas) {
        let mut s = SharkScene::new(c);
        s.resize(c, 120, 36);
        s.retheme(c);
        s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
        let mut cv = Canvas::new(120, 36, Color::Reset);
        s.draw(c, &mut cv);
        (s, cv)
    }

    /// How many cells differ between two pictures.
    fn differing(a: &Canvas, b: &Canvas) -> usize {
        let mut n = 0;
        for y in 0..a.h() as i32 {
            for x in 0..a.w() as i32 {
                n += (a.get(x, y) != b.get(x, y)) as usize;
            }
        }
        n
    }

    /// Flagpoles in view of the camera.
    fn poles_in_view(s: &SharkScene) -> usize {
        let p = s.sea.patrias.as_ref().unwrap();
        let mut found = [(0, 0); 4];
        let (lo, hi) = (s.sea.bx as f32, (s.sea.bx + s.sea.bw) as f32);
        s.sea.flagpoles(p, (lo, hi), 0.0, &mut found)
    }

    /// From the first day to the Grito there is a flagpole by the shark and
    /// paint on its face; on an ordinary day none of it.
    #[test]
    fn the_holiday_comes_to_the_sea() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts, Fiesta::NONE);
        let (s, plain) = first(&off);
        assert!(s.sea.patrias.is_none());
        let dab = s.sea.patrias_dab(face(s.sea.shark.kind).side);
        assert!(dab.over(55, 13, b'R').is_none(), "paint on an ordinary day");
        assert!(s.sea.patrias_mote(0, 1.0, 0.0).is_none());
        assert!(s.sea.patrias_lamp(0).is_none() && s.sea.patrias_fleck(30).is_none());
        for (m, d) in [(9, 1), (9, 15), (9, 16)] {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (s, cv) = first(&c);
            assert!(s.sea.patrias.is_some(), "{m}-{d}: not dressed");
            assert!(poles_in_view(&s) > 0, "{m}-{d}: no flagpole in sight");
            let dab = s.sea.patrias_dab(face(s.sea.shark.kind).side);
            assert!(dab.over(55, 13, b'R').is_some(), "{m}-{d}: no face paint");
            assert!(differing(&plain, &cv) > 100, "{m}-{d}: looks ordinary");
        }
    }

    /// Other days and other holidays leave the sea as it is.
    #[test]
    fn an_ordinary_day_is_untouched() {
        let theme = Theme::classic();
        let opts = opts("");
        let (a, pa) = first(&ctx(&theme, &opts, Fiesta::NONE));
        for fiesta in [
            Fiesta::of_day(6, 15),
            Fiesta::of_day(8, 20),
            Fiesta::height(fiesta::Kind::Candelaria),
        ] {
            let (b, pb) = first(&ctx(&theme, &opts, fiesta));
            assert!(b.sea.patrias.is_none());
            assert_eq!(differing(&pa, &pb), 0);
            assert_eq!(a.sea.shark.x, b.sea.shark.x);
        }
        // The other holidays dress the sea in their own way only.
        for day in [(12, 24), (11, 2)] {
            let (s, _) = first(&ctx(&theme, &opts, Fiesta::of_day(day.0, day.1)));
            assert!(s.sea.patrias.is_none(), "{day:?}");
        }
        let (s, _) = first(&ctx(&theme, &opts, Fiesta::of_day(9, 15)));
        assert!(s.sea.navidad.is_none() && s.sea.muertos.is_none());
    }

    /// Every face paint lies on its drawing and uses only letters with colors.
    #[test]
    fn the_face_paint_fits() {
        let theme = Theme::classic();
        let opts = opts("");
        let c = ctx(&theme, &opts, Fiesta::of_day(9, 15));
        let d = Dress::of(&c).unwrap();
        for kind in Kind::ALL {
            let (rig, f) = (kind.rig(), face(kind));
            for (sprite, paint) in [
                (rig.side, f.side),
                (rig.quarter, f.quarter),
                (rig.front, f.front),
            ] {
                let w = paint.rows[0].len() as i32;
                assert!(paint.rows.iter().all(|r| r.len() as i32 == w), "{kind:?}");
                assert!(paint.x + w <= sprite[0].len() as i32, "{kind:?}");
                assert!(
                    paint.y + paint.rows.len() as i32 <= sprite.len() as i32,
                    "{kind:?}"
                );
                for ch in paint
                    .rows
                    .iter()
                    .flat_map(|r| r.bytes())
                    .filter(|&c| c != b'.')
                {
                    assert!(PAINT_LETTERS.contains(&ch), "{kind:?}: {}", ch as char);
                    assert!(d.color(ch.to_ascii_lowercase()).is_some());
                }
            }
        }
        // And the drawings of the holiday fit.
        let mut all: Vec<Sprite> = vec![ESCUDO];
        all.extend(VAQUITA);
        all.extend(CALF);
        for sprite in all {
            let w = sprite[0].len();
            for row in sprite {
                assert_eq!(row.len(), w, "{row}");
                for ch in row.bytes().filter(|&c| c != b'.') {
                    assert!(
                        d.color(ch).is_some() || VAQUITA_COLORS.iter().any(|&(k, _)| k == ch),
                        "no color for {}",
                        ch as char
                    );
                }
            }
        }
        assert_eq!(VAQUITA[0].len(), VAQUITA[1].len());
        assert_eq!(CALF[0].len(), CALF[1].len());
    }

    /// The mes patrio builds up: the gill slits are painted from the middle
    /// of it, and there is more in the picture.
    #[test]
    fn the_mes_patrio_builds_up() {
        let theme = Theme::classic();
        let opts = opts("");
        let gills = |m, d| {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (s, _) = first(&c);
            // The first gill slit of a great white, its own letter.
            let paint = face(s.sea.shark.kind).side;
            let dab = s.sea.patrias_dab(paint);
            let (c, r) = (paint.x as usize, paint.y as usize + 5);
            dab.over(c, r, b'G').is_some()
        };
        assert!(!gills(9, 1), "gills painted on the first day");
        assert!(gills(9, 15), "no gills at the Grito");
        let plain = first(&ctx(&theme, &opts, Fiesta::NONE)).1;
        let lit = |m, d| differing(&plain, &first(&ctx(&theme, &opts, Fiesta::of_day(m, d))).1);
        assert!(lit(9, 1) < lit(9, 15));
    }

    /// The flag ripples: the wave runs down it, grows towards the free end,
    /// and never throws it far.
    #[test]
    fn the_flag_ripples() {
        let mut moved = false;
        for step in 0..400 {
            let t = step as f32 * 0.05;
            let mut prev = 0.0;
            for i in 0..FLAG_W {
                let dy = ripple(i, t, 1.1);
                assert!(dy.abs() <= 4.5, "{dy}");
                // Neighbouring columns never jump.
                assert!((dy - prev).abs() <= 3.0 || i == 0, "{prev} {dy}");
                prev = dy;
            }
            moved |= (ripple(FLAG_W - 1, t, 1.0) - ripple(FLAG_W - 1, t + 0.4, 1.0)).abs() > 1.0;
        }
        assert!(moved);
        // The hoist hardly moves.
        assert!(ripple(0, 1.0, 1.0).abs() < 0.5);
    }

    /// The holiday comes and goes through `retheme`, as at midnight.
    #[test]
    fn retheme_dresses_and_undresses() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts, Fiesta::NONE);
        let on = ctx(&theme, &opts, Fiesta::of_day(9, 12));
        let (mut s, _) = first(&off);
        let mut cv = Canvas::new(120, 36, Color::Reset);
        for (c, dressed) in [(&on, true), (&off, false), (&on, true)] {
            s.retheme(c);
            assert_eq!(s.sea.patrias.is_some(), dressed);
            for _ in 0..90 {
                s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
            }
            cv.clear();
            s.draw(c, &mut cv);
            assert_eq!(
                s.sea.patrias.as_ref().map_or(0, |_| poles_in_view(&s)) > 0,
                dressed
            );
            // A resize keeps the dress.
            s.resize(c, 120, 36);
            assert_eq!(s.sea.patrias.is_some(), dressed);
        }
    }

    /// Odd sizes, options and steps with the holiday on, under two themes.
    #[test]
    fn the_holiday_survives_extremes() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        let cases = [
            "",
            "kind = 'whale-shark'\nzoom = 0\nwater = false",
            "kind = 'hammerhead'\nzoom = 6\nspeed = 50.0\nday = 10",
            "kind = 'tiger'\nfish = 0\nrays = false",
        ];
        for theme in &themes {
            for case in cases {
                let opts = opts(case);
                for day in [(9, 1), (9, 15), (9, 16)] {
                    let c = ctx(theme, &opts, Fiesta::of_day(day.0, day.1));
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (3, 2), (1, 40), (300, 90), (80, 24)] {
                        s.resize(&c, w, h);
                        s.retheme(&c);
                        let mut cv = Canvas::new(w, h, Color::Reset);
                        for dt in [0.0, f32::NAN, 5.0, 0.05, 1e9, 1.0 / 30.0] {
                            s.tick(&c, dt, 0.05);
                            cv.clear();
                            s.draw(&c, &mut cv);
                        }
                    }
                }
            }
        }
    }

    /// A long swim: vaquitas pass now and then, one at a time, and at the
    /// height of the holiday with a calf; the flagpoles keep coming.
    #[test]
    fn a_vaquita_passes() {
        let theme = Theme::classic();
        let opts = opts("day = 60");
        for (m, d, calf) in [(9, 3, false), (9, 15, true)] {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (mut s, mut cv) = first(&c);
            let (mut seen, mut calves, mut most) = (0, false, 0);
            let mut was = false;
            let mut poles = std::collections::HashSet::new();
            for i in 0..30 * 400 {
                s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
                let p = s.sea.patrias.as_ref().unwrap();
                let now = !p.vaquitas.is_empty();
                seen += (now && !was) as usize;
                was = now;
                most = most.max(p.vaquitas.len());
                calves |= p.vaquitas.iter().any(|v| v.calf);
                if i % 30 == 0 {
                    cv.clear();
                    s.draw(&c, &mut cv);
                    let mut found = [(0, 0); 4];
                    let p = s.sea.patrias.as_ref().unwrap();
                    let (lo, hi) = (s.sea.bx as f32, (s.sea.bx + s.sea.bw) as f32);
                    let n = s.sea.flagpoles(p, (lo, hi), 0.0, &mut found);
                    poles.extend(found[..n].iter().map(|f| f.0));
                }
            }
            assert!(seen >= 2, "{m}-{d}: only {seen} vaquitas");
            assert_eq!(most, 1);
            assert_eq!(calves, calf, "{m}-{d}");
            assert!(poles.len() >= 2, "{m}-{d}: {poles:?}");
        }
    }
}
