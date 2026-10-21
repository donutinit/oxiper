//! constellation: drifting stars that link up when close.
//!
//! Everything is simulated in "units" of one cell width on both axes, so a
//! cell is 1×2 units and braille dots (2×4 per cell) are 0.5×0.5 units. The
//! star field extends one link distance past every edge, so stars drift in
//! and out of view (with their links) instead of popping when they wrap.
//!
//! On Día de Muertos the sky takes the holiday's colors (marigold, bone,
//! rosa mexicano, turquoise and violet stars, links leaning violet, marigold
//! meteors) and now and then a handful of stars leave their drift, glide
//! together and draw a calavera de azúcar, La Catrina, a cempasúchil, a
//! monarch or a veladora with their links; a spark traces it, its icing
//! and flowers light up, and after a while the stars let go and drift on.
//! Figures come more often as the holiday builds up, up to three at once on
//! November 2 (see `muertos`).

mod muertos;

use std::f32::consts::TAU;

use crate::color::{Color, mix_oklab, rgb};
use crate::render::{BOLD, Braille, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "constellation",
    aliases: &["stars"],
    about: "drifting stars that link up when close",
    options: &[
        Opt {
            key: "density",
            default: Val::F(1.0),
            help: "Star density multiplier (the count scales with the screen area)",
        },
        Opt {
            key: "link_distance",
            default: Val::F(11.0),
            help: "Stars closer than this many cells get linked",
        },
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Drift speed multiplier",
        },
        Opt {
            key: "twinkle",
            default: Val::F(1.0),
            help: "Twinkle strength (0 = steady stars)",
        },
        Opt {
            key: "shooting_stars",
            default: Val::F(4.0),
            help: "Shooting stars per minute (0 = none)",
        },
        Opt {
            key: "max_links",
            default: Val::I(3),
            help: "Most lines per star, nearest first (0 = no lines)",
        },
    ],
    make: |ctx| Box::new(Constellation::new(ctx)),
};

/// Cells per star at density 1.
const CELLS_PER_STAR: f32 = 24.0;
const MAX_STARS: usize = 6000;
const MAX_METEORS: usize = 3;
/// The dimmest a line gets; fainter links thin out into sparse dots instead.
const LINK_FLOOR: f32 = 0.45;
/// Link strength below which lines start thinning out.
const DISSOLVE: f32 = 0.3;

/// Spectral tints under `classic`, blue-white to orange, with how common each is.
const NATIVE_TINTS: [u32; 7] = [
    0xa9bcff, 0xc8d6ff, 0xf2f3ff, 0xfff3e6, 0xffe4bd, 0xffcf96, 0xffad7a,
];
const TINT_WEIGHTS: [u32; 7] = [12, 22, 28, 16, 10, 8, 4];
const NATIVE_LINK: u32 = 0x6f86d6;

#[derive(Clone, Copy)]
struct Star {
    /// Position in field units (screen x = x - margin).
    x: f32,
    y: f32,
    heading: f32,
    speed: f32,
    turn: f32,
    /// 0 = faint dust … 3 = bright star.
    kind: u8,
    tint: u8,
    base: f32,
    p1: f32,
    f1: f32,
    p2: f32,
    f2: f32,
    /// Brightness this frame.
    b: f32,
    links: u8,
    /// Día de Muertos: the figure that called this star (0 = none, or
    /// `muertos::LETTING_GO`), how visible its ordinary links are, how much
    /// it shines as part of a figure, and its figure's hue.
    fig: u8,
    gain: f32,
    glow: f32,
    hue: u8,
}

#[derive(Clone, Copy, Default)]
struct Meteor {
    on: bool,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    speed: f32,
    age: f32,
    life: f32,
    len: f32,
}

