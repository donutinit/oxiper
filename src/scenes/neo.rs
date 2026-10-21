//! neo: digital rain from the Matrix, a port of neo.
//!
//! Ported from neo by Stewart Reive, <https://github.com/st3w/neo>
//! (`cloud.cpp`, `droplet.cpp`, `neo.cpp`). Copyright (C) 2021 Stewart Reive.
//! neo is free software under the GNU General Public License, version 3 or
//! (at your option) any later version; this port is distributed under
//! version 3 of it only, like the rest of oxiper.
//!
//! The model is neo's: a persistent screen that droplets draw into as their
//! heads crawl down and that their tails erase behind them; droplets that die
//! early and linger; per-column speeds in async mode; a fixed map of which
//! cells glitch and which shade they get; glitches every 300-400 ms that swap
//! characters and flash the glitched cells bright, then dim. It runs in fixed
//! ticks at neo's default 60 fps, so it moves exactly like neo (including the
//! way neo's rounding makes droplets crawl about twice the nominal speed) at
//! any frame rate oxiper uses.
//!
//! On Día de Muertos the rain falls in cempasúchil, with droplets in rosa
//! mexicano (through terciopelo, golden-headed) joining in as the holiday
//! comes. Words are hidden in it and light up as it falls: the letters
//! drop down their columns and land in place, DÍA DE MUERTOS, CEMPASÚCHIL,
//! MICTLÁN, the names on an ofrenda, a word for the day. Now and then a
//! calavera de azúcar is painted by the rain, out of its own heaviest
//! glyphs: bone, flower eyes in cempasúchil or rosa with golden hearts,
//! turquesa icing, a smile of teeth. Under other themes the rain takes the
//! accents nearest in hue to those colors. It all draws from its own RNG on
//! top of the screen neo keeps, so the rain falls exactly as it does on any
//! other day.

use crate::color::{self, Color};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "neo",
    aliases: &["matrix", "cmatrix"],
    about: "digital rain from the Matrix, after neo",
    options: &[
        Opt {
            key: "charset",
            default: Val::S("auto"),
            help: "ascii, extended, english, digits, punc, binary, hex, katakana, greek, cyrillic, arabic, hebrew, devanagari, braille or runic; join with '+' (auto: extended, or only `chars` when given)",
        },
        Opt {
            key: "chars",
            default: Val::S(""),
            help: "Your own characters to rain (added to `charset` when that is set too)",
        },
        Opt {
            key: "colors",
            default: Val::S("auto"),
            help: "green, green2, green3, gold, yellow, orange, red, blue, cyan, purple, pink, pink2, rainbow, vaporwave, gray, one color, or a dark-to-bright list \"#hex,#hex,...\" (auto: green under classic, the theme ramp otherwise)",
        },
        Opt {
            key: "speed",
            default: Val::F(8.0),
            help: "Scroll speed in characters per second, as neo -S",
        },
        Opt {
            key: "density",
            default: Val::F(1.0),
            help: "Droplet density, as neo -d",
        },
        Opt {
            key: "async",
            default: Val::B(false),
            help: "Give every column its own speed, as neo -a",
        },
        Opt {
            key: "glitch",
            default: Val::B(true),
            help: "Let characters glitch (false = neo --noglitch)",
        },
        Opt {
            key: "glitch_pct",
            default: Val::F(10.0),
            help: "Percentage of screen cells that glitch, as neo -G",
        },
        Opt {
            key: "glitch_ms",
            default: Val::S("300,400"),
            help: "Milliseconds between glitches as \"min,max\", as neo -g",
        },
        Opt {
            key: "linger_ms",
            default: Val::S("1,3000"),
            help: "How long stopped droplets linger as \"min,max\" milliseconds, as neo -l",
        },
        Opt {
            key: "shading",
            default: Val::S("auto"),
            help: "random (neo -M 0) or distance from the head (-M 1); auto: random with neo's colors, distance with the theme ramp",
        },
        Opt {
            key: "bold",
            default: Val::I(1),
            help: "0 = never bold, 1 = random, 2 = always, as neo -b",
        },
        Opt {
            key: "message",
            default: Val::S(""),
            help: "A message the rain reveals in the middle of the screen, as neo -m",
        },
        Opt {
            key: "rip_pct",
            default: Val::F(33.33333),
            help: "Percentage of droplets that die early, as neo -r",
        },
        Opt {
            key: "short_pct",
            default: Val::F(50.0),
            help: "Percentage of shortened droplets, as neo --shortpct",
        },
        Opt {
            key: "max_dpc",
            default: Val::I(3),
            help: "Maximum droplets per column (1-3), as neo --maxdpc",
        },
    ],
    make: |ctx| Box::new(Neo::new(ctx)),
};

/// neo's default frame rate; the simulation always advances in steps of 1/HZ s.
const HZ: f64 = 60.0;
/// Never simulate more than this much backlog in one update (after a suspend).
const MAX_BACKLOG: f64 = 4.0;
const CHAR_POOL_SIZE: usize = 2048;
const GLITCH_POOL_SIZE: usize = 1024;

/// What neo's screen holds in one cell: a character and the color pair it was
/// drawn with (`pair == 0` means erased).
#[derive(Clone, Copy, PartialEq)]
struct Glyph {
    ch: char,
    pair: u8,
    bold: bool,
    /// Which palette: 0 the scene's, 1 Día de Muertos' second one.
    hue: u8,
}

const BLANK: Glyph = Glyph {
    ch: ' ',
    pair: 0,
    bold: false,
    hue: 0,
};

#[derive(Clone, Copy, Default)]
struct Droplet {
    alive: bool,
    head_crawling: bool,
    tail_crawling: bool,
    col: u32,
    /// Where the head is being advanced to.
    head_put: u32,
    /// Where the head was last drawn.
    head_cur: u32,
    /// Where the tail is being advanced to; `None` until the tail starts moving.
    tail_put: Option<u32>,
    /// The last line the tail erased.
    tail_cur: u32,
    /// The head stops here.
    end_line: u32,
    char_pool_idx: u32,
    length: u32,
    chars_per_sec: f32,
    /// Tick of the last advance.
    last: u64,
    /// Tick at which the head stopped.
    head_stop: Option<u64>,
    /// Seconds to stay put once the head stops.
    linger: f64,
    /// The palette it falls in (see `Glyph::hue`).
    hue: u8,
}

#[derive(Clone, Copy)]
struct ColumnStatus {
    max_speed_pct: f32,
    num_droplets: u8,
    can_spawn: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum CharLoc {
    Middle,
    Tail,
    Head,
}

#[derive(Clone, Copy, PartialEq)]
enum Shading {
    Random,
    DistanceFromHead,
}

#[derive(Clone, Copy, PartialEq)]
enum BoldMode {
    Off,
    Random,
    All,
}

struct MsgChar {
    /// (line, col), or `None` when the message runs off the screen.
    pos: Option<(u32, u32)>,
    val: char,
    draw: bool,
}

struct Neo {
    chars_per_sec: f32,
    density: f32,
    asynchronous: bool,
    glitchy: bool,
    glitch_pct: f32,
    glitch_ms: (u32, u32),
    linger_ms: (u32, u32),
    short_pct: f32,
    die_early_pct: f32,
    max_droplets_per_column: u8,
    bold_mode: BoldMode,
    shading_opt: Option<Shading>,
    shading: Shading,
    colors: String,

    char_pool: Vec<char>,
    glitch_pool: Vec<char>,
    glitch_pool_idx: usize,
    /// Color of each pair; index 0 is unused, the last entry is the head color.
    pairs: Vec<Color>,

    lines: usize,
    cols: usize,
    screen: Vec<Glyph>,
    glitch_map: Vec<bool>,
    color_pair_map: Vec<u8>,
    droplets: Vec<Droplet>,
    col_stat: Vec<ColumnStatus>,
    message: Vec<MsgChar>,
    droplets_per_sec: f32,

    tick: u64,
    backlog: f64,
    last_spawn: f64,
    last_glitch: f64,
    next_glitch: f64,
    force_draw_everything: bool,
    rng: fastrand::Rng,
    muertos: Muertos,
}

impl Neo {
    fn new(ctx: &Ctx) -> Neo {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let chars = build_chars(o.s("charset"), o.s("chars"));
        let char_pool = (0..CHAR_POOL_SIZE)
            .map(|_| chars[rng.usize(0..chars.len())])
            .collect();
        let glitch_pool = (0..GLITCH_POOL_SIZE)
            .map(|_| chars[rng.usize(0..chars.len())])
            .collect();
        let glitchy = o.b("glitch");
        let shading_opt = match o.s("shading").trim().to_ascii_lowercase().as_str() {
            "random" | "0" => Some(Shading::Random),
            "distance" | "distance_from_head" | "head" | "1" => Some(Shading::DistanceFromHead),
            _ => None,
        };
        let pct = |key: &str| (o.f(key) / 100.0).clamp(0.0, 1.0);
        Neo {
            chars_per_sec: finite(o.f("speed"), 8.0).clamp(0.01, 1_000_000.0),
            density: finite(o.f("density"), 1.0).clamp(0.01, 100.0),
            asynchronous: o.b("async"),
            glitchy,
            glitch_pct: if glitchy { pct("glitch_pct") } else { 0.0 },
            glitch_ms: parse_range(o.s("glitch_ms"), (300, 400)),
            linger_ms: parse_range(o.s("linger_ms"), (1, 3000)),
            short_pct: pct("short_pct"),
            die_early_pct: pct("rip_pct"),
            max_droplets_per_column: o.i("max_dpc").clamp(1, 3) as u8,
            bold_mode: match o.i("bold") {
                i64::MIN..=0 => BoldMode::Off,
                1 => BoldMode::Random,
                _ => BoldMode::All,
            },
            shading_opt,
            shading: shading_opt.unwrap_or(Shading::Random),
            colors: o.s("colors").to_string(),
            char_pool,
            glitch_pool,
            glitch_pool_idx: 0,
            pairs: neo_scheme("green").unwrap_or_default(),
            lines: 1,
            cols: 1,
            screen: Vec::new(),
            glitch_map: Vec::new(),
            color_pair_map: Vec::new(),
            droplets: Vec::new(),
            col_stat: Vec::new(),
            message: o
                .s("message")
                .chars()
                .filter(|&c| !odd_width(c))
                .map(|val| MsgChar {
                    pos: None,
                    val,
                    draw: false,
                })
                .collect(),
            droplets_per_sec: 0.0,
            tick: 0,
            backlog: 0.0,
            last_spawn: 0.0,
            last_glitch: 0.0,
            next_glitch: 0.0,
            force_draw_everything: false,
            rng,
            muertos: Muertos::new(ctx.seed, finite(o.f("speed"), 8.0), &chars),
        }
    }

