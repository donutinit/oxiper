//! rain's Día de Muertos: cempasúchil petals fall with the rain and gather
//! along the ground into the path home, veladoras glow in the downpour and
//! warm the drops around them, splashes throw up petals, and every flash of
//! lightning catches the dead dancing in the dark: a skeleton kicking up its
//! heels, a charro with maracas, La Catrina in her feathered hat, a
//! guitarrista, each in a new step at every flicker, fading back into the
//! night with the afterglow.
//!
//! All of it draws from its own RNG, so ordinary days keep their numbers.

use crate::color::{Color, mix_oklab, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Canvas};
use crate::scene::Ctx;

const CEMPA: Color = rgb(0xff8c00);
const CEMPA_LIGHT: Color = rgb(0xffaa1e);
const CEMPA_DARK: Color = rgb(0xd66200);
const ROSA: Color = rgb(0xe4007c);
const TERCIOPELO: Color = rgb(0xb00048);
const MORADO: Color = rgb(0x7028a0);
const TURQUESA: Color = rgb(0x00aab8);
const AMARILLO: Color = rgb(0xffd200);
const LIMON: Color = rgb(0x8cc828);
const HUESO: Color = rgb(0xf4eee2);
const MADERA: Color = rgb(0xb4783c);
const FLAMA: Color = rgb(0xffd678);
const FLAMA_EDGE: Color = rgb(0xff9628);

/// Salt for the holiday's own RNG ("muertos!").
const SALT: u64 = 0x6d75_6572_746f_7321;

const MAX_DANCERS: usize = 6;
/// Seconds the dancers stay faintly visible after a strike.
const AFTERGLOW: f32 = 2.4;

/// A figure in characters, with a parallel ink map: a space is bone, a
/// letter one of the papel picado colors (see [`ink`]).
struct Sprite {
    art: &'static [&'static str],
    ink: &'static [&'static str],
    w: i32,
}

const SPRITES: [Sprite; 9] = [
    // 0-2: el bailarín, arms up, a kick, hands on hips.
    Sprite {
        art: &[
            "\\  .-.  /",
            " \\(o.o)/",
            "  `=|=´",
            "   =|=",
            "   /^\\",
            "  /   \\",
            " /     \\",
        ],
        ink: &["", "   t t"],
        w: 9,
    },
    Sprite {
        art: &[
            "    .-. /",
            "   (o.o)/",
            "  ,=|=´",
            " /  =|=",
            "    /^\\_",
            "   /    `",
            "  /",
        ],
        ink: &["", "    t t"],
        w: 9,
    },
    Sprite {
        art: &[
            "   .-.",
            "  (o.o)",
            " /`=|=´\\",
            " \\ =|= /",
            "   /^\\",
            " _/   \\",
            "       \\",
        ],
        ink: &["", "   t t"],
        w: 9,
    },
    // 3-4: the charro with his maracas, under a sombrero.
    Sprite {
        art: &[
            "     _",
            "   _/ \\_",
            " ‹=======›",
            "   (o.o)",
            "  /`=|=´\\",
            " o  =|=  o",
            "    /^\\",
            "    | |",
            "   _| |_",
        ],
        ink: &[
            "     o",
            "   oo oo",
            " rooooooor",
            "    r r",
            "",
            " o       o",
        ],
        w: 11,
    },
    Sprite {
        art: &[
            "     _",
            "   _/ \\_",
            " ‹=======› o",
            "   (o.o) /",
            "  /`=|=´´",
            " o  =|=",
            "    /^\\_",
            "    |   `",
            "   _|",
        ],
        ink: &["     o", "   oo oo", " rooooooor o", "    r r", "", " o"],
        w: 12,
    },
    // 5-6: La Catrina, feathers, flowers on the brim, a long dress.
    Sprite {
        art: &[
            "  \\\\ | //",
            " \\\\\\\\|////",
            "‹=========›",
            "   (o.o)",
            "    )=(",
            "  _/|=|\\_",
            "   /   \\",
            "  /     \\",
            " /_______\\",
        ],
        ink: &[
            "  rm y mt",
            " rrmmyttvv",
            "roooyoyooor",
            "    r r",
            "",
            "",
            "   c   c",
            "  c     c",
            " ccccccccc",
        ],
        w: 11,
    },
    Sprite {
        art: &[
            "  \\\\ | //",
            " \\\\\\\\|////",
            "‹=========›",
            "   (o.o)  *",
            "    )=(  /",
            "  _/|=|´",
            "   /   \\",
            "   \\    \\",
            "   /_____\\",
        ],
        ink: &[
            "  rm y mt",
            " rrmmyttvv",
            "roooyoyooor",
            "    r r   r",
            "",
            "",
            "   c   c",
            "   c    c",
            "   ccccccc",
        ],
        w: 11,
    },
    // 7-8: the guitarrista strumming, then stamping.
    Sprite {
        art: &[
            "    .-.",
            "   (o.o)",
            "  /`=|=´\\",
            " (@)=|===#",
            "  `´/^\\",
            "   /   \\",
            "  /     \\",
        ],
        ink: &["", "    v v", "", " www wwww"],
        w: 10,
    },
    Sprite {
        art: &[
            "    .-.",
            "   (o.o)",
            "  /`=|=´\\",
            " (@)=|===#",
            "  `´/^\\",
            "   /   `\\_",
            "  /",
        ],
        ink: &["", "    v v", "", " www wwww"],
        w: 10,
    },
];

