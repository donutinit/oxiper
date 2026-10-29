//! The facial recognition of the Mictlán: a scanner that checks a sugar skull
//! against the register of the ofrenda, its brackets closing in, a line
//! sweeping down the bone, the landmarks lighting up (the flower eyes, the
//! nose, the teeth) and the match climbing, until it lets the soul in
//! ("ACCESO AL MICTLÁN CONCEDIDO") or, for a living face now and then, says
//! "ACCESO AL MICTLÁN DENEGADO: todavía no te toca". The higher the holiday,
//! the fewer living faces are turned away.

use super::sprite::Sprite;
use super::{Env, Pal, Rect, Widget, muertos, put, put_ch};
use crate::color::{Color, rgb};
use crate::fiesta::Fiesta;
use crate::render::{BOLD, Canvas};
use fastrand::Rng;
use std::fmt::Write;

/// The skull, left half and middle column (`W` bone), 21 pixels wide once
/// mirrored.
const SKULL: [&str; 20] = [
    "......WWWWW",
    "....WWWWWWW",
    "...WWWWWWWW",
    "..WWWWWWWWW",
    ".WWWWWWWWWW",
    ".WWWWWWWWWW",
    "WWWWWWWWWWW",
    "WWWWWWWWWWW",
    "WWWWWWWWWWW",
    "WWWWWWWWWWW",
    ".WWWWWWWWWW",
    ".WWWWWWWWWW",
    "..WWWWWWWWW",
    "..WWWWWWWWW",
    "...WWWWWWWW",
    "...WWWWWWWW",
    "...WWWWWWWW",
    "....WWWWWWW",
    "....WWWWWWW",
    ".....WWWWWW",
];
const SW: i32 = 21;
const SH: i32 = 20;
/// The sprite is the skull with room for the scanner's brackets.
const FW: usize = 29;
const FH: usize = 26;
const PX: i32 = 4;
const PY: i32 = 3;

/// What the scan picks out: (x, y) on the skull.
const LANDMARKS: [(i32, i32); 7] = [
    (10, 2),
    (5, 7),
    (15, 7),
    (10, 10),
    (6, 14),
    (14, 14),
    (10, 18),
];

/// Seconds: scanning, analyzing, the verdict.
const SCAN: f32 = 3.5;
const ANALYZE: f32 = 5.5;
const CYCLE: f32 = 9.0;

struct Subject {
    name: &'static str,
    living: bool,
    /// The match the scan ends with, in percent.
    pct: f32,
    id: u32,
}

pub struct Face {
    mx: Fiesta,
    rng: Rng,
    face: Sprite,
    subject: Subject,
    t: f32,
    /// Where the lines of text are written, reused every frame.
    buf: String,
}

impl Face {
    pub fn new(seed: u64, mx: Fiesta) -> Face {
        let mut f = Face {
            mx,
            rng: Rng::with_seed(seed ^ 0xca1a),
            face: Sprite::new(FW, FH),
            subject: Subject {
                name: "",
                living: false,
                pct: 0.0,
                id: 0,
            },
            t: 0.0,
            buf: String::new(),
        };
        f.t = f.rng.f32() * CYCLE;
        f.next();
        f
    }

    fn next(&mut self) {
        let living = self.rng.f32() < 0.35 - 0.25 * self.mx.level;
        self.subject = Subject {
            name: if living {
                "usuario (vivo)"
            } else {
                muertos::soul(&mut self.rng, self.mx.level)
            },
            living,
            pct: if living {
                self.rng.f32() * 3.0
            } else {
                92.0 + self.rng.f32() * 7.9
            },
            id: self.rng.u32(1000..99999),
        };
    }

