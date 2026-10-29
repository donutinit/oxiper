//! Las Fiestas Patrias in the jungle.
//!
//! The tiger becomes a jaguar, the Mexican one, golden with rosettes (see
//! `art.rs`), and the mes patrio dresses the forest around it: [`Dress`] holds
//! the colors, from the brief's own under `classic` and from the theme's
//! accents otherwise. Papel picado in verde, blanco y rojo is strung from tree
//! to tree with the patriotic cut-outs (the águila, the Campana de Dolores, a
//! star, a sombrero de charro, a nopal, a banderita); nopales grow in the
//! undergrowth with their red tunas; and by the path stand the ruins, a great
//! flag in their doorway, flags on their poles, and over them the emblem of the
//! nation: the águila real on its nopal, a serpent in its beak. The jaguar
//! calls on them and roars. The saucer is the Campana de Dolores, swinging on
//! its yoke and ringing. The higher the holiday, the more of all of it, up to
//! the Grito on the night of September 15. Everything here is hashed from the
//! seed, like the jungle it dresses.

use super::art::Sprite;
use super::fiestas::by_hue;
use super::jungle::{FEET, Jungle, View};
use super::muertos::{
    ARCH, BED_CELL, PILLAR, POLE_TOP, Papel, RUIN, RUIN_FOOT, RUIN_H, RUIN_HALF, STONE,
};
use super::{Kind, Rgb, Rng, hash, rgb_of};
use crate::scene::Ctx;

// ----- the colors of the holiday -----

const VERDE: Rgb = Rgb(0, 104, 71);
const BLANCO: Rgb = Rgb(244, 241, 234);
const ROJO: Rgb = Rgb(206, 17, 38);
const AGUILA: Rgb = Rgb(122, 74, 30);
const NOPAL: Rgb = Rgb(79, 138, 42);
const ORO: Rgb = Rgb(212, 160, 23);
const TINTA: Rgb = Rgb(30, 22, 20);
/// The jaguar's coat.
const PELAJE: Rgb = Rgb(214, 160, 70);

/// The holiday's colors, and how far it has come.
#[derive(Clone, Copy, Debug)]
pub struct Dress {
    /// 0 to 1, as in `Fiesta::level`.
    pub level: f32,
    /// The Grito, the night of the 15th, and the parade of the 16th.
    pub grito: bool,
    pub verde: Rgb,
    pub blanco: Rgb,
    pub rojo: Rgb,
    /// The águila's brown, the nopal's green, the bell's and the embroidery's
    /// gold.
    pub aguila: Rgb,
    pub nopal: Rgb,
    pub oro: Rgb,
    /// The ink of rosettes and eyes.
    pub deep: Rgb,
    /// The jaguar's coat.
    pub coat: Rgb,
}

impl Dress {
    /// The brief's own colors.
    fn classic(level: f32, grito: bool) -> Dress {
        Dress {
            level,
            grito,
            verde: VERDE,
            blanco: BLANCO,
            rojo: ROJO,
            aguila: AGUILA,
            nopal: NOPAL,
            oro: ORO,
            deep: TINTA,
            coat: PELAJE,
        }
    }

