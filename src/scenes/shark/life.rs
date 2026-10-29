//! Everything else alive in the sea, and what drifts in it: schools of fish,
//! rays and turtles passing by, jellyfish, lanternfish at night, bubbles,
//! marine snow, and flecks of light (scales, glowing plankton, foam).

use super::art::{self, Species, Sprite};
use super::hunter::Shark;
use super::muertos;
use super::sea::{SURF, Sea, View};
use super::{Dice, Rgb, Rng, WORLD_H};

/// A speck of marine snow, drifting at its own depth in the picture.
#[derive(Clone, Copy)]
struct Mote {
    /// Position in its own layer.
    x: f32,
    y: f32,
    /// How fast its layer scrolls: under 1 is behind the shark, over 1 in front.
    p: f32,
    phase: f32,
}

#[derive(Clone, Copy)]
pub struct Fish {
    pub x: f32,
    pub y: f32,
    vx: f32,
    vy: f32,
    phase: f32,
    /// 1 facing right, -1 facing left; it only turns once really going the
    /// other way, so a school does not flicker.
    face: f32,
    pub alive: bool,
}

pub struct School {
    species: &'static Species,
    /// Its fish are `first..first + n` of [`Life::fish`].
    first: usize,
    n: usize,
    /// Where it is drifting to.
    tx: f32,
    ty: f32,
    /// How frightened it is: 0 calm, 1 fleeing for its life.
    pub fear: f32,
    /// Drawn in front of the shark rather than behind it.
    front: bool,
}

/// A bubble rising to the surface.
#[derive(Clone, Copy)]
struct Bubble {
    x: f32,
    y: f32,
    big: bool,
    phase: f32,
}

/// What a fleck is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Speck {
    /// A scale glittering as it sinks.
    Scale,
    /// Plankton glowing where something stirred the water at night.
    Spark,
    /// Plankton drawn into a feeding whale shark's mouth.
    Plankton,
    /// Foam where a fin cuts the surface.
    Foam,
}

/// A fleck, drawn straight onto the screen.
#[derive(Clone, Copy)]
pub struct Fleck {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    /// Steps left, and how many it started with.
    life: u16,
    span: u16,
    kind: Speck,
}

/// Who drops by now and then.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Visit {
    /// A ray gliding over the sand.
    Ray,
    /// A turtle passing through, up to the surface for a breath at times.
    Turtle,
}

#[derive(Clone, Copy)]
pub struct Visitor {
    pub kind: Visit,
    pub x: f32,
    pub y: f32,
    vx: f32,
    /// The depth it is making for.
    ty: f32,
    phase: f32,
}

/// A jellyfish, pulsing up and sinking back.
#[derive(Clone, Copy)]
pub struct Jelly {
    pub x: f32,
    pub y: f32,
    pub phase: f32,
    /// Drawn in front of the shark rather than behind it.
    pub front: bool,
}

impl Jelly {
    /// One step of its drifting: a pulse lifts it, short of the surface;
    /// between pulses it sinks a little. Once left far behind, it comes back
    /// in from the other side.
    pub fn drift(&mut self, cam: f32, reach: f32, rng: &mut impl Dice) {
        self.phase = (self.phase + 0.035) % std::f32::consts::TAU;
        self.y += if self.phase < 1.2 && self.y > SURF as f32 + 18.0 {
            -0.16
        } else {
            0.035
        };
        self.x += 0.04;
        if (self.x - cam).abs() > reach || self.y > 115.0 {
            self.x = cam
                + if self.x < cam {
                    reach - 5.0
                } else {
                    5.0 - reach
                };
            self.y = rng.rangef(40.0, 105.0);
        }
    }
}

/// A lanternfish: up from the deep at night, its lights blinking.
#[derive(Clone, Copy)]
struct Lantern {
    x: f32,
    y: f32,
    vx: f32,
    phase: f32,
}

