//! Navidad under the sea. The jellyfish glow like strings of foquitos, red,
//! green, blue and gold, blinking in chase, alternating and twinkling
//! patterns; a piñata of siete picos is dropped into the sea now and then
//! and sinks slowly, turning, until the shark rams it and the candy and fruit
//! of a posada, colación, tejocotes, cañas and mandarinas, scatter and sink
//! to the sand; the marine snow glints like escarcha, and the lanternfish
//! carry foquitos too.
//!
//! The spots where Día de Muertos puts its ofrendas hold a portal de Belén
//! instead: two mossy stone posts under a roof of heno, its strands hanging
//! and swaying, strings of foquitos along the roofline and between the posts,
//! the Estrella de Belén on the ridge, nochebuenas in clay pots, esferas on
//! the sand and a path of farolitos, paper lanterns lit from inside, leading
//! to it. It is built up with the posadas, the way the nine nights climb to
//! Nochebuena: a few bulbs, the heno and the farolitos on December 12; more
//! strings, the star and the esferas by the middle of the posadas; at the
//! end the manger glowing under the roof and, from Nochebuena on, the Niño
//! Dios in it. The higher the level, the more portals the shark swims past,
//! the more jellyfish light up and the sooner the next piñata comes.
//!
//! Like the ofrendas, the portals are dealt out by hashing their place, and
//! what moves on its own (the piñata, the candy, the extra jellyfish) draws
//! from its own generator, so an ordinary day's sea is the same as ever.

use std::f32::consts::{PI, TAU};

use super::art::Sprite;
use super::life::{Jelly, Speck};
use super::muertos::OFR_W;
use super::sea::{SURF, Sea};
use super::{Dice, Rgb, Rng, col, hash, rgb_of};
use crate::fiesta;
use crate::scene::Ctx;

const ROJO: Rgb = Rgb(200, 16, 46);
const PINO: Rgb = Rgb(31, 111, 58);
const DORADO: Rgb = Rgb(242, 182, 50);
const PLATA: Rgb = Rgb(201, 204, 214);
const BLANCO: Rgb = Rgb(244, 241, 234);
const HENO: Rgb = Rgb(154, 165, 138);
const MUSGO: Rgb = Rgb(79, 122, 40);
const TEJOCOTE: Rgb = Rgb(232, 137, 28);
const CANA: Rgb = Rgb(201, 178, 107);
const CANELA: Rgb = Rgb(139, 74, 30);
/// The foquitos: rojo, verde, azul, amarillo, rosa, naranja.
const BULBS: [Rgb; 6] = [
    Rgb(255, 59, 59),
    Rgb(59, 209, 111),
    Rgb(59, 123, 255),
    Rgb(255, 210, 59),
    Rgb(255, 95, 180),
    Rgb(255, 140, 26),
];
/// The papel de china of the piñata: rosa mexicano, amarillo, naranja,
/// turquesa, morado, verde limón.
const CHINA: [Rgb; 6] = [
    Rgb(228, 0, 124),
    Rgb(255, 210, 0),
    Rgb(255, 140, 0),
    Rgb(0, 170, 184),
    Rgb(112, 40, 160),
    Rgb(140, 200, 40),
];
/// The warm light of a candle or a lit manger in the water.
const WARM: Rgb = Rgb(255, 190, 90);

/// The holiday's colors under the theme, and how far it has come.
#[derive(Clone, Copy)]
pub struct Dress {
    pub level: f32,
    /// From Nochebuena on, the Niño Dios lies in the manger.
    nino: bool,
    /// The foquitos' colors and the piñata's papel de china: the theme's
    /// accents, or the holiday's own under `classic`. Fruit, heno and
    /// the nochebuena keep theirs.
    pub bulbs: [Rgb; 6],
    pub china: [Rgb; 6],
}

impl Dress {
    /// The dress for today, if today is Navidad.
    pub fn of(ctx: &Ctx) -> Option<Dress> {
        if !ctx.fiesta.is(fiesta::Kind::Navidad) {
            return None;
        }
        let (mut bulbs, mut china) = (BULBS, CHINA);
        for (i, c) in bulbs.iter_mut().enumerate() {
            *c = rgb_of(ctx.pick(i, col(*c))).unwrap_or(*c);
        }
        for (i, c) in china.iter_mut().enumerate() {
            *c = rgb_of(ctx.pick(i + 3, col(*c))).unwrap_or(*c);
        }
        let (month, day) = ctx.fiesta.date();
        Some(Dress {
            level: ctx.fiesta.level.clamp(0.0, 1.0),
            nino: month == 1 || (month == 12 && day >= 24),
            bulbs,
            china,
        })
    }

    /// The color of a letter of the holiday's drawings that has a fixed one.
    fn color(&self, ch: u8) -> Option<Rgb> {
        Some(match ch {
            b'r' => ROJO,
            b'R' => Rgb(232, 52, 74),
            b'Y' => DORADO,
            b'l' => Rgb(74, 136, 52),
            b'L' => Rgb(46, 94, 40),
            b'q' => Rgb(168, 84, 46),
            b'Q' => Rgb(116, 54, 32),
            b'h' => HENO,
            b'H' => Rgb(190, 200, 170),
            b'x' => CANELA,
            b'X' => Rgb(96, 52, 22),
            b'p' => PLATA,
            b'P' => BLANCO,
            b'k' => Rgb(96, 64, 30),
            b'o' => TEJOCOTE,
            b'O' => Rgb(246, 176, 70),
            b'D' => Rgb(176, 96, 16),
            b'm' => Rgb(245, 124, 24),
            b'M' => Rgb(255, 170, 60),
            b'N' => Rgb(196, 80, 12),
            b'c' => CANA,
            b'C' => Rgb(150, 126, 62),
            b'K' => Rgb(228, 210, 152),
            b'b' => Rgb(176, 140, 84),
            _ => return None,
        })
    }
}

