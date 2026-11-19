//! hollywood with a preset (spec 8.3, 11.3): a preset dresses every ordinary
//! widget as a holiday at its height would, in its own words and no English
//! prose, with call signs where a number stands beside a name, and it wins
//! over the holiday.

use super::feed::{self, Feed, FeedKind};
use super::garb::Garb;
use super::neutro::BASE;
use super::preset::{self, Preset};
use super::*;
use crate::theme::Theme;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// The example preset of the docs, parsed once.
pub(super) fn example() -> &'static Preset {
    static P: OnceLock<&'static Preset> = OnceLock::new();
    P.get_or_init(|| {
        parsed(include_bytes!(
            "../../../docs/presets/hollywood/software-libre.toml"
        ))
    })
}

/// An empty preset: the neutral base and nothing else.
pub(super) fn bare() -> &'static Preset {
    static P: OnceLock<&'static Preset> = OnceLock::new();
    P.get_or_init(|| parsed(b""))
}

fn parsed(bytes: &[u8]) -> &'static Preset {
    match preset::parse(bytes) {
        Ok(l) => l.preset,
        Err(e) => panic!("{e}"),
    }
}

/// The example preset's file, as the `preset` option names it.
fn example_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/presets/hollywood/software-libre.toml")
}

/// `preset = '<path>'`.
fn naming(path: &Path) -> String {
    format!("preset = '{}'", path.display())
}

/// hollywood with the options `opts` on the day `fiesta`, built at `w`x`h` as
/// `Player::make` builds it (new, resize, retheme), handed to `f`.
fn scene(opts: &str, fiesta: Fiesta, w: u16, h: u16, f: impl FnOnce(&mut Hollywood, &Ctx)) {
    let theme = Theme::classic();
    let o = crate::scene::Opts::new(INFO.options, opts.parse().unwrap());
    let ctx = Ctx {
        theme: &theme,
        opts: &o,
        bg: Color::Reset,
        seed: 11,
        fiesta,
    };
    let mut s = Hollywood::new(&ctx);
    s.resize(&ctx, w, h);
    s.retheme(&ctx);
    f(&mut s, &ctx);
}

/// What the widgets read under a preset: its palette, babble at 7.
fn env() -> Env {
    let theme = Theme::classic();
    let o = crate::scene::Opts::new(INFO.options, toml::Table::new());
    let ctx = Ctx {
        theme: &theme,
        opts: &o,
        bg: Color::Reset,
        seed: 11,
        fiesta: Fiesta::NONE,
    };
    Env {
        pal: Pal::dressed(&ctx, true),
        level: 7,
        day: 0.0,
    }
}

/// Cells drawn outside `r`.
fn outside(c: &Canvas, r: Rect) -> usize {
    let mut n = 0;
    for y in 0..c.h() as i32 {
        for x in 0..c.w() as i32 {
            let inside = x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h;
            if !inside && c.get(x, y).is_some_and(|cell| cell.ch != ' ') {
                n += 1;
            }
        }
    }
    n
}

/// A widget drawn alone into a `w`x`h` canvas.
fn drawn(widget: &mut dyn Widget, w: u16, h: u16, env: &Env) -> Canvas {
    let mut c = Canvas::new(w, h, Color::Reset);
    widget.draw(&mut c, whole(w, h), env);
    c
}

fn whole(w: u16, h: u16) -> Rect {
    Rect {
        x: 0,
        y: 0,
        w: w as i32,
        h: h as i32,
    }
}

/// Row `y` of `r`, as drawn (`r` inside the canvas).
fn row(c: &Canvas, r: Rect, y: i32) -> String {
    let at = ((r.y + y) * c.w() as i32 + r.x) as usize;
    c.cells()[at..at + r.w as usize]
        .iter()
        .map(|cell| cell.ch)
        .collect()
}

fn rows(c: &Canvas) -> Vec<String> {
    let r = whole(c.w(), c.h());
    (0..r.h).map(|y| row(c, r, y)).collect()
}

// ------------------------------------------------------------------ no English prose

/// English words that are neither Spanish words nor keywords of the code
/// listings (spec 11.3).
const STOP: &[&str] = &[
    "the",
    "of",
    "and",
    "with",
    "from",
    "into",
    "your",
    "our",
    "their",
    "this",
    "that",
    "these",
    "each",
    "every",
    "please",
    "unable",
    "failed",
    "cannot",
    "not",
    "such",
    "uploading",
    "downloading",
    "syncing",
];