    #[inline]
    fn now(&self) -> f64 {
        self.tick as f64 / HZ
    }

    /// The number of color pairs (the head's pair).
    #[inline]
    fn num_pairs(&self) -> usize {
        self.pairs.len().saturating_sub(1).max(1)
    }

    /// `Cloud::Reset`: a fresh, empty screen of the current size.
    fn reset(&mut self) {
        let screen_size = self.lines * self.cols;
        let num_droplets = ((1.5 * self.cols as f32).round() as usize).max(1);
        self.droplets.clear();
        self.droplets.resize(num_droplets, Droplet::default());
        self.screen.clear();
        self.screen.resize(screen_size, BLANK);
        self.glitch_map.resize(screen_size, false);
        self.color_pair_map.resize(screen_size, 1);
        self.fill_glitch_map();
        self.fill_color_map();
        self.droplets_per_sec =
            self.cols as f32 * self.density * self.chars_per_sec / self.lines as f32;
        self.col_stat.clear();
        self.col_stat.resize(
            self.cols,
            ColumnStatus {
                max_speed_pct: 1.0,
                num_droplets: 0,
                can_spawn: true,
            },
        );
        for cs in &mut self.col_stat {
            cs.max_speed_pct = if self.asynchronous {
                1.0 / 3.0 + self.rng.f32() * (2.0 / 3.0)
            } else {
                1.0
            };
        }
        self.reset_message();
        let now = self.now();
        self.last_glitch = now;
        self.next_glitch = now + self.rand_glitch_secs();
        self.last_spawn = now;
    }

    fn fill_glitch_map(&mut self) {
        let (pct, glitchy) = (self.glitch_pct, self.glitchy);
        for g in &mut self.glitch_map {
            *g = glitchy && self.rng.f32() < pct;
        }
    }

    fn fill_color_map(&mut self) {
        let n = self.num_pairs() as u8;
        let (lo, hi) = match n {
            0..=2 => (1, 1),
            3 => (2, 2),
            _ => (2, n - 2),
        };
        for p in &mut self.color_pair_map {
            *p = self.rng.u8(lo..=hi);
        }
    }

    fn rand_glitch_secs(&mut self) -> f64 {
        let (lo, hi) = self.glitch_ms;
        self.rng.u32(lo..=hi) as f64 / 1000.0
    }

    /// `Cloud::Rain`: one 1/60 s step.
    fn rain(&mut self) {
        let now = self.now();
        self.spawn_droplets(now);
        if self.force_draw_everything {
            self.screen.fill(BLANK);
        }
        let time_for_glitch = self.glitchy && now >= self.next_glitch;
        for i in 0..self.droplets.len() {
            if !self.droplets[i].alive {
                continue;
            }
            self.advance(i);
            if time_for_glitch {
                self.do_glitch(i);
            }
            self.draw_droplet(i, now);
            let d = self.droplets[i];
            if !d.alive {
                let cs = &mut self.col_stat[d.col as usize];
                cs.num_droplets = cs.num_droplets.saturating_sub(1);
                // If the droplet dies very early, the column is free again.
                if d.tail_put.is_some_and(|t| t <= self.lines as u32 / 4) {
                    cs.can_spawn = true;
                }
            }
        }
        if !self.message.is_empty() {
            self.calc_message();
            self.draw_message();
        }
        if time_for_glitch {
            self.last_glitch = now;
            self.next_glitch = now + self.rand_glitch_secs();
        }
        self.force_draw_everything = false;
    }

    fn spawn_droplets(&mut self, now: f64) {
        let elapsed = now - self.last_spawn;
        let to_spawn = ((elapsed * self.droplets_per_sec as f64) as usize).min(self.droplets.len());
        if to_spawn == 0 {
            return;
        }
        let mut idx = 0;
        let mut spawned = 0;
        for _ in 0..to_spawn {
            let col = self.rng.usize(0..self.cols);
            let cs = self.col_stat[col];
            if !cs.can_spawn || cs.num_droplets >= self.max_droplets_per_column {
                continue;
            }
            while idx < self.droplets.len() && self.droplets[idx].alive {
                idx += 1;
            }
            if idx >= self.droplets.len() {
                break;
            }
            self.fill_droplet(idx, col);
            let cs = &mut self.col_stat[col];
            cs.can_spawn = false;
            cs.num_droplets += 1;
            spawned += 1;
        }
        if spawned > 0 {
            self.last_spawn = now;
        }
    }

    /// `Cloud::FillDroplet` + `Droplet::Activate`.
    fn fill_droplet(&mut self, idx: usize, col: usize) {
        let lines = self.lines as u32;
        let mut end_line = lines - 1;
        if self.rng.f32() < self.die_early_pct {
            end_line = self.rng.u32(0..=lines.saturating_sub(2));
        }
        let char_pool_idx = self.rng.u32(0..CHAR_POOL_SIZE as u32);
        let mut length = lines;
        if self.rng.f32() < self.short_pct {
            length = self.rng.u32(1..=lines.saturating_sub(2).max(1));
        }
        let mut linger = 0.001;
        if end_line <= length {
            let (lo, hi) = self.linger_ms;
            linger = self.rng.u32(lo..=hi) as f64 / 1000.0;
        }
        let hue = self.muertos.hue();
        self.droplets[idx] = Droplet {
            alive: true,
            head_crawling: true,
            tail_crawling: true,
            col: col as u32,
            head_put: 0,
            head_cur: 0,
            tail_put: None,
            tail_cur: 0,
            end_line,
            char_pool_idx,
            length,
            chars_per_sec: self.col_stat[col].max_speed_pct * self.chars_per_sec,
            last: self.tick,
            head_stop: None,
            linger,
            hue,
        };
    }

    /// `Droplet::Advance`.
    fn advance(&mut self, i: usize) {
        let tick = self.tick;
        let thresh = self.lines as u32 / 4;
        let d = &mut self.droplets[i];
        let elapsed = (tick - d.last) as f32 / HZ as f32;
        let chars = (d.chars_per_sec * elapsed).round();
        if !chars.is_finite() || chars < 1.0 {
            return;
        }
        let chars = chars.min(1e6) as u32;

        if d.head_crawling {
            d.head_put = d.head_put.saturating_add(chars).min(d.end_line);
            // When the head reaches its end line it stops, and so does the tail for a while.
            if d.head_put == d.end_line {
                d.head_crawling = false;
                if d.head_stop.is_none() {
                    d.head_stop = Some(tick);
                    if d.linger > 0.0 {
                        d.tail_crawling = false;
                    }
                }
            }
        }

        if d.tail_crawling && (d.head_put >= d.length || d.head_put >= d.end_line) {
            let t = match d.tail_put {
                Some(t) => t.saturating_add(chars),
                None => chars,
            }
            .min(d.end_line);
            d.tail_put = Some(t);
            // Once the tail is far enough down, other droplets may spawn in this column.
            if d.tail_cur <= thresh && t > thresh {
                self.col_stat[d.col as usize].can_spawn = true;
            }
        }

        if !d.tail_crawling
            && d.head_stop
                .is_some_and(|hs| (tick - hs) as f64 / HZ >= d.linger)
        {
            d.tail_crawling = true;
        }
        if d.tail_put == Some(d.head_put) {
            d.alive = false;
        }
        d.last = tick;
    }

    /// `Cloud::DoGlitch`: swaps the characters of this droplet's glitched cells.
    fn do_glitch(&mut self, i: usize) {
        let d = self.droplets[i];
        let start = d.tail_put.map_or(0, |t| t + 1);
        for line in start..=d.head_put {
            if self.glitch_map[line as usize * self.cols + d.col as usize] {
                let idx = (d.char_pool_idx + line) as usize % CHAR_POOL_SIZE;
                self.char_pool[idx] = self.glitch_pool[self.glitch_pool_idx];
                self.glitch_pool_idx = (self.glitch_pool_idx + 1) % GLITCH_POOL_SIZE;
            }
        }
    }

