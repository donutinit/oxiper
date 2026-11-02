//! rain's Fiestas Patrias: September rain on a street of fondas and puestos
//! under tricolor papel picado. The fondas stand along the ground under
//! toldos in verde, blanco y rojo with their names on a sign and a lit
//! window, the puestos beside them each with a bandera on a pole; strings of
//! papel picado hang soaked across the street and drip, the drops falling
//! straight down past the shop fronts to splash on the wet ground, where
//! scraps of paper stick; the lit windows warm the rain around them. Every
//! flash of lightning catches charros galloping down the street, flag in
//! hand, a new stride at each flicker, fading back into the night after it.
//! More of it all as the mes patrio climbs to the Grito.
//!
//! All of it draws from its own RNG, so ordinary days keep their numbers.

use super::art::{self, Cord, Glow, Sprite};
use crate::color::{self, Color, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Canvas};
use crate::scene::Ctx;

const VERDE: Color = rgb(0x006847);
const BLANCO: Color = rgb(0xf4f1ea);
const ROJO: Color = rgb(0xce1126);
const CAFE: Color = rgb(0x7a4a1e);
const MADERA: Color = rgb(0xb4783c);
const CANA: Color = rgb(0xc9b26b);
const PLATA: Color = rgb(0xc9ccd6);
/// Natural string, like the overlay's.
const CUERDA: Color = rgb(0xbcae94);
/// The light of a lamp in a window.
const LAMPARA: Color = rgb(0xff9628);

/// Salt for the holiday's own RNG ("mespatri").
const SALT: u64 = 0x6d65_7370_6174_7269;

const MAX_STALLS: usize = 200;
const MAX_RIDERS: usize = 4;
/// Seconds the riders stay faintly visible after a strike.
const AFTERGLOW: f32 = 2.4;

/// The inks of the street.
const ROOF: u8 = 0;
/// A toldo's stripes: verde, blanco, rojo.
const STRIPE: u8 = 1;
const POST: u8 = 4;
const WINDOW: u8 = 5;
const SIGN: u8 = 6;
const COUNTER: u8 = 7;

/// The names on the signs.
const FONDAS: [&str; 5] = ["FONDA DOÑA LUPE", "ANTOJITOS", "POZOLE", "FONDA", "ATOLE"];
const PUESTOS: [&str; 4] = ["AGUAS FRESCAS", "TAMALES", "ELOTES", "PAN"];

/// A flag of papel picado, four wide: a band of paper along the string and
/// the cut fringe below it, in one of these.
const FRINGES: [&str; 3] = ["▀▄▀▄", "▀▀▄▀", "▄▀▀▄"];

/// The ink map's letters of a rider: bandera's rojo, blanco and verde, the
/// horse's coat, the sombrero, the jacket and the flagpole (the default ink
/// is the charro's own face).
const LETTERS: &str = "bwvnhpc";
/// The index in the inks of the coat, which each horse picks.
const COAT: usize = 4;

