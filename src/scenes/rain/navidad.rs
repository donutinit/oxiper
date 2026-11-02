//! rain's Navidad: a string of foquitos (series lights) hangs along the
//! eaves, blinking in chase, alternating, twinkling and fading patterns, and
//! the drops falling past catch the colors of the bulbs; puddles on the
//! ground reflect them as they blink; a few farolitos burn along the street;
//! and every flash of lightning catches a posada walking by in the rain:
//! José with his staff and lantern, María on the burro, peregrinos and
//! little ones under umbrellas, each with a candle, a step at every flicker,
//! the candles still burning in the dark after the rest has gone.
//!
//! All of it draws from its own RNG, so ordinary days keep their numbers.

use super::art::{Glow, Sprite};
use crate::color::{Color, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Canvas};
use crate::scene::Ctx;

/// The bulbs of the series: rojo, verde, azul, amarillo, rosa, naranja.
const FOQUITOS: [Color; 6] = [
    rgb(0xff3b3b),
    rgb(0x3bd16f),
    rgb(0x3b7bff),
    rgb(0xffd23b),
    rgb(0xff5fb4),
    rgb(0xff8c1a),
];
const ROJO: Color = rgb(0xc8102e);
const PINO: Color = rgb(0x1f6f3a);
const DORADO: Color = rgb(0xf2b632);
const BLANCO: Color = rgb(0xf4f1ea);
const CANELA: Color = rgb(0x8b4a1e);
const CANA: Color = rgb(0xc9b26b);
const AZUL: Color = rgb(0x2a5caa);
/// The burro's coat, and the paper of a farolito (bolsa de estraza).
const BURRO: Color = rgb(0x9a8f80);
const ESTRAZA: Color = rgb(0xe6c58a);
/// A candle's flame, and the amber it throws on the rain.
const FLAMA: Color = rgb(0xffd678);
const FLAMA_EDGE: Color = rgb(0xff9628);

/// Salt for the holiday's own RNG ("posadas!").
const SALT: u64 = 0x706f_7361_6461_7321;

/// The lamps the rain can catch the light of: bulbs and farolitos (the
/// glow field names them with a byte).
const MAX_LAMPS: usize = 240;
/// The most bulbs on one string.
const MAX_BULBS: i32 = 100;
const MAX_WALKERS: usize = 8;
/// Seconds the candles stay alight after a strike.
const AFTERGLOW: f32 = 2.4;

/// The ink map's letters: umbrella, flame, wax, canela, caña, azul, burro,
/// dorado (the default ink is the figure's own, bone).
const LETTERS: &str = "ufwchapd";
/// The index in the inks of the umbrella, whose color each walker picks.
const UMBRELLA: usize = 1;
/// The flame and the candle wax.
const CANDLE: &str = "fw";

/// A veladora's flame as it flickers.
const FLAME: [char; 3] = ['♦', '◆', '♦'];