#[derive(Default)]
pub struct Life {
    motes: Vec<Mote>,
    pub fish: Vec<Fish>,
    pub schools: Vec<School>,
    bubbles: Vec<Bubble>,
    pub flecks: Vec<Fleck>,
    pub visitors: Vec<Visitor>,
    pub jellies: Vec<Jelly>,
    lanterns: Vec<Lantern>,
}

impl Life {
    pub fn populate(&mut self, shark: &Shark, tropical: bool, fish: f32, vw: f32, rng: &mut Rng) {
        let n = ((vw * 108.0) / 260.0).clamp(12.0, 240.0) as usize;
        self.motes = (0..n)
            .map(|_| Mote {
                x: shark.x + rng.rangef(-vw, vw),
                y: rng.rangef(0.0, WORLD_H as f32),
                p: rng.rangef(0.55, 1.4),
                phase: rng.rangef(0.0, 6.3),
            })
            .collect();
        let schools = (3.0 * fish).round().clamp(0.0, 12.0) as usize;
        let per = (15.0 * fish.clamp(0.5, 2.0)) as usize;
        self.fish = Vec::with_capacity(schools * (per + 8));
        self.schools = Vec::with_capacity(schools);
        let kinds: &'static [Species; 2] = if tropical {
            &art::WARM_SCHOOLS
        } else {
            &art::COOL_SCHOOLS
        };
        for i in 0..schools {
            let n = per - 4 + rng.usize(..9);
            let species = &kinds[rng.usize(..kinds.len())];
            self.schools.push(School {
                species,
                first: self.fish.len(),
                n,
                tx: 0.0,
                ty: 0.0,
                fear: 0.0,
                front: i % 3 == 2,
            });
            for _ in 0..n {
                self.fish.push(Fish {
                    x: 0.0,
                    y: 0.0,
                    vx: 0.0,
                    vy: 0.0,
                    phase: rng.rangef(0.0, 6.3),
                    face: 1.0,
                    alive: true,
                });
            }
        }
        self.bubbles = Vec::with_capacity(160);
        self.flecks = Vec::with_capacity(240);
        self.visitors = Vec::with_capacity(2);
        let jellies = (vw / 70.0).clamp(1.0, 6.0) as usize;
        self.jellies = (0..jellies)
            .map(|i| Jelly {
                x: shark.x + rng.rangef(-vw, vw),
                y: rng.rangef(30.0, 100.0),
                phase: rng.rangef(0.0, 6.3),
                front: i % 3 == 1,
            })
            .collect();
        let lanterns = (vw / 12.0).clamp(4.0, 30.0) as usize;
        self.lanterns = (0..lanterns)
            .map(|_| Lantern {
                x: shark.x + rng.rangef(-vw, vw),
                y: rng.rangef(40.0, 110.0),
                vx: rng.rangef(-0.15, 0.15),
                phase: rng.rangef(0.0, 6.3),
            })
            .collect();
    }

    pub fn shift(&mut self, dx: f32) {
        for m in &mut self.motes {
            m.x -= dx * m.p;
        }
        for f in &mut self.fish {
            f.x -= dx;
        }
        for s in &mut self.schools {
            s.tx -= dx;
        }
        for b in &mut self.bubbles {
            b.x -= dx;
        }
        for f in &mut self.flecks {
            f.x -= dx;
        }
        for v in &mut self.visitors {
            v.x -= dx;
        }
        for j in &mut self.jellies {
            j.x -= dx;
        }
        for l in &mut self.lanterns {
            l.x -= dx;
        }
    }

    /// The middle of school `i`'s living fish, if any are left.
    pub fn center(&self, i: usize) -> Option<(f32, f32)> {
        let s = &self.schools[i];
        let (mut x, mut y, mut n) = (0.0, 0.0, 0);
        for f in self.fish[s.first..s.first + s.n].iter().filter(|f| f.alive) {
            x += f.x;
            y += f.y;
            n += 1;
        }
        (n > 0).then(|| (x / n as f32, y / n as f32))
    }

    pub fn bubble(&mut self, x: f32, y: f32, big: bool, phase: f32) {
        if self.bubbles.len() < self.bubbles.capacity() {
            self.bubbles.push(Bubble { x, y, big, phase });
        }
    }

    pub fn fleck(&mut self, kind: Speck, (x, y): (f32, f32), (vx, vy): (f32, f32), life: u16) {
        if self.flecks.len() < self.flecks.capacity() {
            self.flecks.push(Fleck {
                x,
                y,
                vx,
                vy,
                life,
                span: life.max(1),
                kind,
            });
        }
    }

    /// A fish caught: it is gone in a puff of bubbles and glittering scales.
    pub fn puff(&mut self, x: f32, y: f32, rng: &mut Rng) {
        for _ in 0..7 {
            let (bx, by) = (x + rng.rangef(-3.0, 3.0), y + rng.rangef(-2.0, 2.0));
            self.bubble(bx, by, rng.chance(0.3), rng.rangef(0.0, 6.3));
        }
        for _ in 0..16 {
            let (vx, vy) = (rng.rangef(-0.5, 0.5), rng.rangef(-0.45, 0.25));
            self.fleck(Speck::Scale, (x, y), (vx, vy), rng.range(50, 110) as u16);
        }
    }
}

