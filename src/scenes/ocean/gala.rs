//! The holidays that put trajineras on the sea: Navidad and the Fiestas
//! Patrias.
//!
//! Boats of Xochimilco drift across the surface in a line, all at the pace
//! of the same slow current, each painted its own way with a woman's name on
//! its arch and hung with the holiday's decorations (`trajinera.rs`); buoys
//! stand out on the water carrying its emblem; and the sea's own fish put on
//! the holiday's colors. A day and night go round while the holiday lasts:
//! by day the paint shows, and as it darkens the stars come out, the lights
//! glow and everything lit shines again, upside down, in the water.
//!
//! Everything here is drawn from its own generator, so the sea of an
//! ordinary day is untouched, and how much there is follows the holiday's
//! level. This file holds what both holidays share; what is special to each
//! is in `navidad.rs` and `patrias.rs`.

use std::f32::consts::TAU;

use super::art::{self, Role, Species};
use super::cohetes::Cohetes;
use super::sea::{Sea, hash};
use super::tank::mul;
use super::trajinera::{self, Boat, Buoy, Lights, REACH, Shape};
use super::{Dice, Rng};
use super::{navidad, patrias};
use crate::color::{self, Color};
use crate::fiesta::{Fiesta, Kind};
use crate::scene::Ctx;

/// Seconds a whole day and night take, and how far into it the sea starts,
/// as a fraction: just before dusk.
const DAY: f32 = 120.0;
const START: f32 = 0.1;

/// The colors of a holiday that a theme changes: the boats' paint, the
/// lights, and the few colors the rest is drawn in.
#[derive(Clone, Copy)]
pub struct Paint {
    /// Papel de china, the colors boats and piñatas are painted in.
    pub boats: [Color; 6],
    /// The foquitos, in the order a string repeats them.
    pub bulbs: [Color; 6],
    /// Letters on a dark arch, and on a light one.
    pub ink: [Color; 2],
    pub cord: Color,
    pub wood: Color,
    pub straw: Color,
    pub gold: Color,
    pub rojo: Color,
    pub verde: Color,
    pub blanco: Color,
    pub plata: Color,
    /// The holiday it is for, the flag's colors in order (verde, blanco,
    /// rojo), the águila's café and the nopal's green.
    pub kind: Kind,
    pub bandera: [Color; 3],
    pub cafe: Color,
    pub nopal: Color,
}

// Navidad: rojo nochebuena, verde pino, dorado, plata and blanco; the
// foquitos, and the papel de china of the piñatas, which paints the boats.
const ROJO: Color = art::rgb(200, 16, 46);
const PINO: Color = art::rgb(31, 111, 58);
const DORADO: Color = art::rgb(242, 182, 50);
const PLATA: Color = art::rgb(201, 204, 214);
const BLANCO: Color = art::rgb(244, 241, 234);
const TINTA: Color = art::rgb(30, 28, 58);
const FOQUITOS: [Color; 6] = [
    art::rgb(255, 59, 59),
    art::rgb(59, 209, 111),
    art::rgb(59, 123, 255),
    art::rgb(255, 210, 59),
    art::rgb(255, 95, 180),
    art::rgb(255, 140, 26),
];
const PAPEL: [Color; 6] = [
    art::rgb(228, 0, 124),
    art::rgb(255, 210, 0),
    art::rgb(255, 140, 0),
    art::rgb(0, 170, 184),
    art::rgb(112, 40, 160),
    art::rgb(140, 200, 40),
];
const CANELA: Color = art::rgb(139, 74, 30);
const CANA: Color = art::rgb(201, 178, 107);
const CORD: Color = art::rgb(24, 84, 44);

// Fiestas Patrias: verde bandera, blanco and rojo, the águila's café, the
// nopal's green and oro.
const VERDE_B: Color = art::rgb(0, 104, 71);
const ROJO_B: Color = art::rgb(206, 17, 38);
const CAFE: Color = art::rgb(122, 74, 30);
const NOPAL: Color = art::rgb(79, 138, 42);
const ORO: Color = art::rgb(212, 160, 23);

