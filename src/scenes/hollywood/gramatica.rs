#![allow(dead_code)] // wired in Task 10

//! The grammar a preset is written in (spec 7.4, 7.5, 8.2): the pure text
//! rules that turn what an author writes into the engine's forms. Nouns
//! carry their article, adjectives agree by `*`, verbs are written in the
//! infinitive and conjugated here, blanks have Spanish names, banners are
//! big letters. Every function takes any `&str` and never panics; an `Err`
//! is the text of a warning, without the key the loader puts before it.

use super::letrero;
use crate::render;

/// Composes the Spanish combining marks, collapses runs of spaces, trims.
/// Err for a character that does not take one cell (control, wide, a stray combining mark).
pub fn tidy(s: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.trim().chars().peekable();
    while let Some(c) = chars.next() {
        let c = match chars.peek().and_then(|&mark| compose(c, mark)) {
            Some(k) => {
                chars.next();
                k
            }
            None => c,
        };
        let code = c as u32;
        if c.is_control() {
            return Err(format!("control character U+{code:04X}"));
        }
        if render::is_wide(c) {
            return Err(format!("U+{code:04X} takes two cells"));
        }
        if takes_no_cell(c) {
            return Err(format!("U+{code:04X} takes no cell"));
        }
        if c == ' ' && out.ends_with(' ') {
            continue;
        }
        out.push(c);
    }
    Ok(out)
}

/// The Spanish letters a file saved in decomposed form spells as a letter
/// and a combining mark: (letter, mark, composed).
const MARKS: &[(char, char, char)] = &[
    ('a', '\u{301}', 'á'),
    ('e', '\u{301}', 'é'),
    ('i', '\u{301}', 'í'),
    ('o', '\u{301}', 'ó'),
    ('u', '\u{301}', 'ú'),
    ('A', '\u{301}', 'Á'),
    ('E', '\u{301}', 'É'),
    ('I', '\u{301}', 'Í'),
    ('O', '\u{301}', 'Ó'),
    ('U', '\u{301}', 'Ú'),
    ('n', '\u{303}', 'ñ'),
    ('N', '\u{303}', 'Ñ'),
    ('u', '\u{308}', 'ü'),
    ('U', '\u{308}', 'Ü'),
];

fn compose(c: char, mark: char) -> Option<char> {
    MARKS
        .iter()
        .find(|&&(l, m, _)| l == c && m == mark)
        .map(|&(_, _, k)| k)
}

/// What a terminal draws in no cell of its own, or draws over the one
/// before: the soft hyphen; the combining marks of Combining Diacritical
/// Marks (U+0300-036F), its Extended and Supplement blocks (U+1AB0-1AFF,
/// U+1DC0-1DFF), Combining Marks for Symbols (U+20D0-20FF), Combining Half
/// Marks (U+FE20-FE2F), Cyrillic (U+0483-0489), Hebrew points and Arabic
/// marks; zero-width and direction formatting characters, the BOM,
/// variation selectors and tags. The combining marks of other scripts
/// (Devanagari, Thai and the rest) are not covered and pass as one cell.
fn takes_no_cell(c: char) -> bool {
    matches!(c as u32,
        0x00ad | 0x0300..=0x036f | 0x0483..=0x0489 | 0x0591..=0x05bd | 0x05bf
        | 0x05c1..=0x05c2 | 0x05c4..=0x05c5 | 0x05c7 | 0x0610..=0x061a | 0x061c
        | 0x064b..=0x065f | 0x0670 | 0x06d6..=0x06dc | 0x06df..=0x06e4 | 0x06e7..=0x06e8
        | 0x06ea..=0x06ed | 0x1ab0..=0x1aff | 0x1dc0..=0x1dff | 0x200b..=0x200f
        | 0x2028..=0x202e | 0x2060..=0x206f | 0x20d0..=0x20ff | 0xfe00..=0xfe0f
        | 0xfe20..=0xfe2f | 0xfeff | 0xe0000..=0xe0fff)
}

/// A common noun with its article: `la licencia`, `las cuatro libertades`,
/// `el área (f)`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Noun {
    /// The noun without its article: `cuatro libertades`.
    pub word: String,
    /// The article, in lower case, and the noun: what fills `{cosa}`.
    pub with_article: String,
    pub fem: bool,
    pub plural: bool,
}

