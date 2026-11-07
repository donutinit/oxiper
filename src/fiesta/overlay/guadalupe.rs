//! The Virgen de Guadalupe, December 12: a garland of rosas de Castilla across
//! the top with papel picado in rosa and verde azulado hanging from it, gold
//! stars glinting in the cut-outs, and rose petals drifting down.

use super::cord::{Hung, Piece};
use super::drift::{Flecks, tone};
use super::papel::{Design, FLAG_H, Papel};
use super::{Fiesta, Look, Overlay, Themed, like, put, put2, wave};
use crate::color::{Color, rgb};
use crate::render::Canvas;
use crate::theme::Theme;

/// The garland's band of leaves: four pixels at the ties, three where the cord
/// sags, its lower edge always on the fourth row; the flags hang from there.
const BAND: i32 = 4;
/// Pixel row where the flags hang (even: they are laid cell by cell).
const FLAG_TOP: i32 = 4;
/// A rosa de Castilla: five pixels across and as many down, and about how many
/// columns apart.
const ROSE_W: i32 = 5;
const PITCH: i32 = 11;
/// How tall the screen has to be for the papel picado to hang from the
/// garland (six rows); a shorter one gets the garland alone.
const FLAG_ROWS: u16 = 16;
/// A rose from above, its petals curling round the heart: `R` the petal, `L`
/// in the light, `D` in shade.
const ROSE: [&str; 5] = [".LRR.", "LRDRR", "RDLDR", "RRDRD", ".RDD."];

/// The cut-outs: `X` for paper, `.` for a hole and `*` for a hole backed with
/// gold. An estrella, a rosa in gold with its paper spiral, the starry sprinkle
/// of the mantle over a little flower of holes, and a veladora: the flame over
/// its glass.
const DESIGNS: &[Design] = &[
    // estrella
    [
        "XXXXXXX", "XXX*XXX", "XX***XX", "X*****X", "XX***XX", "XX*X*XX", "XXXXXXX", "X.X.X.X",
    ],
    // rosa
    [
        "XXXXXXX", "XX***XX", "X**X**X", "X*X*X*X", "X**X**X", "XX***XX", "XXXXXXX", "X.X.X.X",
    ],
    // manto
    [
        "XXXXXXX", "X*XXX*X", "XXX.XXX", "XX.*.XX", "XXX.XXX", "X*XXX*X", "XXXXXXX", "X.X.X.X",
    ],
    // veladora
    [
        "XXXXXXX", "XXX*XXX", "XXX*XXX", "XX***XX", "XX*X*XX", "XX***XX", "XXXXXXX", "X.X.X.X",
    ],
];

/// The papel picado under `classic`: three rosas, then three verdes azulados.
/// The flags alternate between the two families.
const PAPEL: Papel = Papel {
    designs: DESIGNS,
    colors: [
        rgb(0xd81b60),
        rgb(0xf06292),
        rgb(0xad1457),
        rgb(0x00796b),
        rgb(0x26a69a),
        rgb(0x00695c),
    ],
    // The cord, natural string.
    cord: rgb(0xbcae94),
    gap: 2,
    ordered: false,
    banderitas: false,
    themed: Themed::Like,
};

/// Rose petals: the two pinks of the roses, and a paler one.
const PETALS: Flecks = Flecks {
    tones: &[tone(0xd81b60), tone(0xf06292), tone(0xf8a5c2)],
    share: 0,
    cells: 150,
    themed: Themed::Like,
};

/// What the garland is made of under `classic`.
pub(super) struct Rosas {
    /// The rosas de Castilla, in two pinks.
    pub roses: [Color; 2],
    /// The rose leaves, in the shade, in the light and in the sun.
    pub leaves: [Color; 3],
    /// The gold behind the stars.
    pub gold: Color,
    /// The ribbon at the ties, verde azulado like the Virgen's mantle.
    pub ribbon: Color,
}