/// The figures of the posada, each with its two steps: José, María on the
/// burro, a peregrino and a little one (the pixel rows of each, two to a
/// cell, drawn in half blocks).
static FIGURES: [[Sprite; 2]; 4] = [
    // jose, one sprite per step
    [
        Sprite {
            art: &[
                "   ▄▄██▄▄   █",
                "  ▀▀▀███▀▀  █",
                "    ▄███▄   █",
                "   ███████▀▀█▀",
                "   ███████  █",
                "   ▀█████▀  █",
                "   ▄█   █▄  █",
            ],
            ink: &[
                "   hhhhhh   f",
                "  hhhhhhhh  c",
                "            c",
                "   ccccccccccc",
                "   ccccccc  c",
                "   ccccccc  c",
                "   cc   cc  c",
            ],
            w: 14,
        },
        Sprite {
            art: &[
                "   ▄▄██▄▄   █",
                "  ▀▀▀███▀▀  █",
                "    ▄███▄   █",
                "   ███████▀▀█▀",
                "   ███████  █",
                "   ▀█████▀  █",
                "     █ █    █",
            ],
            ink: &[
                "   hhhhhh   f",
                "  hhhhhhhh  c",
                "            c",
                "   ccccccccccc",
                "   ccccccc  c",
                "   ccccccc  c",
                "     c c    c",
            ],
            w: 14,
        },
    ],
    // maria, one sprite per step
    [
        Sprite {
            art: &[
                "         ▄▀▀▀▀▄",
                "         ▄████▄       ▄ ▄",
                "        ▄██████▄      █▄█",
                "       ██████████▄   ▄████▄▄",
                "      ████████████▄▄██████ █",
                "  ▄█████████████████████▀",
                "▄████████████████████▀▀",
                "▀ █▀██▀▀▀▀▀▀▀▀▀██▀██",
                "  █ ██         ██ ██",
                "  ▀ ▀▀         ▀▀ ▀▀",
            ],
            ink: &[
                "         dddddd",
                "         aaaaaa       p p",
                "        aaaaaaaa      ppp",
                "       aaaaaaaaaaa   ppppppp",
                "      aaaaaaaaaaaapppppppp p",
                "  ppppaaaaaaaaaaapppppppp",
                "ppppppppppppppppppppppp",
                "p pppppppppppppppppp",
                "  p pp         pp pp",
                "  p pp         pp pp",
            ],
            w: 28,
        },
        Sprite {
            art: &[
                "         ▄▀▀▀▀▄",
                "         ▄████▄       ▄ ▄",
                "        ▄██████▄      █▄█",
                "       ██████████▄   ▄████▄▄",
                "      ████████████▄▄██████ █",
                "  ▄█████████████████████▀",
                "▄████████████████████▀▀",
                "▀ ▀██▀█▀▀▀▀▀▀▀█▀██▀▀▄",
                "   █▀  █    ▄▀  █▀   ▀▄",
                "  ▀▀   ▀▀  ▀▀  ▀▀     ▀▀",
            ],
            ink: &[
                "         dddddd",
                "         aaaaaa       p p",
                "        aaaaaaaa      ppp",
                "       aaaaaaaaaaa   ppppppp",
                "      aaaaaaaaaaaapppppppp p",
                "  ppppaaaaaaaaaaapppppppp",
                "ppppppppppppppppppppppp",
                "p ppppppppppppppppppp",
                "   pp  p    pp  pp   pp",
                "  pp   pp  pp  pp     pp",
            ],
            w: 28,
        },
    ],
    // peregrino, one sprite per step
    [
        Sprite {
            art: &[
                "  ▄█████▄",
                "▄█████████▄",
                "     █   ▄",
                "    ███  █",
                "   █████▀▀",
                "   █████",
                "  ▄█   █▄",
            ],
            ink: &[
                "  uuuuuuu",
                "uuuuuuuuuuu",
                "         f",
                "         w",
                "",
                "",
                "",
            ],
            w: 11,
        },
        Sprite {
            art: &[
                "  ▄█████▄",
                "▄█████████▄",
                "     █   ▄",
                "    ███  █",
                "   █████▀▀",
                "   █████",
                "    █ █",
            ],
            ink: &[
                "  uuuuuuu",
                "uuuuuuuuuuu",
                "         f",
                "         w",
                "",
                "",
                "",
            ],
            w: 11,
        },
    ],
    // nino, one sprite per step
    [
        Sprite {
            art: &[" ▄█████▄", "▀▀▀▀█▀▀▀▀", "   ███ █", "  █████▀", "  ▄█ █▄"],
            ink: &[" uuuuuuu", "uuuuuuuuu", "       f", "       w", ""],
            w: 9,
        },
        Sprite {
            art: &[" ▄█████▄", "▀▀▀▀█▀▀▀▀", "   ███ █", "  █████▀", "   █ █"],
            ink: &[" uuuuuuu", "uuuuuuuuu", "       f", "       w", ""],
            w: 9,
        },
    ],
];

