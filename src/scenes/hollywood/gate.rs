#![allow(dead_code)] // wired in Task 10

//! The last gate (spec 8.4): every rule the engine relies on, checked over a
//! whole `Vocab`. A vocabulary that passes cannot make `lexico.rs`, `feed.rs`
//! or the widgets panic: no list they pick from is empty, and the `Dress`
//! keeps the shapes the widgets assume (spec 5).

use super::letrero;
use super::lexico::Vocab;

/// Checks `v` against every rule the engine relies on. The error names the
/// field and the entry's index, never its text, so that it fits one HUD line:
/// `dress.key_title: 15 bytes, at most 14`, `hosts[3]: not ASCII`.
pub fn check(v: &Vocab) -> Result<(), String> {
    // `pick` indexes `rng.usize(..len)`; titles and names are not blank.
    filled("people", v.people, blank)?;
    filled("users", v.users, blank)?;
    filled("things", v.things, blank)?;
    filled("places", v.places, blank)?;
    filled("files", v.files, blank)?;
    filled("dirs", v.dirs, blank)?;
    filled("daemons", v.daemons, blank)?;
    filled("hosts", v.hosts, blank)?;
    filled("errnos", v.errnos, |e| e.0.is_empty() || e.2.is_empty())?;
    filled("locals", v.locals, blank)?;
    filled("errs", v.errs, blank)?;
    filled("holy", v.holy, blank)?;
    filled("passwords", v.passwords, blank)?;
    filled("headlines", v.headlines, blank)?;
    filled("nouns", v.nouns, |n| n.0.is_empty())?;
    filled("adjs", v.adjs, blank)?;
    filled("ofs", v.ofs, blank)?;
    filled("verbs", v.verbs, |t| [t.0, t.1, t.2].contains(&""))?;
    filled("states", v.states, blank)?;
    filled("errors", v.errors, blank)?;
    filled("units", v.units, blank)?;
    filled("canned_es", v.canned_es, blank)?;
    filled("canned_en", v.canned_en, blank)?;
    filled("mundane", v.mundane, blank)?;
    filled("packets", v.packets, |p| {
        p.proto.is_empty() || p.info.is_empty()
    })?;
    filled("dress.rain_words", v.dress.rain_words, blank)?;
    if let Some(i) = v.feeds.iter().position(|s| s.is_empty()) {
        return Err(format!("feeds[{i}]: empty"));
    }

    // `next_verse` reads `lines[verse.min(len - 1)]`.
    if let Some(i) = v.songs.iter().position(|s| s.1.is_empty()) {
        return Err(format!("songs[{i}]: no lines"));
    }
    // K12: the first `mundane_es` lines are Spanish, and there are no more.
    if v.mundane_es > v.mundane.len() {
        return Err(format!(
            "mundane_es: {}, more than the {} lines of mundane",
            v.mundane_es,
            v.mundane.len()
        ));
    }
    // `rng.u16(a..=b)`.
    if let Some((i, p)) = v
        .packets
        .iter()
        .enumerate()
        .find(|(_, p)| p.len.0 > p.len.1)
    {
        return Err(format!(
            "packets[{i}]: length {} to {}, out of order",
            p.len.0, p.len.1
        ));
    }

    let d = &v.dress;
    // `Keygen` draws `17 - 3 - len` dashes after the title.
    if d.key_title.len() > 14 {
        return Err(format!(
            "dress.key_title: {} bytes, at most 14",
            d.key_title.len()
        ));
    }
    // `Rain` spells its words byte by byte.
    if let Some(i) = d
        .rain_words
        .iter()
        .position(|w| !w.is_ascii() || w.len() < 3)
    {
        return Err(format!(
            "dress.rain_words[{i}]: not ASCII of 3 bytes or more"
        ));
    }
    // The trackers: five satellites, a market of eight with one that climbs.
    if d.sat_names.len() < 5 {
        return Err(format!(
            "dress.sat_names: {} names, at least 5",
            d.sat_names.len()
        ));
    }
    if d.symbols.len() < 8 {
        return Err(format!(
            "dress.symbols: {} symbols, at least 8",
            d.symbols.len()
        ));
    }
    if !d.symbols.iter().any(|s| s.1) {
        return Err("dress.symbols: none is bullish".to_string());
    }
    if let Some(i) = d.users.iter().position(|u| u.is_empty()) {
        return Err(format!("dress.users[{i}]: empty"));
    }
    // The banner's two lines, in big letters; `letrero::fit` needs a word in each.
    for (name, words) in [("dress.granted", &d.granted), ("dress.denied", &d.denied)] {
        for (i, w) in words.iter().enumerate().take(2) {
            if w.split_whitespace().next().is_none() {
                return Err(format!("{name}[{i}]: no word"));
            }
            if let Some(ch) = w.chars().find(|&c| c != ' ' && !letrero::has_glyph(c)) {
                return Err(format!("{name}[{i}]: {ch:?} has no big letter"));
            }
        }
    }

    // Identifiers.
    ascii("passwords", v.passwords)?;
    ascii("holy", v.holy)?;
    ascii("locals", v.locals)?;
    ascii("errs", v.errs)?;
    ascii("files", v.files)?;
    ascii("dirs", v.dirs)?;
    ascii("daemons", v.daemons)?;
    ascii("hosts", v.hosts)
}

