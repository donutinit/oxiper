//! The fireworks on the Jueves de Corpus: the atrium of a village church on
//! the night of the procession, with its papel picado, its candles, the
//! children who bring huacales of flowers and a mulita de hoja de maíz to it,
//! and the shells that are flowers of papel and a mulita in sparks.

use super::*;

/// Seeds the Jueves de Corpus's own random numbers.
const CORPUS_SEED: u64 = 0x636f_7270_7573_2121;

// The Jueves de Corpus under `classic`: the colors of papel de china and of the
// hoja de maíz, brightened to burn in the night sky.
pub(super) const CO_ROSA: V3 = c8(255, 28, 140);
pub(super) const CO_AMARILLO: V3 = c8(255, 214, 24);
pub(super) const CO_MORADO: V3 = c8(168, 84, 236);
pub(super) const CO_NARANJA: V3 = c8(255, 140, 26);
pub(super) const CO_LIMON: V3 = c8(128, 200, 56);
pub(super) const CO_REBOZO: V3 = c8(64, 124, 235);
pub(super) const CO_HOJA: V3 = c8(236, 208, 130);

/// A flower of papel: petals in rosa mexicano round one in amarillo, a
/// morado heart ringed in naranja.
pub(super) const FLOR_PAPEL: &[Stroke] = &[
    Stroke::Leaf(0.0, 0.0, 1.0, 0.24, 7, 0.0, 0),
    Stroke::Leaf(0.0, 0.0, 0.7, 0.26, 7, 0.07, 1),
    Stroke::Bloom(0.0, 0.0, 0.3, 0.3, 7.0, 2),
    Stroke::Arc(0.0, 0.0, 0.1, 0.0, 1.0, 3),
];

/// A mulita de hoja de maíz facing right, her huacal heaped with flowers: `7`
/// the hoja, `3` the wood of the huacal, `6` and `5` her rebozo and the
/// manta under it, `0`, `1` and `2` the flowers (rosa, amarillo, morado) and
/// `4` their leaves. The digits are the colors of a shaped shell
/// (`Fest::shape`).
pub(super) const MULITA: &[&str] = &[
    "        2224000 4            ",
    "        2221000 111          ",
    "     1 422210002111          ",
    "    111001131122111          ",
    "     10101111221224   7  7   ",
    "    4 400 114 222  4 77 77   ",
    "     333 333 333 333 77 77   ",
    "     333 333 333 333 77 77   ",
    "     333 333 333 333 770777  ",
    "     333 333 333 3377777 77  ",
    "     333 333 333 337777777777",
    "   7776666666666666777777777 ",
    "   77766666666666667777777777",
    "  777755555555555557         ",
    "  777757575757575757         ",
    "     777777777777777         ",
    "      77  77   77 77         ",
    "      77  77   77 77         ",
    "      77  77   77 77         ",
];

/// A string of papel picado's flag: `#` paper, `.` the cut-out.
const PICADO: [&str; 4] = ["###", "#.#", "###", "#.#"];

/// The children and the mulita who walk to the church, facing left, one char
/// per pixel and two frames each, their legs a step apart: `h` hair, `s`
/// skin, `m` manta, `r` rebozo, `w` the wood of a huacal and `p`, `y`, `v`,
/// `o` and `g` the flowers, fruit and leaves it is heaped with, `l`
/// huaraches, `H` and `T` the hoja de maíz and its toasted folds, `e` an eye.
pub(super) const NINO: [&[&str]; 2] = [
    &[
        "...hh.yp", "...ss.gv", "..mmmwww", "..mmmwow", "..mmmwww", "..mmm...", "..mmm...",
        "..l.l...", ".l...l..",
    ],
    &[
        "...hh.yp", "...ss.gv", "..mmmwww", "..mmmwow", "..mmmwww", "..mmm...", "..mmm...",
        "...l....", "...ll...",
    ],
];
pub(super) const NINA: [&[&str]; 2] = [
    &[
        "...hh.op", "...ss.gy", "..rrrwww", "..rrrwyw", "..rrmwww", "..mmmm..", "..mmmm..",
        "..l..l..", ".l....l.",
    ],
    &[
        "...hh.op", "...ss.gy", "..rrrwww", "..rrrwyw", "..rrmwww", "..mmmm..", "..mmmm..",
        "...ll...", "...ll...",
    ],
];
pub(super) const MULITA_CHICA: [&[&str]; 2] = [
    &[
        ".H.H..ypgv..",
        ".HHH.wwwww..",
        "HeHHrwowwrH.",
        "HHHHrrrrrrHH",
        "..HHHHHHHHHT",
        "..HHHHHHHHT.",
        "..H.H..H.H..",
        "..T.T..T.T..",
    ],
    &[
        ".H.H..ypgv..",
        ".HHH.wwwww..",
        "HeHHrwowwrH.",
        "HHHHrrrrrrHH",
        "..HHHHHHHHHT",
        "..HHHHHHHHT.",
        "...HH..HH...",
        "...TT..TT...",
    ],
];
/// Who walks, from the front of the procession to the back, and how many
/// pixels behind the first each stands.
pub(super) const PROCESION: [(&[&[&str]; 2], i32); 3] =
    [(&NINO, 0), (&MULITA_CHICA, 9), (&NINA, 22)];

