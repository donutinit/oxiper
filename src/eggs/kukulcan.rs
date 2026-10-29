//! El Castillo of Chichén Itzá rises at the bottom of the screen on the
//! Equinoccio, and Kukulcán, the feathered serpent, comes down its stairs.
//!
//! On the equinoxes, in the afternoon, the sun throws the shadows of the
//! pyramid's nine terraces onto the west balustrade of the north stairway
//! and seven triangles of light appear along it, one after another, until
//! they read as the body of a serpent sliding down to the stone head carved
//! at its foot. This draws that, in original oxiper pixel art that scales to
//! the screen: the pyramid rises in limestone with its nine terraces, its
//! stairway between two balustrades, its temple and roof comb, lit from the
//! west (the right) as the sun sinks and turns to ember; a few people in
//! white watch at its foot. Then the triangles come on one by one from the
//! top, each growing down from its edge and meeting the one before it, until
//! the serpent is whole: the stone head on that side wakes with glowing eyes
//! and a flicking tongue, jade plumes fringe the body, a pulse of brighter
//! light runs down it again and again, sparks lift off it and the crowd
//! raises its arms. When the sun is gone the light fades from the tail down
//! and the pyramid sinks back into the ground. All within 30 s.

use super::equinoccio::{Palette, fade};
use super::pen::{blit, hash};
use super::{Egg, Info, paint, pixel};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;
use std::f32::consts::TAU;

pub const INFO: Info = Info {
    name: "kukulcan",
    start,
    fiesta: Kind::Equinoccio,
    when: super::any_day,
};

/// When things happen, in seconds: the pyramid is risen, the first triangle
/// of light comes on and the next ones follow, the serpent is whole, the
/// light starts to go, the pyramid sinks back and it is over.
const RISEN: f32 = 3.2;
const FIRST: f32 = 4.5;
const GAP: f32 = 1.15;
const GROW: f32 = 0.9;
const WHOLE: f32 = FIRST + GAP * (TRIANGLES - 1) as f32 + GROW;
const FADING: f32 = 21.0;
const SINKING: f32 = 23.4;
const END: f32 = 26.0;
/// The triangles of light, as many as the legend says.
const TRIANGLES: usize = 7;
/// El Castillo has nine terraces.
const TERRACES: f32 = 9.0;

/// Proportions, in halves of the pyramid's base: the stairway's half width
/// and each balustrade's width at the bottom and at the top (the stairway
/// narrows toward the top, as seen from the plaza below).
const STAIR: (f32, f32) = (0.13, 0.075);
const RAIL: (f32, f32) = (0.2, 0.115);
/// How much the terraces narrow from the bottom to the top.
const TAPER: f32 = 0.64;
/// The temple's half width, and the height of the pyramid's body and of the
/// whole in pixels, as shares of the screen's height.
const TEMPLE: f32 = 0.27;
const BODY: f32 = 0.76;

fn start(_w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    // The people, as fractions across the pyramid's base, and how each dresses.
    let people = (0..40)
        .map(|_| Person {
            at: rng.f32() * 2.0 - 1.0,
            cheer: rng.f32() * TAU,
            top: *b"WWWWYWRW".get(rng.usize(..8)).unwrap_or(&b'W'),
            row: rng.u8(0..3),
        })
        .collect();
    Box::new(Kukulcan {
        t: 0.0,
        people,
        seed: rng.i32(0..10_000),
    })
}

struct Person {
    /// Across the base, from -1 (its left corner) to 1.
    at: f32,
    cheer: f32,
    /// The letter of the color of their top.
    top: u8,
    /// In which of the three rows of the crowd they stand.
    row: u8,
}

struct Kukulcan {
    t: f32,
    people: Vec<Person>,
    /// Shifts the pattern of the stones so no two runs build the same wall.
    seed: i32,
}

/// Where the pyramid stands on this screen.
struct Geo {
    /// The middle column, and half the base, in pixels.
    cx: f32,
    b: f32,
    /// The height of the whole and of the terraced body, and of one terrace.
    h: f32,
    hb: f32,
    th: f32,
    /// The pixel row just under the base (it is below the screen while the
    /// pyramid is rising or sinking).
    base: i32,
}

