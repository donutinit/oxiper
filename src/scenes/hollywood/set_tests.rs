//! hollywood in set mode (spec 10, 11.5): a layout that is a function of the
//! seed and the size and holds still, titles dealt once, a forced holiday's
//! signature widget in the first shape pane, borders born settled, and the
//! scene slower (`set_pace`) and dimmer (`set_dim`).

use super::foro::{self, Role};
use super::*;
use crate::scene::Opts;
use crate::theme::Theme;
use std::path::{Path, PathBuf};

/// What a set scene's context is made of.
struct Set {
    theme: Theme,
    o: Opts,
    fiesta: Fiesta,
    seed: u64,
    bg: Color,
}

impl Set {
    fn ctx(&self) -> Ctx<'_> {
        Ctx {
            theme: &self.theme,
            opts: &self.o,
            bg: self.bg,
            seed: self.seed,
            fiesta: self.fiesta,
        }
    }
}

/// hollywood on a film set with the options `opts`, built at `w`x`h` from
/// `seed` as `Player::make` builds it (new, resize, retheme).
fn seeded(seed: u64, opts: &str, fiesta: Fiesta, w: u16, h: u16) -> (Hollywood, Set) {
    let set = Set {
        theme: Theme::classic(),
        o: Opts::new(INFO.options, opts.parse().unwrap()).with_set_mode(true),
        fiesta,
        seed,
        bg: Color::Reset,
    };
    let ctx = set.ctx();
    let mut s = Hollywood::new(&ctx);
    s.resize(&ctx, w, h);
    s.retheme(&ctx);
    (s, set)
}

fn set_scene(opts: &str, fiesta: Fiesta, w: u16, h: u16) -> (Hollywood, Set) {
    seeded(11, opts, fiesta, w, h)
}

/// The example preset's file, as the `preset` option names it.
fn example() -> String {
    naming(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/presets/hollywood/software-libre.toml"),
    )
}

fn naming(path: &Path) -> String {
    format!("preset = '{}'", path.display())
}

/// A preset of two `titulos`, in a file of this test's own.
fn two_titles(name: &str) -> PathBuf {
    cache::temp(name, "titulos = ['Primer titulo', 'Segundo titulo']\n")
}

/// Every pane's rectangle and kind, in reading order.
fn layout(s: &Hollywood) -> Vec<(Rect, usize)> {
    s.rects().into_iter().zip(s.kinds()).collect()
}

fn titles(s: &Hollywood) -> Vec<String> {
    let mut v = Vec::new();
    if let Some(r) = &s.root {
        r.each(&mut |p| v.push(p.title.clone()));
    }
    v
}

fn names(s: &Hollywood) -> Vec<&'static str> {
    let mut v = Vec::new();
    if let Some(r) = &s.root {
        r.each(&mut |p| v.push(p.widget.name()));
    }
    v
}

/// The plan for a `w`x`h` terminal: its area is the terminal less the ticker's row.
fn plan_of(seed: u64, w: u16, h: u16) -> foro::Plan {
    foro::plan(seed, i32::from(w), i32::from(h) - 1)
}

fn draw(s: &mut Hollywood, ctx: &Ctx, w: u16, h: u16) -> Canvas {
    let mut c = Canvas::new(w, h, Color::Reset);
    s.draw(ctx, &mut c);
    c
}

/// Characters the ticker loses in `secs` seconds of `update` at 30 frames a second.
fn tape_moves(s: &mut Hollywood, ctx: &Ctx, secs: f32) -> usize {
    s.feed_tape();
    let before = s.tape.len();
    for _ in 0..(secs * 30.0) as usize {
        s.update(ctx, 1.0 / 30.0);
    }
    before - s.tape.len()
}

const HOLIDAYS: [Kind; 4] = [
    Kind::Muertos,
    Kind::Navidad,
    Kind::Patrias,
    Kind::Revolucion,
];

#[test]
fn the_layout_is_the_plans() {
    for (w, h) in foro::SIZES {
        for seed in [0, 7, 11, 4821] {
            let (s, _) = seeded(seed, "", Fiesta::NONE, w, h);
            let want: Vec<_> = plan_of(seed, w, h)
                .seats
                .iter()
                .map(|seat| (seat.rect, seat.kind))
                .collect();
            assert_eq!(layout(&s), want, "{w}x{h} seed {seed}");
        }
    }
}

#[test]
fn nothing_moves_in_ten_minutes() {
    for opts in ["", "panes = 12\nreshuffle = 1"] {
        for (w, h) in [(67, 60), (120, 33)] {
            let (mut s, set) = set_scene(opts, Fiesta::NONE, w, h);
            let ctx = set.ctx();
            let (first, named) = (layout(&s), titles(&s));
            for step in 0..600 {
                s.tick(&ctx, 0.5, 1.0);
                if step % 60 == 0 {
                    draw(&mut s, &ctx, w, h);
                }
            }
            assert_eq!(layout(&s), first, "{opts:?} {w}x{h}");
            assert_eq!(titles(&s), named, "{opts:?} {w}x{h}");
        }
    }
}

