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
//!
//! At Navidad the pipes are strings of series lights: foquitos of every color
//! (rojo, verde, azul, amarillo, rosa, naranja) every few cells along a verde
//! pino cord, blinking in chase patterns that run along each pipe, in turns,
//! in odds and evens or at random. An esfera or a flor de nochebuena (budding
//! and opening) hangs at some of the joints and some hanging stretches are
//! heno. The further the posadas have climbed, the more pipes are strung, the
//! more joints are trimmed and the more heno hangs. Under a theme the colors
//! are the theme's accents closest to those hues. Again only the drawing
//! changes.
//!
//! On the Fiestas Patrias every pipe is verde, blanco or rojo, in that
//! rotation, with banderitas hanging along its level stretches in the same
//! three colors, one after another, and an escarapela (a tricolor rosette) at
//! some of the joints. The mes patrio builds up: from the first days of
//! September a few pipes carry a banderita every third cell, and by the Grito
//! every pipe is hung with them, every other cell, the escarapelas multiply
//! and stars of oro shine at some joints.

use crate::color::{Color, rgb};
use crate::fiesta::{Fiesta, Kind};
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

/// Navidad under `classic`: verde pino for the cord, rojo nochebuena, dorado,
/// plata and blanco for the esferas and flowers, and heno.
const PINO: Color = rgb(0x1F6F3A);
const NOCHEBUENA: Color = rgb(0xC8102E);
const DORADO: Color = rgb(0xF2B632);
const PLATA: Color = rgb(0xC9CCD6);
const BLANCO: Color = rgb(0xF4F1EA);
const HENO: Color = rgb(0x9AA58A);
/// The Fiestas Patrias under `classic`: verde bandera, blanco (shared above),
/// rojo and oro, for the bell and the embroidery.
const VERDE: Color = rgb(0x006847);
const ROJO: Color = rgb(0xCE1126);
const ORO: Color = rgb(0xD4A017);
/// Its palette follows the pipes' colors: verde, blanco, rojo and oro.
const P_ORO: usize = 3;
/// The foquitos: rojo, verde, azul, amarillo, rosa and naranja.
const FOQUITOS: [Color; 6] = [
    rgb(0xFF3B3B),
    rgb(0x3BD16F),
    rgb(0x3B7BFF),
    rgb(0xFFD23B),
    rgb(0xFF5FB4),
    rgb(0xFF8C1A),
];
/// Navidad's palette follows the cord: every foquito in three tiers (off,
/// glowing, lit), then the colors below.
const TIERS: usize = 3;
const N_ROJO: usize = FOQUITOS.len() * TIERS;
const N_DORADO: usize = N_ROJO + 1;
const N_PLATA: usize = N_ROJO + 2;
const N_BLANCO: usize = N_ROJO + 3;
const N_HENO: usize = N_ROJO + 4;
/// Cells between one foquito and the next along a string of lights.
const GAP: u16 = 3;
/// Steps of the chase patterns per second.
const BEATS: f32 = 4.0;

/// How today's holiday dresses the pipes.
#[derive(Clone, Copy, PartialEq)]
enum Garb {
    Plain,
    Muertos,
    Navidad,
    Patrias,
}

impl Garb {
    fn of(fiesta: Fiesta) -> Garb {
        if fiesta.is(Kind::Muertos) {
            Garb::Muertos
        } else if fiesta.is(Kind::Navidad) {
            Garb::Navidad
        } else if fiesta.is(Kind::Patrias) {
            Garb::Patrias
        } else {
            Garb::Plain
        }
    }
}

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
    /// Which turn of the rotation of colors its pipe was on.
    rot: u16,
    born: u32,
}