/// The colors a frame needs, found once.
struct Cols {
    stone: Color,
    mid: Color,
    dark: Color,
    shadow: Color,
    sol: Color,
    glow: Color,
    ember: Color,
    jade: Color,
    jade_light: Color,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Light that comes on as `t` goes from 0 to 1, quickly at first.
fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

/// The serpent's stone head, seen from the front, 12 by 12: a crest of
/// plumes, eyes, nostrils, the open mouth with its fangs and a forked
/// tongue. `e` is an eye.
const HEAD: &[&str] = &[
    "..J.jJ.jJ...",
    ".jJjJJjJJj..",
    "..qPPPPPPq..",
    ".qPPPPPPPPq.",
    "qPPeePPeePPq",
    "qPPeePPeePPq",
    "qPPPqPPqPPPq",
    "qPPPPPPPPPPq",
    "qSSSSSSSSSSq",
    "qSWSWSSWSWSq",
    ".qSSSSSSSSq.",
    "..qqRRRRqq..",
];
/// The tongue, drawn apart so it can flick: out, or drawn in.
const TONGUE: [&[&str]; 2] = [&["...R....R..."], &["....R..R...."]];

/// A person seen from a distance, with arms down and with arms raised.
const PERSON: [&[&str]; 2] = [
    &[".n.", ".K.", "TTT", ".T.", ".n.", ".n."],
    &["TnT", "TKT", ".T.", ".T.", ".n.", ".n."],
];

impl Kukulcan {
    /// How far the pyramid has risen: 0 under the ground, 1 standing.
    fn rise(&self) -> f32 {
        let up = ease_out(self.t / RISEN);
        let down = ((self.t - SINKING) / (END - SINKING)).clamp(0.0, 1.0);
        (up - down * down).clamp(0.0, 1.0)
    }

    fn geo(&self, w: i32, rows: i32) -> Geo {
        let screen = rows * 2;
        let h = (screen as f32 * 0.66)
            .min(w as f32 * 0.95 / 1.6)
            .clamp(10.0, 170.0);
        let hb = h * BODY;
        Geo {
            cx: w as f32 / 2.0,
            b: h * 0.8,
            h,
            hb,
            th: hb / TERRACES,
            base: screen + ((1.0 - self.rise()) * (h + 6.0)) as i32,
        }
    }

    /// Triangle `i` of light, 0 at the top: how much of it has come on (it
    /// grows down from its upper edge) and how much of its light is left.
    fn triangle(&self, i: usize) -> (f32, f32) {
        let grown = ((self.t - FIRST - GAP * i as f32) / GROW).clamp(0.0, 1.0);
        let gone = ((self.t - FADING - 0.25 * i as f32) / 0.9).clamp(0.0, 1.0);
        (grown, 1.0 - gone)
    }

    /// Is the serpent whole, and awake?
    fn awake(&self) -> bool {
        self.t >= WHOLE - 0.4 && self.t < FADING + 1.0
    }

    /// A pulse of light running down the serpent from tail to head; `v` is
    /// 0 at its head and 1 at the top.
    fn pulse(&self, v: f32) -> f32 {
        if self.t < WHOLE {
            return 0.0;
        }
        let along = (self.t * 0.85 + v * 1.1).fract();
        (1.0 - (along - 0.5).abs() * 5.0).clamp(0.0, 1.0)
    }

