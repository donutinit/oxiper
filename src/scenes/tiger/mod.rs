//! tiger: an immortal tiger rules an endless jungle, and now and then a UFO
//! takes it away for a while.
//!
//! Original oxiper work, in the manner of the shark and dog scenes. The
//! jungle is pixel art of a fixed height ([`WORLD_H`]) and endless width,
//! every part of it dealt out from the seed by hashing, so the camera
//! trailing the tiger can follow it forever. Faraway hills and two layers of
//! forest scroll slower than the tiger's own ground (parallax) and fade into
//! the haze; rivers and pools cross its path, tall grass hides it while it
//! stalks, and the light follows the wall clock from dawn mist to fireflies.
//!
//! - `art.rs`: every sprite, drawn by hand one character per pixel: the
//!   tiger's torso, heads (calm, blinking, snarling, roaring), its front view
//!   and its sleep; deer, boar, peafowl, langurs, birds, the crocodile, the
//!   bear, the dholes and the saucer; and how each animal is built.
//! - `body.rs`: how a four-legged animal is put together on screen: the
//!   hand-drawn body and head, legs bent by inverse kinematics for each gait
//!   and pose, a tail that sways, all tilted without losing a pixel.
//! - `hunter.rs`: the tiger's moods (patrolling, clawing a tree, stalking,
//!   the charge and the pounce, feeding, drinking, swimming, napping,
//!   roaring, seeing off rivals) and its gaits.
//! - `life.rs`: herds that graze and bolt, langurs that bark alarms from the
//!   trees, birds that flush, the rivals that come to try it, and specks of
//!   dust, leaves, snow and fireflies.
//! - `ufo.rs`: the saucer: it comes rarely, freezes the jungle, beams the
//!   tiger up, and brings it back a while later.
//! - `jungle.rs`: the ground, the water and the forest layers, the light,
//!   the camera, and the projection onto the screen.
//!
//! The world steps 30 times a second on an accumulator scaled by `speed`,
//! while the clocks (the day, how long a nap or a meal lasts, when the tiger
//! is hungry again, when the saucer comes and brings it back) follow the wall
//! clock.

mod art;
mod body;
mod hunter;
mod jungle;
mod life;
mod ufo;

use crate::color::Color;
use crate::render::Canvas;
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use jungle::Jungle;

pub const INFO: SceneInfo = SceneInfo {
    name: "tiger",
    aliases: &["tigers"],
    about: "an immortal tiger rules an endless jungle, hunting through the day and night",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Pace of the tiger and the jungle (the day keeps real time)",
        },
        Opt {
            key: "day",
            default: Val::I(180),
            help: "Real seconds for a full day, from dawn through night to dawn (10-86400)",
        },
        Opt {
            key: "zoom",
            default: Val::I(2),
            help: "How close the camera is to the tiger, 0 (the whole jungle) to 6",
        },
        Opt {
            key: "kind",
            default: Val::S("bengal"),
            help: "bengal, siberian (snowy taiga), white, or random",
        },
        Opt {
            key: "prey",
            default: Val::F(1.0),
            help: "How many deer, boar, peafowl and birds live around it; 0 for none",
        },
        Opt {
            key: "ufo",
            default: Val::F(1.0),
            help: "Saucer visits per hour of playing time, on average; 0 = never",
        },
        Opt {
            key: "sky",
            default: Val::B(true),
            help: "Paint the sky; off leaves it see-through above the forest",
        },
    ],
    make: |ctx| Box::new(TigerScene::new(ctx)),
};

