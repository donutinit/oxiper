//! The last good preset, by path (spec 8.4, K5): where a preset file lives,
//! and one process-wide cache of what each path last loaded. A scene is built
//! again on every `r`, arrow key and option change; it asks here, and an
//! unchanged file costs one `stat`. A file that is bad now (half saved,
//! broken, emptied by an editor, deleted) keeps the preset that last loaded
//! from it, so the generic screen never reaches the camera; only a path that
//! has never loaded answers with an error.

use super::preset::{self, Loaded, Preset};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

/// What a load of a preset file found.
pub enum Answer {
    /// The file as it is on disk.
    Good {
        preset: &'static Preset,
        warnings: Vec<String>,
    },
    /// The file is bad now: the last good preset from this path, why, and
    /// which failed save it is.
    Kept {
        preset: &'static Preset,
        why: String,
        save: u32,
    },
    /// This path has never loaded: why, and which failed save it is.
    Bad(String, u32),
}

impl Answer {
    pub fn preset(&self) -> Option<&'static Preset> {
        match self {
            Answer::Good { preset, .. } | Answer::Kept { preset, .. } => Some(preset),
            Answer::Bad(..) => None,
        }
    }
}

/// A bare name is `<config_dir>/presets/hollywood/<name>.toml`; a value with a `/`
/// or ending in `.toml` is a path (a leading `~/` is the home directory).
/// With no config dir, a bare name is looked for under where oxiper was started.
pub fn locate(value: &str, config_dir: Option<&Path>) -> PathBuf {
    if value.contains('/') || value.ends_with(".toml") {
        if let (Some(rest), Some(home)) = (value.strip_prefix("~/"), std::env::var_os("HOME")) {
            return Path::new(&home).join(rest);
        }
        return PathBuf::from(value);
    }
    config_dir
        .unwrap_or(Path::new(""))
        .join("presets")
        .join("hollywood")
        .join(format!("{value}.toml"))
}

/// The `preset` option of a `[scenes.hollywood]` table. Ok(None): not set, or "".
/// Err: set and not a string; the message says to quote the name.
pub fn wanted(table: &toml::Table, config_dir: Option<&Path>) -> Result<Option<PathBuf>, String> {
    match table.get("preset") {
        None => Ok(None),
        Some(toml::Value::String(s)) if s.is_empty() => Ok(None),
        Some(toml::Value::String(s)) => Ok(Some(locate(s, config_dir))),
        Some(v) => Err(format!(
            "expected a string, found {}: quote the name",
            v.type_str()
        )),
    }
}

/// A file's stamp: its mtime and length.
type Stamp = (SystemTime, u64);

/// What one path last showed.
struct Entry {
    /// The stamp of the file last read; `None` when it was not there or could
    /// not be read, so the next load looks again.
    stamp: Option<Stamp>,
    /// The last preset that loaded from this path, and the bytes it came
    /// from: a save that changes nothing (`touch`) answers with it as it is.
    good: Option<Loaded>,
    bytes: Vec<u8>,
    /// Why the file is bad right now.
    failure: Option<String>,
    /// Failed saves so far: a scene that said one failure says the next,
    /// even with the same reason (spec 8.6).
    failures: u32,
    /// The stamp seen at the last failure (`None` inside: no file); `None`
    /// once a load succeeds. A file that fails again as it was, unreadable
    /// or still gone, is the same failed save.
    failed_at: Option<Option<Stamp>>,
    #[cfg(test)]
    parses: u32,
}

impl Entry {
    fn answer(&self) -> Answer {
        match (&self.good, &self.failure) {
            (Some(l), None) => Answer::Good {
                preset: l.preset,
                warnings: l.warnings.clone(),
            },
            (Some(l), Some(why)) => Answer::Kept {
                preset: l.preset,
                why: why.clone(),
                save: self.failures,
            },
            (None, why) => Answer::Bad(
                why.clone().unwrap_or_else(|| "not loaded".into()),
                self.failures,
            ),
        }
    }

    /// The file at stamp `seen` (`None`: not there) failed: a new failed save
    /// unless it is the one the last failure saw.
    fn fail(&mut self, why: String, seen: Option<Stamp>) -> Answer {
        if self.failed_at != Some(seen) {
            self.failures = self.failures.wrapping_add(1);
            self.failed_at = Some(seen);
        }
        self.failure = Some(why);
        self.answer()
    }
}

/// Every path that has been asked for, with what it last showed.
static CACHE: Mutex<Option<HashMap<PathBuf, Entry>>> = Mutex::new(None);

