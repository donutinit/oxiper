//! What the holidays of the jungle share: hue matching for the colors of a
//! theme, which holiday builds what (the ruin to call on, the flowers, the
//! saucer's room), and what a frame painted, for the tests. Each holiday
//! keeps the rest to itself: `muertos.rs`, `navidad.rs`.

use super::{Pal, Rgb};

/// The distance between two hues, in degrees.
pub fn hue_gap(a: f32, b: f32) -> f32 {
    let d = (a - b).abs() % 360.0;
    d.min(360.0 - d)
}

/// Hue (degrees), saturation and value of a color.
pub fn hsv(c: Rgb) -> (f32, f32, f32) {
    let (r, g, b) = (c.0 as f32 / 255.0, c.1 as f32 / 255.0, c.2 as f32 / 255.0);
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    let h = if d <= 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    (h, if max <= 0.0 { 0.0 } else { d / max }, max)
}

/// The theme's colorful accent whose hue is closest to `hue`, within `tol`
/// degrees, and not one of `avoid`.
pub fn by_hue(cols: &[Rgb], hue: f32, tol: f32, avoid: &[Rgb]) -> Option<Rgb> {
    cols.iter()
        .copied()
        .filter_map(|c| {
            let (h, s, v) = hsv(c);
            let gap = hue_gap(h, hue);
            (s >= 0.25 && v >= 0.3 && gap <= tol && !avoid.contains(&c)).then_some((gap, c))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

/// A place a holiday builds by the path: a ruin the tiger can call on.
#[derive(Clone, Copy, Debug)]
pub struct Site {
    /// How far the holiday has come, 0 to 1.
    pub level: f32,
    /// Half the width of what stands there.
    pub half: f32,
    /// The chance that a cell of the jungle has one, and how much more of it
    /// there is as the holiday builds up.
    pub chance: (f32, f32),
    /// How far from its middle the tiger stands to call on it, jaws first.
    pub stand: f32,
}

/// How the saucer is framed: how many rows the camera pulls back to, how
/// much taller than the ordinary saucer it stands, and how far it keeps clear
/// of what is strung across the top.
#[derive(Clone, Copy, Debug)]
pub struct Room {
    pub wide: f32,
    pub tall: f32,
    pub clear: f32,
}

impl Pal {
    /// The ruin the holiday builds, if it builds one.
    pub fn site(&self) -> Option<Site> {
        let level = self
            .fest
            .map(|d| d.level)
            .or(self.navidad.map(|d| d.level))?;
        Some(Site {
            level,
            half: 28.0,
            chance: (0.46, 0.36),
            stand: 12.0,
        })
    }

    /// How much of the holiday there is in flower, if its flowers grow wild.
    pub fn bloom_level(&self) -> Option<f32> {
        self.fest.map(|d| d.level).or(self.navidad.map(|d| d.level))
    }

    /// The room the saucer needs; `ordinary` rows is all it takes on any
    /// other day.
    pub fn room(&self, ordinary: f32) -> Room {
        let (wide, tall, clear) = if self.fest.is_some() {
            (96.0, super::muertos::SAUCER_EXTRA, 14.0)
        } else if self.navidad.is_some() {
            (92.0, super::navidad::COMET_EXTRA, 6.0)
        } else {
            (ordinary, 0.0, 0.0)
        };
        Room { wide, tall, clear }
    }

    /// The light the saucer washes over the jungle: what tints it, and the
    /// sky above.
    pub fn eerie(&self) -> (Rgb, Rgb) {
        if self.fest.is_some() {
            // Amber and plum over the marigolds, instead of the saucer's green.
            (Rgb(255, 206, 140), Rgb(44, 20, 40))
        } else if self.navidad.is_some() {
            // The star's gold, over a deep blue night.
            (Rgb(255, 228, 160), Rgb(24, 26, 56))
        } else {
            (Rgb(170, 255, 200), Rgb(20, 40, 36))
        }
    }

    /// Is a holiday dressing the jungle in a way that sends the tiger to a
    /// ruin?
    pub fn visits(&self) -> bool {
        self.site().is_some()
    }
}

/// What a frame painted of the holidays, for the tests.
#[cfg(test)]
#[derive(Default, Debug)]
pub struct Painted {
    pub flags: u32,
    pub blossoms: u32,
    pub ruins: u32,
    pub candles: u32,
    pub petals: u32,
    /// The tiger was painted as an alebrije, and the saucer as a skull.
    pub alebrije: bool,
    pub saucer: bool,
    /// Navidad: bulbs of the strings of lights, tufts of heno, layers of the
    /// nacimientos, the Niño Dios in a manger, the kings of the caravan, and
    /// the saucer painted as the Estrella de Belén.
    pub bulbs: u32,
    pub heno: u32,
    pub belenes: u32,
    pub ninos: u32,
    pub reyes: u32,
    pub comet: bool,
}
