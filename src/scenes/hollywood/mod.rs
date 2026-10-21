//! hollywood: a terminal full of Hollywood-style "hacker" activity.
//!
//! After hollywood by Dustin Kirkland, <https://github.com/dustinkirkland/hollywood>,
//! Copyright 2014-2026 Dustin Kirkland, licensed under the Apache License,
//! Version 2.0 (<http://www.apache.org/licenses/LICENSE-2.0>). The original
//! splits a tmux window into panes that keep re-splitting, each running a
//! widget (hex dumps, log tails, htop, matrix rain, code, trees, keys...).
//! This is a native re-imagining: no tmux and no external commands, every
//! widget is simulated and drawn here, and the technobabble is about ten
//! times thicker (see `babble.rs`). Apache-2.0 is compatible with oxiper's
//! GPL-3.0-only.
//!
//! Structure: the screen is a binary tree of panes (`Node`) whose leaves each
//! own a `Widget`. Every 10 to 25 real seconds one pane splits, two panes
//! merge, or the whole layout reshuffles. Panes tile the screen exactly; each
//! draws its own title bar and left edge, so neighbours share one thin line.

mod babble;
mod feed;
mod map;
mod widgets;

use crate::color::{Color, Gradient, rgb};
use crate::render::{BOLD, Canvas, DIM};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use fastrand::Rng;

pub const INFO: SceneInfo = SceneInfo {
    name: "hollywood",
    aliases: &["hacker", "technobabble"],
    about: "a Hollywood hacker screen full of technobabble, after hollywood",
    options: &[
        Opt {
            key: "speed",
            default: Val::F(1.0),
            help: "Motion pace of the widgets",
        },
        Opt {
            key: "panes",
            default: Val::I(6),
            help: "Most panes on screen at once (3 to 12)",
        },
        Opt {
            key: "reshuffle",
            default: Val::F(15.0),
            help: "Average seconds between layout changes (real time)",
        },
        Opt {
            key: "babble",
            default: Val::I(7),
            help: "How much technobabble, 0 (plain logs) to 10 (maximum)",
        },
    ],
    make: |ctx| Box::new(Hollywood::new(ctx)),
};

/// A rectangle in cells.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    fn area(self) -> i32 {
        self.w * self.h
    }
}

/// A semantic color; widgets store these so a theme change repaints them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tone {
    Main,
    Dim,
    Bright,
    Cyan,
    Amber,
    Red,
    Green,
}

/// The palette: the classic hacker greens under `classic`, theme colors otherwise.
#[derive(Clone)]
pub struct Pal {
    pub main: Color,
    pub dim: Color,
    pub bright: Color,
    pub cyan: Color,
    pub amber: Color,
    pub red: Color,
    pub green: Color,
    /// Text drawn on top of a colored background (banners, table headers).
    pub ink: Color,
    pub ramp: Gradient,
    bg: Color,
}

impl Pal {
    fn new(ctx: &Ctx) -> Pal {
        let native = ctx.theme.native;
        let main = if native { rgb(0x33ee66) } else { ctx.theme.fg };
        Pal {
            main,
            dim: ctx.fade(main, 0.5),
            bright: if native {
                rgb(0xcfffd8)
            } else {
                ctx.pick(5, main)
            },
            cyan: ctx.pick(4, rgb(0x33d6e8)),
            amber: ctx.pick(2, rgb(0xffb000)),
            red: ctx.pick(0, rgb(0xff4545)),
            green: ctx.pick(3, rgb(0x66ff66)),
            ink: if native { rgb(0x04110a) } else { ctx.theme.bg },
            ramp: ctx.ramp(&[rgb(0x03140a), rgb(0x0f8a35), rgb(0x33ee66), rgb(0xd8ffe2)]),
            bg: ctx.bg,
        }
    }