    /// `Droplet::Draw`: erases behind the tail, then draws what changed.
    fn draw_droplet(&mut self, i: usize, now: f64) {
        let d = self.droplets[i];
        let (cols, col) = (self.cols, d.col as usize);
        let mut start = 0;
        if let Some(tp) = d.tail_put {
            for line in d.tail_cur..=tp {
                self.screen[line as usize * cols + col] = BLANK;
            }
            self.droplets[i].tail_cur = tp;
            start = tp + 1;
        }
        let head_bright = d.head_crawling
            || d.head_stop
                .is_some_and(|hs| (self.tick - hs) as f64 / HZ <= 0.1);
        for line in start..=d.head_put {
            let idx = line as usize * cols + col;
            let glitched = self.glitchy && self.glitch_map[idx];
            let val = self.char_pool[(d.char_pool_idx + line) as usize % CHAR_POOL_SIZE];
            let mut loc = CharLoc::Middle;
            if d.tail_put.is_some_and(|tp| line == tp + 1) {
                loc = CharLoc::Tail;
            }
            if line == d.head_put && head_bright {
                loc = CharLoc::Head;
            }
            // Unchanged characters between the tail and the old head stay as drawn.
            if loc == CharLoc::Middle
                && line < d.head_cur
                && !glitched
                && line != d.end_line
                && self.shading != Shading::DistanceFromHead
                && !self.force_draw_everything
            {
                continue;
            }
            let (pair, bold) = self.attr(idx, line, val, loc, now, d.head_put, d.length);
            self.screen[idx] = Glyph {
                ch: val,
                pair,
                bold,
                hue: d.hue,
            };
        }
        self.droplets[i].head_cur = d.head_put;
    }

    /// `Cloud::GetAttr`: the color pair and boldness of one character.
    #[allow(clippy::too_many_arguments)]
    fn attr(
        &self,
        idx: usize,
        line: u32,
        val: char,
        loc: CharLoc,
        now: f64,
        head_put: u32,
        length: u32,
    ) -> (u8, bool) {
        let n = self.num_pairs() as i32;
        let mut bold = self.bold_mode == BoldMode::Random && (line ^ val as u32) % 2 == 1;
        let mut pair = self.color_pair_map[idx] as i32;
        if self.shading == Shading::DistanceFromHead {
            let dist = (head_put - line) as f32 / length.max(1) as f32;
            pair = n - (dist * (n - 1) as f32).round() as i32;
        }
        if self.glitchy && self.glitch_map[idx] {
            if self.is_bright(now) {
                pair += 1;
                bold = true;
            } else if self.is_dim(now) {
                pair -= 1;
                bold = false;
            }
        }
        match loc {
            CharLoc::Tail => {
                pair = 1;
                bold = false;
            }
            CharLoc::Head => {
                pair = n;
                bold = true;
            }
            CharLoc::Middle => pair = pair.min(n - 1).max(1),
        }
        match self.bold_mode {
            BoldMode::Off => bold = false,
            BoldMode::All => bold = true,
            BoldMode::Random => {}
        }
        (pair.clamp(1, n) as u8, bold)
    }

    /// The first quarter of the time between two glitches: glitched cells flash bright.
    fn is_bright(&self, now: f64) -> bool {
        let span = self.next_glitch - self.last_glitch;
        now >= self.last_glitch && span > 0.0 && (now - self.last_glitch) / span <= 0.25
    }

    /// The last quarter: glitched cells dim.
    fn is_dim(&self, now: f64) -> bool {
        let span = self.next_glitch - self.last_glitch;
        now > self.next_glitch || (span > 0.0 && (now - self.last_glitch) / span >= 0.75)
    }

    /// `Cloud::ResetMessage`: centers the message between the first and last
    /// quarter of the screen, wrapping it over several lines if needed.
    fn reset_message(&mut self) {
        if self.message.is_empty() {
            return;
        }
        let (lines, cols) = (self.lines as u32, self.cols as u32);
        let first_col = cols / 4;
        let last_col = 3 * cols / 4;
        let chars_per_col = last_col - first_col + 1;
        let msg_lines = self.message.len() as u32 / chars_per_col + 1;
        let first_line = (lines / 2).saturating_sub(msg_lines / 2);
        let mut remaining = self.message.len() as u32;
        let (mut line, mut col) = (first_line, first_col);
        if remaining < chars_per_col {
            col += (chars_per_col - remaining) / 2;
        }
        for m in &mut self.message {
            m.draw = false;
            m.pos = (line < lines).then_some((line, col));
            if col == last_col {
                line += 1;
                col = first_col;
                if remaining < chars_per_col {
                    col += (chars_per_col - remaining) / 2;
                }
            } else {
                col += 1;
            }
            remaining -= 1;
        }
    }

    /// `Cloud::CalcMessage`: message characters appear once rain has touched them.
    fn calc_message(&mut self) {
        for m in &mut self.message {
            let Some((line, col)) = m.pos else { break };
            let g = self.screen[line as usize * self.cols + col as usize];
            if g.pair != 0 && g.ch != ' ' {
                m.draw = true;
            }
        }
    }

    fn draw_message(&mut self) {
        let pair = self.num_pairs() as u8;
        let bold = self.bold_mode != BoldMode::Off;
        for m in &self.message {
            if let (true, Some((line, col))) = (m.draw, m.pos) {
                self.screen[line as usize * self.cols + col as usize] = Glyph {
                    ch: m.val,
                    pair,
                    bold,
                    hue: 0,
                };
            }
        }
    }
}

impl Scene for Neo {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.cols = w.max(1) as usize;
        self.lines = h.max(1) as usize;
        self.reset();
        self.muertos.figs.clear();
    }

    fn retheme(&mut self, ctx: &Ctx) {
        let (pairs, native) = resolve_colors(ctx, &self.colors);
        self.pairs = pairs;
        let auto = matches!(self.colors.trim(), "" | "auto");
        if let Some(pairs) = self.muertos.retheme(ctx, auto, native) {
            self.pairs = pairs;
        }
        self.shading = self.shading_opt.unwrap_or(if native {
            Shading::Random
        } else {
            Shading::DistanceFromHead
        });
        // Like neo's SetColor: new random shades, then redraw everything.
        self.fill_color_map();
        self.force_draw_everything = true;
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        if dt.is_nan() || dt <= 0.0 {
            return;
        }
        self.backlog = (self.backlog + dt as f64).min(MAX_BACKLOG);
        while self.backlog >= 1.0 / HZ {
            self.backlog -= 1.0 / HZ;
            self.tick += 1;
            self.rain();
        }
    }

    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        if self.muertos.on {
            let head = self.num_pairs() as u8;
            let (w, h) = (self.cols, self.lines);
            self.muertos.tick(dt, real, &self.screen, w, h, head);
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let n = self.num_pairs();
        // Día de Muertos' second palette, when the rain has one.
        let alt = &self.muertos.alt;
        let alt = if alt.len() == self.pairs.len() {
            alt
        } else {
            &self.pairs
        };
        for (y, row) in self.screen.chunks_exact(self.cols).enumerate() {
            for (x, g) in row.iter().enumerate() {
                if g.pair == 0 || g.ch == ' ' {
                    continue;
                }
                let pairs = if g.hue == 0 { &self.pairs } else { alt };
                let fg = pairs[(g.pair as usize).min(n)];
                let attr = if g.bold { BOLD } else { 0 };
                c.set_attr(x as i32, y as i32, g.ch, fg, attr);
            }
        }
        if self.muertos.on {
            self.muertos.draw(ctx, c, &self.char_pool);
        }
    }
}

fn finite(v: f32, fallback: f32) -> f32 {
    if v.is_finite() { v } else { fallback }
}

/// Parses neo's `NUM1,NUM2` millisecond ranges.
fn parse_range(s: &str, fallback: (u32, u32)) -> (u32, u32) {
    let mut it = s.split(',').map(|p| p.trim().parse::<u32>());
    match (it.next(), it.next(), it.next()) {
        (Some(Ok(lo)), Some(Ok(hi)), None) if lo > 0 && lo <= hi && hi <= 0xFFFF => (lo, hi),
        (Some(Ok(v)), None, None) if v > 0 && v <= 0xFFFF => (v, v),
        _ => fallback,
    }
}

// ---------------------------------------------------------------------------
// Characters

/// neo's character sets as Unicode ranges. Combining marks, unassigned code
/// points and other zero-width characters that neo's raw ranges contain are
/// left out, since they would break the column grid.
fn charset_ranges(name: &str) -> Option<&'static [(u32, u32)]> {
    const ENGLISH: &[(u32, u32)] = &[(0x41, 0x5A), (0x61, 0x7A)];
    const DIGITS: &[(u32, u32)] = &[(0x30, 0x39)];
    const PUNC: &[(u32, u32)] = &[(0x21, 0x2F), (0x3A, 0x40), (0x5B, 0x60), (0x7B, 0x7E)];
    // Half-width katakana letters ｦ..ﾝ (neo also has the comma, middle dot and sound marks).
    const KATAKANA: &[(u32, u32)] = &[(0xFF66, 0xFF9D)];
    const ASCII: &[(u32, u32)] = &[
        (0x41, 0x5A),
        (0x61, 0x7A),
        (0x30, 0x39),
        (0x21, 0x2F),
        (0x3A, 0x40),
        (0x5B, 0x60),
        (0x7B, 0x7E),
    ];
    const EXTENDED: &[(u32, u32)] = &[
        (0x30, 0x39),
        (0x21, 0x2F),
        (0x3A, 0x40),
        (0x5B, 0x60),
        (0x7B, 0x7E),
        (0xFF66, 0xFF9D),
    ];
    Some(match name {
        "ascii" | "default" => ASCII,
        "extended" => EXTENDED,
        "english" | "letters" => ENGLISH,
        "digits" | "dec" | "decimal" => DIGITS,
        "punc" | "punctuation" => PUNC,
        "bin" | "binary" => &[(0x30, 0x31)],
        "hex" | "hexadecimal" => &[(0x30, 0x39), (0x41, 0x46)],
        "katakana" => KATAKANA,
        // U+0370-03FF without its unassigned code points.
        "greek" => &[
            (0x370, 0x377),
            (0x37A, 0x37F),
            (0x384, 0x38A),
            (0x38C, 0x38C),
            (0x38E, 0x3A1),
            (0x3A3, 0x3FF),
        ],
        "cyrillic" => &[(0x410, 0x44F)],
        "arabic" => &[(0x627, 0x649)],
        // Letters and spacing punctuation of U+0590-05FF and U+FB1D-FB4F (no points or cantillation).
        "hebrew" => &[
            (0x5BE, 0x5BE),
            (0x5C0, 0x5C0),
            (0x5C3, 0x5C3),
            (0x5C6, 0x5C6),
            (0x5D0, 0x5EA),
            (0x5EF, 0x5F4),
            (0xFB1D, 0xFB1D),
            (0xFB1F, 0xFB36),
            (0xFB38, 0xFB3C),
            (0xFB3E, 0xFB3E),
            (0xFB40, 0xFB41),
            (0xFB43, 0xFB44),
            (0xFB46, 0xFB4F),
        ],
        // Letters, digits and signs of U+0900-097F (no vowel signs or viramas).
        "devanagari" => &[
            (0x904, 0x939),
            (0x93D, 0x93D),
            (0x950, 0x950),
            (0x958, 0x961),
            (0x964, 0x97F),
        ],
        "braille" => &[(0x2800, 0x28FF)],
        "runic" => &[(0x16A0, 0x16F8)],
        _ => return None,
    })
}

