//! Scrolling-text widgets: hex dump, log tail, code, errno and kernel panics,
//! packet sniffer, directory tree, sequence aligner, PEM keys, mission log,
//! passwords and login attempts. They share one engine: lines are born at a
//! steady rate (when a line is born its text and colors are decided once),
//! kept in a ring and drawn bottom-aligned, so drawing never allocates.
//!
//! The widget ideas (hexdump, tail of logs, errno, tree, code, ssh-keygen
//! art...) are those of Dustin Kirkland's hollywood (Apache License 2.0).

use super::lexico::{self, Vocab};
use super::{Env, Rect, Tone, Widget, babble, muertos, put};
use crate::fiesta::{Fiesta, Kind};
use crate::render::{BOLD, Canvas};
use fastrand::Rng;
use std::collections::VecDeque;
use std::fmt::Write;

/// One screen line: colored runs of text.
pub type Line = Vec<(Tone, String)>;

/// Appends text to a line, merging with the previous run of the same tone.
fn run(line: &mut Line, tone: Tone, s: &str) {
    match line.last_mut() {
        Some((t, buf)) if *t == tone => buf.push_str(s),
        _ => line.push((tone, s.to_string())),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeedKind {
    Hex,
    Log,
    Code,
    Errno,
    Sniffer,
    Tree,
    Dna,
    Pem,
    Mission,
    Passwords,
    Auth,
}

/// Real errno values: (name, number, message).
const ERRNOS: &[(&str, u8, &str)] = &[
    ("EPERM", 1, "Operation not permitted"),
    ("ENOENT", 2, "No such file or directory"),
    ("ESRCH", 3, "No such process"),
    ("EINTR", 4, "Interrupted system call"),
    ("EIO", 5, "Input/output error"),
    ("ENXIO", 6, "No such device or address"),
    ("E2BIG", 7, "Argument list too long"),
    ("ENOEXEC", 8, "Exec format error"),
    ("EBADF", 9, "Bad file descriptor"),
    ("ECHILD", 10, "No child processes"),
    ("EAGAIN", 11, "Resource temporarily unavailable"),
    ("ENOMEM", 12, "Cannot allocate memory"),
    ("EACCES", 13, "Permission denied"),
    ("EFAULT", 14, "Bad address"),
    ("EBUSY", 16, "Device or resource busy"),
    ("EEXIST", 17, "File exists"),
    ("ENODEV", 19, "No such device"),
    ("ENOTDIR", 20, "Not a directory"),
    ("EINVAL", 22, "Invalid argument"),
    ("EMFILE", 24, "Too many open files"),
    ("ENOSPC", 28, "No space left on device"),
    ("EPIPE", 32, "Broken pipe"),
    ("EDOM", 33, "Numerical argument out of domain"),
    ("ERANGE", 34, "Numerical result out of range"),
    ("EDEADLK", 35, "Resource deadlock avoided"),
    ("ENOSYS", 38, "Function not implemented"),
    ("ELOOP", 40, "Too many levels of symbolic links"),
    ("EPROTO", 71, "Protocol error"),
    ("EOVERFLOW", 75, "Value too large for defined data type"),
    ("ENOTSOCK", 88, "Socket operation on non-socket"),
    ("EADDRINUSE", 98, "Address already in use"),
    ("ENETUNREACH", 101, "Network is unreachable"),
    ("ECONNRESET", 104, "Connection reset by peer"),
    ("ETIMEDOUT", 110, "Connection timed out"),
    ("ECONNREFUSED", 111, "Connection refused"),
    ("EHOSTUNREACH", 113, "No route to host"),
    ("ESTALE", 116, "Stale file handle"),
    ("ECANCELED", 125, "Operation canceled"),
    ("EKEYREVOKED", 128, "Key has been revoked"),
    ("EHWPOISON", 133, "Memory page has hardware error"),
];

const EXTS: &[&str] = &[
    "c", "h", "rs", "so", "key", "pem", "db", "log", "bin", "sh", "cfg", "tgz", "dat", "py",
    "conf", "pcap", "enc", "shadow",
];

const USERS: &[&str] = &[
    "root",
    "admin",
    "guest",
    "oracle",
    "neo",
    "trinity",
    "morpheus",
    "operator",
    "sysop",
    "hal9000",
    "wopr",
    "joshua",
    "mainframe",
    "ghost",
    "service",
    "deploy",
];

const C_KEYWORDS: &[&str] = &[
    "if", "else", "for", "while", "return", "static", "const", "struct", "int", "void", "unsigned",
    "char", "sizeof", "goto", "break", "continue", "switch", "case", "do",
];
const C_TYPES: &[&str] = &[
    "uint8_t", "uint16_t", "uint32_t", "uint64_t", "size_t", "ssize_t", "bool", "u32", "u64",
    "NULL",
];
const RUST_KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "if", "else", "for", "in", "while", "return", "match", "pub", "struct",
    "impl", "use", "loop", "self", "Self", "as", "unsafe", "const", "static",
];
const RUST_TYPES: &[&str] = &[
    "u8", "u16", "u32", "u64", "usize", "i32", "i64", "bool", "Vec", "String", "Result", "Option",
    "Ok", "Err", "Some", "None",
];
/// What a holiday program calls its variables.
const LOCALS_MX: &[&str] = &[
    "pan", "copal", "vela", "alma", "flor", "sal", "petalo", "ofrenda", "nivel", "xolo", "llama",
    "altar", "foto", "nombre",
];
/// Errors a holiday program can fail with.
const ERRS_MX: &[&str] = &[
    "SinPan",
    "NoEsTuHora",
    "ElXoloSeFue",
    "VeladoraApagada",
    "FaltaLaSal",
    "AnimaPerdida",
];
const REGS: &[&str] = &[
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", "eax", "ebx", "ecx", "edx", "r8", "r9",
    "r10", "r11", "r12", "r13", "r14", "r15", "xmm0", "xmm1", "al", "cl",
];
const LOCALS: &[&str] = &[
    "acc", "buf", "len", "idx", "key", "sum", "tmp", "ptr", "seed", "mask", "state", "nonce",
    "salt", "rounds", "flux",
];

/// A sentence: on Día de Muertos the Mictlán's among the ordinary ones.
/// Returns whether it is Spanish.
fn say(rng: &mut Rng, level: u8, mx: Fiesta, out: &mut String) -> bool {
    if mx.is(Kind::Muertos) {
        muertos::mixed(rng, level, mx.level, out)
    } else if let Some(v) = lexico::of(mx) {
        v.mixed(rng, level, mx.level, out)
    } else {
        babble::sentence(rng, level, out);
        false
    }
}

/// An UPPER-CASE progress label, the Mictlán's or an ordinary one.
fn label(rng: &mut Rng, level: u8, mx: Fiesta, out: &mut String) {
    if mx.is(Kind::Muertos) && rng.f32() < muertos::share(mx.level) {
        muertos::progress_label(rng, level, out);
    } else if let Some(v) = lexico::of(mx)
        && rng.f32() < lexico::share(mx.level)
    {
        v.progress_label(rng, level, out);
    } else {
        babble::progress_label(rng, level, out);
    }
}

/// A title, the Mictlán's or an ordinary one.
fn title(rng: &mut Rng, mx: Fiesta, out: &mut String) {
    if mx.is(Kind::Muertos) {
        muertos::mixed_title(rng, mx.level, out);
    } else if let Some(v) = lexico::of(mx) {
        v.mixed_title(rng, mx.level, out);
    } else {
        babble::title(rng, out);
    }
}

/// A daemon name, a holiday one half the time on Día de Muertos.
fn daemon(rng: &mut Rng, mx: Fiesta) -> &'static str {
    if mx.is(Kind::Muertos) && rng.f32() < 0.5 * muertos::share(mx.level) {
        babble::pick(rng, muertos::DAEMONS)
    } else if let Some(v) = lexico::of(mx)
        && rng.f32() < 0.5 * lexico::share(mx.level)
    {
        babble::pick(rng, v.daemons)
    } else {
        babble::pick(rng, babble::DAEMONS)
    }
}