/// An sRGB color.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub const BLACK: Rgb = Rgb(0, 0, 0);
    pub const WHITE: Rgb = Rgb(255, 255, 255);

    /// Linear interpolation towards `o`; `t` is clamped to `[0, 1]`. In
    /// fixed point: it runs for most pixels of every frame.
    #[inline]
    pub fn lerp(self, o: Rgb, t: f32) -> Rgb {
        let k = (t.clamp(0.0, 1.0) * 256.0 + 0.5) as i32;
        let f = |a: u8, b: u8| ((a as i32 * 256 + (b as i32 - a as i32) * k + 128) >> 8) as u8;
        Rgb(f(self.0, o.0), f(self.1, o.1), f(self.2, o.2))
    }

    /// Multiplies every channel by `k` (saturating).
    #[inline]
    pub fn scale(self, k: f32) -> Rgb {
        let f = |a: u8| (a as f32 * k).clamp(0.0, 255.0) as u8;
        Rgb(f(self.0), f(self.1), f(self.2))
    }

    /// Lit by light of color `l`: channel by channel, white leaves it as is.
    #[inline]
    pub fn mul(self, l: Rgb) -> Rgb {
        let f = |a: u8, b: u8| ((a as u32 * b as u32 + 127) / 255) as u8;
        Rgb(f(self.0, l.0), f(self.1, l.1), f(self.2, l.2))
    }

    /// Perceived brightness in `[0, 255]`.
    #[inline]
    pub fn luma(self) -> u8 {
        ((self.0 as u32 * 54 + self.1 as u32 * 183 + self.2 as u32 * 19) >> 8) as u8
    }
}

/// The scene's color type for an [`Rgb`].
#[inline]
fn col(c: Rgb) -> Color {
    Color::Rgb(c.0, c.1, c.2)
}

/// An [`Rgb`] for a theme color, when it has one (palette entries do not).
fn rgb_of(c: Color) -> Option<Rgb> {
    match c {
        Color::Rgb(r, g, b) => Some(Rgb(r, g, b)),
        _ => None,
    }
}

/// Marks sky pixels of the world while the sky is see-through.
const KEY: Rgb = Rgb(255, 0, 255);

/// Dice on top of fastrand.
trait Dice {
    /// An integer in `[lo, hi)`, or `lo` when the range is empty.
    fn range(&mut self, lo: i32, hi: i32) -> i32;
    fn rangef(&mut self, lo: f32, hi: f32) -> f32;
    fn chance(&mut self, p: f32) -> bool;
    /// -1 or 1.
    fn sign(&mut self) -> f32;
}

impl Dice for fastrand::Rng {
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        if hi <= lo { lo } else { self.i32(lo..hi) }
    }

    fn rangef(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.f32()
    }

    fn chance(&mut self, p: f32) -> bool {
        self.f32() < p
    }

    fn sign(&mut self) -> f32 {
        if self.bool() { 1.0 } else { -1.0 }
    }
}

type Rng = fastrand::Rng;

#[inline]
fn hash(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = (x as u32).wrapping_mul(374_761_393) ^ (y as u32).wrapping_mul(668_265_263) ^ seed;
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

/// A number in `[0, 1)` for `x`, always the same for the same `x`, `salt` and `seed`.
#[inline]
fn unit(x: i32, salt: i32, seed: u32) -> f32 {
    (hash(x, salt, seed) >> 8) as f32 / (1u32 << 24) as f32
}

/// Smooth value noise in `[0, 1)`: hashed values `1/f` apart, eased in between.
#[inline]
fn noise(x: f64, f: f64, salt: i32, seed: u32) -> f32 {
    let u = x * f;
    let i = u.floor();
    let t = (u - i) as f32;
    let t = t * t * (3.0 - 2.0 * t);
    let i = i as i64 as i32;
    let (a, b) = (unit(i, salt, seed), unit(i.wrapping_add(1), salt, seed));
    a + (b - a) * t
}

/// Seconds per step of the world.
const STEP: f64 = 1.0 / 30.0;
/// The most steps one frame may catch up on (after a suspend, say).
const MAX_STEPS: u32 = 300;
/// Real seconds of a day at which the naps and meals were timed.
const BASE_DAY: f32 = 180.0;
/// How much the theme tints the sky and the greens.
const TINT: f32 = 0.3;

/// Height of the world in pixels; its width is endless.
const WORLD_H: i32 = 112;
/// Camera zoom levels, as the number of world pixel rows in view.
const ZOOMS: [f32; 7] = [112.0, 94.0, 78.0, 66.0, 56.0, 48.0, 42.0];

/// Which tiger, and so which forest.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Orange and black, in a monsoon forest of sal and teak.
    Bengal,
    /// Paler and shaggier, in the snowy taiga by a frozen river.
    Siberian,
    /// White with dark stripes and blue eyes, in the Bengal forest.
    White,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::Bengal, Kind::Siberian, Kind::White];

    fn parse(s: &str) -> Option<Kind> {
        match s {
            "bengal" | "orange" => Some(Kind::Bengal),
            "siberian" | "amur" | "snow" => Some(Kind::Siberian),
            "white" => Some(Kind::White),
            _ => None,
        }
    }

    /// A forest of snow and conifers rather than the monsoon jungle.
    pub fn snowy(self) -> bool {
        self == Kind::Siberian
    }
}

