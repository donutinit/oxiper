//! The sea: its clocks and light, the camera, and how the part in view gets
//! painted and projected onto the screen.

use super::art::Kind;
use super::hunter::Shark;
use super::life::Life;
use super::muertos::Muertos;
use super::navidad::Navidad;
use super::patrias::Patrias;
use super::water::{self, Col, RAY_LEVELS, SandRow};
use super::{
    Dice, Dressing, KEY, Light, Pal, Rgb, Rng, STEP, Settings, WORLD_H, ZOOMS, col, keys, noise,
};
use crate::render::Pixels;

/// Mean row of the waterline; above it is the sky, seen through the surface.
pub const SURF: i32 = 9;
/// Mean row of the sand.
pub const BED: i32 = 128;
/// World pixels kept clear around the shark at every zoom level.
const CAM_MARGIN: f32 = 4.0;

/// What part of the world is on screen: top-left corner in world pixels,
/// screen pixels per world pixel, and the size in world pixels.
#[derive(Clone, Copy, Debug)]
pub struct View {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub h: f32,
}

impl View {
    pub fn to_screen(self, x: f32, y: f32) -> (i32, i32) {
        (
            ((x - self.x) * self.z).floor() as i32,
            ((y - self.y) * self.z).floor() as i32,
        )
    }
}

/// The light at one depth.
#[derive(Clone, Copy, Default)]
pub struct Row {
    /// The water itself.
    pub water: Rgb,
    /// What things at this depth are lit by.
    pub lit: Rgb,
    /// The water brightened by a light shaft, by level.
    pub ray: [Rgb; RAY_LEVELS],
    /// The far ridges and the near reef, their top edge and below it.
    pub far_top: Rgb,
    pub far: Rgb,
    pub mid_top: Rgb,
    pub mid: Rgb,
}

/// One screen pixel's share of world pixels along one axis.
#[derive(Clone, Copy, Default)]
struct Tap {
    /// First world pixel (relative to the painted area) and how many.
    first: u32,
    n: u32,
    /// Index of its first weight.
    w: u32,
}

pub struct Sea {
    /// Screen size in half-block pixels.
    sw: i32,
    sph: i32,
    /// The projected picture, kept between steps.
    pub scr: Pixels,
    /// The part of the world in view, painted before projecting: columns
    /// `bx..bx + bw` and rows `by..by + bh`.
    pub buf: Vec<Rgb>,
    pub bx: i32,
    pub by: i32,
    pub bw: i32,
    pub bh: i32,
    /// Where local x 0 is on the endless sea, so that local coordinates stay
    /// small however far the shark swims.
    pub base: i64,
    pub seed: u32,
    /// Time of day, `[0, 1)`.
    pub t: f32,
    pub tick: u32,
    pub light: Light,
    /// How much daylight there is, 0 at night to 1 by day.
    pub daylight: f32,
    pub pal: Pal,
    pub set: Settings,
    /// World pixel rows in view.
    view_h: f32,
    /// What the camera centres on, trailing the shark.
    cam_x: f32,
    cam_y: f32,
    pub shark: Shark,
    pub life: Life,
    /// Warm water and coral reefs, or cool water and kelp.
    pub tropical: bool,
    /// Light by world row.
    pub rows: [Row; WORLD_H as usize],
    /// The painted columns' outlines and the sand's rows, for the backdrop.
    pub cols: Vec<Col>,
    pub sand: [SandRow; WORLD_H as usize],
    /// Light shaft levels along the slanted coordinate.
    pub rays: Vec<u8>,
    pub caustics: Vec<u8>,
    taps_x: Vec<Tap>,
    taps_y: Vec<Tap>,
    /// Weights of the taps, 256 being a whole screen pixel.
    weights: Vec<u32>,
    /// Día de Muertos, when it is on.
    pub muertos: Option<Muertos>,
    /// Navidad, when it is on.
    pub navidad: Option<Navidad>,
    /// The Fiestas Patrias, when they are on.
    pub patrias: Option<Patrias>,
}

