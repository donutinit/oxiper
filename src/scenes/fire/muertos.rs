//! Día de Muertos: the bonfire becomes a field of veladoras.
//!
//! Rows of tall glass candle holders stand along the bottom, the front row
//! big and the ones behind smaller and higher, in painted glass (a band of
//! gold, a Guadalupe, a heart, a cross, a flower or a little calavera) lit
//! from inside. The fire's heat buffer is no longer fed along the bottom but
//! at every wick, so aafire's own flames rise out of the glasses, flickering
//! and now and then guttering in a draft. Between the candles grow bunches of
//! cempasúchil, terciopelo and nube, petals are strewn on the ground, and
//! clay sahumadores burn copal whose smoke curls up behind the flames. Sugar
//! skulls come with the angelitos (October 31) and pan de muerto with the
//! Todos Santos; how many candles burn, and how many rows of them, follows
//! the holiday's level.
//!
//! Everything here rolls its own dice (seeded from the scene's seed), so the
//! fire's random numbers on ordinary days are untouched.

use super::{FP, PAD};
use crate::color::{Color, mix_oklab, rgb};
use crate::render::{BOLD, Canvas, Pixels};
use crate::scene::Ctx;

/// Mixed into the scene's seed for the holiday's own dice.
const SALT: u64 = 0x6d75_6572_746f_7321;
/// Rows of candles at most, the front one included.
const MAX_ROWS: usize = 5;
/// How much each row back fades into the night.
const DIM: f32 = 0.15;

/// A pixel sprite: rows of one char per pixel, `.` see-through.
type Sprite = &'static [&'static str];

/// A veladora: a tall glass candle holder, open at the top, with the spot
/// for its painted motif. `r` rim, `w` wax, `G` glass, `h` its lit edge,
/// `g` its shaded edge, `b` a painted band, `d` the thick glass foot.
struct Glass {
    rows: Sprite,
    /// Top row of the motif (columns 1 to 3 of a medium glass, 2 to 4 of a large).
    motif: i32,
}

/// Veladoras by size (small, medium, large), tall and short.
#[rustfmt::skip]
const GLASSES: [[Glass; 2]; 3] = [
    [
        Glass { rows: &["r.r", "rwr", "hGg", "bbb", "hGg", ".d."], motif: -1 },
        Glass { rows: &["r.r", "rwr", "hGg", "bbb", ".d."], motif: -1 },
    ],
    [
        Glass {
            rows: &["r...r", "rwwwr", "hGGGg", "bbbbb", "hGGGg", "hGGGg", "hGGGg",
                    "hGGGg", "bbbbb", ".ddd."],
            motif: 4,
        },
        Glass {
            rows: &["r...r", "rwwwr", "hGGGg", "hGGGg", "hGGGg", "hGGGg", "bbbbb", ".ddd."],
            motif: 2,
        },
    ],
    [
        Glass {
            rows: &["r.....r", "rwwwwwr", "hGGGGGg", "bbbbbbb", "hGGGGGg", "hGGGGGg",
                    "hGGGGGg", "hGGGGGg", "hGGGGGg", "hGGGGGg", "hGGGGGg", "bbbbbbb",
                    "hGGGGGg", "hGGGGGg", ".ddddd."],
            motif: 5,
        },
        Glass {
            rows: &["r.....r", "rwwwwwr", "hGGGGGg", "bbbbbbb", "hGGGGGg", "hGGGGGg",
                    "hGGGGGg", "hGGGGGg", "hGGGGGg", "bbbbbbb", "hGGGGGg", ".ddddd."],
            motif: 4,
        },
    ],
];

/// The same veladoras in characters, for the `ascii` style: one row per
/// two pixel rows, the same widths. `=` wax, `~` the painted band, `M` the
/// motif's row, `'` and `-` on the last row the foot.
#[rustfmt::skip]
const GLASSES_ASCII: [[Sprite; 2]; 3] = [
    [&["| |", "|=|", "'-'"], &["|=|", "'-'"]],
    [&["|   |", "|===|", "|~~~|", "|MMM|", "|~~~|", "'---'"],
     &["|   |", "|===|", "|MMM|", "'---'"]],
    [&["|     |", "|=====|", "|~~~~~|", "| MMM |", "| MMM |", "|     |", "|~~~~~|", "'-----'"],
     &["|     |", "|=====|", "|~~~~~|", "| MMM |", "| MMM |", "'-----'"]],
];

/// Motifs painted on the glass: 3×4 on a large glass, the top 3×3 on a
/// medium one. `o` bone or gold, `m` a second color, `k` dark.
#[rustfmt::skip]
const MOTIFS: [[&str; 4]; 5] = [
    // La Virgen de Guadalupe in her mandorla of gold rays.
    [".o.", "omo", "omo", ".o."],
    // A heart.
    ["m.m", "mmm", ".m.", "..."],
    // A cross.
    [".o.", "ooo", ".o.", ".o."],
    // A flower.
    [".m.", "mom", ".m.", "..."],
    // A little calavera.
    ["ooo", "kok", "ooo", ".o."],
];

