//! Navidad in the sea: the Guadalupe–Reyes on the water of Xochimilco.
//!
//! What the holidays of the boats share is in `gala.rs` and
//! `trajinera.rs`: the trajineras drifting by, the night, the reflections.
//! Navidad hangs foquitos (series lights) blinking in chase patterns along
//! the arches and out to the poles, and, as the posadas climb, a piñata de
//! siete picos off each boat's bow; the buoys carry the Estrella de Belén;
//! the castle wears a string of foquitos between its towers; nochebuenas
//! grow on the sand; esferas dropped from the boats sink and lie on the
//! bottom among them; fish in rojo nochebuena and verde pino join the
//! schools and cross on their own, and sharks, orcas and dolphins wear a
//! gorro navideño (`art.rs`). On Año Nuevo fireworks go up (`cohetes.rs`).

use super::art::{self, Species};
use super::gala::Paint;
use super::sea::Sea;
use super::tank::mul;
use super::trajinera;
use crate::color::Color;

/// The women's names on the arches, to Xochimilco's taste: Lupita for the
/// Virgen de Guadalupe, Estrella for Belén's star, Gloria for the angels.
pub const NAMES: [&str; 10] = [
    "LUPITA",
    "ESPERANZA",
    "ESTRELLA",
    "NOELIA",
    "BELEN",
    "MARIA",
    "GLORIA",
    "ROSARIO",
    "PAZ",
    "ALEGRIA",
];

/// The fish of the holiday, for schools and for crossing alone.
pub fn fish(school: bool) -> &'static Species {
    if school {
        &art::NOCHEBUENITA
    } else {
        &art::NOCHEBUENA
    }
}

pub fn is_fish(sp: &Species) -> bool {
    [art::NOCHEBUENITA.name, art::NOCHEBUENA.name].contains(&sp.name)
}

/// The Estrella de Belén on a buoy's mast, `size` pixels tall and wide,
/// plotted through `put(dx, up, color, glow)` from its foot: gold, its tips
/// flashing white now and then.
pub fn star(p: &Paint, size: i32, t: f32, put: &mut dyn FnMut(i32, i32, Color, f32)) {
    const STAR7: [&str; 7] = [
        "...x...", "...x...", "..xxx..", "xxxoxxx", ".xxxxx.", "..x.x..", ".x...x.",
    ];
    const STAR5: [&str; 5] = ["..x..", "xxoxx", ".xxx.", ".x.x.", "x...x"];
    const STAR3: [&str; 3] = [".x.", "xox", "x.x"];
    let art: &[&str] = match size {
        7.. => &STAR7,
        5..=6 => &STAR5,
        _ => &STAR3,
    };
    let flash = ((t * 0.09).sin() * 2.0 - 1.0).clamp(0.0, 1.0);
    let rows = art.len() as i32;
    for (r, row) in art.iter().enumerate() {
        for (c, ch) in row.bytes().enumerate() {
            let color = match ch {
                b'.' => continue,
                b'o' => p.blanco,
                _ => p.gold.lerp(p.blanco, 0.7 * flash),
            };
            let dx = c as i32 - row.len() as i32 / 2;
            put(dx, rows - 1 - r as i32, color, 0.7 + 0.3 * flash);
        }
    }
}

impl Sea {
    /// A nochebuena: `x` is the middle of its stem and `y` the sand.
    pub fn plant_navidad(&mut self, x: i32, y: i32) {
        let p = self.pal.paint;
        let (r, g, gold) = (p.rojo, p.verde, p.gold);
        for (dx, dy, c) in [
            (-1, -3, r),
            (1, -3, r),
            (-2, -2, r),
            (-1, -2, mul(r, 0.85)),
            (0, -2, gold),
            (1, -2, mul(r, 0.85)),
            (2, -2, r),
            (-1, -1, g),
            (0, -1, r),
            (1, -1, g),
            (0, 0, mul(g, 0.8)),
        ] {
            self.put(x + dx, y + dy, c);
        }
    }

    /// An esfera with its cap, centered on (`x`, `y`).
    fn esfera(&mut self, x: i32, y: i32, color: u8, shine: bool) {
        let p = self.pal.paint;
        let c = [p.rojo, p.gold, p.plata, p.verde][color as usize % 4];
        self.put(x, y - 1, p.gold);
        self.put(
            x - 1,
            y,
            if shine {
                p.blanco
            } else {
                c.lerp(p.blanco, 0.35)
            },
        );
        self.put(x, y, c);
        self.put(x + 1, y, mul(c, 0.75));
        self.put(x, y + 1, mul(c, 0.6));
    }

    /// An esfera dropped from a boat: `hue` is rojo, dorado, plata or verde.
    pub fn sinker_navidad(&mut self, x: i32, y: i32, hue: u8, shine: bool) {
        self.esfera(x, y, hue, shine);
    }

    /// A string of foquitos between the castle's towers.
    pub fn dress_castle_navidad(&mut self, t: f32) {
        let Some((x0, y0)) = self.castle else { return };
        let p = self.pal.paint;
        for (i, (dx, dy)) in [(4, 4), (5, 5), (6, 6), (7, 6), (8, 6), (9, 5), (10, 4)]
            .into_iter()
            .enumerate()
        {
            let (x, y) = (x0 + dx, y0 + dy);
            if i == 0 || i == 6 {
                self.put(x, y, p.cord);
                continue;
            }
            let k = trajinera::blink(i as u8 % 3, i as i32, t);
            let c = p.bulbs[(i + 2) % 6];
            self.put(x, y, mul(c, 0.35 + 0.65 * k));
            if k > 0.9 {
                self.halo(x, y, c, 1.0);
            }
        }
    }
}
