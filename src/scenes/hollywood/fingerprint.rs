//! A fingerprint of what hollywood shows on ordinary days and at the height
//! of its holidays, recorded from the code before presets and set mode, so
//! that work can prove it moved nothing. It hashes only what does not pass
//! through `libm` (CI runs on Linux and macOS): pane rectangles, kinds and
//! titles, the text of the panes that draw no curves, and the ticker.

use super::*;
use crate::theme::Theme;

/// FNV-1a, 64 bits.
struct Fnv(u64);

impl Fnv {
    fn new() -> Fnv {
        Fnv(0xcbf2_9ce4_8422_2325)
    }

    fn bytes(&mut self, b: &[u8]) {
        for &x in b {
            self.0 ^= x as u64;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn ch(&mut self, c: char) {
        let mut buf = [0u8; 4];
        self.bytes(c.encode_utf8(&mut buf).as_bytes());
    }
}

/// Kinds whose cells are hashed: the ordinary ones, but for the three that
/// draw with `sin` and `cos` (12, 13 and 15).
fn plain(kind: usize) -> bool {
    kind < KINDS && !matches!(kind, 12 | 13 | 15)
}

/// Two simulated minutes of hollywood at `w`x`h` on the day `fiesta`, with
/// the classic theme, the default options and seed 2026, hashed once a
/// second.
pub fn of(fiesta: Fiesta, w: u16, h: u16) -> u64 {
    let theme = Theme::classic();
    let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
    let ctx = Ctx {
        theme: &theme,
        opts: &o,
        bg: Color::Reset,
        seed: 2026,
        fiesta,
    };
    // As `Player::make` builds a scene.
    let mut s = Hollywood::new(&ctx);
    s.resize(&ctx, w, h);
    s.retheme(&ctx);
    let mut hash = Fnv::new();
    for i in 1..=3600 {
        s.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        if i % 30 != 0 {
            continue;
        }
        let mut c = Canvas::new(w, h, Color::Reset);
        s.draw(&ctx, &mut c);
        let Some(root) = &s.root else {
            continue;
        };
        root.each(&mut |p| {
            for v in [p.rect.x, p.rect.y, p.rect.w, p.rect.h] {
                hash.bytes(&v.to_le_bytes());
            }
            hash.bytes(&(p.kind as u64).to_le_bytes());
            hash.bytes(p.title.as_bytes());
        });
        root.each(&mut |p| {
            if !plain(p.kind) {
                return;
            }
            let r = p.rect;
            for y in r.y..r.y + r.h {
                for x in r.x..r.x + r.w {
                    if let Some(cell) = c.get(x, y) {
                        hash.ch(cell.ch);
                    }
                }
            }
        });
        for &ch in s.tape.iter().take(w as usize) {
            hash.ch(ch);
        }
    }
    hash.0
}

#[test]
fn ordinary_days_and_holidays_do_not_move() {
    let days = [
        Fiesta::NONE,
        Fiesta::height(Kind::Muertos),
        Fiesta::height(Kind::Navidad),
        Fiesta::height(Kind::Patrias),
        Fiesta::height(Kind::Revolucion),
    ];
    const WIDE: [u64; 5] = [
        786516498852993032,
        14955140875357974651,
        4210743927005387500,
        17277911737923954209,
        3168684557623075798,
    ];
    const TALL: [u64; 5] = [
        12235833525103617086,
        3851611810284997948,
        805669349719252904,
        3936998074935822539,
        17038652574698672151,
    ];
    for (i, f) in days.into_iter().enumerate() {
        assert_eq!(of(f, 140, 40), WIDE[i], "140x40 {f:?}");
        assert_eq!(of(f, 67, 60), TALL[i], "67x60 {f:?}");
    }
}

#[test]
fn the_fingerprint_is_its_own_twice() {
    assert_eq!(of(Fiesta::NONE, 140, 40), of(Fiesta::NONE, 140, 40));
}

#[test]
fn the_fingerprint_sees_a_holiday() {
    assert_ne!(
        of(Fiesta::NONE, 140, 40),
        of(Fiesta::height(Kind::Navidad), 140, 40)
    );
}
