//! On the twenty-third of December the Noche de Rábanos fills the zócalo of
//! Oaxaca: a row of displays carved from radishes, each with its little sign,
//! people looking, and a judge who pins a ribbon on the winner.
//!
//! For over a century Oaxaca has spent the night of the twenty-third admiring
//! radishes. The growers leave them in the ground long past their season until
//! they swell into great red roots, and carve them into nacimientos, calendas,
//! mariachis, dancers of the Guelaguetza and scenes of Oaxacan life, the white
//! flesh showing under the red skin and the leaves left on for greenery;
//! flowers of siempreviva (everlasting) and figures of totomoxtle (corn husk)
//! compete in their own categories, and a jury walks the tables and gives the
//! prizes. This draws it in original oxiper pixel art that grows with the
//! screen. Up to four mesas (fewer on a narrow screen) rise from the ground one
//! after another, each with its carved scene: a nacimiento under a thatch of
//! husk with siempreviva along its eaves; a calenda, the great globe of a
//! marmota on its pole between a woman with a canasta of flowers on her head and
//! a man in a sombrero; three mariachis in white under an arch of leaves; a
//! dancer of the Danza de la Pluma under the round penacho of feathers, his
//! shield on one arm and his palma in the other hand. On the cloth in front of
//! each is a little sign with its name. People come to look, one after another,
//! the children among them, in rebozos and sombreros: they stand in the gaps
//! between the mesas and turn to the displays, a hand to the chin, an arm out to
//! point, a phone up with its flash. The judge walks along the row, stops at
//! each display and bends to it with his clipboard, goes back to the one he
//! liked best and pins a ribbon of gold on its sign. Everybody applauds, the
//! display glints and "¡Primer lugar!" rises over it; then the people drift
//! away and the mesas sink back into the ground. All within 30 s.

use super::guadalupe::{Place, fit_down};
use super::palette::{Palette, Role, fade};
use super::{Egg, Info};
use crate::fiesta::Kind;
use crate::render::{BOLD, Canvas, Cell};
use crate::theme::Theme;
use std::f32::consts::{PI, TAU};

pub const INFO: Info = Info {
    name: "rabanos",
    start,
    fiesta: Kind::Navidad,
    when: |fiesta| fiesta.rabanos(),
};

/// When things happen, in seconds: the first mesa rises and the others follow
/// `MESA_GAP` apart, each taking `MESA_SECS`; the first watcher comes on and
/// the others follow `WATCHERS_GAP` apart; the judge comes on, stays `DWELL`
/// at each display and is at the winner's by `ARRIVES`, waits `WAIT` and pins;
/// the applause lasts `APPLAUSE`; then the people leave, the mesas sink one
/// after another (`SINK_GAP` apart, `SINK_SECS` each) and it is over.
const MESA_FIRST: f32 = 0.2;
const MESA_GAP: f32 = 0.5;
const MESA_SECS: f32 = 1.6;
const WATCHERS_FIRST: f32 = 1.0;
const WATCHERS_GAP: f32 = 0.9;
const JUDGE_COMES: f32 = 2.4;
const DWELL: f32 = 1.4;
const ARRIVES: f32 = 15.0;
const WAIT: f32 = 0.9;
const APPLAUSE: f32 = 4.0;
const SINK_GAP: f32 = 0.25;
const SINK_SECS: f32 = 1.8;
/// The title comes up after the first mesas.
const TITLE_FROM: f32 = 2.6;
/// How long "¡Primer lugar!" stays over the winner.
const PRIZE: f32 = 3.6;

/// How tall the stage is, in art pixels (the tallest display), and how many
/// displays there are.
const HEIGHT: i32 = 42;
const DISPLAYS: usize = 4;
/// A display's art: its size in art pixels. The mesa under it is a little
/// wider and `MESA_H` tall; it stands `DEPTH` behind the people.
const ART_W: usize = 20;
const ART_H: usize = 24;
const MESA_W: f32 = 22.0;
const MESA_H: f32 = 11.0;
const DEPTH: f32 = 4.0;
/// How far from the edge of the screen people stand at the nearest, and how
/// far back from the gap the judge stands (in art pixels).
const EDGE: f32 = 5.0;
const JUDGE_BACK: f32 = 3.5;
/// How far beyond the screen people come from and go to, in art pixels, and
/// how far a walker goes in one step (two strides).
const MARGIN: f32 = 14.0;
const STRIDE: f32 = 14.0;
/// How far below the ground a sunken mesa is, in art pixels.
const SUNK: f32 = 44.0;
/// The most people looking, and how big a person is next to what they
/// look at.
const WATCHERS: usize = 6;
const PERSON: f32 = 0.66;

/// Letter, color under `classic`, and role under other themes. Letters that a
/// `Shade` refers to come before it.
const COLORS: &[(u8, u32, Role)] = &[
    // The radish: its red skin, in shade and lit; the white flesh under it, in
    // shade, and the pink where skin and flesh meet.
    (b'R', 0xd1263b, Role::Hue),
    (b'r', 0x8a1928, Role::Shade(b'R', 0.66)),
    (b'q', 0xe8606f, Role::Shade(b'R', 1.3)),
    (b'W', 0xf7f0ef, Role::Bone(1.0)),
    (b'w', 0xc4b8b8, Role::Bone(0.75)),
    (b'p', 0xf0c4c4, Role::Bone(0.9)),
    // The leaves, in shade and light.
    (b'G', 0x4caf50, Role::Hue),
    (b'g', 0x2e7031, Role::Shade(b'G', 0.65)),
    (b'l', 0x8fd392, Role::Shade(b'G', 1.35)),
    // Siempreviva, the everlasting flowers: morado, in shade and light.
    (b'M', 0x8e44ad, Role::Hue),
    (b'm', 0x5c2c71, Role::Shade(b'M', 0.65)),
    (b'I', 0xb67fcf, Role::Shade(b'M', 1.45)),
    // Totomoxtle, the corn husk: its pale, its shade and its light.
    (b'H', 0xd9c48a, Role::Hue),
    (b'h', 0xb08d4a, Role::Shade(b'H', 0.8)),
    (b'E', 0xece0b0, Role::Shade(b'H', 1.12)),
    // Wood, and its dark.
    (b'D', 0xa0703a, Role::Shade(b'H', 0.75)),
    (b'd', 0x6a4520, Role::Shade(b'H', 0.5)),
    // Skin and its shade; hair and the dark inks.
    (b'K', 0xd7a066, Role::Hue),
    (b'k', 0xa87040, Role::Shade(b'K', 0.78)),
    (b'N', 0x2a1a14, Role::Ink(0.2)),
    (b'n', 0x1c1424, Role::Ink(0.06)),
    (b'z', 0x4a3c5e, Role::Ink(0.3)),
    // Manta (cotton) and its shade, and the paper of a sign.
    (b'A', 0xefe6cf, Role::Bone(0.95)),
    (b'a', 0xc9bfa6, Role::Bone(0.72)),
    (b'S', 0xf4ecd8, Role::Bone(0.97)),
    // Rebozos, tablecloth bands and ribbons: azul, rosa mexicano, turquesa and
    // naranja, and the dark of the azul.
    (b'B', 0x2a5caa, Role::Hue),
    (b'b', 0x1c3f78, Role::Shade(b'B', 0.65)),
    (b'P', 0xe4007c, Role::Hue),
    (b'T', 0x00aab8, Role::Hue),
    (b'O', 0xff8c1a, Role::Hue),
    // Gold, for a ribbon and the glint on a prize, and its shade.
    (b'Y', 0xf2c14e, Role::Hue),
    (b'y', 0xc08c20, Role::Shade(b'Y', 0.75)),
];