    /// Paints the skull (or a living face) with the scan over it.
    fn paint(&mut self, pal: &Pal) {
        let hueso = rgb(0xf4eee2);
        let (body, shade) = if self.subject.living {
            (rgb(0xe8c9a0), rgb(0xc9a77c))
        } else {
            (hueso, rgb(0xc9bfa9))
        };
        let hole = rgb(0x24142e);
        let (naranja, amarillo) = (rgb(0xff8c00), rgb(0xffd200));
        let (rosa, turquesa, morado, verde) =
            (pal.picado[0], pal.picado[3], pal.picado[2], pal.picado[5]);
        let (phase, scan) = self.phase();
        let living = self.subject.living;
        let s = &mut self.face;
        s.clear();
        for (y, row) in SKULL.iter().enumerate() {
            let mut edge = true;
            for (x, ch) in row.chars().enumerate() {
                if ch != 'W' {
                    continue;
                }
                let col = if edge { shade } else { body };
                edge = false;
                let (x, y) = (x as i32, y as i32);
                s.dot(PX + x, PY + y, col);
                s.dot(PX + SW - 1 - x, PY + y, col);
            }
        }
        let dot = |s: &mut Sprite, x: i32, y: i32, col: Color| s.dot(PX + x, PY + y, col);
        if living {
            let hair = rgb(0x3a2418);
            let lip = rgb(0xb5495b);
            for y in 0..4 {
                for x in 0..SW {
                    if (y < 3 || !(5..=15).contains(&x)) && s.is_set(PX + x, PY + y) {
                        dot(s, x, y, hair);
                    }
                }
            }
            for cx in [5, 15] {
                for dx in -1..=1 {
                    dot(s, cx + dx, 5, hair);
                }
                dot(s, cx, 7, hole);
                dot(s, cx, 8, hole);
            }
            for (x, y) in [(10, 10), (10, 11), (9, 11), (11, 11)] {
                dot(s, x, y, shade);
            }
            for x in 7..=13 {
                dot(s, x, 14, lip);
            }
            dot(s, 6, 13, lip);
            dot(s, 14, 13, lip);
            dot(s, 4, 11, rgb(0xe9a39a));
            dot(s, 16, 11, rgb(0xe9a39a));
        } else {
            // A marigold on the forehead, flower eyes, a heart for a nose,
            // teeth, icing in the six colors.
            for (x, y) in [(10, 1), (9, 2), (11, 2), (10, 3)] {
                dot(s, x, y, naranja);
            }
            dot(s, 10, 2, amarillo);
            for (x, y, col) in [
                (7, 2, rosa),
                (13, 2, rosa),
                (5, 4, amarillo),
                (15, 4, amarillo),
                (7, 4, turquesa),
                (13, 4, turquesa),
                (4, 11, naranja),
                (16, 11, naranja),
                (3, 12, turquesa),
                (17, 12, turquesa),
                (5, 13, rosa),
                (15, 13, rosa),
                (6, 18, morado),
                (14, 18, morado),
                (10, 19, verde),
            ] {
                dot(s, x, y, col);
            }
            for cx in [5, 15] {
                for dx in -1..=1 {
                    dot(s, cx + dx, 5, rosa);
                    dot(s, cx + dx, 9, rosa);
                    for y in 6..=8 {
                        dot(s, cx + dx, y, hole);
                    }
                }
                for y in 6..=8 {
                    dot(s, cx - 2, y, rosa);
                    dot(s, cx + 2, y, rosa);
                }
                dot(s, cx, 7, naranja);
            }
            for (x, y) in [(9, 10), (11, 10), (10, 11)] {
                dot(s, x, y, hole);
            }
            for x in 6..=14 {
                dot(s, x, 14, hole);
                dot(s, x, 17, hole);
            }
            for x in [6, 8, 10, 12, 14] {
                dot(s, x, 15, hole);
                dot(s, x, 16, hole);
            }
        }
        // The scan: what it has not reached yet is dim, a bright line sweeps
        // down, the landmarks light up as it passes.
        let sy = PY + (scan * (SH + 2) as f32) as i32 - 1;
        if phase == 0 {
            s.recolor(|y, col| if y > sy { pal.fade(col, 0.75) } else { col });
        }
        let lit = if phase == 0 { pal.cyan } else { pal.amber };
        if phase < 2 {
            for &(x, y) in &LANDMARKS {
                if phase > 0 || PY + y <= sy {
                    let (x, y) = (PX + x, PY + y);
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        s.dot(x + dx, y + dy, lit);
                    }
                    s.dot(x, y, pal.bright);
                }
            }
        }
        if phase == 0 {
            s.rect(PX - 2, sy, SW + 4, 1, pal.bright);
            s.rect(PX - 2, sy - 1, SW + 4, 1, pal.fade(pal.cyan, 0.4));
        }
        // The brackets close in a little as the scan goes on.
        let k = match phase {
            0 => 0,
            1 => 1,
            _ => 2,
        };
        let bracket = match phase {
            2 if living => pal.red,
            2 => pal.green,
            0 => pal.cyan,
            _ => pal.amber,
        };
        let (x0, y0, x1, y1) = (PX - 3 + k, PY - 2 + k, PX + SW + 2 - k, PY + SH + 1 - k);
        for (cx, cy, dx, dy) in [
            (x0, y0, 1, 1),
            (x1, y0, -1, 1),
            (x0, y1, 1, -1),
            (x1, y1, -1, -1),
        ] {
            for i in 0..3 {
                s.dot(cx + dx * i, cy, bracket);
                s.dot(cx, cy + dy * i, bracket);
            }
        }
    }

    /// 0 scanning (and how far down), 1 analyzing, 2 the verdict.
    fn phase(&self) -> (u8, f32) {
        if self.t < SCAN {
            (0, self.t / SCAN)
        } else if self.t < ANALYZE {
            (1, 1.0)
        } else {
            (2, 1.0)
        }
    }

    /// The match as shown now: climbing while it analyzes.
    fn shown(&self) -> f32 {
        let (phase, _) = self.phase();
        match phase {
            0 => 0.0,
            1 => {
                let k = ((self.t - SCAN) / (ANALYZE - SCAN)).clamp(0.0, 1.0);
                self.subject.pct * (1.0 - (1.0 - k) * (1.0 - k))
            }
            _ => self.subject.pct,
        }
    }
}

