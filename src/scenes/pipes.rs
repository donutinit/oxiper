//! pipes: pipes snake and turn until the screen is full.
//!
//! After pipes.sh (github.com/pipeseroni/pipes.sh, MIT): every step each pipe
//! moves one cell, wraps around the edges (picking a new color and piece set
//! unless `keep` is on), turns left or right with probability 1/straightness,
//! and leaves the joint piece for "came from this way, leaves that way" behind.
//!
//! On Día de Muertos the pipes are garlands: papel picado flags of every
//! color hang from the level stretches (in rosa mexicano, naranja, morado,
//! turquesa, amarillo and verde limón under `classic`, the theme's colors
//! otherwise), a marigold is tied at the joints, budding and opening as the
//! pipe goes on, and some hanging stretches become strings of cempasúchil.
//! The higher the holiday, the more joints blossom, the more strings of
//! flowers hang, and terciopelo and nube join the marigolds. It is all in how
//! the pieces are drawn: the pipes grow exactly as on any other day.

use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Canvas, DIM};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "pipes",
    aliases: &["pipes.sh"],
    about: "pipes snake and turn until the screen is full (after pipes.sh)",
    options: &[
        Opt {
            key: "count",
            default: Val::I(0),
            help: "Pipes growing at once (0 = scale with the screen)",
        },
        Opt {
            key: "straightness",
            default: Val::I(13),
            help: "Turn with probability 1/straightness at every step, like pipes.sh -s",
        },
        Opt {
            key: "style",
            default: Val::S("heavy"),
            help: "Piece set: heavy, rounded, light, double, ascii, slash, dots, beads, railway, \
                   knobby, or pipes.sh -t 0-9; 'light,double' mixes, 'all' uses every set, \
                   'c' + 16 characters is a custom set",
        },
        Opt {
            key: "limit",
            default: Val::I(-1),
            help: "Clear the screen after this many pieces, like pipes.sh -r \
                   (0 = never, -1 = about one screenful; never when fading)",
        },
        Opt {
            key: "speed",
            default: Val::F(24.0),
            help: "Segments per second for each pipe",
        },
        Opt {
            key: "colors",
            default: Val::B(true),
            help: "Give every pipe its own color (false = one color, like pipes.sh -C)",
        },
        Opt {
            key: "fade",
            default: Val::B(false),
            help: "Let old segments fade away instead of piling up",
        },
        Opt {
            key: "random_start",
            default: Val::B(true),
            help: "Start pipes at random places and directions (pipes.sh -R); \
                   false starts them all in the middle, going up",
        },
        Opt {
            key: "keep",
            default: Val::B(false),
            help: "Keep color and piece set when wrapping around an edge (pipes.sh -K)",
        },
        Opt {
            key: "bold",
            default: Val::B(true),
            help: "Draw pieces in bold like pipes.sh (false = pipes.sh -B)",
        },
    ],
    make: |ctx| Box::new(Pipes::new(ctx)),
};

/// pipes.sh's piece sets. Index `from * 4 + to` with directions
/// 0 up, 1 right, 2 down, 3 left; reversals never happen (the spaces).
const SETS: [&str; 10] = [
    "┃┏ ┓┛━┓  ┗┃┛┗ ┏━",
    "│╭ ╮╯─╮  ╰│╯╰ ╭─",
    "│┌ ┐┘─┐  └│┘└ ┌─",
    "║╔ ╗╝═╗  ╚║╝╚ ╔═",
    "|+ ++-+  +|++ +-",
    "|/ \\/-\\  \\|/\\ /-",
    ".. ....  .... ..",
    ".o oo.o  o.oo o.",
    "-\\ /\\|/  /-\\/ \\|",
    "╿┍ ┑┚╼┒  ┕╽┙┖ ┎╾",
];

const SET_NAMES: [&str; 10] = [
    "heavy", "rounded", "light", "double", "ascii", "slash", "dots", "beads", "railway", "knobby",
];

/// pipes.sh draws with `tput setaf 1..7` (+ bold); the bright half adds variety
/// while still following the user's terminal color scheme.
const CLASSIC: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15];

