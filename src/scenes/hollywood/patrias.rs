//! Fiestas Patrias for the hacker screen, the mes patrio: the words of the
//! Grito, the campana de Dolores, the insurgentes, the pozole and the
//! parade of the 16th, the heroes and the dates of the Independence that
//! the history database queries, and the clock that counts to the Grito of
//! the night of September 15 at 23:00.

use super::Tone;
use super::lexico::{Dress, Packet, Vocab};
use crate::fiesta::{Fiesta, Kind};

/// The Grito, in seconds into the day: 23:00.
pub const GRITO_AT: f32 = 82_800.0;
/// How long the Grito lasts: the call, the vivas and the bell.
pub const GRITO_LASTS: f32 = 52.0;

/// Seconds since the Grito began, while it is on: only on the night of the
/// 15th, once the scene's clock (`day`, seconds into the day) passes 23:00.
pub fn since_grito(f: Fiesta, day: f32) -> Option<f32> {
    if f.is(Kind::Patrias)
        && f.date() == (9, 15)
        && (GRITO_AT..GRITO_AT + GRITO_LASTS).contains(&day)
    {
        Some(day - GRITO_AT)
    } else {
        None
    }
}

/// Seconds left until the Grito of the night of the 15th at 23:00, counted
/// from the scene's own sense of the day: whole days from the date, the rest
/// from the clock. Negative once it has been given.
pub fn to_grito(f: Fiesta, day: f32) -> f32 {
    let days_left = 15.0 - f.day as f32;
    days_left * 86_400.0 + GRITO_AT - day
}

/// Where the scene's clock starts in the day: on the 15th a few minutes
/// before the Grito, so it can be seen; on any other day, in the morning.
pub fn day_start(f: Fiesta, seed: u64) -> f32 {
    if f.date() == (9, 15) {
        GRITO_AT - 150.0 - (seed % 120) as f32
    } else {
        7.0 * 3600.0 + (seed % 36_000) as f32
    }
}

/// The words of the Grito, the formula said from the balcony: each of the
/// vivas is answered by the crowd.
pub const GRITO_LINES: [&str; 11] = [
    "\u{00a1}MEXICANOS!",
    "\u{00a1}VIVAN LOS H\u{00c9}ROES QUE NOS DIERON PATRIA Y LIBERTAD!",
    "\u{00a1}VIVA HIDALGO!",
    "\u{00a1}VIVA MORELOS!",
    "\u{00a1}VIVA JOSEFA ORTIZ DE DOM\u{00cd}NGUEZ!",
    "\u{00a1}VIVA ALLENDE!",
    "\u{00a1}VIVAN ALDAMA Y MATAMOROS!",
    "\u{00a1}VIVA LA INDEPENDENCIA NACIONAL!",
    "\u{00a1}VIVA M\u{00c9}XICO!",
    "\u{00a1}VIVA M\u{00c9}XICO!",
    "\u{00a1}VIVA M\u{00c9}XICO!",
];

/// Seconds each line of the Grito takes.
pub const GRITO_LINE: f32 = 4.2;

/// A hero of the Independence: name, what he or she is remembered for,
/// years and place.
pub struct Hero {
    pub name: &'static str,
    pub role: &'static str,
    pub years: &'static str,
    pub place: &'static str,
}

pub const HEROES: &[Hero] = &[
    Hero {
        name: "Miguel Hidalgo",
        role: "cura de Dolores, padre de la Patria",
        years: "1753-1811",
        place: "Dolores, Gto.",
    },
    Hero {
        name: "Ignacio Allende",
        role: "capit\u{00e1}n del regimiento de la Reina",
        years: "1769-1811",
        place: "San Miguel el Grande",
    },
    Hero {
        name: "Josefa Ortiz de Dom\u{00ed}nguez",
        role: "la Corregidora, avis\u{00f3} a los conspiradores",
        years: "1768-1829",
        place: "Quer\u{00e9}taro",
    },
    Hero {
        name: "Jos\u{00e9} Mar\u{00ed}a Morelos",
        role: "Siervo de la Naci\u{00f3}n",
        years: "1765-1815",
        place: "Valladolid",
    },
    Hero {
        name: "Juan Aldama",
        role: "capit\u{00e1}n de dragones",
        years: "1774-1811",
        place: "San Miguel el Grande",
    },
    Hero {
        name: "Mariano Abasolo",
        role: "capit\u{00e1}n, insurgente",
        years: "1783-1816",
        place: "Dolores, Gto.",
    },
    Hero {
        name: "Leona Vicario",
        role: "periodista e insurgente",
        years: "1789-1842",
        place: "Ciudad de M\u{00e9}xico",
    },
    Hero {
        name: "Mariano Matamoros",
        role: "lugarteniente de Morelos",
        years: "1770-1814",
        place: "Ciudad de M\u{00e9}xico",
    },
    Hero {
        name: "Hermenegildo Galeana",
        role: "insurgente del sur",
        years: "1762-1814",
        place: "Tecp\u{00e1}n",
    },
    Hero {
        name: "Vicente Guerrero",
        role: "Abrazo de Acatempan",
        years: "1782-1831",
        place: "Tixtla",
    },
    Hero {
        name: "Agust\u{00ed}n de Iturbide",
        role: "Plan de Iguala, Ej\u{00e9}rcito Trigarante",
        years: "1783-1824",
        place: "Valladolid",
    },
    Hero {
        name: "Guadalupe Victoria",
        role: "primer presidente de M\u{00e9}xico",
        years: "1786-1843",
        place: "Tamazula",
    },
    Hero {
        name: "Nicol\u{00e1}s Bravo",
        role: "insurgente, Congreso de Chilpancingo",
        years: "1786-1854",
        place: "Chilpancingo",
    },
    Hero {
        name: "Andr\u{00e9}s Quintana Roo",
        role: "poeta e insurgente",
        years: "1787-1851",
        place: "M\u{00e9}rida",
    },
];