/// The colors a theme changes: the daytime sky and the greens. Under the
/// native theme they are the jungle's own.
#[derive(Clone, Copy)]
struct Pal {
    day_top: Rgb,
    day_bot: Rgb,
    green: Rgb,
    green_k: f32,
}

const DAY_TOP: Rgb = Rgb(84, 150, 214);
const DAY_BOT: Rgb = Rgb(204, 226, 222);
const SNOW_DAY_TOP: Rgb = Rgb(124, 160, 204);
const SNOW_DAY_BOT: Rgb = Rgb(222, 230, 238);

impl Pal {
    fn new(ctx: &Ctx, kind: Kind) -> Pal {
        let (top, bot) = if kind.snowy() {
            (SNOW_DAY_TOP, SNOW_DAY_BOT)
        } else {
            (DAY_TOP, DAY_BOT)
        };
        let native = Pal {
            day_top: top,
            day_bot: bot,
            green: Rgb::BLACK,
            green_k: 0.0,
        };
        if ctx.theme.native {
            return native;
        }
        let g = &ctx.theme.grad;
        let mix = |base: Rgb, c: Color| rgb_of(c).map_or(base, |c| base.lerp(c, TINT));
        match rgb_of(g.at(0.7)) {
            Some(green) => Pal {
                day_top: mix(top, g.at(0.55)),
                day_bot: mix(bot, g.at(0.9)),
                green,
                green_k: TINT,
            },
            None => native,
        }
    }
}

/// The light at one time of day.
#[derive(Clone, Copy, Debug)]
struct Light {
    /// The sky at the top and at the horizon.
    top: Rgb,
    bot: Rgb,
    /// What lights things up: white by day, dim blue by night.
    tint: Rgb,
}

impl Light {
    fn lerp(self, o: Light, t: f32) -> Light {
        Light {
            top: self.top.lerp(o.top, t),
            bot: self.bot.lerp(o.bot, t),
            tint: self.tint.lerp(o.tint, t),
        }
    }
}

const NIGHT: Light = Light {
    top: Rgb(6, 10, 26),
    bot: Rgb(22, 32, 58),
    tint: Rgb(70, 84, 132),
};

/// Time of day keyframes.
fn keys(p: &Pal) -> [(f32, Light); 8] {
    let day = Light {
        top: p.day_top,
        bot: p.day_bot,
        tint: Rgb::WHITE,
    };
    [
        (0.0, NIGHT),
        (0.17, NIGHT),
        (
            0.23,
            Light {
                top: Rgb(70, 74, 140),
                bot: Rgb(250, 170, 128),
                tint: Rgb(232, 186, 170),
            },
        ),
        (0.3, day),
        (0.68, day),
        (
            0.75,
            Light {
                top: Rgb(80, 60, 130),
                bot: Rgb(252, 146, 84),
                tint: Rgb(240, 170, 140),
            },
        ),
        (0.82, NIGHT),
        (1.0, NIGHT),
    ]
}

/// Which tiger the `kind` option asks for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum KindOpt {
    One(Kind),
    Random,
}

/// What the options choose.
#[derive(Clone, Copy)]
struct Settings {
    sky: bool,
    /// How many animals live around the tiger, 1 being the usual.
    prey: f32,
    /// Saucer visits per hour.
    ufo: f32,
    /// Index into [`ZOOMS`].
    zoom: usize,
    kind: KindOpt,
    /// Real seconds per day over [`BASE_DAY`]: naps last as long a part of the day.
    rest_scale: f32,
}

/// A number from the options, or `default` when it is not a number.
fn finite(v: f32, default: f32) -> f32 {
    if v.is_finite() { v } else { default }
}