/// How brightly foquito `i` shines at time `t`, from 0.3 (dim) to 1 (lit).
/// The strings go through the patterns foquitos have: a chase, one bulb
/// after another; an alternating blink, even and odd; and a twinkle.
pub fn blink(i: usize, t: f32) -> f32 {
    let beat = t * 1.8;
    let lit = match (t / 7.0) as u32 % 3 {
        0 => (1.0 - (beat - i as f32).rem_euclid(4.0)).max(0.0),
        1 => (0.5 + 0.5 * (PI * (beat + i as f32)).cos()).powi(2),
        _ => 0.5 + 0.5 * (t * 3.1 + i as f32 * 2.4).sin(),
    };
    0.3 + 0.7 * lit
}

// ----- the portal de Belén -----

/// Rows from the foot to the roof's ridge, how fast the roof falls away
/// from it, and how thick the thatch is.
const PEAK: f32 = 46.0;
const SLOPE: f32 = 0.6;
const THICK: f32 = 5.0;
/// The posts' columns, left and right.
const POSTS: [(i32, i32); 2] = [(3, 6), (31, 34)];

/// The Estrella de Belén on the ridge, in rays.
#[rustfmt::skip]
const STAR: Sprite = &[
    "...x...",
    "...x...",
    "..xxx..",
    "xxxxxxx",
    "..xxx..",
    "...x...",
    "...x...",
];

/// A nochebuena in a clay pot: red bracts round a yellow heart.
#[rustfmt::skip]
const POT: Sprite = &[
    ".r.R.r.",
    "rrRYRrr",
    ".rRYRr.",
    "..lLl..",
    ".qqqqq.",
    "..qQq..",
    "..qQq..",
];

/// A glass esfera: the cap, the glass and its highlight.
#[rustfmt::skip]
const ESFERA: Sprite = &[
    ".p.",
    "wWw",
    "www",
];

/// A farolito, a paper bag with a candle in it: the paper, and the light
/// shining through (`y` warm, `Y` at the flame).
#[rustfmt::skip]
const FAROLITO: Sprite = &[
    "bbb",
    "yYy",
    "yYy",
    "bbb",
];

/// The pesebre: a wooden trough with heno heaped in it.
#[rustfmt::skip]
const MANGER: Sprite = &[
    "..hHhHhHHhH..",
    ".hHhHHhHhHhH.",
    "xxxxxxxxxxxxx",
    ".xXxxxxxxxXx.",
    "..x..xxx..x..",
    "..x..x.x..x..",
];

/// The Niño Dios, swaddled.
#[rustfmt::skip]
const NINO: Sprite = &[
    ".P.",
    "PPP",
];

/// Esferas on the sand: column from the portal's left, and the level from
/// which they lie there.
const ESFERAS: &[(i32, f32)] = &[
    (9, 0.55),
    (28, 0.55),
    (12, 0.7),
    (25, 0.8),
    (-3, 0.9),
    (41, 0.9),
];

// ----- the piñata and what is in it -----

/// Radius of the piñata, from the middle to the tips of its picos.
const PINATA_R: f32 = 10.0;
/// How many pieces of candy and fruit can lie about at once.
const PIECES: usize = 220;

/// A piñata of siete picos, sinking.
#[derive(Clone, Copy)]
struct Pinata {
    x: f32,
    y: f32,
    vy: f32,
    rot: f32,
    spin: f32,
    phase: f32,
    /// Steps since it was dropped.
    age: u32,
}

/// What a piñata holds.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Fruit {
    /// Colación: little sugared candies.
    Colacion,
    /// Scraps of the papel de china it was made of.
    Confeti,
    Tejocote,
    /// Caña de azúcar, in sticks.
    Cana,
    Mandarina,
}

impl Fruit {
    /// What a handful of a piñata is made of: mostly candy.
    const MIX: [Fruit; 20] = [
        Fruit::Colacion,
        Fruit::Colacion,
        Fruit::Confeti,
        Fruit::Colacion,
        Fruit::Tejocote,
        Fruit::Colacion,
        Fruit::Confeti,
        Fruit::Cana,
        Fruit::Colacion,
        Fruit::Mandarina,
        Fruit::Colacion,
        Fruit::Confeti,
        Fruit::Colacion,
        Fruit::Tejocote,
        Fruit::Cana,
        Fruit::Colacion,
        Fruit::Confeti,
        Fruit::Mandarina,
        Fruit::Colacion,
        Fruit::Tejocote,
    ];

    /// How hard the water pulls it down: confetti flutters, fruit drops.
    fn weight(self) -> f32 {
        match self {
            Fruit::Confeti => 0.0018,
            Fruit::Colacion => 0.004,
            Fruit::Cana => 0.0044,
            Fruit::Mandarina => 0.0052,
            Fruit::Tejocote => 0.0056,
        }
    }

    /// Its drawing, for a piece with `tone`: the first of the sprites is
    /// the way it lies.
    fn sprite(self, tone: u8) -> Sprite {
        match self {
            Fruit::Colacion => &["Ww", "ww"],
            Fruit::Confeti if tone.is_multiple_of(2) => &["ww"],
            Fruit::Confeti => &["w", "w"],
            Fruit::Tejocote => &[".k.", "Ooo", "ooD"],
            Fruit::Cana if tone.is_multiple_of(2) => &["KKCKKK", "ccCccc"],
            Fruit::Cana => &["Kc", "Kc", "CC", "Kc", "Kc", "Kc"],
            Fruit::Mandarina => &[".ll.", "MmmN", "mmmN", ".NN."],
        }
    }
}

/// A piece of candy or fruit, adrift or lying on the sand.
#[derive(Clone, Copy)]
struct Piece {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    phase: f32,
    kind: Fruit,
    tone: u8,
    lying: bool,
    /// Steps left on the sand.
    rest: u16,
}

/// The holiday's own state in the sea.
pub struct Navidad {
    pub dress: Dress,
    rng: Rng,
    /// Where the first portal stands, just ahead of where the shark was
    /// when the holiday came.
    pub first: i64,
    /// Jellyfish that glow with the others as the posadas climb.
    lamps: Vec<Jelly>,
    pinata: Option<Pinata>,
    pieces: Vec<Piece>,
    /// Steps until the next piñata is dropped.
    wait: u32,
}

