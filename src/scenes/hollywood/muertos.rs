//! Día de Muertos for the hacker screen: the Mictlán's technobabble.
//!
//! Hand-written fragments in Spanish and English, joined by a small grammar
//! (genders and participles agree in Spanish: "la ofrenda cuántica fue
//! sincronizada") and by canned lines with blanks to fill (`{s}` a soul,
//! `{n}` a number...). Like `babble.rs` everything is built into a
//! caller-supplied `String`, never allocated per frame. The share of
//! holiday lines among the ordinary technobabble grows with how far the
//! holiday has come (`share`).

use super::babble;
use fastrand::Rng;

/// Names of those coming home: family, friends and a few good dogs.
pub const SOULS: &[&str] = &[
    "Abuela Chela",
    "Abuelo Nacho",
    "Tía Lupe",
    "Tío Beto",
    "Don Chuy",
    "Doña Carmen",
    "Papá Chema",
    "Comadre Rosa",
    "Tía Chona",
    "Abuela Pachita",
    "Tío Rigo",
    "Primo Lalo",
    "Don Memo",
    "Doña Chayo",
    "Padrino Nando",
    "Madrina Conchi",
    "Tía Meche",
    "Abuelo Tomás",
    "Don Pancho",
    "Doña Lola",
    "Tata Chuy",
    "Nana Tere",
    "Tío Chucho",
    "Tía Cuca",
    "Mamá Lupita",
    "Abuelo Gil",
    "Doña Reyna",
    "Don Fermín",
    "Tía Beba",
    "Tío Güero",
    "Don Ignacio",
    "Doña Panchita",
    "Compadre Toño",
    "Tía Goya",
    "Abuela Eva",
    "Firulais",
    "Canelo",
    "Michi",
    "Pelusa",
    "Chispas",
];

/// The little ones, the angelitos, who come on October 31 and November 1.
pub const ANGELITOS: &[&str] = &[
    "Lupita",
    "Chucho",
    "Panchito",
    "Tomasito",
    "Rosita",
    "Mateíto",
    "Paquito",
    "Toñito",
    "Lalito",
    "Carmelita",
    "Pepito",
    "Mariquita",
    "Juanito",
    "Memito",
    "Güerita",
    "Beto Jr.",
];

/// What they are given: toys and sweets.
pub const TOYS: &[&str] = &[
    "trompo",
    "carrito de madera",
    "muñeca de trapo",
    "dulces",
    "pan dulce",
    "canicas",
    "balero",
    "atole",
    "leche",
    "chocolate",
    "paleta de dulce",
    "calaverita de azúcar",
    "globo",
    "silbato",
    "lotería",
    "yoyo",
];

/// A soul that comes home: an adult, or from the 31st (level 0.6) on one of
/// the angelitos, most of all on November 1 (0.8), the day of the children.
pub fn soul(rng: &mut Rng, level: f32) -> &'static str {
    let angelito = match level {
        l if l < 0.55 => 0.0,
        l if l < 0.7 => 0.2,
        l if l < 0.9 => 0.45,
        _ => 0.3,
    };
    if angelito > 0.0 && rng.f32() < angelito {
        pick(rng, ANGELITOS)
    } else {
        pick(rng, SOULS)
    }
}

/// What is offered to `soul`: toys and sweets for the little ones.
pub fn offering(rng: &mut Rng, soul: &str) -> &'static str {
    if ANGELITOS.contains(&soul) {
        pick(rng, TOYS)
    } else {
        pick(rng, OFFERINGS)
    }
}

/// The same souls as user names, for logins and shells.
pub const USERS: &[&str] = &[
    "abuela_chela",
    "abuelo_nacho",
    "tia_lupe",
    "tio_beto",
    "don_chuy",
    "dona_carmen",
    "papa_chema",
    "comadre_rosa",
    "tia_chona",
    "tio_rigo",
    "primo_lalo",
    "don_memo",
    "dona_chayo",
    "padrino_nando",
    "tia_meche",
    "tata_chuy",
    "nana_tere",
    "tio_chucho",
    "la_catrina",
    "xolo",
    "posada",
    "jose_alfredo",
    "pedro_infante",
    "frida",
];

/// What goes on an ofrenda.
pub const OFFERINGS: &[&str] = &[
    "pan de muerto",
    "mole poblano",
    "tamales de rajas",
    "atole de champurrado",
    "mezcal",
    "tequila",
    "cigarros",
    "café de olla",
    "chocolate caliente",
    "calaverita de azúcar",
    "fotografía",
    "cempasúchil",
    "copal",
    "agua y sal",
    "juguete",
    "dulce de calabaza",
    "tejocotes",
    "caña de azúcar",
    "mandarinas",
    "papel picado",
    "veladoras",
    "una carta",
    "guitarra",
    "sombrero",
    "rebozo",
    "pulque",
    "elote",
    "pozole",
    "enchiladas",
    "cerveza fría",
    "tlacoyos",
    "chongos zamoranos",
];

const TOWNS: &[&str] = &[
    "Mixquic",
    "Janitzio",
    "Pátzcuaro",
    "Tzintzuntzan",
    "Xoxocotlán",
    "Cuilápam",
    "Ocotepec",
    "Oaxaca",
    "Mérida",
    "Xochimilco",
    "Coyoacán",
    "Tlalpan",
    "Morelia",
    "Aguascalientes",
];

/// File names for transfers and trees.
pub const FILES: &[&str] = &[
    "pan_de_muerto.tar.gz",
    "ofrenda_completa.zip",
    "cempasuchil_2026.csv",
    "fotos_de_la_abuela.tar.xz",
    "recetario_familiar.pdf",
    "calaveritas_literarias.txt",
    "veladoras.db",
    "camino_de_petalos.gpx",
    "mole_poblano.sql",
    "playlist_boleros.m3u",
    "retratos_del_altar.zip",
    "cartas_al_mictlan.enc",
    "lista_de_invitados.csv",
    "xolo_gps_tracks.json",
    "papel_picado_hd.svg",
    "copal_humo.wav",
    "mariachi_en_vivo.flac",
    "tamales_para_todos.iso",
    "recuerdos.tar.zst",
    "nombres_del_altar.db",
    "historias_del_abuelo.mp3",
    "alebrijes.blend",
    "mapa_del_mictlan.kml",
    "sal_y_agua.cfg",
];

/// Names of directories for `tree`.
pub const DIRS: &[&str] = &[
    "ofrenda",
    "altar",
    "nivel-1",
    "nivel-3",
    "nivel-7",
    "nivel-9",
    "panteon",
    "recuerdos",
    "fotos",
    "recetas",
    "cartas",
    "veladoras",
    "cempasuchil",
    "xolos",
    "papel-picado",
    "mariachis",
    "calaveritas",
    "pan",
    "copal",
    "mictlan",
];

