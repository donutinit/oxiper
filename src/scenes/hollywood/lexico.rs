//! The words of the holidays after Día de Muertos: Navidad, Fiestas Patrias
//! and Día de la Revolución each bring a [`Vocab`], a pile of hand-written
//! Spanish (and a little English) about their own things, and this module is
//! the engine that turns it into technobabble.
//!
//! Like `muertos.rs` it joins fragments with a small grammar (genders and
//! participles agree: "la piñata cuántica fue sincronizada") and canned
//! lines with blanks to fill (`{s}` a person, `{n}` a number...). Every
//! sentence is built into a caller-supplied `String`, never allocated per
//! frame. A holiday's nouns, adjectives and verbs are mixed 55/45 with the
//! plain technobabble tables of this file, and the share of holiday lines
//! among the ordinary technobabble grows with how far the holiday has come
//! (`share`).

use super::{Tone, babble};
use crate::fiesta::{Fiesta, Kind};
use fastrand::Rng;

/// A packet of the sniffer that belongs to a holiday: `info` has blanks.
pub struct Packet {
    pub proto: &'static str,
    pub tone: Tone,
    pub len: (u16, u16),
    pub info: &'static str,
}

/// How the ordinary widgets are renamed for a holiday: their titles, their
/// labels, the words that fall in the rain, the symbols of the market...
pub struct Dress {
    /// `htop`'s title, the six user names, the memory and swap meters and
    /// the word for "tasks".
    pub htop: &'static str,
    pub users: [&'static str; 6],
    pub mem: &'static str,
    pub swap: &'static str,
    pub tasks: &'static str,
    /// `bmon`'s title and interface, and what comes in and goes out.
    pub bmon: &'static str,
    pub iface: &'static str,
    pub down: &'static str,
    pub up: &'static str,
    pub down_unit: &'static str,
    pub up_unit: &'static str,
    /// The matrix rain's title and the words it spells.
    pub rain: &'static str,
    pub rain_words: &'static [&'static str],
    /// The trace map: title, the status line ("AGUINALDO EN CAMINO") and the
    /// counter of what arrived.
    pub map: &'static str,
    pub trace: &'static str,
    pub arrived: &'static str,
    /// The radar: title, its list header, the middle of the screen and what
    /// the contacts are.
    pub radar: &'static str,
    pub contacts: &'static str,
    pub home: &'static str,
    /// `pv`'s title and what a finished bar says.
    pub progress: &'static str,
    /// The satellite tracker: title, header and names.
    pub sats: &'static str,
    pub sats_head: &'static str,
    pub sat_names: &'static [&'static str],
    /// The market: title, header and (symbol, bullish).
    pub ticker: &'static str,
    pub ticker_head: &'static str,
    pub symbols: &'static [(&'static str, bool)],
    /// `authd`'s title, and the banners: the words of the two lines and the
    /// captions on the frame, for granted and for denied.
    pub authd: &'static str,
    pub granted: [&'static str; 4],
    pub denied: [&'static str; 4],
    /// The login attempts say these two words.
    pub deny: &'static str,
    pub admit: &'static str,
    /// The randomart: title, the closing line and its piles of marks.
    pub keygen: &'static str,
    pub key_title: &'static str,
    pub key_end: &'static str,
    pub key_lines: [&'static str; 3],
    pub art: [char; 15],
    pub art_start: char,
    pub art_walker: char,
    pub art_end: char,
}

/// Everything a holiday adds to the hacker screen's text.
pub struct Vocab {
    /// Who and what the blanks stand for: `{s}` a person, `{u}` a user name,
    /// `{o}` a thing, `{t}` a place.
    pub people: &'static [&'static str],
    pub users: &'static [&'static str],
    pub things: &'static [&'static str],
    pub places: &'static [&'static str],
    /// File names for transfers and trees, directories for `tree`, daemons
    /// for `htop` and the logs, hosts for the log.
    pub files: &'static [&'static str],
    pub dirs: &'static [&'static str],
    pub daemons: &'static [&'static str],
    pub hosts: &'static [&'static str],
    /// Errno values: (name, number, message).
    pub errnos: &'static [(&'static str, u8, &'static str)],
    /// Variable names and error variants in the code listings.
    pub locals: &'static [&'static str],
    pub errs: &'static [&'static str],
    /// Words for identifiers ("piñata_enrutador") and for passwords.
    pub holy: &'static [&'static str],
    pub passwords: &'static [&'static str],
    /// The tape along the bottom (UPPER-CASE, with blanks), and the
    /// headlines of particular days of the holiday besides those.
    pub headlines: &'static [&'static str],
    pub days: fn(Fiesta) -> &'static [&'static str],
    /// Songs whose verses scroll through the logs, in order, among the
    /// technobabble: (title, lines), and how often (at the height of the
    /// holiday it is a bit more than twice `verse_odds`).
    pub songs: &'static [(&'static str, &'static [&'static str])],
    pub verse_odds: f32,
    /// The holiday's own words for the grammar.
    pub nouns: &'static [(&'static str, bool)],
    pub adjs: &'static [&'static str],
    pub ofs: &'static [&'static str],
    pub verbs: &'static [(&'static str, &'static str, &'static str)],
    pub states: &'static [&'static str],
    pub errors: &'static [&'static str],
    pub units: &'static [&'static str],
    /// Hand-written lines with blanks, in Spanish and in English.
    pub canned_es: &'static [&'static str],
    pub canned_en: &'static [&'static str],
    /// Plain lines for `babble = 0`; the first `mundane_es` are Spanish.
    pub mundane: &'static [&'static str],
    pub mundane_es: usize,
    /// Packets of the sniffer, the domain they resolve under and its header.
    pub packets: &'static [Packet],
    pub domain: &'static str,
    pub sniff: &'static str,
    /// The titles of the eleven text widgets, in `FeedKind` order.
    pub feeds: [&'static str; 11],
    /// The sequence aligner: header, name of the score, of the kinship and
    /// the closing line of `tree` (its last word is the count of people).
    pub dna_head: &'static str,
    pub dna_kin: &'static str,
    pub tree_end: &'static str,
    /// Key blocks and mission phases.
    pub pem: (&'static str, &'static str),
    pub phase: &'static str,
    /// The mark on the tape between headlines.
    pub mark: char,
    pub dress: Dress,
}

