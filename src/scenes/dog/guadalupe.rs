//! The Virgen de Guadalupe in the meadow: the Basílica at the end of the
//! street, with roses at the foot of its walls, and the pilgrims on their
//! way to it, on foot and on bicycles, with estandartes, framed images on
//! their backs and roses in their hands. Her image is only ever what pilgrims
//! carry: still and upright on an estandarte or a cuadro, never mirrored,
//! never moved about by itself.

use super::*;

/// Seeds the Virgen's day's own dice.
const GUADALUPE_SEED: u64 = 0x0047_5541_4441_4C55;

/// Rosas de Castilla in two pinks, and the leaves of the roses.
pub(super) const ROSA_CASTILLA: Rgb = Rgb(216, 27, 96);
pub(super) const ROSA_CLARA: Rgb = Rgb(240, 98, 146);
pub(super) const HOJA_ROSAL: Rgb = Rgb(46, 125, 50);
/// Her mantle verde azulado with its stars of gold, her rose gown, her rays.
pub(super) const MANTO: Rgb = Rgb(0, 121, 107);
pub(super) const ORO_MANTO: Rgb = Rgb(242, 193, 78);
pub(super) const VESTIDO: Rgb = Rgb(229, 115, 115);
/// The cantera of the Basílica, and its roofs gone copper-green.
pub(super) const CANTERA: Rgb = Rgb(191, 169, 138);
pub(super) const COBRE: Rgb = Rgb(79, 143, 122);

/// What the pilgrims carry, facing right, one pixel per char: `P` and `R`
/// the roses, `G` their leaves, `h` hair, `s` face, `c` clothes, `b` the
/// bicycle, `l` a leg; on the estandarte and the cuadro `q` the stars and
/// the frame, `u` her mantle, `e` her gown, `s` her face and `y` the gold of
/// the border, and `w` wood.
const BICI: [Sprite; 2] = [
    &[
        ".........hh....",
        ".........ss....",
        "........ccc....",
        "........cccRPR.",
        "..bbb...cl.bbb.",
        ".b...b.cbl.b..b",
        ".b...bbbbbbb..b",
        ".b...b.bl..b..b",
        "..bbb..l..bbb..",
    ],
    &[
        ".........hh....",
        ".........ss....",
        "........ccc....",
        "........cccRPR.",
        "..bbb...cc.bbb.",
        ".b...b.cbl.b..b",
        ".b...bbbblb.b.b",
        ".b...b.b...b..b",
        "..bbb.....bbb..",
    ],
];
pub(super) const ESTANDARTE: Sprite = &[
    "yyyyyyy", "yqqqqqy", "yquuuqy", "yuuseuy", "yuueeuy", "yquueqy", "yqqqqqy", ".y.y.y.",
    "...w...",
];
const CUADRO: Sprite = &["qqqqq", "quuuq", "uusuu", "ueeeu", "qeeeq", "qqqqq"];
const ROSAL: Sprite = &[
    "..R.P..", ".PRGRP.", "RGRPGR.", ".GGRGG.", "..GGG..", "...b...",
];

/// What a pilgrim carries.
#[derive(Clone, Copy, PartialEq)]
enum Carga {
    /// An estandarte with her image, high over the head.
    Estandarte,
    /// A framed image on the back.
    Cuadro,
    /// A ramo of roses in the hand.
    Rosas,
    /// Nothing but the walk.
    Nada,
    /// On a bicycle, roses round the handlebars.
    Bici,
}

pub(super) struct Pilgrim {
    /// How far behind the head of the line it walks.
    pub(super) off: f32,
    carga: Carga,
    hair: Rgb,
    cloth: Rgb,
}

/// The Virgen de Guadalupe in the meadow, laid out for one world.
pub(super) struct Guadalupe {
    base: i32,
    /// The old Basílica's left edge, if there is room for it, and the new
    /// one's left edge and width.
    pub(super) antigua: Option<i32>,
    pub(super) nueva: i32,
    pub(super) width: i32,
    pub(super) line: Line,
    pub(super) pilgrims: Vec<Pilgrim>,
    /// Bushes of rosas de Castilla at the foot of the walls.
    pub(super) rosales: Vec<i32>,
    /// Petals lying along the street.
    petals: Vec<(i32, i32, bool)>,
    rosa: [Rgb; 2],
    hoja: Rgb,
}

impl Guadalupe {
    /// The holiday for this world, if it is on.
    pub(super) fn of(ctx: &Ctx, m: &Meadow) -> Option<Guadalupe> {
        ctx.fiesta
            .guadalupe()
            .then(|| Guadalupe::new(ctx, m.w, m.fence))
    }