const EMPTY: Piece = Piece {
    ch: '\0',
    color: 0,
    way: 0,
    seq: 0,
    rot: 0,
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
    /// Counts the pipes at first and one more at every wrap around an edge:
    /// the Fiestas Patrias go round verde, blanco and rojo by it.
    rot: u16,
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

    /// The holiday, cached in `retheme`. On a holiday the palette holds
    /// `lines` colors for the pipes themselves, followed by the holiday's own.
    fiesta: Fiesta,
    garb: Garb,
    lines: usize,
    /// Mixed into the hashes that decide which joints are trimmed.
    salt: u32,
    /// Navidad's lights blink on the beat: steps taken, and the share of the
    /// next one.
    beat: u32,
    beat_acc: f32,
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
            garb: Garb::Plain,
            lines: 1,
            salt: (ctx.seed ^ (ctx.seed >> 32)) as u32 ^ 0x0211_1102,
            beat: 0,
            beat_acc: 0.0,
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
                rot: 0,
            }
        } else {
            Pipe {
                x: self.w / 2,
                y: self.h / 2,
                dir: 0,
                set,
                color,
                seq: 0,
                rot: 0,
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
                p.rot = p.rot.wrapping_add(1);
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
                rot: p.rot,
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

    /// Navidad: the piece at cell `i` as part of a string of lights. A foquito
    /// every few cells blinks along the cord (most of the pipes are strung,
    /// all of them at Nochebuena), some hanging stretches are heno and some
    /// joints are trimmed with an esfera or a nochebuena. Returns the
    /// character and its palette index (the cord, then Navidad's colors).
    fn dress_navidad(&self, i: usize, p: &Piece) -> (char, usize) {
        let (from, to) = (p.way / 4, p.way % 4);
        let level = self.fiesta.level.clamp(0.0, 1.0);
        let lines = self.lines;
        let ascii = p.ch.is_ascii();
        if from != to {
            // Hashes rather than the RNG, so ordinary days draw the same numbers.
            let h = mix(i as u32 ^ p.born.rotate_left(13) ^ self.salt);
            if h % 1024 < (1024.0 * (0.15 + 0.5 * level)) as u32 {
                if (h >> 10) % 100 < 50 {
                    let tint = [N_ROJO, N_DORADO, N_ROJO, N_PLATA, N_DORADO, N_BLANCO];
                    return (
                        if ascii { '@' } else { '◉' },
                        lines + tint[(h >> 17) as usize % tint.len()],
                    );
                }
                return self.nochebuena(p, (h >> 17).is_multiple_of(5));
            }
        } else {
            let id = mix(p.color as u32 ^ self.salt ^ 0x1224_2512);
            if id % 100 < (35.0 + 65.0 * level) as u32 && p.seq.is_multiple_of(GAP) {
                return self.foquito(p, id);
            }
            // Some hanging stretches, a few cells at a time, are heno.
            if from % 2 == 0
                && mix(id.wrapping_add((p.seq / 4) as u32)) % 100 < (8.0 + 32.0 * level) as u32
            {
                let k = (id >> 4) as usize + p.seq as usize;
                let ch = if ascii {
                    ['}', '{', ')']
                } else {
                    ['ʃ', 'ʅ', 'ƒ']
                }[k % 3];
                return (ch, lines + N_HENO);
            }
        }
        (p.ch, p.color as usize % lines)
    }

    /// Fiestas Patrias: the piece at cell `i` as part of a pipe in the colors
    /// of the flag. Level stretches hang banderitas, each the next color of
    /// the three, and some joints are an escarapela or, at the Grito, a star
    /// of oro. Returns the character and its palette index (the pipes' verde,
    /// blanco and rojo, then the same three and oro).
    fn dress_patrias(&self, i: usize, p: &Piece) -> (char, usize) {
        let (from, to) = (p.way / 4, p.way % 4);
        let level = self.fiesta.level.clamp(0.0, 1.0);
        let lines = self.lines;
        let ascii = p.ch.is_ascii();
        if from != to {
            // Hashes rather than the RNG, so ordinary days draw the same numbers.
            let h = mix(i as u32 ^ p.born.rotate_left(13) ^ self.salt);
            if h % 1024 < (1024.0 * (0.2 + 0.5 * level)) as u32 {
                if level >= 0.9 && (h >> 10) % 100 < 25 {
                    return (if ascii { '*' } else { '★' }, lines + P_ORO);
                }
                let color = (h >> 17) as usize % 3;
                return (if ascii { '@' } else { '❂' }, lines + color);
            }
        } else if from % 2 == 1 {
            // Going sideways: some pipes hang banderitas, more as the month
            // goes, every third cell and then every other.
            let gap = if level < 0.4 { 3 } else { 2 };
            let id = mix(p.color as u32 ^ (p.rot as u32) << 8 ^ self.salt ^ 0x0915_1810);
            if id % 100 < (40.0 + 60.0 * level) as u32 && p.seq.is_multiple_of(gap) {
                let color = p.rot as usize + 1 + (p.seq / gap) as usize;
                return (if ascii { 'v' } else { '▼' }, lines + color % 3);
            }
        }
        (p.ch, p.rot as usize % lines)
    }

    /// A foquito on the cord, lit, glowing or off as the pipe's pattern goes:
    /// a chase with a tail running forward, a chase running back, odds and
    /// evens, or at random.
    fn foquito(&self, p: &Piece, id: u32) -> (char, usize) {
        let n = (p.seq / GAP) as u32;
        let beat = self.beat;
        let tier = match (id >> 8) % 4 {
            0 => match n.wrapping_sub(beat) % 4 {
                0 => 2,
                3 => 1,
                _ => 0,
            },
            1 => match n.wrapping_add(beat) % 3 {
                0 => 2,
                1 => 1,
                _ => 0,
            },
            2 if n.wrapping_add(beat / 2).is_multiple_of(2) => 2,
            2 => 0,
            _ => match mix(n.wrapping_mul(0x9E37_79B1) ^ (beat / 2) ^ id) % 4 {
                0 | 1 => 2,
                2 => 1,
                _ => 0,
            },
        };
        let ch = match (p.ch.is_ascii(), tier) {
            (false, 0) => '•',
            (false, _) => '●',
            (true, 0) => '.',
            (true, 1) => 'o',
            (true, _) => 'O',
        };
        let color = ((id >> 12) + n) as usize % FOQUITOS.len();
        (ch, self.lines + color * TIERS + tier)
    }

    /// A flor de nochebuena tied at a joint, budding and opening in the first
    /// steps after its pipe laid it.
    fn nochebuena(&self, p: &Piece, white: bool) -> (char, usize) {
        let ascii = p.ch.is_ascii();
        let lines = self.lines;
        match self.tick.wrapping_sub(p.born) {
            0..=2 => (if ascii { '.' } else { '•' }, p.color as usize % lines),
            3..=7 => (if ascii { '*' } else { '❀' }, lines + N_ROJO),
            _ => (
                if ascii { '%' } else { '❋' },
                lines + if white { N_BLANCO } else { N_ROJO },
            ),
        }
    }
}

/// A small integer hash.
fn mix(mut h: u32) -> u32 {
    h = h.wrapping_mul(0x9E37_79B1);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85EB_CA77);
    h ^ (h >> 13)
}