/// Who walks: indices into `FIGURES`.
const JOSE: u8 = 0;
const MARIA: u8 = 1;
const PEREGRINO: u8 = 2;
const NINO: u8 = 3;
/// After María come the peregrinos, grown-ups and little ones in turn.
const FOLLOWERS: [u8; 7] = [PEREGRINO, NINO, PEREGRINO, PEREGRINO, NINO, PEREGRINO, NINO];

/// What a lamp is.
#[derive(Clone, Copy, PartialEq)]
enum Lamp {
    /// A bulb on string 0 or 1, the `k`th along it.
    Bulb {
        string: u8,
        k: u16,
    },
    Farolito,
}

#[derive(Clone, Copy)]
struct Light {
    lamp: Lamp,
    x: i32,
    y: i32,
    /// Index into the hues: the six bulbs, then the farolito's amber.
    hue: u8,
    phase: f32,
    /// Current brightness, about 0.15 to 1.
    lit: f32,
}

/// A cell of the cord the lights hang from.
#[derive(Clone, Copy)]
struct Cord {
    x: i16,
    y: i16,
    ch: char,
}

#[derive(Clone, Copy, Default)]
struct Walker {
    who: u8,
    /// Which umbrella.
    look: u8,
    /// Columns behind the one leading, and its width.
    off: i32,
    w: i32,
    step: u8,
}

pub(super) struct Navidad {
    rng: fastrand::Rng,
    fiesta: Fiesta,
    w: i32,
    h: i32,
    /// Seconds the lights have been blinking.
    t: f32,
    cords: Vec<Cord>,
    /// The bulbs (`..bulbs`) and then the farolitos.
    lamps: Vec<Light>,
    bulbs: usize,
    /// The light of each lamp on the rain around it.
    glow: Glow,
    /// Puddles on the ground: (left column, width).
    puddles: Vec<(i32, i32)>,
    /// For each column, the bulb nearest it and how far (255 = none near).
    under: Vec<(u8, u8)>,
    /// The posada the last strike caught.
    walkers: [Walker; MAX_WALKERS],
    nwalkers: usize,
    /// Which way it walks (1 right, -1 left) and its left edge.
    dir: i32,
    left: i32,
    total: i32,
    /// Seconds since the last strike.
    since: f32,

    hues: [Color; 7],
    /// Each bulb's colors: unlit, glowing, lit.
    bulb_cols: [[Color; 3]; 6],
    cord_col: Color,
    water: Color,
    bag: Color,
    umbrellas: [Color; 4],
    inks: [Color; LETTERS.len() + 1],
    flash_color: Color,
}

impl Navidad {
    pub fn new(seed: u64) -> Navidad {
        Navidad {
            rng: fastrand::Rng::with_seed(seed ^ SALT),
            fiesta: Fiesta::NONE,
            w: 0,
            h: 0,
            t: 0.0,
            cords: Vec::new(),
            lamps: Vec::new(),
            bulbs: 0,
            glow: Glow::new(),
            puddles: Vec::new(),
            under: Vec::new(),
            walkers: [Walker::default(); MAX_WALKERS],
            nwalkers: 0,
            dir: 1,
            left: 0,
            total: 0,
            since: AFTERGLOW,
            hues: [Color::Reset; 7],
            bulb_cols: [[Color::Reset; 3]; 6],
            cord_col: Color::Reset,
            water: Color::Reset,
            bag: Color::Reset,
            umbrellas: [Color::Reset; 4],
            inks: [Color::Reset; LETTERS.len() + 1],
            flash_color: Color::Reset,
        }
    }

    pub fn on(&self) -> bool {
        self.fiesta.is(Kind::Navidad)
    }

    fn level(&self) -> f32 {
        self.fiesta.level.clamp(0.0, 1.0)
    }

