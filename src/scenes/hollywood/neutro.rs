//! The neutral Spanish base (spec 8.1): what the hacker screen says under a
//! preset when the preset has nothing to say. Spanish technobabble about
//! nothing in particular, with no holiday, no country and no real person in
//! it. A preset is laid over it, so every field is filled and every shape
//! rule of the widgets holds (`gate::check`). Its lines lean on blanks, so a
//! preset's names reach them; its things carry their articles, as a preset's
//! do, and `Vocab::expand` contracts "de el" into "del" where they land.

use super::Tone;
use super::lexico::{ADJS, Dress, ERRORS, NOUNS, Packet, STATES, UNITS, VERBS, Vocab};
use crate::fiesta::Fiesta;

/// Call signs for the radar's contacts and the aligner (`NODO-417`): what a
/// preset's short names stand in for. No people.
pub static CALLS: &[&str] = &[
    "NODO", "ECO", "VECTOR", "FARO", "PULSO", "ORBITA", "ATALAYA", "RELEVO", "CENIT", "ANCLA",
    "BRUJULA", "DELTA", "SIGMA", "TANGENTE", "PRISMA", "CENTELLA",
];

/// What a transfer is doing, in Spanish: "subiendo respaldo.tar.gz".
pub static TRANSFERS: &[&str] = &[
    "subiendo",
    "descargando",
    "respaldando",
    "transfiriendo",
    "restaurando",
    "sincronizando",
];

const PEOPLE: &[&str] = &[
    "Operador Nocturno",
    "Operadora Vega",
    "Ingeniero Paredes",
    "Doctora Luna",
    "Técnico Ruiz",
    "Agente Ibarra",
    "Coordinadora Salas",
    "Analista Ortega",
    "Supervisor Campos",
    "Becaria Núñez",
    "Administradora Rojas",
    "Auditor Medina",
    "Comandante Escobar",
    "Especialista Prado",
    "Guardia Molina",
    "Jefa de Sistemas",
    "Soporte Nivel 2",
    "Ingeniera Cortés",
];

const USERS: &[&str] = &[
    "operador", "admin", "respaldo", "monitor", "vega", "aparedes", "dluna", "truiz", "ibarra",
    "csalas", "ortega", "campos", "auditor", "guardia", "soporte", "invitado",
];

const THINGS: &[&str] = &[
    "el kernel",
    "el servidor central",
    "el cortafuegos",
    "el protocolo de enlace",
    "el certificado raíz",
    "el módulo de cifrado",
    "el clúster de cómputo",
    "el túnel cifrado",
    "la base de datos",
    "la bitácora",
    "la red interna",
    "la consola principal",
    "la copia de seguridad",
    "la memoria compartida",
    "los respaldos",
    "los registros del sistema",
    "los puertos abiertos",
    "las llaves maestras",
    "las credenciales",
    "las tablas de rutas",
];

const PLACES: &[&str] = &[
    "Nodo Norte",
    "Nodo Sur",
    "Sala de Servidores 3",
    "Centro de Datos Delta",
    "Laboratorio B",
    "Sótano 2",
    "Estación Eco",
    "Sector 7",
    "Torre de Control",
    "Búnker de Respaldo",
    "Subestación 12",
    "Planta Piloto",
    "Hangar 4",
    "Bóveda Central",
];

const FILES: &[&str] = &[
    "respaldo_completo.tar.gz",
    "bitacora_2026.log",
    "llaves_maestras.pem",
    "volcado_memoria.bin",
    "config_red.yaml",
    "credenciales.kdbx",
    "registro_accesos.csv",
    "parche_seguridad.patch",
    "imagen_disco.iso",
    "tabla_rutas.db",
    "firmware_v3.bin",
    "informe_auditoria.pdf",
    "mapa_red.svg",
    "certificado_raiz.crt",
    "nodos_activos.json",
    "respaldo_nocturno.tar.zst",
];

const DIRS: &[&str] = &[
    "sistema",
    "respaldos",
    "bitacoras",
    "llaves",
    "red",
    "nodos",
    "modulos",
    "registros",
    "auditoria",
    "parches",
    "temporal",
    "compartido",
    "seguridad",
    "volcados",
];

const DAEMONS: &[&str] = &[
    "vigilanted",
    "respaldod",
    "cifradod",
    "enlaced",
    "bitacorad",
    "relojd",
    "nodod",
    "auditord",
    "centinelad",
    "sincrod",
    "rastreod",
    "guardiand",
    "faro-redd",
    "pulsod",
];