    /// The color of the pixel at (`x`, `y`), or `None` for the sky.
    fn stone(&self, g: &Geo, k: &Cols, x: i32, y: i32) -> Option<Color> {
        let h = (g.base - 1 - y) as f32 + 0.5;
        if !(0.0..g.h).contains(&h) {
            return None;
        }
        let dx = x as f32 + 0.5 - g.cx;
        let ax = dx.abs();
        // The afternoon sun is in the west, on the right: that side is lit,
        // and the whole stone warms as the sun goes down.
        let light = 0.55 + 0.45 * ((dx / g.b + 0.3) / 1.3).clamp(0.0, 1.0);
        let warm = (self.t / FADING).min(1.0) * 0.3;
        let dusk = ((self.t - FADING) / (SINKING - FADING)).clamp(0.0, 1.0);
        let lit = |c: Color, k2: f32| {
            let c = c.scale(light * k2).lerp(k.ember, warm * light);
            c.scale(1.0 - 0.45 * dusk)
        };
        if h >= g.hb {
            return self.temple(g, k, (h - g.hb) / (g.h - g.hb), ax, &lit);
        }
        let tier = ((h / g.th) as i32).min(TERRACES as i32 - 1);
        let hw = g.b * (1.0 - TAPER * tier as f32 / TERRACES);
        if ax > hw {
            return None;
        }
        let f = h / g.th - tier as f32;
        let v = h / g.hb;
        let sw = g.b * lerp(STAIR.0, STAIR.1, v);
        let rail = g.b * lerp(RAIL.0, RAIL.1, v);
        if ax <= sw {
            // The stairway: treads and risers, a step every pixel or more.
            let step = (g.h / 40.0).max(1.0);
            let tread = (h / step) as i32 % 2 == 0;
            return Some(lit(if tread { k.stone } else { k.mid }, 1.04));
        }
        if ax <= sw + rail {
            return Some(self.balustrade(g, k, dx > 0.0, (ax - sw) / rail, v, &lit));
        }
        // A terrace's wall, in courses of stones; a bright edge on top, a
        // shadow under it.
        let edge = 0.9 / g.th;
        if f > 1.0 - edge {
            return Some(lit(k.stone, 1.12));
        }
        if f < edge && tier > 0 {
            return Some(lit(k.dark, 0.9));
        }
        let mut shade = 1.0;
        if g.th >= 5.0 {
            let course = (f * 3.0) as i32 + tier * 3;
            let stone = ((x as f32 + course as f32 * 2.7) / (g.th * 1.1)) as i32;
            shade = 0.93 + 0.14 * hash(stone + self.seed, course);
            let (mortar, joint) = (
                (f * 3.0).fract(),
                ((x as f32 + course as f32 * 2.7) / (g.th * 1.1)).fract(),
            );
            if mortar < 0.8 / (g.th / 3.0) || joint < 0.7 / (g.th * 1.1) {
                shade *= 0.8;
            }
        } else if hash(x + self.seed, y) > 0.8 {
            shade = 0.9;
        }
        Some(lit(k.stone, shade))
    }

    /// A balustrade: limestone sloping down to the serpent's head, and on
    /// the west one (the right) the triangles of light. `u` crosses it from
    /// the stairway (0) outward (1), `v` goes from the bottom (0) to the top.
    fn balustrade(
        &self,
        g: &Geo,
        k: &Cols,
        west: bool,
        u: f32,
        v: f32,
        lit: &impl Fn(Color, f32) -> Color,
    ) -> Color {
        let base = lit(k.mid, if west { 1.05 } else { 0.9 });
        if !west {
            return base;
        }
        // Seven segments from the top down; in each, a right triangle with
        // its long side down the balustrade, pointing alternately outward
        // and inward so one meets the next at its corner.
        let top_down = (1.0 - v) * TRIANGLES as f32;
        let seg = (top_down as usize).min(TRIANGLES - 1);
        let q = top_down - seg as f32;
        let (grown, glow) = self.triangle(seg);
        if grown <= 0.0 || glow <= 0.0 || q > grown {
            return base;
        }
        let inside = if seg.is_multiple_of(2) {
            u >= q - 0.1
        } else {
            u <= 1.1 - q
        };
        if !inside {
            return base;
        }
        let pulse = self.pulse(v);
        let body = k.sol.lerp(k.glow, pulse * 0.9);
        // Plumes of jade fringe the outer edge once the serpent is whole.
        let plume =
            self.t > WHOLE - 1.0 && u > 0.78 && (((g.h * (1.0 - v)) as i32) + seg as i32) % 3 == 0;
        let body = if plume {
            k.jade.lerp(k.jade_light, pulse)
        } else {
            body
        };
        // Newly lit, it flares; then it settles.
        let flare = (1.0 - grown).max(0.0) * 0.5;
        base.lerp(body.lerp(k.glow, flare), 0.92 * glow)
    }

