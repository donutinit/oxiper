//! hollywood with a preset (spec 8.3, 11.3): a preset dresses every ordinary
//! widget as a holiday at its height would, in its own words and no English
//! prose, with call signs where a number stands beside a name, and it wins
//! over the holiday.

use super::feed::{self, Feed, FeedKind};
use super::garb::Garb;
use super::neutro::BASE;
use super::preset::{self, Preset, Round};
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
        pal: Pal::dressed(&ctx, true, 0.0),
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
    // 1. What is drawn: nothing on the stop list, widget by widget, under
    // the example and an empty preset.
    for p in [example(), bare()] {
        let (words, rows) = leaks(Garb::preset(p));
        assert!(words.is_empty(), "{words:?} in {rows:#?}");
    }
}

#[test]
fn no_english_prose_in_the_scenes_ticker_and_titles() {
    // 1. What is drawn: nothing on the stop list in the scene's ticker and
    // titles, under the example and an empty preset.
    let empty = cache::temp("prose-empty", "");
    for path in [example_path(), empty.clone()] {
        let (words, rows) = stopped(&scene_texts(&path));
        assert!(words.is_empty(), "{path:?}: {words:?} in {rows:#?}");
    }
    std::fs::remove_file(&empty).ok();
}

#[test]
fn the_check_for_english_prose_can_fail() {
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

// ------------------------------------------------------------------ what the camera reads

/// The mark between two items of the ticker (the base's).
const MARK: &str = "  \u{25c6}  ";

/// hollywood with `opts` for `secs` simulated seconds at 140x40: every item
/// of the ticker in the order it was fed, and every item seen whole on its row.
fn ticker(opts: &str, secs: u32) -> (Vec<String>, BTreeSet<String>) {
    let (mut fed, mut seen) = (String::new(), BTreeSet::new());
    scene(opts, Fiesta::NONE, 140, 40, |s, ctx| {
        assert!(s.preset.is_some());
        for step in 0..secs * 10 {
            s.tick(ctx, 0.1, 0.1);
            let kept = s.tape.len();
            if step % 10 == 0 {
                let mut c = Canvas::new(140, 40, Color::Reset);
                s.draw(ctx, &mut c);
                let tape = row(&c, whole(140, 40), 39);
                let parts: Vec<&str> = tape.split(MARK).collect();
                // The first and the last are cut by the edges.
                for p in parts.iter().skip(1).take(parts.len().saturating_sub(2)) {
                    seen.insert(p.to_string());
                }
            } else {
                s.feed_tape();
            }
            fed.extend(s.tape.get(kept..).unwrap_or(&[]));
        }
    });
    let mut items: Vec<String> = fed.split(MARK).map(str::to_string).collect();
    items.pop(); // what follows the last mark
    (items, seen)
}

/// Is `item` the template `tpl` with its blanks filled, in capitals? (Its
/// fixed words, in order; "DE" may have become "DEL".)
fn filled(tpl: &str, item: &str) -> bool {
    let tpl = tpl.to_uppercase();
    let mut rest = item;
    for (i, piece) in tpl.split(['{', '}']).enumerate() {
        let piece = piece.trim();
        if i % 2 == 1 || piece.is_empty() {
            continue; // a blank's key
        }
        match rest.find(piece) {
            Some(at) if i > 0 || at == 0 => rest = &rest[at + piece.len()..],
            _ => return false,
        }
    }
    true
}

#[test]
fn the_ticker_is_the_files_headlines() {
    const FIVE: [&str; 5] = [
        "Primera noticia del día",
        "1991: Torvalds publica el kernel",
        "Libre como en libertad",
        "Año de la GPL",
        "Copyleft",
    ];
    let path = cache::temp(
        "ticker-five",
        &format!("titulares = '''\n{}\n'''\n", FIVE.join("\n")),
    );
    let want: BTreeSet<String> = FIVE.iter().map(|h| h.to_uppercase()).collect();
    let (items, seen) = ticker(&naming(&path), 300);
    std::fs::remove_file(&path).ok();
    assert!(items.len() > 20, "{items:?}");
    for item in &items {
        assert!(want.contains(item), "{item:?}");
    }
    assert!(!seen.is_empty());
    for item in &seen {
        assert!(want.contains(item), "on screen: {item:?}");
    }
    // Every line once before any repeats.
    for run in items.as_chunks::<5>().0 {
        let run: BTreeSet<String> = run.iter().cloned().collect();
        assert_eq!(run, want, "{items:?}");
    }
    // The example's, with or without babble: its headlines and nothing else.
    for opts in [
        naming(&example_path()),
        format!("{}\nbabble = 0", naming(&example_path())),
    ] {
        let (items, seen) = ticker(&opts, 300);
        assert!(items.len() > 20);
        for item in items.iter().chain(&seen) {
            assert!(
                !item.contains("MISI\u{00d3}N") && !item.contains("MISSION"),
                "{opts}: {item:?}"
            );
            assert_eq!(item, &item.to_uppercase(), "{opts}");
            let tpls = example().vocab.headlines;
            assert!(tpls.iter().any(|t| filled(t, item)), "{opts}: {item:?}");
        }
    }
}

#[test]
fn without_titulares_the_ticker_is_the_engines_mix() {
    let path = cache::temp("ticker-names", "nombres = '''\nGNU\nLinux\nDebian\n'''\n");
    let (items, seen) = ticker(&naming(&path), 300);
    std::fs::remove_file(&path).ok();
    assert!(items.len() > 10, "{items:?}");
    let (mut missions, mut headlines) = (0, 0);
    for item in items.iter().chain(&seen) {
        assert!(!item.contains("MISSION"), "{item:?}");
        if item.starts_with("MISI\u{00d3}N \u{25c6} ") {
            missions += 1;
        } else {
            assert!(BASE.headlines.iter().any(|t| filled(t, item)), "{item:?}");
            headlines += 1;
        }
    }
    assert!(missions > 0 && headlines > 0, "{items:?}");
}

/// Every pane's title, `secs` simulated seconds at 140x40, checked by
/// `each(title, process name)`; every few seconds the scene is drawn and
/// `drawn(canvas, title, its rect, palette)` sees each title as drawn.
fn titles(
    opts: &str,
    secs: u32,
    mut each: impl FnMut(&str, &str),
    mut drawn: impl FnMut(&Canvas, &str, Rect, &Pal),
) {
    scene(opts, Fiesta::NONE, 140, 40, |s, ctx| {
        assert!(s.preset.is_some());
        for step in 0..secs * 10 {
            s.tick(ctx, 0.1, 0.1);
            if let Some(root) = &s.root {
                root.each(&mut |p| each(&p.title, p.widget.name()));
            }
            if step % 50 == 0 {
                let mut c = Canvas::new(140, 40, Color::Reset);
                s.draw(ctx, &mut c);
                if let Some(root) = &s.root {
                    root.each(&mut |p| drawn(&c, &p.title, p.rect, &s.env.pal));
                }
            }
        }
    });
}

#[test]
fn titles_come_first() {
    let given = example().titles;
    for (opts, secs) in [
        (naming(&example_path()), 600),
        (format!("{}\nbabble = 0", naming(&example_path())), 120),
    ] {
        let mut shown = BTreeSet::new();
        let mut bright = 0;
        titles(
            &opts,
            secs,
            |title, name| {
                let t = given
                    .iter()
                    .find(|t| title == format!("{t} \u{00b7} {name}"))
                    .unwrap_or_else(|| panic!("{opts}: {title:?}"));
                shown.insert(*t);
            },
            |c, title, r, pal| {
                // The title's first word, as far as the bar shows it.
                let tw = (r.w - 5).max(0) as usize;
                let first = title.split(' ').next().unwrap_or("").chars().count();
                if tw < 4 {
                    return;
                }
                for i in 0..first.min(tw) {
                    let cell = c.get(r.x + 3 + i as i32, r.y).unwrap();
                    assert_eq!(cell.fg, pal.bright, "{title:?} at {i}");
                    bright += 1;
                }
            },
        );
        assert!(bright > 0);
        if secs >= 600 {
            assert_eq!(shown.len(), given.len(), "{shown:?}");
        }
    }
}

#[test]
fn without_titulos_titles_are_generated() {
    let p = parsed(b"nombres = '''\nGNU\nLinux\n'''\n");
    assert!(p.titles.is_empty());
    let g = Garb::preset(p);
    let mut round = Round::new(p.titles.len());
    let mut a = Rng::with_seed(9);
    for i in 0..2000 {
        let kind = i % KINDS;
        let mut pane = Pane {
            rect: Rect::default(),
            kind,
            widget: make_widget(kind, i as u64, 7, g),
            title: String::new(),
            retitle: 0.0,
            age: 0.0,
            content: (0, 0),
        };
        // The very draws of `Vocab::title`, never `babble::title`'s.
        let mut b = a.clone();
        Hollywood::retitle(&mut pane, &mut a, 7, g, &mut round);
        let mut want = format!("{} \u{00b7} ", pane.widget.name());
        let from = want.len();
        p.vocab.title(&mut b, &mut want);
        assert_eq!(want[from..], want[from..].to_lowercase());
        let _ = b.f32();
        assert_eq!(pane.title, want);
        assert_eq!(a.clone().u64(..), b.u64(..));
    }
    // In the scene: the process name, then the generated words.
    let path = cache::temp("titles-generated", "nombres = '''\nGNU\nLinux\n'''\n");
    titles(
        &naming(&path),
        60,
        |title, name| {
            let rest = title.strip_prefix(&format!("{name} \u{00b7} "));
            assert!(rest.is_some_and(|r| r == r.to_lowercase()), "{title:?}");
        },
        |_, _, _, _| {},
    );
    std::fs::remove_file(&path).ok();
}

/// An `Access` dressed by `p`, stepped until a banner comes up: its index.
fn next_banner(a: &mut widgets::Access, env: &Env) -> usize {
    while a.showing().is_some() {
        a.update(0.1, env);
    }
    loop {
        a.update(0.1, env);
        if let Some(i) = a.showing() {
            return i;
        }
    }
}

/// The box of the banner `b` as `letrero` fits it in a `w`x`h` pane, as a
/// rect of that pane.
fn banner_box(b: &preset::Banner, w: i32, h: i32) -> Rect {
    let f = letrero::fit(b.top, b.bottom, w, h).unwrap();
    Rect {
        x: (w - f.w) / 2,
        y: (h - f.h) / 2,
        w: f.w,
        h: f.h,
    }
}

#[test]
fn the_banner_shows_the_files_pairs() {
    let env = env();
    let pal = &env.pal;
    for p in [example(), parsed(b"letrero_verde = 'SOFTWARE / LIBRE'\n")] {
        let mut a = widgets::Access::new(3, 7, Garb::preset(p));
        a.resize(66, 19);
        let mut dealt = Vec::new();
        for _ in 0..200 {
            let i = next_banner(&mut a, &env);
            dealt.push(i);
            let b = &p.banners[i];
            let c = drawn(&mut a, 66, 19, &env);
            let bx = banner_box(b, 66, 19);
            let frame = c.get(bx.x, bx.y).unwrap();
            assert_eq!(frame.ch, '\u{2551}');
            assert_eq!(frame.fg, if b.green { pal.green } else { pal.red });
            // Nothing in the box is in the other color.
            let other = if b.green { pal.red } else { pal.green };
            for y in bx.y..bx.y + bx.h {
                for x in bx.x..bx.x + bx.w {
                    assert_ne!(c.get(x, y).unwrap().fg, other, "({x}, {y})");
                }
            }
        }
        // Each once before any repeats.
        let all: BTreeSet<usize> = (0..p.banners.len()).collect();
        for run in dealt.chunks_exact(p.banners.len()) {
            assert_eq!(run.iter().copied().collect::<BTreeSet<_>>(), all);
        }
        if p.banners.len() == 1 {
            assert!(p.banners[0].green);
        }
    }
}

#[test]
fn the_banner_is_fitted() {
    let env = env();
    let ink = env.pal.ink;
    let p = parsed(b"letrero_verde = 'SOFTWARE / LIBRE'\n");
    let mut a = widgets::Access::new(4, 7, Garb::preset(p));
    next_banner(&mut a, &env);
    // The cells of the box: its width and height.
    let inked = |c: &Canvas| -> Option<(i32, i32, i32, i32)> {
        let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, -1, -1);
        let mut n = 0;
        for y in 0..c.h() as i32 {
            for x in 0..c.w() as i32 {
                if c.get(x, y).is_some_and(|cell| cell.bg == ink) {
                    (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
                    n += 1;
                }
            }
        }
        (n > 0).then(|| {
            assert_eq!(n, (x1 - x0 + 1) * (y1 - y0 + 1), "not a box");
            (x0, y0, x1 - x0 + 1, y1 - y0 + 1)
        })
    };
    for ((w, h), size) in [((66, 19), (66, 13)), ((33, 9), (18, 8))] {
        a.resize(w, h);
        let c = drawn(&mut a, w, h, &env);
        let (.., bw, bh) = inked(&c).unwrap_or_else(|| panic!("{w}x{h}: no banner"));
        assert_eq!((bw, bh), size, "{w}x{h}");
    }
    // Below pocket: today's small form, one line of text in three rows.
    a.resize(33, 7);
    let c = drawn(&mut a, 33, 7, &env);
    let (x, y, bw, bh) = inked(&c).unwrap();
    assert_eq!((bw, bh), (18, 3));
    let text = row(&c, Rect { x, y, w: bw, h: bh }, 1);
    assert_eq!(text, "\u{2551} SOFTWARE LIBRE \u{2551}");
    for (w, h) in [(9, 9), (33, 2)] {
        a.resize(w, h);
        let c = drawn(&mut a, w, h, &env);
        assert_eq!(inked(&c), None, "{w}x{h}");
        assert!(
            !c.cells().iter().any(|cell| cell.ch == '\u{2550}'),
            "{w}x{h}"
        );
    }
}

#[test]
fn the_captions_are_tema_and_the_authors() {
    let env = env();
    let p = parsed(
        "tema = 'Software libre'\nletrero_verde = '''\nSOFTWARE / LIBRE / desde 1983\nGNU / NO ES UNIX\n'''\n"
            .as_bytes(),
    );
    let mut a = widgets::Access::new(5, 7, Garb::preset(p));
    a.resize(66, 19);
    let mut both = BTreeSet::new();
    for _ in 0..8 {
        let i = next_banner(&mut a, &env);
        let b = &p.banners[i];
        let c = drawn(&mut a, 66, 19, &env);
        let bx = banner_box(b, 66, 19);
        let top = row(&c, Rect { h: 1, ..bx }, 0);
        let bottom = row(&c, Rect { h: 1, ..bx }, bx.h - 1);
        assert!(top.contains(" Software libre "), "{top}");
        if b.caption.is_empty() {
            let plain = format!("\u{2551}{}\u{2551}", "\u{2550}".repeat(bx.w as usize - 2));
            assert_eq!(bottom, plain);
        } else {
            assert_eq!(b.caption, "desde 1983");
            assert!(bottom.contains(" desde 1983 "), "{bottom}");
        }
        both.insert(i);
    }
    assert_eq!(both.len(), 2);
}

// ------------------------------------------------------------------ live edits (spec 8.7, 11.4)

/// `secs` seconds of the interactive loop at 10 frames a second.
fn play(s: &mut Hollywood, ctx: &Ctx, secs: f32) {
    let mut c = Canvas::new(s.w.max(1) as u16, s.h.max(1) as u16, Color::Reset);
    for _ in 0..(secs * 10.0) as usize {
        s.tick(ctx, 0.1, 0.1);
        c.clear();
        s.draw(ctx, &mut c);
    }
}

/// The ticker as it stands, fed to its length.
fn tape(s: &mut Hollywood) -> String {
    s.feed_tape();
    s.tape.iter().collect()
}

/// A file with one headline, `text`.
fn one_headline(text: &str) -> String {
    format!("titulares = ['{text}']\n")
}

#[test]
fn an_edit_shows_after_refresh() {
    let path = cache::temp("edit", &one_headline("Primera version"));
    scene(&naming(&path), Fiesta::NONE, 140, 40, |s, ctx| {
        play(s, ctx, 3.0);
        assert!(tape(s).contains("PRIMERA VERSION"));
        assert_eq!(s.refresh(ctx), None);
        let (rects, kinds) = (s.rects(), s.kinds());
        let first = s.preset.unwrap();
        std::fs::write(&path, one_headline("La segunda version")).unwrap();
        assert_eq!(s.refresh(ctx), Some(Ok("preset reloaded".into())));
        assert!(!std::ptr::eq(s.preset.unwrap(), first));
        assert_eq!((s.rects(), s.kinds()), (rects.clone(), kinds.clone()));
        let t = tape(s);
        assert!(t.starts_with("LA SEGUNDA VERSION"), "{t}");
        assert!(!t.contains("PRIMERA"), "{t}");
        assert_eq!(s.refresh(ctx), None);
        // An entry dropped: the HUD says how many.
        std::fs::write(&path, "titulares = ['Tercera version', 'Hola {n']\n").unwrap();
        assert_eq!(
            s.refresh(ctx),
            Some(Ok("preset reloaded \u{b7} 1 warning".into()))
        );
        assert!(tape(s).starts_with("TERCERA VERSION"));
        std::fs::write(&path, "titulares = ['Cuarta', 'Hola {n', 'Adios }']\n").unwrap();
        assert_eq!(
            s.refresh(ctx),
            Some(Ok("preset reloaded \u{b7} 2 warnings".into()))
        );
        assert_eq!((s.rects(), s.kinds()), (rects, kinds));
        play(s, ctx, 3.0);
    });
    std::fs::remove_file(&path).ok();
}

#[test]
fn a_broken_file_is_said_once() {
    let path = cache::temp("broken-once", &one_headline("Lo que se dijo"));
    scene(&naming(&path), Fiesta::NONE, 140, 40, |s, ctx| {
        play(s, ctx, 1.0);
        let first = s.preset.unwrap();
        std::fs::write(&path, "tema = \n").unwrap();
        let Some(Err(m)) = s.refresh(ctx) else {
            panic!("a broken file was not said");
        };
        assert!(m.starts_with("preset: ") && m.contains("line"), "{m}");
        assert!(!m.contains('\n') && m.chars().count() <= 60, "{m:?}");
        play(s, ctx, 10.0);
        assert!(std::ptr::eq(s.preset.unwrap(), first));
        assert!(tape(s).contains("LO QUE SE DIJO"));
        assert_eq!(s.refresh(ctx), None);
        assert_eq!(s.refresh(ctx), None);
        std::fs::write(&path, one_headline("Lo que se arreglo")).unwrap();
        assert!(matches!(s.refresh(ctx), Some(Ok(_))));
        assert!(tape(s).starts_with("LO QUE SE ARREGLO"));
        assert_eq!(s.refresh(ctx), None);
    });
    std::fs::remove_file(&path).ok();
}

/// A second bad save is said again, even with the same reason (spec 8.6).
#[test]
fn each_bad_save_is_said() {
    let path = cache::temp("bad-saves", &one_headline("Lo primero"));
    scene(&naming(&path), Fiesta::NONE, 80, 24, |s, ctx| {
        std::fs::write(&path, "tema = \n").unwrap();
        let Some(Err(first)) = s.refresh(ctx) else {
            panic!("the first bad save was not said");
        };
        assert_eq!(s.refresh(ctx), None);
        std::fs::write(&path, "tema = \n# otra vez\n").unwrap();
        assert_eq!(s.refresh(ctx), Some(Err(first)));
        assert_eq!(s.refresh(ctx), None);
        play(s, ctx, 1.0);
        assert!(tape(s).contains("LO PRIMERO"));
    });
    std::fs::remove_file(&path).ok();
}

#[test]
fn a_deleted_file_keeps_the_words() {
    let path = cache::temp("deleted-scene", &one_headline("Aqui sigue"));
    scene(&naming(&path), Fiesta::NONE, 80, 24, |s, ctx| {
        play(s, ctx, 1.0);
        std::fs::remove_file(&path).unwrap();
        assert!(matches!(s.refresh(ctx), Some(Err(_))));
        // Said once while the file stays gone.
        for _ in 0..5 {
            assert_eq!(s.refresh(ctx), None);
        }
        play(s, ctx, 3.0);
        assert!(s.preset.is_some());
        assert!(tape(s).contains("AQUI SIGUE"));
    });
}

/// A scene built while the file is bad (by `r`, an arrow key) has the last
/// good preset, and says why at its first `refresh`, once.
#[test]
fn a_bad_file_does_not_reach_the_screen_by_r() {
    let good = "tema = 'software libre'\n";
    for (name, broken) in [
        ("by-r-broken", Some("tema = \n")),
        ("by-r-emptied", Some("")),
        ("by-r-deleted", None),
    ] {
        let path = cache::temp(name, good);
        scene(&naming(&path), Fiesta::NONE, 80, 24, |s, ctx| {
            let first = s.preset.unwrap();
            match broken {
                Some(text) => std::fs::write(&path, text).unwrap(),
                None => std::fs::remove_file(&path).unwrap(),
            }
            let mut again = Hollywood::new(ctx);
            again.resize(ctx, 80, 24);
            again.retheme(ctx);
            assert!(std::ptr::eq(again.preset.unwrap(), first), "{name}");
            let Some(Err(m)) = again.refresh(ctx) else {
                panic!("{name}: the first refresh did not say why");
            };
            assert!(m.starts_with("preset: "), "{name}: {m}");
            assert_eq!(again.refresh(ctx), None, "{name}");
            // The scene that was on screen says it too, once.
            assert!(matches!(s.refresh(ctx), Some(Err(_))), "{name}");
            assert_eq!(s.refresh(ctx), None, "{name}");
        });
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn rebuilds_do_not_read() {
    let path = cache::temp("rebuilds", "tema = 'software libre'\n");
    let theme = Theme::classic();
    let o = crate::scene::Opts::new(INFO.options, naming(&path).parse().unwrap());
    let ctx = Ctx {
        theme: &theme,
        opts: &o,
        bg: Color::Reset,
        seed: 11,
        fiesta: Fiesta::NONE,
    };
    let first = Hollywood::new(&ctx).preset.unwrap();
    for _ in 0..1000 {
        let mut s = Hollywood::new(&ctx);
        assert!(std::ptr::eq(s.preset.unwrap(), first));
        assert_eq!(s.refresh(&ctx), None);
    }
    assert_eq!(cache::parses(&path), 1);
    std::fs::remove_file(&path).ok();
}

#[test]
fn refresh_without_a_preset_is_none() {
    for opts in ["", "preset = ''"] {
        scene(opts, Fiesta::NONE, 80, 24, |s, ctx| {
            for _ in 0..5 {
                assert_eq!(s.refresh(ctx), None);
                play(s, ctx, 1.0);
            }
        });
    }
}
