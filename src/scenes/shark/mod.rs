//! shark: a shark patrols the open sea, hunting through the day and night.
//!
//! Original oxiper work, in the manner of the dog scene. The sea is pixel art
//! of a fixed height ([`WORLD_H`]) and endless width, every part of it dealt
//! out from the seed by hashing, so the camera trailing the shark can follow
//! it forever. Faraway ridges and nearer reefs scroll slower (parallax) and
//! fade into the water's haze; shafts of light slant down from a surface seen
//! from below, with the sky glowing through Snell's window, and caustics
//! dance on the sand.
//!
//! - `art.rs`: every sprite, drawn by hand one character per pixel: four
//!   sharks (side view, tail and mouth frames, and the views a turn passes
//!   through), reef pieces, small fish, riders and passers-by.
//! - `hunter.rs`: the shark's moods (cruising, investigating, stalking and
//!   striking, gliding with its fin out, resting by night, filter feeding)
//!   and how its parts are put together, tilted and bent, on screen.
//! - `life.rs`: schools that keep clear of it, rays, turtles, jellyfish,
//!   lanternfish, bubbles, marine snow and glowing plankton.
//! - `water.rs` and `floor.rs`: the backdrop, painted in one pass, and what
//!   grows and lies on the floor; warm seas get coral, cool ones kelp.
//! - `sea.rs`: light by depth and time of day, the camera, and the
//!   projection onto the screen.
//! - `muertos.rs`: Día de Muertos, below.
//! - `navidad.rs`: Navidad, below.
//! - `patrias.rs`: the Fiestas Patrias, below.
//!
//! On Día de Muertos the shark wears calavera face paint (a sugar skull's
//! white face, marigold eye sockets, a stitched grin, gill slits in papel
//! picado colors), the plankton glows like embers, the lanternfish carry
//! candlelight, and sunken ofrendas stand on the sand with their veladoras
//! lit, a path of cempasúchil petals leading to each; the higher the level,
//! the fuller they are. Petals float on the surface and sink slowly to the
//! sand. None of it exists on an ordinary day, and its randomness is its own.
//!
//! On Navidad the jellyfish glow like strings of foquitos, in red, green,
//! blue and gold, blinking in turns; a piñata of siete picos is dropped into
//! the sea now and then and sinks slowly, turning, until the shark rams it
//! and colación, tejocotes, cañas and mandarinas scatter and sink to the
//! sand; and the spots where Día de Muertos puts its ofrendas hold a portal
//! de Belén thatched in heno, strung with foquitos and lit by farolitos. The
//! further into the posadas, the more of all of it.
//!
//! On the Fiestas Patrias the shark wears tricolor face paint (the flag on
//! its cheeks, the eagle on the white, its gill slits in verde, blanco y
//! rojo), the plankton glows in the same three colors, flagpoles stand sunk
//! in the sand where the ofrendas would, their flags rippling in the current
//! and banderitas strung from them, and once in a while a vaquita marina,
//! Mexico's own tiny porpoise, passes by. The mes patrio builds up to the
//! Grito: more flagpoles, more banderitas, the paint on the gills, the
//! vaquita more often and then with her calf.
//!
//! The only things done to the sprites at run time are tilting and bending
//! them (whole pixels moved, none lost or doubled) and lighting them. The
//! world steps 30 times a second on an accumulator scaled by `speed`, while
//! the clocks (the day, how long a rest or a glide lasts, when the shark is
//! hungry again) follow the wall clock.

mod art;
mod floor;
mod hunter;
mod life;
mod muertos;
mod navidad;
mod patrias;
mod sea;
mod water;

use crate::color::Color;
use crate::render::Canvas;
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use sea::Sea;