    fn new(ctx: &Ctx, w: i32, fence: i32) -> Guadalupe {
        let mut rng = Rng::with_seed(ctx.seed ^ GUADALUPE_SEED);
        let base = fence + 4;
        // The Basílicas stand at one end of the street, the new one nearer
        // the middle, and the pilgrims come in from the far side.
        let left = rng.bool();
        let old = w >= 140;
        let width = if old { 72 } else { (w - 8).clamp(20, 72) };
        let total = width + if old { 42 } else { 0 };
        let x0 = if left { 6 } else { w - total - 6 };
        let (antigua, nueva) = match (old, left) {
            (true, true) => (Some(x0), x0 + 42),
            (true, false) => (Some(x0 + width + 4), x0),
            (false, _) => (None, x0.max(4)),
        };
        let door = nueva + width / 2;
        let way = if left { -1.0 } else { 1.0 };
        let [rosa, claro, hoja] = themed(ctx, [ROSA_CASTILLA, ROSA_CLARA, HOJA_ROSAL], 1);
        let cloths = [
            Rgb(236, 226, 200),
            Rgb(190, 60, 50),
            Rgb(70, 120, 170),
            Rgb(210, 170, 60),
            Rgb(60, 130, 90),
        ];
        let mut pilgrims = Vec::new();
        let mut off = 0.0;
        for i in 0..9 {
            let carga = match i {
                0 => Carga::Estandarte,
                3 | 7 => Carga::Bici,
                2 => Carga::Cuadro,
                5 => Carga::Rosas,
                8 => Carga::Estandarte,
                _ => *rng.pick(&[Carga::Nada, Carga::Rosas, Carga::Cuadro]),
            };
            pilgrims.push(Pilgrim {
                off,
                carga,
                hair: *rng.pick(&CABELLOS),
                cloth: *rng.pick(&cloths),
            });
            off += if carga == Carga::Bici { 19.0 } else { 12.0 } + rng.rangef(0.0, 4.0);
        }
        let mut line = Line::new(w as f32, door as f32, way, off, 0.0);
        line.walk = rng.rangef(0.35, 0.75) * line.far();
        let rosales = (0..w.max(1) / 26 + 2).map(|_| rng.range(0, w)).collect();
        let petals = (0..w / 3)
            .map(|_| (rng.range(0, w.max(1)), rng.range(1, 6), rng.bool()))
            .collect();
        Guadalupe {
            base,
            antigua,
            nueva,
            width,
            line,
            pilgrims,
            rosales,
            petals,
            rosa: [rosa, claro],
            hoja,
        }
    }

    /// The rows of the meadow the street covers, which the grass leaves clear.
    pub(super) fn cleared(&self, fence: i32) -> (i32, i32) {
        (fence + 1, self.base + 5)
    }

    /// One step: the pilgrims walk on.
    pub(super) fn step(&mut self) {
        self.line.step(0.2);
    }
}

impl Meadow {
    /// The street and the Basílicas, with the roses at their foot.
    pub(super) fn draw_basilica(&mut self, gu: &Guadalupe) {
        let (w, b) = (self.w, gu.base);
        // The plaza: a banqueta, and paving of cantera under the pilgrims.
        self.fill(0, b + 1, w, 2, self.shade(Rgb(190, 176, 154)));
        let stones =
            [Rgb(168, 150, 124), Rgb(154, 136, 112), Rgb(178, 160, 132)].map(|c| self.shade(c));
        let mortar = self.shade(Rgb(112, 98, 82));
        for y in b + 3..=self.ground + 5 {
            let r = (y - b - 3) / 3;
            for x in 0..w {
                let o = x + 3 * (r & 1);
                let c = if (y - b - 3) % 3 == 2 || o % 6 == 5 {
                    mortar
                } else {
                    stones[(hash(o / 6, r, 0x6A00) % 3) as usize]
                };
                self.put(x, y, c);
            }
        }
        // Petals strewn along the way.
        for &(x, dy, light) in &gu.petals {
            let c = self.shade(gu.rosa[light as usize]);
            self.put(x, b + 2 + dy, c);
        }
        if let Some(ox) = gu.antigua {
            self.draw_antigua(ox, b);
        }
        self.draw_nueva(gu.nueva, gu.width, b);
        for &x in &gu.rosales {
            let (rosa, claro, hoja) = (gu.rosa[0], gu.rosa[1], gu.hoja);
            self.stamp(ROSAL, x, b - 4, false, 1.0, |ch| match ch {
                b'R' => Some((rosa, false)),
                b'P' => Some((claro, false)),
                b'G' => Some((hoja, false)),
                b'b' => Some((Rgb(104, 72, 46), false)),
                _ => None,
            });
        }
    }