    /// The dress for today: the holiday's colors, or the theme's own where
    /// it has something close.
    pub fn new(ctx: &Ctx) -> Dress {
        let level = ctx.fiesta.level.clamp(0.0, 1.0);
        let grito = ctx.fiesta.date() == (9, 15);
        if ctx.theme.native {
            return Dress::classic(level, grito);
        }
        let t = ctx.theme;
        let cols: Vec<Rgb> = t.colors.iter().filter_map(|&c| rgb_of(c)).collect();
        if cols.is_empty() {
            return Dress::classic(level, grito);
        }
        let pick = |i: usize| cols[i % cols.len()];
        let near = by_hue(&cols, 40.0, 30.0, &[]);
        let coat = near.unwrap_or_else(|| rgb_of(t.grad.at(0.7)).unwrap_or(PELAJE));
        let verde = by_hue(&cols, 125.0, 50.0, &[]).unwrap_or_else(|| pick(2));
        // A theme with no red takes the first color that is not its green.
        let rojo = by_hue(&cols, 355.0, 35.0, &[verde])
            .or_else(|| cols.iter().copied().find(|&c| c != verde))
            .unwrap_or_else(|| verde.lerp(Rgb::WHITE, 0.5));
        let oro = by_hue(&cols, 45.0, 25.0, &[verde, rojo]).unwrap_or(coat);
        let fg = rgb_of(t.fg).unwrap_or(BLANCO);
        Dress {
            level,
            grito,
            verde,
            blanco: fg,
            rojo,
            aguila: oro.scale(0.55),
            nopal: verde.lerp(fg, 0.15),
            oro,
            deep: coat.scale(0.14),
            coat,
        }
    }

    /// The jaguar's colors by letter, for the tiger of this `kind`: golden,
    /// paler in the taiga, and cream with rosettes in grey for the white one.
    pub fn jaguar(&self, kind: Kind) -> [(u8, Rgb); 14] {
        let (coat, ink) = match kind {
            Kind::Bengal => (self.coat, self.deep),
            Kind::Siberian => (
                self.coat.lerp(self.blanco, 0.3),
                self.deep.lerp(self.coat, 0.15),
            ),
            Kind::White => (
                self.blanco.lerp(self.coat, 0.18),
                self.deep.lerp(self.blanco, 0.4),
            ),
        };
        let belly = self.blanco.lerp(coat, 0.1);
        [
            (b'O', coat),
            (b'o', coat.scale(0.75)),
            (b'S', ink),
            (b's', ink.lerp(coat, 0.25)),
            (b'Y', coat.scale(0.52)),
            (b'W', belly),
            (b'w', belly.scale(0.84)),
            (b'E', ink),
            (b'e', self.nopal.lerp(self.oro, 0.5)),
            (b'k', self.deep),
            (b'n', self.rojo.lerp(self.blanco, 0.45)),
            (b'm', self.rojo.scale(0.5)),
            (b't', self.blanco),
            (b'p', self.rojo.lerp(Rgb::WHITE, 0.4)),
        ]
    }

    /// The emblem's colors by letter: the águila and its serpent, the nopal
    /// with its spines and tunas.
    pub fn emblema(&self) -> [(u8, Rgb); 11] {
        [
            (b'A', self.aguila),
            (b'a', self.aguila.scale(0.7)),
            (b'B', self.aguila.lerp(self.oro, 0.45)),
            (b'K', self.deep),
            (b'D', self.oro),
            (b'G', self.verde),
            (b'g', self.verde.scale(0.65)),
            (b'N', self.nopal),
            (b'n', self.nopal.scale(0.66)),
            (b'k', self.blanco.scale(0.92)),
            (b'R', self.rojo),
        ]
    }

    /// The bell's colors by letter: bronze in light and shade, the yoke's
    /// wood and its straps.
    pub fn campana(&self) -> [(u8, Rgb); 7] {
        let wood = self.aguila.lerp(self.oro, 0.1);
        [
            (b'O', self.oro),
            (b'o', self.oro.scale(0.7)),
            (b'L', self.oro.lerp(Rgb::WHITE, 0.5)),
            (b'Y', wood),
            (b'y', wood.scale(0.66)),
            (b'C', self.oro.scale(0.5)),
            (b'K', self.deep),
        ]
    }

    /// The papel picado: three colors, and the patriotic cut-outs.
    pub fn papel(&self) -> Papel {
        Papel {
            flags: [
                self.verde,
                self.blanco,
                self.rojo,
                self.verde,
                self.blanco,
                self.rojo,
            ],
            cutouts: CUTOUTS,
        }
    }
}

// ----- the drawings -----

/// A campana, cut out of the paper: the crown, the mouth, the clapper.
pub const FLAG_CAMPANA: Sprite = &[
    "########", "###..###", "##....##", "##....##", "#......#", "#......#", "#..##..#", "#.####.#",
    "..#..#..",
];