/// Process names for `htop` and the logs.
pub const DAEMONS: &[&str] = &[
    "xolo-guiad",
    "copal-smoked",
    "veladorad",
    "ofrendad",
    "catrinad",
    "cempasuchild",
    "alebrijed",
    "mariachid",
    "pan-hornod",
    "calaverad",
    "apanohuayad",
    "petalod",
    "mictlan-gwd",
    "papel-picadod",
    "tamalerod",
    "animad",
    "panteond",
    "nichod",
    "monarcad",
    "sahumerio",
];

/// Hosts for the log: where the souls are served from.
pub const HOSTS: &[&str] = &[
    "panteon", "ofrenda", "altar", "mictlan", "xolo", "mixquic", "janitzio", "oaxaca", "nicho",
    "copal",
];

/// Errno values the Mictlán adds: (name, number, message).
pub const ERRNOS: &[(&str, u8, &str)] = &[
    ("ENOPAN", 201, "Se acabó el pan de muerto"),
    ("EVELA", 202, "Veladora apagada por el viento"),
    ("ENOTUHORA", 203, "Todavía no te toca"),
    ("EXOLO", 204, "El xolo no responde (está dormido)"),
    ("ECEMPAS", 205, "Pétalos insuficientes en el camino"),
    ("EMICTLAN", 206, "Nivel 7 del Mictlán saturado"),
    ("EOFRENDA", 207, "Ofrenda incompleta: falta la sal"),
    ("ENOFOTO", 208, "Retrato no encontrado en el altar"),
    ("ECOPAL", 209, "Demasiado humo de copal"),
    ("EANIMA", 210, "Ánima sin conexión"),
    ("ECATRINA", 211, "La Catrina está en una reunión"),
    ("EPAPEL", 212, "Papel picado atascado"),
    ("ECALAVERA", 213, "Calavera sin nombre en la frente"),
    ("EMEZCAL", 214, "Botella vacía (alguien se adelantó)"),
    ("EABUELA", 215, "La abuela dice que comas algo"),
    ("ENOSTALGIA", 216, "Desbordamiento de nostalgia"),
];

/// The tape along the bottom: headlines of the Mictlán.
const HEADLINES: &[&str] = &[
    "MICTLÁN GATEWAY ONLINE",
    "MICTLÁN GATEWAY ONLINE",
    "UPLOADING PAN_DE_MUERTO.TAR.GZ",
    "ACCESO AL MICTLÁN DENEGADO",
    "FELIZ DÍA DE MUERTOS",
    "OFRENDA SYNC COMPLETE",
    "RASTREANDO ALMAS EN TRÁNSITO: {n}",
    "SOULS IN TRANSIT: {n}",
    "XOLO ONLINE: RÍO APANOHUAYA CRUZABLE",
    "CAMINO DE CEMPASÚCHIL: 100% DISPONIBLE",
    "VELADORAS ENCENDIDAS: {n}",
    "COPAL FIREWALL ACTIVE",
    "MARIGOLD PETALS: {n} AND COUNTING",
    "PUERTA DEL MICTLÁN: ABIERTA",
    "BIENVENIDOS A CASA",
    "NO ES TU HORA: ACCESO DENEGADO",
    "PAN DE MUERTO: {p}% HORNEADO",
    "LA CATRINA HA ENTRADO AL CANAL",
    "ALTAR DE {s}: ACTUALIZADO",
    "SAL, AGUA, PAN Y VELAS: TODO EN ORDEN",
];

/// The share of lines that are holiday ones: most already on the first day,
/// nearly all at the height of the holiday.
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

/// Appends `s` with its `*` turned into `o` or `a`: "cuántic*" is
/// "cuántico" for a masculine noun, "cuántica" for a feminine one.
fn agree(s: &str, fem: bool, out: &mut String) {
    for ch in s.chars() {
        if ch == '*' {
            out.push(if fem { 'a' } else { 'o' });
        } else {
            out.push(ch);
        }
    }
}

// ---------------------------------------------------------------- Spanish

/// Nouns: (word, feminine). Technobabble, dev slang and the holiday's things.
const NOUNS_ES: &[(&str, bool)] = &[
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
    ("núcleo", false),
    ("procesador", false),
    ("sistema de archivos", false),
    ("gestor de arranque", false),
    ("planificador", false),
    ("puerto serie", false),
    ("backend", false),
    ("dashboard", false),
    ("wifi", false),
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
    ("fuente de poder", true),
    ("copia de seguridad", true),
    ("cadena de bloques", true),
    ("máquina virtual", true),
    ("consola", true),
    ("bitácora", true),
    ("lista de control de acceso", true),
    ("granja de servidores", true),
    ("conexión", true),
    ("arquitectura", true),
    ("API", true),
    ("red mesh", true),
    ("memoria RAM", true),
    ("rama principal", true),
    // The holiday's.
    ("pan de muerto", false),
    ("cempasúchil", false),
    ("copal", false),
    ("altar", false),
    ("panteón", false),
    ("xoloitzcuintle", false),
    ("alebrije", false),
    ("papel picado", false),
    ("mole", false),
    ("tamal", false),
    ("atole", false),
    ("mezcal", false),
    ("camino de pétalos", false),
    ("río Apanohuaya", false),
    ("inframundo", false),
    ("retrato", false),
    ("mariachi", false),
    ("sahumerio", false),
    ("cráneo de azúcar", false),
    ("puente de cempasúchil", false),
    ("nicho", false),
    ("ofrenda", true),
    ("veladora", true),
    ("calavera", true),
    ("catrina", true),
    ("llama", true),
    ("flor de cempasúchil", true),
    ("calaverita de azúcar", true),
    ("tumba", true),
    ("cruz", true),
    ("fotografía", true),
    ("procesión", true),
    ("mariposa monarca", true),
    ("canoa de Janitzio", true),
    ("lápida", true),
    ("corona de flores", true),
    ("tamalada", true),
    ("puerta del Mictlán", true),
    ("fila de almas", true),
    ("lista de invitados", true),
    ("escalinata del Mictlán", true),
    ("orquesta de mariachis", true),
];

/// Adjectives; a `*` is the `o` or `a` that agrees with the noun.
const ADJ_ES: &[&str] = &[
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
    "inmortal",
    "ancestral",
    "espectral",
    "sagrad*",
    "celestial",
    "etére*",
    "dorad*",
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
    "de obsidiana",
    "de nueve niveles",
    "de cempasúchil",
    "de la abuela",
    "artesanal",
    "orgánic*",
    "sin gluten",
    "de temporada",
    "a prueba de fantasmas",
    "certificad* por la Catrina",
    "tamaño familiar",
    "con denominación de origen",
    "hech* a mano",
    "bendecid*",
    "de ultratumba",
    "mágic*",
    "en la nube",
    "open source",
    "legacy",
    "zero-day",
];

