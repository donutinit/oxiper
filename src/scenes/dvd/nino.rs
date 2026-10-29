//! The bouncing logo on Día del Niño: a trompo that spins, spinning faster
//! at a corner, in a shower of canicas and stars.

use super::{Air, BitKind, Fall, Look, React, Size, Tint};
use crate::color::{Color, rgb};

/// Toy colors, which the stripes of the trompo take in turn.
const JUGUETES: [Color; 5] = [
    rgb(0xe8322a), // rojo
    rgb(0xffd200), // amarillo
    rgb(0x1f7ae0), // azul
    rgb(0x2fb84a), // verde
    rgb(0xff8c00), // naranja
];
const MADERA: Color = rgb(0xc08a4a);
const CUERDA: Color = rgb(0xefe6cf);
const PUA: Color = rgb(0x9aa0aa);
const AMARILLO: Color = rgb(0xffd200);

/// The trompo, one char per pixel: `a` and `b` its painted stripes, `w` the
/// wood of its knob and its belt, `z` the cuerda wound on it and `m` the
/// metal of its púa. Four frames of each size, the stripes sliding around
/// it a quarter of a turn at a time, which is how it spins.
const TROMPO: [&str; 24] = [
    ".......www.......",
    ".......www.......",
    ".....aabaaab.....",
    "...baaabaaabba...",
    "..aaabbbaaaabba..",
    ".baaabbbaaaabbaa.",
    ".aaaabbbaaaaabba.",
    "baaaabbbaaaaabbaa",
    "wwwwwwwwwwwwwwwww",
    ".wwwwwwwwwwwwwww.",
    "..zzzzzzzzzzzzz..",
    "..zzzzzzzzzzzzz..",
    "...baabbaaabba...",
    "....aabbaaabb....",
    "....baabaaaba....",
    ".....aabaaab.....",
    ".....bzzzzza.....",
    "......zzzzz......",
    ".......aab.......",
    ".......aab.......",
    ".......baa.......",
    "........a........",
    ".......mmm.......",
    ".......mmm.......",
];
const TROMPO_2: [&str; 24] = [
    ".......www.......",
    ".......www.......",
    ".....abaaaba.....",
    "...aaabaaabaaa...",
    "..aabbbaaabbbaa..",
    ".aaabbbaaabbbaaa.",
    ".aabbbaaaaabbbaa.",
    "baabbbaaaaabbbaab",
    "wwwwwwwwwwwwwwwww",
    ".wwwwwwwwwwwwwww.",
    "..zzzzzzzzzzzzz..",
    "..zzzzzzzzzzzzz..",
    "...aabbaaabbaa...",
    "....abbaaabba....",
    "....aabaaabaa....",
    ".....abaaaba.....",
    ".....bzzzzzb.....",
    "......zzzzz......",
    ".......bab.......",
    ".......bab.......",
    ".......bab.......",
    "........a........",
    ".......mmm.......",
    ".......mmm.......",
];
const TROMPO_3: [&str; 24] = [
    ".......www.......",
    ".......www.......",
    ".....baabbaa.....",
    "...abbaabbaaab...",
    "..abbaaabbbbaaa..",
    ".aabbaaabbbbaaab.",
    ".abbaaaabbbbaaaa.",
    "aabbaaaabbbbaaaab",
    "wwwwwwwwwwwwwwwww",
    ".wwwwwwwwwwwwwww.",
    "..zzzzzzzzzzzzz..",
    "..zzzzzzzzzzzzz..",
    "...abbaabbbaab...",
    "....bbaabbbaa....",
    "....abaabbaab....",
    ".....baabbaa.....",
    ".....azzzzzb.....",
    "......zzzzz......",
    ".......bba.......",
    ".......bba.......",
    ".......abb.......",
    "........b........",
    ".......mmm.......",
    ".......mmm.......",
];
const TROMPO_4: [&str; 24] = [
    ".......www.......",
    ".......www.......",
    ".....aaabaaa.....",
    "...bbaaabaaabb...",
    "..baaaabbbaaaab..",
    ".bbaaaabbbaaaabb.",
    ".bbaaabbbbbaaabb.",
    "abbaaabbbbbaaabba",
    "wwwwwwwwwwwwwwwww",
    ".wwwwwwwwwwwwwww.",
    "..zzzzzzzzzzzzz..",
    "..zzzzzzzzzzzzz..",
    "...bbaabbbaabb...",
    "....baabbbaab....",
    "....bbaabaabb....",
    ".....baabaab.....",
    ".....azzzzza.....",
    "......zzzzz......",
    ".......aba.......",
    ".......aba.......",
    ".......aba.......",
    "........b........",
    ".......mmm.......",
    ".......mmm.......",
];
const TROMPITO: [&str; 16] = [
    "....www....",
    "....www....",
    "...abaaa...",
    ".baabaaaba.",
    ".wwwwwwwww.",
    "wwwwwwwwwww",
    ".zzzzzzzzz.",
    ".zzzzzzzzz.",
    "..babaaba..",
    "...abaab...",
    "...baaaa...",
    "....aaa....",
    "....baa....",
    ".....a.....",
    "....mmm....",
    "....mmm....",
];
const TROMPITO_2: [&str; 16] = [
    "....www....",
    "....www....",
    "...aaaaa...",
    ".aaaaaaaaa.",
    ".wwwwwwwww.",
    "wwwwwwwwwww",
    ".zzzzzzzzz.",
    ".zzzzzzzzz.",
    "..aababaa..",
    "...ababa...",
    "...ababa...",
    "....bab....",
    "....aaa....",
    ".....a.....",
    "....mmm....",
    "....mmm....",
];
const TROMPITO_3: [&str; 16] = [
    "....www....",
    "....www....",
    "...aabba...",
    ".abaabbaab.",
    ".wwwwwwwww.",
    "wwwwwwwwwww",
    ".zzzzzzzzz.",
    ".zzzzzzzzz.",
    "..ababbab..",
    "...babba...",
    "...aabab...",
    "....aba....",
    "....abb....",
    ".....b.....",
    "....mmm....",
    "....mmm....",
];
const TROMPITO_4: [&str; 16] = [
    "....www....",
    "....www....",
    "...aabaa...",
    ".bbaabaabb.",
    ".wwwwwwwww.",
    "wwwwwwwwwww",
    ".zzzzzzzzz.",
    ".zzzzzzzzz.",
    "..baabaab..",
    "...aabaa...",
    "...babab...",
    "....aba....",
    "....bbb....",
    ".....b.....",
    "....mmm....",
    "....mmm....",
];
const TROMPILLO: [&str; 10] = [
    "..www..", "..www..", ".wwwww.", "wwwwwww", ".aaaab.", "..aaa..", "..aab..", "...a...",
    "..mmm..", "..mmm..",
];
const TROMPILLO_2: [&str; 10] = [
    "..www..", "..www..", ".wwwww.", "wwwwwww", ".ababa.", "..bab..", "..aaa..", "...a...",
    "..mmm..", "..mmm..",
];
const TROMPILLO_3: [&str; 10] = [
    "..www..", "..www..", ".wwwww.", "wwwwwww", ".babaa.", "..aba..", "..bba..", "...b...",
    "..mmm..", "..mmm..",
];
const TROMPILLO_4: [&str; 10] = [
    "..www..", "..www..", ".wwwww.", "wwwwwww", ".babab.", "..aba..", "..bbb..", "...b...",
    "..mmm..", "..mmm..",
];
const SIZES: [Size; 3] = [
    Size {
        frames: &[&TROMPO, &TROMPO_2, &TROMPO_3, &TROMPO_4],
        react: &[],
        trim: None,
    },
    Size {
        frames: &[&TROMPITO, &TROMPITO_2, &TROMPITO_3, &TROMPITO_4],
        react: &[],
        trim: None,
    },
    Size {
        frames: &[&TROMPILLO, &TROMPILLO_2, &TROMPILLO_3, &TROMPILLO_4],
        react: &[],
        trim: None,
    },
];