/// Where a string of papel picado hangs at column `x`: it sags a little in
/// the middle.
fn string_y(a: &Actor, x: i32) -> i32 {
    let span = a.size.max(1.0);
    let u = ((x as f32 - a.x) / span).clamp(0.0, 1.0);
    (a.y + (a.left as f32 - a.y) * u + span * 0.28 * u * (1.0 - u)).round() as i32
}

impl Fireworks {
    /// Lays out the atrium that stands in for the city on the Jueves de
    /// Corpus: a village church (its tower with the bell lit in the belfry,
    /// the gable of the nave with its rose window, the dome behind, the door
    /// open on the candles inside), the houses of the village either side, a
    /// stone cross, a tree and two poles in the atrium, strings of papel
    /// picado from the church to the poles and the rooftops, a crowd with
    /// banderitas, the children and their mulita walking to the church and a
    /// castillo to be lit; the cohetes go up from the atrium. The layout
    /// depends only on the seed and the size.
    pub(super) fn build_atrio(&mut self) {
        let (w, ph) = (self.w, self.ph);
        if !self.fiesta.is(Kind::Corpus) || !self.show_city || w == 0 || ph < 12 {
            return;
        }
        self.town.resize(w * ph, 0);
        self.town_col.resize(w * ph, [0.0; 3]);
        let rng = &mut fastrand::Rng::with_seed(self.seed ^ CORPUS_SEED);
        let lit = 0.9;
        let k = (ph / 50).clamp(1, 3);
        let ground = (if ph >= 40 { 2 } else { 1 }) * k;
        let gy = ph - ground;
        self.lay_ground(rng, gy);
        let mut tops = vec![gy as i32; w];
        let wi = w as i32;
        // The church is 31 units wide and 25 tall: a unit is a pixel, or a
        // block of them on a big screen, as long as it fits.
        let u = k.min(w / 56).min(ph / 40) as i32;
        let b = gy as i32;
        if u < 1 {
            self.facades(rng, (0, w), gy, k, lit, &mut tops);
            self.crowd(rng, gy, k, 0.6, 0.12);
            self.sites.push(wi / 2);
            if w >= 24 {
                let (l, r) = (rng.i32(0..wi / 3), rng.i32(wi * 2 / 3..wi));
                let (a, z) = ((l, tops[l as usize] - 1), (r, tops[r as usize] - 1));
                self.hang_papel(rng, a, z, k);
            }
            return;
        }
        let x0 = (wi * 30 / 100 + rng.i32(-wi / 12..wi / 12 + 1)).clamp(2 * u, wi - 52 * u);
        let ax = x0 + 36 * u;
        let x1 = (ax + (wi / 5).clamp(14 * u, 36 * u)).min(wi - 4);
        self.facades(rng, (0, x0.max(0) as usize), gy, k, lit, &mut tops);
        self.facades(rng, (x1 as usize, w), gy, k, lit, &mut tops);
        self.crowd(rng, gy, k, 0.6, 0.12);
        self.build_church((x0, b), u, lit, rng);

        // The cross of the atrium, in stone, and a tree beside it.
        let ft = &self.fest;
        let (stone, trim, pine, wood) = (ft.wall[0], ft.white, ft.pine, ft.wood);
        let stone = mix(stone, trim, 0.3);
        self.rect(ax - u, b - 2 * u, 3 * u, 2 * u, stone);
        self.rect(ax, b - 9 * u, u, 7 * u, stone);
        self.rect(ax - u, b - 7 * u, 3 * u, u, stone);
        let tx = ax + 9 * u;
        self.rect(tx, b - 5 * u, u, 5 * u, wood);
        self.disc(
            ((tx + u / 2) as f32, (b - 6 * u) as f32),
            3.5 * u as f32,
            false,
            scale(pine, 0.8),
        );
        self.disc(
            ((tx - 2 * u) as f32, (b - 5 * u) as f32),
            2.5 * u as f32,
            false,
            pine,
        );
        self.disc(
            ((tx + 3 * u) as f32, (b - 5 * u) as f32),
            2.5 * u as f32,
            false,
            pine,
        );

        // Two poles hold the strings of papel picado between the church and
        // the houses.
        let (p1, p2) = (ax + 4 * u, x1 - 2 * u);
        for (px, hp) in [(p1, 13 * u), (p2, 15 * u)] {
            self.rect(px, b - hp, u, hp, wood);
        }
        let right = (x1 + rng.i32(6 * u..14 * u + 1)).min(wi - 1);
        let left = rng.i32(0..(x0 - 4 * u).max(1));
        let (left, right) = (left as usize, right as usize);
        let strings = [
            ((left as i32, tops[left] - 1), (x0 + 7 * u, b - 14 * u)),
            ((x0 + 7 * u, b - 14 * u), (p1, b - 13 * u)),
            ((p1, b - 13 * u), (p2, b - 15 * u)),
            ((p2, b - 15 * u), (right as i32, tops[right] - 1)),
        ];
        for (from, to) in strings {
            self.hang_papel(rng, from, to, u as usize);
        }

        // The cohetes of the procession go up from the atrium.
        self.sites
            .extend([x0 + 16 * u, ax, (ax + x1) / 2, x1 - 3 * u]);
        // The castillo, in front of the houses on the right.
        if w >= 70 && ph >= 36 {
            self.actors.push(Actor {
                role: Role::Castillo,
                x: ((x1 + wi) / 2).min(wi - 8) as f32,
                y: gy as f32,
                size: (ph as f32 * 0.34).clamp(14.0, 90.0),
                state: 0,
                left: rng.i32(120..240),
                seed: rng.f32() * TAU,
            });
        }
        // The children and their mulita, on their way to the church door.
        if w >= 60 {
            self.actors.push(Actor {
                role: Role::Procesion,
                x: (wi * 65 / 100 + rng.i32(0..wi / 10 + 1)) as f32,
                y: gy as f32,
                size: k as f32,
                state: 0,
                left: 0,
                seed: (x0 + 14 * u) as f32,
            });
        }
    }

