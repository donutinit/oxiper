//! dvd: the bouncing logo, hoping to hit a corner.
//!
//! The logo moves two columns per row so it travels at 45° on screen, changes
//! color on every bounce, and celebrates when it lands exactly in a corner.

use crate::color::{Color, rgb};
use crate::render::{BOLD, Canvas};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "dvd",
    aliases: &["bounce"],
    about: "the bouncing logo, hoping to hit a corner",
    options: &[
        Opt {
            key: "text",
            default: Val::S(""),
            help: "Bounce this text in a block font instead of the DVD logo",
        },
        Opt {
            key: "speed",
            default: Val::F(5.0),
            help: "Rows per second (columns move twice as fast, for a 45° path)",
        },
        Opt {
            key: "count",
            default: Val::I(1),
            help: "How many logos bounce around",
        },
        Opt {
            key: "corner_assist",
            default: Val::B(false),
            help: "Quietly nudge the path now and then so corners actually happen",
        },
        Opt {
            key: "counter",
            default: Val::B(true),
            help: "Show how many corners were hit",
        },
    ],
    make: |ctx| Box::new(Dvd::new(ctx)),
};

const LOGO: [&str; 6] = [
    "██████▄  ██▄   ▄██ ██████▄ ",
    "██   ███  ██▄ ▄██  ██   ███",
    "██████▀    ▀███▀   ██████▀ ",
    "      ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄      ",
    "▄███████ V I D E O ███████▄",
    " ▀▀▀▀▀███████████████▀▀▀▀▀ ",
];

/// The colors old DVD players cycled through.
const CLASSIC: [Color; 8] = [
    rgb(0xff2b2b),
    rgb(0xff8a00),
    rgb(0xffe100),
    rgb(0x3ce03c),
    rgb(0x00d5ff),
    rgb(0x2f6bff),
    rgb(0xb04bff),
    rgb(0xff3fb4),
];

