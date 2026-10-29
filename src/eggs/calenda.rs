//! A calenda goes by during the Guelaguetza: a Danza de la Pluma dancer,
//! marmotas, monos de calenda and a banda.
//!
//! A calenda is the street parade that announces a festival in Oaxaca:
//! dancers, a banda de música, and the figures that only a calenda has. The
//! marmotas are great globes of cloth painted with stripes and flowers, held
//! up on long poles by women who twirl them as they dance, with ribbons
//! streaming from them; the monos de calenda are giant puppets of papier-mâché,
//! a man and a woman with enormous heads and wide costumes, carried on the
//! shoulders of a dancer whose legs show below. Original oxiper pixel art:
//! the Danza de la Pluma dancer, in a grana tunic with a gold collar, a skirt
//! of morado and bells at his ankles, wears the great round headdress, a disc
//! of feathers in rings of rosa, turquesa, morado, verde and gold fanning
//! out from his head, which sways as he stamps and turns; in one hand he
//! holds a round painted shield, in the other a sonaja he shakes. A marmota
//! (two of them) spins on its pole above a woman in a huipil, her ribbons
//! streaming; the monos tilt and nod, their arms flung about, their carrier's
//! feet keeping time; and at the back come three músicos of the banda in straw
//! hats, with a sousaphone-style tuba, a trombone sliding in and out and a
//! trumpet, brass flashing in the light and notes (♪ ♫) floating up from
//! their bells. They all go along together, each bobbing to its own beat. The
//! parade grows with the width of the screen (the pluma dancer, a marmota and
//! a mono come first, the banda and the rest follow where there is room), its
//! figures scale with the height, and it crosses from either side within
//! 30 s.

use super::blit::{Xf, blit, block, fit, hash, line, whole};
use super::guelaguetza::{BODY, LOOKS, Palette, Stance, WIDTH, arm, woman};
use super::palette::fade;
use super::{Egg, Info, pixel};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "calenda",
    start,
    fiesta: Kind::Guelaguetza,
    when: super::any_day,
};