    /// How much more often lightning strikes: the posada wants to be seen.
    pub fn storm(&self) -> f32 {
        if self.on() {
            1.0 + 3.0 * self.level()
        } else {
            1.0
        }
    }

    /// Seconds until a first strike once the holiday comes on.
    pub fn soon(&mut self) -> f32 {
        3.0 + self.rng.f32() * 6.0
    }

    /// Follows the holiday and the screen size, laying everything out again
    /// when either changes. True when the holiday has just come on.
    pub fn set(&mut self, fiesta: Fiesta, w: i32, h: i32) -> bool {
        let fiesta = if fiesta.is(Kind::Navidad) {
            fiesta
        } else {
            Fiesta::NONE
        };
        if fiesta == self.fiesta && (w, h) == (self.w, self.h) {
            return false;
        }
        let came = fiesta.is(Kind::Navidad) && !self.fiesta.is(Kind::Navidad);
        self.fiesta = fiesta;
        self.w = w;
        self.h = h;
        self.layout();
        came
    }

    fn layout(&mut self) {
        self.cords.clear();
        self.lamps.clear();
        self.bulbs = 0;
        self.glow.reset(0, 0);
        self.puddles.clear();
        self.under.clear();
        self.nwalkers = 0;
        if !self.on() {
            return;
        }
        let (w, h) = (self.w, self.h);
        let l = self.level();
        let g = h - 1;
        self.glow.reset(w, h);

        // The string along the eaves, under the overlay's own: a bulb every
        // few columns, closer together as the posadas climb, and a second
        // string below the first from the middle of the posadas on.
        if w >= 12 && h >= 12 {
            let top = if h >= 24 {
                6
            } else if h >= 16 {
                5
            } else {
                4
            };
            let depth = if h >= 24 { 2.0 } else { 1.0 };
            let strings = if h >= 20 && l >= 0.65 { 2 } else { 1 };
            let pitch = ((6.5 - 3.0 * l).round() as i32)
                .clamp(3, 7)
                .max((w + MAX_BULBS - 1) / MAX_BULBS);
            self.cords.reserve_exact(w as usize * strings as usize);
            for s in 0..strings {
                let swag = 26 + 7 * s;
                let shift = self.rng.i32(0..swag);
                let y0 = top + 4 * s;
                let ys: Vec<i32> = (0..w)
                    .map(|x| {
                        let u = ((x + shift) % swag) as f32 / swag as f32;
                        y0 + (depth * 4.0 * u * (1.0 - u)).round() as i32
                    })
                    .collect();
                for x in 0..w {
                    let (y, next) = (ys[x as usize], ys[(x as usize + 1).min(ys.len() - 1)]);
                    let ch = match next.cmp(&y) {
                        std::cmp::Ordering::Greater => '╲',
                        std::cmp::Ordering::Less => '╱',
                        std::cmp::Ordering::Equal => '─',
                    };
                    self.cords.push(Cord {
                        x: x as i16,
                        y: y as i16,
                        ch,
                    });
                }
                let mut k = 0u16;
                let mut x = pitch / 2 + 1 + s * (pitch / 2).max(1);
                while x < w - 1 {
                    let hue = (k % 6) as u8;
                    self.lamps.push(Light {
                        lamp: Lamp::Bulb { string: s as u8, k },
                        x,
                        y: ys[x as usize] + 1,
                        hue,
                        phase: self.rng.f32(),
                        lit: 1.0,
                    });
                    k += 1;
                    x += pitch;
                }
            }
        }
        self.bulbs = self.lamps.len();

        // Farolitos in little rows along the street.
        if w >= 12 && h >= 6 {
            let n = ((w as f32 / 40.0) * (0.5 + 0.6 * l))
                .round()
                .clamp(1.0, 8.0) as i32;
            let slot = w as f32 / n as f32;
            for i in 0..n {
                let jitter = (self.rng.f32() - 0.5) * slot * 0.4;
                let cx = ((i as f32 + 0.5) * slot + jitter) as i32;
                let k = 2 + self.rng.i32(0..=(1 + (l * 1.5) as i32));
                for j in 0..k {
                    self.lamps.push(Light {
                        lamp: Lamp::Farolito,
                        x: (cx + 4 * j - 2 * (k - 1)).clamp(1, w - 2),
                        y: g - 1,
                        hue: 6,
                        phase: self.rng.f32() * 100.0,
                        lit: 1.0,
                    });
                }
            }
        }
        self.lamps.truncate(MAX_LAMPS);

        // Their light on the rain, and in the puddles.
        for (i, lt) in self.lamps.iter().enumerate() {
            let radii = if lt.lamp == Lamp::Farolito {
                (8.0, 4.0)
            } else {
                (5.0, 3.0)
            };
            self.glow.add(i as u8, (lt.x, lt.y), radii);
        }
        self.under.resize(w as usize, (255, 255));
        for (i, lt) in self.lamps[..self.bulbs].iter().enumerate() {
            for dx in -4..=4i32 {
                let x = lt.x + dx;
                if x < 0 || x >= w {
                    continue;
                }
                let d = dx.unsigned_abs() as u8;
                let slot = &mut self.under[x as usize];
                if d < slot.1 {
                    *slot = (i as u8, d);
                }
            }
        }

        // Puddles in the street, bigger and more of them as it rains on.
        if w >= 8 && h >= 4 {
            let n = ((w as f32 / 22.0) * (0.5 + 0.5 * l))
                .round()
                .clamp(1.0, 20.0) as i32;
            let slot = w / n;
            for i in 0..n {
                let len = (5.0 + self.rng.f32() * (4.0 + 5.0 * l)) as i32;
                let len = len.min((slot - 2).max(2));
                let x0 = i * slot + self.rng.i32(0..=(slot - len).max(0));
                self.puddles.push((x0, len));
            }
        }
    }

