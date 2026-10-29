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
//! On Día de Muertos the screen goes Mictlán: the hacker greens turn to
//! cempasúchil, rosa mexicano, turquesa and morado, every pane hangs a string
//! of papel picado from its title bar, the ticker along the bottom brings the
//! headlines of the underworld ("MICTLÁN GATEWAY ONLINE", "UPLOADING
//! PAN_DE_MUERTO.TAR.GZ") and the technobabble goes bilingual (`muertos.rs`:
//! "sincronizando la ofrenda cuántica con el Mictlán", "marigold petals
//! routed through the quantum firewall"). The widgets get holiday versions:
//! a radar and a world map that track the souls coming home, a gateway with
//! the nine levels of the Mictlán and an ofrenda database with its altar
//! (`mictlan.rs`), a facial recognition that scans sugar skulls
//! (`rostro.rs`), the stock market of marigolds and candles, flowers in the
//! randomart, rain that spells MICTLAN, and "ACCESO AL MICTLÁN DENEGADO" in
//! big letters. The higher the holiday, the more of the text is the
//! Mictlán's, the more souls are on the move, the more candles are lit and
//! the more flags hang from the strings; from October 31 the angelitos come
//! home too. Ordinary days are untouched (the holiday draws on its own
//! random numbers).
//!
//! On Navidad, the Guadalupe–Reyes, the screen strings a series of foquitos
//! along every title bar (a bulb every fourth column, then every third and,
//! at Nochebuena, every other, blinking in a chase) and the texts go
//! Spanish about posadas, piñatas, ponche and Reyes Magos (`navidad.rs` and
//! `lexico.rs`, the same grammar as the Mictlán's), with the villancicos
//! scrolling through the logs verse by verse. Four widgets are the holiday's
//! (`posada.rs`): "RASTREO DE LOS REYES MAGOS", Melchor, Gaspar and Baltasar
//! riding to Mexico under the Estrella de Belén, farther along every day
//! with an ETA to January 6; the database of cartas a los Reyes with a globo
//! de cantoya carrying one up; the piñata's impact sensor ("golpes: 7/9
//! ¡dale!") beside the piñata of seven points swinging, bursting and raining
//! sweets; and the ponche's thermometer over its olla, hottest on Nochebuena.
//! The other widgets are renamed and re-worded for the posada (shepherds on
//! the radar, aguinaldos on the map, a market of ponche and piñatas, the
//! access banner that answers "ENTREN, SANTOS PEREGRINOS" or "AQUÍ NO ES
//! MESÓN"). The posadas count up to Nochebuena and the Reyes close in
//! through January, with headlines for Año Nuevo and the rosca. On December
//! 28, the Santos Inocentes, the whole screen now and then turns red with
//! "SISTEMA COMPROMETIDO" and a wipe of /dev/sda1 until it owns up:
//! "¡INOCENTE PALOMITA!" (`inocentes.rs`). Ordinary days are untouched.
//!
//! On the Fiestas Patrias, the mes patrio, the screen goes verde, blanco y
//! rojo: white text, tricolor banderitas along every title bar (a flag every
//! third column, then every other and, on the night of the Grito, one after
//! another), headlines on the ticker ("GRITO.EXE: CUENTA REGRESIVA A LAS
//! 23:00", "¡VIVA MÉXICO!"), and technobabble about the Grito, the campana de
//! Dolores, the insurgentes and the pozole (`patrias.rs`) with the Himno
//! Nacional scrolling through the logs. Five widgets are the holiday's
//! (`grito.rs`): GRITO.EXE, a countdown to 23:00 of September 15 on the
//! scene's own sense of the day (`Env::day`, started by the date) that says
//! the Grito line by line when it runs out; the control panel of the campana
//! de Dolores beside a pixel bell that swings and rings with it; the
//! decibel meter of the ¡VIVA! the crowd answers; the route of the parade of
//! the 16th with its contingents marching; and the history database of the
//! Independence (Hidalgo, Allende, Josefa Ortiz de Domínguez, Morelos, 1810)
//! with a flag waving beside it. The other widgets are renamed and re-worded
//! (insurgentes on the radar, echoes of the Grito on the map, a market of
//! pozole and banderas, "FALTA TU VIVA" on the access banner). Every widget
//! follows the same clock, so the bell rings and the meter peaks when the
//! Grito is given; the month builds up the crowd, the contingents and the
//! flags. Ordinary days are untouched.
//!
//! Structure: the screen is a binary tree of panes (`Node`) whose leaves each
//! own a `Widget`. Every 10 to 25 real seconds one pane splits, two panes
//! merge, or the whole layout reshuffles. Panes tile the screen exactly; each
//! draws its own title bar and left edge, so neighbours share one thin line.

mod babble;
mod feed;
mod grito;
mod inocentes;
mod lexico;
mod map;
mod mictlan;
mod muertos;
mod navidad;
mod patrias;
mod posada;
mod rostro;
mod sprite;
mod widgets;

use crate::color::{Color, Gradient, rgb};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Canvas, DIM};
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};
use fastrand::Rng;
use inocentes::Fool;

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
    /// The six colors of the garland a holiday hangs from the title bars
    /// (the theme's first six accents unless `classic`): Día de Muertos'
    /// papel picado, Navidad's series of foquitos, the tricolor banderitas
    /// of the Fiestas Patrias and the Revolución.
    pub picado: [Color; 6],
    /// Día de Muertos: a marigold and a candle flame, which keep their own
    /// hue under every theme.
    pub flor: Color,
    pub vela: Color,
    /// Navidad, Fiestas Patrias, Revolución: the holiday's gold (the star of
    /// Belén, the bell of Dolores, a brass cartridge), also in its own hue
    /// under every theme.
    pub oro: Color,
    /// Navidad: the papel de china of the piñata (the theme's accents unless
    /// `classic`).
    pub papel: [Color; 6],
    bg: Color,
}

/// The classic palette of the Mictlán: cempasúchil text, rosa mexicano for
/// what is denied, turquesa, amarillo and verde limón for the rest.
const MX_RAMP: [Color; 6] = [
    rgb(0x24104a),
    rgb(0x7028a0),
    rgb(0xe4007c),
    rgb(0xff8c00),
    rgb(0xffd200),
    rgb(0xfff2d0),
];

/// Papel picado under `classic`: rosa mexicano, naranja, morado, turquesa,
/// amarillo and verde limón.
const PICADO: [Color; 6] = [
    rgb(0xe4007c),
    rgb(0xff8c00),
    rgb(0x8a3cc8),
    rgb(0x00aab8),
    rgb(0xffd200),
    rgb(0x8cc828),
];

/// The papel de china of a piñata under `classic`: rosa mexicano, amarillo,
/// naranja, turquesa, morado and verde limón.
const PAPEL: [Color; 6] = [
    rgb(0xe4007c),
    rgb(0xffd200),
    rgb(0xff8c00),
    rgb(0x00aab8),
    rgb(0x7028a0),
    rgb(0x8cc828),
];

/// The classic colors of a holiday after Día de Muertos (all `0xrrggbb`).
struct Tints {
    main: u32,
    bright: u32,
    cyan: u32,
    amber: u32,
    red: u32,
    green: u32,
    ink: u32,
    ramp: [u32; 5],
    garland: [u32; 6],
    oro: u32,
}

/// Navidad: verde de los foquitos for the text, blanco, rojo nochebuena
/// and dorado, the series lights in their six colors.
const NAVIDAD: Tints = Tints {
    main: 0x3bd16f,
    bright: 0xf4f1ea,
    cyan: 0x4d8dff,
    amber: 0xf2b632,
    red: 0xff3b3b,
    green: 0x6be28f,
    ink: 0x07140b,
    ramp: [0x0c2a16, 0x1f6f3a, 0xc8102e, 0xf2b632, 0xf4f1ea],
    garland: [0xff3b3b, 0x3bd16f, 0x3b7bff, 0xffd23b, 0xff5fb4, 0xff8c1a],
    oro: 0xf2b632,
};

