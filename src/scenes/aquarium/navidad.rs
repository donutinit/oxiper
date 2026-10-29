//! The Navidad at sea (the season's own fish, plants, props and passers-by
//! are in `assets/navidad.rs`): kelp hung with the series' blinking lights,
//! esferas rising like bubbles, the Niño Dios laid in the nacimiento's
//! manger from Nochebuena, and, at night during the posadas, a procession of
//! small fish carrying candles.

use super::biome::hash;
use super::library::library;
use super::*;

/// The bulbs of the series (los foquitos): rojo, verde, azul, amarillo, rosa
/// and naranja (a dim yellow).
const FOCOS: [u8; 6] = [
    R + BRIGHT,
    G + BRIGHT,
    B + BRIGHT,
    Y + BRIGHT,
    M + BRIGHT,
    Y,
];
/// The esferas: rojo nochebuena, verde pino, dorado and plata.
const ESFERAS: [u8; 4] = [R + BRIGHT, G, Y + BRIGHT, W];

/// Where the Niño Dios lies in a nacimiento, from its top left corner.
pub(super) fn manger(name: &str) -> Option<(i32, i32)> {
    match name {
        "nacimiento" => Some((32, 8)),
        "nacimientito" => Some((17, 5)),
        _ => None,
    }
}

impl Aquarium {
    /// A bulb of the series: lit, glowing or off, the light chasing along
    /// the string as the bulbs' numbers climb.
    fn bulb(&self, n: i64) -> char {
        match (n + (self.time * 3.0) as i64).rem_euclid(3) {
            0 => 'O',
            1 => 'o',
            _ => '.',
        }
    }

    /// Bulbs on a plant, drawn right after it so whatever swims in front
    /// of the plant covers them: some of its rows hold one, blinking in
    /// chase, more of them as the holiday comes.
    pub(super) fn dress_plant(&self, e: &Ent, c: &mut Canvas) {
        let Some(w) = self.world.as_ref() else {
            return;
        };
        let lib = library();
        let Some(s) = sprites().get(e.spr as usize) else {
            return;
        };
        if e.layer > 0
            || s.h < 3
            || lib
                .plants
                .get(e.who as usize)
                .is_none_or(|p| p.season != assets::ALWAYS)
        {
            return;
        }
        let (x0, y0) = (e.x.floor() as i32, e.y.floor() as i32);
        let share = (0.22 + 0.3 * w.level.clamp(0.0, 1.0)) * 1000.0;
        let mut done = -1;
        for cell in s.frame(e.frame as usize) {
            let row = cell.dy as i32;
            if cell.ch == ' ' || row == done {
                continue;
            }
            // One look per row, at its first cell.
            done = row;
            let r = hash(w.seed ^ (e.id as u64) << 8 ^ row as u64 ^ 0xb01b);
            if row == 0 || (r % 1000) as f32 >= share {
                continue;
            }
            let pen = FOCOS[(r >> 16) as usize % FOCOS.len()];
            let (col, attr) = self.glow(pen);
            c.set_attr(
                x0 + cell.dx as i32,
                y0 + row,
                self.bulb((r >> 24 & 7) as i64),
                col,
                attr,
            );
        }
    }

    /// The series hung between plants, in the background: a green cord
    /// with a bulb every third cell, the light chasing along it.
    pub(super) fn draw_lights(&self, c: &mut Canvas, seed: u64, level: f32) {
        self.garlands(seed, level, |g| {
            let (ch, pen) = if g.k % 3 == 1 {
                let n = (g.k / 3) as i64;
                (
                    self.bulb(n),
                    FOCOS[(n + g.id as i64).rem_euclid(FOCOS.len() as i64) as usize],
                )
            } else {
                (['/', '-', '\\'][(g.slope + 1) as usize], G)
            };
            let (col, attr) = self.glow(pen);
            c.set_attr(g.x, g.y, ch, col, attr);
        });
    }