/// `el`, `la`, `los` or `las` and the noun; `(f)` at the end marks a
/// feminine noun that takes `el` (`el área (f)`).
pub fn noun(entry: &str) -> Result<Noun, String> {
    let mut words = entry.split_whitespace();
    let article = words.next().unwrap_or("").to_lowercase();
    let (fem, plural) = match article.as_str() {
        "el" => (false, false),
        "la" => (true, false),
        "los" => (false, true),
        "las" => (true, true),
        _ => return Err("no article: write el, la, los or las".into()),
    };
    let mut rest: Vec<&str> = words.collect();
    let marked = rest.last().is_some_and(|w| w.eq_ignore_ascii_case("(f)"));
    if marked {
        rest.pop();
        if article != "el" {
            return Err("(f) only marks a noun after el".into());
        }
    }
    if rest.is_empty() {
        return Err("an article and no noun".into());
    }
    let word = rest.join(" ");
    Ok(Noun {
        with_article: format!("{article} {word}"),
        word,
        fem: fem || marked,
        plural,
    })
}

/// The engine's form: `privativ*`, `libre`, `con copyleft`, `protegid* por copyleft`.
pub fn adjective(entry: &str) -> Result<String, String> {
    let words: Vec<&str> = entry.split_whitespace().collect();
    if words.is_empty() {
        return Err("empty".into());
    }
    // A `*` stands for the `o` or `a` that ends a word, so it follows the
    // stem: `seri*`, `continu*`, `fe*`, `europe*`, never `privativo*`.
    // (`libre*` cannot be told from `europe*`, so it passes.)
    let starred = words.iter().any(|w| w.contains('*'));
    for w in &words {
        let Some(stem) = w.strip_suffix('*') else {
            if w.contains('*') {
                return Err("a * only stands for a word's last o or a".into());
            }
            continue;
        };
        if stem.contains('*') || stem.is_empty() {
            return Err("a * only stands for a word's last o or a".into());
        }
        if stem.ends_with(['o', 'a']) {
            return Err("the * replaces the last o or a".into());
        }
    }
    if let [word] = words[..]
        && !starred
        && let Some(stem) = word.strip_suffix('o')
        && !stem.is_empty()
    {
        return Ok(format!("{stem}*"));
    }
    Ok(words.join(" "))
}

/// The families of verbs whose gerund or participle the regular rule gets
/// wrong, matched by the end of the infinitive so that compounds are
/// covered (`describir`, `deshacer`, `devolver`); the longest ending that
/// matches wins. (ending, its gerund, its participle without the final
/// `o`, whether it must be the whole word).
const IRREGULAR: &[(&str, &str, &str, bool)] = &[
    ("ir", "yendo", "id", true),
    ("ver", "viendo", "vist", true),
    ("prever", "previendo", "previst", false),
    ("entrever", "entreviendo", "entrevist", false),
    ("rever", "reviendo", "revist", false),
    // `corromper` is regular.
    ("romper", "rompiendo", "rot", true),
    ("poder", "pudiendo", "podid", true),
    ("scribir", "scribiendo", "scrit", false),
    ("poner", "poniendo", "puest", false),
    ("hacer", "haciendo", "hech", false),
    ("facer", "faciendo", "fech", false),
    ("volver", "volviendo", "vuelt", false),
    ("solver", "solviendo", "suelt", false),
    ("brir", "briendo", "biert", false),
    ("decir", "diciendo", "dich", false),
    ("bendecir", "bendiciendo", "bendecid", false),
    ("maldecir", "maldiciendo", "maldecid", false),
    ("morir", "muriendo", "muert", false),
    ("dormir", "durmiendo", "dormid", false),
    ("pudrir", "pudriendo", "podrid", false),
    ("freír", "friendo", "frit", false),
    ("eír", "iendo", "eíd", false),
    // The same written without the accent, a likely slip.
    ("freir", "friendo", "frit", false),
    ("eir", "iendo", "eíd", false),
    ("güir", "guyendo", "güid", false),
    ("eñir", "iñendo", "eñid", false),
    // Stem-changers: `e` becomes `i` in the gerund.
    ("vertir", "virtiendo", "vertid", false),
    ("pedir", "pidiendo", "pedid", false),
    ("medir", "midiendo", "medid", false),
    ("petir", "pitiendo", "petid", false),
    ("seguir", "siguiendo", "seguid", false),
    ("servir", "sirviendo", "servid", false),
    ("hervir", "hirviendo", "hervid", false),
    ("legir", "ligiendo", "legid", false),
    ("regir", "rigiendo", "regid", false),
    ("sentir", "sintiendo", "sentid", false),
    ("mentir", "mintiendo", "mentid", false),
    ("ferir", "firiendo", "ferid", false),
    ("gerir", "giriendo", "gerid", false),
    ("herir", "hiriendo", "herid", false),
    ("querir", "quiriendo", "querid", false),
    ("venir", "viniendo", "venid", false),
    ("vestir", "vistiendo", "vestid", false),
    ("gemir", "gimiendo", "gemid", false),
    ("rendir", "rindiendo", "rendid", false),
    ("cebir", "cibiendo", "cebid", false),
    ("derretir", "derritiendo", "derretid", false),
    ("erguir", "irguiendo", "erguid", false),
];