/// The motifs in characters, two rows each.
const MOTIFS_ASCII: [[&str; 2]; 5] = [
    ["(^)", "(_)"],
    ["<3 ", "   "],
    [" + ", " | "],
    ["(@)", " Y "],
    ["o.o", " = "],
];

/// Flower heads by size. Cempasúchil: `Y` light, `O` orange, `o` dark,
/// `D` deep. Terciopelo: `V` velvet, `v` its shade. Nube: `n` on twigs `l`.
#[rustfmt::skip]
const MARIGOLDS: [Sprite; 3] = [
    &["YO", "oD"],
    &[".Y.", "YOO", "OoD", ".D."],
    &[".YOY.", "YOYOO", "OYOoO", "OoOoD", ".DoD."],
];
#[rustfmt::skip]
const VELVETS: [Sprite; 3] = [
    &["V"],
    &["VvV", "vVv"],
    &["V.V.V", "VvVvV", "vVVVv", ".vVv."],
];
#[rustfmt::skip]
const NUBES: [Sprite; 3] = [
    &["n"],
    &["n.n", "nl.", ".l."],
    &[".n..n", "n.ln.", ".nl.n", "..l.."],
];

/// Clay sahumadores burning copal: `E`/`e` glowing coals, `c` clay, `C` its
/// shade, `y` painted dots.
#[rustfmt::skip]
const CENSERS: [Sprite; 3] = [
    &["eEe", "ccc", ".C."],
    &[".EeE.", "ccccc", ".yCy.", "..c..", ".CCC."],
    &[".EeEeE.", "ccccccc", ".yCyCy.", "..CCC..", "...c...", "..ccc..", ".CCCCC."],
];
const CENSERS_ASCII: [Sprite; 3] = [
    &["\\_/"],
    &[".:*:.", "\\___/", " /_\\ "],
    &[" .:*:. ", "\\_____/", "  \\_/  ", "  /_\\  "],
];

/// Calaveras de azúcar: `H` sugar, `S` its shade, `E` icing round the eyes,
/// `i` an icing dot, `k` dark.
#[rustfmt::skip]
const SKULLS: [Sprite; 3] = [
    &["HH", "kk"],
    &[".HHH.", "HEHES", "HHkHS", ".HkS."],
    &[".HHHHH.", "HHHiHHS", "HEEHEES", "HEkHEkS", "HHHkHHS", ".HkHkS.", "..HHS.."],
];
const SKULLS_ASCII: [Sprite; 3] = [&["o"], &["(o.o)"], &[" .-. ", "(o o)", " |=| "]];

/// Pan de muerto: `P` crust, `p` its shade, `B` the dough bones, `s` sugar.
#[rustfmt::skip]
const BREADS: [Sprite; 3] = [
    &["PP"],
    &[".sBs.", "PBPBP", ".pPp."],
    &["..sBs..", ".BPBPB.", "PPBPBPp", "PPPPPPp", ".ppppp."],
];
const BREADS_ASCII: [Sprite; 3] = [&["o"], &["(_o_)"], &[" _o_ ", "(___)"]];

#[derive(Clone, Copy, PartialEq)]
enum Item {
    Censer,
    Skull,
    Bread,
}

#[derive(Clone, Copy)]
struct Candle {
    /// Left column and top pixel row.
    x: i32,
    y: i32,
    size: usize,
    tall: bool,
    /// 0 is the front row.
    depth: u8,
    glass: u8,
    accent: u8,
    motif: u8,
    phase: f32,
    flick: f32,
    aim: f32,
    sway: i32,
}

impl Candle {
    /// Far back, a candle is a speck of light: its flame is drawn, too small
    /// for the fire to carry.
    fn far(&self) -> bool {
        self.size == 0 && self.depth > 0
    }
}

#[derive(Clone, Copy)]
struct Bloom {
    /// Left column and top pixel row of the head; the stem runs down to `ground`.
    x: i32,
    y: i32,
    ground: i32,
    size: usize,
    /// 0 cempasúchil, 1 terciopelo, 2 nube.
    kind: u8,
    depth: u8,
    leaf_side: i32,
}

#[derive(Clone, Copy)]
struct Thing {
    item: Item,
    x: i32,
    y: i32,
    size: usize,
    depth: u8,
    icing: u8,
}

#[derive(Clone, Copy)]
struct Puff {
    x: f32,
    y: f32,
    age: f32,
    life: f32,
    /// Which censer it came from, for its own curl.
    from: u8,
    dither: u8,
}

/// One row of candles being drawn over the flames.
struct Layer<'a> {
    ctx: &'a Ctx<'a>,
    px: &'a mut Pixels,
    /// Where the flames are.
    hot: &'a dyn Fn(i32, i32) -> bool,
    /// A row behind the front one: the flames of the rows in front hide it.
    behind: bool,
    /// How far it fades into the night.
    dim: f32,
}

impl Layer<'_> {
    #[inline]
    fn put(&mut self, x: i32, y: i32, col: Color) {
        if !(self.behind && (self.hot)(x, y)) {
            self.px.set(x, y, self.ctx.fade(col, self.dim));
        }
    }
}