impl Palette {
    /// The Noche de Rábanos visitor's colors under a theme.
    pub fn rabanos(theme: &Theme) -> Palette {
        Palette::with(theme, COLORS)
    }
}

/// A nacimiento under a thatch of husk, siempreviva along its eaves: the
/// Virgen kneeling in her mantle, the Niño in his manger, José with his cloak.
const NACIMIENTO: &[&str] = &[
    ".........WW.........",
    "........WWWW........",
    ".........WW.........",
    "....................",
    ".........hh.........",
    "........hEEh........",
    ".......hEHHEh.......",
    "......hEHEEHEh......",
    ".....hEHEHHEHEh.....",
    "....hEHEHEEHEHEh....",
    "...hEHEHEHHEHEHEh...",
    "..hhMhhMhhMhhMhhhh..",
    "...D............D...",
    "...D.RR......WW.D...",
    "...DRppR.....pp.D...",
    "...D.RR.....WWWRD...",
    "...DRRRR....WWWRD...",
    "...DRpWR....WWWRD...",
    "...DRWWR.pp.WWRRD...",
    "...DRWWRHWWHWWRRD...",
    "...DRRRRHEEHWWWWD...",
    "...D.RR.hhhh.WW.D...",
    "GgGgGMGgGlGgGMGgGlGg",
    "gGgGgGgGgGgGgGgGgGgG",
];
/// A calenda: the marmota, a great globe of red and white lozenges with
/// leaves on top and streamers, on its pole, between a woman with a canasta of
/// flowers on her head and a man in a sombrero.
const CALENDA: &[&str] = &[
    "........GGGG........",
    ".........GG.........",
    "........qWRw........",
    "......WRWRWRWr......",
    ".....qWRWRWRWRw.....",
    "....GWRWRWRWRWrG....",
    "...l.qWRWRWRWRw.l...",
    "....GWRWRWRWRWrG....",
    "...l.qWRWRWRWRw.l...",
    "..M.RWMWRWRWRWhEh...",
    "..HMHRHWRWRWhhhhhhh.",
    ".HEHEHEHWRWr..ppp...",
    "..hhhhh..Dd...ppp...",
    "...rrr...Dd..WWWWW..",
    "...ppp...Dd.WWWWWW..",
    "...ppp...Dd..WWRWW..",
    "..WWWWW..Dd..WWRWW..",
    ".RWWWWWR.Dd..WWWWW..",
    "..WWRWW..Dd..RRRRR..",
    "...WWW...Dd..WW.WW..",
    "..RWRWR..Dd..WW.WW..",
    ".RWRWRWR.Dd..WW.WW..",
    ".RWRWRWR.Dd..WW.WW..",
    ".RRRRRRR.Dd..rr.rr..",
];
/// Three mariachis in white with red sombreros under an arch of leaves and
/// siempreviva: a trumpet, a guitarrón and a violin.
const MARIACHI: &[&str] = &[
    "....................",
    "....................",
    "....................",
    ".........M..........",
    ".....M.ggGggGM......",
    "....ggGGlGlGlggG.M..",
    ".MgGlGl......GlGgg..",
    ".glG............lGG.",
    "GG................lg",
    "l..................G",
    "...RR....RR....RR...",
    "..RWWR..RWWR..RWWR..",
    ".rrrrrrrrrrrrrrrrrr.",
    "...pp....pp....pp...",
    "...pEEEE.ph....ph...",
    "..WWWW..WWhW..WhWW..",
    ".WRWWRWWHHHRWWRWEEW.",
    ".WWWWWWWHhHWWWWWEEW.",
    "..WRRW..HHHW..WRRW..",
    "..WWWW..WWWW..WWWW..",
    "..WW.WW.WW.WW.WW.WW.",
    "..WW.WW.WW.WW.WW.WW.",
    "..WW.WW.WW.WW.WW.WW.",
    "..rr.rr.rr.rr.rr.rr.",
];
/// A dancer of the Danza de la Pluma: the round penacho of feathers in red,
/// white, green and morado round her face, the shield on one arm and the
/// palma in the other hand, a skirt of red and white, a wide stance.
const PLUMA: &[&str] = &[
    ".......GGG..........",
    ".......GGGMM.RR.....",
    "...R.WWWGGMMMRRR....",
    "..RRRWWWGGMMRRRR....",
    "..RRRRWWHHHMRRWW....",
    "...MRRHHrrrHHWWW....",
    "..MMMMHrhhhhHWWWGG..",
    "..MMMHrrpppprHGGGG..",
    ".GGGGHrrpppprHGGGG..",
    ".GGGGHrrrpprrHMMMG.G",
    ".GGWWWHr...rHMMMMlGl",
    "..WWWWRWWWWWWRRMM.G.",
    "...WppRWHHHHWRppR.D.",
    ".RR.ppRWHRRHWWppR.D.",
    "RWWRRR.WWWWWWWRR..D.",
    "RWWRRRRWRWRWRW....D.",
    ".RR...RWRWRWRW....D.",
    "......WRWRWRWR....D.",
    "......RRRRRRRR....D.",
    ".......WW..WW.....D.",
    ".......WW..WW.....D.",
    "......WW....WW....D.",
    "......WW....WW....D.",
    ".....rrr....rrr...D.",
];
/// The displays, in the order they stand, their signs (the longest that fits
/// the cloth first) and the color of the band on each mesa's cloth.
const ARTS: [&[&str]; DISPLAYS] = [NACIMIENTO, CALENDA, MARIACHI, PLUMA];
const NAMES: [&[&str]; DISPLAYS] = [
    &["Nacimiento", "Belén"],
    &["Calenda", "Marmota"],
    &["Mariachi"],
    &["Danza de la Pluma", "La Pluma", "Pluma"],
];
const BANDS: [u8; DISPLAYS] = *b"PBTO";

/// What the Noche de Rábanos shouts and writes.
const TITLE: &str = "Noche de Rábanos";
const PRIMER: &str = "¡Primer lugar!";
const BRAVO: &str = "¡Bravo!";

fn start(w: u16, h: u16, rng: &mut fastrand::Rng) -> Box<dyn Egg> {
    Box::new(Rabanos::new(w, h, rng))
}

/// What a person is doing at a stop.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    /// Standing at a display, looking.
    Watch,
    /// The judge, bent to a display.
    Examine,
    /// The judge, with the ribbon.
    Pin,
}

/// Somewhere a person stops, and for how long.
#[derive(Clone, Copy)]
struct Stop {
    x: f32,
    face: f32,
    arrive: f32,
    leave: f32,
    mode: Mode,
}

/// Where a person is at some moment.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Spot {
    x: f32,
    /// Which way they face: where they go while they walk, the display while
    /// they stand.
    face: f32,
    walking: bool,
    mode: Mode,
    /// How long they have been standing here.
    since: f32,
}

