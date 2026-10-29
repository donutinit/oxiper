//! Navidad: a fogata at a posada.
//!
//! A big olla de barro of ponche stands on its three stones over the fire,
//! and aafire's flames lick up its belly. Tejocotes, guayabas and ciruelas
//! pasas bob in the ponche as it turns, cañas and sticks of canela poke out
//! of it, a cucharón leans on its rim, and steam rises. Farolitos (paper
//! lanterns with a candle inside) line the ground on both sides of the fire,
//! in rows that fill up as the posadas go, with nochebuenas in pots among
//! them. Posada by posada the ponche fills with more fruit and steams
//! harder, the farolitos multiply, and on Nochebuena the fire is at its
//! fullest.

use super::{Fuel, Piece, Rect, Steam, cool, disc, line, profile, stone};
use crate::color::{Color, rgb};
use crate::render::Pixels;
use crate::scene::Ctx;

/// How much a row of farolitos behind the front one fades into the night.
const DIM: f32 = 0.22;

/// The olla's body from its mouth to its foot: half-width as a share of the
/// belly's, at each share of the way down.
const BODY: [(f32, f32); 7] = [
    (0.0, 0.80),
    (0.10, 0.74),
    (0.30, 0.94),
    (0.48, 1.0),
    (0.75, 0.90),
    (0.95, 0.64),
    (1.0, 0.56),
];

/// Farolitos by size: `r` the lit rim, `p` paper, `w` a window cut in it,
/// `s` the sand it stands in, `f` the flame, `Y` its core.
#[rustfmt::skip]
const FAROLITOS: [&[&str]; 3] = [
    &[".f.", "rYr", "ppp", "sss"],
    &["..f..", "rrYrr", "ppwpp", "pwwwp", "ppwpp", "sssss"],
    &["...f...", "..fYf..", "rrrrrrr", "pppwppp", "ppwwwpp", "pwwwwwp", "ppwwwpp", "pppwppp", "sssssss"],
];

/// Nochebuenas in their pots: `R` and `r` bracts, `y` the little yellow
/// flowers, `g` leaves, `c` clay and `C` its shade.
#[rustfmt::skip]
const NOCHEBUENAS: [&[&str]; 3] = [
    &["RyR", "gRg", ".c."],
    &["R.R.R", "RRyRR", ".gRg.", ".ccc.", "..C.."],
    &["r.R.R.r", ".RRyRR.", "gRRyRRg", ".gRRRg.", "..ggg..", ".ccccc.", "..CCC.."],
];

#[derive(Clone, Copy)]
struct Olla {
    cx: i32,
    /// Pixel row of the mouth's center, and of the foot.
    cy: i32,
    bottom: i32,
    /// Half the belly's width, and the mouth's half-width and half-height.
    hw: i32,
    rx: i32,
    ry: i32,
    /// How tall the stones it stands on are.
    stone: i32,
    /// The ground's pixel row.
    ground: i32,
}

#[derive(Clone, Copy)]
struct Fruit {
    /// 0 tejocote, 1 guayaba, 2 ciruela pasa.
    kind: u8,
    /// Where it floats as the ponche turns: how far from the middle (0 to 1)
    /// and at which angle, and how fast it goes round.
    rho: f32,
    ang: f32,
    spin: f32,
    phase: f32,
}

/// A caña or a stick of canela sticking out of the pot, from the mouth's
/// middle to its tip.
#[derive(Clone, Copy)]
struct Stick {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    canela: bool,
}

#[derive(Clone, Copy)]
struct Farolito {
    x: i32,
    feet: i32,
    size: usize,
    depth: u8,
    paper: u8,
    phase: f32,
}

#[derive(Clone, Copy)]
struct Nochebuena {
    x: i32,
    feet: i32,
    size: usize,
    depth: u8,
}

/// Colors of the holiday, cached in `recolor`.
struct Pal {
    /// The olla's clay: lit, in between, shaded.
    clay: [Color; 3],
    rim: Color,
    ponche: [Color; 2],
    tejocote: [Color; 2],
    guayaba: [Color; 2],
    ciruela: Color,
    cana: Color,
    canela: Color,
    wood: Color,
    stone: [Color; 3],
    /// Crema, rojo, verde, dorado.
    paper: [Color; 4],
    sand: Color,
    flame: Color,
    flame_edge: Color,
    ember: Color,
    bract: [Color; 2],
    yellow: Color,
    leaf: Color,
    steam: Color,
}

