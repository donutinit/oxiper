//! Candelaria: a tamalera steaming on the fire.
//!
//! A tall aluminum tamalera, soot-black at its foot, stands on its stones
//! over aafire's flames with its lid slid off to one side, and the tamales in
//! their husks peek out of its mouth, one of them already opened (masa, a dab
//! of salsa). Steam pours out of it. Beside it a jarro de barro of atole
//! steams, with a smaller one next to it; on the other side a plate of
//! tamales waits on the ground among loose husks of maize; and velitas, the
//! candles that are blessed on this day, burn here and there.

use super::{Fuel, Mouth, Piece, Rect, Steam, cool, hash, profile, stone};
use crate::color::{Color, rgb};
use crate::render::Pixels;
use crate::scene::Ctx;

/// The tamalera's body from its mouth to its foot: half-width as a share of
/// the widest, at each share of the way down.
const BODY: [(f32, f32); 5] = [
    (0.0, 1.0),
    (0.04, 1.04),
    (0.1, 1.0),
    (0.9, 0.97),
    (1.0, 0.92),
];

/// A jarro's body: fat in the middle, narrow at the mouth and the foot.
const JARRO: [(f32, f32); 5] = [
    (0.0, 0.8),
    (0.1, 0.76),
    (0.5, 1.0),
    (0.85, 0.86),
    (1.0, 0.72),
];

#[derive(Clone, Copy)]
struct Tamalera {
    cx: i32,
    /// Pixel row of the mouth's center, and of the foot.
    cy: i32,
    bottom: i32,
    /// Half its width, and the mouth's half-height.
    hw: i32,
    ry: i32,
    /// How tall the stones it stands on are.
    stone: i32,
    ground: i32,
}

/// A tamal standing in the tamalera, its husk tip showing.
#[derive(Clone, Copy)]
struct Tip {
    x: i32,
    /// How far above the mouth's middle its top is.
    top: i32,
    /// 0 in its husk, 1 opened with salsa roja, 2 with salsa verde.
    kind: u8,
}

/// A jarro of atole.
#[derive(Clone, Copy)]
struct Jarro {
    x: i32,
    feet: i32,
    h: i32,
    hw: i32,
}

#[derive(Clone, Copy)]
struct Plate {
    x: i32,
    ground: i32,
    rx: i32,
    /// Rows of tamales stacked on it, the widest first.
    rows: i32,
}

#[derive(Clone, Copy)]
struct Vela {
    x: i32,
    feet: i32,
    h: i32,
    phase: f32,
}

/// Colors of the holiday, cached in `recolor`.
struct Pal {
    /// The tamalera's aluminum: shaded, in between, lit.
    alu: [Color; 3],
    soot: Color,
    inside: Color,
    /// Hojas de maíz: shaded, in between, lit.
    husk: [Color; 3],
    masa: Color,
    salsa: [Color; 2],
    tie: Color,
    clay: [Color; 3],
    cenefa: Color,
    atole: [Color; 2],
    stone: [Color; 3],
    wax: Color,
    flame: Color,
    flame_edge: Color,
    ember: Color,
    steam: Color,
}

pub(super) struct Tamales {
    level: f32,
    seed: u64,
    pot: Option<Tamalera>,
    tips: Vec<Tip>,
    jarros: Vec<Jarro>,
    plate: Option<Plate>,
    velas: Vec<Vela>,
    /// Loose husks on the ground: column, pixel row, length.
    husks: Vec<(i32, i32, i32)>,
    steam: Steam,
    t: f32,
    pal: Pal,
}