/// A charro on horseback in three strides of the gallop, flag in hand (the
/// pixel rows of each, two to a cell, drawn in half blocks).
static GALLOP: [Sprite; 3] = [
    Sprite {
        art: &[
            "             ▄███",
            "   ▄▄▄▄▄▄▄██▄████▄▄",
            "  ██████████ ████      █▄█",
            "  █▀▀▀▀▀▀▀██▄███▀     ▄███▄",
            "          ██ ███    ▄████▀███▄",
            " ▄████████████████▄█████   ▀▀▀",
            "█▀    ██████████████████",
            "      ▀█████████████████",
            "   ▄▄█▀██▀         ▀▀██▀▀█▄▄",
            " █▀▀▀▄█▀              ▀█▄  ▀▀",
            "   ▄█▀                 ▀█",
        ],
        ink: &[
            "             hhhh",
            "   bbwwwvvphhhhhhhh",
            "  bbbwwwvvvc h  h      nnn",
            "  bbbwwwvvvcppppp     nnnnn",
            "          cc ppp    nnnnnnnnnn",
            " nnnnnnnnnccppppnnnnnnnn   nnn",
            "nn    nnnnppnnpnnnnnnnnn",
            "      nnnnnnnnnnnnnnnnnn",
            "   nnnnnnn         nnnnnnnnn",
            " nnnnnnn              nnn  nn",
            "   nnn                 nn",
        ],
        w: 34,
    },
    Sprite {
        art: &[
            "             ▄███",
            "  ▄▄▄▄▄▄▄▄██▄████▄▄",
            "  ██████████ ████      █▄█",
            "  ▀▀▀▀▀▀▀▀██▄███▀     ▄███▄",
            "          ██ ███    ▄████▀███▄",
            "  ▄▄▄▄▄███████████▄█████   ▀▀▀",
            " ▄█▀▀▀██████████████████",
            "█▀    ▀████████████████▀",
            "         ▀█▀██      ▀█▄▄",
            "         ▄█  ▀█      ██▀▀",
            "        ██    ▀▀     █▀",
        ],
        ink: &[
            "             hhhh",
            "  bbbwwwvvphhhhhhhh",
            "  bbbwwwvvvc h  h      nnn",
            "  bbbwwwvvvcppppp     nnnnn",
            "          cc ppp    nnnnnnnnnn",
            "  nnnnnnnnccppppnnnnnnnn   nnn",
            " nnnnnnnnnppnnpnnnnnnnnn",
            "nn    nnnnnnnnnnnnnnnnnn",
            "         nnnnn      nnnn",
            "         nn  nn      nnnn",
            "        nn    nn     nn",
        ],
        w: 34,
    },
    Sprite {
        art: &[
            "             ▄███",
            "  ▄▄▄▄▄▄▄▄██▄████▄▄",
            "  ██████████ ████      █▄█",
            "  ▀▀▀▀▀▀▀▀██▄███▀     ▄███▄",
            "          ██ ███    ▄████▀███▄",
            "██████████████████▄█████   ▀▀▀",
            "      ██████████████████",
            "     ▄██████████████████▄▄",
            "▄▄██▀▀███▀          ▀▀███▀▀██",
            "   ▄▄█▀                ▀██▄",
            "   ▀▀                     ▀",
        ],
        ink: &[
            "             hhhh",
            "  bbbwwwvvphhhhhhhh",
            "  bbbwwwvvvc h  h      nnn",
            "  bbbwwwvvvcppppp     nnnnn",
            "          cc ppp    nnnnnnnnnn",
            "nnnnnnnnnnccppppnnnnnnnn   nnn",
            "      nnnnppnnpnnnnnnnnn",
            "     nnnnnnnnnnnnnnnnnnnnn",
            "nnnnnnnnnn          nnnnnnnnn",
            "   nnnn                nnnn",
            "   nn                     n",
        ],
        w: 34,
    },
];

/// A row of a shop front.
#[derive(Clone, Copy, PartialEq)]
enum Row {
    Roof,
    Toldo,
    Zig,
    Window,
    Counter,
}

/// A cell of a shop front, and which shop it is.
#[derive(Clone, Copy)]
struct Tile {
    x: i16,
    y: i16,
    ch: char,
    ink: u8,
    stall: u8,
}

/// The lamp in a shop's window.
#[derive(Clone, Copy)]
struct Stall {
    phase: f32,
    /// Current brightness, about 0.7 to 1.
    lit: f32,
}

/// A bandera on its pole: the flag in rows `y` and below, the pole down to
/// `foot`.
#[derive(Clone, Copy)]
struct Pole {
    x: i16,
    y: i16,
    foot: i16,
}

/// A flag of papel picado hanging from a string.
#[derive(Clone, Copy)]
struct Paper {
    x: i16,
    y: i16,
    /// Verde, blanco or rojo.
    hue: u8,
    cut: u8,
    /// Seconds to the next drip from its fringe.
    next: f32,
}

#[derive(Clone, Copy)]
struct Drip {
    x: f32,
    y: f32,
    vy: f32,
}

#[derive(Clone, Copy, Default)]
struct Rider {
    x: i32,
    coat: u8,
    step: u8,
}