/// Colors of the holiday, cached in `recolor`.
struct Pal {
    glass: [Color; 7],
    gold: Color,
    bone: Color,
    bone_shade: Color,
    wax: Color,
    flame: Color,
    flame_edge: Color,
    dark: Color,
    marigold: [Color; 4],
    velvet: [Color; 2],
    nube: Color,
    leaf: [Color; 2],
    clay: [Color; 3],
    bread: [Color; 4],
    smoke: Color,
}

/// Cempasúchil, rosa mexicano, morado, turquesa, amarillo, verde limón,
/// terciopelo: the colors of the glass.
const GLASS: [u32; 7] = [
    0xff8c00, 0xe4007c, 0x7028a0, 0x00aab8, 0xffd200, 0x8cc828, 0xb00048,
];

pub(super) struct Veladoras {
    on: bool,
    level: f32,
    seed: u64,
    rng: fastrand::Rng,
    ascii: bool,
    w: i32,
    ph: i32,
    candles: Vec<Candle>,
    blooms: Vec<Bloom>,
    things: Vec<Thing>,
    /// Petals strewn on the ground: column, pixel row, shade, row of candles.
    petals: Vec<(i32, i32, u8, u8)>,
    puffs: Vec<Puff>,
    /// Smoke emitted but not yet released, per censer.
    smoke_acc: f32,
    t: f32,
    known_bg: bool,
    pal: Pal,
}

/// Sizes of the front row: large, medium or small glasses.
fn front_size(ph: i32) -> Option<usize> {
    match ph {
        60.. => Some(2),
        26.. => Some(1),
        10.. => Some(0),
        _ => None,
    }
}

impl Veladoras {
    pub fn new(seed: u64, ascii: bool) -> Veladoras {
        Veladoras {
            on: false,
            level: 0.0,
            seed,
            rng: fastrand::Rng::with_seed(seed ^ SALT),
            ascii,
            w: 0,
            ph: 0,
            candles: Vec::new(),
            blooms: Vec::new(),
            things: Vec::new(),
            petals: Vec::new(),
            puffs: Vec::new(),
            smoke_acc: 0.0,
            t: 0.0,
            known_bg: false,
            pal: Pal {
                glass: [Color::Reset; 7],
                gold: Color::Reset,
                bone: Color::Reset,
                bone_shade: Color::Reset,
                wax: Color::Reset,
                flame: Color::Reset,
                flame_edge: Color::Reset,
                dark: Color::Reset,
                marigold: [Color::Reset; 4],
                velvet: [Color::Reset; 2],
                nube: Color::Reset,
                leaf: [Color::Reset; 2],
                clay: [Color::Reset; 3],
                bread: [Color::Reset; 4],
                smoke: Color::Reset,
            },
        }
    }

    pub fn on(&self) -> bool {
        self.on
    }

    /// How many candles, flowers and things (censers, skulls, bread) are out.
    #[cfg(test)]
    pub fn counts(&self) -> (usize, usize, usize) {
        (self.candles.len(), self.blooms.len(), self.things.len())
    }

    /// Follows the holiday; returns whether it changed.
    pub fn set(&mut self, on: bool, level: f32) -> bool {
        let level = if level.is_finite() {
            level.clamp(0.0, 1.0)
        } else {
            1.0
        };
        let changed = on != self.on || level != self.level;
        self.on = on;
        self.level = level;
        changed
    }

    pub fn recolor(&mut self, ctx: &Ctx) {
        let themed = !ctx.theme.native;
        let p = &mut self.pal;
        for (i, slot) in p.glass.iter_mut().enumerate() {
            *slot = ctx.pick(i, rgb(GLASS[i]));
        }
        let fg = ctx.theme.fg;
        let or = |native: u32, theme: Color| if themed { theme } else { rgb(native) };
        p.gold = or(0xffd200, mix_oklab(fg, ctx.theme.grad.at(0.8), 0.5));
        p.bone = or(0xf4eee2, fg);
        p.bone_shade = or(0xc8bca8, ctx.fade(fg, 0.3));
        p.dark = or(0x3a2030, ctx.fade(ctx.theme.grad.at(0.0), 0.2));
        // A candle flame and a marigold keep their own hues under any theme.
        p.flame = rgb(0xffd678);
        p.flame_edge = rgb(0xff9628);
        p.wax = rgb(0xf6e6c0);
        p.marigold = [rgb(0xffbe32), rgb(0xff8c00), rgb(0xd66200), rgb(0x9a3c00)];
        p.velvet = [
            ctx.pick(1, rgb(0xd0105c)),
            ctx.pick(1, rgb(0xb00048)).scale(0.7),
        ];
        p.nube = p.bone;
        p.leaf = [
            or(0x3c8c32, ctx.pick(3, Color::Reset).scale(0.65)),
            or(0x24602a, ctx.pick(3, Color::Reset).scale(0.42)),
        ];
        p.clay = [
            or(0xc06438, ctx.theme.grad.at(0.55)),
            or(0x8c4428, ctx.theme.grad.at(0.35)),
            or(0xffd200, ctx.pick(4, Color::Reset)),
        ];
        p.bread = [
            or(0xc87832, ctx.theme.grad.at(0.6)),
            or(0x96501e, ctx.theme.grad.at(0.4)),
            or(0xe6a050, ctx.theme.grad.at(0.75)),
            or(0xfff0dc, fg),
        ];
        p.smoke = or(0xd8d4e4, fg);
        self.known_bg = matches!(ctx.bg, Color::Rgb(..));
    }