impl Navidad {
    pub fn new(dress: Dress, seed: u32, first: i64) -> Navidad {
        let mut rng = Rng::with_seed(seed as u64 ^ 0x6e61_7669_6461_6421);
        let wait = 60 + rng.range(0, 90) as u32;
        Navidad {
            dress,
            first,
            rng,
            lamps: Vec::with_capacity(8),
            pinata: None,
            pieces: Vec::with_capacity(PIECES),
            wait,
        }
    }

    /// Everything here is in local coordinates, which shift when the sea is
    /// rebased.
    pub fn shift(&mut self, dx: f32) {
        for j in &mut self.lamps {
            j.x -= dx;
        }
        if let Some(p) = &mut self.pinata {
            p.x -= dx;
        }
        for q in &mut self.pieces {
            q.x -= dx;
        }
    }
}

impl Sea {
    /// Puts on (or takes off) the holiday's dress, keeping what drifts in
    /// the sea if it was already on.
    pub fn dress_navidad(&mut self, dress: Option<Dress>) {
        match (dress, self.navidad.as_mut()) {
            (None, _) => self.navidad = None,
            (Some(d), Some(n)) => {
                let more = (d.level - n.dress.level).abs() > 1e-3;
                n.dress = d;
                if more {
                    self.light_lamps();
                }
            }
            (Some(d), None) => {
                let first = self.base + (self.shark.x + self.shark.dir * 70.0) as i64;
                self.navidad = Some(Navidad::new(d, self.seed, first));
                self.light_lamps();
            }
        }
    }

    /// Hangs the extra glowing jellyfish round the camera: more of them the
    /// further into the posadas.
    fn light_lamps(&mut self) {
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        let Some(n) = self.navidad.as_mut() else {
            return;
        };
        let count = (vw / 45.0 * n.dress.level).round().clamp(0.0, 8.0) as usize;
        n.lamps.clear();
        for i in 0..count {
            let x = cam + n.rng.rangef(-0.7, 0.7) * vw;
            let (y, phase) = (n.rng.rangef(30.0, 100.0), n.rng.rangef(0.0, 6.3));
            n.lamps.push(Jelly {
                x,
                y,
                phase,
                front: i % 3 == 1,
            });
        }
    }

    /// The middle (local column) of every portal within `reach` of local
    /// columns `lo..hi`, with its number: the same spots as the ofrendas.
    fn portals(
        &self,
        n: &Navidad,
        range: (f32, f32),
        reach: f32,
        out: &mut [(i64, i32); 4],
    ) -> usize {
        self.spots(0.45 + 0.45 * n.dress.level, n.first, range, reach, out)
    }

    /// Whether a portal stands on the floor's cell `x0..x0 + w`, which then
    /// stays bare.
    pub fn portal_on(&self, x0: i32, w: i32) -> bool {
        let Some(n) = &self.navidad else {
            return false;
        };
        let mut found = [(0, 0); 4];
        let (lo, hi) = (x0 as f32 - OFR_W as f32, (x0 + w) as f32 + OFR_W as f32);
        self.portals(n, (lo, hi), 0.0, &mut found) > 0
    }

    // ----- jellyfish and lights -----

    /// The color and brightness of jellyfish `i`'s glow, if it glows: the
    /// jellies are the bulbs of a string, red, green, blue and gold.
    pub fn jelly_glow(&self, i: usize) -> Option<(Rgb, f32)> {
        let n = self.navidad.as_ref()?;
        Some((n.dress.bulbs[i % 4], blink(i, self.time())))
    }

    /// The extra glowing jellyfish, which follow the ordinary ones in the
    /// string.
    pub fn draw_lamps(&mut self, front: bool) {
        let Some(n) = self.navidad.take() else {
            return;
        };
        let first = self.life.jellies.len();
        let t = self.time();
        for (i, j) in n.lamps.iter().enumerate().filter(|(_, j)| j.front == front) {
            let glow = Some((n.dress.bulbs[(first + i) % 4], blink(first + i, t)));
            self.draw_jelly(*j, glow);
        }
        self.navidad = Some(n);
    }

    /// The glow round a jellyfish bell at `(x, y)`: faint by day, a lamp at
    /// night.
    pub fn jelly_halo(&mut self, x: i32, y: i32, light: Rgb, k: f32) {
        let dark = 1.0 - self.daylight;
        self.halo(x, y, light, (0.18 + 0.55 * dark) * k.powf(1.5), 10.0);
    }

    /// A soft glow of `c` round `(x, y)`, at most `k` strong at the middle
    /// and gone `reach` pixels out.
    fn halo(&mut self, x: i32, y: i32, c: Rgb, k: f32, reach: f32) {
        let r = reach.ceil() as i32;
        for dy in -r..=r {
            for dx in -r..=r {
                let d = ((dx * dx + dy * dy) as f32).sqrt() / reach;
                if d < 1.0 {
                    self.blend(x + dx, y + dy, c, k * (1.0 - d) * (1.0 - d));
                }
            }
        }
    }

    /// How the holiday colors a speck of marine snow: it glints like
    /// escarcha, silver and gold. Its color and strength.
    pub fn navidad_mote(&self, i: usize, near: f32, phase: f32) -> Option<(Rgb, f32)> {
        let n = self.navidad.as_ref()?;
        let roll = (hash(i as i32, 9, 0x3e5c) % 100) as f32;
        if roll >= 25.0 + 35.0 * n.dress.level {
            return None;
        }
        let c = if hash(i as i32, 10, 0x3e5c) & 1 == 0 {
            PLATA
        } else {
            DORADO
        };
        let dark = 1.0 - self.daylight;
        let twinkle = 0.55 + 0.45 * (self.time() * 2.3 + phase * 3.0).sin();
        Some((c, (0.2 + 0.4 * near) * (0.5 + 0.5 * dark) * twinkle))
    }