    /// The church, its tower's left edge at `x0`, in units of `u` pixels
    /// standing on the ground row: first the dome that rises behind the
    /// nave, then the tower and the nave in front of it.
    fn build_church(&mut self, (x0, b): (i32, i32), u: i32, lit: f32, rng: &mut fastrand::Rng) {
        let w = self.w;
        let ft = &self.fest;
        let (wall, window, roof, gold, trim) = (ft.wall[0], ft.window, ft.roof, ft.gold, ft.white);
        // Whitewash, warm in the light of the candles and the lamps.
        let wall = scale(mix(wall, gold, 0.4), 1.5);
        let trim = mix(wall, trim, 0.45);
        let shade = scale(wall, 0.8);
        let dark = scale(ft.stone, 0.5);
        let glow = scale(window, 1.0 + 0.3 * lit);
        let nx = x0 + 16 * u;
        // The dome, with its drum, lantern and cross, over a low wing of the
        // church.
        let dome = mix(roof, gold, 0.12);
        self.rect(x0 + 25 * u, b - 8 * u, 6 * u, 8 * u, shade);
        self.rect(x0 + 24 * u, b - 9 * u, 8 * u, u, trim);
        self.rect(x0 + 21 * u, b - 10 * u, 10 * u, 2 * u, shade);
        self.disc(
            ((x0 + 26 * u) as f32, (b - 10 * u) as f32),
            5.0 * u as f32,
            true,
            dome,
        );
        for dx in [-3 * u, 0, 3 * u] {
            self.rect(x0 + 26 * u + dx, b - 15 * u, u, 5 * u, scale(dome, 1.25));
        }
        self.rect(x0 + 26 * u, b - 17 * u, u, 2 * u, shade);
        self.rect(x0 + 26 * u, b - 20 * u, u, 3 * u, trim);
        self.rect(x0 + 25 * u, b - 19 * u, 3 * u, u, trim);
        self.rect(
            x0 + 28 * u,
            b - 7 * u,
            u,
            4 * u,
            if rng.f32() < lit { window } else { dark },
        );
        // The nave: its wall, the gable over it and a cornice between.
        self.rect(x0 + 7 * u, b - 10 * u, 18 * u, 10 * u, wall);
        self.gable(nx, (b - 10 * u, 7 * u), 9.0 * u as f32, trim);
        self.gable(nx, (b - 10 * u, 6 * u), 8.0 * u as f32, wall);
        self.rect(x0 + 6 * u, b - 11 * u, 20 * u, u, trim);
        self.rect(x0 + 7 * u, b - 10 * u, u, 10 * u, trim);
        self.rect(x0 + 24 * u, b - 10 * u, u, 10 * u, trim);
        // The cross over the gable, and the rose window under it.
        self.rect(nx, b - 21 * u, u, 3 * u, trim);
        self.rect(nx - u, b - 20 * u, 3 * u, u, trim);
        self.disc(
            (nx as f32 + 0.5 * u as f32, (b - 14 * u) as f32),
            2.0 * u as f32,
            false,
            trim,
        );
        self.disc(
            (nx as f32 + 0.5 * u as f32, (b - 14 * u) as f32),
            1.3 * u as f32,
            false,
            glow,
        );
        // The door, open on the candles inside, in a frame of stone.
        self.rect(nx - 3 * u, b - 7 * u, 7 * u, 7 * u, trim);
        self.disc(
            (nx as f32 + 0.5 * u as f32, (b - 7 * u) as f32),
            3.5 * u as f32,
            true,
            trim,
        );
        self.rect(nx - 2 * u, b - 6 * u, 5 * u, 6 * u, glow);
        self.disc(
            (nx as f32 + 0.5 * u as f32, (b - 6 * u) as f32),
            2.5 * u as f32,
            true,
            glow,
        );
        // Side windows, long and narrow, lit.
        for wx in [x0 + 10 * u, x0 + 21 * u] {
            self.rect(
                wx,
                b - 8 * u,
                u,
                4 * u,
                if rng.f32() < lit { window } else { dark },
            );
        }
        // The tower: its body, the belfry with the bell in it, the cap and
        // its cross.
        self.rect(x0, b - 16 * u, 7 * u, 16 * u, wall);
        self.rect(x0, b - 16 * u, u, 16 * u, trim);
        self.rect(x0 + 6 * u, b - 16 * u, u, 16 * u, shade);
        self.rect(x0 - u, b - 17 * u, 9 * u, u, trim);
        self.rect(x0 + 2 * u, b - 15 * u, 3 * u, 5 * u, scale(glow, 0.55));
        self.disc(
            ((x0 + 3 * u) as f32 + 0.5 * u as f32, (b - 15 * u) as f32),
            1.5 * u as f32,
            true,
            scale(glow, 0.55),
        );
        self.rect(x0 + 3 * u, b - 14 * u, u, 2 * u, gold);
        self.rect(x0 + 3 * u - u / 2, b - 12 * u, 2 * u, u, gold);
        self.rect(
            x0 + 3 * u,
            b - 8 * u,
            u,
            4 * u,
            if rng.f32() < lit { window } else { dark },
        );
        self.gable(x0 + 3 * u, (b - 17 * u, 5 * u), 4.0 * u as f32, roof);
        self.rect(x0 + 3 * u, b - 25 * u, u, 3 * u, trim);
        self.rect(x0 + 2 * u, b - 24 * u, 3 * u, u, trim);
        // The candles inside the door.
        for dx in [-u, 0, u] {
            let (x, y) = ((nx + dx) as usize, (b - 2 * u) as usize);
            if x < w && y < self.ph {
                self.town[y * w + x] = 2;
                self.candles.push(Candle {
                    x: x as i32,
                    y: y as i32,
                    phase: rng.f32() * TAU,
                    power: 0.9,
                });
            }
        }
    }

