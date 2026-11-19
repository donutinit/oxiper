#![allow(dead_code)] // titles, tape, banners, phrases and rounds are used in Tasks 11 and 19

//! A preset, from its file to a vocabulary (spec 7, 8.2, 8.4): the TOML an
//! author writes between takes becomes a [`Preset`], a `Vocab` laid over a
//! copy of the neutral base plus the few things `Vocab` has no field for.
//! Reading is total: any bytes give a preset and its warnings, or one error,
//! never a panic. Each entry the grammar refuses is dropped with a warning
//! that names its key and its number; the rest is used. What comes out is
//! `'static`: strings are interned, so a text is leaked once however often
//! it is read, and the slices and the `Preset` are leaked on each parse.

use super::gate;
use super::gramatica::{self, Noun};
use super::lexico::{Dress, Vocab};
use super::neutro::{BASE, CALLS, TRANSFERS};
use fastrand::Rng;
use std::collections::BTreeSet;
use std::sync::Mutex;

/// One banner of the access pane: two lines of big letters and the caption
/// for the bottom of the frame, as written ("" when there is none).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Banner {
    pub green: bool,
    pub top: &'static str,
    pub bottom: &'static str,
    pub caption: &'static str,
}

pub struct Preset {
    pub vocab: &'static Vocab,
    /// `titulos`, blanks in the engine's keys. Empty: titles are generated.
    pub titles: &'static [&'static str],
    /// `titulares` was given: the ticker shows `vocab.headlines` and nothing else.
    pub verbatim_tape: bool,
    /// Never empty: the file's pairs, or the base's two.
    pub banners: &'static [Banner],
    /// `frases` alone, for the set's log. May be empty.
    pub phrases: &'static [&'static str],
    /// Eight or more: upper-case slugs of `nombres` of 9 characters or fewer, then `neutro::CALLS`.
    pub contacts: &'static [&'static str],
    pub transfers: &'static [&'static str],
}

pub struct Loaded {
    pub preset: &'static Preset,
    pub warnings: Vec<String>,
    /// Known keys present in the file (0: an empty preset).
    pub keys: usize,
}

/// The largest file that is read.
pub(super) const MAX_BYTES: usize = 256 * 1024;
/// The most entries a list keeps.
const MAX_ENTRIES: usize = 400;
/// A HUD line, and so the longest error or warning.
const LINE: usize = 60;
/// A banner's caption, drawn on the frame.
const CAPTION: usize = 40;
/// The radar's contacts and the htop users the widgets need.
const CONTACTS: usize = 8;
const USERS: usize = 6;

/// A key of the file: its longest entry in characters, and whether it takes
/// blanks (`{persona}`) and the `*` of agreement.
struct Key {
    name: &'static str,
    max: usize,
    blanks: bool,
    star: bool,
}

const fn key(name: &'static str, max: usize, blanks: bool, star: bool) -> Key {
    Key {
        name,
        max,
        blanks,
        star,
    }
}

/// Every key, `tema` first (spec 7.4). The limits of `adjetivos`, `verbos`
/// and the banners are the plan's: a phrase, room for three written forms,
/// and two lines of 12 with a caption of 40 (12 + " / " + 12 + " / " + 40).
const KEYS: &[Key] = &[
    key("tema", 24, false, false),
    key("titulares", 100, true, false),
    key("titulos", 40, true, false),
    key("letrero_verde", 70, false, false),
    key("letrero_rojo", 70, false, false),
    key("personas", 28, false, false),
    key("nombres", 20, false, false),
    key("cosas", 40, false, false),
    key("adjetivos", 40, false, true),
    key("verbos", 60, false, false),
    key("frases", 120, true, false),
];

/// One line, at most `LINE` characters: a longer one ends in `…`. Control
/// characters (a quoted key can hold an escape sequence) become spaces.
pub(super) fn one_line(s: String) -> String {
    let s: String = s
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    if s.chars().count() <= LINE {
        return s;
    }
    let mut cut: String = s.chars().take(LINE - 1).collect();
    cut.push('…');
    cut
}

/// Bytes to a preset, or the one line that says why not. Total: never panics.
pub fn parse(bytes: &[u8]) -> Result<Loaded, String> {
    if bytes.len() > MAX_BYTES {
        return Err(format!("{} bytes, 256 KiB at most", bytes.len()));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|e| format!("not UTF-8 (byte {})", e.valid_up_to() + 1))?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let table = text
        .parse::<toml::Table>()
        .map_err(|e| toml_error(text, &e))?;
    let mut file = File::default();
    for (name, value) in &table {
        file.read(name, value);
    }
    let keys = file.keys;
    let warnings = std::mem::take(&mut file.warnings);
    let preset = build(file)?;
    Ok(Loaded {
        preset,
        warnings,
        keys,
    })
}

/// `line N: message`, on one line.
fn toml_error(text: &str, e: &toml::de::Error) -> String {
    let msg = e.message().split_whitespace().collect::<Vec<_>>().join(" ");
    one_line(match e.span() {
        Some(span) => {
            let before = text.as_bytes().get(..span.start).unwrap_or(text.as_bytes());
            let line = before.iter().filter(|&&b| b == b'\n').count() + 1;
            format!("line {line}: {msg}")
        }
        None => msg,
    })
}

/// What the file said, entry by entry, after the grammar.
#[derive(Default)]
struct File {
    keys: usize,
    warnings: Vec<String>,
    tema: Option<String>,
    headlines: Vec<String>,
    titles: Vec<String>,
    green: Vec<(String, String, String)>,
    red: Vec<(String, String, String)>,
    people: Vec<String>,
    names: Vec<String>,
    nouns: Vec<Noun>,
    adjs: Vec<String>,
    verbs: Vec<(String, String, String)>,
    phrases: Vec<String>,
}

impl File {
    fn warn(&mut self, w: String) {
        self.warnings.push(one_line(w));
    }