    /// The light of lanternfish `i`: a foquito.
    pub fn navidad_lamp(&self, i: usize) -> Option<Rgb> {
        let n = self.navidad.as_ref()?;
        Some(n.dress.bulbs[i % 6])
    }

    // ----- the piñata -----

    /// Whether the shark, minding its own business, catches sight of the
    /// piñata sinking by and fancies it. Checked twice a second.
    pub fn notices_pinata(&mut self) -> bool {
        use super::hunter::Mood;
        if self.navidad.is_none() || !self.tick.is_multiple_of(15) {
            return false;
        }
        if !matches!(
            self.shark.mood,
            Mood::Cruise | Mood::Investigate | Mood::Glide | Mood::Feed | Mood::Rest
        ) || self.shark.turn > 0.0
        {
            return false;
        }
        let (vw, _) = self.view_size();
        let top = self.view().y;
        let sx = self.shark.x;
        let Some(n) = self.navidad.as_mut() else {
            return false;
        };
        let Some(p) = n.pinata else {
            return false;
        };
        // In sight, for a moment at least, and not yet on the sand.
        p.age > 60 && p.y > top + 4.0 && (p.x - sx).abs() < vw * 0.8 && n.rng.chance(0.35)
    }

    /// Where the shark aims to hit the piñata: its snout, a little beyond it.
    pub fn pinata_aim(&self) -> Option<(f32, f32)> {
        let p = self.navidad.as_ref()?.pinata?;
        Some((p.x + self.shark.heading() * 12.0, p.y - 3.0))
    }

    /// One step of what moves on its own: the extra jellyfish, the piñata
    /// sinking and bursting, the candy and fruit falling and lying.
    pub fn update_navidad(&mut self) {
        let Some(mut n) = self.navidad.take() else {
            return;
        };
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        let reach = vw * 0.75 + 30.0;
        for j in &mut n.lamps {
            j.drift(cam, reach, &mut n.rng);
        }
        match n.pinata {
            None => {
                n.wait = n.wait.saturating_sub(1);
                if n.wait == 0 {
                    self.drop_pinata(&mut n);
                }
            }
            Some(mut p) => {
                p.age += 1;
                p.phase += 0.05;
                // Plunged in, it settles to a slow sink, turning as it goes.
                p.vy += (0.07 - p.vy) * 0.04;
                p.y += p.vy;
                p.x += 0.1 * p.phase.sin() * 0.5;
                p.rot += p.spin;
                n.pinata = Some(p);
                // The snout, or the sand: either breaks it.
                let (mx, my) = self.shark.mouth();
                let hit = ((mx - p.x).powi(2) + (my - p.y).powi(2)).sqrt() < PINATA_R - 1.5;
                if hit || p.y >= self.bed(p.x) as f32 - PINATA_R + 1.5 {
                    self.burst_pinata(&mut n, p);
                }
            }
        }
        for i in 0..n.pieces.len() {
            let mut q = n.pieces[i];
            if q.lying {
                q.rest = q.rest.saturating_sub(1);
            } else {
                q.phase += 0.12;
                q.vx *= 0.96;
                q.vy = q.vy * 0.96 + q.kind.weight();
                q.x += q.vx;
                if q.kind == Fruit::Confeti {
                    q.x += 0.14 * q.phase.sin();
                }
                q.y += q.vy;
                let h = q.kind.sprite(q.tone).len() as f32;
                let sand = self.bed(q.x) as f32 - h + 1.0;
                if q.y >= sand {
                    q.y = sand + n.rng.rangef(0.0, 1.5);
                    q.lying = true;
                    q.rest = n.rng.range(900, 2400) as u16;
                }
            }
            n.pieces[i] = q;
        }
        n.pieces.retain(|q| !q.lying || q.rest > 0);
        self.navidad = Some(n);
    }

    /// Lets a piñata fall into the sea ahead of the shark, with a splash
    /// where it comes in.
    fn drop_pinata(&mut self, n: &mut Navidad) {
        let (vw, _) = self.view_size();
        let top = self.view().y;
        let x = self.shark.x + self.shark.heading() * (vw * 0.3 + n.rng.rangef(-10.0, 30.0));
        // From the surface, or from just above the picture when that is
        // further down.
        let surface = SURF as f32 + 2.0;
        let y = surface.max(top - 14.0);
        // Not right under the shark's nose: it would be broken before anyone
        // had seen it.
        let (sx, sy) = self.shark.center();
        if (x - sx).abs() < 45.0 && (y - sy).abs() < 45.0 {
            n.wait = 45;
            return;
        }
        n.pinata = Some(Pinata {
            x,
            y,
            vy: if y == surface { 0.5 } else { 0.07 },
            rot: n.rng.rangef(0.0, TAU),
            spin: n.rng.rangef(0.012, 0.02) * n.rng.sign(),
            phase: n.rng.rangef(0.0, 6.3),
            age: 0,
        });
        if y == surface {
            for _ in 0..8 {
                let (bx, by) = (x + n.rng.rangef(-6.0, 6.0), y + n.rng.rangef(0.0, 5.0));
                self.life
                    .bubble(bx, by, n.rng.chance(0.3), n.rng.rangef(0.0, 6.3));
            }
            for _ in 0..8 {
                let v = (n.rng.rangef(-0.3, 0.3), n.rng.rangef(-0.1, 0.0));
                let at = (x + n.rng.rangef(-7.0, 7.0), SURF as f32 - 0.5);
                self.life
                    .fleck(Speck::Foam, at, v, n.rng.range(20, 45) as u16);
            }
        }
    }