/// The characters to rain: the charset(s) plus the user's own, like neo's `InitChars`.
fn build_chars(charset: &str, custom: &str) -> Vec<char> {
    let custom: Vec<char> = custom.chars().filter(|&c| !odd_width(c)).collect();
    let mut chars = Vec::new();
    let charset = charset.trim().to_ascii_lowercase();
    let auto = matches!(charset.as_str(), "" | "auto");
    if !auto {
        for name in charset.split(['+', ',', ' ']).filter(|n| !n.is_empty()) {
            for &(a, b) in charset_ranges(name).unwrap_or(&[]) {
                chars.extend((a..=b).filter_map(char::from_u32));
            }
        }
    }
    chars.extend_from_slice(&custom);
    if chars.is_empty() {
        for &(a, b) in charset_ranges("extended").unwrap_or(&[]) {
            chars.extend((a..=b).filter_map(char::from_u32));
        }
    }
    chars
}

/// Characters that do not take exactly one cell: control characters, common
/// combining marks and zero-width characters, and East Asian wide characters
/// and emoji (a best-effort table; enough to keep user input from breaking the grid).
fn odd_width(c: char) -> bool {
    const RANGES: &[(u32, u32)] = &[
        (0x00, 0x1F),
        (0x7F, 0x9F),
        (0x300, 0x36F),
        (0x483, 0x489),
        (0x591, 0x5BD),
        (0x5BF, 0x5BF),
        (0x5C1, 0x5C2),
        (0x5C4, 0x5C5),
        (0x5C7, 0x5C7),
        (0x610, 0x61A),
        (0x64B, 0x65F),
        (0x670, 0x670),
        (0x6D6, 0x6ED),
        (0x900, 0x903),
        (0x93A, 0x93C),
        (0x93E, 0x94F),
        (0x951, 0x957),
        (0x962, 0x963),
        (0x1100, 0x115F),
        (0x1AB0, 0x1AFF),
        (0x1DC0, 0x1DFF),
        (0x200B, 0x200F),
        (0x2028, 0x202E),
        (0x2060, 0x206F),
        (0x20D0, 0x20FF),
        (0x231A, 0x231B),
        (0x2329, 0x232A),
        (0x23E9, 0x23EC),
        (0x23F0, 0x23F0),
        (0x23F3, 0x23F3),
        (0x25FD, 0x25FE),
        (0x2614, 0x2615),
        (0x2648, 0x2653),
        (0x267F, 0x267F),
        (0x2693, 0x2693),
        (0x26A1, 0x26A1),
        (0x26AA, 0x26AB),
        (0x26BD, 0x26BE),
        (0x26C4, 0x26C5),
        (0x26CE, 0x26CE),
        (0x26D4, 0x26D4),
        (0x26EA, 0x26EA),
        (0x26F2, 0x26F5),
        (0x26FA, 0x26FD),
        (0x2705, 0x2705),
        (0x270A, 0x270B),
        (0x2728, 0x2728),
        (0x274C, 0x274E),
        (0x2753, 0x2755),
        (0x2757, 0x2757),
        (0x2795, 0x2797),
        (0x27B0, 0x27BF),
        (0x2B1B, 0x2B1C),
        (0x2B50, 0x2B55),
        (0x2E80, 0x303E),
        (0x3041, 0x33FF),
        (0x3400, 0x4DBF),
        (0x4E00, 0x9FFF),
        (0xA000, 0xA4CF),
        (0xA960, 0xA97F),
        (0xAC00, 0xD7A3),
        (0xF900, 0xFAFF),
        (0xFE00, 0xFE0F),
        (0xFE10, 0xFE19),
        (0xFE20, 0xFE2F),
        (0xFE30, 0xFE6F),
        (0xFEFF, 0xFEFF),
        (0xFF00, 0xFF60),
        (0xFFE0, 0xFFE6),
        (0x1F000, 0x1F02F),
        (0x1F0A0, 0x1F0FF),
        (0x1F18E, 0x1F18E),
        (0x1F191, 0x1F19A),
        (0x1F200, 0x1F2FF),
        (0x1F300, 0x1F64F),
        (0x1F680, 0x1F6FF),
        (0x1F7E0, 0x1F7EB),
        (0x1F900, 0x1F9FF),
        (0x1FA70, 0x1FAFF),
        (0x20000, 0x3FFFD),
        (0xE0000, 0xE0FFF),
    ];
    let c = c as u32;
    RANGES.iter().any(|&(a, b)| (a..=b).contains(&c))
}

// ---------------------------------------------------------------------------
// Colors

/// A palette entry as neo defines it: an xterm-256 index, or the ncurses
/// `init_color` value (0-1000 per channel) neo assigns in truecolor mode.
#[derive(Clone, Copy)]
enum C {
    X(u8),
    N(u32, u32, u32),
}

use C::{N, X};

/// neo's color schemes, pair 1 (darkest, the tail end) to the last (the head).
const SCHEMES: &[(&str, &[C])] = &[
    (
        "green",
        &[
            N(71, 141, 83),
            N(149, 243, 161),
            N(188, 596, 318),
            N(188, 714, 397),
            N(227, 925, 561),
            N(271, 973, 667),
            N(667, 1000, 941),
        ],
    ),
    (
        "green2",
        &[
            N(16, 180, 59),
            N(59, 246, 117),
            N(46, 512, 172),
            N(262, 749, 332),
            N(520, 945, 578),
            N(676, 969, 758),
            N(906, 1000, 898),
        ],
    ),
    (
        "green3",
        &[
            N(0, 373, 0),
            N(0, 529, 0),
            N(0, 686, 0),
            N(373, 686, 0),
            N(373, 843, 0),
            N(373, 1000, 0),
            N(686, 1000, 686),
        ],
    ),
    (
        "gold",
        &[
            N(839, 545, 216),
            N(905, 694, 447),
            N(945, 831, 635),
            N(1000, 922, 565),
            N(1000, 953, 796),
            N(976, 976, 968),
            X(231),
        ],
    ),
    (
        "yellow",
        &[X(100), X(142), X(184), X(226), X(227), X(229), X(230)],
    ),
    (
        "orange",
        &[X(52), X(94), X(130), X(166), X(202), X(208), X(231)],
    ),
    (
        "red",
        &[X(234), X(52), X(88), X(124), X(160), X(196), X(217)],
    ),
    // neo sets pair 4 twice (19, then 20); the second one wins.
    ("blue", &[X(234), X(17), X(18), X(20), X(21), X(75), X(159)]),
    ("cyan", &[X(24), X(25), X(31), X(32), X(38), X(45), X(159)]),
    (
        "purple",
        &[X(60), X(61), X(62), X(63), X(69), X(111), X(225)],
    ),
    (
        "pink",
        &[X(133), X(139), X(176), X(212), X(218), X(224), X(231)],
    ),
    (
        "pink2",
        &[X(145), X(181), X(217), X(218), X(224), X(225), X(231)],
    ),
    (
        "rainbow",
        &[X(196), X(208), X(226), X(46), X(21), X(93), X(201)],
    ),
    (
        "vaporwave",
        &[
            X(53),
            X(54),
            X(55),
            X(134),
            X(177),
            X(219),
            X(214),
            X(220),
            X(227),
            X(229),
            X(87),
            X(123),
            X(159),
            X(195),
            X(231),
        ],
    ),
    (
        "gray",
        &[
            X(234),
            X(237),
            X(240),
            X(243),
            X(246),
            X(249),
            X(251),
            X(252),
            X(231),
        ],
    ),
];

fn to_color(c: C) -> Color {
    match c {
        X(n) => {
            let (r, g, b) = color::ansi_rgb(n);
            Color::Rgb(r, g, b)
        }
        // What xterm-like terminals do with ncurses' init_color values.
        N(r, g, b) => Color::Rgb(
            (r * 255 / 1000) as u8,
            (g * 255 / 1000) as u8,
            (b * 255 / 1000) as u8,
        ),
    }
}

/// One of neo's schemes as a pair table (index 0 unused).
fn neo_scheme(name: &str) -> Option<Vec<Color>> {
    let name = if name == "grey" { "gray" } else { name };
    let (_, scheme) = SCHEMES.iter().find(|(n, _)| *n == name)?;
    Some(
        std::iter::once(Color::Reset)
            .chain(scheme.iter().map(|&c| to_color(c)))
            .collect(),
    )
}

