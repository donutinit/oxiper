//! The Jueves de Corpus in the meadow: the atrium of a church, with its
//! papel picado, a stand of mulitas de hoja de maíz of every size, and the
//! children in manta and rebozo who come to the church door with huacales of
//! flowers on their backs, leading a mulita.

use super::*;

/// Seeds the Jueves de Corpus' own dice.
const CORPUS_SEED: u64 = 0x0043_4F52_5055_5300;

/// Papel de china: rosa mexicano, amarillo, morado, naranja and limón.
pub(super) const PAPELES: [Rgb; 5] = [
    Rgb(228, 0, 124),
    Rgb(255, 210, 0),
    Rgb(112, 40, 160),
    Rgb(255, 140, 26),
    Rgb(124, 179, 66),
];
pub(super) const HOJA: Rgb = Rgb(217, 196, 138);
const TOSTADA: Rgb = Rgb(176, 141, 74);
const MANTA: Rgb = Rgb(239, 230, 207);
pub(super) const REBOZO: Rgb = Rgb(42, 92, 170);
/// The wood of a huacal.
pub(super) const HUACAL: Rgb = Rgb(160, 112, 58);
/// The whitewash of the church.
pub(super) const CAL: Rgb = Rgb(236, 226, 200);

/// The children and the mulita who walk to the church, facing right, one
/// pixel per char and two steps each: `h` hair, `s` face, `m` manta, `r`
/// rebozo, `w` the wood of a huacal, `p`, `y`, `v`, `o` and `g` the flowers,
/// fruit and leaves heaped in it, `l` huaraches, `H` and `T` the hoja de maíz
/// and its toasted folds.
const NINO: [Sprite; 2] = [
    &[
        "py.hh...", "vg.ss...", "wwwmmm..", "wowmmm..", "wwwmmm..", "...mmm..", "...mmm..",
        "...l.l..", "..l...l.",
    ],
    &[
        "py.hh...", "vg.ss...", "wwwmmm..", "wowmmm..", "wwwmmm..", "...mmm..", "...mmm..",
        "....l...", "...ll...",
    ],
];
const NINA: [Sprite; 2] = [
    &[
        "po.hh...", "yg.ss...", "wwwrrr..", "wywrrr..", "wwwmrr..", "..mmmm..", "..mmmm..",
        "..l..l..", ".l....l.",
    ],
    &[
        "po.hh...", "yg.ss...", "wwwrrr..", "wywrrr..", "wwwmrr..", "..mmmm..", "..mmmm..",
        "...ll...", "...ll...",
    ],
];
const MULITA: [Sprite; 2] = [
    &[
        "..vgpy..H.H.",
        "..wwwww.HHH.",
        ".HrwwowrmHmH",
        "HHrrrrrrHHHH",
        "THHHHHHHHH..",
        ".THHHHHHHH..",
        "..H.H..H.H..",
        "..T.T..T.T..",
    ],
    &[
        "..vgpy..H.H.",
        "..wwwww.HHH.",
        ".HrwwowrmHmH",
        "HHrrrrrrHHHH",
        "THHHHHHHHH..",
        ".THHHHHHHH..",
        "...HH..HH...",
        "...TT..TT...",
    ],
];

/// The mulitas of the stand, in three sizes, and the vendor, facing right: the same
/// letters, `m` here the manta of an eye.
const MULITA_G: Sprite = &[
    ".....yv.gpyp...",
    "....vpygypvpy..",
    ".....gyppvyyg..",
    ".H..wwwwwwwww..",
    ".HH.wwwowwwow..",
    "mHH.wwwwwwwww..",
    "HHHHrrrrrrrrHHH",
    ".HHHHHHHHHHHHHH",
    ".THHHHHHHHHHHH.",
    "..THHHHHHHHHHH.",
    "..HH.HH..HH.HH.",
    "..TT.TT..TT.TT.",
];
const MULITA_M: Sprite = &[
    "..vgpy..H.H.",
    "..wwwww.HHH.",
    ".HrwwowrmHmH",
    "HHrrrrrrHHHH",
    "THHHHHHHHH..",
    ".THHHHHHHH..",
    "..H.H..H.H..",
    "..T.T..T.T..",
];
const MULITA_C: Sprite = &[
    "....gyp.", ".H..www.", "mHH.wow.", "HHHrrrrH", ".HHHHHHH", ".H.H.H.H",
];