/// The vocabulary of the day's holiday, if it has one (Día de Muertos has
/// its own, `muertos.rs`).
pub fn of(f: Fiesta) -> Option<&'static Vocab> {
    match f.kind {
        Kind::Navidad => Some(&super::navidad::VOCAB),
        _ => None,
    }
}

/// The share of lines that are the holiday's: half already on the first
/// day, nearly all at the height.
pub fn share(level: f32) -> f32 {
    0.5 + 0.45 * level.clamp(0.0, 1.0)
}

#[inline]
fn pick<T: Copy>(rng: &mut Rng, list: &[T]) -> T {
    list[rng.usize(..list.len())]
}

/// Upper-cases the first letter of `s[from..]` in place (accents included).
fn capitalize_from(s: &mut String, from: usize) {
    if let Some(c) = s[from..].chars().next()
        && c.is_lowercase()
    {
        let up: String = c.to_uppercase().collect();
        s.replace_range(from..from + c.len_utf8(), &up);
    }
}

/// Appends `s` with its `*` turned into `o` or `a`: "cuántic*" is "cuántico"
/// for a masculine noun, "cuántica" for a feminine one.
fn agree(s: &str, fem: bool, out: &mut String) {
    for ch in s.chars() {
        if ch == '*' {
            out.push(if fem { 'a' } else { 'o' });
        } else {
            out.push(ch);
        }
    }
}

// ------------------------------------------------------- plain technobabble

/// Nouns: (word, feminine).
const NOUNS: &[(&str, bool)] = &[
    ("servidor", false),
    ("firewall", false),
    ("enrutador", false),
    ("búfer", false),
    ("kernel", false),
    ("algoritmo", false),
    ("protocolo", false),
    ("mainframe", false),
    ("disco duro", false),
    ("cortafuegos", false),
    ("repositorio", false),
    ("compilador", false),
    ("hipervisor", false),
    ("balanceador de carga", false),
    ("certificado", false),
    ("hash", false),
    ("token", false),
    ("demonio", false),
    ("túnel VPN", false),
    ("clúster", false),
    ("firmware", false),
    ("procesador", false),
    ("sistema de archivos", false),
    ("planificador", false),
    ("backend", false),
    ("dashboard", false),
    ("pull request", false),
    ("deploy", false),
    ("cron", false),
    ("script", false),
    ("bug", false),
    ("base de datos", true),
    ("red neuronal", true),
    ("nube", true),
    ("contraseña", true),
    ("llave", true),
    ("puerta de enlace", true),
    ("memoria caché", true),
    ("tabla de páginas", true),
    ("pila", true),
    ("cola de mensajes", true),
    ("interfaz", true),
    ("matriz", true),
    ("antena parabólica", true),
    ("señal", true),
    ("dirección IP", true),
    ("tarjeta madre", true),
    ("copia de seguridad", true),
    ("cadena de bloques", true),
    ("máquina virtual", true),
    ("consola", true),
    ("bitácora", true),
    ("granja de servidores", true),
    ("conexión", true),
    ("arquitectura", true),
    ("API", true),
    ("red mesh", true),
    ("memoria RAM", true),
];