impl Tamales {
    pub fn new(seed: u64) -> Tamales {
        let none = Color::Reset;
        Tamales {
            level: 0.0,
            seed,
            pot: None,
            tips: Vec::new(),
            jarros: Vec::new(),
            plate: None,
            velas: Vec::new(),
            husks: Vec::new(),
            steam: Steam::new(seed ^ 0x57ea),
            t: 0.0,
            pal: Pal {
                alu: [none; 3],
                soot: none,
                inside: none,
                husk: [none; 3],
                masa: none,
                salsa: [none; 2],
                tie: none,
                clay: [none; 3],
                cenefa: none,
                atole: [none; 2],
                stone: [none; 3],
                wax: none,
                flame: none,
                flame_edge: none,
                ember: none,
                steam: none,
            },
        }
    }
}

impl Piece for Tamales {
    fn recolor(&mut self, ctx: &Ctx) {
        let themed = !ctx.theme.native;
        let fg = ctx.theme.fg;
        let g = &ctx.theme.grad;
        let or = |native: u32, theme: Color| if themed { theme } else { rgb(native) };
        let p = &mut self.pal;
        p.alu = [
            or(0x6a6e7a, ctx.fade(fg, 0.55)),
            or(0xaeb2be, ctx.fade(fg, 0.25)),
            or(0xe6e8f0, fg),
        ];
        p.soot = or(0x1c1a1c, ctx.fade(fg, 0.9));
        p.inside = or(0x2a2630, ctx.fade(fg, 0.85));
        p.husk = [
            or(0xa8904e, g.at(0.45)),
            or(0xd9c48a, g.at(0.65)),
            or(0xefe0aa, g.at(0.8)),
        ];
        p.masa = or(0xf2e2b0, ctx.pick(2, Color::Reset));
        p.salsa = [
            or(0xc8281e, ctx.pick(6, Color::Reset)),
            or(0x4f8a2a, ctx.pick(3, Color::Reset)),
        ];
        p.tie = or(0x7a5a2a, g.at(0.3));
        p.clay = [
            or(0xd88a54, g.at(0.7)),
            or(0xa8502c, g.at(0.5)),
            or(0x5c2a1a, g.at(0.25)),
        ];
        p.cenefa = or(0xf0e4c8, fg);
        p.atole = [or(0xc89a6a, g.at(0.6)), or(0xe6c898, g.at(0.8))];
        p.stone = [
            or(0x9a8e84, ctx.fade(fg, 0.45)),
            or(0x6e625a, ctx.fade(fg, 0.6)),
            or(0x463d38, ctx.fade(fg, 0.78)),
        ];
        // Candles and their flames keep their own hues under any theme.
        p.wax = rgb(0xf6ecd2);
        p.flame = rgb(0xffe8a0);
        p.flame_edge = rgb(0xff9628);
        p.ember = rgb(0xff7a1a);
        p.steam = or(0xe4e2ee, fg);
    }