/// The vendor of the stand, 6 by 13: `h` her braids, `s` her face, `r` her rebozo
/// and `m` her manta.
const VENDEDORA: Sprite = &[
    "..hh..", ".hhhh.", ".ssss.", ".ssss.", "rrrrrr", "rrmmrr", "rrmmrr", "rrmmrr", "mmmmmm",
    "mmmmmm", "mmmmmm", "mmmmmm", "mmmmmm",
];

/// How wide the stand of mulitas is, between its poles.
const STAND_W: i32 = 40;
/// How far apart the poles of the papel picado are.
const POLES_PITCH: i32 = 38;

/// Who walks to the church: a child in manta, a girl in rebozo, or a mulita.
#[derive(Clone, Copy, PartialEq)]
enum Kid {
    Nino,
    Nina,
    Mulita,
}

/// Where a walker of the line stands behind its head, and who it is.
pub(super) struct Walker {
    pub(super) off: f32,
    kid: Kid,
}

/// The Jueves de Corpus in the meadow, laid out for one world.
pub(super) struct Corpus {
    base: i32,
    /// The church's left edge and the stand's left pole.
    church: i32,
    pub(super) stand: Option<i32>,
    /// The poles the strands of papel picado hang from, and the atrial cross.
    pub(super) poles: Vec<i32>,
    cross: i32,
    pub(super) line: Line,
    pub(super) walkers: Vec<Walker>,
    papeles: [Rgb; 5],
    rebozo: Rgb,
}

impl Corpus {
    /// The holiday for this world, if it is on.
    pub(super) fn of(ctx: &Ctx, m: &Meadow) -> Option<Corpus> {
        ctx.fiesta
            .is(Kind::Corpus)
            .then(|| Corpus::new(ctx, m.w, m.fence))
    }

    fn new(ctx: &Ctx, w: i32, fence: i32) -> Corpus {
        let mut rng = Rng::with_seed(ctx.seed ^ CORPUS_SEED);
        let base = fence + 4;
        // The church stands at one end of the street, the stand at the
        // other, and the children come in from the far side.
        let left = rng.bool();
        let church = if left { 4 } else { w - 66 };
        let door = church + 34;
        let stand = (w >= 150).then(|| if left { w - STAND_W - 12 } else { 12 });
        let mut poles = vec![if left { church + 62 } else { church - 2 }];
        loop {
            let last = *poles.last().unwrap_or(&0);
            let next = if left {
                last + POLES_PITCH
            } else {
                last - POLES_PITCH
            };
            if next < -POLES_PITCH || next > w + POLES_PITCH {
                break;
            }
            poles.push(next);
        }
        let cross = if left { church + 70 } else { church - 8 };
        let way = if left { -1.0 } else { 1.0 };
        // Children and their mulitas, one behind the other.
        let mut walkers = Vec::new();
        let mut off = 0.0;
        for i in 0..6 {
            let kid = match i % 3 {
                0 => {
                    if rng.bool() {
                        Kid::Nino
                    } else {
                        Kid::Nina
                    }
                }
                1 => Kid::Mulita,
                _ => {
                    if rng.bool() {
                        Kid::Nina
                    } else {
                        Kid::Nino
                    }
                }
            };
            walkers.push(Walker { off, kid });
            off += if kid == Kid::Mulita { 16.0 } else { 11.0 } + rng.rangef(0.0, 4.0);
        }
        let mut line = Line::new(w as f32, door as f32, way, off, 0.0);
        line.walk = rng.rangef(0.3, 0.7) * line.far();
        let [rebozo] = themed_at(ctx, [REBOZO], [2]);
        Corpus {
            base,
            church,
            stand,
            poles,
            cross,
            line,
            walkers,
            papeles: themed(ctx, PAPELES, 0),
            rebozo,
        }
    }