/// Steps of the precomputed fade ramp per color.
const FADE_STEPS: usize = 24;

/// Día de Muertos garlands under `classic`: papel picado's tissue paper
/// (rosa mexicano, naranja, morado, turquesa, amarillo, verde limón).
const PICADO: [Color; 6] = [
    rgb(0xE4007C),
    rgb(0xFF8C00),
    rgb(0x8A3CC8),
    rgb(0x00AAB8),
    rgb(0xFFD200),
    rgb(0x8CC828),
];

/// Blossoms tied at the joints, after the garland colors in the palette:
/// cempasúchil in three shades, terciopelo, nube and the green bud. They keep
/// their own hue under every theme.
const BLOOMS: [Color; 6] = [
    rgb(0xFF8C00),
    rgb(0xFFAA1E),
    rgb(0xD66200),
    rgb(0xC8105A),
    rgb(0xF4EEE2),
    rgb(0x78B42A),
];
const TERCIOPELO: usize = 3;
const NUBE: usize = 4;
const BUD: usize = 5;

/// Pieces of one color along a garland, like one paper flag after another.
const FLAG: usize = 2;

#[derive(Clone, Copy)]
struct Piece {
    /// `'\0'` marks an empty cell.
    ch: char,
    color: u8,
    /// Direction it came from times 4 plus where it went (it turned where
    /// they differ, and Día de Muertos ties a marigold there).
    way: u8,
    /// How far along its pipe this piece was laid (the garland's flags).
    seq: u16,
    born: u32,
}

const EMPTY: Piece = Piece {
    ch: '\0',
    color: 0,
    way: 0,
    seq: 0,
    born: 0,
};

#[derive(Clone, Copy)]
struct Pipe {
    x: i32,
    y: i32,
    dir: u8,
    set: u8,
    color: u8,
    /// Pieces laid so far (wrapping).
    seq: u16,
}

struct Pipes {
    rng: fastrand::Rng,
    /// Piece sets in use: pipes.sh's ten plus an optional custom one.
    sets: Vec<[char; 16]>,
    /// Indices into `sets` a pipe may pick from (pipes.sh's `-t` list).
    chosen: Vec<u8>,
    count_opt: i64,
    straight: u32,
    limit_opt: i64,
    speed: f32,
    colored: bool,
    fade: bool,
    random_start: bool,
    keep: bool,
    bold: bool,

    w: i32,
    h: i32,
    grid: Vec<Piece>,
    pipes: Vec<Pipe>,
    acc: f32,
    tick: u32,
    drawn: u64,
    limit: u64,
    /// Ticks until a piece has faded away completely.
    life: f32,

    palette: Vec<Color>,
    /// `FADE_STEPS` faded versions of every palette color (RGB palettes only).
    faded: Vec<Color>,
    rgb: bool,

    /// The holiday, cached in `retheme`. On Día de Muertos the palette holds
    /// `lines` garland colors followed by `BLOOMS`.
    fiesta: Fiesta,
    lines: usize,
    /// Mixed into the hash that decides which joints blossom.
    salt: u32,
}

/// Parses `style`: names, pipes.sh numbers, comma lists, `all`, or `c` + 16 chars.
fn parse_style(ctx: &Ctx, sets: &mut Vec<[char; 16]>) -> Vec<u8> {
    let mut chosen = Vec::new();
    match ctx.opts.table().get("style") {
        Some(toml::Value::Integer(n)) => {
            if (0..SETS.len() as i64).contains(n) {
                chosen.push(*n as u8);
            }
        }
        Some(toml::Value::Array(a)) => {
            for v in a {
                match v {
                    toml::Value::Integer(n) if (0..SETS.len() as i64).contains(n) => {
                        chosen.push(*n as u8)
                    }
                    toml::Value::String(s) => style_word(s, sets, &mut chosen),
                    _ => {}
                }
            }
        }
        _ => {
            let s = ctx.opts.s("style");
            // A custom set may itself contain commas, so try it whole first.
            if is_custom(s) {
                style_word(s, sets, &mut chosen);
            } else {
                for word in s.split(',') {
                    style_word(word, sets, &mut chosen);
                }
            }
        }
    }
    if chosen.is_empty() {
        chosen.push(0);
    }
    chosen
}