    fn layout(&mut self, w: usize, ph: usize, level: f32) {
        self.pot = None;
        self.tips.clear();
        self.jarros.clear();
        self.plate = None;
        self.velas.clear();
        self.husks.clear();
        self.steam.reset(0.0);
        let (w, ph) = (w as i32, ph as i32);
        self.level = level;
        if ph < 20 || w < 24 {
            return;
        }
        let mut rng = fastrand::Rng::with_seed(self.seed ^ 0x5eed);
        // The tamalera: tall, as high as two fifths of the screen at most.
        let h = (ph * 4 / 10).clamp(11, 56);
        let hw = (h * 33 / 100).max(4).min((w / 2 - 4) * 10 / 46);
        let ry = (hw * 2 / 7).clamp(2, 6);
        let stone = (h / 8).max(2);
        let ground = ph - 1;
        let bottom = ground - stone;
        let pot = Tamalera {
            cx: w / 2,
            cy: bottom - h + 1 + ry,
            bottom,
            hw,
            ry,
            stone,
            ground,
        };
        self.pot = Some(pot);

        // Tamales standing in it: a row at the back, peeking over the rim,
        // and a lower row in front, one of them opened.
        let tw = (hw / 3).clamp(2, 6);
        let n = ((2 * hw - 2) / (tw + 1)).clamp(2, 6);
        let opened = rng.i32(0..n);
        for i in 0..n {
            let fx = (i as f32 + 0.5) / n as f32 * 2.0 - 1.0;
            let x = pot.cx + (fx * (hw - tw / 2 - 1) as f32).round() as i32;
            let back = ry + 2 + rng.i32(0..=2 + hw / 5);
            self.tips.push(Tip {
                x,
                top: if i % 2 == 0 {
                    back
                } else {
                    ry / 2 + rng.i32(0..=1)
                },
                kind: if i == opened { 1 + rng.u8(0..2) } else { 0 },
            });
        }
        // Steam pours out of the open side, and a little under the lid.
        self.steam.reset(3.0 + ph as f32 * 0.085);
        self.steam.mouths.push(Mouth {
            x: (pot.cx - hw / 3) as f32,
            y: (pot.cy - ry) as f32,
            half: hw as f32 * 0.5,
            rate: hw as f32 * 2.6,
            acc: 0.0,
        });
        self.steam.mouths.push(Mouth {
            x: (pot.cx + hw * 3 / 5) as f32,
            y: (pot.cy - ry - hw / 4) as f32,
            half: hw as f32 * 0.25,
            rate: hw as f32 * 0.8,
            acc: 0.0,
        });

        // A jarro of atole beside the fire, a smaller one next to it.
        let jh = (h * 34 / 100).max(6);
        let jhw = (jh * 4 / 10).max(3);
        let jx = pot.cx + hw * 23 / 10 + jhw;
        if jx + jhw + 3 < w {
            self.jarros.push(Jarro {
                x: jx,
                feet: ground,
                h: jh,
                hw: jhw,
            });
            let (sh, shw) = ((jh * 7 / 10).max(5), (jhw * 7 / 10).max(2));
            let sx = jx + jhw + shw + 2;
            if sx + shw + 2 < w {
                self.jarros.push(Jarro {
                    x: sx,
                    feet: ground,
                    h: sh,
                    hw: shw,
                });
            }
        }
        for j in &self.jarros {
            self.steam.mouths.push(Mouth {
                x: j.x as f32,
                y: (j.feet - j.h) as f32,
                half: j.hw as f32 * 0.4,
                rate: 1.5 + j.hw as f32 * 0.4,
                acc: 0.0,
            });
        }
        self.steam.reserve();

        // A plate of tamales on the other side, among loose husks.
        let prx = (hw * 9 / 10).max(6);
        let px = pot.cx - hw * 23 / 10 - prx;
        if px - prx >= 1 {
            self.plate = Some(Plate {
                x: px,
                ground,
                rx: prx,
                rows: if ph >= 90 { 3 } else { 2 },
            });
            for _ in 0..4 + hw / 3 {
                let x = px + rng.i32(-prx - 2..=prx + 2);
                if (1..w - 4).contains(&x) {
                    self.husks
                        .push((x, ground - rng.i32(0..=1), 3 + rng.i32(0..=2)));
                }
            }
        }
        // Velitas, the candles blessed on the day, further out on both sides.
        let vh = (ph / 12).clamp(4, 10);
        let edge_l = self.plate.map_or(pot.cx - hw * 3, |p| p.x - p.rx);
        let edge_r = self.jarros.last().map_or(pot.cx + hw * 3, |j| j.x + j.hw);
        for (side, edge) in [(-1, edge_l), (1, edge_r)] {
            for k in 0..2 + (ph >= 60) as i32 {
                let x = edge + side * (4 + k * (3 + rng.i32(0..=3)));
                if (1..w - 1).contains(&x) {
                    self.velas.push(Vela {
                        x,
                        feet: ground,
                        h: vh - rng.i32(0..=2).min(vh - 3),
                        phase: rng.f32() * std::f32::consts::TAU,
                    });
                }
            }
        }
    }

