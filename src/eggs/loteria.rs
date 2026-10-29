//! A game of lotería is called on Día del Niño: the gritón sings a verse and
//! turns the next card of a row, until the last one and "¡Lotería!".
//!
//! Lotería is the Mexican bingo played with a deck of picture cards, and
//! every child knows it: the gritón calls each card with a rhyme and a name
//! ("El Gallo", "La Dama", "El Catrín"...) and the players mark it on their
//! boards. The cards' names are traditional; everything else here is
//! oxiper's own: eleven cards drawn in original pixel art (El Gallo, La
//! Dama, El Catrín, La Sirena, El Valiente, La Luna, El Corazón, El Sol,
//! La Estrella, El Nopal, La Rana), each with its own number, a flat color
//! behind it and a cream card around it, and a verse in two lines per card,
//! written for the occasion.
//!
//! A row of face-down cards (as many as the screen holds, up to seven, from
//! a different draw each time) is dealt up from the bottom of the screen.
//! Then, card by card, the gritón's verse is typed out above them while a
//! pointer marks the card; then the card turns over, its width closing to
//! nothing and opening again with its face, and its name appears on a plate
//! below. When the last card is up, the cards hop in a wave, "¡LOTERÍA!"
//! bounces above them in letters of every color and confetti falls. The
//! cards, and the lettering, grow with the screen; all of it is over within
//! 30 s.

use super::nino::{Palette, fade};
use super::pen::{fill, glyph_on, text};
use super::{Egg, Info, pixel};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas, Cell};
use crate::theme::Theme;
use std::f32::consts::PI;

pub const INFO: Info = Info {
    name: "loteria",
    start,
    fiesta: Kind::Nino,
    when: super::any_day,
};