    #[inline]
    pub fn tone(&self, t: Tone) -> Color {
        match t {
            Tone::Main => self.main,
            Tone::Dim => self.dim,
            Tone::Bright => self.bright,
            Tone::Cyan => self.cyan,
            Tone::Amber => self.amber,
            Tone::Red => self.red,
            Tone::Green => self.green,
        }
    }

    /// Blends toward the terminal background (`k = 1` vanishes).
    pub fn fade(&self, c: Color, k: f32) -> Color {
        match self.bg {
            Color::Rgb(..) => c.lerp(self.bg, k),
            _ => c.scale(1.0 - k),
        }
    }
}

/// What every widget may read: the palette and the babble level.
pub struct Env {
    pub pal: Pal,
    pub level: u8,
}

/// One thing running in a pane.
pub trait Widget {
    /// The "process" shown in the title.
    fn name(&self) -> &'static str;
    /// The pane's content area changed size.
    fn resize(&mut self, _w: u16, _h: u16) {}
    /// Advances by `dt` seconds (already speed-scaled and capped).
    fn update(&mut self, dt: f32, env: &Env);
    /// Draws into `r`; everything must stay inside it.
    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env);
}

/// Draws `s` at (`x`, `y`) relative to `r`, clipped to it. Returns the next x.
pub fn put(c: &mut Canvas, r: Rect, x: i32, y: i32, s: &str, col: Color, attr: u8) -> i32 {
    let mut cx = x;
    if y < 0 || y >= r.h {
        return cx;
    }
    for ch in s.chars() {
        if cx >= r.w {
            break;
        }
        if cx >= 0 {
            c.set_attr(r.x + cx, r.y + y, ch, col, attr);
        }
        cx += 1;
    }
    cx
}

/// Sets one cell relative to `r`, clipped.
#[inline]
pub fn put_ch(c: &mut Canvas, r: Rect, x: i32, y: i32, ch: char, col: Color, attr: u8) {
    if x >= 0 && y >= 0 && x < r.w && y < r.h {
        c.set_attr(r.x + x, r.y + y, ch, col, attr);
    }
}

const MIN_W: i32 = 30;
const MIN_H: i32 = 7;

/// Widget kinds; `make_widget` builds each.
const KINDS: usize = 19;

struct Pane {
    rect: Rect,
    kind: usize,
    widget: Box<dyn Widget>,
    title: String,
    /// Seconds until the title's babble changes.
    retitle: f32,
    /// Seconds since the pane appeared (new panes flash their border).
    age: f32,
    content: (u16, u16),
}

enum Node {
    Leaf(Box<Pane>),
    Split {
        /// Side by side (true) or stacked (false).
        vert: bool,
        frac: f32,
        a: Box<Node>,
        b: Box<Node>,
    },
}

fn divide(r: Rect, vert: bool, frac: f32) -> (Rect, Rect) {
    if vert {
        let wa = ((r.w as f32 * frac).round() as i32).clamp(1, (r.w - 1).max(1));
        (
            Rect { w: wa, ..r },
            Rect {
                x: r.x + wa,
                w: r.w - wa,
                ..r
            },
        )
    } else {
        let ha = ((r.h as f32 * frac).round() as i32).clamp(1, (r.h - 1).max(1));
        (
            Rect { h: ha, ..r },
            Rect {
                y: r.y + ha,
                h: r.h - ha,
                ..r
            },
        )
    }
}

impl Node {
    /// Assigns rectangles and tells widgets whose content size changed.
    fn layout(&mut self, r: Rect) {
        match self {
            Node::Leaf(p) => {
                p.rect = r;
                let cw = (r.w - 1).max(0) as u16;
                let ch = (r.h - 1).max(0) as u16;
                if p.content != (cw, ch) {
                    p.content = (cw, ch);
                    p.widget.resize(cw, ch);
                }
            }
            Node::Split { vert, frac, a, b } => {
                let (ra, rb) = divide(r, *vert, *frac);
                a.layout(ra);
                b.layout(rb);
            }
        }
    }

    fn count(&self) -> usize {
        match self {
            Node::Leaf(_) => 1,
            Node::Split { a, b, .. } => a.count() + b.count(),
        }
    }