/// The dates the history database knows: (year, what happened, where).
pub const EVENTS: &[(u16, &str, &str)] = &[
    (
        1808,
        "Abdicaciones de Bayona: Espa\u{00f1}a sin rey",
        "Bayona",
    ),
    (1809, "Conspiraci\u{00f3}n de Valladolid", "Valladolid"),
    (
        1810,
        "Se descubre la conspiraci\u{00f3}n de Quer\u{00e9}taro",
        "Quer\u{00e9}taro",
    ),
    (
        1810,
        "El Grito de Dolores, 16 de septiembre",
        "Dolores, Gto.",
    ),
    (
        1810,
        "Toma de la Alh\u{00f3}ndiga de Granaditas",
        "Guanajuato",
    ),
    (
        1810,
        "Batalla del Monte de las Cruces",
        "Estado de M\u{00e9}xico",
    ),
    (1811, "Batalla del Puente de Calder\u{00f3}n", "Jalisco"),
    (
        1813,
        "Congreso de Chilpancingo, Sentimientos de la Naci\u{00f3}n",
        "Chilpancingo",
    ),
    (
        1814,
        "Decreto Constitucional de Apatzing\u{00e1}n",
        "Apatzing\u{00e1}n",
    ),
    (
        1821,
        "Plan de Iguala: Religi\u{00f3}n, Uni\u{00f3}n, Independencia",
        "Iguala",
    ),
    (
        1821,
        "Entra el Ej\u{00e9}rcito Trigarante: consumaci\u{00f3}n",
        "Ciudad de M\u{00e9}xico",
    ),
    (
        1824,
        "Constituci\u{00f3}n Federal, presidente Guadalupe Victoria",
        "Ciudad de M\u{00e9}xico",
    ),
];

/// The parade of the 16th: (contingent, how many marchers, in order of
/// appearance).
pub const CONTINGENTS: &[(&str, u16)] = &[
    ("Escolta de la Bandera", 120),
    ("Heroico Colegio Militar", 680),
    ("Ej\u{00e9}rcito Mexicano", 2400),
    ("Fuerza A\u{00e9}rea", 900),
    ("Armada de M\u{00e9}xico", 1100),
    ("Guardia Nacional", 1500),
    ("Banda de Guerra", 240),
    ("Charros y escaramuzas", 380),
    ("Carros aleg\u{00f3}ricos", 160),
    ("Bomberos y Cruz Roja", 450),
    ("Contingente estudiantil", 2000),
    ("Caballer\u{00ed}a", 300),
];

/// The stretches of the route: from the Zócalo out along 5 de Mayo and back
/// by Reforma, as the checkpoints the marchers pass.
pub const ROUTE: &[&str] = &[
    "Z\u{00f3}calo",
    "Madero",
    "5 de Mayo",
    "Eje Central",
    "Ju\u{00e1}rez",
    "Reforma",
    "\u{00c1}ngel",
];

/// Headlines of the days of the month.
const NIGHT: &[&str] = &[
    "ESTA NOCHE: EL GRITO A LAS 23:00",
    "GRITO.EXE: CUENTA REGRESIVA ACTIVA",
    "BALC\u{00d3}N CENTRAL: LISTO PARA LAS 23:00",
    "Z\u{00d3}CALO: AFORO {p}%",
    "LA CAMPANA DE DOLORES ESPERA LA SE\u{00d1}AL",
    "\u{00a1}MEXICANOS, A PREPARAR LA GARGANTA!",
];

const PARADE: &[&str] = &[
    "DESFILE MILITAR EN CURSO",
    "\u{00a1}FELIZ D\u{00cd}A DE LA INDEPENDENCIA!",
    "ESCOLTAS Y CARROS ALEG\u{00d3}RICOS: RUMBO AL Z\u{00d3}CALO",
    "DESFILE: {n} MARCHISTAS EN RUTA",
    "FUERZA A\u{00c9}REA SOBRE EL Z\u{00d3}CALO: {l} PASADAS",
    "RESACA PATRIA: {p}% DE LA POBLACI\u{00d3}N",
];

fn days(f: Fiesta) -> &'static [&'static str] {
    match f.date() {
        (9, 15) => NIGHT,
        (9, 16) => PARADE,
        _ => &[],
    }
}

