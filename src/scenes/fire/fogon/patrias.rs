//! Fiestas Patrias: a fogón on the night of the Grito.
//!
//! A pozolera (a big enamelled stewpot, verde bandera with a white rim)
//! bubbles over the fire: the pozole is rojo, its hominy blanco, and every
//! now and then a bubble swells and bursts. Beside it a comal, the round
//! griddle, roasts elotes, their husks peeled back and the kernels charring
//! where the flames lick round it. Luces de bengala stand in the ground
//! around the hearth and sputter white-gold sparks, and banderitas wave in
//! rows on both sides of it. Through the mes patrio more elotes go on the
//! comal, the pozole boils harder, more bengalas are lit and the banderitas
//! multiply; on the night of the Grito every one of them sputters.

use super::{Fuel, Mouth, Piece, Rect, Sparks, Steam, cool, disc, hash, line, profile, stone};
use crate::color::{Color, rgb};
use crate::render::Pixels;
use crate::scene::Ctx;

/// How much a row of banderitas behind the front one fades into the night.
const DIM: f32 = 0.25;

/// The pozolera's body from its mouth to its foot: half-width as a share of
/// the widest, at each share of the way down. Nearly straight, with a lip.
const POT: [(f32, f32); 5] = [
    (0.0, 0.93),
    (0.06, 1.0),
    (0.14, 0.94),
    (0.9, 0.92),
    (1.0, 0.84),
];

#[derive(Clone, Copy)]
struct Pozolera {
    cx: i32,
    /// Pixel row of the mouth's center, and of the foot.
    cy: i32,
    bottom: i32,
    /// Half its width, and the mouth's half-height.
    hw: i32,
    ry: i32,
    /// How tall the stones it stands on are.
    stone: i32,
}

#[derive(Clone, Copy)]
struct Comal {
    cx: i32,
    /// Pixel row of the middle of its top, and how far it spreads: half its
    /// width and half its depth. Under it, a slab `thick` pixels deep.
    cy: i32,
    rx: i32,
    ry: i32,
    thick: i32,
}

/// An elote lying on the comal.
#[derive(Clone, Copy)]
struct Elote {
    /// Where on the comal, from -1 to 1 across and from -1 (back) to 1.
    fx: f32,
    fy: f32,
    /// Which way it leans, in pixel rows over its length.
    slope: i32,
    phase: f32,
}

/// A bubble swelling in the pozole, where it is in the mouth (-1 to 1).
#[derive(Clone, Copy)]
struct Bubble {
    x: f32,
    y: f32,
    age: f32,
    life: f32,
}

/// A luz de bengala stuck in the ground.
#[derive(Clone, Copy)]
struct Bengala {
    x: i32,
    ground: i32,
    len: i32,
    /// Where its tip leans, in pixels at the top.
    lean: i32,
    /// Where it is in its burn (0 to 1, then a new one is lit), and how long
    /// one lasts.
    phase: f32,
    period: f32,
}

#[derive(Clone, Copy)]
struct Banderita {
    x: i32,
    feet: i32,
    size: usize,
    depth: u8,
    phase: f32,
}

/// Colors of the holiday, cached in `recolor`.
struct Pal {
    /// The pozolera's enamel: shaded, in between, lit.
    green: [Color; 3],
    white: Color,
    /// The pozole, and where it boils.
    red: [Color; 2],
    leaf: Color,
    /// The comal's iron: its top, its rim, its slab.
    iron: [Color; 3],
    /// Kernels, and the lighter ones; the husks; charred.
    corn: [Color; 2],
    husk: [Color; 2],
    char: Color,
    stone: [Color; 3],
    ember: Color,
    wood: Color,
    gold: Color,
    brown: Color,
    plata: Color,
    steam: Color,
}

pub(super) struct Grito {
    level: f32,
    seed: u64,
    rng: fastrand::Rng,
    w: i32,
    ph: i32,
    pot: Option<Pozolera>,
    comal: Option<Comal>,
    elotes: Vec<Elote>,
    bubbles: Vec<Bubble>,
    bubble_acc: f32,
    bengalas: Vec<Bengala>,
    banderitas: Vec<Banderita>,
    /// How many bengalas are lit, the first ones.
    lit: usize,
    steam: Steam,
    sparks: Sparks,
    t: f32,
    pal: Pal,
}

