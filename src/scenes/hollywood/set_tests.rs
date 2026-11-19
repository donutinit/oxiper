//! hollywood in set mode (spec 10, 11.5): a layout that is a function of the
//! seed and the size and holds still, titles dealt once, a forced holiday's
//! signature widget in the first shape pane, borders born settled, and the
//! scene slower (`set_pace`) and dimmer (`set_dim`), and a log that comes a
//! line at a time, every third sentence one of the preset's `frases`.

use super::feed::{Feed, FeedKind};
use super::foro::{self, Role};
use super::*;
use crate::scene::Opts;
use crate::theme::{TermColors, Theme};
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
    let dimmed = |opts: &str, bg: Color| {
        let (mut s, mut set) = set_scene(opts, Fiesta::NONE, 67, 60);
        set.bg = bg;
        let ctx = set.ctx();
        s.retheme(&ctx);
        (s.env.pal.clone(), Pal::new(&ctx))
    };
    let (pal, plain) = dimmed("set_dim = 0", Color::Rgb(16, 16, 24));
    for t in TONES {
        assert_eq!(pal.tone(t), plain.tone(t), "{t:?}");
    }
    let (pal, plain) = dimmed("set_dim = 0.25", Color::Rgb(16, 16, 24));
    for t in TONES {
        assert!(
            pal.tone(t).luma() < plain.tone(t).luma(),
            "{t:?}: {:?} {:?}",
            pal.tone(t),
            plain.tone(t)
        );
    }
    assert!(pal.ramp.at(0.9).luma() < plain.ramp.at(0.9).luma());
    // `ink` is darker than that background, and a fade toward it brightens
    // `ink`; toward black every color is darker.
    let (pal, plain) = dimmed("set_dim = 0.25", Color::Rgb(0, 0, 0));
    let more = |p: &Pal| [p.ink, p.oro, p.flor, p.vela, p.picado[0], p.papel[0]];
    for (i, (dim, was)) in more(&pal).into_iter().zip(more(&plain)).enumerate() {
        assert!(dim.luma() < was.luma(), "{i}: {dim:?} {was:?}");
    }
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

/// Spec 11.5: every pane's widget is the one its seat's seed makes, with no
/// preset and with one; a widget seeded from anything else draws otherwise.
#[test]
fn every_widget_comes_from_its_seat() {
    for opts in [String::new(), example()] {
        let (mut s, _) = set_scene(&opts, Fiesta::NONE, 67, 60);
        let plan = plan_of(11, 67, 60);
        let (level, garb) = (s.env.level, s.garb());
        let env = &s.env;
        let drawn = |wd: &mut dyn Widget, w: u16, h: u16| {
            let mut c = Canvas::new(w, h, Color::Reset);
            let r = Rect {
                x: 0,
                y: 0,
                w: i32::from(w),
                h: i32::from(h),
            };
            wd.draw(&mut c, r, env);
            c
        };
        let mut at = 0;
        let root = s.root.as_mut().expect("seated");
        root.each_mut(&mut |p| {
            let seat = &plan.seats[at];
            at += 1;
            assert_eq!(p.kind, seat.kind, "{opts:?} pane {at}");
            let (w, h) = p.content;
            let mut made = make_widget(seat.kind, seat.seed, level, garb);
            let mut other = make_widget(seat.kind, seat.seed ^ 1, level, garb);
            made.resize(w, h);
            other.resize(w, h);
            for _ in 0..30 {
                p.widget.update(1.0 / 60.0, env);
                made.update(1.0 / 60.0, env);
                other.update(1.0 / 60.0, env);
            }
            let shown = drawn(p.widget.as_mut(), w, h);
            assert!(
                shown.cells() == drawn(made.as_mut(), w, h).cells(),
                "{opts:?} pane {at}: {}",
                p.widget.name()
            );
            assert!(
                shown.cells() != drawn(other.as_mut(), w, h).cells(),
                "{opts:?} pane {at}: {} is the same from another seed",
                p.widget.name()
            );
        });
        assert_eq!(at, plan.seats.len());
    }
}