/// 5×5 block font: one byte per row, bit 4 is the leftmost pixel.
const FONT: &[(char, [u8; 5])] = &[
    ('A', [0x0e, 0x11, 0x1f, 0x11, 0x11]),
    ('B', [0x1e, 0x11, 0x1e, 0x11, 0x1e]),
    ('C', [0x0f, 0x10, 0x10, 0x10, 0x0f]),
    ('D', [0x1e, 0x11, 0x11, 0x11, 0x1e]),
    ('E', [0x1f, 0x10, 0x1e, 0x10, 0x1f]),
    ('F', [0x1f, 0x10, 0x1e, 0x10, 0x10]),
    ('G', [0x0f, 0x10, 0x13, 0x11, 0x0f]),
    ('H', [0x11, 0x11, 0x1f, 0x11, 0x11]),
    ('I', [0x1f, 0x04, 0x04, 0x04, 0x1f]),
    ('J', [0x07, 0x01, 0x01, 0x11, 0x0e]),
    ('K', [0x11, 0x12, 0x1c, 0x12, 0x11]),
    ('L', [0x10, 0x10, 0x10, 0x10, 0x1f]),
    ('M', [0x11, 0x1b, 0x15, 0x11, 0x11]),
    ('N', [0x11, 0x19, 0x15, 0x13, 0x11]),
    ('O', [0x0e, 0x11, 0x11, 0x11, 0x0e]),
    ('P', [0x1e, 0x11, 0x1e, 0x10, 0x10]),
    ('Q', [0x0e, 0x11, 0x15, 0x12, 0x0d]),
    ('R', [0x1e, 0x11, 0x1e, 0x12, 0x11]),
    ('S', [0x0f, 0x10, 0x0e, 0x01, 0x1e]),
    ('T', [0x1f, 0x04, 0x04, 0x04, 0x04]),
    ('U', [0x11, 0x11, 0x11, 0x11, 0x0e]),
    ('V', [0x11, 0x11, 0x11, 0x0a, 0x04]),
    ('W', [0x11, 0x11, 0x15, 0x1b, 0x11]),
    ('X', [0x11, 0x0a, 0x04, 0x0a, 0x11]),
    ('Y', [0x11, 0x0a, 0x04, 0x04, 0x04]),
    ('Z', [0x1f, 0x02, 0x04, 0x08, 0x1f]),
    ('0', [0x0e, 0x13, 0x15, 0x19, 0x0e]),
    ('1', [0x04, 0x0c, 0x04, 0x04, 0x0e]),
    ('2', [0x1e, 0x01, 0x0e, 0x10, 0x1f]),
    ('3', [0x1e, 0x01, 0x06, 0x01, 0x1e]),
    ('4', [0x12, 0x12, 0x1f, 0x02, 0x02]),
    ('5', [0x1f, 0x10, 0x1e, 0x01, 0x1e]),
    ('6', [0x0e, 0x10, 0x1e, 0x11, 0x0e]),
    ('7', [0x1f, 0x01, 0x02, 0x04, 0x04]),
    ('8', [0x0e, 0x11, 0x0e, 0x11, 0x0e]),
    ('9', [0x0e, 0x11, 0x0f, 0x01, 0x0e]),
    ('!', [0x04, 0x04, 0x04, 0x00, 0x04]),
    ('?', [0x0e, 0x11, 0x06, 0x00, 0x04]),
    ('.', [0x00, 0x00, 0x00, 0x00, 0x04]),
    (',', [0x00, 0x00, 0x00, 0x04, 0x08]),
    (':', [0x00, 0x04, 0x00, 0x04, 0x00]),
    (';', [0x00, 0x04, 0x00, 0x04, 0x08]),
    ('-', [0x00, 0x00, 0x0e, 0x00, 0x00]),
    ('+', [0x00, 0x04, 0x0e, 0x04, 0x00]),
    ('=', [0x00, 0x1f, 0x00, 0x1f, 0x00]),
    ('_', [0x00, 0x00, 0x00, 0x00, 0x1f]),
    ('\'', [0x04, 0x04, 0x00, 0x00, 0x00]),
    ('"', [0x0a, 0x0a, 0x00, 0x00, 0x00]),
    ('/', [0x01, 0x02, 0x04, 0x08, 0x10]),
    ('\\', [0x10, 0x08, 0x04, 0x02, 0x01]),
    ('*', [0x15, 0x0e, 0x1f, 0x0e, 0x15]),
    ('#', [0x0a, 0x1f, 0x0a, 0x1f, 0x0a]),
    ('(', [0x02, 0x04, 0x04, 0x04, 0x02]),
    (')', [0x08, 0x04, 0x04, 0x04, 0x08]),
    ('[', [0x06, 0x04, 0x04, 0x04, 0x06]),
    (']', [0x0c, 0x04, 0x04, 0x04, 0x0c]),
    ('<', [0x02, 0x04, 0x08, 0x04, 0x02]),
    ('>', [0x08, 0x04, 0x02, 0x04, 0x08]),
    ('%', [0x19, 0x1a, 0x04, 0x0b, 0x13]),
    ('&', [0x0c, 0x12, 0x0d, 0x12, 0x0d]),
    ('@', [0x0e, 0x11, 0x17, 0x10, 0x0f]),
    ('$', [0x0f, 0x14, 0x0e, 0x05, 0x1e]),
    ('^', [0x04, 0x0a, 0x11, 0x00, 0x00]),
    ('~', [0x00, 0x08, 0x15, 0x02, 0x00]),
    ('♥', [0x0a, 0x1f, 0x1f, 0x0e, 0x04]),
];

fn glyph(ch: char) -> Option<[u8; 5]> {
    let up = ch.to_ascii_uppercase();
    FONT.iter().find(|(c, _)| *c == up).map(|(_, g)| *g)
}

/// Columns a glyph occupies (empty columns trimmed), as (first bit, width).
fn glyph_span(g: &[u8; 5]) -> (u32, u32) {
    let bits = g.iter().fold(0u8, |a, b| a | b);
    if bits == 0 {
        return (0, 0);
    }
    let hi = 7 - bits.leading_zeros(); // leftmost set bit (4 = column 0)
    let lo = bits.trailing_zeros();
    (hi, hi - lo + 1)
}

/// A small picture of characters; spaces are transparent.
#[derive(Default)]
struct Sprite {
    w: i32,
    h: i32,
    cells: Vec<char>,
}