/// One card: its number and name, the color behind its picture (a letter of
/// the palette), the verse the gritón calls it with, and its picture, 11
/// pixels across and 14 down.
struct Card {
    number: u8,
    name: &'static str,
    bg: u8,
    verse: [&'static str; 2],
    art: [&'static str; 14],
}

const CARDS: &[Card] = &[
    Card {
        number: 1,
        name: "EL GALLO",
        bg: b'T',
        verse: ["Con su cresta de campeón", "madruga sin ton ni son."],
        art: [
            ".......RR..",
            "...G..RRRR.",
            "..GBG.OOkYY",
            ".GBGBGOOOY.",
            ".GBGBGOOOR.",
            "GBGBGROOR..",
            "GBGRRRRRRR.",
            ".GRRYYRRRR.",
            "..RRYYRRRR.",
            "...RRRRRR..",
            "....RRRR...",
            ".....Y.Y...",
            ".....Y.Y...",
            "....YY.YY..",
        ],
    },
    Card {
        number: 3,
        name: "LA DAMA",
        bg: b'P',
        verse: ["Con su sombrero y su porte,", "se pasea por toda la corte."],
        art: [
            "...MMMMM...",
            "..MMMMMMM..",
            "..MYYYYYM..",
            "MMMMMMMMMMM",
            "..OKKKKKO..",
            ".OOKkKkKOO.",
            ".OOKKRKKOO.",
            "..OKKKKKO..",
            "..WWWWWWW..",
            ".BBWWWWWBB.",
            "BBBBBBBBBBB",
            "BBBBBBBBBBB",
            ".BBBBBBBBB.",
            "BBBBBBBBBBB",
        ],
    },
    Card {
        number: 4,
        name: "EL CATRÍN",
        bg: b'G',
        verse: [
            "Con bastón y sombrero de copa,",
            "pasea lo mejor de su ropa.",
        ],
        art: [
            "...kkkkk...",
            "...kkkkk...",
            "...kRRRk...",
            "..kkkkkkk..",
            "...KKKKK...",
            "...KkKkK...",
            "...kKkKk...",
            "....KKK....",
            "..kkWWWkkYY",
            ".kkkWRWkkkY",
            ".kkkkWkkkkY",
            ".kkkkkkkkkY",
            "..kk..kk..Y",
            "..kk..kk..Y",
        ],
    },
    Card {
        number: 6,
        name: "LA SIRENA",
        bg: b'B',
        verse: ["Entre las olas canta y canta;", "al marinero lo encanta."],
        art: [
            "...OOOOO...",
            "..OOOOOOO..",
            "..OOKKKOO..",
            "..OOKkKOO..",
            "..OOKRKOO..",
            "..OOKKKOO..",
            "...OKKKO...",
            "...KKKKK...",
            "...GGGGG...",
            "...GgGgG...",
            "....GgG....",
            ".....GG....",
            "...GGGGGG..",
            "..GG.GG.GG.",
        ],
    },
    Card {
        number: 12,
        name: "EL VALIENTE",
        bg: b'O',
        verse: ["No hay lunes que lo detenga,", "ni error que lo contenga."],
        art: [
            "..........S",
            "...DDD....S",
            "..DDDDD...S",
            ".DDDDDDD..S",
            "...KKKK..YY",
            "..KkKkK..KK",
            "..KKRKK..K.",
            "...KKK.....",
            "..BBBBBB...",
            ".BBWWWWBB..",
            ".BBBBBBBB..",
            ".BBBBBBBB..",
            "..rr..rr...",
            "..rr..rr...",
        ],
    },
    Card {
        number: 23,
        name: "LA LUNA",
        bg: b'M',
        verse: ["Redonda y de plata fina,", "alumbra sin gasolina."],
        art: [
            "....YYYY...",
            "..YYYYYY...",
            ".YYYYy.....",
            ".YYYy...W.W",
            "YYYYy......",
            "YYYYy..W...",
            "YYkYy......",
            "YYYYy..W...",
            "YYYYYy.....",
            ".YYYYYy....",
            ".YYYYYYY...",
            "..YYYYYYY..",
            "...YYYYYY..",
            "....YYYY...",
        ],
    },
    Card {
        number: 27,
        name: "EL CORAZÓN",
        bg: b'Y',
        verse: ["Late que late sin parar:", "no lo vayas a flechar."],
        art: [
            "...........",
            "...........",
            ".RRR...RRR.",
            "RRRRR.RRRRr",
            "RWRRRRRRRRr",
            "RWRRRRRRRRr",
            "RRRRRRRRRrr",
            ".RRRRRRRRr.",
            "..RRRRRRr..",
            "...RRRRr...",
            "....RRr....",
            ".....r.....",
            "...........",
            "...........",
        ],
    },
    Card {
        number: 46,
        name: "EL SOL",
        bg: b'B',
        verse: ["Madruga sin despertador", "y calienta con su fulgor."],
        art: [
            ".....O.....",
            ".O...O...O.",
            "..O.....O..",
            "....YYY....",
            "..YYYYYYY..",
            ".YYYYYYYYY.",
            "OYYkYYYkYYO",
            "OYYYYYYYYYO",
            ".YYOYYYOYY.",
            "..YYkkkYY..",
            "...YYYYY...",
            "..O.YYY.O..",
            ".O...O...O.",
            ".....O.....",
        ],
    },
    Card {
        number: 35,
        name: "LA ESTRELLA",
        bg: b'R',
        verse: ["Chiquita y bien lejana,", "alumbra hasta la mañana."],
        art: [
            "...........",
            ".....Y.....",
            ".....Y.....",
            "....YYY....",
            "YYYYYYYYYYY",
            ".YYYYYYYYY.",
            "..YYYYYYY..",
            "...YYYYY...",
            "..YYYYYYY..",
            "..YYY.YYY..",
            ".YYY...YYY.",
            ".YY.....YY.",
            "...........",
            "...........",
        ],
    },
    Card {
        number: 39,
        name: "EL NOPAL",
        bg: b'T',
        verse: ["Con sus tunas coloradas", "y sus espinas bien cuidadas."],
        art: [
            "......RGG..",
            "..GG..GkGG.",
            ".GkGG.GGGGR",
            ".GGGG.GGkG.",
            "R.GGGGGGG..",
            "..GGGGGG...",
            "..GkGGkGG..",
            "..GGGGGGG..",
            "...GGGkG...",
            "...GGGGG...",
            "....GGG....",
            "....ddd....",
            "...DDDDD...",
            "...DDDDD...",
        ],
    },
    Card {
        number: 54,
        name: "LA RANA",
        bg: b'Y',
        verse: ["Salta y croa en la laguna", "cuando se asoma la luna."],
        art: [
            "...........",
            "..GG...GG..",
            ".GWkG.GkWG.",
            ".GGGGGGGGG.",
            "GGGGGGGGGGG",
            "GGgGGGGGgGG",
            "GGGGGGGGGGG",
            "GkkkkkkkkkG",
            ".GGGGGGGGG.",
            ".GGGGGGGGG.",
            "GG.GGGGG.GG",
            "GGG.....GGG",
            "GGG.....GGG",
            "...........",
        ],
    },
];

/// A card's size in its own pixels, and the cells its name takes: the
/// pixels it is drawn in, and the plate for the name under it.
const CARD_W: i32 = 13;
const CARD_H: i32 = 22;
/// How many cards at most, how fast the verse is typed (letters per
/// second), the time before the card turns, the turn, how long the face
/// stays before the next verse, the time to deal, the finale.
const MAX_CARDS: usize = 7;
const CPS: f32 = 40.0;
const PAUSE: f32 = 0.25;
const FLIP: f32 = 0.45;
const HOLD: f32 = 0.7;
const DEAL: f32 = 1.4;
const FINALE: f32 = 4.4;
/// The letters of the banner, and the colors they and the confetti take.
const BANNER: &str = "¡LOTERÍA!";
const RAINBOW: [u8; 7] = *b"RYGBPOT";

/// How big the cards are (they grow with the screen, as far as five of them
/// still fit and there is room for the verse above) and how many fit in a row.
fn layout(w: u16, h: u16) -> (i32, usize) {
    let k = ((h as i32 - 9) / 11)
        .min((w as i32 - 2) / (5 * (CARD_W + 2)))
        .clamp(1, 3);
    let n = ((w as i32 - 2) / ((CARD_W + 2) * k)).clamp(1, MAX_CARDS as i32);
    (k, n as usize)
}

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let (k, n) = layout(w, h);
    // A different draw each time.
    let mut deck: Vec<usize> = (0..CARDS.len()).collect();
    for i in (1..deck.len()).rev() {
        deck.swap(i, rng.usize(..=i));
    }
    let mut order = [0; MAX_CARDS];
    order[..n].copy_from_slice(&deck[..n]);
    let mut starts = [0.0; MAX_CARDS + 1];
    starts[0] = DEAL;
    for i in 0..n {
        starts[i + 1] = starts[i] + Loteria::slot(&CARDS[order[i]]);
    }
    Box::new(Loteria {
        t: 0.0,
        k,
        n,
        order,
        starts,
    })
}

struct Loteria {
    t: f32,
    /// The scale of the cards, how many there are and which, and when the
    /// gritón starts each one's verse (and, last, the finale).
    k: i32,
    n: usize,
    order: [usize; MAX_CARDS],
    starts: [f32; MAX_CARDS + 1],
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(2)
}

impl Loteria {
    /// How long a verse takes to type.
    fn typing(card: &Card) -> f32 {
        card.verse.iter().map(|l| l.chars().count()).sum::<usize>() as f32 / CPS
    }

