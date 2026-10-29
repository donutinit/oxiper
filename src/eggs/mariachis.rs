//! A little band of skeleton mariachis walks across the bottom of the screen,
//! playing, during Día de Muertos.
//!
//! Original oxiper pixel art: three calaveras in charro suits with silver
//! botonadura down the trousers, rosa bow ties and wide sombreros trimmed in
//! gold. The trumpet leads, its bell bobbing with the beat; the violin
//! follows, bow sawing; the guitarrón brings up the rear, strumming. Their
//! steps keep time but not quite in step, notes (♪ ♫ ♩ ♬) float up from the
//! instruments in the colors of papel picado, sway and fade, and now and
//! then one of them lets out a grito. They come from either side and are
//! gone within 30 s.

use super::muertos::{Palette, fade};
use super::{Egg, Info, sprite};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas};
use crate::theme::Theme;

pub const INFO: Info = Info {
    name: "mariachis",
    start,
    fiesta: Kind::Muertos,
    when: super::any_day,
};

type Frames = [&'static [&'static str]; 2];

/// The trumpet: bell level, then raised with the beat.
const TRUMPET: Frames = [
    &[
        "........nnnn..........",
        ".......nnnnnn.........",
        ".......nnnnnn.........",
        ".......AAAAAA.........",
        "A..nnnnnnnnnnnnnn..A..",
        "AAnnnnnnnnnnnnnnnnAA..",
        ".AAAAAAAAAAAAAAAAAA...",
        ".......WWWWW..........",
        "......WWWWWWW.........",
        "......WkkWkkW.........",
        "......WkkWkkW.........",
        "......WWWkWWW.....AA..",
        ".......WkWkWAAAAAAAAY.",
        "........WWWAaWaWAAAYY.",
        ".......RRWRR.WW...AA..",
        "......nRRnRRnnW.......",
        ".....nnnSnnnnn........",
        ".....nAnSnnnAn........",
        ".....nAnSnnnAn........",
        ".....nnnSnnnnn........",
        "......nnnnnnn.........",
    ],
    &[
        "........nnnn..........",
        ".......nnnnnn.........",
        ".......nnnnnn.........",
        ".......AAAAAA.........",
        "A..nnnnnnnnnnnnnn..A..",
        "AAnnnnnnnnnnnnnnnnAA..",
        ".AAAAAAAAAAAAAAAAAA...",
        ".......WWWWW..........",
        "......WWWWWWW.........",
        "......WkkWkkW.....AA..",
        "......WkkWkkW...AAAAY.",
        "......WWWkWWWAAAAAAYY.",
        ".......WkWkWAaWaW.AA..",
        "........WWW..WW.......",
        ".......RRWRR.WW.......",
        "......nRRnRRnnW.......",
        ".....nnnSnnnnn........",
        ".....nAnSnnnAn........",
        ".....nAnSnnnAn........",
        ".....nnnSnnnnn........",
        "......nnnnnnn.........",
    ],
];
/// The violin: the bow drawn up, then down.
const VIOLIN: Frames = [
    &[
        "........nnnn..........",
        ".......nnnnnn.........",
        ".......nnnnnn.........",
        ".......AAAAAA.........",
        "A..nnnnnnnnnnnnnn..A..",
        "AAnnnnnnnnnnnnnnnnAA..",
        ".AAAAAAAAAAAAAAAAAA...",
        ".......WWWWW..........",
        "......WWWWWWW.........",
        "......WkkWkkW.........",
        "......WkkWkkW.........",
        "...b..WWWkWWW.........",
        "..bBBBBWkWkW...w......",
        ".WBBbBBBWWW...w.......",
        "..WBBBBRRWRR.w........",
        "...WW.nRRnRRWW........",
        ".....nnnSnnnnn........",
        ".....nAnSnnnAn........",
        ".....nAnSnnnAn........",
        ".....nnnSnnnnn........",
        "......nnnnnnn.........",
    ],
    &[
        "........nnnn..........",
        ".......nnnnnn.........",
        ".......nnnnnn.........",
        ".......AAAAAA.........",
        "A..nnnnnnnnnnnnnn..A..",
        "AAnnnnnnnnnnnnnnnnAA..",
        ".AAAAAAAAAAAAAAAAAA...",
        ".......WWWWW..........",
        "......WWWWWWW.........",
        "......WkkWkkW.........",
        "......WkkWkkW.........",
        "...b..WWWkWWW.........",
        "..bBBBBWkWkW..........",
        ".WBBbBBBWWW...........",
        "..WBBBBRRWRRwwwwww....",
        "...WW.nRRnRRnnWW......",
        ".....nnnSnnnnn........",
        ".....nAnSnnnAn........",
        ".....nAnSnnnAn........",
        ".....nnnSnnnnn........",
        "......nnnnnnn.........",
    ],
];
/// The guitarrón: plucking, then the hand drawn back.
const GUITARRON: Frames = [
    &[
        "........nnnn..........",
        ".......nnnnnn.........",
        ".......nnnnnn.........",
        ".......AAAAAA.........",
        "A..nnnnnnnnnnnnnn..A..",
        "AAnnnnnnnnnnnnnnnnAA..",
        ".AAAAAAAAAAAAAAAAAA...",
        ".......WWWWW..........",
        "......WWWWWWW.........",
        "......WkkWkkW.........",
        "......WkkWkkW.........",
        "......WWWkWWW.......bb",
        ".......WkWkW.......bB.",
        "........WWW.......bB..",
        ".......RRWRR.....bBW..",
        "......nRRnRRn...bB....",
        ".....nnnSnnn...bB.....",
        "....BBBBBBBBBBbB......",
        "...BBBBBBBBBBBB.......",
        "..BBBBBbbbBBBBBW......",
        "..BBBBbbbbbBBBBW......",
        "..BBBBBbbbBBBBB.......",
        "...BBBBBBBBBBBB.......",
        "....BBBBBBBBBB........",
        ".....BBBBBBBB.........",
    ],
    &[
        "........nnnn..........",
        ".......nnnnnn.........",
        ".......nnnnnn.........",
        ".......AAAAAA.........",
        "A..nnnnnnnnnnnnnn..A..",
        "AAnnnnnnnnnnnnnnnnAA..",
        ".AAAAAAAAAAAAAAAAAA...",
        ".......WWWWW..........",
        "......WWWWWWW.........",
        "......WkkWkkW.........",
        "......WkkWkkW.........",
        "......WWWkWWW.......bb",
        ".......WkWkW.......bB.",
        "........WWW.......bW..",
        ".......RRWRR.....bBW..",
        "......nRRnRRn...bB....",
        ".....nnnSnnn...bB.....",
        "....BBBBBBBBBBbB......",
        "...BBBBBBBBBBBB.......",
        "..BBBBBbbbBBBBB.......",
        "..BBBBbbbbbBBBBB......",
        "..BBBBBbbbBBBBBW......",
        "...BBBBBBBBBBBBW......",
        "....BBBBBBBBBB........",
        ".....BBBBBBBB.........",
    ],
];
/// Legs: standing, and mid-step.
const LEGS: Frames = [
    &[
        ".......nn.nn........",
        ".......nS.nS........",
        ".......nS.nS........",
        ".......nn.nn........",
        ".......nS.nS........",
        ".......nn.nn........",
        "......bbb.bbb.......",
    ],
    &[
        ".......nnnnn........",
        ".......nS.nS........",
        "......nS...nS.......",
        "......nn...nn.......",
        ".....nS.....nS......",
        ".....nn.....nn......",
        "....bbb.....bbb.....",
    ],
];