/// The scene: the jungle, stepped at a steady pace.
struct TigerScene {
    jungle: Jungle,
    rng: Rng,
    set: Settings,
    speed: f64,
    /// Real seconds for a day.
    day: f32,
    /// Scene seconds not yet spent on steps.
    acc: f64,
    /// Whether the picture needs painting again.
    dirty: bool,
    /// The time of day last painted.
    drawn_t: f32,
}

impl TigerScene {
    fn new(ctx: &Ctx) -> TigerScene {
        let o = ctx.opts;
        let day = o.i("day").clamp(10, 86_400) as f32;
        let kind = match o.s("kind").trim().to_ascii_lowercase().as_str() {
            "random" | "any" => KindOpt::Random,
            s => KindOpt::One(Kind::parse(s).unwrap_or(Kind::Bengal)),
        };
        let set = Settings {
            sky: o.b("sky"),
            prey: finite(o.f("prey"), 1.0).clamp(0.0, 10.0),
            ufo: finite(o.f("ufo"), 1.0).clamp(0.0, 3600.0),
            zoom: o.i("zoom").clamp(0, ZOOMS.len() as i64 - 1) as usize,
            kind,
            rest_scale: day / BASE_DAY,
        };
        let mut rng = ctx.rng();
        let kind = match set.kind {
            KindOpt::One(k) => k,
            KindOpt::Random => Kind::ALL[rng.usize(..Kind::ALL.len())],
        };
        let jungle = Jungle::new(80, 24, &mut rng, Pal::new(ctx, kind), set, kind);
        TigerScene {
            jungle,
            rng,
            set,
            speed: finite(o.f("speed"), 1.0).clamp(0.0, 50.0) as f64,
            day,
            acc: 0.0,
            dirty: true,
            drawn_t: 0.0,
        }
    }
}