fn hsv(c: Color) -> Option<(f32, f32, f32)> {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => color::ansi_rgb(n),
        Color::Reset => return None,
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    let h = if d <= 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    Some((h, if max > 0.0 { d / max } else { 0.0 }, max))
}

/// The theme's colorful accent whose hue is nearest `hue`, if it is close.
fn by_hue(colors: &[Color], hue: f32) -> Option<Color> {
    colors
        .iter()
        .copied()
        .filter_map(|c| {
            let (h, s, v) = hsv(c)?;
            let d = (h - hue).abs() % 360.0;
            (s >= 0.3 && v >= 0.35).then_some((d.min(360.0 - d), c))
        })
        .filter(|&(d, _)| d <= 45.0)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

impl Paint {
    /// The paint of the holiday on `ctx`.
    pub fn new(ctx: &Ctx) -> Paint {
        let mut p = Paint::basic(ctx);
        if ctx.fiesta.is(Kind::Patrias) {
            // The flag's own greens and reds, boats and lights in them.
            if ctx.theme.native {
                (p.verde, p.rojo, p.gold) = (VERDE_B, ROJO_B, ORO);
            }
            p.bandera = [p.verde, p.blanco, p.rojo];
            p.boats = [p.verde, p.blanco, p.rojo, p.gold, p.verde, p.rojo];
            p.bulbs = [p.verde, p.blanco, p.rojo, p.verde, p.blanco, p.rojo];
        }
        p
    }

    fn basic(ctx: &Ctx) -> Paint {
        let native = Paint {
            boats: PAPEL,
            bulbs: FOQUITOS,
            ink: [BLANCO, TINTA],
            cord: CORD,
            wood: CANELA,
            straw: CANA,
            gold: DORADO,
            rojo: ROJO,
            verde: PINO,
            blanco: BLANCO,
            plata: PLATA,
            kind: ctx.fiesta.kind,
            bandera: [VERDE_B, BLANCO, ROJO_B],
            cafe: CAFE,
            nopal: NOPAL,
        };
        if ctx.theme.native {
            return native;
        }
        // Under a theme the boats and the bulbs wear its accents, and the
        // reds, greens and golds are the nearest of them in hue.
        let t = ctx.theme;
        let near = |hue: f32, i: usize, native: Color| {
            by_hue(&t.colors, hue).unwrap_or_else(|| ctx.pick(i, native))
        };
        let (light, dark) = if t.fg.luma() >= t.bg.luma() {
            (t.fg, t.bg)
        } else {
            (t.bg, t.fg)
        };
        Paint {
            boats: [0, 1, 2, 3, 4, 5].map(|i| ctx.pick(i, PAPEL[i])),
            bulbs: [0, 1, 2, 3, 4, 5].map(|i| ctx.pick(i + 3, FOQUITOS[i])),
            ink: [light, dark],
            cord: dark.lerp(near(140.0, 1, CORD), 0.5),
            wood: dark.lerp(light, 0.3),
            straw: near(50.0, 2, CANA),
            gold: near(45.0, 3, DORADO),
            rojo: near(355.0, 0, ROJO),
            verde: near(140.0, 1, PINO),
            blanco: light,
            plata: light.lerp(dark, 0.25),
            kind: ctx.fiesta.kind,
            bandera: [near(140.0, 1, VERDE_B), light, near(355.0, 0, ROJO_B)],
            cafe: near(30.0, 2, CAFE),
            nopal: near(110.0, 1, NOPAL),
        }
    }

    /// The colors of a fish of the holiday: `B` body, `D` fins, `L` belly,
    /// `S` spots and `F` tail. On Navidad one coat has a red body and green
    /// fins, the other the other way round; on the Patrias a fish wears the
    /// flag, verde, blanco and rojo from its tail to its nose.
    pub fn coat(&self, school: bool, variant: usize) -> [Color; 5] {
        if self.kind == Kind::Patrias {
            let (fin, tail) = if school {
                (mul(self.verde, 0.65), mul(self.verde, 0.8))
            } else {
                (self.gold, self.gold)
            };
            return [self.verde, fin, self.blanco, self.rojo, tail];
        }
        let (body, fin) = if variant.is_multiple_of(2) {
            (self.rojo, self.verde)
        } else {
            (self.verde, self.rojo)
        };
        [body, fin, body.lerp(self.blanco, 0.55), self.gold, fin]
    }
}

/// How dark it is `t` ticks in, 0 by day and 1 at night. The more of the
/// holiday has come, the longer the night.
pub fn cycle(t: u32, level: f32) -> f32 {
    let x = (t as f32 / (20.0 * DAY) + START).fract();
    let d = (x - 0.5).abs() * 2.0;
    let k = ((0.6 + 0.15 * level - d) / 0.3).clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

/// Something dropped from a boat: it sinks, sways, and comes to rest on the
/// sand for a while.
pub struct Sinker {
    pub x: f32,
    pub y: f32,
    vy: f32,
    phase: f32,
    /// Ticks it stays on the bottom, once it is there.
    pub life: u16,
    /// Which of the holiday's colors it is.
    pub hue: u8,
    pub rest: bool,
}

/// The holiday's dress for the sea.
pub struct Gala {
    pub on: bool,
    pub kind: Kind,
    /// How far the holiday has come, 0 to 1.
    pub level: f32,
    /// Whether the posadas are on, December 16 to 24.
    pub posadas: bool,
    /// The holiday's own dice, so an ordinary sea's are left alone.
    pub rng: Rng,
    /// How dark it is, 0 by day to 1 at night.
    pub dusk: f32,
    pub shape: Option<Shape>,
    pub boats: Vec<Boat>,
    pub buoys: Vec<Buoy>,
    /// How big the buoys' toppers are, and how tall their masts.
    top: i32,
    mast: i32,
    /// The length of the loop the boats go round, in pixels.
    span: f32,
    /// Plants on the sand, as (column, sand row), and what falls from the
    /// boats to sink and settle there.
    pub plants: Vec<(i32, i32)>,
    pub sinkers: Vec<Sinker>,
    /// The most sinkers sinking or resting at once.
    most: usize,
    /// Whether there are fireworks, and the ones in the air.
    pub fireworks: bool,
    pub works: Cohetes,
}

impl Gala {
    pub fn new(fiesta: Fiesta, seed: u64) -> Gala {
        let on = fiesta.is(Kind::Navidad) || fiesta.is(Kind::Patrias);
        Gala {
            on,
            kind: if on { fiesta.kind } else { Kind::None },
            level: if on {
                fiesta.level.clamp(0.0, 1.0)
            } else {
                0.0
            },
            posadas: fiesta.posadas(),
            rng: Rng::with_seed(seed ^ 0x7472_616a_696e_6572),
            dusk: 0.0,
            shape: None,
            boats: Vec::new(),
            buoys: Vec::new(),
            top: 0,
            mast: 0,
            span: 1.0,
            plants: Vec::new(),
            sinkers: Vec::new(),
            most: 0,
            fireworks: on && (fiesta.ano_nuevo() || fiesta.is(Kind::Patrias)),
            works: Cohetes::default(),
        }
    }

    /// How many boats and buoys there are.
    #[cfg(test)]
    pub fn counts(&self) -> (usize, usize) {
        (self.boats.len(), self.buoys.len())
    }

    /// How many plants and sinkers there are.
    #[cfg(test)]
    pub fn floor_counts(&self) -> (usize, usize) {
        (self.plants.len(), self.sinkers.len())
    }

    /// How many rockets have gone up.
    #[cfg(test)]
    pub fn works_sent(&self) -> u32 {
        self.works.launched
    }

    /// The names written on the arches.
    #[cfg(test)]
    pub fn names(&self) -> Vec<&'static str> {
        self.boats.iter().map(|b| b.name).collect()
    }
}

impl Sea {
    /// Lays out the holiday: the boats in their line, the buoys, and what
    /// is special to the holiday.
    pub fn dress_gala(&mut self) {
        let (w, surf) = (self.w, self.surf);
        let g = &mut self.gala;
        let lvl = g.level;
        g.dusk = cycle(0, lvl);
        g.shape = Shape::of(surf, w);
        let rng = &mut g.rng;
        let current = rng.sign() * rng.rangef(0.045, 0.07);
        if let Some(sh) = g.shape {
            let names: &[&str] = if g.kind == Kind::Patrias {
                &patrias::NAMES
            } else {
                &navidad::NAMES
            };
            let narrowest = names.iter().map(|n| sh.width(n)).min().unwrap_or(1);
            let most = (w / (narrowest + 6)).max(1);
            // Fireworks need sky to be seen in, so fewer boats on those nights.
            let thin = if g.fireworks { 0.75 } else { 1.0 };
            let n = (((w as f32 / 70.0) * (0.6 + 0.7 * lvl) * thin).round() as i32).clamp(1, most);
            // Each boat's name: any that fits, none twice.
            let room = w / n - 6;
            let mut taken = vec![false; names.len()];
            let mut chosen = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let fits: Vec<usize> = (0..names.len())
                    .filter(|&i| !taken[i] && sh.width(names[i]) <= room)
                    .collect();
                let i = if fits.is_empty() {
                    (0..names.len())
                        .filter(|&i| !taken[i])
                        .min_by_key(|&i| sh.width(names[i]))
                        .unwrap_or(0)
                } else {
                    fits[rng.usize(..fits.len())]
                };
                taken[i] = true;
                chosen.push(names[i]);
            }
            let widest = chosen.iter().map(|n| sh.width(n)).max().unwrap_or(1);
            // Past the edge a boat is off the screen whole before it goes round.
            g.span = (w + widest + 2 * REACH + 2).max(n * (widest + 6)) as f32;
            // The piñatas come with the posadas.
            let pinatas = if g.kind == Kind::Navidad {
                ((lvl - 0.3) / 0.7).clamp(0.0, 1.0)
            } else {
                -1.0
            };
            // The flag's three colors paint the boats of the Patrias, the six
            // of papel de china those of Navidad.
            let (shades, near, far) = if g.kind == Kind::Patrias {
                (3, 1, 2)
            } else {
                (6, 2, 4)
            };
            let dir = current.signum();
            for (k, name) in chosen.into_iter().enumerate() {
                let panel = rng.usize(..shades);
                let (hull, trim) = if rng.bool() {
                    ((panel + near) % shades, (panel + far) % shades)
                } else {
                    ((panel + far) % shades, (panel + near) % shades)
                };
                let slot = (k as f32 + rng.rangef(0.0, 0.3)) / n as f32;
                g.boats.push(Boat {
                    x: slot * g.span - (widest / 2 + REACH) as f32,
                    vx: current,
                    dir,
                    name,
                    w: sh.width(name),
                    panel,
                    hull,
                    trim,
                    phase: rng.rangef(0.0, TAU),
                    pinata: (k as f32 + 0.5) / n as f32 <= pinatas || (k == 0 && g.posadas),
                    blink: rng.usize(..3) as u8,
                });
            }
        }
        if surf >= 7 {
            g.top = if surf >= 12 {
                7
            } else if surf >= 9 {
                5
            } else {
                3
            };
            g.mast = (surf - 5 - g.top).clamp(0, 3);
            let two = w >= 150 && lvl >= 0.7;
            let wf = w as f32;
            for k in 0..1 + two as usize {
                let (lo, hi) = if two {
                    (0.1 + 0.45 * k as f32, 0.45 + 0.4 * k as f32)
                } else {
                    (0.1, 0.9)
                };
                g.buoys.push(Buoy {
                    x: rng.rangef(wf * lo, wf * hi) as i32,
                    phase: rng.rangef(0.0, TAU),
                });
            }
        }
        self.dress_floor();
        if self.gala.fireworks {
            self.dress_cohetes();
        }
    }

    /// A tick of the holiday: the day goes round and the boats drift.
    pub fn gala_tick(&mut self) {
        let t = self.t;
        let g = &mut self.gala;
        g.dusk = cycle(t, g.level);
        let (w, span) = (self.w as f32, g.span);
        for b in &mut g.boats {
            b.x += b.vx;
            // Off one side and back on the other.
            let half = b.w as f32 * 0.5;
            if b.x - half > w + REACH as f32 {
                b.x -= span;
            } else if b.x + half < -REACH as f32 {
                b.x += span;
            }
        }
        self.floor_tick();
        if self.gala.fireworks {
            self.cohetes_tick();
        }
    }

    /// Where a boat sits: the column of its left end and the row of the
    /// bottom of its hull, bobbing on the water.
    fn berth(&self, b: &Boat, t: f32) -> (i32, i32) {
        let bob = if (t * 0.05 + b.phase).sin() > 0.55 {
            1
        } else {
            0
        };
        ((b.x - b.w as f32 * 0.5).round() as i32, self.surf - 1 - bob)
    }

    /// Where a buoy's can sits: its column and the row of its bottom.
    fn float(&self, b: &Buoy, t: f32) -> (i32, i32) {
        let bob = if (t * 0.04 + b.phase).sin() > 0.3 {
            1
        } else {
            0
        };
        (b.x, self.surf - 1 - bob)
    }

    /// The light of something lit at (`x`, `y`) in the sky and its
    /// reflection in the water, upside down below the surface, broken by the
    /// ripples and dimmer the deeper it goes. `glow` is 0 for anything that
    /// is only painted, which shows faintly.
    pub(super) fn mirror(&mut self, x: i32, y: i32, c: Color, glow: f32, t: f32) {
        let ry = 2 * self.surf - 1 - y;
        let depth = ry - self.surf;
        // Lights carry far down, painted wood only a little way.
        let reach = if glow > 0.0 { 15 } else { 5 };
        if y >= self.surf || !(0..=reach).contains(&depth) {
            return;
        }
        let dusk = self.gala.dusk;
        let tick = (t * 0.25) as i32;
        let h = hash(x, ry + tick * 7, 0x6761_6c61);
        let sway = 0.6 + depth as f32 * 0.1;
        let wave = ((t * 0.12 + ry as f32 * 1.7 + x as f32 * 0.3).sin() * sway).round() as i32;
        let k = 1.0 - depth as f32 / (reach + 1) as f32;
        let a = if glow > 0.0 {
            // Glints rather than a wash: whole pixels that come and go.
            if h % (reach as u32 + 3) < depth as u32 {
                return;
            }
            (0.2 + 0.7 * dusk * glow) * k
        } else {
            if h.is_multiple_of(4) {
                return;
            }
            (0.34 - 0.16 * dusk) * k
        };
        self.blend(x + wave, ry, c, a);
    }

    /// The halo a lit bulb throws on the air around it.
    pub(super) fn halo(&mut self, x: i32, y: i32, c: Color, glow: f32) {
        let a = 0.17 * self.gala.dusk * glow;
        if a < 0.01 {
            return;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            self.glow(x + dx, y + dy, c, a);
        }
        for (dx, dy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
            self.glow(x + dx, y + dy, c, a * 0.5);
        }
    }

    /// The night: stars coming out, and the boats' and buoys' lights
    /// glowing in the air and shining in the water.
    pub fn paint_eve(&mut self, t: f32) {
        let (w, top, dusk) = (self.w, self.surf, self.gala.dusk);
        let paint = self.pal.paint;
        if dusk > 0.02 && self.set.water && top > 2 {
            for y in 0..top * 2 / 3 {
                for x in 0..w {
                    let h = hash(x, y, 0x5eed_57a2);
                    if !h.is_multiple_of(53) {
                        continue;
                    }
                    let tw = (t * 0.05 + (h >> 8) as f32).sin() * 0.5 + 0.5;
                    let a = (0.3 + 0.65 * tw) * (1.0 - y as f32 / top as f32) * dusk;
                    self.blend(x, y, paint.blanco, a);
                }
            }
        }
        if self.gala.fireworks {
            self.paint_cohetes(t);
        }
        let lights = Lights::of(self.gala.level);
        if let Some(sh) = self.gala.shape {
            for i in 0..self.gala.boats.len() {
                let b = self.gala.boats[i];
                let (lx, by) = self.berth(&b, t);
                if lx + b.w < -REACH || lx > w + REACH {
                    continue;
                }
                trajinera::plot(&b, &sh, &paint, lights, t, &mut |dx, u, c, g| {
                    self.mirror(lx + dx, by - u, c, g, t);
                    if g > 0.0 {
                        self.halo(lx + dx, by - u, c, g);
                    }
                });
            }
        }
        for i in 0..self.gala.buoys.len() {
            let b = self.gala.buoys[i];
            let (bx, by) = self.float(&b, t);
            let (kind, mast, size) = (self.gala.kind, self.gala.mast, self.gala.top);
            trajinera::plot_buoy(&paint, kind, mast, size, t, &mut |dx, u, c, g| {
                self.mirror(bx + dx, by - u, c, g, t);
                if g > 0.0 {
                    self.halo(bx + dx, by - u, c, g);
                }
            });
        }
    }

    /// Plants the floor and makes room for what falls from the boats.
    fn dress_floor(&mut self) {
        let w = self.w;
        let sand = &self.sand_top;
        let g = &mut self.gala;
        let lvl = g.level;
        let rng = &mut g.rng;
        let n = ((w as f32 / 12.0) * (0.3 + 0.7 * lvl)).round().max(1.0) as usize;
        for _ in 0..n {
            let x = rng.range(2, w - 2);
            g.plants.push((x, sand[x.clamp(0, w - 1) as usize]));
        }
        g.most = ((w as f32 / 14.0) * (0.3 + 0.7 * lvl)) as usize + 2;
        g.sinkers.reserve(g.most + 1);
    }

    /// A tick of the floor: things drop from the boats, sink and settle.
    fn floor_tick(&mut self) {
        let (w, surf) = (self.w, self.surf);
        let wf = w as f32;
        let t = self.t as f32;
        let sand = &self.sand_top;
        let g = &mut self.gala;
        g.sinkers.retain_mut(|e| {
            if e.rest {
                e.life = e.life.saturating_sub(1);
                return e.life > 0;
            }
            e.y += e.vy;
            e.x += (t * 0.05 + e.phase).sin() * 0.05;
            let bed = sand[(e.x.round() as i32).clamp(0, w - 1) as usize] as f32 - 2.0;
            if e.y >= bed {
                e.y = bed;
                e.rest = true;
            }
            true
        });
        // Now and then one slips from a boat.
        let rate = (0.004 + 0.012 * g.level) * wf / 100.0;
        if g.sinkers.len() < g.most && g.rng.chance(rate) {
            let rng = &mut g.rng;
            let x = if g.boats.is_empty() {
                rng.rangef(0.0, wf)
            } else {
                let b = g.boats[rng.usize(..g.boats.len())];
                b.x + rng.rangef(-0.4, 0.4) * b.w as f32
            };
            g.sinkers.push(Sinker {
                x: x.clamp(0.0, wf - 1.0),
                y: surf as f32 + 1.0,
                vy: rng.rangef(0.04, 0.09),
                phase: rng.rangef(0.0, TAU),
                life: rng.range(500, 1600) as u16,
                hue: rng.usize(..4) as u8,
                rest: false,
            });
        }
    }

    /// What lies on the sea floor, and the castle's dress.
    pub fn draw_gala_floor(&mut self, t: f32) {
        for i in 0..self.gala.plants.len() {
            let (x, y) = self.gala.plants[i];
            match self.gala.kind {
                Kind::Patrias => self.plant_patrias(x, y),
                _ => self.plant_navidad(x, y),
            }
        }
        for i in 0..self.gala.sinkers.len() {
            let e = &self.gala.sinkers[i];
            if e.rest {
                let (x, y, hue) = (e.x.round() as i32, e.y.round() as i32, e.hue);
                let shine = ((t * 0.07 + e.phase).sin() > 0.9) && e.life > 20;
                self.sinker(x, y, hue, shine);
            }
        }
        match self.gala.kind {
            Kind::Patrias => self.dress_castle_patrias(t),
            _ => self.dress_castle_navidad(t),
        }
    }

    /// The boats and the buoys, and anything falling from them.
    pub fn draw_gala(&mut self, t: f32) {
        let (w, paint) = (self.w, self.pal.paint);
        let lights = Lights::of(self.gala.level);
        for i in 0..self.gala.buoys.len() {
            let b = self.gala.buoys[i];
            let (bx, by) = self.float(&b, t);
            let (kind, mast, size) = (self.gala.kind, self.gala.mast, self.gala.top);
            trajinera::plot_buoy(&paint, kind, mast, size, t, &mut |dx, u, c, _| {
                self.put(bx + dx, by - u, c);
            });
        }
        if let Some(sh) = self.gala.shape {
            for i in 0..self.gala.boats.len() {
                let b = self.gala.boats[i];
                let (lx, by) = self.berth(&b, t);
                if lx + b.w < -REACH || lx > w + REACH {
                    continue;
                }
                trajinera::plot(&b, &sh, &paint, lights, t, &mut |dx, u, c, _| {
                    self.put(lx + dx, by - u, c);
                });
            }
        }
        for i in 0..self.gala.sinkers.len() {
            let e = &self.gala.sinkers[i];
            if !e.rest {
                let (x, y, hue) = (e.x.round() as i32, e.y.round() as i32, e.hue);
                self.sinker(x, y, hue, false);
            }
        }
    }

    /// What falls from the boats: esferas or pieces of papel picado.
    fn sinker(&mut self, x: i32, y: i32, hue: u8, shine: bool) {
        match self.gala.kind {
            Kind::Patrias => self.sinker_patrias(x, y, hue, shine),
            _ => self.sinker_navidad(x, y, hue, shine),
        }
    }

    /// The paint of a fish of the holiday, when `sp` is one.
    pub fn gala_coat(&self, sp: &Species, k: usize) -> Option<[Color; 5]> {
        let mine = match self.gala.kind {
            Kind::Navidad => navidad::is_fish(sp),
            Kind::Patrias => patrias::is_fish(sp),
            _ => false,
        };
        (self.gala.on && mine).then(|| self.pal.paint.coat(sp.role == Role::School, k))
    }

    /// One of the holiday's fish, when the roll comes up for them: more of
    /// them as the holiday comes.
    pub fn holiday_fish(&mut self, school: bool) -> Option<&'static Species> {
        if !self.gala.on {
            return None;
        }
        let lvl = self.gala.level;
        let p = if school {
            0.3 + 0.3 * lvl
        } else {
            0.15 + 0.25 * lvl
        };
        let kind = self.gala.kind;
        self.gala.rng.chance(p).then(|| {
            if kind == Kind::Patrias {
                patrias::fish(school)
            } else {
                navidad::fish(school)
            }
        })
    }
}
