//! Navidad: a string of foquitos (series lights) with heno and esferas, a
//! piñata during the posadas, and confetti and serpentinas on Año Nuevo.

use super::cord::{Hung, Piece};
use super::drift::{Flecks, tone};
use super::ribbon::Ribbons;
use super::{Fiesta, Look, Overlay, Themed, glow, like, put, solid, wave};
use crate::color::{Color, rgb};
use crate::render::Canvas;
use crate::theme::Theme;

/// How deep the cord sags in the middle of a swag, in pixels, and about how
/// many columns the ties are apart.
const DEPTH: f32 = 3.0;
const SWAG: i32 = 30;
/// An esfera: four pixels wide, a cap and a ball of four rows.
const BALL_W: i32 = 4;
const BALL_H: i32 = 5;
/// The lowest pixel row the lights and the esferas reach (five rows).
const BOTTOM: i32 = 9;
/// The piñata: its rope in pixels, how far the bottom swings either side, and
/// its seven cones, each digit the cone (and color) of a pixel.
const ROPE: i32 = 2;
const SWING: f32 = 2.4;
const PINATA: [&str; 10] = [
    ".....0.....",
    ".6..000..1.",
    ".666000111.",
    "..6660111..",
    "..5660112..",
    ".555542222.",
    "55554432222",
    "...44433...",
    "...44433...",
    "...4...3...",
];

/// What hangs on the string under `classic`.
pub(super) struct Lights {
    /// The bulbs, in the order they repeat along the string.
    pub bulbs: [Color; 6],
    /// The esferas: rojo nochebuena, dorado, plata and verde pino.
    pub esferas: [Color; 4],
    /// The piñata's papel de china.
    pub papel: [Color; 6],
    /// The cord, green like the cable of any series.
    pub cord: Color,
    /// Heno, the gray-green moss.
    pub heno: Color,
}

const LIGHTS: Lights = Lights {
    bulbs: [
        rgb(0xff3b3b), // rojo
        rgb(0x3bd16f), // verde
        rgb(0x3b7bff), // azul
        rgb(0xffd23b), // amarillo
        rgb(0xff5fb4), // rosa
        rgb(0xff8c1a), // naranja
    ],
    esferas: [
        rgb(0xc8102e), // rojo nochebuena
        rgb(0xf2b632), // dorado
        rgb(0xc9ccd6), // plata
        rgb(0x1f6f3a), // verde pino
    ],
    papel: [
        rgb(0xe4007c), // rosa mexicano
        rgb(0xffd200), // amarillo
        rgb(0xff8c00), // naranja
        rgb(0x00aab8), // turquesa
        rgb(0x7028a0), // morado
        rgb(0x8cc828), // verde limón
    ],
    cord: rgb(0x1f6f3a),
    heno: rgb(0x9aa58a),
};

/// Confetti on Año Nuevo: the bulbs' colors, dorado, plata and blanco.
const CONFETTI: Flecks = Flecks {
    tones: &[
        tone(0xff3b3b),
        tone(0x3bd16f),
        tone(0x3b7bff),
        tone(0xffd23b),
        tone(0xff5fb4),
        tone(0xff8c1a),
        tone(0xf2b632),
        tone(0xc9ccd6),
    ],
    share: 0,
    cells: 110,
    themed: Themed::Accents,
};

/// And serpentinas.
const SERPENTINAS: Ribbons = Ribbons {
    colors: &[
        rgb(0xff3b3b),
        rgb(0x3bd16f),
        rgb(0x3b7bff),
        rgb(0xffd23b),
        rgb(0xff5fb4),
        rgb(0xf2b632),
    ],
    cells: 700,
};

/// What Navidad hangs: the lights always, the piñata through the posadas,
/// confetti and serpentinas on Año Nuevo.
pub(super) fn look(fiesta: Fiesta) -> Look {
    let nuevo = fiesta.ano_nuevo();
    Look {
        key: if nuevo { 3 } else { 2 },
        salt: 0x6e61_7669_6461_6421,
        lights: Some(&LIGHTS),
        pinata: fiesta.posadas(),
        flecks: nuevo.then_some(&CONFETTI),
        ribbons: nuevo.then_some(&SERPENTINAS),
        ..Look::NONE
    }
}