impl Sea {
    pub fn new(
        cols: u16,
        rows: u16,
        rng: &mut Rng,
        pal: Pal,
        set: Settings,
        kind: Kind,
        dressing: Dressing,
    ) -> Sea {
        let (sw, sph) = (cols.max(1) as i32, 2 * rows.max(1) as i32);
        let seed = rng.u32(..);
        let shark = Shark::new(kind, rng);
        let mut sea = Sea {
            sw,
            sph,
            scr: Pixels::new(cols.max(1), rows.max(1)),
            buf: Vec::new(),
            bx: 0,
            by: 0,
            bw: 0,
            bh: 0,
            base: 0,
            seed,
            // Start somewhere in the morning or early afternoon.
            t: rng.rangef(0.3, 0.5),
            tick: 0,
            light: keys(&pal)[3].1,
            daylight: 1.0,
            pal,
            set,
            view_h: ZOOMS[set.zoom.min(ZOOMS.len() - 1)],
            cam_x: shark.x,
            cam_y: shark.y,
            shark,
            life: Life::default(),
            tropical: kind.tropical(),
            rows: [Row::default(); WORLD_H as usize],
            cols: Vec::new(),
            sand: [SandRow::default(); WORLD_H as usize],
            rays: Vec::new(),
            caustics: water::caustics(rng),
            taps_x: Vec::new(),
            taps_y: Vec::new(),
            weights: Vec::new(),
            muertos: None,
            navidad: None,
            patrias: None,
        };
        let (vw, vh) = sea.view_size();
        let (bw, bh) = (vw.ceil() as usize + 3, vh.ceil() as usize + 3);
        sea.buf = vec![Rgb::BLACK; bw * bh];
        sea.cols = vec![Col::default(); bw];
        sea.rays = vec![0; bw + bh + 8];
        sea.taps_x = vec![Tap::default(); sw as usize];
        sea.taps_y = vec![Tap::default(); sph as usize];
        // Each screen pixel takes at most this many world pixels per axis.
        let per = (bh as f32 / sph as f32).ceil() as usize + 2;
        sea.weights = vec![0; (sw as usize + sph as usize) * per];
        sea.life
            .populate(&sea.shark, sea.tropical, sea.set.fish, vw, rng);
        sea.place_schools(rng);
        sea.update_light();
        sea.wear(dressing);
        sea
    }

    /// Puts on (or takes off) the dress of whichever holiday is on.
    pub fn wear(&mut self, dressing: Dressing) {
        self.dress(dressing.muertos);
        self.dress_navidad(dressing.navidad);
        self.dress_patrias(dressing.patrias);
    }

    pub fn cam(&self) -> (f32, f32) {
        (self.cam_x, self.cam_y)
    }

    // ----- clocks and light -----

    /// Runs the clocks by `real` wall-clock seconds.
    pub fn clocks(&mut self, real: f32) {
        self.shark.clocks(real);
    }

    pub fn update_light(&mut self) {
        let t = self.t;
        let k = keys(&self.pal);
        for win in k.windows(2) {
            let (t0, a) = win[0];
            let (t1, b) = win[1];
            if t >= t0 && t <= t1 {
                let f = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
                self.light = a.lerp(b, f);
                break;
            }
        }
        let night = super::NIGHT.tint.luma() as f32;
        self.daylight = ((self.light.tint.luma() as f32 - night) / (255.0 - night)).clamp(0.0, 1.0);
        if self.tropical && self.pal.native {
            // Warm seas are clearer and greener-blue than cool ones.
            let (l, day) = (&mut self.light, self.daylight);
            l.shallow = l.shallow.lerp(Rgb(64, 204, 206), 0.4 * day);
            l.deep = l.deep.lerp(Rgb(14, 92, 140), 0.4 * day);
        }
    }

    /// Fills the light table for every row: the water's color, how things are
    /// lit at that depth (the deeper, the less red), the shafts' levels and
    /// the far layers' colors.
    fn light_rows(&mut self) {
        let l = self.light;
        let shaft = l.sky.lerp(Rgb::WHITE, 0.4);
        let strength = if self.set.rays {
            0.32 * (0.4 + 0.6 * self.daylight)
        } else {
            0.0
        };
        for y in 0..WORLD_H {
            let d = ((y - SURF) as f32 / (BED - SURF) as f32).clamp(0.0, 1.0);
            let water = l.shallow.lerp(l.deep, d.powf(0.8));
            let absorb = Rgb(
                (255.0 * (1.0 - 0.38 * d)) as u8,
                (255.0 * (1.0 - 0.2 * d)) as u8,
                (255.0 * (1.0 - 0.06 * d)) as u8,
            );
            let row = &mut self.rows[y as usize];
            row.water = water;
            row.lit = l.tint.mul(absorb);
            // Shafts fade out with depth.
            let k = strength * (1.0 - d).powf(1.6);
            for (i, r) in row.ray.iter_mut().enumerate() {
                *r = water.lerp(shaft, k * i as f32 / (RAY_LEVELS - 1) as f32);
            }
            let [far_top, far, mid_top, mid] = self.layer_colors(y);
            let row = &mut self.rows[y as usize];
            (row.far_top, row.far, row.mid_top, row.mid) = (far_top, far, mid_top, mid);
        }
    }