/// Adjectives; a `*` is the `o` or `a` that agrees with the noun.
const ADJS: &[&str] = &[
    "cuántic*",
    "polimórfic*",
    "asíncron*",
    "redundante",
    "distribuid*",
    "cifrad*",
    "heurístic*",
    "recursiv*",
    "neuronal",
    "holográfic*",
    "fractal",
    "estocástic*",
    "ultrasecret*",
    "clandestin*",
    "autónom*",
    "sintiente",
    "hiperconectad*",
    "virtual",
    "biométric*",
    "comprometid*",
    "inestable",
    "volátil",
    "persistente",
    "efímer*",
    "inmutable",
    "blindad*",
    "hackead*",
    "de grado militar",
    "de alta disponibilidad",
    "tolerante a fallas",
    "artesanal",
    "orgánic*",
    "de temporada",
    "en la nube",
    "open source",
    "legacy",
    "zero-day",
];

const DET_M: &[&str] = &[
    "el",
    "el",
    "el",
    "el",
    "el",
    "este",
    "aquel",
    "cada",
    "nuestro",
    "tu",
    "el único",
    "todo el",
    "el mismísimo",
    "el famoso",
];
const DET_F: &[&str] = &[
    "la",
    "la",
    "la",
    "la",
    "la",
    "esta",
    "aquella",
    "cada",
    "nuestra",
    "tu",
    "la única",
    "toda la",
    "la mismísima",
    "la famosa",
];

/// Verbs as (infinitive, gerund, participle without its `o`/`a`).
const VERBS: &[(&str, &str, &str)] = &[
    ("sincronizar", "sincronizando", "sincronizad"),
    ("compilar", "compilando", "compilad"),
    ("descifrar", "descifrando", "descifrad"),
    ("cifrar", "cifrando", "cifrad"),
    ("reenrutar", "reenrutando", "reenrutad"),
    ("calibrar", "calibrando", "calibrad"),
    ("desfragmentar", "desfragmentando", "desfragmentad"),
    ("hackear", "hackeando", "hackead"),
    ("rastrear", "rastreando", "rastread"),
    ("triangular", "triangulando", "triangulad"),
    ("encender", "encendiendo", "encendid"),
    ("apagar", "apagando", "apagad"),
    ("desplegar", "desplegando", "desplegad"),
    ("reiniciar", "reiniciando", "reiniciad"),
    ("comprimir", "comprimiendo", "comprimid"),
    ("subir", "subiendo", "subid"),
    ("descargar", "descargando", "descargad"),
    ("respaldar", "respaldando", "respaldad"),
    ("clonar", "clonando", "clonad"),
    ("parchear", "parcheando", "parchead"),
    ("escanear", "escaneando", "escanead"),
    ("indexar", "indexando", "indexad"),
    ("canalizar", "canalizando", "canalizad"),
    ("liberar", "liberando", "liberad"),
    ("autenticar", "autenticando", "autenticad"),
    ("verificar", "verificando", "verificad"),
    ("optimizar", "optimizando", "optimizad"),
    ("restaurar", "restaurando", "restaurad"),
    ("monitorear", "monitoreando", "monitoread"),
    ("transmitir", "transmitiendo", "transmitid"),
    ("interceptar", "interceptando", "interceptad"),
    ("inyectar", "inyectando", "inyectad"),
    ("enrutar", "enrutando", "enrutad"),
    ("amplificar", "amplificando", "amplificad"),
    ("modular", "modulando", "modulad"),
    ("encriptar", "encriptando", "encriptad"),
    ("desencriptar", "desencriptando", "desencriptad"),
    ("resetear", "reseteando", "resetead"),
    ("debuggear", "debuggeando", "debuggead"),
    ("deployear", "deployeando", "deployead"),
    ("commitear", "commiteando", "commitead"),
    ("pushear", "pusheando", "pushead"),
    ("virtualizar", "virtualizando", "virtualizad"),
    ("balancear", "balanceando", "balancead"),
];

