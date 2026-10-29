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
//! On Día de Muertos the tiger is an Oaxacan alebrije, striped and dotted in
//! magenta, turquoise and yellow, and the jungle is dressed for the dead
//! coming home: papel picado strung from tree to tree, cempasúchil in the
//! grass and in beds, and by the path the ruins of a stone shrine with an
//! ofrenda and its candles, which the tiger now and then calls on, bowing its
//! head. The saucer becomes a calavera de azúcar with marigold lights and a
//! beam of petals. The higher the holiday, the more of it, and every bit of it
//! comes from the seed like the rest of the jungle, so ordinary days are
//! exactly what they were.
//!
//! At Navidad the jungle is strung with foquitos that blink in chase patterns
//! from tree to tree, heno hangs from the cords and the limbs, nochebuenas
//! flower red in the grass and in beds, and by the path stand the ruins with
//! a nacimiento in the doorway (the Niño Dios laid in his manger from
//! Nochebuena), farolitos lighting the way to it, which the tiger calls on,
//! bowing its head. The saucer becomes the Estrella de Belén, a comet with a
//! long tail and a golden beam, and from January 2 to 6 the Reyes Magos cross
//! the hills behind on a horse, a camel and an elephant. The posadas climb to
//! Nochebuena, and the jungle with them.
//!
//! On Fiestas Patrias the tiger is a jaguar, golden with rosettes, and the mes
//! patrio dresses the forest in verde, blanco y rojo: papel picado with the
//! águila, the campana and the star cut into it strung from tree to tree,
//! nopales with their red tunas in the undergrowth, and by the path the ruins
//! with a great flag in their doorway, flags on their poles and, over the wall,
//! the águila on its nopal with the serpent in its beak, which the jaguar calls
//! on and roars at, "¡Viva México!". The saucer becomes the Campana de
//! Dolores, swinging on its yoke and ringing, and the fireflies glow in the
//! three colors. The month builds up to the Grito on the 15th.
//!
//! At the Equinoccio the ruins become El Castillo of Chichén Itzá: nine
//! terraces, the temple on top, the stairway up its north face. As the
//! afternoon sun goes down, seven triangles of light slither one after another
//! down the stairway's balustrade, as the shadows of the terraces do on March
//! 20 and 21, to the stone head of Kukulcán at its foot, whose eyes light up
//! when it arrives. The tiger comes to sit before it and watch.
//!
//! - `art.rs`: every sprite, drawn by hand one character per pixel: the
//!   tiger's torso, heads (calm, blinking, snarling, roaring), its front view
//!   and its sleep, and the alebrije's likenesses of them; deer, boar,
//!   peafowl, langurs, birds, the crocodile, the bear, the dholes and the
//!   saucer; and how each animal is built.
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
//! - `fiestas.rs`: what the holidays share: which ruins and flowers there are,
//!   how the saucer is framed, and matching theme colors by hue.
//! - `muertos.rs`: Día de Muertos: its colors (the holiday's own under
//!   `classic`, the theme's otherwise), the papel picado, the marigolds, the
//!   ruins with their ofrendas, the skull saucer and its petal beam.
//! - `navidad.rs`: Navidad: its colors, the foquitos and heno, the
//!   nochebuenas, the nacimientos and farolitos, the Reyes' caravan and the
//!   comet of the Estrella de Belén.
//! - `equinoccio.rs`: the Equinoccio: its colors, El Castillo and the serpent
//!   of light that comes down its stairway with the afternoon.
//! - `patrias.rs`: Fiestas Patrias: its colors, the jaguar's, the papel picado
//!   in three colors, the nopales, the ruins with their emblem and flags, and
//!   the Campana de Dolores.
//! - `jungle.rs`: the ground, the water and the forest layers, the light,
//!   the camera, and the projection onto the screen.
//!
//! The world steps 30 times a second on an accumulator scaled by `speed`,
//! while the clocks (the day, how long a nap or a meal lasts, when the tiger
//! is hungry again, when the saucer comes and brings it back) follow the wall
//! clock.