    fn each<'a>(&'a self, f: &mut impl FnMut(&'a Pane)) {
        match self {
            Node::Leaf(p) => f(p),
            Node::Split { a, b, .. } => {
                a.each(f);
                b.each(f);
            }
        }
    }

    fn each_mut(&mut self, f: &mut impl FnMut(&mut Pane)) {
        match self {
            Node::Leaf(p) => f(p),
            Node::Split { a, b, .. } => {
                a.each_mut(f);
                b.each_mut(f);
            }
        }
    }

    /// Rectangles of the leaves that are big enough to split, in order.
    fn splittable(&self, r: Rect, out: &mut Vec<Rect>) {
        match self {
            Node::Leaf(_) => {
                if r.w >= 2 * MIN_W || r.h >= 2 * MIN_H {
                    out.push(r);
                }
            }
            Node::Split { vert, frac, a, b } => {
                let (ra, rb) = divide(r, *vert, *frac);
                a.splittable(ra, out);
                b.splittable(rb, out);
            }
        }
    }

    /// Splits the leaf whose rectangle is `target`; `fresh` makes the new pane.
    fn split_at(
        &mut self,
        r: Rect,
        target: Rect,
        vert: bool,
        frac: f32,
        fresh: &mut Option<Box<Pane>>,
    ) {
        match self {
            Node::Leaf(_) => {
                if r == target
                    && let Some(new) = fresh.take()
                {
                    let old = std::mem::replace(self, Node::Leaf(Box::new(dummy_pane())));
                    let new = Node::Leaf(new);
                    // Randomly put the newcomer first or second.
                    *self = if frac < 0.5 {
                        Node::Split {
                            vert,
                            frac: 1.0 - frac,
                            a: Box::new(new),
                            b: Box::new(old),
                        }
                    } else {
                        Node::Split {
                            vert,
                            frac,
                            a: Box::new(old),
                            b: Box::new(new),
                        }
                    };
                }
            }
            Node::Split {
                vert: v,
                frac: f,
                a,
                b,
            } => {
                let (ra, rb) = divide(r, *v, *f);
                a.split_at(ra, target, vert, frac, fresh);
                b.split_at(rb, target, vert, frac, fresh);
            }
        }
    }

    /// Number of splits whose two children are both leaves.
    fn mergeable(&self) -> usize {
        match self {
            Node::Leaf(_) => 0,
            Node::Split { a, b, .. } => {
                if matches!((&**a, &**b), (Node::Leaf(_), Node::Leaf(_))) {
                    1
                } else {
                    a.mergeable() + b.mergeable()
                }
            }
        }
    }

    /// Collapses the `n`-th mergeable split into one of its two leaves.
    fn merge_nth(&mut self, n: &mut usize, keep_first: bool) {
        if let Node::Split { a, b, .. } = self {
            if matches!((&**a, &**b), (Node::Leaf(_), Node::Leaf(_))) {
                if *n == 0 {
                    let keep = if keep_first {
                        std::mem::replace(&mut **a, Node::Leaf(Box::new(dummy_pane())))
                    } else {
                        std::mem::replace(&mut **b, Node::Leaf(Box::new(dummy_pane())))
                    };
                    *self = keep;
                }
                *n = n.wrapping_sub(1);
            } else {
                a.merge_nth(n, keep_first);
                b.merge_nth(n, keep_first);
            }
        }
    }

    #[allow(clippy::vec_box)]
    fn into_panes(self, out: &mut Vec<Box<Pane>>) {
        match self {
            Node::Leaf(p) => out.push(p),
            Node::Split { a, b, .. } => {
                a.into_panes(out);
                b.into_panes(out);
            }
        }
    }
}

/// A throwaway placeholder used while moving nodes around (never drawn).
fn dummy_pane() -> Pane {
    Pane {
        rect: Rect::default(),
        kind: 0,
        widget: Box::new(widgets::Nothing),
        title: String::new(),
        retitle: 0.0,
        age: 0.0,
        content: (0, 0),
    }
}