/// A person's way across the stage: the stops, and the pace between them.
#[derive(Clone, Copy)]
struct Path {
    stops: [Stop; 8],
    n: usize,
    pace: f32,
}

impl Path {
    /// A path that starts at `x` (off the screen) at second `at`.
    fn new(x: f32, at: f32, pace: f32) -> Path {
        let start = Stop {
            x,
            face: 1.0,
            arrive: at,
            leave: at,
            mode: Mode::Watch,
        };
        Path {
            stops: [start; 8],
            n: 1,
            pace,
        }
    }

    /// Walks on to `x` and stays there `stay` seconds, facing `face`.
    fn push(&mut self, x: f32, face: f32, stay: f32, mode: Mode) {
        let last = self.stops[self.n - 1];
        let arrive = last.leave + (x - last.x).abs() / self.pace.max(1.0);
        self.stops[self.n.min(7)] = Stop {
            x,
            face,
            arrive,
            leave: arrive + stay,
            mode,
        };
        self.n = (self.n + 1).min(8);
    }

    /// The second they have gone.
    fn gone(&self) -> f32 {
        self.stops[self.n - 1].arrive
    }

    /// Where they are at second `t`; `None` before they come on and after they
    /// have gone.
    fn at(&self, t: f32) -> Option<Spot> {
        if t < self.stops[0].leave || t >= self.gone() {
            return None;
        }
        for i in 0..self.n {
            let s = self.stops[i];
            if t <= s.leave {
                return Some(Spot {
                    x: s.x,
                    face: s.face,
                    walking: false,
                    mode: s.mode,
                    since: (t - s.arrive).max(0.0),
                });
            }
            let next = self.stops[(i + 1).min(self.n - 1)];
            if t < next.arrive {
                let u = (t - s.leave) / (next.arrive - s.leave).max(1e-3);
                return Some(Spot {
                    x: s.x + (next.x - s.x) * u,
                    face: if next.x >= s.x { 1.0 } else { -1.0 },
                    walking: true,
                    mode: Mode::Watch,
                    since: 0.0,
                });
            }
        }
        None
    }
}

/// Who a person is: a woman or a man, a child, the judge, and a number that
/// picks the colors of their clothes.
#[derive(Clone, Copy)]
struct Who {
    woman: bool,
    child: bool,
    judge: bool,
    look: u32,
}

impl Who {
    /// The shirt or blouse.
    fn top(&self) -> u8 {
        if self.judge {
            b'z'
        } else if self.woman {
            b'A'
        } else {
            (*b"ABTA")[(self.look >> 2) as usize % 4]
        }
    }

    fn rebozo(&self) -> u8 {
        (*b"BPTO")[(self.look >> 5) as usize % 4]
    }

    fn skirt(&self) -> u8 {
        (*b"zbmr")[(self.look >> 8) as usize % 4]
    }

    fn trousers(&self) -> u8 {
        if self.judge {
            b'n'
        } else {
            (*b"zbdm")[(self.look >> 11) as usize % 4]
        }
    }
}

/// What a person does with their arms.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Arms {
    Down,
    /// A hand at the chin, looking closely.
    Chin,
    /// One arm out toward a display, a little up or down.
    Point(f32),
    /// A phone up, the flash lit or not.
    Photo(bool),
    /// Both arms up, clapping.
    Cheer(f32),
    /// The judge's clipboard.
    Clip,
    /// The judge's arm out to the sign, `reach` away, holding the ribbon or not.
    Pin(f32, bool),
}

struct Pose {
    /// The step: radians into the stride.
    phase: f32,
    walking: bool,
    arms: Arms,
    /// How far forward the body leans, and how far it bobs.
    lean: f32,
    bob: f32,
}

#[derive(Clone, Copy)]
struct Watcher {
    who: Who,
    path: Path,
}

struct Rabanos {
    t: f32,
    /// The screen's size in cells.
    w: i32,
    h: i32,
    /// Screen pixels to an art pixel.
    k: f32,
    /// How many displays stand in the row, and which of the four is the first.
    shown: usize,
    first: usize,
    /// The row's left edge, and the distance between two displays.
    x0: f32,
    pitch: f32,
    seed: u32,
    /// Which display wins.
    winner: usize,
    /// The judge's way, and when he pins the ribbon.
    judge: Path,
    pin: f32,
    watchers: [Watcher; WATCHERS],
    /// How many of them there are on this screen.
    n: usize,
    /// When the mesas start to sink and when it is over.
    sink: f32,
    end: f32,
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

impl Rabanos {
    fn new(w: u16, h: u16, rng: &mut fastrand::Rng) -> Rabanos {
        let mut rng = fastrand::Rng::with_seed(rng.u64(..));
        // As many displays as the screen has room for (all four from a hundred
        // columns), a run of the four when it has fewer.
        let shown = (usize::from(w) / 25).clamp(2, DISPLAYS);
        let first = rng.usize(..=DISPLAYS - shown);
        let k = fit_down(h, w, HEIGHT, 25.0 * shown as f32);
        let wide = w as f32 / k;
        let pitch = (wide / shown as f32).clamp(25.0, 36.0);
        let x0 = (wide - pitch * shown as f32) / 2.0;
        let row = pitch * shown as f32;
        // Where people come on and go off: just off the screen, or, on a very
        // wide one, not far from the row.
        let (west, east) = (
            (x0 - 70.0).max(-MARGIN),
            (x0 + row + 70.0).min(wide + MARGIN),
        );
        let mid = (west + east) / 2.0;
        let walk = ((east - west) / 6.0).clamp(16.0, 45.0);
        let (winner, dir) = (rng.usize(..shown), if rng.bool() { 1.0 } else { -1.0 });

        // The judge stands at the gap before each display (after it, going the
        // other way), a little back from the display so that he stands beside
        // the people in the gap, and walks the row once and back to the winner.
        let spot = |i: usize| {
            let gap = x0 + pitch * (if dir > 0.0 { i } else { i + 1 }) as f32;
            (gap - dir * JUDGE_BACK).min(wide - EDGE).max(EDGE)
        };
        let order = |j: usize| if dir > 0.0 { j } else { shown - 1 - j };
        let start = if dir > 0.0 { west } else { east };
        let (from, to) = (spot(order(0)), spot(order(shown - 1)));
        let road = (from - start).abs() + (to - from).abs() + (spot(winner) - to).abs();
        let budget = ARRIVES - JUDGE_COMES - shown as f32 * DWELL;
        let pace = (road / budget.max(1.0)).clamp(14.0, 45.0);
        let mut judge = Path::new(start, JUDGE_COMES, pace);
        for j in 0..shown {
            judge.push(spot(order(j)), dir, DWELL, Mode::Examine);
        }
        judge.push(spot(winner), dir, WAIT + APPLAUSE + 0.4, Mode::Pin);
        let pin = judge.stops[judge.n - 1].arrive + WAIT;
        let leaves = if spot(winner) < mid { west } else { east };
        judge.push(leaves, 1.0, 0.0, Mode::Watch);
        let mut sink = judge.gone();

        // The people who look: each in a gap between the mesas, turned to a
        // display, coming from the nearer side.
        let n = (shown + usize::from(wide >= 130.0) + usize::from(wide >= 170.0)).min(WATCHERS);
        let mut gaps = [0, 1, 2, 3, 4];
        let gaps = &mut gaps[..=shown];
        for i in (1..gaps.len()).rev() {
            gaps.swap(i, rng.usize(..=i));
        }
        // The gap where the judge pins the ribbon is the last to be taken.
        let pins = if dir > 0.0 { winner } else { winner + 1 };
        if let Some(at) = gaps.iter().position(|&g| g == pins) {
            gaps.swap(at, shown);
        }
        const WHO: [(bool, bool); WATCHERS] = [
            (true, false),
            (false, false),
            (true, true),
            (false, false),
            (false, true),
            (true, false),
        ];
        let mut watchers = [Watcher {
            who: Who {
                woman: false,
                child: false,
                judge: false,
                look: 0,
            },
            path: Path::new(0.0, 0.0, 1.0),
        }; WATCHERS];
        for (i, watcher) in watchers.iter_mut().enumerate().take(n) {
            let gap = gaps[i % gaps.len()];
            let x = x0 + pitch * gap as f32 + (i / gaps.len()) as f32 * 5.0 + rng.f32() * 2.0 - 1.0;
            let x = x.min(wide - EDGE).max(EDGE);
            let face = if gap == 0 {
                1.0
            } else if gap == shown || rng.bool() {
                -1.0
            } else {
                1.0
            };
            let (near, enter) = (
                if x < mid { west } else { east },
                WATCHERS_FIRST + WATCHERS_GAP * i as f32,
            );
            let mut path = Path::new(near, enter, walk);
            // They stay until the applause is over, one after another.
            let stay = pin + APPLAUSE + 0.6 + 0.3 * i as f32 - enter - (x - near).abs() / walk;
            path.push(x, face, stay.max(0.0), Mode::Watch);
            path.push(near, 1.0, 0.0, Mode::Watch);
            sink = sink.max(path.gone());
            *watcher = Watcher {
                who: Who {
                    woman: WHO[i].0,
                    child: WHO[i].1,
                    judge: false,
                    look: rng.u32(..),
                },
                path,
            };
        }
        let sink = sink + 0.3;
        Rabanos {
            t: 0.0,
            w: i32::from(w),
            h: i32::from(h),
            k,
            shown,
            first,
            x0,
            pitch,
            seed: rng.u32(..),
            winner,
            judge,
            pin,
            watchers,
            n,
            sink,
            end: sink + SINK_GAP * shown as f32 + SINK_SECS + 0.2,
        }
    }