impl Sprite {
    fn from_lines(lines: &[&str]) -> Sprite {
        let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let mut s = Sprite {
            w: w as i32,
            h: lines.len() as i32,
            cells: vec![' '; w * lines.len()],
        };
        for (y, l) in lines.iter().enumerate() {
            for (x, ch) in l.chars().enumerate() {
                s.cells[y * w + x] = ch;
            }
        }
        s
    }

    /// Plain one-line text.
    fn plain(text: &str) -> Sprite {
        Sprite::from_lines(&[text])
    }

    /// Width in font pixels of `text` set in the block font.
    fn text_px(text: &str) -> i32 {
        let mut w = 0;
        for ch in text.chars() {
            w += match glyph(ch).or(glyph('?')) {
                _ if ch == ' ' => 3,
                Some(g) => glyph_span(&g).1 as i32 + 1,
                None => 0,
            };
        }
        (w - 1).max(0)
    }

    /// Renders `text` in the 5×5 block font. Scale 1 packs two pixel rows per
    /// cell with half blocks (square pixels, 3 rows tall); scale 2 uses a full
    /// block two columns wide per pixel (5 rows tall).
    fn from_text(text: &str, scale: i32) -> Sprite {
        let pw = Sprite::text_px(text).max(1);
        let mut px = vec![false; pw as usize * 5];
        let mut x0 = 0;
        for ch in text.chars() {
            if ch == ' ' {
                x0 += 3;
                continue;
            }
            let Some(g) = glyph(ch).or(glyph('?')) else {
                continue;
            };
            let (hi, w) = glyph_span(&g);
            for (row, bits) in g.iter().enumerate() {
                for i in 0..w {
                    if bits >> (hi - i) & 1 == 1 && x0 + (i as i32) < pw {
                        px[row * pw as usize + (x0 as usize + i as usize)] = true;
                    }
                }
            }
            x0 += w as i32 + 1;
        }
        let at = |x: i32, y: i32| y < 5 && px[(y * pw + x) as usize];
        if scale >= 2 {
            let (w, h) = (pw * 2, 5);
            let mut cells = vec![' '; (w * h) as usize];
            for y in 0..h {
                for x in 0..w {
                    if at(x / 2, y) {
                        cells[(y * w + x) as usize] = '█';
                    }
                }
            }
            Sprite { w, h, cells }
        } else {
            let (w, h) = (pw, 3);
            let mut cells = vec![' '; (w * h) as usize];
            for y in 0..h {
                for x in 0..w {
                    cells[(y * w + x) as usize] = match (at(x, y * 2), at(x, y * 2 + 1)) {
                        (true, true) => '█',
                        (true, false) => '▀',
                        (false, true) => '▄',
                        _ => ' ',
                    };
                }
            }
            Sprite { w, h, cells }
        }
    }

    /// The biggest rendering of `text` that leaves room to move on a `sw`×`sh`
    /// screen; the big font is used when it takes at most `big`% of the width.
    fn fit_text(text: &str, sw: i32, sh: i32, big: i32) -> Sprite {
        let px = Sprite::text_px(text);
        // Keep at least twice the sprite's height free to bounce in.
        if px * 2 * 100 <= sw * big && sh >= 10 {
            Sprite::from_text(text, 2)
        } else if px + 2 <= sw && sh >= 4 {
            Sprite::from_text(text, 1)
        } else {
            Sprite::plain(text)
        }
    }

    fn draw(&self, c: &mut Canvas, x: i32, y: i32, fg: Color, attr: u8) {
        for row in 0..self.h {
            for col in 0..self.w {
                let ch = self.cells[(row * self.w + col) as usize];
                if ch != ' ' {
                    c.set_attr(x + col, y + row, ch, fg, attr);
                }
            }
        }
    }
}

struct Logo {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: usize,
    since_bounce: f32,
    since_corner: f32,
}

#[derive(Clone, Copy)]
struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    ttl: f32,
    color: u8,
}

const SPARK_GLYPHS: [char; 5] = ['✦', '✧', '*', '+', '·'];
/// How long the "CORNER!" celebration lasts.
const PARTY: f32 = 2.4;

struct Dvd {
    rng: fastrand::Rng,
    text: String,
    speed: f32,
    count: usize,
    assist: bool,
    show_counter: bool,