/// The hue of a color in degrees, with its saturation and brightness.
fn hsv(c: Color) -> Option<(f32, f32, f32)> {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Ansi(n) => crate::color::ansi_rgb(n),
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

/// The accent of the theme whose hue is closest to `hue`, within `tol`
/// degrees and not one of `avoid`; the theme's accent number `nth` if none is.
fn near(ctx: &Ctx, hue: f32, tol: f32, avoid: &[Color], nth: usize) -> Color {
    let gap = |a: f32| {
        let d = (a - hue).abs() % 360.0;
        d.min(360.0 - d)
    };
    ctx.theme
        .colors
        .iter()
        .copied()
        .filter(|c| !avoid.contains(c))
        .filter_map(|c| {
            let (h, s, v) = hsv(c)?;
            (s >= 0.25 && v >= 0.3).then_some((gap(h), c))
        })
        .filter(|&(d, _)| d <= tol)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map_or_else(|| ctx.pick(nth, Color::Reset), |(_, c)| c)
}

/// The Fiestas Patrias' colors: verde, blanco, rojo and oro. Under `classic`
/// the flag's own, else the theme's accents nearest each hue (and its text
/// color for the blanco).
fn patrias_colors(ctx: &Ctx) -> [Color; 4] {
    let t = ctx.theme;
    if t.native {
        return [VERDE, BLANCO, ROJO, ORO];
    }
    let verde = near(ctx, 130.0, 55.0, &[], 2);
    let rojo = near(ctx, 355.0, 35.0, &[verde], 1);
    [verde, t.fg, rojo, near(ctx, 45.0, 25.0, &[verde, rojo], 3)]
}

/// Navidad's colors: the cord, then the foquitos in three tiers and the rest
/// (see `N_ROJO`). Under `classic` the holiday's own, else the theme's
/// accents nearest each hue.
fn navidad_colors(ctx: &Ctx) -> (Color, Vec<Color>) {
    let t = ctx.theme;
    let (pino, rojo, dorado, plata, blanco, heno, bulbs) = if t.native {
        (PINO, NOCHEBUENA, DORADO, PLATA, BLANCO, HENO, FOQUITOS)
    } else {
        let rojo = near(ctx, 355.0, 35.0, &[], 1);
        let verde = near(ctx, 130.0, 55.0, &[rojo], 2);
        let amarillo = near(ctx, 50.0, 22.0, &[rojo, verde], 3);
        let azul = near(ctx, 220.0, 45.0, &[rojo, verde, amarillo], 4);
        let rosa = near(ctx, 325.0, 35.0, &[rojo, verde, amarillo, azul], 5);
        let naranja = near(ctx, 28.0, 15.0, &[rojo, verde, amarillo, azul, rosa], 6);
        (
            verde,
            rojo,
            amarillo,
            ctx.fade(t.fg, 0.2),
            t.fg,
            verde.lerp(t.fg, 0.55),
            [rojo, verde, azul, amarillo, rosa, naranja],
        )
    };
    let mut colors = Vec::with_capacity(N_HENO + 1);
    for c in bulbs {
        colors.extend([c.scale(0.3), c.scale(0.65), c]);
    }
    colors.extend([rojo, dorado, plata, blanco, heno]);
    (pino, colors)
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
        for i in 0..n {
            let mut p = self.spawn();
            p.rot = i as u16;
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
        self.garb = Garb::of(ctx.fiesta);
        match self.garb {
            Garb::Plain => {}
            Garb::Muertos => {
                if self.colored && ctx.theme.native {
                    self.palette.clear();
                    self.palette.extend(PICADO);
                }
                self.lines = self.palette.len();
                self.palette.extend(BLOOMS);
            }
            Garb::Navidad => {
                let (cord, colors) = navidad_colors(ctx);
                if self.colored {
                    self.palette.clear();
                    self.palette.push(cord);
                }
                self.lines = self.palette.len();
                self.palette.extend(colors);
            }
            Garb::Patrias => {
                let flag = patrias_colors(ctx);
                if self.colored {
                    self.palette.clear();
                    self.palette.extend(&flag[..3]);
                }
                self.lines = self.palette.len();
                self.palette.extend(flag);
            }
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
        if self.garb == Garb::Navidad {
            self.beat_acc += dt.clamp(0.0, 1.0) * BEATS;
            let steps = self.beat_acc.floor();
            self.beat_acc -= steps;
            self.beat = self.beat.wrapping_add(steps as u32);
        }
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
        let garb = self.garb;
        let n = if garb != Garb::Plain {
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
            let (ch, ci) = match garb {
                Garb::Plain => (p.ch, p.color as usize % n),
                Garb::Muertos => self.dress(i, p, n, bloom),
                Garb::Navidad => self.dress_navidad(i, p),
                Garb::Patrias => self.dress_patrias(i, p),
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

    /// An ordinary day, Día de Muertos at two levels, and the other holidays
    /// the pipes dress for.
    fn days() -> [Fiesta; 7] {
        [
            Fiesta::NONE,
            Fiesta {
                kind: Kind::Muertos,
                level: 1.0,
                ..Fiesta::NONE
            },
            Fiesta {
                kind: Kind::Muertos,
                level: 0.25,
                ..Fiesta::NONE
            },
            Fiesta::height(Kind::Navidad),
            Fiesta::of_day(12, 16),
            Fiesta::height(Kind::Patrias),
            Fiesta::of_day(9, 2),
        ]
    }

    /// Runs the scene with `opts` through odd sizes, huge steps, themes and
    /// holidays.
    fn torture(opts: &str) {
        let table: toml::Table = opts.parse().unwrap();
        let o = crate::scene::Opts::new(INFO.options, table);
        for (theme, bg) in [("classic", Color::Reset), ("nord", Color::Rgb(20, 20, 30))] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for fiesta in days() {
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
            for (month, day) in [(11, 2), (10, 28), (6, 15)] {
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
                if ctx.fiesta.is(Kind::Muertos) {
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

    fn frame(s: &mut dyn Scene, ctx: &Ctx) -> Canvas {
        let mut c = Canvas::new(100, 30, Color::Reset);
        c.clear();
        s.draw(ctx, &mut c);
        c
    }

    fn count(c: &Canvas, set: &str) -> usize {
        c.cells().iter().filter(|x| set.contains(x.ch)).count()
    }

    /// A scene on `fiesta`, grown for `secs` seconds at 100x30.
    fn grown(
        opts: &crate::scene::Opts,
        th: &crate::theme::Theme,
        fiesta: Fiesta,
        secs: u32,
    ) -> (Box<dyn Scene>, Canvas) {
        let ctx = Ctx {
            theme: th,
            opts,
            bg: Color::Reset,
            seed: 7,
            fiesta,
        };
        let mut s = (INFO.make)(&ctx);
        s.resize(&ctx, 100, 30);
        s.retheme(&ctx);
        run(s.as_mut(), &ctx, secs);
        let c = frame(s.as_mut(), &ctx);
        (s, c)
    }

    /// Foquitos, trimmings and heno on screen at Navidad.
    fn strung(c: &Canvas) -> (usize, usize, usize) {
        (count(c, "●•"), count(c, "◉❋❀"), count(c, "ʃʅƒ"))
    }

    #[test]
    fn dresses_for_navidad() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for (month, day) in [(12, 24), (12, 16), (1, 6), (6, 15)] {
                let (_, c) = grown(&o, &th, Fiesta::of_day(month, day), 3);
                let (bulbs, trims, heno) = strung(&c);
                if month == 6 {
                    assert_eq!((bulbs, trims, heno), (0, 0, 0), "{theme}");
                } else {
                    assert!(bulbs > 40 && trims > 3 && heno > 3, "{theme} {month}-{day}");
                }
            }
        }
        // Under `classic` the cord is verde pino and the foquitos shine in
        // their own colors.
        let th = crate::theme::Theme::classic();
        let (_, c) = grown(&o, &th, Fiesta::height(Kind::Navidad), 3);
        assert!(c.cells().iter().any(|x| x.fg == PINO));
        for foquito in FOQUITOS {
            assert!(c.cells().iter().any(|x| x.fg == foquito), "{foquito:?}");
        }
    }

    #[test]
    fn the_posadas_string_more_lights() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::classic();
        let (_, first) = grown(&o, &th, Fiesta::of_day(12, 12), 4);
        let (_, last) = grown(&o, &th, Fiesta::of_day(12, 24), 4);
        let (a, b) = (strung(&first), strung(&last));
        assert!(a.0 < b.0 && a.1 < b.1 && a.2 < b.2, "{a:?} {b:?}");
    }

    #[test]
    fn the_foquitos_blink_along_the_pipes() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::classic();
        let fiesta = Fiesta::height(Kind::Navidad);
        let (mut s, before) = grown(&o, &th, fiesta, 3);
        let ctx = Ctx {
            theme: &th,
            opts: &o,
            bg: Color::Reset,
            seed: 7,
            fiesta,
        };
        run(s.as_mut(), &ctx, 1);
        let after = frame(s.as_mut(), &ctx);
        // The same bulbs, in the same places, now lit or off.
        let blinking = before
            .cells()
            .iter()
            .zip(after.cells())
            .filter(|(a, b)| "●•".contains(a.ch) && "●•".contains(b.ch) && a != b)
            .count();
        assert!(blinking > 20, "{blinking}");
    }

    /// A holiday the pipes have no costume for, and an ordinary day, draw
    /// the same.
    #[test]
    fn an_ordinary_day_is_untouched() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "nord"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let (_, plain) = grown(&o, &th, Fiesta::NONE, 5);
            for other in [Kind::Candelaria, Kind::Carnaval, Kind::Nino] {
                let (_, c) = grown(&o, &th, Fiesta::height(other), 5);
                assert!(plain.cells() == c.cells(), "{theme} {other:?}");
            }
        }
    }

    /// The pipes grow from the same random numbers on every day: only how
    /// they are drawn changes.
    #[test]
    fn the_pipes_grow_the_same_on_every_day() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::classic();
        let grown = |fiesta| {
            let ctx = Ctx {
                theme: &th,
                opts: &o,
                bg: Color::Reset,
                seed: 5,
                fiesta,
            };
            let mut p = Pipes::new(&ctx);
            p.resize(&ctx, 80, 24);
            p.retheme(&ctx);
            for _ in 0..200 {
                p.step();
            }
            p.grid
                .iter()
                .map(|x| (x.ch, x.color, x.way, x.seq, x.rot, x.born))
                .collect::<Vec<_>>()
        };
        let plain = grown(Fiesta::NONE);
        for fiesta in [
            Fiesta::height(Kind::Navidad),
            Fiesta::height(Kind::Patrias),
            Fiesta::of_day(11, 2),
        ] {
            assert!(plain == grown(fiesta), "{fiesta:?}");
        }
    }

    #[test]
    fn navidad_comes_and_goes_through_retheme() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::classic();
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
        assert_eq!(strung(&frame(s.as_mut(), &ctx)), (0, 0, 0));
        ctx.fiesta = Fiesta::height(Kind::Navidad);
        s.retheme(&ctx);
        let (bulbs, trims, heno) = strung(&frame(s.as_mut(), &ctx));
        assert!(bulbs > 40 && trims > 3 && heno > 3);
        // From a holiday to another, and back to an ordinary day.
        ctx.fiesta = Fiesta::of_day(11, 2);
        s.retheme(&ctx);
        assert_eq!(count(&frame(s.as_mut(), &ctx), "ʃʅƒ◉❋"), 0);
        ctx.fiesta = Fiesta::NONE;
        s.retheme(&ctx);
        assert_eq!(strung(&frame(s.as_mut(), &ctx)), (0, 0, 0));
        assert!(
            frame(s.as_mut(), &ctx)
                .cells()
                .iter()
                .all(|x| !x.fg.is_rgb())
        );
    }

    /// Banderitas, escarapelas and stars of oro on screen at the Fiestas
    /// Patrias.
    fn hung(c: &Canvas) -> (usize, usize, usize) {
        (count(c, "▼"), count(c, "❂"), count(c, "★"))
    }

    #[test]
    fn dresses_for_the_fiestas_patrias() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "dracula"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            for (month, day) in [(9, 15), (9, 16), (9, 5), (6, 15)] {
                let (_, c) = grown(&o, &th, Fiesta::of_day(month, day), 3);
                let (flags, rosettes, stars) = hung(&c);
                match (month, day) {
                    (6, _) => assert_eq!((flags, rosettes, stars), (0, 0, 0), "{theme}"),
                    (9, 15) => assert!(flags > 40 && rosettes > 3 && stars > 0, "{theme}"),
                    _ => assert!(flags > 10 && rosettes > 1, "{theme} {month}-{day}"),
                }
            }
        }
        // Under `classic` the banderitas and the pipes are verde, blanco and
        // rojo, and the stars oro.
        let th = crate::theme::Theme::classic();
        let (_, c) = grown(&o, &th, Fiesta::height(Kind::Patrias), 3);
        for color in [VERDE, BLANCO, ROJO, ORO] {
            assert!(c.cells().iter().any(|x| x.fg == color), "{color:?}");
        }
        for banderita in c.cells().iter().filter(|x| x.ch == '▼') {
            assert!([VERDE, BLANCO, ROJO].contains(&banderita.fg));
        }
    }

    #[test]
    fn the_pipes_go_round_verde_blanco_and_rojo() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::classic();
        let ctx = Ctx {
            theme: &th,
            opts: &o,
            bg: Color::Reset,
            seed: 5,
            fiesta: Fiesta::of_day(9, 1),
        };
        let mut p = Pipes::new(&ctx);
        p.resize(&ctx, 100, 30);
        p.retheme(&ctx);
        for _ in 0..80 {
            p.step();
        }
        let mut c = Canvas::new(100, 30, Color::Reset);
        p.draw(&ctx, &mut c);
        let mut seen = [0; 3];
        for (piece, cell) in p.grid.iter().zip(c.cells()) {
            // A plain stretch of pipe is the color of its turn of the rotation.
            if piece.ch != '\0' && cell.ch == piece.ch {
                let flag = [VERDE, BLANCO, ROJO][piece.rot as usize % 3];
                assert_eq!(cell.fg, flag);
                seen[piece.rot as usize % 3] += 1;
            }
        }
        assert!(seen.iter().all(|&n| n > 10), "{seen:?}");
    }

    #[test]
    fn the_mes_patrio_builds_up() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        let th = crate::theme::Theme::classic();
        let (_, first) = grown(&o, &th, Fiesta::of_day(9, 1), 4);
        let (_, grito) = grown(&o, &th, Fiesta::of_day(9, 15), 4);
        let (a, b) = (hung(&first), hung(&grito));
        assert!(a.0 < b.0 && a.1 < b.1 && a.2 == 0 && b.2 > 0, "{a:?} {b:?}");
    }

    #[test]
    fn the_patrias_come_and_go_through_retheme() {
        let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
        for theme in ["classic", "nord"] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
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
            assert_eq!(hung(&frame(s.as_mut(), &ctx)), (0, 0, 0));
            ctx.fiesta = Fiesta::height(Kind::Patrias);
            s.retheme(&ctx);
            let (flags, rosettes, _) = hung(&frame(s.as_mut(), &ctx));
            assert!(flags > 20 && rosettes > 2, "{theme}");
            // From one holiday to another, and back to an ordinary day.
            ctx.fiesta = Fiesta::height(Kind::Navidad);
            s.retheme(&ctx);
            assert_eq!(count(&frame(s.as_mut(), &ctx), "▼❂★"), 0);
            ctx.fiesta = Fiesta::NONE;
            s.retheme(&ctx);
            assert_eq!(hung(&frame(s.as_mut(), &ctx)), (0, 0, 0));
            if theme == "classic" {
                assert!(
                    frame(s.as_mut(), &ctx)
                        .cells()
                        .iter()
                        .all(|x| !x.fg.is_rgb())
                );
            }
        }
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