    /// The rows of the meadow the atrium covers, which the grass leaves clear.
    pub(super) fn cleared(&self, fence: i32) -> (i32, i32) {
        (fence + 1, self.base + 5)
    }

    /// The row of the strand between the poles at `xa` and `xb`, at `x`.
    fn strand_y(&self, j: i32, xa: i32, xb: i32, x: i32) -> i32 {
        let t = (x - xa) as f32 / (xb - xa).max(1) as f32;
        self.base - 50 + 7 * j + (3.0 * 4.0 * t * (1.0 - t)) as i32
    }

    /// One step: the children walk on.
    pub(super) fn step(&mut self) {
        self.line.step(0.2);
    }
}

impl Meadow {
    /// Fills the upper half of a disc: a dome, an arch.
    pub(super) fn dome(&mut self, cx: i32, cy: i32, r: i32, c: Rgb) {
        for y in cy - r..=cy {
            let dy = (cy - y) as f32 + 0.5;
            let half = ((r * r) as f32 - dy * dy).max(0.0).sqrt();
            let (a, b) = (
                (cx as f32 - half).round() as i32,
                (cx as f32 + half).round() as i32,
            );
            self.fill(a, y, b - a, 1, c);
        }
    }

    /// Fills a triangle standing on row `base - 1`, `h` rows tall and
    /// `half` wide either side of `cx` at its foot: a gable, a cap.
    pub(super) fn gable(&mut self, cx: i32, base: i32, h: i32, half: f32, c: Rgb) {
        for r in 0..h {
            let hw = half * (1.0 - r as f32 / h as f32);
            let (a, b) = (
                (cx as f32 - hw).round() as i32,
                (cx as f32 + hw).round() as i32,
            );
            self.fill(a, base - 1 - r, b - a, 1, c);
        }
    }

    /// The atrium: its paving, the church with its tower, dome and lit
    /// door, the cross of stone, the stand of mulitas and the strands of
    /// papel picado between their poles.
    pub(super) fn draw_atrio(&mut self, co: &Corpus) {
        let (w, b) = (self.w, co.base);
        // The paving: laid stones, warm and pale.
        let stones =
            [Rgb(176, 160, 138), Rgb(162, 146, 126), Rgb(186, 170, 146)].map(|c| self.shade(c));
        let mortar = self.shade(Rgb(120, 108, 94));
        for y in b + 1..=self.ground + 5 {
            let r = (y - b - 1) / 3;
            for x in 0..w {
                let o = x + 3 * (r & 1);
                let c = if (y - b - 1) % 3 == 2 || o % 6 == 5 {
                    mortar
                } else {
                    stones[(hash(o / 6, r, 0xC027) % 3) as usize]
                };
                self.put(x, y, c);
            }
        }
        self.draw_iglesia(co);
        // The cross of the atrium, in stone.
        let stone = self.shade(Rgb(170, 164, 152));
        self.fill(co.cross - 2, b - 3, 5, 4, stone);
        self.fill(co.cross, b - 16, 2, 14, stone);
        self.fill(co.cross - 3, b - 13, 8, 2, stone);
        if let Some(sx) = co.stand {
            self.draw_puesto_de_mulitas(co, sx);
        }
        self.draw_papel_picado(co);
    }