    /// Sets the candles, flowers and things out for a screen of `w` columns
    /// and `ph` pixel rows, as many as the holiday's level brings.
    pub fn layout(&mut self, w: usize, ph: usize) {
        self.candles.clear();
        self.blooms.clear();
        self.things.clear();
        self.petals.clear();
        self.puffs.clear();
        let (w, ph) = (w as i32, ph as i32);
        self.w = w;
        self.ph = ph;
        let Some(front) = front_size(ph) else {
            return;
        };
        if !self.on {
            return;
        }
        let mut rng = fastrand::Rng::with_seed(self.seed ^ SALT ^ 0x5eed);
        let level = self.level;
        let ascii = self.ascii;
        let ground = ph - 1;
        // Rows of candles, front to back, climbing the screen like a hillside
        // of graves on the night of the vigil: how big, where their feet
        // stand, and how much of the row the holiday has lit. The front row
        // is there from the first day; the ones behind light up day by day.
        let mut rows = [(0usize, 0i32, 0.0f32); MAX_ROWS];
        let spacing = match front {
            2 => (ph / 8).max(8),
            1 => (ph / 9).max(4),
            _ => 0,
        };
        let nrows = match front {
            0 => 1,
            2 if ph >= 100 => 4,
            2 => 3,
            _ if ph >= 40 => 3,
            _ => 2,
        };
        for (k, row) in rows[..nrows].iter_mut().enumerate() {
            let k32 = k as i32;
            // Closer together as they recede.
            let mut lift = k32 * spacing - k32 * (k32 - 1) * spacing / 8;
            if ascii {
                lift &= !1;
            }
            let start = [0.0, 0.2, 0.4, 0.6, 0.75][k];
            let lit = if k == 0 {
                0.75 + 0.25 * level
            } else {
                ((level - start) / (1.0 - start)).clamp(0.0, 1.0)
            };
            *row = (front.saturating_sub(k), ground - lift, lit);
        }
        let width = |size: usize| [3, 5, 7][size];
        let height = |size: usize, tall: bool| {
            let t = if tall { 0 } else { 1 };
            if ascii {
                2 * GLASSES_ASCII[size][t].len() as i32
            } else {
                GLASSES[size][t].rows.len() as i32
            }
        };
        let front_pitch = width(front) * 2 + 1;
        let mut censers_left = if w >= 4 * front_pitch && level >= 0.8 {
            2
        } else if w >= 2 * front_pitch {
            1
        } else {
            0
        };
        for (depth, &(size, feet, lit)) in rows[..nrows].iter().enumerate() {
            let cw = width(size);
            let pitch = if depth == 0 { front_pitch } else { cw * 2 + 2 };
            let n = ((w - 2) / pitch).max(1);
            let span = n * pitch - (pitch - cw);
            let mut x0 = (w - span) / 2 + rng.i32(0..=2);
            if depth % 2 == 1 {
                // Behind the gaps of the row in front.
                x0 += pitch / 2;
            }
            // Where the sahumadores go: away from the edges and each other.
            let censer_slots = if depth == 0 && n >= 3 {
                [n / 4 + (n >= 8) as i32, n - 1 - n / 4]
            } else {
                [-1, -1]
            };
            let mut last_glass = u8::MAX;
            for i in 0..n {
                let x = x0 + i * pitch + rng.i32(-1..=1);
                let rank = rng.f32();
                let tall = rng.bool();
                let mut glass = rng.u8(0..7);
                if glass == last_glass {
                    glass = (glass + 1 + rng.u8(0..6)) % 7;
                }
                let accent = (glass + 2 + rng.u8(0..5)) % 7;
                let motif = rng.u8(0..MOTIFS.len() as u8);
                let phase = rng.f32() * std::f32::consts::TAU;
                if x < 0 || x + cw > w {
                    continue;
                }
                if censer_slots[..censers_left.min(2)].contains(&i) {
                    let cs = CENSERS[size].len() as i32;
                    let ch = if ascii {
                        2 * CENSERS_ASCII[size].len() as i32
                    } else {
                        cs
                    };
                    self.things.push(Thing {
                        item: Item::Censer,
                        x: x + (cw - CENSERS[size][0].len() as i32) / 2,
                        y: feet + 1 - ch,
                        size,
                        depth: depth as u8,
                        icing: 0,
                    });
                    continue;
                }
                if rank >= lit {
                    continue;
                }
                last_glass = glass;
                let h = height(size, tall);
                self.candles.push(Candle {
                    x,
                    y: feet + 1 - h,
                    size,
                    tall,
                    depth: depth as u8,
                    glass,
                    accent,
                    motif,
                    phase,
                    flick: 1.0,
                    aim: 1.0,
                    sway: 0,
                });
            }
            censers_left = 0;
            if depth <= 1 && size + depth == front {
                // Bunches of flowers in the gaps of the two front rows, in
                // front of the row behind.
                let fsize = size;
                let ground = feet;
                let tops = [(1, 3), (2, 7), (3, 10)][fsize];
                for i in -1..n {
                    let gx = x0 + i * pitch + cw + (pitch - cw) / 2;
                    let fill = if depth == 0 {
                        0.35 + 0.65 * level
                    } else {
                        lit * 0.7
                    };
                    if rng.f32() > fill {
                        continue;
                    }
                    let heads = 1 + rng.usize(0..=(1 + (level * 2.5) as usize) >> depth);
                    for k in 0..heads {
                        let kind = match rng.u8(0..20) {
                            0..=12 => 0,
                            13..=16 => 1,
                            _ => 2,
                        };
                        let hw = [2, 3, 5][fsize];
                        let hx = gx - hw / 2 + rng.i32(-(hw / 2 + 1)..=(hw / 2 + 1));
                        let lift = rng.i32(tops.0..=tops.1) - k as i32 / 2;
                        let hy = ground - lift.max(1);
                        if hx < -1 || hx + hw > w + 1 {
                            continue;
                        }
                        self.blooms.push(Bloom {
                            x: hx,
                            y: hy - [2, 4, 5][fsize] + 1,
                            ground,
                            size: fsize,
                            kind,
                            depth: depth as u8,
                            leaf_side: if rng.bool() { 1 } else { -1 },
                        });
                    }
                    // A sugar skull or a pan de muerto at the foot of some.
                    let skull = level >= 0.6 && rng.f32() < 0.15 + 0.25 * level;
                    let bread = level >= 0.8 && rng.f32() < 0.2;
                    if depth > 0 {
                        continue;
                    }
                    let item = if skull {
                        Some(Item::Skull)
                    } else if bread {
                        Some(Item::Bread)
                    } else {
                        None
                    };
                    if let Some(item) = item {
                        let art = match item {
                            Item::Skull => SKULLS[fsize],
                            _ => BREADS[fsize],
                        };
                        let iw = art[0].len() as i32;
                        let ih = if ascii {
                            2 * match item {
                                Item::Skull => SKULLS_ASCII[fsize].len(),
                                _ => BREADS_ASCII[fsize].len(),
                            } as i32
                        } else {
                            art.len() as i32
                        };
                        let ix = gx - iw / 2 + rng.i32(-1..=1);
                        if ix >= 0 && ix + iw <= w {
                            self.things.push(Thing {
                                item,
                                x: ix,
                                y: ground + 1 - ih,
                                size: fsize,
                                depth: 0,
                                icing: rng.u8(0..7),
                            });
                        }
                    }
                }
            }
        }
        // Petals strewn on the ground, the path home.
        let density = 0.12 + 0.45 * level;
        for x in 0..w {
            for (dy, k) in [(0, 1.0), (1, 0.35)] {
                if rng.f32() < density * k {
                    self.petals.push((x, ground - dy, rng.u8(0..8), 0));
                }
            }
        }
        let censers = self
            .things
            .iter()
            .filter(|t| t.item == Item::Censer)
            .count();
        self.puffs.reserve_exact(censers * 400);
    }

