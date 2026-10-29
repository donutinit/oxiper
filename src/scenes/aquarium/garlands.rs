//! Strings hung from plant to plant, which the season dresses: the series'
//! lights at Navidad. Each string sags between a plant and the nearest one on
//! its right, from a point a little below the top of each; how many plants
//! hang one follows how far the holiday has come, and at its height there are
//! two strings between them.

use super::biome::hash;
use super::*;

/// One cell of a string.
pub(super) struct Hung {
    pub x: i32,
    pub y: i32,
    /// How far along the string it is (0 at the first plant).
    pub k: i32,
    /// The plant the string hangs from, to tell strings apart.
    pub id: u32,
    /// How much the string has dropped since the cell before it: -1, 0 or 1.
    pub slope: i32,
}

impl Aquarium {
    /// Where a plant holds its string: its middle, below its top.
    fn hook(&self, e: &Ent, seed: u64, row: u8) -> (i32, i32) {
        let h = e.h.max(1.0) as i32;
        let r = hash(seed ^ (e.id as u64) << 12 ^ 0x4f0c);
        let down = (h as f32 * (0.15 + 0.3 * (r & 255) as f32 / 255.0)) as i32 + 2 * row as i32;
        (
            e.x.floor() as i32 + e.w as i32 / 2,
            e.y.floor() as i32 + down.min(h - 1),
        )
    }

    /// Every cell of every string, to `put`.
    pub(super) fn garlands(&self, seed: u64, level: f32, mut put: impl FnMut(Hung)) {
        let top = self.water_top();
        let plant = |e: &&Ent| {
            e.kind == Kind::Seaweed && !e.dead && e.layer == 0 && e.h >= 3.0 && e.who != NOBODY
        };
        let level = level.clamp(0.0, 1.0);
        let share = (0.3 + 0.6 * level) * 1000.0;
        let rows = 1 + u8::from(level >= 0.85);
        for a in self.ents.iter().filter(plant) {
            if (hash(seed ^ (a.id as u64) << 8 ^ 0x57a1) % 1000) as f32 >= share {
                continue;
            }
            let ax = self.hook(a, seed, 0).0;
            // The nearest plant on its right.
            let mut next: Option<(&Ent, i32)> = None;
            for b in self.ents.iter().filter(plant) {
                let d = self.hook(b, seed, 0).0 - ax;
                if (4..=30).contains(&d) && next.is_none_or(|(_, n)| d < n) {
                    next = Some((b, d));
                }
            }
            let Some((b, dx)) = next else {
                continue;
            };
            for row in 0..rows {
                let (_, ay) = self.hook(a, seed, row);
                let (_, by) = self.hook(b, seed, row);
                if (by - ay).abs() > dx / 2 {
                    continue;
                }
                let sag = (dx as f32 / 6.0).clamp(1.0, 3.0) + 0.5 * row as f32;
                let mut last = ay;
                for k in 0..=dx {
                    let t = k as f32 / dx as f32;
                    let y = (ay as f32 + (by - ay) as f32 * t + sag * 4.0 * t * (1.0 - t)).round()
                        as i32;
                    if y >= top && y < self.h {
                        put(Hung {
                            x: ax + k,
                            y,
                            k,
                            id: a.id.wrapping_add(row as u32 * 7919),
                            slope: (y - last).clamp(-1, 1),
                        });
                    }
                    last = y;
                }
            }
        }
    }
}