pub const FLAG_ESTRELLA: Sprite = &[
    "########", "###..###", "###..###", "#......#", "##....##", "#..##..#", "#.#..#.#", "#.####.#",
    "..#..#..",
];

/// The águila with its wings spread.
pub const FLAG_AGUILA: Sprite = &[
    "########", "#.####.#", "#..##..#", "#......#", "##.##.##", "###..###", "###..###", "#.####.#",
    "..#..#..",
];

pub const FLAG_SOMBRERO: Sprite = &[
    "########", "###..###", "###..###", "##....##", "#......#", "########", "#.#..#.#", "#.####.#",
    "..#..#..",
];

pub const FLAG_NOPAL: Sprite = &[
    "########", "###..###", "##....##", "###..###", "#.#..#.#", "##....##", "###..###", "#.####.#",
    "..#..#..",
];

/// A banderita on its pole.
pub const FLAG_BANDERA: Sprite = &[
    "########", "#.....##", "#.###.##", "#.###.##", "#.....##", "#.######", "#.######", "#.####.#",
    "..#..#..",
];

/// The cut-outs of the papel picado.
const CUTOUTS: [Sprite; 6] = [
    FLAG_CAMPANA,
    FLAG_AGUILA,
    FLAG_ESTRELLA,
    FLAG_SOMBRERO,
    FLAG_NOPAL,
    FLAG_BANDERA,
];

/// The eagle on the nopal over the ruins, wings raised. `A` `a` `B` its feathers, `D` beak and talons, `K` its eye, `G` `g` the serpent in its beak, `N` `n` the pencas of the nopal, `k` their spines, `R` the tunas.
pub const EMBLEM_0: Sprite = &[
    ".............AAAAaA.BB..",
    "............AaAAaBBBB...",
    "............AAAaAAaaA...",
    ".....BBB....AAaAaaABBBBB",
    "....AAAA....aAAAAAAaaA..",
    ".DDAKAAAA...AaaAaaaA....",
    "DD.DAAAAA..AaaAaAAA.....",
    "D...AAAAAAAAAAaAA.......",
    "G....AAAAAAAAAA.........",
    "Gg....ABBBAAAAAA........",
    ".G....BBBBBAAAAA........",
    ".Gg...BBBBBAAAAA........",
    "..G...BBBBBAAAAA........",
    "..gG..BBBBBAAAAAAA......",
    "....G..BBBAAAAAAAA......",
    ".....G..DDDDNNAAAA......",
    ".......DDDDDDNBBBB......",
    ".......NRRNNNNNNN.......",
    "......NNNnNNnNNRRn......",
    "......NNNN.nN.kNNN......",
    ".....NNkNNn..NNNkNn.....",
    ".....NNNNNN..NNNNnn.....",
    "......NNNkNNNNNNNn......",
    "......RRNNNNNNNRRn......",
    ".......NkNNNNNNNn.......",
    ".......NNNNNkNNNn.......",
    ".......NNNNNNNNkn.......",
    ".......NNNkNNNNnn.......",
    "........NNNNNNnn........",
    ".........NNNNNN.........",
];

/// The same, wings lowered.
pub const EMBLEM_1: Sprite = &[
    ".................BB.BB..",
    ".................AaaA...",
    "...............AAAAABB..",
    ".....BBB.......AaaAABB.B",
    "....AAAA......AaAAAAaaBB",
    ".DDAKAAAA....AaAAAaaA...",
    "DD.DAAAAA...AaAAaaAABBB.",
    "D...AAAAAAAAAAAAAAAAA...",
    "G....AAAAAAAAAAaaaaaaaBB",
    "Gg....ABBBAAAAAAAAAAA...",
    ".G....BBBBBAAAAAaaAABBB.",
    ".Gg...BBBBBAAAAAAAaaA...",
    "..G...BBBBBAAAAA..AAaBBB",
    "..gG..BBBBBAAAAAAA......",
    "....G..BBBAAAAAAAA......",
    ".....G..DDDDNNAAAA......",
    ".......DDDDDDNBBBB......",
    ".......NRRNNNNNNN.......",
    "......NNNnNNnNNRRn......",
    "......NNNN.nN.kNNN......",
    ".....NNkNNn..NNNkNn.....",
    ".....NNNNNN..NNNNnn.....",
    "......NNNkNNNNNNNn......",
    "......RRNNNNNNNRRn......",
    ".......NkNNNNNNNn.......",
    ".......NNNNNkNNNn.......",
    ".......NNNNNNNNkn.......",
    ".......NNNkNNNNnn.......",
    "........NNNNNNnn........",
    ".........NNNNNN.........",
];