    fn fuel(&self, sw: usize) -> Option<Fuel> {
        let o = self.pot?;
        let reach = o.hw * 21 / 10;
        Some(Fuel::span(
            o.cx - reach,
            o.cx + reach,
            sw,
            0.8 + 0.2 * self.level,
        ))
    }

    fn reach(&self) -> f32 {
        if self.pot.is_some() { 0.95 } else { 1.0 }
    }

    fn update(&mut self, dt: f32, wind: f32) {
        self.t += dt;
        if self.pot.is_some() {
            self.steam.update(dt, wind);
        }
    }

    fn block(&self, heat: &mut [u16], stride: usize) {
        let Some(o) = &self.pot else { return };
        let rows = (o.bottom - o.cy).max(1);
        for y in o.cy - o.ry..=o.bottom {
            let t = ((y - o.cy).max(0)) as f32 / rows as f32;
            let half = (o.hw as f32 * profile(&BODY, t)).round() as i32 + 1;
            cool(
                heat,
                stride,
                y,
                o.cx - half,
                o.cx + half,
                0.55 + 0.3 * t.powf(1.5),
            );
        }
    }

    fn steam(&self) -> Option<(&Steam, Color)> {
        self.pot.map(|_| (&self.steam, self.pal.steam))
    }

    fn draw(&self, _ctx: &Ctx, px: &mut Pixels) {
        if let Some(o) = &self.pot {
            self.draw_pot(px, o);
        }
    }

    fn lick(&self) -> Option<Rect> {
        let o = self.pot?;
        Some(Rect {
            x0: o.cx - o.hw - 1,
            x1: o.cx + o.hw + 1,
            y0: o.bottom - (o.bottom - o.cy) * 55 / 100,
            y1: o.bottom,
        })
    }

    fn draw_front(&self, _ctx: &Ctx, px: &mut Pixels) {
        let Some(o) = &self.pot else { return };
        let p = &self.pal;
        // The stones it stands on, warmed by the fire between them.
        let sw = (o.hw * 11 / 20).max(3);
        for side in [-1, 1] {
            let x0 = o.cx + side * (o.hw * 3 / 5) - sw / 2;
            let at = Rect {
                x0,
                y0: o.ground - o.stone + 1,
                x1: x0 + sw - 1,
                y1: o.ground,
            };
            stone(px, at, -side, &p.stone, p.ember);
        }
        for &(x, y, len) in &self.husks {
            for i in 0..len {
                let col = p.husk[(i + x) as usize % 2];
                px.set(x + i, y - (i == len - 1) as i32, col);
            }
        }
        if let Some(pl) = &self.plate {
            self.draw_plate(px, pl);
        }
        for j in &self.jarros {
            self.draw_jarro(px, j);
        }
        for v in &self.velas {
            self.draw_vela(px, v);
        }
    }

    #[cfg(test)]
    fn counts(&self) -> (usize, usize, usize) {
        (self.tips.len(), self.jarros.len(), self.velas.len())
    }
}