/// The boilerplate of the tools imitated, which is not prose: the kernel's
/// panic, the tree's refusal, the mission log's check, the fixed words of
/// the packet lines. (The level tags hold no word of the stop list.)
const BOILERPLATE: &[&str] = &[
    "Kernel panic - not syncing",
    "Call Trace:",
    "Tainted:",
    "[ OK ]",
    "[permission denied]",
    "Standard query",
    "Application Data",
    "Echo (ping) request",
    "Encrypted packet",
    "Who has",
    "Tell",
];

const LEVELS: &[&str] = &["INFO", "DEBUG", "NOTE", "WARN", "ERROR", "CRIT"];

const HEX: usize = 0;
const CODE: usize = 2;
const PEM: usize = 7;
const RAIN: usize = 11;

/// A character of a word (an identifier, `extend_from_slice`, is one):
/// kana, braille, box drawing and punctuation split words.
fn letter(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || "áéíóúüñÁÉÍÓÚÜÑ".contains(c)
}

/// `s` without the word an edge cut short at its end.
fn uncut(s: &str) -> &str {
    s.trim_end_matches(letter)
}

/// What a reader sees of a widget of kind `kind` in `r`, as texts: each row
/// less a word the pane's edge may have cut short (and in a code listing, a
/// comment cut short), the hexdump's sentences put back together from its
/// bytes, and the rain read downward. A key's base64 is not read.
fn read(c: &Canvas, r: Rect, kind: usize, out: &mut Vec<String>) {
    match kind {
        RAIN => {
            for x in 0..r.w {
                let col: String = (0..r.h)
                    .map(|y| c.get(r.x + x, r.y + y).map_or(' ', |cell| cell.ch))
                    .collect();
                out.push(uncut(col.trim_start_matches(letter)).to_string());
            }
        }
        HEX => {
            // The bytes, row after row; a sentence lies between two zeros.
            let mut bytes = Vec::new();
            for y in 0..r.h {
                let line = row(c, r, y);
                let hex = line.split('|').next().unwrap_or("");
                for tok in hex.split_whitespace().skip(1) {
                    if let (2, Ok(b)) = (tok.len(), u8::from_str_radix(tok, 16)) {
                        bytes.push(b);
                    }
                }
            }
            // Between two zeros: random bytes, then a sentence, whole. (The
            // first and the last run may be cut by the pane.)
            let parts: Vec<&[u8]> = bytes.split(|&b| b == 0).collect();
            for part in parts.iter().skip(1).take(parts.len().saturating_sub(2)) {
                let text = String::from_utf8_lossy(part);
                let tail = text
                    .rsplit(|c: char| c.is_control() || c == '\u{fffd}')
                    .next()
                    .unwrap_or("");
                if tail.chars().count() >= 8 {
                    out.push(tail.to_string());
                }
            }
        }
        _ => {
            for y in 0..r.h {
                let line = row(c, r, y);
                if kind == PEM && !line.trim().contains(' ') {
                    continue;
                }
                let mut text = uncut(&line).to_string();
                if kind == CODE {
                    // A comment quoted into a call is cut at 24 characters,
                    // one after a `;` at 30.
                    let parts: Vec<&str> = text.split('"').collect();
                    let n = parts.len();
                    let parts: Vec<&str> = parts
                        .into_iter()
                        .enumerate()
                        .map(|(i, p)| if i + 1 < n { uncut(p) } else { p })
                        .collect();
                    text = parts.join("\"");
                    if text.contains(" ; ") {
                        text = uncut(text.trim_end()).to_string();
                    }
                }
                out.push(text);
            }
        }
    }
}

/// The words of `text` once the boilerplate is gone, in lower case.
fn words(text: &str) -> Vec<String> {
    let mut t = text.to_string();
    for b in BOILERPLATE {
        t = t.replace(b, " ");
    }
    t.split(|c| !letter(c))
        .filter(|w| !w.is_empty() && !LEVELS.contains(w))
        .map(str::to_lowercase)
        .collect()
}

/// The words of the stop list in `texts`, and the texts they came from.
fn stopped(texts: &BTreeSet<String>) -> (BTreeSet<String>, Vec<String>) {
    let mut found = BTreeSet::new();
    let mut from = Vec::new();
    for t in texts {
        let hit: Vec<String> = words(t)
            .into_iter()
            .filter(|w| STOP.contains(&w.as_str()))
            .collect();
        if !hit.is_empty() {
            found.extend(hit);
            from.push(t.clone());
        }
    }
    (found, from)
}