/// Each musician's two frames, and the column the music comes out of (in
/// the art, facing right); the notes rise from above the sombreros.
const BAND: [(&Frames, i32); 3] = [(&GUITARRON, 20), (&VIOLIN, 3), (&TRUMPET, 20)];
/// Each musician's width, the room between them, and the whole band's width.
const ART_W: i32 = 22;
const GAP: i32 = 4;
const BAND_W: i32 = 3 * ART_W + 2 * GAP;
/// The upper body's rows; the legs go below it.
const UPPER: i32 = 21;
const HEIGHT: i32 = UPPER + 7;

/// Columns per second at a walk, and columns per step.
const WALK: f32 = 6.0;
const CROSSING: f32 = 26.0;
const STEP: f32 = 2.5;
/// Beats per second.
const TEMPO: f32 = 2.2;

/// Notes in the air at most, how long one lasts, and how fast it rises (rows per second).
const NOTES: usize = 32;
const NOTE_LIFE: f32 = 2.8;
const RISE: f32 = 2.4;
const GLYPHS: [char; 4] = ['♪', '♫', '♩', '♬'];
const NOTE_COLORS: [u8; 6] = *b"ROQYGP";
const GRITOS: [&str; 3] = ["¡Ajúa!", "¡Ay, ay, ay!", "¡Ajúa!"];
const GRITO_FOR: f32 = 1.6;

fn start(w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let mut rng = fastrand::Rng::with_seed(rng.u64(..));
    Box::new(Band {
        t: 0.0,
        dist: 0.0,
        speed: WALK.max((w as f32 + BAND_W as f32 + 2.0) / CROSSING),
        left: rng.bool(),
        w: w as i32,
        notes: [Note::default(); NOTES],
        next: 0,
        note_in: [0.3, 0.7, 0.1],
        grito: (rng.usize(..3), 2.0 + rng.f32() * 3.0, 0),
        rng,
    })
}

#[derive(Clone, Copy, Default)]
struct Note {
    /// Cell column it started from, and its row from the top of the band.
    x: f32,
    y: f32,
    /// Seconds since it was played; 0 means none.
    age: f32,
    glyph: u8,
    color: u8,
}

struct Band {
    t: f32,
    /// Columns walked.
    dist: f32,
    speed: f32,
    /// Walking right to left (mirrored)?
    left: bool,
    w: i32,
    notes: [Note; NOTES],
    next: usize,
    /// Seconds until each musician's next note.
    note_in: [f32; 3],
    /// Who shouts next, in how many seconds, and which grito.
    grito: (usize, f32, usize),
    rng: fastrand::Rng,
}