const HOSTS: &[&str] = &[
    "nodo-central",
    "eco",
    "vector",
    "faro",
    "boveda",
    "atalaya",
    "torre",
    "relevo",
    "archivo",
    "puente",
    "centinela",
    "nucleo",
    "cenit",
    "ancla",
];

const ERRNOS: &[(&str, u8, &str)] = &[
    ("ENOENLACE", 201, "El enlace no responde"),
    ("ENODO", 202, "Nodo fuera de línea"),
    ("ELLAVE", 203, "Llave inválida o vencida"),
    ("EFIRMA", 204, "La firma no coincide"),
    ("ESESION", 205, "Sesión vencida: vuelva a autenticarse"),
    ("ECANAL", 206, "Canal cerrado por el otro extremo"),
    ("EBLOQUE", 207, "Bloque corrupto en el disco"),
    ("ERELOJ", 208, "Reloj desfasado más allá de lo tolerable"),
    ("ECUOTA", 209, "Cuota de disco excedida"),
    ("EACCESO", 210, "Acceso denegado por la política vigente"),
    ("ERESPALDO", 211, "Respaldo incompleto"),
    ("ECIFRADO", 212, "El módulo de cifrado no arrancó"),
    ("ERUTA", 213, "No hay ruta hacia el destino"),
    ("EPULSO", 214, "Se perdió el pulso del nodo"),
    ("EQUORUM", 215, "Sin quórum: faltan nodos por votar"),
    ("EPARCHE", 216, "El parche no aplica sobre esta versión"),
    ("ECERT", 217, "Certificado no reconocido"),
    ("EBITACORA", 218, "La bitácora está llena"),
    ("ETURNO", 219, "Nadie de guardia en este turno"),
    ("EESPEJO", 220, "El espejo no está sincronizado"),
    ("ESENAL", 221, "Señal demasiado débil"),
    ("EPUERTO", 222, "Puerto ocupado por otro proceso"),
    ("ECOLA", 223, "Cola de mensajes desbordada"),
    ("EINTENTOS", 224, "Demasiados intentos fallidos"),
    ("EVERSION", 225, "Versión incompatible del protocolo"),
    ("EHUELLA", 226, "La huella digital no coincide"),
];

const LOCALS: &[&str] = &[
    "nodo", "enlace", "llave", "registro", "bitacora", "paquete", "sesion", "intento", "firma",
    "canal", "sector", "bloque", "pulso", "turno",
];

const ERRS: &[&str] = &[
    "SinEnlace",
    "LlaveInvalida",
    "NodoCaido",
    "AccesoDenegado",
    "FirmaRota",
    "TiempoAgotado",
    "SesionVencida",
    "CanalCerrado",
    "BloqueCorrupto",
    "SinPermiso",
    "DiscoLleno",
    "RelojDesfasado",
];

const HOLY: &[&str] = &[
    "nodo",
    "enlace",
    "vector",
    "eco",
    "faro",
    "centinela",
    "boveda",
    "bitacora",
    "llave",
    "sello",
    "firma",
    "umbral",
    "pulso",
    "senal",
    "atalaya",
    "relevo",
];

const PASSWORDS: &[&str] = &[
    "contrasena",
    "administrador",
    "puertatrasera",
    "clavemaestra",
    "boveda",
    "centinela",
    "acceso",
    "sesamo",
    "llavero",
    "nodocentral",
    "respaldo",
    "cortafuegos",
    "invitado",
    "turnonoche",
];