impl Tamales {
    /// The tamalera: aluminum, blackened by the fire, with the tamales in
    /// its mouth and its lid slid aside.
    fn draw_pot(&self, px: &mut Pixels, o: &Tamalera) {
        let p = &self.pal;
        let rows = (o.bottom - o.cy).max(1);
        let breath = 0.9 + 0.1 * (self.t * 5.0).sin();
        for y in o.cy..=o.bottom {
            let t = (y - o.cy) as f32 / rows as f32;
            let half = o.hw as f32 * profile(&BODY, t);
            let hx = half.round() as i32;
            for dx in -hx..=hx {
                let u = (dx as f32 / (half + 0.5)).clamp(-1.0, 1.0);
                // Polished metal: a bright streak on the lit side.
                let n = (1.0 - u * u).sqrt();
                let mut lam = 0.3 + 0.7 * (0.6 * n - 0.4 * u).clamp(0.0, 1.0);
                if (u + 0.45).abs() < 0.1 {
                    lam = (lam + 0.3).min(1.0);
                }
                let mut col = if lam < 0.6 {
                    p.alu[0].lerp(p.alu[1], lam / 0.6)
                } else {
                    p.alu[1].lerp(p.alu[2], (lam - 0.6) / 0.4)
                };
                // The fire's soot, thicker toward the foot and in streaks.
                let streak = hash(dx, y / 3, 5) as f32 / u32::MAX as f32;
                let soot = ((t - 0.35) * 1.6).clamp(0.0, 1.0) * (0.55 + 0.45 * streak);
                col = col.lerp(p.soot, soot.clamp(0.0, 0.92));
                if t < 0.04 {
                    // The rolled rim.
                    col = p.alu[2].scale(0.7 + 0.3 * lam);
                } else if (0.5..0.53).contains(&t) {
                    // The seam where the two halves of the pot are joined.
                    col = col.scale(0.7);
                }
                let glow = (t * t * 0.55 + 0.1 * u.abs()) * breath;
                col = col.lerp(p.ember, glow.clamp(0.0, 0.8));
                if dx.abs() == hx {
                    col = col.scale(0.78);
                }
                px.set(o.cx + dx, y, col);
            }
        }
        // Two handles, ears of black bakelite at the top.
        let hy = o.cy + rows / 7;
        for side in [-1, 1] {
            let x = o.cx + side * (o.hw + 1);
            px.set(x, hy, p.soot);
            px.set(x + side, hy + 1, p.soot);
            px.set(x + side, hy + 2, p.soot);
            px.set(x, hy + 3, p.soot);
        }
        // The mouth: dark inside, the tamales standing in it, the front of
        // the rim over their feet.
        let rx = o.hw as f32;
        let ry = o.ry as f32;
        let tw = (o.hw / 3).clamp(2, 6);
        let half_at = |dy: i32| {
            (rx * (1.0 - (dy as f32 / (ry + 0.5)).powi(2)).max(0.0).sqrt()).round() as i32
        };
        let inside = |dx: i32, dy: i32| {
            (dx as f32 / (rx - 1.3)).powi(2) + (dy as f32 / (ry - 0.6)).powi(2) <= 1.0
        };
        for dy in -o.ry..=o.ry {
            for dx in -half_at(dy)..=half_at(dy) {
                let col = if inside(dx, dy) {
                    p.inside.lerp(p.ember, 0.18 * breath)
                } else if dy < 0 {
                    p.alu[2].scale(0.85)
                } else {
                    p.alu[1]
                };
                px.set(o.cx + dx, o.cy + dy, col);
            }
        }
        for tip in &self.tips {
            self.draw_tip(px, o, tip, tw);
        }
        for dy in 0..=o.ry {
            for dx in -half_at(dy)..=half_at(dy) {
                if !inside(dx, dy) {
                    px.set(o.cx + dx, o.cy + dy, p.alu[1]);
                }
            }
        }
        self.draw_lid(px, o);
    }

    /// A tamal's tip: its husk folded to a point, tied, or opened.
    fn draw_tip(&self, px: &mut Pixels, o: &Tamalera, tip: &Tip, tw: i32) {
        let p = &self.pal;
        let top = o.cy - tip.top;
        let bottom = o.cy + o.ry;
        for y in top..=bottom {
            let r = y - top;
            let w = if r == 0 { 1 } else { tw };
            for i in 0..w {
                let dx = i - w / 2;
                let lit = match dx {
                    ..=-1 => 2,
                    1.. if tw > 2 => 0,
                    _ => 1,
                };
                let mut col = p.husk[lit];
                // The folds of the husk, and the strip that ties it.
                if dx == 0 && r % 2 == 1 && tw > 2 {
                    col = p.husk[0];
                }
                if r == 3 && tip.kind == 0 {
                    col = p.tie;
                }
                if tip.kind > 0 && r < 3 {
                    // Opened: masa, and a dab of salsa on it.
                    col = if r == 0 || (i == w / 2 && r == 1) {
                        p.salsa[tip.kind as usize - 1]
                    } else {
                        p.masa
                    };
                }
                px.set(tip.x + dx, y, col);
            }
        }
    }