impl Band {
    /// The left column of musician `i` (0 is the back of the band).
    fn x(&self, i: usize) -> i32 {
        let d = self.dist as i32;
        let back = i as i32 * (ART_W + GAP);
        if self.left {
            self.w - d + BAND_W - ART_W - back
        } else {
            d - BAND_W + back
        }
    }

    /// Mirrors a column of the art when walking left.
    fn mx(&self, x: i32) -> i32 {
        if self.left { ART_W - 1 - x } else { x }
    }

    /// Is musician `i` on the upbeat?
    fn up(&self, i: usize) -> bool {
        (self.t * TEMPO + i as f32 * 0.35) as i32 % 2 == 1
    }
}

impl Egg for Band {
    fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 1.0);
        self.t += dt;
        self.dist += self.speed * dt;
        for n in self.notes.iter_mut().filter(|n| n.age > 0.0) {
            n.age += dt;
            if n.age > NOTE_LIFE {
                n.age = 0.0;
            }
        }
        for (i, &(_, horn)) in BAND.iter().enumerate() {
            self.note_in[i] -= dt;
            if self.note_in[i] > 0.0 {
                continue;
            }
            self.note_in[i] = 0.35 + self.rng.f32() * 0.4;
            let x = self.x(i) + self.mx(horn);
            if x < 0 || x >= self.w {
                continue;
            }
            self.notes[self.next] = Note {
                x: x as f32,
                y: -0.5 - self.rng.f32(),
                age: 1e-3,
                glyph: self.rng.u8(..GLYPHS.len() as u8),
                color: NOTE_COLORS[self.rng.usize(..NOTE_COLORS.len())],
            };
            self.next = (self.next + 1) % NOTES;
        }
        self.grito.1 -= dt;
        if self.grito.1 < -GRITO_FOR {
            self.grito = (
                self.rng.usize(..BAND.len()),
                3.0 + self.rng.f32() * 4.0,
                self.rng.usize(..GRITOS.len()),
            );
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::new(theme);
        let ground = c.h() as i32 * 2 - 1;
        let top = ground + 1 - HEIGHT;
        for (i, (frames, _)) in BAND.iter().enumerate() {
            let x = self.x(i);
            if x + ART_W < 0 || x >= c.w() as i32 {
                continue;
            }
            // Not quite in step: each one's stride is a little behind the one before.
            let step = ((self.dist + i as f32 * 1.2) / STEP) as i32 % 2 == 1;
            let legs = if step { LEGS[1] } else { LEGS[0] };
            let bob = i32::from(!step);
            sprite(
                c,
                (x, top + UPPER),
                legs,
                self.left,
                |_| 0,
                |ch| pal.get(ch),
            );
            let art = frames[usize::from(self.up(i))];
            sprite(c, (x, top - bob), art, self.left, |_| 0, |ch| pal.get(ch));
        }
        // The notes rise and sway, and fade at the end.
        let row0 = top.div_euclid(2);
        for n in self.notes.iter().filter(|n| n.age > 0.0) {
            let k = n.age / NOTE_LIFE;
            let x = n.x + (n.age * 3.0 + n.glyph as f32).sin() * 1.2;
            let y = row0 as f32 + n.y - n.age * RISE;
            let col = fade(theme, pal.of(n.color), ((k - 0.65) / 0.35).max(0.0));
            c.set_attr(
                x.round() as i32,
                y.round() as i32,
                GLYPHS[n.glyph as usize],
                col,
                BOLD,
            );
        }
        let (who, when, which) = self.grito;
        if when <= 0.0 {
            let s = GRITOS[which];
            let len = s.chars().count() as i32;
            let x = self.x(who) + ART_W / 2 - len / 2;
            let rise = (-when / GRITO_FOR * 2.0) as i32;
            c.text_attr(x, row0 - 2 - rise, s, pal.of(b'Y'), BOLD);
        }
    }

    fn done(&self) -> bool {
        let gone = if self.left {
            self.x(BAND.len() - 1) + ART_W < 0
        } else {
            self.x(0) > self.w
        };
        gone && self.notes.iter().all(|n| n.age == 0.0) || self.t > 29.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn the_art_fits() {
        for (frames, hx) in BAND {
            for f in frames {
                assert!(f.iter().all(|r| r.len() as i32 <= ART_W));
                assert!(f.len() as i32 >= UPPER);
                assert!(hx < ART_W);
            }
        }
        assert!(LEGS.iter().all(|l| l.len() as i32 == HEIGHT - UPPER));
    }

    /// The band walks across playing: notes rise above it.
    #[test]
    fn walks_and_plays() {
        let theme = Theme::builtin("dracula").unwrap();
        for seed in 0..4 {
            let mut e = start(120, 36, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(120, 36, Color::Reset);
            let mut notes = 0;
            for _ in 0..60 {
                e.update(0.25);
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                notes += text.chars().filter(|ch| GLYPHS.contains(ch)).count();
            }
            assert!(notes > 20, "{notes}");
        }
    }
}