    w: i32,
    h: i32,
    sprite: Sprite,
    banner: Sprite,
    logos: Vec<Logo>,
    sparks: Vec<Spark>,
    spark_cap: usize,
    corners: u32,
    /// Seconds since the last corner hit.
    party_t: f32,
    clock: f32,

    palette: Vec<Color>,
    counter_dim: Color,
}

impl Dvd {
    fn new(ctx: &Ctx) -> Dvd {
        let o = ctx.opts;
        Dvd {
            rng: ctx.rng(),
            text: o.s("text").replace(['\n', '\t'], " ").trim().to_string(),
            speed: o.f("speed").clamp(0.0, 500.0),
            count: o.i("count").clamp(1, 64) as usize,
            assist: o.b("corner_assist"),
            show_counter: o.b("counter"),
            w: 1,
            h: 1,
            sprite: Sprite::default(),
            banner: Sprite::default(),
            logos: Vec::new(),
            sparks: Vec::new(),
            spark_cap: 0,
            corners: 0,
            party_t: PARTY,
            clock: 0.0,
            palette: CLASSIC.to_vec(),
            counter_dim: Color::Reset,
        }
    }

    fn max_x(&self) -> f32 {
        (self.w - self.sprite.w).max(0) as f32
    }

    fn max_y(&self) -> f32 {
        (self.h - self.sprite.h).max(0) as f32
    }

    fn other_color(&mut self, cur: usize) -> usize {
        let n = self.palette.len().max(1);
        if n == 1 {
            return 0;
        }
        (cur + 1 + self.rng.usize(..n - 1)) % n
    }

    /// With corner assist, bend the vertical speed a little (right at an x
    /// bounce, where nobody notices) so the next x wall arrives together with a
    /// y wall.
    fn plan(&mut self, i: usize) {
        let (mx, my) = (self.max_x(), self.max_y());
        let base = self.speed;
        let l = &mut self.logos[i];
        let sy = if l.vy < 0.0 { -1.0 } else { 1.0 };
        l.vy = sy * base;
        if !self.assist || mx <= 0.0 || my <= 0.0 || base <= 0.0 || self.rng.f32() > 0.5 {
            return;
        }
        let dist_x = if l.vx > 0.0 { mx - l.x } else { l.x };
        let travel = base * dist_x / l.vx.abs().max(1e-3);
        if travel < 1.0 {
            return;
        }
        let d0 = if l.vy > 0.0 { my - l.y } else { l.y };
        let k = ((travel - d0) / my).round().max(0.0);
        let ratio = (d0 + k * my) / travel;
        if (0.86..=1.14).contains(&ratio) {
            l.vy = sy * base * ratio;
        }
    }

    fn celebrate(&mut self, i: usize) {
        self.corners = self.corners.saturating_add(1);
        self.party_t = 0.0;
        let (mx, my) = (self.max_x(), self.max_y());
        let l = &self.logos[i];
        let left = l.x < mx * 0.5 || mx <= 0.0;
        let top = l.y < my * 0.5 || my <= 0.0;
        // Sparks leave from the logo's edges that face into the screen.
        let (lx, ly) = (l.x.round(), l.y.round());
        let (sw, sh) = (self.sprite.w as f32, self.sprite.h as f32);
        let edge_x = if left { lx + sw } else { lx - 1.0 };
        let edge_y = if top { ly + sh } else { ly - 1.0 };
        let (dx, dy) = (if left { 1.0 } else { -1.0 }, if top { 1.0 } else { -1.0 });
        let n = self.palette.len().max(1);
        let burst = (self.spark_cap * 2 / 3).max(1);
        for _ in 0..burst {
            if self.sparks.len() >= self.spark_cap {
                break;
            }
            // Pick the vertical or horizontal edge by its on-screen length.
            let (x, y) = if self.rng.f32() * (sw + sh * 2.0) < sw {
                (lx + self.rng.f32() * sw, edge_y)
            } else {
                (edge_x, ly + self.rng.f32() * sh)
            };
            // Spread over the quarter circle that points into the screen.
            let a = self.rng.f32() * std::f32::consts::FRAC_PI_2;
            let v = 4.0 + self.rng.f32() * self.rng.f32() * 30.0;
            self.sparks.push(Spark {
                x,
                y,
                vx: dx * a.cos() * v * 2.0,
                vy: dy * a.sin() * v,
                age: 0.0,
                ttl: 1.0 + self.rng.f32() * 1.4,
                color: self.rng.usize(..n) as u8,
            });
        }
    }