/// How a noun can be: invariable or with a `*` for its gender.
const STATES: &[&str] = &[
    "sobrecalentándose",
    "oscilando sin control",
    "perdiendo entropía",
    "alucinando",
    "atrapad* en un bucle recursivo",
    "completamente comprometid*",
    "peligrosamente inestable",
    "vibrando en resonancia",
    "haciendo ruidos extraños",
    "ignorando todos los comandos",
    "fuera de fase",
    "consciente de sí mism*",
    "negándose a cooperar",
    "inusualmente silencios*",
    "cargando (desde 1998)",
    "en punto muerto",
    "en interbloqueo",
    "esperando quórum",
    "pendiente de reinicio",
    "a punto de explotar",
    "fuera de las tolerancias de diseño",
    "hablando con la subred equivocada",
    "intercambiad* en caliente por error",
    "sin punto y coma",
    "sin café",
    "llamando a casa",
    "aprendiendo demasiado rápido",
    "dormid*",
];

/// Whole failures: "Error al desplegar la X: {this}".
const ERRORS: &[&str] = &[
    "falla de segmentación en el múltiple de flujo",
    "desbordamiento de búfer en 0xDEADBEEF",
    "el certificado fue revocado por una autoridad superior",
    "volcado de núcleo",
    "pánico del kernel: no se sincroniza",
    "permiso denegado (prueba con sudo, pero con más respeto)",
    "tiempo de espera agotado",
    "excepción de punto flotante",
    "heisenbug observado (ahora sí funciona)",
    "la entropía se agotó",
    "violación de protocolo",
    "bucle infinito en la cola de espera",
    "paradoja temporal detectada",
    "referencia nula en el módulo principal",
    "doble liberación o corrupción",
    "firma inválida",
    "no hay espacio en el disco",
    "conexión rechazada por el servidor",
    "demasiados archivos abiertos",
    "memoria insuficiente",
    "sin señal",
    "token vencido",
];

const TIMES: &[&str] = &[
    "milisegundos",
    "segundos",
    "microsegundos",
    "ciclos de reloj",
    "latidos",
    "nanosegundos",
    "jiffies",
    "épocas",
];

const UNITS: &[&str] = &[
    "THz",
    "GB/s",
    "teslas",
    "qubits",
    "rpm",
    "megavatios",
    "pársecs",
    "kilobaudios",
    "petaflops",
    "terabytes",
];

/// Technical words that join a holiday's in identifiers.
const TECH: &[&str] = &[
    "servidor",
    "enrutador",
    "cache",
    "buffer",
    "token",
    "socket",
    "hilo",
    "pila",
    "cola",
    "nucleo",
    "red",
    "nube",
    "llave",
    "firma",
    "puente",
    "tunel",
    "registro",
    "indice",
    "demonio",
    "sincronizador",
    "cifrador",
    "planificador",
    "gateway",
    "daemon",
    "handler",
    "parser",
    "relay",
];

// ------------------------------------------------------------------ engine

impl Vocab {
    fn noun(&self, rng: &mut Rng) -> (&'static str, bool) {
        if rng.u8(..20) < 11 {
            pick(rng, self.nouns)
        } else {
            pick(rng, NOUNS)
        }
    }