impl Overlay {
    /// Strings the lights across the width, a bulb every few columns, with
    /// esferas in some places and swags of heno in others; closer together
    /// and more of them as the holiday builds up.
    pub(super) fn hang_lights(&mut self, _: &Lights, salt: u64) {
        let w = self.w as i32;
        let level = self.level.clamp(0.0, 1.0);
        let mut rng = fastrand::Rng::with_seed(
            super::SEED ^ salt ^ (w as u64) << 32 ^ level.to_bits() as u64,
        );
        self.tie(SWAG);
        self.hung.clear();

        // Heno first, so the bulbs show over it.
        let swags = (w as f32 / 22.0 * (0.4 + 0.9 * level)) as i32;
        for _ in 0..swags {
            self.hung.push(Hung {
                x: rng.i32(-2..w.max(1)),
                piece: Piece::Heno,
                color: 0,
                len: 5 + rng.u8(..9),
                phase: 0.0,
            });
        }

        let pitch = ((7.0 - 3.0 * level).round() as i32).clamp(4, 7);
        let esfera = 0.05 + 0.12 * level;
        let mut last = false;
        let mut x = pitch / 2 + 1;
        let mut k = 0usize;
        while x < w - 1 {
            let ball = !last && rng.f32() < esfera && x + BALL_W < w;
            last = ball;
            let (piece, color, len) = if ball {
                // A short thread where the cord sags, so that the ball keeps
                // to the rows.
                let sag = self.sag_by(x + BALL_W / 2, DEPTH);
                let thread = (1 + rng.u8(..3) as i32).min(BOTTOM - BALL_H - sag);
                (Piece::Esfera, rng.u8(..4), thread.max(1) as u8)
            } else {
                (Piece::Bulb, (k % 6) as u8, (k % 256) as u8)
            };
            self.hung.push(Hung {
                x,
                piece,
                color,
                len,
                phase: rng.f32() * std::f32::consts::TAU,
            });
            k += 1;
            x += pitch + if ball { BALL_W } else { 0 };
        }
    }

    /// Draws the string: the cord, the heno, the bulbs and the esferas.
    pub(super) fn draw_lights(&self, c: &mut Canvas, theme: &Theme, lights: &Lights) {
        let (mut bulbs, mut esferas) = (lights.bulbs, lights.esferas);
        let (mut cord, mut heno) = (lights.cord, lights.heno);
        let mut base = Color::Reset;
        if !theme.native {
            for (i, slot) in bulbs.iter_mut().enumerate() {
                *slot = solid(theme.colors.get(i % theme.colors.len().max(1)), *slot);
            }
            for slot in esferas.iter_mut() {
                *slot = like(theme, *slot);
            }
            let fg = solid(Some(&theme.fg), cord);
            cord = fg.scale(0.7);
            heno = fg.scale(0.6);
            base = theme.bg;
        }
        self.draw_cord(c, cord, DEPTH);
        for h in &self.hung {
            if h.piece == Piece::Heno {
                self.draw_heno(c, h, heno);
            }
        }
        for h in &self.hung {
            match h.piece {
                Piece::Bulb => {
                    let color = bulbs[h.color as usize % bulbs.len()];
                    self.draw_bulb(c, h, color, cord, base);
                }
                Piece::Esfera => {
                    let color = esferas[h.color as usize % esferas.len()];
                    self.draw_esfera(c, h, color, cord);
                }
                _ => {}
            }
        }
    }