fn is_custom(s: &str) -> bool {
    s.starts_with('c') && s.chars().count() == 17
}

fn style_word(word: &str, sets: &mut Vec<[char; 16]>, chosen: &mut Vec<u8>) {
    if is_custom(word) {
        let mut set = [' '; 16];
        for (slot, ch) in set.iter_mut().zip(word.chars().skip(1)) {
            *slot = ch;
        }
        if sets.len() < 255 {
            sets.push(set);
            chosen.push((sets.len() - 1) as u8);
        }
        return;
    }
    let w = word.trim().to_ascii_lowercase();
    if w == "all" || w == "random" || w == "mix" {
        chosen.extend(0..SETS.len() as u8);
        return;
    }
    if let Ok(n) = w.parse::<usize>() {
        if n < SETS.len() {
            chosen.push(n as u8);
        }
        return;
    }
    let alias = match w.as_str() {
        "thick" | "bold" => "heavy",
        "round" | "curved" => "rounded",
        "thin" | "single" | "normal" => "light",
        "plus" => "ascii",
        "dot" => "dots",
        "o" | "bead" => "beads",
        "rail" | "railroad" | "track" => "railway",
        "knob" | "knobs" => "knobby",
        other => other,
    };
    if let Some(i) = SET_NAMES.iter().position(|n| *n == alias) {
        chosen.push(i as u8);
    }
}

impl Pipes {
    fn new(ctx: &Ctx) -> Pipes {
        let mut sets: Vec<[char; 16]> = SETS
            .iter()
            .map(|s| {
                let mut a = [' '; 16];
                for (slot, ch) in a.iter_mut().zip(s.chars()) {
                    *slot = ch;
                }
                a
            })
            .collect();
        let chosen = parse_style(ctx, &mut sets);
        let o = ctx.opts;
        Pipes {
            rng: ctx.rng(),
            sets,
            chosen,
            count_opt: o.i("count"),
            straight: o.i("straightness").clamp(1, 1_000_000) as u32,
            limit_opt: o.i("limit"),
            speed: o.f("speed").clamp(0.0, 10_000.0),
            colored: o.b("colors"),
            fade: o.b("fade"),
            random_start: o.b("random_start"),
            keep: o.b("keep"),
            bold: o.b("bold"),
            w: 0,
            h: 0,
            grid: Vec::new(),
            pipes: Vec::new(),
            acc: 0.0,
            tick: 0,
            drawn: 0,
            limit: 0,
            life: 1.0,
            palette: vec![Color::Reset],
            faded: Vec::new(),
            rgb: false,
            fiesta: Fiesta::NONE,
            lines: 1,
            salt: (ctx.seed ^ (ctx.seed >> 32)) as u32 ^ 0x0211_1102,
        }
    }

    fn pick_set(&mut self) -> u8 {
        self.chosen[self.rng.usize(..self.chosen.len())]
    }

    fn spawn(&mut self) -> Pipe {
        let set = self.pick_set();
        let color = self.rng.u8(..);
        if self.random_start {
            Pipe {
                x: self.rng.i32(0..self.w),
                y: self.rng.i32(0..self.h),
                dir: self.rng.u8(0..4),
                set,
                color,
                seq: 0,
            }
        } else {
            Pipe {
                x: self.w / 2,
                y: self.h / 2,
                dir: 0,
                set,
                color,
                seq: 0,
            }
        }
    }

    fn clear(&mut self) {
        self.grid.fill(EMPTY);
        self.drawn = 0;
    }