/// The text of a 200-column log's last line, as drawn.
fn last_line(f: &mut Feed, env: &Env) -> String {
    let mut c = Canvas::new(200, 1, Color::Reset);
    let r = Rect {
        x: 0,
        y: 0,
        w: 200,
        h: 1,
    };
    f.draw(&mut c, r, env);
    let row: String = (0..200)
        .map(|x| c.get(x, 0).map_or(' ', |cell| cell.ch))
        .collect();
    row.trim_end().to_string()
}

/// Spec 10.2 and 11.5: on the set the mission log and the logins bring one
/// line a scene second, never two in a frame, whatever the frame's length.
#[test]
fn the_calm_log_brings_one_line_at_a_time() {
    let (s, set) = set_scene("", Fiesta::NONE, 67, 60);
    assert!(s.garb().calm);
    let off = Opts::new(INFO.options, toml::Table::new());
    let ctx = Ctx {
        opts: &off,
        ..set.ctx()
    };
    assert!(!Hollywood::new(&ctx).garb().calm);
    for kind in [FeedKind::Mission, FeedKind::Auth] {
        for seed in [1, 7, 4821] {
            let calm = Garb::plain(Fiesta::NONE).with_calm(true);
            let mut f = Feed::new(kind, seed, 7, calm);
            f.resize(60, 20);
            // A minute on the wall at 30 frames a second and the default pace.
            for frame in 0..1_800 {
                let before = f.born();
                f.update(1.0 / 60.0, &s.env);
                assert!(f.born() - before <= 1, "{kind:?} seed {seed} frame {frame}");
            }
            let born = f.born();
            assert!((20..=40).contains(&born), "{kind:?} seed {seed}: {born}");
            f.update(5.0, &s.env);
            assert_eq!(f.born(), born + 1, "{kind:?} seed {seed}");
        }
    }
}