    /// Strings papel picado from `from` to `to` (the string runs left to
    /// right): the cord is part of the town, and the flags are an actor, for
    /// they flutter. `u` is the size of a block of them.
    fn hang_papel(&mut self, rng: &mut fastrand::Rng, from: (i32, i32), to: (i32, i32), u: usize) {
        let (from, to) = if from.0 <= to.0 {
            (from, to)
        } else {
            (to, from)
        };
        let a = Actor {
            role: Role::Papel,
            x: from.0 as f32,
            y: from.1 as f32,
            size: (to.0 - from.0) as f32,
            state: u.min(255) as u8,
            left: to.1,
            seed: rng.f32() * TAU,
        };
        let cord = self.fest.cable;
        for x in from.0..=to.0 {
            self.put(x, string_y(&a, x), cord);
        }
        self.actors.push(a);
    }

    /// A string of papel picado: a flag every few pixels, each in a color of
    /// the day with its cut-out, swinging a little in the air, the lower
    /// rows more.
    pub(super) fn draw_papel(&mut self, a: &Actor) {
        let k = (a.state as i32).max(1);
        let (x0, span) = (a.x.round() as i32, a.size.round() as i32);
        let t = self.frame as f32;
        let hues = self.fest.hues;
        let mut x = x0 + 2 * k;
        let mut i = a.seed as usize;
        while x < x0 + span - 3 * k {
            let y = string_y(a, x) + 1;
            let sway = (t * 0.11 + a.seed + i as f32 * 1.3).sin();
            let col = to_color(scale(hues[i % 6], 0.92));
            for (ry, row) in PICADO.iter().enumerate() {
                let dx = (sway * ry as f32 * 0.35 * k as f32).round() as i32;
                for (rx, ch) in row.bytes().enumerate() {
                    if ch != b'#' {
                        continue;
                    }
                    for (bx, by) in (0..k).flat_map(|i| (0..k).map(move |j| (i, j))) {
                        self.px
                            .set(x + rx as i32 * k + bx + dx - k, y + ry as i32 * k + by, col);
                    }
                }
            }
            x += 5 * k;
            i += 1;
        }
    }