/// (infinitive, gerund, participle without its final `o`), the engine's form.
/// Three forms written out, `proveer | proveyendo | provisto`, are taken as
/// they are.
pub fn verb(entry: &str) -> Result<(String, String, String), String> {
    let entry = entry.to_lowercase();
    if entry.contains('|') {
        return written(&entry);
    }
    let mut words = entry.split_whitespace();
    let (Some(inf), None) = (words.next(), words.next()) else {
        return Err("one word: the infinitive".into());
    };
    infinitive(inf)?;
    if let Some((stem, ger, part)) = IRREGULAR
        .iter()
        .filter(|(end, _, _, whole)| !whole || inf == *end)
        .filter_map(|&(end, ger, part, _)| Some((inf.strip_suffix(end)?, ger, part)))
        .min_by_key(|(stem, ..)| stem.len())
    {
        return Ok((
            inf.to_string(),
            format!("{stem}{ger}"),
            format!("{stem}{part}"),
        ));
    }
    let (stem, ger, part) = if let Some(stem) = inf.strip_suffix("ar") {
        (stem, "ando", "ad")
    } else {
        let stem = ["er", "ir", "ír"]
            .iter()
            .find_map(|end| inf.strip_suffix(end))
            .unwrap_or("");
        let vowel = stem.ends_with(['a', 'e', 'i', 'o', 'u'])
            && !stem.ends_with("gu")
            && !stem.ends_with("qu");
        if vowel {
            // `leyendo`, `leíd`; but `distribuid`, with no accent after `u`.
            (stem, "yendo", if stem.ends_with('u') { "id" } else { "íd" })
        } else if stem.ends_with('ñ') || stem.ends_with("ll") {
            (stem, "endo", "id")
        } else {
            (stem, "iendo", "id")
        }
    };
    if stem.is_empty() {
        return Err("not an infinitive (-ar, -er, -ir)".into());
    }
    Ok((
        inf.to_string(),
        format!("{stem}{ger}"),
        format!("{stem}{part}"),
    ))
}

/// Is `w` one lower-case word ending in `-ar`, `-er`, `-ir` or `-ír`?
fn infinitive(w: &str) -> Result<(), String> {
    let letters = w
        .chars()
        .all(|c| c.is_ascii_lowercase() || "áéíóúüñ".contains(c));
    if letters
        && ["arse", "erse", "irse", "írse"]
            .iter()
            .any(|e| w.ends_with(e))
    {
        return Err("reflexive: write it without se".into());
    }
    if !letters || !["ar", "er", "ir", "ír"].iter().any(|e| w.ends_with(e)) {
        return Err("not an infinitive (-ar, -er, -ir)".into());
    }
    Ok(())
}

/// `infinitive | gerund | participle`, as the author wrote them.
fn written(entry: &str) -> Result<(String, String, String), String> {
    let parts: Vec<&str> = entry.split('|').map(str::trim).collect();
    let [inf, ger, part] = parts[..] else {
        return Err("write infinitive | gerund | participle".into());
    };
    let word = |w: &str| !w.is_empty() && w.chars().all(char::is_alphabetic);
    if !word(ger) || !word(part) {
        return Err("write infinitive | gerund | participle".into());
    }
    infinitive(inf)?;
    if ger.strip_suffix("ndo").is_none_or(str::is_empty) {
        return Err("the gerund ends in -ndo".into());
    }
    let Some(stem) = part.strip_suffix('o').filter(|s| !s.is_empty()) else {
        return Err("the participle ends in -o".into());
    };
    Ok((inf.to_string(), ger.to_string(), stem.to_string()))
}