/// Each dancer's steps, shown one per flicker: (sprite, mirrored).
const DANCES: [&[(usize, bool)]; 4] = [
    &[(0, false), (1, false), (2, false), (1, true)],
    &[(3, false), (4, false), (4, true)],
    &[(5, false), (6, false), (6, true)],
    &[(7, false), (8, false), (7, true), (8, true)],
];

/// The ink map's letters, as indices into `Muertos::ink`.
fn ink_of(c: char) -> usize {
    match c {
        'o' => 1,
        'y' => 2,
        'r' => 3,
        'm' => 4,
        't' => 5,
        'v' => 6,
        'w' => 7,
        'c' => 8,
        _ => 0,
    }
}

/// The character seen in a mirror.
fn mirror(c: char) -> char {
    match c {
        '/' => '\\',
        '\\' => '/',
        '(' => ')',
        ')' => '(',
        '‹' => '›',
        '›' => '‹',
        '`' => '´',
        '´' => '`',
        c => c,
    }
}

/// Tumbling petals, by depth (near, middle, far): face on, turning, edge on.
const PETAL: [[char; 4]; 3] = [
    ['●', '◖', '•', '◗'],
    ['•', '◦', '•', '·'],
    ['·', '˙', '·', '.'],
];
/// A whole flower head spinning down (near only).
const FLOWER: [char; 2] = ['✿', '❀'];
/// Petals lying on the ground.
const FALLEN: [char; 8] = ['•', ',', '●', '•', '°', '◦', '•', '❀'];
/// A veladora's flame as it flickers.
const FLAME: [char; 3] = ['♦', '◆', '♦'];

#[derive(Clone, Copy)]
struct Petal {
    x: f32,
    y: f32,
    vy: f32,
    /// Tumble phase (a turn per unit) and speed.
    turn: f32,
    spin: f32,
    /// Sway phase.
    sway: f32,
    /// 0 = near, 2 = far.
    depth: u8,
    /// 0 = cempasúchil, 1 = terciopelo, 2 = a whole flower.
    kind: u8,
}

#[derive(Clone, Copy, Default)]
struct Fallen {
    /// Index into `FALLEN` plus one; 0 = bare ground.
    look: u8,
    shade: u8,
    life: f32,
}

#[derive(Clone, Copy)]
struct Candle {
    x: i32,
    /// Rows of glass under the flame.
    tall: u8,
    glass: u8,
    phase: f32,
    /// Current brightness, about 0.6 to 1.
    lit: f32,
}

#[derive(Clone, Copy, Default)]
struct Dancer {
    x: i32,
    who: u8,
    step: u8,
}

pub(super) struct Muertos {
    rng: fastrand::Rng,
    fiesta: Fiesta,
    w: i32,
    h: i32,
    petals: Vec<Petal>,
    ground: Vec<Fallen>,
    /// Petals on the ground now, and how many the ground holds.
    fallen: usize,
    fallen_cap: usize,
    candles: Vec<Candle>,
    /// Marigolds standing by the candles: (column, look).
    blooms: Vec<(i32, u8)>,
    /// Candle light per cell: (strength 0-255, candle index).
    glow: Vec<(u8, u8)>,
    dancers: [Dancer; MAX_DANCERS],
    ndancers: usize,
    /// Seconds since the last strike.
    since: f32,