/// A local variable name.
fn local(rng: &mut Rng, mx: Fiesta) -> &'static str {
    if mx.is(Kind::Muertos) && rng.f32() < muertos::share(mx.level) {
        babble::pick(rng, LOCALS_MX)
    } else if let Some(v) = lexico::of(mx)
        && rng.f32() < lexico::share(mx.level)
    {
        babble::pick(rng, v.locals)
    } else {
        babble::pick(rng, LOCALS)
    }
}

/// Writes a snake_case identifier from the babble tables: "flux_capacitor".
fn ident(rng: &mut Rng, sep: char, mx: Fiesta, out: &mut String) {
    if mx.is(Kind::Muertos) && rng.f32() < muertos::share(mx.level) {
        muertos::ident(rng, sep, out);
        return;
    }
    if let Some(v) = lexico::of(mx)
        && rng.f32() < lexico::share(mx.level)
    {
        v.ident(rng, sep, out);
        return;
    }
    let word = |list: &[&str], rng: &mut Rng, out: &mut String| {
        for ch in babble::pick(rng, list).chars() {
            out.push(if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                sep
            });
        }
    };
    word(babble::MODS, rng, out);
    out.push(sep);
    word(babble::NOUNS, rng, out);
}

fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Syntax-colors one line of C (0), Rust (1) or x86 assembly (2).
fn highlight(src: &str, lang: u8, out: &mut Line) {
    let (kw, ty) = if lang == 0 {
        (C_KEYWORDS, C_TYPES)
    } else {
        (RUST_KEYWORDS, RUST_TYPES)
    };
    let b: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut first_word = lang == 2;
    let mut buf = String::new();
    while i < b.len() {
        let c = b[i];
        let comment = (c == '/' && b.get(i + 1) == Some(&'/'))
            || (c == '/' && b.get(i + 1) == Some(&'*'))
            || (lang == 2 && c == ';');
        if comment {
            buf.clear();
            buf.extend(&b[i..]);
            run(out, Tone::Dim, &buf);
            return;
        }
        buf.clear();
        if c == '"' {
            buf.push(c);
            i += 1;
            while i < b.len() && b[i] != '"' {
                buf.push(b[i]);
                i += 1;
            }
            if i < b.len() {
                buf.push('"');
                i += 1;
            }
            run(out, Tone::Amber, &buf);
        } else if c.is_ascii_digit() {
            while i < b.len() && is_word(b[i]) {
                buf.push(b[i]);
                i += 1;
            }
            run(out, Tone::Amber, &buf);
            first_word = false;
        } else if c.is_ascii_alphabetic() || c == '_' {
            while i < b.len() && is_word(b[i]) {
                buf.push(b[i]);
                i += 1;
            }
            let tone = if lang == 2 {
                if first_word && b.get(i) == Some(&':') {
                    Tone::Bright
                } else if first_word {
                    Tone::Cyan
                } else if REGS.contains(&buf.as_str()) {
                    Tone::Amber
                } else {
                    Tone::Main
                }
            } else if kw.contains(&buf.as_str()) {
                Tone::Cyan
            } else if ty.contains(&buf.as_str()) {
                Tone::Green
            } else if b.get(i) == Some(&'(') || b.get(i) == Some(&'!') {
                Tone::Bright
            } else {
                Tone::Main
            };
            run(out, tone, &buf);
            first_word = false;
        } else {
            buf.push(c);
            i += 1;
            if !c.is_whitespace() {
                first_word = false;
            }
            run(out, Tone::Main, &buf);
        }
    }
}

pub struct Feed {
    kind: FeedKind,
    rng: Rng,
    level: u8,
    /// The day's holiday, or `Fiesta::NONE`.
    mx: Fiesta,
    /// Navidad: the villancico that is scrolling by and the verse it is at.
    song: usize,
    verse: usize,
    w: usize,
    h: usize,
    lines: VecDeque<Line>,
    pending: VecDeque<Line>,
    acc: f32,
    rate: f32,
    /// Bytes per hex row.
    bpr: usize,
    addr: u32,
    blob: VecDeque<u8>,
    /// Simulated clock in seconds.
    clock: f32,
    seq: u32,
    depth: usize,
    n: u32,
    scratch: String,
}

impl Feed {
    pub fn new(kind: FeedKind, seed: u64, level: u8, mx: Fiesta) -> Feed {
        let mut rng = Rng::with_seed(seed);
        let rate = match kind {
            FeedKind::Hex => 7.0,
            FeedKind::Log => 4.0,
            FeedKind::Code => 6.0,
            FeedKind::Errno => 4.0,
            FeedKind::Sniffer => 9.0,
            FeedKind::Tree => 7.0,
            FeedKind::Dna => 5.0,
            FeedKind::Pem => 11.0,
            FeedKind::Mission => 1.6,
            FeedKind::Passwords => 5.0,
            FeedKind::Auth => 3.0,
        };
        Feed {
            kind,
            level,
            mx,
            // (Not drawn from the generator: ordinary days draw the same numbers.)
            song: (seed % 9) as usize,
            verse: 0,
            w: 0,
            h: 0,
            lines: VecDeque::new(),
            pending: VecDeque::new(),
            acc: 0.0,
            rate,
            bpr: 16,
            addr: rng.u32(..) & !0xf,
            blob: VecDeque::new(),
            clock: rng.f32() * 80_000.0,
            seq: rng.u32(1..500),
            depth: 0,
            n: 0,
            scratch: String::new(),
            rng,
        }
    }