impl Grito {
    pub fn new(seed: u64) -> Grito {
        let none = Color::Reset;
        Grito {
            level: 0.0,
            seed,
            rng: fastrand::Rng::with_seed(seed ^ 0xb0bb),
            w: 0,
            ph: 0,
            pot: None,
            comal: None,
            elotes: Vec::new(),
            bubbles: Vec::new(),
            bubble_acc: 0.0,
            bengalas: Vec::new(),
            banderitas: Vec::new(),
            lit: 0,
            steam: Steam::new(seed ^ 0x57ea),
            sparks: Sparks::new(seed ^ 0x5ba2),
            t: 0.0,
            pal: Pal {
                green: [none; 3],
                white: none,
                red: [none; 2],
                leaf: none,
                iron: [none; 3],
                corn: [none; 2],
                husk: [none; 2],
                char: none,
                stone: [none; 3],
                ember: none,
                wood: none,
                gold: none,
                brown: none,
                plata: none,
                steam: none,
            },
        }
    }
}

impl Piece for Grito {
    fn recolor(&mut self, ctx: &Ctx) {
        let themed = !ctx.theme.native;
        let fg = ctx.theme.fg;
        let g = &ctx.theme.grad;
        let or = |native: u32, theme: Color| if themed { theme } else { rgb(native) };
        let p = &mut self.pal;
        let green = or(0x006847, ctx.pick(3, Color::Reset));
        p.green = [green.scale(0.55), green, green.lerp(fg, 0.28)];
        p.white = or(0xf4f1ea, fg);
        p.red = [
            or(0xb01428, ctx.pick(6, Color::Reset).scale(0.8)),
            or(0xe0483c, ctx.pick(6, Color::Reset)),
        ];
        p.leaf = or(0x4f8a2a, ctx.pick(3, Color::Reset).scale(0.9));
        p.iron = [
            or(0x3a2f2a, ctx.fade(fg, 0.85)),
            or(0x6a5a50, ctx.fade(fg, 0.6)),
            or(0x251d1a, ctx.fade(fg, 0.92)),
        ];
        // Corn keeps its own hues under any theme, like a flame.
        p.corn = [rgb(0xf2c230), rgb(0xffe27a)];
        p.husk = [or(0x6f9a34, ctx.pick(3, Color::Reset)), rgb(0xc9b26b)];
        p.char = rgb(0x4a2c12);
        p.stone = [
            or(0x9a8e84, ctx.fade(fg, 0.45)),
            or(0x6e625a, ctx.fade(fg, 0.6)),
            or(0x463d38, ctx.fade(fg, 0.78)),
        ];
        p.ember = rgb(0xff7a1a);
        p.wood = or(0x9a6a3a, g.at(0.45));
        p.gold = or(0xd4a017, ctx.pick(4, Color::Reset));
        p.brown = or(0x7a4a1e, g.at(0.3));
        p.plata = or(0xc9ccd6, fg);
        p.steam = or(0xe4e2ee, fg);
    }