    /// How bright bulb `h` is just now: the string runs through a chase, a
    /// breathing wave, an alternation and a twinkle, a few seconds each.
    fn brightness(&self, h: &Hung) -> f32 {
        let i = h.len as f32;
        let s = (self.t % 7.0) as f32;
        match (self.t / 7.0) as u64 % 4 {
            // A comet of light running along the string.
            0 => 0.3 + 0.7 * (1.0 - 0.5 * (s * 4.5 - i).rem_euclid(3.0)).max(0.0),
            // Everything breathing, a wave through it.
            1 => 0.65 + 0.35 * (s * 2.6 + i * 0.7).sin(),
            // Every other bulb in turn.
            2 => 0.3 + 0.7 * (0.5 + 0.5 * (s * 5.0 + std::f32::consts::PI * (i % 2.0)).sin()),
            // Steady, with one now and then flickering.
            _ => {
                let n = (s * 5.0) as u64 + h.x as u64 * 31;
                let flick = (n.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 40).is_multiple_of(9);
                if flick { 0.25 } else { 0.95 }
            }
        }
    }

    /// A foquito: its socket under the cord, the glass lit as bright as it
    /// is just now, a glow round it over what the scene shows.
    fn draw_bulb(&self, c: &mut Canvas, h: &Hung, color: Color, cord: Color, base: Color) {
        let b = self.brightness(h);
        let y = self.sag_by(h.x, DEPTH).max(self.sag_by(h.x + 1, DEPTH));
        let white = Color::Rgb(255, 255, 255);
        for x in h.x..h.x + 2 {
            put(c, x, y + 1, cord.scale(0.55));
            put(c, x, y + 3, color.scale(0.3 + 0.7 * b));
        }
        put(
            c,
            h.x,
            y + 2,
            color.scale(0.35 + 0.7 * b).lerp(white, 0.4 * b),
        );
        put(c, h.x + 1, y + 2, color.scale(0.3 + 0.75 * b));
        for x in [h.x - 1, h.x + 2] {
            glow(c, x, y + 2, color, 0.4 * b, base);
            glow(c, x, y + 3, color, 0.25 * b, base);
        }
        for x in h.x..h.x + 2 {
            glow(c, x, y + 4, color, 0.3 * b, base);
        }
    }

    /// An esfera: a thread, a gold cap and a round ball with its highlight,
    /// turning on the thread a little.
    fn draw_esfera(&self, c: &mut Canvas, h: &Hung, color: Color, cord: Color) {
        let y = self.sag_by(h.x + BALL_W / 2, DEPTH);
        let turn = (wave(self.t, 1.1, h.phase).sin() * 0.7).round() as i32;
        let (top, left) = (y + 1 + h.len as i32, h.x + turn);
        for r in 1..=h.len as i32 {
            put(
                c,
                h.x + 1 + turn * r / h.len.max(1) as i32,
                y + r,
                cord.scale(1.2),
            );
        }
        let gold = rgb(0xf2b632);
        put(c, left + 1, top, gold);
        put(c, left + 2, top, gold.scale(0.8));
        const BALL: [&str; 4] = [".XX.", "XXXX", "XXXX", ".XX."];
        for (r, row) in BALL.iter().enumerate() {
            for (i, ch) in row.bytes().enumerate() {
                if ch != b'X' {
                    continue;
                }
                // Light from the upper left.
                let shade = match (r, i) {
                    (1, 1) | (1, 0) => color.lerp(Color::Rgb(255, 255, 255), 0.55),
                    (2, 3) | (3, 2) | (3, 1) => color.scale(0.65),
                    _ => color,
                };
                put(c, left + i as i32, top + 1 + r as i32, shade);
            }
        }
    }