/// Fiestas Patrias: blanco for the text, verde bandera and rojo for what
/// the flag is made of, oro for the bell and the embroidery.
const PATRIAS: Tints = Tints {
    main: 0xf4f1ea,
    bright: 0xffffff,
    cyan: 0x1fb26a,
    amber: 0xe3b023,
    red: 0xe8283c,
    green: 0x2fd27c,
    ink: 0x07140d,
    ramp: [0x06201a, 0x006847, 0xf4f1ea, 0xce1126, 0xff4a5a],
    garland: [0x008f5d, 0xf4f1ea, 0xce1126, 0x008f5d, 0xf4f1ea, 0xce1126],
    oro: 0xd4a017,
};

impl Tints {
    /// The colors of the day's holiday, if it is one of those.
    fn of(f: Fiesta) -> Option<&'static Tints> {
        match f.kind {
            Kind::Navidad => Some(&NAVIDAD),
            Kind::Patrias => Some(&PATRIAS),
            _ => None,
        }
    }
}

impl Pal {
    fn new(ctx: &Ctx) -> Pal {
        let native = ctx.theme.native;
        let mx = ctx.fiesta.is(Kind::Muertos);
        let tint = Tints::of(ctx.fiesta);
        let main = match (native, mx, tint) {
            (true, true, _) => rgb(0xffa01e),
            (true, false, Some(t)) => rgb(t.main),
            (true, false, None) => rgb(0x33ee66),
            _ => ctx.theme.fg,
        };
        let pick = |i: usize, plain: u32, holiday: u32, tinted: Option<u32>| {
            ctx.pick(
                i,
                rgb(if mx {
                    holiday
                } else if let Some(t) = tinted {
                    t
                } else {
                    plain
                }),
            )
        };
        Pal {
            main,
            dim: ctx.fade(main, 0.5),
            bright: if native {
                rgb(if mx {
                    0xffecb8
                } else if let Some(t) = tint {
                    t.bright
                } else {
                    0xcfffd8
                })
            } else {
                ctx.pick(5, main)
            },
            cyan: pick(4, 0x33d6e8, 0x00c0cc, tint.map(|t| t.cyan)),
            amber: pick(2, 0xffb000, 0xffd200, tint.map(|t| t.amber)),
            red: pick(0, 0xff4545, 0xf0208c, tint.map(|t| t.red)),
            green: pick(3, 0x66ff66, 0x8cc828, tint.map(|t| t.green)),
            ink: if native {
                rgb(if mx {
                    0x120e28
                } else if let Some(t) = tint {
                    t.ink
                } else {
                    0x04110a
                })
            } else {
                ctx.theme.bg
            },
            ramp: if mx {
                ctx.ramp(&MX_RAMP)
            } else if let Some(t) = tint {
                ctx.ramp(&t.ramp.map(rgb))
            } else {
                ctx.ramp(&[rgb(0x03140a), rgb(0x0f8a35), rgb(0x33ee66), rgb(0xd8ffe2)])
            },
            picado: match tint {
                Some(t) if !mx => std::array::from_fn(|i| ctx.pick(i, rgb(t.garland[i]))),
                _ => std::array::from_fn(|i| ctx.pick(i, PICADO[i])),
            },
            flor: rgb(0xff8c00),
            vela: rgb(0xffd678),
            oro: rgb(tint.map_or(0xf2b632, |t| t.oro)),
            papel: std::array::from_fn(|i| ctx.pick(i, PAPEL[i])),
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

/// What every widget may read: the palette, the babble level and the scene's
/// own sense of the day.
pub struct Env {
    pub pal: Pal,
    pub level: u8,
    /// Seconds into the day, by the scene's clock (which runs on real time):
    /// the Grito of the Fiestas Patrias is at 23:00 (82,800).
    pub day: f32,
}

/// The holiday as the screen cares about it: Día de Muertos, Navidad (and
/// the rest as they are dressed); any other day looks like an ordinary one.
fn holiday(ctx: &Ctx) -> Fiesta {
    if ctx.fiesta.is(Kind::Muertos) || Tints::of(ctx.fiesta).is_some() {
        ctx.fiesta
    } else {
        Fiesta::NONE
    }
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

/// The first `n` characters of `s`.
pub fn truncate(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

const MIN_W: i32 = 30;
const MIN_H: i32 = 7;

/// Widget kinds; `make_widget` builds each.
const KINDS: usize = 19;
/// On Día de Muertos three more: the Mictlán gateway, the ofrenda database
/// and the facial recognition of sugar skulls.
const KINDS_MX: usize = 22;
const RADAR: usize = 13;
const GATEWAY: usize = 19;
const OFRENDA: usize = 20;
const FACE: usize = 21;
/// The widgets that make a screen read as Día de Muertos at a glance; every
/// layout of the holiday keeps at least one of them.
const SIGNATURE: [usize; 4] = [RADAR, GATEWAY, OFRENDA, FACE];
/// Navidad has four of its own: the tracking of the Reyes Magos, the cartas
/// a los Reyes, the piñata's impact sensor and the ponche's thermometer.
const REYES: usize = 22;
const CARTAS: usize = 23;
const PINATA: usize = 24;
const PONCHE: usize = 25;
const SIGNATURE_NV: [usize; 4] = [REYES, CARTAS, PINATA, PONCHE];
/// The Fiestas Patrias have five: GRITO.EXE with its countdown, the panel of
/// the campana de Dolores, the decibel meter of the vivas, the route of the
/// parade of the 16th and the history database of the Independence.
const GRITO: usize = 26;
const CAMPANA: usize = 27;
const VIVAS: usize = 28;
const DESFILE: usize = 29;
const HISTORIA: usize = 30;
const SIGNATURE_PT: [usize; 5] = [GRITO, CAMPANA, VIVAS, DESFILE, HISTORIA];

/// The kinds of widget a holiday adds to the ordinary ones.
fn extras(f: Fiesta) -> std::ops::Range<usize> {
    match f.kind {
        Kind::Muertos => GATEWAY..KINDS_MX,
        Kind::Navidad => REYES..PONCHE + 1,
        Kind::Patrias => GRITO..HISTORIA + 1,
        _ => KINDS..KINDS,
    }
}

/// The widgets that make a screen read as the day's holiday at a glance.
fn signature(f: Fiesta) -> &'static [usize] {
    match f.kind {
        Kind::Muertos => &SIGNATURE,
        Kind::Navidad => &SIGNATURE_NV,
        Kind::Patrias => &SIGNATURE_PT,
        _ => &[],
    }
}

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

fn make_widget(kind: usize, seed: u64, level: u8, mx: Fiesta) -> Box<dyn Widget> {
    use feed::{Feed, FeedKind as K};
    match kind {
        0 => Box::new(Feed::new(K::Hex, seed, level, mx)),
        1 => Box::new(Feed::new(K::Log, seed, level, mx)),
        2 => Box::new(Feed::new(K::Code, seed, level, mx)),
        3 => Box::new(Feed::new(K::Errno, seed, level, mx)),
        4 => Box::new(Feed::new(K::Sniffer, seed, level, mx)),
        5 => Box::new(Feed::new(K::Tree, seed, level, mx)),
        6 => Box::new(Feed::new(K::Dna, seed, level, mx)),
        7 => Box::new(Feed::new(K::Pem, seed, level, mx)),
        8 => Box::new(Feed::new(K::Mission, seed, level, mx)),
        9 => Box::new(widgets::Htop::new(seed, mx)),
        10 => Box::new(widgets::Bandwidth::new(seed, mx)),
        11 => Box::new(widgets::Rain::new(seed, mx)),
        12 => Box::new(widgets::WorldMap::new(seed, mx)),
        13 => Box::new(widgets::Radar::new(seed, mx)),
        14 => Box::new(widgets::Progress::new(seed, mx)),
        15 => Box::new(widgets::Satellites::new(seed, mx)),
        16 => Box::new(widgets::Ticker::new(seed, mx)),
        17 => Box::new(widgets::Access::new(seed, level, mx)),
        GATEWAY => Box::new(mictlan::Gateway::new(seed, mx)),
        OFRENDA => Box::new(mictlan::Ofrenda::new(seed, mx)),
        FACE => Box::new(rostro::Face::new(seed, mx)),
        REYES => Box::new(posada::Reyes::new(seed, mx)),
        CARTAS => Box::new(posada::Cartas::new(seed, mx)),
        PINATA => Box::new(posada::Pinata::new(seed, mx)),
        PONCHE => Box::new(posada::Ponche::new(seed, mx)),
        GRITO => Box::new(grito::Cuenta::new(seed, mx)),
        CAMPANA => Box::new(grito::Campana::new(seed, mx)),
        VIVAS => Box::new(grito::Vivas::new(seed, mx)),
        DESFILE => Box::new(grito::Desfile::new(seed, mx)),
        HISTORIA => Box::new(grito::Historia::new(seed, mx)),
        _ => Box::new(widgets::Keygen::new(seed, level, mx)),
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
    /// The day's holiday (or `Fiesta::NONE`), cached; `retheme` re-dresses the
    /// panes when it changes.
    fiesta: Fiesta,
    seed: u64,
    /// Scene time, for the lights that blink along the title bars.
    clock: f32,
    /// Santos Inocentes: the prank that takes over the panes now and then.
    fool: Option<Fool>,
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
                day: patrias::day_start(holiday(ctx), ctx.seed),
            },
            fiesta: holiday(ctx),
            seed: ctx.seed,
            clock: 0.0,
            fool: holiday(ctx).inocentes().then(|| Fool::new(ctx.seed)),
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

    /// A widget kind: any of them, or on a holiday with widgets of its own
    /// (Día de Muertos three more kinds, Navidad four) the signature ones more
    /// often the higher the holiday.
    fn roll_kind(&mut self) -> usize {
        let sig = signature(self.fiesta);
        if sig.is_empty() {
            return self.rng.usize(..KINDS);
        }
        if self.rng.f32() < 0.1 + 0.2 * self.fiesta.level {
            sig[self.rng.usize(..sig.len())]
        } else {
            let more = extras(self.fiesta);
            let k = self.rng.usize(..KINDS + more.len());
            if k < KINDS { k } else { more.start + k - KINDS }
        }
    }

    fn new_pane(&mut self, taken: &[usize]) -> Box<Pane> {
        let mut kind = self.roll_kind();
        for _ in 0..24 {
            if !taken.contains(&kind) {
                break;
            }
            kind = self.roll_kind();
        }
        let seed = self.rng.u64(..);
        let mut p = Pane {
            rect: Rect::default(),
            kind,
            widget: make_widget(kind, seed, self.env.level, self.fiesta),
            title: String::new(),
            retitle: 0.0,
            age: 0.0,
            content: (u16::MAX, u16::MAX),
        };
        Self::retitle(&mut p, &mut self.rng, self.env.level, self.fiesta);
        Box::new(p)
    }

    fn retitle(p: &mut Pane, rng: &mut Rng, level: u8, mx: Fiesta) {
        p.title.clear();
        p.title.push_str(p.widget.name());
        if level > 0 {
            p.title.push_str(" \u{00b7} ");
            if mx.is(Kind::Muertos) {
                muertos::mixed_title(rng, mx.level, &mut p.title);
            } else if let Some(v) = lexico::of(mx) {
                v.mixed_title(rng, mx.level, &mut p.title);
            } else {
                babble::title(rng, &mut p.title);
            }
        }
        p.retitle = 8.0 + rng.f32() * 14.0;
    }

    /// A holiday with widgets of its own keeps at least one signature widget
    /// on screen by turning the biggest pane into the Mictlán gateway (or the
    /// tracking of the Reyes Magos) if there is none.
    fn ensure_signature(&mut self) {
        let sig = signature(self.fiesta);
        if sig.is_empty() {
            return;
        }
        let fallback = if self.fiesta.is(Kind::Muertos) {
            GATEWAY
        } else {
            sig[0]
        };
        let area = self.area();
        let (level, mx) = (self.env.level, self.fiesta);
        let Some(root) = &mut self.root else {
            return;
        };
        let (mut have, mut biggest) = (false, 0);
        root.each(&mut |p| {
            have |= sig.contains(&p.kind);
            biggest = biggest.max(p.rect.area());
        });
        if have {
            return;
        }
        let rng = &mut self.rng;
        let mut done = false;
        root.each_mut(&mut |p| {
            if !done && p.rect.area() == biggest {
                done = true;
                p.kind = fallback;
                p.widget = make_widget(fallback, rng.u64(..), level, mx);
                p.content = (u16::MAX, u16::MAX);
                Self::retitle(p, rng, level, mx);
            }
        });
        root.layout(area);
    }

    /// Dresses the panes for a holiday (or undresses them): every seat gets
    /// the widget of the new day.
    fn dress(&mut self, mx: Fiesta) {
        self.fiesta = mx;
        self.env.day = patrias::day_start(mx, self.seed);
        self.fool = mx.inocentes().then(|| Fool::new(self.seed));
        self.tape.clear();
        self.tape_off = 0.0;
        let area = self.area();
        let level = self.env.level;
        if let Some(root) = &mut self.root {
            let rng = &mut self.rng;
            root.each_mut(&mut |p| {
                if p.kind >= KINDS && !extras(mx).contains(&p.kind) {
                    p.kind = rng.usize(..KINDS);
                }
                p.widget = make_widget(p.kind, rng.u64(..), level, mx);
                p.content = (u16::MAX, u16::MAX);
                Self::retitle(p, rng, level, mx);
            });
            root.layout(area);
        }
        self.ensure_signature();
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
        self.ensure_signature();
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
            self.ensure_signature();
        } else if roll < 65 && n > 3.min(self.max_panes) && root.mergeable() > 0 {
            let mut k = self.rng.usize(..root.mergeable());
            let keep_first = self.rng.bool();
            root.merge_nth(&mut k, keep_first);
            root.layout(area);
            self.root = Some(root);
            self.ensure_signature();
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
            let mut s = std::mem::take(&mut self.tape_buf);
            if self.fiesta.is(Kind::Muertos) {
                // Headlines of the Mictlán among its mission log.
                if self.rng.f32() < 0.3 + 0.3 * self.fiesta.level {
                    muertos::headline(&mut self.rng, &mut s);
                } else {
                    s.push_str("MISI\u{00d3}N \u{273f} ");
                    let from = s.len();
                    muertos::mixed(&mut self.rng, self.env.level, self.fiesta.level, &mut s);
                    let up = s[from..].to_uppercase();
                    s.truncate(from);
                    s.push_str(&up);
                }
                s.push_str("  \u{273f}  ");
            } else if let Some(v) = lexico::of(self.fiesta) {
                // Headlines of the holiday among its mission log; the days of
                // the turn of the year and of the Reyes have some of their own.
                let day = (v.days)(self.fiesta);
                if !day.is_empty() && self.rng.f32() < 0.4 {
                    let tpl = day[self.rng.usize(..day.len())];
                    let from = s.len();
                    v.expand(&mut self.rng, tpl, &mut s);
                    let up = s[from..].to_uppercase();
                    s.truncate(from);
                    s.push_str(&up);
                } else if self.rng.f32() < 0.3 + 0.3 * self.fiesta.level {
                    v.headline(&mut self.rng, &mut s);
                } else {
                    s.push_str("MISI\u{00d3}N ");
                    s.push(v.mark);
                    s.push(' ');
                    let from = s.len();
                    v.mixed(&mut self.rng, self.env.level, self.fiesta.level, &mut s);
                    let up = s[from..].to_uppercase();
                    s.truncate(from);
                    s.push_str(&up);
                }
                s.push_str("  ");
                s.push(v.mark);
                s.push_str("  ");
            } else {
                s.push_str("MISSION \u{25c6} ");
                babble::gerund_phrase(&mut self.rng, self.env.level, &mut s);
                s.make_ascii_uppercase();
                s.push_str("  \u{25c6}  ");
            }
            self.tape.extend(s.chars());
            self.tape_buf = s;
        }
    }

    #[cfg(test)]
    fn kinds(&self) -> Vec<usize> {
        let mut v = Vec::new();
        if let Some(r) = &self.root {
            r.each(&mut |p| v.push(p.kind));
        }
        v
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
        // The holiday came or went (or moved on a day): new widgets in the seats.
        let mx = holiday(ctx);
        if mx != self.fiesta {
            self.dress(mx);
        }
    }

    fn update(&mut self, _ctx: &Ctx, dt: f32) {
        let dt = (dt * self.speed).clamp(0.0, 0.5);
        self.clock += dt;
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
        self.env.day += real;
        if let Some(f) = &mut self.fool {
            f.tick(real);
        }
        self.next_event -= real;
        if self.next_event <= 0.0 && self.w > 0 {
            self.change_layout();
            self.next_event = Self::interval(&mut self.rng, self.reshuffle);
        }
        let (level, mx) = (self.env.level, self.fiesta);
        if let Some(r) = &mut self.root {
            let rng = &mut self.rng;
            r.each_mut(&mut |p| {
                p.retitle -= real;
                if p.retitle <= 0.0 {
                    Self::retitle(p, rng, level, mx);
                }
            });
        }
    }

    fn draw(&mut self, _ctx: &Ctx, c: &mut Canvas) {
        self.feed_tape();
        let env = &self.env;
        let pal = &env.pal;
        let alarm = self.fool.as_ref().and_then(|f| f.edge(pal));
        let clock = self.clock;
        if let Some(root) = &mut self.root {
            root.each_mut(&mut |p| {
                let r = p.rect;
                if r.w < 3 || r.h < 3 {
                    return;
                }
                // New panes light their border up for a moment.
                let edge = if let Some(col) = alarm {
                    col
                } else if p.age < 1.0 {
                    pal.fade(pal.bright, p.age * 0.5)
                } else {
                    pal.fade(pal.main, 0.45)
                };
                c.set(r.x, r.y, '\u{250c}', edge);
                // Día de Muertos hangs papel picado along the title bar: a flag
                // every third column, then every other and, at the height of
                // the holiday, one after another, in the six colors.
                let gap = if !(self.fiesta.is(Kind::Muertos) || self.fiesta.is(Kind::Patrias)) {
                    0
                } else if self.fiesta.level < 0.5 {
                    3
                } else if self.fiesta.level < 0.8 {
                    2
                } else {
                    1
                };
                if self.fiesta.is(Kind::Navidad) {
                    // Navidad strings a series of foquitos along the title bar,
                    // a bulb every fourth column, then every third and, at
                    // Nochebuena, every other, in the six colors of the
                    // lights. They blink in a chase: two of every three lit,
                    // the dark one running along the string.
                    let gap = if self.fiesta.level < 0.5 {
                        4
                    } else if self.fiesta.level < 0.8 {
                        3
                    } else {
                        2
                    };
                    let phase = (clock * 3.0) as usize;
                    for x in 1..r.w {
                        if (x + r.x + r.y) % gap == 0 {
                            let n = ((x + r.x) / gap) as usize;
                            let lit = !(n + phase).is_multiple_of(3);
                            let bulb = pal.picado[n % 6];
                            let (col, at) = if lit {
                                (bulb, BOLD)
                            } else {
                                (pal.fade(bulb, 0.65), 0)
                            };
                            c.set_attr(r.x + x, r.y, '\u{25cf}', col, at);
                        } else {
                            c.set(r.x + x, r.y, '\u{2500}', edge);
                        }
                    }
                } else {
                    for x in 1..r.w {
                        if gap > 0 && (x + r.x + r.y) % gap == 0 {
                            let k = ((x + r.x) / gap.max(1)) as usize % 6;
                            c.set(r.x + x, r.y, '\u{25bc}', pal.picado[k]);
                        } else {
                            c.set(r.x + x, r.y, '\u{2500}', edge);
                        }
                    }
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
            let mark = lexico::of(self.fiesta).map_or('\0', |v| v.mark);
            for (i, &ch) in self.tape.iter().take(self.w as usize).enumerate() {
                let (col, at) = if ch == '\u{25c6}' {
                    (pal.amber, BOLD)
                } else if ch == '\u{273f}' {
                    (pal.flor, BOLD)
                } else if ch == mark {
                    (pal.oro, BOLD)
                } else {
                    (pal.cyan, 0)
                };
                c.set_attr(i as i32, y, ch, col, at);
            }
        }
        if let Some(f) = &mut self.fool {
            f.draw(c, pal);
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

    // ------------------------------------------------------------ Día de Muertos

    use crate::fiesta::Fiesta;

    fn ctx_of<'a>(theme: &'a Theme, o: &'a crate::scene::Opts, fiesta: Fiesta) -> Ctx<'a> {
        Ctx {
            theme,
            opts: o,
            bg: Color::Reset,
            seed: 11,
            fiesta,
        }
    }

    fn opts(src: &str) -> crate::scene::Opts {
        crate::scene::Opts::new(INFO.options, src.parse().unwrap())
    }

    fn frame(s: &mut Hollywood, ctx: &Ctx, w: u16, h: u16) -> Canvas {
        let mut c = Canvas::new(w, h, Color::Reset);
        s.draw(ctx, &mut c);
        c
    }

    fn count(c: &Canvas, ch: char) -> usize {
        c.cells().iter().filter(|x| x.ch == ch).count()
    }

    fn text(c: &Canvas) -> String {
        c.to_text()
    }

    /// Runs `secs` seconds of scene time at 30 fps.
    fn run(s: &mut Hollywood, ctx: &Ctx, secs: f32) {
        for _ in 0..(secs * 30.0) as usize {
            s.tick(ctx, 1.0 / 30.0, 1.0 / 30.0);
        }
    }

    #[test]
    fn the_holiday_dresses_the_screen() {
        let theme = Theme::classic();
        let o = opts("");
        for fiesta in [Fiesta::of_day(11, 2), Fiesta::of_day(10, 28)] {
            let ctx = ctx_of(&theme, &o, fiesta);
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 140, 40);
            run(&mut s, &ctx, 3.0);
            let c = frame(&mut s, &ctx, 140, 40);
            // Flowers on the tape and in the widgets, papel picado on the
            // title bars (in the six colors), cempasúchil for the text.
            assert!(count(&c, '\u{273f}') >= 1, "{}", text(&c));
            assert!(count(&c, '\u{25bc}') > 6);
            let orange = s.env.pal.main;
            assert_eq!(orange, rgb(0xffa01e));
            assert!(c.cells().iter().any(|x| x.fg == orange));
            let mut flags: Vec<Color> = Vec::new();
            for x in c.cells().iter().filter(|x| x.ch == '\u{25bc}') {
                if !flags.contains(&x.fg) {
                    flags.push(x.fg);
                }
            }
            assert!(flags.len() >= 3, "{flags:?}");
            assert!(s.kinds().iter().any(|k| SIGNATURE.contains(k)));
        }
    }

    #[test]
    fn ordinary_days_have_no_holiday() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::NONE);
        let mut s = Hollywood::new(&ctx);
        s.resize(&ctx, 140, 40);
        run(&mut s, &ctx, 30.0);
        let c = frame(&mut s, &ctx, 140, 40);
        for ch in ['\u{273f}', '\u{2740}', '\u{2726}', '\u{2302}'] {
            assert_eq!(count(&c, ch), 0, "{ch}");
        }
        assert_eq!(s.env.pal.main, rgb(0x33ee66));
        assert!(s.kinds().iter().all(|&k| k < KINDS));
        assert!(!text(&c).contains("MICTL"));
        assert!(text(&c).lines().last().unwrap().starts_with("MISSION"));
    }

    #[test]
    fn the_holiday_comes_and_goes_through_retheme() {
        let theme = Theme::classic();
        let o = opts("");
        let ordinary = ctx_of(&theme, &o, Fiesta::NONE);
        let first = ctx_of(&theme, &o, Fiesta::of_day(10, 28));
        let height = ctx_of(&theme, &o, Fiesta::of_day(11, 2));
        let mut s = Hollywood::new(&ordinary);
        s.resize(&ordinary, 160, 48);
        s.retheme(&ordinary);
        run(&mut s, &ordinary, 2.0);
        assert_eq!(count(&frame(&mut s, &ordinary, 160, 48), '\u{273f}'), 0);
        for ctx in [&first, &height, &ordinary, &height, &ordinary] {
            s.retheme(ctx);
            run(&mut s, ctx, 2.0);
            let c = frame(&mut s, ctx, 160, 48);
            let on = ctx.fiesta.is(Kind::Muertos);
            assert_eq!(count(&c, '\u{273f}') > 0, on, "{}", text(&c));
            assert_eq!(s.fiesta, if on { ctx.fiesta } else { Fiesta::NONE });
            assert_eq!(s.env.pal.main == rgb(0xffa01e), on);
            if !on {
                assert!(s.kinds().iter().all(|&k| k < KINDS), "{:?}", s.kinds());
            }
        }
        // Dressing only changes the widgets: the panes are where they were
        // (until the layout moves on its own, which takes a few seconds).
        let mut t = Hollywood::new(&ordinary);
        t.resize(&ordinary, 160, 48);
        let rects = t.rects();
        t.retheme(&height);
        assert_eq!(t.rects(), rects);
        t.retheme(&ordinary);
        assert_eq!(t.rects(), rects);
    }

    #[test]
    fn the_holiday_grows_with_its_level() {
        let theme = Theme::classic();
        let o = opts("");
        let flags = |fiesta: Fiesta| {
            let ctx = ctx_of(&theme, &o, fiesta);
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 160, 48);
            frame(&mut s, &ctx, 160, 48)
        };
        let low = count(&flags(Fiesta::of_day(10, 28)), '\u{25bc}');
        let mid = count(&flags(Fiesta::of_day(10, 31)), '\u{25bc}');
        let high = count(&flags(Fiesta::of_day(11, 2)), '\u{25bc}');
        assert!(low < mid && mid < high, "{low} {mid} {high}");
        // More souls on the radar and more crossing the nine levels.
        let radar = |f: Fiesta| widgets::Radar::new(1, f);
        assert!(
            radar(Fiesta::of_day(10, 28)).blips_len() < radar(Fiesta::of_day(11, 2)).blips_len()
        );
        let gate = |f: Fiesta| mictlan::Gateway::new(1, f).crossing();
        assert!(gate(Fiesta::of_day(10, 28)) < gate(Fiesta::of_day(11, 2)));
        // And more of the text is the Mictlán's.
        assert!(muertos::share(0.25) < muertos::share(1.0));
    }

    #[test]
    fn the_mictlan_is_on_screen_at_every_size_and_time() {
        let theme = Theme::classic();
        let o = opts("reshuffle = 1");
        for fiesta in [Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
            let ctx = ctx_of(&theme, &o, fiesta);
            for &(w, h) in &[(200u16, 60u16), (120, 36), (80, 24), (60, 20), (31, 9)] {
                let mut s = Hollywood::new(&ctx);
                s.resize(&ctx, w, h);
                for step in 0..120 {
                    s.tick(&ctx, 0.2, 1.0);
                    assert!(
                        s.kinds().iter().any(|k| SIGNATURE.contains(k)),
                        "{w}x{h} step {step}: {:?}",
                        s.kinds()
                    );
                }
            }
        }
    }

    #[test]
    fn odd_sizes_and_huge_steps_survive_the_holiday() {
        let o = opts("");
        let dracula = Theme::builtin("dracula").unwrap();
        for theme in [Theme::classic(), dracula] {
            for fiesta in [Fiesta::of_day(11, 2), Fiesta::of_day(10, 28)] {
                let ctx = ctx_of(&theme, &o, fiesta);
                for &(w, h) in &[
                    (1u16, 1u16),
                    (2, 3),
                    (7, 2),
                    (30, 7),
                    (29, 8),
                    (45, 12),
                    (80, 24),
                    (200, 60),
                    (300, 90),
                ] {
                    let mut s = Hollywood::new(&ctx);
                    s.resize(&ctx, w, h);
                    s.retheme(&ctx);
                    for step in 0..60 {
                        let (dt, real) = if step % 5 == 0 {
                            (5.0, 5.0)
                        } else {
                            (0.05, 0.3)
                        };
                        s.tick(&ctx, dt, real);
                        frame(&mut s, &ctx, w, h);
                    }
                    // A resize in the middle of it.
                    s.resize(&ctx, w / 2 + 1, h / 2 + 1);
                    frame(&mut s, &ctx, w / 2 + 1, h / 2 + 1);
                }
            }
        }
    }

    #[test]
    fn every_widget_stays_in_its_pane_on_and_off_the_holiday() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(11, 2));
        let env = Env {
            pal: Pal::new(&ctx),
            level: 7,
            day: 0.0,
        };
        for fiesta in [Fiesta::NONE, Fiesta::of_day(10, 28), Fiesta::of_day(11, 2)] {
            for kind in 0..KINDS_MX {
                if kind >= KINDS && !fiesta.is(Kind::Muertos) {
                    continue;
                }
                for &(w, h) in &[
                    (1u16, 1u16),
                    (3, 3),
                    (12, 4),
                    (29, 6),
                    (44, 11),
                    (80, 30),
                    (120, 40),
                ] {
                    let mut widget = make_widget(kind, 5, 7, fiesta);
                    widget.resize(w, h);
                    let r = Rect {
                        x: 3,
                        y: 2,
                        w: w as i32,
                        h: h as i32,
                    };
                    for step in 0..40 {
                        widget.update(if step % 9 == 0 { 0.5 } else { 0.07 }, &env);
                        let mut c = Canvas::new(w + 8, h + 6, Color::Reset);
                        widget.draw(&mut c, r, &env);
                        for y in 0..c.h() as i32 {
                            for x in 0..c.w() as i32 {
                                let inside = x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h;
                                if !inside {
                                    let cell = c.get(x, y).unwrap();
                                    assert_eq!(
                                        cell.ch, ' ',
                                        "kind {kind} {fiesta:?} {w}x{h} drew {:?} at {x},{y}",
                                        cell.ch
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_tape_brings_the_brief_s_headlines() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(11, 2));
        let mut s = Hollywood::new(&ctx);
        s.resize(&ctx, 100, 30);
        let mut seen = String::new();
        for _ in 0..400 {
            s.tape.clear();
            s.feed_tape();
            seen.extend(s.tape.iter());
        }
        for line in [
            "MICTL\u{00c1}N GATEWAY ONLINE",
            "UPLOADING PAN_DE_MUERTO.TAR.GZ",
            "ACCESO AL MICTL\u{00c1}N DENEGADO",
        ] {
            assert!(seen.contains(line), "{line}");
        }
    }

    #[test]
    fn the_access_banner_speaks_spanish() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(11, 2));
        let env = Env {
            pal: Pal::new(&ctx),
            level: 7,
            day: 0.0,
        };
        let mut seen = String::new();
        for seed in 0..40u64 {
            let mut a = widgets::Access::new(seed, 7, ctx.fiesta);
            a.resize(70, 20);
            for step in 0..300 {
                a.update(0.1, &env);
                if step % 6 == 0 {
                    let mut c = Canvas::new(70, 20, Color::Reset);
                    a.draw(
                        &mut c,
                        Rect {
                            x: 0,
                            y: 0,
                            w: 70,
                            h: 20,
                        },
                        &env,
                    );
                    seen.push_str(&c.to_text());
                }
            }
        }
        assert!(seen.contains("TODAV\u{00cd}A NO TE TOCA"), "denied banner");
        assert!(seen.contains("PASA, TE ESPERAN"), "granted banner");
        assert!(seen.contains("DENEGADO") && seen.contains("MICTL\u{00c1}N"));
        assert!(!seen.contains("ACCESS"));
    }

    // ------------------------------------------------------------------ Navidad

    /// The foquitos strung along the top row, where every pane has its title bar.
    fn bulbs(c: &Canvas) -> usize {
        (0..c.w() as i32)
            .filter(|&x| c.get(x, 0).is_some_and(|cell| cell.ch == '\u{25cf}'))
            .count()
    }

    /// Days across the holiday: the first, a posada, Nochebuena, Navidad, the
    /// Inocentes, Año Nuevo and the Reyes closing in and arrived.
    const NAVIDAD_DAYS: [(u32, u32); 8] = [
        (12, 12),
        (12, 20),
        (12, 24),
        (12, 25),
        (12, 28),
        (12, 31),
        (1, 2),
        (1, 6),
    ];

    #[test]
    fn navidad_dresses_the_screen() {
        let theme = Theme::classic();
        let o = opts("");
        for (m, d) in NAVIDAD_DAYS {
            let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 140, 40);
            run(&mut s, &ctx, 3.0);
            let c = frame(&mut s, &ctx, 140, 40);
            // Series lights along the title bars in their six colors, the
            // green of the foquitos for the text, a star on the tape.
            assert!(bulbs(&c) > 6, "{m}-{d}\n{}", text(&c));
            let mut colors: Vec<Color> = Vec::new();
            for x in (0..140)
                .filter_map(|x| c.get(x, 0))
                .filter(|x| x.ch == '\u{25cf}')
            {
                if !colors.contains(&x.fg) {
                    colors.push(x.fg);
                }
            }
            assert!(colors.len() >= 4, "{m}-{d} {colors:?}");
            assert_eq!(s.env.pal.main, rgb(0x3bd16f));
            assert!(c.cells().iter().any(|x| x.fg == s.env.pal.main));
            assert!(
                text(&c).lines().last().unwrap().contains('\u{2736}'),
                "{m}-{d}"
            );
            assert!(s.kinds().iter().any(|k| SIGNATURE_NV.contains(k)));
            assert!(
                s.kinds()
                    .iter()
                    .all(|k| *k < KINDS || extras(s.fiesta).contains(k))
            );
            // And not a flower of the other holiday.
            assert_eq!(count(&c, '\u{273f}'), 0);
        }
    }

    #[test]
    fn an_ordinary_day_has_no_navidad() {
        let theme = Theme::classic();
        let o = opts("");
        // Days that are ordinary, and holidays this scene does not dress for.
        for fiesta in [
            Fiesta::NONE,
            Fiesta::of_day(6, 15),
            Fiesta::of_day(8, 20),
            Fiesta::of_day(2, 2),
            Fiesta::of_day(4, 30),
            Fiesta::of_day(5, 3),
        ] {
            let ctx = ctx_of(&theme, &o, fiesta);
            let mut s = Hollywood::new(&ctx);
            assert_eq!(s.fiesta, Fiesta::NONE, "{fiesta:?}");
            s.resize(&ctx, 140, 40);
            run(&mut s, &ctx, 30.0);
            let c = frame(&mut s, &ctx, 140, 40);
            assert_eq!(bulbs(&c), 0, "{fiesta:?}");
            assert_eq!(s.env.pal.main, rgb(0x33ee66));
            assert!(s.fool.is_none());
            assert!(s.kinds().iter().all(|&k| k < KINDS));
            assert!(text(&c).lines().last().unwrap().starts_with("MISSION"));
            let t = text(&c);
            for word in ["RASTREO", "posada", "PI\u{00d1}ATA"] {
                assert!(!t.contains(word), "{word}");
            }
        }
    }

    #[test]
    fn navidad_comes_and_goes_through_retheme() {
        let theme = Theme::classic();
        let o = opts("");
        let ordinary = ctx_of(&theme, &o, Fiesta::NONE);
        let first = ctx_of(&theme, &o, Fiesta::of_day(12, 12));
        let height = ctx_of(&theme, &o, Fiesta::of_day(12, 24));
        let reyes = ctx_of(&theme, &o, Fiesta::of_day(1, 6));
        let muertos = ctx_of(&theme, &o, Fiesta::of_day(11, 2));
        let mut s = Hollywood::new(&ordinary);
        s.resize(&ordinary, 160, 48);
        s.retheme(&ordinary);
        run(&mut s, &ordinary, 2.0);
        assert_eq!(bulbs(&frame(&mut s, &ordinary, 160, 48)), 0);
        for ctx in [&first, &height, &ordinary, &reyes, &muertos, &ordinary] {
            s.retheme(ctx);
            run(&mut s, ctx, 2.0);
            let c = frame(&mut s, ctx, 160, 48);
            let on = ctx.fiesta.is(Kind::Navidad);
            assert_eq!(bulbs(&c) > 0, on, "{}", text(&c));
            assert_eq!(s.fiesta, holiday(ctx));
            assert_eq!(s.env.pal.main == rgb(0x3bd16f), on);
            if !on && !ctx.fiesta.is(Kind::Muertos) {
                assert!(s.kinds().iter().all(|&k| k < KINDS), "{:?}", s.kinds());
            }
            // Only the right holiday's widgets are ever in the seats.
            assert!(
                s.kinds()
                    .iter()
                    .all(|k| *k < KINDS || extras(s.fiesta).contains(k))
            );
        }
        // Dressing only changes the widgets: the panes stay where they were.
        let mut t = Hollywood::new(&ordinary);
        t.resize(&ordinary, 160, 48);
        let rects = t.rects();
        t.retheme(&height);
        assert_eq!(t.rects(), rects);
        t.retheme(&ordinary);
        assert_eq!(t.rects(), rects);
    }

    #[test]
    fn navidad_grows_with_its_level() {
        let theme = Theme::classic();
        let o = opts("");
        let bulbs = |fiesta: Fiesta| {
            let ctx = ctx_of(&theme, &o, fiesta);
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 160, 48);
            bulbs(&frame(&mut s, &ctx, 160, 48))
        };
        let (low, mid, high) = (
            bulbs(Fiesta::of_day(12, 12)),
            bulbs(Fiesta::of_day(12, 18)),
            bulbs(Fiesta::of_day(12, 24)),
        );
        assert!(low < mid && mid < high, "{low} {mid} {high}");
        // More of the text is the holiday's, more shepherds on the radar.
        assert!(lexico::share(0.3) < lexico::share(1.0));
        let radar = |f: Fiesta| widgets::Radar::new(1, f);
        assert!(
            radar(Fiesta::of_day(12, 12)).blips_len() < radar(Fiesta::of_day(12, 24)).blips_len()
        );
    }

    #[test]
    fn the_reyes_are_on_screen_at_every_size_and_time() {
        let theme = Theme::classic();
        let o = opts("reshuffle = 1");
        for (m, d) in [(12, 24), (12, 12), (1, 4)] {
            let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
            for &(w, h) in &[(200u16, 60u16), (120, 36), (80, 24), (60, 20), (31, 9)] {
                let mut s = Hollywood::new(&ctx);
                s.resize(&ctx, w, h);
                for step in 0..120 {
                    s.tick(&ctx, 0.2, 1.0);
                    assert!(
                        s.kinds().iter().any(|k| SIGNATURE_NV.contains(k)),
                        "{m}-{d} {w}x{h} step {step}: {:?}",
                        s.kinds()
                    );
                }
            }
        }
    }

    #[test]
    fn odd_sizes_and_huge_steps_survive_navidad() {
        let o = opts("");
        let dracula = Theme::builtin("dracula").unwrap();
        for theme in [Theme::classic(), dracula] {
            for (m, d) in NAVIDAD_DAYS {
                let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
                for &(w, h) in &[
                    (1u16, 1u16),
                    (2, 3),
                    (7, 2),
                    (30, 7),
                    (29, 8),
                    (45, 12),
                    (80, 24),
                    (200, 60),
                    (300, 90),
                ] {
                    let mut s = Hollywood::new(&ctx);
                    s.resize(&ctx, w, h);
                    s.retheme(&ctx);
                    for step in 0..60 {
                        let (dt, real) = if step % 5 == 0 {
                            (5.0, 5.0)
                        } else {
                            (0.05, 0.3)
                        };
                        s.tick(&ctx, dt, real);
                        frame(&mut s, &ctx, w, h);
                    }
                    // A resize in the middle of it.
                    s.resize(&ctx, w / 2 + 1, h / 2 + 1);
                    frame(&mut s, &ctx, w / 2 + 1, h / 2 + 1);
                }
            }
        }
    }

    #[test]
    fn navidads_widgets_stay_in_their_panes() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(12, 24));
        let env = Env {
            pal: Pal::new(&ctx),
            level: 7,
            day: 0.0,
        };
        for (m, d) in [(12, 12), (12, 24), (1, 2), (1, 6)] {
            let fiesta = Fiesta::of_day(m, d);
            for kind in (0..KINDS).chain(extras(fiesta)) {
                for &(w, h) in &[
                    (1u16, 1u16),
                    (3, 3),
                    (12, 4),
                    (29, 6),
                    (44, 11),
                    (80, 30),
                    (120, 40),
                ] {
                    let mut widget = make_widget(kind, 5, 7, fiesta);
                    widget.resize(w, h);
                    let r = Rect {
                        x: 3,
                        y: 2,
                        w: w as i32,
                        h: h as i32,
                    };
                    for step in 0..40 {
                        widget.update(if step % 9 == 0 { 0.5 } else { 0.07 }, &env);
                        let mut c = Canvas::new(w + 8, h + 6, Color::Reset);
                        widget.draw(&mut c, r, &env);
                        for y in 0..c.h() as i32 {
                            for x in 0..c.w() as i32 {
                                let inside = x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h;
                                if !inside {
                                    let cell = c.get(x, y).unwrap();
                                    assert_eq!(
                                        cell.ch, ' ',
                                        "kind {kind} {m}-{d} {w}x{h} drew {:?} at {x},{y}",
                                        cell.ch
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_tape_brings_navidads_headlines() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(12, 24));
        let mut s = Hollywood::new(&ctx);
        s.resize(&ctx, 100, 30);
        let mut seen = String::new();
        for _ in 0..400 {
            s.tape.clear();
            s.feed_tape();
            seen.extend(s.tape.iter());
        }
        for line in [
            "RASTREO DE LOS REYES MAGOS: ETA 06-ENE",
            "UPLOADING AGUINALDO_2026.TAR.GZ",
            "AQU\u{00cd} NO ES MES\u{00d3}N: ACCESO DENEGADO",
        ] {
            assert!(seen.contains(line), "{line}");
        }
        // The turn of the year and the Reyes have headlines of their own.
        for (m, d, line) in [
            (12, 31, "DOCE UVAS"),
            (1, 1, "PR\u{00d3}SPERO A\u{00d1}O NUEVO"),
            (1, 4, "ROSCA DE REYES"),
        ] {
            let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 100, 30);
            let mut seen = String::new();
            for _ in 0..400 {
                s.tape.clear();
                s.feed_tape();
                seen.extend(s.tape.iter());
            }
            assert!(seen.contains(line), "{m}-{d} {line}");
        }
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(12, 20));
        let mut s = Hollywood::new(&ctx);
        s.resize(&ctx, 100, 30);
        let mut seen = String::new();
        for _ in 0..400 {
            s.tape.clear();
            s.feed_tape();
            seen.extend(s.tape.iter());
        }
        assert!(!seen.contains("DOCE UVAS"));
    }

    #[test]
    fn the_villancicos_scroll_through_the_log() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(12, 24));
        let env = Env {
            pal: Pal::new(&ctx),
            level: 7,
            day: 0.0,
        };
        let mut seen = String::new();
        for seed in 0..12u64 {
            let mut f = feed::Feed::new(feed::FeedKind::Log, seed, 7, ctx.fiesta);
            f.resize(120, 40);
            for step in 0..400 {
                f.update(0.1, &env);
                if step % 40 == 0 {
                    let mut c = Canvas::new(120, 40, Color::Reset);
                    f.draw(
                        &mut c,
                        Rect {
                            x: 0,
                            y: 0,
                            w: 120,
                            h: 40,
                        },
                        &env,
                    );
                    seen.push_str(&c.to_text());
                }
            }
        }
        assert!(seen.contains("\u{266a} "), "{seen}");
        // Songs go by in order: a verse is followed by the next one.
        assert!(
            seen.contains("Campana sobre campana,")
                || seen.contains("Aqu\u{00ed} no es mes\u{00f3}n")
                || seen.contains("no pierdas el tino")
                || seen.contains("Noche de paz")
        );
        // An ordinary log has none.
        let mut f = feed::Feed::new(feed::FeedKind::Log, 3, 7, Fiesta::NONE);
        f.resize(120, 40);
        for _ in 0..400 {
            f.update(0.1, &env);
        }
        let mut c = Canvas::new(120, 40, Color::Reset);
        f.draw(
            &mut c,
            Rect {
                x: 0,
                y: 0,
                w: 120,
                h: 40,
            },
            &env,
        );
        assert!(!c.to_text().contains('\u{266a}'));
    }

    #[test]
    fn the_inocentes_prank_runs_on_december_28_only() {
        let theme = Theme::classic();
        let o = opts("");
        let prank = |m: u32, d: u32| {
            let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 140, 40);
            let mut seen = String::new();
            // Two minutes of wall-clock time, a frame every half second.
            for _ in 0..240 {
                s.tick(&ctx, 0.5, 0.5);
                seen.push_str(&frame(&mut s, &ctx, 140, 40).to_text());
            }
            seen
        };
        let seen = prank(12, 28);
        assert!(seen.contains("SISTEMA COMPROMETIDO"));
        assert!(seen.contains("\u{00a1}\u{00a1}ALERTA!!"));
        assert!(seen.contains("INOCENTE PALOMITA, QUE TE DEJASTE ENGA\u{00d1}AR"));
        // Any other day of Navidad leaves the panes alone.
        for (m, d) in [(12, 27), (12, 29), (12, 24), (1, 6)] {
            let seen = prank(m, d);
            assert!(!seen.contains("SISTEMA COMPROMETIDO"), "{m}-{d}");
            assert!(!seen.contains("INOCENTE PALOMITA"), "{m}-{d}");
        }
        // It comes and goes with the day.
        let ordinary = ctx_of(&theme, &o, Fiesta::NONE);
        let day = ctx_of(&theme, &o, Fiesta::of_day(12, 28));
        let mut s = Hollywood::new(&ordinary);
        s.resize(&ordinary, 100, 30);
        assert!(s.fool.is_none());
        s.retheme(&day);
        assert!(s.fool.is_some());
        s.retheme(&ordinary);
        assert!(s.fool.is_none());
    }

    // ------------------------------------------------------------ Fiestas Patrias

    /// The banderitas hung along the top row, where every pane has its title bar.
    fn banderitas(c: &Canvas) -> usize {
        (0..c.w() as i32)
            .filter(|&x| c.get(x, 0).is_some_and(|cell| cell.ch == '\u{25bc}'))
            .count()
    }

    /// Days across the mes patrio: the first, mid-month, the eve, the Grito and
    /// the parade.
    const PATRIAS_DAYS: [(u32, u32); 5] = [(9, 1), (9, 8), (9, 14), (9, 15), (9, 16)];

    #[test]
    fn patrias_dresses_the_screen() {
        let theme = Theme::classic();
        let o = opts("");
        for (m, d) in PATRIAS_DAYS {
            let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 140, 40);
            run(&mut s, &ctx, 3.0);
            let c = frame(&mut s, &ctx, 140, 40);
            // Banderitas on the title bars in verde, blanco and rojo, white
            // text, a star on the tape.
            assert!(banderitas(&c) > 6, "{m}-{d}\n{}", text(&c));
            let mut colors: Vec<Color> = Vec::new();
            for x in (0..140)
                .filter_map(|x| c.get(x, 0))
                .filter(|x| x.ch == '\u{25bc}')
            {
                if !colors.contains(&x.fg) {
                    colors.push(x.fg);
                }
            }
            assert!(colors.len() >= 3, "{m}-{d} {colors:?}");
            assert_eq!(s.env.pal.main, rgb(0xf4f1ea));
            assert!(c.cells().iter().any(|x| x.fg == s.env.pal.main));
            assert!(
                text(&c).lines().last().unwrap().contains('\u{2605}'),
                "{m}-{d}"
            );
            assert!(s.kinds().iter().any(|k| SIGNATURE_PT.contains(k)));
            assert!(
                s.kinds()
                    .iter()
                    .all(|k| *k < KINDS || extras(s.fiesta).contains(k))
            );
            assert_eq!(count(&c, '\u{273f}'), 0);
            assert!(s.fool.is_none());
        }
    }

    #[test]
    fn patrias_comes_and_goes_through_retheme() {
        let theme = Theme::classic();
        let o = opts("");
        let ordinary = ctx_of(&theme, &o, Fiesta::NONE);
        let first = ctx_of(&theme, &o, Fiesta::of_day(9, 1));
        let grito = ctx_of(&theme, &o, Fiesta::of_day(9, 15));
        let navidad = ctx_of(&theme, &o, Fiesta::of_day(12, 24));
        let mut s = Hollywood::new(&ordinary);
        s.resize(&ordinary, 160, 48);
        s.retheme(&ordinary);
        run(&mut s, &ordinary, 2.0);
        assert_eq!(banderitas(&frame(&mut s, &ordinary, 160, 48)), 0);
        for ctx in [&first, &grito, &ordinary, &navidad, &grito, &ordinary] {
            s.retheme(ctx);
            run(&mut s, ctx, 2.0);
            let c = frame(&mut s, ctx, 160, 48);
            let on = ctx.fiesta.is(Kind::Patrias);
            assert_eq!(banderitas(&c) > 0, on, "{}", text(&c));
            assert_eq!(s.fiesta, holiday(ctx));
            assert_eq!(s.env.pal.main == rgb(0xf4f1ea), on);
            assert!(
                s.kinds()
                    .iter()
                    .all(|k| *k < KINDS || extras(s.fiesta).contains(k))
            );
        }
        // The scene's day starts a few minutes before the Grito on the 15th.
        s.retheme(&grito);
        assert!(s.env.day > patrias::GRITO_AT - 400.0 && s.env.day < patrias::GRITO_AT);
        // And runs on real time only.
        let day = s.env.day;
        for _ in 0..20 {
            s.tick(&grito, 5.0, 0.0);
        }
        assert_eq!(s.env.day, day);
        for _ in 0..20 {
            s.tick(&grito, 0.0, 1.0);
        }
        assert!((s.env.day - day - 20.0).abs() < 0.01);
    }

    #[test]
    fn patrias_grows_with_its_level() {
        let theme = Theme::classic();
        let o = opts("");
        let flags = |fiesta: Fiesta| {
            let ctx = ctx_of(&theme, &o, fiesta);
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 160, 48);
            banderitas(&frame(&mut s, &ctx, 160, 48))
        };
        let (low, mid, high) = (
            flags(Fiesta::of_day(9, 1)),
            flags(Fiesta::of_day(9, 12)),
            flags(Fiesta::of_day(9, 15)),
        );
        assert!(low < mid && mid < high, "{low} {mid} {high}");
        // More contingents march and the crowd is bigger.
        let marching = |f: Fiesta| grito::Desfile::new(1, f).marching();
        assert!(marching(Fiesta::of_day(9, 1)) < marching(Fiesta::of_day(9, 14)));
        let radar = |f: Fiesta| widgets::Radar::new(1, f);
        assert!(radar(Fiesta::of_day(9, 1)).blips_len() < radar(Fiesta::of_day(9, 15)).blips_len());
    }

    #[test]
    fn the_patria_is_on_screen_at_every_size_and_time() {
        let theme = Theme::classic();
        let o = opts("reshuffle = 1");
        for (m, d) in [(9, 15), (9, 3), (9, 16)] {
            let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
            for &(w, h) in &[(200u16, 60u16), (120, 36), (80, 24), (60, 20), (31, 9)] {
                let mut s = Hollywood::new(&ctx);
                s.resize(&ctx, w, h);
                for step in 0..120 {
                    s.tick(&ctx, 0.2, 1.0);
                    assert!(
                        s.kinds().iter().any(|k| SIGNATURE_PT.contains(k)),
                        "{m}-{d} {w}x{h} step {step}: {:?}",
                        s.kinds()
                    );
                }
            }
        }
    }

    #[test]
    fn odd_sizes_and_huge_steps_survive_patrias() {
        let o = opts("");
        let dracula = Theme::builtin("dracula").unwrap();
        for theme in [Theme::classic(), dracula] {
            for (m, d) in PATRIAS_DAYS {
                let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
                for &(w, h) in &[
                    (1u16, 1u16),
                    (2, 3),
                    (7, 2),
                    (30, 7),
                    (29, 8),
                    (45, 12),
                    (80, 24),
                    (200, 60),
                    (300, 90),
                ] {
                    let mut s = Hollywood::new(&ctx);
                    s.resize(&ctx, w, h);
                    s.retheme(&ctx);
                    for step in 0..60 {
                        let (dt, real) = if step % 5 == 0 {
                            (5.0, 5.0)
                        } else {
                            (0.05, 0.3)
                        };
                        s.tick(&ctx, dt, real);
                        frame(&mut s, &ctx, w, h);
                    }
                    s.resize(&ctx, w / 2 + 1, h / 2 + 1);
                    frame(&mut s, &ctx, w / 2 + 1, h / 2 + 1);
                }
            }
        }
    }

    #[test]
    fn patrias_widgets_stay_in_their_panes() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(9, 15));
        for (m, d) in [(9, 1), (9, 15), (9, 16)] {
            let fiesta = Fiesta::of_day(m, d);
            // The Grito itself, in the middle of it, and a morning.
            for day in [patrias::GRITO_AT + 20.0, patrias::GRITO_AT + 40.0, 30_000.0] {
                let env = Env {
                    pal: Pal::new(&ctx),
                    level: 7,
                    day,
                };
                for kind in (0..KINDS).chain(extras(fiesta)) {
                    for &(w, h) in &[
                        (1u16, 1u16),
                        (3, 3),
                        (12, 4),
                        (29, 6),
                        (44, 11),
                        (80, 30),
                        (120, 40),
                    ] {
                        let mut widget = make_widget(kind, 5, 7, fiesta);
                        widget.resize(w, h);
                        let r = Rect {
                            x: 3,
                            y: 2,
                            w: w as i32,
                            h: h as i32,
                        };
                        for step in 0..40 {
                            widget.update(if step % 9 == 0 { 0.5 } else { 0.07 }, &env);
                            let mut c = Canvas::new(w + 8, h + 6, Color::Reset);
                            widget.draw(&mut c, r, &env);
                            for y in 0..c.h() as i32 {
                                for x in 0..c.w() as i32 {
                                    let inside =
                                        x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h;
                                    if !inside {
                                        let cell = c.get(x, y).unwrap();
                                        assert_eq!(
                                            cell.ch, ' ',
                                            "kind {kind} {m}-{d} {w}x{h} drew {:?} at {x},{y}",
                                            cell.ch
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_tape_brings_the_grito_s_headlines() {
        let theme = Theme::classic();
        let o = opts("");
        let seen = |m: u32, d: u32| {
            let ctx = ctx_of(&theme, &o, Fiesta::of_day(m, d));
            let mut s = Hollywood::new(&ctx);
            s.resize(&ctx, 100, 30);
            let mut seen = String::new();
            for _ in 0..400 {
                s.tape.clear();
                s.feed_tape();
                seen.extend(s.tape.iter());
            }
            seen
        };
        let all = seen(9, 8);
        for line in [
            "GRITO.EXE: CUENTA REGRESIVA A LAS 23:00",
            "\u{00a1}VIVA M\u{00c9}XICO!",
            "CAMPANA DE DOLORES: LISTA",
        ] {
            assert!(all.contains(line), "{line}");
        }
        assert!(seen(9, 15).contains("ESTA NOCHE: EL GRITO A LAS 23:00"));
        assert!(seen(9, 16).contains("DESFILE MILITAR EN CURSO"));
        assert!(!all.contains("DESFILE MILITAR EN CURSO"));
    }

    #[test]
    fn the_himno_scrolls_through_the_log_in_september_only() {
        let theme = Theme::classic();
        let o = opts("");
        let ctx = ctx_of(&theme, &o, Fiesta::of_day(9, 15));
        let env = Env {
            pal: Pal::new(&ctx),
            level: 7,
            day: 0.0,
        };
        let mut seen = String::new();
        for seed in 0..20u64 {
            let mut f = feed::Feed::new(feed::FeedKind::Log, seed, 7, ctx.fiesta);
            f.resize(120, 40);
            for step in 0..400 {
                f.update(0.1, &env);
                if step % 40 == 0 {
                    let mut c = Canvas::new(120, 40, Color::Reset);
                    f.draw(
                        &mut c,
                        Rect {
                            x: 0,
                            y: 0,
                            w: 120,
                            h: 40,
                        },
                        &env,
                    );
                    seen.push_str(&c.to_text());
                }
            }
        }
        assert!(
            seen.contains("\u{266a} Mexicanos, al grito de guerra"),
            "{seen}"
        );
        assert!(!seen.contains("Campana sobre campana"));
    }
}