    /// The piñata breaks: its candy and fruit fly out of it, bubbles and
    /// glitter with them, and the schools nearby bolt.
    fn burst_pinata(&mut self, n: &mut Navidad, p: Pinata) {
        n.pinata = None;
        let level = n.dress.level;
        n.wait = (30.0 * (36.0 - 18.0 * level)) as u32 + n.rng.range(0, 300) as u32;
        // A shark that rammed it knocks the pieces on.
        let (mx, my) = self.shark.mouth();
        let near = ((mx - p.x).powi(2) + (my - p.y).powi(2)).sqrt() < 30.0;
        let kick = if near {
            self.shark.speed * self.shark.heading() * 0.5
        } else {
            0.0
        };
        let count = (26.0 + 34.0 * level) as usize;
        let start = n.rng.usize(..Fruit::MIX.len());
        for i in 0..count {
            if n.pieces.len() >= PIECES {
                break;
            }
            let a = n.rng.rangef(0.0, TAU);
            let speed = n.rng.rangef(0.35, 1.25);
            n.pieces.push(Piece {
                x: p.x + a.cos() * 3.0,
                y: p.y + a.sin() * 3.0,
                vx: a.cos() * speed * 1.25 + kick,
                vy: a.sin() * speed * 0.85 - 0.1,
                phase: n.rng.rangef(0.0, 6.3),
                kind: Fruit::MIX[(start + i) % Fruit::MIX.len()],
                tone: n.rng.range(0, 7) as u8,
                lying: false,
                rest: 0,
            });
        }
        for _ in 0..9 {
            let (bx, by) = (p.x + n.rng.rangef(-7.0, 7.0), p.y + n.rng.rangef(-6.0, 6.0));
            self.life
                .bubble(bx, by, n.rng.chance(0.3), n.rng.rangef(0.0, 6.3));
        }
        for _ in 0..14 {
            let v = (n.rng.rangef(-0.5, 0.5), n.rng.rangef(-0.45, 0.25));
            self.life
                .fleck(Speck::Scale, (p.x, p.y), v, n.rng.range(50, 110) as u16);
        }
        self.scatter(p.x, p.y, 40.0);
    }

    // ----- painting -----

    /// Paints what Navidad puts in the sea, onto the floor and behind the
    /// shark: the portals in view with their lights, the candy and fruit,
    /// the piñata.
    pub fn paint_navidad(&mut self) {
        let Some(n) = self.navidad.take() else {
            return;
        };
        let mut found = [(0, 0); 4];
        let (lo, hi) = (self.bx as f32, (self.bx + self.bw) as f32);
        let count = self.portals(&n, (lo, hi), OFR_W as f32 + 110.0, &mut found);
        for &(k, cx) in &found[..count] {
            self.portal(&n, k, cx);
        }
        for q in &n.pieces {
            self.treat(&n.dress, q);
        }
        if let Some(p) = n.pinata {
            self.pinata(&n.dress, &p);
        }
        self.navidad = Some(n);
    }

    /// Draws a drawing in the holiday's colors, its top left at `(left,
    /// top)`, lit for its depth; `w` stands for `glass` and `W` for its
    /// highlight. Faded into what is behind it by `fade`.
    fn thing(&mut self, d: &Dress, sprite: Sprite, (left, top): (i32, i32), glass: Rgb, fade: f32) {
        for (j, row) in sprite.iter().enumerate() {
            for (i, ch) in row.bytes().enumerate() {
                let c = match ch {
                    b'w' => Some(glass),
                    b'W' => Some(glass.lerp(Rgb::WHITE, 0.6)),
                    _ => d.color(ch),
                };
                let Some(c) = c else {
                    continue;
                };
                let (x, y) = (left + i as i32, top + j as i32);
                let c = self.shade(c, y, 0.05);
                if fade >= 1.0 {
                    self.put(x, y, c);
                } else {
                    self.blend(x, y, c, fade);
                }
            }
        }
    }

    /// One piece of candy or fruit.
    fn treat(&mut self, d: &Dress, q: &Piece) {
        let sprite = q.kind.sprite(q.tone);
        // Confetti turns over as it falls, candy comes in every color.
        let glass = match q.kind {
            Fruit::Colacion if q.tone == 6 => BLANCO,
            Fruit::Colacion => d.china[q.tone as usize % 6].lerp(Rgb::WHITE, 0.55),
            _ => d.china[q.tone as usize % 6],
        };
        let sprite = if q.kind == Fruit::Confeti && !q.lying && q.phase.sin() > 0.0 {
            q.kind.sprite(q.tone + 1)
        } else {
            sprite
        };
        // The last moments on the sand, it melts away.
        let fade = if q.lying {
            (q.rest as f32 / 45.0).min(1.0)
        } else {
            1.0
        };
        self.thing(
            d,
            sprite,
            (q.x.round() as i32, q.y.round() as i32),
            glass,
            fade,
        );
    }

    /// The piñata: seven cones of papel de china, each a color, round a
    /// hub, turning, with the end of its cord trailing above.
    fn pinata(&mut self, d: &Dress, p: &Pinata) {
        let (cx, cy) = (p.x.round() as i32, p.y.round() as i32);
        // The cord it was hung by, drifting up behind it.
        let cord = Rgb(184, 148, 92);
        for j in 0..14 {
            let sway = (1.6 * (p.phase * 1.3 + j as f32 * 0.35).sin()).round() as i32;
            self.dot(cx + sway, cy - PINATA_R as i32 - 1 - j, cord);
        }
        let r = PINATA_R.ceil() as i32 + 1;
        let sector = TAU / 7.0;
        for dy in -r..=r {
            for dx in -r..=r {
                let (fx, fy) = (dx as f32, dy as f32);
                let rad = (fx * fx + fy * fy).sqrt();
                if rad > PINATA_R {
                    continue;
                }
                let c = if rad <= 3.0 {
                    // The hub, with a ring round it and a glint.
                    let hub = d.china[1].lerp(Rgb::WHITE, 0.3);
                    if rad > 2.2 {
                        hub.scale(0.8)
                    } else if dx == -1 && dy == -1 {
                        Rgb::WHITE
                    } else {
                        hub
                    }
                } else {
                    let th = fy.atan2(fx) - p.rot;
                    let k = (th / sector).round();
                    let off = th - k * sector;
                    let along = rad * off.cos();
                    let perp = rad * off.sin().abs();
                    let half = 2.4 * (1.0 - (along - 2.0) / (PINATA_R - 2.0)).clamp(0.0, 1.0);
                    if perp > half + 0.35 {
                        continue;
                    }
                    let paper = d.china[(k as i32).rem_euclid(7) as usize % 6];
                    // The fringes: bands of paper, lighter and darker.
                    let c = if (along * 0.8).floor() as i32 % 2 == 0 {
                        paper.lerp(Rgb::WHITE, 0.18)
                    } else {
                        paper.scale(0.88)
                    };
                    if perp > half - 0.6 { c.scale(0.8) } else { c }
                };
                self.dot(cx + dx, cy + dy, c);
            }
        }
    }