/// Lower case, accents stripped, spaces to `_`, ASCII only. None for a word with `ñ` or with nothing left.
pub fn slug(s: &str) -> Option<String> {
    let mut out = String::new();
    let mut gap = false;
    for c in s.chars() {
        let c = match c {
            'á' | 'Á' => 'a',
            'é' | 'É' => 'e',
            'í' | 'Í' => 'i',
            'ó' | 'Ó' => 'o',
            'ú' | 'Ú' | 'ü' | 'Ü' => 'u',
            c => c,
        };
        if c.is_ascii_alphanumeric() {
            if gap && !out.is_empty() {
                out.push('_');
            }
            gap = false;
            out.push(c.to_ascii_lowercase());
        } else if c.is_alphanumeric() {
            // `ñ`, or a letter with no ASCII form: left out, not misspelled.
            return None;
        } else {
            gap = true;
        }
    }
    (!out.is_empty()).then_some(out)
}

/// "Richard Stallman" -> "stallman": the slug of the last word.
pub fn login(person: &str) -> Option<String> {
    slug(person.split_whitespace().next_back()?)
}

/// The blanks an author writes and the engine's keys for them (spec 7.4).
const BLANKS: &[(&str, char)] = &[
    ("persona", 's'),
    ("nombre", 't'),
    ("cosa", 'o'),
    ("usuario", 'u'),
    ("n", 'n'),
    ("d", 'l'),
    ("pct", 'p'),
    ("hex", 'x'),
];

/// `{persona}` -> `{s}` and the rest of the table in spec 7.4. Err for an unknown blank or a stray brace.
pub fn blanks(line: &str) -> Result<String, String> {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(i) = rest.find(['{', '}']) {
        let (before, from) = rest.split_at(i);
        out.push_str(before);
        let Some(from) = from.strip_prefix('{') else {
            return Err("a } without its {".into());
        };
        let Some((name, after)) = from.split_once('}').filter(|(n, _)| !n.contains('{')) else {
            return Err("a { without its }".into());
        };
        let Some(&(_, key)) = BLANKS.iter().find(|(n, _)| *n == name) else {
            let short: String = name.chars().take(16).collect();
            return Err(format!("unknown blank {{{short}}}"));
        };
        out.push('{');
        out.push(key);
        out.push('}');
        rest = after;
    }
    out.push_str(rest);
    Ok(out)
}

/// The longest line and the longest word in big letters (spec 7.5).
const LETTERS: usize = 12;

/// Big letters: upper case, accents stripped, `Ñ` kept. Err names a character outside
/// A-Z, Ñ, 0-9, space and `! ¡ ? ¿ : - . +`, a line over 12 characters, a word over 12.
pub fn letters(line: &str) -> Result<String, String> {
    let mut out = String::new();
    for word in line.split_whitespace() {
        if !out.is_empty() {
            out.push(' ');
        }
        for c in word.chars().flat_map(char::to_uppercase) {
            let c = match c {
                'Á' => 'A',
                'É' => 'E',
                'Í' => 'I',
                'Ó' => 'O',
                'Ú' | 'Ü' => 'U',
                c => c,
            };
            if !letrero::has_glyph(c) {
                return Err(format!("{c:?} has no big letter"));
            }
            out.push(c);
        }
    }
    if out.is_empty() {
        return Err("no letters".into());
    }
    // A count past 99 is shown as `99+`, so the warning keeps to one line.
    let count = |n: usize| {
        if n > 99 {
            "99+".to_string()
        } else {
            n.to_string()
        }
    };
    let longest = out.split(' ').map(|w| w.chars().count()).max().unwrap_or(0);
    if longest > LETTERS {
        return Err(format!(
            "word of {} letters, {LETTERS} at most",
            count(longest)
        ));
    }
    let n = out.chars().count();
    if n > LETTERS {
        return Err(format!("{} characters, {LETTERS} at most", count(n)));
    }
    Ok(out)
}

