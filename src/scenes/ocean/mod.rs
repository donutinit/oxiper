//! ocean: tanim's fish tank, opened up to the sea and its food chain.
//!
//! Adapted from the `aquarium` animation of tanim
//! (<https://github.com/SantiagoSchez/tanim>), which is under the MIT License:
//!
//! ```text
//! MIT License
//!
//! Copyright (c) 2026 Santiago Sánchez
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//! ```
//!
//! The tank is tanim's, in half-block pixels: light rays through the water, a
//! rippling surface, sandy dunes with rocks, pebbles, shells and a castle,
//! swaying weeds, the crab, bubbles, and its fish and shark as drawn there.
//! Here a strip of sky sits over the water, and the fish no longer just
//! cross: they eat and get eaten.
//!
//! - `art.rs`: every drawing, tanim's and the newcomers' (sardines,
//!   barracudas, orcas, dolphins, mantas).
//! - `tank.rs`: the water, the sky, the floor and what grows and lives on
//!   it, the bubbles, and the glints a catch leaves.
//! - `life.rs`: the food chain. Sardines and tetras school and scatter;
//!   groupers and barracudas snap up the smallest; sharks stalk the
//!   schools; dolphin pods herd fish and leap; orca pods round a school
//!   into a bait ball and take turns through it, and rush any shark they
//!   find; mantas glide past feeding on plankton and loop over. The eaten
//!   are replaced by fish swimming in from the sides.
//! - `sea.rs`: the sea as a whole: stepping, the visits and the drawing.
//! - `muertos.rs`: Día de Muertos, Janitzio's night on the water. Night
//!   falls with a thin moon and stars; veladoras in colored glass float on
//!   rafts of cempasúchil, haloed and glinting in the water, among drifting
//!   petals and candles far across the lake; petals sink and rest on the
//!   sand among marigolds, terciopelo and nube; the castle glows from its
//!   windows and door, with candles in its crenels and papel picado between
//!   its towers. Alebrijes join the schools and swim on their own, sharks
//!   and orcas wear calavera paint, and there is more of it all as the
//!   holiday comes to November 2. An ordinary day's sea is left as it was,
//!   down to its random numbers.
//! - `navidad.rs`, with `gala.rs` (what the holidays of the boats share),
//!   `trajinera.rs` (the boats and the buoys) and `cohetes.rs` (fireworks):
//!   Navidad, the Guadalupe–Reyes on the water of Xochimilco. Trajineras,
//!   flat painted boats with a woman's name across their arch (Lupita,
//!   Esperanza), drift across the surface in a line hung with foquitos that
//!   blink in chase patterns, a trajinero poling at the stern and, as the
//!   posadas climb, a piñata de siete picos swinging off each bow. Buoys
//!   carry the Estrella de Belén. A day and night go round: by night the
//!   stars come out, the lights glow and shine again in the water. The
//!   castle strings foquitos between its towers, nochebuenas grow on the
//!   sand, esferas dropped from the boats sink and settle among them, and
//!   fish in rojo nochebuena and verde pino join the schools and cross on
//!   their own. Sharks, orcas and dolphins wear a gorro navideño, and on
//!   Año Nuevo fireworks burst over the water and in it. There is more of
//!   it all as the posadas come to Nochebuena; an ordinary day's sea is
//!   left as it was, down to its random numbers.
//! - `patrias.rs`, on the same footing: the Fiestas Patrias, the mes patrio
//!   on the water of Xochimilco. The trajineras (Josefa, Leona, Victoria)
//!   are painted in verde, blanco y rojo and fly the flag from a pole at
//!   either end, string papel picado between the poles and the arch and
//!   wear tricolor lights; buoys carry a flag on their mast; the castle
//!   flies one from its tower and strings papel picado between its towers;
//!   nopales with their red tunas grow on the sand and pieces of papel
//!   picado dropped from the boats sink among them; fish in verde, blanco y
//!   rojo, band by band like the flag, join the schools and cross on their
//!   own; sharks, orcas and dolphins wear a banda tricolor. By night
//!   tricolor fireworks burst over the water and in it, more of them, and
//!   volleys, as the month climbs to the Grito. An ordinary day's sea is
//!   left as it was.
//!
//! tanim runs its tank at 20 ticks a second and moves things by pixels per
//! tick; the sea steps at that rate on an accumulator scaled by `speed`, so
//! its fish keep tanim's pace. When the big animals come by follows the wall
//! clock.

mod art;
mod cohetes;
mod gala;
mod life;
mod muertos;
mod navidad;
mod patrias;
mod sea;
mod tank;
mod trajinera;

use crate::color::Color;
use crate::fiesta::{Fiesta, Kind};
use crate::render::Canvas;
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use sea::Sea;