    /// The procession walks to the church, leftwards across the atrium (state
    /// 0), stops at the door, where its place is in `seed` (1), goes in and is
    /// gone for a while (2), then comes again from the right.
    pub(super) fn walk_procesion(&mut self, a: &mut Actor) {
        match a.state {
            0 => {
                a.x -= 0.1 * a.size.sqrt();
                if a.x <= a.seed {
                    a.state = 1;
                    a.left = self.frng.i32(90..180);
                }
            }
            1 => {
                a.left -= 1;
                if a.left <= 0 {
                    a.state = 2;
                    a.left = self.frng.i32(150..300);
                }
            }
            _ => {
                a.left -= 1;
                if a.left <= 0 {
                    a.state = 0;
                    a.x = self.w as f32;
                }
            }
        }
    }

    /// The colors the procession is drawn in.
    fn ink(&self, ch: u8) -> Option<V3> {
        let ft = &self.fest;
        Some(match ch {
            b'h' | b'e' | b'l' => scale(ft.dark, 0.7),
            b's' if self.pal.native => c8(190, 130, 92),
            b's' => scale(ft.gold, 0.7),
            b'm' => scale(ft.white, 0.85),
            b'r' => ft.hues[5],
            b'w' => mix(ft.wood, ft.cane, 0.55),
            b'p' => ft.hues[0],
            b'y' => ft.hues[1],
            b'v' => ft.hues[2],
            b'o' => ft.hues[3],
            b'g' => ft.hues[4],
            b'H' => ft.gold,
            b'T' => scale(ft.gold, 0.6),
            _ => return None,
        })
    }

    /// The children in manta and rebozo, carrying huacales of flowers on
    /// their backs, and the mulita de hoja that follows them; they walk a
    /// step every eighth of a second.
    pub(super) fn draw_procesion(&mut self, a: &Actor) {
        if a.state == 2 {
            return;
        }
        let k = a.size.round().max(1.0) as i32;
        // They stand still when they are at the door.
        let step = if a.state == 0 {
            (self.frame / 8) as usize % 2
        } else {
            0
        };
        let (x, ground) = (a.x.round() as i32, a.y.round() as i32);
        for (frames, off) in PROCESION {
            let art = frames[step];
            let top = ground - art.len() as i32 * k;
            for (sy, row) in art.iter().enumerate() {
                for (sx, ch) in row.bytes().enumerate() {
                    let Some(c) = self.ink(ch) else { continue };
                    let col = to_color(scale(c, 0.9));
                    for (dx, dy) in (0..k).flat_map(|i| (0..k).map(move |j| (i, j))) {
                        self.px.set(
                            x + (off + sx as i32) * k + dx,
                            top + sy as i32 * k + dy,
                            col,
                        );
                    }
                }
            }
        }
    }
}