    fn layout(&mut self, w: usize, ph: usize, level: f32) {
        self.pot = None;
        self.comal = None;
        self.elotes.clear();
        self.bubbles.clear();
        self.bengalas.clear();
        self.banderitas.clear();
        self.sparks.clear();
        self.steam.reset(0.0);
        let (w, ph) = (w as i32, ph as i32);
        self.w = w;
        self.ph = ph;
        self.level = level;
        if ph < 20 || w < 26 {
            return;
        }
        let mut rng = fastrand::Rng::with_seed(self.seed ^ 0x5eed);
        let h = (ph * 33 / 100).clamp(10, 50);
        // The pozolera and the comal side by side, as wide as the screen allows.
        let hw = (h * 47 / 100).max(5).min((w - 8) * 10 / 52);
        let gap = (hw / 4).max(2);
        let crx = hw * 5 / 4;
        let total = 2 * hw + gap + 2 * crx;
        let left = w / 2 - total / 2;
        let ground = ph - 1;
        let stone_h = (h / 7).max(2);
        let ry = (hw * 2 / 7).clamp(2, 7);
        let pot = Pozolera {
            cx: left + hw,
            cy: ground - stone_h - h + 1 + ry,
            bottom: ground - stone_h,
            hw,
            ry,
            stone: stone_h,
        };
        let cry = (crx * 3 / 10).clamp(2, 7);
        let thick = (h / 14).max(1);
        let comal = Comal {
            cx: left + 2 * hw + gap + crx,
            cy: ground - stone_h - cry - thick,
            rx: crx,
            ry: cry,
            thick,
        };
        self.pot = Some(pot);
        self.comal = Some(comal);

        // Elotes on the comal: two or three rows of them, more as the mes goes.
        let n = 2 + (level * 3.4) as usize;
        for i in 0..n.min(5) {
            self.elotes.push(Elote {
                fx: [-0.45, 0.4, 0.0, -0.5, 0.5][i],
                fy: [-0.5, -0.45, 0.45, 0.5, 0.45][i],
                slope: [0, 1, -1, 1, 0][i],
                phase: rng.f32() * 20.0,
            });
        }
        // Steam from the pozolera, a thread of smoke from the elotes.
        self.steam.reset(3.0 + ph as f32 * 0.085);
        self.steam.mouths.push(Mouth {
            x: pot.cx as f32,
            y: (pot.cy - ry) as f32,
            half: hw as f32 * 0.7,
            rate: hw as f32 * (1.0 + 2.0 * level),
            acc: 0.0,
        });
        self.steam.mouths.push(Mouth {
            x: comal.cx as f32,
            y: (comal.cy - cry - 1) as f32,
            half: crx as f32 * 0.5,
            rate: 1.0 + 3.0 * level,
            acc: 0.0,
        });
        self.steam.reserve();
        self.bubbles.reserve_exact(64);

        // Bengalas in the ground on both sides of the hearth.
        let len = (h as f32 * 0.4).round() as i32;
        let edge = total / 2 + 3;
        let slots = if ph >= 90 { 3 } else { 2 };
        for k in 0..slots {
            for side in [-1, 1] {
                let off = edge + k * (len / 2 + 5) + rng.i32(0..=2);
                let x = w / 2 + side * off;
                if x < 1 || x >= w - 1 {
                    continue;
                }
                self.bengalas.push(Bengala {
                    x,
                    ground,
                    len,
                    lean: rng.i32(-2..=2),
                    phase: rng.f32(),
                    period: 16.0 + rng.f32() * 10.0,
                });
            }
        }
        // The Grito lights them all; the first of September, one.
        let more = self.bengalas.len().saturating_sub(1) as f32;
        self.lit = (1 + (level * more).round() as usize).min(self.bengalas.len());
        self.sparks.reserve(self.bengalas.len() * 40 + 16);

        // Banderitas planted along the ground beyond them, in rows that fill
        // up through the month.
        let size: usize = if ph >= 90 {
            2
        } else if ph >= 40 {
            1
        } else {
            0
        };
        let fw = [4, 6, 9];
        let fh = [5, 8, 11];
        let rows = [
            (size, ground, 0.2 + 0.8 * level, 0u8),
            (
                size.saturating_sub(1),
                ground - (ph / 12).max(fh[size] / 2 + 2),
                (level - 0.45) / 0.55,
                1u8,
            ),
        ];
        for (size, feet, lit, depth) in rows {
            let pitch = fw[size] * 2 + 3;
            let offset = if depth == 1 { pitch / 2 } else { 0 };
            for side in [-1, 1] {
                let mut x = w / 2 + side * (edge + (len / 2 + 5) * slots + 4 + offset);
                while x >= 1 && x + fw[size] < w - 1 {
                    let at = x + rng.i32(-1..=1);
                    let roll = rng.f32();
                    let phase = rng.f32() * std::f32::consts::TAU;
                    x += side * pitch;
                    if lit <= 0.0 || roll >= lit {
                        continue;
                    }
                    self.banderitas.push(Banderita {
                        x: at,
                        feet,
                        size,
                        depth,
                        phase,
                    });
                }
            }
        }
    }

    fn fuel(&self, sw: usize) -> Option<Fuel> {
        let (p, c) = (self.pot?, self.comal?);
        Some(Fuel::span(
            p.cx - p.hw * 2,
            c.cx + c.rx * 2,
            sw,
            0.8 + 0.2 * self.level,
        ))
    }

    fn reach(&self) -> f32 {
        if self.pot.is_some() { 0.95 } else { 1.0 }
    }