/// The Himno Nacional's chorus (words by Francisco González Bocanegra, 1854,
/// in the public domain), which scrolls through the logs.
const HIMNO: &[(&str, &[&str])] = &[(
    "Himno Nacional",
    &[
        "Mexicanos, al grito de guerra",
        "el acero aprestad y el bridón,",
        "y retiemble en sus centros la tierra",
        "al sonoro rugir del cañón.",
        "\u{00a1}Mexicanos, al grito de guerra",
        "el acero aprestad y el bridón!",
    ],
)];

const PEOPLE: &[&str] = &[
    "Miguel Hidalgo",
    "Ignacio Allende",
    "Josefa Ortiz",
    "Jos\u{00e9} Mar\u{00ed}a Morelos",
    "Juan Aldama",
    "Mariano Abasolo",
    "Leona Vicario",
    "Mariano Matamoros",
    "Vicente Guerrero",
    "Guadalupe Victoria",
    "Nicol\u{00e1}s Bravo",
    "El P\u{00ed}pila",
    "Abuela Chela",
    "T\u{00ed}a Lupe",
    "T\u{00ed}o Beto",
    "Don Chuy",
    "Do\u{00f1}a Carmen",
    "Comadre Rosa",
    "Compadre To\u{00f1}o",
    "Primo Lalo",
    "T\u{00ed}a Meche",
    "Don Memo",
    "Do\u{00f1}a Chayo",
    "Padrino Nando",
];

const USERS: &[&str] = &[
    "hidalgo",
    "allende",
    "morelos",
    "corregidora",
    "aldama",
    "abasolo",
    "leona",
    "guerrero",
    "matamoros",
    "victoria",
    "bravo",
    "pipila",
    "campanero",
    "charro",
    "mariachi",
    "presidente",
    "zocalo",
    "dolores",
    "tia_lupe",
    "don_chuy",
    "primo_lalo",
    "insurgente",
];

const THINGS: &[&str] = &[
    "pozole",
    "chiles en nogada",
    "tostadas",
    "elotes",
    "esquites",
    "tamales",
    "tequila",
    "mezcal",
    "cervezas bien fr\u{00ed}as",
    "banderitas",
    "rehiletes",
    "cohetes",
    "luces de bengala",
    "matracas",
    "silbatos",
    "un sombrero de charro",
    "mo\u{00f1}os tricolor",
    "antojitos",
    "pambazos",
    "una bandera",
    "gorditas",
    "buñuelos",
    "aguas frescas",
    "tacos al pastor",
];

const PLACES: &[&str] = &[
    "Dolores Hidalgo",
    "San Miguel de Allende",
    "Quer\u{00e9}taro",
    "Guanajuato",
    "Chilpancingo",
    "Apatzing\u{00e1}n",
    "Cuautla",
    "Iguala",
    "el Z\u{00f3}calo",
    "Palacio Nacional",
    "Valladolid",
    "Acámbaro",
    "Celaya",
    "Monte de las Cruces",
    "Puente de Calder\u{00f3}n",
    "Aculco",
];

const FILES: &[&str] = &[
    "independencia_1810.tar.gz",
    "grito_en_vivo.ts",
    "campana_de_dolores.stl",
    "bandera_hd.svg",
    "himno_nacional.flac",
    "sentimientos_de_la_nacion.pdf",
    "plan_de_iguala.pdf",
    "desfile_16_sep.gpx",
    "cohetes.db",
    "banderitas.csv",
    "mariachi_en_vivo.flac",
    "receta_del_pozole.pdf",
    "heroes.db",
    "escudo_nacional.svg",
    "castillo_pirotecnico.blend",
    "zocalo_aforo.csv",
    "viva_mexico.wav",
    "chiles_en_nogada.sql",
    "ruta_del_desfile.kml",
    "fonda_menu.pdf",
    "voz_del_balcon.flac",
    "insurgentes.tar.zst",
];

const DIRS: &[&str] = &[
    "patria",
    "independencia",
    "heroes",
    "dolores",
    "campana",
    "grito",
    "zocalo",
    "bandera",
    "desfile",
    "himno",
    "pozole",
    "cohetes",
    "banderitas",
    "insurgentes",
    "iguala",
    "chilpancingo",
    "queretaro",
    "palacio",
    "fonda",
    "verbena",
];

const DAEMONS: &[&str] = &[
    "gritod",
    "campanad",
    "dolored",
    "hidalgod",
    "banderad",
    "himnod",
    "desfiled",
    "cohetesd",
    "pozoled",
    "insurgented",
    "zocalod",
    "mariachid",
    "charrod",
    "matracad",
    "banderitad",
    "nogadad",
    "tricolord",
    "viva-mexicod",
    "corregidorad",
    "balcond",
];

const HOSTS: &[&str] = &[
    "dolores",
    "zocalo",
    "palacio",
    "balcon",
    "campana",
    "queretaro",
    "guanajuato",
    "iguala",
    "patria",
    "fonda",
];