impl Sea {
    /// Puts school `i` somewhere ahead of the shark and out of sight, or
    /// anywhere around it at the start, calm.
    fn place_school(&mut self, i: usize, rng: &mut Rng, anywhere: bool) {
        let (vw, _) = self.view_size();
        let s = &self.shark;
        let ahead = if anywhere {
            rng.rangef(-0.6, 0.9) * vw
        } else {
            rng.rangef(0.7, 1.3) * vw
        };
        let x = s.x + s.heading() * ahead;
        let bed = self.bed(x) as f32;
        let y = rng.rangef(SURF as f32 + 18.0, bed - 14.0);
        let dir = rng.sign();
        let sc = &mut self.life.schools[i];
        sc.tx = x + dir * 40.0;
        sc.ty = y;
        sc.fear = 0.0;
        let pace = sc.species.pace;
        let (first, n) = (sc.first, sc.n);
        for f in &mut self.life.fish[first..first + n] {
            f.x = x + rng.rangef(-12.0, 12.0);
            f.y = y + rng.rangef(-5.0, 5.0);
            f.vx = dir * pace;
            f.vy = 0.0;
            f.face = dir;
            f.alive = true;
        }
    }

    pub fn place_schools(&mut self, rng: &mut Rng) {
        for i in 0..self.life.schools.len() {
            self.place_school(i, rng, true);
        }
    }

    pub fn update_life(&mut self, rng: &mut Rng) {
        let t = self.time();
        for m in &mut self.life.motes {
            m.y += 0.02 * m.p;
            m.x += 0.02 * m.p + 0.03 * (t * 0.7 + m.phase).sin();
        }
        self.update_schools(rng);
        self.update_visitors(rng);
        self.update_drifters(rng);
        // Bubbles wobble up and burst at the surface.
        let surf = SURF as f32;
        for b in &mut self.life.bubbles {
            b.phase += 0.2;
            b.y -= if b.big { 0.5 } else { 0.32 };
            b.x += 0.15 * b.phase.sin();
        }
        self.life.bubbles.retain(|b| b.y > surf);
        // Now and then a few bubbles rise from the floor.
        if rng.chance(0.012) {
            let (vw, _) = self.view_size();
            let x = self.shark.x + rng.rangef(-0.6, 0.6) * vw;
            let y = self.bed(x) as f32;
            for k in 0..rng.range(2, 6) {
                self.life.bubble(
                    x + rng.rangef(-1.0, 1.0),
                    y + k as f32 * 3.0,
                    rng.chance(0.3),
                    rng.rangef(0.0, 6.3),
                );
            }
        }
        for f in &mut self.life.flecks {
            f.x += f.vx;
            f.y += f.vy;
            // Scales slow down and sink; foam and plankton just drift.
            if f.kind == Speck::Scale {
                f.vx *= 0.94;
                f.vy = f.vy * 0.94 + 0.006;
            } else if f.kind != Speck::Plankton {
                f.vx *= 0.96;
                f.vy *= 0.96;
            }
            f.life = f.life.saturating_sub(1);
        }
        self.life.flecks.retain(|f| f.life > 0);
    }