    /// A swag of heno draped over the cord: a gray-green fringe, hanging a
    /// little lower in the middle, in strands of different lengths.
    fn draw_heno(&self, c: &mut Canvas, h: &Hung, heno: Color) {
        let len = h.len as i32;
        for dx in 0..len {
            let x = h.x + dx;
            let n = (x as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 33;
            let tint = heno.scale(0.75 + 0.12 * (n % 4) as f32);
            let drape =
                (1.4 * (std::f32::consts::PI * (dx as f32 + 0.5) / len as f32).sin()) as i32;
            let y = self.sag_by(x, DEPTH) + drape;
            put(c, x, y, tint);
            // Ragged: strands now and then, longer in the middle.
            if !n.is_multiple_of(5) {
                let reach = 1 + (n >> 3) as i32 % (2 + drape);
                for r in 1..=reach {
                    put(c, x, y + r, tint.scale(1.0 - 0.18 * r as f32));
                }
            }
        }
    }

    /// The piñata of siete picos on its rope, swinging from the middle of the
    /// string: a cone of papel de china in each color around a round body.
    pub(super) fn draw_pinata(&self, c: &mut Canvas, theme: &Theme, lights: &Lights) {
        let mut papel = lights.papel;
        if !theme.native {
            for (i, slot) in papel.iter_mut().enumerate() {
                *slot = solid(theme.colors.get(i % theme.colors.len().max(1)), *slot);
            }
        }
        let cx = self.w as i32 / 2;
        let top = self.sag_by(cx, DEPTH);
        let total = (ROPE + PINATA.len() as i32) as f32;
        let swing = SWING * wave(self.t, 1.25, 0.0).sin();
        // It swings from the rope's top: the lower, the further.
        let shift = |r: i32| (swing * r as f32 / total).round() as i32;
        for r in 1..=ROPE {
            put(c, cx + shift(r), top + r, rgb(0xbcae94));
        }
        for (py, row) in PINATA.iter().enumerate() {
            let r = ROPE + 1 + py as i32;
            for (px, ch) in row.bytes().enumerate() {
                if let Some(s) = (ch as char).to_digit(7) {
                    // Seven cones, six colors: the seventh borrows another.
                    let col = papel[[0, 1, 2, 3, 4, 5, 3][s as usize]];
                    let x = cx - row.len() as i32 / 2 + px as i32 + shift(r);
                    put(c, x, top + r, col);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{after, changed, scene};
    use super::*;
    use crate::fiesta::Kind;
    use crate::render::Cell;

    /// Cells the overlay changed in rows `from..to`.
    fn cells(c: &Canvas, w: u16, from: i32, to: i32) -> usize {
        changed(c, w)
            .iter()
            .filter(|&&(_, y)| (from..to).contains(&y))
            .count()
    }

    #[test]
    fn dresses_every_day_of_navidad() {
        let theme = Theme::classic();
        for (month, day) in [
            (12, 12),
            (12, 16),
            (12, 24),
            (12, 28),
            (12, 31),
            (1, 1),
            (1, 6),
        ] {
            let fiesta = Fiesta::of_day(month, day);
            assert!(fiesta.is(Kind::Navidad));
            let mut o = Overlay::new(120, 36);
            let c = after(&mut o, fiesta, 4.0, &theme);
            let name = format!("{month}-{day}");
            let lights = cells(&c, 120, 0, 5);
            assert!(lights > 180, "{name}: {lights} cells of lights");
            // Confetti and serpentinas only on Año Nuevo.
            let below = cells(&c, 120, 8, 36);
            if fiesta.ano_nuevo() {
                assert!(below > 10, "{name}: {below} cells falling");
            } else {
                assert_eq!(below, 0, "{name}");
            }
            // The piñata only through the posadas (what falls aside).
            o.petals.clear();
            o.ribbons.clear();
            let mut c = scene(120, 36);
            o.draw(&mut c, &theme, fiesta);
            let pinata = cells(&c, 120, 5, 8);
            if fiesta.posadas() {
                assert!(pinata > 3, "{name}: {pinata} cells of piñata");
            } else {
                assert_eq!(pinata, 0, "{name}");
            }
        }
    }

    #[test]
    fn builds_up_to_nochebuena() {
        let theme = Theme::classic();
        let count = |month, day| {
            let mut o = Overlay::new(200, 60);
            let c = after(&mut o, Fiesta::of_day(month, day), 2.0, &theme);
            // Lights, heno and esferas, not the piñata.
            cells(&c, 200, 0, 5)
        };
        assert!(count(12, 24) > count(12, 12) + 40);
        assert!(count(12, 20) > count(12, 12));
        // The bulbs lie closer together.
        let bulbs = |f| {
            let mut o = Overlay::new(200, 60);
            o.update(0.0, f);
            o.hung.iter().filter(|h| h.piece == Piece::Bulb).count()
        };
        let (few, many) = (bulbs(Fiesta::of_day(12, 12)), bulbs(Fiesta::of_day(12, 24)));
        assert!(many > few + 10, "{few} then {many}");
    }

    #[test]
    fn lights_keep_to_five_rows_and_the_pinata_to_eight() {
        let theme = Theme::classic();
        for (fiesta, rows) in [
            (Fiesta::of_day(12, 12), 5),
            (Fiesta::of_day(12, 24), 8),
            (Fiesta::of_day(1, 6), 5),
            (Fiesta::of_day(12, 31), 5),
        ] {
            for w in [9, 40, 80, 121, 200] {
                let mut o = Overlay::new(w, 30);
                o.update(0.0, fiesta);
                // What falls over the whole screen aside.
                o.petals.clear();
                o.ribbons.clear();
                for step in 0..60 {
                    o.update(0.17, fiesta);
                    let mut c = scene(w, 30);
                    o.draw(&mut c, &theme, fiesta);
                    assert_eq!(cells(&c, w, rows, 30), 0, "{w} wide, step {step}");
                    if w >= 40 {
                        assert!(cells(&c, w, 0, rows) > w as usize, "{w} wide");
                    }
                }
            }
        }
    }

    #[test]
    fn the_pinata_needs_room() {
        let theme = Theme::classic();
        let fiesta = Fiesta::of_day(12, 20);
        for (h, hangs) in [(12, false), (19, false), (20, true), (40, true)] {
            let mut o = Overlay::new(100, h);
            let c = after(&mut o, fiesta, 2.0, &theme);
            assert_eq!(cells(&c, 100, 5, h as i32) > 0, hangs, "{h} rows");
        }
    }

    #[test]
    fn the_bulbs_blink_and_the_piñata_swings() {
        let theme = Theme::classic();
        let fiesta = Fiesta::of_day(12, 20);
        let mut o = Overlay::new(100, 30);
        let mut frames = vec![after(&mut o, fiesta, 1.0, &theme).to_text()];
        for _ in 0..8 {
            frames.push(after(&mut o, fiesta, 1.5, &theme).to_text());
        }
        frames.dedup();
        assert!(frames.len() >= 8, "{} different frames", frames.len());
    }

    #[test]
    fn falls_on_ano_nuevo_only() {
        let theme = Theme::classic();
        let new_year = Fiesta::of_day(1, 1);
        let mut o = Overlay::new(100, 30);
        o.update(0.0, new_year);
        assert!(!o.petals.is_empty() && !o.ribbons.is_empty());
        let c = after(&mut o, new_year, 3.0, &theme);
        assert!(cells(&c, 100, 8, 30) > 10);
        // What falls touches only its own pixels.
        o.hung.clear();
        o.ties.clear();
        for _ in 0..40 {
            o.update(0.1, new_year);
            let mut c = scene(100, 30);
            o.draw(&mut c, &theme, new_year);
            let n = changed(&c, 100).len();
            let ribbons: usize = o.ribbons.iter().map(|r| 3 * r.len as usize).sum();
            assert!(n <= 2 * o.petals.len() + ribbons, "{n} cells");
        }
        // The next day's look has none of it.
        let c = after(&mut o, Fiesta::of_day(1, 2), 1.0, &theme);
        assert_eq!(cells(&c, 100, 8, 30), 0);
    }

    #[test]
    fn nothing_on_an_ordinary_day() {
        for fiesta in [Fiesta::NONE, Fiesta::of_day(6, 15), Fiesta::of_day(8, 20)] {
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
        assert!(on(&mut o, Fiesta::of_day(12, 12)));
        assert!(o.hung.iter().any(|h| h.piece == Piece::Bulb));
        assert!(!on(&mut o, Fiesta::NONE));
        assert!(on(&mut o, Fiesta::of_day(12, 24)));
        assert!(!on(&mut o, Fiesta::of_day(2, 2)));
        assert!(on(&mut o, Fiesta::of_day(1, 6)));
        // Día de Muertos takes its place, and gives it back.
        assert!(on(&mut o, Fiesta::of_day(11, 2)));
        assert!(!o.flags.is_empty());
        assert!(on(&mut o, Fiesta::of_day(12, 20)));
    }

    #[test]
    fn theme_colors_under_other_themes() {
        // The esferas stay red, green, gold and silver in the theme's own
        // colors, and the bulbs take its accents in order.
        let nord = Theme::builtin("nord").unwrap();
        let red = like(&nord, rgb(0xc8102e));
        assert!(nord.colors.contains(&red) && red != rgb(0xc8102e));
        assert_eq!(like(&Theme::classic(), rgb(0xc8102e)), rgb(0xc8102e));
        let fiesta = Fiesta::of_day(12, 24);
        let themed = after(&mut Overlay::new(100, 30), fiesta, 2.0, &nord);
        let classic = after(&mut Overlay::new(100, 30), fiesta, 2.0, &Theme::classic());
        assert_ne!(themed.cells(), classic.cells());
    }

    #[test]
    fn the_glow_blends_only_where_it_can() {
        use super::super::halves;
        let (scene_color, bulb, bg) = (rgb(0x204060), rgb(0xff3b3b), rgb(0x282a36));
        let mut c = Canvas::new(4, 2, Color::Reset);
        let lit = Cell {
            ch: '▀',
            fg: scene_color,
            bg: scene_color,
            attr: 0,
        };
        c.set_cell(0, 0, lit);
        glow(&mut c, 0, 0, bulb, 0.5, Color::Reset);
        assert_eq!(halves(c.get(0, 0).unwrap()).0, scene_color.lerp(bulb, 0.5));
        assert_eq!(halves(c.get(0, 0).unwrap()).1, scene_color);
        // Over nothing it glows into the theme's background, or not at all.
        glow(&mut c, 1, 0, bulb, 0.5, bg);
        assert_eq!(halves(c.get(1, 0).unwrap()).0, bg.lerp(bulb, 0.5));
        glow(&mut c, 2, 0, bulb, 0.5, Color::Reset);
        assert_eq!(c.get(2, 0), Some(&Cell::BLANK));
        // Off the canvas: nothing.
        glow(&mut c, -1, 0, bulb, 0.5, bg);
        glow(&mut c, 0, -3, bulb, 0.5, bg);
        glow(&mut c, 0, 9, bulb, 0.5, bg);
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
                (21, 20),
                (300, 90),
                (500, 150),
            ] {
                for fiesta in [
                    Fiesta::of_day(12, 12),
                    Fiesta::of_day(12, 24),
                    Fiesta::of_day(12, 31),
                    Fiesta::of_day(1, 6),
                ] {
                    let mut o = Overlay::new(w, h);
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for dt in [5.0, f32::NAN, -3.0, f32::INFINITY, 0.0, 5.0, 1e9] {
                        o.update(dt, fiesta);
                        c.clear();
                        o.draw(&mut c, &theme, fiesta);
                    }
                    o.resize(h, w);
                    c.resize(h, w);
                    c.strict = true;
                    o.update(5.0, fiesta);
                    o.draw(&mut c, &theme, fiesta);
                }
            }
        }
    }

    #[test]
    fn same_frames_every_time() {
        let theme = Theme::classic();
        for fiesta in [Fiesta::of_day(12, 20), Fiesta::of_day(12, 31)] {
            let frame = || after(&mut Overlay::new(90, 28), fiesta, 6.0, &theme).to_text();
            assert_eq!(frame(), frame());
        }
    }
}