/// Spec 10.2 and 11.5: under a preset, every third sentence of the set's
/// log is the next of its `frases`, in a shuffled round. They are also among
/// the preset's canned lines (PF13), so the round is checked by position.
#[test]
fn one_sentence_in_three_is_a_frase() {
    const FRASES: [&str; 4] = [
        "Primera frase del guion",
        "Segunda frase del guion",
        "Tercera frase del guion",
        "Cuarta frase del guion",
    ];
    let text = format!("frases = '''\n{}\n'''\n", FRASES.join("\n"));
    let p = match preset::parse(text.as_bytes()) {
        Ok(l) => l.preset,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(p.phrases, FRASES);
    let (s, _) = set_scene("", Fiesta::NONE, 67, 60);
    for seed in [3, 11, 4821] {
        let mut f = Feed::new(FeedKind::Mission, seed, 7, Garb::preset(p).with_calm(true));
        f.resize(200, 1);
        // Every line, from the first the log wrote; a line that does not
        // start with the clock goes on the sentence before it.
        let mut entries: Vec<String> = Vec::new();
        let mut sentences = 0;
        let mut row = last_line(&mut f, &s.env);
        while sentences < 301 && entries.len() < 5_000 {
            match row.strip_prefix("[T+").and_then(|r| r.split_once("] ")) {
                Some((_, rest)) => {
                    let rest = rest.strip_prefix("[ OK ] ").unwrap_or(rest);
                    if !rest.starts_with(p.vocab.phase) {
                        sentences += 1;
                    }
                    entries.push(rest.to_string());
                }
                None => match entries.last_mut() {
                    Some(e) => {
                        e.push(' ');
                        e.push_str(row.trim());
                    }
                    None => panic!("seed {seed}: {row:?}"),
                },
            }
            let before = f.born();
            f.update(1.0, &s.env);
            assert_eq!(f.born(), before + 1);
            row = last_line(&mut f, &s.env);
        }
        let sentences: Vec<&str> = entries
            .iter()
            .map(String::as_str)
            .filter(|e| !e.starts_with(p.vocab.phase))
            .take(300)
            .collect();
        assert_eq!(sentences.len(), 300, "seed {seed}");
        let thirds: Vec<&str> = sentences.iter().skip(2).step_by(3).copied().collect();
        for (i, t) in thirds.iter().enumerate() {
            assert!(
                FRASES.contains(t),
                "seed {seed}: sentence {}: {t:?}",
                3 * i + 3
            );
        }
        let mut want = FRASES.to_vec();
        want.sort_unstable();
        for four in thirds.as_chunks::<4>().0 {
            let mut got = four.to_vec();
            got.sort_unstable();
            assert_eq!(got, want, "seed {seed}: {four:?}");
        }
        let phrases = sentences.iter().filter(|e| FRASES.contains(e)).count();
        assert!(
            (75..=120).contains(&phrases),
            "seed {seed}: {phrases} of 300"
        );
    }
}

/// A feed that is not calm is the feed it was: the same lines, frame by frame.
#[test]
fn a_feed_that_is_not_calm_is_as_it_was() {
    let (s, _) = set_scene("", Fiesta::NONE, 67, 60);
    let drawn = |f: &mut Feed| {
        let mut c = Canvas::new(60, 20, Color::Reset);
        let r = Rect {
            x: 0,
            y: 0,
            w: 60,
            h: 20,
        };
        f.draw(&mut c, r, &s.env);
        c
    };
    let example = preset_tests::example();
    let dressed: [(Garb, Garb); 2] = [
        (Fiesta::NONE.into(), Garb::plain(Fiesta::NONE)),
        (Garb::preset(example), Garb::preset(example)),
    ];
    for (bare, garb) in dressed {
        for kind in [FeedKind::Mission, FeedKind::Auth, FeedKind::Log] {
            let mut was = Feed::new(kind, 9, 7, bare);
            let mut is = Feed::new(kind, 9, 7, garb.with_calm(false));
            was.resize(60, 20);
            is.resize(60, 20);
            // Ten simulated seconds at 30 frames a second.
            for frame in 0..300 {
                was.update(1.0 / 30.0, &s.env);
                is.update(1.0 / 30.0, &s.env);
                assert!(
                    drawn(&mut was).cells() == drawn(&mut is).cells(),
                    "{kind:?} frame {frame}"
                );
            }
            assert_eq!(was.born(), is.born());
        }
    }
}

// ------------------------------------------------------------------ the calm banner (spec 10.5)

/// The terminal's background in the banner's tests: known, so fades blend.
const BG: Color = Color::Rgb(16, 16, 24);
/// A frame on the scene's clock: 30 frames a second at the default pace.
const FRAME: f32 = 1.0 / 60.0;
/// The banner's pane: the banner band of a 67x60 screen.
const PANE: Rect = Rect {
    x: 0,
    y: 0,
    w: 66,
    h: 19,
};

/// What a set scene's widgets read, under `theme` on a terminal whose
/// background is `bg`.
fn set_env(theme: Theme, bg: Color, fiesta: Fiesta) -> Env {
    let set = Set {
        theme,
        o: Opts::new(INFO.options, toml::Table::new()).with_set_mode(true),
        fiesta,
        seed: 11,
        bg,
    };
    let ctx = set.ctx();
    let mut s = Hollywood::new(&ctx);
    s.resize(&ctx, 67, 60);
    s.retheme(&ctx);
    Env {
        pal: s.env.pal.clone(),
        level: 7,
        day: 0.0,
    }
}

/// The access pane on the set, dressed by `garb`, in a 66x19 pane.
fn calm_access(seed: u64, garb: Garb) -> widgets::Access {
    let mut a = widgets::Access::new(seed, 7, garb.with_calm(true));
    a.resize(66, 19);
    a
}

/// The pane as drawn on a background of `BG`.
fn pane(w: &mut dyn Widget, env: &Env) -> Canvas {
    let mut c = Canvas::new(PANE.w as u16, PANE.h as u16, BG);
    w.draw(&mut c, PANE, env);
    c
}

/// Each channel of a color known by its RGB.
fn channels(c: Color) -> [i32; 3] {
    match c {
        Color::Rgb(r, g, b) => [r, g, b].map(i32::from),
        other => panic!("{other:?} is no RGB color"),
    }
}

/// Does each channel of `from -> to` move by no more than a fifth of its
/// distance between `a` and `b`, plus 1 for rounding?
fn a_fifth(from: Color, to: Color, a: Color, b: Color) -> bool {
    let (from, to, a, b) = (channels(from), channels(to), channels(a), channels(b));
    (0..3).all(|i| (to[i] - from[i]).abs() <= (b[i] - a[i]).abs() / 5 + 1)
}

/// The banners a run of strengths shows: each one's first and last frame.
fn shows(strength: &[Option<f32>]) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    for (i, s) in strength.iter().enumerate() {
        if s.is_some() {
            match v.last_mut() {
                Some(last) if last.1 + 1 == i => last.1 = i,
                _ => v.push((i, i)),
            }
        }
    }
    v
}