    pub fn strike(&mut self) {
        if !self.on() {
            return;
        }
        self.since = 0.0;
        self.nwalkers = 0;
        let (w, h) = (self.w, self.h);
        if h < 14 || w < 30 {
            return;
        }
        // José leads, María follows on the burro, and then come the
        // peregrinos with their candles, more of them as the posadas climb.
        let want = 1 + (4.0 * self.level()).round() as usize;
        let mut off = 0;
        for i in 0..MAX_WALKERS.min(2 + want) {
            let who = match i {
                0 => JOSE,
                1 => MARIA,
                i => FOLLOWERS[(i - 2) % FOLLOWERS.len()],
            };
            let fw = FIGURES[who as usize][0].w;
            if off + fw > w - 2 {
                break;
            }
            self.walkers[self.nwalkers] = Walker {
                who,
                look: self.rng.u8(0..4),
                off,
                w: fw,
                step: self.rng.u8(0..2),
            };
            self.nwalkers += 1;
            off += fw + 2;
        }
        self.total = off - 2;
        self.dir = if self.rng.bool() { 1 } else { -1 };
        // Room to take a few steps either way.
        let room = (w - self.total - 6).max(0);
        self.left = 3 + self.rng.i32(0..=room);
    }

    pub fn step(&mut self, dt: f32) {
        if !self.on() {
            return;
        }
        self.since = (self.since + dt).min(AFTERGLOW);
        self.t = (self.t + dt) % 7200.0;
        let t = self.t;
        for lt in self.lamps.iter_mut() {
            lt.lit = match lt.lamp {
                Lamp::Bulb { string, k } => {
                    // The controller changes program every so often.
                    let mode = (t / 9.0) as usize + string as usize;
                    blink(mode % 4, k as usize, lt.phase, t)
                }
                Lamp::Farolito => {
                    let t = lt.phase + t;
                    let n = (t * 7.3).sin() * 0.5 + (t * 13.1 + 1.7).sin() * 0.3;
                    0.82 + 0.18 * n
                }
            };
        }
    }