fn make_widget(kind: usize, seed: u64, level: u8) -> Box<dyn Widget> {
    use feed::{Feed, FeedKind as K};
    match kind {
        0 => Box::new(Feed::new(K::Hex, seed, level)),
        1 => Box::new(Feed::new(K::Log, seed, level)),
        2 => Box::new(Feed::new(K::Code, seed, level)),
        3 => Box::new(Feed::new(K::Errno, seed, level)),
        4 => Box::new(Feed::new(K::Sniffer, seed, level)),
        5 => Box::new(Feed::new(K::Tree, seed, level)),
        6 => Box::new(Feed::new(K::Dna, seed, level)),
        7 => Box::new(Feed::new(K::Pem, seed, level)),
        8 => Box::new(Feed::new(K::Mission, seed, level)),
        9 => Box::new(widgets::Htop::new(seed)),
        10 => Box::new(widgets::Bandwidth::new(seed)),
        11 => Box::new(widgets::Rain::new(seed)),
        12 => Box::new(widgets::WorldMap::new(seed)),
        13 => Box::new(widgets::Radar::new(seed)),
        14 => Box::new(widgets::Progress::new(seed)),
        15 => Box::new(widgets::Satellites::new(seed)),
        16 => Box::new(widgets::Ticker::new(seed)),
        17 => Box::new(widgets::Access::new(seed, level)),
        _ => Box::new(widgets::Keygen::new(seed, level)),
    }
}

pub struct Hollywood {
    rng: Rng,
    env: Env,
    root: Option<Node>,
    w: i32,
    h: i32,
    max_panes: usize,
    reshuffle: f32,
    speed: f32,
    /// Real seconds until the next layout change.
    next_event: f32,
    /// The mission ticker along the bottom row.
    tape: Vec<char>,
    tape_off: f32,
    tape_buf: String,
}

impl Hollywood {
    fn new(ctx: &Ctx) -> Hollywood {
        let o = ctx.opts;
        let mut rng = ctx.rng();
        let reshuffle = o.f("reshuffle");
        let reshuffle = if reshuffle.is_finite() {
            reshuffle.clamp(1.0, 600.0)
        } else {
            15.0
        };
        let speed = o.f("speed");
        let mut s = Hollywood {
            env: Env {
                pal: Pal::new(ctx),
                level: o.i("babble").clamp(0, 10) as u8,
            },
            max_panes: o.i("panes").clamp(3, 12) as usize,
            reshuffle,
            speed: if speed.is_finite() {
                speed.clamp(0.0, 20.0)
            } else {
                1.0
            },
            next_event: 0.0,
            root: None,
            w: 0,
            h: 0,
            tape: Vec::new(),
            tape_off: 0.0,
            tape_buf: String::new(),
            rng: Rng::with_seed(0),
        };
        s.next_event = Self::interval(&mut rng, reshuffle);
        s.rng = rng;
        s
    }

    /// A jittered wait: with the default 15 s it lands between about 10 and 25.
    fn interval(rng: &mut Rng, reshuffle: f32) -> f32 {
        reshuffle * (0.66 + rng.f32() * 1.0)
    }

    /// The area panes tile: everything but the ticker row.
    fn area(&self) -> Rect {
        let ticker = if self.h >= 8 { 1 } else { 0 };
        Rect {
            x: 0,
            y: 0,
            w: self.w,
            h: (self.h - ticker).max(0),
        }
    }

    fn new_pane(&mut self, taken: &[usize]) -> Box<Pane> {
        let mut kind = self.rng.usize(..KINDS);
        for _ in 0..24 {
            if !taken.contains(&kind) {
                break;
            }
            kind = self.rng.usize(..KINDS);
        }
        let seed = self.rng.u64(..);
        let mut p = Pane {
            rect: Rect::default(),
            kind,
            widget: make_widget(kind, seed, self.env.level),
            title: String::new(),
            retitle: 0.0,
            age: 0.0,
            content: (u16::MAX, u16::MAX),
        };
        Self::retitle(&mut p, &mut self.rng, self.env.level);
        Box::new(p)
    }

