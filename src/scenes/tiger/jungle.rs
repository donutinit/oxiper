//! The jungle: its clocks and light, the ground, the water and the forest
//! layers, the camera, and how the part in view gets painted and projected
//! onto the screen.

use super::body::{Figure, Shears};
use super::hunter::Tiger;
use super::life::Life;
use super::ufo::Ufo;
use super::{
    Dice, KEY, Kind, Light, NIGHT, Pal, Rgb, Rng, STEP, Settings, WORLD_H, ZOOMS, col, hash, keys,
    noise, rgb_of, unit,
};
use crate::color::Color;
use crate::render::{Canvas, Pixels};

/// The row the animals stand on.
pub const FEET: i32 = 100;
/// The far edge of the ground, where the forest begins.
pub const BACK: i32 = 92;
/// World pixels kept clear around the tiger.
const CAM_MARGIN: f32 = 4.0;
/// How fast the layers scroll, the tiger's ground being 1.
const HILL_P: f64 = 0.12;
const FAR_P: f64 = 0.3;
pub const MID_P: f64 = 0.6;
const FRONT_P: f64 = 1.3;
/// Water comes in cells this wide: a river, a pool, or dry land.
const WATER_CELL: i64 = 560;
/// Big trees on the tiger's ground, and patches of tall grass.
pub const TREE_CELL: i64 = 170;
const GRASS_CELL: i64 = 60;
/// Trees and lianas of the middle layer.
pub const MID_CELL: i64 = 34;
const LIANA_CELL: i64 = 23;
/// How long it is, nose to tail tip.
pub const TIGER_LONG: f32 = 60.0;
/// Rows of view the saucer needs, at least, to be seen with its beam.
const UFO_VIEW: f32 = 86.0;

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

/// A stretch of water across the ground, in local x.
#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub a: f32,
    pub b: f32,
    /// Deep enough to swim; a pool is only for wading and drinking.
    pub river: bool,
    /// A crocodile lives in it.
    pub croc: bool,
    /// Which one it is, for remembering what happened there.
    pub id: i64,
}

/// A big tree on the tiger's ground, in local x.
#[derive(Clone, Copy, Debug)]
pub struct Trunk {
    pub x: f32,
    pub w: i32,
    /// Its two big limbs: height, length and side (-1 left, 1 right).
    pub limbs: [(i32, i32, i32); 2],
    pub id: i64,
}

/// A word or a mark floating over the picture: z's, alarms.
#[derive(Clone, Copy)]
pub struct Note {
    pub x: f32,
    pub y: f32,
    pub text: &'static str,
    pub color: Rgb,
    pub age: u16,
    pub span: u16,
    pub rise: f32,
}

/// A glowing firefly, or a snowflake, drifting near the camera.
#[derive(Clone, Copy)]
struct Mote {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    phase: f32,
}

/// Where things begin in one painted column.
#[derive(Clone, Copy, Default)]
struct Col {
    /// Rows where the hills, the far forest and the trees of the middle
    /// layer begin, and where the canopy above ends.
    hill: i32,
    far: i32,
    canopy: i32,
    /// The middle layer's trunk here: 0 none, 1 trunk, 2 its lit edge.
    trunk: u8,
    /// The layer coordinates, for texture.
    uf: i32,
    um: i32,
    /// The water here, if any: 0 none, 1 bank, 2 water.
    water: u8,
}

/// One screen pixel's share of world pixels along one axis.
#[derive(Clone, Copy, Default)]
struct Tap {
    first: u32,
    n: u32,
    w: u32,
}

pub struct Jungle {
    /// Screen size in half-block pixels.
    sw: i32,
    sph: i32,
    /// The projected picture, kept between steps.
    pub scr: Pixels,
    /// The part of the world in view, painted before projecting.
    buf: Vec<Rgb>,
    pub bx: i32,
    pub by: i32,
    pub bw: i32,
    pub bh: i32,
    cols: Vec<Col>,
    /// Where local x 0 is on the endless jungle.
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
    pub kind: Kind,
    /// World pixel rows in view now; it eases toward the zoom level.
    view_h: f32,
    cam_x: f32,
    cam_y: f32,
    /// Seconds of shaking left, after a roar.
    pub shake: f32,
    pub tiger: Tiger,
    pub life: Life,
    pub ufo: Ufo,
    /// Trees the tiger has clawed, by id.
    pub marks: Vec<i64>,
    pub fig: Figure,
    pub notes: Vec<Note>,
    motes: Vec<Mote>,
    /// Stars, as fractions of the screen, with their twinkle phase.
    stars: Vec<(f32, f32, f32)>,
    taps_x: Vec<Tap>,
    taps_y: Vec<Tap>,
    weights: Vec<u32>,
    /// What the last frame painted of the holiday, for the tests.
    #[cfg(test)]
    pub painted: super::fiestas::Painted,
}

impl Jungle {
    pub fn new(cols: u16, rows: u16, rng: &mut Rng, pal: Pal, set: Settings, kind: Kind) -> Jungle {
        let (sw, sph) = (cols.max(1) as i32, 2 * rows.max(1) as i32);
        let seed = rng.u32(..);
        let tiger = Tiger::new(kind, rng);
        let stars = (0..(sw * sph / 70).max(4))
            .map(|_| (rng.f32(), rng.f32() * 0.6, rng.rangef(0.0, 6.3)))
            .collect();
        let mut j = Jungle {
            sw,
            sph,
            scr: Pixels::new(cols.max(1), rows.max(1)),
            buf: Vec::new(),
            bx: 0,
            by: 0,
            bw: 0,
            bh: 0,
            cols: Vec::new(),
            base: 0,
            seed,
            // Start somewhere in the morning or early afternoon.
            t: rng.rangef(0.3, 0.5),
            tick: 0,
            light: keys(&pal)[3].1,
            daylight: 1.0,
            pal,
            set,
            kind,
            view_h: ZOOMS[set.zoom.min(ZOOMS.len() - 1)],
            cam_x: 0.0,
            cam_y: 0.0,
            shake: 0.0,
            tiger,
            life: Life::default(),
            ufo: Ufo::default(),
            marks: Vec::new(),
            fig: Figure::new(),
            notes: Vec::new(),
            motes: Vec::new(),
            stars,
            taps_x: vec![Tap::default(); sw as usize],
            taps_y: vec![Tap::default(); sph as usize],
            weights: Vec::new(),
            #[cfg(test)]
            painted: Default::default(),
        };
        // Start on dry land, away from the trees.
        while let Some(s) = j.water(j.tiger.x) {
            j.tiger.x = s.b + 40.0;
        }
        j.tiger.tx = j.tiger.x + j.tiger.dir * 150.0;
        j.cam_x = j.tiger.x;
        // Room for the widest view the camera will ever take.
        let aspect = sw as f32 / sph as f32;
        let fit_w = (TIGER_LONG + 2.0 * CAM_MARGIN) / aspect;
        let vh = ZOOMS[0].max(UFO_VIEW).max(fit_w).max(j.fit_h());
        let (bw, bh) = ((vh * aspect).ceil() as usize + 3, vh.ceil() as usize + 3);
        j.buf = vec![Rgb::BLACK; bw * bh];
        j.cols = vec![Col::default(); bw];
        let per = (bh as f32 / sph as f32).ceil() as usize + 2;
        j.weights = vec![0; (sw as usize + sph as usize) * per];
        j.cam_y = WORLD_H as f32 - j.view_size().1;
        j.update_light();
        j.populate(rng);
        j
    }

