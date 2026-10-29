//! A calaverita literaria types itself out on a card in a corner of the
//! screen during Día de Muertos.
//!
//! Calaveritas are the rhymed verses Mexicans write for the holiday, in which
//! La Muerte comes for someone and something funny happens on the way. These
//! are oxiper's own, about its own characters (the immortal tiger and its
//! UFO, the shark, the dog, the bonsai, Nyan Cat, the DVD logo, the
//! hollywood hacker, the orcas, the manta, sl's train, the duck, the aurora,
//! the fish of the aquarium, Neo, the pipes, the orrery, the bonfire, the
//! fireworks, the rain, the flock, the programmer who installs screensavers
//! instead of working, and the one reading them), in octosyllables rhyming
//! ABCB, with the Mexican slang the genre likes. There are 23, a quatrain or
//! two each; one is picked at random each time.
//!
//! The card opens in a bottom corner, framed in rosa mexicano with
//! "Calaverita" between two marigolds on its top edge; the verse types
//! itself out behind a blinking cempasúchil cursor, pausing at commas and
//! line ends, each letter glowing like a candle as it lands; it holds long
//! enough to read and laugh, then crumbles away cell by cell. Inside the card
//! the scene is cleared so the verse can be read; outside it, nothing is
//! touched.

use super::muertos::{Palette, fade};
use super::{Egg, Info};
use crate::color::Color;
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas, Cell, ITALIC};
use crate::theme::Theme;

pub const INFO: Info = Info {
    name: "calaverita",
    start,
    fiesta: Kind::Muertos,
    when: super::any_day,
};