/// The dancer of the Danza de la Pluma, facing front, without arms and without
/// the headdress, which is drawn round his head: black hair, a gold headband,
/// a bold face, a tunic of grana with a gold collar and embroidered bands, a
/// skirt of morado with a fringe, bare legs with ankle bells and sandals.
const PLUMA: &[&str] = &[
    "......KKKKKKKKK......",
    "......KYYYYYYYK......",
    "......KHHHHHHHK......",
    "......KHKKHKKHK......",
    "......KHkWHWkHK......",
    "......KhHHhHHhK......",
    "......BHKKhKKHB......",
    "......KHHTTTHHK......",
    "......KKHHHHHKK......",
    ".........hhh.........",
    "...gGYYYYYYYYYYYGg...",
    "...gGGGBYGYGYBGGGg...",
    "....gGGGBYYYBGGGg....",
    "....gGGGGBYBGGGGg....",
    "....gGGGGGBGGGGGg....",
    "....gGGGGGGGGGGGg....",
    "....YQYQYQYQYQYQY....",
    "....QYQYQYQYQYQYQ....",
    "....gGGGGGGGGGGGg....",
    "....GGWGGWYGWGGWG....",
    "....gGQGGYRYGGQGg....",
    "....gQRQGGYGGQRQg....",
    "....YYYYYRRRYYYYY....",
    "....YyYyYRRRYyYyY....",
    "....mMPMMMMPMMMMP....",
    "....mPMMMMPMMMMPm....",
    "....PMRMMPMMMMRMm....",
    "....mRWRPMYMMRWRm....",
    "...mMMRPMYWYPMRMMP...",
    "...mMMPMMMYPMMMMPm...",
    "...mMPMMMMPMMMMPMm...",
    "..YYYYYYYYYYYYYYYYY..",
    "..YyYyYyYyYyYyYyYyY..",
    "..Y.Y.Y.Y.Y.Y.Y.Y.Y..",
    "......HHH...HHH......",
    "......HHH...HHH......",
    ".....BBBB...BBBB.....",
    ".....dddd...dddd.....",
];
/// A músico of the banda, without arms or instrument: a straw hat, a red
/// neckerchief, a white shirt, dark trousers.
const MUSICO: &[&str] = &[
    ".....AAA.....",
    "....AAAAA....",
    "....AAAAA....",
    "....GGGGG....",
    "AaAaAaAaAaAaA",
    ".aaaaaaaaaaa.",
    "....HHHHH....",
    "....HkHkH....",
    "....hHHHh....",
    "....HKKKH....",
    ".....GGG.....",
    "..wWWWWWWWw..",
    "..wWWWWWWWw..",
    "..wWWWwWWWw..",
    "..wWWWWWWWw..",
    "..wWWWwWWWw..",
    "..wWWWWWWWw..",
    "..wWWWwWWWw..",
    "..wWWWWWWWw..",
    "..wWWWWWWWw..",
    "..dddBBBddd..",
    "...nNNNNNN...",
    "...NNNNNNN...",
    "...NNNNNNN...",
    "...nNN.NNN...",
    "...NNN.NNN...",
    "...NNN.NNN...",
    "...nNN.NNN...",
    "...NNN.NNN...",
    "...ddd.ddd...",
    "...ddd.ddd...",
];
/// A trumpet, its mouthpiece at the left, its bell at the right.
const TRUMPET: &[&str] = &[
    "............B",
    "..........BFB",
    "BFBBFBBBBBBBB",
    "bbbbbbbbbbBBB",
    "..........bBB",
    "...........bb",
];
/// A trombone with its slide in.
const TROMBONE_IN: &[&str] = &[
    "B............BBBBBB",
    "bBBFBBBBFBBBBBBBFBB",
    ".B....b.....BBBBBBB",
    ".B....b.....BbbBBBB",
    ".BbbbbbbbbbbB..bBBB",
    "................bBB",
    ".................bB",
];
/// The same with its slide out.
const TROMBONE_OUT: &[&str] = &[
    "B............BBBBBB",
    "bBBFBBBBFBBBBBBBFBB",
    ".B......b....BBBBBB",
    ".B......b....bbBBBB",
    ".BbbbbbbbbbbbbbbBBB",
    "................bBB",
    ".................bB",
];
/// A tuba: its bell above, its coil round the chest.
const TUBA: &[&str] = &[
    ".......bbbbbbbbbbb",
    ".......bBFBBBBBBBb",
    "........bFBBBBBBb.",
    ".........bFBBBBBb.",
    "..........bBBBBb..",
    "...........bBBBb..",
    "...........bBBb...",
    "............bBb...",
    "............BBb...",
    "............BBb...",
    "............FBb...",
    "............BBb...",
    "............BBb...",
    "............BBb...",
    "............FBb...",
    "............BBb...",
    "............BBb...",
    "............BBb...",
    "............BBb...",
    "............BBb...",
    "............BBb...",
    "B....FbbbbbbbBb...",
    "Bb.FbbBBBBBBBbb...",
    ".bbbBBB.....BBBb..",
    ".BbBB.........BBb.",
    "..bb...........Bb.",
    ".bBb...........BBb",
    ".bBB...........BBb",
    ".bBB...........BBb",
    "..bB...........Bb.",
    "..bBB.........BBb.",
    "...bBBB.....BBFb..",
    "....bbBBBBBBBbb...",
    "......bbbbbbb.....",
];
/// The head of a giant puppet: a man in a straw sombrero, goggle eyes, a
/// great mustache.
const MONO_HOMBRE: &[&str] = &[
    ".......AAA.......",
    ".....AAAAAAA.....",
    "....AAAAAAAAA....",
    "....GGGGGGGGG....",
    "....AAAAAAAAA....",
    "AaAaAaAaAaAaAaAaA",
    ".aaKKKKaaaKKKKaa.",
    ".....HHHHHHH.....",
    "....WWWHHHWWW....",
    "...WWkWWHWWkWW...",
    "...WWkWWHWWkWW...",
    "..HHWWWhhhWWWHH..",
    "..HHHHHhhhHHHHH..",
    "...KKHHhhhHHKK...",
    "...HKKKKKKKKKH...",
    "....HkWkWkWkH....",
    ".....kkkkkkk.....",
    "........K........",
];
/// The head of a giant puppet: a woman with a flower in her hair and braids.
const MONO_MUJER: &[&str] = &[
    ".......KKK.......",
    "....KKKKKKKKK....",
    "...KKKKKKKVKRKV..",
    "..KKKKKKKKKRYRK..",
    "..KKKKKKKKKKRKK..",
    "..KKKKKKKKKKKKK..",
    "..KKKKKKKKKKKKK..",
    "..KKKKHHHHHKKKK..",
    "..KKWWWHHHWWWKK..",
    "..KKWkWHHHWkWKK..",
    ".KKWWkWWHWWkWWKK.",
    ".RKKWWWHHHWWWKKR.",
    ".KKKHHHHhHHHHKKK.",
    ".KKKrrHHhHHrrKKK.",
    ".QYHHHHHHHHHHHYQ.",
    ".K..HHRRRRRHH..K.",
    ".K...HHTTTHH...K.",
    ".Y.....HHH.....Y.",
];

/// What goes in the parade.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Piece {
    /// A marmota carried by the woman with this look.
    Marmota(usize),
    /// A mono de calenda: a woman (else a man).
    Mono(bool),
    /// The dancer of the Danza de la Pluma.
    Pluma,
    /// A músico of the banda.
    Musico(Brass),
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Brass {
    Tuba,
    Trombone,
    Trumpet,
}