pub struct Constellation {
    rng: fastrand::Rng,
    density: f32,
    link: f32,
    speed: f32,
    twinkle: f32,
    meteor_rate: f32,
    max_links: u8,
    /// Screen size in units.
    sw: f32,
    sh: f32,
    margin: f32,
    fw: f32,
    fh: f32,
    stars: Vec<Star>,
    meteors: [Meteor; MAX_METEORS],
    gw: usize,
    gh: usize,
    /// Counting-sort buckets: start index of every grid cell in `order`.
    starts: Vec<u32>,
    order: Vec<u32>,
    cell_of: Vec<u32>,
    pairs: Vec<(f32, u32, u32)>,
    br: Braille,
    tints: [Color; 7],
    link_tints: [Color; 7],
    meteor_col: Color,
    meteor_tail: Color,
    holiday: muertos::Muertos,
}

impl Constellation {
    fn new(ctx: &Ctx) -> Constellation {
        let o = ctx.opts;
        let link = o.f("link_distance").clamp(1.0, 200.0);
        Constellation {
            rng: ctx.rng(),
            density: o.f("density").clamp(0.0, 20.0),
            link,
            speed: o.f("speed").clamp(0.0, 50.0),
            twinkle: o.f("twinkle").clamp(0.0, 1.0),
            meteor_rate: o.f("shooting_stars").clamp(0.0, 600.0),
            max_links: o.i("max_links").clamp(0, 32) as u8,
            sw: 0.0,
            sh: 0.0,
            margin: link,
            fw: 1.0,
            fh: 1.0,
            stars: Vec::new(),
            meteors: [Meteor::default(); MAX_METEORS],
            gw: 1,
            gh: 1,
            starts: Vec::new(),
            order: Vec::new(),
            cell_of: Vec::new(),
            pairs: Vec::new(),
            br: Braille::new(1, 1),
            tints: [Color::Reset; 7],
            link_tints: [Color::Reset; 7],
            meteor_col: Color::Reset,
            meteor_tail: Color::Reset,
            holiday: muertos::Muertos::new(ctx.seed),
        }
    }

    fn new_star(&mut self, x: f32, y: f32) -> Star {
        let r = &mut self.rng;
        let roll = r.f32();
        let kind = if roll < 0.52 {
            0
        } else if roll < 0.8 {
            1
        } else if roll < 0.94 {
            2
        } else {
            3
        };
        let base = match kind {
            0 => 0.4 + 0.2 * r.f32(),
            1 => 0.5 + 0.25 * r.f32(),
            2 => 0.7 + 0.2 * r.f32(),
            _ => 0.88 + 0.12 * r.f32(),
        };
        let total: u32 = TINT_WEIGHTS.iter().sum();
        let mut pick = r.u32(0..total);
        let mut tint = 0;
        for (i, &wt) in TINT_WEIGHTS.iter().enumerate() {
            if pick < wt {
                tint = i;
                break;
            }
            pick -= wt;
        }
        Star {
            x,
            y,
            heading: r.f32() * TAU,
            // Units per second: a slow, uneven drift.
            speed: 0.35 + 0.9 * r.f32(),
            turn: (r.f32() - 0.5) * 0.25,
            kind,
            tint: tint as u8,
            base,
            p1: r.f32() * TAU,
            f1: 0.6 + 2.2 * r.f32(),
            p2: r.f32() * TAU,
            f2: 1.7 + 3.5 * r.f32(),
            b: base,
            links: 0,
            fig: 0,
            gain: 1.0,
            glow: 0.0,
            hue: 0,
        }
    }

    fn spawn_meteor(&mut self) {
        let Some(slot) = self.meteors.iter().position(|m| !m.on) else {
            return;
        };
        let r = &mut self.rng;
        // Mostly sideways and down, like the ones you notice.
        let ang = (0.25 + 0.55 * r.f32()) * if r.bool() { 1.0 } else { -1.0 };
        let left_to_right = r.bool();
        let dx = ang.cos() * if left_to_right { 1.0 } else { -1.0 };
        let dy = ang.sin().abs();
        let speed = 45.0 + 35.0 * r.f32();
        let life = 0.7 + 0.7 * r.f32();
        let travel = speed * life;
        // Start where the streak will mostly cross the screen.
        let x = if left_to_right {
            r.f32() * self.sw * 0.7 - travel * 0.15
        } else {
            self.sw * 0.3 + r.f32() * self.sw * 0.7 + travel * 0.15
        };
        let y = r.f32() * self.sh * 0.55;
        self.meteors[slot] = Meteor {
            on: true,
            x,
            y,
            dx,
            dy,
            speed,
            age: 0.0,
            life,
            len: 10.0 + 12.0 * r.f32(),
        };
    }