    /// How far mesa `i` has risen: 0 under the ground, 1 standing.
    fn rise(&self, i: usize) -> f32 {
        let up = ease_out((self.t - MESA_FIRST - MESA_GAP * i as f32) / MESA_SECS);
        let down = ((self.t - self.sink - SINK_GAP * i as f32) / SINK_SECS).clamp(0.0, 1.0);
        (up - down * down).clamp(0.0, 1.0)
    }

    /// The middle of display `i`, in art pixels across.
    fn cx(&self, i: usize) -> f32 {
        self.x0 + self.pitch * (i as f32 + 0.5)
    }

    /// The pixel row of the ground.
    fn ground(&self) -> f32 {
        self.h as f32 * 2.0
    }

    /// The cell row the tops of the displays are in.
    fn top_row(&self) -> i32 {
        ((self.ground() - (DEPTH + MESA_H + ART_H as f32) * self.k) / 2.0).floor() as i32
    }

    /// The cell row the signs are in, on the cloth in front of the mesas.
    fn sign_row(&self) -> i32 {
        ((self.ground() - (DEPTH + 5.0) * self.k) / 2.0).floor() as i32
    }

    /// The sign of display `i`: the longest name that fits the cloth, and the
    /// column its first cell is in.
    fn sign(&self, i: usize) -> Option<(&'static str, i32)> {
        let room = (MESA_W * self.k).floor() as usize;
        let name = NAMES[self.first + i]
            .iter()
            .find(|n| n.chars().count() + 2 <= room)?;
        let len = name.chars().count() as i32 + 2;
        Some((name, (self.cx(i) * self.k).round() as i32 - len / 2))
    }

    /// Is everybody clapping?
    fn applause(&self) -> bool {
        (self.pin + 0.3..self.pin + APPLAUSE).contains(&self.t)
    }
}

impl Egg for Rabanos {
    fn update(&mut self, dt: f32) {
        self.t += dt.clamp(0.0, 1.0);
    }

    fn draw(&self, c: &mut Canvas, theme: &Theme) {
        if self.done() {
            return;
        }
        let pal = Palette::rabanos(theme);
        let base = Place {
            pal: &pal,
            k: self.k,
            s: 1.0,
            ox: 0.0,
            ground: self.ground(),
        };
        // The mesas with their displays, the signs on their cloths, the winner's
        // ribbon.
        for i in 0..self.shown {
            let rise = self.rise(i);
            if rise <= 0.0 {
                continue;
            }
            let table = Place {
                ground: base.ground - DEPTH * self.k + (1.0 - rise) * SUNK * self.k,
                ..base
            };
            self.mesa(c, &table, i);
        }
        for i in 0..self.shown {
            if self.rise(i) >= 0.98 {
                self.write_sign(c, &pal, i);
            }
        }
        // The people: the ones looking behind (a step higher up the screen),
        // the judge in front of them.
        let back = Place {
            ground: base.ground - 0.8 * self.k,
            ..base
        };
        for (i, w) in self.watchers.iter().enumerate().take(self.n) {
            if let Some(spot) = w.path.at(self.t) {
                self.watcher(c, &back, i, w, spot);
            }
        }
        if let Some(spot) = self.judge.at(self.t) {
            self.the_judge(c, &base, spot);
        }
        // The ribbon over everybody (it is the prize), and what glints round it.
        if self.t >= self.pin && self.rise(self.winner) >= 0.98 {
            self.rosette(c, &pal, self.winner);
        }
        if self.t >= self.pin && self.t < self.pin + PRIZE {
            self.sparkles(c, &base);
        }
        self.words(c, theme, &pal);
    }

    fn done(&self) -> bool {
        self.t > self.end || self.t > 29.5
    }
}

impl Rabanos {
    /// The mesa and its display: the table with its cloth, a band woven into
    /// it, and what is carved on it.
    fn mesa(&self, c: &mut Canvas, at: &Place, i: usize) {
        let pal = at.pal;
        let left = self.cx(i) - MESA_W / 2.0;
        let (cloth, band) = (pal.of(b'A'), pal.of(BANDS[self.first + i]));
        at.block(c, (left + 1.0, 0.0), (MESA_W - 2.0, MESA_H - 2.0), cloth);
        at.block(c, (left + 1.0, 1.4), (MESA_W - 2.0, 2.0), band);
        at.block(c, (left + 1.0, 3.4), (MESA_W - 2.0, 0.7), pal.of(b'a'));
        at.block(c, (left, MESA_H - 2.0), (MESA_W, 2.0), pal.of(b'D'));
        at.block(c, (left, MESA_H - 2.0), (MESA_W, 0.7), pal.of(b'd'));
        let top = MESA_H + ART_H as f32;
        at.sprite(
            c,
            ARTS[self.first + i],
            (self.cx(i) - ART_W as f32 / 2.0, top),
            (0.0, 1.0),
            |l| pal.get(l),
        );
    }