    /// One portal de Belén, built up to the level of the posadas, with its
    /// bulbs and farolitos lit.
    fn portal(&mut self, n: &Navidad, k: i64, cx: i32) {
        let d = n.dress;
        let level = d.level;
        let t = self.time();
        let dark = 1.0 - self.daylight;
        let left = cx - OFR_W / 2;
        let foot = self.bed(cx as f32) + 1;
        let seed = self.seed ^ k as u32 ^ 0x4e61_7669;
        // Height of the roof's top edge above the foot, at column `c`.
        let top_h = |c: i32| PEAK - SLOPE * (c as f32 - 18.5).abs();
        let eave = |c: i32| foot - (top_h(c) - THICK).round() as i32;
        let ridge = |c: i32| foot - top_h(c).round() as i32;
        let side = if hash(k as i32, 402, self.seed) & 1 == 0 {
            -1
        } else {
            1
        };

        // Musgo along the foot, as under any nacimiento.
        for c in -2..OFR_W + 2 {
            let x = left + c;
            let bed = self.bed(x as f32);
            let h = hash(x, bed, seed);
            for y in bed - h.is_multiple_of(3) as i32..bed + 2 {
                let tone = if h.is_multiple_of(5) { 0.8 } else { 1.0 };
                self.dot(x, y, MUSGO.scale(tone));
            }
        }

        // The dark inside, lit by the manger.
        for c in POSTS[0].1 + 1..POSTS[1].0 {
            let x = left + c;
            for y in eave(c)..self.bed(x as f32) {
                let plank = (c - POSTS[0].1) % 5 == 0;
                self.dot(
                    x,
                    y,
                    Rgb(30, 36, 52).lerp(Rgb(60, 52, 60), plank as u8 as f32 * 0.4),
                );
            }
        }
        let flicker = 0.85 + 0.15 * (t * 6.7 + k as f32).sin() * (t * 4.1).sin();
        if level >= 0.75 {
            let k = (0.2 + 0.25 * dark) * flicker;
            self.halo(left + 19, foot - 6, WARM, k, 15.0);
        }
        if level >= 0.45 {
            self.thing(
                &d,
                MANGER,
                (left + 13, foot - MANGER.len() as i32),
                Rgb::BLACK,
                1.0,
            );
        }
        if d.nino {
            self.thing(
                &d,
                NINO,
                (left + 18, foot - MANGER.len() as i32 - 1),
                Rgb::BLACK,
                1.0,
            );
            self.halo(
                left + 19,
                foot - 8,
                DORADO,
                (0.3 + 0.3 * dark) * flicker,
                7.0,
            );
        }

        // The posts: weathered stone, mossy at the top.
        for &(c0, c1) in &POSTS {
            for c in c0..=c1 {
                let x = left + c;
                for y in eave(c)..=self.bed(x as f32) {
                    let h = hash(x, y, seed);
                    let course = (foot - y) % 6 == 0;
                    let stone = if y < eave(c) + 3 && h.is_multiple_of(3) {
                        MUSGO.scale(0.9)
                    } else if h.is_multiple_of(31) {
                        MUSGO.scale(0.7)
                    } else if c == c0 {
                        Rgb(120, 112, 120)
                    } else if course || c == c1 {
                        Rgb(70, 64, 76)
                    } else {
                        Rgb(98, 92, 102)
                    };
                    self.dot(x, y, stone);
                }
            }
        }

        // The roof, thatched in heno, ragged along its lower edge.
        for c in -1..=OFR_W {
            let x = left + c;
            let (y0, y1) = (ridge(c), eave(c));
            let straw = hash(x, 0, seed) % 4;
            for y in y0..y1 {
                let h = hash(x, y, seed);
                if y == y1 - 1 && h.is_multiple_of(3) {
                    continue;
                }
                let mut c = HENO.scale(0.8 + 0.07 * straw as f32 + 0.1 * ((h % 5) as f32 / 4.0));
                if y == y0 {
                    c = c.lerp(Rgb::WHITE, 0.14);
                } else if y == y1 - 1 {
                    c = c.scale(0.72);
                }
                if h.is_multiple_of(23) {
                    c = MUSGO;
                }
                self.dot(x, y, c);
            }
        }
        // Strands of heno hanging from it, swaying in the current: more of
        // them with the posadas.
        let step = if level < 0.5 {
            4
        } else if level < 0.8 {
            3
        } else {
            2
        };
        for c in (1..OFR_W - 1).step_by(step) {
            let x = left + c;
            let len = 3 + (hash(c, 1, seed) % 7) as i32 + (level * 4.0) as i32;
            for j in 0..len {
                let f = j as f32 / len as f32;
                let sway = ((t * 0.9 + c as f32 * 0.5 + j as f32 * 0.18).sin() * (0.4 + 1.6 * f))
                    .round() as i32;
                self.dot(x + sway, eave(c) + j, HENO.lerp(Rgb(70, 80, 64), f * 0.6));
            }
        }

        // The foquitos. One string runs along the roofline; from the middle
        // of the posadas on another hangs between the posts, and at the
        // end a third climbs the posts.
        let glow = 0.1 + 0.45 * dark;
        let cord = PINO.scale(0.7);
        let mut bulb = 0usize;
        // Where in the chase of colors this portal's string begins.
        let shift = k.rem_euclid(6) as usize;
        let spacing = (8.0 - 4.0 * level).round() as usize;
        for c in 0..OFR_W {
            self.dot(left + c, ridge(c) - 1, cord);
        }
        for c in (1..OFR_W - 1).step_by(spacing) {
            self.bulb(left + c, ridge(c) - 3, bulb + shift, &d, glow);
            bulb += 1;
        }
        if level >= 0.55 {
            let (a, b) = (POSTS[0].1 + 1, POSTS[1].0 - 1);
            for c in a..=b {
                let u = (c as f32 - 18.5) / 12.0;
                let y = eave(c) + 1 + (7.0 * (1.0 - u * u).max(0.0)) as i32;
                self.dot(left + c, y, cord);
                if (c - a) % 3 == 1 {
                    self.bulb(left + c, y + 1, bulb + shift, &d, glow);
                    bulb += 1;
                }
            }
        }
        if level >= 0.85 {
            for (x, out) in [(left + 2, -2), (left + OFR_W - 3, 1)] {
                let low = self.bed(x as f32);
                for y in eave(2)..low {
                    self.dot(x, y, cord);
                    if (low - y) % 5 == 2 {
                        self.bulb(x + out, y, bulb + shift, &d, glow);
                        bulb += 1;
                    }
                }
            }
        }

        // The Estrella de Belén on the ridge, twinkling.
        if level >= 0.45 {
            let star = d.bulbs[3].lerp(DORADO, 0.5);
            let twinkle = 0.75 + 0.25 * (t * 2.3 + k as f32).sin();
            let (sx, sy) = (left + 19, ridge(19) - 8);
            self.halo(sx, sy, star, (0.12 + 0.4 * dark) * twinkle, 10.0);
            for (j, row) in STAR.iter().enumerate() {
                for (i, ch) in row.bytes().enumerate() {
                    if ch != b'.' {
                        let c = star.scale(twinkle).lerp(Rgb::WHITE, 0.25);
                        self.put(sx - 3 + i as i32, sy - 3 + j as i32, c);
                    }
                }
            }
        }

        // Nochebuenas in clay pots either side, esferas on the sand.
        for x0 in [left - 8, left + OFR_W + 1] {
            let foot = self.bed((x0 + 3) as f32) + 1;
            self.thing(&d, POT, (x0, foot - POT.len() as i32), Rgb::BLACK, 1.0);
        }
        for (i, &(c, from)) in ESFERAS.iter().enumerate() {
            if level >= from {
                let x = left + c;
                let foot = self.bed(x as f32) + 1;
                let glass = [
                    d.bulbs[0], d.bulbs[1], d.bulbs[3], PLATA, d.bulbs[2], d.bulbs[4],
                ][(i + k.rem_euclid(6) as usize) % 6];
                self.thing(&d, ESFERA, (x, foot - ESFERA.len() as i32), glass, 1.0);
            }
        }

        // The path of farolitos that leads the posada to the door.
        let lanterns = (3.0 + 7.0 * level) as i32;
        for s in 0..lanterns {
            let x = if side > 0 {
                left + OFR_W + 5 + s * 7
            } else {
                left - 7 - s * 7
            };
            self.farolito(x, glow, hash(s, k as i32, self.seed));
        }
    }