    fn step(&mut self, dt: f32) {
        let (mx, my) = (self.max_x(), self.max_y());
        for i in 0..self.logos.len() {
            let (mut bx, mut by) = (false, false);
            {
                let l = &mut self.logos[i];
                l.since_bounce += dt;
                l.since_corner += dt;
                if mx > 0.0 {
                    l.x += l.vx * dt;
                    if l.x < 0.0 {
                        l.x = -l.x;
                        l.vx = l.vx.abs();
                        bx = true;
                    } else if l.x > mx {
                        l.x = 2.0 * mx - l.x;
                        l.vx = -l.vx.abs();
                        bx = true;
                    }
                    l.x = l.x.clamp(0.0, mx);
                }
                if my > 0.0 {
                    l.y += l.vy * dt;
                    if l.y < 0.0 {
                        l.y = -l.y;
                        l.vy = l.vy.abs();
                        by = true;
                    } else if l.y > my {
                        l.y = 2.0 * my - l.y;
                        l.vy = -l.vy.abs();
                        by = true;
                    }
                    l.y = l.y.clamp(0.0, my);
                }
            }
            if !(bx || by) {
                continue;
            }
            let (x, y, since_bounce, since_corner) = {
                let l = &self.logos[i];
                (l.x, l.y, l.since_bounce, l.since_corner)
            };
            // A corner: both walls at once, or the other wall visually touched.
            let near_x = mx <= 0.0 || x < 1.0 || x > mx - 1.0;
            let near_y = my <= 0.0 || y < 0.5 || y > my - 0.5;
            let corner = (bx || near_x) && (by || near_y) && mx > 0.0 && my > 0.0;
            if since_bounce > 0.15 {
                let c = self.other_color(self.logos[i].color);
                self.logos[i].color = c;
            }
            self.logos[i].since_bounce = 0.0;
            if corner && since_corner > 1.0 {
                self.logos[i].since_corner = 0.0;
                self.celebrate(i);
            }
            if bx {
                self.plan(i);
            }
        }

        for s in self.sparks.iter_mut() {
            s.age += dt;
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            s.vy += 9.0 * dt;
            s.vx *= 1.0 - 0.9 * dt;
        }
        self.sparks.retain(|s| s.age < s.ttl);
        self.party_t = (self.party_t + dt).min(PARTY);
        self.clock = (self.clock + dt) % 3600.0;
    }