    /// The temple on top: a doorway between pillars, the cornice with its
    /// frieze, and the roof comb; `t` is 0 at its floor and 1 at its top.
    fn temple(
        &self,
        g: &Geo,
        k: &Cols,
        t: f32,
        ax: f32,
        lit: &impl Fn(Color, f32) -> Color,
    ) -> Option<Color> {
        let b = g.b;
        if t < 0.6 {
            if ax > b * TEMPLE {
                return None;
            }
            if ax < b * 0.11 && t < 0.46 {
                // The doorway, dark, between two pillars.
                let pillar = ax > b * 0.075 && ax < b * 0.105;
                return Some(if pillar {
                    lit(k.mid, 1.0)
                } else {
                    lit(k.shadow, 1.0)
                });
            }
            return Some(lit(k.stone, if t < 0.06 { 0.85 } else { 1.0 }));
        }
        if t < 0.8 {
            if ax > b * (TEMPLE + 0.04) {
                return None;
            }
            // The cornice: a dark band with a row of lit studs.
            let stud = ((ax / (b * 0.045)) as i32) % 2 == 0 && t > 0.67 && t < 0.74;
            return Some(lit(if stud { k.stone } else { k.dark }, 0.95));
        }
        // The roof comb, with teeth along its top.
        let width = b * (TEMPLE - 0.06);
        let teeth = ((ax / (b * 0.06)) as i32) % 2 == 0;
        if ax > width || (t > 0.93 && !teeth) {
            return None;
        }
        Some(lit(if t < 0.86 { k.mid } else { k.stone }, 1.0))
    }
}

impl Egg for Kukulcan {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::equinoccio(theme);
        let (w, rows) = (c.w() as i32, c.h() as i32);
        let g = self.geo(w, rows);
        let k = Cols {
            stone: pal.of(b'P'),
            mid: pal.of(b'p'),
            dark: pal.of(b'q'),
            shadow: pal.of(b'S'),
            sol: pal.of(b'Y'),
            glow: pal.of(b'y'),
            ember: pal.of(b'O'),
            jade: pal.of(b'J'),
            jade_light: pal.of(b'j'),
        };
        self.sun(c, theme, &pal, &g);

        // The pyramid, cell by cell.
        let (x0, x1) = (
            ((g.cx - g.b - 1.0) as i32).max(0),
            ((g.cx + g.b + 1.0) as i32).min(w - 1),
        );
        let top_row = ((g.base as f32 - g.h) / 2.0).floor() as i32;
        for row in top_row.max(0)..rows {
            for col in x0..=x1 {
                let up = self.stone(&g, &k, col, row * 2);
                let down = self.stone(&g, &k, col, row * 2 + 1);
                paint(c, col, row, up, down);
            }
        }

        // The stone heads at the foot of the balustrades; the west one wakes.
        let head_scale = (g.h * 0.3 / HEAD.len() as f32).max(0.5);
        let head_w = (HEAD[0].len() as f32 * head_scale) as i32;
        let head_h = (HEAD.len() as f32 * head_scale) as i32;
        let foot = g.b * (STAIR.0 + RAIL.0 / 2.0);
        for west in [false, true] {
            let cx = g.cx + if west { foot } else { -foot };
            let awake = west && self.awake();
            let glint = if awake { self.pulse(0.0).max(0.4) } else { 0.0 };
            let eye = if awake {
                k.sol.lerp(k.glow, glint)
            } else {
                k.dark
            };
            let at = (cx as i32 - head_w / 2, g.base - head_h);
            blit(c, at, HEAD, head_scale, false, |ch| match ch {
                b'e' => Some(eye),
                b'J' | b'j' if !awake => Some(k.mid),
                ch => pal.get(ch),
            });
            if awake {
                let flick = ((self.t * 4.0) as usize) % 2;
                let tongue_at = (at.0, at.1 + (HEAD.len() as f32 * head_scale) as i32);
                blit(c, tongue_at, TONGUE[flick], head_scale, false, |ch| {
                    pal.get(ch)
                });
            }
        }