/// The box of a banner at full strength: the cells whose background is
/// not the terminal's.
fn inked(c: &Canvas) -> Option<Rect> {
    let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, -1, -1);
    for y in 0..c.h() as i32 {
        for x in 0..c.w() as i32 {
            if c.get(x, y).is_some_and(|cell| cell.bg != BG) {
                (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
            }
        }
    }
    (x1 >= 0).then(|| Rect {
        x: x0,
        y: y0,
        w: x1 - x0 + 1,
        h: y1 - y0 + 1,
    })
}

fn cells_of(r: Rect) -> impl Iterator<Item = (i32, i32)> {
    (r.y..r.y + r.h).flat_map(move |y| (r.x..r.x + r.w).map(move |x| (x, y)))
}

/// The garbs the banner's tests run in: no preset, a holiday, the example.
fn banner_garbs() -> [Garb; 3] {
    [
        Garb::plain(Fiesta::NONE),
        Garb::plain(Fiesta::height(Kind::Muertos)),
        Garb::preset(preset_tests::example()),
    ]
}

#[test]
fn the_calm_banner_keeps_its_times() {
    let env = set_env(Theme::classic(), BG, Fiesta::NONE);
    for garb in banner_garbs() {
        for seed in [1, 7, 4821] {
            let mut a = calm_access(seed, garb);
            let strength: Vec<Option<f32>> = (0..(120.0 / FRAME) as usize)
                .map(|_| {
                    a.update(FRAME, &env);
                    a.strength()
                })
                .collect();
            let shows = shows(&strength);
            assert!(shows.len() >= 8, "seed {seed}: {shows:?}");
            assert!((shows[0].0 as f32) * FRAME < 2.0, "seed {seed}: {shows:?}");
            let near =
                |frames: usize, secs: f32| (frames as f32 * FRAME - secs).abs() <= 1.5 * FRAME;
            // The last one may still be up when the two minutes end.
            for &(first, last) in &shows[..shows.len() - 1] {
                let s: Vec<f32> = strength[first..=last].iter().map(|s| s.unwrap()).collect();
                assert!(
                    s.iter().all(|k| (0.0..=1.0).contains(k)),
                    "seed {seed}: {s:?}"
                );
                let full = s.iter().filter(|&&k| k == 1.0).count();
                let stay = full as f32 * FRAME;
                assert!(
                    (6.0 - 1.5 * FRAME..=10.0 + 1.5 * FRAME).contains(&stay),
                    "seed {seed}: stays {stay}"
                );
                let into = s.iter().position(|&k| k == 1.0).unwrap();
                let out = s.len() - 1 - s.iter().rposition(|&k| k == 1.0).unwrap();
                assert!(
                    near(into, 0.5) && near(out, 0.5),
                    "seed {seed}: {into} {out}"
                );
                assert_eq!(into + full + out, s.len(), "seed {seed}: it stays at 1");
                assert!(
                    s[..=into].is_sorted() && s[s.len() - out - 1..].is_sorted_by(|a, b| a >= b)
                );
            }
            for pair in shows.windows(2) {
                let gap = (pair[1].0 - pair[0].1 - 1) as f32 * FRAME;
                assert!(
                    (2.0 - 1.5 * FRAME..=4.0 + 1.5 * FRAME).contains(&gap),
                    "seed {seed}: a gap of {gap}"
                );
            }
        }
    }
}

