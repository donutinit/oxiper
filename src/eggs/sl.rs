//! The steam locomotive from `sl`, the joke you get when you mistype `ls`.
//!
//! Art and motion are ported from sl 5.03 (sl.c, sl.h):
//!
//!   Copyright 1993,1998,2014 Toyoda Masashi (mtoyoda@acm.org)
//!
//!   Everyone is permitted to do anything on this program including copying,
//!   modifying, and improving, unless you try to pretend that you wrote it.
//!   i.e., the above copyright notice has to appear in all copies.
//!   THE AUTHOR DISCLAIMS ANY RESPONSIBILITY WITH REGARD TO THIS SOFTWARE.
//!
//! sl advances one column every 40 ms; that pace is kept as columns per
//! second so it does not depend on our frame rate. The locomotive is picked
//! the way sl's options would: the D51 (default), the little one (`-l`) or the
//! C51 (`-c`), and now and then the `-a` accident with people shouting help.
//! Like sl, the train is opaque: the spaces in its art clear what is behind.

use super::{Egg, Info};

use crate::render::{Canvas, Cell};
use crate::theme::Theme;

pub const INFO: Info = Info { name: "sl", start };

fn start(w: u16, _h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    let kind = match rng.u8(0..100) {
        0..60 => Kind::D51,
        60..80 => Kind::Logo,
        _ => Kind::C51,
    };
    Box::new(Train::new(kind, rng.u8(0..100) < 12, w))
}

/// Columns per second: sl sleeps 40 ms between one-column steps.
const COLS_PER_SEC: f32 = 25.0;

const D51HEIGHT: i32 = 10;
const D51FUNNEL: i32 = 7;
const D51LENGTH: i32 = 83;
const LOGOHEIGHT: i32 = 6;
const LOGOFUNNEL: i32 = 4;
const LOGOLENGTH: i32 = 84;
const C51HEIGHT: i32 = 11;
const C51FUNNEL: i32 = 7;
const C51LENGTH: i32 = 87;
const PATTERNS: i32 = 6;

const D51STR1: &str = "      ====        ________                ___________ ";
const D51STR2: &str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
const D51STR3: &str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
const D51STR4: &str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
const D51STR5: &str = "  |      |  |   H  |__--------------------| [___] |   ";
const D51STR6: &str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
const D51STR7: &str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";
const D51WHL11: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL12: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL13: &str = "  \\_/      \\O=====O=====O=====O_/      \\_/            ";
const D51WHL21: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL22: &str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
const D51WHL23: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";
const D51WHL31: &str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
const D51WHL32: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL33: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";
const D51WHL41: &str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
const D51WHL42: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL43: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";
const D51WHL51: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL52: &str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
const D51WHL53: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";
const D51WHL61: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL62: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL63: &str = "  \\_/      \\_O=====O=====O=====O/      \\_/            ";
const D51DEL: &str = "                                                      ";
const COAL01: &str = "                              ";
const COAL02: &str = "                              ";
const COAL03: &str = "    _________________         ";
const COAL04: &str = "   _|                \\_____A  ";
const COAL05: &str = " =|                        |  ";
const COAL06: &str = " -|                        |  ";
const COAL07: &str = "__|________________________|_ ";
const COAL08: &str = "|__________________________|_ ";
const COAL09: &str = "   |_D__D__D_|  |_D__D__D_|   ";
const COAL10: &str = "    \\_/   \\_/    \\_/   \\_/    ";
const COALDEL: &str = "                              ";
const LOGO1: &str = "     ++      +------ ";
const LOGO2: &str = "     ||      |+-+ |  ";
const LOGO3: &str = "   /---------|| | |  ";
const LOGO4: &str = "  + ========  +-+ |  ";
const LWHL11: &str = " _|--O========O~\\-+  ";
const LWHL12: &str = "//// \\_/      \\_/    ";
const LWHL21: &str = " _|--/O========O\\-+  ";
const LWHL22: &str = "//// \\_/      \\_/    ";
const LWHL31: &str = " _|--/~O========O-+  ";
const LWHL32: &str = "//// \\_/      \\_/    ";
const LWHL41: &str = " _|--/~\\------/~\\-+  ";
const LWHL42: &str = "//// \\_O========O    ";
const LWHL51: &str = " _|--/~\\------/~\\-+  ";
const LWHL52: &str = "//// \\O========O/    ";
const LWHL61: &str = " _|--/~\\------/~\\-+  ";
const LWHL62: &str = "//// O========O_/    ";
const LCOAL1: &str = "____                 ";
const LCOAL2: &str = "|   \\@@@@@@@@@@@     ";
const LCOAL3: &str = "|    \\@@@@@@@@@@@@@_ ";
const LCOAL4: &str = "|                  | ";
const LCOAL5: &str = "|__________________| ";
const LCOAL6: &str = "   (O)       (O)     ";
const LCAR1: &str = "____________________ ";
const LCAR2: &str = "|  ___ ___ ___ ___ | ";
const LCAR3: &str = "|  |_| |_| |_| |_| | ";
const LCAR4: &str = "|__________________| ";
const LCAR5: &str = "|__________________| ";
const LCAR6: &str = "   (O)        (O)    ";
const DELLN: &str = "                     ";
const C51DEL: &str = "                                                       ";
const C51STR1: &str = "        ___                                            ";
const C51STR2: &str = "       _|_|_  _     __       __             ___________";
const C51STR3: &str = "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________|";
const C51STR4: &str = "     | `---'   |:: `--'  H  `--'         |  |___ ___|  ";
const C51STR5: &str = "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  ";
const C51STR6: &str = "    ||        | ::       H  +=====+      |  |::  ...|  ";
const C51STR7: &str = "|    | _______|_::-----------------[][]-----|       |  ";
const C51WH61: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH62: &str = "------'|oOo|==[]=-     ||      ||      |  ||=======_|__";
const C51WH63: &str = "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|     ";
const C51WH64: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";
const C51WH51: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH52: &str = "------'|oOo|===[]=-    ||      ||      |  ||=======_|__";
const C51WH53: &str = "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|     ";
const C51WH54: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";
const C51WH41: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH42: &str = "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__";
const C51WH43: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51WH44: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";
const C51WH31: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH32: &str = "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__";
const C51WH33: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51WH34: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";
const C51WH21: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH22: &str = "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__";
const C51WH23: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51WH24: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";
const C51WH11: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH12: &str = "------'|oOo|=[]=-      ||      ||      |  ||=======_|__";
const C51WH13: &str = "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|     ";
const C51WH14: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const D51: [[&str; 11]; 6] = [
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL11, D51WHL12,
        D51WHL13, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL21, D51WHL22,
        D51WHL23, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL31, D51WHL32,
        D51WHL33, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL41, D51WHL42,
        D51WHL43, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL51, D51WHL52,
        D51WHL53, D51DEL,
    ],
    [
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL61, D51WHL62,
        D51WHL63, D51DEL,
    ],
];
const D51COAL: [&str; 11] = [
    COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
];