const ROSAS: Rosas = Rosas {
    roses: [rgb(0xd81b60), rgb(0xf06292)],
    leaves: [rgb(0x1b5e20), rgb(0x2e7d32), rgb(0x4f9a52)],
    gold: rgb(0xf2c14e),
    ribbon: rgb(0x00796b),
};

/// What December 12 hangs, instead of the Navidad's foquitos.
pub(super) fn look(_: Fiesta) -> Look {
    Look {
        key: 10,
        salt: 0x6775_6164_616c_7570,
        papel: Some(&PAPEL),
        flecks: Some(&PETALS),
        rosas: Some(&ROSAS),
        ..Look::NONE
    }
}

/// The colors the garland is drawn in, under a theme.
struct Colors {
    roses: [Color; 2],
    leaves: [Color; 3],
    gold: Color,
    ribbon: Color,
}

impl Colors {
    /// Under `classic` the colors of the day; under any other theme each takes
    /// the theme's closest in hue, the second rose a paler one of the first.
    fn of(theme: &Theme, r: &Rosas) -> Colors {
        if theme.native {
            return Colors {
                roses: r.roses,
                leaves: r.leaves,
                gold: r.gold,
                ribbon: r.ribbon,
            };
        }
        let white = Color::Rgb(255, 255, 255);
        let green = like(theme, r.leaves[1]);
        let rose = like(theme, r.roses[0]);
        Colors {
            roses: [rose, rose.lerp(white, 0.3)],
            leaves: [green.scale(0.65), green, green.lerp(white, 0.3)],
            gold: like(theme, r.gold),
            ribbon: like(theme, r.ribbon),
        }
    }
}

/// A repeatable number from a column and a salt: the same leaves every frame.
fn grain(x: i32, salt: u64) -> u64 {
    ((x as u64) ^ salt).wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 33
}

impl Overlay {
    /// Hangs the garland on the cord `hang_papel` has strung: roses evenly
    /// along it, and the flags (rosa and verde azulado in turn) from its lower
    /// edge, unless the screen is too short for them.
    pub(super) fn hang_rosas(&mut self, _: &Rosas, salt: u64) {
        let w = self.w as i32;
        if self.h < FLAG_ROWS {
            self.flags.clear();
            self.upper = 0;
            self.cover.fill(0);
        }
        // The flags alternate between the families, a shade of each in turn.
        for (i, f) in self.flags.iter_mut().enumerate() {
            f.top = FLAG_TOP;
            f.color = ((i % 2) * 3 + (i / 2) % 3) as u8;
        }
        self.hung.clear();
        let mut rng = fastrand::Rng::with_seed(super::SEED ^ salt ^ (w as u64) << 32);
        let n = ((w - 2) / PITCH).max(0);
        let x0 = (w - (n * PITCH - (PITCH - ROSE_W))) / 2;
        for i in 0..n {
            self.hung.push(Hung {
                x: x0 + i * PITCH,
                piece: Piece::Rosa,
                color: (i % 2) as u8,
                len: 0,
                phase: rng.f32() * std::f32::consts::TAU,
            });
        }
    }

    /// Draws over the papel picado: the gold in its stars, then the garland
    /// (its leaves, the ribbons at the ties, the roses).
    pub(super) fn draw_rosas(&self, c: &mut Canvas, theme: &Theme, r: &Rosas) {
        let colors = Colors::of(theme, r);
        self.draw_gold(c, &colors);
        self.draw_band(c, &colors);
        for &x in &self.ties {
            self.draw_bow(c, x, &colors);
        }
        for h in self.hung.iter().filter(|h| h.piece == Piece::Rosa) {
            self.draw_rose(c, h, &colors);
        }
    }

