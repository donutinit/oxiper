//! The Fiestas Patrias at sea (the season's own fish, plants, props and
//! passers-by are in `assets/patrias.rs`): the castle flies the Mexican flag,
//! and strings of banderitas in verde, blanco y rojo hang between the kelp,
//! two of them at the height of the mes patrio, the Grito.

use super::biome::hash;
use super::library::library;
use super::*;

/// The castle's flag: how wide it is, in cells.
const FLAG_W: i32 = 9;

impl Aquarium {
    /// The pen of stripe `k` (0 verde, 1 blanco, 2 rojo) of the flag.
    fn stripe(&self, k: i32) -> u8 {
        [G, W + BRIGHT, R][k.clamp(0, 2) as usize]
    }

    /// The Mexican flag on the castle's tower, rippling in the current: green,
    /// white with the águila in its middle, red. It takes the pennant's place
    /// at the top of the pole, which grows if there is room above.
    pub(super) fn draw_castle_flag(&self, c: &mut Canvas) {
        let lib = library();
        let top = self.water_top();
        let beat = (self.time * 3.0) as i64;
        for e in &self.ents {
            let castle = e.kind == Kind::Decor
                && !e.dead
                && lib
                    .decor
                    .get(e.who as usize)
                    .is_some_and(|d| d.name == "castle");
            if !castle {
                continue;
            }
            let (x, y) = (e.x.floor() as i32 + 16, e.y.floor() as i32);
            // Three rows above the tower's top if the water allows, else the
            // two rows beside its pole.
            let (y0, rows) = if y - 3 >= top { (y - 2, 3) } else { (y, 2) };
            for r in 0..rows {
                for k in 0..FLAG_W {
                    let stripe = k / 3;
                    let (ch, pen) = if stripe == 1 {
                        if r == rows / 2 && k == 4 {
                            ('@', Y)
                        } else {
                            (':', self.stripe(1))
                        }
                    } else if (k as i64 + r as i64 + beat).rem_euclid(3) == 0 {
                        ('%', self.stripe(stripe))
                    } else {
                        ('#', self.stripe(stripe))
                    };
                    let (col, attr) = self.pens[pen as usize];
                    c.set_attr(x + k, y0 + r, ch, col, attr);
                }
            }
            if rows == 3 {
                // The pole, up to its finial.
                let (col, attr) = self.pens[(W + BRIGHT) as usize];
                for dy in 0..2 {
                    c.set_attr(x - 1, y0 + dy, '|', col, attr);
                }
                let (col, attr) = self.pens[(Y + BRIGHT) as usize];
                c.set_attr(x - 1, y0 - 1, 'o', col, attr);
            }
        }
    }

    /// Banderitas hung between the plants in the background: a little cord
    /// with a pennant every other cell, verde, blanco y rojo in turn,
    /// fluttering. As the month goes more plants hold them; at the Grito
    /// there are two strings.
    pub(super) fn draw_banderitas(&self, c: &mut Canvas, seed: u64, level: f32) {
        let beat = (self.time * 2.0) as i64;
        self.garlands(seed, level, |g| {
            let (ch, pen) = if g.k % 2 == 1 {
                let n = (g.k / 2) as i64 + g.id as i64;
                let flutter = (n + beat + hash(g.id as u64) as i64).rem_euclid(2) == 0;
                (
                    if flutter { 'v' } else { 'V' },
                    self.stripe(n.rem_euclid(3) as i32),
                )
            } else {
                (['/', '-', '\\'][(g.slope + 1) as usize], W)
            };
            let (col, attr) = self.pens[pen as usize];
            c.set_attr(g.x, g.y, ch, col, attr);
        });
    }
}