    /// The church, its left edge at `co.church`: a tower with a bell in its
    /// belfry and a cap of tiles, the nave with its gable and rose window,
    /// the door open on the candles inside, a dome behind it.
    fn draw_iglesia(&mut self, co: &Corpus) {
        let (b, x0) = (co.base, co.church);
        let wall = self.shade(CAL);
        let light = self.shade(CAL.lerp(Rgb::WHITE, 0.4));
        let shade = self.shade(CAL.scale(0.82));
        let tiles = [self.shade(TEJA), self.shade(TEJA.scale(0.82))];
        let glow = self.candlelit(Rgb(255, 222, 150));
        let dark = self.shade(Rgb(70, 56, 48));
        // The dome behind the nave, with its drum, lantern and cross.
        let dome = self.shade(TEJA.lerp(Rgb(214, 170, 90), 0.25));
        self.fill(x0 + 42, b - 29, 16, 4, shade);
        self.dome(x0 + 50, b - 29, 10, dome);
        self.fill(x0 + 49, b - 43, 2, 4, glow);
        self.fill(x0 + 50, b - 49, 1, 6, light);
        self.fill(x0 + 48, b - 47, 5, 1, light);
        // The nave, with a low wing to the side of the dome.
        self.fill(x0 + 54, b - 18, 9, 19, shade);
        self.fill(x0 + 14, b - 24, 40, 25, wall);
        self.fill(x0 + 14, b - 25, 40, 1, light);
        self.fill(x0 + 14, b - 24, 2, 25, light);
        self.fill(x0 + 52, b - 24, 2, 25, light);
        self.gable(x0 + 34, b - 25, 10, 20.0, light);
        self.gable(x0 + 34, b - 25, 9, 18.0, wall);
        self.fill(x0 + 34, b - 41, 2, 6, light);
        self.fill(x0 + 32, b - 39, 6, 2, light);
        // The rose window.
        self.dome(x0 + 35, b - 29, 3, light);
        self.fill(x0 + 32, b - 29, 7, 3, light);
        self.dome(x0 + 35, b - 29, 2, glow);
        self.fill(x0 + 33, b - 29, 5, 2, glow);
        // The door, in a frame of stone, open on the candles.
        self.fill(x0 + 28, b - 17, 14, 18, light);
        self.dome(x0 + 35, b - 17, 7, light);
        self.fill(x0 + 29, b - 16, 12, 17, glow);
        self.dome(x0 + 35, b - 16, 6, glow);
        for dx in [32, 35, 38] {
            self.fill(x0 + dx, b - 3, 1, 3, self.shade(MANTA));
            self.put(x0 + dx, b - 4, flame(x0 + dx, b, self.tick));
        }
        // Long narrow windows, lit.
        for wx in [x0 + 19, x0 + 47] {
            self.fill(wx, b - 20, 3, 9, glow);
        }
        // The tower, its belfry with the bell and its cap with a cross.
        self.fill(x0, b - 46, 14, 47, wall);
        self.fill(x0, b - 46, 2, 47, light);
        self.fill(x0 + 12, b - 46, 2, 47, shade);
        self.fill(x0 - 1, b - 47, 16, 2, light);
        self.fill(x0 + 4, b - 42, 6, 10, glow.scale(0.7));
        self.dome(x0 + 7, b - 42, 3, glow.scale(0.7));
        self.fill(x0 + 6, b - 40, 3, 3, self.shade(DORADO));
        self.fill(x0 + 7, b - 37, 1, 2, self.shade(DORADO));
        self.fill(
            x0 + 6,
            b - 22,
            2,
            8,
            if hash(x0, 5, 0xC0DE) & 1 == 0 {
                glow
            } else {
                dark
            },
        );
        for r in 0..10 {
            let half = 7.0 - r as f32 * 0.65;
            let (a, z) = (
                (x0 as f32 + 7.0 - half).round() as i32,
                (x0 as f32 + 7.0 + half).round() as i32,
            );
            self.fill(a, b - 48 - r, z - a, 1, tiles[(r & 1) as usize]);
        }
        self.fill(x0 + 6, b - 64, 2, 7, light);
        self.fill(x0 + 4, b - 61, 6, 2, light);
    }

