//! Jueves de Corpus: an enramada across the top, a garland of green leaves and
//! paper flowers with small mulitas de hoja de maíz hanging from it, swaying.

use super::cord::{Hung, Piece};
use super::{Fiesta, Look, Overlay, like, put, put2, wave};
use crate::color::{Color, rgb};
use crate::render::Canvas;
use crate::theme::Theme;

/// How deep the garland sags in the middle of a swag, in pixels, and about how
/// many columns the ties are apart.
const DEPTH: f32 = 2.5;
const SWAG: i32 = 34;
/// The leaves' band: three pixels thick, with tips hanging from it.
const BAND: i32 = 3;
/// A paper flower: five pixels across, and about how many columns apart.
const FLOWER_W: i32 = 5;
const PITCH: i32 = 10;
/// How tall the screen has to be for the mulitas to hang (they take ten rows).
const MULITA_ROWS: u16 = 20;
/// A mulita de hoja de maíz facing right, with the load in its panniers: `H`
/// and `h` husk, `I` its lightest, `n` an eye, `P` and `Y` the load.
const MULITA: [&str; 11] = [
    ".......h.h...",
    ".......hHh...",
    ".......HHHHh.",
    ".PY....HnHHHI",
    "PYPY.hHHHHHII",
    "hhhhhhHHHHH..",
    "hHhHhHHHHHh..",
    ".hHhHhHHHh...",
    ".hhhhhhhhh...",
    "..H.H..H.H...",
    "..h.h..h.h...",
];
/// A paper flower: petals and its heart, `c`.
const FLOWER: [&str; 5] = [".X.X.", "XXXXX", "XXcXX", "XXXXX", ".X.X."];

/// What an enramada hangs under `classic`.
pub(super) struct Enramada {
    /// The leaves, in the shade, in the light and in the sun.
    pub leaves: [Color; 3],
    /// The paper flowers: rosa mexicano, amarillo and morado.
    pub flowers: [Color; 3],
    /// The hoja de maíz of the mulitas (its shade and its light follow).
    pub husk: Color,
    /// What the mulitas carry: mango, tuna, limón and naranja.
    pub loads: [Color; 4],
    /// The knots of the garland, wood.
    pub knot: Color,
}

const ENRAMADA: Enramada = Enramada {
    leaves: [rgb(0x1c6a28), rgb(0x2f9a3a), rgb(0x66bd55)],
    flowers: [rgb(0xe4007c), rgb(0xffd200), rgb(0x7028a0)],
    husk: rgb(0xd9c48a),
    loads: [rgb(0xffb02e), rgb(0xc2185b), rgb(0x7cb342), rgb(0xff8c1a)],
    knot: rgb(0xa0703a),
};

/// What Corpus hangs: the enramada, the same all day (it is one day).
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 9,
        salt: 0x636f_7270_7573_2121,
        enramada: Some(&ENRAMADA),
        ..Look::NONE
    }
}

/// The colors an enramada is drawn in, under a theme.
struct Colors {
    leaves: [Color; 3],
    flowers: [Color; 3],
    husk: [Color; 3],
    loads: [Color; 4],
    knot: Color,
    thread: Color,
}

impl Colors {
    /// Under `classic` the colors of the day; under any other theme each takes
    /// the theme's closest in hue, the shades and lights following from those.
    fn of(theme: &Theme, e: &Enramada) -> Colors {
        let white = Color::Rgb(255, 255, 255);
        let husk = like(theme, e.husk);
        let mut colors = Colors {
            leaves: e.leaves,
            flowers: e.flowers,
            husk: [husk, husk.scale(0.8), husk.lerp(white, 0.4)],
            loads: e.loads,
            knot: e.knot,
            thread: e.husk.scale(0.9),
        };
        if !theme.native {
            let green = like(theme, e.leaves[1]);
            colors.leaves = [green.scale(0.65), green, green.lerp(white, 0.3)];
            colors.flowers = e.flowers.map(|f| like(theme, f));
            colors.loads = e.loads.map(|f| like(theme, f));
            colors.knot = like(theme, e.knot);
            colors.thread = husk.scale(0.9);
        }
        colors
    }
}