    fn read(&mut self, name: &str, value: &toml::Value) {
        let Some(k) = KEYS.iter().find(|k| k.name == name) else {
            if value.is_table() {
                self.warn(format!("{name}: a table, ignored (keys go at the top)"));
            } else {
                self.warn(format!("{name}: unknown key"));
            }
            return;
        };
        self.keys += 1;
        if k.name == "tema" {
            let Some(s) = value.as_str() else {
                self.warn("tema: expected a string".into());
                return;
            };
            match tidy(k, s) {
                Ok(t) if !t.is_empty() => self.tema = Some(t),
                Ok(_) => {}
                Err(e) => self.warn(format!("tema: {e}")),
            }
            return;
        }
        let entries = self.entries(k, value);
        for (n, entry) in entries {
            let at = |e: String| format!("{}[{n}]: {e}", k.name);
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    self.warn(at(e));
                    continue;
                }
            };
            let done = match k.name {
                "titulares" => gramatica::blanks(&entry).map(|t| self.headlines.push(t)),
                "titulos" => gramatica::blanks(&entry).map(|t| self.titles.push(t)),
                "frases" => gramatica::blanks(&entry).map(|t| self.phrases.push(t)),
                "letrero_verde" | "letrero_rojo" => banner(&entry).map(|b| {
                    if k.name == "letrero_verde" {
                        self.green.push(b);
                    } else {
                        self.red.push(b);
                    }
                }),
                "personas" => {
                    self.people.push(entry);
                    Ok(())
                }
                "nombres" => {
                    self.names.push(entry);
                    Ok(())
                }
                "cosas" => gramatica::noun(&entry).map(|n| self.nouns.push(n)),
                "adjetivos" => gramatica::adjective(&entry).map(|a| self.adjs.push(a)),
                _ => gramatica::verb(&entry).map(|v| self.verbs.push(v)),
            };
            if let Err(e) = done {
                self.warn(at(e));
            }
        }
    }

    /// The entries of a list key, numbered from 1, tidied and checked
    /// against the key's limits: the lines of a block (blank lines and `#`
    /// comments skipped), or the strings of an array.
    fn entries(&mut self, k: &Key, value: &toml::Value) -> Vec<(usize, Result<String, String>)> {
        let raw: Vec<Option<&str>> = match value {
            toml::Value::String(s) => s
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(Some)
                .collect(),
            toml::Value::Array(a) => a
                .iter()
                .map(toml::Value::as_str)
                .filter(|s| s.is_none_or(|s| !s.trim().is_empty()))
                .collect(),
            _ => {
                self.warn(format!("{}: expected ''' lines ''' or a list", k.name));
                return Vec::new();
            }
        };
        if raw.len() > MAX_ENTRIES {
            self.warn(format!(
                "{}[{}]: past the {MAX_ENTRIES}th entry, {} dropped",
                k.name,
                MAX_ENTRIES + 1,
                raw.len() - MAX_ENTRIES
            ));
        }
        raw.into_iter()
            .take(MAX_ENTRIES)
            .enumerate()
            .map(|(i, s)| {
                (
                    i + 1,
                    s.ok_or_else(|| String::from("not a string"))
                        .and_then(|s| tidy(k, s)),
                )
            })
            .collect()
    }
}

/// `gramatica::tidy`, then the key's limit, then the marks the key gives no
/// meaning to: braces outside the keys with blanks, `*` outside `adjetivos`.
fn tidy(k: &Key, s: &str) -> Result<String, String> {
    let t = gramatica::tidy(s)?;
    let n = t.chars().count();
    if n > k.max {
        return Err(format!("{n} characters, {} at most", k.max));
    }
    if !k.blanks && t.contains(['{', '}']) {
        return Err("{ and } have no meaning here".into());
    }
    if !k.star && t.contains('*') {
        return Err("* only marks agreement in adjetivos".into());
    }
    Ok(t)
}

/// A banner line: its two lines in big letters and its caption ("" for none).
fn banner(entry: &str) -> Result<(String, String, String), String> {
    let (top, bottom, caption) = gramatica::banner(entry)?;
    let caption = caption.unwrap_or_default();
    let n = caption.chars().count();
    if n > CAPTION {
        return Err(format!("caption of {n} characters, {CAPTION} at most"));
    }
    Ok((top, bottom, caption))
}

// ---------------------------------------------------------------- interning

/// Every string a preset has handed out: each distinct text is leaked once.
static INTERNED: Mutex<BTreeSet<&'static str>> = Mutex::new(BTreeSet::new());

thread_local! {
    /// How many strings this thread's parses have leaked.
    static LEAKED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn intern(s: &str) -> &'static str {
    let mut set = INTERNED.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(&have) = set.get(s) {
        return have;
    }
    let new: &'static str = s.to_owned().leak();
    set.insert(new);
    LEAKED.with(|n| n.set(n.get() + 1));
    new
}

/// Strings that this thread's parses have added to the interner (a thread-local count:
/// each test runs on its own thread, so nobody else's parse is in it).
#[cfg(test)]
pub fn leaked() -> usize {
    LEAKED.with(std::cell::Cell::get)
}

fn leak<T>(v: Vec<T>) -> &'static [T] {
    Box::leak(v.into_boxed_slice())
}

/// The preset's own list, or the base's when it gave none.
fn or<T>(v: Vec<T>, base: &'static [T]) -> &'static [T] {
    if v.is_empty() { base } else { leak(v) }
}

fn interned<S: AsRef<str>>(list: &[S]) -> Vec<&'static str> {
    list.iter().map(|s| intern(s.as_ref())).collect()
}

/// Appends what is not there yet.
fn push_new(list: &mut Vec<String>, s: String) {
    if !list.contains(&s) {
        list.push(s);
    }
}

// ------------------------------------------------------------- the vocabulary

/// Extensions for the file names made of slugs: `gnu.tar.gz`, `linux.log`.
const EXTS: &[&str] = &[
    ".tar.gz", ".log", ".conf", ".pem", ".bin", ".iso", ".db", ".patch",
];