    /// Petal colors by depth, kind and tumble shade.
    petal_cols: [[[Color; 2]; 3]; 3],
    fallen_cols: [Color; 5],
    glass_cols: [Color; 4],
    ink: [Color; 9],
    flash_color: Color,
}

impl Muertos {
    pub fn new(seed: u64) -> Muertos {
        Muertos {
            rng: fastrand::Rng::with_seed(seed ^ SALT),
            fiesta: Fiesta::NONE,
            w: 0,
            h: 0,
            petals: Vec::new(),
            ground: Vec::new(),
            fallen: 0,
            fallen_cap: 0,
            candles: Vec::new(),
            blooms: Vec::new(),
            glow: Vec::new(),
            dancers: [Dancer::default(); MAX_DANCERS],
            ndancers: 0,
            since: AFTERGLOW,
            petal_cols: [[[Color::Reset; 2]; 3]; 3],
            fallen_cols: [Color::Reset; 5],
            glass_cols: [Color::Reset; 4],
            ink: [Color::Reset; 9],
            flash_color: Color::Reset,
        }
    }

    pub fn on(&self) -> bool {
        self.fiesta.is(Kind::Muertos)
    }

    fn level(&self) -> f32 {
        self.fiesta.level.clamp(0.0, 1.0)
    }

