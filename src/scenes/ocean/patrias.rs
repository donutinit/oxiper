//! The Fiestas Patrias in the sea: the mes patrio on the water of Xochimilco.
//!
//! What the two holidays share is in `gala.rs` and `trajinera.rs`: the
//! trajineras drifting by, the night, the reflections. For the Patrias the
//! boats fly the flag from a pole at each end, string papel picado in verde,
//! blanco y rojo between the poles and the arch, and wear tricolor lights
//! along it; the buoys carry a flag on their mast; the castle flies one from
//! its tower and strings papel picado between its towers; nopales with their
//! red tunas grow on the sand and pieces of papel picado dropped from the
//! boats sink and settle there; and fish in verde, blanco y rojo join the
//! schools and cross on their own (`art.rs`). At night tricolor fireworks
//! burst over the water and show in it (`cohetes.rs`), more of them as the
//! month climbs to the Grito.

use super::art::{self, Species};
use super::gala::Paint;
use super::sea::Sea;
use super::tank::mul;
use crate::color::Color;

/// The women's names on the arches: the heroínas of the Independencia,
/// Josefa Ortiz and Leona Vicario among them, and the names the patria
/// goes by.
pub const NAMES: [&str; 10] = [
    "JOSEFA", "LEONA", "DOLORES", "VICTORIA", "LIBERTAD", "PATRIA", "SOLEDAD", "MARIANA", "CARMEN",
    "ROSA",
];

/// The fish of the holiday, for schools and for crossing alone.
pub fn fish(school: bool) -> &'static Species {
    if school {
        &art::BANDERITA
    } else {
        &art::BANDERA
    }
}

pub fn is_fish(sp: &Species) -> bool {
    [art::BANDERITA.name, art::BANDERA.name].contains(&sp.name)
}

/// A flag of `w` by `h` pixels hoisted at column 0 of `put(dx, up, color,
/// glow)` and flying toward `out` (1 for right, -1 for left), `up` counting
/// from its bottom row: verde, blanco, rojo, with the águila a brown dot in
/// the middle of the white, rippling in the wind.
pub fn flag(
    p: &Paint,
    (w, h): (i32, i32),
    out: i32,
    (t, phase): (f32, f32),
    put: &mut dyn FnMut(i32, i32, Color, f32),
) {
    for c in 0..w {
        let band = (c * 3 / w) as usize;
        let amp = 0.3 + 0.8 * c as f32 / w as f32;
        let wave = ((t * 0.22 + phase - c as f32 * 0.9).sin() * amp).round() as i32;
        for r in 0..h {
            let color = if band == 1 && c == w / 2 && r == h / 2 && h >= 3 {
                p.cafe
            } else {
                p.bandera[band]
            };
            put(out * c, r + wave, color, 0.0);
        }
    }
}

/// The flag on a buoy's mast, `size` pixels wide: a pole to its top with a
/// dorado ball, and the flag flying from it.
pub fn topper(p: &Paint, size: i32, t: f32, put: &mut dyn FnMut(i32, i32, Color, f32)) {
    let h = (size * 4 / 7).max(2);
    for u in 0..=h {
        put(0, u, p.wood, 0.0);
    }
    put(0, h + 1, p.gold, 0.0);
    flag(p, (size, h), 1, (t, 0.0), &mut |dx, u, c, g| {
        put(dx + 1, u + 1, c, g)
    });
}

impl Sea {
    /// A nopal with its tunas, `x` the middle of its stem and `y` the sand.
    pub fn plant_patrias(&mut self, x: i32, y: i32) {
        let p = self.pal.paint;
        for (dx, dy, c) in [
            (-1, -5, p.bandera[2]),
            (1, -5, p.bandera[2]),
            (-1, -4, p.nopal),
            (0, -4, mul(p.nopal, 1.15)),
            (1, -4, p.nopal),
            (-1, -3, mul(p.nopal, 0.85)),
            (0, -3, p.nopal),
            (1, -3, mul(p.nopal, 0.85)),
            (0, -2, mul(p.nopal, 0.8)),
            (0, -1, mul(p.nopal, 0.8)),
            (-1, -1, p.nopal),
        ] {
            self.put(x + dx, y + dy, c);
        }
    }

    /// A piece of papel picado dropped from a boat, `hue` its color in the
    /// flag's order.
    pub fn sinker_patrias(&mut self, x: i32, y: i32, hue: u8, shine: bool) {
        let p = self.pal.paint;
        let c = p.bandera[hue as usize % 3];
        let c = if shine { c.lerp(p.blanco, 0.6) } else { c };
        self.put(x, y, c);
        self.put(x + 1, y, mul(c, 0.85));
        self.put(x, y + 1, mul(c, 0.7));
    }

    /// The flag on the castle's left tower and papel picado on a twine
    /// between the towers.
    pub fn dress_castle_patrias(&mut self, t: f32) {
        let Some((x0, y0)) = self.castle else { return };
        let p = self.pal.paint;
        for u in 1..=5 {
            self.put(x0 + 2, y0 - u, p.wood);
        }
        self.put(x0 + 2, y0 - 6, p.gold);
        flag(&p, (4, 3), 1, (t, 1.0), &mut |dx, u, c, _| {
            self.put(x0 + 3 + dx, y0 - 3 - u, c)
        });
        for x in 4..=10 {
            let sag = if (6..=8).contains(&x) { 2 } else { 1 };
            self.put(x0 + x, y0 + 3 + sag, p.cord);
            if x % 2 == 1 {
                let c = p.bandera[(x as usize / 2) % 3];
                self.put(x0 + x, y0 + 4 + sag, c);
                self.put(x0 + x, y0 + 5 + sag, mul(c, 0.85));
            }
        }
    }
}