pub(super) struct Posada {
    level: f32,
    seed: u64,
    w: i32,
    ph: i32,
    olla: Option<Olla>,
    fruits: Vec<Fruit>,
    sticks: Vec<Stick>,
    farolitos: Vec<Farolito>,
    plants: Vec<Nochebuena>,
    steam: Steam,
    t: f32,
    pal: Pal,
}

impl Posada {
    pub fn new(seed: u64) -> Posada {
        let none = Color::Reset;
        Posada {
            level: 0.0,
            seed,
            w: 0,
            ph: 0,
            olla: None,
            fruits: Vec::new(),
            sticks: Vec::new(),
            farolitos: Vec::new(),
            plants: Vec::new(),
            steam: Steam::new(seed ^ 0x57ea),
            t: 0.0,
            pal: Pal {
                clay: [none; 3],
                rim: none,
                ponche: [none; 2],
                tejocote: [none; 2],
                guayaba: [none; 2],
                ciruela: none,
                cana: none,
                canela: none,
                wood: none,
                stone: [none; 3],
                paper: [none; 4],
                sand: none,
                flame: none,
                flame_edge: none,
                ember: none,
                bract: [none; 2],
                yellow: none,
                leaf: none,
                steam: none,
            },
        }
    }
}

impl Piece for Posada {
    fn recolor(&mut self, ctx: &Ctx) {
        let themed = !ctx.theme.native;
        let fg = ctx.theme.fg;
        let g = &ctx.theme.grad;
        let or = |native: u32, theme: Color| if themed { theme } else { rgb(native) };
        let p = &mut self.pal;
        p.clay = [
            or(0xd88a54, g.at(0.7)),
            or(0xa8502c, g.at(0.5)),
            or(0x5c2a1a, g.at(0.25)),
        ];
        p.rim = or(0xe6a878, g.at(0.8));
        p.ponche = [
            or(0x8a1c1c, ctx.pick(6, rgb(0x8a1c1c)).scale(0.7)),
            or(0xd2502a, ctx.pick(0, rgb(0xd2502a))),
        ];
        // The fruit keeps its own hues under any theme, like a flame.
        p.tejocote = [rgb(0xe8891c), rgb(0xffc060)];
        p.guayaba = [rgb(0xd8cc58), rgb(0xf0a8a0)];
        p.ciruela = rgb(0x5a2e62);
        p.cana = or(0xc9b26b, ctx.pick(5, Color::Reset));
        p.canela = or(0x8b4a1e, ctx.pick(4, Color::Reset).scale(0.6));
        p.wood = or(0x9a6a3a, g.at(0.45));
        p.stone = [
            or(0x9a8e84, ctx.fade(fg, 0.45)),
            or(0x6e625a, ctx.fade(fg, 0.6)),
            or(0x463d38, ctx.fade(fg, 0.78)),
        ];
        p.paper = [
            or(0xf4f1ea, fg),
            or(0xc8102e, ctx.pick(6, Color::Reset)),
            or(0x1f6f3a, ctx.pick(3, Color::Reset)),
            or(0xf2b632, ctx.pick(4, Color::Reset)),
        ];
        p.sand = or(0x8a7a5a, ctx.fade(fg, 0.6));
        // A candle flame and a fire's glow keep their own hues, too.
        p.flame = rgb(0xffe8a0);
        p.flame_edge = rgb(0xff9628);
        p.ember = rgb(0xff7a1a);
        p.bract = [
            or(0xc8102e, ctx.pick(6, Color::Reset)),
            or(0x8e0c22, ctx.pick(6, Color::Reset).scale(0.65)),
        ];
        p.yellow = or(0xf2b632, ctx.pick(4, Color::Reset));
        p.leaf = or(0x1f6f3a, ctx.pick(3, Color::Reset).scale(0.7));
        p.steam = or(0xe4e2ee, fg);
    }