    /// Esferas rising like bubbles from the floor to the waves, swaying a
    /// little and carried by the current; a few big ones among them, with
    /// their caps on. They pass behind the fish.
    pub(super) fn draw_esferas(&self, c: &mut Canvas, seed: u64, drift: f32, level: f32) {
        let (w, h) = (self.w, self.h);
        let top = self.water_top();
        let rows = h - top;
        if w < 8 || rows < 4 {
            return;
        }
        let n = ((w * rows) as f32 / 480.0 * (0.4 + 0.8 * level))
            .ceil()
            .clamp(1.0, 80.0) as u64;
        let span = (rows + 2) as f32;
        for i in 0..n {
            let r = hash(seed ^ 0x035f_e7a5 ^ i.wrapping_mul(0x9e37));
            let speed = 1.0 + (r >> 8 & 63) as f32 / 63.0 * 1.4;
            let climbed = self.time * speed + (r >> 20 & 1023) as f32 / 1023.0 * span;
            // Each time one reaches the waves another starts from the floor.
            let cycle = (climbed / span).floor() as i64 as u64;
            let y = (h as f32 - 1.0 - climbed.rem_euclid(span)) as i32;
            if y < top + 1 {
                continue;
            }
            let q = hash(r ^ cycle.wrapping_mul(0x51ed));
            let x = (q % w as u64) as f32 + (self.time * 0.7 + i as f32).sin() * 1.2 + drift * 0.7;
            let x = x.rem_euclid(w as f32) as i32;
            let (col, attr) = self.glow(ESFERAS[(q >> 16) as usize % ESFERAS.len()]);
            let (cap, cap_attr) = self.glow(W + BRIGHT);
            if q >> 40 & 7 == 0 && rows >= 12 {
                c.set_attr(x + 1, y - 1, '^', cap, cap_attr);
                c.text_attr(x, y, "(@)", col, attr);
            } else {
                c.set_attr(x, y - 1, '\'', cap, cap_attr);
                c.set_attr(x, y, '@', col, attr);
            }
        }
    }

    /// From Nochebuena the Niño Dios lies in every manger, glowing.
    pub(super) fn draw_nino(&self, c: &mut Canvas) {
        let lib = library();
        let (col, attr) = self.glow(W + BRIGHT);
        let (halo, halo_attr) = self.glow(Y + BRIGHT);
        let beat = (self.time * 1.5) as i64 % 2 == 0;
        for e in &self.ents {
            if e.kind != Kind::Decor || e.dead || e.who == NOBODY {
                continue;
            }
            let Some((dx, dy)) = lib.decor.get(e.who as usize).and_then(|d| manger(d.name)) else {
                continue;
            };
            let (x, y) = (e.x.floor() as i32 + dx, e.y.floor() as i32 + dy);
            c.set_attr(x, y, 'o', col, attr);
            c.set_attr(x, y - 1, if beat { '*' } else { '+' }, halo, halo_attr);
        }
    }

    /// During the posadas, at night: a procession of small fish carrying
    /// candles swims across, led by the one with the star, as the peregrinos
    /// walk from house to house. More of them as the posadas go.
    pub(super) fn draw_procession(&self, c: &mut Canvas, seed: u64, level: f32) {
        let (w, h) = (self.w, self.h);
        let top = self.water_top();
        let rows = h - top;
        if w < 24 || rows < 8 {
            return;
        }
        let n = (3 + (level.clamp(0.0, 1.0) * 6.0) as i32).clamp(2, (w / 8).max(2));
        let gap = 7;
        let lap = (w + n * gap + 12) as f32;
        let right = hash(seed ^ 0x9051) & 1 == 0;
        let head = (self.time * 2.0).rem_euclid(lap) as i32 - 6;
        let sea = top + (rows as f32 * 0.5) as i32;
        for k in 0..n {
            let behind = head - k * gap;
            let x = if right { behind } else { w - 4 - behind };
            let y = sea + (1.6 * (behind as f32 * 0.11 + self.time * 0.4).sin()).round() as i32;
            // The fish, its candle and the flame, flickering.
            let (fish, pen) = (
                if right { "<(o>" } else { "<o)>" },
                [C, W, B + BRIGHT][k as usize % 3],
            );
            let (col, attr) = self.pens[pen as usize];
            c.text_attr(x, y, fish, col, attr);
            let (col, attr) = self.glow(W + BRIGHT);
            c.set_attr(x + 2 - (!right) as i32, y - 1, 'i', col, attr);
            let flick =
                (self.time * 6.0 + k as f32 * 2.3).sin() + (self.time * 15.0 + k as f32).sin();
            let flame = if k == 0 {
                '*'
            } else if flick > 0.8 {
                '('
            } else if flick < -0.8 {
                ')'
            } else {
                '\''
            };
            let (col, attr) = self.glow(if flick > -0.2 { Y + BRIGHT } else { R + BRIGHT });
            c.set_attr(x + 2 - (!right) as i32, y - 2, flame, col, attr);
        }
    }
}