/// "TOP / BOTTOM" or "TOP / BOTTOM / caption": split on " / ", at most three parts.
/// The two lines come back in big letters (`letters`), the caption as written.
pub fn banner(line: &str) -> Result<(String, String, Option<String>), String> {
    let mut parts = line.splitn(3, " / ").map(str::trim);
    let (top, bottom) = match (parts.next(), parts.next()) {
        (Some(top), Some(bottom)) if !top.is_empty() && !bottom.is_empty() => (top, bottom),
        _ => return Err("two lines, split by \" / \"".into()),
    };
    let caption = parts.next().filter(|c| !c.is_empty()).map(String::from);
    let top = letters(top).map_err(|e| format!("top: {e}"))?;
    let bottom = letters(bottom).map_err(|e| format!("bottom: {e}"))?;
    Ok((top, bottom, caption))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(x: &str) -> String {
        x.to_string()
    }

    #[test]
    fn nouns_carry_their_article() {
        for (entry, word, fem, plural, with_article) in [
            ("el kernel", "kernel", false, false, "el kernel"),
            ("la licencia", "licencia", true, false, "la licencia"),
            ("los parches", "parches", false, true, "los parches"),
            (
                "las cuatro libertades",
                "cuatro libertades",
                true,
                true,
                "las cuatro libertades",
            ),
            ("el área (f)", "área", true, false, "el área"),
            ("La GPL", "GPL", true, false, "la GPL"),
        ] {
            let want = Noun {
                word: s(word),
                with_article: s(with_article),
                fem,
                plural,
            };
            assert_eq!(noun(entry), Ok(want), "{entry}");
        }
        for entry in ["kernel", "un kernel", "el", "", "la (f)", "los parches (f)"] {
            assert!(noun(entry).is_err(), "{entry}");
        }
    }

    #[test]
    fn adjectives_by_their_ending() {
        for (entry, want) in [
            ("privativo", "privativ*"),
            ("libre", "libre"),
            ("con copyleft", "con copyleft"),
            ("sin puertas traseras", "sin puertas traseras"),
            ("protegid* por copyleft", "protegid* por copyleft"),
            ("español", "español"),
            ("auditable", "auditable"),
            ("seri*", "seri*"),
            ("continu*", "continu*"),
            ("fe*", "fe*"),
            ("europe*", "europe*"),
        ] {
            assert_eq!(adjective(entry), Ok(s(want)), "{entry}");
        }
        for entry in [
            "",
            "*",
            "pro*tegido",
            "protegid** por copyleft",
            "* libre",
            "privativo*",
            "rapida*",
            "protegido* por copyleft",
        ] {
            assert!(adjective(entry).is_err(), "{entry}");
        }
    }

    #[test]
    fn verbs_from_the_infinitive() {
        for (inf, ger, part) in [
            ("liberar", "liberando", "liberado"),
            ("compartir", "compartiendo", "compartido"),
            ("distribuir", "distribuyendo", "distribuido"),
            ("leer", "leyendo", "leído"),
            ("extraer", "extrayendo", "extraído"),
            ("distinguir", "distinguiendo", "distinguido"),
            ("escribir", "escribiendo", "escrito"),
            ("describir", "describiendo", "descrito"),
            ("abrir", "abriendo", "abierto"),
            ("hacer", "haciendo", "hecho"),
            ("deshacer", "deshaciendo", "deshecho"),
            ("componer", "componiendo", "compuesto"),
            ("resolver", "resolviendo", "resuelto"),
            ("devolver", "devolviendo", "devuelto"),
            ("ver", "viendo", "visto"),
            ("romper", "rompiendo", "roto"),
            ("decir", "diciendo", "dicho"),
            ("convertir", "convirtiendo", "convertido"),
            ("pedir", "pidiendo", "pedido"),
            ("seguir", "siguiendo", "seguido"),
            ("elegir", "eligiendo", "elegido"),
            ("corregir", "corrigiendo", "corregido"),
            ("servir", "sirviendo", "servido"),
            ("dormir", "durmiendo", "dormido"),
            // Beyond the floor of spec 11.2.
            ("cubrir", "cubriendo", "cubierto"),
            ("morir", "muriendo", "muerto"),
            ("medir", "midiendo", "medido"),
            ("repetir", "repitiendo", "repetido"),
            ("prever", "previendo", "previsto"),
            ("mover", "moviendo", "movido"),
            ("corromper", "corrompiendo", "corrompido"),
            ("transferir", "transfiriendo", "transferido"),
            ("prevenir", "previniendo", "prevenido"),
            ("oír", "oyendo", "oído"),
            ("reír", "riendo", "reído"),
            ("construir", "construyendo", "construido"),
            ("delinquir", "delinquiendo", "delinquido"),
            ("gruñir", "gruñendo", "gruñido"),
            ("teñir", "tiñendo", "teñido"),
            ("satisfacer", "satisfaciendo", "satisfecho"),
            ("bendecir", "bendiciendo", "bendecido"),
            ("ir", "yendo", "ido"),
            ("reir", "riendo", "reído"),
            ("freir", "friendo", "frito"),
            ("sonreir", "sonriendo", "sonreído"),
            ("freír", "friendo", "frito"),
            ("sonreír", "sonriendo", "sonreído"),
            ("rever", "reviendo", "revisto"),
            ("entrever", "entreviendo", "entrevisto"),
            ("Compilar", "compilando", "compilado"),
        ] {
            let got = verb(inf).map(|(i, g, p)| (i, g, format!("{p}o")));
            assert_eq!(got, Ok((inf.to_lowercase(), s(ger), s(part))), "{inf}");
        }
        assert_eq!(
            verb("proveer | proveyendo | provisto"),
            Ok((s("proveer"), s("proveyendo"), s("provist")))
        );
        for entry in [
            "liberarse",
            "libre",
            "ar",
            "er",
            "",
            "poner en marcha",
            "proveer | proveyendo",
            "proveer | proveyendo | provisto | x",
            "proveer | proveer | provisto",
            "proveer | proveyendo | provista",
            "proveer | ndo | provisto",
            "proveer | proveyendo | o",
            "libre | liberando | liberado",
            "c0mpilar",
        ] {
            assert!(verb(entry).is_err(), "{entry}");
        }
    }

    #[test]
    fn slugs_are_ascii() {
        for (word, want) in [
            ("Código fuente", Some("codigo_fuente")),
            ("GNU", Some("gnu")),
            ("Free Software Foundation", Some("free_software_foundation")),
            ("GNU/Linux", Some("gnu_linux")),
            ("pingüino", Some("pinguino")),
            ("diseño", None),
            ("¡!", None),
            ("", None),
        ] {
            assert_eq!(slug(word).as_deref(), want, "{word}");
        }
        assert_eq!(login("Richard Stallman").as_deref(), Some("stallman"));
        assert_eq!(login("Linus Torvalds").as_deref(), Some("torvalds"));
        assert_eq!(login("Ñu"), None);
        assert_eq!(login(""), None);
    }

    #[test]
    fn blanks_become_the_engines() {
        assert_eq!(
            blanks("{persona} revisa {cosa} de {nombre}: {n} {d} {pct} {hex} {usuario}"),
            Ok(s("{s} revisa {o} de {t}: {n} {l} {p} {x} {u}"))
        );
        assert_eq!(blanks("sin blancos"), Ok(s("sin blancos")));
        for line in ["{cosa", "}", "{nada}", "{}", "{{cosa}", "{s}", "a } b {n}"] {
            assert!(blanks(line).is_err(), "{line}");
        }
        assert!(blanks("{nada}").unwrap_err().contains("{nada}"));
    }

    #[test]
    fn letters_for_the_banner() {
        for (line, want) in [
            ("Código", "CODIGO"),
            ("año", "AÑO"),
            ("¿Qué es", "¿QUE ES"),
            ("C++", "C++"),
            ("WEB 3.0", "WEB 3.0"),
            ("GPL-3", "GPL-3"),
            ("¡Atención!", "¡ATENCION!"),
            ("pingüino", "PINGUINO"),
            ("ABCDEFGHIJKL", "ABCDEFGHIJKL"),
            ("NO ES UNIX:", "NO ES UNIX:"),
        ] {
            assert_eq!(letters(line), Ok(s(want)), "{line}");
        }
        let slash = letters("GNU/LINUX").unwrap_err();
        assert!(slash.contains('/'), "{slash}");
        for line in ["ABCDEFGHIJKLM", "CONSTITUCIONAL X", "", "   ", "garçon"] {
            assert!(letters(line).is_err(), "{line}");
        }
        assert!(letters("CONSTITUCIONAL X").unwrap_err().contains("14"));
    }

    #[test]
    fn banner_lines_split_on_spaced_slashes() {
        assert_eq!(
            banner("SOFTWARE / LIBRE"),
            Ok((s("SOFTWARE"), s("LIBRE"), None))
        );
        assert_eq!(
            banner("GNU / NO ES UNIX / desde 1983"),
            Ok((s("GNU"), s("NO ES UNIX"), Some(s("desde 1983"))))
        );
        assert_eq!(
            banner("A / B / GNU/Linux / x"),
            Ok((s("A"), s("B"), Some(s("GNU/Linux / x"))))
        );
        assert_eq!(
            banner("código / cerrado"),
            Ok((s("CODIGO"), s("CERRADO"), None))
        );
        for line in [
            "SOLO",
            " / LIBRE",
            "LIBRE / ",
            "SOFTWARE/LIBRE",
            "GNU/LINUX / X",
        ] {
            assert!(banner(line).is_err(), "{line}");
        }
    }

    #[test]
    fn tidy_composes_and_refuses() {
        for (raw, want) in [
            ("co\u{301}digo", "código"),
            ("n\u{303}", "ñ"),
            ("ping\u{fc}ino", "pingüino"),
            ("pingu\u{308}ino", "pingüino"),
            ("A\u{301}REA", "ÁREA"),
            ("  a   b ", "a b"),
            ("\tlicencia\r", "licencia"),
            ("«libre»", "«libre»"),
            ("", ""),
        ] {
            assert_eq!(tidy(raw), Ok(s(want)), "{raw:?}");
        }
        for raw in [
            "a\u{0}b",
            "a\tb",
            "界",
            "\u{1f600}",
            "\u{301}",
            "x\u{301}",
            "a\u{200b}b",
            "a\u{fe0f}",
        ] {
            assert!(tidy(raw).is_err(), "{raw:?}");
        }
    }

    #[test]
    fn nothing_panics() {
        let long = "x".repeat(10_000);
        let long_verb = format!("{}ir", "e".repeat(10_000));
        let hostile = [
            "",
            "a",
            "ñ",
            long.as_str(),
            long_verb.as_str(),
            "\u{1f600}",
            "\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f467}",
            "שלום עולם",
            "مرحبا بالعالم",
            "{{{{",
            "}}}}",
            "{",
            "*",
            "|",
            "| | |",
            " / ",
            " /  / ",
            "\u{301}\u{301}",
            "\u{0}",
        ];
        for x in hostile {
            let _ = tidy(x);
            let _ = noun(x);
            let _ = adjective(x);
            let _ = verb(x);
            let _ = slug(x);
            let _ = login(x);
            let _ = blanks(x);
            let _ = letters(x);
            let _ = banner(x);
        }
    }

    #[test]
    fn warnings_fit_the_hud_line() {
        // The loader puts `cosas[399]: ` before them; the HUD line is 60.
        let long = "x".repeat(10_000);
        let wide = "界".repeat(100);
        let errors: [Result<(), String>; 15] = [
            noun("").map(drop),
            noun(&long).map(drop),
            noun("el").map(drop),
            noun("la licencia (f)").map(drop),
            adjective("*").map(drop),
            adjective("").map(drop),
            adjective("privativo*").map(drop),
            verb(&long).map(drop),
            verb("liberarse").map(drop),
            verb("a | b").map(drop),
            verb("proveer | proveyendo | provista").map(drop),
            verb("poner en marcha").map(drop),
            blanks(&format!("{{{long}}}")).map(drop),
            blanks("{").map(drop),
            blanks("}").map(drop),
        ];
        for e in errors {
            let e = e.unwrap_err();
            assert!(e.chars().count() <= 44, "{e}");
        }
        // Banner lines go under `letrero_verde[400]: `, and they are tidied.
        let banners: [Result<(), String>; 12] = [
            tidy("\u{0}").map(drop),
            tidy("\u{e0001}").map(drop),
            tidy(&wide).map(drop),
            tidy("\u{301}").map(drop),
            letters(&long).map(drop),
            letters(&format!("{long} {long}")).map(drop),
            letters("/").map(drop),
            letters("").map(drop),
            banner(&long).map(drop),
            banner(&format!("{long} / {long}")).map(drop),
            banner(&format!("A / {long} {long}")).map(drop),
            banner("A / \u{10ffff}").map(drop),
        ];
        for e in banners {
            let e = e.unwrap_err();
            assert!(e.chars().count() <= 40, "{e}");
        }
    }
}