/// One banner on the set, frame by frame: its strength and the pane as
/// drawn, from the frame before it to the frame after it.
struct Show {
    strength: Vec<Option<f32>>,
    frames: Vec<Canvas>,
    /// The log alone, as the pane's own feed draws it, frame by frame.
    log: Vec<Canvas>,
}

/// The `n`th banner (from 0) of `seed` dressed by `garb`.
fn show(seed: u64, garb: Garb, n: usize, env: &Env) -> Show {
    let mut a = calm_access(seed, garb);
    let mut log = Feed::new(FeedKind::Auth, seed, 7, garb.with_calm(true));
    log.resize(66, 19);
    let mut out = Show {
        strength: Vec::new(),
        frames: Vec::new(),
        log: Vec::new(),
    };
    let (mut ended, mut was, mut before) = (0, None, None);
    loop {
        a.update(FRAME, env);
        log.update(FRAME, env);
        let s = a.strength();
        if was.is_some() && s.is_none() {
            ended += 1;
        }
        was = s;
        let (frame, logged) = (pane(&mut a, env), pane(&mut log, env));
        if ended < n {
            continue;
        }
        if ended == n && s.is_none() {
            before = Some((frame, logged));
            continue;
        }
        if out.frames.is_empty() {
            let (f, l) = before.take().expect("a frame before the banner");
            out.strength.push(None);
            out.frames.push(f);
            out.log.push(l);
        }
        out.strength.push(s);
        out.frames.push(frame);
        out.log.push(logged);
        if ended > n {
            return out;
        }
    }
}

/// Runs `check` on the `n`th banner of `seed` for each `(seed, n)` of
/// `cases`, under each of `banner_garbs`, a thread each.
fn each_show(cases: &[(u64, usize)], check: impl Fn(&Show, &Env, u64) + Sync) {
    std::thread::scope(|scope| {
        for garb in banner_garbs() {
            for &(seed, n) in cases {
                let check = &check;
                scope.spawn(move || {
                    let env = set_env(Theme::classic(), BG, Fiesta::NONE);
                    check(&show(seed, garb, n, &env), &env, seed);
                });
            }
        }
    });
}