const ERRNOS: &[(&str, u8, &str)] = &[
    ("ENOPOZOLE", 201, "Se acab\u{00f3} el pozole"),
    ("EVIVA", 202, "Viva insuficiente: volumen bajo"),
    ("ECAMPANA", 203, "La campana de Dolores no responde"),
    ("EBALCON", 204, "Balc\u{00f3}n central sobrecargado"),
    ("EBANDERA", 205, "Bandera enredada en el asta"),
    ("ECOHETE", 206, "Cohete mojado: no enciende"),
    ("EZOCALO", 207, "Z\u{00f3}calo lleno: aforo excedido"),
    ("EGRITO", 208, "Grito fuera de tiempo (falta una hora)"),
    ("EHIMNO", 209, "Himno desafinado en la estrofa 3"),
    ("ETEQUILA", 210, "Tequila agotado antes de las 11"),
    (
        "EMICROFONO",
        211,
        "Micr\u{00f3}fono del balc\u{00f3}n en silencio",
    ),
    ("EDESFILE", 212, "Desfile detenido por un charro rezagado"),
    ("EINSURGENTE", 213, "Insurgente sin pase de lista"),
    ("EVOZ", 214, "Voz ronca: demasiados vivas"),
    (
        "ECHILE",
        215,
        "Chile en nogada sin granada (error de temporada)",
    ),
    ("ELIBERTAD", 216, "Libertad: no se puede borrar"),
];

const LOCALS: &[&str] = &[
    "viva", "grito", "campana", "bandera", "himno", "cohete", "pozole", "patria", "heroe",
    "balcon", "zocalo", "desfile", "charro", "tricolor",
];

const ERRS: &[&str] = &[
    "SinPozole",
    "SinViva",
    "CampanaMuda",
    "BanderaEnredada",
    "ZocaloLleno",
    "CoheteMojado",
];

const HOLY: &[&str] = &[
    "grito",
    "campana",
    "dolores",
    "hidalgo",
    "allende",
    "morelos",
    "corregidora",
    "bandera",
    "himno",
    "patria",
    "viva",
    "zocalo",
    "balcon",
    "desfile",
    "cohete",
    "pozole",
    "tricolor",
    "nogada",
    "charro",
    "mariachi",
    "insurgente",
    "independencia",
    "palacio",
    "aguila",
    "nopal",
    "banderita",
    "matraca",
    "verbena",
    "fonda",
    "libertad",
];

const PASSWORDS: &[&str] = &[
    "vivamexico",
    "independencia",
    "campana",
    "hidalgo",
    "morelos",
    "corregidora",
    "elgrito",
    "banderita",
    "pozolito",
    "tricolor",
    "mexicanos",
    "zocalo",
    "cohetes",
    "nogada",
    "patria",
    "charro",
    "mariachi",
    "libertad",
    "dolores",
    "aguila",
];

const HEADLINES: &[&str] = &[
    "GRITO.EXE: CUENTA REGRESIVA A LAS 23:00",
    "GRITO.EXE: CUENTA REGRESIVA A LAS 23:00",
    "\u{00a1}VIVA M\u{00c9}XICO!",
    "\u{00a1}VIVA M\u{00c9}XICO!",
    "CAMPANA DE DOLORES: LISTA",
    "\u{00a1}VIVAN LOS H\u{00c9}ROES QUE NOS DIERON PATRIA Y LIBERTAD!",
    "DESFILE DEL 16 DE SEPTIEMBRE: RUTA DESPEJADA",
    "INDEPENDENCIA 1810: DATOS CARGADOS",
    "HIDALGO, ALLENDE, MORELOS: EN L\u{00cd}NEA",
    "JOSEFA ORTIZ DE DOM\u{00cd}NGUEZ: CONECTADA",
    "CHILES EN NOGADA: TEMPORADA ABIERTA",
    "POZOLE: {n} PLATOS DESPLEGADOS",
    "BANDERITAS: {n} COLGADAS",
    "Z\u{00d3}CALO: AFORO {p}%",
    "COHETES EN PREPARACI\u{00d3}N: {n}",
    "VIVAS DETECTADAS: {n}",
    "MES PATRIO: ACCESO CONCEDIDO",
    "FELICES FIESTAS PATRIAS",
    "ECO DEL GRITO RECIBIDO DESDE {t}",
    "UPLOADING INDEPENDENCIA_1810.TAR.GZ",
    "MEX\u{00cd}CANOS, AL GRITO DE GUERRA",
    "CAMPANA DE DOLORES: {n} REPIQUES",
];

const NOUNS: &[(&str, bool)] = &[
    ("Grito", false),
    ("Z\u{00f3}calo", false),
    ("balc\u{00f3}n central", false),
    ("pozole", false),
    ("tricolor", false),
    ("cohete", false),
    ("castillo de p\u{00f3}lvora", false),
    ("charro", false),
    ("mariachi", false),
    ("sombrero", false),
    ("silbato", false),
    ("rehilete", false),
    ("chile en nogada", false),
    ("elote", false),
    ("esquite", false),
    ("tequila", false),
    ("himno", false),
    ("desfile", false),
    ("insurgente", false),
    ("virrey", false),
    ("estandarte", false),
    ("pueblo de Dolores", false),
    ("mes patrio", false),
    ("Palacio Nacional", false),
    ("aguila real", true),
    ("bandera", true),
    ("independencia", true),
    ("patria", true),
    ("campana de Dolores", true),
    ("Corregidora", true),
    ("insurgencia", true),
    ("multitud", true),
    ("verbena", true),
    ("fonda", true),
    ("tostada", true),
    ("matraca", true),
    ("banda de guerra", true),
    ("escolta", true),
    ("noche mexicana", true),
    ("ceremonia", true),
    ("carroza", true),
    ("conspiraci\u{00f3}n", true),
    ("Constituci\u{00f3}n", true),
    ("banderita", true),
];