    /// The light row for world row `y`.
    #[inline]
    pub fn row(&self, y: i32) -> &Row {
        &self.rows[y.clamp(0, WORLD_H - 1) as usize]
    }

    /// `c` as seen at row `y`: lit for its depth, then `haze` of the way
    /// into the water's color.
    #[inline]
    pub fn shade(&self, c: Rgb, y: i32, haze: f32) -> Rgb {
        let r = self.row(y);
        c.mul(r.lit).lerp(r.water, haze)
    }

    /// Time in scene seconds, for things that sway.
    pub fn time(&self) -> f32 {
        self.tick as f32 * STEP as f32
    }

    // ----- simulation -----

    /// One step of the world, 30 of which make a second.
    pub fn update(&mut self, rng: &mut Rng) {
        self.tick = self.tick.wrapping_add(1);
        self.update_shark(rng);
        self.update_life(rng);
        self.update_muertos();
        self.update_navidad();
        self.update_patrias();
        self.follow();
        self.rebase();
    }

    /// The seabed's surface row under absolute x `u`.
    pub fn bed_at(&self, u: f64) -> i32 {
        let s = self.seed;
        let v = 4.5 * (noise(u, 1.0 / 110.0, 11, s) * 2.0 - 1.0)
            + 1.5 * (noise(u, 1.0 / 31.0, 12, s) * 2.0 - 1.0);
        BED + v.round() as i32
    }

    /// The seabed's row under local x.
    pub fn bed(&self, x: f32) -> i32 {
        self.bed_at(self.base as f64 + x as f64)
    }

    /// Width and height of the view in world pixels. Zooming in stops short
    /// of the point where the whole shark would not fit.
    pub fn view_size(&self) -> (f32, f32) {
        let aspect = self.sw as f32 / self.sph as f32;
        let fit_w = (Shark::W as f32 + 2.0 * CAM_MARGIN) / aspect;
        let fit_h = Shark::H as f32 + 2.0 * CAM_MARGIN;
        // On a very narrow screen the view may even be taller than the world,
        // showing more sky and sand, rather than cut the shark.
        let vh = self.view_h.min(WORLD_H as f32).max(fit_h).max(fit_w);
        (vh * aspect, vh)
    }

    /// Moves the camera after the shark: a little ahead of it, without
    /// bobbing for every small change of depth, never letting it out of frame.
    fn follow(&mut self) {
        let (vw, vh) = self.view_size();
        let s = &self.shark;
        let (cx, cy) = s.center();
        let (left, top, right, bottom) = s.bounds();
        // Where the camera may be and still show all of the shark.
        let span = |lo: f32, hi: f32, half: f32| {
            let (a, b) = (hi - half + CAM_MARGIN, lo + half - CAM_MARGIN);
            if a <= b {
                (a, b)
            } else {
                ((a + b) / 2.0, (a + b) / 2.0)
            }
        };
        let (x0, x1) = span(left, right, vw / 2.0);
        let (y0, y1) = span(top, bottom, vh / 2.0);
        // Keep room ahead of the shark. It may fall back a little before the
        // camera follows (a still picture is calmer to watch, and lighter
        // for the terminal), but never gets ahead of its place.
        let h = s.heading();
        let ahead = cx + vw * 0.13 * h;
        let behind = ((x1 - x0) * 0.25).min(vw * 0.15);
        let off = (ahead - self.cam_x) * h;
        let target = if off > 0.0 {
            ahead
        } else if off < -behind {
            ahead + behind * h
        } else {
            self.cam_x
        };
        self.cam_x += (target - self.cam_x) * 0.04;
        self.cam_x = self.cam_x.clamp(x0, x1);
        // Vertically the camera rests on the seabed, like a diver lying on
        // the sand, and only rises as far as it must to keep the shark in.
        let floor = WORLD_H as f32 - vh / 2.0;
        let want = floor.min(cy + (y1 - cy) * 0.85);
        self.cam_y += (want - self.cam_y) * 0.04;
        self.cam_y = self.cam_y.clamp(y0, y1);
    }

