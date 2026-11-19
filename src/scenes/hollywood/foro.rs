//! The set layout (spec 10.3): in set mode the panes are a pure function of
//! the seed and the size of the area they tile. A tall screen is a stack of
//! full-width bands, one pane each; a wide one is a main column (the banner
//! over the log) beside a side column of one or two panes. The shaped widgets
//! go where their shape fits. Every draw comes from a generator of this
//! module's own, so neither the preset nor the scene's stream can move a pane.

use super::Rect;
use fastrand::Rng;

/// The access pane.
pub const BANNER: usize = 17;
/// The mission log.
pub const LOG: usize = 8;
/// The shaped widgets: radar, map, rain, bandwidth, progress.
pub const POOL: [usize; 5] = [13, 12, 11, 10, 14];

const RADAR: usize = 13;
const MAP: usize = 12;
const RAIN: usize = 11;
const BANDWIDTH: usize = 10;
const PROGRESS: usize = 14;

/// Keeps this module's draws off any other stream seeded from the same seed.
const SALT: u64 = 0x666f_726f_5f73_6574;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Banner,
    Log,
    Shape,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Seat {
    pub rect: Rect,
    pub role: Role,
    pub kind: usize,
    pub seed: u64,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Plan {
    /// In reading order: from the top, then from the left.
    pub seats: Vec<Seat>,
    /// Seeds the dealing of the titles.
    pub title_seed: u64,
    /// A spare draw: which signature widget a forced holiday gets.
    pub lot: u64,
}

/// `c` x `r` is the area the panes tile (the terminal less the ticker's row).
pub fn plan(seed: u64, c: i32, r: i32) -> Plan {
    // No terminal is larger, and the arithmetic below stays in `i32`.
    let max = i32::from(u16::MAX);
    let (c, r) = (c.min(max), r.min(max));
    let mut rng = Rng::with_seed(seed ^ SALT);
    let mut left = POOL.to_vec();
    rng.shuffle(&mut left);
    let mut cells = if c <= 0 || r <= 0 {
        Vec::new()
    } else if 2 * i64::from(r) > i64::from(c) {
        tall(&mut rng, c, r)
    } else {
        wide(&mut rng, c, r)
    };
    cells.sort_by_key(|(rect, _)| (rect.y, rect.x));
    let seats = cells
        .into_iter()
        .map(|(rect, role)| {
            let kind = match role {
                Role::Banner => BANNER,
                Role::Log => LOG,
                // There are two shape seats at most and five kinds.
                Role::Shape => {
                    let i = left
                        .iter()
                        .position(|&k| suits(k, rect.w, rect.h))
                        .unwrap_or(0);
                    left.remove(i)
                }
            };
            Seat {
                rect,
                role,
                kind,
                seed: rng.u64(..),
            }
        })
        .collect();
    Plan {
        seats,
        title_seed: rng.u64(..),
        lot: rng.u64(..),
    }
}

/// Whether a pane of `w` x `h` cells has the physical shape (`w / (2h)`)
/// that `kind` wants (spec 10.3).
pub fn suits(kind: usize, w: i32, h: i32) -> bool {
    if w <= 0 || h <= 0 {
        return false;
    }
    let (w, h) = (i64::from(w), i64::from(h));
    match kind {
        // 0.6 ..= 2.4
        RADAR => 5 * w >= 6 * h && 5 * w <= 24 * h,
        // 1.2 or more
        MAP => 5 * w >= 12 * h,
        RAIN => true,
        // 0.6 or more
        BANDWIDTH => 5 * w >= 6 * h,
        // 1.0 or more, and 40 columns
        PROGRESS => w >= 2 * h && w >= 40,
        _ => false,
    }
}

/// Rows of the banner band a tall screen of `c` columns asks for: the title
/// row and the box of three lines of eight big letters at the biggest size
/// that fits across.
pub fn banner_band(c: i32) -> i32 {
    match c {
        163.. => 46,
        131.. => 37,
        99.. => 29,
        _ => 20,
    }
}

/// A pane of the layout before it gets its kind and seed.
fn cell(x: i32, y: i32, w: i32, h: i32, role: Role) -> (Rect, Role) {
    (Rect { x, y, w, h }, role)
}

/// `num / den` rounded half up (`floor(x + 0.5)`), for non-negative `num` and positive `den`.
fn round(num: i64, den: i64) -> i32 {
    ((2 * num + den) / (2 * den)) as i32
}

/// A stack of full-width bands, one pane each, the banner first or second.
fn tall(rng: &mut Rng, c: i32, r: i32) -> Vec<(Rect, Role)> {
    if c < 30 || r < 21 {
        return vec![cell(0, 0, c, r, Role::Banner)];
    }
    let log = round(r.into(), 8).clamp(7, 14);
    let mut ban = banner_band(c).min(2 * r / 5).max(14);
    let rest = r - ban - log;
    let mut bands = vec![(log, Role::Log)];
    if rest >= 18 {
        bands.push((rest - rest / 2, Role::Shape));
        bands.push((rest / 2, Role::Shape));
    } else if rest >= 9 {
        bands.push((rest, Role::Shape));
    } else {
        ban = r - log;
    }
    rng.shuffle(&mut bands);
    bands.insert(rng.usize(..2), (ban, Role::Banner));
    let mut y = 0;
    bands
        .into_iter()
        .map(|(h, role)| {
            y += h;
            cell(0, y - h, c, h, role)
        })
        .collect()
}

/// The banner over the log in the main column, beside a side column of one
/// or two panes when there is room for it; the seed picks the sides.
fn wide(rng: &mut Rng, c: i32, r: i32) -> Vec<(Rect, Role)> {
    let side = round(38 * i64::from(c), 100).max(30);
    let has_side = c - side >= 44 && r >= 7;
    let main = if has_side { c - side } else { c };
    let main_left = !has_side || rng.bool();
    let (mx, sx) = if main_left { (0, main) } else { (side, 0) };
    let log = if r >= 21 {
        round(3 * i64::from(r), 10).clamp(7, 14)
    } else {
        0
    };
    let mut out = vec![cell(mx, 0, main, r - log, Role::Banner)];
    if log > 0 {
        out.push(cell(mx, r - log, main, log, Role::Log));
    }
    if !has_side {
        return out;
    }
    let pane = |x, y, w, h| cell(x, y, w, h, Role::Shape);
    if 2 * r >= side && r >= 24 {
        let top = r - r / 2;
        out.push(pane(sx, 0, side, top));
        out.push(pane(sx, top, side, r / 2));
    } else if 2 * r < side && side >= 60 {
        let first = side - side / 2;
        out.push(pane(sx, 0, first, r));
        out.push(pane(sx + first, 0, side / 2, r));
    } else {
        out.push(pane(sx, 0, side, r));
    }
    out
}

/// The terminals of spec 11.5: tall, wide, in between.
#[cfg(test)]
pub(super) const SIZES: [(u16, u16); 14] = [
    (44, 31),
    (60, 55),
    (67, 60),
    (90, 80),
    (120, 106),
    (130, 110),
    (80, 24),
    (100, 30),
    (120, 33),
    (213, 60),
    (240, 35),
    (240, 65),
    (106, 60),
    (100, 50),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::hollywood::letrero::{self, Size};

    /// Every area of `SIZES` (the terminal less the ticker's row) with seeds 0 to 99.
    fn each(mut f: impl FnMut(u64, i32, i32, &Plan)) {
        for (w, h) in SIZES {
            let (c, r) = (i32::from(w), i32::from(h) - 1);
            for s in 0..100 {
                f(s, c, r, &plan(s, c, r));
            }
        }
    }

    /// Every area from 1x1 to 80x40, seeds 0 to 99.
    fn each_small(mut f: impl FnMut(u64, i32, i32, &Plan)) {
        for c in 1..=80 {
            for r in 1..=40 {
                for s in 0..100 {
                    f(s, c, r, &plan(s, c, r));
                }
            }
        }
    }

    fn banner(p: &Plan) -> Seat {
        let b: Vec<_> = p.seats.iter().filter(|s| s.role == Role::Banner).collect();
        assert_eq!(b.len(), 1, "{p:?}");
        *b[0]
    }

    fn count(p: &Plan, role: Role) -> usize {
        p.seats.iter().filter(|s| s.role == role).count()
    }

    fn shapes(p: &Plan) -> usize {
        count(p, Role::Shape)
    }

    fn tiles(c: i32, r: i32, p: &Plan) {
        let mut sum = 0;
        for (i, a) in p.seats.iter().enumerate() {
            let a = a.rect;
            assert!(
                a.x >= 0 && a.y >= 0 && a.w > 0 && a.h > 0 && a.x + a.w <= c && a.y + a.h <= r,
                "{c}x{r}: {a:?} outside"
            );
            sum += a.area();
            for b in &p.seats[i + 1..] {
                let b = b.rect;
                let apart =
                    a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y;
                assert!(apart, "{c}x{r}: {a:?} overlaps {b:?}");
            }
        }
        assert_eq!(sum, c * r, "{c}x{r}: {p:?}");
    }

    #[test]
    fn the_panes_tile_the_area() {
        each(|_, c, r, p| tiles(c, r, p));
    }

    #[test]
    fn one_banner_one_log() {
        each(|_, _, r, p| {
            assert_eq!(banner(p).kind, 17);
            let logs: Vec<_> = p.seats.iter().filter(|s| s.role == Role::Log).collect();
            if r >= 21 {
                assert_eq!(logs.len(), 1, "{p:?}");
                assert_eq!(logs[0].kind, 8);
            }
        });
    }

    #[test]
    fn no_kind_twice() {
        each(|_, _, _, p| {
            for (i, a) in p.seats.iter().enumerate() {
                assert!(p.seats[i + 1..].iter().all(|b| b.kind != a.kind), "{p:?}");
                if a.role == Role::Shape {
                    assert!(POOL.contains(&a.kind), "{p:?}");
                }
            }
        });
    }

    #[test]
    fn a_tall_screen_is_bands() {
        each(|_, c, r, p| {
            if 2 * r <= c {
                return;
            }
            assert!(
                p.seats.iter().all(|s| s.rect.x == 0 && s.rect.w == c),
                "{p:?}"
            );
            let at = p.seats.iter().position(|s| s.role == Role::Banner);
            assert!(matches!(at, Some(0 | 1)), "{p:?}");
            assert!(p.seats.len() <= 4, "{p:?}");
            if r >= 41 {
                assert_eq!(p.seats.len(), 4, "{p:?}");
            }
        });
    }

    #[test]
    fn a_wide_screen_is_two_columns() {
        each(|_, c, r, p| {
            if 2 * r > c {
                return;
            }
            let b = banner(p).rect;
            let log = p.seats.iter().find(|s| s.role == Role::Log).expect("a log");
            assert_eq!(
                (log.rect.x, log.rect.y, log.rect.w),
                (b.x, b.y + b.h, b.w),
                "{p:?}"
            );
            assert!(matches!(p.seats.len(), 3 | 4), "{p:?}");
        });
    }

    #[test]
    fn panes_are_big() {
        let big = |p: &Plan| {
            for s in &p.seats {
                if s.role != Role::Banner {
                    assert!(s.rect.w >= 30 && s.rect.h >= 7, "{p:?}");
                }
            }
        };
        each(|_, _, _, p| big(p));
        each_small(|_, _, _, p| big(p));
    }

    #[test]
    fn the_banner_has_room() {
        each(|_, c, r, p| {
            if r < 14 {
                return;
            }
            let b = banner(p).rect;
            let inside = b.h - 1;
            assert!(inside >= 13, "{p:?}");
            if c >= 60 {
                let fit = letrero::fit("ABCDEFGH", "ABCDEFGH", b.w - 1, b.h - 1).expect("fits");
                assert_eq!(fit.lines.len(), 2, "{fit:?}");
                assert!(fit.size >= Size::Narrow, "{fit:?}");
            }
        });
    }

    #[test]
    fn widgets_go_where_their_shape_fits() {
        each(|_, _, _, p| {
            let mut left = POOL.to_vec();
            for s in p.seats.iter().filter(|s| s.role == Role::Shape) {
                let (w, h) = (s.rect.w, s.rect.h);
                assert!(
                    suits(s.kind, w, h) || !left.iter().any(|&k| suits(k, w, h)),
                    "{p:?}"
                );
                left.retain(|&k| k != s.kind);
            }
        });
    }

    #[test]
    fn suits_by_the_table() {
        let yes = [
            (13, 67, 16),
            (13, 60, 13),
            (13, 38, 15),
            (12, 46, 16),
            (12, 67, 16),
            (11, 1, 100),
            (11, 200, 9),
            (10, 45, 34),
            (14, 60, 14),
            (14, 40, 20),
        ];
        let no = [
            (13, 60, 12),
            (13, 30, 30),
            (12, 38, 16),
            (10, 30, 30),
            (14, 40, 21),
            (14, 38, 15),
        ];
        for (k, w, h) in yes {
            assert!(suits(k, w, h), "{k} {w}x{h}");
        }
        for (k, w, h) in no {
            assert!(!suits(k, w, h), "{k} {w}x{h}");
        }
    }

    #[test]
    fn the_banner_band_by_width() {
        for (c, rows) in [
            (60, 20),
            (67, 20),
            (98, 20),
            (99, 29),
            (130, 29),
            (131, 37),
            (162, 37),
            (163, 46),
            (300, 46),
        ] {
            assert_eq!(banner_band(c), rows, "{c}");
        }
    }

    #[test]
    fn the_tables_of_examples() {
        const B: usize = 17;
        const L: usize = 8;
        const RADAR: usize = 13;
        const MAP: usize = 12;
        const RAIN: usize = 11;
        const BW: usize = 10;
        const PROG: usize = 14;
        // (kind, columns, rows)
        type Panes = &'static [(usize, i32, i32)];
        let rows: [((i32, i32), Panes); 7] = [
            (
                (60, 55),
                &[(B, 60, 20), (RADAR, 60, 13), (PROG, 60, 14), (L, 60, 7)],
            ),
            (
                (67, 60),
                &[(B, 67, 20), (RADAR, 67, 16), (MAP, 67, 16), (L, 67, 7)],
            ),
            (
                (120, 106),
                &[(B, 120, 29), (RADAR, 120, 31), (MAP, 120, 32), (L, 120, 13)],
            ),
            (
                (100, 30),
                &[(B, 62, 20), (L, 62, 9), (RADAR, 38, 15), (RAIN, 38, 14)],
            ),
            (
                (120, 33),
                &[(B, 74, 22), (L, 74, 10), (RADAR, 46, 16), (MAP, 46, 16)],
            ),
            (
                (213, 60),
                &[(B, 132, 45), (L, 132, 14), (RADAR, 81, 30), (MAP, 81, 29)],
            ),
            (
                (240, 35),
                &[(B, 149, 24), (L, 149, 10), (RAIN, 46, 34), (BW, 45, 34)],
            ),
        ];
        for ((w, h), want) in rows {
            let found = (0..2000).any(|s| {
                let p = plan(s, w, h - 1);
                let b = banner(&p).rect;
                // The main column top down, then the rest in reading order;
                // on a tall screen that is reading order.
                let mut seats = p.seats.clone();
                seats.sort_by_key(|s| (s.rect.x != b.x || s.rect.w != b.w, s.rect.y, s.rect.x));
                let got: Vec<_> = seats.iter().map(|s| (s.kind, s.rect.w, s.rect.h)).collect();
                got == want
            });
            assert!(found, "{w}x{h}: no seed below 2000 gives {want:?}");
        }
    }

    #[test]
    fn the_minimums_at_their_boundaries() {
        for s in 0..100 {
            // tall, 36 columns
            let p = plan(s, 36, 20);
            assert_eq!(p.seats.len(), 1, "{p:?}");
            assert_eq!(p.seats[0].role, Role::Banner);
            let p = plan(s, 36, 21);
            assert_eq!((p.seats.len(), count(&p, Role::Log)), (2, 1), "{p:?}");
            assert_eq!(shapes(&plan(s, 36, 29)), 0);
            assert_eq!(shapes(&plan(s, 36, 30)), 1);
            // tall, 60 columns
            assert_eq!(shapes(&plan(s, 60, 40)), 1);
            assert_eq!(shapes(&plan(s, 60, 41)), 2);
            // wide, 30 rows
            assert_eq!(shapes(&plan(s, 73, 30)), 0);
            assert_eq!(shapes(&plan(s, 74, 30)), 2);
            // wide, 120 columns
            assert_eq!(count(&plan(s, 120, 20), Role::Log), 0);
            assert_eq!(count(&plan(s, 120, 21), Role::Log), 1);
        }
    }

    #[test]
    fn the_same_seed_the_same_panes() {
        each(|s, c, r, p| {
            assert_eq!(*p, plan(s, c, r));
            for (i, a) in p.seats.iter().enumerate() {
                for b in &p.seats[i + 1..] {
                    assert!((a.rect.y, a.rect.x) < (b.rect.y, b.rect.x), "{p:?}");
                    assert_ne!(a.seed, b.seed, "{p:?}");
                }
            }
        });
    }

    #[test]
    fn it_returns_at_every_size() {
        let has_banner = |c: i32, r: i32, p: &Plan| {
            if c >= 3 && r >= 3 {
                assert_eq!(count(p, Role::Banner), 1, "{c}x{r}: {p:?}");
            }
        };
        each_small(|_, c, r, p| has_banner(c, r, p));
        for (c, r) in [(1, 1), (2, 3), (7, 2), (13, 5), (300, 89)] {
            for s in 0..100 {
                has_banner(c, r, &plan(s, c, r));
            }
        }
        assert!(plan(1, 0, 0).seats.is_empty());
        // Total over its inputs: the largest areas return too.
        for (c, r) in [
            (i32::MAX, i32::MAX),
            (i32::MAX, 3),
            (3, i32::MAX),
            (-5, i32::MAX),
        ] {
            let p = plan(7, c, r);
            assert!(p.seats.len() <= 4, "{c}x{r}: {p:?}");
        }
        assert_eq!(plan(7, i32::MAX, i32::MAX), plan(7, 65535, 65535));
    }
}