/// The Campana de Dolores, hanging from its yoke. `O` `o` `L` bronze in light and shade, `Y` `y` the wooden yoke, `C` its straps and the clapper, `K` the dark of its mouth.
pub const BELL: Sprite = &[
    ".YYYYYCCYYYYYCCYYYY.",
    ".yyyyyCCyyyyyCCyyyy.",
    "......CCoooo.CC.....",
    "......OLLOOooo......",
    "......OLLOOooo......",
    "......OLOOOooo......",
    "......LLOOOOoo......",
    "......LLOOOOoo......",
    ".....OLLOOOOooo.....",
    ".....oooooooooo.....",
    ".....LLOOOOOOoo.....",
    "....OLLOOOOOOooo....",
    "....LLOOOOOOOOoo....",
    "....LLOOOOOOOOoo....",
    "...LLOOOOOOOOOOoo...",
    "...oooooooooooooo...",
    "..LLOOOOOOOOOOOOoo..",
    "..LLOOOOOOOOOOOOoo..",
    ".LLOOOOOOOOOOOOOOoo.",
    "oooooooooooooooooooo",
    "..KKKKKKKKKKKKKKKK..",
    ".........CC.........",
    ".........CC.........",
];

/// The emblem, in its two phases of the flap.
const EMBLEMA: [Sprite; 2] = [EMBLEM_0, EMBLEM_1];

/// How much taller the bell is than the saucer it replaces.
pub const BELL_EXTRA: f32 = 13.0;

/// Where the emblem stands on the ruin: the column of the ruin's drawing and the row of its wall
/// top, and the size of the emblem.
const EMBLEM_AT: (f32, i32) = (17.5, 4);

/// The great flag in the doorway: its size and the row of the ruin it hangs from.
const GRAN_BANDERA: (i32, i32, i32) = (12, 18, 10);

impl Jungle {
    // ----- the ruins and their emblem -----

    /// The ruins in view: the stone, a great flag in the doorway, banderitas along its arch, flags
    /// on the poles, and the águila on its nopal over the wall.
    pub fn paint_emblemas(&mut self) {
        let Some(d) = self.pal.patrias else { return };
        let ruins = self.ruins(self.bx as f32, (self.bx + self.bw) as f32);
        if ruins.as_slice().is_empty() {
            return;
        }
        let night = 1.0 - self.daylight;
        let stone = self.lut(&STONE);
        let emblem = self.lut_glow(&d.emblema(), 0.15 + 0.3 * night);
        let frame = ((self.tick / 20) % 2) as usize;
        for r in ruins.as_slice() {
            let face = if r.flip { -1.0 } else { 1.0 };
            self.sprite(RUIN, r.x, RUIN_FOOT as f32, face, &stone);
            self.poles(r, [d.verde, d.rojo]);
            #[cfg(test)]
            {
                self.painted.ruins += 1;
            }
            let x0 = r.x.round() as i32 - RUIN_HALF as i32;
            let y0 = RUIN_FOOT - RUIN_H + 1;
            self.gran_bandera(x0, y0, &d);
            if d.level >= 0.5 {
                self.banderines(x0, y0, r.id, &d);
            }
            self.mastiles(r.x, &d);
            // The águila stands on the wall, over the doorway, facing the way it always does.
            let col = if r.flip {
                (2 * RUIN_HALF as i32 - 1) as f32 - EMBLEM_AT.0
            } else {
                EMBLEM_AT.0
            };
            let sprite = EMBLEMA[frame];
            let top = y0 + EMBLEM_AT.1 - sprite.len() as i32;
            self.stamp(
                sprite,
                x0 + col as i32 - sprite[0].len() as i32 / 2,
                top,
                &emblem,
            );
            #[cfg(test)]
            {
                self.painted.emblems += 1;
            }
        }
    }