pub(super) struct Patrias {
    rng: fastrand::Rng,
    fiesta: Fiesta,
    w: i32,
    h: i32,
    /// Seconds the flags have been fluttering.
    t: f32,
    tiles: Vec<Tile>,
    stalls: Vec<Stall>,
    poles: Vec<Pole>,
    cords: Vec<Cord>,
    papers: Vec<Paper>,
    drips: Vec<Drip>,
    drip_cap: usize,
    /// Where drips have landed: column and age.
    splats: Vec<(i16, f32)>,
    /// Scraps of paper on the wet street: column, glyph, hue.
    scraps: Vec<(i16, char, u8)>,
    /// The light of the windows on the rain around them.
    glow: Glow,
    riders: [Rider; MAX_RIDERS],
    nriders: usize,
    /// Which way they gallop (1 right, -1 left).
    dir: i32,
    /// Seconds since the last strike.
    since: f32,

    /// The flag's verde, blanco and rojo, and the soaked paper's.
    bandera: [Color; 3],
    papel: [Color; 3],
    inks: [Color; 8],
    string: Color,
    water: Color,
    rider_inks: [Color; LETTERS.len() + 1],
    coats: [Color; 4],
    flash_color: Color,
}

impl Patrias {
    pub fn new(seed: u64) -> Patrias {
        Patrias {
            rng: fastrand::Rng::with_seed(seed ^ SALT),
            fiesta: Fiesta::NONE,
            w: 0,
            h: 0,
            t: 0.0,
            tiles: Vec::new(),
            stalls: Vec::new(),
            poles: Vec::new(),
            cords: Vec::new(),
            papers: Vec::new(),
            drips: Vec::new(),
            drip_cap: 0,
            splats: Vec::new(),
            scraps: Vec::new(),
            glow: Glow::new(),
            riders: [Rider::default(); MAX_RIDERS],
            nriders: 0,
            dir: 1,
            since: AFTERGLOW,
            bandera: [Color::Reset; 3],
            papel: [Color::Reset; 3],
            inks: [Color::Reset; 8],
            string: Color::Reset,
            water: Color::Reset,
            rider_inks: [Color::Reset; LETTERS.len() + 1],
            coats: [Color::Reset; 4],
            flash_color: Color::Reset,
        }
    }

    pub fn on(&self) -> bool {
        self.fiesta.is(Kind::Patrias)
    }

    fn level(&self) -> f32 {
        self.fiesta.level.clamp(0.0, 1.0)
    }