/// The pair table for the `colors` option, and whether it is one of neo's own
/// palettes (which neo shades randomly) rather than a smooth ramp.
fn resolve_colors(ctx: &Ctx, colors: &str) -> (Vec<Color>, bool) {
    let s = colors.trim().to_ascii_lowercase();
    if let Some(p) = neo_scheme(&s) {
        return (p, true);
    }
    // A dark-to-bright list of colors, like a neo color file.
    if s.contains(',') {
        let list: Option<Vec<Color>> = s.split(',').map(Color::parse).collect();
        if let Some(list) = list.filter(|l| l.len() >= 2 && !l.contains(&Color::Reset)) {
            return (std::iter::once(Color::Reset).chain(list).collect(), true);
        }
    }
    // One color: shades of it, fading into the background, with a near-white head.
    if let Some(c) = Color::parse(&s).filter(|c| c.is_rgb()) {
        let pairs = vec![
            Color::Reset,
            ctx.fade(c, 0.8),
            ctx.fade(c, 0.6),
            ctx.fade(c, 0.4),
            ctx.fade(c, 0.2),
            c,
            c.lerp(WHITE, 0.3),
            c.lerp(WHITE, 0.75),
        ];
        return (pairs, true);
    }
    if ctx.theme.native {
        return (neo_scheme("green").unwrap_or_default(), true);
    }
    // The theme ramp: the tail end fades into the background, the head is the
    // brightest ramp color pushed toward white.
    const LEVELS: usize = 12;
    let g = &ctx.theme.grad;
    let mut pairs = Vec::with_capacity(LEVELS + 1);
    pairs.push(Color::Reset);
    for i in 1..LEVELS {
        let t = (i - 1) as f32 / (LEVELS - 2) as f32;
        pairs.push(ctx.fade(g.at(0.1 + 0.85 * t), 0.7 * (1.0 - t) * (1.0 - t)));
    }
    let top = g.at(1.0);
    pairs.push(if top.is_rgb() {
        top.lerp(WHITE, 0.45)
    } else {
        top
    });
    (pairs, false)
}

// ---------------------------------------------------------------------------
// Día de Muertos

/// Mixed into the seed for the holiday's own random numbers, so the rain
/// itself falls exactly as it does on any other day.
const MUERTOS_SEED: u64 = 0x000D_1ADE_0001_1102;
/// At most this many words and calaveras hidden in the rain at once.
const MAX_FIGS: usize = 6;
/// Cells per figure (the biggest calavera; words are shorter).
const SLAB: usize = 25 * 14;
/// Seconds a figure takes to dim and crumble back into the rain.
const FADE: f64 = 2.0;

/// Words the rain carries.
const WORDS: &[&str] = &[
    "DÍA DE MUERTOS",
    "CEMPASÚCHIL",
    "MICTLÁN",
    "OFRENDA",
    "CALAVERITA",
    "LA CATRINA",
    "PAN DE MUERTO",
    "PAPEL PICADO",
    "VELADORAS",
    "COPAL",
    "ALEBRIJE",
    "XOLOITZCUINTLE",
    "TERCIOPELO",
    "JANITZIO",
    "MONARCAS",
    "BIENVENIDOS A CASA",
    "AQUÍ LOS RECORDAMOS",
];

/// Names written on an ofrenda.
const NAMES: &[&str] = &[
    "ABUELITA LUPE",
    "DON CHUY",
    "TÍA CHOLE",
    "ABUELO NACHO",
    "DOÑA ROSARIO",
    "TÍO BETO",
    "MAMÁ TOÑA",
    "PAPÁ GOYO",
    "MADRINA LUCHA",
    "PRIMO PEPE",
    "DOÑA CUCA",
    "TATA CHENTE",
];

/// The words of the day, by `fiesta.level`.
fn day_word(level: f32) -> Option<&'static str> {
    Some(match (level * 100.0).round() as u32 {
        45 => "ÁNIMAS SOLAS",
        50 => "HASTA EL AÑO QUE ENTRA",
        60 => "ANGELITOS",
        80 => "TODOS SANTOS",
        100 => "FIELES DIFUNTOS",
        _ => return None,
    })
}

/// Calaveras de azúcar the rain paints, small to big: `#` bone, `o` the
/// petals of the flower eyes, `*` their hearts, `i` icing, `.` a hole.
const SKULLS: &[&[&str]] = &[
    &[
        "   #######   ",
        " ####iii#### ",
        "#ooo##i##ooo#",
        "o.*.o###o.*.o",
        "#ooo##.##ooo#",
        " ####...#### ",
        "  #.#.#.#.#  ",
        "   #######   ",
    ],
    &[
        "     #########     ",
        "   #############   ",
        "  ######iii######  ",
        " #######i*i####### ",
        "###ooooo###ooooo###",
        "##oo...oo#oo...oo##",
        "##oo.*.oo#oo.*.oo##",
        " ##ooooo#.#ooooo## ",
        "  ######...######  ",
        "   #.#.#.#.#.#.#   ",
        "    ###########    ",
    ],
    &[
        "        #########        ",
        "     ###############     ",
        "   #######iiiii#######   ",
        "  ########i*i*i########  ",
        " ##########iii########## ",
        " ###ooooooo###ooooooo### ",
        "###oo.....oo#oo.....oo###",
        "###oo..*..oo#oo..*..oo###",
        "###oo.....oo#oo.....oo###",
        " ###ooooooo#.#ooooooo### ",
        "  #########...#########  ",
        "   ##.#.#.#.#.#.#.#.##   ",
        "    #################    ",
        "       ###########       ",
    ],
];

#[derive(Clone, Copy)]
enum Shape {
    /// A word, and whether it is a name (names glow like candlelight, words
    /// like marigolds).
    Word(&'static str, bool),
    /// A calavera (an index into `SKULLS`), with rosa mexicano eyes rather
    /// than cempasúchil ones.
    Skull(usize, bool),
}

impl Shape {
    fn size(self) -> (usize, usize) {
        match self {
            Shape::Word(s, _) => (s.chars().count(), 1),
            Shape::Skull(i, _) => (SKULLS[i][0].len(), SKULLS[i].len()),
        }
    }

    /// Is (dx, dy) part of the figure?
    fn has(self, dx: usize, dy: usize) -> bool {
        match self {
            Shape::Word(s, _) => s.chars().nth(dx).is_some_and(|c| c != ' '),
            Shape::Skull(i, _) => SKULLS[i][dy].as_bytes()[dx] != b' ',
        }
    }
}

/// A word or a calavera hidden in the rain. The rain lights it up: a word's
/// letters fall down their columns like droplet heads and land in place; a
/// calavera is painted column by column, top-down, by droplets of its own
/// (and by any real one that gets there first).
#[derive(Clone, Copy)]
struct Figure {
    shape: Shape,
    /// Its cells' times live at `slot * SLAB` in `Muertos::touched` and `head`.
    slot: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    /// When it was hidden and how long it stays (real seconds).
    born: f64,
    life: f64,
    /// When the rain starts lighting it up (scene seconds).
    start: f64,
    /// How many rows a word's letters fall before they land.
    fall: f32,
    /// Picks when each column lights up, the glyphs a calavera is made of
    /// and the order its cells crumble.
    salt: u32,
}

impl Figure {
    /// Scene seconds after `start` that the rain sets off down column `dx`.
    fn delay(&self, dx: usize) -> f64 {
        let spread = match self.shape {
            Shape::Word(..) => 1.2,
            Shape::Skull(..) => 1.8,
        };
        (spread * hash01(self.salt, dx)) as f64
    }

    /// Scene seconds after `start` that the rain reaches (dx, dy), falling
    /// `speed` rows a second.
    fn reach(&self, dx: usize, dy: usize, speed: f32) -> f64 {
        let rows = match self.shape {
            Shape::Word(..) => self.fall,
            Shape::Skull(..) => dy as f32,
        };
        self.delay(dx) + (rows / speed) as f64
    }
}

/// A number in 0..1 from a salt and an index.
fn hash01(salt: u32, i: usize) -> f32 {
    let mut h = salt ^ (i as u32).wrapping_mul(0x9E37_79B9);
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    (h >> 8) as f32 / (1 << 24) as f32
}

/// What Día de Muertos adds to the rain: a second palette, and the figures.
struct Muertos {
    on: bool,
    level: f32,
    /// Seeded apart from the rain's own.
    rng: fastrand::Rng,
    /// How fast the rain falls, in rows a second (neo's droplets crawl about
    /// twice their nominal speed).
    speed: f32,
    /// The heaviest of the rain's characters, that calaveras are made of.
    dense: Vec<char>,
    /// The rosa mexicano palette some droplets fall in (as long as the
    /// scene's pair table), or empty when the rain keeps the user's colors.
    alt: Vec<Color>,
    word: Color,
    name: Color,
    bone: Color,
    petals: [Color; 2],
    heart: Color,
    icing: Color,
    flash: Color,
    figs: Vec<Figure>,
    /// For each figure slot and cell, when the rain lit it up (negative: not
    /// yet), and when a droplet's head last went through it (scene seconds).
    touched: Vec<f64>,
    head: Vec<f64>,
    /// Real seconds, for how long things stay.
    clock: f64,
    /// Scene seconds, for how things move.
    motion: f64,
    next_word: f64,
    next_skull: f64,
}

impl Muertos {
    fn new(seed: u64, chars_per_sec: f32, chars: &[char]) -> Muertos {
        const DENSE: &str = "#@%&$8096BWMQGDNHKRX";
        let mut dense: Vec<char> = DENSE.chars().filter(|c| chars.contains(c)).collect();
        if dense.is_empty() {
            dense = chars.to_vec();
        }
        if dense.is_empty() {
            dense.push('#');
        }
        Muertos {
            on: false,
            level: 0.0,
            rng: fastrand::Rng::with_seed(seed ^ MUERTOS_SEED),
            speed: (2.0 * chars_per_sec).clamp(4.0, 60.0),
            dense,
            alt: Vec::new(),
            word: Color::Reset,
            name: Color::Reset,
            bone: Color::Reset,
            petals: [Color::Reset; 2],
            heart: Color::Reset,
            icing: Color::Reset,
            flash: Color::Reset,
            figs: Vec::with_capacity(MAX_FIGS),
            touched: vec![-1.0; MAX_FIGS * SLAB],
            head: vec![-1.0; MAX_FIGS * SLAB],
            clock: 0.0,
            motion: 0.0,
            next_word: 0.0,
            next_skull: 0.0,
        }
    }