    /// Boids: each fish keeps close to its school, matches its neighbours,
    /// keeps out of their way, and flees the shark.
    fn update_schools(&mut self, rng: &mut Rng) {
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        let shark = &self.shark;
        let (hx, hy) = shark.mouth();
        let (sx, sy) = shark.center();
        let half = Shark::W as f32 / 2.0;
        for i in 0..self.life.schools.len() {
            let Some((cx, cy)) = self.life.center(i) else {
                self.place_school(i, rng, false);
                continue;
            };
            if (cx - cam).abs() > 1.6 * vw + 60.0 {
                self.place_school(i, rng, false);
                continue;
            }
            let bed = self.bed(cx) as f32;
            let sc = &mut self.life.schools[i];
            let (first, n, pace) = (sc.first, sc.n, sc.species.pace);
            // Wander: a new spot nearby once there.
            if (sc.tx - cx).abs() < 8.0 && (sc.ty - cy).abs() < 6.0 {
                sc.tx = cx + rng.rangef(-70.0, 70.0);
                sc.ty = (cy + rng.rangef(-18.0, 18.0)).clamp(SURF as f32 + 14.0, bed - 10.0);
            }
            sc.fear *= 0.985;
            let (tx, ty) = (sc.tx, sc.ty);
            let (mut avx, mut avy, mut alive) = (0.0, 0.0, 0);
            for f in self.life.fish[first..first + n].iter().filter(|f| f.alive) {
                avx += f.vx;
                avy += f.vy;
                alive += 1;
            }
            let k = 1.0 / alive.max(1) as f32;
            let (avx, avy) = (avx * k, avy * k);
            let mut scare: f32 = 0.0;
            for a in first..first + n {
                if !self.life.fish[a].alive {
                    continue;
                }
                let f = self.life.fish[a];
                let (mut ax, mut ay) = (
                    (cx - f.x) * 0.0016 + (avx - f.vx) * 0.04,
                    (cy - f.y) * 0.0025 + (avy - f.vy) * 0.04,
                );
                // Personal space: a body length ahead and behind, less above
                // and below.
                for b in first..first + n {
                    let o = &self.life.fish[b];
                    if b == a || !o.alive {
                        continue;
                    }
                    let (dx, dy) = ((f.x - o.x) / 11.0, (f.y - o.y) / 5.0);
                    let d2 = dx * dx + dy * dy;
                    if d2 < 1.0 && d2 > 1e-6 {
                        let d = d2.sqrt();
                        ax += dx / d * (1.0 - d) * 0.05;
                        ay += dy / d * (1.0 - d) * 0.035;
                    }
                }
                let (gx, gy) = (tx - f.x, ty - f.y);
                let g = (gx * gx + gy * gy).sqrt().max(1.0);
                ax += gx / g * 0.012;
                ay += gy / g * 0.008;
                // The shark: from its mouth most of all, but its whole length.
                let along = ((f.x - sx) / half).clamp(-1.0, 1.0);
                let (bx, by) = (sx + along * half * 0.8, sy);
                for (px, py, r) in [(hx, hy, 34.0), (bx, by, 22.0)] {
                    let (dx, dy) = (f.x - px, f.y - py);
                    let d = (dx * dx + dy * dy).sqrt().max(0.5);
                    if d < r {
                        let push = (1.0 - d / r) * 0.35;
                        ax += dx / d * push;
                        ay += dy / d * push * 0.7;
                        scare = scare.max(1.0 - d / r);
                    }
                }
                if f.y < SURF as f32 + 6.0 {
                    ay += 0.05;
                }
                if f.y > bed - 5.0 {
                    ay -= 0.05;
                }
                let fear = self.life.schools[i].fear;
                let f = &mut self.life.fish[a];
                f.vx += ax + rng.rangef(-0.01, 0.01);
                f.vy += ay + rng.rangef(-0.008, 0.008);
                let top = pace * (1.0 + 3.0 * fear);
                let sp = (f.vx * f.vx + f.vy * f.vy).sqrt();
                if sp > top {
                    f.vx *= top / sp;
                    f.vy *= top / sp;
                }
                f.vy *= 0.97;
                f.x += f.vx;
                f.y += f.vy;
                f.phase += 0.18 + sp * 0.5;
                if f.vx * f.face < -0.12 {
                    f.face = -f.face;
                }
            }
            let sc = &mut self.life.schools[i];
            sc.fear = sc.fear.max(scare);
        }
    }