    /// How much more often lightning strikes: September storms, and the
    /// charros want to be seen.
    pub fn storm(&self) -> f32 {
        if self.on() {
            1.0 + 2.5 * self.level()
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
        let fiesta = if fiesta.is(Kind::Patrias) {
            fiesta
        } else {
            Fiesta::NONE
        };
        if fiesta == self.fiesta && (w, h) == (self.w, self.h) {
            return false;
        }
        let came = fiesta.is(Kind::Patrias) && !self.fiesta.is(Kind::Patrias);
        self.fiesta = fiesta;
        self.w = w;
        self.h = h;
        self.layout();
        came
    }

    fn layout(&mut self) {
        self.tiles.clear();
        self.stalls.clear();
        self.poles.clear();
        self.cords.clear();
        self.papers.clear();
        self.drips.clear();
        self.splats.clear();
        self.scraps.clear();
        self.glow.reset(0, 0);
        self.drip_cap = 0;
        self.nriders = 0;
        if !self.on() {
            return;
        }
        let (w, h) = (self.w, self.h);
        let l = self.level();
        self.glow.reset(w, h);

        // The street: fondas and puestos along the ground, closer together
        // as the month goes on.
        let rows = match h {
            64.. => 7,
            52.. => 6,
            40.. => 5,
            24.. => 4,
            14.. => 3,
            10.. => 2,
            _ => 0,
        };
        if rows > 0 && w >= 14 {
            let mut x = self.rng.i32(0..4);
            while x < w - 6 && self.stalls.len() < MAX_STALLS {
                let fonda = self.rng.f32() < 0.55;
                let sw = if fonda {
                    11 + self.rng.i32(0..7)
                } else {
                    7 + self.rng.i32(0..3)
                };
                let sw = sw.min(w - x);
                if sw < 6 {
                    break;
                }
                self.build(x, sw, rows, fonda);
                x += sw + 3 + self.rng.i32(0..6) + ((1.0 - l) * 8.0) as i32;
            }
        }

        // Papel picado across the street, soaked: one string, and a second
        // at the Grito, hung with flags in verde, blanco y rojo.
        if h >= 18 && w >= 14 {
            let strings = if h >= 22 && l >= 0.75 { 2 } else { 1 };
            let depth = if h >= 24 { 2.0 } else { 1.0 };
            let py = ((h as f32 * 0.42) as i32).max(8);
            self.cords.reserve_exact(w as usize * strings as usize);
            for s in 0..strings {
                let swag = 28 + 6 * s;
                let shift = self.rng.i32(0..swag);
                let base = self.cords.len();
                art::hang(&mut self.cords, w, py + 6 * s, (depth, swag, shift));
                let fill = 0.3 + 0.55 * l;
                let (mut k, mut x) = (0u8, 1 + 3 * s);
                while x + 4 <= w {
                    if self.rng.f32() < fill {
                        self.papers.push(Paper {
                            x: x as i16,
                            y: self.cords[base + x as usize + 1].y + 1,
                            hue: k % 3,
                            cut: self.rng.u8(0..3),
                            next: self.rng.f32() * 6.0,
                        });
                    }
                    k = k.wrapping_add(1);
                    x += 5;
                }
            }
            self.drip_cap = (self.papers.len() / 2).clamp(8, 80);
            self.drips.reserve_exact(self.drip_cap);
            self.splats.reserve_exact(self.drip_cap);
        }

        // Scraps of paper stuck to the wet street.
        if w >= 8 && h >= 4 {
            let n = ((w as f32 / 12.0) * (0.4 + 0.8 * l)) as usize;
            for _ in 0..n {
                let glyph = [',', '▪', '.', ','][self.rng.usize(0..4)];
                let x = self.rng.i32(0..w) as i16;
                self.scraps.push((x, glyph, self.rng.u8(0..3)));
            }
        }
    }

    /// Builds the shop front `w` wide that starts at column `x`: a toldo
    /// in stripes over a lit window with a sign, a counter below it, and
    /// perhaps a bandera on a pole above.
    fn build(&mut self, x: i32, w: i32, rows: i32, fonda: bool) {
        // Puestos stand a row lower.
        let rows = if fonda { rows } else { rows - 1 };
        let top = self.h - 2 - (rows - 1);
        let idx = self.stalls.len() as u8;
        // From the top: the roof (on the tall ones), the toldo with its
        // zigzag edge, as many rows of window as there is room for, and the
        // counter.
        let mut kinds = [Row::Window; 7];
        let head: &[Row] = match rows {
            2 => &[Row::Toldo],
            3 | 4 => &[Row::Toldo, Row::Zig],
            _ => &[Row::Roof, Row::Toldo, Row::Zig],
        };
        kinds[..head.len()].copy_from_slice(head);
        if rows >= 4 {
            kinds[rows as usize - 1] = Row::Counter;
        }
        // Any name that fits between the posts.
        let names: &[&str] = if fonda { &FONDAS } else { &PUESTOS };
        let fits = |n: &&&str| n.chars().count() as i32 + 2 <= w;
        let name = match names.iter().filter(fits).count() {
            0 => "",
            n => names
                .iter()
                .filter(fits)
                .nth(self.rng.usize(0..n))
                .unwrap_or(&""),
        };
        let off = self.rng.i32(0..6);
        let mut window = None;
        for (r, &kind) in kinds[..rows as usize].iter().enumerate() {
            let y = top + r as i32;
            for c in 0..w {
                let stripe = STRIPE + ((c + off) / 2 % 3) as u8;
                let (ch, ink) = match kind {
                    Row::Roof => ('▄', ROOF),
                    Row::Toldo => ('█', stripe),
                    Row::Zig => (if c % 2 == 0 { '▀' } else { '▄' }, stripe),
                    Row::Window if c == 0 || c == w - 1 => ('│', POST),
                    Row::Window => ('░', WINDOW),
                    Row::Counter => ('▄', COUNTER),
                };
                self.push_tile(x + c, y, ch, ink, idx);
            }
            if kind == Row::Window && window.is_none() {
                window = Some(y);
                let len = name.chars().count() as i32;
                for (i, ch) in name.chars().enumerate() {
                    self.push_tile(x + (w - len) / 2 + i as i32, y, ch, SIGN, idx);
                }
            }
        }
        let l = self.level();
        let chance = if fonda { 0.3 + 0.5 * l } else { 0.5 + 0.4 * l };
        if self.rng.f32() < chance && top >= 4 {
            let px = if !fonda {
                x + w / 2
            } else if self.rng.bool() {
                x
            } else {
                x + w - 1
            };
            self.poles.push(Pole {
                x: px as i16,
                y: (top - 4) as i16,
                foot: (top - 1) as i16,
            });
        }
        self.glow
            .add(idx, (x + w / 2, window.unwrap_or(top)), (10.0, 4.0));
        self.stalls.push(Stall {
            phase: self.rng.f32() * 100.0,
            lit: 1.0,
        });
    }

    fn push_tile(&mut self, x: i32, y: i32, ch: char, ink: u8, stall: u8) {
        self.tiles.push(Tile {
            x: x as i16,
            y: y as i16,
            ch,
            ink,
            stall,
        });
    }

    pub fn strike(&mut self) {
        if !self.on() {
            return;
        }
        self.since = 0.0;
        self.nriders = 0;
        let (w, h) = (self.w, self.h);
        let sw = GALLOP[0].w;
        if h < 16 || w < sw + 8 {
            return;
        }
        // A charro, and more of them as the month climbs to the Grito.
        let want = 1 + (2.0 * self.level()).round() as i32;
        let n = want.min((w - 8) / (sw + 4)).clamp(1, MAX_RIDERS as i32);
        let slot = w / n;
        self.dir = if self.rng.bool() { 1 } else { -1 };
        for i in 0..n {
            let room = (slot - sw - 8).max(0);
            self.riders[i as usize] = Rider {
                x: i * slot + 4 + self.rng.i32(0..=room),
                coat: self.rng.u8(0..4),
                step: self.rng.u8(0..3),
            };
        }
        self.nriders = n as usize;
    }

    pub fn step(&mut self, dt: f32, wind: f32) {
        if !self.on() {
            return;
        }
        self.since = (self.since + dt).min(AFTERGLOW);
        self.t = (self.t + dt) % 7200.0;
        for s in self.stalls.iter_mut() {
            let t = s.phase + self.t;
            s.lit = 0.85 + 0.15 * ((t * 2.3).sin() * 0.6 + (t * 5.9 + 1.3).sin() * 0.4);
        }

        // Paper that drips: a drop forms at the fringe and lets go.
        let slow = 1.7 - 0.9 * self.level();
        for i in 0..self.papers.len() {
            self.papers[i].next -= dt;
            if self.papers[i].next <= 0.0 {
                let p = self.papers[i];
                self.papers[i].next = (1.5 + self.rng.f32() * 3.5) * slow;
                if self.drips.len() < self.drip_cap {
                    let tip = 2 * self.rng.i32(0..2);
                    self.drips.push(Drip {
                        x: (p.x as i32 + tip) as f32 + 0.5,
                        y: p.y as f32 + 2.0,
                        vy: 0.0,
                    });
                }
            }
        }
        let ground = (self.h - 1) as f32;
        let mut i = 0;
        while i < self.drips.len() {
            let mut d = self.drips[i];
            d.vy += 55.0 * dt;
            d.y += d.vy * dt;
            d.x += wind * 0.25 * d.vy * dt;
            if d.y >= ground {
                if self.splats.len() < self.drip_cap {
                    let x = (d.x.floor() as i32).rem_euclid(self.w.max(1));
                    self.splats.push((x as i16, 0.0));
                }
                self.drips.swap_remove(i);
            } else {
                self.drips[i] = d;
                i += 1;
            }
        }
        for sp in self.splats.iter_mut() {
            sp.1 += dt;
        }
        self.splats.retain(|s| s.1 < 0.24);
    }

    pub fn retheme(&mut self, ctx: &Ctx, flash: Color, water: Color) {
        self.flash_color = flash;
        let native = ctx.theme.native;
        let [verde, blanco, rojo] = tricolor(ctx);
        self.bandera = [verde, blanco, rojo];
        // The paper is soaked: darker than it was hung.
        self.papel = [verde.scale(0.85), blanco.scale(0.7), rojo.scale(0.85)];
        let fg = if native { BLANCO } else { ctx.theme.fg };
        let cafe = if native { CAFE } else { ctx.fade(fg, 0.55) };
        self.inks = [
            cafe,
            self.papel[0],
            self.papel[1],
            self.papel[2],
            ctx.fade(cafe, 0.2),
            LAMPARA,
            fg.scale(0.85),
            cafe.lerp(fg, 0.3),
        ];
        self.string = if native { CUERDA } else { ctx.fade(fg, 0.45) };
        self.water = water;
        self.rider_inks = [
            fg,
            rojo,
            blanco,
            verde,
            cafe,
            ctx.pick(2, CANA),
            if native { PLATA } else { fg.scale(0.7) },
            if native { MADERA } else { ctx.fade(fg, 0.3) },
        ];
        // Alazán, bayo, tordo and blanco.
        self.coats = if native {
            [rgb(0x9b5a2a), rgb(0xc9a96b), rgb(0xa0a4b0), rgb(0xe8e4d8)]
        } else {
            [0.35, 0.2, 0.1, 0.0].map(|k| ctx.fade(fg, k + 0.15))
        };
    }

    /// A raindrop's color near a lit window: it catches the lamplight.
    #[inline]
    pub fn tint(&self, x: i32, y: i32, c: Color) -> Color {
        let Some((s, i)) = self.glow.at(x, y) else {
            return c;
        };
        c.lerp(LAMPARA, s * self.stalls[i].lit * 0.7)
    }

    /// The street behind the rain: the shop fronts with their lit windows,
    /// the banderas on their poles, and the papel picado on its strings.
    pub fn draw_street(&self, c: &mut Canvas, wind: f32, flash: f32) {
        let lift = |col: Color, k: f32| col.lerp(self.flash_color, flash * k);
        for t in &self.tiles {
            let col = if t.ink == WINDOW {
                let lit = self.stalls[t.stall as usize].lit;
                lift(self.inks[WINDOW as usize].scale(0.3 + 0.3 * lit), 0.4)
            } else {
                lift(self.inks[t.ink as usize], 0.5)
            };
            c.set(t.x as i32, t.y as i32, t.ch, col);
        }
        // Flags stream from the pole in the wind, the green at the hoist.
        let out = if wind >= 0.0 { 1 } else { -1 };
        let pole = lift(self.inks[POST as usize], 0.5);
        for p in &self.poles {
            for y in p.y + 2..=p.foot {
                c.set(p.x as i32, y as i32, '│', pole);
            }
            for i in 0..3 {
                let ripple = ((self.t * 5.0 + i as f32 * 1.4 + p.x as f32) as usize) % 4;
                let col = lift(self.bandera[i], 0.4);
                let x = p.x as i32 + out * (1 + i as i32);
                c.set_attr(x, p.y as i32, ['█', '▀', '█', '▄'][ripple], col, BOLD);
                c.set_attr(x, p.y as i32 + 1, ['▄', '█', '▀', '█'][ripple], col, BOLD);
            }
        }
        let cord = lift(self.string, 0.5);
        for p in &self.cords {
            c.set(p.x as i32, p.y as i32, p.ch, cord);
        }
        for p in &self.papers {
            let col = lift(self.papel[p.hue as usize], 0.5);
            let (x, y) = (p.x as i32, p.y as i32);
            for dx in 0..4 {
                c.set(x + dx, y, '█', col);
            }
            for (dx, ch) in FRINGES[p.cut as usize].chars().enumerate() {
                c.set(x + dx as i32, y + 1, ch, col.scale(0.8));
            }
        }
    }

    /// The scraps of paper stuck to the street.
    pub fn draw_scraps(&self, c: &mut Canvas, flash: f32) {
        let g = self.h - 1;
        for &(x, ch, hue) in &self.scraps {
            let col = self.papel[hue as usize].lerp(self.flash_color, flash * 0.3);
            c.set(x as i32, g, ch, col);
        }
    }

    /// The drops that fall from the papel picado, and where they land.
    pub fn draw_drips(&self, c: &mut Canvas) {
        let g = self.h - 1;
        for d in &self.drips {
            let ch = if d.vy > 14.0 { '¦' } else { '\'' };
            let x = (d.x.floor() as i32).rem_euclid(self.w.max(1));
            c.set_attr(x, d.y as i32, ch, self.water, BOLD);
        }
        for &(x, age) in &self.splats {
            if age < 0.08 {
                c.set(x as i32, g, '.', self.water);
            } else {
                c.set(x as i32 - 1, g, '˙', self.water);
                c.set(x as i32 + 1, g, '˙', self.water);
            }
        }
    }

    /// The charros the last strike caught, lit by `flash` and fading after,
    /// a new stride at each flicker of the flash (`beat`).
    pub fn draw_riders(&self, ctx: &Ctx, c: &mut Canvas, flash: f32, beat: usize) {
        if self.since >= AFTERGLOW || self.nriders == 0 {
            return;
        }
        // The eye holds what the flash showed, then lets it go.
        let after = 0.8 * (-(self.since - 0.3).max(0.0) / 0.6).exp();
        let vis = flash.max(after);
        if vis < 0.03 {
            return;
        }
        let mut inks = self.rider_inks;
        inks[0] = inks[0].lerp(self.flash_color, flash * 0.5);
        for col in inks.iter_mut() {
            *col = ctx.fade(*col, 1.0 - vis);
        }
        let bottom = self.h - 2;
        for r in &self.riders[..self.nriders] {
            let s = &GALLOP[(r.step as usize + beat) % GALLOP.len()];
            let mut inks = inks;
            inks[COAT] = ctx.fade(self.coats[r.coat as usize], 1.0 - vis);
            let x = r.x + self.dir * 4 * beat as i32;
            s.draw(c, (x, bottom), self.dir < 0, (LETTERS, &inks), None);
        }
    }
}

/// Verde, blanco and rojo as the theme paints them: the flag's own colors
/// under `classic`; otherwise the theme's accents closest to green and red,
/// and its foreground for the white.
fn tricolor(ctx: &Ctx) -> [Color; 3] {
    if ctx.theme.native {
        return [VERDE, BLANCO, ROJO];
    }
    let near = |hue: f32, i: usize, native: Color| {
        by_hue(&ctx.theme.colors, hue).unwrap_or_else(|| ctx.pick(i, native))
    };
    [near(135.0, 2, VERDE), ctx.theme.fg, near(355.0, 1, ROJO)]
}

/// The colorful accent whose hue is nearest `hue`, if it is close.
fn by_hue(colors: &[Color], hue: f32) -> Option<Color> {
    colors
        .iter()
        .copied()
        .filter_map(|c| {
            let (h, s, v) = hsv(c)?;
            let d = (h - hue).abs() % 360.0;
            (s >= 0.2 && v >= 0.3).then_some((d.min(360.0 - d), c))
        })
        .filter(|&(d, _)| d <= 55.0)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

/// Hue (degrees), saturation and value of a color.
fn hsv(c: Color) -> Option<(f32, f32, f32)> {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => color::ansi_rgb(n),
        Color::Reset => return None,
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    let h = if d <= 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    Some((h, if max > 0.0 { d / max } else { 0.0 }, max))
}