pub const INFO: SceneInfo = SceneInfo {
    name: "ocean",
    aliases: &["sea"],
    about: "tanim's fish tank opened to the sea: schools, sharks, orcas, dolphins and mantas",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "How fast everything swims (the visits keep real time)",
        },
        Opt {
            key: "fish",
            default: Val::F(1.0),
            help: "How many fish, 1 being the usual; 0 for none (0-5)",
        },
        Opt {
            key: "sharks",
            default: Val::B(true),
            help: "Sharks come by to hunt the schools",
        },
        Opt {
            key: "orcas",
            default: Val::B(true),
            help: "Pods of orcas herd schools into bait balls and take sharks",
        },
        Opt {
            key: "dolphins",
            default: Val::B(true),
            help: "Pods of dolphins chase fish and leap from the water",
        },
        Opt {
            key: "mantas",
            default: Val::B(true),
            help: "Mantas glide by feeding, now and then looping over",
        },
        Opt {
            key: "visits",
            default: Val::F(1.0),
            help: "How often the big animals come by, 1 being the usual (0 = never, up to 10)",
        },
        Opt {
            key: "surface",
            default: Val::B(true),
            help: "A strip of sky over the water, for the dolphins to leap into",
        },
        Opt {
            key: "water",
            default: Val::B(true),
            help: "Paint the water and the sky; off leaves them see-through",
        },
    ],
    make: |ctx| Box::new(OceanScene::new(ctx)),
};

/// Seconds per tick: tanim's tank runs at 20 a second.
const STEP: f64 = 1.0 / 20.0;
/// The most ticks one frame may catch up on; beyond it time is dropped.
const MAX_STEPS: u32 = 40;
/// How much a theme tints the water and the sky.
const TINT: f32 = 0.3;

/// Dice on top of fastrand.
trait Dice {
    /// An integer in `[lo, hi)`, or `lo` when the range is empty.
    fn range(&mut self, lo: i32, hi: i32) -> i32;
    fn rangef(&mut self, lo: f32, hi: f32) -> f32;
    fn chance(&mut self, p: f32) -> bool;
    /// -1 or 1.
    fn sign(&mut self) -> f32;
    fn pick<T: Copy>(&mut self, s: &[T]) -> T;
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

    fn pick<T: Copy>(&mut self, s: &[T]) -> T {
        s[self.usize(..s.len())]
    }
}

type Rng = fastrand::Rng;

/// What the options choose.
#[derive(Clone, Copy)]
struct Settings {
    /// Fish density, 1 being the usual.
    fish: f32,
    sharks: bool,
    orcas: bool,
    dolphins: bool,
    mantas: bool,
    /// How often the big animals come, 1 being the usual.
    visits: f32,
    surface: bool,
    water: bool,
}

/// The colors a theme changes: the water, its light and the sky.
#[derive(Clone, Copy)]
struct Pal {
    /// The water under the surface, halfway down and at the bottom.
    water: [Color; 3],
    ray: Color,
    surface: Color,
    /// The sky at the top and at the horizon.
    sky: [Color; 2],
    /// Día de Muertos: the night sky, from the top through dusk to the
    /// last glow at the horizon, and the colors of papel picado and
    /// veladora glass.
    night: [Color; 3],
    papel: [Color; 5],
    /// The holidays that put boats on the water: the evening sky, the water
    /// and its light at night (what the day goes over to), and the paint.
    eve: [Color; 3],
    deep: [Color; 3],
    moon: Color,
    glow: Color,
    paint: gala::Paint,
}

const WATER_TOP: Color = art::rgb(26, 104, 160);
const WATER_MID: Color = art::rgb(10, 54, 104);
const WATER_BOT: Color = art::rgb(4, 20, 52);
const RAY: Color = art::rgb(110, 190, 225);
const SURFACE: Color = art::rgb(160, 220, 245);
const SKY_TOP: Color = art::rgb(112, 170, 222);
const SKY_LOW: Color = art::rgb(198, 226, 244);

// Janitzio's night: twilight water under the moon, candlelight on the
// ripples, and a sky going from noche to the last of the sunset.
const NIGHT_TOP: Color = art::rgb(22, 62, 116);
const NIGHT_MID: Color = art::rgb(10, 32, 78);
const NIGHT_BOT: Color = art::rgb(5, 12, 38);
const MOON: Color = art::rgb(112, 140, 206);
const CANDLELIT: Color = art::rgb(240, 150, 110);
const NIGHT_SKY: [Color; 3] = [
    art::rgb(18, 14, 40),
    art::rgb(44, 22, 78),
    art::rgb(120, 46, 110),
];
// The evening of the boats: an indigo sky with a last warm glow at the
// horizon over water gone dark, the surface catching the moon.
const EVE_SKY: [Color; 3] = [
    art::rgb(8, 12, 42),
    art::rgb(24, 30, 84),
    art::rgb(112, 70, 110),
];
const EVE_GLOW: Color = art::rgb(150, 166, 220);
const PAPEL: [Color; 5] = [
    art::rgb(228, 0, 124),
    art::rgb(255, 140, 0),
    art::rgb(140, 60, 200),
    art::rgb(0, 170, 184),
    art::rgb(255, 210, 0),
];