    /// The lid, a dome slid off to the right and tilted on the rim.
    fn draw_lid(&self, px: &mut Pixels, o: &Tamalera) {
        let p = &self.pal;
        let (lrx, lry) = (o.hw as f32 * 0.9, (o.hw as f32 * 0.34).max(2.0));
        let (lx, ly) = (
            o.cx as f32 + o.hw as f32 * 0.7,
            o.cy as f32 - o.ry as f32 * 0.4 - 1.0,
        );
        let (sin, cos) = 0.36f32.sin_cos();
        let reach = (lrx + lry + 2.0) as i32;
        for dy in -reach..=reach {
            for dx in -reach..=reach {
                // Into the lid's own frame, tilted to slide down to the right.
                let (fx, fy) = (dx as f32, dy as f32);
                let (u, v) = (fx * cos + fy * sin, -fx * sin + fy * cos);
                let dome = (u / lrx).powi(2) + (v / lry).powi(2);
                let in_dome = v <= 0.0 && dome <= 1.0;
                let in_rim = (0.0..1.6).contains(&v) && u.abs() <= lrx;
                if !in_dome && !in_rim {
                    continue;
                }
                let col = if in_rim {
                    p.alu[0]
                } else {
                    let lam = (0.55 - 0.35 * u / lrx + 0.3 * v / lry).clamp(0.0, 1.0);
                    let edge = if dome > 0.82 { 0.8 } else { 1.0 };
                    p.alu[0].lerp(p.alu[2], lam).scale(edge)
                };
                px.set((lx + fx).round() as i32, (ly + fy).round() as i32, col);
            }
        }
        // Its knob.
        let (kx, ky) = (-lry * sin * 1.4, -lry * cos * 1.4 - 1.0);
        let (kx, ky) = ((lx + kx).round() as i32, (ly + ky).round() as i32);
        px.set(kx, ky, p.soot);
        px.set(kx + 1, ky, p.soot);
    }

    /// A jarro de barro with atole in it, painted with bands, its handle on
    /// the right.
    fn draw_jarro(&self, px: &mut Pixels, j: &Jarro) {
        let p = &self.pal;
        let top = j.feet + 1 - j.h;
        let ry = (j.hw / 3).clamp(1, 3);
        let rows = (j.feet - top - ry).max(1);
        let cy = top + ry;
        for y in cy..=j.feet {
            let t = (y - cy) as f32 / rows as f32;
            let half = j.hw as f32 * profile(&JARRO, t);
            let hx = half.round() as i32;
            for dx in -hx..=hx {
                let u = (dx as f32 / (half + 0.5)).clamp(-1.0, 1.0);
                let n = (1.0 - u * u).sqrt();
                let lam = 0.3 + 0.7 * (0.6 * n - 0.4 * u).clamp(0.0, 1.0);
                let mut col = if lam < 0.65 {
                    p.clay[2].lerp(p.clay[1], lam / 0.65)
                } else {
                    p.clay[1].lerp(p.clay[0], (lam - 0.65) / 0.35)
                };
                // Two painted bands, white and dark.
                if (0.3..0.4).contains(&t) {
                    col = p.cenefa.scale(0.6 + 0.4 * lam);
                } else if (0.62..0.7).contains(&t) {
                    col = p.clay[2];
                }
                // Warmed by the fire a few steps away.
                col = col.lerp(p.ember, (0.12 * (1.0 - u) * t).clamp(0.0, 0.3));
                if dx.abs() == hx {
                    col = col.scale(0.8);
                }
                px.set(j.x + dx, y, col);
            }
        }
        // The handle, a loop on the right.
        let hx = (j.hw as f32 * profile(&JARRO, 0.35)).round() as i32;
        let (y0, y1) = (cy + rows / 5, cy + rows * 3 / 5);
        px.set(j.x + hx + 1, y0, p.clay[1]);
        for y in y0 + 1..y1 {
            px.set(j.x + hx + 2, y, p.clay[1]);
        }
        px.set(j.x + hx + 1, y1, p.clay[1]);
        // The mouth, with the atole in it.
        let rx = (j.hw as f32 * 0.8).round();
        let ryf = ry as f32;
        for dy in -ry..=ry {
            let half =
                (rx * (1.0 - (dy as f32 / (ryf + 0.5)).powi(2)).max(0.0).sqrt()).round() as i32;
            for dx in -half..=half {
                let inner = (dx as f32 / (rx - 1.0).max(1.0)).powi(2)
                    + (dy as f32 / (ryf - 0.4).max(0.6)).powi(2);
                let col = if inner <= 1.0 {
                    // Atole, with a skin of foam at the back.
                    if dy < 0 { p.atole[1] } else { p.atole[0] }
                } else {
                    p.clay[0]
                };
                px.set(j.x + dx, cy + dy, col);
            }
        }
    }