    /// Dresses up for the holiday (or not). Returns the rain's own palette
    /// when it changes too: cempasúchil under `classic`, the theme accent
    /// nearest to it otherwise, unless the user picked the colors.
    fn retheme(&mut self, ctx: &Ctx, auto: bool, native: bool) -> Option<Vec<Color>> {
        let was = self.on;
        self.on = ctx.fiesta.is(Kind::Muertos);
        self.level = ctx.fiesta.level.clamp(0.0, 1.0);
        self.alt.clear();
        if !self.on {
            self.figs.clear();
            return None;
        }
        if !was {
            self.figs.clear();
            self.next_word = self.clock + 0.3;
            self.next_skull = self.clock + 1.2;
        }
        let theme = ctx.theme;
        // Under other themes, the accents nearest in hue to the holiday's colors.
        let cempa = nearest(theme, CEMPASUCHIL, None);
        let rosa = nearest(theme, ROSA, Some(cempa));
        let gold = nearest(theme, AMARILLO, None);
        if theme.native {
            self.word = Color::Rgb(255, 216, 64);
            self.name = HUESO;
            self.bone = HUESO;
            self.petals = [CEMPASUCHIL, ROSA];
            self.heart = AMARILLO;
            self.icing = TURQUESA;
            self.flash = Color::Rgb(255, 250, 236);
        } else {
            self.word = gold;
            self.name = theme.fg;
            self.bone = theme.fg.lerp(WHITE, 0.3);
            self.petals = [cempa, rosa];
            self.heart = gold;
            self.icing = nearest(theme, TURQUESA, None);
            self.flash = theme.fg.lerp(WHITE, 0.8);
        }
        if !auto {
            return None;
        }
        let (main, alt) = if native {
            let table = |t: &[(u8, u8, u8)]| {
                std::iter::once(Color::Reset)
                    .chain(t.iter().map(|&(r, g, b)| Color::Rgb(r, g, b)))
                    .collect()
            };
            (table(CEMPASUCHIL_RAIN), table(ROSA_RAIN))
        } else {
            (
                accent_pairs(ctx, cempa, gold.lerp(WHITE, 0.6)),
                accent_pairs(ctx, rosa, gold.lerp(WHITE, 0.2)),
            )
        };
        self.alt = alt;
        Some(main)
    }

    /// The palette of a new droplet: more fall in rosa mexicano as the holiday comes.
    fn hue(&mut self) -> u8 {
        if self.alt.is_empty() {
            return 0;
        }
        (self.rng.f32() < 0.2 + 0.25 * self.level) as u8
    }

    /// Lights up what the rain reaches, retires old figures and hides new ones.
    fn tick(&mut self, dt: f32, real: f32, screen: &[Glyph], cols: usize, lines: usize, head: u8) {
        if dt > 0.0 {
            self.motion += (dt as f64).min(60.0);
        }
        if real > 0.0 {
            self.clock += (real as f64).min(60.0);
        }
        let (now, t) = (self.clock, self.motion);
        self.figs.retain(|f| now < f.born + f.life);
        for f in &self.figs {
            for dy in 0..f.h {
                for dx in 0..f.w {
                    let (x, y) = (f.x + dx, f.y + dy);
                    if x >= cols || y >= lines || !f.shape.has(dx, dy) {
                        continue;
                    }
                    let k = f.slot * SLAB + dy * f.w + dx;
                    let g = screen[y * cols + x];
                    let rain = g.pair != 0 && g.ch != ' ';
                    if self.touched[k] < 0.0 {
                        let reach = f.start + f.reach(dx, dy, self.speed);
                        if t >= reach {
                            self.touched[k] = reach;
                        } else if rain && matches!(f.shape, Shape::Skull(..)) {
                            self.touched[k] = t;
                        }
                    }
                    if rain && g.pair == head {
                        self.head[k] = t;
                    }
                }
            }
        }
        let l = self.level as f64;
        if now >= self.next_word {
            self.next_word = now + self.rng.f64() * 2.5 + 2.5 / (0.5 + l);
            self.spawn(false, cols, lines);
        }
        if now >= self.next_skull {
            self.next_skull = now + (9.0 + self.rng.f64() * 5.0) / (0.4 + 0.6 * l);
            self.spawn(true, cols, lines);
        }
    }

    /// Hides a new word or calavera somewhere free in the rain, if it fits.
    fn spawn(&mut self, skull: bool, cols: usize, lines: usize) {
        let area = (cols * lines) as f32;
        let cap = ((2.0 + self.level * area / 1800.0) as usize).clamp(1, MAX_FIGS);
        let skulls = self
            .figs
            .iter()
            .filter(|f| matches!(f.shape, Shape::Skull(..)))
            .count();
        let max_skulls = if self.level >= 0.8 && area >= 8000.0 {
            2
        } else {
            1
        };
        if self.figs.len() >= cap || (skull && skulls >= max_skulls) {
            return;
        }
        let Some(slot) = (0..MAX_FIGS).find(|&s| self.figs.iter().all(|f| f.slot != s)) else {
            return;
        };
        let shape = if skull {
            let i = if lines >= 44 && cols >= 100 {
                2
            } else if lines >= 24 && cols >= 40 {
                1
            } else {
                0
            };
            Shape::Skull(i, self.rng.bool())
        } else {
            let r = self.rng.f32();
            match day_word(self.level) {
                Some(w) if r < 0.15 => Shape::Word(w, false),
                _ if r < 0.5 => Shape::Word(NAMES[self.rng.usize(0..NAMES.len())], true),
                _ => Shape::Word(WORDS[self.rng.usize(0..WORDS.len())], false),
            }
        };
        let (w, h) = shape.size();
        // Keep clear of the papel picado strung across the top.
        let top = match lines {
            0..18 => 0,
            _ if self.level >= 0.8 => 6,
            _ => 4,
        };
        // A word keeps a free cell at each end.
        let pad = matches!(shape, Shape::Word(..)) as usize;
        if w + 2 * pad > cols || h + top > lines {
            return;
        }
        for _ in 0..12 {
            let x = self.rng.usize(pad..=cols - w - pad);
            let y = self.rng.usize(top..=lines - h);
            let clear = self
                .figs
                .iter()
                .all(|f| x + w + 2 <= f.x || f.x + f.w + 2 <= x || y + h < f.y || f.y + f.h < y);
            if !clear {
                continue;
            }
            let cells = slot * SLAB..slot * SLAB + w * h;
            self.touched[cells.clone()].fill(-1.0);
            self.head[cells].fill(-1.0);
            let life = if skull { 11.0 } else { 8.0 } + self.rng.f64() * 2.0;
            self.figs.push(Figure {
                shape,
                slot,
                x,
                y,
                w,
                h,
                born: self.clock,
                life,
                start: self.motion,
                fall: (y as f32).min(10.0),
                salt: self.rng.u32(..),
            });
            return;
        }
    }

    fn draw(&self, ctx: &Ctx, c: &mut Canvas, pool: &[char]) {
        for f in &self.figs {
            // At the end, cells dim and crumble back into the rain one by one.
            let out = ((self.clock - (f.born + f.life - FADE)) / FADE).clamp(0.0, 1.0) as f32;
            let base = f.slot * SLAB;
            let (fx, fy) = (f.x as i32, f.y as i32);
            match f.shape {
                Shape::Word(word, name) => {
                    let color = if name { self.name } else { self.word };
                    let mut prev = false;
                    for (dx, ch) in word.chars().enumerate() {
                        let x = fx + dx as i32;
                        if ch == ' ' {
                            // Clear between words once the letter before has landed.
                            if prev {
                                c.set(x, fy, ' ', Color::Reset);
                            }
                            continue;
                        }
                        if let Some(fg) = self.glow(ctx, color, base + dx, out, f.salt, dx) {
                            c.set_attr(x, fy, ch, fg, BOLD);
                            prev = true;
                            if dx == 0 {
                                c.set(x - 1, fy, ' ', Color::Reset);
                            }
                            if dx + 1 == f.w {
                                c.set(x + 1, fy, ' ', Color::Reset);
                            }
                            continue;
                        }
                        prev = false;
                        if self.touched[base + dx] >= 0.0 {
                            continue;
                        }
                        // Still falling: the letter leads a short trail down its column.
                        let since = self.motion - f.start - f.delay(dx);
                        if since < 0.0 {
                            continue;
                        }
                        let y = fy - f.fall as i32 + (since * self.speed as f64) as i32;
                        c.set_attr(x, y, ch, self.flash, BOLD);
                        for i in 1..4 {
                            if y - i < fy - f.fall as i32 {
                                break;
                            }
                            let g =
                                pool[(f.salt as usize + (y - i) as usize * 13 + dx) % pool.len()];
                            c.set(x, y - i, g, ctx.fade(color, 0.25 * i as f32));
                        }
                    }
                }
                Shape::Skull(i, rosa) => {
                    for (dy, row) in SKULLS[i].iter().enumerate() {
                        for (dx, m) in row.bytes().enumerate() {
                            let color = match m {
                                b'#' => self.bone,
                                b'o' => self.petals[rosa as usize],
                                b'*' => self.heart,
                                b'i' => self.icing,
                                b'.' => Color::Reset,
                                _ => continue,
                            };
                            let k = dy * f.w + dx;
                            let Some(fg) = self.glow(ctx, color, base + k, out, f.salt, k) else {
                                continue;
                            };
                            let (x, y) = (fx + dx as i32, fy + dy as i32);
                            // Clear the rain right around it, so its outline reads.
                            for (nx, ny) in [(-2, 0), (-1, 0), (1, 0), (2, 0), (0, -1), (0, 1)] {
                                let (ex, ey) = (dx as i32 + nx, dy as i32 + ny);
                                let inside = ex >= 0
                                    && ey >= 0
                                    && (ex as usize) < f.w
                                    && (ey as usize) < f.h
                                    && f.shape.has(ex as usize, ey as usize);
                                if !inside {
                                    c.set(fx + ex, fy + ey, ' ', Color::Reset);
                                }
                            }
                            if m == b'.' {
                                c.set(x, y, ' ', Color::Reset);
                            } else {
                                let g = pool[(f.salt as usize + k * 7) % pool.len()];
                                let ch = self.dense[g as usize % self.dense.len()];
                                c.set_attr(x, y, ch, fg, BOLD);
                            }
                        }
                    }
                }
            }
        }
    }