pub const INFO: SceneInfo = SceneInfo {
    name: "shark",
    aliases: &["sharks"],
    about: "a shark patrols the open sea, hunting through the day and night",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Pace of the shark, the fish and the sea (the day keeps real time)",
        },
        Opt {
            key: "day",
            default: Val::I(180),
            help: "Real seconds for a full day, from dawn through night to dawn (10-86400)",
        },
        Opt {
            key: "zoom",
            default: Val::I(2),
            help: "How close the camera is to the shark, 0 (the whole sea) to 6",
        },
        Opt {
            key: "kind",
            default: Val::S("great-white"),
            help: "great-white, hammerhead, tiger, whale-shark, or random",
        },
        Opt {
            key: "fish",
            default: Val::F(1.0),
            help: "How many fish swim in schools; 0 for none",
        },
        Opt {
            key: "rays",
            default: Val::B(true),
            help: "Shafts of light from the surface and caustics on the sand",
        },
        Opt {
            key: "water",
            default: Val::B(true),
            help: "Paint the water; off leaves it see-through around the sea life",
        },
    ],
    make: |ctx| Box::new(SharkScene::new(ctx)),
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

/// Marks water pixels of the world while the water is see-through.
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
/// Real seconds of a day at which the rests were timed.
const BASE_DAY: f32 = 180.0;
/// How much the theme tints the water and the reef.
const TINT: f32 = 0.3;

/// Height of the world in pixels; its width is endless.
const WORLD_H: i32 = 144;
/// Camera zoom levels, as the number of world pixel rows in view.
const ZOOMS: [f32; 7] = [144.0, 124.0, 108.0, 94.0, 82.0, 72.0, 62.0];

/// The colors a theme changes: the daylit water and the reef's living colors.
/// Under the native theme they are the sea's own.
#[derive(Clone, Copy)]
struct Pal {
    shallow: Rgb,
    deep: Rgb,
    life: Rgb,
    life_k: f32,
    /// The sea's own colors, untouched by a theme.
    native: bool,
}

/// Daylit water just under the surface and down at the sand.
const DAY_SHALLOW: Rgb = Rgb(52, 178, 196);
const DAY_DEEP: Rgb = Rgb(10, 62, 112);

impl Pal {
    fn new(ctx: &Ctx) -> Pal {
        let native = Pal {
            shallow: DAY_SHALLOW,
            deep: DAY_DEEP,
            life: Rgb::BLACK,
            life_k: 0.0,
            native: true,
        };
        if ctx.theme.native {
            return native;
        }
        let g = &ctx.theme.grad;
        let mix = |base: Rgb, c: Color| rgb_of(c).map_or(base, |c| base.lerp(c, TINT));
        match rgb_of(g.at(0.6)) {
            Some(life) => Pal {
                shallow: mix(DAY_SHALLOW, g.at(0.75)),
                deep: mix(DAY_DEEP, g.at(0.2)),
                life,
                life_k: TINT,
                native: false,
            },
            None => native,
        }
    }
}

/// The light at one time of day.
#[derive(Clone, Copy, Debug)]
struct Light {
    /// Water just under the surface and at the bottom.
    shallow: Rgb,
    deep: Rgb,
    /// What lights things up: white by day, dim blue by night.
    tint: Rgb,
    /// The sky seen through the surface.
    sky: Rgb,
}

impl Light {
    fn lerp(self, o: Light, t: f32) -> Light {
        Light {
            shallow: self.shallow.lerp(o.shallow, t),
            deep: self.deep.lerp(o.deep, t),
            tint: self.tint.lerp(o.tint, t),
            sky: self.sky.lerp(o.sky, t),
        }
    }
}

const NIGHT: Light = Light {
    shallow: Rgb(18, 50, 92),
    deep: Rgb(4, 14, 38),
    tint: Rgb(92, 116, 168),
    sky: Rgb(44, 66, 110),
};

/// Time of day keyframes.
fn keys(p: &Pal) -> [(f32, Light); 8] {
    let day = Light {
        shallow: p.shallow,
        deep: p.deep,
        tint: Rgb::WHITE,
        sky: Rgb(214, 246, 250),
    };
    [
        (0.0, NIGHT),
        (0.17, NIGHT),
        (
            0.23,
            Light {
                shallow: Rgb(96, 120, 160),
                deep: Rgb(12, 34, 80),
                tint: Rgb(236, 186, 170),
                sky: Rgb(255, 192, 150),
            },
        ),
        (0.3, day),
        (0.68, day),
        (
            0.75,
            Light {
                shallow: Rgb(108, 98, 146),
                deep: Rgb(14, 28, 74),
                tint: Rgb(240, 168, 138),
                sky: Rgb(255, 152, 96),
            },
        ),
        (0.82, NIGHT),
        (1.0, NIGHT),
    ]
}