const HEADLINES: &[&str] = &[
    "AVISO DE {s}: REINICIO PROGRAMADO EN {n} MINUTOS",
    "AUDITORÍA DE {o}: {p}% COMPLETADA",
    "ACCESO A {o}: RESTRINGIDO",
    "{t}: TODOS LOS NODOS EN LÍNEA",
    "{t}: ENLACE RESTABLECIDO",
    "{s} INICIA SESIÓN DESDE {t}",
    "RESPALDO NOCTURNO: {n} ARCHIVOS VERIFICADOS",
    "SINCRONIZACIÓN DE {o} AL {p}%",
    "ALERTA EN {t}: TEMPERATURA DE {o} AL LÍMITE",
    "NIVEL DE AMENAZA: {l} DE 9",
    "INTENTOS DE INTRUSIÓN BLOQUEADOS: {n}",
    "{s} AUTORIZA EL ACCESO A {o}",
    "PARCHE {l}.{l}.{n} DESPLEGADO SIN INCIDENTES",
    "MANTENIMIENTO PROGRAMADO EN {t}: {n} MINUTOS",
    "LATENCIA DE {t}: {p} MS",
    "CIFRADO DE {o}: ACTIVO",
    "TURNO DE {s}: SIN NOVEDAD",
    "{n} PAQUETES ANALIZADOS EN {t}",
    "NODO 0x{x}: SEÑAL RECUPERADA",
    "VERIFICANDO LA INTEGRIDAD DE {o}",
    "{s} SOLICITA UNA COPIA DE {o}",
    "CARGA DEL CLÚSTER: {p}%",
    "ÚLTIMA COPIA DE SEGURIDAD: HACE {l} HORAS",
    "EN ESPERA DE {s}: QUÓRUM {l} DE 9",
    "TRANSMISIÓN DESDE {t}: {n} KB/S",
    "PROTOCOLO {x} ACTIVADO POR {s}",
];

fn day_headlines(_: Fiesta) -> &'static [&'static str] {
    &[]
}

const OFS: &[&str] = &[
    "del sistema",
    "de respaldo",
    "de la red interna",
    "del nodo central",
    "de emergencia",
    "de alta seguridad",
    "del sector 7",
    "de producción",
    "de prueba",
    "del turno de noche",
    "de la bóveda",
    "del laboratorio",
];

const CANNED_ES: &[&str] = &[
    "Aviso de {s}: reinicio programado en {n} minutos",
    "Auditoría de {o}: {p}% completada, sin hallazgos críticos",
    "Acceso a {o} concedido a {u} desde {t}",
    "Acceso a {o} denegado: {u} no tiene permisos",
    "Verificando la integridad de {o}: {n} bloques revisados",
    "Respaldo de {o} completado en {n} segundos",
    "Sincronizando {o} con el nodo de {t} ({p}%)",
    "{s} solicita acceso de emergencia a {o}",
    "{s} cerró la sesión en {t}",
    "Nueva sesión de {u} desde 10.0.{l}.{p}",
    "Cifrando {o} con una llave de {n} bits",
    "Rotando las llaves de {o}: {l} de 9",
    "Escaneando los puertos de {t}: {n} abiertos, {l} sospechosos",
    "Enlace con {t} restablecido tras {n} ms",
    "Latencia hacia {t}: {p} ms (dentro de lo tolerable)",
    "Parche {l}.{l}.{n} aplicado sobre {o}",
    "Error en {o}: código 0x{x}",
    "Alerta: {o} supera el umbral de carga ({p}%)",
    "Recuperando {o} desde la copia de {t}",
    "Turno de {s}: {n} eventos, ninguno crítico",
    "{s} aprueba el despliegue en {t}",
    "Firma de {o} verificada por {s}",
    "Desfragmentando {o}: bloque {n} de {n}",
    "Migrando {o} a {t}: {p}% transferido",
    "Bitácora de {u}: {n} entradas nuevas",
    "Reintentando la conexión con {t} (intento {l})",
    "Huella de {o}: 0x{x}{x}",
    "{u} pidió reiniciar {o}; esperando aprobación de {s}",
    "Copia de {o} enviada a {t}",
    "Analizando el tráfico hacia {t}: {n} paquetes por segundo",
    "Bloqueando la dirección 192.168.{l}.{p} por actividad sospechosa",
    "Reconstruyendo el índice de {o}: {p}%",
    "Mensaje de {s}: no tocar {o} hasta nuevo aviso",
    "Compilando el módulo de {o} para {t}",
    "Temperatura de {t}: {p} °C, ventiladores al máximo",
    "$ ssh {u}@eco.red.example -p {n}",
    "$ sudo systemctl restart vigilanted.service",
    "$ tar -czf respaldo_{n}.tar.gz /var/respaldos",
    "$ ping -c {l} nodo-central: 0% de pérdida",
    "$ git commit -m \"parche para {o}\"",
    "$ grep -c denegado /var/log/accesos.log: {n}",
    "Quórum alcanzado: {l} de 9 nodos votaron por {s}",
    "Desplegando la versión {l}.{n} en {t} sin tiempo de inactividad",
    "Revisión de {o} asignada a {s}",
];