        self.crowd(c, &pal, &g);
        self.sparks(c, theme, &pal, &g);

        // A word for what it is, once the serpent is whole.
        let caption = "Kukulcán desciende";
        let shown = ((self.t - WHOLE) / 0.8).clamp(0.0, 1.0)
            * (1.0 - ((self.t - FADING) / 1.4).clamp(0.0, 1.0));
        if shown > 0.0 && top_row >= 1 {
            let x = (w - caption.chars().count() as i32) / 2;
            let color = fade(theme, pal.of(b'Y'), 1.0 - shown);
            c.text_attr(x, top_row - 1, caption, color, BOLD);
        }
    }

    fn done(&self) -> bool {
        self.t > END || self.t > 29.5
    }
}

impl Kukulcan {
    /// The sun in the west, sinking toward the horizon as the afternoon goes,
    /// from gold to ember, in a halo.
    fn sun(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, g: &Geo) {
        let screen = c.h() as i32 * 2;
        let low = ((self.t - 0.5) / (SINKING - 0.5)).clamp(0.0, 1.0);
        let r = (screen as f32 * 0.05).max(2.5);
        let x = (g.cx + (g.b * 1.35).max(c.w() as f32 * 0.3)).min(c.w() as f32 - r - 3.0);
        let y = lerp(screen as f32 * 0.2, screen as f32 - r * 0.6, low * low);
        let color = pal.of(b'Y').lerp(pal.of(b'O'), low.powf(1.5));
        let fade_in = ease_out(self.t / 1.5);
        for py in (y - r - 3.0) as i32..=(y + r + 3.0) as i32 {
            for px in (x - r - 3.0) as i32..=(x + r + 3.0) as i32 {
                let d = (px as f32 + 0.5 - x).hypot(py as f32 + 0.5 - y);
                // The disc, then a halo of rings that thins out.
                let k: f32 = if d <= r {
                    0.0
                } else if d <= r + 1.2 {
                    0.55
                } else if d <= r + 2.6 && ((px + py) & 1) == 0 {
                    0.8
                } else {
                    continue;
                };
                pixel(c, px, py, fade(theme, color, k.max(1.0 - fade_in)));
            }
        }
    }

    /// The people at the foot of the pyramid, who raise their arms when the
    /// serpent is whole.
    fn crowd(&self, c: &mut Canvas, pal: &Palette, g: &Geo) {
        let scale = (g.h / 40.0).max(0.6);
        let cheering = self.awake();
        let n = ((g.b / 3.0) as usize).clamp(5, self.people.len());
        // Far rows first, so the near ones stand in front.
        for row in 0..3u8 {
            for p in self.people[..n].iter().filter(|p| p.row == row) {
                // They stand to either side of the stairway, not in front of the heads.
                let x = g.cx + p.at.signum() * (0.34 + 0.66 * p.at.abs()) * g.b * 1.05;
                let hop = if cheering && (self.t * 5.0 + p.cheer).sin() > 0.6 {
                    1
                } else {
                    0
                };
                let y = g.base - (6.0 * scale) as i32 - i32::from(row) - hop;
                let frame = usize::from(cheering && (self.t * 3.0 + p.cheer).sin() > -0.3);
                blit(
                    c,
                    (x as i32, y),
                    PERSON[frame],
                    scale,
                    false,
                    |ch| match ch {
                        b'T' => pal.get(p.top),
                        ch => pal.get(ch),
                    },
                );
            }
        }
    }