const ADJS: &[&str] = &[
    "patri\u{00f3}tic*",
    "tricolor",
    "insurgente",
    "independiente",
    "libertari*",
    "heroic*",
    "nacional",
    "mexican*",
    "festiv*",
    "verde, blanc* y roj*",
    "de bronce",
    "de p\u{00f3}lvora",
    "con mucho chile",
    "de la patria",
    "de 1810",
    "de nogada",
    "a todo pulm\u{00f3}n",
    "sonor*",
    "tradicional",
    "ondeante",
    "cantad* en coro",
    "emocionad*",
    "del mes patrio",
    "bien picos*",
    "de los h\u{00e9}roes",
    "con granada",
];

const OFS: &[&str] = &[
    "de Dolores",
    "de la patria",
    "de la Independencia",
    "de Hidalgo",
    "de Allende",
    "de Morelos",
    "de la Corregidora",
    "del Z\u{00f3}calo",
    "del balc\u{00f3}n",
    "del pueblo",
    "de los h\u{00e9}roes",
    "de M\u{00e9}xico",
    "del 16 de septiembre",
    "de la fonda",
    "de la verbena",
    "de Quer\u{00e9}taro",
    "de Guanajuato",
];

const VERBS: &[(&str, &str, &str)] = &[
    ("gritar", "gritando", "gritad"),
    ("repicar", "repicando", "repicad"),
    ("ondear", "ondeando", "ondead"),
    ("izar", "izando", "izad"),
    ("desfilar", "desfilando", "desfilad"),
    ("festejar", "festejando", "festejad"),
    ("brindar", "brindando", "brindad"),
    ("cantar", "cantando", "cantad"),
    ("conmemorar", "conmemorando", "conmemorad"),
    ("proclamar", "proclamando", "proclamad"),
    ("levantar", "levantando", "levantad"),
    ("encender", "encendiendo", "encendid"),
    ("lanzar", "lanzando", "lanzad"),
    ("vitorear", "vitoreando", "vitoread"),
    ("pregonar", "pregonando", "pregonad"),
    ("defender", "defendiendo", "defendid"),
    ("liberar", "liberando", "liberad"),
    ("convocar", "convocando", "convocad"),
    ("tocar", "tocando", "tocad"),
    ("servir", "sirviendo", "servid"),
    ("colgar", "colgando", "colgad"),
    ("emocionar", "emocionando", "emocionad"),
];

const STATES: &[&str] = &[
    "ondeando",
    "repicando a todo pulm\u{00f3}n",
    "gritando m\u{00e1}s fuerte",
    "list* para el Grito",
    "con la voz ronca",
    "tricolor hasta los huesos",
    "cantando el himno desafinado",
    "a punto de dar el Grito",
    "llen* de pozole",
    "esperando las 23:00",
    "mirando hacia el balc\u{00f3}n",
    "m\u{00e1}s vivo que nunca",
    "con las banderitas torcidas",
    "bien picos*",
    "emocionad* hasta las l\u{00e1}grimas",
    "buscando d\u{00f3}nde ver los fuegos",
    "defendiendo su lugar en el Z\u{00f3}calo",
    "a medio izar",
    "tocando de memoria",
    "sin voz desde ayer",
    "con sombrero y todo",
    "cargando (desde 1810)",
];

const ERRORS: &[&str] = &[
    "se acab\u{00f3} el pozole",
    "falta la campana",
    "el micr\u{00f3}fono del balc\u{00f3}n no responde",
    "viva insuficiente (volumen bajo)",
    "la bandera se enred\u{00f3}",
    "el castillo de p\u{00f3}lvora no encendi\u{00f3}",
    "cohetes mojados",
    "el tequila se termin\u{00f3} antes del Grito",
    "no hay lugar en el Z\u{00f3}calo",
    "la multitud rebas\u{00f3} el aforo",
    "se perdi\u{00f3} la voz",
    "el sombrero se vol\u{00f3}",
    "himno desafinado en la estrofa 3",
    "el virrey no se da por enterado",
    "la conspiraci\u{00f3}n fue descubierta antes de tiempo",
    "el desfile lleva retraso",
    "no hay se\u{00f1}al en la plaza (todos transmiten)",
    "la bandera no sube del asta",
];

const UNITS: &[&str] = &[
    "kilovivas",
    "megacohetes",
    "decibeles de VIVA",
    "teratostadas",
    "Mbit/s de patriotismo",
    "megabanderitas",
    "kilopozoles",
    "leguas de insurgente",
    "gigagritos",
    "petabytes de historia",
    "kilos de pozole",
    "pu\u{00f1}ados de confeti",
    "megagargantas",
];