/// What flies out of a toy box: canicas of every color, estrellas,
/// confeti and caramelos.
const JUGUETES_BITS: [BitKind; 6] = [
    BitKind::new(['●', '◉'], [Tint::Cycle(0), Tint::Cycle(0)], 6.0),
    BitKind::new(['●', '◉'], [Tint::Cycle(2), Tint::Cycle(2)], 6.0),
    BitKind::new(['●', '◉'], [Tint::Cycle(3), Tint::Cycle(3)], 6.0),
    BitKind::new(
        ['✦', '✧'],
        [Tint::Accent(2, AMARILLO), Tint::Accent(2, AMARILLO)],
        3.4,
    ),
    BitKind::new(['•', '·'], [Tint::Cycle(1), Tint::Cycle(4)], 3.0),
    BitKind::new(['♦', '◆'], [Tint::Cycle(4), Tint::Cycle(1)], 4.0),
];

/// Canicas, stars, confetti and caramelos in turn; mostly confetti when shed.
fn pick(r: &mut fastrand::Rng, thrown: bool) -> u8 {
    let n = r.u8(..20);
    if thrown {
        match n {
            0..=2 => 0,
            3..=5 => 1,
            6..=8 => 2,
            9..=12 => 3,
            13..=16 => 4,
            _ => 5,
        }
    } else {
        match n {
            0..=11 => 4,
            12..=15 => 3,
            _ => n % 3,
        }
    }
}

pub(super) const LOOK: Look = Look {
    sizes: &SIZES,
    parts: &[
        (b'a', Tint::Cycle(0)),
        (b'b', Tint::Cycle(1)),
        (b'w', Tint::Accent(5, MADERA)),
        (b'z', Tint::Ink(CUERDA)),
        (b'm', Tint::Dim(PUA)),
    ],
    back: b'w',
    solid: false,
    classic: &JUGUETES,
    banner: &["¡Feliz Día", "del Niño!"],
    react: React::Spin,
    // Eight frames a second: a spin that takes half a second a turn.
    beat: 8.0,
    fall: Fall {
        kinds: &JUGUETES_BITS,
        pick,
        cap: 240,
        lie: (10.0, 30.0),
        shed: (0.4, 0.8),
        burst: (0.3, 0.3),
        speed: (4.0, 26.0),
        ttl: ((4.0, 4.0), (6.0, 4.0)),
        air: Air {
            drag: 0.8,
            settle: 0.0,
            gravity: 18.0,
            sway: 0.0,
        },
    },
    joke: None,
};