    /// The great flag hung in the doorway, rippling: verde, blanco and rojo, and the águila's
    /// brown at its heart.
    fn gran_bandera(&mut self, x0: i32, y0: i32, d: &Dress) {
        let (w, h, row) = GRAN_BANDERA;
        let night = 1.0 - self.daylight;
        // It hangs in the light of the doorway's lamps: lit by the day, then brought back to its
        // own colors at night.
        let tone = |c: Rgb| self.lit(c).lerp(c, 0.2 + 0.5 * night);
        let (v, b, r, e) = (tone(d.verde), tone(d.blanco), tone(d.rojo), tone(d.aguila));
        let time = self.time();
        let left = x0 + 28 - w / 2;
        // A wooden rail along the top, and the cloth hanging from it, its columns swaying a pixel
        // or so one after another.
        let rail = self.lit(d.aguila);
        for c in -1..=w {
            self.put(left + c, y0 + row - 1, rail);
        }
        for c in 0..w {
            let color = [v, b, r][(c * 3 / w) as usize];
            let sway = ((time * 2.2 + c as f32 * 0.7).sin() * 0.9).round() as i32;
            for k in 0..h {
                let shade = if (c * 5 + k) % 7 == 0 { 0.9 } else { 1.0 };
                // The first rows hang straight from the rail.
                let dy = if k < 2 { 0 } else { sway };
                self.put(left + c, y0 + row + k + dy, color.scale(shade));
            }
        }
        // The águila: a small brown cross in the middle of the white.
        let (mx, my) = (left + w / 2, y0 + row + h / 2);
        for (dx, dy) in [(-1, 0), (0, 0), (1, 0), (0, -1), (0, 1)] {
            self.put(mx + dx, my + dy, e);
        }
        #[cfg(test)]
        {
            self.painted.flags += 1;
        }
    }

    /// Banderitas along the doorway's arch, in the three colors.
    fn banderines(&mut self, x0: i32, y0: i32, id: i32, d: &Dress) {
        let colors = [d.verde, d.blanco, d.rojo].map(|c| self.lit(c));
        let mut k = id.unsigned_abs() as usize;
        for &(r, c) in &ARCH {
            for x in [x0 + c, x0 + 2 * RUIN_HALF as i32 - 1 - c] {
                let color = colors[k % 3];
                // A pennant: two pixels hanging from the arch's edge, a point below.
                self.put(x, y0 + r - 1, color);
                self.put(x, y0 + r, color);
                self.put(x, y0 + r + 1, color.scale(0.8));
                k += 1;
            }
        }
    }

    /// A flag on the top of each pole of the ruin, waving in the wind.
    fn mastiles(&mut self, rx: f32, d: &Dress) {
        let colors = [d.verde, d.blanco, d.rojo].map(|c| self.lit(c));
        let time = self.time();
        for (i, px) in [rx - PILLAR, rx + PILLAR].into_iter().enumerate() {
            let x = px.round() as i32 + 1;
            // Nine wide and six tall, and on the night of the Grito a third as big again; the wind
            // raises it along its length.
            let (w, h) = if d.grito { (13, 8) } else { (9, 6) };
            for c in 0..w {
                let color = colors[(c * 3 / w) as usize];
                let wave = (time * 3.0 + c as f32 * 0.8 + i as f32 * 2.0).sin() * 0.8;
                for k in 0..h {
                    self.put(
                        x + c,
                        POLE_TOP as i32 + k + (wave * (c as f32 / w as f32)).round() as i32,
                        color,
                    );
                }
            }
            #[cfg(test)]
            {
                self.painted.flags += 1;
            }
        }
    }