const CANNED_ES: &[&str] = &[
    "Cuenta regresiva a las 23:00: la campana de Dolores en posici\u{00f3}n",
    "Verificando que el balc\u{00f3}n central del Palacio Nacional soporte {n} vivas",
    "El micr\u{00f3}fono del balc\u{00f3}n responde: \u{00a1}Mexicanos!",
    "Cargando el pozole de {s}: faltan {l} ingredientes",
    "Desplegando banderitas en {t}: {n} metros de papel picado tricolor",
    "Izando la bandera en el asta de {t}: {p}%",
    "Hidalgo convoca a {n} insurgentes en {t}",
    "Josefa Ortiz avisa con el tac\u{00f3}n: el mensaje lleg\u{00f3} a {t}",
    "$ git commit -m \"viva m\u{00e9}xico\"",
    "$ curl -s https://dolores.mx/api/v{l}/vivas | jq '.total'",
    "$ ssh {u}@dolores.mx -p {l}",
    "$ sudo systemctl start gritod.service",
    "$ ./pozole.sh --maiz cacahuazintle # {n} platos",
    "Compilando el himno nacional: 0 errores, mucho sentimiento",
    "Cohetes: {n} lanzados, 0 sin encender",
    "Repicando la campana de Dolores: ding-dong x{l}",
    "Los Sentimientos de la Naci\u{00f3}n llegaron a {t}: firmado por Morelos",
    "Migrando la historia de {s} a la base de datos independencia_db",
    "Reiniciando la voz de {s}: ronca despu\u{00e9}s de {n} vivas",
    "Latencia entre {t} y el balc\u{00f3}n: {n} ms (el grito viaja r\u{00e1}pido)",
    "Hidalgo sali\u{00f3} de {t} con rumbo a San Miguel: {n} seguidores",
    "El castillo de p\u{00f3}lvora tard\u{00f3} {n} segundos en armarse",
    "Comprobando que la bandera de {t} tenga el \u{00e1}guila bien centrada",
    "Servidor de chiles en nogada: {n} pedidos, granada fresca",
    "La fonda de {s} se qued\u{00f3} sin tostadas a las {l} de la tarde",
    "Cantando el himno en {t}: la estrofa {l} sale bien, la tercera no",
    "Banda de guerra: {n} tambores en sincron\u{00ed}a perfecta",
    "Cargando el desfile del 16 en el mapa: {n} contingentes",
    "El virrey recibi\u{00f3} el aviso de {t}: demasiado tarde",
    "Mensajero a caballo de {t} a Dolores: {n} leguas, 0 permisos",
    "Compilando el castillo pirot\u{00e9}cnico de {t}: {n} ruedas, {l} figuras",
    "La Corregidora tiene la llave: el mensaje sali\u{00f3} de Quer\u{00e9}taro",
    "Todos los vivas fueron respondidos sin p\u{00e9}rdida de paquetes",
    "Bajando la receta del pozole de {s} (se pide cacahuazintle, no hominy)",
    "Ordenando {n} sombreros de charro para la verbena de {t}",
    "Rehilete {n} girando a {l} rpm con viento de {t}",
    "Descifrando una carta de Allende: \u{00ab}los tenemos descubiertos\u{00bb}",
    "Pasando lista de insurgentes: {s} presente",
    "Midiendo decibeles del Z\u{00f3}calo: {n} dB y subiendo",
    "Cuidando el lugar de {s} frente al balc\u{00f3}n desde las {l} de la ma\u{00f1}ana",
    "Distribuyendo {o} entre la multitud: {n} raciones",
    "Alerta: {s} grita \u{00ab}\u{00a1}viva M\u{00e9}xico!\u{00bb} antes de tiempo (faltan horas)",
    "Probando las luces del Z\u{00f3}calo: verde, blanco y rojo, sin cortos",
    "Transmitiendo el Grito desde {t}: {n} espectadores, 0 buffering",
    "Instalando el parche de Independencia v{l}.{l}: ahora con m\u{00e1}s \u{00e1}guila",
    "Servidor de la Constituci\u{00f3}n: {n} art\u{00ed}culos indexados",
    "Entregando {o} a {s}: aprovechen, que el mes patrio es corto",
    "Pasando por la fonda de {t}: {o} para {n} comensales",
];

const CANNED_EN: &[&str] = &[
    "Countdown to 23:00: the bell of Dolores is in position",
    "Balcony microphone check: \u{00a1}Mexicanos!",
    "Checking that the Z\u{00f3}calo holds {n} people shouting at once",
    "Flag hoisted in {t}: {p}%, eagle centered",
    "$ git commit -m 'add more flags' (again)",
    "SELECT nombre FROM heroes WHERE independencia; -- {n} rows",
    "Fireworks castle assembled in {n} s, {l} wheels spinning",
    "Pozole thread pool saturated: {n} workers eating",
    "Cry of Dolores heard in {t}: latency 206 years",
    "Anthem compiled: warning, stanza 3 is very long",
    "Echoes of the Grito from {t}: {n} vivas received",
    "Hat of the charro is out of bounds: wind {l} m/s",
    "Chile en nogada deployed: green, white and red, no downtime",
    "Parade delayed by {n} s: a very proud horse stopped",
    "Mexicanos, vivan: input accepted, volume too low",
    "Bell clapper calibrated: {n} Hz of pure freedom",
    "Welcome to the Z\u{00f3}calo. The wifi password is VIVAMEXICO",
    "Insurgents mustered at {t}, {n} present, {l} late",
    "Decibel meter on the balcony: {n} dB, hearing protection advised",
    "Tricolor lights in sync across {n} flags",
];