/// The file at `path`, from the cache while its stamp is unchanged.
pub fn load(path: &Path) -> Answer {
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let entry = cache
        .get_or_insert_with(HashMap::new)
        .entry(path.to_path_buf())
        .or_insert_with(|| Entry {
            stamp: None,
            good: None,
            bytes: Vec::new(),
            failure: None,
            failures: 0,
            failed_at: None,
            #[cfg(test)]
            parses: 0,
        });
    let stamp = match look(path) {
        Ok(stamp) => stamp,
        Err(why) => {
            entry.stamp = None;
            return entry.fail(why, None);
        }
    };
    if entry.stamp == Some(stamp) {
        return entry.answer();
    }
    let bytes = match read(path) {
        Ok(bytes) => bytes,
        Err(why) => {
            entry.stamp = None;
            return entry.fail(why, Some(stamp));
        }
    };
    entry.stamp = Some(stamp);
    // The very file that last loaded: the same preset, not a new one to dress.
    if entry.good.is_some() && bytes == entry.bytes {
        entry.failure = None;
        entry.failed_at = None;
        return entry.answer();
    }
    #[cfg(test)]
    {
        entry.parses += 1;
    }
    match preset::parse(&bytes) {
        // An editor that truncates before it writes: keep what had keys.
        Ok(l) if l.keys == 0 && entry.good.as_ref().is_some_and(|g| g.keys > 0) => {
            let why = l.warnings.into_iter().next();
            entry.fail(
                why.unwrap_or_else(|| "the file is empty".into()),
                Some(stamp),
            )
        }
        Ok(l) => {
            entry.good = Some(l);
            entry.bytes = bytes;
            entry.failure = None;
            entry.failed_at = None;
            entry.answer()
        }
        Err(why) => entry.fail(why, Some(stamp)),
    }
}

/// The stamp of a regular file, without opening it.
fn look(path: &Path) -> Result<Stamp, String> {
    let meta = std::fs::metadata(path).map_err(|e| io_error(&e))?;
    if !meta.is_file() {
        return Err("not a file".into());
    }
    let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    Ok((mtime, meta.len()))
}

/// The file's bytes, one past the most `preset::parse` takes at most, so a
/// file that grew past the limit is refused without reading all of it.
fn read(path: &Path) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(path).map_err(|e| io_error(&e))?;
    // Swapped for something else since `look`: still never read a device.
    if !file.metadata().is_ok_and(|m| m.is_file()) {
        return Err("not a file".into());
    }
    let mut bytes = Vec::new();
    file.take(preset::MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| io_error(&e))?;
    Ok(bytes)
}

/// An I/O error as a short reason, without the OS's error number.
fn io_error(e: &std::io::Error) -> String {
    match e.kind() {
        std::io::ErrorKind::NotFound => "no such file".into(),
        kind => kind.to_string(),
    }
}

/// The HUD's form of a reason: `preset: <why>`, one line, 60 characters at most.
pub fn brief(why: &str) -> String {
    preset::one_line(format!("preset: {why}"))
}

/// How often this path was parsed.
#[cfg(test)]
pub fn parses(path: &Path) -> u32 {
    let cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    cache
        .as_ref()
        .and_then(|c| c.get(path))
        .map_or(0, |e| e.parses)
}