    fn update(&mut self, dt: f32, wind: f32) {
        self.t += dt;
        let Some(pot) = self.pot else { return };
        self.steam.update(dt, wind);
        // Bubbles swell in the pozole and burst.
        self.bubble_acc += dt * pot.hw as f32 * (0.5 + 1.5 * self.level);
        while self.bubble_acc >= 1.0 {
            self.bubble_acc -= 1.0;
            if self.bubbles.len() < self.bubbles.capacity() {
                let b = Bubble {
                    x: self.rng.f32() * 1.7 - 0.85,
                    y: self.rng.f32() * 1.5 - 0.75,
                    age: 0.0,
                    life: 0.5 + self.rng.f32() * 0.7,
                };
                self.bubbles.push(b);
            }
        }
        self.bubbles.retain_mut(|b| {
            b.age += dt;
            b.age < b.life
        });
        // The bengalas sputter: bursts of sparks that come and go.
        let t = self.t;
        for (i, b) in self.bengalas.iter().enumerate().take(self.lit) {
            let frac = (t / b.period + b.phase).fract();
            if frac < 0.04 {
                continue;
            }
            let (tx, ty) = b.tip(frac);
            let sputter = (t * 11.0 + i as f32 * 2.3).sin() * (t * 3.7 + i as f32).sin();
            let rate = (25.0 + 60.0 * sputter.max(0.0)) * dt;
            let n = rate as usize + (self.rng.f32() < rate.fract()) as usize;
            for _ in 0..n {
                self.sparks.spawn(tx, ty, 34.0, 0);
            }
        }
        self.sparks.update(dt);
    }