    fn layout(&mut self, w: usize, ph: usize, level: f32) {
        self.olla = None;
        self.fruits.clear();
        self.sticks.clear();
        self.farolitos.clear();
        self.plants.clear();
        self.steam.reset(0.0);
        let (w, ph) = (w as i32, ph as i32);
        self.w = w;
        self.ph = ph;
        self.level = level;
        if ph < 18 || w < 14 {
            return;
        }
        let mut rng = fastrand::Rng::with_seed(self.seed ^ 0x5eed);
        // The olla: a third of the screen's height at most, as wide as tall.
        let h = (ph * 33 / 100).clamp(9, 50);
        let hw = (h / 2).max(5).min((w - 4) / 2);
        let rx = (hw * 4 / 5).max(3);
        let ry = (rx * 2 / 7).clamp(2, 7);
        let stone = (h / 7).max(2);
        let ground = ph - 1;
        let bottom = ground - stone;
        let olla = Olla {
            cx: w / 2,
            cy: bottom - h + 1 + ry,
            bottom,
            hw,
            rx,
            ry,
            stone,
            ground,
        };
        self.olla = Some(olla);

        // Fruit bobbing in the ponche: more as the posadas go.
        let room = (rx * 2 / 3).max(3) as usize;
        let n = (3.0 + level * 6.0) as usize;
        for i in 0..n.min(room) {
            self.fruits.push(Fruit {
                kind: [0, 1, 0, 2, 0, 1][i % 6],
                rho: 0.25 + 0.6 * rng.f32(),
                ang: rng.f32() * std::f32::consts::TAU,
                spin: (0.12 + 0.25 * rng.f32()) * if i % 2 == 0 { 1.0 } else { -0.7 },
                phase: rng.f32() * std::f32::consts::TAU,
            });
        }
        // Cañas and canela sticking out of it, leaning away from each other.
        let sticks = 1 + (level * 1.6) as i32;
        let len = (h as f32 * 0.34).max(3.0);
        for i in 0..sticks * 2 {
            let canela = i % 2 == 1;
            let side = if (i / 2) % 2 == 0 { -1.0 } else { 1.0 };
            let x0 = ((rng.f32() * 1.2 - 0.6) * rx as f32) as i32;
            let lean = side * (0.15 + 0.35 * rng.f32());
            let l = len * if canela { 0.65 } else { 1.0 } * (0.75 + 0.25 * rng.f32());
            self.sticks.push(Stick {
                x0,
                y0: 0,
                x1: x0 + (lean * l).round() as i32,
                y1: -((1.0 - lean * lean).sqrt() * l).round() as i32 - ry,
                canela,
            });
        }
        // Steam rises from the whole mouth.
        self.steam.reset(3.0 + ph as f32 * 0.085);
        self.steam.mouths.push(super::Mouth {
            x: olla.cx as f32,
            y: (olla.cy - ry) as f32,
            half: rx as f32 * 0.75,
            rate: rx as f32 * (1.2 + 1.8 * level),
            acc: 0.0,
        });
        self.steam.reserve();

        // Farolitos along the ground on both sides of the fire, nochebuenas
        // in pots among them.
        let fs: usize = if ph >= 90 {
            2
        } else if ph >= 40 {
            1
        } else {
            0
        };
        let fw = [3, 5, 7];
        let fh = [4, 6, 8];
        let edge = hw * 3 / 2 + 2;
        let rows = [
            (fs, ground, 0.3 + 0.7 * level, 0u8),
            (
                fs.saturating_sub(1),
                ground - (ph / 12).max(fh[fs] / 2 + 2),
                (level - 0.4) / 0.6 * 0.85,
                1u8,
            ),
        ];
        for (size, feet, lit, depth) in rows {
            let pitch = fw[size] * 2 + 2;
            let offset = if depth == 1 { pitch / 2 } else { 0 };
            for side in [-1, 1] {
                let mut x = olla.cx + side * (edge + fw[size] / 2 + offset);
                let mut last = u8::MAX;
                while x - fw[size] / 2 >= 0 && x + fw[size] / 2 < w {
                    let at = x - fw[size] / 2 + rng.i32(-1..=1);
                    let roll = rng.f32();
                    let paper = {
                        let mut p = [0, 0, 0, 1, 2, 3][rng.usize(0..6)];
                        if p == last {
                            p = (p + 1) % 4;
                        }
                        p
                    };
                    let phase = rng.f32() * std::f32::consts::TAU;
                    let pot = rng.f32();
                    x += side * pitch;
                    if lit <= 0.0 || roll >= lit {
                        continue;
                    }
                    if depth == 0 && level >= 0.55 && pot < 0.14 {
                        self.plants.push(Nochebuena {
                            x: at,
                            feet,
                            size,
                            depth,
                        });
                        continue;
                    }
                    last = paper;
                    self.farolitos.push(Farolito {
                        x: at,
                        feet,
                        size,
                        depth,
                        paper,
                        phase,
                    });
                }
            }
        }
    }

    fn fuel(&self, sw: usize) -> Option<Fuel> {
        let o = self.olla?;
        let reach = o.hw * 9 / 4;
        Some(Fuel::span(
            o.cx - reach,
            o.cx + reach,
            sw,
            0.8 + 0.2 * self.level,
        ))
    }