/// The verses; an empty line parts two stanzas.
const VERSES: &[&[&str]] = &[
    &[
        "Mil años, tras el felino",
        "que nunca se ha de morir,",
        "la Flaca, de mata en mata,",
        "ni un bigote logró asir.",
        "",
        "Pero un ovni, de repente,",
        "descendió y se lo llevó;",
        "«¡Me quitan hasta la chamba",
        "los marcianos!», protestó.",
    ],
    &[
        "Bajó la Muerte hasta el mar",
        "por el tiburón temido;",
        "lo quería sepultar",
        "en un arrecife hundido.",
        "",
        "Mas el tiburón la olió,",
        "le dio una enorme mordida",
        "y la escupió, muy ofendido:",
        "«¡Qué flaca y qué desabrida!»",
    ],
    &[
        "Vino la Muerte hasta el prado",
        "por el perro juguetón;",
        "él la vio, movió la cola",
        "y le robó el esternón.",
        "",
        "Lo enterró junto a un rosal,",
        "y la Flaca, sin su hueso,",
        "volvió chueca al panteón",
        "gritando: «¡Perro travieso!»",
    ],
    &[
        "Con su guadaña, la Flaca",
        "vino a cortar el bonsái;",
        "tiró tajos a lo loco",
        "como un ninja samurái.",
        "",
        "Quedó tan bonito el árbol",
        "que hoy la Flaca es jardinera:",
        "poda bonsáis los domingos",
        "y almas, la semana entera.",
    ],
    &[
        "Persiguió por las galaxias",
        "al gato de la galleta;",
        "lo alcanzó la Calavera",
        "trepada sobre un cometa.",
        "",
        "Mas su canción pegajosa",
        "se le quedó en la cabeza:",
        "hoy la Muerte canta «nyan»",
        "y, cuando termina, empieza.",
    ],
    &[
        "La Muerte esperó en la esquina",
        "al logo del DVD;",
        "rebotaba y rebotaba",
        "a un pelito de su pie.",
        "",
        "Diez años ahí sentada;",
        "se hartó, por fin, y se fue,",
        "y en cuanto se dio la vuelta...",
        "¡pum!, la esquina. ¡Y no lo ve!",
    ],
    &[
        "Tecleó tanto aquel hacker",
        "que se quedó en puro hueso;",
        "llegó la Muerte y le dijo:",
        "«¡Qué bien vas con el proceso!»",
    ],
    &[
        "Salió la Muerte en su bote",
        "por las orcas, a alta mar;",
        "ellas, bien organizadas,",
        "la mandaron a nadar:",
        "",
        "le mordieron el timón,",
        "la hicieron una bolita,",
        "y sin barca y sin guadaña",
        "volvió a nado, empapadita.",
    ],
    &[
        "Por la pantalla pasaba",
        "un tren con su chu-chu-chú;",
        "Gritó «¡Control-C!» la Muerte",
        "y el tren no dijo ni mu.",
        "",
        "Quedó la Flaca tiznada",
        "y, entre toses, se quejó:",
        "«¡Aquí quien arrolla a todos",
        "se supone que soy yo!»",
    ],
    &[
        "Fue la Muerte por el pato",
        "que nadaba en su charquito;",
        "él solo le dijo «cuac»",
        "y ella no entendió ni un pito.",
        "",
        "Tanto tiempo ahí perdió,",
        "tanto «cuac» y tanto rato,",
        "que, al volver tarde al Mictlán,",
        "ella fue quien pagó el pato.",
    ],
    &[
        "Viajó la Muerte hasta el polo",
        "por la aurora boreal;",
        "temblaba de hueso a hueso",
        "y pidió un chal y un tamal.",
        "",
        "Quiso meterla en un saco",
        "y la luz no se dejó;",
        "volvió sin nada al Mictlán",
        "¡y hasta un catarro pescó!",
    ],
    &[
        "Fue la Muerte a la pecera",
        "y a un pez le dijo: «Ya es hora»;",
        "mas él, tres segundos luego,",
        "le preguntó: «¿Y usted, señora?»",
        "",
        "Se lo explicó muchas veces,",
        "y el pez, con su cortesía,",
        "«¡Mucho gusto!», contestaba...",
        "y ahí siguen, todavía.",
    ],
    &[
        "Al programador ocioso",
        "que, en lugar de trabajar,",
        "ponía salvapantallas,",
        "la Muerte lo fue a buscar.",
        "",
        "«¡Ándale, vámonos!», dijo.",
        "«¡Ya voy, ya estoy compilando!",
        "Es Rust, ya casi termina...»",
        "La Flaca sigue esperando.",
    ],
    &[
        "Prendió la Flaca un cohete:",
        "se le atoró en el sombrero;",
        "salió disparada al cielo",
        "y la gente: «¡Qué lucero!»",
    ],
    &[
        "Fue por Neo la Calaca",
        "y él, doblándose hacia atrás,",
        "esquivó la guadañada",
        "como no se vio jamás.",
        "",
        "«No hay guadaña», dijo Neo,",
        "y la Muerte, confundida,",
        "tomó la pastilla azul",
        "y se durmió de por vida.",
    ],
    &[
        "Se metió la Muerte a un tubo",
        "buscando a quién sepultar,",
        "y giró, y giró, y giró",
        "sin la salida encontrar.",
        "",
        "Llenó toda la pantalla",
        "de una enorme tubería,",
        "y desde entonces la Flaca",
        "es experta en plomería.",
    ],
    &[
        "Quiso la Muerte ir al sol",
        "y se montó en un planeta;",
        "dio vueltas, vueltas y vueltas",
        "sin llegar nunca a la meta.",
        "",
        "Lleva cuatro mil millones",
        "de años girando en el cielo,",
        "y dice que ya mañana",
        "se baja... si encuentra el suelo.",
    ],
    &[
        "La Muerte, muerta de frío,",
        "se acercó hasta la fogata,",
        "se calentó los huesitos",
        "y se le quemó la bata.",
        "",
        "Quedó dorada y crujiente,",
        "y en la ofrenda, por error,",
        "la pusieron con el pan...",
        "¡y la sopeó un señor!",
    ],
    &[
        "Sin paraguas, la Huesuda",
        "salió bajo la tormenta;",
        "le cayó encima un relámpago",
        "y se quedó muy contenta:",
        "",
        "«¡Qué radiografía gratis!»,",
        "dijo, «y salí tan entera:",
        "ni una fractura en los huesos,",
        "aunque me veo muy güera».",
    ],
    &[
        "Vestida de halcón, la Flaca",
        "voló tras una parvada;",
        "las aves, al verla, en bola",
        "la dejaron despeinada.",
        "",
        "Lo raro es que la Pelona",
        "no ocupa ni un peluquero:",
        "volvió con un nido puesto",
        "¡y hoy lo luce de sombrero!",
    ],
    &[
        "Muerta de sueño, la Muerte",
        "vio una manta en alta mar",
        "y se la echó por encima:",
        "la manta se fue a nadar.",
        "",
        "Como la manta era viva,",
        "siguió nadando campante;",
        "la Flaca volvió a su casa",
        "empapada y tiritante.",
    ],
    &[
        "Vino la Muerte a mi casa",
        "a llevarse lo que hubiera;",
        "vio oxiper en la pantalla:",
        "se quedó la noche entera.",
        "",
        "Vio el bonsái, vio el tiburón,",
        "los peces y el tren que pita,",
        "y al alba, de la emoción,",
        "se olvidó de su visita.",
    ],
    &[
        "Tú, que miras la pantalla",
        "en lugar de trabajar:",
        "no te espantes, la Huesuda",
        "hoy te deja descansar.",
        "",
        "Ella es buena y muy paciente,",
        "te concede un rato más;",
        "pero el que viene, inclemente,",
        "¡es tu jefe, por detrás!",
    ],
];