/// A repeatable number from a column and a salt: the same leaves every frame.
fn grain(x: i32, salt: u64) -> u64 {
    ((x as u64) ^ salt).wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 33
}

impl Overlay {
    /// Hangs the enramada across the width: flowers evenly along the garland,
    /// and, on a screen tall enough, a mulita in every third gap between them.
    pub(super) fn hang_enramada(&mut self, _: &Enramada, salt: u64) {
        let w = self.w as i32;
        let mut rng = fastrand::Rng::with_seed(super::SEED ^ salt ^ (w as u64) << 32);
        self.tie(SWAG);
        self.hung.clear();
        let n = ((w - 2) / PITCH).max(0);
        let x0 = (w - (n * PITCH - (PITCH - FLOWER_W))) / 2;
        let mut avoid = [u8::MAX; 2];
        for i in 0..n {
            let x = x0 + i * PITCH;
            let color = super::papel::other(&mut rng, avoid, 3);
            avoid = [avoid[1], color];
            self.hung.push(Hung {
                x,
                piece: Piece::Flower,
                color,
                len: 0,
                phase: rng.f32() * std::f32::consts::TAU,
            });
            // A mulita hangs in the gap after every third flower.
            let (mx, last) = (x + PITCH / 2 + FLOWER_W / 2 - 6, i + 1 == n);
            if self.h >= MULITA_ROWS && i % 3 == 1 && !last && mx >= 0 {
                self.hung.push(Hung {
                    x: mx,
                    piece: Piece::Mulita,
                    color: rng.u8(..4),
                    len: 2 + rng.u8(..2),
                    phase: rng.f32() * std::f32::consts::TAU,
                });
            }
        }
    }

    /// Draws the garland: its leaves, the mulitas on their threads, the
    /// flowers over them, and the knots at the ties.
    pub(super) fn draw_enramada(&self, c: &mut Canvas, theme: &Theme, e: &Enramada) {
        let colors = Colors::of(theme, e);
        self.draw_leaves(c, &colors);
        for h in self.hung.iter().filter(|h| h.piece == Piece::Mulita) {
            self.draw_mulita(c, h, &colors);
        }
        for h in self.hung.iter().filter(|h| h.piece == Piece::Flower) {
            self.draw_flower(c, h, &colors);
        }
        for &x in &self.ties {
            put2(
                c,
                x,
                0,
                Some(colors.knot.scale(1.15)),
                Some(colors.knot.scale(0.8)),
            );
        }
    }

    /// The band of leaves along the garland: dark, light and in the sun in
    /// patches, tips hanging below it here and there and stirring in the breeze.
    fn draw_leaves(&self, c: &mut Canvas, colors: &Colors) {
        let [shade, mid, sun] = colors.leaves;
        for x in 0..self.w as i32 {
            let y = self.sag_by(x, DEPTH);
            let n = grain(x, 0x1ea5);
            // Fuller in places, where a bunch of leaves is tied on.
            let thick = BAND + i32::from((n >> 9).is_multiple_of(4));
            for r in 0..thick {
                let color = match (r, n % 5) {
                    (0, 0 | 3) => sun,
                    (0, _) => mid,
                    (r, 1) if r + 1 < thick => shade,
                    (r, _) if r + 1 == thick => shade,
                    (_, 2 | 4) => sun.scale(0.8),
                    _ => mid,
                };
                put(c, x, y + r, color);
            }
            // A leaf's tip hangs below, a second one under it now and then.
            if (n >> 3).is_multiple_of(2) {
                let stir = (0.7 * wave(self.t, 1.4, x as f32 * 0.37).sin()).round() as i32;
                put(c, x + stir, y + thick, if x % 2 == 0 { mid } else { shade });
                if (n >> 6).is_multiple_of(3) {
                    put(c, x + stir, y + thick + 1, sun.scale(0.85));
                }
            }
        }
    }