    // ----- papel picado -----

    /// The strings of papel picado in verde, blanco y rojo, from tree to tree and to the poles of
    /// the ruins.
    pub fn paint_banderas(&mut self) {
        let Some(d) = self.pal.patrias else { return };
        self.hang_papel(d.level, &d.papel());
    }

    /// The strings of the middle layer, in the haze.
    pub fn paint_banderas_far(&mut self, v: View) {
        let Some(d) = self.pal.patrias else { return };
        self.hang_papel_far(v, d.level, &d.papel());
    }

    // ----- nopales -----

    /// A nopal plant standing with its foot at `(x, base)`: a penca or two with their spines and,
    /// along the rim, red tunas. `h` is its dice.
    pub fn nopal_plant(&mut self, x: i32, base: i32, h: u32, big: bool, d: &Dress) {
        let (pad, edge) = (
            self.lit(self.green(d.nopal)),
            self.lit(self.green(d.nopal.scale(0.66))),
        );
        let spine = self.lit(d.blanco.scale(0.9));
        let tuna = self.lit(d.rojo);
        let (rx, ry) = if big { (3.6, 3.2) } else { (2.6, 2.2) };
        let penca = |me: &mut Jungle, cx: i32, cy: i32, rx: f32, ry: f32, seed: u32| {
            for dy in -(ry.ceil() as i32)..=ry.ceil() as i32 {
                for dx in -(rx.ceil() as i32)..=rx.ceil() as i32 {
                    let q = (dx as f32 / rx).powi(2) + (dy as f32 / ry).powi(2);
                    if q <= 1.0 {
                        let rim = q > 0.55 && (dx > 0 || dy > 0);
                        me.put(cx + dx, cy + dy, if rim { edge } else { pad });
                    }
                }
            }
            if seed.is_multiple_of(2) {
                me.put(cx - 1, cy - 1, spine);
            }
            if (seed >> 1).is_multiple_of(2) {
                me.put(cx + 1, cy, spine);
            }
            // A tuna on the upper rim.
            let side = if (seed >> 2) & 1 == 0 { -1 } else { 1 };
            me.put(cx + side * (rx as i32), cy - ry as i32 + 1, tuna);
            me.put(cx + side * (rx as i32), cy - ry as i32, tuna);
        };
        let cy = base - ry as i32;
        penca(self, x, cy, rx, ry, h);
        if big {
            // A second penca grows from the first, leaning to one side.
            let lean = if (h >> 3) & 1 == 0 { -2 } else { 2 };
            penca(self, x + lean, cy - 5, rx * 0.72, ry * 0.8, h >> 4);
        }
        #[cfg(test)]
        {
            self.painted.blossoms += 1;
        }
    }

    /// The nopales of the undergrowth, in the same beds the other holidays plant their flowers
    /// in: the back row stands behind the animals, the front row, smaller, in front of their paws.
    pub fn paint_nopales(&mut self, front: bool) {
        let Some(d) = self.pal.patrias else { return };
        let (bx, bw) = (self.bx, self.bw);
        let (u0, u1) = (
            self.base as f64 + bx as f64,
            self.base as f64 + (bx + bw) as f64,
        );
        let cells =
            (u0 / BED_CELL as f64).floor() as i64 - 1..=(u1 / BED_CELL as f64).floor() as i64 + 1;
        for cell in cells {
            let Some((a, b)) = self.bed_in(cell, 0.15 + 0.4 * d.level) else {
                continue;
            };
            // One nopal to every ten pixels of the bed or so.
            let n = (((b - a) / 10.0) as i32).max(1);
            for i in 0..n {
                let h = hash(cell as i32, i + 7 * front as i32, self.seed ^ 0x2a9a);
                let x = (a
                    + (i as f32 + 0.3 + 0.4 * (h & 0xff) as f32 / 255.0) * (b - a) / n as f32)
                    .round() as i32;
                if x < bx - 8 || x > bx + bw + 8 {
                    continue;
                }
                let base = if front {
                    FEET + 5 + ((h >> 8) % 4) as i32
                } else {
                    FEET - 3 + ((h >> 8) % 4) as i32
                };
                self.nopal_plant(x, base, h >> 12, !front, &d);
            }
        }
    }