    /// The new Basílica, round, its great roof a tent of copper-green with
    /// its ribs, a lantern and a cross on top, over a ring of cantera with
    /// tall lit windows and three doors: `d` wide, its left edge at `x0`.
    fn draw_nueva(&mut self, x0: i32, d: i32, b: i32) {
        let wall = self.shade(CANTERA.scale(0.95));
        let light = self.shade(CANTERA.lerp(Rgb::WHITE, 0.4));
        let glow = self.candlelit(Rgb(255, 222, 150));
        let dark = self.shade(Rgb(70, 56, 48));
        let (cx, wh, rh) = (x0 + d / 2, 10, d * 36 / 100);
        let top = b - wh;
        for r in 0..rh {
            let t = r as f32 / rh as f32;
            let hw = ((d as f32 / 2.0 + 2.0) * (1.0 - t).powf(1.5)).max(1.5);
            let shade = 0.7 + 0.5 * (1.0 - t);
            let (a, z) = (
                (cx as f32 - hw).round() as i32,
                (cx as f32 + hw).round() as i32,
            );
            for x in a..z {
                let rib = (x - cx).rem_euclid(7) == 0;
                let c = self.shade(COBRE.scale(shade * if rib { 0.78 } else { 1.0 }));
                self.put(x, top - 1 - r, c);
            }
        }
        self.fill(x0, top, d, wh + 1, wall);
        self.fill(x0 - 2, top, d + 4, 2, light);
        for x in (x0 + 4..x0 + d - 4).step_by(6) {
            let on = !hash(x, 3, 0x6A01).is_multiple_of(5);
            self.fill(x, top + 3, 2, 6, if on { glow } else { dark });
        }
        for dx in [-14, 0, 14] {
            if d < 50 && dx != 0 {
                continue;
            }
            let x = cx + dx - 4;
            self.fill(x - 1, b - 10, 10, 11, light);
            self.fill(x, b - 9, 8, 10, glow);
            self.dome(x + 4, b - 9, 4, glow);
        }
        let apex = top - rh;
        self.fill(cx - 2, apex - 5, 4, 5, glow);
        self.fill(cx - 1, apex - 14, 2, 9, light);
        self.fill(cx - 4, apex - 11, 8, 2, light);
    }

    /// The old Basílica, baroque: a facade of cantera between two towers with
    /// domed caps, a pediment over it and the great dome rising behind, the
    /// door open, a niche lit over it: 38 wide, its left edge at `x0`.
    fn draw_antigua(&mut self, x0: i32, b: i32) {
        let wall = self.shade(CANTERA.scale(0.88));
        let light = self.shade(CANTERA.lerp(Rgb::WHITE, 0.4));
        let shade = self.shade(CANTERA.scale(0.7));
        let cap = self.shade(COBRE.lerp(Rgb(214, 170, 90), 0.1));
        let glow = self.candlelit(Rgb(255, 222, 150));
        let dark = self.shade(Rgb(70, 56, 48));
        let cx = x0 + 19;
        // The dome behind, with its lantern and cross.
        self.fill(cx - 7, b - 32, 14, 5, shade);
        self.dome(cx, b - 32, 9, cap);
        self.fill(cx - 1, b - 45, 2, 4, glow);
        self.fill(cx, b - 51, 1, 6, light);
        self.fill(cx - 2, b - 49, 5, 1, light);
        // The facade, its cornice and pediment.
        self.fill(x0 + 7, b - 24, 24, 25, wall);
        self.gable(cx, b - 25, 7, 12.0, light);
        self.gable(cx, b - 25, 6, 10.0, wall);
        self.fill(x0 + 6, b - 25, 26, 1, light);
        // The door, the niche over it, a window on either side.
        self.fill(x0 + 14, b - 15, 10, 16, light);
        self.dome(cx, b - 15, 5, light);
        self.fill(x0 + 15, b - 14, 8, 15, glow);
        self.dome(cx, b - 14, 4, glow);
        self.fill(cx - 2, b - 23, 4, 5, glow);
        self.dome(cx, b - 23, 2, glow);
        for wx in [x0 + 9, x0 + 27] {
            let on = hash(wx, 4, 0x6A02) & 1 == 0;
            self.fill(wx, b - 18, 2, 8, if on { glow } else { dark });
        }
        // The towers, each with its belfry and a cap with a cross.
        for tx in [x0, x0 + 31] {
            self.fill(tx, b - 36, 7, 37, wall);
            self.fill(tx, b - 36, 1, 37, light);
            self.fill(tx + 6, b - 36, 1, 37, shade);
            self.fill(tx - 1, b - 37, 9, 1, light);
            self.fill(tx + 2, b - 32, 3, 7, glow.scale(0.7));
            self.fill(tx + 3, b - 20, 1, 8, glow);
            self.dome(tx + 3, b - 37, 4, cap);
            self.fill(tx + 3, b - 46, 1, 5, light);
            self.fill(tx + 1, b - 44, 5, 1, light);
        }
    }