    fn reach(&self) -> f32 {
        // Flames about as high as the olla, a little above its shoulders.
        if self.olla.is_some() { 0.7 } else { 1.0 }
    }

    fn update(&mut self, dt: f32, wind: f32) {
        self.t += dt;
        if self.olla.is_some() {
            self.steam.update(dt, wind);
        }
    }

    fn block(&self, heat: &mut [u16], stride: usize) {
        let Some(o) = &self.olla else { return };
        // The olla is in the way: flames go round it, and what gets through
        // its belly is soon spent. The gap between the stones stays open.
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
        self.olla.map(|_| (&self.steam, self.pal.steam))
    }

    fn draw(&self, _ctx: &Ctx, px: &mut Pixels) {
        let Some(o) = &self.olla else { return };
        self.draw_olla(px, o);
        self.draw_ponche(px, o);
    }

    fn lick(&self) -> Option<Rect> {
        let o = self.olla?;
        Some(Rect {
            x0: o.cx - o.hw - 1,
            x1: o.cx + o.hw + 1,
            y0: o.bottom - (o.bottom - o.cy) * 55 / 100,
            y1: o.bottom,
        })
    }

    fn draw_front(&self, ctx: &Ctx, px: &mut Pixels) {
        let Some(o) = &self.olla else { return };
        // The stones the olla stands on, warmed by the fire between them.
        let p = &self.pal;
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
        for f in &self.farolitos {
            self.draw_farolito(ctx, px, f);
        }
        for n in &self.plants {
            self.draw_plant(ctx, px, n);
        }
    }

    #[cfg(test)]
    fn counts(&self) -> (usize, usize, usize) {
        (self.fruits.len(), self.farolitos.len(), self.plants.len())
    }
}

impl Posada {
    /// The olla's body, its handles and its rim.
    fn draw_olla(&self, px: &mut Pixels, o: &Olla) {
        let p = &self.pal;
        let rows = (o.bottom - o.cy).max(1);
        let breath = 0.9 + 0.1 * (self.t * 5.0).sin();
        for y in o.cy..=o.bottom {
            let t = (y - o.cy) as f32 / rows as f32;
            let half = o.hw as f32 * profile(&BODY, t);
            let hx = half.round() as i32;
            for dx in -hx..=hx {
                let u = (dx as f32 / (half + 0.5)).clamp(-1.0, 1.0);
                // Round, lit from the left, and warmed from below by the fire.
                let n = (1.0 - u * u).sqrt();
                let lam = 0.3 + 0.7 * (0.6 * n - 0.4 * u).clamp(0.0, 1.0);
                let mut col = if lam < 0.65 {
                    p.clay[2].lerp(p.clay[1], lam / 0.65)
                } else {
                    p.clay[1].lerp(p.clay[0], (lam - 0.65) / 0.35)
                };
                // A band of dots round the shoulder, a thin line lower down.
                if (0.2..0.27).contains(&t) {
                    col = if (dx + y) & 1 == 0 { p.rim } else { p.clay[2] };
                } else if (0.64..0.68).contains(&t) {
                    col = col.lerp(p.clay[2], 0.7);
                }
                let glow = (t * t * 0.8 + 0.1 * u.abs()) * breath;
                col = col.lerp(p.ember, glow.clamp(0.0, 0.85));
                if dx.abs() == hx {
                    col = col.scale(0.78);
                }
                px.set(o.cx + dx, y, col);
            }
        }
        // Two ears at the neck.
        let ey = o.cy + rows / 6;
        let neck = (o.hw as f32 * profile(&BODY, 0.17)).round() as i32;
        for side in [-1, 1] {
            let x = o.cx + side * (neck + 1);
            px.set(x, ey, p.clay[1]);
            px.set(x + side, ey + 1, p.clay[2]);
            px.set(x, ey + 2, p.clay[2]);
        }
    }