    fn retitle(p: &mut Pane, rng: &mut Rng, level: u8) {
        p.title.clear();
        p.title.push_str(p.widget.name());
        if level > 0 {
            p.title.push_str(" \u{00b7} ");
            babble::title(rng, &mut p.title);
        }
        p.retitle = 8.0 + rng.f32() * 14.0;
    }

    /// Builds a fresh layout of `n` panes from `pool` (reused panes first).
    #[allow(clippy::vec_box)]
    fn build(&mut self, mut pool: Vec<Box<Pane>>, n: usize) {
        let area = self.area();
        let first = pool.pop().unwrap_or_else(|| self.new_pane(&[]));
        let mut root = Node::Leaf(first);
        for _ in 1..n {
            let mut cand = Vec::new();
            root.splittable(area, &mut cand);
            if cand.is_empty() {
                break;
            }
            let pane = pool.pop().unwrap_or_else(|| {
                let t = {
                    let mut v = Vec::new();
                    root.each(&mut |p| v.push(p.kind));
                    v
                };
                self.new_pane(&t)
            });
            Self::split_random(&mut root, area, &cand, pane, &mut self.rng);
        }
        root.layout(area);
        self.root = Some(root);
    }

    /// Splits one of the candidate leaves (bigger ones are likelier).
    fn split_random(root: &mut Node, area: Rect, cand: &[Rect], pane: Box<Pane>, rng: &mut Rng) {
        let total: i32 = cand.iter().map(|r| r.area()).sum();
        let mut pick = rng.i32(0..total.max(1));
        let mut target = cand[0];
        for r in cand {
            if pick < r.area() {
                target = *r;
                break;
            }
            pick -= r.area();
        }
        // Cells are about twice as tall as wide: compare in square-ish units.
        let can_v = target.w >= 2 * MIN_W;
        let can_h = target.h >= 2 * MIN_H;
        let vert = match (can_v, can_h) {
            (true, false) => true,
            (false, true) => false,
            _ => (target.w as f32 / 2.0 > target.h as f32) != rng.u8(..5).eq(&0),
        };
        let frac = 0.38 + rng.f32() * 0.24;
        let mut fresh = Some(pane);
        root.split_at(area, target, vert, frac, &mut fresh);
    }

    #[cfg(test)]
    fn pane_count(&self) -> usize {
        self.root.as_ref().map_or(0, Node::count)
    }

    /// One layout change: a pane splits, two merge, or everything reshuffles.
    fn change_layout(&mut self) {
        let Some(mut root) = self.root.take() else {
            return;
        };
        let n = root.count();
        let area = self.area();
        let roll = self.rng.u8(..100);
        let mut cand = Vec::new();
        root.splittable(area, &mut cand);
        if roll < 40 && n < self.max_panes && !cand.is_empty() {
            let taken: Vec<usize> = {
                let mut v = Vec::new();
                root.each(&mut |p| v.push(p.kind));
                v
            };
            let pane = self.new_pane(&taken);
            Self::split_random(&mut root, area, &cand, pane, &mut self.rng);
            root.layout(area);
            self.root = Some(root);
        } else if roll < 65 && n > 3.min(self.max_panes) && root.mergeable() > 0 {
            let mut k = self.rng.usize(..root.mergeable());
            let keep_first = self.rng.bool();
            root.merge_nth(&mut k, keep_first);
            root.layout(area);
            self.root = Some(root);
        } else {
            // Full reshuffle: keep some widgets, replace others, new tiling.
            let mut pool = Vec::new();
            root.into_panes(&mut pool);
            self.rng.shuffle(&mut pool);
            let want = self.rng.usize(3.min(self.max_panes)..=self.max_panes);
            let keep = pool.len().min(want.saturating_sub(1));
            pool.truncate(keep);
            for p in pool.iter_mut() {
                p.age = 0.0;
            }
            self.build(pool, want);
        }
    }