    // ----- clocks and light -----

    /// Runs the clocks by `real` wall-clock seconds.
    pub fn clocks(&mut self, real: f32, rng: &mut Rng) {
        self.tiger.clocks(real);
        self.life.clocks(real);
        self.ufo_clocks(real, rng);
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
        // While the saucer hangs over the jungle, its light washes everything.
        let eerie = self.ufo.glow();
        if eerie > 0.0 {
            let l = &mut self.light;
            let (tint, top) = self.pal.eerie();
            l.tint = l.tint.lerp(tint, 0.35 * eerie);
            l.top = l.top.lerp(top, 0.4 * eerie);
        }
        let night = NIGHT.tint.luma() as f32;
        self.daylight = ((self.light.tint.luma() as f32 - night) / (255.0 - night)).clamp(0.0, 1.0);
    }

    /// Time in scene seconds, for things that sway.
    pub fn time(&self) -> f32 {
        self.tick as f32 * STEP as f32
    }

    // ----- simulation -----

    /// One step of the world, 30 of which make a second.
    pub fn update(&mut self, rng: &mut Rng) {
        self.tick = self.tick.wrapping_add(1);
        self.update_ufo(rng);
        self.update_tiger(rng);
        self.update_life(rng);
        self.update_motes(rng);
        for n in &mut self.notes {
            n.age += 1;
            n.y -= n.rise;
            n.x += 0.1 * (n.age as f32 * 0.15).sin() * n.rise.signum();
        }
        self.notes.retain(|n| n.age < n.span);
        self.shake = (self.shake - STEP as f32).max(0.0);
        self.follow();
        self.rebase();
    }

    /// Adds a note over the picture.
    pub fn note(&mut self, x: f32, y: f32, text: &'static str, color: Rgb, span: u16, rise: f32) {
        if self.notes.len() < 40 {
            self.notes.push(Note {
                x,
                y,
                text,
                color,
                age: 0,
                span,
                rise,
            });
        }
    }

    // ----- what lies where -----

    fn span_in(&self, cell: i64) -> Option<Span> {
        let (s, c) = (self.seed, cell as i32);
        let r = unit(c, 71, s);
        let (river, width) = if r < 0.32 {
            (true, 46.0 + 34.0 * unit(c, 72, s))
        } else if r < 0.62 {
            (false, 18.0 + 14.0 * unit(c, 72, s))
        } else {
            return None;
        };
        let mid =
            (cell * WATER_CELL) as f64 + 90.0 + unit(c, 73, s) as f64 * (WATER_CELL - 180) as f64;
        let a = (mid - self.base as f64 - width as f64 / 2.0) as f32;
        Some(Span {
            a,
            b: a + width,
            river,
            croc: river && !self.kind.snowy() && unit(c, 74, s) < 0.6,
            id: cell,
        })
    }

    /// The water at local x, if there is any.
    pub fn water(&self, x: f32) -> Option<Span> {
        let u = self.base as f64 + x as f64;
        let cell = (u / WATER_CELL as f64).floor() as i64;
        (cell - 1..=cell + 1)
            .filter_map(|c| self.span_in(c))
            .find(|s| x >= s.a && x < s.b)
    }

    /// The water nearest to local x within `range`, ahead first.
    pub fn water_near(&self, x: f32, range: f32) -> Option<Span> {
        let u = self.base as f64 + x as f64;
        let cell = (u / WATER_CELL as f64).floor() as i64;
        (cell - 2..=cell + 2)
            .filter_map(|c| self.span_in(c))
            .filter(|s| (s.a - x).abs().min((s.b - x).abs()) < range)
            .min_by(|p, q| {
                let d = |s: &Span| (s.a - x).abs().min((s.b - x).abs());
                d(p).total_cmp(&d(q))
            })
    }

    pub fn trunk_in(&self, cell: i64) -> Option<Trunk> {
        let (s, c) = (self.seed, cell as i32);
        if unit(c, 81, s) > 0.62 {
            return None;
        }
        let u = (cell * TREE_CELL) as f64 + 24.0 + unit(c, 82, s) as f64 * (TREE_CELL - 48) as f64;
        let x = (u - self.base as f64) as f32;
        let w = 7 + (unit(c, 83, s) * 5.0) as i32;
        if self.water(x - w as f32).is_some() || self.water(x + w as f32).is_some() {
            return None;
        }
        let side = |k: i32| if unit(c, 84 + k, s) < 0.5 { -1 } else { 1 };
        Some(Trunk {
            x,
            w,
            limbs: [
                (
                    34 + (unit(c, 86, s) * 10.0) as i32,
                    14 + (unit(c, 87, s) * 12.0) as i32,
                    side(0),
                ),
                (
                    52 + (unit(c, 88, s) * 10.0) as i32,
                    12 + (unit(c, 89, s) * 10.0) as i32,
                    side(1),
                ),
            ],
            id: cell,
        })
    }

    /// The big trees with any part between local `x0` and `x1`.
    pub fn trunks(&self, x0: f32, x1: f32) -> Vec<Trunk> {
        let a = ((self.base as f64 + x0 as f64 - 40.0) / TREE_CELL as f64).floor() as i64;
        let b = ((self.base as f64 + x1 as f64 + 40.0) / TREE_CELL as f64).floor() as i64;
        (a..=b).filter_map(|c| self.trunk_in(c)).collect()
    }

    /// A patch of tall grass: from, to, and how tall.
    fn grass_in(&self, cell: i64) -> Option<(f32, f32, f32)> {
        let (s, c) = (self.seed, cell as i32);
        if unit(c, 91, s) > 0.55 {
            return None;
        }
        let w = 18.0 + 26.0 * unit(c, 92, s);
        let u = (cell * GRASS_CELL) as f64 + unit(c, 93, s) as f64 * (GRASS_CELL as f64 - w as f64);
        let a = (u - self.base as f64) as f32;
        let tall = 9.0 + 8.0 * unit(c, 94, s);
        if self.water(a).is_some() || self.water(a + w).is_some() {
            return None;
        }
        Some((a, a + w, tall))
    }

    /// How tall the grass stands at local x; 0 where there is none.
    pub fn grass(&self, x: f32) -> f32 {
        let u = self.base as f64 + x as f64;
        let cell = (u / GRASS_CELL as f64).floor() as i64;
        (cell - 1..=cell + 1)
            .filter_map(|c| self.grass_in(c))
            .filter(|&(a, b, _)| x >= a && x < b)
            .map(|(_, _, t)| t)
            .fold(0.0, f32::max)
    }