/// Each of the nineteen kinds dressed by `garb`, in a 120x40 and a 60x20
/// pane, three simulated minutes each at `dt = 0.1`, every drawn row read:
/// the words of the stop list, and the rows they came from.
fn leaks(garb: Garb) -> (BTreeSet<String>, Vec<String>) {
    let texts = std::thread::scope(|sc| {
        let threads: Vec<_> = (0..KINDS)
            .map(|kind| sc.spawn(move || seen(kind, garb)))
            .collect();
        let mut all = BTreeSet::new();
        for t in threads {
            all.extend(t.join().unwrap());
        }
        all
    });
    stopped(&texts)
}

/// Every text a widget of kind `kind` shows (`leaks`).
fn seen(kind: usize, garb: Garb) -> BTreeSet<String> {
    let env = env();
    let mut texts = BTreeSet::new();
    let mut frame = Vec::new();
    for (w, h) in [(120u16, 40u16), (60, 20)] {
        let mut widget = make_widget(kind, 40 + kind as u64, 7, garb);
        widget.resize(w, h);
        let mut c = Canvas::new(w, h, Color::Reset);
        for _ in 0..1800 {
            widget.update(0.1, &env);
            c.clear();
            widget.draw(&mut c, whole(w, h), &env);
            read(&c, whole(w, h), kind, &mut frame);
            texts.extend(frame.drain(..));
        }
    }
    texts
}

/// The whole scene with the preset at `path`, three simulated minutes at
/// 140x40: its ticker row (less the words cut at both ends) and every pane's
/// title (whole: the title row shows as much of it as fits).
fn scene_texts(path: &Path) -> BTreeSet<String> {
    let mut texts = BTreeSet::new();
    scene(&naming(path), Fiesta::NONE, 140, 40, |s, ctx| {
        assert!(s.preset.is_some());
        for _ in 0..1800 {
            s.tick(ctx, 0.1, 0.1);
            let mut c = Canvas::new(140, 40, Color::Reset);
            s.draw(ctx, &mut c);
            let tape = row(&c, whole(140, 40), 39);
            texts.insert(uncut(tape.trim_start_matches(letter)).to_string());
            if let Some(root) = &s.root {
                root.each(&mut |p| {
                    texts.insert(p.title.clone());
                });
            }
        }
    });
    texts
}

#[test]
fn no_english_prose_under_a_preset() {
    // 1. What is drawn: nothing on the stop list, widget by widget and in
    // the scene's ticker and titles, under the example and an empty preset.
    for p in [example(), bare()] {
        let (words, rows) = leaks(Garb::preset(p));
        assert!(words.is_empty(), "{words:?} in {rows:#?}");
    }
    let empty = cache::temp("prose-empty", "");
    for path in [example_path(), empty.clone()] {
        let (words, rows) = stopped(&scene_texts(&path));
        assert!(words.is_empty(), "{path:?}: {words:?} in {rows:#?}");
    }
    std::fs::remove_file(&empty).ok();
    // 3. It can fail: the preset's words with a holiday's share of them let
    // `babble`'s sentences, its transfers and the real errno table through.
    let (words, rows) = leaks(Garb::leaky(example()));
    let any = |list: &[&str]| list.iter().any(|w| words.contains(*w));
    assert!(any(&["uploading", "downloading", "syncing"]), "{words:?}");
    assert!(any(&["the", "of", "with", "and"]), "{words:?}");
    assert!(
        rows.iter()
            .any(|r| feed::ERRNOS.iter().any(|e| r.contains(e.2))),
        "{rows:#?}"
    );
}