#[test]
fn the_calm_banner_eases() {
    each_show(
        &[(1, 0), (7, 1), (4821, 2), (5, 0), (5, 1)],
        |s, env, seed| {
            let ink = env.pal.ink;
            let full = s.strength.iter().position(|&k| k == Some(1.0)).unwrap();
            let at_full = &s.frames[full];
            let bx = inked(at_full).expect("a box at full strength");
            let boxed = |i: usize| s.strength[i].is_some_and(|k| k >= 0.5);
            for i in 1..s.frames.len() {
                let (was, is) = (&s.frames[i - 1], &s.frames[i]);
                for (x, y) in cells_of(bx) {
                    let (a, b) = (was.get(x, y).unwrap(), is.get(x, y).unwrap());
                    assert!(
                        a_fifth(a.bg, b.bg, BG, ink),
                        "seed {seed} frame {i} ({x}, {y}): {:?} to {:?}",
                        a.bg,
                        b.bg
                    );
                    // The letters, the frame and the captions, from the
                    // background to their colors and back.
                    let last = at_full.get(x, y).unwrap();
                    if last.ch == ' ' {
                        continue;
                    }
                    let (from, to) = match (boxed(i - 1), boxed(i)) {
                        (true, true) => (a.fg, b.fg),
                        (false, true) => (BG, b.fg),
                        (true, false) => (a.fg, BG),
                        (false, false) => continue,
                    };
                    assert!(
                        a_fifth(from, to, BG, last.fg),
                        "seed {seed} frame {i} ({x}, {y}) {:?}: {from:?} to {to:?}",
                        last.ch
                    );
                }
            }
            // The first frame with a box has it at less than a fifth.
            let first = (0..s.frames.len()).find(|&i| boxed(i)).unwrap();
            let corner = s.frames[first].get(bx.x, bx.y).unwrap();
            let last = at_full.get(bx.x, bx.y).unwrap();
            assert_eq!(corner.ch, last.ch);
            assert!(a_fifth(BG, corner.fg, BG, last.fg), "seed {seed}");
            assert!(a_fifth(BG, corner.bg, BG, ink), "seed {seed}");
        },
    );
}

#[test]
fn the_log_goes_first() {
    each_show(&[(1, 0), (7, 1), (4821, 2)], |s, env, seed| {
        let pal = &env.pal;
        let full = s.strength.iter().position(|&k| k == Some(1.0)).unwrap();
        let bx = inked(&s.frames[full]).expect("a box at full strength");
        let mut faded = 0;
        for i in 1..s.frames.len() {
            let Some(k) = s.strength[i].filter(|&k| k < 0.5) else {
                continue;
            };
            let (frame, log) = (&s.frames[i], &s.log[i]);
            for y in 0..PANE.h {
                for x in 0..PANE.w {
                    let (cell, under) = (frame.get(x, y).unwrap(), log.get(x, y).unwrap());
                    let at = format!("seed {seed} frame {i} ({x}, {y})");
                    assert_eq!(cell.bg, s.frames[0].get(x, y).unwrap().bg, "{at}");
                    assert_eq!(cell.ch, under.ch, "{at}");
                    let inside = x >= bx.x && x < bx.x + bx.w && y >= bx.y && y < bx.y + bx.h;
                    if inside && under.ch != ' ' {
                        assert_eq!(cell.fg, pal.fade(under.fg, 2.0 * k), "{at}");
                        faded += 1;
                    } else {
                        assert_eq!(cell, under, "{at}");
                    }
                }
            }
            // Coming in, fainter each frame.
            let prev = s.strength[i - 1];
            if i < full {
                assert!(prev.is_none_or(|p| p < k), "seed {seed} frame {i}");
            }
        }
        assert!(faded > 0, "seed {seed}: no log under the banner");
    });
}

/// The box of the banner `words` with `captions` fitted to the pane by
/// `letrero`, as `pal` draws it at full strength in `col`.
fn fitted_box(words: [&str; 2], captions: (&str, &str), col: Color, pal: &Pal) -> (Rect, Canvas) {
    let fit = letrero::fit(words[0], words[1], PANE.w, PANE.h).unwrap();
    let mut c = Canvas::new(PANE.w as u16, PANE.h as u16, BG);
    let look = letrero::Look {
        letters: col,
        frame: col,
        caption: pal.bright,
        ink: Some(pal.ink),
    };
    letrero::draw(&mut c, PANE, &fit, captions, &look);
    let r = Rect {
        x: (PANE.w - fit.w) / 2,
        y: (PANE.h - fit.h) / 2,
        w: fit.w,
        h: fit.h,
    };
    (r, c)
}