/// Things that hang off a noun: "la ofrenda cuántica del Mictlán".
const OF_ES: &[&str] = &[
    "del Mictlán",
    "de la abuela",
    "del panteón",
    "del inframundo",
    "de cempasúchil",
    "de obsidiana",
    "de las ánimas",
    "de los difuntos",
    "de la nube",
    "del más allá",
    "de ultratumba",
    "de papel picado",
    "de copal",
    "de la casa",
    "del cielo",
    "de Janitzio",
    "de Mixquic",
    "del tío Memo",
    "de la Catrina",
    "del xolo",
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
const VERBS_ES: &[(&str, &str, &str)] = &[
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
    ("descomprimir", "descomprimiendo", "descomprimid"),
    ("subir", "subiendo", "subid"),
    ("descargar", "descargando", "descargad"),
    ("respaldar", "respaldando", "respaldad"),
    ("clonar", "clonando", "clonad"),
    ("parchear", "parcheando", "parchead"),
    ("escanear", "escaneando", "escanead"),
    ("indexar", "indexando", "indexad"),
    ("canalizar", "canalizando", "canalizad"),
    ("hornear", "horneando", "hornead"),
    ("ofrendar", "ofrendando", "ofrendad"),
    ("adornar", "adornando", "adornad"),
    ("cruzar", "cruzando", "cruzad"),
    ("guiar", "guiando", "guiad"),
    ("invocar", "invocando", "invocad"),
    ("liberar", "liberando", "liberad"),
    ("autenticar", "autenticando", "autenticad"),
    ("verificar", "verificando", "verificad"),
    ("optimizar", "optimizando", "optimizad"),
    ("reconstruir", "reconstruyendo", "reconstruid"),
    ("restaurar", "restaurando", "restaurad"),
    ("monitorear", "monitoreando", "monitoread"),
    ("transmitir", "transmitiendo", "transmitid"),
    ("interceptar", "interceptando", "interceptad"),
    ("inyectar", "inyectando", "inyectad"),
    ("ofuscar", "ofuscando", "ofuscad"),
    ("enrutar", "enrutando", "enrutad"),
    ("traducir", "traduciendo", "traducid"),
    ("amplificar", "amplificando", "amplificad"),
    ("modular", "modulando", "modulad"),
    ("encriptar", "encriptando", "encriptad"),
    ("desencriptar", "desencriptando", "desencriptad"),
    ("formatear", "formateando", "formatead"),
    ("resetear", "reseteando", "resetead"),
    ("debuggear", "debuggeando", "debuggead"),
    ("deployear", "deployeando", "deployead"),
    ("forkear", "forkeando", "forkead"),
    ("commitear", "commiteando", "commitead"),
    ("pushear", "pusheando", "pushead"),
    ("scrapear", "scrapeando", "scrapead"),
    ("crackear", "crackeando", "crackead"),
    ("pingear", "pingueando", "pinguead"),
    ("tunelear", "tuneleando", "tunelead"),
    ("virtualizar", "virtualizando", "virtualizad"),
    ("balancear", "balanceando", "balancead"),
    ("sahumar", "sahumando", "sahumad"),
    ("espolvorear", "espolvoreando", "espolvoread"),
    ("recortar", "recortando", "recortad"),
    ("decorar", "decorando", "decorad"),
    ("despertar", "despertando", "despertad"),
    ("recordar", "recordando", "recordad"),
    ("honrar", "honrando", "honrad"),
];

/// How a noun can be: invariable or with a `*` for its gender.
const STATES_ES: &[&str] = &[
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
    "en llamas (metafóricamente)",
    "consciente de sí mism*",
    "negándose a cooperar",
    "negociando consigo mism*",
    "inusualmente silencios*",
    "cargando (desde 1998)",
    "en punto muerto",
    "en interbloqueo",
    "sin memoria (se la dejó a la abuela)",
    "esperando quórum",
    "pendiente de reinicio",
    "a punto de explotar",
    "fuera de las tolerancias de diseño",
    "hablando con la subred equivocada",
    "intercambiad* en caliente por error",
    "sin punto y coma",
    "minando algo",
    "sin café",
    "llamando a casa",
    "aprendiendo demasiado rápido",
    "olvidándolo todo",
    "dormid*",
    "bailando cumbia",
    "cantando rancheras",
    "pidiendo más pan",
    "extrañando a la abuela",
    "convertid* en calaverita",
    "más viv* que nunca",
    "muert* de risa",
    "descansando en paz (RIP)",
    "en modo ultratumba",
    "de buen humor",
    "oliendo a copal",
    "vestid* de catrina",
    "esperando su turno en el Mictlán",
];

/// Whole failures: "Error al desplegar la ofrenda: {this}".
const ERRORS_ES: &[&str] = &[
    "falla de segmentación en el múltiple de flujo",
    "desbordamiento de búfer en 0xDEADBEEF",
    "el certificado fue revocado por una autoridad superior",
    "no hay ruta al Mictlán",
    "se acabó el pan de muerto",
    "demasiadas almas en la ofrenda",
    "la veladora se apagó (corriente de aire)",
    "volcado de núcleo (la abuela lo vio)",
    "desbordamiento de pila de cempasúchil",
    "pánico del kernel: no se sincroniza",
    "permiso denegado (prueba con sudo, pero con más respeto)",
    "tiempo de espera agotado en el inframundo",
    "excepción de punto flotante",
    "heisenbug observado (ahora sí funciona)",
    "la entropía se agotó",
    "violación de protocolo",
    "el río Apanohuaya está crecido",
    "el xolo no responde",
    "bucle infinito en la fila del panteón",
    "paradoja temporal detectada",
    "referencia nula en el nivel 7 del Mictlán",
    "uso después de liberar (el alma ya se fue)",
    "doble liberación o corrupción",
    "firma inválida: eso no es de la abuela",
    "no hay espacio en el disco (el pan ocupó todo)",
    "nombre ilegible en la ofrenda (¿Chuy o Chucho?)",
    "conexión rechazada por el tío que no cree en esto",
    "clave caducada el 2 de noviembre",
    "demasiados archivos abiertos (todos son recuerdos)",
    "falta la sal en el altar",
    "no se encontró el retrato",
    "la Catrina no está disponible (en reunión)",
    "el copal ahumó el sensor",
    "entropía insuficiente: pon más cempasúchil",
    "error de sintaxis en la calaverita (no rima)",
    "memoria insuficiente (se llenó de fotos)",
    "el puente de pétalos no soporta tanta carga",
    "sin señal en el panteón",
    "token vencido: la abuela lo cambió",
];

const TIMES_ES: &[&str] = &[
    "milisegundos",
    "segundos",
    "microsegundos",
    "ciclos de reloj",
    "latidos",
    "nanosegundos",
    "jiffies",
    "épocas",
    "campanadas",
    "veladoras",
];

/// Measures of the Mictlán in Spanish: "4.2 kilopanes".
const UNITS_ES: &[&str] = &[
    "kilopanes",
    "megatamales",
    "gigapétalos/s",
    "teraveladoras",
    "Mbit/s de cariño",
    "kilomoles",
    "teramariachis",
    "veladoras/s",
    "pétalos/ms",
    "decibeles de cumbia",
    "nanocopales",
    "megaofrendas",
    "gigacalaveras",
    "petabytes de recuerdos",
    "kilocandelas",
    "megaabrazos",
    "años luz de nostalgia",
    "teraflops de cariño",
    "gigavatios",
    "pársecs",
    "milisegundos",
    "THz",
    "GB/s",
    "teslas",
    "qubits",
    "rpm",
];

/// ...and in English: "4.2 petals/s".
const UNITS_EN: &[&str] = &[
    "petals/s",
    "candlepower",
    "teramarigolds",
    "megatamales",
    "kilopans",
    "megahugs",
    "gigacandles",
    "terabytes of memories",
    "megawatts of nostalgia",
    "THz",
    "GB/s",
    "light-years",
    "parsecs",
    "teslas",
    "qubits",
    "rpm",
];

/// "4.2 kilopanes" (or "4.2 petals/s" when not `es`).
fn measure(rng: &mut Rng, es: bool, out: &mut String) {
    use std::fmt::Write;
    let _ = match rng.u8(..3) {
        0 => write!(out, "{}", rng.u32(2..999)),
        1 => write!(out, "{}.{}", rng.u32(1..99), rng.u32(0..9)),
        _ => write!(out, "{}", rng.u32(10..9999)),
    };
    out.push(' ');
    out.push_str(pick(rng, if es { UNITS_ES } else { UNITS_EN }));
}

/// "la ofrenda cuántica del Mictlán"; `det` adds the article. Returns
/// whether the noun is feminine, for what has to agree with it.
fn noun_phrase_es(rng: &mut Rng, level: u8, det: bool, out: &mut String) -> bool {
    let (noun, fem) = pick(rng, NOUNS_ES);
    if det {
        out.push_str(pick(rng, if fem { DET_F } else { DET_M }));
        out.push(' ');
    }
    out.push_str(noun);
    if rng.u8(..10) < 7 {
        out.push(' ');
        agree(pick(rng, ADJ_ES), fem, out);
        if level >= 7 && rng.u8(..10) < 3 {
            out.push_str(" y ");
            agree(pick(rng, ADJ_ES), fem, out);
        }
    }
    if rng.u8(..10) < 3 {
        out.push(' ');
        out.push_str(pick(rng, OF_ES));
    }
    fem
}

/// `prefix` (ending in "de ") and a noun phrase, contracting "de el" into "del".
fn de_np(rng: &mut Rng, level: u8, prefix: &str, out: &mut String) {
    out.push_str(prefix);
    let at = out.len();
    noun_phrase_es(rng, level, true, out);
    if out[at..].starts_with("el ") {
        out.replace_range(at - 3..at + 3, "del ");
    }
}

/// A trailing clause in Spanish: "mediante el X", "a 4 THz", "(código 0x1F)".
fn tail_es(rng: &mut Rng, level: u8, out: &mut String) {
    use std::fmt::Write;
    out.push(' ');
    match rng.u8(..13) {
        0 => {
            out.push_str("con ");
            noun_phrase_es(rng, level, true, out);
        }
        1 => {
            out.push_str("mediante ");
            noun_phrase_es(rng, level, true, out);
        }
        2 => de_np(rng, level, "a través de ", out),
        3 => {
            out.push_str("usando ");
            noun_phrase_es(rng, level, true, out);
        }
        4 => de_np(rng, level, "dentro de ", out),
        5 => {
            out.push_str("contra ");
            noun_phrase_es(rng, level, true, out);
        }
        6 => {
            out.push_str("vía ");
            noun_phrase_es(rng, level, true, out);
        }
        7 => {
            out.push_str("hacia ");
            noun_phrase_es(rng, level, true, out);
        }
        8 => {
            out.push_str("a ");
            measure(rng, true, out);
        }
        9 => {
            let _ = write!(out, "por {} {}", rng.u32(2..9999), pick(rng, TIMES_ES));
        }
        10 => {
            out.push_str("en modo ");
            agree(pick(rng, ADJ_ES), false, out);
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
fn sentence_es(rng: &mut Rng, level: u8, out: &mut String) {
    use std::fmt::Write;
    let from = out.len();
    let tails = |rng: &mut Rng, out: &mut String| {
        for _ in 0..rng.u8(..=(level / 3).min(3)) {
            tail_es(rng, level, out);
        }
    };
    match rng.u8(..12) {
        0..=2 => {
            out.push_str(pick(rng, VERBS_ES).1);
            out.push(' ');
            noun_phrase_es(rng, level, true, out);
            tails(rng, out);
        }
        3 => {
            let _ = write!(out, "{} ", pick(rng, VERBS_ES).1);
            noun_phrase_es(rng, level, true, out);
            let _ = write!(out, " y {} ", pick(rng, VERBS_ES).1);
            noun_phrase_es(rng, level, true, out);
        }
        4 => {
            let _ = write!(out, "error al {} ", pick(rng, VERBS_ES).0);
            noun_phrase_es(rng, level, true, out);
            let _ = write!(out, ": {}", pick(rng, ERRORS_ES));
        }
        5 => {
            let fem = noun_phrase_es(rng, level, true, out);
            let _ = write!(out, " fue {}", pick(rng, VERBS_ES).2);
            out.push(if fem { 'a' } else { 'o' });
            out.push_str(" con éxito (");
            measure(rng, true, out);
            out.push(')');
        }
        6 => {
            out.push_str("advertencia: ");
            let fem = noun_phrase_es(rng, level, true, out);
            out.push_str(" está ");
            agree(pick(rng, STATES_ES), fem, out);
        }
        7 => {
            out.push_str("activando ");
            noun_phrase_es(rng, level, true, out);
            tails(rng, out);
        }
        8 => {
            let fem = noun_phrase_es(rng, level, true, out);
            out.push_str(" está ");
            agree(pick(rng, STATES_ES), fem, out);
            let _ = write!(out, "; {} ", pick(rng, VERBS_ES).1);
            noun_phrase_es(rng, level, true, out);
        }
        9 => {
            let _ = write!(out, "favor de {} ", pick(rng, VERBS_ES).0);
            noun_phrase_es(rng, level, true, out);
            out.push_str(" antes de que ");
            let fem = noun_phrase_es(rng, level, true, out);
            out.push_str(" esté ");
            agree(pick(rng, STATES_ES), fem, out);
        }
        10 => {
            let _ = write!(out, "{} ", pick(rng, VERBS_ES).1);
            noun_phrase_es(rng, level, true, out);
            out.push_str(" a ");
            measure(rng, true, out);
        }
        _ => {
            let _ = write!(out, "no se pudo {} ", pick(rng, VERBS_ES).0);
            noun_phrase_es(rng, level, true, out);
            let _ = write!(
                out,
                " (reintentar en {} {})",
                rng.u32(2..900),
                pick(rng, TIMES_ES)
            );
        }
    }
    capitalize_from(out, from);
}

// ---------------------------------------------------------------- English

const NOUNS_EN: &[&str] = &[
    "ofrenda",
    "marigold relay",
    "copal filter",
    "sugar-skull cache",
    "pan de muerto",
    "papel picado firewall",
    "xolo daemon",
    "Mictlán gateway",
    "soul registry",
    "candle array",
    "petal buffer",
    "altar",
    "ancestor database",
    "calavera compiler",
    "alebrije renderer",
    "cempasúchil cluster",
    "river Apanohuaya",
    "marigold path",
    "ofrenda database",
    "catrina protocol",
    "monarch swarm",
    "copal smoke signal",
    "tamale oven",
    "mariachi uplink",
    "veladora grid",
    "grandma's wifi",
    "skeleton key",
    "nine-level load balancer",
    "memory lane",
    "photo frame buffer",
    "candle scheduler",
    "sugar skull generator",
    "marigold garbage collector",
    "family tree",
    "afterlife API",
    "soul load balancer",
    "ofrenda router",
    "bread oven",
    "petal pipeline",
    "bone-white hash table",
];

const ADJ_EN: &[&str] = &[
    "spectral",
    "ancestral",
    "marigold-scented",
    "sugar-coated",
    "candlelit",
    "ethereal",
    "sacred",
    "golden",
    "eternal",
    "afterlife-grade",
    "family-sized",
    "hand-painted",
    "hand-cut",
    "artisanal",
    "gluten-free",
    "organic",
    "certified",
    "vintage",
    "immortal",
    "ever-burning",
    "joyful",
    "dancing",
    "singing",
    "glowing",
    "cempasúchil-powered",
    "festive",
    "cherished",
    "beloved",
    "multi-generational",
    "grandma-approved",
];

const VERBS_EN: &[(&str, &str, &str)] = &[
    ("light", "lighting", "lit"),
    ("bake", "baking", "baked"),
    ("upload", "uploading", "uploaded"),
    ("guide", "guiding", "guided"),
    ("summon", "summoning", "summoned"),
    ("bless", "blessing", "blessed"),
    ("garland", "garlanding", "garlanded"),
    ("sprinkle", "sprinkling", "sprinkled"),
    ("sync", "syncing", "synced"),
    ("cut", "cutting", "cut"),
    ("decorate", "decorating", "decorated"),
    ("welcome", "welcoming", "welcomed"),
    ("remember", "remembering", "remembered"),
    ("honor", "honoring", "honored"),
    ("serve", "serving", "served"),
    ("route", "routing", "routed"),
    ("deploy", "deploying", "deployed"),
    ("back up", "backing up", "backed up"),
    ("decrypt", "decrypting", "decrypted"),
    ("fork", "forking", "forked"),
];

const STATES_EN: &[&str] = &[
    "dancing",
    "humming a bolero",
    "out of marigolds",
    "extra crispy",
    "glowing softly",
    "waiting for its turn at Mictlán",
    "still warm from the oven",
    "napping (it is a dog)",
    "smelling of copal",
    "full of nostalgia",
    "refusing to let the bread go",
    "overflowing with petals",
    "more alive than ever",
    "dying of laughter",
    "resting in peace (RIP)",
    "sorely missed",
    "under heavy tamale load",
    "dressed as La Catrina",
];

const ERRORS_EN: &[&str] = &[
    "marigold supply exhausted",
    "pan de muerto not found (404)",
    "soul not found in registry",
    "candle flame below minimum",
    "copal smoke overflow",
    "no route to Mictlán",
    "xolo not responding (napping)",
    "river Apanohuaya flooded",
    "ofrenda is missing the salt",
    "sugar-skull checksum mismatch",
    "permission denied: not your time yet",
    "name on the altar unreadable",
    "papel picado jam",
    "grandma's password expired",
    "too many souls on level 7",
    "La Catrina is in a meeting",
    "tequila bottle already empty",
    "tamales exceed quota",
    "photo frame buffer overflow",
    "unexpected end of the festival",
    "not enough petals on the path",
];

/// "the ancestral marigold relay": holiday nouns among the ordinary ones.
fn noun_phrase_en(rng: &mut Rng, level: u8, out: &mut String) {
    const DETS: &[&str] = &[
        "the", "the", "the", "the", "our", "your", "their", "each", "every", "that", "this",
    ];
    out.push_str(pick(rng, DETS));
    out.push(' ');
    out.push_str(if rng.u8(..3) == 0 {
        pick(rng, babble::ADJ)
    } else {
        pick(rng, ADJ_EN)
    });
    out.push(' ');
    if level >= 7 && rng.u8(..10) < 3 {
        out.push_str(pick(rng, ADJ_EN));
        out.push(' ');
    }
    if rng.u8(..3) == 0 {
        out.push_str(pick(rng, babble::MODS));
        out.push(' ');
        out.push_str(pick(rng, babble::NOUNS));
    } else {
        out.push_str(pick(rng, NOUNS_EN));
    }
}

fn tail_en(rng: &mut Rng, level: u8, out: &mut String) {
    use std::fmt::Write;
    out.push(' ');
    match rng.u8(..10) {
        0 => {
            let _ = write!(
                out,
                "with {} {} flags",
                pick(rng, ADJ_EN),
                pick(rng, babble::MODS)
            );
        }
        1 => {
            out.push_str("through ");
            noun_phrase_en(rng, level, out);
        }
        2 => {
            out.push_str("via ");
            noun_phrase_en(rng, level, out);
        }
        3 => {
            out.push_str("across ");
            noun_phrase_en(rng, level, out);
        }
        4 => {
            out.push_str("at ");
            measure(rng, false, out);
        }
        5 => {
            let _ = write!(out, "for {} {}", rng.u32(2..9999), pick(rng, babble::TIMES));
        }
        6 => {
            let _ = write!(out, "in {} mode", pick(rng, ADJ_EN));
        }
        7 => {
            let _ = write!(out, "(code 0x{:04X})", rng.u16(..));
        }
        8 => {
            let _ = write!(out, "on port {}", rng.u16(1..65535));
        }
        _ => {
            out.push_str("for ");
            out.push_str(pick(rng, SOULS));
        }
    }
}

/// A sentence built from the English tables.
fn sentence_en(rng: &mut Rng, level: u8, out: &mut String) {
    use std::fmt::Write;
    let from = out.len();
    let tails = |rng: &mut Rng, out: &mut String| {
        for _ in 0..rng.u8(..=(level / 3).min(3)) {
            tail_en(rng, level, out);
        }
    };
    let verb = |rng: &mut Rng| {
        if rng.bool() {
            pick(rng, VERBS_EN)
        } else {
            pick(rng, babble::VERBS)
        }
    };
    match rng.u8(..10) {
        0..=2 => {
            out.push_str(verb(rng).1);
            out.push(' ');
            noun_phrase_en(rng, level, out);
            tails(rng, out);
        }
        3 => {
            let _ = write!(out, "failed to {} ", verb(rng).0);
            noun_phrase_en(rng, level, out);
            let _ = write!(out, ": {}", pick(rng, ERRORS_EN));
        }
        4 => {
            noun_phrase_en(rng, level, out);
            let _ = write!(out, " was {} successfully (", verb(rng).2);
            measure(rng, false, out);
            out.push(')');
        }
        5 => {
            out.push_str("warning: ");
            noun_phrase_en(rng, level, out);
            let _ = write!(out, " is {}", pick(rng, STATES_EN));
        }
        6 => {
            noun_phrase_en(rng, level, out);
            let _ = write!(out, " is {}; {} ", pick(rng, STATES_EN), verb(rng).1);
            noun_phrase_en(rng, level, out);
        }
        7 => {
            let _ = write!(out, "please {} ", verb(rng).0);
            noun_phrase_en(rng, level, out);
            out.push_str(" before ");
            noun_phrase_en(rng, level, out);
            let _ = write!(out, " is {}", pick(rng, STATES_EN));
        }
        8 => {
            let _ = write!(out, "unable to {} ", verb(rng).0);
            noun_phrase_en(rng, level, out);
            let _ = write!(
                out,
                " (retry in {} {})",
                rng.u32(2..900),
                pick(rng, babble::TIMES)
            );
        }
        _ => {
            out.push_str("engaging ");
            noun_phrase_en(rng, level, out);
            tails(rng, out);
        }
    }
    capitalize_from(out, from);
}

// ---------------------------------------------------------------- canned lines

/// Hand-written lines in Spanish. The blanks: `{n}` a number, `{l}` a level
/// of the Mictlán (1 to 9), `{s}` a soul, `{u}` a user name, `{o}` an
/// offering, `{t}` a town, `{p}` a percentage, `{x}` a hex word.
const CANNED_ES: &[&str] = &[
    "Subiendo ofrenda.tar.gz al nivel {l} del Mictlán ({n} MB/s)",
    "Descargando el recetario de la abuela ({n} MB de puro amor)",
    "Aplicando parche de seguridad a la tumba de {s}",
    "Indexando {n} fotos de familia en la nube (no le digan a {s})",
    "Reiniciando el router de {s}: sigue sin wifi en el más allá",
    "Restableciendo la contraseña de {s}: era el nombre del perro",
    "Cargando camino de pétalos... {p}%",
    "Autenticando a {s} con huella digital de cempasúchil",
    "Copia de seguridad del altar completa (menos el pan, se lo comió {s})",
    "El xolo cruzó el río Apanohuaya con {s} a cuestas",
    "Horneando {n} piezas de pan de muerto en el clúster",
    "Banda ancha del más allá: {n} Mbit/s (el cielo tiene fibra óptica)",
    "La señal llega mejor desde el panteón de {t}",
    "Descifrando la receta del mole de {s}: faltan {l} ingredientes",
    "Ofrenda de {o} lista para desplegar en producción",
    "Probando la ofrenda de {s} en staging antes de producción",
    "Prueba de humo (de copal): aprobada",
    "TODO: encender las veladoras antes de que llegue {s}",
    "$ git commit -m \"agrego cempasúchil al camino\"",
    "$ sudo apt install pan-de-muerto",
    "$ systemctl start xolo-guia.service",
    "$ ssh {u}@mictlan.mx -p {l}",
    "$ curl -s https://mictlan.mx/api/v{l}/almas | jq '.[0]'",
    "$ chmod +x ofrenda.sh && ./ofrenda.sh",
    "$ grep -ri \"abuela\" /var/recuerdos | wc -l # {n}",
    "$ rm -rf /tristeza # permiso denegado, qué bueno",
    "$ kill -9 olvido: operación no permitida mientras alguien te recuerde",
    "$ ping mictlan.mx: {l} saltos, 0% de pérdida, 0 ms (el más allá no tiene latencia)",
    "traceroute al Mictlán: {l} saltos, el último es un xolo",
    "Compilando recuerdos de {s}: 0 errores, {n} advertencias (todas de cariño)",
    "Migrando las anécdotas de {s} a la base de datos ofrenda_db",
    "Respaldando las historias del abuelo (se repiten, pero se respaldan)",
    "Actualizando el altar de {s}: veladoras +{l}, flores +{n}",
    "Instalando el parche de la Catrina v{l}.{l}: ahora con sombrero de plumas",
    "El cortafuegos de copal bloqueó {n} intentos de acceso de vivos curiosos",
    "Verificando que haya sal, agua y pan en el altar de {s}",
    "Canalizando {n} MW de nostalgia por el puerto {n}",
    "Memoria caché de la abuela: 100% de aciertos (se acuerda de todo)",
    "Sincronizando calaveritas literarias con el servidor de {t}",
    "Rimando calavera.rs: error de sintaxis en el verso {l}",
    "Excepción atrapada: ¡Ay, caray! (NullPointerException en el nivel {l})",
    "Depurando el altar de {s}: 1 bug encontrado (una cucaracha, no pasa nada)",
    "La Catrina aprobó el pull request #{n} con un emoji de calavera",
    "Fusionando ramas: la rama de {s} y la rama de los primos",
    "Veladoras: {n} encendidas, 0 apagadas por el viento",
    "Latencia al nivel {l} del Mictlán: {n} ms (tráfico de Día de Muertos)",
    "Escaneando puertos del Mictlán: nueve abiertos, ninguno cerrado (hoy se puede pasar)",
    "Subiendo la foto de {s} al altar: formato aceptado (.jpg, .png, .sepia)",
    "Firmando el certificado de {s} con la llave del Mictlán: válido hasta el 2 de noviembre",
    "El pan de muerto aún no termina de hornearse: {p}% listo",
    "Ancho de banda reservado para los angelitos: prioridad alta",
    "Pétalos de cempasúchil en el búfer: {n} (el camino sigue abierto)",
    "Humo de copal dentro de los parámetros normales: {n} ppm",
    "Conectando con el Mictlán vía satélite SATMEX-{l}",
    "Moviendo la carpeta 'Recuerdos' de /vivos a /ofrenda",
    "Cifrando cartas al más allá con AES-256-GCM-mole",
    "Recibiendo {n} paquetes de nostalgia (sin pérdidas)",
    "Buscando a {s} en el directorio de almas: encontrado en el nivel {l}",
    "{s} no confirmó asistencia; se le guardó su lugar en la mesa",
    "Descargando el DLC 'Pan de muerto: edición familiar' ({l}.{l} GB)",
    "Habilitando modo oscuro: ya era de noche en el panteón",
    "Sirviendo {o} a {s}: entregado, aún caliente",
    "Alerta: {s} llegó antes de tiempo; hay que esconder el {o}",
    "Cargando playlist de {s}: {n} boleros, 0 anuncios",
    "Aplicando el filtro sepia a las fotos del altar: {p}% procesado",
    "Cálculo de la distancia entre {t} y el Mictlán: {n} leguas, 1 xolo",
    "Todos los tamales fueron desplegados sin downtime",
    "Reintentando conexión con el más allá (intento {n} de ∞)",
    "Comprimiendo {n} abrazos en un solo .tar.gz",
    "Las flores de cempasúchil crecen en O(n) y huelen en O(1)",
    "Prueba unitaria del altar: {n} aprobadas, 1 omitida (el tequila)",
    "Renderizando papel picado a 60 cuadros por segundo",
    "Agregando a {s} a la lista de invitados: la mesa ya está servida",
    "Ajustando la llama de la veladora {n} para que no dé sombra a la foto",
];

/// Hand-written lines in English, with the same blanks.
const CANNED_EN: &[&str] = &[
    "Uploading pan_de_muerto.tar.gz to level {l} of Mictlán ({n} MB/s)",
    "Bypassing the Apanohuaya river firewall with one very good dog",
    "Marigold petals routed through the quantum firewall",
    "Calibrating the copal smoke detector ({n} ppm)",
    "Sugar skull checksum mismatch: missing sugar",
    "{s}'s tamales exceed the bandwidth quota",
    "DNS: abuela.mictlan.mx resolves to home (127.0.0.1)",
    "Brute-forcing the nine gates of Mictlán (politely)",
    "Ping {s}: 0% packet loss, {l} hops, the last one is a dog",
    "Marigold supply chain stable at {n} petals/s",
    "Ofrenda cache hit ratio 99% (the cache is a pan de muerto)",
    "Checking the altar for salt, water and candles",
    "$ systemctl status xolo: active (running), guiding {n} souls",
    "Rendering papel picado at 60 fps",
    "Copal incense levels nominal",
    "Compiling grandma's mole recipe: missing dependency 'patience'",
    "Installing sombrero-for-catrina v{l}.{l}.{l}",
    "The candle array lit {n} wicks in {n} ms",
    "Syncing {s}'s favorite songs to the altar playlist",
    "Mariachi uplink established: {n} kbaud of pure feeling",
    "Decrypting {s}'s letter: it says 'eat something'",
    "Handshake with the xolo OK: tail wagging at {n} Hz",
    "Granting {s} visitor access until November 3",
    "Access to Mictlán level {l} DENIED: not your time yet",
    "Soul {n} reached home. Tamales detected. Deploying hugs",
    "Scanning the altar: {n} sugar skulls, {l} candles, 1 very large pan",
    "Rotating the marigold keys (every 365 days, by tradition)",
    "Garbage-collecting sadness: 0 bytes freed, memories retained",
    "Remembering {s}: heap size 2^64, never freed",
    "Latency to the afterlife: 0 ms (they were already here)",
    "Cache invalidation is hard, but forgetting {s} is harder",
    "Papel picado jam on printer {n} (again)",
    "Rhyming skull poems: {n} verses compiled, 0 syntax errors",
    "Unit tests for the altar: {n} passed, 1 skipped (the tequila)",
    "Marigold path width: {l} meters, lit by {n} candles",
    "Warning: {s} is already in the kitchen; hide the pan",
    "Request timed out: the dead are busy dancing",
    "$ git commit -m 'add more marigolds' (again)",
    "SELECT name FROM altar WHERE loved = TRUE; -- {n} rows",
    "Backing up memories of {s}: 100% complete, 0 bytes lost",
    "Loading afterlife.so: symbol 'peace_of_mind' found",
    "Opening port {l} for souls only (tamales exempt)",
    "Mictlán gateway uptime: {n} days since the last November 2",
    "Pan de muerto thread pool saturated: {n} workers eating",
    "Encrypting the offering with AES-256-mole",
    "Smoke signal (copal) received: {s} says hola",
    "Offering of {o} delivered to {s}: ACK",
    "Welcome home, {s}. The wifi password is the same",
    "The dead are online: {n} souls, {l} of them dancing",
];

/// Plain lines for `babble = 0`: the holiday's own, still busy-looking. The
/// first `MUNDANE_ES` are Spanish.
const MUNDANE_ES: usize = 13;
const MUNDANE_MX: &[&str] = &[
    "ofrenda recibida",
    "veladora encendida",
    "sesión abierta para el usuario abuela",
    "pan de muerto servido",
    "camino de pétalos actualizado",
    "xolo en línea",
    "latido ok (el de la abuela)",
    "copal encendido",
    "altar sincronizado",
    "conexión cerrada por el host remoto",
    "puerta del Mictlán abierta",
    "alma llegó a casa",
    "limpieza periódica iniciada",
    "marigold path updated",
    "candles lit, all quiet",
    "souls arriving on schedule",
];

/// Expands the blanks of a canned line into `out`.
fn expand(rng: &mut Rng, tpl: &str, out: &mut String) {
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
            's' => out.push_str(pick(rng, SOULS)),
            'u' => out.push_str(pick(rng, USERS)),
            'o' => out.push_str(pick(rng, OFFERINGS)),
            't' => out.push_str(pick(rng, TOWNS)),
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
pub fn sentence(rng: &mut Rng, level: u8, out: &mut String) -> bool {
    let from = out.len();
    let es = if level == 0 {
        let i = rng.usize(..MUNDANE_MX.len());
        out.push_str(MUNDANE_MX[i]);
        i < MUNDANE_ES
    } else {
        match rng.u8(..10) {
            0..=1 => {
                let tpl = pick(rng, CANNED_ES);
                expand(rng, tpl, out);
                true
            }
            2 => {
                let tpl = pick(rng, CANNED_EN);
                expand(rng, tpl, out);
                false
            }
            3..=6 => {
                sentence_es(rng, level.min(10), out);
                true
            }
            _ => {
                sentence_en(rng, level.min(10), out);
                false
            }
        }
    };
    capitalize_from(out, from);
    es
}

/// A sentence of the Mictlán or an ordinary one, in the proportion
/// `share(fiesta_level)`. Returns whether it is Spanish.
pub fn mixed(rng: &mut Rng, level: u8, fiesta_level: f32, out: &mut String) -> bool {
    if rng.f32() < share(fiesta_level) {
        sentence(rng, level, out)
    } else {
        babble::sentence(rng, level, out);
        false
    }
}

/// A failure for a kernel panic: "el río Apanohuaya está crecido".
pub fn error(rng: &mut Rng, out: &mut String) {
    out.push_str(if rng.u8(..3) > 0 {
        pick(rng, ERRORS_ES)
    } else {
        pick(rng, ERRORS_EN)
    });
}

/// A short lower-case name for a pane title: "ofrenda cuántica".
pub fn title(rng: &mut Rng, out: &mut String) {
    let from = out.len();
    if rng.u8(..3) > 0 {
        let (noun, fem) = pick(rng, NOUNS_ES);
        out.push_str(noun);
        out.push(' ');
        agree(pick(rng, ADJ_ES), fem, out);
    } else {
        out.push_str(pick(rng, ADJ_EN));
        out.push(' ');
        out.push_str(pick(rng, NOUNS_EN));
    }
    let lower = out[from..].to_lowercase();
    out.truncate(from);
    out.push_str(&lower);
}

/// A title of the Mictlán or an ordinary one.
pub fn mixed_title(rng: &mut Rng, fiesta_level: f32, out: &mut String) {
    if rng.f32() < share(fiesta_level) {
        title(rng, out);
    } else {
        babble::title(rng, out);
    }
}

/// An UPPER-CASE progress label ("SINCRONIZANDO LA OFRENDA CUÁNTICA") or a
/// transfer ("uploading pan_de_muerto.tar.gz").
pub fn progress_label(rng: &mut Rng, level: u8, out: &mut String) {
    use std::fmt::Write;
    if rng.u8(..5) < 2 {
        const VERBS: &[&str] = &[
            "uploading",
            "uploading",
            "downloading",
            "subiendo",
            "descargando",
            "respaldando",
            "transfiriendo",
            "syncing",
            "restaurando",
        ];
        // The brief's own transfer comes up often.
        let (verb, file) = if rng.u8(..4) == 0 {
            ("uploading", FILES[0])
        } else {
            (pick(rng, VERBS), pick(rng, FILES))
        };
        let _ = write!(out, "{verb} {file} ({}.{} GB)", rng.u8(0..9), rng.u8(0..10));
        return;
    }
    let from = out.len();
    if rng.u8(..4) == 0 {
        out.push_str(pick(rng, VERBS_EN).1);
        out.push(' ');
        noun_phrase_en(rng, level.min(4), out);
    } else {
        out.push_str(pick(rng, VERBS_ES).1);
        out.push(' ');
        noun_phrase_es(rng, level.min(4), true, out);
    }
    let up = out[from..].to_uppercase();
    out.truncate(from);
    out.push_str(&up);
}

/// A headline for the tape along the bottom (already UPPER-CASE).
pub fn headline(rng: &mut Rng, out: &mut String) {
    let from = out.len();
    let tpl = pick(rng, HEADLINES);
    expand(rng, tpl, out);
    let up = out[from..].to_uppercase();
    out.truncate(from);
    out.push_str(&up);
}

/// "ofrenda_cuantica", "token-altar": an identifier in snake or kebab case.
pub fn ident(rng: &mut Rng, sep: char, out: &mut String) {
    const HOLY: &[&str] = &[
        "ofrenda",
        "altar",
        "veladora",
        "calavera",
        "catrina",
        "copal",
        "cempasuchil",
        "xolo",
        "alebrije",
        "mictlan",
        "panteon",
        "pan_muerto",
        "papel_picado",
        "mole",
        "tamal",
        "atole",
        "alma",
        "camino",
        "puerta",
        "rio",
        "flor",
        "petalo",
        "tumba",
        "cruz",
        "nicho",
        "retrato",
        "mariachi",
        "monarca",
        "abuela",
        "calaca",
    ];
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
        "guia",
        "gateway",
        "daemon",
        "handler",
        "parser",
        "relay",
    ];
    let word = |w: &str, out: &mut String| {
        for ch in w.chars() {
            out.push(if ch == '_' { sep } else { ch });
        }
    };
    let (a, b) = (pick(rng, HOLY), pick(rng, TECH));
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

/// A password the way a family would choose it: "C3mp4suchil!1102".
pub fn password(rng: &mut Rng, out: &mut String) {
    const WORDS: &[&str] = &[
        "cempasuchil",
        "pandemuerto",
        "calaverita",
        "catrina",
        "mictlan",
        "ofrenda",
        "xoloitzcuintle",
        "veladora",
        "alebrije",
        "copal",
        "abuelita",
        "tamalada",
        "mezcalito",
        "panteon",
        "mariachi",
        "tequila",
        "marigold",
        "sugarskull",
        "nochebuena",
        "calaca",
    ];
    let word = pick(rng, WORDS);
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

/// Number of hand-written fragments (for the tests).
#[cfg(test)]
pub fn fragment_count() -> usize {
    SOULS.len()
        + ANGELITOS.len()
        + TOYS.len()
        + USERS.len()
        + OFFERINGS.len()
        + FILES.len()
        + DIRS.len()
        + DAEMONS.len()
        + ERRNOS.len()
        + HEADLINES.len()
        + NOUNS_ES.len()
        + ADJ_ES.len()
        + OF_ES.len()
        + VERBS_ES.len() * 3
        + STATES_ES.len()
        + ERRORS_ES.len()
        + UNITS_ES.len()
        + UNITS_EN.len()
        + NOUNS_EN.len()
        + ADJ_EN.len()
        + VERBS_EN.len() * 3
        + STATES_EN.len()
        + ERRORS_EN.len()
        + CANNED_ES.len()
        + CANNED_EN.len()
        + MUNDANE_MX.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn plenty_of_text_in_both_languages() {
        assert!(fragment_count() >= 800, "{}", fragment_count());
        let mut rng = Rng::with_seed(9);
        let mut set = HashSet::new();
        let mut s = String::new();
        let (mut es, mut en) = (0, 0);
        for _ in 0..10_000 {
            s.clear();
            sentence(&mut rng, 7, &mut s);
            assert!(
                !s.is_empty() && !s.contains("  ") && !s.contains('{') && !s.contains('*'),
                "{s:?}"
            );
            if s.chars().any(|c| "áéíóúñ¡¿".contains(c)) || s.contains(" el ") || s.contains(" la ")
            {
                es += 1;
            } else {
                en += 1;
            }
            set.insert(s.clone());
        }
        assert!(set.len() >= 8_500, "{}", set.len());
        assert!(es > 2_500 && en > 1_500, "{es} {en}");
    }

    #[test]
    fn spanish_agrees() {
        let mut rng = Rng::with_seed(3);
        let mut s = String::new();
        for _ in 0..3_000 {
            s.clear();
            sentence_es(&mut rng, 10, &mut s);
            assert!(!s.contains('*') && !s.contains("  "), "{s}");
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
    fn the_brief_s_lines_are_there() {
        let mut have = [false; 3];
        let mut rng = Rng::with_seed(1);
        let mut s = String::new();
        for _ in 0..400 {
            s.clear();
            headline(&mut rng, &mut s);
            have[0] |= s == "MICTLÁN GATEWAY ONLINE";
            have[1] |= s == "UPLOADING PAN_DE_MUERTO.TAR.GZ";
            have[2] |= s == "ACCESO AL MICTLÁN DENEGADO";
        }
        assert!(have.iter().all(|&h| h), "{have:?}");
        assert!(FILES.contains(&"pan_de_muerto.tar.gz"));
    }

    #[test]
    fn share_grows_with_the_holiday() {
        assert!(share(0.25) > 0.5 && share(0.25) < share(0.6));
        assert!(share(0.6) < share(1.0) && share(1.0) <= 1.0);
    }

    #[test]
    fn titles_labels_idents_and_passwords_are_clean() {
        let mut rng = Rng::with_seed(4);
        let mut s = String::new();
        for _ in 0..500 {
            s.clear();
            title(&mut rng, &mut s);
            assert!(!s.is_empty() && !s.contains('*') && s == s.to_lowercase());
            s.clear();
            progress_label(&mut rng, 8, &mut s);
            assert!(!s.is_empty() && !s.contains('*') && !s.contains("  "));
            s.clear();
            ident(&mut rng, '_', &mut s);
            assert!(s.is_ascii() && s.contains('_'), "{s}");
            s.clear();
            password(&mut rng, &mut s);
            assert!(s.len() >= 8 && s.is_ascii());
        }
    }
}