    /// Sparks lifting off the serpent while it shines.
    fn sparks(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, g: &Geo) {
        if self.t < WHOLE || self.t > FADING + 1.5 {
            return;
        }
        for i in 0..16 {
            let cycle = 1.5 + hash(i, 7) * 0.8;
            let n = self.t / cycle + hash(i, 3);
            let (round, phase) = (n.floor() as i32, n.fract());
            let v = 0.08 + 0.9 * hash(i, round);
            let sw = g.b * lerp(STAIR.0, STAIR.1, v);
            let rail = g.b * lerp(RAIL.0, RAIL.1, v);
            let x = g.cx + sw + rail * hash(i + 40, round);
            let y = g.base as f32 - g.hb * v - phase * g.h * 0.12;
            if phase < 0.85 {
                pixel(
                    c,
                    x as i32,
                    y as i32,
                    fade(theme, pal.of(b'y'), phase * phase * 0.5),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_art_is_whole() {
        assert!(HEAD.iter().all(|r| r.len() == HEAD[0].len()));
        assert_eq!(HEAD[0].len(), 12);
        for frame in PERSON {
            assert!(frame.iter().all(|r| r.len() == 3));
        }
        assert!(TONGUE.iter().all(|t| t[0].len() == HEAD[0].len()));
    }

    /// Plays `secs` more seconds of the egg and counts what is painted, and
    /// how much of it is sol-colored: the light.
    fn play(e: &mut Box<dyn Egg>, c: &mut Canvas, secs: f32) -> (usize, usize) {
        for _ in 0..(secs * 10.0) as usize {
            e.update(0.1);
        }
        c.clear();
        e.draw(c, &Theme::classic());
        let golden = |cell: &&crate::render::Cell| matches!(cell.fg, Color::Rgb(r, g, b) if r > 230 && g > 180 && b < 140);
        let painted = c.cells().iter().filter(|cell| cell.ch != ' ').count();
        (painted, c.cells().iter().filter(golden).count())
    }

    /// The pyramid rises, the light runs down its west stairway triangle by
    /// triangle, and it all goes away again.
    #[test]
    fn the_serpent_of_light_comes_down() {
        let mut e = start(100, 30, &mut fastrand::Rng::with_seed(1));
        let mut c = Canvas::new(100, 30, Color::Reset);
        let (rising, _) = play(&mut e, &mut c, 0.3);
        let (risen, none) = play(&mut e, &mut c, 3.7);
        assert!(risen > rising * 2, "{rising} then {risen} cells");
        // Light on the stairway's west side: sol-colored pixels, more as
        // the triangles come (the sun's own disc is the first of them).
        let (_, some) = play(&mut e, &mut c, 4.0);
        let (_, whole) = play(&mut e, &mut c, 4.0);
        assert!(some > none && whole > some, "{none} {some} {whole}");
        assert!(!e.done());
        play(&mut e, &mut c, 16.0);
        assert!(e.done());
    }

    #[test]
    fn the_stone_has_a_shape() {
        let e = Kukulcan {
            t: RISEN,
            people: Vec::new(),
            seed: 0,
        };
        let g = e.geo(100, 30);
        let k = Cols {
            stone: Color::Rgb(200, 180, 140),
            mid: Color::Rgb(160, 140, 100),
            dark: Color::Rgb(120, 100, 70),
            shadow: Color::Rgb(60, 50, 40),
            sol: Color::Rgb(255, 207, 74),
            glow: Color::Rgb(255, 240, 168),
            ember: Color::Rgb(255, 138, 48),
            jade: Color::Rgb(46, 139, 87),
            jade_light: Color::Rgb(88, 201, 138),
        };
        let wide = |y: i32| {
            (0..100)
                .filter(|&x| e.stone(&g, &k, x, y).is_some())
                .count()
        };
        // Wider at the bottom than at the top, and the sky above it.
        assert!(
            wide(g.base - 2) > wide(g.base - 30),
            "{} {}",
            wide(g.base - 2),
            wide(g.base - 30)
        );
        assert_eq!(wide(g.base - g.h as i32 - 2), 0);
        assert!(e.stone(&g, &k, 0, g.base - 5).is_none());
        // And its base is, at most, as wide as the screen allows.
        assert!(wide(g.base - 1) as f32 <= g.b * 2.0 + 2.0);
    }
}