    /// A plate of tamales lying in a pile, tied at both ends.
    fn draw_plate(&self, px: &mut Pixels, pl: &Plate) {
        let p = &self.pal;
        // The plate: a flat clay dish.
        for dx in -pl.rx..=pl.rx {
            let edge = dx.abs() == pl.rx;
            px.set(
                pl.x + dx,
                pl.ground,
                if edge { p.clay[2] } else { p.clay[1] },
            );
            if dx.abs() < pl.rx - 1 {
                px.set(pl.x + dx, pl.ground - 1, p.cenefa.scale(0.85));
            }
        }
        let lt = ((pl.rx as f32 * 0.8) as i32).max(5);
        let tt = if pl.rx >= 10 { 3 } else { 2 };
        for row in 0..pl.rows {
            let n = pl.rows - row;
            let y = pl.ground - 1 - (row + 1) * tt + 1;
            let span = n * lt - (n - 1);
            let x0 = pl.x - span / 2 + (row & 1);
            for k in 0..n {
                let tx = x0 + k * (lt - 1);
                for i in 0..lt {
                    for r in 0..tt {
                        // Rounded ends, a tie near each, the underside in shade.
                        if (i == 0 || i == lt - 1) && (r == 0 || r == tt - 1) && tt > 2 {
                            continue;
                        }
                        let col = if i == 1 || i == lt - 2 {
                            p.tie
                        } else if r == tt - 1 {
                            p.husk[0]
                        } else if r == 0 {
                            p.husk[2]
                        } else {
                            p.husk[1]
                        };
                        px.set(tx + i, y + r, col);
                    }
                }
            }
        }
    }

    /// A vela burning, its flame flickering.
    fn draw_vela(&self, px: &mut Pixels, v: &Vela) {
        let p = &self.pal;
        let lit = 0.8 + 0.2 * (self.t * 8.0 + v.phase).sin();
        let top = v.feet + 1 - v.h;
        let w = if v.h >= 6 { 2 } else { 1 };
        for y in top..=v.feet {
            for x in 0..w {
                let shade = if x == w - 1 && w > 1 { 0.78 } else { 1.0 };
                px.set(
                    v.x + x,
                    y,
                    p.wax.scale(shade).lerp(p.flame_edge, 0.15 * lit),
                );
            }
        }
        for x in 0..w {
            px.set(v.x + x, top - 1, p.flame);
        }
        px.set(
            v.x,
            top - 2,
            if lit > 0.85 { p.flame } else { p.flame_edge },
        );
        if lit > 0.9 {
            px.set(v.x, top - 3, p.flame_edge);
        }
    }
}