/// Which shark the `kind` option asks for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum KindOpt {
    One(art::Kind),
    Random,
}

/// What the options choose.
#[derive(Clone, Copy)]
struct Settings {
    water: bool,
    rays: bool,
    /// School density, 1 being the usual.
    fish: f32,
    /// Index into [`ZOOMS`].
    zoom: usize,
    kind: KindOpt,
    /// Real seconds per day over [`BASE_DAY`]: rests last as long a part of the day.
    rest_scale: f32,
}

/// What the holiday on today dresses the sea in: at most one of them is set.
#[derive(Clone, Copy, Default)]
struct Dressing {
    muertos: Option<muertos::Dress>,
    navidad: Option<navidad::Dress>,
    patrias: Option<patrias::Dress>,
}

impl Dressing {
    fn of(ctx: &Ctx) -> Dressing {
        Dressing {
            muertos: muertos::Dress::of(ctx),
            navidad: navidad::Dress::of(ctx),
            patrias: patrias::Dress::of(ctx),
        }
    }
}

/// The scene: the sea, stepped at a steady pace.
struct SharkScene {
    sea: Sea,
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
    /// Today's holiday, dressed in the theme's colors.
    dressing: Dressing,
}

impl SharkScene {
    fn new(ctx: &Ctx) -> SharkScene {
        let o = ctx.opts;
        let day = o.i("day").clamp(10, 86_400) as f32;
        let kind = match o.s("kind").trim().to_ascii_lowercase().as_str() {
            "random" | "any" => KindOpt::Random,
            s => KindOpt::One(art::Kind::parse(s).unwrap_or(art::Kind::GreatWhite)),
        };
        let set = Settings {
            water: o.b("water"),
            rays: o.b("rays"),
            fish: o.f("fish").clamp(0.0, 10.0),
            zoom: o.i("zoom").clamp(0, ZOOMS.len() as i64 - 1) as usize,
            kind,
            rest_scale: day / BASE_DAY,
        };
        let mut rng = ctx.rng();
        let kind = match set.kind {
            KindOpt::One(k) => k,
            KindOpt::Random => art::Kind::ALL[rng.usize(..art::Kind::ALL.len())],
        };
        let dressing = Dressing::of(ctx);
        let sea = Sea::new(80, 24, &mut rng, Pal::new(ctx), set, kind, dressing);
        SharkScene {
            sea,
            rng,
            set,
            speed: o.f64("speed").clamp(0.0, 50.0),
            day,
            acc: 0.0,
            dirty: true,
            drawn_t: 0.0,
            dressing,
        }
    }
}