    /// How long a card's turn takes, from the start of its verse to the end
    /// of its hold.
    fn slot(card: &Card) -> f32 {
        Self::typing(card) + PAUSE + FLIP + HOLD
    }

    fn card(&self, i: usize) -> &'static Card {
        &CARDS[self.order[i]]
    }

    /// How far card `i` has turned, 0 face down to 1 face up.
    fn turned(&self, i: usize) -> f32 {
        let at = self.starts[i] + Self::typing(self.card(i)) + PAUSE;
        ((self.t - at) / FLIP).clamp(0.0, 1.0)
    }

    /// The finale starts when the last card is up.
    fn finale(&self) -> f32 {
        self.starts[self.n]
    }

    /// The color of pixel (`u`, `v`) of a card, in the card's own pixels.
    fn card_px(&self, pal: &Palette, card: &Card, face: bool, u: i32, v: i32) -> Color {
        if u == 0 || u == CARD_W - 1 || v == 0 || v == CARD_H - 1 {
            return pal.of(b'c');
        }
        if !face {
            // The back: a blue lattice with a gold diamond in the middle.
            let (du, dv) = ((u - 6).abs(), (2 * v - 21).abs() / 2);
            if du + dv < 3 {
                return pal.of(if du + dv < 2 { b'Y' } else { b'O' });
            }
            let lattice = (u + v) % 4 == 0 || (u + CARD_H - v) % 4 == 0;
            return pal.of(if lattice { b'B' } else { b'b' });
        }
        if v < 7 {
            // The header: the number, in dark red on cream.
            let digits = card.number.to_string();
            for (j, ch) in digits.chars().enumerate() {
                if glyph_on(ch, u - 2 - 4 * j as i32, v - 1) {
                    return pal.of(b'r');
                }
            }
            return pal.of(b'C');
        }
        match card.art[(v - 7) as usize].as_bytes()[(u - 1) as usize] {
            b'.' => pal.of(card.bg),
            ch => pal.of(ch),
        }
    }