const TITLE: &str = " ✿ Calaverita ✿ ";

/// Letters per second while typing, and the pauses after a comma, at the end
/// of a line and between stanzas.
const CPS: f32 = 24.0;
const PAUSE_COMMA: f32 = 0.18;
const PAUSE_LINE: f32 = 0.35;
const PAUSE_STANZA: f32 = 0.5;
/// Seconds the card takes to open and to crumble away.
const OPEN: f32 = 0.5;
const CRUMBLE: f32 = 2.5;
/// How long a letter glows after it lands.
const GLOW: f32 = 0.45;
/// The whole visit stays under this.
const MOST: f32 = 28.0;

fn start(_w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let verse = VERSES[rng.usize(..VERSES.len())];
    let typing: f32 = typing_secs(verse);
    let letters: usize = verse.iter().map(|l| l.chars().count()).sum();
    let hold = (5.0 + letters as f32 / 40.0).clamp(6.0, 10.0);
    // Faster typing if the visit would run long.
    let pace = (typing / (MOST - OPEN - hold - CRUMBLE)).max(1.0);
    Box::new(Card {
        t: 0.0,
        verse,
        pace,
        hold,
        typing: typing / pace,
        right: rng.bool(),
        seed: rng.u32(..),
    })
}

/// Seconds a verse takes to type at the normal pace.
fn typing_secs(verse: &[&str]) -> f32 {
    let mut t = 0.0;
    for line in verse {
        if line.is_empty() {
            t += PAUSE_STANZA;
            continue;
        }
        for ch in line.chars() {
            t += 1.0 / CPS + pause(ch);
        }
        t += PAUSE_LINE;
    }
    t
}

fn pause(ch: char) -> f32 {
    match ch {
        ',' | ';' | ':' => PAUSE_COMMA,
        '.' | '!' | '?' | '»' => PAUSE_COMMA * 0.6,
        _ => 0.0,
    }
}