    /// The sign of display `i`, in the terminal's own letters on a paper
    /// background.
    fn write_sign(&self, c: &mut Canvas, pal: &Palette, i: usize) {
        let Some((name, left)) = self.sign(i) else {
            return;
        };
        let (paper, ink) = (pal.of(b'S'), pal.of(b'n'));
        let row = self.sign_row();
        let (fg, bg, attr) = if c.strict {
            (paper, c.bg(), BOLD)
        } else {
            (ink, paper, BOLD)
        };
        let cells = std::iter::once(' ')
            .chain(name.chars())
            .chain(std::iter::once(' '));
        for (j, ch) in cells.enumerate() {
            c.set_cell(left + j as i32, row, Cell { ch, fg, bg, attr });
        }
    }

    /// Where the ribbon goes on display `i`: the screen column of its middle,
    /// just before its sign (or where the sign would be), and its size.
    fn rosette_at(&self, i: usize) -> (f32, f32) {
        let left = self
            .sign(i)
            .map_or((self.cx(i) * self.k).round() as i32 - 5, |(_, left)| left);
        let s = self.k.max(0.75);
        (left as f32 + 0.5 - 2.6 * s, s)
    }

    /// The ribbon pinned to the end of the winner's sign: a gold rosette with
    /// a red tail and a white one.
    fn rosette(&self, c: &mut Canvas, pal: &Palette, i: usize) {
        let (ox, s) = self.rosette_at(i);
        let at = Place {
            pal,
            k: s,
            s: 1.0,
            ox,
            ground: (self.sign_row() * 2 + 3) as f32,
        };
        // The tails, then the rosette over them, its ruffle and its heart.
        at.block(c, (-1.8, -1.6), (1.6, 3.6), pal.of(b'R'));
        at.block(c, (0.2, -1.6), (1.6, 3.6), pal.of(b'W'));
        at.disc(c, (0.0, 3.6), 2.6, pal.of(b'Y'));
        at.disc(c, (0.0, 3.6), 1.7, pal.of(b'y'));
        at.disc(c, (0.0, 3.6), 0.9, pal.of(b'Y'));
    }

    /// Glints round the winner's display for a while after the ribbon.
    fn sparkles(&self, c: &mut Canvas, at: &Place) {
        let pal = at.pal;
        let rise = self.rise(self.winner);
        if rise < 0.98 {
            return;
        }
        let ground = at.ground - DEPTH * self.k;
        let place = Place { ground, ..*at };
        let age = self.t - self.pin;
        for j in 0..9u32 {
            let h = |salt: u32| super::blit::hash(self.seed ^ salt, j);
            // Each comes and goes in turn.
            let s = ((age * 2.2 + h(1) * TAU).sin() + 0.2).max(0.0);
            let fade_out = ((PRIZE - age) / 1.0).clamp(0.0, 1.0);
            let size = s * fade_out;
            if size < 0.25 {
                continue;
            }
            let x = self.cx(self.winner) + (h(2) - 0.5) * 24.0;
            let y = MESA_H + 2.0 + h(3) * (ART_H as f32 + 4.0);
            let color = if j % 2 == 0 {
                pal.of(b'Y')
            } else {
                pal.of(b'W')
            };
            // A glint: a small x of light.
            for (dx, dy) in [
                (0.0, 0.0),
                (-1.0, -1.0),
                (1.0, -1.0),
                (-1.0, 1.0),
                (1.0, 1.0),
            ] {
                place.block(c, (x + dx - 0.5, y + dy - 0.5), (1.0, 1.0), color);
            }
        }
    }

    /// The title, the winner's prize and the crowd's cheer.
    fn words(&self, c: &mut Canvas, theme: &Theme, pal: &Palette) {
        let gone = ((self.t - self.sink) / 1.0).clamp(0.0, 1.0);
        // The title over the displays.
        let title_row = self.top_row() - 1;
        let come = ((self.t - TITLE_FROM) / 0.8).clamp(0.0, 1.0);
        if self.w >= 20 && come > 0.0 {
            let left = (self.w - TITLE.chars().count() as i32) / 2;
            let color = fade(theme, pal.of(b'Y'), (1.0 - come).max(gone));
            c.text_attr(left, title_row, TITLE, color, BOLD);
        }
        // The prize rises from the winner's display.
        let age = self.t - self.pin;
        if (0.0..PRIZE).contains(&age) && self.rise(self.winner) >= 0.98 {
            let row = self.top_row() - 2 - (age * 2.0).min(1.0) as i32;
            let len = PRIMER.chars().count() as i32;
            let left = (self.cx(self.winner) * self.k).round() as i32 - len / 2;
            let color = fade(
                theme,
                pal.of(b'Y'),
                ((age - (PRIZE - 0.6)) / 0.6).clamp(0.0, 1.0),
            );
            c.text_attr(left, row.max(0), PRIMER, color, BOLD);
        }
        // The crowd, over two heads.
        if self.applause() {
            let row = ((self.ground() - 32.0 * self.k) / 2.0).floor() as i32 - 1;
            for (i, w) in self.watchers.iter().enumerate().take(self.n).step_by(2) {
                if let Some(spot) = w.path.at(self.t) {
                    let len = BRAVO.chars().count() as i32;
                    let left = (spot.x * self.k).round() as i32 - len / 2;
                    let bob = ((self.t * 4.0 + i as f32).sin() * 0.6).round() as i32;
                    let color = fade(theme, pal.of(b'W'), gone);
                    c.text_attr(left, (row + bob).max(0), BRAVO, color, BOLD);
                }
            }
        }
    }

    /// One of the people looking.
    fn watcher(&self, c: &mut Canvas, at: &Place, i: usize, w: &Watcher, spot: Spot) {
        let arms = if spot.walking {
            Arms::Down
        } else if self.applause() {
            Arms::Cheer((self.t * 9.0 + i as f32 * 1.3).sin())
        } else {
            let turn = (spot.since / 3.6) as u32 + (w.who.look >> 14);
            match turn % 4 {
                0 | 2 => Arms::Chin,
                1 => Arms::Point(0.8 * (self.t * 0.9 + i as f32).sin()),
                _ if w.who.look & 1 == 1 => Arms::Photo((self.t * 1.3 + i as f32).fract() < 0.1),
                _ => Arms::Down,
            }
        };
        let phase = spot.x / STRIDE * TAU * spot.face;
        let bob = if spot.walking {
            0.5 * (phase * 2.0).cos().abs()
        } else {
            0.25 * (self.t * 1.6 + i as f32 * 2.1).sin()
        };
        let here = Place {
            s: spot.face,
            ox: spot.x * self.k,
            ..*at
        };
        let pose = Pose {
            phase,
            walking: spot.walking,
            arms,
            lean: 0.0,
            bob,
        };
        self.person(c, here, &w.who, &pose);
    }