    pub fn retheme(&mut self, ctx: &Ctx, flash: Color, water: Color) {
        self.flash_color = flash;
        let native = ctx.theme.native;
        for (i, slot) in self.hues.iter_mut().take(6).enumerate() {
            *slot = ctx.pick(i, FOQUITOS[i]);
        }
        self.hues[6] = FLAMA_EDGE;
        for (cols, &hue) in self.bulb_cols.iter_mut().zip(&self.hues) {
            *cols = [ctx.fade(hue, 0.75), ctx.fade(hue, 0.3), hue];
        }
        self.cord_col = ctx.fade(ctx.pick(1, PINO), 0.1);
        self.water = water;
        let fg = if native { BLANCO } else { ctx.theme.fg };
        self.bag = if native { ESTRAZA } else { ctx.fade(fg, 0.15) };
        self.umbrellas = [
            ctx.pick(0, ROJO),
            ctx.pick(1, PINO),
            ctx.pick(2, DORADO),
            ctx.pick(4, AZUL),
        ];
        self.inks = [
            fg,
            ROJO,
            FLAMA,
            fg,
            ctx.pick(3, CANELA),
            ctx.pick(2, CANA),
            ctx.pick(4, AZUL),
            if native { BURRO } else { ctx.fade(fg, 0.3) },
            ctx.pick(2, DORADO),
        ];
    }

    /// A raindrop's color near a lamp: it catches the light of the bulb.
    #[inline]
    pub fn tint(&self, x: i32, y: i32, c: Color) -> Color {
        let Some((s, i)) = self.glow.at(x, y) else {
            return c;
        };
        let lt = &self.lamps[i];
        c.lerp(self.hues[lt.hue as usize], s * lt.lit * 0.85)
    }

    /// The string along the eaves: the cord, and the bulbs on it.
    pub fn draw_lights(&self, c: &mut Canvas, flash: f32) {
        let cord = self.cord_col.lerp(self.flash_color, flash * 0.5);
        for p in &self.cords {
            c.set(p.x as i32, p.y as i32, p.ch, cord);
        }
        for lt in &self.lamps[..self.bulbs] {
            let (ch, shade, attr) = match lt.lit {
                l if l > 0.72 => ('●', 2, BOLD),
                l if l > 0.4 => ('•', 1, 0),
                _ => ('·', 0, 0),
            };
            let col = self.bulb_cols[lt.hue as usize][shade].lerp(self.flash_color, flash * 0.3);
            c.set_attr(lt.x, lt.y, ch, col, attr);
        }
    }

    /// Puddles on the ground, with the bulbs reflected in them, a little
    /// broken by the ripples.
    pub fn draw_puddles(&self, c: &mut Canvas, flash: f32) {
        let g = self.h - 1;
        let last = self.under.len() as i32 - 1;
        for &(x0, len) in &self.puddles {
            for x in x0..x0 + len {
                let wobble = ((self.t * 1.9 + x as f32 * 0.9).sin() * 1.4).round() as i32;
                let (mut col, mut attr) = (self.water, 0);
                if let Some(&(i, d)) = self.under.get((x + wobble).clamp(0, last) as usize)
                    && d <= 3
                {
                    let lt = &self.lamps[i as usize];
                    let k = ((lt.lit - 0.2) / 0.8).max(0.0) * (1.0 - 0.22 * d as f32);
                    col = col.lerp(self.hues[lt.hue as usize], k * 0.95);
                    if k > 0.55 {
                        attr = BOLD;
                    }
                }
                col = col.lerp(self.flash_color, flash * 0.4);
                c.set_attr(x, g, '≈', col, attr);
            }
        }
    }