    /// A foquito, hanging at `(x, y)`, one of the string: it dims and
    /// brightens, and lights the water round it.
    fn bulb(&mut self, x: i32, y: i32, i: usize, d: &Dress, glow: f32) {
        let lit = blink(i, self.time());
        let c = d.bulbs[i % 6];
        let body = c.scale(0.35 + 0.65 * lit);
        self.put(x, y, body.lerp(Rgb::WHITE, 0.35 * lit));
        self.put(x + 1, y, body);
        self.put(x, y + 1, body);
        self.put(x + 1, y + 1, body.scale(0.8));
        self.halo(x, y, c, glow * lit * lit, 6.0);
    }

    /// A farolito standing on the sand at column `x`, its candle flickering.
    fn farolito(&mut self, x: i32, glow: f32, h: u32) {
        let t = self.time();
        let foot = self.bed(x as f32 + 1.0) + 1;
        let flicker =
            0.8 + 0.2 * ((t * 7.0 + (h % 40) as f32).sin() * (t * 11.3 + (h % 17) as f32).sin());
        let top = foot - FAROLITO.len() as i32;
        for (j, row) in FAROLITO.iter().enumerate() {
            for (i, ch) in row.bytes().enumerate() {
                let (px, py) = (x + i as i32, top + j as i32);
                match ch {
                    b'b' => self.dot(px, py, Rgb(176, 140, 84)),
                    b'Y' => self.put(px, py, Rgb(255, 238, 170).scale(flicker)),
                    _ => self.put(px, py, Rgb(255, 190, 90).scale(0.7 + 0.3 * flicker)),
                }
            }
        }
        self.halo(x + 1, top + 2, WARM, (0.14 + 0.4 * glow) * flicker, 6.0);
    }
}

#[cfg(test)]
mod tests {
    use super::super::{INFO, SharkScene};
    use super::*;
    use crate::color::Color;
    use crate::fiesta::Fiesta;
    use crate::render::Canvas;
    use crate::scene::{Opts, Scene};
    use crate::theme::Theme;

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

    /// Portals in view of the camera.
    fn portals_in_view(s: &SharkScene) -> usize {
        let n = s.sea.navidad.as_ref().unwrap();
        let mut found = [(0, 0); 4];
        let (lo, hi) = (s.sea.bx as f32, (s.sea.bx + s.sea.bw) as f32);
        s.sea.portals(n, (lo, hi), 0.0, &mut found)
    }