    /// Keeps local coordinates small: once the camera is far from local 0,
    /// shifts everything back and moves `base` instead.
    fn rebase(&mut self) {
        const CHUNK: f32 = 4096.0;
        if self.cam_x.abs() < CHUNK {
            return;
        }
        let shift = (self.cam_x / CHUNK).trunc() * CHUNK;
        self.base += shift as i64;
        self.cam_x -= shift;
        self.shark.shift(shift);
        self.life.shift(shift);
        if let Some(n) = &mut self.navidad {
            n.shift(shift);
        }
        if let Some(p) = &mut self.patrias {
            p.shift(shift);
        }
    }

    pub fn view(&self) -> View {
        let (vw, vh) = self.view_size();
        let z = self.sph as f32 / vh;
        // Snap to whole screen pixels so the picture only shifts in visible
        // steps instead of every cell's colour creeping with sub-pixel moves.
        let snap = |v: f32| (v * z).round() / z;
        let x = snap(self.cam_x - vw / 2.0);
        let room = WORLD_H as f32 - vh;
        let y = if room >= 0.0 {
            (self.cam_y - vh / 2.0).clamp(0.0, room)
        } else {
            room / 2.0
        };
        let y = snap(y);
        View {
            x,
            y,
            z,
            w: vw,
            h: vh,
        }
    }

    // ----- painting -----