#[test]
fn the_words_come_from_the_preset() {
    // 2. Where it comes from.
    let g = Garb::preset(example());
    let v = example().vocab;
    let env = env();
    // Errno lines: screenful after screenful of `dmesg`.
    let mut errnos = Feed::new(FeedKind::Errno, 3, 7, g);
    let mut n = 0;
    while n < 10_000 {
        errnos.resize(160, 80);
        for r in rows(&drawn(&mut errnos, 160, 80, &env)) {
            if r.starts_with('[') || r.trim().is_empty() {
                continue; // a kernel panic
            }
            let msg: String = r.chars().skip(18).collect();
            let msg = msg.split("  \u{2014} ").next().unwrap_or("").trim_end();
            assert!(v.errnos.iter().any(|e| e.2 == msg), "{r}");
            assert!(!feed::ERRNOS.iter().any(|e| e.2 == msg), "{r}");
            n += 1;
        }
    }
    // Logins, screenful after screenful of `authd`. (The base's `admin` is
    // also one of `feed::USERS`: what counts is where it was drawn from.)
    let mut auth = Feed::new(FeedKind::Auth, 4, 7, g);
    let mut n = 0;
    while n < 10_000 {
        auth.resize(160, 80);
        for r in rows(&drawn(&mut auth, 160, 80, &env)) {
            let user = r.split('@').next().unwrap_or("");
            assert!(v.users.contains(&user), "{r}");
            assert!(!feed::USERS.contains(&user) || v.users.contains(&user));
            n += 1;
        }
    }
    let mut rng = Rng::with_seed(5);
    for _ in 0..10_000 {
        let d = feed::daemon(&mut rng, g);
        assert!(
            v.daemons.contains(&d) && !babble::DAEMONS.contains(&d),
            "{d}"
        );
    }
    // Sentences, titles and labels are the vocabulary's own, number for
    // number: nothing went through `babble` or a holiday's share.
    let mut a = Rng::with_seed(6);
    for i in 0..10_000 {
        let level = (i % 11) as u8;
        let (mut x, mut y) = (String::new(), String::new());
        let mut b = a.clone();
        assert!(feed::say(&mut a, level, g, &mut x));
        v.sentence(&mut b, level, &mut y);
        assert_eq!((&x, a.clone().u64(..)), (&y, b.clone().u64(..)));
        let (mut x, mut y) = (String::new(), String::new());
        let mut b = a.clone();
        feed::title(&mut a, g, &mut x);
        v.title(&mut b, &mut y);
        assert_eq!((&x, a.clone().u64(..)), (&y, b.clone().u64(..)));
        let (mut x, mut y) = (String::new(), String::new());
        let mut b = a.clone();
        feed::label(&mut a, level, g, &mut x);
        v.progress_label_es(&mut b, level, example().transfers, &mut y);
        assert_eq!((&x, a.clone().u64(..)), (&y, b.clone().u64(..)));
    }
}

#[test]
fn people_never_stand_beside_a_number() {
    let g = Garb::preset(example());
    let contacts = example().contacts;
    let env = env();
    let people = |r: &str| r.contains("Stallman") || r.contains("Torvalds");
    let mut radar = make_widget(RADAR, 8, 7, g);
    radar.resize(80, 24);
    for _ in 0..600 {
        radar.update(0.1, &env);
        let rs = rows(&drawn(radar.as_mut(), 80, 24, &env));
        for r in &rs {
            assert!(!people(r), "{r}");
        }
        // The list: one row a contact, from the third.
        for r in &rs[2..2 + 11] {
            let entry = r.trim_start_matches(|c: char| !c.is_ascii_alphanumeric());
            let tag = entry.split(' ').next().unwrap_or("");
            let (name, id) = tag.rsplit_once('-').unwrap_or(("", ""));
            assert!(contacts.contains(&name), "{r}");
            assert!(
                id.len() == 3 && id.bytes().all(|b| b.is_ascii_digit()),
                "{r}"
            );
        }
    }
    let mut dna = Feed::new(FeedKind::Dna, 9, 7, g);
    dna.resize(100, 30);
    let mut shown = 0;
    for _ in 0..600 {
        dna.update(0.1, &env);
        for r in rows(&drawn(&mut dna, 100, 30, &env)) {
            assert!(!people(&r), "{r}");
            if let Some(rest) = r.strip_prefix("> ") {
                let name = rest.split("  ").next().unwrap_or("");
                assert!(contacts.contains(&name), "{r}");
                shown += 1;
            }
        }
    }
    assert!(shown > 0);
}

#[test]
fn a_preset_radar_is_at_its_height() {
    let r = widgets::Radar::new(1, Garb::preset(example()));
    assert_eq!(r.blips_len(), 11);
}

#[test]
fn dressed_widgets_stay_in_their_panes() {
    let env = env();
    for kind in 0..KINDS {
        for &(w, h) in &[
            (1u16, 1u16),
            (3, 3),
            (12, 4),
            (29, 6),
            (44, 11),
            (80, 30),
            (120, 40),
            (34, 29),
            (60, 54),
            (67, 59),
            (120, 105),
        ] {
            let mut widget = make_widget(kind, 5, 7, Garb::preset(example()));
            widget.resize(w, h);
            let r = Rect {
                x: 3,
                y: 2,
                w: w as i32,
                h: h as i32,
            };
            for step in 0..40 {
                widget.update(if step % 9 == 0 { 0.5 } else { 0.07 }, &env);
                let mut c = Canvas::new(w + 8, h + 6, Color::Reset);
                widget.draw(&mut c, r, &env);
                assert_eq!(outside(&c, r), 0, "kind {kind} {w}x{h} step {step}");
            }
        }
    }
}