    /// Draws a card with its top-left corner at (`x`, `y`) in pixels, turned
    /// `turn` of the way.
    fn draw_card(&self, c: &mut Canvas, pal: &Palette, card: &Card, (x, y): (i32, i32), turn: f32) {
        let k = self.k;
        let wide = (CARD_W * k) as f32 * (turn * PI).cos().abs();
        let wide = wide.round() as i32;
        if wide <= 0 {
            return;
        }
        let left = x + (CARD_W * k - wide) / 2;
        let face = turn >= 0.5;
        let (top, bottom) = ((-y).max(0), (c.h() as i32 * 2 - y).min(CARD_H * k));
        let (first, last) = ((-left).max(0), (c.w() as i32 - left).min(wide));
        for dy in top..bottom {
            let v = dy / k;
            for dx in first..last {
                let u = ((dx as f32 + 0.5) / wide as f32 * CARD_W as f32) as i32;
                pixel(
                    c,
                    left + dx,
                    y + dy,
                    self.card_px(pal, card, face, u.min(CARD_W - 1), v),
                );
            }
        }
    }
}

impl Egg for Loteria {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::nino(theme);
        let (w, h) = (c.w() as i32, c.h() as i32);
        let (k, n) = (self.k, self.n as i32);
        // The row of cards, in the middle, with room for the verse above.
        let (cw, gap) = (CARD_W * k, 2 * k);
        let left = (w - (n * cw + (n - 1) * gap)) / 2;
        let rows = 11 * k + 1;
        let top_row = ((h - rows) / 2 + 2).max(0);
        let finale = self.finale();
        let hop = |i: usize| {
            if self.t < finale {
                return 0;
            }
            ((self.t * 7.0 + i as f32 * 0.9).sin() * 1.5 * k as f32).round() as i32
        };
        let mut pointed = None;
        for i in 0..self.n {
            let dealt = (self.t - 0.15 * i as f32) / 0.5;
            if dealt <= 0.0 {
                continue;
            }
            let x = left + i as i32 * (cw + gap);
            // Dealt up from below the screen, then turned when called.
            let slide = ((1.0 - ease_out(dealt)) * (h as f32 * 2.0 - top_row as f32 * 2.0)) as i32;
            let y = top_row * 2 + slide + hop(i);
            let turn = self.turned(i);
            self.draw_card(c, &pal, self.card(i), (x, y), turn);
            if self.t >= self.starts[i] && turn < 1.0 {
                pointed = Some(x + cw / 2);
            }
            // Its name, on a plate under the card, once it is face up.
            if turn >= 1.0 && slide == 0 {
                let name = self.card(i).name;
                let row = (y + CARD_H * k) / 2;
                let pad = (cw - name.chars().count() as i32) / 2;
                for dx in 0..cw {
                    let ch = name
                        .chars()
                        .nth((dx - pad) as usize)
                        .filter(|_| dx >= pad)
                        .unwrap_or(' ');
                    if c.strict {
                        c.set_attr(x + dx, row, ch, pal.of(b'C'), BOLD);
                    } else {
                        let cell = Cell {
                            ch,
                            fg: pal.of(b'k'),
                            bg: pal.of(b'C'),
                            attr: BOLD,
                        };
                        c.set_cell(x + dx, row, cell);
                    }
                }
            }
        }