impl Widget for Face {
    fn name(&self) -> &'static str {
        "rostro-id --calavera"
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.t += dt;
        if self.t >= CYCLE {
            self.t %= CYCLE;
            self.next();
        }
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let (phase, scan) = self.phase();
        let big = r.w >= 62 && r.h > FH as i32 / 2;
        let x0 = if big { 1 + FW as i32 + 2 } else { 0 };
        if big {
            self.paint(pal);
            let y0 = ((r.h - FH as i32 / 2) / 2).max(0);
            self.face.blit(c, r, r.x + 1, r.y + y0);
        }
        let living = self.subject.living;
        let verdict = if living { pal.red } else { pal.green };
        let blink = phase < 2 || self.t < CYCLE - 1.0 || (self.t * 8.0) as i32 % 2 == 0;
        put(
            c,
            r,
            x0,
            0,
            "RECONOCIMIENTO FACIAL DEL MICTL\u{00c1}N",
            pal.bright,
            BOLD,
        );
        let mut y = if big { 2 } else { 1 };
        let mut line = std::mem::take(&mut self.buf);
        let mut row = |y: i32, label: &str, value: std::fmt::Arguments, col: Color| {
            line.clear();
            let _ = write!(line, "{label:<13}");
            let x = put(c, r, x0, y, &line, pal.dim, 0);
            line.clear();
            let _ = line.write_fmt(value);
            put(c, r, x, y, &line, col, 0);
        };
        row(y, "SUJETO", format_args!("{}", self.subject.name), pal.main);
        y += 1;
        if big {
            let id = self.subject.id;
            row(y, "REGISTRO", format_args!("ofrenda_{id:05}"), pal.main);
            y += 1;
        }
        let pct = self.shown();
        let pct_col = match phase {
            0 => pal.dim,
            1 => pal.amber,
            _ => verdict,
        };
        row(y, "COINCIDENCIA", format_args!("{pct:5.1}%"), pct_col);
        y += 1;
        let (state, state_col) = match (phase, living) {
            (0, _) => ("ESCANEANDO", pal.cyan),
            (1, _) => ("ANALIZANDO", pal.amber),
            (_, true) => ("VIVO", pal.red),
            (_, false) => ("EN LA OFRENDA", pal.green),
        };
        row(y, "ESTADO", format_args!("{state}"), state_col);
        y += if big { 2 } else { 1 };
        if phase == 0 {
            let bw = (r.w - x0 - 8).clamp(4, 30);
            let filled = (scan * bw as f32) as i32;
            put(c, r, x0, y, "[", pal.dim, 0);
            for i in 0..bw {
                let (ch, col) = if i < filled {
                    ('\u{2588}', pal.ramp.at(0.45 + 0.5 * i as f32 / bw as f32))
                } else {
                    ('\u{2591}', pal.dim)
                };
                put_ch(c, r, x0 + 1 + i, y, ch, col, 0);
            }
            line.clear();
            let _ = write!(line, "] {:>3.0}%", scan * 100.0);
            put(c, r, x0 + 1 + bw, y, &line, pal.amber, 0);
        } else if phase == 2 && blink {
            put(c, r, x0, y, "ACCESO AL MICTL\u{00c1}N", verdict, BOLD);
            let word = if living { "DENEGADO" } else { "CONCEDIDO" };
            put(c, r, x0, y + 1, word, verdict, BOLD);
            let caption = if living {
                "TODAV\u{00cd}A NO TE TOCA"
            } else {
                "\u{00a1}BIENVENIDO A CASA!"
            };
            put(
                c,
                r,
                x0,
                y + if big { 3 } else { 2 },
                caption,
                pal.bright,
                0,
            );
        }
        self.buf = line;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Fiesta;
    use crate::render::Canvas;
    use crate::scene::Ctx;
    use crate::scenes::hollywood::Pal;
    use crate::theme::Theme;