const MUNDANE: &[&str] = &[
    "sistema en línea",
    "enlace estable",
    "respaldo completado",
    "sesión iniciada",
    "sesión cerrada",
    "nodo en espera",
    "sincronización completa",
    "verificación de integridad ok",
    "certificado vigente",
    "cola de mensajes vacía",
    "conexión cerrada por el host remoto",
    "limpieza periódica iniciada",
    "reloj sincronizado",
    "sin novedad en el turno",
    "puerto 22 escuchando",
    "carga del sistema normal",
];

const PACKETS: &[Packet] = &[
    Packet {
        proto: "ENLACE",
        tone: Tone::Green,
        len: (60, 300),
        info: "Enlace con {t} estable, latencia {p} ms",
    },
    Packet {
        proto: "AUTH",
        tone: Tone::Bright,
        len: (80, 400),
        info: "{u} se autentica desde {t}: respuesta {l}00",
    },
    Packet {
        proto: "LLAVE",
        tone: Tone::Amber,
        len: (64, 64),
        info: "Intercambio de llaves 0x{x}, {n} bits",
    },
    Packet {
        proto: "PULSO",
        tone: Tone::Cyan,
        len: (40, 90),
        info: "Pulso del nodo {l}: sin novedad",
    },
    Packet {
        proto: "ALERTA",
        tone: Tone::Red,
        len: (60, 200),
        info: "Acceso a {o} denegado a {u}",
    },
    Packet {
        proto: "RESPALD",
        tone: Tone::Main,
        len: (200, 1400),
        info: "Bloque {n} del respaldo de {o}, {p}% enviado",
    },
    Packet {
        proto: "AUDIT",
        tone: Tone::Bright,
        len: (90, 600),
        info: "Auditoría de {o} solicitada por {s}",
    },
    Packet {
        proto: "SINCRO",
        tone: Tone::Green,
        len: (60, 400),
        info: "Réplica de {t} al {p}%",
    },
];