/// A file of this test's own under the temporary directory, holding `text`.
#[cfg(test)]
pub fn temp(name: &str, text: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("oxiper-preset-{}-{name}.toml", std::process::id()));
    std::fs::write(&path, text).expect("write a test file");
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::hollywood::neutro::BASE;

    const GOOD: &str = "tema = 'software libre'\n";
    const OTHER: &str = "tema = 'software libre'\nnombres = ['GNU', 'Linux']\n";

    fn good(a: &Answer) -> &'static Preset {
        match a {
            Answer::Good { preset, .. } => preset,
            _ => panic!("expected Good, got {}", show(a)),
        }
    }

    fn kept(a: &Answer) -> (&'static Preset, &str) {
        match a {
            Answer::Kept { preset, why, .. } => (preset, why),
            _ => panic!("expected Kept, got {}", show(a)),
        }
    }

    fn show(a: &Answer) -> String {
        match a {
            Answer::Good { warnings, .. } => format!("Good {warnings:?}"),
            Answer::Kept { why, .. } => format!("Kept {why:?}"),
            Answer::Bad(why, _) => format!("Bad {why:?}"),
        }
    }

    fn is_bad(a: &Answer) -> bool {
        matches!(a, Answer::Bad(why, _) if !why.is_empty() && !why.contains('\n'))
    }

    #[test]
    fn a_name_is_a_file_under_the_config_dir() {
        let dir = Some(Path::new("/c/oxiper"));
        assert_eq!(
            locate("software-libre", dir),
            Path::new("/c/oxiper/presets/hollywood/software-libre.toml")
        );
        assert_eq!(locate("borrador.toml", dir), Path::new("borrador.toml"));
        assert_eq!(locate("a/b", dir), Path::new("a/b"));
        assert_eq!(locate("/abs/p", dir), Path::new("/abs/p"));
        if let Some(home) = std::env::var_os("HOME") {
            let p = locate("~/p.toml", dir);
            assert!(p.starts_with(&home), "{p:?}");
            assert_eq!(p, Path::new(&home).join("p.toml"));
        }
    }

    #[test]
    fn the_option_must_be_a_string() {
        let dir = Some(Path::new("/c/oxiper"));
        let table = |s: &str| s.parse::<toml::Table>().unwrap();
        assert_eq!(wanted(&table(""), dir), Ok(None));
        assert_eq!(wanted(&table("preset = ''"), dir), Ok(None));
        assert_eq!(
            wanted(&table("preset = 'x'"), dir),
            Ok(Some(locate("x", dir)))
        );
        for bad in ["preset = 2026", "preset = true"] {
            let e = wanted(&table(bad), dir).unwrap_err();
            assert!(e.contains("quote"), "{e}");
            assert!(!e.contains('\n') && e.chars().count() <= 60, "{e}");
        }
    }

    #[test]
    fn an_unchanged_file_is_not_read_again() {
        let path = temp("unchanged", GOOD);
        let first = good(&load(&path));
        for _ in 0..1000 {
            let p = load(&path).preset().unwrap();
            assert!(std::ptr::eq(p, first));
        }
        assert_eq!(parses(&path), 1);
        std::fs::remove_file(&path).ok();
    }

    /// A save that changes nothing (`touch`, or the file written back as it
    /// was) is the preset it was: not parsed, not leaked, the same pointer.
    #[test]
    fn the_same_bytes_are_the_same_preset() {
        let path = temp("same-bytes", GOOD);
        let first = good(&load(&path));
        let (n, leaked) = (parses(&path), preset::leaked());
        let f = std::fs::File::options().write(true).open(&path).unwrap();
        for secs in [1_000_000, 2_000_000] {
            f.set_modified(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs))
                .unwrap();
            assert!(std::ptr::eq(good(&load(&path)), first));
        }
        // Broken, then written back as it was: the same preset again.
        std::fs::write(&path, "tema = \n").unwrap();
        kept(&load(&path));
        std::fs::write(&path, GOOD).unwrap();
        assert!(std::ptr::eq(good(&load(&path)), first));
        assert_eq!(parses(&path), n + 1);
        assert_eq!(preset::leaked(), leaked);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn a_bad_save_keeps_the_last_good() {
        let path = temp("bad-save", GOOD);
        let first = good(&load(&path));
        std::fs::write(&path, "tema = \n").unwrap();
        let a = load(&path);
        let (p, why) = kept(&a);
        assert!(std::ptr::eq(p, first));
        assert!(why.contains("line"), "{why}");
        let n = parses(&path);
        let a = load(&path);
        let (p, _) = kept(&a);
        assert!(std::ptr::eq(p, first));
        assert_eq!(parses(&path), n);
        std::fs::write(&path, OTHER).unwrap();
        let second = good(&load(&path));
        assert!(!std::ptr::eq(second, first));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn an_emptied_file_keeps_the_last_good() {
        let path = temp("emptied", GOOD);
        let first = good(&load(&path));
        std::fs::write(&path, "").unwrap();
        let a = load(&path);
        let (p, why) = kept(&a);
        assert!(std::ptr::eq(p, first));
        assert_eq!(why, "the file is empty");
        std::fs::remove_file(&path).ok();
    }

    /// A file whose only key is a typo is not "empty": the reason is the typo.
    #[test]
    fn a_file_of_typos_keeps_the_last_good() {
        let path = temp("typo", GOOD);
        let first = good(&load(&path));
        std::fs::write(&path, "titulo = \"x\"\n").unwrap();
        let a = load(&path);
        let (p, why) = kept(&a);
        assert!(std::ptr::eq(p, first));
        assert_eq!(why, "titulo: unknown key");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn a_deleted_file_keeps_the_last_good() {
        let path = temp("deleted", GOOD);
        let first = good(&load(&path));
        std::fs::remove_file(&path).unwrap();
        let a = load(&path);
        let (p, why) = kept(&a);
        assert!(std::ptr::eq(p, first));
        assert!(!why.is_empty() && !why.contains('\n'), "{why}");
    }

    /// Which failed save an answer is.
    fn save(a: &Answer) -> u32 {
        match a {
            Answer::Kept { save, .. } | Answer::Bad(_, save) => *save,
            Answer::Good { .. } => panic!("expected a failure, got {}", show(a)),
        }
    }

    /// Each save that fails is a new failed save, with the same reason or
    /// not; a file that fails again as it was is not.
    #[test]
    fn each_failed_save_is_counted() {
        let path = temp("failed-saves", GOOD);
        good(&load(&path));
        std::fs::write(&path, "tema = \n").unwrap();
        let a = load(&path);
        let (one, why) = (save(&a), kept(&a).1.to_string());
        assert_eq!(save(&load(&path)), one);
        std::fs::write(&path, "tema = \n# otra vez\n").unwrap();
        let a = load(&path);
        assert_eq!(kept(&a).1, why);
        let two = save(&a);
        assert_ne!(two, one);
        std::fs::remove_file(&path).unwrap();
        let three = save(&load(&path));
        assert_ne!(three, two);
        for _ in 0..5 {
            assert_eq!(save(&load(&path)), three);
        }
        // Good again, then gone again: a new failed save.
        std::fs::write(&path, OTHER).unwrap();
        good(&load(&path));
        std::fs::remove_file(&path).unwrap();
        assert_ne!(save(&load(&path)), three);
        // Never there: the same failed save every time.
        let missing = std::env::temp_dir().join(format!(
            "oxiper-preset-{}-never-there.toml",
            std::process::id()
        ));
        let n = save(&load(&missing));
        assert_eq!(save(&load(&missing)), n);
    }

    #[test]
    fn nothing_good_before_is_an_error() {
        let broken = temp("never-good", "tema = \n");
        assert!(is_bad(&load(&broken)), "{}", show(&load(&broken)));
        std::fs::remove_file(&broken).ok();
        let missing = std::env::temp_dir().join(format!(
            "oxiper-preset-{}-not-there.toml",
            std::process::id()
        ));
        assert!(is_bad(&load(&missing)), "{}", show(&load(&missing)));
        let empty = temp("never-empty", "");
        let a = load(&empty);
        let p = good(&a);
        assert_eq!(p.vocab.headlines, BASE.headlines);
        assert!(p.titles.is_empty() && !p.verbatim_tape);
        std::fs::remove_file(&empty).ok();
    }

    #[test]
    fn paths_do_not_share() {
        let a = temp("share-a", GOOD);
        let b = temp("share-b", GOOD);
        let pa = good(&load(&a));
        let pb = good(&load(&b));
        assert!(!std::ptr::eq(pa, pb));
        assert!(std::ptr::eq(load(&a).preset().unwrap(), pa));
        assert_eq!((parses(&a), parses(&b)), (1, 1));
        std::fs::remove_file(&a).ok();
        std::fs::remove_file(&b).ok();
    }

    /// Review Focus 5: nothing that is not a regular file is read.
    #[test]
    fn not_a_regular_file_is_an_error_at_once() {
        let dir = std::env::temp_dir();
        assert!(is_bad(&load(&dir)), "{}", show(&load(&dir)));
        assert_eq!(parses(&dir), 0);
        #[cfg(unix)]
        {
            let zero = Path::new("/dev/zero");
            assert!(is_bad(&load(zero)), "{}", show(&load(zero)));
            assert_eq!(parses(zero), 0);
        }
    }

    /// A file past the limit is read only to its first byte too many.
    #[test]
    fn a_big_file_is_read_to_the_limit_and_no_further() {
        let big = "#".repeat(preset::MAX_BYTES + 4096);
        let path = temp("big", &big);
        assert_eq!(read(&path).map(|b| b.len()), Ok(preset::MAX_BYTES + 1));
        match load(&path) {
            Answer::Bad(why, _) => assert_eq!(why, "the file is over 256 KiB"),
            a => panic!("expected Bad, got {}", show(&a)),
        }
        assert_eq!(
            brief("the file is over 256 KiB"),
            "preset: the file is over 256 KiB"
        );
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn brief_fits_the_hud() {
        for why in ["x".repeat(200), "line 1: one\nline 2: two".into()] {
            let b = brief(&why);
            assert!(b.starts_with("preset: "), "{b}");
            assert!(!b.contains('\n'), "{b:?}");
            assert!(b.chars().count() <= 60, "{b}");
        }
        assert_eq!(brief("the file is empty"), "preset: the file is empty");
        // Cut once, after the prefix: the reason keeps as much as fits.
        assert_eq!(brief(&"x".repeat(200)).chars().count(), 60);
    }
}