const LOGO: [[&str; 7]; 6] = [
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
    [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN],
];
const LOGOCOAL: [&str; 7] = [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];
const LOGOCAR: [&str; 7] = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

const C51: [[&str; 12]; 6] = [
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH11, C51WH12, C51WH13,
        C51WH14, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH21, C51WH22, C51WH23,
        C51WH24, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH31, C51WH32, C51WH33,
        C51WH34, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH41, C51WH42, C51WH43,
        C51WH44, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH51, C51WH52, C51WH53,
        C51WH54, C51DEL,
    ],
    [
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH61, C51WH62, C51WH63,
        C51WH64, C51DEL,
    ],
];
const C51COAL: [&str; 12] = [
    COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10,
    COALDEL,
];

/// Smoke puffs: for each kind, the shape at each of the 16 stages of a puff's
/// life, the eraser shape sl used (not needed here, we redraw every frame),
/// and how far the puff moves at each stage.
const SMOKE: [[&str; 16]; 2] = [
    [
        "(   )", "(    )", "(    )", "(   )", "(  )", "(  )", "( )", "( )", "()", "()", "O", "O",
        "O", "O", "O", " ",
    ],
    [
        "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)", "(@@)", "(@)", "(@)", "@@", "@@", "@", "@",
        "@", "@", "@", " ",
    ],
];
const SMOKE_DY: [i32; 16] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const SMOKE_DX: [i32; 16] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    D51,
    Logo,
    C51,
}

impl Kind {
    fn height(self) -> i32 {
        match self {
            Kind::D51 => D51HEIGHT,
            Kind::Logo => LOGOHEIGHT,
            Kind::C51 => C51HEIGHT,
        }
    }
    fn length(self) -> i32 {
        match self {
            Kind::D51 => D51LENGTH,
            Kind::Logo => LOGOLENGTH,
            Kind::C51 => C51LENGTH,
        }
    }
    fn funnel(self) -> i32 {
        match self {
            Kind::D51 => D51FUNNEL,
            Kind::Logo => LOGOFUNNEL,
            Kind::C51 => C51FUNNEL,
        }
    }
    /// Row of the train's top relative to the screen's middle, as in sl.c.
    fn top(self, rows: i32) -> i32 {
        rows / 2 - if self == Kind::Logo { 3 } else { 5 }
    }
}

/// One puff of smoke; `y` is relative to the top of the train, so the train
/// needs no screen size to move its smoke.
struct Puff {
    y: i32,
    x: i32,
    ptrn: usize,
    kind: usize,
}

struct Train {
    kind: Kind,
    accident: bool,
    /// Column of the train's left edge (sl's `x`), starting at the right edge.
    col: i32,
    /// Fraction of a column travelled but not yet stepped.
    acc: f32,
    smoke: Vec<Puff>,
    /// How many puffs were ever made; sl alternates their kind with it.
    sum: usize,
}

impl Train {
    fn new(kind: Kind, accident: bool, cols: u16) -> Train {
        let mut t = Train {
            kind,
            accident,
            col: cols as i32 - 1,
            acc: 0.0,
            smoke: Vec::new(),
            sum: 0,
        };
        t.add_smoke();
        t
    }