    /// The vocabulary of the day's holiday, if it has one besides Muertos.
    fn voc(&self) -> Option<&'static Vocab> {
        lexico::of(self.mx)
    }

    /// The next line of the villancico that is scrolling by, and on to the
    /// next song when it ends.
    fn next_verse(&mut self) -> &'static str {
        let Some(v) = self.voc() else {
            return "";
        };
        let songs = v.songs;
        if songs.is_empty() {
            return "";
        }
        let (_, lines) = songs[self.song % songs.len()];
        let line = lines[self.verse.min(lines.len() - 1)];
        self.verse += 1;
        if self.verse >= lines.len() {
            self.verse = 0;
            self.song = (self.song + 1 + self.rng.usize(..3)) % songs.len();
        }
        line
    }

    /// Rows taken by a pinned header.
    fn header(&self) -> Option<&'static str> {
        if let Some(v) = self.voc() {
            match self.kind {
                FeedKind::Sniffer => return Some(v.sniff),
                FeedKind::Dna => return Some(v.dna_head),
                _ => {}
            }
        }
        match self.kind {
            FeedKind::Sniffer if self.mx.is(Kind::Muertos) => Some(
                "  No.  Time      Source           Destination      Proto  Len Info (mictlan0)",
            ),
            FeedKind::Sniffer => {
                Some("  No.  Time      Source           Destination      Proto  Len Info")
            }
            FeedKind::Dna if self.mx.is(Kind::Muertos) => {
                Some("BLASTN 2.14.1+  parentesco: ANCESTROS  gap open/extend: 11/1")
            }
            FeedKind::Dna => Some("BLASTN 2.14.1+  matrix: BLOSUM62  gap open/extend: 11/1"),
            _ => None,
        }
    }

    /// Pushes `text` word-wrapped to the pane width after `prefix`.
    fn wrapped(&mut self, prefix: Line, text: &str, tone: Tone) {
        let indent: usize = prefix.iter().map(|(_, s)| s.chars().count()).sum();
        let room = self.w.saturating_sub(indent).max(8);
        let mut cur = prefix;
        let mut len = 0;
        for word in text.split(' ') {
            let wl = word.chars().count();
            if len > 0 && len + 1 + wl > room {
                self.pending.push_back(std::mem::take(&mut cur));
                cur.push((Tone::Dim, " ".repeat(indent)));
                len = 0;
            }
            if len > 0 {
                run(&mut cur, tone, " ");
                len += 1;
            }
            run(&mut cur, tone, word);
            len += wl;
        }
        self.pending.push_back(cur);
    }

    fn hms(&self) -> (u32, u32, u32) {
        let s = self.clock as u32 % 86_400;
        (s / 3600, s / 60 % 60, s % 60)
    }

    fn birth(&mut self) {
        match self.kind {
            FeedKind::Hex => self.birth_hex(),
            FeedKind::Log => self.birth_log(),
            FeedKind::Code => self.birth_code(),
            FeedKind::Errno => self.birth_errno(),
            FeedKind::Sniffer => self.birth_sniffer(),
            FeedKind::Tree => self.birth_tree(),
            FeedKind::Dna => self.birth_dna(),
            FeedKind::Pem => self.birth_pem(),
            FeedKind::Mission => self.birth_mission(),
            FeedKind::Passwords => self.birth_password(),
            FeedKind::Auth => self.birth_auth(),
        }
    }

    fn birth_hex(&mut self) {
        let mut line = Line::new();
        let _ = write!(self.scratch, "{:08x}  ", self.addr);
        run(&mut line, Tone::Dim, &self.scratch);
        self.scratch.clear();
        let mut row = [0u8; 16];
        for b in row.iter_mut().take(self.bpr) {
            if self.blob.is_empty() && self.rng.u8(..100) < 4 {
                self.scratch.clear();
                say(&mut self.rng, self.level, self.mx, &mut self.scratch);
                self.blob.extend(self.scratch.bytes());
                self.blob.push_back(0);
                self.scratch.clear();
            }
            *b = match self.blob.pop_front() {
                Some(v) => v,
                None => match self.rng.u8(..10) {
                    0..=3 => 0,
                    4 => 0xff,
                    _ => self.rng.u8(..),
                },
            };
        }
        for (i, &b) in row.iter().take(self.bpr).enumerate() {
            let tone = if b == 0 {
                Tone::Dim
            } else if (0x20..0x7f).contains(&b) {
                Tone::Bright
            } else {
                Tone::Main
            };
            self.scratch.clear();
            let _ = write!(self.scratch, "{b:02x} ");
            if i == 7 {
                self.scratch.push(' ');
            }
            run(&mut line, tone, &self.scratch);
        }
        let mut asc = String::from("|");
        for &b in row.iter().take(self.bpr) {
            asc.push(if (0x20..0x7f).contains(&b) {
                b as char
            } else {
                '.'
            });
        }
        asc.push('|');
        run(&mut line, Tone::Cyan, &asc);
        self.scratch.clear();
        self.addr = self.addr.wrapping_add(self.bpr as u32);
        self.pending.push_back(line);
    }

    fn birth_log(&mut self) {
        self.clock += 0.05 + self.rng.f32() * 1.8;
        let (h, m, s) = self.hms();
        let mut prefix = Line::new();
        let pid = self.rng.u32(100..32000);
        let _ = write!(self.scratch, "{h:02}:{m:02}:{s:02} ");
        run(&mut prefix, Tone::Dim, &self.scratch);
        self.scratch.clear();
        // Narrow panes drop the host and daemon so the message has room.
        if self.w >= 64 {
            if self.mx.is(Kind::Muertos) && self.rng.bool() {
                let host = babble::pick(&mut self.rng, muertos::HOSTS);
                let _ = write!(self.scratch, "{host}-{:02} ", self.rng.u8(1..40));
            } else if let Some(v) = self.voc()
                && self.rng.bool()
            {
                let host = babble::pick(&mut self.rng, v.hosts);
                let _ = write!(self.scratch, "{host}-{:02} ", self.rng.u8(1..40));
            } else {
                let _ = write!(self.scratch, "node-{:02} ", self.rng.u8(1..40));
            }
            run(&mut prefix, Tone::Cyan, &self.scratch);
            self.scratch.clear();
            let _ = write!(self.scratch, "{}[{pid}]: ", daemon(&mut self.rng, self.mx));
            run(&mut prefix, Tone::Main, &self.scratch);
            self.scratch.clear();
        }
        // Navidad: now and then the message is a line of a villancico (the
        // Fiestas Patrias, of the Himno), in order, so the songs can be read
        // as they go by.
        let odds = self.voc().map_or(0.0, |v| {
            if v.songs.is_empty() {
                0.0
            } else {
                v.verse_odds
            }
        });
        let verse = odds > 0.0 && self.rng.f32() < odds * (1.0 + 1.2 * self.mx.level);
        let (lv, tone) = match self.rng.u8(..100) {
            _ if verse => ("NOTE ", Tone::Cyan),
            0..=54 => ("INFO ", Tone::Green),
            55..=69 => ("DEBUG", Tone::Dim),
            70..=79 => ("NOTE ", Tone::Cyan),
            80..=91 => ("WARN ", Tone::Amber),
            92..=97 => ("ERROR", Tone::Red),
            _ => ("CRIT ", Tone::Red),
        };
        run(&mut prefix, tone, lv);
        run(&mut prefix, Tone::Main, " ");
        let es = if verse {
            self.scratch.push_str("\u{266a} ");
            let line = self.next_verse();
            self.scratch.push_str(line);
            true
        } else {
            say(&mut self.rng, self.level, self.mx, &mut self.scratch)
        };
        if !verse && self.rng.u8(..4) == 0 {
            self.scratch.push_str(if es { " desde " } else { " from " });
            babble::ip(&mut self.rng, &mut self.scratch);
        }
        let text = std::mem::take(&mut self.scratch);
        let body = if verse {
            Tone::Bright
        } else if tone == Tone::Red {
            Tone::Red
        } else if tone == Tone::Amber {
            Tone::Amber
        } else {
            Tone::Main
        };
        self.wrapped(prefix, &text, body);
        self.scratch = text;
        self.scratch.clear();
    }

    fn birth_code(&mut self) {
        let lang = self.rng.u8(..3);
        let mut src: Vec<String> = Vec::new();
        let (mut name, mut ty, mut call, mut call2, mut f) = (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        );
        ident(&mut self.rng, '_', self.mx, &mut name);
        ident(&mut self.rng, '_', self.mx, &mut ty);
        ident(&mut self.rng, '_', self.mx, &mut call);
        ident(&mut self.rng, '_', self.mx, &mut call2);
        f.push_str(local(&mut self.rng, self.mx));
        let v = local(&mut self.rng, self.mx);
        let n = self.rng.u32(2..4096);
        let k = self.rng.u32(1..31);
        let hex = self.rng.u32(..);
        let mut cm = String::new();
        say(&mut self.rng, self.level.max(1), self.mx, &mut cm);
        let stmts = self.rng.usize(3..7);
        match lang {
            0 => {
                src.push(format!("static int {name}(struct {ty} *ctx, uint32_t a)"));
                src.push("{".into());
                src.push(format!("    uint32_t {v} = ctx->{f} ^ 0x{hex:08x};"));
                for _ in 0..stmts {
                    match self.rng.u8(..6) {
                        0 => src.push(format!(
                            "    if (!ctx || a > {n}) return -E{};",
                            ERRNOS[self.rng.usize(..ERRNOS.len())]
                                .0
                                .trim_start_matches('E')
                        )),
                        1 => {
                            src.push(format!("    for (int i = 0; i < {n}; i++) {{"));
                            src.push(format!("        {v} = rotl32({v}, {k}) + {call}(ctx, i);"));
                            src.push("    }".into());
                        }
                        2 => src.push(format!("    /* {cm} */")),
                        3 => src.push(format!("    memcpy(ctx->{f}, {call2}(ctx), {n});")),
                        4 => src.push(format!(
                            "    while (ctx->{f} & 0x{:x}) ctx->{f} >>= {k};",
                            hex & 0xfff
                        )),
                        _ => src.push(format!(
                            "    {call}(ctx, \"{}\");",
                            cm.chars().take(24).collect::<String>().to_lowercase()
                        )),
                    }
                }
                src.push(format!("    return {v};"));
                src.push("}".into());
            }
            1 => {
                src.push(format!(
                    "pub fn {name}(ctx: &mut {}, a: u32) -> Result<u32, Error> {{",
                    camel(&ty)
                ));
                src.push(format!("    let mut {v}: u32 = ctx.{f} ^ 0x{hex:08x};"));
                for _ in 0..stmts {
                    match self.rng.u8(..6) {
                        0 => {
                            let err = if self.mx.is(Kind::Muertos) && self.rng.bool() {
                                babble::pick(&mut self.rng, ERRS_MX)
                            } else if let Some(v) = self.voc()
                                && self.rng.bool()
                            {
                                babble::pick(&mut self.rng, v.errs)
                            } else {
                                "Overflow"
                            };
                            src.push(format!("    if a > {n} {{ return Err(Error::{err}); }}"))
                        }
                        1 => {
                            src.push(format!("    for i in 0..{n} {{"));
                            src.push(format!(
                                "        {v} = {v}.rotate_left({k}) ^ {call}(ctx, i)?;"
                            ));
                            src.push("    }".into());
                        }
                        2 => src.push(format!("    // {cm}")),
                        3 => src.push(format!(
                            "    ctx.{f}.extend_from_slice(&{call2}(ctx)?[..{}]);",
                            n.min(64)
                        )),
                        4 => src.push(format!("    let {v} = {v}.wrapping_mul(0x{:x});", hex | 1)),
                        _ => src.push(format!(
                            "    log::debug!(\"{}\");",
                            cm.chars().take(24).collect::<String>().to_lowercase()
                        )),
                    }
                }
                src.push(format!("    Ok({v})"));
                src.push("}".into());
            }
            _ => {
                src.push(format!("{name}:"));
                src.push("    push rbp".into());
                src.push("    mov rbp, rsp".into());
                for _ in 0..stmts + 2 {
                    let r1 = REGS[self.rng.usize(..REGS.len())];
                    let r2 = REGS[self.rng.usize(..REGS.len())];
                    match self.rng.u8(..7) {
                        0 => src.push(format!(
                            "    mov {r1}, [rdi+0x{:x}]",
                            self.rng.u8(..) & 0xf8
                        )),
                        1 => src.push(format!("    xor {r1}, 0x{hex:08x}")),
                        2 => src.push(format!("    call {call}")),
                        3 => src.push(format!("    cmp {r1}, {r2}")),
                        4 => src.push(format!("    jnz .L{}", self.rng.u16(..999))),
                        5 => src.push(format!(
                            "    shl {r1}, {k}  ; {}",
                            cm.chars().take(30).collect::<String>().to_lowercase()
                        )),
                        _ => src.push(format!("    lea {r1}, [{r2}+{r2}*4+{k}]")),
                    }
                }
                src.push("    pop rbp".into());
                src.push("    ret".into());
            }
        }
        src.push(String::new());
        for s in src {
            let mut l = Line::new();
            highlight(&s, lang, &mut l);
            self.pending.push_back(l);
        }
    }

    fn birth_errno(&mut self) {
        if self.rng.u8(..100) < 35 {
            let ts = self.clock + self.rng.f32() * 9000.0;
            self.clock = ts;
            let t = |l: &mut Line, n: f32| {
                let mut s = String::new();
                let _ = write!(
                    s,
                    "[{:>5}.{:06}] ",
                    n as u32 % 100_000,
                    (n.fract() * 1e6) as u32
                );
                run(l, Tone::Dim, &s);
            };
            let mut err = String::new();
            if self.mx.is(Kind::Muertos) && self.rng.f32() < muertos::share(self.mx.level) {
                muertos::error(&mut self.rng, &mut err);
            } else if let Some(v) = self.voc()
                && self.rng.f32() < lexico::share(self.mx.level)
            {
                v.error(&mut self.rng, &mut err);
            } else {
                let eidx = self.rng.usize(..babble::ERRORS.len());
                err.push_str(babble::ERRORS[eidx]);
            }
            let mut l = Line::new();
            t(&mut l, ts);
            run(
                &mut l,
                Tone::Red,
                &format!("Kernel panic - not syncing: {err}"),
            );
            self.pending.push_back(l);
            let mut l = Line::new();
            t(&mut l, ts + 0.0001);
            let daemon = daemon(&mut self.rng, self.mx);
            run(
                &mut l,
                Tone::Main,
                &format!(
                    "CPU: {} PID: {} Comm: {daemon} Tainted: G W 5.{}.{}-hax #1",
                    self.rng.u8(..64),
                    self.rng.u32(1..40000),
                    self.rng.u8(4..19),
                    self.rng.u8(..30)
                ),
            );
            self.pending.push_back(l);
            let mut l = Line::new();
            t(&mut l, ts + 0.0002);
            run(&mut l, Tone::Amber, "Call Trace:");
            self.pending.push_back(l);
            for i in 0..self.rng.usize(3..6) {
                let mut l = Line::new();
                t(&mut l, ts + 0.0003 + i as f32 * 0.0001);
                run(
                    &mut l,
                    Tone::Dim,
                    &format!(
                        " [<{:016x}>] ",
                        0xffff_ffff_8100_0000u64.wrapping_add(self.rng.u32(..) as u64 & 0xff_ffff)
                    ),
                );
                let mut id = String::new();
                ident(&mut self.rng, '_', self.mx, &mut id);
                run(&mut l, Tone::Cyan, &id);
                run(
                    &mut l,
                    Tone::Main,
                    &format!(
                        "+0x{:x}/0x{:x}",
                        self.rng.u16(..0x3ff),
                        self.rng.u16(0x400..0xfff)
                    ),
                );
                self.pending.push_back(l);
            }
            let mut l = Line::new();
            t(&mut l, ts + 0.001);
            run(
                &mut l,
                Tone::Red,
                &format!("---[ end Kernel panic - not syncing: {err} ]---"),
            );
            self.pending.push_back(l);
        } else {
            let (name, num, msg) = if self.mx.is(Kind::Muertos)
                && self.rng.f32() < 0.6 * muertos::share(self.mx.level)
            {
                muertos::ERRNOS[self.rng.usize(..muertos::ERRNOS.len())]
            } else if let Some(v) = self.voc()
                && self.rng.f32() < 0.6 * lexico::share(self.mx.level)
            {
                v.errnos[self.rng.usize(..v.errnos.len())]
            } else {
                ERRNOS[self.rng.usize(..ERRNOS.len())]
            };
            let mut l = Line::new();
            run(&mut l, Tone::Red, &format!("{name:<13}"));
            run(&mut l, Tone::Amber, &format!("{num:>3}  "));
            run(&mut l, Tone::Main, msg);
            if self.level > 0 {
                let mut t = String::from("  \u{2014} ");
                title(&mut self.rng, self.mx, &mut t);
                run(&mut l, Tone::Dim, &t);
            }
            self.pending.push_back(l);
        }
    }

    fn birth_sniffer(&mut self) {
        self.seq += 1;
        self.clock += self.rng.f32() * 0.05;
        let mut src = String::new();
        let mut dst = String::new();
        babble::ip(&mut self.rng, &mut src);
        babble::ip(&mut self.rng, &mut dst);
        let special = if self.mx.is(Kind::Muertos) && self.rng.f32() < 0.12 + 0.3 * self.mx.level {
            Some(self.mx_packet())
        } else if let Some(v) = self.voc()
            && self.rng.f32() < 0.12 + 0.3 * self.mx.level
        {
            Some(v.packet(&mut self.rng))
        } else {
            None
        };
        let (proto, tone, len, info) = if let Some(packet) = special {
            packet
        } else {
            self.ordinary_packet(&src, &dst)
        };
        let mut l = Line::new();
        run(
            &mut l,
            Tone::Dim,
            &format!("{:>5}  {:>8.4}  ", self.seq, self.clock % 1000.0),
        );
        run(&mut l, Tone::Main, &format!("{src:<17}{dst:<17}"));
        run(&mut l, tone, &format!("{proto:<7}"));
        run(&mut l, Tone::Main, &format!("{len:>4} {info}"));
        self.pending.push_back(l);
    }

    /// A packet of the Mictlán: (protocol, tone, length, info).
    fn mx_packet(&mut self) -> (&'static str, Tone, u16, String) {
        let soul = muertos::soul(&mut self.rng, self.mx.level);
        let lvl = self.rng.u8(1..10);
        match self.rng.u8(..7) {
            0 => (
                "MICTLAN",
                Tone::Green,
                self.rng.u16(60..300),
                format!("Alma {} cruza el nivel {lvl}", self.rng.u16(100..9999)),
            ),
            1 => (
                "XOLO",
                Tone::Cyan,
                64,
                format!("Handshake OK, la cola mueve {} Hz", self.rng.u8(2..9)),
            ),
            2 => (
                "COPAL",
                Tone::Amber,
                self.rng.u16(60..200),
                format!("Humo {} ppm, ventilacion normal", self.rng.u8(3..60)),
            ),
            3 => (
                "OFRENDA",
                Tone::Bright,
                self.rng.u16(200..1400),
                format!(
                    "{soul}: {} entregado",
                    muertos::offering(&mut self.rng, soul)
                ),
            ),
            4 => (
                "PETALO",
                Tone::Amber,
                self.rng.u16(60..900),
                format!("Peticion de petalos x{}", self.rng.u16(2..900)),
            ),
            5 => (
                "VELA",
                Tone::Red,
                self.rng.u16(60..200),
                format!("Llama estable en el nivel {lvl}"),
            ),
            _ => (
                "ALMA",
                Tone::Green,
                self.rng.u16(60..400),
                format!("{soul} -> casa (ETA {} min)", self.rng.u8(1..90)),
            ),
        }
    }

    /// An everyday packet: (protocol, tone, length, info).
    fn ordinary_packet(&mut self, src: &str, dst: &str) -> (&'static str, Tone, u16, String) {
        match self.rng.u8(..9) {
            0 | 1 => {
                let flags = [
                    "[SYN]",
                    "[SYN, ACK]",
                    "[ACK]",
                    "[PSH, ACK]",
                    "[FIN, ACK]",
                    "[RST]",
                ];
                (
                    "TCP",
                    Tone::Green,
                    self.rng.u16(54..1514),
                    format!(
                        "{} \u{2192} {} {} Seq={} Win={}",
                        self.rng.u16(1024..65535),
                        [22u16, 80, 443, 8080, 3306, 5432][self.rng.usize(..6)],
                        babble::pick(&mut self.rng, &flags),
                        self.rng.u32(..) % 100_000,
                        self.rng.u16(512..65535)
                    ),
                )
            }
            2 => {
                let mut id = String::new();
                ident(&mut self.rng, '-', self.mx, &mut id);
                let domain = if self.mx.is(Kind::Muertos) {
                    "mictlan.mx"
                } else if let Some(v) = self.voc() {
                    v.domain
                } else {
                    "example.net"
                };
                (
                    "DNS",
                    Tone::Cyan,
                    self.rng.u16(70..120),
                    format!("Standard query 0x{:04x} A {id}.{domain}", self.rng.u16(..)),
                )
            }
            3 => {
                let mut id = String::new();
                ident(&mut self.rng, '_', self.mx, &mut id);
                (
                    "HTTP",
                    Tone::Bright,
                    self.rng.u16(200..900),
                    format!("GET /api/v{}/{id} HTTP/1.1", self.rng.u8(1..4)),
                )
            }
            4 => (
                "TLSv1.3",
                Tone::Main,
                self.rng.u16(90..1514),
                "Application Data".to_string(),
            ),
            5 => (
                "ICMP",
                Tone::Red,
                98,
                format!(
                    "Echo (ping) request id=0x{:04x}, seq={}",
                    self.rng.u16(..),
                    self.seq % 400
                ),
            ),
            6 => ("ARP", Tone::Amber, 42, format!("Who has {dst}? Tell {src}")),
            7 => (
                "SSH",
                Tone::Cyan,
                self.rng.u16(60..400),
                format!("Encrypted packet (len={})", self.rng.u16(16..300)),
            ),
            _ => (
                "UDP",
                Tone::Amber,
                self.rng.u16(60..600),
                format!(
                    "{} \u{2192} {} Len={}",
                    self.rng.u16(1024..65535),
                    self.rng.u16(1024..65535),
                    self.rng.u16(8..500)
                ),
            ),
        }
    }

    fn birth_tree(&mut self) {
        if self.n > 40 {
            self.n = 0;
            let mut l = Line::new();
            let text = if self.mx.is(Kind::Muertos) {
                format!(
                    "{} directorios, {} archivos, {} almas",
                    self.rng.u16(40..900),
                    self.rng.u16(300..9000),
                    self.rng.u16(1..300)
                )
            } else if let Some(v) = self.voc() {
                format!(
                    "{} directorios, {} archivos, {} {}",
                    self.rng.u16(40..900),
                    self.rng.u16(300..9000),
                    self.rng.u16(1..300),
                    v.tree_end
                )
            } else {
                format!(
                    "{} directories, {} files",
                    self.rng.u16(40..900),
                    self.rng.u16(300..9000)
                )
            };
            run(&mut l, Tone::Dim, &text);
            self.pending.push_back(l);
            self.depth = 0;
            return;
        }
        self.n += 1;
        let r = self.rng.u8(..100);
        let mut dir = false;
        if r < 25 && self.depth < 4 {
            dir = true;
        } else if r < 42 && self.depth > 0 {
            self.depth -= 1;
        }
        let mut name = String::new();
        // Día de Muertos: an ofrenda of directories and files by their names.
        let holiday: Option<(&'static [&'static str], &'static [&'static str])> =
            if self.mx.is(Kind::Muertos) && self.rng.f32() < muertos::share(self.mx.level) {
                Some((muertos::DIRS, muertos::FILES))
            } else if let Some(v) = self.voc()
                && self.rng.f32() < lexico::share(self.mx.level)
            {
                Some((v.dirs, v.files))
            } else {
                None
            };
        let mx = holiday.is_some();
        if let Some((dirs, files)) = holiday {
            let list = if dir { dirs } else { files };
            name.push_str(babble::pick(&mut self.rng, list));
        } else {
            ident(&mut self.rng, '-', self.mx, &mut name);
        }
        let mut l = Line::new();
        let mut pre = String::new();
        for _ in 0..self.depth {
            pre.push_str("\u{2502}   ");
        }
        pre.push_str(if self.rng.u8(..4) == 0 {
            "\u{2514}\u{2500}\u{2500} "
        } else {
            "\u{251c}\u{2500}\u{2500} "
        });
        run(&mut l, Tone::Dim, &pre);
        if dir {
            run(&mut l, Tone::Cyan, &name);
            run(&mut l, Tone::Cyan, "/");
            self.depth += 1;
        } else {
            let ext = if mx {
                name.rsplit('.').next().unwrap_or("")
            } else {
                babble::pick(&mut self.rng, EXTS)
            };
            let tone = match ext {
                "key" | "pem" | "shadow" | "enc" => Tone::Red,
                "log" | "dat" => Tone::Dim,
                "sh" | "py" | "so" => Tone::Green,
                _ => Tone::Main,
            };
            if mx {
                run(&mut l, tone, &name);
            } else {
                run(&mut l, tone, &format!("{name}.{ext}"));
            }
            if tone == Tone::Red && self.rng.bool() {
                run(&mut l, Tone::Amber, "  [permission denied]");
            }
        }
        self.pending.push_back(l);
    }

    fn birth_dna(&mut self) {
        let len = self.w.saturating_sub(24).clamp(8, 60);
        let protein = self.rng.u8(..10) < 4;
        let alpha: &[u8] = if protein {
            b"ACDEFGHIKLMNPQRSTVWY"
        } else {
            b"ACGT"
        };
        let mut q = Vec::with_capacity(len);
        let mut s = Vec::with_capacity(len);
        let mut mid = Vec::with_capacity(len);
        for _ in 0..len {
            let a = alpha[self.rng.usize(..alpha.len())];
            q.push(a);
            let roll = self.rng.u8(..100);
            if roll < 4 {
                s.push(b'-');
                mid.push(b' ');
            } else if roll < 18 {
                s.push(alpha[self.rng.usize(..alpha.len())]);
                mid.push(if s[s.len() - 1] == a { b'|' } else { b' ' });
            } else {
                s.push(a);
                mid.push(b'|');
            }
        }
        let start = self.rng.u32(1..90_000);
        let color = |line: &mut Line, seq: &[u8]| {
            let mut one = [0u8; 4];
            for &ch in seq {
                let tone = if protein {
                    Tone::Main
                } else {
                    match ch {
                        b'A' => Tone::Green,
                        b'C' => Tone::Cyan,
                        b'G' => Tone::Amber,
                        b'T' => Tone::Red,
                        _ => Tone::Dim,
                    }
                };
                run(line, tone, (ch as char).encode_utf8(&mut one));
            }
        };
        let mut l = Line::new();
        let ident_pct = mid.iter().filter(|&&c| c == b'|').count() * 100 / len;
        let text = if let Some(v) = self.voc() {
            format!(
                "> {}  Puntaje={} bits  Esperado={}e-{}  {}={ident_pct}%",
                babble::pick(&mut self.rng, v.people),
                self.rng.u16(40..900),
                self.rng.u8(1..9),
                self.rng.u16(5..180),
                v.dna_kin
            )
        } else if self.mx.is(Kind::Muertos) {
            // Relatives, matched by their sequences: it runs in the family.
            format!(
                "> {}  Puntaje={} bits  Esperado={}e-{}  Parentesco={ident_pct}%",
                muertos::soul(&mut self.rng, self.mx.level),
                self.rng.u16(40..900),
                self.rng.u8(1..9),
                self.rng.u16(5..180)
            )
        } else {
            let mut name = String::new();
            ident(&mut self.rng, '_', self.mx, &mut name);
            format!(
                "> {name}  Score={} bits  Expect={}e-{}  Identities={ident_pct}%",
                self.rng.u16(40..900),
                self.rng.u8(1..9),
                self.rng.u16(5..180)
            )
        };
        run(&mut l, Tone::Bright, &text);
        self.pending.push_back(l);
        let mut l = Line::new();
        run(&mut l, Tone::Dim, &format!("Query {start:>7} "));
        color(&mut l, &q);
        run(&mut l, Tone::Dim, &format!(" {}", start + len as u32));
        self.pending.push_back(l);
        let mut l = Line::new();
        run(&mut l, Tone::Dim, &format!("      {:>7} ", ""));
        run(&mut l, Tone::Main, std::str::from_utf8(&mid).unwrap_or(""));
        self.pending.push_back(l);
        let mut l = Line::new();
        let sstart = self.rng.u32(1..90_000);
        run(&mut l, Tone::Dim, &format!("Sbjct {sstart:>7} "));
        color(&mut l, &s);
        run(&mut l, Tone::Dim, &format!(" {}", sstart + len as u32));
        self.pending.push_back(l);
        self.pending.push_back(Line::new());
    }

    fn birth_pem(&mut self) {
        let width = self.w.clamp(8, 64);
        if self.n == 0 {
            let mut l = Line::new();
            let text = if let Some(v) = self.voc() {
                v.pem.0
            } else if self.mx.is(Kind::Muertos) {
                "-----BEGIN LLAVE PRIVADA DEL MICTLAN-----"
            } else {
                "-----BEGIN RSA PRIVATE KEY-----"
            };
            run(&mut l, Tone::Amber, text);
            self.pending.push_back(l);
            self.n = self.rng.u32(8..22);
        } else if self.n == 1 {
            let mut l = Line::new();
            let text = if let Some(v) = self.voc() {
                v.pem.1
            } else if self.mx.is(Kind::Muertos) {
                "-----END LLAVE PRIVADA DEL MICTLAN-----"
            } else {
                "-----END RSA PRIVATE KEY-----"
            };
            run(&mut l, Tone::Amber, text);
            self.pending.push_back(l);
            let mut fp = String::from("SHA256:");
            for _ in 0..43 {
                fp.push(B64[self.rng.usize(..62)] as char);
            }
            let mut l = Line::new();
            run(&mut l, Tone::Cyan, &fp);
            self.pending.push_back(l);
            self.pending.push_back(Line::new());
            self.n = 0;
        } else {
            let mut s = String::with_capacity(width);
            if self.n == 2 || self.rng.u8(..8) == 0 {
                s.push_str("MIIEowIBAAKCAQEA");
            }
            while s.len() < width {
                s.push(B64[self.rng.usize(..64)] as char);
            }
            s.truncate(width);
            let mut l = Line::new();
            run(&mut l, Tone::Main, &s);
            self.pending.push_back(l);
            self.n -= 1;
        }
    }

    fn birth_mission(&mut self) {
        self.clock += 1.0 + self.rng.f32() * 20.0;
        let (h, m, s) = self.hms();
        let mut prefix = Line::new();
        run(
            &mut prefix,
            Tone::Dim,
            &format!("[T+{h:02}:{m:02}:{s:02}] "),
        );
        let roll = self.rng.u8(..100);
        let mut text = String::new();
        let tone = if roll < 14 {
            text.push_str(if let Some(v) = self.voc() {
                v.phase
            } else if self.mx.is(Kind::Muertos) {
                "=== FASE "
            } else {
                "=== PHASE "
            });
            let _ = write!(text, "{}: ", self.rng.u8(1..99));
            label(&mut self.rng, self.level, self.mx, &mut text);
            text.push_str(" ===");
            Tone::Amber
        } else if roll < 55 {
            run(&mut prefix, Tone::Green, "[ OK ] ");
            say(&mut self.rng, self.level, self.mx, &mut text);
            Tone::Bright
        } else {
            say(&mut self.rng, self.level, self.mx, &mut text);
            Tone::Main
        };
        self.wrapped(prefix, &text, tone);
    }

    fn birth_password(&mut self) {
        const CS: &[u8] = b"abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789!@#$%^&*-_=+";
        let len = self.rng.usize(10..24);
        let pw: String =
            if self.mx.is(Kind::Muertos) && self.rng.f32() < muertos::share(self.mx.level) {
                // A family's passwords: the name of the dog, a year, an exclamation.
                let mut pw = String::new();
                muertos::password(&mut self.rng, &mut pw);
                pw
            } else if let Some(v) = self.voc()
                && self.rng.f32() < lexico::share(self.mx.level)
            {
                let mut pw = String::new();
                v.password(&mut self.rng, &mut pw);
                pw
            } else {
                (0..len)
                    .map(|_| CS[self.rng.usize(..CS.len())] as char)
                    .collect()
            };
        let len = pw.chars().count();
        let mut l = Line::new();
        run(&mut l, Tone::Bright, &format!("{pw:<24}"));
        run(
            &mut l,
            Tone::Dim,
            &format!("{:>5.1} bits  ", len as f32 * 6.1),
        );
        run(&mut l, Tone::Cyan, &format!("{:08x}", self.rng.u32(..)));
        self.pending.push_back(l);
    }

    fn birth_auth(&mut self) {
        let mut l = Line::new();
        let mut ip = String::new();
        babble::ip(&mut self.rng, &mut ip);
        let user = if self.mx.is(Kind::Muertos) && self.rng.f32() < muertos::share(self.mx.level) {
            babble::pick(&mut self.rng, muertos::USERS)
        } else if let Some(v) = self.voc()
            && self.rng.f32() < lexico::share(self.mx.level)
        {
            babble::pick(&mut self.rng, v.users)
        } else {
            babble::pick(&mut self.rng, USERS)
        };
        run(&mut l, Tone::Cyan, &format!("{user}@{ip} "));
        let pw: String = (0..self.rng.usize(5..12))
            .map(|_| (b'a' + self.rng.u8(..26)) as char)
            .collect();
        run(&mut l, Tone::Main, &format!("pw:{pw} "));
        // The souls are let in more and more as the holiday builds up.
        let denied = if self.mx.is(Kind::Muertos) || self.voc().is_some() {
            88 - (30.0 * self.mx.level) as u8
        } else {
            88
        };
        if self.rng.u8(..100) < denied {
            run(
                &mut l,
                Tone::Red,
                if let Some(v) = self.voc() {
                    v.dress.deny
                } else if self.mx.is(Kind::Muertos) {
                    "DENEGADO"
                } else {
                    "DENIED"
                },
            );
        } else {
            run(
                &mut l,
                Tone::Green,
                if let Some(v) = self.voc() {
                    v.dress.admit
                } else if self.mx.is(Kind::Muertos) {
                    "ADMITIDO"
                } else {
                    "GRANTED"
                },
            );
        }
        self.pending.push_back(l);
    }

    fn step(&mut self) {
        if self.pending.is_empty() {
            self.birth();
        }
        if let Some(l) = self.pending.pop_front() {
            self.lines.push_back(l);
            let cap = self.h.max(1) + 1;
            while self.lines.len() > cap {
                self.lines.pop_front();
            }
        }
    }
}

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// "flux_capacitor" -> "FluxCapacitor".
fn camel(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| {
                f.to_ascii_uppercase().to_string() + c.as_str()
            })
        })
        .collect()
}