    /// One pipes.sh iteration: every pipe advances one cell.
    fn step(&mut self) {
        let (w, h) = (self.w, self.h);
        for i in 0..self.pipes.len() {
            let mut p = self.pipes[i];
            match p.dir {
                0 => p.y -= 1,
                1 => p.x += 1,
                2 => p.y += 1,
                _ => p.x -= 1,
            }
            if p.x < 0 || p.x >= w || p.y < 0 || p.y >= h {
                if !self.keep {
                    p.color = self.rng.u8(..);
                    p.set = self.pick_set();
                }
                p.x = p.x.rem_euclid(w);
                p.y = p.y.rem_euclid(h);
            }
            let next = if self.rng.u32(0..self.straight) == 0 {
                (p.dir + if self.rng.bool() { 1 } else { 3 }) % 4
            } else {
                p.dir
            };
            let set = &self.sets[p.set as usize % self.sets.len()];
            let ch = set[(p.dir * 4 + next) as usize];
            self.grid[(p.y * w + p.x) as usize] = Piece {
                ch,
                color: p.color,
                way: p.dir * 4 + next,
                seq: p.seq,
                born: self.tick,
            };
            p.dir = next;
            p.seq = p.seq.wrapping_add(1);
            self.pipes[i] = p;
        }
        self.tick = self.tick.wrapping_add(1);
        self.drawn += self.pipes.len() as u64;
        if self.limit > 0 && self.drawn >= self.limit {
            self.clear();
        }
    }

    /// Día de Muertos: the piece at cell `i` dressed as part of a garland.
    /// Level stretches hang papel picado flags of every color, a marigold is
    /// tied at the joints and some hanging stretches are strings of them.
    /// Returns the character and its palette index (`lines` garland colors,
    /// then `BLOOMS`).
    fn dress(&self, i: usize, p: &Piece, lines: usize, bloom: u32) -> (char, usize) {
        let (from, to) = (p.way / 4, p.way % 4);
        let level = self.fiesta.level;
        if from != to {
            // Hashes rather than the RNG, so ordinary days draw the same numbers.
            let h = mix(i as u32 ^ p.born.rotate_left(13) ^ self.salt);
            if h % 1024 < bloom {
                let roll = ((h >> 10) % 100) as f32;
                let kind = if roll < 22.0 * level {
                    TERCIOPELO
                } else if roll >= 100.0 - 10.0 * level {
                    NUBE
                } else {
                    (h >> 17) as usize % 3
                };
                return self.blossom(p, kind, lines);
            }
        } else if from % 2 == 1 {
            // Going sideways: a flag every other cell, each the next color.
            if p.seq.is_multiple_of(FLAG as u16) {
                let flag = p.color as usize + 1 + p.seq as usize / FLAG;
                return (if p.ch.is_ascii() { 'v' } else { '▼' }, flag % lines);
            }
        } else if p.seq.is_multiple_of(2)
            && (mix(p.color as u32 ^ self.salt) % 100) < (30.0 * level) as u32
        {
            // Going up or down, some pipes string marigolds of every shade.
            return self.blossom(p, (p.seq / 2) as usize % 3, lines);
        }
        (p.ch, p.color as usize % lines)
    }

    /// A flower of `kind` (an index into `BLOOMS`) that buds and opens in the
    /// first steps after its pipe laid it.
    fn blossom(&self, p: &Piece, kind: usize, lines: usize) -> (char, usize) {
        let ascii = p.ch.is_ascii();
        let (ch, k) = match self.tick.wrapping_sub(p.born) {
            0..=2 => (if ascii { '.' } else { '•' }, BUD),
            3..=7 => (if ascii { 'o' } else { '❀' }, 1),
            _ if ascii => (if kind == NUBE { '*' } else { '@' }, kind),
            _ => (['✿', '✿', '✿', '✾', '❁'][kind], kind),
        };
        (ch, lines + k)
    }
}

/// A small integer hash.
fn mix(mut h: u32) -> u32 {
    h = h.wrapping_mul(0x9E37_79B1);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85EB_CA77);
    h ^ (h >> 13)
}