impl Scene for TigerScene {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        // The view is built for the screen's shape, so a resize starts a fresh
        // jungle with the same kind of tiger; the time of day carries over.
        let (t, kind) = (self.jungle.t, self.jungle.kind);
        self.jungle = Jungle::new(w, h, &mut self.rng, self.jungle.pal, self.set, kind);
        self.jungle.t = t;
        self.jungle.update_light();
        self.dirty = true;
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.jungle.pal = Pal::new(ctx, self.jungle.kind);
        self.jungle.update_light();
        self.dirty = true;
    }

    fn update(&mut self, ctx: &Ctx, dt: f32) {
        self.tick(ctx, dt, dt);
    }

    fn tick(&mut self, _ctx: &Ctx, dt: f32, real: f32) {
        // The clocks first: they run on the wall clock, speed or not.
        if real.is_finite() && real > 0.0 {
            let real = real.min(3600.0);
            self.jungle.t = (self.jungle.t + real / self.day).rem_euclid(1.0);
            self.jungle.update_light();
            self.jungle.clocks(real, &mut self.rng);
            if (self.jungle.t - self.drawn_t).abs() >= 0.0004 {
                self.dirty = true;
            }
        }
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.acc += dt as f64 * self.speed;
        let n = ((self.acc / STEP) as u32).min(MAX_STEPS);
        // Beyond the cap the time is dropped rather than caught up on.
        self.acc = if n == MAX_STEPS {
            0.0
        } else {
            self.acc - n as f64 * STEP
        };
        for _ in 0..n {
            self.jungle.update(&mut self.rng);
        }
        self.dirty |= n > 0;
    }

    fn next_change(&self) -> f32 {
        if self.speed <= 0.0 {
            return 1.0;
        }
        ((STEP - self.acc).max(0.0) / self.speed) as f32
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        if self.dirty {
            self.jungle.render();
            self.drawn_t = self.jungle.t;
            self.dirty = false;
        }
        self.jungle.scr.blit(c);
        self.jungle.draw_marks(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::{TermColors, Theme};
    use hunter::Mood;
    use ufo::Phase;

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: Color::Reset,
            seed: 3,
            fiesta: crate::fiesta::Fiesta::NONE,
        }
    }

    fn opts(table: &str) -> Opts {
        Opts::new(INFO.options, table.parse().unwrap())
    }

    fn scene(table: &str, w: u16, h: u16) -> TigerScene {
        let theme = Theme::classic();
        let o = opts(table);
        let c = ctx(&theme, &o);
        let mut s = TigerScene::new(&c);
        s.resize(&c, w, h);
        s
    }

    /// Steps the scene one frame at 30 fps, real time and all.
    fn frame(s: &mut TigerScene) {
        let theme = Theme::classic();
        let o = opts("");
        let c = ctx(&theme, &o);
        s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
    }

    /// Every drawing is a rectangle, and every letter in it has a color.
    #[test]
    fn the_drawings_fit_together() {
        for (name, sprite, pal) in art::all() {
            let w = sprite[0].len();
            for row in sprite {
                assert_eq!(row.len(), w, "{name}: {row}");
                for ch in row.bytes().filter(|&c| c != b'.') {
                    assert!(
                        pal.iter().any(|&(k, _)| k == ch),
                        "{name}: no color for {}",
                        ch as char
                    );
                }
            }
        }
        // The heads swap for one another.
        let heads = [
            art::TIGER_HEAD,
            art::TIGER_HEAD_BLINK,
            art::TIGER_HEAD_SNARL,
            art::TIGER_HEAD_ROAR,
        ];
        for h in heads {
            assert_eq!((h[0].len(), h.len()), (14, 15));
        }
        // The legs' and tail's letters have colors too.
        for pal in art::TIGER_COLORS {
            let b = &art::TIGER;
            for ch in [b.leg.0, b.leg.1, b.paw.0, b.paw.1, b'S', b's'] {
                assert!(pal.iter().any(|&(k, _)| k == ch), "{}", ch as char);
            }
        }
    }

    /// Odd option values, sizes and steps must not panic (the registry test covers the defaults).
    #[test]
    fn extreme_options_survive() {
        let cases = [
            "speed = 50.0\nday = 1\nzoom = 99\nsky = false\nprey = 10.0\nkind = 'random'\nufo = 3600.0",
            "speed = 0.0\nday = 999999999\nzoom = -4\nprey = -3.0\nkind = 'nope'\nufo = -1.0",
            "kind = 'siberian'\nprey = 0\nufo = nan",
            "kind = 'white'\nzoom = 6\nspeed = inf",
            "kind = 'bengal'\nzoom = 0\nsky = false\nprey = inf",
        ];
        let themes = [
            Theme::classic(),
            Theme::builtin("mono").unwrap(),
            Theme::builtin("dracula").unwrap(),
            Theme::terminal(&TermColors::default()),
        ];
        for case in cases {
            let o = opts(case);
            for theme in &themes {
                let c = ctx(theme, &o);
                let mut s = (INFO.make)(&c);
                for &(w, h) in &[
                    (1u16, 1u16),
                    (2, 3),
                    (7, 2),
                    (3, 1),
                    (1, 40),
                    (300, 90),
                    (80, 24),
                ] {
                    s.resize(&c, w, h);
                    s.retheme(&c);
                    let mut cv = Canvas::new(w, h, Color::Reset);
                    for dt in [0.0, f32::NAN, -1.0, f32::INFINITY, 0.05, 5.0, 1e9] {
                        s.tick(&c, dt, 0.05);
                        s.next_change();
                        cv.clear();
                        s.draw(&c, &mut cv);
                    }
                }
            }
        }
    }

    /// The day follows the wall clock: a tenth of the speed moves the
    /// jungle slower but not the sun.
    #[test]
    fn the_day_keeps_real_time() {
        let theme = Theme::classic();
        let o = opts("day = 100");
        let c = ctx(&theme, &o);
        let (mut fast, mut slow) = (TigerScene::new(&c), TigerScene::new(&c));
        fast.resize(&c, 80, 24);
        slow.resize(&c, 80, 24);
        let t0 = (fast.jungle.t, slow.jungle.t);
        for _ in 0..300 {
            fast.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            slow.tick(&c, 0.1 / 30.0, 1.0 / 30.0);
        }
        for (a, b) in [(fast.jungle.t, t0.0), (slow.jungle.t, t0.1)] {
            assert!(((a - b).rem_euclid(1.0) - 0.1).abs() < 1e-3, "{a} {b}");
        }
        assert!(
            fast.jungle.tick > 290 && slow.jungle.tick < 35,
            "{} {}",
            fast.jungle.tick,
            slow.jungle.tick
        );
    }

    /// A nap lasts real seconds, even while the jungle stands still.
    #[test]
    fn naps_follow_the_wall_clock() {
        let theme = Theme::classic();
        let o = opts("");
        let c = ctx(&theme, &o);
        let mut s = TigerScene::new(&c);
        s.resize(&c, 80, 24);
        s.jungle.tiger.set(Mood::Nap);
        s.jungle.tiger.clock = 5.0;
        for _ in 0..100 {
            s.tick(&c, 0.0, 0.05);
        }
        assert_eq!(s.jungle.tick, 0);
        assert!(s.jungle.tiger.clock < 0.01, "{}", s.jungle.tiger.clock);
    }

    /// With the sky off nothing is painted above the forest, but the
    /// ground always is.
    #[test]
    fn a_see_through_sky_stays_empty() {
        let theme = Theme::classic();
        for (sky, painted) in [(false, false), (true, true)] {
            let o = opts(&format!("sky = {sky}\nzoom = 0\nkind = 'siberian'"));
            let c = ctx(&theme, &o);
            let mut s = TigerScene::new(&c);
            s.resize(&c, 80, 24);
            let mut cv = Canvas::new(80, 24, Color::Reset);
            s.draw(&c, &mut cv);
            let top = (0..80)
                .filter(|&x| cv.get(x, 0).is_some_and(|c| c.ch != ' '))
                .count();
            // The top row is mostly sky; big trees may still reach it.
            assert_eq!(top > 60, painted, "sky = {sky}: {top}");
            assert!((0..80).all(|x| cv.get(x, 23).is_some_and(|c| c.ch != ' ')));
        }
    }

    /// Over a few days the tiger hunts: herds notice it and bolt, and some
    /// of its pounces land. It is never hurt and never leaves the ground
    /// for long, and with the saucer off it never goes anywhere.
    #[test]
    fn hunts_happen_and_prey_flees() {
        for kind in ["bengal", "siberian"] {
            let mut s = scene(&format!("day = 60\nufo = 0\nkind = '{kind}'"), 100, 30);
            let mut moods = Vec::new();
            let mut swam = false;
            let mut cv = Canvas::new(100, 30, Color::Reset);
            let theme = Theme::classic();
            let o = opts("");
            for i in 0..30 * 60 * 10 {
                frame(&mut s);
                let t = &s.jungle.tiger;
                assert!(!t.gone, "{kind}: taken with the saucer off");
                assert!(
                    t.x.is_finite() && t.lift >= 0.0 && t.lift < 40.0,
                    "{kind}: {} {}",
                    t.x,
                    t.lift
                );
                if !moods.contains(&t.mood) {
                    moods.push(t.mood);
                }
                swam |= t.sink > 0.9;
                if i % 20 == 0 {
                    cv.clear();
                    s.draw(&ctx(&theme, &o), &mut cv);
                }
            }
            let (t, life) = (&s.jungle.tiger, &s.jungle.life);
            assert!(life.bolted > 0, "{kind}: nothing ever ran");
            assert!(t.caught > 0, "{kind}: nothing caught ({} missed)", t.missed);
            assert_eq!(life.eaten, t.caught);
            for m in [
                Mood::Patrol,
                Mood::Stalk,
                Mood::Charge,
                Mood::Pounce,
                Mood::Feed,
            ] {
                assert!(moods.contains(&m), "{kind}: never {m:?}: {moods:?}");
            }
            assert!(moods.len() >= 8, "{kind}: {moods:?}");
            // In the taiga the rivers are frozen: it walks across.
            if kind == "bengal" {
                assert!(moods.contains(&Mood::Drink), "{kind}: never drank");
                assert!(swam, "{kind}: never swam");
            } else {
                assert!(!swam, "{kind}: swam through ice");
            }
            assert_eq!(s.jungle.ufo.visits, 0);
        }
    }

    /// Bears, dogs and crocodiles try it, and every one of them runs.
    #[test]
    fn rivals_always_lose() {
        let mut s = scene("day = 60\nufo = 0", 100, 30);
        let mut fends = 0;
        for _ in 0..30 * 60 * 8 {
            let before = s.jungle.tiger.mood;
            frame(&mut s);
            let t = &s.jungle.tiger;
            if t.mood == Mood::Fend && before != Mood::Fend {
                fends += 1;
            }
            assert!(!t.gone);
            // Whoever comes, it is never the tiger that runs.
            if t.mood == Mood::Fend && t.steps > 60 {
                assert!(
                    t.rival.is_none()
                        || s.jungle
                            .life
                            .rivals
                            .iter()
                            .all(|r| { !matches!(r.state, life::RivalState::Come) })
                );
            }
        }
        assert!(fends > 0, "no rival came");
        assert!(s.jungle.life.routed > 0, "no rival ran");
    }

    /// At every zoom level, the whole tiger stays on screen, whatever the
    /// shape of the terminal.
    #[test]
    fn camera_keeps_the_tiger_in_frame() {
        for zoom in [0, 3, 6] {
            for (w, h) in [(80u16, 24u16), (200, 50), (40, 30), (24, 40), (120, 12)] {
                let mut s = scene(&format!("zoom = {zoom}\nday = 40\nufo = 0"), w, h);
                for i in 0..30 * 90 {
                    frame(&mut s);
                    if i < 60 {
                        // The camera may still be easing in.
                        continue;
                    }
                    let j = &s.jungle;
                    let v = j.view();
                    let (l, t, r, b) = j.tiger.bounds();
                    let tag = format!("zoom {zoom} {w}x{h}");
                    let e = 1.0 + 1.0 / v.z;
                    assert!(l >= v.x - e && r <= v.x + v.w + e, "{tag}: cut on a side");
                    assert!(
                        t >= v.y - e && b <= v.y + v.h + e,
                        "{tag}: cut above or below"
                    );
                }
            }
        }
    }

    /// The saucer comes, takes the tiger up its beam while the jungle holds
    /// still, goes away with it, and brings it back: it lands and roars.
    #[test]
    fn the_saucer_takes_it_and_brings_it_back() {
        let mut s = scene("ufo = 0\nday = 120", 100, 30);
        s.jungle.summon();
        let mut seen = Vec::new();
        let mut was_gone = false;
        let mut froze = false;
        for _ in 0..30 * 120 {
            let before: Vec<f32> = s.jungle.life.beasts.iter().map(|b| b.x).collect();
            frame(&mut s);
            let j = &s.jungle;
            if !seen.contains(&j.ufo.phase) {
                seen.push(j.ufo.phase);
            }
            was_gone |= j.tiger.gone;
            if j.ufo.phase == Phase::Beam {
                // Nothing in the jungle moves while it hangs there.
                let after: Vec<f32> = j.life.beasts.iter().map(|b| b.x).collect();
                if before.len() == after.len() {
                    assert_eq!(before, after);
                    froze = true;
                }
            }
            if j.ufo.returns > 0 && j.ufo.phase == Phase::Gone {
                break;
            }
        }
        let j = &s.jungle;
        assert_eq!(j.ufo.visits, 1);
        assert_eq!(j.ufo.returns, 1, "phases: {seen:?}");
        assert!(was_gone && froze);
        for p in [
            Phase::Come,
            Phase::Beam,
            Phase::Leave,
            Phase::Away,
            Phase::Return,
            Phase::Lower,
            Phase::Go,
        ] {
            assert!(seen.contains(&p), "never {p:?}");
        }
        assert!(!j.tiger.gone && j.tiger.lift == 0.0);
    }

    /// With `ufo = 0` the saucer never comes; at a high rate it soon does.
    #[test]
    fn the_saucer_comes_only_when_asked() {
        let mut off = scene("ufo = 0", 80, 24);
        let mut often = scene("ufo = 3600", 80, 24);
        for _ in 0..30 * 120 {
            frame(&mut off);
            frame(&mut often);
        }
        assert_eq!(off.jungle.ufo.visits, 0);
        assert!(often.jungle.ufo.visits > 0);
    }
}