#[test]
fn a_preset_wins_over_the_holiday() {
    for fiesta in [Fiesta::height(Kind::Navidad), Fiesta::height(Kind::Muertos)] {
        scene(&naming(&example_path()), fiesta, 140, 40, |s, ctx| {
            assert!(s.preset.is_some());
            for i in 0..1800 {
                s.tick(ctx, 1.0 / 30.0, 1.0 / 30.0);
                assert_eq!(s.fiesta, Fiesta::NONE);
                assert!(s.kinds().iter().all(|&k| k < KINDS), "{:?}", s.kinds());
                assert!(s.fool.is_none());
                if i % 30 != 0 {
                    continue;
                }
                let mut c = Canvas::new(140, 40, Color::Reset);
                s.draw(ctx, &mut c);
                for r in s.rects() {
                    let bar = row(&c, Rect { h: 1, ..r }, 0);
                    assert!(!bar.contains(['\u{25cf}', '\u{25bc}']), "{fiesta:?}: {bar}");
                }
            }
            assert_eq!(s.env.pal.main, rgb(0x33ee66));
            assert_eq!(s.env.pal.oro, s.env.pal.amber);
            // A day later the calendar still does not dress the screen.
            s.retheme(ctx);
            assert_eq!(s.fiesta, Fiesta::NONE);
            assert_eq!(s.env.pal.oro, s.env.pal.amber);
        });
    }
}

#[test]
fn the_map_runs_between_the_worlds_cities() {
    let env = env();
    let mut map = make_widget(12, 4, 7, Garb::preset(example()));
    map.resize(100, 30);
    let mut routes = 0;
    for _ in 0..600 {
        map.update(0.1, &env);
        let status = rows(&drawn(map.as_mut(), 100, 30, &env)).swap_remove(0);
        if status.trim().is_empty() {
            continue;
        }
        let between = map::CITIES.iter().any(|a| {
            map::CITIES
                .iter()
                .any(|b| status.contains(&format!(": {} -> {} ", a.0, b.0)))
        });
        assert!(between, "{status}");
        routes += 1;
    }
    assert!(routes > 0);
}

#[test]
fn names_fall_in_the_rain() {
    let env = env();
    let words = example().vocab.dress.rain_words;
    let mut rain = make_widget(RAIN, 6, 7, Garb::preset(example()));
    rain.resize(80, 30);
    let mut fell = false;
    for _ in 0..600 {
        rain.update(0.1, &env);
        let mut cols = Vec::new();
        read(
            &drawn(rain.as_mut(), 80, 30, &env),
            whole(80, 30),
            RAIN,
            &mut cols,
        );
        fell |= cols.iter().any(|c| words.iter().any(|w| c.contains(w)));
    }
    assert!(fell);
}

#[test]
fn babble_zero_shows_the_bases_plain_lines() {
    let env = env();
    let mut log = Feed::new(FeedKind::Log, 2, 0, Garb::preset(example()));
    log.resize(120, 30);
    let mut seen = 0;
    for _ in 0..600 {
        log.update(0.1, &env);
        for r in rows(&drawn(&mut log, 120, 30, &env)) {
            // "12:00:00 host-07 daemond[123]: INFO  message"; a wrapped
            // message goes on under it.
            let Some((_, rest)) = r.split_once("]: ") else {
                continue;
            };
            let msg: String = rest.chars().skip(6).collect();
            // (Capitalized as a sentence is.)
            let msg = msg.trim_end().to_lowercase();
            assert!(
                BASE.mundane
                    .iter()
                    .map(|m| m.to_lowercase())
                    .any(|m| msg.starts_with(&m) || m.starts_with(&msg)),
                "{r}"
            );
            seen += 1;
        }
    }
    assert!(seen > 100);
}

#[test]
fn a_preset_that_is_not_there_is_no_preset() {
    let missing = std::env::temp_dir().join(format!(
        "oxiper-preset-{}-not-there-scene.toml",
        std::process::id()
    ));
    for opts in [naming(&missing), "preset = 2026".to_string()] {
        scene(&opts, Fiesta::NONE, 80, 24, |s, ctx| {
            assert!(s.preset.is_none());
            for _ in 0..30 {
                s.tick(ctx, 0.1, 0.1);
            }
            let mut c = Canvas::new(80, 24, Color::Reset);
            s.draw(ctx, &mut c);
        });
    }
}