    /// The judge: walking, bent to a display with his clipboard, or pinning.
    fn the_judge(&self, c: &mut Canvas, at: &Place, spot: Spot) {
        let who = Who {
            woman: false,
            child: false,
            judge: true,
            look: 0,
        };
        let here = Place {
            s: spot.face,
            ox: spot.x * self.k,
            ..*at
        };
        let phase = spot.x / STRIDE * TAU * spot.face;
        let (arms, lean) = match spot.mode {
            _ if spot.walking => (Arms::Clip, 0.0),
            Mode::Examine => {
                // He comes closer, looks, and writes.
                let bend =
                    ease_out(spot.since / 0.5) * (1.0 - ease_out((spot.since - DWELL + 0.4) / 0.4));
                (Arms::Clip, 2.6 * bend)
            }
            Mode::Pin => {
                // His arm reaches the ribbon's place (in his own, smaller units).
                let (col, s) = self.rosette_at(self.winner);
                let reach = (((col - 0.5 + 2.6 * s) / self.k - spot.x) * spot.face / PERSON)
                    .clamp(3.0, 15.0);
                let holding = self.t < self.pin;
                let raise = ease_out(spot.since / WAIT);
                if self.applause() {
                    (Arms::Cheer((self.t * 9.0).sin()), 0.0)
                } else {
                    (Arms::Pin(reach * raise, holding), 0.8 * raise)
                }
            }
            Mode::Watch => (Arms::Down, 0.0),
        };
        let pose = Pose {
            phase,
            walking: spot.walking,
            arms,
            lean,
            bob: if spot.walking {
                0.5 * (phase * 2.0).cos().abs()
            } else {
                0.0
            },
        };
        self.person(c, here, &who, &pose);
    }

    /// A person in profile facing the way their place faces, in the colors of
    /// their `who`: legs, skirt or trousers, torso, rebozo, head with its hair
    /// or hat, and arms as the pose has them.
    fn person(&self, c: &mut Canvas, at: Place, who: &Who, pose: &Pose) {
        let pal = at.pal;
        let at = at.scaled(if who.child { 0.78 * PERSON } else { PERSON });
        let (skin, shade) = (pal.of(b'K'), pal.of(b'k'));
        let hip = 10.0 + pose.bob;
        let shoulder = (0.4 + 0.5 * pose.lean, 18.5 + pose.bob);
        let head = (0.9 + pose.lean, 22.3 + pose.bob);
        let top = pal.of(who.top());
        // The legs, one a step ahead of the other.
        for j in [1.0f32, 0.0] {
            let psi = pose.phase + j * PI;
            let foot = if pose.walking {
                (psi.sin() * 3.6, psi.cos().max(0.0) * 1.8)
            } else {
                ((j - 0.5) * 2.4, 0.0)
            };
            if who.woman {
                at.limb(c, (0.2, 4.5), (foot.0, foot.1 + 1.0), 1.7, skin);
            } else {
                at.limb(
                    c,
                    (0.2, hip),
                    (foot.0, foot.1 + 0.8),
                    3.0,
                    pal.of(who.trousers()),
                );
            }
            at.block(c, (foot.0 - 1.0, foot.1), (4.0, 1.4), pal.of(b'd'));
        }
        // The far arm, behind the body.
        self.arm(
            c,
            &at,
            (shoulder.0 - 0.6, shoulder.1 - 0.9),
            who,
            pose,
            true,
        );
        // A woman's skirt, wider toward the hem, with a lighter band.
        if who.woman {
            let skirt = pal.of(who.skirt());
            for r in 0..7 {
                let half = 2.7 + r as f32 * 0.5;
                let color = if r == 6 { skirt.scale(1.5) } else { skirt };
                at.block(c, (0.2 - half, hip - r as f32), (2.0 * half, 1.0), color);
            }
        }
        // The body, and over it a woman's rebozo, a man's collar or the judge's
        // tie.
        at.limb(
            c,
            (0.0, hip + 0.5),
            (shoulder.0 - 0.2, shoulder.1 - 0.3),
            if who.woman { 5.0 } else { 5.6 },
            top,
        );
        if who.woman {
            at.limb(
                c,
                (-2.0, shoulder.1 - 0.2),
                (1.8, hip + 2.0),
                2.4,
                pal.of(who.rebozo()),
            );
            at.block(
                c,
                (-3.2, hip - 1.0),
                (2.0, 6.0),
                pal.of(who.rebozo()).scale(0.8),
            );
        } else if who.judge {
            at.block(c, (1.6, hip + 3.0), (1.2, 6.0), pal.of(b'A'));
            at.block(c, (1.9, hip + 3.0), (0.8, 4.6), pal.of(b'R'));
        }
        // The neck and head, the eye, the nose.
        at.block(c, (head.0 - 1.0, shoulder.1), (2.0, 2.0), shade);
        at.disc(c, (head.0, head.1), 3.2, skin);
        at.block(c, (head.0 + 1.5, head.1 + 0.4), (1.0, 1.0), pal.of(b'n'));
        at.block(c, (head.0 + 2.8, head.1 - 0.8), (1.2, 1.2), skin);
        if who.woman {
            // Hair gathered back, a braid down the back tied with a ribbon.
            at.disc(c, (head.0 - 1.0, head.1 + 0.7), 3.2, pal.of(b'N'));
            at.disc(c, (head.0 + 0.6, head.1 - 0.2), 2.8, skin);
            at.limb(
                c,
                (head.0 - 2.6, head.1),
                (-3.4, hip + 3.0),
                1.5,
                pal.of(b'N'),
            );
            at.block(c, (-4.2, hip + 2.2), (1.8, 1.2), pal.of(b'P'));
        } else if who.judge {
            at.block(c, (head.0 - 2.8, head.1 + 1.8), (4.4, 1.6), pal.of(b'w'));
            at.block(c, (head.0 - 3.0, head.1 - 0.6), (1.6, 2.6), pal.of(b'w'));
            at.block(c, (head.0 + 1.0, head.1 + 0.2), (2.4, 0.5), pal.of(b'n'));
        } else {
            // A sombrero of palma: crown, band and a wide brim.
            at.block(c, (head.0 - 4.6, head.1 + 2.1), (9.6, 1.2), pal.of(b'E'));
            at.block(c, (head.0 - 2.0, head.1 + 3.1), (4.2, 2.2), pal.of(b'H'));
            at.block(c, (head.0 - 2.0, head.1 + 3.1), (4.2, 0.9), pal.of(b'R'));
        }
        // The near arm, in front.
        self.arm(
            c,
            &at,
            (shoulder.0 + 0.8, shoulder.1 - 0.9),
            who,
            pose,
            false,
        );
    }