    fn feed_tape(&mut self) {
        let need = self.w.max(1) as usize + 80;
        while self.tape.len() < need {
            self.tape_buf.clear();
            self.tape_buf.push_str("MISSION \u{25c6} ");
            let mut s = std::mem::take(&mut self.tape_buf);
            babble::gerund_phrase(&mut self.rng, self.env.level, &mut s);
            s.make_ascii_uppercase();
            s.push_str("  \u{25c6}  ");
            self.tape.extend(s.chars());
            self.tape_buf = s;
        }
    }

    #[cfg(test)]
    fn rects(&self) -> Vec<Rect> {
        let mut v = Vec::new();
        if let Some(r) = &self.root {
            r.each(&mut |p| v.push(p.rect));
        }
        v
    }
}

impl Scene for Hollywood {
    fn resize(&mut self, _ctx: &Ctx, w: u16, h: u16) {
        self.w = w as i32;
        self.h = h as i32;
        let mut pool = Vec::new();
        if let Some(r) = self.root.take() {
            r.into_panes(&mut pool);
        }
        for p in pool.iter_mut() {
            p.content = (u16::MAX, u16::MAX);
        }
        let want = if pool.is_empty() {
            self.rng.usize(4.min(self.max_panes)..=self.max_panes)
        } else {
            pool.len().max(3.min(self.max_panes))
        };
        self.build(pool, want.min(self.max_panes));
        self.tape.clear();
        self.tape_off = 0.0;
    }