#[test]
fn a_preset_cannot_move_a_pane() {
    let other = two_titles("set-other");
    for (w, h) in [(67, 60), (120, 33), (240, 35), (60, 55)] {
        let plain = layout(&set_scene("", Fiesta::NONE, w, h).0);
        let (s, _) = set_scene(&example(), Fiesta::NONE, w, h);
        assert!(s.preset.is_some());
        assert_eq!(layout(&s), plain, "{w}x{h}: the example");
        let (s, _) = set_scene(&naming(&other), Fiesta::NONE, w, h);
        assert!(s.preset.is_some());
        assert_eq!(layout(&s), plain, "{w}x{h}: another preset");
    }
    std::fs::remove_file(&other).ok();
}

/// A live edit of the preset re-dresses the panes where they stand: the
/// layout never comes from the generator the words use (spec 10.3).
#[test]
fn an_edit_cannot_move_a_pane() {
    let path = cache::temp("set-edit", "titulos = ['Primera version']\n");
    let (mut s, set) = set_scene(&naming(&path), Fiesta::NONE, 67, 60);
    let ctx = set.ctx();
    assert_eq!(s.refresh(&ctx), None);
    for _ in 0..90 {
        s.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
        draw(&mut s, &ctx, 67, 60);
    }
    let first = layout(&s);
    std::fs::write(&path, "titulos = ['La segunda version', 'Otra mas']\n").unwrap();
    assert_eq!(s.refresh(&ctx), Some(Ok("preset reloaded".into())));
    assert_eq!(layout(&s), first);
    assert!(titles(&s)[..2].iter().any(|t| t.starts_with("La segunda")));
    s.resize(&ctx, 120, 33);
    assert_ne!(layout(&s), first);
    s.resize(&ctx, 67, 60);
    assert_eq!(layout(&s), first);
    std::fs::remove_file(&path).ok();
}

#[test]
fn a_resize_and_back() {
    let (mut s, set) = set_scene("", Fiesta::NONE, 67, 60);
    let ctx = set.ctx();
    let first = layout(&s);
    s.resize(&ctx, 120, 33);
    let want: Vec<_> = plan_of(11, 120, 33)
        .seats
        .iter()
        .map(|seat| (seat.rect, seat.kind))
        .collect();
    assert_eq!(layout(&s), want);
    s.resize(&ctx, 67, 60);
    assert_eq!(layout(&s), first);
}

#[test]
fn titles_are_dealt_by_position() {
    let given = preset_tests::example().titles;
    let (s, _) = set_scene(&example(), Fiesta::NONE, 67, 60);
    let shown = titles(&s);
    assert_eq!(shown.len(), 4);
    let mut dealt: Vec<&str> = shown
        .iter()
        .zip(names(&s))
        .map(|(t, name)| {
            *given
                .iter()
                .find(|g| *t == format!("{g} \u{00b7} {name}"))
                .unwrap_or_else(|| panic!("{t:?}"))
        })
        .collect();
    assert_eq!(
        titles(&set_scene(&example(), Fiesta::NONE, 67, 60).0),
        shown
    );
    dealt.sort_unstable();
    dealt.dedup();
    assert_eq!(dealt.len(), 4, "{shown:?}");
    // Two `titulos`: the first two panes have them, the others a generated one.
    let path = two_titles("set-two");
    let (s, _) = set_scene(&naming(&path), Fiesta::NONE, 67, 60);
    let shown = titles(&s);
    let names = names(&s);
    let mut first: Vec<String> = shown[..2]
        .iter()
        .zip(&names)
        .map(|(t, name)| {
            t.strip_suffix(&format!(" \u{00b7} {name}"))
                .unwrap_or_else(|| panic!("{t:?}"))
                .to_string()
        })
        .collect();
    first.sort();
    assert_eq!(first, ["Primer titulo", "Segundo titulo"]);
    for (t, name) in shown[2..].iter().zip(&names[2..]) {
        let words = t.strip_prefix(&format!("{name} \u{00b7} "));
        assert!(words.is_some_and(|w| !w.is_empty()), "{t:?}");
    }
    std::fs::remove_file(&path).ok();
}

#[test]
fn a_forced_holiday_gets_its_signature() {
    for kind in HOLIDAYS {
        let fiesta = Fiesta::height(kind);
        assert_eq!(fiesta.kind, kind);
        for (w, h) in [(67, 60), (120, 33)] {
            for seed in 0..50 {
                let (s, _) = seeded(seed, "", fiesta, w, h);
                let kinds = s.kinds();
                assert!(kinds.contains(&foro::BANNER) && kinds.contains(&foro::LOG));
                let plan = plan_of(seed, w, h);
                let at = plan.seats.iter().position(|s| s.role == Role::Shape);
                let at = at.expect("a shape seat");
                assert!(
                    signature(fiesta).contains(&kinds[at]),
                    "{kind:?} {w}x{h} seed {seed}: {kinds:?}"
                );
                let mut once = kinds.clone();
                once.sort_unstable();
                once.dedup();
                assert_eq!(once.len(), kinds.len(), "{kind:?} seed {seed}: {kinds:?}");
            }
        }
        // With a preset the holiday is ignored inside hollywood.
        for seed in 0..50 {
            let (s, _) = seeded(seed, &example(), fiesta, 67, 60);
            for k in s.kinds() {
                assert!(
                    k == foro::BANNER || k == foro::LOG || foro::POOL.contains(&k),
                    "{kind:?} seed {seed}: {k}"
                );
            }
        }
    }
}

