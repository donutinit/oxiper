//! pipes: pipes snake and turn until the screen is full.
//!
//! After pipes.sh (github.com/pipeseroni/pipes.sh, MIT): every step each pipe
//! moves one cell, wraps around the edges (picking a new color and piece set
//! unless `keep` is on), turns left or right with probability 1/straightness,
//! and leaves the joint piece for "came from this way, leaves that way" behind.

use crate::color::Color;
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

#[derive(Clone, Copy)]
struct Piece {
    /// `'\0'` marks an empty cell.
    ch: char,
    color: u8,
    born: u32,
}

const EMPTY: Piece = Piece {
    ch: '\0',
    color: 0,
    born: 0,
};

#[derive(Clone, Copy)]
struct Pipe {
    x: i32,
    y: i32,
    dir: u8,
    set: u8,
    color: u8,
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
            }
        } else {
            Pipe {
                x: self.w / 2,
                y: self.h / 2,
                dir: 0,
                set,
                color,
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
            p.dir = next;
            self.grid[(p.y * w + p.x) as usize] = Piece {
                ch,
                color: p.color,
                born: self.tick,
            };
            self.pipes[i] = p;
        }
        self.tick = self.tick.wrapping_add(1);
        self.drawn += self.pipes.len() as u64;
        if self.limit > 0 && self.drawn >= self.limit {
            self.clear();
        }
    }
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
        let n = self.palette.len();
        let base_attr = if self.bold { BOLD } else { 0 };
        for (i, p) in self.grid.iter().enumerate() {
            if p.ch == '\0' {
                continue;
            }
            let (x, y) = (i as i32 % w, i as i32 / w);
            let ci = p.color as usize % n;
            if !self.fade {
                c.set_attr(x, y, p.ch, self.palette[ci], base_attr);
                continue;
            }
            let k = self.tick.wrapping_sub(p.born) as f32 / self.life;
            if k >= 1.0 {
                continue;
            }
            if self.rgb {
                let s = (k * FADE_STEPS as f32) as usize;
                let attr = if k < 0.25 { base_attr } else { 0 };
                c.set_attr(x, y, p.ch, self.faded[ci * FADE_STEPS + s], attr);
            } else {
                // Palette colors cannot be blended: age through bold, normal, dim.
                let attr = if k < 0.3 {
                    base_attr
                } else if k < 0.65 {
                    0
                } else {
                    DIM
                };
                c.set_attr(x, y, p.ch, self.palette[ci], attr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the scene with `opts` through odd sizes, huge steps and themes.
    fn torture(opts: &str) {
        let table: toml::Table = opts.parse().unwrap();
        let o = crate::scene::Opts::new(INFO.options, table);
        for (theme, bg) in [("classic", Color::Reset), ("nord", Color::Rgb(20, 20, 30))] {
            let th = crate::theme::Theme::builtin(theme).unwrap();
            let ctx = Ctx {
                theme: &th,
                opts: &o,
                bg,
                seed: 11,
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