    /// The farolitos: a paper bag glowing with its candle.
    pub fn draw_farolitos(&self, c: &mut Canvas, flash: f32) {
        let g = self.h - 1;
        for lt in &self.lamps[self.bulbs..] {
            let core = FLAMA.lerp(FLAMA_EDGE, 1.0 - lt.lit);
            let f = ((lt.phase + self.t) * 5.0) as usize % FLAME.len();
            c.set_attr(lt.x, g - 1, FLAME[f], core, BOLD);
            let bag = self
                .bag
                .scale(0.55)
                .lerp(FLAMA, 0.5 * lt.lit)
                .lerp(self.flash_color, flash * 0.3);
            c.set(lt.x - 1, g, '▐', bag);
            c.set(lt.x, g, '█', bag);
            c.set(lt.x + 1, g, '▌', bag);
        }
    }

    /// The posada the last strike caught, lit by `flash` and fading after,
    /// with a step at each flicker of the flash (`beat`); the candles stay
    /// alight a while longer in the dark.
    pub fn draw_procession(&self, ctx: &Ctx, c: &mut Canvas, flash: f32, beat: usize) {
        if self.since >= AFTERGLOW || self.nwalkers == 0 {
            return;
        }
        // The eye holds what the flash showed, then lets it go.
        let after = 0.8 * (-(self.since - 0.3).max(0.0) / 0.6).exp();
        let vis = flash.max(after);
        // The candles burn on until the afterglow is nearly over.
        let burn = (0.9 * (AFTERGLOW - self.since) / 0.8).clamp(0.0, 0.9);
        let flames = vis.max(burn);
        if flames < 0.03 {
            return;
        }
        let mut inks = self.inks;
        inks[0] = inks[0].lerp(self.flash_color, flash * 0.5);
        for (k, col) in inks.iter_mut().enumerate() {
            let candle = k > 0 && CANDLE.contains(LETTERS.as_bytes()[k - 1] as char);
            *col = ctx.fade(*col, 1.0 - if candle { flames } else { vis });
        }
        let bottom = self.h - 2;
        let stride = 2 * beat as i32;
        for d in &self.walkers[..self.nwalkers] {
            let s = &FIGURES[d.who as usize][(d.step as usize + beat) % 2];
            let x = if self.dir > 0 {
                self.left + self.total - d.off - d.w + stride
            } else {
                self.left + d.off - stride
            };
            let mut inks = inks;
            inks[UMBRELLA] = ctx.fade(self.umbrellas[d.look as usize], 1.0 - vis);
            let only = (vis < 0.03).then_some(CANDLE);
            s.draw(c, (x, bottom), self.dir < 0, (LETTERS, &inks), only);
        }
    }
}

/// How bright bulb `k` of a string is at time `t` in a program: 0.15 to 1.
fn blink(mode: usize, k: usize, phase: f32, t: f32) -> f32 {
    use std::f32::consts::{PI, TAU};
    let b = match mode {
        // A chase: a run of light travels along the string.
        0 => {
            let u = (t * 3.0 - k as f32).rem_euclid(6.0);
            let b = (1.0 - (u - 1.0).abs() / 1.6).clamp(0.0, 1.0);
            b * b * (3.0 - 2.0 * b)
        }
        // Every other bulb, then the others.
        1 => {
            let s = (t * TAU / 1.6 + (k & 1) as f32 * PI).sin();
            0.5 + 0.5 * (s * 4.0).clamp(-1.0, 1.0)
        }
        // Twinkling, each bulb to its own beat.
        2 => {
            let s = (t * (0.9 + (phase * 5.0).fract() * 1.6) + phase * 7.0).sin();
            (0.5 + 0.5 * s).powi(2)
        }
        // The colors swell and ebb in turn.
        _ => 0.5 + 0.5 * (t * 1.3 - (k % 6) as f32 * PI / 3.0).sin(),
    };
    0.15 + 0.85 * b
}