fn blank(s: &&str) -> bool {
    s.is_empty()
}

/// `list` has an entry, and none of its entries is `blank`.
fn filled<T>(name: &str, list: &[T], blank: impl Fn(&T) -> bool) -> Result<(), String> {
    if list.is_empty() {
        return Err(format!("{name}: empty, at least one entry"));
    }
    match list.iter().position(blank) {
        Some(i) => Err(format!("{name}[{i}]: empty")),
        None => Ok(()),
    }
}

/// Every entry of `list` is ASCII.
fn ascii(name: &str, list: &[&str]) -> Result<(), String> {
    match list.iter().position(|s| !s.is_ascii()) {
        Some(i) => Err(format!("{name}[{i}]: not ASCII")),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::hollywood::Tone;
    use crate::scenes::hollywood::lexico::{self, Dress, Packet};
    use crate::scenes::hollywood::navidad;

    /// A way to break a vocabulary.
    type Break = fn(&mut Vocab);

    /// A copy of Navidad's vocabulary, field by field.
    fn copy() -> Vocab {
        let n = &navidad::VOCAB;
        Vocab {
            dress: Dress { ..n.dress },
            ..*n
        }
    }

    /// The error `check` gives Navidad's vocabulary once `f` has broken it.
    fn broken(f: impl FnOnce(&mut Vocab)) -> String {
        let mut v = copy();
        f(&mut v);
        check(&v).expect_err("the broken vocabulary passed")
    }

    /// Asserts that the error is about `field`.
    fn names(e: &str, field: &str) {
        assert!(
            e.starts_with(&format!("{field}:")) || e.starts_with(&format!("{field}[")),
            "{field}: {e}"
        );
    }

    #[test]
    fn the_holidays_pass() {
        for v in lexico::ALL {
            assert_eq!(check(v), Ok(()), "{}", v.feeds[0]);
        }
    }

    #[test]
    fn every_rule_fires() {
        // Lists the engine picks from without looking.
        let empty: &[(&str, Break)] = &[
            ("people", |v| v.people = &[]),
            ("users", |v| v.users = &[]),
            ("things", |v| v.things = &[]),
            ("places", |v| v.places = &[]),
            ("files", |v| v.files = &[]),
            ("dirs", |v| v.dirs = &[]),
            ("daemons", |v| v.daemons = &[]),
            ("hosts", |v| v.hosts = &[]),
            ("errnos", |v| v.errnos = &[]),
            ("locals", |v| v.locals = &[]),
            ("errs", |v| v.errs = &[]),
            ("holy", |v| v.holy = &[]),
            ("passwords", |v| v.passwords = &[]),
            ("headlines", |v| v.headlines = &[]),
            ("nouns", |v| v.nouns = &[]),
            ("adjs", |v| v.adjs = &[]),
            ("ofs", |v| v.ofs = &[]),
            ("verbs", |v| v.verbs = &[]),
            ("states", |v| v.states = &[]),
            ("errors", |v| v.errors = &[]),
            ("units", |v| v.units = &[]),
            ("canned_es", |v| v.canned_es = &[]),
            ("canned_en", |v| v.canned_en = &[]),
            ("mundane", |v| {
                v.mundane = &[];
                v.mundane_es = 0;
            }),
            ("packets", |v| v.packets = &[]),
            ("dress.rain_words", |v| v.dress.rain_words = &[]),
        ];
        for (field, f) in empty {
            names(&broken(f), field);
        }

        // Entries that are empty strings.
        names(&broken(|v| v.things = &["reno", ""]), "things");
        names(&broken(|v| v.nouns = &[("", true)]), "nouns");
        names(&broken(|v| v.verbs = &[("ir", "", "id")]), "verbs");
        names(&broken(|v| v.errnos = &[("EPINATA", 7, "")]), "errnos");
        names(
            &broken(|v| {
                v.packets = &[Packet {
                    proto: "",
                    tone: Tone::Main,
                    len: (40, 90),
                    info: "x",
                }]
            }),
            "packets",
        );
        names(&broken(|v| v.feeds[3] = ""), "feeds");

        // A song without lines.
        names(&broken(|v| v.songs = &[("silencio", &[])]), "songs");

        // K12: at most, not less than.
        let e = broken(|v| v.mundane_es = v.mundane.len() + 1);
        names(&e, "mundane_es");
        let mut all_es = copy();
        all_es.mundane_es = all_es.mundane.len();
        assert_eq!(check(&all_es), Ok(()));

        // Packet lengths out of order.
        names(
            &broken(|v| {
                v.packets = &[Packet {
                    proto: "UDP",
                    tone: Tone::Main,
                    len: (91, 90),
                    info: "x",
                }]
            }),
            "packets",
        );

        // The randomart's title: 14 bytes fit, 15 do not.
        assert_eq!(
            broken(|v| v.dress.key_title = "[NAVIDAD  4096]"),
            "dress.key_title: 15 bytes, at most 14"
        );
        let mut fits = copy();
        fits.dress.key_title = "[NAVIDAD 4096]";
        assert_eq!(check(&fits), Ok(()));

        // Rain words: ASCII, three bytes or more.
        names(
            &broken(|v| v.dress.rain_words = &["NIEVE", "PI\u{00d1}ATA"]),
            "dress.rain_words",
        );
        names(
            &broken(|v| v.dress.rain_words = &["NIEVE", "OH"]),
            "dress.rain_words",
        );

        // The trackers.
        names(
            &broken(|v| v.dress.sat_names = &["A", "B", "C", "D"]),
            "dress.sat_names",
        );
        names(
            &broken(|v| v.dress.symbols = &[("A", true); 7]),
            "dress.symbols",
        );
        names(
            &broken(|v| v.dress.symbols = &[("A", false); 8]),
            "dress.symbols",
        );

        // htop's users.
        names(&broken(|v| v.dress.users[2] = ""), "dress.users");

        // The banner.
        names(&broken(|v| v.dress.granted[0] = ""), "dress.granted");
        names(
            &broken(|v| v.dress.granted[1] = "S\u{00cd}"),
            "dress.granted",
        );
        names(&broken(|v| v.dress.denied[0] = ""), "dress.denied");
        names(&broken(|v| v.dress.denied[1] = "NO~"), "dress.denied");
        assert_eq!(
            broken(|v| v.dress.granted[1] = "   "),
            "dress.granted[1]: no word"
        );

        // Identifiers.
        let ascii: &[(&str, Break)] = &[
            ("passwords", |v| v.passwords = &["pi\u{00f1}ata"]),
            ("holy", |v| v.holy = &["pi\u{00f1}ata"]),
            ("locals", |v| v.locals = &["pi\u{00f1}ata"]),
            ("errs", |v| v.errs = &["Pi\u{00f1}ata"]),
            ("files", |v| v.files = &["pi\u{00f1}ata.tar"]),
            ("dirs", |v| v.dirs = &["pi\u{00f1}atas"]),
            ("daemons", |v| v.daemons = &["pi\u{00f1}atad"]),
            ("hosts", |v| v.hosts = &["pi\u{00f1}ata"]),
        ];
        for (field, f) in ascii {
            names(&broken(f), field);
        }
    }

    #[test]
    fn errors_fit_the_hud_line() {
        // An entry of any length still gives a message for one HUD line.
        const LONG: &str = "pi\u{00f1}ata_de_siete_picos_rellena_de_colaci\u{00f3}n_y_tejocotes";
        const RAIN: &str = "PI\u{00d1}ATA_DE_SIETE_PICOS_RELLENA_DE_COLACI\u{00d3}N_Y_TEJOCOTES";
        let breaks: &[Break] = &[
            |v| v.passwords = &[LONG],
            |v| v.holy = &[LONG],
            |v| v.locals = &[LONG],
            |v| v.errs = &[LONG],
            |v| v.files = &[LONG],
            |v| v.dirs = &[LONG],
            |v| v.daemons = &[LONG],
            |v| v.hosts = &[LONG],
            |v| v.dress.rain_words = &[RAIN],
            |v| v.dress.granted[1] = RAIN,
        ];
        for f in breaks {
            let e = broken(f);
            assert!(e.chars().count() <= 60, "{e}");
        }
    }

    #[test]
    fn the_banner_takes_letreros_glyphs() {
        let mut v = copy();
        v.dress.granted[1] = "A\u{00d1}O \u{00bf}?";
        v.dress.denied[0] = "NO. -+";
        assert_eq!(check(&v), Ok(()));
    }
}