        // The gritón's verse, typed out line by line over the cards.
        if let Some(cur) = (0..self.n)
            .rev()
            .find(|&i| self.t >= self.starts[i])
            .filter(|_| self.t < finale)
        {
            let verse = self.card(cur).verse;
            let mut typed = ((self.t - self.starts[cur]) * CPS) as usize;
            c.text_attr(
                (w - 17) / 2,
                top_row - 5,
                "El gritón canta:",
                pal.of(b'w'),
                0,
            );
            for (j, line) in verse.iter().enumerate() {
                let len = line.chars().count();
                let end = line
                    .char_indices()
                    .nth(typed)
                    .map_or(line.len(), |(b, _)| b);
                let shown = &line[..end];
                typed = typed.saturating_sub(len);
                let x = (w - len as i32) / 2;
                c.text_attr(x, top_row - 4 + j as i32, shown, pal.of(b'Y'), BOLD);
            }
            if let Some(px) = pointed {
                c.set_attr(px, top_row - 1, '▼', pal.of(b'Y'), BOLD);
            }
        }

        if self.t >= finale {
            self.finale_show(c, theme, &pal, top_row);
        }
    }

    fn done(&self) -> bool {
        self.t > self.finale() + FINALE
    }
}

impl Loteria {
    /// "¡LOTERÍA!" bouncing over the cards in letters of every color, and
    /// confetti falling.
    fn finale_show(&self, c: &mut Canvas, theme: &Theme, pal: &Palette, top_row: i32) {
        let (w, h) = (c.w() as i32, c.h() as i32);
        let since = self.t - self.finale();
        let letters = BANNER.chars().count() as i32;
        // Whole font pixels per banner pixel, as many as fit with room around.
        let scale = ((w as f32 * 0.75) / (letters * 4 - 1) as f32).clamp(1.0, 4.0) as i32;
        let width = (letters * 4 - 1) * scale;
        let x0 = (w - width) / 2;
        let base = (top_row - 1) * 2 - 5 * scale - 3;
        let pop = ease_out(since / 0.4);
        for (j, ch) in BANNER.chars().enumerate() {
            let bounce = ((since * 8.0 + j as f32 * 0.7).sin() * scale as f32).round() as i32;
            let rise = ((1.0 - pop) * 6.0 * scale as f32) as i32;
            let color = pal.of(RAINBOW[j % RAINBOW.len()]);
            let at = (x0 + j as i32 * 4 * scale, base + bounce + rise);
            text(c, at, ch.encode_utf8(&mut [0; 4]), scale, color);
        }
        // Confetti, from the top; each piece falls at its own speed.
        let count = (w * h / 50).clamp(30, 250);
        for i in 0..count {
            let hash = |s: i32| super::pen::hash(i, s);
            let speed = 7.0 + 13.0 * hash(2);
            let y = (since * speed + hash(3) * h as f32 * 2.0) % (h as f32 * 2.0);
            let x = hash(1) * w as f32 + (since * 2.0 + hash(4) * 6.0).sin() * 1.5;
            let color = pal.of(RAINBOW[(hash(5) * 7.0) as usize % 7]);
            let color = fade(theme, color, ((since - FINALE + 1.0).max(0.0)) * 0.8);
            fill(
                c,
                (x as i32, y as i32),
                (1 + (i % 2), 1 + (i % 3 == 0) as i32),
                color,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    /// Every card's art is a full 11 by 14, its name fits the plate, its
    /// verse fits a small screen, and no two cards are alike.
    #[test]
    fn the_cards_are_whole() {
        for card in CARDS {
            assert!(card.art.iter().all(|r| r.len() == 11), "{}", card.name);
            assert!(
                card.name.chars().count() <= CARD_W as usize,
                "{}",
                card.name
            );
            for line in card.verse {
                assert!(line.chars().count() <= 32, "{line}");
            }
            // Something is drawn on it besides its background.
            assert!(card.art.iter().any(|r| r.bytes().any(|b| b != b'.')));
        }
        for (i, a) in CARDS.iter().enumerate() {
            for b in &CARDS[i + 1..] {
                assert_ne!(a.number, b.number);
                assert_ne!(a.name, b.name);
                assert_ne!(a.art, b.art);
            }
        }
        // The names the brief asks for are all there.
        for name in [
            "EL GALLO",
            "LA DAMA",
            "EL CATRÍN",
            "EL CORAZÓN",
            "LA LUNA",
            "EL VALIENTE",
            "LA SIRENA",
        ] {
            assert!(CARDS.iter().any(|c| c.name == name), "{name}");
        }
    }

    /// The cards are turned one by one, the names show, the game ends with
    /// "¡LOTERÍA!" and takes no more than 30 s, at any size.
    #[test]
    fn the_cards_are_called_and_turned() {
        let theme = Theme::classic();
        for (seed, w, h) in [(1, 100, 30), (2, 160, 48), (3, 60, 20), (4, 240, 70)] {
            let mut e = start(w, h, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(w, h, Color::Reset);
            let mut names = Vec::new();
            let mut sang = false;
            let mut secs = 0.0;
            while !e.done() && secs < 40.0 {
                e.update(0.1);
                secs += 0.1;
                c.clear();
                e.draw(&mut c, &theme);
                let text = c.to_text();
                sang |= text.contains("El gritón canta");
                for card in CARDS {
                    if text.contains(card.name) && !names.contains(&card.name) {
                        names.push(card.name);
                    }
                }
            }
            assert!(secs <= 29.0, "{secs} s at {w}x{h}");
            assert!(sang);
            assert_eq!(names.len(), layout(w, h).1, "{names:?} at {w}x{h}");
        }
    }

    /// A card's width closes and opens again as it turns.
    #[test]
    fn a_card_turns_over() {
        let theme = Theme::classic();
        let e = Loteria {
            t: 0.0,
            k: 1,
            n: 1,
            order: [0; MAX_CARDS],
            starts: [0.0; MAX_CARDS + 1],
        };
        let pal = Palette::nino(&theme);
        let width = |turn: f32| {
            let mut c = Canvas::new(30, 14, Color::Reset);
            e.draw_card(&mut c, &pal, &CARDS[0], (4, 2), turn);
            (0..30)
                .filter(|&x| (0..14).any(|y| c.get(x, y).unwrap().ch != ' '))
                .count()
        };
        assert_eq!(width(0.0), 13);
        assert!(width(0.25) < 13 && width(0.25) > 2);
        assert_eq!(width(0.5), 0);
        assert_eq!(width(1.0), 13);
    }
}