    /// Scatters the fish of every school near `(x, y)` away from it.
    pub fn scatter(&mut self, x: f32, y: f32, r: f32) {
        for i in 0..self.life.schools.len() {
            let (first, n) = (self.life.schools[i].first, self.life.schools[i].n);
            let mut hit = false;
            for f in self.life.fish[first..first + n]
                .iter_mut()
                .filter(|f| f.alive)
            {
                let (dx, dy) = (f.x - x, f.y - y);
                let d = (dx * dx + dy * dy).sqrt().max(0.5);
                if d < r {
                    let k = 2.2 * (1.0 - d / r) + 0.6;
                    f.vx += dx / d * k;
                    f.vy += dy / d * k * 0.6;
                    hit = true;
                }
            }
            if hit {
                self.life.schools[i].fear = 1.0;
            }
        }
    }

    /// Draws the schools on one side of the shark.
    fn draw_schools(&mut self, front: bool) {
        for i in 0..self.life.schools.len() {
            let sc = &self.life.schools[i];
            if sc.front != front {
                continue;
            }
            let (first, n, species) = (sc.first, sc.n, sc.species);
            for a in first..first + n {
                let f = self.life.fish[a];
                if !f.alive {
                    continue;
                }
                let frame = species.frames[(f.phase.sin() > 0.0) as usize];
                self.draw_small(frame, &species.colors, f.x, f.y, f.face, 0.12);
            }
        }
    }

    /// A small sprite centred on `(x, y)`, facing `face`, lit for its depth.
    pub fn draw_small(
        &mut self,
        sprite: Sprite,
        colors: &[(u8, Rgb)],
        x: f32,
        y: f32,
        face: f32,
        haze: f32,
    ) {
        let w = sprite[0].len() as i32;
        let h = sprite.len() as i32;
        let (x0, y0) = (
            (x - w as f32 / 2.0).round() as i32,
            (y - h as f32 / 2.0).round() as i32,
        );
        for (j, row) in sprite.iter().enumerate() {
            let py = y0 + j as i32;
            for (i, ch) in row.bytes().enumerate() {
                let Some(&(_, c)) = colors.iter().find(|(k, _)| *k == ch) else {
                    continue;
                };
                let px = if face > 0.0 {
                    x0 + i as i32
                } else {
                    x0 + w - 1 - i as i32
                };
                let c = self.shade(c, py, haze);
                self.put(px, py, c);
            }
        }
    }

    pub fn draw_life_behind(&mut self) {
        self.draw_visitors();
        self.draw_lanterns();
        self.draw_jellies(false);
        self.draw_schools(false);
    }

    pub fn draw_life_front(&mut self) {
        self.draw_schools(true);
        self.draw_jellies(true);
        self.draw_bubbles();
    }