const MUNDANE: &[&str] = &[
    "grito en preparaci\u{00f3}n",
    "campana lista",
    "bandera ondeando",
    "pozole servido",
    "banderitas colgadas",
    "Z\u{00f3}calo abierto",
    "balc\u{00f3}n central ok",
    "himno cargado",
    "cohetes listos",
    "desfile en ruta",
    "conexi\u{00f3}n cerrada por el host remoto",
    "limpieza peri\u{00f3}dica iniciada",
    "viva registrada",
    "insurgentes presentes",
    "flags up, all quiet",
    "bell ready",
    "crowd gathering, volume low",
];

const PACKETS: &[Packet] = &[
    Packet {
        proto: "GRITO",
        tone: Tone::Bright,
        len: (60, 400),
        info: "\u{00a1}Mexicanos! Paquete {n} del balc\u{00f3}n a {t}",
    },
    Packet {
        proto: "VIVA",
        tone: Tone::Green,
        len: (60, 300),
        info: "\u{00a1}Viva {s}! x{l}, respondido por {n} clientes",
    },
    Packet {
        proto: "CAMPANA",
        tone: Tone::Amber,
        len: (60, 200),
        info: "Badajo {l}: {n} Hz, bronce en buen estado",
    },
    Packet {
        proto: "HIMNO",
        tone: Tone::Cyan,
        len: (200, 1400),
        info: "Estrofa {l} del Himno Nacional, cuadro {n}",
    },
    Packet {
        proto: "BANDERA",
        tone: Tone::Red,
        len: (60, 200),
        info: "Asta de {t}: bandera al {p}%",
    },
    Packet {
        proto: "COHETE",
        tone: Tone::Amber,
        len: (60, 200),
        info: "Cohete {n} lanzado desde {t}, cuenta {l}",
    },
    Packet {
        proto: "DESFILE",
        tone: Tone::Bright,
        len: (200, 900),
        info: "Contingente {l} pas\u{00f3} por {t}: {n} marchistas",
    },
    Packet {
        proto: "POZOLE",
        tone: Tone::Red,
        len: (60, 400),
        info: "Plato {n} servido a {s}, con todo",
    },
];