    fn draw_counter(&self, c: &mut Canvas) {
        let mut buf = [0u8; 24];
        let prefix = b"corners: ";
        buf[..prefix.len()].copy_from_slice(prefix);
        let mut n = self.corners;
        let mut digits = [0u8; 10];
        let mut nd = 0;
        loop {
            digits[nd] = b'0' + (n % 10) as u8;
            nd += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        for k in 0..nd {
            buf[prefix.len() + k] = digits[nd - 1 - k];
        }
        let len = prefix.len() + nd;
        let s = std::str::from_utf8(&buf[..len]).unwrap_or("");
        let x = self.w - len as i32 - 1;
        let y = self.h - 1;
        let fresh = self.party_t < PARTY;
        let col = if fresh {
            self.palette[(self.clock * 10.0) as usize % self.palette.len()]
        } else {
            self.counter_dim
        };
        c.text_attr(x.max(0), y, s, col, if fresh { BOLD } else { 0 });
    }
}

impl Scene for Dvd {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.w = w.max(1) as i32;
        self.h = h.max(1) as i32;
        let (lw, lh) = (LOGO[4].chars().count() as i32, LOGO.len() as i32);
        self.sprite = if !self.text.is_empty() {
            Sprite::fit_text(&self.text, self.w, self.h, 40)
        } else if lw + 6 <= self.w && lh + 3 <= self.h {
            Sprite::from_lines(&LOGO)
        } else {
            Sprite::fit_text("DVD", self.w, self.h, 40)
        };
        self.banner = Sprite::fit_text("CORNER!", self.w, self.h, 50);
        let (mx, my) = (self.max_x(), self.max_y());
        let n = self.palette.len().max(1);
        if self.logos.len() == self.count {
            // A resize keeps the logos where they were, as far as they fit.
            for l in self.logos.iter_mut() {
                l.x = l.x.clamp(0.0, mx);
                l.y = l.y.clamp(0.0, my);
            }
            self.sparks.clear();
            return;
        }
        self.logos.clear();
        for _ in 0..self.count {
            let l = Logo {
                x: self.rng.f32() * mx,
                y: self.rng.f32() * my,
                vx: if self.rng.bool() { 2.0 } else { -2.0 } * self.speed,
                vy: if self.rng.bool() { 1.0 } else { -1.0 } * self.speed,
                color: self.rng.usize(..n),
                since_bounce: 1.0,
                since_corner: 10.0,
            };
            self.logos.push(l);
        }
        self.spark_cap = 180 * self.count.min(3);
        self.sparks = Vec::with_capacity(self.spark_cap);
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.palette.clear();
        if ctx.theme.native {
            self.palette.extend_from_slice(&CLASSIC);
            self.counter_dim = ctx.fade(rgb(0xb0b0b0), 0.45);
        } else {
            let n = ctx.theme.colors.len().max(1);
            self.palette
                .extend((0..n).map(|i| ctx.pick(i, Color::Reset)));
            self.counter_dim = ctx.fade(ctx.theme.fg, 0.45);
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let mut left = dt.clamp(0.0, 2.0);
        while left > 0.0 {
            let h = left.min(1.0 / 60.0);
            self.step(h);
            left -= h;
        }
    }

    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas) {
        let n = self.palette.len().max(1);
        if self.party_t < PARTY {
            // Blink fast at first, then hold and fade out.
            let t = self.party_t;
            let on = t > 1.2 || (t * 7.0) as i32 % 2 == 0;
            if on {
                let col = self.palette[(t * 9.0) as usize % n];
                let col = if t > PARTY - 0.6 {
                    ctx.fade(col, (t - (PARTY - 0.6)) / 0.6)
                } else {
                    col
                };
                let b = &self.banner;
                b.draw(c, (self.w - b.w) / 2, (self.h - b.h) / 2, col, BOLD);
            }
        }
        for s in &self.sparks {
            let k = s.age / s.ttl;
            let mut gi = (k * SPARK_GLYPHS.len() as f32) as usize;
            // Twinkle: now and then a spark flares back up for a frame.
            if ((s.ttl * 97.0 + s.age * 23.0) as u32).is_multiple_of(7) {
                gi = gi.saturating_sub(2);
            }
            let g = SPARK_GLYPHS[gi.min(SPARK_GLYPHS.len() - 1)];
            let col = ctx.fade(self.palette[s.color as usize % n], k * 0.7);
            c.set(s.x.round() as i32, s.y.round() as i32, g, col);
        }
        for l in &self.logos {
            self.sprite.draw(
                c,
                l.x.round() as i32,
                l.y.round() as i32,
                self.palette[l.color % n],
                0,
            );
        }
        // A HUD on top of everything, so a corner hit never hides it.
        if self.show_counter {
            self.draw_counter(c);
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
        torture("count = 64\nspeed = 500.0\ncorner_assist = true");
        torture("text = 'Hello, wörld ♥ 123 !?'\nspeed = 0.0\ncounter = false");
        torture("text = '   '\ncount = 0\ncorner_assist = true\nspeed = 40.0");
        torture(
            "text = 'a very long line of text that will never fit anywhere at all on small screens'",
        );
    }

    #[test]
    fn logo_rows_line_up() {
        let w = LOGO[4].chars().count();
        for l in LOGO {
            assert!(l.chars().count() <= w, "{l}");
        }
    }

    #[test]
    fn font_renders() {
        let s = Sprite::from_text("Hi!", 1);
        assert_eq!(s.h, 3);
        assert_eq!(s.w, Sprite::text_px("Hi!"));
        let s = Sprite::from_text("A", 2);
        assert_eq!((s.w, s.h), (10, 5));
    }
}