    /// Gold in the stars of the flags, swaying with the paper and glinting.
    fn draw_gold(&self, c: &mut Canvas, colors: &Colors) {
        let white = Color::Rgb(255, 255, 255);
        for f in &self.flags[..self.upper] {
            let s = self.sway(f.x, f.phase);
            let design = &DESIGNS[f.design as usize % DESIGNS.len()];
            for (r, row) in design.iter().enumerate() {
                let shift = (s * r as f32 / (FLAG_H - 1) as f32).round() as i32;
                for (i, ch) in row.bytes().enumerate() {
                    if ch != b'*' {
                        continue;
                    }
                    let glint = wave(self.t, 2.3, f.phase + (i + 3 * r) as f32).sin();
                    let gold = colors
                        .gold
                        .scale(0.88 + 0.12 * glint)
                        .lerp(white, (0.45 * (glint - 0.6)).max(0.0));
                    put(c, f.x + i as i32 + shift, f.top + r as i32, gold);
                }
            }
        }
    }

    /// The band of leaves along the cord: dark, light and in the sun in
    /// patches, tips hanging below it here and there and stirring in the
    /// breeze.
    fn draw_band(&self, c: &mut Canvas, colors: &Colors) {
        let [shade, mid, sun] = colors.leaves;
        for x in 0..self.w as i32 {
            let y = self.sag(x);
            let n = grain(x, 0x7e5a);
            for r in y..BAND {
                let color = match (BAND - 1 - r, n % 5) {
                    (0, _) => shade,
                    (_, 0 | 3) if r == y => sun,
                    (_, 1) => shade,
                    (_, 2 | 4) => sun.scale(0.8),
                    _ => mid,
                };
                put(c, x, r, color);
            }
            if (n >> 3).is_multiple_of(2) {
                let stir = (0.7 * wave(self.t, 1.4, x as f32 * 0.37).sin()).round() as i32;
                put(c, x + stir, BAND, if x % 2 == 0 { mid } else { shade });
            }
        }
    }

    /// A bow of ribbon tied at the cord, its tails hanging.
    fn draw_bow(&self, c: &mut Canvas, x: i32, colors: &Colors) {
        let ribbon = colors.ribbon;
        let tail = (0.6 * wave(self.t, 1.7, x as f32 * 0.5).sin()).round() as i32;
        put2(c, x, 0, Some(ribbon.scale(1.2)), Some(ribbon));
        for dx in [-1, 1] {
            put(c, x + dx, 0, ribbon.scale(1.1));
            put(c, x + dx, 1, ribbon.scale(0.85));
            put(c, x + dx * 2 + tail * dx, 2, ribbon.scale(0.8));
        }
    }