/// The card's size in cells for a verse: the frame, a margin and the lines.
fn size(verse: &[&str], screen_w: i32) -> (i32, i32) {
    let widest = verse.iter().map(|l| l.chars().count()).max().unwrap_or(0) as i32;
    let title = TITLE.chars().count() as i32 + 4;
    // Two spaces of margin when the screen has room, else one.
    let pad = if widest + 4 + 2 + 2 <= screen_w { 2 } else { 1 };
    ((widest + 2 + 2 * pad).max(title), verse.len() as i32 + 2)
}

struct Card {
    t: f32,
    verse: &'static [&'static str],
    /// How much faster than normal it types.
    pace: f32,
    hold: f32,
    /// Seconds of typing at that pace.
    typing: f32,
    /// In the bottom-right corner (else bottom-left)?
    right: bool,
    seed: u32,
}

impl Card {
    /// How far it has crumbled, 0 to 1.
    fn crumbled(&self) -> f32 {
        ((self.t - OPEN - self.typing - self.hold) / CRUMBLE).clamp(0.0, 1.0)
    }

    /// A cell's own number in 0..1, for the order it crumbles in.
    fn grain(&self, x: i32, y: i32) -> f32 {
        let mut v = (x as u32)
            .wrapping_mul(0x9e37_79b9)
            .wrapping_add((y as u32).wrapping_mul(0x85eb_ca6b))
            ^ self.seed;
        v ^= v >> 15;
        v = v.wrapping_mul(0x2c1b_3c6d);
        v ^= v >> 12;
        (v & 0xffff) as f32 / 65536.0
    }
}

impl Egg for Card {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 5.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let pal = Palette::new(theme);
        let (sw, sh) = (c.w() as i32, c.h() as i32);
        let (cw, ch) = size(self.verse, sw);
        // In a bottom corner, a cell from the edges when there is room.
        let mx = i32::from(sw >= cw + 2);
        let my = i32::from(sh >= ch + 2);
        let x0 = if self.right { sw - cw - mx } else { mx };
        let y0 = sh - ch - my;
        // It opens upward from the bottom edge.
        let open = (self.t / OPEN).min(1.0);
        let shown = (ch as f32 * open).ceil() as i32;
        let crumble = self.crumbled();
        let (bg, frame, gold, ink) = (c.bg(), pal.of(b'R'), pal.of(b'O'), pal.of(b'W'));
        let put = |c: &mut Canvas, x: i32, y: i32, chr: char, fg: Color, attr: u8| {
            if y < y0 + ch - shown {
                return;
            }
            let g = self.grain(x, y);
            if crumble > 0.0 && g < crumble {
                return;
            }
            let fg = fade(theme, fg, (crumble / g.max(1e-3)).min(1.0) * 0.8);
            c.set_cell(
                x,
                y,
                Cell {
                    ch: chr,
                    fg,
                    bg,
                    attr,
                },
            );
        };
        // The frame, with the title on its top edge.
        let (x1, y1) = (x0 + cw - 1, y0 + ch - 1);
        for x in x0..=x1 {
            for (y, corner) in [(y0, ['╭', '╮']), (y1, ['╰', '╯'])] {
                let chr = match x {
                    _ if x == x0 => corner[0],
                    _ if x == x1 => corner[1],
                    _ => '─',
                };
                put(c, x, y, chr, frame, 0);
            }
        }
        for y in y0 + 1..y1 {
            put(c, x0, y, '│', frame, 0);
            put(c, x1, y, '│', frame, 0);
            for x in x0 + 1..x1 {
                put(c, x, y, ' ', ink, 0);
            }
        }
        let tx = x0 + (cw - TITLE.chars().count() as i32) / 2;
        for (i, chr) in TITLE.chars().enumerate() {
            let col = if chr == '✿' { gold } else { pal.of(b'Y') };
            put(c, tx + i as i32, y0, chr, col, BOLD);
        }
        // The verse, typed so far; a letter glows as it lands.
        let pad = (cw - 2 - self.width()) / 2;
        let mut clock = OPEN;
        let mut cursor = None;
        for (row, line) in self.verse.iter().enumerate() {
            let y = y0 + 1 + row as i32;
            if line.is_empty() {
                clock += PAUSE_STANZA / self.pace;
                continue;
            }
            for (col, chr) in line.chars().enumerate() {
                clock += 1.0 / CPS / self.pace;
                let x = x0 + 1 + pad + col as i32;
                if clock > self.t {
                    cursor.get_or_insert((x, y));
                    break;
                }
                let glow = ((self.t - clock) / GLOW).min(1.0);
                put(c, x, y, chr, gold.lerp(ink, glow), ITALIC);
                clock += pause(chr) / self.pace;
            }
            clock += PAUSE_LINE / self.pace;
            if cursor.is_some() {
                break;
            }
        }
        // The cursor blinks where it types, and after the last letter.
        let last = self.verse.last().map_or(0, |l| l.chars().count()) as i32;
        let (cx, cy) = cursor.unwrap_or((x0 + 1 + pad + last, y1 - 1));
        if crumble == 0.0 && (cursor.is_some() || (self.t * 2.0) as i32 % 2 == 0) {
            put(c, cx, cy, '▌', gold, 0);
        }
    }

    fn done(&self) -> bool {
        self.crumbled() >= 1.0 || self.t > 29.5
    }
}