    // ----- the camera -----

    /// Rows the view needs, at least: the ground and a standing tiger.
    fn fit_h(&self) -> f32 {
        (WORLD_H - FEET) as f32 + 30.0 + 2.0 * CAM_MARGIN
    }

    /// Width and height of the view in world pixels. Zooming in stops short
    /// of the point where the whole tiger would not fit.
    pub fn view_size(&self) -> (f32, f32) {
        let aspect = self.sw as f32 / self.sph as f32;
        let fit_w = (TIGER_LONG + 2.0 * CAM_MARGIN) / aspect;
        let vh = self.view_h.max(fit_w).max(self.fit_h());
        (vh * aspect, vh)
    }

    /// Moves the camera after the tiger, a little ahead of it, never letting
    /// it out of frame; widens the view while the saucer is about.
    fn follow(&mut self) {
        let zoom = ZOOMS[self.set.zoom.min(ZOOMS.len() - 1)];
        let want_h = if self.ufo.wide() {
            // On a holiday it pulls back further, if its saucer is bigger or
            // the saucer must hang clear of what is strung across the top.
            zoom.max(self.pal.room(UFO_VIEW).wide).min(WORLD_H as f32)
        } else {
            zoom
        };
        self.view_h += (want_h - self.view_h) * 0.05;
        if (want_h - self.view_h).abs() < 0.05 {
            self.view_h = want_h;
        }
        let (vw, vh) = self.view_size();
        let room = WORLD_H as f32 - vh;
        let mut want_y = room;
        if !self.tiger.gone {
            let (l, top, r, _) = self.tiger.bounds();
            let h = self.tiger.heading();
            let cx = (l + r) / 2.0;
            // Hunting or squaring up, the camera takes in the other animal too.
            let ahead = match self.focus() {
                Some(f) if (f - cx).abs() < vw * 0.8 => (cx + f) / 2.0,
                _ => cx + vw * 0.12 * h,
            };
            let behind = vw * 0.12;
            let off = (ahead - self.cam_x) * h;
            let target = if off > 0.0 {
                ahead
            } else if off < -behind {
                ahead + behind * h
            } else {
                self.cam_x
            };
            self.cam_x += (target - self.cam_x) * 0.05;
            // Never let it out of frame.
            let half = vw / 2.0 - CAM_MARGIN;
            let (lo, hi) = (r - half, l + half);
            self.cam_x = if lo <= hi {
                self.cam_x.clamp(lo, hi)
            } else {
                (l + r) / 2.0
            };
            want_y = want_y.min(top - CAM_MARGIN);
        }
        if let Some(top) = self.ufo.top() {
            // A holiday's saucer may stand taller, and keep clear of the flags.
            let room = self.pal.room(UFO_VIEW);
            want_y = want_y.min(top - 2.0 - room.tall - room.clear);
        }
        let want_y = want_y.max(0.0).min(room.max(0.0));
        self.cam_y += (want_y - self.cam_y) * 0.1;
        if !self.tiger.gone {
            // Rising or rearing, the head never leaves the frame.
            let (_, top, _, _) = self.tiger.bounds();
            self.cam_y = self.cam_y.min((top - 1.0).max(0.0));
        }
        self.cam_y = self.cam_y.clamp(0.0, room.max(0.0));
    }

    /// What the tiger is after or facing, if anything: its x.
    fn focus(&self) -> Option<f32> {
        use super::hunter::Mood;
        let t = &self.tiger;
        match t.mood {
            Mood::Stalk | Mood::Charge | Mood::Pounce => {
                t.prey.and_then(|id| self.life.beast(id)).map(|b| b.x)
            }
            Mood::Fend => t.rival.and_then(|id| self.life.rival(id)),
            _ => None,
        }
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
        self.tiger.shift(shift);
        self.life.shift(shift);
        self.ufo.shift(shift);
        for n in &mut self.notes {
            n.x -= shift;
        }
        for m in &mut self.motes {
            m.x -= shift;
        }
    }

    pub fn view(&self) -> View {
        let (vw, vh) = self.view_size();
        let z = self.sph as f32 / vh;
        // Snap to whole screen pixels so the picture only shifts in visible
        // steps instead of every cell's colour creeping with sub-pixel moves.
        let snap = |v: f32| (v * z).round() / z;
        // A roar shakes the picture by a screen pixel or two.
        let jolt = if self.shake > 0.0 {
            const J: [f32; 6] = [-1.0, 1.0, 0.0, 1.0, -1.0, 0.0];
            J[(self.tick % 6) as usize] * 1.5 * self.shake.min(1.0) / z.min(1.0)
        } else {
            0.0
        };
        let room = WORLD_H as f32 - vh;
        let y = if room >= 0.0 {
            self.cam_y.clamp(0.0, room)
        } else {
            room / 2.0
        };
        View {
            x: snap(self.cam_x - vw / 2.0 + jolt),
            y: snap(y),
            z,
            w: vw,
            h: vh,
        }
    }