    /// The pilgrims, on foot and on bicycles, on their way to the Basílica:
    /// each goes in at its door as it gets there.
    pub(super) fn draw_peregrinos(&mut self, gu: &Guadalupe) {
        let feet = self.ground - 3;
        let way = gu.line.way as i32;
        let flip = way < 0;
        for (i, p) in gu.pilgrims.iter().enumerate() {
            let Some(xf) = gu.line.at(p.off) else {
                continue;
            };
            let x = xf.round() as i32;
            let a = ((gu.line.door - xf).abs() / 8.0).clamp(0.0, 1.0);
            let step = ((self.tick / 6 + i as u32) % 2) as usize;
            let (hair, cloth) = (p.hair, p.cloth);
            let (rosa, claro, hoja) = (gu.rosa[0], gu.rosa[1], gu.hoja);
            if p.carga == Carga::Bici {
                self.stamp(BICI[step], x - 7, feet - 8, flip, a, |ch| match ch {
                    b'h' => Some((hair, false)),
                    b's' => Some((PIEL, false)),
                    b'c' => Some((cloth, false)),
                    b'l' => Some((Rgb(60, 44, 36), false)),
                    b'b' => Some((Rgb(60, 60, 68), false)),
                    b'R' => Some((rosa, false)),
                    b'P' => Some((claro, false)),
                    _ => None,
                });
                continue;
            }
            self.stamp(PEREGRINO[step], x - 2, feet - 9, flip, a, |ch| {
                Some(match ch {
                    b'h' => (hair, false),
                    b's' => (PIEL, false),
                    b'c' => (cloth, false),
                    b'd' => (Rgb(60, 44, 36), false),
                    _ => return None,
                })
            });
            let front = x + 3 * way;
            match p.carga {
                Carga::Estandarte => {
                    // The pole in the hand and the estandarte high above the
                    // head: her image, upright, never turned round.
                    let pole = self.shade(CANA);
                    self.fill(front, feet - 11, 1, 8, pole);
                    self.estandarte(front - 3, feet - 20, a);
                }
                Carga::Cuadro => {
                    // A framed image on the back.
                    self.estandarte_chico(x - way * 5 - 2, feet - 8, a);
                }
                Carga::Rosas => {
                    self.fill(front - 1, feet - 7, 3, 3, self.shade(rosa));
                    self.put(front, feet - 6, self.shade(claro));
                    self.fill(front, feet - 4, 1, 2, self.shade(hoja));
                }
                _ => {}
            }
        }
    }

    /// An estandarte, 7 by 9: her image on a banner with a border of gold.
    fn estandarte(&mut self, x: i32, y: i32, a: f32) {
        self.stamp(ESTANDARTE, x, y, false, a, image_colors);
    }

    /// A framed image, 5 by 6.
    fn estandarte_chico(&mut self, x: i32, y: i32, a: f32) {
        self.stamp(CUADRO, x, y, false, a, image_colors);
    }

    /// The glow of the Basílicas' windows and doors by night, straight onto
    /// the screen.
    pub(super) fn draw_guadalupe_lights(&mut self, gu: &Guadalupe, v: View, night: f32) {
        let k = ((night - 0.2) / 0.5).clamp(0.0, 1.0);
        if k <= 0.0 {
            return;
        }
        let b = gu.base;
        let warm = Rgb(255, 214, 120);
        let r = 8.0 * v.z.max(0.7);
        let mut lights = vec![];
        if let Some(ox) = gu.antigua {
            lights.extend([(ox + 19, b - 8), (ox + 10, b - 14), (ox + 28, b - 14)]);
        }
        for dx in [-14, 0, 14] {
            if gu.width >= 50 || dx == 0 {
                lights.push((gu.nueva + gu.width / 2 + dx, b - 5));
            }
        }
        for (x, y) in lights {
            let (sx, sy) = v.to_screen(x as f32 + 0.5, y as f32);
            if self.visible(sx, sy, r.ceil() as i32) {
                self.glow(sx, sy, r, warm, 0.3 * k);
            }
        }
    }
}

/// The colors of her image on an estandarte or a cuadro: the border of gold,
/// her mantle verde azulado with its stars, her rose gown and her face.
pub(super) fn image_colors(ch: u8) -> Option<(Rgb, bool)> {
    Some(match ch {
        b'y' => (Rgb(242, 182, 50), true),
        b'q' => (ORO_MANTO, true),
        b'u' => (MANTO, false),
        b'e' => (VESTIDO, false),
        b's' => (Rgb(170, 112, 78), false),
        b'w' => (Rgb(124, 78, 44), false),
        _ => return None,
    })
}