impl Pal {
    fn new(ctx: &Ctx) -> Pal {
        let muertos = ctx.fiesta.is(Kind::Muertos);
        let (water, ray, surface) = if muertos {
            ([NIGHT_TOP, NIGHT_MID, NIGHT_BOT], MOON, CANDLELIT)
        } else {
            ([WATER_TOP, WATER_MID, WATER_BOT], RAY, SURFACE)
        };
        let papel = [0, 1, 2, 3, 4].map(|i| ctx.pick(i, PAPEL[i]));
        let paint = gala::Paint::new(ctx);
        let native = Pal {
            water,
            ray,
            surface,
            sky: [SKY_TOP, SKY_LOW],
            night: NIGHT_SKY,
            papel,
            eve: EVE_SKY,
            deep: [NIGHT_TOP, NIGHT_MID, NIGHT_BOT],
            moon: MOON,
            glow: EVE_GLOW,
            paint,
        };
        if ctx.theme.native {
            return native;
        }
        let g = &ctx.theme.grad;
        let tint = |c: Color, k: f32| c.lerp(g.at(k), TINT);
        Pal {
            water: [
                tint(water[0], 0.75),
                tint(water[1], 0.45),
                tint(water[2], 0.15),
            ],
            ray: tint(ray, 0.95),
            surface: tint(surface, 1.0),
            sky: [tint(SKY_TOP, 0.8), tint(SKY_LOW, 1.0)],
            night: [
                tint(NIGHT_SKY[0], 0.1),
                tint(NIGHT_SKY[1], 0.5),
                tint(NIGHT_SKY[2], 0.9),
            ],
            papel,
            eve: [
                tint(EVE_SKY[0], 0.1),
                tint(EVE_SKY[1], 0.5),
                tint(EVE_SKY[2], 0.9),
            ],
            deep: [
                tint(NIGHT_TOP, 0.75),
                tint(NIGHT_MID, 0.45),
                tint(NIGHT_BOT, 0.15),
            ],
            moon: tint(MOON, 0.95),
            glow: tint(EVE_GLOW, 1.0),
            paint,
        }
    }
}

/// The scene: the sea, stepped at tanim's pace.
struct OceanScene {
    sea: Sea,
    rng: Rng,
    set: Settings,
    /// The holiday the sea is dressed for, the seed its dress is drawn
    /// from, and the screen's size in cells.
    fiesta: Fiesta,
    seed: u64,
    size: (u16, u16),
    speed: f64,
    /// Scene seconds not yet spent on ticks.
    acc: f64,
    /// Whether the picture needs painting again.
    dirty: bool,
}

/// A number option clamped to its range; non-finite values get the default.
fn num(v: f32, default: f32, lo: f32, hi: f32) -> f32 {
    if v.is_finite() {
        v.clamp(lo, hi)
    } else {
        default
    }
}

impl OceanScene {
    fn new(ctx: &Ctx) -> OceanScene {
        let o = ctx.opts;
        let set = Settings {
            fish: num(o.f("fish"), 1.0, 0.0, 5.0),
            sharks: o.b("sharks"),
            orcas: o.b("orcas"),
            dolphins: o.b("dolphins"),
            mantas: o.b("mantas"),
            visits: num(o.f("visits"), 1.0, 0.0, 10.0),
            surface: o.b("surface"),
            water: o.b("water"),
        };
        let mut rng = ctx.rng();
        let sea = Sea::new(80, 24, &mut rng, Pal::new(ctx), set, ctx.fiesta, ctx.seed);
        OceanScene {
            sea,
            rng,
            set,
            fiesta: ctx.fiesta,
            seed: ctx.seed,
            size: (80, 24),
            speed: num(o.f("speed"), 1.0, 0.0, 20.0) as f64,
            acc: 0.0,
            dirty: true,
        }
    }
}

impl OceanScene {
    /// A new sea for the screen and the day.
    fn fresh(&mut self, pal: Pal) -> Sea {
        let (w, h) = self.size;
        Sea::new(w, h, &mut self.rng, pal, self.set, self.fiesta, self.seed)
    }
}

impl Scene for OceanScene {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        // The tank is built for the screen, so a resize fills a fresh one.
        self.size = (w, h);
        self.sea = self.fresh(self.sea.pal);
        self.dirty = true;
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let pal = Pal::new(ctx);
        if ctx.fiesta != self.fiesta {
            // A new day of the holiday, or its end: a sea dressed for it.
            self.fiesta = ctx.fiesta;
            self.sea = self.fresh(pal);
        } else {
            self.sea.pal = pal;
        }
        self.dirty = true;
    }

    fn update(&mut self, ctx: &Ctx, dt: f32) {
        self.tick(ctx, dt, dt);
    }

    fn tick(&mut self, _ctx: &Ctx, dt: f32, real: f32) {
        // The visits run on the wall clock, speed or not.
        if real.is_finite() && real > 0.0 {
            self.dirty |= self.sea.clocks(real.min(3600.0), &mut self.rng);
        }
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.acc += dt as f64 * self.speed;
        let n = ((self.acc / STEP) as u32).min(MAX_STEPS);
        self.acc = if n == MAX_STEPS {
            0.0
        } else {
            self.acc - n as f64 * STEP
        };
        for _ in 0..n {
            self.sea.step(&mut self.rng);
        }
        self.dirty |= n > 0;
    }

    fn next_change(&self) -> f32 {
        if self.speed <= 0.0 {
            return 1.0;
        }
        ((STEP - self.acc).max(0.0) / self.speed) as f32
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        if self.dirty {
            let bg = matches!(ctx.bg, Color::Rgb(..)).then_some(ctx.bg);
            self.sea.render(bg);
            self.dirty = false;
        }
        self.sea.scr.blit(c);
    }
}

#[cfg(test)]
mod tests {
    use super::art::{self, Role, Species};
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

    fn scene(table: &str, w: u16, h: u16, f: impl FnOnce(&Ctx, &mut OceanScene)) {
        let theme = Theme::classic();
        let opts = opts(table);
        let c = ctx(&theme, &opts);
        let mut s = OceanScene::new(&c);
        s.resize(&c, w, h);
        f(&c, &mut s);
    }