    /// Paints `c` at local world pixel `(x, y)`, if it is in the painted area.
    #[inline]
    pub fn put(&mut self, x: i32, y: i32, c: Rgb) {
        let (i, j) = (x - self.bx, y - self.by);
        if i >= 0 && j >= 0 && i < self.bw && j < self.bh {
            self.buf[(j * self.bw + i) as usize] = c;
        }
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> Option<Rgb> {
        let (i, j) = (x - self.bx, y - self.by);
        (i >= 0 && j >= 0 && i < self.bw && j < self.bh)
            .then(|| self.buf[(j * self.bw + i) as usize])
    }

    /// Mixes `c` into local world pixel `(x, y)`; nothing over see-through water.
    #[inline]
    pub fn blend(&mut self, x: i32, y: i32, c: Rgb, a: f32) {
        let (i, j) = (x - self.bx, y - self.by);
        if i >= 0 && j >= 0 && i < self.bw && j < self.bh {
            let p = &mut self.buf[(j * self.bw + i) as usize];
            if *p != KEY {
                *p = p.lerp(c, a);
            }
        }
    }

    /// The sand's row under painted local column `x`, if it is painted.
    #[inline]
    pub fn bed_painted(&self, x: i32) -> Option<i32> {
        let i = x - self.bx;
        (i >= 0 && i < self.bw).then(|| self.cols[i as usize].bed)
    }

    /// How far a layer that scrolls at `p` is shifted: the layer coordinate of
    /// local column `x` is `x + shift`.
    pub fn shift(&self, v: View, p: f64) -> i64 {
        let left = self.base as f64 + v.x as f64;
        (left * p - v.x as f64).round() as i64
    }

    /// Paints the world, then projects it through the camera.
    pub fn render(&mut self) {
        let v = self.view();
        self.bx = v.x.floor() as i32;
        self.by = v.y.floor() as i32;
        let cap_w = self.cols.len() as i32;
        self.bw = ((v.x + v.w).ceil() as i32 - self.bx + 1).min(cap_w);
        self.bh = (v.y + v.h).ceil() as i32 + 1 - self.by;
        let cap_h = self.buf.len() as i32 / cap_w.max(1);
        self.bh = self.bh.min(cap_h);
        self.light_rows();
        self.paint_backdrop(v);
        self.paint_floor(v);
        self.paint_ofrendas();
        self.paint_navidad();
        self.paint_patrias();
        self.draw_life_behind();
        self.draw_shark();
        self.draw_life_front();
        self.paint_front(v);
        self.project(v);
        self.draw_motes(v);
        self.draw_muertos(v);
    }

    /// Resamples the painted world onto the screen: box-filtered when zoomed
    /// out, nearest pixel when zoomed in. See-through water stays empty.
    fn project(&mut self, v: View) {
        let inv = 1.0 / v.z;
        let mut wi = 0usize;
        // Each screen column's and row's share of world pixels.
        for axis in 0..2 {
            let (n, start, origin, limit) = if axis == 0 {
                (self.sw, v.x, self.bx, self.bw)
            } else {
                (self.sph, v.y, self.by, self.bh)
            };
            for s in 0..n {
                let a = start + s as f32 * inv - origin as f32;
                let b = a + inv;
                let tap = if v.z >= 1.0 {
                    let p = (((a + b) * 0.5) as i32).clamp(0, limit - 1);
                    self.weights[wi] = 256;
                    Tap {
                        first: p as u32,
                        n: 1,
                        w: wi as u32,
                    }
                } else {
                    let first = (a.floor() as i32).clamp(0, limit - 1);
                    let last = ((b.ceil() as i32) - 1).clamp(first, limit - 1);
                    let (mut k, mut sum, mut big) = (0, 0, wi);
                    for p in first..=last {
                        let w = (b.min(p as f32 + 1.0) - a.max(p as f32)).max(0.0);
                        if wi + k < self.weights.len() {
                            // As a share of the screen pixel, so sums stay small.
                            let w = (w * 256.0 * v.z).round() as u32;
                            self.weights[wi + k] = w;
                            if w > self.weights[big] {
                                big = wi + k;
                            }
                            sum += w;
                            k += 1;
                        }
                    }
                    // Make the shares add up to exactly 256, so a pixel needs
                    // no division unless see-through water is part of it.
                    if k > 0 {
                        self.weights[big] = (self.weights[big] + 256).saturating_sub(sum);
                    }
                    Tap {
                        first: first as u32,
                        n: k as u32,
                        w: wi as u32,
                    }
                };
                wi += tap.n as usize;
                if axis == 0 {
                    self.taps_x[s as usize] = tap;
                } else {
                    self.taps_y[s as usize] = tap;
                }
            }
        }
        let bw = self.bw as usize;
        // With the water painted nothing in the world is see-through, and
        // the filter can skip asking.
        let clear = !self.set.water;
        for sy in 0..self.sph {
            let ty = self.taps_y[sy as usize];
            let wys = &self.weights[ty.w as usize..(ty.w + ty.n) as usize];
            for sx in 0..self.sw {
                let tx = self.taps_x[sx as usize];
                let c = if ty.n == 1 && tx.n == 1 {
                    let p = self.buf[ty.first as usize * bw + tx.first as usize];
                    (p != KEY).then_some(p)
                } else {
                    let wxs = &self.weights[tx.w as usize..(tx.w + tx.n) as usize];
                    let (mut r, mut g, mut b, mut lit, mut key) = (0u32, 0u32, 0u32, 0u32, 0u32);
                    for (j, &wy) in wys.iter().enumerate() {
                        let start = (ty.first as usize + j) * bw + tx.first as usize;
                        let px = &self.buf[start..start + wxs.len()];
                        for (p, &wx) in px.iter().zip(wxs) {
                            let wt = wy * wx;
                            if clear && *p == KEY {
                                key += wt;
                            } else {
                                lit += wt;
                                r += p.0 as u32 * wt;
                                g += p.1 as u32 * wt;
                                b += p.2 as u32 * wt;
                            }
                        }
                    }
                    if key == 0 {
                        // The shares add up to 256 x 256: no division needed.
                        let f = |v: u32| ((v + 32_768) >> 16) as u8;
                        Some(Rgb(f(r), f(g), f(b)))
                    } else if lit == 0 || key > lit {
                        None
                    } else {
                        let k = 1.0 / lit as f32;
                        let f = |v: u32| (v as f32 * k + 0.5) as u8;
                        Some(Rgb(f(r), f(g), f(b)))
                    }
                };
                self.scr
                    .set(sx, sy, c.map_or(crate::color::Color::Reset, col));
            }
        }
    }

    /// Mixes `c` into a screen pixel; nothing over see-through water.
    #[inline]
    pub fn blend_out(&mut self, x: i32, y: i32, c: Rgb, a: f32) {
        if let Some(p) = super::rgb_of(self.scr.get(x, y)) {
            self.scr.set(x, y, col(p.lerp(c, a)));
        }
    }
}