    /// How far a layer that scrolls at `p` is shifted: the layer coordinate of
    /// local column `x` is `x + shift`.
    pub fn shift(&self, v: View, p: f64) -> i64 {
        let left = self.base as f64 + v.x as f64;
        (left * p - v.x as f64).round() as i64
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

    /// Mixes `c` into local world pixel `(x, y)`; nothing over see-through sky.
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

    /// `c` as lit by the light of the hour.
    #[inline]
    pub fn lit(&self, c: Rgb) -> Rgb {
        c.mul(self.light.tint)
    }

    /// A theme's greens: the jungle's own under the native theme.
    #[inline]
    pub fn green(&self, c: Rgb) -> Rgb {
        c.lerp(self.pal.green, self.pal.green_k)
    }

    /// `c` lit, then `k` of the way into the haze of the distance.
    #[inline]
    pub fn far(&self, c: Rgb, k: f32) -> Rgb {
        self.lit(c)
            .lerp(self.light.bot.lerp(self.light.top, 0.25), k)
    }

    /// A palette lit for the hour: letters to colors. Eyes shine at night.
    pub fn lut(&self, pal: &[(u8, Rgb)]) -> [Option<Rgb>; 128] {
        let mut lut = [None; 128];
        let dark = 1.0 - self.daylight;
        for &(ch, c) in pal {
            let mut lit = self.lit(c);
            if ch == b'e' && dark > 0.4 {
                lit = lit.lerp(c.lerp(Rgb(200, 255, 150), 0.4), (dark - 0.4) * 1.6);
            }
            lut[(ch & 127) as usize] = Some(lit);
        }
        lut
    }

    /// Places the figure composed in [`Jungle::fig`]: its body pixel `(cx,
    /// ground)` lands on local world pixel `(x, y)`, it is mirrored when
    /// facing left, turned by `angle` (nose down positive) about body pixel
    /// `pivot`, and anything below the world row `water` is seen through it.
    #[allow(clippy::too_many_arguments)]
    pub fn place(
        &mut self,
        x: f32,
        y: f32,
        (cx, ground): (i32, i32),
        face: f32,
        angle: f32,
        pivot: (i32, i32),
        lut: &[Option<Rgb>; 128],
        water: Option<(i32, Rgb)>,
    ) {
        let (ox, oy) = (x.round() as i32, y.round() as i32);
        let turned = angle.abs() >= 0.02;
        let sh = Shears::new(angle);
        let Jungle {
            fig,
            buf,
            bx,
            by,
            bw,
            bh,
            ..
        } = self;
        fig.each(|px, py, ch| {
            let Some(c) = lut[(ch & 127) as usize] else {
                return;
            };
            let (dx, dy) = if turned {
                let (a, b) = sh.turn(px - pivot.0, py - pivot.1);
                (a + pivot.0, b + pivot.1)
            } else {
                (px, py)
            };
            let wx = if face >= 0.0 {
                ox + dx - cx
            } else {
                ox + cx - dx
            };
            let wy = oy + dy - ground;
            let c = match water {
                Some((line, w)) if wy >= line => c.lerp(w, 0.72),
                _ => c,
            };
            let (i, j) = (wx - *bx, wy - *by);
            if i >= 0 && j >= 0 && i < *bw && j < *bh {
                buf[(j * *bw + i) as usize] = c;
            }
        });
    }

    /// Draws a sprite as it is, with its bottom middle on `(x, y)`.
    pub fn sprite(
        &mut self,
        s: super::art::Sprite,
        x: f32,
        y: f32,
        face: f32,
        lut: &[Option<Rgb>; 128],
    ) {
        let w = s[0].len() as i32;
        let h = s.len() as i32;
        let (x0, y0) = (x.round() as i32 - w / 2, y.round() as i32 - h + 1);
        for (r, row) in s.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                if let Some(col) = lut[(ch & 127) as usize] {
                    let cx = if face >= 0.0 {
                        c as i32
                    } else {
                        w - 1 - c as i32
                    };
                    self.put(x0 + cx, y0 + r as i32, col);
                }
            }
        }
    }

    /// The color of the water at the hour: the sky in it, darker nearer.
    pub fn water_color(&self) -> Rgb {
        if self.kind.snowy() {
            self.lit(Rgb(196, 222, 236))
        } else {
            self.light
                .bot
                .lerp(self.light.top, 0.4)
                .lerp(self.lit(Rgb(36, 84, 96)), 0.55)
        }
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
        #[cfg(test)]
        {
            self.painted = Default::default();
        }
        self.paint_sky(v);
        self.paint_layers(v);
        self.paint_reyes(v);
        self.paint_papel_far(v);
        self.paint_luces_far(v);
        self.paint_banderas_far(v);
        self.paint_ground();
        self.paint_beds(false);
        self.paint_nopales(false);
        self.paint_ruins();
        self.paint_belenes();
        self.paint_emblemas();
        self.paint_castillos();
        self.paint_trees();
        self.paint_papel();
        self.paint_luces();
        self.paint_banderas();
        self.paint_mist(v);
        self.draw_life_back();
        self.draw_tiger();
        self.draw_life_front();
        self.paint_grass();
        self.paint_beds(true);
        self.paint_nopales(true);
        self.paint_front(v);
        self.draw_specks();
        self.draw_ufo();
        self.project(v);
        self.draw_lights(v);
    }

    /// Where the hills begin at the painted column `x`, a world pixel: the
    /// ground's far edge where nothing is painted.
    pub fn ridge(&self, x: i32) -> i32 {
        let i = x - self.bx;
        if i < 0 || i >= self.bw {
            return BACK;
        }
        self.cols[i as usize].hill.min(BACK)
    }

    /// Is the world pixel `(x, y)` of the hills hidden behind the far forest,
    /// the middle layer's canopy or one of its trunks?
    pub fn behind_forest(&self, x: i32, y: i32) -> bool {
        let i = x - self.bx;
        if i < 0 || i >= self.bw {
            return false;
        }
        let c = self.cols[i as usize];
        y >= c.far || y < c.canopy || (c.trunk > 0 && y < BACK + 1)
    }

    /// The sky's color at world row `y`.
    #[inline]
    fn sky_at(&self, y: i32) -> Rgb {
        self.light
            .top
            .lerp(self.light.bot, (y as f32 / BACK as f32).clamp(0.0, 1.0))
    }

    fn paint_sky(&mut self, v: View) {
        let (bw, bh, by) = (self.bw as usize, self.bh, self.by);
        for j in 0..bh {
            let c = if self.set.sky {
                self.sky_at(by + j)
            } else {
                KEY
            };
            let line = j as usize * bw;
            self.buf[line..line + bw].fill(c);
        }
        if self.set.sky {
            self.paint_sun_moon(v);
        }
    }

    /// The sun by day and the moon by night, crossing the sky in view.
    fn paint_sun_moon(&mut self, v: View) {
        use std::f32::consts::PI;
        let r = (v.h / 18.0).max(2.0);
        // Sun from 0.19 to 0.81, moon for the rest of the day.
        let (u, sun) = if (0.19..0.81).contains(&self.t) {
            ((self.t - 0.19) / 0.62, true)
        } else {
            ((self.t - 0.81).rem_euclid(1.0) / 0.38, false)
        };
        let arc = (PI * u).sin();
        let cx = v.x + u * (v.w + 4.0 * r) - 2.0 * r;
        let cy = v.y + v.h * 0.55 - arc * v.h * 0.5;
        let color = if sun {
            Rgb(255, 150, 60).lerp(Rgb(255, 240, 170), (arc * 1.6).min(1.0))
        } else {
            Rgb(236, 232, 212)
        };
        let glow_r = r * if sun { 2.6 } else { 1.8 };
        for y in (cy - glow_r) as i32..=(cy + glow_r) as i32 {
            if y >= BACK {
                break;
            }
            for x in (cx - glow_r) as i32..=(cx + glow_r) as i32 {
                let (dx, dy) = (x as f32 - cx, y as f32 - cy);
                let d = (dx * dx + dy * dy).sqrt();
                if d <= r {
                    // Crescent: carve out a shifted disc.
                    let (ex, ey) = (dx - r * 0.55, dy + r * 0.2);
                    if sun || ex * ex + ey * ey > r * r * 0.8 {
                        self.put(x, y, color);
                    }
                } else if d < glow_r {
                    let a = (1.0 - (d - r) / (glow_r - r)) * if sun { 0.4 } else { 0.18 };
                    self.blend(x, y, color, a);
                }
            }
        }
    }

    /// Where the far forest begins at its layer coordinate `u`.
    fn far_top(&self, u: i64) -> i32 {
        let s = self.seed;
        let mut top = BACK as f32;
        if self.kind.snowy() {
            // Spires of fir, packed close.
            const CELL: i64 = 9;
            let cell = u.div_euclid(CELL) as i32;
            for c in cell - 1..=cell + 1 {
                let cx = c as f32 * CELL as f32 + unit(c, 41, s) * CELL as f32;
                let tip = 48.0 + 14.0 * unit(c, 42, s) + 6.0 * noise(c as f64, 0.1, 43, s);
                top = top.min(tip + (u as f32 - cx).abs() / 0.42);
            }
        } else {
            // Round crowns, shoulder to shoulder.
            const CELL: i64 = 15;
            let cell = u.div_euclid(CELL) as i32;
            for c in cell - 1..=cell + 1 {
                let cx = c as f32 * CELL as f32 + unit(c, 41, s) * CELL as f32;
                let r = 6.0 + 4.0 * unit(c, 42, s);
                let cy = 58.0 + 9.0 * noise(c as f64, 0.13, 43, s);
                let d = u as f32 - cx;
                if d.abs() < r {
                    top = top.min(cy - (r * r - d * d).sqrt());
                }
            }
            top = top.min(64.0 + 6.0 * noise(u as f64, 1.0 / 40.0, 44, s));
        }
        top as i32
    }

    /// The middle layer's tree at layer coordinate `u`: 2 on a trunk's lit
    /// edge, 1 on the trunk, 0 between trunks.
    fn mid_trunk(&self, u: i64) -> u8 {
        let s = self.seed;
        let cell = u.div_euclid(MID_CELL) as i32;
        for c in cell - 1..=cell + 1 {
            if unit(c, 51, s) > 0.62 {
                continue;
            }
            let cx = c as f32 * MID_CELL as f32 + unit(c, 52, s) * MID_CELL as f32;
            let hw = 1.5 + 2.0 * unit(c, 53, s);
            let d = u as f32 + 0.5 - cx;
            if d.abs() <= hw {
                return if d < 1.0 - hw { 2 } else { 1 };
            }
        }
        0
    }

    /// The bottom of the canopy overhead at the middle layer's `u`.
    fn canopy(&self, u: i64) -> i32 {
        let s = self.seed;
        let base = 10.0
            + 16.0 * noise(u as f64, 1.0 / 56.0, 55, s)
            + 5.0 * noise(u as f64, 1.0 / 13.0, 56, s);
        base as i32
    }

    /// The forest behind the tiger's ground: hills, the far forest, and the
    /// middle layer of trunks, canopy and lianas.
    fn paint_layers(&mut self, v: View) {
        let (bx, bw, by, bh) = (self.bx, self.bw, self.by, self.bh);
        let (sh_h, sh_f, sh_m) = (
            self.shift(v, HILL_P),
            self.shift(v, FAR_P),
            self.shift(v, MID_P),
        );
        let snowy = self.kind.snowy();
        let s = self.seed;
        for i in 0..bw {
            let x = (bx + i) as i64;
            let (uh, uf, um) = (x + sh_h, x + sh_f, x + sh_m);
            let hill = 36.0
                + 18.0 * noise(uh as f64, 1.0 / 150.0, 31, s)
                + 6.0 * noise(uh as f64, 1.0 / 41.0, 32, s);
            self.cols[i as usize] = Col {
                hill: hill as i32,
                far: self.far_top(uf),
                canopy: if snowy { -1 } else { self.canopy(um) },
                trunk: if snowy { 0 } else { self.mid_trunk(um) },
                uf: uf as i32,
                um: um as i32,
                water: 0,
            };
        }
        let green = |c: Rgb| self.green(c);
        let hill_c = self.far(green(Rgb(86, 120, 110)), 0.62);
        let hill_cap = self.far(Rgb(236, 240, 246), 0.45);
        let far_c = self.far(green(Rgb(40, 84, 52)), 0.42);
        let far_lit = self.far(green(Rgb(70, 122, 66)), 0.42);
        let far_snow = self.far(Rgb(226, 234, 242), 0.35);
        let leaf = self.far(green(Rgb(30, 72, 40)), 0.2);
        let leaf_lit = self.far(green(Rgb(58, 112, 54)), 0.2);
        let leaf_dark = self.far(green(Rgb(20, 50, 30)), 0.2);
        let bark = self.far(Rgb(84, 70, 58), 0.24);
        let bark_lit = self.far(Rgb(118, 100, 82), 0.24);
        for j in 0..bh {
            let y = by + j;
            if y >= BACK + 2 {
                break;
            }
            let line = (j * bw) as usize;
            for i in 0..bw {
                let c = self.cols[i as usize];
                let px = if y < c.canopy {
                    match hash(c.um, y, s) % 7 {
                        0 | 1 => leaf_lit,
                        2 => leaf_dark,
                        _ => leaf,
                    }
                } else if c.trunk > 0 && y < BACK + 1 {
                    if c.trunk == 2 { bark_lit } else { bark }
                } else if y >= c.far {
                    if snowy {
                        // Snow caught on the firs, here and there.
                        if y == c.far || hash(c.uf, y, s).is_multiple_of(6) {
                            far_snow
                        } else {
                            far_c
                        }
                    } else if y <= c.far + 1 || hash(c.uf, y, s).is_multiple_of(9) {
                        far_lit
                    } else {
                        far_c
                    }
                } else if y >= c.hill {
                    if snowy && y < c.hill + 3 {
                        hill_cap
                    } else {
                        hill_c
                    }
                } else {
                    continue;
                };
                self.buf[line + i as usize] = px;
            }
        }
        if snowy {
            self.paint_firs(sh_m);
        } else {
            self.paint_lianas(sh_m);
        }
    }

    /// Lianas hanging from the canopy of the middle layer, swaying a little.
    fn paint_lianas(&mut self, sh: i64) {
        let s = self.seed;
        let time = self.time();
        let vine = self.far(self.green(Rgb(46, 86, 38)), 0.2);
        let leaf = self.far(self.green(Rgb(76, 130, 60)), 0.2);
        let (u0, u1) = (self.bx as i64 + sh, (self.bx + self.bw) as i64 + sh);
        let (c0, c1) = (u0.div_euclid(LIANA_CELL) - 1, u1.div_euclid(LIANA_CELL) + 1);
        for c in c0..=c1 {
            let ci = c as i32;
            if unit(ci, 61, s) > 0.5 {
                continue;
            }
            let ux = c * LIANA_CELL + (unit(ci, 62, s) * LIANA_CELL as f32) as i64;
            let top = self.canopy(ux);
            let len = 18 + (unit(ci, 63, s) * 46.0) as i32;
            let phase = unit(ci, 64, s) * 6.3;
            let sway = 1.2 * (time * 0.5 + phase).sin();
            for k in 0..len {
                let y = top + k;
                let f = k as f32 / len as f32;
                let x = (ux - sh) as f32 + 2.0 * (y as f32 * 0.09 + phase).sin() + sway * f * f;
                let xi = x.round() as i32;
                self.put(xi, y, vine);
                if k % 5 == 2 {
                    self.put(xi + if k % 10 == 2 { 1 } else { -1 }, y, leaf);
                }
            }
        }
    }

    /// The taiga's middle layer: tall firs, snow on every tier.
    fn paint_firs(&mut self, sh: i64) {
        let s = self.seed;
        let green = self.far(self.green(Rgb(30, 64, 46)), 0.22);
        let dark = self.far(self.green(Rgb(20, 44, 34)), 0.22);
        let snow = self.far(Rgb(236, 242, 250), 0.18);
        let bark = self.far(Rgb(96, 70, 54), 0.22);
        let (u0, u1) = (self.bx as i64 + sh, (self.bx + self.bw) as i64 + sh);
        const CELL: i64 = 30;
        let (c0, c1) = (u0.div_euclid(CELL) - 1, u1.div_euclid(CELL) + 1);
        for c in c0..=c1 {
            let ci = c as i32;
            if unit(ci, 57, s) > 0.7 {
                continue;
            }
            let cx = (c * CELL - sh) as f32 + unit(ci, 58, s) * CELL as f32;
            let tip = 14 + (unit(ci, 59, s) * 26.0) as i32;
            let foot = BACK - 8 - (unit(ci, 60, s) * 8.0) as i32;
            let tall = 6 + (unit(ci, 65, s) * 3.0) as i32;
            for y in tip..BACK + 1 {
                let d = (y - tip) as f32;
                if y < foot {
                    // Tiers: each widens, then steps back in, with snow
                    // lying on its outer branches.
                    let tier = (d as i32) % tall;
                    let hw = 1.0 + d * 0.3 + tier as f32 * 0.5;
                    for x in (cx - hw).round() as i32..=(cx + hw).round() as i32 {
                        let out = (x as f32 - cx).abs() / hw.max(1.0);
                        let h = hash(x + ci * 17, y, s);
                        let c = if (tier <= 1 && out > 0.45 && !h.is_multiple_of(3))
                            || h.is_multiple_of(19)
                        {
                            snow
                        } else if (x as f32) < cx {
                            dark
                        } else {
                            green
                        };
                        self.put(x, y, c);
                    }
                } else {
                    for x in cx.round() as i32 - 1..=cx.round() as i32 {
                        self.put(x, y, bark);
                    }
                }
            }
        }
    }

    /// The ground the tiger walks on, with its rivers and pools.
    fn paint_ground(&mut self) {
        let (bx, bw, by, bh) = (self.bx, self.bw, self.by, self.bh);
        let s = self.seed;
        let snowy = self.kind.snowy();
        let time = self.time();
        // Water by column: 1 for a bank, 2 for water.
        let mut spans: Vec<Span> = Vec::new();
        let (u0, u1) = (
            self.base as f64 + bx as f64,
            self.base as f64 + (bx + bw) as f64,
        );
        for c in (u0 / WATER_CELL as f64).floor() as i64 - 1
            ..=(u1 / WATER_CELL as f64).floor() as i64 + 1
        {
            if let Some(sp) = self.span_in(c) {
                spans.push(sp);
            }
        }
        for i in 0..bw {
            let x = (bx + i) as f32 + 0.5;
            let w = spans.iter().find_map(|sp| {
                (x >= sp.a - 2.0 && x < sp.b + 2.0).then_some(if x < sp.a || x >= sp.b {
                    1
                } else {
                    2
                })
            });
            self.cols[i as usize].water = w.unwrap_or(0);
        }
        let (far_c, near_c) = if snowy {
            (Rgb(226, 234, 244), Rgb(208, 218, 232))
        } else {
            (self.green(Rgb(126, 136, 66)), Rgb(122, 92, 58))
        };
        let (fleck_a, fleck_b) = if snowy {
            (Rgb(180, 196, 222), Rgb(250, 252, 255))
        } else {
            (Rgb(170, 120, 54), self.green(Rgb(84, 112, 50)))
        };
        let bank = if snowy {
            Rgb(200, 210, 224)
        } else {
            Rgb(92, 72, 50)
        };
        let water = self.water_color();
        let glint = water.lerp(Rgb::WHITE, if snowy { 0.5 } else { 0.4 });
        let slow = (time * 3.0) as i32;
        for j in 0..bh {
            let y = by + j;
            if y < BACK {
                continue;
            }
            let d = (y - BACK) as f32 / (WORLD_H - BACK) as f32;
            let base = self.lit(far_c.lerp(near_c, d.powf(0.8)));
            let (fa, fb) = (self.lit(fleck_a), self.lit(fleck_b));
            let deep = water.scale(1.0 - 0.25 * d);
            let line = (j * bw) as usize;
            for i in 0..bw {
                let u = (self.base + (bx + i) as i64) as i32;
                let c = match self.cols[i as usize].water {
                    2 => {
                        // Ripples glint here and there; on ice, cracks.
                        let h = hash(
                            u.wrapping_add(if snowy { 0 } else { slow / 2 }),
                            y,
                            s ^ 0x77,
                        );
                        if (snowy && h.is_multiple_of(31)) || (!snowy && h.is_multiple_of(37)) {
                            glint
                        } else {
                            deep
                        }
                    }
                    1 => self.lit(bank),
                    _ => match hash(u, y, s) % 13 {
                        0 => fa,
                        1 | 2 => fb,
                        3 => base.scale(0.9),
                        _ => base,
                    },
                };
                self.buf[line + i as usize] = c;
            }
        }
    }

    /// The big trees on the tiger's ground: trunks, limbs, crowns, claw marks.
    fn paint_trees(&mut self) {
        let trunks = self.trunks(self.bx as f32, (self.bx + self.bw) as f32);
        let snowy = self.kind.snowy();
        let s = self.seed;
        let (bark, bark_dark, bark_lit) = if snowy {
            (Rgb(150, 92, 58), Rgb(100, 60, 40), Rgb(190, 126, 80))
        } else {
            (Rgb(104, 88, 72), Rgb(70, 58, 48), Rgb(140, 120, 98))
        };
        let (bark, bark_dark, bark_lit) = (self.lit(bark), self.lit(bark_dark), self.lit(bark_lit));
        let leaf = self.lit(self.green(Rgb(36, 92, 44)));
        let leaf_lit = self.lit(self.green(Rgb(72, 136, 60)));
        let leaf_dark = self.lit(self.green(Rgb(24, 62, 32)));
        let snow = self.lit(Rgb(244, 248, 255));
        let claw = self.lit(Rgb(220, 196, 160));
        let time = self.time();
        for t in trunks {
            let x0 = t.x.round() as i32 - t.w / 2;
            // Roots flare out at the foot.
            for y in 0..=FEET + 1 {
                let flare = ((y - (FEET - 5)).max(0) as f32 * 0.9) as i32;
                for x in x0 - flare..x0 + t.w + flare {
                    let k = x - x0;
                    let c = if k <= 0 {
                        bark_lit
                    } else if k >= t.w - 2 || hash(x + t.id as i32 * 31, y / 3, s).is_multiple_of(5)
                    {
                        bark_dark
                    } else {
                        bark
                    };
                    self.put(x, y, c);
                }
            }
            // Claw marks, once the tiger has raked it.
            if self.marks.contains(&t.id) {
                for k in 0..3 {
                    for d in 0..9 {
                        self.put(x0 + 2 + k * 2 + d / 4, FEET - 34 + d + k, claw);
                    }
                }
            }
            for &(h, len, side) in &t.limbs {
                let y0 = FEET - h - 10;
                let start = if side > 0 { x0 + t.w } else { x0 };
                for k in 0..len {
                    let x = start + side * k;
                    let y = y0 - k / 3;
                    let thick = if k < len / 2 { 3 } else { 2 };
                    for d in 0..thick {
                        self.put(x, y + d, if d == 0 { bark_lit } else { bark });
                    }
                    if snowy {
                        self.put(x, y - 1, snow);
                    }
                }
                // Leaves at the end of the limb.
                if !snowy {
                    let (lx, ly) = (start + side * len, y0 - len / 3);
                    let sway = (time * 0.7 + t.id as f32).sin();
                    for dy in -5..=4 {
                        for dx in -6..=6 {
                            if dx * dx + dy * dy * 2 <= 36 {
                                let c = match hash(lx + dx + t.id as i32, ly + dy, s) % 5 {
                                    0 => leaf_lit,
                                    1 => leaf_dark,
                                    _ => leaf,
                                };
                                self.put(
                                    lx + dx + (sway * (5 - dy) as f32 * 0.15) as i32,
                                    ly + dy,
                                    c,
                                );
                            }
                        }
                    }
                }
            }
            // The crown, filling the top of the world.
            let cx = t.x;
            let reach = 18.0 + t.w as f32 * 1.5;
            for y in 0..22 {
                let hw = reach * (1.0 - (y as f32 / 26.0).powi(2)).max(0.0);
                for x in (cx - hw) as i32..=(cx + hw) as i32 {
                    let h = hash(x + t.id as i32 * 7, y, s) % 6;
                    let c = if snowy {
                        if y % 5 == 0 || h == 0 {
                            snow
                        } else {
                            leaf_dark
                        }
                    } else {
                        match h {
                            0 => leaf_lit,
                            1 => leaf_dark,
                            _ => leaf,
                        }
                    };
                    self.put(x, y, c);
                }
            }
        }
    }

    /// Dawn mist lying low among the trees.
    fn paint_mist(&mut self, v: View) {
        let t = self.t;
        let k = if (0.17..0.42).contains(&t) {
            let rise = ((t - 0.17) / 0.06).min(1.0);
            let fall = ((0.42 - t) / 0.12).min(1.0);
            rise.min(fall).max(0.0)
        } else {
            0.0
        };
        if k <= 0.0 {
            return;
        }
        let mist = self.light.bot.lerp(Rgb::WHITE, 0.55);
        let sh = self.shift(v, 0.8);
        let time = self.time();
        let s = self.seed;
        let (bx, bw, by, bh) = (self.bx, self.bw, self.by, self.bh);
        for i in 0..bw {
            let u = (bx + i) as i64 + sh;
            let wisp = noise(u as f64 + time as f64 * 2.0, 1.0 / 30.0, 66, s);
            for j in 0..bh {
                let y = by + j;
                if !(54..=FEET + 6).contains(&y) {
                    continue;
                }
                let band = 1.0 - ((y - (BACK - 6)) as f32 / 30.0).abs().min(1.0);
                let a = k * 0.6 * band * (0.45 + 0.55 * wisp);
                if a > 0.02 {
                    self.blend(bx + i, y, mist, a);
                }
            }
        }
    }

    /// Patches of tall grass on the tiger's ground, in front of it.
    fn paint_grass(&mut self) {
        let (bx, bw) = (self.bx, self.bw);
        let (u0, u1) = (
            self.base as f64 + bx as f64,
            self.base as f64 + (bx + bw) as f64,
        );
        let s = self.seed;
        let snowy = self.kind.snowy();
        let time = self.time();
        let wind = 0.8 + 0.6 * (time * 0.13).sin();
        let blades: [Rgb; 4] = if snowy {
            [
                Rgb(176, 156, 108),
                Rgb(150, 128, 88),
                Rgb(196, 178, 128),
                Rgb(130, 112, 80),
            ]
        } else {
            [
                self.green(Rgb(96, 150, 56)),
                self.green(Rgb(70, 126, 48)),
                Rgb(170, 160, 76),
                self.green(Rgb(58, 108, 40)),
            ]
        };
        let blades = blades.map(|c| self.lit(c));
        let tip = self.lit(if snowy {
            Rgb(246, 248, 255)
        } else {
            Rgb(206, 196, 110)
        });
        let mut patches: Vec<(f32, f32, f32)> = Vec::new();
        for c in (u0 / GRASS_CELL as f64).floor() as i64 - 1
            ..=(u1 / GRASS_CELL as f64).floor() as i64 + 1
        {
            if let Some(p) = self.grass_in(c) {
                patches.push(p);
            }
        }
        if let Some(p) = self.tiger.cover() {
            patches.push(p);
        }
        for (a, b, tall) in patches {
            for x in a.ceil() as i32..b.floor() as i32 {
                let u = (self.base + x as i64) as i32;
                // Thinner and shorter toward the patch's ends.
                let edge = ((x as f32 - a).min(b - x as f32) / 6.0).min(1.0);
                for k in 0..2 {
                    let h = hash(u, k, s ^ 0x6a55);
                    if h % 10 < 3 {
                        continue;
                    }
                    let base = FEET + 1 + (h >> 8) as i32 % 4;
                    let len =
                        (tall * edge * (0.55 + 0.45 * ((h >> 12) % 100) as f32 / 100.0)) as i32;
                    let sway = wind * (time * 1.4 + u as f32 * 0.23).sin() * 1.6;
                    let color = blades[((h >> 4) % 4) as usize];
                    for r in 0..len.max(1) {
                        let f = r as f32 / len.max(1) as f32;
                        let dx = (sway * f * f).round() as i32;
                        let c = if r == len - 1 { tip } else { color };
                        self.put(x + dx, base - r, c);
                    }
                    // On a holiday of flowers some of the blades flower.
                    if let Some(level) = self.pal.bloom_level()
                        && (h >> 16) % 100 < (14.0 + 22.0 * level) as u32
                    {
                        let top = len.max(1);
                        let f = (top - 1) as f32 / top as f32;
                        let dx = (sway * f * f).round() as i32;
                        self.bloom(x + dx, base - top - 1, h >> 9, false);
                    }
                }
            }
        }
    }

    /// Ferns and grass right in front of the camera, at the bottom.
    fn paint_front(&mut self, v: View) {
        let sh = self.shift(v, FRONT_P);
        let s = self.seed;
        let snowy = self.kind.snowy();
        let dark = self.lit(if snowy {
            Rgb(160, 176, 200)
        } else {
            self.green(Rgb(30, 70, 34))
        });
        let mid = self.lit(if snowy {
            Rgb(200, 212, 230)
        } else {
            self.green(Rgb(44, 92, 42))
        });
        let (bx, bw) = (self.bx, self.bw);
        for i in 0..bw {
            let u = (bx + i) as i64 + sh;
            let n = noise(u as f64, 1.0 / 11.0, 68, s);
            let h = (n * n * 9.0) as i32 + (hash(u as i32, 0, s) % 2) as i32;
            for r in 0..h {
                let c = if r == h - 1 { mid } else { dark };
                self.put(bx + i, WORLD_H - 1 - r, c);
            }
        }
    }

    /// Resamples the painted world onto the screen: box-filtered when zoomed
    /// out, nearest pixel when zoomed in. See-through sky stays empty.
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
                    if wi < self.weights.len() {
                        self.weights[wi] = 256;
                    }
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
                    // no division unless see-through sky is part of it.
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
        let clear = !self.set.sky;
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
                self.scr.set(sx, sy, c.map_or(Color::Reset, col));
            }
        }
    }

    /// Mixes `c` into a screen pixel; nothing over see-through sky.
    #[inline]
    pub fn blend_out(&mut self, x: i32, y: i32, c: Rgb, a: f32) {
        if let Some(p) = rgb_of(self.scr.get(x, y)) {
            self.scr.set(x, y, col(p.lerp(c, a)));
        }
    }

    /// Keeps fireflies (or snowflakes) drifting around the camera.
    fn update_motes(&mut self, rng: &mut Rng) {
        let (vw, vh) = self.view_size();
        let snowy = self.kind.snowy();
        let want = if snowy {
            (vw * vh / 40.0) as usize
        } else {
            (vw / 6.0) as usize
        }
        .clamp(4, 400);
        let (left, top) = (self.cam_x - vw / 2.0, WORLD_H as f32 - vh);
        while self.motes.len() < want {
            self.motes.push(Mote {
                x: left + rng.rangef(0.0, vw),
                y: if snowy {
                    rng.rangef(top - 4.0, WORLD_H as f32)
                } else {
                    rng.rangef(FEET as f32 - 40.0, FEET as f32 + 4.0)
                },
                vx: 0.0,
                vy: 0.0,
                phase: rng.rangef(0.0, 6.3),
            });
        }
        self.motes.truncate(want);
        for m in &mut self.motes {
            if snowy {
                m.vx = 0.15 * (m.phase + self.tick as f32 * 0.02).sin() - 0.05;
                m.vy = 0.22 + 0.1 * (m.phase * 3.0).fract();
            } else {
                m.vx = (m.vx + rng.rangef(-0.04, 0.04)).clamp(-0.3, 0.3);
                m.vy = (m.vy + rng.rangef(-0.03, 0.03)).clamp(-0.15, 0.15);
            }
            m.x += m.vx;
            m.y += m.vy;
            // Wrap round the view, so there are always some in it.
            if m.x < left {
                m.x += vw;
            } else if m.x > left + vw {
                m.x -= vw;
            }
            if snowy {
                if m.y > WORLD_H as f32 {
                    m.y = top - 2.0;
                }
            } else if m.y < FEET as f32 - 44.0 || m.y > FEET as f32 + 6.0 {
                m.vy = -m.vy;
                m.y = m.y.clamp(FEET as f32 - 44.0, FEET as f32 + 6.0);
            }
        }
    }

    /// Point lights go straight onto the screen so zooming out cannot blur
    /// them away: stars, fireflies, snowflakes, the saucer's lights.
    fn draw_lights(&mut self, v: View) {
        let tick = self.tick as f32;
        let night = 1.0 - self.daylight;
        let dark = ((night - 0.5) * 2.0).clamp(0.0, 1.0);
        if dark > 0.0 && self.set.sky {
            for i in 0..self.stars.len() {
                let (fx, fy, ph) = self.stars[i];
                let (sx, sy) = ((fx * self.sw as f32) as i32, (fy * self.sph as f32) as i32);
                // Only where the sky shows: the world pixel under it is sky.
                let wx = (v.x + (sx as f32 + 0.5) / v.z).floor() as i32;
                let wy = (v.y + (sy as f32 + 0.5) / v.z).floor() as i32;
                let (i0, j0) = (wx - self.bx, wy - self.by);
                if i0 < 0 || j0 < 0 || i0 >= self.bw || j0 >= self.bh {
                    continue;
                }
                if self.buf[(j0 * self.bw + i0) as usize] != self.sky_at(wy) {
                    continue;
                }
                let a = dark * (0.55 + 0.45 * (tick * 0.07 + ph).sin());
                self.blend_out(sx, sy, Rgb(230, 230, 255), a);
            }
        }
        if self.kind.snowy() {
            let flake = Rgb(250, 252, 255);
            for i in 0..self.motes.len() {
                let m = self.motes[i];
                let (x, y) = v.to_screen(m.x, m.y);
                self.blend_out(x, y, flake, 0.75);
            }
        } else if night > 0.05 && !self.ufo.hushed() {
            // The fireflies of Día de Muertos are warm, like the candles, at
            // Navidad they are foquitos of every color, on Fiestas Patrias
            // they glow in verde, blanco y rojo, and at the Equinoccio in the
            // jade of the quetzal's feathers.
            let glow = if self.pal.fest.is_some() {
                Rgb(255, 200, 90)
            } else {
                Rgb(210, 255, 110)
            };
            for i in 0..self.motes.len() {
                let m = self.motes[i];
                let glow = if let Some(d) = self.pal.navidad {
                    d.bulbs[(m.phase * 10.0) as usize % 6]
                } else if let Some(d) = self.pal.patrias {
                    [d.verde.lerp(Rgb::WHITE, 0.35), d.blanco, d.rojo]
                        [(m.phase * 10.0) as usize % 3]
                } else if let Some(d) = self.pal.equinoccio {
                    d.jade.lerp(Rgb::WHITE, 0.3)
                } else {
                    glow
                };
                let b = night * (tick * 0.05 + m.phase).sin().max(0.0).powi(3);
                let (x, y) = v.to_screen(m.x, m.y);
                self.blend_out(x, y, glow, b);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    self.blend_out(x + dx, y + dy, glow, b * 0.25);
                }
            }
        }
        self.draw_ufo_lights(v);
        self.glow_lights(v);
        self.glow_belenes(v);
        self.glow_castillos(v);
    }

    /// Notes as letters over the picture: z's, alarms.
    pub fn draw_marks(&self, c: &mut Canvas) {
        let v = self.view();
        for n in &self.notes {
            let (x, y) = v.to_screen(n.x, n.y);
            let fade = if n.age + 12 > n.span {
                (n.span - n.age) as f32 / 12.0
            } else {
                1.0
            };
            for (cx, ch) in (x - n.text.len() as i32 / 2..).zip(n.text.chars()) {
                let below = self.scr.get(cx, y);
                let fg = rgb_of(below).map_or(n.color, |b| b.lerp(n.color, fade));
                if below != Color::Reset {
                    c.set_bg(cx, y >> 1, below);
                }
                c.set(cx, y >> 1, ch, col(fg));
            }
        }
    }
}