impl Card {
    /// The widest line, in letters.
    fn width(&self) -> i32 {
        self.verse
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn there_are_many_verses_all_different() {
        assert!(VERSES.len() >= 20);
        for (i, a) in VERSES.iter().enumerate() {
            for b in &VERSES[i + 1..] {
                assert_ne!(a, b);
                assert_ne!(a[0], b[0], "two verses start alike");
            }
        }
    }

    #[test]
    fn verses_are_quatrains() {
        for v in VERSES {
            let stanzas: Vec<_> = v.split(|l| l.is_empty()).collect();
            assert!(stanzas.iter().all(|s| s.len() == 4), "{v:?}");
            assert!(!v.last().unwrap().is_empty());
        }
    }

    /// Every verse fits its card on a 40x12 screen, and comes out whole.
    #[test]
    fn verses_fit_their_card() {
        let theme = Theme::classic();
        for (w, h) in [(40u16, 12u16), (80, 24), (200, 60)] {
            for v in VERSES {
                let (cw, ch) = size(v, w as i32);
                assert!(cw <= w as i32 && ch <= h as i32, "{v:?} at {w}x{h}");
                let mut card = Card {
                    t: 0.0,
                    verse: v,
                    pace: 1.0,
                    hold: 8.0,
                    typing: typing_secs(v),
                    right: w == 80,
                    seed: 7,
                };
                card.t = OPEN + card.typing + 1.0;
                let mut c = Canvas::new(w, h, Color::Reset);
                card.draw(&mut c, &theme);
                let text = c.to_text();
                for line in v.iter() {
                    assert!(text.contains(line), "{line:?} missing at {w}x{h}:\n{text}");
                }
                assert!(text.contains("Calaverita"));
            }
        }
    }

    /// It types, holds and crumbles away within the time an egg has.
    #[test]
    fn types_holds_and_goes() {
        let theme = Theme::builtin("dracula").unwrap();
        for seed in 0..30 {
            let mut e = start(80, 24, &mut fastrand::Rng::with_seed(seed));
            let mut c = Canvas::new(80, 24, Color::Reset);
            let mut secs = 0.0;
            let mut most = 0;
            while !e.done() {
                e.update(0.1);
                secs += 0.1;
                c.clear();
                e.draw(&mut c, &theme);
                most = most.max(c.cells().iter().filter(|cell| cell.ch != ' ').count());
            }
            assert!(secs < 29.0, "{secs}");
            assert!(most > 150, "{most}");
            assert!(c.cells().iter().all(|cell| cell.ch == ' '));
        }
    }
}