    /// Rays and turtles: one at a time, now and then, from beyond the edge
    /// of the picture and on across it.
    fn update_visitors(&mut self, rng: &mut Rng) {
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        for i in 0..self.life.visitors.len() {
            let v = self.life.visitors[i];
            let bed = self.bed(v.x) as f32;
            let v = &mut self.life.visitors[i];
            v.x += v.vx;
            match v.kind {
                Visit::Ray => {
                    v.phase += 0.06;
                    v.ty = bed - 6.0 + 1.5 * (v.phase * 0.21).sin();
                }
                Visit::Turtle => {
                    v.phase += 0.09;
                    // Up for a breath now and then, then back down.
                    if v.y < SURF as f32 + 8.0 && rng.chance(0.01) {
                        v.ty = rng.rangef(50.0, (bed - 20.0).max(51.0));
                    } else if rng.chance(0.0015) {
                        v.ty = SURF as f32 + 6.0;
                    }
                }
            }
            v.y += (v.ty - v.y).clamp(-0.25, 0.25);
        }
        let gone = |v: &Visitor| (v.x - cam) * v.vx.signum() > vw * 0.8 + 40.0;
        self.life.visitors.retain(|v| !gone(v));
        if self.life.visitors.is_empty() && self.daylight > 0.2 && rng.chance(1.0 / 700.0) {
            let side = rng.sign();
            let x = cam + side * (vw * 0.5 + 30.0);
            let kind = if rng.chance(0.55) {
                Visit::Ray
            } else {
                Visit::Turtle
            };
            let (y, speed) = match kind {
                Visit::Ray => (self.bed(x) as f32 - 6.0, 0.45),
                Visit::Turtle => (rng.rangef(40.0, 90.0), 0.32),
            };
            self.life.visitors.push(Visitor {
                kind,
                x,
                y,
                vx: -side * speed,
                ty: y,
                phase: rng.rangef(0.0, 6.3),
            });
        }
    }

    /// Jellyfish and lanternfish drift around the camera; those left far
    /// behind come back in from the other side.
    fn update_drifters(&mut self, rng: &mut Rng) {
        let (vw, _) = self.view_size();
        let (cam, _) = self.cam();
        let reach = vw * 0.75 + 30.0;
        for j in &mut self.life.jellies {
            j.drift(cam, reach, rng);
        }
        for l in &mut self.life.lanterns {
            l.phase += 0.2;
            l.x += l.vx;
            l.y += 0.08 * (l.phase * 0.13).sin();
            if rng.chance(0.01) {
                l.vx = rng.rangef(-0.2, 0.2);
            }
            if (l.x - cam).abs() > reach {
                l.x = cam + if l.x < cam { reach - 5.0 } else { 5.0 - reach };
                l.y = rng.rangef(40.0, 110.0);
            }
        }
    }

    /// How far up from the deep the lanternfish have come: all the way at
    /// night, none of the way by day, rising at dusk and sinking at dawn.
    fn lantern_rise(&self) -> f32 {
        ((0.45 - self.daylight) / 0.35).clamp(0.0, 1.0)
    }

    fn draw_visitors(&mut self) {
        for i in 0..self.life.visitors.len() {
            let v = self.life.visitors[i];
            let face = v.vx.signum();
            match v.kind {
                Visit::Ray => {
                    let f = (v.phase as usize) % 4;
                    self.draw_small(art::RAY[f], &art::RAY_COLORS, v.x, v.y, face, 0.12);
                }
                Visit::Turtle => {
                    let f = (v.phase as usize) % 4;
                    self.draw_small(art::TURTLE[f], &art::TURTLE_COLORS, v.x, v.y, face, 0.12);
                }
            }
        }
    }

    /// Jellyfish, see-through, on one side of the shark.
    fn draw_jellies(&mut self, front: bool) {
        for i in 0..self.life.jellies.len() {
            let j = self.life.jellies[i];
            if j.front != front {
                continue;
            }
            let glow = self.jelly_glow(i);
            self.draw_jelly(j, glow);
        }
        self.draw_lamps(front);
    }