    /// The mouth with the ponche in it, the fruit bobbing, the sticks and the
    /// cucharón.
    fn draw_ponche(&self, px: &mut Pixels, o: &Olla) {
        let p = &self.pal;
        let (rx, ry) = (o.rx as f32, o.ry as f32);
        for dy in -o.ry..=o.ry {
            let half =
                (rx * (1.0 - (dy as f32 / (ry + 0.5)).powi(2)).max(0.0).sqrt()).round() as i32;
            for dx in -half..=half {
                let inner = (dx as f32 / (rx - 1.2)).powi(2) + (dy as f32 / (ry - 0.7)).powi(2);
                let col = if inner <= 1.0 {
                    // The ponche, shimmering, darker at the back.
                    let wave = 0.5 + 0.5 * (dx as f32 * 0.7 + dy as f32 * 1.9 + self.t * 2.4).sin();
                    let back = if dy < 0 { 0.8 } else { 1.0 };
                    p.ponche[0].lerp(p.ponche[1], 0.15 + 0.5 * wave * back)
                } else if dy < 0 {
                    p.rim
                } else {
                    p.rim.scale(0.8)
                };
                px.set(o.cx + dx, o.cy + dy, col);
            }
        }
        // The sticks stand out of the ponche; the fruit floats on it.
        for s in &self.sticks {
            let col = if s.canela { p.canela } else { p.cana };
            line(px, o.cx + s.x0, o.cy + s.y0, o.cx + s.x1, o.cy + s.y1, col);
            if !s.canela {
                // A caña is thicker, with a node at its tip.
                line(
                    px,
                    o.cx + s.x0 + 1,
                    o.cy + s.y0,
                    o.cx + s.x1 + 1,
                    o.cy + s.y1,
                    col.scale(0.8),
                );
                px.set(o.cx + s.x1, o.cy + s.y1, col.scale(0.6));
            }
        }
        let fr = (rx / 7.0).clamp(0.9, 2.6);
        for f in &self.fruits {
            let a = f.ang + self.t * f.spin;
            let x = o.cx + (f.rho * a.cos() * (rx - 1.5)).round() as i32;
            let bob = (self.t * 2.0 + f.phase).sin() * 0.7;
            let y = o.cy + (f.rho * a.sin() * (ry - 1.0) + bob).round() as i32;
            let (body, lit) = match f.kind {
                0 => (p.tejocote[0], p.tejocote[1]),
                1 => (p.guayaba[0], p.guayaba[1]),
                _ => (p.ciruela, p.ciruela.lerp(p.rim, 0.4)),
            };
            disc(px, x, y, fr, body);
            px.set(x, y - (fr as i32).max(1) + 1, lit);
        }
        // A cucharón leans on the rim, its handle out in the open.
        if o.hw >= 9 {
            let top = o.cy - o.ry - (o.bottom - o.cy) * 2 / 5;
            line(px, o.cx + o.rx / 3, o.cy, o.cx + o.rx * 5 / 4, top, p.wood);
            px.set(o.cx + o.rx * 5 / 4 + 1, top, p.wood.scale(0.7));
        }
    }

    fn draw_farolito(&self, ctx: &Ctx, px: &mut Pixels, f: &Farolito) {
        let p = &self.pal;
        let art = FAROLITOS[f.size];
        let lit = 0.8 + 0.2 * (self.t * 7.0 + f.phase).sin();
        let paper = p.paper[f.paper as usize];
        let dim = f.depth as f32 * DIM;
        let top = f.feet + 1 - art.len() as i32;
        for (ry, row) in art.iter().enumerate() {
            for (rx, ch) in row.bytes().enumerate() {
                let col = match ch {
                    b'f' if lit > 0.88 => p.flame_edge,
                    b'f' => continue,
                    b'Y' => p.flame,
                    b'r' => paper.lerp(p.flame, 0.5 * lit),
                    b'p' => paper.scale(0.6).lerp(p.flame_edge, 0.3 * lit),
                    b'w' => p.flame.lerp(paper, 0.25),
                    b's' => p.sand,
                    _ => continue,
                };
                px.set(f.x + rx as i32, top + ry as i32, ctx.fade(col, dim));
            }
        }
    }

    fn draw_plant(&self, ctx: &Ctx, px: &mut Pixels, n: &Nochebuena) {
        let p = &self.pal;
        let art = NOCHEBUENAS[n.size];
        let top = n.feet + 1 - art.len() as i32;
        for (ry, row) in art.iter().enumerate() {
            for (rx, ch) in row.bytes().enumerate() {
                let col = match ch {
                    b'R' => p.bract[0],
                    b'r' => p.bract[1],
                    b'y' => p.yellow,
                    b'g' => p.leaf,
                    b'c' => p.clay[1],
                    b'C' => p.clay[2],
                    _ => continue,
                };
                px.set(
                    n.x + rx as i32,
                    top + ry as i32,
                    ctx.fade(col, n.depth as f32 * DIM),
                );
            }
        }
    }
}