    // ----- the Campana de Dolores -----

    /// How far the bell has swung, in radians: clockwise on screen, positive.
    fn swing(&self) -> f32 {
        0.42 * (self.tick as f32 * 0.14).sin()
    }

    /// The saucer's place is taken by the Campana de Dolores: sound ringing out from its mouth,
    /// then the bell itself, swinging from its yoke.
    pub fn draw_campana(&mut self, d: &Dress) {
        use super::body::Shears;
        let (x, y) = (self.ufo.x, self.ufo.y);
        let mut lut = [None; 128];
        for &(ch, c) in &d.campana() {
            lut[ch as usize] = Some(c);
        }
        let angle = self.swing();
        let sh = Shears::new(angle);
        let h = BELL.len() as i32;
        let (x0, y0) = (
            x.round() as i32 - BELL[0].len() as i32 / 2,
            y.round() as i32 + 10 - h,
        );
        // It swings about the middle of its yoke.
        let (px, py) = (BELL[0].len() as i32 / 2, 1);
        for (r, row) in BELL.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                if let Some(col) = lut[(ch & 127) as usize] {
                    let (dx, dy) = sh.turn(c as i32 - px, r as i32 - py);
                    self.put(x0 + px + dx, y0 + py + dy, col);
                }
            }
        }
        #[cfg(test)]
        {
            self.painted.campana = true;
        }
        // Where its mouth is now, and the sound spreading from it in arcs, `)))` and `(((`.
        let (mx, my) = {
            let (dx, dy) = sh.turn(0, h - 3 - py);
            ((x0 + px + dx) as f32, (y0 + py + dy) as f32)
        };
        let tick = self.tick as f32;
        let ring = d.oro.lerp(d.blanco, 0.5);
        for k in 0..3 {
            let r = 7.0 + (tick * 0.5 + k as f32 * 10.0) % 30.0;
            let a = 0.55 * (1.0 - (r - 7.0) / 30.0);
            let mut t = -0.55f32;
            while t <= 0.55 {
                for side in [-1.0f32, 1.0] {
                    let (qx, qy) = (mx + side * r * t.cos() * 1.1, my - 4.0 + r * t.sin() * 0.7);
                    self.blend(qx.round() as i32, qy.round() as i32, ring, a);
                }
                t += 0.09;
            }
        }
    }

    /// Confetti in the three colors rising up the beam from the ground to the bell, as many as
    /// the beam is strong. `top` is the row the beam starts on.
    pub fn beam_confeti(&mut self, x: f32, top: i32, beam: f32) {
        let Some(d) = self.pal.patrias else { return };
        let n = (28.0 * beam) as i32;
        let tick = self.tick as f32;
        let height = (FEET + 1 - top).max(1) as f32;
        let colors = [d.verde.lerp(Rgb::WHITE, 0.25), d.blanco, d.rojo, d.oro];
        for k in 0..n {
            let age = (tick * 0.012 + k as f32 / 28.0).fract();
            // Up, so the older the piece, the nearer the bell.
            let y = FEET as f32 - age * height;
            let depth = 1.0 - age;
            let hw = (4.0 + 12.0 * depth) * 0.85;
            let px = x + hw * (age * 11.0 + k as f32 * 1.9).sin();
            let c = colors[(k % 4) as usize];
            let (ix, iy) = (px.round() as i32, y.round() as i32);
            self.put(ix, iy, c);
            if k % 2 == 0 {
                self.put(ix + 1, iy, c);
            }
        }
    }

    /// The bell's glints, straight onto the screen: a little gold sparking along its lip as it
    /// swings.
    pub fn campana_lights(&mut self, v: View, d: &Dress) {
        let (x, y) = (self.ufo.x, self.ufo.y + 7.0);
        let (sx, sy) = v.to_screen(x, y);
        let r = (BELL[0].len() as f32 / 2.0 * v.z) as i32 + 2;
        let tick = self.tick;
        let swing = (self.swing() * 12.0 * v.z) as i32;
        for dx in -r..=r {
            if (dx + (tick / 4) as i32).rem_euclid(5) == 0 {
                let c = if (dx / 5).rem_euclid(2) == 0 {
                    d.oro.lerp(Rgb::WHITE, 0.6)
                } else {
                    d.oro
                };
                for dy in -1..=1 {
                    let a = if dy == 0 { 0.5 } else { 0.18 };
                    self.blend_out(sx + dx - swing / 2, sy + dy, c, a);
                }
            }
        }
    }

    /// The jaguar's cheer: "¡Viva México!" over its head as it roars, in one of the three colors
    /// each time, and a shower of confetti.
    pub fn cheer(&mut self, d: &Dress, rng: &mut Rng) {
        let (x, y) = (self.tiger.jaws(), FEET as f32 - 40.0);
        let colors = [d.verde.lerp(Rgb::WHITE, 0.3), d.blanco, d.rojo];
        let which = (self.tiger.steps / 150) as usize + (hash(x as i32, 0, self.seed) % 3) as usize;
        self.note(x, y, "¡VIVA MÉXICO!", colors[which % 3], 80, 0.22);
        self.life.petals(x, y + 8.0, 14, rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::tiger::art;

    /// Every drawing is a rectangle, and every letter in it has a color.
    #[test]
    fn the_drawings_fit_together() {
        let dress = Dress::classic(1.0, true);
        let jaguar = [
            art::JAGUAR_TORSO,
            art::JAGUAR_FRONT,
            art::JAGUAR_SLEEP_0,
            art::JAGUAR_SLEEP_1,
        ]
        .into_iter()
        .chain(art::JAGUAR_SKIN.heads);
        type Set = (&'static str, Vec<Sprite>, Vec<(u8, Rgb)>);
        let sets: [Set; 4] = [
            (
                "jaguar",
                jaguar.collect(),
                dress.jaguar(Kind::Bengal).into(),
            ),
            ("cut-outs", CUTOUTS.to_vec(), vec![(b'#', Rgb::WHITE)]),
            ("emblem", EMBLEMA.to_vec(), dress.emblema().into()),
            ("bell", vec![BELL], dress.campana().into()),
        ];
        for (name, sprites, pal) in sets {
            for s in sprites {
                let w = s[0].len();
                for row in s {
                    assert_eq!(row.len(), w, "{name}: {row}");
                    for ch in row.bytes().filter(|&c| c != b'.') {
                        assert!(pal.iter().any(|&(k, _)| k == ch), "{name}: {}", ch as char);
                    }
                }
            }
        }
        // A paper flag's rows swing as far as it has rows, like the others'.
        assert!(CUTOUTS.iter().all(|f| f.len() == 9 && f[0].len() == 8));
        assert_eq!(EMBLEMA[0].len(), EMBLEMA[1].len());
        assert_eq!(EMBLEMA[0][0].len(), EMBLEMA[1][0].len());
        // The bell stands taller than the saucer, as the camera counts on.
        assert_eq!(BELL.len() as f32 - art::UFO[0].len() as f32, BELL_EXTRA);
        // The jaguar takes the tiger's place: the same size in every drawing.
        let (a, b) = (art::JAGUAR_SKIN, art::TIGER_SKIN);
        assert_eq!(a.torso.len(), b.torso.len());
        assert_eq!(a.torso[0].len(), b.torso[0].len());
        assert_eq!(a.front.len(), b.front.len());
        assert_eq!(a.torso_at, b.torso_at);
    }
}