    /// A stand of mulitas de hoja de maíz of every size: a big one and a
    /// small one on the counter, tiny ones hung from the awning, and the
    /// vendor in her rebozo between them, under a striped awning.
    fn draw_puesto_de_mulitas(&mut self, co: &Corpus, x0: i32) {
        let b = co.base;
        let wood = self.shade(POST);
        self.fill(x0 + 1, b - 22, STAND_W - 2, 13, self.shade(Rgb(92, 66, 48)));
        self.fill(x0, b - 28, 1, 29, wood);
        self.fill(x0 + STAND_W - 1, b - 28, 1, 29, wood);
        // The awning in the colors of the papel, with a scalloped valance.
        for x in x0 - 1..x0 + STAND_W + 1 {
            let c = self.shade(co.papeles[((x - x0 + 3) / 3).rem_euclid(5) as usize]);
            self.fill(x, b - 28, 1, 5, c);
            if (x - x0).rem_euclid(3) < 2 {
                self.put(x, b - 23, c);
            }
        }
        // Tiny mulitas hanging from the valance on their strings.
        let cord = self.shade(Rgb(214, 204, 180));
        for dx in [6, 15, 26, 34] {
            self.fill(x0 + dx + 3, b - 22, 1, 3, cord);
            self.stamp(MULITA_C, x0 + dx, b - 19, false, 1.0, |ch| {
                hoja_de_maiz(ch, co)
            });
        }
        // The vendor.
        let rebozo = co.rebozo;
        self.stamp(VENDEDORA, x0 + 17, b - 22, false, 1.0, |ch| match ch {
            b'h' => Some((Rgb(34, 24, 20), false)),
            b's' => Some((PIEL, false)),
            b'r' => Some((rebozo, false)),
            b'm' => Some((MANTA, false)),
            _ => None,
        });
        // The counter with its cloth, and the mulitas on it.
        let cloth = self.shade(MANTA);
        self.fill(x0 + 1, b - 9, STAND_W - 2, 1, self.shade(WOOD));
        self.fill(x0 + 1, b - 8, STAND_W - 2, 9, cloth);
        self.fill(x0 + 1, b - 5, STAND_W - 2, 2, self.shade(co.rebozo));
        self.stamp(MULITA_G, x0 + 2, b - 21, false, 1.0, |ch| {
            hoja_de_maiz(ch, co)
        });
        self.stamp(MULITA_M, x0 + 26, b - 17, false, 1.0, |ch| {
            hoja_de_maiz(ch, co)
        });
        for dx in [8, 30] {
            self.put(x0 + dx, b - 22, self.shade(Rgb(40, 30, 24)));
            let bulb = self.candlelit(Rgb(255, 226, 140));
            self.put(x0 + dx, b - 21, bulb);
        }
    }

    /// The strands of papel picado between the poles, and between the tower
    /// of the church and the first pole, swaying a little in the wind.
    fn draw_papel_picado(&mut self, co: &Corpus) {
        let b = co.base;
        let mast = self.shade(POST);
        for &x in &co.poles {
            self.fill(x, b - 52, 1, 54, mast);
        }
        let mut spans: Vec<(i32, i32)> = co.poles.windows(2).map(|p| (p[0], p[1])).collect();
        // From the tower of the church to the first pole.
        let tower = if co.poles[0] > co.church {
            co.church + 12
        } else {
            co.church + 2
        };
        spans.push((tower, co.poles[0]));
        let cord = self.shade(Rgb(214, 204, 180));
        let tick = self.tick as f32;
        for j in 0..2 {
            for (k, &(p, q)) in spans.iter().enumerate() {
                let (xa, xb) = (p.min(q), p.max(q));
                for x in xa..=xb {
                    self.put(x, co.strand_y(j, xa, xb, x), cord);
                }
                let (mut x, mut i) = (xa + 3, 0);
                while x + 5 < xb {
                    let y = co.strand_y(j, xa, xb, x + 2) + 1;
                    let c = self.shade(co.papeles[(i + 2 * j as usize + k) % 5]);
                    let sway =
                        ((tick * 0.07 + x as f32 * 0.21 + j as f32).sin() * 1.3).round() as i32;
                    for (ry, row) in PAPEL.iter().enumerate() {
                        for (rx, ch) in row.bytes().enumerate() {
                            if ch == b'#' {
                                let dx = if ry >= 2 { sway } else { 0 };
                                self.put(x + rx as i32 + dx, y + ry as i32, c);
                            }
                        }
                    }
                    x += 6;
                    i += 1;
                }
            }
        }
    }