/// The parade in order, the one at the head first, with the room each takes
/// (art pixels) and how soon it is given room on a narrow screen (0 first).
const PARADE: [(Piece, f32, usize); 8] = [
    (Piece::Marmota(0), 28.0, 1),
    (Piece::Mono(false), 28.0, 2),
    (Piece::Pluma, 38.0, 0),
    (Piece::Mono(true), 28.0, 4),
    (Piece::Marmota(3), 28.0, 5),
    (Piece::Musico(Brass::Tuba), 26.0, 3),
    (Piece::Musico(Brass::Trombone), 30.0, 6),
    (Piece::Musico(Brass::Trumpet), 26.0, 7),
];

/// How tall the tallest of them is, in art pixels: a marmota, the woman under
/// it and the globe above.
const HEIGHT: i32 = 66;
/// The marmota: the skirt of its carrier, the globe's radius and how high
/// above her huipil its center is.
const SKIRT: i32 = 14;
const GLOBE: f32 = 9.0;
const ABOVE: f32 = 15.0;
/// The pluma dancer: where his head is in the art, and the headdress's radius.
const PLUMA_HEAD: (f32, f32) = (10.0, 4.5);
const HEADDRESS: f32 = 17.0;
/// A mono: the head, the body and the carrier's legs, in art pixels.
const MONO_HEAD: (i32, i32) = (17, 18);
const MONO_BODY: i32 = 24;
const MONO_LEGS: i32 = 9;

/// Seconds the parade takes to cross.
const CROSSING: f32 = 21.0;
/// Spokes in the headdress, and ribbons on a marmota.
const SPOKES: i32 = 30;
const RIBBONS: usize = 7;
const RIBBON_COLORS: [u8; 7] = *b"RQYGMVO";

/// For a screen `w` columns by `h` rows: the scale of the art, and the
/// parade that fits (as the indexes of [`PARADE`], in order).
fn layout(w: u16, h: u16) -> (f32, Vec<usize>) {
    // At least the dancer, a marmota and a mono have to fit across.
    let k = fit(h, HEIGHT, 0.85).min(whole(w as f32 / 100.0));
    let mut chosen: Vec<usize> = Vec::new();
    // Give room to each in turn, the first ones first, while the screen
    // is wide enough for them.
    for rank in 0..PARADE.len() {
        let i = PARADE.iter().position(|p| p.2 == rank).unwrap_or(0);
        let width: f32 = chosen.iter().chain([&i]).map(|&j| PARADE[j].1).sum();
        if chosen.len() < 3 || width * k <= w as f32 * 0.95 {
            chosen.push(i);
        }
    }
    chosen.sort_unstable();
    (k, chosen)
}

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let mut rng = fastrand::Rng::with_seed(rng.u64(..));
    let (k, chosen) = layout(w, h);
    let mut x = 0.0;
    let parade: Vec<Figure> = chosen
        .iter()
        .map(|&i| {
            let (piece, room, _) = PARADE[i];
            let figure = Figure {
                piece,
                at: x,
                room,
                beat: rng.f32(),
            };
            x += room;
            figure
        })
        .collect();
    let span = (x * k).round() as i32;
    Box::new(Calenda {
        t: 0.0,
        dist: 0.0,
        speed: ((w as i32 + span) as f32 / CROSSING).max(5.0),
        left: rng.bool(),
        w: w as i32,
        k,
        span,
        parade,
    })
}

struct Figure {
    piece: Piece,
    /// Where its room starts, in art pixels from the back of the parade, and
    /// how wide it is.
    at: f32,
    room: f32,
    /// Where in its beat it starts (0 to 1).
    beat: f32,
}

struct Calenda {
    t: f32,
    /// Columns walked.
    dist: f32,
    speed: f32,
    /// Going right to left?
    left: bool,
    w: i32,
    k: f32,
    /// The parade's width in columns.
    span: i32,
    parade: Vec<Figure>,
}

impl Calenda {
    /// The column the back of the parade is at.
    fn origin(&self) -> i32 {
        let d = self.dist as i32;
        if self.left { self.w - d } else { d - self.span }
    }

    /// The left column of the room of figure `i`. The parade is laid out
    /// with its head first; going right the head is at the right.
    fn x(&self, i: usize) -> i32 {
        let f = &self.parade[i];
        let from_head = (f.at * self.k).round() as i32;
        let room = (f.room * self.k).round() as i32;
        if self.left {
            self.origin() + from_head
        } else {
            self.origin() + self.span - from_head - room
        }
    }
}

impl Egg for Calenda {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::guelaguetza(theme);
        let ground = c.h() as i32 * 2;
        let reach = (20.0 * self.k) as i32;
        // The ones behind first, so the ones in front are over them.
        let order: Vec<usize> = if self.left {
            (0..self.parade.len()).rev().collect()
        } else {
            (0..self.parade.len()).collect()
        };
        for i in order {
            let f = &self.parade[i];
            let x = self.x(i);
            if x + (f.room * self.k) as i32 + reach < 0 || x - reach > self.w {
                continue;
            }
            let beat = (self.t * 1.8 + f.beat) * TAU;
            let at = Pen {
                pal: &pal,
                theme,
                k: self.k,
                t: self.t,
                beat,
                x,
                ground,
                room: f.room,
                seed: i as u32 * 31 + 7,
            };
            match f.piece {
                Piece::Marmota(look) => at.marmota(c, look),
                Piece::Mono(woman) => at.mono(c, woman),
                Piece::Pluma => at.pluma(c),
                Piece::Musico(brass) => at.musico(c, brass),
            }
        }
    }

    fn done(&self) -> bool {
        let gone = if self.left {
            self.origin() + self.span < 0
        } else {
            self.origin() > self.w
        };
        gone || self.t > 29.5
    }
}

