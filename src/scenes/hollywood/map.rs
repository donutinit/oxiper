//! A crude world map: continents as hand-placed polygons (longitude, latitude),
//! rasterized into braille dots once per resize, plus a list of cities for
//! traces and satellites to point at.

use super::Rect;
use crate::color::Color;
use crate::render::Canvas;

/// Braille dots (2x4 per cell, one color per cell) that blit into a pane
/// rectangle; `render::Braille` can only blit at the canvas origin.
pub struct Dots {
    w: usize,
    h: usize,
    bits: Vec<u8>,
    col: Vec<Color>,
}

const BITS: [[u8; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

impl Dots {
    pub fn new(cols: u16, rows: u16) -> Dots {
        let n = cols as usize * rows as usize;
        Dots {
            w: cols as usize,
            h: rows as usize,
            bits: vec![0; n],
            col: vec![Color::Reset; n],
        }
    }

    pub fn pw(&self) -> i32 {
        self.w as i32 * 2
    }

    pub fn ph(&self) -> i32 {
        self.h as i32 * 4
    }

    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, c: Color) {
        if x < 0 || y < 0 || x >= self.pw() || y >= self.ph() {
            return;
        }
        let i = (y as usize / 4) * self.w + x as usize / 2;
        self.bits[i] |= BITS[y as usize & 3][x as usize & 1];
        self.col[i] = c;
    }

    /// Bresenham line in dot coordinates.
    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, c: Color) {
        let (mut x, mut y) = (x0.round() as i32, y0.round() as i32);
        let (x1, y1) = (x1.round() as i32, y1.round() as i32);
        let (dx, dy) = ((x1 - x).abs(), -(y1 - y).abs());
        let (sx, sy) = (if x < x1 { 1 } else { -1 }, if y < y1 { 1 } else { -1 });
        let mut err = dx + dy;
        for _ in 0..4096 {
            self.set(x, y, c);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draws every non-empty cell inside `r`.
    pub fn blit(&self, c: &mut Canvas, r: Rect) {
        for row in 0..self.h.min(r.h.max(0) as usize) {
            for col in 0..self.w.min(r.w.max(0) as usize) {
                let i = row * self.w + col;
                if self.bits[i] != 0 {
                    let ch = char::from_u32(0x2800 + self.bits[i] as u32).unwrap_or(' ');
                    c.set(r.x + col as i32, r.y + row as i32, ch, self.col[i]);
                }
            }
        }
    }
}

/// Rough continent outlines as (lon, lat) pairs.
const LAND: &[&[(f32, f32)]] = &[
    // North America
    &[
        (-165.0, 60.0),
        (-165.0, 68.0),
        (-140.0, 70.0),
        (-120.0, 72.0),
        (-95.0, 73.0),
        (-80.0, 68.0),
        (-62.0, 58.0),
        (-55.0, 50.0),
        (-67.0, 44.0),
        (-75.0, 38.0),
        (-81.0, 31.0),
        (-80.0, 25.0),
        (-90.0, 29.0),
        (-97.0, 26.0),
        (-97.0, 18.0),
        (-88.0, 16.0),
        (-83.0, 9.0),
        (-78.0, 8.0),
        (-86.0, 12.0),
        (-92.0, 15.0),
        (-105.0, 20.0),
        (-110.0, 24.0),
        (-117.0, 32.0),
        (-124.0, 40.0),
        (-125.0, 49.0),
        (-135.0, 58.0),
    ],
    // Greenland
    &[
        (-55.0, 60.0),
        (-45.0, 60.0),
        (-20.0, 70.0),
        (-20.0, 82.0),
        (-60.0, 82.0),
        (-72.0, 78.0),
        (-55.0, 70.0),
    ],
    // South America
    &[
        (-78.0, 8.0),
        (-62.0, 10.0),
        (-50.0, 0.0),
        (-35.0, -6.0),
        (-39.0, -15.0),
        (-48.0, -26.0),
        (-58.0, -38.0),
        (-66.0, -46.0),
        (-70.0, -55.0),
        (-75.0, -48.0),
        (-72.0, -30.0),
        (-70.0, -18.0),
        (-80.0, -5.0),
    ],
    // Eurasia
    &[
        (-10.0, 37.0),
        (-9.0, 43.0),
        (0.0, 50.0),
        (5.0, 53.0),
        (8.0, 57.0),
        (12.0, 55.0),
        (20.0, 55.0),
        (24.0, 60.0),
        (30.0, 70.0),
        (45.0, 68.0),
        (70.0, 73.0),
        (100.0, 77.0),
        (140.0, 72.0),
        (170.0, 70.0),
        (180.0, 66.0),
        (160.0, 58.0),
        (142.0, 52.0),
        (140.0, 45.0),
        (130.0, 42.0),
        (122.0, 40.0),
        (122.0, 30.0),
        (110.0, 20.0),
        (106.0, 10.0),
        (100.0, 2.0),
        (98.0, 15.0),
        (92.0, 22.0),
        (80.0, 15.0),
        (77.0, 8.0),
        (72.0, 20.0),
        (60.0, 25.0),
        (57.0, 25.0),
        (50.0, 30.0),
        (44.0, 15.0),
        (35.0, 28.0),
        (35.0, 36.0),
        (28.0, 36.0),
        (26.0, 40.0),
        (20.0, 40.0),
        (12.0, 44.0),
        (8.0, 44.0),
        (3.0, 43.0),
        (-2.0, 37.0),
    ],
    // Africa
    &[
        (-17.0, 21.0),
        (-10.0, 35.0),
        (10.0, 37.0),
        (32.0, 31.0),
        (43.0, 12.0),
        (51.0, 12.0),
        (40.0, -5.0),
        (40.0, -15.0),
        (33.0, -26.0),
        (20.0, -35.0),
        (15.0, -28.0),
        (12.0, -15.0),
        (8.0, 0.0),
        (-8.0, 4.0),
        (-17.0, 14.0),
    ],
    // Australia
    &[
        (113.0, -22.0),
        (122.0, -17.0),
        (131.0, -12.0),
        (142.0, -11.0),
        (146.0, -19.0),
        (153.0, -26.0),
        (150.0, -37.0),
        (140.0, -38.0),
        (130.0, -32.0),
        (115.0, -34.0),
    ],
    // Great Britain
    &[
        (-6.0, 50.0),
        (1.0, 51.0),
        (2.0, 53.0),
        (-2.0, 58.0),
        (-6.0, 57.0),
        (-4.0, 54.0),
    ],
    // Japan
    &[
        (130.0, 31.0),
        (135.0, 34.0),
        (141.0, 38.0),
        (142.0, 44.0),
        (140.0, 41.0),
        (136.0, 36.0),
    ],
];

/// Named places: (name, lat, lon).
pub const CITIES: &[(&str, f32, f32)] = &[
    ("LONDON", 51.5, -0.1),
    ("TOKYO", 35.7, 139.7),
    ("NEW YORK", 40.7, -74.0),
    ("SAN FRANCISCO", 37.8, -122.4),
    ("MOSCOW", 55.8, 37.6),
    ("BEIJING", 39.9, 116.4),
    ("SYDNEY", -33.9, 151.2),
    ("SAO PAULO", -23.5, -46.6),
    ("CAIRO", 30.0, 31.2),
    ("MUMBAI", 19.1, 72.9),
    ("LAGOS", 6.5, 3.4),
    ("BERLIN", 52.5, 13.4),
    ("SINGAPORE", 1.3, 103.8),
    ("MEXICO CITY", 19.4, -99.1),
    ("REYKJAVIK", 64.1, -21.9),
    ("CAPE TOWN", -33.9, 18.4),
    ("ANCHORAGE", 61.2, -149.9),
    ("SEOUL", 37.6, 127.0),
];

/// Día de Muertos: where the souls come from (the diaspora and the world):
/// (name, lat, lon).
pub const ORIGINS: &[(&str, f32, f32)] = &[
    ("LOS ANGELES", 34.1, -118.2),
    ("CHICAGO", 41.9, -87.6),
    ("HOUSTON", 29.8, -95.4),
    ("NUEVA YORK", 40.7, -74.0),
    ("MADRID", 40.4, -3.7),
    ("PARIS", 48.9, 2.4),
    ("BUENOS AIRES", -34.6, -58.4),
    ("BOGOTA", 4.7, -74.1),
    ("LA HABANA", 23.1, -82.4),
    ("TORONTO", 43.7, -79.4),
    ("SAN ANTONIO", 29.4, -98.5),
    ("PHOENIX", 33.4, -112.1),
    ("TOKIO", 35.7, 139.7),
    ("SYDNEY", -33.9, 151.2),
    ("BERLIN", 52.5, 13.4),
    ("LIMA", -12.0, -77.0),
];

/// ...and the towns they come home to.
pub const HOMES: &[(&str, f32, f32)] = &[
    ("OAXACA", 17.1, -96.7),
    ("PATZCUARO", 19.5, -101.6),
    ("MORELIA", 19.7, -101.2),
    ("MERIDA", 21.0, -89.6),
    ("PUEBLA", 19.0, -98.2),
    ("GUADALAJARA", 20.7, -103.3),
    ("MIXQUIC", 19.2, -99.0),
    ("CIUDAD DE MEXICO", 19.4, -99.1),
    ("JANITZIO", 19.6, -101.6),
    ("ZACATECAS", 22.8, -102.6),
    ("TAXCO", 18.6, -99.6),
    ("CHIAPAS", 16.7, -93.1),
];

/// Fiestas Patrias: where the echoes of the Grito go, the towns of the
/// Independence: (name, lat, lon).
pub const SITES_PATRIAS: &[(&str, f32, f32)] = &[
    ("DOLORES HIDALGO", 21.2, -100.9),
    ("CIUDAD DE MEXICO", 19.4, -99.1),
    ("QUERETARO", 20.6, -100.4),
    ("SAN MIGUEL DE ALLENDE", 20.9, -100.7),
    ("GUANAJUATO", 21.0, -101.3),
    ("CHILPANCINGO", 17.6, -99.5),
    ("IGUALA", 18.3, -99.5),
    ("APATZINGAN", 19.1, -102.4),
    ("CUAUTLA", 18.8, -98.9),
    ("VALLADOLID", 19.7, -101.2),
    ("ACAMBARO", 20.0, -100.7),
    ("MONTE DE LAS CRUCES", 19.3, -99.4),
];

fn inside(poly: &[(f32, f32)], x: f32, y: f32) -> bool {
    let mut c = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        if (yi > y) != (yj > y) && x < (xj - xi) * (y - yi) / (yj - yi) + xi {
            c = !c;
        }
        j = i;
    }
    c
}

/// Longitude and latitude to dot coordinates of a `pw` x `ph` plot.
#[inline]
pub fn project(lat: f32, lon: f32, pw: i32, ph: i32) -> (f32, f32) {
    (
        (lon + 180.0) / 360.0 * (pw - 1) as f32,
        (84.0 - lat) / 150.0 * (ph - 1) as f32,
    )
}

/// Rasterizes the continents into `layer` (which should be freshly cleared).
pub fn paint_land(layer: &mut Dots, color: Color) {
    let (pw, ph) = (layer.pw(), layer.ph());
    if pw < 2 || ph < 2 {
        return;
    }
    for y in 0..ph {
        let lat = 84.0 - y as f32 / (ph - 1) as f32 * 150.0;
        for x in 0..pw {
            let lon = x as f32 / (pw - 1) as f32 * 360.0 - 180.0;
            if LAND.iter().any(|p| inside(p, lon, lat)) {
                layer.set(x, y, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continents_are_where_they_should_be() {
        let land = |lat, lon| LAND.iter().any(|p| inside(p, lon, lat));
        assert!(land(48.0, 10.0)); // central Europe
        assert!(land(-25.0, 135.0)); // Australia
        assert!(land(40.0, -100.0)); // US
        assert!(!land(0.0, -150.0)); // Pacific
        assert!(!land(-40.0, 0.0)); // South Atlantic
    }
}