    fn adj(&self, rng: &mut Rng) -> &'static str {
        if rng.u8(..20) < 9 {
            pick(rng, self.adjs)
        } else {
            pick(rng, ADJS)
        }
    }

    fn verb(&self, rng: &mut Rng) -> (&'static str, &'static str, &'static str) {
        if rng.u8(..20) < 9 {
            pick(rng, self.verbs)
        } else {
            pick(rng, VERBS)
        }
    }

    fn state(&self, rng: &mut Rng) -> &'static str {
        if rng.u8(..20) < 10 {
            pick(rng, self.states)
        } else {
            pick(rng, STATES)
        }
    }

    fn error_text(&self, rng: &mut Rng) -> &'static str {
        if rng.u8(..20) < 10 {
            pick(rng, self.errors)
        } else {
            pick(rng, ERRORS)
        }
    }

    /// "4.2 kilopiñatas": a measure of the holiday or an ordinary one.
    fn measure(&self, rng: &mut Rng, out: &mut String) {
        use std::fmt::Write;
        let _ = match rng.u8(..3) {
            0 => write!(out, "{}", rng.u32(2..999)),
            1 => write!(out, "{}.{}", rng.u32(1..99), rng.u32(0..9)),
            _ => write!(out, "{}", rng.u32(10..9999)),
        };
        out.push(' ');
        out.push_str(if rng.bool() {
            pick(rng, self.units)
        } else {
            pick(rng, UNITS)
        });
    }

    /// "la piñata cuántica de la abuela"; `det` adds the article. Returns
    /// whether the noun is feminine, for what has to agree with it.
    fn noun_phrase(&self, rng: &mut Rng, level: u8, det: bool, out: &mut String) -> bool {
        let (noun, fem) = self.noun(rng);
        if det {
            out.push_str(pick(rng, if fem { DET_F } else { DET_M }));
            out.push(' ');
        }
        out.push_str(noun);
        if rng.u8(..10) < 7 {
            out.push(' ');
            agree(self.adj(rng), fem, out);
            if level >= 7 && rng.u8(..10) < 3 {
                out.push_str(" y ");
                agree(self.adj(rng), fem, out);
            }
        }
        if rng.u8(..10) < 3 {
            out.push(' ');
            out.push_str(pick(rng, self.ofs));
        }
        fem
    }

    /// `prefix` (ending in "de ") and a noun phrase, contracting "de el"
    /// into "del".
    fn de_np(&self, rng: &mut Rng, level: u8, prefix: &str, out: &mut String) {
        out.push_str(prefix);
        let at = out.len();
        self.noun_phrase(rng, level, true, out);
        if out[at..].starts_with("el ") {
            out.replace_range(at - 3..at + 3, "del ");
        }
    }

    /// A trailing clause: "mediante el X", "a 4 THz", "(código 0x1F)".
    fn tail(&self, rng: &mut Rng, level: u8, out: &mut String) {
        use std::fmt::Write;
        out.push(' ');
        match rng.u8(..13) {
            0 => {
                out.push_str("con ");
                self.noun_phrase(rng, level, true, out);
            }
            1 => {
                out.push_str("mediante ");
                self.noun_phrase(rng, level, true, out);
            }
            2 => self.de_np(rng, level, "a través de ", out),
            3 => {
                out.push_str("usando ");
                self.noun_phrase(rng, level, true, out);
            }
            4 => self.de_np(rng, level, "dentro de ", out),
            5 => {
                out.push_str("contra ");
                self.noun_phrase(rng, level, true, out);
            }
            6 => {
                out.push_str("vía ");
                self.noun_phrase(rng, level, true, out);
            }
            7 => {
                out.push_str("hacia ");
                self.noun_phrase(rng, level, true, out);
            }
            8 => {
                out.push_str("a ");
                self.measure(rng, out);
            }
            9 => {
                let _ = write!(out, "por {} {}", rng.u32(2..9999), pick(rng, TIMES));
            }
            10 => {
                out.push_str("en modo ");
                agree(self.adj(rng), false, out);
            }
            11 => {
                let _ = write!(out, "(código 0x{:04X})", rng.u16(..));
            }
            _ => {
                let _ = write!(out, "en el puerto {}", rng.u16(1..65535));
            }
        }
    }

    /// A sentence built from the Spanish tables.
    fn sentence_es(&self, rng: &mut Rng, level: u8, out: &mut String) {
        use std::fmt::Write;
        let from = out.len();
        let tails = |rng: &mut Rng, out: &mut String| {
            for _ in 0..rng.u8(..=(level / 3).min(3)) {
                self.tail(rng, level, out);
            }
        };
        match rng.u8(..12) {
            0..=2 => {
                out.push_str(self.verb(rng).1);
                out.push(' ');
                self.noun_phrase(rng, level, true, out);
                tails(rng, out);
            }
            3 => {
                let _ = write!(out, "{} ", self.verb(rng).1);
                self.noun_phrase(rng, level, true, out);
                let _ = write!(out, " y {} ", self.verb(rng).1);
                self.noun_phrase(rng, level, true, out);
            }
            4 => {
                let _ = write!(out, "error al {} ", self.verb(rng).0);
                self.noun_phrase(rng, level, true, out);
                let _ = write!(out, ": {}", self.error_text(rng));
            }
            5 => {
                let fem = self.noun_phrase(rng, level, true, out);
                let _ = write!(out, " fue {}", self.verb(rng).2);
                out.push(if fem { 'a' } else { 'o' });
                out.push_str(" con éxito (");
                self.measure(rng, out);
                out.push(')');
            }
            6 => {
                out.push_str("advertencia: ");
                let fem = self.noun_phrase(rng, level, true, out);
                out.push_str(" está ");
                agree(self.state(rng), fem, out);
            }
            7 => {
                out.push_str("activando ");
                self.noun_phrase(rng, level, true, out);
                tails(rng, out);
            }
            8 => {
                let fem = self.noun_phrase(rng, level, true, out);
                out.push_str(" está ");
                agree(self.state(rng), fem, out);
                let _ = write!(out, "; {} ", self.verb(rng).1);
                self.noun_phrase(rng, level, true, out);
            }
            9 => {
                let _ = write!(out, "favor de {} ", self.verb(rng).0);
                self.noun_phrase(rng, level, true, out);
                out.push_str(" antes de que ");
                let fem = self.noun_phrase(rng, level, true, out);
                out.push_str(" esté ");
                agree(self.state(rng), fem, out);
            }
            10 => {
                let _ = write!(out, "{} ", self.verb(rng).1);
                self.noun_phrase(rng, level, true, out);
                out.push_str(" a ");
                self.measure(rng, out);
            }
            _ => {
                let _ = write!(out, "no se pudo {} ", self.verb(rng).0);
                self.noun_phrase(rng, level, true, out);
                let _ = write!(
                    out,
                    " (reintentar en {} {})",
                    rng.u32(2..900),
                    pick(rng, TIMES)
                );
            }
        }
        capitalize_from(out, from);
    }

    /// Expands the blanks of a canned line into `out`: `{n}` a number, `{l}`
    /// a digit from 1 to 9, `{s}` a person, `{u}` a user name, `{o}` a thing,
    /// `{t}` a place, `{p}` a percentage, `{x}` a hex word.
    pub fn expand(&self, rng: &mut Rng, tpl: &str, out: &mut String) {
        use std::fmt::Write;
        let mut chars = tpl.chars();
        while let Some(ch) = chars.next() {
            if ch != '{' {
                out.push(ch);
                continue;
            }
            let key = chars.next().unwrap_or('}');
            let _ = chars.next(); // the closing brace
            match key {
                'n' => {
                    let _ = write!(out, "{}", rng.u32(2..9999));
                }
                'l' => {
                    let _ = write!(out, "{}", rng.u8(1..=9));
                }
                's' => out.push_str(pick(rng, self.people)),
                'u' => out.push_str(pick(rng, self.users)),
                'o' => out.push_str(pick(rng, self.things)),
                't' => out.push_str(pick(rng, self.places)),
                'p' => {
                    let _ = write!(out, "{}", rng.u8(1..100));
                }
                'x' => {
                    let _ = write!(out, "{:04X}", rng.u16(..));
                }
                _ => {}
            }
        }
    }

    /// A holiday sentence, in Spanish or English, canned or built. At `level`
    /// 0 (plain logs) one of the plain lines. Returns whether it is Spanish.
    pub fn sentence(&self, rng: &mut Rng, level: u8, out: &mut String) -> bool {
        let from = out.len();
        let es = if level == 0 {
            let i = rng.usize(..self.mundane.len());
            out.push_str(self.mundane[i]);
            i < self.mundane_es
        } else {
            match rng.u8(..10) {
                0..=2 => {
                    let tpl = pick(rng, self.canned_es);
                    self.expand(rng, tpl, out);
                    true
                }
                3 => {
                    let tpl = pick(rng, self.canned_en);
                    self.expand(rng, tpl, out);
                    false
                }
                _ => {
                    self.sentence_es(rng, level.min(10), out);
                    true
                }
            }
        };
        capitalize_from(out, from);
        es
    }

    /// A sentence of the holiday or an ordinary one, in the proportion
    /// `share(fiesta_level)`. Returns whether it is Spanish.
    pub fn mixed(&self, rng: &mut Rng, level: u8, fiesta_level: f32, out: &mut String) -> bool {
        if rng.f32() < share(fiesta_level) {
            self.sentence(rng, level, out)
        } else {
            babble::sentence(rng, level, out);
            false
        }
    }

    /// A failure for a kernel panic.
    pub fn error(&self, rng: &mut Rng, out: &mut String) {
        out.push_str(self.error_text(rng));
    }

    /// A short lower-case name for a pane title: "piñata cuántica".
    pub fn title(&self, rng: &mut Rng, out: &mut String) {
        let from = out.len();
        let (noun, fem) = self.noun(rng);
        out.push_str(noun);
        out.push(' ');
        agree(self.adj(rng), fem, out);
        let lower = out[from..].to_lowercase();
        out.truncate(from);
        out.push_str(&lower);
    }

    /// A title of the holiday or an ordinary one.
    pub fn mixed_title(&self, rng: &mut Rng, fiesta_level: f32, out: &mut String) {
        if rng.f32() < share(fiesta_level) {
            self.title(rng, out);
        } else {
            babble::title(rng, out);
        }
    }

    /// An UPPER-CASE progress label ("SINCRONIZANDO LA PIÑATA CUÁNTICA") or a
    /// transfer ("uploading aguinaldo_2026.tar.gz").
    pub fn progress_label(&self, rng: &mut Rng, level: u8, out: &mut String) {
        use std::fmt::Write;
        if rng.u8(..5) < 2 {
            const VERBS: &[&str] = &[
                "uploading",
                "downloading",
                "subiendo",
                "descargando",
                "respaldando",
                "transfiriendo",
                "syncing",
                "restaurando",
            ];
            let _ = write!(
                out,
                "{} {} ({}.{} GB)",
                pick(rng, VERBS),
                pick(rng, self.files),
                rng.u8(0..9),
                rng.u8(0..10)
            );
            return;
        }
        let from = out.len();
        out.push_str(self.verb(rng).1);
        out.push(' ');
        self.noun_phrase(rng, level.min(4), true, out);
        let up = out[from..].to_uppercase();
        out.truncate(from);
        out.push_str(&up);
    }

    /// A headline for the tape along the bottom (already UPPER-CASE).
    pub fn headline(&self, rng: &mut Rng, out: &mut String) {
        let from = out.len();
        let tpl = pick(rng, self.headlines);
        self.expand(rng, tpl, out);
        let up = out[from..].to_uppercase();
        out.truncate(from);
        out.push_str(&up);
    }

    /// "pinata_servidor", "token-posada": an identifier in snake or kebab case.
    pub fn ident(&self, rng: &mut Rng, sep: char, out: &mut String) {
        let word = |w: &str, out: &mut String| {
            for ch in w.chars() {
                out.push(if ch == '_' { sep } else { ch });
            }
        };
        let (a, b) = (pick(rng, self.holy), pick(rng, TECH));
        if rng.bool() {
            word(a, out);
            out.push(sep);
            word(b, out);
        } else {
            word(b, out);
            out.push(sep);
            word(a, out);
        }
    }

    /// A password the way a family would choose it: "P1n4ta!2025".
    pub fn password(&self, rng: &mut Rng, out: &mut String) {
        let word = pick(rng, self.passwords);
        let leet = rng.u8(..3) > 0;
        for (i, ch) in word.chars().enumerate() {
            let c = match ch {
                'a' if leet && rng.bool() => '4',
                'e' if leet && rng.bool() => '3',
                'o' if leet && rng.bool() => '0',
                'i' if leet && rng.bool() => '1',
                's' if leet && rng.u8(..3) == 0 => '5',
                c if i == 0 && rng.bool() => c.to_ascii_uppercase(),
                c => c,
            };
            out.push(c);
        }
        out.push(pick(rng, &['!', '#', '_', '.', '*', '-']));
        for _ in 0..rng.usize(2..5) {
            out.push((b'0' + rng.u8(..10)) as char);
        }
    }

    /// A packet of the sniffer: (protocol, tone, length, info).
    pub fn packet(&self, rng: &mut Rng) -> (&'static str, Tone, u16, String) {
        let p = &self.packets[rng.usize(..self.packets.len())];
        let mut info = String::new();
        self.expand(rng, p.info, &mut info);
        (p.proto, p.tone, rng.u16(p.len.0..=p.len.1), info)
    }

    /// Number of hand-written fragments (for the tests).
    #[cfg(test)]
    pub fn fragment_count(&self) -> usize {
        self.people.len()
            + self.users.len()
            + self.things.len()
            + self.places.len()
            + self.files.len()
            + self.dirs.len()
            + self.daemons.len()
            + self.errnos.len()
            + self.headlines.len()
            + self.nouns.len()
            + self.adjs.len()
            + self.ofs.len()
            + self.verbs.len() * 3
            + self.states.len()
            + self.errors.len()
            + self.units.len()
            + self.canned_es.len()
            + self.canned_en.len()
            + self.mundane.len()
            + NOUNS.len()
            + ADJS.len()
            + VERBS.len() * 3
            + STATES.len()
            + ERRORS.len()
    }
}