    /// From the first day to its height there is a portal by the shark and
    /// jellyfish glowing; on an ordinary day the sea is not touched.
    #[test]
    fn the_holiday_comes_to_the_sea() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts, Fiesta::NONE);
        let (_, plain) = first(&off);
        for (m, d) in [(12, 12), (12, 24), (1, 6)] {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (s, cv) = first(&c);
            let n = s.sea.navidad.as_ref().expect("dressed");
            assert!(portals_in_view(&s) > 0, "{m}-{d}: no portal in sight");
            assert!(s.sea.jelly_glow(0).is_some());
            assert!(n.dress.level > 0.0);
            assert!(differing(&plain, &cv) > 100, "{m}-{d}: looks ordinary");
        }
        let (s, _) = first(&off);
        assert!(s.sea.navidad.is_none());
        assert!(s.sea.jelly_glow(0).is_none() && s.sea.navidad_lamp(0).is_none());
        assert!(s.sea.navidad_mote(0, 1.0, 0.0).is_none());
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
            assert!(b.sea.navidad.is_none());
            assert_eq!(differing(&pa, &pb), 0);
            assert_eq!(a.sea.shark.x, b.sea.shark.x);
        }
    }

    /// The foquitos go through their patterns and never go out.
    #[test]
    fn the_foquitos_blink() {
        let mut seen = std::collections::BTreeSet::new();
        for step in 0..2000 {
            let t = step as f32 * 0.05;
            for i in 0..8 {
                let b = blink(i, t);
                assert!((0.3 - 1e-3..=1.0 + 1e-3).contains(&b), "{b}");
                seen.insert((b * 10.0) as u32);
            }
        }
        assert!(seen.len() >= 6, "{seen:?}");
        // Neighbours do not blink as one in the alternating pattern.
        assert!(blink(0, 10.0) != blink(1, 10.0));
    }

    /// The posadas build up: more lights, more portals' worth of decoration.
    #[test]
    fn the_posadas_build_up() {
        let theme = Theme::classic();
        let opts = opts("");
        let lamps = |m, d| {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (s, _) = first(&c);
            s.sea.navidad.as_ref().unwrap().lamps.len()
        };
        assert!(lamps(12, 12) < lamps(12, 24));
        // And the picture itself has more in it: farolitos, bulbs, strings.
        let lit = |m, d| {
            let c = ctx(&theme, &opts, Fiesta::of_day(m, d));
            let (_, cv) = first(&c);
            let plain = first(&ctx(&theme, &opts, Fiesta::NONE)).1;
            differing(&plain, &cv)
        };
        assert!(lit(12, 12) < lit(12, 24));
    }

    /// The holiday comes and goes through `retheme`, as at midnight.
    #[test]
    fn retheme_dresses_and_undresses() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts, Fiesta::NONE);
        let on = ctx(&theme, &opts, Fiesta::of_day(12, 20));
        let (mut s, _) = first(&off);
        let mut cv = Canvas::new(120, 36, Color::Reset);
        for (c, dressed) in [(&on, true), (&off, false), (&on, true)] {
            s.retheme(c);
            assert_eq!(s.sea.navidad.is_some(), dressed);
            for _ in 0..90 {
                s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
            }
            cv.clear();
            s.draw(c, &mut cv);
            // The portal by the shark, or nothing of it.
            assert_eq!(
                s.sea.navidad.as_ref().map_or(0, |_| portals_in_view(&s)) > 0,
                dressed
            );
            // A resize keeps the dress.
            s.resize(c, 120, 36);
            assert_eq!(s.sea.navidad.is_some(), dressed);
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
                for day in [(12, 12), (12, 24), (1, 6)] {
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

    /// Every drawing is as wide in every row and every letter has a color.
    #[test]
    fn the_drawings_fit() {
        let theme = Theme::classic();
        let opts = opts("");
        let c = ctx(&theme, &opts, Fiesta::of_day(12, 24));
        let d = Dress::of(&c).unwrap();
        let fruit = [
            Fruit::Colacion,
            Fruit::Confeti,
            Fruit::Tejocote,
            Fruit::Cana,
            Fruit::Mandarina,
        ];
        // Letters painted by hand: the star's rays, the farolito's light and
        // the glass of esferas, candy and confetti.
        let mut all: Vec<(Sprite, &[u8])> = vec![
            (STAR, b"x"),
            (POT, b""),
            (ESFERA, b"wW"),
            (FAROLITO, b"yY"),
            (MANGER, b""),
            (NINO, b""),
        ];
        for kind in fruit {
            for tone in [0, 1] {
                all.push((kind.sprite(tone), b"wW"));
            }
        }
        for (sprite, hand) in all {
            let w = sprite[0].len();
            for row in sprite {
                assert_eq!(row.len(), w, "{row}");
                for ch in row.bytes().filter(|&c| c != b'.') {
                    assert!(
                        hand.contains(&ch) || d.color(ch).is_some(),
                        "no color for {}",
                        ch as char
                    );
                }
            }
        }
        assert_eq!(Fruit::MIX.len(), 20);
    }

    /// A long swim: piñatas are dropped, the shark goes for them, they burst
    /// into candy and fruit that sinks and lies on the sand.
    #[test]
    fn the_shark_rams_the_pinata() {
        use super::super::hunter::Mood;
        let theme = Theme::classic();
        let opts = opts("day = 60");
        let c = ctx(&theme, &opts, Fiesta::of_day(12, 24));
        let (mut s, mut cv) = first(&c);
        let (mut dropped, mut rammed, mut burst, mut lay) = (false, false, false, false);
        let mut had = false;
        for i in 0..30 * 300 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let n = s.sea.navidad.as_ref().unwrap();
            dropped |= n.pinata.is_some();
            rammed |= s.sea.shark.mood == Mood::Ram;
            burst |= had && n.pinata.is_none() && !n.pieces.is_empty();
            had = n.pinata.is_some();
            lay |= n.pieces.iter().any(|q| q.lying);
            assert!(n.pieces.len() <= PIECES);
            if i % 15 == 0 {
                cv.clear();
                s.draw(&c, &mut cv);
            }
        }
        assert!(
            dropped && rammed && burst && lay,
            "{dropped} {rammed} {burst} {lay}"
        );
    }
}