impl Widget for Feed {
    fn name(&self) -> &'static str {
        if let Some(v) = self.voc() {
            return v.feeds[self.kind as usize];
        }
        if self.mx.is(Kind::Muertos) {
            return match self.kind {
                FeedKind::Hex => "hexdump -C /dev/ofrenda",
                FeedKind::Log => "tail -f /var/log/mictlan.log",
                FeedKind::Code => "less ofrenda/altar.rs",
                FeedKind::Errno => "dmesg -w",
                FeedKind::Sniffer => "tcpdump -i mictlan0",
                FeedKind::Tree => "tree -a /ofrenda",
                FeedKind::Dna => "blastn -query ancestros",
                FeedKind::Pem => "ssh-keygen -t mictlan",
                FeedKind::Mission => "bit\u{00e1}cora --ofrenda",
                FeedKind::Passwords => "apg -a 1 --familiar",
                FeedKind::Auth => "authd --mictlan",
            };
        }
        match self.kind {
            FeedKind::Hex => "hexdump -C /dev/mem",
            FeedKind::Log => "tail -f /var/log/syslog",
            FeedKind::Code => "less core/exploit",
            FeedKind::Errno => "dmesg -w",
            FeedKind::Sniffer => "tcpdump -i eth0",
            FeedKind::Tree => "tree -a /",
            FeedKind::Dna => "blastn -query",
            FeedKind::Pem => "ssh-keygen -t rsa",
            FeedKind::Mission => "mission-log",
            FeedKind::Passwords => "apg -a 1",
            FeedKind::Auth => "authd",
        }
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.w = w as usize;
        self.h = h as usize;
        self.bpr = if self.w >= 78 {
            16
        } else if self.w >= 44 {
            8
        } else {
            4
        };
        self.lines.clear();
        self.pending.clear();
        self.n = 0;
        // Start with a screenful, not an empty pane.
        for _ in 0..self.h.min(80) {
            self.step();
        }
    }

    fn update(&mut self, dt: f32, _env: &Env) {
        self.acc += dt * self.rate * (0.6 + self.rng.f32() * 0.8);
        let mut steps = 0;
        while self.acc >= 1.0 && steps < 60 {
            self.acc -= 1.0;
            steps += 1;
            self.step();
        }
        self.acc = self.acc.min(1.0);
    }

    fn draw(&mut self, c: &mut Canvas, r: Rect, env: &Env) {
        let pal = &env.pal;
        let top = if let Some(hd) = self.header() {
            if r.h >= 3 {
                put(c, r, 0, 0, hd, pal.cyan, BOLD);
                1
            } else {
                0
            }
        } else {
            0
        };
        let rows = (r.h - top).max(0) as usize;
        for (i, line) in self.lines.iter().rev().take(rows).enumerate() {
            let y = r.h - 1 - i as i32;
            let mut x = 0;
            for (tone, s) in line {
                x = put(
                    c,
                    r,
                    x,
                    y,
                    s,
                    pal.tone(*tone),
                    if *tone == Tone::Bright { BOLD } else { 0 },
                );
            }
        }
    }
}