    /// sl's `add_smoke`: every fourth column of the funnel, all puffs age and
    /// drift, and a new one leaves the funnel.
    fn add_smoke(&mut self) {
        let x = self.col + self.kind.funnel();
        if x % 4 != 0 {
            return;
        }
        for p in &mut self.smoke {
            p.y -= SMOKE_DY[p.ptrn];
            p.x += SMOKE_DX[p.ptrn];
            p.ptrn = (p.ptrn + 1).min(15);
        }
        self.smoke.push(Puff {
            y: -1,
            x,
            ptrn: 0,
            kind: self.sum % 2,
        });
        self.sum += 1;
    }
}

impl Egg for Train {
    fn update(&mut self, dt: f32) {
        self.acc += dt * COLS_PER_SEC;
        while self.acc >= 1.0 && !self.done() {
            self.acc -= 1.0;
            self.col -= 1;
            if !self.done() {
                self.add_smoke();
            }
        }
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        let fg = theme.fg;
        let bg = c.bg();
        let (kind, x) = (self.kind, self.col);
        let y = kind.top(c.h() as i32);
        // sl's `my_mvaddstr`, opaque: spaces clear what is behind the train.
        let put = |c: &mut Canvas, y: i32, x: i32, s: &str| {
            for (i, ch) in s.chars().enumerate() {
                let cell = Cell {
                    ch,
                    fg,
                    bg,
                    attr: 0,
                };
                c.set_cell(x + i as i32, y, cell);
            }
        };
        let wheel = (kind.length() + x).rem_euclid(PATTERNS) as usize;
        for i in 0..=kind.height() as usize {
            let row = y + i as i32;
            match kind {
                Kind::D51 => {
                    put(c, row, x, D51[wheel][i]);
                    put(c, row, x + 53, D51COAL[i]);
                }
                Kind::Logo => {
                    put(c, row, x, LOGO[wheel][i]);
                    put(c, row, x + 21, LOGOCOAL[i]);
                    put(c, row, x + 42, LOGOCAR[i]);
                    put(c, row, x + 63, LOGOCAR[i]);
                }
                Kind::C51 => {
                    put(c, row, x, C51[wheel][i]);
                    put(c, row, x + 55, C51COAL[i]);
                }
            }
        }
        if self.accident {
            // sl's `add_man`: people on the train, waving and shouting.
            let spots: &[(i32, i32)] = match kind {
                Kind::D51 => &[(2, 43), (2, 47)],
                Kind::C51 => &[(3, 45), (3, 49)],
                Kind::Logo => &[(1, 14), (1, 45), (1, 53), (1, 66), (1, 74)],
            };
            for &(dy, dx) in spots {
                let man =
                    [["", "(O)"], ["Help!", "\\O/"]][((LOGOLENGTH + x + dx) / 12 % 2) as usize];
                for (i, s) in man.iter().enumerate() {
                    put(c, y + dy + i as i32, x + dx, s);
                }
            }
        }
        for p in &self.smoke {
            put(c, y + p.y, p.x, SMOKE[p.kind][p.ptrn]);
        }
    }

    fn done(&self) -> bool {
        self.col < -self.kind.length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    fn width(rows: &[&str]) -> usize {
        let w = rows[0].chars().count();
        assert!(rows.iter().all(|r| r.chars().count() == w), "{rows:?}");
        w
    }

    #[test]
    fn art_matches_sl_dimensions() {
        for f in &D51 {
            assert_eq!(f.len() as i32, D51HEIGHT + 1);
            assert_eq!(width(f), 54);
        }
        assert_eq!(width(&D51COAL), 30);
        assert_eq!(53 + 30, D51LENGTH);
        for f in &LOGO {
            assert_eq!(f.len() as i32, LOGOHEIGHT + 1);
            assert_eq!(width(f), 21);
        }
        assert_eq!(width(&LOGOCOAL), 21);
        assert_eq!(width(&LOGOCAR), 21);
        assert_eq!(63 + 21, LOGOLENGTH);
        for f in &C51 {
            assert_eq!(f.len() as i32, C51HEIGHT + 1);
            assert_eq!(width(f), 55);
        }
        assert_eq!(width(&C51COAL), 30);
    }

    #[test]
    fn crosses_at_sls_pace_and_finishes() {
        let mut t = Train::new(Kind::D51, false, 80);
        let mut secs = 0.0f32;
        while !t.done() {
            t.update(1.0 / 60.0);
            secs += 1.0 / 60.0;
        }
        // (80 + 83) columns at 25 per second.
        assert!((secs - 163.0f32 / 25.0).abs() < 0.1, "{secs}");
    }

    #[test]
    fn smoke_rises_and_draws() {
        let mut t = Train::new(Kind::D51, true, 120);
        let theme = Theme::builtin("nord").unwrap();
        let mut c = Canvas::new(120, 30, Color::Reset);
        for _ in 0..200 {
            t.update(1.0 / 30.0);
        }
        t.draw(&mut c, &theme);
        let text = c.to_text();
        assert!(text.contains("(@@@") || text.contains("(   )") || text.contains('O'));
        assert!(text.contains("____"));
        assert!(!t.smoke.is_empty());
    }
}