    /// The color of a figure's cell `k`: nothing until the rain lights it
    /// up, then a flash that settles into `color`, flashing again whenever a
    /// droplet's head goes through; dimmer, then gone, as the figure fades `out`.
    fn glow(
        &self,
        ctx: &Ctx,
        color: Color,
        k: usize,
        out: f32,
        salt: u32,
        i: usize,
    ) -> Option<Color> {
        let touched = self.touched[k];
        if touched < 0.0 {
            return None;
        }
        // Each cell crumbles at its own moment of the fade.
        if out > 0.0 && out >= 0.15 + 0.85 * hash01(salt ^ 0x5EED, i) {
            return None;
        }
        let since = (self.motion - touched) as f32;
        let mut flash = (1.0 - since / 0.6).max(0.0);
        if self.head[k] >= 0.0 {
            flash = flash.max(0.8 * (1.0 - (self.motion - self.head[k]) as f32 / 0.35).max(0.0));
        }
        if !color.is_rgb() {
            return Some(color);
        }
        Some(ctx.fade(color.lerp(self.flash, flash), 0.6 * out))
    }
}

const WHITE: Color = Color::Rgb(255, 255, 255);
const CEMPASUCHIL: Color = Color::Rgb(255, 140, 0);
const ROSA: Color = Color::Rgb(228, 0, 124);
const AMARILLO: Color = Color::Rgb(255, 210, 0);
const TURQUESA: Color = Color::Rgb(0, 170, 184);
const HUESO: Color = Color::Rgb(244, 238, 226);

/// The rain in cempasúchil under `classic`, pair 1 (the tail end) to the head.
const CEMPASUCHIL_RAIN: &[(u8, u8, u8)] = &[
    (52, 20, 0),
    (124, 48, 0),
    (190, 82, 0),
    (230, 114, 0),
    (255, 148, 12),
    (255, 190, 60),
    (255, 236, 170),
];

/// And in rosa mexicano, through terciopelo, with golden heads.
const ROSA_RAIN: &[(u8, u8, u8)] = &[
    (46, 0, 26),
    (112, 0, 54),
    (176, 0, 72),
    (212, 0, 106),
    (236, 34, 138),
    (250, 116, 184),
    (255, 206, 96),
];

/// The theme's accent closest in hue to `to` (grays come last), other than `not`.
fn nearest(theme: &crate::theme::Theme, to: Color, not: Option<Color>) -> Color {
    let Some((h, _)) = hue_sat(to) else { return to };
    let far = |c: Color| match hue_sat(c) {
        Some((h2, s)) => {
            let d = (h - h2).abs() % 360.0;
            d.min(360.0 - d) + 120.0 * (1.0 - s)
        }
        None => f32::MAX,
    };
    theme
        .colors
        .iter()
        .copied()
        .filter(|&c| Some(c) != not)
        .min_by(|&a, &b| far(a).total_cmp(&far(b)))
        .unwrap_or(to)
}

/// Hue (degrees) and saturation (0 to 1) of an RGB color, or of one of the
/// terminal's 16 colors (when it would not tell us its palette) by its name.
fn hue_sat(c: Color) -> Option<(f32, f32)> {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        // Bit 0 red, bit 1 green, bit 2 blue; black, white and the grays have no hue.
        Color::Ansi(i) if i < 16 => match i & 7 {
            0 | 7 => (128, 128, 128),
            m => (255 * (m & 1), 255 * ((m >> 1) & 1), 255 * (m >> 2)),
        },
        _ => return None,
    };
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let d = max - min;
    if d <= 0.0 {
        return Some((0.0, 0.0));
    }
    let h = if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    Some((h, d / max))
}