/// The `Vocab` of spec 8.2 over a copy of the base, checked by the last gate.
fn build(f: File) -> Result<&'static Preset, String> {
    let b = &BASE;
    let bd = &b.dress;

    // Login names: `stallman`.
    let mut logins: Vec<String> = Vec::new();
    for p in &f.people {
        if let Some(l) = gramatica::login(p) {
            push_new(&mut logins, l);
        }
    }
    let mut users = logins.clone();
    if !users.is_empty() {
        for u in b.users {
            if users.len() >= USERS {
                break;
            }
            push_new(&mut users, u.to_string());
        }
    }
    let dress_users = if logins.is_empty() {
        bd.users
    } else {
        // `htop` shows eight columns of a user.
        let mut u: Vec<String> = vec!["root".into()];
        let short = logins.iter().map(|l| l.get(..8).unwrap_or(l).to_string());
        for l in short.chain(bd.users.iter().skip(1).map(|s| s.to_string())) {
            if u.len() >= USERS - 1 {
                break;
            }
            push_new(&mut u, l);
        }
        u.push("nobody".into());
        <[&'static str; USERS]>::try_from(interned(&u)).unwrap_or(bd.users)
    };

    // Slugs: `names` from `nombres` alone, `words` from `nombres` and `cosas`.
    let mut names: Vec<String> = Vec::new();
    for n in &f.names {
        if let Some(s) = gramatica::slug(n) {
            push_new(&mut names, s);
        }
    }
    let mut words: Vec<String> = names.iter().filter(|s| s.len() <= 16).cloned().collect();
    for n in &f.nouns {
        if let Some(s) = gramatica::slug(&n.word).filter(|s| s.len() <= 16) {
            push_new(&mut words, s);
        }
    }
    // The names of things on the screen come from `nombres`, else from `cosas`.
    let pool: Vec<String> = if names.is_empty() {
        words.clone()
    } else {
        names.iter().filter(|s| s.len() <= 16).cloned().collect()
    };
    let up = |s: &String| s.to_ascii_uppercase();
    let glued = |s: &String| s.replace('_', "");
    let kebab = |s: &String| s.replace('_', "-");

    let files: Vec<String> = words
        .iter()
        .zip(EXTS.iter().cycle())
        .map(|(w, e)| format!("{w}{e}"))
        .collect();
    let passwords: Vec<String> = words.iter().map(glued).filter(|p| p.len() >= 3).collect();
    let daemons: Vec<String> = pool
        .iter()
        .map(glued)
        .filter(|d| d.len() <= 12)
        .map(|d| format!("{d}d"))
        .collect();
    let hosts: Vec<String> = pool.iter().map(kebab).collect();
    let rain: Vec<String> = pool
        .iter()
        .map(|s| up(&glued(s)))
        .filter(|w| (3..=12).contains(&w.len()))
        .collect();

    let mut sats: Vec<String> = pool
        .iter()
        .map(|s| up(&kebab(s)))
        .filter(|s| s.len() <= 10)
        .enumerate()
        .map(|(i, s)| format!("{s}-{}", i + 1))
        .collect();
    // Topped up to five from the base's; with no name of the file's, all the base's.
    for s in bd.sat_names {
        if sats.is_empty() || sats.len() >= 5 {
            break;
        }
        push_new(&mut sats, s.to_string());
    }
    // The market: names of six letters or fewer, every other one climbing.
    let mut symbols: Vec<(&'static str, bool)> = Vec::new();
    for s in pool.iter().map(|s| up(&glued(s))).filter(|s| s.len() <= 6) {
        if !symbols.iter().any(|(have, _)| *have == s) {
            let bull = symbols.len().is_multiple_of(2);
            symbols.push((intern(&s), bull));
        }
    }
    // Topped up to eight from the base's, which has one that climbs.
    for &(s, bull) in bd.symbols {
        if symbols.is_empty() || symbols.len() >= 8 {
            break;
        }
        if !symbols.iter().any(|(have, _)| *have == s) {
            symbols.push((s, bull));
        }
    }

    // "de ..." tails: `del kernel`, `de la licencia`, `de GNU`, `de Richard Stallman`.
    let mut ofs: Vec<String> = Vec::new();
    for n in &f.nouns {
        let of = match n.with_article.strip_prefix("el ") {
            Some(rest) => format!("del {rest}"),
            None => format!("de {}", n.with_article),
        };
        push_new(&mut ofs, of);
    }
    for n in f.names.iter().chain(&f.people) {
        push_new(&mut ofs, format!("de {n}"));
    }

    let nouns: Vec<(&'static str, bool)> = f
        .nouns
        .iter()
        .filter(|n| !n.plural)
        .map(|n| (intern(&n.word), n.fem))
        .collect();
    let things: Vec<&'static str> = f.nouns.iter().map(|n| intern(&n.with_article)).collect();
    let verbs: Vec<(&'static str, &'static str, &'static str)> = f
        .verbs
        .iter()
        .map(|(i, g, p)| (intern(i), intern(g), intern(p)))
        .collect();
    let phrases = interned(&f.phrases);
    let canned: &'static [&'static str] = if phrases.is_empty() {
        b.canned_es
    } else {
        leak(phrases.iter().chain(b.canned_es).copied().collect())
    };

    let mut feeds = b.feeds;
    if let Some(first) = pool.first() {
        let at = |i: usize| pool.get(i % pool.len()).unwrap_or(first);
        feeds[0] = intern(&format!("hexdump -C /dev/{}", at(0)));
        feeds[1] = intern(&format!("tail -f /var/log/{}.log", at(1)));
        feeds[5] = intern(&format!("tree -a /{}", at(2)));
    }
    let domain = match pool.first() {
        Some(s) => intern(&format!("{}.example", kebab(s))),
        None => b.domain,
    };

    // The banners: the first pair of each color dresses the access pane.
    let caption = f.tema.as_ref().map(|t| intern(&format!(" {t} ")));
    let pair = |base: [&'static str; 4], first: Option<&(String, String, String)>| {
        let mut words = base;
        if let Some((top, bottom, cap)) = first {
            words[0] = intern(top);
            words[1] = intern(bottom);
            words[3] = if cap.is_empty() {
                ""
            } else {
                intern(&format!(" {cap} "))
            };
        }
        if let Some(c) = caption {
            words[2] = c;
        }
        words
    };
    let granted = pair(bd.granted, f.green.first());
    let denied = pair(bd.denied, f.red.first());
    let mut banners: Vec<Banner> = Vec::new();
    for (green, list) in [(true, &f.green), (false, &f.red)] {
        for (top, bottom, cap) in list {
            banners.push(Banner {
                green,
                top: intern(top),
                bottom: intern(bottom),
                caption: intern(cap),
            });
        }
    }
    if banners.is_empty() {
        for (green, w) in [(true, bd.granted), (false, bd.denied)] {
            banners.push(Banner {
                green,
                top: w[0],
                bottom: w[1],
                caption: w[3].trim(),
            });
        }
    }
    let pem = match &f.tema {
        Some(t) => {
            let t = t.to_uppercase();
            (
                intern(&format!("-----BEGIN {t}-----")),
                intern(&format!("-----END {t}-----")),
            )
        }
        None => b.pem,
    };

    // The radar's contacts: short names, never a word of a person's name.
    let person_words: Vec<String> = f
        .people
        .iter()
        .flat_map(|p| p.split_whitespace())
        .filter_map(gramatica::slug)
        .map(|s| s.to_ascii_uppercase())
        .collect();
    let mut contacts: Vec<String> = Vec::new();
    for c in names.iter().filter(|s| s.len() <= 9).map(up) {
        if !person_words.contains(&c) {
            push_new(&mut contacts, c);
        }
    }
    // Topped up to eight with the call signs; `NODO1` and on only if the
    // people's names took the call signs themselves.
    let calls = CALLS.iter().map(|c| c.to_string());
    let spare = (1..=10_000).map(|n| format!("NODO{n}"));
    for c in calls.chain(spare) {
        if contacts.len() >= CONTACTS {
            break;
        }
        if !person_words.contains(&c) {
            push_new(&mut contacts, c);
        }
    }

    let vocab = Vocab {
        people: or(interned(&f.people), b.people),
        users: or(interned(&users), b.users),
        things: or(things, b.things),
        places: or(interned(&f.names), b.places),
        files: or(interned(&files), b.files),
        dirs: or(interned(&words), b.dirs),
        daemons: or(interned(&daemons), b.daemons),
        hosts: or(interned(&hosts), b.hosts),
        locals: or(interned(&words), b.locals),
        holy: or(interned(&words), b.holy),
        passwords: or(interned(&passwords), b.passwords),
        headlines: or(interned(&f.headlines), b.headlines),
        nouns: or(nouns, b.nouns),
        adjs: or(interned(&f.adjs), b.adjs),
        ofs: or(interned(&ofs), b.ofs),
        verbs: or(verbs, b.verbs),
        canned_es: canned,
        canned_en: canned,
        domain,
        feeds,
        pem,
        dress: Dress {
            users: dress_users,
            rain_words: or(interned(&rain), bd.rain_words),
            sat_names: or(interned(&sats), bd.sat_names),
            symbols: or(symbols, bd.symbols),
            granted,
            denied,
            ..BASE.dress
        },
        ..BASE
    };
    gate::check(&vocab).map_err(one_line)?;
    Ok(Box::leak(Box::new(Preset {
        vocab: Box::leak(Box::new(vocab)),
        titles: leak(interned(&f.titles)),
        verbatim_tape: !f.headlines.is_empty(),
        banners: leak(banners),
        phrases: leak(phrases),
        contacts: leak(interned(&contacts)),
        transfers: TRANSFERS,
    })))
}

// ------------------------------------------------------------------ rounds

/// Every index once, in a shuffled order, before any repeats; a new round never starts with the last one's last.
pub struct Round {
    order: Vec<usize>,
    /// The next place in `order`; past its end, a new round is shuffled.
    at: usize,
    /// The index drawn last, which the next round may not start with.
    last: Option<usize>,
}

impl Round {
    pub fn new(len: usize) -> Round {
        Round {
            order: (0..len).collect(),
            at: len,
            last: None,
        }
    }

    /// The next index; 0 when `len` is 0.
    pub fn next(&mut self, rng: &mut Rng) -> usize {
        let len = self.order.len();
        if len == 0 {
            return 0;
        }
        if self.at >= len {
            rng.shuffle(&mut self.order);
            if len > 1 && self.order.first().copied() == self.last {
                self.order.swap(0, len - 1);
            }
            self.at = 0;
        }
        let i = self.order.get(self.at).copied().unwrap_or(0);
        self.at += 1;
        self.last = Some(i);
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::hollywood::lexico;

    const EXAMPLE: &str = include_str!("../../../docs/presets/hollywood/software-libre.toml");

    fn load(text: &str) -> Loaded {
        match parse(text.as_bytes()) {
            Ok(l) => l,
            Err(e) => panic!("{e}\n{text}"),
        }
    }

    fn example() -> Loaded {
        load(EXAMPLE)
    }

    /// The example with `key = ...` and its block removed.
    fn without(key: &str) -> String {
        let mut out = String::new();
        let mut skip = false;
        for line in EXAMPLE.lines() {
            if line.starts_with(&format!("{key} =")) {
                skip = line.ends_with("'''");
                continue;
            }
            if skip {
                skip = line != "'''";
                continue;
            }
            out.push_str(line);
            out.push('\n');
        }
        out
    }

    #[test]
    fn the_example_loads_clean() {
        let l = example();
        assert_eq!(l.warnings, Vec::<String>::new());
        assert_eq!(l.keys, 11);
        let p = l.preset;
        assert_eq!(gate::check(p.vocab), Ok(()));
        assert!(p.verbatim_tape);
        assert_eq!(p.titles.len(), 7);
        assert_eq!(p.banners.len(), 4);
        assert_eq!(p.banners.iter().filter(|b| b.green).count(), 3);
        assert_eq!(p.banners.iter().filter(|b| !b.green).count(), 1);
        assert_eq!(
            (p.banners[0].top, p.banners[0].bottom),
            ("SOFTWARE", "LIBRE")
        );
        assert!(p.banners[0].green);
        assert_eq!(p.phrases.len(), 4);
        assert_eq!(p.vocab.headlines.len(), 12);
        assert_eq!(p.transfers, TRANSFERS);
    }

    #[test]
    fn an_empty_file_is_the_base() {
        let l = load("");
        assert_eq!(l.keys, 0);
        assert!(l.warnings.is_empty(), "{:?}", l.warnings);
        let p = l.preset;
        let v = p.vocab;
        assert_eq!(v.headlines, BASE.headlines);
        assert_eq!(v.people, BASE.people);
        assert_eq!(v.canned_es, BASE.canned_es);
        assert_eq!(v.users, BASE.users);
        assert_eq!(v.holy, BASE.holy);
        assert_eq!(v.ofs, BASE.ofs);
        assert_eq!(v.domain, BASE.domain);
        assert_eq!(v.feeds, BASE.feeds);
        assert_eq!(v.pem, BASE.pem);
        assert_eq!(v.dress.users, BASE.dress.users);
        assert_eq!(v.dress.granted, BASE.dress.granted);
        assert_eq!(v.dress.denied, BASE.dress.denied);
        assert_eq!(v.dress.sat_names, BASE.dress.sat_names);
        assert_eq!(v.dress.symbols, BASE.dress.symbols);
        assert!(p.titles.is_empty());
        assert!(p.phrases.is_empty());
        assert!(!p.verbatim_tape);
        let pairs: Vec<_> = p
            .banners
            .iter()
            .map(|b| (b.green, b.top, b.bottom))
            .collect();
        assert_eq!(
            pairs,
            [(true, "ACCESO", "CONCEDIDO"), (false, "ACCESO", "DENEGADO")]
        );
        assert_eq!(p.contacts, &CALLS[..8]);
        // A `titulares` with nothing left in it is no ticker of the author's.
        let l = load("titulares = '''\n# nada\n'''\npersonas = ['Ana {n}']");
        assert_eq!((l.keys, l.warnings.len()), (2, 1));
        assert!(!l.preset.verbatim_tape);
        assert_eq!(l.preset.vocab.headlines, BASE.headlines);
        assert_eq!(l.preset.vocab.people, BASE.people);
        // Comments and blank lines are nothing, too.
        assert_eq!(load("# nada\n\n").keys, 0);
    }

    /// Two entries of each list key.
    const LISTS: &[(&str, [&str; 2])] = &[
        ("titulares", ["GNU no es Unix", "{n} parches de {nombre}"]),
        ("titulos", ["Copyleft", "Código fuente"]),
        (
            "letrero_verde",
            ["SOFTWARE / LIBRE", "GNU / NO ES UNIX / GNU/Linux"],
        ),
        ("letrero_rojo", ["CODIGO / CERRADO", "SIN / FUENTE"]),
        ("personas", ["Richard Stallman", "Linus Torvalds"]),
        ("nombres", ["GNU", "Emacs"]),
        ("cosas", ["el kernel", "las cuatro libertades"]),
        ("adjetivos", ["privativo", "con copyleft"]),
        ("verbos", ["distribuir", "proveer | proveyendo | provisto"]),
        (
            "frases",
            ["Compilando {cosa}: {n} avisos", "\"Hola\" dijo {persona}"],
        ),
    ];

    /// Everything a list key reaches, to compare two presets by.
    fn lists(p: &Preset) -> Vec<String> {
        let v = p.vocab;
        vec![
            format!("{:?}", v.headlines),
            format!("{:?}", p.titles),
            format!("{:?}", p.banners),
            format!("{:?}", v.people),
            format!("{:?}", v.places),
            format!("{:?}", v.things),
            format!("{:?}", v.nouns),
            format!("{:?}", v.adjs),
            format!("{:?}", v.verbs),
            format!("{:?}", p.phrases),
            format!("{:?}", v.canned_es),
        ]
    }

    #[test]
    fn blocks_and_arrays_are_the_same() {
        let mut block = String::new();
        let mut array = String::new();
        for (key, [a, b]) in LISTS {
            block.push_str(&format!("{key} = '''\n{a}\n# a comment\n\n   {b}  \n'''\n"));
            array.push_str(&format!("{key} = ['{a}', '{b}']\n"));
        }
        let (bl, ar) = (load(&block), load(&array));
        assert!(bl.warnings.is_empty(), "{:?}", bl.warnings);
        assert!(ar.warnings.is_empty(), "{:?}", ar.warnings);
        assert_eq!(bl.keys, LISTS.len());
        assert_eq!(lists(bl.preset), lists(ar.preset));
        assert_eq!(
            bl.preset.vocab.people,
            ["Richard Stallman", "Linus Torvalds"]
        );
        assert_eq!(bl.preset.banners[1].caption, "GNU/Linux");
        assert_eq!(bl.preset.banners[1].bottom, "NO ES UNIX");
        assert_eq!(bl.preset.phrases[1], "\"Hola\" dijo {s}");
        // One key at a time, too.
        for (key, [a, b]) in LISTS {
            let bl = load(&format!("{key} = '''\n{a}\n\n# x\n{b}\n'''\n"));
            let ar = load(&format!("{key} = ['{a}', '{b}']\n"));
            assert_eq!(lists(bl.preset), lists(ar.preset), "{key}");
            assert_eq!(bl.keys, 1);
        }
    }

    #[test]
    fn what_each_key_feeds() {
        let p = example().preset;
        let v = p.vocab;
        assert_eq!(v.people, ["Richard Stallman", "Linus Torvalds"]);
        assert!(!v.people.iter().any(|x| BASE.people.contains(x)));
        assert_eq!(
            v.places,
            ["GNU", "Linux", "GPL", "Emacs", "GCC", "Debian", "FSF"]
        );
        assert_eq!(v.things.len(), 10);
        assert!(v.things.contains(&"el código fuente"));
        assert!(v.things.contains(&"las cuatro libertades"));
        assert!(v.nouns.contains(&("licencia", true)));
        assert!(v.nouns.contains(&("kernel", false)));
        assert!(!v.nouns.iter().any(|n| n.0 == "cuatro libertades"));
        assert_eq!(v.nouns.len(), 9);
        assert!(
            v.verbs
                .contains(&("distribuir", "distribuyendo", "distribuid"))
        );
        assert!(v.verbs.contains(&("liberar", "liberando", "liberad")));
        assert_eq!(v.verbs.len(), 6);
        assert!(v.adjs.contains(&"privativ*"));
        assert!(v.adjs.contains(&"con copyleft"));
        for f in [
            "Compilando {o} con GCC: {n} advertencias, 0 errores",
            "{s} revisa el parche {n} de {t}",
            "Clonando el repositorio de {t}: {p}% completado",
            "Licencia verificada: GPL versión {l}",
        ] {
            assert!(v.canned_es.contains(&f), "{f}");
            assert!(p.phrases.contains(&f), "{f}");
        }
        assert!(BASE.canned_es.iter().all(|c| v.canned_es.contains(c)));
        assert_eq!(v.canned_en, v.canned_es);
        assert!(
            v.headlines
                .contains(&"{n} parches recibidos de la comunidad")
        );
        assert!(p.titles.contains(&"Proyecto GNU, 1983"));
        for of in [
            "del kernel",
            "de la licencia",
            "de las cuatro libertades",
            "de GNU",
            "de Richard Stallman",
        ] {
            assert!(v.ofs.contains(&of), "{of}: {:?}", v.ofs);
        }
        assert!(v.users.contains(&"stallman") && v.users.contains(&"torvalds"));
        assert!(v.users.len() >= 6);
        assert!(v.dress.users.iter().all(|u| !u.is_empty()));
        assert!(v.dress.users.contains(&"stallman"));
        for w in ["gnu", "linux", "emacs"] {
            assert!(
                v.dress.rain_words.iter().any(|r| r.eq_ignore_ascii_case(w)),
                "{w}: {:?}",
                v.dress.rain_words
            );
        }
        assert!(v.domain.ends_with(".example"), "{}", v.domain);
        assert!(v.daemons.contains(&"linuxd"), "{:?}", v.daemons);
        assert!(v.holy.contains(&"gnu") && v.holy.contains(&"codigo_fuente"));
        assert!(
            v.dress.sat_names.contains(&"EMACS-4"),
            "{:?}",
            v.dress.sat_names
        );
        assert!(v.dress.symbols.len() >= 8 && v.dress.symbols.iter().any(|s| s.1));
        assert!(v.dress.symbols.contains(&("GNU", true)));
        assert!(
            v.feeds.contains(&"tail -f /var/log/linux.log"),
            "{:?}",
            v.feeds
        );
        // The banner's first pairs dress the access pane.
        assert_eq!(&v.dress.granted[..2], ["SOFTWARE", "LIBRE"]);
        assert_eq!(&v.dress.denied[..2], ["CODIGO", "CERRADO"]);
        // What the file cannot reach is the base's.
        assert_eq!(v.errnos, BASE.errnos);
        assert_eq!(v.mundane, BASE.mundane);
        assert_eq!(v.mundane_es, v.mundane.len());
        assert_eq!(v.dress.key_title, BASE.dress.key_title);
        assert_eq!(v.dress.art, BASE.dress.art);
    }

    #[test]
    fn tema_is_the_caption_or_the_bases() {
        let v = example().preset.vocab;
        assert!(v.dress.granted[2].contains("Software libre"));
        assert!(v.dress.denied[2].contains("Software libre"));
        assert_eq!(v.pem.0, "-----BEGIN SOFTWARE LIBRE-----");

        let p = load(&without("tema")).preset;
        let v = p.vocab;
        assert_eq!(v.dress.granted[2], BASE.dress.granted[2]);
        assert_eq!(v.dress.denied[2], BASE.dress.denied[2]);
        assert_eq!(v.pem, BASE.pem);
        // `parse` never sees a file name, so none can show.
        let mut shown: Vec<&str> = vec![v.domain, v.pem.0, v.pem.1];
        shown.extend(v.dress.granted);
        shown.extend(v.dress.denied);
        shown.extend(v.feeds);
        shown.extend(p.titles);
        shown.extend(v.headlines);
        shown.extend(p.banners.iter().flat_map(|b| [b.top, b.bottom, b.caption]));
        for s in shown {
            let s = s.to_lowercase();
            assert!(!s.contains("software-libre") && !s.contains(".toml"), "{s}");
        }
    }

    #[test]
    fn contacts_are_never_people() {
        let p = example().preset;
        assert!(p.contacts.len() >= 8, "{:?}", p.contacts);
        for c in p.contacts {
            assert!(
                (1..=9).contains(&c.len()) && c.bytes().all(|b| !b.is_ascii_lowercase()),
                "{c}"
            );
            assert!(c.is_ascii(), "{c}");
        }
        assert!(p.contacts.contains(&"GNU") && p.contacts.contains(&"EMACS"));
        // Seven names of nine or fewer: one call sign tops them up (PF7).
        assert_eq!(p.contacts.len(), 8);
        assert_eq!(p.contacts[7], CALLS[0]);
        let people: Vec<String> = p
            .vocab
            .people
            .iter()
            .flat_map(|x| x.split_whitespace())
            .filter_map(gramatica::slug)
            .map(|s| s.to_ascii_uppercase())
            .collect();
        for c in p.contacts {
            assert!(!people.iter().any(|w| w == c), "{c}");
        }
        // A name that is also a person's is left out, and so is a call sign.
        let p = load("personas = ['Linus Torvalds', 'Ana Eco']\nnombres = ['Linux', 'Torvalds']")
            .preset;
        assert!(!p.contacts.contains(&"TORVALDS") && !p.contacts.contains(&"ECO"));
        assert!(p.contacts.contains(&"LINUX") && p.contacts.len() == 8);
        // Ten short names need no call sign; a long one is not a contact.
        let many = "nombres = ['A1', 'B2', 'C3', 'D4', 'E5', 'F6', 'G7', 'H8', 'I9', 'J10', 'Kilometraje']";
        let p = load(many).preset;
        assert_eq!(p.contacts.len(), 10);
        assert!(
            !p.contacts
                .iter()
                .any(|c| CALLS.contains(c) || *c == "KILOMETRAJE")
        );
    }

    /// `text` loads with one warning that starts with `prefix`, and `kept`
    /// still reaches the preset.
    fn one_warning(text: &str, prefix: &str, kept: impl Fn(&Preset) -> bool) {
        let l = load(text);
        assert_eq!(l.warnings.len(), 1, "{text}: {:?}", l.warnings);
        let w = &l.warnings[0];
        assert!(w.starts_with(prefix), "{text}: {w}");
        assert!(w.chars().count() <= LINE && !w.contains('\n'), "{w}");
        assert!(kept(l.preset), "{text}: the other entries are gone");
    }

    #[test]
    fn every_warning_names_its_entry() {
        let people = |p: &Preset| p.vocab.people == ["Ana Pérez"];
        let heads = |p: &Preset| p.vocab.headlines == ["Hola"];
        let names = |p: &Preset| p.vocab.places == ["GNU"];
        let things = |p: &Preset| p.vocab.things == ["el kernel"];
        let verbs = |p: &Preset| p.vocab.verbs == [("liberar", "liberando", "liberad")];
        let green = |p: &Preset| {
            p.banners.len() == 1 && p.banners[0].top == "SOFTWARE" && p.banners[0].green
        };
        let any = |_: &Preset| true;
        // Keys.
        one_warning("titulo = 'x'\npersonas = ['Ana Pérez']", "titulo: ", people);
        one_warning("personas = 3\ntitulares = ['Hola']", "personas: ", heads);
        one_warning("tema = ['x']\npersonas = ['Ana Pérez']", "tema: ", people);
        one_warning(
            "personas = ['Ana Pérez']\n[seccion.uno]\ntitulares = 'x'",
            "seccion: ",
            people,
        );
        one_warning("personas = ['Ana Pérez', 3]", "personas[2]: ", people);
        // Entries over their limit.
        let long = |n: usize| "a".repeat(n);
        for (key, n, kept) in [
            ("titulares", 101, &heads as &dyn Fn(&Preset) -> bool),
            ("personas", 29, &people),
            ("nombres", 21, &names),
        ] {
            let ok = match key {
                "titulares" => "Hola",
                "personas" => "Ana Pérez",
                _ => "GNU",
            };
            one_warning(
                &format!("{key} = ['{ok}', '{}']", long(n)),
                &format!("{key}[2]: "),
                kept,
            );
        }
        one_warning(
            &format!("titulos = ['Uno', '{}']", long(41)),
            "titulos[2]: ",
            |p| p.titles == ["Uno"],
        );
        one_warning(
            &format!("frases = ['Uno', '{}']", long(121)),
            "frases[2]: ",
            |p| p.phrases == ["Uno"],
        );
        one_warning(
            &format!("cosas = ['el kernel', 'la {}']", long(38)),
            "cosas[2]: ",
            things,
        );
        one_warning(
            &format!("tema = '{}'\npersonas = ['Ana Pérez']", long(25)),
            "tema: ",
            people,
        );
        // Within the limits, at the limits.
        for (key, n) in [
            ("titulares", 100),
            ("titulos", 40),
            ("frases", 120),
            ("personas", 28),
            ("nombres", 20),
        ] {
            assert!(
                load(&format!("{key} = ['{}']", long(n)))
                    .warnings
                    .is_empty(),
                "{key}"
            );
        }
        assert!(load(&format!("tema = '{}'", long(24))).warnings.is_empty());
        // The 401st entry and on: one warning.
        let many: Vec<String> = (1..=450).map(|i| format!("Titular {i}")).collect();
        let l = load(&format!("titulares = '''\n{}\n'''", many.join("\n")));
        assert_eq!(l.warnings.len(), 1, "{:?}", l.warnings);
        assert!(
            l.warnings[0].starts_with("titulares[401]: "),
            "{}",
            l.warnings[0]
        );
        assert_eq!(l.preset.vocab.headlines.len(), 400);
        // Characters that take no cell or two.
        one_warning(
            "personas = ['Ana Pérez', 'Wang 世界']",
            "personas[2]: ",
            people,
        );
        one_warning(
            "personas = ['Ana Pérez', 'Ana 😀']",
            "personas[2]: ",
            people,
        );
        one_warning(
            "personas = ['Ana Pérez', \"Ana\\u0007\"]",
            "personas[2]: ",
            people,
        );
        one_warning(
            "tema = 'Software 自由'\npersonas = ['Ana Pérez']",
            "tema: ",
            people,
        );
        one_warning(
            "letrero_verde = ['SOFTWARE / LIBRE', 'GNU / LIBRE / 自由']",
            "letrero_verde[2]: ",
            green,
        );
        // Blanks, braces and stars.
        one_warning(
            "titulares = ['Hola', 'Hola {fecha}']",
            "titulares[2]: ",
            heads,
        );
        one_warning("titulares = ['Hola', 'Hola {n']", "titulares[2]: ", heads);
        one_warning("titulares = ['Hola', 'Hola n}']", "titulares[2]: ", heads);
        one_warning(
            "titulares = ['Hola', 'Hola* a todos']",
            "titulares[2]: ",
            heads,
        );
        one_warning("frases = ['Uno', 'libr* {n}']", "frases[2]: ", |p| {
            p.phrases == ["Uno"]
        });
        one_warning("titulos = ['Uno', 'Dos {x}']", "titulos[2]: ", |p| {
            p.titles == ["Uno"]
        });
        one_warning("cosas = ['el kernel', 'el {cosa}']", "cosas[2]: ", things);
        one_warning("cosas = ['el kernel', 'el parche*']", "cosas[2]: ", things);
        one_warning(
            "personas = ['Ana Pérez', 'Ana {persona}']",
            "personas[2]: ",
            people,
        );
        one_warning("personas = ['Ana Pérez', 'Ana*']", "personas[2]: ", people);
        one_warning("nombres = ['GNU', '{nombre}']", "nombres[2]: ", names);
        one_warning("nombres = ['GNU', 'Linux}']", "nombres[2]: ", names);
        one_warning("nombres = ['GNU', 'GNU*']", "nombres[2]: ", names);
        one_warning("verbos = ['liberar', 'liberar{']", "verbos[2]: ", verbs);
        one_warning("verbos = ['liberar', 'compil*r']", "verbos[2]: ", verbs);
        one_warning(
            "tema = 'Software {n}'\npersonas = ['Ana Pérez']",
            "tema: ",
            people,
        );
        one_warning(
            "tema = 'Software*'\npersonas = ['Ana Pérez']",
            "tema: ",
            people,
        );
        one_warning(
            "letrero_verde = ['SOFTWARE / LIBRE', 'GNU / LIBRE / {persona}']",
            "letrero_verde[2]: ",
            green,
        );
        one_warning(
            "letrero_verde = ['SOFTWARE / LIBRE', 'GNU / LIBRE / libr*']",
            "letrero_verde[2]: ",
            green,
        );
        one_warning("adjetivos = ['libre', 'libr{e}']", "adjetivos[2]: ", |p| {
            p.vocab.adjs == ["libre"]
        });
        // The grammar.
        one_warning("cosas = ['el kernel', 'kernel']", "cosas[2]: ", things);
        one_warning("verbos = ['liberar', 'liberado']", "verbos[2]: ", verbs);
        one_warning("verbos = ['liberar', 'liberarse']", "verbos[2]: ", verbs);
        one_warning(
            "adjetivos = ['libre', 'privativo*']",
            "adjetivos[2]: ",
            |p| p.vocab.adjs == ["libre"],
        );
        // Banners.
        let l = load("letrero_verde = ['SOFTWARE / LIBRE', 'GNU/LINUX / LIBRE']");
        assert_eq!(l.warnings.len(), 1);
        assert!(l.warnings[0].starts_with("letrero_verde[2]: ") && l.warnings[0].contains('/'));
        assert!(green(l.preset));
        one_warning(
            "letrero_verde = ['SOFTWARE / LIBRE', 'ABCDEFGHIJKLM / LIBRE']",
            "letrero_verde[2]: ",
            green,
        );
        one_warning(
            "letrero_verde = ['SOFTWARE / LIBRE', 'ABCDEF GHIJKL / LIBRE']",
            "letrero_verde[2]: ",
            green,
        );
        one_warning(
            "letrero_verde = ['SOFTWARE / LIBRE', 'SOFTWARE LIBRE']",
            "letrero_verde[2]: ",
            green,
        );
        one_warning(
            &format!(
                "letrero_verde = ['SOFTWARE / LIBRE', 'GNU / LIBRE / {}']",
                long(41)
            ),
            "letrero_verde[2]: ",
            green,
        );
        // At their limits: a caption of 40 on two lines of 12, three written forms.
        let l = load(&format!(
            "letrero_verde = ['ABCDEFGHIJKL / ABCDEFGHIJKL / {}']\nverbos = ['{}']",
            long(40),
            "desentumecer | desentumeciendo | desentumecido"
        ));
        assert!(l.warnings.is_empty(), "{:?}", l.warnings);
        assert_eq!(l.preset.banners[0].caption, long(40));
        assert_eq!(l.preset.vocab.verbs[0].1, "desentumeciendo");
        one_warning(
            &format!(
                "verbos = ['liberar', 'proveer | proveyendo | {}o']",
                long(39)
            ),
            "verbos[2]: ",
            verbs,
        );
        one_warning(
            "letrero_rojo = ['NO']\nletrero_verde = ['SOFTWARE / LIBRE']",
            "letrero_rojo[1]: ",
            green,
        );
        // Warnings come in the order of the entries.
        let l = load("personas = ['Ana*', 'Wang 世界', 3, 'a{']\ncosas = ['kernel', 'el {x}']");
        let at: Vec<&str> = l
            .warnings
            .iter()
            .filter_map(|w| w.split(':').next())
            .collect();
        assert_eq!(
            at,
            [
                "cosas[1]",
                "cosas[2]",
                "personas[1]",
                "personas[2]",
                "personas[3]",
                "personas[4]"
            ]
        );
        // Every message is one line of 60 characters or fewer, even at its longest.
        let l = load(&format!(
            "letrero_verde = '''\n{}\n'''",
            vec!["ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKL / X"; 450].join("\n")
        ));
        assert_eq!(l.warnings.len(), 401);
        for w in &l.warnings {
            assert!(w.chars().count() <= LINE, "{w}");
        }
        assert!(any(l.preset));
        // A quoted key cannot put a control character on the screen.
        let l = load("\"a\\u001b[2Jb\" = 1");
        assert_eq!(l.warnings, ["a [2Jb: unknown key"]);
    }

    #[test]
    fn every_error_is_one_line() {
        let errs = [
            parse(&[b't', b'e', 0xff, 0xfe]),
            parse(&vec![b' '; MAX_BYTES + 1]),
            parse(b"titulares = "),
            parse(b"tema = 'x'\n\npersonas = = 3\n"),
            parse(b"tema = 'x'\r\n\r\npersonas = = 3\r\n"),
            parse(b"tema = 'x'\ntema = 'y'\n"),
        ];
        let errs: Vec<String> = errs
            .into_iter()
            .map(|r| match r {
                Err(e) => e,
                Ok(_) => panic!("loaded"),
            })
            .collect();
        for e in &errs {
            assert!(!e.contains('\n') && !e.contains('\r'), "{e:?}");
            assert!(!e.is_empty() && e.chars().count() <= LINE, "{e:?}");
        }
        assert!(errs[0].contains("UTF-8"), "{}", errs[0]);
        assert!(errs[1].contains("256 KiB"), "{}", errs[1]);
        assert!(errs[2].starts_with("line 1: "), "{}", errs[2]);
        assert!(errs[3].starts_with("line 3: "), "{}", errs[3]);
        assert!(errs[4].starts_with("line 3: "), "{}", errs[4]);
        assert!(errs[5].starts_with("line 2: "), "{}", errs[5]);
        // At the limit, a file is read.
        assert!(parse(&vec![b' '; MAX_BYTES]).is_ok());
    }

    #[test]
    fn bom_crlf_and_decomposed_text_load_the_same() {
        let clean = example().preset;
        let texts = [
            format!("\u{feff}{EXAMPLE}"),
            EXAMPLE.replace('\n', "\r\n"),
            EXAMPLE.replace('ó', "o\u{301}"),
        ];
        assert!(texts[2].contains("o\u{301}"));
        for t in &texts {
            let l = load(t);
            assert!(l.warnings.is_empty(), "{:?}", l.warnings);
            assert_eq!(l.keys, 11);
            let p = l.preset;
            assert_eq!(p.vocab.headlines, clean.vocab.headlines);
            assert_eq!(p.titles, clean.titles);
            assert_eq!(p.banners, clean.banners);
            assert_eq!(p.vocab.things, clean.vocab.things);
        }
    }

    #[test]
    fn a_caption_may_have_a_slash() {
        let l = load("letrero_verde = \"SOFTWARE / LIBRE / GNU/Linux\"");
        assert!(l.warnings.is_empty(), "{:?}", l.warnings);
        let b = l.preset.banners;
        assert_eq!(b.len(), 1);
        assert_eq!(
            (b[0].top, b[0].bottom, b[0].caption),
            ("SOFTWARE", "LIBRE", "GNU/Linux")
        );
        assert_eq!(l.preset.vocab.dress.granted[3], " GNU/Linux ");
        // A slash in the letters is a character with no glyph.
        let l = load("letrero_verde = \"GNU/LINUX / LIBRE\"");
        assert_eq!(l.warnings.len(), 1, "{:?}", l.warnings);
        let pairs: Vec<_> = l.preset.banners.iter().map(|b| (b.top, b.bottom)).collect();
        assert_eq!(pairs, [("ACCESO", "CONCEDIDO"), ("ACCESO", "DENEGADO")]);
        assert_eq!(l.preset.vocab.dress.granted, BASE.dress.granted);
        // One color given: only that color.
        let l = load("letrero_rojo = 'CODIGO / CERRADO'");
        assert!(l.preset.banners.iter().all(|b| !b.green));
        assert_eq!(l.preset.vocab.dress.granted, BASE.dress.granted);
        assert_eq!(l.preset.vocab.dress.denied[3], "");
        // Big letters: accents stripped, `Ñ` kept.
        let l = load("letrero_verde = 'Código / Año ¿Qué?'");
        assert!(l.warnings.is_empty(), "{:?}", l.warnings);
        assert_eq!(
            (l.preset.banners[0].top, l.preset.banners[0].bottom),
            ("CODIGO", "AÑO ¿QUE?")
        );
    }

    /// `Ok` with a vocabulary that passes the last gate, or a one-line `Err`.
    fn survives(bytes: &[u8]) {
        match parse(bytes) {
            Ok(l) => {
                assert_eq!(gate::check(l.preset.vocab), Ok(()));
                assert!(!l.preset.banners.is_empty() && l.preset.contacts.len() >= 8);
                for w in &l.warnings {
                    assert!(!w.contains('\n') && w.chars().count() <= LINE, "{w}");
                }
            }
            Err(e) => assert!(!e.contains('\n') && e.chars().count() <= LINE, "{e}"),
        }
    }

    #[test]
    fn hostile_input_never_panics() {
        let big = "a".repeat(10_000);
        let five_thousand: String = (0..5_000).map(|i| format!("el x{i}\n")).collect();
        let values: Vec<&str> = vec![
            "",
            " ",
            "x",
            "ñ",
            "*",
            "{",
            "}",
            "{}",
            "}{",
            "{{persona}}",
            "/",
            " / ",
            " / / / ",
            "|",
            "| |",
            "a | b | c",
            "el",
            "el (f)",
            "los",
            &big,
            "😀",
            "😀 / 😀",
            "世界",
            "e\u{301}\u{302}\u{303}",
            "\u{301}",
            "שלום עולם",
            "\u{202e}abc",
            "مرحبا",
            "\u{200b}",
            "\u{feff}",
            "\t",
            "los parches\nlas libertades\nlos ñus",
            "Ñandú\nPiña\nAño\nÑ",
            "ab\ncd\nef",
            "x-1\ny_2\nz.3\n#4",
            &five_thousand,
        ];
        for v in &values {
            for k in KEYS {
                let name = k.name;
                survives(format!("{name} = '''\n{v}\n'''\n").as_bytes());
                let escaped = v
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n");
                survives(format!("{name} = [\"{escaped}\", \"\"]\n").as_bytes());
                survives(format!("{name} = \"{escaped}\"\n").as_bytes());
            }
        }
        // All keys at once, each with every value.
        let mut all = String::new();
        for k in KEYS {
            let joined: Vec<String> = values
                .iter()
                .map(|v| {
                    v.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n")
                })
                .collect();
            all.push_str(&format!("{} = [\"{}\"]\n", k.name, joined.join("\", \"")));
        }
        survives(all.as_bytes());
        // NUL, raw and escaped.
        survives(b"personas = '''\nA\0B\n'''");
        survives(b"personas = [\"A\\u0000B\"]");
        survives(b"\0");
        // Types and tables.
        survives(b"personas = 3\ntema = 4\ncosas = [1, [2], {a = 3}]\n[x]\n[[y]]\n");
        survives(b"tema = 1979-05-27T07:32:00Z\nverbos = true");
        // A file of 1 MiB.
        assert!(parse(&vec![b'#'; 1 << 20]).is_err());
        // Random bytes, raw and inside a block.
        let mut rng = Rng::with_seed(8);
        let alphabet: Vec<char> = "aeiouñáé ÑAZ09{}*/|#'\"\\\n\t\r(f)elos😀世\u{301}\u{0}-.:!¿?"
            .chars()
            .collect();
        for i in 0..2_000 {
            let n = rng.usize(..300);
            if i % 2 == 0 {
                let bytes: Vec<u8> = (0..n).map(|_| rng.u8(..)).collect();
                survives(&bytes);
            } else {
                let k = &KEYS[rng.usize(..KEYS.len())];
                let body: String = (0..n)
                    .map(|_| alphabet[rng.usize(..alphabet.len())])
                    .collect();
                survives(format!("{} = '''\n{body}\n'''\n", k.name).as_bytes());
            }
        }
    }

    #[test]
    fn the_same_text_leaks_once() {
        let text = "personas = ['Persona Qwertz Exclusiva De Fuga']\nnombres = ['Zyxwvq']";
        let before = leaked();
        assert!(parse(text.as_bytes()).is_ok());
        let first = leaked();
        assert!(first > before, "{before} {first}");
        assert!(parse(text.as_bytes()).is_ok());
        assert_eq!(leaked(), first);
    }

    #[test]
    fn the_preset_writes_clean_text() {
        let p = example().preset;
        let v = p.vocab;
        lexico::assert_writes_clean(v, "example");
        let mut rng = Rng::with_seed(30);
        let mut s = String::new();
        for tpl in v.headlines.iter().chain(p.titles).chain(p.phrases) {
            for _ in 0..1_000 {
                s.clear();
                v.expand(&mut rng, tpl, &mut s);
                assert!(lexico::contracted(&s), "{s:?}");
                assert!(!s.contains(['{', '}', '*']) && !s.contains("  "), "{s:?}");
            }
        }
    }

    #[test]
    fn a_round_goes_through_everything() {
        let mut rng = Rng::with_seed(5);
        for len in [1, 2, 7] {
            let mut r = Round::new(len);
            let mut last = None;
            for _ in 0..50 {
                let mut seen: Vec<usize> = (0..len).map(|_| r.next(&mut rng)).collect();
                if len > 1 {
                    assert_ne!(Some(seen[0]), last, "len {len}");
                    assert!(seen.windows(2).all(|w| w[0] != w[1]));
                }
                last = seen.last().copied();
                seen.sort_unstable();
                assert_eq!(seen, (0..len).collect::<Vec<_>>());
            }
        }
        assert_eq!(Round::new(0).next(&mut rng), 0);
        assert_eq!(Round::new(0).next(&mut rng), 0);
    }
}