    /// One jellyfish, see-through. On a holiday it may glow instead, in a
    /// color and at a brightness (0 to 1).
    pub fn draw_jelly(&mut self, j: Jelly, glow: Option<(Rgb, f32)>) {
        let sprite = art::JELLY[(j.phase < 1.2) as usize];
        let w = sprite[0].len() as i32;
        let (x0, y0) = ((j.x - w as f32 / 2.0).round() as i32, j.y.round() as i32);
        if let Some((light, k)) = glow {
            self.jelly_halo(x0 + w / 2, y0 + 2, light, k);
        }
        for (r, row) in sprite.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                let Some(&(_, color)) = art::JELLY_COLORS.iter().find(|(k, _)| *k == ch) else {
                    continue;
                };
                let y = y0 + r as i32;
                let mut a = if ch == b't' { 0.32 } else { 0.55 };
                let color = match glow {
                    // Lit from inside: the rings white-hot, the bell paler
                    // than its rim, all of it brighter as it blinks.
                    Some((light, k)) => {
                        a += 0.3 * k;
                        let c = match ch {
                            b'g' => light.lerp(Rgb::WHITE, 0.7),
                            b'A' => light.lerp(Rgb::WHITE, 0.3),
                            _ => light,
                        };
                        c.scale(0.75 + 0.5 * k)
                    }
                    None => self.shade(color, y, 0.0),
                };
                self.blend(x0 + c as i32, y, color, a);
            }
        }
    }

    /// Lanternfish bodies, only at night; their lights come later, on screen.
    fn draw_lanterns(&mut self) {
        let rise = self.lantern_rise();
        if rise <= 0.0 {
            return;
        }
        let deep = 60.0 * (1.0 - rise);
        for i in 0..self.life.lanterns.len() {
            let l = self.life.lanterns[i];
            let face = if l.vx >= 0.0 { 1.0 } else { -1.0 };
            let f = (l.phase.sin() > 0.0) as usize;
            let y = l.y + deep;
            if y < self.bed(l.x) as f32 - 2.0 {
                self.draw_small(art::LANTERN[f], &art::LANTERN_COLORS, l.x, y, face, 0.2);
            }
        }
    }

    fn draw_bubbles(&mut self) {
        for i in 0..self.life.bubbles.len() {
            let b = self.life.bubbles[i];
            let (x, y) = (b.x.round() as i32, b.y.round() as i32);
            let rim = self.row(y).water.lerp(Rgb::WHITE, 0.55);
            if b.big {
                for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                    self.blend(x + dx, y + dy, rim, 0.6);
                }
                self.blend(x, y - 1, Rgb::WHITE, 0.5);
            } else {
                self.blend(x, y, rim, 0.7);
            }
        }
    }

    /// How a holiday colors a speck of marine snow, if it does: its color
    /// and strength.
    fn fiesta_mote(&self, i: usize, near: f32, phase: f32) -> Option<(Rgb, f32)> {
        self.navidad_mote(i, near, phase)
    }

    /// The color of the lights lanternfish `i` carries on a holiday.
    fn fiesta_lamp(&self, i: usize) -> Option<Rgb> {
        self.navidad_lamp(i)
    }

    /// Marine snow and flecks of light, straight onto the screen: one pixel
    /// each, so zooming out cannot blur them away.
    pub fn draw_motes(&mut self, v: View) {
        let span_w = v.w + 16.0;
        let span_h = v.h + 16.0;
        let tint = self.light.tint;
        for i in 0..self.life.motes.len() {
            let m = self.life.motes[i];
            let vx = (m.x - v.x * m.p).rem_euclid(span_w) - 8.0;
            let vy = (m.y - v.y * m.p).rem_euclid(span_h) - 8.0;
            let wy = v.y + vy;
            if wy < SURF as f32 + 2.0 {
                continue;
            }
            let (sx, sy) = ((vx * v.z) as i32, (vy * v.z) as i32);
            let near = ((m.p - 0.55) / 0.85).clamp(0.0, 1.0);
            let a = (0.05 + 0.22 * near) * (0.3 + 0.7 * self.daylight);
            let c = Rgb(214, 228, 226).mul(tint);
            match &self.muertos {
                // Día de Muertos: plankton glowing like embers, twinkling,
                // brightest at night.
                Some(md) if md.plankton(i) => {
                    let dark = 1.0 - self.daylight;
                    let twinkle = 0.65 + 0.35 * (self.time() * 1.7 + m.phase * 3.0).sin();
                    let a = (0.25 + 0.45 * near) * (0.45 + 0.55 * dark) * twinkle;
                    self.blend_out(sx, sy, muertos::GLOW, a);
                }
                _ => match self.fiesta_mote(i, near, m.phase) {
                    Some((fc, fa)) => self.blend_out(sx, sy, fc, fa),
                    None => self.blend_out(sx, sy, c, a),
                },
            }
        }
        let tick = self.tick;
        // Lanternfish lights and the jellies' faint glow, at night.
        let dark = self.lantern_rise();
        if dark > 0.0 {
            // On Día de Muertos the lanternfish carry candlelight, and the
            // jellies glow rosa mexicano.
            let (lamp, jelly) = match &self.muertos {
                Some(m) => (muertos::LAMP, m.dress.icing[0]),
                None => (Rgb(140, 230, 255), Rgb(210, 160, 255)),
            };
            let deep = 60.0 * (1.0 - dark);
            for i in 0..self.life.lanterns.len() {
                let l = self.life.lanterns[i];
                let y = l.y + deep;
                if y >= self.bed(l.x) as f32 - 2.0 {
                    continue;
                }
                let face = if l.vx >= 0.0 { 1.0 } else { -1.0 };
                let blink = 0.55 + 0.45 * (l.phase * 0.31).sin();
                let lamp = self.fiesta_lamp(i).unwrap_or(lamp);
                let (sx, sy) = v.to_screen(l.x + face * 1.5, y + 0.6);
                self.blend_out(sx, sy, lamp, dark * blink);
                let (sx, sy) = v.to_screen(l.x - face * 1.2, y + 0.6);
                self.blend_out(sx, sy, lamp, dark * blink * 0.6);
            }
            for i in 0..self.life.jellies.len() {
                let j = self.life.jellies[i];
                let (sx, sy) = v.to_screen(j.x, j.y + 2.0);
                let glow = dark * (0.35 + 0.25 * (j.phase).sin());
                let jelly = self.jelly_glow(i).map_or(jelly, |(c, _)| c);
                self.blend_out(sx, sy, jelly, glow);
            }
        }
        for i in 0..self.life.flecks.len() {
            let f = self.life.flecks[i];
            let (sx, sy) = v.to_screen(f.x, f.y);
            let fade = (f.life as f32 / f.span as f32).min(1.0);
            match f.kind {
                Speck::Spark => {
                    let c = self.fleck_color(f.kind).unwrap_or(Rgb(120, 255, 220));
                    self.blend_out(sx, sy, c, 0.9 * fade)
                }
                Speck::Scale => {
                    // Scales catch the light as they turn.
                    let twinkle = (tick.wrapping_add(i as u32 * 7) / 3).is_multiple_of(4);
                    let c = Rgb(236, 244, 250).mul(tint.lerp(Rgb::WHITE, 0.4));
                    self.blend_out(sx, sy, c, fade * if twinkle { 0.95 } else { 0.4 });
                }
                Speck::Plankton => {
                    let c = Rgb(200, 236, 200).mul(tint.lerp(Rgb::WHITE, 0.3));
                    let c = self.fleck_color(f.kind).unwrap_or(c);
                    self.blend_out(sx, sy, c, 0.55 * fade);
                }
                Speck::Foam => {
                    let c = Rgb::WHITE.mul(tint.lerp(Rgb::WHITE, 0.5));
                    self.blend_out(sx, sy, c, 0.85 * fade);
                }
            }
        }
    }
}