impl Scene for SharkScene {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        // The view is built for the screen's shape, so a resize starts a fresh
        // sea with the same kind of shark; the time of day carries over.
        let (t, kind) = (self.sea.t, self.sea.shark.kind);
        self.sea = Sea::new(
            w,
            h,
            &mut self.rng,
            self.sea.pal,
            self.set,
            kind,
            self.dressing,
        );
        self.sea.t = t;
        self.sea.update_light();
        self.dirty = true;
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.sea.pal = Pal::new(ctx);
        self.sea.update_light();
        self.dressing = Dressing::of(ctx);
        self.sea.wear(self.dressing);
        self.dirty = true;
    }

    fn update(&mut self, ctx: &Ctx, dt: f32) {
        self.tick(ctx, dt, dt);
    }

    fn tick(&mut self, _ctx: &Ctx, dt: f32, real: f32) {
        // The clocks first: they run on the wall clock, speed or not.
        if real.is_finite() && real > 0.0 {
            let real = real.min(3600.0);
            self.sea.t = (self.sea.t + real / self.day).rem_euclid(1.0);
            self.sea.update_light();
            self.sea.clocks(real);
            if (self.sea.t - self.drawn_t).abs() >= 0.0004 {
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
            self.sea.update(&mut self.rng);
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
            self.sea.render();
            self.drawn_t = self.sea.t;
            self.dirty = false;
        }
        self.sea.scr.blit(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Opts;
    use crate::theme::Theme;

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

    /// At every zoom level, the whole shark stays on screen through turns,
    /// climbs, dives and strikes, whatever the shape of the terminal.
    #[test]
    fn camera_keeps_the_shark_in_frame() {
        let theme = Theme::classic();
        for kind in art::Kind::ALL {
            for zoom in [0, 3, 6] {
                let opts = opts(&format!("zoom = {zoom}\nday = 40\nkind = '{kind:?}'"));
                let c = ctx(&theme, &opts);
                for (sw, sh) in [(80u16, 24u16), (200, 50), (40, 30), (24, 40), (120, 12)] {
                    let mut s = SharkScene::new(&c);
                    s.resize(&c, sw, sh);
                    for _ in 0..30 * 60 {
                        s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
                        let v = s.sea.view();
                        let (l, t, r, b) = s.sea.shark.bounds();
                        let tag = format!("{kind:?} zoom {zoom} {sw}x{sh}");
                        // The view snaps to whole screen pixels.
                        let e = 1.0 + 0.5 / v.z;
                        assert!(l >= v.x - e && r <= v.x + v.w + e, "{tag}: cut on a side");
                        assert!(
                            t >= v.y - e && b <= v.y + v.h + e,
                            "{tag}: cut above or below"
                        );
                    }
                }
            }
        }
    }

    /// The day follows the wall clock: a tenth of the speed moves the sea
    /// slower but not the sun.
    #[test]
    fn the_day_keeps_real_time() {
        let theme = Theme::classic();
        let opts = opts("day = 100");
        let c = ctx(&theme, &opts);
        let (mut fast, mut slow) = (SharkScene::new(&c), SharkScene::new(&c));
        fast.resize(&c, 80, 24);
        slow.resize(&c, 80, 24);
        let t0 = (fast.sea.t, slow.sea.t);
        for _ in 0..300 {
            fast.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            slow.tick(&c, 0.1 / 30.0, 1.0 / 30.0);
        }
        // Ten real seconds of a hundred-second day.
        for (a, b) in [(fast.sea.t, t0.0), (slow.sea.t, t0.1)] {
            assert!(((a - b).rem_euclid(1.0) - 0.1).abs() < 1e-3, "{a} {b}");
        }
        assert!(
            fast.sea.tick > 290 && slow.sea.tick < 35,
            "{} {}",
            fast.sea.tick,
            slow.sea.tick
        );
    }

    /// A rest near the bottom lasts real seconds, however slowly the sea moves.
    #[test]
    fn rests_follow_the_wall_clock() {
        let theme = Theme::classic();
        let opts = opts("");
        let c = ctx(&theme, &opts);
        let mut s = SharkScene::new(&c);
        s.resize(&c, 80, 24);
        s.sea.shark.mood = hunter::Mood::Rest;
        s.sea.shark.clock = 5.0;
        s.sea.shark.hunger = 5.0;
        for _ in 0..100 {
            s.tick(&c, 0.001, 0.05);
        }
        assert!(s.sea.shark.clock < 0.01, "{}", s.sea.shark.clock);
        assert!(s.sea.shark.hunger < 0.01, "{}", s.sea.shark.hunger);
    }

    /// With the water off nothing is painted in it, but the sea floor and
    /// the shark still are.
    #[test]
    fn see_through_water_stays_empty() {
        let theme = Theme::classic();
        for (water, painted) in [(false, false), (true, true)] {
            let opts = opts(&format!("water = {water}\nzoom = 0"));
            let c = ctx(&theme, &opts);
            let mut s = SharkScene::new(&c);
            s.resize(&c, 80, 24);
            let mut cv = Canvas::new(80, 24, Color::Reset);
            s.draw(&c, &mut cv);
            let empty = |y: i32| {
                (0..80)
                    .filter(|&x| cv.get(x, y).is_some_and(|c| c.ch == ' '))
                    .count()
            };
            // The middle of the water.
            assert_eq!(empty(6) == 0, painted, "water = {water}");
            // The sand at the bottom is always there.
            assert_eq!(empty(23), 0, "water = {water}");
        }
    }

    /// Every kind's drawings fit together: the parts are as tall as the side
    /// view, the tails and heads as wide as the columns they replace, and
    /// every letter has a color.
    #[test]
    fn the_drawings_fit_together() {
        for kind in art::Kind::ALL {
            let rig = kind.rig();
            let (w, h) = (rig.side[0].len(), rig.side.len());
            assert_eq!(
                (w, h),
                (hunter::Shark::W as usize, hunter::Shark::H as usize)
            );
            let all =
                rig.tails
                    .iter()
                    .chain(&rig.heads)
                    .chain([&rig.side, &rig.quarter, &rig.front]);
            for sprite in all {
                assert_eq!(sprite.len(), h, "{kind:?}");
                let sw = sprite[0].len();
                for row in sprite.iter() {
                    assert_eq!(row.len(), sw, "{kind:?}: {row}");
                    for ch in row.bytes().filter(|&c| c != b'.') {
                        assert!(
                            rig.palette.iter().any(|&(k, _)| k == ch),
                            "{kind:?}: no color for {}",
                            ch as char
                        );
                    }
                }
            }
            for t in rig.tails {
                assert_eq!(t[0].len(), art::TAIL_W, "{kind:?}");
            }
            for head in rig.heads {
                assert_eq!(head[0].len(), w - rig.head_x, "{kind:?}");
            }
        }
    }

    /// Odd option values, sizes and steps must not panic (the registry test covers the defaults).
    #[test]
    fn extreme_options_survive() {
        let cases = [
            "speed = 50.0\nday = 1\nzoom = 99\nwater = false\nfish = 10.0\nkind = 'random'",
            "speed = 0.0\nday = 999999999\nzoom = -4\nrays = false\nfish = -3.0\nkind = 'nope'",
            "kind = 'whale-shark'\nfish = 0",
            "kind = 'hammerhead'\nzoom = 6",
            "kind = 'tiger'\nzoom = 0\nwater = false",
        ];
        let themes = [
            Theme::classic(),
            Theme::builtin("mono").unwrap(),
            Theme::builtin("dracula").unwrap(),
            Theme::terminal(&crate::theme::TermColors::default()),
        ];
        for case in cases {
            let opts = opts(case);
            for theme in &themes {
                let c = ctx(theme, &opts);
                let mut s = (INFO.make)(&c);
                for &(w, h) in &[(1u16, 1u16), (2, 3), (3, 1), (1, 40), (500, 150), (80, 24)] {
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

    /// A few short days: the shark goes through its moods, catches a fish or
    /// two, and stays in the water.
    #[test]
    fn a_long_run_visits_the_moods() {
        use hunter::Mood;
        let theme = Theme::classic();
        let opts = opts("day = 60");
        let c = ctx(&theme, &opts);
        let mut s = SharkScene::new(&c);
        s.resize(&c, 100, 30);
        let mut cv = Canvas::new(100, 30, Color::Reset);
        let mut seen = Vec::new();
        let mut eaten = 0;
        for i in 0..30 * 60 * 6 {
            let alive = s.sea.life.fish.iter().filter(|f| f.alive).count();
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let now = s.sea.life.fish.iter().filter(|f| f.alive).count();
            eaten += alive.saturating_sub(now);
            let sh = &s.sea.shark;
            let bed = s.sea.bed(sh.x) as f32;
            assert!(sh.y > sea::SURF as f32 && sh.y < bed, "shark at {}", sh.y);
            if !seen.contains(&sh.mood) {
                seen.push(sh.mood);
            }
            if i % 15 == 0 {
                cv.clear();
                s.draw(&c, &mut cv);
            }
        }
        for mood in [Mood::Cruise, Mood::Stalk, Mood::Strike, Mood::Rest] {
            assert!(seen.contains(&mood), "never {mood:?}: {seen:?}");
        }
        assert!(seen.len() >= 5, "{seen:?}");
        assert!(eaten > 0, "nothing caught");
    }

    /// A whale shark filter feeds; it never stalks or bites.
    #[test]
    fn whale_sharks_filter_feed() {
        use hunter::Mood;
        let theme = Theme::classic();
        let opts = opts("day = 60\nkind = 'whale-shark'");
        let c = ctx(&theme, &opts);
        let mut s = SharkScene::new(&c);
        s.resize(&c, 100, 30);
        let mut fed = false;
        for _ in 0..30 * 60 * 4 {
            s.tick(&c, 1.0 / 30.0, 1.0 / 30.0);
            let mood = s.sea.shark.mood;
            assert!(!matches!(mood, Mood::Stalk | Mood::Strike), "{mood:?}");
            fed |= mood == Mood::Feed;
        }
        assert!(fed);
    }
}