    /// A rosa de Castilla, lit from the upper left.
    fn draw_rose(&self, c: &mut Canvas, h: &Hung, colors: &Colors) {
        let base = colors.roses[h.color as usize % 2];
        let white = Color::Rgb(255, 255, 255);
        for (r, row) in ROSE.iter().enumerate() {
            for (i, ch) in row.bytes().enumerate() {
                let color = match ch {
                    b'R' => base,
                    b'L' => base.lerp(white, 0.4),
                    b'D' => base.scale(0.68),
                    _ => continue,
                };
                put(c, h.x + i as i32, r as i32, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{after, changed, scene};
    use super::super::{Overlay, UPPER};
    use super::*;
    use crate::fiesta::Kind;

    /// The Virgen de Guadalupe.
    fn day() -> Fiesta {
        let f = Fiesta::of_day(12, 12);
        assert!(f.is(Kind::Navidad) && f.guadalupe());
        f
    }

    /// Cells the overlay changed in rows `from..to`.
    fn cells(c: &Canvas, w: u16, from: i32, to: i32) -> usize {
        changed(c, w)
            .iter()
            .filter(|&&(_, y)| (from..to).contains(&y))
            .count()
    }

    fn roses(o: &Overlay) -> usize {
        o.hung.iter().filter(|h| h.piece == Piece::Rosa).count()
    }

    #[test]
    fn the_art_is_whole() {
        assert!(ROSE.iter().all(|r| r.len() == ROSE_W as usize));
        assert!(ROSE.iter().all(|r| r.bytes().all(|b| b".LRD".contains(&b))));
        for design in DESIGNS {
            assert!(design.iter().all(|r| r.len() == 7));
            assert!(
                design
                    .iter()
                    .all(|r| r.bytes().all(|b| b"X.*".contains(&b)))
            );
            assert_eq!(design[0], "XXXXXXX", "a folded hem");
            assert_eq!(design[7], "X.X.X.X", "the fringe");
            // Every one has its gold.
            assert!(design.iter().any(|r| r.contains('*')));
        }
    }

    #[test]
    fn dresses_the_virgen_de_guadalupe() {
        let theme = Theme::classic();
        for (w, h) in [(80, 24), (120, 36), (200, 60), (500, 150)] {
            let mut o = Overlay::new(w, h);
            let c = after(&mut o, day(), 4.0, &theme);
            // The garland and the papel picado across the top, petals below.
            assert!(cells(&c, w, 0, 2) > w as usize, "{w}x{h}");
            assert!(cells(&c, w, 2, 6) > w as usize, "{w}x{h}");
            assert!(cells(&c, w, 6, h as i32) > 0, "{w}x{h}: no petals");
            assert_eq!(roses(&o) as i32, ((w as i32 - 2) / PITCH).max(0));
            assert_eq!(o.upper, o.flags.len());
            assert!(!o.flags.is_empty());
        }
    }

    #[test]
    fn the_flags_alternate_rosa_and_verde_azulado() {
        let mut o = Overlay::new(120, 30);
        o.update(0.0, day());
        assert!(o.flags.len() > 10);
        for (i, f) in o.flags.iter().enumerate() {
            assert_eq!(f.color >= 3, i % 2 == 1, "flag {i}");
            assert_eq!(f.top, FLAG_TOP);
        }
        // Neighbors never share a cut-out.
        assert!(o.flags.windows(2).all(|p| p[0].design != p[1].design));
    }

    #[test]
    fn the_colors_of_the_day() {
        let c = after(&mut Overlay::new(100, 30), day(), 2.0, &Theme::classic());
        let colors: Vec<Color> = c.cells().iter().flat_map(|c| [c.fg, c.bg]).collect();
        // The paper darkens a little as it swings: near the color will do.
        let near = |hex: u32| {
            colors.iter().any(|c| match (c, rgb(hex)) {
                (Color::Rgb(r, g, b), Color::Rgb(r2, g2, b2)) => {
                    r.abs_diff(r2).max(g.abs_diff(g2)).max(b.abs_diff(b2)) < 24
                }
                _ => false,
            })
        };
        // Both roses, the leaves, the gold, both families of paper.
        for hex in [
            0xd81b60, 0xf06292, 0x2e7d32, 0xf2c14e, 0xad1457, 0x00796b, 0x26a69a, 0x00695c,
        ] {
            assert!(near(hex), "{hex:06x}");
        }
    }

    #[test]
    fn the_garland_and_the_papel_keep_to_six_rows() {
        let theme = Theme::classic();
        for w in [9, 40, 80, 121, 200] {
            let mut o = Overlay::new(w, 30);
            o.update(0.0, day());
            o.petals.clear();
            for step in 0..60 {
                o.update(0.17, day());
                let mut c = scene(w, 30);
                o.draw(&mut c, &theme, day());
                assert_eq!(cells(&c, w, 6, 30), 0, "{w} wide, step {step}");
                if w >= 40 {
                    assert!(cells(&c, w, 0, 6) > 3 * w as usize, "{w} wide");
                }
            }
        }
    }

    #[test]
    fn a_short_screen_hangs_the_garland_alone() {
        let theme = Theme::classic();
        for h in [4, 8, 12, 15] {
            let mut o = Overlay::new(100, h);
            let c = after(&mut o, day(), 2.0, &theme);
            o.petals.clear();
            assert!(o.flags.is_empty(), "{h} rows");
            assert!(roses(&o) > 5);
            let mut c2 = scene(100, h);
            o.draw(&mut c2, &theme, day());
            assert_eq!(cells(&c2, 100, 3, h as i32), 0, "{h} rows");
            assert!(cells(&c, 100, 0, 3) > 200, "{h} rows");
        }
        let mut o = Overlay::new(100, FLAG_ROWS);
        after(&mut o, day(), 0.5, &theme);
        assert!(!o.flags.is_empty());
    }

    #[test]
    fn petals_touch_only_their_cells() {
        let mut o = Overlay::new(120, 40);
        o.update(0.0, day());
        o.flags.clear();
        o.upper = 0;
        o.cover.fill(UPPER);
        o.ties.clear();
        o.hung.clear();
        for _ in 0..50 {
            o.update(0.1, day());
            let mut c = scene(120, 40);
            o.draw(&mut c, &Theme::classic(), day());
            // The band along the top aside, each petal covers two pixels at
            // most.
            let n = cells(&c, 120, 3, 40);
            assert!(n > 0 && n <= 2 * o.petals.len(), "{n} cells");
        }
    }

    #[test]
    fn the_flags_sway_and_the_gold_glints() {
        let theme = Theme::classic();
        let mut o = Overlay::new(100, 30);
        o.update(0.0, day());
        o.petals.clear();
        let mut frames = Vec::new();
        for _ in 0..80 {
            o.update(0.17, day());
            let mut c = scene(100, 30);
            o.draw(&mut c, &theme, day());
            frames.push(c.to_text());
        }
        frames.dedup();
        assert!(frames.len() > 20, "{} frames", frames.len());
    }

    #[test]
    fn the_theme_paints_the_garland() {
        let nord = Theme::builtin("nord").unwrap();
        let classic = after(&mut Overlay::new(100, 30), day(), 2.0, &Theme::classic());
        let themed = after(&mut Overlay::new(100, 30), day(), 2.0, &nord);
        assert_ne!(classic.cells(), themed.cells());
        assert_eq!(cells(&themed, 100, 0, 30), cells(&classic, 100, 0, 30));
        // Nothing of the classic pinks is left.
        let pinks = themed
            .cells()
            .iter()
            .flat_map(|c| [c.fg, c.bg])
            .filter(|&c| c == rgb(0xd81b60) || c == rgb(0xf06292))
            .count();
        assert_eq!(pinks, 0);
    }

    #[test]
    fn the_foquitos_are_back_the_day_after() {
        let theme = Theme::classic();
        let mut o = Overlay::new(120, 36);
        let twelfth = after(&mut o, day(), 2.0, &theme);
        assert!(roses(&o) > 0 && o.hung.iter().all(|h| h.piece == Piece::Rosa));
        let thirteenth = after(&mut o, Fiesta::of_day(12, 13), 2.0, &theme);
        assert_ne!(twelfth.cells(), thirteenth.cells());
        assert_eq!(roses(&o), 0);
        assert!(o.hung.iter().any(|h| h.piece == Piece::Bulb));
        // Lights, five rows; what it left behind does not show.
        assert_eq!(cells(&thirteenth, 120, 5, 36), 0);
        // And the Virgen again the year after.
        after(&mut o, day(), 1.0, &theme);
        assert!(roses(&o) > 0);
    }

    #[test]
    fn nothing_on_the_days_around_it() {
        // December 11 is an ordinary day.
        for fiesta in [Fiesta::NONE, Fiesta::of_day(12, 11), Fiesta::of_day(8, 20)] {
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
        assert!(!on(&mut o, Fiesta::of_day(12, 11)));
        assert!(on(&mut o, day()));
        assert!(on(&mut o, Fiesta::of_day(12, 16)));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, day()));
        // Another holiday takes the string over, and the Virgen again.
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert_eq!(roses(&o), 0);
        assert!(on(&mut o, day()));
        assert!(roses(&o) > 0);
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