/// A pair table like the theme ramp's (see `resolve_colors`), in shades of
/// one accent, with its own head color.
fn accent_pairs(ctx: &Ctx, accent: Color, head: Color) -> Vec<Color> {
    const LEVELS: usize = 12;
    let g = color::Gradient::new(&[ctx.fade(accent, 0.7), accent, accent.lerp(WHITE, 0.35)]);
    let mut pairs = Vec::with_capacity(LEVELS + 1);
    pairs.push(Color::Reset);
    for i in 1..LEVELS {
        let t = (i - 1) as f32 / (LEVELS - 2) as f32;
        pairs.push(ctx.fade(g.at(0.1 + 0.85 * t), 0.7 * (1.0 - t) * (1.0 - t)));
    }
    pairs.push(head);
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::scene::Opts;
    use crate::theme::Theme;

    #[test]
    fn charsets_are_single_width() {
        for name in [
            "ascii",
            "extended",
            "english",
            "digits",
            "punc",
            "binary",
            "hex",
            "katakana",
            "greek",
            "cyrillic",
            "arabic",
            "hebrew",
            "devanagari",
            "braille",
            "runic",
        ] {
            let chars = build_chars(name, "");
            assert!(!chars.is_empty(), "{name}");
            assert!(chars.iter().all(|&c| !odd_width(c)), "{name}");
        }
        assert_eq!(build_chars("auto", "01"), vec!['0', '1']);
        assert_eq!(build_chars("binary", "x"), vec!['0', '1', 'x']);
        // Wide characters are dropped; nothing usable left falls back to neo's default.
        assert_eq!(build_chars("", "漢字"), build_chars("extended", ""));
        assert_eq!(build_chars("katakana", "").len(), 0xFF9D - 0xFF66 + 1);
    }

    #[test]
    fn neo_green_truecolor() {
        let p = neo_scheme("green").unwrap();
        assert_eq!(p.len(), 8);
        assert_eq!(p[1], Color::Rgb(18, 35, 21));
        assert_eq!(p[7], Color::Rgb(170, 255, 239));
        assert_eq!(neo_scheme("vaporwave").unwrap().len(), 16);
    }

    /// Odd option values and sizes must not panic (the registry test covers the defaults).
    #[test]
    fn extreme_options_survive() {
        let cases = [
            "speed = 1000000.0\ndensity = 99.0\nglitch_ms = '1,1'\nasync = true\ncharset = 'arabic+hebrew'\nshading = 'distance'\nmessage = 'hello there, this is a long message for a tiny screen'",
            "chars = '漢字'\nspeed = 0.0001\ndensity = 0.0001\nglitch_pct = 100.0\nrip_pct = 100.0\nshort_pct = 100.0\nlinger_ms = '65535,65535'\nbold = 2\ncolors = '#ff0000'\nmax_dpc = 9",
            "glitch = false\ncolors = '#000,#fff'\nbold = -4\nspeed = -3.0\ndensity = 0.0\nglitch_pct = -5.0\nmessage = 'x'",
        ];
        for (i, case) in cases.iter().enumerate() {
            let opts = Opts::new(INFO.options, case.parse().unwrap());
            for theme in [Theme::classic(), Theme::builtin("nord").unwrap()] {
                let ctx = Ctx {
                    theme: &theme,
                    opts: &opts,
                    bg: Color::Rgb(10, 10, 20),
                    seed: 3 + i as u64,
                    fiesta: crate::fiesta::Fiesta::NONE,
                };
                let mut s = (INFO.make)(&ctx);
                for &(w, h) in &[(1u16, 1u16), (2, 3), (3, 1), (1, 40), (500, 150), (80, 24)] {
                    s.resize(&ctx, w, h);
                    s.retheme(&ctx);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for dt in [
                        0.0,
                        f32::NAN,
                        -1.0,
                        f32::INFINITY,
                        1.0 / 30.0,
                        5.0,
                        1.0 / 30.0,
                    ] {
                        s.update(&ctx, dt);
                        c.clear();
                        s.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }

    #[test]
    fn ranges_parse() {
        assert_eq!(parse_range("300,400", (1, 2)), (300, 400));
        assert_eq!(parse_range("400,300", (1, 2)), (1, 2));
        assert_eq!(parse_range("0,3", (1, 2)), (1, 2));
        assert_eq!(parse_range("50", (1, 2)), (50, 50));
    }

    /// A neo dressed (or not) for a day, at a size.
    fn dressed(theme: &Theme, opts: &Opts, fiesta: Fiesta, w: u16, h: u16) -> Neo {
        let ctx = Ctx {
            theme,
            opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 11,
            fiesta,
        };
        let mut n = Neo::new(&ctx);
        n.resize(&ctx, w, h);
        n.retheme(&ctx);
        n
    }

    /// Runs `secs` seconds of frames, in 30 fps steps, calling `look` after each.
    fn run(n: &mut Neo, ctx: &Ctx, secs: f32, mut look: impl FnMut(&Neo, &Canvas)) {
        let (w, h) = (n.cols as u16, n.lines as u16);
        let mut c = Canvas::new(w, h, Color::Reset);
        for _ in 0..(secs * 30.0) as usize {
            n.tick(ctx, 1.0 / 30.0, 1.0 / 30.0);
            c.clear();
            n.draw(ctx, &mut c);
            look(n, &c);
        }
    }

    fn row_is(c: &Canvas, x: usize, y: usize, text: &str) -> bool {
        text.chars().enumerate().all(|(i, ch)| {
            c.get((x + i) as i32, y as i32)
                .is_some_and(|cell| cell.ch == ch)
        })
    }

    /// What the holiday adds shows: the rain in its two palettes, words that
    /// land in place, calaveras made of heavy glyphs with flower eyes.
    #[test]
    fn muertos_lights_the_rain() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let mut most = Vec::new();
        for fiesta in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
            let ctx = Ctx {
                theme: &theme,
                opts: &opts,
                bg: Color::Rgb(13, 17, 23),
                seed: 11,
                fiesta,
            };
            let mut n = dressed(&theme, &opts, fiesta, 120, 36);
            assert!(n.muertos.on);
            let cempa: Vec<Color> = CEMPASUCHIL_RAIN
                .iter()
                .map(|&(r, g, b)| Color::Rgb(r, g, b))
                .collect();
            assert_eq!(&n.pairs[1..], &cempa[..], "cempasúchil, not neo green");
            assert_eq!(n.muertos.alt.len(), n.pairs.len());
            let (mut word, mut skull, mut rosa, mut figs) = (false, false, false, 0);
            run(&mut n, &ctx, 40.0, |n, c| {
                figs = figs.max(n.muertos.figs.len());
                rosa |= n.screen.iter().any(|g| g.hue == 1 && g.pair != 0);
                for f in &n.muertos.figs {
                    let settled = n.muertos.clock < f.born + f.life - FADE;
                    match f.shape {
                        Shape::Word(w, _) if settled => {
                            let lit =
                                (0..f.w).all(|dx| n.muertos.touched[f.slot * SLAB + dx] >= 0.0);
                            word |= lit && row_is(c, f.x, f.y, w);
                        }
                        Shape::Skull(..) if settled => {
                            // Flower eyes in petals and golden hearts, on bone glyphs.
                            let bone = c.cells().iter().filter(|x| x.fg == n.muertos.bone).count();
                            let eyes = c.cells().iter().any(|x| x.fg == n.muertos.heart);
                            skull |= bone > 40 && eyes;
                        }
                        _ => {}
                    }
                }
            });
            assert!(word, "a word landed in the rain");
            assert!(skull, "a calavera was painted");
            assert!(rosa, "some droplets fell in rosa mexicano");
            most.push(figs);
        }
        assert!(
            most[1] > most[0],
            "more is hidden in the rain at the height: {most:?}"
        );
    }

    /// Every word and calavera fits its slab, in cells the rain can light up.
    #[test]
    fn figures_fit_their_slabs() {
        for w in WORDS.iter().chain(NAMES) {
            assert!(
                w.chars().count() <= SLAB && w.chars().all(|c| !odd_width(c)),
                "{w}"
            );
        }
        for level in [0.25, 0.35, 0.45, 0.5, 0.6, 0.8, 1.0] {
            if let Some(w) = day_word(level) {
                assert!(
                    w.chars().count() <= SLAB && w.chars().all(|c| !odd_width(c)),
                    "{w}"
                );
            }
        }
        for (i, s) in SKULLS.iter().enumerate() {
            let w = s[0].len();
            assert!(s.iter().all(|r| r.len() == w), "skull {i} is a rectangle");
            assert!(w * s.len() <= SLAB, "skull {i}");
            assert!(s.iter().all(|r| r.bytes().all(|b| b" #o*i.".contains(&b))));
        }
        // The days of the holiday each have a word of their own, besides the first two.
        for level in [0.45, 0.5, 0.6, 0.8, 1.0] {
            assert!(day_word(level).is_some(), "{level}");
        }
        assert!(day_word(0.25).is_none() && day_word(0.35).is_none());
    }

    /// On an ordinary day there is nothing of it: neo's own green, no figures.
    #[test]
    fn ordinary_days_have_none() {
        let theme = Theme::classic();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 11,
            fiesta: Fiesta::NONE,
        };
        let mut n = dressed(&theme, &opts, Fiesta::NONE, 120, 36);
        assert!(!n.muertos.on);
        assert_eq!(n.pairs, neo_scheme("green").unwrap());
        run(&mut n, &ctx, 40.0, |n, _| {
            assert!(n.muertos.figs.is_empty() && n.muertos.alt.is_empty());
            assert!(n.screen.iter().all(|g| g.hue == 0));
        });
        // Not even the random numbers: the same rain as one that never heard of it.
        let mut a = dressed(&theme, &opts, Fiesta::NONE, 80, 24);
        let mut b = dressed(&theme, &opts, Fiesta::NONE, 80, 24);
        for _ in 0..300 {
            a.update(&ctx, 1.0 / 30.0);
            b.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
        assert!(a.screen == b.screen);
    }

    /// The holiday comes and goes through `retheme`, in place.
    #[test]
    fn holiday_comes_and_goes() {
        let theme = Theme::builtin("dracula").unwrap();
        let opts = Opts::new(INFO.options, toml::Table::new());
        let mut ctx = Ctx {
            theme: &theme,
            opts: &opts,
            bg: Color::Rgb(13, 17, 23),
            seed: 11,
            fiesta: Fiesta::NONE,
        };
        let mut n = dressed(&theme, &opts, Fiesta::NONE, 100, 30);
        let plain = n.pairs.clone();
        run(&mut n, &ctx, 3.0, |_, _| {});
        ctx.fiesta = Fiesta::of_day(11, 2);
        n.retheme(&ctx);
        assert!(n.muertos.on && n.pairs != plain && !n.muertos.alt.is_empty());
        let mut seen = false;
        run(&mut n, &ctx, 30.0, |n, _| {
            seen |= !n.muertos.figs.is_empty()
        });
        assert!(seen, "words or calaveras were hidden");
        ctx.fiesta = Fiesta::NONE;
        n.retheme(&ctx);
        assert!(!n.muertos.on && n.pairs == plain && n.muertos.alt.is_empty());
        run(&mut n, &ctx, 30.0, |n, _| {
            assert!(n.muertos.figs.is_empty())
        });
        // And back on, with the holiday's own palette again.
        ctx.fiesta = Fiesta::of_day(10, 28);
        n.retheme(&ctx);
        assert!(n.muertos.on && n.pairs != plain);
        // A palette picked by the user stays theirs.
        let opts = Opts::new(INFO.options, "colors = 'blue'".parse().unwrap());
        let mut blue = dressed(&theme, &opts, Fiesta::of_day(11, 2), 100, 30);
        assert!(blue.muertos.on && blue.muertos.alt.is_empty());
        assert_eq!(blue.pairs, neo_scheme("blue").unwrap());
        run(&mut blue, &ctx, 5.0, |_, _| {});
    }

    /// The holiday survives tiny and huge screens, huge steps and odd options,
    /// under `classic` and another theme.
    #[test]
    fn holiday_survives() {
        let cases = [
            "",
            "speed = 1000000.0\ndensity = 99.0\nasync = true\ncharset = 'katakana'\nmessage = 'hello there, this is a long message for a tiny screen'",
            "charset = 'binary'\nspeed = 0.0001\nglitch_pct = 100.0\nrip_pct = 100.0\nshort_pct = 100.0\nlinger_ms = '65535,65535'\nbold = 2",
            "chars = '#@'\ncolors = '#ff0000'\nspeed = -3.0\ndensity = 0.0\nmax_dpc = 9",
        ];
        for (i, case) in cases.iter().enumerate() {
            let opts = Opts::new(INFO.options, case.parse().unwrap());
            for theme in [Theme::classic(), Theme::builtin("nord").unwrap()] {
                for fiesta in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
                    let ctx = Ctx {
                        theme: &theme,
                        opts: &opts,
                        bg: Color::Rgb(10, 10, 20),
                        seed: 3 + i as u64,
                        fiesta,
                    };
                    let mut s = (INFO.make)(&ctx);
                    let sizes = [(1u16, 1u16), (300, 90), (2, 3), (3, 1), (1, 40), (17, 5)]
                        .into_iter()
                        .chain([(40, 12), (80, 24)]);
                    for (w, h) in sizes {
                        // The biggest screen is slow in a debug build: two options will do.
                        if w == 300 && i > 1 {
                            continue;
                        }
                        s.resize(&ctx, w, h);
                        s.retheme(&ctx);
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for f in 0..24 {
                            let dt = match f % 9 {
                                0 => 5.0,
                                3 => 0.0,
                                5 => f32::NAN,
                                7 => f32::INFINITY,
                                _ => 1.0 / 30.0,
                            };
                            s.tick(&ctx, dt, if f % 13 == 12 { 5.0 } else { 1.0 / 30.0 });
                            c.clear();
                            s.draw(&ctx, &mut c);
                        }
                    }
                }
            }
        }
    }
}