    fn block(&self, heat: &mut [u16], stride: usize) {
        if let Some(o) = &self.pot {
            let rows = (o.bottom - o.cy).max(1);
            for y in o.cy - o.ry..=o.bottom {
                let t = ((y - o.cy).max(0)) as f32 / rows as f32;
                let half = (o.hw as f32 * profile(&POT, t)).round() as i32 + 1;
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
        if let Some(c) = &self.comal {
            // The comal is a plate: what gets through it is a flicker.
            for y in c.cy - c.ry..=c.cy + c.ry + c.thick {
                cool(heat, stride, y, c.cx - c.rx - 1, c.cx + c.rx + 1, 0.4);
            }
        }
    }

    fn steam(&self) -> Option<(&Steam, Color)> {
        self.pot.map(|_| (&self.steam, self.pal.steam))
    }

    fn draw(&self, _ctx: &Ctx, px: &mut Pixels) {
        if let Some(o) = &self.pot {
            self.draw_pot(px, o);
        }
        if let Some(c) = &self.comal {
            self.draw_comal(px, c);
        }
    }

    fn lick(&self) -> Option<Rect> {
        let (o, c) = (self.pot?, self.comal?);
        Some(Rect {
            x0: o.cx - o.hw - 1,
            x1: c.cx + c.rx + 1,
            y0: o.bottom - (o.bottom - o.cy) * 55 / 100,
            y1: o.bottom + o.stone,
        })
    }

    fn draw_front(&self, ctx: &Ctx, px: &mut Pixels) {
        let (Some(o), Some(c)) = (&self.pot, &self.comal) else {
            return;
        };
        // The stones they stand on, warmed by the fire between them.
        let p = &self.pal;
        let ground = o.bottom + o.stone;
        let sw = (o.hw * 11 / 20).max(3);
        for (cx, reach) in [(o.cx, o.hw * 3 / 5), (c.cx, c.rx * 3 / 5)] {
            for side in [-1, 1] {
                let x0 = cx + side * reach - sw / 2;
                let at = Rect {
                    x0,
                    y0: ground - o.stone + 1,
                    x1: x0 + sw - 1,
                    y1: ground,
                };
                stone(px, at, -side, &p.stone, p.ember);
            }
        }
        for b in &self.banderitas {
            self.draw_banderita(ctx, px, b);
        }
        for (i, b) in self.bengalas.iter().enumerate() {
            self.draw_bengala(px, b, i < self.lit);
        }
        self.sparks.draw(px);
    }

    #[cfg(test)]
    fn counts(&self) -> (usize, usize, usize) {
        (self.elotes.len(), self.lit, self.banderitas.len())
    }
}

impl Bengala {
    /// Where its tip is when it has burned `frac` of its way down.
    fn tip(&self, frac: f32) -> (f32, f32) {
        let k = 1.0 - 0.7 * frac;
        (
            self.x as f32 + self.lean as f32 * k,
            self.ground as f32 - self.len as f32 * k,
        )
    }
}

impl Grito {
    /// The pozolera: enamel, white-speckled, and the pozole in its mouth.
    fn draw_pot(&self, px: &mut Pixels, o: &Pozolera) {
        let p = &self.pal;
        let rows = (o.bottom - o.cy).max(1);
        let breath = 0.9 + 0.1 * (self.t * 5.0).sin();
        for y in o.cy..=o.bottom {
            let t = (y - o.cy) as f32 / rows as f32;
            let half = o.hw as f32 * profile(&POT, t);
            let hx = half.round() as i32;
            for dx in -hx..=hx {
                let u = (dx as f32 / (half + 0.5)).clamp(-1.0, 1.0);
                let n = (1.0 - u * u).sqrt();
                let lam = 0.3 + 0.7 * (0.6 * n - 0.4 * u).clamp(0.0, 1.0);
                let mut col = if lam < 0.65 {
                    p.green[0].lerp(p.green[1], lam / 0.65)
                } else {
                    p.green[1].lerp(p.green[2], (lam - 0.65) / 0.35)
                };
                if t < 0.07 {
                    // The white rim.
                    col = p.white.scale(0.55 + 0.45 * lam);
                } else if hash(o.cx + dx, y, 3) % 100 < 4 {
                    // The enamel's white flecks.
                    col = col.lerp(p.white, 0.7);
                }
                let glow = (t * t * 0.8 + 0.1 * u.abs()) * breath;
                col = col.lerp(p.ember, glow.clamp(0.0, 0.85));
                if dx.abs() == hx {
                    col = col.scale(0.78);
                }
                px.set(o.cx + dx, y, col);
            }
        }
        // Two ears at the lip.
        let ey = o.cy + rows / 7;
        for side in [-1, 1] {
            let x = o.cx + side * (o.hw + 1);
            px.set(x, ey, p.white.scale(0.8));
            px.set(x + side, ey + 1, p.white.scale(0.6));
            px.set(x, ey + 2, p.white.scale(0.7));
        }
        // The mouth, and the pozole in it.
        let rx = (o.hw as f32 * 0.93).round();
        let ry = o.ry as f32;
        for dy in -o.ry..=o.ry {
            let half =
                (rx * (1.0 - (dy as f32 / (ry + 0.5)).powi(2)).max(0.0).sqrt()).round() as i32;
            for dx in -half..=half {
                let inner = (dx as f32 / (rx - 1.2)).powi(2) + (dy as f32 / (ry - 0.7)).powi(2);
                let col = if inner <= 1.0 {
                    let wave = 0.5 + 0.5 * (dx as f32 * 0.8 + dy as f32 * 1.7 + self.t * 3.0).sin();
                    let broth = p.red[0].lerp(p.red[1], 0.15 + 0.5 * wave);
                    match hash(dx, dy, 7) % 100 {
                        0..=13 => p.white.scale(0.9),
                        14..=16 => p.leaf,
                        _ => broth,
                    }
                } else if dy < 0 {
                    p.white
                } else {
                    p.white.scale(0.78)
                };
                px.set(o.cx + dx, o.cy + dy, col);
            }
        }
        for b in &self.bubbles {
            let k = b.age / b.life;
            let x = o.cx + (b.x * (rx - 1.5)).round() as i32;
            let y = o.cy + (b.y * (ry - 0.9)).round() as i32;
            if k < 0.8 {
                disc(px, x, y, 0.4 + 1.4 * k, p.red[1].lerp(p.white, 0.3));
                px.set(x, y - 1, p.white);
            } else {
                // It bursts: a ring of drops.
                for (dx, dy) in [(-2, 0), (2, 0), (0, -2), (-1, 1), (1, 1)] {
                    px.set(x + dx, y + dy, p.red[1].lerp(p.white, 0.6));
                }
            }
        }
    }

    /// The comal and the elotes roasting on it.
    fn draw_comal(&self, px: &mut Pixels, c: &Comal) {
        let p = &self.pal;
        let (rx, ry) = (c.rx as f32, c.ry as f32);
        for dy in -c.ry..=c.ry {
            let half =
                (rx * (1.0 - (dy as f32 / (ry + 0.5)).powi(2)).max(0.0).sqrt()).round() as i32;
            for dx in -half..=half {
                // A dark iron top, glowing at its edges where the fire reaches.
                let edge = (dx as f32 / rx).powi(2) + (dy as f32 / (ry + 0.5)).powi(2);
                let mut col = p.iron[0].lerp(p.ember, (edge - 0.55).max(0.0) * 0.9);
                if edge > 0.85 {
                    col = p.iron[1].lerp(p.ember, 0.35);
                }
                px.set(c.cx + dx, c.cy + dy, col);
            }
        }
        // Its slab, lit from below.
        for dx in -c.rx..=c.rx {
            let u = dx as f32 / rx;
            let front = c.cy + (ry * (1.0 - u * u).max(0.0).sqrt()).round() as i32;
            for k in 1..=c.thick {
                let glow = 0.25 + 0.5 * k as f32 / c.thick as f32;
                px.set(c.cx + dx, front + k, p.iron[2].lerp(p.ember, glow * 0.7));
            }
        }
        for e in &self.elotes {
            self.draw_elote(px, c, e);
        }
    }

    fn draw_elote(&self, px: &mut Pixels, c: &Comal, e: &Elote) {
        let p = &self.pal;
        let len = ((c.rx as f32 * 0.55) as i32).max(5);
        let thick = ((c.ry as f32 * 0.8) as i32).clamp(2, 4);
        let x0 = c.cx + (e.fx * (c.rx as f32 - len as f32 / 2.0 - 1.0)).round() as i32 - len / 2;
        let y0 = c.cy + (e.fy * (c.ry as f32 - 1.0)).round() as i32;
        // It is turned now and then, and chars where it lay.
        let turn = (self.t / 6.0 + e.phase).floor() as i32;
        let char_pct = 12 + (8.0 * (self.t * 0.4 + e.phase).sin()) as i32;
        let husk = len / 3;
        for i in 0..len {
            let lean = e.slope * (2 * i - len) / (2 * len);
            for r in 0..thick {
                let (x, y) = (x0 + i, y0 + lean + r - thick / 2);
                let col = if i < husk {
                    // The husks, peeled back.
                    p.husk[((i + r) & 1) as usize]
                } else if hash(i, r + turn * 7, 11) % 100 < char_pct as u32 {
                    p.char
                } else if (i + r + turn) & 1 == 0 {
                    p.corn[0]
                } else {
                    p.corn[1]
                };
                // The underside is in shadow.
                let col = if r == thick - 1 { col.scale(0.7) } else { col };
                px.set(x, y, col);
            }
        }
    }

    fn draw_bengala(&self, px: &mut Pixels, b: &Bengala, lit: bool) {
        let p = &self.pal;
        let frac = if lit {
            (self.t / b.period + b.phase).fract()
        } else {
            0.0
        };
        let (tx, ty) = b.tip(frac);
        let (tx, ty) = (tx.round() as i32, ty.round() as i32);
        line(px, b.x, b.ground, tx, ty, p.plata.scale(0.75));
        if lit && frac >= 0.04 {
            // The burning end.
            px.set(tx, ty, rgb(0xfffbe8));
            px.set(tx, ty - 1, rgb(0xffd23b));
        } else {
            px.set(tx, ty, p.plata.scale(0.45));
        }
    }

    fn draw_banderita(&self, ctx: &Ctx, px: &mut Pixels, b: &Banderita) {
        let p = &self.pal;
        let (fw, fh, stick) = [(4, 3, 5), (6, 4, 8), (9, 5, 11)][b.size];
        let dim = b.depth as f32 * DIM;
        // The flag flies from the stick's top.
        let top = b.feet - stick;
        for y in top..=b.feet {
            px.set(b.x, y, ctx.fade(p.wood, dim));
        }
        px.set(b.x, top - 1, ctx.fade(p.gold, dim));
        for i in 0..fw {
            let wave = ((self.t * 4.5 + i as f32 * 0.8 + b.phase).sin() * 0.9 * (i as f32 + 1.0)
                / fw as f32)
                .round() as i32;
            let stripe = (i * 3 / fw).min(2);
            let col = [p.green[1], p.white, p.red[1]][stripe as usize];
            for y in 0..fh {
                let mut c = col;
                if stripe == 1 && fh >= 4 && y == fh / 2 && i == fw / 2 {
                    c = p.brown;
                }
                px.set(b.x + i + 1, top + y + wave, ctx.fade(c, dim));
            }
        }
    }
}