    /// Fills `pairs` with every pair of stars closer than the link distance.
    fn find_pairs(&mut self) {
        self.pairs.clear();
        let n = self.stars.len();
        if n < 2 || self.max_links == 0 {
            return;
        }
        let (gw, gh) = (self.gw, self.gh);
        let inv = 1.0 / self.link;
        self.starts.iter_mut().for_each(|s| *s = 0);
        self.cell_of.clear();
        let mut linkable = 0;
        for s in &self.stars {
            // Faint dust never links; only real stars form constellations.
            // Stars drawing a holiday figure link only within it.
            if s.kind == 0 || s.gain <= 0.0 {
                self.cell_of.push(u32::MAX);
                continue;
            }
            let cx = ((s.x * inv) as usize).min(gw - 1);
            let cy = ((s.y * inv) as usize).min(gh - 1);
            let c = cy * gw + cx;
            self.cell_of.push(c as u32);
            self.starts[c + 1] += 1;
            linkable += 1;
        }
        for i in 0..gw * gh {
            self.starts[i + 1] += self.starts[i];
        }
        self.order.resize(linkable, 0);
        // Fill using a running cursor; `starts[c]` is restored afterwards.
        for (i, &c) in self.cell_of.iter().enumerate() {
            if c == u32::MAX {
                continue;
            }
            let slot = &mut self.starts[c as usize];
            self.order[*slot as usize] = i as u32;
            *slot += 1;
        }
        for c in (1..=gw * gh).rev() {
            self.starts[c] = self.starts[c - 1];
        }
        self.starts[0] = 0;

        let l2 = self.link * self.link;
        // Half neighborhood so every pair is seen once.
        const NB: [(isize, isize); 4] = [(1, 0), (-1, 1), (0, 1), (1, 1)];
        for cy in 0..gh {
            for cx in 0..gw {
                let c = cy * gw + cx;
                let (a0, a1) = (self.starts[c] as usize, self.starts[c + 1] as usize);
                for ai in a0..a1 {
                    let i = self.order[ai];
                    let si = self.stars[i as usize];
                    for &bj in &self.order[ai + 1..a1] {
                        let sj = &self.stars[bj as usize];
                        let d2 = (si.x - sj.x).powi(2) + (si.y - sj.y).powi(2);
                        if d2 < l2 {
                            self.pairs.push((d2, i, bj));
                        }
                    }
                    for &(ox, oy) in &NB {
                        let (nx, ny) = (cx as isize + ox, cy as isize + oy);
                        if nx < 0 || ny < 0 || nx >= gw as isize || ny >= gh as isize {
                            continue;
                        }
                        let nc = ny as usize * gw + nx as usize;
                        for bi in self.starts[nc] as usize..self.starts[nc + 1] as usize {
                            let j = self.order[bi];
                            let sj = &self.stars[j as usize];
                            let d2 = (si.x - sj.x).powi(2) + (si.y - sj.y).powi(2);
                            if d2 < l2 {
                                self.pairs.push((d2, i, j));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Scene for Constellation {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.holiday.cancel(&mut self.stars);
        let (sw, sh) = (w.max(1) as f32, h.max(1) as f32 * 2.0);
        let m = self.margin;
        let (fw, fh) = (sw + 2.0 * m, sh + 2.0 * m);
        // Keep existing stars where they are, relative to the field.
        let (kx, ky) = (fw / self.fw, fh / self.fh);
        for s in &mut self.stars {
            s.x = (s.x * kx).clamp(0.0, fw - 0.001);
            s.y = (s.y * ky).clamp(0.0, fh - 0.001);
        }
        self.sw = sw;
        self.sh = sh;
        self.fw = fw;
        self.fh = fh;
        let want = ((fw * fh * 0.5 / CELLS_PER_STAR) * self.density).round() as usize;
        let want = want.min(MAX_STARS);
        self.stars.truncate(want);
        while self.stars.len() < want {
            let (x, y) = (self.rng.f32() * fw, self.rng.f32() * fh);
            let s = self.new_star(x, y);
            self.stars.push(s);
        }
        self.gw = ((fw / self.link).ceil() as usize).max(1);
        self.gh = ((fh / self.link).ceil() as usize).max(1);
        self.starts.clear();
        self.starts.resize(self.gw * self.gh + 1, 0);
        self.order.clear();
        self.order.reserve(want);
        self.cell_of.clear();
        self.cell_of.reserve(want);
        self.pairs.clear();
        self.pairs.reserve(want * 4);
        self.br.resize(w, h);
    }

    fn retheme(&mut self, ctx: &Ctx) {
        if ctx.theme.native {
            for (i, &hex) in NATIVE_TINTS.iter().enumerate() {
                self.tints[i] = rgb(hex);
                self.link_tints[i] = mix_oklab(rgb(hex), rgb(NATIVE_LINK), 0.6);
            }
            self.meteor_col = rgb(0xf4f6ff);
        } else {
            let g = ctx.ramp(&[]);
            self.tints = [
                g.at(1.0),
                g.at(0.92),
                g.at(0.85),
                ctx.pick(0, Color::Reset),
                ctx.pick(1, Color::Reset),
                ctx.pick(2, Color::Reset),
                ctx.pick(3, Color::Reset),
            ];
            let link = g.at(0.6);
            for i in 0..7 {
                self.link_tints[i] = mix_oklab(self.tints[i], link, 0.5);
            }
            self.meteor_col = g.at(1.0);
        }
        self.meteor_tail = self.meteor_col;
        self.holiday.set(ctx.fiesta, &mut self.stars);
        if self.holiday.on {
            self.holiday.retheme(
                ctx,
                &mut self.tints,
                &mut self.link_tints,
                &mut self.meteor_col,
                &mut self.meteor_tail,
            );
        }
    }

    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        if self.holiday.on {
            let dt = if dt.is_finite() {
                dt.clamp(0.0, 0.25)
            } else {
                0.0
            };
            let lines = self.max_links > 0;
            let (sw, sh, m) = (self.sw, self.sh, self.margin);
            self.holiday
                .tick(&mut self.stars, sw, sh, m, dt, real, lines);
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = if dt.is_finite() {
            dt.clamp(0.0, 0.25)
        } else {
            0.0
        };
        let (fw, fh) = (self.fw, self.fh);
        let v = self.speed * dt;
        for s in &mut self.stars {
            if s.fig != 0 && s.fig != muertos::LETTING_GO {
                // A holiday figure holds it.
                s.p1 = (s.p1 + s.f1 * dt).rem_euclid(TAU);
                s.p2 = (s.p2 + s.f2 * dt).rem_euclid(TAU);
                continue;
            }
            s.heading = (s.heading + s.turn * dt).rem_euclid(TAU);
            s.x = (s.x + s.heading.cos() * s.speed * v).rem_euclid(fw);
            s.y = (s.y + s.heading.sin() * s.speed * v).rem_euclid(fh);
            // rem_euclid can round up to the modulus itself.
            if s.x >= fw {
                s.x = 0.0;
            }
            if s.y >= fh {
                s.y = 0.0;
            }
            s.p1 = (s.p1 + s.f1 * dt).rem_euclid(TAU);
            s.p2 = (s.p2 + s.f2 * dt).rem_euclid(TAU);
        }
        for m in &mut self.meteors {
            if m.on {
                m.age += dt;
                m.x += m.dx * m.speed * dt;
                m.y += m.dy * m.speed * dt;
                if m.age >= m.life {
                    m.on = false;
                }
            }
        }
        // Poisson arrivals; scale the chance with the screen width so a wide
        // screen does not feel emptier than a narrow one.
        let area_k = (self.sw / 100.0).clamp(0.3, 3.0);
        if self.meteor_rate > 0.0 && self.rng.f32() < self.meteor_rate / 60.0 * dt * area_k {
            self.spawn_meteor();
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let tw = self.twinkle;
        for s in &mut self.stars {
            let n = 0.5 + 0.3 * s.p1.sin() + 0.2 * s.p2.sin();
            // Mostly steady, with dips; big stars twinkle less.
            let depth = tw * [0.75, 0.65, 0.55, 0.45][s.kind as usize & 3];
            s.b = s.base * (1.0 - depth * n * n);
            s.links = 0;
        }

        self.find_pairs();
        let max = self.max_links;
        let stars = &mut self.stars;
        self.pairs.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        self.pairs.retain(|&(_, i, j)| {
            let (i, j) = (i as usize, j as usize);
            if stars[i].links < max && stars[j].links < max {
                stars[i].links += 1;
                stars[j].links += 1;
                true
            } else {
                false
            }
        });

        self.br.clear();
        let m = self.margin;
        let inv_l = 1.0 / self.link;
        // Farthest first, so the brighter, closer links win shared cells.
        for &(d2, i, j) in self.pairs.iter().rev() {
            let (a, b) = (&self.stars[i as usize], &self.stars[j as usize]);
            let (x0, y0) = ((a.x - m) * 2.0, (a.y - m) * 2.0);
            let (x1, y1) = ((b.x - m) * 2.0, (b.y - m) * 2.0);
            let (pw, ph) = (self.br.pw() as f32, self.br.ph() as f32);
            if (x0 < 0.0 && x1 < 0.0)
                || (y0 < 0.0 && y1 < 0.0)
                || (x0 >= pw && x1 >= pw)
                || (y0 >= ph && y1 >= ph)
            {
                continue;
            }
            let k = d2.sqrt() * inv_l;
            let fall = (1.0 - k).max(0.0);
            let mut alpha = fall.powf(0.8) * (0.5 + 0.3 * (a.b + b.b));
            if self.holiday.on {
                // Stars called by a figure fade out of the ordinary links,
                // and the sky clears around the figure.
                alpha *= a.gain.min(b.gain) * self.holiday.clear(a.x, a.y, b.x, b.y);
            }
            // Below a floor brightness, links dissolve into ever sparser dots
            // instead of getting darker: on a terminal whose background we do
            // not know, "darker" can end up darker than the background.
            let lit = LINK_FLOOR
                + (0.92 - LINK_FLOOR) * ((alpha - DISSOLVE) / (1.0 - DISSOLVE)).clamp(0.0, 1.0);
            let stride = if alpha >= DISSOLVE {
                1
            } else {
                (DISSOLVE / alpha.max(1e-3)).round() as usize
            };
            if stride > 6 {
                continue;
            }
            let ca = ctx.fade(self.link_tints[a.tint as usize], 1.0 - lit);
            let cb = ctx.fade(self.link_tints[b.tint as usize], 1.0 - lit);
            let n = ((x1 - x0).abs().max((y1 - y0).abs()).ceil() as usize).clamp(1, 4096);
            let inv_n = 1.0 / n as f32;
            for s in (0..=n).step_by(stride) {
                let t = s as f32 * inv_n;
                self.br.setf(
                    x0 + (x1 - x0) * t + 0.5,
                    y0 + (y1 - y0) * t + 0.5,
                    ca.lerp(cb, t),
                );
            }
        }

        for mt in &self.meteors {
            if !mt.on {
                continue;
            }
            let env = (mt.age / 0.12).min(1.0) * ((mt.life - mt.age) / 0.35).clamp(0.0, 1.0);
            let len = mt.len.min(mt.age * mt.speed);
            let (hx, hy) = (mt.x * 2.0, mt.y * 2.0);
            let (tx, ty) = ((mt.x - mt.dx * len) * 2.0, (mt.y - mt.dy * len) * 2.0);
            // Head to tail: bright, then dimmer, then dissolving into sparse dots.
            let n = ((hx - tx).abs().max((hy - ty).abs()).ceil() as usize).clamp(1, 4096);
            for s in 0..=n {
                let f = s as f32 / n as f32;
                let v = env * (1.0 - f).powf(1.6);
                if v < DISSOLVE {
                    let stride = (DISSOLVE / v.max(1e-3)).round() as usize;
                    if stride > 4 || !s.is_multiple_of(stride) {
                        continue;
                    }
                }
                let base = if self.holiday.on {
                    self.meteor_col.lerp(self.meteor_tail, f * 1.6)
                } else {
                    self.meteor_col
                };
                let col = ctx.fade(base, 1.0 - v.max(LINK_FLOOR));
                self.br
                    .setf(hx + (tx - hx) * f + 0.5, hy + (ty - hy) * f + 0.5, col);
            }
        }
        if self.holiday.on {
            self.holiday.draw_lines(ctx, &self.stars, &mut self.br, m);
            self.holiday.paint_nodes(&mut self.stars);
        }
        self.br.blit(c);

        for s in &self.stars {
            let (x, y) = (s.x - m, s.y - m);
            if x < 0.0 || y < 0.0 || x >= self.sw || y >= self.sh {
                continue;
            }
            if s.glow > 0.0 {
                // A star drawing a holiday figure shines in its hue.
                let g = s.glow;
                let b = s.b + (1.0 - s.b) * g;
                let (ch, attr) = match s.kind {
                    _ if g < 0.5 => ('•', 0),
                    2 | 3 => ('✦', BOLD),
                    _ => ('•', BOLD),
                };
                let tint = self.tints[s.tint as usize].lerp(self.holiday.node_color(s), g);
                let col = ctx.fade(tint, 1.0 - b.clamp(0.0, 1.0));
                c.set_attr(x as i32, (y * 0.5) as i32, ch, col, attr);
                continue;
            }
            let b = s.b;
            let (ch, attr) = match s.kind {
                0 => ('·', 0),
                1 if b > 0.62 => ('•', 0),
                1 => ('∙', 0),
                2 if b > 0.78 => ('*', 0),
                2 if b > 0.45 => ('⋆', 0),
                2 => ('∙', 0),
                _ if b > 0.9 => ('✦', BOLD),
                _ if b > 0.6 => ('✦', 0),
                _ => ('✧', 0),
            };
            let col = ctx.fade(self.tints[s.tint as usize], 1.0 - b.clamp(0.0, 1.0));
            c.set_attr(x as i32, (y * 0.5) as i32, ch, col, attr);
        }

        if self.holiday.on {
            self.holiday.draw_marks(ctx, &self.stars, c, m);
        }

        // Meteor heads on top of everything.
        for mt in &self.meteors {
            if mt.on && mt.age < mt.life - 0.2 {
                c.set_attr(
                    mt.x.floor() as i32,
                    (mt.y * 0.5).floor() as i32,
                    '•',
                    self.meteor_col,
                    BOLD,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::scene::Opts;
    use crate::theme::Theme;

    const MARKS: [char; 3] = ['✿', '❀', '♥'];

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: theme.bg,
            seed: 11,
            fiesta,
        }
    }

    /// Runs `secs` seconds at 30 fps and draws the last frame.
    fn run(s: &mut Constellation, ctx: &Ctx, c: &mut Canvas, secs: f32) {
        for _ in 0..(secs * 30.0) as usize {
            s.tick(ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
        c.clear();
        s.draw(ctx, c);
    }

    fn marks(c: &Canvas) -> usize {
        c.cells().iter().filter(|x| MARKS.contains(&x.ch)).count()
    }

    /// Dots of finished figure links the holiday draws now.
    fn traced(s: &Constellation, ctx: &Ctx) -> usize {
        let mut br = Braille::new(s.sw as u16, (s.sh * 0.5) as u16);
        s.holiday.draw_lines(ctx, &s.stars, &mut br, s.margin)
    }

    /// A figure is drawn within seconds on the holiday's first day and at
    /// its height: its stars held and shining, its links traced.
    #[test]
    fn figures_come_on_the_holiday() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        for day in [(10, 28), (11, 2)] {
            let ctx = ctx(&theme, &opts, Fiesta::of_day(day.0, day.1));
            let mut s = Constellation::new(&ctx);
            s.resize(&ctx, 120, 36);
            s.retheme(&ctx);
            let mut c = Canvas::new(120, 36, Color::Reset);
            // Well into the first figure's stay.
            run(&mut s, &ctx, &mut c, 10.0);
            assert!(s.holiday.figures_on() >= 1, "{day:?}");
            assert!(s.stars.iter().any(|st| st.glow > 0.9), "{day:?}");
            assert!(traced(&s, &ctx) > 100, "{day:?}");
            assert_ne!(s.tints[0], rgb(NATIVE_TINTS[0]));
        }
    }

    /// No figures, no holiday colors on an ordinary day.
    #[test]
    fn ordinary_days_stay_plain() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = ctx(&theme, &opts, Fiesta::NONE);
        let mut s = Constellation::new(&ctx);
        s.resize(&ctx, 120, 36);
        s.retheme(&ctx);
        let mut c = Canvas::new(120, 36, Color::Reset);
        for _ in 0..6 {
            run(&mut s, &ctx, &mut c, 5.0);
            assert_eq!(s.holiday.figures_on(), 0);
            assert!(s.stars.iter().all(|st| st.fig == 0 && st.glow == 0.0));
            assert_eq!(traced(&s, &ctx), 0);
            assert_eq!(marks(&c), 0);
        }
        assert_eq!(s.tints[0], rgb(NATIVE_TINTS[0]));
    }

    /// The holiday arrives and leaves through `retheme`, as at midnight.
    #[test]
    fn retheme_switches_the_holiday() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let plain = ctx(&theme, &opts, Fiesta::NONE);
        let fiesta = ctx(&theme, &opts, Fiesta::of_day(11, 1));
        let mut s = Constellation::new(&plain);
        s.resize(&plain, 100, 30);
        s.retheme(&plain);
        let tints = s.tints;
        let mut c = Canvas::new(100, 30, Color::Reset);
        run(&mut s, &plain, &mut c, 3.0);
        s.retheme(&fiesta);
        assert_ne!(s.tints, tints);
        run(&mut s, &fiesta, &mut c, 9.0);
        assert!(s.holiday.figures_on() >= 1);
        assert!(traced(&s, &fiesta) > 100);
        s.retheme(&plain);
        assert_eq!(s.tints, tints);
        assert_eq!(s.holiday.figures_on(), 0);
        assert!(
            s.stars
                .iter()
                .all(|st| st.fig == 0 && st.gain == 1.0 && st.glow == 0.0)
        );
        run(&mut s, &plain, &mut c, 12.0);
        assert_eq!(s.holiday.figures_on(), 0);
        assert_eq!(marks(&c), 0);
    }

    /// Odd sizes, resizes mid-figure, huge steps and odd options survive
    /// the holiday under two themes.
    #[test]
    fn holiday_survives_sizes_and_steps() {
        let variants = [
            "",
            "density = 0.05",
            "density = 20.0\nlink_distance = 1.0",
            "max_links = 0\ntwinkle = 0.0",
            "speed = 50.0\nshooting_stars = 600.0",
        ];
        for v in variants {
            for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
                let opts = Opts::new(INFO.options, v.parse().unwrap());
                let ctx = ctx(&theme, &opts, Fiesta::of_day(11, 2));
                let mut s = Constellation::new(&ctx);
                s.retheme(&ctx);
                let sizes = [
                    (120, 36),
                    (1, 1),
                    (2, 3),
                    (7, 2),
                    (300, 90),
                    (40, 12),
                    (3, 40),
                ];
                for &(w, h) in &sizes {
                    s.resize(&ctx, w, h);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for f in 0..200 {
                        let dt = if f % 37 == 36 { 5.0 } else { 1.0 / 30.0 };
                        s.tick(&ctx, dt, dt);
                        c.clear();
                        s.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }

    /// Every figure fits the node budget and links only its own nodes.
    #[test]
    fn figures_are_well_formed() {
        muertos::check_figures();
    }
}