    /// The children in manta and rebozo and their mulitas, walking to the
    /// church door; each goes in as it gets there.
    pub(super) fn draw_romeria(&mut self, co: &Corpus) {
        let feet = self.ground - 3;
        let flip = co.line.way < 0.0;
        for (i, wk) in co.walkers.iter().enumerate() {
            let Some(x) = co.line.at(wk.off) else {
                continue;
            };
            // It fades into the doorway.
            let a = ((co.line.door - x).abs() / 8.0).clamp(0.0, 1.0);
            let step = ((self.tick / 6 + i as u32) % 2) as usize;
            let (art, rows): (Sprite, i32) = match wk.kid {
                Kid::Nino => (NINO[step], 9),
                Kid::Nina => (NINA[step], 9),
                Kid::Mulita => (MULITA[step], 8),
            };
            let n = art[0].len() as i32;
            let rebozo = co.rebozo;
            let x0 = x.round() as i32 - n / 2;
            self.stamp(art, x0, feet - rows + 1, flip, a, |ch| match ch {
                b'h' => Some((Rgb(34, 24, 20), false)),
                b's' => Some((PIEL, false)),
                b'm' => Some((MANTA, false)),
                b'r' => Some((rebozo, false)),
                b'w' => Some((HUACAL, false)),
                b'p' => Some((co.papeles[0], false)),
                b'y' => Some((co.papeles[1], false)),
                b'v' => Some((co.papeles[2], false)),
                b'o' => Some((co.papeles[3], false)),
                b'g' => Some((co.papeles[4], false)),
                b'l' => Some((Rgb(74, 48, 32), false)),
                b'H' => Some((HOJA, false)),
                b'T' => Some((TOSTADA, false)),
                _ => None,
            });
        }
    }

    /// The glow of the church's windows and door by night, of the candles
    /// in it, and of the bombillas of the stand, straight onto the screen.
    pub(super) fn draw_corpus_lights(&mut self, co: &Corpus, v: View, night: f32) {
        let k = ((night - 0.2) / 0.5).clamp(0.0, 1.0);
        if k <= 0.0 {
            return;
        }
        let (b, x0) = (co.base, co.church);
        let tick = self.tick as f32;
        let warm = Rgb(255, 214, 120);
        let r = 7.0 * v.z.max(0.7);
        for (i, (dx, dy)) in [(35, -8), (20, -15), (48, -15), (7, -36), (35, -29)]
            .into_iter()
            .enumerate()
        {
            let (x, y) = v.to_screen((x0 + dx) as f32 + 0.5, (b + dy) as f32);
            if self.visible(x, y, r.ceil() as i32) {
                let flicker = 0.9 + 0.1 * (tick * 0.5 + i as f32 * 2.1).sin();
                self.glow(x, y, r, warm, 0.3 * k * flicker);
            }
        }
        if let Some(sx) = co.stand {
            for dx in [8, 30] {
                let (x, y) = v.to_screen((sx + dx) as f32 + 0.5, (b - 20) as f32);
                let r = 2.5 + 2.0 * v.z;
                if self.visible(x, y, r.ceil() as i32) {
                    self.glow(x, y, r, warm, 0.5 * k);
                }
            }
        }
    }
}

/// The colors of the mulitas of the stand: `H` the hoja de maíz and `T` its
/// toasted folds, `m` and `r` the manta and the rebozo, `w` the wood of the
/// little huacal on their backs and `p`, `y`, `v`, `o` and `g` its flowers.
fn hoja_de_maiz(ch: u8, co: &Corpus) -> Option<(Rgb, bool)> {
    Some(match ch {
        b'H' => (HOJA, false),
        b'T' => (TOSTADA, false),
        b'm' => (MANTA, false),
        b'r' => (co.rebozo, false),
        b'w' => (HUACAL, false),
        b'p' => (co.papeles[0], false),
        b'y' => (co.papeles[1], false),
        b'v' => (co.papeles[2], false),
        b'o' => (co.papeles[3], false),
        b'g' => (co.papeles[4], false),
        _ => return None,
    })
}