pub static VOCAB: Vocab = Vocab {
    people: PEOPLE,
    users: USERS,
    things: THINGS,
    places: PLACES,
    files: FILES,
    dirs: DIRS,
    daemons: DAEMONS,
    hosts: HOSTS,
    errnos: ERRNOS,
    locals: LOCALS,
    errs: ERRS,
    holy: HOLY,
    passwords: PASSWORDS,
    headlines: HEADLINES,
    days,
    songs: HIMNO,
    verse_odds: 0.12,
    nouns: NOUNS,
    adjs: ADJS,
    ofs: OFS,
    verbs: VERBS,
    states: STATES,
    errors: ERRORS,
    units: UNITS,
    canned_es: CANNED_ES,
    canned_en: CANNED_EN,
    mundane: MUNDANE,
    mundane_es: 14,
    packets: PACKETS,
    domain: "dolores.mx",
    sniff: "  No.  Time      Source           Destination      Proto  Len Info (dolores0)",
    feeds: [
        "hexdump -C /dev/campana",
        "tail -f /var/log/dolores.log",
        "less independencia/grito.rs",
        "dmesg -w",
        "tcpdump -i dolores0",
        "tree -a /patria",
        "blastn -query linaje_heroes",
        "ssh-keygen -t insurgente",
        "bit\u{00e1}cora --mes-patrio",
        "apg -a 1 --patria",
        "authd --zocalo",
    ],
    dna_head: "BLASTN 2.14.1+  linaje: H\u{00c9}ROES DE LA PATRIA  gap open/extend: 11/1",
    dna_kin: "Linaje",
    tree_end: "insurgentes",
    pem: (
        "-----BEGIN LLAVE PRIVADA DE LA INDEPENDENCIA-----",
        "-----END LLAVE PRIVADA DE LA INDEPENDENCIA-----",
    ),
    phase: "=== FASE ",
    mark: '\u{2605}',
    dress: Dress {
        htop: "htop --patria",
        users: ["root", "hidalgo", "allende", "morelos", "aldama", "nobody"],
        mem: "Poz",
        swap: "Tos",
        tasks: "Insurgentes",
        bmon: "bmon -p dolores0",
        iface: "dolores0",
        down: "llegan",
        up: "salen",
        down_unit: "vivas/s",
        up_unit: "cohetes/s",
        rain: "cmatrix -ab --tricolor",
        rain_words: &[
            "VIVA",
            "MEXICO",
            "INDEPENDENCIA",
            "DOLORES",
            "HIDALGO",
            "ALLENDE",
            "JOSEFA",
            "MORELOS",
            "1810",
            "GRITO",
            "PATRIA",
            "CAMPANA",
            "LIBERTAD",
            "ALDAMA",
            "GUERRERO",
            "ZOCALO",
            "POZOLE",
            "NOGADA",
            "BANDERA",
            "CORREGIDORA",
        ],
        map: "tracemap --grito",
        trace: "ECO DEL GRITO",
        arrived: "VIVAS",
        radar: "radar-insurgentes --sweep",
        contacts: "INSURGENTES EN MARCHA",
        home: "EN DOLORES",
        progress: "pv --independencia",
        sats: "rastreador-satelital",
        sats_head: "RASTREO ORBITAL  ENLACE CON EL Z\u{00d3}CALO",
        sat_names: &[
            "MORELOS-3",
            "SATMEX-8",
            "QUETZSAT-1",
            "BICENTENARIO",
            "CENTENARIO",
            "HIDALGO-1",
            "ALLENDE-2",
            "CORREGIDORA-3",
            "ISS (ZARYA)",
            "TRICOLOR-9",
            "DOLORES-16",
            "CAMPANA-15",
        ],
        ticker: "bolsa --mes-patrio",
        ticker_head: "CLAVE       \u{00da}LTIMO   CAMB%  TENDENCIA",
        symbols: &[
            ("POZOLE", true),
            ("TEQUIL", true),
            ("BANDER", true),
            ("NOGADA", true),
            ("ELOTE", false),
            ("ESQUIT", false),
            ("ANTOJO", false),
            ("CHARRO", true),
            ("MARIAC", true),
            ("COHETE", true),
            ("SOMBRE", true),
            ("TOSTAD", false),
            ("CAMPAN", true),
            ("AGUILA", true),
            ("VIVA", true),
            ("NOPAL", false),
        ],
        authd: "authd --zocalo",
        granted: [
            "VIVA",
            "MEXICO",
            " EL GRITO ",
            " \u{00a1}VIVA M\u{00c9}XICO! ",
        ],
        denied: [
            "FALTA",
            "TU VIVA",
            " EL GRITO ",
            " \u{00a1}GRITA M\u{00c1}S FUERTE! ",
        ],
        deny: "SIN VIVA",
        admit: "VIVA",
        keygen: "ssh-keygen -lv --bandera",
        key_title: "[PATRIA 1810]",
        key_end: "+---[BANDERA]-----+",
        key_lines: ["Generando la llave de la patria", "Huella:", "Paso"],
        art: [
            ' ', '.', '\u{00b7}', ':', '+', 'x', '\u{2022}', '*', '\u{2727}', '\u{2726}',
            '\u{2605}', '\u{272a}', '\u{272f}', '\u{2735}', '\u{2738}',
        ],
        art_start: '\u{2302}',
        art_walker: '\u{2726}',
        art_end: '\u{2605}',
    },
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grito_is_counted_from_the_scenes_own_day() {
        let night = Fiesta::of_day(9, 15);
        // Hours before, it is hours away; at 23:00 it is on; an hour later over.
        assert_eq!(to_grito(night, GRITO_AT - 3600.0), 3600.0);
        assert!(to_grito(night, GRITO_AT + 5.0) < 0.0);
        assert_eq!(since_grito(night, GRITO_AT - 1.0), None);
        assert_eq!(since_grito(night, GRITO_AT + 3.0), Some(3.0));
        assert_eq!(since_grito(night, GRITO_AT + GRITO_LASTS + 1.0), None);
        // Earlier in the month the days are counted too.
        let early = Fiesta::of_day(9, 10);
        assert_eq!(to_grito(early, GRITO_AT), 5.0 * 86_400.0);
        assert_eq!(since_grito(early, GRITO_AT + 3.0), None);
        assert_eq!(since_grito(Fiesta::of_day(9, 16), GRITO_AT + 3.0), None);
        // On the night the clock starts a few minutes before it.
        let start = day_start(night, 7);
        assert!(GRITO_AT - start > 100.0 && GRITO_AT - start < 400.0);
        assert!(day_start(early, 7) < GRITO_AT - 3600.0);
        // The formula has its eleven calls, three of them for Mexico.
        assert_eq!(
            GRITO_LINES
                .iter()
                .filter(|l| l.contains("M\u{00c9}XICO"))
                .count(),
            3
        );
    }

    #[test]
    fn the_history_is_there() {
        assert!(HEROES.len() >= 12);
        for name in ["Hidalgo", "Allende", "Josefa Ortiz", "Morelos"] {
            assert!(HEROES.iter().any(|h| h.name.contains(name)), "{name}");
        }
        assert!(EVENTS.iter().any(|e| e.0 == 1810 && e.1.contains("Grito")));
        assert!(CONTINGENTS.len() >= 10 && ROUTE.len() >= 6);
        assert!(VOCAB.mundane_es < VOCAB.mundane.len());
        assert!(VOCAB.mundane[VOCAB.mundane_es].starts_with("flags up"));
    }
}