/// What a figure needs to be drawn: the palette, the scale, the time and the
/// beat, its room's left column, the ground and its room's width.
struct Pen<'a> {
    pal: &'a Palette,
    theme: &'a Theme,
    k: f32,
    t: f32,
    beat: f32,
    x: i32,
    ground: i32,
    room: f32,
    seed: u32,
}

/// A bit lighter, for the spine of a feather.
fn lighter(l: u8) -> u8 {
    match l {
        b'R' => b'r',
        b'Q' => b'z',
        b'V' => b'l',
        b'M' => b'P',
        b'Y' => b'L',
        b'O' => b'o',
        other => other,
    }
}

impl Pen<'_> {
    /// The column of the middle of the room.
    fn mid(&self) -> f32 {
        self.x as f32 + self.room * 0.5 * self.k
    }

    /// A length in art pixels, in pixels of at least one.
    fn px(&self, art: f32) -> i32 {
        ((art * self.k).round() as i32).max(1)
    }

    /// A woman carrying a marmota: she dances with both hands on the pole,
    /// the globe spinning above her head and its ribbons streaming.
    fn marmota(&self, c: &mut Canvas, look: usize) {
        let (k, pal) = (self.k, self.pal);
        let sway = self.beat.sin() * 3.0;
        let bob = (0.5 - 0.5 * (self.beat * 2.0).cos()).round();
        let feet = ((BODY + SKIRT) as f32 * k).round() as i32;
        let st = Stance {
            at: (
                (self.mid() - WIDTH as f32 * 0.5 * k).round() as i32,
                self.ground - feet - (bob * k) as i32,
            ),
            k,
            turn: 0.0,
            flare: 0.15 + 0.15 * self.beat.sin(),
            sway,
            skirt: SKIRT,
            feet: [
                self.beat.sin().max(0.0) * 1.5,
                (-self.beat.sin()).max(0.0) * 1.5,
            ],
        };
        let look = &LOOKS[look % LOOKS.len()];
        woman(c, pal, look, &st);
        // Both hands up on the pole, above her head.
        let hands = [st.point(-2.5, -1.0), st.point(2.5, -1.0)];
        arm(c, pal, look, st.point(-7.5, 12.5), hands[0], k);
        arm(c, pal, look, st.point(7.5, 12.5), hands[1], k);
        // The globe sways a little over her head; the pole goes up into it.
        let wob = (self.t * 2.7 + self.seed as f32).sin();
        let center = (
            self.mid() + wob * 1.2 * k + sway * 0.4 * k,
            st.point(0.0, -ABOVE).1,
        );
        let base = ((hands[0].0 + hands[1].0) / 2.0, hands[0].1);
        line(c, base, center, self.px(1.6), pal.of(b'D'));
        self.globe(c, center, GLOBE * k, self.t * 4.5 + self.seed as f32);
        // The ribbons: from the lower rim of the globe, streaming and waving.
        for j in 0..RIBBONS {
            let a = (j as f32 / (RIBBONS - 1) as f32 - 0.5) * 2.4;
            let from = (
                center.0 + a.sin() * GLOBE * 0.85 * k,
                center.1 + GLOBE * 0.55 * k,
            );
            let len = 9.0 + 3.0 * hash(self.seed, j as u32);
            let col = pal.of(RIBBON_COLORS[(j + self.seed as usize) % RIBBON_COLORS.len()]);
            let steps = (len * k).ceil() as i32;
            for s in 0..=steps {
                let f = s as f32 / steps as f32;
                let wave = (self.t * 6.0 + j as f32 * 1.3 + f * 4.0).sin() * 1.6 * f * k;
                let (x, y) = (
                    from.0 + a * 9.0 * f * k + wave,
                    from.1 + f * len * k * (1.0 - 0.3 * a.abs()),
                );
                block(
                    c,
                    x.round() as i32,
                    y.round() as i32,
                    self.px(1.0).min(2),
                    col,
                );
            }
        }
    }

    /// A cloth globe: painted stripes and a belt of flowers, shaded as a sphere
    /// lit from the upper left, turned by `spin`.
    fn globe(&self, c: &mut Canvas, (cx, cy): (f32, f32), r: f32, spin: f32) {
        let pal = self.pal;
        for y in (cy - r).floor() as i32..=(cy + r).ceil() as i32 {
            for x in (cx - r).floor() as i32..=(cx + r).ceil() as i32 {
                let (nx, ny) = ((x as f32 + 0.5 - cx) / r, (y as f32 + 0.5 - cy) / r);
                let d2 = nx * nx + ny * ny;
                if d2 > 1.0 {
                    continue;
                }
                let z = (1.0 - d2).sqrt();
                let lon = nx.atan2(z) + spin;
                let lat = ny.asin();
                let band = lon.rem_euclid(TAU) / TAU * 8.0;
                let along = band.fract();
                let letter = if lat.abs() > 1.2 {
                    // The caps, top and bottom.
                    if lat < 0.0 { b'Y' } else { b'G' }
                } else if lat.abs() < 0.2 {
                    // The belt of flowers: green with gold ones.
                    if (band * 2.0).floor() as i32 % 2 == 0 {
                        b'Y'
                    } else {
                        b'V'
                    }
                } else if (0.38..0.62).contains(&along) {
                    (*b"GQRM")[band.floor() as usize % 4]
                } else if (0.0..0.06).contains(&along) {
                    b'w'
                } else {
                    b'W'
                };
                let lit = 0.5 + 0.5 * (-0.4 * nx - 0.5 * ny + 0.76 * z);
                let col = pal.of(letter);
                let col = if lit < 0.4 {
                    col.scale(0.72)
                } else if lit > 0.88 {
                    col.lerp(pal.of(b'F'), 0.5)
                } else {
                    col
                };
                pixel(c, x, y, col);
            }
        }
    }

    /// A giant puppet of papier-mâché: a huge head that nods and tilts, a
    /// wide costume that leans, arms flung about, and the legs of the dancer
    /// carrying it showing below, keeping time.
    fn mono(&self, c: &mut Canvas, woman: bool) {
        let (k, pal) = (self.k, self.pal);
        let axis = self.mid();
        let step = self.beat.sin();
        let bob = (0.5 - 0.5 * (self.beat * 2.0).cos()) * 2.0;
        let total = (MONO_HEAD.1 - 2 + MONO_BODY + MONO_LEGS) as f32;
        let top = self.ground as f32 - total * k - bob * k;
        // The costume leans from side to side, the head more.
        let lean = 5.0 * (self.t * 1.9 + self.seed as f32).sin() * k;
        let body_top = top + (MONO_HEAD.1 - 2) as f32 * k;
        let (cloth, shade, band) = if woman {
            (b'M', b'm', b'R')
        } else {
            (b'Q', b'q', b'Y')
        };
        let hem = body_top + MONO_BODY as f32 * k;
        // The dancer's legs and shoes under the hem, stepping.
        for (side, s) in [(-1.0f32, step), (1.0, -step)] {
            let lift = s.max(0.0) * 2.5;
            let x = axis + side * 4.0 * k + lean * 0.1;
            line(
                c,
                (x, hem),
                (x + side * 0.8 * k, hem + (MONO_LEGS as f32 - lift) * k),
                self.px(2.0),
                pal.of(b'N'),
            );
            block(
                c,
                (x + side * 0.8 * k).round() as i32 - self.px(1.0),
                (hem + (MONO_LEGS as f32 - lift) * k).round() as i32 - 1,
                self.px(3.0),
                pal.of(b'd'),
            );
        }
        // The costume, a bell: a shirt, a sash and trousers, or a huipil and a
        // skirt with a band and lace.
        for r in 0..MONO_BODY {
            let f = r as f32 / MONO_BODY as f32;
            let half = (7.0 + 6.0 * f.powf(0.9)) * k;
            let mid = axis + lean * (1.0 - f);
            let y0 = body_top + r as f32 * k;
            for y in y0.round() as i32..(y0 + k).round() as i32 {
                for x in (mid - half).round() as i32..=(mid + half).round() as i32 {
                    let u = (x as f32 + 0.5 - mid) / half.max(0.5);
                    let edge = u.abs() > 0.82;
                    let letter = if woman {
                        if r < 9 {
                            if (r == 3 || r == 6) && (u * 4.0).floor() as i32 % 2 == 0 && !edge {
                                if r == 3 { b'R' } else { b'Y' }
                            } else if edge {
                                b'w'
                            } else {
                                b'W'
                            }
                        } else if r == 9 {
                            b'Y'
                        } else if r >= MONO_BODY - 2 {
                            if (u * 6.0).floor() as i32 % 2 == 0 {
                                b'W'
                            } else {
                                b'w'
                            }
                        } else if (MONO_BODY - 7..MONO_BODY - 4).contains(&r) {
                            band
                        } else if (u * 5.0).floor() as i32 % 2 == 0 {
                            cloth
                        } else {
                            shade
                        }
                    } else if r < 11 {
                        if edge {
                            b'w'
                        } else if u.abs() < 0.08 && r % 3 == 1 {
                            b'k'
                        } else {
                            b'W'
                        }
                    } else if r < 13 {
                        b'G'
                    } else if r >= MONO_BODY - 2 {
                        b'Y'
                    } else if u.abs() < 0.1 || edge {
                        shade
                    } else {
                        cloth
                    };
                    pixel(c, x, y, pal.of(letter));
                }
            }
        }
        // The arms flung about, in sleeves, ending in big hands.
        for (side, sgn) in [(0usize, -1.0f32), (1, 1.0)] {
            let sw = (self.t * 4.2 + self.seed as f32 + side as f32 * PI).sin();
            let ang = 0.55 + 0.7 * sw;
            let from = (axis + sgn * 6.5 * k + lean * 0.9, body_top + 3.0 * k);
            let to = (
                from.0 + sgn * ang.sin() * 15.0 * k,
                from.1 + ang.cos() * 15.0 * k,
            );
            line(c, from, to, self.px(3.5), pal.of(b'W'));
            block(
                c,
                to.0.round() as i32 - self.px(1.5),
                to.1.round() as i32 - self.px(1.5),
                self.px(3.0),
                pal.of(b'H'),
            );
        }
        // The head, hung from the shoulders by its chin, nodding and tilting.
        let art = if woman { MONO_MUJER } else { MONO_HOMBRE };
        let tilt = 0.14 * (self.t * 1.9 + self.seed as f32 + 0.6).sin() + 0.04 * self.beat.sin();
        let xf = Xf {
            pivot: (MONO_HEAD.0 as f32 * 0.5, MONO_HEAD.1 as f32 - 1.0),
            at: (axis + lean * 1.25, body_top + 1.5 * k),
            scale: (k, k),
            rot: tilt,
            flip: false,
        };
        blit(c, art, xf, |_| 0, |l| pal.get(l));
    }

    /// The dancer of the Danza de la Pluma, stamping, his headdress swaying,
    /// a shield in one hand and a sonaja in the other.
    fn pluma(&self, c: &mut Canvas) {
        let (k, pal) = (self.k, self.pal);
        let axis = self.mid();
        // He stamps: the whole of him drops on the beat, and sways.
        let stamp = (0.5 + 0.5 * (self.beat * 2.0).cos()).round();
        let sway = self.beat.sin() * 1.5;
        let h = PLUMA.len() as f32;
        let top = self.ground as f32 - h * k + stamp * 1.5 * k;
        let x = (axis - 10.5 * k + sway * k).round() as i32;
        let head = (x as f32 + (PLUMA_HEAD.0 + 0.5) * k, top + PLUMA_HEAD.1 * k);
        // The headdress: a disc of feathers behind his head, in rings.
        self.headdress(c, head, sway * 0.05 + 0.04 * (self.t * 1.3).sin());
        // His arms, behind the body at the shoulders: the right (on the left
        // of the screen) holds the shield out, the left shakes the sonaja up.
        let shake = (self.t * 22.0).sin();
        let shoulder = |dx: f32| (x as f32 + (10.5 + dx) * k, top + 12.5 * k);
        let rs = shoulder(-6.5);
        let ls = shoulder(6.5);
        let rh = (rs.0 - 4.5 * k, rs.1 + 8.0 * k + stamp * k);
        let lh = (ls.0 + 5.5 * k, ls.1 - 6.0 * k + shake * k);
        for (a, b) in [(rs, rh), (ls, lh)] {
            line(c, a, b, self.px(3.0), pal.of(b'H'));
            let w = (a.0 + (b.0 - a.0) * 0.8, a.1 + (b.1 - a.1) * 0.8);
            line(c, w, b, self.px(3.0), pal.of(b'B'));
        }
        let shift = |r: i32| {
            // The legs stomp one after the other.
            if r >= 34 {
                (self.beat.sin() * 1.2).round() as i32
            } else {
                0
            }
        };
        blit(c, PLUMA, Xf::new(x, top.round() as i32, k), shift, |l| {
            pal.get(l)
        });
        // The shield on the left, in front of the arm.
        self.shield(c, (rh.0 - 2.0 * k, rh.1 - 1.0 * k));
        // The sonaja up on the right: a stick, a gourd, a tuft of feathers.
        line(
            c,
            (lh.0, lh.1 + 2.0 * k),
            (lh.0 - 0.6 * k, lh.1 - 4.0 * k),
            self.px(1.2),
            pal.of(b'D'),
        );
        let gourd = (lh.0 - 0.6 * k, lh.1 - 6.0 * k);
        for y in (gourd.1 - 2.6 * k).round() as i32..=(gourd.1 + 2.6 * k).round() as i32 {
            for xx in (gourd.0 - 2.2 * k).round() as i32..=(gourd.0 + 2.2 * k).round() as i32 {
                let (dx, dy) = (
                    (xx as f32 + 0.5 - gourd.0) / (2.2 * k),
                    (y as f32 + 0.5 - gourd.1) / (2.6 * k),
                );
                if dx * dx + dy * dy <= 1.0 {
                    let stripe = ((dx + 1.0) * 3.0).floor() as i32 % 2 == 0;
                    pixel(c, xx, y, pal.of(if stripe { b'Y' } else { b'G' }));
                }
            }
        }
        for (j, col) in (*b"RQM").into_iter().enumerate() {
            let a = (j as f32 - 1.0) * 0.45 + shake * 0.1;
            line(
                c,
                (gourd.0, gourd.1 - 2.4 * k),
                (
                    gourd.0 + a.sin() * 5.0 * k,
                    gourd.1 - 2.4 * k - a.cos() * 5.0 * k,
                ),
                self.px(1.0),
                pal.of(col),
            );
        }
    }

    /// The great round headdress of the Danza de la Pluma: a disc of
    /// feathers fanning out from the middle in rings, each spoke with a lighter
    /// spine, turning a little with the dancer's sway.
    fn headdress(&self, c: &mut Canvas, (cx, cy): (f32, f32), rot: f32) {
        let (k, pal) = (self.k, self.pal);
        let reach = HEADDRESS * k;
        for y in (cy - reach).floor() as i32..=(cy + reach).ceil() as i32 {
            for x in (cx - reach).floor() as i32..=(cx + reach).ceil() as i32 {
                let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
                let d2 = (dx * dx + dy * dy) / (k * k);
                if !(9.0..=HEADDRESS * HEADDRESS).contains(&d2) {
                    continue;
                }
                let r = d2.sqrt();
                let sp = (dy.atan2(dx) + rot) / TAU * SPOKES as f32;
                let (idx, f) = (sp.floor() as i32, sp.rem_euclid(1.0));
                // The feathers fan out, so the gaps close up with the radius.
                let gap = 0.5 * (1.0 - (0.4 + 0.6 * (r / HEADDRESS)).min(1.0));
                if (f - 0.5).abs() > 0.5 - gap {
                    continue;
                }
                let colors = *b"RQMVYO";
                let pick =
                    |ring: i32| colors[(idx + ring).rem_euclid(colors.len() as i32) as usize];
                let letter = if r < 6.0 {
                    b'Y'
                } else if r < 10.0 {
                    pick(0)
                } else if r < 14.0 {
                    pick(2)
                } else if r < 16.0 {
                    pick(4)
                } else {
                    b'W'
                };
                let letter = if (f - 0.5).abs() < 0.09 {
                    lighter(letter)
                } else {
                    letter
                };
                pixel(c, x, y, pal.of(letter));
            }
        }
    }

    /// A round painted shield, a chimal: a gold rim, a grana field with a
    /// turquoise cross and a boss.
    fn shield(&self, c: &mut Canvas, (cx, cy): (f32, f32)) {
        let (k, pal) = (self.k, self.pal);
        let r = 6.4 * k;
        for y in (cy - r).floor() as i32..=(cy + r).ceil() as i32 {
            for x in (cx - r).floor() as i32..=(cx + r).ceil() as i32 {
                let (dx, dy) = ((x as f32 + 0.5 - cx) / k, (y as f32 + 0.5 - cy) / k);
                let d = (dx * dx + dy * dy).sqrt();
                if d > 6.4 {
                    continue;
                }
                let letter = if d > 5.2 {
                    b'Y'
                } else if d > 4.4 {
                    b'g'
                } else if d < 0.9 {
                    b'R'
                } else if d < 1.9 {
                    b'Y'
                } else if dx.abs() < 1.1 || dy.abs() < 1.1 {
                    b'Q'
                } else {
                    b'G'
                };
                pixel(c, x, y, pal.of(letter));
            }
        }
    }

    /// A músico of the banda, walking and playing: his brass flashes, and
    /// notes float up from the bell.
    fn musico(&self, c: &mut Canvas, brass: Brass) {
        let (k, pal) = (self.k, self.pal);
        let step = self.beat.sin() > 0.0;
        let bob = i32::from(step);
        let h = MUSICO.len() as f32;
        let x = (self.mid() - 6.5 * k).round() as i32;
        let top = self.ground - (h * k).round() as i32 - (bob as f32 * k) as i32;
        let shift = |r: i32| {
            if r >= 21 {
                if step { 1 } else { -1 }
            } else {
                0
            }
        };
        // The tuba is worn round the chest and its bell is above his head.
        let (art, off, bell): (&[&str], (f32, f32), (f32, f32)) = match brass {
            Brass::Tuba => (TUBA, (-3.0, -12.0), (4.0, 1.0)),
            Brass::Trombone => (
                if (self.beat.sin() > 0.0) ^ (self.seed.is_multiple_of(2)) {
                    TROMBONE_OUT
                } else {
                    TROMBONE_IN
                },
                (5.0, 7.0),
                (17.0, 3.0),
            ),
            Brass::Trumpet => (TRUMPET, (5.0, 7.0), (11.5, 3.0)),
        };
        let brass_at = (
            x + (off.0 * k).round() as i32,
            top + (off.1 * k).round() as i32,
        );
        blit(c, MUSICO, Xf::new(x, top, k), shift, |l| pal.get(l));
        blit(
            c,
            art,
            Xf::new(brass_at.0, brass_at.1, k),
            |_| 0,
            |l| pal.get(l),
        );
        // The glints: stars of light on the brass, in turns.
        let mouth = (
            brass_at.0 as f32 + bell.0 * k,
            brass_at.1 as f32 + bell.1 * k,
        );
        let flash = ((self.t * 1.7 + self.seed as f32 * 0.37).fract() * 4.0).min(2.0);
        if flash < 0.5 {
            let r = ((1.0 - flash * 2.0) * 3.0 * k.max(1.0)).round() as i32;
            let (gx, gy) = (mouth.0.round() as i32, mouth.1.round() as i32);
            for d in -r..=r {
                pixel(c, gx + d, gy, pal.of(b'F'));
                pixel(c, gx, gy + d, pal.of(b'F'));
            }
        }
        // The notes, each rising, swaying and fading out.
        for j in 0..3u32 {
            let period = 0.6;
            let phase = hash(self.seed, 5) * period;
            let n = ((self.t - phase) / period).floor() - j as f32;
            let age = self.t - (n * period + phase);
            if !(0.0..1.8).contains(&age) || n < 0.0 {
                continue;
            }
            let glyph = ['♪', '♫', '♪', '♫'][(hash(n as u32, self.seed) * 4.0) as usize % 4];
            let colour = pal.of((*b"YFRQ")[(hash(self.seed, n as u32) * 4.0) as usize % 4]);
            let col = fade(
                self.theme,
                colour,
                ((age / 1.8 - 0.55) / 0.45).clamp(0.0, 1.0),
            );
            let nx = mouth.0 + (age * 3.0 + n).sin() * 1.5;
            let ny = mouth.1 / 2.0 - 1.0 - age * 3.0;
            c.set_attr(nx.round() as i32, ny.round() as i32, glyph, col, BOLD);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_is_whole() {
        let pal = Palette::guelaguetza(&Theme::classic());
        for art in [
            PLUMA,
            MUSICO,
            TRUMPET,
            TROMBONE_IN,
            TROMBONE_OUT,
            TUBA,
            MONO_HOMBRE,
            MONO_MUJER,
        ] {
            assert!(art.iter().all(|r| r.len() == art[0].len()), "{art:?}");
            for &b in art.iter().flat_map(|r| r.as_bytes()) {
                assert!(b == b'.' || pal.get(b).is_some(), "{}", b as char);
            }
        }
        assert_eq!(MONO_HOMBRE.len() as i32, MONO_HEAD.1);
        assert_eq!(MONO_MUJER[0].len() as i32, MONO_HEAD.0);
    }

    /// Whatever the width, the parade has its dancer, a marmota and a mono
    /// first, and the rest where there is room: all of them on a wide
    /// screen, and the figures grow with the height of the screen.
    #[test]
    fn the_parade_grows_with_the_screen() {
        let narrow = layout(60, 12).1;
        let wide = layout(400, 50).1;
        assert!(
            narrow.len() >= 3 && narrow.len() < wide.len(),
            "{narrow:?} {wide:?}"
        );
        assert_eq!(wide.len(), PARADE.len());
        for parade in [&narrow, &wide, &layout(1, 1).1, &layout(500, 150).1] {
            let pieces: Vec<Piece> = parade.iter().map(|&i| PARADE[i].0).collect();
            assert!(pieces.contains(&Piece::Pluma) && pieces.contains(&Piece::Marmota(0)));
            assert!(pieces.contains(&Piece::Mono(false)));
            // They are in the order of the parade.
            assert!(parade.windows(2).all(|w| w[0] < w[1]));
        }
        assert!(layout(40, 12).0 < layout(400, 120).0);
        // Narrow and tall: the figures shrink so that the first three fit.
        assert!(layout(100, 150).0 <= 1.0);
    }

    /// The parade crosses from one side to the other along the bottom of the
    /// screen, with the banda's notes floating up over it and each figure
    /// bobbing on its own beat.
    #[test]
    fn goes_by_with_its_banda_playing() {
        let theme = Theme::classic();
        for seed in 0..4 {
            let mut e = start(220, 40, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(220, 40, Color::Reset);
            let (mut secs, mut notes, mut seen, mut tops) = (0.0, 0, 0, Vec::new());
            while !e.done() {
                e.update(0.05);
                secs += 0.05;
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                notes += text.chars().filter(|ch| matches!(ch, '♪' | '♫')).count();
                let rows: Vec<bool> = text.lines().map(|l| !l.trim().is_empty()).collect();
                if rows.iter().filter(|r| **r).count() > 15 {
                    seen += 1;
                    tops.push(rows.iter().position(|r| *r).unwrap());
                }
            }
            assert!(secs < 29.0, "{secs}");
            assert!(seen > 100, "{seen} frames with the parade on the screen");
            assert!(notes > 20, "{notes} notes");
            assert!(tops.iter().min() != tops.iter().max(), "nobody bobbed");
        }
    }
}