    /// Feeds the fire at every wick (in place of aafire's fuel along the
    /// bottom), one simulation step.
    pub fn feed(&mut self, heat: &mut [u16], stride: usize, sw: usize, intensity: f32, wind: f32) {
        let t = self.t;
        for c in &mut self.candles {
            // The flame breathes, now and then a draft makes it gutter.
            c.flick += (c.aim - c.flick) * 0.3;
            if self.rng.f32() < 0.08 {
                c.aim = 0.8 + self.rng.f32() * 0.25;
            }
            if self.rng.f32() < 0.003 {
                c.aim = 0.45;
            }
            let lean = (t * 1.3 + c.phase).sin() * 1.4 + (t * 3.7 + c.phase * 2.0).sin() * 0.7;
            c.sway = (lean + wind).round().clamp(-3.0, 3.0) as i32;
            if c.far() {
                continue;
            }
            let cw = [3, 5, 7][c.size];
            let half = [1, 3, 4][c.size];
            let base = [255.0, 200.0][c.depth.min(1) as usize];
            let mid = 2 * (c.x + cw / 2) + 1 + c.sway;
            for dy in 0..2 {
                let y = c.y + dy;
                if y < 0 {
                    continue;
                }
                let row = y as usize * stride + PAD;
                for sx in mid - half..mid + half {
                    if sx < 0 || sx as usize >= sw || row + sx as usize >= heat.len() {
                        continue;
                    }
                    // Hottest in the middle of the wick, so the edges die
                    // first and the flame tapers to a tip.
                    let d = (sx - mid) as f32 + 0.5;
                    let edge = 1.0 - 0.75 * (d / half as f32).powi(2);
                    let v = (base - self.rng.f32() * 50.0) * intensity * c.flick * edge;
                    heat[row + sx as usize] = v.clamp(0.0, 255.0) as u16 * FP as u16;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32, wind: f32) {
        self.t += dt;
        let t = self.t;
        let ph = self.ph as f32;
        // Copal: a steady trickle of smoke from each sahumador.
        self.smoke_acc += dt * 26.0;
        while self.smoke_acc >= 1.0 {
            self.smoke_acc -= 1.0;
            for (i, s) in self.things.iter().enumerate() {
                if s.item != Item::Censer || self.puffs.len() >= self.puffs.capacity() {
                    continue;
                }
                let cw = CENSERS[s.size][0].len() as f32;
                self.puffs.push(Puff {
                    x: s.x as f32 + cw / 2.0 + (self.rng.f32() - 0.5) * 1.5,
                    y: s.y as f32,
                    age: 0.0,
                    life: 7.0 + self.rng.f32() * 4.0,
                    from: i as u8,
                    dither: self.rng.u8(..),
                });
            }
        }
        let rise = 2.5 + ph * 0.055;
        self.puffs.retain_mut(|p| {
            p.age += dt;
            let k = p.age / p.life;
            // Straight up out of the bowl, then curling as it climbs and slows.
            let a = (p.age * 0.9).min(3.5);
            let f = p.from as f32 * 1.7;
            let curl = (p.y * 0.16 + t * 0.55 + f).sin() + 0.6 * (p.y * 0.37 - t * 0.9 + f).sin();
            p.x += (a * curl + wind * 3.0 + 0.4) * dt;
            p.y -= rise * (1.0 - 0.55 * k) * dt;
            p.age < p.life && p.y > -1.0
        });
    }

    /// Smoke, drawn first: the flames and everything else are in front.
    pub fn draw_smoke(&self, ctx: &Ctx, px: &mut Pixels) {
        for p in &self.puffs {
            let k = p.age / p.life;
            let alpha = (1.0 - k).powf(1.3) * (p.age * 2.0).min(1.0);
            let col = if self.known_bg {
                ctx.fade(self.pal.smoke, 1.0 - alpha * 0.75)
            } else if (p.dither as f32) < alpha * 300.0 {
                self.pal.smoke.scale(0.8)
            } else {
                continue;
            };
            // Thin out of the bowl, a wider plume as it rises.
            let spread = (p.dither as f32 / 255.0 - 0.5) * k * 5.0;
            let (x, y) = ((p.x + spread).round() as i32, p.y.round() as i32);
            px.set(x, y, col);
            if k > 0.3 && p.dither & 1 == 0 {
                px.set(x + 1, y, col);
            }
        }
    }

    /// The candles, flowers and things, back to front, over the flames.
    /// `hot` says where the flames are: the rows behind stay behind them.
    pub fn draw(&self, ctx: &Ctx, px: &mut Pixels, hot: &dyn Fn(i32, i32) -> bool) {
        for &(x, y, k, depth) in &self.petals {
            let col = match k {
                0..=2 => self.pal.marigold[1],
                3 | 4 => self.pal.marigold[2],
                5 => self.pal.marigold[0],
                6 => self.pal.marigold[3],
                _ => self.pal.velvet[0],
            };
            px.set(x, y, ctx.fade(col, depth as f32 * DIM));
        }
        for depth in (0..MAX_ROWS as u8).rev() {
            let mut l = Layer {
                ctx,
                px: &mut *px,
                hot,
                behind: depth > 0,
                dim: depth as f32 * DIM,
            };
            for c in self.candles.iter().filter(|c| c.depth == depth) {
                self.draw_candle(&mut l, c);
            }
            for b in self.blooms.iter().filter(|b| b.depth == depth) {
                self.draw_bloom(&mut l, b);
            }
            for s in self.things.iter().filter(|s| s.depth == depth) {
                self.draw_thing(&mut l, s);
            }
        }
    }

    fn draw_candle(&self, l: &mut Layer, c: &Candle) {
        let p = &self.pal;
        let g = GLASSES[c.size][!c.tall as usize].rows;
        let glass = p.glass[c.glass as usize];
        let accent = p.glass[c.accent as usize];
        let h = g.len() as f32;
        let lit = c.flick.clamp(0.0, 1.2);
        for (ry, row) in g.iter().enumerate() {
            // Lit from inside: brightest just under the flame.
            let near = (1.0 - ry as f32 / (h * 0.85)).max(0.0);
            let glow = lit * near * near * 0.55;
            let body = glass.scale(0.55 + 0.35 * near).lerp(p.flame_edge, glow);
            for (rx, ch) in row.bytes().enumerate() {
                let col = match ch {
                    b'r' => mix_oklab(glass, p.flame, 0.55 * lit),
                    b'w' => p.wax.lerp(p.flame, 0.5 * lit),
                    b'G' => body,
                    b'h' => body.lerp(p.bone, 0.35),
                    b'g' => body.scale(0.72),
                    b'b' => p.gold.scale(0.6 + 0.4 * near).lerp(p.flame, glow * 0.5),
                    b'd' => glass.scale(0.4),
                    _ => continue,
                };
                l.put(c.x + rx as i32, c.y + ry as i32, col);
            }
        }
        if c.far() {
            // A speck of a flame: a white-hot core, then gold, then a tip
            // that comes and goes and leans with the draft.
            let x = c.x + 1;
            let dim = std::mem::replace(&mut l.dim, 0.0);
            l.put(x, c.y, p.flame.lerp(p.wax, 0.5));
            l.put(x, c.y - 1, p.flame);
            if lit > 0.88 {
                l.put(x + c.sway / 2, c.y - 2, p.flame_edge);
            }
            l.dim = dim;
        }
        let anchor = GLASSES[c.size][!c.tall as usize].motif;
        if anchor >= 0 {
            let dx = if c.size == 2 { 2 } else { 1 };
            let rows = if c.size == 2 { 4 } else { 3 };
            let motif = MOTIFS[c.motif as usize];
            let gold = c.motif == 0;
            for (ry, row) in motif[..rows].iter().enumerate() {
                for (rx, ch) in row.bytes().enumerate() {
                    let col = match ch {
                        b'o' if gold => p.gold,
                        b'o' => p.bone,
                        b'm' => accent,
                        b'k' => glass.scale(0.3),
                        _ => continue,
                    };
                    let col = col.scale(0.8 + 0.2 * lit.min(1.0));
                    l.put(c.x + dx + rx as i32, c.y + anchor + ry as i32, col);
                }
            }
        }
    }

    fn draw_bloom(&self, l: &mut Layer, b: &Bloom) {
        let p = &self.pal;
        let art = match b.kind {
            0 => MARIGOLDS[b.size],
            1 => VELVETS[b.size],
            _ => NUBES[b.size],
        };
        let hw = art[0].len() as i32;
        let sx = b.x + hw / 2;
        // The stem, with a leaf every few pixels.
        let top = b.y + art.len() as i32;
        for y in top..=b.ground {
            l.put(sx, y, p.leaf[0]);
            let k = b.ground - y;
            if k > 0 && k % 3 == 1 {
                let side = if (k / 3) % 2 == 0 {
                    b.leaf_side
                } else {
                    -b.leaf_side
                };
                l.put(sx + side, y, p.leaf[1]);
                if b.size == 2 {
                    l.put(sx + 2 * side, y - 1, p.leaf[1]);
                }
            }
        }
        for (ry, row) in art.iter().enumerate() {
            for (rx, ch) in row.bytes().enumerate() {
                let col = match ch {
                    b'Y' => p.marigold[0],
                    b'O' => p.marigold[1],
                    b'o' => p.marigold[2],
                    b'D' => p.marigold[3],
                    b'V' => p.velvet[0],
                    b'v' => p.velvet[1],
                    b'n' => p.nube,
                    b'l' => p.leaf[0],
                    _ => continue,
                };
                l.put(b.x + rx as i32, b.y + ry as i32, col);
            }
        }
    }

    fn draw_thing(&self, l: &mut Layer, s: &Thing) {
        let p = &self.pal;
        let art = match s.item {
            Item::Censer => CENSERS[s.size],
            Item::Skull => SKULLS[s.size],
            Item::Bread => BREADS[s.size],
        };
        let coals = 0.75 + 0.25 * (self.t * 7.0 + s.x as f32).sin();
        for (ry, row) in art.iter().enumerate() {
            for (rx, ch) in row.bytes().enumerate() {
                let col = match (s.item, ch) {
                    (Item::Censer, b'E') => p.flame.lerp(p.flame_edge, 1.0 - coals),
                    (Item::Censer, b'e') => p.flame_edge.scale(0.6 + 0.4 * coals),
                    (Item::Censer, b'c') => p.clay[0],
                    (Item::Censer, b'C') => p.clay[1],
                    (Item::Censer, b'y') => p.clay[2],
                    (Item::Skull, b'H') => p.bone,
                    (Item::Skull, b'S') => p.bone_shade,
                    (Item::Skull, b'E') => p.glass[s.icing as usize],
                    (Item::Skull, b'i') => p.glass[(s.icing as usize + 3) % 7],
                    (Item::Skull, b'k') => p.dark,
                    (Item::Bread, b'P') => p.bread[0],
                    (Item::Bread, b'p') => p.bread[1],
                    (Item::Bread, b'B') => p.bread[2],
                    (Item::Bread, b's') => p.bread[3],
                    _ => continue,
                };
                l.put(s.x + rx as i32, s.y + ry as i32, col);
            }
        }
    }

    /// The `ascii` style: smoke, candles, flowers and things in characters.
    pub fn draw_ascii(&self, ctx: &Ctx, c: &mut Canvas, hot: &dyn Fn(i32, i32) -> bool) {
        let p = &self.pal;
        for q in &self.puffs {
            let k = q.age / q.life;
            let alpha = (1.0 - k).powf(1.3) * (q.age * 2.0).min(1.0);
            if !self.known_bg && (q.dither as f32) >= alpha * 300.0 {
                continue;
            }
            let (x, y) = (q.x.round() as i32, (q.y / 2.0).floor() as i32);
            if c.get(x, y).is_none_or(|cell| cell.ch != ' ') {
                continue;
            }
            let ch = match (k * 4.0) as u8 {
                0 => '|',
                1 => ['(', ')'][(q.dither & 1) as usize],
                2 => '~',
                _ => '.',
            };
            c.set(x, y, ch, ctx.fade(p.smoke, 1.0 - alpha * 0.75));
        }
        for &(x, y, k, _) in &self.petals {
            if y == self.ph - 1 && c.get(x, y / 2).is_some_and(|cell| cell.ch == ' ') {
                let col = if k == 7 {
                    p.velvet[0]
                } else {
                    p.marigold[1 + (k as usize % 2)]
                };
                c.set(x, y / 2, [',', '.', '\''][k as usize % 3], col);
            }
        }
        for depth in (0..MAX_ROWS as u8).rev() {
            let dim = depth as f32 * DIM;
            // The rows behind stay behind the flames.
            let hidden = |x: i32, y: i32| depth > 0 && (hot(x, 2 * y) || hot(x, 2 * y + 1));
            for v in self.candles.iter().filter(|v| v.depth == depth) {
                let art = GLASSES_ASCII[v.size][!v.tall as usize];
                let glass = p.glass[v.glass as usize];
                let lit = v.flick.clamp(0.0, 1.0);
                let n = art.len() as i32;
                let mut m = 0;
                for (ry, row) in art.iter().enumerate() {
                    let y = v.y / 2 + ry as i32;
                    let near = 1.0 - ry as f32 / n as f32;
                    let body = glass.lerp(p.flame_edge, lit * near * near * 0.5);
                    let mrow = MOTIFS_ASCII[v.motif as usize][m.min(1)].as_bytes();
                    let mut mi = 0;
                    let inside = row.len() - 1;
                    for (rx, ch) in row.chars().enumerate() {
                        let x = v.x + rx as i32;
                        if hidden(x, y) {
                            continue;
                        }
                        let (ch, col) = match ch {
                            // The glass hides what burns behind it.
                            ' ' if ry > 0 && rx > 0 && rx < inside => (' ', Color::Reset),
                            ' ' if ry == 0 && v.far() && rx == 1 => ('*', p.flame),
                            ' ' => continue,
                            '=' => ('=', p.wax.lerp(p.flame, 0.5 * lit)),
                            '~' => ('~', p.gold),
                            'M' => {
                                mi += 1;
                                match mrow[(mi - 1).min(2)] {
                                    b' ' => (' ', Color::Reset),
                                    mc => (mc as char, p.glass[v.accent as usize]),
                                }
                            }
                            '\'' | '-' if ry + 1 == art.len() => (ch, glass.scale(0.6)),
                            _ => (ch, body),
                        };
                        c.set(x, y, ch, ctx.fade(col, dim));
                    }
                    if mi > 0 {
                        m += 1;
                    }
                }
            }
            for b in self.blooms.iter().filter(|b| b.depth == depth) {
                let hw = MARIGOLDS[b.size][0].len() as i32;
                let x = b.x + hw / 2;
                let top = b.y / 2;
                for y in top + 1..=b.ground / 2 {
                    let leaf = (b.ground / 2 - y) % 2 == 1;
                    let ch = match (leaf, b.leaf_side) {
                        (true, 1) => '/',
                        (true, _) => '\\',
                        _ => '|',
                    };
                    if !hidden(x, y) {
                        c.set(x, y, ch, ctx.fade(p.leaf[0], dim));
                    }
                }
                let (ch, col) = match b.kind {
                    0 => ('@', p.marigold[1]),
                    1 => ('&', p.velvet[0]),
                    _ => ('*', p.nube),
                };
                if hidden(x, top) {
                    continue;
                }
                c.set_attr(x, top, ch, ctx.fade(col, dim), BOLD);
                if b.size == 2 && b.kind == 0 {
                    c.set_attr(x - 1, top, '(', ctx.fade(p.marigold[2], dim), 0);
                    c.set_attr(x + 1, top, ')', ctx.fade(p.marigold[2], dim), 0);
                }
            }
            for s in self.things.iter().filter(|s| s.depth == depth) {
                let (art, col) = match s.item {
                    Item::Censer => (CENSERS_ASCII[s.size], p.clay[0]),
                    Item::Skull => (SKULLS_ASCII[s.size], p.bone),
                    Item::Bread => (BREADS_ASCII[s.size], p.bread[0]),
                };
                for (ry, row) in art.iter().enumerate() {
                    for (rx, ch) in row.chars().enumerate() {
                        let col = match (s.item, ch) {
                            (_, ' ') => continue,
                            (Item::Censer, '.' | ':' | '*') => p.flame_edge,
                            (Item::Skull, 'o') => p.glass[s.icing as usize],
                            (Item::Bread, 'o') => p.bread[2],
                            _ => col,
                        };
                        let (x, y) = (s.x + rx as i32, s.y / 2 + ry as i32);
                        if !hidden(x, y) {
                            c.set(x, y, ch, ctx.fade(col, dim));
                        }
                    }
                }
            }
        }
    }
}