    fn env(fiesta: Fiesta) -> Env {
        let theme = Theme::classic();
        let o = crate::scene::Opts::new(&[], toml::Table::new());
        let ctx = Ctx {
            theme: &theme,
            opts: &o,
            bg: Color::Reset,
            seed: 1,
            fiesta,
        };
        Env {
            pal: Pal::new(&ctx),
            level: 7,
            day: 0.0,
        }
    }

    #[test]
    fn it_scans_analyzes_and_decides() {
        let fiesta = Fiesta::of_day(10, 28);
        let env = env(fiesta);
        let (mut granted, mut denied, mut scanning, mut analyzing) = (false, false, false, false);
        for seed in 0..30 {
            let mut f = Face::new(seed, fiesta);
            for _ in 0..400 {
                f.update(0.1, &env);
                let mut c = Canvas::new(80, 24, Color::Reset);
                f.draw(
                    &mut c,
                    Rect {
                        x: 0,
                        y: 0,
                        w: 80,
                        h: 24,
                    },
                    &env,
                );
                let t = c.to_text();
                granted |= t.contains("CONCEDIDO");
                denied |= t.contains("DENEGADO") && t.contains("TODAV\u{00cd}A NO TE TOCA");
                scanning |= t.contains("ESCANEANDO");
                analyzing |= t.contains("ANALIZANDO");
            }
        }
        assert!(granted && denied && scanning && analyzing);
    }

    #[test]
    fn fewer_living_faces_are_turned_away_as_the_holiday_builds() {
        let count = |fiesta: Fiesta| {
            let mut f = Face::new(5, fiesta);
            (0..2000)
                .filter(|_| {
                    f.next();
                    f.subject.living
                })
                .count()
        };
        let (first, height) = (count(Fiesta::of_day(10, 28)), count(Fiesta::of_day(11, 2)));
        assert!(first > height * 2 && height > 0, "{first} {height}");
    }

    #[test]
    fn the_skull_has_flowers_and_the_living_face_does_not() {
        let env = env(Fiesta::of_day(11, 2));
        for living in [false, true] {
            let mut f = Face::new(2, Fiesta::of_day(11, 2));
            f.subject.living = living;
            f.t = ANALYZE + 0.5;
            f.paint(&env.pal);
            let mut c = Canvas::new(40, 20, Color::Reset);
            f.face.blit(
                &mut c,
                Rect {
                    x: 0,
                    y: 0,
                    w: 40,
                    h: 20,
                },
                0,
                0,
            );
            let has = |col: Color| {
                c.cells()
                    .iter()
                    .any(|x| x.fg == col || (x.bg == col && x.ch != ' '))
            };
            assert_eq!(has(rgb(0xff8c00)), !living, "the marigold on the forehead");
            assert!(has(env.pal.green) != living || has(env.pal.red) == living);
        }
    }
}