/// Each banner of `seed` at full strength, until both colors were seen:
/// granted or not, and the pane.
fn at_full(seed: u64, garb: Garb, env: &Env) -> Vec<(bool, Canvas)> {
    let mut a = calm_access(seed, garb);
    let mut seen = Vec::new();
    let mut was = None;
    for _ in 0..(600.0 / FRAME) as usize {
        a.update(FRAME, env);
        let s = a.strength();
        if s == Some(1.0) && was != Some(1.0) {
            let c = pane(&mut a, env);
            let bx = inked(&c).expect("a box");
            let granted = c.get(bx.x, bx.y).unwrap().fg == env.pal.green;
            seen.push((granted, c));
            if seen.iter().any(|s| s.0) && seen.iter().any(|s| !s.0) {
                return seen;
            }
        }
        was = s;
    }
    panic!("seed {seed}: not both colors in ten minutes");
}

#[test]
fn the_set_banner_is_fitted_without_a_preset() {
    for fiesta in [Fiesta::NONE, Fiesta::height(Kind::Navidad)] {
        let env = set_env(Theme::classic(), BG, fiesta);
        let pal = &env.pal;
        let dress = lexico::of(fiesta).map(|v| &v.dress);
        assert_eq!(dress.is_some(), fiesta.is(Kind::Navidad));
        for seed in [1, 7, 4821] {
            for (granted, c) in at_full(seed, Garb::plain(fiesta), &env) {
                let col = if granted { pal.green } else { pal.red };
                let (words, captions) = match (dress, granted) {
                    (Some(d), true) => ([d.granted[0], d.granted[1]], (d.granted[2], d.granted[3])),
                    (Some(d), false) => ([d.denied[0], d.denied[1]], (d.denied[2], d.denied[3])),
                    (None, true) => (["ACCESS", "GRANTED"], ("", "")),
                    (None, false) => (["ACCESS", "DENIED"], ("", "")),
                };
                let bx = inked(&c).unwrap();
                if dress.is_none() {
                    assert_eq!(
                        (bx.w, bx.h),
                        (if granted { 58 } else { 50 }, 13),
                        "seed {seed}"
                    );
                }
                let (want, drawn) = fitted_box(words, captions, col, pal);
                assert_eq!(bx, want, "{:?} seed {seed} {words:?}", fiesta.kind);
                for (x, y) in cells_of(bx) {
                    assert_eq!(
                        c.get(x, y),
                        drawn.get(x, y),
                        "{:?} seed {seed} {words:?} ({x}, {y})",
                        fiesta.kind
                    );
                }
            }
        }
    }
}

#[test]
fn a_palette_without_rgb_shows_whole_banners() {
    let env = set_env(
        Theme::terminal(&TermColors::default()),
        Color::Reset,
        Fiesta::NONE,
    );
    let pal = &env.pal;
    for (w, h) in [(66, 19), (20, 5)] {
        let mut a = widgets::Access::new(7, 7, Garb::plain(Fiesta::NONE).with_calm(true));
        a.resize(w as u16, h as u16);
        let r = Rect { x: 2, y: 3, w, h };
        let mut c = Canvas::new(w as u16 + 4, h as u16 + 6, Color::Reset);
        let mut whole = false;
        for frame in 0..(60.0 / FRAME) as usize {
            a.update(FRAME, &env);
            c.clear();
            a.draw(&mut c, r, &env);
            for y in 0..c.h() as i32 {
                for x in 0..c.w() as i32 {
                    let inside = x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h;
                    if !inside {
                        let cell = c.get(x, y).unwrap();
                        assert!(
                            cell.ch == ' ' && cell.bg == Color::Reset,
                            "{w}x{h} frame {frame}"
                        );
                    }
                }
            }
            whole |= a.strength() == Some(1.0);
            // Without RGB a fade leaves a theme's color as it is: whole.
            if a.strength().is_some_and(|k| k >= 0.5) {
                let mut frames = c.cells().iter().filter(|cell| {
                    matches!(cell.ch, '\u{2550}' | '\u{2551}')
                        && (cell.fg == pal.green || cell.fg == pal.red)
                });
                assert!(frames.next().is_some(), "{w}x{h} frame {frame}");
            }
        }
        assert!(whole, "{w}x{h}");
    }
}