    fn retheme(&mut self, ctx: &Ctx) {
        self.env.pal = Pal::new(ctx);
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = (dt * self.speed).clamp(0.0, 0.5);
        let env = &self.env;
        if let Some(r) = &mut self.root {
            r.each_mut(&mut |p| {
                p.widget.update(dt, env);
                p.age += dt;
            });
        }
        // Pane titles drift on real time in `tick`; the tape moves with motion.
        self.tape_off += dt * 14.0;
        let n = self.tape_off as usize;
        if n > 0 {
            self.tape_off -= n as f32;
            let n = n.min(self.tape.len());
            self.tape.drain(..n);
        }
    }

    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        self.update(ctx, dt);
        let real = real.clamp(0.0, 30.0);
        self.next_event -= real;
        if self.next_event <= 0.0 && self.w > 0 {
            self.change_layout();
            self.next_event = Self::interval(&mut self.rng, self.reshuffle);
        }
        let level = self.env.level;
        if let Some(r) = &mut self.root {
            let rng = &mut self.rng;
            r.each_mut(&mut |p| {
                p.retitle -= real;
                if p.retitle <= 0.0 {
                    Self::retitle(p, rng, level);
                }
            });
        }
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        self.feed_tape();
        let env = &self.env;
        let pal = &env.pal;
        if let Some(root) = &mut self.root {
            root.each_mut(&mut |p| {
                let r = p.rect;
                if r.w < 3 || r.h < 3 {
                    return;
                }
                // New panes light their border up for a moment.
                let edge = if p.age < 1.0 {
                    pal.fade(pal.bright, p.age * 0.5)
                } else {
                    pal.fade(pal.main, 0.45)
                };
                c.set(r.x, r.y, '\u{250c}', edge);
                for x in 1..r.w {
                    c.set(r.x + x, r.y, '\u{2500}', edge);
                }
                for y in 1..r.h {
                    c.set(r.x, r.y + y, '\u{2502}', edge);
                }
                let tw = (r.w - 5).max(0) as usize;
                if tw >= 4 {
                    let mut x = r.x + 2;
                    c.set(x, r.y, '\u{2524}', edge);
                    x += 1;
                    let mut name = true;
                    for (i, ch) in p.title.chars().enumerate() {
                        if i >= tw {
                            break;
                        }
                        if ch == '\u{00b7}' {
                            name = false;
                        }
                        let (col, at) = if name {
                            (pal.bright, BOLD)
                        } else {
                            (pal.cyan, DIM)
                        };
                        c.set_attr(x, r.y, ch, col, at);
                        x += 1;
                    }
                    c.set(x, r.y, '\u{251c}', edge);
                }
                let content = Rect {
                    x: r.x + 1,
                    y: r.y + 1,
                    w: r.w - 1,
                    h: r.h - 1,
                };
                p.widget.draw(c, content, env);
            });
        }
        // The mission ticker.
        if self.h >= 8 {
            let y = self.h - 1;
            for (i, &ch) in self.tape.iter().take(self.w as usize).enumerate() {
                let (col, at) = if ch == '\u{25c6}' {
                    (pal.amber, BOLD)
                } else {
                    (pal.cyan, 0)
                };
                c.set_attr(i as i32, y, ch, col, at);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    fn make(opts: &str, theme: Theme) -> (Hollywood, crate::scene::Opts) {
        let table: toml::Table = opts.parse().unwrap();
        let o = crate::scene::Opts::new(INFO.options, table);
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 11,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        (Hollywood::new(&ctx), o)
    }

    fn with_ctx<R>(o: &crate::scene::Opts, f: impl FnOnce(&Ctx) -> R) -> R {
        let theme = Theme::classic();
        let ctx = Ctx {
            theme: &theme,
            opts: o,
            bg: Color::Reset,
            seed: 11,
            fiesta: crate::fiesta::Fiesta::NONE,
        };
        f(&ctx)
    }

    #[test]
    fn panes_tile_the_screen() {
        for &(w, h) in &[
            (140u16, 40u16),
            (80, 24),
            (60, 20),
            (200, 60),
            (30, 10),
            (1, 1),
            (2, 3),
        ] {
            let (mut s, o) = make("", Theme::classic());
            with_ctx(&o, |ctx| {
                s.resize(ctx, w, h);
                for step in 0..300 {
                    s.tick(ctx, 0.05, 1.0);
                    if step % 3 == 0 {
                        let area = s.area();
                        let rects = s.rects();
                        let sum: i32 = rects.iter().map(|r| r.area()).sum();
                        assert_eq!(sum, area.area(), "{w}x{h} step {step}");
                        for (i, a) in rects.iter().enumerate() {
                            assert!(
                                a.x >= 0 && a.y >= 0 && a.x + a.w <= area.w && a.y + a.h <= area.h
                            );
                            for b in &rects[i + 1..] {
                                let overlap = a.x < b.x + b.w
                                    && b.x < a.x + a.w
                                    && a.y < b.y + b.h
                                    && b.y < a.y + a.h;
                                assert!(!overlap, "{a:?} overlaps {b:?}");
                            }
                        }
                    }
                }
            });
        }
    }

    #[test]
    fn layout_changes_follow_real_time_only() {
        let (mut s, o) = make("reshuffle = 10", Theme::classic());
        with_ctx(&o, |ctx| {
            s.resize(ctx, 140, 40);
            let first = s.next_event;
            assert!((6.0..=17.0).contains(&first));
            // Lots of scene time, no real time: nothing changes.
            for _ in 0..50 {
                s.tick(ctx, 5.0, 0.0);
            }
            assert_eq!(s.next_event, first);
            // Real time does it.
            let mut changed = false;
            for _ in 0..40 {
                let before = s.next_event;
                s.tick(ctx, 0.0, 1.0);
                changed |= s.next_event > before;
            }
            assert!(changed);
        });
    }

    #[test]
    fn pane_limit_is_respected() {
        let (mut s, o) = make("panes = 4", Theme::classic());
        with_ctx(&o, |ctx| {
            s.resize(ctx, 200, 60);
            for _ in 0..200 {
                s.tick(ctx, 0.1, 5.0);
                assert!(s.pane_count() <= 4 && s.pane_count() >= 1);
            }
        });
    }
}