#[cfg(test)]
pub const ALL: [&Vocab; 1] = [&super::navidad::VOCAB];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn plenty_of_clean_text_in_both_languages() {
        for v in ALL {
            assert!(
                v.fragment_count() >= 600,
                "{} {}",
                v.feeds[0],
                v.fragment_count()
            );
            let mut rng = Rng::with_seed(9);
            let mut set = HashSet::new();
            let mut s = String::new();
            let (mut es, mut en) = (0, 0);
            for _ in 0..10_000 {
                s.clear();
                v.sentence(&mut rng, 7, &mut s);
                assert!(
                    !s.is_empty()
                        && !s.contains("  ")
                        && !s.contains('{')
                        && !s.contains('}')
                        && !s.contains('*'),
                    "{}: {s:?}",
                    v.feeds[0]
                );
                if s.chars().any(|c| "áéíóúñ¡¿".contains(c))
                    || s.contains(" el ")
                    || s.contains(" la ")
                {
                    es += 1;
                } else {
                    en += 1;
                }
                set.insert(s.clone());
            }
            assert!(set.len() >= 8_000, "{} {}", v.feeds[0], set.len());
            assert!(es > 4_000 && en > 300, "{} {es} {en}", v.feeds[0]);
        }
    }

    #[test]
    fn spanish_agrees() {
        for v in ALL {
            let mut rng = Rng::with_seed(3);
            let mut s = String::new();
            for _ in 0..3_000 {
                s.clear();
                v.sentence_es(&mut rng, 10, &mut s);
                assert!(!s.contains('*') && !s.contains("  "), "{s}");
            }
        }
        let mut out = String::new();
        agree("cuántic*", true, &mut out);
        out.push(' ');
        agree("cuántic*", false, &mut out);
        out.push(' ');
        agree("fractal", true, &mut out);
        assert_eq!(out, "cuántica cuántico fractal");
    }

    #[test]
    fn titles_labels_idents_passwords_and_packets_are_clean() {
        for v in ALL {
            let mut rng = Rng::with_seed(4);
            let mut s = String::new();
            for _ in 0..500 {
                s.clear();
                v.title(&mut rng, &mut s);
                assert!(!s.is_empty() && !s.contains('*') && s == s.to_lowercase());
                s.clear();
                v.progress_label(&mut rng, 8, &mut s);
                assert!(
                    !s.is_empty() && !s.contains('*') && !s.contains("  "),
                    "{s}"
                );
                s.clear();
                v.ident(&mut rng, '_', &mut s);
                assert!(s.is_ascii() && s.contains('_'), "{s}");
                s.clear();
                v.password(&mut rng, &mut s);
                assert!(s.len() >= 6 && s.is_ascii(), "{s}");
                s.clear();
                v.headline(&mut rng, &mut s);
                assert!(!s.contains('{') && s == s.to_uppercase(), "{s}");
                s.clear();
                v.error(&mut rng, &mut s);
                assert!(!s.is_empty() && !s.contains('*'));
                let (proto, _, len, info) = v.packet(&mut rng);
                assert!(
                    !proto.is_empty() && len >= 40 && !info.contains('{'),
                    "{info}"
                );
            }
        }
    }

    #[test]
    fn share_grows_with_the_holiday() {
        assert!(share(0.25) > 0.5 && share(0.25) < share(0.6));
        assert!(share(0.6) < share(1.0) && share(1.0) <= 1.0);
    }

    #[test]
    fn the_dress_fits_the_widgets() {
        for v in ALL {
            let d = &v.dress;
            assert!(d.rain_words.iter().all(|w| w.is_ascii() && w.len() >= 3));
            assert!(d.sat_names.len() >= 5 && d.symbols.len() >= 8);
            assert!(d.symbols.iter().any(|s| s.1));
            assert!(d.granted.iter().chain(&d.denied).all(|t| !t.is_empty()));
            // The banner words must be drawable in the 3x5 font.
            for w in [d.granted[0], d.granted[1], d.denied[0], d.denied[1]] {
                assert!(
                    w.chars()
                        .all(|c| c == ' ' || super::super::widgets::has_glyph(c)),
                    "{w}"
                );
            }
        }
    }
}