    /// Runs `n` ticks of real time at tanim's pace.
    fn run(c: &Ctx, s: &mut OceanScene, n: u32, mut each: impl FnMut(&OceanScene)) {
        for _ in 0..n {
            s.tick(c, STEP as f32, STEP as f32);
            each(s);
        }
    }

    const ALL: [&Species; 19] = [
        &art::TETRA,
        &art::CLOWN,
        &art::ANGEL,
        &art::PUFFER,
        &art::TANG,
        &art::GOLDFISH,
        &art::GROUPER,
        &art::SHARK,
        &art::SARDINE,
        &art::BARRACUDA,
        &art::ORCA,
        &art::DOLPHIN,
        &art::MANTA,
        &art::ALEBRIJITO,
        &art::ALEBRIJE,
        &art::NOCHEBUENITA,
        &art::NOCHEBUENA,
        &art::BANDERITA,
        &art::BANDERA,
    ];

    /// Every drawing is a rectangle, all of an animal's frames are the same
    /// size, and every letter has a color.
    #[test]
    fn sprites_are_rectangular() {
        let fish = ALL.iter().flat_map(|s| s.frames.iter().copied());
        for s in fish.chain(art::CRAB).chain([art::CASTLE, art::PUFFED]) {
            assert!(
                s.iter().all(|r| r.len() == s[0].len()),
                "ragged sprite: {s:?}"
            );
        }
        for sp in ALL {
            let size = art::size(sp.frames[0]);
            for f in sp.frames {
                assert_eq!(art::size(f), size, "{}", sp.name);
                for ch in f.iter().flat_map(|r| r.bytes()) {
                    assert!(b".BDLSFwk".contains(&ch), "{}: {}", sp.name, ch as char);
                }
            }
            assert!(
                sp.cruise.0 <= sp.cruise.1 && sp.cruise.1 < sp.burst,
                "{}",
                sp.name
            );
            // Calavera paint fits its animal exactly.
            if let Some(face) = art::face_paint(sp) {
                assert_eq!(art::size(face), size, "{}", sp.name);
                for ch in face.iter().flat_map(|r| r.bytes()) {
                    assert!(b".hoptym".contains(&ch), "{}: {}", sp.name, ch as char);
                }
            }
            // So do the holidays' hats and bandas, a banda lying on the
            // animal's body in every frame.
            for kind in [Kind::Navidad, Kind::Patrias] {
                if let Some(dress) = art::dress(sp, kind) {
                    assert_eq!(art::size(dress), size, "{}", sp.name);
                    for ch in dress.iter().flat_map(|r| r.bytes()) {
                        assert!(b".RWG".contains(&ch), "{}: {}", sp.name, ch as char);
                    }
                    if kind == Kind::Patrias {
                        for f in sp.frames {
                            for (y, row) in dress.iter().enumerate() {
                                for (x, ch) in row.bytes().enumerate() {
                                    let body = f[y].as_bytes()[x];
                                    assert!(
                                        ch == b'.' || body != b'.',
                                        "{}: banda off its body at {x},{y}",
                                        sp.name
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Odd option values, sizes and steps must not panic.
    #[test]
    fn variants_survive() {
        let cases = [
            "",
            "speed = 20.0\nfish = 5.0\nvisits = 10.0",
            "speed = nan\nfish = -3.0\nvisits = inf",
            "fish = 0\nvisits = 0",
            "orcas = false\ndolphins = false\nsharks = false\nmantas = false",
            "water = false\nsurface = false\nvisits = 10.0",
        ];
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        for case in cases {
            let opts = opts(case);
            for theme in &themes {
                for bg in [Color::Reset, Color::Rgb(16, 16, 24)] {
                    let c = Ctx {
                        bg,
                        ..ctx(theme, &opts)
                    };
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (2, 3), (7, 2), (80, 24), (300, 90), (40, 12)] {
                        s.resize(&c, w, h);
                        s.retheme(&c);
                        let mut cv = Canvas::new(w, h, Color::Reset);
                        for dt in [0.05, f32::NAN, -1.0, f32::INFINITY, 5.0, 1e9, 0.05] {
                            s.tick(&c, dt, dt);
                            s.next_change();
                            cv.clear();
                            s.draw(&c, &mut cv);
                        }
                        for _ in 0..200 {
                            s.tick(&c, 0.05, 0.5);
                        }
                        cv.clear();
                        s.draw(&c, &mut cv);
                    }
                }
            }
        }
    }

    fn count(s: &OceanScene, role: Role) -> usize {
        s.sea
            .swim
            .iter()
            .filter(|f| f.alive && f.sp.role == role)
            .count()
    }

    /// Over a few minutes fish get eaten, and new ones swim in so the sea
    /// never empties.
    #[test]
    fn the_food_chain_runs_and_refills() {
        scene("visits = 3.0", 160, 50, |c, s| {
            let start = count(s, Role::School) + count(s, Role::Reef);
            assert!(start > 20, "{start}");
            let mut fewest = usize::MAX;
            run(c, s, 20 * 240, |s| {
                fewest = fewest.min(count(s, Role::School) + count(s, Role::Reef));
            });
            let end = count(s, Role::School) + count(s, Role::Reef);
            assert!(s.sea.eaten >= 5, "only {} eaten", s.sea.eaten);
            assert!(
                fewest * 5 >= start,
                "the sea nearly emptied: {start} -> {fewest}"
            );
            assert!(end * 2 >= start, "the sea emptied: {start} -> {end}");
        });
    }

    /// Orcas, dolphins and mantas come by when they may, and never when not.
    #[test]
    fn visitors_follow_their_options() {
        let roles = [
            ("orcas", Role::Orca),
            ("dolphins", Role::Dolphin),
            ("mantas", Role::Manta),
        ];
        for (key, role) in roles {
            for on in [true, false] {
                let table = format!("visits = 10.0\n{key} = {on}");
                scene(&table, 160, 50, |c, s| {
                    let mut seen = false;
                    run(c, s, 20 * 300, |s| seen |= count(s, role) > 0);
                    assert_eq!(seen, on, "{key} = {on}");
                });
            }
        }
    }

    /// An orca pod that finds a shark runs it down.
    #[test]
    fn orcas_take_sharks() {
        scene("visits = 0", 200, 60, |c, s| {
            let mut rng = Rng::with_seed(9);
            s.sea.send_pod(&art::ORCA, 3, &mut rng);
            for o in s.sea.swim.iter_mut().filter(|f| f.sp.role == Role::Orca) {
                o.x = o.x.clamp(10.0, 190.0);
            }
            let mut shark = s.sea.swimmer(&art::SHARK, (100.0, 60.0), 1.0, &mut rng);
            shark.stay = 100_000;
            s.sea.swim.push(shark);
            run(c, s, 20 * 120, |_| {});
            assert_eq!(s.sea.sharks_taken, 1);
        });
    }

    fn holiday<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            fiesta,
            ..ctx(theme, opts)
        }
    }

    fn alebrijes(s: &OceanScene) -> usize {
        s.sea
            .swim
            .iter()
            .filter(|f| f.alive && [art::ALEBRIJE.name, art::ALEBRIJITO.name].contains(&f.sp.name))
            .count()
    }

    /// Día de Muertos dresses the sea from its first day to its height:
    /// night, veladoras and petals on the water, flowers on the floor and
    /// alebrijes, more of it all on November 2. An ordinary day has none.
    #[test]
    fn muertos_dresses_the_sea() {
        let theme = Theme::classic();
        let opts = opts("visits = 0");
        let mut counts = Vec::new();
        for f in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2), Fiesta::NONE] {
            let c = holiday(&theme, &opts, f);
            let mut s = OceanScene::new(&c);
            s.resize(&c, 160, 48);
            s.retheme(&c);
            assert_eq!(s.sea.fest.on, f.on());
            let mut seen = alebrijes(&s);
            run(&c, &mut s, 20 * 120, |s| seen = seen.max(alebrijes(s)));
            let mut cv = Canvas::new(160, 48, Color::Reset);
            s.draw(&c, &mut cv);
            // The sky at the top: night, or the day's blue.
            let night = s.sea.scr.get(0, 0) == s.sea.pal.night[0];
            let (candles, petals, blooms) = s.sea.fest.counts();
            if f.on() {
                assert!(night && seen > 0, "{f:?}: {seen} alebrijes");
                assert!(candles > 0 && petals > 0 && blooms > 0, "{f:?}");
            } else {
                assert!(!night && seen == 0 && candles + petals + blooms == 0);
            }
            counts.push((candles, blooms));
        }
        assert!(
            counts[1].0 > counts[0].0 && counts[1].1 > counts[0].1,
            "{counts:?}"
        );
    }

    /// The holiday comes and goes with `retheme`, as it does at midnight.
    #[test]
    fn retheme_switches_the_holiday() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts);
        let on = holiday(&theme, &opts, Fiesta::of_day(11, 1));
        let mut s = OceanScene::new(&off);
        s.resize(&off, 120, 36);
        s.retheme(&off);
        assert!(!s.sea.fest.on);
        s.retheme(&on);
        assert!(s.sea.fest.on && s.sea.fest.counts().0 > 0);
        run(&on, &mut s, 200, |_| {});
        s.retheme(&off);
        assert!(!s.sea.fest.on && s.sea.fest.counts() == (0, 0, 0));
        run(&off, &mut s, 20 * 60, |s| assert_eq!(alebrijes(s), 0));
    }

    /// The holiday survives odd sizes, huge steps and another theme.
    #[test]
    fn muertos_survives() {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        for case in [
            "",
            "visits = 10.0\nfish = 5.0",
            "water = false\nsurface = false",
        ] {
            let opts = opts(case);
            for theme in &themes {
                for f in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
                    let c = holiday(theme, &opts, f);
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[(1u16, 1u16), (3, 2), (9, 4), (80, 24), (300, 90), (40, 12)] {
                        s.resize(&c, w, h);
                        s.retheme(&c);
                        let mut cv = Canvas::new(w, h, Color::Reset);
                        for dt in [0.05, 5.0, f32::NAN, 1e9, 0.05] {
                            s.tick(&c, dt, dt);
                            cv.clear();
                            s.draw(&c, &mut cv);
                        }
                        for _ in 0..100 {
                            s.tick(&c, 0.5, 0.5);
                        }
                        cv.clear();
                        s.draw(&c, &mut cv);
                    }
                }
            }
        }
    }

    /// How many of Navidad's fish are swimming.
    fn nochebuenas(s: &OceanScene) -> usize {
        s.sea
            .swim
            .iter()
            .filter(|f| f.alive && navidad::is_fish(f.sp))
            .count()
    }

    /// How many of the Fiestas Patrias' fish are swimming.
    fn banderas(s: &OceanScene) -> usize {
        s.sea
            .swim
            .iter()
            .filter(|f| f.alive && patrias::is_fish(f.sp))
            .count()
    }

    /// The sea as pixels, to compare two seas.
    fn pixels(s: &OceanScene) -> Vec<Color> {
        let scr = &s.sea.scr;
        (0..scr.ph())
            .flat_map(|y| (0..scr.pw()).map(move |x| (x, y)))
            .map(|(x, y)| scr.get(x, y))
            .collect()
    }

    /// Whether any pixel of the sky is painted `c`.
    fn sky_has(s: &OceanScene, c: Color) -> bool {
        (0..s.sea.surf).any(|y| (0..s.sea.w).any(|x| s.sea.scr.get(x, y) == c))
    }

    /// Navidad dresses the sea from the first of the Guadalupe–Reyes to
    /// Nochebuena: trajineras with their names and lights, the Estrella on
    /// its buoy, nochebuenas on the floor, esferas sinking and fish in rojo
    /// and verde, and a night that comes and goes. An ordinary day has none.
    #[test]
    fn navidad_dresses_the_sea() {
        let theme = Theme::classic();
        let opts = opts("visits = 0");
        let mut counts = Vec::new();
        for f in [Fiesta::of_day(12, 12), Fiesta::of_day(12, 24), Fiesta::NONE] {
            let c = holiday(&theme, &opts, f);
            let mut s = OceanScene::new(&c);
            s.resize(&c, 200, 60);
            s.retheme(&c);
            assert_eq!(s.sea.gala.on, f.on());
            let (mut fish, mut esferas) = (nochebuenas(&s), 0);
            let (mut darkest, mut brightest) = (0.0f32, 1.0f32);
            run(&c, &mut s, 20 * 240, |s| {
                fish = fish.max(nochebuenas(s));
                esferas = esferas.max(s.sea.gala.floor_counts().1);
                darkest = darkest.max(s.sea.gala.dusk);
                brightest = brightest.min(s.sea.gala.dusk);
            });
            let (boats, buoys) = s.sea.gala.counts();
            let (plants, _) = s.sea.gala.floor_counts();
            let pinatas = s.sea.gala.boats.iter().filter(|b| b.pinata).count();
            if f.on() {
                assert!(boats > 0 && buoys > 0 && plants > 0, "{f:?}");
                assert!(
                    fish > 0 && esferas > 0,
                    "{f:?}: {fish} fish, {esferas} esferas"
                );
                assert!(darkest > 0.99 && brightest < 0.01, "{darkest} {brightest}");
                for name in s.sea.gala.names() {
                    assert!(navidad::NAMES.contains(&name));
                }
                // The boats are painted on the sky by day.
                let mut cv = Canvas::new(200, 60, Color::Reset);
                s.sea.gala.dusk = 0.0;
                s.dirty = true;
                s.draw(&c, &mut cv);
                let paint = s.sea.pal.paint;
                assert!(
                    s.sea
                        .gala
                        .boats
                        .iter()
                        .any(|b| sky_has(&s, paint.boats[b.panel])),
                    "{f:?}: no boats painted"
                );
            } else {
                assert!(boats + buoys + plants + esferas == 0);
                assert!(fish == 0 && darkest == 0.0, "{fish} {darkest}");
            }
            counts.push((boats, pinatas));
        }
        // More of it as the posadas climb, the piñatas most of all.
        assert!(counts[1].0 > counts[0].0, "{counts:?}");
        assert!(counts[0].1 == 0 && counts[1].1 == counts[1].0, "{counts:?}");
    }

    /// The first of the posadas hangs the first piñata.
    #[test]
    fn the_posadas_hang_a_pinata() {
        let theme = Theme::classic();
        let opts = opts("");
        let c = holiday(&theme, &opts, Fiesta::of_day(12, 16));
        let mut s = OceanScene::new(&c);
        s.resize(&c, 200, 60);
        s.retheme(&c);
        assert!(s.sea.gala.posadas && s.sea.gala.boats[0].pinata);
        let c = holiday(&theme, &opts, Fiesta::of_day(12, 13));
        s.retheme(&c);
        assert!(!s.sea.gala.posadas && s.sea.gala.boats.iter().all(|b| !b.pinata));
    }

    /// Every letter of every name is drawn, and the boats fit the sky.
    #[test]
    fn the_names_are_drawn() {
        for c in b'A'..=b'Z' {
            assert_ne!(trajinera::glyph(c), [0; 5], "{}", c as char);
        }
        for name in navidad::NAMES.iter().chain(&patrias::NAMES) {
            assert!(name.bytes().all(|c| c.is_ascii_uppercase()), "{name}");
        }
        for surf in 0..30 {
            if let Some(sh) = trajinera::Shape::of(surf, 200) {
                assert!(sh.rows() < surf, "{surf}: {sh:?}");
                for name in navidad::NAMES.iter().chain(&patrias::NAMES) {
                    assert!(sh.width(name) > 2 * sh.gap);
                }
            }
        }
    }

    /// Navidad comes and goes with `retheme`, as it does at midnight.
    #[test]
    fn retheme_switches_navidad() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts);
        let on = holiday(&theme, &opts, Fiesta::of_day(12, 20));
        let mut s = OceanScene::new(&off);
        s.resize(&off, 160, 50);
        s.retheme(&off);
        assert!(!s.sea.gala.on);
        s.retheme(&on);
        assert!(s.sea.gala.on && s.sea.gala.counts().0 > 0);
        run(&on, &mut s, 200, |_| {});
        s.retheme(&off);
        assert!(!s.sea.gala.on && s.sea.gala.counts() == (0, 0));
        run(&off, &mut s, 20 * 60, |s| {
            assert_eq!(nochebuenas(s), 0);
            assert_eq!(s.sea.gala.dusk, 0.0);
        });
    }

    /// A holiday the sea has no dress for leaves it as on an ordinary day.
    #[test]
    fn an_ordinary_day_is_untouched() {
        let theme = Theme::classic();
        let opts = opts("");
        let mut seas = Vec::new();
        // Nothing, a day that is nothing, and holidays it knows nothing of.
        for f in [
            Fiesta::NONE,
            Fiesta::of_day(6, 15),
            Fiesta::of_day(2, 2),
            Fiesta::of_day(5, 3),
        ] {
            let c = holiday(&theme, &opts, f);
            let mut s = OceanScene::new(&c);
            s.resize(&c, 160, 50);
            s.retheme(&c);
            run(&c, &mut s, 20 * 30, |_| {});
            let mut cv = Canvas::new(160, 50, Color::Reset);
            s.draw(&c, &mut cv);
            seas.push(pixels(&s));
        }
        assert!(seas.windows(2).all(|p| p[0] == p[1]));
    }

    /// The days of a holiday survive odd sizes, huge steps and another theme.
    fn survives(days: &[Fiesta]) {
        let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
        for case in [
            "",
            "visits = 10.0\nfish = 5.0",
            "water = false\nsurface = false",
        ] {
            let opts = opts(case);
            for theme in &themes {
                for &f in days {
                    let c = holiday(theme, &opts, f);
                    let mut s = (INFO.make)(&c);
                    for &(w, h) in &[
                        (1u16, 1u16),
                        (3, 2),
                        (9, 4),
                        (80, 24),
                        (300, 90),
                        (40, 12),
                        (500, 150),
                        (60, 20),
                        (120, 36),
                    ] {
                        s.resize(&c, w, h);
                        s.retheme(&c);
                        let mut cv = Canvas::new(w, h, Color::Reset);
                        for dt in [0.05, 5.0, f32::NAN, 1e9, 0.05] {
                            s.tick(&c, dt, dt);
                            cv.clear();
                            s.draw(&c, &mut cv);
                        }
                        for _ in 0..100 {
                            s.tick(&c, 0.5, 0.5);
                        }
                        cv.clear();
                        s.draw(&c, &mut cv);
                    }
                }
            }
        }
    }

    #[test]
    fn navidad_survives() {
        survives(&[
            Fiesta::of_day(12, 12),
            Fiesta::of_day(12, 16),
            Fiesta::of_day(12, 24),
            Fiesta::of_day(12, 31),
            Fiesta::of_day(1, 6),
        ]);
    }

    #[test]
    fn the_patrias_survive() {
        survives(&[
            Fiesta::of_day(9, 1),
            Fiesta::of_day(9, 8),
            Fiesta::of_day(9, 15),
            Fiesta::of_day(9, 16),
        ]);
    }

    /// The Fiestas Patrias dress the sea from the first of September to the
    /// Grito: trajineras in the flag's colors with their flags, buoys with
    /// flags, nopales, papel picado sinking, fish in verde, blanco y rojo,
    /// tricolor fireworks at night, more of it all on the 15th.
    #[test]
    fn the_patrias_dress_the_sea() {
        let theme = Theme::classic();
        let opts = opts("visits = 0");
        let mut counts = Vec::new();
        for f in [Fiesta::of_day(9, 1), Fiesta::of_day(9, 15), Fiesta::NONE] {
            let c = holiday(&theme, &opts, f);
            let mut s = OceanScene::new(&c);
            s.resize(&c, 200, 60);
            s.retheme(&c);
            assert_eq!(s.sea.gala.on, f.on());
            assert_eq!(s.sea.gala.fireworks, f.on());
            let (mut fish, mut papeles, mut sparks) = (banderas(&s), 0, 0);
            let mut darkest = 0.0f32;
            run(&c, &mut s, 20 * 240, |s| {
                fish = fish.max(banderas(s));
                papeles = papeles.max(s.sea.gala.floor_counts().1);
                sparks = sparks.max(s.sea.gala.works.counts().1);
                darkest = darkest.max(s.sea.gala.dusk);
            });
            let (boats, buoys) = s.sea.gala.counts();
            let (plants, _) = s.sea.gala.floor_counts();
            if f.on() {
                assert!(boats > 0 && buoys > 0 && plants > 0, "{f:?}");
                assert!(fish > 0 && papeles > 0, "{f:?}: {fish} fish, {papeles}");
                assert!(sparks > 20 && darkest > 0.99, "{f:?}: {sparks} {darkest}");
                for name in s.sea.gala.names() {
                    assert!(patrias::NAMES.contains(&name));
                }
                // No piñatas and no nochebuenas: only the flag's colors paint
                // the boats, and the flags fly in the sky by day.
                assert!(s.sea.gala.boats.iter().all(|b| !b.pinata));
                assert_eq!(nochebuenas(&s), 0);
                let mut cv = Canvas::new(200, 60, Color::Reset);
                s.sea.gala.dusk = 0.0;
                s.dirty = true;
                s.draw(&c, &mut cv);
                let [verde, blanco, rojo] = s.sea.pal.paint.bandera;
                assert!(
                    [verde, blanco, rojo].iter().all(|&c| sky_has(&s, c)),
                    "{f:?}: the flag is missing from the sky"
                );
            } else {
                assert!(boats + buoys + plants + papeles == 0);
                assert!(
                    fish == 0 && sparks == 0 && darkest == 0.0,
                    "{fish} {sparks}"
                );
            }
            counts.push((boats, s.sea.gala.works_sent()));
        }
        // More boats and many more fireworks at the Grito.
        assert!(counts[1].0 > counts[0].0, "{counts:?}");
        assert!(counts[1].1 > counts[0].1 * 2, "{counts:?}");
    }

    /// The flags and the papel picado come with the month: one flag on the
    /// first of September, two and the papel picado before the Grito.
    #[test]
    fn the_month_hangs_more_flags() {
        let lights = |d| trajinera::Lights::of(Fiesta::of_day(9, d).level);
        let (early, late) = (lights(1), lights(15));
        assert!(early.flags == 1 && early.papel == 0, "{early:?}");
        assert!(late.flags == 2 && late.papel == 2, "{late:?}");
    }

    /// The Fiestas Patrias come and go with `retheme`, as at midnight.
    #[test]
    fn retheme_switches_the_patrias() {
        let theme = Theme::classic();
        let opts = opts("");
        let off = ctx(&theme, &opts);
        let on = holiday(&theme, &opts, Fiesta::of_day(9, 10));
        let mut s = OceanScene::new(&off);
        s.resize(&off, 160, 50);
        s.retheme(&off);
        assert!(!s.sea.gala.on);
        s.retheme(&on);
        assert!(s.sea.gala.on && s.sea.gala.kind == Kind::Patrias);
        assert!(s.sea.gala.counts().0 > 0 && s.sea.gala.fireworks);
        run(&on, &mut s, 200, |_| {});
        // From the Patrias straight into Navidad, whose boats carry piñatas.
        let navidad = holiday(&theme, &opts, Fiesta::of_day(12, 24));
        s.retheme(&navidad);
        assert!(s.sea.gala.kind == Kind::Navidad && !s.sea.gala.fireworks);
        assert!(s.sea.gala.boats.iter().all(|b| b.pinata));
        s.retheme(&off);
        assert!(!s.sea.gala.on && s.sea.gala.counts() == (0, 0));
        run(&off, &mut s, 20 * 60, |s| {
            assert_eq!(banderas(s) + nochebuenas(s), 0);
            assert_eq!(s.sea.gala.dusk, 0.0);
        });
    }

    /// Under another theme the Patrias take its colors.
    #[test]
    fn the_patrias_under_a_theme_use_its_colors() {
        let classic = Theme::classic();
        let theme = Theme::builtin("dracula").unwrap();
        let opts = opts("");
        let f = Fiesta::of_day(9, 15);
        let (a, b) = (holiday(&classic, &opts, f), holiday(&theme, &opts, f));
        let (mut ca, mut cb) = (OceanScene::new(&a), OceanScene::new(&b));
        let (pa, pb) = (ca.sea.pal.paint, cb.sea.pal.paint);
        ca.retheme(&a);
        cb.retheme(&b);
        assert_eq!(pb.blanco, theme.fg);
        assert_ne!(pa.bandera, pb.bandera);
        assert!(pb.boats.iter().all(|c| !pa.boats.contains(c)));
    }

    /// Año Nuevo sends fireworks up at night, in the sky and in the water;
    /// the other days of the Navidad have none.
    #[test]
    fn ano_nuevo_has_fireworks() {
        let theme = Theme::classic();
        let opts = opts("visits = 0");
        for (f, works) in [
            (Fiesta::of_day(12, 31), true),
            (Fiesta::of_day(1, 1), true),
            (Fiesta::of_day(12, 24), false),
            (Fiesta::of_day(1, 6), false),
        ] {
            let c = holiday(&theme, &opts, f);
            let mut s = OceanScene::new(&c);
            s.resize(&c, 200, 60);
            s.retheme(&c);
            let (mut rockets, mut sparks) = (0, 0);
            let mut by_day = 0;
            run(&c, &mut s, 20 * 120, |s| {
                let (r, k) = s.sea.gala.works.counts();
                rockets = rockets.max(r);
                sparks = sparks.max(k);
                by_day += (s.sea.gala.dusk < 0.7 && r + k > 0) as u32;
            });
            assert_eq!(s.sea.gala.fireworks, works, "{f:?}");
            if works {
                assert!(rockets > 0 && sparks > 20, "{f:?}: {rockets} {sparks}");
            } else {
                assert_eq!((rockets, sparks), (0, 0), "{f:?}");
            }
            // Only the last of the sparks outlives the evening.
            assert!(by_day < 80, "{by_day}");
        }
    }
}