    /// An arm from the shoulder at `from`; the far one hangs or claps, the
    /// near one does what the pose says.
    fn arm(&self, c: &mut Canvas, at: &Place, from: (f32, f32), who: &Who, pose: &Pose, far: bool) {
        let pal = at.pal;
        let hip = 10.0 + pose.bob;
        let sleeve = if who.woman {
            pal.of(who.rebozo())
        } else {
            pal.of(who.top())
        };
        let sleeve = if far { sleeve.scale(0.75) } else { sleeve };
        let skin = pal.of(b'K');
        let swing = if pose.walking {
            3.0 * (pose.phase + if far { 0.0 } else { PI }).sin()
        } else {
            0.0
        };
        // Where the elbow and the hand are, local to the person.
        let (elbow, hand) = match (pose.arms, far) {
            (Arms::Cheer(a), _) => {
                let up = if far { -0.8 } else { 0.8 };
                (
                    (from.0 + 1.6, from.1 + 3.0),
                    (from.0 + 2.2 + up * a, from.1 + 9.0 + 0.8 * a),
                )
            }
            (Arms::Chin, false) => ((from.0 + 2.6, hip + 3.8), (from.0 + 3.6, from.1 + 3.2)),
            (Arms::Point(a), false) => (
                (from.0 + 3.6, from.1 - 0.5),
                (from.0 + 7.4, from.1 - 0.5 + 2.0 * a),
            ),
            (Arms::Photo(_), false) => ((from.0 + 3.2, from.1 - 2.0), (from.0 + 5.6, from.1 + 2.6)),
            (Arms::Clip, false) => ((from.0 + 2.6, hip + 3.0), (from.0 + 3.8, hip + 6.4)),
            (Arms::Pin(reach, _), false) => (
                (from.0 + reach * 0.5, hip + 5.0),
                (from.0 + reach, hip + 3.6),
            ),
            _ => (
                (from.0 + 0.2 + swing * 0.3, hip + 5.6),
                (from.0 + 0.6 + swing, hip + 1.4),
            ),
        };
        at.limb(c, from, elbow, 1.7, sleeve);
        at.limb(c, elbow, hand, 1.5, sleeve);
        at.block(c, (hand.0 - 0.7, hand.1 - 0.7), (1.4, 1.4), skin);
        if far {
            return;
        }
        match pose.arms {
            Arms::Clip => {
                // The clipboard in his hand, a sheet on it, and the pencil.
                at.block(c, (hand.0 - 1.4, hand.1 - 0.4), (3.6, 4.6), pal.of(b'D'));
                at.block(c, (hand.0 - 0.8, hand.1 + 0.2), (2.4, 3.4), pal.of(b'A'));
            }
            Arms::Photo(flash) => {
                at.block(c, (hand.0 - 0.6, hand.1 + 0.4), (1.8, 3.0), pal.of(b'n'));
                if flash {
                    let white = pal.of(b'W');
                    at.block(c, (hand.0 + 1.2, hand.1 + 3.6), (1.0, 3.0), white);
                    at.block(c, (hand.0 + 0.2, hand.1 + 4.6), (3.0, 1.0), white);
                }
            }
            Arms::Pin(_, true) => {
                at.disc(c, (hand.0 + 0.3, hand.1 + 0.2), 1.6, pal.of(b'Y'));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ALL, Eggs};
    use super::*;
    use crate::color::{Color, rgb};
    use crate::fiesta::Fiesta;

    /// The egg of a screen, with the judge going the way he is told.
    fn egg(w: u16, h: u16, seed: u64) -> Rabanos {
        Rabanos::new(w, h, &mut fastrand::Rng::with_seed(seed))
    }

    /// Every cell of the screen at second `t` (the whole picture).
    fn frame(r: &mut Rabanos, w: u16, h: u16, t: f32, theme: &Theme) -> Canvas {
        r.t = t;
        let mut c = Canvas::new(w, h, Color::Reset);
        r.draw(&mut c, theme);
        c
    }

    /// Which way the judge goes along the row: 1 from left to right.
    fn dir(r: &Rabanos) -> f32 {
        if r.judge.stops[1].x > r.judge.stops[0].x {
            1.0
        } else {
            -1.0
        }
    }

    fn has(c: &Canvas, hex: u32) -> bool {
        c.cells()
            .iter()
            .any(|cell| cell.fg == rgb(hex) || cell.bg == rgb(hex))
    }

    #[test]
    fn the_art_is_whole() {
        for (i, art) in ARTS.iter().enumerate() {
            assert_eq!(art.len(), ART_H, "display {i}");
            assert!(art.iter().all(|r| r.len() == ART_W), "display {i}");
            // And every letter of it has a color.
            let pal = Palette::rabanos(&Theme::classic());
            for l in art.iter().flat_map(|r| r.bytes()) {
                assert!(l == b'.' || pal.get(l).is_some(), "{}", l as char);
            }
        }
        assert_eq!(ARTS.len(), NAMES.len());
        assert!(NAMES.iter().all(|n| !n.is_empty()));
    }

    #[test]
    fn every_letter_has_a_color_under_every_theme() {
        for name in ["classic", "dracula", "nord", "gruvbox", "mono"] {
            let Some(t) = Theme::builtin(name) else {
                continue;
            };
            let pal = Palette::rabanos(&t);
            for &(ch, ..) in COLORS {
                assert!(pal.get(ch).is_some(), "{} under {name}", ch as char);
            }
            assert_eq!(pal.get(b'.'), None);
        }
    }

    #[test]
    fn classic_keeps_the_holiday_colors_and_themes_their_own() {
        let pal = Palette::rabanos(&Theme::classic());
        assert_eq!(pal.of(b'R'), rgb(0xd1263b));
        assert_eq!(pal.of(b'W'), rgb(0xf7f0ef));
        assert_eq!(pal.of(b'G'), rgb(0x4caf50));
        assert_eq!(pal.of(b'M'), rgb(0x8e44ad));
        assert_eq!(pal.of(b'H'), rgb(0xd9c48a));
        let t = Theme::builtin("dracula").unwrap();
        let pal = Palette::rabanos(&t);
        assert!(t.colors.contains(&pal.of(b'R')));
        assert!(t.colors.contains(&pal.of(b'G')));
    }

    /// It comes on the twenty-third of December, in any year, and not on the
    /// days around it, an ordinary day or another holiday; the visitors of the
    /// posadas that come on the 23rd keep coming.
    #[test]
    fn it_comes_on_the_noche_de_rabanos_only() {
        let info = ALL.iter().find(|i| i.name == "rabanos").unwrap();
        assert_eq!(info.fiesta, Kind::Navidad);
        let comes = |name: &str, f: Fiesta| {
            let info = ALL.iter().find(|i| i.name == name).unwrap();
            let mut e = Eggs::new(80, 24);
            e.start(&[info], f);
            e.active()
        };
        for (y, m, d) in [(2026, 12, 23), (2027, 12, 23), (2028, 12, 23)] {
            let f = Fiesta::of_date(y, m, d);
            assert!(f.rabanos(), "{y}");
            assert!(comes("rabanos", f), "{y}-{m}-{d}");
            for other in ["posada", "pinata", "pastorela"] {
                assert!(comes(other, f), "{other} on {y}-{m}-{d}");
            }
        }
        assert!(!comes("rabanos", Fiesta::NONE));
        for (y, m, d) in [
            (2026, 12, 22),
            (2026, 12, 24),
            (2027, 12, 22),
            (2027, 12, 24),
            (2026, 12, 12),
            (2026, 12, 16),
            (2026, 12, 25),
            (2027, 1, 6),
            (2027, 6, 15),
            (2027, 11, 2),
            (2027, 9, 15),
        ] {
            assert!(!comes("rabanos", Fiesta::of_date(y, m, d)), "{y}-{m}-{d}");
        }
    }

    /// The mesas rise one after another, stand through the visit and sink the
    /// same way; nothing is on the screen before them or after.
    #[test]
    fn the_mesas_rise_and_sink() {
        let theme = Theme::classic();
        let mut r = egg(100, 30, 3);
        let c = frame(&mut r, 100, 30, 0.0, &theme);
        assert!(c.cells().iter().all(|cell| cell.ch == ' '));
        let rises = |r: &mut Rabanos, t: f32| {
            r.t = t;
            (0..r.shown).map(|i| r.rise(i)).collect::<Vec<_>>()
        };
        let early = rises(&mut r, 1.0);
        assert!(early[0] > early[1] && early[1] > early[3], "{early:?}");
        assert!(rises(&mut r, 4.0).iter().all(|&x| x > 0.99));
        let pin = r.pin;
        assert!(rises(&mut r, pin).iter().all(|&x| x > 0.99));
        let (sink, end) = (r.sink, r.end);
        let sinking = rises(&mut r, sink + 1.0);
        assert!(sinking[0] < sinking[3] && sinking[0] < 0.9, "{sinking:?}");
        assert!(rises(&mut r, end).iter().all(|&x| x == 0.0));
        let c = frame(&mut r, 100, 30, end + 0.01, &theme);
        assert!(c.cells().iter().all(|cell| cell.ch == ' '));
        assert!(r.done());
    }

    /// The egg of a screen with nobody in it, so that every sign can be read.
    fn empty(w: u16, h: u16, seed: u64) -> Rabanos {
        let mut r = egg(w, h, seed);
        (r.n, r.judge) = (0, Path::new(0.0, 100.0, 1.0));
        r
    }

    /// The displays show the colors of the radish, the leaves, the husk and the
    /// siempreviva, and the signs are written on the cloth, the long names only
    /// where they fit; a narrower screen has room for fewer displays (a run of
    /// the four).
    #[test]
    fn the_displays_and_their_signs() {
        let theme = Theme::classic();
        let mut r = empty(100, 30, 3);
        let c = frame(&mut r, 100, 30, 6.0, &theme);
        for hex in [0xd1263b, 0xf7f0ef, 0x4caf50, 0x8e44ad, 0xd9c48a] {
            assert!(has(&c, hex), "{hex:06x}");
        }
        let text = c.to_text();
        for name in ["Nacimiento", "Calenda", "Mariachi", "Danza de la Pluma"] {
            assert!(text.contains(name), "no sign for {name}");
        }
        assert!(text.contains("Noche de Rábanos"));
        for (w, h, shown) in [(100, 30, 4), (80, 24, 3), (60, 20, 2)] {
            let mut r = empty(w, h, 3);
            assert_eq!(r.shown, shown, "{w}x{h}");
            let text = frame(&mut r, w, h, 6.0, &theme).to_text();
            let signs = NAMES
                .iter()
                .filter(|names| names.iter().any(|name| text.contains(name)))
                .count();
            assert_eq!(signs, shown, "{w}x{h}:\n{text}");
        }
    }

    /// The people come on and go off the screen, through its sides.
    #[test]
    fn people_come_and_go() {
        let theme = Theme::classic();
        for seed in 1..6 {
            let mut r = egg(100, 30, seed);
            let mut seen = 0;
            let mut t = 0.0;
            while t < r.end {
                let c = frame(&mut r, 100, 30, t, &theme);
                if has(&c, 0xd7a066) {
                    seen += 1;
                }
                t += 0.5;
            }
            assert!(seen > 30, "{seen} frames with people");
            for w in r.watchers.iter().take(r.n) {
                assert!(w.path.at(0.0).is_none());
                assert!(w.path.at(r.sink).is_none(), "still there as the mesas sink");
                assert!(w.path.at(r.pin).is_some());
            }
            assert!(r.judge.at(r.sink).is_none());
        }
    }

    /// The judge walks the row once in order, bent to each display in turn,
    /// goes to the winner and pins the ribbon there.
    #[test]
    fn the_judge_pins_the_ribbon_on_the_winner() {
        let theme = Theme::classic();
        for seed in 1..9 {
            let mut r = egg(100, 30, seed);
            // He stops at each display: seen at its gap, in order.
            let mut visited = Vec::new();
            let mut t = 0.0;
            while t < r.pin {
                let looking = r
                    .judge
                    .at(t)
                    .filter(|s| s.mode == Mode::Examine && !s.walking);
                if let Some(s) = looking {
                    let gap = ((s.x - r.x0) / r.pitch).round() as usize;
                    let i = if dir(&r) > 0.0 { gap } else { gap - 1 };
                    if visited.last() != Some(&i) {
                        visited.push(i);
                    }
                }
                t += 0.1;
            }
            let order: Vec<usize> = if dir(&r) > 0.0 {
                (0..r.shown).collect()
            } else {
                (0..r.shown).rev().collect()
            };
            assert_eq!(visited, order, "seed {seed}");
            // At the winner's when he pins; the ribbon only after.
            let s = r.judge.at(r.pin).unwrap();
            let gap = ((s.x - r.x0) / r.pitch).round() as usize;
            assert_eq!(if dir(&r) > 0.0 { gap } else { gap - 1 }, r.winner);
            assert_eq!(s.mode, Mode::Pin);
            let pin = r.pin;
            let before = frame(&mut r, 100, 30, pin - 0.2, &theme);
            let after = frame(&mut r, 100, 30, pin + 1.0, &theme);
            let gold = |c: &Canvas| {
                c.cells()
                    .iter()
                    .filter(|cell| cell.fg == rgb(0xf2c14e) || cell.bg == rgb(0xf2c14e))
                    .count()
            };
            assert!(gold(&after) > gold(&before) + 2, "seed {seed}");
            assert!(after.to_text().contains("¡Primer lugar!"));
            assert!(!before.to_text().contains("¡Primer lugar!"));
            assert!(after.to_text().contains("¡Bravo!"));
        }
    }

    /// Whatever the screen, everyone has gone before the mesas sink, and it is
    /// all over with time to spare.
    #[test]
    fn it_finishes_in_time_at_any_size() {
        for (w, h) in [
            (1, 1),
            (2, 40),
            (7, 3),
            (20, 8),
            (40, 12),
            (80, 24),
            (100, 30),
            (200, 50),
            (300, 90),
            (500, 40),
            (500, 150),
        ] {
            for seed in 0..4 {
                let r = egg(w, h, seed);
                assert!(r.end < 29.4, "{w}x{h}: over at {}", r.end);
                assert!(r.pin < r.sink, "{w}x{h}");
                for wa in r.watchers.iter().take(r.n) {
                    assert!(wa.path.gone() < r.sink, "{w}x{h}");
                }
                assert!(r.judge.gone() < r.sink, "{w}x{h}");
                // The judge keeps a walking pace.
                assert!(r.judge.pace <= 45.0 && r.judge.pace >= 14.0, "{w}x{h}");
            }
        }
    }

    /// Nothing is drawn at any size or step that goes beyond the canvas.
    #[test]
    fn drawing_survives_odd_sizes_and_themes() {
        for theme in [Theme::classic(), Theme::builtin("nord").unwrap()] {
            for (w, h) in [(1, 1), (2, 40), (7, 3), (40, 12), (300, 90), (500, 150)] {
                let mut r = egg(w, h, 7);
                let mut c = Canvas::new(w, h, Color::Reset);
                for strict in [false, true] {
                    c.strict = strict;
                    let mut t = 0.0;
                    while t < r.end + 1.0 {
                        r.t = t;
                        c.clear();
                        r.draw(&mut c, &theme);
                        t += 0.7;
                    }
                }
            }
        }
    }
}