mod art;
mod body;
mod equinoccio;
mod fiestas;
mod hunter;
mod jungle;
mod life;
mod muertos;
mod navidad;
mod patrias;
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
    /// The holiday's colors, while there is a holiday: at most one of these.
    fest: Option<muertos::Dress>,
    navidad: Option<navidad::Dress>,
    patrias: Option<patrias::Dress>,
    equinoccio: Option<equinoccio::Dress>,
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
        let fest = ctx
            .fiesta
            .is(crate::fiesta::Kind::Muertos)
            .then(|| muertos::Dress::new(ctx));
        let navidad = ctx
            .fiesta
            .is(crate::fiesta::Kind::Navidad)
            .then(|| navidad::Dress::new(ctx));
        let patrias = ctx
            .fiesta
            .is(crate::fiesta::Kind::Patrias)
            .then(|| patrias::Dress::new(ctx));
        let equinoccio = ctx
            .fiesta
            .is(crate::fiesta::Kind::Equinoccio)
            .then(|| equinoccio::Dress::new(ctx));
        // The sky of the equinox is a little bluer, under the native theme.
        let (top, bot) = match equinoccio {
            Some(d) if ctx.theme.native => (top.lerp(d.cielo, 0.3), bot.lerp(d.cielo, 0.25)),
            _ => (top, bot),
        };
        let native = Pal {
            day_top: top,
            day_bot: bot,
            green: Rgb::BLACK,
            green_k: 0.0,
            fest,
            navidad,
            patrias,
            equinoccio,
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
                ..native
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
        if !self.jungle.pal.visits() {
            self.jungle.undress(&mut self.rng);
        }
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
    use crate::fiesta::Fiesta;
    use crate::scene::Opts;
    use crate::theme::{TermColors, Theme};
    use hunter::Mood;
    use ufo::Phase;

    fn ctx<'a>(theme: &'a Theme, opts: &'a Opts) -> Ctx<'a> {
        fiesta_ctx(theme, opts, Fiesta::NONE)
    }

    fn fiesta_ctx<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts,
            bg: Color::Reset,
            seed: 3,
            fiesta,
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

    /// Plays a scene for `secs` seconds of 30 frames, drawing every `every`
    /// frames, and hands each drawn frame to `f`; stops early when it says so.
    fn play(
        s: &mut TigerScene,
        c: &Ctx,
        (w, h): (u16, u16),
        secs: u32,
        every: u32,
        mut f: impl FnMut(&TigerScene) -> bool,
    ) {
        let mut cv = Canvas::new(w, h, Color::Reset);
        for i in 0..30 * secs {
            s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
            if i % every == 0 {
                cv.clear();
                s.draw(c, &mut cv);
                if f(s) {
                    return;
                }
            }
        }
    }

    /// The holiday's elements: the alebrije, papel picado, marigolds and, a
    /// while along the path, an ofrenda at a ruin with its candles lit; or
    /// strings of foquitos and heno, nochebuenas, the layers of a nacimiento,
    /// the Niño Dios and the Reyes.
    #[derive(Default, Debug)]
    struct Seen {
        alebrije: bool,
        jaguar: bool,
        emblems: u32,
        castillos: u32,
        triangles: u32,
        flags: u32,
        blossoms: u32,
        ruins: u32,
        candles: u32,
        bulbs: u32,
        heno: u32,
        belenes: u32,
        ninos: u32,
        reyes: u32,
    }

    impl Seen {
        /// Nothing of any holiday on screen.
        fn nothing(&self) -> bool {
            !self.alebrije
                && !self.jaguar
                && self.emblems == 0
                && (self.castillos, self.triangles) == (0, 0)
                && (self.flags, self.blossoms, self.ruins, self.candles) == (0, 0, 0, 0)
                && (self.bulbs, self.heno, self.belenes, self.ninos, self.reyes) == (0, 0, 0, 0, 0)
        }
    }

    fn seen_over(fiesta: Fiesta, theme: &Theme, kind: &str, secs: u32) -> Seen {
        let o = opts(&format!("kind = '{kind}'\nufo = 0\nzoom = 1"));
        let c = fiesta_ctx(theme, &o, fiesta);
        let mut s = TigerScene::new(&c);
        s.resize(&c, 200, 50);
        s.retheme(&c);
        let mut seen = Seen::default();
        play(&mut s, &c, (200, 50), secs, 15, |s| {
            let p = &s.jungle.painted;
            seen.alebrije |= p.alebrije;
            seen.jaguar |= p.jaguar;
            seen.emblems += p.emblems;
            seen.castillos += p.castillos;
            seen.triangles = seen.triangles.max(p.triangles);
            seen.flags += p.flags;
            seen.blossoms += p.blossoms;
            seen.ruins += p.ruins;
            seen.candles += p.candles;
            seen.bulbs += p.bulbs;
            seen.heno += p.heno;
            seen.belenes += p.belenes;
            seen.ninos += p.ninos;
            seen.reyes += p.reyes;
            false
        });
        seen
    }

    /// At the height of the holiday and on its first day, the tiger is an
    /// alebrije and the jungle is dressed; on an ordinary day nothing is.
    #[test]
    fn the_holiday_dresses_the_jungle() {
        let theme = Theme::classic();
        for kind in ["bengal", "siberian", "white"] {
            for fiesta in [Fiesta::of_day(11, 2), Fiesta::of_day(10, 28)] {
                let seen = seen_over(fiesta, &theme, kind, 150);
                assert!(seen.alebrije, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.flags > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.blossoms > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.ruins > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.candles > 0, "{kind} {fiesta:?}: {seen:?}");
            }
            let seen = seen_over(Fiesta::NONE, &theme, kind, 150);
            assert!(!seen.alebrije, "{kind}: {seen:?}");
            assert_eq!(
                (seen.flags, seen.blossoms, seen.ruins, seen.candles),
                (0, 0, 0, 0),
                "{kind}: {seen:?}"
            );
        }
    }

    /// The higher the holiday, the more of it: more papel picado, and the
    /// ofrenda gains candles as its layers pile up.
    #[test]
    fn the_holiday_builds_up() {
        let theme = Theme::classic();
        let low = seen_over(Fiesta::of_day(10, 28), &theme, "bengal", 240);
        let high = seen_over(Fiesta::of_day(11, 2), &theme, "bengal", 240);
        assert!(low.ruins > 0 && high.ruins > 0, "{low:?} {high:?}");
        assert!(high.flags > low.flags, "{low:?} {high:?}");
        assert!(high.blossoms > low.blossoms, "{low:?} {high:?}");
        assert!(
            high.candles * low.ruins > low.candles * high.ruins,
            "{low:?} {high:?}"
        );
    }

    /// Switching the holiday on and off through `retheme` dresses the jungle
    /// and undresses it again, at once.
    #[test]
    fn the_holiday_comes_and_goes_with_retheme() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nzoom = 1");
        let off = fiesta_ctx(&theme, &o, Fiesta::NONE);
        let on = fiesta_ctx(&theme, &o, Fiesta::of_day(11, 2));
        let mut s = TigerScene::new(&off);
        s.resize(&off, 200, 50);
        s.retheme(&off);
        let mut cv = Canvas::new(200, 50, Color::Reset);
        for _ in 0..3 {
            for (c, dressed) in [(&on, true), (&off, false), (&on, true), (&off, false)] {
                s.retheme(c);
                for _ in 0..5 {
                    s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
                }
                cv.clear();
                s.draw(c, &mut cv);
                let p = &s.jungle.painted;
                assert_eq!(p.alebrije, dressed, "{p:?}");
                assert_eq!(p.flags > 0 || p.blossoms > 0, dressed, "{p:?}");
                if !dressed {
                    assert_eq!((p.ruins, p.candles, p.petals), (0, 0, 0), "{p:?}");
                }
            }
        }
        // A tiger on its way to an ofrenda is called off when the holiday ends.
        s.retheme(&on);
        s.jungle.tiger.set(Mood::Visit);
        s.retheme(&off);
        assert_ne!(s.jungle.tiger.mood, Mood::Visit);
    }

    /// Odd sizes, odd options and huge steps survive with the holiday on,
    /// under classic and under other themes.
    #[test]
    fn the_holiday_survives_odd_sizes_and_steps() {
        let cases = [
            "speed = 50.0\nday = 1\nzoom = 99\nsky = false\nprey = 10.0\nkind = 'random'\nufo = 3600.0",
            "speed = 0.0\nday = 999999999\nzoom = -4\nprey = -3.0\nkind = 'nope'\nufo = -1.0",
            "kind = 'siberian'\nprey = 0\nufo = nan",
            "kind = 'white'\nzoom = 6\nspeed = inf",
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
                for day in [(10, 28), (11, 2), (11, 3)] {
                    let c = fiesta_ctx(theme, &o, Fiesta::of_day(day.0, day.1));
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (2, 3), (7, 2), (1, 40), (300, 90), (80, 24)] {
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
    }

    /// With the holiday on the tiger now and then calls on an ofrenda: it
    /// walks to the ruin, faces it and bows its head a while.
    #[test]
    fn the_tiger_calls_on_the_ofrenda() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nday = 60");
        let c = fiesta_ctx(&theme, &o, Fiesta::of_day(11, 2));
        let mut s = TigerScene::new(&c);
        s.resize(&c, 120, 36);
        s.retheme(&c);
        let (mut bowed, mut walked) = (false, false);
        for _ in 0..30 * 60 * 12 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let t = &s.jungle.tiger;
            if t.mood == Mood::Visit {
                walked = true;
                if t.mouth == 0.0 && t.steps > 10 && t.speed < 0.02 {
                    bowed = true;
                }
            }
            assert!(!t.gone && t.x.is_finite());
        }
        assert!(walked && bowed, "walked {walked}, bowed {bowed}");
    }

    /// On an ordinary day it never does, whatever the ruins.
    #[test]
    fn the_tiger_calls_on_nobody_on_an_ordinary_day() {
        let mut s = scene("ufo = 0\nday = 60", 120, 36);
        for _ in 0..30 * 60 * 6 {
            frame(&mut s);
            assert_ne!(s.jungle.tiger.mood, Mood::Visit);
        }
    }

    /// The saucer is a calavera de azúcar on Día de Muertos, and the old
    /// one on any other day.
    #[test]
    fn the_saucer_wears_a_skull_on_the_holiday() {
        let theme = Theme::classic();
        for (fiesta, skull) in [(Fiesta::of_day(11, 2), true), (Fiesta::NONE, false)] {
            let o = opts("ufo = 0\nday = 120");
            let c = fiesta_ctx(&theme, &o, fiesta);
            let mut s = TigerScene::new(&c);
            s.resize(&c, 100, 30);
            s.retheme(&c);
            s.jungle.summon();
            let mut cv = Canvas::new(100, 30, Color::Reset);
            let mut beamed = false;
            for _ in 0..30 * 60 {
                s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
                cv.clear();
                s.draw(&c, &mut cv);
                if s.jungle.ufo.phase == Phase::Beam {
                    beamed = true;
                    assert_eq!(s.jungle.painted.saucer, skull);
                }
            }
            assert!(beamed);
        }
    }

    /// The holiday's colors come from the theme, and stay told apart.
    #[test]
    fn the_dress_keeps_its_colors_apart() {
        for name in crate::theme::names() {
            let Some(theme) = Theme::builtin(name) else {
                continue;
            };
            let o = opts("");
            let c = fiesta_ctx(&theme, &o, Fiesta::of_day(11, 2));
            let d = muertos::Dress::new(&c);
            assert_ne!(d.rosa, d.turquesa, "{name}");
            for kind in Kind::ALL {
                let mut colors: Vec<Rgb> = d.alebrije(kind).iter().map(|&(_, c)| c).collect();
                colors.sort_by_key(|c| (c.0, c.1, c.2));
                colors.dedup();
                assert!(colors.len() >= 10, "{name} {kind:?}: {colors:?}");
            }
            assert!(d.flags.iter().all(|f| *f != Rgb::BLACK), "{name}");
        }
    }

    // ----- Navidad -----

    /// At the height of the Navidad and on its first day, the jungle is strung with foquitos and
    /// heno, nochebuenas flower, and a nacimiento stands in the ruins with its farolitos; on an
    /// ordinary day none of it is there. The tiger is the tiger, whatever the kind.
    #[test]
    fn navidad_dresses_the_jungle() {
        let theme = Theme::classic();
        for kind in ["bengal", "siberian", "white"] {
            for fiesta in [Fiesta::of_day(12, 24), Fiesta::of_day(12, 12)] {
                let seen = seen_over(fiesta, &theme, kind, 150);
                assert!(
                    !seen.alebrije && seen.flags == 0,
                    "{kind} {fiesta:?}: {seen:?}"
                );
                assert!(seen.bulbs > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.heno > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.blossoms > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.ruins > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.belenes > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.candles > 0, "{kind} {fiesta:?}: {seen:?}");
            }
            let seen = seen_over(Fiesta::NONE, &theme, kind, 150);
            assert!(seen.nothing(), "{kind}: {seen:?}");
        }
    }

    /// The holidays it has no costume for leave it as on an ordinary day: not one of them paints
    /// a thing of its own over the jungle.
    #[test]
    fn the_other_holidays_stay_ordinary() {
        use crate::fiesta::Kind as Day;
        let theme = Theme::classic();
        for kind in Day::ALL {
            if matches!(
                kind,
                Day::Muertos | Day::Navidad | Day::Patrias | Day::Equinoccio
            ) {
                continue;
            }
            let fiesta = Fiesta::height(kind);
            assert!(fiesta.is(kind), "{kind:?}");
            let seen = seen_over(fiesta, &theme, "bengal", 90);
            assert!(seen.nothing(), "{kind:?}: {seen:?}");
        }
    }

    /// The posadas climb to Nochebuena: more foquitos, more of the nacimiento.
    #[test]
    fn navidad_builds_up() {
        let theme = Theme::classic();
        let low = seen_over(Fiesta::of_day(12, 12), &theme, "bengal", 240);
        let high = seen_over(Fiesta::of_day(12, 24), &theme, "bengal", 240);
        assert!(low.ruins > 0 && high.ruins > 0, "{low:?} {high:?}");
        assert!(high.bulbs > low.bulbs, "{low:?} {high:?}");
        assert!(high.blossoms > low.blossoms, "{low:?} {high:?}");
        assert!(
            high.candles * low.ruins > low.candles * high.ruins,
            "{low:?} {high:?}"
        );
        assert!(
            high.belenes * low.ruins > low.belenes * high.ruins,
            "{low:?} {high:?}"
        );
    }

    /// The Niño Dios is laid in his manger on Nochebuena and stays there to the Reyes; before
    /// then the manger is empty.
    #[test]
    fn the_nino_dios_comes_at_nochebuena() {
        let theme = Theme::classic();
        for (month, day, nino) in [
            (12, 16, false),
            (12, 23, false),
            (12, 24, true),
            (12, 28, true),
            (1, 3, true),
        ] {
            let seen = seen_over(Fiesta::of_day(month, day), &theme, "bengal", 240);
            assert!(seen.ruins > 0, "{month}/{day}: {seen:?}");
            assert_eq!(seen.ninos > 0, nino, "{month}/{day}: {seen:?}");
        }
    }

    /// The Reyes Magos cross the hills from January 2 to 6, and no other day.
    #[test]
    fn the_reyes_cross_on_their_days() {
        let theme = Theme::classic();
        for (month, day, reyes) in [
            (1, 1, false),
            (1, 2, true),
            (1, 4, true),
            (1, 6, true),
            (12, 24, false),
            (12, 28, false),
        ] {
            let seen = seen_over(Fiesta::of_day(month, day), &theme, "bengal", 150);
            assert_eq!(seen.reyes > 0, reyes, "{month}/{day}: {seen:?}");
        }
        // In the taiga the hills are firs, and they do not cross.
        let seen = seen_over(Fiesta::of_day(1, 3), &theme, "siberian", 150);
        assert_eq!(seen.reyes, 0, "{seen:?}");
    }

    /// Switching Navidad on and off through `retheme` dresses the jungle and undresses it again,
    /// at once, and a tiger on its way to a nacimiento is called off.
    #[test]
    fn navidad_comes_and_goes_with_retheme() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nzoom = 1");
        let off = fiesta_ctx(&theme, &o, Fiesta::NONE);
        let on = fiesta_ctx(&theme, &o, Fiesta::of_day(12, 24));
        let mut s = TigerScene::new(&off);
        s.resize(&off, 200, 50);
        s.retheme(&off);
        let mut cv = Canvas::new(200, 50, Color::Reset);
        for _ in 0..3 {
            for (c, dressed) in [(&on, true), (&off, false), (&on, true), (&off, false)] {
                s.retheme(c);
                for _ in 0..5 {
                    s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
                }
                cv.clear();
                s.draw(c, &mut cv);
                let p = &s.jungle.painted;
                assert_eq!(
                    p.bulbs > 0 || p.heno > 0 || p.blossoms > 0,
                    dressed,
                    "{p:?}"
                );
                if !dressed {
                    assert_eq!((p.ruins, p.candles, p.belenes), (0, 0, 0), "{p:?}");
                }
            }
        }
        s.retheme(&on);
        s.jungle.tiger.set(Mood::Visit);
        s.retheme(&off);
        assert_ne!(s.jungle.tiger.mood, Mood::Visit);
    }

    /// Odd sizes, odd options and huge steps survive Navidad, on any of its days, under classic
    /// and under other themes.
    #[test]
    fn navidad_survives_odd_sizes_and_steps() {
        let cases = [
            "speed = 50.0\nday = 1\nzoom = 99\nsky = false\nprey = 10.0\nkind = 'random'\nufo = 3600.0",
            "speed = 0.0\nday = 999999999\nzoom = -4\nprey = -3.0\nkind = 'nope'\nufo = -1.0",
            "kind = 'siberian'\nprey = 0\nufo = nan",
            "kind = 'white'\nzoom = 6\nspeed = inf\nufo = 3600.0",
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
                for day in [(12, 12), (12, 24), (12, 28), (1, 1), (1, 3), (1, 6)] {
                    let c = fiesta_ctx(theme, &o, Fiesta::of_day(day.0, day.1));
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (2, 3), (7, 2), (1, 40), (300, 90), (80, 24)] {
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
    }

    /// With Navidad on the tiger now and then calls on a nacimiento: it walks to the ruin, faces
    /// it and bows its head a while.
    #[test]
    fn the_tiger_calls_on_the_nacimiento() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nday = 60");
        let c = fiesta_ctx(&theme, &o, Fiesta::of_day(12, 24));
        let mut s = TigerScene::new(&c);
        s.resize(&c, 120, 36);
        s.retheme(&c);
        let (mut bowed, mut walked) = (false, false);
        for _ in 0..30 * 60 * 12 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let t = &s.jungle.tiger;
            if t.mood == Mood::Visit {
                walked = true;
                if t.mouth == 0.0 && t.steps > 10 && t.speed < 0.02 {
                    bowed = true;
                }
            }
            assert!(!t.gone && t.x.is_finite());
        }
        assert!(walked && bowed, "walked {walked}, bowed {bowed}");
    }

    /// The saucer is the Estrella de Belén at Navidad: a comet with a long tail, and no skull.
    #[test]
    fn the_saucer_is_a_comet_at_navidad() {
        let theme = Theme::classic();
        for (fiesta, comet, skull) in [
            (Fiesta::of_day(12, 24), true, false),
            (Fiesta::of_day(11, 2), false, true),
            (Fiesta::NONE, false, false),
        ] {
            let o = opts("ufo = 0\nday = 120");
            let c = fiesta_ctx(&theme, &o, fiesta);
            let mut s = TigerScene::new(&c);
            s.resize(&c, 100, 30);
            s.retheme(&c);
            s.jungle.summon();
            let mut cv = Canvas::new(100, 30, Color::Reset);
            let mut beamed = false;
            for _ in 0..30 * 60 {
                s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
                cv.clear();
                s.draw(&c, &mut cv);
                if s.jungle.ufo.phase == Phase::Beam {
                    beamed = true;
                    assert_eq!(s.jungle.painted.comet, comet);
                    assert_eq!(s.jungle.painted.saucer, skull);
                }
            }
            assert!(beamed);
        }
    }

    /// Navidad's colors come from the theme, and stay told apart.
    #[test]
    fn the_navidad_dress_keeps_its_colors_apart() {
        for name in crate::theme::names() {
            let Some(theme) = Theme::builtin(name) else {
                continue;
            };
            let o = opts("");
            let c = fiesta_ctx(&theme, &o, Fiesta::of_day(12, 24));
            let d = navidad::Dress::new(&c);
            assert_ne!(d.rojo, d.verde, "{name}");
            let mut colors: Vec<Rgb> = d.bulbs.to_vec();
            colors.sort_by_key(|c| (c.0, c.1, c.2));
            colors.dedup();
            assert!(colors.len() >= 4, "{name}: {colors:?}");
            let mut belen: Vec<Rgb> = d.belen().iter().map(|&(_, c)| c).collect();
            belen.sort_by_key(|c| (c.0, c.1, c.2));
            belen.dedup();
            assert!(belen.len() >= 12, "{name}: {belen:?}");
            assert!(d.bulbs.iter().all(|c| *c != Rgb::BLACK), "{name}");
        }
    }

    // ----- Fiestas Patrias -----

    /// During the mes patrio the tiger is a jaguar, the jungle is strung with papel picado in
    /// three colors, nopales grow in the undergrowth, and the ruins carry their flags and their
    /// emblem; on an ordinary day nothing is.
    #[test]
    fn patrias_dresses_the_jungle() {
        let theme = Theme::classic();
        for kind in ["bengal", "siberian", "white"] {
            for fiesta in [Fiesta::of_day(9, 15), Fiesta::of_day(9, 1)] {
                let seen = seen_over(fiesta, &theme, kind, 150);
                assert!(seen.jaguar && !seen.alebrije, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.flags > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.blossoms > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.ruins > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.emblems > 0, "{kind} {fiesta:?}: {seen:?}");
                assert_eq!(seen.bulbs + seen.heno + seen.belenes, 0, "{seen:?}");
            }
        }
    }

    /// The month builds up to the Grito: more papel picado, more nopales, more of everything.
    #[test]
    fn patrias_builds_up() {
        let theme = Theme::classic();
        let low = seen_over(Fiesta::of_day(9, 1), &theme, "bengal", 240);
        let high = seen_over(Fiesta::of_day(9, 15), &theme, "bengal", 240);
        assert!(low.ruins > 0 && high.ruins > 0, "{low:?} {high:?}");
        assert!(high.flags > low.flags, "{low:?} {high:?}");
        assert!(high.blossoms > low.blossoms, "{low:?} {high:?}");
    }

    /// Switching the holiday on and off through `retheme` makes the tiger a jaguar and a tiger
    /// again, at once, and a jaguar on its way to the emblem is called off.
    #[test]
    fn patrias_comes_and_goes_with_retheme() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nzoom = 1");
        let off = fiesta_ctx(&theme, &o, Fiesta::NONE);
        let on = fiesta_ctx(&theme, &o, Fiesta::of_day(9, 15));
        let mut s = TigerScene::new(&off);
        s.resize(&off, 200, 50);
        s.retheme(&off);
        let mut cv = Canvas::new(200, 50, Color::Reset);
        for _ in 0..3 {
            for (c, dressed) in [(&on, true), (&off, false), (&on, true), (&off, false)] {
                s.retheme(c);
                for _ in 0..5 {
                    s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
                }
                cv.clear();
                s.draw(c, &mut cv);
                let p = &s.jungle.painted;
                assert_eq!(p.jaguar, dressed, "{p:?}");
                assert_eq!(p.flags > 0 || p.blossoms > 0, dressed, "{p:?}");
                if !dressed {
                    assert_eq!((p.ruins, p.emblems), (0, 0), "{p:?}");
                }
            }
        }
        s.retheme(&on);
        s.jungle.tiger.set(Mood::Visit);
        s.retheme(&off);
        assert_ne!(s.jungle.tiger.mood, Mood::Visit);
    }

    /// Odd sizes, odd options and huge steps survive Fiestas Patrias, on any of its days, under
    /// classic and under other themes.
    #[test]
    fn patrias_survives_odd_sizes_and_steps() {
        let cases = [
            "speed = 50.0\nday = 1\nzoom = 99\nsky = false\nprey = 10.0\nkind = 'random'\nufo = 3600.0",
            "speed = 0.0\nday = 999999999\nzoom = -4\nprey = -3.0\nkind = 'nope'\nufo = -1.0",
            "kind = 'siberian'\nprey = 0\nufo = nan",
            "kind = 'white'\nzoom = 6\nspeed = inf\nufo = 3600.0",
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
                for day in [(9, 1), (9, 8), (9, 15), (9, 16)] {
                    let c = fiesta_ctx(theme, &o, Fiesta::of_day(day.0, day.1));
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (2, 3), (7, 2), (1, 40), (300, 90), (80, 24)] {
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
    }

    /// The jaguar calls on the ruins too: it walks to the emblem over the wall, looks up at it and
    /// roars "¡Viva México!", which shows over its head.
    #[test]
    fn the_jaguar_roars_at_the_emblem() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nday = 60");
        let c = fiesta_ctx(&theme, &o, Fiesta::of_day(9, 15));
        let mut s = TigerScene::new(&c);
        s.resize(&c, 120, 36);
        s.retheme(&c);
        let (mut walked, mut roared, mut cheered) = (false, false, false);
        for _ in 0..30 * 60 * 12 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let t = &s.jungle.tiger;
            if t.mood == Mood::Visit {
                walked = true;
                roared |= t.mouth > 0.9;
            }
            cheered |= s.jungle.notes.iter().any(|n| n.text.contains("VIVA"));
            assert!(!t.gone && t.x.is_finite());
        }
        assert!(walked && roared && cheered, "{walked} {roared} {cheered}");
    }

    /// The saucer is the Campana de Dolores on Fiestas Patrias, ringing: no skull, no comet.
    #[test]
    fn the_saucer_is_a_bell_at_patrias() {
        let theme = Theme::classic();
        for (fiesta, bell) in [
            (Fiesta::of_day(9, 15), true),
            (Fiesta::of_day(12, 24), false),
        ] {
            let o = opts("ufo = 0\nday = 120");
            let c = fiesta_ctx(&theme, &o, fiesta);
            let mut s = TigerScene::new(&c);
            s.resize(&c, 100, 30);
            s.retheme(&c);
            s.jungle.summon();
            let mut cv = Canvas::new(100, 30, Color::Reset);
            let (mut beamed, mut rang) = (false, false);
            for _ in 0..30 * 60 {
                s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
                cv.clear();
                s.draw(&c, &mut cv);
                rang |= s
                    .jungle
                    .notes
                    .iter()
                    .any(|n| n.text == "DON" || n.text == "DAN");
                if s.jungle.ufo.phase == Phase::Beam {
                    beamed = true;
                    assert_eq!(s.jungle.painted.campana, bell);
                    assert!(!s.jungle.painted.saucer);
                }
            }
            assert!(beamed);
            assert_eq!(rang, bell);
        }
    }

    /// The colors of Fiestas Patrias come from the theme, and stay told apart; the jaguar wears
    /// them in every kind.
    #[test]
    fn the_patrias_dress_keeps_its_colors_apart() {
        for name in crate::theme::names() {
            let Some(theme) = Theme::builtin(name) else {
                continue;
            };
            let o = opts("");
            let c = fiesta_ctx(&theme, &o, Fiesta::of_day(9, 15));
            let d = patrias::Dress::new(&c);
            assert_ne!(d.verde, d.rojo, "{name}");
            for kind in Kind::ALL {
                let mut colors: Vec<Rgb> = d.jaguar(kind).iter().map(|&(_, c)| c).collect();
                colors.sort_by_key(|c| (c.0, c.1, c.2));
                colors.dedup();
                assert!(colors.len() >= 9, "{name} {kind:?}: {colors:?}");
            }
            let papel = d.papel();
            assert!(papel.flags.iter().all(|f| *f != Rgb::BLACK), "{name}");
        }
    }

    // ----- Equinoccio -----

    /// On the days of the equinox the ruins are El Castillo, with the serpent of light coming
    /// down its stairway in the afternoon; on an ordinary day there is none.
    #[test]
    fn equinoccio_dresses_the_jungle() {
        let theme = Theme::classic();
        for kind in ["bengal", "siberian", "white"] {
            for fiesta in [Fiesta::of_day(3, 20), Fiesta::of_day(3, 19)] {
                let seen = seen_over(fiesta, &theme, kind, 240);
                assert!(
                    !seen.alebrije && !seen.jaguar,
                    "{kind} {fiesta:?}: {seen:?}"
                );
                assert!(seen.castillos > 0, "{kind} {fiesta:?}: {seen:?}");
                assert!(seen.triangles > 0, "{kind} {fiesta:?}: {seen:?}");
                assert_eq!(seen.ruins + seen.flags + seen.bulbs, 0, "{seen:?}");
            }
        }
    }

    /// The serpent forms with the afternoon sun, holds, and fades as it sets; the higher the
    /// holiday, the further down it comes.
    #[test]
    fn the_serpent_follows_the_afternoon() {
        let theme = Theme::classic();
        let o = opts("ufo = 0");
        let mut reach = Vec::new();
        for day in [(3, 20), (3, 19)] {
            let c = fiesta_ctx(&theme, &o, Fiesta::of_day(day.0, day.1));
            let mut s = TigerScene::new(&c);
            s.resize(&c, 100, 30);
            s.retheme(&c);
            let d = s.jungle.pal.equinoccio.expect("dressed");
            let at = |s: &mut TigerScene, t: f32| {
                s.jungle.t = t;
                s.jungle.serpent(&d)
            };
            assert_eq!(at(&mut s, 0.3), 0.0, "morning");
            assert_eq!(at(&mut s, 0.5), 0.0, "noon");
            let (early, later, late) = (at(&mut s, 0.6), at(&mut s, 0.64), at(&mut s, 0.7));
            assert!(
                0.0 < early && early < later && later < late,
                "{early} {later} {late}"
            );
            assert!(at(&mut s, 0.79) < late && at(&mut s, 0.79) > 0.0);
            assert_eq!(at(&mut s, 0.9), 0.0, "night");
            reach.push(late);
        }
        // At the height of the holiday it reaches the head; two days off it falls short.
        assert!(reach[0] > 0.99 && reach[1] < 0.9, "{reach:?}");
    }

    /// Played through, El Castillo shows all seven triangles at the height of the holiday, and
    /// fewer than that two days off.
    #[test]
    fn the_serpent_comes_down_the_stairway() {
        let theme = Theme::classic();
        let most = |fiesta: Fiesta| {
            let o = opts("ufo = 0\nzoom = 0");
            let c = fiesta_ctx(&theme, &o, fiesta);
            let mut s = TigerScene::new(&c);
            s.resize(&c, 200, 60);
            s.retheme(&c);
            let mut most = 0;
            play(&mut s, &c, (200, 60), 400, 9, |s| {
                most = most.max(s.jungle.painted.triangles);
                false
            });
            most
        };
        let (height, edge) = (most(Fiesta::of_day(3, 20)), most(Fiesta::of_day(3, 19)));
        assert_eq!(height, 7, "{height} {edge}");
        assert!(0 < edge && edge < 7, "{height} {edge}");
    }

    /// With the equinox on the tiger comes to sit before El Castillo: it walks over, faces it,
    /// and sits up on its haunches a good while, its mouth shut, then goes its way.
    #[test]
    fn the_tiger_sits_to_watch_the_serpent() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nday = 60");
        let c = fiesta_ctx(&theme, &o, Fiesta::of_day(3, 20));
        let mut s = TigerScene::new(&c);
        s.resize(&c, 120, 36);
        s.retheme(&c);
        let (mut walked, mut sat, mut rose) = (false, false, false);
        for _ in 0..30 * 60 * 12 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let t = &s.jungle.tiger;
            if t.mood == Mood::Visit {
                walked = true;
                sat |= t.sit > 0.95 && t.mouth == 0.0 && t.speed < 0.02;
            } else if sat && t.sit < 0.05 {
                rose = true;
            }
            assert!(!t.gone && t.x.is_finite() && t.sit.is_finite());
        }
        assert!(
            walked && sat && rose,
            "walked {walked}, sat {sat}, rose {rose}"
        );
    }

    /// Switching the equinox on and off through `retheme` builds and takes down El Castillo at
    /// once, and a tiger on its way to sit before it is called off.
    #[test]
    fn equinoccio_comes_and_goes_with_retheme() {
        let theme = Theme::classic();
        let o = opts("ufo = 0\nzoom = 1");
        let off = fiesta_ctx(&theme, &o, Fiesta::NONE);
        let on = fiesta_ctx(&theme, &o, Fiesta::of_day(3, 20));
        let mut s = TigerScene::new(&off);
        s.resize(&off, 200, 50);
        s.retheme(&off);
        let mut cv = Canvas::new(200, 50, Color::Reset);
        let mut built = false;
        for _ in 0..4 {
            for (c, dressed) in [(&on, true), (&off, false), (&on, true), (&off, false)] {
                s.retheme(c);
                for _ in 0..90 {
                    s.tick(c, 1.0 / 30.0, 1.0 / 30.0);
                }
                cv.clear();
                s.draw(c, &mut cv);
                let p = &s.jungle.painted;
                built |= p.castillos > 0;
                if !dressed {
                    assert_eq!((p.castillos, p.triangles), (0, 0), "{p:?}");
                }
            }
        }
        assert!(built);
        s.retheme(&on);
        s.jungle.tiger.set(Mood::Visit);
        s.retheme(&off);
        assert_ne!(s.jungle.tiger.mood, Mood::Visit);
    }

    /// Odd sizes, odd options and huge steps survive the equinox, on any of its days, under
    /// classic and under other themes.
    #[test]
    fn equinoccio_survives_odd_sizes_and_steps() {
        let cases = [
            "speed = 50.0\nday = 1\nzoom = 99\nsky = false\nprey = 10.0\nkind = 'random'\nufo = 3600.0",
            "speed = 0.0\nday = 999999999\nzoom = -4\nprey = -3.0\nkind = 'nope'\nufo = -1.0",
            "kind = 'siberian'\nprey = 0\nufo = nan",
            "kind = 'white'\nzoom = 6\nspeed = inf\nufo = 3600.0",
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
                for day in [(3, 19), (3, 20), (3, 21), (3, 22)] {
                    let c = fiesta_ctx(theme, &o, Fiesta::of_day(day.0, day.1));
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (2, 3), (7, 2), (1, 40), (300, 90), (80, 24)] {
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
    }

    /// The colors of the equinox come from the theme, and stay told apart.
    #[test]
    fn the_equinoccio_dress_keeps_its_colors_apart() {
        for name in crate::theme::names() {
            let Some(theme) = Theme::builtin(name) else {
                continue;
            };
            let o = opts("");
            let c = fiesta_ctx(&theme, &o, Fiesta::of_day(3, 20));
            let d = equinoccio::Dress::new(&c);
            assert_ne!(d.sol, d.jade, "{name}");
            assert_ne!(d.caliza, d.sombra, "{name}");
            let mut colors: Vec<Rgb> = d.serpiente().iter().map(|&(_, c)| c).collect();
            colors.sort_by_key(|c| (c.0, c.1, c.2));
            colors.dedup();
            assert!(colors.len() >= 5, "{name}: {colors:?}");
        }
    }
}