impl Scene for Pipes {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.w = w.max(1) as i32;
        self.h = h.max(1) as i32;
        let area = (self.w * self.h) as i64;
        self.grid.clear();
        self.grid.resize(area as usize, EMPTY);
        let n = if self.count_opt > 0 {
            self.count_opt.min(1000)
        } else {
            (area as f32 / 600.0).round().clamp(1.0, 64.0) as i64
        } as usize;
        self.limit = match self.limit_opt {
            l if l > 0 => l as u64,
            l if l < 0 && !self.fade => area.max(64) as u64,
            _ => 0,
        };
        // Keep roughly half a screen visible when fading.
        self.life = (area as f32 * 0.5 / n as f32).max(8.0);
        self.pipes.clear();
        for _ in 0..n {
            let p = self.spawn();
            self.pipes.push(p);
        }
        self.drawn = 0;
        self.acc = 0.0;
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.palette.clear();
        if !self.colored {
            self.palette.push(if ctx.theme.native {
                Color::Reset
            } else {
                ctx.theme.fg
            });
        } else if ctx.theme.native {
            self.palette.extend(CLASSIC.iter().map(|&n| Color::Ansi(n)));
        } else {
            let n = ctx.theme.colors.len().max(1);
            self.palette
                .extend((0..n).map(|i| ctx.pick(i, Color::Reset)));
        }
        self.fiesta = ctx.fiesta;
        if self.fiesta.on() {
            if self.colored && ctx.theme.native {
                self.palette.clear();
                self.palette.extend(PICADO);
            }
            self.lines = self.palette.len();
            self.palette.extend(BLOOMS);
        }
        self.rgb = self.palette.iter().all(|c| c.is_rgb());
        self.faded.clear();
        if self.rgb {
            for &c in &self.palette {
                for s in 0..FADE_STEPS {
                    let k = s as f32 / (FADE_STEPS - 1) as f32;
                    // Ease in so pieces stay vivid for a while before dimming.
                    self.faded.push(ctx.fade(c, k * k * 0.9));
                }
            }
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        if self.pipes.is_empty() {
            return;
        }
        self.acc += dt.clamp(0.0, 1.0) * self.speed;
        let steps = self.acc.floor();
        self.acc -= steps;
        // Bound the work after a stall or with absurd speeds.
        let cap = (40_000 / self.pipes.len()).clamp(1, 2_000) as f32;
        for _ in 0..steps.min(cap) as u32 {
            self.step();
        }
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        let w = self.w;
        let fiesta = self.fiesta.on();
        let n = if fiesta {
            self.lines.max(1)
        } else {
            self.palette.len()
        };
        let base_attr = if self.bold { BOLD } else { 0 };
        // Día de Muertos: out of 1024 joints, how many blossom.
        let bloom = (1024.0 * (0.4 + 0.6 * self.fiesta.level.clamp(0.0, 1.0))) as u32;
        for (i, p) in self.grid.iter().enumerate() {
            if p.ch == '\0' {
                continue;
            }
            let (x, y) = (i as i32 % w, i as i32 / w);
            let (ch, ci) = if fiesta {
                self.dress(i, p, n, bloom)
            } else {
                (p.ch, p.color as usize % n)
            };
            if !self.fade {
                c.set_attr(x, y, ch, self.palette[ci], base_attr);
                continue;
            }
            let k = self.tick.wrapping_sub(p.born) as f32 / self.life;
            if k >= 1.0 {
                continue;
            }
            if self.rgb {
                let s = (k * FADE_STEPS as f32) as usize;
                let attr = if k < 0.25 { base_attr } else { 0 };
                c.set_attr(x, y, ch, self.faded[ci * FADE_STEPS + s], attr);
            } else {
                // Palette colors cannot be blended: age through bold, normal, dim.
                let attr = if k < 0.3 {
                    base_attr
                } else if k < 0.65 {
                    0
                } else {
                    DIM
                };
                c.set_attr(x, y, ch, self.palette[ci], attr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAYS: [Fiesta; 3] = [
        Fiesta::NONE,
        Fiesta {
            kind: crate::fiesta::Kind::Muertos,
            level: 1.0,
        },
        Fiesta {
            kind: crate::fiesta::Kind::Muertos,
            level: 0.25,
        },
    ];

    /// Runs the scene with `opts` through odd sizes, huge steps, themes and
    /// holidays.
    fn torture(opts: &str) {
        let table: toml::Table = opts.parse().unwrap();
        let o = crate::scene::Opts::new(INFO.options, table);
        for (theme, bg) in [("classic", Color::Reset), ("nord", Color::Rgb(20, 20, 30))] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for fiesta in DAYS {
                let ctx = Ctx {
                    theme: &th,
                    opts: &o,
                    bg,
                    seed: 11,
                    fiesta,
                };
                let mut s = (INFO.make)(&ctx);
                for &(w, h) in &[(1u16, 1u16), (2, 3), (500, 150), (3, 1), (1, 40), (37, 11)] {
                    s.resize(&ctx, w, h);
                    s.retheme(&ctx);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for i in 0..60 {
                        s.update(&ctx, if i % 20 == 19 { 5.0 } else { 1.0 / 30.0 });
                        c.clear();
                        s.draw(&ctx, &mut c);
                        let clean = c.cells().iter().all(|cell| cell.bg == Color::Reset);
                        assert!(clean || opts.contains("_bg = true"), "{opts}");
                    }
                }
            }
        }
    }

    /// Papel picado flags and marigolds on screen after a few seconds.
    fn dressed(s: &mut dyn Scene, ctx: &Ctx) -> (usize, usize) {
        let mut c = Canvas::new(100, 30, Color::Reset);
        c.clear();
        s.draw(ctx, &mut c);
        let count = |set: &str| c.cells().iter().filter(|x| set.contains(x.ch)).count();
        (count("▼"), count("✿✾❁"))
    }

    fn run(s: &mut dyn Scene, ctx: &Ctx, secs: u32) {
        for _ in 0..secs * 30 {
            s.update(ctx, 1.0 / 30.0);
        }
    }

    #[test]
    fn dresses_for_dia_de_muertos() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for (month, day) in [(11, 2), (10, 28), (1, 1)] {
                let ctx = Ctx {
                    theme: &th,
                    opts: &o,
                    bg: Color::Reset,
                    seed: 7,
                    fiesta: Fiesta::of_day(month, day),
                };
                let mut s = (INFO.make)(&ctx);
                s.resize(&ctx, 100, 30);
                s.retheme(&ctx);
                run(s.as_mut(), &ctx, 3);
                let (flags, flowers) = dressed(s.as_mut(), &ctx);
                if ctx.fiesta.on() {
                    assert!(flags > 20 && flowers > 5, "{theme} {month}-{day}");
                } else {
                    assert_eq!((flags, flowers), (0, 0), "{theme}");
                }
            }
        }
    }

    #[test]
    fn the_holiday_comes_and_goes_through_retheme() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::builtin("classic").unwrap();
        let mut ctx = Ctx {
            theme: &th,
            opts: &o,
            bg: Color::Reset,
            seed: 3,
            fiesta: Fiesta::NONE,
        };
        let mut s = (INFO.make)(&ctx);
        s.resize(&ctx, 100, 30);
        s.retheme(&ctx);
        run(s.as_mut(), &ctx, 3);
        assert_eq!(dressed(s.as_mut(), &ctx), (0, 0));
        ctx.fiesta = Fiesta::of_day(11, 1);
        s.retheme(&ctx);
        let (flags, flowers) = dressed(s.as_mut(), &ctx);
        assert!(flags > 20 && flowers > 5);
        // The garlands take the papel picado colors under `classic`.
        let mut c = Canvas::new(100, 30, Color::Reset);
        s.draw(&ctx, &mut c);
        assert!(c.cells().iter().any(|x| x.fg == PICADO[0]));
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert_eq!(dressed(s.as_mut(), &ctx), (0, 0));
        let mut c = Canvas::new(100, 30, Color::Reset);
        s.draw(&ctx, &mut c);
        assert!(c.cells().iter().all(|x| !x.fg.is_rgb()));
    }

    #[test]
    fn survives_options() {
        torture("");
        torture("count = 300\nspeed = 2000.0\nstraightness = 1\nstyle = 'all'\nfade = true");
        torture(
            "style = 7\nlimit = 3\nkeep = true\nrandom_start = false\ncolors = false\nbold = false",
        );
        torture("style = 'c0123456789abcdef'\nspeed = 0.0\ncount = -4\nlimit = 0");
        torture("style = ['light', 3, 'nope']\nfade = true\nlimit = 50");
    }

    #[test]
    fn sets_have_sixteen_pieces() {
        for s in SETS {
            assert_eq!(s.chars().count(), 16, "{s}");
        }
    }
}