#[test]
fn no_border_is_bright_at_birth() {
    for fiesta in [Fiesta::NONE, Fiesta::height(Kind::Navidad)] {
        for (w, h) in [(67, 60), (120, 33)] {
            let (mut s, set) = set_scene("", fiesta, w, h);
            let ctx = set.ctx();
            let c = draw(&mut s, &ctx, w, h);
            let settled = s.env.pal.fade(s.env.pal.main, 0.45);
            for r in s.rects() {
                let cell = c.get(r.x, r.y).unwrap();
                assert_eq!(cell.ch, '\u{250c}', "{r:?}");
                assert_eq!(cell.fg, settled, "{w}x{h} {r:?}");
            }
        }
    }
}

#[test]
fn set_dim_fades_every_tone() {
    const TONES: [Tone; 7] = [
        Tone::Main,
        Tone::Dim,
        Tone::Bright,
        Tone::Cyan,
        Tone::Amber,
        Tone::Red,
        Tone::Green,
    ];
    let dimmed = |opts: &str| {
        let (mut s, mut set) = set_scene(opts, Fiesta::NONE, 67, 60);
        set.bg = Color::Rgb(16, 16, 24);
        let ctx = set.ctx();
        s.retheme(&ctx);
        (s.env.pal.clone(), Pal::new(&ctx))
    };
    let (pal, plain) = dimmed("set_dim = 0");
    for t in TONES {
        assert_eq!(pal.tone(t), plain.tone(t), "{t:?}");
    }
    let (pal, plain) = dimmed("set_dim = 0.25");
    for t in TONES {
        assert!(
            pal.tone(t).luma() < plain.tone(t).luma(),
            "{t:?}: {:?} {:?}",
            pal.tone(t),
            plain.tone(t)
        );
    }
    assert!(pal.ramp.at(0.9).luma() < plain.ramp.at(0.9).luma());
}

#[test]
fn set_pace_halves_the_ticker() {
    for (opts, want) in [("", 70), ("set_pace = 1", 140)] {
        let (mut s, set) = set_scene(opts, Fiesta::NONE, 67, 60);
        let moved = tape_moves(&mut s, &set.ctx(), 10.0);
        assert!(moved.abs_diff(want) <= 1, "{opts:?}: {moved}");
    }
}

#[test]
fn pace_and_dim_out_of_range_are_clamped() {
    let opts = ["0", "-1", "99", "nan"]
        .map(|v| format!("set_pace = {v}"))
        .into_iter()
        .chain(["-1", "5", "nan"].map(|v| format!("set_dim = {v}")));
    for opts in opts {
        let (mut s, set) = set_scene(&opts, Fiesta::NONE, 67, 60);
        let ctx = set.ctx();
        assert!(0.1 <= s.pace && s.pace <= 1.0, "{opts}: {}", s.pace);
        assert!(0.0 <= s.dim && s.dim <= 0.9, "{opts}: {}", s.dim);
        assert!(tape_moves(&mut s, &ctx, 10.0) > 0, "{opts}");
        let c = draw(&mut s, &ctx, 67, 60);
        assert!(c.cells().iter().any(|x| x.ch != ' '), "{opts}");
    }
}

#[test]
fn set_mode_survives_stress() {
    let sizes: Vec<(u16, u16)> = [(1, 1), (2, 3), (7, 2), (13, 5), (300, 90)]
        .into_iter()
        .chain(foro::SIZES)
        .collect();
    let fiestas = std::iter::once(Fiesta::NONE).chain(HOLIDAYS.map(Fiesta::height));
    std::thread::scope(|scope| {
        for fiesta in fiestas {
            for opts in [String::new(), example()] {
                let sizes = &sizes;
                scope.spawn(move || {
                    for &(w, h) in sizes {
                        let (mut s, set) = set_scene(&opts, fiesta, w, h);
                        let ctx = set.ctx();
                        let mut c = Canvas::new(w, h, Color::Reset);
                        for _ in 0..240 {
                            s.tick(&ctx, 1.0 / 30.0, 1.0 / 30.0);
                            c.clear();
                            s.draw(&ctx, &mut c);
                        }
                        for _ in 0..3 {
                            s.tick(&ctx, 5.0, 5.0);
                            c.clear();
                            s.draw(&ctx, &mut c);
                        }
                    }
                });
            }
        }
    });
}
