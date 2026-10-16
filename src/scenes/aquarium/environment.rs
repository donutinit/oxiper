//! Light, depth, current and weather.
//!
//! The day moves between light and dark, far creatures fade into the
//! background, a slow current sways the plants and carries drifting things,
//! and now and then a storm passes over. Everything here only runs beyond
//! `classic` (`pen_table` and `water_pen` hand back the classic pens there).

use std::f32::consts::{PI, TAU};

use super::assets::{DEEP, GLOW, LEGENDARY, NOCTURNAL};
use super::biome::{between, hash};
use super::*;

/// Stars, bright to faint.
const STARS: [char; 3] = ['✦', '✧', '∙'];
/// Glow sparks around luminous creatures, bright to faint.
const SPARKS: [char; 3] = ['✦', '✧', '∙'];
/// The sun's disc, and its two sets of rays (straight, diagonal) at `SUN_RAY_AT`.
const SUN: [&str; 2] = ["◢█◣", "◥█◤"];
const SUN_RAY_AT: [(i32, i32); 10] = [
    (1, -1),
    (1, 2),
    (-1, 0),
    (-1, 1),
    (3, 0),
    (3, 1),
    (-1, -1),
    (3, -1),
    (-1, 2),
    (3, 2),
];
const SUN_RAYS: [[char; 10]; 2] = [
    ['│', '│', '─', '─', '─', '─', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', '╲', '╱', '╱', '╲'],
];
/// Moon phases (two rows each): full, gibbous and half lit on the right, then
/// the waning ones.
const MOON: [[&str; 2]; 5] = [
    ["◢██◣", "◥██◤"],
    ["  ◢█", "  ◥█"],
    ["█◣  ", "█◤  "],
    [" ◢██", " ◥██"],
    ["██◣ ", "██◤ "],
];
/// Two shapes of cloud, two rows each, soft at the edges.
const CLOUDS: [[&str; 2]; 2] = [
    ["  ▗▄██▄▖  ", "░▒██████▒░"],
    ["   ▄██▄▖   ", "░▒▓██████▓▒░"],
];
/// The same clouds at night: thin and dim.
const CLOUDS_NIGHT: [[&str; 2]; 2] = [["  ░░░░   ", "░░░▒▒░░░░"], ["   ░░░░░   ", "░░░░▒▒░░░░"]];
/// Rain: slanting right, slanting left, a short drop, a streak.
const RAIN: [char; 4] = ['╲', '╱', '╎', '│'];
/// Lightning: leaning left, leaning right, halo.
const BOLT: [char; 3] = ['╱', '╲', '░'];

/// Fraction of the day cycle (0 is midnight): the night is dark within this
/// distance of midnight, and the dawn and dusk take the next stretch.
const NIGHT_HALF: f32 = 0.15;
const TWILIGHT: f32 = 0.075;
/// Seconds a storm takes to build up and to die down.
const STORM_RAMP: f32 = 6.0;
/// Current (cells per second) that bends the plants, and that lets them go.
const LEAN_ON: f32 = 0.7;
const LEAN_OFF: f32 = 0.35;
/// Rows the rain falls through (the sky, the waves and the top of the water).
const RAIN_ROWS: i32 = 12;

/// The environment's own state.
#[derive(Default)]
pub(super) struct Env {
    /// Seconds for a whole day and night; 0 is always day.
    pub(super) day_length: f32,
    /// Share of creatures in the far layer.
    pub(super) depth: f32,
    /// Strength of the current.
    pub(super) current: f32,
    /// Storms on or off.
    pub(super) weather: bool,
    /// Seconds between storms (shortest, longest) and how long one lasts.
    pub(super) gap: (f32, f32),
    pub(super) span: (f32, f32),
    started: bool,
    /// Time of day, 0 to 1, with 0 at midnight.
    phase: f32,
    /// Daylight before the weather, 0 to 1.
    daylight: f32,
    t: f32,
    // Current.
    cur: f32,
    ph: [f32; 2],
    gust: f32,
    gust_cd: f32,
    calm: f32,
    calm_goal: f32,
    calm_cd: f32,
    /// Which way the plants lean: -1, 0 or 1.
    lean: i8,
    // Storm.
    storm_cd: f32,
    storm_left: f32,
    level: f32,
    dir: f32,
    bolt_cd: f32,
    flash: f32,
    bolt_x: i32,
    bolt_seed: u64,
    wave_phase: f64,
    // Pens, from the last retheme.
    ready: bool,
    native: bool,
    last_key: u32,
    day: [(Color, u8); 16],
    dusk: [(Color, u8); 16],
    night: [(Color, u8); 16],
    far_day: [(Color, u8); 16],
    far_night: [(Color, u8); 16],
    far: [(Color, u8); 16],
    water_day: [(Color, u8); 4],
    water_night: [(Color, u8); 4],
    water_flash: [(Color, u8); 4],
}

impl Env {
    pub(super) fn from_opts(o: &crate::scene::Opts) -> Env {
        let num = |k: &str, max: f32, def: f32| {
            let v = o.f(k);
            if v.is_finite() {
                v.clamp(0.0, max)
            } else {
                def
            }
        };
        Env {
            day_length: num("day_length", 1e6, 900.0),
            depth: num("depth", 1.0, 0.3),
            current: num("current", 10.0, 1.0),
            weather: o.b("weather"),
            gap: (300.0, 900.0),
            span: (40.0, 90.0),
            daylight: 1.0,
            calm: 1.0,
            calm_goal: 1.0,
            ..Env::default()
        }
    }

    /// Daylight at time of day `p`: 0 at night, 1 by day, smooth between.
    fn day_light(p: f32) -> f32 {
        let d = p.min(1.0 - p);
        let k = ((d - NIGHT_HALF) / TWILIGHT).clamp(0.0, 1.0);
        k * k * (3.0 - 2.0 * k)
    }

    /// Advances the clock, the weather and the current.
    /// `dt` is scene time (the current, the twinkle); `real` is wall-clock
    /// time, which the day and the storms follow.
    fn tick(&mut self, rng: &mut fastrand::Rng, dt: f32, real: f32, surface: bool) {
        if !self.started {
            self.started = true;
            self.phase = rng.f32();
            self.ph = [rng.f32() * TAU, rng.f32() * TAU];
            self.storm_cd = between(rng, self.gap.0 * 0.3, self.gap.1);
            self.gust_cd = between(rng, 10.0, 40.0);
            self.calm_cd = between(rng, 30.0, 90.0);
            self.bolt_cd = 1.0;
        }
        self.t += dt;
        if self.day_length > 0.0 {
            self.phase = (self.phase + real / self.day_length).rem_euclid(1.0);
            self.daylight = Env::day_light(self.phase);
        } else {
            self.phase = 0.5;
            self.daylight = 1.0;
        }
        // Storm.
        if !(self.weather && surface) {
            self.storm_left = 0.0;
            self.level = 0.0;
        } else if self.storm_left > 0.0 {
            self.storm_left -= real;
        } else {
            self.storm_cd -= real;
            if self.storm_cd <= 0.0 {
                self.storm_left = between(rng, self.span.0, self.span.1);
                self.storm_cd = between(rng, self.gap.0, self.gap.1);
                self.dir = if rng.bool() { 1.0 } else { -1.0 };
            }
        }
        let goal = if self.storm_left > 0.0 { 1.0 } else { 0.0 };
        let step = real / STORM_RAMP;
        self.level = if (goal - self.level).abs() <= step {
            goal
        } else {
            self.level + step * (goal - self.level).signum()
        };
        self.flash = (self.flash - real).max(0.0);
        if self.level > 0.6 {
            self.bolt_cd -= real;
            if self.bolt_cd <= 0.0 {
                self.bolt_cd = between(rng, 3.0, 12.0);
                self.flash = 0.12;
                self.bolt_x = rng.i32(0..1000);
                self.bolt_seed = rng.u64(..);
            }
        }
        self.wave_phase += (dt * self.level * 1.5) as f64;
        if self.wave_phase > 1e6 {
            self.wave_phase = 0.0;
        }
        // Current: two slow waves, gusts and calm spells.
        self.gust_cd -= dt;
        if self.gust_cd <= 0.0 {
            let sign = if rng.bool() { 1.0 } else { -1.0 };
            self.gust = sign * between(rng, 0.6, 1.0);
            self.gust_cd = between(rng, 25.0, 70.0);
        }
        self.gust *= (1.0 - dt / 4.0).max(0.0);
        self.calm_cd -= dt;
        if self.calm_cd <= 0.0 {
            if self.calm_goal > 0.5 {
                self.calm_goal = 0.1;
                self.calm_cd = between(rng, 12.0, 35.0);
            } else {
                self.calm_goal = 1.0;
                self.calm_cd = between(rng, 40.0, 120.0);
            }
        }
        self.calm += (self.calm_goal - self.calm) * (dt * 0.4).min(1.0);
        let w1 = (self.t * TAU / 83.0 + self.ph[0]).sin();
        let w2 = (self.t * TAU / 37.0 + self.ph[1]).sin();
        let raw = ((0.75 * w1 + 0.45 * w2 + self.gust) * self.calm).clamp(-1.5, 1.5);
        self.cur = (raw + self.dir * self.level * 1.2) * self.current;
        // Plants lean when it is strong and let go when it fades.
        let a = self.cur.abs();
        let sign = if self.cur > 0.0 { 1 } else { -1 };
        self.lean = if a < LEAN_OFF {
            0
        } else if a > LEAN_ON {
            sign
        } else {
            self.lean
        };
    }

    /// Daylight now, with the storm's shadow; 0 in the deep.
    fn light(&self, deep: bool) -> f32 {
        if deep {
            0.0
        } else {
            self.daylight * (1.0 - 0.4 * self.level)
        }
    }
}

/// A color that can be mixed: palette colors become RGB, and the terminal's
/// default (which cannot be dimmed) becomes grey.
fn mixable(c: Color) -> Color {
    match c {
        Color::Ansi(n) => {
            let (r, g, b) = color::ansi_rgb(n);
            Color::Rgb(r, g, b)
        }
        Color::Reset => Color::Rgb(170, 170, 170),
        c => c,
    }
}

/// `c` faded by `k` toward the background, then tinted a little toward moonlight.
fn moonlit(ctx: &Ctx, c: Color, k: f32, tint: f32) -> Color {
    let c = ctx.fade(mixable(c), k);
    c.lerp(Color::Rgb(40, 60, 120), tint)
}

fn mix<const N: usize>(
    a: &[(Color, u8); N],
    b: &[(Color, u8); N],
    t: f32,
    out: &mut [(Color, u8); N],
) {
    for i in 0..N {
        out[i] = (a[i].0.lerp(b[i].0, t), a[i].1);
    }
}

/// How far along its arc (0 to 1) a body that is up for `len` of the cycle,
/// from `from` on, is at time of day `p`; `None` while it is down.
fn arc(p: f32, from: f32, len: f32) -> Option<f32> {
    let f = (p - from).rem_euclid(1.0) / len;
    (f <= 1.0).then_some(f)
}

impl Aquarium {
    /// Once per step, before anything moves.
    pub(super) fn environment(&mut self, dt: f32) {
        let surface = self.surface();
        let Some(w) = self.world.as_mut() else {
            return;
        };
        let env = &mut w.env;
        env.tick(&mut self.rng, dt, dt * self.clock, surface);
        let (cur, lean) = (env.cur, env.lean);
        for e in &mut self.ents {
            if e.dead {
                continue;
            }
            match e.kind {
                Kind::Bubble => e.x += cur * dt * 0.7,
                Kind::Food => e.x += cur * dt * 0.8,
                Kind::Ink => e.x += cur * dt,
                Kind::Fish | Kind::Critter
                    if matches!(
                        e.motion,
                        Motion::Swim {
                            style: assets::Swim::Drift,
                            ..
                        }
                    ) =>
                {
                    e.x += cur * dt * 0.3;
                }
                Kind::Seaweed if lean != 0 => {
                    let n = sprites().get(e.spr as usize).map_or(1, |s| s.frame_count());
                    if n >= 2 {
                        // The engine adds `fps * dt` after this: aim a little
                        // short of the end of the frame so the sum lands in it.
                        let k = if lean > 0 { (n - 1) as f32 } else { 0.0 };
                        e.frame = k + 0.98 - e.fps * dt;
                    }
                }
                _ => {}
            }
        }
    }

    /// After a theme change (also in classic, where it does nothing).
    pub(super) fn retheme_world(&mut self, ctx: &Ctx) {
        let Some(w) = self.world.as_mut() else {
            return;
        };
        let env = &mut w.env;
        env.native = ctx.theme.native;
        env.day = self.pens;
        env.water_day = self.water;
        if env.native {
            env.dusk = self.pens.map(|(c, a)| (c, a & !BOLD));
            for i in 0..16 {
                let n = (i & 7) as u8;
                let bright = i >= 8;
                env.night[i] = match n {
                    K => (Color::Ansi(8), 0),
                    W if bright => (Color::Ansi(7), 0),
                    W => (Color::Ansi(B), 0),
                    n => (Color::Ansi(n), 0),
                };
                env.far_day[i] = match n {
                    B | C => (Color::Ansi(B), 0),
                    _ => (Color::Ansi(8), 0),
                };
            }
            env.far_night = env.far_day;
            env.water_night = [(Color::Ansi(B), 0); 4];
            env.water_flash = [(Color::Ansi(W), BOLD); 4];
        } else {
            for i in 0..16 {
                let (c, a) = self.pens[i];
                env.night[i] = (moonlit(ctx, c, 0.55, 0.2), a);
                env.far_day[i] = (moonlit(ctx, c, 0.65, 0.0), 0);
                env.far_night[i] = (moonlit(ctx, c, 0.85, 0.1), 0);
            }
            for i in 0..4 {
                let c = self.water[i].0;
                env.water_night[i] = (moonlit(ctx, c, 0.55, 0.2), 0);
                env.water_flash[i] = (mixable(c).lerp(Color::Rgb(255, 255, 255), 0.85), BOLD);
            }
        }
        env.far = env.far_day;
        env.ready = true;
        env.last_key = u32::MAX;
    }

    /// Before drawing a frame: dims the pens as the light falls.
    pub(super) fn prepare_draw(&mut self) {
        let Some(w) = self.world.as_mut() else {
            return;
        };
        let env = &mut w.env;
        if !env.ready {
            return;
        }
        let l = env.light(w.biome == DEEP);
        let flash = env.flash > 0.0;
        let key = (l * 255.0) as u32 | (flash as u32) << 8;
        if key == env.last_key {
            return;
        }
        env.last_key = key;
        let t = 1.0 - l;
        if env.native {
            // Sixteen colors cannot fade: step from day to dusk (no bold) to night.
            self.pens = if l >= 0.66 {
                env.day
            } else if l >= 0.33 {
                env.dusk
            } else {
                env.night
            };
            env.far = if l >= 0.5 { env.far_day } else { env.far_night };
            self.water = if flash {
                env.water_flash
            } else if l >= 0.5 {
                env.water_day
            } else {
                env.water_night
            };
        } else {
            mix(&env.day, &env.night, t, &mut self.pens);
            mix(&env.far_day, &env.far_night, t, &mut env.far);
            if flash {
                self.water = env.water_flash;
            } else {
                mix(&env.water_day, &env.water_night, t, &mut self.water);
            }
        }
    }

    /// The sky over the water, before everything else is drawn.
    pub(super) fn draw_sky(&self, c: &mut Canvas) {
        let Some(w) = self.world.as_ref() else {
            return;
        };
        let env = &w.env;
        if !self.surface() || !env.ready || self.w <= 0 {
            return;
        }
        let (wd, time) = (self.w as u64, self.time);
        let storm = env.level;
        let night = (1.0 - env.daylight) * (1.0 - 0.8 * storm);
        // Stars come out one by one as it gets dark.
        if night > 0.05 {
            let n = (self.w / 5).max(1) as u64;
            for i in 0..n {
                let r = hash(w.seed ^ i.wrapping_mul(0x51_7c_c1) ^ 0x57A2);
                let order = ((r >> 50) & 1023) as f32 / 1024.0;
                if night <= order * 0.9 {
                    continue;
                }
                let speed = 0.8 + ((r >> 30) % 100) as f32 / 100.0 * 1.5;
                let tw = (time * speed + ((r >> 40) % 628) as f32 / 100.0).sin();
                let (ch, pen) = if tw > 0.6 {
                    (STARS[0], W + BRIGHT)
                } else if tw > -0.2 {
                    (STARS[1], W + BRIGHT)
                } else if tw > -0.7 {
                    (STARS[2], W)
                } else {
                    continue;
                };
                let (col, attr) = env.day[pen as usize];
                c.set_attr((r % wd) as i32, ((r >> 20) % 5) as i32, ch, col, attr);
            }
        }
        if storm < 0.6 {
            let p = env.phase;
            let span = self.w as f32 + 6.0;
            if let Some(f) = arc(p, NIGHT_HALF, 1.0 - 2.0 * NIGHT_HALF)
                && env.daylight > 0.0
            {
                let x = (-3.0 + f * span) as i32;
                let y = 4 - ((f * PI).sin() * 3.4).round() as i32;
                let pen = if !(0.12..=0.88).contains(&f) {
                    R + BRIGHT
                } else {
                    Y + BRIGHT
                };
                let (col, attr) = env.day[pen as usize];
                for (dy, row) in SUN.iter().enumerate() {
                    for (dx, ch) in row.chars().enumerate() {
                        c.set_attr(x + dx as i32, y + dy as i32, ch, col, attr);
                    }
                }
                // Rays: straight and diagonal ones take turns.
                let rays = &SUN_RAYS[(time * 1.5) as usize & 1];
                for (&(dx, dy), ch) in SUN_RAY_AT.iter().zip(rays) {
                    c.set_attr(x + dx, y + dy, *ch, col, attr);
                }
            }
            if let Some(f) = arc(p, 1.0 - NIGHT_HALF, 2.0 * NIGHT_HALF)
                && night > 0.3
            {
                let x = (-3.0 + f * span) as i32;
                let y = 3 - ((f * PI).sin() * 2.6).round() as i32;
                let (col, attr) = env.day[(W + BRIGHT) as usize];
                // Each night shows one phase of the moon.
                let rows = &MOON[(w.seed >> 7) as usize % MOON.len()];
                for (dy, row) in rows.iter().enumerate() {
                    for (dx, ch) in row.chars().enumerate() {
                        if ch != ' ' {
                            c.set_attr(x + dx as i32, y + dy as i32, ch, col, attr);
                        }
                    }
                }
            }
        }
        // Clouds drift slowly; a storm fills the sky with them.
        let calm = (self.w / 45).clamp(0, 3) as u64;
        let n = calm + (storm * self.w as f32 / 20.0) as u64;
        let twilight = env.daylight > 0.05 && env.daylight < 0.95;
        let dim = storm <= 0.3 && night > 0.5;
        let pen = if dim {
            K
        } else if storm > 0.3 {
            K + BRIGHT
        } else if twilight {
            if env.phase < 0.5 { Y } else { R }
        } else {
            W
        };
        let (col, attr) = self.pens[pen as usize];
        let shapes = if dim { &CLOUDS_NIGHT } else { &CLOUDS };
        for i in 0..n.min(16) {
            let r = hash(w.seed ^ i.wrapping_mul(0x9e37) ^ 0xC10D);
            let speed = 0.25 + ((r >> 8) % 100) as f32 / 100.0 * 0.4;
            let lap = self.w as f32 + 16.0;
            let x = (((r >> 16) % 4096) as f32 + time * speed).rem_euclid(lap) as i32 - 8;
            let y = if i >= calm {
                (r >> 40) % 2
            } else {
                (r >> 40) % 3
            } as i32;
            for (dy, row) in shapes[(r >> 3) as usize % CLOUDS.len()].iter().enumerate() {
                for (dx, ch) in row.chars().enumerate() {
                    if ch != ' ' {
                        c.set_attr(x + dx as i32, y + dy as i32, ch, col, attr);
                    }
                }
            }
        }
    }

    /// Over everything: rain, lightning, glows.
    pub(super) fn draw_env_overlay(&self, c: &mut Canvas) {
        let Some(w) = self.world.as_ref() else {
            return;
        };
        let env = &w.env;
        if !env.ready || self.w <= 0 || self.h <= 0 {
            return;
        }
        let surface = self.surface();
        if surface && env.level > 0.02 {
            let rows = RAIN_ROWS.min(self.h);
            let n = (self.w as f32 * 0.6 * env.level) as u64;
            let slant = (env.cur * 0.45).clamp(-1.0, 1.0);
            let (col, attr) = self.pens[C as usize];
            for i in 0..n {
                let r = hash(w.seed ^ i.wrapping_mul(0x2545_f491) ^ 0x8A17);
                let speed = 14.0 + ((r >> 20) % 100) as f32 / 100.0 * 8.0;
                let y =
                    (((r >> 32) % rows as u64) as f32 + self.time * speed).rem_euclid(rows as f32);
                let x = ((r % self.w as u64) as f32 + y * slant).rem_euclid(self.w as f32);
                let ch = if slant > 0.2 {
                    RAIN[0]
                } else if slant < -0.2 {
                    RAIN[1]
                } else if r & 3 == 0 {
                    RAIN[2]
                } else {
                    RAIN[3]
                };
                let (x, y) = (x as i32, y as i32);
                // Rain falls behind the ships and the waves' crests.
                if c.get(x, y).is_some_and(|k| k.ch == ' ') {
                    c.set_attr(x, y, ch, col, attr);
                }
            }
        }
        if surface && env.flash > 0.0 {
            // A jagged bolt from the clouds down to the waves.
            let mut x = env.bolt_x % self.w;
            let pen = if env.bolt_seed & 1 == 0 {
                W + BRIGHT
            } else {
                Y + BRIGHT
            };
            let (col, attr) = env.day[pen as usize];
            for row in 0..5 {
                let left = (env.bolt_seed >> (row + 1)) & 1 == 0;
                x += if left { -1 } else { 1 };
                let (a, b) = if left { (BOLT[0], 1) } else { (BOLT[1], -1) };
                // A faint halo on the side the bolt leans away from.
                if c.get(x + b, row).is_some_and(|k| k.ch == ' ') {
                    c.set_attr(x + b, row, BOLT[2], col, attr);
                }
                c.set_attr(x, row, a, col, attr);
            }
        }
        // Glowing things sparkle in the dark.
        if self.light() < 0.5 {
            let clock = self.time * 0.9;
            for e in &self.ents {
                if e.dead || e.traits & GLOW == 0 {
                    continue;
                }
                let h = hash((e.id as u64) << 8 ^ 0x6C0);
                let ph = (h & 0xff) as f32 / 256.0 * TAU;
                let v = (clock + ph).sin();
                if v < 0.88 {
                    continue;
                }
                // Each pulse shows up in a new spot on the creature.
                let cycle = ((clock + ph) / TAU).floor() as i64 as u64;
                let r = hash(h ^ cycle.wrapping_mul(0x9e37_79b9));
                let ew = e.w.max(1.0) as u64;
                let eh = e.h.max(1.0) as u64 + 2;
                let x = e.x.floor() as i32 + (r % ew) as i32;
                let y = e.y.floor() as i32 + ((r >> 16) % eh) as i32 - 1;
                let ch = if v > 0.97 {
                    SPARKS[0]
                } else if v > 0.93 {
                    SPARKS[1]
                } else {
                    SPARKS[2]
                };
                let pen = [C, G, B, Y][(h >> 8) as usize & 3] + BRIGHT;
                let (col, attr) = env.day[pen as usize];
                c.set_attr(x, y, ch, col, attr);
            }
        }
    }

    /// How far the storm has pushed water line `i` sideways, in cells.
    pub(super) fn storm_shift(&self, i: usize) -> i64 {
        let v = [0.0, 1.2, -0.9, 0.6][i & 3];
        self.world
            .as_ref()
            .map_or(0, |w| (w.env.wave_phase * v).floor() as i64)
    }

    /// How stormy it is, 0 (calm) to 1 (full storm).
    #[cfg(test)]
    pub(super) fn storm(&self) -> f32 {
        self.world.as_ref().map_or(0.0, |w| w.env.level)
    }

    /// The pens to draw `e` with (also in classic).
    pub(super) fn pen_table(&self, e: &Ent) -> &[(Color, u8); 16] {
        let Some(w) = self.world.as_ref() else {
            return &self.pens;
        };
        if e.layer > 0 {
            return &w.env.far;
        }
        // Glowing things keep their day colors in the dark.
        if e.traits & GLOW != 0 && w.env.ready && w.env.light(w.biome == DEEP) < 0.5 {
            return &w.env.day;
        }
        &self.pens
    }

    /// The pen for water line `i` (also in classic).
    pub(super) fn water_pen(&self, i: usize) -> (Color, u8) {
        self.water[i]
    }

    /// How likely a creature with these traits is to come now, relative to
    /// its weight (1 = as usual).
    pub(super) fn activity(&self, traits: assets::Traits) -> f32 {
        let dark = 1.0 - self.light();
        if traits & NOCTURNAL != 0 {
            0.25 + 2.75 * dark
        } else {
            1.0 - 0.4 * dark
        }
    }

    /// The layer a new group of species `si` swims in: 0 near, 1 far.
    pub(super) fn pick_layer(&mut self, si: usize) -> u8 {
        let depth = self.world.as_ref().map_or(0.0, |w| w.env.depth);
        let Some(sp) = library::library().species.get(si) else {
            return 0;
        };
        if sp.size > 120 || sp.def.traits & LEGENDARY != 0 {
            return 0;
        }
        u8::from(self.rng.f32() < depth)
    }

    /// Sends a new creature to the far layer (its `layer` is already set).
    pub(super) fn make_far(&mut self, e: &mut Ent) {
        e.z = Z_FAR + self.rng.i32(0..2);
        e.vx *= 0.6;
        e.speed *= 0.6;
        e.physical = false;
        // The middle band of the water, if there is room for it.
        let top = self.water_top() as f32;
        let span = (self.h as f32 - e.h - top).max(0.0);
        let (lo, hi) = ((top + span * 0.25).round(), (top + span * 0.75).round());
        if span > 0.0 && hi >= lo {
            e.y = e.y.clamp(lo, hi);
            if let Motion::Swim { base, .. } = &mut e.motion {
                *base = base.clamp(lo, hi);
            }
        }
    }

    /// The water's current now, in cells per second.
    pub(super) fn current(&self) -> (f32, f32) {
        (self.world.as_ref().map_or(0.0, |w| w.env.cur), 0.0)
    }

    /// Daylight: 1 at noon, 0 at night (always 0 in the deep).
    pub(super) fn light(&self) -> f32 {
        self.world
            .as_ref()
            .map_or(1.0, |w| w.env.light(w.biome == DEEP))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    fn val(v: impl Into<toml::Value>) -> toml::Value {
        v.into()
    }

    /// A scene with `opts` under `theme`, sized `size`, already themed.
    fn scene(
        theme: &Theme,
        opts: &[(&str, toml::Value)],
        size: (u16, u16),
        f: impl FnOnce(&Ctx, &mut Aquarium),
    ) {
        let mut t = toml::Table::new();
        for (k, v) in opts {
            t.insert(k.to_string(), v.clone());
        }
        let opts = crate::scene::Opts::new(INFO.options, t);
        let bg = if theme.native {
            Color::Reset
        } else {
            Color::Rgb(10, 12, 20)
        };
        let ctx = Ctx {
            theme,
            opts: &opts,
            bg,
            seed: 9,
        };
        let mut a = Aquarium::new(&ctx);
        a.retheme(&ctx);
        a.resize(&ctx, size.0, size.1);
        f(&ctx, &mut a);
    }

    fn reef(opts: &[(&str, toml::Value)], f: impl FnOnce(&Ctx, &mut Aquarium)) {
        let mut all = vec![("biome", val("reef"))];
        all.extend_from_slice(opts);
        scene(&Theme::classic(), &all, (120, 40), f);
    }

    fn env(a: &mut Aquarium) -> &mut Env {
        &mut a.world.as_mut().unwrap().env
    }

    fn run(ctx: &Ctx, a: &mut Aquarium, secs: f32) {
        for _ in 0..(secs * 20.0) as usize {
            a.update(ctx, 0.05);
        }
    }

    #[test]
    fn light_cycles_between_day_and_night() {
        reef(
            &[("day_length", val(100.0)), ("weather", val(false))],
            |ctx, a| {
                let (mut lo, mut hi) = (1.0f32, 0.0f32);
                for _ in 0..2000 {
                    a.update(ctx, 0.05);
                    lo = lo.min(a.light());
                    hi = hi.max(a.light());
                }
                assert!(lo < 0.01 && hi > 0.99, "{lo} {hi}");
            },
        );
    }

    #[test]
    fn day_and_night_shares() {
        let n = 10_000;
        let share = |f: &dyn Fn(f32) -> bool| {
            (0..n)
                .filter(|&i| f(Env::day_light(i as f32 / n as f32)))
                .count() as f32
                / n as f32
        };
        let day = share(&|l| l > 0.99);
        let dark = share(&|l| l < 0.01);
        assert!((day - 0.55).abs() < 0.02, "{day}");
        assert!((dark - 0.30).abs() < 0.02, "{dark}");
    }

    #[test]
    fn the_deep_is_always_dark_and_a_zero_day_always_light() {
        scene(
            &Theme::classic(),
            &[("biome", val("deep")), ("day_length", val(50.0))],
            (100, 30),
            |ctx, a| {
                for _ in 0..600 {
                    a.update(ctx, 0.05);
                    assert_eq!(a.light(), 0.0);
                }
            },
        );
        reef(
            &[("day_length", val(0.0)), ("weather", val(false))],
            |ctx, a| {
                for _ in 0..600 {
                    a.update(ctx, 0.05);
                    assert_eq!(a.light(), 1.0);
                }
            },
        );
    }

    #[test]
    fn classic_keeps_its_pens() {
        scene(
            &Theme::classic(),
            &[("biome", val("classic"))],
            (100, 30),
            |ctx, a| {
                run(ctx, a, 5.0);
                let mut c = Canvas::new(100, 30, Color::Reset);
                a.draw(ctx, &mut c);
                let e = Ent::new(Kind::Fish, SP_FISH, 0.0, 0.0, 5, W);
                assert!(std::ptr::eq(a.pen_table(&e), &a.pens));
                for i in 0..4 {
                    assert_eq!(a.water_pen(i), a.water[i]);
                }
                assert_eq!(a.pens, classic_pens());
                assert_eq!(a.light(), 1.0);
                assert_eq!(a.current(), (0.0, 0.0));
            },
        );
    }

    #[test]
    fn nights_are_dim_and_native_loses_its_bold() {
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            let native = theme.native;
            let opts = [
                ("biome", val("reef")),
                ("day_length", val(1e6)),
                ("weather", val(false)),
            ];
            scene(&theme, &opts, (100, 30), |ctx, a| {
                a.update(ctx, 0.05);
                env(a).phase = 0.5;
                a.update(ctx, 0.05);
                a.prepare_draw();
                let day = a.pens;
                env(a).phase = 0.0;
                a.update(ctx, 0.05);
                a.prepare_draw();
                assert_ne!(a.pens, day);
                if native {
                    assert!(a.pens.iter().all(|p| p.1 & BOLD == 0));
                } else {
                    let w = (W + BRIGHT) as usize;
                    assert!(a.pens[w].0.luma() < day[w].0.luma());
                }
                let mut c = Canvas::new(100, 30, Color::Reset);
                a.draw(ctx, &mut c);
            });
        }
    }

    #[test]
    fn glowing_things_keep_their_colors_in_the_dark() {
        reef(
            &[("day_length", val(1e6)), ("weather", val(false))],
            |ctx, a| {
                a.update(ctx, 0.05);
                env(a).phase = 0.0;
                a.update(ctx, 0.05);
                a.prepare_draw();
                let mut e = Ent::new(Kind::Fish, SP_FISH, 3.0, 12.0, 5, W);
                e.traits = GLOW;
                assert!(std::ptr::eq(a.pen_table(&e), &env(a).day));
                e.traits = 0;
                assert!(std::ptr::eq(a.pen_table(&e), &a.pens));
                // It sparkles now and then.
                e.traits = GLOW;
                a.spawn(e);
                let mut sparks = 0;
                for _ in 0..400 {
                    a.update(ctx, 0.05);
                    let mut c = Canvas::new(120, 40, Color::Reset);
                    a.draw_env_overlay(&mut c);
                    sparks += (0..40)
                        .flat_map(|y| (0..120).map(move |x| (x, y)))
                        .filter(|&(x, y)| SPARKS.contains(&c.get(x, y).unwrap().ch))
                        .count();
                }
                assert!(sparks > 0);
            },
        );
    }

    #[test]
    fn nocturnal_creatures_come_at_night() {
        reef(
            &[("day_length", val(1e6)), ("weather", val(false))],
            |ctx, a| {
                a.update(ctx, 0.05);
                env(a).phase = 0.5;
                a.update(ctx, 0.05);
                let (n_day, o_day) = (a.activity(NOCTURNAL), a.activity(0));
                env(a).phase = 0.0;
                a.update(ctx, 0.05);
                let (n_night, o_night) = (a.activity(NOCTURNAL), a.activity(0));
                assert!(n_day < 0.5 && n_night > 2.5, "{n_day} {n_night}");
                assert!(o_day > 0.99 && (0.5..0.7).contains(&o_night));
            },
        );
    }

    fn small_species() -> usize {
        library::library()
            .species
            .iter()
            .position(|s| {
                s.size <= 120 && s.def.traits & LEGENDARY == 0 && s.def.swim != assets::Swim::Crawl
            })
            .unwrap()
    }

    #[test]
    fn the_far_share_follows_depth_and_far_creatures_hang_back() {
        reef(&[("depth", val(0.4))], |_, a| {
            let si = small_species();
            let far = (0..4000).filter(|_| a.pick_layer(si) == 1).count() as f32 / 4000.0;
            assert!((far - 0.4).abs() < 0.05, "{far}");
            env(a).depth = 1.0;
            if let Some(big) = library::library().species.iter().position(|s| s.size > 120) {
                assert_eq!(a.pick_layer(big), 0);
            }
            let before = a.ents.len();
            for _ in 0..10 {
                a.spawn_group(si, 3, true, false);
            }
            let new: Vec<_> = a.ents[before..].to_vec();
            assert!(!new.is_empty());
            for e in new {
                assert_eq!(e.layer, 1);
                assert!(e.z > Z_CASTLE && !e.physical);
                assert!(e.y >= a.water_top() as f32);
            }
        });
        reef(&[("depth", val(0.0))], |_, a| {
            let si = small_species();
            assert!((0..200).all(|_| a.pick_layer(si) == 0));
        });
    }

    #[test]
    fn the_current_carries_bubbles_sideways() {
        reef(
            &[("current", val(5.0)), ("weather", val(false))],
            |ctx, a| {
                a.update(ctx, 0.01);
                let mut hit = false;
                for _ in 0..4000 {
                    a.update(ctx, 0.01);
                    if a.current().0.abs() < 0.2 {
                        continue;
                    }
                    let at = a.ents.len();
                    a.add_bubble_at(60.0, 30.0, 5);
                    a.environment(0.05);
                    let b = a.ents[at];
                    assert!(((b.x - 60.0) - a.current().0 * 0.05 * 0.7).abs() < 1e-3);
                    hit = true;
                    break;
                }
                assert!(hit);
            },
        );
        reef(
            &[("current", val(0.0)), ("weather", val(false))],
            |ctx, a| {
                run(ctx, a, 30.0);
                assert_eq!(a.current(), (0.0, 0.0));
            },
        );
    }

    #[test]
    fn plants_lean_together_and_sway_free_again() {
        reef(
            &[("current", val(10.0)), ("weather", val(false))],
            |ctx, a| {
                let mut t = 0.0;
                while env(a).lean == 0 && t < 600.0 {
                    run(ctx, a, 1.0);
                    t += 1.0;
                }
                let lean = env(a).lean;
                assert!(lean != 0);
                let swaying = |e: &&Ent| {
                    e.kind == Kind::Seaweed && sprites()[e.spr as usize].frame_count() >= 2
                };
                let frames: Vec<(usize, usize)> = a
                    .ents
                    .iter()
                    .filter(swaying)
                    .map(|e| (e.frame as usize, sprites()[e.spr as usize].frame_count()))
                    .collect();
                assert!(frames.len() > 3);
                // Right after the step they are all at the same end.
                for (f, n) in frames {
                    assert_eq!(f, if lean > 0 { n - 1 } else { 0 });
                }
                env(a).current = 0.0;
                run(ctx, a, 1.0);
                assert_eq!(env(a).lean, 0);
                let at: Vec<f32> = a.ents.iter().filter(swaying).map(|e| e.frame).collect();
                run(ctx, a, 4.0);
                let after: Vec<f32> = a.ents.iter().filter(swaying).map(|e| e.frame).collect();
                assert_ne!(at, after);
            },
        );
    }

    #[test]
    fn a_storm_starts_and_ends_within_its_limits() {
        reef(&[("weather", val(true))], |ctx, a| {
            a.update(ctx, 0.05);
            {
                let e = env(a);
                e.storm_cd = 2.0;
                e.gap = (1000.0, 1000.0);
                e.span = (10.0, 12.0);
            }
            let (mut first, mut last, mut peak) = (None, 0.0, 0.0f32);
            let mut t = 0.0;
            while t < 60.0 {
                a.update(ctx, 0.05);
                t += 0.05;
                let s = a.storm();
                if s > 0.0 {
                    first.get_or_insert(t);
                    last = t;
                    peak = peak.max(s);
                }
            }
            let first = first.expect("no storm");
            assert!((1.5..4.0).contains(&first), "{first}");
            assert!(last - first <= 12.0 + STORM_RAMP + 0.5, "{}", last - first);
            assert!(last - first >= 10.0, "{}", last - first);
            assert_eq!(peak, 1.0);
            assert_eq!(a.storm(), 0.0);
        });
        reef(&[("weather", val(false))], |ctx, a| {
            a.update(ctx, 0.05);
            env(a).storm_cd = 0.0;
            run(ctx, a, 5.0);
            assert_eq!(a.storm(), 0.0);
        });
    }

    #[test]
    fn nothing_panics_on_any_screen_or_theme() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        for theme in &themes {
            for biome in ["reef", "deep"] {
                for size in [(1, 1), (5, 3), (300, 12), (80, 30)] {
                    let opts = [
                        ("biome", val(biome)),
                        ("day_length", val(20.0)),
                        ("weather", val(true)),
                        ("depth", val(0.6)),
                    ];
                    scene(theme, &opts, size, |ctx, a| {
                        a.update(ctx, 0.05);
                        let e = env(a);
                        e.storm_cd = 1.0;
                        e.gap = (15.0, 20.0);
                        e.span = (8.0, 10.0);
                        let mut c = Canvas::new(size.0, size.1, Color::Reset);
                        for i in 0..800 {
                            a.update(ctx, 0.05);
                            if i % 7 == 0 {
                                c.clear();
                                a.draw(ctx, &mut c);
                            }
                        }
                    });
                }
            }
        }
    }

    #[test]
    fn the_sky_is_drawn_in_unicode() {
        use super::super::biome::{SAND, SNOW, STONES};
        use super::super::events::{CANDLE, HAND, HAND_FOOD, PETALS};
        let mut all = String::new();
        for g in [&STARS, &SPARKS, &RAIN[..], &BOLT[..], &SAND, &SNOW, &STONES]
            .into_iter()
            .chain([&PETALS[..], &CANDLE, &HAND_FOOD])
        {
            all.extend(g.iter());
        }
        for r in SUN
            .iter()
            .chain(MOON.iter().flatten())
            .chain(CLOUDS.iter().flatten())
            .chain(CLOUDS_NIGHT.iter().flatten())
            .chain(HAND.iter())
        {
            all.push_str(r);
        }
        for r in SUN_RAYS {
            all.extend(r.iter());
        }
        for ch in all.chars().filter(|&c| c != ' ') {
            assert!(
                super::super::assets::drawable(ch) && !ch.is_ascii_punctuation(),
                "{ch:?}"
            );
        }
    }
}