    /// How much more often lightning strikes: the dead want to be seen.
    pub fn storm(&self) -> f32 {
        if self.on() {
            1.0 + 4.0 * self.level()
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
        let fiesta = if fiesta.is(Kind::Muertos) {
            fiesta
        } else {
            Fiesta::NONE
        };
        if fiesta == self.fiesta && (w, h) == (self.w, self.h) {
            return false;
        }
        let came = fiesta.is(Kind::Muertos) && !self.fiesta.is(Kind::Muertos);
        self.fiesta = fiesta;
        self.w = w;
        self.h = h;
        self.layout();
        came
    }

    fn layout(&mut self) {
        self.petals.clear();
        self.ground.clear();
        self.candles.clear();
        self.blooms.clear();
        self.glow.clear();
        self.fallen = 0;
        self.fallen_cap = 0;
        self.ndancers = 0;
        if !self.on() {
            return;
        }
        let (w, h) = (self.w, self.h);
        let l = self.level();
        let area = (w * h) as f32;

        let n = ((area * 0.012 * (0.5 + 0.5 * l)).round() as usize).min(4000);
        self.petals.reserve_exact(n);
        for _ in 0..n {
            let mut p = Petal {
                x: 0.0,
                y: 0.0,
                vy: 0.0,
                turn: 0.0,
                spin: 0.0,
                sway: 0.0,
                depth: 0,
                kind: 0,
            };
            self.spawn(&mut p, true);
            self.petals.push(p);
        }

        // The path home along the ground, already laid when the rain starts.
        self.ground.resize(w as usize, Fallen::default());
        self.fallen_cap = (w as f32 * (0.5 + 0.4 * l)).round() as usize;
        for x in 0..w {
            if self.fallen < self.fallen_cap && self.rng.f32() < 0.5 + 0.4 * l {
                let left = 0.2 + self.rng.f32() * 0.8;
                self.lay(x, left);
            }
        }

        // Veladoras in little groups along the ground, with marigolds by them.
        if w >= 8 && h >= 5 {
            let n = ((w as f32 / 22.0) * (0.7 + 0.3 * l))
                .round()
                .clamp(1.0, 24.0) as i32;
            let slot = w as f32 / n as f32;
            let top = if h >= 20 {
                3
            } else if h >= 10 {
                2
            } else {
                1
            };
            for i in 0..n {
                let jitter = (self.rng.f32() - 0.5) * slot * 0.4;
                let cx = ((i as f32 + 0.5) * slot + jitter) as i32;
                let k = (1 + (self.rng.f32() * (1.2 + 2.0 * l)) as i32).min(3);
                for j in 0..k {
                    let x = cx + 2 * j - (k - 1);
                    // The middle one stands tallest.
                    let tall = if k == 3 && j != 1 { top - 1 } else { top };
                    let tall = (tall - self.rng.i32(0..2)).max(1);
                    self.candles.push(Candle {
                        x: x.clamp(0, w - 1),
                        tall: tall as u8,
                        glass: self.rng.u8(0..4),
                        phase: self.rng.f32() * 100.0,
                        lit: 1.0,
                    });
                }
                if h >= 10 {
                    for side in [-1, 1] {
                        if self.rng.f32() < 0.5 + 0.4 * l {
                            let x = cx + side * (k + 1 + self.rng.i32(0..2));
                            self.blooms.push((x, self.rng.u8(0..3)));
                        }
                    }
                }
            }
            // Their light on the rain: an ellipse twice as wide as tall.
            self.glow.resize((w * h) as usize, (0, 0));
            let (rx, ry) = (16.0f32, 8.0f32);
            for (i, cd) in self.candles.iter().enumerate() {
                let flame = h - 1 - cd.tall as i32;
                for y in (flame - ry as i32).max(0)..=(flame + 1).min(h - 1) {
                    for x in (cd.x - rx as i32).max(0)..=(cd.x + rx as i32).min(w - 1) {
                        let dx = (x - cd.x) as f32 / rx;
                        let dy = (y - flame) as f32 / ry;
                        let d = (dx * dx + dy * dy).sqrt();
                        if d >= 1.0 {
                            continue;
                        }
                        let s = ((1.0 - d).powf(1.2) * 255.0) as u8;
                        let slot = &mut self.glow[(y * w + x) as usize];
                        if s > slot.0 {
                            *slot = (s, i as u8);
                        }
                    }
                }
            }
        }
    }

    fn spawn(&mut self, p: &mut Petal, anywhere: bool) {
        let r = &mut self.rng;
        p.depth = match r.u32(0..20) {
            0..=6 => 0,
            7..=14 => 1,
            _ => 2,
        };
        let l = self.fiesta.level;
        p.kind = match r.f32() {
            f if p.depth == 0 && f < 0.08 + 0.12 * l => 2,
            f if f < 0.32 => 1,
            _ => 0,
        };
        p.vy = [6.5, 4.8, 3.2][p.depth as usize] * (0.8 + r.f32() * 0.4);
        p.spin = (1.2 + r.f32() * 2.0) * if r.bool() { 1.0 } else { -1.0 };
        p.turn = r.f32() * 4.0;
        p.sway = r.f32() * std::f32::consts::TAU;
        p.x = r.f32() * self.w as f32;
        p.y = if anywhere {
            r.f32() * (self.h as f32 + 2.0) - 2.0
        } else {
            -r.f32() * 3.0
        };
    }

    /// Lays a petal on the ground at `x` (or beside it) with `life` seconds
    /// to go, if the ground has room.
    fn lay(&mut self, x: i32, life: f32) {
        if self.fallen >= self.fallen_cap || self.w <= 0 {
            return;
        }
        for dx in [0, 1, -1] {
            let i = (x + dx).rem_euclid(self.w) as usize;
            if self.ground[i].look == 0 {
                let look = match self.rng.u32(0..40) {
                    0..=2 => FALLEN.len() - 1,
                    _ => self.rng.usize(0..FALLEN.len() - 1),
                };
                let shade = self.rng.u8(0..5);
                let life = life * (18.0 + self.rng.f32() * 30.0);
                self.ground[i] = Fallen {
                    look: look as u8 + 1,
                    shade,
                    life,
                };
                self.fallen += 1;
                return;
            }
        }
    }

    /// A big drop hits the ground at `x`: does it throw up petals? Some stay.
    pub fn splash(&mut self, x: i32) -> bool {
        if !self.on() || self.rng.f32() >= 0.15 + 0.25 * self.level() {
            return false;
        }
        if self.rng.bool() {
            let dx = self.rng.i32(-2..3);
            self.lay(x + dx, 1.0);
        }
        true
    }

    pub fn strike(&mut self) {
        if !self.on() {
            return;
        }
        self.since = 0.0;
        self.ndancers = 0;
        let (w, h) = (self.w, self.h);
        if h < 13 || w < 12 {
            return;
        }
        let want = (2.0 + 3.0 * self.level()).round() as i32;
        let n = want.min(w / 16).clamp(1, MAX_DANCERS as i32);
        let slot = w / n;
        let first = self.rng.usize(0..DANCES.len());
        for i in 0..n {
            let who = (first + i as usize) % DANCES.len();
            let x0 = i * slot;
            let room = (slot - 12).max(0);
            self.dancers[i as usize] = Dancer {
                x: x0 + self.rng.i32(0..=room),
                who: who as u8,
                step: self.rng.u8(0..4),
            };
        }
        self.ndancers = n as usize;
    }

    pub fn step(&mut self, dt: f32, wind: f32, speed: f32) {
        if !self.on() {
            return;
        }
        self.since = (self.since + dt).min(AFTERGLOW);
        let (w, ground) = (self.w as f32, (self.h - 1) as f32);
        for i in 0..self.petals.len() {
            let mut p = self.petals[i];
            let v = p.vy * speed * dt;
            p.y += v;
            p.sway += dt * 1.4;
            p.turn += p.spin * dt;
            // Petals are light: the wind carries them further than the rain.
            p.x = (p.x + wind * 1.3 * v + p.sway.cos() * 1.1 * dt).rem_euclid(w);
            if p.y >= ground + 1.0 {
                if p.depth < 2 {
                    self.lay(p.x as i32, 1.0);
                }
                self.spawn(&mut p, false);
            }
            self.petals[i] = p;
        }
        for g in self.ground.iter_mut() {
            if g.look != 0 {
                g.life -= dt;
                if g.life <= 0.0 {
                    g.look = 0;
                    self.fallen -= 1;
                }
            }
        }
        for c in self.candles.iter_mut() {
            c.phase += dt;
            let t = c.phase;
            let n = (t * 7.3).sin() * 0.5 + (t * 13.1 + 1.7).sin() * 0.3 + (t * 23.7).sin() * 0.2;
            c.lit = 0.8 + 0.2 * n;
        }
    }

    pub fn retheme(&mut self, ctx: &Ctx, flash: Color) {
        self.flash_color = flash;
        let native = ctx.theme.native;
        // Marigolds keep their orange, leaning toward the theme.
        let tint = |i: usize, c: Color| {
            if native {
                c
            } else {
                mix_oklab(c, ctx.pick(i, c), 0.4)
            }
        };
        let kinds = [
            [tint(0, CEMPA_LIGHT), tint(0, CEMPA_DARK)],
            [ctx.pick(1, ROSA), ctx.pick(1, TERCIOPELO)],
            [tint(2, CEMPA), tint(2, AMARILLO)],
        ];
        for (d, slot) in self.petal_cols.iter_mut().enumerate() {
            let k = [0.0, 0.15, 0.45][d];
            for (kind, cols) in slot.iter_mut().enumerate() {
                for (s, c) in cols.iter_mut().enumerate() {
                    *c = ctx.fade(kinds[kind][s], k);
                }
            }
        }
        self.fallen_cols = [
            tint(0, CEMPA),
            tint(0, CEMPA_LIGHT),
            ctx.fade(tint(0, CEMPA_DARK), 0.15),
            tint(2, AMARILLO),
            ctx.pick(1, TERCIOPELO),
        ];
        // Red, white, pink and purple glass.
        self.glass_cols = [
            ctx.pick(1, TERCIOPELO),
            if native { HUESO } else { ctx.theme.fg },
            ctx.pick(3, ROSA),
            ctx.pick(4, MORADO),
        ];
        self.ink = [
            if native { HUESO } else { ctx.theme.fg },
            tint(0, CEMPA),
            ctx.pick(2, AMARILLO),
            ctx.pick(1, ROSA),
            ctx.pick(3, MORADO),
            ctx.pick(4, TURQUESA),
            ctx.pick(5, LIMON),
            if native { MADERA } else { ctx.pick(6, MADERA) },
            ctx.pick(1, TERCIOPELO),
        ];
    }

    /// A raindrop's color near a veladora: it catches the candlelight.
    #[inline]
    pub fn warm(&self, x: i32, y: i32, c: Color) -> Color {
        let Some(&(s, i)) = self.glow.get((y * self.w + x) as usize) else {
            return c;
        };
        if s == 0 {
            return c;
        }
        let k = s as f32 / 255.0 * self.candles[i as usize].lit;
        c.lerp(FLAMA_EDGE, k * 0.9)
    }

    /// The color of petals thrown up by a splash `age` seconds old.
    pub fn spray(&self, age: f32) -> Color {
        self.petal_cols[0][0][(age / 0.08) as usize & 1]
    }

    /// The petals at one depth (0 = near).
    pub fn draw_petals(&self, c: &mut Canvas, depth: u8, flash: f32) {
        let w = self.w;
        for p in &self.petals {
            if p.depth != depth || p.y < 0.0 {
                continue;
            }
            let f = p.turn.rem_euclid(4.0) as usize;
            let (ch, shade) = if p.kind == 2 {
                (FLOWER[f / 2], f / 2)
            } else {
                (PETAL[depth as usize][f], f & 1)
            };
            let mut col = self.petal_cols[depth as usize][p.kind as usize][shade];
            if flash > 0.0 {
                col = col.lerp(self.flash_color, flash * 0.35);
            }
            let (x, y) = ((p.x as i32).rem_euclid(w.max(1)), p.y as i32);
            if depth == 0 {
                c.set_attr(x, y, ch, col, BOLD);
            } else {
                c.set(x, y, ch, col);
            }
        }
    }

    /// The petals lying on the ground row.
    pub fn draw_fallen(&self, ctx: &Ctx, c: &mut Canvas, flash: f32) {
        let g = self.h - 1;
        for (x, f) in self.ground.iter().enumerate() {
            if f.look == 0 {
                continue;
            }
            let mut col = self.fallen_cols[f.shade as usize % 5];
            if f.life < 4.0 {
                // Washed away, slowly.
                col = ctx.fade(col, 0.8 - 0.2 * f.life);
            }
            if flash > 0.0 {
                col = col.lerp(self.flash_color, flash * 0.3);
            }
            c.set(x as i32, g, FALLEN[f.look as usize - 1], col);
        }
    }

    /// The veladoras: a glass of color with a flame on top.
    pub fn draw_candles(&self, c: &mut Canvas) {
        let g = self.h - 1;
        for &(x, look) in &self.blooms {
            let col = if look == 2 {
                self.petal_cols[0][1][1]
            } else {
                self.petal_cols[0][2][look as usize]
            };
            c.set_attr(x, g - 1, FLOWER[look as usize & 1], col, BOLD);
        }
        for cd in &self.candles {
            let flame = g - cd.tall as i32;
            let core = FLAMA.lerp(FLAMA_EDGE, 1.0 - cd.lit);
            let f = (cd.phase * 5.0) as usize % FLAME.len();
            c.set_attr(cd.x, flame, FLAME[f], core, BOLD);
            // The glass glows brightest just under the flame.
            let glass = self.glass_cols[cd.glass as usize];
            for y in flame + 1..=g {
                let k = (0.65 - 0.2 * (y - flame - 1) as f32) * cd.lit;
                let col = glass.scale(0.75).lerp(FLAMA, k.max(0.0));
                c.set(cd.x, y, '█', col);
            }
        }
    }

    /// The dancers the last strike caught, lit by `flash` and fading after;
    /// `beat` is the flicker of the flash, one step each.
    pub fn draw_dancers(&self, ctx: &Ctx, c: &mut Canvas, flash: f32, beat: usize) {
        if self.since >= AFTERGLOW {
            return;
        }
        // The eye holds what the flash showed, then lets it go.
        let after = 0.8 * (-(self.since - 0.3).max(0.0) / 0.6).exp();
        let vis = flash.max(after);
        if vis < 0.03 {
            return;
        }
        let mut ink = self.ink;
        ink[0] = ink[0].lerp(self.flash_color, flash * 0.5);
        for col in ink.iter_mut() {
            *col = ctx.fade(*col, 1.0 - vis);
        }
        let bottom = self.h - 2;
        for d in &self.dancers[..self.ndancers] {
            let steps = DANCES[d.who as usize];
            let (si, flip) = steps[(d.step as usize + beat) % steps.len()];
            let s = &SPRITES[si];
            let top = bottom + 1 - s.art.len() as i32;
            for (row, line) in s.art.iter().enumerate() {
                let inks = s.ink.get(row).copied().unwrap_or("");
                let mut inks = inks.chars();
                for (i, ch) in line.chars().enumerate() {
                    let k = ink_of(inks.next().unwrap_or(' '));
                    if ch == ' ' {
                        continue;
                    }
                    let (x, ch) = if flip {
                        (d.x + s.w - 1 - i as i32, mirror(ch))
                    } else {
                        (d.x + i as i32, ch)
                    };
                    c.set(x, top + row as i32, ch, ink[k]);
                }
            }
        }
    }
}