    /// A paper flower sitting on the band, shaded on its lower right.
    fn draw_flower(&self, c: &mut Canvas, h: &Hung, colors: &Colors) {
        let petal = colors.flowers[h.color as usize % 3];
        let heart = colors.flowers[if h.color == 1 { 0 } else { 1 }];
        let y = self.sag_by(h.x + FLOWER_W / 2, DEPTH) - 1;
        for (r, row) in FLOWER.iter().enumerate() {
            for (i, ch) in row.bytes().enumerate() {
                let color = match ch {
                    b'X' if r + i > 4 => petal.scale(0.78),
                    b'X' => petal,
                    b'c' => heart,
                    _ => continue,
                };
                put(c, h.x + i as i32, y + r as i32, color);
            }
        }
    }

    /// A mulita hanging by a thread from the band, turned one way or the other,
    /// swaying from its thread with the breeze.
    fn draw_mulita(&self, c: &mut Canvas, h: &Hung, colors: &Colors) {
        let flip = h.phase > std::f32::consts::PI;
        let (w, rows) = (MULITA[0].len() as i32, MULITA.len() as i32);
        let hang = h.x + w / 2 - 1;
        let start = self.sag_by(hang, DEPTH) + BAND;
        let top = start + h.len as i32;
        let s = self.sway(h.x, h.phase) * 1.6;
        let load = [
            colors.loads[h.color as usize % 4],
            colors.loads[(h.color as usize + 1) % 4],
        ];
        for y in start..top {
            put(c, hang, y, colors.thread);
        }
        for (r, row) in MULITA.iter().enumerate() {
            let dx = (s * (r as f32 + 1.0) / rows as f32).round() as i32;
            for (i, ch) in row.bytes().enumerate() {
                let color = match ch {
                    b'H' => colors.husk[0],
                    b'h' => colors.husk[1],
                    b'I' => colors.husk[2],
                    b'n' => colors.husk[1].scale(0.4),
                    b'P' => load[0],
                    b'Y' => load[1],
                    _ => continue,
                };
                let col = if flip { w - 1 - i as i32 } else { i as i32 };
                put(c, h.x + col + dx, top + r as i32, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Overlay;
    use super::super::tests::{after, changed, scene};
    use super::*;
    use crate::fiesta::Kind;

    /// The Thursday of Corpus Christi of 2027.
    fn day() -> Fiesta {
        let f = Fiesta::of_date(2027, 5, 27);
        assert!(f.is(Kind::Corpus));
        f
    }

    /// Cells the overlay changed in rows `from..to`.
    fn cells(c: &Canvas, w: u16, from: i32, to: i32) -> usize {
        changed(c, w)
            .iter()
            .filter(|&&(_, y)| (from..to).contains(&y))
            .count()
    }

    fn count(o: &Overlay, piece: Piece) -> usize {
        o.hung.iter().filter(|h| h.piece == piece).count()
    }

    #[test]
    fn the_art_is_whole() {
        assert!(MULITA.iter().all(|r| r.len() == MULITA[0].len()));
        assert!(
            MULITA
                .iter()
                .all(|r| r.bytes().all(|b| b".HhInPY".contains(&b)))
        );
        assert!(FLOWER.iter().all(|r| r.len() == FLOWER_W as usize));
    }

    #[test]
    fn dresses_jueves_de_corpus() {
        let theme = Theme::classic();
        for (w, h) in [(80, 24), (120, 36), (200, 60), (500, 150)] {
            let mut o = Overlay::new(w, h);
            let c = after(&mut o, day(), 4.0, &theme);
            // The garland across the top, mulitas under it, nothing below.
            assert!(cells(&c, w, 0, 4) > w as usize * 2, "{w}x{h}");
            assert!(cells(&c, w, 4, 11) > w as usize / 3, "{w}x{h}");
            assert_eq!(cells(&c, w, 11, h as i32), 0, "{w}x{h}");
            assert!(count(&o, Piece::Flower) + 1 >= w as usize / PITCH as usize);
            assert!(count(&o, Piece::Mulita) >= (w / 40) as usize);
            // Neighboring flowers never share a color.
            let flowers: Vec<u8> = o
                .hung
                .iter()
                .filter(|h| h.piece == Piece::Flower)
                .map(|h| h.color)
                .collect();
            assert!(flowers.windows(2).all(|p| p[0] != p[1]));
        }
    }

    #[test]
    fn a_short_screen_hangs_a_garland_and_no_mulitas() {
        let theme = Theme::classic();
        for h in [8, 12, 19] {
            let mut o = Overlay::new(100, h);
            let c = after(&mut o, day(), 3.0, &theme);
            assert_eq!(count(&o, Piece::Mulita), 0, "{h} rows");
            assert!(count(&o, Piece::Flower) > 5);
            assert!(cells(&c, 100, 0, 4) > 200, "{h} rows");
            assert_eq!(cells(&c, 100, 5, h as i32), 0, "{h} rows");
        }
        let mut o = Overlay::new(100, 20);
        after(&mut o, day(), 0.5, &theme);
        assert!(count(&o, Piece::Mulita) >= 2);
    }

    #[test]
    fn the_mulitas_sway_and_the_garland_stirs() {
        let theme = Theme::classic();
        let fiesta = day();
        let mut o = Overlay::new(100, 30);
        o.update(0.0, fiesta);
        let mut frames = Vec::new();
        for _ in 0..80 {
            o.update(0.17, fiesta);
            let mut c = scene(100, 30);
            o.draw(&mut c, &theme, fiesta);
            frames.push(c.to_text());
        }
        frames.dedup();
        assert!(frames.len() > 20, "{} frames", frames.len());
    }

    #[test]
    fn the_theme_paints_the_garland() {
        let fiesta = day();
        let classic = after(&mut Overlay::new(100, 30), fiesta, 2.0, &Theme::classic());
        let colors: Vec<Color> = classic.cells().iter().map(|c| c.fg).collect();
        for hex in [0x2f9a3a, 0xe4007c, 0xffd200, 0x7028a0, 0xd9c48a, 0xc2185b] {
            assert!(colors.contains(&rgb(hex)), "{hex:06x}");
        }
        let nord = Theme::builtin("nord").unwrap();
        let themed = after(&mut Overlay::new(100, 30), fiesta, 2.0, &nord);
        assert_ne!(classic.cells(), themed.cells());
        assert_eq!(cells(&themed, 100, 0, 30), cells(&classic, 100, 0, 30));
    }

    #[test]
    fn nothing_on_the_days_around_it() {
        for fiesta in [
            Fiesta::NONE,
            Fiesta::of_date(2027, 5, 26),
            Fiesta::of_date(2027, 5, 28),
            Fiesta::of_day(6, 15),
        ] {
            let mut o = Overlay::new(80, 24);
            let c = after(&mut o, fiesta, 3.0, &Theme::classic());
            assert!(changed(&c, 80).is_empty());
        }
    }

    #[test]
    fn comes_and_goes_with_the_holiday() {
        let theme = Theme::builtin("dracula").unwrap();
        let mut o = Overlay::new(100, 30);
        let on = |o: &mut Overlay, f| !changed(&after(o, f, 1.0, &theme), 100).is_empty();
        assert!(on(&mut o, day()));
        assert!(count(&o, Piece::Mulita) > 0);
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, Fiesta::of_date(2028, 6, 15)));
        // Another holiday takes the string over, and Corpus again.
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert_eq!(count(&o, Piece::Mulita), 0);
        assert!(on(&mut o, day()));
        assert!(count(&o, Piece::Mulita) > 0);
    }

    #[test]
    fn survives_odd_sizes_and_huge_steps() {
        for theme in [Theme::classic(), Theme::builtin("dracula").unwrap()] {
            for (w, h) in [
                (1, 1),
                (2, 3),
                (7, 4),
                (8, 1),
                (16, 2),
                (22, 30),
                (300, 90),
                (500, 150),
            ] {
                let mut o = Overlay::new(w, h);
                let mut c = Canvas::new(w, h, Color::Reset);
                for dt in [5.0, f32::NAN, -3.0, f32::INFINITY, 0.0, 5.0, 1e9] {
                    o.update(dt, day());
                    c.clear();
                    o.draw(&mut c, &theme, day());
                }
                o.resize(h, w);
                c.resize(h, w);
                c.strict = true;
                o.update(5.0, day());
                o.draw(&mut c, &theme, day());
            }
        }
    }

    #[test]
    fn same_frames_every_time() {
        let theme = Theme::classic();
        let frame = || after(&mut Overlay::new(90, 28), day(), 6.0, &theme).to_text();
        assert_eq!(frame(), frame());
    }
}