/// The base, field by field.
pub static BASE: Vocab = Vocab {
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
    days: day_headlines,
    songs: &[],
    verse_odds: 0.0,
    nouns: NOUNS,
    adjs: ADJS,
    ofs: OFS,
    verbs: VERBS,
    states: STATES,
    errors: ERRORS,
    units: UNITS,
    canned_es: CANNED_ES,
    // No English: its "English" lines are its Spanish ones.
    canned_en: CANNED_ES,
    mundane: MUNDANE,
    mundane_es: MUNDANE.len(),
    packets: PACKETS,
    domain: "red.example",
    sniff: "  No.  Tiempo    Origen           Destino          Proto  Lon Info (red0)",
    feeds: [
        "hexdump -C /dev/nodo",
        "tail -f /var/log/sistema.log",
        "less nucleo/enlace.rs",
        "dmesg -w",
        "tcpdump -i red0",
        "tree -a /sistema",
        "blastn -query firma_nodo",
        "ssh-keygen -t ed25519",
        "bit\u{00e1}cora --turno",
        "apg -a 1 --boveda",
        "authd --acceso",
    ],
    dna_head: "BLASTN 2.14.1+  consulta: FIRMA DEL NODO  gap open/extend: 11/1",
    dna_kin: "Similitud",
    tree_end: "operadores",
    pem: (
        "-----BEGIN LLAVE PRIVADA-----",
        "-----END LLAVE PRIVADA-----",
    ),
    phase: "=== FASE ",
    mark: '\u{25c6}',
    dress: Dress {
        htop: "htop --sistema",
        users: ["root", "operador", "admin", "respaldo", "monitor", "nobody"],
        mem: "Mem",
        swap: "Int",
        tasks: "Tareas",
        bmon: "bmon -p red0",
        iface: "red0",
        down: "entran",
        up: "salen",
        down_unit: "Mbit/s",
        up_unit: "Mbit/s",
        rain: "cmatrix -ab --red",
        rain_words: &[
            "ACCESO", "ENLACE", "NODO", "SISTEMA", "LLAVE", "CIFRADO", "SENAL", "VECTOR", "PULSO",
            "FARO", "BOVEDA", "RED", "DATOS", "FIRMA", "TURNO", "ECO", "CLAVE", "PUERTO",
        ],
        map: "tracemap --enlace",
        trace: "SE\u{00d1}AL EN CAMINO",
        arrived: "RECIBIDAS",
        radar: "radar --barrido",
        contacts: "CONTACTOS",
        home: "EN BASE",
        progress: "pv --transferencia",
        sats: "rastreador-satelital",
        sats_head: "RASTREO ORBITAL  ENLACE ESTABLE",
        sat_names: &[
            "ECO-1",
            "FARO-2",
            "VECTOR-3",
            "CENIT-4",
            "ORBITA-5",
            "RELEVO-6",
            "ATALAYA-7",
            "PULSO-8",
            "DELTA-9",
            "SIGMA-10",
            "ISS (ZARYA)",
            "PRISMA-12",
        ],
        ticker: "bolsa --mercado",
        ticker_head: "CLAVE       \u{00da}LTIMO   CAMB%  TENDENCIA",
        symbols: &[
            ("NODO", true),
            ("ENLACE", true),
            ("CIFRA", true),
            ("DATOS", true),
            ("NUBE", false),
            ("DISCO", false),
            ("CINTA", false),
            ("COBRE", false),
            ("FIBRA", true),
            ("SILIC", true),
            ("RELE", false),
            ("ANTENA", false),
        ],
        authd: "authd --acceso",
        granted: [
            "ACCESO",
            "CONCEDIDO",
            " AUTENTICACI\u{00d3}N ",
            " BIENVENIDO AL SISTEMA ",
        ],
        denied: [
            "ACCESO",
            "DENEGADO",
            " AUTENTICACI\u{00d3}N ",
            " CREDENCIALES INV\u{00c1}LIDAS ",
        ],
        deny: "DENEGADO",
        admit: "CONCEDIDO",
        keygen: "ssh-keygen -lv --nodo",
        key_title: "[ED25519 256]",
        key_end: "+---[VERIFICADA]--+",
        key_lines: ["Generando la llave del nodo", "Huella:", "Paso"],
        art: [
            ' ', '.', '\u{00b7}', ':', '+', '=', 'o', '*', '\u{2022}', 'O', '\u{25cb}', '\u{25c7}',
            '\u{25c8}', '\u{25c6}', '\u{25a0}',
        ],
        art_start: '\u{2302}',
        art_walker: '\u{25c7}',
        art_end: '\u{25c6}',
    },
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::hollywood::{gate, lexico};

    #[test]
    fn the_base_passes_the_gate() {
        assert_eq!(gate::check(&BASE), Ok(()));
    }

    #[test]
    fn the_base_has_its_floor() {
        let b = &BASE;
        assert!(b.canned_es.len() >= 40, "canned_es {}", b.canned_es.len());
        assert!(b.headlines.len() >= 24, "headlines {}", b.headlines.len());
        assert!(b.people.len() >= 16, "people {}", b.people.len());
        assert!(b.errnos.len() >= 24, "errnos {}", b.errnos.len());
        assert!(CALLS.len() >= 12, "calls {}", CALLS.len());
        for (name, list) in [
            ("users", b.users),
            ("things", b.things),
            ("places", b.places),
            ("files", b.files),
            ("dirs", b.dirs),
            ("daemons", b.daemons),
            ("hosts", b.hosts),
            ("locals", b.locals),
            ("errs", b.errs),
            ("holy", b.holy),
            ("passwords", b.passwords),
        ] {
            assert!(list.len() >= 12, "{name} {}", list.len());
        }
    }

    #[test]
    fn the_base_is_all_spanish() {
        assert_eq!(BASE.mundane_es, BASE.mundane.len());
        for line in BASE.canned_en {
            assert!(BASE.canned_es.contains(line), "{line}");
        }
        assert!((BASE.days)(Fiesta::NONE).is_empty());
        assert!((BASE.days)(Fiesta::of_day(12, 31)).is_empty());
        assert!((BASE.days)(Fiesta::of_day(9, 15)).is_empty());
        assert!(BASE.songs.is_empty());
        assert_eq!(TRANSFERS.len(), 6);
    }

    #[test]
    fn the_base_writes_clean_text() {
        assert!(BASE.fragment_count() >= 600, "{}", BASE.fragment_count());
        let distinct = lexico::assert_writes_clean(&BASE, "base");
        assert!(distinct >= 8_000, "{distinct}");
    }

    #[test]
    fn the_base_things_carry_their_article() {
        for t in BASE.things {
            assert!(
                ["el ", "la ", "los ", "las "]
                    .iter()
                    .any(|a| t.starts_with(a)),
                "{t}"
            );
        }
    }

    #[test]
    fn call_signs_fit_the_radar() {
        for c in CALLS {
            assert!(
                (1..=9).contains(&c.len()) && c.bytes().all(|b| b.is_ascii_uppercase()),
                "{c}"
            );
        }
    }
}
